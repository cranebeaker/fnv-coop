//! `collision`: a survey of the game's collision. Every model the game can
//! see (loose or in the archives it loads) is read for its Havok blocks:
//! how often each shape, body, phantom and constraint type occurs, which
//! layers bodies and triangle sub-parts sit on (and whether the player runs
//! into them, by the game's layer matrix), how bodies move, and the
//! arrangements a reader has to get right (collision below the top node,
//! several collision objects, offsets on `bhkRigidBodyT`, scaled shapes).
//! Then what `nif::collision` doesn't read. Last, the references placed with
//! a primitive (`XPRM`): their bases, shapes and collision layers (`XTRI`).

use std::collections::{BTreeMap, BTreeSet};
use std::io::Write;
use std::panic::{self, AssertUnwindSafe};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Mutex;

use assets::Assets;
use esm::{FourCC, LoadOrder};
use nif::collision::layers;
use nif::{Block, Nif, Transform};

use crate::fmt::thousands;
use crate::CliError;

/// How often something occurs, in how many models, and the first model
/// (alphabetically) it was seen in.
#[derive(Default, Clone)]
struct Tally {
    count: usize,
    files: usize,
    example: String,
}

impl Tally {
    fn add(&mut self, count: usize, path: &str) {
        self.count += count;
        self.files += 1;
        if self.example.is_empty() || path < self.example.as_str() {
            self.example = path.to_string();
        }
    }

    fn merge(&mut self, other: Tally) {
        self.count += other.count;
        self.files += other.files;
        if self.example.is_empty() || (!other.example.is_empty() && other.example < self.example) {
            self.example = other.example;
        }
    }
}

fn merge_map<K: Ord>(into: &mut BTreeMap<K, Tally>, from: BTreeMap<K, Tally>) {
    for (k, v) in from {
        into.entry(k).or_default().merge(v);
    }
}

/// Counts within one model, turned into tallies at the end.
#[derive(Default)]
struct Local<K: Ord> {
    counts: BTreeMap<K, usize>,
}

impl<K: Ord> Local<K> {
    fn bump(&mut self, key: K) {
        *self.counts.entry(key).or_default() += 1;
    }

    fn into_tallies(self, path: &str, into: &mut BTreeMap<K, Tally>) {
        for (k, n) in self.counts {
            into.entry(k).or_default().add(n, path);
        }
    }
}

#[derive(Default)]
struct Totals {
    models: usize,
    with_collision: usize,
    failed: Vec<(String, String)>,
    /// Havok block types (`bhk…`, `hk…`).
    types: BTreeMap<String, Tally>,
    /// (layer, what carries it).
    layers: BTreeMap<(u8, &'static str), Tally>,
    /// Bodies by motion system.
    motion: BTreeMap<u8, Tally>,
    features: BTreeMap<String, Tally>,
    /// Block types `nif::collision` met and didn't read.
    unhandled: BTreeMap<String, Tally>,
    /// Errors from `nif::collision`.
    errors: BTreeMap<String, Tally>,
    /// Models whose solid collision reaches past what they draw: how far
    /// (game units) and the model. A check on how the shapes are read and
    /// placed.
    outside: Vec<(f32, String)>,
    /// Models with both drawing and solid collision, compared.
    compared: usize,
}

impl Totals {
    fn merge(&mut self, other: Totals) {
        self.models += other.models;
        self.outside.extend(other.outside);
        self.compared += other.compared;
        self.with_collision += other.with_collision;
        self.failed.extend(other.failed);
        merge_map(&mut self.types, other.types);
        merge_map(&mut self.layers, other.layers);
        merge_map(&mut self.motion, other.motion);
        merge_map(&mut self.features, other.features);
        merge_map(&mut self.unhandled, other.unhandled);
        merge_map(&mut self.errors, other.errors);
    }
}

fn le_u16(b: &[u8], at: usize) -> Option<u16> {
    Some(u16::from_le_bytes(b.get(at..at + 2)?.try_into().ok()?))
}

fn le_u32(b: &[u8], at: usize) -> Option<u32> {
    Some(u32::from_le_bytes(b.get(at..at + 4)?.try_into().ok()?))
}

fn le_i32(b: &[u8], at: usize) -> Option<i32> {
    le_u32(b, at).map(|v| v as i32)
}

fn le_f32(b: &[u8], at: usize) -> Option<f32> {
    le_u32(b, at).map(f32::from_bits)
}

/// The layers of a `hkPackedNiTriStripsData`'s sub-parts (triangles, 8
/// bytes each; vertices, 12 bytes or 6 compressed; then 12-byte sub-parts
/// with the filter first).
fn packed_layers(b: &[u8]) -> Option<Vec<u8>> {
    let triangles = le_u32(b, 0)? as usize;
    let mut at = 4 + triangles.checked_mul(8)?;
    let vertices = le_u32(b, at)? as usize;
    let compressed = *b.get(at + 4)? != 0;
    at += 5 + vertices.checked_mul(if compressed { 6 } else { 12 })?;
    let parts = usize::from(le_u16(b, at)?);
    at += 2;
    (0..parts).map(|k| b.get(at + 12 * k).copied()).collect()
}

/// The scale of the packed triangles a collision object's body holds
/// (through a MOPP tree), if that's its shape.
fn packed_scale_under(nif: &Nif, object: usize) -> Option<f32> {
    let block = |i: i32| usize::try_from(i).ok().filter(|&i| i < nif.blocks().len());
    let body = block(le_i32(nif.block_bytes(object), 6)?)?;
    let mut shape = block(le_i32(nif.block_bytes(body), 0)?)?;
    if nif.block_type(shape) == "bhkMoppBvTreeShape" {
        shape = block(le_i32(nif.block_bytes(shape), 0)?)?;
    }
    (nif.block_type(shape) == "bhkPackedNiTriStripsShape")
        .then(|| le_f32(nif.block_bytes(shape), 16))
        .flatten()
}

fn is_identity(t: &Transform) -> bool {
    let close = |a: f32, b: f32| (a - b).abs() < 1e-4;
    t.translation.iter().all(|&v| v.abs() < 1e-3)
        && (0..3).all(|i| (0..3).all(|j| close(t.rotation[i][j], if i == j { 1.0 } else { 0.0 })))
        && close(t.scale, 1.0)
}

/// One model's collision, tallied.
fn survey_model(nif: &Nif, path: &str, totals: &mut Totals) {
    totals.models += 1;
    let mut types = Local::default();
    let mut layer_counts: Local<(u8, &'static str)> = Local::default();
    let mut motion = Local::default();
    let mut features: Local<String> = Local::default();
    let mut collision_objects = BTreeSet::new();
    for i in 0..nif.blocks().len() {
        let t = nif.block_type(i);
        if !(t.starts_with("bhk") || t.starts_with("hk")) {
            continue;
        }
        types.bump(t.to_string());
        let b = nif.block_bytes(i);
        match t {
            "bhkRigidBody" | "bhkRigidBodyT" => {
                let (Some(&layer), Some(&flags), Some(&system)) = (b.get(4), b.get(5), b.get(212))
                else {
                    features.bump(format!("{t} shorter than expected"));
                    continue;
                };
                let kind = match system {
                    7 => "fixed body",
                    6 => "keyframed body",
                    _ => "moving body",
                };
                layer_counts.bump((layer, kind));
                motion.bump(system);
                if flags & layers::NO_COLLISION_FLAG != 0 {
                    features.bump("body with the no-collision flag (0x40)".into());
                }
                if b.len() != 236 {
                    let constraints = le_u32(b, 228).unwrap_or(0);
                    features.bump(format!("body with {constraints} constraint(s)"));
                }
                if t == "bhkRigidBodyT" {
                    let f = |k: usize| le_f32(b, k).unwrap_or(0.0);
                    let moved = (52..64).step_by(4).any(|k| f(k).abs() > 1e-4);
                    let turned = (68..80).step_by(4).any(|k| f(k).abs() > 1e-4);
                    if moved || turned {
                        features.bump(format!(
                            "bhkRigidBodyT with an offset{}",
                            if turned { " and a turn" } else { "" }
                        ));
                    }
                }
            }
            "bhkSimpleShapePhantom" | "bhkAabbPhantom" => {
                if let Some(&layer) = b.get(4) {
                    layer_counts.bump((layer, "phantom"));
                }
            }
            "hkPackedNiTriStripsData" => match packed_layers(b) {
                Some(parts) => {
                    if parts.is_empty() {
                        features.bump("packed triangles without sub-parts".into());
                    }
                    for layer in parts {
                        layer_counts.bump((layer, "triangle sub-part"));
                    }
                }
                None => features.bump("hkPackedNiTriStripsData not understood".into()),
            },
            "bhkPackedNiTriStripsShape" => {
                if let Some(r) = le_f32(b, 8) {
                    features.bump(format!("{t} radius {r:.3}"));
                }
                let s = [16, 20, 24].map(|k| le_f32(b, k).unwrap_or(1.0));
                if s.iter().any(|v| (v - 1.0).abs() > 1e-4) {
                    let uniform = (s[0] - s[1]).abs() < 1e-5 && (s[0] - s[2]).abs() < 1e-5;
                    features.bump(format!(
                        "bhkPackedNiTriStripsShape with a scale other than 1 ({})",
                        if uniform { "uniform" } else { "not uniform" }
                    ));
                }
            }
            "bhkMoppBvTreeShape" => {
                if let Some(child) = le_i32(b, 0).and_then(|c| usize::try_from(c).ok()) {
                    if child < nif.blocks().len() {
                        features.bump(format!("bhkMoppBvTreeShape over {}", nif.block_type(child)));
                    }
                }
                if let Some(s) = le_f32(b, 16).filter(|s| (s - 1.0).abs() > 1e-4) {
                    features.bump(format!("bhkMoppBvTreeShape with shape scale {s}"));
                }
            }
            "bhkTransformShape" | "bhkConvexTransformShape" => {
                // The matrix's first column (at 20): a rotation's is 1 long.
                let length = [20, 24, 28]
                    .map(|k| le_f32(b, k).unwrap_or(0.0))
                    .iter()
                    .map(|v| v * v)
                    .sum::<f32>()
                    .sqrt();
                if (length - 1.0).abs() > 1e-3 {
                    features.bump(format!("{t} whose matrix also scales (by {length:.3})"));
                }
            }
            "bhkConvexVerticesShape" | "bhkBoxShape" | "bhkNiTriStripsShape" => {
                if let Some(r) = le_f32(b, 4) {
                    features.bump(format!("{t} radius {r:.3}"));
                }
            }
            _ if t.ends_with("CollisionObject") => {
                collision_objects.insert(i);
            }
            _ => {}
        }
    }
    if collision_objects.is_empty() {
        types.into_tallies(path, &mut totals.types);
        return;
    }
    totals.with_collision += 1;
    if collision_objects.len() > 1 {
        features.bump("model with more than one collision object".into());
    }

    // The scene graph: where each collision object hangs and what moves it.
    let mut reached = BTreeSet::new();
    let mut stack: Vec<(i32, Transform, usize, bool)> = nif
        .roots()
        .iter()
        .map(|&r| (r, Transform::IDENTITY, 0usize, false))
        .collect();
    let mut seen = vec![false; nif.blocks().len()];
    while let Some((reference, below_root, depth, hidden)) = stack.pop() {
        let Some(index) = usize::try_from(reference)
            .ok()
            .filter(|&i| i < nif.blocks().len())
        else {
            continue;
        };
        if std::mem::replace(&mut seen[index], true) || depth > 64 {
            continue;
        }
        let (av, children, active, geometry) = match nif.block(index) {
            Ok(Block::Node(n)) => (n.av, n.children, n.active_child, false),
            Ok(Block::Geometry(g)) => (g.av, Vec::new(), None, true),
            _ => continue,
        };
        let here = if depth == 0 {
            Transform::IDENTITY
        } else {
            below_root.then_child(&av.transform)
        };
        if let Some(object) = usize::try_from(av.collision)
            .ok()
            .filter(|&i| i < nif.blocks().len())
        {
            reached.insert(object);
            let kind = nif.block_type(object);
            if geometry {
                features.bump(format!("{kind} on a shape (geometry) block"));
            }
            if depth == 0 && !is_identity(&av.transform) {
                features.bump(format!("{kind} on a moved top node (ignored when placed)"));
            }
            if depth > 0 {
                features.bump(format!("{kind} below the top node"));
                if !is_identity(&here) {
                    features.bump(format!("{kind} on a node its parents move"));
                }
                if (here.scale - 1.0).abs() > 1e-4 {
                    features.bump(format!("{kind} on a scaled node"));
                    // Whether the triangles' own scale already holds it.
                    match packed_scale_under(nif, object) {
                        Some(s) if (s - here.scale).abs() < 1e-3 => features.bump(
                            "  ... packed triangles scaled by the node's scale themselves".into(),
                        ),
                        Some(s) if (s - 1.0).abs() < 1e-4 => {
                            features.bump("  ... packed triangles unscaled".into())
                        }
                        Some(_) => features.bump("  ... packed triangles scaled otherwise".into()),
                        None => features.bump("  ... not packed triangles".into()),
                    }
                }
            }
            if hidden {
                features.bump(format!("{kind} under a switch/LOD node's other children"));
            }
        }
        for (k, child) in children.iter().enumerate() {
            let off = active.is_some_and(|a| a != k);
            stack.push((*child, here, depth + 1, hidden || off));
        }
    }
    for &object in &collision_objects {
        if !reached.contains(&object) {
            let target = le_i32(nif.block_bytes(object), 0)
                .and_then(|t| usize::try_from(t).ok())
                .filter(|&t| t < nif.blocks().len())
                .map_or("nothing", |t| nif.block_type(t));
            features.bump(format!(
                "{} not reached from the top node (target {target})",
                nif.block_type(object)
            ));
        }
    }

    match nif.collision() {
        Ok(c) => {
            for (t, n) in c.unhandled {
                totals.unhandled.entry(t).or_default().add(n, path);
            }
            compare_with_drawing(nif, path, totals);
        }
        Err(e) => {
            let text = e.to_string();
            let key = text
                .split(':')
                .next()
                .unwrap_or(&text)
                .chars()
                .take(80)
                .collect::<String>();
            totals.errors.entry(key).or_default().add(1, path);
        }
    }
    types.into_tallies(path, &mut totals.types);
    layer_counts.into_tallies(path, &mut totals.layers);
    motion.into_tallies(path, &mut totals.motion);
    features.into_tallies(path, &mut totals.features);
}

/// The box around points, if any.
fn bounds(points: impl Iterator<Item = [f32; 3]>) -> Option<([f32; 3], [f32; 3])> {
    let mut lo = [f32::INFINITY; 3];
    let mut hi = [f32::NEG_INFINITY; 3];
    for p in points {
        for k in 0..3 {
            lo[k] = lo[k].min(p[k]);
            hi[k] = hi[k].max(p[k]);
        }
    }
    lo[0].is_finite().then_some((lo, hi))
}

/// How far a model's solid collision (placed as the game places it) reaches
/// past the box around what it draws, beyond a margin of 16 units and a
/// tenth of the drawing's size: the shapes' layouts, scales and the nodes'
/// transforms would show up here when misread.
fn compare_with_drawing(nif: &Nif, path: &str, totals: &mut Totals) {
    let (Ok(scene), Ok(collision)) = (nif.placed_scene(), nif.placed_collision()) else {
        return;
    };
    let solid: Vec<[f32; 3]> = collision
        .parts
        .iter()
        .filter(|p| layers::blocks_walking(p.layer) && !p.dynamic)
        .flat_map(|p| match &p.shape {
            nif::CollisionShape::Triangles {
                vertices,
                triangles,
            } => triangles
                .iter()
                .flatten()
                .filter_map(|&i| vertices.get(i as usize).copied())
                .collect::<Vec<_>>(),
            nif::CollisionShape::Convex { vertices, .. } => vertices.clone(),
            nif::CollisionShape::Sphere { center, radius } => {
                vec![center.map(|c| c - radius), center.map(|c| c + radius)]
            }
            nif::CollisionShape::Capsule { a, b, radius } => vec![
                a.map(|c| c - radius),
                a.map(|c| c + radius),
                b.map(|c| c - radius),
                b.map(|c| c + radius),
            ],
        })
        .collect();
    let drawn = bounds(
        scene
            .meshes
            .iter()
            .filter(|m| !m.skinned)
            .flat_map(|m| m.model_positions()),
    );
    let (Some((clo, chi)), Some((vlo, vhi))) = (bounds(solid.into_iter()), drawn) else {
        return;
    };
    totals.compared += 1;
    let size = (0..3).map(|k| vhi[k] - vlo[k]).fold(0.0f32, f32::max);
    let margin = 16.0 + 0.1 * size;
    let past = (0..3)
        .map(|k| (vlo[k] - clo[k]).max(chi[k] - vhi[k]))
        .fold(0.0f32, f32::max);
    if past > margin {
        totals.outside.push((past, path.to_string()));
    }
}

/// `collision`: every model the game can see, then the placed primitives.
pub fn survey(out: &mut impl Write, order: &LoadOrder, assets: &Assets) -> Result<(), CliError> {
    let mut paths: Vec<&str> = assets.paths().filter(|p| p.ends_with(".nif")).collect();
    paths.sort_unstable();
    let next = AtomicUsize::new(0);
    let totals = Mutex::new(Totals::default());
    let threads = std::thread::available_parallelism()
        .map_or(4, |n| n.get())
        .min(16);
    let previous_hook = panic::take_hook();
    panic::set_hook(Box::new(|_| {}));
    std::thread::scope(|scope| {
        for _ in 0..threads {
            scope.spawn(|| {
                let mut local = Totals::default();
                loop {
                    let i = next.fetch_add(1, Ordering::Relaxed);
                    let Some(&path) = paths.get(i) else { break };
                    let bytes = match assets.read(path) {
                        Ok(Some(b)) => b,
                        Ok(None) => continue,
                        Err(e) => {
                            local.failed.push((path.to_string(), e.to_string()));
                            continue;
                        }
                    };
                    let outcome = panic::catch_unwind(AssertUnwindSafe(|| {
                        let nif = Nif::parse(bytes)?;
                        let mut one = Totals::default();
                        survey_model(&nif, path, &mut one);
                        Ok::<_, nif::Error>(one)
                    }));
                    match outcome {
                        Ok(Ok(one)) => local.merge(one),
                        Ok(Err(e)) => local.failed.push((path.to_string(), e.to_string())),
                        Err(_) => local.failed.push((path.to_string(), "panicked".into())),
                    }
                }
                totals
                    .lock()
                    .unwrap_or_else(|p| p.into_inner())
                    .merge(local);
            });
        }
    });
    panic::set_hook(previous_hook);
    let totals = totals.into_inner().unwrap_or_else(|p| p.into_inner());
    report(out, &totals)?;
    primitives(out, order)
}

fn tally_line(out: &mut impl Write, label: &str, t: &Tally) -> Result<(), CliError> {
    writeln!(
        out,
        "  {label:<58} {:>8} in {:>6} models  e.g. {}",
        thousands(t.count),
        thousands(t.files),
        t.example
    )?;
    Ok(())
}

/// What `nif::collision` does with each Havok block type.
fn handling(t: &str) -> &'static str {
    match t {
        "bhkCollisionObject" => "read",
        "bhkRigidBody" | "bhkRigidBodyT" => "read",
        "bhkMoppBvTreeShape"
        | "bhkPackedNiTriStripsShape"
        | "hkPackedNiTriStripsData"
        | "bhkBoxShape"
        | "bhkSphereShape"
        | "bhkCapsuleShape"
        | "bhkConvexVerticesShape"
        | "bhkConvexTransformShape"
        | "bhkTransformShape"
        | "bhkListShape"
        | "bhkNiTriStripsShape" => "read",
        "bhkSPCollisionObject" | "bhkSimpleShapePhantom" | "bhkAabbPhantom" => {
            "phantom: never solid"
        }
        "bhkBlendCollisionObject" => "ragdoll bones, nif::ragdoll; biped layer: not solid",
        t if t.contains("Constraint") => "joints: not needed to stand on",
        "bhkBlendController" | "bhkLiquidAction" | "bhkOrientHingedBodyAction" => {
            "animation and effects: not solid"
        }
        _ => "",
    }
}

fn report(out: &mut impl Write, totals: &Totals) -> Result<(), CliError> {
    writeln!(
        out,
        "Collision in the {} models the game can see: {} carry a collision object; {} couldn't be read.",
        thousands(totals.models),
        thousands(totals.with_collision),
        thousands(totals.failed.len())
    )?;
    for (path, e) in totals.failed.iter().take(10) {
        writeln!(out, "  unreadable: {path}: {e}")?;
    }

    writeln!(
        out,
        "\nHavok blocks (blocks, models, example; what nv-rs does):"
    )?;
    let mut types: Vec<_> = totals.types.iter().collect();
    types.sort_by(|a, b| b.1.count.cmp(&a.1.count).then(a.0.cmp(b.0)));
    for (t, tally) in types {
        let what = handling(t);
        let label = if what.is_empty() {
            t.clone()
        } else {
            format!("{t} ({what})")
        };
        tally_line(out, &label, tally)?;
    }

    writeln!(
        out,
        "\nLayers (body and triangle sub-part filters; * = the player and people run into it):"
    )?;
    for ((layer, kind), tally) in &totals.layers {
        let star = if layers::blocks_walking(*layer) {
            "*"
        } else {
            " "
        };
        let label = format!("{star}{layer:>2} {} ({kind})", layers::name(*layer));
        tally_line(out, &label, tally)?;
    }

    writeln!(out, "\nBodies by motion system:")?;
    for (system, tally) in &totals.motion {
        let name = match system {
            0 => "invalid",
            1 => "dynamic",
            2 => "sphere inertia",
            3 => "sphere stabilized",
            4 => "box inertia",
            5 => "box stabilized",
            6 => "keyframed",
            7 => "fixed",
            8 => "thin box",
            9 => "character",
            _ => "unknown",
        };
        tally_line(out, &format!("{system} {name}"), tally)?;
    }

    writeln!(out, "\nArrangements:")?;
    for (feature, tally) in &totals.features {
        tally_line(out, feature, tally)?;
    }

    let mut outside = totals.outside.clone();
    outside.sort_by(|a, b| b.0.total_cmp(&a.0).then(a.1.cmp(&b.1)));
    writeln!(
        out,
        "\nSolid fixed collision against drawing: {} of {} models reach more than 16 units and a tenth of their size past what they draw; the furthest:",
        outside.len(),
        thousands(totals.compared)
    )?;
    for (past, path) in outside.iter().take(15) {
        writeln!(out, "  {past:>8.0}  {path}")?;
    }

    writeln!(out, "\nWhat nif::collision met and doesn't read:")?;
    if totals.unhandled.is_empty() {
        writeln!(out, "  nothing")?;
    }
    for (t, tally) in &totals.unhandled {
        tally_line(out, t, tally)?;
    }
    if !totals.errors.is_empty() {
        writeln!(out, "\nCollision that couldn't be read:")?;
        for (e, tally) in &totals.errors {
            tally_line(out, e, tally)?;
        }
    }
    Ok(())
}

/// References placed with a primitive (`XPRM`): by base, shape and the
/// collision layer the reference names (`XTRI`).
fn primitives(out: &mut impl Write, order: &LoadOrder) -> Result<(), CliError> {
    const XPRM: FourCC = FourCC::new(b"XPRM");
    const XTRI: FourCC = FourCC::new(b"XTRI");
    const NAME: FourCC = FourCC::new(b"NAME");
    let mut groups: BTreeMap<(String, String, u32, String), (usize, String)> = BTreeMap::new();
    for kind in [
        FourCC::new(b"REFR"),
        FourCC::new(b"ACRE"),
        FourCC::new(b"ACHR"),
    ] {
        for rr in order.records_of_type(kind) {
            let Ok(record) = rr.record() else { continue };
            let Some(prim) = record.get(XPRM).filter(|s| s.data.len() >= 32) else {
                continue;
            };
            let shape = le_u32(&prim.data, 28).unwrap_or(0);
            let layer = record
                .get(XTRI)
                .and_then(|s| le_u32(&s.data, 0))
                .map_or("no XTRI".to_string(), |l| {
                    format!("{l} {}", layers::name(l.min(255) as u8))
                });
            let (base_type, base_name) = record
                .get(NAME)
                .and_then(|s| le_u32(&s.data, 0))
                .map(|b| rr.plugin.to_global(esm::FormId(b)))
                .and_then(|b| order.get(b))
                .map_or(("?".to_string(), "?".to_string()), |b| {
                    (
                        b.entry.header.kind.to_string(),
                        b.editor_id().ok().flatten().unwrap_or_default(),
                    )
                });
            let label = rr
                .editor_id()
                .ok()
                .flatten()
                .unwrap_or_else(|| format!("{:08X}", rr.form_id.0));
            let slot = groups
                .entry((base_type, base_name, shape, layer))
                .or_insert((0, label.clone()));
            slot.0 += 1;
        }
    }
    writeln!(
        out,
        "\nPlaced primitives (XPRM; shape 1 box, 2 sphere, 3 plane), by base, shape and layer (XTRI):"
    )?;
    let mut list: Vec<_> = groups.into_iter().collect();
    list.sort_by(|a, b| b.1 .0.cmp(&a.1 .0).then(a.0.cmp(&b.0)));
    for ((base_type, base, shape, layer), (n, example)) in list {
        writeln!(
            out,
            "  {n:>6}  {base_type} {base:<32} shape {shape}  layer {layer:<24} e.g. {example}"
        )?;
    }
    Ok(())
}

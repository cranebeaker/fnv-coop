//! `particles`: the game's particle systems. With no path, every model the
//! game can see is read for its particle blocks: each block type decoded
//! (and whether its bytes were used up exactly, the check on the layouts in
//! `nif::particles`), then what the systems use: emitters, modifiers,
//! controllers and their keys, colliders, spawning, how they're drawn. With
//! a model's path, that model's systems in full; with `run SECONDS` after
//! it, the systems run as the game runs them (`world::particles`), the
//! model placed at the origin playing what a placed object plays, and what
//! they hold each second.

use std::collections::BTreeMap;
use std::io::Write;
use std::panic::{self, AssertUnwindSafe};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Mutex;

use assets::Assets;
use nif::particles::{
    ColliderShape, ControllerKind, EmitterShape, ModifierKind, ParticleBlock, ParticleSystem,
};
use nif::Nif;

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
    fn merge(&mut self, other: Tally) {
        self.count += other.count;
        self.files += other.files;
        if self.example.is_empty() || (!other.example.is_empty() && other.example < self.example) {
            self.example = other.example;
        }
    }
}

#[derive(Default)]
struct Totals {
    models: usize,
    with_particles: usize,
    failed: Vec<(String, String)>,
    /// Block type → decoded exactly.
    decoded: BTreeMap<String, Tally>,
    /// Block type → error, by message.
    errors: BTreeMap<String, Tally>,
    features: BTreeMap<String, Tally>,
}

impl Totals {
    fn merge(&mut self, other: Totals) {
        self.models += other.models;
        self.with_particles += other.with_particles;
        self.failed.extend(other.failed);
        for (map, from) in [
            (&mut self.decoded, other.decoded),
            (&mut self.errors, other.errors),
            (&mut self.features, other.features),
        ] {
            for (k, v) in from {
                map.entry(k).or_default().merge(v);
            }
        }
    }
}

/// Counts within one model.
#[derive(Default)]
struct Local {
    decoded: BTreeMap<String, usize>,
    errors: BTreeMap<String, usize>,
    features: BTreeMap<String, usize>,
}

impl Local {
    fn feature(&mut self, key: impl Into<String>) {
        *self.features.entry(key.into()).or_default() += 1;
    }

    fn into_totals(self, path: &str, totals: &mut Totals) {
        for (map, from) in [
            (&mut totals.decoded, self.decoded),
            (&mut totals.errors, self.errors),
            (&mut totals.features, self.features),
        ] {
            for (k, n) in from {
                let t = map.entry(k).or_default();
                t.count += n;
                t.files += 1;
                if t.example.is_empty() || path < t.example.as_str() {
                    t.example = path.to_string();
                }
            }
        }
    }
}

fn round(v: f32) -> String {
    let r = (v * 1000.0).round() / 1000.0;
    format!("{r}")
}

fn survey_model(nif: &Nif, local: &mut Local) -> bool {
    let mut any = false;
    for i in 0..nif.blocks().len() {
        let t = nif.block_type(i).to_string();
        if !nif::particles::is_particle_type(&t)
            && !(matches!(
                t.as_str(),
                "NiFloatInterpolator" | "NiBoolInterpolator" | "NiBoolTimelineInterpolator"
            ) && used_by_particles(nif, i))
        {
            continue;
        }
        any = true;
        match nif.particle_block(i) {
            Ok(block) => {
                *local.decoded.entry(t.clone()).or_default() += 1;
                if let ParticleBlock::Controller(c) = &block {
                    local.feature(format!(
                        "controller {} flags {:#06x}",
                        c.type_name, c.time.flags
                    ));
                    if c.time.frequency != 1.0 {
                        local.feature(format!("controller {} frequency not 1", c.type_name));
                    }
                }
            }
            Err(e) => {
                let message = e.to_string();
                let short: String = message.chars().take(140).collect();
                *local.errors.entry(format!("{t}: {short}")).or_default() += 1;
            }
        }
    }
    if !any {
        return false;
    }
    // What the systems as placed use.
    match nif.particle_systems(false) {
        Ok(systems) => {
            for s in &systems {
                system_features(s, local);
            }
        }
        Err(e) => {
            let short: String = e.to_string().chars().take(140).collect();
            *local
                .errors
                .entry(format!("particle_systems: {short}"))
                .or_default() += 1;
        }
    }
    // Sequences naming particle controllers (driven by a controller manager).
    if let Ok(names) = sequence_controller_types(nif) {
        for n in names {
            local.feature(format!("sequence drives {n}"));
        }
    }
    match nif.particle_sequences() {
        Ok(sequences) => {
            for s in &sequences {
                for t in &s.tracks {
                    local.feature(format!(
                        "sequence track {} {}",
                        t.controller_type,
                        track_value_name(&t.value)
                    ));
                }
                if !s.tracks.is_empty() {
                    local.feature(format!(
                        "sequence with particle tracks cycle {} frequency {}",
                        s.cycle,
                        round(s.frequency)
                    ));
                }
            }
        }
        Err(e) => {
            let short: String = e.to_string().chars().take(140).collect();
            *local
                .errors
                .entry(format!("particle_sequences: {short}"))
                .or_default() += 1;
        }
    }
    true
}

fn track_value_name(v: &nif::particles::TrackValue) -> String {
    match v {
        nif::particles::TrackValue::Float(f) => {
            format!(
                "float keys kind {} count {}",
                f.keys.kind,
                f.keys.keys.len()
            )
        }
        nif::particles::TrackValue::Bool(interp) => format!(
            "bool{} value {} keys kind {} count {}",
            if interp.timeline { " timeline" } else { "" },
            interp.value,
            interp.kind,
            interp.keys.len()
        ),
    }
}

/// Whether a block is an interpolator one of the particle controllers uses.
fn used_by_particles(nif: &Nif, index: usize) -> bool {
    (0..nif.blocks().len()).any(|i| {
        nif.block_type(i).starts_with("NiPSys") && nif.block_type(i).ends_with("Ctlr") && {
            let b = nif.block_bytes(i);
            // The interpolator reference sits after NiTimeController's 26
            // bytes; the emitter's active interpolator after its name.
            let at = |o: usize| {
                b.get(o..o + 4)
                    .map(|s| i32::from_le_bytes(s.try_into().unwrap()))
            };
            [at(26), at(34)].contains(&Some(index as i32))
        }
    })
}

/// The controller types a model's sequences drive.
fn sequence_controller_types(nif: &Nif) -> Result<Vec<String>, nif::Error> {
    let mut out = Vec::new();
    for i in 0..nif.blocks().len() {
        if nif.block_type(i) != "NiControllerSequence" {
            continue;
        }
        let b = nif.block_bytes(i);
        let strings = &nif.header().strings;
        let u32_at = |o: usize| {
            b.get(o..o + 4)
                .map(|s| u32::from_le_bytes(s.try_into().unwrap()))
        };
        let Some(count) = u32_at(4) else { continue };
        let mut at = 12;
        for _ in 0..count.min(1024) {
            // interpolator, controller, priority, node, property type,
            // controller type, controller ID, interpolator ID.
            let Some(ctype) = u32_at(at + 17) else { break };
            if let Some(s) = strings.get(ctype as usize) {
                if s.contains("PSys") {
                    out.push(s.clone());
                }
            }
            at += 29;
        }
    }
    Ok(out)
}

fn system_features(s: &ParticleSystem, local: &mut Local) {
    local.feature(format!("system {}", s.type_name));
    local.feature(format!("system world space {}", s.world_space));
    let d = &s.data;
    local.feature(format!(
        "data colors {} radii {} sizes {} rotations {} angles {} axes {} texture indices {} rotation speeds {}",
        d.has_colors,
        d.has_radii,
        d.has_sizes,
        d.has_rotations,
        d.has_rotation_angles,
        d.has_rotation_axes,
        d.has_texture_indices,
        d.has_rotation_speeds
    ));
    local.feature(format!("data sub-textures {}", d.subtexture_offsets.len()));
    let orders: Vec<String> = s
        .modifiers
        .iter()
        .map(|m| format!("{}:{}", kind_name(&m.kind), m.order))
        .collect();
    local.feature(format!("modifier order {}", orders.join(" ")));
    for m in &s.modifiers {
        if !m.active {
            local.feature(format!("modifier {} starts inactive", kind_name(&m.kind)));
        }
        match &m.kind {
            ModifierKind::AgeDeath { spawn_on_death, .. } => {
                local.feature(format!("age death spawn on death {spawn_on_death}"));
            }
            ModifierKind::Emitter(e) => {
                let (shape, object) = match &e.shape {
                    EmitterShape::Box { object, .. } => ("box", object.is_some()),
                    EmitterShape::Cylinder { object, .. } => ("cylinder", object.is_some()),
                    EmitterShape::Sphere { object, .. } => ("sphere", object.is_some()),
                    EmitterShape::Array { object } => ("array", object.is_some()),
                    EmitterShape::Mesh {
                        velocity_type,
                        emission_type,
                        meshes,
                        ..
                    } => {
                        local.feature(format!(
                            "mesh emitter velocity type {velocity_type} emission type {emission_type} meshes {}",
                            meshes.len()
                        ));
                        ("mesh", !meshes.is_empty())
                    }
                };
                local.feature(format!("emitter {shape} object {object}"));
            }
            ModifierKind::Gravity(g) => {
                local.feature(format!(
                    "gravity force type {} decay {} turbulence {} world aligned {} object {}",
                    g.force_type,
                    g.decay != 0.0,
                    g.turbulence != 0.0,
                    g.world_aligned,
                    g.object.is_some()
                ));
            }
            ModifierKind::Drag(dr) => {
                local.feature(format!("drag object {}", dr.object.is_some()));
            }
            ModifierKind::Bomb(b) => {
                local.feature(format!(
                    "bomb decay type {} symmetry {} object {}",
                    b.decay_type,
                    b.symmetry_type,
                    b.object.is_some()
                ));
            }
            ModifierKind::GrowFade(g) => {
                local.feature(format!(
                    "grow fade generations {} {} base scale {}",
                    g.grow_generation,
                    g.fade_generation,
                    round(g.base_scale)
                ));
            }
            ModifierKind::Rotation(r) => {
                local.feature(format!(
                    "rotation random axis {} random sign {}",
                    r.random_axis, r.random_speed_sign
                ));
            }
            ModifierKind::Color { kind, keys } => {
                local.feature(format!("color keys kind {kind} count {}", keys.len()));
            }
            ModifierKind::SimpleColor(c) => {
                local.feature(format!(
                    "simple color percents {} {} {} {}",
                    round(c.color1_end),
                    round(c.color2_start),
                    round(c.color2_end),
                    round(c.color3_start)
                ));
            }
            ModifierKind::Colliders(cs) => {
                for c in cs {
                    let shape = match &c.shape {
                        ColliderShape::Plane { .. } => "plane".to_string(),
                        ColliderShape::Sphere { .. } => "sphere".to_string(),
                        ColliderShape::Other(t) => t.clone(),
                    };
                    local.feature(format!(
                        "collider {shape} spawn {} die {} object {}",
                        c.spawn_on_collide,
                        c.die_on_collide,
                        c.object.is_some()
                    ));
                }
            }
            ModifierKind::Spawn(sp) => {
                local.feature(format!(
                    "spawn generations {} min {} max {}",
                    sp.generations, sp.min, sp.max
                ));
            }
            _ => {}
        }
    }
    for c in &s.controllers {
        match &c.kind {
            ControllerKind::Emitter {
                birth_rate,
                active,
                multi_target,
                ..
            } => {
                local.feature(format!(
                    "emitter ctlr cycle {} manager {} multi {}",
                    c.time.cycle(),
                    c.time.manager_controlled(),
                    multi_target.is_some()
                ));
                if let Some(b) = birth_rate {
                    local.feature(format!(
                        "birth rate keys kind {} count {}",
                        b.keys.kind,
                        b.keys.keys.len().min(3)
                    ));
                }
                match active {
                    Some(a) => local.feature(format!(
                        "emitter active value {} keys kind {} count {}",
                        a.value,
                        a.kind,
                        a.keys.len().min(3)
                    )),
                    None => local.feature("emitter active none"),
                }
            }
            ControllerKind::ModifierActive { value: Some(v), .. } => {
                local.feature(format!(
                    "modifier active ctlr keys kind {} count {} manager {}",
                    v.kind,
                    v.keys.len().min(3),
                    c.time.manager_controlled()
                ));
            }
            ControllerKind::ModifierFloat { value: Some(v), .. } => {
                local.feature(format!(
                    "{} keys kind {} manager {}",
                    c.type_name,
                    v.keys.kind,
                    c.time.manager_controlled()
                ));
            }
            _ => {}
        }
    }
    local.feature(format!("controllers {}", {
        let names: Vec<&str> = s.controllers.iter().map(|c| c.type_name.as_str()).collect();
        names.join(" ")
    }));
    // Drawing.
    let shader = s
        .shader
        .as_ref()
        .map(|sh| {
            format!(
                "{} type {} flags {:#010x}/{:#010x}",
                sh.type_name, sh.shader_type, sh.shader_flags, sh.shader_flags2
            )
        })
        .unwrap_or_else(|| "no shader".into());
    local.feature(format!("draw {shader}"));
    let alpha = s
        .alpha
        .map(|a| format!("alpha {:#06x} threshold {}", a.flags, a.threshold))
        .unwrap_or_else(|| "no alpha".into());
    local.feature(format!("draw {alpha}"));
    local.feature(format!(
        "draw zbuffer {}",
        s.zbuffer
            .map(|z| format!("{:#x}", z.flags))
            .unwrap_or_else(|| "none".into())
    ));
    local.feature(format!("draw texture {}", s.texture.is_some()));
    local.feature(format!("draw properties {}", s.property_types.join(" ")));
}

fn kind_name(k: &ModifierKind) -> &'static str {
    match k {
        ModifierKind::AgeDeath { .. } => "age",
        ModifierKind::BoundUpdate { .. } => "bound",
        ModifierKind::Position => "position",
        ModifierKind::Emitter(_) => "emitter",
        ModifierKind::Spawn(_) => "spawn",
        ModifierKind::Gravity(_) => "gravity",
        ModifierKind::Drag(_) => "drag",
        ModifierKind::Bomb(_) => "bomb",
        ModifierKind::GrowFade(_) => "growfade",
        ModifierKind::Rotation(_) => "rotation",
        ModifierKind::Color { .. } => "color",
        ModifierKind::SimpleColor(_) => "simplecolor",
        ModifierKind::Colliders(_) => "colliders",
        ModifierKind::Wind { .. } => "wind",
        ModifierKind::ParentVelocity { .. } => "parentvelocity",
        ModifierKind::StripUpdate { .. } => "strip",
        ModifierKind::Other(_) => "other",
    }
}

/// `particles`: every model the game can see.
pub fn survey(out: &mut impl Write, assets: &Assets) -> Result<(), CliError> {
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
                let mut mine = Totals::default();
                loop {
                    let i = next.fetch_add(1, Ordering::Relaxed);
                    let Some(&path) = paths.get(i) else { break };
                    mine.models += 1;
                    let bytes = match assets.read(path) {
                        Ok(Some(b)) => b,
                        Ok(None) => continue,
                        Err(e) => {
                            mine.failed.push((path.to_string(), e.to_string()));
                            continue;
                        }
                    };
                    let outcome = panic::catch_unwind(AssertUnwindSafe(|| {
                        let nif = Nif::parse(bytes)?;
                        let mut local = Local::default();
                        let any = survey_model(&nif, &mut local);
                        Ok::<_, nif::Error>((any, local))
                    }));
                    match outcome {
                        Ok(Ok((any, local))) => {
                            if any {
                                mine.with_particles += 1;
                                local.into_totals(path, &mut mine);
                            }
                        }
                        Ok(Err(e)) => mine.failed.push((path.to_string(), e.to_string())),
                        Err(_) => mine.failed.push((path.to_string(), "panicked".into())),
                    }
                }
                totals.lock().unwrap_or_else(|p| p.into_inner()).merge(mine);
            });
        }
    });
    panic::set_hook(previous_hook);
    let totals = totals.into_inner().unwrap_or_else(|p| p.into_inner());
    writeln!(
        out,
        "{} models, {} with particle blocks, {} unreadable",
        thousands(totals.models),
        thousands(totals.with_particles),
        totals.failed.len()
    )?;
    writeln!(out, "\nBlocks decoded exactly:")?;
    for (t, tally) in &totals.decoded {
        line(out, t, tally)?;
    }
    if totals.errors.is_empty() {
        writeln!(out, "\nNo particle block failed to decode.")?;
    } else {
        writeln!(out, "\nBlocks that failed to decode:")?;
        for (t, tally) in &totals.errors {
            line(out, t, tally)?;
        }
    }
    writeln!(out, "\nWhat the systems use:")?;
    for (t, tally) in &totals.features {
        line(out, t, tally)?;
    }
    for (path, e) in totals.failed.iter().take(10) {
        writeln!(out, "  unreadable: {path}: {e}")?;
    }
    Ok(())
}

fn line(out: &mut impl Write, label: &str, t: &Tally) -> Result<(), CliError> {
    writeln!(
        out,
        "  {label:<80} {:>7} in {:>5} models  e.g. {}",
        thousands(t.count),
        thousands(t.files),
        t.example
    )?;
    Ok(())
}

/// A model's embedded sequences (`NiControllerSequence`): name, cycle,
/// frequency, key range, and the particle controllers each drives.
fn sequences(out: &mut impl Write, nif: &Nif) -> Result<(), CliError> {
    let strings = &nif.header().strings;
    for i in 0..nif.blocks().len() {
        let t = nif.block_type(i);
        if t == "BSXFlags" {
            let b = nif.block_bytes(i);
            if let Some(v) = b.get(4..8) {
                writeln!(
                    out,
                    "BSXFlags {:#x}",
                    u32::from_le_bytes(v.try_into().unwrap())
                )?;
            }
        }
        if t != "NiControllerSequence" {
            continue;
        }
        let b = nif.block_bytes(i);
        let u32_at = |o: usize| {
            b.get(o..o + 4)
                .map(|s| u32::from_le_bytes(s.try_into().unwrap()))
        };
        let f32_at = |o: usize| {
            b.get(o..o + 4)
                .map(|s| f32::from_le_bytes(s.try_into().unwrap()))
        };
        let string = |o: usize| {
            u32_at(o)
                .and_then(|k| strings.get(k as usize))
                .cloned()
                .unwrap_or_default()
        };
        let count = u32_at(4).unwrap_or(0) as usize;
        let tail = 12 + 29 * count;
        writeln!(
            out,
            "sequence \"{}\" (block {i}): {} controlled blocks, weight {}, cycle {}, frequency {}, keys {}..{}",
            string(0),
            count,
            f32_at(tail).unwrap_or(0.0),
            u32_at(tail + 8).unwrap_or(0),
            f32_at(tail + 12).unwrap_or(0.0),
            f32_at(tail + 16).unwrap_or(0.0),
            f32_at(tail + 20).unwrap_or(0.0)
        )?;
        for k in 0..count.min(256) {
            let at = 12 + 29 * k;
            let interp = u32_at(at).map(|v| v as i32).unwrap_or(-1);
            let itype = usize::try_from(interp)
                .ok()
                .filter(|&x| x < nif.blocks().len())
                .map(|x| nif.block_type(x).to_string())
                .unwrap_or_else(|| "none".into());
            let ctype = string(at + 17);
            if !ctype.contains("PSys") && k > 3 {
                continue;
            }
            writeln!(
                out,
                "    {} on \"{}\" id \"{}\" interp {} ({})",
                ctype,
                string(at + 9),
                string(at + 21),
                interp,
                itype
            )?;
        }
    }
    Ok(())
}

/// `particles [PATH [run SECONDS]]`.
pub fn run(out: &mut impl Write, assets: &Assets, args: &[String]) -> Result<(), CliError> {
    match args {
        [] => survey(out, assets),
        [path] => describe(out, assets, path),
        [path, run, seconds] if run == "run" => {
            let seconds: f32 = seconds
                .parse()
                .map_err(|_| CliError::Usage(format!("'{seconds}' isn't a number of seconds")))?;
            simulate(out, assets, path, seconds)
        }
        _ => Err(CliError::Usage(
            "particles [PATH [run SECONDS]]".to_string(),
        )),
    }
}

fn read_model(assets: &Assets, path: &str) -> Result<(String, Nif), CliError> {
    let normalized = path.replace('/', "\\").to_ascii_lowercase();
    let normalized = if normalized.starts_with("meshes\\") {
        normalized
    } else {
        format!("meshes\\{normalized}")
    };
    let bytes = assets
        .read(&normalized)
        .map_err(|e| CliError::Usage(format!("{normalized}: {e}")))?
        .ok_or_else(|| CliError::Usage(format!("the game has no {normalized}")))?;
    let nif = Nif::parse(bytes).map_err(|e| CliError::Usage(format!("{normalized}: {e}")))?;
    Ok((normalized, nif))
}

/// `particles <PATH> run <SECONDS>`: the systems run at 60 frames a second
/// with the model at the origin (as a placed object: the top node's
/// transform left out), each second's particles summed up: how many, their
/// sizes on screen (2 × radius × size), colours and alpha as the game
/// packs them, where they are.
fn simulate(
    out: &mut impl Write,
    assets: &Assets,
    path: &str,
    seconds: f32,
) -> Result<(), CliError> {
    let (normalized, nif) = read_model(assets, path)?;
    let systems: Vec<std::sync::Arc<ParticleSystem>> = nif
        .particle_systems(false)
        .map_err(|e| CliError::Usage(format!("{normalized}: {e}")))?
        .into_iter()
        .map(std::sync::Arc::new)
        .collect();
    let sequences = nif.particle_sequences().unwrap_or_default();
    let all = nif.sequences().unwrap_or_default();
    let playing: Vec<(usize, bool)> = preview::cell::placed_sequences(&all, false)
        .iter()
        .filter_map(|p| {
            sequences
                .iter()
                .position(|s| s.name.eq_ignore_ascii_case(&p.sequence.name))
                .map(|i| (i, p.runs))
        })
        .collect();
    writeln!(
        out,
        "{normalized}: {} particle systems; sequences playing: {}",
        systems.len(),
        if playing.is_empty() {
            "none".to_string()
        } else {
            playing
                .iter()
                .map(|&(i, runs)| {
                    format!("{}{}", sequences[i].name, if runs { "" } else { " (held)" })
                })
                .collect::<Vec<_>>()
                .join(", ")
        }
    )?;
    let motion = preview::cell::placed_sequences(&all, false)
        .into_iter()
        .filter(|p| !p.sequence.tracks.is_empty())
        .map(|p| (std::sync::Arc::new(p.sequence), p.runs))
        .collect();
    let unread: Vec<&String> = playing
        .iter()
        .flat_map(|&(i, _)| &sequences[i].unread_movers)
        .collect();
    if !unread.is_empty() {
        writeln!(
            out,
            "moved by interpolators not read here (stays put): {}",
            unread
                .iter()
                .map(|s| s.as_str())
                .collect::<Vec<_>>()
                .join(", ")
        )?;
    }
    let mut placed = world::particles::PlacedParticles::new(&systems, sequences, playing, 1);
    placed.set_motion(motion);
    let step = 1.0 / 60.0;
    let frames = (seconds / step).round().max(0.0) as usize;
    for frame in 0..=frames {
        let t = frame as f32 * step;
        placed.update(t, &nif::Transform::IDENTITY, [0.0; 3]);
        if frame % 60 != 0 {
            continue;
        }
        writeln!(out, "t = {t:.0} s")?;
        for system in &placed.systems {
            let ps = system.active();
            if ps.is_empty() {
                writeln!(out, "  {}: none", system.def.name)?;
                continue;
            }
            let range = |f: &dyn Fn(&world::particles::Particle) -> f32| {
                let lo = ps.iter().map(f).fold(f32::MAX, f32::min);
                let hi = ps.iter().map(f).fold(f32::MIN, f32::max);
                format!("{}..{}", round(lo), round(hi))
            };
            let color = |p: &world::particles::Particle| {
                let [b, g, r, a] = world::particles::pack_color(p.color);
                (r, g, b, a)
            };
            let (r, g, b, a) = color(&ps[0]);
            writeln!(
                out,
                "  {}: {} particles; sides {}; alpha {}; first ({r},{g},{b},{a}); x {} y {} z {}",
                system.def.name,
                ps.len(),
                range(&|p| 2.0 * p.radius * p.size),
                range(&|p| f32::from(world::particles::pack_color(p.color)[3])),
                range(&|p| p.position[0]),
                range(&|p| p.position[1]),
                range(&|p| p.position[2]),
            )?;
        }
    }
    Ok(())
}

/// `particles <PATH>`: one model's systems in full.
pub fn describe(out: &mut impl Write, assets: &Assets, path: &str) -> Result<(), CliError> {
    let (normalized, nif) = read_model(assets, path)?;
    let systems = nif
        .particle_systems(false)
        .map_err(|e| CliError::Usage(format!("{normalized}: {e}")))?;
    writeln!(out, "{normalized}: {} particle systems", systems.len())?;
    sequences(out, &nif)?;
    if let Ok(parsed) = nif.particle_sequences() {
        for s in parsed.iter().filter(|s| !s.tracks.is_empty()) {
            writeln!(out, "sequence \"{}\" hands particle controllers:", s.name)?;
            for t in &s.tracks {
                let keys = match &t.value {
                    nif::particles::TrackValue::Float(f) => format!(
                        "value {} keys {:?}",
                        round(f.value),
                        f.keys
                            .keys
                            .iter()
                            .map(|k| (round(k.time), round(k.value)))
                            .collect::<Vec<_>>()
                    ),
                    nif::particles::TrackValue::Bool(interp) => format!(
                        "value {} keys {:?}",
                        interp.value,
                        interp
                            .keys
                            .iter()
                            .map(|(t, v)| (round(*t), *v))
                            .collect::<Vec<_>>()
                    ),
                };
                writeln!(
                    out,
                    "    {} \"{}\" \"{}\" ({}): {} {}",
                    t.controller_type,
                    t.node,
                    t.controller_id,
                    t.interpolator_id,
                    track_value_name(&t.value),
                    keys
                )?;
            }
        }
    }
    for s in &systems {
        let path: Vec<&str> = s.nodes.iter().map(|(n, _)| n.as_str()).collect();
        writeln!(
            out,
            "\n{} \"{}\" (block {}), under {}",
            s.type_name,
            s.name,
            s.block,
            path.join(" > ")
        )?;
        let t = s.transform();
        writeln!(
            out,
            "  in the model at ({}, {}, {}) scale {}; {}",
            round(t.translation[0]),
            round(t.translation[1]),
            round(t.translation[2]),
            round(t.scale),
            if s.world_space {
                "particles live in world space"
            } else {
                "particles move with the model"
            }
        )?;
        writeln!(out, "  data: {:?}", s.data)?;
        for m in &s.modifiers {
            writeln!(
                out,
                "  modifier \"{}\" order {} active {}: {:?}",
                m.name, m.order, m.active, m.kind
            )?;
        }
        for c in &s.controllers {
            writeln!(
                out,
                "  controller {} flags {:#06x} frequency {} phase {} keys {}..{}: {:?}",
                c.type_name,
                c.time.flags,
                round(c.time.frequency),
                round(c.time.phase),
                round(c.time.start),
                round(c.time.stop),
                c.kind
            )?;
        }
        writeln!(
            out,
            "  drawn with {:?}, texture {:?}",
            s.property_types, s.texture
        )?;
        if let Some(sh) = &s.shader {
            writeln!(
                out,
                "  shader {} flags {:#010x}/{:#010x} falloff {:?}",
                sh.type_name, sh.shader_flags, sh.shader_flags2, sh.falloff
            )?;
        }
        if let Some(a) = s.alpha {
            writeln!(
                out,
                "  alpha flags {:#06x} threshold {}",
                a.flags, a.threshold
            )?;
        }
        if let Some(m) = &s.material {
            writeln!(out, "  material {m:?}")?;
        }
        if let Some(z) = s.zbuffer {
            writeln!(out, "  depth flags {:#x}", z.flags)?;
        }
    }
    Ok(())
}

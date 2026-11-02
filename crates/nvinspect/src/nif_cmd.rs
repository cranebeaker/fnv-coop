//! Commands for NIF meshes, loose or inside archives.

use std::collections::BTreeMap;
use std::io::Write;
use std::panic::{self, AssertUnwindSafe};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Mutex;
use std::time::Instant;

use bsa::Archive;
use nif::{version_string, Nif};

use crate::fmt::{human_bytes, thousands, truncate};
use crate::{expect_args, file_name_of, CliError, Options};

enum Command {
    Info,
    Blocks,
    Block(usize),
    Obj(Option<String>),
}

fn parse_command(command: &str, rest: &[String]) -> Result<Command, CliError> {
    match command {
        "info" => {
            expect_args(command, rest, 0, 0)?;
            Ok(Command::Info)
        }
        "blocks" => {
            expect_args(command, rest, 0, 0)?;
            Ok(Command::Blocks)
        }
        "block" => {
            expect_args(command, rest, 1, 0)?;
            let index = rest[0]
                .parse()
                .map_err(|_| CliError::Usage(format!("'{}' isn't a block number", rest[0])))?;
            Ok(Command::Block(index))
        }
        "obj" => {
            expect_args(command, rest, 0, 1)?;
            Ok(Command::Obj(rest.first().cloned()))
        }
        other => Err(CliError::Usage(format!(
            "'{other}' isn't a mesh command; for .nif files use info, blocks, block or obj"
        ))),
    }
}

/// `block <N>`: one block's bytes, four at a time, as hex, integer and
/// float, for working out layouts this reader doesn't decode yet.
fn block_dump(out: &mut impl Write, nif: &Nif, index: usize) -> Result<(), CliError> {
    if index >= nif.blocks().len() {
        return Err(CliError::Usage(format!(
            "the file has {} blocks (0 to {})",
            nif.blocks().len(),
            nif.blocks().len().saturating_sub(1)
        )));
    }
    let bytes = nif.block_bytes(index);
    writeln!(
        out,
        "Block {index}: {} ({} bytes)",
        nif.block_type(index),
        bytes.len()
    )?;
    for (k, chunk) in bytes.chunks(4).enumerate() {
        let hex: Vec<String> = chunk.iter().map(|b| format!("{b:02x}")).collect();
        if chunk.len() == 4 {
            let v = [chunk[0], chunk[1], chunk[2], chunk[3]];
            writeln!(
                out,
                "  +{:<5} {}  {:>12}  {}",
                k * 4,
                hex.join(" "),
                i32::from_le_bytes(v),
                f32::from_le_bytes(v)
            )?;
        } else {
            writeln!(out, "  +{:<5} {}", k * 4, hex.join(" "))?;
        }
    }
    Ok(())
}

/// Runs a command on a loose .nif file.
pub fn run_file(
    out: &mut impl Write,
    path: &Path,
    command: &str,
    rest: &[String],
    options: &Options,
) -> Result<(), CliError> {
    let command = parse_command(command, rest)?;
    let nif = Nif::open(path).map_err(|e| match e {
        nif::Error::Io(io) => CliError::Open {
            path: path.display().to_string(),
            message: io.to_string(),
        },
        other => CliError::InFile {
            path: path.display().to_string(),
            message: other.to_string(),
        },
    })?;
    let label = file_name_of(path);
    match command {
        Command::Info => info(out, &nif, &label),
        Command::Blocks => blocks(out, &nif, options),
        Command::Block(index) => block_dump(out, &nif, index),
        Command::Obj(output) => {
            let default = default_obj_name(&label);
            export_obj(out, &nif, &label, output, &default, options.force)
        }
    }
}

/// Parses a .nif read from an archive.
pub fn parse_archived(bytes: Vec<u8>, path: &str) -> Result<Nif, CliError> {
    Nif::parse(bytes).map_err(|e| CliError::InFile {
        path: path.to_string(),
        message: e.to_string(),
    })
}

pub fn default_obj_name(nif_name: &str) -> String {
    let file = nif_name.rsplit(['\\', '/']).next().unwrap_or(nif_name);
    let stem = file.rsplit_once('.').map_or(file, |(stem, _)| stem);
    format!("{stem}.obj")
}

pub fn info(out: &mut impl Write, nif: &Nif, label: &str) -> Result<(), CliError> {
    info_with(out, nif, label, None)
}

/// `nif <PATH>` on a Data folder: finds a model wherever the game would load
/// it from (a path as written in a record, relative to `meshes\`, works
/// too) and describes it, checking its textures.
pub fn info_in_game(
    out: &mut impl Write,
    assets: &assets::Assets,
    wanted: &str,
) -> Result<(), CliError> {
    let path = assets::mesh_path(wanted);
    let Some(source) = assets.locate(&path) else {
        return Err(CliError::NotFound(format!(
            "the game has no model at {path}. `find` shows where a file would come from."
        )));
    };
    let bytes = source.read().map_err(|e| CliError::InFile {
        path: path.clone(),
        message: e.to_string(),
    })?;
    let nif = Nif::parse(bytes)?;
    writeln!(out, "Loaded from:  {}", source.describe())?;
    info_with(out, &nif, &path, Some(assets))
}

/// Like [`info`], and with the game's files at hand, says whether each
/// texture can be found.
pub fn info_with(
    out: &mut impl Write,
    nif: &Nif,
    label: &str,
    assets: Option<&assets::Assets>,
) -> Result<(), CliError> {
    let h = nif.header();
    writeln!(
        out,
        "File:         {label} ({})",
        human_bytes(nif.file_size() as u64)
    )?;
    writeln!(
        out,
        "Format:       Gamebryo {}, user version {}, Bethesda version {}",
        version_string(h.version),
        h.user_version,
        h.bs_version
    )?;
    if !h.author.is_empty() {
        writeln!(out, "Author:       {}", h.author)?;
    }
    let counts = nif.type_counts();
    writeln!(
        out,
        "Blocks:       {} of {} types",
        thousands(nif.blocks().len()),
        counts.len()
    )?;
    // Animations (.kf): each sequence, and how far its accumulation root
    // carries the actor.
    if let Ok(sequences) = nif.sequences() {
        for s in &sequences {
            writeln!(
                out,
                "Sequence:     {} ({:.2} s, {}, {} bones moved, accumulation root {})",
                s.name,
                s.stop - s.start,
                if s.looping { "looping" } else { "once" },
                s.tracks.len(),
                s.accum_root.as_deref().unwrap_or("none")
            )?;
            // The moments it names (`Hit`, `end`).
            if let Ok(keys) = nif.text_keys() {
                if !keys.is_empty() {
                    let keys: Vec<String> =
                        keys.iter().map(|(t, k)| format!("{k} {t:.2} s")).collect();
                    writeln!(out, "              text keys: {}", keys.join(", "))?;
                }
            }
            // A short list of bones (a pose of a few): which.
            if s.tracks.len() <= 12 {
                let names: Vec<&str> = s.tracks.iter().map(|t| t.node.as_str()).collect();
                writeln!(out, "              moves {}", names.join(", "))?;
            }
            // The bones' blend priorities (the game's blending puts the
            // highest first) and the text keys the game times groups by.
            let mut priorities: Vec<u8> = s.tracks.iter().map(|t| t.priority).collect();
            priorities.sort_unstable();
            priorities.dedup();
            if !priorities.is_empty() {
                let list: Vec<String> = priorities
                    .iter()
                    .map(|p| {
                        let n = s.tracks.iter().filter(|t| t.priority == *p).count();
                        format!("{p} ({n} bones)")
                    })
                    .collect();
                writeln!(out, "              priorities {}", list.join(", "))?;
            }
            if !s.text_keys.is_empty() {
                let keys: Vec<String> = s
                    .text_keys
                    .iter()
                    .map(|(t, k)| format!("{t:.3} {k}"))
                    .collect();
                writeln!(out, "              text keys: {}", keys.join("; "))?;
            }
            // Its text keys (what the engine does at each moment: sounds,
            // start and end), when there are only a few.
            if !s.text_keys.is_empty() && s.text_keys.len() <= 16 {
                let keys: Vec<String> = s
                    .text_keys
                    .iter()
                    .map(|(t, k)| format!("{t:.2} s \"{k}\""))
                    .collect();
                writeln!(out, "              text keys: {}", keys.join(", "))?;
            }
            if let (Some(d), Some(speed)) = (s.root_travel(), s.root_speed()) {
                writeln!(
                    out,
                    "              moves the actor {:.1}, {:.1}, {:.1} units: {speed:.1} units a second",
                    d[0], d[1], d[2]
                )?;
                // How evenly: the root's progress at the quarter points, as
                // shares of the whole travel (0.25, 0.5, 0.75 when even).
                let length = s.stop - s.start;
                let along = (d[0] * d[0] + d[1] * d[1]).sqrt().max(1e-6);
                let shares: Vec<String> = [0.25, 0.5, 0.75]
                    .iter()
                    .filter_map(|f| s.root_offset(length * f))
                    .map(|o| format!("{:.3}", (o[0] * d[0] + o[1] * d[1]) / along / along))
                    .collect();
                writeln!(
                    out,
                    "              progress at quarter points: {}",
                    shares.join(", ")
                )?;
            }
            // Which way the root and the body face at the start and end
            // (sitting down and getting up turn the body).
            let root = s.accum_root.clone().unwrap_or_default();
            for bone in [root.clone(), format!("{root} NonAccum")] {
                let length = s.stop - s.start;
                if let (Some(a), Some(b)) =
                    (s.bone_heading(&bone, 0.0), s.bone_heading(&bone, length))
                {
                    if a.abs() > 1e-3 || b.abs() > 1e-3 {
                        writeln!(
                            out,
                            "              {bone} faces {:.1}° at the start, {:.1}° at the end",
                            a.to_degrees(),
                            b.to_degrees()
                        )?;
                    }
                }
            }
        }
    }

    let scene = match nif.scene() {
        Ok(scene) => scene,
        // Still worth showing the block list for meshes from other games.
        Err(nif::Error::Unsupported(reason)) => {
            writeln!(out, "Meshes:       can't be read: {reason}")?;
            return block_type_summary(out, &counts);
        }
        Err(e) => return Err(e.into()),
    };
    writeln!(
        out,
        "Meshes:       {} visible, {} triangles",
        scene.meshes.len(),
        thousands(scene.triangle_count())
    )?;
    // A skeleton (no meshes, or named one: people's carries a one-triangle
    // marker): its bones, as a tree, with where each sits relative to its
    // parent.
    if scene.meshes.is_empty() || label.to_ascii_lowercase().ends_with("skeleton.nif") {
        if let Ok(bones) = nif.skeleton() {
            if !bones.is_empty() {
                writeln!(out, "\nNodes ({}):", bones.len())?;
            }
            for (i, b) in bones.iter().enumerate() {
                let mut depth = 0;
                let mut p = b.parent;
                while let Some(up) = p {
                    depth += 1;
                    p = bones[up].parent;
                }
                let t = b.local.translation;
                // Turned: where its x, y and z axes point in the parent's.
                let r = b.local.rotation;
                let turned = (0..3)
                    .any(|i| (0..3).any(|j| (r[i][j] - f32::from(u8::from(i == j))).abs() > 1e-3));
                let turn = if turned {
                    format!(
                        "  axes x({:.2},{:.2},{:.2}) y({:.2},{:.2},{:.2}) z({:.2},{:.2},{:.2})",
                        r[0][0],
                        r[1][0],
                        r[2][0],
                        r[0][1],
                        r[1][1],
                        r[2][1],
                        r[0][2],
                        r[1][2],
                        r[2][2]
                    )
                } else {
                    String::new()
                };
                writeln!(
                    out,
                    "  {:>3} {}{}  at ({:.1}, {:.1}, {:.1}){turn}",
                    i,
                    "  ".repeat(depth.min(20)),
                    b.name,
                    t[0],
                    t[1],
                    t[2]
                )?;
            }
            ragdoll_summary(out, nif, &bones)?;
        }
    }
    if !scene.meshes.is_empty() {
        writeln!(out)?;
        writeln!(
            out,
            "  {:<28} {:>9} {:>7}  Diffuse texture",
            "Mesh", "Triangles", "Verts"
        )?;
        for mesh in &scene.meshes {
            let mut notes = Vec::new();
            if let Some(alpha) = mesh.alpha {
                if alpha.testing() {
                    notes.push("alpha-tested");
                }
                if alpha.blending() {
                    notes.push("alpha-blended");
                }
            }
            if mesh.double_sided {
                notes.push("double-sided");
            }
            if mesh.skinned {
                notes.push("skinned");
            }
            let notes = if notes.is_empty() {
                String::new()
            } else {
                format!("  [{}]", notes.join(", "))
            };
            writeln!(
                out,
                "  {:<28} {:>9} {:>7}  {}{notes}",
                truncate(&mesh.name, 28),
                thousands(mesh.triangles.len()),
                thousands(mesh.positions.len()),
                mesh.diffuse_texture().unwrap_or("(none)")
            )?;
        }
    }
    if !scene.meshes.is_empty() {
        materials(out, &scene, assets)?;
    }
    // Segmented shapes (distant-object blocks): each segment's triangles
    // and the outdoor square its middle stands in.
    for mesh in &scene.meshes {
        let segments = nif.segments(mesh.block);
        if segments.is_empty() {
            continue;
        }
        writeln!(out, "\nSegments of {} ({}):", mesh.name, segments.len())?;
        for (i, s) in segments.iter().enumerate() {
            let tris = mesh
                .triangles
                .iter()
                .skip(s.first_triangle as usize)
                .take(s.triangles as usize);
            let (mut sum, mut n) = ([0.0f32; 3], 0.0f32);
            for t in tris {
                for &v in t {
                    let p = mesh.transform.apply_point(mesh.positions[v as usize]);
                    for k in 0..3 {
                        sum[k] += p[k];
                    }
                    n += 1.0;
                }
            }
            let middle = sum.map(|c| c / n.max(1.0));
            let square = world::square_of(middle);
            writeln!(
                out,
                "  {i:>2}: flags {:#04x}, {} triangles from {}, middle in square {},{}",
                s.flags, s.triangles, s.first_triangle, square.0, square.1
            )?;
        }
    }
    if let Some(t) = &scene.root_transform {
        writeln!(
            out,
            "\nTop node:     {} (viewers apply this; the game replaces it when the model is placed)",
            preview::cell::describe_transform(t)
        )?;
    }
    if !scene.unhandled.is_empty() {
        let list: Vec<String> = scene
            .unhandled
            .iter()
            .map(|(t, n)| format!("{t} ({n})"))
            .collect();
        writeln!(out, "\nNot drawn yet: {}", list.join(", "))?;
    }
    if scene.invalid_references > 0 || scene.dropped_triangles > 0 {
        writeln!(
            out,
            "Problems:     {} broken block references, {} triangles pointing past their vertices",
            scene.invalid_references, scene.dropped_triangles
        )?;
    }
    camera_summary(out, nif)?;
    collision_summary(out, nif, &scene)?;

    block_type_summary(out, &counts)
}

/// A skeleton's ragdoll (`nif::ragdoll`): each body (its bone, mass,
/// capsule and how far it sits from the bone), and each joint with its
/// limits and the file's own pose against them: the two pivots should meet
/// and every angle fall inside its limits, which checks how they're read.
fn ragdoll_summary(out: &mut impl Write, nif: &Nif, bones: &[nif::Bone]) -> Result<(), CliError> {
    let ragdoll = match nif.ragdoll() {
        Ok(Some(r)) => r,
        Ok(None) => return Ok(()),
        Err(e) => {
            writeln!(out, "\nRagdoll:      can't be read: {e}")?;
            return Ok(());
        }
    };
    let rig = preview::ragdoll::RagdollRig::new(bones, ragdoll.clone());
    writeln!(out, "\nRagdoll bodies ({}):", ragdoll.bodies.len())?;
    for (b, offset) in ragdoll.bodies.iter().zip(&rig.offsets) {
        let capsule = b.capsule.map_or("no shape".to_string(), |(a, c, r)| {
            let len =
                ((a[0] - c[0]).powi(2) + (a[1] - c[1]).powi(2) + (a[2] - c[2]).powi(2)).sqrt();
            format!("capsule {len:.1} long, radius {r:.1}")
        });
        let o = offset.translation;
        writeln!(
            out,
            "  {:<20} part {:>2} (push x{:.2})  mass {:>5.1}  {capsule}  from the bone ({:.1}, {:.1}, {:.1})  friction {:.2}, damping {:.2}/{:.2}",
            b.bone_name,
            b.part,
            world::combat::death_push_share(b.part),
            b.mass,
            o[0],
            o[1],
            o[2],
            b.friction,
            b.linear_damping,
            b.angular_damping
        )?;
    }
    writeln!(
        out,
        "\nRagdoll joints ({}), in the file's pose:",
        ragdoll.joints.len()
    )?;
    let gaps = preview::ragdoll::file_pose_gaps(&ragdoll);
    let angles = preview::ragdoll::file_pose_angles(&ragdoll);
    let deg = |r: f32| r.to_degrees();
    let check = |v: f32, lo: f32, hi: f32| {
        if v >= lo - 1e-3 && v <= hi + 1e-3 {
            "ok"
        } else {
            "OUTSIDE"
        }
    };
    for ((j, gap), angle) in ragdoll.joints.iter().zip(gaps).zip(angles) {
        let name = |k: usize| ragdoll.bodies[j.bodies[k]].bone_name.as_str();
        let limits = match (j.limit, angle) {
            (
                nif::JointLimit::Ragdoll {
                    cone,
                    plane_range,
                    twist_range,
                    ..
                },
                physics::ragdoll::JointAngles::Cone {
                    cone: c,
                    plane,
                    twist,
                },
            ) => format!(
                "cone {:.0}° of {:.0}° {}, plane {:.0}° in {:.0}..{:.0}° {}, twist {:.0}° in {:.0}..{:.0}° {}",
                deg(c),
                deg(cone),
                check(c, 0.0, cone),
                deg(plane),
                deg(plane_range.0),
                deg(plane_range.1),
                check(plane, plane_range.0, plane_range.1),
                deg(twist),
                deg(twist_range.0),
                deg(twist_range.1),
                check(twist, twist_range.0, twist_range.1)
            ),
            (
                nif::JointLimit::Hinge { range, .. },
                physics::ragdoll::JointAngles::Hinge { misaligned, angle },
            ) => format!(
                "hinge {:.0}° in {:.0}..{:.0}° {}, axles {:.1}° apart",
                deg(angle),
                deg(range.0),
                deg(range.1),
                check(angle, range.0, range.1),
                deg(misaligned)
            ),
            _ => "free".to_string(),
        };
        writeln!(
            out,
            "  {:<16} on {:<16} pivots {gap:.2} apart; {limits}",
            name(0),
            name(1)
        )?;
    }
    // Dropped: standing in the file's pose on a flat floor, thrown as a
    // killing blow from the front with a hunting shotgun would be (kill
    // impulse 25: 437 units a second, `world::combat::death_push`), and
    // left for up to ten seconds.
    let mut floor = physics::Collider::new();
    floor.add(
        &[
            [-2000.0, -2000.0, 0.0],
            [2000.0, -2000.0, 0.0],
            [2000.0, 2000.0, 0.0],
            [-2000.0, 2000.0, 0.0],
        ],
        &[[0, 1, 2], [0, 2, 3]],
    );
    let mut pose = preview::ragdoll::file_pose(bones);
    // The file has `Bip01` at the pelvis; standing, the feet are at 0.
    let lowest = ragdoll
        .bodies
        .iter()
        .filter_map(|b| {
            let (a, c, r) = b.capsule?;
            Some(b.frame.apply_point(a)[2].min(b.frame.apply_point(c)[2]) - r)
        })
        .fold(f32::INFINITY, f32::min);
    for t in &mut pose {
        t.translation[2] -= lowest;
    }
    let mut sim = rig.start(&pose, &nif::Transform::IDENTITY);
    let shares: Vec<f32> = ragdoll
        .bodies
        .iter()
        .map(|b| world::combat::death_push_share(b.part))
        .collect();
    let speed = 25.0 * 2.5 * nif::collision::HAVOK_SCALE;
    sim.throw_from(
        world::combat::death_push_origin([0.0, 0.0, 90.0], [0.0, -1.0, 0.0]),
        speed,
        &shares,
    );
    let mut seconds = 0.0;
    while seconds < 10.0 && !sim.asleep {
        sim.step(&floor);
        seconds += physics::ragdoll::STEP;
    }
    writeln!(
        out,
        "\nDropped on a flat floor: {} after {seconds:.1} s; bodies' heights (centre):",
        if sim.asleep {
            "at rest"
        } else {
            "still moving"
        }
    )?;
    let heights: Vec<String> = ragdoll
        .bodies
        .iter()
        .enumerate()
        .map(|(i, b)| {
            let (r, o) = sim.frame(i);
            let c = nif::math::mat_vec(&r, b.center);
            format!(
                "{} {:.0}",
                b.bone_name.trim_start_matches("Bip01 "),
                o[2] + c[2]
            )
        })
        .collect();
    writeln!(out, "  {}", heights.join(", "))?;
    let worst = (0..ragdoll.joints.len())
        .map(|j| sim.joint_gap(j))
        .fold(0.0f32, f32::max);
    let ((v, vi), (w, wi)) = sim.fastest();
    writeln!(
        out,
        "  joints' widest gap {worst:.2}; fastest {v:.1} units/s ({}), {w:.2} rad/s ({})",
        ragdoll.bodies[vi].bone_name, ragdoll.bodies[wi].bone_name
    )?;
    Ok(())
}

/// A V.A.T.S. camera model (`meshes\vatscameras\*.nif`): where its keyed
/// translation takes the camera and its field of view over time.
fn camera_summary(out: &mut impl Write, nif: &Nif) -> Result<(), CliError> {
    let Some(cam) = nif.camera_model()? else {
        return Ok(());
    };
    let kind = |k: &nif::camera::Keys<[f32; 3]>| match k {
        nif::camera::Keys::Linear(v) => format!("{} linear", v.len()),
        nif::camera::Keys::Quadratic(v) => format!("{} quadratic", v.len()),
        nif::camera::Keys::Tcb(v) => format!("{} TCB", v.len()),
        nif::camera::Keys::Step(v) => format!("{} step", v.len()),
    };
    let f = cam.frustum;
    writeln!(
        out,
        "\nV.A.T.S. camera {}: frustum left {:.4} right {:.4} top {:.4} bottom {:.4} near {} far {} ({:.1}° wide at 4:3)",
        cam.root_name,
        f.left,
        f.right,
        f.top,
        f.bottom,
        f.near,
        f.far,
        2.0 * f.right.atan().to_degrees()
    )?;
    match &cam.translation {
        Some((t, keys)) => writeln!(
            out,
            "  path: {} keys, controller flags {:#06x}, frequency {}, phase {}, {}–{} s",
            kind(keys),
            t.flags,
            t.frequency,
            t.phase,
            t.start,
            t.stop
        )?,
        None => writeln!(
            out,
            "  no keyed path: stays at ({:.1}, {:.1}, {:.1})",
            cam.root_translation[0], cam.root_translation[1], cam.root_translation[2]
        )?,
    }
    if let Some((t, keys)) = &cam.fov {
        writeln!(
            out,
            "  field of view: {} keys, {}–{} s",
            keys.len(),
            t.start,
            t.stop
        )?;
    }
    match &cam.rotation {
        Some((_, nif::camera::Rotation::Euler(axes))) => writeln!(
            out,
            "  turn: X, Y, Z angle keys {}, {}, {} (laid on the look at the target)",
            axes[0].as_ref().map_or(0, |k| k.len()),
            axes[1].as_ref().map_or(0, |k| k.len()),
            axes[2].as_ref().map_or(0, |k| k.len())
        )?,
        Some((_, nif::camera::Rotation::Quaternion { kind, keys })) => writeln!(
            out,
            "  turn: {keys} quaternion keys of type {kind} (not read: the camera isn't turned by them)"
        )?,
        None => {}
    }
    let end = cam
        .translation
        .as_ref()
        .map_or(0.0, |(t, _)| t.stop)
        .max(cam.fov.as_ref().map_or(0.0, |(t, _)| t.stop))
        .max(cam.rotation.as_ref().map_or(0.0, |(t, _)| t.stop));
    let steps = 8;
    for i in 0..=steps {
        let time = end * i as f32 / steps as f32;
        let p = cam.translation_at(time, None);
        let fr = cam.frustum_at(time, None);
        let turn = cam.angles_at(time, None).map_or(String::new(), |a| {
            format!(
                ", turned {:.1}°, {:.1}°, {:.1}°",
                a[0].to_degrees(),
                a[1].to_degrees(),
                a[2].to_degrees()
            )
        });
        writeln!(
            out,
            "  {time:>6.2} s: at ({:>8.1}, {:>8.1}, {:>8.1}), {:.1}° wide{turn}",
            p[0],
            p[1],
            p[2],
            2.0 * fr.right.atan().to_degrees()
        )?;
    }
    Ok(())
}

/// The model's collision (Havok shapes, in game units, as `nif::collision`
/// reads them): its pieces by kind and layer, and its bounds next to the
/// visible meshes' bounds, which should roughly agree.
fn collision_summary(out: &mut impl Write, nif: &Nif, scene: &nif::Scene) -> Result<(), CliError> {
    // Furniture: where it's used from.
    for m in nif.furniture_markers() {
        writeln!(
            out,
            "\nFurniture:    marker {} at ({:.1}, {:.1}, {:.1}), heading {:.3}",
            m.marker, m.offset[0], m.offset[1], m.offset[2], m.heading
        )?;
    }
    // A skeleton's box for its actor's collision.
    if let Some(b) = nif.bound() {
        let [x, y, z] = b.half_extents;
        writeln!(
            out,
            "\nBound:        centre ({:.1}, {:.1}, {:.1}), half extents {x:.1} × {y:.1} × {z:.1} \
             (creature collision radius {:.1})",
            b.center[0],
            b.center[1],
            b.center[2],
            world::combat_ai::creature_radius(b.half_extents, 1.0)
        )?;
    }
    let collision = match nif.collision() {
        Ok(c) => c,
        Err(e) => {
            writeln!(out, "\nCollision:    can't be read: {e}")?;
            return Ok(());
        }
    };
    if collision.parts.is_empty() && collision.unhandled.is_empty() {
        return Ok(());
    }
    let mut kinds: BTreeMap<String, usize> = BTreeMap::new();
    let mut lo = [f32::INFINITY; 3];
    let mut hi = [f32::NEG_INFINITY; 3];
    let mut grow = |p: [f32; 3]| {
        for k in 0..3 {
            lo[k] = lo[k].min(p[k]);
            hi[k] = hi[k].max(p[k]);
        }
    };
    for part in &collision.parts {
        let kind = match &part.shape {
            nif::CollisionShape::Triangles {
                vertices,
                triangles,
            } => {
                for t in triangles {
                    for &i in t {
                        grow(vertices[i as usize]);
                    }
                }
                "triangle mesh"
            }
            nif::CollisionShape::Convex { vertices, .. } => {
                vertices.iter().for_each(|&v| grow(v));
                "convex hull"
            }
            nif::CollisionShape::Sphere { center, radius } => {
                grow(center.map(|c| c - radius));
                grow(center.map(|c| c + radius));
                "sphere"
            }
            nif::CollisionShape::Capsule { a, b, radius } => {
                for p in [a, b] {
                    grow(p.map(|c| c - radius));
                    grow(p.map(|c| c + radius));
                }
                "capsule"
            }
        };
        let motion = if part.dynamic {
            ", moves".to_string()
        } else if part.keyframed {
            // What the animation moves it through, and the collision
            // object's flags (0x80: synced to its node every update).
            let chain: Vec<&str> = part.nodes.iter().map(|(n, _)| n.as_str()).collect();
            format!(
                ", animated on {} (collision flags 0x{:x})",
                chain.join(" > "),
                part.flags
            )
        } else {
            String::new()
        };
        let solid = if nif::collision::layers::blocks_walking(part.layer) {
            ""
        } else {
            ", not solid to people"
        };
        *kinds
            .entry(format!(
                "{kind} (layer {} {}{motion}{solid})",
                part.layer,
                nif::collision::layers::name(part.layer)
            ))
            .or_default() += 1;
    }
    let list: Vec<String> = kinds.iter().map(|(k, n)| format!("{n} {k}")).collect();
    writeln!(
        out,
        "\nCollision:    {}; {} triangles",
        list.join(", "),
        thousands(collision.triangle_count())
    )?;
    if lo[0].is_finite() {
        writeln!(
            out,
            "              spans x {:.0}..{:.0}, y {:.0}..{:.0}, z {:.0}..{:.0}",
            lo[0], hi[0], lo[1], hi[1], lo[2], hi[2]
        )?;
    }
    let (vlo, vhi) = scene.meshes.iter().flat_map(|m| m.model_positions()).fold(
        ([f32::INFINITY; 3], [f32::NEG_INFINITY; 3]),
        |(lo, hi), p| {
            (
                [lo[0].min(p[0]), lo[1].min(p[1]), lo[2].min(p[2])],
                [hi[0].max(p[0]), hi[1].max(p[1]), hi[2].max(p[2])],
            )
        },
    );
    if vlo[0].is_finite() {
        writeln!(
            out,
            "              (visible meshes span x {:.0}..{:.0}, y {:.0}..{:.0}, z {:.0}..{:.0})",
            vlo[0], vhi[0], vlo[1], vhi[1], vlo[2], vhi[2]
        )?;
    }
    if !collision.unhandled.is_empty() {
        let list: Vec<String> = collision
            .unhandled
            .iter()
            .map(|(t, n)| format!("{t} ({n})"))
            .collect();
        writeln!(out, "              not read: {}", list.join(", "))?;
    }
    if collision.no_collision > 0 {
        writeln!(
            out,
            "              {} bodies flagged \"no collision\" left out",
            collision.no_collision
        )?;
    }
    if let Some(shell) = collision
        .parts
        .iter()
        .map(|p| p.shell)
        .filter(|&s| s > 0.0)
        .reduce(f32::max)
    {
        writeln!(
            out,
            "              surfaces stand out {shell:.2} units from their shapes (Havok's convex radius)"
        )?;
    }
    Ok(())
}

/// Names of the Gamebryo blend factors, by their number in NiAlphaProperty.
const BLEND_FACTORS: [&str; 11] = [
    "One",
    "Zero",
    "SrcColor",
    "InvSrcColor",
    "DstColor",
    "InvDstColor",
    "SrcAlpha",
    "InvSrcAlpha",
    "DstAlpha",
    "InvDstAlpha",
    "SrcAlphaSaturate",
];

const ALPHA_TESTS: [&str; 8] = [
    "always",
    "less",
    "equal",
    "less-or-equal",
    "greater",
    "not-equal",
    "greater-or-equal",
    "never",
];

/// Shader flag bits worth naming when working out how a mesh is drawn.
const SHADER_FLAGS: [(u32, &str); 10] = [
    (0x0000_0001, "specular"),
    (0x0000_0002, "skinned"),
    (0x0000_0008, "vertex alpha"),
    (ENVIRONMENT_MAP, "environment map"),
    (0x0000_0100, "alpha texture"),
    (0x0002_0000, "eye environment map"),
    (0x0020_0000, "window environment map"),
    (0x0400_0000, "decal"),
    (0x0800_0000, "dynamic decal"),
    (0x2000_0000, "external emittance"),
];

const ENVIRONMENT_MAP: u32 = 0x0000_0080;

const SHADER_FLAGS2: [(u32, &str); 4] = [
    (0x0000_0001, "z-write"),
    (0x0000_0020, "vertex colors"),
    (0x0000_0400, "vertex lighting"),
    (0x0001_0000, "wireframe"),
];

/// How each mesh is drawn: shader, blending, material and placement.
fn materials(
    out: &mut impl Write,
    scene: &nif::Scene,
    assets: Option<&assets::Assets>,
) -> Result<(), CliError> {
    writeln!(out, "\nHow each mesh is drawn:")?;
    for mesh in &scene.meshes {
        let (lo, hi) = mesh.model_positions().fold(
            ([f32::INFINITY; 3], [f32::NEG_INFINITY; 3]),
            |(lo, hi), p| {
                (
                    [lo[0].min(p[0]), lo[1].min(p[1]), lo[2].min(p[2])],
                    [hi[0].max(p[0]), hi[1].max(p[1]), hi[2].max(p[2])],
                )
            },
        );
        writeln!(
            out,
            "  {}  (spans x {:.0}..{:.0}, y {:.0}..{:.0}, z {:.0}..{:.0})",
            mesh.name, lo[0], hi[0], lo[1], hi[1], lo[2], hi[2]
        )?;
        match &mesh.shader {
            Some(s) => {
                let named = |flags: u32, table: &[(u32, &str)]| -> String {
                    let names: Vec<&str> = table
                        .iter()
                        .filter(|(bit, _)| flags & bit != 0)
                        .map(|(_, name)| *name)
                        .collect();
                    if names.is_empty() {
                        String::new()
                    } else {
                        format!(" [{}]", names.join(", "))
                    }
                };
                writeln!(
                    out,
                    "      shader: {}{}, flags {:#010x}{} / {:#010x}{}",
                    s.type_name,
                    if s.lit { "" } else { " (no lighting)" },
                    s.shader_flags,
                    named(s.shader_flags, &SHADER_FLAGS),
                    s.shader_flags2,
                    named(s.shader_flags2, &SHADER_FLAGS2)
                )?;
                if let Some(f) = s.falloff {
                    writeln!(
                        out,
                        "      falloff: opacity {:.2} at {:.2} to {:.2} at {:.2}",
                        f.start_opacity, f.start_angle, f.stop_opacity, f.stop_angle
                    )?;
                }
                if s.shader_flags & ENVIRONMENT_MAP != 0 {
                    writeln!(out, "      reflection strength: {:.2}", s.env_map_scale)?;
                }
            }
            None => writeln!(out, "      shader: none (older texturing style)")?,
        }
        for (slot, texture) in mesh.textures.iter().enumerate() {
            if !texture.is_empty() {
                let label = [
                    "diffuse",
                    "normal map",
                    "glow",
                    "height",
                    "environment",
                    "environment mask",
                ]
                .get(slot)
                .copied()
                .unwrap_or("extra");
                let found = match assets {
                    None => String::new(),
                    Some(assets) => {
                        let path = assets::texture_path(texture);
                        match assets.locate(&path) {
                            Some(source) => format!("  (found in {})", source.describe()),
                            None => "  (MISSING)".to_string(),
                        }
                    }
                };
                writeln!(out, "      {label}: {texture}{found}")?;
            }
        }
        match mesh.alpha {
            Some(a) => {
                let mut parts = Vec::new();
                if a.blending() {
                    let factor =
                        |bits: u16| BLEND_FACTORS.get(usize::from(bits)).copied().unwrap_or("?");
                    parts.push(format!(
                        "blend {} x source + {} x destination",
                        factor((a.flags >> 1) & 0xF),
                        factor((a.flags >> 5) & 0xF)
                    ));
                }
                if a.testing() {
                    parts.push(format!(
                        "test {} {}",
                        ALPHA_TESTS[usize::from((a.flags >> 10) & 7)],
                        a.threshold
                    ));
                }
                if parts.is_empty() {
                    parts.push("present but neither blending nor testing".into());
                }
                writeln!(
                    out,
                    "      alpha: {} (flags {:#06x})",
                    parts.join(", "),
                    a.flags
                )?;
            }
            None => writeln!(out, "      alpha: opaque")?,
        }
        if let Some(m) = &mesh.material {
            writeln!(
                out,
                "      material: opacity {:.2}, glow {:.2},{:.2},{:.2} x {:.2}, specular {:.2},{:.2},{:.2}, glossiness {:.1}",
                m.alpha,
                m.emissive[0],
                m.emissive[1],
                m.emissive[2],
                m.emissive_mult,
                m.specular[0],
                m.specular[1],
                m.specular[2],
                m.glossiness
            )?;
        }
        let mut extra = Vec::new();
        let colors = vertex_color_summary(&mesh.colors);
        if let Some(colors) = &colors {
            extra.push(colors.as_str());
        }
        if mesh.double_sided {
            extra.push("double-sided");
        }
        if !extra.is_empty() {
            writeln!(out, "      {}", extra.join(", "))?;
        }
        if let Some(line) = tangent_summary(mesh) {
            writeln!(out, "      {line}")?;
        }
        if let Some(skin) = &mesh.skin {
            writeln!(out, "      {}", skin_summary(skin))?;
        }
    }
    Ok(())
}

/// A skinned mesh's bones and its partitions' body parts (gore caps
/// marked), with their triangle counts.
fn skin_summary(skin: &nif::Skin) -> String {
    let parts: Vec<String> = skin
        .partitions
        .iter()
        .map(|p| {
            format!(
                "{}{} ({} triangles)",
                p.body_part,
                if p.is_cap() { " cap" } else { "" },
                p.triangles.len()
            )
        })
        .collect();
    format!(
        "skin: {} bones ({}); partitions: {}",
        skin.bones.len(),
        skin.bones.join(", "),
        if parts.is_empty() {
            "none".to_string()
        } else {
            parts.join(", ")
        }
    )
}

/// How the stored tangent arrays line up with the directions the texture
/// runs across each triangle (U: increasing u, V: increasing v), as average
/// cosines: 1 means the same direction, -1 the opposite.
fn tangent_summary(mesh: &nif::Mesh) -> Option<String> {
    let n = mesh.positions.len();
    if mesh.tangents.len() != n || mesh.bitangents.len() != n || mesh.uvs.len() != n {
        return None;
    }
    let sub = |a: [f32; 3], b: [f32; 3]| [a[0] - b[0], a[1] - b[1], a[2] - b[2]];
    let cos = |a: [f32; 3], b: [f32; 3]| {
        let d = a[0] * b[0] + a[1] * b[1] + a[2] * b[2];
        let l =
            (a.iter().map(|x| x * x).sum::<f32>() * b.iter().map(|x| x * x).sum::<f32>()).sqrt();
        (l > 1e-12).then(|| d / l)
    };
    // first·U, first·V, second·U, second·V
    let mut sums = [0.0f64; 4];
    let mut count = 0usize;
    for t in &mesh.triangles {
        let [a, b, c] = t.map(usize::from);
        let (e1, e2) = (
            sub(mesh.positions[b], mesh.positions[a]),
            sub(mesh.positions[c], mesh.positions[a]),
        );
        let (du1, dv1) = (
            mesh.uvs[b][0] - mesh.uvs[a][0],
            mesh.uvs[b][1] - mesh.uvs[a][1],
        );
        let (du2, dv2) = (
            mesh.uvs[c][0] - mesh.uvs[a][0],
            mesh.uvs[c][1] - mesh.uvs[a][1],
        );
        let det = du1 * dv2 - du2 * dv1;
        if det.abs() < 1e-12 {
            continue;
        }
        let along_u = [0, 1, 2].map(|k| (e1[k] * dv2 - e2[k] * dv1) / det);
        let along_v = [0, 1, 2].map(|k| (e2[k] * du1 - e1[k] * du2) / det);
        for v in [a, b, c] {
            let values = [
                cos(mesh.tangents[v], along_u),
                cos(mesh.tangents[v], along_v),
                cos(mesh.bitangents[v], along_u),
                cos(mesh.bitangents[v], along_v),
            ];
            if let [Some(w), Some(x), Some(y), Some(z)] = values {
                for (s, value) in sums.iter_mut().zip([w, x, y, z]) {
                    *s += f64::from(value);
                }
                count += 1;
            }
        }
    }
    if count == 0 {
        return None;
    }
    let avg = sums.map(|s| s / count as f64);
    Some(format!(
        "tangent space: first array vs texture U {:+.2}, V {:+.2}; second array vs U {:+.2}, V {:+.2}",
        avg[0], avg[1], avg[2], avg[3]
    ))
}

/// "has vertex colors", with their average and range (0-255), since they
/// often carry baked shading.
fn vertex_color_summary(colors: &[[f32; 4]]) -> Option<String> {
    if colors.is_empty() {
        return None;
    }
    let n = colors.len() as f32;
    let byte = |v: f32| (v * 255.0).round() as i32;
    let mut sum = [0.0f32; 4];
    let mut low = [f32::MAX; 4];
    let mut high = [f32::MIN; 4];
    for c in colors {
        for k in 0..4 {
            sum[k] += c[k];
            low[k] = low[k].min(c[k]);
            high[k] = high[k].max(c[k]);
        }
    }
    let channels: Vec<String> = (0..4)
        .map(|k| format!("{} ({}-{})", byte(sum[k] / n), byte(low[k]), byte(high[k])))
        .collect();
    Some(format!(
        "has vertex colors: average R {} G {} B {} A {}",
        channels[0], channels[1], channels[2], channels[3]
    ))
}

fn block_type_summary(out: &mut impl Write, counts: &[(String, usize)]) -> Result<(), CliError> {
    writeln!(out, "\nBlock types:")?;
    for (name, count) in counts.iter().take(15) {
        writeln!(out, "  {name:<32} {count:>5}")?;
    }
    if counts.len() > 15 {
        writeln!(out, "  ... and {} more (see `blocks`)", counts.len() - 15)?;
    }
    Ok(())
}

fn blocks(out: &mut impl Write, nif: &Nif, options: &Options) -> Result<(), CliError> {
    writeln!(out, "{:>5}  {:<32} {:>8}  Name", "#", "Type", "Size")?;
    let mut shown = 0;
    for i in 0..nif.blocks().len() {
        if options.limit_reached(shown) {
            break;
        }
        let type_name = nif.block_type(i);
        let name = nif.block_name(i);
        if !options.matches(&[Some(type_name), name.as_deref()]) {
            continue;
        }
        let line = format!(
            "{i:>5}  {type_name:<32} {:>8}  {}",
            thousands(nif.blocks()[i].size),
            name.unwrap_or_default()
        );
        writeln!(out, "{}", line.trim_end())?;
        shown += 1;
    }
    writeln!(out, "\n{} shown of {} blocks", shown, nif.blocks().len())?;
    Ok(())
}

pub fn export_obj(
    out: &mut impl Write,
    nif: &Nif,
    label: &str,
    output: Option<String>,
    default_name: &str,
    force: bool,
) -> Result<(), CliError> {
    let scene = nif.scene()?;
    let target = match output {
        Some(o) if Path::new(&o).is_dir() => Path::new(&o).join(default_name),
        Some(o) => PathBuf::from(o),
        None => PathBuf::from(default_name),
    };
    if target.exists() && !force {
        return Err(CliError::NotFound(format!(
            "{} already exists; add --force to overwrite it",
            target.display()
        )));
    }
    let mut text = Vec::new();
    nif::obj::write_obj(&mut text, &scene.meshes, label)?;
    std::fs::write(&target, &text).map_err(|e| CliError::Open {
        path: target.display().to_string(),
        message: e.to_string(),
    })?;
    writeln!(
        out,
        "Wrote {} ({} meshes, {} triangles)",
        target.display(),
        scene.meshes.len(),
        thousands(scene.triangle_count())
    )?;
    Ok(())
}

// ---------------------------------------------------------------------------
// check-nifs: parse every mesh in an archive
// ---------------------------------------------------------------------------

const EXAMPLES_PER_GROUP: usize = 3;

#[derive(Default)]
struct CheckTotals {
    parsed: usize,
    meshes: usize,
    skinned: usize,
    untextured: usize,
    triangles: usize,
    invalid_references: usize,
    dropped_triangles: usize,
    unhandled: BTreeMap<String, usize>,
    /// Meshes without a texture, keyed by the property types they carry.
    untextured_by_properties: BTreeMap<String, usize>,
    /// A few "path (shape name)" examples per untextured group.
    untextured_examples: BTreeMap<String, Vec<String>>,
    unreadable_properties: BTreeMap<String, usize>,
    shaders_checked: usize,
    /// Shader blocks whose size didn't match the expected layout, by type.
    shader_layout_mismatches: BTreeMap<String, usize>,
    /// (archive path, error message)
    failures: Vec<(String, String)>,
    /// Failure counts by block type (or error kind when not in a block).
    failure_kinds: BTreeMap<String, usize>,
}

impl CheckTotals {
    fn merge(&mut self, other: CheckTotals) {
        self.parsed += other.parsed;
        self.meshes += other.meshes;
        self.skinned += other.skinned;
        self.untextured += other.untextured;
        self.triangles += other.triangles;
        self.invalid_references += other.invalid_references;
        self.dropped_triangles += other.dropped_triangles;
        for (k, v) in other.unhandled {
            *self.unhandled.entry(k).or_default() += v;
        }
        for (k, v) in other.untextured_by_properties {
            *self.untextured_by_properties.entry(k).or_default() += v;
        }
        for (k, v) in other.unreadable_properties {
            *self.unreadable_properties.entry(k).or_default() += v;
        }
        for (k, mut v) in other.untextured_examples {
            let list = self.untextured_examples.entry(k).or_default();
            list.append(&mut v);
            list.sort();
            list.truncate(EXAMPLES_PER_GROUP);
        }
        self.shaders_checked += other.shaders_checked;
        for (k, v) in other.shader_layout_mismatches {
            *self.shader_layout_mismatches.entry(k).or_default() += v;
        }
        self.failures.extend(other.failures);
        for (k, v) in other.failure_kinds {
            *self.failure_kinds.entry(k).or_default() += v;
        }
    }

    fn record_failure(&mut self, path: &str, kind: String, message: String) {
        self.failures.push((path.to_string(), message));
        *self.failure_kinds.entry(kind).or_default() += 1;
    }
}

fn check_one(archive: &Archive, index: usize, totals: &mut CheckTotals) {
    let entry = &archive.files()[index];
    let bytes = match archive.read(entry) {
        Ok(b) => b,
        Err(e) => {
            totals.record_failure(&entry.path, "archive read".into(), e.to_string());
            return;
        }
    };
    let outcome = panic::catch_unwind(AssertUnwindSafe(|| {
        Nif::parse(bytes).and_then(|n| n.scene())
    }));
    match outcome {
        Ok(Ok(scene)) => {
            totals.parsed += 1;
            totals.meshes += scene.meshes.len();
            totals.skinned += scene.meshes.iter().filter(|m| m.skinned).count();
            for mesh in scene
                .meshes
                .iter()
                .filter(|m| m.diffuse_texture().is_none())
            {
                totals.untextured += 1;
                let mut kinds = mesh.property_types.clone();
                kinds.sort();
                kinds.dedup();
                let key = if kinds.is_empty() {
                    "(no properties)".to_string()
                } else {
                    kinds.join(" + ")
                };
                let examples = totals.untextured_examples.entry(key.clone()).or_default();
                if examples.len() < EXAMPLES_PER_GROUP {
                    examples.push(format!("{} ({})", entry.path, mesh.name));
                }
                *totals.untextured_by_properties.entry(key).or_default() += 1;
            }
            for shader in scene.meshes.iter().filter_map(|m| m.shader.as_ref()) {
                totals.shaders_checked += 1;
                if !shader.layout_ok {
                    *totals
                        .shader_layout_mismatches
                        .entry(shader.type_name.clone())
                        .or_default() += 1;
                }
            }
            for (k, v) in &scene.unreadable_properties {
                *totals.unreadable_properties.entry(k.clone()).or_default() += v;
            }
            totals.triangles += scene.triangle_count();
            totals.invalid_references += scene.invalid_references;
            totals.dropped_triangles += scene.dropped_triangles;
            for (k, v) in scene.unhandled {
                *totals.unhandled.entry(k).or_default() += v;
            }
        }
        Ok(Err(e)) => {
            let kind = match &e {
                nif::Error::InBlock { type_name, .. } => type_name.clone(),
                nif::Error::Unsupported(_) => "unsupported".into(),
                nif::Error::NotANif { .. } => "not a NIF".into(),
                _ => "file structure".into(),
            };
            totals.record_failure(&entry.path, kind, e.to_string());
        }
        Err(_) => totals.record_failure(
            &entry.path,
            "crash (reader bug)".into(),
            "the reader panicked on this file".into(),
        ),
    }
}

pub fn check_archive(
    out: &mut impl Write,
    archive: &Archive,
    options: &Options,
) -> Result<(), CliError> {
    let mut targets: Vec<usize> = archive
        .files()
        .iter()
        .enumerate()
        .filter(|(_, f)| f.name.ends_with(".nif") && options.matches(&[Some(&f.path)]))
        .map(|(i, _)| i)
        .collect();
    if let Some(limit) = options.limit {
        targets.truncate(limit);
    }
    if targets.is_empty() {
        writeln!(out, "No .nif files to check in this archive.")?;
        return Ok(());
    }

    let started = Instant::now();
    let next = AtomicUsize::new(0);
    let done = AtomicUsize::new(0);
    let totals = Mutex::new(CheckTotals::default());
    let threads = std::thread::available_parallelism()
        .map_or(4, |n| n.get())
        .min(16);

    // Keep panics from printing their own messages; they're reported below.
    let previous_hook = panic::take_hook();
    panic::set_hook(Box::new(|_| {}));
    std::thread::scope(|scope| {
        for _ in 0..threads {
            scope.spawn(|| {
                let mut local = CheckTotals::default();
                loop {
                    let i = next.fetch_add(1, Ordering::Relaxed);
                    let Some(&index) = targets.get(i) else { break };
                    check_one(archive, index, &mut local);
                    let finished = done.fetch_add(1, Ordering::Relaxed) + 1;
                    if finished % 2000 == 0 {
                        eprintln!(
                            "  checked {} of {}...",
                            thousands(finished),
                            thousands(targets.len())
                        );
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

    let mut totals = totals.into_inner().unwrap_or_else(|p| p.into_inner());
    let elapsed = started.elapsed();
    writeln!(
        out,
        "Checked {} .nif files in {:.1} s",
        thousands(targets.len()),
        elapsed.as_secs_f64()
    )?;
    writeln!(out, "  parsed completely:  {:>9}", thousands(totals.parsed))?;
    writeln!(
        out,
        "  failed:             {:>9}",
        thousands(totals.failures.len())
    )?;
    writeln!(out, "  visible meshes:     {:>9}", thousands(totals.meshes))?;
    writeln!(
        out,
        "    skinned:          {:>9}",
        thousands(totals.skinned)
    )?;
    writeln!(
        out,
        "    no texture found: {:>9}",
        thousands(totals.untextured)
    )?;
    writeln!(
        out,
        "  triangles:          {:>9}",
        thousands(totals.triangles)
    )?;
    if totals.invalid_references + totals.dropped_triangles > 0 {
        writeln!(
            out,
            "  broken references:  {:>9}\n  bad triangles:      {:>9}",
            thousands(totals.invalid_references),
            thousands(totals.dropped_triangles)
        )?;
    }
    if !totals.untextured_by_properties.is_empty() {
        let mut groups: Vec<_> = totals.untextured_by_properties.into_iter().collect();
        groups.sort_by_key(|entry| std::cmp::Reverse(entry.1));
        writeln!(
            out,
            "\nMeshes with no texture found, by the properties they have:"
        )?;
        for (kinds, n) in groups.iter().take(12) {
            writeln!(out, "  {:>7}  {kinds}", thousands(*n))?;
            for example in totals.untextured_examples.get(kinds).into_iter().flatten() {
                writeln!(out, "             e.g. {example}")?;
            }
        }
        if groups.len() > 12 {
            writeln!(out, "  ... and {} rarer combinations", groups.len() - 12)?;
        }
    }
    if totals.shaders_checked > 0 {
        if totals.shader_layout_mismatches.is_empty() {
            writeln!(
                out,
                "\nShader blocks: all {} read at the expected size",
                thousands(totals.shaders_checked)
            )?;
        } else {
            writeln!(
                out,
                "\nShader blocks with an unexpected size (possible misread) out of {}:",
                thousands(totals.shaders_checked)
            )?;
            for (t, n) in &totals.shader_layout_mismatches {
                writeln!(out, "  {t:<36} {:>7}", thousands(*n))?;
            }
        }
    }
    if !totals.unreadable_properties.is_empty() {
        writeln!(
            out,
            "\nProperty blocks that couldn't be read (meshes kept without them):"
        )?;
        for (t, n) in &totals.unreadable_properties {
            writeln!(out, "  {t:<36} {:>7}", thousands(*n))?;
        }
    }
    if !totals.unhandled.is_empty() {
        let mut unhandled: Vec<_> = totals.unhandled.into_iter().collect();
        unhandled.sort_by_key(|entry| std::cmp::Reverse(entry.1));
        writeln!(out, "\nScene objects not drawn yet:")?;
        for (t, n) in unhandled.iter().take(12) {
            writeln!(out, "  {t:<36} {:>7}", thousands(*n))?;
        }
    }
    if !totals.failures.is_empty() {
        let mut kinds: Vec<_> = totals.failure_kinds.into_iter().collect();
        kinds.sort_by_key(|entry| std::cmp::Reverse(entry.1));
        writeln!(out, "\nFailures by cause (block type where known):")?;
        for (k, n) in &kinds {
            writeln!(out, "  {k:<36} {:>7}", thousands(*n))?;
        }
        totals.failures.sort();
        writeln!(out, "\nFirst failures:")?;
        for (path, message) in totals.failures.iter().take(15) {
            writeln!(out, "  {path}\n    {message}")?;
        }
    }
    Ok(())
}

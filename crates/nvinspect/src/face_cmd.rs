//! `face <ID>`: a person's head parts and the morphs that move them (which
//! channel each answers to, or that the game never uses it). `lip <PATH>`:
//! one line's lip sync file, its timing and its strongest weights per
//! frame.

use std::io::Write;

use assets::Assets;
use esm::{sig, LoadOrder};
use world::face::{self, FaceMorphs, FaceSettings, Reshape};
use world::lip::{lip_path, Lip};

use crate::records::{describe_id, find_record};
use crate::{CliError, Options};

fn length(v: [f32; 3]) -> f32 {
    (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt()
}

/// `face`: every FaceGen part of a person, its `.tri` and its morphs.
pub fn face(
    out: &mut impl Write,
    order: &LoadOrder,
    assets: &Assets,
    target: &str,
) -> Result<(), CliError> {
    let found = find_record(order, target)?;
    let base = if found.entry.header.kind == sig::ACHR || found.entry.header.kind == sig::ACRE {
        world::scripting::base_of(order, found.form_id)
            .ok_or_else(|| CliError::NotFound(format!("{target}'s base isn't loaded")))?
    } else {
        found.form_id
    };
    let look = world::actor_look(order, base).ok_or_else(|| {
        CliError::NotFound(format!(
            "{} isn't a person this can assemble",
            describe_id(order, base)
        ))
    })?;
    writeln!(out, "{}", describe_id(order, base))?;
    let read = |path: &str| assets.read(path).ok().flatten();
    for part in look.parts.iter().filter(|p| p.facegen) {
        let model = assets::mesh_path(&part.model);
        writeln!(out, "\n{model}")?;
        let Some(tri_path) = nif::tri::tri_path(&model) else {
            continue;
        };
        let Some(bytes) = read(&tri_path) else {
            writeln!(out, "  no {tri_path}: it never moves")?;
            continue;
        };
        let tri = match nif::Tri::parse(&bytes) {
            Ok(tri) => tri,
            Err(e) => {
                writeln!(out, "  {tri_path} can't be read: {e}")?;
                continue;
            }
        };
        let meshes: Vec<(String, Vec<[f32; 3]>)> = read(&model)
            .and_then(|b| nif::Nif::parse(b).ok())
            .and_then(|n| n.placed_scene().ok())
            .map(|s| {
                s.meshes
                    .into_iter()
                    .map(|m| (m.name, m.positions))
                    .collect()
            })
            .unwrap_or_default();
        let egm = model
            .rsplit_once('.')
            .and_then(|(stem, _)| read(&format!("{stem}.egm")))
            .and_then(|b| nif::Egm::parse(&b).ok());
        writeln!(
            out,
            "  {tri_path}: {} vertices, {} differential and {} statistical morphs ({} targets)",
            tri.base.len(),
            tri.differential.len(),
            tri.statistical.len(),
            tri.target_count()
        )?;
        for (name, positions) in &meshes {
            let moved = tri
                .base
                .iter()
                .zip(positions)
                .filter(|(a, b)| length([a[0] - b[0], a[1] - b[1], a[2] - b[2]]) > 0.001)
                .count();
            writeln!(
                out,
                "  mesh {name:?}: {} vertices ({moved} not where the .tri has them)",
                positions.len()
            )?;
        }
        if let Some(egm) = &egm {
            let fits =
                tri.target_count() > 0 && egm.vertices == tri.base.len() + tri.target_count();
            writeln!(
                out,
                "  .egm: {} vertices{}",
                egm.vertices,
                if fits {
                    " = the .tri's vertices + its targets (the face reshapes the targets too)"
                } else {
                    ""
                }
            )?;
        }
        let used = |name: &str| match face::channel(name) {
            Some(c) => format!("{:?} {}", c.group, c.name()),
            None => "never used (no channel has this name)".into(),
        };
        for m in &tri.differential {
            let most = m.offsets.iter().map(|&d| length(d)).fold(0.0, f32::max);
            writeln!(
                out,
                "    {:<14} differential, moves up to {most:.3}: {}",
                m.name,
                used(&m.name)
            )?;
        }
        for m in &tri.statistical {
            let most = m
                .vertices
                .iter()
                .zip(&m.targets)
                .filter_map(|(&i, t)| {
                    let b = tri.base.get(i as usize)?;
                    Some(length([t[0] - b[0], t[1] - b[1], t[2] - b[2]]))
                })
                .fold(0.0, f32::max);
            writeln!(
                out,
                "    {:<14} statistical, {} vertices, up to {most:.3}: {}",
                m.name,
                m.vertices.len(),
                used(&m.name)
            )?;
        }
        // What the viewer will move, for the first mesh, with this face.
        if let Some((_, positions)) = meshes.first() {
            let mut shaped = positions.clone();
            let reshape = match (&egm, &look.face) {
                (Some(egm), Some(f)) => {
                    egm.apply(&mut shaped, &f.symmetric, &f.asymmetric);
                    Some(Reshape {
                        egm,
                        symmetric: &f.symmetric,
                        asymmetric: &f.asymmetric,
                    })
                }
                _ => None,
            };
            let morphs = FaceMorphs::build(&tri, &shaped, reshape);
            let names: Vec<&str> = morphs.morphs.iter().map(|m| m.channel.name()).collect();
            writeln!(out, "  moves by: {}", names.join(", "))?;
            // How far this face moves the targets (where a blink's lids
            // land) from where the file has them.
            if let (Some(r), true) = (reshape, tri.target_count() > 0) {
                let mut targets: Vec<[f32; 3]> = tri
                    .statistical
                    .iter()
                    .flat_map(|m| m.targets.iter().copied())
                    .collect();
                let stored = targets.clone();
                r.egm
                    .apply_rows(tri.base.len(), &mut targets, r.symmetric, r.asymmetric);
                let most = targets
                    .iter()
                    .zip(&stored)
                    .map(|(a, b)| length([a[0] - b[0], a[1] - b[1], a[2] - b[2]]))
                    .fold(0.0, f32::max);
                // If those rows are the targets, each moves much as the
                // vertex it targets does (the two sit close together).
                let moved = |row: usize| {
                    let mut p = [[0.0f32; 3]];
                    r.egm.apply_rows(row, &mut p, r.symmetric, r.asymmetric);
                    p[0]
                };
                let mut apart: f32 = 0.0;
                for m in &tri.statistical {
                    for (k, &i) in m.vertices.iter().enumerate() {
                        let t = moved(tri.base.len() + m.first_target + k);
                        let v = moved(i as usize);
                        apart = apart.max(length([t[0] - v[0], t[1] - v[1], t[2] - v[2]]));
                    }
                }
                writeln!(
                    out,
                    "  this face moves the targets by up to {most:.3} (its .egm rows past the mesh), each within {apart:.3} of how it moves the vertex it targets"
                )?;
            }
        }
    }
    Ok(())
}

/// `lip`: a lip sync file (or a voice file's), when its frames are
/// reached and when the voice starts, and each frame's strongest weights.
pub fn lip(
    out: &mut impl Write,
    order: &LoadOrder,
    assets: &Assets,
    ini: &assets::IniSettings,
    path: &str,
    options: &Options,
) -> Result<(), CliError> {
    let path = if path.to_ascii_lowercase().ends_with(".lip") {
        path.to_string()
    } else {
        lip_path(path).unwrap_or_else(|| path.to_string())
    };
    let bytes = assets
        .read(&path)
        .map_err(|e| CliError::InFile {
            path: path.clone(),
            message: e.to_string(),
        })?
        .ok_or_else(|| CliError::NotFound(format!("{path} isn't in the game's files")))?;
    let lip = Lip::parse(&bytes).map_err(|message| CliError::InFile {
        path: path.clone(),
        message,
    })?;
    let settings = FaceSettings::read(order, |section, key| ini.float(section, key));
    let mut animation = face::FaceAnimation::new(0);
    let timing = animation.speak(&lip, &settings);
    let ease = lip.lead_in().min(0.2);
    writeln!(out, "{path}")?;
    writeln!(
        out,
        "  {} frames ({:.3} s), lead-in {} frames ({:.3} s)",
        lip.frames.len(),
        lip.length(),
        -lip.offset,
        lip.lead_in()
    )?;
    writeln!(
        out,
        "  face eases to rest over {ease:.3} s; frame k is reached at {ease:.3} + (k + 1)/30 s"
    )?;
    writeln!(
        out,
        "  voice starts at {:.3} s (lead-in + fSpeechDelay {}); the line lasts {:.3} s",
        timing.voice_delay, settings.speech_delay, timing.length
    )?;
    let named = |values: &[f32], names: &[&str]| -> Vec<String> {
        let mut v: Vec<(f32, &str)> = values
            .iter()
            .zip(names)
            .filter(|(w, _)| w.abs() >= 0.05)
            .map(|(&w, &n)| (w, n))
            .collect();
        v.sort_by(|a, b| b.0.abs().total_cmp(&a.0.abs()));
        v.into_iter()
            .take(4)
            .map(|(w, n)| format!("{n} {w:.3}"))
            .collect()
    };
    for (k, frame) in lip.frames.iter().enumerate() {
        if options.limit_reached(k) {
            break;
        }
        let mut parts = named(&frame.phonemes, &face::PHONEMES);
        parts.extend(named(&frame.modifiers, &face::MODIFIERS));
        writeln!(
            out,
            "  {k:>4}  at {:.3} s  (voice {:+.3} s)  {}",
            ease + (k + 1) as f32 / 30.0,
            (k as f32 + lip.offset as f32) / 30.0,
            parts.join(", ")
        )?;
    }
    Ok(())
}

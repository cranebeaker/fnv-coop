//! `meshes <CELL>`: every piece of every model placed in a cell, with its
//! vertex and triangle counts and the settings that decide how the game
//! draws it (shader and its flags, blending and alpha test, depth test and
//! write, sides, sorting). Recordings of the game's Direct3D calls are
//! matched to meshes by their vertex counts, so this is the list to look
//! them up in.

use std::collections::BTreeMap;
use std::io::Write;

use assets::Assets;
use esm::LoadOrder;
use world::load_cell;

use crate::render_cmd::{single_cell, world_error};
use crate::CliError;

pub fn meshes(
    out: &mut impl Write,
    order: &LoadOrder,
    assets: &Assets,
    query: &str,
) -> Result<(), CliError> {
    let cell_id = single_cell(order, query)?;
    let loaded = load_cell(order, cell_id).map_err(world_error)?;
    // Model path → where it's placed (and as what).
    let mut models: BTreeMap<String, Vec<String>> = BTreeMap::new();
    for object in &loaded.objects {
        let paths: Vec<&str> = match object.model.as_deref() {
            Some(m) if assets.contains(&assets::mesh_path(m)) => vec![m],
            _ => object
                .parts
                .iter()
                .filter_map(|p| p.model.as_deref())
                .collect(),
        };
        let [x, y, z] = object.position;
        let [rx, ry, rz] = object.rotation.map(f32::to_degrees);
        for path in paths {
            models
                .entry(assets::mesh_path(path))
                .or_default()
                .push(format!(
                    "{} {} at {x:.1},{y:.1},{z:.1} turned {rx:.1},{ry:.1},{rz:.1} scale {:.2}",
                    object.base_type, object.form_id, object.scale
                ));
        }
    }
    writeln!(
        out,
        "{}: {} models placed {} times",
        loaded.info.label(),
        models.len(),
        models.values().map(Vec::len).sum::<usize>()
    )?;
    writeln!(
        out,
        "  per piece: vertices, triangles, shader, its flags (first / second set), \
         then how the game draws it"
    )?;
    for (path, places) in &models {
        writeln!(out, "\n{path} (x{})", places.len())?;
        let shown: Vec<&str> = places.iter().take(4).map(String::as_str).collect();
        let more = places.len().saturating_sub(4);
        writeln!(
            out,
            "  placed: {}{}",
            shown.join("; "),
            if more > 0 {
                format!("; and {more} more")
            } else {
                String::new()
            }
        )?;
        let nif = match assets.read(path) {
            Ok(Some(bytes)) => match nif::Nif::parse(bytes) {
                Ok(nif) => nif,
                Err(e) => {
                    writeln!(out, "  unreadable: {e}")?;
                    continue;
                }
            },
            _ => {
                writeln!(out, "  not found")?;
                continue;
            }
        };
        let scene = match nif.placed_scene() {
            Ok(scene) => scene,
            Err(e) => {
                writeln!(out, "  unreadable: {e}")?;
                continue;
            }
        };
        for mesh in &scene.meshes {
            let shader = mesh.shader.as_ref().map_or("no shader".to_string(), |s| {
                format!(
                    "{} {:#010x}/{:#010x}",
                    s.type_name
                        .trim_start_matches("BSShader")
                        .trim_end_matches("Property"),
                    s.shader_flags,
                    s.shader_flags2
                )
            });
            let state = preview::cell::DrawState::of(mesh);
            let (c, r) = mesh.model_bound();
            writeln!(
                out,
                "  {:<24} {:>6} v {:>6} t  {shader}  {}  (bound {:.1},{:.1},{:.1} r {:.1})",
                crate::fmt::truncate(&mesh.name, 24),
                mesh.positions.len(),
                mesh.triangles.len(),
                state.describe(),
                c[0],
                c[1],
                c[2],
                r
            )?;
        }
        // The model's animations, and which a placed copy plays (as
        // something that opens and closes, or not).
        let sequences = nif.sequences().unwrap_or_default();
        if !sequences.is_empty() {
            let list: Vec<String> = sequences
                .iter()
                .map(|s| {
                    let moves: Vec<&str> = s.tracks.iter().map(|t| t.node.as_str()).collect();
                    format!(
                        "{} ({:.2} s{}, moves {})",
                        s.name,
                        s.stop - s.start,
                        if s.looping { ", loops" } else { "" },
                        if moves.is_empty() {
                            "no nodes".to_string()
                        } else {
                            moves.join(", ")
                        }
                    )
                })
                .collect();
            writeln!(out, "  sequences: {}", list.join("; "))?;
            let names = |openable| {
                let list: Vec<String> = preview::cell::placed_sequences(&sequences, openable)
                    .iter()
                    .map(|p| {
                        if p.runs {
                            p.sequence.name.clone()
                        } else {
                            format!("{} (first frame only)", p.sequence.name)
                        }
                    })
                    .collect();
                if list.is_empty() {
                    "nothing".to_string()
                } else {
                    list.join(", ")
                }
            };
            writeln!(
                out,
                "  plays when placed: {}; as something that opens: {}",
                names(false),
                names(true)
            )?;
        }
        if !scene.unhandled.is_empty() {
            let list: Vec<String> = scene
                .unhandled
                .iter()
                .map(|(t, n)| format!("{t} ({n})"))
                .collect();
            writeln!(out, "  not drawn: {}", list.join(", "))?;
        }
    }
    Ok(())
}

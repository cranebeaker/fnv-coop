//! `grass <WORLD> <X> <Y>`: the grass the game grows on one outdoor grid
//! square (`world::grass`): the settings it uses, the spots and what grows
//! in each, and per grass its record, model and blades.

use std::collections::BTreeMap;
use std::io::Write;
use std::path::Path;

use assets::{mesh_path, Assets, IniSettings};
use esm::{FormId, LoadOrder};
use world::grass::{
    fill_spots, land_spots, water_level, GrassCatalog, GrassSettings, Ground, NO_WATER,
};
use world::land::CELL_SIZE;
use world::{find_worldspace, WorldGrid};

use crate::render_cmd::world_error;
use crate::CliError;

pub fn grass(
    out: &mut impl Write,
    order: &LoadOrder,
    assets: &Assets,
    data_dir: &Path,
    world: &str,
    square: (i32, i32),
) -> Result<(), CliError> {
    let world = find_worldspace(order, world)
        .map_err(world_error)?
        .ok_or_else(|| {
            CliError::NotFound(format!(
                "no worldspace matches '{world}'. `worlds` lists them."
            ))
        })?;
    let grid = WorldGrid::load(order, world).map_err(world_error)?;
    let ini = IniSettings::load(&assets::default_settings_files(data_dir));
    let settings = GrassSettings::from_ini(|s, k| ini.float(s, k));
    writeln!(
        out,
        "Settings: eval size {} (spots {} units), up to {} grasses per texture, \
         opacity threshold {}, smallest spacing {}, fade {} to {}, wind {} to {}{}",
        settings.eval_size(),
        256 * settings.eval_size(),
        settings.max_types_per_texture + 1,
        settings.threshold(),
        settings.min_grass_size,
        settings.start_fade,
        settings.fade_end(),
        settings.wind_min,
        settings.wind_max,
        if settings.makes_grass() {
            ""
        } else {
            " (grass is off)"
        }
    )?;
    let Some(cell) = grid.cell_at(square) else {
        writeln!(out, "No cell at {},{}.", square.0, square.1)?;
        return Ok(());
    };
    let Some(land) = grid.land(order, square).map_err(world_error)? else {
        writeln!(out, "No terrain record.")?;
        return Ok(());
    };
    let water = water_level(order, &grid, cell);
    writeln!(
        out,
        "Cell {} at {},{}: water {}",
        crate::records::describe_id(order, cell),
        square.0,
        square.1,
        if water == NO_WATER {
            "none".to_string()
        } else {
            format!("at {water}")
        }
    )?;

    let mut around = Vec::new();
    for dx in -1..=1 {
        for dy in -1..=1 {
            let at = (square.0 + dx, square.1 + dy);
            if at != square {
                if let Some(l) = grid.land(order, at).map_err(world_error)? {
                    around.push((at, l));
                }
            }
        }
    }
    let mut ground = Ground::new();
    ground.insert(square, &land);
    for (at, l) in &around {
        ground.insert(*at, l);
    }
    let mut catalog = GrassCatalog::new();
    let origin = [square.0 as f32 * CELL_SIZE, square.1 as f32 * CELL_SIZE];
    let spots = land_spots(&land, origin, Some(order), &mut catalog, &settings);
    let name = |id: FormId| crate::records::describe_id(order, id);

    // Which textures grow what, per quarter.
    for (q, quarter) in land.quarters.iter().enumerate() {
        let label = ["south-west", "south-east", "north-west", "north-east"][q];
        let in_quarter: Vec<_> = spots.iter().filter(|s| s.quarter == q).collect();
        let mut counts: BTreeMap<FormId, (usize, usize)> = BTreeMap::new();
        for spot in &in_quarter {
            for e in &spot.entries {
                let c = counts.entry(e.grass).or_default();
                c.0 += 1;
                c.1 += e.density.iter().filter(|d| **d > 0.0).count();
            }
        }
        let textures: Vec<String> = quarter
            .base
            .into_iter()
            .chain(quarter.layers.iter().map(|l| l.texture))
            .filter(|t| !world::grass::texture_grasses(order, *t).is_empty())
            .map(name)
            .collect();
        writeln!(
            out,
            "  {label}: {} of 16 spots with grass{}",
            in_quarter.len(),
            if textures.is_empty() {
                String::new()
            } else {
                format!(" (from {})", textures.join(", "))
            }
        )?;
        for (id, (spots, samples)) in counts {
            writeln!(
                out,
                "    {}: in {spots} spots, {samples} of their {} samples",
                name(id),
                spots * 9
            )?;
        }
    }

    let batches = fill_spots(&spots, &catalog, &ground, water, &settings);
    let total: usize = batches.iter().map(|b| b.instances.len()).sum();
    // Where to look: the 512-unit patch with the most blades.
    let mut patches: BTreeMap<(i32, i32), (usize, f32)> = BTreeMap::new();
    for i in batches.iter().flat_map(|b| &b.instances) {
        let at = (
            (i.data[0] / 512.0).floor() as i32,
            (i.data[1] / 512.0).floor() as i32,
        );
        let p = patches.entry(at).or_default();
        p.0 += 1;
        p.1 = p.1.max(i.data[2].floor());
    }
    if let Some(((px, py), (count, top))) = patches.iter().max_by_key(|(_, p)| p.0) {
        writeln!(
            out,
            "Most grass: {count} blades around {}, {} (ground up to {top})",
            px * 512 + 256,
            py * 512 + 256
        )?;
    }
    writeln!(out, "{total} blades:")?;
    for batch in &batches {
        let g = &batch.grass;
        writeln!(
            out,
            "  {}: {} blades; density {}%, slope {}-{}°, water rule {} ({} units), \
             position range {}, height range {}, colour range {}, wave period {}, flags {:#x}",
            name(g.form_id),
            batch.instances.len(),
            g.density,
            g.min_slope,
            g.max_slope,
            g.water_rule,
            g.units_from_water,
            g.position_range,
            g.height_range,
            g.color_range,
            g.wave_period,
            g.flags
        )?;
        let (lo_b, hi_b) = batch
            .instances
            .iter()
            .fold((f32::MAX, f32::MIN), |(lo, hi), i| {
                (lo.min(i.brightness), hi.max(i.brightness))
            });
        let (lo_s, hi_s) = batch
            .instances
            .iter()
            .fold((f32::MAX, f32::MIN), |(lo, hi), i| {
                (lo.min(i.scale), hi.max(i.scale))
            });
        writeln!(
            out,
            "    brightness {lo_b:.3} to {hi_b:.3}, size {lo_s:.2} to {hi_s:.2}"
        )?;
        for i in batch.instances.iter().take(3) {
            let [x, y, z, w] = i.data;
            let n = [x, y, z].map(|c| 2.0 * (c - c.floor()) - 1.0);
            writeln!(
                out,
                "    e.g. at {:.0}, {:.0}, {:.0}, normal ({:.2}, {:.2}, {:.2}), w {w:.3}",
                x.floor(),
                y.floor(),
                z.floor(),
                n[0],
                n[1],
                n[2]
            )?;
        }
        match g.model.as_deref() {
            Some(model) => {
                let path = mesh_path(model);
                match assets.read(&path) {
                    Ok(Some(bytes)) => describe_model(out, &path, &bytes)?,
                    _ => writeln!(out, "    model {path}: not found")?,
                }
            }
            None => writeln!(out, "    no model")?,
        }
    }
    Ok(())
}

fn describe_model(out: &mut impl Write, path: &str, bytes: &[u8]) -> Result<(), CliError> {
    let scene = nif::Nif::parse(bytes.to_vec())?.scene()?;
    for mesh in &scene.meshes {
        let moved = mesh.transform != nif::math::Transform::IDENTITY;
        writeln!(
            out,
            "    model {path}: {} vertices, {} triangles, texture {}, alpha test {}{}",
            mesh.positions.len(),
            mesh.triangles.len(),
            mesh.diffuse_texture().unwrap_or("(none)"),
            mesh.alpha
                .as_ref()
                .filter(|a| a.testing())
                .map_or("none".to_string(), |a| format!("> {}", a.threshold)),
            if moved {
                format!(", node transform {:?}", mesh.transform)
            } else {
                String::new()
            }
        )?;
    }
    Ok(())
}

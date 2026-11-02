//! `worlds` and `land <WORLD> <X> <Y>`: the outdoor worldspaces, and one
//! grid square's terrain and objects.

use std::io::Write;

use esm::LoadOrder;
use world::land::{QUARTER_GRID, SPACING};
use world::{find_worldspace, worldspaces, LandTexture, WorldGrid};

use crate::render_cmd::world_error;
use crate::CliError;

pub fn worlds(out: &mut impl Write, order: &LoadOrder) -> Result<(), CliError> {
    let list = worldspaces(order);
    writeln!(out, "{} worldspaces:", list.len())?;
    for w in list {
        let parent = match w.parent {
            Some((p, _)) => match order.get(p).and_then(|r| r.editor_id().ok().flatten()) {
                Some(e) => format!(", inside {e}"),
                None => format!(", inside {p}"),
            },
            None => String::new(),
        };
        writeln!(
            out,
            "  {} {:<28} {}{}",
            w.form_id,
            w.editor_id.as_deref().unwrap_or("-"),
            w.name
                .as_deref()
                .map(|n| format!("\"{n}\""))
                .unwrap_or_default(),
            parent
        )?;
    }
    Ok(())
}

fn find_world(order: &LoadOrder, query: &str) -> Result<esm::FormId, CliError> {
    find_worldspace(order, query)
        .map_err(world_error)?
        .ok_or_else(|| {
            CliError::NotFound(format!(
                "no worldspace matches '{query}'. `worlds` lists them."
            ))
        })
}

pub fn land(
    out: &mut impl Write,
    order: &LoadOrder,
    world: &str,
    square: (i32, i32),
) -> Result<(), CliError> {
    let world = find_world(order, world)?;
    let grid = WorldGrid::load(order, world).map_err(world_error)?;
    writeln!(
        out,
        "{}: {} cells; default land height {}, water {}",
        grid.world.label(),
        grid.cells.len(),
        grid.world.default_land_height,
        grid.world.default_water_height
    )?;
    let Some(cell) = grid.cell_at(square) else {
        writeln!(out, "No cell at {},{}.", square.0, square.1)?;
        return Ok(());
    };
    let loaded = grid
        .load_square(order, square)
        .map_err(world_error)?
        .expect("the cell exists");
    writeln!(
        out,
        "Cell {} at {},{}: {} objects, {} markers, {} people and creatures",
        crate::records::describe_id(order, cell),
        square.0,
        square.1,
        loaded.objects.len(),
        loaded.markers.len(),
        loaded.actors.len()
    )?;
    if let Some(l) = &loaded.info.lighting {
        writeln!(
            out,
            "Lighting ({}): ambient {:?}, sun {:?} toward {:?}, fog {:?} from {} to {} (power {})",
            loaded.info.lighting_source,
            l.ambient,
            l.directional,
            l.toward_directional().map(|c| (c * 100.0).round() / 100.0),
            l.fog_color,
            l.fog_near,
            l.fog_far,
            l.fog_power
        )?;
    }
    if let Some([upper, horizon, lower]) = loaded.info.sky {
        writeln!(
            out,
            "Sky by day: upper {upper:?}, horizon {horizon:?}, lower {lower:?}"
        )?;
    }
    if let Some(space) = &loaded.info.image_space {
        writeln!(
            out,
            "Image space: {}",
            space.editor_id.as_deref().unwrap_or("(unnamed)")
        )?;
    }
    let Some(land) = grid.land(order, square).map_err(world_error)? else {
        writeln!(out, "No terrain record.")?;
        return Ok(());
    };
    writeln!(out, "Terrain {} flags {:#x}", land.form_id, land.flags)?;
    if let Some(h) = &land.heights {
        let lo = h.iter().copied().fold(f32::MAX, f32::min);
        let hi = h.iter().copied().fold(f32::MIN, f32::max);
        writeln!(
            out,
            "  heights {lo:.0} to {hi:.0} (corners SW {:.0}, SE {:.0}, NW {:.0}, NE {:.0}); points {SPACING} units apart",
            h[0],
            h[32],
            h[32 * 33],
            h[32 * 33 + 32]
        )?;
    }
    if let Some(n) = &land.normals {
        let steepest = n.iter().map(|v| v[2]).fold(1.0f32, f32::min);
        writeln!(
            out,
            "  normals: steepest {:.0}° from flat",
            steepest.clamp(-1.0, 1.0).acos().to_degrees()
        )?;
    }
    if let Some(c) = &land.colors {
        let white = c.iter().filter(|c| **c == [255, 255, 255]).count();
        writeln!(out, "  vertex colours: {} of {} white", white, c.len())?;
    }
    let name = |id: Option<esm::FormId>| match id {
        None => "(default)".to_string(),
        Some(id) => match LandTexture::load(order, id) {
            Some(t) => format!(
                "{} {}",
                t.editor_id.unwrap_or_else(|| id.to_string()),
                t.diffuse.unwrap_or_else(|| "(no diffuse)".into())
            ),
            None => format!("{id} (not a land texture)"),
        },
    };
    for (i, q) in land.quarters.iter().enumerate() {
        let label = ["south-west", "south-east", "north-west", "north-east"][i];
        writeln!(out, "  {label}: base {}", name(q.base))?;
        for l in &q.layers {
            let painted = l.opacity.iter().filter(|o| **o > 0.0).count();
            let full = l.opacity.iter().filter(|o| **o >= 1.0).count();
            let max = l.opacity.iter().copied().fold(0.0f32, f32::max);
            writeln!(
                out,
                "    layer {:>2}: {} ({painted} of {} points painted, {full} fully, highest {max:.3})",
                l.layer,
                name(Some(l.texture)),
                QUARTER_GRID * QUARTER_GRID
            )?;
        }
        // Where layers overlap, the sum of their opacities.
        let mut most = 0.0f32;
        for p in 0..QUARTER_GRID * QUARTER_GRID {
            let sum: f32 = q.layers.iter().map(|l| l.opacity[p]).sum();
            most = most.max(sum);
        }
        if q.layers.len() > 1 {
            writeln!(out, "    most opacity summed at one point: {most:.3}")?;
        }
    }
    Ok(())
}

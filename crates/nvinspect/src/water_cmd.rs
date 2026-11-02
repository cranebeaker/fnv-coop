//! `water <WORLD|CELL|TYPE> [X Y]`: where the water is and how it looks.
//!
//! - a water type (`WATR`): its values as the game's shaders get them;
//! - a worldspace: its default water, every square whose water stands
//!   above part of its terrain (water you can see), and its placed water;
//! - a worldspace and a square: that square's water and placed water;
//! - an interior cell: its placed water.

use std::io::Write;

use esm::{sig, FormId, FourCC, LoadOrder};
use world::water::{cell_water, placed_flags, PlaceableWater, WaterType, WorldWater};
use world::{find_worldspace, Land, WorldGrid};

use crate::records::{describe_id, find_record};
use crate::render_cmd::world_error;
use crate::CliError;

const WATR: FourCC = FourCC::new(b"WATR");
const PWAT: FourCC = FourCC::new(b"PWAT");
const REFR: FourCC = FourCC::new(b"REFR");
const NAME: FourCC = FourCC::new(b"NAME");

pub fn water(
    out: &mut impl Write,
    order: &LoadOrder,
    target: &str,
    square: Option<(i32, i32)>,
) -> Result<(), CliError> {
    if let Some(world) = find_worldspace(order, target).map_err(world_error)? {
        return match square {
            Some(square) => world_square(out, order, world, square),
            None => whole_world(out, order, world),
        };
    }
    let rr = find_record(order, target)?;
    match rr.entry.header.kind {
        k if k == WATR => {
            let w = WaterType::load(order, rr.form_id)
                .ok_or_else(|| CliError::NotFound(format!("{target} can't be read")))?;
            water_type(out, &w)
        }
        k if k == PWAT => {
            let p = PlaceableWater::load(order, rr.form_id)
                .ok_or_else(|| CliError::NotFound(format!("{target} can't be read")))?;
            placeable(out, order, &p)
        }
        k if k == sig::CELL => cell(out, order, rr.form_id),
        other => Err(CliError::NotFound(format!(
            "{target} is a {other} record; give a worldspace, a cell, a WATR or a PWAT"
        ))),
    }
}

fn water_type(out: &mut impl Write, w: &WaterType) -> Result<(), CliError> {
    writeln!(
        out,
        "{} {}: noise {}, opacity {} ({:.2}), flags {:#04x}",
        w.form_id,
        w.label(),
        w.noise.as_deref().unwrap_or("(none)"),
        w.opacity,
        f32::from(w.opacity) / 100.0,
        w.flags
    )?;
    let Some(v) = &w.visual else {
        writeln!(out, "  no 196-byte DNAM")?;
        return Ok(());
    };
    let rgb = |c: [u8; 4]| format!("({}, {}, {})", c[0], c[1], c[2]);
    writeln!(
        out,
        "  colours: shallow {}, deep {}, reflection {}",
        rgb(v.shallow),
        rgb(v.deep),
        rgb(v.reflection)
    )?;
    writeln!(
        out,
        "  sun power {} (VarAmounts.x), reflectivity {} (.y), distortion {} (.w)",
        v.sun_power, v.reflectivity, v.distortion
    )?;
    writeln!(
        out,
        "  fresnel {} (FresnelRI.x), reflection multiplier {} -> {} (.w), shininess {} (.z)",
        v.fresnel,
        v.reflection_multiplier_raw,
        v.reflection_multiplier(),
        v.shininess
    )?;
    writeln!(
        out,
        "  fog above water {} to {}, amount {}; under water {} to {}, amount {}",
        v.fog_near,
        v.fog_far,
        v.fog_amount,
        v.underwater_fog_near,
        v.underwater_fog_far,
        v.underwater_fog_amount
    )?;
    writeln!(
        out,
        "  depth falloff {:?} -> {:?}",
        v.depth_falloff,
        v.falloff_constant()
    )?;
    writeln!(
        out,
        "  ripples: one noise texture per {} units, normal strength {}",
        v.noise_tile_size, v.noise_scale
    )?;
    for i in 0..3 {
        writeln!(
            out,
            "    layer {}: toward {}°, {} a second, amplitude {}, uv scale {} -> {}",
            i + 1,
            v.wind_directions[i],
            v.wind_speeds[i],
            v.amplitudes[i],
            v.uv_scales[i],
            v.noise_uv_scale(i)
        )?;
    }
    writeln!(
        out,
        "  lit water: light radius {}, brightness {}; wading displacement strength {}",
        v.light_radius, v.light_brightness, v.displacement_strength
    )?;
    Ok(())
}

fn flag_names(flags: u32) -> String {
    use placed_flags::*;
    let names = [
        (REFLECTS, "reflects"),
        (REFRACTS, "refracts"),
        (DEPTH, "depth"),
        (OBJECT_UVS, "object UVs"),
        (NO_UNDERWATER_FOG, "no underwater fog"),
    ];
    let on: Vec<&str> = names
        .iter()
        .filter(|(bit, _)| flags & bit != 0)
        .map(|(_, n)| *n)
        .collect();
    format!("{flags:#010x} ({})", on.join(", "))
}

fn placeable(out: &mut impl Write, order: &LoadOrder, p: &PlaceableWater) -> Result<(), CliError> {
    writeln!(
        out,
        "{} {}: model {}, flags {}, water {}",
        p.form_id,
        p.editor_id.as_deref().unwrap_or("-"),
        p.model.as_deref().unwrap_or("(none)"),
        flag_names(p.flags),
        p.water_type
            .map_or_else(|| "(none)".to_string(), |w| describe_id(order, w))
    )?;
    // Where it's placed.
    let placed: Vec<_> = placed_water(order, order.records_of_type(REFR))
        .into_iter()
        .filter(|(_, base, _, _)| base.form_id == p.form_id)
        .collect();
    writeln!(out, "Placed {} times:", placed.len())?;
    for (id, _, at, scale) in placed {
        let place = order
            .get(id)
            .and_then(|r| order.cell_of(&r))
            .map_or_else(|| "?".to_string(), |c| describe_id(order, c));
        writeln!(
            out,
            "  {id} in {place} at ({:.0}, {:.0}, {:.0}) scale {scale:.2}",
            at[0], at[1], at[2]
        )?;
    }
    Ok(())
}

/// Placed water references among some references.
fn placed_water<'a>(
    order: &'a LoadOrder,
    refs: impl Iterator<Item = esm::RecordRef<'a>>,
) -> Vec<(FormId, PlaceableWater, [f32; 3], f32)> {
    let mut found = Vec::new();
    for rr in refs {
        if rr.entry.header.kind != REFR || rr.entry.header.is_deleted() {
            continue;
        }
        let Ok(record) = rr.record() else { continue };
        let Some(base) = record.get(NAME).filter(|s| s.data.len() >= 4).map(|s| {
            rr.plugin
                .to_global(FormId(u32::from_le_bytes(s.data[..4].try_into().unwrap())))
        }) else {
            continue;
        };
        if !order.get(base).is_some_and(|b| b.entry.header.kind == PWAT) {
            continue;
        }
        let Some(p) = PlaceableWater::load(order, base) else {
            continue;
        };
        let f = |data: &[u8], at: usize| f32::from_le_bytes(data[at..at + 4].try_into().unwrap());
        let position = record
            .get(sig::DATA)
            .filter(|s| s.data.len() >= 12)
            .map_or([0.0; 3], |s| [f(&s.data, 0), f(&s.data, 4), f(&s.data, 8)]);
        let scale = record
            .get(FourCC::new(b"XSCL"))
            .filter(|s| s.data.len() >= 4)
            .map_or(1.0, |s| f(&s.data, 0));
        found.push((rr.form_id, p, position, scale));
    }
    found
}

fn print_placed(
    out: &mut impl Write,
    order: &LoadOrder,
    placed: &[(FormId, PlaceableWater, [f32; 3], f32)],
) -> Result<(), CliError> {
    for (id, p, at, scale) in placed {
        writeln!(
            out,
            "  {id} {} at ({:.0}, {:.0}, {:.0}) scale {scale:.2}: flags {}, water {}",
            p.editor_id.as_deref().unwrap_or("-"),
            at[0],
            at[1],
            at[2],
            flag_names(p.flags),
            p.water_type
                .map_or_else(|| "(none)".to_string(), |w| describe_id(order, w))
        )?;
    }
    Ok(())
}

fn world_header(
    out: &mut impl Write,
    order: &LoadOrder,
    world: FormId,
) -> Result<WorldGrid, CliError> {
    let grid = WorldGrid::load(order, world).map_err(world_error)?;
    let water = WorldWater::load(order, world);
    writeln!(
        out,
        "{}: default water height {}, water {}, ripple noise {}",
        grid.world.label(),
        water.as_ref().map_or(0.0, |w| w.default_height),
        water
            .as_ref()
            .and_then(|w| w.water_type)
            .map_or_else(|| "(none)".to_string(), |w| describe_id(order, w)),
        water
            .as_ref()
            .and_then(|w| w.noise.clone())
            .unwrap_or_else(|| "(none)".into())
    )?;
    Ok(grid)
}

fn whole_world(out: &mut impl Write, order: &LoadOrder, world: FormId) -> Result<(), CliError> {
    let grid = world_header(out, order, world)?;
    let mut squares: Vec<((i32, i32), FormId)> = grid.cells.iter().map(|(s, c)| (*s, *c)).collect();
    squares.sort();
    let mut with_water = 0;
    let mut own_height = 0;
    let mut visible = Vec::new();
    for (square, cell) in squares {
        let Some(w) = cell_water(order, cell).map_err(world_error)? else {
            continue;
        };
        with_water += 1;
        if !w.default_height {
            own_height += 1;
        }
        let Some(land) = Land::of_cell(order, cell).map_err(world_error)? else {
            continue;
        };
        let Some(heights) = &land.heights else {
            continue;
        };
        let lo = heights.iter().copied().fold(f32::MAX, f32::min);
        let hi = heights.iter().copied().fold(f32::MIN, f32::max);
        if w.height > lo {
            let wet = heights.iter().filter(|&&h| h < w.height).count();
            visible.push((square, cell, w, lo, hi, wet, heights.len()));
        }
    }
    writeln!(
        out,
        "{} squares, {with_water} with water ({own_height} at their own height); {} where it stands above part of the terrain:",
        grid.cells.len(),
        visible.len()
    )?;
    for (square, cell, w, lo, hi, wet, all) in &visible {
        writeln!(
            out,
            "  {:>4},{:<4} {}: water at {:.0} ({}), terrain {lo:.0} to {hi:.0}, {}% of points under",
            square.0,
            square.1,
            describe_id(order, *cell),
            w.height,
            w.water_type
                .and_then(|t| WaterType::load(order, t))
                .map_or_else(|| "no type".to_string(), |t| t.label()),
            100 * wet / all.max(&1)
        )?;
    }
    let placed = placed_water(
        order,
        order
            .records_of_type(REFR)
            .filter(|r| order.world_of(r) == Some(world)),
    );
    writeln!(out, "{} placed water:", placed.len())?;
    print_placed(out, order, &placed)?;
    Ok(())
}

fn world_square(
    out: &mut impl Write,
    order: &LoadOrder,
    world: FormId,
    square: (i32, i32),
) -> Result<(), CliError> {
    let grid = world_header(out, order, world)?;
    let Some(cell) = grid.cell_at(square) else {
        writeln!(out, "No cell at {},{}.", square.0, square.1)?;
        return Ok(());
    };
    match cell_water(order, cell).map_err(world_error)? {
        Some(w) => {
            writeln!(
                out,
                "Square {},{} ({}): water at {}{}, type {}",
                square.0,
                square.1,
                describe_id(order, cell),
                w.height,
                if w.default_height {
                    " (the worldspace's default)"
                } else {
                    ""
                },
                w.water_type
                    .map_or_else(|| "(none)".to_string(), |t| describe_id(order, t))
            )?;
            if let Some(t) = w.water_type.and_then(|t| WaterType::load(order, t)) {
                water_type(out, &t)?;
            }
            // Where the ground is under the water: every fourth terrain
            // point (512 units apart), north at the top; `~~~~` under
            // water, else the ground's height above the water.
            if let Some(heights) = Land::of_cell(order, cell)
                .map_err(world_error)?
                .and_then(|l| l.heights)
            {
                writeln!(
                    out,
                    "  ground above the water (every 512 units from x {}, y {}; north up):",
                    square.0 * 4096,
                    square.1 * 4096
                )?;
                for row in (0..33).rev().step_by(4) {
                    let line: Vec<String> = (0..33)
                        .step_by(4)
                        .map(|col| {
                            let above = heights[row * 33 + col] - w.height;
                            if above < 0.0 {
                                " ~~~~".to_string()
                            } else {
                                format!("{above:>5.0}")
                            }
                        })
                        .collect();
                    writeln!(
                        out,
                        "    y {:>6}: {}",
                        square.1 * 4096 + row as i32 * 128,
                        line.join("")
                    )?;
                }
            }
        }
        None => writeln!(out, "Square {},{}: no water", square.0, square.1)?,
    }
    let loaded = grid.load_square(order, square).map_err(world_error)?;
    let ids: Vec<FormId> = loaded
        .iter()
        .flat_map(|l| l.objects.iter())
        .filter(|o| o.base_type == PWAT)
        .map(|o| o.form_id)
        .collect();
    let placed = placed_water(order, ids.iter().filter_map(|&id| order.get(id)));
    writeln!(out, "{} placed water:", placed.len())?;
    print_placed(out, order, &placed)?;
    Ok(())
}

fn cell(out: &mut impl Write, order: &LoadOrder, cell: FormId) -> Result<(), CliError> {
    let info = world::cell_info(order, cell).map_err(world_error)?;
    match cell_water(order, cell).map_err(world_error)? {
        Some(w) => writeln!(
            out,
            "{}: water at {}, type {}",
            info.label(),
            w.height,
            w.water_type
                .map_or_else(|| "(none)".to_string(), |t| describe_id(order, t))
        )?,
        None => writeln!(
            out,
            "{}: no water of its own{}",
            info.label(),
            if info.interior {
                " (interiors never have any; only placed water)"
            } else {
                ""
            }
        )?,
    }
    let placed = placed_water(order, order.references_in_cell(cell).into_iter());
    writeln!(out, "{} placed water:", placed.len())?;
    print_placed(out, order, &placed)?;
    Ok(())
}

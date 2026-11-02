//! `walk <CELL>`: walks the player's capsule through a cell's real
//! collision from where the player arrives, in each direction, and says
//! how far it got. A check on the collision and the character controller
//! (`physics`) without the viewer.

use std::io::Write;

use assets::Assets;
use esm::LoadOrder;
use physics::{Character, CharacterShape};
use preview::cell::build_scene_with;
use world::land::CELL_SIZE;
use world::{load_cell, Arrival, RotationConvention, WorldGrid};

use crate::render_cmd::{single_cell, world_error};
use crate::{CliError, Options};

/// The game's running speed: `fMoveBaseSpeed` 77 × `fMoveRunMult` 4.
const RUN_SPEED: f32 = 308.0;
const STEP: f32 = 1.0 / 60.0;

pub fn walk(
    out: &mut impl Write,
    order: &LoadOrder,
    assets: &Assets,
    query: &str,
    seconds: f32,
    options: &Options,
) -> Result<(), CliError> {
    let cell_id = single_cell(order, query)?;
    let info = world::cell_info(order, cell_id).map_err(world_error)?;
    let keep_root = options.render.keep_root_transforms;
    let (label, collider, arrival) = match (info.interior, info.world, info.grid) {
        (false, Some(world), Some(square)) => {
            outdoors(out, order, assets, world, square, keep_root)?
        }
        _ => {
            let loaded = load_cell(order, cell_id).map_err(world_error)?;
            let label = loaded.info.label();
            let scene = build_scene_with(assets, loaded, keep_root);
            for (path, e) in &scene.report.unreadable_collision {
                writeln!(out, "  unreadable collision: {path}: {e}")?;
            }
            describe_collision(out, &scene)?;
            let arrival = scene.cell.arrivals.first().cloned().ok_or_else(|| {
                CliError::NotFound(format!("{label} has no arrival point to start from"))
            })?;
            (label, scene.collider(RotationConvention::DEFAULT), arrival)
        }
    };
    writeln!(
        out,
        "{label}: {} collision triangles",
        collider.triangle_count()
    )?;
    let shape = CharacterShape::PLAYER;
    let mut settled = Character::new(arrival.position);
    for _ in 0..120 {
        settled.update(&collider, &shape, [0.0, 0.0], STEP);
    }
    writeln!(
        out,
        "Arrives at {:.0}, {:.0}, {:.0} via {}; settles at z {:.1} ({}).",
        arrival.position[0],
        arrival.position[1],
        arrival.position[2],
        arrival.via,
        settled.feet[2],
        if settled.on_ground {
            "standing"
        } else {
            "still falling: no floor under the arrival point?"
        }
    )?;
    let heading = arrival.rotation[2];
    writeln!(
        out,
        "Running {seconds} s each way at {RUN_SPEED} units/s (facing {:.0}° at the start):",
        heading.to_degrees()
    )?;
    for (name, turn) in [
        ("forward", 0.0f32),
        ("right", 90.0),
        ("back", 180.0),
        ("left", 270.0),
    ] {
        let angle = heading + turn.to_radians();
        // Heading is clockwise from north (+y).
        let velocity = [angle.sin() * RUN_SPEED, angle.cos() * RUN_SPEED];
        let mut c = settled.clone();
        let mut lowest = c.feet[2];
        let mut highest = c.feet[2];
        let steps = (seconds / STEP).round() as usize;
        // Where the height changed by more than a few units in a frame:
        // steps up and drops.
        let mut changes = Vec::new();
        for _ in 0..steps {
            let before = c.feet;
            c.update(&collider, &shape, velocity, STEP);
            lowest = lowest.min(c.feet[2]);
            highest = highest.max(c.feet[2]);
            let rise = c.feet[2] - before[2];
            if rise.abs() > 4.0 {
                changes.push(format!(
                    "{} {:.0} at {:.0}, {:.0}",
                    if rise > 0.0 { "up" } else { "down" },
                    rise.abs(),
                    before[0],
                    before[1]
                ));
            }
        }
        let dx = c.feet[0] - settled.feet[0];
        let dy = c.feet[1] - settled.feet[1];
        let along = dx * angle.sin() + dy * angle.cos();
        writeln!(
            out,
            "  {name:<8} {along:>7.0} of {:>5.0} units, ends at {:.0}, {:.0}, {:.0}; height {:.0}..{:.0}{}",
            RUN_SPEED * seconds,
            c.feet[0],
            c.feet[1],
            c.feet[2],
            lowest,
            highest,
            if c.on_ground { "" } else { ", in the air" }
        )?;
        if !changes.is_empty() {
            writeln!(out, "           {}", changes.join("; "))?;
        }
    }
    Ok(())
}

/// What a place's collision is made of: the doors (and which start open),
/// the collision markers' boxes, the people (who block the player in the
/// viewer), and the biggest objects drawn without anything solid, which the
/// player walks through (in the game too, if their models have none).
fn describe_collision(
    out: &mut impl Write,
    scene: &preview::cell::CellScene,
) -> Result<(), CliError> {
    let convention = RotationConvention::DEFAULT;
    let blocks = |p: &nif::CollisionPart| nif::collision::layers::blocks_walking(p.layer);
    let mut solid = vec![false; scene.cell.objects.len()];
    let mut moving = 0;
    for instance in &scene.instances {
        let parts = &scene.models[instance.model].collision;
        if parts.iter().any(blocks) {
            solid[instance.object] = true;
        }
        if parts.iter().any(|p| blocks(p) && p.dynamic) {
            moving += 1;
        }
    }
    let doors: Vec<&world::Placement> = scene
        .cell
        .objects
        .iter()
        .filter(|o| preview::cell::is_opening_door(o))
        .collect();
    let load_doors = scene
        .cell
        .objects
        .iter()
        .filter(|o| o.teleport.is_some())
        .count();
    let markers: Vec<&world::Placement> = scene
        .cell
        .markers
        .iter()
        .filter(|m| m.base == world::COLLISION_MARKER && m.primitive.is_some())
        .collect();
    let shape_count = |s: u32| {
        markers
            .iter()
            .filter(|m| m.primitive.is_some_and(|p| p.shape == s))
            .count()
    };
    writeln!(
        out,
        "  solid: {} of {} placed objects ({moving} of them moving clutter, solid where placed); \
         {} doors that open ({} placed open), {load_doors} load doors; \
         {} collision markers ({} boxes, {} planes, {} spheres); {} people",
        solid.iter().filter(|&&s| s).count(),
        scene.cell.objects.len(),
        doors.len(),
        doors.iter().filter(|d| d.open_by_default).count(),
        markers.len(),
        shape_count(1),
        shape_count(3),
        shape_count(2),
        scene.cell.actors.len()
    )?;
    // The biggest things drawn with nothing solid.
    let bounds = scene.object_bounds(convention);
    let mut open: Vec<(f32, &world::Placement)> = bounds
        .iter()
        .enumerate()
        .filter_map(|(i, b)| {
            let (lo, hi, effect) = (*b)?;
            let object = &scene.cell.objects[i];
            if solid[i]
                || effect
                || object.actor.is_some()
                || object.light.is_some() && object.model.is_none()
            {
                return None;
            }
            let size = (0..3).map(|k| hi[k] - lo[k]).fold(0.0f32, f32::max);
            (size >= 64.0).then_some((size, object))
        })
        .collect();
    open.sort_by(|a, b| b.0.total_cmp(&a.0));
    if !open.is_empty() {
        writeln!(
            out,
            "  drawn without collision ({} at least 64 units across; the biggest):",
            open.len()
        )?;
        for (size, object) in open.iter().take(8) {
            writeln!(
                out,
                "    {:>5.0}  {} {} at {:.0}, {:.0}, {:.0}",
                size,
                object.form_id,
                object
                    .model
                    .as_deref()
                    .or(object.base_editor_id.as_deref())
                    .unwrap_or("?"),
                object.position[0],
                object.position[1],
                object.position[2]
            )?;
        }
    }
    Ok(())
}

/// An outdoor cell: the collision (objects and terrain) of it and the
/// squares around, and a start on the ground in its middle.
fn outdoors(
    out: &mut impl Write,
    order: &LoadOrder,
    assets: &Assets,
    world: esm::FormId,
    square: (i32, i32),
    keep_root: bool,
) -> Result<(String, physics::Collider, Arrival), CliError> {
    let grid = WorldGrid::load(order, world).map_err(world_error)?;
    let mut collider = physics::Collider::new();
    for dx in -1..=1 {
        for dy in -1..=1 {
            let at = (square.0 + dx, square.1 + dy);
            let Some(loaded) = grid.load_square(order, at).map_err(world_error)? else {
                continue;
            };
            let scene = build_scene_with(assets, loaded, keep_root);
            for (path, e) in &scene.report.unreadable_collision {
                writeln!(out, "  unreadable collision: {path}: {e}")?;
            }
            if at == square {
                describe_collision(out, &scene)?;
            }
            collider.extend(&scene.collider(RotationConvention::DEFAULT));
            if let Some(land) = grid.land(order, at).map_err(world_error)? {
                let origin = [at.0 as f32 * CELL_SIZE, at.1 as f32 * CELL_SIZE];
                for q in 0..4 {
                    if let Some(mesh) = land.quarter_mesh(q, origin) {
                        let triangles: Vec<[u32; 3]> = mesh
                            .indices
                            .chunks_exact(3)
                            .map(|t| [t[0], t[1], t[2]].map(u32::from))
                            .collect();
                        collider.add_solid(&mesh.positions, &triangles, physics::TERRAIN_SHELL, 0);
                    }
                }
            }
        }
    }
    let middle = [
        (square.0 as f32 + 0.5) * CELL_SIZE,
        (square.1 as f32 + 0.5) * CELL_SIZE,
    ];
    let top = 100_000.0;
    let ground = collider
        .raycast([middle[0], middle[1], top], [0.0, 0.0, -1.0], 2.0 * top)
        .map(|(d, _)| top - d)
        .ok_or_else(|| CliError::NotFound("no ground in the middle of the cell".into()))?;
    let label = format!(
        "{} (square {},{} of {} and the eight around it)",
        world::cell_info(order, grid.cell_at(square).unwrap_or(world))
            .map(|i| i.label())
            .unwrap_or_default(),
        square.0,
        square.1,
        grid.world.label()
    );
    let arrival = Arrival {
        position: [middle[0], middle[1], ground],
        rotation: [0.0; 3],
        via: "the middle of the cell, on the ground".into(),
    };
    Ok((label, collider, arrival))
}

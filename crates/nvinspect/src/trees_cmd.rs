//! `trees <TREE> [SEED]`: a tree record grown as the game grows it
//! (`world::tree`, `cellview::tree`, `speedtree`): its values, the size it
//! grew to, its branches and leaves per level of detail, its textures and
//! the distances where each level shows. `trees <WORLD> <X> <Y>`: the trees
//! placed on one outdoor grid square.

use std::io::Write;
use std::path::Path;

use assets::{Assets, IniSettings};
use esm::LoadOrder;
use speedtree::lod::{lod_level, visible, LodLimits};
use world::tree::{TreeBase, TreeSettings};
use world::{find_worldspace, WorldGrid};

use crate::render_cmd::world_error;
use crate::CliError;

pub fn trees(
    out: &mut impl Write,
    order: &LoadOrder,
    assets: &Assets,
    data_dir: &Path,
    args: &[String],
) -> Result<(), CliError> {
    let ini = IniSettings::load(&assets::default_settings_files(data_dir));
    let settings = TreeSettings::load(order, |s, k| ini.float(s, k));
    if args.len() == 3 {
        return square(out, order, &settings, args);
    }
    let id = crate::records::find_record(order, &args[0])?.form_id;
    let base = TreeBase::load(order, id)
        .ok_or_else(|| CliError::NotFound(format!("'{}' isn't a tree (TREE) record", args[0])))?;
    let seed = match args.get(1) {
        Some(s) => s
            .parse::<i32>()
            .map_err(|_| CliError::Usage(format!("'{s}' isn't a seed (a whole number)")))?,
        None => base.seed(Some(0)),
    };
    writeln!(
        out,
        "{} {}: model {} ({}), leaves {}",
        id,
        base.editor_id.as_deref().unwrap_or("-"),
        base.model,
        base.spt_path(),
        base.leaf_texture_path().as_deref().unwrap_or("none")
    )?;
    writeln!(
        out,
        "  seeds {:?}; curvature {}, bud angles {}..{}, branch dimming {}, leaf dimming {}, \
         shadow radius {}, rock speed {}, rustle speed {}, billboard {}×{}",
        base.seeds,
        base.curvature,
        base.min_bud_angle,
        base.max_bud_angle,
        base.branch_dimming,
        base.leaf_dimming,
        base.shadow_radius,
        base.rock_speed,
        base.rustle_speed,
        base.billboard[0],
        base.billboard[1]
    )?;
    writeln!(
        out,
        "  settings: size × {}, levels of detail from {} to {} units",
        settings.size_conversion, settings.near, settings.far
    )?;
    let path = base.spt_path();
    let bytes = assets
        .read(&path)
        .ok()
        .flatten()
        .ok_or_else(|| CliError::NotFound(format!("{path} isn't in the archives or Data")))?;
    let spt = speedtree::SptFile::parse(&bytes).map_err(|e| CliError::InFile {
        path: path.clone(),
        message: e.to_string(),
    })?;
    let missing = std::cell::RefCell::new(Vec::new());
    let model = cellview::tree::grow_model(base, seed, &path, &spt, &settings, |p| {
        let found = assets.read(p).ok().flatten().is_some();
        if !found {
            missing.borrow_mut().push(p.to_string());
        }
        None
    });
    writeln!(
        out,
        "Grown with seed {seed}: size {:.4}, reaching {:.1} units from its base; trunk {:.1} across, first branch at {:.1}",
        model.size, model.radius, model.trunk.0, model.trunk.1
    )?;
    for (i, b) in model.branches.iter().enumerate() {
        writeln!(
            out,
            "  branch level {i}: {} vertices, a strip of {} ({} triangles)",
            b.positions.len(),
            b.strip.len(),
            b.triangles().len()
        )?;
    }
    for (i, l) in model.leaves.iter().enumerate() {
        writeln!(
            out,
            "  leaf level {i}: {} leaves, cards × {}",
            l.positions.len() / 4,
            l.blend.first().map_or(1.0, |b| b[3])
        )?;
    }
    writeln!(
        out,
        "  leaves rock {} and rustle {} (the file's), curvature {}; level change mode {}, \
         cross-fade width {}, start {}, exponent {}",
        model.rock_amount,
        model.rustle_amount,
        model.curvature,
        model.fade.mode,
        model.fade.width,
        model.fade.start,
        model.fade.exponent
    )?;
    for p in missing.borrow().iter() {
        writeln!(out, "  missing texture: {p}")?;
    }
    // Where the levels change, by (fast) distance.
    let limits = LodLimits {
        near: settings.near,
        far: settings.far,
    };
    let mut last = None;
    let mut d = 0.0f32;
    writeln!(out, "What's drawn by distance from the camera:")?;
    while d <= settings.far + 256.0 {
        let lod = lod_level([0.0; 3], [d, 0.0, 0.0], limits);
        let v = visible(lod, model.branch_levels(), model.leaf_levels(), model.fade);
        let shown = (
            v.branches.map(|b| b.0),
            v.leaves.iter().map(|l| l.0).collect::<Vec<_>>(),
        );
        if last.as_ref() != Some(&shown) {
            writeln!(
                out,
                "  from {d:>6.0}: branches {}, leaves {}",
                match shown.0 {
                    Some(l) if model.branches[l as usize].strip.is_empty() =>
                        format!("level {l} (empty)"),
                    Some(l) => format!("level {l}"),
                    None => "none".into(),
                },
                if shown.1.is_empty() {
                    "none".to_string()
                } else {
                    shown
                        .1
                        .iter()
                        .map(|l| format!("level {l}"))
                        .collect::<Vec<_>>()
                        .join(" + ")
                }
            )?;
            last = Some(shown);
        }
        d += 16.0;
    }
    Ok(())
}

fn square(
    out: &mut impl Write,
    order: &LoadOrder,
    _settings: &TreeSettings,
    args: &[String],
) -> Result<(), CliError> {
    let coordinate = |s: &String| {
        s.parse::<i32>()
            .map_err(|_| CliError::Usage(format!("'{s}' isn't a grid coordinate (a whole number)")))
    };
    let at = (coordinate(&args[1])?, coordinate(&args[2])?);
    let world = find_worldspace(order, &args[0])
        .map_err(world_error)?
        .ok_or_else(|| {
            CliError::NotFound(format!(
                "no worldspace matches '{}'. `worlds` lists them.",
                args[0]
            ))
        })?;
    let grid = WorldGrid::load(order, world).map_err(world_error)?;
    let Some(cell) = grid.load_square(order, at).map_err(world_error)? else {
        writeln!(out, "No cell at {},{}.", at.0, at.1)?;
        return Ok(());
    };
    let trees = cellview::tree::placed_trees(order, &cell);
    writeln!(out, "{} trees at {},{}:", trees.len(), at.0, at.1)?;
    for t in &trees {
        let base = TreeBase::load(order, t.base);
        writeln!(
            out,
            "  {} {} at {:.1}, {:.1}, {:.1}, angles {:.1}° {:.1}° {:.1}°, scale {:.2}, XSED {}, seed {}",
            t.reference,
            base.as_ref()
                .and_then(|b| b.editor_id.as_deref())
                .unwrap_or("?"),
            t.position[0],
            t.position[1],
            t.position[2],
            t.rotation[0].to_degrees(),
            t.rotation[1].to_degrees(),
            t.rotation[2].to_degrees(),
            t.scale,
            t.xsed.map_or("none".to_string(), |x| x.to_string()),
            base.as_ref().map_or(0, |b| b.seed(t.xsed))
        )?;
    }
    Ok(())
}

//! `cells` and `render-cell`: list interiors, and draw one into PNG files.

use std::io::Write;
use std::path::{Path, PathBuf};
use std::time::Instant;

use assets::Assets;
use esm::LoadOrder;
use preview::cell::{
    build_scene_with, describe_transform, draw_plan_markers, horizontal_fov, plan_camera,
    plan_size, render, view_camera, CellScene, LightingMode, RenderOptions, DEFAULT_CUT_HEIGHT,
    EYE_HEIGHT, GAME_FOV_DEGREES,
};
use preview::raster::Camera;
use world::{find_cells, interior_cells, load_cell, RotationConvention};

use crate::fmt::{thousands, truncate};
use crate::{CliError, Options};

/// Options for `render-cell`.
#[derive(Default)]
pub struct RenderFlags {
    pub rotation: Option<RotationConvention>,
    pub variants: bool,
    pub size: Option<(usize, usize)>,
    pub plan_size: Option<usize>,
    pub fov: Option<f32>,
    pub from: Option<[f32; 3]>,
    pub heading: Option<f32>,
    pub pitch: Option<f32>,
    pub arrival: Option<usize>,
    pub cut: Option<f32>,
    pub brightness: Option<f32>,
    pub fullbright: bool,
    pub mark_tilted: bool,
    pub no_cull: bool,
    pub supersample: Option<usize>,
    pub keep_root_transforms: bool,
    /// Any of the above was given.
    pub any: bool,
}

impl RenderFlags {
    /// Handles one `render-cell` flag; returns false if `flag` isn't one.
    pub fn parse(
        &mut self,
        flag: &str,
        mut value: impl FnMut() -> Result<String, CliError>,
    ) -> Result<bool, CliError> {
        let bad = |flag: &str, value: &str, expected: &str| {
            CliError::Usage(format!("{flag} expects {expected}, got '{value}'"))
        };
        let number = |flag: &str, v: String| -> Result<f32, CliError> {
            v.trim()
                .parse::<f32>()
                .ok()
                .filter(|n| n.is_finite())
                .ok_or_else(|| bad(flag, &v, "a number"))
        };
        let count = |flag: &str, v: String| -> Result<usize, CliError> {
            v.trim()
                .parse::<usize>()
                .ok()
                .filter(|&n| n > 0)
                .ok_or_else(|| bad(flag, &v, "a whole number above 0"))
        };
        match flag {
            "--rotation" => {
                let v = value()?;
                let names: Vec<&str> = RotationConvention::ALL.iter().map(|c| c.name()).collect();
                self.rotation = Some(
                    RotationConvention::parse(&v)
                        .ok_or_else(|| bad(flag, &v, &format!("one of {}", names.join(", "))))?,
                );
            }
            "--variants" => self.variants = true,
            "--size" => {
                let v = value()?;
                let parts: Vec<&str> = v.split(['x', 'X']).collect();
                let parsed = match parts.as_slice() {
                    [w, h] => w
                        .trim()
                        .parse::<usize>()
                        .ok()
                        .zip(h.trim().parse::<usize>().ok()),
                    _ => None,
                };
                let (w, h) = parsed
                    .filter(|&(w, h)| (16..=8192).contains(&w) && (16..=8192).contains(&h))
                    .ok_or_else(|| bad(flag, &v, "a size like 1600x900"))?;
                self.size = Some((w, h));
            }
            "--plan-size" => self.plan_size = Some(count(flag, value()?)?.clamp(16, 8192)),
            "--fov" => {
                let v = value()?;
                let fov = number(flag, v.clone())?;
                if !(10.0..=170.0).contains(&fov) {
                    return Err(bad(flag, &v, "degrees between 10 and 170"));
                }
                self.fov = Some(fov);
            }
            "--from" => {
                let v = value()?;
                let parts: Vec<Option<f32>> =
                    v.split(',').map(|p| p.trim().parse::<f32>().ok()).collect();
                self.from = Some(match parts.as_slice() {
                    [Some(x), Some(y), Some(z)] => [*x, *y, *z],
                    [Some(x), Some(y)] => [*x, *y, f32::NAN],
                    _ => return Err(bad(flag, &v, "a position like 120,-340,15")),
                });
            }
            "--heading" => self.heading = Some(number(flag, value()?)?),
            "--pitch" => self.pitch = Some(number(flag, value()?)?),
            "--arrival" => self.arrival = Some(count(flag, value()?)?),
            "--cut" => self.cut = Some(number(flag, value()?)?),
            "--brightness" => {
                let v = value()?;
                let b = number(flag, v.clone())?;
                if b <= 0.0 {
                    return Err(bad(flag, &v, "a number above 0"));
                }
                self.brightness = Some(b);
            }
            "--fullbright" => self.fullbright = true,
            "--mark-tilted" => self.mark_tilted = true,
            "--no-cull" => self.no_cull = true,
            "--supersample" => self.supersample = Some(count(flag, value()?)?.min(4)),
            "--keep-root-transforms" => self.keep_root_transforms = true,
            _ => return Ok(false),
        }
        self.any = true;
        Ok(true)
    }
}

/// Lists interior cells.
pub fn cells(out: &mut impl Write, order: &LoadOrder, options: &Options) -> Result<(), CliError> {
    let cells = interior_cells(order).map_err(world_error)?;
    let mut shown = 0;
    writeln!(
        out,
        "{:<8}  {:<34}  {:<34}  {:>7}",
        "Form ID", "Editor ID", "Name", "Objects"
    )?;
    for c in &cells {
        if !options.matches(&[c.editor_id.as_deref(), c.name.as_deref()]) {
            continue;
        }
        if options.limit_reached(shown) {
            break;
        }
        shown += 1;
        let plugin_note = if order.is_single() || c.form_id.mod_index() == 0 {
            String::new()
        } else {
            format!(
                "  ({})",
                order.slot_name(c.form_id.mod_index()).unwrap_or("?")
            )
        };
        writeln!(
            out,
            "{}  {:<34}  {:<34}  {:>7}{plugin_note}",
            c.form_id,
            truncate(c.editor_id.as_deref().unwrap_or("-"), 34),
            truncate(c.name.as_deref().unwrap_or("-"), 34),
            thousands(c.references)
        )?;
    }
    writeln!(
        out,
        "{} of {} interior cells",
        thousands(shown),
        thousands(cells.len())
    )?;
    Ok(())
}

pub(crate) fn world_error(e: world::Error) -> CliError {
    match e {
        world::Error::Esm(e) => CliError::Esm(e),
        other => CliError::NotFound(other.to_string()),
    }
}

fn degrees(radians: f32) -> f32 {
    radians.to_degrees().rem_euclid(360.0)
}

fn pos(p: [f32; 3]) -> String {
    format!("({:.0}, {:.0}, {:.0})", p[0], p[1], p[2])
}

/// The one cell a query (editor ID, form ID or name) names, or an error
/// listing the candidates.
pub(crate) fn single_cell(order: &LoadOrder, query: &str) -> Result<esm::FormId, CliError> {
    let matches = find_cells(order, query).map_err(world_error)?;
    match matches.as_slice() {
        [] => Err(CliError::NotFound(format!(
            "no cell matches '{query}'. Try `cells --grep {query}` to search by name."
        ))),
        [one] => Ok(*one),
        many => {
            let mut msg = format!(
                "'{query}' matches {} cells; use an editor ID or form ID:",
                many.len()
            );
            for &id in many.iter().take(15) {
                if let Some(rr) = order.get(id) {
                    msg.push_str(&format!("\n  {}", world::describe_record(&rr)));
                }
            }
            if many.len() > 15 {
                msg.push_str(&format!("\n  ... and {} more", many.len() - 15));
            }
            Err(CliError::NotFound(msg))
        }
    }
}

/// Renders a cell to a floor plan and a first-person view.
pub fn render_cell(
    out: &mut impl Write,
    order: &LoadOrder,
    assets: &Assets,
    query: &str,
    output: Option<&str>,
    options: &Options,
) -> Result<(), CliError> {
    let flags = &options.render;
    let started = Instant::now();
    let cell_id = single_cell(order, query)?;

    let loaded = load_cell(order, cell_id).map_err(world_error)?;
    describe_cell(out, &loaded)?;
    let scene = build_scene_with(assets, loaded, flags.keep_root_transforms);
    describe_scene(out, &scene)?;
    describe_root_transforms(out, &scene, flags.keep_root_transforms)?;
    out.flush()?;

    let conventions: Vec<RotationConvention> = if flags.variants {
        RotationConvention::ALL[..2].to_vec()
    } else {
        vec![flags.rotation.unwrap_or_default()]
    };
    let base_convention = conventions[0];

    // Floors and the ceiling cut.
    let floors = scene.floor_levels(base_convention);
    let floor = floors.first().map(|f| f.0);
    if let Some(&(_, largest)) = floors.first() {
        let list: Vec<String> = floors
            .iter()
            .filter(|(_, area)| *area >= largest * 0.02)
            .take(4)
            .map(|(z, area)| format!("{z:.0} ({} sq units)", thousands(*area as usize)))
            .collect();
        writeln!(out, "Floor heights:  {}", list.join(", "))?;
    }
    let cut = flags
        .cut
        .unwrap_or_else(|| floor.unwrap_or(0.0) + DEFAULT_CUT_HEIGHT);
    let cut_note = match (flags.cut, floor) {
        (Some(_), _) => "(from --cut)".to_string(),
        (None, Some(_)) => {
            format!("({DEFAULT_CUT_HEIGHT:.0} above the main floor; change with --cut)")
        }
        (None, None) => "(no floor found; change with --cut)".to_string(),
    };
    writeln!(
        out,
        "Plan cut:       everything above {cut:.0} removed {cut_note}"
    )?;

    let bounds = scene.plan_bounds(base_convention, cut);
    let Some((min, max)) = bounds else {
        writeln!(
            out,
            "Nothing to draw: no object below the cut has a model that loaded."
        )?;
        return Ok(());
    };

    // The first-person camera.
    let (width, height) = flags.size.unwrap_or((1600, 900));
    let fov = horizontal_fov(
        flags.fov.unwrap_or(GAME_FOV_DEGREES),
        width as f32 / height as f32,
    );
    let arrivals = &scene.cell.arrivals;
    if !arrivals.is_empty() {
        writeln!(out, "Arrival points:")?;
        for (i, a) in arrivals.iter().enumerate() {
            writeln!(
                out,
                "  {}. {} at {}, facing {:.0}°",
                i + 1,
                a.via,
                pos(a.position),
                degrees(a.rotation[2])
            )?;
        }
    }
    let arrival = flags.arrival.unwrap_or(1) - 1;
    if flags.arrival.is_some() && arrival >= arrivals.len() {
        return Err(CliError::Usage(format!(
            "--arrival {} asked for, but the cell has {} arrival point(s)",
            arrival + 1,
            arrivals.len()
        )));
    }
    let center = [(min[0] + max[0]) * 0.5, (min[1] + max[1]) * 0.5];
    let (mut camera, mut camera_note) =
        view_camera(&scene, arrival, (center, floor.unwrap_or(0.0)), fov);
    if flags.from.is_some() || flags.heading.is_some() || flags.pitch.is_some() {
        let eye = match flags.from {
            Some([x, y, z]) => {
                let feet = if z.is_nan() { floor.unwrap_or(0.0) } else { z };
                [x, y, feet + EYE_HEIGHT]
            }
            None => camera.eye,
        };
        let current = camera.forward[0].atan2(camera.forward[1]);
        let heading = flags.heading.map_or(current, f32::to_radians);
        let pitch = flags.pitch.unwrap_or(0.0).to_radians();
        camera = Camera::first_person(eye, heading, pitch, fov);
        camera_note = "from the options given".into();
    }
    let feet = [camera.eye[0], camera.eye[1], camera.eye[2] - EYE_HEIGHT];
    writeln!(
        out,
        "View camera:    {camera_note}; standing at {}, facing {:.0}°, eye {EYE_HEIGHT:.0} units up, {:.0}° field of view",
        pos(feet),
        degrees(camera.forward[0].atan2(camera.forward[1])),
        fov.to_degrees()
    )?;

    let tilted: Vec<_> = scene
        .instances
        .iter()
        .filter(|i| scene.is_tilted(i))
        .map(|i| i.object)
        .collect::<std::collections::BTreeSet<_>>()
        .into_iter()
        .collect();
    if tilted.is_empty() {
        writeln!(
            out,
            "Tilted objects: none (rotation order only matters for objects turned on more than one axis)"
        )?;
    } else {
        writeln!(
            out,
            "Tilted objects: {} turned on more than one axis, where the rotation order matters{}:",
            tilted.len(),
            if flags.mark_tilted {
                " (tinted pink)"
            } else {
                "; --mark-tilted tints them pink"
            }
        )?;
        for &i in tilted.iter().take(12) {
            let o = &scene.cell.objects[i];
            let r = o.rotation.map(degrees);
            writeln!(
                out,
                "  {} {:<28} at {}, angles {:.0}°, {:.0}°, {:.0}°",
                o.form_id,
                truncate(o.base_editor_id.as_deref().unwrap_or("?"), 28),
                pos(o.position),
                r[0],
                r[1],
                r[2]
            )?;
        }
        if tilted.len() > 12 {
            writeln!(out, "  ... and {} more", tilted.len() - 12)?;
        }
    }
    out.flush()?;

    // Render.
    let prefix = output_prefix(output, &scene);
    let (plan_w, plan_h) = plan_size(min, max, flags.plan_size.unwrap_or(1600));
    let plan_cam = plan_camera(min, max, cut + 1000.0, plan_w, plan_h);
    let base_options = RenderOptions {
        convention: base_convention,
        lighting: if flags.fullbright {
            LightingMode::Neutral
        } else {
            LightingMode::Cell {
                brightness: flags.brightness.unwrap_or(1.0),
            }
        },
        supersample: flags.supersample.unwrap_or(2),
        cull: !flags.no_cull,
        mark_tilted: flags.mark_tilted,
        effects: true,
    };
    let mut written = Vec::new();
    for convention in &conventions {
        let suffix = if flags.variants {
            format!("_{}", convention.name())
        } else {
            String::new()
        };
        let plan_options = RenderOptions {
            convention: *convention,
            lighting: LightingMode::Neutral,
            effects: false,
            ..base_options
        };
        let plan = render(
            &scene,
            plan_cam,
            plan_w,
            plan_h,
            &plan_options,
            Some(cut),
            |r, ss| draw_plan_markers(r, ss, &scene, Some(&camera)),
        );
        written.push(save(&prefix, &format!("{suffix}_plan"), &plan)?);

        let view_options = RenderOptions {
            convention: *convention,
            ..base_options
        };
        let view = render(
            &scene,
            camera,
            width,
            height,
            &view_options,
            None,
            |_, _| {},
        );
        written.push(save(&prefix, &format!("{suffix}_view"), &view)?);
    }

    let list_path = write_object_list(
        &prefix,
        &scene,
        flags.keep_root_transforms,
        base_convention,
        &plan_cam,
        (plan_w, plan_h),
    )?;

    for (path, w, h) in &written {
        writeln!(out, "Wrote {} ({w}x{h})", path.display())?;
    }
    if let preview::raster::Projection::Orthographic { half_width } = plan_cam.projection {
        let units_per_pixel = 2.0 * half_width / plan_w as f32;
        let (x0, x1) = (plan_cam.eye[0] - half_width, plan_cam.eye[0] + half_width);
        let half_height = half_width * plan_h as f32 / plan_w as f32;
        let (y0, y1) = (plan_cam.eye[1] - half_height, plan_cam.eye[1] + half_height);
        writeln!(
            out,
            "Plan area:      x {x0:.0} to {x1:.0}, y {y0:.0} to {y1:.0}; 1 pixel = {units_per_pixel:.2} units"
        )?;
    }
    writeln!(
        out,
        "Object list:    {} (every object, where it lands on the plan, and what was left out)",
        list_path.display()
    )?;
    writeln!(
        out,
        "Plan: north is up; white lines are where walls and objects cross the cut; \
         yellow = arrival points, cyan = the view camera, orange = lights, \
         pink dots = people and creatures; the bar at the bottom left is 256 units (about 3.7 m)."
    )?;
    if flags.variants {
        writeln!(
            out,
            "Variants: xyz matches the game; zyx is there for comparison."
        )?;
    }
    writeln!(out, "Rendered in {:.1} s", started.elapsed().as_secs_f32())?;
    if let Some(editor_id) = &scene.cell.info.editor_id {
        writeln!(
            out,
            "To compare in game: open the console (~), type `coc {editor_id}`, then `tm` to hide the HUD. \
             `player.getpos x` / `y` / `z` and `player.getangle z` give a spot to pass to --from and --heading."
        )?;
    }
    Ok(())
}

fn describe_cell(out: &mut impl Write, cell: &world::LoadedCell) -> Result<(), CliError> {
    let info = &cell.info;
    let name = match (&info.editor_id, &info.name) {
        (Some(e), Some(n)) => format!("{e} \"{n}\""),
        (Some(e), None) => e.clone(),
        (None, Some(n)) => format!("\"{n}\""),
        (None, None) => "(no editor ID)".into(),
    };
    let kind = if info.interior {
        "interior".to_string()
    } else {
        match info.grid {
            Some((x, y)) => format!("exterior {x},{y} (terrain isn't drawn yet)"),
            None => "exterior (terrain isn't drawn yet)".into(),
        }
    };
    writeln!(
        out,
        "Cell:           {name}  {}  {kind}, from {}",
        info.form_id, info.plugin
    )?;
    match &info.lighting {
        Some(l) => writeln!(
            out,
            "Lighting:       ambient {}, directional {}, fog {} ({})",
            rgb(l.ambient),
            rgb(l.directional),
            rgb(l.fog_color),
            info.lighting_source
        )?,
        None => writeln!(out, "Lighting:       {}", info.lighting_source)?,
    }
    match &info.image_space {
        Some(space) => {
            let name = space
                .editor_id
                .clone()
                .unwrap_or_else(|| space.form_id.to_string());
            match space.cinematic() {
                Some(c) => writeln!(
                    out,
                    "Image space:    {name}: saturation {:.2}, tint {:.2},{:.2},{:.2} at {:.2}, \
                     brightness {:.2}, contrast {:.2} around {:.2} (not applied to these pictures)",
                    c.saturation,
                    c.tint[0],
                    c.tint[1],
                    c.tint[2],
                    c.tint_amount,
                    c.brightness,
                    c.contrast,
                    c.contrast_average
                )?,
                None => writeln!(
                    out,
                    "Image space:    {name}: values not understood ({} bytes)",
                    space.size
                )?,
            }
        }
        None => writeln!(out, "Image space:    none named")?,
    }
    let lights = cell.lights().count();
    let lights_on = cell
        .lights()
        .filter(|(_, l)| !l.is_off_by_default() && !l.is_negative())
        .count();
    writeln!(
        out,
        "Placed:         {} objects ({lights} lights, {lights_on} on), {} markers, {} people and creatures",
        thousands(cell.objects.len()),
        thousands(cell.markers.len()),
        thousands(cell.actors.len())
    )?;
    if !cell.skipped.is_empty() {
        let parts: Vec<String> = cell
            .skipped
            .iter()
            .map(|(reason, n)| format!("{n} {reason}"))
            .collect();
        writeln!(out, "Left out:       {}", parts.join(", "))?;
    }
    Ok(())
}

fn describe_scene(out: &mut impl Write, scene: &CellScene) -> Result<(), CliError> {
    let r = &scene.report;
    writeln!(
        out,
        "Loaded:         {} models, {} textures, {} triangles",
        thousands(scene.models.len()),
        thousands(scene.textures.len()),
        thousands(scene.triangle_count())
    )?;
    if r.collections_from_parts > 0 {
        writeln!(
            out,
            "                {} static collections drawn piece by piece (combined model missing)",
            r.collections_from_parts
        )?;
    }
    if !scene.cell.actors.is_empty() {
        writeln!(
            out,
            "People:         {} of {} drawn, posed by their idle's first frame",
            r.actors_drawn,
            scene.cell.actors.len()
        )?;
        for path in &r.missing_animations {
            writeln!(out, "                missing animation: {path}")?;
        }
    }
    if r.renamed_textures > 0 {
        writeln!(
            out,
            "                {} textures found after correcting a wrong file extension",
            r.renamed_textures
        )?;
    }
    if r.skinned_meshes > 0 {
        writeln!(
            out,
            "                {} skinned meshes drawn in their bind pose",
            r.skinned_meshes
        )?;
    }
    let lists: [(&str, Vec<String>); 4] = [
        (
            "Missing models",
            r.missing_models
                .iter()
                .map(|(p, n)| {
                    if *n > 1 {
                        format!("{p} (x{n})")
                    } else {
                        p.clone()
                    }
                })
                .collect(),
        ),
        (
            "Unreadable models",
            r.unreadable_models
                .iter()
                .map(|(p, e)| format!("{p}: {e}"))
                .collect(),
        ),
        (
            "Missing textures (drawn grey)",
            r.missing_textures.keys().cloned().collect(),
        ),
        (
            "Unreadable textures",
            r.unreadable_textures
                .iter()
                .map(|(p, e)| format!("{p}: {e}"))
                .collect(),
        ),
    ];
    for (title, items) in lists {
        if items.is_empty() {
            continue;
        }
        writeln!(out, "{title}: {}", items.len())?;
        for item in items.iter().take(10) {
            writeln!(out, "  {item}")?;
        }
        if items.len() > 10 {
            writeln!(out, "  ... and {} more", items.len() - 10)?;
        }
    }
    Ok(())
}

/// Writes `<prefix>_objects.txt`: every placed object with its model,
/// placement, world-space box and position on the plan image, then the
/// markers, people and references left out.
fn write_object_list(
    prefix: &Path,
    scene: &CellScene,
    keep_root: bool,
    convention: RotationConvention,
    plan: &Camera,
    (plan_w, plan_h): (usize, usize),
) -> Result<PathBuf, CliError> {
    use std::fmt::Write as _;
    let cell = &scene.cell;
    let bounds = scene.object_bounds(convention);
    let pixel = |p: [f32; 3]| {
        plan.project(p, plan_w, plan_h)
            .map_or("-".to_string(), |(x, y)| format!("{x:.0},{y:.0}"))
    };
    let angles = |r: [f32; 3]| {
        let d = r.map(degrees);
        format!("{:.0},{:.0},{:.0}", d[0], d[1], d[2])
    };
    let xyz = |p: [f32; 3]| format!("{:.0},{:.0},{:.0}", p[0], p[1], p[2]);
    let name = |id: Option<&str>| id.unwrap_or("-").to_string();

    let mut text = String::new();
    let info = &cell.info;
    let _ = writeln!(
        text,
        "Objects in {} ({}), rotation convention {}.",
        info.label(),
        info.form_id,
        convention.name()
    );
    let _ = writeln!(
        text,
        "\"plan\" is the object's position in pixels on the plan image (from the top left);"
    );
    let _ = writeln!(
        text,
        "\"box\" is the world-space box around what's drawn, and its corners on the plan.\n"
    );

    let _ = writeln!(text, "DRAWN ({})", cell.objects.len());
    for (i, o) in cell.objects.iter().enumerate() {
        let mut line = format!(
            "{}  {}  {}  {}  pos {}  rot {}  scale {:.2}  plan {}",
            o.form_id,
            o.base_type,
            name(o.base_editor_id.as_deref()),
            o.model.as_deref().unwrap_or("(no model)"),
            xyz(o.position),
            angles(o.rotation),
            o.scale,
            pixel(o.position)
        );
        match bounds[i] {
            Some((lo, hi, effect)) => {
                let _ = write!(
                    line,
                    "  box x {:.0}..{:.0} y {:.0}..{:.0} z {:.0}..{:.0} (plan {} to {})",
                    lo[0],
                    hi[0],
                    lo[1],
                    hi[1],
                    lo[2],
                    hi[2],
                    pixel([lo[0], hi[1], 0.0]),
                    pixel([hi[0], lo[1], 0.0])
                );
                if effect {
                    line.push_str("  [light effect]");
                }
            }
            None if o.light.is_some() => {}
            None => line.push_str("  [model not drawn]"),
        }
        if let Some(e) = &o.editor_id {
            let _ = write!(line, "  ref {e}");
        }
        if world::is_tilted(o.rotation) {
            line.push_str("  [tilted]");
        }
        if o.flags & esm::flags::INITIALLY_DISABLED != 0 {
            line.push_str("  [flagged initially disabled]");
        }
        if let Some((parent, opposite)) = o.enable_parent {
            let _ = write!(
                line,
                "  [enable parent {parent}{}]",
                if opposite { ", opposite" } else { "" }
            );
        }
        if let (Some(l), Some(used)) = (&o.light, scene.cell.placed_light(o)) {
            let byte = |c: f32| (c * 255.0).round() as u8;
            let _ = write!(
                line,
                "  [light radius {:.0} color {} fade {:.2}{}; base {} + reference {:.0}, color {}{}]",
                used.radius,
                rgb(used.color.map(byte)),
                used.fade,
                if l.is_off_by_default() { " off" } else { "" },
                l.radius,
                o.radius.unwrap_or(0.0),
                rgb(l.color),
                match o.emittance.and_then(|e| scene.cell.color_of(e)) {
                    Some(e) => format!(" tinted by emittance {}", rgb(e.map(byte))),
                    None => String::new(),
                }
            );
        }
        if let Some(t) = &o.teleport {
            let _ = write!(line, "  [door to {}]", t.door);
        }
        let (missing, untextured) = texture_problems(scene, i);
        for path in &missing {
            let _ = write!(line, "  [texture missing: {path}]");
        }
        if untextured > 0 {
            let _ = write!(
                line,
                "  [{untextured} piece{} with no texture]",
                if untextured == 1 { "" } else { "s" }
            );
        }
        if let Some(t) = root_transform_of(scene, i) {
            let _ = write!(
                line,
                "  [model's top node {}: {}]",
                describe_transform(&t),
                if keep_root {
                    "applied"
                } else {
                    "ignored, as in game"
                }
            );
        }
        if !o.parts.is_empty() {
            let _ = write!(line, "  [{} collection pieces]", o.parts.len());
        }
        if !o.plugin.eq_ignore_ascii_case("FalloutNV.esm") {
            let _ = write!(line, "  [from {}]", o.plugin);
        }
        let _ = writeln!(text, "{line}");
    }
    for (title, list) in [
        ("MARKERS", &cell.markers),
        ("PEOPLE AND CREATURES", &cell.actors),
    ] {
        let _ = writeln!(text, "\n{title} ({})", list.len());
        for o in list {
            let _ = writeln!(
                text,
                "{}  {}  {}  pos {}  rot {}  plan {}",
                o.form_id,
                o.base_type,
                name(o.base_editor_id.as_deref()),
                xyz(o.position),
                angles(o.rotation),
                pixel(o.position)
            );
        }
    }
    let _ = writeln!(text, "\nLEFT OUT ({})", cell.left_out.len());
    for l in &cell.left_out {
        let _ = writeln!(
            text,
            "{}  {}  base {}  {}  {}",
            l.form_id,
            l.record_type,
            l.base.map_or("-".to_string(), |b| b.to_string()),
            name(l.base_editor_id.as_deref()),
            l.reason
        );
    }

    let mut path = prefix.as_os_str().to_owned();
    path.push("_objects.txt");
    let path = PathBuf::from(path);
    if let Some(parent) = path.parent().filter(|p| !p.as_os_str().is_empty()) {
        std::fs::create_dir_all(parent)?;
    }
    std::fs::write(&path, text).map_err(|e| CliError::Open {
        path: path.display().to_string(),
        message: e.to_string(),
    })?;
    Ok(path)
}

/// Diffuse textures an object's models name but that couldn't be loaded,
/// and how many of its pieces (light effects aside) have no texture.
fn texture_problems(scene: &CellScene, object: usize) -> (Vec<String>, usize) {
    let mut missing = Vec::new();
    let mut untextured = 0;
    for instance in scene.instances.iter().filter(|i| i.object == object) {
        for mesh in &scene.models[instance.model].meshes {
            match (&mesh.texture_path, mesh.texture) {
                (Some(path), None) if !missing.contains(path) => missing.push(path.clone()),
                (None, _) if !mesh.effect => untextured += 1,
                _ => {}
            }
        }
    }
    (missing, untextured)
}

/// The top-node transform of the model drawn for an object, if it has one.
fn root_transform_of(scene: &CellScene, object: usize) -> Option<nif::math::Transform> {
    scene
        .instances
        .iter()
        .find(|i| i.object == object && i.part.is_none())
        .and_then(|i| scene.models[i.model].root_transform)
}

/// Lists the models whose top node carries a transform, which the game
/// ignores when placing them.
fn describe_root_transforms(
    out: &mut impl Write,
    scene: &CellScene,
    keep: bool,
) -> Result<(), CliError> {
    let mut models: Vec<(&str, String, usize)> = Vec::new();
    for (index, model) in scene.models.iter().enumerate() {
        if let Some(t) = &model.root_transform {
            let uses = scene.instances.iter().filter(|i| i.model == index).count();
            models.push((&model.path, describe_transform(t), uses));
        }
    }
    if models.is_empty() {
        return Ok(());
    }
    models.sort();
    let how = if keep {
        "applied here because of --keep-root-transforms"
    } else {
        "the game replaces it with the placement, and so does this render \
         (--keep-root-transforms applies it instead)"
    };
    writeln!(
        out,
        "Model top nodes: {} models store a transform on their top node; {how}:",
        models.len()
    )?;
    for (path, what, uses) in models.iter().take(12) {
        let times = if *uses == 1 {
            String::new()
        } else {
            format!(" (placed {uses} times)")
        };
        writeln!(out, "  {path}: {what}{times}")?;
    }
    if models.len() > 12 {
        writeln!(
            out,
            "  ... and {} more (see the object list)",
            models.len() - 12
        )?;
    }
    Ok(())
}

fn rgb(c: [u8; 3]) -> String {
    format!("{},{},{}", c[0], c[1], c[2])
}

/// `OUT` as given (without a .png extension), else the cell's editor ID.
fn output_prefix(output: Option<&str>, scene: &CellScene) -> PathBuf {
    if let Some(out) = output {
        let trimmed = out
            .strip_suffix(".png")
            .or_else(|| out.strip_suffix(".PNG"))
            .unwrap_or(out);
        return PathBuf::from(trimmed);
    }
    let info = &scene.cell.info;
    let name = info
        .editor_id
        .clone()
        .unwrap_or_else(|| format!("cell_{}", info.form_id));
    let safe: String = name
        .chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() || c == '_' || c == '-' {
                c
            } else {
                '_'
            }
        })
        .collect();
    PathBuf::from(safe)
}

fn save(
    prefix: &Path,
    suffix: &str,
    image: &preview::cell::Rendered,
) -> Result<(PathBuf, usize, usize), CliError> {
    let mut name = prefix.as_os_str().to_owned();
    name.push(format!("{suffix}.png"));
    let path = PathBuf::from(name);
    if let Some(parent) = path.parent().filter(|p| !p.as_os_str().is_empty()) {
        std::fs::create_dir_all(parent).map_err(|e| CliError::Open {
            path: parent.display().to_string(),
            message: e.to_string(),
        })?;
    }
    let file = std::fs::File::create(&path).map_err(|e| CliError::Open {
        path: path.display().to_string(),
        message: e.to_string(),
    })?;
    let mut w = std::io::BufWriter::new(file);
    dds::png::write_rgba(&mut w, image.width as u32, image.height as u32, &image.rgba)?;
    w.flush()?;
    Ok((path, image.width, image.height))
}

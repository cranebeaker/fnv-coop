//! `menu` and `font`: the game's menus as its code works them out (the
//! `ui` crate), and its fonts' glyph tables.

use std::collections::HashMap;
use std::io::Write;
use std::path::Path;

use assets::{Assets, IniSettings};
use esm::LoadOrder;
use ui::draw::{DrawKind, Textures};
use ui::names::{kind, t};
use ui::{Atlas, Font, TileId, Ui};

use crate::CliError;

/// Every text game setting (`GMST` named `s…`), for `&-sName;`.
pub fn text_settings(order: &LoadOrder) -> HashMap<String, String> {
    let mut out = HashMap::new();
    for rr in order.records_of_type(esm::FourCC::new(b"GMST")) {
        let Ok(record) = rr.record() else {
            continue;
        };
        let Some(name) = record.get(esm::sig::EDID).map(|s| s.zstring()) else {
            continue;
        };
        if name.starts_with('s') {
            if let Some(text) = record.get(esm::sig::DATA).map(|s| s.zstring()) {
                out.insert(name, text);
            }
        }
    }
    out
}

/// Texture sizes and atlases, read from the game's files.
struct GameTextures<'a> {
    assets: &'a Assets,
    sizes: HashMap<String, Option<(u32, u32)>>,
}

impl Textures for GameTextures<'_> {
    fn size(&mut self, path: &str) -> Option<(u32, u32)> {
        if let Some(s) = self.sizes.get(path) {
            return *s;
        }
        let size = self
            .assets
            .read(path)
            .ok()
            .flatten()
            .and_then(|b| dds::Dds::parse(b).ok())
            .map(|d| (d.width(), d.height()));
        self.sizes.insert(path.to_string(), size);
        size
    }

    fn atlas(&mut self, path: &str) -> Option<Atlas> {
        let bytes = self.assets.read(path).ok().flatten()?;
        Some(Atlas::parse(&String::from_utf8_lossy(&bytes)))
    }
}

fn kind_name(k: i32) -> &'static str {
    match k {
        kind::RECT => "rect",
        kind::IMAGE => "image",
        kind::TEXT => "text",
        kind::NIF => "nif",
        kind::MENU => "menu",
        kind::HOTRECT => "hotrect",
        kind::WINDOW => "window",
        kind::RADIAL => "radial",
        _ => "?",
    }
}

/// The screen size the game draws at: `[Display] iSize W` / `iSize H`.
fn screen_size(ini: &IniSettings) -> (u32, u32) {
    let get = |k: &str| {
        ini.get("Display", k)
            .and_then(|v| v.trim().parse::<u32>().ok())
    };
    (
        get("iSize W").unwrap_or(1920),
        get("iSize H").unwrap_or(1080),
    )
}

/// `menu <PATH>`: a menu file worked out, tile by tile: where each one is
/// on the screen (in menu units), its size, whether it's shown, its
/// alpha, depth and colour, its text or picture; then what's drawn. The
/// HUD (`menus\main\hud_main_menu.xml`) is set up and updated once the way
/// the game's code does it, with full health and action points.
pub fn menu(
    out: &mut impl Write,
    order: &LoadOrder,
    assets: &Assets,
    data_dir: &Path,
    path: &str,
) -> Result<(), CliError> {
    let ini = IniSettings::load(&assets::default_settings_files(data_dir));
    let (w, h) = screen_size(&ini);
    let mut read = |p: &str| assets.read(p).ok().flatten();
    let ini_get = |s: &str, k: &str| ini.get(s, k).map(str::to_string);
    let mut settings = text_settings(order);
    // The exe's own text for what V.A.T.S.'s menu names and the data leaves
    // to it.
    for (name, text) in ui::vats::EXE_TEXT {
        settings
            .entry((*name).to_string())
            .or_insert_with(|| (*text).to_string());
    }
    let mut ui = ui::game::new_ui(&mut read, &ini_get, settings, w, h);
    let lower = path.trim().replace('/', "\\").to_ascii_lowercase();
    let menu = if let Some(spec) = path.strip_prefix("message:") {
        // `message:TEXT|BUTTON|BUTTON...`: a message box filled in by the
        // message menu's code (`ui::menus::message`).
        // A title first when it starts with `^`: `message:^TITLE|TEXT|...`.
        let mut parts = spec.split('|').peekable();
        let title = parts
            .peek()
            .and_then(|p| p.strip_prefix('^'))
            .map(str::to_string);
        if title.is_some() {
            parts.next();
        }
        let text = parts.next().unwrap_or_default().to_string();
        let buttons: Vec<Option<String>> = parts.map(|b| Some(b.to_string())).collect();
        let alpha = ui
            .globals
            .zip(ui.names.lookup("_background_fill_alpha"))
            .map(|(g, id)| ui.number(g, id))
            .unwrap_or(204.0);
        let mut code = ui::menus::message::MessageMenu::new(0);
        let tile = ui::menu::load(&mut ui, &mut read, ui::menus::message::FILE, &mut code, 2.0)
            .map_err(CliError::NotFound)?;
        code.menu = tile;
        code.push(ui::menus::message::MessageBox::script(
            &text,
            title.as_deref(),
            &buttons,
            alpha,
        ));
        code.open(&mut ui, None);
        tile
    } else if let Some(spec) = path.strip_prefix("dialog:") {
        // `dialog:NAME|LINE` shows a line; `dialog:NAME|>TOPIC|>TOPIC...`
        // the topics (a topic starting `-` dimmed, `!` a failed check).
        let mut parts = spec.split('|');
        let name = parts.next().unwrap_or_default().to_string();
        let rest: Vec<&str> = parts.collect();
        let mut code = ui::menus::dialog::DialogMenu::new(0);
        let tile = ui::menu::load(&mut ui, &mut read, ui::menus::dialog::FILE, &mut code, 2.0)
            .map_err(CliError::NotFound)?;
        code.menu = tile;
        code.open(&mut ui, &name, true, false);
        // Two seconds of frames: the camera's zoom brings the menu in.
        for _ in 0..120 {
            code.update(&mut ui, 1.0 / 60.0, 1.5, 0.5);
        }
        if rest.iter().all(|r| r.starts_with('>')) && !rest.is_empty() {
            let topics: Vec<ui::menus::dialog::Topic> = rest
                .iter()
                .map(|r| {
                    let r = &r[1..];
                    ui::menus::dialog::Topic {
                        text: r.trim_start_matches(['-', '!']).to_string(),
                        dim: r.starts_with('-'),
                        failed: r.starts_with('!'),
                    }
                })
                .collect();
            code.show_topics(&mut ui, &topics, 0.0);
        } else {
            code.show_line(&mut ui, rest.first().copied().unwrap_or_default());
        }
        tile
    } else if lower.ends_with(&ui::hud::MENU_FILE[6..]) || lower == "hud" {
        let mut hud = ui::hud::load(&mut ui, &mut read).map_err(CliError::NotFound)?;
        hud.update(
            &mut ui,
            &ui::HudInput {
                health: 100.0,
                health_max: 100.0,
                action_points: 80.0,
                action_points_max: 80.0,
                opacity: 1.0,
                crosshair: true,
                ..Default::default()
            },
        );
        hud.menu
    } else if lower.ends_with("vats_menu.xml") || lower == "vats" {
        // V.A.T.S.'s menu set up and updated once as the game's code does,
        // on a made-up target with a torso and a head, one attack queued.
        let mut v = ui::vats::load(&mut ui, &mut read).map_err(CliError::NotFound)?;
        let part = |part_type: i32, group: i32, name: &str, at: [f32; 2]| ui::vats::PartView {
            part_type,
            group,
            name: name.into(),
            percent: "52%".into(),
            meter: 1.0,
            screen: Some(at),
        };
        let mut input = ui::vats::VatsInput {
            time: 0.0,
            mode: 2,
            settled: true,
            part_selection: true,
            health: 200.0,
            health_max: 200.0,
            action_points: 80.0,
            action_points_max: 80.0,
            cost: 17.0,
            target: Some(ui::vats::TargetView {
                key: 1,
                name: "Target".into(),
                is_actor: true,
                alive: true,
                hostile: true,
                health: 1.0,
                health_after: 0.7,
                parts: vec![
                    part(0, 26, "Torso", [0.5, 0.5]),
                    part(1, 25, "Head", [0.5, 0.7]),
                ],
                selected: 1,
            }),
            queue: vec!["Target: Head".into()],
            ..Default::default()
        };
        input.compass.opacity = 255.0;
        v.update(&mut ui, &input);
        input.time = 0.5;
        v.update(&mut ui, &input);
        v.menu
    } else {
        let file = if lower.starts_with("menus\\") {
            lower.clone()
        } else {
            format!("menus\\{lower}")
        };
        let text = read(&file).ok_or_else(|| CliError::NotFound(format!("{file} not found")))?;
        let menu = ui
            .load_menu(&text, &mut read)
            .map_err(|e| CliError::InFile {
                path: file.clone(),
                message: e,
            })?;
        ui.set_number(menu, t::VISIBLE, 1.0);
        menu
    };
    let screen = ui.screen_size;
    writeln!(
        out,
        "Screen: {} x {} pixels = {:.2} x {:.2} menu units ({:.4} pixels a unit), safe zone {} x {}",
        screen.width_px,
        screen.height_px,
        screen.width(),
        screen.height(),
        1.0 / screen.resolution_converter(),
        screen.safe_x,
        screen.safe_y
    )?;
    let fonts: Vec<String> = ui
        .fonts
        .iter()
        .enumerate()
        .map(|(i, f)| {
            format!(
                "{}: {}",
                i + 1,
                if f.is_some() { "loaded" } else { "missing" }
            )
        })
        .collect();
    writeln!(out, "Fonts: {}", fonts.join(", "))?;
    for w in &ui.warnings {
        writeln!(out, "warning: {w}")?;
    }
    writeln!(out)?;
    print_tile(out, &mut ui, menu, 0)?;
    let mut textures = GameTextures {
        assets,
        sizes: HashMap::new(),
    };
    let items = ui::draw_list(&mut ui, menu, &mut textures, &|_| None);
    writeln!(out, "\nDrawn, back to front ({}):", items.len())?;
    for item in &items {
        let name = &ui.tiles[item.tile].name;
        let c = item.color;
        let color = format!("tint {:.3} {:.3} {:.3} alpha {:.3}", c[0], c[1], c[2], c[3]);
        match &item.kind {
            DrawKind::Image {
                texture, rect, uv, ..
            } => writeln!(
                out,
                "  {name}: {texture} at {:.2}, {:.2} size {:.2} x {:.2} uv {:.5} {:.5} .. {:.5} {:.5}, depth {}, {color}",
                rect[0], rect[1], rect[2], rect[3], uv[0], uv[1], uv[2], uv[3], item.depth
            )?,
            DrawKind::Model { triangles, .. } => writeln!(out, "model {} triangles depth {}", triangles.len(), item.depth)?,
            DrawKind::Text { font, glyphs } => {
                let left = glyphs.iter().map(|g| g.0[0]).fold(f32::INFINITY, f32::min);
                let top = glyphs.iter().map(|g| g.0[1]).fold(f32::INFINITY, f32::min);
                writeln!(
                    out,
                    "  {name}: text, font {font}, {} glyphs from {left:.2}, {top:.2}, depth {}, {color}",
                    glyphs.len(),
                    item.depth
                )?
            }
        }
    }
    Ok(())
}

fn print_tile(
    out: &mut impl Write,
    ui: &mut Ui,
    tile: TileId,
    depth: usize,
) -> Result<(), CliError> {
    let k = ui.tiles[tile].kind;
    let (x, y) = ui.screen_position(tile);
    let w = ui.number(tile, t::WIDTH);
    let h = ui.number(tile, t::HEIGHT);
    let shown = ui.shown(tile);
    let mut line = format!(
        "{:indent$}{} \"{}\" at {:.2}, {:.2} size {:.2} x {:.2}",
        "",
        kind_name(k),
        ui.tiles[tile].name,
        x,
        y,
        w,
        h,
        indent = depth * 2
    );
    if !shown {
        line.push_str(" (hidden)");
    }
    if ui.has(tile, t::ALPHA) {
        line.push_str(&format!(" alpha {}", ui.number(tile, t::ALPHA)));
    }
    if ui.has(tile, t::DEPTH) {
        line.push_str(&format!(" depth {}", ui.number(tile, t::DEPTH)));
    }
    if ui.has(tile, t::SYSTEMCOLOR) {
        line.push_str(&format!(" colour {}", ui.number(tile, t::SYSTEMCOLOR)));
    }
    if ui.has(tile, t::BRIGHTNESS) {
        line.push_str(&format!(" brightness {}", ui.number(tile, t::BRIGHTNESS)));
    }
    if k == kind::TEXT {
        let s = ui.string(tile, t::STRING).unwrap_or_default();
        line.push_str(&format!(
            " font {} \"{}\"",
            ui.number(tile, t::FONT),
            s.replace('\n', "\\n")
        ));
    }
    if let Some(f) = ui
        .string(tile, t::FILENAME)
        .filter(|f| !f.trim().is_empty())
    {
        line.push_str(&format!(" [{}]", f.trim()));
    }
    writeln!(out, "{line}")?;
    let children = ui.tiles[tile].children.clone();
    for c in children {
        print_tile(out, ui, c, depth + 1)?;
    }
    Ok(())
}

/// `pipboy <stats|items|data> [PAGE] [ITEM...]`: the Pip-Boy's menu for a
/// new character (`ui::pipboy`): what the game's state gives it, then the
/// menu worked out tile by tile and what it draws, on the stats page or the
/// items/data tab given (from 0). Items named after the page are given to
/// the character first (`WeapNV9mmPistol`, `Stimpak:5`; a `=` after the
/// name equips it: `OutfitPrewarSpring01=`).
pub fn pipboy(
    out: &mut impl Write,
    order: &LoadOrder,
    assets: &Assets,
    data_dir: &Path,
    which: &str,
    page: Option<usize>,
    give: &[String],
) -> Result<(), CliError> {
    use ui::pipboy::{gather, Pipboy, Section};
    let ini = IniSettings::load(&assets::default_settings_files(data_dir));
    let (w, h) = screen_size(&ini);
    let mut read = |p: &str| assets.read(p).ok().flatten();
    let ini_get = |s: &str, k: &str| ini.get(s, k).map(str::to_string);
    let mut ui = ui::game::new_ui(&mut read, &ini_get, text_settings(order), w, h);
    let mut pipboy = Pipboy::load(&mut ui, &mut read).map_err(CliError::NotFound)?;
    let section = match which.to_ascii_lowercase().as_str() {
        "stats" => Section::Stats,
        "items" => Section::Items,
        "data" => Section::Data,
        other => {
            return Err(CliError::Usage(format!(
                "pipboy: '{other}' isn't stats, items or data"
            )))
        }
    };
    // A new character, in the Mojave's world map, with the items asked for.
    let mut state = world::scripting::GameState::new(order);
    let player = world::dialogue::PLAYER_REF;
    for arg in give {
        let (name, equip) = match arg.strip_suffix('=') {
            Some(n) => (n, true),
            None => (arg.as_str(), false),
        };
        let (name, count) = match name.split_once(':') {
            Some((n, c)) => (
                n,
                c.parse::<i32>()
                    .map_err(|_| CliError::Usage(format!("not a count: {c}")))?,
            ),
            None => (name, 1),
        };
        let id = crate::records::find_record(order, name)?.form_id;
        *state.items.entry((player, id)).or_insert(0) += count;
        if equip {
            state.equip(order, player, id);
        }
    }
    // Nowhere in particular (no place name), with the Mojave's world map.
    let mut at = gather::Whereabouts::default();
    if let Some(world_id) = order.form_by_editor_id("WastelandNV") {
        if let Ok(grid) = world::WorldGrid::load(order, world_id) {
            at.world = Some(world_id);
            at.markers = grid
                .persistent
                .map(|p| world::map::markers(order, p))
                .unwrap_or_default();
        }
    }
    let input = gather::gather(order, &state, &at);
    writeln!(
        out,
        "{} - level {}, HP {}/{}, AP {}/{}, XP {:?}, {} items, {} caps, {} perks, {} quests, {} map markers on the world map",
        input.name,
        input.level,
        input.health.0,
        input.health.1,
        input.action_points.0,
        input.action_points.1,
        input.xp,
        input.items.len(),
        input.caps,
        input.perks.len(),
        input.quests.len(),
        input.world_map.as_ref().map_or(0, |m| m.markers.len())
    )?;
    pipboy.fill(&mut ui, &input);
    pipboy.show(&mut ui, section);
    if let Some(page) = page {
        match section {
            Section::Stats => pipboy.stats.show_page(&mut ui, page, &input),
            Section::Items => pipboy.items.show_tab(&mut ui, page, &input),
            Section::Data => pipboy.data.show_tab(&mut ui, page, &input),
        }
    }
    let menu = pipboy.menu();
    let mut textures = GameTextures {
        assets,
        sizes: HashMap::new(),
    };
    ui::draw::update_file_sizes(&mut ui, menu, &mut textures);
    for w in &ui.warnings {
        writeln!(out, "warning: {w}")?;
    }
    writeln!(out)?;
    print_tile(out, &mut ui, menu, 0)?;
    let items = ui::draw_list(&mut ui, menu, &mut textures, &|_| None);
    writeln!(
        out,
        "\nDrawn into the Pip-Boy's picture ({} x {} units), back to front ({}):",
        ui::pipboy::PICTURE_SIZE[0],
        ui::pipboy::PICTURE_SIZE[1],
        items.len()
    )?;
    for item in &items {
        let name = ui.tiles[item.tile].name.clone();
        let c = item.color;
        let color = format!("tint {:.3} {:.3} {:.3} alpha {:.3}", c[0], c[1], c[2], c[3]);
        match &item.kind {
            DrawKind::Image { texture, rect, .. } => writeln!(
                out,
                "  {name}: {texture} at {:.1}, {:.1} size {:.1} x {:.1}, {color}",
                rect[0], rect[1], rect[2], rect[3]
            )?,
            DrawKind::Model { .. } => {}
            DrawKind::Text { glyphs, .. } => {
                let text = ui
                    .string(item.tile, t::STRING)
                    .unwrap_or_default()
                    .replace('\n', "\\n");
                let left = glyphs.iter().map(|g| g.0[0]).fold(f32::INFINITY, f32::min);
                let top = glyphs.iter().map(|g| g.0[1]).fold(f32::INFINITY, f32::min);
                writeln!(
                    out,
                    "  {name}: \"{text}\" from {left:.1}, {top:.1}, {color}"
                )?
            }
        }
    }
    Ok(())
}

/// `font <N|PATH>`: one of the game's fonts (1 to 8 as the INI names
/// them, or a `.fnt` path) after the loader's adjustments: line height,
/// pictures, and every glyph that has a size.
pub fn font(
    out: &mut impl Write,
    assets: &Assets,
    data_dir: &Path,
    which: &str,
) -> Result<(), CliError> {
    let ini = IniSettings::load(&assets::default_settings_files(data_dir));
    let ini_get = |s: &str, k: &str| ini.get(s, k).map(str::to_string);
    let path = match which.parse::<usize>() {
        Ok(n) if (1..=8).contains(&n) => ui::game::font_paths(&ini_get)[n - 1].clone(),
        _ => which.to_string(),
    };
    let bytes = assets
        .read(&path)
        .ok()
        .flatten()
        .ok_or_else(|| CliError::NotFound(format!("{path} not found")))?;
    let font = Font::parse(&bytes).map_err(|e| CliError::InFile {
        path: path.clone(),
        message: e.to_string(),
    })?;
    writeln!(out, "{path}")?;
    writeln!(
        out,
        "Line height {}, descent {} (the first line's pen sits {} below a text tile's y)",
        font.line_height,
        font.descent,
        -2.0 * (font.line_height - font.descent)
    )?;
    for name in &font.textures {
        let tex = Font::texture_path(&name.to_ascii_lowercase());
        let size = assets
            .read(&tex)
            .ok()
            .flatten()
            .and_then(|b| ui::font::read_tex(&b).map(|(w, h, _)| (w, h)));
        match size {
            Some((w, h)) => writeln!(out, "Picture {tex}: {w} x {h}")?,
            None => writeln!(out, "Picture {tex}: not found")?,
        }
    }
    writeln!(
        out,
        "\ncode char  width height  left right baseline  advance   uv top-left .. bottom-right"
    )?;
    for (code, g) in font.glyphs.iter().enumerate() {
        if g.width == 0.0 && g.height == 0.0 && g.kern_right == 0.0 {
            continue;
        }
        let ch = match code as u8 {
            c @ 0x21..=0x7e => (c as char).to_string(),
            b' ' => "' '".into(),
            _ => String::new(),
        };
        writeln!(
            out,
            "{code:4} {ch:>4} {:6} {:6} {:5} {:5} {:8} {:8}   {:.4} {:.4} .. {:.4} {:.4}",
            g.width,
            g.height,
            g.kern_left,
            g.kern_right,
            g.baseline,
            (g.width + g.kern_right) as i32,
            g.uv[0][0],
            g.uv[0][1],
            g.uv[3][0],
            g.uv[3][1]
        )?;
    }
    Ok(())
}

//! The start menu as the pause menu (`ui::menus::start`): the Escape
//! control opens it over the game (`0070c4a0` → `007cb7d0(1, 0)`), which
//! stops while it's up; its Save and Load pages use the viewer's own saves
//! (`docs/PERSISTENCE.md`: F5's `nv-rs-quicksave.txt`, the scripts'
//! `nv-rs-autosave.txt`, and `nv-rs-save-NNNN.txt` made here); Exit Game
//! ends the program. Behind it the file's `pause_background` model
//! (`Interface\PauseScreen\PauseScreen01.NIF`) is drawn flat on the
//! screen as a `Tile3D` (Xbox PDB) draws it: scaled by 1.3333
//! (`007cbaf0`), each piece blended as its `NiAlphaProperty` says.
//!
//! Labelled guesses: the menu's 3D pieces lie in the menus' flat
//! (orthographic) space with the model's x across and z up, and are drawn
//! farthest (largest y) first; nothing of the world shows behind them (a
//! black screen under the additive slides: the pause screen as the game
//! shows it); the save list's order (newest first, by the files' times).

use std::collections::HashMap;
use std::path::{Path, PathBuf};

use bevy::prelude::*;
use cellview::Game;
use ui::draw::{DrawItem, DrawKind};
use ui::menus::start::{self, values, Request, SaveEntry, Setting, StartMenu};
use ui::names::t;
use ui::TileId;

use super::{OpenMenu, Screen};

/// The start menu on screen, and whether the game was already paused
/// when it opened.
pub struct StartScreen {
    pub menu: StartMenu,
    was_paused: bool,
}

impl StartScreen {
    pub fn new(menu: StartMenu, was_paused: bool) -> StartScreen {
        StartScreen { menu, was_paused }
    }
}

/// The settings the user options show and change: (value, default) by
/// setting, as the INI holds them (the exe's defaults where it's silent).
/// Changes stay in this run: the user's own `FalloutPrefs.ini` isn't
/// written (`007d6d70` would).
#[derive(Resource, Default)]
pub struct GameSettings {
    pub values: HashMap<Setting, f32>,
    loaded: bool,
}

/// A save or load the start menu asked for, carried out by the save
/// system (`scripts::save_and_load`, as F5 / F9 with this file).
#[derive(Resource, Default)]
pub struct SaveFiles(pub Option<SaveFile>);

#[derive(Debug, Clone, PartialEq)]
pub enum SaveFile {
    Save(PathBuf),
    Load(PathBuf),
}

/// The viewer's save files.
const QUICKSAVE: &str = "nv-rs-quicksave.txt";
const AUTOSAVE: &str = "nv-rs-autosave.txt";
const SAVE_PREFIX: &str = "nv-rs-save-";

/// An INI value, else the exe's default.
fn ini(game: &Game, section: &str, key: &str, default: f32) -> f32 {
    game.settings.float(section, key).unwrap_or(default)
}

/// The settings' values at the start (the INI's, the exe's defaults
/// where it's silent: `007cc6e0`'s handlers' readers).
fn initial_settings(game: &Game) -> HashMap<Setting, f32> {
    use Setting::*;
    let v = cellview::music::volumes(&game.settings);
    let mut m = HashMap::new();
    m.insert(KillCam, ini(game, "GamePlay", "iKillCamera", 2.0));
    m.insert(Hardcore, 0.0);
    m.insert(Difficulty, ini(game, "GamePlay", "iDifficulty", 2.0));
    m.insert(SaveOnRest, ini(game, "GamePlay", "bSaveOnRest", 1.0));
    m.insert(SaveOnWait, ini(game, "GamePlay", "bSaveOnWait", 1.0));
    m.insert(SaveOnTravel, ini(game, "GamePlay", "bSaveOnTravel", 1.0));
    m.insert(
        TrueIronSights,
        ini(game, "GamePlay", "bTrueIronSights", 1.0),
    );
    m.insert(Crosshair, ini(game, "GamePlay", "bCrossHair", 1.0));
    m.insert(
        DialogueSubtitles,
        ini(game, "GamePlay", "bDialogueSubtitles", 1.0),
    );
    m.insert(
        GeneralSubtitles,
        ini(game, "GamePlay", "bGeneralSubtitles", 0.0),
    );
    m.insert(Brightness, ini(game, "Display", "fGamma", 1.0));
    m.insert(HudOpacity, ini(game, "Interface", "fHudOpacity", 1.0));
    m.insert(TextureSize, ini(game, "Display", "iTexMipMapSkip", 0.0));
    m.insert(MasterVolume, v.master);
    m.insert(MusicVolume, v.music);
    m.insert(RadioVolume, v.radio);
    m.insert(
        FootstepVolume,
        ini(game, "Audio", "fDefaultFootVolume", 1.0),
    );
    m.insert(VoiceVolume, ini(game, "Audio", "fDefaultVoiceVolume", 1.0));
    m.insert(
        EffectsVolume,
        ini(game, "Audio", "fDefaultEffectsVolume", 1.0),
    );
    m.insert(InvertY, ini(game, "Controls", "bInvertYValues", 0.0));
    m.insert(
        MouseSensitivity,
        ini(game, "Controls", "fMouseSensitivity", 0.002),
    );
    for (s, (_, _, key, default)) in fade_ranges() {
        m.insert(s, ini(game, key.0, key.1, default));
    }
    m
}

/// The display page's fade meters: (setting, (min, max, (section, key),
/// default)) from the settings `007d1bb0` .. `007d2650` read; the value
/// placed in a straight line between min and max [guess: the handlers'
/// exact steps aren't translated].
#[allow(clippy::type_complexity)]
fn fade_ranges() -> Vec<(Setting, (f32, f32, (&'static str, &'static str), f32))> {
    use Setting::*;
    vec![
        (
            ActorFade,
            (2.0, 15.0, ("LOD", "fLODFadeOutMultActors"), 6.5),
        ),
        (ItemFade, (1.0, 15.0, ("LOD", "fLODFadeOutMultItems"), 2.0)),
        (
            ObjectFade,
            (1.0, 15.0, ("LOD", "fLODFadeOutMultObjects"), 5.0),
        ),
        (
            GrassFade,
            (400.0, 7000.0, ("Grass", "fGrassStartFadeDistance"), 3500.0),
        ),
        (
            ShadowFade,
            (100.0, 300.0, ("Display", "fShadowLODStartFade"), 200.0),
        ),
        (
            LightFade,
            (200.0, 1200.0, ("Display", "fLightLODStartFade"), 1000.0),
        ),
        (
            SpecularFade,
            (200.0, 600.0, ("Display", "fSpecularLODStartFade"), 500.0),
        ),
        (
            ObjectLodFade,
            (
                25000.0,
                75000.0,
                ("TerrainManager", "fBlockLoadDistanceLow"),
                50000.0,
            ),
        ),
        (
            TreeLodFade,
            (
                10000.0,
                40000.0,
                ("TerrainManager", "fTreeLoadDistance"),
                25000.0,
            ),
        ),
    ]
}

/// A setting's value as the option's index of `count`, and its default's
/// (the handlers' readers, `StartMenu::Set…` (Xbox PDB)).
pub fn to_index(settings: &HashMap<Setting, f32>, s: Setting, count: i32) -> (i32, i32) {
    use Setting::*;
    let v = settings.get(&s).copied().unwrap_or(0.0);
    match s {
        // `007d1260`, `007d11d0`: the number itself; defaults 2.
        KillCam | Difficulty => (v as i32, 2),
        // `007d13a0`: hardcore on or off, default off.
        Hardcore => (values::bool_to_index(v != 0.0), 0),
        SaveOnRest | SaveOnWait | SaveOnTravel | TrueIronSights | Crosshair | DialogueSubtitles => {
            (values::bool_to_index(v != 0.0), 1)
        }
        GeneralSubtitles | InvertY => (values::bool_to_index(v != 0.0), 0),
        // `007d19c0`: gamma between `fGammaMin` 1.4 and `fGammaMax` 0.6.
        Brightness => (
            values::gamma_to_index(v, 1.4, 0.6, count),
            values::gamma_to_index(1.0, 1.4, 0.6, count),
        ),
        // `007d1750`, `007d2790`: 0..1, default the top.
        HudOpacity | MasterVolume | MusicVolume | FootstepVolume | VoiceVolume | EffectsVolume
        | RadioVolume => (values::unit_to_index(v, count), count - 1),
        MouseSensitivity => (
            values::mouse_to_index(v, count),
            ((count - 1) as f32 * values::MOUSE_DEFAULT).round() as i32,
        ),
        // `007d1840` / `007d1900`: the colour's place in the table at
        // `011dac50` (not read here: the default, Amber, index 2).
        HudColor | PipboyColor => (2, 2),
        TextureSize => (v as i32, 0),
        Controller | Rumble => (0, 0),
        _ => {
            let Some((_, (min, max, _, default))) =
                fade_ranges().into_iter().find(|(f, _)| *f == s)
            else {
                return (0, 0);
            };
            let idx = |x: f32| ((count - 1) as f32 * (x - min) / (max - min)).round() as i32;
            (idx(v), idx(default))
        }
    }
}

/// A user option's index back as the setting's value (the handlers'
/// writers).
pub fn from_index(s: Setting, index: i32, count: i32) -> f32 {
    use Setting::*;
    match s {
        KillCam | Difficulty | TextureSize | HudColor | PipboyColor => index as f32,
        Hardcore | SaveOnRest | SaveOnWait | SaveOnTravel | TrueIronSights | Crosshair
        | DialogueSubtitles | GeneralSubtitles | InvertY | Controller | Rumble => {
            f32::from(u8::from(values::index_to_bool(index)))
        }
        Brightness => values::index_to_gamma(index, 1.4, 0.6, count),
        HudOpacity | MasterVolume | MusicVolume | FootstepVolume | VoiceVolume | EffectsVolume
        | RadioVolume => values::index_to_unit(index, count),
        MouseSensitivity => values::index_to_mouse(index, count),
        _ => {
            let Some((_, (min, max, _, _))) = fade_ranges().into_iter().find(|(f, _)| *f == s)
            else {
                return 0.0;
            };
            min + (max - min) * values::index_to_unit(index, count)
        }
    }
}

/// The save files in the folder the viewer runs in, newest first.
pub fn save_files(dir: &Path) -> Vec<(PathBuf, String, std::time::SystemTime)> {
    let mut out = Vec::new();
    let Ok(entries) = std::fs::read_dir(dir) else {
        return out;
    };
    for e in entries.flatten() {
        let name = e.file_name().to_string_lossy().to_string();
        let prefix = if name.eq_ignore_ascii_case(QUICKSAVE) {
            "QUICK".to_string()
        } else if name.eq_ignore_ascii_case(AUTOSAVE) {
            "AUTO".to_string()
        } else if let Some(n) = name
            .strip_prefix(SAVE_PREFIX)
            .and_then(|r| r.strip_suffix(".txt"))
            .and_then(|n| n.parse::<u32>().ok())
        {
            format!("#{n:04}")
        } else {
            continue;
        };
        let time = e
            .metadata()
            .and_then(|m| m.modified())
            .unwrap_or(std::time::UNIX_EPOCH);
        out.push((e.path(), prefix, time));
    }
    out.sort_by_key(|f| std::cmp::Reverse(f.2));
    out
}

/// The next new save's file (`#` numbers count on from the highest).
pub fn next_save_file(dir: &Path) -> PathBuf {
    let top = save_files(dir)
        .iter()
        .filter_map(|(_, p, _)| p.strip_prefix('#').and_then(|n| n.parse::<u32>().ok()))
        .max()
        .unwrap_or(0);
    dir.join(format!("{SAVE_PREFIX}{:04}.txt", top + 1))
}

/// A save's line: its place's name (the cell's, else its worldspace's).
fn save_entry(game: &Game, path: &Path, prefix: String) -> SaveEntry {
    let order = &game.order;
    let text = std::fs::read_to_string(path).ok();
    let loaded = text.as_deref().and_then(|t| world::save::load(t).ok());
    let name_of = |f: esm::FormId| {
        order
            .get(f)
            .and_then(|r| r.record().ok())
            .and_then(|r| r.full_name())
            .filter(|n| !n.is_empty())
    };
    let place = loaded
        .as_ref()
        .and_then(|(_, p)| p.as_ref())
        .and_then(|p| name_of(p.cell).or_else(|| p.world.and_then(name_of)));
    SaveEntry {
        key: path.to_string_lossy().to_string(),
        prefix,
        place: place.unwrap_or_default(),
        player: loaded
            .as_ref()
            .and_then(|(s, _)| s.player_name.clone())
            .unwrap_or_default(),
        level: None,
        play_time: String::new(),
        corrupt: loaded.is_none(),
    }
}

/// The saves as the start menu lists them.
pub fn entries(game: &Game) -> Vec<SaveEntry> {
    save_files(Path::new("."))
        .into_iter()
        .map(|(path, prefix, _)| save_entry(game, &path, prefix))
        .collect()
}

/// Opens the start menu as the pause menu (`007cb7d0(1, 0)` →
/// `007cbaf0`).
pub fn open(
    screen: &mut Screen,
    game: &Game,
    settings: &mut GameSettings,
    can_save: bool,
    was_paused: bool,
) -> bool {
    if !settings.loaded {
        settings.values = initial_settings(game);
        settings.loaded = true;
    }
    let mut menu = StartMenu::new(0, &screen.ui);
    let tile = match screen.load(game, start::FILE, &mut menu) {
        Ok(t) => t,
        Err(e) => {
            println!("The start menu can't be shown: {e}");
            return false;
        }
    };
    menu.menu = tile;
    menu.save_entries = entries(game);
    let have_saves = !menu.save_entries.is_empty();
    let size = screen.size;
    let widescreen = size.x as f32 / size.y.max(1) as f32 > 1.3333;
    let values = settings.values.clone();
    let read = move |s: Setting, count: i32| to_index(&values, s, count);
    if !menu.open(
        &mut screen.ui,
        true,
        can_save,
        have_saves,
        widescreen,
        &read,
    ) {
        println!("MENUS: Start Menu Creation Failed.");
        screen.ui.detach(tile);
        return false;
    }
    screen.ui.set_number(tile, t::VISIBLE, 1.0);
    println!("Pause menu.");
    screen.open.push(OpenMenu::Start(Box::new(StartScreen::new(
        menu, was_paused,
    ))));
    true
}

/// What the start menu asked for this frame, for the caller.
#[derive(Debug, Default)]
pub struct Outcome {
    pub resume: Option<bool>,
    pub exit: bool,
    /// Help asked for (`007d0770`): the help manual, for the caller.
    pub help: bool,
    pub save: Option<SaveFile>,
    pub applied: Vec<(Setting, f32)>,
    pub sounds: Vec<String>,
}

/// Every frame: the menu's fades and saving, then its requests.
pub fn frame(screen: &mut Screen, game: &Game, settings: &mut GameSettings, now: f64) -> Outcome {
    let mut out = Outcome::default();
    let Screen { ui, open, .. } = screen;
    for m in open.iter_mut() {
        let OpenMenu::Start(s) = m else {
            continue;
        };
        s.menu.update(ui, now);
        out.sounds.append(&mut s.menu.sounds);
        for r in std::mem::take(&mut s.menu.requests) {
            match r {
                Request::Resume => {
                    println!("Pause menu closed.");
                    out.resume = Some(s.was_paused);
                }
                Request::SaveNew => out.save = Some(SaveFile::Save(next_save_file(Path::new(".")))),
                Request::SaveOver(key) => out.save = Some(SaveFile::Save(PathBuf::from(key))),
                Request::Load(key) => out.save = Some(SaveFile::Load(PathBuf::from(key))),
                Request::LoadNewest => {
                    if let Some((p, _, _)) = save_files(Path::new(".")).into_iter().next() {
                        out.save = Some(SaveFile::Load(p));
                    }
                }
                Request::Delete(key) => {
                    match std::fs::remove_file(&key) {
                        Ok(()) => println!("Deleted {key}."),
                        Err(e) => println!("Couldn't delete {key}: {e}"),
                    }
                    s.menu.save_entries = entries(game);
                    let saves = s.menu.save_entries.clone();
                    s.menu.open_saves(ui, saves);
                }
                Request::ExitGame => out.exit = true,
                Request::MainMenu => {
                    println!("Main Menu: the main menu isn't made here (not implemented).")
                }
                // `007d0770`: the version shows (the menu's +0x50, tile
                // 10), then the help manual opens over it (the caller).
                Request::Help => {
                    if let Some(v) = s.menu.tiles[start::id::VERSION as usize] {
                        ui.set_number(v, t::VISIBLE, 1.0);
                    }
                    out.help = true;
                }
                Request::Apply {
                    setting,
                    value,
                    count,
                } => {
                    let v = from_index(setting, value, count);
                    settings.values.insert(setting, v);
                    println!("Setting {setting:?}: {v}.");
                    out.applied.push((setting, v));
                }
                Request::SavePreferences => {}
            }
        }
    }
    out
}

/// The pause background's pieces (`pause_background`, id 13) as model
/// draws, inserted under the start menu's other pictures (the file's
/// depth order; the model is its first tile).
pub fn background_draws(
    ui: &mut ui::Ui,
    open: &[OpenMenu],
    game: &Game,
    cache: &mut HashMap<String, Option<Vec<ModelPiece>>>,
    items: &mut Vec<DrawItem>,
    menu_tile: TileId,
    first: usize,
) {
    let Some(bg) = open.iter().find_map(|m| match m {
        OpenMenu::Start(s) if s.menu.menu == menu_tile => s.menu.tiles[13],
        _ => None,
    }) else {
        return;
    };
    if !ui.shown(bg) {
        return;
    }
    let file = ui.string(bg, t::FILENAME).unwrap_or_default();
    let pieces = cache
        .entry(file.clone())
        .or_insert_with(|| model_pieces(game, &file))
        .clone();
    let Some(pieces) = pieces else {
        return;
    };
    let (x, y) = ui.screen_position(bg);
    let depth = ui.screen_depth(bg);
    let s = start::BACKGROUND_SCALE;
    let mut draws = vec![DrawItem {
        tile: bg,
        depth,
        color: [0.0, 0.0, 0.0, 1.0],
        kind: DrawKind::Model {
            texture: None,
            triangles: vec![
                [
                    ([0.0, 0.0], [0.0, 0.0]),
                    ([4000.0, 0.0], [1.0, 0.0]),
                    ([0.0, 4000.0], [0.0, 1.0]),
                ],
                [
                    ([4000.0, 0.0], [1.0, 0.0]),
                    ([4000.0, 4000.0], [1.0, 1.0]),
                    ([0.0, 4000.0], [0.0, 1.0]),
                ],
            ],
            alpha: Vec::new(),
            blend: None,
        },
    }];
    for p in pieces {
        let triangles = p
            .triangles
            .iter()
            .map(|tri| tri.map(|(pos, uv)| ([x + s * pos[0], y - s * pos[1]], uv)))
            .collect();
        draws.push(DrawItem {
            tile: bg,
            depth,
            color: [1.0, 1.0, 1.0, 1.0],
            kind: DrawKind::Model {
                texture: p.texture.clone(),
                triangles,
                alpha: Vec::new(),
                blend: p.blend,
            },
        });
    }
    let at = items[first..]
        .iter()
        .position(|it| it.depth >= depth)
        .map_or(items.len(), |i| first + i);
    items.splice(at..at, draws);
}

/// A model piece: its triangles in the model's space ((x, y, z), (u, v)),
/// its texture and blending.
#[derive(Debug, Clone)]
pub struct ModelPiece {
    pub triangles: Vec<[([f32; 3], [f32; 2]); 3]>,
    pub texture: Option<String>,
    pub blend: Option<(u8, u8)>,
    /// How far into the screen (the model's y), for the order.
    pub far: f32,
}

/// A `nif` tile's model read (`meshes\` + its `filename`), its pieces
/// farthest first.
fn model_pieces(game: &Game, file: &str) -> Option<Vec<ModelPiece>> {
    let path = format!("meshes\\{}", file.trim().to_ascii_lowercase());
    let Some(bytes) = game.assets.read(&path).ok().flatten() else {
        println!("  {path} not found.");
        return None;
    };
    let nif = nif::Nif::parse(bytes)
        .map_err(|e| println!("  {path}: {e}"))
        .ok()?;
    let scene = nif
        .placed_scene()
        .map_err(|e| println!("  {path}: {e}"))
        .ok()?;
    let mut out = Vec::new();
    for mesh in &scene.meshes {
        let tf = &mesh.transform;
        let place = |v: [f32; 3]| {
            let r = nif::math::mat_vec(&tf.rotation, v);
            [
                r[0] * tf.scale + tf.translation[0],
                r[1] * tf.scale + tf.translation[1],
                r[2] * tf.scale + tf.translation[2],
            ]
        };
        let positions: Vec<[f32; 3]> = mesh.positions.iter().map(|p| place(*p)).collect();
        let uv = |i: u16| mesh.uvs.get(i as usize).copied().unwrap_or([0.0, 0.0]);
        let triangles: Vec<_> = mesh
            .triangles
            .iter()
            .map(|t| t.map(|i| (positions[i as usize], uv(i))))
            .collect();
        let far = -positions.iter().map(|p| p[2]).sum::<f32>() / positions.len().max(1) as f32;
        let blend = mesh
            .alpha
            .as_ref()
            .filter(|a| a.blending())
            .map(|a| (((a.flags >> 1) & 0xf) as u8, ((a.flags >> 5) & 0xf) as u8));
        out.push(ModelPiece {
            triangles,
            texture: mesh.diffuse_texture().map(|t| t.to_ascii_lowercase()),
            blend,
            far,
        });
    }
    out.sort_by(|a, b| {
        b.far
            .partial_cmp(&a.far)
            .unwrap_or(std::cmp::Ordering::Equal)
    });
    println!(
        "  {path}: {} pieces ({}).",
        out.len(),
        out.iter()
            .map(|p| format!(
                "{:?} {:?} {} triangles",
                p.texture,
                p.blend,
                p.triangles.len()
            ))
            .collect::<Vec<_>>()
            .join(", ")
    );
    Some(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The settings' values to options and back as the handlers do.
    #[test]
    fn settings_round_trip_through_the_options() {
        let mut m = HashMap::new();
        m.insert(Setting::MusicVolume, 0.6);
        m.insert(Setting::MouseSensitivity, 0.002);
        m.insert(Setting::Crosshair, 1.0);
        m.insert(Setting::Difficulty, 2.0);
        assert_eq!(to_index(&m, Setting::MusicVolume, 26), (15, 25));
        assert!((from_index(Setting::MusicVolume, 15, 26) - 0.6).abs() < 1e-6);
        assert_eq!(to_index(&m, Setting::MouseSensitivity, 16), (2, 2));
        assert_eq!(to_index(&m, Setting::Crosshair, 2), (1, 1));
        assert_eq!(from_index(Setting::Crosshair, 0, 2), 0.0);
        assert_eq!(to_index(&m, Setting::Difficulty, 5), (2, 2));
    }

    /// The save files: the quick and auto saves and the numbered ones,
    /// the next number after the highest.
    #[test]
    fn save_files_are_listed_and_numbered() {
        let dir = std::env::temp_dir().join(format!("nv-rs-start-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        for f in ["nv-rs-quicksave.txt", "nv-rs-save-0002.txt", "other.txt"] {
            std::fs::write(dir.join(f), "x").unwrap();
        }
        let files = save_files(&dir);
        let mut prefixes: Vec<String> = files.iter().map(|f| f.1.clone()).collect();
        prefixes.sort();
        assert_eq!(prefixes, ["#0002", "QUICK"]);
        assert_eq!(next_save_file(&dir), dir.join("nv-rs-save-0003.txt"));
        std::fs::remove_dir_all(&dir).unwrap();
    }
}

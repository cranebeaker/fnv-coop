//! The HUD (`hud_main_menu.xml`, class `HUDMainMenu`, vtable `01072df4`
//! in FalloutNV.exe). Its file only holds empty containers and templates;
//! the game's code places them and fills them. This follows that code:
//! the setup (`0076bfe0`), and the update every frame (`00770430`), with
//! the meters (`007748b0`), ammunition and condition (`007721c0`), the
//! compass (`00779070`) and the messages. Checked against a recorded
//! frame of the game's HUD (Doc Mitchell's house): every piece below
//! lands on the recording's positions, sizes and colours.

use std::collections::VecDeque;

use crate::anim::Animations;
use crate::names::t;
use crate::tile::{TileId, Ui};
use crate::xp::{Experience, XpMeter};

/// The HUD menu's file.
pub const MENU_FILE: &str = "menus\\main\\hud_main_menu.xml";
/// The compass strip's alpha map (`0076bfe0`, attached to the compass
/// window's `TILE1001` shader).
pub const COMPASS_ALPHA_MAP: &str = "textures\\interface\\hud\\hud_compass_alphamap.dds";

/// The default message icons by type (`00775380`).
pub const MESSAGE_ICONS: [&str; 4] = [
    "Interface\\Icons\\Message Icons\\glow_message_vaultboy_neutral.dds",
    "Interface\\Icons\\Message Icons\\glow_message_vaultboy_very_happy.dds",
    "Interface\\Icons\\Message Icons\\glow_message_vaultboy_sad.dds",
    "Interface\\Icons\\Message Icons\\glow_message_vaultboy_in_pain.dds",
];

/// How long messages fade in and out (`0.35` at `01021f44`).
pub const MESSAGE_FADE: f32 = 0.35;

/// How long a message stays when its sender doesn't say: 2 seconds (the
/// float at `010162c0` most callers of the message queue `007052f0` pass;
/// a script's message box without buttons passes the message's own time).
pub const MESSAGE_SECONDS: f32 = 2.0;

/// The HUD's parts whose code isn't followed here yet: enemy health,
/// hotkeys, the region name, radiation, explosives, hardcore needs, the
/// breath meter, crippled limbs, the damage threshold icons and the
/// ammunition type. They're hidden; in normal play the game draws them all
/// at alpha 0 (every one had alpha 0 in the recorded frame), so nothing on
/// screen differs until aiming at someone and so on. (The quest reminder
/// is `crate::quest_text`.)
pub const NOT_FOLLOWED: [&str; 12] = [
    "EnemyHealth",
    "Hokeys",
    "Region_Location",
    "RadiationMeter",
    "BreathMeter",
    "Explosive_positioning_rect",
    "crippled_limb_indicator",
    "DDTIcon",
    "DDTIconAP",
    "DDTIconEnemy",
    "DDTIconEnemyAP",
    "HardcoreMode",
];

/// What a mask hid ([`Hud::apply_mask`]), to show again when it's lifted.
#[derive(Debug, Clone, Default)]
pub struct Masked(Vec<(TileId, f32)>);

/// The HUD's pieces as `00771700` shows and hides them, one bit each (the
/// order of the HUD object's fields +0x114 .. +0x160, read from its
/// disassembly; the tiles each bit covers: [`Hud::apply_mask`]).
pub mod part {
    pub const ACTION_POINTS: u32 = 0x1;
    pub const HIT_POINTS: u32 = 0x2;
    pub const RADIATION: u32 = 0x4;
    pub const ENEMY_HEALTH: u32 = 0x8;
    pub const QUEST_REMINDER: u32 = 0x10;
    /// The Info roll-over prompt (`HUDMainMenu` +0x138, `00771700`).
    pub const INFO: u32 = 0x200;
    pub const RETICLE: u32 = 0x40;
    pub const SNEAK: u32 = 0x80;
    pub const MESSAGES: u32 = 0x100;
    pub const SUBTITLES: u32 = 0x400;
    pub const HOTKEYS: u32 = 0x800;
    pub const XP_METER: u32 = 0x1000;
    pub const BREATH: u32 = 0x2000;
    pub const EXPLOSIVE: u32 = 0x4000;
    pub const HARDCORE: u32 = 0x10000;
    /// Everything (no menu open, `00771700` mode 2).
    pub const ALL: u32 = 0x1ffff;
}

/// The HUD's masks while another menu runs (`00771700`, see
/// [`Hud::apply_mask`]; the game's menus' masks: [`parts_for_menu`]).
pub mod mask {
    use super::part;
    /// V.A.T.S.'s menu on top (menu 0x420, mode 7): the messages only.
    pub const VATS_MENU: u32 = part::MESSAGES;
    /// V.A.T.S. playing (its mode 4, no menu): messages and enemy health.
    pub const VATS_PLAYBACK: u32 = part::MESSAGES | part::ENEMY_HEALTH;
    /// Gameplay while movement controls are disabled (`00771700`, mode 0xC).
    /// The game leaves radiation, quest reminders, subtitles, XP and
    /// hardcore needs visible; AP, HP/compass, reticle and Info are hidden.
    pub const MOVEMENT_DISABLED: u32 = 0x1514;
    /// Normal gameplay with `DisablePlayerControls`' rollover bit set.
    pub const ROLLOVER_DISABLED: u32 = part::ALL & !part::INFO;
}

/// Which of the HUD's pieces stay on screen while menus are open
/// (`0070c4a0` picks a mode by the menu that opened from the game, then
/// `00771700` the mode's pieces; a menu opening over another changes
/// nothing). No menu: all; the message box (mode 0x11), the pause menu,
/// level-up, character making, the love tester, barter, waiting, books and
/// the rest (mode 4): none; dialogue (6): the quest reminders, messages and
/// XP meter; a container (9): the XP meter; a terminal or hacking (0x10,
/// 0xf): the quest reminders and XP meter; the Pip-Boy's pages,
/// lockpicking, repairing, item mods (3) and V.A.T.S. (7): the messages.
pub fn parts_for_menu(class: Option<i32>) -> u32 {
    let mode = match class {
        None => 2,
        Some(1001) => 0x11,
        Some(1002) | Some(1003) | Some(1014) | Some(1023) | Some(1035) | Some(1061) => 3,
        Some(1007) => 5,
        Some(1008) => 9,
        Some(1009) => 6,
        Some(1027) | Some(1048) => 0xe,
        Some(1055) => 0xf,
        Some(1056) => 7,
        Some(1057) => 0x10,
        Some(1060) => 0x12,
        Some(1074) => 0x13,
        Some(1080..=1083) => 0x19,
        Some(_) => 4,
    };
    match mode {
        2 => part::ALL,
        3 | 7 => part::MESSAGES,
        6 => part::QUEST_REMINDER | part::MESSAGES | part::XP_METER,
        8 => part::MESSAGES | part::ENEMY_HEALTH,
        9 => part::XP_METER,
        0xf | 0x10 => part::QUEST_REMINDER | part::XP_METER,
        _ => 0,
    }
}

/// The HUD parts the player sees in gameplay when no game menu is open.
/// `00771700` switches to mode 0xC while movement controls are off, and
/// otherwise removes only Info when the rollover control is off.
pub fn gameplay_parts(movement_disabled: bool, rollover_disabled: bool) -> u32 {
    if movement_disabled {
        mask::MOVEMENT_DISABLED
    } else if rollover_disabled {
        mask::ROLLOVER_DISABLED
    } else {
        part::ALL
    }
}

/// The pieces the code keeps (the menu object's fields).
#[derive(Debug, Clone, Default)]
pub struct HudTiles {
    pub action_points: TileId,
    pub ap_bracket: TileId,
    pub ap_meter: TileId,
    pub ap_label: TileId,
    pub ammo: TileId,
    pub condition_label: TileId,
    pub condition_meter: TileId,
    pub condition_background: TileId,
    pub condition_arrows: Option<TileId>,
    /// `AmmoTypeLabel` (`HUDMainMenu` +0x15c): the loaded ammunition's
    /// abbreviation (`AMMO` `QNAM`).
    pub ammo_type: Option<TileId>,
    pub hit_points: TileId,
    pub hp_bracket: TileId,
    pub hp_meter: TileId,
    pub hp_label: TileId,
    pub compass: TileId,
    /// The marker, quest and NPC icon groups (+0x34, +0x38, +0x3c).
    pub compass_groups: [TileId; 3],
    pub compass_markers: Vec<TileId>,
    pub compass_quests: Vec<TileId>,
    pub compass_player: Option<TileId>,
    pub compass_npcs: Vec<TileId>,
    pub reticle_center: TileId,
    pub reticle: TileId,
    /// The Info roll-over panel (`HUDMainMenu` +0x138, mask bit 0x200).
    pub info: TileId,
    pub info_hotrect: TileId,
    pub info_pc_shortcut: TileId,
    pub info_xbox_button: TileId,
    pub info_target: TileId,
    /// The Info panel's other lines (`HUDMainMenu` +0xac … +0xc4,
    /// `0076bfe0`): the lock line, "Empty", the weight and its label, the
    /// value and its label, the separator.
    pub info_lock: TileId,
    pub info_empty: TileId,
    pub info_weight: TileId,
    pub info_weight_label: TileId,
    pub info_value: TileId,
    pub info_value_label: TileId,
    pub info_separator: TileId,
    /// The sneak meter (+0x130) and its text (`sneak_nif`, +0x88).
    pub sneak_meter: TileId,
    pub sneak_text: TileId,
    pub messages: TileId,
    pub message_icon: TileId,
    pub message_text: TileId,
    pub message_bracket: TileId,
    pub subtitles: TileId,
    pub subtitle_text: TileId,
}

/// What the native Info updater (`00775a00`) puts in the panel, as the
/// game state layer works it out (`world::activation::info`). `shortcut`
/// is the bound key's name; the HUD appends the native closing
/// parenthesis (for example `E)`).
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct InfoPrompt {
    /// The action ("Take", "Sit" …); `None`: no action line (a gecko's
    /// name alone).
    pub action: Option<String>,
    pub target: String,
    pub shortcut: Option<String>,
    /// Native system colour: ordinary HUD (1) or crime warning (2).
    pub crime: bool,
    /// The lock line ("[Locked - Easy]") and the "Empty" line.
    pub lock: Option<String>,
    pub empty: Option<String>,
    /// An item's weight, its label, its value and its label.
    pub weight_value: Option<[String; 4]>,
}

impl Hud {
    /// Fills the native Info panel (`00775a00`): the action and key
    /// (shown with an action: bit 2), the name and the lock line (bit 4;
    /// the lock line transparent unless locked), "Empty" (with the name,
    /// while the lock line is transparent), weight, value, their labels and
    /// the separator (bits 8 … 0x80, for items), all in the crime colour
    /// when taking it is a crime. `None` hides it all. The lines' height
    /// adjustment for names that wrap (the end of `00775a00`) isn't done.
    pub fn update_info(&self, ui: &mut Ui, prompt: Option<&InfoPrompt>, opacity: f32) {
        let tiles = &self.tiles;
        let action = prompt.and_then(|p| p.action.as_deref()).unwrap_or("");
        let target = prompt.map_or("", |p| p.target.as_str());
        let shortcut = prompt
            .and_then(|p| p.shortcut.as_deref())
            .map(|key| format!("{key})"))
            .unwrap_or_default();
        let pc_button_text = ui.names.lookup_or_add("_PCButtonText").unwrap_or(0);
        ui.set_string(tiles.info_hotrect, t::STRING, action);
        ui.set_string(tiles.info_hotrect, pc_button_text, &shortcut);
        ui.set_string(tiles.info_target, t::STRING, target);
        let color = if prompt.is_some_and(|p| p.crime) {
            2.0
        } else {
            1.0
        };
        let full = opacity.clamp(0.0, 1.0) * 255.0;
        let lines = [
            tiles.info_hotrect,
            tiles.info_target,
            tiles.info_lock,
            tiles.info_empty,
            tiles.info_weight,
            tiles.info_weight_label,
            tiles.info_value,
            tiles.info_value_label,
            tiles.info_separator,
        ];
        for tile in lines {
            ui.set_number(tile, t::SYSTEMCOLOR, color);
        }
        ui.set_number(tiles.info_hotrect, t::ALPHA, full);
        ui.set_number(tiles.info_target, t::ALPHA, full);
        let shows = |b: bool| if b { 1.0 } else { 0.0 };
        let has_action = prompt.is_some_and(|p| p.action.is_some());
        let has_name = prompt.is_some();
        ui.set_number(tiles.info_hotrect, t::VISIBLE, shows(has_action));
        ui.set_number(tiles.info_target, t::VISIBLE, shows(has_name));
        // The lock line: shown with the name, transparent unless locked.
        let lock = prompt.and_then(|p| p.lock.as_deref());
        if let Some(text) = lock {
            ui.set_string(tiles.info_lock, t::STRING, text);
        }
        ui.set_number(
            tiles.info_lock,
            t::ALPHA,
            if lock.is_some() { full } else { 0.0 },
        );
        ui.set_number(tiles.info_lock, t::VISIBLE, shows(has_name));
        let empty = prompt.and_then(|p| p.empty.as_deref());
        if let Some(text) = empty {
            ui.set_string(tiles.info_empty, t::STRING, text);
        }
        ui.set_number(
            tiles.info_empty,
            t::ALPHA,
            if empty.is_some() { full } else { 0.0 },
        );
        ui.set_number(
            tiles.info_empty,
            t::VISIBLE,
            shows(has_name && lock.is_none()),
        );
        let weight_value = prompt.and_then(|p| p.weight_value.as_ref());
        if let Some([weight, weight_label, value, value_label]) = weight_value {
            ui.set_string(tiles.info_weight, t::STRING, weight);
            ui.set_string(tiles.info_weight_label, t::STRING, weight_label);
            ui.set_string(tiles.info_value, t::STRING, value);
            ui.set_string(tiles.info_value_label, t::STRING, value_label);
        }
        // Their alpha: `fHudOpacity` × 255 (`011d979c`, set at the setup).
        ui.set_number(tiles.info_weight, t::ALPHA, full);
        ui.set_number(tiles.info_value, t::ALPHA, full);
        for tile in [
            tiles.info_weight,
            tiles.info_weight_label,
            tiles.info_value,
            tiles.info_value_label,
            tiles.info_separator,
        ] {
            ui.set_number(tile, t::VISIBLE, shows(weight_value.is_some()));
        }
        ui.set_number(
            tiles.info_pc_shortcut,
            t::VISIBLE,
            shows(has_action && prompt.and_then(|p| p.shortcut.as_ref()).is_some()),
        );
        // Xbox input naming isn't resolved yet, so don't leave its template
        // glyph visible as a blank placeholder.
        ui.set_number(tiles.info_xbox_button, t::VISIBLE, 0.0);
    }

    /// The sneak meter for this frame (`00770430`): while the player
    /// sneaks, its text and colour (`007732d0`, [`SneakMeterState`]),
    /// faded in over 0.5 s (from where its alpha is), or, for [DANGER],
    /// flashed three times first (`00a07c60` mode 2) on entering it; not
    /// sneaking, faded out over 0.5 s.
    pub fn update_sneak(
        &mut self,
        ui: &mut Ui,
        meter: Option<SneakMeterState>,
        opacity: f32,
        now: f32,
    ) {
        let text = self.tiles.sneak_text;
        // Shown in normal play (`00771700` mode 2's mask has 0x80; other
        // masks hide it after this).
        ui.set_number(self.tiles.sneak_meter, t::VISIBLE, 1.0);
        let now = f64::from(now);
        let full = opacity.clamp(0.0, 1.0) * 255.0;
        let alpha = ui.number(text, t::ALPHA);
        let Some(meter) = meter else {
            // Translated from 00770430 (decompiled, FalloutNV.exe 1.4.0.525)
            if alpha > 0.0 && self.anims.done(text, t::ALPHA, now) {
                self.anims
                    .start(text, t::ALPHA, alpha, 0.0, SNEAK_FADE, now);
            }
            self.sneak_flashing = false;
            return;
        };
        // Translated from 007732d0 (decompiled, FalloutNV.exe 1.4.0.525)
        let (setting, exe) = meter.text();
        let words = ui.setting_text(setting).unwrap_or_else(|| exe.to_string());
        ui.set_string(text, t::STRING, &words);
        ui.set_number(text, t::SYSTEMCOLOR, if meter.red() { 2.0 } else { 1.0 });
        let flashing = meter == SneakMeterState::Danger;
        if flashing && !self.sneak_flashing {
            self.anims.flash(text, t::ALPHA, 0.0, full, SNEAK_FADE, now);
        } else if (!flashing && self.anims.mode(text, t::ALPHA) == 2)
            || (self.anims.done(text, t::ALPHA, now) && alpha != full)
        {
            self.anims
                .start(text, t::ALPHA, alpha, full, SNEAK_FADE, now);
        }
        self.sneak_flashing = flashing;
    }
}

/// How long the sneak meter fades and each of its flashes lasts (the float
/// 0.5 at `01016248`).
pub const SNEAK_FADE: f32 = 0.5;

/// What the sneak meter says (`007732d0`): in combat [DANGER] (red,
/// flashing), [CAUTION] (red) while every one fighting the player is
/// searching, or the player's hostile detection flag is set; out of it
/// [HIDDEN] while no one detects the player (the highest detection level,
/// `00973710`, below 1), else [DETECTED].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SneakMeterState {
    Hidden,
    Detected,
    Caution,
    Danger,
}

impl SneakMeterState {
    /// From the player's combat flags (`PlayerCharacter` +0xdf0
    /// `bPlayerInCombat`, +0xdf1 `bAllCombatTargetsSearching`, +0x5f8
    /// `bHostileDetection` (Xbox PDB)) and the highest detection level.
    // Translated from 007732d0 (decompiled, FalloutNV.exe 1.4.0.525)
    pub fn of(
        in_combat: bool,
        all_searching: bool,
        hostile_detection: bool,
        level: i32,
    ) -> SneakMeterState {
        if in_combat || all_searching {
            if all_searching {
                SneakMeterState::Caution
            } else {
                SneakMeterState::Danger
            }
        } else if hostile_detection {
            SneakMeterState::Caution
        } else if level < 1 {
            SneakMeterState::Hidden
        } else {
            SneakMeterState::Detected
        }
    }

    /// The setting for its words and the exe's default.
    pub fn text(self) -> (&'static str, &'static str) {
        match self {
            SneakMeterState::Hidden => ("sSneakHidden", "[HIDDEN]"),
            SneakMeterState::Detected => ("sSneakDetected", "[DETECTED]"),
            SneakMeterState::Caution => ("sSneakCaution", "[CAUTION]"),
            SneakMeterState::Danger => ("sSneakDanger", "[DANGER]"),
        }
    }

    /// The crime / alarm colour (system colour 2).
    pub fn red(self) -> bool {
        matches!(self, SneakMeterState::Caution | SneakMeterState::Danger)
    }
}

/// What the HUD shows, from the game's state.
#[derive(Debug, Clone, Default)]
pub struct HudInput {
    /// Seconds since the start (for fades and timers).
    pub time: f32,
    pub health: f32,
    /// The player's permanent health (the HP meter's full width).
    pub health_max: f32,
    pub dead: bool,
    pub action_points: f32,
    pub action_points_max: f32,
    pub weapon: Option<WeaponState>,
    /// The player's heading, degrees clockwise from north (with the
    /// worldspace's north turned in, as the game adds it outdoors).
    pub heading: f32,
    pub position: [f32; 3],
    pub interior: bool,
    pub markers: Vec<CompassMarker>,
    pub actors: Vec<CompassActor>,
    /// The active quest's targets (`crate::compass::CompassQuest`).
    pub quests: Vec<crate::compass::CompassQuest>,
    /// `fHudOpacity` (INI, 1 by default).
    pub opacity: f32,
    /// The crosshair is shown (on foot, nothing else on screen).
    pub crosshair: bool,
    /// What's being said, for the subtitles.
    pub subtitle: Option<String>,
    /// The player's experience, for the XP meter.
    pub experience: Option<Experience>,
    /// A menu other than the dialogue menu is open (the XP meter hides and
    /// starts afresh).
    pub menu_open: bool,
    /// What lets the quest text show (`crate::quest_text::Gate`).
    pub quest_gate: crate::quest_text::Gate,
}

/// The weapon in hand.
#[derive(Debug, Clone, PartialEq)]
pub struct WeaponState {
    /// Changes when the weapon does (the game re-shows the condition
    /// pieces then).
    pub id: u32,
    /// For weapons that use ammunition: the rounds in the clip, and the
    /// count of their ammunition less the clip (`"%i/%i"`, `007721c0`).
    pub ammo: Option<(i32, i32)>,
    /// The loaded ammunition's abbreviation (`AMMO` `QNAM`: "HP" for hollow
    /// points; standard rounds have none), for `AmmoTypeLabel`.
    pub ammo_abbrev: Option<String>,
    /// Condition, 0 to 1, and as the percentage the blinking reads.
    pub condition: f32,
}

/// A map marker the compass may show.
#[derive(Debug, Clone, PartialEq)]
pub struct CompassMarker {
    pub position: [f32; 3],
    pub found: bool,
}

/// A person or creature the compass may tick.
#[derive(Debug, Clone, PartialEq)]
pub struct CompassActor {
    pub position: [f32; 3],
    pub hostile: bool,
}

/// A message waiting or showing (`00775380`): text, icon, how long.
#[derive(Debug, Clone, PartialEq)]
pub struct Message {
    pub text: String,
    pub icon: String,
    pub seconds: f32,
}

/// The HUD, made and kept up to date.
pub struct Hud {
    pub menu: TileId,
    pub tiles: HudTiles,
    total_width: i32,
    image_width: i32,
    original_x: i32,
    heading_trait: i32,
    distance_trait: i32,
    /// `_AlphaDown`, where a quest icon keeps its blink state.
    alpha_down_trait: i32,
    /// The last update's time, for the blink's frame seconds.
    last_time: Option<f32>,
    /// The compass strip's scroll (`TexScroll`: x offset, y, scale x, y).
    pub compass_scroll: [f32; 4],
    condition_alpha: Option<f32>,
    condition_down: bool,
    last_weapon: Option<u32>,
    last_condition: Option<f32>,
    /// The ammunition label moved the condition pieces 50 left
    /// (`007721c0`, `011d96b8`).
    ammo_label_shifted: bool,
    pub queue: VecDeque<Message>,
    message_started: Option<f32>,
    /// Traits moving (`00a07c60`): the messages' and the XP meter's fades.
    anims: Animations,
    /// The XP meter, when the menu has one.
    pub xp: Option<XpMeter>,
    /// The sneak meter's flash started for this [DANGER] (its text tile's
    /// trait 0x1004, `007732d0`).
    sneak_flashing: bool,
    /// The quest text (`QuestReminder`), when the menu has it.
    pub quest: Option<crate::quest_text::QuestText>,
    /// Its game settings.
    pub quest_timing: crate::quest_text::Timing,
    /// Sounds the HUD asks for (editor IDs), for the caller to play.
    pub sounds: Vec<String>,
}

/// What [`Hud::create`] needs: the menu's text and a file reader for its
/// prefabs.
pub fn load(ui: &mut Ui, read: &mut dyn FnMut(&str) -> Option<Vec<u8>>) -> Result<Hud, String> {
    let text = read(MENU_FILE).ok_or_else(|| format!("{MENU_FILE} not found"))?;
    let menu = ui.load_menu(&text, read)?;
    Hud::create(ui, menu)
}

impl Hud {
    /// Places and fills the HUD's pieces as `0076bfe0` does. W, H: the
    /// screen tile's size truncated; sx, sy: the safe zone.
    pub fn create(ui: &mut Ui, menu: TileId) -> Result<Hud, String> {
        let screen = ui.screen_size;
        let w = screen.width() as i32;
        let h = screen.height() as i32;
        let sx = screen.safe_x as i32;
        let sy = screen.safe_y as i32;
        let names = |ui: &mut Ui, n: &str| ui.names.lookup_or_add(n).unwrap_or(0);
        let original_x = names(ui, "_OriginalX");
        let total_width = names(ui, "_TotalWidth");
        let image_width = names(ui, "_ImageWidth");
        let heading_trait = names(ui, "_Heading");
        let distance_trait = names(ui, "_Distance");
        let alpha_down_trait = names(ui, "_AlphaDown");
        let find = |ui: &Ui, n: &str| {
            ui.find(menu, n)
                .ok_or_else(|| format!("the HUD has no {n}"))
        };
        let make = |ui: &mut Ui, parent: TileId, n: &str| {
            ui.instantiate(menu, parent, n)
                .ok_or_else(|| format!("the HUD has no template {n}"))
        };
        // The brightness each kind of piece gets (`00774800`): 1 and 3:
        // 175, 2: 255, 4: 44 then (no break) 100, others 255.
        let bright = |ui: &mut Ui, tile: TileId, kind: i32| {
            let b = match kind {
                1 | 3 => 175.0,
                2 => 255.0,
                4 | 5 => 100.0,
                _ => 255.0,
            };
            ui.set_number(tile, t::BRIGHTNESS, b);
        };

        // Action points, bottom right.
        let ap = find(ui, "ActionPoints")?;
        let ap_w = ui.number(ap, t::WIDTH);
        ui.set_number(ap, t::X, (w - sx * 2) as f32 - ap_w + 30.0);
        let ap_h = ui.number(ap, t::HEIGHT);
        ui.set_number(ap, t::Y, (h - sy * 2) as f32 - ap_h);
        ui.set_number(ap, t::LOCUS, 1.0);
        let ap_bracket = make(ui, ap, "template_right_bracket")?;
        bright(ui, ap_bracket, 2);
        ui.set_number(ap_bracket, t::X, -37.0);
        let ap_meter = make(ui, ap, "template_meter")?;
        ui.set_number(ap_meter, original_x, 326.0);
        ui.set_number(ap_meter, t::Y, 41.0);
        bright(ui, ap_meter, 3);
        let ap_label = make(ui, ap, "template_justify_right_text")?;
        ui.set_number(ap_label, t::X, 315.0);
        ui.set_number(ap_label, t::Y, 8.0);
        bright(ui, ap_label, 0);
        let ammo = make(ui, ap, "template_justify_right_text")?;
        ui.set_number(ammo, t::X, 315.0);
        ui.set_number(ammo, t::Y, 80.0);
        ui.set_number(ammo, t::FONT, 7.0);
        bright(ui, ammo, 0);
        let condition_label = make(ui, ap, "template_justify_right_text")?;
        ui.set_number(condition_label, t::X, 80.0);
        let ammo_y = ui.number(ammo, t::Y);
        ui.set_number(condition_label, t::Y, ammo_y);
        ui.set_number(condition_label, t::FONT, 7.0);
        let cnd = ui
            .setting_text("sInventoryCondition")
            .unwrap_or_else(|| "CND".into());
        ui.set_string(condition_label, t::STRING, &cnd);
        bright(ui, condition_label, 0);
        let condition_meter = make(ui, ap, "template_meter")?;
        ui.set_number(condition_meter, original_x, 90.0);
        ui.set_number(condition_meter, t::Y, ammo_y + 3.0);
        ui.set_string(
            condition_meter,
            t::FILENAME,
            "Interface\\VATS\\vats_bar.dds",
        );
        ui.set_number(condition_meter, total_width, 60.0);
        ui.set_number(condition_meter, image_width, 1.0);
        ui.set_number(condition_meter, t::ZOOM, 200.0);
        bright(ui, condition_meter, 3);
        let condition_background = make(ui, ap, "template_meter_background")?;
        ui.set_number(condition_background, t::Y, ammo_y + 3.0);
        bright(ui, condition_background, 4);
        let condition_arrows = ui.find(menu, "CNDArrows");
        if let Some(arrows) = condition_arrows {
            let x = ui.number(ap, t::X)
                + ui.number(condition_label, t::X)
                + ui.number(condition_meter, t::X)
                + ui.number(condition_background, t::WIDTH) * 0.75
                + 8.0;
            ui.set_number(arrows, t::X, x);
            let y = ui.number(ap, t::Y) + ui.number(condition_meter, t::Y);
            ui.set_number(arrows, t::Y, y);
            let bh = ui.number(condition_background, t::HEIGHT);
            ui.set_number(arrows, t::HEIGHT, bh);
            ui.set_number(arrows, t::VISIBLE, 0.0);
        }
        // The ammunition type label (`0076bfe0` at `0076f43e`): at the
        // action points' rect plus the ammunition count's place, 165 to the
        // left (the double at `01072f00`), shown, at full brightness.
        let ammo_type = ui.find(menu, "AmmoTypeLabel");
        if let Some(label) = ammo_type {
            let x = ui.number(ap, t::X) + ui.number(ammo, t::X) - 165.0;
            let y = ui.number(ap, t::Y) + ui.number(ammo, t::Y);
            ui.set_number(label, t::X, x);
            ui.set_number(label, t::Y, y);
            bright(ui, label, 0);
            ui.set_number(label, t::VISIBLE, 1.0);
        }

        // Hit points and the compass, bottom left.
        let hp = find(ui, "HitPoints")?;
        ui.set_number(hp, t::X, (sx * 2 + 10) as f32);
        let hp_h = ui.number(hp, t::HEIGHT);
        ui.set_number(hp, t::Y, (h - sy * 2) as f32 - hp_h);
        ui.set_number(hp, t::LOCUS, 1.0);
        let hp_bracket = make(ui, hp, "template_left_bracket")?;
        bright(ui, hp_bracket, 2);
        let hp_meter = make(ui, hp, "template_meter")?;
        ui.set_number(hp_meter, original_x, 20.0);
        ui.set_number(hp_meter, t::Y, 41.0);
        bright(ui, hp_meter, 3);
        let hp_label = make(ui, hp, "template_justify_right_text")?;
        ui.set_number(hp_label, t::X, 53.0);
        ui.set_number(hp_label, t::Y, 8.0);
        bright(ui, hp_label, 0);
        let compass = make(ui, hp, "template_compass_window")?;
        let mut groups = [0; 3];
        for (i, (y, depth)) in [(30.0, 4.0), (30.0, 3.0), (10.0, 2.0)]
            .into_iter()
            .enumerate()
        {
            let g = make(ui, compass, "template_compass_icon_group")?;
            ui.set_number(g, t::Y, y);
            ui.set_number(g, t::DEPTH, depth);
            groups[i] = g;
        }
        // `iHUDMaxCompassNPCTicks` (10).
        let npc_count = 10;
        let mut compass_npcs = Vec::new();
        for _ in 0..npc_count {
            compass_npcs.push(make(ui, groups[2], "compass_npc_icon")?);
        }
        let mut compass_quests = Vec::new();
        for _ in 0..10 {
            compass_quests.push(make(ui, groups[1], "template_compass_icon_quest")?);
        }
        let compass_player = ui.instantiate(menu, groups[1], "template_compass_icon_player");
        let mut compass_markers = Vec::new();
        for _ in 0..10 {
            compass_markers.push(make(ui, groups[0], "template_compass_icon_marker")?);
        }

        // The crosshair, in the middle.
        let reticle_center = find(ui, "ReticleCenter")?;
        let rw = ui.number(reticle_center, t::WIDTH);
        ui.set_number(reticle_center, t::X, (w / 2) as f32 - rw / 2.0 - 3.0);
        let rh = ui.number(reticle_center, t::HEIGHT);
        ui.set_number(reticle_center, t::Y, (h / 2) as f32 - rh / 2.0 - 3.0);
        ui.set_number(reticle_center, t::LOCUS, 1.0);
        ui.set_number(reticle_center, t::VISIBLE, 1.0);
        let reticle = make(ui, reticle_center, "template_reticle_center")?;
        ui.set_number(reticle, t::SYSTEMCOLOR, 1.0);
        ui.set_number(reticle, t::VISIBLE, 1.0);

        // The native roll-over panel (`0076d7ae`): its centered hotrect owns
        // the action label and PC shortcut, while the text line names the
        // target. The action/target selection itself is supplied by the game
        // state layer (`00775a00`).
        let info = find(ui, "Info")?;
        let iw = ui.number(info, t::WIDTH);
        ui.set_number(info, t::X, (w / 2) as f32 - iw / 2.0);
        let ih = ui.number(info, t::HEIGHT);
        ui.set_number(info, t::Y, (h - sy * 2) as f32 - ih - 30.0);
        ui.set_number(info, t::LOCUS, 1.0);
        // This already exists in hud_main_menu.xml: 0076d7ae calls the
        // tile lookup (00a08b20), not template instantiation (00a1ddb0).
        let info_hotrect = ui
            .find(info, "justify_center_hotrect")
            .ok_or_else(|| "the HUD's Info has no justify_center_hotrect".to_string())?;
        let info_pc_shortcut = ui
            .find(info_hotrect, "PCShortcutLabel")
            .ok_or_else(|| "the HUD's Info hotrect has no PCShortcutLabel".to_string())?;
        let info_xbox_button = ui
            .find(info_hotrect, "xbox_button")
            .ok_or_else(|| "the HUD's Info hotrect has no xbox_button".to_string())?;
        // 0106ec90 is the string "_x", consumed by text_box.xml's layout.
        let info_center_x = names(ui, "_x");
        ui.set_number(info_hotrect, info_center_x, iw / 2.0);
        bright(ui, info_hotrect, 0);
        let info_target = make(ui, info, "template_justify_center_text")?;
        bright(ui, info_target, 0);
        ui.set_number(info_target, t::X, iw / 2.0);
        ui.set_number(info_target, t::Y, 0.0);
        ui.set_number(info_target, t::WRAPWIDTH, iw);
        // The lock line and "Empty" (+0xac, +0xb0): centred, as wide as the
        // panel, 65 down, transparent and hidden, `sLocked` and `sEmpty` in
        // them until `update_info` fills them.
        let line = |ui: &mut Ui, setting: &str, exe: &str| -> Result<TileId, String> {
            let tile = make(ui, info, "template_justify_center_text")?;
            bright(ui, tile, 0);
            ui.set_number(tile, t::X, iw / 2.0);
            ui.set_number(tile, t::WRAPWIDTH, iw);
            ui.set_number(tile, t::Y, 65.0);
            ui.set_number(tile, t::ALPHA, 0.0);
            let s = ui.setting_text(setting).unwrap_or_else(|| exe.into());
            ui.set_string(tile, t::STRING, &s);
            ui.set_number(tile, t::VISIBLE, 0.0);
            Ok(tile)
        };
        let info_lock = line(ui, "sLocked", "Locked")?;
        let info_empty = line(ui, "sEmpty", "Empty")?;
        // Weight and value, 60 down: the weight right-justified to 25 left
        // of the middle with "WG" at 10; the value right-justified to 30
        // from the right with "VAL" 5 right of the middle; then the
        // separator.
        let text_at = |ui: &mut Ui, template: &str, x: f32| -> Result<TileId, String> {
            let tile = make(ui, info, template)?;
            bright(ui, tile, 0);
            ui.set_number(tile, t::X, x);
            ui.set_number(tile, t::Y, 60.0);
            Ok(tile)
        };
        let info_weight = text_at(ui, "template_justify_right_text", iw / 2.0 - 25.0)?;
        let info_weight_label = text_at(ui, "template_justify_left_text", 10.0)?;
        let info_value = text_at(ui, "template_justify_right_text", iw - 30.0)?;
        let info_value_label = text_at(ui, "template_justify_left_text", iw / 2.0 + 5.0)?;
        let info_separator = make(ui, info, "template_info_seperator")?;
        bright(ui, info_separator, 2);
        for tile in [
            info_weight,
            info_weight_label,
            info_value,
            info_value_label,
            info_separator,
        ] {
            ui.set_number(tile, t::VISIBLE, 0.0);
        }

        // The sneak meter, top middle: its text (`sneak_nif`) transparent
        // until sneaking (`0076bfe0`).
        let sneak_meter = find(ui, "SneakMeter")?;
        ui.set_number(sneak_meter, t::X, (w / 2) as f32);
        ui.set_number(sneak_meter, t::Y, (sy * 2) as f32);
        ui.set_number(sneak_meter, t::LOCUS, 1.0);
        let sneak_text = ui
            .find(sneak_meter, "sneak_nif")
            .ok_or_else(|| "the HUD's SneakMeter has no sneak_nif".to_string())?;
        ui.set_number(sneak_text, t::ALPHA, 0.0);

        // The subtitles sit above Info (`0076bfe0`).
        let subtitles = find(ui, "Subtitles")?;
        let sw = ui.number(subtitles, t::WIDTH);
        ui.set_number(subtitles, t::X, (w / 2) as f32 - sw / 2.0);
        let sh = ui.number(subtitles, t::HEIGHT);
        let info_y = ui.number(info, t::Y);
        ui.set_number(subtitles, t::Y, info_y - sh - 50.0);
        ui.set_number(subtitles, t::LOCUS, 1.0);
        let subtitle_text = make(ui, subtitles, "template_justify_center_text")?;
        bright(ui, subtitle_text, 0);
        ui.set_number(subtitle_text, t::X, sw / 2.0);
        ui.set_number(subtitle_text, t::WRAPWIDTH, sw);

        // Messages, top left.
        let messages = find(ui, "Messages")?;
        ui.set_number(messages, t::X, (sx * 2 - 5) as f32);
        ui.set_number(messages, t::Y, (sy * 2 + 25) as f32);
        ui.set_number(messages, t::LOCUS, 1.0);
        let message_icon = make(ui, messages, "template_message_icon")?;
        bright(ui, message_icon, 6);
        let message_text = make(ui, messages, "template_justify_left_text")?;
        bright(ui, message_text, 0);
        ui.set_number(message_text, t::X, 130.0);
        ui.set_number(message_text, t::Y, 5.0);
        ui.set_number(message_text, t::WRAPWIDTH, 320.0);
        ui.set_number(message_text, t::DEPTH, 2.0);
        ui.set_number(message_text, t::ALPHA, 0.0);
        ui.set_number(message_text, t::VISIBLE, 1.0);
        let message_bracket = make(ui, messages, "template_message_bracket")?;
        bright(ui, message_bracket, 1);

        // The labels (`0076fb05`).
        let ammo_text = ui
            .setting_text("sInventoryAmmo")
            .unwrap_or_else(|| "AMMO".into());
        ui.set_string(ammo, t::STRING, &ammo_text);
        let ap_text = ui
            .setting_text("sActionPointsShort")
            .unwrap_or_else(|| "AP".into());
        ui.set_string(ap_label, t::STRING, &ap_text);
        let hp_text = ui
            .setting_text("sHitPointsShort")
            .unwrap_or_else(|| "HP".into());
        ui.set_string(hp_label, t::STRING, &hp_text);
        for name in NOT_FOLLOWED {
            if let Some(tile) = ui.find(menu, name) {
                ui.set_number(tile, t::VISIBLE, 0.0);
            }
        }
        // The XP meter (placed with the rest, 0076bfe0).
        let xp = XpMeter::create(ui, menu);
        // The quest reminder (0076bfe0, after the compass).
        let quest = crate::quest_text::QuestText::create(ui, menu);
        // The menu shows.
        ui.set_number(menu, t::VISIBLE, 1.0);

        Ok(Hud {
            menu,
            tiles: HudTiles {
                action_points: ap,
                ap_bracket,
                ap_meter,
                ap_label,
                ammo,
                condition_label,
                condition_meter,
                condition_background,
                condition_arrows,
                ammo_type,
                hit_points: hp,
                hp_bracket,
                hp_meter,
                hp_label,
                compass,
                compass_groups: groups,
                compass_markers,
                compass_quests,
                compass_player,
                compass_npcs,
                reticle_center,
                reticle,
                info,
                info_hotrect,
                info_pc_shortcut,
                info_xbox_button,
                info_target,
                info_lock,
                info_empty,
                info_weight,
                info_weight_label,
                info_value,
                info_value_label,
                info_separator,
                sneak_meter,
                sneak_text,
                messages,
                message_icon,
                message_text,
                message_bracket,
                subtitles,
                subtitle_text,
            },
            total_width,
            image_width,
            original_x,
            heading_trait,
            distance_trait,
            alpha_down_trait,
            last_time: None,
            compass_scroll: [0.0, 0.0, 1.0, 1.0],
            condition_alpha: None,
            condition_down: false,
            last_weapon: None,
            last_condition: None,
            ammo_label_shifted: false,
            queue: VecDeque::new(),
            message_started: None,
            anims: Animations::default(),
            xp,
            sneak_flashing: false,
            quest,
            quest_timing: crate::quest_text::Timing::default(),
            sounds: Vec::new(),
        })
    }

    /// A meter's width and place (`007748b0`): `f` clamped to 0..1; with
    /// total = trunc(`_TotalWidth`) and step = trunc(`_ImageWidth`), the
    /// width is trunc(total × f / step) × step (centred, mode 2: in
    /// steps of two), at least `minimum` steps; x = trunc(`_OriginalX`),
    /// less the width (mode 1: it grows to the left) or half of it (mode
    /// 2), plus `offset`.
    pub fn set_meter(
        &self,
        ui: &mut Ui,
        meter: TileId,
        f: f32,
        mode: i32,
        minimum: i32,
        offset: i32,
    ) {
        let f = f.clamp(0.0, 1.0);
        let total = ui.number(meter, self.total_width) as i32;
        let step = ui.number(meter, self.image_width) as i32;
        if step == 0 {
            return;
        }
        let mut width = if mode == 2 {
            ((total as f32 * f) / (2 * step) as f32) as i32 * 2 * step
        } else {
            ((total as f32 * f) / step as f32) as i32 * step
        };
        if width < step * minimum {
            width = step * minimum;
        }
        let mut x = ui.number(meter, self.original_x) as i32;
        if mode == 1 {
            x -= width;
        } else if mode == 2 {
            x -= width / 2;
        }
        ui.set_number(meter, t::WIDTH, width as f32);
        ui.set_number(meter, t::X, (x + offset) as f32);
    }

    /// Shows a message (`00775380`): straight away if none is showing,
    /// else after the others. `icon` none: the neutral Vault Boy.
    pub fn queue_message(&mut self, ui: &mut Ui, text: &str, icon: Option<&str>, seconds: f32) {
        let message = Message {
            text: text.to_string(),
            icon: icon.unwrap_or(MESSAGE_ICONS[0]).to_string(),
            seconds,
        };
        if self.queue.is_empty() {
            ui.set_string(self.tiles.message_text, t::STRING, &message.text);
            ui.set_string(self.tiles.message_icon, t::FILENAME, &message.icon);
        }
        self.queue.push_back(message);
    }

    /// Queues a quest's name or custom text for the quest reminder
    /// (`crate::quest_text::QuestText::queue`); `now` in seconds.
    pub fn queue_quest(&mut self, notice: crate::quest_text::Notice, now: f32) {
        if let Some(q) = &mut self.quest {
            q.queue(notice, f64::from(now) * 1000.0);
        }
    }

    /// Queues an objective line for the quest reminder.
    pub fn queue_objective(&mut self, objective: crate::quest_text::Objective, now: f32) {
        if let Some(q) = &mut self.quest {
            q.queue_objective(objective, f64::from(now) * 1000.0);
        }
    }

    /// Hides the pieces another menu's mask leaves out (`00771700`): each
    /// bit is one piece's `visible` (0x01 `ActionPoints` and `CNDArrows`,
    /// 0x02 `HitPoints`, 0x04 `RadiationMeter`, 0x08 `EnemyHealth`, 0x10
    /// `QuestReminder`, 0x40 `ReticleCenter`, 0x80 `SneakMeter`, 0x100
    /// `Messages`, 0x400 `Subtitles`, 0x800 `Hokeys`, 0x1000 `XPMeter`,
    /// 0x2000 `BreathMeter`, 0x4000 `Explosive_positioning_rect`, 0x10000
    /// `HardcoreMode`; without 0x01 `DDTIcon`, without 0x08 `DDTIconEnemy`
    /// too). With V.A.T.S.'s menu on top the mask is
    /// [`mask::VATS_MENU`], while it plays [`mask::VATS_PLAYBACK`]. Pieces
    /// whose bit is set keep what [`Hud::update`] made of them. What each
    /// piece showed before the first mask is kept in `saved`, for
    /// [`Hud::lift_mask`] (the game sets every piece's flag each frame from
    /// its state; here the pieces this HUD hides, `NOT_FOLLOWED`, stay
    /// hidden that way).
    pub fn apply_mask(&self, ui: &mut Ui, bits: u32, saved: &mut Masked) {
        let pieces = Self::masked_pieces();
        if saved.0.is_empty() {
            for (_, name) in pieces {
                if let Some(tile) = ui.find(self.menu, name) {
                    saved.0.push((tile, ui.number(tile, t::VISIBLE)));
                }
            }
        }
        for (bit, name) in pieces {
            if bits & bit == 0 {
                if let Some(tile) = ui.find(self.menu, name) {
                    ui.set_number(tile, t::VISIBLE, 0.0);
                }
            }
        }
    }

    /// The mask is off: the pieces show as they did before it.
    pub fn lift_mask(&self, ui: &mut Ui, saved: &mut Masked) {
        for (tile, visible) in saved.0.drain(..) {
            ui.set_number(tile, t::VISIBLE, visible);
        }
    }

    /// Each mask bit's piece (`00771700`).
    fn masked_pieces() -> [(u32, &'static str); 18] {
        [
            (0x01, "ActionPoints"),
            (0x01, "CNDArrows"),
            (0x02, "HitPoints"),
            (0x04, "RadiationMeter"),
            (0x08, "EnemyHealth"),
            (0x10, "QuestReminder"),
            (part::INFO, "Info"),
            (0x40, "ReticleCenter"),
            (0x80, "SneakMeter"),
            (0x100, "Messages"),
            (0x400, "Subtitles"),
            (0x800, "Hokeys"),
            (0x1000, "XPMeter"),
            (0x2000, "BreathMeter"),
            (0x4000, "Explosive_positioning_rect"),
            (0x10000, "HardcoreMode"),
            (0x01, "DDTIcon"),
            (0x08, "DDTIconEnemy"),
        ]
    }

    /// Experience gained, for the XP meter (the HUD's queue at +0x264).
    pub fn add_experience(&mut self, amount: i32) {
        if let Some(xp) = &mut self.xp {
            xp.add(amount);
        }
    }

    fn fade(&mut self, ui: &mut Ui, tile: TileId, from: f32, to: f32, now: f32) {
        ui.set_number(tile, t::ALPHA, from);
        self.anims
            .start(tile, t::ALPHA, from, to, MESSAGE_FADE, f64::from(now));
    }

    fn fading(&self, tile: TileId) -> bool {
        self.anims.moving(tile, t::ALPHA)
    }

    /// One frame (`00770430` and what it calls).
    pub fn update(&mut self, ui: &mut Ui, input: &HudInput) {
        let tiles = self.tiles.clone();
        let opacity = input.opacity * 255.0;
        let screen = ui.screen_size;
        let (w, h) = (screen.width() as i32, screen.height() as i32);
        let (sx, sy) = (screen.safe_x as i32, screen.safe_y as i32);

        // Traits moving (00a080d0).
        let now = input.time;
        self.anims.step(ui, f64::from(now));

        self.update_compass(ui, input, opacity);
        self.update_weapon(ui, input, opacity);

        // Placing the two halves again, and the meters.
        let hp = tiles.hit_points;
        ui.set_number(hp, t::X, (sx * 2 + 10) as f32);
        let hp_h = ui.number(hp, t::HEIGHT);
        ui.set_number(hp, t::Y, (h - sy * 2) as f32 - hp_h);
        let ap = tiles.action_points;
        let ap_w = ui.number(ap, t::WIDTH);
        ui.set_number(ap, t::X, (w - sx * 2) as f32 - ap_w + 30.0);
        let ap_h = ui.number(ap, t::HEIGHT);
        ui.set_number(ap, t::Y, (h - sy * 2) as f32 - ap_h);
        // HP (`00771540` with 0x10): health over its permanent value, at
        // least one step while alive. AP (0x0C): growing to the left.
        let f = if input.health_max != 0.0 {
            (input.health.max(0.0) / input.health_max).min(1.0)
        } else {
            0.0
        };
        self.set_meter(ui, tiles.hp_meter, f, 0, if input.dead { 0 } else { 1 }, 0);
        let f = if input.action_points_max != 0.0 {
            input.action_points.max(0.0) / input.action_points_max
        } else {
            0.0
        };
        self.set_meter(ui, tiles.ap_meter, f.min(1.0), 1, 0, 0);

        // The crosshair.
        ui.set_number(
            tiles.reticle_center,
            t::VISIBLE,
            if input.crosshair { 1.0 } else { 0.0 },
        );

        // Subtitles.
        match &input.subtitle {
            Some(line) => {
                ui.set_string(tiles.subtitle_text, t::STRING, line);
                ui.set_number(tiles.subtitle_text, t::VISIBLE, 1.0);
            }
            None => ui.set_number(tiles.subtitle_text, t::VISIBLE, 0.0),
        }

        // Messages: fade the bracket, icon and text in; after the
        // message's time, the next one, or fade out.
        if let Some(message) = self.queue.front().cloned() {
            let bracket_alpha = ui.number(tiles.message_bracket, t::ALPHA);
            if bracket_alpha == 0.0 && !self.fading(tiles.message_bracket) {
                for tile in [
                    tiles.message_bracket,
                    tiles.message_icon,
                    tiles.message_text,
                ] {
                    self.fade(ui, tile, 0.0, opacity, now);
                }
                self.message_started = None;
            }
            if !self.fading(tiles.message_bracket) {
                let started = *self.message_started.get_or_insert(now);
                if now - started > message.seconds {
                    self.queue.pop_front();
                    match self.queue.front().cloned() {
                        Some(next) => {
                            ui.set_string(tiles.message_text, t::STRING, &next.text);
                            ui.set_string(tiles.message_icon, t::FILENAME, &next.icon);
                            self.message_started = Some(now);
                        }
                        None => {
                            for tile in [
                                tiles.message_bracket,
                                tiles.message_icon,
                                tiles.message_text,
                            ] {
                                self.fade(ui, tile, opacity, 0.0, now);
                            }
                            self.message_started = None;
                        }
                    }
                }
            }
        }

        // The quest reminder (`0077a650`).
        if let Some(q) = &mut self.quest {
            let sounds = q.update(
                ui,
                &mut self.anims,
                f64::from(now),
                input.opacity,
                input.quest_gate,
                &self.quest_timing,
            );
            self.sounds.extend(sounds);
        }

        // The XP meter and "LEVEL UP" (`0077c4e0`, after the first 200
        // ms of the HUD).
        if let (Some(xp), Some(e)) = (&mut self.xp, &input.experience) {
            let label = ui.setting_text("sStatsXP").unwrap_or_else(|| "XP".into());
            let level_up = ui
                .setting_text("sLevelUp")
                .unwrap_or_else(|| "LEVEL UP".into());
            xp.update(
                ui,
                &mut self.anims,
                e,
                input.menu_open,
                opacity,
                f64::from(now),
                &label,
                &level_up,
            );
        }
        ui.refresh();
    }

    /// The XP meter's place under the game's menus (`00771700`, run when the
    /// menus change; which pieces show is [`Hud::apply_mask`] with
    /// [`parts_for_menu`]): in the dialogue menu moved up 330 from where it
    /// was placed (its `user3`; `01073488`), back down otherwise.
    pub fn place_xp_meter(&self, ui: &mut Ui, dialogue: bool) {
        if let Some(xp) = &self.xp {
            let placed = ui.number(xp.tiles.meter, t::USER0 + 3);
            let y = if dialogue { placed - 330.0 } else { placed };
            if ui.number(xp.tiles.meter, t::Y) != y {
                ui.set_number(xp.tiles.meter, t::Y, y);
            }
        }
    }

    /// The ammunition count and the weapon's condition (`007721c0`).
    fn update_weapon(&mut self, ui: &mut Ui, input: &HudInput, opacity: f32) {
        let tiles = &self.tiles;
        let pieces = [
            Some(tiles.ammo),
            Some(tiles.condition_meter),
            Some(tiles.condition_background),
            Some(tiles.condition_label),
            tiles.condition_arrows,
        ];
        let Some(weapon) = &input.weapon else {
            for tile in pieces.into_iter().flatten() {
                ui.set_number(tile, t::VISIBLE, 0.0);
            }
            self.last_weapon = None;
            self.last_condition = None;
            return;
        };
        // The ammunition's abbreviation (`007721c0`): with ammunition
        // loaded, the label says it (or nothing) and, while it says
        // something, the condition pieces sit 50 further left (`0077f890`
        // moves the HUD's +0x4c, +0x50, +0x54 and +0x180 pieces: +0x50 is
        // the condition meter, `007748b0`; the others taken as its label,
        // background and arrows).
        let shift = |ui: &mut Ui, dx: f32| {
            for tile in [
                Some(tiles.condition_label),
                Some(tiles.condition_meter),
                Some(tiles.condition_background),
                tiles.condition_arrows,
            ]
            .into_iter()
            .flatten()
            {
                let x = ui.number(tile, t::X);
                ui.set_number(tile, t::X, x + dx);
            }
        };
        if weapon.ammo.is_some() {
            let abbrev = weapon.ammo_abbrev.as_deref().filter(|a| !a.is_empty());
            if let Some(label) = tiles.ammo_type {
                ui.set_string(label, t::STRING, abbrev.unwrap_or(""));
            }
            if abbrev.is_some() && !self.ammo_label_shifted {
                shift(ui, -50.0);
                self.ammo_label_shifted = true;
            } else if abbrev.is_none() && self.ammo_label_shifted {
                shift(ui, 50.0);
                self.ammo_label_shifted = false;
            }
        }
        match weapon.ammo {
            Some((clip, held)) => {
                ui.set_string(tiles.ammo, t::STRING, &format!("{clip}/{held}"));
                ui.set_number(tiles.ammo, t::VISIBLE, 1.0);
            }
            None => ui.set_number(tiles.ammo, t::VISIBLE, 0.0),
        }
        if self.last_weapon != Some(weapon.id) {
            // A new weapon: the pieces go back (`007721c0`, `0077f890(50)`).
            if self.ammo_label_shifted {
                shift(ui, 50.0);
                self.ammo_label_shifted = false;
            }
            for tile in [
                Some(tiles.condition_label),
                Some(tiles.condition_meter),
                Some(tiles.condition_background),
                tiles.condition_arrows,
            ]
            .into_iter()
            .flatten()
            {
                ui.set_number(tile, t::VISIBLE, 1.0);
            }
            self.last_weapon = Some(weapon.id);
        }
        // Below 25% condition the meter blinks: its alpha goes up and
        // down by 10 / 255 of the full alpha a frame.
        let percent = weapon.condition * 100.0;
        let mut alpha = self.condition_alpha.unwrap_or(opacity);
        if percent < 25.0 {
            alpha = alpha.clamp(0.0, opacity);
            if alpha == 0.0 || alpha == opacity {
                self.condition_down = !self.condition_down;
            }
            let step = opacity / 255.0 * 10.0;
            alpha = if self.condition_down {
                alpha - step
            } else {
                alpha + step
            };
        } else {
            alpha = opacity;
        }
        self.condition_alpha = Some(alpha);
        let meter = tiles.condition_meter;
        ui.set_number(meter, t::ALPHA, alpha);
        if self.last_condition != Some(percent) {
            self.set_meter(ui, meter, weapon.condition, 0, 0, 0);
            self.last_condition = Some(percent);
        }
    }

    /// The compass (`00779070`): the strip scrolled by the heading, map
    /// markers and nearby people as icons placed by their bearing.
    fn update_compass(&mut self, ui: &mut Ui, input: &HudInput, opacity: f32) {
        let heading = input.heading.rem_euclid(360.0);
        // heading / 360 less 0.15, wrapped (the recording: heading 266.39
        // gave 0.58996).
        let mut offset = heading / 360.0 - 0.15;
        if offset < 0.0 {
            offset += 1.0;
        }
        self.compass_scroll = [offset, 0.0, 1.0, 1.0];
        // Half the window less 35 (`0076fb91`), and the bearing limit 70.
        let compass_w = ui.number(self.tiles.compass, t::WIDTH);
        let half = (compass_w / 2.0) as i32 - 35;
        let limit = 70.0f32;
        let fade_from = half as f32 * 0.8 * 2.0;
        let fade_to = (half * 2) as f32;
        let place = |rel: f32| -> f32 { (2 * half) as f32 * ((rel + limit) / (2.0 * limit)) };
        let fade = |x: f32| -> f32 {
            if x <= fade_from {
                opacity
            } else {
                // From full at the 80% point to nothing at the end.
                opacity * (1.0 - (x - fade_from) / (fade_to - fade_from))
            }
        };
        // Map markers within `iMapMarkerVisibleDistance` (20000).
        let reach = 20000.0f32;
        let mut markers = input
            .markers
            .iter()
            .filter(|m| distance_sq(m.position, input.position) <= reach * reach);
        for &icon in &self.tiles.compass_markers {
            let Some(m) = markers.next() else {
                ui.set_number(icon, t::VISIBLE, 0.0);
                continue;
            };
            let file = if m.found {
                "Interface\\HUD\\glow_hud_compass_landmark_discovered.dds"
            } else {
                "Interface\\HUD\\glow_hud_compass_landmark.dds"
            };
            ui.set_string(icon, t::FILENAME, file);
            let rel = bearing(m.position, input.position, heading);
            if rel.abs() >= limit {
                ui.set_number(icon, t::VISIBLE, 0.0);
                continue;
            }
            let x = place(rel);
            ui.set_number(icon, self.heading_trait, x);
            // Squared, as the game passes it (`004a7290`, then `fabs`).
            ui.set_number(
                icon,
                self.distance_trait,
                distance_sq(m.position, input.position),
            );
            ui.set_number(icon, t::VISIBLE, 1.0);
            ui.set_number(icon, t::ALPHA, fade(x));
        }
        // People within `fSneakMaxDistance` (× `fSneakExteriorDistanceMult`
        // outdoors): ticks, red for enemies.
        let reach = if input.interior { 1500.0 } else { 3000.0 };
        let mut actors = input
            .actors
            .iter()
            .filter(|a| distance_sq(a.position, input.position) <= reach * reach);
        for &icon in &self.tiles.compass_npcs {
            let Some(a) = actors.next() else {
                ui.set_number(icon, t::VISIBLE, 0.0);
                continue;
            };
            let rel = bearing(a.position, input.position, heading);
            if rel.abs() >= limit {
                ui.set_number(icon, t::VISIBLE, 0.0);
                continue;
            }
            let x = place(rel);
            ui.set_number(icon, self.heading_trait, x + 2.0);
            ui.set_number(icon, t::VISIBLE, 1.0);
            ui.set_number(icon, t::SYSTEMCOLOR, if a.hostile { 2.0 } else { 1.0 });
            ui.set_number(icon, t::ALPHA, fade(x));
        }
        // The active quest's targets, blinking (the HUD's menu state is 1).
        let seconds = self
            .last_time
            .map_or(0.0, |last| (input.time - last).max(0.0));
        self.last_time = Some(input.time);
        crate::compass::place_quests(
            ui,
            &self.tiles.compass_quests,
            [
                self.heading_trait,
                self.distance_trait,
                self.alpha_down_trait,
            ],
            &input.quests,
            heading,
            input.position,
            half,
            opacity,
            Some(crate::compass::BlinkClock {
                now_ms: f64::from(input.time) * 1000.0,
                seconds,
            }),
        );
        if let Some(p) = self.tiles.compass_player {
            ui.set_number(p, t::VISIBLE, 0.0);
        }
    }
}

fn distance_sq(a: [f32; 3], b: [f32; 3]) -> f32 {
    (a[0] - b[0]).powi(2) + (a[1] - b[1]).powi(2) + (a[2] - b[2]).powi(2)
}

/// Where something lies on the compass (`00778b80`): its bearing from the
/// player (clockwise from north) less the player's heading, in radians:
/// above π it becomes 2π less it (the game's own mirroring), below -π
/// gains 2π; then degrees × 1.45.
pub fn bearing(target: [f32; 3], player: [f32; 3], heading_deg: f32) -> f32 {
    let dx = target[0] - player[0];
    let dy = target[1] - player[1];
    // `004b1550`: atan(dx / dy), + π when dy < 0; ±π/2 straight east or
    // west.
    let angle = if dy == 0.0 {
        if dx > 0.0 {
            std::f32::consts::FRAC_PI_2
        } else {
            -std::f32::consts::FRAC_PI_2
        }
    } else {
        let a = (dx / dy).atan();
        if dy < 0.0 {
            a + std::f32::consts::PI
        } else {
            a
        }
    };
    let mut rel = angle - heading_deg.to_radians();
    use std::f32::consts::{PI, TAU};
    if rel >= -PI {
        if rel > PI {
            rel = TAU - rel;
        }
    } else {
        rel += TAU;
    }
    rel.to_degrees() * 1.45
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tile::{Screen, SystemColors};

    /// The HUD's file, cut down to what the setup reads (the game's own
    /// files aren't part of the tests).
    const MENU: &str = r#"<menu name="HUDMainMenu"><visible>&false;</visible><locus>&true;</locus><x>0</x><y>0</y>
      <include src="HUDTemplates.xml"/>
      <rect name="ActionPoints"><id>&noglow_branch;</id><systemcolor>&hudmain;</systemcolor><width>386</width><height>127</height></rect>
      <rect name="HitPoints"><id>&noglow_branch;</id><systemcolor>&hudmain;</systemcolor><width>369</width><height>127</height></rect>
      <rect name="ReticleCenter"><id>&noglow_branch;</id><systemcolor>&hudmain;</systemcolor><width>63</width><height>75</height></rect>
      <rect name="Messages"><id>&noglow_branch;</id><systemcolor>&hudmain;</systemcolor><width>460</width><height>90</height><locus>&true;</locus></rect>
      <rect name="Subtitles"><id>&noglow_branch;</id><systemcolor>&hudmain;</systemcolor><width>800</width><height>110</height></rect>
      <rect name="Info"><id>&noglow_branch;</id><systemcolor>&hudmain;</systemcolor><width> 320 </width><height> 80 </height><rect name="justify_center_hotrect"><visible>&false;</visible><x><copy src="me()" trait="_x"/></x><text name="button_text"><string><copy src="parent()" trait="string"/></string></text><text name="PCShortcutLabel"><font>7</font><string><copy src="parent()" trait="_PCButtonText"/></string></text><image name="xbox_button"/></rect></rect>
      <image name="CNDArrows"><locus>&true;</locus><width>16</width><height>16</height></image>
      <rect name="SneakMeter"><id>&noglow_branch;</id><systemcolor>&hudmain;</systemcolor><text name="sneak_nif"><string>[HIDDEN]</string><font>1</font></text></rect>
    </menu>"#;

    const TEMPLATES: &str = r#"
      <template name="template_justify_right_text"><text name="justify_right_text"><justify>&right;</justify><font> 7 </font><x>0</x><y>0</y></text></template>
      <template name="template_justify_left_text"><text name="justify_left_text"><justify>&left;</justify><font> 7 </font><wrapwidth>2048</wrapwidth><alpha>0</alpha><visible>&false;</visible></text></template>
      <template name="template_justify_center_text"><text name="justify_center_text"><justify>&center;</justify><font> 7 </font></text></template>
      <template name="template_right_bracket"><image name="right_bracket"><depth>-1</depth><width>512</width><height>256</height></image></template>
      <template name="template_left_bracket"><image name="left_bracket"><depth>-1</depth><width>512</width><height>256</height></image></template>
      <template name="template_meter"><image name="meter"><filename> Interface\HUD\hud_tick_mark.dds </filename><tile> &true; </tile><height> 20 </height><width> 8 </width><_TotalWidth> 300 </_TotalWidth><_ImageWidth> 8 </_ImageWidth><_OriginalX> 0 </_OriginalX></image></template>
      <template name="template_meter_background"><image name="MeterBackground"><depth>-1</depth><height> 20 </height><width> 60 </width><x>90</x></image></template>
      <template name="template_reticle_center"><image name="reticle_center"><width> 64 </width><height> 64 </height><x>2</x><y>9</y></image></template>
      <template name="template_message_bracket"><image name="message_bracket"><y>-20</y><width>470</width><height>192</height><alpha> 0 </alpha></image></template>
      <template name="template_message_icon"><image name="message_icon"><x>-00</x><y>-25</y><depth>2</depth><width> 160 </width><height> 160 </height><alpha> 0 </alpha></image></template>
      <template name="template_compass_window"><image name="compass_window"><locus> &true; </locus><width> 345 </width><height> 64 </height><depth>1</depth><zoom> 100 </zoom><x>20</x><y>65</y></image></template>
      <template name="template_compass_icon_group"><rect name="compass_icon_group"><locus>&true;</locus><y>5</y></rect></template>
      <template name="template_compass_icon_quest"><image name="compass_icon"><depth>5</depth><locus>&true;</locus><width>40</width><height>50</height><x><copy src="me()" trait="_Heading"/></x><_Heading>0</_Heading><y>5</y></image></template>
      <template name="template_compass_icon_player"><image name="compass_icon"><x><copy src="me()" trait="_Heading"/></x><_Heading>0</_Heading><y>5</y></image></template>
      <template name="template_compass_icon_marker"><image name="compass_icon"><x><copy src="me()" trait="_Heading"/></x><_Heading>0</_Heading><y>5</y></image></template>
      <template name="template_info_seperator"><image name="info_seperator"><width>300</width><height>8</height><y>55</y></image></template>
      <template name="compass_npc_icon"><image name="compass_npc_icon"><x><copy src="me()" trait="_Heading"/></x><_Heading>0</_Heading><depth>-10</depth><width>32</width><height>32</height><y>-10</y></image></template>
    "#;

    fn hud() -> (Ui, Hud) {
        let mut ui = Ui::new(
            Screen {
                width_px: 1920,
                height_px: 1080,
                safe_x: 15.0,
                safe_y: 15.0,
            },
            SystemColors::new(None, None),
            Box::new(|n| match n {
                "sHitPointsShort" => Some("HP".into()),
                "sActionPointsShort" => Some("AP".into()),
                _ => None,
            }),
        );
        let hud = load(&mut ui, &mut |p| match p {
            MENU_FILE => Some(MENU.as_bytes().to_vec()),
            "menus\\prefabs\\HUDTemplates.xml" => Some(TEMPLATES.as_bytes().to_vec()),
            _ => None,
        })
        .unwrap();
        (ui, hud)
    }

    /// Positions from the recorded frame of the game's HUD
    /// (`ui_frames\hud_draws.txt`): HitPoints at 40, 803; its bracket
    /// there; its meter at 60, 844; the compass at 60, 868; "HP" at 93,
    /// 811; the AP bracket at 1283, 803; its meter at 1350, 844 (grown
    /// left); "AP" at 1635, 811; "CND" at 1400, 883; the ammunition at
    /// 1635, 883; the condition background at 1410, 886; the crosshair at
    /// 820.5, 448.5; the message bracket at 25, 35.
    #[test]
    fn the_recorded_layout() {
        let (mut ui, mut hud) = hud();
        let input = HudInput {
            health: 100.0,
            health_max: 100.0,
            action_points: 80.0,
            action_points_max: 80.0,
            weapon: Some(WeaponState {
                id: 1,
                ammo: Some((2, 8)),
                ammo_abbrev: None,
                condition: 11.0 / 60.0,
            }),
            opacity: 1.0,
            crosshair: true,
            heading: 266.39,
            ..HudInput::default()
        };
        hud.update(&mut ui, &input);
        let at = |ui: &mut Ui, tile| ui.screen_position(tile);
        let tl = hud.tiles.clone();
        // Hollow points ("HP"): the label says so, the condition pieces sit
        // 50 further left (`007721c0`); standard rounds again: back.
        let mut hollow = input.clone();
        if let Some(w) = hollow.weapon.as_mut() {
            w.ammo_abbrev = Some("HP".into());
        }
        hud.update(&mut ui, &hollow);
        assert_eq!(at(&mut ui, tl.condition_label), (1350.0, 883.0));
        if let Some(label) = tl.ammo_type {
            assert_eq!(ui.string(label, t::STRING).as_deref(), Some("HP"));
            assert_eq!(at(&mut ui, label).1, 883.0);
        }
        hud.update(&mut ui, &input);
        assert_eq!(at(&mut ui, tl.hit_points), (40.0, 803.0));
        assert_eq!(at(&mut ui, tl.hp_bracket), (40.0, 803.0));
        assert_eq!(at(&mut ui, tl.hp_meter), (60.0, 844.0));
        assert_eq!(ui.number(tl.hp_meter, t::WIDTH), 296.0);
        assert_eq!(at(&mut ui, tl.compass), (60.0, 868.0));
        assert_eq!(at(&mut ui, tl.hp_label), (93.0, 811.0));
        assert_eq!(at(&mut ui, tl.ap_bracket), (1283.0, 803.0));
        assert_eq!(at(&mut ui, tl.ap_meter), (1350.0, 844.0));
        assert_eq!(at(&mut ui, tl.ap_label), (1635.0, 811.0));
        assert_eq!(at(&mut ui, tl.condition_label), (1400.0, 883.0));
        assert_eq!(at(&mut ui, tl.ammo), (1635.0, 883.0));
        assert_eq!(ui.string(tl.ammo, t::STRING).as_deref(), Some("2/8"));
        assert_eq!(at(&mut ui, tl.condition_background), (1410.0, 886.0));
        // The condition meter: 11 of 60 steps, at the background's place.
        assert_eq!(at(&mut ui, tl.condition_meter), (1410.0, 886.0));
        assert_eq!(ui.number(tl.condition_meter, t::WIDTH), 11.0);
        assert_eq!(at(&mut ui, tl.reticle), (820.5, 448.5));
        assert_eq!(at(&mut ui, tl.message_bracket), (25.0, 35.0));
        assert_eq!(at(&mut ui, tl.message_icon), (25.0, 30.0));
        assert_eq!(at(&mut ui, tl.message_text), (155.0, 60.0));
        assert!((hud.compass_scroll[0] - 0.5899626).abs() < 1e-5);
        assert_eq!(ui.string(tl.hp_label, t::STRING).as_deref(), Some("HP"));
        assert_eq!(ui.number(tl.hp_meter, t::BRIGHTNESS), 175.0);
        assert_eq!(ui.number(tl.condition_background, t::BRIGHTNESS), 100.0);
        assert_eq!(ui.number(tl.ap_bracket, t::BRIGHTNESS), 255.0);
    }

    #[test]
    fn the_menus_can_be_shared_between_threads() {
        // A game engine keeps them in its world (Bevy's resources must be
        // Send and Sync).
        fn shareable<T: Send + Sync>() {}
        shareable::<Ui>();
        shareable::<Hud>();
    }

    #[test]
    fn parts_not_followed_stay_hidden() {
        let (mut ui, mut hud) = hud();
        hud.update(
            &mut ui,
            &HudInput {
                opacity: 1.0,
                ..HudInput::default()
            },
        );
        // The sneak meter's text is there but transparent until sneaking.
        let sneak = ui.find(hud.menu, "sneak_nif").unwrap();
        assert_eq!(ui.number(sneak, t::ALPHA), 0.0);
        assert!(ui.shown(hud.tiles.hp_label));
    }

    #[test]
    fn vats_leaves_only_the_messages() {
        let (mut ui, mut hud) = hud();
        hud.update(
            &mut ui,
            &HudInput {
                opacity: 1.0,
                crosshair: true,
                ..HudInput::default()
            },
        );
        assert!(ui.shown(hud.tiles.hp_label));
        assert!(ui.shown(hud.tiles.reticle));
        let cnd_arrows = hud.tiles.condition_arrows;
        let arrows_before = cnd_arrows.map(|a| ui.number(a, t::VISIBLE));
        let mut saved = Masked::default();
        hud.apply_mask(&mut ui, mask::VATS_MENU, &mut saved);
        assert!(!ui.shown(hud.tiles.hp_label));
        assert!(!ui.shown(hud.tiles.ap_label));
        assert!(!ui.shown(hud.tiles.reticle));
        assert!(ui.shown(hud.tiles.messages));
        // A second frame keeps what was saved the first time.
        hud.apply_mask(&mut ui, mask::VATS_PLAYBACK, &mut saved);
        // Lifted: everything as before, the pieces this HUD hides too.
        hud.lift_mask(&mut ui, &mut saved);
        assert!(ui.shown(hud.tiles.hp_label));
        assert!(ui.shown(hud.tiles.reticle));
        assert_eq!(cnd_arrows.map(|a| ui.number(a, t::VISIBLE)), arrows_before);
        // The sneak meter is masked like the rest (bit 0x80) and back.
        hud.update_sneak(&mut ui, Some(SneakMeterState::Hidden), 1.0, 0.0);
        hud.apply_mask(&mut ui, mask::VATS_MENU, &mut saved);
        assert!(!ui.shown(hud.tiles.sneak_meter));
        hud.lift_mask(&mut ui, &mut saved);
        assert!(ui.shown(hud.tiles.sneak_meter));
    }

    #[test]
    fn meters_and_dying() {
        let (mut ui, mut hud) = hud();
        let mut input = HudInput {
            health: 1.0,
            health_max: 100.0,
            action_points: 40.0,
            action_points_max: 80.0,
            opacity: 1.0,
            ..HudInput::default()
        };
        hud.update(&mut ui, &input);
        let m = hud.tiles.hp_meter;
        // 3 of 300 is under one step: one step while alive.
        assert_eq!(ui.number(m, t::WIDTH), 8.0);
        let ap = hud.tiles.ap_meter;
        // Half of 300 in steps of 8: 144, grown left from 326.
        assert_eq!(ui.number(ap, t::WIDTH), 144.0);
        assert_eq!(ui.number(ap, t::X), 182.0);
        input.dead = true;
        input.health = 0.0;
        hud.update(&mut ui, &input);
        assert_eq!(ui.number(m, t::WIDTH), 0.0);
        // No weapon: no ammunition or condition.
        assert_eq!(ui.number(hud.tiles.ammo, t::VISIBLE), 0.0);
    }

    /// `0070c4a0` / `00771700`: the pieces each menu leaves on screen.
    #[test]
    fn menus_leave_some_pieces_on_screen() {
        assert_eq!(parts_for_menu(None), part::ALL);
        assert_eq!(parts_for_menu(Some(1001)), 0);
        assert_eq!(
            parts_for_menu(Some(1009)),
            part::QUEST_REMINDER | part::MESSAGES | part::XP_METER
        );
        assert_eq!(parts_for_menu(Some(1008)), part::XP_METER);
        assert_eq!(
            parts_for_menu(Some(1057)),
            part::QUEST_REMINDER | part::XP_METER
        );
        assert_eq!(parts_for_menu(Some(1014)), part::MESSAGES);
        assert_eq!(parts_for_menu(Some(1053)), 0);
        assert_eq!(parts_for_menu(Some(1027)), 0);
        assert_eq!(parts_for_menu(Some(1056)), mask::VATS_MENU);
        let (mut ui, mut hud) = hud();
        hud.update(
            &mut ui,
            &HudInput {
                opacity: 1.0,
                ..HudInput::default()
            },
        );
        let hp_before = ui.number(hud.tiles.hit_points, t::VISIBLE);
        let mut saved = Masked::default();
        hud.apply_mask(&mut ui, parts_for_menu(Some(1009)), &mut saved);
        hud.place_xp_meter(&mut ui, true);
        assert_eq!(ui.number(hud.tiles.hit_points, t::VISIBLE), 0.0);
        if let Some(xp) = &hud.xp {
            let placed = ui.number(xp.tiles.meter, t::USER0 + 3);
            assert_eq!(ui.number(xp.tiles.meter, t::Y), placed - 330.0);
        }
        hud.lift_mask(&mut ui, &mut saved);
        hud.place_xp_meter(&mut ui, false);
        assert_eq!(ui.number(hud.tiles.hit_points, t::VISIBLE), hp_before);
    }

    /// `00771700`: movement restrictions hide the main gameplay HUD; the
    /// separate rollover restriction removes only the Info tile.
    #[test]
    fn gameplay_controls_choose_native_hud_masks() {
        assert_eq!(gameplay_parts(false, false), part::ALL);
        assert_eq!(gameplay_parts(false, true), 0x1fdff);
        assert_eq!(gameplay_parts(true, false), 0x1514);
        assert_eq!(gameplay_parts(true, true), 0x1514);

        let movement = gameplay_parts(true, false);
        for hidden in [
            part::ACTION_POINTS,
            part::HIT_POINTS,
            part::RETICLE,
            part::INFO,
        ] {
            assert_eq!(movement & hidden, 0);
        }
        for visible in [part::RADIATION, part::QUEST_REMINDER, part::SUBTITLES] {
            assert_ne!(movement & visible, 0);
        }
        assert_eq!(gameplay_parts(false, true) & part::INFO, 0);
    }

    #[test]
    fn info_tile_participates_in_masks_and_is_restored() {
        let (mut ui, hud) = hud();
        let info = ui.find(hud.menu, "Info").unwrap();
        let before = ui.number(info, t::VISIBLE);
        let mut saved = Masked::default();
        hud.apply_mask(&mut ui, mask::MOVEMENT_DISABLED, &mut saved);
        assert_eq!(ui.number(info, t::VISIBLE), 0.0);
        hud.lift_mask(&mut ui, &mut saved);
        assert_eq!(ui.number(info, t::VISIBLE), before);
    }

    #[test]
    fn info_prompt_uses_native_action_target_and_shortcut_traits() {
        let (mut ui, hud) = hud();
        let prompt = InfoPrompt {
            action: Some("Talk".into()),
            target: "Doc Mitchell".into(),
            shortcut: Some("E".into()),
            ..InfoPrompt::default()
        };
        hud.update_info(&mut ui, Some(&prompt), 0.5);
        ui.refresh();
        assert_eq!(ui.number(hud.tiles.info_hotrect, t::X), 160.0);
        let action_text = ui.find(hud.tiles.info_hotrect, "button_text").unwrap();
        assert_eq!(ui.string(action_text, t::STRING).as_deref(), Some("Talk"));
        assert_eq!(
            ui.string(hud.tiles.info_pc_shortcut, t::STRING).as_deref(),
            Some("E)")
        );
        let shortcut_trait = ui.names.lookup_or_add("_PCButtonText").unwrap();
        assert_eq!(
            ui.string(hud.tiles.info_hotrect, t::STRING).as_deref(),
            Some("Talk")
        );
        assert_eq!(
            ui.string(hud.tiles.info_hotrect, shortcut_trait).as_deref(),
            Some("E)")
        );
        assert_eq!(
            ui.string(hud.tiles.info_target, t::STRING).as_deref(),
            Some("Doc Mitchell")
        );
        assert_eq!(ui.number(hud.tiles.info_pc_shortcut, t::VISIBLE), 1.0);
        assert_eq!(ui.number(hud.tiles.info_xbox_button, t::VISIBLE), 0.0);
        assert_eq!(ui.number(hud.tiles.info_target, t::ALPHA), 127.5);
        let mut owned = prompt.clone();
        owned.crime = true;
        hud.update_info(&mut ui, Some(&owned), 1.0);
        assert_eq!(ui.number(hud.tiles.info_target, t::SYSTEMCOLOR), 2.0);
        hud.update_info(&mut ui, Some(&prompt), 1.0);
        assert_eq!(ui.number(hud.tiles.info_target, t::SYSTEMCOLOR), 1.0);

        hud.update_info(&mut ui, None, 0.5);
        assert_eq!(
            ui.string(hud.tiles.info_hotrect, t::STRING).as_deref(),
            Some("")
        );
        assert_eq!(
            ui.string(hud.tiles.info_target, t::STRING).as_deref(),
            Some("")
        );
        assert_eq!(ui.number(hud.tiles.info_hotrect, t::VISIBLE), 0.0);
        assert_eq!(ui.number(hud.tiles.info_target, t::VISIBLE), 0.0);
        assert_eq!(ui.number(hud.tiles.info_pc_shortcut, t::VISIBLE), 0.0);
    }

    /// The Info panel's other lines (`0076bfe0`, `00775a00`): placed in
    /// the panel, the lock line and "Empty" at 65, weight and value at 60;
    /// shown by what's under the crosshair.
    #[test]
    fn info_lines_for_locks_empty_containers_and_items() {
        let (mut ui, hud) = hud();
        let tl = hud.tiles.clone();
        assert_eq!(ui.number(tl.info_lock, t::Y), 65.0);
        assert_eq!(ui.number(tl.info_empty, t::X), 160.0);
        assert_eq!(ui.number(tl.info_weight, t::X), 135.0);
        assert_eq!(ui.number(tl.info_weight_label, t::X), 10.0);
        assert_eq!(ui.number(tl.info_value, t::X), 290.0);
        assert_eq!(ui.number(tl.info_value_label, t::X), 165.0);
        assert_eq!(ui.number(tl.info_value, t::Y), 60.0);
        // An item: its weight and value, no lock.
        let item = InfoPrompt {
            action: Some("Take".into()),
            target: "Tin Can".into(),
            shortcut: Some("E".into()),
            weight_value: Some(["0.5".into(), "WG".into(), "2".into(), "VAL".into()]),
            ..InfoPrompt::default()
        };
        hud.update_info(&mut ui, Some(&item), 1.0);
        assert_eq!(ui.string(tl.info_weight, t::STRING).as_deref(), Some("0.5"));
        assert_eq!(
            ui.string(tl.info_value_label, t::STRING).as_deref(),
            Some("VAL")
        );
        for tile in [tl.info_weight, tl.info_value, tl.info_separator] {
            assert_eq!(ui.number(tile, t::VISIBLE), 1.0);
        }
        assert_eq!(ui.number(tl.info_lock, t::ALPHA), 0.0);
        // A locked container: the lock line shows, "Empty" doesn't.
        let locker = InfoPrompt {
            action: Some("Open".into()),
            target: "Locker".into(),
            lock: Some("[Locked - Easy]".into()),
            ..InfoPrompt::default()
        };
        hud.update_info(&mut ui, Some(&locker), 1.0);
        assert_eq!(ui.number(tl.info_weight, t::VISIBLE), 0.0);
        assert_eq!(
            ui.string(tl.info_lock, t::STRING).as_deref(),
            Some("[Locked - Easy]")
        );
        assert_eq!(ui.number(tl.info_lock, t::ALPHA), 255.0);
        assert_eq!(ui.number(tl.info_empty, t::VISIBLE), 0.0);
        // An empty one.
        let chest = InfoPrompt {
            action: Some("Open".into()),
            target: "Chest".into(),
            empty: Some("Empty".into()),
            ..InfoPrompt::default()
        };
        hud.update_info(&mut ui, Some(&chest), 1.0);
        assert_eq!(ui.number(tl.info_lock, t::ALPHA), 0.0);
        assert_eq!(ui.number(tl.info_empty, t::VISIBLE), 1.0);
        assert_eq!(ui.number(tl.info_empty, t::ALPHA), 255.0);
        // A gecko: the name, no action line.
        let gecko = InfoPrompt {
            target: "Gecko".into(),
            ..InfoPrompt::default()
        };
        hud.update_info(&mut ui, Some(&gecko), 1.0);
        assert_eq!(ui.number(tl.info_hotrect, t::VISIBLE), 0.0);
        assert_eq!(ui.number(tl.info_target, t::VISIBLE), 1.0);
    }

    /// The sneak meter (`007732d0`, `00770430`): its words by the
    /// player's state, faded in while sneaking, [DANGER] flashed first,
    /// faded out after.
    #[test]
    fn the_sneak_meter_says_hidden_detected_caution_or_danger() {
        assert_eq!(
            SneakMeterState::of(false, false, false, 0),
            SneakMeterState::Hidden
        );
        assert_eq!(
            SneakMeterState::of(false, false, false, 1),
            SneakMeterState::Detected
        );
        assert_eq!(
            SneakMeterState::of(false, false, true, 50),
            SneakMeterState::Caution
        );
        assert_eq!(
            SneakMeterState::of(true, false, false, 0),
            SneakMeterState::Danger
        );
        assert_eq!(
            SneakMeterState::of(true, true, false, 0),
            SneakMeterState::Caution
        );
        let (mut ui, mut hud) = hud();
        let text = hud.tiles.sneak_text;
        let x = ui.number(hud.tiles.sneak_meter, t::X);
        // Half the screen across (1706 menu units wide at 1920 pixels), 2 × the safe zone down.
        assert_eq!((x, ui.number(hud.tiles.sneak_meter, t::Y)), (853.0, 30.0));
        // As `Hud::update` runs them: the animations first, then the meter.
        let step = |hud: &mut Hud, ui: &mut Ui, s: Option<SneakMeterState>, now: f32| {
            hud.anims.step(ui, f64::from(now));
            hud.update_sneak(ui, s, 1.0, now);
        };
        step(&mut hud, &mut ui, Some(SneakMeterState::Hidden), 0.0);
        assert_eq!(ui.string(text, t::STRING).as_deref(), Some("[HIDDEN]"));
        assert_eq!(ui.number(text, t::SYSTEMCOLOR), 1.0);
        step(&mut hud, &mut ui, Some(SneakMeterState::Hidden), 0.25);
        assert_eq!(ui.number(text, t::ALPHA), 127.5);
        step(&mut hud, &mut ui, Some(SneakMeterState::Hidden), 0.5);
        assert_eq!(ui.number(text, t::ALPHA), 255.0);
        // Danger: red, flashing from transparent.
        step(&mut hud, &mut ui, Some(SneakMeterState::Danger), 1.0);
        assert_eq!(ui.number(text, t::SYSTEMCOLOR), 2.0);
        assert_eq!(ui.string(text, t::STRING).as_deref(), Some("[DANGER]"));
        step(&mut hud, &mut ui, Some(SneakMeterState::Danger), 1.125);
        assert_eq!(ui.number(text, t::ALPHA), 127.5);
        step(&mut hud, &mut ui, Some(SneakMeterState::Danger), 1.25);
        assert_eq!(ui.number(text, t::ALPHA), 255.0);
        // Out of danger mid-flash: back up from where it is.
        step(&mut hud, &mut ui, Some(SneakMeterState::Detected), 1.375);
        assert_eq!(ui.number(text, t::SYSTEMCOLOR), 1.0);
        step(&mut hud, &mut ui, Some(SneakMeterState::Detected), 2.0);
        assert_eq!(ui.number(text, t::ALPHA), 255.0);
        // Standing up: faded out over half a second.
        step(&mut hud, &mut ui, None, 3.0);
        step(&mut hud, &mut ui, None, 3.5);
        assert_eq!(ui.number(text, t::ALPHA), 0.0);
    }

    /// Quest icons (`00779070`): one per target, clamped to the compass's
    /// ends rather than hidden, `_Distance` squared, the rest hidden; and
    /// their blinking (`00778c20`).
    #[test]
    fn quest_targets_on_the_compass() {
        use crate::compass::CompassQuest;
        let (mut ui, mut hud) = hud();
        let mut input = HudInput {
            opacity: 1.0,
            time: 1.0,
            quests: vec![
                // Ahead, behind, and far off to the left.
                CompassQuest {
                    position: [0.0, 300.0, 0.0],
                },
                CompassQuest {
                    position: [0.0, -1000.0, 0.0],
                },
                CompassQuest {
                    position: [-5000.0, 100.0, 0.0],
                },
            ],
            ..HudInput::default()
        };
        hud.update(&mut ui, &input);
        let icons = hud.tiles.compass_quests.clone();
        let (heading_trait, alpha_down) = (hud.heading_trait, hud.alpha_down_trait);
        let heading = |ui: &mut Ui, i: usize| ui.number(icons[i], heading_trait);
        // Half the window less 35: 137; straight ahead in the middle.
        assert_eq!(heading(&mut ui, 0), 137.0);
        // Behind: the bearing's limit, the right end (274).
        assert_eq!(heading(&mut ui, 1), 274.0);
        // Far left: the left end.
        assert_eq!(heading(&mut ui, 2), 0.0);
        assert_eq!(ui.number(icons[0], hud.distance_trait), 90000.0);
        assert_eq!(
            ui.string(icons[0], t::FILENAME).as_deref(),
            Some("Interface\\HUD\\glow_hud_compass_objective_marker.dds")
        );
        for (i, &icon) in icons.iter().enumerate() {
            assert_eq!(ui.number(icon, t::VISIBLE), f32::from(i < 3), "{i}");
        }

        // Blinking: rising (state 0) from the icon's full alpha; the next
        // frame passes full and holds (3); the hold's end (0) has passed,
        // so then it falls (1).
        let near = icons[0];
        let alpha = |ui: &mut Ui| ui.number(near, t::ALPHA);
        let state = |ui: &mut Ui| ui.number(near, alpha_down);
        assert_eq!((alpha(&mut ui), state(&mut ui)), (255.0, 0.0));
        input.time = 1.0 + 1.0 / 32.0;
        hud.update(&mut ui, &input);
        assert_eq!((alpha(&mut ui), state(&mut ui)), (255.0, 3.0));
        input.time = 1.05;
        hud.update(&mut ui, &input);
        assert_eq!((alpha(&mut ui), state(&mut ui)), (255.0, 1.0));
        // 300 away (90000 squared, under 500²): falling at
        // 1000 + 500 × (90000 - 250000) / (65536 - 250000) a second.
        let speed = 1000.0 + 500.0 * (90000.0 - 250000.0) / (65536.0 - 250000.0);
        input.time = 1.1;
        hud.update(&mut ui, &input);
        assert!((alpha(&mut ui) - (255.0 - speed * 0.05)).abs() < 0.01);
        // Down to nothing: rising again, the hold ending after the pause
        // 600 - 550 × (90000 - 250000) / (65536 - 250000) ms.
        let pause = 600.0 - 550.0 * (90000.0 - 250000.0) / (65536.0 - 250000.0);
        let mut frames = 0;
        while state(&mut ui) == 1.0 {
            input.time += 0.05;
            hud.update(&mut ui, &input);
            frames += 1;
            assert!(frames < 100);
        }
        assert_eq!((alpha(&mut ui), state(&mut ui)), (0.0, 0.0));
        let bottom = input.time;
        let timer = ui.number(near, t::USER0 + 10);
        assert!((timer - (bottom * 1000.0 + pause).floor()).abs() <= 1.0);
        // It rises to full and holds there until the pause has passed.
        while state(&mut ui) != 1.0 {
            input.time += 0.01;
            hud.update(&mut ui, &input);
            if state(&mut ui) == 3.0 {
                assert_eq!(alpha(&mut ui), 255.0);
            }
        }
        assert!(input.time * 1000.0 > timer);
    }

    #[test]
    fn compass_bearings() {
        // Straight ahead, then 30° to the right while facing north.
        assert_eq!(bearing([0.0, 100.0, 0.0], [0.0; 3], 0.0), 0.0);
        let right = bearing([100.0, 100.0 * 3f32.sqrt(), 0.0], [0.0; 3], 0.0);
        assert!((right - 30.0 * 1.45).abs() < 1e-3);
        // Facing east, north lies 90° to the left.
        let left = bearing([0.0, 100.0, 0.0], [0.0; 3], 90.0);
        assert!((left + 90.0 * 1.45).abs() < 1e-3);
    }

    #[test]
    fn messages_fade_in_stay_and_go() {
        let (mut ui, mut hud) = hud();
        let mut input = HudInput {
            opacity: 1.0,
            ..HudInput::default()
        };
        hud.queue_message(&mut ui, "You have discovered Goodsprings", None, 2.0);
        assert_eq!(
            ui.string(hud.tiles.message_text, t::STRING).as_deref(),
            Some("You have discovered Goodsprings")
        );
        hud.update(&mut ui, &input);
        input.time = MESSAGE_FADE / 2.0;
        hud.update(&mut ui, &input);
        let a = ui.number(hud.tiles.message_bracket, t::ALPHA);
        assert!((a - 127.5).abs() < 1e-3);
        input.time = 1.0;
        hud.update(&mut ui, &input);
        assert_eq!(ui.number(hud.tiles.message_text, t::ALPHA), 255.0);
        input.time = 4.0;
        hud.update(&mut ui, &input);
        input.time = 5.0;
        hud.update(&mut ui, &input);
        assert_eq!(ui.number(hud.tiles.message_text, t::ALPHA), 0.0);
        assert!(hud.queue.is_empty());
    }
}

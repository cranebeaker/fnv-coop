//! V.A.T.S.'s menu (`menus\vats_menu.xml`, class `VATSMenu`, vtable
//! `010700d4` in FalloutNV.exe), laid out and filled as the game's code
//! does (`%USERPROFILE%\nv-re\findings\vats_camera_menu.md` §4–§6): the
//! setup (`007e9200`) builds its own action point, hit point (with the
//! compass) and enemy health brackets from `HUDTemplates.xml`, and each
//! frame (`007ec810`) fills them, places a label by each body part
//! (`007f6e50`, `007f1840`, `007f0ea0`, `007f07a0`), lists the queued
//! attacks (`007efa10`) and names the special attacks (`007eb920`). The
//! menu file's own pieces (the arrows, the buttons, the queue's list box)
//! are worked out from its operators as for any menu.
//!
//! Positions are menu units (960 high); `W`, `H` the screen tile's size,
//! `sx`, `sy` the safe zone.

use std::collections::HashMap;

use crate::anim::Animations;
use crate::compass::{self, CompassInput, CompassTiles};
use crate::names::t;
use crate::tile::{TileId, Ui};

/// The menu's file.
pub const MENU_FILE: &str = "menus\\vats_menu.xml";

/// `justify` values (`&left;` …).
pub mod justify {
    pub const LEFT: f32 = 1.0;
    pub const CENTER: f32 = 2.0;
    pub const RIGHT: f32 = 4.0;
    pub const UP: f32 = 5.0;
    pub const DOWN: f32 = 6.0;
}

/// The exe's own text for the game settings the menu shows (the data sets
/// none of these): what `&-sName;` gives when the plugins don't.
pub const EXE_TEXT: &[(&str, &str)] = &[
    ("sAccept", "Accept"),
    ("sReturn", "Return"),
    ("sVatsSelect", "Select"),
    ("sVatsBodyPart", "Body Part"),
    ("sVatsTarget", "Target"),
    ("sVatsAiming", "Aiming"),
    ("sActionPointsShort", "AP"),
    ("sHitPointsShort", "HP"),
    ("sVATSMessageLowAP", "Low"),
    ("sInventoryCondition", "CND"),
    ("sPCMenuHintE", "E)"),
    ("sPCMenuHintRMB", "RMB)"),
    ("sPCMenuHintR", "R)"),
    ("sPCMenuHintF", "F)"),
];

/// The pieces the code keeps (the menu object's fields).
#[derive(Debug, Clone, Default)]
pub struct VatsTiles {
    /// `ActionPoints` and `+0x48`…`+0x68`.
    pub action_points: TileId,
    pub ap_bracket: TileId,
    pub ap_meter: TileId,
    pub ap_cost: TileId,
    pub ap_label: TileId,
    pub low_ap: TileId,
    pub ammo: TileId,
    pub condition_label: TileId,
    pub condition_meter: TileId,
    pub condition_background: TileId,
    /// `HitPoints` and `+0x6c`…`+0x84`.
    pub hit_points: TileId,
    pub hp_bracket: TileId,
    pub hp_meter: TileId,
    pub hp_label: TileId,
    pub compass: CompassTiles,
    /// `EnemyHealth` and `+0x88`…`+0x9c`.
    pub enemy: TileId,
    pub enemy_background: TileId,
    pub enemy_bracket: TileId,
    pub enemy_meter: TileId,
    pub enemy_loss: TileId,
    pub enemy_loss_right: TileId,
    pub enemy_name: TileId,
    /// The menu file's own.
    pub glow_branch: TileId,
    pub accept: TileId,
    pub ret: TileId,
    pub special: [TileId; 2],
    pub queued_actions: TileId,
}

/// User traits the code sets.
#[derive(Debug, Clone, Copy, Default)]
struct Traits {
    original_x: i32,
    total_width: i32,
    image_width: i32,
    limb_name: i32,
    limb_name_visible: i32,
    meter_percent: i32,
    visible_items: i32,
}

/// One part of the target, as the menu shows it.
#[derive(Debug, Clone, PartialEq)]
pub struct PartView {
    /// The body part type (`BPND` byte 5: 0 torso, 1 head …, 14 the
    /// weapon), or −1 for a target without parts.
    pub part_type: i32,
    /// Parts in one group (same actor value) share a label's look
    /// (`007f3bf0`).
    pub group: i32,
    pub name: String,
    /// The shown chance, "%i%%" (`007f0ea0`).
    pub percent: String,
    /// The part's condition, 0–1 (`_MeterPercent`).
    pub meter: f32,
    /// Where its node is on the screen, 0–1 across and 0–1 up
    /// (`0045c670`); `None` behind the camera.
    pub screen: Option<[f32; 2]>,
}

/// The target as the menu shows it.
#[derive(Debug, Clone, PartialEq)]
pub struct TargetView {
    /// The caller's key for the target (its labels are kept per target).
    pub key: u64,
    pub name: String,
    pub is_actor: bool,
    pub alive: bool,
    /// Fighting the player (`00967090`): the bracket in system colour 2.
    pub hostile: bool,
    /// Health over its base, and the same less the damage the queued
    /// attacks and the one composed would do (`007f48e0`).
    pub health: f32,
    pub health_after: f32,
    pub parts: Vec<PartView>,
    pub selected: usize,
}

/// The weapon's pieces of the action points bracket (`007f28d0`).
#[derive(Debug, Clone, PartialEq)]
pub struct WeaponView {
    /// Rounds in the clip and the rest after the queue, "%i/%i".
    pub ammo: Option<(i32, i32)>,
    /// Condition, 0–1.
    pub condition: f32,
}

/// What the menu shows this frame.
#[derive(Debug, Clone, Default)]
pub struct VatsInput {
    /// Real seconds (the menus' animations run on the clock, not the
    /// game's time).
    pub time: f64,
    /// V.A.T.S.'s mode (1 menu, 2 ready, 3 scanning).
    pub mode: u8,
    /// The camera's turn and zoom are done (`+0xf8`): labels may show.
    pub settled: bool,
    /// A ranged weapon: parts can be chosen (else only one label shows:
    /// the head's, or the last).
    pub part_selection: bool,
    pub health: f32,
    pub health_max: f32,
    pub dead: bool,
    /// Action points left after the queue, the most, and the cost shown
    /// (the attack's, or a special's while hovered); `hovering` when a
    /// target and something to buy are under the cursor (`[011db180]`).
    pub action_points: f32,
    pub action_points_max: f32,
    pub cost: f32,
    pub hovering: bool,
    pub weapon: Option<WeaponView>,
    pub compass: CompassInput,
    pub target: Option<TargetView>,
    /// The queue's lines (`007efa10`).
    pub queue: Vec<String>,
    /// The two special attack buttons' names.
    pub specials: [Option<String>; 2],
}

/// Parts sharing a group (`007f0ea0`): for each part of type 1–12, every
/// later part in its group has its label hidden, except that a later head
/// (type 1) keeps its own and hides the earlier one's; the shown label's
/// meter is the lowest of the group's (the chances' highest is shared by
/// `world::vats::share_highest`). Returns whether each label shows and the
/// meter it shows. The W and S keys skip the hidden ones (`007f3070`).
pub fn shared_labels(parts: &[PartView]) -> (Vec<bool>, Vec<f32>) {
    let mut shown = vec![true; parts.len()];
    let mut meter: Vec<f32> = parts.iter().map(|p| p.meter).collect();
    for i in 0..parts.len() {
        if !(1..=12).contains(&parts[i].part_type) {
            continue;
        }
        for j in i + 1..parts.len() {
            if parts[j].group != parts[i].group {
                continue;
            }
            if parts[j].part_type == 1 {
                shown[i] = false;
            } else {
                shown[j] = false;
            }
            if meter[i] > meter[j] {
                meter[i] = meter[j];
            }
        }
    }
    (shown, meter)
}

/// The menu, made and kept up to date.
pub struct VatsMenu {
    pub menu: TileId,
    pub tiles: VatsTiles,
    traits: Traits,
    /// Each target's part labels.
    labels: HashMap<u64, Vec<TileId>>,
    /// Which labels' parts were on screen when placed (`+0x2c`).
    on_screen: HashMap<u64, Vec<bool>>,
    shown: Option<u64>,
    /// Labels placed since they last hid (`+0xf9`).
    placed: bool,
    queue_items: Vec<TileId>,
    list_index: i32,
    anims: Animations,
    /// The compass strip's scroll (for its `TILE1001` shader).
    pub compass_scroll: [f32; 4],
}

/// What [`VatsMenu::create`] needs: the menu's text and a file reader for
/// its prefabs.
pub fn load(
    ui: &mut Ui,
    read: &mut dyn FnMut(&str) -> Option<Vec<u8>>,
) -> Result<VatsMenu, String> {
    let text = read(MENU_FILE).ok_or_else(|| format!("{MENU_FILE} not found"))?;
    let menu = ui.load_menu(&text, read)?;
    VatsMenu::create(ui, menu)
}

/// The brightness each kind of piece gets (`00774800`): 1 and 3: 175, 2:
/// 255, 4: 44 then (no break) 100, others 255.
fn bright(ui: &mut Ui, tile: TileId, kind: i32) {
    let b = match kind {
        1 | 3 => 175.0,
        2 => 255.0,
        4 | 5 => 100.0,
        _ => 255.0,
    };
    ui.set_number(tile, t::BRIGHTNESS, b);
}

impl VatsMenu {
    /// Places and fills the pieces as `007e9200` does.
    pub fn create(ui: &mut Ui, menu: TileId) -> Result<VatsMenu, String> {
        let screen = ui.screen_size;
        let w = screen.width() as i32;
        let h = screen.height() as i32;
        let sx = screen.safe_x as i32;
        let sy = screen.safe_y as i32;
        let name = |ui: &mut Ui, n: &str| ui.names.lookup_or_add(n).unwrap_or(0);
        let traits = Traits {
            original_x: name(ui, "_OriginalX"),
            total_width: name(ui, "_TotalWidth"),
            image_width: name(ui, "_ImageWidth"),
            limb_name: name(ui, "_LimbName"),
            limb_name_visible: name(ui, "_LimbNameVisible"),
            meter_percent: name(ui, "_MeterPercent"),
            visible_items: name(ui, "_number_of_visible_items"),
        };
        let heading_trait = name(ui, "_Heading");
        let distance_trait = name(ui, "_Distance");
        let find = |ui: &Ui, n: &str| {
            ui.find(menu, n)
                .ok_or_else(|| format!("the V.A.T.S. menu has no {n}"))
        };
        let make = |ui: &mut Ui, parent: TileId, n: &str| {
            ui.instantiate(menu, parent, n)
                .ok_or_else(|| format!("the V.A.T.S. menu has no template {n}"))
        };
        let text = |ui: &Ui, n: &str, default: &str| {
            ui.setting_text(n).unwrap_or_else(|| default.to_string())
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
        ui.set_number(ap_meter, traits.original_x, 326.0);
        ui.set_number(ap_meter, t::Y, 41.0);
        ui.set_number(ap_meter, t::DEPTH, 5.0);
        bright(ui, ap_meter, 3);
        let ap_cost = make(ui, ap, "template_meter")?;
        ui.set_number(ap_cost, traits.original_x, 326.0);
        ui.set_number(ap_cost, t::Y, 41.0);
        ui.set_number(ap_cost, t::ALPHA, 128.0);
        bright(ui, ap_cost, 3);
        let ap_label = make(ui, ap, "template_justify_right_text")?;
        ui.set_number(ap_label, t::X, 315.0);
        ui.set_number(ap_label, t::Y, 8.0);
        bright(ui, ap_label, 0);
        let s = text(ui, "sActionPointsShort", "AP");
        ui.set_string(ap_label, t::STRING, &s);
        let low_ap = make(ui, ap, "template_justify_right_text")?;
        ui.set_number(low_ap, t::X, 270.0);
        ui.set_number(low_ap, t::Y, 8.0);
        bright(ui, low_ap, 0);
        let s = text(ui, "sVATSMessageLowAP", "Low");
        ui.set_string(low_ap, t::STRING, &s);
        ui.set_number(low_ap, t::VISIBLE, 0.0);
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
        let s = text(ui, "sInventoryCondition", "CND");
        ui.set_string(condition_label, t::STRING, &s);
        bright(ui, condition_label, 0);
        let condition_meter = make(ui, ap, "template_meter")?;
        ui.set_number(condition_meter, traits.original_x, 90.0);
        ui.set_number(condition_meter, t::Y, ammo_y + 3.0);
        ui.set_string(
            condition_meter,
            t::FILENAME,
            "Interface\\VATS\\vats_bar.dds",
        );
        ui.set_number(condition_meter, traits.total_width, 60.0);
        ui.set_number(condition_meter, traits.image_width, 1.0);
        ui.set_number(condition_meter, t::ZOOM, 200.0);
        bright(ui, condition_meter, 3);
        let condition_background = make(ui, ap, "template_meter_background")?;
        ui.set_number(condition_background, t::Y, ammo_y + 3.0);
        bright(ui, condition_background, 4);

        // Hit points and the compass, bottom left.
        let hp = find(ui, "HitPoints")?;
        ui.set_number(hp, t::X, (sx * 2 + 10) as f32);
        let hp_h = ui.number(hp, t::HEIGHT);
        ui.set_number(hp, t::Y, (h - sy * 2) as f32 - hp_h);
        ui.set_number(hp, t::LOCUS, 1.0);
        let hp_bracket = make(ui, hp, "template_left_bracket")?;
        bright(ui, hp_bracket, 2);
        let hp_meter = make(ui, hp, "template_meter")?;
        ui.set_number(hp_meter, traits.original_x, 20.0);
        ui.set_number(hp_meter, t::Y, 41.0);
        bright(ui, hp_meter, 3);
        let hp_label = make(ui, hp, "template_justify_right_text")?;
        ui.set_number(hp_label, t::X, 53.0);
        ui.set_number(hp_label, t::Y, 8.0);
        bright(ui, hp_label, 0);
        let s = text(ui, "sHitPointsShort", "HP");
        ui.set_string(hp_label, t::STRING, &s);
        let window = make(ui, hp, "template_compass_window")?;
        let mut groups = [0; 3];
        for (i, y) in [30.0, 30.0, 10.0].into_iter().enumerate() {
            let g = make(ui, window, "template_compass_icon_group")?;
            ui.set_number(g, t::Y, y);
            groups[i] = g;
        }
        let mut npcs = Vec::new();
        for _ in 0..10 {
            npcs.push(make(ui, groups[2], "compass_npc_icon")?);
        }
        let mut quests = Vec::new();
        for _ in 0..10 {
            quests.push(make(ui, groups[1], "template_compass_icon_quest")?);
        }
        let player = ui.instantiate(menu, groups[1], "template_compass_icon_player");
        let mut markers = Vec::new();
        for _ in 0..10 {
            markers.push(make(ui, groups[0], "template_compass_icon_marker")?);
        }

        // Enemy health, bottom middle.
        let enemy = find(ui, "EnemyHealth")?;
        let ew = ui.number(enemy, t::WIDTH);
        ui.set_number(enemy, t::X, (w / 2) as f32 - ew / 2.0);
        ui.set_number(enemy, t::Y, (h - sy * 2 - 88) as f32);
        ui.set_number(enemy, t::SYSTEMCOLOR, 2.0);
        let enemy_background = make(ui, enemy, "template_warning_background")?;
        let ew = ui.number(enemy, t::WIDTH);
        ui.set_number(enemy_background, t::WIDTH, ew);
        let eh = ui.number(enemy, t::HEIGHT);
        ui.set_number(enemy_background, t::HEIGHT, eh + 5.0);
        let enemy_bracket = make(ui, enemy, "template_enemy_health_bracket")?;
        bright(ui, enemy_bracket, 1);
        ui.set_number(enemy_bracket, t::VISIBLE, 1.0);
        let enemy_meter = make(ui, enemy, "template_meter")?;
        ui.set_number(enemy_meter, traits.original_x, 171.0);
        ui.set_number(enemy_meter, traits.total_width, 240.0);
        ui.set_number(enemy_meter, t::Y, 3.0);
        bright(ui, enemy_meter, 3);
        ui.set_number(enemy_meter, t::VISIBLE, 1.0);
        let enemy_loss = make(ui, enemy, "template_meter")?;
        ui.set_number(enemy_loss, traits.original_x, 171.0);
        ui.set_number(enemy_loss, traits.total_width, 240.0);
        ui.set_number(enemy_loss, t::Y, 3.0);
        bright(ui, enemy_loss, 3);
        ui.set_number(enemy_loss, t::VISIBLE, 0.0);
        let enemy_loss_right = make(ui, enemy_loss, "template_meter")?;
        ui.set_number(enemy_loss_right, traits.original_x, 175.0);
        ui.set_number(enemy_loss_right, traits.total_width, 240.0);
        ui.set_number(enemy_loss_right, t::Y, 3.0);
        bright(ui, enemy_loss_right, 3);
        ui.set_number(enemy_loss_right, t::VISIBLE, 0.0);
        let enemy_name = make(ui, enemy, "template_justify_center_text")?;
        ui.set_number(enemy_name, t::X, 170.0);
        ui.set_number(enemy_name, t::Y, 35.0);
        ui.set_number(enemy_name, t::WRAPWIDTH, 350.0);
        bright(ui, enemy_name, 0);

        let tiles = VatsTiles {
            action_points: ap,
            ap_bracket,
            ap_meter,
            ap_cost,
            ap_label,
            low_ap,
            ammo,
            condition_label,
            condition_meter,
            condition_background,
            hit_points: hp,
            hp_bracket,
            hp_meter,
            hp_label,
            compass: CompassTiles {
                window,
                markers,
                quests,
                player,
                npcs,
                heading_trait,
                distance_trait,
            },
            enemy,
            enemy_background,
            enemy_bracket,
            enemy_meter,
            enemy_loss,
            enemy_loss_right,
            enemy_name,
            glow_branch: find(ui, "GlowBranch")?,
            accept: find(ui, "accept_button")?,
            ret: find(ui, "return_button")?,
            special: [
                find(ui, "special_attack_button")?,
                find(ui, "special_attack_2_button")?,
            ],
            queued_actions: find(ui, "queued_actions")?,
        };
        ui.set_number(menu, t::VISIBLE, 1.0);
        ui.refresh();
        Ok(VatsMenu {
            menu,
            tiles,
            traits,
            labels: HashMap::new(),
            on_screen: HashMap::new(),
            shown: None,
            placed: false,
            queue_items: Vec::new(),
            list_index: 0,
            anims: Animations::default(),
            compass_scroll: [0.0, 0.0, 1.0, 1.0],
        })
    }

    /// Two meters as one (`00774b00`): `a` shows `f1`, `b` the rest up to
    /// `f2` (its alpha pulsing). Widths in steps of `_ImageWidth` (mode 2:
    /// two steps, centred); mode 1 grows left from `_OriginalX`, `b` just
    /// left of `a`; mode 0 `b` starts at `a`'s width; mode 2 `a` centred
    /// on `_OriginalX`, `b` on its left and `b`'s first child on its right.
    /// `a` with no width gives `b` at least `minimum` steps.
    #[allow(clippy::too_many_arguments)]
    fn two_meters(
        &mut self,
        ui: &mut Ui,
        a: TileId,
        b: TileId,
        f1: f32,
        f2: f32,
        mode: i32,
        minimum: i32,
        now: f64,
    ) {
        let f1 = f1.clamp(0.0, 1.0);
        let f2 = f2.clamp(0.0, 1.0);
        let tr = self.traits;
        let total = ui.number(a, tr.total_width) as i32;
        let step = ui.number(a, tr.image_width) as i32;
        if step == 0 {
            return;
        }
        let (w1, w2) = if mode == 2 {
            (
                ((total as f32 * f1) / (2 * step) as f32) as i32 * 2 * step,
                ((total as f32 * f2) / (2 * step) as f32) as i32 * 2 * step,
            )
        } else {
            (
                ((total as f32 * f1) / step as f32) as i32 * step,
                ((total as f32 * f2) / step as f32) as i32 * step,
            )
        };
        let mut x1 = ui.number(a, tr.original_x) as i32;
        if mode == 1 {
            x1 -= w1;
        } else if mode == 2 {
            x1 -= w1 / 2;
        }
        let mut wb = w2;
        let mut xb = ui.number(b, tr.original_x) as i32;
        if mode == 1 {
            wb = w2 - w1;
            xb = x1 - wb;
            let ob = ui.number(b, tr.original_x) as i32;
            wb = wb.min(ob - xb);
        } else if mode == 0 {
            xb += w1;
        } else if mode == 2 {
            xb -= w2 / 2;
            wb = x1 - xb;
            if let Some(&right) = ui.tiles[b].children.first() {
                ui.set_number(right, t::WIDTH, wb as f32);
                ui.set_number(right, t::X, (x1 + w1) as f32);
                if !self.anims.moving(right, t::ALPHA) {
                    self.anims.pulse(right, t::ALPHA, 0.0, 255.0, 1.0, now);
                }
            }
        }
        if w1 == 0 && wb < step * minimum {
            wb = step * minimum;
        }
        ui.set_number(a, t::WIDTH, w1 as f32);
        ui.set_number(a, t::X, x1 as f32);
        ui.set_number(b, t::WIDTH, wb as f32);
        ui.set_number(b, t::X, xb as f32);
        if !self.anims.moving(b, t::ALPHA) {
            self.anims.pulse(b, t::ALPHA, 0.0, 255.0, 1.0, now);
        }
    }

    /// One meter (`007748b0`, as the HUD's).
    fn meter(&self, ui: &mut Ui, meter: TileId, f: f32, mode: i32, minimum: i32) {
        let f = f.clamp(0.0, 1.0);
        let tr = self.traits;
        let total = ui.number(meter, tr.total_width) as i32;
        let step = ui.number(meter, tr.image_width) as i32;
        if step == 0 {
            return;
        }
        let mut width = ((total as f32 * f) / step as f32) as i32 * step;
        if width < step * minimum {
            width = step * minimum;
        }
        let mut x = ui.number(meter, tr.original_x) as i32;
        if mode == 1 {
            x -= width;
        }
        ui.set_number(meter, t::WIDTH, width as f32);
        ui.set_number(meter, t::X, x as f32);
    }

    /// The labels for a target's parts (`007f6e50`: a `body_part_percent`
    /// under `GlowBranch` each, justified by part type).
    fn labels_for(&mut self, ui: &mut Ui, target: &TargetView) -> Vec<TileId> {
        if let Some(l) = self.labels.get(&target.key) {
            if l.len() == target.parts.len() {
                return l.clone();
            }
        }
        let mut out = Vec::new();
        for p in &target.parts {
            let Some(tile) = ui.instantiate(self.menu, self.tiles.glow_branch, "body_part_percent")
            else {
                continue;
            };
            let j = match p.part_type {
                0 => justify::DOWN,
                3 | 4 | 7 | 8 | 9 | 14 => justify::LEFT,
                5 | 6 | 10 | 11 | 12 => justify::RIGHT,
                _ => justify::UP,
            };
            ui.set_number(tile, t::JUSTIFY, j);
            out.push(tile);
        }
        self.labels.insert(target.key, out.clone());
        self.on_screen
            .insert(target.key, vec![false; target.parts.len()]);
        out
    }

    /// Places the labels (`007f1840`); see the findings §5.
    fn place_labels(&mut self, ui: &mut Ui, target: &TargetView, labels: &[TileId], now: f64) {
        let screen = ui.screen_size;
        let (w, h) = (screen.width(), screen.height());
        let (sx, sy) = (screen.safe_x, screen.safe_y);
        let half_w = (w / 2.0) as i32;
        let half_h = (h / 2.0) as i32;
        let min_x = (sx * 2.0 + 100.0) as i32;
        let min_y = (sy * 2.0 + 100.0) as i32;
        let max_x = (w - min_x as f32 - 50.0) as i32;
        let hp_h = ui.number(self.tiles.hp_bracket, t::HEIGHT);
        let max_y = (h - sy * 2.0 - hp_h + 50.0) as i32;
        let tr = self.traits;
        let mut seen = vec![false; labels.len()];
        let (_, meters) = shared_labels(&target.parts);
        for (i, (&tile, part)) in labels.iter().zip(&target.parts).enumerate() {
            // `_LimbName` and the meter.
            ui.set_string(tile, tr.limb_name, &part.name);
            ui.set_number(tile, tr.meter_percent, meters[i].clamp(0.0, 1.0));
            let Some([u, v]) = part.screen else {
                self.hide(ui, tile, now);
                continue;
            };
            let (x, y) = (u * w, (1.0 - v) * h);
            if x < 0.0 || x > w || y < 0.0 || y > h {
                self.hide(ui, tile, now);
                continue;
            }
            seen[i] = true;
            let (mut lx, mut ly) = match part.part_type - 1 {
                0 | 1 => (
                    x as i32,
                    (y + if y > half_h as f32 { 100.0 } else { -100.0 }) as i32,
                ),
                2..=11 | 13 => (
                    (x + if x > half_w as f32 { 100.0 } else { -100.0 }) as i32,
                    (y + if y > half_h as f32 { 50.0 } else { -50.0 }) as i32,
                ),
                _ => (x as i32, y as i32),
            };
            lx = lx.min(max_x).max(min_x);
            ly = ly.min(max_y).max(min_y);
            ui.set_number(tile, t::X, lx as f32);
            ui.set_number(tile, t::Y, ly as f32);
            if (3..=12).contains(&part.part_type) || part.part_type == 14 {
                let j = if lx < half_w {
                    justify::RIGHT
                } else {
                    justify::LEFT
                };
                ui.set_number(tile, t::JUSTIFY, j);
            }
        }
        // Pushing apart labels that overlap (up to 31 passes).
        let shown: Vec<TileId> = labels
            .iter()
            .zip(&seen)
            .filter(|(_, s)| **s)
            .map(|(t, _)| *t)
            .collect();
        let gap = (150.0f32, 100.0f32);
        let push = (gap.0 * gap.0 + gap.1 * gap.1).sqrt() / 2.0;
        let clamp = |p: [f32; 2]| {
            [
                p[0].min(max_x as f32).max(min_x as f32),
                p[1].min(max_y as f32).max(min_y as f32),
            ]
        };
        let mut passes = 0;
        loop {
            let mut moved = false;
            passes += 1;
            for i in 0..shown.len() {
                for j in i + 1..shown.len() {
                    let (a, b) = (shown[i], shown[j]);
                    let mut pa = [ui.number(a, t::X), ui.number(a, t::Y)];
                    let mut pb = [ui.number(b, t::X), ui.number(b, t::Y)];
                    if (pa[0] - pb[0]).abs() < gap.0 && (pa[1] - pb[1]).abs() < gap.1 {
                        if pa == pb {
                            pb[0] += 1.0;
                        }
                        let m = [
                            pa[0].min(pb[0]) + (pa[0] - pb[0]).abs() / 2.0,
                            pa[1].min(pb[1]) + (pa[1] - pb[1]).abs() / 2.0,
                        ];
                        let away = |p: [f32; 2]| {
                            let d = [p[0] - m[0], p[1] - m[1]];
                            let l = (d[0] * d[0] + d[1] * d[1]).sqrt();
                            if l == 0.0 {
                                m
                            } else {
                                [m[0] + d[0] / l * push, m[1] + d[1] / l * push]
                            }
                        };
                        pa = clamp(away(pa));
                        pb = clamp(away(pb));
                        ui.set_number(a, t::X, pa[0]);
                        ui.set_number(a, t::Y, pa[1]);
                        ui.set_number(b, t::X, pb[0]);
                        ui.set_number(b, t::Y, pb[1]);
                        moved = true;
                    }
                }
            }
            if !moved || passes > 30 {
                break;
            }
        }
        self.on_screen.insert(target.key, seen);
    }

    /// A label leaving the screen fades out over half a second.
    fn hide(&mut self, ui: &mut Ui, tile: TileId, now: f64) {
        self.anims.stop(tile, t::ALPHA);
        let a = ui.number(tile, t::ALPHA);
        self.anims.start(tile, t::ALPHA, a, 0.0, 0.5, now);
    }

    /// The labels' strings (`007f0ea0`) and alphas (`007f07a0`).
    fn update_labels(&mut self, ui: &mut Ui, input: &VatsInput) {
        let now = input.time;
        // Another target's labels go.
        if let Some(old) = self.shown {
            if input.target.as_ref().map(|t| t.key) != Some(old) {
                if let Some(labels) = self.labels.get(&old).cloned() {
                    for tile in labels {
                        self.anims.stop(tile, t::ALPHA);
                        ui.set_number(tile, t::ALPHA, 0.0);
                    }
                }
                self.placed = false;
            }
        }
        let Some(target) = input.target.clone() else {
            self.shown = None;
            return;
        };
        self.shown = Some(target.key);
        let labels = self.labels_for(ui, &target);
        let (shown, _) = shared_labels(&target.parts);
        for ((&tile, part), shown) in labels.iter().zip(&target.parts).zip(&shown) {
            ui.set_string(tile, t::STRING, &part.percent);
            if !shown {
                ui.set_number(tile, t::VISIBLE, 0.0);
            }
        }
        // Hidden while choosing the target, scanning, or the view moves.
        if input.mode == 1 || input.mode == 3 || (input.mode == 2 && !input.settled) {
            for &tile in &labels {
                if ui.number(tile, t::ALPHA) > 0.0 {
                    self.anims.stop(tile, t::ALPHA);
                    ui.set_number(tile, t::ALPHA, 0.0);
                }
            }
            self.placed = false;
            return;
        }
        if !self.placed {
            self.placed = true;
            self.place_labels(ui, &target, &labels, now);
            return;
        }
        let seen = self.on_screen.get(&target.key).cloned().unwrap_or_default();
        let sel = target.selected.min(target.parts.len().saturating_sub(1));
        let sel_group = target.parts.get(sel).map(|p| p.group);
        let head = target.parts.iter().position(|p| p.part_type == 1);
        let last = target.parts.len().saturating_sub(1);
        let lnv = self.traits.limb_name_visible;
        for (i, &tile) in labels.iter().enumerate() {
            let part = &target.parts[i];
            let cur = ui.number(tile, t::ALPHA);
            if !seen.get(i).copied().unwrap_or(false) {
                if self.anims.target(ui, tile, t::ALPHA) > 0.0 {
                    self.anims.start(tile, t::ALPHA, cur, 0.0, 0.5, now);
                }
            } else if input.part_selection {
                let same = i == sel || Some(part.group) == sel_group;
                if !same {
                    let goal = self.anims.target(ui, tile, t::ALPHA);
                    let restart =
                        (cur == goal && cur != 255.0) || self.anims.mode(tile, t::ALPHA) == 1;
                    if restart {
                        self.anims.start(tile, t::ALPHA, cur, 255.0, 0.2, now);
                    }
                } else if self.anims.done(tile, t::ALPHA, now) {
                    self.anims.pulse(tile, t::ALPHA, 0.0, 255.0, 1.0, now);
                }
            } else if (head == Some(i) || (head.is_none() && i == last))
                && self.anims.done(tile, t::ALPHA, now)
            {
                self.anims.pulse(tile, t::ALPHA, 0.0, 255.0, 1.0, now);
            }
            let visible = if Some(part.group) == sel_group {
                1.0
            } else {
                0.0
            };
            ui.set_number(tile, lnv, visible);
        }
    }

    /// The queue's lines (`007efa10`, `007f6fe0`): one `queued_action` each
    /// in `queued_actions`, numbered in order (`listindex`); the list as
    /// tall as `_number_of_visible_items` of them.
    fn update_queue(&mut self, ui: &mut Ui, lines: &[String]) -> bool {
        let list = self.tiles.queued_actions;
        let mut changed = false;
        while self.queue_items.len() > lines.len() {
            if let Some(tile) = self.queue_items.pop() {
                ui.set_number(tile, t::VISIBLE, 0.0);
                if let Some(p) = ui.tiles[tile].parent {
                    ui.tiles[p].children.retain(|&c| c != tile);
                }
                self.list_index -= 1;
                changed = true;
            }
        }
        while self.queue_items.len() < lines.len() {
            let Some(item) = ui.instantiate(self.menu, list, "queued_action") else {
                break;
            };
            if !ui.has(item, t::ID) {
                ui.set_number(item, t::ID, -1.0);
            }
            ui.set_number(item, t::LISTINDEX, self.list_index as f32);
            self.list_index += 1;
            if self.queue_items.is_empty() {
                let n = ui.number(list, self.traits.visible_items);
                if n > 0.0 {
                    let ih = ui.number(item, t::HEIGHT);
                    ui.set_number(list, t::HEIGHT, ih * n);
                }
            }
            self.queue_items.push(item);
            changed = true;
        }
        for (item, line) in self.queue_items.iter().zip(lines) {
            if ui.string(*item, t::STRING).as_deref() != Some(line.as_str()) {
                ui.set_string(*item, t::STRING, line);
            }
        }
        let count = ui.tiles[list].children.len() as f32;
        ui.set_number(list, t::CHILDCOUNT, count);
        changed
    }

    /// One frame (`007ec810` and what it calls).
    pub fn update(&mut self, ui: &mut Ui, input: &VatsInput) {
        let now = input.time;
        self.anims.step(ui, now);
        let tiles = self.tiles.clone();

        // Enemy health (`007ec810`).
        match input.target.as_ref().filter(|t| t.is_actor) {
            Some(target) => {
                self.two_meters(
                    ui,
                    tiles.enemy_meter,
                    tiles.enemy_loss,
                    target.health_after,
                    target.health,
                    2,
                    if target.alive { 2 } else { 0 },
                    now,
                );
                ui.set_string(tiles.enemy_name, t::STRING, &target.name);
                for tile in [
                    tiles.enemy_bracket,
                    tiles.enemy_meter,
                    tiles.enemy_loss,
                    tiles.enemy_loss_right,
                    tiles.enemy_name,
                ] {
                    ui.set_number(tile, t::VISIBLE, 1.0);
                }
                let colour = if target.hostile { 2.0 } else { 1.0 };
                for tile in [
                    tiles.enemy_bracket,
                    tiles.enemy_meter,
                    tiles.enemy_loss,
                    tiles.enemy_loss_right,
                    tiles.enemy_name,
                ] {
                    ui.set_number(tile, t::SYSTEMCOLOR, colour);
                }
            }
            None => {
                for tile in [
                    tiles.enemy_bracket,
                    tiles.enemy_meter,
                    tiles.enemy_loss,
                    tiles.enemy_loss_right,
                    tiles.enemy_name,
                ] {
                    ui.set_number(tile, t::VISIBLE, 0.0);
                }
            }
        }
        // Hit points and the compass.
        let f = if input.health_max != 0.0 {
            (input.health.max(0.0) / input.health_max).min(1.0)
        } else {
            0.0
        };
        self.meter(ui, tiles.hp_meter, f, 0, 0);
        self.compass_scroll = compass::update(ui, &tiles.compass, &input.compass);
        // Action points: what's left after the cost, and the cost pulsing
        // beside it; past what's left, the cost alone.
        let max = input.action_points_max.max(1e-6);
        let ap = input.action_points;
        if input.target.is_none() || input.cost <= ap || !input.hovering {
            self.two_meters(
                ui,
                tiles.ap_meter,
                tiles.ap_cost,
                (ap - input.cost) / max,
                ap / max,
                1,
                0,
                now,
            );
            ui.set_number(tiles.low_ap, t::VISIBLE, 0.0);
        } else {
            self.two_meters(
                ui,
                tiles.ap_meter,
                tiles.ap_cost,
                input.cost / max,
                0.0,
                1,
                0,
                now,
            );
        }
        // Ammunition and the weapon's condition (`007f28d0`).
        match &input.weapon {
            Some(wv) => {
                match wv.ammo {
                    Some((clip, rest)) => {
                        ui.set_string(tiles.ammo, t::STRING, &format!("{clip}/{rest}"));
                        ui.set_number(tiles.ammo, t::VISIBLE, 1.0);
                    }
                    None => ui.set_number(tiles.ammo, t::VISIBLE, 0.0),
                }
                for tile in [
                    tiles.condition_label,
                    tiles.condition_meter,
                    tiles.condition_background,
                ] {
                    ui.set_number(tile, t::VISIBLE, 1.0);
                }
                self.meter(ui, tiles.condition_meter, wv.condition, 0, 0);
            }
            None => {
                for tile in [
                    tiles.ammo,
                    tiles.condition_label,
                    tiles.condition_meter,
                    tiles.condition_background,
                ] {
                    ui.set_number(tile, t::VISIBLE, 0.0);
                }
            }
        }
        // The queue, and the labels placed again when it changes.
        if self.update_queue(ui, &input.queue) {
            self.placed = false;
        }
        ui.set_number(
            tiles.accept,
            t::TARGET,
            if input.queue.is_empty() { 0.0 } else { 1.0 },
        );
        // Special attacks (`007eb920`).
        for (tile, name) in tiles.special.iter().zip(&input.specials) {
            match name {
                Some(n) => {
                    ui.set_string(*tile, t::STRING, n);
                    ui.set_number(*tile, t::VISIBLE, 1.0);
                }
                None => ui.set_number(*tile, t::VISIBLE, 0.0),
            }
        }
        self.update_labels(ui, input);
        ui.refresh();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tile::{Screen, SystemColors};

    /// The menu file cut down to what the code reads, with the templates
    /// it instances (the game's own files aren't part of the tests).
    const MENU: &str = r#"<menu name="VATSMenu"><class>&VATSMenu;</class><locus>&true;</locus>
      <template name="template_justify_right_text"><text name="t"><justify>&right;</justify><font>7</font></text></template>
      <template name="template_justify_center_text"><text name="t"><justify>&center;</justify><font>7</font></text></template>
      <template name="template_right_bracket"><image name="rb"><width>512</width><height>256</height></image></template>
      <template name="template_left_bracket"><image name="lb"><width>512</width><height>256</height></image></template>
      <template name="template_meter"><image name="meter"><height>20</height><width>8</width><_TotalWidth>300</_TotalWidth><_ImageWidth>8</_ImageWidth><_OriginalX>0</_OriginalX></image></template>
      <template name="template_meter_background"><image name="mb"><height>20</height><width>60</width><x>90</x></image></template>
      <template name="template_enemy_health_bracket"><image name="eb"><width>500</width><height>64</height></image></template>
      <template name="template_warning_background"><rect name="wb"></rect></template>
      <template name="template_compass_window"><image name="cw"><width>300</width><height>40</height></image></template>
      <template name="template_compass_icon_group"><rect name="g"></rect></template>
      <template name="compass_npc_icon"><image name="n"></image></template>
      <template name="template_compass_icon_quest"><image name="q"></image></template>
      <template name="template_compass_icon_player"><image name="p"></image></template>
      <template name="template_compass_icon_marker"><image name="m"></image></template>
      <rect name="GlowBranch">
        <template name="body_part_percent"><rect name="limb_percent"><alpha>0</alpha><locus>&true;</locus></rect></template>
        <rect name="HitPoints"><width>360</width><height>127</height></rect>
        <rect name="ActionPoints"><width>386</width><height>127</height></rect>
        <rect name="EnemyHealth"><width>350</width><height>52</height><locus>&true;</locus></rect>
        <hotrect name="accept_button"></hotrect>
        <hotrect name="return_button"></hotrect>
        <hotrect name="special_attack_button"></hotrect>
        <hotrect name="special_attack_2_button"></hotrect>
        <hotrect name="queued_actions"><_number_of_visible_items>6</_number_of_visible_items></hotrect>
        <template name="queued_action"><rect name="item"><height>30</height></rect></template>
      </rect>
    </menu>"#;

    fn menu() -> (Ui, VatsMenu) {
        let mut ui = Ui::new(
            Screen {
                width_px: 1920,
                height_px: 1080,
                safe_x: 15.0,
                safe_y: 15.0,
            },
            SystemColors::new(None, None),
            Box::new(|_| None),
        );
        let m = ui.load_menu(MENU.as_bytes(), &mut |_| None).unwrap();
        let v = VatsMenu::create(&mut ui, m).unwrap();
        (ui, v)
    }

    fn part(part_type: i32, group: i32, screen: Option<[f32; 2]>) -> PartView {
        PartView {
            part_type,
            group,
            name: format!("P{part_type}"),
            percent: "50%".into(),
            meter: 1.0,
            screen,
        }
    }

    fn input(parts: Vec<PartView>, selected: usize) -> VatsInput {
        VatsInput {
            time: 10.0,
            mode: 2,
            settled: true,
            part_selection: true,
            health: 100.0,
            health_max: 200.0,
            action_points: 80.0,
            action_points_max: 80.0,
            cost: 17.0,
            target: Some(TargetView {
                key: 1,
                name: "Sunny Smiles".into(),
                is_actor: true,
                alive: true,
                hostile: false,
                health: 1.0,
                health_after: 0.6,
                parts,
                selected,
            }),
            ..VatsInput::default()
        }
    }

    #[test]
    fn the_brackets_are_placed_as_the_setup_places_them() {
        let (mut ui, v) = menu();
        // W = 1706.67 → 1706, sx = sy = 15: AP at 1706 − 30 − 386 + 30.
        assert_eq!(ui.number(v.tiles.action_points, t::X), 1320.0);
        assert_eq!(ui.number(v.tiles.action_points, t::Y), 960.0 - 30.0 - 127.0);
        assert_eq!(ui.number(v.tiles.hit_points, t::X), 40.0);
        // Enemy health: centred, 88 above the bottom of the safe zone.
        assert_eq!(ui.number(v.tiles.enemy, t::X), 853.0 - 175.0);
        assert_eq!(ui.number(v.tiles.enemy, t::Y), 960.0 - 30.0 - 88.0);
        assert_eq!(ui.number(v.tiles.ap_meter, v.traits.original_x), 326.0);
        assert_eq!(ui.number(v.tiles.ap_cost, t::ALPHA), 128.0);
    }

    #[test]
    fn the_action_point_meter_shows_whats_left_and_the_cost_beside_it() {
        let (mut ui, mut v) = menu();
        v.update(&mut ui, &input(vec![part(0, 26, Some([0.5, 0.5]))], 0));
        // 300 wide in steps of 8: (80 − 17)/80 → 236, the full 80 → 296;
        // the left part grows left from 326, the cost (60) just left of it.
        assert_eq!(ui.number(v.tiles.ap_meter, t::WIDTH), 232.0);
        assert_eq!(ui.number(v.tiles.ap_meter, t::X), 326.0 - 232.0);
        assert_eq!(ui.number(v.tiles.ap_cost, t::WIDTH), 296.0 - 232.0);
        assert_eq!(ui.number(v.tiles.ap_cost, t::X), 94.0 - 64.0);
        // The cost pulses.
        assert_eq!(v.anims.mode(v.tiles.ap_cost, t::ALPHA), 1);
    }

    #[test]
    fn labels_wait_for_the_view_then_sit_by_their_parts_without_overlapping() {
        let (mut ui, mut v) = menu();
        let parts = vec![
            part(1, 25, Some([0.5, 0.7])),
            part(0, 26, Some([0.5, 0.5])),
            part(3, 27, Some([0.48, 0.55])),
            part(5, 28, Some([1.5, 0.5])),
        ];
        let mut i = input(parts, 1);
        i.settled = false;
        v.update(&mut ui, &i);
        let labels = v.labels[&1].clone();
        assert_eq!(labels.len(), 4);
        assert!(labels.iter().all(|&l| ui.number(l, t::ALPHA) == 0.0));
        // Settled: placed (head above the middle → 100 up; the torso as it
        // is, the left arm 100 left and 50 up).
        i.settled = true;
        v.update(&mut ui, &i);
        let w = ui.screen_size.width();
        let head = (ui.number(labels[0], t::X), ui.number(labels[0], t::Y));
        assert_eq!(head.0, (0.5 * w) as i32 as f32);
        assert_eq!(head.1, 188.0);
        // The torso and the arm were within 150 × 100 of each other: pushed
        // apart.
        let torso = [ui.number(labels[1], t::X), ui.number(labels[1], t::Y)];
        let arm = [ui.number(labels[2], t::X), ui.number(labels[2], t::Y)];
        assert!(
            (torso[0] - arm[0]).abs() >= 150.0 || (torso[1] - arm[1]).abs() >= 100.0,
            "{torso:?} {arm:?}"
        );
        // The arm on the left of the middle points right.
        assert_eq!(ui.number(labels[2], t::JUSTIFY), justify::RIGHT);
        // Off screen: not shown.
        assert!(!v.on_screen[&1][3]);
        // Next frame: the selected torso pulses, the others fade in.
        i.time += 0.1;
        v.update(&mut ui, &i);
        assert_eq!(v.anims.mode(labels[1], t::ALPHA), 1);
        assert!(v.anims.moving(labels[0], t::ALPHA));
        assert_eq!(v.anims.target(&mut ui, labels[0], t::ALPHA), 255.0);
        assert!(!v.anims.moving(labels[3], t::ALPHA));
        // The limb name shows only for the selected part's group.
        let lnv = v.traits.limb_name_visible;
        assert_eq!(ui.number(labels[1], lnv), 1.0);
        assert_eq!(ui.number(labels[0], lnv), 0.0);
    }

    #[test]
    fn parts_of_one_group_show_one_label_with_the_lowest_meter() {
        // Two arms in one group (27): the later one's label goes and the
        // first shows the lower meter. A later head (type 1) in the head's
        // group keeps its own label and hides the earlier one.
        let mut parts = vec![
            part(3, 27, None),
            part(4, 27, None),
            part(2, 25, None),
            part(1, 25, None),
            part(0, 26, None),
        ];
        parts[0].meter = 0.9;
        parts[1].meter = 0.4;
        let (shown, meters) = shared_labels(&parts);
        assert_eq!(shown, vec![true, false, false, true, true]);
        assert_eq!(meters[0], 0.4);
        // In the menu: hidden labels.
        let (mut ui, mut v) = menu();
        v.update(&mut ui, &input(parts, 4));
        let labels = v.labels[&1].clone();
        assert_eq!(ui.number(labels[1], t::VISIBLE), 0.0);
        assert_eq!(ui.number(labels[2], t::VISIBLE), 0.0);
        assert!(!ui.has(labels[0], t::VISIBLE) || ui.number(labels[0], t::VISIBLE) != 0.0);
    }

    #[test]
    fn the_queue_lists_each_attack_and_the_accept_button_wakes() {
        let (mut ui, mut v) = menu();
        let mut i = input(vec![part(0, 26, Some([0.5, 0.5]))], 0);
        v.update(&mut ui, &i);
        assert_eq!(ui.number(v.tiles.accept, t::TARGET), 0.0);
        i.queue = vec!["Sunny Smiles: Torso".into(), "Sunny Smiles: Head".into()];
        v.update(&mut ui, &i);
        assert_eq!(v.queue_items.len(), 2);
        assert_eq!(
            ui.string(v.queue_items[1], t::STRING).as_deref(),
            Some("Sunny Smiles: Head")
        );
        assert_eq!(ui.number(v.queue_items[1], t::LISTINDEX), 1.0);
        // Six lines tall.
        assert_eq!(ui.number(v.tiles.queued_actions, t::HEIGHT), 180.0);
        assert_eq!(ui.number(v.tiles.accept, t::TARGET), 1.0);
        i.queue.pop();
        v.update(&mut ui, &i);
        assert_eq!(v.queue_items.len(), 1);
    }
}

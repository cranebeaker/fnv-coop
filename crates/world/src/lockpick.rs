//! The lockpicking game, read from the game's code
//! (`%USERPROFILE%\nv-re\findings\lockpick.md`): the menu `LockPickMenu`
//! (class 1014, vtable `0107439c`), opened on a locked door or container
//! the player has no key for (`005180b0` → `0078db00`), drawn from
//! `menus\lockpick_menu.xml` with two models, `meshes\terminals\
//! LockInterface01.NIF` (the lock, its frame and the screwdriver) and
//! `BobbyPin01.NIF` (the pin).
//!
//! The sweet spot (`00791000`): the pick's position runs across the menu's
//! `LPM_DetectMeter` (the main rectangle's width: the screen less four
//! safe zones), 0 at the left, its width at the right. Around a random
//! centre lie six rings, the innermost the sweet spot itself: its length
//! and the rings' spacing come from `iSweetSpotLength{Min,Max}<Level>` and
//! `iConcentricLength{Min,Max}<Level>`, mixed by how far the player's
//! Lockpick is above the lock's requirement (half way to the next level's
//! requirement gives the maximum). Where the pick sits says how far the
//! lock turns (`007908f0`): 90° in the sweet spot, 15° less per ring out,
//! and outside the rings falling from 15° to 0 at the far end.
//!
//! Turning (W, A, S or D, the movement controls) turns the cylinder 90° a
//! second (`0078eb50`, stage 3) up to that limit; at 90° the lock opens;
//! short of it the pick strains (stage 4) and loses health at
//! 1 / `fPickBreakSecs` a second until it breaks (a new pin is put in, if
//! any is left). Letting go, the cylinder springs back 90° a second.
//! Forcing (F) succeeds with a chance of Lockpick − requirement +
//! `fLockSkillBase` (in 0..100); failing breaks the lock for good (a key is
//! needed from then on; the "Ignore Broken Lock" perk entry allows one
//! more). Every number here is the exe's (`FalloutNV.esm` overrides none
//! of these settings), and each rule names the function it was read from.

use esm::{FormId, LoadOrder};

use crate::dialogue::PLAYER_REF;
use crate::scripting::{game_setting, game_setting_text, Facts, GameState};

/// The menu's file (`0078db00` loads `Data\Menus\lockpick_menu.xml`).
pub const MENU_FILE: &str = "menus\\lockpick_menu.xml";
/// The lock, its frame and the screwdriver (`0078e1c0`).
pub const LOCK_MODEL: &str = "meshes\\terminals\\lockinterface01.nif";
/// The bobby pin (`0078e1c0`).
pub const PIN_MODEL: &str = "meshes\\terminals\\bobbypin01.nif";

/// The Bobby Pin (`MISC` `0000000A`): the engine's default object
/// "BobbyPin" (`0046a370` makes it with this form ID when no plugin has
/// it; `FalloutNV.esm` names it `Lockpick`). One is used up each time a
/// pin breaks (`0078eb50`, player vtable +0x17c).
pub const BOBBY_PIN: FormId = FormId(0x0000_000A);

/// The menu's class number (`&LockPickMenu;`, `0078da90` returns 0x3F6).
pub const MENU_CLASS: u32 = 1014;

/// The Lockpick skill's actor value.
pub const LOCKPICK: u16 = crate::locks::LOCKPICK;

/// The perk entry point "Ignore Broken Lock" (32).
pub const IGNORE_BROKEN_LOCK: u8 = 32;

/// How far the cylinder turns to open the lock, and how fast it turns
/// (both 90: `01066808` and the `IMUL 0x5a` on the milliseconds in
/// `0078eb50`).
pub const FULL_TURN: f32 = 90.0;
pub const TURN_PER_SECOND: f32 = 90.0;
/// Squeaks when the cylinder passes these angles (`0101db88`, `01012638`).
pub const SQUEAK_A_AT: f32 = 30.0;
pub const SQUEAK_B_AT: f32 = 60.0;
/// The pin's turn about its pivot (`00790df0`): +90° at the meter's left
/// end, −90° at its right (`0106b160` −180, then + 90), about the lock
/// model's y axis through (0, 0, 0.6) (`01018180`).
pub const PIN_PIVOT: [f32; 3] = [0.0, 0.0, 0.6];
/// The pin model's own offset under the scene's root (`0078e1c0`:
/// translation (0, 0, −1.6) at `010744c0`, rotation the identity
/// `011a9448`).
pub const PIN_OFFSET: [f32; 3] = [0.0, 0.0, -1.6];
/// Straining shakes the cylinder by ±0.25° and the pick by ±2.5 units
/// (`010290b0`, `01018c00`), the sign turning every frame.
pub const CYLINDER_SHAKE: f32 = 0.25;
pub const PICK_SHAKE: f32 = 2.5;
/// After a pin breaks, the next goes in no sooner than a second after the
/// break's animation (`0078eb50` stage 6 waits for 1.0, `01012070`).
pub const NEW_PIN_DELAY: f32 = 1.0;
/// How the pin-break and give-up sounds fade (`00ad8da0` with 100 and 500
/// milliseconds on the tension sound, `0078eb50`).
pub const TENSION_FADE_ON_BREAK_MS: u32 = 100;
pub const TENSION_FADE_ON_RELEASE_MS: u32 = 500;

/// The six rings' numbers (`id` traits 17..22, `00791000`).
pub const FIRST_RING_ID: i32 = 17;
pub const RINGS: usize = 6;

/// The game's settings for picking locks: each `GMST` the code reads, with
/// the exe's default when the data has none (`FalloutNV.esm` has none of
/// them; names and defaults read from their static initializers).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Settings {
    /// `iSweetSpotLengthMin…` / `Max…` by difficulty (`011a0328`,
    /// `011a033c`): 400/400, 200/200, 100/100, 50/50, 25/25.
    pub sweet_spot: [(i32, i32); 5],
    /// `iConcentricLengthMin…` / `Max…` (`011a0350`, `011a0364`): 200 each.
    pub concentric: [(i32, i32); 5],
    /// The Lockpick each difficulty needs (`iLockLevelMax…`: 0, 25, 50, 75,
    /// 100), then 101 for "above very hard" (`00f90990` fills `011da28c`).
    pub required: [i32; 6],
    /// `fLockSkillBase` (10): forcing's chance is Lockpick − requirement +
    /// this, truncated (`00648d30`).
    pub skill_base: f32,
    /// `fLockpickBonusHealth` (0.01): a pin's health is 1 + Lockpick × this
    /// (`0078d930`).
    pub bonus_health: f32,
    /// `fPickBreakSecs` (1): straining takes 1 / this of health a second.
    pub break_secs: f32,
}

impl Default for Settings {
    fn default() -> Settings {
        Settings {
            sweet_spot: [(400, 400), (200, 200), (100, 100), (50, 50), (25, 25)],
            concentric: [(200, 200); 5],
            required: [0, 25, 50, 75, 100, 101],
            skill_base: 10.0,
            bonus_health: 0.01,
            break_secs: 1.0,
        }
    }
}

impl Settings {
    /// The settings as the load order gives them.
    pub fn load(order: &LoadOrder) -> Settings {
        let d = Settings::default();
        let int =
            |name: String, default: i32| game_setting(order, &name).map_or(default, |v| v as i32);
        let float = |name: &str, default: f32| game_setting(order, name).unwrap_or(default);
        let mut s = d;
        for (i, word) in crate::experience::DIFFICULTIES.iter().enumerate() {
            s.sweet_spot[i] = (
                int(format!("iSweetSpotLengthMin{word}"), d.sweet_spot[i].0),
                int(format!("iSweetSpotLengthMax{word}"), d.sweet_spot[i].1),
            );
            s.concentric[i] = (
                int(format!("iConcentricLengthMin{word}"), d.concentric[i].0),
                int(format!("iConcentricLengthMax{word}"), d.concentric[i].1),
            );
            s.required[i] = int(format!("iLockLevelMax{word}"), d.required[i]);
        }
        s.skill_base = float("fLockSkillBase", d.skill_base);
        s.bonus_health = float("fLockpickBonusHealth", d.bonus_health);
        s.break_secs = float("fPickBreakSecs", d.break_secs);
        s
    }

    /// The Lockpick a difficulty needs (index clamped to 0..4, as
    /// `0078db00` clamps it).
    pub fn needs(&self, difficulty: u8) -> i32 {
        self.required[usize::from(difficulty.min(4))]
    }

    /// How far above the requirement the skill is, as a share (0..1) of
    /// twice the step to the next level's requirement (`00791000`).
    pub fn mix(&self, difficulty: u8, skill: i32) -> f32 {
        let d = usize::from(difficulty.min(4));
        let step = self.required[d + 1] - self.required[d];
        let t = (skill - self.required[d]) as f32 / (step as f32 + step as f32);
        t.clamp(0.0, 1.0)
    }

    /// The sweet spot's length and the rings' spacing, in meter units
    /// (`00791000`: `min + (max − min) × mix`, truncated).
    pub fn lengths(&self, difficulty: u8, skill: i32) -> (i32, i32) {
        let d = usize::from(difficulty.min(4));
        let t = f64::from(self.mix(difficulty, skill));
        let lerp = |(lo, hi): (i32, i32)| (f64::from(lo) + f64::from(hi - lo) * t) as i32;
        (lerp(self.sweet_spot[d]), lerp(self.concentric[d]))
    }

    /// Forcing's chance in percent (`00648d30`): Lockpick − requirement +
    /// trunc(`fLockSkillBase`), within 0..100. The menu's "Force Lock
    /// [n%]" shows it.
    pub fn force_chance(&self, difficulty: u8, skill: i32) -> i32 {
        (skill - self.needs(difficulty) + self.skill_base as i32).clamp(0, 100)
    }

    /// A new pin's health (`0078d930`, `0078eb50` stage 6): 1 + Lockpick ×
    /// `fLockpickBonusHealth` (the skill as a float of the whole number).
    pub fn pin_health(&self, skill: i32) -> f32 {
        skill as f32 * self.bonus_health + 1.0
    }
}

/// One of the six rings round the sweet spot (`LPM_SweetSpotTemplate`
/// instanced under `LPM_DetectMeter`, `00791000`): its `id`, `x` and
/// `width` (centred on the sweet spot), `depth`, and colour (green in the
/// middle to red outside, only shown in the menu's debug mode).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Ring {
    pub id: i32,
    pub x: i32,
    pub width: f32,
    pub depth: i32,
    pub color: [f32; 3],
}

/// Where the sweet spot is (`00791000`).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Zone {
    /// The meter's width (`LPM_DetectMeter`'s, from the menu's layout).
    pub meter_width: f32,
    /// The sweet spot's centre (menu `+0x84`).
    pub center: f32,
    /// The sweet spot's length and the rings' spacing.
    pub sweet: i32,
    pub concentric: i32,
    /// sweet / 2 + concentric (menu `+0x88`): past this from the centre the
    /// pick is outside every ring.
    pub half: i32,
    pub rings: [Ring; RINGS],
}

impl Zone {
    /// Lays the zone out: the centre at `random01` (0..1) of the meter's
    /// width, kept at least `half` from either end; the first ring starts
    /// at trunc(centre − sweet / 2) and each next one `concentric / 5`
    /// further out (whole numbers, as the code divides them), each as wide
    /// as twice its distance from the centre.
    pub fn new(
        settings: &Settings,
        difficulty: u8,
        skill: i32,
        meter_width: f32,
        random01: f32,
    ) -> Zone {
        let (sweet, concentric) = settings.lengths(difficulty, skill);
        let half = sweet / 2 + concentric;
        let mut center = (f64::from(random01) * f64::from(meter_width)) as f32;
        center = center.max(half as f32);
        center = center.min(meter_width - half as f32);
        let mut x = (center - (sweet / 2) as f32) as i32;
        let green = [0.0f32, 1.0, 0.0];
        let red = [1.0f32, 0.0, 0.0];
        let mut color = green;
        let mut rings = [Ring {
            id: 0,
            x: 0,
            width: 0.0,
            depth: 0,
            color,
        }; RINGS];
        for (i, ring) in rings.iter_mut().enumerate() {
            let w = center - x as f32;
            *ring = Ring {
                id: FIRST_RING_ID + i as i32,
                x,
                width: w + w,
                depth: 6 - i as i32,
                color,
            };
            x -= concentric / 5;
            let t = (i + 1) as f32 / 5.0;
            color = [0, 1, 2].map(|k| green[k] + (red[k] - green[k]) * t);
        }
        Zone {
            meter_width,
            center,
            sweet,
            concentric,
            half,
            rings,
        }
    }

    /// How far the lock turns with the pick at `x` (`007908f0`): outside
    /// the rings (`|x − centre| ≥ half`), 15° × (half + d − W) / (2 half −
    /// W), falling from 15° at the edge to 0 at the meter's far end;
    /// inside, the narrowest ring whose half width is more than the
    /// distance gives (1 − (id − 17) / 6) × 90°. Between the outermost
    /// ring's edge and `half` (where the whole-number spacing leaves a gap)
    /// no ring is found and the limit stays as it was: `None`.
    pub fn max_turn(&self, x: f32) -> Option<f32> {
        let d = (x - self.center).abs();
        if (self.half as f32) <= d {
            let w = f64::from(self.meter_width);
            let h = f64::from(self.half);
            let share = ((h + f64::from(d)) - w) / (h * 2.0 - w);
            return Some(share as f32 * 15.0);
        }
        let mut best = self.meter_width;
        let mut found = None;
        for ring in &self.rings {
            let half_width = (f64::from(ring.width) / 2.0) as f32;
            if half_width < best && d < half_width {
                best = half_width;
                found = Some(ring.id);
            }
        }
        found.map(|id| ((1.0 - (f64::from(id) - 17.0) / 6.0) * 90.0) as f32)
    }
}

/// The pin's turn (radians) about [`PIN_PIVOT`] for the pick at `x` on a
/// meter `width` wide (`00790df0`): (x / width × −180 + 90)°.
pub fn pin_angle(x: f32, width: f32) -> f32 {
    ((f64::from(x / width) * -180.0 + 90.0) * (std::f64::consts::PI / 180.0)) as f32
}

/// The pin model's own transform under the scene's root for the pick at
/// `x` (`00790df0`): rotation R = the turn about the model's y axis
/// (`0043f850`: rows (c, 0, −s), (0, 1, 0), (s, 0, c)), translation
/// P − R(P − t), P the pivot and t [`PIN_OFFSET`]: the pin turns about the
/// pivot. Rows of the rotation, then the translation.
pub fn pin_transform(x: f32, width: f32) -> ([[f32; 3]; 3], [f32; 3]) {
    let a = pin_angle(x, width);
    let (s, c) = a.sin_cos();
    let r = [[c, 0.0, -s], [0.0, 1.0, 0.0], [s, 0.0, c]];
    let p = PIN_PIVOT;
    let t = PIN_OFFSET;
    let d = [p[0] - t[0], p[1] - t[1], p[2] - t[2]];
    let rd = [0, 1, 2].map(|i| r[i][0] * d[0] + r[i][1] * d[1] + r[i][2] * d[2]);
    (r, [p[0] - rd[0], p[1] - rd[1], p[2] - rd[2]])
}

/// The menu's stages (menu `+0x28`, `0078eb50`'s switch).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Stage {
    /// The lock's and pin's `Forward` sequences play (the screwdriver and
    /// the pin go in).
    Entering,
    /// A new pin goes in (its `Forward`).
    NewPin,
    /// Ready: the pick moves with the mouse; a turn key starts turning.
    Ready,
    /// The cylinder turns.
    Turning,
    /// Stuck short of the full turn: the pin strains.
    Straining,
    /// The pin breaks (its `Left` sequence).
    Breaking,
    /// After a second, a new pin (or the message that none are left).
    Reset,
    /// The menu closes.
    Closing,
}

/// A sequence of one of the two models: its first and last key times
/// (`NiControllerSequence` +0x2c, +0x30).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Span {
    pub begin: f32,
    pub end: f32,
}

impl Span {
    /// The time `share` of the way through (`begin + (end − begin) ×
    /// share`, as `00790f40` and the pin's strain set it).
    pub fn at(&self, share: f32) -> f32 {
        (f64::from(self.begin) + (f64::from(self.end) - f64::from(self.begin)) * f64::from(share))
            as f32
    }
}

/// The models' sequences the menu uses (`0078e1c0`): the lock's `Forward`
/// and `Backward` (the cylinder's turn, scrubbed), the pin's `Forward`
/// (going in), `Backward` (bending, scrubbed) and `Left` (breaking). A
/// missing one skips what needs it, as in the game.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Sequences {
    pub lock_forward: Option<Span>,
    pub lock_backward: Option<Span>,
    pub pin_forward: Option<Span>,
    pub pin_backward: Option<Span>,
    pub pin_left: Option<Span>,
}

/// Which sequence a model shows.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Shows {
    Forward,
    Backward,
    Left,
}

/// What the two models show: each one's sequence and the time in it, and
/// the pick's position the pin was last turned for (the game sets these
/// as it goes: `0078eb50`, `00790df0`, `00790f40`).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Scene {
    pub lock: (Shows, f32),
    pub pin: (Shows, f32),
    pub pin_x: f32,
}

/// One frame's input (`0078eb50`).
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Frame {
    /// Milliseconds since the menu's last update (the game's timer at
    /// `011f6394` +0x14, whole milliseconds).
    pub ms: u32,
    /// The menu is the top one (`00702450`): only then does it react.
    pub active: bool,
    /// The mouse's movement across, in menu units (`007908f0`: the raw
    /// mouse movement × the menu width over the screen's pixel width).
    pub mouse: f32,
    /// A turn control went down this frame / is held (`00791540`: Left,
    /// Right, Forward or Back, controls 2, 3, 0, 1).
    pub turn_pressed: bool,
    pub turn_held: bool,
    /// The lockpicking tutorial (`world::tutorial`, id 0x14; 0x1C with a
    /// pad) hasn't been shown yet.
    pub tutorial_unseen: bool,
}

/// What the menu does as it goes, for the caller to carry out.
#[derive(Debug, Clone, PartialEq)]
pub enum Effect {
    /// Play a sound record by editor ID (`00453a70`).
    Sound(&'static str),
    /// Start the straining sound (`UILockpickingPickTensionLPM`), kept to
    /// be faded out later (`011da2b0`).
    TensionStart,
    /// Fade the straining sound out over these milliseconds (`00ad8da0`).
    TensionFade(u32),
    /// A message in the HUD's corner (`007052f0`): the game setting's text
    /// (with its default), its icon, and its sound.
    Message {
        setting: &'static str,
        default: &'static str,
        icon: &'static str,
        sound: Option<&'static str>,
    },
    /// The lock opened: [`picked`] has its consequences; the caller then
    /// uses the door or container as the player (`00573170`).
    Opened,
    /// A pin broke: one is used up ([`use_up_pin`]).
    PinBroken,
    /// Forcing failed: the lock is broken ([`break_lock`]).
    LockBroken,
    /// The menu closes.
    Close,
    /// Ask for the lockpicking tutorial (`ShowMessage(id, 1014, 0)`, the
    /// "ready" stage of `0078eb50`): the answer goes in
    /// [`Picking::tutorial_wait`].
    Tutorial,
}

/// The surprised Vault Boy (the menu's messages, `01049638`).
pub const SURPRISED_ICON: &str =
    "Interface\\Icons\\Message Icons\\glow_message_vaultboy_surprised.dds";
/// The padlock (refusals, `0102b0d0`).
pub const PADLOCK_ICON: &str = "Interface\\Icons\\Message Icons\\glow_message_padlock.dds";

/// The lockpicking menu's state (the menu object's fields, `0078d930`).
#[derive(Debug, Clone, PartialEq)]
pub struct Picking {
    /// The locked reference (`+0x6c`).
    pub reference: FormId,
    /// Its difficulty (0..4, `+0x70`) and the player's Lockpick (`+0x74`).
    pub difficulty: u8,
    pub skill: i32,
    pub settings: Settings,
    /// Bobby pins left (`LPM_PickCountDisplay`'s `_Value`).
    pub pins: i32,
    pub stage: Stage,
    /// Seconds in this stage (`011da2ac`): 0 on the first frame after a
    /// change, then counting.
    pub stage_time: f32,
    last_stage: Option<Stage>,
    pub zone: Zone,
    /// The pick's place on the meter (`LPM_DetectArrow`'s `_x`).
    pub pick_x: f32,
    /// The cylinder's turn and how far it can go now (`+0x8c`, `+0x90`).
    pub cylinder: f32,
    pub max_turn: f32,
    /// The pin's health (`+0x94`).
    pub health: f32,
    /// Forced open (`+0x98`).
    pub forced: bool,
    /// The straining shake's sign (`011a0394`, starts at 1).
    shake: i32,
    /// The last frame's pick movement (`011da2c0`).
    last_move: f32,
    pub sequences: Sequences,
    pub scene: Scene,
    /// Waiting on the tutorial asked for (`+0x99`): nothing happens until
    /// the menu is on top and the tutorial has been shown.
    pub tutorial_wait: bool,
}

/// Why the menu doesn't open (`0078db00`).
#[derive(Debug, Clone, PartialEq)]
pub enum Refusal {
    /// Lockpick below the requirement: `sLockpickSkillTooLow` with it, the
    /// padlock and `UIPopUpMessageGeneral`.
    SkillTooLow(i32),
}

impl Picking {
    /// The menu opening on `reference` (a lock of `difficulty`): with
    /// Lockpick at least the requirement (`0078db00`), `pins` bobby pins,
    /// the pick in the meter's middle (the arrow's `_x` in the file) and a
    /// new pin. `random01` places the sweet spot.
    #[allow(clippy::too_many_arguments)]
    pub fn open(
        reference: FormId,
        difficulty: u8,
        skill: i32,
        pins: i32,
        settings: Settings,
        meter_width: f32,
        random01: f32,
        sequences: Sequences,
    ) -> Result<Picking, Refusal> {
        let difficulty = difficulty.min(4);
        if skill < settings.needs(difficulty) {
            return Err(Refusal::SkillTooLow(settings.needs(difficulty)));
        }
        let zone = Zone::new(&settings, difficulty, skill, meter_width, random01);
        let lock_begin = sequences.lock_forward.map_or(0.0, |s| s.begin);
        let pin_begin = sequences.pin_forward.map_or(0.0, |s| s.begin);
        Ok(Picking {
            reference,
            difficulty,
            skill,
            settings,
            pins,
            stage: Stage::Entering,
            stage_time: 0.0,
            last_stage: None,
            zone,
            pick_x: meter_width / 2.0,
            cylinder: 0.0,
            max_turn: 0.0,
            health: settings.pin_health(skill),
            forced: false,
            shake: 1,
            last_move: 0.0,
            sequences,
            scene: Scene {
                lock: (Shows::Forward, lock_begin),
                pin: (Shows::Forward, pin_begin),
                pin_x: meter_width / 2.0,
            },
            tutorial_wait: false,
        })
    }

    /// "Force Lock [n%]" (`0078db00`: `sForceLock`, `"%s [%d"`, `"%]"`).
    pub fn force_label(&self, force_lock: &str) -> String {
        format!(
            "{force_lock} [{}%]",
            self.settings.force_chance(self.difficulty, self.skill)
        )
    }

    /// The cylinder shown at `cylinder + extra` degrees (`00790f40`): the
    /// lock's `Backward` that share of the way through.
    fn show_cylinder(&mut self, extra: f32) {
        if let Some(span) = self.sequences.lock_backward {
            let share = (self.cylinder + extra) / FULL_TURN;
            self.scene.lock = (Shows::Backward, span.at(share));
        }
    }

    /// The pick moved by the mouse (`007908f0`): along the meter within its
    /// ends; a movement sound when it crosses a hundred units and a random
    /// 0..9 (`roll10`, drawn only then) comes up 0, else when it starts
    /// moving; then the lock's limit for where it is.
    fn move_pick(
        &mut self,
        mouse: f32,
        roll10: &mut dyn FnMut() -> u32,
        effects: &mut Vec<Effect>,
    ) {
        let old = self.pick_x;
        let mut x = old + mouse;
        let old_hundreds = (old as i32) % 100;
        let new_hundreds = (x as i32) % 100;
        x = x.max(0.0);
        x = x.min(self.zone.meter_width);
        let crossed = (mouse > 0.0 && new_hundreds < old_hundreds)
            || (mouse < 0.0 && old_hundreds < new_hundreds);
        if (crossed && roll10() == 0) || (mouse != 0.0 && self.last_move == 0.0) {
            effects.push(Effect::Sound("UILockpickingPickMovement"));
        }
        self.pick_x = x;
        self.last_move = mouse;
        self.scene.pin_x = x;
        if let Some(limit) = self.zone.max_turn(x) {
            self.max_turn = limit;
        }
    }

    /// One frame (`0078eb50`). `roll10` draws a random 0..9 (the pick
    /// movement sound's chance).
    pub fn update(&mut self, frame: &Frame, roll10: &mut dyn FnMut() -> u32) -> Vec<Effect> {
        let mut effects = Vec::new();
        // The stage's clock: none on the first frame, nor the first after a
        // change; then the milliseconds added up.
        match self.last_stage {
            Some(s) if s == self.stage => self.stage_time += frame.ms as f32 / 1000.0,
            _ => self.stage_time = 0.0,
        }
        let ms = if self.last_stage.is_some() {
            frame.ms
        } else {
            0
        };
        self.last_stage = Some(self.stage);
        let turned = (ms * 90) as f32;
        // Waiting on the tutorial: the stage isn't run until the menu is on
        // top again and the tutorial has been shown.
        if self.tutorial_wait {
            if !frame.active || frame.tutorial_unseen {
                return effects;
            }
            self.tutorial_wait = false;
        }
        match self.stage {
            Stage::Entering => {
                let (Some(lock), Some(pin)) =
                    (self.sequences.lock_forward, self.sequences.pin_forward)
                else {
                    return effects;
                };
                self.scene.lock = (Shows::Forward, lock.begin + self.stage_time);
                if self.stage_time != 0.0 || self.pins > 0 {
                    self.scene.pin = (Shows::Forward, pin.begin + self.stage_time);
                }
                if self.stage_time == 0.0 {
                    if self.pins > 0 {
                        effects.push(Effect::Sound("UILockpickingEnter"));
                    } else {
                        effects.push(out_of_pins());
                    }
                }
                if lock.end <= self.stage_time {
                    self.stage = Stage::Ready;
                    if let Some(b) = self.sequences.lock_backward {
                        self.scene.lock = (Shows::Backward, b.begin);
                    }
                    if let Some(b) = self.sequences.pin_backward {
                        self.scene.pin = (Shows::Backward, b.begin);
                    }
                }
            }
            Stage::NewPin => {
                let Some(pin) = self.sequences.pin_forward else {
                    return effects;
                };
                self.scene.pin = (Shows::Forward, pin.begin + self.stage_time);
                if pin.end <= self.stage_time {
                    self.stage = Stage::Ready;
                    if let Some(b) = self.sequences.pin_backward {
                        self.scene.pin = (Shows::Backward, b.begin);
                    }
                }
            }
            Stage::Ready => {
                // The tutorial first, if it hasn't been shown (asked for
                // every frame until it has).
                if frame.tutorial_unseen {
                    effects.push(Effect::Tutorial);
                    return effects;
                }
                if !frame.active {
                    return effects;
                }
                if self.pins > 0 {
                    if frame.turn_pressed {
                        effects.push(Effect::Sound("UILockpickingCylinderTurn"));
                        self.stage = Stage::Turning;
                    }
                    self.move_pick(frame.mouse, roll10, &mut effects);
                }
                if 0.0 < self.cylinder {
                    let old = self.cylinder;
                    self.cylinder = (self.cylinder - turned / 1000.0).max(0.0);
                    self.show_cylinder(0.0);
                    if self.cylinder < SQUEAK_A_AT && SQUEAK_A_AT < old {
                        effects.push(Effect::Sound("UILockpickingCylinderSqueakA"));
                    }
                    if self.cylinder < SQUEAK_B_AT && SQUEAK_B_AT < old {
                        effects.push(Effect::Sound("UILockpickingCylinderSqueakB"));
                    }
                }
            }
            Stage::Turning => {
                if !self.forced && (!frame.active || !frame.turn_held) {
                    self.stage = Stage::Ready;
                    return effects;
                }
                let old = self.cylinder;
                if self.cylinder <= self.max_turn {
                    self.cylinder = (self.cylinder + turned / 1000.0).min(self.max_turn);
                }
                self.show_cylinder(0.0);
                if old < SQUEAK_A_AT && SQUEAK_A_AT < self.cylinder {
                    effects.push(Effect::Sound("UILockpickingCylinderSqueakA"));
                }
                if old < SQUEAK_B_AT && SQUEAK_B_AT < self.cylinder {
                    effects.push(Effect::Sound("UILockpickingCylinderSqueakB"));
                }
                if self.max_turn <= self.cylinder {
                    if self.cylinder < FULL_TURN {
                        effects.push(Effect::Sound("UILockpickingCylinderStop"));
                        effects.push(Effect::TensionStart);
                        self.stage = Stage::Straining;
                    } else {
                        effects.push(Effect::Opened);
                        effects.push(Effect::Sound("UILockpickingUnlock"));
                        effects.push(Effect::Close);
                        self.stage = Stage::Closing;
                    }
                }
            }
            Stage::Straining => {
                if !frame.active || !frame.turn_held {
                    effects.push(Effect::TensionFade(TENSION_FADE_ON_RELEASE_MS));
                    self.stage = Stage::Ready;
                    return effects;
                }
                let seconds = ms as f32 / 1000.0;
                if self.settings.break_secs != 0.0 {
                    self.health -= seconds / self.settings.break_secs;
                }
                self.health = self.health.max(0.0);
                self.show_cylinder(self.shake as f32 * CYLINDER_SHAKE);
                if let Some(span) = self.sequences.pin_backward {
                    self.scene.pin = (Shows::Backward, span.at(1.0 - self.health));
                }
                self.scene.pin_x = self.pick_x + self.shake as f32 * PICK_SHAKE;
                self.shake = -self.shake;
                if self.health <= 0.0 {
                    effects.push(Effect::TensionFade(TENSION_FADE_ON_BREAK_MS));
                    effects.push(Effect::Sound("UILockpickingPickBreak"));
                    effects.push(Effect::PinBroken);
                    self.pins -= 1;
                    if let Some(left) = self.sequences.pin_left {
                        self.scene.pin = (Shows::Left, left.begin);
                    }
                    self.stage = Stage::Breaking;
                }
            }
            Stage::Breaking => {
                if let (Some(left), true) = (self.sequences.pin_left, frame.active) {
                    self.scene.pin = (Shows::Left, left.begin + self.stage_time);
                    if left.end <= self.stage_time {
                        self.stage = Stage::Reset;
                    }
                }
            }
            Stage::Reset => {
                if NEW_PIN_DELAY <= self.stage_time && frame.active {
                    self.health = self.settings.pin_health(self.skill);
                    self.cylinder = 0.0;
                    self.show_cylinder(0.0);
                    self.pick_x = self.zone.meter_width / 2.0;
                    self.scene.pin_x = self.pick_x;
                    let begin = self.sequences.pin_forward.map_or(0.0, |s| s.begin);
                    self.scene.pin = (Shows::Forward, begin);
                    if self.pins == 0 {
                        effects.push(out_of_pins());
                        self.stage = Stage::Ready;
                    } else {
                        self.stage = Stage::NewPin;
                    }
                }
            }
            Stage::Closing => {}
        }
        effects
    }

    /// The Force Lock button (id 9, F; `00790330`): `roll100` a random
    /// 0..99. Below the chance the cylinder turns all the way (stage 3 with
    /// the limit at 90°); else the lock breaks and the menu closes with
    /// `sLockBroken`. (The handler also asks the "Ignore Broken Lock" perk
    /// entry, but uses it only in the debug menu without a reference.)
    pub fn force(&mut self, roll100: u32) -> Vec<Effect> {
        let chance = self.settings.force_chance(self.difficulty, self.skill);
        if (roll100 as i64) < i64::from(chance) {
            self.forced = true;
            self.max_turn = FULL_TURN;
            self.stage = Stage::Turning;
            return Vec::new();
        }
        self.stage = Stage::Closing;
        vec![
            Effect::LockBroken,
            Effect::Message {
                setting: "sLockBroken",
                default: "The lock has been broken.",
                icon: SURPRISED_ICON,
                sound: Some("UILockpickingForceFail"),
            },
            Effect::Close,
        ]
    }

    /// The Exit button (id 10, E): the menu closes.
    pub fn exit(&mut self) -> Vec<Effect> {
        self.stage = Stage::Closing;
        vec![Effect::Close]
    }
}

/// `sOutOfLockpicks` with the surprised Vault Boy.
fn out_of_pins() -> Effect {
    Effect::Message {
        setting: "sOutOfLockpicks",
        default: "You are out of lockpicks.",
        icon: SURPRISED_ICON,
        sound: None,
    }
}

/// The player's Lockpick as the menu takes it (`0066ef20`: the current
/// value, rounded down to a whole number).
pub fn player_skill(order: &LoadOrder, state: &GameState) -> i32 {
    Facts {
        order,
        state,
        speaker: None,
    }
    .current_actor_value(PLAYER_REF, LOCKPICK)
    .unwrap_or(0.0)
    .floor() as i32
}

/// Bobby pins the player has.
pub fn pins(order: &LoadOrder, state: &GameState) -> i32 {
    state.item_count(order, PLAYER_REF, BOBBY_PIN)
}

/// A pin broke: the player loses one (player vtable +0x17c, `0078eb50`).
pub fn use_up_pin(order: &LoadOrder, state: &mut GameState) {
    state.stock(order, PLAYER_REF);
    let key = (PLAYER_REF, BOBBY_PIN);
    let left = state.items.get(&key).copied().unwrap_or(0) - 1;
    if left > 0 {
        state.items.insert(key, left);
    } else {
        state.items.remove(&key);
    }
}

/// Forcing failed (`00790330`): the lock's broken count (lock `+0xC`) goes
/// up; from then on it needs its key ([`is_broken`]).
pub fn break_lock(state: &mut GameState, reference: FormId) {
    *state.broken_locks.entry(reference).or_insert(0) += 1;
}

/// Whether a lock is broken (`00430ae0`, `GetIsLockBroken`): its broken
/// count at least 1, or 2 when the player's "Ignore Broken Lock" perk
/// entry gives something other than 0.
pub fn is_broken(order: &LoadOrder, state: &GameState, reference: FormId) -> bool {
    let ignore = crate::perks::apply(order, state, IGNORE_BROKEN_LOCK, 0.0);
    let limit = if ignore == 0.0 { 1 } else { 2 };
    state.broken_locks.get(&reference).copied().unwrap_or(0) >= limit
}

/// The lock picked (`0078eb50`, stage 3 at the full turn): the "Locks
/// Picked" statistic (4) goes up; the first time this lock is picked it's
/// worth `iXPRewardPickLock<difficulty>`; if it has an owner the stealing
/// rules run with no item (`008bfa40`: −5 karma unless the owner is evil,
/// and a crime if a member of the owner sees it; `world::crime::steal`;
/// the code doesn't ask whether the player may use the owner's things);
/// then it's unlocked. The caller uses the door or container afterwards
/// (`00573170`, the player activating it).
pub fn picked(order: &LoadOrder, state: &mut GameState, reference: FormId, difficulty: u8) {
    crate::stats::bump(state, crate::stats::LOCKS_PICKED, 1);
    if state.picked.insert(reference) {
        let setting = crate::experience::by_difficulty("iXPRewardPickLock", difficulty);
        crate::experience::reward_setting(order, state, &setting);
    }
    if let Some(owner) = crate::crime::owner_of(order, state, reference) {
        crate::crime::steal(order, state, reference, owner);
    }
    state.locks.insert(reference, None);
}

/// A message's text: the setting's, else the exe's default.
pub fn message_text(order: &LoadOrder, setting: &str, default: &str) -> String {
    game_setting_text(order, setting).unwrap_or_else(|| default.to_string())
}

/// The text settings the menu and its messages use, with the exe's
/// defaults (read from their static initializers; `FalloutNV.esm`
/// overrides none of them): what `&-sName;` and the code read when the
/// data has no `GMST`.
pub const EXE_TEXT: [(&str, &str); 17] = [
    ("sLockpickSkillText", "Lockpick Skill"),
    ("sPicksRemainingText", "Bobby Pins"),
    ("sLockLevelText", "Lock Level"),
    ("sExit", "Exit"),
    ("sForceLock", "Force Lock"),
    ("sPCMenuHintF", "F)"),
    ("sPCMenuHintE", "E)"),
    ("sOutOfLockpicks", "You are out of lockpicks."),
    ("sLockBroken", "The lock has been broken."),
    (
        "sImpossibleLock",
        "This lock cannot be picked. It requires a key to open.",
    ),
    (
        "sLockpickSkillTooLow",
        "You need a lockpick skill of %d to pick this lock.",
    ),
    ("sLockLevelNameVeryEasy", "Very Easy"),
    ("sLockLevelNameEasy", "Easy"),
    ("sLockLevelNameAverage", "Average"),
    ("sLockLevelNameHard", "Hard"),
    ("sLockLevelNameVeryHard", "Very Hard"),
    ("sLockLevelNameImpossible", "Requires Key"),
];

/// A text setting the menu uses: the data's, else the exe's default.
pub fn text(order: &LoadOrder, setting: &str) -> Option<String> {
    game_setting_text(order, setting).or_else(|| {
        EXE_TEXT
            .iter()
            .find(|(n, _)| n.eq_ignore_ascii_case(setting))
            .map(|(_, d)| d.to_string())
    })
}

/// The lock level's name (`01184a98`: `sLockLevelName…`, "Very Easy" …
/// "Very Hard", then "Requires Key").
pub fn level_name(order: &LoadOrder, difficulty: u8) -> String {
    const NAMES: [(&str, &str); 6] = [
        ("sLockLevelNameVeryEasy", "Very Easy"),
        ("sLockLevelNameEasy", "Easy"),
        ("sLockLevelNameAverage", "Average"),
        ("sLockLevelNameHard", "Hard"),
        ("sLockLevelNameVeryHard", "Very Hard"),
        ("sLockLevelNameImpossible", "Requires Key"),
    ];
    let (setting, default) = NAMES[usize::from(difficulty.min(5))];
    message_text(order, setting, default)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sweet_spot_lengths_and_force_chance() {
        let s = Settings::default();
        // Average lock (needs 50): at 50 the minimum, at 75 (half the step
        // past 50 to the next level's 75… twice the step is 50) the mix is
        // 0.5; the default table's minimum and maximum are equal anyway.
        assert_eq!(s.mix(2, 50), 0.0);
        assert_eq!(s.mix(2, 75), 0.5);
        assert_eq!(s.mix(2, 100), 1.0);
        assert_eq!(s.lengths(2, 60), (100, 200));
        assert_eq!(s.lengths(4, 100), (25, 200));
        // Very hard: the step to "101" is 1.
        assert_eq!(s.mix(4, 100), 0.0);
        // A table with a range mixes and truncates.
        let mut t = s;
        t.sweet_spot[2] = (100, 151);
        assert_eq!(t.lengths(2, 60), (110, 200));
        // Forcing: skill − requirement + 10, within 0..100.
        assert_eq!(s.force_chance(2, 50), 10);
        assert_eq!(s.force_chance(2, 73), 33);
        assert_eq!(s.force_chance(0, 100), 100);
        assert_eq!(s.force_chance(4, 100), 10);
        assert_eq!(s.pin_health(50), 1.5);
    }

    #[test]
    fn rings_and_how_far_the_lock_turns() {
        let s = Settings::default();
        // Average lock at skill 50 on a 1646.67-unit meter, centre in the
        // middle: sweet 100, spacing 200, half 250.
        let z = Zone::new(&s, 2, 50, 1646.6666, 0.5);
        assert_eq!((z.sweet, z.concentric, z.half), (100, 200, 250));
        assert!((z.center - 823.3333).abs() < 1e-3);
        // Ring x: trunc(823.33 − 50) = 773, then 40 further out each.
        let xs: Vec<i32> = z.rings.iter().map(|r| r.x).collect();
        assert_eq!(xs, vec![773, 733, 693, 653, 613, 573]);
        assert!((z.rings[0].width - 100.6667).abs() < 1e-3);
        assert_eq!(z.rings[0].id, 17);
        assert_eq!(z.rings[5].depth, 1);
        assert_eq!(z.rings[5].color, [1.0, 0.0, 0.0]);
        // In the sweet spot: the full turn; one ring out 75°; …
        assert_eq!(z.max_turn(z.center + 10.0), Some(90.0));
        assert_eq!(z.max_turn(z.center + 60.0), Some(75.0));
        assert_eq!(z.max_turn(z.center - 249.0), Some(15.0));
        // Past the rings: 15° at the edge, falling to 0 at the far end.
        assert_eq!(z.max_turn(z.center + 250.0), Some(15.0));
        let far = z.max_turn(0.0).unwrap();
        assert!((far - 15.0 * (250.0 + 823.3333 - 1646.6666) / (500.0 - 1646.6666)).abs() < 1e-3);
        // The centre stays `half` from the ends.
        assert_eq!(Zone::new(&s, 2, 50, 1646.6666, 0.0).center, 250.0);
        assert_eq!(Zone::new(&s, 2, 50, 1000.0, 0.99).center, 750.0);
    }

    #[test]
    fn the_pin_turns_about_its_pivot() {
        // In the middle the pin is upright and where the file puts it.
        let (r, t) = pin_transform(500.0, 1000.0);
        assert!((r[0][0] - 1.0).abs() < 1e-6 && r[0][2].abs() < 1e-6);
        assert!((t[2] - PIN_OFFSET[2]).abs() < 1e-5);
        // At the left end it lies a quarter turn over: its tip (up, +z)
        // points to −x, and its offset turns about the pivot.
        let (r, t) = pin_transform(0.0, 1000.0);
        let up = [r[0][2], r[1][2], r[2][2]];
        assert!((up[0] + 1.0).abs() < 1e-6 && up[2].abs() < 1e-6);
        assert!((t[0] - 2.2).abs() < 1e-5 && (t[2] - 0.6).abs() < 1e-5);
    }

    fn sequences() -> Sequences {
        Sequences {
            lock_forward: Some(Span {
                begin: 0.0,
                end: 1.0,
            }),
            lock_backward: Some(Span {
                begin: 0.0,
                end: 3.0,
            }),
            pin_forward: Some(Span {
                begin: 0.0,
                end: 0.5,
            }),
            pin_backward: Some(Span {
                begin: 0.0,
                end: 1.0,
            }),
            pin_left: Some(Span {
                begin: 0.0,
                end: 0.4,
            }),
        }
    }

    fn frame(ms: u32) -> Frame {
        Frame {
            ms,
            active: true,
            ..Frame::default()
        }
    }

    fn run(p: &mut Picking, f: Frame) -> Vec<Effect> {
        p.update(&f, &mut || 5)
    }

    #[test]
    fn ready_asks_for_the_tutorial_and_waits_on_it() {
        let s = Settings::default();
        let mut p = Picking::open(FormId(1), 2, 50, 3, s, 1000.0, 0.5, sequences()).unwrap();
        let unseen = |ms| Frame {
            tutorial_unseen: true,
            ..frame(ms)
        };
        // Entering isn't held up by it.
        while p.stage == Stage::Entering {
            assert!(!run(&mut p, unseen(16)).contains(&Effect::Tutorial));
        }
        // Ready: asked for, nothing else (a turn key does nothing).
        let mut f = unseen(16);
        f.turn_pressed = true;
        f.turn_held = true;
        assert_eq!(run(&mut p, f), vec![Effect::Tutorial]);
        assert_eq!(p.stage, Stage::Ready);
        // Asked for (`ShowMessage` said yes): the menu waits while the
        // tutorial is up (not on top) and until it has been shown.
        p.tutorial_wait = true;
        let mut f = frame(16);
        f.active = false;
        f.turn_pressed = true;
        f.turn_held = true;
        assert!(run(&mut p, f).is_empty());
        let mut f = unseen(16);
        f.turn_pressed = true;
        f.turn_held = true;
        assert!(run(&mut p, f).is_empty());
        assert!(p.tutorial_wait);
        // Shown and back on top: it goes on.
        let mut f = frame(16);
        f.turn_pressed = true;
        f.turn_held = true;
        let e = run(&mut p, f);
        assert!(!p.tutorial_wait);
        assert!(e.contains(&Effect::Sound("UILockpickingCylinderTurn")));
        assert_eq!(p.stage, Stage::Turning);
    }

    #[test]
    fn entering_turning_and_opening() {
        let s = Settings::default();
        assert_eq!(
            Picking::open(FormId(1), 2, 40, 3, s, 1000.0, 0.5, sequences()),
            Err(Refusal::SkillTooLow(50))
        );
        let mut p = Picking::open(FormId(1), 2, 50, 3, s, 1000.0, 0.5, sequences()).unwrap();
        // The first frame plays the entering sound; the screwdriver and pin
        // go in over the lock's `Forward` (1 s).
        assert_eq!(
            run(&mut p, frame(16)),
            vec![Effect::Sound("UILockpickingEnter")]
        );
        let mut frames = 1;
        while p.stage == Stage::Entering {
            run(&mut p, frame(16));
            frames += 1;
        }
        // The stage's clock counts from the second frame: 63 × 16 ms ≥ 1 s.
        assert_eq!(frames, 64);
        assert_eq!(p.stage, Stage::Ready);
        // The pick is in the middle: the sweet spot is too (centre 500).
        let mut f = frame(16);
        f.turn_pressed = true;
        f.turn_held = true;
        let e = run(&mut p, f);
        assert!(e.contains(&Effect::Sound("UILockpickingCylinderTurn")));
        assert_eq!(p.max_turn, 90.0);
        assert_eq!(p.stage, Stage::Turning);
        // 90° a second: after a second of turning it opens (the first frame
        // of a stage counts no time).
        f.turn_pressed = false;
        let mut opened = false;
        for _ in 0..80 {
            let e = run(&mut p, f);
            if e.contains(&Effect::Opened) {
                opened = true;
                assert!(e.contains(&Effect::Close));
                break;
            }
        }
        assert!(opened);
        assert_eq!(p.cylinder, 90.0);
    }

    #[test]
    fn straining_breaks_the_pin_and_a_new_one_goes_in() {
        let s = Settings::default();
        let mut p = Picking::open(FormId(1), 2, 50, 2, s, 1000.0, 0.5, sequences()).unwrap();
        while p.stage != Stage::Ready {
            run(&mut p, frame(16));
        }
        // Move the pick off the sweet spot: 300 units right is outside
        // the rings (half 250): (250 + 300 − 1000) / (500 − 1000) × 15 = 13.5°.
        let mut f = frame(16);
        f.mouse = 300.0;
        run(&mut p, f);
        assert!((p.max_turn - 13.5).abs() < 1e-4);
        f.mouse = 0.0;
        f.turn_pressed = true;
        f.turn_held = true;
        run(&mut p, f);
        f.turn_pressed = false;
        let mut e = Vec::new();
        while p.stage == Stage::Turning {
            e = run(&mut p, f);
        }
        assert!(e.contains(&Effect::Sound("UILockpickingCylinderStop")));
        assert!(e.contains(&Effect::TensionStart));
        assert_eq!(p.stage, Stage::Straining);
        // Health 1.5 at skill 50: a second and a half of straining.
        let mut frames = 0;
        let mut broke = false;
        while p.stage == Stage::Straining {
            let e = run(&mut p, f);
            frames += 1;
            if e.contains(&Effect::PinBroken) {
                broke = true;
                assert!(e.contains(&Effect::Sound("UILockpickingPickBreak")));
                assert!(e.contains(&Effect::TensionFade(100)));
            }
        }
        assert!(broke);
        // 1.5 s at 16 ms a frame (health uses each frame's milliseconds,
        // the first one in the stage too).
        assert_eq!(frames, 94);
        assert_eq!(p.pins, 1);
        // The break plays, a second passes, a new pin goes in.
        while p.stage != Stage::NewPin {
            run(&mut p, f);
        }
        assert_eq!(p.health, 1.5);
        assert_eq!(p.cylinder, 0.0);
        assert_eq!(p.pick_x, 500.0);
    }

    #[test]
    fn letting_go_springs_back_and_no_pins_left() {
        let s = Settings::default();
        let mut p = Picking::open(FormId(1), 0, 30, 0, s, 1000.0, 0.5, sequences()).unwrap();
        let e = run(&mut p, frame(16));
        assert_eq!(e, vec![out_of_pins()]);
        while p.stage != Stage::Ready {
            run(&mut p, frame(16));
        }
        // With no pins the turn keys do nothing.
        let mut f = frame(16);
        f.turn_pressed = true;
        f.turn_held = true;
        run(&mut p, f);
        assert_eq!(p.stage, Stage::Ready);
        // Springing back: 90° a second, squeaking past 60 and 30.
        p.cylinder = 61.0;
        let e = run(&mut p, frame(100));
        assert!((p.cylinder - 52.0).abs() < 1e-4);
        assert_eq!(e, vec![Effect::Sound("UILockpickingCylinderSqueakB")]);
    }

    #[test]
    fn forcing() {
        let s = Settings::default();
        let mut p = Picking::open(FormId(1), 2, 60, 3, s, 1000.0, 0.5, sequences()).unwrap();
        // Chance 20: a roll of 19 forces it, the cylinder turning all the
        // way however the pick sits; 20 breaks the lock.
        assert!(p.force(19).is_empty());
        assert!(p.forced);
        assert_eq!((p.stage, p.max_turn), (Stage::Turning, 90.0));
        let mut q = Picking::open(FormId(1), 2, 60, 3, s, 1000.0, 0.5, sequences()).unwrap();
        let e = q.force(20);
        assert_eq!(e[0], Effect::LockBroken);
        assert_eq!(q.stage, Stage::Closing);
        assert_eq!(p.force_label("Force Lock"), "Force Lock [20%]");
    }
}

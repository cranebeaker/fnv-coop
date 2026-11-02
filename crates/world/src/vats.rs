//! V.A.T.S., the Vault-Tec Assisted Targeting System, read from the game's
//! code (`%USERPROFILE%\nv-re\findings\vats.md`; the functions named
//! below). The player stops the world, picks people and their body parts by
//! the chance to hit each, queues attacks paid for with action points, and
//! watches them play out.
//!
//! - **Action points** (actor value 12): most `fAVDActionPointsBase` 65 +
//!   `fAVDActionPointsMult` 3 × Agility (`006439d0`; 80 at Agility 5);
//!   back at `fActionPointsRestoreRate` 0.06 × the most a second × perk
//!   entry 39, only while V.A.T.S. is off ([`regenerate`], `0088b660`); the
//!   cost of an attack is taken when it has played ([`spend`]).
//! - **What an attack costs** ([`attack_cost`], `0066dce0`): the weapon's
//!   own `DNAM` f32 at 68 when its second flags (`DNAM` u32 at 56) have
//!   0x08 (every vanilla weapon: the 9mm pistol 17), else the setting for
//!   its kind of attack ([`kind`], `fActionPointsAttackPistol` 20 …), then
//!   perk entry 40 (Fast Shot × 0.8 for Guns and Energy Weapons). A queued
//!   gun attack that runs the projected clip out adds `fActionPointsReload`
//!   10 and is followed by a "Reload" line ([`Plan::queue`]).
//! - **Who can be targeted** ([`eligible`], `007f52c0`): living people and
//!   creatures 2 to `fVATSMaxEngageDistance` (5000) units away, in line of
//!   sight, on screen (their root or one of their parts' nodes) or nearer
//!   than the sneak distance (`fSneakMaxDistance` 2500 indoors, × 2
//!   outside); not those whose base is in `BannedVATSTargets`. The first
//!   picked by [`first_target`] (`007e9200`).
//! - **Parts** ([`parts_offered`]): every part the body part data has
//!   (`BPTD`, aimed at its `BPNT` node), and the weapon held unless it's
//!   built in (`DNAM` flags 0x20).
//! - **The chance to hit** ([`hit_chance`], `00646f70`; [`part_chance`],
//!   `007f1290`): see there. Shown truncated, at most 95
//!   ([`shown_percent`], `007f0ea0`).
//! - **The queue** ([`Plan`], `007ec810`): hit or miss is rolled when an
//!   attack is queued; Concentrated Fire adds 5 points for each attack
//!   before it in a row on the same part.
//! - **While it plays** ([`player_damage_mult`], [`critical_bonus`],
//!   [`attack_damage_mult`]; `009b5a30`, `009b7060`, `009b5170`): the
//!   player takes `fVATSPlayerDamageMult` 0.75 of their damage, queued hits
//!   are `fVATSCriticalChanceBonus` 5 points likelier to be critical, and
//!   melee moves hit harder. Without a camera shot (`CAMS`) the world runs
//!   at its own speed and the player's attacks `fVATSPlayerTimeUpdateMult`
//!   6 times as fast.
//!
//! Settings: the value `FalloutNV.esm` sets, else the exe's own default
//! ([`Settings`]).

use std::f32::consts::PI;

use esm::{FormId, FourCC, LoadOrder};

use crate::body_parts::{part, BodyPart, BodyPartData};
use crate::cell::{le_f32, le_u32};
use crate::combat::Weapon;
use crate::dialogue::PLAYER_REF;
use crate::perks::{self, Tab};
use crate::scripting::{base_of, game_setting, Facts, GameState, Value};

const DNAM: FourCC = FourCC::new(b"DNAM");
const VATS: FourCC = FourCC::new(b"VATS");
const OBND: FourCC = FourCC::new(b"OBND");
const DAT2: FourCC = FourCC::new(b"DAT2");
const VANM: FourCC = FourCC::new(b"VANM");

/// The action points actor value.
pub const ACTION_POINTS: u16 = 12;

/// The default unarmed weapon, which perks' weapon conditions are asked
/// about when the player holds none (`[011ca278]`; taken to be `Fists`,
/// `000001F4`: inferred).
pub const FISTS: FormId = FormId(0x1F4);

/// `VATSBannedWeaponsList` (`FLST` `00174256`): weapons V.A.T.S. can't be
/// used with (`007e9200`).
pub const BANNED_WEAPONS: FormId = FormId(0x0017_4256);

/// `BannedVATSTargets` (`FLST` `001770BC`): bases that can't be targeted.
pub const BANNED_TARGETS: FormId = FormId(0x0017_70BC);

/// The V.A.T.S. menu's number for the scripts' `MenuMode` blocks (the
/// menu ids made in `0071e420`: `VATSMenu` 1056). It's open in modes 1–3,
/// not during playback.
pub const VATS_MENU: u16 = 1056;

/// The V.A.T.S. manager's mode (`[011f2250]+0x08`, `009c6c30`; what
/// `GetVATSMode` answers, `005a2590`).
pub mod mode {
    pub const OFF: u8 = 0;
    /// The menu is open: choosing.
    pub const MENU: u8 = 1;
    /// A target's parts have been scanned.
    pub const READY: u8 = 2;
    /// A target's parts are being scanned.
    pub const SCANNING: u8 = 3;
    /// The queue plays.
    pub const PLAYBACK: u8 = 4;
}

/// Kinds of attack: the action point numbers (`0066dba0` maps a weapon's
/// animation type to them; the cost table at `0119baf8` is indexed by
/// them).
pub mod kind {
    pub const UNARMED: u8 = 0;
    pub const ONE_HAND_MELEE: u8 = 1;
    pub const TWO_HAND_MELEE: u8 = 2;
    pub const PISTOL: u8 = 3;
    pub const RIFLE: u8 = 4;
    pub const HANDLE: u8 = 5;
    pub const LAUNCHER: u8 = 6;
    pub const GRENADE: u8 = 7;
    pub const MINE: u8 = 8;
    pub const RELOAD: u8 = 9;
    /// A weapon's own special attack (its `VATS` record).
    pub const SPECIAL: u8 = 0x10;
    pub const UPPERCUT: u8 = 0x11;
    pub const THROWN: u8 = 0x13;
    pub const CROSS: u8 = 0x14;
    pub const STOMP: u8 = 0x15;
    /// No attack at all (a lunchbox mine drop).
    pub const NONE: u8 = 0x16;
}

/// Weapon `DNAM` flags V.A.T.S. reads.
pub mod flags {
    /// First set (u8 at 12): fires while held.
    pub const AUTOMATIC: u8 = 0x02;
    /// First set: built into a creature (a turret's gun), not a target.
    pub const EMBEDDED: u8 = 0x20;
    /// Second set (u32 at 56): the attack's cost is `DNAM` f32 at 68.
    pub const OVERRIDE_AP: u32 = 0x08;
    /// Second set: nothing beyond the max range.
    pub const FIXED_RANGE: u32 = 0x20;
    /// Second set: automatic bursts last `fVATSShotLongBurstTime`.
    pub const LONG_BURST: u32 = 0x800;
}

/// The settings V.A.T.S. reads, as `FalloutNV.esm` sets them or else the
/// exe's defaults (`findings\vats.md` §11; the defaults from the exe's
/// settings table, `decomp\hits\settings_all.txt`).
#[derive(Debug, Clone, PartialEq)]
pub struct Settings {
    pub max_engage_distance: f32,
    pub target_angle: f32,
    pub screen_percent_factor: f32,
    pub melee_max_distance: f32,
    pub thrown_range_mult: f32,
    pub grenade_range_mult: f32,
    pub grenade_range_min: f32,
    pub skill_factor: f32,
    pub grenade_skill_factor: f32,
    pub grenade_chance_mult: f32,
    pub spread_mult: f32,
    pub grenade_target_area: f32,
    pub destructible_mult: f32,
    pub range_spread_max: f32,
    pub hit_chance_mult: f32,
    pub stealth_mult: f32,
    pub max_chance: f32,
    pub concentrated_fire_bonus: f32,
    pub paralyze_palm_chance: f32,
    pub shot_burst_time: f32,
    pub shot_long_burst_time: f32,
    pub restore_rate: f32,
    /// Each kind of attack's cost (by [`kind`] number; 0 where none).
    pub costs: [f32; 22],
    pub uppercut_threshold: f32,
    pub uppercut_cost: f32,
    pub cross_threshold: f32,
    pub cross_cost: f32,
    pub player_time_mult: f32,
    pub playback_delay: f32,
    pub camera_max_time: f32,
    pub melee_warp_mult: f32,
    pub h2h_warp_mult: f32,
    pub melee_reach_mult: f32,
    pub miss_ratio_low: f32,
    pub miss_ratio_high: f32,
    pub auto_aim_max_degrees: f32,
    pub auto_aim_max_miss_degrees: f32,
    pub shotgun_spread_ratio: f32,
    pub critical_bonus: f32,
    pub player_damage_mult: f32,
    pub automatic_melee_mult: f32,
    pub uppercut_mult: f32,
    pub cross_mult: f32,
    pub stomp_mult: f32,
    pub sneak_max_distance: f32,
    pub sneak_exterior_mult: f32,
    pub npc_max_gun_wobble: f32,
    pub standing_spread: f32,
    pub walking_spread: f32,
    pub running_spread: f32,
    pub unaimed_spread: f32,
    pub crippled_arm_1h: f32,
    pub crippled_arm_2h: f32,
    pub crippled_arms_1h: f32,
    pub crippled_arms_2h: f32,
    pub min_gun_spread: f32,
    pub wobble_to_skill: f32,
    pub strength_req_penalty: f32,
}

/// The cost table's settings by kind (`0119baf8`; 15–18 and 20–21 have
/// none, 19 is `fActionPointsAttackThrown`).
const COST_SETTINGS: [Option<&str>; 22] = [
    Some("fActionPointsAttackUnarmed"),
    Some("fActionPointsAttackOneHandMelee"),
    Some("fActionPointsAttackTwoHandMelee"),
    Some("fActionPointsAttackPistol"),
    Some("fActionPointsAttackRifle"),
    Some("fActionPointsAttackHandle"),
    Some("fActionPointsAttackLauncher"),
    Some("fActionPointsAttackGrenade"),
    Some("fActionPointsAttackMine"),
    Some("fActionPointsReload"),
    Some("fActionPointsCrouch"),
    Some("fActionPointsStand"),
    Some("fActionPointsSwitchWeapon"),
    Some("fActionPointsToggleWeaponDrawn"),
    Some("fActionPointsHeal"),
    None,
    None,
    None,
    None,
    Some("fActionPointsAttackThrown"),
    None,
    None,
];

impl Settings {
    pub fn load(order: &LoadOrder) -> Settings {
        let g = |name: &str, exe: f32| game_setting(order, name).unwrap_or(exe);
        let mut costs = [0.0; 22];
        for (cost, name) in costs.iter_mut().zip(COST_SETTINGS) {
            // Every one is 1.0 in the exe; the data sets all but the
            // thrown one.
            if let Some(n) = name {
                *cost = g(n, 1.0);
            }
        }
        Settings {
            max_engage_distance: g("fVATSMaxEngageDistance", 5000.0),
            target_angle: g("iVatsTargetAngle", 15.0),
            screen_percent_factor: g("fVATSScreenPercentFactor", 1.0),
            melee_max_distance: g("fVATSMeleeMaxDistance", 300.0),
            thrown_range_mult: g("fVATSThrownWeaponRangeMult", 2.0),
            grenade_range_mult: g("fVATSGrenadeRangeMult", 2.0),
            grenade_range_min: g("fVATSGrenadeRangeMin", 128.0),
            skill_factor: g("fVATSSkillFactor", 1.0),
            grenade_skill_factor: g("fVATSGrenadeSkillFactor", 0.4),
            grenade_chance_mult: g("fVATSGrenadeChanceMult", 1.0),
            spread_mult: g("fVATSSpreadMult", 1.0),
            grenade_target_area: g("fVATSGrenadeTargetArea", 25.0),
            destructible_mult: g("fVATSDestructibleMult", 1.25),
            range_spread_max: g("fVATSRangeSpreadMax", 2.0),
            hit_chance_mult: g("fVATSHitChanceMult", 1.0),
            stealth_mult: g("fVATSStealthMult", 0.5),
            max_chance: g("fVATSMaxChance", 95.0),
            concentrated_fire_bonus: g("iVATSConcentratedFireBonus", 5.0),
            paralyze_palm_chance: g("fVATSParalyzePalmChance", 0.1),
            shot_burst_time: g("fVATSShotBurstTime", 0.43),
            shot_long_burst_time: g("fVATSShotLongBurstTime", 0.75),
            restore_rate: g("fActionPointsRestoreRate", 1.0),
            costs,
            uppercut_threshold: g("fUpperCutThreshold", 50.0),
            uppercut_cost: g("fUpperCutAPCost", 20.0),
            cross_threshold: g("fCrossThreshold", 75.0),
            cross_cost: g("fCrossAPCost", 20.0),
            player_time_mult: g("fVATSPlayerTimeUpdateMult", 3.0),
            playback_delay: g("fVATSPlaybackDelay", 3.0),
            camera_max_time: g("fVATSCameraMaxTime", 20.0),
            melee_warp_mult: g("fVATSMeleeWarpDistanceMult", 0.5),
            h2h_warp_mult: g("fVATSH2HWarpDistanceMult", 0.5),
            melee_reach_mult: g("fVATSMeleeReachMult", 2.0),
            miss_ratio_low: g("fAutoAimMissRatioLow", 0.86),
            miss_ratio_high: g("fAutoAimMissRatioHigh", 1.15),
            auto_aim_max_degrees: g("fAutoAimMaxDegreesVATS", 15.0),
            auto_aim_max_miss_degrees: g("fAutoAimMaxDegreesMiss", 3.0),
            shotgun_spread_ratio: g("fVatsShotgunSpreadRatio", 1.0),
            critical_bonus: g("fVATSCriticalChanceBonus", 15.0),
            player_damage_mult: g("fVATSPlayerDamageMult", 1.0),
            automatic_melee_mult: g("fVATSAutomaticMeleeDamageMult", 2.0),
            uppercut_mult: g("fUpperCutVatsMultiplier", 1.15),
            cross_mult: g("fCrossVatsMultiplier", 1.1),
            stomp_mult: g("fGroundAttackVatsMultiplier", 2.0),
            sneak_max_distance: g("fSneakMaxDistance", 2500.0),
            sneak_exterior_mult: g("fSneakExteriorDistanceMult", 2.0),
            npc_max_gun_wobble: g("fNPCMaxGunWobbleAngle", 5.0),
            standing_spread: g("fStandingSpreadPenalty", 0.1),
            walking_spread: g("fWalkingSpreadPenalty", 0.1),
            running_spread: g("fRunningSpreadPenalty", 0.2),
            unaimed_spread: g("fUnaimedSpreadPenalty", 0.2),
            crippled_arm_1h: g("fCrippledArm1HSpreadPenalty", 0.2),
            crippled_arm_2h: g("fCrippledArm2HSpreadPenalty", 0.4),
            crippled_arms_1h: g("fCrippledArms1HSpreadPenalty", 0.4),
            crippled_arms_2h: g("fCrippledArms2HSpreadPenalty", 0.6),
            min_gun_spread: g("fMinGunSpreadValue", 0.01),
            wobble_to_skill: g("fWobbleToSkillConversion", 0.5),
            strength_req_penalty: g("fWeapStrengthReqPenalty", 0.1),
        }
    }

    /// A kind of attack's cost from the table (0 for kinds without one).
    pub fn cost(&self, kind: u8) -> f32 {
        self.costs.get(usize::from(kind)).copied().unwrap_or(0.0)
    }
}

/// What V.A.T.S. is doing now, kept in the game state (not saved).
#[derive(Debug, Clone, Default, PartialEq)]
pub struct VatsNow {
    /// [`mode`].
    pub mode: u8,
    /// The attack being composed (the menu's template, `007f5280`) or
    /// playing (mode 4): what `GetVATSValue` asks about, and whether hits
    /// now carry V.A.T.S.'s critical bonus.
    pub attack: Option<AttackFacts>,
    /// The player's "smart camera" checks for the attack playing, worked
    /// out by whoever can cast lines (the viewer) as it starts: what the
    /// camera paths' `GetVATS…AreaFree` / `…TargetVisible` conditions ask.
    pub smart_camera: Option<SmartCamera>,
}

// ---------------------------------------------------------------------------
// The smart camera checks (`008bd830`, `008bdbd0`), which the camera
// paths' conditions ask about the attacker.

/// A side of someone: where the eight `GetVATS<Side>AreaFree` and
/// `GetVATS<Side>TargetVisible` functions look.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Side {
    Front,
    Right,
    Left,
    Back,
}

impl Side {
    pub const ALL: [Side; 4] = [Side::Front, Side::Right, Side::Left, Side::Back];

    /// The turn from the actor's heading the function passes (`005a5610`
    /// 0, `005a5460` the float at `0101ff38` = π/2, `005a54f0` `0101ff34`
    /// = 3π/2, `005a5580` `0102b3c8` = π).
    pub fn turn(self) -> f32 {
        match self {
            Side::Front => 0.0,
            Side::Right => PI / 2.0,
            Side::Left => 1.5 * PI,
            Side::Back => PI,
        }
    }

    /// Which function names look this way.
    pub fn of_function(name: &str) -> Option<(Side, bool)> {
        let (side, rest) = match name.strip_prefix("GetVATS") {
            Some(r) if r.starts_with("Front") => (Side::Front, &r[5..]),
            Some(r) if r.starts_with("Right") => (Side::Right, &r[5..]),
            Some(r) if r.starts_with("Left") => (Side::Left, &r[4..]),
            Some(r) if r.starts_with("Back") => (Side::Back, &r[4..]),
            _ => return None,
        };
        match rest {
            "AreaFree" => Some((side, false)),
            "TargetVisible" => Some((side, true)),
            _ => None,
        }
    }
}

/// The settings the checks read (`FalloutNV.esm`'s values, else the
/// exe's).
#[derive(Debug, Clone, PartialEq)]
pub struct SmartCameraSettings {
    /// `fVATSSmartCameraCheckHeight` (exe 96, `FalloutNV.esm` 64): the
    /// lines start that high above the actor's position.
    pub check_height: f32,
    /// `fVATSSmartCameraCheckStepDistance` (exe 128, `FalloutNV.esm` 64)
    /// and `…StepCount` (5): the sample points of the visibility check.
    pub step_distance: f32,
    pub step_count: u32,
}

impl SmartCameraSettings {
    pub fn load(order: &LoadOrder) -> SmartCameraSettings {
        let g = |name: &str, exe: f32| game_setting(order, name).unwrap_or(exe);
        SmartCameraSettings {
            check_height: g("fVATSSmartCameraCheckHeight", 96.0),
            step_distance: g("fVATSSmartCameraCheckStepDistance", 128.0),
            step_count: g("fVATSSmartCameraCheckStepCount", 5.0).max(0.0) as u32,
        }
    }
}

/// How far an area-free line reaches (`008bd830`, the float at
/// `01022958`), and its answer when nothing is met (`01016970`, the
/// largest float).
pub const AREA_FREE_LENGTH: f32 = 10000.0;
pub const AREA_FREE_NONE: f32 = f32::MAX;

/// A hit on a visibility line blocks it only up to this share of the way
/// to the target (`008bdbd0`, the double at `01019de8`), and a hit on an
/// actor only within this many units of the sample point (`010231d8`).
pub const VISIBLE_BLOCKED_FRACTION: f32 = 0.85;
pub const VISIBLE_ACTOR_RANGE: f32 = 256.0;

/// The direction `side` of someone facing `heading` (clockwise from
/// north): `004a0c90` makes the turn's matrix and `00439f50` takes its
/// second column, (sin, cos, 0) of the heading plus the side's turn.
pub fn side_direction(heading: f32, side: Side) -> [f32; 3] {
    let a = heading + side.turn();
    [a.sin(), a.cos(), 0.0]
}

/// Where the checks start: the actor's position (its reference's,
/// vfunc `+0x1f4`) raised by the check height.
pub fn check_start(s: &SmartCameraSettings, position: [f32; 3]) -> [f32; 3] {
    [position[0], position[1], position[2] + s.check_height]
}

/// `GetVATS<Side>AreaFree` (`008bd830`): the line the check casts (on the
/// `CAMERAPICK` layer, 35), from the raised position [`AREA_FREE_LENGTH`]
/// units along the side.
pub fn area_free_line(
    s: &SmartCameraSettings,
    position: [f32; 3],
    heading: f32,
    side: Side,
) -> ([f32; 3], [f32; 3]) {
    let from = check_start(s, position);
    let d = side_direction(heading, side);
    (from, [0, 1, 2].map(|i| from[i] + d[i] * AREA_FREE_LENGTH))
}

/// Its answer: the first thing met along the line that isn't the actor,
/// as its share of the line's length, gives that many units; nothing,
/// [`AREA_FREE_NONE`].
pub fn area_free(first_hit: Option<f32>) -> f32 {
    first_hit.map_or(AREA_FREE_NONE, |f| f * AREA_FREE_LENGTH)
}

/// `GetVATS<Side>TargetVisible` (`008bdbd0`): the sample points, `step
/// count` of them `step distance` apart along the side from the raised
/// position (the first a step out).
pub fn visible_samples(
    s: &SmartCameraSettings,
    position: [f32; 3],
    heading: f32,
    side: Side,
) -> Vec<[f32; 3]> {
    let from = check_start(s, position);
    let d = side_direction(heading, side);
    (1..=s.step_count)
        .map(|k| {
            let r = s.step_distance * k as f32;
            [0, 1, 2].map(|i| from[i] + d[i] * r)
        })
        .collect()
}

/// A hit on a line from a sample point to the target, as the check reads
/// it: its share of the way, and whether what was hit is an actor.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LineHit {
    pub fraction: f32,
    pub actor: bool,
}

/// Its answer: each sample in turn casts a line to the target's node
/// (`hits` gives everything met on it that isn't the target itself); the
/// line is blocked by a hit within [`VISIBLE_BLOCKED_FRACTION`] of the way,
/// unless it's an actor more than [`VISIBLE_ACTOR_RANGE`] units from the
/// sample point. The answer is the farthest sample reached before a
/// blocked one, as its distance from the raised position (0 when the
/// first is blocked, or there's no target).
pub fn target_visible(
    s: &SmartCameraSettings,
    samples: &[[f32; 3]],
    target: Option<[f32; 3]>,
    mut hits: impl FnMut([f32; 3], [f32; 3]) -> Vec<LineHit>,
) -> f32 {
    let Some(target) = target else {
        return 0.0;
    };
    let mut reached = 0.0;
    for (k, &p) in samples.iter().enumerate() {
        let len = {
            let d = [0, 1, 2].map(|i| target[i] - p[i]);
            (d[0] * d[0] + d[1] * d[1] + d[2] * d[2]).sqrt()
        };
        let blocked = hits(p, target).iter().any(|h| {
            let mut b = h.fraction <= VISIBLE_BLOCKED_FRACTION;
            if h.actor && len * h.fraction > VISIBLE_ACTOR_RANGE {
                b = false;
            }
            b
        });
        if blocked {
            break;
        }
        reached = s.step_distance * (k + 1) as f32;
    }
    reached
}

/// The eight checks' answers for the attack playing, by [`Side::ALL`]'s
/// order.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct SmartCamera {
    pub area_free: [f32; 4],
    pub target_visible: [f32; 4],
}

impl SmartCamera {
    pub fn get(&self, side: Side, visible: bool) -> f32 {
        let i = Side::ALL.iter().position(|s| *s == side).unwrap_or(0);
        if visible {
            self.target_visible[i]
        } else {
            self.area_free[i]
        }
    }
}

/// An attack as `GetVATSValue` sees it (`00594e40`).
#[derive(Debug, Clone, PartialEq)]
pub struct AttackFacts {
    pub weapon: Option<FormId>,
    pub target: FormId,
    /// The part as its actor value (25 head, 26 torso, 27–28 arms, 29–30
    /// legs; -1 the weapon).
    pub part_av: i32,
    /// [`kind`].
    pub kind: u8,
    /// Whether it was rolled a hit.
    pub hit: bool,
    /// Between the player's and the target's edges (`009a64d0`).
    pub distance: f32,
    pub paralyzing_palm: bool,
}

/// The kind of attack a weapon makes (`0066dba0`): fists and hand-to-hand
/// weapons (animation type 0) unarmed, 1 one-handed melee, 2 two-handed
/// melee, 3–4 pistol, 5–7 rifle, 8 handle, 9 launcher, 10 grenade, 11
/// mine, 13 thrown, 12 (a lunchbox mine drop) none.
pub fn attack_kind(animation: Option<u32>) -> u8 {
    match animation {
        None | Some(0) => kind::UNARMED,
        Some(1) => kind::ONE_HAND_MELEE,
        Some(2) => kind::TWO_HAND_MELEE,
        Some(3 | 4) => kind::PISTOL,
        Some(5..=7) => kind::RIFLE,
        Some(8) => kind::HANDLE,
        Some(9) => kind::LAUNCHER,
        Some(10) => kind::GRENADE,
        Some(11) => kind::MINE,
        Some(13) => kind::THROWN,
        _ => kind::NONE,
    }
}

/// The melee kinds (`009c9f60`).
pub fn is_melee_kind(k: u8) -> bool {
    matches!(
        k,
        kind::UNARMED
            | kind::ONE_HAND_MELEE
            | kind::TWO_HAND_MELEE
            | kind::SPECIAL
            | kind::UPPERCUT
            | kind::CROSS
            | kind::STOMP
    )
}

/// A kind's line in the queue (`0066ddc0`, the table at `0119bb50`: the
/// exe's own words, which `FalloutNV.esm` doesn't change; the specials'
/// from `sVatsUnarmedAttack1`, `…2`, `…Ground`).
pub fn kind_label(k: u8) -> &'static str {
    match k {
        kind::UNARMED => "Punch",
        kind::ONE_HAND_MELEE | kind::TWO_HAND_MELEE | kind::SPECIAL => "Attack",
        kind::PISTOL..=kind::LAUNCHER => "Shot",
        kind::GRENADE | kind::THROWN => "Throw",
        kind::MINE => "Drop",
        kind::RELOAD => "Reload",
        kind::UPPERCUT => "Uppercut",
        kind::CROSS => "Cross",
        kind::STOMP => "Stomp",
        _ => "",
    }
}

/// The weapon fields V.A.T.S. reads besides `world::combat::Weapon`'s.
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct WeaponVats {
    /// `DNAM` u8 at 40: the base chance to hit the weapon when someone
    /// holds it and their body has no part for it (`00647730`).
    pub to_hit: u8,
    /// `DNAM` f32 at 68: the attack's cost when [`flags::OVERRIDE_AP`].
    pub ap: f32,
    /// `DNAM` u32 at 168 and 200: the Strength and skill it asks for
    /// (`00647810`, `00647870`).
    pub strength_req: u32,
    pub skill_req: u32,
    /// The `VATS` record: its own special attack.
    pub special: Option<WeaponSpecial>,
}

/// A weapon's special V.A.T.S. attack (`VATS`, 20 bytes; the machete's:
/// no spell, skill 0, damage × 0.7, 16 action points; its name is the
/// weapon's `VANM`, [`attack_name`]).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct WeaponSpecial {
    /// Form at 0: the effect it casts.
    pub effect: Option<FormId>,
    /// f32 at 4: the skill needed.
    pub skill: f32,
    /// f32 at 8: × the damage.
    pub damage_mult: f32,
    /// f32 at 12: its cost (none above 0: no special).
    pub ap: f32,
    /// Bytes 16 and 17.
    pub silent: bool,
    pub mod_required: bool,
}

impl WeaponVats {
    pub fn load(order: &LoadOrder, weapon: FormId) -> Option<WeaponVats> {
        let rr = order
            .get(weapon)
            .filter(|r| r.entry.header.kind.as_bytes() == b"WEAP")?;
        let record = rr.record().ok()?;
        let dnam = record.get(DNAM).map(|s| s.data.as_slice()).unwrap_or(&[]);
        let f = |at: usize| (dnam.len() >= at + 4).then(|| le_f32(dnam, at));
        let u = |at: usize| (dnam.len() >= at + 4).then(|| le_u32(dnam, at));
        let special = record
            .get(VATS)
            .filter(|s| s.data.len() >= 18)
            .map(|s| WeaponSpecial {
                effect: Some(rr.plugin.to_global(FormId(le_u32(&s.data, 0)))).filter(|f| f.0 != 0),
                skill: le_f32(&s.data, 4),
                damage_mult: le_f32(&s.data, 8),
                ap: le_f32(&s.data, 12),
                silent: s.data[16] != 0,
                mod_required: s.data[17] != 0,
            });
        Some(WeaponVats {
            to_hit: dnam.get(40).copied().unwrap_or(0),
            ap: f(68).unwrap_or(0.0),
            strength_req: u(168).unwrap_or(0),
            skill_req: u(200).unwrap_or(0),
            special,
        })
    }
}

/// The perk tabs for an attack with `weapon` (or the default unarmed one).
fn weapon_tab(weapon: Option<&Weapon>) -> Tab {
    Tab::Weapon(weapon.map_or(FISTS, |w| w.form_id))
}

/// What an attack with `weapon` (`None`: fists) costs the player
/// (`0066dce0`): the weapon's own cost (`DNAM` f32 at 68) when its second
/// flags have [`flags::OVERRIDE_AP`], else the setting for its kind
/// (fists: `fActionPointsAttackUnarmed` 22), then the player's perks'
/// "Action Point Cost" (entry point 40, asked about the weapon: Fast Shot
/// × 0.8 with Guns or Energy Weapons). God mode (0) isn't modelled.
pub fn attack_cost(
    order: &LoadOrder,
    state: &GameState,
    s: &Settings,
    weapon: Option<&Weapon>,
) -> f32 {
    let k = attack_kind(weapon.map(|w| w.animation));
    let base = match weapon {
        Some(w) if w.flags2 & flags::OVERRIDE_AP != 0 => {
            WeaponVats::load(order, w.form_id).map_or(0.0, |v| v.ap)
        }
        _ => s.cost(k),
    };
    perks::apply_with(
        order,
        state,
        perks::entry::ACTION_POINT_COST,
        base,
        &[weapon_tab(weapon)],
    )
}

/// A special attack on offer (`007eb920`).
#[derive(Debug, Clone, PartialEq)]
pub struct Special {
    /// [`kind::SPECIAL`], [`kind::UPPERCUT`], [`kind::STOMP`] or
    /// [`kind::CROSS`].
    pub kind: u8,
    pub name: String,
    pub cost: f32,
}

/// The special attacks the player can make (`007eb920`): a weapon whose
/// `VATS` record costs more than 0 once the player's skill with it (whole
/// points, the actor value owner's `+8`) is at least the record's (its
/// cost through perk entry 40; its label the weapon's V.A.T.S. attack
/// name, `VANM`, the form's `+0x368` that `00522be0` reads: the super
/// sledge's "Mauler", the machete's "Back Slash"); unarmed (no weapon, or
/// one using Unarmed) with Unarmed at least `fUpperCutThreshold` (50):
/// Stomp on someone down, else Uppercut (`fUpperCutAPCost` 20), and at
/// least `fCrossThreshold` (75) Cross (`fCrossAPCost` 20) when not
/// stomping.
pub fn specials(
    order: &LoadOrder,
    state: &GameState,
    s: &Settings,
    weapon: Option<&Weapon>,
    target_down: bool,
) -> Vec<Special> {
    let facts = Facts {
        order,
        state,
        speaker: None,
    };
    let av = |a: u16| {
        facts
            .current_actor_value(PLAYER_REF, a)
            .unwrap_or(0.0)
            .trunc() as f32
    };
    let cost = |c: f32| {
        perks::apply_with(
            order,
            state,
            perks::entry::ACTION_POINT_COST,
            c,
            &[weapon_tab(weapon)],
        )
    };
    let mut out = Vec::new();
    if let Some(w) = weapon {
        if let Some(sp) = WeaponVats::load(order, w.form_id).and_then(|v| v.special) {
            if sp.ap > 0.0 && av(w.skill) >= sp.skill {
                out.push(Special {
                    kind: kind::SPECIAL,
                    name: attack_name(order, w.form_id).unwrap_or_default(),
                    cost: cost(sp.ap),
                });
            }
        }
    }
    let unarmed = weapon.map_or(true, |w| w.skill == crate::combat::av::UNARMED);
    let skill = av(crate::combat::av::UNARMED);
    if unarmed && skill >= s.uppercut_threshold {
        let (k, name) = if target_down {
            (kind::STOMP, "Stomp")
        } else {
            (kind::UPPERCUT, "Uppercut")
        };
        out.push(Special {
            kind: k,
            name: name.into(),
            cost: cost(s.uppercut_cost),
        });
        if !target_down && skill >= s.cross_threshold {
            out.push(Special {
                kind: kind::CROSS,
                name: "Cross".into(),
                cost: cost(s.cross_cost),
            });
        }
    }
    out
}

/// A weapon's V.A.T.S. attack name (`VANM`, the form's `+0x368`): the
/// label of its special (`007eb920`).
pub fn attack_name(order: &LoadOrder, weapon: FormId) -> Option<String> {
    let rr = order
        .get(weapon)
        .filter(|r| r.entry.header.kind.as_bytes() == b"WEAP")?;
    let record = rr.record().ok()?;
    record.get(VANM).map(|s| s.zstring())
}

/// The animation group a queued melee attack plays (`00948310`, the
/// player's attack in V.A.T.S.): a weapon's special its forward power
/// attack (`AttackForwardPower`; the weapon's own forward power attack
/// when it names one, `0094a0a0`, and `AttackPower` when the group isn't
/// there, are the animation's); Uppercut `Attack6` unless sneaking; Cross
/// `Attack7`; Stomp `stomp` (0xa9). Others keep the ordinary attack.
// Translated from 00948310 (decompiled, FalloutNV.exe 1.4.0.525)
pub fn attack_group(k: u8, sneaking: bool) -> Option<u8> {
    use crate::melee::group;
    match k {
        kind::SPECIAL => Some(group::ATTACK_FORWARD_POWER),
        kind::UPPERCUT if !sneaking => Some(group::ATTACK6),
        kind::CROSS => Some(group::ATTACK7),
        kind::STOMP => Some(group::STOMP),
        _ => None,
    }
}

/// The spell a queued special casts on the one it strikes (`0089a760`,
/// after the damage): for the player's weapon special or Uppercut while
/// a queued hit plays (as [`attack_damage_mult`] asks), the effect of the
/// weapon's `VATS` record (the form's `+0x370`, `0051f510`), cast by the
/// one struck on themselves (`00815b00`, `00824110`): the Mauler's
/// `MaulerKnockdownSpell`, Grand Slam's `VictoryRifleKnockdownSpell`.
// Translated from 0089a760 (decompiled, FalloutNV.exe 1.4.0.525)
pub fn special_effect(
    order: &LoadOrder,
    state: &GameState,
    attacker: FormId,
    weapon: Option<&Weapon>,
) -> Option<FormId> {
    let a = state.vats.attack.as_ref().filter(|a| a.hit)?;
    if attacker != PLAYER_REF || state.vats.mode != mode::PLAYBACK {
        return None;
    }
    if !matches!(a.kind, kind::SPECIAL | kind::UPPERCUT) {
        return None;
    }
    WeaponVats::load(order, weapon?.form_id)?.special?.effect
}

/// Shots in one attack (`007f5050`): an automatic weapon's (`DNAM` flags
/// 0x02) burst, round((`DNAM` f32 at 60 + a rate-of-fire mod) × `DNAM` f32
/// at 64 × `fVATSShotBurstTime` 0.43 s), at least 1, with the extra shots
/// of a long burst (`DNAM` second flags 0x800: `fVATSShotLongBurstTime`
/// 1.75 s) as the second number; any other weapon's ammo use (`DNAM` byte
/// 14), at least 1. The assault carbine (12 a second) 5; the 10mm SMG (9)
/// 4; the minigun (20, long burst) 9 and 26 more. Weapon mods aren't
/// modelled.
pub fn shots(s: &Settings, weapon: Option<&Weapon>) -> (u8, u8) {
    let Some(w) = weapon else {
        return (1, 0);
    };
    if w.flags1 & flags::AUTOMATIC == 0 {
        return (w.ammo_use.max(1), 0);
    }
    let rate = w.attack_mult * w.fire_rate;
    let count = |t: f32| (rate * t).round().clamp(1.0, 255.0) as u8;
    let short = count(s.shot_burst_time);
    let long = if w.flags2 & flags::LONG_BURST != 0 {
        count(s.shot_long_burst_time)
    } else {
        short
    };
    (short, long.saturating_sub(short))
}

/// The player's most action points (actor value 12, permanent:
/// `fAVDActionPointsBase` 65 + `fAVDActionPointsMult` 3 × Agility,
/// `006439d0`).
pub fn max_action_points(order: &LoadOrder, state: &GameState) -> f32 {
    Facts {
        order,
        state,
        speaker: None,
    }
    .permanent_actor_value(PLAYER_REF, ACTION_POINTS)
    .unwrap_or(0.0) as f32
}

/// The player's action points now, as V.A.T.S. shows them (at least 0).
pub fn action_points(order: &LoadOrder, state: &GameState) -> f32 {
    (Facts {
        order,
        state,
        speaker: None,
    }
    .current_actor_value(PLAYER_REF, ACTION_POINTS)
    .unwrap_or(0.0) as f32)
        .max(0.0)
}

/// Takes action points from the player (an attack that has played,
/// `009c7240`: player vfunc `+0x33c`).
pub fn spend(state: &mut GameState, amount: f32) {
    if amount > 0.0 {
        *state
            .value_damage
            .entry((PLAYER_REF, ACTION_POINTS))
            .or_insert(0.0) += f64::from(amount);
    }
}

/// Action points coming back (`0088b660`, from the player's magic update):
/// only while V.A.T.S. is off (mode 0), `fActionPointsRestoreRate` (0.06)
/// × seconds × the player's perks' "Action Point Regen" (entry point 39,
/// on 1) × the most action points, onto what's been taken (never past the
/// most: `0088b740`). 6% of the most a second: all of them in about 16.7
/// seconds.
pub fn regenerate(order: &LoadOrder, state: &mut GameState, seconds: f32) {
    if state.vats.mode != mode::OFF || seconds <= 0.0 {
        return;
    }
    let key = (PLAYER_REF, ACTION_POINTS);
    let Some(&taken) = state.value_damage.get(&key) else {
        return;
    };
    if taken <= 0.0 {
        return;
    }
    let rate = game_setting(order, "fActionPointsRestoreRate").unwrap_or(1.0);
    let perk = perks::apply(order, state, perks::entry::ACTION_POINT_REGEN, 1.0);
    let back = rate * seconds * perk * max_action_points(order, state);
    let left = (taken - f64::from(back)).max(0.0);
    if left == 0.0 {
        state.value_damage.remove(&key);
    } else {
        state.value_damage.insert(key, left);
    }
}

/// How the player stands, for the gun's wobble (`00646910`'s inputs).
/// While V.A.T.S. scans a target the game forces aiming on and sneaking off
/// (`findings\hits.md` §1); standing still is taken (the movement flags
/// then aren't traced: a guess).
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Stance {
    pub sneaking: bool,
    pub swimming: bool,
    pub walking: bool,
    pub running: bool,
    pub aiming: bool,
}

impl Stance {
    /// As V.A.T.S. scans: aiming, not sneaking, standing.
    pub const SCANNING: Stance = Stance {
        sneaking: false,
        swimming: false,
        walking: false,
        running: false,
        aiming: true,
    };
}

/// Someone's gun wobble (`00646910`, `findings\hits.md` §1; 0 without a
/// weapon): `fStandingSpreadPenalty` 0.1 when not sneaking, walking 0.1 or
/// running 0.2 (÷ perk entry 54 for the player, at least 1), not aiming
/// 0.2, crippled arms (both: 0.6 two-handed / 0.4; one: 0.4 two-handed,
/// 0.2 one-handed with the right one), Strength short of the weapon's
/// (`DNAM` u32 at 168, less perk entry 53) clamp(0, 10) ×
/// `fWeapStrengthReqPenalty` 0.025, skill short of its skill requirement
/// (`DNAM` u32 at 200) ceil(÷ 10) clamped (0, 10) × the same setting; all ×
/// (1 − skill × `fWobbleToSkillConversion` 0.5 / 100), at least
/// `fMinGunSpreadValue` 0.01, then perk entry 34 (the weapon's tab).
pub fn wobble(
    order: &LoadOrder,
    state: &GameState,
    s: &Settings,
    who: FormId,
    weapon: Option<&Weapon>,
    stance: Stance,
) -> f32 {
    let Some(w) = weapon else {
        return 0.0;
    };
    let facts = Facts {
        order,
        state,
        speaker: None,
    };
    let av = |a: u16| facts.current_actor_value(who, a).unwrap_or(0.0) as f32;
    let player = who == PLAYER_REF;
    let mut sum = 0.0;
    if !(stance.sneaking && !stance.swimming) {
        sum += s.standing_spread;
    }
    let pace = if player {
        perks::apply(order, state, perks::entry::MODIFY_AIMING_MOVE_SPEED, 1.0).max(1.0)
    } else {
        1.0
    };
    if stance.walking {
        sum += s.walking_spread / pace;
    } else if stance.running {
        sum += s.running_spread / pace;
    }
    if !stance.aiming {
        sum += s.unaimed_spread;
    }
    if av(crate::body_parts::av::IGNORE_CRIPPLED_LIMBS) <= 0.0 {
        let (left, right) = (av(27) <= 0.0, av(28) <= 0.0);
        let two = w.two_handed();
        if left && right {
            sum += if two {
                s.crippled_arms_2h
            } else {
                s.crippled_arms_1h
            };
        } else if left || right {
            if two {
                sum += s.crippled_arm_2h;
            } else if right {
                sum += s.crippled_arm_1h;
            }
        }
    }
    let extra = WeaponVats::load(order, w.form_id).unwrap_or_default();
    let strength = av(crate::combat::av::STRENGTH);
    let req = extra.strength_req as f32;
    if req >= strength {
        let perk = if player {
            perks::apply(order, state, perks::entry::MODIFY_WEAPON_STRENGTH_REQ, 0.0)
        } else {
            0.0
        };
        sum += (req - strength - perk).clamp(0.0, 10.0) * s.strength_req_penalty;
    }
    let skill = av(w.skill);
    let skill_req = extra.skill_req as f32;
    if skill <= skill_req {
        sum += ((skill_req - skill) / 10.0).ceil().clamp(0.0, 10.0) * s.strength_req_penalty;
    }
    sum *= 1.0 - skill * s.wobble_to_skill * 0.01;
    let sum = sum.max(s.min_gun_spread);
    if player {
        perks::apply_with(
            order,
            state,
            perks::entry::CALCULATE_GUN_SPREAD,
            sum,
            &[weapon_tab(weapon)],
        )
    } else {
        sum
    }
}

/// A target's size as V.A.T.S. measures it (`0050ebf0` × 2): the length of
/// the diagonal of its base's `OBND` box (Doc Mitchell's (−23, −17, 0)–(23,
/// 17, 132): 143.9), 128 when that's under 1.
pub fn bound_of(order: &LoadOrder, target: FormId) -> f32 {
    let is_base = order
        .get(target)
        .is_some_and(|r| matches!(r.entry.header.kind.as_bytes(), b"NPC_" | b"CREA"));
    let base = if is_base {
        Some(target)
    } else {
        base_of(order, target)
    };
    let length = base
        .and_then(|b| order.get(b))
        .and_then(|r| r.record().ok())
        .and_then(|r| {
            let s = r.get(OBND).filter(|s| s.data.len() >= 12)?;
            let v = |i: usize| f32::from(i16::from_le_bytes([s.data[i * 2], s.data[i * 2 + 1]]));
            Some(((v(3) - v(0)).powi(2) + (v(4) - v(1)).powi(2) + (v(5) - v(2)).powi(2)).sqrt())
        })
        .unwrap_or(0.0);
    if length < 1.0 {
        128.0
    } else {
        length
    }
}

/// How the attack aims, for [`hit_chance`].
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Aim {
    /// No weapon, or a melee one (animation types 0–2).
    Melee,
    /// Grenades, mines and thrown weapons (10–13): the weapon's max range
    /// (`DNAM` 48), the player's skill with it, whether their arms work
    /// (the arm factor above 0, `00646880`).
    Thrown {
        max_range: f32,
        skill: f32,
        arms: bool,
    },
    /// Guns (3–9): the spread in degrees (the weapon's min spread, `DNAM`
    /// 16, + the player's wobble × `fNPCMaxGunWobbleAngle`), and the max
    /// range when the weapon's range is fixed (`DNAM` second flags 0x20).
    Gun {
        spread: f32,
        fixed_range: Option<f32>,
    },
}

/// What one chance to hit takes (`00646f70`'s arguments).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ChanceInputs {
    pub aim: Aim,
    /// Between the bodies' edges (`009a64d0`: centre to centre less both
    /// radii).
    pub distance: f32,
    /// [`bound_of`].
    pub bound: f32,
    /// The share of the part on screen that isn't hidden (0–1).
    pub visible: f32,
    /// The part's to-hit chance (`BPND` byte 8), else the target's weapon's
    /// (`DNAM` byte 40), else 1.
    pub base: f32,
    /// The projectile goes off by proximity (`PROJ` `DATA` f32 at 28 not
    /// 0: mines): no chance at all.
    pub proximity: bool,
    /// A destructible object, a live grenade (objects aren't targeted
    /// here).
    pub destructible: bool,
    pub grenade: bool,
    /// The target has Chameleon (actor value 49) or the attacker
    /// Invisibility (48) — as the code reads it — and the player lacks
    /// perk entry 69.
    pub stealthed: bool,
}

/// The chance to hit (0–1), as `00646f70` works it out:
/// - melee and unarmed: 100% within `fVATSMeleeMaxDistance` (300) of the
///   edges, else none;
/// - a proximity projectile: none;
/// - thrown: none beyond max range × `fVATSThrownWeaponRangeMult` (2); else
///   (1 − max(d − 128, 0) / (2 × max range − 128) + ((1 − 1) + 0.4 ×
///   skill/100) × (1, or 0.8 with a crippled arm)) × 100
///   (`fVATSGrenadeRangeMin`, `…RangeMult`, `fVATSSkillFactor`,
///   `fVATSGrenadeSkillFactor`, `fCrippledArm1HSpreadPenalty`,
///   `fVATSGrenadeChanceMult`);
/// - guns: none past a fixed range; else r = tan(spread ×
///   `fVATSSpreadMult` 1.1) × d, the target's area T = bound² / 3 (× 2r /
///   bound when 2r < bound; `fVATSGrenadeTargetArea` for a grenade; ×
///   `fVATSDestructibleMult` 1.25 for a destructible), and the chance =
///   `fVATSHitChanceMult` × visible × min(T / πr², `fVATSRangeSpreadMax`
///   100) × base, in percent (base is the part's percentage);
/// - a destructible or grenade target: × 100, at least 1 if above 0;
/// - × `fVATSStealthMult` 0.5 when stealthed;
/// - at most `fVATSMaxChance` 95, then ÷ 100. No rounding, no floor.
pub fn hit_chance(s: &Settings, i: &ChanceInputs) -> f32 {
    let factor = s.screen_percent_factor.min(1.0);
    let visible = (1.0 - factor) + factor * i.visible;
    let mut p = match i.aim {
        Aim::Melee => {
            if i.distance <= s.melee_max_distance {
                100.0
            } else {
                0.0
            }
        }
        _ if i.proximity => return 0.0,
        Aim::Thrown {
            max_range,
            skill,
            arms,
        } => {
            if max_range * s.thrown_range_mult < i.distance {
                return 0.0;
            }
            let gap = max_range * s.grenade_range_mult - s.grenade_range_min;
            let odds = 1.0 - (i.distance - s.grenade_range_min).max(0.0) / gap;
            let arm = if arms { 1.0 } else { 1.0 - s.crippled_arm_1h };
            let skill_term =
                ((1.0 - s.skill_factor) + s.grenade_skill_factor * skill / 100.0) * arm;
            (odds + skill_term) * s.grenade_chance_mult * 100.0
        }
        Aim::Gun {
            spread,
            fixed_range,
        } => {
            if fixed_range.is_some_and(|r| r < i.distance) {
                return 0.0;
            }
            let r = (spread * s.spread_mult).to_radians().tan() * i.distance;
            let area = r * r * PI;
            let mut target = i.bound / 3.0 * i.bound;
            if i.grenade {
                target = s.grenade_target_area;
            } else {
                if r + r < i.bound {
                    target *= (r + r) / i.bound;
                }
                if i.destructible {
                    target *= s.destructible_mult;
                }
            }
            let ratio = if area > 0.0 {
                (target / area).min(s.range_spread_max)
            } else {
                0.0
            };
            s.hit_chance_mult * visible * ratio * i.base
        }
    };
    if !matches!(i.aim, Aim::Melee) && (i.destructible || i.grenade) {
        p *= 100.0;
        if p > 0.0 && p < 1.0 {
            p = 1.0;
        }
    }
    if i.stealthed {
        p *= s.stealth_mult;
    }
    p.min(s.max_chance) / 100.0
}

/// The projectile a weapon's shots are: its current ammunition's (`AMMO`
/// `DAT2` form at 4), else its own (`DNAM` 36).
fn projectile_of(order: &LoadOrder, state: &GameState, weapon: &Weapon) -> Option<FormId> {
    let from_ammo = weapon
        .ammo_in_use(order, state, PLAYER_REF)
        .and_then(|a| order.get(a))
        .and_then(|rr| {
            let d = rr.record().ok()?.get(DAT2)?.data.clone();
            (d.len() >= 8)
                .then(|| rr.plugin.to_global(FormId(le_u32(&d, 4))))
                .filter(|f| f.0 != 0)
        });
    from_ammo.or(weapon.projectile)
}

/// Whether a projectile goes off by proximity (`PROJ` `DATA` f32 at 28).
fn proximity(order: &LoadOrder, projectile: FormId) -> bool {
    order
        .get(projectile)
        .and_then(|r| r.record().ok())
        .and_then(|r| r.get(esm::sig::DATA).map(|s| s.data.clone()))
        .is_some_and(|d| d.len() >= 32 && le_f32(&d, 28) != 0.0)
}

/// One part's chance, as the menu asks for it.
#[derive(Debug, Clone, Copy)]
pub struct ChanceQuery<'a> {
    pub target: FormId,
    /// The body part data's part for the entry's slot (none for the
    /// weapon when the body has no part for it, and for objects).
    pub part: Option<&'a BodyPart>,
    /// The part's actor value as the attack would carry it (for the perks'
    /// `GetVATSValue 5`).
    pub part_av: i32,
    /// The player's weapon (`None`: fists).
    pub weapon: Option<&'a Weapon>,
    /// Between the bodies' edges.
    pub distance: f32,
    /// The part's visible share (0–1).
    pub visible: f32,
    pub stance: Stance,
}

/// A part's chance to hit (0–1) as the menu stores it (`007f1290`): the
/// target's size, the player's wobble (mode 3: × `fNPCMaxGunWobbleAngle`
/// 15; 1° without a weapon), the part's to-hit chance (else the target's
/// weapon's, else 1), its visible share and the distance into
/// [`hit_chance`]; then, with the attack's template filled in (so perks can
/// ask `GetVATSValue`), the player's perks' "Calculate To Hit Chance"
/// (entry point 8, asked about the weapon, or `Fists`, and the target), of
/// which the larger is kept: perks only raise it (Commando × 1.25 for
/// rifles, Sniper × 1.25 at the head).
pub fn part_chance(order: &LoadOrder, state: &mut GameState, s: &Settings, q: &ChanceQuery) -> f32 {
    let weapon = q.weapon;
    let extra_spread = match weapon {
        Some(_) => wobble(order, state, s, PLAYER_REF, weapon, q.stance) * s.npc_max_gun_wobble,
        None => 1.0,
    };
    let held = crate::combat::weapon_in_hand(order, state, q.target);
    let base = match q.part {
        Some(p) => f32::from(p.to_hit_chance),
        None => held
            .as_ref()
            .and_then(|w| {
                // With a V.A.T.S. mod, its value more (`00647730`).
                let extra = crate::weapon_mods::bonus(
                    order,
                    crate::weapon_mods::flags(state, q.target, w.form_id),
                    w.form_id,
                    crate::weapon_mods::effect::VATS_BONUS,
                )
                .map_or(0, |v| crate::weapon_mods::round(v) & 0xff);
                WeaponVats::load(order, w.form_id).map(|v| (i32::from(v.to_hit) + extra) as f32)
            })
            .unwrap_or(1.0),
    };
    let aim = match weapon {
        None => Aim::Melee,
        Some(w) if w.is_melee() => Aim::Melee,
        Some(w) if (3..=9).contains(&w.animation) => Aim::Gun {
            spread: w.min_spread + extra_spread,
            fixed_range: (w.flags2 & flags::FIXED_RANGE != 0).then_some(w.max_range),
        },
        Some(w) => Aim::Thrown {
            max_range: w.max_range,
            skill: Facts {
                order,
                state,
                speaker: None,
            }
            .current_actor_value(PLAYER_REF, w.skill)
            .unwrap_or(0.0) as f32,
            arms: thrown_arms(order, state, w),
        },
    };
    let facts = Facts {
        order,
        state,
        speaker: None,
    };
    let av = |who: FormId, a: u16| facts.current_actor_value(who, a).unwrap_or(0.0);
    let improved = perks::apply(order, state, perks::entry::HAS_IMPROVED_DETECTION, 0.0) > 0.0;
    let stealthed = (av(q.target, 49) > 0.0 || av(PLAYER_REF, 48) > 0.0) && !improved;
    let inputs = ChanceInputs {
        aim,
        distance: q.distance,
        bound: bound_of(order, q.target),
        visible: q.visible,
        base,
        proximity: weapon
            .filter(|w| !w.is_melee())
            .and_then(|w| projectile_of(order, state, w))
            .is_some_and(|p| proximity(order, p)),
        destructible: false,
        grenade: false,
        stealthed,
    };
    let chance = hit_chance(s, &inputs);
    // The template the perks' conditions ask about (`007f1290`: target,
    // part; `009b5050` fills the rest).
    let saved = state.vats.attack.replace(AttackFacts {
        weapon: weapon.map(|w| w.form_id),
        target: q.target,
        part_av: q.part_av,
        kind: attack_kind(weapon.map(|w| w.animation)),
        hit: false,
        distance: q.distance,
        paralyzing_palm: false,
    });
    let perked = perks::apply_with(
        order,
        state,
        perks::entry::CALCULATE_TO_HIT_CHANCE,
        chance,
        &[weapon_tab(weapon), Tab::Target(q.target)],
    );
    state.vats.attack = saved;
    chance.max(perked)
}

/// Whether the player's arms let them throw (`00646880` above 0: both
/// arms for two-handed kinds, else the right one; ignoring crippled
/// limbs counts as working).
fn thrown_arms(order: &LoadOrder, state: &GameState, weapon: &Weapon) -> bool {
    if crate::body_parts::ignores_crippled_limbs(order, state, PLAYER_REF) {
        return true;
    }
    let up = |a: u16| !crate::body_parts::is_crippled(order, state, PLAYER_REF, a);
    if matches!(weapon.animation, 0 | 2 | 5..=9) {
        up(27) || up(28)
    } else {
        up(28)
    }
}

/// Whether the player has Concentrated Fire (perk entry 33 above 0,
/// `007f1290` → menu `+0x104`).
pub fn concentrated_fire(order: &LoadOrder, state: &GameState) -> bool {
    perks::apply(order, state, perks::entry::HAS_CONCENTRATED_FIRE, 0.0) != 0.0
}

/// The percentage a part shows (`007f0ea0`): max(0, min(trunc(100 ×
/// chance + `iVATSConcentratedFireBonus` 5 × n), 95)), n the attacks
/// already queued in a row on it (only with Concentrated Fire); 95 is
/// written into the code, not the setting.
pub fn shown_percent(s: &Settings, chance: f32, in_a_row: u32, concentrated_fire: bool) -> u32 {
    let bonus = if concentrated_fire {
        s.concentrated_fire_bonus * in_a_row as f32
    } else {
        0.0
    };
    ((100.0 * chance + bonus) as i64).clamp(0, 95) as u32
}

/// One part a target offers (the menu's part entry, `007f6820`).
#[derive(Debug, Clone, PartialEq)]
pub struct PartEntry {
    /// The body part data's slot (0–14, 14 the weapon held; -1 the whole
    /// of something without parts).
    pub slot: i8,
    /// `BPTN` ("Left Arm"); "Weapon".
    pub name: String,
    /// The node it's aimed at (`BPNT`, "Bip01 L Forearm"); the weapon's
    /// own model for the weapon ("Weapon" here).
    pub node: String,
    /// Its condition's actor value (25–31; -1 the weapon; 26 for a whole
    /// object).
    pub actor_value: i32,
    /// The chance stored once scanned (0–1; 0 until then).
    pub chance: f32,
    pub scanned: bool,
}

/// The parts a target offers (`007f52c0`): one per slot 0–14 its body part
/// data has, in slot order, plus the weapon it holds (slot 14) unless that
/// is built in (`DNAM` flags 0x20, `0046e8c0`); with no body part data,
/// one entry for the whole of it.
pub fn parts_offered(data: Option<&BodyPartData>, held: Option<&Weapon>) -> Vec<PartEntry> {
    let mut out: Vec<PartEntry> = data
        .map(|d| {
            d.parts
                .iter()
                .enumerate()
                .filter_map(|(slot, p)| {
                    let p = p.as_ref()?;
                    Some(PartEntry {
                        slot: slot as i8,
                        name: p.name.clone(),
                        node: p.target.clone(),
                        actor_value: if slot == usize::from(part::WEAPON) {
                            -1
                        } else {
                            i32::from(p.actor_value)
                        },
                        chance: 0.0,
                        scanned: false,
                    })
                })
                .collect()
        })
        .unwrap_or_default();
    if held.is_some_and(|w| w.flags1 & flags::EMBEDDED == 0) {
        out.push(PartEntry {
            slot: part::WEAPON as i8,
            name: "Weapon".into(),
            node: "Weapon".into(),
            actor_value: -1,
            chance: 0.0,
            scanned: false,
        });
    }
    if out.is_empty() {
        out.push(PartEntry {
            slot: -1,
            name: "Target".into(),
            node: String::new(),
            actor_value: 26,
            chance: 0.0,
            scanned: false,
        });
    }
    out
}

/// Parts sharing an actor value show the same, highest, chance
/// (`007f0ea0`: each part of slot 1–12 takes a later one's chance when
/// it's higher).
pub fn share_highest(parts: &mut [PartEntry]) {
    for i in 0..parts.len() {
        if !(1..13).contains(&parts[i].slot) {
            continue;
        }
        for j in i + 1..parts.len() {
            if parts[j].actor_value == parts[i].actor_value && parts[j].chance > parts[i].chance {
                parts[i].chance = parts[j].chance;
            }
        }
    }
}

/// After 2 seconds of scanning (`007f3e00`), parts not scanned yet (off
/// screen) whose chance isn't above 0 get the average of those that were.
pub fn fill_unscanned(parts: &mut [PartEntry]) {
    let done: Vec<f32> = parts
        .iter()
        .filter(|p| p.scanned)
        .map(|p| p.chance)
        .collect();
    if done.is_empty() {
        return;
    }
    let average = done.iter().sum::<f32>() / done.len() as f32;
    for p in parts.iter_mut().filter(|p| !p.scanned && p.chance <= 0.0) {
        p.chance = average;
        p.scanned = true;
    }
}

/// The part chosen once a target is scanned (`007f3e00`): the head (actor
/// value 25) when the player is within 300 units, else the part last
/// aimed at (`[011a59f0]`, the head to begin with), else the torso (26),
/// else the first.
pub fn default_part(parts: &[PartEntry], distance: f32, last_aimed: i32) -> usize {
    let find = |av: i32| parts.iter().position(|p| p.actor_value == av);
    let near = (distance <= 300.0).then(|| find(25)).flatten();
    near.or_else(|| find(last_aimed))
        .or_else(|| find(26))
        .unwrap_or(0)
}

/// Someone V.A.T.S. might target, as the viewer sees them.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Candidate {
    pub reference: FormId,
    pub alive: bool,
    /// Centre to centre (`005723b0`).
    pub distance: f32,
    pub line_of_sight: bool,
    /// Their root, or one of their parts' nodes, inside the screen.
    pub on_screen: bool,
}

/// Whether someone can be targeted (`007f52c0`): alive, 2 to
/// `fVATSMaxEngageDistance` (5000) away, in line of sight, and on screen
/// unless within the sneak distance (`fSneakMaxDistance` 2500 indoors,
/// × `fSneakExteriorDistanceMult` 2 outdoors); not one whose base is in
/// `BannedVATSTargets` (`007e9200`).
pub fn eligible(order: &LoadOrder, s: &Settings, c: &Candidate, interior: bool) -> bool {
    if !c.alive || c.distance < 2.0 || c.distance > s.max_engage_distance || !c.line_of_sight {
        return false;
    }
    let sneak = if interior {
        s.sneak_max_distance
    } else {
        s.sneak_max_distance * s.sneak_exterior_mult
    };
    if !c.on_screen && c.distance > sneak {
        return false;
    }
    !banned_target(order, c.reference)
}

/// Whether a reference's base is in `BannedVATSTargets`.
pub fn banned_target(order: &LoadOrder, reference: FormId) -> bool {
    base_of(order, reference).is_some_and(|b| perks::form_list(order, BANNED_TARGETS).contains(&b))
}

/// Whether a weapon is in `VATSBannedWeaponsList`.
pub fn banned_weapon(order: &LoadOrder, weapon: FormId) -> bool {
    perks::form_list(order, BANNED_WEAPONS).contains(&weapon)
}

/// The target V.A.T.S. opens on (`007e9200`), from each target's angle off
/// the player's heading (degrees, 0–180) and distance: scored (180 −
/// angle) / 180 + 40 / distance (a distance at most 0 taken as 0.001); in
/// list order a target is taken when it scores higher (until one inside
/// `iVatsTargetAngle` 15° is taken, which stops taking by score), or when
/// it's inside that angle and nearer than the one taken. (Each target's
/// byte `+0x20`, which the second test also asks, is taken to be set: it's
/// copied from the reference's form type, never 0: inferred.)
pub fn first_target(s: &Settings, targets: &[(f32, f32)]) -> Option<usize> {
    let mut best: Option<usize> = None;
    let mut best_score = 0.0;
    let mut locked = false;
    for (i, &(angle, distance)) in targets.iter().enumerate() {
        let d = if distance <= 0.0 { 0.001 } else { distance };
        let score = (180.0 - angle) / 180.0 + 40.0 / d;
        let inside = angle < s.target_angle;
        let take = if score <= best_score || locked {
            inside && best.map_or(true, |b| targets[b].1 > distance)
        } else {
            true
        };
        if take {
            if inside {
                locked = true;
            }
            best_score = score;
            best = Some(i);
        }
    }
    best
}

/// Why V.A.T.S. didn't open (`00942800`, `007e9200`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EnterRefusal {
    /// Already on, a menu open, swimming, or the fighting controls off:
    /// nothing happens.
    Busy,
    /// The weapon is in `VATSBannedWeaponsList`.
    BannedWeapon,
    /// Nobody to target: the `UIVATSEnterFail` sound.
    NoTargets,
}

/// Whether V.A.T.S. can open (`00942800`): not already on, no menu, not
/// swimming, the fighting controls on (`DisablePlayerControls`'s third),
/// a weapon not banned. No action points are needed.
pub fn can_enter(
    order: &LoadOrder,
    state: &GameState,
    weapon: Option<&Weapon>,
    swimming: bool,
    menu_open: bool,
) -> Result<(), EnterRefusal> {
    if state.vats.mode != mode::OFF
        || menu_open
        || swimming
        || state.controls_off[crate::scripting::controls::FIGHTING]
    {
        return Err(EnterRefusal::Busy);
    }
    if weapon.is_some_and(|w| banned_weapon(order, w.form_id)) {
        return Err(EnterRefusal::BannedWeapon);
    }
    Ok(())
}

/// A queued attack (`009ca4a0`).
#[derive(Debug, Clone, PartialEq)]
pub struct QueuedAttack {
    /// [`kind`].
    pub kind: u8,
    /// Rolled when queued.
    pub hit: bool,
    /// Shots (a burst's), and a long burst's extra ones.
    pub shots: u8,
    pub extra_shots: u8,
    pub target: FormId,
    /// The part: its slot in the target's list and its actor value.
    pub slot: i8,
    pub part_av: i32,
    /// Its cost, a reload's included.
    pub ap: f32,
    /// A "Reload" line follows it in the queue (`007efa10`): its shots run
    /// the projected clip out and there's more to load. Playing, the
    /// weapon reloads itself once empty, as it does outside V.A.T.S.
    pub reload: bool,
    pub paralyzing_palm: bool,
}

/// What queueing one attack needs to know (`007ec810`).
#[derive(Debug, Clone, PartialEq)]
pub struct Attempt {
    pub target: FormId,
    pub is_actor: bool,
    /// The selected part: slot and actor value.
    pub slot: i8,
    pub part_av: i32,
    /// The selected part's stored chance (0–1).
    pub chance: f32,
    /// [`attack_kind`], or a special's.
    pub kind: u8,
    /// [`attack_cost`] (or the special's).
    pub cost: f32,
    /// [`shots`].
    pub shots: (u8, u8),
    /// With a weapon: whether it's melee (no weapon counts as melee).
    pub melee: bool,
    pub unarmed: bool,
    /// A gun's clip size, when it uses ammunition.
    pub clip_size: Option<u32>,
    /// Its rounds a shot (`DNAM` byte 14); 1 for anything else.
    pub ammo_use: u8,
    /// Grenades and thrown weapons: how many are carried.
    pub thrown_left: Option<u32>,
    /// The target's part entries (slot, actor value), for melee's random
    /// part.
    pub parts: Vec<(i8, i32)>,
    pub concentrated_fire: bool,
    pub paralyzing_palm: bool,
    /// `bVatsAlwaysHit:Combat` (the INI).
    pub always_hit: bool,
}

/// Why an attack wasn't queued (`007ec810`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Refusal {
    /// The part's chance shows under 1%: `sVATSMessageZeroChance`.
    ZeroChance,
    /// More than the action points left: the `UIVATSInsufficientAP` sound.
    NotEnoughAp,
    /// No rounds (or nothing to throw): `sVATSMessageNoAmmo`.
    NoAmmo,
    /// A kind that can't attack.
    CantAttack,
}

impl Refusal {
    /// The game's words (the exe's defaults; `FalloutNV.esm` doesn't
    /// change them).
    pub fn message(self) -> Option<&'static str> {
        match self {
            Refusal::ZeroChance => Some("You have zero chance to hit."),
            Refusal::NoAmmo => Some("You don't have enough ammo."),
            _ => None,
        }
    }

    /// The sound it makes (`SOUN` editor IDs).
    pub fn sound(self) -> Option<&'static str> {
        match self {
            Refusal::ZeroChance | Refusal::NoAmmo => Some("UIVATSEnterFail"),
            Refusal::NotEnoughAp => Some("UIVATSInsufficientAP"),
            Refusal::CantAttack => None,
        }
    }
}

/// The menu's plan: the queue and what's left after it (menu `+0xe0`
/// action points, `+0xf0` rounds in the clip, `+0xf4` rounds carried
/// besides).
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Plan {
    pub ap_left: f32,
    pub clip: u32,
    pub reserve: u32,
    pub attacks: Vec<QueuedAttack>,
    /// What each attack changed, to undo it.
    undo: Vec<(f32, u32, u32)>,
}

impl Plan {
    pub fn new(ap: f32, clip: u32, reserve: u32) -> Plan {
        Plan {
            ap_left: ap,
            clip,
            reserve,
            attacks: Vec::new(),
            undo: Vec::new(),
        }
    }

    /// Attacks at the end of the queue in a row on this target and part
    /// (`007ef930`; reload lines aren't separate entries here).
    pub fn in_a_row(&self, target: FormId, part_av: i32) -> u32 {
        self.attacks
            .iter()
            .rev()
            .take_while(|a| a.target == target && a.part_av == part_av)
            .count() as u32
    }

    /// Queues an attack (`007ec810`), rolling its hit now: refused when
    /// the part's chance shows under 1%, it costs more than the action
    /// points left, there's nothing to fire or throw, or the kind can't
    /// attack. **The reload** (read from `007ec810` l.563–567 and 866–877,
    /// `007f28d0`, `007efa10`): a gun attack whose shots are at least the
    /// rounds the projected clip holds (the clip more than 1 round) costs
    /// `fActionPointsReload` 10 more; the projection then takes its shots
    /// off the clip and, with the clip run out, refills it from the rounds
    /// carried (the last gun attack's "ran out" flag is what `007f28d0`
    /// returns); a "Reload" line follows the attack when that happened,
    /// unless the clip holds just one shot's worth (clip size = ammo use)
    /// or nothing is left to load. So the reload is charged on, and
    /// listed after, the attack that empties the clip, not before the
    /// next one. Melee (and fists) aim at a random part among slots 0–6
    /// (torso, heads, arms: never legs), number trunc(max(r, 1) × count /
    /// 100) for r = U(0, 100), Uppercut and Stomp at the head; a melee kind
    /// on someone without those parts at the head, else the torso, else
    /// their first part. Paralyzing Palm: unarmed, with U(0, 100) ≤
    /// `fVATSParalyzePalmChance` 0.3 × 100. Hit when the chance (+ 0.05 ×
    /// the attacks before it in a row on the part, with Concentrated Fire)
    /// ≥ U[0, 1) (`004dff20`). `rand` gives U[0, 1).
    pub fn queue(
        &mut self,
        s: &Settings,
        a: &Attempt,
        rand: &mut dyn FnMut() -> f32,
    ) -> Result<&QueuedAttack, Refusal> {
        let (short, extra) = a.shots;
        let reload_cost = a
            .clip_size
            .is_some_and(|size| (self.clip as f32) <= f32::from(short) && size > 1);
        let cost = if reload_cost {
            a.cost + s.cost(kind::RELOAD)
        } else {
            a.cost
        };
        let no_ammo = a.clip_size.is_some() && self.clip + self.reserve == 0;
        let nothing_to_throw = a.thrown_left.is_some_and(|n| n == 0);
        let zero = ((a.chance * 100.0) as i64) < 1;
        if zero {
            return Err(Refusal::ZeroChance);
        }
        if cost > self.ap_left {
            return Err(Refusal::NotEnoughAp);
        }
        if no_ammo || nothing_to_throw {
            return Err(Refusal::NoAmmo);
        }
        if a.kind == kind::NONE {
            return Err(Refusal::CantAttack);
        }
        // The part.
        let mut part = (a.slot, a.part_av);
        let mut picked = false;
        if a.melee {
            let r = rand() * 100.0;
            let close: Vec<(i8, i32)> = a
                .parts
                .iter()
                .copied()
                .filter(|(slot, _)| (0..=6).contains(slot))
                .collect();
            let head = close
                .iter()
                .copied()
                .find(|(slot, _)| (1..=2).contains(slot));
            let special_two = matches!(a.kind, kind::UPPERCUT | kind::STOMP);
            if a.unarmed && special_two && head.is_some() {
                part = head.unwrap_or(part);
                picked = true;
            } else if !close.is_empty() {
                let n = (r.max(1.0) * close.len() as f32 / 100.0) as usize;
                if let Some(&p) = close.get(n) {
                    part = p;
                    picked = true;
                }
            }
        }
        if is_melee_kind(a.kind) && a.is_actor && !picked {
            let by_av = |av: i32| a.parts.iter().copied().find(|(_, v)| *v == av);
            if let Some(p) = by_av(25)
                .or_else(|| by_av(26))
                .or_else(|| a.parts.first().copied())
            {
                part = p;
            }
        }
        let paralyzing_palm = a.paralyzing_palm
            && a.kind == kind::UNARMED
            && rand() * 100.0 <= s.paralyze_palm_chance * 100.0;
        // The roll.
        let bonus = if a.concentrated_fire {
            s.concentrated_fire_bonus * self.in_a_row(a.target, part.1) as f32 / 100.0
        } else {
            0.0
        };
        let hit = a.always_hit || a.chance + bonus >= rand();
        self.undo.push((self.ap_left, self.clip, self.reserve));
        self.ap_left -= cost;
        // The projection (`007f28d0`): the shots come off the clip; run
        // out, it refills from the reserve.
        let mut reload = false;
        if let Some(size) = a.clip_size {
            self.clip = self.clip.saturating_sub(u32::from(short));
            if self.clip == 0 {
                let loaded = size.min(self.reserve);
                self.clip = loaded;
                self.reserve -= loaded;
                reload = size != u32::from(a.ammo_use) && self.clip + self.reserve > 0;
            }
        }
        self.attacks.push(QueuedAttack {
            kind: a.kind,
            hit,
            shots: short,
            extra_shots: extra,
            target: a.target,
            slot: part.0,
            part_av: part.1,
            ap: cost,
            reload,
            paralyzing_palm,
        });
        Ok(self.attacks.last().expect("just pushed"))
    }

    /// Takes the last attack back (its action points and rounds too).
    pub fn undo(&mut self) -> Option<QueuedAttack> {
        let a = self.attacks.pop()?;
        if let Some((ap, clip, reserve)) = self.undo.pop() {
            self.ap_left = ap;
            self.clip = clip;
            self.reserve = reserve;
        }
        Some(a)
    }
}

/// How far off a missed shot is aimed (`00965620`): an angle of the
/// target's bound radius (16 when 0; taken as half of [`bound_of`]: the
/// game's is its model's bound, a guess) × U(`fAutoAimMissRatioLow` 1,
/// `…High` 1.3) / distance, in a random direction, each of the turn's two
/// parts (heading, pitch) at most `fAutoAimMaxDegreesMiss` (3°). `u1` and
/// `u2` are U[0, 1); radians (heading, pitch).
pub fn miss_offset(s: &Settings, bound_radius: f32, distance: f32, u1: f32, u2: f32) -> (f32, f32) {
    let radius = if bound_radius > 0.0 {
        bound_radius
    } else {
        16.0
    };
    let ratio = s.miss_ratio_low + (s.miss_ratio_high - s.miss_ratio_low) * u1;
    let angle = radius * ratio / distance.max(1.0);
    let theta = 2.0 * PI * u2;
    let most = s.auto_aim_max_miss_degrees.to_radians();
    (
        (angle * theta.cos()).clamp(-most, most),
        (angle * theta.sin()).clamp(-most, most),
    )
}

/// How far from the target a melee attack puts the player (`009c9280`):
/// the reach (`009a69c0`: the weapon's reach × 128 × scale, 64 unarmed) ×
/// `fVATSMeleeWarpDistanceMult` 0.32 (`fVATSH2HWarpDistanceMult` 0.27
/// unarmed).
pub fn warp_distance(s: &Settings, weapon: Option<&Weapon>, scale: f32) -> f32 {
    let reach = Weapon::melee_reach(weapon) * scale;
    let unarmed = weapon.map_or(true, |w| w.animation == 0);
    reach
        * if unarmed {
            s.h2h_warp_mult
        } else {
            s.melee_warp_mult
        }
}

/// V.A.T.S.'s mode changes (`009c6c30`): off clears the attack.
pub fn set_mode(state: &mut GameState, m: u8) {
    state.vats.mode = m;
    if m == mode::OFF {
        state.vats.attack = None;
    }
}

/// An attack starts playing (`009c8e00`): what `GetVATSValue` and the hits
/// now see.
pub fn begin_attack(state: &mut GameState, facts: AttackFacts) {
    state.vats.mode = mode::PLAYBACK;
    state.vats.attack = Some(facts);
}

/// An attack has played (`009c7240`): its action points are taken.
pub fn finish_attack(state: &mut GameState, attack: &QueuedAttack) {
    spend(state, attack.ap);
    state.vats.attack = None;
}

/// Playback is over (`009c8950`): V.A.T.S. is off, and if the player
/// killed anyone while it played, the perks' "Player Kill AP Reward"
/// (entry point 35, on 0: Grim Reaper's Sprint 20) comes back.
pub fn end(order: &LoadOrder, state: &mut GameState, kills: u32) {
    set_mode(state, mode::OFF);
    if kills > 0 {
        let reward = perks::apply(order, state, perks::entry::PLAYER_KILL_AP_REWARD, 0.0);
        if reward > 0.0 {
            let key = (PLAYER_REF, ACTION_POINTS);
            let taken = state.value_damage.get(&key).copied().unwrap_or(0.0);
            let left = (taken - f64::from(reward)).max(0.0);
            state.value_damage.insert(key, left);
        }
    }
}

/// What the player takes while V.A.T.S. is on (`009b5a30`): ×
/// `fVATSPlayerDamageMult` (0.75); everyone else × 1.
pub fn player_damage_mult(order: &LoadOrder, state: &GameState, target: FormId) -> f32 {
    if target == PLAYER_REF && state.vats.mode != mode::OFF {
        game_setting(order, "fVATSPlayerDamageMult").unwrap_or(1.0)
    } else {
        1.0
    }
}

/// The critical chance's V.A.T.S. bonus (`009b7060`): the player's hits
/// from a queued hit while it plays get `fVATSCriticalChanceBonus` (5)
/// percentage points.
pub fn critical_bonus(order: &LoadOrder, state: &GameState, attacker: FormId) -> f32 {
    let vats_hit =
        state.vats.mode == mode::PLAYBACK && state.vats.attack.as_ref().is_some_and(|a| a.hit);
    if attacker == PLAYER_REF && vats_hit {
        game_setting(order, "fVATSCriticalChanceBonus").unwrap_or(15.0)
    } else {
        0.0
    }
}

/// What V.A.T.S. does to the player's melee damage while a queued hit
/// plays (`009b5170`): a weapon's special × its `VATS` damage multiplier,
/// Uppercut × `fUpperCutVatsMultiplier` 1.15, Cross × `fCrossVatsMultiplier`
/// 1.1, Stomp × `fGroundAttackVatsMultiplier` 2, an automatic melee weapon
/// × `fVATSAutomaticMeleeDamageMult` 2. Otherwise 1.
pub fn attack_damage_mult(
    order: &LoadOrder,
    state: &GameState,
    attacker: FormId,
    weapon: Option<&Weapon>,
) -> f32 {
    let Some(a) = state.vats.attack.as_ref().filter(|a| a.hit) else {
        return 1.0;
    };
    if attacker != PLAYER_REF || state.vats.mode != mode::PLAYBACK {
        return 1.0;
    }
    if weapon.is_some_and(|w| !w.is_melee()) {
        return 1.0;
    }
    let g = |n: &str, d: f32| game_setting(order, n).unwrap_or(d);
    let mut m = match a.kind {
        kind::SPECIAL => weapon
            .and_then(|w| WeaponVats::load(order, w.form_id))
            .and_then(|v| v.special)
            .map_or(1.0, |sp| sp.damage_mult),
        kind::UPPERCUT => g("fUpperCutVatsMultiplier", 1.15),
        kind::CROSS => g("fCrossVatsMultiplier", 1.1),
        kind::STOMP => g("fGroundAttackVatsMultiplier", 2.0),
        _ => 1.0,
    };
    if weapon.is_some_and(|w| w.flags1 & flags::AUTOMATIC != 0) {
        m *= g("fVATSAutomaticMeleeDamageMult", 2.0);
    }
    m
}

/// `GetWeaponAnimType`'s answer for a weapon's animation type (`005a09b0`,
/// the table at `0118a838`): 1 hand to hand (and no weapon), 2 one-handed
/// melee, 3 two-handed melee, 4 pistols (and energy pistols), 5 rifles
/// (and energy rifles), 6 automatic rifles, 7 handles, 8 launchers, 9
/// grenades and thrown weapons, 10 mines, 11 lunchbox mines. Gunslinger
/// asks for 4, Commando 5 or 6.
pub fn weapon_anim_type(animation: Option<u32>) -> u32 {
    const TABLE: [u32; 14] = [1, 2, 3, 4, 4, 5, 6, 5, 7, 8, 9, 10, 11, 9];
    match animation {
        None => 1,
        Some(a) => TABLE.get(a as usize).copied().unwrap_or(0),
    }
}

/// The condition functions V.A.T.S. answers (`Facts::value`):
/// - `GetVATSMode`: the mode (`005a2590`);
/// - `GetVATSValue n p` (`00594e40`) about the attack being composed or
///   playing (0 with none): 0 its weapon is `p`, 1 its weapon is in list
///   `p`, 2 the target's base is `p`, 3 … in list `p`, 4 the distance, 5
///   the part's actor value is `p`, 6 its kind is `p`, 7 it's a hit, 15 the
///   weapon's animation type is `p`, 17 Paralyzing Palm, 16 a Mysterious
///   Stranger's or Miss Fortune's attack (the attack's bytes `+6` / `+0x24`:
///   never here, no helpers); 8, 11, 12, 13, 14 read the precomputed hit
///   record's flags 0x4, 0x10, 0x40, 0x20, 0x80 (`0058cba0` on `+0x58`:
///   critical, fatal, explode, dismember, cripple) and 9 / 10 its critical
///   effect (`+0x50`) against `p` / list `p`: the record isn't worked out
///   before the hit lands here, so they give 0;
/// - `GetVATS<Side>AreaFree` / `GetVATS<Side>TargetVisible` (`005a5460`…
///   `005a5840` → `008bd830` / `008bdbd0`; [`Side`]): the player's
///   [`SmartCamera`] for the attack playing, which the viewer works out
///   with its lines of sight; 0 for anyone else or outside playback (the
///   game casts them on the spot for any actor);
/// - `GetWeaponAnimType` on someone: [`weapon_anim_type`] of the weapon in
///   their hands;
/// - `IsWeaponInList` on someone: whether the weapon in their hands is in
///   the list.
pub fn function_value(
    facts: &Facts,
    name: &str,
    on: Option<FormId>,
    args: &[Value],
) -> Option<f64> {
    let (order, s) = (facts.order, facts.state);
    let flag = |b: bool| if b { 1.0 } else { 0.0 };
    let arg = |i: usize| args.get(i).cloned().unwrap_or(Value::Number(0.0));
    if let Some((side, visible)) = Side::of_function(name) {
        let value = match (on, s.vats.smart_camera.as_ref()) {
            (Some(PLAYER_REF), Some(sc)) => sc.get(side, visible),
            _ => 0.0,
        };
        return Some(f64::from(value));
    }
    match name {
        "GetVATSMode" => Some(f64::from(s.vats.mode)),
        "GetVATSValue" => {
            let Some(a) = s.vats.attack.as_ref() else {
                return Some(0.0);
            };
            let p = arg(1).number();
            let form = FormId(p as i64 as u32);
            let in_list =
                |f: Option<FormId>| f.is_some_and(|f| perks::form_list(order, form).contains(&f));
            let base = base_of(order, a.target);
            Some(match arg(0).number() as i64 {
                0 => flag(a.weapon == Some(form)),
                1 => flag(in_list(a.weapon)),
                2 => flag(base == Some(form)),
                3 => flag(in_list(base)),
                4 => f64::from(a.distance),
                5 => flag(f64::from(a.part_av) == p),
                6 => flag(f64::from(a.kind) == p),
                7 => flag(a.hit),
                15 => {
                    let animation = a
                        .weapon
                        .and_then(|w| Weapon::load(order, w))
                        .map_or(0, |w| w.animation);
                    flag(f64::from(animation) == p)
                }
                // No Mysterious Stranger or Miss Fortune here.
                16 => 0.0,
                17 => flag(a.paralyzing_palm),
                _ => 0.0,
            })
        }
        "GetWeaponAnimType" => {
            let held = crate::combat::weapon_in_hand(order, s, on?);
            Some(f64::from(weapon_anim_type(held.map(|w| w.animation))))
        }
        "IsWeaponInList" => {
            let held = crate::combat::weapon_in_hand(order, s, on?);
            Some(flag(held.is_some_and(|w| {
                perks::form_list(order, arg(0).form()).contains(&w.form_id)
            })))
        }
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// `FalloutNV.esm`'s values (and the exe's where it sets none).
    fn data() -> Settings {
        Settings {
            max_engage_distance: 5000.0,
            target_angle: 15.0,
            screen_percent_factor: 1.0,
            melee_max_distance: 300.0,
            thrown_range_mult: 2.0,
            grenade_range_mult: 2.0,
            grenade_range_min: 128.0,
            skill_factor: 1.0,
            grenade_skill_factor: 0.4,
            grenade_chance_mult: 1.0,
            spread_mult: 1.1,
            grenade_target_area: 1640.0,
            destructible_mult: 1.25,
            range_spread_max: 100.0,
            hit_chance_mult: 1.0,
            stealth_mult: 0.5,
            max_chance: 95.0,
            concentrated_fire_bonus: 5.0,
            paralyze_palm_chance: 0.3,
            shot_burst_time: 0.43,
            shot_long_burst_time: 1.75,
            restore_rate: 0.06,
            costs: {
                let mut c = [0.0; 22];
                c[..15].copy_from_slice(&[
                    22.0, 15.0, 30.0, 20.0, 25.0, 20.0, 40.0, 35.0, 10.0, 10.0, 10.0, 10.0, 10.0,
                    10.0, 10.0,
                ]);
                c[19] = 1.0;
                c
            },
            uppercut_threshold: 50.0,
            uppercut_cost: 20.0,
            cross_threshold: 75.0,
            cross_cost: 20.0,
            player_time_mult: 6.0,
            playback_delay: 0.17,
            camera_max_time: 20.0,
            melee_warp_mult: 0.32,
            h2h_warp_mult: 0.27,
            melee_reach_mult: 2.0,
            miss_ratio_low: 1.0,
            miss_ratio_high: 1.3,
            auto_aim_max_degrees: 15.0,
            auto_aim_max_miss_degrees: 3.0,
            shotgun_spread_ratio: 1.0,
            critical_bonus: 5.0,
            player_damage_mult: 0.75,
            automatic_melee_mult: 2.0,
            uppercut_mult: 1.15,
            cross_mult: 1.1,
            stomp_mult: 2.0,
            sneak_max_distance: 2500.0,
            sneak_exterior_mult: 2.0,
            npc_max_gun_wobble: 15.0,
            standing_spread: 0.1,
            walking_spread: 0.1,
            running_spread: 0.2,
            unaimed_spread: 0.2,
            crippled_arm_1h: 0.2,
            crippled_arm_2h: 0.4,
            crippled_arms_1h: 0.4,
            crippled_arms_2h: 0.6,
            min_gun_spread: 0.01,
            wobble_to_skill: 0.5,
            strength_req_penalty: 0.025,
        }
    }

    fn gun(spread: f32, distance: f32, base: f32) -> ChanceInputs {
        ChanceInputs {
            aim: Aim::Gun {
                spread,
                fixed_range: None,
            },
            distance,
            bound: 143.9,
            visible: 1.0,
            base,
            proximity: false,
            destructible: false,
            grenade: false,
            stealthed: false,
        }
    }

    #[test]
    fn the_findings_worked_example() {
        // Guns 50, a 9mm pistol (min spread 0.7°), standing, aiming: wobble
        // (0.1) × 0.75 × 15 = 1.125°, so 1.825° in all, against a Doc
        // Mitchell-sized target (bound 143.9), fully visible.
        let s = data();
        let spread = 0.7 + 1.125;
        // 500 units: torso 60 → 104.5 → capped at 95; head 30 → 52; arms
        // 45 → 78.
        let at = |d: f32, base: f32| hit_chance(&s, &gun(spread, d, base));
        assert!((at(500.0, 60.0) - 0.95).abs() < 1e-6);
        let head = at(500.0, 30.0);
        assert!((head - 0.5226).abs() < 0.002, "{head}");
        assert_eq!(shown_percent(&s, head, 0, false), 52);
        assert_eq!(shown_percent(&s, at(500.0, 45.0), 0, false), 78);
        // 1500 units: the circle is past half the bound: torso 34, head 17.
        assert_eq!(shown_percent(&s, at(1500.0, 60.0), 0, false), 34);
        assert_eq!(shown_percent(&s, at(1500.0, 30.0), 0, false), 17);
    }

    #[test]
    fn the_chance_falls_with_distance_then_faster() {
        let s = data();
        let at = |d: f32| hit_chance(&s, &gun(2.0, d, 30.0));
        // Up close (2r < bound) it goes as 1/d; far off as 1/d².
        let (a, b) = (at(400.0), at(800.0));
        assert!((a / b - 2.0).abs() < 0.01, "{a} {b}");
        let (c, d) = (at(4000.0), at(8000.0));
        assert!((c / d - 4.0).abs() < 0.01, "{c} {d}");
    }

    #[test]
    fn hidden_parts_stealth_and_the_cap() {
        let s = data();
        let mut i = gun(2.0, 1000.0, 30.0);
        let full = hit_chance(&s, &i);
        i.visible = 0.5;
        assert!((hit_chance(&s, &i) - full * 0.5).abs() < 1e-6);
        i.visible = 0.0;
        assert_eq!(hit_chance(&s, &i), 0.0);
        i.visible = 1.0;
        i.stealthed = true;
        assert!((hit_chance(&s, &i) - full * 0.5).abs() < 1e-6);
        // Melee: 95% within 300 units of the edges, none beyond.
        let mut m = gun(0.0, 300.0, 30.0);
        m.aim = Aim::Melee;
        assert_eq!(hit_chance(&s, &m), 0.95);
        m.distance = 301.0;
        assert_eq!(hit_chance(&s, &m), 0.0);
        // A proximity mine: nothing.
        let mut p = gun(2.0, 100.0, 60.0);
        p.proximity = true;
        assert_eq!(hit_chance(&s, &p), 0.0);
        // A fixed range: nothing past it.
        let mut f = gun(2.0, 2001.0, 60.0);
        f.aim = Aim::Gun {
            spread: 2.0,
            fixed_range: Some(2000.0),
        };
        assert_eq!(hit_chance(&s, &f), 0.0);
    }

    #[test]
    fn thrown_weapons_by_range_and_skill() {
        let s = data();
        let throw = |d: f32, skill: f32, arms: bool| {
            hit_chance(
                &s,
                &ChanceInputs {
                    aim: Aim::Thrown {
                        max_range: 1000.0,
                        skill,
                        arms,
                    },
                    ..gun(0.0, d, 1.0)
                },
            )
        };
        // Within 128: odds 1, plus 0.4 × 50/100 = 0.2 → 120 → 95.
        assert_eq!(throw(100.0, 50.0, true), 0.95);
        // At 1064: 1 − 936/1872 = 0.5, + 0.2 = 70%.
        assert!((throw(1064.0, 50.0, true) - 0.70).abs() < 1e-5);
        // A crippled arm: 0.2 × 0.8 = 0.16 → 66%.
        assert!((throw(1064.0, 50.0, false) - 0.66).abs() < 1e-5);
        // Past twice the range: none.
        assert_eq!(throw(2001.0, 100.0, true), 0.0);
    }

    #[test]
    fn concentrated_fire_shows_five_more_a_shot() {
        let s = data();
        assert_eq!(shown_percent(&s, 0.52, 2, true), 62);
        assert_eq!(shown_percent(&s, 0.52, 2, false), 52);
        assert_eq!(shown_percent(&s, 0.93, 3, true), 95);
        // Truncated, not rounded.
        assert_eq!(shown_percent(&s, 0.529, 0, false), 52);
    }

    #[test]
    fn kinds_by_animation_type() {
        assert_eq!(attack_kind(None), kind::UNARMED);
        assert_eq!(attack_kind(Some(4)), kind::PISTOL);
        assert_eq!(attack_kind(Some(7)), kind::RIFLE);
        assert_eq!(attack_kind(Some(9)), kind::LAUNCHER);
        assert_eq!(attack_kind(Some(12)), kind::NONE);
        assert_eq!(attack_kind(Some(13)), kind::THROWN);
        assert!(is_melee_kind(kind::UPPERCUT) && !is_melee_kind(kind::PISTOL));
        assert_eq!(kind_label(kind::RIFLE), "Shot");
        // GetWeaponAnimType's own numbering.
        assert_eq!(weapon_anim_type(Some(3)), 4);
        assert_eq!(weapon_anim_type(Some(4)), 4);
        assert_eq!(weapon_anim_type(Some(7)), 5);
        assert_eq!(weapon_anim_type(Some(6)), 6);
        assert_eq!(weapon_anim_type(None), 1);
    }

    #[test]
    fn the_first_target_by_score_and_the_cone() {
        let s = data();
        // By score: (180 − 40)/180 + 40/100 = 1.18 for the near one off to
        // the side against (180 − 0)/180 + 40/2000 = 1.02 for the far one
        // straight ahead; the one inside the 15° cone only takes over when
        // it's nearer.
        assert_eq!(first_target(&s, &[(40.0, 100.0), (0.0, 2000.0)]), Some(0));
        // Taken first, the one inside the cone stops score wins.
        assert_eq!(first_target(&s, &[(0.0, 2000.0), (40.0, 100.0)]), Some(0));
        // Two inside the cone: the nearer.
        assert_eq!(
            first_target(&s, &[(5.0, 2000.0), (10.0, 500.0), (60.0, 50.0)]),
            Some(1)
        );
        // None inside: the best score.
        assert_eq!(first_target(&s, &[(90.0, 1000.0), (30.0, 900.0)]), Some(1));
        assert_eq!(first_target(&s, &[]), None);
    }

    #[test]
    fn missed_shots_go_wide_by_at_most_three_degrees() {
        let s = data();
        // A 72-unit radius at 2000 units × 1.0: 0.036 rad, straight right.
        let (h, p) = miss_offset(&s, 72.0, 2000.0, 0.0, 0.0);
        assert!((h - 0.036).abs() < 1e-6 && p.abs() < 1e-6);
        // Close up it would be wider than 3°: clamped.
        let (h, _) = miss_offset(&s, 72.0, 200.0, 1.0, 0.0);
        assert!((h - 3f32.to_radians()).abs() < 1e-6);
    }

    #[test]
    fn parts_share_their_highest_and_fill_in_the_average() {
        let entry = |slot: i8, av: i32, chance: f32, scanned: bool| PartEntry {
            slot,
            name: String::new(),
            node: String::new(),
            actor_value: av,
            chance,
            scanned,
        };
        let mut parts = vec![
            entry(0, 26, 0.6, true),
            entry(1, 25, 0.2, true),
            entry(2, 25, 0.3, true),
            entry(3, 27, 0.0, false),
        ];
        share_highest(&mut parts);
        assert_eq!(parts[1].chance, 0.3);
        fill_unscanned(&mut parts);
        assert!((parts[3].chance - (0.6 + 0.3 + 0.3) / 3.0).abs() < 1e-6);
        // The head near by, else the last part aimed at, else the torso.
        assert_eq!(default_part(&parts, 200.0, 27), 1);
        assert_eq!(default_part(&parts, 900.0, 27), 3);
        assert_eq!(default_part(&parts, 900.0, 29), 0);
    }

    fn attempt(chance: f32, cost: f32) -> Attempt {
        Attempt {
            target: FormId(0x100),
            is_actor: true,
            slot: 1,
            part_av: 25,
            chance,
            kind: kind::PISTOL,
            cost,
            shots: (1, 0),
            melee: false,
            unarmed: false,
            clip_size: Some(13),
            ammo_use: 1,
            thrown_left: None,
            parts: vec![(0, 26), (1, 25), (3, 27), (5, 28), (7, 29), (10, 30)],
            concentrated_fire: false,
            paralyzing_palm: false,
            always_hit: false,
        }
    }

    #[test]
    fn queueing_rolls_now_and_spends_the_plan() {
        let s = data();
        let mut plan = Plan::new(80.0, 2, 10);
        // A roll under the chance hits, over misses.
        let a = attempt(0.5, 17.0);
        let first = plan.queue(&s, &a, &mut || 0.49).unwrap().clone();
        assert!(first.hit && !first.reload);
        assert_eq!((plan.ap_left, plan.clip), (63.0, 1));
        // The second shot runs the clip out: it costs the reload's 10 more
        // and is followed by a "Reload" line; the projected clip refills
        // from the 10 carried (`007f28d0`).
        let second = plan.queue(&s, &a, &mut || 0.51).unwrap().clone();
        assert!(!second.hit && second.reload);
        assert_eq!(second.ap, 27.0);
        assert_eq!((plan.ap_left, plan.clip, plan.reserve), (36.0, 10, 0));
        // The third is an ordinary shot from the refilled clip.
        let third = plan.queue(&s, &a, &mut || 0.0).unwrap().clone();
        assert!(!third.reload);
        assert_eq!(third.ap, 17.0);
        assert_eq!((plan.ap_left, plan.clip, plan.reserve), (19.0, 9, 0));
        // Not enough left (19 for 20; exactly enough is enough).
        assert_eq!(
            plan.queue(&s, &attempt(0.5, 20.0), &mut || 0.0)
                .unwrap_err(),
            Refusal::NotEnoughAp
        );
        let mut exact = plan.clone();
        assert!(exact.queue(&s, &attempt(0.5, 19.0), &mut || 0.0).is_ok());
        // Undo gives it back.
        plan.undo();
        assert_eq!((plan.ap_left, plan.clip, plan.reserve), (36.0, 10, 0));
        // Under 1% shown: zero chance, whatever else.
        assert_eq!(
            plan.queue(&s, &attempt(0.009, 1.0), &mut || 0.0)
                .unwrap_err(),
            Refusal::ZeroChance
        );
        // Nothing left to fire.
        let mut dry = Plan::new(80.0, 0, 0);
        assert_eq!(dry.queue(&s, &a, &mut || 0.0).unwrap_err(), Refusal::NoAmmo);
    }

    #[test]
    fn the_reload_is_charged_on_the_attack_that_empties_the_clip() {
        let s = data();
        // A burst of 5 with 3 in the clip: the clip can't cover it, so the
        // attack costs 10 more; the line follows it.
        let mut plan = Plan::new(80.0, 3, 20);
        let mut burst = attempt(0.5, 20.0);
        burst.shots = (5, 0);
        let q = plan.queue(&s, &burst, &mut || 0.0).unwrap().clone();
        assert!(q.reload && q.ap == 30.0);
        assert_eq!((plan.clip, plan.reserve), (13, 7));
        // Exactly the shots in the clip counts as running out too.
        let mut plan = Plan::new(80.0, 1, 5);
        let q = plan
            .queue(&s, &attempt(0.5, 17.0), &mut || 0.0)
            .unwrap()
            .clone();
        assert!(q.reload && q.ap == 27.0);
        assert_eq!((plan.clip, plan.reserve), (5, 0));
        // Nothing left to load: charged, but no "Reload" line.
        let mut plan = Plan::new(80.0, 1, 0);
        let q = plan
            .queue(&s, &attempt(0.5, 17.0), &mut || 0.0)
            .unwrap()
            .clone();
        assert!(!q.reload && q.ap == 27.0);
        assert_eq!((plan.clip, plan.reserve), (0, 0));
        assert_eq!(
            plan.queue(&s, &attempt(0.5, 17.0), &mut || 0.0)
                .unwrap_err(),
            Refusal::NoAmmo
        );
        // A clip of one shot's worth (clip size = ammo use) gets no line
        // either, and a clip of 1 no charge.
        let mut plan = Plan::new(80.0, 1, 5);
        let mut single = attempt(0.5, 17.0);
        single.clip_size = Some(1);
        let q = plan.queue(&s, &single, &mut || 0.0).unwrap().clone();
        assert!(!q.reload && q.ap == 17.0);
        assert_eq!((plan.clip, plan.reserve), (1, 4));
        // A melee weapon has no clip: nothing of this.
        let mut plan = Plan::new(80.0, 0, 0);
        let mut blade = attempt(0.95, 15.0);
        blade.kind = kind::ONE_HAND_MELEE;
        blade.melee = true;
        blade.clip_size = None;
        let q = plan.queue(&s, &blade, &mut || 0.0).unwrap().clone();
        assert!(!q.reload && q.ap == 15.0);
    }

    #[test]
    fn the_smart_camera_checks_look_from_each_side() {
        let s = SmartCameraSettings {
            check_height: 96.0,
            step_distance: 128.0,
            step_count: 5,
        };
        // Facing north: right is east, left west, back south; a turn of
        // 90° clockwise moves them round.
        let d = |h: f32, side: Side| side_direction(h, side).map(|v| (v * 1000.0).round() / 1000.0);
        assert_eq!(d(0.0, Side::Front), [0.0, 1.0, 0.0]);
        assert_eq!(d(0.0, Side::Right), [1.0, 0.0, 0.0]);
        assert_eq!(d(0.0, Side::Left), [-1.0, 0.0, 0.0]);
        assert_eq!(d(0.0, Side::Back), [0.0, -1.0, 0.0]);
        assert_eq!(d(PI / 2.0, Side::Front), [1.0, 0.0, 0.0]);
        // The area-free line: from 96 up, 10000 along; the answer the
        // distance of the first thing met, else the largest float.
        let (from, to) = area_free_line(&s, [10.0, 20.0, 30.0], 0.0, Side::Right);
        assert_eq!(from, [10.0, 20.0, 126.0]);
        let near = |a: [f32; 3], b: [f32; 3]| (0..3).all(|i| (a[i] - b[i]).abs() < 0.01);
        assert!(near(to, [10010.0, 20.0, 126.0]), "{to:?}");
        assert_eq!(area_free(Some(0.025)), 250.0);
        assert_eq!(area_free(None), f32::MAX);
        // Five samples a step apart along the side.
        let samples = visible_samples(&s, [0.0; 3], 0.0, Side::Right);
        assert_eq!(samples.len(), 5);
        assert!(near(samples[0], [128.0, 0.0, 96.0]), "{:?}", samples[0]);
        assert!(near(samples[4], [640.0, 0.0, 96.0]), "{:?}", samples[4]);
        let target = Some([0.0, 500.0, 96.0]);
        // Nothing in the way: all five reached.
        assert_eq!(target_visible(&s, &samples, target, |_, _| vec![]), 640.0);
        // The third sample's line blocked: two reached.
        let wall = |p: [f32; 3], _: [f32; 3]| {
            if p[0] >= 384.0 {
                vec![LineHit {
                    fraction: 0.5,
                    actor: false,
                }]
            } else {
                vec![]
            }
        };
        assert_eq!(target_visible(&s, &samples, target, wall), 256.0);
        // A hit past 85% of the way (at the target's own body) doesn't
        // block; an actor more than 256 units from the sample doesn't
        // either, a nearer one does.
        let late = |_: [f32; 3], _: [f32; 3]| {
            vec![LineHit {
                fraction: 0.9,
                actor: false,
            }]
        };
        assert_eq!(target_visible(&s, &samples, target, late), 640.0);
        let far_actor = |_: [f32; 3], _: [f32; 3]| {
            vec![LineHit {
                fraction: 0.8,
                actor: true,
            }]
        };
        assert_eq!(target_visible(&s, &samples, target, far_actor), 640.0);
        let near_actor = |_: [f32; 3], _: [f32; 3]| {
            vec![LineHit {
                fraction: 0.1,
                actor: true,
            }]
        };
        assert_eq!(target_visible(&s, &samples, target, near_actor), 0.0);
        // No target: 0.
        assert_eq!(target_visible(&s, &samples, None, |_, _| vec![]), 0.0);
        // The function names.
        assert_eq!(
            Side::of_function("GetVATSLeftTargetVisible"),
            Some((Side::Left, true))
        );
        assert_eq!(
            Side::of_function("GetVATSBackAreaFree"),
            Some((Side::Back, false))
        );
        assert_eq!(Side::of_function("GetVATSValue"), None);
    }

    #[test]
    fn concentrated_fire_adds_five_points_a_shot_in_a_row() {
        let s = data();
        let mut plan = Plan::new(1000.0, 100, 0);
        let mut a = attempt(0.30, 10.0);
        a.concentrated_fire = true;
        // The third in a row needs a roll under 0.40.
        plan.queue(&s, &a, &mut || 0.0).unwrap();
        plan.queue(&s, &a, &mut || 0.0).unwrap();
        assert_eq!(plan.in_a_row(a.target, 25), 2);
        assert!(plan.queue(&s, &a, &mut || 0.399).unwrap().hit);
        // Another part breaks the run.
        let mut torso = a.clone();
        torso.part_av = 26;
        torso.slot = 0;
        plan.queue(&s, &torso, &mut || 0.0).unwrap();
        assert!(!plan.queue(&s, &a, &mut || 0.31).unwrap().hit);
    }

    #[test]
    fn melee_aims_at_a_random_part_above_the_legs() {
        let s = data();
        let mut plan = Plan::new(1000.0, 0, 0);
        let mut a = attempt(0.95, 15.0);
        a.kind = kind::ONE_HAND_MELEE;
        a.melee = true;
        a.clip_size = None;
        // Four parts in slots 0–6 (torso, head, two arms): r = 60 → number
        // trunc(60 × 4 / 100) = 2, the left arm.
        let mut rolls = [0.6f32, 0.0].into_iter();
        let q = plan.queue(&s, &a, &mut || rolls.next().unwrap()).unwrap();
        assert_eq!((q.slot, q.part_av), (3, 27));
        // r under 1 counts as 1: the torso.
        let mut rolls = [0.001f32, 0.0].into_iter();
        let q = plan.queue(&s, &a, &mut || rolls.next().unwrap()).unwrap();
        assert_eq!(q.part_av, 26);
        // Uppercut, unarmed: the head.
        a.kind = kind::UPPERCUT;
        a.unarmed = true;
        let mut rolls = [0.9f32, 0.0].into_iter();
        let q = plan.queue(&s, &a, &mut || rolls.next().unwrap()).unwrap();
        assert_eq!(q.part_av, 25);
    }
}

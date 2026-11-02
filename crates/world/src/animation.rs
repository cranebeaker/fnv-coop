//! Actor animation as the game plays it: which animation group plays for
//! what an actor does, how one blends into the next, and how fast the
//! walks and runs are played. Read from the exe
//! (`%USERPROFILE%\nv-re\findings\animation.md`): the group table at
//! `011977d8`, `Actor::PickAnimations` (`00895110`), the `AnimData`'s play
//! (`004949a0`), stop (`004994f0`) and update (`00491180`), the text-key
//! parser (`005f3a20`), Gamebryo's `NiControllerSequence::Update`
//! (`00a34ba0`) and `NiBlendInterpolator::ComputeNormalizedWeights`
//! (`00a37260`).
//!
//! The model: an actor's skeleton has eight **body sections** (idle,
//! movement, left arm, left hand, weapon, weapon up, weapon down, special
//! idle); each plays one group at a time. Every sequence that plays is a
//! Gamebryo controller sequence with its own clock; when a new group
//! replaces a section's old one they **cross-fade** over a blend time (the
//! files' `Blend:` text keys in frames of a 30th of a second, else
//! `fAnimationDefaultBlend` 0.2 s), and when a section starts from
//! nothing the new sequence blends in **from the pose** the actor is in.
//! All the sequences playing are blended per bone by **priority**: each
//! controlled block in a `.kf` carries one (idles 10, walks 30, aims 25–45
//! on the arms and 0 on the legs, attacks to 55), and the highest wins,
//! the next one below only filling in while the highest fades in or out.
//! Walks and runs are played at the rate that makes the file's root
//! travel the actor's own speed (`fMoveBaseSpeed` 77 × SpeedMult ÷ 100,
//! × `fMoveRunMult` 4 running) and the actor is moved by the root's
//! travel; so nothing jumps: a walk starting from the idle eases in over
//! 0.2 s while the body already moves at full speed.

use std::sync::Arc;

use esm::{FormId, LoadOrder};
use nif::anim::{Bone, Pose, Quat, Sequence, Track};
use nif::Transform;

use crate::scripting::{game_setting, Facts, GameState};

pub mod camera;
pub mod groups;
pub mod pick;
pub mod snapshot;

/// The animation settings the rules read: `fAnimationDefaultBlend`
/// (`[General]` in the INI, 0.2 built in) and `fAnimationMult` (1).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Settings {
    pub default_blend: f32,
    pub mult: f32,
}

impl Default for Settings {
    fn default() -> Settings {
        Settings {
            default_blend: 0.2,
            mult: 1.0,
        }
    }
}

/// The game's 245 animation groups (the table at `011977d8`, in its
/// order): name, body section, kind. Sections: 0 idle, 1 movement, 2 left
/// arm, 3 left hand, 4 weapon, 5 weapon up, 6 weapon down, 7 special idle,
/// 20 the whole body. Kinds: 0 one-shot, 1 loop, 2 special idle, 3 equip,
/// 4 unequip, 5 attack, 6 power attack, 7 throw, 8 place mine, 9 spin
/// attack, 10 reload W–Z.
pub const GROUPS: [(&str, u8, u8); 245] = [
    ("Idle", 0, 1),
    ("DynamicIdle", 0, 1),
    ("SpecialIdle", 7, 2),
    ("Forward", 1, 1),
    ("Backward", 1, 1),
    ("Left", 1, 1),
    ("Right", 1, 1),
    ("FastForward", 1, 1),
    ("FastBackward", 1, 1),
    ("FastLeft", 1, 1),
    ("FastRight", 1, 1),
    ("DodgeForward", 1, 0),
    ("DodgeBack", 1, 0),
    ("DodgeLeft", 1, 0),
    ("DodgeRight", 1, 0),
    ("TurnLeft", 1, 1),
    ("TurnRight", 1, 1),
    ("Aim", 4, 1),
    ("AimUp", 5, 1),
    ("AimDown", 6, 1),
    ("AimIS", 4, 1),
    ("AimISUp", 5, 1),
    ("AimISDown", 6, 1),
    ("Holster", 4, 0),
    ("Equip", 4, 3),
    ("Unequip", 4, 4),
    ("AttackLeft", 4, 5),
    ("AttackLeftUp", 5, 5),
    ("AttackLeftDown", 6, 5),
    ("AttackLeftIS", 4, 5),
    ("AttackLeftISUp", 5, 5),
    ("AttackLeftISDown", 6, 5),
    ("AttackRight", 4, 5),
    ("AttackRightUp", 5, 5),
    ("AttackRightDown", 6, 5),
    ("AttackRightIS", 4, 5),
    ("AttackRightISUp", 5, 5),
    ("AttackRightISDown", 6, 5),
    ("Attack3", 4, 5),
    ("Attack3Up", 5, 5),
    ("Attack3Down", 6, 5),
    ("Attack3IS", 4, 5),
    ("Attack3ISUp", 5, 5),
    ("Attack3ISDown", 6, 5),
    ("Attack4", 4, 5),
    ("Attack4Up", 5, 5),
    ("Attack4Down", 6, 5),
    ("Attack4IS", 4, 5),
    ("Attack4ISUp", 5, 5),
    ("Attack4ISDown", 6, 5),
    ("Attack5", 4, 5),
    ("Attack5Up", 5, 5),
    ("Attack5Down", 6, 5),
    ("Attack5IS", 4, 5),
    ("Attack5ISUp", 5, 5),
    ("Attack5ISDown", 6, 5),
    ("Attack6", 4, 5),
    ("Attack6Up", 5, 5),
    ("Attack6Down", 6, 5),
    ("Attack6IS", 4, 5),
    ("Attack6ISUp", 5, 5),
    ("Attack6ISDown", 6, 5),
    ("Attack7", 4, 5),
    ("Attack7Up", 5, 5),
    ("Attack7Down", 6, 5),
    ("Attack7IS", 4, 5),
    ("Attack7ISUp", 5, 5),
    ("Attack7ISDown", 6, 5),
    ("Attack8", 4, 5),
    ("Attack8Up", 5, 5),
    ("Attack8Down", 6, 5),
    ("Attack8IS", 4, 5),
    ("Attack8ISUp", 5, 5),
    ("Attack8ISDown", 6, 5),
    ("AttackLoop", 4, 1),
    ("AttackLoopUp", 5, 1),
    ("AttackLoopDown", 6, 1),
    ("AttackLoopIS", 4, 1),
    ("AttackLoopISUp", 5, 1),
    ("AttackLoopISDown", 6, 1),
    ("AttackSpin", 4, 9),
    ("AttackSpinUp", 5, 9),
    ("AttackSpinDown", 6, 9),
    ("AttackSpinIS", 4, 9),
    ("AttackSpinISUp", 5, 9),
    ("AttackSpinISDown", 6, 9),
    ("AttackSpin2", 4, 9),
    ("AttackSpin2Up", 5, 9),
    ("AttackSpin2Down", 6, 9),
    ("AttackSpin2IS", 4, 9),
    ("AttackSpin2ISUp", 5, 9),
    ("AttackSpin2ISDown", 6, 9),
    ("AttackPower", 4, 6),
    ("AttackForwardPower", 4, 6),
    ("AttackBackPower", 4, 6),
    ("AttackLeftPower", 4, 6),
    ("AttackRightPower", 4, 6),
    ("AttackCustom1Power", 4, 6),
    ("AttackCustom2Power", 4, 6),
    ("AttackCustom3Power", 4, 6),
    ("AttackCustom4Power", 4, 6),
    ("AttackCustom5Power", 4, 6),
    ("PlaceMine", 4, 8),
    ("PlaceMineUp", 5, 8),
    ("PlaceMineDown", 6, 8),
    ("PlaceMineIS", 4, 8),
    ("PlaceMineISUp", 5, 8),
    ("PlaceMineISDown", 6, 8),
    ("PlaceMine2", 4, 8),
    ("PlaceMine2Up", 5, 8),
    ("PlaceMine2Down", 6, 8),
    ("PlaceMine2IS", 4, 8),
    ("PlaceMine2ISUp", 5, 8),
    ("PlaceMine2ISDown", 6, 8),
    ("AttackThrow", 4, 7),
    ("AttackThrowUp", 5, 7),
    ("AttackThrowDown", 6, 7),
    ("AttackThrowIS", 4, 7),
    ("AttackThrowISUp", 5, 7),
    ("AttackThrowISDown", 6, 7),
    ("AttackThrow2", 4, 7),
    ("AttackThrow2Up", 5, 7),
    ("AttackThrow2Down", 6, 7),
    ("AttackThrow2IS", 4, 7),
    ("AttackThrow2ISUp", 5, 7),
    ("AttackThrow2ISDown", 6, 7),
    ("AttackThrow3", 4, 7),
    ("AttackThrow3Up", 5, 7),
    ("AttackThrow3Down", 6, 7),
    ("AttackThrow3IS", 4, 7),
    ("AttackThrow3ISUp", 5, 7),
    ("AttackThrow3ISDown", 6, 7),
    ("AttackThrow4", 4, 7),
    ("AttackThrow4Up", 5, 7),
    ("AttackThrow4Down", 6, 7),
    ("AttackThrow4IS", 4, 7),
    ("AttackThrow4ISUp", 5, 7),
    ("AttackThrow4ISDown", 6, 7),
    ("AttackThrow5", 4, 7),
    ("AttackThrow5Up", 5, 7),
    ("AttackThrow5Down", 6, 7),
    ("AttackThrow5IS", 4, 7),
    ("AttackThrow5ISUp", 5, 7),
    ("AttackThrow5ISDown", 6, 7),
    ("Attack9", 4, 5),
    ("Attack9Up", 5, 5),
    ("Attack9Down", 6, 5),
    ("Attack9IS", 4, 5),
    ("Attack9ISUp", 5, 5),
    ("Attack9ISDown", 6, 5),
    ("AttackThrow6", 4, 7),
    ("AttackThrow6Up", 5, 7),
    ("AttackThrow6Down", 6, 7),
    ("AttackThrow6IS", 4, 7),
    ("AttackThrow6ISUp", 5, 7),
    ("AttackThrow6ISDown", 6, 7),
    ("AttackThrow7", 4, 7),
    ("AttackThrow7Up", 5, 7),
    ("AttackThrow7Down", 6, 7),
    ("AttackThrow7IS", 4, 7),
    ("AttackThrow7ISUp", 5, 7),
    ("AttackThrow7ISDown", 6, 7),
    ("AttackThrow8", 4, 7),
    ("AttackThrow8Up", 5, 7),
    ("AttackThrow8Down", 6, 7),
    ("AttackThrow8IS", 4, 7),
    ("AttackThrow8ISUp", 5, 7),
    ("AttackThrow8ISDown", 6, 7),
    ("Counter", 4, 6),
    ("stomp", 4, 6),
    ("BlockIdle", 2, 1),
    ("BlockHit", 4, 0),
    ("Recoil", 4, 0),
    ("ReloadWStart", 4, 0),
    ("ReloadXStart", 4, 0),
    ("ReloadYStart", 4, 0),
    ("ReloadZStart", 4, 0),
    ("ReloadA", 4, 0),
    ("ReloadB", 4, 0),
    ("ReloadC", 4, 0),
    ("ReloadD", 4, 0),
    ("ReloadE", 4, 0),
    ("ReloadF", 4, 0),
    ("ReloadG", 4, 0),
    ("ReloadH", 4, 0),
    ("ReloadI", 4, 0),
    ("ReloadJ", 4, 0),
    ("ReloadK", 4, 0),
    ("ReloadL", 4, 0),
    ("ReloadM", 4, 0),
    ("ReloadN", 4, 0),
    ("ReloadO", 4, 0),
    ("ReloadP", 4, 0),
    ("ReloadQ", 4, 0),
    ("ReloadR", 4, 0),
    ("ReloadS", 4, 0),
    ("ReloadW", 4, 10),
    ("ReloadX", 4, 10),
    ("ReloadY", 4, 10),
    ("ReloadZ", 4, 10),
    ("JamA", 4, 0),
    ("JamB", 4, 0),
    ("JamC", 4, 0),
    ("JamD", 4, 0),
    ("JamE", 4, 0),
    ("JamF", 4, 0),
    ("JamG", 4, 0),
    ("JamH", 4, 0),
    ("JamI", 4, 0),
    ("JamJ", 4, 0),
    ("JamK", 4, 0),
    ("JamL", 4, 0),
    ("JamM", 4, 0),
    ("JamN", 4, 0),
    ("JamO", 4, 0),
    ("JamP", 4, 0),
    ("JamQ", 4, 0),
    ("JamR", 4, 0),
    ("JamS", 4, 0),
    ("JamW", 4, 0),
    ("JamX", 4, 0),
    ("JamY", 4, 0),
    ("JamZ", 4, 0),
    ("Stagger", 1, 0),
    ("Death", 20, 0),
    ("Talking", 3, 0),
    ("PipBoy", 2, 6),
    ("JumpStart", 1, 0),
    ("JumpLoop", 1, 1),
    ("JumpLand", 1, 0),
    ("HandGrip1", 2, 1),
    ("HandGrip2", 2, 1),
    ("HandGrip3", 2, 1),
    ("HandGrip4", 2, 1),
    ("HandGrip5", 2, 1),
    ("HandGrip6", 2, 1),
    ("JumpLoopForward", 1, 1),
    ("JumpLoopBackward", 1, 1),
    ("JumpLoopLeft", 1, 1),
    ("JumpLoopRight", 1, 1),
    ("PipBoyChild", 2, 6),
    ("JumpLandForward", 1, 0),
    ("JumpLandBackward", 1, 0),
    ("JumpLandLeft", 1, 0),
    ("JumpLandRight", 1, 0),
];

/// Group numbers used by name.
pub mod group {
    pub const IDLE: u8 = 0;
    pub const DYNAMIC_IDLE: u8 = 1;
    pub const SPECIAL_IDLE: u8 = 2;
    pub const FORWARD: u8 = 3;
    pub const BACKWARD: u8 = 4;
    pub const LEFT: u8 = 5;
    pub const RIGHT: u8 = 6;
    pub const FAST_FORWARD: u8 = 7;
    pub const FAST_BACKWARD: u8 = 8;
    pub const FAST_LEFT: u8 = 9;
    pub const FAST_RIGHT: u8 = 10;
    pub const TURN_LEFT: u8 = 15;
    pub const TURN_RIGHT: u8 = 16;
    pub const AIM: u8 = 17;
    pub const HOLSTER: u8 = 23;
    pub const EQUIP: u8 = 24;
    pub const UNEQUIP: u8 = 25;
    pub const ATTACK_LEFT: u8 = 26;
    pub const ATTACK_RIGHT: u8 = 32;
    pub const RELOAD_A: u8 = 177;
    pub const RELOAD_Z: u8 = 199;
}

/// The body sections.
pub mod section {
    pub const IDLE: u8 = 0;
    pub const MOVEMENT: u8 = 1;
    pub const LEFT_ARM: u8 = 2;
    pub const LEFT_HAND: u8 = 3;
    pub const WEAPON: u8 = 4;
    pub const WEAPON_UP: u8 = 5;
    pub const WEAPON_DOWN: u8 = 6;
    pub const SPECIAL_IDLE: u8 = 7;
    pub const COUNT: usize = 8;
}

/// The section a group plays in (the whole body, 20, counts as the
/// movement section and the upper body, 21, as the weapon section:
/// `00494740`).
pub fn section_of(group: u8) -> u8 {
    slot(
        GROUPS
            .get(usize::from(group))
            .map_or(section::IDLE, |g| g.1),
    )
}

/// The section slot a section number plays in (`004301b0`, `00491040`,
/// `0070f490`): the whole body (0x14) in the movement slot, the upper body
/// (0x15) in the weapon slot.
pub fn slot(section: u8) -> u8 {
    match section {
        20 => section::MOVEMENT,
        21 => section::WEAPON,
        s => s,
    }
}

/// A group's kind (see [`GROUPS`]).
pub fn kind_of(group: u8) -> u8 {
    GROUPS.get(usize::from(group)).map_or(0, |g| g.2)
}

/// A group's number by its name (case ignored).
pub fn group_named(name: &str) -> Option<u8> {
    GROUPS
        .iter()
        .position(|g| g.0.eq_ignore_ascii_case(name))
        .and_then(|i| u8::try_from(i).ok())
}

/// What a sequence's text keys say about its group (`TESAnimGroup`,
/// `005f3a20`): blend frames and the action times.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GroupData {
    /// `Blend: N`, `BlendIn: N`, `BlendOut: N` in frames (30 a second).
    pub blend: u8,
    pub blend_in: u8,
    pub blend_out: u8,
    /// The `start` and `end` keys' times (the sequence's own when it lacks
    /// them), and a special idle's `StartLoop` / `EndLoop` (defaulting to
    /// start and end, as the parser fills them).
    pub start: f32,
    pub end: f32,
    pub loop_start: f32,
    pub loop_end: f32,
    /// The accumulation root's travel a second over the sequence, in the
    /// skeleton's axes (zero when it stays): the group's movement vector
    /// (group +0x1c; inferred from its use as velocity × dt).
    pub travel: [f32; 3],
    /// An equip's `Attach` or an unequip's `Detach` key (their kinds'
    /// second action, `01199a50`): when the weapon goes to the hand or
    /// back (`00491180` counts the keys passed, `00895110` acts on the
    /// first). The parser's key array starts zeroed (`005f2450`), so a file
    /// without one attaches at once (0).
    pub attach: f32,
}

impl GroupData {
    /// Read off a sequence's text keys.
    pub fn read(seq: &Sequence) -> GroupData {
        let mut d = GroupData {
            blend: 0,
            blend_in: 0,
            blend_out: 0,
            start: seq.start,
            end: seq.stop,
            loop_start: f32::NAN,
            loop_end: f32::NAN,
            travel: [0.0; 3],
            attach: 0.0,
        };
        let number = |rest: &str| -> u8 {
            let rest = rest.trim();
            let digits: String = rest.chars().take_while(|c| c.is_ascii_digit()).collect();
            digits.parse::<u32>().map_or(0, |n| n.min(255) as u8)
        };
        for (time, text) in &seq.text_keys {
            let key = text.trim();
            let lower = key.to_ascii_lowercase();
            if let Some(rest) = lower.strip_prefix("blendin:") {
                d.blend_in = number(rest);
            } else if let Some(rest) = lower.strip_prefix("blendout:") {
                d.blend_out = number(rest);
            } else if let Some(rest) = lower.strip_prefix("blend:") {
                d.blend = number(rest);
            } else if lower == "start" {
                d.start = *time;
            } else if lower == "end" {
                d.end = *time;
            } else if lower == "startloop" {
                d.loop_start = *time;
            } else if lower == "endloop" {
                d.loop_end = *time;
            } else if lower == "attach" || lower == "detach" {
                d.attach = *time;
            }
        }
        // The parser's fixes for special idles: a loop start before the
        // start is the start, a loop end not past it is the end.
        if d.loop_start.is_nan() || d.loop_start < d.start {
            d.loop_start = d.start;
        }
        if d.loop_end.is_nan() || d.loop_end <= d.loop_start {
            d.loop_end = d.end;
        }
        if let Some(t) = seq.root_travel() {
            let length = seq.stop - seq.start;
            if length > 0.0 {
                d.travel = [t[0] / length, t[1] / length, t[2] / length];
            }
        }
        d
    }

    /// The frames to blend in by (`004954e0`: `Blend` or `BlendIn`,
    /// whichever is more).
    pub fn blend_in_frames(&self) -> u8 {
        self.blend.max(self.blend_in)
    }

    /// The frames to blend out by (`00495520`: `Blend` or `BlendOut`).
    pub fn blend_out_frames(&self) -> u8 {
        self.blend.max(self.blend_out)
    }

    /// How fast the root travels along the ground, units a second.
    pub fn speed(&self) -> f32 {
        (self.travel[0] * self.travel[0] + self.travel[1] * self.travel[1]).sqrt()
    }
}

/// The names of each group kind's action keys (the table at `01199a50`,
/// action × 11 + kind): a kind's actions are matched in this order, an
/// empty name ending them.
pub const ACTION_KEYS: [[&str; 5]; 11] = [
    ["Start", "End", "", "", ""],
    ["Start", "End", "", "", ""],
    ["Start", "StartLoop", "EndLoop", "End", ""],
    ["Start", "Attach", "End", "", ""],
    ["Start", "Detach", "End", "", ""],
    ["Start", "Hit", "Eject", "a:", "End"],
    ["Start", "Hit", "End", "", ""],
    ["Start", "Hold", "Release", "Attach", "End"],
    ["Start", "Release", "Attach", "End", ""],
    ["Start", "Fire", "Loop", "End", ""],
    ["Start", "Hit", "End", "", ""],
];

/// The times a sequence's text keys give its group's actions
/// (`TESAnimGroup::GetTime` (Xbox PDB), the group's array at +0x18, as the
/// text key parser `005f3a20` fills it): the keys are read in order, line
/// by line; lines starting `m:`, `BlendIn:`, `BlendOut:`, `Blend:`,
/// `Decal:`, `Sound:` or `Enum:` (and a key starting `prn:`) are other
/// things; any other line is the group's next action if it starts with
/// that action's name ([`ACTION_KEYS`], case aside: an attack's are
/// `Start`, `Hit`, `Eject`, `a:`, `End`), when it sets that action's time;
/// lines that don't are passed over. An attack's third key may skip
/// `Eject` for `a:` (`01199ad4`). Actions without a key keep 0 (the array
/// starts zeroed, `005f2450`). The special idle's `StartLoop` rule for
/// group 2 isn't here.
///
/// Translated from 005f3a20 (decompiled, FalloutNV.exe 1.4.0.525).
pub fn action_times(group: u8, keys: &[(f32, String)]) -> [f32; 5] {
    let kind = GROUPS
        .get(usize::from(group))
        .map_or(0, |g| usize::from(g.2));
    let names = ACTION_KEYS[kind.min(10)];
    let starts = |line: &str, prefix: &str| {
        line.len() >= prefix.len()
            && line.as_bytes()[..prefix.len()].eq_ignore_ascii_case(prefix.as_bytes())
    };
    let mut times = [0.0f32; 5];
    let mut stage = 0usize;
    for (time, text) in keys {
        if starts(text, "prn:") {
            continue;
        }
        for line in text.split(['\r', '\n']).filter(|l| !l.is_empty()) {
            if [
                "m:",
                "BlendIn:",
                "BlendOut:",
                "Blend:",
                "Decal:",
                "Sound:",
                "Enum:",
            ]
            .iter()
            .any(|p| starts(line, p))
            {
                continue;
            }
            // The kind's stages run 0, 1, 2 … to its first empty name
            // (the table at `011977e8` holds -1 there).
            if stage >= 5 || (stage > 0 && names[stage].is_empty()) {
                continue;
            }
            let mut action = stage;
            if kind == 5 && stage == 2 && starts(line, names[3]) {
                action = 3;
                stage = 3;
            }
            if starts(line, names[action]) {
                if *time > -1.0 {
                    times[action] = *time;
                }
                stage += 1;
            }
        }
    }
    times
}

/// The blend time from one group to the next (`004949a0`): the frames
/// the old group blends out by or the new one in by, whichever is more,
/// at 30 a second; none → `fAnimationDefaultBlend`; ÷ `fAnimationMult`.
pub fn blend_seconds(old: Option<&GroupData>, new: &GroupData, s: &Settings) -> f32 {
    let frames = old
        .map_or(0, GroupData::blend_out_frames)
        .max(new.blend_in_frames());
    let blend = if frames != 0 {
        f32::from(frames) / 30.0
    } else {
        s.default_blend
    };
    blend / s.mult
}

/// The blend time a group eases out by when it ends or is stopped
/// (`004994f0`): its blend-out frames at 30 a second, else the default;
/// ÷ `fAnimationMult`.
pub fn stop_blend_seconds(playing: &GroupData, s: &Settings) -> f32 {
    let frames = playing.blend_out_frames();
    let blend = if frames != 0 {
        f32::from(frames) / 30.0
    } else {
        s.default_blend
    };
    blend / s.mult
}

/// An actor's movement this frame, as the mover's flags say it
/// (`008846e0`: 0x1 forward, 0x2 back, 0x4 left, 0x8 right, 0x10 turn
/// left, 0x20 turn right, 0x200 running).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct MoveFlags {
    pub forward: bool,
    pub backward: bool,
    pub left: bool,
    pub right: bool,
    pub turn_left: bool,
    pub turn_right: bool,
    pub running: bool,
}

/// The movement group for the flags (`00895110`): the first direction
/// set in the order forward, back, left, right, as Forward–Right or
/// FastForward–FastRight when running; with no direction, a turn; and a
/// speed under 1 unit a second makes it the idle (none).
pub fn movement_group(flags: MoveFlags, speed: f32) -> Option<u8> {
    let direction = [flags.forward, flags.backward, flags.left, flags.right]
        .iter()
        .position(|&on| on);
    match direction {
        None if flags.turn_left => Some(group::TURN_LEFT),
        None if flags.turn_right => Some(group::TURN_RIGHT),
        None => None,
        Some(_) if speed < 1.0 => None,
        Some(d) => {
            let base = if flags.running {
                group::FAST_FORWARD
            } else {
                group::FORWARD
            };
            Some(base + d as u8)
        }
    }
}

/// The rate a movement animation plays at (`00895110`, into
/// `AnimData` +0x10c): the actor's speed over the root travel of the
/// Forward (or FastForward) group's sequence, the latter as a whole
/// number (`00494300` converts it with `_ftol`); 1 when that's 0.
pub fn movement_rate(speed: f32, forward_speed: f32) -> f32 {
    let whole = forward_speed.trunc();
    if whole != 0.0 {
        speed / whole
    } else {
        1.0
    }
}

/// Someone's walking speed before their scale and their legs' condition
/// (`00647d10`): SpeedMult (actor value 21, 100 when unknown) ÷ 100 ×
/// `fMoveBaseSpeed` (77 in the data, 85 built in). The armour and
/// no-weapon terms aren't applied (nobody here wears medium or heavy
/// armour by the game's reckoning; the no-weapon ×1.1 is the player's).
pub fn base_speed(order: &LoadOrder, state: &GameState, who: FormId) -> f32 {
    let mult = Facts {
        order,
        state,
        speaker: None,
    }
    .current_actor_value(who, 21)
    .map_or(1.0, |v| v as f32 * 0.01);
    let base = game_setting(order, "fMoveBaseSpeed").unwrap_or(85.0);
    (mult * base).max(0.0)
}

/// Someone's walking speed before their scale: [`base_speed`] × the
/// crippled-legs multiplier (`body_parts::leg_speed_mult`).
pub fn walk_speed(order: &LoadOrder, state: &GameState, who: FormId) -> f32 {
    base_speed(order, state, who) * crate::body_parts::leg_speed_mult(order, state, who)
}

/// Someone's running speed before their scale: walking × `fMoveRunMult`
/// (4) (`00647f00`).
pub fn run_speed(order: &LoadOrder, state: &GameState, who: FormId) -> f32 {
    walk_speed(order, state, who) * run_mult(order)
}

/// `fMoveRunMult`: running is this times walking (4).
pub fn run_mult(order: &LoadOrder) -> f32 {
    game_setting(order, "fMoveRunMult").unwrap_or(4.0)
}

/// A playing sequence's state (`NiControllerSequence` +0x44).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum State {
    Animating,
    /// Fading in over the length: the ease spinner runs 0 → 1.
    EaseIn,
    /// Fading out: the spinner runs 1 → 0, then it's gone.
    EaseOut,
    /// The source of a blend from a pose: its weight runs 1 → 0.
    TransSource,
    /// The destination of one: its weight runs 0 → 1.
    TransDest,
}

/// One sequence playing on an actor, with its own clock and blend.
#[derive(Debug, Clone)]
struct Active {
    /// The sequence, or none for a frozen pose (`__TempBlendSequence__`).
    seq: Option<Arc<Sequence>>,
    /// A frozen pose: per bone the priority and pose it holds.
    frozen: Vec<Option<(u8, Pose)>>,
    /// The sequence's track for each bone of the skeleton.
    bone_tracks: Vec<Option<usize>>,
    group: u8,
    section: u8,
    data: GroupData,
    state: State,
    /// Seconds into the ease, and its length.
    ease: f32,
    ease_length: f32,
    /// Seconds into the sequence (its own clock, from `start`).
    time: f32,
    /// Loops left after this one (−1 for ever; the game's +0x7c).
    loops_left: i32,
    /// The sequence's weight (`m_fSeqWeight`).
    weight: f32,
}

impl Active {
    fn playing(&self) -> bool {
        matches!(
            self.state,
            State::Animating | State::EaseIn | State::TransDest
        )
    }

    /// The ease spinner and the weight factor the sequence's blend
    /// interpolators get this frame (`00a34ba0`: linear ramps).
    fn spinner_and_factor(&self) -> (f32, f32) {
        let f = if self.ease_length > 0.0 {
            (self.ease / self.ease_length).clamp(0.0, 1.0)
        } else {
            1.0
        };
        match self.state {
            State::Animating => (1.0, 1.0),
            State::EaseIn => (f, 1.0),
            State::EaseOut => (1.0 - f, 1.0),
            State::TransSource => (1.0, 1.0 - f),
            State::TransDest => (1.0, f),
        }
    }

    /// The sequence's own time to sample at: wrapped into its length when
    /// it loops (Gamebryo's `ComputeScaledTime`, `00a30970`), else held at
    /// its end.
    fn sample_time(&self, seq: &Sequence) -> f32 {
        let length = seq.stop - seq.start;
        if length <= 0.0 {
            seq.start
        } else if seq.looping {
            seq.start + self.time.rem_euclid(length)
        } else {
            seq.start + self.time.clamp(0.0, length)
        }
    }

    fn loops(&self, seq: &Sequence) -> bool {
        match kind_of(self.group) {
            0 | 1 => seq.looping,
            2 => true,
            _ => false,
        }
    }
}

/// An actor's animations playing: the sections' groups, every sequence
/// active with its clock and blend, and the rates.
#[derive(Debug, Clone)]
pub struct Player {
    active: Vec<Active>,
    pub settings: Settings,
    /// The movement animations' rate (`AnimData` +0x10c).
    pub movement_rate: f32,
    /// The weapon animations' rate (+0x110).
    pub weapon_rate: f32,
    /// `cSkipNextBlend` (Xbox PDB name of `Animation` +0x120, the same
    /// offset on PC): set, the next group played or section stopped
    /// switches at once instead of blending (`004949a0` takes blend 0,
    /// `004994f0` stops with blend 0, `00496080` deactivates at once);
    /// cleared at the end of the next update (`00491180`). Set through
    /// [`Self::skip_next_blend`].
    skip_blend: bool,
    /// The weapon is drawn (the process's `GetWeaponDrawn`, Xbox PDB,
    /// `008a16d0`): a group that ends in the weapon section isn't eased out
    /// but held for the aim to cross-fade from (`004994f0`: with the weapon
    /// drawn only the weapon up and down sections stop, and `008b28c0`
    /// plays the aim over the ended group).
    pub weapon_drawn: bool,
}

impl Default for Player {
    fn default() -> Player {
        Player::new(Settings::default())
    }
}

/// What [`Player::update`] reports: a group that finished playing
/// (reached its end with no loops left) and started easing out.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Finished {
    pub section: u8,
    pub group: u8,
}

impl Player {
    pub fn new(settings: Settings) -> Player {
        Player {
            active: Vec::new(),
            settings,
            movement_rate: 1.0,
            weapon_rate: 1.0,
            skip_blend: false,
            weapon_drawn: false,
        }
    }

    /// The next group change switches without a blend, until the next
    /// [`Self::update`] ends (`004974a0` sets `cSkipNextBlend`, Xbox PDB).
    /// The furniture procedures set it wherever they turn the actor by the
    /// marker's heading delta or half a turn, so the animation's body turn
    /// and the actor's heading change in the same frame (`009213e0` after
    /// the entry, `00921e80` as the exit starts and after it ends).
    // Translated from 004974a0 (decompiled, FalloutNV.exe 1.4.0.525)
    pub fn skip_next_blend(&mut self) {
        self.skip_blend = true;
    }

    /// Whether the next group change skips its blend.
    pub fn skips_next_blend(&self) -> bool {
        self.skip_blend
    }

    /// The index of the sequence playing in a section (not one easing
    /// out).
    fn current(&self, section: u8) -> Option<usize> {
        self.active
            .iter()
            .position(|a| a.section == section && a.playing() && a.seq.is_some())
    }

    /// The group playing in a section, if any.
    pub fn playing(&self, section: u8) -> Option<u8> {
        self.current(section).map(|i| self.active[i].group)
    }

    /// The sequence playing in a section.
    pub fn sequence(&self, section: u8) -> Option<&Arc<Sequence>> {
        self.current(section)
            .and_then(|i| self.active[i].seq.as_ref())
    }

    /// The state of the sequence playing in a section.
    pub fn state(&self, section: u8) -> Option<State> {
        self.current(section).map(|i| self.active[i].state)
    }

    /// Seconds into the sequence playing in a section.
    pub fn time(&self, section: u8) -> Option<f32> {
        self.current(section).map(|i| self.active[i].time)
    }

    /// Everything active (playing or fading): (group, state), for tests
    /// and listings.
    pub fn all(&self) -> Vec<(u8, State)> {
        self.active
            .iter()
            .filter(|a| a.seq.is_some())
            .map(|a| (a.group, a.state))
            .collect()
    }

    /// Whether any sequence or frozen blend source remains active.
    ///
    /// Unlike [`Self::all`], this includes the source side of a
    /// BlendFromPose transition, whose `seq` is intentionally absent.
    pub fn is_empty(&self) -> bool {
        self.active.is_empty()
    }

    /// Plays a group's sequence in its section as the game does
    /// (`004949a0`): nothing if that section already plays this group and
    /// sequence; else the blend time by the groups' text keys, and a
    /// cross-fade from the section's old sequence, or a blend from the
    /// pose the actor is in when the section has none. `loops` is how
    /// many more times a looping group plays (−1 for ever). True when
    /// something started.
    pub fn play(&mut self, group: u8, seq: &Arc<Sequence>, loops: i32, bones: &[Bone]) -> bool {
        self.play_from_pose(group, section_of(group), seq, loops, bones, None)
    }

    /// Plays a group in a section other than its own (`004949a0` with a
    /// section: an idle played as its `IDLE` record's section, `00498290`;
    /// the whole body, 0x14, plays in the movement section and the upper
    /// body, 0x15, in the weapon section).
    pub fn play_in(
        &mut self,
        section: u8,
        group: u8,
        seq: &Arc<Sequence>,
        loops: i32,
        bones: &[Bone],
    ) -> bool {
        self.play_from_pose(group, slot(section), seq, loops, bones, None)
    }

    /// What the text keys of the sequence playing in a section say.
    pub fn data(&self, section: u8) -> Option<GroupData> {
        self.current(slot(section)).map(|i| self.active[i].data)
    }

    fn play_from_pose(
        &mut self,
        group: u8,
        section: u8,
        seq: &Arc<Sequence>,
        loops: i32,
        bones: &[Bone],
        pose: Option<Vec<nif::Transform>>,
    ) -> bool {
        let old = self.current(section);
        if let Some(i) = old {
            let a = &self.active[i];
            if a.group == group && a.seq.as_ref().is_some_and(|s| Arc::ptr_eq(s, seq)) {
                return false;
            }
        }
        let data = GroupData::read(seq);
        // `cSkipNextBlend`: no blend (`004949a0`).
        let blend = if self.skip_blend {
            0.0
        } else {
            blend_seconds(old.map(|i| &self.active[i].data), &data, &self.settings)
        };
        let bone_tracks = bone_tracks(seq, bones);
        let mut new = Active {
            seq: Some(seq.clone()),
            frozen: Vec::new(),
            bone_tracks,
            group,
            section,
            data,
            state: State::Animating,
            ease: 0.0,
            ease_length: 0.0,
            time: 0.0,
            loops_left: loops,
            weight: 1.0,
        };
        if blend >= 0.01 {
            match old {
                Some(i) => {
                    // CrossFade: the old eases out, the new eases in.
                    let o = &mut self.active[i];
                    o.state = State::EaseOut;
                    o.ease = 0.0;
                    o.ease_length = blend;
                    new.state = State::EaseIn;
                    new.ease_length = blend;
                }
                None => {
                    // BlendFromPose: the pose now, frozen, fades out as the
                    // new fades in; the frozen pose holds the new
                    // sequence's bones at its priorities.
                    let locals = pose.unwrap_or_else(|| self.locals(bones));
                    let frozen = new
                        .bone_tracks
                        .iter()
                        .zip(&locals)
                        .map(|(track, local)| {
                            let t = (*track)?;
                            let priority = seq.tracks[t].priority;
                            Some((
                                priority,
                                Pose {
                                    translation: Some(local.translation),
                                    rotation: Some(quat_from_matrix(&local.rotation)),
                                    scale: Some(local.scale),
                                },
                            ))
                        })
                        .collect();
                    self.active.push(Active {
                        seq: None,
                        frozen,
                        bone_tracks: Vec::new(),
                        group,
                        section,
                        data,
                        state: State::TransSource,
                        ease: 0.0,
                        ease_length: blend,
                        time: 0.0,
                        loops_left: 0,
                        weight: 1.0,
                    });
                    new.state = State::TransDest;
                    new.ease_length = blend;
                }
            }
        }
        // Anything else still playing in the section is cut at once.
        self.active
            .retain(|a| !(a.section == section && a.playing() && a.seq.is_some()));
        self.active.push(new);
        true
    }

    /// Plays a loaded NPC script idle after the deferred flag-0x80 request
    /// (`008ba600` -> `00497f20`, mode 3). Unlike the immediate request,
    /// this does not use `00498f80`'s startup-state gate. The loaded idle
    /// is forcibly freed (`00498290` -> `00498910(0,1)`): its section slot
    /// is cleared, and `00498670` removes its controller sequence. Thus
    /// `004949a0` uses the new idle's blend-in, from the current pose,
    /// rather than cross-fading with the removed sequence's blend-out.
    ///
    /// This bounded path owns loaded special-idle section 7 only. Other
    /// loaded groups, asynchronous loading and IDLE tree selection are
    /// the caller's responsibility. A repeated script request restarts.
    pub fn play_script_idle(&mut self, seq: &Arc<Sequence>, loops: i32, bones: &[Bone]) {
        self.play_idle_in(section::SPECIAL_IDLE, seq, loops, bones);
    }

    /// Plays a loaded idle (the `SpecialIdle` group of its `.kf`) in the
    /// section its `IDLE` record names (`00498290` → `00494740` with the
    /// record's section: 0 the base loop, 1 or 0x14 the movement section,
    /// 0x15 the weapon section, 7 the special idle), the old one there freed
    /// at once and the new one blended in from the pose (`00498910(0,1)`).
    pub fn play_idle_in(&mut self, section: u8, seq: &Arc<Sequence>, loops: i32, bones: &[Bone]) {
        let section = slot(section);
        let pose = self.locals(bones);
        self.active.retain(|a| a.section != section);
        self.play_from_pose(group::SPECIAL_IDLE, section, seq, loops, bones, Some(pose));
    }

    /// Whether the special idle `seq`, meant to play in `section`, is still
    /// on its way in: not playing there yet (the game's idle whose sequence
    /// hasn't loaded), or playing but in state 2 (`EaseIn`) or 5
    /// (`TransDest`). Once it plays at full weight it no longer counts.
    ///
    /// Translated from 00498f80 (decompiled, FalloutNV.exe 1.4.0.525): the
    /// animation's current idle (+0x124, `spAnimIdle`) and its queued one
    /// (+0x128) are each "starting" when they have no sequence yet (unless
    /// the idle's state, +8, is 3) or their sequence's state (+0x44) is 2 or
    /// 5. The queued idle has no counterpart here (idles here are played as
    /// soon as they're taken, their sequences loaded). Getting up from furniture
    /// (`00921e80` cases 4 and 9) waits only while this holds.
    pub fn idle_starting(&self, section: u8, seq: &Arc<Sequence>) -> bool {
        let section = slot(section);
        let loaded = self.playing(section) == Some(group::SPECIAL_IDLE)
            && self.sequence(section).is_some_and(|s| Arc::ptr_eq(s, seq));
        !loaded || matches!(self.state(section), Some(State::EaseIn | State::TransDest))
    }

    /// Requests an already loaded special-idle sequence immediately.
    /// `Actor::PlayIdle`'s request path (`00498f80`, called by `008dae00`)
    /// refuses another request while the current special-idle sequence is
    /// in state 2 (`EaseIn`) or state 5 (`TransDest`). This models only that
    /// request gate: it does not validate actor IDLE data or model the
    /// asynchronous sequence loader. Once permitted, loaded-idle replacement
    /// uses the same forced free as the deferred path. `008dab40` also
    /// rejects an already requested idle via `00498d30`; callers use cached
    /// sequences here so matching pointers preserve that rejection.
    pub fn request_special_idle(
        &mut self,
        seq: &Arc<Sequence>,
        loops: i32,
        bones: &[Bone],
    ) -> bool {
        if matches!(
            self.state(section::SPECIAL_IDLE),
            Some(State::EaseIn | State::TransDest)
        ) {
            return false;
        }
        if self
            .sequence(section::SPECIAL_IDLE)
            .is_some_and(|old| Arc::ptr_eq(old, seq))
        {
            return false;
        }
        self.play_script_idle(seq, loops, bones);
        true
    }

    /// [`Self::request_special_idle`] for an idle that plays in its
    /// record's section (`00498290`: 0, 1, 7, 0x14 or 0x15) while the
    /// current one plays in `current`: refused while the current one is
    /// starting or is this very sequence; else the current one is freed at
    /// once wherever it plays and the new one blended in from the pose.
    pub fn request_idle_in(
        &mut self,
        current: u8,
        section: u8,
        seq: &Arc<Sequence>,
        loops: i32,
        bones: &[Bone],
    ) -> bool {
        let current = slot(current);
        let is_idle = self.playing(current) == Some(group::SPECIAL_IDLE);
        if is_idle && matches!(self.state(current), Some(State::EaseIn | State::TransDest)) {
            return false;
        }
        if is_idle
            && self
                .sequence(current)
                .is_some_and(|old| Arc::ptr_eq(old, seq))
        {
            return false;
        }
        if is_idle && current != slot(section) {
            self.cut_section(current);
        }
        self.play_idle_in(section, seq, loops, bones);
        true
    }

    /// Releases the loaded special idle (`00498910`, non-forced path).
    /// Like a new request, freeing a sequence that just started in EaseIn
    /// or TransDest is refused. `RemoveScriptPackage` calls this for both
    /// player view skeletons (`005cc7c0`); accepted releases use the normal
    /// group blend-out. Pending asynchronous requests are outside this model.
    pub fn free_special_idle(&mut self) -> bool {
        self.free_idle_in(section::SPECIAL_IDLE)
    }

    /// Removes everything in a section at once (a forced free,
    /// `00498910(0,1)` → `00498670`: the sequence deactivated with no
    /// blend).
    pub fn cut_section(&mut self, section: u8) {
        let section = slot(section);
        self.active.retain(|a| a.section != section);
    }

    /// [`Self::free_special_idle`] for an idle playing in another section.
    pub fn free_idle_in(&mut self, section: u8) -> bool {
        let section = slot(section);
        if matches!(self.state(section), Some(State::EaseIn | State::TransDest)) {
            return false;
        }
        if self.playing(section) == Some(group::SPECIAL_IDLE) {
            self.stop_section(section);
        }
        true
    }

    /// Stops a section's group (`004994f0`): it eases out over its
    /// blend-out time (the default without one). Which sections go
    /// (`00496080`): the weapon section takes the weapon up and down ones
    /// with it; the upper body (0x15) is the left arm and hand and the
    /// weapon sections; the whole body (0x14) everything (the special idle,
    /// the base loop unless `cSkipNextBlend` is set, the movement, the
    /// upper body). (`00496080` keeps the special idle and the left arm for
    /// one untraced state, `00702640` = 1.)
    pub fn stop_section(&mut self, section: u8) {
        use self::section as s;
        let skip = self.skip_blend;
        let all: &[u8] = match section {
            0x14 if skip => &[s::SPECIAL_IDLE, s::MOVEMENT, 2, 3, 5, 6, s::WEAPON],
            0x14 => &[s::SPECIAL_IDLE, s::IDLE, s::MOVEMENT, 2, 3, 5, 6, s::WEAPON],
            0x15 => &[2, 3, 5, 6, s::WEAPON],
            s::WEAPON => &[5, 6, s::WEAPON],
            other => &[other][..],
        };
        // One blend for all (`004994f0`): the stopped section's own
        // group's blend-out, none when it has nothing (or with
        // `cSkipNextBlend`).
        let blend = match self.current(slot(section)) {
            Some(i) if !skip => stop_blend_seconds(&self.active[i].data, &self.settings),
            _ => 0.0,
        };
        for &each in all {
            if let Some(i) = self.current(each) {
                self.ease_out(i, blend);
            }
        }
    }

    /// Stops the weapon up and down sections only (`004994f0` with the
    /// weapon drawn: the weapon section itself goes on into the aim).
    pub fn stop_weapon_up_down(&mut self) {
        let blend = match self.current(section::WEAPON) {
            Some(i) if !self.skip_blend => stop_blend_seconds(&self.active[i].data, &self.settings),
            _ => 0.0,
        };
        for s in [section::WEAPON_UP, section::WEAPON_DOWN] {
            if let Some(i) = self.current(s) {
                self.ease_out(i, blend);
            }
        }
    }

    fn ease_out(&mut self, i: usize, blend: f32) {
        if blend > 0.0 {
            let a = &mut self.active[i];
            a.state = State::EaseOut;
            a.ease = 0.0;
            a.ease_length = blend;
        } else {
            self.active.remove(i);
        }
    }

    /// Sets the clock of the sequence playing in a section (seconds from
    /// its start), for a caller that times it itself.
    pub fn sync_time(&mut self, section: u8, time: f32) {
        if let Some(i) = self.current(section) {
            self.active[i].time = time;
        }
    }

    /// Advances everything by `dt` seconds (`00491180`, `00a34ba0`):
    /// blends progress and end, sequences run on (movement ones at the
    /// movement rate while animating, at the file's rate while blending),
    /// loops wrap at the `end` key, and groups that reach their end with
    /// no loops left ease out. Reports those.
    pub fn update(&mut self, dt: f32) -> Vec<Finished> {
        let mut finished = Vec::new();
        let mut gone = Vec::new();
        for (i, a) in self.active.iter_mut().enumerate() {
            match a.state {
                State::EaseIn | State::TransDest => {
                    a.ease += dt;
                    if a.ease >= a.ease_length {
                        a.state = State::Animating;
                        a.ease = 0.0;
                        a.ease_length = 0.0;
                    }
                }
                State::EaseOut | State::TransSource => {
                    a.ease += dt;
                    if a.ease >= a.ease_length {
                        gone.push(i);
                        continue;
                    }
                }
                State::Animating => {}
            }
            let Some(seq) = a.seq.clone() else {
                continue;
            };
            let rate = if a.state == State::Animating {
                match a.group {
                    g if (group::FORWARD..=group::TURN_RIGHT).contains(&g) => self.movement_rate,
                    g if g > group::UNEQUIP && g <= 0xa8 => self.weapon_rate,
                    _ => 1.0,
                }
            } else {
                1.0
            };
            a.time += dt * rate;
            let d = a.data;
            let at = seq.start + a.time;
            let ends = if a.loops(&seq) {
                if at >= d.loop_end {
                    if a.loops_left != 0 {
                        a.time -= (d.loop_end - d.loop_start).max(1e-3);
                        if a.loops_left > 0 {
                            a.loops_left -= 1;
                        }
                        false
                    } else {
                        at >= d.end
                    }
                } else {
                    false
                }
            } else {
                at >= d.end
            };
            if ends && a.playing() {
                finished.push(Finished {
                    section: a.section,
                    group: a.group,
                });
            }
        }
        for i in gone.into_iter().rev() {
            self.active.remove(i);
        }
        for f in &finished {
            if f.section == section::WEAPON && self.weapon_drawn {
                self.stop_weapon_up_down();
                continue;
            }
            if let Some(i) = self.current(f.section) {
                if self.active[i].group == f.group {
                    let blend = if self.skip_blend {
                        0.0
                    } else {
                        stop_blend_seconds(&self.active[i].data, &self.settings)
                    };
                    self.ease_out(i, blend);
                }
            }
        }
        // The update over, `cSkipNextBlend` is cleared (`00491180`).
        self.skip_blend = false;
        finished
    }

    /// The bones' local transforms now: each sequence's pose for the
    /// bone blended by priority and weight (Gamebryo's blend
    /// interpolators), over the skeleton's own where nothing moves it.
    ///
    /// The accumulation root (`Bip01`) carries the actor's own movement from
    /// where it stands: where no sequence moves it (the idle doesn't) it
    /// sits at the actor's origin unturned, as [`nif::posed`] has it, not
    /// as the skeleton file stores it (68 up and turned 90°: idle people
    /// stood a quarter turn off their heading, and off their walk, whose
    /// `Bip01` keys are unturned).
    pub fn locals(&self, bones: &[Bone]) -> Vec<Transform> {
        let roots = self.accum_roots();
        let mut items: Vec<Item> = Vec::new();
        bones
            .iter()
            .enumerate()
            .map(|(b, bone)| {
                items.clear();
                for a in &self.active {
                    let (spinner, factor) = a.spinner_and_factor();
                    let weight = a.weight * factor;
                    match &a.seq {
                        Some(seq) => {
                            if let Some(t) = a.bone_tracks.get(b).copied().flatten() {
                                let track: &Track = &seq.tracks[t];
                                items.push(Item {
                                    priority: track.priority,
                                    weight,
                                    spinner,
                                    pose: track.sample(a.sample_time(seq)),
                                });
                            }
                        }
                        None => {
                            if let Some((priority, pose)) = a.frozen.get(b).copied().flatten() {
                                items.push(Item {
                                    priority,
                                    weight,
                                    spinner,
                                    pose,
                                });
                            }
                        }
                    }
                }
                let own = if roots.iter().any(|r| r.eq_ignore_ascii_case(&bone.name)) {
                    Transform::IDENTITY
                } else {
                    bone.local
                };
                blend_items(&items, &own)
            })
            .collect()
    }

    /// The accumulation roots of the sequences active.
    fn accum_roots(&self) -> Vec<&str> {
        self.active
            .iter()
            .filter_map(|a| a.seq.as_ref())
            .filter_map(|s| s.accum_root.as_deref())
            .collect()
    }

    /// The bones' transforms in the skeleton's space (parents applied),
    /// with the accumulation root kept at the origin as [`nif::posed`]
    /// keeps it (its travel is the actor's own movement).
    pub fn pose(&self, bones: &[Bone]) -> Vec<Transform> {
        let locals = self.locals(bones);
        let roots = self.accum_roots();
        let mut world: Vec<Transform> = Vec::with_capacity(bones.len());
        for (bone, local) in bones.iter().zip(locals) {
            let mut local = local;
            if roots.iter().any(|r| r.eq_ignore_ascii_case(&bone.name)) {
                local.translation = [0.0; 3];
            }
            let w = match bone.parent.and_then(|p| world.get(p)) {
                Some(parent) => parent.then_child(&local),
                None => local,
            };
            world.push(w);
        }
        world
    }

    /// How far the actor moves this frame by its movement animation
    /// (`00491180`): while the movement sequence is easing in (or blending
    /// from a pose) the group's travel a second × dt × the rate; while
    /// animating, the root's travel over the frame (the same on the game's
    /// files, whose roots move evenly). In the skeleton's axes, before the
    /// actor's scale and heading.
    pub fn movement_this_frame(&self, dt: f32) -> [f32; 3] {
        let Some(i) = self.current(section::MOVEMENT) else {
            return [0.0; 3];
        };
        let a = &self.active[i];
        let t = a.data.travel;
        let k = dt * self.movement_rate;
        [t[0] * k, t[1] * k, 0.0]
    }
}

/// One sequence's say on one bone.
#[derive(Debug, Clone, Copy)]
struct Item {
    priority: u8,
    weight: f32,
    spinner: f32,
    pose: Pose,
}

/// The sequence's track index for each bone (by name).
fn bone_tracks(seq: &Sequence, bones: &[Bone]) -> Vec<Option<usize>> {
    bones
        .iter()
        .map(|b| {
            seq.tracks
                .iter()
                .position(|t| t.node.eq_ignore_ascii_case(&b.name))
        })
        .collect()
}

/// Gamebryo's normalized weights (`NiBlendInterpolator::
/// ComputeNormalizedWeights`, `00a37260`; two items `00a36bd0`): the
/// items of the highest priority share `highEase` = the largest ease
/// spinner among them, the next priority down shares `1 − highEase`,
/// lower ones get nothing; each item's share is its weight × its spinner,
/// normalized over the total.
pub fn normalized_weights(items: &[(u8, f32, f32)]) -> Vec<f32> {
    if items.len() == 1 {
        return vec![1.0];
    }
    let high = items.iter().map(|i| i.0).max().unwrap_or(0);
    let next = items.iter().map(|i| i.0).filter(|&p| p < high).max();
    let mut high_sum = 0.0;
    let mut high_ease = 0.0f32;
    let mut next_sum = 0.0;
    for &(p, w, e) in items {
        if p == high {
            high_sum += w * e;
            high_ease = high_ease.max(e);
        } else if Some(p) == next {
            next_sum += w * e;
        }
    }
    let total = high_ease * high_sum + (1.0 - high_ease) * next_sum;
    let over = if total > 0.0 { 1.0 / total } else { 0.0 };
    items
        .iter()
        .map(|&(p, w, e)| {
            if p == high {
                w * high_ease * e * over
            } else if Some(p) == next {
                w * (1.0 - high_ease) * e * over
            } else {
                0.0
            }
        })
        .collect()
}

/// Blends the items' poses by their normalized weights
/// (`NiBlendTransformInterpolator::BlendValues`, as Gamebryo 2 does it:
/// translations and scales averaged, rotations accumulated by slerp with
/// their signs matched; a channel an item lacks leaves it out of that
/// channel, the others renormalized), over the bone's own transform.
fn blend_items(items: &[Item], own: &Transform) -> Transform {
    if items.is_empty() {
        return *own;
    }
    let weights = normalized_weights(
        &items
            .iter()
            .map(|i| (i.priority, i.weight, i.spinner))
            .collect::<Vec<_>>(),
    );
    let mut out = *own;
    // Translation.
    let (mut sum, mut total) = ([0.0f32; 3], 0.0f32);
    for (item, w) in items.iter().zip(&weights) {
        if let Some(t) = item.pose.translation {
            for k in 0..3 {
                sum[k] += t[k] * w;
            }
            total += w;
        }
    }
    if total > 0.0 {
        out.translation = sum.map(|v| v / total);
    }
    // Rotation.
    let mut rotation: Option<Quat> = None;
    let mut total = 0.0f32;
    for (item, &w) in items.iter().zip(&weights) {
        let Some(q) = item.pose.rotation else {
            continue;
        };
        if w <= 0.0 {
            continue;
        }
        rotation = Some(match rotation {
            None => q,
            Some(acc) => {
                let q = if dot(acc, q) < 0.0 { q.map(|c| -c) } else { q };
                slerp(acc, q, w / (total + w))
            }
        });
        total += w;
    }
    if let Some(q) = rotation {
        out.rotation = nif::anim::quat_matrix(q);
    }
    // Scale.
    let (mut sum, mut total) = (0.0f32, 0.0f32);
    for (item, w) in items.iter().zip(&weights) {
        if let Some(s) = item.pose.scale {
            sum += s * w;
            total += w;
        }
    }
    if total > 0.0 {
        out.scale = sum / total;
    }
    out
}

fn dot(a: Quat, b: Quat) -> f32 {
    a.iter().zip(b).map(|(x, y)| x * y).sum()
}

fn normalize_quat(q: Quat) -> Quat {
    let len = dot(q, q).sqrt();
    if len > 0.0 {
        q.map(|c| c / len)
    } else {
        [1.0, 0.0, 0.0, 0.0]
    }
}

/// Spherical interpolation from `a` to `b` by `f` (the signs already
/// matched).
fn slerp(a: Quat, b: Quat, f: f32) -> Quat {
    let d = dot(a, b).clamp(-1.0, 1.0);
    if d > 0.9995 {
        return normalize_quat([0, 1, 2, 3].map(|k| a[k] + (b[k] - a[k]) * f));
    }
    let theta = d.acos();
    let s = theta.sin();
    let wa = ((1.0 - f) * theta).sin() / s;
    let wb = (f * theta).sin() / s;
    normalize_quat([0, 1, 2, 3].map(|k| wa * a[k] + wb * b[k]))
}

/// A unit quaternion (w, x, y, z) from a rotation matrix.
pub fn quat_from_matrix(m: &nif::math::Mat3) -> Quat {
    let trace = m[0][0] + m[1][1] + m[2][2];
    let q = if trace > 0.0 {
        let s = (trace + 1.0).sqrt() * 2.0;
        [
            0.25 * s,
            (m[2][1] - m[1][2]) / s,
            (m[0][2] - m[2][0]) / s,
            (m[1][0] - m[0][1]) / s,
        ]
    } else if m[0][0] > m[1][1] && m[0][0] > m[2][2] {
        let s = (1.0 + m[0][0] - m[1][1] - m[2][2]).sqrt() * 2.0;
        [
            (m[2][1] - m[1][2]) / s,
            0.25 * s,
            (m[0][1] + m[1][0]) / s,
            (m[0][2] + m[2][0]) / s,
        ]
    } else if m[1][1] > m[2][2] {
        let s = (1.0 + m[1][1] - m[0][0] - m[2][2]).sqrt() * 2.0;
        [
            (m[0][2] - m[2][0]) / s,
            (m[0][1] + m[1][0]) / s,
            0.25 * s,
            (m[1][2] + m[2][1]) / s,
        ]
    } else {
        let s = (1.0 + m[2][2] - m[0][0] - m[1][1]).sqrt() * 2.0;
        [
            (m[1][0] - m[0][1]) / s,
            (m[0][2] + m[2][0]) / s,
            (m[1][2] + m[2][1]) / s,
            0.25 * s,
        ]
    };
    normalize_quat(q)
}

#[cfg(test)]
mod tests {
    use super::*;
    use nif::anim::Motion;

    fn bone(name: &str, parent: Option<usize>, z: f32) -> Bone {
        Bone {
            name: name.into(),
            parent,
            local: Transform {
                translation: [0.0, 0.0, z],
                ..Transform::IDENTITY
            },
        }
    }

    /// A sequence moving `node` to `at` (constant), with keys.
    fn still(name: &str, node: &str, at: [f32; 3], priority: u8, keys: &[(f32, &str)]) -> Sequence {
        Sequence {
            name: name.into(),
            start: 0.0,
            stop: 1.0,
            looping: true,
            accum_root: Some("Bip01".into()),
            materials: Vec::new(),
            text_keys: keys.iter().map(|(t, k)| (*t, k.to_string())).collect(),
            tracks: vec![Track {
                node: node.into(),
                priority,
                motion: Motion::Keys {
                    translation: vec![(0.0, at)],
                    rotation: Vec::new(),
                    scale: Vec::new(),
                    default: (None, None, None),
                    euler: None,
                },
            }],
        }
    }

    /// A walk: the root travels `travel` over `length` seconds, the arm
    /// held at `arm`.
    fn walk(travel: [f32; 3], length: f32, arm: [f32; 3], looping: bool) -> Sequence {
        Sequence {
            name: "Forward".into(),
            start: 0.0,
            stop: length,
            looping,
            accum_root: Some("Bip01".into()),
            materials: Vec::new(),
            text_keys: vec![(0.0, "start".into()), (0.033, "Blend:6".into())],
            tracks: vec![
                Track {
                    node: "Bip01".into(),
                    priority: 30,
                    motion: Motion::Keys {
                        translation: vec![(0.0, [0.0; 3]), (length, travel)],
                        rotation: Vec::new(),
                        scale: Vec::new(),
                        default: (None, None, None),
                        euler: None,
                    },
                },
                Track {
                    node: "Arm".into(),
                    priority: 30,
                    motion: Motion::Keys {
                        translation: vec![(0.0, arm)],
                        rotation: Vec::new(),
                        scale: Vec::new(),
                        default: (None, None, None),
                        euler: None,
                    },
                },
            ],
        }
    }

    fn skeleton() -> Vec<Bone> {
        vec![bone("Bip01", None, 68.0), bone("Arm", Some(0), 10.0)]
    }

    #[test]
    fn the_table_matches_the_exe() {
        assert_eq!(GROUPS.len(), 245);
        assert_eq!(group_named("Forward"), Some(group::FORWARD));
        assert_eq!(group_named("fastright"), Some(group::FAST_RIGHT));
        assert_eq!(group_named("JumpLandRight"), Some(244));
        assert_eq!(section_of(group::AIM), section::WEAPON);
        assert_eq!(section_of(group::ATTACK_RIGHT), section::WEAPON);
        assert_eq!(section_of(group::SPECIAL_IDLE), section::SPECIAL_IDLE);
        // Death's "whole body" counts as the movement section.
        assert_eq!(section_of(224), section::MOVEMENT);
        assert_eq!(kind_of(group::ATTACK_LEFT), 5);
        assert_eq!(kind_of(group::RELOAD_A), 0);
    }

    #[test]
    fn text_keys_give_the_blend_frames_and_times() {
        let seq = still(
            "Backward",
            "Bip01",
            [0.0; 3],
            30,
            &[
                (0.0, "start"),
                (0.033, "Blend: 9"),
                (0.2, "Enum: Left"),
                (1.133, "end"),
            ],
        );
        let d = GroupData::read(&seq);
        assert_eq!((d.blend, d.blend_in, d.blend_out), (9, 0, 0));
        assert_eq!(d.blend_in_frames(), 9);
        assert_eq!(d.start, 0.0);
        assert!((d.end - 1.133).abs() < 1e-6);
        // Without a space, and BlendIn/BlendOut on their own.
        let seq = still(
            "AttackRight",
            "Arm",
            [0.0; 3],
            55,
            &[(0.0, "start"), (0.467, "BlendIn:1"), (0.5, "BlendOut: 4")],
        );
        let d = GroupData::read(&seq);
        assert_eq!((d.blend, d.blend_in, d.blend_out), (0, 1, 4));
        assert_eq!((d.blend_in_frames(), d.blend_out_frames()), (1, 4));
        // No end key: the sequence's own end; loops default to the whole.
        assert_eq!(d.end, 1.0);
        assert_eq!((d.loop_start, d.loop_end), (0.0, 1.0));
    }

    #[test]
    fn blend_times_follow_the_games_rules() {
        let s = Settings::default();
        let walk = GroupData {
            blend: 6,
            blend_in: 0,
            blend_out: 0,
            start: 0.0,
            end: 1.2,
            loop_start: 0.0,
            loop_end: 1.2,
            travel: [0.0; 3],
            attach: 0.0,
        };
        let run = GroupData { blend: 9, ..walk };
        let plain = GroupData { blend: 0, ..walk };
        // Nothing set: the default 0.2 s.
        assert!((blend_seconds(None, &plain, &s) - 0.2).abs() < 1e-6);
        // The larger of the old's blend-out and the new's blend-in frames.
        assert!((blend_seconds(Some(&walk), &run, &s) - 0.3).abs() < 1e-6);
        assert!((blend_seconds(Some(&run), &walk, &s) - 0.3).abs() < 1e-6);
        assert!((blend_seconds(None, &walk, &s) - 0.2).abs() < 1e-6);
        // Stopping: the group's blend-out frames, else the default.
        assert!((stop_blend_seconds(&run, &s) - 0.3).abs() < 1e-6);
        assert!((stop_blend_seconds(&plain, &s) - 0.2).abs() < 1e-6);
        // fAnimationMult divides.
        let fast = Settings {
            default_blend: 0.2,
            mult: 2.0,
        };
        assert!((blend_seconds(None, &plain, &fast) - 0.1).abs() < 1e-6);
    }

    #[test]
    fn movement_groups_and_rates_as_pick_animations() {
        let f = MoveFlags {
            forward: true,
            ..Default::default()
        };
        assert_eq!(movement_group(f, 77.0), Some(group::FORWARD));
        assert_eq!(
            movement_group(MoveFlags { running: true, ..f }, 308.0),
            Some(group::FAST_FORWARD)
        );
        assert_eq!(
            movement_group(
                MoveFlags {
                    left: true,
                    ..Default::default()
                },
                77.0
            ),
            Some(group::LEFT)
        );
        // Too slow: the idle.
        assert_eq!(movement_group(f, 0.5), None);
        // Turning only without a direction.
        assert_eq!(
            movement_group(
                MoveFlags {
                    turn_left: true,
                    ..Default::default()
                },
                0.0
            ),
            Some(group::TURN_LEFT)
        );
        assert_eq!(
            movement_group(
                MoveFlags {
                    turn_right: true,
                    ..f
                },
                77.0
            ),
            Some(group::FORWARD)
        );
        // The walk's 85.3 units a second counts as 85: 77 / 85.
        assert!((movement_rate(77.0, 85.3) - 77.0 / 85.0).abs() < 1e-6);
        assert_eq!(movement_rate(77.0, 0.0), 1.0);
    }

    #[test]
    fn higher_priority_wins_and_lower_fills_in_while_it_fades() {
        // Idle (10) at full; a walk (30) half eased in: the walk's share
        // is spinner × (weight × spinner) over the total, the idle gets
        // (1 − spinner) × its own: 1/3 and 2/3 (the exe's `00a37260`).
        let w = normalized_weights(&[(10, 1.0, 1.0), (30, 1.0, 0.5)]);
        assert!(
            (w[0] - 2.0 / 3.0).abs() < 1e-6 && (w[1] - 1.0 / 3.0).abs() < 1e-6,
            "{w:?}"
        );
        // Nearly in: 0.9² / (0.9² + 0.1).
        let w = normalized_weights(&[(10, 1.0, 1.0), (30, 1.0, 0.9)]);
        assert!((w[1] - 0.81 / 0.91).abs() < 1e-6, "{w:?}");
        // Fully in: the idle gets nothing.
        let w = normalized_weights(&[(10, 1.0, 1.0), (30, 1.0, 1.0)]);
        assert_eq!(w, vec![0.0, 1.0]);
        // Two of the same priority cross-fading: by their spinners.
        let w = normalized_weights(&[(30, 1.0, 0.75), (30, 1.0, 0.25)]);
        assert!((w[0] - 0.75).abs() < 1e-6 && (w[1] - 0.25).abs() < 1e-6);
        // A third, lower one never shows; the next one down takes the rest.
        let w = normalized_weights(&[(10, 1.0, 1.0), (30, 1.0, 0.5), (25, 1.0, 1.0)]);
        assert!(
            (w[0]).abs() < 1e-6 && (w[2] - 2.0 / 3.0).abs() < 1e-6,
            "{w:?}"
        );
        // Weight (a blend from a pose) rather than spinner: the lower
        // priority stays out.
        let w = normalized_weights(&[(10, 1.0, 1.0), (30, 0.3, 1.0), (30, 0.7, 1.0)]);
        assert!(w[0].abs() < 1e-6 && (w[1] - 0.3).abs() < 1e-6 && (w[2] - 0.7).abs() < 1e-6);
    }

    #[test]
    fn a_walk_cross_fades_in_over_its_blend_and_the_idle_shows_through() {
        let bones = skeleton();
        let idle = Arc::new(still(
            "Idle",
            "Arm",
            [0.0, 0.0, 10.0],
            10,
            &[(0.0, "start")],
        ));
        let walk = Arc::new(walk([0.0, 102.3, 0.0], 1.2, [0.0, 0.0, 20.0], true));
        let mut p = Player::default();
        assert!(p.play(group::IDLE, &idle, -1, &bones));
        assert!(!p.play(group::IDLE, &idle, -1, &bones));
        p.update(1.0);
        // Nothing in the movement section: the walk blends in from the pose
        // (the arm at 10) over Blend:6 = 0.2 s.
        assert!(p.play(group::FORWARD, &walk, -1, &bones));
        assert_eq!(p.state(section::MOVEMENT), Some(State::TransDest));
        let arm = |p: &Player| p.locals(&bones)[1].translation[2];
        assert!((arm(&p) - 10.0).abs() < 1e-4, "{}", arm(&p));
        p.update(0.1);
        assert!((arm(&p) - 15.0).abs() < 1e-3, "halfway: {}", arm(&p));
        p.update(0.1);
        assert!((arm(&p) - 20.0).abs() < 1e-3, "{}", arm(&p));
        assert_eq!(p.state(section::MOVEMENT), Some(State::Animating));
        // Stopping: the walk eases out over its blend-out (0.2 s) and the
        // idle (lower priority) fills in as it goes: a quarter of the way,
        // at a spinner of 0.75, the walk's share is 0.75² / (0.75² + 0.25).
        p.stop_section(section::MOVEMENT);
        assert_eq!(p.playing(section::MOVEMENT), None);
        p.update(0.05);
        let share = 0.75 * 0.75 / (0.75 * 0.75 + 0.25);
        assert!(
            (arm(&p) - (10.0 + 10.0 * share)).abs() < 1e-3,
            "{}",
            arm(&p)
        );
        p.update(0.2);
        assert!((arm(&p) - 10.0).abs() < 1e-4, "{}", arm(&p));
        assert_eq!(p.all(), vec![(group::IDLE, State::Animating)]);
    }

    #[test]
    fn a_run_cross_fades_from_the_walk_with_the_larger_blend() {
        let bones = skeleton();
        let mut run = walk([0.0, 283.1, 0.0], 0.8, [0.0, 0.0, 40.0], true);
        let walk = Arc::new(walk([0.0, 102.3, 0.0], 1.2, [0.0, 0.0, 20.0], true));
        run.name = "FastForward".into();
        run.text_keys = vec![(0.0, "start".into()), (0.033, "Blend:9".into())];
        let run = Arc::new(run);
        let mut p = Player::default();
        p.play(group::FORWARD, &walk, -1, &bones);
        p.update(0.5);
        p.play(group::FAST_FORWARD, &run, -1, &bones);
        // Both playing: the walk easing out, the run in, over 0.3 s.
        assert_eq!(
            p.all(),
            vec![
                (group::FORWARD, State::EaseOut),
                (group::FAST_FORWARD, State::EaseIn)
            ]
        );
        p.update(0.15);
        let arm = p.locals(&bones)[1].translation[2];
        assert!((arm - 30.0).abs() < 1e-3, "{arm}");
        p.update(0.2);
        assert_eq!(p.all(), vec![(group::FAST_FORWARD, State::Animating)]);
    }

    #[test]
    fn deferred_script_idle_replaces_startup_from_the_current_pose() {
        let bones = skeleton();
        let first = Arc::new(still(
            "SpecialIdle",
            "Arm",
            [0.0, 0.0, 20.0],
            10,
            &[(0.0, "start"), (0.0, "BlendOut:30")],
        ));
        let second = Arc::new(still(
            "SpecialIdle",
            "Arm",
            [0.0, 0.0, 40.0],
            10,
            &[(0.0, "start"), (0.0, "BlendIn:3")],
        ));
        let mut p = Player::default();
        p.play_script_idle(&first, -1, &bones);
        p.update(0.05);
        let before = p.locals(&bones);
        // Flag 0x80 does not wait for TransDest, and the old BlendOut
        // must not lengthen the new sequence's three-frame blend-in.
        p.play_script_idle(&second, 0, &bones);
        assert_eq!(p.state(section::SPECIAL_IDLE), Some(State::TransDest));
        assert_eq!(p.locals(&bones), before);
        assert_eq!(p.all(), vec![(group::SPECIAL_IDLE, State::TransDest)]);
        p.update(0.1);
        assert_eq!(p.state(section::SPECIAL_IDLE), Some(State::Animating));
        p.update(0.05);
        p.play_script_idle(&second, 0, &bones);
        assert_eq!(p.time(section::SPECIAL_IDLE), Some(0.0));
        assert_eq!(p.state(section::SPECIAL_IDLE), Some(State::TransDest));
    }

    #[test]
    fn special_idle_requests_wait_for_the_initial_transition_and_keep_duplicates() {
        let bones = skeleton();
        let first = Arc::new(still(
            "SpecialIdle",
            "Arm",
            [0.0, 0.0, 20.0],
            10,
            &[(0.0, "start")],
        ));
        let second = Arc::new(still(
            "SpecialIdle",
            "Arm",
            [0.0, 0.0, 40.0],
            10,
            &[(0.0, "start")],
        ));
        let mut p = Player::default();

        assert!(p.request_special_idle(&first, -1, &bones));
        assert_eq!(p.state(section::SPECIAL_IDLE), Some(State::TransDest));
        assert!(!p.request_special_idle(&second, -1, &bones));
        assert!(!p.free_special_idle());
        assert!(Arc::ptr_eq(
            p.sequence(section::SPECIAL_IDLE).unwrap(),
            &first
        ));
        assert_eq!(p.state(section::SPECIAL_IDLE), Some(State::TransDest));

        p.update(0.2);
        assert_eq!(p.state(section::SPECIAL_IDLE), Some(State::Animating));
        p.update(0.07);
        let first_time = p.time(section::SPECIAL_IDLE).unwrap();
        assert!(!p.request_special_idle(&first, -1, &bones));
        assert!(Arc::ptr_eq(
            p.sequence(section::SPECIAL_IDLE).unwrap(),
            &first
        ));
        assert_eq!(p.time(section::SPECIAL_IDLE), Some(first_time));

        assert!(p.request_special_idle(&second, -1, &bones));
        assert_eq!(p.state(section::SPECIAL_IDLE), Some(State::TransDest));
        assert!(!p.request_special_idle(&first, -1, &bones));
        assert!(!p.free_special_idle());
        assert!(Arc::ptr_eq(
            p.sequence(section::SPECIAL_IDLE).unwrap(),
            &second
        ));
        p.update(0.2);
        assert!(p.free_special_idle());
        assert!(p.all().contains(&(group::SPECIAL_IDLE, State::EaseOut)));
        p.update(0.2);
        assert!(p.sequence(section::SPECIAL_IDLE).is_none());
    }

    #[test]
    fn movement_sequences_run_at_the_rate_once_animating() {
        let bones = skeleton();
        let walk = Arc::new(walk([0.0, 102.3, 0.0], 1.2, [0.0; 3], true));
        let mut p = Player {
            movement_rate: movement_rate(77.0, 85.3),
            ..Default::default()
        };
        p.play(group::FORWARD, &walk, -1, &bones);
        // Blending in (0.2 s): the file's own rate.
        p.update(0.1);
        assert!((p.time(section::MOVEMENT).unwrap() - 0.1).abs() < 1e-5);
        // Then 77/85 of real time (from the frame the blend ends on, as
        // the game applies it after the manager's update).
        p.update(0.1);
        p.update(1.0);
        let t = p.time(section::MOVEMENT).unwrap();
        assert!((t - (0.1 + 1.1 * 77.0 / 85.0)).abs() < 1e-4, "{t}");
        // The movement this frame: the group's travel a second × rate.
        let m = p.movement_this_frame(0.1);
        assert!(
            (m[1] - 102.3 / 1.2 * 77.0 / 85.0 * 0.1).abs() < 1e-3,
            "{m:?}"
        );
        // Looping: past the end it wraps.
        p.update(1.0);
        assert!(p.time(section::MOVEMENT).unwrap() < 1.2);
        assert_eq!(p.playing(section::MOVEMENT), Some(group::FORWARD));
    }

    #[test]
    fn a_one_shot_ends_at_its_end_key_and_eases_out() {
        let bones = skeleton();
        let mut attack = walk([0.0; 3], 0.5, [0.0, 0.0, 30.0], false);
        attack.name = "AttackRight".into();
        attack.text_keys = vec![
            (0.0, "start".into()),
            (0.4, "a:R".into()),
            (0.467, "BlendIn:1".into()),
            (0.5, "end".into()),
        ];
        let attack = Arc::new(attack);
        let idle = Arc::new(still("Idle", "Arm", [0.0; 3], 10, &[(0.0, "start")]));
        let mut p = Player::default();
        p.play(group::IDLE, &idle, -1, &bones);
        p.play(group::ATTACK_RIGHT, &attack, 0, &bones);
        // BlendIn:1 frame = 0.033 s from the pose.
        assert!((p.active.last().unwrap().ease_length - 1.0 / 30.0).abs() < 1e-6);
        assert!(p.update(0.3).is_empty());
        let done = p.update(0.25);
        assert_eq!(
            done,
            vec![Finished {
                section: section::WEAPON,
                group: group::ATTACK_RIGHT
            }]
        );
        // Easing out over the default 0.2 s, then gone.
        assert_eq!(p.playing(section::WEAPON), None);
        assert!(p.all().contains(&(group::ATTACK_RIGHT, State::EaseOut)));
        p.update(0.25);
        assert_eq!(p.all(), vec![(group::IDLE, State::Animating)]);
    }

    #[test]
    fn skip_next_blend_switches_at_once_for_one_update() {
        // Sitting down as the game ends it: the entry (Blend:15, 0.5 s)
        // over the seated loop; the procedure turns the actor and sets
        // `cSkipNextBlend` before freeing the entry (`009213e0`).
        let bones = skeleton();
        let seat = Arc::new(still("Seat", "Arm", [0.0, 0.0, 5.0], 35, &[(0.0, "start")]));
        let entry = Arc::new(still(
            "Entry",
            "Arm",
            [0.0, 0.0, 50.0],
            80,
            &[(0.0, "start"), (0.5, "Blend:15"), (1.0, "end")],
        ));
        let mut p = Player::default();
        p.play(group::DYNAMIC_IDLE, &seat, -1, &bones);
        p.play(group::SPECIAL_IDLE, &entry, 0, &bones);
        p.update(0.6);
        // Without the flag the entry would ease out over its 15 frames.
        let mut eased = p.clone();
        eased.stop_section(section::SPECIAL_IDLE);
        assert!(eased.all().contains(&(group::SPECIAL_IDLE, State::EaseOut)));
        // With it the entry is gone at once and the seated loop shows.
        p.skip_next_blend();
        p.stop_section(section::SPECIAL_IDLE);
        assert_eq!(p.all(), vec![(group::DYNAMIC_IDLE, State::Animating)]);
        let arm = p.locals(&bones)[1].translation;
        assert_eq!(arm, [0.0, 0.0, 5.0]);
        // A group started while it's set cuts in without a blend too.
        let exit = Arc::new(still(
            "Exit",
            "Arm",
            [0.0, 0.0, 40.0],
            80,
            &[(0.0, "start"), (0.6, "Blend:15")],
        ));
        assert!(p.play(group::SPECIAL_IDLE, &exit, 0, &bones));
        assert_eq!(p.state(section::SPECIAL_IDLE), Some(State::Animating));
        assert_eq!(p.locals(&bones)[1].translation, [0.0, 0.0, 40.0]);
        // The update clears it: the next change blends again.
        p.update(0.1);
        assert!(!p.skips_next_blend());
        p.stop_section(section::SPECIAL_IDLE);
        assert!(p.all().contains(&(group::SPECIAL_IDLE, State::EaseOut)));
    }

    #[test]
    fn a_looping_group_counts_its_loops() {
        let bones = skeleton();
        let seq = Arc::new(walk([0.0; 3], 1.0, [0.0; 3], true));
        let mut p = Player::default();
        // Once more after this one.
        p.play(group::FORWARD, &seq, 1, &bones);
        p.update(0.5);
        p.update(1.0);
        assert_eq!(p.playing(section::MOVEMENT), Some(group::FORWARD));
        assert!(p.update(0.4).is_empty());
        assert_eq!(p.update(0.2).len(), 1);
    }

    #[test]
    fn a_blend_from_pose_keeps_lower_priorities_out() {
        // The aim (25 on the arm) over the idle (10): blending the aim in
        // from the pose, the frozen pose and the aim share the arm, the
        // idle has no say.
        let bones = skeleton();
        let idle = Arc::new(still(
            "Idle",
            "Arm",
            [0.0, 0.0, 10.0],
            10,
            &[(0.0, "start")],
        ));
        let aim = Arc::new(still("Aim", "Arm", [0.0, 0.0, 50.0], 25, &[(0.0, "start")]));
        let mut p = Player::default();
        p.play(group::IDLE, &idle, -1, &bones);
        p.play(group::AIM, &aim, -1, &bones);
        p.update(0.1);
        let arm = p.locals(&bones)[1].translation[2];
        assert!((arm - 30.0).abs() < 1e-3, "{arm}");
        // Then stopping the aim eases it out: at a spinner of 0.5 its share
        // is 0.5 × 0.5 / (0.5 × 0.5 + 0.5 × 1) = 1/3, the idle's 2/3.
        p.update(0.2);
        p.stop_section(section::WEAPON);
        p.update(0.1);
        let arm = p.locals(&bones)[1].translation[2];
        assert!((arm - (10.0 * 2.0 + 50.0) / 3.0).abs() < 1e-3, "{arm}");
    }

    #[test]
    fn the_pose_keeps_the_root_at_the_origin_and_chains_parents() {
        let bones = skeleton();
        let walk = Arc::new(walk([0.0, 102.3, 0.0], 1.2, [0.0, 0.0, 20.0], true));
        let mut p = Player::default();
        p.play(group::FORWARD, &walk, -1, &bones);
        p.update(0.6);
        let pose = p.pose(&bones);
        assert_eq!(pose[0].translation, [0.0; 3]);
        assert!((pose[1].translation[2] - 20.0).abs() < 1e-4);
        // Without anything playing, the skeleton's own pose.
        let p = Player::default();
        let pose = p.pose(&bones);
        assert_eq!(pose[1].translation, [0.0, 0.0, 78.0]);
    }

    #[test]
    fn an_idle_that_leaves_the_root_alone_keeps_it_unturned() {
        // The skeleton file's `Bip01` is turned a quarter (as `skeleton.
        // nif`'s); the idle moves only the arm, the walk the root too.
        let quarter = [[0.0, -1.0, 0.0], [1.0, 0.0, 0.0], [0.0, 0.0, 1.0]];
        let mut bones = skeleton();
        bones[0].local.rotation = quarter;
        let idle = Arc::new(still("Idle", "Arm", [0.0, 0.0, 10.0], 10, &[]));
        let mut p = Player::default();
        p.play(group::IDLE, &idle, -1, &bones);
        p.update(1.0);
        let pose = p.pose(&bones);
        // Facing the actor's own way, at its origin, as when walking.
        assert_eq!(pose[0].rotation, Transform::IDENTITY.rotation);
        assert_eq!(pose[0].translation, [0.0; 3]);
        let walk = Arc::new(walk([0.0, 102.3, 0.0], 1.2, [0.0, 0.0, 20.0], true));
        p.play(group::FORWARD, &walk, -1, &bones);
        p.update(0.1);
        assert_eq!(p.pose(&bones)[0].rotation, Transform::IDENTITY.rotation);
    }

    #[test]
    fn rotations_blend_by_slerp_and_matrices_round_trip() {
        let s = std::f32::consts::FRAC_1_SQRT_2;
        let q = [s, 0.0, 0.0, s];
        let m = nif::anim::quat_matrix(q);
        let back = quat_from_matrix(&m);
        assert!(
            back.iter().zip(q).all(|(a, b)| (a - b).abs() < 1e-5),
            "{back:?}"
        );
        let half = slerp([1.0, 0.0, 0.0, 0.0], q, 0.5);
        let m = nif::anim::quat_matrix(half);
        // 45° about z: x goes to (cos 45, sin 45).
        assert!((m[0][0] - s).abs() < 1e-5 && (m[1][0] - s).abs() < 1e-5);
    }
}

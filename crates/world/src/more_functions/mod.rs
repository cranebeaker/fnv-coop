//! More of the game's script functions (a second round after
//! [`crate::script_functions`]), each carried out as its own handler in
//! `FalloutNV.exe` does it; the notes are in `%USERPROFILE%\nv-re\
//! findings\functions2.md`.
//!
//! [`value`] answers the ones that read; [`change`] carries out the ones
//! that change things (and, for a script, `GetChallengeCompleted`, whose
//! script and condition versions differ). What they keep is [`State`]
//! (`GameState::more`), saved with the game. What the viewer must show goes
//! out as [`Shown`] inside [`Event::More`].
//!
//! Some of what these functions ask is what the viewer sees and the world
//! doesn't keep: how people move, the last idle they played, the AI
//! procedure they're at. The viewer reports it ([`report`]); until it does
//! (or headless), people count as standing still, with no idle played and
//! no procedure (as the game's handlers answer for someone without an AI
//! process).

pub mod actors;
pub mod challenges;
pub mod destruction;
pub mod placed;
pub mod procedures;
pub mod radio;
pub mod shaders;
pub mod traps;

use std::collections::{HashMap, HashSet};

use esm::{FormId, FourCC, LoadOrder};

use crate::cell::le_u32;
use crate::dialogue::PLAYER_REF;
use crate::scripting::{Event, Facts, GameState, Runner, Value};

const ACBS: FourCC = FourCC::new(b"ACBS");
const TACT: FourCC = FourCC::new(b"TACT");
const XESP: FourCC = FourCC::new(b"XESP");
const NPC_: FourCC = FourCC::new(b"NPC_");
const CREA: FourCC = FourCC::new(b"CREA");
const FACT: FourCC = FourCC::new(b"FACT");
const SNAM: FourCC = FourCC::new(b"SNAM");

/// Something for the viewer to show, from these functions.
#[derive(Debug, Clone, PartialEq)]
pub enum Shown {
    /// `PlaceAtMe`: a reference made (`placed::Made` in the state).
    Placed { reference: FormId },
    /// `SetActorAlpha`: how see-through someone is (0 to 1).
    ActorAlpha { who: FormId, alpha: f32 },
    /// `SetGhost`: someone became a ghost (true) or stopped being one.
    Ghost { who: FormId, on: bool },
    /// `PlayMagicShaderVisuals`: an effect shader (`EFSH`) runs on a
    /// reference, for `seconds` or (`None`) until stopped.
    ShaderVisual {
        reference: FormId,
        shader: FormId,
        seconds: Option<f32>,
    },
    /// `StopMagicShaderVisuals`: the reference's effects with that shader
    /// ended.
    ShaderVisualStopped { reference: FormId, shader: FormId },
    /// `PushActorAway`: `who` is knocked down (the AI process's knock
    /// state 2) and thrown as a ragdoll from `from`'s centre with this
    /// force ([`actors::push_force`]).
    PushedAway {
        who: FormId,
        from: FormId,
        force: f32,
    },
    /// `FireWeapon`: a placed object fires a weapon ([`traps::shot_from`]).
    WeaponFired { from: FormId, weapon: FormId },
    /// `Autosave`, `ForceSave`, `SystemSave`: a save the game asks for.
    Save(SaveKind),
    /// `SetGlobalTimeMultiplier`: everything runs this much faster.
    TimeMultiplier(f32),
    /// `DamageObject` reached a destruction stage (counted from 1), or
    /// `ClearDestruction` made the object whole (stage 0): the stage's
    /// replacement model or its model's damage stage, its explosion and
    /// its debris with their count.
    Destruction {
        what: FormId,
        stage: u8,
        model: Option<String>,
        damage_stage: u8,
        explosion: Option<FormId>,
        debris: Option<(FormId, i32)>,
    },
}

/// The saves scripts ask for: `Autosave` (the save manager's +0xf),
/// `ForceSave` (+0x10), `SystemSave` (+0x11).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SaveKind {
    Autosave,
    Force,
    System,
}

/// The movement flags the game's AI keeps (`008846e0`, read by `IsMoving`,
/// `IsRunning`, `IsSneaking`).
pub mod movement {
    pub const FORWARD: u16 = 0x001;
    pub const BACK: u16 = 0x002;
    pub const LEFT: u16 = 0x004;
    pub const RIGHT: u16 = 0x008;
    pub const RUNNING: u16 = 0x200;
    pub const SNEAKING: u16 = 0x400;
    /// Cancels sneaking for `IsSneaking` (`004997b0`).
    pub const NOT_SNEAKING: u16 = 0x800;
    /// Turning: `IsTurning` gives 1 for 0x10 and 2 for 0x20 (`005a4160`;
    /// which way is which is the editor documentation's: left 1, right 2).
    pub const TURNING_LEFT: u16 = 0x010;
    pub const TURNING_RIGHT: u16 = 0x020;
}

/// What the viewer last saw of someone.
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct Seen {
    /// [`movement`] flags.
    pub movement: u16,
    /// The last idle played (the AI process's +0x10c).
    pub last_idle: Option<FormId>,
    /// `GetCurrentAIProcedure`'s number now ([`procedures`]).
    pub procedure: Option<i32>,
    /// In water deep enough to swim (actor +0x14d).
    pub swimming: bool,
    /// An idle playing now (`IsIdlePlaying`: the animation data's idle,
    /// `004985f0`).
    pub idle_playing: bool,
}

/// What these functions keep.
#[derive(Debug, Clone, Default)]
pub struct State {
    /// People who are ghosts (`SetGhost`: extra data 0x1F and the AI
    /// process's flag 0x10000000).
    pub ghosts: HashSet<FormId>,
    /// People's alpha as `SetActorAlpha` set it (the AI process's +0x5b0).
    pub alpha: HashMap<FormId, f32>,
    /// People alerted (`SetAlert`, the process's vtable +0x320).
    pub alerted: HashSet<FormId>,
    /// Bases whose essential flag (`ACBS` 0x02) a script changed
    /// (`SetEssential`), and references marked essential themselves
    /// (`SetActorRefEssential`: actor +0x140 bit 31).
    pub essential_bases: HashMap<FormId, bool>,
    pub essential_refs: HashSet<FormId>,
    /// Essential people brought to 0 health and down for now, with the
    /// seconds till they get up (`world::combat::hurt`).
    pub down: HashMap<FormId, f32>,
    /// People whose lines always show subtitles (actor +0x140 bit 30).
    pub always_subtitles: HashSet<FormId>,
    /// Combat styles scripts gave people (`SetCombatStyle`: extra data
    /// 0x69).
    pub combat_styles: HashMap<FormId, FormId>,
    /// Talking activators' speakers (`SetTalkingActivatorActor`: the
    /// `TACT` base's +0x90), by base.
    pub speakers: HashMap<FormId, FormId>,
    /// Radio stations switched on or off (`SetBroadcastState`), by base.
    pub broadcasting: HashMap<FormId, bool>,
    /// `ResetPipboyManager` asked the player's Pip-Boy manager to reset
    /// (its +0x16c; what reads it isn't traced). The radio functions act
    /// on `GameState::radio` ([`radio`]).
    pub pipboy_reset: bool,
    /// People's critical stage (actor +0x10c: 1 goo start, 2 goo end, 3
    /// disintegrate start, 4 disintegrate end).
    pub critical_stage: HashMap<FormId, i32>,
    /// Ash and goo piles `AttachAshPile` made, and the corpse each stands
    /// for (the `ExtraAshPileRef` both carry, `0041e340`), by pile.
    pub ash_piles: HashMap<FormId, FormId>,
    /// Placed mines disarmed or gone (`crate::mines`).
    pub mines: crate::mines::MineStates,
    /// Limbs gone (extra data 0x5f): nothing dismembers here yet.
    pub limbs_gone: HashSet<(FormId, u8)>,
    /// People told to sneak (`SetForceSneak`, actor +0x125) and whose AI is
    /// switched off (`SetActorsAI 0`, actor +0xbc).
    pub forced_sneak: HashSet<FormId>,
    pub ai_off: HashSet<FormId>,
    /// The player's "in character generation" flag (+0x75c).
    pub in_chargen: bool,
    /// `SetGlobalTimeMultiplier` (the time manager's multiplier, 1 when
    /// never set).
    pub time_multiplier: Option<f32>,
    /// `SetNoAvoidance` and `SetSceneIsComplex` (both write the global at
    /// `012682f8`), `SetAllVisible` (`011df678`): switches for the
    /// renderer and path finding, kept.
    pub no_avoidance: bool,
    pub all_visible: bool,
    /// What the viewer last saw of people (not saved).
    pub seen: HashMap<FormId, Seen>,
    /// The sequences active on placed objects' models this frame, by
    /// reference, as the viewer last reported them ([`report_sequences`];
    /// not saved). An object missing here has no 3D loaded.
    pub sequences: HashMap<FormId, Vec<String>>,
    /// The line of sight each actor's last detection run found to another
    /// (`GetLineOfSight` asks it, [`crate::sight`]), by (who, whom), as the
    /// viewer reports it ([`report_detection_sight`]; not saved).
    pub detection_sight: HashMap<(FormId, FormId), bool>,
    /// References with 3D loaded now (in an attached cell), as the viewer
    /// last reported them ([`report_loaded`]; not saved).
    pub loaded: HashSet<FormId>,
    /// Whether each person's spine faces up (`IsFacingUp`), as the viewer
    /// last reported it ([`report_facing_up`]; not saved). Someone missing
    /// has no 3D.
    pub facing_up: HashMap<FormId, bool>,
    /// Effect shaders scripts put on references ([`shaders`]; not saved).
    pub shader_visuals: Vec<shaders::ShaderVisual>,
    /// What scripts did to people's dispositions toward the player
    /// ([`actors`]), saved.
    pub dispositions: actors::Dispositions,
    /// How people died ([`actors::cause`]; the dismembered limbs extra
    /// data's +0x10), saved.
    pub cause_of_death: HashMap<FormId, i32>,
    /// References `SetVATSTarget` turned ([`traps`]), saved.
    pub vats_overrides: traps::VatsOverrides,
    /// The menu open now, while its `MenuMode` blocks run or the viewer
    /// shows it (not saved).
    pub menu_open: Option<u16>,
    pub challenges: challenges::Challenges,
    pub placed: placed::Placed,
    pub damaged: destruction::Damaged,
    /// Securitrons' faces (`SetSecuritronExpression`: extra data 0x8F with
    /// two names, the face's kind and its expression).
    pub securitron_faces: HashMap<FormId, (String, String)>,
    /// Points the SPECIAL and tag-skill menus give to spend
    /// (`AddSPECIALPoints`, `SetSPECIALPoints`, `AddTagSkills`: `007062a0`
    /// → `00753420`, kind 0 SPECIAL, 1 tag skills).
    pub special_points: i32,
    pub tag_points: i32,
    /// Bases whose value `SetItemValue` changed (`005d3e30` → `0048e960`:
    /// the form's value data, +4, for every one of them; the form marked
    /// changed, so saved).
    pub item_values: HashMap<FormId, i32>,
    /// The seconds the effect script running now covers
    /// (`ScriptEffectElapsedSeconds`; `None` outside one; not saved).
    pub effect_seconds: Option<f32>,
}

/// What happened, in plain words (for the viewer's and `nvinspect`'s
/// messages).
pub fn describe(order: &LoadOrder, state: &GameState, shown: &Shown) -> String {
    let name = |id: FormId| {
        order
            .get(id)
            .and_then(|r| r.editor_id().ok().flatten())
            .or_else(|| placed::base(state, id).and_then(|b| order.get(b)?.editor_id().ok()?))
            .unwrap_or_else(|| id.to_string())
    };
    match shown {
        Shown::Placed { reference } => match state.more.placed.refs.get(reference) {
            Some(m) => format!(
                "a {} is placed at {:.0},{:.0},{:.0} ({reference})",
                name(m.base),
                m.position[0],
                m.position[1],
                m.position[2]
            ),
            None => format!("{reference} is placed"),
        },
        Shown::ActorAlpha { who, alpha } => format!("{} fades to {alpha:.2}", name(*who)),
        Shown::Ghost { who, on } => format!(
            "{} {}",
            name(*who),
            if *on {
                "becomes a ghost"
            } else {
                "is no longer a ghost"
            }
        ),
        Shown::Save(kind) => format!(
            "the game asks for {}",
            match kind {
                SaveKind::Autosave => "an autosave",
                SaveKind::Force => "a forced save",
                SaveKind::System => "a system save",
            }
        ),
        Shown::TimeMultiplier(m) => format!("time runs {m}x as fast"),
        Shown::ShaderVisual {
            reference,
            shader,
            seconds,
        } => match seconds {
            Some(t) => format!("{} shows {} for {t} s", name(*reference), name(*shader)),
            None => format!("{} shows {}", name(*reference), name(*shader)),
        },
        Shown::ShaderVisualStopped { reference, shader } => {
            format!("{} stops showing {}", name(*reference), name(*shader))
        }
        Shown::PushedAway { who, from, force } => {
            format!("{} pushes {} away (force {force})", name(*from), name(*who))
        }
        Shown::WeaponFired { from, weapon } => {
            format!("{} fires {}", name(*from), name(*weapon))
        }
        Shown::Destruction {
            what,
            stage,
            model,
            damage_stage,
            explosion,
            debris,
        } => {
            if *stage == 0 {
                return format!("{} is whole again", name(*what));
            }
            let mut s = format!("{} reaches destruction stage {stage}", name(*what));
            match model {
                Some(m) => s += &format!(", model {m}"),
                None => s += &format!(", its model's damage stage {damage_stage}"),
            }
            if let Some(e) = explosion {
                s += &format!(", explosion {}", name(*e));
            }
            if let Some((d, n)) = debris {
                s += &format!(", {n} debris {}", name(*d));
            }
            s
        }
    }
}

/// The viewer tells what it saw of someone this frame.
pub fn report(state: &mut GameState, who: FormId, seen: Seen) {
    state.more.seen.insert(who, seen);
}

/// The viewer tells which sequences are active on each placed object's
/// model this frame (`NiControllerSequence` state, +0x44, not inactive),
/// replacing the last report: a script's `PlayGroup` sequence, held at its
/// end once played, or the ones the model plays from the start.
pub fn report_sequences(state: &mut GameState, sequences: HashMap<FormId, Vec<String>>) {
    state.more.sequences = sequences;
}

/// The viewer tells which references have 3D loaded now, replacing the
/// last report.
pub fn report_loaded(state: &mut GameState, loaded: HashSet<FormId>) {
    state.more.loaded = loaded;
}

/// The viewer tells, for each person with 3D, whether their spine node
/// (`Bip01 Spine`, else `Bip01 Spine01`) faces up: its world rotation's
/// [2][1] above 0 (`00c6b7b0`, the node's +0x84). Replaces the last report.
pub fn report_facing_up(state: &mut GameState, facing: HashMap<FormId, bool>) {
    state.more.facing_up = facing;
}

/// The viewer tells what `who`'s detection run found of `other`: whether
/// it had a line of sight (kept with its detection data, `008f6930`).
pub fn report_detection_sight(state: &mut GameState, who: FormId, other: FormId, sight: bool) {
    state.more.detection_sight.insert((who, other), sight);
}

/// The functions answered ([`value`]), by the game's own names.
pub const READS: &[&str] = &[
    "GetIsGhost",
    "GetIsAlerted",
    "IsEssential",
    "IsActorRefEssential",
    "IsActorTalkingThroughActivator",
    "GetBroadcastState",
    "IsLimbGone",
    "GetCauseofDeath",
    "IsFacingUp",
    "IsInCriticalStage",
    "IsSneaking",
    "IsRunning",
    "IsMoving",
    "IsLastIdlePlayed",
    "GetCurrentAIProcedure",
    "MenuMode",
    "GetInCharGen",
    "Sqrt",
    "IsActor",
    "IsWin32",
    "IsXBox",
    "GetIgnoreCrime",
    "GetIgnoreFriendlyHits",
    "GetXPForNextLevel",
    "GetParentRef",
    "GetIsObjectType",
    "IsPlayerGrabbedRef",
    "IsSwimming",
    "GetSandman",
    "GetDestructionStage",
    "GetChallengeCompleted",
    "IsTurning",
    "GetArmorRating",
    "GetFactionRankDifference",
    "IsCombatTarget",
    "IsIdlePlaying",
    "IsAnimPlaying",
    "GetIsAlignment",
];

/// The functions that change things ([`change`]), by the game's own names.
pub const CHANGES: &[&str] = &[
    "SetGhost",
    "SetActorAlpha",
    "SetAlert",
    "SetEssential",
    "SetActorRefEssential",
    "AlwaysShowActorSubtitles",
    "SetCombatStyle",
    "SetTalkingActivatorActor",
    "SetBroadcastState",
    "SetCriticalStage",
    "AttachAshPile",
    "SetForceSneak",
    "SetActorsAI",
    "SetInChargen",
    "SetNoAvoidance",
    "SetSceneIsComplex",
    "SetAllVisible",
    "SetAllReachable",
    "TrapUpdate",
    "PreloadMagicEffect",
    "Autosave",
    "ForceSave",
    "SystemSave",
    "SetGlobalTimeMultiplier",
    "ClearOwnership",
    "WakeUpPC",
    "IncrementScriptedChallenge",
    "RemoveRecurringFromChallenge",
    "PlaceAtMe",
    "PlaceLeveledActorAtMe",
    "DamageObject",
    "ClearDestruction",
    "ScriptEffectElapsedSeconds",
    "StopCombatAlarmOnActor",
    "SetSecuritronExpression",
    "AddSPECIALPoints",
    "SetSPECIALPoints",
    "AddTagSkills",
    "SetRumble",
];

/// Every function here.
pub fn handled() -> impl Iterator<Item = &'static str> {
    READS
        .iter()
        .chain(CHANGES)
        .chain(radio::CHANGES)
        .chain(shaders::CHANGES)
        .chain(actors::FUNCTIONS)
        .chain(traps::FUNCTIONS)
        .chain(crate::sight::FUNCTIONS)
        .copied()
}

/// The game's form type numbers (the byte at form +4), by record type:
/// `GetIsObjectType` compares a reference's base's.
pub const FORM_TYPES: [&str; 121] = [
    "NONE", "TES4", "GRUP", "GMST", "TXST", "MICN", "GLOB", "CLAS", "FACT", "HDPT", "HAIR", "EYES",
    "RACE", "SOUN", "ASPC", "SKIL", "MGEF", "SCPT", "LTEX", "ENCH", "SPEL", "ACTI", "TACT", "TERM",
    "ARMO", "BOOK", "CLOT", "CONT", "DOOR", "INGR", "LIGH", "MISC", "STAT", "SCOL", "MSTT", "PWAT",
    "GRAS", "TREE", "FLOR", "FURN", "WEAP", "AMMO", "NPC_", "CREA", "LVLC", "LVLN", "KEYM", "ALCH",
    "IDLM", "NOTE", "COBJ", "PROJ", "LVLI", "WTHR", "CLMT", "REGN", "NAVI", "CELL", "REFR", "ACHR",
    "ACRE", "PMIS", "PGRE", "PBEA", "PFLA", "WRLD", "LAND", "NAVM", "TLOD", "DIAL", "INFO", "QUST",
    "IDLE", "PACK", "CSTY", "LSCR", "LVSP", "ANIO", "WATR", "EFSH", "TOFT", "EXPL", "DEBR", "IMGS",
    "IMAD", "FLST", "PERK", "BPTD", "ADDN", "AVIF", "RADS", "CAMS", "CPTH", "VTYP", "IPCT", "IPDS",
    "ARMA", "ECZN", "MESG", "RGDL", "DOBJ", "LGTM", "MUSC", "IMOD", "REPU", "PCBE", "RCPE", "RCCT",
    "CHIP", "CSNO", "LSCT", "MSET", "ALOC", "CHAL", "AMEF", "CCRD", "CMNY", "CDCK", "DEHY", "HUNG",
    "SLPD",
];

/// A record type's form type number.
pub fn form_type_number(kind: FourCC) -> Option<u8> {
    FORM_TYPES
        .iter()
        .position(|t| t.as_bytes() == kind.as_bytes())
        .map(|i| i as u8)
}

fn kind_of(order: &LoadOrder, id: FormId) -> Option<FourCC> {
    order.get(id).map(|r| r.entry.header.kind)
}

/// Whether a reference is a person or creature (placed, or made by
/// `PlaceAtMe`).
pub fn is_actor(order: &LoadOrder, state: &GameState, r: FormId) -> bool {
    if crate::script_functions::is_actor(order, r) {
        return true;
    }
    placed::base(state, r)
        .and_then(|b| kind_of(order, b))
        .is_some_and(|k| k == NPC_ || k == CREA)
}

/// Whether someone is a ghost (`GetIsGhost`, `008ace90`), read where the game
/// reads it: a ghost isn't hit (the actor's hit handling `00899cb0` and its
/// reaction `008987f0` return at once; shots and explosions pass it by,
/// `00816f10`, `00817b90`, `00818ce0`), and noticing someone (`008ff350`)
/// starts no fight with or by a ghost.
pub fn is_ghost(state: &GameState, who: FormId) -> bool {
    state.more.ghosts.contains(&who)
}

/// Someone's combat style now: one a script gave them (`SetCombatStyle`),
/// else their record's ([`crate::combat_ai::CombatStyle::of`]).
pub fn combat_style(
    order: &LoadOrder,
    state: &GameState,
    who: FormId,
) -> crate::combat_ai::CombatStyle {
    state
        .more
        .combat_styles
        .get(&who)
        .and_then(|&s| crate::combat_ai::CombatStyle::load(order, s))
        .unwrap_or_else(|| crate::combat_ai::CombatStyle::of(order, who))
}

/// `IsEssential` (`005a3980` → `0087f3d0`): a player's teammate outside
/// hardcore (actor +0x18d, the player's +0x7bc), or the base's essential
/// flag (`ACBS` 0x02, as `SetEssential` left it; a leveled actor's
/// original base, extra data 0x2e, isn't kept here), or the reference's own
/// (`SetActorRefEssential`).
/// A critical stage's number by its name, as scripts write it
/// (`SetCriticalStage DisintegrateStart`; the game's names at `0119bbb0`:
/// None, GooStart, GooEnd, DisintegrateStart, DisintegrateEnd).
pub fn critical_stage_number(name: &str) -> Option<u8> {
    [
        "None",
        "GooStart",
        "GooEnd",
        "DisintegrateStart",
        "DisintegrateEnd",
    ]
    .iter()
    .position(|n| n.eq_ignore_ascii_case(name))
    .map(|i| i as u8)
}

/// The goo's and the disintegration's end (critical stages 2 and 4),
/// when the body's 3D is culled.
pub const GOO_END: i32 = 2;
pub const DISINTEGRATE_END: i32 = 4;

/// Whether someone's body is gone (critical stage [`GOO_END`] or
/// [`DISINTEGRATE_END`]: `008a1a70` takes its collision out of the Havok
/// world, `0057b520(0)`, and culls its 3D), for the viewer to stop
/// drawing it and letting shots, swings and the crosshair meet it.
pub fn body_gone(state: &GameState, who: FormId) -> bool {
    matches!(
        state.more.critical_stage.get(&who),
        Some(&GOO_END) | Some(&DISINTEGRATE_END)
    )
}

/// `SetCriticalStage` on an actor (`008a1a40`: actor +0x10c, then
/// `008a1a70`). Stages 2 and 4 (the goo's and the disintegration's end)
/// cull the body's 3D (`00450f90(1)`; the viewer stops drawing it,
/// [`body_gone`]) and, when the one disintegrated isn't the player and
/// their killer (actor +0xc0, which `Actor::Kill` `0089d900` writes) is,
/// count a Disintegration (miscellaneous statistic 29, `004d5c60(0x1d)`).
/// Stage 1's shader (`00c80ce0`, `00c803a0`) and stages 2 and 4's
/// `0057b520(0)` and vtable +0x1c0 aren't done here. The game warns
/// about an actor who isn't dead and carries on.
// Translated from 008a1a70 (decompiled, FalloutNV.exe 1.4.0.525).
pub fn set_critical_stage(state: &mut GameState, who: FormId, stage: i32) {
    state.more.critical_stage.insert(who, stage);
    if !matches!(stage, GOO_END | DISINTEGRATE_END) {
        return;
    }
    let killer = state
        .dead
        .contains(&who)
        .then(|| state.last_blow.get(&who).map(|b| b.0))
        .flatten();
    if who != PLAYER_REF && killer == Some(PLAYER_REF) {
        crate::stats::bump(state, crate::stats::DISINTEGRATIONS, 1);
    }
}

pub fn is_essential(order: &LoadOrder, state: &GameState, who: FormId) -> bool {
    if !is_actor(order, state, who) {
        return false;
    }
    let Some(base) = placed::base_now(order, state, who) else {
        return false;
    };
    if state.teammates.contains(&who) && !state.living.hardcore {
        return true;
    }
    let base_flag = match state.more.essential_bases.get(&base) {
        Some(&on) => on,
        None => order
            .get(base)
            .and_then(|rr| rr.record().ok())
            .and_then(|r| {
                r.get(ACBS)
                    .filter(|s| s.data.len() >= 4)
                    .map(|s| le_u32(&s.data, 0))
            })
            .is_some_and(|f| f & 0x02 != 0),
    };
    base_flag || state.more.essential_refs.contains(&who)
}

/// The movement flags someone has now: what the viewer saw, else for the
/// player what the state knows (running, sneaking; the direction only from
/// the viewer), else none.
fn movement_of(state: &GameState, who: FormId) -> u16 {
    if let Some(s) = state.more.seen.get(&who) {
        return s.movement;
    }
    if who == PLAYER_REF {
        let mut f = 0;
        if state.player_running {
            f |= movement::RUNNING;
        }
        if state.player_sneaking {
            f |= movement::SNEAKING;
        }
        return f;
    }
    0
}

/// The Pip-Boy's menus, all of which `MenuMode 1` asks about (`00702680`
/// with 1: stats 1003, inventory 1002, repair 1035, item mods 1061, map
/// 1023).
const PIPBOY_MENUS: [u16; 5] = [1003, 1002, 1035, 1061, 1023];

/// A function that reads, asked about `on`: `Some(answer)` when it's one of
/// these.
pub(crate) fn value(
    facts: &Facts,
    name: &str,
    on: Option<FormId>,
    args: &[Value],
) -> Option<Option<f64>> {
    READS.contains(&name).then(|| read(facts, name, on, args))
}

fn flag(b: bool) -> f64 {
    if b {
        1.0
    } else {
        0.0
    }
}

fn read(facts: &Facts, name: &str, on: Option<FormId>, args: &[Value]) -> Option<f64> {
    let order = facts.order;
    let s = facts.state;
    let arg = |i: usize| args.get(i).cloned().unwrap_or(Value::Number(0.0));
    let actor = |r: FormId| is_actor(order, s, r);
    Some(match name {
        // `005a2a60` → `008ace90`.
        "GetIsGhost" => {
            let who = on?;
            flag(actor(who) && is_ghost(s, who))
        }
        // `005a0260` → `008a5e80`: the process's alert flag.
        "GetIsAlerted" => {
            let who = on?;
            flag(actor(who) && s.more.alerted.contains(&who))
        }
        "IsEssential" => flag(is_essential(order, s, on?)),
        // `005dfc70` → `008c1b30`: the reference's own flag only.
        "IsActorRefEssential" => {
            let who = on?;
            flag(actor(who) && s.more.essential_refs.contains(&who))
        }
        // `005a43b0` → `00973d50`: a talking activator now speaking (its
        // +0x81) whose base's speaker is this person.
        "IsActorTalkingThroughActivator" => {
            let who = on?;
            if !actor(who) {
                return Some(0.0);
            }
            flag(s.speaking.iter().any(|&r| {
                placed::base_now(order, s, r)
                    .filter(|&b| kind_of(order, b) == Some(TACT))
                    .and_then(|b| s.more.speakers.get(&b))
                    == Some(&who)
            }))
        }
        // `005d81b0` → `00835b90`: the station (the reference's base) is
        // broadcasting; one never switched here: its base's form flag
        // 0x40000000, which `SetBroadcastState` writes (`00511370`).
        "GetBroadcastState" => {
            let base = placed::base_now(order, s, on?)?;
            let on = match s.more.broadcasting.get(&base) {
                Some(&b) => b,
                None => order
                    .get(base)
                    .is_some_and(|r| r.entry.header.flags & 0x4000_0000 != 0),
            };
            flag(on)
        }
        // `005be760` / `005a3db0` → `00573090`: the limb's bit in the
        // dismemberment extra data; with a second number every limb from
        // the first to it.
        "IsLimbGone" => {
            let who = on?;
            if !actor(who) {
                return Some(0.0);
            }
            let from = arg(0).number() as i64;
            let to = args.get(1).map_or(-1000, |v| v.number() as i64);
            let gone =
                |l: i64| (0..=255).contains(&l) && s.more.limbs_gone.contains(&(who, l as u8));
            if to < 1 {
                flag(gone(from))
            } else {
                flag((from..=to).any(gone))
            }
        }
        // `005be740` → `005a3d30` → `005730d0`: a person or creature's
        // cause of death (dismembered limbs extra data +0x10); −1 when
        // none was kept or for anything else.
        "GetCauseofDeath" => {
            let who = on?;
            match s.more.cause_of_death.get(&who) {
                Some(&c) if actor(who) => f64::from(c),
                _ => -1.0,
            }
        }
        // `005cb720` → `005a0710`: on a person or creature, 1 when the
        // spine node faces up ([`report_facing_up`]) or isn't there (no
        // 3D, or neither node in it); 0 for anything else.
        "IsFacingUp" => {
            let who = on?;
            flag(actor(who) && s.more.facing_up.get(&who).copied().unwrap_or(true))
        }
        // `005a2910`: the actor's critical stage is this one.
        "IsInCriticalStage" => {
            let who = on?;
            flag(
                actor(who)
                    && s.more.critical_stage.get(&who).copied().unwrap_or(0)
                        == arg(0).number() as i32,
            )
        }
        // `005a2f60` → `004997b0`: sneaking and not the flag that cancels it.
        "IsSneaking" => {
            let who = on?;
            let f = movement_of(s, who);
            flag(actor(who) && f & movement::SNEAKING != 0 && f & movement::NOT_SNEAKING == 0)
        }
        // `005a3010` → `00884730`.
        "IsRunning" => {
            let who = on?;
            flag(actor(who) && movement_of(s, who) & movement::RUNNING != 0)
        }
        // `005a40d0`: 1 forward, 2 back, 3 left, 4 right (the first that's
        // set), else 0.
        "IsMoving" => {
            let who = on?;
            if !actor(who) {
                return Some(0.0);
            }
            let f = movement_of(s, who);
            [
                movement::FORWARD,
                movement::BACK,
                movement::LEFT,
                movement::RIGHT,
            ]
            .iter()
            .position(|m| f & m != 0)
            .map_or(0.0, |i| (i + 1) as f64)
        }
        // `005a4780`: the process's last idle (+0x10c) is this one.
        "IsLastIdlePlayed" => {
            let who = on?;
            let idle = arg(0).form();
            flag(
                actor(who)
                    && idle.0 != 0
                    && s.more.seen.get(&who).and_then(|x| x.last_idle) == Some(idle),
            )
        }
        // `005a1210` ([`procedures`]): what the viewer saw the person's
        // package at; no process (not seen), or not a person: 0.
        "GetCurrentAIProcedure" => {
            let who = on?;
            if !actor(who) {
                return Some(0.0);
            }
            f64::from(s.more.seen.get(&who).and_then(|x| x.procedure).unwrap_or(0))
        }
        // `0059c380`: 0 asks whether any menu is open (`00702360`), 1
        // whether a Pip-Boy menu is, any other number that menu.
        "MenuMode" => {
            let menu = arg(0).number() as i64;
            let open = s.more.menu_open;
            flag(match menu {
                0 => open.is_some(),
                1 => open.is_some_and(|m| PIPBOY_MENUS.contains(&m)),
                n => open.is_some_and(|m| i64::from(m) == n),
            })
        }
        // `005c7800`: the player's +0x75c.
        "GetInCharGen" => flag(s.more.in_chargen),
        // `005dc350` → `004019b0`.
        "Sqrt" => arg(0).number().sqrt(),
        // `005a39d0`: vtable +0x100.
        "IsActor" => flag(actor(on?)),
        // `005a3870` / `005a37b0`: whether this is the Xbox's build
        // (`00709d40`); this is the PC's.
        "IsWin32" => 1.0,
        "IsXBox" => 0.0,
        // `005a59a0`: an actor ignoring crime (vtable +0x320).
        "GetIgnoreCrime" => {
            let who = on?;
            flag(actor(who) && crate::script_functions::ignores_crime(s, who))
        }
        // `005a3760` → `005a3790`: form flag 0x100000.
        "GetIgnoreFriendlyHits" => flag(crate::script_functions::ignores_friendly_hits(
            order, s, on?,
        )),
        // `005a58c0`: below the top level, the experience the next level
        // needs (`00648b50`) minus what the player has; at the top 0.
        "GetXPForNextLevel" => {
            let level = s.player_level;
            if level >= crate::experience::max_level(order) {
                0.0
            } else {
                crate::experience::xp_for_level(order, level + 1) - crate::experience::xp(s)
            }
        }
        // `005ce630` → `0056a9f0`: the enable parent (`XESP`).
        "GetParentRef" => {
            let r = on?;
            let parent = order
                .get(r)
                .and_then(|rr| {
                    let record = rr.record().ok()?;
                    let x = record.get(XESP).filter(|x| x.data.len() >= 4)?;
                    Some(rr.plugin.to_global(FormId(le_u32(&x.data, 0))))
                })
                .map_or(0, |f| f.0);
            f64::from(parent)
        }
        // `005a4410`: the base's form type is this number.
        "GetIsObjectType" => {
            let base = placed::base_now(order, s, on?)?;
            let t = kind_of(order, base).and_then(form_type_number)?;
            flag(f64::from(t) == arg(0).number())
        }
        // `005a4c20`: the player's grabbed reference (+0x638) is this one;
        // nothing can be grabbed here.
        "IsPlayerGrabbedRef" => 0.0,
        // `005a1fa0` → `005a2030`: actor +0x14d.
        "IsSwimming" => {
            let who = on?;
            flag(actor(who) && s.more.seen.get(&who).is_some_and(|x| x.swimming))
        }
        // `005a2810`: the player in the middle of a Sandman kill (vtable
        // +0x36c); there are no Sandman kills here.
        "GetSandman" => 0.0,
        "GetDestructionStage" => f64::from(destruction::stage(order, s, on?)),
        // The condition's version (`005a60f0`): completed or not, as 0 or
        // 2. Scripts get [`change`]'s.
        "GetChallengeCompleted" => {
            let c = arg(0).form();
            if c.0 == 0 {
                return None;
            }
            challenges::completed_for_conditions(s, c)
        }
        // `005a4160`: turning one way 1, the other 2 ([`movement`]).
        "IsTurning" => {
            let who = on?;
            let f = movement_of(s, who);
            if !actor(who) {
                0.0
            } else if f & movement::TURNING_LEFT != 0 {
                1.0
            } else if f & movement::TURNING_RIGHT != 0 {
                2.0
            } else {
                0.0
            }
        }
        // `005a0150`: an actor's damage resistance now (actor value 18).
        "GetArmorRating" => {
            let who = on?;
            if !actor(who) {
                return Some(0.0);
            }
            facts.current_actor_value(who, 18)?
        }
        // `0059e510` → `0047d680`: the caller's rank in the faction minus
        // the other's, each from their base's faction list (the player's
        // own, with what scripts changed, for the player); 0 when either
        // isn't in it.
        "GetFactionRankDifference" => {
            let (faction, other) = (arg(0).form(), arg(1).form());
            let who = on?;
            if !actor(who) || other.0 == 0 || kind_of(order, faction) != Some(FACT) {
                return Some(0.0);
            }
            match (
                base_rank(order, s, who, faction),
                base_rank(order, s, other, faction),
            ) {
                (Some(a), Some(b)) => f64::from(a) - f64::from(b),
                _ => 0.0,
            }
        }
        // `005a0f10`: an actor with animation data playing an idle (what the
        // viewer saw).
        "IsIdlePlaying" => {
            let who = on?;
            flag(actor(who) && s.more.seen.get(&who).is_some_and(|x| x.idle_playing))
        }
        // `005c14a0`: on an object, whether its model's sequence for the
        // group (looked up by the group's name, `00438170` →
        // `0047a520`) is active, or without a group any of them
        // (`00495d00`/`00495d20`); "active" is the sequence's state
        // (+0x44, read by `008041a0`) not 0. Nothing in the executable
        // deactivates a clamped sequence at its end (only `00a35030`, on
        // request, or an ease-out), so a played `Forward` stays active
        // until another sequence replaces it. No 3D loaded: 0. People's
        // animation data (vtable +0x1e4: eight sequence slots) aren't
        // carried out.
        "IsAnimPlaying" => {
            let r = on?;
            // A person's animation data aren't carried out: one who is down
            // isn't animating, one on their feet is (their idle). [G] Not
            // traced, so only with `crate::guesses` on; else the script
            // stops as before.
            if actor(r) {
                return match arg(0) {
                    _ if !crate::guesses::enabled() => None,
                    Value::Text(_) => None,
                    _ => Some(flag(!s.more.down.contains_key(&r) && !s.dead.contains(&r))),
                };
            }
            let playing = s.more.sequences.get(&r);
            flag(match arg(0) {
                Value::Text(group) => {
                    playing.is_some_and(|v| v.iter().any(|n| n.eq_ignore_ascii_case(&group)))
                }
                _ => playing.is_some_and(|v| !v.is_empty()),
            })
        }
        // `005a53e0` → `008bc700`: the caller is fighting and targets this
        // one.
        "IsCombatTarget" => {
            let who = on?;
            flag(actor(who) && s.combat.get(&who) == Some(&arg(0).form()))
        }
        // `005a4dd0`: whether the caller's Karma (actor value 23) is in the
        // band asked (`0047e040`, [`crate::reputation::alignment`]: 0 good,
        // 1 neutral, 2 evil, 3 very good, 4 very evil).
        "GetIsAlignment" => {
            let who = on?;
            let karma = facts.current_actor_value(who, 23).unwrap_or(0.0) as f32;
            flag(
                actor(who)
                    && i64::from(crate::reputation::alignment(order, karma))
                        == arg(0).number() as i64,
            )
        }
        _ => return None,
    })
}

/// Someone's rank in a faction as `0047d680` reads it: the base's faction
/// list (`SNAM`, through a template giving factions), and for the player
/// the player's own list with what scripts changed; `None` when not in it.
fn base_rank(order: &LoadOrder, state: &GameState, who: FormId, faction: FormId) -> Option<i8> {
    if who == PLAYER_REF {
        if let Some(&r) = state.faction_changes.get(&(who, faction)) {
            return (r >= 0).then_some(r);
        }
    }
    let base = placed::base_now(order, state, who)?;
    let (rr, record) = crate::actor::data_record(order, base, crate::actor::USE_FACTIONS)?;
    let rank = record
        .get_all(SNAM)
        .filter(|s| s.data.len() >= 5)
        .find(|s| rr.plugin.to_global(FormId(le_u32(&s.data, 0))) == faction)
        .map(|s| s.data[4] as i8);
    rank.filter(|r| *r >= 0)
}

/// `StopCombatAlarmOnActor` (`005c1b00` → `00972840(actor, 0)`, then the
/// actor's vtable +0x434): whoever is fighting the actor stops fighting it,
/// and the actor stops fighting. For the player, every person nearby
/// (here: in the player's cell or fighting) has their factions' "enemy of
/// the player for crimes" flag cleared (`008bcb40`, faction +0x34 bit
/// 0x10). People sent by an alarm package after the actor stop it
/// (`00881680`): there are no alarm packages here.
fn stop_combat_alarm(runner: &mut Runner, actor: FormId) {
    let order = runner.order;
    let state = &mut *runner.state;
    let fighting: Vec<FormId> = state
        .combat
        .iter()
        .filter(|(_, t)| **t == actor)
        .map(|(a, _)| *a)
        .collect();
    if actor == PLAYER_REF {
        let mut nearby = fighting.clone();
        if let Some(cell) = state.player_cell {
            nearby.extend(
                order
                    .references_in_cell(cell)
                    .into_iter()
                    .filter(|rr| matches!(rr.entry.header.kind.as_bytes(), b"ACHR" | b"ACRE"))
                    .map(|rr| rr.form_id),
            );
        }
        for who in nearby {
            for f in crate::factions::factions_of(order, state, who) {
                state.crime_enemies.remove(&f);
            }
        }
    }
    for who in fighting {
        state.combat.remove(&who);
    }
    state.combat.remove(&actor);
}

/// A function that changes things, on `target`: `Some(value)` when it's
/// one of these (`None` inside when it can't be carried out).
pub(crate) fn change(
    runner: &mut Runner,
    name: &str,
    target: Option<FormId>,
    args: &[Value],
) -> Option<Option<f64>> {
    // Statistics bumped elsewhere count for challenges whenever scripts
    // run.
    if !runner.state.more.challenges.stat_bumps.is_empty() {
        challenges::catch_up(runner);
    }
    if name == "GetChallengeCompleted" {
        // The script's version (`005deef0`): completed or recurred.
        let c = args.first().map(Value::form).filter(|f| f.0 != 0);
        return Some(c.map(|c| flag(challenges::completed_for_scripts(runner.state, c))));
    }
    if name == "GetLineOfSight" {
        // `005c1ce0`: on the caller, with whom it looks for.
        let whom = args.first().map(Value::form).unwrap_or(FormId(0));
        return Some(crate::sight::line_of_sight(runner, target, whom));
    }
    if radio::CHANGES.contains(&name) {
        return Some(radio::carry_out(runner, name, target, args));
    }
    if actors::FUNCTIONS.contains(&name) {
        return Some(actors::carry_out(runner, name, target, args));
    }
    if traps::FUNCTIONS.contains(&name) {
        return Some(traps::carry_out(runner, name, target, args));
    }
    if shaders::CHANGES.contains(&name) {
        return Some(shaders::carry_out(runner, name, target, args));
    }
    CHANGES
        .contains(&name)
        .then(|| carry_out(runner, name, target, args))
}

fn st<'a>(runner: &'a mut Runner<'_>) -> &'a mut State {
    &mut runner.state.more
}

fn carry_out(
    runner: &mut Runner,
    name: &str,
    target: Option<FormId>,
    args: &[Value],
) -> Option<f64> {
    let order = runner.order;
    let arg = |i: usize| args.get(i).cloned().unwrap_or(Value::Number(0.0));
    let number_or = |i: usize, d: f64| args.get(i).map_or(d, Value::number);
    let actor_target = |runner: &Runner| target.filter(|&t| is_actor(order, runner.state, t));
    match name {
        // `005cffb0` → `008acf10(n > 0)`.
        "SetGhost" => {
            if let Some(who) = actor_target(runner) {
                let on = arg(0).number() > 0.0;
                if on {
                    st(runner).ghosts.insert(who);
                } else {
                    st(runner).ghosts.remove(&who);
                }
                runner
                    .state
                    .events
                    .push(Event::More(Shown::Ghost { who, on }));
            }
        }
        // `005d23d0`: on an actor (the player when none), alpha clamped to
        // 0..1 (`00404010`, `0040ebd0`) into the process (+0x5b0); the fade
        // shown is it times the process's other fade (`008c4640`).
        "SetActorAlpha" => {
            let who = target.unwrap_or(PLAYER_REF);
            if is_actor(order, runner.state, who) {
                let alpha = (arg(0).number() as f32).clamp(0.0, 1.0);
                st(runner).alpha.insert(who, alpha);
                runner
                    .state
                    .events
                    .push(Event::More(Shown::ActorAlpha { who, alpha }));
            }
        }
        // `005c9670` → `008a5e40(n > 0)`.
        "SetAlert" => {
            if let Some(who) = actor_target(runner) {
                if arg(0).number() > 0.0 {
                    st(runner).alerted.insert(who);
                } else {
                    st(runner).alerted.remove(&who);
                }
            }
        }
        // `005cd870` → `0047dd50(2, n > 0)`: the base's essential flag.
        "SetEssential" => {
            let base = arg(0).form();
            if base.0 != 0 {
                let on = arg(1).number() > 0.0;
                st(runner).essential_bases.insert(base, on);
            }
        }
        // `005dfd30` → `008c1ac0(n > 0)`: the reference's own flag.
        "SetActorRefEssential" => {
            if let Some(who) = actor_target(runner) {
                if arg(0).number() > 0.0 {
                    st(runner).essential_refs.insert(who);
                } else {
                    st(runner).essential_refs.remove(&who);
                }
            }
        }
        // `005dff70` → `008c1b50(n > 0)`.
        "AlwaysShowActorSubtitles" => {
            if let Some(who) = actor_target(runner) {
                if arg(0).number() > 0.0 {
                    st(runner).always_subtitles.insert(who);
                } else {
                    st(runner).always_subtitles.remove(&who);
                }
            }
        }
        // `005cd650` → `008a8010`: the style goes into the actor's extra
        // data (none removes it), and a fight under way takes it up.
        "SetCombatStyle" => {
            if let Some(who) = actor_target(runner) {
                let style = arg(0).form();
                if style.0 == 0 {
                    st(runner).combat_styles.remove(&who);
                } else {
                    st(runner).combat_styles.insert(who, style);
                }
            }
        }
        // `005d4cc0` → `004ff0e0`: on a talking activator, its base's
        // speaker becomes the person (not an actor: nothing changes).
        "SetTalkingActivatorActor" => {
            let r = target?;
            let base = placed::base_now(order, runner.state, r)?;
            if kind_of(order, base) == Some(TACT) {
                let who = arg(0).form();
                if who.0 == 0 {
                    st(runner).speakers.remove(&base);
                } else if is_actor(order, runner.state, who) {
                    st(runner).speakers.insert(base, who);
                }
            }
        }
        // `005d8220` → `008359e0(r, n > 0)`: the station's base switched
        // on or off (form flag 0x40000000); switching off stops its
        // broadcast (no radio here yet).
        "SetBroadcastState" => {
            let base = placed::base_now(order, runner.state, target?)?;
            let on = arg(0).number() > 0.0;
            st(runner).broadcasting.insert(base, on);
        }
        // `005dc010` → `008a1a40`: the actor's critical stage, and what it
        // does (`008a1a70`, [`set_critical_stage`]).
        "SetCriticalStage" => {
            if let Some(who) = actor_target(runner) {
                set_critical_stage(runner.state, who, arg(0).number() as i32);
            }
        }
        // `005db870`: an ash pile (or, given 2, a goo pile) under an
        // actor, standing for its corpse ([`placed::attach_ash_pile`]).
        "AttachAshPile" => {
            let who = actor_target(runner)?;
            placed::attach_ash_pile(runner, who, arg(0).number() as i32)?;
        }
        // `005ce910` → `005ce9d0`: actor +0x125 (whether it makes them
        // sneak isn't traced: not applied to their movement).
        "SetForceSneak" => {
            if let Some(who) = actor_target(runner) {
                if arg(0).number() != 0.0 {
                    st(runner).forced_sneak.insert(who);
                } else {
                    st(runner).forced_sneak.remove(&who);
                }
            }
            return Some(1.0);
        }
        // `005d6a60`: actor +0xbc (AI processing on).
        "SetActorsAI" => {
            if let Some(who) = actor_target(runner) {
                if arg(0).number() != 0.0 {
                    st(runner).ai_off.remove(&who);
                } else {
                    st(runner).ai_off.insert(who);
                }
            }
        }
        // `005c7790` → `00950010`.
        "SetInChargen" => st(runner).in_chargen = arg(0).number() != 0.0,
        // `005bc200`, `005d49c0`: the global at `012682f8`.
        "SetNoAvoidance" | "SetSceneIsComplex" => {
            st(runner).no_avoidance = arg(0).number() != 0.0;
        }
        // `005bc1a0`: the global at `011df678`.
        "SetAllVisible" => st(runner).all_visible = arg(0).number() != 0.0,
        // `005bc150`, `005d4a40`: read their arguments and do nothing.
        "SetAllReachable" | "TrapUpdate" => {}
        // `005d22d0`: loads the effect's model ahead of use; nothing in the
        // game changes.
        "PreloadMagicEffect" => {}
        // `005c5560`, `005e0160`, `005c5590`: with `bAllowScriptedAutosave`
        // (or `…ForceSave`) in `[SaveGame]` (1 in the exe), the save
        // manager is asked for an autosave, a system save or a forced save.
        // The INI isn't read by the world: the exe's 1 is taken.
        "Autosave" | "ForceSave" | "SystemSave" => {
            let kind = match name {
                "Autosave" => SaveKind::Autosave,
                "ForceSave" => SaveKind::Force,
                _ => SaveKind::System,
            };
            runner.state.events.push(Event::More(Shown::Save(kind)));
        }
        // `005d5560` → `00aa4db0(m, 1)`: the time manager's multiplier.
        "SetGlobalTimeMultiplier" => {
            let mult = number_or(0, 1.0) as f32;
            st(runner).time_multiplier = Some(mult);
            runner
                .state
                .events
                .push(Event::More(Shown::TimeMultiplier(mult)));
        }
        // `005d6d10`: the reference's ownership extra data goes.
        "ClearOwnership" => {
            let r = target?;
            runner.state.set_by_scripts.owners.remove(&r);
        }
        // `005cd5b0`: while the player sleeps (+0x658), the hours left
        // become n (at least 0) and sleeping stays on; 0 also closes the
        // sleep menu (`007055c0`), which ends when no hours are left.
        "WakeUpPC" => {
            if runner.state.living.sleeping {
                runner.state.living.hours_left = (arg(0).number() as i32).max(0);
            }
        }
        "IncrementScriptedChallenge" => {
            let c = arg(0).form();
            if c.0 == 0 {
                return None;
            }
            challenges::increment_scripted(runner, c, 1);
        }
        "RemoveRecurringFromChallenge" => {
            let c = arg(0).form();
            if c.0 == 0 {
                return None;
            }
            challenges::stop_recurring(runner.state, c);
        }
        // `005c4a70`: the last reference made is the value.
        "PlaceAtMe" => {
            let caller = target?;
            let base = arg(0).form();
            if base.0 == 0 {
                return None;
            }
            let made = placed::place_at_me(
                runner,
                caller,
                base,
                number_or(1, 1.0) as i32,
                number_or(2, 0.0) as f32,
                number_or(3, 0.0) as i32,
            );
            return Some(f64::from(made.map_or(0, |r| r.0)));
        }
        "PlaceLeveledActorAtMe" => {
            let caller = target?;
            let made = placed::place_leveled_actor(runner, caller, arg(0).form());
            return Some(f64::from(made.map_or(0, |r| r.0)));
        }
        // `005d4b90`: placed objects only (`destruction::damage`).
        "DamageObject" => {
            destruction::damage(runner, target?, arg(0).number() as f32)?;
        }
        "ClearDestruction" => destruction::clear(runner, target?),
        // `005c43a0`: the running effect's seconds (its +4); none running:
        // 0.
        "ScriptEffectElapsedSeconds" => {
            return Some(f64::from(st(runner).effect_seconds.unwrap_or(0.0)));
        }
        "StopCombatAlarmOnActor" => {
            if let Some(who) = actor_target(runner) {
                stop_combat_alarm(runner, who);
            }
        }
        // `005cf680`: a Securitron with its 3D loaded gets its face extra
        // data (0x8F) with the face's kind and expression, and shows them
        // (`00437f90`; the face's textures aren't swapped here).
        "SetSecuritronExpression" => {
            let r = arg(0).form();
            let (Value::Text(kind), Value::Text(expression)) = (arg(1), arg(2)) else {
                return None;
            };
            if r.0 != 0 {
                st(runner).securitron_faces.insert(r, (kind, expression));
            }
        }
        // `005d72f0`, `005d7290`, `005d73d0` → `007062a0(kind, n, add)`.
        "AddSPECIALPoints" => st(runner).special_points += arg(0).number() as i32,
        "SetSPECIALPoints" => st(runner).special_points = arg(0).number() as i32,
        "AddTagSkills" => st(runner).tag_points += arg(0).number() as i32,
        // `005db110` → `00a255b0`: the first Xbox controller's motors run
        // for a while (`XInputSetState`) when one is connected; nothing in
        // the game changes, and nv-rs drives no rumble.
        "SetRumble" => {}
        _ => return None,
    }
    Some(0.0)
}

/// Saved lines.
pub(crate) fn save_lines(state: &GameState, line: &mut dyn FnMut(String)) {
    let id = |f: FormId| format!("{:08X}", f.0);
    let m = &state.more;
    let sorted = |set: &HashSet<FormId>| {
        let mut v: Vec<FormId> = set.iter().copied().collect();
        v.sort();
        v
    };
    for (word, set) in [
        ("ghost", &m.ghosts),
        ("alerted", &m.alerted),
        ("essentialref", &m.essential_refs),
        ("subtitles", &m.always_subtitles),
        ("forcesneak", &m.forced_sneak),
        ("aioff", &m.ai_off),
    ] {
        for f in sorted(set) {
            line(format!("{word} {}", id(f)));
        }
    }
    let mut v: Vec<_> = m.alpha.iter().collect();
    v.sort_by_key(|(k, _)| **k);
    for (who, a) in v {
        line(format!("alpha {} {a}", id(*who)));
    }
    let mut v: Vec<_> = m.essential_bases.iter().collect();
    v.sort_by_key(|(k, _)| **k);
    for (base, on) in v {
        line(format!("essentialbase {} {}", id(*base), u8::from(*on)));
    }
    let mut v: Vec<_> = m.broadcasting.iter().collect();
    v.sort_by_key(|(k, _)| **k);
    for (base, on) in v {
        line(format!("broadcast {} {}", id(*base), u8::from(*on)));
    }
    let mut v: Vec<_> = m.item_values.iter().collect();
    v.sort_by_key(|(k, _)| **k);
    for (base, value) in v {
        line(format!("itemvalue {} {value}", id(*base)));
    }
    for (word, map) in [("combatstyle", &m.combat_styles), ("speaker", &m.speakers)] {
        let mut v: Vec<_> = map.iter().collect();
        v.sort_by_key(|(k, _)| **k);
        for (a, b) in v {
            line(format!("{word} {} {}", id(*a), id(*b)));
        }
    }
    let mut v: Vec<_> = m.critical_stage.iter().collect();
    v.sort_by_key(|(k, _)| **k);
    for (who, s) in v {
        line(format!("criticalstage {} {s}", id(*who)));
    }
    let mut v: Vec<_> = m.ash_piles.iter().collect();
    v.sort_by_key(|(k, _)| **k);
    for (pile, corpse) in v {
        line(format!("ashpile {} {}", id(*pile), id(*corpse)));
    }
    let mut v: Vec<_> = m.limbs_gone.iter().collect();
    v.sort();
    for (who, limb) in v {
        line(format!("limbgone {} {limb}", id(*who)));
    }
    if m.in_chargen {
        line("inchargen".to_string());
    }
    if let Some(t) = m.time_multiplier {
        line(format!("timemult {t}"));
    }
    if m.no_avoidance {
        line("noavoidance".to_string());
    }
    if m.all_visible {
        line("allvisible".to_string());
    }
    for (c, (count, flags)) in &m.challenges.progress {
        line(format!("chal {} {count} {flags}", id(*c)));
    }
    let mut v: Vec<_> = m.securitron_faces.iter().collect();
    v.sort_by_key(|(k, _)| **k);
    for (r, (kind, expression)) in v {
        line(format!("secface {} {kind} {expression}", id(*r)));
    }
    if m.special_points != 0 {
        line(format!("specialpoints {}", m.special_points));
    }
    if m.tag_points != 0 {
        line(format!("tagpoints {}", m.tag_points));
    }
    placed::save_lines(state, line);
    destruction::save_lines(state, line);
    radio::save_lines(state, line);
    actors::save_lines(state, line);
    traps::save_lines(state, line);
    crate::mines::save_lines(state, line);
}

/// A saved line back: `None` if the word isn't one of these.
pub(crate) fn load_line(state: &mut GameState, raw: &str) -> Option<Result<(), String>> {
    let parts: Vec<&str> = raw.split_whitespace().collect();
    let word = *parts.first()?;
    if let Some(r) = placed::load_line(state, &parts)
        .or_else(|| destruction::load_line(state, &parts))
        .or_else(|| radio::load_line(state, &parts))
        .or_else(|| actors::load_line(state, &parts))
        .or_else(|| traps::load_line(state, &parts))
        .or_else(|| crate::mines::load_line(state, &parts))
    {
        return Some(r);
    }
    let bad = || format!("can't read '{raw}'");
    let form = |i: usize| -> Result<FormId, String> {
        parts
            .get(i)
            .and_then(|s| u32::from_str_radix(s, 16).ok())
            .map(FormId)
            .ok_or_else(bad)
    };
    let num = |i: usize| -> Result<f64, String> {
        parts
            .get(i)
            .and_then(|s| s.parse::<f64>().ok())
            .ok_or_else(bad)
    };
    let m = &mut state.more;
    let result = (|| -> Result<bool, String> {
        match word {
            "ghost" => {
                m.ghosts.insert(form(1)?);
            }
            "alerted" => {
                m.alerted.insert(form(1)?);
            }
            "essentialref" => {
                m.essential_refs.insert(form(1)?);
            }
            "subtitles" => {
                m.always_subtitles.insert(form(1)?);
            }
            "forcesneak" => {
                m.forced_sneak.insert(form(1)?);
            }
            "aioff" => {
                m.ai_off.insert(form(1)?);
            }
            "alpha" => {
                m.alpha.insert(form(1)?, num(2)? as f32);
            }
            "essentialbase" => {
                m.essential_bases.insert(form(1)?, num(2)? != 0.0);
            }
            "broadcast" => {
                m.broadcasting.insert(form(1)?, num(2)? != 0.0);
            }
            "combatstyle" => {
                m.combat_styles.insert(form(1)?, form(2)?);
            }
            "itemvalue" => {
                m.item_values.insert(form(1)?, num(2)? as i32);
            }
            "speaker" => {
                m.speakers.insert(form(1)?, form(2)?);
            }
            "criticalstage" => {
                m.critical_stage.insert(form(1)?, num(2)? as i32);
            }
            "ashpile" => {
                m.ash_piles.insert(form(1)?, form(2)?);
            }
            "limbgone" => {
                m.limbs_gone.insert((form(1)?, num(2)? as u8));
            }
            "inchargen" => m.in_chargen = true,
            "timemult" => m.time_multiplier = Some(num(1)? as f32),
            "noavoidance" => m.no_avoidance = true,
            "allvisible" => m.all_visible = true,
            "secface" => {
                let text = |i: usize| parts.get(i).map(|s| s.to_string()).ok_or_else(bad);
                m.securitron_faces.insert(form(1)?, (text(2)?, text(3)?));
            }
            "specialpoints" => m.special_points = num(1)? as i32,
            "tagpoints" => m.tag_points = num(1)? as i32,
            "chal" => {
                m.challenges
                    .progress
                    .insert(form(1)?, (num(2)? as i32, num(3)? as u32));
            }
            _ => return Ok(false),
        }
        Ok(true)
    })();
    match result {
        Ok(true) => Some(Ok(())),
        Ok(false) => None,
        Err(e) => Some(Err(e)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn functions_go_by_the_games_own_names_and_once() {
        for name in handled() {
            assert!(
                script::functions::FUNCTIONS.iter().any(|f| f.name == name),
                "{name} isn't a function's name in the game's table"
            );
            assert!(
                !crate::scripting::HANDLED.contains(&name),
                "{name} is carried out twice"
            );
            assert!(
                !crate::script_functions::READS.contains(&name)
                    && !crate::script_functions::CHANGES.contains(&name)
                    && !crate::living::HANDLED.contains(&name),
                "{name} is also carried out elsewhere"
            );
        }
        assert!(READS.iter().all(|r| !CHANGES.contains(r)));
    }

    #[test]
    fn form_type_numbers_are_the_exes() {
        assert_eq!(form_type_number(FourCC::new(b"WEAP")), Some(0x28));
        assert_eq!(form_type_number(FourCC::new(b"NPC_")), Some(0x2A));
        assert_eq!(form_type_number(FourCC::new(b"SLPD")), Some(120));
        assert_eq!(form_type_number(FourCC::new(b"ZZZZ")), None);
    }
}

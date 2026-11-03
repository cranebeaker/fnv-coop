//! More of the game's script functions, each carried out the way its own
//! handler in `FalloutNV.exe` does it. The function table starts at
//! `01190910`: 40-byte entries (name, short name, opcode, help, then the
//! "needs a reference" and parameter-count words at +0x10, the parameters
//! at +0x14, the script handler at +0x18 and the condition version at
//! +0x20). Every rule below names the handler it was read from; the full
//! notes are in `%USERPROFILE%\nv-re\findings\functions.md`.
//!
//! [`value`] answers the ones that read (for scripts and conditions, through
//! [`crate::scripting::Facts`]); [`change`] carries out the ones that change
//! things (through [`crate::scripting::Runner`]). What they keep is in
//! [`SetByScripts`] (`GameState::set_by_scripts`), saved with the game.

use std::collections::{BTreeSet, HashMap, HashSet};

use esm::{FormId, FourCC, LoadOrder};

use crate::cell::{le_f32, le_u32};
use crate::dialogue::{PLAYER_BASE, PLAYER_REF};
use crate::scripting::{
    base_of, game_setting, whereabouts, Event, Facts, GameState, Runner, Value,
};

const ACHR: FourCC = FourCC::new(b"ACHR");
const ACRE: FourCC = FourCC::new(b"ACRE");
const NPC_: FourCC = FourCC::new(b"NPC_");
const CREA: FourCC = FourCC::new(b"CREA");
const LVLC: FourCC = FourCC::new(b"LVLC");
const WEAP: FourCC = FourCC::new(b"WEAP");
const RACE: FourCC = FourCC::new(b"RACE");
const FLST: FourCC = FourCC::new(b"FLST");
const LNAM: FourCC = FourCC::new(b"LNAM");
const LVLI: FourCC = FourCC::new(b"LVLI");
const IDLE: FourCC = FourCC::new(b"IDLE");
const CELL: FourCC = FourCC::new(b"CELL");
const MESG: FourCC = FourCC::new(b"MESG");
const QUST: FourCC = FourCC::new(b"QUST");
const XOWN: FourCC = FourCC::new(b"XOWN");
const XSCL: FourCC = FourCC::new(b"XSCL");
const RNAM: FourCC = FourCC::new(b"RNAM");
const CNAM: FourCC = FourCC::new(b"CNAM");
const ACBS: FourCC = FourCC::new(b"ACBS");
const SNAM: FourCC = FourCC::new(b"SNAM");
const NAM1: FourCC = FourCC::new(b"NAM1");
const MNAM: FourCC = FourCC::new(b"MNAM");
const FNAM: FourCC = FourCC::new(b"FNAM");
const INDX: FourCC = FourCC::new(b"INDX");
const MODL: FourCC = FourCC::new(b"MODL");
const HNAM: FourCC = FourCC::new(b"HNAM");
const ENAM: FourCC = FourCC::new(b"ENAM");

/// The fists, a weapon record every game has (`0046a370` looks up form
/// `000001F4` for it): what `IsWeaponInList` asks about someone with no
/// weapon in hand.
pub const FISTS: FormId = FormId(0x1F4);

/// What scripts set through these functions, kept in the game state and
/// saved.
#[derive(Debug, Clone, Default)]
pub struct SetByScripts {
    /// People held in place (`SetRestrained 1`: the actor's life state 5,
    /// `008ace50`).
    pub restrained: HashSet<FormId>,
    /// People who ignore crime (`IgnoreCrime 1`: actor+0x144, `0087d630`).
    pub ignoring_crime: HashSet<FormId>,
    /// References' "ignore friendly hits" flag (form flag 0x100000) as
    /// `SetIgnoreFriendlyHits` left it (`005d3d50`).
    pub ignore_friendly_hits: HashMap<FormId, bool>,
    /// Doors' "open" flag (action flag 4) where it has changed from how
    /// they were placed: opened (true) or shut by activation, scripts'
    /// `SetOpenState` or a lock (`world::doors`).
    pub open: HashMap<FormId, bool>,
    /// References' owners as `SetOwnership` set them (`005b57e0`).
    pub owners: HashMap<FormId, FormId>,
    /// Cells' owners (`SetCellOwnership`) and public flags
    /// (`SetCellPublicFlag`).
    pub cell_owners: HashMap<FormId, FormId>,
    pub public_cells: HashMap<FormId, bool>,
    /// Achievements unlocked (`AddAchievement`, the platform's numbers).
    pub achievements: BTreeSet<u32>,
    /// Challenges unlocked (`UnlockChallenge`: the challenge's flag 0x01).
    pub challenges: HashSet<FormId>,
    /// Forms added to form lists (`AddFormToFormList`), by list.
    pub list_additions: HashMap<FormId, Vec<FormId>>,
    /// `EnableFastTravel`'s three flags.
    pub fast_travel: FastTravel,
    /// Items whose quest-item flag (form flag 0x400) a script changed
    /// (`SetQuestObject`).
    pub quest_items: HashMap<FormId, bool>,
    /// Base records' names as `SetActorFullName` set them.
    pub names: HashMap<FormId, String>,
    /// Whom each person looks at (`Look`; `StopLook` ends it).
    pub looking: HashMap<FormId, FormId>,
    /// References' x and y rotations (radians) as `SetAngle` set them;
    /// the z one (the heading) is kept with their position.
    pub tilts: HashMap<FormId, [f32; 2]>,
    /// Objectives completed while not shown (state 2 in the game; shown
    /// ones are in `GameState::objectives`).
    pub hidden_completed: BTreeSet<(FormId, i32)>,
}

/// `EnableFastTravel enable [allow waiting] [keep]` (`005d13e0`): the
/// player's flags at +0x66d (bit 0 fast travel allowed, bit 1 keep it as
/// it is when the player is moved) and +0x66e (waiting allowed).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FastTravel {
    pub enabled: bool,
    pub keep: bool,
    pub can_wait: bool,
}

impl Default for FastTravel {
    /// A new player (`00958fc0`, `00938180`): fast travel and waiting
    /// allowed.
    fn default() -> FastTravel {
        FastTravel {
            enabled: true,
            keep: false,
            can_wait: true,
        }
    }
}

/// The functions answered here ([`value`]), by the game's own names.
pub const READS: &[&str] = &[
    "GetGameSetting",
    "IsChild",
    "GetRestrained",
    "GetOpenState",
    "IsOwner",
    "GetFriendHit",
    "GetFactionRank",
    "GetPos",
    "GetAngle",
    "GetStartingPos",
    "GetStartingAngle",
    "GetIsCreature",
    "GetIsCreatureType",
    "GetIsClass",
    "GetWeaponAnimType",
    "IsWeaponSkillType",
    "IsWeaponInList",
    "IsInList",
    "IsInInterior",
    "IsImageSpaceActive",
    "IsKiller",
    "IsPlayerTagSkill",
    "GetInCellParam",
    "GetHeadingAngle",
    "IsPS3",
    "GetScale",
    "GetActorFactionPlayerEnemy",
    "Exists",
    "GetCasinoWinningStage",
    "PlayerInRegion",
    "IsTalking",
    "GetCurrentAIPackage",
    "IsPC1stPerson",
];

/// `GetCurrentAIPackage`'s numbers by the package's type (`PKDT` byte 4,
/// kept as is at package+0x20 by `00670fc0`, 17 read as 0), from the
/// handler's switch (`005a0b60`): the record types 0–16 (find, follow,
/// escort, eat, sleep, wander 13, travel 14, accompany 15, use item at 16,
/// ambush 17, flee 18, cast magic 19, sandbox 36, patrol 37, guard 35,
/// dialogue 34, use weapon 33); the game's own packages from 18 on.
const PACKAGE_NUMBERS: [i8; 32] = [
    0, 1, 2, 3, 4, 13, 14, 15, 16, 17, 18, 19, 36, 37, 35, 34, 33, -1, 5, 20, 7, 8, 10, 11, 9, 12,
    21, 24, 6, 28, 29, 32,
];

/// The functions carried out here ([`change`]), by the game's own names.
pub const CHANGES: &[&str] = &[
    "SetRestrained",
    "IgnoreCrime",
    "SetOpenState",
    "SetOwnership",
    "SetCellOwnership",
    "SetCellPublicFlag",
    "AddItemHealthPercent",
    "AddAchievement",
    "SetIgnoreFriendlyHits",
    "EnableFastTravel",
    "RemoveNote",
    "SetPos",
    "SetAngle",
    "AddFormToFormList",
    "CompleteAllObjectives",
    "UnlockChallenge",
    "ResetHealth",
    "PurgeCellBuffers",
    "RemoveFromAllFactions",
    "SetActorFullName",
    "ResetInventory",
    "ResetQuest",
    "SetQuestObject",
    "RemoveAllTypedItems",
    "Say",
    "PlayGroup",
    "PlayIdle",
    "SwapTextureOnRef",
    "Look",
    "ShowWarning",
    "SendAssaultAlarm",
    "EnterTrigger",
];

thread_local! {
    /// How deep `EnterTrigger` runs are inside each other: the game runs
    /// the trigger's script only below 5 (a per-thread counter, `005d8cf0`).
    static TRIGGER_DEPTH: std::cell::Cell<u8> = const { std::cell::Cell::new(0) };
}

/// Every function carried out here.
pub fn handled() -> impl Iterator<Item = &'static str> {
    READS
        .iter()
        .chain(CHANGES)
        .chain(crate::living::HANDLED)
        .copied()
        .chain(crate::more_functions::handled())
}

/// Game settings the game's own scripts ask for (`GetGameSetting`) that
/// `FalloutNV.esm` doesn't define, with the values the exe gives them
/// (`%USERPROFILE%\nv-re\decomp\combat\all_settings.txt`; the setting
/// objects at the addresses given).
const EXE_SETTINGS: &[(&str, f32)] = &[
    ("iFriendHitCombatAllowed", 3.0),    // 011cd204 (the data has 4)
    ("iFriendHitNonCombatAllowed", 0.0), // 011cd5b0
    ("iLockLevelMaxVeryEasy", 0.0),      // 011c3a4c
    ("iLockLevelMaxEasy", 25.0),         // 011c3a30
    ("iLockLevelMaxAverage", 50.0),      // 011c3a00
    ("iLockLevelMaxHard", 75.0),         // 011c3ab0
    ("iLockLevelMaxVeryHard", 100.0),    // 011c399c
    ("iLockLevelMaxImpossible", 255.0),  // 011c3a24
];

/// `GetGameSetting name` (`005be900`): an integer (`i…`) or float (`f…`)
/// setting's value; text and other settings, and names that aren't
/// settings, give 0. The value is the data's (`GMST`), else the exe's own
/// (only those in [`EXE_SETTINGS`] are known here: others are `None`).
pub fn game_setting_value(order: &LoadOrder, name: &str) -> Option<f64> {
    if let Some(v) = game_setting(order, name) {
        return Some(f64::from(v));
    }
    if !matches!(name.as_bytes().first(), Some(b'i' | b'I' | b'f' | b'F')) {
        // Defined (text) or not: the game leaves 0.
        return order
            .form_by_editor_id(name)
            .is_some_and(|id| {
                order
                    .get(id)
                    .is_some_and(|r| r.entry.header.kind.as_bytes() == b"GMST")
            })
            .then_some(0.0);
    }
    EXE_SETTINGS
        .iter()
        .find(|(n, _)| n.eq_ignore_ascii_case(name))
        .map(|(_, v)| f64::from(*v))
}

pub(crate) fn kind_of(order: &LoadOrder, id: FormId) -> Option<FourCC> {
    order.get(id).map(|r| r.entry.header.kind)
}

/// Whether a reference is a person or creature (the game's actor test,
/// vtable +0x100: true for characters and creatures, false for other
/// references).
pub fn is_actor(order: &LoadOrder, r: FormId) -> bool {
    r == PLAYER_REF || matches!(kind_of(order, r), Some(k) if k == ACHR || k == ACRE)
}

fn form_at(rr: &esm::RecordRef<'_>, data: &[u8], at: usize) -> Option<FormId> {
    (data.len() >= at + 4)
        .then(|| rr.plugin.to_global(FormId(le_u32(data, at))))
        .filter(|f| f.0 != 0)
}

fn sub_form(order: &LoadOrder, id: FormId, kind: FourCC) -> Option<FormId> {
    let rr = order.get(id)?;
    let record = rr.record().ok()?;
    let s = record.get(kind)?;
    form_at(&rr, &s.data, 0)
}

/// A form list's forms now: its `LNAM`s, then what scripts added
/// (`AddFormToFormList`, `00590010`: a form already in it isn't added
/// again).
pub fn form_list(order: &LoadOrder, state: &GameState, list: FormId) -> Vec<FormId> {
    let mut out = Vec::new();
    if let Some(rr) = order.get(list).filter(|r| r.entry.header.kind == FLST) {
        if let Ok(record) = rr.record() {
            out.extend(
                record
                    .get_all(LNAM)
                    .filter_map(|s| form_at(&rr, &s.data, 0)),
            );
        }
    }
    if let Some(added) = state.set_by_scripts.list_additions.get(&list) {
        for f in added {
            if !out.contains(f) {
                out.push(*f);
            }
        }
    }
    out
}

/// Whether an item is a quest item (form flag 0x400, the record's header
/// flag; `SetQuestObject` changes it, vtable +0xcc).
pub fn is_quest_item(order: &LoadOrder, state: &GameState, item: FormId) -> bool {
    match state.set_by_scripts.quest_items.get(&item) {
        Some(&q) => q,
        None => order
            .get(item)
            .is_some_and(|r| r.entry.header.flags & 0x400 != 0),
    }
}

/// A reference's own owner (its ownership extra data, 0x21): as
/// `SetOwnership` set it, else its `XOWN`.
pub fn own_owner(order: &LoadOrder, state: &GameState, reference: FormId) -> Option<FormId> {
    if let Some(&o) = state.set_by_scripts.owners.get(&reference) {
        return Some(o);
    }
    sub_form(order, reference, XOWN)
}

/// A cell's owner now: as `SetCellOwnership` set it, else its record's
/// (`world::crime::cell_owner`).
pub fn cell_owner_now(order: &LoadOrder, state: &GameState, cell: FormId) -> Option<FormId> {
    match state.set_by_scripts.cell_owners.get(&cell) {
        Some(&o) => Some(o),
        None => crate::crime::cell_owner(order, cell),
    }
}

/// Whether a cell counts as public for trespassing: its flag 0x20 as
/// `SetCellPublicFlag` set it (`00544340` sets or clears the cell's
/// in-memory flag 0x20), else its `DATA`'s; or its `DATA` flag 0x40.
pub fn cell_is_public(order: &LoadOrder, state: &GameState, cell: FormId) -> bool {
    let flags = order
        .get(cell)
        .and_then(|rr| rr.record().ok())
        .and_then(|r| r.get(esm::sig::DATA).and_then(|s| s.data.first().copied()))
        .unwrap_or(0);
    let public = state
        .set_by_scripts
        .public_cells
        .get(&cell)
        .copied()
        .unwrap_or(flags & 0x20 != 0);
    public || flags & 0x40 != 0
}

/// Whether a person ignores crime (`IgnoreCrime`; read back by vtable
/// +0x320, `0087d650`).
pub fn ignores_crime(state: &GameState, who: FormId) -> bool {
    state.set_by_scripts.ignoring_crime.contains(&who)
}

/// Whether a reference ignores friendly hits (form flag 0x100000,
/// `005a3790`): as `SetIgnoreFriendlyHits` left it, else the record's
/// header flag.
pub fn ignores_friendly_hits(order: &LoadOrder, state: &GameState, who: FormId) -> bool {
    match state.set_by_scripts.ignore_friendly_hits.get(&who) {
        Some(&f) => f,
        None => order
            .get(who)
            .is_some_and(|r| r.entry.header.flags & 0x0010_0000 != 0),
    }
}

/// The player has been moved (a door, `MoveTo`, fast travel: the
/// "position player" request, `0093bea0`): fast travel a script turned
/// off is allowed again, unless the script asked to keep it off (bit 1)
/// (or the player's flag 0x02 at +0x244 is set, not modelled).
pub fn player_moved(state: &mut GameState) {
    // Its last step brings followers and teammates along (`0093c200` →
    // `00973de0`, `world::companions::come_along`).
    state.player_placed = true;
    let ft = &mut state.set_by_scripts.fast_travel;
    if !ft.keep {
        ft.enabled = true;
    }
}

/// Whether a cell's editor ID begins with another cell's, ignoring case:
/// `GetInCell` and `GetInCellParam` (`0059ef60`) compare as many letters
/// as the asked-for cell's editor ID has (`strnicmp`), so `GetInCell
/// GSDocMitchellHouse` also holds in `GSDocMitchellHouseBasement`; an
/// asked-for cell without an editor ID matches any.
pub fn cell_matches(order: &LoadOrder, cell: FormId, wanted: FormId) -> bool {
    if order.get(wanted).map(|r| r.entry.header.kind) != Some(CELL) {
        return false;
    }
    let edid = |id: FormId| -> String {
        order
            .get(id)
            .and_then(|r| r.editor_id().ok().flatten())
            .unwrap_or_default()
            .to_ascii_lowercase()
    };
    edid(cell).starts_with(&edid(wanted))
}

/// The cell a reference is in now (the player's from the state).
fn cell_of(order: &LoadOrder, state: &GameState, who: FormId) -> Option<FormId> {
    if who == PLAYER_REF {
        return state.player_cell;
    }
    state.place(order, who).map(|p| p.1)
}

/// `IsChild` (`008d40e0` on people, called with 0; creatures and other
/// references: never): the person's race (`RNAM`, from the template when
/// it gives the traits) has `DATA` flag 0x04 (at byte 32; the child races
/// have it), or one of the race's three body models for the person's sex
/// (`NAM1` part 0–2) has "child" in its path. The game then also counts a
/// loaded person whose model stands under 110 units (height ÷ scale): not
/// here (no models in the world crate).
pub fn is_child(order: &LoadOrder, state: &GameState, who: FormId) -> bool {
    let Some(base) = base_of(order, who) else {
        return false;
    };
    if kind_of(order, base) != Some(NPC_) {
        return false;
    }
    let Some((rr, record)) = crate::actor::data_record(order, base, crate::actor::USE_TRAITS)
    else {
        return false;
    };
    let female = if who == PLAYER_REF {
        state.player_female.unwrap_or(false)
    } else {
        record
            .get(ACBS)
            .filter(|s| s.data.len() >= 4)
            .is_some_and(|s| le_u32(&s.data, 0) & 1 != 0)
    };
    let Some(race) = record.get(RNAM).and_then(|s| form_at(&rr, &s.data, 0)) else {
        return false;
    };
    let Some(race_rr) = order.get(race).filter(|r| r.entry.header.kind == RACE) else {
        return false;
    };
    let Ok(race_record) = race_rr.record() else {
        return false;
    };
    let flags = race_record
        .get(esm::sig::DATA)
        .filter(|s| s.data.len() >= 36)
        .map_or(0, |s| le_u32(&s.data, 32));
    if flags & 0x04 != 0 {
        return true;
    }
    // The body parts: after NAM1, MNAM (male) or FNAM (female), INDX 0..2
    // with their MODL.
    let mut in_body = false;
    let mut sex = 0usize;
    let mut index: Option<u32> = None;
    for sub in &race_record.subrecords {
        match sub.kind {
            k if k == NAM1 => in_body = true,
            k if k == HNAM || k == ENAM => in_body = false,
            k if k == MNAM => sex = 0,
            k if k == FNAM => sex = 1,
            k if k == INDX && sub.data.len() >= 4 => index = Some(le_u32(&sub.data, 0)),
            k if k == MODL
                && in_body
                && sex == usize::from(female)
                && index.is_some_and(|i| i <= 2)
                && sub.zstring().to_ascii_lowercase().contains("child") =>
            {
                return true;
            }
            _ => {}
        }
    }
    false
}

/// The factions a person is in as the game lists them for `GetFactionRank`
/// (`008b8ca0`): the base's `SNAM`s with a rank of 0 or more, then the
/// changes scripts made (a rank of -1 takes one out, 0 or more adds it).
fn faction_list(order: &LoadOrder, state: &GameState, who: FormId) -> Vec<FormId> {
    let mut out = Vec::new();
    if who == PLAYER_REF {
        out.push(crate::factions::PLAYER_FACTION);
    }
    if let Some((rr, record)) = base_of(order, who)
        .and_then(|b| crate::actor::data_record(order, b, crate::actor::USE_FACTIONS))
    {
        for s in record.get_all(SNAM).filter(|s| s.data.len() >= 5) {
            if (s.data[4] as i8) >= 0 {
                if let Some(f) = form_at(&rr, &s.data, 0) {
                    out.push(f);
                }
            }
        }
    }
    let mut changes: Vec<(FormId, i8)> = state
        .faction_changes
        .iter()
        .filter(|((w, _), _)| *w == who)
        .map(|((_, f), r)| (*f, *r))
        .collect();
    changes.sort();
    for (faction, rank) in changes {
        out.retain(|f| *f != faction);
        if rank >= 0 {
            out.push(faction);
        }
    }
    out
}

/// Where a reference was placed (its record's `DATA`): position and
/// rotation (radians).
fn placed(order: &LoadOrder, r: FormId) -> Option<([f32; 3], [f32; 3])> {
    let rr = order.get(r)?;
    let record = rr.record().ok()?;
    let d = record.get(esm::sig::DATA).filter(|s| s.data.len() >= 24)?;
    let f = |i: usize| le_f32(&d.data, i * 4);
    Some(([f(0), f(1), f(2)], [f(3), f(4), f(5)]))
}

/// An axis argument (`X`, `Y`, `Z`) as 0, 1, 2.
fn axis(v: &Value) -> Option<usize> {
    match v {
        Value::Text(t) => match t.to_ascii_uppercase().as_str() {
            "X" => Some(0),
            "Y" => Some(1),
            "Z" => Some(2),
            _ => None,
        },
        _ => None,
    }
}

/// A reference's rotation now (radians): x and y as `SetAngle` left them,
/// else placed; z its heading now.
fn rotation_now(order: &LoadOrder, state: &GameState, r: FormId) -> Option<[f32; 3]> {
    let (_, rot) = placed(order, r)?;
    let heading = state.place(order, r).map_or(rot[2], |p| p.3);
    let tilt = state
        .set_by_scripts
        .tilts
        .get(&r)
        .copied()
        .unwrap_or([rot[0], rot[1]]);
    Some([tilt[0], tilt[1], heading])
}

/// The weapon animation types (`WEAP` `DNAM` byte 0) as `GetWeaponAnimType`
/// gives them (the table at `0118a838`): hand-to-hand 1, one-hand melee 2,
/// two-hand melee 3, pistols (and energy pistols) 4, rifles (and energy
/// rifles) 5, automatics 6, handles 7, launchers 8, grenades and thrown 9,
/// mines 10, lunchbox mines 11.
const WEAPON_ANIM_TYPES: [u8; 14] = [1, 2, 3, 4, 4, 5, 6, 5, 7, 8, 9, 10, 11, 9];

/// The weapon in someone's hands as the process has it (`GetWeaponAnimType`,
/// `IsWeaponInList`): the player's equipped one only (none: fists); others
/// as the engine has them fight ([`crate::combat::weapon_in_hand`]: what
/// they're given or carry is drawn).
fn held_weapon(order: &LoadOrder, state: &GameState, who: FormId) -> Option<FormId> {
    if who == PLAYER_REF {
        return state
            .equipped
            .get(&who)
            .into_iter()
            .flatten()
            .copied()
            .find(|&i| kind_of(order, i) == Some(WEAP));
    }
    crate::combat::weapon_in_hand(order, state, who).map(|w| w.form_id)
}

/// Someone's killer: who struck the blow that killed them (the actor's
/// +0xc0; here the last blow kept with a death).
fn killer(state: &GameState, who: FormId) -> Option<FormId> {
    if !state.dead.contains(&who) {
        return None;
    }
    state.last_blow.get(&who).map(|b| b.0)
}

/// A function that reads, asked about `on`: `Some(answer)` when it's one
/// of these (the answer `None` when it can't be given), `None` when it
/// isn't.
pub(crate) fn value(
    facts: &Facts,
    name: &str,
    on: Option<FormId>,
    args: &[Value],
) -> Option<Option<f64>> {
    READS.contains(&name).then(|| read(facts, name, on, args))
}

fn read(facts: &Facts, name: &str, on: Option<FormId>, args: &[Value]) -> Option<f64> {
    let order = facts.order;
    let s = facts.state;
    let arg = |i: usize| args.get(i).cloned().unwrap_or(Value::Number(0.0));
    let flag = |b: bool| if b { 1.0 } else { 0.0 };
    {
        Some(match name {
            "GetGameSetting" => {
                let Value::Text(setting) = arg(0) else {
                    return None;
                };
                game_setting_value(order, &setting)?
            }
            "IsChild" => flag(is_child(order, s, on?)),
            // `0059ff90`: the life state is 5.
            "GetRestrained" => flag(s.set_by_scripts.restrained.contains(&on?)),
            // `005a1c30` / `0047b250`: 0 for anything that doesn't open; a
            // door 1 open, 3 shut, 2 opening and 4 closing while its
            // sequence plays (`world::doors`).
            "GetOpenState" => f64::from(crate::doors::open_state(order, s, on?) as i32),
            // `0059fc50` / `008b8290`: 1 if the person is in the faction,
            // else -1. New Vegas's own code gives no rank here.
            "GetFactionRank" => {
                let who = on?;
                if !is_actor(order, who) {
                    -1.0
                } else if faction_list(order, s, who).contains(&arg(0).form()) {
                    1.0
                } else {
                    -1.0
                }
            }
            // `0059c0c0`: the position now on an axis.
            "GetPos" => {
                let r = on?;
                let i = axis(&arg(0))?;
                let p = if r == PLAYER_REF {
                    s.player_position?
                } else {
                    s.place(order, r)?.2
                };
                f64::from(p[i])
            }
            // `0059c170`: the rotation now in degrees (radians × 57.295776).
            // The player's heading isn't kept in the state.
            "GetAngle" => {
                let r = on?;
                if r == PLAYER_REF {
                    return None;
                }
                let i = axis(&arg(0))?;
                f64::from(rotation_now(order, s, r)?[i] * 57.295776)
            }
            // `0059c230` / `0059c2d0`: where the reference started (as
            // placed); people's starting rotation is their heading alone
            // (`008a8290`: x and y 0).
            "GetStartingPos" => {
                let i = axis(&arg(0))?;
                f64::from(placed(order, on?)?.0[i])
            }
            "GetStartingAngle" => {
                let r = on?;
                let i = axis(&arg(0))?;
                let (_, mut rot) = placed(order, r)?;
                if is_actor(order, r) {
                    rot[0] = 0.0;
                    rot[1] = 0.0;
                }
                f64::from(rot[i] * 57.295776)
            }
            // `0059ecc0`: the base is a creature (leveled creatures place
            // creatures).
            "GetIsCreature" => {
                let r = on?;
                let base = base_of(order, r).and_then(|b| kind_of(order, b));
                flag(matches!(base, Some(k) if k == CREA || k == LVLC))
            }
            // `0059f300`: a creature's type (`CREA` `DATA` byte 0).
            "GetIsCreatureType" => {
                let base = base_of(order, on?)?;
                let rr = order.get(base).filter(|r| r.entry.header.kind == CREA);
                let kind = rr
                    .and_then(|r| r.record().ok())
                    .and_then(|r| r.get(esm::sig::DATA).and_then(|d| d.data.first().copied()));
                flag(kind.is_some_and(|k| f64::from(k) == arg(0).number()))
            }
            // `0059f180`: a person's class (`CNAM`).
            "GetIsClass" => {
                let base = base_of(order, on?)?;
                let class = (kind_of(order, base) == Some(NPC_))
                    .then(|| sub_form(order, base, CNAM))
                    .flatten();
                flag(class == Some(arg(0).form()))
            }
            // `005a09b0`: the weapon in hand's animation type by
            // [`WEAPON_ANIM_TYPES`]; none in hand 1; not a person 0.
            "GetWeaponAnimType" => {
                let who = on?;
                if !is_actor(order, who) {
                    0.0
                } else {
                    match held_weapon(order, s, who)
                        .and_then(|w| crate::combat::Weapon::load(order, w))
                    {
                        Some(w) => {
                            f64::from(*WEAPON_ANIM_TYPES.get(w.animation as usize).unwrap_or(&0))
                        }
                        None => 1.0,
                    }
                }
            }
            // `005a0ab0`, asked about a weapon (a perk's weapon
            // condition): its skill (`DNAM` at 104) is the actor value;
            // anything else counts as unarmed (45).
            "IsWeaponSkillType" => {
                let r = on?;
                let weapon = if kind_of(order, r) == Some(WEAP) {
                    Some(r)
                } else {
                    base_of(order, r).filter(|b| kind_of(order, *b) == Some(WEAP))
                };
                let av = arg(0).number();
                match weapon.and_then(|w| crate::combat::Weapon::load(order, w)) {
                    Some(w) => flag(f64::from(w.skill) == av),
                    None => flag(av == 45.0),
                }
            }
            // `005a05e0`: the weapon in hand (else the fists) is in the
            // list; not a person 0.
            "IsWeaponInList" => {
                let who = on?;
                if !is_actor(order, who) {
                    0.0
                } else {
                    let weapon = held_weapon(order, s, who).unwrap_or(FISTS);
                    flag(form_list(order, s, arg(0).form()).contains(&weapon))
                }
            }
            // `0059f890`: the reference's base is in the list.
            "IsInList" => {
                let base = base_of(order, on?)?;
                flag(form_list(order, s, arg(0).form()).contains(&base))
            }
            // `005a3270`: the reference's cell is an interior.
            "IsInInterior" => {
                let r = on?;
                if r == PLAYER_REF {
                    flag(s.player_cell.is_some() && s.player_world.is_none())
                } else {
                    let (space, cell, _, _) = s.place(order, r)?;
                    flag(space == cell)
                }
            }
            // `005de4a0`: an image space modifier of this kind is playing.
            "IsImageSpaceActive" => flag(s.modifiers.contains(&arg(0).form())),
            // `005a3f00`: the person is dead and this one killed them.
            "IsKiller" => flag(killer(s, on?) == Some(arg(0).form())),
            // `005a5e90` / `005a5f40`: a skill (32–45) among the player's
            // tagged ones (the player's class's tag skills, which the tag
            // menu sets).
            "IsPlayerTagSkill" => {
                let av = arg(0).number();
                flag((32.0..=45.0).contains(&av) && s.tag_skills.contains(&(av as u16)))
            }
            // `0059ef60`: the reference (else the one asked about) is in
            // the cell, by editor ID ([`cell_matches`]).
            "GetInCellParam" => {
                let who = match arg(1).form() {
                    FormId(0) => on?,
                    r => r,
                };
                flag(cell_of(order, s, who).is_some_and(|c| cell_matches(order, c, arg(0).form())))
            }
            // `005a2e20`: the reference's own owner is this one (else the
            // player).
            "IsOwner" => {
                let wanted = match args.first().map(Value::form) {
                    Some(f) if f.0 != 0 => f,
                    _ => PLAYER_BASE,
                };
                flag(own_owner(order, s, on?) == Some(wanted))
            }
            // `005a0410`: from the person's heading to the direction of
            // the other, in degrees, −180 to 180 (positive clockwise).
            "GetHeadingAngle" => {
                let me = on?;
                if !is_actor(order, me) || me == PLAYER_REF {
                    return if is_actor(order, me) { None } else { Some(0.0) };
                }
                let (_, _, from, heading) = s.place(order, me)?;
                let to = match arg(0).form() {
                    PLAYER_REF => s.player_position?,
                    r => s.place(order, r)?.2,
                };
                let (dx, dy) = (to[0] - from[0], to[1] - from[1]);
                let mut a = dx.atan2(dy) - heading;
                let pi = std::f32::consts::PI;
                while a < -pi {
                    a += 2.0 * pi;
                }
                while a > pi {
                    a -= 2.0 * pi;
                }
                f64::from(a * 57.295_776)
            }
            // `005a3820`: always 0 on the PC.
            "IsPS3" => 0.0,
            // `005a3d00`: the player's third-person flag (player+0x64A) is
            // clear. This engine has only the first-person view.
            "IsPC1stPerson" => 1.0,
            // `0059c930`: the reference's scale (`SetScale`, else `XSCL`).
            "GetScale" => {
                let r = on?;
                let scale = match s.scales.get(&r) {
                    Some(&v) => v,
                    None => order
                        .get(r)
                        .and_then(|rr| rr.record().ok())
                        .and_then(|rec| {
                            rec.get(XSCL)
                                .filter(|d| d.data.len() >= 4)
                                .map(|d| le_f32(&d.data, 0))
                        })
                        .unwrap_or(1.0),
                };
                f64::from(scale)
            }
            // `005a30c0`: how often the player has hit this friend or ally
            // lately (extra data 0x45, `world::crime::friend_hit_count`).
            "GetFriendHit" => {
                let who = on?;
                if !is_actor(order, who) {
                    0.0
                } else {
                    f64::from(crate::crime::friend_hit_count(order, s, who))
                }
            }
            // `005a49f0` / `008b87a0`: one of the person's factions holds
            // the player as an enemy for their crimes.
            "GetActorFactionPlayerEnemy" => {
                let who = on?;
                flag(
                    is_actor(order, who)
                        && faction_list(order, s, who)
                            .iter()
                            .any(|f| s.crime_enemies.contains(f)),
                )
            }
            // `005a4210`: the reference exists and isn't this one.
            "Exists" => {
                let r = on?;
                flag(r.0 != 0 && r != arg(0).form())
            }
            // (`GetChallengeCompleted`: `world::more_functions::
            // challenges`.)
            // `005a6170`: the player's winnings at the casino by quarters
            // of its limit (`world::casino`; 0 where not played).
            "GetCasinoWinningStage" => {
                f64::from(crate::casino::winnings_level(order, s, arg(0).form()))
            }
            // `005a6280`: the region is in the player's list of regions
            // (player+0x764), the same list `IsPlayerInRegion` reads
            // (`005cf490`).
            "PlayerInRegion" => flag(facts.player_in_region(arg(0).form())),
            // `005a1150`: a person's process says a voice line is playing
            // (HighProcess +0x4b8, `008f6fc0`; the person's flag +0x7d,
            // which also counts, isn't traced); here, someone the viewer
            // has saying a line (`GameState::speaking`). Talking
            // activators speak for others: not here.
            "IsTalking" => flag(s.speaking.contains(&on?)),
            // `005a0b60`: the person's current package's type by
            // [`PACKAGE_NUMBERS`]; none 0; not a person 0. Someone
            // fighting follows the game's own combat package, whose
            // number isn't traced: not answered.
            "GetCurrentAIPackage" => {
                let who = on?;
                if !is_actor(order, who) {
                    return Some(0.0);
                }
                if s.combat.contains_key(&who) {
                    return None;
                }
                match crate::ai::current_package(order, s, who) {
                    Some(p) => f64::from(
                        PACKAGE_NUMBERS
                            .get(usize::from(if p.kind == 0x11 { 0 } else { p.kind }))
                            .copied()
                            .unwrap_or(-1),
                    ),
                    None => 0.0,
                }
            }
            _ => return None,
        })
    }
}

/// Whether a door is open now (`world::doors::is_open`).
pub fn door_open(order: &LoadOrder, state: &GameState, door: FormId) -> bool {
    crate::doors::is_open(order, state, door)
}

/// Whether an objective has been completed (shown or not; the game's
/// objective state bit 0x02).
pub fn objective_completed(state: &GameState, quest: FormId, index: i32) -> bool {
    state
        .objectives
        .get(&(quest, index))
        .copied()
        .unwrap_or(false)
        || state
            .set_by_scripts
            .hidden_completed
            .contains(&(quest, index))
}

/// An objective's state changes (`005ec5d0`): 0 neither shown nor done, 1
/// shown, 2 done but not shown, 3 shown and done. Showing it (1) shows
/// "objective added" and starts its quest; going from shown to done (3)
/// shows "objective completed".
pub fn set_objective(runner: &mut Runner, quest: FormId, index: i32, new: u8) {
    let key = (quest, index);
    let shown = runner.state.objectives.contains_key(&key);
    runner.state.emit(crate::coop::Domain::Objective, quest);
    runner.state.emit(crate::coop::Domain::Flags, quest);
    match new {
        0 => {
            runner.state.objectives.remove(&key);
            runner.state.set_by_scripts.hidden_completed.remove(&key);
        }
        1 => {
            runner.state.objectives.insert(key, false);
            runner.state.set_by_scripts.hidden_completed.remove(&key);
            runner.state.running.insert(quest);
            // `005ec5d0`: the quest becomes active when none is.
            crate::quest_targets::objective_shown(runner.state, quest);
            let text = objective_text(runner, quest, index);
            runner.state.events.push(Event::Objective {
                quest,
                text,
                completed: false,
            });
            // The first shown announces its quest ("Quest added").
            crate::quest_text::objective_shown(runner.state, quest);
        }
        2 => {
            runner.state.objectives.remove(&key);
            runner.state.set_by_scripts.hidden_completed.insert(key);
        }
        _ => {
            runner.state.objectives.insert(key, true);
            runner.state.set_by_scripts.hidden_completed.remove(&key);
            // Only for an objective that was shown, of a quest not
            // completed (`005ec5d0`).
            if shown && !runner.state.completed.contains(&quest) {
                let text = objective_text(runner, quest, index);
                runner.state.events.push(Event::Objective {
                    quest,
                    text,
                    completed: true,
                });
            }
        }
    }
}

fn objective_text(runner: &Runner, quest: FormId, index: i32) -> String {
    runner
        .scripts
        .quest(runner.order, quest)
        .and_then(|q| q.objective(index).map(|o| o.text.clone()))
        .unwrap_or_default()
}

/// The form type numbers `RemoveAllTypedItems` takes (the forms' type at
/// +4: the item types the game's own "item added" notices list, `004ce380`:
/// 0x18 armour … 0x74 caravan money).
pub fn form_type(kind: FourCC) -> Option<u8> {
    Some(match kind.as_bytes() {
        b"ARMO" => 0x18,
        b"BOOK" => 0x19,
        b"CLOT" => 0x1A,
        b"INGR" => 0x1D,
        b"LIGH" => 0x1E,
        b"MISC" => 0x1F,
        b"WEAP" => 0x28,
        b"AMMO" => 0x29,
        b"KEYM" => 0x2E,
        b"ALCH" => 0x2F,
        b"NOTE" => 0x31,
        b"COBJ" => 0x32,
        b"IMOD" => 0x67,
        b"CHIP" => 0x6C,
        b"CCRD" => 0x73,
        b"CMNY" => 0x74,
        _ => return None,
    })
}

/// `RemoveAllItems` and `RemoveAllTypedItems` (both `004ce380`): a
/// holder's items go to another (or are gone), all of them or those of one
/// form type (−1 all), leaving those in a holdout form list, and the
/// player's quest items stay with the player. What's moved is taken off
/// if worn. The ownership flags (marking moved items as the holder's) and
/// the "added" notices aren't modelled.
pub fn remove_all(
    order: &LoadOrder,
    state: &mut GameState,
    from: FormId,
    to: Option<FormId>,
    kind: i64,
    holdout: Option<FormId>,
) {
    state.stock(order, from);
    if let Some(to) = to {
        state.stock(order, to);
    }
    let keep: Vec<FormId> = holdout.map_or_else(Vec::new, |l| form_list(order, state, l));
    let items = state.inventory(order, from);
    for (item, n) in items {
        if keep.contains(&item) {
            continue;
        }
        if kind >= 0 {
            let t = kind_of(order, item).and_then(form_type);
            if t.map(i64::from) != Some(kind) {
                continue;
            }
        }
        if from == PLAYER_REF && is_quest_item(order, state, item) {
            continue;
        }
        state.items.remove(&(from, item));
        state.unequip_item(order, from, item);
        if let Some(to) = to {
            *state.items.entry((to, item)).or_insert(0) += n;
            state.added(order, to, item, n);
        }
    }
}

/// A function that changes things, on `target`: `Some(value)` when it's
/// one of these (`None` inside when it can't be carried out), `None` when
/// it isn't.
pub(crate) fn change(
    runner: &mut Runner,
    name: &str,
    target: Option<FormId>,
    args: &[Value],
) -> Option<Option<f64>> {
    CHANGES
        .contains(&name)
        .then(|| carry_out(runner, name, target, args))
}

fn carry_out(
    runner: &mut Runner,
    name: &str,
    target: Option<FormId>,
    args: &[Value],
) -> Option<f64> {
    let order = runner.order;
    let arg = |i: usize| args.get(i).cloned().unwrap_or(Value::Number(0.0));
    let number_or = |i: usize, default: f64| args.get(i).map_or(default, Value::number);
    {
        match name {
            // `005d0920` / `008ace50`: 1 sets the person's life state to
            // restrained (5); 0 frees them if restrained.
            "SetRestrained" => {
                let who = target.filter(|&w| is_actor(order, w))?;
                let r = &mut runner.state.set_by_scripts.restrained;
                if arg(0).number() != 0.0 {
                    r.insert(who);
                } else {
                    r.remove(&who);
                }
            }
            // `005db3f0`: a person ignores crime (above 0) or not.
            "IgnoreCrime" => {
                let who = target.filter(|&w| is_actor(order, w))?;
                let set = &mut runner.state.set_by_scripts.ignoring_crime;
                if arg(0).number() > 0.0 {
                    set.insert(who);
                } else {
                    set.remove(&who);
                }
            }
            // `005ced30`: a door is activated (by nobody) when it isn't
            // already as asked: open or opening (1) or shut or closing (0).
            // The activation (`00573170`) runs the door's own `OnActivate`
            // block when it has one, and the door only moves if that block
            // calls `Activate`; a lock isn't in the way (`005180b0` only
            // asks the lock about an actor). Its sound still plays
            // (`world::doors`).
            "SetOpenState" => {
                let door = target?;
                let open = arg(0).number() != 0.0;
                let now = crate::doors::open_state(order, runner.state, door);
                if now == crate::doors::OpenState::None || now.is_open() == open {
                    return Some(0.0);
                }
                let scripted = crate::scripting::script_of(order, door)
                    .and_then(|s| runner.scripts.script(order, s))
                    .is_some_and(|s| s.blocks.iter().any(|b| b.kind == "onactivate"));
                if scripted {
                    let before = runner.state.events.len();
                    // `00573170` runs the block only fewer than 5 deep, so a
                    // door whose own script calls `SetOpenState` on itself
                    // stops after five levels.
                    runner.run_activate_block(door, FormId(0));
                    let went_on = runner.state.events[before..]
                        .iter()
                        .any(|e| matches!(e, Event::Activate { what, .. } if *what == door));
                    runner
                        .state
                        .events
                        .retain(|e| !matches!(e, Event::Activate { what, .. } if *what == door));
                    if !went_on {
                        return Some(0.0);
                    }
                }
                if let Some(a) = crate::doors::set_open_state(order, runner.state, door, open) {
                    if let Some(sound) = crate::doors::sound(order, door, a) {
                        runner.state.events.push(Event::Sound(sound));
                    }
                }
            }
            // `005b57e0`: the reference is owned by this one (else the
            // player).
            "SetOwnership" => {
                let r = target?;
                let owner = match args.first().map(Value::form) {
                    Some(f) if f.0 != 0 => f,
                    _ => PLAYER_BASE,
                };
                runner.state.set_by_scripts.owners.insert(r, owner);
            }
            // `005d17f0`: the cell is owned by this one (else the player).
            "SetCellOwnership" => {
                let cell = arg(0).form();
                if cell.0 == 0 {
                    return None;
                }
                let owner = match args.get(1).map(Value::form) {
                    Some(f) if f.0 != 0 => f,
                    _ => PLAYER_BASE,
                };
                runner.state.set_by_scripts.cell_owners.insert(cell, owner);
            }
            // `005c7660`: the cell's public flag (0x20) on (above 0) or off.
            "SetCellPublicFlag" => {
                let cell = arg(0).form();
                if cell.0 == 0 {
                    return None;
                }
                let public = arg(1).number() > 0.0;
                runner
                    .state
                    .set_by_scripts
                    .public_cells
                    .insert(cell, public);
            }
            // `005d8e80`: like `AddItem` (a leveled list gives what it
            // picks, a form list each of its items), the items at a health
            // (0–1). Item condition is kept for weapons and armour
            // (`GameState::weapon_health`). The flag 1
            // silences the "added" notice (none here).
            "AddItemHealthPercent" => {
                let holder = target?;
                let item = arg(0).form();
                let count = arg(1).number() as i32;
                let health = arg(2).number() as f32;
                runner.state.stock(order, holder);
                let mut added: Vec<(FormId, i32)> = Vec::new();
                match kind_of(order, item) {
                    Some(k) if k == LVLI => {
                        let level = runner.state.player_level;
                        let state = &mut *runner.state;
                        added = crate::leveled::resolve(order, item, count, level, &mut || {
                            state.roll()
                        });
                    }
                    Some(k) if k == FLST => {
                        for f in form_list(order, runner.state, item) {
                            if kind_of(order, f).is_some_and(crate::scripting::is_item) {
                                added.push((f, count));
                            }
                        }
                    }
                    Some(k) if crate::scripting::is_item(k) => added.push((item, count)),
                    _ => return None,
                }
                for (i, n) in added {
                    *runner.state.items.entry((holder, i)).or_insert(0) += n;
                    runner.state.added(order, holder, i, n);
                    if crate::repair::has_condition(order, i) {
                        runner
                            .state
                            .weapon_health
                            .insert((holder, i), health.clamp(0.0, 1.0));
                    }
                }
            }
            // `005d3b20`: the platform's achievement by number.
            "AddAchievement" => {
                let n = arg(0).number() as u32;
                runner.state.set_by_scripts.achievements.insert(n);
            }
            // `005d3d50`: the reference's form flag 0x100000 on (not 0) or
            // off; -1 (or nothing) leaves it.
            "SetIgnoreFriendlyHits" => {
                let r = target?;
                let v = number_or(0, -1.0);
                if v != -1.0 {
                    runner
                        .state
                        .set_by_scripts
                        .ignore_friendly_hits
                        .insert(r, v != 0.0);
                }
            }
            // `005d13e0`: `EnableFastTravel 1 [x keep]` allows fast travel
            // and waiting, keeping it allowed across moves only when keep
            // is 1; `EnableFastTravel 0 [wait keep]` forbids fast travel,
            // allows waiting by the second flag (1 when left out), and a
            // third 1 keeps it forbidden when the player is moved.
            "EnableFastTravel" => {
                let enable = arg(0).number() != 0.0;
                let ft = &mut runner.state.set_by_scripts.fast_travel;
                let keep = number_or(2, 0.0) == 1.0;
                if enable {
                    ft.enabled = true;
                    ft.can_wait = true;
                    ft.keep = keep;
                } else {
                    ft.enabled = false;
                    ft.can_wait = number_or(1, 1.0) != 0.0;
                    if keep {
                        ft.keep = true;
                    }
                }
            }
            // `005d52a0`: the player no longer has the note.
            "RemoveNote" => {
                runner.state.notes.remove(&arg(0).form());
            }
            // `005c9740` / `0060c950`: every objective of the quest is
            // done, shown ones staying shown (state 1 → 3, 0 → 2).
            "CompleteAllObjectives" => {
                let quest = arg(0).form();
                let objectives: Vec<i32> = runner
                    .scripts
                    .quest(order, quest)
                    .map(|q| q.objectives.iter().map(|o| o.index).collect())
                    .unwrap_or_default();
                for index in objectives {
                    let shown = runner.state.objectives.contains_key(&(quest, index));
                    if objective_completed(runner.state, quest, index) {
                        continue;
                    }
                    set_objective(runner, quest, index, if shown { 3 } else { 2 });
                }
            }
            // `005dee30` / `005f64f0`: the challenge is unlocked (its flag
            // 0x01).
            "UnlockChallenge" => {
                let c = arg(0).form();
                if c.0 == 0 {
                    return None;
                }
                runner.state.set_by_scripts.challenges.insert(c);
            }
            // `005de600` / `00590010`: the form goes into the list unless
            // it's there.
            "AddFormToFormList" => {
                let (list, form) = (arg(0).form(), arg(1).form());
                if list.0 != 0 && form.0 != 0 {
                    let added = runner
                        .state
                        .set_by_scripts
                        .list_additions
                        .entry(list)
                        .or_default();
                    if !added.contains(&form) {
                        added.push(form);
                    }
                }
            }
            // `005c6b60`: a person's health is restored by what it lacks
            // (permanent − current), and the seven body parts (25–31) by
            // 999 (`0088b740` restores only what's damaged).
            "ResetHealth" => {
                let who = target.filter(|&w| is_actor(order, w))?;
                let facts = Facts {
                    order,
                    state: runner.state,
                    speaker: None,
                };
                let lacking = facts.permanent_actor_value(who, 16).unwrap_or(0.0)
                    - facts.current_actor_value(who, 16).unwrap_or(0.0);
                if lacking > 0.0 {
                    crate::magic::change(order, runner.state, who, 16, lacking, who);
                }
                for av in 25..=31 {
                    crate::magic::change(order, runner.state, who, av, 999.0, who);
                }
            }
            // `005b6cd0`: unloads cells the game keeps in memory. Nothing
            // to unload here (no cell buffers), and nothing in the game
            // changes.
            "PurgeCellBuffers" => {}
            // `005c8340` / `004370f0`: every faction the person is in is
            // taken away (rank −1).
            "RemoveFromAllFactions" => {
                let who = target.filter(|&w| is_actor(order, w))?;
                for f in faction_list(order, runner.state, who) {
                    runner.state.faction_changes.insert((who, f), -1);
                }
            }
            // `005d1920`: a person's base (or a talking activator's) is
            // renamed to the message's name: its full-name component at
            // +0x18, read through `00408da0` (the name at +4 of a
            // `TESFullName`), i.e. the message's `FULL` ("" when it has
            // none, as `NamedActorYesMan`).
            "SetActorFullName" => {
                let r = target?;
                let message = arg(0).form();
                let rr = order.get(message).filter(|r| r.entry.header.kind == MESG)?;
                let text = rr.record().ok()?.full_name().unwrap_or_default();
                let base = base_of(order, r)?;
                let renamed = is_actor(order, r)
                    || kind_of(order, base).is_some_and(|k| k.as_bytes() == b"TACT");
                if renamed {
                    runner.state.set_by_scripts.names.insert(base, text);
                }
            }
            // `005dac60` (people: `0089fb80`): the inventory goes back to
            // the base record's (leveled lists picked again when it's next
            // changed) and what was worn comes off.
            "ResetInventory" => {
                let r = target?;
                runner.state.stocked.remove(&r);
                runner.state.items.retain(|(h, _), _| *h != r);
                runner.state.equipped.remove(&r);
                runner.state.taken_off.remove(&r);
            }
            // `005c05f0` / `005c0740`: one coordinate of the position.
            "SetPos" => {
                let r = target?;
                let i = axis(&arg(0))?;
                let v = arg(1).number() as f32;
                if r == PLAYER_REF {
                    let mut p = runner.state.player_position?;
                    p[i] = v;
                    runner.state.player_position = Some(p);
                } else {
                    let (space, cell, mut p, heading) = runner.state.place(order, r)?;
                    p[i] = v;
                    runner.state.positions.insert(r, (p, heading));
                    runner.state.spaces.insert(r, (space, cell));
                }
            }
            // `005c09c0` / `005c0b10`: one rotation, in degrees (× 0.017453292).
            // The player's heading isn't kept in the state.
            "SetAngle" => {
                let r = target.filter(|&r| r != PLAYER_REF)?;
                let i = axis(&arg(0))?;
                let v = arg(1).number() as f32 * 0.017_453_292;
                let (space, cell, p, _) = runner.state.place(order, r)?;
                if i == 2 {
                    runner.state.positions.insert(r, (p, v));
                    runner.state.spaces.insert(r, (space, cell));
                } else {
                    let rot = rotation_now(order, runner.state, r)?;
                    let mut tilt = [rot[0], rot[1]];
                    tilt[i] = v;
                    runner.state.set_by_scripts.tilts.insert(r, tilt);
                }
            }
            // `005da180` / `0060d720`: the quest as at the start: no stage
            // done, every objective neither shown nor done, its script's
            // variables fresh, not completed or failed, and running only
            // if it starts with the game.
            "ResetQuest" => {
                let quest = arg(0).form();
                if kind_of(order, quest) != Some(QUST) {
                    return None;
                }
                let st = &mut *runner.state;
                st.stages.remove(&quest);
                st.stages_done.retain(|(q, _)| *q != quest);
                st.objectives.retain(|(q, _), _| *q != quest);
                st.set_by_scripts
                    .hidden_completed
                    .retain(|(q, _)| *q != quest);
                st.variables.remove(&quest);
                st.completed.remove(&quest);
                st.failed.remove(&quest);
                st.quests_announced.remove(&quest);
                st.quest_timers.remove(&quest);
                let starts = order
                    .get(quest)
                    .and_then(|r| r.record().ok())
                    .and_then(|r| r.get(esm::sig::DATA).and_then(|s| s.data.first().copied()))
                    .is_some_and(|f| f & crate::quest::START_GAME_ENABLED != 0);
                if starts {
                    st.running.insert(quest);
                } else {
                    st.running.remove(&quest);
                }
            }
            // `005cd910`: the form's quest-item flag on (above 0) or off.
            "SetQuestObject" => {
                let form = arg(0).form();
                if form.0 == 0 {
                    return None;
                }
                let on = arg(1).number() > 0.0;
                runner.state.set_by_scripts.quest_items.insert(form, on);
            }
            // `005b55a0`: see [`remove_all`]; the parameters are the
            // container (none: gone), "ownership added", "no message", the
            // form type (−1 or left out: all) and the holdout list.
            "RemoveAllTypedItems" => {
                let from = target?;
                let to = args.first().map(Value::form).filter(|f| f.0 != 0);
                let kind = number_or(3, -1.0) as i64;
                let holdout = args.get(4).map(Value::form).filter(|f| f.0 != 0);
                remove_all(order, runner.state, from, to, kind, holdout);
            }
            // `005c8a50`: the person says a line of the topic aloud, to no
            // one in particular (as `SayTo` without a listener).
            "Say" => {
                let speaker = target?;
                let topic = arg(0).form();
                runner
                    .state
                    .coop_talk(speaker, FormId(0), (topic.0 != 0).then_some(topic), false);
                runner.state.events.push(Event::Talk {
                    speaker,
                    to: FormId(0),
                    topic: (topic.0 != 0).then_some(topic),
                    conversation: false,
                });
            }
            // `005c0df0`: an animation group to play on a person or an
            // object's model.
            "PlayGroup" => {
                let who = target?;
                let Value::Text(group) = arg(0) else {
                    return None;
                };
                let flags = arg(1).number() as i32;
                runner
                    .state
                    .events
                    .push(Event::PlayGroup { who, group, flags });
            }
            // `005cb2d0`: an idle (`IDLE`, by editor ID) for a person to
            // play.
            "PlayIdle" => {
                let who = target.filter(|&w| is_actor(order, w))?;
                let Value::Text(idle_name) = arg(0) else {
                    return None;
                };
                let idle = order
                    .form_by_editor_id(&idle_name)
                    .filter(|f| kind_of(order, *f) == Some(IDLE))?;
                runner.state.events.push(Event::PlayIdle { who, idle });
            }
            // `005cf860`: a node of a reference's model gets another
            // texture (`Textures\<name>.dds`, its first slot). Only on the
            // model while it's loaded: nothing is kept.
            "SwapTextureOnRef" => {
                let what = arg(0).form();
                let (Value::Text(node), Value::Text(texture)) = (arg(1), arg(2)) else {
                    return None;
                };
                if what.0 == 0 {
                    return None;
                }
                runner.state.events.push(Event::SwapTexture {
                    what,
                    node,
                    texture: format!("Textures\\{texture}.dds"),
                });
            }
            // `005c9790`: a person looks at someone (head tracking; kept
            // here, not shown). `StopLook` ends it.
            "Look" => {
                let who = target.filter(|&w| is_actor(order, w))?;
                let at = arg(0).form();
                if at.0 != 0 {
                    runner.state.set_by_scripts.looking.insert(who, at);
                }
            }
            // `005d8bf0`: the text is formatted (`005b4960`) and dropped:
            // the shipped game shows nothing.
            "ShowWarning" => {}
            // `005d8cf0`: the trigger it runs on takes someone as having
            // entered it: its script's `OnTriggerEnter` and `OnTrigger`
            // events are flagged for them (0x20000000, 0x10000000,
            // `005a8e20`) and the script runs once with them as the action
            // reference (`00565870`), unless 5 such runs are already under
            // way.
            "EnterTrigger" => {
                let trigger = target?;
                let who = arg(0).form();
                if who.0 == 0 {
                    return None;
                }
                let depth = TRIGGER_DEPTH.with(|d| d.get());
                if depth < 5 {
                    TRIGGER_DEPTH.with(|d| d.set(depth + 1));
                    runner.run_events(trigger, &["ontriggerenter", "ontrigger"], who);
                    TRIGGER_DEPTH.with(|d| d.set(depth));
                }
            }
            // `005da2a0`, run on a person (else nothing): someone reports
            // the player's assault (`world::crime::assault_crime`,
            // `008c0460`) and turns on the player (the process's alarm,
            // +0x33c). Who: the person it runs on (no arguments); the
            // person named (not the player); or, for `SendAssaultAlarm
            // Player <faction>`, the faction's nearest loaded member who
            // notices the player (`00970b30`; here someone in the player's
            // cell within 10000 units whose detection of the player is
            // above −20). `SendAssaultAlarm Player` alone does nothing.
            "SendAssaultAlarm" => {
                let caller = target.filter(|&w| is_actor(order, w))?;
                let who = args.first().map(Value::form).filter(|f| f.0 != 0);
                let faction = args.get(1).map(Value::form).filter(|f| f.0 != 0);
                let victim = match (who, faction) {
                    (None, None) => Some(caller),
                    (Some(a), _) if a != PLAYER_REF => Some(a),
                    (_, Some(f)) => alarmed_member(order, runner.state, f),
                    _ => None,
                };
                if let Some(v) = victim {
                    crate::crime::assault_crime(order, runner.state, v);
                    runner.state.combat.insert(v, PLAYER_REF);
                }
            }
            _ => return None,
        }
        Some(0.0)
    }
}

/// A faction's member to raise an alarm (`00970b30` with both flags): the
/// nearest to the player of the living people in the player's cell who
/// are in the faction (a rank of 0 or more) and notice the player, within
/// 10000 units.
fn alarmed_member(order: &LoadOrder, state: &GameState, faction: FormId) -> Option<FormId> {
    let cell = state.player_cell?;
    let feet = state.player_position?;
    let facts = Facts {
        order,
        state,
        speaker: None,
    };
    let mut best: Option<(f32, FormId)> = None;
    for rr in order.references_in_cell(cell) {
        let who = rr.form_id;
        if rr.entry.header.kind != ACHR
            || state.dead.contains(&who)
            || !faction_list(order, state, who).contains(&faction)
        {
            continue;
        }
        if !facts
            .detection(who, PLAYER_REF)
            .is_some_and(|v| v > crate::detection::NOTICED)
        {
            continue;
        }
        let Some((_, _, p, _)) = state.place(order, who) else {
            continue;
        };
        let d = (0..3).map(|i| (p[i] - feet[i]).powi(2)).sum::<f32>().sqrt();
        if d < 10_000.0 && best.map_or(true, |(b, _)| d < b) {
            best = Some((d, who));
        }
    }
    best.map(|(_, who)| who)
}

/// The lines these functions add to a save.
pub(crate) fn save_lines(state: &GameState, line: &mut dyn FnMut(String)) {
    let id = |f: FormId| format!("{:08X}", f.0);
    let x = &state.set_by_scripts;
    let sorted = |set: &HashSet<FormId>| {
        let mut v: Vec<FormId> = set.iter().copied().collect();
        v.sort();
        v
    };
    for (word, set) in [
        ("restrained", &x.restrained),
        ("ignorecrime", &x.ignoring_crime),
        ("challenge", &x.challenges),
    ] {
        for f in sorted(set) {
            line(format!("{word} {}", id(f)));
        }
    }
    let flags = |m: &HashMap<FormId, bool>| {
        let mut v: Vec<(FormId, bool)> = m.iter().map(|(k, v)| (*k, *v)).collect();
        v.sort();
        v
    };
    for (word, map) in [
        ("friendlyhits", &x.ignore_friendly_hits),
        ("open", &x.open),
        ("publiccell", &x.public_cells),
        ("questitem", &x.quest_items),
    ] {
        for (f, on) in flags(map) {
            line(format!("{word} {} {}", id(f), u8::from(on)));
        }
    }
    let pairs = |m: &HashMap<FormId, FormId>| {
        let mut v: Vec<(FormId, FormId)> = m.iter().map(|(k, v)| (*k, *v)).collect();
        v.sort();
        v
    };
    for (word, map) in [
        ("owner", &x.owners),
        ("cellowner", &x.cell_owners),
        ("look", &x.looking),
    ] {
        for (a, b) in pairs(map) {
            line(format!("{word} {} {}", id(a), id(b)));
        }
    }
    for a in &x.achievements {
        line(format!("achievement {a}"));
    }
    let mut lists: Vec<_> = x.list_additions.iter().collect();
    lists.sort();
    for (list, forms) in lists {
        for f in forms {
            line(format!("listadd {} {}", id(*list), id(*f)));
        }
    }
    let ft = x.fast_travel;
    if ft != FastTravel::default() {
        line(format!(
            "fasttravel {} {} {}",
            u8::from(ft.enabled),
            u8::from(ft.keep),
            u8::from(ft.can_wait)
        ));
    }
    let mut names: Vec<_> = x.names.iter().collect();
    names.sort();
    for (base, name) in names {
        // The rest of the line, spaces and all.
        line(format!("fullname {} {name}", id(*base)));
    }
    let mut tilts: Vec<_> = x.tilts.iter().collect();
    tilts.sort_by_key(|(r, _)| **r);
    for (r, t) in tilts {
        line(format!("tilt {} {} {}", id(*r), t[0], t[1]));
    }
    for (q, i) in &x.hidden_completed {
        line(format!("hiddenobjective {} {i}", id(*q)));
    }
    // Who killed whom (`IsKiller`).
    let mut dead: Vec<FormId> = state.dead.iter().copied().collect();
    dead.sort();
    for who in dead {
        if let Some((by, _)) = state.last_blow.get(&who) {
            line(format!("killer {} {}", id(who), id(*by)));
        }
    }
}

/// Reads one of [`save_lines`]'s lines back: `None` if the word isn't one
/// of them.
pub(crate) fn load_line(state: &mut GameState, raw: &str) -> Option<Result<(), String>> {
    let parts: Vec<&str> = raw.split_whitespace().collect();
    let word = *parts.first()?;
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
    let x = &mut state.set_by_scripts;
    let result = (|| -> Result<bool, String> {
        match word {
            "restrained" => {
                x.restrained.insert(form(1)?);
            }
            "ignorecrime" => {
                x.ignoring_crime.insert(form(1)?);
            }
            "challenge" => {
                x.challenges.insert(form(1)?);
            }
            "friendlyhits" => {
                x.ignore_friendly_hits.insert(form(1)?, num(2)? != 0.0);
            }
            "open" => {
                x.open.insert(form(1)?, num(2)? != 0.0);
            }
            "publiccell" => {
                x.public_cells.insert(form(1)?, num(2)? != 0.0);
            }
            "questitem" => {
                x.quest_items.insert(form(1)?, num(2)? != 0.0);
            }
            "owner" => {
                x.owners.insert(form(1)?, form(2)?);
            }
            "cellowner" => {
                x.cell_owners.insert(form(1)?, form(2)?);
            }
            "look" => {
                x.looking.insert(form(1)?, form(2)?);
            }
            "achievement" => {
                x.achievements.insert(num(1)? as u32);
            }
            "listadd" => x.list_additions.entry(form(1)?).or_default().push(form(2)?),
            "fasttravel" => {
                x.fast_travel = FastTravel {
                    enabled: num(1)? != 0.0,
                    keep: num(2)? != 0.0,
                    can_wait: num(3)? != 0.0,
                };
            }
            "fullname" => {
                let base = form(1)?;
                let rest = raw.trim_start()["fullname".len()..].trim_start();
                let name = rest.get(8..).map(str::trim).unwrap_or_default();
                x.names.insert(base, name.to_string());
            }
            "tilt" => {
                x.tilts.insert(form(1)?, [num(2)? as f32, num(3)? as f32]);
            }
            "hiddenobjective" => {
                x.hidden_completed.insert((form(1)?, num(2)? as i32));
            }
            "killer" => {
                state.last_blow.insert(form(1)?, (form(2)?, 0.0));
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

/// A reference's name now: as `SetActorFullName` renamed its base, else
/// the base's `FULL` (for references made at run time too).
pub fn full_name(order: &LoadOrder, state: &GameState, reference: FormId) -> Option<String> {
    let base = crate::more_functions::placed::base_now(order, state, reference)?;
    if let Some(n) = state.set_by_scripts.names.get(&base) {
        return Some(n.clone());
    }
    order.get(base)?.record().ok()?.full_name()
}

/// Where a reference was placed, for callers outside (the starting
/// position `GetStartingPos` gives).
pub fn starting_position(order: &LoadOrder, reference: FormId) -> Option<[f32; 3]> {
    whereabouts(order, reference).map(|w| w.position)
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
        }
        assert!(READS.iter().all(|r| !CHANGES.contains(r)));
    }

    #[test]
    fn weapon_animation_types_follow_the_games_table() {
        // Pistols and energy pistols alike (4); thrown like grenades (9).
        assert_eq!(WEAPON_ANIM_TYPES[3], 4);
        assert_eq!(WEAPON_ANIM_TYPES[4], 4);
        assert_eq!(WEAPON_ANIM_TYPES[13], 9);
        assert_eq!(form_type(FourCC::new(b"WEAP")), Some(40));
        assert_eq!(form_type(FourCC::new(b"ARMO")), Some(24));
    }
}

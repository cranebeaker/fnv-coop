//! The running game: what has happened so far (quest stages, objectives,
//! script variables, items given and taken, what scripts enabled or
//! disabled) and the scripts that change it.
//!
//! - [`GameState`] holds it.
//! - [`Facts`] answers the game's questions about it: the functions that
//!   dialogue conditions ask and scripts read (`GetStage`, `GetIsID`,
//!   `GetItemCount`, …), one implementation for both.
//! - [`Runner`] runs scripts against it (it's the scripts'
//!   [`script::interp::Host`]) and carries out the functions that change
//!   things (`SetStage`, `AddItem`, `Disable`, `ShowMessage`, …). What the
//!   player should see happen (a message, someone starting to talk, a
//!   door being enabled) is queued as [`Event`]s for the viewer.
//!
//! A function this doesn't carry out yet stops the script where its value
//! is needed, rather than going on with a made-up value (which could set
//! quest stages wrongly); as a plain statement it's skipped. Both are
//! counted in [`GameState::unhandled`].

use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::sync::{Arc, Mutex};

use esm::{FormId, FourCC, LoadOrder};
use script::interp::{self, Host, Locals};
use script::{Arg, Call, Script};

pub use script::interp::Flow;

use crate::cell::{le_f32, le_u32};
use crate::chargen::CharacterMenu;
use crate::dialogue::{Condition, Speaker, PLAYER_BASE, PLAYER_REF};
use crate::quest::{self, Quest};

const QUST: FourCC = FourCC::new(b"QUST");
const GLOB: FourCC = FourCC::new(b"GLOB");
const FLTV: FourCC = FourCC::new(b"FLTV");
const SCPT: FourCC = FourCC::new(b"SCPT");
const SCRI: FourCC = FourCC::new(b"SCRI");
const SCTX: FourCC = FourCC::new(b"SCTX");
const SLSD: FourCC = FourCC::new(b"SLSD");
const SCVR: FourCC = FourCC::new(b"SCVR");
const MESG: FourCC = FourCC::new(b"MESG");
const DESC: FourCC = FourCC::new(b"DESC");
const FULL: FourCC = FourCC::new(b"FULL");
const NAME: FourCC = FourCC::new(b"NAME");
const DNAM: FourCC = FourCC::new(b"DNAM");
const PRKR: FourCC = FourCC::new(b"PRKR");
const XPRM: FourCC = FourCC::new(b"XPRM");
const AIDT: FourCC = FourCC::new(b"AIDT");
const CREA: FourCC = FourCC::new(b"CREA");
const FURN: FourCC = FourCC::new(b"FURN");
const WEAP: FourCC = FourCC::new(b"WEAP");
const TERM: FourCC = FourCC::new(b"TERM");
const XSCL: FourCC = FourCC::new(b"XSCL");
const OBND: FourCC = FourCC::new(b"OBND");
const XCNT: FourCC = FourCC::new(b"XCNT");
const CONT: FourCC = FourCC::new(b"CONT");
const CNTO: FourCC = FourCC::new(b"CNTO");
const ACBS: FourCC = FourCC::new(b"ACBS");
const XCLR: FourCC = FourCC::new(b"XCLR");
const XMRK: FourCC = FourCC::new(b"XMRK");
const FNAM: FourCC = FourCC::new(b"FNAM");
const GMST: FourCC = FourCC::new(b"GMST");
const ITXT: FourCC = FourCC::new(b"ITXT");
const CTDA: FourCC = FourCC::new(b"CTDA");

/// The player's controls, in the order `DisablePlayerControls` and
/// `EnablePlayerControls` take them (`VCG01`'s own comments: "1 1 1 1 0 0
/// 1 ; disable everything but looking", "1 0 0 1 1 1 0; only enable
/// movement and looking and 3rd person").
pub mod controls {
    pub const MOVEMENT: usize = 0;
    pub const PIPBOY: usize = 1;
    pub const FIGHTING: usize = 2;
    pub const POV: usize = 3;
    pub const LOOKING: usize = 4;
    pub const ROLLOVER: usize = 5;
    pub const SNEAKING: usize = 6;
    /// What a flag left out means: `DisablePlayerControls` alone turns off
    /// movement, the Pip-Boy, fighting and the view switch;
    /// `EnablePlayerControls` alone turns everything on (both as the
    /// editor documents them, not traced).
    pub const DISABLE_DEFAULTS: [bool; 7] = [true, true, true, true, false, false, false];
    pub const ENABLE_DEFAULTS: [bool; 7] = [true; 7];
}

/// A message's text with a script's values in its `%` places (`%.0f`,
/// `%g`, `%5.2f`…, in order; `%%` is a percent sign), as C's `printf`
/// writes them (`%g` without its exponent form).
pub fn fill_values(text: &str, values: &[f64]) -> String {
    let mut out = String::new();
    let mut next = values.iter();
    let mut chars = text.chars().peekable();
    while let Some(c) = chars.next() {
        if c != '%' {
            out.push(c);
            continue;
        }
        if chars.peek() == Some(&'%') {
            chars.next();
            out.push('%');
            continue;
        }
        // Flags, width, precision, then the conversion letter.
        let mut spec = String::new();
        while let Some(&d) = chars.peek() {
            if d.is_ascii_digit() || matches!(d, '.' | '-' | '+' | ' ' | '#') {
                spec.push(d);
                chars.next();
            } else {
                break;
            }
        }
        let Some(&kind) = chars
            .peek()
            .filter(|k| matches!(k, 'f' | 'g' | 'e' | 'd' | 'i'))
        else {
            out.push('%');
            out.push_str(&spec);
            continue;
        };
        chars.next();
        let v = next.next().copied().unwrap_or(0.0);
        let precision = spec
            .split_once('.')
            .and_then(|(_, p)| p.parse::<usize>().ok());
        out.push_str(&match kind {
            'd' | 'i' => format!("{}", v as i64),
            'f' => format!("{v:.*}", precision.unwrap_or(6)),
            // `%g` drops trailing zeros.
            _ => {
                let s = format!("{v:.*}", precision.unwrap_or(6));
                if s.contains('.') {
                    s.trim_end_matches('0').trim_end_matches('.').to_string()
                } else {
                    s
                }
            }
        });
    }
    out
}

/// A game setting (`GMST`) by name: floats (`f…`) and integers (`i…`).
pub fn game_setting(order: &LoadOrder, name: &str) -> Option<f32> {
    let rr = order.get(order.form_by_editor_id(name)?)?;
    if rr.entry.header.kind != GMST {
        return None;
    }
    let data = rr.subrecord(esm::sig::DATA).ok()??;
    if data.len() < 4 {
        return None;
    }
    match name.as_bytes().first() {
        Some(b'f') => Some(le_f32(&data, 0)),
        Some(b'i') => Some(le_u32(&data, 0) as i32 as f32),
        _ => None,
    }
}

/// A text game setting (`GMST` named `s…`): its text.
pub fn game_setting_text(order: &LoadOrder, name: &str) -> Option<String> {
    let rr = order.get(order.form_by_editor_id(name)?)?;
    if rr.entry.header.kind != GMST || !name.starts_with('s') {
        return None;
    }
    let record = rr.record().ok()?;
    record.get(esm::sig::DATA).map(|s| s.zstring())
}

/// A map marker's flags (`FNAM` after `XMRK`: 0x01 shown on the map from
/// the start, 0x02 can be travelled to); `None` if it isn't a map marker.
pub fn map_marker_flags(order: &LoadOrder, marker: FormId) -> Option<u8> {
    let record = order.get(marker)?.record().ok()?;
    let mut seen = false;
    for sub in &record.subrecords {
        if sub.kind == XMRK {
            seen = true;
        } else if seen && sub.kind == FNAM {
            return sub.data.first().copied();
        }
    }
    seen.then_some(0)
}

thread_local! {
    static DEPTH: std::cell::Cell<u8> = const { std::cell::Cell::new(0) };
    /// How deep `Activate … 1` runs of `OnActivate` blocks are inside each
    /// other: the game stops at 5 (a per-thread counter, `005b59f0`).
    static ACTIVATE_DEPTH: std::cell::Cell<u8> = const { std::cell::Cell::new(0) };
}

/// Runs `f` unless already this deep inside itself (a package whose
/// conditions ask which package is current); `None` then.
fn with_depth_guard<T>(f: impl FnOnce() -> T) -> Option<T> {
    let depth = DEPTH.with(|d| d.get());
    if depth >= 2 {
        return None;
    }
    DEPTH.with(|d| d.set(depth + 1));
    let out = f();
    DEPTH.with(|d| d.set(depth));
    Some(out)
}

/// How often a quest's script runs when the quest doesn't set its own
/// delay: `fQuestScriptDelayTime` in `[Main]` of the game's INI (5.0 in
/// this install's `Fallout.ini`).
pub const QUEST_SCRIPT_DELAY: f32 = 5.0;

/// Stages setting stages setting stages: deeper than this is cut off.
const MAX_DEPTH: u8 = 16;

/// What has happened in the game. A new game: every quest at stage 0, the
/// "start game enabled" ones running, globals at their records' values.
#[derive(Debug, Clone, Default)]
pub struct GameState {
    /// Optional presentation continuation in the custom save format. The
    /// viewer refreshes this from live state when saving; simulation does
    /// not use it as the player's physical position or aim.
    pub saved_camera: Option<crate::save::camera::Camera>,
    /// Each quest's current stage (the highest set so far).
    pub stages: HashMap<FormId, u16>,
    /// Every (quest, stage) set so far.
    pub stages_done: HashSet<(FormId, u16)>,
    pub running: HashSet<FormId>,
    pub completed: HashSet<FormId>,
    pub failed: HashSet<FormId>,
    /// Quests whose "Quest added" has been shown (the quest's flag 0x20,
    /// `world::quest_text`).
    pub quests_announced: HashSet<FormId>,
    pub globals: HashMap<FormId, f32>,
    /// Lines said (for "say once").
    pub said: HashSet<FormId>,
    /// Script variables by owner: a quest, or a reference with a script.
    pub variables: HashMap<FormId, Locals>,
    /// Objectives shown in the Pip-Boy: (quest, number) → completed.
    pub objectives: BTreeMap<(FormId, i32), bool>,
    /// References scripts enabled (false) or disabled (true).
    pub disabled: HashMap<FormId, bool>,
    /// Items held, (holder, item) → count, for the holders in `stocked`;
    /// the others hold what their base record lists ([`base_contents`]).
    pub items: HashMap<(FormId, FormId), i32>,
    /// Holders whose contents are kept in `items` (copied from their
    /// record the first time something changes them).
    pub stocked: HashSet<FormId>,
    /// The original game's inventory stacks of holders imported from a
    /// `.fos` save (`world::inventory_stacks`), kept through nv-rs's own
    /// saves. `items`, `equipped` and `weapon_health` were derived from
    /// them on import; gameplay changes those views, not these stacks.
    pub inventory_stacks: crate::inventory_stacks::SavedInventories,
    /// Scripted items' own scripts (the game's `ExtraScript` on each
    /// scripted item, `004821a0` adding them one at a time): holder, item,
    /// variables and the events waiting for the holder's next script run
    /// (`00565870` → `004d2480`). Not saved.
    pub item_scripts: Vec<ItemScript>,
    /// Topics the player has learned (`AddTopic`).
    pub topics: HashSet<FormId>,
    /// Seconds since each running quest's script last ran.
    pub quest_timers: HashMap<FormId, f32>,
    /// The cell the player is in, and its worldspace when outdoors.
    pub player_cell: Option<FormId>,
    pub player_world: Option<FormId>,
    /// Where the player's feet are, when known.
    pub player_position: Option<[f32; 3]>,
    /// Which way the player faces (radians clockwise from north), kept by
    /// whatever moves them; what [`Self::place`] gives for them.
    pub player_heading: f32,
    /// The player was put somewhere (`MoveTo`, fast travel:
    /// [`crate::script_functions::player_moved`]) and those coming along
    /// haven't been brought yet (`world::companions::come_along`; the
    /// viewer takes it). Not saved.
    pub player_placed: bool,
    /// The player's level (1 in a new game) and perks, with their ranks
    /// past the first (`world::perks`).
    pub player_level: u16,
    pub perks: HashSet<FormId>,
    pub perk_ranks: HashMap<FormId, u8>,
    /// A level-up is waiting for the player to be out of combat
    /// (`world::experience`).
    pub level_up_pending: bool,
    /// The player's companions (`SetPlayerTeammate`).
    pub teammates: HashSet<FormId>,
    /// Damage the player and teammates have done to each person, for the
    /// experience a kill gives (not saved: the game keeps it on the
    /// person's AI process).
    pub kill_share: HashMap<FormId, f64>,
    /// Locks the player has picked (experience only the first time).
    pub picked: HashSet<FormId>,
    /// Points put into each of the player's skills at level-ups, on top
    /// of what SPECIAL and tags give (`world::chargen::player_skill`).
    pub skill_points: BTreeMap<u16, u32>,
    /// The Pip-Boy's miscellaneous statistics by number (`world::stats`).
    pub misc_stats: BTreeMap<u8, u32>,
    /// Fame and infamy by reputation (`world::reputation`).
    pub reputations: BTreeMap<FormId, (f32, f32)>,
    /// Crime (`world::crime`): factions holding the player as an enemy
    /// for their crimes, each faction's (minor, major) crime counts and the
    /// player's, steal warnings given, and the day of the month of the last
    /// witnessed theft.
    pub crime_enemies: HashSet<FormId>,
    pub faction_crimes: BTreeMap<FormId, (u32, u32)>,
    pub player_crimes: (u32, u32),
    pub steal_warnings: u32,
    pub last_theft_day: Option<u32>,
    /// When the player hit each friend or ally lately, in [`Self::seconds`]
    /// (forgiven up to an allowance, `world::crime::assault`), and whether
    /// the player is a murderer (`IsPCAMurderer`).
    pub friendly_hits: HashMap<FormId, Vec<f64>>,
    pub player_murderer: bool,
    /// How the player is moving, as the viewer last saw (for detection).
    pub player_moving: bool,
    pub player_running: bool,
    pub player_sneaking: bool,
    /// Who is saying a line now, as the viewer last saw (`IsTalking`; not
    /// saved).
    pub speaking: HashSet<FormId>,
    /// Notes the player holds (`AddNote`).
    pub notes: HashSet<FormId>,
    /// People who have talked to the player.
    pub talked_to: HashSet<FormId>,
    /// Objects set "destroyed" (`SetDestroyed 1`): a door that won't open,
    /// a machine that's been used.
    pub destroyed: HashSet<FormId>,
    /// Packages scripts gave people (`AddScriptPackage`), followed before
    /// their own.
    pub script_packages: HashMap<FormId, FormId>,
    /// The package whose begin action was last requested for each person
    /// (`world::ai::actions::begin`), and whether its end action has been
    /// since (`world::ai::actions::end`): a package `AddScriptPackage`
    /// already began doesn't begin again when the AI takes it up, and a
    /// package ends once per start. Not saved.
    pub package_begun: HashMap<FormId, (FormId, bool)>,
    /// People whose packages are to be looked at again at once: scripts
    /// asked (`EvaluatePackage`, `ResetAI`) or changed them
    /// (`AddScriptPackage`, `RemoveScriptPackage`); packages are otherwise
    /// looked at every 20 s and each game hour (`world::movement::
    /// PackageClock`). Not saved.
    pub evaluate: HashSet<FormId>,
    /// Everyone's packages to be looked at once at their next update (as
    /// [`GameState::evaluate`] for each): set when the face menu closes.
    /// Seen in a recording of the original (2026-10-08): Doc Mitchell gets
    /// up from his chair the moment the opening's face menu closes, his
    /// standing package's `GetStage VCG01 >= 40` having become true while
    /// it was open; the executable's path for it isn't traced (not
    /// `SetStage` `0060d510`, `StartQuest` `0060c9c0`, the line's end
    /// `00935f60` nor the menu's finish `007ada40`). Not saved.
    pub evaluate_everyone: bool,
    /// Actor resets queued by ResetAI (005c9530 -> 008a6ce0(0, 1)).
    /// Unlike EvaluatePackage, the deferred reset releases furniture and
    /// discards the running procedure before choosing a package. Not saved.
    pub reset_ai: HashSet<FormId>,
    /// Where people who have moved are: position and heading (radians
    /// clockwise from north). Others are where they're placed.
    pub positions: HashMap<FormId, ([f32; 3], f32)>,
    /// The furniture someone sits in (or uses), by person: from when they
    /// head for it (`IsCurrentFurnitureRef`).
    pub furniture: HashMap<FormId, FormId>,
    /// How far each of them has got (`world::furniture`: the marker, the
    /// sit state `GetSitting` reports, the entry or exit playing). Not
    /// saved: someone loaded in furniture is seated there.
    pub sitters: HashMap<FormId, crate::furniture::Sitter>,
    /// Actor values scripts changed, (person, number) → value: what's set
    /// (`SetActorValue`, `ForceActorValue`) plus what's added
    /// (`ModActorValue`), over the value from their record.
    pub actor_values: HashMap<(FormId, u16), f64>,
    /// Who activated what's running (`IsActionRef`, `GetActionRef`).
    pub action_ref: Option<FormId>,
    /// Quest script delays scripts set (`SetQuestDelay`), seconds.
    pub quest_delays: HashMap<FormId, f32>,
    /// Where references were moved to (`MoveTo`): the interior cell or
    /// worldspace, and the cell (their position is in `positions`).
    pub spaces: HashMap<FormId, (FormId, FormId)>,
    /// People knocked out by scripts (`SetUnconscious 1`).
    pub unconscious: HashSet<FormId>,
    /// Knock states other than normal: knocked out by fatigue or
    /// paralysis (`world::fatigue`).
    pub knocks: crate::fatigue::Knocks,
    /// Flees `ForceFlee` started (`world::ai::flee::force`).
    pub forced_flee: HashMap<FormId, crate::ai::flee::ForcedFlee>,
    /// Who joined whose combat group (`world::combat_groups`).
    pub combat_groups: crate::combat_groups::CombatGroups,
    /// Sizes scripts set (`SetScale`).
    pub scales: HashMap<FormId, f32>,
    /// Objects Havok moved (shot, blown or pushed clutter: the game's
    /// "Havok moved" reference change, `CHANGE_REFR_HAVOK_MOVE`, named by
    /// `0083fef0`): where they came to rest, as their model's turn
    /// (row-major) and position in the world, which they keep when their
    /// place loads again.
    pub havok_moved: HashMap<FormId, ([[f32; 3]; 3], [f32; 3])>,
    /// Of those, the ones still moving: their linear and angular velocity
    /// (game units and radians a second). The game saves an active body's
    /// two velocities after its pose and sets them, waking it, on loading
    /// (`TESObjectREFR::SaveHavokDataForCollisionObject` (Xbox PDB),
    /// `00563220`, flag bit 1 and two 12-byte vectors; loaded by
    /// `00563380`).
    pub havok_velocity: HashMap<FormId, ([f32; 3], [f32; 3])>,
    /// Which weather it is, what's fading, the climate's pick, scripts'
    /// override, the player's weather region (`world::weather`).
    pub weather: crate::weather::WeatherState,
    /// Map markers scripts revealed (`ShowMap`).
    pub map_markers: HashSet<FormId>,
    /// Map markers scripts made fast-travel destinations (`ShowMap … 1`).
    pub map_marker_travel: HashSet<FormId>,
    /// Image space modifiers (`IMAD`) scripts applied, oldest first.
    pub modifiers: Vec<FormId>,
    /// The button the player pressed in the last message box (the box's
    /// callback `005b4a70` keeps it at `0118c684`), until its owner asks
    /// (`GetButtonPressed` gives it once, then -1).
    pub button: Option<i32>,
    /// Whose that button is (`011cac64`): set by the latest `ShowMessage`
    /// ([`Runner::message_owner`]); `GetButtonPressed` from anyone else
    /// gives -1 and leaves the button where it is (`005b4a80`).
    pub button_owner: Option<FormId>,
    /// The player's controls scripts turned off (`DisablePlayerControls`),
    /// by [`controls`] number.
    pub controls_off: [bool; 7],
    /// The player as made: name, sex (`SexChange`; else the record's), and
    /// tag skills (actor values).
    pub player_name: Option<String>,
    pub player_female: Option<bool>,
    pub tag_skills: BTreeSet<u16>,
    /// Health each person has lost (`world::combat`), and who's dead.
    pub damage: HashMap<FormId, f64>,
    pub dead: HashSet<FormId>,
    /// The last blow each person took: who struck it and how much damage
    /// (for how the dead fall; not saved).
    pub last_blow: HashMap<FormId, (FormId, f64)>,
    /// Deaths whose bodies haven't been handed to their ragdolls yet: how
    /// each began (`world::combat::DeathStart`). Taken by whoever drops the
    /// body; the dead at load have none and already lie where they fell
    /// (not saved).
    pub deaths: HashMap<FormId, crate::combat::DeathStart>,
    /// What other actor values have been damaged by, (person, number) →
    /// amount (`DamageActorValue`, harmful effects; restoring takes it
    /// back off). For rads and the hardcore needs it's the value itself.
    pub value_damage: HashMap<(FormId, u16), f64>,
    /// Effects working on people (`world::magic`), oldest first.
    pub active_effects: Vec<crate::magic::ActiveEffect>,
    /// Weapons' and armour's condition (0 to 1) where something changed
    /// it (scripts, wear, repairs), (holder, item); others are in full
    /// condition.
    pub weapon_health: HashMap<(FormId, FormId), f32>,
    /// The player's hot keys 1 to 8 (`InventoryChanges::SetHotKeyItem`
    /// (Xbox PDB), `004bf800`): the item on each.
    pub hotkeys: [Option<FormId>; 8],
    /// Weapons' fitted mod slots (`ExtraWeaponModFlags`: 1, 2, 4), (holder,
    /// weapon) (`world::weapon_mods`).
    pub weapon_mods: HashMap<(FormId, FormId), u8>,
    /// Weapons people have dropped, (holder, weapon): a crippled arm or a
    /// critical hit on the weapon (`world::body_parts::hurt_part`). They
    /// don't fight with them again.
    pub dropped: HashSet<(FormId, FormId)>,
    /// The body part each person's last hit landed on (`GetHitLocation`,
    /// a part type; -1 none), and the one the hit that killed them did
    /// (`GetKillingBlowLimb`). Not saved.
    pub hit_location: HashMap<FormId, i32>,
    pub killing_blow_limb: HashMap<FormId, i32>,
    /// The hits dealt since the animation last looked (who, the part, and
    /// whether it killed): the game asks the idle tree for a hit reaction
    /// while the hit's data is set (`0089a760`). Not saved.
    pub hits_taken: Vec<(FormId, i32, bool)>,
    /// The quest the Pip-Boy shows as active (`ForceActiveQuest`).
    pub active_quest: Option<FormId>,
    /// The player's own map marker, set on the Pip-Boy's map
    /// (`world::map::CustomMarker`; the player's `+0x6f4`).
    pub custom_marker: Option<crate::map::CustomMarker>,
    /// The radio (`world::radio`): stations, what plays, what's been
    /// found; its discovered list and on/tuned are saved.
    pub radio: crate::radio::Radio,
    /// The local map's fog of war (`SeenData` (Xbox PDB)), saved.
    pub seen: crate::local_map::Seen,
    /// The body part condition (actor value 25 .. 30) the Pip-Boy's STATS
    /// healing mode aims at while it's on (`StatsMenu` `+0x2a0` and its
    /// body part controller, `007e06b0`): effects added then are aimed at
    /// it ([`crate::magic::ActiveEffect::part`]). Not saved.
    pub healing_part: Option<u16>,
    /// Locks changed since they were placed (`world::locks`): `None`
    /// unlocked, `Some(level)` locked.
    pub locks: HashMap<FormId, Option<u8>>,
    /// Seconds the state has run for (every update adds its seconds):
    /// the clock doors' sequences are timed by (`world::doors`). Not saved.
    pub seconds: f64,
    /// Doors whose `Open` or `Close` sequence is playing (`world::doors`).
    /// Not saved: a loaded door is shown at its end.
    pub door_motion: HashMap<FormId, crate::doors::Motion>,
    /// How long door models' sequences play, by base record, filled by
    /// whoever loads the models (the viewer). Not saved.
    pub door_lengths: crate::doors::DoorLengths,
    /// Locks broken by failed forcing (`world::lockpick`: the lock's count
    /// at `+0xC`, `00790330`).
    pub broken_locks: HashMap<FormId, u32>,
    /// What placed terminals remember: hacked, lockouts
    /// (`world::terminal::TerminalState`).
    pub terminal_states: HashMap<FormId, crate::terminal::TerminalState>,
    /// Map markers the player has found (`world::map`).
    pub discovered: HashSet<FormId>,
    /// What people have equipped (`EquipItem`), by person.
    pub equipped: HashMap<FormId, Vec<FormId>>,
    /// Clothes and armour people other than the player wore from the start
    /// (`world::outfit`) that came off, by person: taken off
    /// (`UnequipItem`: `None`), or for something put on over the same body
    /// slots (`Some(that)`, `0088db20`).
    pub taken_off: HashMap<FormId, Vec<(FormId, Option<FormId>)>>,
    /// Things equipped with `EquipItem`'s no-unequip flag, (person, item):
    /// the worn entry's extra data 0x3e (`005d0060` → `0041ab70`), which
    /// keeps them in their slot when a person picks their armour again
    /// (`004c8220`).
    pub equip_locked: HashSet<(FormId, FormId)>,
    /// Who's fighting whom: attacker → target (`StartCombat`, being hit,
    /// an aggressive creature seeing the player).
    pub combat: HashMap<FormId, FormId>,
    /// Attackers someone already fighting takes on as further targets on
    /// being hurt by them (`world::combat_ai::attacked_by`: `0097f580` →
    /// `CombatController::AddTarget`, Xbox PDB), victim → attackers, until
    /// the viewer moves them into the fighter's target list. Not saved.
    pub hit_targets: HashMap<FormId, Vec<FormId>>,
    /// Whose weapon (or fists) is out: the game's `IsWeaponOut` flag
    /// (`process+0x135`, read by `00915d40`). Kept by the viewer for the
    /// player (`combat::player_attack`: a new game and every change of
    /// weapon start holstered; the Ready Item key and attacking draw it).
    /// Not saved.
    pub weapon_out: HashSet<FormId>,
    /// The ammunition someone chose with the Ammo Swap control (the
    /// process's current ammo, `SetCurrentAmmo` (Xbox PDB), `009462c0`;
    /// `world::ammo_swap`): holder → ammo.
    pub ammo_loaded: HashMap<FormId, FormId>,
    /// Who is blocking now (anim action 7, `00894d60`), with their heading
    /// (radians clockwise from north) for the hit cone (`world::melee`).
    /// Kept by the viewer for the player. Not saved.
    pub blocking: HashMap<FormId, f32>,
    /// Hits blocked since the viewer last looked (`009b5a30` flags the hit
    /// blocked, `00407e00(1, 1)`): the blocker, for the block-hit
    /// animation and the counter-attack timer. Not saved.
    pub blocked_hits: Vec<FormId>,
    /// Whose attack under way is a power attack (`00948310`; the hit's
    /// damage × `fDamagePowerAttackBonus`, `009b5170`). Not saved.
    pub power_attacking: HashSet<FormId>,
    /// People's weapon choice, clips and reloads in a fight, and whose
    /// `OnStartCombat` has run (`world::npc_combat`). Not saved.
    pub npc_combat: crate::npc_combat::State,
    /// The noise of each attacker's last attack, while it lasts
    /// (`world::noise`). Not saved.
    pub noise: HashMap<FormId, crate::noise::Noise>,
    /// Faction relations scripts changed (`SetEnemy`, `SetAlly`): (faction,
    /// other) → reaction (0 neutral, 1 enemy, 2 ally, 3 friend).
    pub faction_relations: HashMap<(FormId, FormId), u8>,
    /// Factions scripts put people in (`AddToFaction`, with the rank) or
    /// took them out of (`RemoveFromFaction`: rank -1).
    pub faction_changes: HashMap<(FormId, FormId), i8>,
    /// The skill a script tagged in each of the three slots
    /// (`SetPlayerTagSkill`, as the psych exam does), so a slot set again
    /// replaces its skill.
    pub tag_slots: BTreeMap<u8, u16>,
    /// V.A.T.S. now (`world::vats`): its mode and the attack being chosen
    /// or playing. Not saved.
    pub vats: crate::vats::VatsNow,
    /// What scripts set through the functions in
    /// [`crate::script_functions`] (restraints, owners, door states, form
    /// list additions, fast travel…).
    pub set_by_scripts: crate::script_functions::SetByScripts,
    /// Sleeping, hardcore mode's needs, pickpocketing and trespass warnings
    /// (`world::living`).
    pub living: crate::living::Living,
    /// What the second round of script functions keeps (ghosts, made
    /// references, challenges, damaged objects…; `world::more_functions`).
    pub more: crate::more_functions::State,
    /// The player's Caravan cards and record (`world::caravan`).
    pub caravan: crate::caravan::Collection,
    /// The player's casinos: the chips won at each and the level reached
    /// (`world::casino`, `PlayerCharacter` +0x610), head first.
    pub casinos: Vec<crate::casino::CasinoData>,
    /// The once-only tutorial messages: which have been shown, which are
    /// asked for (`world::tutorial`; only the shown ones are saved).
    pub tutorials: crate::tutorial::Tutorials,
    /// For the viewer: what to show or do, oldest first.
    pub events: Vec<Event>,
    /// Functions scripts called that aren't carried out yet, with counts,
    /// and the first call of each: whose script, on what, with what.
    pub unhandled: BTreeMap<&'static str, usize>,
    pub unhandled_first: BTreeMap<&'static str, String>,
    /// Script sources that didn't parse, by owner (for messages).
    pub broken: BTreeMap<FormId, String>,
    /// Advances every call to `GetRandomPercent` and every update.
    pub dice: u64,
}

impl GameState {
    /// Mouse looking is blocked independently by the control mask and the
    /// player's script package. In the supported package path, `005cc4f0`
    /// calls `005cc7a0(1)` (player+0x79b); `005cc7c0` clears it on removal.
    /// `0093a740` rejects input in `009445b0` while that flag is set, even
    /// after EnablePlayerControls. Observed set during the original opening.
    /// Deriving this from the saved package also preserves it across loading.
    pub fn player_looking_blocked(&self) -> bool {
        self.controls_off[controls::LOOKING] || self.script_packages.contains_key(&PLAYER_REF)
    }

    pub fn new(order: &LoadOrder) -> GameState {
        let mut state = GameState {
            dice: 0x9E37_79B9_7F4A_7C15,
            player_level: 1,
            living: crate::living::Living::new_game(),
            ..GameState::default()
        };
        for rr in order.records_of_type(QUST) {
            let Ok(record) = rr.record() else { continue };
            let flags = record
                .get(esm::sig::DATA)
                .and_then(|s| s.data.first().copied());
            if flags.is_some_and(|f| f & quest::START_GAME_ENABLED != 0) {
                state.running.insert(rr.form_id);
            }
        }
        for rr in order.records_of_type(GLOB) {
            let Ok(record) = rr.record() else { continue };
            if let Some(v) = record.get(FLTV).filter(|s| s.data.len() >= 4) {
                state.globals.insert(rr.form_id, le_f32(&v.data, 0));
            }
        }
        state.stock(order, PLAYER_REF);
        state
    }

    /// Copies a holder's record contents into `items`, once, picking from
    /// leveled lists for the player's level ([`crate::leveled`]).
    pub fn stock(&mut self, order: &LoadOrder, holder: FormId) {
        if !self.stocked.insert(holder) {
            return;
        }
        let level = self.player_level;
        for (form, n) in record_contents(order, holder) {
            let picked = crate::leveled::resolve(order, form, n, level, &mut || self.roll());
            for (item, count) in picked {
                if order
                    .get(item)
                    .is_some_and(|r| is_item(r.entry.header.kind))
                {
                    *self.items.entry((holder, item)).or_insert(0) += count;
                }
            }
        }
    }

    /// How many of an item a holder has.
    pub fn item_count(&self, order: &LoadOrder, holder: FormId, item: FormId) -> i32 {
        if self.stocked.contains(&holder) {
            return self.items.get(&(holder, item)).copied().unwrap_or(0);
        }
        base_contents(order, holder)
            .into_iter()
            .filter(|(i, _)| *i == item)
            .map(|(_, n)| n)
            .sum()
    }

    /// What a holder has: (item, count), sorted by item.
    pub fn inventory(&self, order: &LoadOrder, holder: FormId) -> Vec<(FormId, i32)> {
        let mut out: Vec<(FormId, i32)> = if self.stocked.contains(&holder) {
            self.items
                .iter()
                .filter(|((h, _), n)| *h == holder && **n > 0)
                .map(|((_, i), n)| (*i, *n))
                .collect()
        } else {
            base_contents(order, holder)
        };
        out.sort();
        out
    }

    /// Moves everything a holder has to another (taking all from a
    /// container): what moved.
    pub fn take_all(&mut self, order: &LoadOrder, from: FormId, to: FormId) -> Vec<(FormId, i32)> {
        self.stock(order, from);
        self.stock(order, to);
        let moved = self.inventory(order, from);
        for (item, n) in &moved {
            self.items.remove(&(from, *item));
            *self.items.entry((to, *item)).or_insert(0) += n;
            self.moved(order, from, to, *item, *n);
        }
        moved
    }

    /// Someone equips an item: a weapon puts away the one in hand; clothes
    /// and armour take off what's worn on any of the same body slots
    /// (`BMDT`, as the game's apparel does). A weapon or armour with no
    /// health left (worn out, or none at full: the master's "Broken" junk)
    /// isn't put on (`0088c830`); the player is told
    /// (`sCantEquipBrokenItem`).
    pub fn equip(&mut self, order: &LoadOrder, who: FormId, item: FormId) {
        if let Some(full) = crate::repair::stated_health(order, item) {
            let full = full as f32;
            let health = self
                .weapon_health
                .get(&(who, item))
                .map_or(full, |c| c * full);
            if health <= 0.0 {
                if who == PLAYER_REF {
                    let text =
                        game_setting_text(order, "sCantEquipBrokenItem").unwrap_or_else(|| {
                            "Broken items cannot be equipped until they have been repaired.".into()
                        });
                    self.events.push(Event::Message {
                        title: None,
                        text,
                        buttons: Vec::new(),
                        icon: crate::message_icon::for_setting("sCantEquipBrokenItem")
                            .map(str::to_string),
                    });
                }
                return;
            }
        }
        self.item_event(order, who, item, event::EQUIP);
        let kind = |f: FormId| order.get(f).map(|r| r.entry.header.kind);
        let item_kind = kind(item);
        let slots = crate::actor::Armor::load(order, item).map_or(0, |a| a.slots);
        // Someone else's clothes from the start on those slots come off
        // (`world::outfit`); one of them put on again is worn again.
        if who != PLAYER_REF && slots != 0 {
            let replaced = crate::outfit::replaced_by(order, who, item);
            let off = self.taken_off.entry(who).or_default();
            off.retain(|(i, _)| *i != item);
            for (piece, by) in replaced {
                if !off.iter().any(|(i, _)| *i == piece) {
                    off.push((piece, by));
                }
            }
            if off.is_empty() {
                self.taken_off.remove(&who);
            }
        }
        let worn = self.equipped.entry(who).or_default();
        worn.retain(|&f| {
            if f == item {
                return false;
            }
            match item_kind {
                Some(k) if k.as_bytes() == b"WEAP" => kind(f) != item_kind,
                Some(k) if k.as_bytes() == b"ARMO" => {
                    crate::actor::Armor::load(order, f).map_or(true, |a| a.slots & slots == 0)
                }
                _ => true,
            }
        });
        worn.push(item);
    }

    /// Someone takes an item off (or puts a weapon away).
    pub fn unequip(&mut self, who: FormId, item: FormId) {
        if let Some(worn) = self.equipped.get_mut(&who) {
            worn.retain(|&f| f != item);
        }
    }

    /// [`Self::unequip`], with the item's script told (`OnUnequip`).
    pub fn unequip_item(&mut self, order: &LoadOrder, who: FormId, item: FormId) {
        if self.is_equipped(who, item) {
            self.item_event(order, who, item, event::UNEQUIP);
        }
        self.unequip(who, item);
        // Clothes someone else wore from the start come off too
        // (`world::outfit`).
        if who != PLAYER_REF && crate::outfit::worn_from_start(order, who, item) {
            let off = self.taken_off.entry(who).or_default();
            off.retain(|(i, _)| *i != item);
            off.push((item, None));
        }
    }

    /// Whether someone has an item equipped.
    pub fn is_equipped(&self, who: FormId, item: FormId) -> bool {
        self.equipped.get(&who).is_some_and(|w| w.contains(&item))
    }

    /// Items added to a holder at runtime (an `AddItem`, a pick-up…; not
    /// a holder's own contents): each one with a script gets a script of
    /// its own (`004821a0`) and its `OnAdd` for the holder (`00574fa0`).
    pub fn added(&mut self, order: &LoadOrder, holder: FormId, item: FormId, count: i32) {
        if count > 0 && item_script(order, item).is_some() {
            for _ in 0..count {
                self.item_scripts.push(ItemScript {
                    holder,
                    item,
                    locals: None,
                    events: vec![(event::ADD, holder)],
                });
            }
        }
    }

    /// Items moved between holders: their scripts go with them, each
    /// seeing `OnDrop` for the one it left (the giver's `RemoveItem` flags
    /// it, `005750a0` → `005ac750(…, 4)`) and `OnAdd` for its new holder;
    /// those without one yet (a holder's own contents) get one.
    fn moved(&mut self, order: &LoadOrder, from: FormId, to: FormId, item: FormId, count: i32) {
        if count <= 0 || item_script(order, item).is_none() {
            return;
        }
        let mut left = count;
        for s in self.item_scripts.iter_mut() {
            if left == 0 {
                break;
            }
            if s.holder == from && s.item == item {
                s.holder = to;
                s.events.push((event::DROP, from));
                s.events.push((event::ADD, to));
                left -= 1;
            }
        }
        for _ in 0..left {
            self.item_scripts.push(ItemScript {
                holder: to,
                item,
                locals: None,
                events: vec![(event::DROP, from), (event::ADD, to)],
            });
        }
    }

    /// Dropped things' scripts go with them into the world (`004c6dd0`
    /// keeps the extra data on the new reference): `count` of the item's
    /// scripts move from `from` to the made reference `to`, each seeing
    /// `OnDrop` for `from` (`005ac750(…, 4)`); those without one yet get one.
    pub fn scripts_follow(
        &mut self,
        order: &LoadOrder,
        from: FormId,
        to: FormId,
        item: FormId,
        count: i32,
    ) {
        if count <= 0 || item_script(order, item).is_none() {
            return;
        }
        let mut left = count;
        for s in self.item_scripts.iter_mut() {
            if left == 0 {
                break;
            }
            if s.holder == from && s.item == item {
                s.holder = to;
                s.events.push((event::DROP, from));
                left -= 1;
            }
        }
        for _ in 0..left {
            self.item_scripts.push(ItemScript {
                holder: to,
                item,
                locals: None,
                events: vec![(event::DROP, from)],
            });
        }
    }

    /// Someone drops things into the world with their scripts
    /// ([`crate::more_functions::placed::drop_into_world`],
    /// [`Self::scripts_follow`]): the new reference.
    pub fn drop_item(
        &mut self,
        order: &LoadOrder,
        holder: FormId,
        item: FormId,
        count: i32,
    ) -> Option<FormId> {
        let made =
            crate::more_functions::placed::drop_into_world(order, self, holder, item, count)?;
        let n = crate::more_functions::placed::held_in_world(order, self, made, item);
        self.scripts_follow(order, holder, made, item, n);
        Some(made)
    }

    /// An event for one of a holder's scripted items (`005ac750`): equipped
    /// (2), unequipped (8), with the reference it's for.
    fn item_event(&mut self, order: &LoadOrder, holder: FormId, item: FormId, mask: u32) {
        if item_script(order, item).is_none() {
            return;
        }
        match self
            .item_scripts
            .iter_mut()
            .find(|s| s.holder == holder && s.item == item)
        {
            Some(s) => s.events.push((mask, holder)),
            None => self.item_scripts.push(ItemScript {
                holder,
                item,
                locals: None,
                events: vec![(mask, holder)],
            }),
        }
    }

    /// Moves up to `count` of an item from one holder to another (as the
    /// container screen does); how many moved.
    pub fn move_item(
        &mut self,
        order: &LoadOrder,
        from: FormId,
        to: FormId,
        item: FormId,
        count: i32,
    ) -> i32 {
        self.stock(order, from);
        self.stock(order, to);
        let have = self.items.get(&(from, item)).copied().unwrap_or(0);
        let n = count.min(have).max(0);
        if n == 0 {
            return 0;
        }
        if have == n {
            self.items.remove(&(from, item));
        } else {
            self.items.insert((from, item), have - n);
        }
        *self.items.entry((to, item)).or_insert(0) += n;
        self.moved(order, from, to, item, n);
        n
    }

    /// What a holder's items weigh together (`InventoryWeight`; the
    /// game's `004d0900`): each item's weight as [`crate::items::weight`]
    /// counts it (ammunition only in hardcore mode), a weapon of 10 or more
    /// through the player's perks' "Adjust Heavy Weapon Weight" (entry
    /// point 73; no perk in the data uses it), and when the holder's perks'
    /// "Modify Light Items" (55, Pack Rat) give more than 0, items of at
    /// most `fPackRatThreshold` (2) × `fPackRatModifier` (0.5; the exe's
    /// defaults) — then × how many they carry.
    pub fn inventory_weight(&self, order: &LoadOrder, holder: FormId) -> f32 {
        let setting = |n: &str, d: f32| game_setting(order, n).unwrap_or(d);
        let light = crate::perks::apply_for(
            order,
            self,
            holder,
            crate::perks::entry::MODIFY_LIGHT_ITEMS,
            0.0,
            &[],
        ) > 0.0;
        let threshold = setting("fPackRatThreshold", 2.0);
        let modifier = setting("fPackRatModifier", 0.5);
        self.inventory(order, holder)
            .into_iter()
            .map(|(item, n)| {
                let mut weight = crate::items::weight(order, item, self.living.hardcore).max(0.0);
                let is_weapon = order
                    .get(item)
                    .is_some_and(|rr| rr.entry.header.kind == WEAP);
                // A weight mod's share off (`004be380`).
                if is_weapon {
                    weight -= crate::weapon_mods::weight_off(order, self, holder, item);
                }
                if is_weapon && weight >= 10.0 {
                    weight *= crate::perks::apply(
                        order,
                        self,
                        crate::perks::entry::ADJUST_HEAVY_WEAPON_WEIGHT,
                        1.0,
                    );
                }
                if light && weight <= threshold {
                    weight *= modifier;
                }
                weight * n as f32
            })
            .sum()
    }

    /// Whether a holder carries more than their carry weight (the player's
    /// `00954cc0`: inventory weight above Carry Weight, actor value 13,
    /// after the perks' "Get Max Carry Weight"). Not when the carry weight
    /// can't be worked out (a plugin without its settings).
    pub fn over_encumbered(&self, order: &LoadOrder, holder: FormId) -> bool {
        let most = Facts {
            order,
            state: self,
            speaker: None,
        }
        .current_actor_value(holder, 13);
        most.is_some_and(|most| (most as f32) < self.inventory_weight(order, holder))
    }

    /// Someone sits in (or uses) a piece of furniture at once (a script's
    /// `Activate`, a loaded game): they're seated there.
    pub fn sit(&mut self, who: FormId, furniture: FormId) {
        self.furniture.insert(who, furniture);
        self.sitters.remove(&who);
    }

    /// Someone is up at once, out of any furniture (through a door, into a
    /// fight). Getting up the game's way is
    /// [`crate::furniture::Sitter::stand_up`].
    pub fn stand(&mut self, who: FormId) {
        self.furniture.remove(&who);
        self.sitters.remove(&who);
    }

    /// Consume a loaded actor's deferred AI reset. Furniture is released at
    /// once (008a6ce0 -> 0088d640 -> process +0x84 / 009287e0), without an
    /// exit animation or moving the actor to a guessed marker position.
    /// The presentation owner must also discard its current procedure/path.
    pub fn take_ai_reset(&mut self, who: FormId) -> bool {
        if !self.reset_ai.remove(&who) {
            return false;
        }
        self.stand(who);
        self.evaluate.insert(who);
        true
    }

    /// Whether a reference is a piece of furniture (`FURN`: chairs, beds,
    /// benches).
    pub fn is_furniture(order: &LoadOrder, reference: FormId) -> bool {
        base_of(order, reference)
            .and_then(|b| order.get(b))
            .is_some_and(|r| r.entry.header.kind == FURN)
    }

    /// The player picks up an item lying in the world: it goes into their
    /// inventory (with its script, when one dropped it there) and the
    /// placed one is gone.
    pub fn pick_up(&mut self, order: &LoadOrder, reference: FormId, item: FormId, count: i32) {
        self.stock(order, PLAYER_REF);
        *self.items.entry((PLAYER_REF, item)).or_insert(0) += count;
        let carried = self
            .item_scripts
            .iter()
            .filter(|s| s.holder == reference && s.item == item)
            .count() as i32;
        let mut left = count;
        for s in self.item_scripts.iter_mut() {
            if left == 0 {
                break;
            }
            if s.holder == reference && s.item == item {
                s.holder = PLAYER_REF;
                s.events.push((event::ADD, PLAYER_REF));
                left -= 1;
            }
        }
        self.added(order, PLAYER_REF, item, count - carried.min(count));
        self.disabled.insert(reference, true);
        self.events.push(Event::Enable(reference, false));
    }

    /// A global variable by editor ID.
    pub fn global(&self, order: &LoadOrder, name: &str) -> Option<f32> {
        self.globals.get(&order.form_by_editor_id(name)?).copied()
    }

    fn set_global(&mut self, order: &LoadOrder, name: &str, value: f32) {
        if let Some(id) = order.form_by_editor_id(name) {
            self.globals.insert(id, value);
        }
    }

    /// Game time passes: `GameHour` moves on at `TimeScale` game seconds
    /// per real second (30 in a new game), and past midnight the day,
    /// month and year roll over (Gregorian month lengths, `GameMonth`
    /// counting from 0: a guess at the game's calendar). `GameDaysPassed`
    /// grows continuously by the hours ÷ 24 (`00867a40` adds them at every
    /// step; hardcore's needs measure time by it).
    pub fn advance_clock(&mut self, order: &LoadOrder, seconds: f32) {
        let scale = self.global(order, "TimeScale").unwrap_or(30.0);
        let Some(mut hour) = self.global(order, "GameHour") else {
            return;
        };
        let hours = seconds * scale / 3600.0;
        hour += hours;
        if let Some(days) = self.global(order, "GameDaysPassed") {
            // Worked out wider and kept as a float, as the game's x87 code
            // does (hardcore's needs count whole points of it).
            let days = (f64::from(hours) / 24.0 + f64::from(days)) as f32;
            self.set_global(order, "GameDaysPassed", days);
        }
        while hour >= 24.0 {
            hour -= 24.0;
            let year = self.global(order, "GameYear").unwrap_or(2281.0) as i32;
            let month = self.global(order, "GameMonth").unwrap_or(0.0) as i32;
            let day = self.global(order, "GameDay").unwrap_or(1.0) as i32 + 1;
            if day > days_in_month(year, month) {
                self.set_global(order, "GameDay", 1.0);
                if month >= 11 {
                    self.set_global(order, "GameMonth", 0.0);
                    self.set_global(order, "GameYear", (year + 1) as f32);
                } else {
                    self.set_global(order, "GameMonth", (month + 1) as f32);
                }
            } else {
                self.set_global(order, "GameDay", day as f32);
            }
        }
        self.set_global(order, "GameHour", hour);
    }

    /// Whether an enemy is near the player (`009764a0`, which also stops
    /// sleeping and fast travel): someone alive and enabled within
    /// `fHostileActorInteriorDistance` (2000) indoors or
    /// `fHostileActorExteriorDistance` (3000) outdoors who fights the
    /// player or would attack them on sight. The game looks at every
    /// loaded person; here, those fighting the player and those in the
    /// player's own cell or square (a simplification), measured in 3D (a
    /// guess). With the player's place unknown, anyone fighting them counts.
    pub fn enemies_near(&self, order: &LoadOrder) -> bool {
        let fighting = |who: FormId| self.combat.get(&who) == Some(&PLAYER_REF);
        let mut candidates: Vec<FormId> = self
            .combat
            .iter()
            .filter(|(_, t)| **t == PLAYER_REF)
            .map(|(a, _)| *a)
            .collect();
        let Some(feet) = self.player_position else {
            return !candidates.is_empty();
        };
        if let Some(cell) = self.player_cell {
            candidates.extend(
                order
                    .references_in_cell(cell)
                    .into_iter()
                    .filter(|rr| matches!(rr.entry.header.kind.as_bytes(), b"ACHR" | b"ACRE"))
                    .map(|rr| rr.form_id),
            );
        }
        let indoors = self.player_world.is_none();
        let reach = if indoors {
            game_setting(order, "fHostileActorInteriorDistance").unwrap_or(2000.0)
        } else {
            game_setting(order, "fHostileActorExteriorDistance").unwrap_or(3000.0)
        };
        let space = self.player_world.or(self.player_cell);
        candidates.into_iter().any(|who| {
            if self.dead.contains(&who) || self.disabled.get(&who) == Some(&true) {
                return false;
            }
            let Some((s, _, p, _)) = self.place(order, who) else {
                return false;
            };
            let d = (0..3).map(|i| (p[i] - feet[i]).powi(2)).sum::<f32>().sqrt();
            Some(s) == space
                && d <= reach
                && (fighting(who)
                    || crate::factions::attacks_on_sight(order, self, who, PLAYER_REF))
        })
    }

    /// The Pip-Boy's wait (T): `hours` (whole ones) pass at once, each as
    /// the game's wait menu passes it (`world::living::sleep`): unless the
    /// game refuses (trespassing, an enemy near ([`Self::enemies_near`]),
    /// a place or script that forbids it, …: its own words are given back).
    /// Each hour heals the player Heal Rate × the hour's real seconds (3600
    /// ÷ `TimeScale`) ÷ 3600 (`0094df80`): with Endurance 9 or 10, 10 ÷ 30 a
    /// game hour; none at 5 or less; in hardcore the needs grow.
    pub fn wait(&mut self, order: &LoadOrder, hours: f32) -> Result<(), String> {
        let whole = hours.max(0.0) as u32;
        crate::living::sleep::wait_all(order, self, whole, false)
    }
    /// The game's dice: a new random number (every caller draws from the
    /// same sequence, so a run repeats).
    pub fn roll(&mut self) -> u64 {
        // xorshift64
        let mut x = self.dice.max(1);
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.dice = x;
        x
    }
}

/// Something for the player to see happen.
#[derive(Debug, Clone, PartialEq)]
pub enum Event {
    /// `ShowMessage`: the message's title (if any) and text, and for a
    /// message box (`DNAM` 0x01) its buttons (`ITXT`) whose conditions
    /// pass, each with its number (what `GetButtonPressed` gives).
    Message {
        title: Option<String>,
        text: String,
        buttons: Vec<(usize, String)>,
        /// The picture beside a corner message (`world::message_icon`;
        /// `None`: the neutral Vault Boy).
        icon: Option<String>,
    },
    /// A box the game itself puts up (not a script's: its answer goes to
    /// no script), with one button `sOk`: a reputation's new title
    /// (`world::reputation::title_box`). The icon is a path under `Data`,
    /// the sound a sound's editor ID.
    Popup {
        title: Option<String>,
        text: String,
        icon: Option<String>,
        sound: Option<String>,
    },
    /// Someone talks: the speaker, who they talk to, and the topic
    /// (`None`: their greeting). `SayTo` only has them say a line
    /// (`conversation` false); `StartConversation` opens the dialogue.
    Talk {
        speaker: FormId,
        to: FormId,
        topic: Option<FormId>,
        conversation: bool,
    },
    /// A reference was enabled (true) or disabled (false).
    Enable(FormId, bool),
    /// The player's controls were enabled (true) or disabled.
    PlayerControls(bool),
    /// `MoveTo`: something moved to where another thing is.
    MoveTo {
        what: FormId,
        to: FormId,
    },
    Sound(FormId),
    /// `Activate`: something activated (by someone).
    Activate {
        what: FormId,
        by: Option<FormId>,
    },
    /// A stage's journal text.
    Journal {
        quest: FormId,
        text: String,
    },
    /// An objective shown (completed: false) or completed.
    Objective {
        quest: FormId,
        text: String,
        completed: bool,
    },
    /// The HUD's quest text: a quest added, completed or failed, or custom
    /// text such as a place discovered (`world::quest_text`).
    QuestText(crate::quest_text::QuestText),
    /// A script opened one of the game's menus (by the number `MenuMode`
    /// blocks use).
    Menu(u16),
    /// An image space modifier (`IMAD`) applied (true) or removed.
    ImageSpace(FormId, bool),
    /// `PlayBink`: a movie to play before anything else happens.
    Video(Video),
    /// `PlayMusic`: a music type (`MUSC`).
    Music(FormId),
    /// The weather forced (`ForceWeather`) or released (`None`).
    Weather(Option<FormId>),
    /// A menu for making the character.
    CharacterMenu(crate::chargen::CharacterMenu),
    /// `ShowBarterMenu`: trading with this merchant.
    Barter(FormId),
    /// `ShowRecipeMenu`: the crafting menu of a recipe category
    /// (`world::crafting`) for the reference it was called on
    /// (`005deb10` opens it, `00726ff0`, only for an actor: "Recipe menu
    /// called with NULL vendor!" otherwise).
    RecipeMenu {
        actor: FormId,
        category: FormId,
    },
    /// `ShowRepairMenu`: this merchant's repairs (`world::repair`).
    RepairServices(FormId),
    /// `ShowTutorialMenu` (`005da630`): the tutorial menu with this help
    /// message (0 when none was given), not through the tutorial manager.
    TutorialMenu(FormId),
    /// `OpenTeammateContainer`: trading things with a companion (the
    /// container menu's mode 3).
    TeammateContainer(FormId),
    /// The companion wheel's Back Up (`00756930` → `008a7760`,
    /// `Actor::InitiateBackUpPackage` (Xbox PDB)): the companion is given
    /// their default package 0x27 (a `BackUpPackage`, away from the
    /// player). Packages are the AI's to carry out.
    BackUp(FormId),
    /// `ShowCaravanMenu`: a game of Caravan against this person with their
    /// deck, the AI's difficulty and the share of their funds they bet
    /// (`world::caravan`).
    Caravan {
        npc: FormId,
        deck: FormId,
        difficulty: i32,
        share: f32,
    },
    /// `ShowSlotMachineMenuParams`, `ShowBlackJackMenuParams`,
    /// `ShowRouletteMenuParams`: a casino game (`world::casino`), its bets'
    /// limits and the least winnings to sit down (0 for none; blackjack's
    /// handler always passes 0).
    Casino {
        game: crate::casino::Game,
        casino: FormId,
        min_bet: i32,
        max_bet: i32,
        min_winnings: i32,
    },
    /// Someone died (killed by `by`).
    Died {
        who: FormId,
        by: FormId,
    },
    /// Someone essential was brought to 0 health: they go down instead of
    /// dying (`world::combat::hurt`); or someone is knocked out by fatigue
    /// below 0 or paralysis (`world::fatigue`).
    KnockedOut {
        who: FormId,
    },
    /// Someone who was down gets up (`world::combat::advance_down`,
    /// `world::fatigue`).
    GotUp {
        who: FormId,
    },
    /// `ForceFlee` started the engine's flee package on someone
    /// (`world::ai::flee::force`), to a reference or cell if given.
    Flees {
        who: FormId,
        to: Option<FormId>,
    },
    /// `PlayGroup`: an animation group (by the game's name: `Forward`,
    /// `Open`, `SpecialIdle`…) to play on a person or an object's model,
    /// with the script's flags (`005c0df0`: not 0 blends it in over flags ×
    /// 0.1 s on objects' controllers).
    PlayGroup {
        who: FormId,
        group: String,
        flags: i32,
    },
    /// `PlayIdle`: an idle (`IDLE`) for a person to play now.
    PlayIdle {
        who: FormId,
        idle: FormId,
    },
    /// An AI package lifecycle action requested by `AddScriptPackage` or
    /// the AI (`world::ai::actions`). The referenced `PACK` record holds
    /// the action's idle, topic and embedded script
    /// (`world::ai::actions::perform` carries it out).
    PackageAction {
        who: FormId,
        package: FormId,
        kind: PackageActionKind,
    },
    /// `SwapTextureOnRef`: a node of a reference's model takes another
    /// texture (a path under `Data`), while it's loaded.
    SwapTexture {
        what: FormId,
        node: String,
        texture: String,
    },
    /// `ShowSleepWaitMenu`: the sleep/wait menu opens, in sleep mode or
    /// not (`world::living::sleep`).
    SleepWaitMenu {
        sleep: bool,
    },
    /// What `world::more_functions` has to show (a reference made, an
    /// actor's alpha, a save asked for…).
    More(crate::more_functions::Shown),
    /// `ForceTerminalBack` (`005dc4e0`): an open terminal menu goes back a
    /// screen (out of the first, it closes).
    TerminalBack,
}

/// Which package lifecycle action is requested (`world::ai::actions`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PackageActionKind {
    Begin,
    Change,
    End,
}

/// The face and body menu (`ShowRaceMenu`): `VCG01SCRIPT` opens it at
/// stage 36 and carries on from a `MenuMode 1036` block.
pub const RACE_SEX_MENU: u16 = 1036;

/// What `PlayBink` asks for. The command's four optional integers have no
/// names in the PC program; their effects were read from its handler
/// (`005d15d0`) and the movie player it calls (1.4.0.525):
///
/// - the first goes to the player's per-frame continue check
///   (vtable `01082564` + 0x40, `00867440`), which ends the movie when control
///   5 or 28 is pressed only if it is set;
/// - the second brackets the movie with the player's audio mute and unmute
///   (+0x4 `008671a0`, +0x8 `00867220`);
/// - the third with its music pause and resume (+0xc, +0x10);
/// - the fourth picks the fit (`00ec2aa0`, `00ec2b20`, `00ec2bb0`): set, the
///   movie fills the screen's width and is centred vertically; clear, it
///   fills the height and is centred horizontally.
///
/// The handler's defaults are 0, 1, 1 and 1. The Xbox 360 prototype's
/// symbols (Xbox PDB) name the handler's locals `iinterruptable`,
/// `imuteGameAudio`, `ipauseGameMusic` and `iletterBoxed`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Video {
    /// The file, relative to `Data\Video`.
    pub file: String,
    pub interruptable: bool,
    pub mute_audio: bool,
    pub pause_music: bool,
    pub letterbox: bool,
}

/// A function argument once worked out.
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Form(FormId),
    Number(f64),
    Text(String),
}

impl Value {
    pub fn form(&self) -> FormId {
        match self {
            Value::Form(f) => *f,
            Value::Number(n) => FormId(*n as u32),
            Value::Text(_) => FormId(0),
        }
    }

    pub fn number(&self) -> f64 {
        match self {
            Value::Number(n) => *n,
            Value::Form(f) => f64::from(f.0),
            Value::Text(_) => 0.0,
        }
    }
}

fn days_in_month(year: i32, month: i32) -> i32 {
    match month {
        1 if (year % 4 == 0 && year % 100 != 0) || year % 400 == 0 => 29,
        1 => 28,
        3 | 5 | 8 | 10 => 30,
        _ => 31,
    }
}

/// The day of the week (0 Sunday) of a Gregorian date, `month` from 0.
pub(crate) fn day_of_week(year: i32, month: i32, day: i32) -> i32 {
    const T: [i32; 12] = [0, 3, 2, 5, 0, 3, 5, 1, 4, 6, 2, 4];
    let y = if month < 2 { year - 1 } else { year };
    (y + y / 4 - y / 100 + y / 400 + T[month.clamp(0, 11) as usize] + day).rem_euclid(7)
}

fn flag(b: bool) -> f64 {
    if b {
        1.0
    } else {
        0.0
    }
}

/// The base record a reference places (the player's is `PLAYER_BASE`).
pub fn base_of(order: &LoadOrder, reference: FormId) -> Option<FormId> {
    if reference == PLAYER_REF {
        return Some(PLAYER_BASE);
    }
    let rr = order.get(reference)?;
    // Its `NAME` alone: asked for very often.
    let name = rr.subrecord(NAME).ok()??;
    if name.len() < 4 {
        return None;
    }
    Some(rr.plugin.to_global(FormId(le_u32(&name, 0))))
}

/// Every `CNTO` (form, count) on the base of a container, NPC or creature
/// (the player's on `PLAYER_BASE`): items and leveled lists.
pub(crate) fn record_contents(order: &LoadOrder, holder: FormId) -> Vec<(FormId, i32)> {
    let Some(base) = base_of(order, holder) else {
        return Vec::new();
    };
    let Some(rr) = order.get(base) else {
        return Vec::new();
    };
    let Ok(record) = rr.record() else {
        return Vec::new();
    };
    record
        .get_all(CNTO)
        .filter(|s| s.data.len() >= 8)
        .map(|s| {
            (
                rr.plugin.to_global(FormId(le_u32(&s.data, 0))),
                le_u32(&s.data, 4) as i32,
            )
        })
        .filter(|(_, n)| *n > 0)
        .collect()
}

/// The items a holder's record lists directly (leveled lists left out:
/// they're picked when the holder's contents are first copied into the
/// state, [`GameState::stock`]).
pub fn base_contents(order: &LoadOrder, holder: FormId) -> Vec<(FormId, i32)> {
    record_contents(order, holder)
        .into_iter()
        .filter(|(item, _)| {
            order
                .get(*item)
                .is_some_and(|r| is_item(r.entry.header.kind))
        })
        .collect()
}

/// Where a placed reference stands, as placed in its record.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Whereabouts {
    pub cell: FormId,
    /// Outdoors: its worldspace.
    pub world: Option<FormId>,
    pub position: [f32; 3],
    /// Its z rotation: the compass heading, radians clockwise from north.
    pub heading: f32,
}

pub fn whereabouts(order: &LoadOrder, reference: FormId) -> Option<Whereabouts> {
    let rr = order.get(reference)?;
    // Only its `DATA` is read (this is asked for many references every
    // frame).
    let data = rr.subrecord(esm::sig::DATA).ok()??;
    let data = &data[..];
    if data.len() < 24 {
        return None;
    }
    Some(Whereabouts {
        cell: order.cell_of(&rr)?,
        world: order.world_of(&rr),
        position: [le_f32(data, 0), le_f32(data, 4), le_f32(data, 8)],
        heading: le_f32(data, 20),
    })
}

impl GameState {
    /// Where a reference is now: (the interior cell or worldspace, the
    /// cell, position, heading). The player's from the state (heading not
    /// kept: 0); others where a script moved them, else as placed.
    pub fn place(&self, order: &LoadOrder, r: FormId) -> Option<(FormId, FormId, [f32; 3], f32)> {
        if r == PLAYER_REF {
            let space = self.player_world.or(self.player_cell)?;
            let cell = self.player_cell.unwrap_or(space);
            return Some((space, cell, self.player_position?, self.player_heading));
        }
        // Made while playing (`PlaceAtMe`): where it was made.
        let w = match self.more.placed.refs.get(&r) {
            Some(m) => Whereabouts {
                cell: m.cell,
                world: (m.space != m.cell).then_some(m.space),
                position: m.position,
                heading: m.rotation[2],
            },
            None => whereabouts(order, r)?,
        };
        let (position, heading) = self
            .positions
            .get(&r)
            .copied()
            .unwrap_or((w.position, w.heading));
        let (space, cell) = self
            .spaces
            .get(&r)
            .copied()
            .unwrap_or((w.world.unwrap_or(w.cell), w.cell));
        Some((space, cell, position, heading))
    }
}

/// Record types that are items: they can be picked up and carried.
pub const ITEM_TYPES: [&[u8; 4]; 13] = [
    b"ALCH", b"AMMO", b"ARMO", b"BOOK", b"CCRD", b"CHIP", b"CMNY", b"IMOD", b"INGR", b"KEYM",
    b"MISC", b"NOTE", b"WEAP",
];

pub fn is_item(kind: FourCC) -> bool {
    ITEM_TYPES.iter().any(|k| kind.as_bytes() == *k)
}

/// A placed object the player can do something with, in a cell: one whose
/// base has a script (people, activators, triggers, doors…), an item lying
/// around, a container, or furniture.
#[derive(Debug, Clone, PartialEq)]
pub struct Interactive {
    pub reference: FormId,
    pub base: FormId,
    pub script: Option<FormId>,
    /// For an item: how many (`XCNT`, else 1).
    pub count: i32,
    pub position: [f32; 3],
    /// Radians, as stored (`DATA`).
    pub rotation: [f32; 3],
    pub scale: f32,
    /// Its trigger volume, for triggers (activators with a primitive,
    /// `XPRM`).
    pub trigger: Option<TriggerBox>,
    /// The base's bounds (`OBND`: lowest and highest corner in its own
    /// axes), for aiming at it.
    pub bounds: Option<([f32; 3], [f32; 3])>,
    /// The base's name (`FULL`).
    pub name: Option<String>,
    /// The base's record type (`ACTI`, `NPC_`, `DOOR`, …).
    pub kind: FourCC,
}

/// A trigger's volume (`XPRM`: three sizes, a colour for the editor, a
/// float, and the shape: 1 box, 2 sphere, 3 portal box). The sizes are
/// taken as half the box's width, depth and height around the
/// reference's position, turned by its rotation (a guess at their
/// meaning), times its scale (`XSCL`).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TriggerBox {
    pub half: [f32; 3],
    pub shape: u32,
}

impl Interactive {
    pub fn is_item(&self) -> bool {
        is_item(self.kind)
    }

    pub fn is_container(&self) -> bool {
        self.kind == CONT
    }

    pub fn is_furniture(&self) -> bool {
        self.kind == FURN
    }

    /// A computer terminal (`world::terminal`).
    pub fn is_terminal(&self) -> bool {
        self.kind == TERM
    }

    /// A world point or direction in its own axes (unturned, unscaled).
    fn local(&self, v: [f32; 3], point: bool) -> [f32; 3] {
        let d = if point {
            [
                v[0] - self.position[0],
                v[1] - self.position[1],
                v[2] - self.position[2],
            ]
        } else {
            v
        };
        // The inverse of the placed rotation is its transpose.
        let m = crate::RotationConvention::DEFAULT.matrix(self.rotation);
        [0, 1, 2].map(|i| m[0][i] * d[0] + m[1][i] * d[1] + m[2][i] * d[2])
    }

    /// Whether a point is inside its trigger volume.
    pub fn contains(&self, p: [f32; 3]) -> bool {
        let Some(t) = self.trigger else {
            return false;
        };
        let local = self.local(p, true);
        if t.shape == 2 {
            let r = t.half[0];
            return local.iter().map(|v| v * v).sum::<f32>() <= r * r;
        }
        (0..3).all(|i| local[i].abs() <= t.half[i])
    }

    /// How far along a ray (from `eye`, unit `dir`) it meets the object's
    /// bounds, if it does.
    pub fn ray_hit(&self, eye: [f32; 3], dir: [f32; 3]) -> Option<f32> {
        self.ray_hit_within(eye, dir, 0.0)
    }

    /// [`Self::ray_hit`] with the bounds grown by `margin` world units on
    /// every side: where a sphere of that radius cast along the ray first
    /// touches them (the corners rounded off aside), as the activation
    /// pick's sphere does (`world::activation::PICK_RADIUS`).
    pub fn ray_hit_within(&self, eye: [f32; 3], dir: [f32; 3], margin: f32) -> Option<f32> {
        let (lo, hi) = self.bounds?;
        let s = if self.scale > 0.0 { self.scale } else { 1.0 };
        let (lo, hi) = (lo.map(|v| v - margin / s), hi.map(|v| v + margin / s));
        let o = self.local(eye, true).map(|v| v / s);
        let d = self.local(dir, false).map(|v| v / s);
        let (mut near, mut far) = (0.0f32, f32::INFINITY);
        for i in 0..3 {
            if d[i].abs() < 1e-9 {
                if o[i] < lo[i] || o[i] > hi[i] {
                    return None;
                }
                continue;
            }
            let (a, b) = ((lo[i] - o[i]) / d[i], (hi[i] - o[i]) / d[i]);
            near = near.max(a.min(b));
            far = far.min(a.max(b));
        }
        // `o + t d` is the world point `eye + t dir` in the object's own
        // units, so `t` is already a world distance.
        (near <= far).then_some(near)
    }
}

/// The objects in a cell the player can do something with: those whose
/// base has a script (people, activators, triggers, doors…), with trigger
/// volumes; items lying around; containers.
pub fn interactive_references(order: &LoadOrder, cell: FormId) -> Vec<Interactive> {
    interactive_from(order, order.references_in_cell(cell))
}

/// One placed reference as [`interactive_references`] lists it (`None`:
/// not a placed reference, or nothing to do with it).
pub fn interactive_reference(order: &LoadOrder, reference: FormId) -> Option<Interactive> {
    interactive_from(order, order.get(reference))
        .into_iter()
        .next()
}

/// [`interactive_references`] for one exterior grid square: its cell's
/// objects, then the worldspace's persistent objects standing in it (the
/// game assigns those to the grid cell they stand in,
/// `TESWorldSpace::AssignPersistentRefsToCell` (Xbox PDB)). Their order
/// within the square is not traced.
pub fn interactive_in_square(
    order: &LoadOrder,
    grid: &crate::WorldGrid,
    square: (i32, i32),
) -> Vec<Interactive> {
    let Some(cell) = grid.cell_at(square) else {
        return Vec::new();
    };
    let persistent = grid
        .persistent_in(square)
        .iter()
        .filter_map(|&id| order.get(id));
    interactive_from(
        order,
        order.references_in_cell(cell).into_iter().chain(persistent),
    )
}

fn interactive_from<'a>(
    order: &LoadOrder,
    references: impl IntoIterator<Item = esm::RecordRef<'a>>,
) -> Vec<Interactive> {
    let mut out = Vec::new();
    for rr in references {
        if rr.entry.header.is_deleted() {
            continue;
        }
        let Ok(record) = rr.record() else { continue };
        let Some(base) = record
            .get(NAME)
            .filter(|s| s.data.len() >= 4)
            .map(|s| rr.plugin.to_global(FormId(le_u32(&s.data, 0))))
        else {
            continue;
        };
        let Some(brr) = order.get(base) else { continue };
        let kind = brr.entry.header.kind;
        let Ok(base_record) = brr.record() else {
            continue;
        };
        let script = base_record
            .get(SCRI)
            .filter(|s| s.data.len() >= 4)
            .map(|s| brr.plugin.to_global(FormId(le_u32(&s.data, 0))));
        if script.is_none() && !is_item(kind) && kind != CONT && kind != FURN && kind != TERM {
            continue;
        }
        let count = record
            .get(XCNT)
            .filter(|s| s.data.len() >= 4)
            .map_or(1, |s| le_u32(&s.data, 0) as i32)
            .max(1);
        let bounds = base_record
            .get(OBND)
            .filter(|s| s.data.len() >= 12)
            .map(|s| {
                let v =
                    |i: usize| f32::from(i16::from_le_bytes([s.data[i * 2], s.data[i * 2 + 1]]));
                ([v(0), v(1), v(2)], [v(3), v(4), v(5)])
            });
        let Some(data) = record.get(esm::sig::DATA).filter(|d| d.data.len() >= 24) else {
            continue;
        };
        let f = |i: usize| le_f32(&data.data, i * 4);
        let scale = record
            .get(XSCL)
            .filter(|s| s.data.len() >= 4)
            .map_or(1.0, |s| le_f32(&s.data, 0));
        let trigger = record.get(XPRM).filter(|s| s.data.len() >= 32).map(|s| {
            let d = &s.data;
            TriggerBox {
                half: [le_f32(d, 0), le_f32(d, 4), le_f32(d, 8)].map(|v| v * scale),
                shape: le_u32(d, 28),
            }
        });
        out.push(Interactive {
            reference: rr.form_id,
            base,
            script,
            count,
            position: [f(0), f(1), f(2)],
            rotation: [f(3), f(4), f(5)],
            scale,
            trigger,
            bounds,
            name: base_record.full_name(),
            kind,
        });
    }
    out
}

/// The script (`SCPT`) whose variables an owner has: a quest's `SCRI`, or
/// a reference's base's.
pub fn script_of(order: &LoadOrder, owner: FormId) -> Option<FormId> {
    let rr = order.get(owner)?;
    let holder = if rr.entry.header.kind == QUST {
        rr
    } else {
        order.get(base_of(order, owner)?)?
    };
    let s = holder.subrecord(SCRI).ok()??;
    if s.len() < 4 {
        return None;
    }
    Some(holder.plugin.to_global(FormId(le_u32(&s, 0))))
}

/// Whether an event block's argument is empty or names `who` (`player`,
/// an editor ID).
fn names_who(order: &LoadOrder, b: &script::Block, who: FormId) -> bool {
    match b.args.first() {
        None => true,
        Some(Arg::Word(w)) => {
            let id = if w.eq_ignore_ascii_case("player") || w.eq_ignore_ascii_case("playerref") {
                Some(PLAYER_REF)
            } else {
                order.form_by_editor_id(w)
            };
            id == Some(who)
        }
        Some(_) => false,
    }
}

/// A scripted item's own script: who holds it, its variables (made on its
/// first run), and its events waiting (bit, and the reference it's for).
#[derive(Debug, Clone)]
pub struct ItemScript {
    pub holder: FormId,
    pub item: FormId,
    pub locals: Option<Locals>,
    pub events: Vec<(u32, FormId)>,
}

/// Item script events (`005ac750`'s callers): the bit and the blocks it
/// runs.
pub mod event {
    pub const ADD: u32 = 1;
    pub const EQUIP: u32 = 2;
    pub const DROP: u32 = 4;
    pub const UNEQUIP: u32 = 8;
    /// Each event's block (`0118e2f0`).
    pub const BLOCKS: [(u32, &str); 4] = [
        (ADD, "onadd"),
        (EQUIP, "onequip"),
        (DROP, "ondrop"),
        (UNEQUIP, "onunequip"),
    ];
}

/// An item record's script (`SCRI` on the item itself).
pub fn item_script(order: &LoadOrder, item: FormId) -> Option<FormId> {
    let rr = order.get(item).filter(|r| is_item(r.entry.header.kind))?;
    let record = rr.record().ok()?;
    let s = record.get(SCRI).filter(|s| s.data.len() >= 4)?;
    Some(rr.plugin.to_global(FormId(le_u32(&s.data, 0)))).filter(|f| f.0 != 0)
}

/// A script variable's name by its number (`SLSD` index, then `SCVR`).
fn variable_name(order: &LoadOrder, script: FormId, index: u32) -> Option<String> {
    let record = order.get(script)?.record().ok()?;
    let mut current: Option<u32> = None;
    for sub in &record.subrecords {
        if sub.kind == SLSD && sub.data.len() >= 4 {
            current = Some(le_u32(&sub.data, 0));
        } else if sub.kind == SCVR && current == Some(index) {
            return Some(sub.zstring());
        }
    }
    None
}

/// The game's questions about the state, as functions by number.
pub struct Facts<'a> {
    pub order: &'a LoadOrder,
    pub state: &'a GameState,
    /// Who's speaking, when known (saves reading their records again).
    pub speaker: Option<&'a Speaker>,
}

impl Facts<'_> {
    /// Who a reference is (their base, voice, race, sex, factions).
    fn person(&self, reference: FormId) -> Option<Speaker> {
        if let Some(s) = self.speaker.filter(|s| s.reference == reference) {
            return Some(s.clone());
        }
        let mut s = Speaker::load(self.order, reference, base_of(self.order, reference)?)?;
        if reference == PLAYER_REF {
            if let Some(female) = self.state.player_female {
                s.female = female;
            }
        }
        Some(s)
    }

    fn base(&self, reference: FormId) -> Option<FormId> {
        if let Some(s) = self.speaker.filter(|s| s.reference == reference) {
            return Some(s.base);
        }
        crate::more_functions::placed::base_now(self.order, self.state, reference)
    }

    /// A function's value asked about `on`, or `None` when it isn't
    /// carried out (or needs a reference and has none).
    pub fn value(&self, function: u16, on: Option<FormId>, args: &[Value]) -> Option<f64> {
        let sig = script::functions::FUNCTIONS.get(usize::from(function))?;
        let arg = |i: usize| args.get(i).cloned().unwrap_or(Value::Number(0.0));
        let s = self.state;
        Some(match sig.name {
            "GetStage" => f64::from(s.stages.get(&arg(0).form()).copied().unwrap_or(0)),
            "GetStageDone" => flag(
                s.stages_done
                    .contains(&(arg(0).form(), arg(1).number() as u16)),
            ),
            "GetQuestRunning" => flag(s.running.contains(&arg(0).form())),
            "GetQuestCompleted" => flag(s.completed.contains(&arg(0).form())),
            "GetObjectiveDisplayed" => flag(
                s.objectives
                    .contains_key(&(arg(0).form(), arg(1).number() as i32)),
            ),
            // Shown or not (`005a5d30`: the objective's state bit 0x02).
            "GetObjectiveCompleted" => flag(crate::script_functions::objective_completed(
                s,
                arg(0).form(),
                arg(1).number() as i32,
            )),
            "GetGlobalValue" => f64::from(s.globals.get(&arg(0).form()).copied().unwrap_or(0.0)),
            // Conditions name the variable by its number.
            "GetQuestVariable" | "GetScriptVariable" => {
                let owner = arg(0).form();
                let script = script_of(self.order, owner)?;
                let name = variable_name(self.order, script, arg(1).number() as u32)?;
                s.variables
                    .get(&owner)
                    .and_then(|l| l.get(&name))
                    .unwrap_or(0.0)
            }
            "GetIsID" => flag(self.base(on?) == Some(arg(0).form())),
            "GetIsReference" => flag(on == Some(arg(0).form())),
            "GetIsVoiceType" => flag(self.person(on?)?.voice == Some(arg(0).form())),
            "GetIsSex" => flag(u32::from(self.person(on?)?.female) == arg(0).number() as u32),
            "GetPCIsSex" => {
                flag(u32::from(self.person(PLAYER_REF)?.female) == arg(0).number() as u32)
            }
            "GetIsRace" => flag(self.person(on?)?.race == Some(arg(0).form())),
            "GetPCIsRace" => flag(self.person(PLAYER_REF)?.race == Some(arg(0).form())),
            // The factions now (with scripts' changes), and the speaker's
            // own as already read.
            "GetInFaction" => {
                let who = on?;
                let faction = arg(0).form();
                let removed = s
                    .faction_changes
                    .get(&(who, faction))
                    .is_some_and(|r| *r < 0);
                let speaking = self
                    .speaker
                    .filter(|sp| sp.reference == who)
                    .is_some_and(|sp| sp.factions.contains(&faction));
                flag(
                    crate::factions::factions_of(self.order, s, who).contains(&faction)
                        || (speaking && !removed),
                )
            }
            // The calling actor's factions' reaction to the target's.
            "GetFactionRelation" => {
                f64::from(crate::factions::reaction(self.order, s, on?, arg(0).form()).code())
            }
            "GetItemCount" => f64::from(s.item_count(self.order, on?, arg(0).form())),
            "GetDisabled" => flag(!crate::placement::enabled_now(self.order, on?, &s.disabled)),
            "GetDead" => flag(s.dead.contains(&on?)),
            // The body part the last hit landed on, and the killing blow
            // (`005a3c30`, `005a3e10`: -1 when none).
            "GetHitLocation" => f64::from(s.hit_location.get(&on?).copied().unwrap_or(-1)),
            "GetKillingBlowLimb" => f64::from(s.killing_blow_limb.get(&on?).copied().unwrap_or(-1)),
            // The dead whose base is this one.
            "GetDeadCount" => {
                let base = arg(0).form();
                s.dead
                    .iter()
                    .filter(|r| base_of(self.order, **r) == Some(base))
                    .count() as f64
            }
            "GetHealthPercentage" => {
                let who = on?;
                let full = crate::combat::max_health(self.order, s, who)?;
                if full > 0.0 {
                    (crate::combat::health(self.order, s, who)? / full).max(0.0)
                } else {
                    0.0
                }
            }
            // A form list asks for any of its forms (`0059da90`: type 0x55,
            // the faction armour scripts' `GetEquipped FactionGearNVNCR`).
            "GetEquipped" => {
                let worn = s.equipped.get(&on?);
                let asked = arg(0).form();
                let is_list = self
                    .order
                    .get(asked)
                    .is_some_and(|r| r.entry.header.kind.as_bytes() == b"FLST");
                let forms = if is_list {
                    crate::script_functions::form_list(self.order, s, asked)
                } else {
                    vec![asked]
                };
                flag(worn.is_some_and(|e| forms.iter().any(|f| e.contains(f))))
            }
            "GetSelf" => f64::from(on?.0),
            "IsActionRef" => flag(s.action_ref == Some(arg(0).form())),
            "GetActionRef" => f64::from(s.action_ref.map_or(0, |f| f.0)),
            // By the cells' editor IDs, the asked-for one's length
            // (`0059ef60`): see `script_functions::cell_matches`.
            "GetInCell" => {
                let cell = match on? {
                    PLAYER_REF => s.player_cell?,
                    r => s.place(self.order, r)?.1,
                };
                flag(crate::script_functions::cell_matches(
                    self.order,
                    cell,
                    arg(0).form(),
                ))
            }
            "GetActorValue" => self.current_actor_value(on?, arg(0).number() as u16)?,
            "GetPermanentActorValue" => self.permanent_actor_value(on?, arg(0).number() as u16)?,
            "GetBaseActorValue" => match arg(0).number() as u16 {
                crate::combat::av::HEALTH => crate::combat::max_health(self.order, s, on?)?,
                v => self.unaffected_actor_value(on?, v)?,
            },
            "HasMagicEffect" => flag(crate::magic::has_effect(s, on?, arg(0).form())),
            "GetLocked" => {
                let r = on?;
                match crate::terminal::placed(self.order, r) {
                    Some(t) => crate::terminal::get_locked(self.order, s, &t, r),
                    None => flag(crate::locks::lock_now(self.order, s, r).is_some()),
                }
            }
            // `0059d1d0`: `00430ae0` on the reference's lock (only locks
            // forcing broke have a count).
            "GetIsLockBroken" => flag(crate::lockpick::is_broken(self.order, s, on?)),
            "GetLockLevel" => {
                let r = on?;
                match crate::terminal::placed(self.order, r) {
                    Some(t) => crate::terminal::get_lock_level(self.order, s, &t, r),
                    None => {
                        crate::locks::lock_now(self.order, s, r).map_or(0.0, |l| f64::from(l.level))
                    }
                }
            }
            "GetLinkedRef" => {
                f64::from(crate::locks::linked_ref(self.order, on?).map_or(0, |r| r.0))
            }
            "IsSpellTarget" => flag(crate::magic::is_target_of(s, on?, arg(0).form())),
            // The weapon in hand's condition, in percent (fists: whole, a
            // guess; the tutorial warns below 25).
            "GetWeaponHealthPerc" => {
                let who = on?;
                match crate::combat::weapon_in_hand(self.order, s, who) {
                    Some(w) => {
                        f64::from(crate::combat::weapon_condition(s, who, w.form_id) * 100.0)
                    }
                    None => 100.0,
                }
            }
            // Whether the caller sees the target: the game's detection value
            // above 0 (`world::detection`).
            "GetDetected" => {
                let (who, other) = (on?, arg(0).form());
                if s.dead.contains(&who) {
                    0.0
                } else {
                    flag(self.detection(who, other)? > crate::detection::SEEN)
                }
            }
            "IsCurrentFurnitureRef" => flag(s.furniture.get(&on?) == Some(&arg(0).form())),
            // The furniture's base, or one in a form list (`0059dfe0`).
            "IsCurrentFurnitureObj" => {
                let sat = s.furniture.get(&on?).and_then(|f| base_of(self.order, *f));
                flag(sat.is_some_and(|b| {
                    crate::furniture::list_or_one(self.order, arg(0).form()).contains(&b)
                }))
            }
            // The sit state as the game maps it (`world::furniture`);
            // seated at once (no state yet): 3.
            "GetSitting" => match s.sitters.get(&on?) {
                Some(sitter) => f64::from(sitter.state.get_sitting()),
                None => flag(s.furniture.contains_key(&on?)) * 3.0,
            },
            "GetSleeping" => s
                .sitters
                .get(&on?)
                .map_or(0.0, |sitter| f64::from(sitter.state.get_sleeping())),
            "GetFurnitureMarkerID" => s
                .sitters
                .get(&on?)
                .map_or(0.0, |sitter| f64::from(sitter.marker.number)),
            "GetDistance" => self.distance(on?, arg(0).form())?,
            "Abs" => arg(0).number().abs(),
            "GetRandomPercent" => (s.dice % 100) as f64,
            "GetCurrentTime" => f64::from(s.global(self.order, "GameHour")?),
            "GetDayOfWeek" => {
                let g = |n: &str| s.global(self.order, n).map(|v| v as i32);
                f64::from(day_of_week(g("GameYear")?, g("GameMonth")?, g("GameDay")?))
            }
            "GetInWorldspace" => {
                let world = match on? {
                    PLAYER_REF => s.player_world,
                    r => {
                        let (space, cell, _, _) = s.place(self.order, r)?;
                        (space != cell).then_some(space)
                    }
                };
                flag(world == Some(arg(0).form()))
            }
            "IsPlayerInRegion" => flag(self.player_in_region(arg(0).form())),
            // The same interior, or the same grid square outdoors.
            "GetInSameCell" => {
                let (sa, ca, pa, _) = s.place(self.order, on?)?;
                let (sb, cb, pb, _) = s.place(self.order, arg(0).form())?;
                let outdoors = sa != ca && sb != cb;
                flag(if outdoors {
                    sa == sb && crate::square_of(pa) == crate::square_of(pb)
                } else {
                    ca == cb
                })
            }
            // The Pip-Boy's statistics ("Times Addicted", "Locks Picked"…):
            // none of what they count happens here yet, so they stay at a
            // new game's 0, except quests completed.
            "GetPCMiscStat" => {
                let Value::Text(stat) = arg(0) else {
                    return None;
                };
                f64::from(crate::stats::get(s, crate::stats::index(&stat)?))
            }
            "GetMapMarkerVisible" => {
                let marker = on?;
                // FalloutNV.exe 1.4.0.525, `005daac0` calls `005a51e0`:
                // 0 is hidden, 1 is visible, and 2 is visible and fast-travelable.
                let flags = map_marker_flags(self.order, marker)?;
                let visible = flags & 1 != 0
                    || s.map_markers.contains(&marker)
                    || s.discovered.contains(&marker);
                let travel = visible
                    && (flags & 2 != 0
                        || s.map_marker_travel.contains(&marker)
                        || s.discovered.contains(&marker));
                f64::from(if travel {
                    2
                } else if visible {
                    1
                } else {
                    0
                })
            }
            "GetIsCurrentPackage" => {
                let wanted = arg(0).form();
                let who = on?;
                // A package's own conditions may ask this; don't go round.
                let current = with_depth_guard(|| crate::ai::current_package(self.order, s, who))?;
                flag(current.is_some_and(|p| p.form_id == wanted))
            }
            "GetUnconscious" => flag(s.unconscious.contains(&on?)),
            "GetHasNote" => flag(s.notes.contains(&arg(0).form())),
            "GetTalkedToPC" => flag(s.talked_to.contains(&on?)),
            "GetLevel" => f64::from(self.level(on?)?),
            "GetPlayerTeammate" => flag(s.teammates.contains(&on?)),
            // Crime (`world::crime`).
            "GetPCEnemyofFaction" => flag(s.crime_enemies.contains(&arg(0).form())),
            "IsPCAMurderer" => flag(s.player_murderer),
            "IsTrespassing" => {
                let who = on?;
                if who != PLAYER_REF {
                    return Some(0.0);
                }
                flag(
                    s.player_cell
                        .is_some_and(|c| crate::crime::trespassing(self.order, s, c)),
                )
            }
            "GetMinorCrimeCount" | "GetMajorCrimeCount" => {
                let who = on?;
                let pick = |c: (u32, u32)| {
                    if sig.name == "GetMinorCrimeCount" {
                        c.0
                    } else {
                        c.1
                    }
                };
                if who == PLAYER_REF {
                    f64::from(pick(s.player_crimes))
                } else {
                    f64::from(
                        crate::factions::factions_of(self.order, s, who)
                            .iter()
                            .filter_map(|f| s.faction_crimes.get(f))
                            .map(|c| pick(*c))
                            .sum::<u32>(),
                    )
                }
            }
            "GetIsCurrentWeather" => flag(s.weather.current == Some(arg(0).form())),
            "GetCurrentWeatherPercent" => f64::from(s.weather.fade),
            "IsRaining" => flag(s.weather.precipitation(self.order, 0x04)),
            "IsSnowing" => flag(s.weather.precipitation(self.order, 0x08)),
            "GetPlayerTeammateCount" => s.teammates.len() as f64,
            "HasPerk" => {
                let r = on?;
                let perk = arg(0).form();
                if r == PLAYER_REF {
                    flag(s.perks.contains(&perk))
                } else {
                    let rr = self.order.get(self.base(r)?)?;
                    let record = rr.record_shared().ok()?;
                    let has = record.get_all(PRKR).any(|p| {
                        p.data.len() >= 4 && rr.plugin.to_global(FormId(le_u32(&p.data, 0))) == perk
                    });
                    flag(has)
                }
            }
            // The pre-order packs, by name: "Tribal Pack" is TribalPack.esm
            // (a guess at how the game matches them).
            "IsDLCInstalled" => {
                let Value::Text(name) = arg(0) else {
                    return None;
                };
                let file = format!("{}.esm", name.replace(' ', ""));
                flag(
                    self.order
                        .plugins()
                        .iter()
                        .any(|p| p.name.eq_ignore_ascii_case(&file)),
                )
            }
            "GetDestroyed" => flag(s.destroyed.contains(&on?)),
            // Fighting someone, or the player being fought.
            "IsInCombat" => {
                let who = on?;
                flag(
                    s.combat.contains_key(&who)
                        || (who == PLAYER_REF && s.combat.values().any(|t| *t == PLAYER_REF)),
                )
            }
            "GetCombatTarget" => f64::from(s.combat.get(&on?).map_or(0, |t| t.0)),
            // 100 when the calling actor would attack the target (`008b06d0`,
            // `factions::attacks_on_sight`), else 0; both must be actors.
            // Not done: the early 0 when both are already fighting
            // (`00992640`, the combat group test) and the call through
            // actor vtable +0x344 before it (its purpose isn't traced).
            // Translated from 0059ed30 (decompiled, FalloutNV.exe 1.4.0.525)
            "GetShouldAttack" => {
                let who = on?;
                let target = arg(0).form();
                let actor = |r: FormId| {
                    r == PLAYER_REF
                        || base_of(self.order, r)
                            .and_then(|b| self.order.get(b))
                            .is_some_and(|b| {
                                matches!(b.entry.header.kind.as_bytes(), b"NPC_" | b"CREA")
                            })
                };
                if actor(who)
                    && actor(target)
                    && crate::factions::attacks_on_sight(self.order, s, who, target)
                {
                    100.0
                } else {
                    0.0
                }
            }
            // True of a new game, where nobody has been eaten yet
            // (hardcore: `world::living`).
            "HasBeenEaten" => 0.0,
            // `005a08c0`: 1 while knocked out or falling to it (knock
            // states 3 and 4; essential people who are down lie knocked
            // out), else 0 (`world::fatigue`).
            "GetKnockedState" => crate::fatigue::knocked_state_value(s, on?),
            // `00893530`: fatigue now ÷ its full value (`world::fatigue`).
            "GetFatiguePercentage" => crate::fatigue::percentage(self.order, s, on?)?,
            // `005a4240`, `005a42b0`: the members and targets of the
            // actor's combat group, 0 out of a fight
            // (`world::combat_groups`).
            "GetGroupMemberCount" => crate::combat_groups::member_count(s, on?) as f64,
            "GetGroupTargetCount" => crate::combat_groups::target_count(s, on?) as f64,
            // `005dec30`: the player's winnings there by quarters of its
            // limit (`world::casino`); 0 where they haven't played.
            "GetCasinoWinningsLevel" => {
                f64::from(crate::casino::winnings_level(self.order, s, arg(0).form()))
            }
            // The process's flag (`00915d40`; nobody without one: 0).
            "IsWeaponOut" => flag(s.weapon_out.contains(&on?)),
            // Reputations (`world::reputation`); a type or axis out of
            // range answers nothing, as the game's handlers refuse it.
            "GetReputation" | "GetReputationPct" => {
                let (rep, kind) = (arg(0).form(), arg(1).number() as i64);
                if !(0..=1).contains(&kind) {
                    return None;
                }
                let value = crate::reputation::get(s, rep, kind as u8);
                if sig.name == "GetReputation" {
                    f64::from(value)
                } else {
                    let max = crate::reputation::reputation_max(self.order, rep)?;
                    f64::from(value / max)
                }
            }
            "GetReputationThreshold" => {
                let axis = arg(1).number() as i64;
                if !(0..=2).contains(&axis) {
                    return None;
                }
                f64::from(crate::reputation::threshold(
                    self.order,
                    s,
                    arg(0).form(),
                    axis as u8,
                ))
            }
            // V.A.T.S. (`world::vats::function_value`); the weapon in hand's
            // `GetWeaponAnimType` and `IsWeaponInList` are `script_functions`'.
            "GetVATSMode"
            | "GetVATSValue"
            | "GetVATSRightAreaFree"
            | "GetVATSLeftAreaFree"
            | "GetVATSBackAreaFree"
            | "GetVATSFrontAreaFree"
            | "GetVATSRightTargetVisible"
            | "GetVATSLeftTargetVisible"
            | "GetVATSBackTargetVisible"
            | "GetVATSFrontTargetVisible" => crate::vats::function_value(self, sig.name, on, args)?,
            other => {
                return crate::living::value(self, other, on, args)
                    .or_else(|| crate::script_functions::value(self, other, on, args))
                    .or_else(|| crate::more_functions::value(self, other, on, args))
                    .flatten()
            }
        })
    }

    /// `IsPlayerInRegion`: the player's cell lists the region (`XCLR`), or
    /// the player stands inside one of its areas (`RPLD` outlines, in its
    /// worldspace `WNAM`). Map regions such as `vMapVictorInNovacRegion`
    /// have only areas: Novac's cells list just the wasteland's region.
    /// That the game counts both is a guess.
    pub(crate) fn player_in_region(&self, region: FormId) -> bool {
        let s = self.state;
        let listed = s
            .player_cell
            .and_then(|cell| self.order.get(cell))
            .is_some_and(|rr| {
                rr.record().is_ok_and(|r| {
                    r.get_all(XCLR).any(|x| {
                        x.data.chunks_exact(4).any(|c| {
                            let id = FormId(u32::from_le_bytes([c[0], c[1], c[2], c[3]]));
                            rr.plugin.to_global(id) == region
                        })
                    })
                })
            });
        if listed {
            return true;
        }
        let (Some(world), Some(p)) = (s.player_world, s.player_position) else {
            return false;
        };
        crate::region::Region::load(self.order, region)
            .is_some_and(|r| r.world == Some(world) && r.contains(p[0], p[1]))
    }

    /// Where a reference is: (the interior cell or worldspace it's in,
    /// position); see [`GameState::place`].
    fn place_of(&self, r: FormId) -> Option<(FormId, [f32; 3])> {
        let (space, _, position, _) = self.state.place(self.order, r)?;
        Some((space, position))
    }

    /// How well `who` detects `other` now (`world::detection::value`), in
    /// the same place only. What the world doesn't know is taken as: a
    /// clear line of sight (no collision here), the target standing still
    /// and not sneaking (unless the state says the player is: `player_
    /// moving`, `player_running`, `player_sneaking`), lit at 50 of 100, no
    /// recent shot, no body armour weight (all guesses until the viewer
    /// supplies them).
    pub fn detection(&self, who: FormId, other: FormId) -> Option<i32> {
        if crate::companions::hidden_with_player(self.state, other) {
            return Some(crate::companions::HIDDEN_WITH_PLAYER);
        }
        let inputs = self.detection_inputs(who, other, true, None)?;
        Some(crate::detection::value(self.order, &inputs))
    }

    /// What a detection test between `who` and `other` takes (see
    /// [`Self::detection`]), with the line of sight given (the viewer casts
    /// a ray; the world can't) and, for someone other than the player, how
    /// they move (`moving`, `running`; standing still without it).
    pub fn detection_inputs(
        &self,
        who: FormId,
        other: FormId,
        line_of_sight: bool,
        motion: Option<(bool, bool)>,
    ) -> Option<crate::detection::Inputs> {
        let s = self.state;
        let (space_a, cell_a, pa, heading) = s.place(self.order, who)?;
        let (space_b, _, pb, _) = s.place(self.order, other)?;
        if space_a != space_b {
            return None;
        }
        let d: f32 = (0..3).map(|i| (pa[i] - pb[i]).powi(2)).sum::<f32>().sqrt();
        // Within ±95° of where `who` faces (heading clockwise from north).
        let toward = (pb[0] - pa[0]).atan2(pb[1] - pa[1]);
        let mut off = (toward - heading).rem_euclid(std::f32::consts::TAU);
        if off > std::f32::consts::PI {
            off = std::f32::consts::TAU - off;
        }
        let cone = game_setting(self.order, "fDetectionViewCone").unwrap_or(190.0);
        let player = other == PLAYER_REF;
        let av = |r: FormId, a: u16| self.current_actor_value(r, a).unwrap_or(0.0) as f32;
        let fighting = s.combat.get(&who).copied();
        let (mut moving, mut running) = if player {
            (s.player_moving, s.player_running)
        } else {
            motion.unwrap_or((false, false))
        };
        // The one noticed's perks' "Ignore Running During Detection"
        // (entry point 31, Silent Running; `008a0d10`): they never count
        // as running, and while sneaking not as moving either.
        let silent = crate::perks::apply_for(
            self.order,
            s,
            other,
            crate::perks::entry::IGNORE_RUNNING_DURING_DETECTION,
            0.0,
            &[],
        ) != 0.0;
        if silent {
            running = false;
            if player && s.player_sneaking {
                moving = false;
            }
        }
        Some(crate::detection::Inputs {
            distance: d,
            outdoors: space_a != cell_a,
            line_of_sight,
            in_cone: off.to_degrees() <= cone / 2.0,
            moving,
            running,
            sneaking: player && s.player_sneaking,
            light: 50.0,
            shot_noise: crate::noise::value(s, other) as f32,
            armour_weight: 0.0,
            armour_penalty: 0.0,
            perception: av(who, 6),
            in_combat: fighting.is_some(),
            fighting_other: fighting.is_some_and(|t| t != other),
            // A teammate sneaks as well as the player does, if better
            // (`008a0d10`).
            sneak: if s.teammates.contains(&other) {
                av(other, 42).max(av(PLAYER_REF, 42))
            } else {
                av(other, 42)
            },
            target_level: f32::from(self.level(other).unwrap_or(1)),
            detector_level: f32::from(self.level(who).unwrap_or(1)),
        })
    }

    /// `GetDistance`: straight-line distance between two references in
    /// the same interior or worldspace. In different ones: not known (what
    /// the game gives then isn't traced).
    fn distance(&self, a: FormId, b: FormId) -> Option<f64> {
        let (sa, pa) = self.place_of(a)?;
        let (sb, pb) = self.place_of(b)?;
        if sa != sb {
            return None;
        }
        let d: f32 = (0..3).map(|i| (pa[i] - pb[i]).powi(2)).sum::<f32>().sqrt();
        Some(f64::from(d))
    }

    /// An actor's level: the player's own; an NPC's from `ACBS` (offset 8),
    /// or for "PC level mult" (flag 0x80) that many thousandths of the
    /// player's, at least `ACBS`'s calc min (a guess at the rounding).
    fn level(&self, on: FormId) -> Option<u16> {
        if on == PLAYER_REF {
            return Some(self.state.player_level);
        }
        let record = self.order.get(self.base(on)?)?.record().ok()?;
        let acbs = &record.get(ACBS)?.data;
        if acbs.len() < 14 {
            return None;
        }
        let flags = le_u32(acbs, 0);
        let level = u16::from_le_bytes([acbs[8], acbs[9]]);
        if flags & 0x80 == 0 {
            return Some(level);
        }
        let calc_min = u16::from_le_bytes([acbs[10], acbs[11]]);
        let scaled =
            (f32::from(self.state.player_level) * f32::from(level) / 1000.0).round() as u16;
        Some(scaled.max(calc_min).max(1))
    }

    /// An actor's value now: what scripts set, else worked out. Action
    /// points, carry weight and melee damage from the game's settings
    /// (`fAVDActionPointsBase` 65 + `fAVDActionPointsMult` 3 × Agility,
    /// `fAVDCarryWeightsBase` 150 + `fAVDCarryWeightMult` 10 × Strength,
    /// `fAVDMeleeDamageStrengthOffset` 0 + `…Mult` 0.5 × Strength: the
    /// shape the settings' names give, not traced in the code); body part
    /// conditions whole (100) until something hurts them; inventory weight
    /// what their items' records weigh; the rest from the record
    /// ([`Self::actor_value`]). Effects working on them add their
    /// modifiers and damage comes off (`world::magic`); rads and the
    /// hardcore needs are the damage itself.
    pub fn current_actor_value(&self, who: FormId, av: u16) -> Option<f64> {
        // Health: full health less the damage taken (`world::combat`).
        if av == crate::combat::av::HEALTH {
            return crate::combat::health(self.order, self.state, who);
        }
        let damage = self
            .state
            .value_damage
            .get(&(who, av))
            .copied()
            .unwrap_or(0.0);
        if crate::magic::COUNTERS.contains(&av) {
            return Some(damage + crate::magic::modifier(self.state, who, av, false));
        }
        Some(
            self.permanent_actor_value(who, av)?
                + crate::magic::modifier(self.state, who, av, false)
                - crate::magic::modifier(self.state, who, av, true)
                - damage,
        )
    }

    /// An actor value with what lasts (abilities) but not what passes
    /// (chems) or damage (`GetPermanentActorValue`).
    pub fn permanent_actor_value(&self, who: FormId, av: u16) -> Option<f64> {
        if av == crate::combat::av::HEALTH {
            return crate::combat::max_health(self.order, self.state, who);
        }
        Some(
            self.unaffected_actor_value(who, av)?
                + crate::magic::modifier(self.state, who, av, true),
        )
    }

    /// An actor value as set and worked out, before effects and damage.
    fn unaffected_actor_value(&self, who: FormId, av: u16) -> Option<f64> {
        if let Some(&v) = self.state.actor_values.get(&(who, av)) {
            return Some(v);
        }
        let setting = |name: &str| game_setting(self.order, name).map(f64::from);
        let derived = |base: &str, mult: &str, attribute: u16| -> Option<f64> {
            Some(setting(base)? + setting(mult)? * self.current_actor_value(who, attribute)?)
        };
        match av {
            12 => derived("fAVDActionPointsBase", "fAVDActionPointsMult", 10),
            // Carry weight, then the player's perks' "Get Max Carry
            // Weight" (entry point 22).
            13 => {
                let w = derived("fAVDCarryWeightsBase", "fAVDCarryWeightMult", 5)?;
                if who != PLAYER_REF {
                    return Some(w);
                }
                Some(f64::from(crate::perks::apply(
                    self.order,
                    self.state,
                    crate::perks::entry::GET_MAX_CARRY_WEIGHT,
                    w as f32,
                )))
            }
            // Heal Rate (`00643a70`): by permanent Endurance, nothing up
            // to 5, then `fAVDHealRateEndurance<6..10>Bonus` (5, 5, 5, 10,
            // 10 in the data).
            15 => {
                let end = self.permanent_actor_value(who, crate::combat::av::ENDURANCE)?;
                let end = (end.floor() as i64).min(10);
                if end < 6 {
                    Some(0.0)
                } else {
                    Some(setting(&format!("fAVDHealRateEndurance{end}Bonus")).unwrap_or(0.0))
                }
            }
            17 => derived(
                "fAVDMeleeDamageStrengthOffset",
                "fAVDMeleeDamageStrengthMult",
                5,
            ),
            // Fatigue: the record's and the derived part (`world::fatigue`).
            22 => crate::fatigue::base_fatigue(self, who),
            25..=31 => Some(100.0),
            // What they carry weighs (the items' records' weights).
            46 => Some(f64::from(self.state.inventory_weight(self.order, who))),
            32..=45 if who == PLAYER_REF => {
                crate::chargen::player_skill(self.order, self.state, av, |a| {
                    self.current_actor_value(who, a)
                })
            }
            _ => self.actor_value(who, av),
        }
    }

    /// An actor value before damage: what scripts set, else the record's.
    pub fn base_actor_value(&self, who: FormId, av: u16) -> Option<f64> {
        match self.state.actor_values.get(&(who, av)) {
            Some(&v) => Some(v),
            None => self.actor_value(who, av),
        }
    }

    /// Someone's level (see [`Self::level`]).
    pub fn level_of(&self, who: FormId) -> Option<u16> {
        self.level(who)
    }

    /// An actor's value from their base record. People (`NPC_`): SPECIAL
    /// (`DATA`, after the base health), skills (`DNAM`, 14 values from
    /// Barter on), health, and the AI's aggression, confidence, energy,
    /// responsibility and mood (`AIDT`'s first five bytes). Creatures
    /// (`CREA` `DATA`: type, three skills, health i16, 2 unused bytes,
    /// damage i16, then SPECIAL); karma from `ACBS`. The ten script
    /// variables start at 0, as do the values only effects change. An actor
    /// made from a template (`TPLT`, leveled lists' first entry) takes its
    /// SPECIAL, skills and health from it with the "use stats" flag (0x02)
    /// and its AI values with "use AI data" (0x10): the cave rats of Broc
    /// Flower Cave have all their values from theirs.
    fn actor_value(&self, on: FormId, index: u16) -> Option<f64> {
        let base = self.base(on)?;
        if let 0..=4 = index {
            let (_, record) =
                crate::actor::data_record(self.order, base, crate::actor::USE_AI_DATA)?;
            let aidt = record.get(AIDT)?;
            return aidt.data.get(usize::from(index)).map(|&v| f64::from(v));
        }
        let rr = self.order.get(base)?;
        let record = rr.record_shared().ok()?;
        // Karma: `ACBS` (flags, fatigue, barter gold, level, calc min and
        // max, speed mult, then karma f32 at 16).
        if index == 23 {
            let acbs = record.get(ACBS).filter(|s| s.data.len() >= 20)?;
            return Some(f64::from(le_f32(&acbs.data, 16)));
        }
        // The script variables, and what only effects and perks raise
        // (paralysis, invisibility, chameleon, night eye, detect life,
        // water breathing, rads, bloody mess, ignoring crippled limbs,
        // hardcore thirst, hunger and sleep): 0, as nothing here applies
        // effects yet.
        if let 47..=51 | 53..=55 | 62..=75 = index {
            return Some(0.0);
        }
        // What the base form's actor value getter (`005f0fb0`, the
        // `TESActorBase` owner's `+8`; people's skills first go through
        // `00607850`) gives for the values no record field holds: 0, so
        // `player.GetAV XP` is 0 before any XP (the script handler,
        // `0059c4f0`, always sets its result from the actor's owner `+0xc`,
        // `0093acb0` for the player: that base plus the modifiers). Poison,
        // radiation, fire, electric, frost, energy and EMP resistance only
        // effects raise. Speed mult is `ACBS`'s (case `0x15`, `008f21d0`).
        if let 19 | 20 | 24 | 52 | 58..=61 = index {
            return Some(0.0);
        }
        if index == 21 {
            let acbs = record.get(ACBS).filter(|s| s.data.len() >= 16)?;
            return Some(f64::from(u16::from_le_bytes([
                acbs.data[14],
                acbs.data[15],
            ])));
        }
        // Assistance (`AIDT` byte 14).
        if index == 57 {
            let (_, record) =
                crate::actor::data_record(self.order, base, crate::actor::USE_AI_DATA)?;
            return record
                .get(AIDT)
                .and_then(|s| s.data.get(14).copied())
                .map(f64::from);
        }
        let (rr, record) = crate::actor::data_record(self.order, base, crate::actor::USE_STATS)?;
        let data = &record.get(esm::sig::DATA)?.data;
        if rr.entry.header.kind == CREA {
            return match index {
                5..=11 => data.get(10 + usize::from(index - 5)).map(|&v| f64::from(v)),
                16 => (data.len() >= 6).then(|| f64::from(i16::from_le_bytes([data[4], data[5]]))),
                _ => None,
            };
        }
        match index {
            5..=11 => data.get(4 + usize::from(index - 5)).map(|&v| f64::from(v)),
            16 => (data.len() >= 4).then(|| f64::from(le_u32(data, 0) as i32)),
            // `DNAM`: the 14 skills, then 14 offsets (`NPC_DATA`'s `cSkill`
            // and `cOffset`, Xbox PDB), the offset added unless the stats
            // are worked out by the game (`ACBS` flag 0x10; `00607850`,
            // `005f0d00`): Mick's Repair is 15 + 60.
            32..=45 => {
                let i = usize::from(index - 32);
                let d = &record.get(DNAM)?.data;
                let skill = f64::from(*d.get(i)?);
                let auto = record
                    .get(ACBS)
                    .filter(|s| s.data.len() >= 4)
                    .is_some_and(|s| le_u32(&s.data, 0) & 0x10 != 0);
                let offset = if auto {
                    0
                } else {
                    d.get(14 + i).copied().unwrap_or(0)
                };
                Some(skill + f64::from(offset))
            }
            _ => None,
        }
    }

    /// Whether conditions pass, asked about `subject` (who says a line;
    /// the player for quest stages) and `target` (who they say it to).
    /// Conditions are joined left to right; one with the OR flag is joined
    /// with the next by OR, which binds before AND (`A or B and C` is
    /// `(A or B) and C`), as the game's editor shows. Functions not carried
    /// out give 0 (a rule of this reimplementation, not the game's).
    pub fn conditions_pass(
        &self,
        conditions: &[Condition],
        subject: FormId,
        target: FormId,
    ) -> bool {
        let mut all = true;
        let mut group: Option<bool> = None;
        for c in conditions {
            let value = match c.global {
                Some(g) => self.state.globals.get(&g).copied().unwrap_or(0.0),
                None => c.value,
            };
            let result = self.condition_value(c, subject, target) as f32;
            let ok = c.compare(result, value);
            let so_far = group.map_or(ok, |g| g || ok);
            if c.or {
                group = Some(so_far);
            } else {
                all &= so_far;
                group = None;
            }
        }
        if let Some(g) = group {
            all &= g;
        }
        all
    }

    /// One condition's function value, asked about `subject` or `target`
    /// as the condition says (0 when the function isn't carried out).
    pub fn condition_value(&self, c: &Condition, subject: FormId, target: FormId) -> f64 {
        // Run on: 0 the subject, 1 the target, 2 a named reference.
        let on = match c.run_on {
            0 => Some(subject),
            1 => Some(target),
            2 => c.reference,
            _ => None,
        };
        let params = script::functions::FUNCTIONS
            .get(usize::from(c.function))
            .map_or(&[][..], |s| s.params);
        let args: Vec<Value> = (0..2)
            .map(|i| match params.get(i) {
                Some(p) if script::param_is_form(p.kind) => Value::Form(c.param_forms[i]),
                _ => Value::Number(f64::from(c.params[i] as i32)),
            })
            .collect();
        self.value(c.function, on, &args).unwrap_or(0.0)
    }
}

/// Parsed scripts and quests, read once.
#[derive(Default)]
pub struct ScriptCache {
    scripts: Mutex<HashMap<FormId, Option<Arc<Script>>>>,
    quests: Mutex<HashMap<FormId, Option<Arc<Quest>>>>,
}

impl ScriptCache {
    /// A script record's (`SCPT`) source, parsed.
    pub fn script(&self, order: &LoadOrder, id: FormId) -> Option<Arc<Script>> {
        let mut map = self.scripts.lock().unwrap();
        map.entry(id)
            .or_insert_with(|| {
                let rr = order.get(id).filter(|r| r.entry.header.kind == SCPT)?;
                let record = rr.record().ok()?;
                let source = esm::text::decode_cp1252(&record.get(SCTX)?.data);
                script::parse(&source).ok().map(Arc::new)
            })
            .clone()
    }

    pub fn quest(&self, order: &LoadOrder, id: FormId) -> Option<Arc<Quest>> {
        let mut map = self.quests.lock().unwrap();
        map.entry(id)
            .or_insert_with(|| Quest::load(order, id).map(Arc::new))
            .clone()
    }
}

/// A runner of quest scripts other than this crate's interpreter (the
/// game's own, `host::quest_vm`). [`Runner::update`] offers it each
/// quest's `GameMode` run; the quests it declines run as before.
pub trait QuestVm {
    /// Runs the quest `quest`'s script `script` (the world's parse of its
    /// source: `parsed`) once, as a quest's script runs while the game
    /// runs, with `locals` its variables (read before, written after) and
    /// `seconds_passed` what `GetSecondsPassed` gives. False when it
    /// doesn't take this quest: then it changed nothing.
    fn run_quest(
        &mut self,
        order: &LoadOrder,
        quest: FormId,
        script: FormId,
        parsed: &Script,
        locals: &mut Locals,
        seconds_passed: f32,
    ) -> bool;
}

/// Runs scripts against the game state.
pub struct Runner<'a> {
    pub order: &'a LoadOrder,
    pub scripts: &'a ScriptCache,
    pub state: &'a mut GameState,
    /// The reference the script runs on: calls without one apply to it.
    pub this: Option<FormId>,
    /// Whose variables the script's own are (a quest, a reference).
    pub owner: Option<FormId>,
    /// `GetSecondsPassed`.
    pub seconds_passed: f32,
    /// A command that may change the loaded references ran (the script
    /// runner's flag +0xa1, set before a command whose table entry has
    /// flag 0x100: `Activate`, `MoveTo`, `ForceFlee`, `ForceTakeCover`,
    /// `ExitGame`, `MoveToFade`; `005e1550`). `Script::Run` returns it and
    /// the reference-script pass stops on it ([`crate::ref_scripts`]).
    /// Each nested `Script::Run` has its own runner in the game
    /// (`005e2590`); here nested runs share this flag (unresolved).
    pub references_changed: bool,
    /// The viewer's camera and collision, for `GetLineOfSight`
    /// ([`crate::sight`]); none headless.
    pub sight: Option<&'a dyn crate::sight::Sight>,
    /// Another runner of quest scripts' `GameMode` runs: the game's own
    /// interpreter (`host::quest_vm`), for the quests it takes; none:
    /// every quest runs through this crate's interpreter.
    quest_vm: Option<&'a mut dyn QuestVm>,
    /// A scripted item's own run: its holder and the item (`RemoveMe`),
    /// and whether `RemoveMe` took it.
    item: Option<(FormId, FormId)>,
    removed: bool,
    /// The reference `DropMe` made for it.
    dropped: Option<FormId>,
    depth: u8,
}

/// The script functions whose table entry has flag 0x100 (byte +0x25),
/// which make the script runner report a possible change to the loaded
/// references (`005e1550` sets runner+0xa1).
const CHANGES_REFERENCES: &[&str] = &[
    "Activate",
    "MoveToMarker",
    "ForceFlee",
    "ForceTakeCover",
    "ExitGame",
    "MoveToMarkerWithFade",
];

impl<'a> Runner<'a> {
    pub fn new(order: &'a LoadOrder, scripts: &'a ScriptCache, state: &'a mut GameState) -> Self {
        Runner {
            order,
            scripts,
            state,
            this: None,
            owner: None,
            seconds_passed: 0.0,
            references_changed: false,
            sight: None,
            quest_vm: None,
            item: None,
            removed: false,
            dropped: None,
            depth: 0,
        }
    }

    /// With the viewer's camera and collision for `GetLineOfSight`.
    pub fn with_sight(mut self, sight: &'a dyn crate::sight::Sight) -> Self {
        self.sight = Some(sight);
        self
    }

    /// With another runner for the quests it takes ([`QuestVm`]).
    pub fn with_quest_vm(mut self, vm: &'a mut dyn QuestVm) -> Self {
        self.quest_vm = Some(vm);
        self
    }

    /// One `GameMode` run of the quest `q`'s script: through the
    /// [`QuestVm`] when there is one and it takes the quest, with the
    /// quest's variables; else (or when it declines) through this crate's
    /// interpreter ([`Runner::run_blocks`]).
    fn run_quest_gamemode(&mut self, q: FormId) {
        if let Some(vm) = self.quest_vm.take() {
            let script = script_of(self.order, q)
                .and_then(|id| Some((id, self.scripts.script(self.order, id)?)));
            let mut ran = false;
            if let Some((id, parsed)) = script {
                if parsed.blocks.iter().any(|b| b.kind == "gamemode") {
                    let mut locals = self.take_locals(Some(q));
                    ran =
                        vm.run_quest(self.order, q, id, &parsed, &mut locals, self.seconds_passed);
                    self.put_locals(Some(q), locals);
                }
            }
            self.quest_vm = Some(vm);
            if ran {
                return;
            }
        }
        self.run_blocks(q, None, "gamemode", |_| true);
    }

    /// The scripted items' own runs (`004d2480` for each holder's scripted
    /// items): items gone from their holder lose their script; each one
    /// held by the player runs (its `GameMode` blocks, each run); every
    /// one with events waiting runs their blocks (`OnAdd`, `OnEquip`,
    /// `OnUnequip`, `OnDrop` naming the reference or none), in the
    /// script's order, the holder the reference it runs on; then its events
    /// are cleared (`005a8ea0`). Holders other than the player run only
    /// for events (the game runs a reference's items with its own script,
    /// `00565870`, which nv-rs doesn't do for every reference).
    pub fn run_item_scripts(&mut self) {
        // Items gone by other means take their scripts with them.
        let mut kept: std::collections::HashMap<(FormId, FormId), i32> =
            std::collections::HashMap::new();
        let order = self.order;
        let state = &*self.state;
        let keep: Vec<bool> = state
            .item_scripts
            .iter()
            .map(|s| {
                let n = kept.entry((s.holder, s.item)).or_insert(0);
                *n += 1;
                // A holder's things, or a dropped one lying in the world.
                let held = state.item_count(order, s.holder, s.item).max(
                    crate::more_functions::placed::held_in_world(order, state, s.holder, s.item),
                );
                *n <= held
            })
            .collect();
        let mut keep = keep.into_iter();
        self.state
            .item_scripts
            .retain(|_| keep.next().unwrap_or(false));
        let mut i = 0;
        while i < self.state.item_scripts.len() {
            let ItemScript {
                holder,
                item,
                ref events,
                ..
            } = self.state.item_scripts[i];
            let events = events.clone();
            if holder != PLAYER_REF && events.is_empty() {
                i += 1;
                continue;
            }
            let Some(script) = item_script(order, item).and_then(|s| self.scripts.script(order, s))
            else {
                i += 1;
                continue;
            };
            let blocks: Vec<&script::Block> = script
                .blocks
                .iter()
                .filter(|b| {
                    b.kind == "gamemode"
                        || event::BLOCKS.iter().any(|(bit, kind)| {
                            b.kind == *kind
                                && events
                                    .iter()
                                    .any(|(m, r)| m & bit != 0 && names_who(order, b, *r))
                        })
                })
                .collect();
            let mut locals = self.state.item_scripts[i]
                .locals
                .take()
                .unwrap_or_else(|| Locals::new(&script));
            self.state.item_scripts[i].events.clear();
            if !blocks.is_empty() {
                let saved = (self.this, self.owner, self.item, self.removed, self.dropped);
                self.this = Some(holder);
                self.owner = None;
                self.item = Some((holder, item));
                self.removed = false;
                self.dropped = None;
                let action = self.state.action_ref.replace(holder);
                for block in blocks {
                    if interp::run(&block.body, &mut locals, self) != Flow::Done {
                        break;
                    }
                }
                self.state.action_ref = action;
                let removed = self.removed;
                let dropped = self.dropped;
                (self.this, self.owner, self.item, self.removed, self.dropped) = saved;
                if let Some(made) = dropped {
                    // `DropMe`: this one lies in the world now, with its
                    // `OnDrop` waiting; `RemoveMe` into a container: it's
                    // there, with `OnDrop` and `OnAdd` waiting.
                    let into_container = !self.state.more.placed.refs.contains_key(&made);
                    let s = &mut self.state.item_scripts[i];
                    s.locals = Some(locals);
                    s.holder = made;
                    s.events.push((event::DROP, holder));
                    if into_container {
                        s.events.push((event::ADD, made));
                    }
                    i += 1;
                    continue;
                }
                if removed {
                    self.state.item_scripts.remove(i);
                    continue;
                }
            }
            if let Some(s) = self.state.item_scripts.get_mut(i) {
                s.locals = Some(locals);
            }
            i += 1;
        }
    }

    fn facts(&self) -> Facts<'_> {
        Facts {
            order: self.order,
            state: self.state,
            speaker: None,
        }
    }

    /// A word as a record: `player`, an editor ID, or (as the game's
    /// console takes them) a form ID written as 8 hex digits.
    fn form(&self, word: &str) -> Option<FormId> {
        if word.eq_ignore_ascii_case("player") || word.eq_ignore_ascii_case("playerref") {
            return Some(PLAYER_REF);
        }
        self.order.form_by_editor_id(word).or_else(|| {
            (word.len() == 8 && word.chars().all(|c| c.is_ascii_hexdigit()))
                .then(|| u32::from_str_radix(word, 16).ok().map(FormId))
                .flatten()
                .filter(|id| self.order.get(*id).is_some())
        })
    }

    /// An owner's variables, taken out of the state while a script uses
    /// them (put back with [`Self::put_locals`]); new ones start at 0.
    fn take_locals(&mut self, owner: Option<FormId>) -> Locals {
        let Some(owner) = owner else {
            return Locals::default();
        };
        if let Some(l) = self.state.variables.remove(&owner) {
            return l;
        }
        script_of(self.order, owner)
            .and_then(|s| self.scripts.script(self.order, s))
            .map(|s| Locals::new(&s))
            .unwrap_or_default()
    }

    fn put_locals(&mut self, owner: Option<FormId>, locals: Locals) {
        if let Some(owner) = owner {
            self.state.variables.insert(owner, locals);
        }
    }

    /// Runs with `this` and `owner` set, then puts them back.
    fn with<T>(
        &mut self,
        this: Option<FormId>,
        owner: Option<FormId>,
        f: impl FnOnce(&mut Self, &mut Locals) -> T,
    ) -> T {
        let saved = (self.this, self.owner);
        self.this = this;
        self.owner = owner;
        let mut locals = self.take_locals(owner);
        let out = f(self, &mut locals);
        self.put_locals(owner, locals);
        (self.this, self.owner) = saved;
        out
    }

    /// Runs a result script (a dialogue line's, a quest stage's): its
    /// statements on `this`, with `owner`'s variables as its own.
    pub fn run_source(
        &mut self,
        source: &str,
        this: Option<FormId>,
        owner: Option<FormId>,
    ) -> interp::Flow {
        let script = match script::parse(source) {
            Ok(s) => s,
            Err(e) => {
                if let Some(o) = owner.or(this) {
                    self.state.broken.insert(o, e.to_string());
                }
                return interp::Flow::Done;
            }
        };
        self.with(this, owner, |runner, locals| {
            // Variables the result script declares itself (a terminal's
            // `ref myLink`), beside its owner's.
            for (kind, name) in &script.variables {
                if locals.get(name).is_none() {
                    locals.insert(name, *kind, 0.0);
                }
            }
            interp::run(&script.body, locals, runner)
        })
    }

    /// Runs an owner's script's blocks of one kind (`gamemode`,
    /// `onactivate`, …) whose argument `matches`; `this` is the reference
    /// it runs on (none for a quest).
    pub fn run_blocks(
        &mut self,
        owner: FormId,
        this: Option<FormId>,
        kind: &str,
        matches: impl Fn(&script::Block) -> bool,
    ) {
        let Some(script) =
            script_of(self.order, owner).and_then(|s| self.scripts.script(self.order, s))
        else {
            return;
        };
        if !script.blocks.iter().any(|b| b.kind == kind) {
            return;
        }
        self.with(this, Some(owner), |runner, locals| {
            interp::run_blocks(&script, kind, matches, locals, runner)
        });
    }

    /// Time passes: each running quest's script runs (its `GameMode`
    /// blocks) once its delay has gone by (one a script set, else its
    /// own, else [`QUEST_SCRIPT_DELAY`]). A quest's script first runs as
    /// soon as it's running (traced for quests with their own delay,
    /// `005ab400`; a guess for the rest, whose countdowns the game
    /// staggers), with `GetSecondsPassed` the time since its last run (on
    /// an own-delay quest's first run, the frame's).
    pub fn update(&mut self, seconds: f32) {
        self.run_item_scripts();
        self.state.roll();
        self.state.seconds += f64::from(seconds.max(0.0));
        self.state.advance_clock(self.order, seconds);
        crate::magic::tick(self, seconds);
        crate::vats::regenerate(self.order, self.state, seconds);
        // The player's own frame (`0093f374`): hardcore's needs grow; and
        // whoever sees the player trespass comes over to warn them
        // (`world::living`).
        crate::living::needs::grow(self.order, self.state);
        crate::living::trespass::update(self.order, self.state, seconds);
        // Statistics bumped since count for challenges.
        crate::more_functions::challenges::catch_up(self);
        let mut running: Vec<FormId> = self.state.running.iter().copied().collect();
        running.sort();
        for q in running {
            let Some(quest) = self.scripts.quest(self.order, q) else {
                continue;
            };
            if quest.script.is_none() {
                continue;
            }
            let own = match self.state.quest_delays.get(&q) {
                Some(&d) if d > 0.0 => Some(d),
                _ if quest.delay > 0.0 => Some(quest.delay),
                _ => None,
            };
            let delay = own.unwrap_or(QUEST_SCRIPT_DELAY);
            let first = !self.state.quest_timers.contains_key(&q);
            let timer = self.state.quest_timers.entry(q).or_insert(delay);
            *timer += seconds;
            if *timer < delay {
                continue;
            }
            let passed = std::mem::take(timer);
            // A quest with its own delay starts with its countdown at 0
            // (`005ab400`), so it runs at once, and `GetSecondsPassed`
            // gives the time gathered since its last run, or this frame's
            // when there's none (`0059c430`, `005ac1e0`): on the first run,
            // the frame's. (`VGenericTimer`, delay 0.1, so counts Doc
            // Mitchell's farewell timer down over its second and third
            // runs.)
            self.seconds_passed = if first && own.is_some() {
                seconds
            } else {
                passed
            };
            self.run_quest_gamemode(q);
        }
    }

    /// A spell cast by a script (`CastImmediateOnSelf`, `Cast`) at
    /// `target`, and at whoever its area reaches
    /// ([`crate::magic::area_targets`]). On each, every effect added goes
    /// through `MagicTarget::CheckAddEffect` (Xbox PDB, `00823210`,
    /// [`crate::magic::cast`]): by the spell's type, an identical effect
    /// from the same spell and caster is dispelled first (its
    /// `ScriptEffectFinish` runs if it had started) or, for a poison, has
    /// the new duration added.
    pub fn cast(&mut self, spell: FormId, caster: FormId, target: FormId) {
        let mut reached = vec![target];
        reached.extend(crate::magic::area_targets(
            self.order, self.state, spell, caster, target, self.sight,
        ));
        for who in reached {
            for mut e in crate::magic::cast(self.order, self.state, who, spell, caster) {
                if let Some(script) = e.script.filter(|_| e.started) {
                    self.run_effect_script(script, &mut e, "scripteffectfinish", 0.0);
                }
            }
            // Each effect starts as it's added (`CheckAddEffect` calls the
            // new effect's start): a script effect's `ScriptEffectStart`
            // runs now, not at the next update.
            let mut i = 0;
            while i < self.state.active_effects.len() {
                let e = &self.state.active_effects[i];
                if e.target == who && e.source == spell && !e.started && e.script.is_some() {
                    let mut e = self.state.active_effects.remove(i);
                    e.started = true;
                    if let Some(script) = e.script {
                        self.run_effect_script(script, &mut e, "scripteffectstart", 0.0);
                    }
                    self.state.active_effects.insert(i, e);
                }
                i += 1;
            }
        }
    }

    /// Runs a script effect's blocks of one kind (`scripteffectstart`, …)
    /// on whoever it affects, with the effect's own variables.
    pub(crate) fn run_effect_script(
        &mut self,
        script: FormId,
        effect: &mut crate::magic::ActiveEffect,
        kind: &str,
        seconds: f32,
    ) {
        let Some(s) = self.scripts.script(self.order, script) else {
            return;
        };
        if !s.blocks.iter().any(|b| b.kind == kind) {
            return;
        }
        if effect.locals == Locals::default() {
            effect.locals = Locals::new(&s);
        }
        let saved = (self.this, self.owner, self.seconds_passed);
        self.this = Some(effect.target);
        self.owner = None;
        self.seconds_passed = seconds;
        // `ScriptEffectElapsedSeconds` gives these seconds meanwhile.
        let was = self.state.more.effect_seconds.replace(seconds);
        interp::run_blocks(&s, kind, |_| true, &mut effect.locals, self);
        self.state.more.effect_seconds = was;
        (self.this, self.owner, self.seconds_passed) = saved;
    }

    /// Something happens to a reference (`onactivate`, `ontriggerenter`,
    /// `ontrigger`, `ontriggerleave`, `onload`…) caused by `who`: its
    /// script's blocks of that kind run whose argument is empty or names
    /// `who`, with `who` as the action reference (`IsActionRef`).
    pub fn run_event(&mut self, reference: FormId, kind: &str, who: FormId) {
        let order = self.order;
        let names_who = |b: &script::Block| names_who(order, b, who);
        let saved = self.state.action_ref.replace(who);
        self.run_blocks(reference, Some(reference), kind, names_who);
        self.state.action_ref = saved;
    }

    /// `speaker` finished saying a line of `topic` it was told to say with
    /// `SayTo`: its script's `SayToDone` blocks (block type 7, the block
    /// table's entry at `0118e408`) run, those naming no topic or this one.
    /// The game raises it when the line is done: the Xbox prototype names
    /// the callback `TESObjectREFR::SayToCallBack` (Xbox PDB); its PC
    /// address and exact timing aren't pinned yet (the scripts that use it
    /// chain the next line from here, as Dead Money's narrator does; the
    /// base game has over a hundred `SayToDone` blocks).
    pub fn say_to_done(&mut self, speaker: FormId, topic: FormId) {
        let order = self.order;
        let names = |b: &script::Block| match b.args.first() {
            None => true,
            Some(Arg::Word(w)) => order.form_by_editor_id(w) == Some(topic),
            Some(_) => false,
        };
        self.run_blocks(speaker, Some(speaker), "saytodone", names);
    }

    /// Several events at once, in one run of the reference's script (as
    /// `EnterTrigger` flags `OnTriggerEnter` and `OnTrigger` and runs it,
    /// `005d8cf0`): the blocks of any of the kinds whose argument is empty
    /// or names `who`, in the script's order, with `who` as the action
    /// reference.
    pub fn run_events(&mut self, reference: FormId, kinds: &[&str], who: FormId) {
        let order = self.order;
        let Some(script) = script_of(order, reference).and_then(|s| self.scripts.script(order, s))
        else {
            return;
        };
        let names_who = |b: &script::Block| names_who(order, b, who);
        let blocks: Vec<&script::Block> = script
            .blocks
            .iter()
            .filter(|b| kinds.contains(&b.kind.as_str()) && names_who(b))
            .collect();
        if blocks.is_empty() {
            return;
        }
        let saved = self.state.action_ref.replace(who);
        self.with(Some(reference), Some(reference), |runner, locals| {
            for block in blocks {
                if interp::run(&block.body, locals, runner) != Flow::Done {
                    break;
                }
            }
        });
        self.state.action_ref = saved;
    }

    /// One run of a reference's object script in game mode, as
    /// `TESObjectREFR::RunScript` (`00565870`) runs it through `Script::Run`
    /// (`005ac1e0`): its `GameMode` blocks and the blocks of the events
    /// flagged in its event list since its last run, in the script's
    /// order; the flags are then cleared (`005a8ea0`, after the run in
    /// `005e0d20`). `events` are (block kind, who it was flagged for:
    /// `None` for an event without one, as `OnLoad`); a block naming
    /// someone runs only if its event was flagged for them, one naming no
    /// one if it was flagged at all (`005a8e20` flags both). `action` is
    /// the action reference meanwhile (`IsActionRef`). Returns whether a
    /// command that may change the loaded references ran
    /// ([`Self::references_changed`]).
    pub fn run_reference_script(
        &mut self,
        reference: FormId,
        events: &[(&str, Option<FormId>)],
        action: Option<FormId>,
    ) -> bool {
        let order = self.order;
        let Some(script) = script_of(order, reference).and_then(|s| self.scripts.script(order, s))
        else {
            return false;
        };
        let names = |b: &script::Block, who: Option<FormId>| match b.args.first() {
            None => true,
            Some(Arg::Word(w)) => {
                let id = if w.eq_ignore_ascii_case("player") || w.eq_ignore_ascii_case("playerref")
                {
                    Some(PLAYER_REF)
                } else {
                    order.form_by_editor_id(w)
                };
                id.is_some() && id == who
            }
            Some(_) => false,
        };
        let blocks: Vec<&script::Block> = script
            .blocks
            .iter()
            .filter(|b| {
                b.kind == "gamemode"
                    || events
                        .iter()
                        .any(|&(kind, who)| kind == b.kind && names(b, who))
            })
            .collect();
        if blocks.is_empty() {
            return false;
        }
        let before = std::mem::take(&mut self.references_changed);
        let saved = self.state.action_ref;
        if action.is_some() {
            self.state.action_ref = action;
        }
        self.with(Some(reference), Some(reference), |runner, locals| {
            for block in blocks {
                if interp::run(&block.body, locals, runner) != Flow::Done {
                    break;
                }
            }
        });
        self.state.action_ref = saved;
        let changed = self.references_changed;
        self.references_changed = before || changed;
        changed
    }

    /// `attacker` hits `target` (a person, a creature or an object) with
    /// `weapon` (`None`: a creature's own attack, or fists). The target's
    /// script runs its `OnHit` blocks (for anyone, or naming the attacker)
    /// and `OnHitWith` blocks (for any weapon, or naming this one), as the
    /// tutorial's bottles count hits; people and creatures take the
    /// damage ([`crate::combat`]) and, if it kills them, run `OnDeath`.
    /// The damage dealt, for people and creatures. No body part is known
    /// for it: see [`Self::hit_at`].
    pub fn hit(
        &mut self,
        attacker: FormId,
        target: FormId,
        weapon: Option<&crate::combat::Weapon>,
    ) -> Option<f32> {
        self.hit_at(attacker, target, weapon, None).map(|h| h.dealt)
    }

    /// [`Self::hit`] landing on a body part (its type, found where the
    /// hit struck: `world::body_parts::BodyPartData::part_of_bone`; `None`
    /// when none was found), as the game's hits go (`009b5170` melee,
    /// `009b5650` shots, `009b6620`, `009b73d0`, `0089a760`): the
    /// weapon's damage, a critical, the armour, the part (limb damage and
    /// its multiplier: × 2 for a shot to a person's head), a sneak
    /// attack's bonus (only with a part), the health; then the part's
    /// condition (crippling, a dropped weapon). What it did, for people and
    /// creatures.
    pub fn hit_at(
        &mut self,
        attacker: FormId,
        target: FormId,
        weapon: Option<&crate::combat::Weapon>,
        part: Option<u8>,
    ) -> Option<crate::combat::Hit> {
        self.strike_at(attacker, target, weapon, part, false)
    }

    /// [`Self::hit`], perhaps a power attack (`world::combat_ai::
    /// power_attack`): a weapon's or fists' damage × `fDamagePowerAttack
    /// Bonus` (2). A creature's own attack isn't changed by it (how the game
    /// powers creatures' attacks isn't traced). No body part is known.
    pub fn strike(
        &mut self,
        attacker: FormId,
        target: FormId,
        weapon: Option<&crate::combat::Weapon>,
        power: bool,
    ) -> Option<f32> {
        self.strike_at(attacker, target, weapon, None, power)
            .map(|h| h.dealt)
    }

    /// [`Self::hit_at`], perhaps a power attack (see [`Self::strike`]).
    pub fn strike_at(
        &mut self,
        attacker: FormId,
        target: FormId,
        weapon: Option<&crate::combat::Weapon>,
        part: Option<u8>,
        power: bool,
    ) -> Option<crate::combat::Hit> {
        self.blow_at(
            attacker,
            target,
            weapon,
            part,
            crate::melee::Blow {
                power,
                special: crate::melee::Special::None,
            },
        )
    }

    /// [`Self::strike_at`] with a melee blow's particulars
    /// (`world::melee::Blow`): a power attack, an unarmed uppercut or
    /// cross (`00899200` → `0089a760`).
    pub fn blow_at(
        &mut self,
        attacker: FormId,
        target: FormId,
        weapon: Option<&crate::combat::Weapon>,
        part: Option<u8>,
        blow: crate::melee::Blow,
    ) -> Option<crate::combat::Hit> {
        let power = blow.power;
        let order = self.order;
        let names = |b: &script::Block, id: Option<FormId>| match b.args.first() {
            None => true,
            Some(Arg::Word(w)) => {
                let named =
                    if w.eq_ignore_ascii_case("player") || w.eq_ignore_ascii_case("playerref") {
                        Some(PLAYER_REF)
                    } else {
                        order.form_by_editor_id(w)
                    };
                named.is_some() && named == id
            }
            Some(_) => false,
        };
        // A ghost (`SetGhost`) isn't hit at all: the actor's hit handling
        // (`00899cb0`) and its reaction (`008987f0`) return at once for one.
        if crate::more_functions::is_ghost(self.state, target) {
            return None;
        }
        let weapon_id = weapon.map(|w| w.form_id);
        let saved = self.state.action_ref.replace(attacker);
        self.run_blocks(target, Some(target), "onhit", |b| names(b, Some(attacker)));
        self.run_blocks(target, Some(target), "onhitwith", |b| names(b, weapon_id));
        self.state.action_ref = saved;
        let kind = base_of(order, target)
            .and_then(|b| order.get(b))
            .map(|r| r.entry.header.kind)?;
        if kind.as_bytes() != b"NPC_" && kind != CREA {
            return None;
        }
        // The game's order (`009b5170`, `009b5650`, `findings\hits.md`):
        // the weapon's damage (a creature's bite, or fists), a critical
        // adding the weapon's critical damage, the armour, then a sneak
        // attack's multiplier.
        // Fists' power attack: a bonus multiplier after the armour
        // (`world::combat::fists_power_bonus`); a creature's own attack
        // has none.
        let mut fists_bonus = 0.0;
        // Bare fists' fatigue damage, half their damage (`00646310`,
        // `world::fatigue`).
        let mut fatigue = 0.0;
        let mut damage = match weapon {
            Some(w) => crate::combat::weapon_damage(order, self.state, attacker, Some(w), power),
            None => crate::combat::creature_damage(order, attacker).unwrap_or_else(|| {
                fists_bonus = crate::combat::fists_power_bonus(order, power);
                let fists = crate::combat::weapon_damage(order, self.state, attacker, None, false);
                fatigue = crate::fatigue::fists_fatigue(order, self.state, attacker, target, fists);
                fists
            }),
        };
        // V.A.T.S.'s melee moves and automatic melee weapons (`world::vats`).
        damage *= crate::vats::attack_damage_mult(order, self.state, attacker, weapon);
        // A teammate's Nerve once more for a melee, unarmed or creature
        // attack (`009b5170`; on top of `00644ce0`'s for weapons and
        // fists); a shot's damage (`009b5650`) has it once.
        if weapon.map_or(true, |w| w.is_melee()) {
            damage *= crate::companions::nerve(order, self.state, attacker);
        }
        let sneak = attacker == PLAYER_REF
            && self.state.player_sneaking
            && self
                .facts()
                .detection(target, PLAYER_REF)
                .map_or(true, |v| v < 1);
        let roll = self.state.roll() % 1000;
        let critical =
            crate::combat::critical(order, self.state, attacker, weapon, target, sneak, roll);
        if critical {
            damage +=
                crate::combat::critical_damage(order, self.state, attacker, weapon, target, damage);
        }
        let ammo = weapon.and_then(|w| w.ammo_in_use(order, self.state, attacker));
        let armoured = crate::combat::armour_hit(
            order,
            self.state,
            damage,
            Some((attacker, weapon_id)),
            target,
            ammo,
        );
        // A blocked hit is flagged (`009b5a30`, `00407e00(1, 1)`), for the
        // blocker's block-hit animation and counter-attack timer.
        if crate::melee::block_bonus(order, self.state, attacker, weapon_id, target).is_some() {
            self.state.blocked_hits.push(target);
        }
        let vats_mult = crate::vats::player_damage_mult(order, self.state, target);
        let after_armour = armoured.damage * vats_mult;
        // The fatigue damage through the armour, the ammunition's added
        // (`009b5a30`, `world::fatigue::through_armour`).
        fatigue = crate::fatigue::through_armour(
            order,
            fatigue,
            ammo,
            weapon.is_some(),
            damage,
            armoured.damage,
        ) * vats_mult;
        // A hit doing fatigue damage wears no armour (`009b5a30`).
        let armour_damage = if fatigue > 0.0 {
            0.0
        } else {
            armoured.armour_damage
        };
        // The player's armour wears (`0089a760`).
        crate::combat::wear_armour(order, self.state, target, part, armour_damage * vats_mult);
        // The part it landed on: limb damage from the damage after armour,
        // and the hit's multiplier (`009b6620`).
        let at = crate::body_parts::part_hit(order, self.state, target, weapon, part, after_armour);
        let mut dealt = at.health_damage;
        // The multiplier last (`009b73d0`): the part's (or a fists' power
        // attack's bonus, the larger: `009b7840`), a sneak attack's bonus
        // when it's at least 1, then the damage × it.
        let mut multiplier = at.multiplier.max(fists_bonus);
        if multiplier > 0.0 {
            if multiplier >= 1.0 && sneak {
                multiplier *=
                    crate::combat::sneak_multiplier(order, weapon.map_or(true, |w| w.is_melee()));
            }
            dealt *= multiplier;
            // An unarmed blow's fatigue takes it too (`009b73d0`: attacker
            // not a creature, the hit's skill Unarmed).
            if weapon.map_or(true, |w| w.skill == crate::combat::av::UNARMED)
                && !crate::combat::is_creature(order, attacker)
            {
                fatigue *= multiplier;
            }
        }
        self.state
            .hit_location
            .insert(target, part.map_or(-1, i32::from));
        // The part's condition (`0089a760`).
        let hurt = crate::body_parts::hurt_part(
            order,
            self.state,
            target,
            &at,
            critical,
            (attacker, weapon_id),
            blow.special,
        );
        // The player hitting a person who isn't fighting them: an assault,
        // unless a friend or ally forgives it (`world::crime::assault`);
        // killing them a murder.
        let was_hostile = self.state.combat.get(&target) == Some(&attacker);
        let person = kind.as_bytes() == b"NPC_";
        let mut fights_back = attacker != target;
        if attacker == PLAYER_REF && person && !was_hostile {
            fights_back &= crate::crime::assault(order, self.state, target);
        }
        let was_alive = !self.state.dead.contains(&target);
        let killed = crate::combat::hurt(order, self.state, target, f64::from(dealt), attacker);
        // Then the fatigue, while above `fMinimumFatigue` (`0089d6f0`).
        if !crate::fatigue::take_hit(order, self.state, target, fatigue) {
            fatigue = 0.0;
        }
        // A critical hit's effect: the energy weapons' disintegration and
        // goo on the killed (`0089a760`, `world::combat::critical_effect`).
        if critical {
            let dead_now = self.state.dead.contains(&target);
            crate::combat::critical_effect(order, self.state, target, weapon, was_alive, dead_now);
        }
        // A V.A.T.S. special's spell (the Mauler's knockdown), cast by the
        // one struck on themselves (`world::vats::special_effect`).
        if let Some(spell) = crate::vats::special_effect(order, self.state, attacker, weapon) {
            crate::magic::add_spell(order, self.state, target, spell, target, false);
        }
        // The attacker's "Knockdown Chance" perks (Super Slam,
        // `world::melee::knockdown_chance`).
        let chance = crate::melee::knockdown_chance(order, self.state, attacker, weapon_id);
        let knocked_down = chance > 0.0 && {
            let roll = (self.state.roll() % 1_000_000) as f32 / 1_000_000.0;
            crate::melee::knocks_down(chance, roll)
        };
        self.state
            .hits_taken
            .push((target, part.map_or(-1, i32::from), killed));
        if killed {
            // The part the killing blow landed on (`GetKillingBlowLimb`; the
            // game keeps it with the dismembered limbs, when it writes it
            // there isn't traced).
            self.state
                .killing_blow_limb
                .insert(target, part.map_or(-1, i32::from));
            // How they died (`GetCauseofDeath`), by what struck.
            let cause = crate::more_functions::actors::cause_of(order, weapon);
            crate::more_functions::actors::record_cause(self.state, target, cause);
            if person
                && !was_hostile
                && (attacker == PLAYER_REF || self.state.teammates.contains(&attacker))
            {
                crate::crime::murder(order, self.state, target, attacker);
            }
            self.run_event(target, "ondeath", attacker);
        } else if fights_back {
            self.attacked(target, attacker);
        }
        Some(crate::combat::Hit {
            dealt,
            part,
            critical,
            multiplier,
            hurt,
            knocked_down,
            fatigue,
        })
    }

    /// `target`, hurt and alive, answers `attacker`. The player's hits (past
    /// `world::crime::assault`) and the player hurt keep the viewer's rule:
    /// the one hurt fights back unless already fighting (for the player that
    /// entry is only their "in combat" state). Anyone else hurt by someone
    /// other than the player follows `Actor::AttackedBy` (Xbox PDB,
    /// [`crate::combat_ai::attacked_by`]): friends and allies tolerate hits,
    /// and stray shots don't start fights.
    fn attacked(&mut self, target: FormId, attacker: FormId) {
        if attacker == PLAYER_REF || target == PLAYER_REF {
            self.state.combat.entry(target).or_insert(attacker);
            return;
        }
        let answer = crate::combat_ai::attacked_by(self.order, self.state, target, attacker);
        if answer.enters_combat {
            self.state.combat.entry(target).or_insert(attacker);
        }
        if answer.new_target {
            let list = self.state.hit_targets.entry(target).or_default();
            if !list.contains(&attacker) {
                list.push(attacker);
            }
        }
    }

    /// An explosion made by `source` (with `weapon`, its maker's weapon)
    /// hurts `target` by `damage` (its damage after falloff,
    /// `world::explosions`), as `009b5770` builds the hit (flag 0x2000) and
    /// `0089a760` takes it: the target's `OnHit` / `OnHitWith` blocks, then
    /// the armour (`009b5a30`), a critical (`009b7060`, adding the weapon's
    /// critical damage after the armour), no body part (so no multiplier,
    /// `009b73d0`), the health, `OnDeath`, and the hurt fighting back. The
    /// limbs the explosion reaches (`009afcc0`, `009b1720`) aren't damaged
    /// here. What it did, for people and creatures.
    pub fn explosion_hit(
        &mut self,
        source: Option<FormId>,
        target: FormId,
        weapon: Option<&crate::combat::Weapon>,
        damage: f32,
    ) -> Option<crate::combat::Hit> {
        let order = self.order;
        if crate::more_functions::is_ghost(self.state, target) {
            return None;
        }
        let attacker = source.unwrap_or(target);
        let weapon_id = weapon.map(|w| w.form_id);
        let names = |b: &script::Block, id: Option<FormId>| match b.args.first() {
            None => true,
            Some(Arg::Word(w)) => {
                let named =
                    if w.eq_ignore_ascii_case("player") || w.eq_ignore_ascii_case("playerref") {
                        Some(PLAYER_REF)
                    } else {
                        order.form_by_editor_id(w)
                    };
                named.is_some() && named == id
            }
            Some(_) => false,
        };
        let saved = self.state.action_ref.replace(attacker);
        self.run_blocks(target, Some(target), "onhit", |b| names(b, source));
        self.run_blocks(target, Some(target), "onhitwith", |b| names(b, weapon_id));
        self.state.action_ref = saved;
        let kind = base_of(order, target)
            .and_then(|b| order.get(b))
            .map(|r| r.entry.header.kind)?;
        if kind.as_bytes() != b"NPC_" && kind != CREA {
            return None;
        }
        let mut dealt = crate::combat::hit_through_armour(
            order,
            self.state,
            damage,
            source.map(|s| (s, weapon_id)),
            target,
            None,
        ) * crate::vats::player_damage_mult(order, self.state, target);
        let critical = source.is_some_and(|s| {
            let roll = self.state.roll() % 1000;
            crate::combat::critical(order, self.state, s, weapon, target, false, roll)
        });
        if let (true, Some(s)) = (critical, source) {
            dealt += crate::combat::critical_damage(order, self.state, s, weapon, target, dealt);
        }
        self.state.hit_location.insert(target, -1);
        let was_hostile = source.is_some_and(|s| self.state.combat.get(&target) == Some(&s));
        let person = kind.as_bytes() == b"NPC_";
        let mut fights_back = source.is_some_and(|s| s != target);
        if source == Some(PLAYER_REF) && person && !was_hostile {
            fights_back &= crate::crime::assault(order, self.state, target);
        }
        let was_alive = !self.state.dead.contains(&target);
        let killed = crate::combat::hurt(order, self.state, target, f64::from(dealt), attacker);
        // The explosion's hit carries its weapon's critical effect too
        // (`009b5770` builds it with the weapon, `009b7060`, `0089a760`).
        if critical {
            let dead_now = self.state.dead.contains(&target);
            crate::combat::critical_effect(order, self.state, target, weapon, was_alive, dead_now);
        }
        if killed {
            self.state.killing_blow_limb.insert(target, -1);
            if let Some(s) = source.filter(|&s| {
                person && !was_hostile && (s == PLAYER_REF || self.state.teammates.contains(&s))
            }) {
                crate::crime::murder(order, self.state, target, s);
            }
            self.run_event(target, "ondeath", attacker);
        } else if let (true, Some(s)) = (fights_back, source) {
            self.attacked(target, s);
        }
        Some(crate::combat::Hit {
            dealt,
            part: None,
            critical,
            multiplier: 0.0,
            hurt: None,
            knocked_down: false,
            fatigue: 0.0,
        })
    }

    /// A menu is open: each running quest's `MenuMode` blocks for it (or
    /// for any menu) run.
    pub fn menu_mode(&mut self, menu: u16) {
        let mut running: Vec<FormId> = self.state.running.iter().copied().collect();
        running.sort();
        // The menu counts as open while its blocks run (`MenuMode n` in
        // them is true).
        let was = self.state.more.menu_open.replace(menu);
        for q in running {
            self.run_blocks(q, None, "menumode", |b| match b.args.first() {
                None => true,
                Some(Arg::Number(n)) => *n as u16 == menu,
                Some(_) => false,
            });
        }
        self.state.more.menu_open = was;
    }

    /// `SetStage`: the stage becomes done (and current, if it's past the
    /// current one), the quest runs, and the stage's first log entry whose
    /// conditions pass gives the journal text, completes or fails the
    /// quest by its flags, and runs its result script. Guesses: a stage
    /// the quest doesn't have does nothing; a stage already done does
    /// nothing unless the quest allows repeated stages; the current stage
    /// never goes down.
    pub fn set_stage(&mut self, quest_id: FormId, stage: u16) -> bool {
        if self.depth >= MAX_DEPTH {
            return false;
        }
        let Some(quest) = self.scripts.quest(self.order, quest_id) else {
            return false;
        };
        let Some(st) = quest.stage(stage) else {
            return false;
        };
        let done = self.state.stages_done.contains(&(quest_id, stage));
        if done && quest.flags & quest::ALLOW_REPEATED_STAGES == 0 {
            return false;
        }
        self.state.running.insert(quest_id);
        let current = self.state.stages.entry(quest_id).or_insert(0);
        *current = (*current).max(stage);
        self.state.stages_done.insert((quest_id, stage));
        let entry = {
            let facts = self.facts();
            st.entries
                .iter()
                .find(|e| facts.conditions_pass(&e.conditions, PLAYER_REF, PLAYER_REF))
                .cloned()
        };
        if let Some(e) = entry {
            if let Some(text) = e.text {
                self.state.events.push(Event::Journal {
                    quest: quest_id,
                    text,
                });
            }
            if e.flags & quest::COMPLETES_QUEST != 0 {
                // `0060fb60` → `0060ca30` (announced when newly completed);
                // no longer the active quest.
                crate::quest_text::complete(self.state, quest_id);
                crate::quest_targets::quest_ended(self.state, quest_id);
            }
            if e.flags & quest::FAILS_QUEST != 0 {
                // `0060caf0`: failed (and completed) unless it was either.
                crate::quest_text::fail(self.state, quest_id);
                crate::quest_targets::quest_ended(self.state, quest_id);
            }
            if let Some(source) = e.script {
                self.depth += 1;
                self.run_source(&source, None, Some(quest_id));
                self.depth -= 1;
            }
        }
        true
    }

    /// An argument worked out by its parameter type: records from editor
    /// IDs or `ref` variables, numbers from variables and globals, actor
    /// values by name.
    fn argument(&mut self, arg: &Arg, kind: Option<u8>, locals: &Locals) -> Value {
        match arg {
            Arg::Number(n) => Value::Number(*n),
            Arg::Str(s) => Value::Text(s.clone()),
            Arg::Word(w) => match kind {
                Some(5) => script::actor_value(w)
                    .map_or(Value::Text(w.clone()), |i| Value::Number(f64::from(i))),
                Some(k) if script::param_is_form(k) => match locals.get(w) {
                    Some(v) => Value::Form(FormId(v as u32)),
                    None => self.form(w).map_or(Value::Text(w.clone()), Value::Form),
                },
                Some(1 | 2 | 23) => match locals.get(w) {
                    Some(v) => Value::Number(v),
                    None => Value::Number(self.resolve(w).unwrap_or(0.0)),
                },
                // A sex: compiled as 0 male, 1 female.
                Some(18) if w.eq_ignore_ascii_case("male") => Value::Number(0.0),
                Some(18) if w.eq_ignore_ascii_case("female") => Value::Number(1.0),
                // A critical stage (parameter type 0x37, `SetCriticalStage`
                // `DisintegrateStart`): its number in the game's name
                // table at `0119bbb0`.
                Some(55) => crate::more_functions::critical_stage_number(w)
                    .map_or(Value::Text(w.clone()), |n| Value::Number(f64::from(n))),
                _ => Value::Text(w.clone()),
            },
        }
    }

    /// Who a message box shown now belongs to, and who asks
    /// `GetButtonPressed` (`005b4630`, `005b4a80`): the reference the script
    /// runs on (its form ID, `0084e3a0` reads form +0xc), unless there is
    /// none or it's a temporary one (form flag 0x4000, `004077c0`), then
    /// the script itself (a quest's script, say). References made while
    /// playing stand for the temporary ones (that `PlaceAtMe` sets 0x4000
    /// isn't traced). A quest stage's result script counts as its quest's
    /// script here (which form the game hands for it isn't traced). A
    /// console line has neither: form 0.
    pub fn message_owner(&self) -> FormId {
        if let Some(this) = self
            .this
            .filter(|t| !self.state.more.placed.refs.contains_key(t))
        {
            return this;
        }
        self.owner
            .and_then(|o| script_of(self.order, o))
            .unwrap_or(FormId(0))
    }

    /// `ShowMessage`: the message with the script's values filled in (its
    /// `%.0f`-style places). A message box (`DNAM` 0x01) gets the buttons
    /// whose conditions pass, asked about the player (a guess at whom they
    /// ask), numbered as stored; one without buttons gets "OK" (a guess).
    fn message(&mut self, id: FormId, values: &[f64]) {
        let Some(rr) = self.order.get(id).filter(|r| r.entry.header.kind == MESG) else {
            return;
        };
        let Ok(record) = rr.record() else {
            return;
        };
        let title = record
            .get(FULL)
            .map(|s| s.zstring())
            .filter(|t| !t.is_empty());
        let text = fill_values(
            &record.get(DESC).map(|s| s.zstring()).unwrap_or_default(),
            values,
        );
        // `005b4630`: once the text is made, any button still waiting is
        // dropped (`005b4940`: `0118c684` = -1) and the box's buttons
        // belong to whoever shows it (`011cac64`), box or corner message.
        self.state.button = None;
        self.state.button_owner = Some(self.message_owner());
        let message_box = record
            .get(DNAM)
            .and_then(|s| s.data.first())
            .is_some_and(|f| f & 1 != 0);
        let mut buttons = Vec::new();
        if message_box {
            let mut all: Vec<(String, Vec<Condition>)> = Vec::new();
            for sub in &record.subrecords {
                if sub.kind == ITXT {
                    all.push((sub.zstring(), Vec::new()));
                } else if sub.kind == CTDA {
                    if let Some((_, conditions)) = all.last_mut() {
                        conditions.extend(crate::dialogue::read_condition(&rr, &sub.data));
                    }
                }
            }
            let facts = self.facts();
            buttons = all
                .into_iter()
                .enumerate()
                .filter(|(_, (_, c))| facts.conditions_pass(c, PLAYER_REF, PLAYER_REF))
                .map(|(i, (label, _))| (i, label))
                .collect();
            if buttons.is_empty() {
                buttons.push((0, "OK".to_string()));
            }
        }
        // A corner message shows the message's own picture (`005b4630`:
        // its icon's path to `QueueUIMessage`, type 0).
        let icon = crate::message_icon::of_message(self.order, id);
        self.state.events.push(Event::Message {
            title,
            text,
            buttons,
            icon,
        });
    }

    /// The functions that change the game. `None` for the rest.
    fn change(&mut self, name: &str, target: Option<FormId>, args: &[Value]) -> Option<f64> {
        let arg = |i: usize| args.get(i).cloned().unwrap_or(Value::Number(0.0));
        let events = &mut self.state.events;
        match name {
            "SetStage" => return Some(flag(self.set_stage(arg(0).form(), arg(1).number() as u16))),
            "StartQuest" => {
                self.state.running.insert(arg(0).form());
            }
            "StopQuest" => {
                self.state.running.remove(&arg(0).form());
            }
            "CompleteQuest" => {
                // `005c7280` → `0060ca30`: completing a quest not completed
                // announces it (`0077a480`), which stops it being the
                // active quest.
                crate::quest_text::complete(self.state, arg(0).form());
            }
            // The objective's state (`005d7c20`, `005d7d30`, `005ec5d0`: 0
            // neither, 1 shown, 2 done unseen, 3 shown and done): showing
            // one not shown makes it 1 (and starts its quest); completing
            // one not done keeps whether it's shown (+2); turning either
            // off makes it 0. An objective the quest doesn't have is an
            // error: nothing changes.
            "SetObjectiveDisplayed" | "SetObjectiveCompleted" => {
                let (quest, index) = (arg(0).form(), arg(1).number() as i32);
                let exists = self
                    .scripts
                    .quest(self.order, quest)
                    .is_some_and(|q| q.objective(index).is_some());
                if !exists {
                    return Some(0.0);
                }
                let on = arg(2).number() != 0.0;
                let shown = self.state.objectives.contains_key(&(quest, index));
                let done = crate::script_functions::objective_completed(self.state, quest, index);
                let new = match (name == "SetObjectiveCompleted", on) {
                    (_, false) => Some(0),
                    (true, true) if !done => Some(if shown { 3 } else { 2 }),
                    (false, true) if !shown => Some(1),
                    _ => None,
                };
                if let Some(new) = new {
                    crate::script_functions::set_objective(self, quest, index, new);
                }
            }
            "ShowMessage" => {
                let values: Vec<f64> = args.iter().skip(1).map(Value::number).collect();
                self.message(arg(0).form(), &values);
            }
            // `005b4a80`: the button only for the owner the box was shown
            // by; anyone else gets -1 and the button stays for the owner.
            "GetButtonPressed" => {
                let mine = self.state.button_owner == Some(self.message_owner());
                return Some(match self.state.button.filter(|&b| b >= 0 && mine) {
                    Some(b) => {
                        self.state.button = None;
                        self.state.button_owner = None;
                        f64::from(b)
                    }
                    None => -1.0,
                });
            }
            "SexChange" => {
                if target? != PLAYER_REF {
                    return None;
                }
                self.state.player_female = Some(arg(0).number() != 0.0);
            }
            // `VCG01` stage 15 ("player chooses his name") uses the second.
            "ShowNameMenu" | "GetPlayerName" => {
                events.push(Event::CharacterMenu(CharacterMenu::Name))
            }
            "ShowLoveTesterMenuParams" => {
                events.push(Event::CharacterMenu(CharacterMenu::Special {
                    points: arg(0).number() as u32,
                }))
            }
            // `005d7350`: the second number (1 when left out) starts the
            // menu with the player's tags picked.
            "SetTagSkills" => events.push(Event::CharacterMenu(CharacterMenu::TagSkills {
                count: arg(0).number() as u32,
                preselect: args.get(1).map_or(true, |a| a.number() == 1.0),
            })),
            "ShowBarterMenu" => events.push(Event::Barter(target?)),
            // `005deb10` → `00704fc0` → `00726ff0`: the crafting menu for
            // the person the script runs on, or for the speaker of the
            // talking activator it runs on (its base's +0x90); the category
            // is optional (0). No one: nothing ("Recipe menu called with
            // NULL vendor!"). Its number (1077) is open for `MenuMode`.
            "ShowRecipeMenu" => {
                let r = target?;
                let actor = if crate::more_functions::is_actor(self.order, self.state, r) {
                    Some(r)
                } else {
                    crate::more_functions::placed::base_now(self.order, self.state, r)
                        .filter(|&b| {
                            self.order
                                .get(b)
                                .is_some_and(|b| b.entry.header.kind.as_bytes() == b"TACT")
                        })
                        .and_then(|b| self.state.more.speakers.get(&b).copied())
                };
                let events = &mut self.state.events;
                match actor {
                    Some(actor) => {
                        events.push(Event::RecipeMenu {
                            actor,
                            category: arg(0).form(),
                        });
                        events.push(Event::Menu(crate::crafting::RECIPE_MENU));
                    }
                    None => println!("Recipe menu called with NULL vendor!  Oh, noes!"),
                }
                return Some(1.0);
            }
            // `005d5200`: on a person or creature only (vtable +0x100), the
            // merchants' repair menu for them (`00704690` → `007b7570`).
            // `005cf250` → `00741060` (`CaravanMenu::Create`): on someone
            // other than the player; with fewer than 30 cards the player's
            // told so (`sCardCountText`) instead.
            // `005cf040` / `005cf0f0` / `005cf1a0`: the game's `Create`
            // (its checks are the viewer's, with the anti-cheat clock). The
            // parameterless forms read no casino (an unset local in the
            // game; no script uses them).
            // A form that isn't a casino (`CSNO`) is only reported.
            "ShowSlotMachineMenuParams" | "ShowBlackJackMenuParams" | "ShowRouletteMenuParams" => {
                let casino = arg(0).form();
                let is_casino = self
                    .order
                    .get(casino)
                    .is_some_and(|r| r.entry.header.kind.as_bytes() == b"CSNO");
                if casino.0 != 0 && !is_casino {
                    println!(
                        "Invalid EditorFormID used in script {} -- is not a valid EditorFormID",
                        name.trim_end_matches("Params")
                    );
                } else if casino.0 != 0 {
                    let game = match name {
                        "ShowSlotMachineMenuParams" => crate::casino::Game::Slots,
                        "ShowBlackJackMenuParams" => crate::casino::Game::Blackjack,
                        _ => crate::casino::Game::Roulette,
                    };
                    events.push(Event::Menu(game.menu()));
                    events.push(Event::Casino {
                        game,
                        casino,
                        min_bet: arg(1).number() as i32,
                        max_bet: arg(2).number() as i32,
                        min_winnings: if game == crate::casino::Game::Blackjack {
                            0
                        } else {
                            arg(3).number() as i32
                        },
                    });
                }
            }
            // `005ded40` (no vanilla script calls it).
            "SetCasinoWinningsLevel" => {
                crate::casino::set_winnings_level(
                    self.order,
                    self.state,
                    arg(0).form(),
                    arg(1).number() as i32,
                );
            }
            // `005d4a40`: the PC's shared `return 1` (the Xbox kept a
            // cheat level nothing reads).
            "SetCasinoCheatLevel" => {}
            "ShowCaravanMenu" => {
                let npc = target?;
                let deck = arg(0).form();
                if npc == PLAYER_REF || deck.0 == 0 {
                    return Some(0.0);
                }
                let c = &self.state.caravan;
                if c.inactive.len() + c.active.len() < crate::caravan::MIN_DECK {
                    let text = crate::scripting::game_setting_text(self.order, "sCardCountText")
                        .unwrap_or_else(|| {
                            "You must have at least 30 cards to play Caravan.".into()
                        });
                    events.push(Event::Message {
                        title: None,
                        text,
                        buttons: Vec::new(),
                        icon: None,
                    });
                } else {
                    events.push(Event::Caravan {
                        npc,
                        deck,
                        difficulty: arg(1).number() as i32,
                        share: arg(2).number() as f32,
                    });
                }
            }
            // `005da630`: `TutorialMenu::Create(message, 0)` (the shared
            // `005d4a40` check is the PC's `return 1`).
            "ShowTutorialMenu" => events.push(Event::TutorialMenu(arg(0).form())),
            "ShowRepairMenu" => {
                let vendor = target?;
                if crate::script_functions::is_actor(self.order, vendor) {
                    events.push(Event::RepairServices(vendor));
                }
            }
            // `VCG01TestSCRIPT` tags the exam's picks this way (slots 0 to
            // 2); the tag menu then starts with them.
            "SetPlayerTagSkill" => {
                let skill = arg(0).number() as u16;
                let slot = arg(1).number() as u8;
                if let Some(old) = self.state.tag_slots.insert(slot, skill) {
                    self.state.tag_skills.remove(&old);
                }
                self.state.tag_skills.insert(skill);
            }
            "ShowTraitMenu" => {
                let max = crate::chargen::max_traits(self.order);
                self.state
                    .events
                    .push(Event::CharacterMenu(CharacterMenu::Traits { max }));
            }
            "AddItem" | "RemoveItem" => {
                let holder = target?;
                self.state.stock(self.order, holder);
                let count = arg(1).number() as i32;
                let form = arg(0).form();
                let leveled = self
                    .order
                    .get(form)
                    .is_some_and(|r| crate::leveled::is_leveled(r.entry.header.kind));
                if name == "AddItem" && leveled {
                    // A leveled list gives what it picks (the tutorial's
                    // `condnvvarmintrifleloot`).
                    let level = self.state.player_level;
                    let picked =
                        crate::leveled::resolve(self.order, form, count, level, &mut || {
                            self.state.roll()
                        });
                    for (item, n) in picked {
                        *self.state.items.entry((holder, item)).or_insert(0) += n;
                        self.state.added(self.order, holder, item, n);
                    }
                    return Some(0.0);
                }
                let n = self.state.items.entry((holder, form)).or_insert(0);
                *n = if name == "AddItem" {
                    *n + count
                } else {
                    (*n - count).max(0)
                };
                if name == "AddItem" {
                    self.state.added(self.order, holder, form, count);
                }
            }
            // One weapon in hand; clothes take off what's on the same slots.
            "EquipItem" => {
                let who = target?;
                let item = arg(0).form();
                self.state.equip(self.order, who, item);
                // The no-unequip flag (`005d0060`: set or cleared on the
                // worn entry, `0041ab70`).
                if self.state.is_equipped(who, item) {
                    if args.get(1).is_some_and(|a| a.number() != 0.0) {
                        self.state.equip_locked.insert((who, item));
                    } else {
                        self.state.equip_locked.remove(&(who, item));
                    }
                }
            }
            "UnequipItem" => {
                let who = target?;
                let item = arg(0).form();
                self.state.unequip_item(self.order, who, item);
                self.state.equip_locked.remove(&(who, item));
            }
            "KillActor" => {
                let who = target?;
                let by = args.first().map_or(who, Value::form);
                // With a limb (`005be2a0` → `008b51b0`), the cause given (or
                // −1) is kept as the cause of death.
                let limb = args.get(1).map_or(-1, |v| v.number() as i32);
                if limb != -1 {
                    let cause = args.get(2).map_or(-1, |v| v.number() as i32);
                    crate::more_functions::actors::record_cause(self.state, who, cause);
                }
                let full = crate::combat::max_health(self.order, self.state, who).unwrap_or(1.0);
                // `KillActor` (`005be2a0`) kills through the actor's death
                // routine (`0089d900`), the one a fatal hit takes, so its
                // `OnDeath` runs as for any death here (where in that
                // routine the event is raised isn't pinned down).
                if crate::combat::hurt(self.order, self.state, who, full.max(1.0) * 10.0, by) {
                    // No hit, and an attacker only when one is given
                    // (`005be2a0` reads it from its first parameter, none
                    // by default).
                    self.state.deaths.insert(
                        who,
                        crate::combat::DeathStart {
                            killer: args.first().map(Value::form),
                            hit: false,
                        },
                    );
                    self.run_event(who, "ondeath", by);
                }
            }
            "ResurrectActor" => {
                let who = target?;
                self.state.dead.remove(&who);
                self.state.deaths.remove(&who);
                self.state.damage.remove(&who);
            }
            "StartCombat" => {
                self.state.combat.insert(target?, arg(0).form());
            }
            // Faction relations (see `crate::factions`): each side's flag
            // 1 makes it neutral (`SetEnemy`) or a friend (`SetAlly`).
            "SetEnemy" | "SetAlly" => {
                let (a, b) = (arg(0).form(), arg(1).form());
                let enemy = name == "SetEnemy";
                let code = |flag: f64| match (enemy, flag != 0.0) {
                    (true, false) => 1,
                    (true, true) => 0,
                    (false, false) => 2,
                    (false, true) => 3,
                };
                self.state
                    .faction_relations
                    .insert((a, b), code(arg(2).number()));
                self.state
                    .faction_relations
                    .insert((b, a), code(arg(3).number()));
            }
            "AddToFaction" | "SetFactionRank" => {
                let who = target?;
                let rank = arg(1).number().clamp(-1.0, 127.0) as i8;
                self.state
                    .faction_changes
                    .insert((who, arg(0).form()), rank);
            }
            "RemoveFromFaction" => {
                let who = target?;
                self.state.faction_changes.insert((who, arg(0).form()), -1);
            }
            "StopCombat" => {
                self.state.combat.remove(&target?);
            }
            // `005d09e0` (`world::ai::flee::force`): an optional cell, then
            // an optional reference to flee to.
            "ForceFlee" => {
                let who = target?;
                let cell = args.first().map(Value::form).filter(|f| f.0 != 0);
                let to = args.get(1).map(Value::form).filter(|f| f.0 != 0);
                crate::ai::flee::force(self.state, who, cell, to);
            }
            // `005b5690`: the same as `RemoveAllTypedItems` for every type
            // (`script_functions::remove_all`): the player's quest items
            // stay.
            "RemoveAllItems" => {
                let holder = target?;
                let to = args.first().map(Value::form).filter(|f| f.0 != 0);
                crate::script_functions::remove_all(self.order, self.state, holder, to, -1, None);
            }
            "Enable" | "Disable" | "MarkForDelete" => {
                let r = target?;
                let enable = name == "Enable";
                self.state.disabled.insert(r, !enable);
                events.push(Event::Enable(r, enable));
            }
            "AddTopic" => {
                self.state.topics.insert(arg(0).form());
            }
            "AddNote" => {
                self.state.notes.insert(arg(0).form());
            }
            "SetDestroyed" => {
                let r = target?;
                if arg(0).number() != 0.0 {
                    self.state.destroyed.insert(r);
                } else {
                    self.state.destroyed.remove(&r);
                }
            }
            "AddPerk" | "RemovePerk" => {
                if target? != PLAYER_REF {
                    return None;
                }
                if name == "AddPerk" {
                    crate::perks::add(self.order, self.state, arg(0).form());
                } else {
                    crate::perks::remove(self.order, self.state, arg(0).form());
                }
            }
            "StartConversation" | "SayTo" => {
                let speaker = target?;
                events.push(Event::Talk {
                    speaker,
                    to: arg(0).form(),
                    topic: args.get(1).map(Value::form).filter(|f| f.0 != 0),
                    conversation: name == "StartConversation",
                });
            }
            "DisablePlayerControls" | "EnablePlayerControls" => {
                let enable = name == "EnablePlayerControls";
                let defaults = if enable {
                    controls::ENABLE_DEFAULTS
                } else {
                    controls::DISABLE_DEFAULTS
                };
                for (i, default) in defaults.iter().enumerate() {
                    let flagged = args.get(i).map_or(*default, |v| v.number() != 0.0);
                    if flagged {
                        self.state.controls_off[i] = !enable;
                    }
                }
                self.state.events.push(Event::PlayerControls(enable));
            }
            // Whether any of the controls asked about (the same defaults as
            // `DisablePlayerControls`) is off.
            "GetPlayerControlsDisabled" => {
                let off = controls::DISABLE_DEFAULTS
                    .iter()
                    .enumerate()
                    .any(|(i, default)| {
                        args.get(i).map_or(*default, |v| v.number() != 0.0)
                            && self.state.controls_off[i]
                    });
                return Some(flag(off));
            }
            // The table's names; scripts write `MoveTo`, its short name.
            // Optional x, y, z offsets follow the marker.
            "MoveToMarker" | "MoveToMarkerWithFade" => {
                let what = target?;
                let to = arg(0).form();
                let (space, cell, mut position, heading) = self.state.place(self.order, to)?;
                for (i, p) in position.iter_mut().enumerate() {
                    *p += arg(i + 1).number() as f32;
                }
                if what == PLAYER_REF {
                    self.state.player_world = (space != cell).then_some(space);
                    self.state.player_cell = Some(cell);
                    self.state.player_position = Some(position);
                    crate::script_functions::player_moved(self.state);
                } else {
                    self.state.spaces.insert(what, (space, cell));
                    self.state.positions.insert(what, (position, heading));
                }
                self.state.events.push(Event::MoveTo { what, to });
            }
            "SetQuestDelay" => {
                self.state
                    .quest_delays
                    .insert(arg(0).form(), arg(1).number() as f32);
            }
            "ApplyImageSpaceModifier" | "RemoveImageSpaceModifier" => {
                let m = arg(0).form();
                let on = name == "ApplyImageSpaceModifier";
                self.state.modifiers.retain(|x| *x != m);
                if on {
                    self.state.modifiers.push(m);
                }
                events.push(Event::ImageSpace(m, on));
            }
            "ForceWeather" | "SetWeather" => {
                let hour = self
                    .order
                    .form_by_editor_id("GameHour")
                    .and_then(|g| self.state.globals.get(&g).copied())
                    .unwrap_or(0.0);
                let flag = args.get(1).is_some_and(|v| v.number() != 0.0);
                let w = arg(0).form();
                if name == "ForceWeather" {
                    self.state.weather.force(w, flag, hour);
                } else {
                    self.state.weather.set(w, flag, hour);
                }
                events.push(Event::Weather(Some(w)));
            }
            "ReleaseWeatherOverride" => {
                // Nothing to tell when nothing was forced (a script calling
                // it every frame would otherwise report it every frame).
                if self.state.weather.forced.take().is_some() {
                    events.push(Event::Weather(None));
                }
            }
            "PlayBink" => {
                let Value::Text(file) = arg(0) else {
                    return None;
                };
                let flag =
                    |i: usize, default: bool| args.get(i).map_or(default, |v| v.number() != 0.0);
                events.push(Event::Video(Video {
                    file,
                    interruptable: flag(1, false),
                    mute_audio: flag(2, true),
                    pause_music: flag(3, true),
                    letterbox: flag(4, true),
                }));
            }
            "PlayMusic" => events.push(Event::Music(arg(0).form())),
            // `005d0760`: knocked out (life state 3, `008ace10`) after
            // stopping their own fight (actor vtable +0x434 with no target,
            // the call `StopCombatAlarmOnActor` makes per attacker); woken
            // only from that state.
            "SetUnconscious" => {
                let who = target?;
                if arg(0).number() != 0.0 {
                    self.state.combat.remove(&who);
                    self.state.unconscious.insert(who);
                } else {
                    self.state.unconscious.remove(&who);
                }
            }
            "SetScale" => {
                self.state.scales.insert(target?, arg(0).number() as f32);
            }
            "ShowMap" => {
                // `005c8620` sets the visible bit (`0044de40`); a nonzero
                // second argument also sets the fast-travel bit (`0044de80`).
                let marker = arg(0).form();
                self.state.map_markers.insert(marker);
                if arg(1).number() != 0.0 {
                    self.state.map_marker_travel.insert(marker);
                }
            }
            // Always the player's, whoever it's called on (`005d5140`).
            "RewardXP" => {
                crate::experience::reward(self.order, self.state, arg(0).number());
            }
            "AddReputation"
            | "RemoveReputation"
            | "AddReputationExact"
            | "RemoveReputationExact"
            | "SetReputation" => {
                let (rep, kind) = (arg(0).form(), arg(1).number() as i64);
                if !(0..=1).contains(&kind) {
                    return None;
                }
                let kind = kind as u8;
                let n = arg(2).number();
                let (order, state) = (self.order, &mut *self.state);
                match name {
                    "AddReputation" => {
                        let by = crate::reputation::bump(order, n as i32)?;
                        crate::reputation::add(order, state, rep, kind, by);
                    }
                    "RemoveReputation" => {
                        let by = crate::reputation::bump(order, n as i32)?;
                        crate::reputation::remove(order, state, rep, kind, by);
                    }
                    "AddReputationExact" => {
                        crate::reputation::add(order, state, rep, kind, n as f32);
                    }
                    "RemoveReputationExact" => {
                        crate::reputation::remove(order, state, rep, kind, n as f32);
                    }
                    _ => crate::reputation::set(state, rep, kind, n as f32),
                }
            }
            // A faction holds the player as an enemy for their crimes, or
            // stops (`world::crime`).
            "SetPCEnemyofFaction" => {
                self.state.crime_enemies.insert(arg(0).form());
            }
            "ClearFactionPlayerEnemyFlag" => {
                self.state.crime_enemies.remove(&arg(0).form());
            }
            // Always the player's, whoever it's called on (`005d51a0`).
            "RewardKarma" => {
                crate::reputation::reward_karma(self.order, self.state, arg(0).number() as i32);
            }
            "ModPCMiscStat" => {
                let Value::Text(stat) = arg(0) else {
                    return None;
                };
                let stat = crate::stats::index(&stat)?;
                crate::stats::bump(self.state, stat, arg(1).number() as i64);
            }
            // `005d9430`: on a person or creature that's the player's
            // teammate, or anyone when the number given isn't 0, the
            // container menu on their things in mode 3 (`00709470`).
            "OpenTeammateContainer" => {
                let who = target?;
                let anyone = args.first().is_some_and(|a| a.number() != 0.0);
                if crate::more_functions::is_actor(self.order, self.state, who)
                    && (anyone || self.state.teammates.contains(&who))
                {
                    self.state.events.push(Event::TeammateContainer(who));
                }
            }
            "SetPlayerTeammate" => {
                let who = target?;
                if arg(0).number() != 0.0 {
                    self.state.teammates.insert(who);
                } else {
                    self.state.teammates.remove(&who);
                }
            }
            // A level, its menu, as the console's `AdvancePCLevel`.
            "AdvancePCLevel" => {
                self.state.level_up_pending = true;
                let max = crate::experience::max_level(self.order);
                if self.state.player_level < max {
                    // The menu opens at once: the waiting flag is what the
                    // viewer watches.
                    let need =
                        crate::experience::xp_for_level(self.order, self.state.player_level + 1);
                    let xp = crate::experience::xp(self.state);
                    if xp < need {
                        self.state
                            .actor_values
                            .insert((PLAYER_REF, crate::experience::XP), need);
                    }
                }
            }
            // Whom a person looks at (`Look`) is forgotten; there's no
            // blood on the screen here, so there's nothing to clear.
            "StopLook" => {
                if let Some(who) = target {
                    self.state.set_by_scripts.looking.remove(&who);
                }
            }
            "ClearScreenSplatter" => {}
            "PlaySound" | "PlaySound3D" => events.push(Event::Sound(arg(0).form())),
            "Activate" => {
                let what = target?;
                let by = args.first().map(Value::form);
                // `005b59f0`: with its second argument 1 the reference's
                // own `OnActivate` block runs (fewer than 5 deep), and does
                // the usual thing only by calling `Activate` itself
                // (`VCG01`'s `VCG01CasualHardcoreMessageREF.Activate player
                // 1` asks about hardcore mode this way); without it the
                // usual thing happens at once.
                let run_block = args.get(1).is_some_and(|v| v.number() != 0.0);
                let has_block = script_of(self.order, what)
                    .and_then(|s| self.scripts.script(self.order, s))
                    .is_some_and(|s| s.blocks.iter().any(|b| b.kind == "onactivate"));
                if run_block && has_block {
                    let depth = ACTIVATE_DEPTH.with(|d| d.get());
                    if depth < 5 {
                        ACTIVATE_DEPTH.with(|d| d.set(depth + 1));
                        let who = by.filter(|f| f.0 != 0).unwrap_or(what);
                        self.run_event(what, "onactivate", who);
                        ACTIVATE_DEPTH.with(|d| d.set(depth));
                    }
                    return Some(0.0);
                }
                let events = &mut self.state.events;
                events.push(Event::Activate { what, by });
                // Someone else using furniture sits in it.
                if let Some(who) = by.filter(|&w| w != PLAYER_REF && w.0 != 0) {
                    if GameState::is_furniture(self.order, what) {
                        self.state.sit(who, what);
                    }
                }
            }
            "ShowRaceMenu" => events.push(Event::Menu(RACE_SEX_MENU)),
            // `005dc4e0`: only while the terminal menu is open (1057,
            // `00a09030`); its screen stack is popped (`00758a80` →
            // `0063f7b0`) and the screen before shown, or with none left
            // the terminal closes (`00757ea0`). The viewer keeps the stack.
            "ForceTerminalBack" => {
                if self.state.more.menu_open == Some(crate::terminal::TERMINAL_MENU) {
                    self.state.events.push(Event::TerminalBack);
                }
            }
            // `005b53d0`: one of the scripted item running goes from its
            // holder (outside an item's own run it does nothing).
            // `005b5860`: the holder drops this one into the world
            // (`RemoveItem` with its drop flag); its script goes with it.
            "DropMe" => {
                if let Some((holder, item)) = self.item.filter(|_| !self.removed) {
                    if let Some(made) = crate::more_functions::placed::drop_into_world(
                        self.order, self.state, holder, item, 1,
                    ) {
                        self.dropped = Some(made);
                        self.removed = true;
                    }
                }
            }
            // `005cf3d0`: the card (the item whose script runs, or the
            // reference's base) joins the player's cards
            // (`PlayerCharacter::AddCaravanCard`).
            "AddCardToPlayer" => {
                let card = match self.item {
                    Some((_, item)) => Some(item),
                    None => target.and_then(|r| base_of(self.order, r)),
                };
                if let Some(card) = card {
                    crate::caravan::add_card_to_player(self.order, self.state, card);
                }
            }
            // `005ce5c0`: the one holding the item whose script this is
            // (the containing object, `0084e3a0`); 0 for a script that
            // isn't a held item's.
            "GetContainer" => {
                return Some(self.item.map_or(0.0, |(holder, _)| f64::from(holder.0)));
            }
            // `005d3e30`: the reference's base's value (`0048e960`), for
            // every one of them. A held item's own script has no reference
            // to give it (here `this` is its holder): nothing.
            "SetItemValue" => {
                let r = target?;
                let own = self.item.is_some_and(|(holder, _)| holder == r);
                let base = base_of(self.order, r);
                if let Some(base) =
                    base.filter(|b| !own && crate::barter::has_value(self.order, *b))
                {
                    self.state
                        .more
                        .item_values
                        .insert(base, arg(0).number() as i32);
                }
            }
            // `005b58d0`: `ref.Drop item count`, into the world.
            "Drop" => {
                let holder = target?;
                let item = arg(0).form();
                let count = args.get(1).map_or(1, |a| a.number() as i32);
                self.state.drop_item(self.order, holder, item, count);
            }
            // `005b53d0`: the holder's remove-item (vtable +0x17c) for one,
            // into the container given if any (its fifth argument). On an
            // actor wearing the item (`00575400`: `004bfda0(item, 0)` finds
            // a worn instance, extra data 0x16 or 0x17) the worn instance
            // is the one taken, which takes it off first (`004c37d0` on a
            // worn list: `0088d7d0` for the player, the actor's +0x188
            // otherwise); else one of the others goes.
            "RemoveMe" => {
                if let Some((holder, item)) = self.item.filter(|_| !self.removed) {
                    if self.state.is_equipped(holder, item) {
                        self.state.unequip_item(self.order, holder, item);
                    }
                    if let Some(n) = self.state.items.get_mut(&(holder, item)) {
                        *n -= 1;
                        if *n <= 0 {
                            self.state.items.remove(&(holder, item));
                        }
                    }
                    self.removed = true;
                    if let Some(to) = args.first().map(Value::form).filter(|f| f.0 != 0) {
                        self.state.stock(self.order, to);
                        *self.state.items.entry((to, item)).or_insert(0) += 1;
                        // Its script goes with it (`OnDrop`, then `OnAdd`
                        // for the container: see `run_item_scripts`).
                        self.dropped = Some(to);
                    }
                }
            }
            "SetActorValue" | "ForceActorValue" | "ModActorValue" | "DamageActorValue"
            | "RestoreActorValue" => {
                let who = target?;
                let av = arg(0).number() as u16;
                let amount = arg(1).number();
                match name {
                    // Damage is taken (health can kill; rads go up),
                    // restoring takes it back off (`world::magic::change`).
                    "DamageActorValue" => {
                        if crate::magic::change(self.order, self.state, who, av, -amount, who) {
                            self.run_event(who, "ondeath", who);
                        }
                    }
                    "RestoreActorValue" => {
                        crate::magic::change(self.order, self.state, who, av, amount, who);
                    }
                    // Rads and the hardcore needs are their damage.
                    _ if crate::magic::COUNTERS.contains(&av) => {
                        let now = self.facts().current_actor_value(who, av)?;
                        let to = if name == "ModActorValue" {
                            now + amount
                        } else {
                            amount
                        };
                        crate::magic::change(self.order, self.state, who, av, now - to, who);
                    }
                    "ModActorValue" => {
                        let base = if av == crate::combat::av::HEALTH {
                            self.facts().base_actor_value(who, av)?
                        } else {
                            self.facts().unaffected_actor_value(who, av)?
                        };
                        self.state.actor_values.insert((who, av), base + amount);
                    }
                    _ => {
                        self.state.actor_values.insert((who, av), amount);
                    }
                }
            }
            // Spells: cast on the caller (`CastImmediateOnSelf`), cast at
            // someone (`Cast spell target`), given (`AddSpell`), taken off.
            "CastImmediateOnSelf" => {
                let who = target?;
                self.cast(arg(0).form(), who, who);
            }
            "Cast" => {
                let caster = target?;
                let at = args.get(1).map_or(caster, Value::form);
                self.cast(arg(0).form(), caster, at);
            }
            "AddSpell" => {
                let who = target?;
                crate::magic::add_spell(self.order, self.state, who, arg(0).form(), who, true);
            }
            "RemoveSpell" | "Dispel" => {
                let who = target?;
                for mut e in crate::magic::remove(self.state, who, arg(0).form()) {
                    if let Some(script) = e.script.filter(|_| e.started) {
                        self.run_effect_script(script, &mut e, "scripteffectfinish", 0.0);
                    }
                }
            }
            "SetWeaponHealthPerc" | "ModWeaponHealthPerc" => {
                let who = target?;
                let weapon = crate::combat::weapon_in_hand(self.order, self.state, who)?.form_id;
                let now = crate::combat::weapon_condition(self.state, who, weapon);
                let percent = arg(0).number() as f32;
                let to = if name == "SetWeaponHealthPerc" {
                    percent / 100.0
                } else {
                    now + percent / 100.0
                };
                self.state
                    .weapon_health
                    .insert((who, weapon), to.clamp(0.0, 1.0));
            }
            "ForceActiveQuest" => self.state.active_quest = Some(arg(0).form()),
            // `Lock [level]` (else the level it was placed with, else 0);
            // `Unlock`.
            "Lock" => {
                let r = target?;
                if crate::terminal::placed(self.order, r).is_some() {
                    let level = args.first().map_or(0, |a| a.number() as i32);
                    crate::terminal::script_lock(self.state, r, level);
                    return Some(0.0);
                }
                let level = args
                    .first()
                    .map(|a| a.number() as u8)
                    .or_else(|| crate::locks::placed_lock(self.order, r).map(|l| l.level));
                self.state.locks.insert(r, Some(level.unwrap_or(0)));
                // `005cbf80`: an open door is shut at once (`0047aec0`),
                // with no sequence or sound.
                if crate::doors::is_swing_door(self.order, r)
                    && crate::doors::open_state(self.order, self.state, r).is_open()
                {
                    crate::doors::set_at_once(self.state, r, false);
                }
            }
            // The table's name (scripts write `Unlock`).
            "UnLock" => {
                let r = target?;
                if crate::terminal::placed(self.order, r).is_some() {
                    crate::terminal::script_unlock(self.state, r);
                } else {
                    self.state.locks.insert(r, None);
                }
            }
            // FalloutNV.exe 1.4.0.525, `AddScriptPackage` at `exe005cc4f0`
            // requests the old package's change action through
            // HighProcess+0x59c (`00903bf0`), then the new package's begin
            // action through +0x598 (`00903a80`), before installing it.
            // There is no identity comparison: assigning the same package
            // requests both actions too. Keep these as ordered requests;
            // `world::ai::actions::perform` carries them out from the PACK
            // record. The package is installed at once (actor vfunc +0x2f4,
            // `PutCreatedPackage` (Xbox PDB)), so the AI taking it up
            // doesn't begin it again (`package_begun`).
            "AddScriptPackage" => {
                let who = target?;
                // `exe005cc4f0` resolves the target as an actor; preserve
                // that cast boundary while accepting the hard-coded player.
                if !crate::script_functions::is_actor(self.order, who) {
                    return Some(0.0);
                }
                let package = arg(0).form();
                let Some(new_package) = crate::ai::Package::load(self.order, package) else {
                    // A null, missing, malformed or non-PACK form is not an
                    // assignable script package.
                    return Some(0.0);
                };
                let previous = self.state.script_packages.get(&who).copied();
                if let Some(old_package) =
                    previous.filter(|id| crate::ai::Package::load(self.order, *id).is_some())
                {
                    self.state.events.push(Event::PackageAction {
                        who,
                        package: old_package,
                        kind: PackageActionKind::Change,
                    });
                }
                self.state.events.push(Event::PackageAction {
                    who,
                    package,
                    kind: PackageActionKind::Begin,
                });
                self.state
                    .package_begun
                    .insert(who, (new_package.form_id, false));
                self.state.script_packages.insert(who, new_package.form_id);
                self.state.evaluate.insert(who);
            }
            "RemoveScriptPackage" => {
                let who = target?;
                self.state.script_packages.remove(&who);
                self.state.evaluate.insert(who);
            }
            // The package is looked at again at once (the viewer's AI).
            // It also ends `ForceFlee`'s flee: the package picked from their
            // list replaces it (`Actor::EvaluatePackage` (Xbox PDB)).
            "EvaluatePackage" => {
                let who = target?;
                self.state.evaluate.insert(who);
                crate::ai::flee::end_forced(self.state, who);
            }
            // The command queues a full reset, not just package evaluation.
            // 005c9530 rejects deleted/disabled references; 008a6ce0 rejects
            // dying/dead actors before setting deferred flags +0x145/+0x146.
            "ResetAI" => {
                let who = target.filter(|&w| crate::script_functions::is_actor(self.order, w))?;
                if self.state.dead.contains(&who)
                    || !crate::placement::enabled_now(self.order, who, &self.state.disabled)
                    || self
                        .order
                        .get(who)
                        .is_some_and(|r| r.entry.header.is_deleted())
                {
                    return Some(0.0);
                }
                self.state.reset_ai.insert(who);
                crate::ai::flee::end_forced(self.state, who);
            }
            "GetSecondsPassed" => return Some(f64::from(self.seconds_passed)),
            "GetRandomPercent" => return Some((self.state.roll() % 100) as f64),
            other => {
                if let Some(done) = crate::living::change(self, other, target, args) {
                    return done;
                }
                if let Some(done) = crate::more_functions::change(self, other, target, args) {
                    return done;
                }
                return crate::script_functions::change(self, other, target, args).flatten();
            }
        }
        Some(0.0)
    }
}

impl Host for Runner<'_> {
    fn call(&mut self, call: &Call, on: Option<u32>, locals: &mut Locals) -> Option<f64> {
        let sig = script::functions::FUNCTIONS.get(usize::from(call.function))?;
        if CHANGES_REFERENCES.contains(&sig.name) {
            self.references_changed = true;
        }
        let target = on.map(FormId).or(self.this);
        // `ShowMessage`'s values after the message aren't in its table
        // entry (the compiler stores them its own way): numbers.
        let kind = |i: usize| {
            sig.params
                .get(i)
                .map(|p| p.kind)
                .or((sig.name == "ShowMessage").then_some(2))
        };
        let args: Vec<Value> = call
            .args
            .iter()
            .enumerate()
            .map(|(i, a)| self.argument(a, kind(i), locals))
            .collect();
        // The commands that run other scripts (a quest stage's result script
        // runs with the quest's own variables) may touch this script's
        // variables, which are out of the state while it runs. The game
        // keeps one copy, so the nested script sees this one's current
        // values and its changes stay: hand them over for the command, take
        // them back after. (`VCG01SCRIPT`'s `setstage VCG01 1` runs stage
        // 1's `set VCG01.fTimer to 2.8`; keeping the run's old copy lost the
        // 2.8 s, and Doc spoke after 0.2 s instead of 3 s.)
        let shared = matches!(sig.name, "SetStage" | "StartQuest" | "StopQuest")
            .then_some(self.owner)
            .flatten();
        if let Some(owner) = shared {
            self.state.variables.insert(owner, locals.clone());
        }
        let value = self
            .change(sig.name, target, &args)
            .or_else(|| self.facts().value(call.function, target, &args));
        if let Some(owner) = shared {
            if let Some(current) = self.state.variables.remove(&owner) {
                *locals = current;
            }
        }
        if value.is_none() {
            *self.state.unhandled.entry(sig.name).or_default() += 1;
            if !self.state.unhandled_first.contains_key(sig.name) {
                let name = |id: Option<FormId>| {
                    id.map_or("nothing".to_string(), |id| {
                        self.order
                            .get(id)
                            .and_then(|r| r.editor_id().ok().flatten())
                            .unwrap_or_else(|| id.to_string())
                    })
                };
                let first = format!(
                    "in {}'s script, on {}, with {:?}",
                    name(self.owner),
                    name(target),
                    args
                );
                self.state.unhandled_first.insert(sig.name, first);
            }
        }
        value
    }

    fn get_var(&mut self, owner: &str, name: &str, locals: &Locals) -> Option<f64> {
        let id = match locals.get(owner) {
            Some(v) => FormId(v as u32),
            None => self.form(owner)?,
        };
        if Some(id) == self.owner {
            return locals.get(name);
        }
        if let Some(l) = self.state.variables.get(&id) {
            return l.get(name);
        }
        // Not used yet: its variables start at 0.
        let l = self.take_locals(Some(id));
        let v = l.get(name);
        self.put_locals(Some(id), l);
        v
    }

    fn set_var(&mut self, owner: &str, name: &str, value: f64, locals: &mut Locals) -> bool {
        if owner.is_empty() {
            // A global variable.
            let Some(id) = self.form(name) else {
                return false;
            };
            if self.order.get(id).map(|r| r.entry.header.kind) != Some(GLOB) {
                return false;
            }
            self.state.globals.insert(id, value as f32);
            return true;
        }
        let id = match locals.get(owner) {
            Some(v) => FormId(v as u32),
            None => match self.form(owner) {
                Some(id) => id,
                None => return false,
            },
        };
        if Some(id) == self.owner {
            return locals.set(name, value);
        }
        let mut l = self.take_locals(Some(id));
        let ok = l.set(name, value);
        self.put_locals(Some(id), l);
        ok
    }

    fn resolve(&mut self, word: &str) -> Option<f64> {
        let id = self.form(word)?;
        if self.order.get(id).map(|r| r.entry.header.kind) == Some(GLOB) {
            return Some(f64::from(
                self.state.globals.get(&id).copied().unwrap_or(0.0),
            ));
        }
        Some(f64::from(id.0))
    }
}

/// Every function the runtime answers or carries out, by the game's own
/// name for it (scripts write short names too, like `MoveTo` for
/// `MoveToMarker`; matching on one of those would never fire). Calls to
/// others are reported as not carried out (`nvinspect <Data> functions`
/// counts them across the game's scripts and conditions).
pub const HANDLED: &[&str] = &[
    "Lock",
    "UnLock",
    "GetLocked",
    "GetLockLevel",
    "GetIsLockBroken",
    "GetLinkedRef",
    "CastImmediateOnSelf",
    "Cast",
    "AddSpell",
    "RemoveSpell",
    "Dispel",
    "IsSpellTarget",
    "GetWeaponHealthPerc",
    "SetWeaponHealthPerc",
    "ModWeaponHealthPerc",
    "GetDetected",
    "GetContainer",
    "SetItemValue",
    "AddCardToPlayer",
    "ShowCaravanMenu",
    "ShowTutorialMenu",
    "ShowSlotMachineMenuParams",
    "ShowBlackJackMenuParams",
    "ShowRouletteMenuParams",
    "SetCasinoWinningsLevel",
    "SetCasinoCheatLevel",
    "ForceActiveQuest",
    "SetEnemy",
    "SetAlly",
    "AddToFaction",
    "SetFactionRank",
    "RemoveFromFaction",
    "GetFactionRelation",
    "IsInCombat",
    "GetCombatTarget",
    "GetShouldAttack",
    "StartCombat",
    "StopCombat",
    "EquipItem",
    "UnequipItem",
    "KillActor",
    "ResurrectActor",
    "GetEquipped",
    "ShowBarterMenu",
    "ShowRepairMenu",
    "OpenTeammateContainer",
    "DropMe",
    "Drop",
    "SetPlayerTagSkill",
    "GetPlayerControlsDisabled",
    "GetPlayerName",
    "GetButtonPressed",
    "SexChange",
    "ShowNameMenu",
    "ShowLoveTesterMenuParams",
    "SetTagSkills",
    "ShowTraitMenu",
    "GetInSameCell",
    "GetPCMiscStat",
    "ApplyImageSpaceModifier",
    "ClearScreenSplatter",
    "ForceWeather",
    "GetIsCurrentPackage",
    "GetMapMarkerVisible",
    "GetUnconscious",
    "GetFatiguePercentage",
    "GetGroupMemberCount",
    "GetGroupTargetCount",
    "ForceFlee",
    "IsPlayerInRegion",
    "PlayBink",
    "PlayMusic",
    "ShowRecipeMenu",
    "ReleaseWeatherOverride",
    "RemoveImageSpaceModifier",
    "SetQuestDelay",
    "SetScale",
    "SetUnconscious",
    "ShowMap",
    "StopLook",
    "Abs",
    "Activate",
    "AddItem",
    "AddNote",
    "AddPerk",
    "AddScriptPackage",
    "AddTopic",
    "CompleteQuest",
    "DamageActorValue",
    "Disable",
    "DisablePlayerControls",
    "Enable",
    "EnablePlayerControls",
    "EvaluatePackage",
    "ForceActorValue",
    "GetActionRef",
    "GetActorValue",
    "GetBaseActorValue",
    "GetCasinoWinningsLevel",
    "GetCurrentTime",
    "GetDayOfWeek",
    "GetDead",
    "GetDeadCount",
    "GetDestroyed",
    "GetDisabled",
    "GetDistance",
    "GetGlobalValue",
    "GetHasNote",
    "GetHealthPercentage",
    "GetInCell",
    "GetInFaction",
    "GetInWorldspace",
    "GetIsID",
    "GetIsRace",
    "GetIsReference",
    "GetIsSex",
    "GetIsVoiceType",
    "GetItemCount",
    "GetKnockedState",
    "GetLevel",
    "GetObjectiveCompleted",
    "GetObjectiveDisplayed",
    "GetPlayerTeammate",
    "GetPlayerTeammateCount",
    "SetPlayerTeammate",
    "AdvancePCLevel",
    "ModPCMiscStat",
    "AddReputation",
    "RemoveReputation",
    "AddReputationExact",
    "RemoveReputationExact",
    "SetReputation",
    "GetReputationPct",
    "RewardKarma",
    "GetPCEnemyofFaction",
    "IsPCAMurderer",
    "SetPCEnemyofFaction",
    "ClearFactionPlayerEnemyFlag",
    "IsTrespassing",
    "GetMinorCrimeCount",
    "GetMajorCrimeCount",
    "SetWeather",
    "GetIsCurrentWeather",
    "GetCurrentWeatherPercent",
    "IsRaining",
    "IsSnowing",
    "GetPCIsRace",
    "GetPCIsSex",
    "GetPermanentActorValue",
    "GetQuestCompleted",
    "GetQuestRunning",
    "GetQuestVariable",
    "GetRandomPercent",
    "GetReputation",
    "GetReputationThreshold",
    "GetScriptVariable",
    "GetSecondsPassed",
    "GetSelf",
    "GetSitting",
    "GetSleeping",
    "GetFurnitureMarkerID",
    "GetStage",
    "GetStageDone",
    "GetTalkedToPC",
    "HasBeenEaten",
    "HasMagicEffect",
    "HasPerk",
    "IsActionRef",
    "IsCurrentFurnitureObj",
    "IsCurrentFurnitureRef",
    "IsDLCInstalled",
    "IsInCombat",
    "IsWeaponOut",
    "MarkForDelete",
    "ModActorValue",
    "MoveToMarker",
    "MoveToMarkerWithFade",
    "PlaySound",
    "PlaySound3D",
    "RemoveAllItems",
    "RemoveItem",
    "RemovePerk",
    "RemoveScriptPackage",
    "ResetAI",
    "RestoreActorValue",
    "RewardXP",
    "SayTo",
    "SetActorValue",
    "SetDestroyed",
    "SetObjectiveCompleted",
    "SetObjectiveDisplayed",
    "SetStage",
    "ShowMessage",
    "ShowRaceMenu",
    "ForceTerminalBack",
    "RemoveMe",
    "StartConversation",
    "StartQuest",
    "StopQuest",
    "GetHitLocation",
    "GetKillingBlowLimb",
    // V.A.T.S. (`world::vats::function_value`).
    "GetVATSMode",
    "GetVATSValue",
    "GetVATSRightAreaFree",
    "GetVATSLeftAreaFree",
    "GetVATSBackAreaFree",
    "GetVATSFrontAreaFree",
    "GetVATSRightTargetVisible",
    "GetVATSLeftTargetVisible",
    "GetVATSBackTargetVisible",
    "GetVATSFrontTargetVisible",
];

#[cfg(test)]
mod tests {
    use super::HANDLED;

    #[test]
    fn handled_functions_go_by_the_games_own_names() {
        for name in HANDLED {
            assert!(
                script::functions::FUNCTIONS.iter().any(|f| f.name == *name),
                "{name} isn't a function's name in the game's table"
            );
        }
        // The short name a script writes finds the same function.
        let (i, _) = script::function("moveto").unwrap();
        assert_eq!(script::function_name(i), "MoveToMarker");
    }
}

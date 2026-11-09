//! xNVSE's event manager (ADR-0016 §8; build step I-9), translated from `EventManager.cpp`,
//! `EventManager.h` and `EventParams.h` (xNVSE): the events scripts set handlers for, their
//! filters and priorities, and where the events come from.
//!
//! - **The events:** the 60 xNVSE registers at its start, in its order (their index is
//!   `eEventID`): 25 game events of the script event masks, `OnActivate` and `OnDropItem`, 15
//!   save, load and exit messages, the unit tests' `nvsetestevent`, 17 newer ones. A name is
//!   found by xNVSE's case-insensitive string hash (`StrHashCI`), which is all its map keeps;
//!   the second `postloadgame` is refused, as `RegisterEventEx` refuses it. A name first used
//!   by `SetEventHandler` becomes a user-defined event (lower-cased).
//! - **Handlers:** per event, ordered by priority (highest first; -9999 to 9999, 0 reserved,
//!   default 1) and, within one priority, in the order they were set (`std::multimap`). A
//!   handler is a function script with the old filters (`"first"`, `"second"`: forms) or the
//!   new numbered ones (`N::value`, 0 for the calling reference; arrays of values match any).
//!   Removing marks a handler; it is erased at the next tick, so a dispatch under way never
//!   loses its place. Handlers set by `SetEventHandlerAlt` go when a game is loaded.
//! - **Dispatch:** the deprecated path (`HandleEvent`: game events and messages, at most two
//!   arguments, the handler's parameters checked against the event's) and the newer one
//!   (`DispatchEventRaw`: `DispatchEventAlt`, the newer events). Outside the main thread a
//!   dispatch is deferred to the tick, where the deferred ones run newest first (xNVSE's
//!   `Stack` and `std::deque` empty from the back). The main thread is the one recorded at the
//!   first frame; before it, every dispatch is deferred, as in xNVSE.
//! - **Where events come from:** the game marks script events through `005ac750`
//!   (`Script::SetActionFlag`, wrapped: xNVSE's `MarkEventHook`), once a game event's handler
//!   has been set (its "hook installed"); xNVSE's messages at the save, load, new game,
//!   delete, rename and exit seams ([`crate::saveload`], [`install`]) and from run-time errors
//!   ([`crate::errors::show`]).
//!
//! - **Threads:** the extender's state is the game's: real threads' engines share it (engine-core's
//!   shared slot, one lock), so an event the game marks on an AI thread finds the handlers and is
//!   kept for the main thread's next tick as in xNVSE. A kept call copies its string arguments'
//!   text (the save name or message it pointed at is gone by then).
//! - **The fixes preset's row** [`set_error_guard`]: an error while a `RuntimeScriptError`
//!   handler runs is not sent to the handlers again (xNVSE recurses until the stack runs out).
//! - `NV_NVSE_EVENT_LOG=FILE` (a test variable) writes a line per handler call.
//!
//! The order of a dispatch's handlers is kept by priority and a serial, so a handler set or a
//! game loaded inside a handler cannot invalidate the walk (xNVSE's iterators would be left
//! dangling by a load's flush there).

use crate::expr::Eval;
use crate::functions::{self, Caller};
use crate::vars::{self, Data, Key, Value};
use crate::{lambda, Nvse};
use engine_core::extend::{Owner, SiteSlot, WrapSlot};
use engine_core::prelude::*;
use std::collections::{BTreeMap, HashMap};

/// `Script::SetActionFlag` (`005ac750`): the game marks a script event on a reference.
pub const SET_ACTION_FLAG: WrapSlot = WrapSlot::new(Owner::Nvse, 0x005a_c750);
/// The console's `qqq` (`QuitGame`, in `005b6c90`), the main menu chosen in game (in
/// `ChooseMainMenu` `007d0a70`) and the game left from the menu (in `007d0bf0`).
pub const QUIT_CONSOLE: SiteSlot = SiteSlot::new(Owner::Nvse, 0x005b_6ca6);
pub const QUIT_TO_MAIN_MENU: SiteSlot = SiteSlot::new(Owner::Nvse, 0x007d_0b17);
pub const QUIT_FROM_MENU: SiteSlot = SiteSlot::new(Owner::Nvse, 0x007d_0c3e);

/// `LookupFormByID` (`0x004839C0`).
const LOOKUP_FORM: u32 = 0x0048_39c0;
/// `game_tolower` (`0x00EC67AA`), which `StrToLower` lower-cases a new event's name with.
pub const GAME_TOLOWER: u32 = 0x00ec_67aa;
/// `BaseExtraList::GetByType` (`0x00410220`) and `kExtraData_LeveledCreature`.
const EXTRA_GET_BY_TYPE: u32 = 0x0041_0220;
const EXTRA_LEVELED_CREATURE: u32 = 0x2e;
/// `RTTI_BGSPlaceableWater`.
const RTTI_PLACEABLE_WATER: u32 = 0x0118_9d70;
/// `TESForm::GetIsReference`, virtual slot `0x3C`.
const IS_REFERENCE: u32 = 0xf0;
/// `BGSListForm`'s type (its list at `+0x18`).
const LIST_FORM: u8 = 0x55;

/// The vtables `IsValidReference` accepts (`EventManager.cpp:54-72`).
const REFERENCE_VTABLES: [u32; 19] = [
    0x0108_aa3c,
    0x0108_6a6c,
    0x0108_70ac,
    0x0102_f55c,
    0x0108_a49c,
    0x0108_4254,
    0x0108_ee04,
    0x0109_00dc,
    0x0108_c3c4,
    0x0108_ea64,
    0x0108_f2f4,
    0x0108_f674,
    0x0108_fa44,
    0x0108_5954,
    0x0107_b394,
    0x0107_a554,
    0x0107_a8f4,
    0x0107_ad84,
    0x0107_b8c4,
];

/// `NVSEEventManagerInterface::ParamType`.
pub mod param {
    pub const FLOAT: u8 = 0;
    pub const INT: u8 = 1;
    pub const STRING: u8 = 2;
    pub const ARRAY: u8 = 3;
    /// `eParamType_RefVar` / `eParamType_AnyForm`.
    pub const ANY_FORM: u8 = 4;
    pub const REFERENCE: u8 = 5;
    pub const BASE_FORM: u8 = 6;
    pub const INVALID: u8 = 7;
    pub const ANYTHING: u8 = 8;
    pub const FLOAT_PTR: u8 = 9;
    pub const BASE_FORM_PTR: u8 = 15;
}
use param::*;

/// `EventFlags`.
pub const FLAG_FLUSH_ON_LOAD: u32 = 1;
pub const FLAG_UNKNOWN_ARG_TYPES: u32 = 2;
pub const FLAG_ALLOW_SCRIPT_DISPATCH: u32 = 4;
pub const FLAG_USER_DEFINED: u32 = FLAG_UNKNOWN_ARG_TYPES | FLAG_ALLOW_SCRIPT_DISPATCH;
pub const FLAG_REPORT_ERROR_IF_NO_RESULT: u32 = 8;

/// `SpecialHandlerPriorities`.
pub const PRIORITY_INVALID: i32 = 0;
pub const PRIORITY_DEFAULT: i32 = 1;
pub const PRIORITY_MAX: i32 = 9999;
pub const PRIORITY_MIN: i32 = -9999;

/// `kNumMaxFilters` (`kMaxUdfParams`).
pub const MAX_FILTERS: u32 = 15;

/// `DispatchReturn`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Dispatch {
    UnknownEvent = -2,
    GenericError = -1,
    Normal = 0,
    EarlyBreak = 1,
    Deferred = 2,
}

/// The internal events' ids (`eEventID`): their index in the registry.
pub mod id {
    pub const ON_HIT_WITH: usize = 8;
    pub const ON_HIT: usize = 7;
    pub const ON_SELL: usize = 14;
    pub const ON_ACTIVATE: usize = 25;
    pub const ON_DROP_ITEM: usize = 26;
    pub const EXIT_GAME: usize = 27;
    pub const EXIT_TO_MAIN_MENU: usize = 28;
    pub const LOAD_GAME: usize = 29;
    pub const SAVE_GAME: usize = 30;
    pub const QQQ: usize = 31;
    pub const POST_LOAD_GAME: usize = 32;
    pub const RUNTIME_SCRIPT_ERROR: usize = 33;
    pub const DELETE_GAME: usize = 34;
    pub const RENAME_GAME: usize = 35;
    pub const RENAME_NEW_GAME: usize = 36;
    pub const NEW_GAME: usize = 37;
    pub const DELETE_GAME_NAME: usize = 38;
    pub const RENAME_GAME_NAME: usize = 39;
    pub const RENAME_NEW_GAME_NAME: usize = 40;
    pub const PRE_LOAD_GAME: usize = 41;
    pub const DEBUG: usize = 42;
    pub const ON_ACTOR_EQUIP: usize = 1;
    pub const ON_DROP: usize = 2;
    // The events xNVSE raises at its own call sites (`crate::call_sites`).
    pub const ON_APPLY_IMOD: usize = 43;
    pub const ON_REMOVE_IMOD: usize = 44;
    pub const ON_LOCK_BROKEN: usize = 45;
    pub const ON_LOCK_PICK_SUCCESS: usize = 46;
    pub const ON_LOCK_PICK_BROKEN: usize = 47;
    pub const ON_TERMINAL_HACKED: usize = 49;
    pub const ON_TERMINAL_HACK_FAILED: usize = 50;
    pub const ON_REPAIR: usize = 51;
    pub const ON_DISABLE: usize = 52;
    pub const ON_ENABLE: usize = 53;
    pub const ON_REF_SET_3D: usize = 54;
    pub const ON_REF_UNSET_3D: usize = 55;
    pub const ON_REF_ATTACH: usize = 56;
    pub const ON_CELL_ATTACH: usize = 57;
    pub const ON_CELL_DETACH: usize = 58;
    pub const ON_CELL_REFS_LOADED: usize = 59;
}

/// `kEventMask_OnActivate`: not a script event mask; only its own hook raises it.
pub const MASK_ON_ACTIVATE: u32 = 0x0100_0000;

/// Which of xNVSE's hooks setting a handler for an event installs (`EventHookInstaller`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Hook {
    None,
    /// `s_MainEventHook`: the mark at `005ac750`.
    Main,
    /// `s_ActorEquipHook`: the main hook and the equip call at `004C0032`.
    ActorEquip,
    /// `s_ActivateHook`: `0057318E`.
    Activate,
}

/// The installed hooks (bits).
const HOOK_MAIN: u8 = 1;
const HOOK_EQUIP: u8 = 2;
const HOOK_ACTIVATE: u8 = 4;

struct Def {
    name: &'static str,
    alias: Option<&'static str>,
    params: &'static [u8],
    mask: u32,
    hook: Hook,
    flags: u32,
}

const GAME: &[u8] = &[ANY_FORM, ANY_FORM];
const ONE_FORM: &[u8] = &[ANY_FORM];
const ONE_STRING: &[u8] = &[STRING];
const ONE_INT: &[u8] = &[INT];
const TWO_REFS_ONE_INT: &[u8] = &[ANY_FORM, ANY_FORM, INT];
const TEST: &[u8] = &[INT, FLOAT, ARRAY, STRING, ANY_FORM, REFERENCE, BASE_FORM];

const fn game(name: &'static str, mask: u32, hook: Hook) -> Def {
    Def {
        name,
        alias: None,
        params: GAME,
        mask,
        hook,
        flags: 0,
    }
}
const fn aliased(name: &'static str, alias: &'static str, mask: u32, hook: Hook) -> Def {
    Def {
        name,
        alias: Some(alias),
        params: GAME,
        mask,
        hook,
        flags: 0,
    }
}
const fn plain(name: &'static str, params: &'static [u8]) -> Def {
    Def {
        name,
        alias: None,
        params,
        mask: 0,
        hook: Hook::None,
        flags: 0,
    }
}

/// xNVSE's 61 registrations, in its order (`EventManager::Init`, `EventManager.cpp:2115-2188`).
static DEFS: [Def; 61] = [
    game("onadd", 0x1, Hook::Main),
    aliased("onactorequip", "onequip", 0x2, Hook::ActorEquip),
    game("ondrop", 0x4, Hook::Main),
    aliased("onactorunequip", "onunequip", 0x8, Hook::Main),
    game("ondeath", 0x10, Hook::Main),
    game("onmurder", 0x20, Hook::Main),
    game("oncombatend", 0x40, Hook::Main),
    game("onhit", 0x80, Hook::Main),
    game("onhitwith", 0x100, Hook::Main),
    game("onpackagechange", 0x800, Hook::Main),
    game("onpackagestart", 0x200, Hook::Main),
    game("onpackagedone", 0x400, Hook::Main),
    game("onload", 0x1000, Hook::Main),
    game("onmagiceffecthit", 0x2000, Hook::Main),
    game("onsell", 0x4000, Hook::None),
    game("onstartcombat", 0x8000, Hook::Main),
    game("saytodone", 0x4_0000, Hook::Main),
    // xNVSE's alias has one zero fewer than the mask.
    aliased("ongrab", "on0x0080000", 0x8_0000, Hook::Main),
    game("onopen", 0x1_0000, Hook::Main),
    game("onclose", 0x2_0000, Hook::Main),
    aliased("onfire", "on0x00400000", 0x40_0000, Hook::Main),
    game("ontrigger", 0x1000_0000, Hook::Main),
    game("ontriggerenter", 0x2000_0000, Hook::Main),
    game("ontriggerleave", 0x4000_0000, Hook::Main),
    game("onreset", 0x8000_0000, Hook::Main),
    game("onactivate", MASK_ON_ACTIVATE, Hook::Activate),
    game("ondropitem", 0, Hook::Main),
    plain("exitgame", &[]),
    plain("exittomainmenu", &[]),
    plain("loadgame", ONE_STRING),
    plain("savegame", ONE_STRING),
    plain("qqq", &[]),
    plain("postloadgame", ONE_INT),
    plain("runtimescripterror", ONE_STRING),
    plain("deletegame", ONE_STRING),
    plain("renamegame", ONE_STRING),
    plain("renamenewgame", ONE_STRING),
    plain("newgame", &[]),
    plain("deletegamename", ONE_STRING),
    plain("renamegamename", ONE_STRING),
    plain("renamenewgamename", ONE_STRING),
    // Registered a second time; refused.
    plain("postloadgame", ONE_INT),
    plain("preloadgame", ONE_STRING),
    Def {
        name: "nvsetestevent",
        alias: None,
        params: TEST,
        mask: 0,
        hook: Hook::None,
        flags: FLAG_ALLOW_SCRIPT_DISPATCH,
    },
    plain("onapplyimod", ONE_FORM),
    plain("onremoveimod", ONE_FORM),
    plain("onlockbroken", ONE_FORM),
    plain("onlockpicksuccess", ONE_FORM),
    plain("onlockpickbroken", ONE_FORM),
    plain("onunlock", TWO_REFS_ONE_INT),
    plain("onterminalhacked", ONE_FORM),
    plain("onterminalhackfailed", ONE_FORM),
    plain("onrepair", TWO_REFS_ONE_INT),
    plain("ondisable", ONE_FORM),
    plain("onenable", ONE_FORM),
    plain("onrefset3d", ONE_FORM),
    plain("onrefunset3d", ONE_FORM),
    plain("onrefattach", ONE_FORM),
    plain("oncellattach", ONE_FORM),
    plain("oncelldetach", ONE_FORM),
    plain("oncellrefsloaded", ONE_FORM),
];

/// A handler (`EventCallback`): its function script (`Maybe_Lambda`, `saved` once its lambda's
/// variables are kept), the old filters (form addresses, 0 for none), the new ones by index
/// (`SelfOwningArrayElement`s: an array filter keeps a reference to its array).
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Callback {
    pub script: u32,
    pub saved: bool,
    pub source: u32,
    pub object: u32,
    pub removed: bool,
    pub pending_remove: bool,
    pub flush_on_load: bool,
    pub filters: BTreeMap<u32, Data>,
}

impl Callback {
    pub fn new(script: u32) -> Callback {
        Callback {
            script,
            ..Callback::default()
        }
    }
}

/// A handler in its event's list: its priority and the serial that keeps the order within one.
#[derive(Clone, Debug)]
pub struct Entry {
    pub priority: i32,
    pub serial: u64,
    pub callback: Callback,
}

/// An event (`EventInfo`).
#[derive(Clone, Debug)]
pub struct EventInfo {
    pub name: Vec<u8>,
    pub alias: Option<Vec<u8>>,
    pub params: Vec<u8>,
    pub mask: u32,
    hook: Hook,
    pub flags: u32,
    pub callbacks: Vec<Entry>,
    has_ptr_arg: bool,
}

impl EventInfo {
    fn unknown_arg_types(&self) -> bool {
        self.flags & FLAG_UNKNOWN_ARG_TYPES != 0
    }
    /// `IsUserDefined`: either of its two bits (so the test event, which allows script dispatch,
    /// counts too).
    fn user_defined(&self) -> bool {
        self.flags & FLAG_USER_DEFINED != 0
    }
    /// `TryGetNthParamType`.
    fn nth_param_type(&self, n: usize) -> u8 {
        if self.unknown_arg_types() {
            ANYTHING
        } else {
            non_ptr(self.params[n])
        }
    }
    /// `GetArgTypesAsStackVector`.
    pub fn arg_types(&self) -> Vec<u8> {
        self.params.iter().map(|&t| non_ptr(t)).collect()
    }
}

/// A deprecated-path call kept for the tick (`DeferredDeprecatedCallback`).
#[derive(Clone, Debug)]
struct Classic {
    info: usize,
    serial: u64,
    arg0: u32,
    arg1: u32,
    types: Vec<u8>,
    cleanup: Option<fn(&mut Engine)>,
    /// The text of each string argument, copied when the call was kept: the string it pointed
    /// at (a save's name on the stack, a message) is gone by the tick.
    texts: Vec<Option<Vec<u8>>>,
}

/// A newer-path dispatch kept for the tick (`DeferredCallback`).
#[derive(Clone, Debug)]
struct Deferred {
    info: usize,
    this_obj: u32,
    args: Vec<u32>,
    types: Vec<u8>,
    /// As [`Classic::texts`].
    texts: Vec<Option<Vec<u8>>>,
}

/// The event manager's state.
#[derive(Default)]
pub struct State {
    /// `s_eventInfos`: the events in registration order (internal ids are their index).
    pub infos: Vec<EventInfo>,
    /// `s_eventInfoMap`: name hash to event.
    names: HashMap<u32, usize>,
    /// `s_eventsInUse`: the masks of the events with handlers.
    pub in_use: u32,
    hooks: u8,
    /// `s_eventStack`: the events being handled, innermost last, per game thread (xNVSE keeps one
    /// under the lock it holds for a whole dispatch; here the lock is given back at safe points,
    /// so each thread keeps its own).
    stack: HashMap<u32, Vec<usize>>,
    /// `s_deferredRemoveList`.
    removals: Vec<(usize, u64)>,
    classic: Vec<Classic>,
    deferred: Vec<Deferred>,
    deferred_alt: Vec<Deferred>,
    /// `s_lastObj`, `s_lastTarget`, `s_lastEvent`; the `OnHitWith` and `OnHit` pairs.
    last: (u32, u32, u32),
    last_hit_with: (u32, u32),
    last_hit: (u32, u32),
    next_serial: u64,
    /// `recursiveLevel`: form lists inside form lists.
    recursion: u32,
    /// The fixes preset's row [`set_error_guard`], and the game threads a `RuntimeScriptError`
    /// dispatch is under way on (`thread_host::current_id`): an error of another thread is its
    /// own, not one raised by a handler.
    pub error_guard: bool,
    pub in_error_event: Vec<u32>,
    /// The fixes preset's row `crate::call_sites::set_imod_form_check`.
    pub imod_form_check: bool,
    /// `NV_NVSE_EVENT_LOG`: every handler call, one line each (a test variable).
    log: Option<std::fs::File>,
}

/// The fixes preset's row (docs/FIXES.md, "The script extender"): an error raised while a
/// `RuntimeScriptError` handler runs is not dispatched again (xNVSE recurses until the stack
/// runs out, as for an error in the handler itself or the call depth limit's).
pub fn set_error_guard(e: &mut Engine, on: bool) {
    state(e).error_guard = on;
}

fn state(e: &mut Engine) -> &mut State {
    &mut e.ext::<Nvse>().events
}

fn non_ptr(t: u8) -> u8 {
    if is_ptr(t) {
        t - FLOAT_PTR
    } else {
        t
    }
}

fn is_ptr(t: u8) -> bool {
    (FLOAT_PTR..=BASE_FORM_PTR).contains(&t)
}

fn is_form_param(t: u8) -> bool {
    matches!(t, ANY_FORM | REFERENCE | BASE_FORM | ANYTHING)
}

/// `Script::VariableType`: float, int, string, array, ref, invalid.
const VAR_FLOAT: u8 = 0;
const VAR_STRING: u8 = 2;
const VAR_ARRAY: u8 = 3;
const VAR_REF: u8 = 4;
const VAR_INVALID: u8 = 5;

/// `ParamTypeToVarType`.
fn param_to_var(t: u8) -> u8 {
    match t {
        INT | FLOAT => VAR_FLOAT,
        STRING => VAR_STRING,
        ARRAY => VAR_ARRAY,
        ANY_FORM | REFERENCE | BASE_FORM => VAR_REF,
        _ => VAR_INVALID,
    }
}

/// `VarTypeToParamType`.
pub fn var_to_param(t: u8) -> u8 {
    match t {
        0 => FLOAT,
        1 => INT,
        2 => STRING,
        3 => ARRAY,
        4 => ANY_FORM,
        _ => INVALID,
    }
}

/// `ParamTypeToDataType`: number, string, array, form, invalid.
fn param_to_data(t: u8) -> u8 {
    match t {
        INT | FLOAT => 1,
        STRING => 3,
        ARRAY => 4,
        ANY_FORM | REFERENCE | BASE_FORM => 2,
        _ => 0,
    }
}

/// `DataTypeToVarType`.
fn data_to_var(t: u8) -> u8 {
    match t {
        1 => VAR_FLOAT,
        2 => VAR_REF,
        3 => VAR_STRING,
        4 => VAR_ARRAY,
        _ => VAR_INVALID,
    }
}

/// `ParamTypeMatches(from, to)`.
pub fn param_type_matches(from: u8, to: u8) -> bool {
    from == to
        || (from == ANY_FORM && matches!(to, ANY_FORM | REFERENCE | BASE_FORM))
        || to == ANYTHING
}

/// `kLwrCaseConverter`: ASCII only.
fn lower(c: u8) -> u8 {
    c.to_ascii_lowercase()
}

/// `StrHashCI`.
pub fn hash_name(name: &[u8]) -> u32 {
    const SHIFTS: [u32; 4] = [4, 0xc, 0x14, 0];
    let mut h: u32 = 0x6b49_d20b;
    for (i, &c) in name.iter().take_while(|&&c| c != 0).enumerate() {
        h = h.wrapping_sub(u32::from(lower(c)) << SHIFTS[i % 4]);
        h = (h << 5).wrapping_sub(h);
    }
    h
}

/// `StrCompare` through the game's `toupper`.
fn str_eq(e: &mut Engine, a: &[u8], b: &[u8]) -> bool {
    let upper = vars::upper_table(e);
    vars::str_compare(&upper, a, b) == std::cmp::Ordering::Equal
}

fn lookup_form(e: &mut Engine, id: u32) -> u32 {
    e.call(LOOKUP_FORM, &args![id]).u32()
}

fn form_type(e: &Engine, form: u32) -> u8 {
    e.mem.u8(form + 4)
}

fn is_reference(e: &mut Engine, form: u32) -> bool {
    e.vcall(form, IS_REFERENCE, &args![]).bool()
}

/// A form list's forms as a range-`for` walks a `tList` (every node, a null form included).
fn list_forms(e: &Engine, list: u32) -> Vec<u32> {
    let mut out = Vec::new();
    let mut node = list + 0x18;
    while node != 0 {
        out.push(e.mem.u32(node));
        node = e.mem.u32(node + 4);
    }
    out
}

/// A form list's forms as `tList::Iterator` walks them (`End`: no node, or a last node with no
/// form).
fn list_forms_iter(e: &Engine, list: u32) -> Vec<u32> {
    let mut out = Vec::new();
    let mut node = list + 0x18;
    while node != 0 {
        let (data, next) = (e.mem.u32(node), e.mem.u32(node + 4));
        if data == 0 && next == 0 {
            break;
        }
        out.push(data);
        node = next;
    }
    out
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), GameObjects.cpp: GetPermanentBaseForm.
/// A reference's base form; for a temporary one (file `0xFF`), placeable water's water or a
/// levelled creature's real base form.
pub fn permanent_base_form(e: &mut Engine, refr: u32) -> u32 {
    if refr == 0 {
        return 0;
    }
    let base = crate::commands::base_form(e, refr);
    if base != 0 && e.mem.u32(base + 0xc) >> 24 == 0xff {
        let water = crate::commands::dynamic_cast(
            e,
            base,
            crate::commands::rtti::TES_FORM,
            RTTI_PLACEABLE_WATER,
        );
        if water != 0 {
            return e.mem.u32(water + 0x4c);
        }
        let extra = e
            .call(
                EXTRA_GET_BY_TYPE,
                &args![refr + 0x44, EXTRA_LEVELED_CREATURE],
            )
            .u32();
        if extra != 0 && e.mem.u32(extra + 0xc) != 0 {
            return e.mem.u32(extra + 0xc);
        }
    }
    base
}

/// `IsValidReference`: the vtable one of a reference's classes (an address that cannot be read
/// is not one, as xNVSE's `__try` answers).
fn is_valid_reference(e: &Engine, refr: u32) -> bool {
    let mut word = [0u8; 4];
    if e.mem.try_read(refr, &mut word).is_err() {
        return false;
    }
    REFERENCE_VTABLES.contains(&u32::from_le_bytes(word))
}

/// A `ShowRuntimeScriptError`: to the evaluator when there is one, else `ShowRuntimeError`.
fn script_error(e: &mut Engine, script: u32, ev: Option<&mut Eval>, text: String) {
    match ev {
        Some(ev) => ev.error_text(text),
        None => crate::errors::show(e, script, &text),
    }
}

// ---- the registry ----

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), EventManager.cpp: RegisterEventEx.
fn register(e: &mut Engine, def: &Def) -> bool {
    if def.params.len() as u32 > MAX_FILTERS {
        return false;
    }
    let s = state(e);
    let hash = hash_name(def.name.as_bytes());
    if s.names.contains_key(&hash) {
        return false;
    }
    let at = s.infos.len();
    s.infos.push(EventInfo {
        name: def.name.as_bytes().to_vec(),
        alias: def.alias.map(|a| a.as_bytes().to_vec()),
        params: def.params.to_vec(),
        mask: def.mask,
        hook: def.hook,
        flags: def.flags,
        callbacks: Vec::new(),
        has_ptr_arg: def.params.iter().any(|&t| is_ptr(t)),
    });
    s.names.insert(hash, at);
    if let Some(alias) = def.alias {
        s.names.entry(hash_name(alias.as_bytes())).or_insert(at);
    }
    true
}

/// `EventManager::Init`: the internal events, once.
pub fn init(e: &mut Engine) {
    if !state(e).infos.is_empty() {
        return;
    }
    for def in DEFS.iter() {
        register(e, def);
    }
}

/// `TryGetEventInfoForName`.
pub fn find(e: &mut Engine, name: &[u8]) -> Option<usize> {
    let hash = hash_name(name);
    state(e).names.get(&hash).copied()
}

/// `StrToLower` on a copy (`game_tolower`, byte by byte).
fn lowered(e: &mut Engine, name: &[u8]) -> Vec<u8> {
    name.iter()
        .map(|&c| e.call(GAME_TOLOWER, &args![u32::from(c)]).u32() as u8)
        .collect()
}

/// The event `name` names, made as a user-defined event when there is none
/// (`SetHandler<false>`'s insert).
fn find_or_add(e: &mut Engine, name: &[u8]) -> usize {
    if let Some(at) = find(e, name) {
        return at;
    }
    let name_copy = lowered(e, name);
    let s = state(e);
    let at = s.infos.len();
    s.infos.push(EventInfo {
        name: name_copy,
        alias: None,
        params: Vec::new(),
        mask: 0,
        hook: Hook::None,
        flags: FLAG_USER_DEFINED,
        callbacks: Vec::new(),
        has_ptr_arg: false,
    });
    s.names.insert(hash_name(name), at);
    at
}

fn info(e: &mut Engine, at: usize) -> &mut EventInfo {
    &mut state(e).infos[at]
}

/// The event at `at`.
pub fn state_info(e: &mut Engine, at: usize) -> &EventInfo {
    &state(e).infos[at]
}

/// How many events there are (`s_eventInfos`).
pub fn count(e: &mut Engine) -> usize {
    state(e).infos.len()
}

/// The entry with this serial, if the event still has it.
fn entry(e: &mut Engine, at: usize, serial: u64) -> Option<&mut Entry> {
    info(e, at)
        .callbacks
        .iter_mut()
        .find(|c| c.serial == serial)
}

/// The handler after `(priority, serial)` in the multimap's order (highest priority first, then
/// as set); the first with `None`.
fn next_after(e: &mut Engine, at: usize, after: Option<(i32, u64)>) -> Option<(i32, u64)> {
    info(e, at)
        .callbacks
        .iter()
        .find(|c| match after {
            None => true,
            Some((p, s)) => c.priority < p || (c.priority == p && c.serial > s),
        })
        .map(|c| (c.priority, c.serial))
}

/// `~EventCallback`: a kept lambda's variables given up, the array filters' references.
pub fn release(e: &mut Engine, cb: &Callback) {
    if cb.saved && cb.script != 0 {
        lambda::unsave_variables(e, cb.script);
    }
    for data in cb.filters.values() {
        release_data(e, data);
    }
}

/// `~SelfOwningArrayElement` for one value.
pub fn release_data(e: &mut Engine, data: &Data) {
    if let Data::Array(id) = data {
        let mut held = *id;
        let owner = vars::array_owning_mod(e, held);
        vars::remove_reference(e, &mut held, owner);
    }
}

/// `SelfOwningArrayElement::SetArray`: an array value keeps a reference from its own file.
pub fn hold_data(e: &mut Engine, data: &mut Data) {
    if let Data::Array(id) = data {
        let to = *id;
        let mut held = 0;
        let owner = vars::array_owning_mod(e, to);
        vars::add_reference(e, &mut held, to, owner);
        *id = held;
    }
}

/// `ArrayElement::operator==` for two filters: the same type and value (strings without case,
/// arrays by id).
fn data_eq(e: &mut Engine, a: &Data, b: &Data) -> bool {
    match (a, b) {
        (Data::Str(x), Data::Str(y)) => str_eq(e, x, y),
        (Data::Number(x), Data::Number(y)) => x == y,
        (Data::Form(x), Data::Form(y)) | (Data::Array(x), Data::Array(y)) => x == y,
        (Data::Invalid, Data::Invalid) => true,
        _ => false,
    }
}

/// `EventCallback::operator==`.
fn callback_eq(e: &mut Engine, a: &Callback, b: &Callback) -> bool {
    if a.script != b.script || a.object != b.object || a.source != b.source {
        return false;
    }
    if a.filters.len() != b.filters.len() {
        return false;
    }
    for ((ka, va), (kb, vb)) in a.filters.iter().zip(b.filters.iter()) {
        if ka != kb || !data_eq(e, va, vb) {
            return false;
        }
    }
    true
}

// ---- filters ----

/// `IsPotentialFilterValid`.
fn potential_filter_valid(
    e: &mut Engine,
    expected: u8,
    filter: &Data,
    num: u32,
) -> Result<(), String> {
    if expected == ANYTHING {
        return Ok(());
    }
    if let Data::Array(id) = filter {
        if !vars::array_exists(e, *id) {
            return Err(format!(
                "Invalid/uninitialized array filter was passed for filter #{num} (array id {id})."
            ));
        }
        return Ok(());
    }
    let expected_data = param_to_data(expected);
    let data = filter.type_code();
    if data != expected_data {
        return Err(format!(
            "Invalid type for filter #{num}: expected {}, got {}.",
            functions::data_type_name(expected_data),
            functions::data_type_name(data)
        ));
    }
    if let Data::Form(id) = filter {
        let form = lookup_form(e, *id);
        if form != 0 && expected == BASE_FORM && is_reference(e, form) {
            return Err(format!(
                "Expected BaseForm-type filter for filter #{num}, got Reference."
            ));
        }
    }
    Ok(())
}

/// `ValidateFirstOrSecondFilter`.
fn first_or_second_valid(
    e: &mut Engine,
    cb: &Callback,
    first: bool,
    info: &EventInfo,
) -> Result<(), String> {
    let filter = if first { cb.source } else { cb.object };
    if filter == 0 {
        return Ok(());
    }
    let name = if first {
        r#""first"/"source""#
    } else {
        r#""second"/"object""#
    };
    if first {
        if info.params.is_empty() {
            return Err(r#"Cannot use "first"/"source" filter; event has 0 args."#.to_string());
        }
    } else if info.params.len() < 2 {
        return Err(format!(
            r#"Cannot use "second"/"object" filter; event has {} args."#,
            info.params.len()
        ));
    }
    let expected = non_ptr(info.params[usize::from(!first)]);
    if !is_form_param(expected) {
        return Err(format!("Cannot set a non-Form type filter for {name}."));
    }
    if expected == BASE_FORM && is_reference(e, filter) {
        return Err(format!(
            "Expected BaseForm-type filter for filter {name}, got Reference."
        ));
    }
    Ok(())
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), EventManager.cpp:
// EventCallback::ValidateFilters.
fn validate_filters(e: &mut Engine, cb: &Callback, info: &EventInfo) -> Result<(), String> {
    if info.unknown_arg_types() {
        return Ok(());
    }
    first_or_second_valid(e, cb, true, info)?;
    first_or_second_valid(e, cb, false, info)?;
    for (&index, filter) in &cb.filters {
        if index as usize > info.params.len() {
            return Err(format!(
                "Index {index} exceeds max number of args for event ({})",
                info.params.len()
            ));
        }
        let expected = if index == 0 {
            REFERENCE
        } else {
            non_ptr(info.params[index as usize - 1])
        };
        potential_filter_valid(e, expected, filter, index)?;
    }
    Ok(())
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), EventManager.cpp: DoesFormMatchFilter.
pub fn form_matches_filter(
    e: &mut Engine,
    input: u32,
    filter: u32,
    expect_reference: bool,
    depth: u32,
) -> bool {
    if filter == input {
        return true;
    }
    if filter == 0 || input == 0 || depth > 100 {
        return false;
    }
    if form_type(e, filter) == LIST_FORM {
        let filters = list_forms(e, filter);
        if form_type(e, input) == LIST_FORM {
            for f in list_forms(e, input) {
                let mut found = false;
                for &g in &filters {
                    if form_matches_filter(e, f, g, expect_reference, depth + 1) {
                        found = true;
                        break;
                    }
                }
                if !found {
                    return false;
                }
            }
            return true;
        }
        for g in filters {
            if form_matches_filter(e, input, g, expect_reference, depth + 1) {
                return true;
            }
        }
    } else {
        if form_type(e, input) == LIST_FORM {
            for f in list_forms(e, input) {
                // xNVSE leaves the depth at its default here.
                if !form_matches_filter(e, f, filter, expect_reference, 0) {
                    return false;
                }
            }
            return true;
        }
        if expect_reference && is_reference(e, input) && filter == permanent_base_form(e, input) {
            return true;
        }
    }
    false
}

/// `float` equality within `kFloatEpsilon`.
fn float_equal(a: f32, b: f32) -> bool {
    let mut m = a - b;
    if m < 0.0 {
        m = -m;
    }
    m < 0.0001
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), EventManager.h: DoesFilterMatch.
/// Whether one filter value matches an argument word (`alt`: numbers always passed as `float`
/// bits, `ExtractIntTypeAsFloat`).
pub fn filter_matches(e: &mut Engine, filter: &Data, arg: u32, arg_type: u8, alt: bool) -> bool {
    match filter {
        Data::Number(n) => {
            let mut number = *n;
            if arg_type == INT {
                number = number.floor();
            }
            let input = if alt {
                let v = f32::from_bits(arg);
                if arg_type == INT {
                    v.floor()
                } else {
                    v
                }
            } else if arg_type == INT {
                arg as f32
            } else {
                f32::from_bits(arg)
            };
            float_equal(input, number as f32)
        }
        Data::Form(id) => {
            let form = lookup_form(e, *id);
            form_matches_filter(e, arg, form, arg_type != BASE_FORM, 0)
        }
        Data::Str(text) => {
            if arg == 0 {
                return false;
            }
            let input = crate::table::guest_text(e, arg);
            str_eq(e, text, &input)
        }
        Data::Array(id) => {
            if arg == 0 || !vars::array_exists(e, arg) || !vars::array_exists(e, *id) {
                return false;
            }
            arrays_equal(e, arg, *id)
        }
        Data::Invalid => true,
    }
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), ArrayVar.cpp: ArrayVar::CompareArrays
// (not deep).
fn arrays_equal(e: &mut Engine, a: u32, b: u32) -> bool {
    let (Some(x), Some(y)) = (vars::array(e, a).cloned(), vars::array(e, b).cloned()) else {
        return false;
    };
    if x.len() != y.len() {
        return false;
    }
    if a == b {
        return true;
    }
    let upper = vars::upper_table(e);
    for (kx, ky) in x.keys().iter().zip(y.keys().iter()) {
        if kx.type_code() != ky.type_code() {
            return false;
        }
        let same_key = match (kx, ky) {
            (Key::Number(p), Key::Number(q)) => p == q,
            (Key::Str(p), Key::Str(q)) => {
                vars::str_compare(&upper, p, q) == std::cmp::Ordering::Equal
            }
            _ => true,
        };
        if !same_key {
            return false;
        }
        let (Some(ex), Some(ey)) = (x.get(&upper, kx), y.get(&upper, ky)) else {
            return false;
        };
        if ex.data.type_code() != ey.data.type_code() || !data_eq(e, &ex.data, &ey.data) {
            return false;
        }
    }
    true
}

/// The array a filter holds, or the error xNVSE reports for a missing one.
fn filter_array(e: &mut Engine, filter: &Data) -> Result<vars::ArrayVar, u32> {
    let id = match filter {
        Data::Array(id) => *id,
        _ => 0,
    };
    vars::array(e, id).cloned().ok_or(id)
}

/// The values of an array, in its order.
fn array_values(a: &vars::ArrayVar) -> Vec<Data> {
    match &a.elements {
        vars::Container::Array(v) => v.iter().map(|x| x.data.clone()).collect(),
        vars::Container::NumMap(v) => v.iter().map(|(_, x)| x.data.clone()).collect(),
        vars::Container::StrMap(v) => v.iter().map(|(_, x)| x.data.clone()).collect(),
    }
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), EventManager.h:
// EventCallback::DoesParamMatchFiltersInArray.
fn param_matches_filters_in_array(
    e: &mut Engine,
    script: u32,
    filter: &Data,
    param_type: u8,
    param: u32,
    index: u32,
    alt: bool,
) -> bool {
    let arr = match filter_array(e, filter) {
        Ok(a) => a,
        Err(id) => {
            let text = format!("While checking event filters in array at index {index}, the array was invalid/uninitialized (array id: {id}).");
            crate::errors::show(e, script, &text);
            return false;
        }
    };
    for elem in array_values(&arr) {
        if param_to_var(param_type) != data_to_var(elem.type_code()) && param_type != ANYTHING {
            continue;
        }
        if filter_matches(e, &elem, param, param_type, alt) {
            return true;
        }
    }
    false
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), EventManager.h:
// EventCallback::DoNewFiltersMatch.
#[allow(clippy::too_many_arguments)]
fn new_filters_match(
    e: &mut Engine,
    cb: &Callback,
    this_obj: u32,
    args: &[u32],
    types: &[u8],
    name: &[u8],
    mut ev: Option<&mut Eval>,
    alt: bool,
) -> bool {
    let event = String::from_utf8_lossy(name).into_owned();
    if args.len() != types.len() {
        let text = format!("While checking if new filters match for event {event}, # of expected args ({}) != # of args passed ({})", types.len(), args.len());
        script_error(e, cb.script, ev, text);
        return false;
    }
    for (&index, filter) in &cb.filters {
        if index as usize > types.len() {
            let text = format!("While checking if new filters match for event {event}, saw filter at invalid index #{index} (# of args: {})", types.len());
            script_error(e, cb.script, ev, text);
            return false;
        }
        let (arg, arg_type) = if index == 0 {
            (this_obj, REFERENCE)
        } else {
            (args[index as usize - 1], types[index as usize - 1])
        };
        let filter_var = data_to_var(filter.type_code());
        let arg_var = param_to_var(arg_type);
        if filter_var != arg_var {
            if filter_var != VAR_ARRAY {
                let text = format!(
                    "While checking if new filters match for event {event}, filter #{index}'s type ({}) did not match the arg's ({})",
                    functions::variable_type_name(filter_var),
                    functions::variable_type_name(arg_var)
                );
                script_error(e, cb.script, ev.as_deref_mut(), text);
                return false;
            }
            if !param_matches_filters_in_array(e, cb.script, filter, arg_type, arg, index, alt) {
                return false;
            }
            continue;
        }
        if filter_matches(e, filter, arg, arg_type, alt) {
            continue;
        }
        if filter_var == VAR_ARRAY
            && param_matches_filters_in_array(e, cb.script, filter, arg_type, arg, index, alt)
        {
            continue;
        }
        return false;
    }
    true
}

/// `DoNewFiltersMatch<true>` for a query (`GetEventHandlers`): numbers as `float` bits.
pub fn filters_match_alt(
    e: &mut Engine,
    cb: &Callback,
    this_obj: u32,
    args: &[u32],
    types: &[u8],
    name: &[u8],
    ev: Option<&mut Eval>,
) -> bool {
    new_filters_match(e, cb, this_obj, args, types, name, ev, true)
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), EventManager.cpp:
// EventCallback::DoDeprecatedFiltersMatch.
fn deprecated_filters_match(
    e: &mut Engine,
    cb: &Callback,
    args: &[u32],
    types: Option<&[u8]>,
    name: &[u8],
    mut ev: Option<&mut Eval>,
) -> bool {
    let event = String::from_utf8_lossy(name).into_owned();
    if let Some(types) = types {
        if args.len() != types.len() {
            let text = format!("While checking if first/second filters match for event {event}, there was a different # of ArgTypes vs. Args");
            script_error(e, cb.script, ev, text);
            return false;
        }
    }
    if cb.source != 0 {
        if let Some(types) = types {
            if args.is_empty() {
                let text = format!("While checking if first/second filters match for User-Defined event {event}, saw a filter for an event that did not dispatch anything.");
                script_error(e, cb.script, ev, text);
                return false;
            }
            if !is_form_param(types[0]) {
                let text = format!("While checking if first/second filters match for User-Defined event {event}, saw a form filter for 'first' arg when a non-form was dispatched.");
                script_error(e, cb.script, ev, text);
                return false;
            }
        }
        if !form_matches_filter(e, args[0], cb.source, true, 0) {
            return false;
        }
    }
    if cb.object != 0 {
        if let Some(types) = types {
            if args.len() < 2 {
                let text = format!("While checking if first/second filters match for User-Defined event {event}, saw a filter for 'second' arg when no 2nd arg was dispatched.");
                script_error(e, cb.script, ev.as_deref_mut(), text);
                return false;
            }
            // xNVSE checks the first argument's type here too.
            if !is_form_param(types[0]) {
                let text = format!("While checking if first/second filters match for User-Defined event {event}, saw a form filter for 'second' arg when a non-form was dispatched.");
                script_error(e, cb.script, ev, text);
                return false;
            }
        }
        if !form_matches_filter(e, args[1], cb.object, true, 0) {
            return false;
        }
    }
    true
}

/// Runs `f` with a filter value as a dispatch would pass it (`ArrayData::GetAsVoidArg`): a
/// number's `float` bits, a form's address, a string copied to the guest stack, an array's id.
fn with_void_arg<T>(e: &mut Engine, data: &Data, f: impl FnOnce(&mut Engine, u32) -> T) -> T {
    match data {
        Data::Number(n) => f(e, (*n as f32).to_bits()),
        Data::Form(id) => {
            let form = lookup_form(e, *id);
            f(e, form)
        }
        Data::Str(s) => with_guest_text(e, s, f),
        Data::Array(id) => f(e, *id),
        Data::Invalid => f(e, 0),
    }
}

/// Runs `f` with `text` as a C string on the guest stack.
pub fn with_guest_text<T>(e: &mut Engine, text: &[u8], f: impl FnOnce(&mut Engine, u32) -> T) -> T {
    let size = (text.len() as u32 + 1).next_multiple_of(4);
    e.with_stack(size, |e, at| {
        e.mem.write(at.addr(), text);
        e.mem.set_u8(at.addr() + text.len() as u32, 0);
        f(e, at.addr())
    })
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), EventManager.cpp:
// EventCallback::ShouldRemoveCallback.
/// Whether `remove` (a handler being removed) covers `existing`: the same script, its old
/// filters equal or absent, each of its new filters as wide as `existing`'s.
fn should_remove(
    e: &mut Engine,
    remove: &Callback,
    existing: &Callback,
    info: &EventInfo,
    mut ev: Option<&mut Eval>,
) -> bool {
    if remove.script != existing.script {
        return false;
    }
    if remove.source != 0 && remove.source != existing.source {
        return false;
    }
    if remove.object != 0 && remove.object != existing.object {
        return false;
    }
    if remove.filters.is_empty() {
        return true;
    }
    if remove.filters.len() > existing.filters.len() {
        return false;
    }
    let script = remove.script;
    for (&index, to_remove) in &remove.filters {
        let Some(existing_filter) = existing.filters.get(&index) else {
            return false;
        };
        let param_type = if index == 0 {
            REFERENCE
        } else {
            info.nth_param_type(index as usize - 1)
        };
        let (rt, xt) = (to_remove.type_code(), existing_filter.type_code());
        if rt == xt {
            if rt == 4 {
                if (param_type == ANYTHING || param_type == ARRAY)
                    && with_void_arg(e, existing_filter, |e, w| {
                        filter_matches(e, to_remove, w, param_type, true)
                    })
                {
                    continue;
                }
                let existing_values = match filter_array(e, existing_filter) {
                    Ok(a) => array_values(&a),
                    Err(id) => {
                        let text = format!("While checking event filters in array at index {index}, the array was invalid/uninitialized (array id: {id}).");
                        script_error(e, script, ev.as_deref_mut(), text);
                        return false;
                    }
                };
                let mut all = true;
                for v in &existing_values {
                    let ok = with_void_arg(e, v, |e, w| {
                        param_matches_filters_in_array(
                            e, script, to_remove, param_type, w, index, true,
                        )
                    });
                    if !ok {
                        all = false;
                        break;
                    }
                }
                if all {
                    continue;
                }
                if param_type != ANYTHING && param_type != ARRAY {
                    return false;
                }
                let mut all = true;
                for v in &existing_values {
                    let ok = with_void_arg(e, v, |e, w| {
                        filter_matches(e, to_remove, w, param_type, true)
                    });
                    if !ok {
                        all = false;
                        break;
                    }
                }
                if all {
                    continue;
                }
                if with_void_arg(e, existing_filter, |e, w| {
                    param_matches_filters_in_array(e, script, to_remove, param_type, w, index, true)
                }) {
                    continue;
                }
                return false;
            }
            if !with_void_arg(e, existing_filter, |e, w| {
                filter_matches(e, to_remove, w, param_type, true)
            }) {
                return false;
            }
        } else if rt == 4 {
            if !with_void_arg(e, existing_filter, |e, w| {
                param_matches_filters_in_array(e, script, to_remove, param_type, w, index, true)
            }) {
                return false;
            }
        } else if xt == 4 {
            let values = match filter_array(e, existing_filter) {
                Ok(a) => array_values(&a),
                Err(id) => {
                    let text = format!("While checking event filters in array at index {index}, the array was invalid/uninitialized (array id: {id}).");
                    script_error(e, script, ev.as_deref_mut(), text);
                    return false;
                }
            };
            for v in &values {
                if !with_void_arg(e, v, |e, w| {
                    filter_matches(e, to_remove, w, param_type, true)
                }) {
                    return false;
                }
            }
        } else {
            let text = format!(
                "While comparing event filters at index {index}, encountered a type mismatch (to-remove filter type: {}, existing filter type: {}).",
                functions::data_type_name(rt),
                functions::data_type_name(xt)
            );
            script_error(e, script, ev.as_deref_mut(), text);
            return false;
        }
    }
    true
}

// ---- setting and removing ----

/// What the hooks on paths every thread runs often read without the lock (a lock-free shared
/// slot), stored under the lock whenever handlers or hooks change, so with no extender script
/// none of them takes the extender's state:
/// - `marks`: the masks of the game events the `005ac750` wrap passes on (`s_eventsInUse` once
///   the main hook is installed) and `kEventMask_OnActivate` once its own hook is;
/// - `handlers`: the internal events (ids below 64) with a handler, for the call sites'
///   events (`crate::call_sites`).
#[derive(Default)]
pub struct LockFree {
    marks: std::sync::atomic::AtomicU32,
    handlers: std::sync::atomic::AtomicU64,
}

fn publish_lock_free(e: &mut Engine) {
    let s = state(e);
    let mut marks = 0;
    if s.hooks & HOOK_MAIN != 0 {
        marks |= s.in_use & !MASK_ON_ACTIVATE;
    }
    if s.hooks & HOOK_ACTIVATE != 0 {
        marks |= s.in_use & MASK_ON_ACTIVATE;
    }
    let handlers = s
        .infos
        .iter()
        .take(64)
        .enumerate()
        .filter(|(_, i)| !i.callbacks.is_empty())
        .fold(0u64, |bits, (at, _)| bits | (1 << at));
    let free = e.ext_free::<LockFree>();
    free.marks
        .store(marks, std::sync::atomic::Ordering::Relaxed);
    free.handlers
        .store(handlers, std::sync::atomic::Ordering::Relaxed);
}

/// The game-event masks with handlers whose hook is installed, read without the lock.
pub fn marks_lock_free(e: &mut Engine) -> u32 {
    e.ext_free::<LockFree>()
        .marks
        .load(std::sync::atomic::Ordering::Relaxed)
}

/// Whether internal event `at` has a handler, read without the lock (a hint: the dispatch
/// checks again under it).
pub fn has_handlers_lock_free(e: &mut Engine, at: usize) -> bool {
    at < 64
        && e.ext_free::<LockFree>()
            .handlers
            .load(std::sync::atomic::Ordering::Relaxed)
            & (1 << at)
            != 0
}

/// `s_eventsInUse`.
pub fn in_use(e: &mut Engine) -> u32 {
    state(e).in_use
}

thread_local! {
    /// `g_inActivateItemInInventory` (thread-local in xNVSE too): set while one of
    /// `Actor::EquipObject`'s calls to `Actor::AddWornItem` runs (`crate::call_sites`), when the
    /// equip hook raises OnActorEquip itself.
    static IN_ADD_WORN_ITEM: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
}

/// Sets or clears `g_inActivateItemInInventory` for this thread.
pub fn set_in_add_worn_item(on: bool) {
    IN_ADD_WORN_ITEM.with(|f| f.set(on));
}

fn install_hook(e: &mut Engine, hook: Hook) {
    let s = state(e);
    match hook {
        Hook::None => {}
        Hook::Main => s.hooks |= HOOK_MAIN,
        Hook::ActorEquip => s.hooks |= HOOK_MAIN | HOOK_EQUIP,
        Hook::Activate => s.hooks |= HOOK_ACTIVATE,
    }
    publish_lock_free(e);
}

/// Whether the hook at `005ac750` is installed (a game event's handler was set once).
pub fn main_hook_installed(e: &mut Engine) -> bool {
    state(e).hooks & HOOK_MAIN != 0
}

/// Whether the `OnActorEquip` call hook at `004C0032` is installed.
pub fn equip_hook_installed(e: &mut Engine) -> bool {
    state(e).hooks & HOOK_EQUIP != 0
}

/// Whether the `OnActivate` hook at `0057318E` is installed.
pub fn activate_hook_installed(e: &mut Engine) -> bool {
    state(e).hooks & HOOK_ACTIVATE != 0
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), EventManager.cpp: DoSetHandler.
/// Adds the handler at `priority`; the callback is kept (and `true`) or given back to be
/// released by the caller (`false`, or `true` when an equal removed one was set again).
fn do_set(e: &mut Engine, at: usize, cb: Callback, priority: i32) -> (bool, Option<Callback>) {
    if cb.script != 0 {
        for first in [true, false] {
            let list = if first { cb.source } else { cb.object };
            if list != 0 && form_type(e, list) == LIST_FORM && state(e).recursion < 100 {
                let mut set = 0;
                for form in list_forms_iter(e, list) {
                    let mut item = Callback::new(cb.script);
                    if first {
                        item.source = form;
                        item.object = cb.object;
                    } else {
                        item.source = cb.source;
                        item.object = form;
                    }
                    state(e).recursion += 1;
                    let (ok, back) = do_set(e, at, item, priority);
                    state(e).recursion -= 1;
                    if let Some(back) = back {
                        release(e, &back);
                    }
                    if ok {
                        set += 1;
                    }
                }
                return (set > 0, Some(cb));
            }
        }
    }
    let hook = info(e, at).hook;
    if hook != Hook::None {
        install_hook(e, hook);
        info(e, at).hook = Hook::None;
    }
    let same: Vec<u64> = info(e, at)
        .callbacks
        .iter()
        .filter(|c| c.priority == priority)
        .map(|c| c.serial)
        .collect();
    for serial in same {
        let existing = entry(e, at, serial).map(|x| x.callback.clone());
        let Some(existing) = existing else {
            continue;
        };
        if callback_eq(e, &existing, &cb) {
            if existing.removed {
                if let Some(x) = entry(e, at, serial) {
                    x.callback.removed = false;
                }
                return (true, Some(cb));
            }
            return (false, Some(cb));
        }
    }
    let mut cb = cb;
    // `Confirm`: a lambda's variables are kept while it is a handler.
    if cb.script != 0 {
        lambda::save_variables(e, cb.script);
        cb.saved = true;
    }
    let s = state(e);
    s.next_serial += 1;
    let serial = s.next_serial;
    let list = &mut s.infos[at].callbacks;
    let pos = list.partition_point(|c| c.priority >= priority);
    list.insert(
        pos,
        Entry {
            priority,
            serial,
            callback: cb,
        },
    );
    s.in_use |= s.infos[at].mask;
    publish_lock_free(e);
    (true, None)
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), EventManager.h: SetHandler<false>.
/// `SetEventHandler`'s and `SetEventHandlerAlt`'s add. The callback is consumed: kept, or
/// released.
pub fn set_handler(
    e: &mut Engine,
    name: &[u8],
    cb: Callback,
    priority: i32,
    ev: Option<&mut Eval>,
) -> bool {
    if cb.script == 0 {
        release(e, &cb);
        return false;
    }
    let range_error = if priority == PRIORITY_INVALID {
        Some(format!("Can't set event handler with invalid priority 0 (below 0 is allowed). Default priority = {PRIORITY_DEFAULT}."))
    } else if priority > PRIORITY_MAX {
        Some(format!(
            "Can't set event handler with priority above {PRIORITY_MAX}."
        ))
    } else if priority < PRIORITY_MIN {
        Some(format!(
            "Can't set event handler with priority below {}.",
            PRIORITY_MIN as u32
        ))
    } else {
        None
    };
    if let Some(text) = range_error {
        script_error(e, cb.script, ev, text);
        release(e, &cb);
        return false;
    }
    let at = find_or_add(e, name);
    let snapshot = info(e, at).clone();
    if let Err(text) = validate_filters(e, &cb, &snapshot) {
        script_error(e, cb.script, ev, text);
        release(e, &cb);
        return false;
    }
    let (ok, back) = do_set(e, at, cb, priority);
    if let Some(back) = back {
        release(e, &back);
    }
    ok
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), EventManager.cpp: DoRemoveHandler.
fn do_remove(
    e: &mut Engine,
    at: usize,
    remove: &Callback,
    priority: i32,
    mut ev: Option<&mut Eval>,
) -> bool {
    if remove.script != 0 {
        for first in [true, false] {
            let list = if first { remove.source } else { remove.object };
            if list != 0 && form_type(e, list) == LIST_FORM && state(e).recursion < 100 {
                let mut removed = 0;
                for form in list_forms_iter(e, list) {
                    let mut item = Callback::new(remove.script);
                    if first {
                        item.source = form;
                        item.object = remove.object;
                    } else {
                        item.source = remove.source;
                        item.object = form;
                    }
                    state(e).recursion += 1;
                    let ok = do_remove(e, at, &item, priority, ev.as_deref_mut());
                    state(e).recursion -= 1;
                    if ok {
                        removed += 1;
                    }
                }
                return removed > 0;
            }
        }
    }
    let snapshot = info(e, at).clone();
    let mut any = false;
    for x in &snapshot.callbacks {
        if priority != PRIORITY_INVALID && x.priority != priority {
            continue;
        }
        if !should_remove(e, remove, &x.callback, &snapshot, ev.as_deref_mut()) {
            continue;
        }
        let Some(live) = entry(e, at, x.serial) else {
            continue;
        };
        if !live.callback.removed {
            live.callback.removed = true;
            any = true;
        }
        if !live.callback.pending_remove {
            live.callback.pending_remove = true;
            state(e).removals.push((at, x.serial));
        }
    }
    any
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), EventManager.cpp: RemoveHandler.
/// `RemoveEventHandler`: marks every handler `remove` covers (priority 0: at any priority).
pub fn remove_handler(
    e: &mut Engine,
    name: &[u8],
    remove: &Callback,
    priority: i32,
    ev: Option<&mut Eval>,
) -> bool {
    if remove.script == 0 {
        return false;
    }
    if priority > PRIORITY_MAX {
        let text = format!("Can't remove event handler with priority above {PRIORITY_MAX}.");
        script_error(e, remove.script, ev, text);
        return false;
    }
    if priority < PRIORITY_MIN {
        let text = format!(
            "Can't remove event handler with priority below {}.",
            PRIORITY_MIN as u32
        );
        script_error(e, remove.script, ev, text);
        return false;
    }
    let Some(at) = find(e, name) else {
        return false;
    };
    let snapshot = info(e, at).clone();
    if let Err(text) = validate_filters(e, remove, &snapshot) {
        if let Some(ev) = ev {
            ev.error_text(text);
        }
        return false;
    }
    do_remove(e, at, remove, priority, ev)
}

/// Erases a handler for good (`~DeferredRemoveCallback` when it is still removed, a flush).
fn erase(e: &mut Engine, at: usize, serial: u64) {
    let list = &mut info(e, at).callbacks;
    let Some(i) = list.iter().position(|c| c.serial == serial) else {
        return;
    };
    let gone = list.remove(i);
    release(e, &gone.callback);
    let s = state(e);
    if s.infos[at].callbacks.is_empty() && s.infos[at].mask != 0 {
        s.in_use &= !s.infos[at].mask;
    }
    publish_lock_free(e);
}

/// `s_deferredRemoveList.Clear()`: the handlers still removed are erased, the others kept.
fn clear_removals(e: &mut Engine) {
    while let Some((at, serial)) = state(e).removals.pop() {
        let removed = entry(e, at, serial).map(|x| x.callback.removed);
        match removed {
            Some(true) => erase(e, at, serial),
            Some(false) => {
                if let Some(x) = entry(e, at, serial) {
                    x.callback.pending_remove = false;
                }
            }
            None => {}
        }
    }
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), EventManager.cpp:
// ClearFlushOnLoadEventHandlers.
/// A game loaded or a new one: the handlers `SetEventHandlerAlt` set (and every handler of an
/// event that flushes) go.
pub fn clear_flush_on_load(e: &mut Engine) {
    clear_removals(e);
    for at in 0..state(e).infos.len() {
        let flushes = info(e, at).flags & FLAG_FLUSH_ON_LOAD != 0;
        let gone: Vec<Entry> = if flushes {
            std::mem::take(&mut info(e, at).callbacks)
        } else {
            let list = std::mem::take(&mut info(e, at).callbacks);
            let (gone, kept): (Vec<Entry>, Vec<Entry>) =
                list.into_iter().partition(|c| c.callback.flush_on_load);
            info(e, at).callbacks = kept;
            gone
        };
        for x in &gone {
            release(e, &x.callback);
        }
        let s = state(e);
        if s.infos[at].callbacks.is_empty() && s.infos[at].mask != 0 {
            s.in_use &= !s.infos[at].mask;
        }
    }
    publish_lock_free(e);
}

// ---- dispatch ----

/// `GetCurrentEventName`: the innermost event being handled, empty when none.
pub fn current_event_name(e: &mut Engine) -> Vec<u8> {
    let thread = engine_core::thread_host::current_id(e);
    let s = state(e);
    let top = s.stack.get(&thread).and_then(|v| v.last()).copied();
    top.map(|at| s.infos[at].name.clone()).unwrap_or_default()
}

fn push_event(e: &mut Engine, at: usize) {
    let thread = engine_core::thread_host::current_id(e);
    state(e).stack.entry(thread).or_default().push(at);
}

fn pop_event(e: &mut Engine) {
    let thread = engine_core::thread_host::current_id(e);
    if let Some(v) = state(e).stack.get_mut(&thread) {
        v.pop();
    }
}

fn on_main_thread(e: &mut Engine) -> bool {
    let thread = engine_core::thread_host::current_id(e);
    e.ext::<Nvse>().tick.main_thread == thread
}

/// The text of each string argument (by the argument types), for a call kept for the tick.
fn copy_texts(e: &Engine, args: &[u32], types: &[u8]) -> Vec<Option<Vec<u8>>> {
    args.iter()
        .enumerate()
        .map(|(i, &a)| {
            (types.get(i) == Some(&STRING) && a != 0).then(|| crate::table::guest_text(e, a))
        })
        .collect()
}

/// Runs `f` with the arguments of a kept call, each copied text on the guest stack again.
fn with_texts<R>(
    e: &mut Engine,
    args: &[u32],
    texts: &[Option<Vec<u8>>],
    f: impl FnOnce(&mut Engine, Vec<u32>) -> R,
) -> R {
    let size: u32 = texts
        .iter()
        .flatten()
        .map(|t| (t.len() as u32 + 1).next_multiple_of(4))
        .sum();
    e.with_stack(size.max(4), |e, base| {
        let mut at = base.addr();
        let mut words = args.to_vec();
        for (i, t) in texts.iter().enumerate() {
            if let (Some(t), Some(w)) = (t, words.get_mut(i)) {
                e.mem.write(at, t);
                e.mem.set_u8(at + t.len() as u32, 0);
                *w = at;
                at += (t.len() as u32 + 1).next_multiple_of(4);
            }
        }
        f(e, words)
    })
}

/// `NV_NVSE_EVENT_LOG`: a handler call as one line (event, handler, game thread, arguments).
fn log_call(e: &mut Engine, at: usize, script: u32, args: &[u32]) {
    log_line(e, at, "handler", script, args);
}

/// `NV_NVSE_EVENT_LOG`: a call kept for the main thread's tick, on the thread that raised it
/// (handler 0: the handlers are found at the tick).
fn log_kept(e: &mut Engine, at: usize, script: u32, args: &[u32]) {
    log_line(e, at, "kept for", script, args);
}

fn log_line(e: &mut Engine, at: usize, what: &str, script: u32, args: &[u32]) {
    if state(e).log.is_none() {
        return;
    }
    use std::io::Write;
    let thread = engine_core::thread_host::current_id(e);
    let id = if script != 0 {
        e.mem.u32(script + 0xc)
    } else {
        0
    };
    // The form ids of the form arguments (by the event's parameter types), for comparing runs.
    let params = state(e).infos[at].params.clone();
    let ids: Vec<String> = args
        .iter()
        .enumerate()
        .filter(|&(i, _)| {
            params
                .get(i)
                .is_some_and(|&p| matches!(non_ptr(p), REFERENCE | ANY_FORM | BASE_FORM))
        })
        .map(|(_, &a)| {
            let mut word = [0u8; 4];
            if a != 0 && e.mem.try_read(a + 0xc, &mut word).is_ok() {
                format!("{:08X}", u32::from_le_bytes(word))
            } else {
                "-".to_string()
            }
        })
        .collect();
    let s = state(e);
    let name = String::from_utf8_lossy(&s.infos[at].name).into_owned();
    if let Some(f) = s.log.as_mut() {
        let _ = writeln!(
            f,
            "{name} {what} {id:08X} thread {thread} args {args:08X?} ids {ids:?}"
        );
    }
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), EventManager.cpp: EventCallback::Invoke
// (a script's), ClassicEventHandlerCaller.
fn invoke_classic(e: &mut Engine, at: usize, script: u32, types: &[u8], arg0: u32, arg1: u32) {
    let count = if arg0 == 0 {
        0
    } else if arg1 == 0 {
        1
    } else {
        2
    };
    let args = [arg0, arg1][..count].to_vec();
    push_event(e, at);
    log_call(e, at, script, &args);
    functions::call(
        e,
        Caller::Internal {
            script,
            this_obj: 0,
            containing: 0,
            args,
            alt: false,
            allow_surplus: false,
            event_types: Some(types.to_vec()),
        },
    );
    pop_event(e);
}

/// The validity of the first argument as a reference, read once per dispatch.
#[derive(Clone, Copy, PartialEq)]
enum RefState {
    NotSet,
    Invalid,
    Valid,
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), EventManager.cpp: HandleEvent (the
// deprecated one, on an EventInfo).
fn handle_classic(
    e: &mut Engine,
    at: usize,
    arg0: u32,
    arg1: u32,
    types: &[u8],
    mut ev: Option<&mut Eval>,
    cleanup: Option<fn(&mut Engine)>,
) {
    let mut args = Vec::new();
    if !types.is_empty() {
        args.push(arg0);
        if types.len() >= 2 {
            args.push(arg1);
        }
    }
    let name = info(e, at).name.clone();
    let mut arg0_valid = RefState::NotSet;
    let mut cursor = None;
    while let Some(key) = next_after(e, at, cursor) {
        cursor = Some(key);
        let Some(cb) = entry(e, at, key.1).map(|x| x.callback.clone()) else {
            continue;
        };
        if cb.removed {
            continue;
        }
        if cb.source != 0 && arg0 != cb.source {
            if arg0_valid == RefState::NotSet {
                arg0_valid = if is_valid_reference(e, arg0) {
                    RefState::Valid
                } else {
                    RefState::Invalid
                };
            }
            if arg0_valid == RefState::Invalid || permanent_base_form(e, arg0) != cb.source {
                continue;
            }
        }
        if cb.object != 0 && cb.object != arg1 {
            continue;
        }
        if !new_filters_match(e, &cb, 0, &args, types, &name, ev.as_deref_mut(), false) {
            continue;
        }
        if !on_main_thread(e) {
            log_kept(e, at, cb.script, &[arg0, arg1]);
            let texts = copy_texts(e, &[arg0, arg1], types);
            state(e).classic.push(Classic {
                info: at,
                serial: key.1,
                arg0,
                arg1,
                types: types.to_vec(),
                cleanup,
                texts,
            });
        } else {
            invoke_classic(e, at, cb.script, types, arg0, arg1);
            if let Some(f) = cleanup {
                f(e);
            }
        }
    }
}

/// `HandleEvent(eEventID, ...)`: an internal event by id, with the event's own argument types.
pub fn handle_event(
    e: &mut Engine,
    at: usize,
    arg0: u32,
    arg1: u32,
    cleanup: Option<fn(&mut Engine)>,
) {
    // Nothing registered (an engine without the events installed) or no handler.
    if !state(e)
        .infos
        .get(at)
        .is_some_and(|i| !i.callbacks.is_empty())
    {
        return;
    }
    let types = info(e, at).arg_types();
    // `GetClassicArgTypesAsStackVector` throws for more than two; no internal caller does that.
    if types.len() > 2 {
        return;
    }
    handle_classic(e, at, arg0, arg1, &types, None, cleanup);
}

/// An `NVSEMessagingInterface` message to the events (`HandleNVSEMessage`): its event with the
/// message's data as the one argument.
pub fn message(e: &mut Engine, at: usize, data: u32) {
    handle_event(e, at, data, 0, None);
}

/// [`message`] with a host string as the data, copied to the guest stack for the dispatch.
pub fn message_text(e: &mut Engine, at: usize, text: &[u8]) {
    if !state(e)
        .infos
        .get(at)
        .is_some_and(|i| !i.callbacks.is_empty())
    {
        return;
    }
    with_guest_text(e, text, |e, p| message(e, at, p));
}

/// `GetEffectiveArgs`: pointer parameters read through.
fn effective_args(e: &Engine, has_ptr_arg: bool, params: &[u8], args: &[u32]) -> Vec<u32> {
    if !has_ptr_arg {
        return args.to_vec();
    }
    params
        .iter()
        .enumerate()
        .map(|(i, &t)| {
            let a = args.get(i).copied().unwrap_or(0);
            if is_ptr(t) {
                e.mem.u32(a)
            } else {
                a
            }
        })
        .collect()
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), EventManager.h: DispatchEventRaw (for
// script handlers: no result callback).
/// The newer dispatch: every handler whose filters match, called with the arguments (`alt`:
/// numbers as `float` bits, as a script passes them).
#[allow(clippy::too_many_arguments)]
pub fn dispatch_raw(
    e: &mut Engine,
    at: usize,
    this_obj: u32,
    args: &[u32],
    types: &[u8],
    defer: bool,
    alt: bool,
    mut ev: Option<&mut Eval>,
) -> Dispatch {
    if defer && !on_main_thread(e) {
        log_kept(e, at, 0, args);
        let d = Deferred {
            info: at,
            this_obj,
            args: args.to_vec(),
            types: types.to_vec(),
            texts: copy_texts(e, args, types),
        };
        let s = state(e);
        if alt {
            s.deferred_alt.push(d);
        } else {
            s.deferred.push(d);
        }
        return Dispatch::Deferred;
    }
    let name = info(e, at).name.clone();
    let unknown = info(e, at).unknown_arg_types();
    let (has_ptr_arg, params) = (info(e, at).has_ptr_arg, info(e, at).params.clone());
    push_event(e, at);
    let mut cursor = None;
    while let Some(key) = next_after(e, at, cursor) {
        cursor = Some(key);
        let Some(cb) = entry(e, at, key.1).map(|x| x.callback.clone()) else {
            continue;
        };
        if cb.removed {
            continue;
        }
        // `GetEffectiveArgs`, per handler as in xNVSE: pointer parameters read through.
        let args = effective_args(e, has_ptr_arg, &params, args);
        let deprecated_types = if unknown { Some(types) } else { None };
        if !deprecated_filters_match(e, &cb, &args, deprecated_types, &name, ev.as_deref_mut()) {
            continue;
        }
        if !new_filters_match(
            e,
            &cb,
            this_obj,
            &args,
            types,
            &name,
            ev.as_deref_mut(),
            alt,
        ) {
            continue;
        }
        log_call(e, at, cb.script, &args);
        functions::call(
            e,
            Caller::Internal {
                script: cb.script,
                this_obj,
                containing: 0,
                args,
                alt,
                allow_surplus: true,
                event_types: None,
            },
        );
    }
    pop_event(e);
    Dispatch::Normal
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), EventManager.cpp: DispatchEvent,
// vDispatchEvent.
/// `DispatchEvent(name, thisObj, ...)` for a built-in event: by name, its own argument types,
/// not deferred; true unless the event is unknown.
pub fn dispatch(e: &mut Engine, name: &[u8], this_obj: u32, args: &[u32]) -> bool {
    dispatch_named(e, name, this_obj, args, false)
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), EventManager.cpp: DispatchEventThreadSafe.
/// `DispatchEventThreadSafe(name, nullptr, thisObj, ...)`: as [`dispatch`], deferred to the main
/// thread's tick when raised on another thread.
pub fn dispatch_thread_safe(e: &mut Engine, name: &[u8], this_obj: u32, args: &[u32]) -> bool {
    dispatch_named(e, name, this_obj, args, true)
}

fn dispatch_named(e: &mut Engine, name: &[u8], this_obj: u32, args: &[u32], defer: bool) -> bool {
    let Some(at) = find(e, name) else {
        return false;
    };
    let n = info(e, at).params.len();
    let mut words = args.to_vec();
    words.resize(n, 0);
    let types = info(e, at).arg_types();
    dispatch_raw(e, at, this_obj, &words, &types, defer, false, None) > Dispatch::GenericError
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), EventManager.cpp:
// EventInfo::ValidateDispatchedArgTypes.
pub fn validate_dispatched_types(
    e: &mut Engine,
    at: usize,
    types: &[u8],
    mut ev: Option<&mut Eval>,
) -> bool {
    let info = info(e, at).clone();
    if info.unknown_arg_types() {
        return true;
    }
    let event = String::from_utf8_lossy(&info.name).into_owned();
    if types.len() != info.params.len() {
        let text = format!("While validating (pseudo?)-dispatched args for event {event}, # of expected args ({}) != # of args passed ({})", info.params.len(), types.len());
        script_error(e, 0, ev, text);
        return false;
    }
    for (i, &got) in types.iter().enumerate() {
        let expected = non_ptr(info.params[i]);
        let ok = match expected {
            REFERENCE | ANY_FORM | BASE_FORM => got == ANY_FORM,
            ARRAY => got == ARRAY,
            INT | FLOAT => got == FLOAT,
            STRING => got == STRING,
            _ => {
                let text = format!("While validating (pseudo?)-dispatched args for event {event}, encountered invalid ArgType.");
                script_error(e, 0, ev, text);
                return false;
            }
        };
        if !ok {
            let text = format!(
                "While validating (pseudo?)-dispatched args for event {event}, encountered invalid ArgType #{got} (expected {}).",
                functions::data_type_name(param_to_data(expected))
            );
            script_error(e, 0, ev.as_deref_mut(), text);
            return false;
        }
    }
    true
}

// ---- the user-defined events (`DispatchEvent`) ----

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), EventManager.cpp:
// DispatchUserDefinedEvent.
pub fn dispatch_user_defined(
    e: &mut Engine,
    name: &[u8],
    sender: u32,
    args_array: u32,
    sender_name: Option<&[u8]>,
    ev: Option<&mut Eval>,
) -> bool {
    let Some(at) = find(e, name) else {
        return true;
    };
    if !info(e, at).user_defined() {
        return false;
    }
    let arr = if args_array != 0 {
        match vars::array(e, args_array) {
            Some(a) if a.key_type == vars::KEY_STRING => args_array,
            _ => return false,
        }
    } else {
        let modi = vars::script_mod_index(e, sender);
        vars::create_array(e, vars::KEY_STRING, false, modi)
    };
    vars::set_element(
        e,
        arr,
        &Key::Str(b"eventName".to_vec()),
        Value::Str(name.to_vec()),
    );
    let sender_name = match sender_name {
        Some(s) => s.to_vec(),
        None if sender != 0 => {
            let modi = vars::script_mod_index(e, sender);
            crate::errors::nth_mod_name(e, u32::from(modi))
        }
        None => b"NVSE".to_vec(),
    };
    vars::set_element(
        e,
        arr,
        &Key::Str(b"eventSender".to_vec()),
        Value::Str(sender_name),
    );
    handle_classic(e, at, arr, 0, &[ARRAY], ev, None);
    true
}

// ---- the tick and the game events ----

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), EventManager.cpp: Tick.
/// Each frame, before the temporaries are cleaned: the deferred calls (newest first), the
/// removed handlers erased, the duplicate guards cleared.
pub fn tick(e: &mut Engine) {
    while let Some(c) = state(e).classic.pop() {
        let script = match entry(e, c.info, c.serial) {
            Some(x) if !x.callback.removed => x.callback.script,
            _ => continue,
        };
        with_texts(e, &[c.arg0, c.arg1], &c.texts, |e, w| {
            invoke_classic(e, c.info, script, &c.types, w[0], w[1]);
        });
        if let Some(f) = c.cleanup {
            f(e);
        }
    }
    while let Some(d) = state(e).deferred.pop() {
        with_texts(e, &d.args, &d.texts, |e, w| {
            dispatch_raw(e, d.info, d.this_obj, &w, &d.types, false, false, None);
        });
    }
    while let Some(d) = state(e).deferred_alt.pop() {
        with_texts(e, &d.args, &d.texts, |e, w| {
            dispatch_raw(e, d.info, d.this_obj, &w, &d.types, false, true, None);
        });
    }
    clear_removals(e);
    let s = state(e);
    s.last = (0, 0, 0);
    s.last_hit_with = (0, 0);
    s.last_hit = (0, 0);
}

/// `EventIDForMask`.
fn event_for_mask(mask: u32) -> Option<usize> {
    if mask == 0 {
        return None;
    }
    DEFS[..=id::ON_ACTIVATE].iter().position(|d| d.mask == mask)
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), EventManager.cpp: HandleGameEvent.
/// A game event on `source` (with `object`), once: the game marks some twice in a row, and
/// `OnHitWith` and `OnHit` several times per hit.
pub fn handle_game_event(e: &mut Engine, mask: u32, source: u32, object: u32) {
    if IN_ADD_WORN_ITEM.with(|f| f.get()) && mask == 2 {
        return;
    }
    if !is_valid_reference(e, source) {
        return;
    }
    let s = state(e);
    if (source, object, mask) == s.last {
        return;
    }
    s.last = (source, object, mask);
    let Some(at) = event_for_mask(mask) else {
        return;
    };
    if at == id::ON_HIT_WITH {
        let s = state(e);
        if (source, object) != s.last_hit_with {
            s.last_hit_with = (source, object);
            handle_event(e, at, source, object, None);
        }
    } else if at == id::ON_HIT {
        let s = state(e);
        if (source, object) != s.last_hit {
            s.last_hit = (source, object);
            handle_event(e, at, source, object, None);
        }
    } else {
        handle_event(e, at, source, object, None);
    }
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), EventManager.cpp: MarkEventHook.
/// The wrap over `Script::SetActionFlag(form, extraList, mask)`: with the main hook installed
/// and a handler for the mask, the event on the list's reference (`extraList - 0x44`), then the
/// game's own marking.
fn set_action_flag(e: &mut Engine, a: &[u32]) -> Ret {
    let (target, extra, mask) = (a[0], a[1], a[2]);
    // Lock-free first: no handler for the mask (no extender script at all, by default), no lock.
    let marked = marks_lock_free(e);
    if extra != 0 && marked & mask != 0 && main_hook_installed(e) && state(e).in_use & mask != 0 {
        handle_game_event(e, mask, extra.wrapping_sub(0x44), target);
    }
    e.call_next(SET_ACTION_FLAG.addr(), a)
}

fn quit_console(e: &mut Engine, _: &[u32]) -> Option<u32> {
    message(e, id::QQQ, 0);
    None
}

fn quit_to_main_menu(e: &mut Engine, _: &[u32]) -> Option<u32> {
    message(e, id::EXIT_TO_MAIN_MENU, 0);
    None
}

fn quit_from_menu(e: &mut Engine, _: &[u32]) -> Option<u32> {
    message(e, id::EXIT_GAME, 0);
    None
}

/// Registers the events and installs the mark wrap and the exit seams.
pub fn install(e: &mut Engine) {
    init(e);
    // Made before the threads start, so every thread's engine shares it.
    e.ext_free::<LockFree>();
    if let Some(path) = std::env::var_os("NV_NVSE_EVENT_LOG") {
        state(e).log = std::fs::File::create(path).ok();
    }
    e.wrap(SET_ACTION_FLAG, set_action_flag);
    e.on_site(QUIT_CONSOLE, quit_console);
    e.on_site(QUIT_TO_MAIN_MENU, quit_to_main_menu);
    e.on_site(QUIT_FROM_MENU, quit_from_menu);
}

#[cfg(test)]
#[path = "events_tests.rs"]
mod tests;

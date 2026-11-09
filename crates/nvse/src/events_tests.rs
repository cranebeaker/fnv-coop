//! Tests of the event manager and its commands (I-9), with the game's functions they call
//! stood in by small test functions; each against the xNVSE 6.4.9 source it translates.

use super::*;
use crate::commands::test_support::*;
use crate::functions::{CREATE_EVENT_LIST, EVENT_LIST_DESTROY, SCRIPT_EXECUTE};
use std::cell::RefCell;

const AREA: u32 = 0x0400_0000;
/// Three function scripts (data `+0x100` after each).
const FN_A: u32 = AREA;
const FN_B: u32 = AREA + 0x200;
const FN_C: u32 = AREA + 0x400;
/// Two references of one base form, and their vtables' `GetIsReference`.
const REF_1: u32 = AREA + 0x6000;
const REF_2: u32 = AREA + 0x6100;
const BASE_1: u32 = AREA + 0x6200;
const BASE_VTABLE: u32 = AREA + 0x6800;
const REFR_VTABLE: u32 = 0x0108_aa3c;
const YES: u32 = 0x7eff_e200;
const NO: u32 = 0x7eff_e204;
/// A bump allocator for event lists.
const HEAP: u32 = AREA + 0x8000;
const HEAP_TOP: u32 = AREA + 0x7ffc;
/// The fake `Script::SetActionFlag`'s call count.
const MARKS: u32 = AREA + 0x7ff8;

/// What a handler saw when it ran: its script, its two parameters (as doubles and as their low
/// words) and the current event's name.
#[derive(Clone, Debug, PartialEq)]
struct Run {
    script: u32,
    a: f64,
    b: f64,
    a_word: u32,
    b_word: u32,
    event: Vec<u8>,
}

thread_local! {
    static LOG: RefCell<Vec<Run>> = const { RefCell::new(Vec::new()) };
}

fn log() -> Vec<Run> {
    LOG.with(|l| l.borrow_mut().drain(..).collect())
}

fn scripts_run() -> Vec<u32> {
    log().into_iter().map(|r| r.script).collect()
}

fn alloc(e: &mut Engine, size: u32) -> u32 {
    let at = e.mem.u32(HEAP_TOP).max(HEAP);
    e.mem.set_u32(HEAP_TOP, at + size.next_multiple_of(8));
    for i in 0..size {
        e.mem.set_u8(at + i, 0);
    }
    at
}

/// An event list with locals 1 and 2.
fn create_event_list(e: &mut Engine, a: &[u32]) -> Ret {
    let list = alloc(e, 0x14);
    e.mem.set_u32(list, a[0]);
    let mut prev = 0;
    for id in [2u32, 1] {
        let local = alloc(e, 0x10);
        e.mem.set_u32(local, id);
        let node = alloc(e, 8);
        e.mem.set_u32(node, local);
        e.mem.set_u32(node + 4, prev);
        prev = node;
    }
    e.mem.set_u32(list + 0xc, prev);
    Ret {
        eax: list,
        ..Ret::default()
    }
}

/// `Script::Execute`: notes the run.
fn execute(e: &mut Engine, a: &[u32]) -> Ret {
    let (script, list) = (a[0], a[2]);
    let l1 = crate::args::event_list_variable(e, list, 1);
    let l2 = crate::args::event_list_variable(e, list, 2);
    let run = Run {
        script,
        a: e.mem.f64(l1 + 8),
        b: e.mem.f64(l2 + 8),
        a_word: e.mem.u32(l1 + 8),
        b_word: e.mem.u32(l2 + 8),
        event: current_event_name(e),
    };
    LOG.with(|l| l.borrow_mut().push(run));
    Ret::default()
}

fn nothing(_: &mut Engine, _: &[u32]) -> Ret {
    Ret::default()
}

fn yes(_: &mut Engine, _: &[u32]) -> Ret {
    Ret {
        eax: 1,
        ..Ret::default()
    }
}

fn upper(_: &mut Engine, a: &[u32]) -> Ret {
    Ret {
        eax: u32::from((a[0] as u8).to_ascii_uppercase()),
        ..Ret::default()
    }
}

fn lower_fn(_: &mut Engine, a: &[u32]) -> Ret {
    Ret {
        eax: u32::from((a[0] as u8).to_ascii_lowercase()),
        ..Ret::default()
    }
}

/// `LookupFormByID` over the test's forms.
fn lookup(e: &mut Engine, a: &[u32]) -> Ret {
    let eax = [FN_A, FN_B, FN_C, REF_1, REF_2, BASE_1]
        .into_iter()
        .find(|&f| a[0] != 0 && e.mem.u32(f + 0xc) == a[0])
        .unwrap_or(0);
    Ret {
        eax,
        ..Ret::default()
    }
}

/// The game's own `SetActionFlag`: counts the marks.
fn set_action_flag_base(e: &mut Engine, _: &[u32]) -> Ret {
    let n = e.mem.u32(MARKS);
    e.mem.set_u32(MARKS, n + 1);
    Ret::default()
}

/// A function script `at` with id `id` taking `params` (`(local, type)`: 0 float, 1 int,
/// 2 string, 3 array, 4 ref).
fn function_script(e: &mut Engine, at: u32, id: u32, params: &[(u16, u8)]) {
    let data = at + 0x100;
    let mut d = vec![0x1d, 0, 0, 0, 0x10, 0];
    let mut block = vec![0x0d, 0, 0, 0, 0, 0, 1, params.len() as u8];
    for (idx, ty) in params {
        block.extend_from_slice(&idx.to_le_bytes());
        block.push(*ty);
    }
    d.extend_from_slice(&(block.len() as u16).to_le_bytes());
    d.extend(block);
    d.resize(0x40, 0);
    e.mem.write(data, &d);
    e.mem.set_u32(at + 0xc, id);
    e.mem.set_u8(at + 4, 0x11);
    e.mem.set_u32(at + 0x30, data);
    e.mem.set_u32(at + 0x20, d.len() as u32);
}

/// The three scripts with `params`.
fn scripts(e: &mut Engine, params: &[(u16, u8)]) {
    function_script(e, FN_A, 0x0500_0a00, params);
    function_script(e, FN_B, 0x0500_0b00, params);
    function_script(e, FN_C, 0x0500_0c00, params);
}

fn setup() -> Engine {
    LOG.with(|l| l.borrow_mut().clear());
    let mut e = engine_with_table();
    e.map(AREA, 0x1_0000);
    for page in [0x011c_3f2c, REFR_VTABLE, 0x011f_6394, 0x011d_aac0] {
        e.map(page & !0xfff, 0x1000);
    }
    e.register(0x00ec_a7f4, upper);
    e.register(GAME_TOLOWER, lower_fn);
    e.register(LOOKUP_FORM, lookup);
    e.register(SCRIPT_EXECUTE, execute);
    e.register(CREATE_EVENT_LIST, create_event_list);
    e.register(EVENT_LIST_DESTROY, nothing);
    e.register(SET_ACTION_FLAG.addr(), set_action_flag_base);
    e.register(YES, yes);
    e.register(NO, nothing);
    e.mem.set_u32(REFR_VTABLE + IS_REFERENCE, YES);
    e.mem.set_u32(BASE_VTABLE + IS_REFERENCE, NO);
    for (r, id) in [(REF_1, 0x0001_0001u32), (REF_2, 0x0001_0002)] {
        e.mem.set_u32(r, REFR_VTABLE);
        e.mem.set_u8(r + 4, 0x3b);
        e.mem.set_u32(r + 0xc, id);
        e.mem.set_u32(r + 0x20, BASE_1);
    }
    e.mem.set_u32(BASE_1, BASE_VTABLE);
    e.mem.set_u8(BASE_1 + 4, 0x1f);
    e.mem.set_u32(BASE_1 + 0xc, 0x0001_0100);
    e.mem.set_u32(SCRIPT + 0xc, 0x0500_0d00);
    crate::functions::install(&mut e);
    crate::tick::install(&mut e);
    crate::saveload::install(&mut e);
    install(&mut e);
    // After the first frame: this thread is the main thread.
    let thread = engine_core::thread_host::current_id(&e);
    e.ext::<Nvse>().tick.main_thread = thread;
    e
}

/// The text of a string local's value.
fn text_of(e: &Engine, id: f64) -> Vec<u8> {
    vars::string(e, id as u32).unwrap_or_default()
}

/// xNVSE's registry: 60 events in its order, found by name in any case, the four aliases, the
/// second `postloadgame` refused.
#[test]
fn the_registry_is_xnvses() {
    let mut e = setup();
    assert_eq!(count(&mut e), 60);
    assert_eq!(find(&mut e, b"OnAdd"), Some(0));
    assert_eq!(find(&mut e, b"onequip"), Some(1));
    assert_eq!(find(&mut e, b"OnUnequip"), Some(3));
    assert_eq!(find(&mut e, b"ON0X0080000"), Some(17));
    assert_eq!(find(&mut e, b"on0x00400000"), Some(20));
    assert_eq!(find(&mut e, b"OnActivate"), Some(id::ON_ACTIVATE));
    assert_eq!(find(&mut e, b"PostLoadGame"), Some(id::POST_LOAD_GAME));
    assert_eq!(find(&mut e, b"preloadgame"), Some(id::PRE_LOAD_GAME));
    assert_eq!(find(&mut e, b"nvsetestevent"), Some(id::DEBUG));
    assert_eq!(state_info(&mut e, 59).name, b"oncellrefsloaded");
    assert_eq!(find(&mut e, b"NoSuchEvent"), None);
    assert_eq!(hash_name(b"OnHit"), hash_name(b"onhit"));
}

/// Handlers run by priority, highest first, and in the order set within one; a second equal
/// handler at the same priority is refused, as are priorities 0 and out of range. The handler
/// gets the message's string and `GetCurrentEventName` answers the event.
#[test]
fn handlers_run_by_priority_then_as_set() {
    let mut e = setup();
    scripts(&mut e, &[(1, 2)]);
    assert!(set_handler(
        &mut e,
        b"LoadGame",
        Callback::new(FN_A),
        1,
        None
    ));
    assert!(set_handler(
        &mut e,
        b"LoadGame",
        Callback::new(FN_B),
        5,
        None
    ));
    assert!(set_handler(
        &mut e,
        b"LoadGame",
        Callback::new(FN_C),
        1,
        None
    ));
    assert!(!set_handler(
        &mut e,
        b"loadgame",
        Callback::new(FN_A),
        1,
        None
    ));
    assert!(!set_handler(
        &mut e,
        b"LoadGame",
        Callback::new(FN_A),
        0,
        None
    ));
    assert!(!set_handler(
        &mut e,
        b"LoadGame",
        Callback::new(FN_A),
        10_000,
        None
    ));
    message_text(&mut e, id::LOAD_GAME, b"Save 1.fos");
    let runs = log();
    assert_eq!(
        runs.iter().map(|r| r.script).collect::<Vec<_>>(),
        [FN_B, FN_A, FN_C]
    );
    for r in &runs {
        assert_eq!(r.event, b"loadgame");
        assert_eq!(text_of(&e, r.a), b"Save 1.fos");
    }
    assert_eq!(current_event_name(&mut e), b"");
}

/// A removed handler stops running at once and is erased at the next tick; set again before
/// then, it comes back in its old place.
#[test]
fn removal_is_deferred_to_the_tick() {
    let mut e = setup();
    scripts(&mut e, &[(1, 2)]);
    set_handler(&mut e, b"LoadGame", Callback::new(FN_A), 1, None);
    set_handler(&mut e, b"LoadGame", Callback::new(FN_B), 1, None);
    let remove = Callback::new(FN_A);
    assert!(remove_handler(
        &mut e,
        b"LoadGame",
        &remove,
        PRIORITY_INVALID,
        None
    ));
    message_text(&mut e, id::LOAD_GAME, b"x");
    assert_eq!(scripts_run(), [FN_B]);
    assert_eq!(state_info(&mut e, id::LOAD_GAME).callbacks.len(), 2);
    assert!(set_handler(
        &mut e,
        b"LoadGame",
        Callback::new(FN_A),
        1,
        None
    ));
    message_text(&mut e, id::LOAD_GAME, b"x");
    assert_eq!(scripts_run(), [FN_A, FN_B]);
    remove_handler(&mut e, b"LoadGame", &remove, 1, None);
    tick(&mut e);
    assert_eq!(state_info(&mut e, id::LOAD_GAME).callbacks.len(), 1);
    // Removing at another priority finds nothing.
    assert!(!remove_handler(
        &mut e,
        b"LoadGame",
        &Callback::new(FN_B),
        7,
        None
    ));
}

/// The deprecated caller checks the handler against the event: as many parameters, each of a
/// type the argument goes to (`PostLoadGame` passes an int); a failed load (0) passes nothing.
#[test]
fn classic_handlers_must_fit_the_event() {
    let mut e = setup();
    function_script(&mut e, FN_A, 0x0500_0a00, &[(1, 1)]);
    function_script(&mut e, FN_B, 0x0500_0b00, &[(1, 0)]);
    function_script(&mut e, FN_C, 0x0500_0c00, &[(1, 1), (2, 1)]);
    for f in [FN_A, FN_B, FN_C] {
        set_handler(&mut e, b"PostLoadGame", Callback::new(f), 1, None);
    }
    message(&mut e, id::POST_LOAD_GAME, 1);
    let runs = log();
    assert_eq!(
        runs.len(),
        1,
        "the float and two-parameter handlers do not run"
    );
    assert_eq!((runs[0].script, runs[0].a), (FN_A, 1.0));
    message(&mut e, id::POST_LOAD_GAME, 0);
    let runs = log();
    assert_eq!((runs[0].script, runs[0].a), (FN_A, 0.0));
}

/// What the `005ac750` wrap reads without the lock.
fn mark_masks(e: &mut Engine) -> u32 {
    marks_lock_free(e)
}

/// The game's marks through the wrap at `005ac750`: only once a game event's handler installed
/// the hook and only for masks in use; the reference is the list's (`- 0x44`); a mark repeated
/// at once counts once until the tick; the game's own function runs every time.
#[test]
fn game_events_come_through_set_action_flag() {
    let mut e = setup();
    scripts(&mut e, &[(1, 4), (2, 4)]);
    let mark = |e: &mut Engine, target: u32, refr: u32, mask: u32| {
        e.call(SET_ACTION_FLAG.addr(), &args![target, refr + 0x44, mask]);
    };
    mark(&mut e, REF_2, REF_1, 0x80);
    assert!(log().is_empty());
    assert_eq!(mark_masks(&mut e), 0, "nothing for the wrap to pass on");
    assert!(set_handler(&mut e, b"OnHit", Callback::new(FN_A), 1, None));
    assert!(main_hook_installed(&mut e));
    assert_eq!(state(&mut e).in_use, 0x80);
    assert_eq!(mark_masks(&mut e), 0x80);
    mark(&mut e, REF_2, REF_1, 0x80);
    mark(&mut e, REF_2, REF_1, 0x80);
    let runs = log();
    assert_eq!(runs.len(), 1);
    assert_eq!((runs[0].a_word, runs[0].b_word), (0x0001_0001, 0x0001_0002));
    assert_eq!(runs[0].event, b"onhit");
    // OnAdd has no handler: its mask is not in use.
    mark(&mut e, REF_2, REF_1, 0x1);
    assert!(log().is_empty());
    tick(&mut e);
    mark(&mut e, REF_2, REF_1, 0x80);
    assert_eq!(log().len(), 1);
    // Not a reference: no event.
    mark(&mut e, REF_2, BASE_1, 0x80);
    assert!(log().is_empty());
    assert_eq!(e.mem.u32(MARKS), 6);
}

/// `SetEventHandler`'s `"first"` filter: the reference itself or its base form; `"second"`: the
/// other argument itself.
#[test]
fn old_filters_take_the_reference_or_its_base() {
    let mut e = setup();
    scripts(&mut e, &[(1, 4), (2, 4)]);
    let mut by_ref = Callback::new(FN_A);
    by_ref.source = REF_1;
    let mut by_base = Callback::new(FN_B);
    by_base.source = BASE_1;
    let mut by_object = Callback::new(FN_C);
    by_object.object = REF_1;
    for cb in [by_ref, by_base, by_object] {
        assert!(set_handler(&mut e, b"OnHit", cb, 1, None));
    }
    handle_game_event(&mut e, 0x80, REF_1, REF_2);
    assert_eq!(scripts_run(), [FN_A, FN_B]);
    handle_game_event(&mut e, 0x80, REF_2, REF_1);
    assert_eq!(scripts_run(), [FN_B, FN_C]);
}

/// `SetEventHandlerAlt`'s numbered filters: a string compared without case, an array of values
/// matching any; a removal without filters covers the filtered handlers, one with another value
/// does not.
#[test]
fn new_filters_and_what_a_removal_covers() {
    let mut e = setup();
    scripts(&mut e, &[(1, 2)]);
    let mut one = Callback::new(FN_A);
    one.filters.insert(1, Data::Str(b"SAVE 1.FOS".to_vec()));
    let any = vars::create_array(&mut e, vars::KEY_NUMBER, true, 5);
    vars::set_element(
        &mut e,
        any,
        &Key::Number(0.0),
        Value::Str(b"a.fos".to_vec()),
    );
    vars::set_element(
        &mut e,
        any,
        &Key::Number(1.0),
        Value::Str(b"b.fos".to_vec()),
    );
    let mut list = Callback::new(FN_B);
    let mut held = Data::Array(any);
    hold_data(&mut e, &mut held);
    list.filters.insert(1, held);
    assert!(set_handler(&mut e, b"LoadGame", one, 1, None));
    assert!(set_handler(&mut e, b"LoadGame", list, 1, None));
    message_text(&mut e, id::LOAD_GAME, b"save 1.fos");
    assert_eq!(scripts_run(), [FN_A]);
    message_text(&mut e, id::LOAD_GAME, b"B.FOS");
    assert_eq!(scripts_run(), [FN_B]);
    let mut other = Callback::new(FN_A);
    other.filters.insert(1, Data::Str(b"other.fos".to_vec()));
    assert!(!remove_handler(&mut e, b"LoadGame", &other, 0, None));
    assert!(remove_handler(
        &mut e,
        b"LoadGame",
        &Callback::new(FN_B),
        0,
        None
    ));
    tick(&mut e);
    assert_eq!(state_info(&mut e, id::LOAD_GAME).callbacks.len(), 1);
    // The erased handler gave up its array; nothing else refers to it.
    vars::clean_arrays(&mut e);
    assert!(!vars::array_exists(&e, any));
}

/// Off the main thread (here: before the first frame recorded it) each matching handler is
/// kept for the tick, which runs the kept calls newest first.
#[test]
fn off_the_main_thread_calls_wait_for_the_tick() {
    let mut e = setup();
    scripts(&mut e, &[(1, 2)]);
    set_handler(&mut e, b"SaveGame", Callback::new(FN_A), 1, None);
    set_handler(&mut e, b"SaveGame", Callback::new(FN_B), 1, None);
    e.ext::<Nvse>().tick.main_thread = 0;
    let name = crate::table::guest_str(&mut e, b"Save 2.fos");
    message(&mut e, id::SAVE_GAME, name);
    assert!(log().is_empty());
    tick(&mut e);
    assert_eq!(scripts_run(), [FN_B, FN_A]);
}

/// A load or a new game takes the handlers `SetEventHandlerAlt` set and keeps the others; a
/// game event's mask goes with its last handler.
#[test]
fn a_load_flushes_the_alt_handlers() {
    let mut e = setup();
    scripts(&mut e, &[(1, 4), (2, 4)]);
    let mut alt = Callback::new(FN_A);
    alt.flush_on_load = true;
    set_handler(&mut e, b"OnHit", alt, 1, None);
    set_handler(&mut e, b"OnLoad", Callback::new(FN_B), 1, None);
    assert_eq!(state(&mut e).in_use, 0x80 | 0x1000);
    assert_eq!(mark_masks(&mut e), 0x80 | 0x1000);
    clear_flush_on_load(&mut e);
    assert!(state_info(&mut e, id::ON_HIT).callbacks.is_empty());
    assert_eq!(state(&mut e).in_use, 0x1000);
    assert_eq!(mark_masks(&mut e), 0x1000, "the lock-free copy follows");
    // The co-save's new game: the message, then the flush has nothing left to take.
    function_script(&mut e, FN_C, 0x0500_0c00, &[]);
    set_handler(&mut e, b"NewGame", Callback::new(FN_C), 1, None);
    crate::saveload::new_game(&mut e);
    assert_eq!(scripts_run(), [FN_C]);
    assert_eq!(state(&mut e).in_use, 0x1000);
}

/// The load's end (`0x00848C91`) and the exits send their messages.
#[test]
fn post_load_and_exit_seams_send_messages() {
    let mut e = setup();
    function_script(&mut e, FN_A, 0x0500_0a00, &[(1, 1)]);
    function_script(&mut e, FN_B, 0x0500_0b00, &[]);
    set_handler(&mut e, b"PostLoadGame", Callback::new(FN_A), 1, None);
    for name in [&b"QQQ"[..], b"ExitGame", b"ExitToMainMenu"] {
        set_handler(&mut e, name, Callback::new(FN_B), 1, None);
    }
    e.site(crate::saveload::POST_LOAD.addr(), &[1]);
    e.site(QUIT_CONSOLE.addr(), &[]);
    e.site(QUIT_FROM_MENU.addr(), &[]);
    e.site(QUIT_TO_MAIN_MENU.addr(), &[]);
    let runs = log();
    assert_eq!(
        runs.iter().map(|r| r.event.clone()).collect::<Vec<_>>(),
        [&b"postloadgame"[..], b"qqq", b"exitgame", b"exittomainmenu"]
    );
    assert_eq!(runs[0].a, 1.0);
}

// ---- through the commands ----

fn expression(tokens: &[Vec<u8>]) -> Vec<u8> {
    let body = tokens.concat();
    let mut b = (body.len() as u16 + 2).to_le_bytes().to_vec();
    b.extend(body);
    b
}
fn num(v: f64) -> Vec<u8> {
    let mut b = vec![b'Z'];
    b.extend_from_slice(&v.to_le_bytes());
    b
}
fn text(t: &str) -> Vec<u8> {
    let mut b = vec![b'S'];
    b.extend_from_slice(&(t.len() as u16).to_le_bytes());
    b.extend_from_slice(t.as_bytes());
    b
}
/// Reference `i` of the calling script.
fn form(i: u8) -> Vec<u8> {
    vec![b'R', i, 0]
}
/// `left::right` as one argument's tokens.
fn pair(left: Vec<u8>, right: Vec<u8>) -> Vec<u8> {
    [left, right, vec![crate::expr::op::MAKE_PAIR]].concat()
}

/// Runs an expression command with one expression per argument.
fn run(e: &mut Engine, opcode: u32, args: &[Vec<u8>]) -> f64 {
    let mut bytes = vec![args.len() as u8];
    for a in args {
        bytes.extend(expression(std::slice::from_ref(a)));
    }
    e.mem.set_u16(DATA, opcode as u16);
    e.mem.set_u16(DATA + 2, bytes.len() as u16);
    e.mem.write(DATA + 4, &bytes);
    e.mem.set_u32(OFFSET, 4);
    e.mem.set_f64(RESULT, 0.0);
    let record = crate::table::record_of(e, opcode);
    let (params, execute) = (e.mem.u32(record + 0x14), e.mem.u32(record + 0x18));
    e.call(execute, &[params, DATA, 0, 0, SCRIPT, LIST, RESULT, OFFSET]);
    e.mem.f64(RESULT)
}

const SET_EVENT_HANDLER: u32 = 0x15e9;
const REMOVE_EVENT_HANDLER: u32 = 0x15ea;
const DISPATCH_EVENT: u32 = 0x15ec;
const DISPATCH_EVENT_ALT: u32 = 0x1647;
const GET_EVENT_HANDLERS: u32 = 0x1649;
const SET_EVENT_HANDLER_ALT: u32 = 0x164b;
const IS_EVENT_HANDLER_FIRST: u32 = 0x1650;
const IS_EVENT_HANDLER_LAST: u32 = 0x1651;

/// The opcodes are xNVSE's (its command list in `names.rs`).
#[test]
fn the_commands_sit_at_xnvses_opcodes() {
    for c in crate::event_commands::COMMANDS {
        let i = (c.opcode - crate::names::FIRST_OPCODE) as usize;
        assert_eq!(crate::names::NAMES[i].0, c.name);
    }
}

/// The first lines of xNVSE's `event_handler_functions.txt`: `SetEventHandler "OnHit" fn` is 1,
/// `SetEventHandlerAlt` of the same is 0 (an equal handler), `GetEventHandlers "OnHit" fn 1`
/// gives `[fn, {}]`, `RemoveEventHandler` is 1 and leaves none; `"first"::ref` is listed, the
/// Alt form refuses string keys; a `"priority"` pair sets the priority.
#[test]
fn set_get_and_remove_through_the_commands() {
    let mut e = setup();
    scripts(&mut e, &[(1, 4), (2, 4)]);
    set_script_ref(&mut e, 1, FN_A);
    set_script_ref(&mut e, 2, REF_1);
    assert_eq!(
        run(&mut e, SET_EVENT_HANDLER, &[text("OnHit"), form(1)]),
        1.0
    );
    assert_eq!(
        run(&mut e, SET_EVENT_HANDLER_ALT, &[text("OnHit"), form(1)]),
        0.0
    );
    let got = run(
        &mut e,
        GET_EVENT_HANDLERS,
        &[text("OnHit"), form(1), num(1.0)],
    ) as u32;
    let first = vars::get_element(&mut e, got, &Key::Number(0.0)).unwrap();
    let Data::Array(handler) = first.data else {
        panic!("an array of handlers");
    };
    let script = vars::get_element(&mut e, handler, &Key::Number(0.0)).unwrap();
    assert_eq!(script.data, Data::Form(0x0500_0a00));
    let Data::Array(filters) = vars::get_element(&mut e, handler, &Key::Number(1.0))
        .unwrap()
        .data
    else {
        panic!("a filter map");
    };
    assert!(vars::array(&e, filters).unwrap().is_empty());
    assert_eq!(
        run(&mut e, REMOVE_EVENT_HANDLER, &[text("OnHit"), form(1)]),
        1.0
    );
    tick(&mut e);
    let got = run(
        &mut e,
        GET_EVENT_HANDLERS,
        &[text("OnHit"), form(1), num(1.0)],
    ) as u32;
    assert!(vars::array(&e, got).unwrap().is_empty());
    // "first"::ref.
    let first_ref = pair(text("first"), form(2));
    assert_eq!(
        run(
            &mut e,
            SET_EVENT_HANDLER,
            &[text("OnHit"), form(1), first_ref.clone()]
        ),
        1.0
    );
    assert_eq!(
        run(
            &mut e,
            SET_EVENT_HANDLER_ALT,
            &[text("OnHit"), form(1), first_ref]
        ),
        0.0
    );
    let cb = &state_info(&mut e, id::ON_HIT).callbacks[0];
    assert_eq!((cb.callback.source, cb.priority), (REF_1, 1));
    // "priority"::7 and a numbered filter with the Alt form.
    let r = run(
        &mut e,
        SET_EVENT_HANDLER_ALT,
        &[
            text("OnHit"),
            form(1),
            pair(text("priority"), num(7.0)),
            pair(num(1.0), form(2)),
        ],
    );
    assert_eq!(r, 1.0);
    let cb = state_info(&mut e, id::ON_HIT).callbacks[0].clone();
    assert_eq!(cb.priority, 7);
    assert_eq!(cb.callback.filters.get(&1), Some(&Data::Form(0x0001_0001)));
    assert!(cb.callback.flush_on_load);
}

/// `DispatchEvent`: a user-defined event set by name gets an array with the name and the
/// sender; `DispatchEventAlt` refuses an event that does not allow it and checks the test
/// event's seven arguments.
#[test]
fn user_defined_and_script_dispatched_events() {
    let mut e = setup();
    scripts(&mut e, &[(1, 3)]);
    set_script_ref(&mut e, 1, FN_A);
    assert_eq!(
        run(&mut e, SET_EVENT_HANDLER, &[text("MyEvent"), form(1)]),
        1.0
    );
    let at = find(&mut e, b"MYEVENT").unwrap();
    assert_eq!(state_info(&mut e, at).name, b"myevent");
    let empty = vars::create_array(&mut e, vars::KEY_STRING, false, 5);
    add_local(&mut e, 9, "aArgs", f64::from(empty));
    assert_eq!(
        run(
            &mut e,
            DISPATCH_EVENT,
            &[text("MyEvent"), vec![b'V', 3, 0, 0, 9, 0], text("Tester")]
        ),
        1.0
    );
    let runs = log();
    assert_eq!(runs.len(), 1);
    let arr = runs[0].a as u32;
    assert_eq!(arr, empty);
    let name = vars::get_element(&mut e, arr, &Key::Str(b"eventName".to_vec())).unwrap();
    assert_eq!(name.data, Data::Str(b"MyEvent".to_vec()));
    let sender = vars::get_element(&mut e, arr, &Key::Str(b"eventSender".to_vec())).unwrap();
    assert_eq!(sender.data, Data::Str(b"Tester".to_vec()));
    // An unknown event: true (it may have no handler yet); LoadGame: not dispatchable.
    assert_eq!(run(&mut e, DISPATCH_EVENT_ALT, &[text("NoSuchEvent")]), 1.0);
    assert_eq!(
        run(&mut e, DISPATCH_EVENT_ALT, &[text("LoadGame"), text("x")]),
        0.0
    );
    assert_eq!(
        run(
            &mut e,
            DISPATCH_EVENT_ALT,
            &[text("nvsetestevent"), num(1.0)]
        ),
        0.0
    );
}

/// `IsEventHandlerFirst`/`Last`: the function at its priority with nothing above (or below), not
/// ignored; priority 0 is refused.
#[test]
fn first_and_last_queries() {
    let mut e = setup();
    scripts(&mut e, &[(1, 2)]);
    set_script_ref(&mut e, 1, FN_A);
    set_script_ref(&mut e, 2, FN_B);
    set_handler(&mut e, b"LoadGame", Callback::new(FN_A), 9, None);
    set_handler(&mut e, b"LoadGame", Callback::new(FN_B), 1, None);
    let q =
        |e: &mut Engine, op: u32, f: u8, p: f64| run(e, op, &[text("LoadGame"), form(f), num(p)]);
    assert_eq!(q(&mut e, IS_EVENT_HANDLER_FIRST, 1, 9.0), 1.0);
    assert_eq!(q(&mut e, IS_EVENT_HANDLER_LAST, 1, 9.0), 0.0);
    assert_eq!(q(&mut e, IS_EVENT_HANDLER_LAST, 2, 1.0), 1.0);
    assert_eq!(q(&mut e, IS_EVENT_HANDLER_FIRST, 2, 1.0), 0.0);
    assert_eq!(q(&mut e, IS_EVENT_HANDLER_FIRST, 2, 0.0), 0.0);
    // Ignoring the other function's script.
    let r = run(
        &mut e,
        IS_EVENT_HANDLER_FIRST,
        &[text("LoadGame"), form(2), num(1.0), form(1)],
    );
    assert_eq!(r, 1.0);
}

const GET_HIGHER: u32 = 0x1652;
const GET_LOWER: u32 = 0x1653;

/// xNVSE's own `event_handler_priority_system.txt`, statement by statement (its `assert`s as
/// assertions), on the manager and the commands.
#[test]
fn xnvses_priority_unit_test() {
    let mut e = setup();
    scripts(&mut e, &[]);
    set_script_ref(&mut e, 1, FN_A);
    set_script_ref(&mut e, 2, FN_B);
    set_script_ref(&mut e, 3, FN_C);
    let ev = b"nvseTestEvent";
    let set = |e: &mut Engine, f: u32, p: i32| set_handler(e, ev, Callback::new(f), p, None);
    let alt = |e: &mut Engine, f: u32, p: i32| {
        let mut cb = Callback::new(f);
        cb.flush_on_load = true;
        set_handler(e, ev, cb, p, None)
    };
    let remove = |e: &mut Engine, f: u32, p: i32| remove_handler(e, ev, &Callback::new(f), p, None);
    let sizes = |e: &mut Engine, f: u8| -> Vec<usize> {
        let got = run(e, GET_EVENT_HANDLERS, &[text("nvseTestEvent"), form(f)]) as u32;
        let a = vars::array(e, got).unwrap().clone();
        let mut out = vec![a.len()];
        for k in a.keys() {
            if let Some(Data::Array(inner)) = vars::get_element(e, got, &k).map(|x| x.data) {
                out.push(vars::array(e, inner).unwrap().len());
            }
        }
        out
    };
    assert!(!set(&mut e, FN_A, 0));
    assert!(set(&mut e, FN_A, 1));
    assert!(!set(&mut e, FN_A, 1));
    assert!(!alt(&mut e, FN_A, 1));
    assert!(!alt(&mut e, FN_A, PRIORITY_DEFAULT));
    assert!(!set(&mut e, FN_A, PRIORITY_DEFAULT));
    assert!(set(&mut e, FN_B, -1));
    assert!(set(&mut e, FN_C, -2));
    // DispatchEventAlt "nvseTestEvent" 0, 0.5, ar_Null, "test", SunnyREF, Player, GSSunnySmiles:
    // as a script passes them (numbers as float bits, forms as AnyForm).
    let types = [FLOAT, FLOAT, ARRAY, STRING, ANY_FORM, ANY_FORM, ANY_FORM];
    let at = find(&mut e, ev).unwrap();
    assert!(validate_dispatched_types(&mut e, at, &types, None));
    let test_text = crate::table::guest_str(&mut e, b"test");
    let words = [0, 0.5f32.to_bits(), 0, test_text, REF_1, REF_2, BASE_1];
    assert_eq!(
        dispatch_raw(&mut e, at, 0, &words, &types, false, true, None),
        Dispatch::Normal
    );
    assert_eq!(scripts_run(), [FN_A, FN_B, FN_C]);
    for p in [-3, 4, 3, 3, 2, 1] {
        assert!(!remove(&mut e, FN_C, p));
    }
    assert!(remove(&mut e, FN_A, 1));
    assert!(remove(&mut e, FN_B, -1));
    assert!(remove(&mut e, FN_C, -2));
    assert_eq!(sizes(&mut e, 1), [0]);
    assert!(set(&mut e, FN_A, 1));
    assert!(set(&mut e, FN_A, -1));
    assert!(set(&mut e, FN_A, -2));
    assert_eq!(sizes(&mut e, 1), [3, 1, 1, 1]);
    assert!(remove(&mut e, FN_A, PRIORITY_INVALID));
    assert_eq!(sizes(&mut e, 1), [0]);
    // IsEventHandlerFirst/Last.
    assert!(set(&mut e, FN_A, 1));
    assert!(set(&mut e, FN_B, -1));
    assert!(set(&mut e, FN_C, -2));
    let q = |e: &mut Engine, op: u32, f: u8, p: f64| {
        run(e, op, &[text("nvseTestEvent"), form(f), num(p)])
    };
    for (op, f, p, want) in [
        (IS_EVENT_HANDLER_FIRST, 1, 1.0, 1.0),
        (IS_EVENT_HANDLER_LAST, 1, 1.0, 0.0),
        (IS_EVENT_HANDLER_LAST, 1, -1.0, 0.0),
        (IS_EVENT_HANDLER_LAST, 1, -2.0, 0.0),
        (IS_EVENT_HANDLER_FIRST, 1, -1.0, 0.0),
        (IS_EVENT_HANDLER_FIRST, 1, -2.0, 0.0),
        (IS_EVENT_HANDLER_LAST, 2, -1.0, 0.0),
        (IS_EVENT_HANDLER_FIRST, 2, -1.0, 0.0),
        (IS_EVENT_HANDLER_FIRST, 3, -2.0, 0.0),
        (IS_EVENT_HANDLER_LAST, 3, -2.0, 1.0),
    ] {
        assert_eq!(q(&mut e, op, f, p), want, "{op:04x} {f} {p}");
    }
    // GetHigher/LowerPriorityEventHandlers: by priority, `[n][0]` the other handler's script.
    let script_at = |e: &mut Engine, arr: u32, p: f64| -> u32 {
        let Some(Data::Array(h)) = vars::get_element(e, arr, &Key::Number(p)).map(|x| x.data)
        else {
            return 0;
        };
        match vars::get_element(e, h, &Key::Number(0.0)).map(|x| x.data) {
            Some(Data::Form(id)) => id,
            _ => 0,
        }
    };
    let len = |e: &mut Engine, arr: u32| vars::array(e, arr).map_or(usize::MAX, |a| a.len());
    let r = q(&mut e, GET_HIGHER, 1, 1.0) as u32;
    assert_eq!(len(&mut e, r), 0);
    assert_eq!(q(&mut e, GET_HIGHER, 1, -1.0), 0.0);
    assert_eq!(q(&mut e, GET_HIGHER, 1, -2.0), 0.0);
    let r = q(&mut e, GET_HIGHER, 2, -1.0) as u32;
    assert_eq!(len(&mut e, r), 1);
    assert_eq!(script_at(&mut e, r, 1.0), 0x0500_0a00);
    let r = q(&mut e, GET_HIGHER, 3, -2.0) as u32;
    assert_eq!(len(&mut e, r), 2);
    assert_eq!(script_at(&mut e, r, 1.0), 0x0500_0a00);
    assert_eq!(script_at(&mut e, r, -1.0), 0x0500_0b00);
    let r = q(&mut e, GET_LOWER, 1, 1.0) as u32;
    assert_eq!(len(&mut e, r), 2);
    assert_eq!(script_at(&mut e, r, -1.0), 0x0500_0b00);
    assert_eq!(script_at(&mut e, r, -2.0), 0x0500_0c00);
    let r = q(&mut e, GET_LOWER, 2, -1.0) as u32;
    assert_eq!(len(&mut e, r), 1);
    assert_eq!(script_at(&mut e, r, -2.0), 0x0500_0c00);
    let r = q(&mut e, GET_LOWER, 3, -2.0) as u32;
    assert_eq!(len(&mut e, r), 0);
    assert!(remove(&mut e, FN_A, 1));
    assert!(remove(&mut e, FN_B, -1));
    assert!(remove(&mut e, FN_C, -2));
    let all = run(&mut e, GET_EVENT_HANDLERS, &[text("nvseTestEvent")]) as u32;
    assert_eq!(len(&mut e, all), 0);
}

/// What the real game printed (`real-tests/gr_h_events.txt`, run `h3`): a handler of
/// `RuntimeScriptError` gets each error's message without the header, the evaluator's errors
/// with the file, offset and command line; a priority below the minimum is printed as an
/// unsigned number; a user-defined event's default sender is the sending script's file name
/// (none for a file beyond the loaded ones), its name as dispatched.
#[test]
fn errors_and_senders_as_the_real_game_shows_them() {
    let mut e = setup();
    e.ext::<Nvse>().main_thread_recorded = true;
    // The console (none) and the script's editor id (none) for the error header.
    e.register(0x0071_b160, nothing);
    e.mem.set_u32(SCRIPT, AREA + 0x6900);
    e.mem.set_u32(AREA + 0x6900 + 0x130, NO);
    function_script(&mut e, FN_A, 0x0500_0a00, &[(1, 2)]);
    function_script(&mut e, FN_B, 0x0500_0b00, &[(1, 3)]);
    set_script_ref(&mut e, 1, FN_B);
    set_handler(&mut e, b"RuntimeScriptError", Callback::new(FN_A), 1, None);
    let r = run(
        &mut e,
        SET_EVENT_HANDLER,
        &[
            text("evtest"),
            form(1),
            pair(text("priority"), num(-10000.0)),
        ],
    );
    assert_eq!(r, 0.0);
    let r = run(
        &mut e,
        SET_EVENT_HANDLER_ALT,
        &[text("evtest"), form(1), pair(text("first"), form(1))],
    );
    assert_eq!(r, 0.0);
    let got: Vec<String> = log()
        .iter()
        .map(|x| String::from_utf8_lossy(&text_of(&e, x.a)).into_owned())
        .collect();
    assert_eq!(got.len(), 2);
    assert!(
        got[0].starts_with("Can't set event handler with priority below 4294957297.\n    File: ")
    );
    assert!(got[0].ends_with("Command: SetEventHandler"), "{}", got[0]);
    assert!(got[1].starts_with(
        "String filter keys are not allowed for this function; use int codes.\n    File: "
    ));
    assert!(
        got[1].ends_with("Command: SetEventHandlerAlt"),
        "{}",
        got[1]
    );
    // DispatchEvent "EvTest": the name as given, no sender name for an unloaded file.
    assert_eq!(
        run(&mut e, SET_EVENT_HANDLER, &[text("evtest"), form(1)]),
        1.0
    );
    assert_eq!(run(&mut e, DISPATCH_EVENT, &[text("EvTest")]), 1.0);
    let runs = log();
    assert_eq!(runs.len(), 1);
    let arr = runs[0].a as u32;
    let name = vars::get_element(&mut e, arr, &Key::Str(b"eventName".to_vec())).unwrap();
    assert_eq!(name.data, Data::Str(b"EvTest".to_vec()));
    let sender = vars::get_element(&mut e, arr, &Key::Str(b"eventSender".to_vec())).unwrap();
    assert_eq!(sender.data, Data::Str(Vec::new()));
    assert_eq!(runs[0].event, b"evtest");
}

/// Review item 3 of #329: a call kept for the tick (off the main thread) keeps its string
/// argument's text; the string it pointed at (here on the guest stack) is gone by then.
#[test]
fn held_messages_keep_their_text() {
    let mut e = setup();
    scripts(&mut e, &[(1, 2)]);
    set_handler(&mut e, b"LoadGame", Callback::new(FN_A), 1, None);
    e.ext::<Nvse>().tick.main_thread = 0;
    message_text(&mut e, id::LOAD_GAME, b"Save 3.fos");
    // The stack the text was on is used again.
    e.with_stack(64, |e, at| e.mem.write(at.addr(), &[0x55; 64]));
    tick(&mut e);
    let runs = log();
    assert_eq!(runs.len(), 1);
    assert_eq!(text_of(&e, runs[0].a), b"Save 3.fos");
}

/// The handler's run counts, and from inside it an error while fewer than five ran.
fn erring_execute(e: &mut Engine, _: &[u32]) -> Ret {
    let n = e.mem.u32(AREA + 0x7ff0) + 1;
    e.mem.set_u32(AREA + 0x7ff0, n);
    if n < 5 {
        crate::errors::show(e, FN_A, "inner");
    }
    Ret::default()
}

/// Review item 2 of #329 (the fixes preset's row): an error inside a `RuntimeScriptError`
/// handler is not dispatched to it again; without the row it is (xNVSE recurses until the
/// stack runs out; here the stand-in stops after five).
#[test]
fn the_error_guard_stops_a_handler_loop() {
    for (guard, runs) in [(true, 1), (false, 5)] {
        let mut e = setup();
        e.register(SCRIPT_EXECUTE, erring_execute);
        scripts(&mut e, &[(1, 2)]);
        set_error_guard(&mut e, guard);
        set_handler(&mut e, b"RuntimeScriptError", Callback::new(FN_A), 1, None);
        crate::errors::show(&mut e, FN_B, "outer");
        assert_eq!(e.mem.u32(AREA + 0x7ff0), runs, "guard {guard}");
    }
}

/// Review item 6 of #329: the guard is per game thread. An error raised on one thread while
/// another thread's `RuntimeScriptError` handlers run is that thread's own and reaches the
/// handlers; the flag is gone after the dispatch.
#[test]
fn the_error_guard_is_per_thread() {
    let mut e = setup();
    e.register(SCRIPT_EXECUTE, erring_execute);
    scripts(&mut e, &[(1, 2)]);
    set_error_guard(&mut e, true);
    set_handler(&mut e, b"RuntimeScriptError", Callback::new(FN_A), 1, None);
    let other = engine_core::thread_host::current_id(&e).wrapping_add(1);
    state(&mut e).in_error_event.push(other);
    crate::errors::show(&mut e, FN_B, "outer");
    assert_eq!(
        e.mem.u32(AREA + 0x7ff0),
        1,
        "this thread's error ran its handler"
    );
    assert_eq!(state(&mut e).in_error_event, [other]);
}

#[path = "call_sites_tests.rs"]
mod call_sites_tests;

//! Tests of the events xNVSE raises at its own call sites (I-9, part 2): each seam called as the
//! translation calls it, with the event manager's test stand-ins (`events_tests.rs`); each
//! against the xNVSE 6.4.9 source it translates.

use super::*;
use crate::call_sites::{self, CellLoads};

/// A cell (form id `0x0001_0300`) and a terminal menu, in the test area.
const CELL: u32 = AREA + 0x7100;
const MENU: u32 = AREA + 0x7400;
/// A container changes entry (its item at `+8`) and an image-space modifier instance (its form
/// at `+0x1C`).
const ENTRY: u32 = AREA + 0x7600;
const INSTANCE: u32 = AREA + 0x7700;
/// The save-load singleton's stand-in.
const SAVE_LOAD: u32 = AREA + 0x7800;
/// The stand-in `Actor::AddWornItem`'s call count.
const WORN_CALLS: u32 = AREA + 0x7ff4;

/// The cell setter (`004512a0`): stores the state.
fn set_cell_state(e: &mut Engine, a: &[u32]) -> Ret {
    e.mem.set_u8(a[0] + 0x26, a[1] as u8);
    Ret::default()
}

/// `Actor::AddWornItem`'s stand-in: the game marks OnActorEquip (mask 2) on the actor's list on
/// the way, as `AddWornItem` does through `SetActionFlag`; answers 1.
fn add_worn_item(e: &mut Engine, a: &[u32]) -> Ret {
    let n = e.mem.u32(WORN_CALLS);
    e.mem.set_u32(WORN_CALLS, n + 1);
    e.call(SET_ACTION_FLAG.addr(), &args![a[1], a[0] + 0x44, 2u32]);
    Ret {
        eax: 1,
        ..Ret::default()
    }
}

/// The events' test engine with the call sites installed over stand-ins.
fn sites() -> Engine {
    let mut e = setup();
    e.map(0x011d_df38 & !0xfff, 0x1000);
    e.register(0x0045_12a0, set_cell_state);
    e.register(0x0088_db20, add_worn_item);
    call_sites::install(&mut e);
    e.mem.set_u32(CELL + 0xc, 0x0001_0300);
    e.mem.set_u8(CELL + 4, 0x39);
    e.mem.set_u32(MENU + 0x198, REF_1);
    e.mem.set_u32(ENTRY + 8, BASE_1);
    e.mem.set_u32(INSTANCE + 0x1c, REF_2);
    e
}

/// The handlers' first parameter (a form, by id) for each run.
fn first_args() -> Vec<(Vec<u8>, u32)> {
    log().into_iter().map(|r| (r.event, r.a_word)).collect()
}

/// The registry indexes the call sites use are xNVSE's event ids for those names.
#[test]
fn the_call_site_events_have_xnvses_ids() {
    let mut e = setup();
    let names: [(usize, &[u8]); 19] = [
        (id::ON_ACTOR_EQUIP, b"onactorequip"),
        (id::ON_DROP, b"ondrop"),
        (id::ON_SELL, b"onsell"),
        (id::ON_DROP_ITEM, b"ondropitem"),
        (id::ON_APPLY_IMOD, b"onapplyimod"),
        (id::ON_REMOVE_IMOD, b"onremoveimod"),
        (id::ON_LOCK_BROKEN, b"onlockbroken"),
        (id::ON_LOCK_PICK_SUCCESS, b"onlockpicksuccess"),
        (id::ON_LOCK_PICK_BROKEN, b"onlockpickbroken"),
        (id::ON_TERMINAL_HACKED, b"onterminalhacked"),
        (id::ON_TERMINAL_HACK_FAILED, b"onterminalhackfailed"),
        (id::ON_DISABLE, b"ondisable"),
        (id::ON_ENABLE, b"onenable"),
        (id::ON_REF_SET_3D, b"onrefset3d"),
        (id::ON_REF_UNSET_3D, b"onrefunset3d"),
        (id::ON_REF_ATTACH, b"onrefattach"),
        (id::ON_CELL_ATTACH, b"oncellattach"),
        (id::ON_CELL_DETACH, b"oncelldetach"),
        (id::ON_CELL_REFS_LOADED, b"oncellrefsloaded"),
    ];
    for (at, name) in names {
        assert_eq!(
            find(&mut e, name),
            Some(at),
            "{}",
            String::from_utf8_lossy(name)
        );
    }
}

/// The one-form events (`Hooks_Other.cpp`): nothing without a handler; with one, the form each
/// site names (the image-space modifier, the lock's or terminal's reference, the reference
/// enabled or disabled, the reference whose 3D is set, unset or attached).
#[test]
fn the_one_form_events_reach_their_handlers() {
    let mut e = sites();
    scripts(&mut e, &[(1, 4)]);
    let calls: [(u32, u32, &[u8]); 13] = [
        (0x0052_9a37, REF_1, b"onapplyimod"),
        (0x0052_9d37, REF_1, b"onremoveimod"),
        (0x0086_fe75, INSTANCE, b"onremoveimod"),
        (0x0079_0461, REF_1, b"onlockbroken"),
        (0x0078_f8e5, REF_1, b"onlockpicksuccess"),
        (0x0078_fe24, REF_1, b"onlockpickbroken"),
        (0x0076_717a, MENU, b"onterminalhacked"),
        (0x0076_74b0, MENU, b"onterminalhackfailed"),
        (0x005c_47b6, REF_1, b"ondisable"),
        (0x005c_453c, REF_1, b"onenable"),
        (0x0057_1090, REF_1, b"onrefset3d"),
        (0x0057_10ac, REF_1, b"onrefunset3d"),
        (0x0054_9787, REF_1, b"onrefattach"),
    ];
    e.mem.set_u32(REF_1 + 0x64, 0x1234);
    for (site, word, _) in calls {
        assert_eq!(e.site(site, &[word]), None);
    }
    assert!(log().is_empty(), "no handler: nothing");
    for (_, _, name) in calls {
        set_handler(&mut e, name, Callback::new(FN_A), 1, None);
    }
    for (site, word, _) in calls {
        assert_eq!(e.site(site, &[word]), None, "{site:08x}");
    }
    let want: Vec<(Vec<u8>, u32)> = calls
        .iter()
        .map(|(site, _, name)| {
            // The instance's form (the second reference) for the one that ran out.
            let id = if *site == 0x0086_fe75 {
                0x0001_0002
            } else {
                0x0001_0001
            };
            (name.to_vec(), id)
        })
        .collect();
    assert_eq!(first_args(), want);
    // No render state: no OnRefUnset3D.
    e.mem.set_u32(REF_1 + 0x64, 0);
    e.site(0x0057_10ac, &[REF_1]);
    assert!(log().is_empty());
}

/// `DispatchEventSafe`: not while a save loads, not for a deleted or temporary form, and on
/// another thread kept for the main thread's tick.
#[test]
fn safe_dispatches_wait_for_the_main_thread_and_skip_loads() {
    let mut e = sites();
    scripts(&mut e, &[(1, 4)]);
    set_handler(&mut e, b"OnRefAttach", Callback::new(FN_A), 1, None);
    e.set_global(0x011d_df38, SAVE_LOAD);
    e.mem.set_u32(SAVE_LOAD + 0x244, 2);
    e.site(0x0054_9787, &[REF_1]);
    e.mem.set_u32(SAVE_LOAD + 0x244, 0);
    e.mem.set_u32(REF_1 + 8, 0x4000);
    e.site(0x0054_9787, &[REF_1]);
    e.mem.set_u32(REF_1 + 8, 0);
    assert!(log().is_empty(), "loading, temporary");
    let main = engine_core::thread_host::current_id(&e);
    e.ext::<Nvse>().tick.main_thread = main.wrapping_add(1);
    e.site(0x0054_9787, &[REF_1]);
    assert!(log().is_empty(), "kept");
    e.ext::<Nvse>().tick.main_thread = main;
    tick(&mut e);
    assert_eq!(first_args(), [(b"onrefattach".to_vec(), 0x0001_0001)]);
}

/// OnActivate and OnActorEquip only once their handlers installed the hooks; the reference
/// first, the activator or item second. While `EquipObject`'s `AddWornItem` runs, the game's own
/// OnActorEquip mark is not raised again; the call's answer is kept.
#[test]
fn activate_and_equip_come_from_their_hooks() {
    let mut e = sites();
    scripts(&mut e, &[(1, 4), (2, 4)]);
    e.site(0x0057_318e, &[REF_1, REF_2]);
    e.site(0x004c_0032, &[REF_1, BASE_1]);
    assert!(log().is_empty());
    set_handler(&mut e, b"OnActivate", Callback::new(FN_A), 1, None);
    set_handler(&mut e, b"OnActorEquip", Callback::new(FN_B), 1, None);
    e.site(0x0057_318e, &[REF_1, REF_2]);
    tick(&mut e);
    e.site(0x004c_0032, &[REF_1, BASE_1]);
    let runs: Vec<_> = log()
        .into_iter()
        .map(|r| (r.script, r.a_word, r.b_word))
        .collect();
    assert_eq!(
        runs,
        [
            (FN_A, 0x0001_0001, 0x0001_0002),
            (FN_B, 0x0001_0001, 0x0001_0100)
        ]
    );
    tick(&mut e);
    // The worn item: the mark inside is not an event; outside it is.
    assert_eq!(e.site(0x0088_cd11, &[REF_1, BASE_1, 1, 0, 0]), Some(1));
    assert_eq!(e.mem.u32(WORN_CALLS), 1);
    assert!(log().is_empty());
    e.call(SET_ACTION_FLAG.addr(), &args![BASE_1, REF_1 + 0x44, 2u32]);
    assert_eq!(scripts_run(), [FN_B]);
}

/// OnSell: the entry's item and the seller, when the seller is a reference; nothing without a
/// handler.
#[test]
fn a_sale_raises_on_sell_with_the_item_and_the_seller() {
    let mut e = sites();
    scripts(&mut e, &[(1, 4), (2, 4)]);
    e.site(0x0072_ff20, &[ENTRY, REF_1]);
    assert!(log().is_empty());
    set_handler(&mut e, b"OnSell", Callback::new(FN_A), 1, None);
    e.site(0x0072_ff20, &[ENTRY, REF_1]);
    e.site(0x0072_fe3e, &[ENTRY, REF_2]);
    e.site(0x0072_fe3e, &[ENTRY, BASE_1]);
    let runs: Vec<_> = log().into_iter().map(|r| (r.a_word, r.b_word)).collect();
    assert_eq!(
        runs,
        [(0x0001_0100, 0x0001_0001), (0x0001_0100, 0x0001_0002)],
        "the base form is no seller"
    );
}

/// A dropped item: OnDrop with the reference made, OnDropItem with the item.
#[test]
fn a_drop_raises_on_drop_and_on_drop_item() {
    let mut e = sites();
    scripts(&mut e, &[(1, 4), (2, 4)]);
    set_handler(&mut e, b"OnDrop", Callback::new(FN_A), 1, None);
    set_handler(&mut e, b"OnDropItem", Callback::new(FN_B), 1, None);
    e.site(0x0057_515a, &[REF_1, BASE_1, REF_2]);
    e.site(0x0057_515a, &[REF_1, BASE_1, 0]);
    let runs: Vec<_> = log()
        .into_iter()
        .map(|r| (r.script, r.a_word, r.b_word))
        .collect();
    assert_eq!(
        runs,
        [
            (FN_A, 0x0001_0001, 0x0001_0002),
            (FN_B, 0x0001_0001, 0x0001_0100),
            (FN_B, 0x0001_0001, 0x0001_0100)
        ]
    );
}

/// The cell's state: the setter stores it, then OnCellDetach for "detaching" (4), OnCellAttach for
/// "attached" (6), nothing for the others.
#[test]
fn the_cell_state_setter_raises_attach_and_detach() {
    let mut e = sites();
    scripts(&mut e, &[(1, 4)]);
    set_handler(&mut e, b"OnCellAttach", Callback::new(FN_A), 1, None);
    set_handler(&mut e, b"OnCellDetach", Callback::new(FN_B), 1, None);
    for state in [2u32, 6, 3, 4, 0] {
        e.call(0x0045_12a0, &args![CELL, state]);
        assert_eq!(u32::from(e.mem.u8(CELL + 0x26)), state);
    }
    assert_eq!(
        first_args(),
        [
            (b"oncellattach".to_vec(), 0x0001_0300),
            (b"oncelldetach".to_vec(), 0x0001_0300)
        ]
    );
}

/// OnCellRefsLoaded: once per cell, for an attached cell with its loaded data and nothing
/// queued, marked whether or not a handler is set; again only after the cell is destroyed.
#[test]
fn cell_refs_loaded_comes_once_per_cell() {
    let mut e = sites();
    scripts(&mut e, &[(1, 4)]);
    e.mem.set_u8(CELL + 0x26, 6);
    e.mem.set_u32(CELL + 0xc4, 0x1000);
    e.mem.set_u32(CELL + 0xa4, 1);
    e.site(0x0055_18a5, &[CELL]);
    assert!(!e.ext_free::<CellLoads>().has(CELL), "a reference queued");
    e.mem.set_u32(CELL + 0xa4, 0);
    e.site(0x0055_18a5, &[CELL]);
    assert!(
        e.ext_free::<CellLoads>().has(CELL),
        "marked without a handler"
    );
    set_handler(&mut e, b"OnCellRefsLoaded", Callback::new(FN_A), 1, None);
    e.site(0x0055_18a5, &[CELL]);
    assert!(log().is_empty(), "once");
    // Another form's removal leaves the cells alone.
    e.mem.set_u8(CELL + 4, 0x3b);
    e.site(0x0048_3669, &[CELL]);
    assert!(
        e.ext_free::<CellLoads>().has(CELL),
        "not a cell: not looked at"
    );
    e.mem.set_u8(CELL + 4, 0x39);
    e.site(0x0048_3669, &[CELL]);
    assert!(!e.ext_free::<CellLoads>().has(CELL));
    e.site(0x0055_18a5, &[CELL]);
    e.site(0x0055_18a5, &[CELL]);
    assert_eq!(first_args(), [(b"oncellrefsloaded".to_vec(), 0x0001_0300)]);
    e.site(0x0086_80a4, &[CELL]);
    assert!(!e.ext_free::<CellLoads>().has(CELL));
}

/// The review of #340: an instance that runs out with a number where xNVSE reads its form (a
/// depth-of-field or radial blur instance) drops the event instead of stopping the game; with the
/// fixes preset's row, only a modifier form's instance (`AsForm`, virtual 0x94) raises it.
#[test]
fn an_instance_without_a_form_raises_no_remove_imod() {
    fn as_form_self(_: &mut Engine, a: &[u32]) -> Ret {
        Ret {
            eax: a[0],
            ..Ret::default()
        }
    }
    fn as_form_none(_: &mut Engine, _: &[u32]) -> Ret {
        Ret::default()
    }
    const BLUR: u32 = AREA + 0x7a00;
    const VTABLE_FORM: u32 = AREA + 0x7b00;
    const VTABLE_BLUR: u32 = AREA + 0x7c00;
    let mut e = sites();
    scripts(&mut e, &[(1, 4)]);
    set_handler(&mut e, b"OnRemoveIMOD", Callback::new(FN_A), 1, None);
    e.register(AREA + 0x7d00, as_form_self);
    e.register(AREA + 0x7d10, as_form_none);
    e.mem.set_u32(VTABLE_FORM + 0x94, AREA + 0x7d00);
    e.mem.set_u32(VTABLE_BLUR + 0x94, AREA + 0x7d10);
    e.mem.set_u32(INSTANCE, VTABLE_FORM);
    e.mem.set_u32(BLUR, VTABLE_BLUR);
    // A blur's duration (1.0) where the form would be: no readable form.
    e.mem.set_u32(BLUR + 0x1c, 0x3f80_0000);
    for check in [false, true] {
        crate::call_sites::set_imod_form_check(&mut e, check);
        e.site(0x0086_fe75, &[BLUR]);
        assert!(log().is_empty(), "check {check}");
        e.site(0x0086_fe75, &[INSTANCE]);
        assert_eq!(first_args(), [(b"onremoveimod".to_vec(), 0x0001_0002)]);
    }
    // A blur whose word happens to be a readable form: vanilla raises it as xNVSE does, the row
    // does not.
    e.mem.set_u32(BLUR + 0x1c, REF_1);
    crate::call_sites::set_imod_form_check(&mut e, false);
    e.site(0x0086_fe75, &[BLUR]);
    assert_eq!(log().len(), 1);
    crate::call_sites::set_imod_form_check(&mut e, true);
    e.site(0x0086_fe75, &[BLUR]);
    assert!(log().is_empty());
}

/// The review of #340: a stop inside `AddWornItem` still clears the flag, so the thread's later
/// OnActorEquip marks are raised.
#[test]
fn a_stop_inside_add_worn_item_clears_the_flag() {
    fn stopping(_: &mut Engine, _: &[u32]) -> Ret {
        panic!("a stop inside AddWornItem")
    }
    let mut e = sites();
    scripts(&mut e, &[(1, 4), (2, 4)]);
    set_handler(&mut e, b"OnActorEquip", Callback::new(FN_B), 1, None);
    e.register(0x0088_db20, stopping);
    let stopped = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        e.site(0x0088_caeb, &[REF_1, BASE_1, 1, 0, 0])
    }));
    assert!(stopped.is_err());
    e.call(SET_ACTION_FLAG.addr(), &args![BASE_1, REF_1 + 0x44, 2u32]);
    assert_eq!(scripts_run(), [FN_B]);
}

/// `RuntimeScriptError`'s dispatch that panics still takes the thread's guard entry out.
#[test]
fn a_panicking_error_dispatch_clears_the_guard() {
    fn panicking(_: &mut Engine, _: &[u32]) -> Ret {
        panic!("a stop inside the handler")
    }
    let mut e = setup();
    e.register(SCRIPT_EXECUTE, panicking);
    scripts(&mut e, &[(1, 2)]);
    set_error_guard(&mut e, true);
    set_handler(&mut e, b"RuntimeScriptError", Callback::new(FN_A), 1, None);
    let shown = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        crate::errors::show(&mut e, FN_B, "outer")
    }));
    assert!(shown.is_err(), "the panic goes on");
    assert!(state(&mut e).in_error_event.is_empty());
}

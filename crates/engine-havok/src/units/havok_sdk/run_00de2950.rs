//! `Havok SDK/run_00de2950` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `hkpContactPointViewer` (class confirmed by the RTTI of its vtables `010d8184`, `010d8164`,
//! `010d8154` and `010d8144`): the thunks of the world listener sub-object at `+0x20`
//! (vtable `010d8154`, slots `+4` and `+8`), the constructor, the destructor and a function of
//! the process sub-object at `+8`. The viewer's own layout: the physics context at `+0x24`
//! (an array of worlds at `+0x6c`, its count at `+0x70`), the post simulation listener
//! sub-object at `+0x28`.

#[allow(unused_imports)]
use crate::prelude::*;

/// `hkpWorld::addWorldPostSimulationListener` (`00c93920`, thiscall on the world, one word).
const WORLD_ADD_LISTENER: u32 = 0x00c9_3920;
/// The removal of a listener from the world's array (`00c918e0`, thiscall on the world, one
/// word).
const WORLD_REMOVE_LISTENER: u32 = 0x00c9_18e0;
/// `hkpWorldViewerBase::hkpWorldViewerBase` (`00ddccb0`, thiscall, one word) and the base
/// class destructor (`00ddcc60`, thiscall, no words).
const BASE_CONSTRUCT: u32 = 0x00dd_ccb0;
const BASE_DESTRUCT: u32 = 0x00dd_cc60;

/// The vtables the viewer stores at `+0`, `+8`, `+0x20` and `+0x28`, and the vtable of
/// `hkpWorldPostSimulationListener` (RTTI) the constructor puts at `+0x28` first and the
/// destructor leaves there.
const VTABLE_VIEWER: u32 = 0x010d_8184;
const VTABLE_PROCESS: u32 = 0x010d_8164;
const VTABLE_CONTEXT_LISTENER: u32 = 0x010d_8154;
const VTABLE_POST_SIMULATION: u32 = 0x010d_8144;
const VTABLE_POST_SIMULATION_BASE: u32 = 0x010c_cf7c;

// Translated from 00de2950 (decompiled, FalloutNV.exe 1.4.0.525)
/// Slot `+4` of the world listener sub-object (`this` = viewer `+0x20`): adds the viewer's post
/// simulation listener, the sub-object at `this + 8` (null when `this` is `0x20`, the
/// compiler's null test of the viewer pointer `this - 0x20`), to the world given on the stack
/// with `hkpWorld::addWorldPostSimulationListener` (`00c93920`).
pub fn fn_00de2950(e: &mut Engine, this: Ptr, world: u32) {
    let viewer = this.addr().wrapping_sub(0x20);
    let listener = if viewer != 0 {
        this.addr().wrapping_add(8)
    } else {
        0
    };
    e.call(WORLD_ADD_LISTENER, &args![world, listener]);
}

// Translated from 00de2980 (decompiled, FalloutNV.exe 1.4.0.525)
/// Slot `+8` of the world listener sub-object (`this` = viewer `+0x20`): removes the viewer's
/// post simulation listener (the sub-object at `this + 8`, null when `this` is `0x20`) from the
/// world given on the stack, with the function at `00c918e0`.
pub fn fn_00de2980(e: &mut Engine, this: Ptr, world: u32) {
    let viewer = this.addr().wrapping_sub(0x20);
    let listener = if viewer != 0 {
        this.addr().wrapping_add(8)
    } else {
        0
    };
    e.call(WORLD_REMOVE_LISTENER, &args![world, listener]);
}

// Translated from 00de29b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpContactPointViewer::hkpContactPointViewer` (constructor; class confirmed by the RTTI),
/// thiscall with two words `(contexts, arg_2)`: runs the base constructor `00ddccb0` with
/// `contexts`, stores the vtable of `hkpWorldPostSimulationListener` at `+0x28`, `0.0` at
/// `+0x2c`, then the viewer's vtables at `+0`, `+8`, `+0x20` and `+0x28`, and `arg_2` at
/// `+0x30`. Returns `this`.
pub fn fn_00de29b0(e: &mut Engine, this: Ptr, contexts: u32, arg_2: u32) -> Ptr {
    let me = this.addr();
    e.call(BASE_CONSTRUCT, &args![me, contexts]);
    e.mem
        .set_u32(me.wrapping_add(0x28), VTABLE_POST_SIMULATION_BASE);
    e.mem.set_f32(me.wrapping_add(0x2c), 0.0);
    e.mem.set_u32(me, VTABLE_VIEWER);
    e.mem.set_u32(me.wrapping_add(8), VTABLE_PROCESS);
    e.mem
        .set_u32(me.wrapping_add(0x20), VTABLE_CONTEXT_LISTENER);
    e.mem.set_u32(me.wrapping_add(0x28), VTABLE_POST_SIMULATION);
    e.mem.set_u32(me.wrapping_add(0x30), arg_2);
    this
}

// Translated from 00de2a00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Function of the process sub-object (`this` = viewer `+8`, slot `+0x14` of vtable
/// `010d8164`; the engine map has no name): for every world of the physics context at
/// `this + 0x1c` (the array at context `+0x6c`, count at `+0x70`, the context re-read each
/// round) calls slot `+4` of the world listener sub-object at `this + 0x18` with the world.
pub fn fn_00de2a00(e: &mut Engine, this: Ptr) {
    let me = this.addr();
    let mut index: i32 = 0;
    let mut context = e.mem.u32(me.wrapping_add(0x1c));
    while context != 0 {
        if index >= e.mem.i32(context.wrapping_add(0x70)) {
            break;
        }
        let worlds = e.mem.u32(context.wrapping_add(0x6c));
        let world = e
            .mem
            .u32(worlds.wrapping_add((index as u32).wrapping_mul(4)));
        e.vcall(me.wrapping_add(0x18), 4, &args![world]);
        index = index.wrapping_add(1);
        context = e.mem.u32(me.wrapping_add(0x1c));
    }
}

// Translated from 00de2a40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpContactPointViewer::~hkpContactPointViewer` (destructor; class confirmed by the RTTI),
/// thiscall: stores the viewer's vtables (`+0`, `+8`, `+0x20`, `+0x28`); while the physics
/// context at `+0x24` is set walks its worlds (array at context `+0x6c`, count at `+0x70`,
/// the context re-read each round) and removes the listener sub-object at `+0x20` from each
/// through its slot `+8` code `00de2980`; stores the post simulation listener's vtable at
/// `+0x28` and ends in the base class destructor `00ddcc60`.
pub fn fn_00de2a40(e: &mut Engine, this: Ptr) {
    let me = this.addr();
    let listener = me.wrapping_add(0x20);
    e.mem.set_u32(me, VTABLE_VIEWER);
    e.mem.set_u32(me.wrapping_add(8), VTABLE_PROCESS);
    e.mem.set_u32(listener, VTABLE_CONTEXT_LISTENER);
    e.mem.set_u32(me.wrapping_add(0x28), VTABLE_POST_SIMULATION);
    if e.mem.u32(me.wrapping_add(0x24)) != 0 {
        let mut index: i32 = 0;
        loop {
            let context = e.mem.u32(me.wrapping_add(0x24));
            if index >= e.mem.i32(context.wrapping_add(0x70)) {
                break;
            }
            let worlds = e.mem.u32(context.wrapping_add(0x6c));
            let world = e
                .mem
                .u32(worlds.wrapping_add((index as u32).wrapping_mul(4)));
            fn_00de2980(e, Ptr::new(listener), world);
            index = index.wrapping_add(1);
            if e.mem.u32(me.wrapping_add(0x24)) == 0 {
                break;
            }
        }
    }
    e.mem
        .set_u32(me.wrapping_add(0x28), VTABLE_POST_SIMULATION_BASE);
    e.call(BASE_DESTRUCT, &args![me]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00de_2950, fn_00de2950(Ptr, u32)),
        entry!(0x00de_2980, fn_00de2980(Ptr, u32)),
        entry!(0x00de_29b0, fn_00de29b0(Ptr, u32, u32) -> Ptr),
        entry!(0x00de_2a00, fn_00de2a00(Ptr)),
        entry!(0x00de_2a40, fn_00de2a40(Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    type Log = Rc<RefCell<Vec<(u32, Vec<u32>)>>>;

    /// Doubles over `addresses` that log `(address, words)`.
    fn log_calls(e: &mut Engine, addresses: &[u32]) -> Log {
        let log: Log = Rc::default();
        for &address in addresses {
            let log = log.clone();
            e.register_double(address, move |_, words| {
                log.borrow_mut().push((address, words.to_vec()));
                Ret::default()
            });
        }
        log
    }

    /// A physics context with `worlds` in its array.
    fn context(e: &mut Engine, worlds: &[u32]) -> u32 {
        let context = e.mem.alloc(0x80);
        let array = e.mem.alloc(16);
        for (i, w) in worlds.iter().enumerate() {
            e.mem.set_u32(array + 4 * i as u32, *w);
        }
        e.mem.set_u32(context + 0x6c, array);
        e.mem.set_u32(context + 0x70, worlds.len() as u32);
        context
    }

    #[test]
    fn world_added_adds_the_post_simulation_listener() {
        let mut e = Engine::new();
        let log = log_calls(&mut e, &[WORLD_ADD_LISTENER]);
        e.call(0x00de_2950, &args![0x1020u32, 0x5555u32]);
        e.call(0x00de_2950, &args![0x20u32, 0x6666u32]);
        assert_eq!(
            *log.borrow(),
            vec![
                (WORLD_ADD_LISTENER, vec![0x5555, 0x1028]),
                (WORLD_ADD_LISTENER, vec![0x6666, 0])
            ]
        );
    }

    #[test]
    fn world_removed_removes_the_post_simulation_listener() {
        let mut e = Engine::new();
        let log = log_calls(&mut e, &[WORLD_REMOVE_LISTENER]);
        e.call(0x00de_2980, &args![0x1020u32, 0x5555u32]);
        e.call(0x00de_2980, &args![0x20u32, 0x6666u32]);
        assert_eq!(
            *log.borrow(),
            vec![
                (WORLD_REMOVE_LISTENER, vec![0x5555, 0x1028]),
                (WORLD_REMOVE_LISTENER, vec![0x6666, 0])
            ]
        );
    }

    #[test]
    fn the_constructor_runs_the_base_and_stores_the_vtables() {
        let mut e = Engine::new();
        let log = log_calls(&mut e, &[BASE_CONSTRUCT]);
        let viewer = e.mem.alloc(0x40);
        e.mem.set_f32(viewer + 0x2c, 9.0);
        let result = e.call(0x00de_29b0, &args![viewer, 0x7000u32, 0x3f80_0000u32]);
        assert_eq!(result.u32(), viewer);
        assert_eq!(*log.borrow(), vec![(BASE_CONSTRUCT, vec![viewer, 0x7000])]);
        assert_eq!(e.mem.u32(viewer), 0x010d_8184);
        assert_eq!(e.mem.u32(viewer + 8), 0x010d_8164);
        assert_eq!(e.mem.u32(viewer + 0x20), 0x010d_8154);
        assert_eq!(e.mem.u32(viewer + 0x28), 0x010d_8144);
        assert_eq!(e.mem.u32(viewer + 0x2c), 0);
        assert_eq!(e.mem.u32(viewer + 0x30), 0x3f80_0000);
    }

    #[test]
    fn init_calls_the_listener_slot_for_every_world() {
        let mut e = Engine::new();
        let process = e.mem.alloc(0x40);
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(process + 0x18, vtable);
        e.mem.set_u32(vtable + 4, 0x0900_0004);
        let ctx = context(&mut e, &[0x111, 0x222, 0x333]);
        e.mem.set_u32(process + 0x1c, ctx);
        let log = log_calls(&mut e, &[0x0900_0004]);
        e.call(0x00de_2a00, &args![process]);
        assert_eq!(
            *log.borrow(),
            vec![
                (0x0900_0004, vec![process + 0x18, 0x111]),
                (0x0900_0004, vec![process + 0x18, 0x222]),
                (0x0900_0004, vec![process + 0x18, 0x333])
            ]
        );
    }

    #[test]
    fn init_without_a_context_does_nothing() {
        let mut e = Engine::new();
        let process = e.mem.alloc(0x40);
        e.call(0x00de_2a00, &args![process]);
    }

    #[test]
    fn the_destructor_removes_the_listener_from_every_world() {
        let mut e = Engine::new();
        let log = log_calls(&mut e, &[WORLD_REMOVE_LISTENER, BASE_DESTRUCT]);
        let viewer = e.mem.alloc(0x40);
        let ctx = context(&mut e, &[0x111, 0x222]);
        e.mem.set_u32(viewer + 0x24, ctx);
        e.call(0x00de_2a40, &args![viewer]);
        assert_eq!(
            *log.borrow(),
            vec![
                (WORLD_REMOVE_LISTENER, vec![0x111, viewer + 0x28]),
                (WORLD_REMOVE_LISTENER, vec![0x222, viewer + 0x28]),
                (BASE_DESTRUCT, vec![viewer])
            ]
        );
        assert_eq!(e.mem.u32(viewer), 0x010d_8184);
        assert_eq!(e.mem.u32(viewer + 8), 0x010d_8164);
        assert_eq!(e.mem.u32(viewer + 0x20), 0x010d_8154);
        assert_eq!(e.mem.u32(viewer + 0x28), 0x010c_cf7c);
    }

    #[test]
    fn the_destructor_without_a_context_only_runs_the_base() {
        let mut e = Engine::new();
        let log = log_calls(&mut e, &[WORLD_REMOVE_LISTENER, BASE_DESTRUCT]);
        let viewer = e.mem.alloc(0x40);
        e.call(0x00de_2a40, &args![viewer]);
        assert_eq!(*log.borrow(), vec![(BASE_DESTRUCT, vec![viewer])]);
    }

    #[test]
    fn the_destructor_stops_when_a_removal_clears_the_context() {
        let mut e = Engine::new();
        let viewer = e.mem.alloc(0x40);
        let ctx = context(&mut e, &[0x111, 0x222]);
        e.mem.set_u32(viewer + 0x24, ctx);
        let log = log_calls(&mut e, &[BASE_DESTRUCT]);
        // the first removal clears the viewer's context
        let removals = log.clone();
        e.register_double(WORLD_REMOVE_LISTENER, move |e, words| {
            removals
                .borrow_mut()
                .push((WORLD_REMOVE_LISTENER, words.to_vec()));
            e.mem.set_u32(viewer + 0x24, 0);
            Ret::default()
        });
        e.call(0x00de_2a40, &args![viewer]);
        assert_eq!(
            *log.borrow(),
            vec![
                (WORLD_REMOVE_LISTENER, vec![0x111, viewer + 0x28]),
                (BASE_DESTRUCT, vec![viewer])
            ]
        );
    }
}

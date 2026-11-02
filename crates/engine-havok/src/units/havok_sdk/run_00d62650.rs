//! `Havok SDK/run_00d62650` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `hkpWorld::addWorldPostSimulationListener` (Xbox PDB, `00c93920`: thiscall on the world, one
/// argument, the listener) and the matching removal (`00c918e0`, same shape: the first array
/// entry equal to the listener is set to zero), `hkpWorldViewerBase`'s constructor (Xbox PDB,
/// `00ddccb0`, thiscall, one argument: the process contexts; it finds the world and stores it at
/// `+0x24`).
const ADD_LISTENER: u32 = 0x00c9_3920;
const REMOVE_LISTENER: u32 = 0x00c9_18e0;
const WORLD_VIEWER_CONSTRUCT: u32 = 0x00dd_ccb0;
/// The vtable pointers the welding viewer's constructor stores (RTTI: `hkpWeldingViewer`): the
/// live ones at `+0`, `+8` and `+0x20`, the listener's at `+0x28`, and the listener's base
/// class one (`hkpWorldPostSimulationListener`).
const VTABLE_VIEWER: u32 = 0x010d_1634;
const VTABLE_PROCESS: u32 = 0x010d_1614;
const VTABLE_FILTER: u32 = 0x010d_1604;
const VTABLE_LISTENER: u32 = 0x010d_15f4;
const VTABLE_LISTENER_BASE: u32 = 0x010c_cf7c;

/// The listener the thunks below pass: `this + 8` when the object they are adjusted to (`this -
/// 0x20`) is not null, otherwise 0.
fn adjusted_listener(this: Ptr) -> u32 {
    if this.addr().wrapping_sub(0x20) != 0 {
        this.addr().wrapping_add(8)
    } else {
        0
    }
}

// Translated from 00d62650 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk-like method of a sub-object at `+0x20` (`this - 0x20` is the whole object): adds the
/// listener `this + 8` (0 when the whole object would be null) to the world's post-simulation
/// listeners (`00c93920` on `world`, which comes as the one stack word).
pub fn fn_00d62650(e: &mut Engine, this: Ptr, world: Ptr) {
    let listener = adjusted_listener(this);
    e.call(ADD_LISTENER, &args![world, listener]);
}

// Translated from 00d62680 (decompiled, FalloutNV.exe 1.4.0.525)
/// Like `00d62650`, removing the listener (`00c918e0`).
pub fn fn_00d62680(e: &mut Engine, this: Ptr, world: Ptr) {
    let listener = adjusted_listener(this);
    e.call(REMOVE_LISTENER, &args![world, listener]);
}

// Translated from 00d626b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The constructor of the viewer whose vtables are `010d1634` and the three after it (RTTI:
/// `hkpWeldingViewer`): runs the `hkpWorldViewerBase` constructor (`00ddccb0`, with the process
/// `contexts`), stores the vtable pointers (`+0x28` first with the listener base class, then
/// `+0`, `+8`, `+0x20` and the listener's own at `+0x28`) and, when a world was found
/// (`+0x24`), registers the listener (`this + 0x28`, `00c93920`) with each of the
/// `[world + 0x70]` entries of the array at `[world + 0x6c]`. Answers `this`.
pub fn fn_00d626b0(e: &mut Engine, this: Ptr, contexts: Ptr) -> Ptr {
    let at = this.addr();
    e.call(WORLD_VIEWER_CONSTRUCT, &args![this, contexts]);
    e.mem.set_u32(at.wrapping_add(0x28), VTABLE_LISTENER_BASE);
    let listener = at.wrapping_add(0x28);
    let world = e.mem.u32(at.wrapping_add(0x24));
    e.mem.set_u32(at, VTABLE_VIEWER);
    e.mem.set_u32(at.wrapping_add(8), VTABLE_PROCESS);
    e.mem.set_u32(at.wrapping_add(0x20), VTABLE_FILTER);
    e.mem.set_u32(listener, VTABLE_LISTENER);
    if world != 0 && e.mem.i32(world.wrapping_add(0x70)) > 0 {
        let mut index = 0u32;
        loop {
            let world = e.mem.u32(at.wrapping_add(0x24));
            let worlds = e.mem.u32(world.wrapping_add(0x6c));
            let target = e.mem.u32(worlds.wrapping_add(index.wrapping_mul(4)));
            e.call(ADD_LISTENER, &args![target, listener]);
            let world = e.mem.u32(at.wrapping_add(0x24));
            index = index.wrapping_add(1);
            if (index as i32) >= e.mem.i32(world.wrapping_add(0x70)) {
                break;
            }
        }
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00d6_2650, fn_00d62650(Ptr, Ptr)),
        entry!(0x00d6_2680, fn_00d62680(Ptr, Ptr)),
        entry!(0x00d6_26b0, fn_00d626b0(Ptr, Ptr) -> Ptr),
    ]
}

#[cfg(test)]
mod welding_viewer_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    type Log = Rc<RefCell<Vec<(u32, Vec<u32>)>>>;

    fn record(e: &mut Engine, log: &Log, address: u32) {
        let seen = Rc::clone(log);
        e.register_double(address, move |_, a| {
            seen.borrow_mut().push((address, a.to_vec()));
            Ret::default()
        });
    }

    #[test]
    fn fn_00d62650_adds_the_adjusted_listener() {
        let mut e = Engine::new();
        let log: Log = Rc::default();
        record(&mut e, &log, ADD_LISTENER);
        e.call(0x00d6_2650, &args![0x2020u32, 0x5000u32]);
        // `this - 0x20` is null: the listener is 0.
        e.call(0x00d6_2650, &args![0x20u32, 0x5000u32]);
        assert_eq!(
            *log.borrow(),
            vec![
                (ADD_LISTENER, vec![0x5000, 0x2028]),
                (ADD_LISTENER, vec![0x5000, 0]),
            ]
        );
    }

    #[test]
    fn fn_00d62680_removes_the_adjusted_listener() {
        let mut e = Engine::new();
        let log: Log = Rc::default();
        record(&mut e, &log, REMOVE_LISTENER);
        e.call(0x00d6_2680, &args![0x2020u32, 0x5000u32]);
        e.call(0x00d6_2680, &args![0x20u32, 0x5000u32]);
        assert_eq!(
            *log.borrow(),
            vec![
                (REMOVE_LISTENER, vec![0x5000, 0x2028]),
                (REMOVE_LISTENER, vec![0x5000, 0]),
            ]
        );
    }

    /// A viewer whose base constructor double finds `world` (written at `+0x24`, after setting
    /// `+0x28` to a marker the constructor must overwrite).
    fn viewer(e: &mut Engine, world: u32) -> (u32, Log) {
        let this = e.mem.alloc(0x40);
        let log: Log = Rc::default();
        record(e, &log, ADD_LISTENER);
        let seen = Rc::clone(&log);
        e.register_double(WORLD_VIEWER_CONSTRUCT, move |e, a| {
            seen.borrow_mut().push((WORLD_VIEWER_CONSTRUCT, a.to_vec()));
            e.mem.set_u32(a[0] + 0x24, world);
            e.mem.set_u32(a[0] + 0x28, 0xdead);
            Ret::default()
        });
        (this, log)
    }

    #[test]
    fn fn_00d626b0_stores_the_vtables_and_registers_with_each_world() {
        let mut e = Engine::new();
        let world = e.mem.alloc(0x80);
        let list = e.mem.alloc(12);
        for i in 0..3u32 {
            e.mem.set_u32(list + 4 * i, 0x1000 + i);
        }
        e.mem.set_u32(world + 0x70, 3);
        e.mem.set_u32(world + 0x6c, list);
        let (this, log) = viewer(&mut e, world);
        let got = e.call(0x00d6_26b0, &args![this, 0x1234u32]).ptr::<()>();
        assert_eq!(got.addr(), this);
        assert_eq!(e.mem.u32(this), VTABLE_VIEWER);
        assert_eq!(e.mem.u32(this + 8), VTABLE_PROCESS);
        assert_eq!(e.mem.u32(this + 0x20), VTABLE_FILTER);
        assert_eq!(e.mem.u32(this + 0x28), VTABLE_LISTENER);
        assert_eq!(
            *log.borrow(),
            vec![
                (WORLD_VIEWER_CONSTRUCT, vec![this, 0x1234]),
                (ADD_LISTENER, vec![0x1000, this + 0x28]),
                (ADD_LISTENER, vec![0x1001, this + 0x28]),
                (ADD_LISTENER, vec![0x1002, this + 0x28]),
            ]
        );
    }

    #[test]
    fn fn_00d626b0_without_a_world_or_worlds_registers_nothing() {
        let mut e = Engine::new();
        let (this, log) = viewer(&mut e, 0);
        e.call(0x00d6_26b0, &args![this, 0u32]);
        assert_eq!(log.borrow().len(), 1);
        let mut e = Engine::new();
        let world = e.mem.alloc(0x80);
        let (this, log) = viewer(&mut e, world);
        e.call(0x00d6_26b0, &args![this, 0u32]);
        assert_eq!(log.borrow().len(), 1);
        assert_eq!(e.mem.u32(this + 0x28), VTABLE_LISTENER);
    }
}

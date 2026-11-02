//! `Havok SDK/run_00d6a1a0` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The memory router (`0x00c8_5750`): its field `+0x10` is the allocator the block is freed through.
const MEMORY_ROUTER: u32 = 0x00c8_5750;

// Translated from 00d6a1f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Deleting destructor: runs the base destructor (`0x00d6_a0a0`) on `this` and, when bit 0 of
/// `flags` is set, returns the block to the memory router's allocator (slot `8`) with the
/// 16-bit size kept at `this + 4`. Returns `this`. The class is unconfirmed.
pub fn deleting_destructor_00d6a1f0(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    e.call(0x00d6_a0a0, &args![this]);
    if flags & 1 != 0 {
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let size = u32::from(e.mem.u16(this.addr() + 4));
        e.vcall(allocator, 8, &args![this, size]);
    }
    this
}

// Translated from 00d6a1b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by `0x8` (`SUB ECX,0x8`) and jumps to `00d6a1f0`, which (`RET 4`) has the shape of a scalar deleting destructor: it reads bit 0 of the one stack word. The caller gets its `EAX`; the class is unconfirmed.
pub fn fn_00d6a1b0(e: &mut Engine, this: Ptr, flags: u32) -> u32 {
    e.call(0x00d6_a1f0, &args![this.addr().wrapping_sub(0x8), flags])
        .u32()
}

// Translated from 00d6a1c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by `0x28` (`SUB ECX,0x28`) and jumps to `00d6a1f0`, which (`RET 4`) has the shape of a scalar deleting destructor: it reads bit 0 of the one stack word. The caller gets its `EAX`; the class is unconfirmed.
pub fn fn_00d6a1c0(e: &mut Engine, this: Ptr, flags: u32) -> u32 {
    e.call(0x00d6_a1f0, &args![this.addr().wrapping_sub(0x28), flags])
        .u32()
}

// Translated from 00d6a1d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by `0x2c` (`SUB ECX,0x2c`) and jumps to `00d6a1f0`, which (`RET 4`) has the shape of a scalar deleting destructor: it reads bit 0 of the one stack word. The caller gets its `EAX`; the class is unconfirmed.
pub fn fn_00d6a1d0(e: &mut Engine, this: Ptr, flags: u32) -> u32 {
    e.call(0x00d6_a1f0, &args![this.addr().wrapping_sub(0x2c), flags])
        .u32()
}

// Translated from 00d6a1e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by `0x20` (`SUB ECX,0x20`) and jumps to `00d6a1f0`, which (`RET 4`) has the shape of a scalar deleting destructor: it reads bit 0 of the one stack word. The caller gets its `EAX`; the class is unconfirmed.
pub fn fn_00d6a1e0(e: &mut Engine, this: Ptr, flags: u32) -> u32 {
    e.call(0x00d6_a1f0, &args![this.addr().wrapping_sub(0x20), flags])
        .u32()
}

/// `hkpWorld::addWorldPostSimulationListener` (Xbox PDB, `00c93920`: thiscall on the world, one
/// argument, the listener) and the matching removal (`00c918e0`, same shape: the first array
/// entry equal to the listener is set to zero), `hkpWorldViewerBase`'s constructor (Xbox PDB,
/// `00ddccb0`, thiscall, one argument: the process contexts; it finds the world and stores it at
/// `+0x24`).
const ADD_LISTENER: u32 = 0x00c9_3920;
const REMOVE_LISTENER: u32 = 0x00c9_18e0;
const WORLD_VIEWER_CONSTRUCT: u32 = 0x00dd_ccb0;
/// The process id the rigid body centre of mass viewer answers (`01269020`).
const CENTRE_OF_MASS_PROCESS_ID: u32 = 0x0126_9020;
/// The vtable pointers of the midphase viewer's constructor (RTTI: `hkpMidphaseViewer`): the
/// live ones at `+0`, `+8` and `+0x20`, the listener's at `+0x28`, and the listener's base class
/// one (`hkpWorldPostSimulationListener`).
const VTABLE_VIEWER: u32 = 0x010d_2330;
const VTABLE_PROCESS: u32 = 0x010d_2310;
const VTABLE_FILTER: u32 = 0x010d_2300;
const VTABLE_LISTENER: u32 = 0x010d_22f0;
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

// Translated from 00d6a1a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Answers the process id kept at `01269020`: slot 1 of the vtable `010d2288` (RTTI:
/// `hkpRigidBodyCentreOfMassViewer`); `ECX` unused.
pub fn fn_00d6a1a0(e: &mut Engine) -> u32 {
    e.global(CENTRE_OF_MASS_PROCESS_ID)
}

// Translated from 00d6a220 (decompiled, FalloutNV.exe 1.4.0.525)
/// Method of a sub-object at `+0x20` (`this - 0x20` is the whole object): adds the listener
/// `this + 8` (0 when the whole object would be null) to the world's post-simulation listeners
/// (`00c93920` on `world`, which comes as the one stack word). Slot 1 of the vtable `010d2300`
/// (RTTI: `hkpMidphaseViewer`).
pub fn fn_00d6a220(e: &mut Engine, this: Ptr, world: Ptr) {
    let listener = adjusted_listener(this);
    e.call(ADD_LISTENER, &args![world, listener]);
}

// Translated from 00d6a250 (decompiled, FalloutNV.exe 1.4.0.525)
/// Like `00d6a220`, removing the listener (`00c918e0`); slot 2 of the vtable `010d2300`.
pub fn fn_00d6a250(e: &mut Engine, this: Ptr, world: Ptr) {
    let listener = adjusted_listener(this);
    e.call(REMOVE_LISTENER, &args![world, listener]);
}

// Translated from 00d6a280 (decompiled, FalloutNV.exe 1.4.0.525)
/// The constructor of the midphase viewer (RTTI: `hkpMidphaseViewer`, vtables `010d2330` and the
/// three after it): runs the `hkpWorldViewerBase` constructor (`00ddccb0`, with the process
/// `contexts`), stores the listener base vtable at `+0x28`, the vtables (`+0`, `+8`, `+0x20`, the
/// listener's at `+0x28`), an empty array at `+0x2c` (data, size, capacity word `0x80000000`)
/// and, when a world was found (`+0x24`) with worlds, registers the listener (`this + 0x28`,
/// `00c93920`) with each of the `[world + 0x70]` entries of the array at `[world + 0x6c]`.
/// Answers `this`.
pub fn fn_00d6a280(e: &mut Engine, this: Ptr, contexts: Ptr) -> Ptr {
    let at = this.addr();
    e.call(WORLD_VIEWER_CONSTRUCT, &args![this, contexts]);
    e.mem.set_u32(at.wrapping_add(0x28), VTABLE_LISTENER_BASE);
    let listener = at.wrapping_add(0x28);
    e.mem.set_u32(at, VTABLE_VIEWER);
    e.mem.set_u32(at.wrapping_add(8), VTABLE_PROCESS);
    e.mem.set_u32(at.wrapping_add(0x20), VTABLE_FILTER);
    e.mem.set_u32(listener, VTABLE_LISTENER);
    e.mem.set_u32(at.wrapping_add(0x2c), 0);
    e.mem.set_u32(at.wrapping_add(0x30), 0);
    e.mem.set_u32(at.wrapping_add(0x34), 0x8000_0000);
    let world = e.mem.u32(at.wrapping_add(0x24));
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
        entry!(0x00d6_a1a0, fn_00d6a1a0() -> u32),
        entry!(0x00d6_a220, fn_00d6a220(Ptr, Ptr)),
        entry!(0x00d6_a250, fn_00d6a250(Ptr, Ptr)),
        entry!(0x00d6_a280, fn_00d6a280(Ptr, Ptr) -> Ptr),
        entry!(0x00d6_a1f0, deleting_destructor_00d6a1f0(Ptr, u8) -> Ptr),
        entry!(0x00d6_a1b0, fn_00d6a1b0(Ptr, u32) -> u32),
        entry!(0x00d6_a1c0, fn_00d6a1c0(Ptr, u32) -> u32),
        entry!(0x00d6_a1d0, fn_00d6a1d0(Ptr, u32) -> u32),
        entry!(0x00d6_a1e0, fn_00d6a1e0(Ptr, u32) -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    const FREE_SLOT_DOUBLE: u32 = 0x0900_0011;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.map(0x00fd_f000, 0x1000);
        e
    }

    /// Installs a base-destructor double at `base` and a memory router whose allocator
    /// frees through a double; both push their arguments (tagged 1 and 2) on the log.
    fn install(e: &mut Engine, base: u32) -> Rc<RefCell<Vec<u32>>> {
        let log = Rc::new(RefCell::new(Vec::new()));
        let base_log = Rc::clone(&log);
        e.register_double(base, move |_, a| {
            base_log.borrow_mut().extend([1, a[0]]);
            Ret::default()
        });
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 8, FREE_SLOT_DOUBLE);
        let allocator = e.mem.alloc(0x10);
        e.mem.set_u32(allocator, vtable);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        let free_log = Rc::clone(&log);
        e.register_double(FREE_SLOT_DOUBLE, move |_, a| {
            free_log.borrow_mut().extend([2, a[1], a[2]]);
            Ret::default()
        });
        log
    }

    /// Without bit 0 only the base destructor runs; with it the block is also freed with
    /// its stored size. Either way `this` is returned.
    fn check(address: u32, base: u32) {
        let mut e = engine();
        let log = install(&mut e, base);
        let this = e.mem.alloc(0x40);
        e.mem.set_u16(this + 4, 0x1234);
        let got = e.call(address, &args![this, 0xffff_fffeu32]).u32();
        assert_eq!(got, this);
        assert_eq!(*log.borrow(), vec![1, this]);
        log.borrow_mut().clear();
        let got = e.call(address, &args![this, 1u32]).u32();
        assert_eq!(got, this);
        assert_eq!(*log.borrow(), vec![1, this, 2, this, 0x1234]);
    }

    #[test]
    fn deleting_destructor_00d6a1f0_frees_only_when_bit_zero_is_set() {
        check(0x00d6_a1f0, 0x00d6_a0a0);
    }
}

#[cfg(test)]
mod a07_thunk_tests {
    use crate::prelude::*;

    /// Calls `thunk` with `this = 0x2000` and one flags word, with a double
    /// over `target` that returns `0xdeadbeef`; checks that the double ran
    /// once, with `this` moved back by `back` and the flags passed on, and
    /// returns the thunk's `EAX`.
    fn check(thunk: u32, target: u32, back: u32) -> u32 {
        let mut e = Engine::new();
        e.register(target, |_, _| Ret {
            eax: 0xdead_beef,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        let result = e.call(thunk, &[0x2000u32, 0x1]);
        let log = e.call_log.take().unwrap();
        let calls: Vec<_> = log.iter().filter(|(a, _)| *a == target).collect();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].1, vec![0x2000 - back, 0x1]);
        result.u32()
    }

    #[test]
    fn thunk_00d6a1b0() {
        assert_eq!(check(0x00d6_a1b0, 0x00d6_a1f0, 0x8), 0xdead_beef);
    }

    #[test]
    fn thunk_00d6a1c0() {
        assert_eq!(check(0x00d6_a1c0, 0x00d6_a1f0, 0x28), 0xdead_beef);
    }

    #[test]
    fn thunk_00d6a1d0() {
        assert_eq!(check(0x00d6_a1d0, 0x00d6_a1f0, 0x2c), 0xdead_beef);
    }

    #[test]
    fn thunk_00d6a1e0() {
        assert_eq!(check(0x00d6_a1e0, 0x00d6_a1f0, 0x20), 0xdead_beef);
    }
}

#[cfg(test)]
mod midphase_viewer_tests {
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
    fn fn_00d6a1a0_answers_the_registered_process_id() {
        let mut e = Engine::new();
        e.map(0x0126_8000, 0x2000);
        e.set_global(CENTRE_OF_MASS_PROCESS_ID, 0x4242u32);
        assert_eq!(e.call(0x00d6_a1a0, &args![0x99u32]).u32(), 0x4242);
    }

    #[test]
    fn fn_00d6a220_and_00d6a250_pass_the_adjusted_listener() {
        let mut e = Engine::new();
        let log: Log = Rc::default();
        record(&mut e, &log, ADD_LISTENER);
        record(&mut e, &log, REMOVE_LISTENER);
        e.call(0x00d6_a220, &args![0x2020u32, 0x5000u32]);
        e.call(0x00d6_a220, &args![0x20u32, 0x5000u32]);
        e.call(0x00d6_a250, &args![0x2020u32, 0x5000u32]);
        e.call(0x00d6_a250, &args![0x20u32, 0x5000u32]);
        assert_eq!(
            *log.borrow(),
            vec![
                (ADD_LISTENER, vec![0x5000, 0x2028]),
                (ADD_LISTENER, vec![0x5000, 0]),
                (REMOVE_LISTENER, vec![0x5000, 0x2028]),
                (REMOVE_LISTENER, vec![0x5000, 0]),
            ]
        );
    }

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
    fn fn_00d6a280_stores_the_vtables_and_registers_with_each_world() {
        let mut e = Engine::new();
        let world = e.mem.alloc(0x80);
        let list = e.mem.alloc(12);
        for i in 0..3u32 {
            e.mem.set_u32(list + 4 * i, 0x1000 + i);
        }
        e.mem.set_u32(world + 0x70, 3);
        e.mem.set_u32(world + 0x6c, list);
        let (this, log) = viewer(&mut e, world);
        let got = e.call(0x00d6_a280, &args![this, 0x1234u32]).ptr::<()>();
        assert_eq!(got.addr(), this);
        assert_eq!(e.mem.u32(this), VTABLE_VIEWER);
        assert_eq!(e.mem.u32(this + 8), VTABLE_PROCESS);
        assert_eq!(e.mem.u32(this + 0x20), VTABLE_FILTER);
        assert_eq!(e.mem.u32(this + 0x28), VTABLE_LISTENER);
        assert_eq!(e.mem.u32(this + 0x34), 0x8000_0000);
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
    fn fn_00d6a280_without_a_world_or_worlds_registers_nothing() {
        let mut e = Engine::new();
        let (this, log) = viewer(&mut e, 0);
        e.call(0x00d6_a280, &args![this, 0u32]);
        assert_eq!(log.borrow().len(), 1);
        let mut e = Engine::new();
        let world = e.mem.alloc(0x80);
        let (this, log) = viewer(&mut e, world);
        e.call(0x00d6_a280, &args![this, 0u32]);
        assert_eq!(log.borrow().len(), 1);
    }
}

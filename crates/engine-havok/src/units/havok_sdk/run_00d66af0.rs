//! `Havok SDK/run_00d66af0` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The memory router (`0x00c8_5750`): its field `+0x10` is the allocator the block is freed through.
const MEMORY_ROUTER: u32 = 0x00c8_5750;

// Translated from 00d66b60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Deleting destructor: runs the base destructor (`0x00d6_6490`) on `this` and, when bit 0 of
/// `flags` is set, returns the block to the memory router's allocator (slot `8`) with the
/// 16-bit size kept at `this + 4`. Returns `this`. The class is unconfirmed.
pub fn deleting_destructor_00d66b60(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    e.call(0x00d6_6490, &args![this]);
    if flags & 1 != 0 {
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let size = u32::from(e.mem.u16(this.addr() + 4));
        e.vcall(allocator, 8, &args![this, size]);
    }
    this
}

// Translated from 00d66b20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by 0x20 (`SUB ECX,0x20`) and jumps to `00d66b60`. `00d66b60` (`RET 4`) has the shape of a deleting destructor: it tests bit 0 of the flags word and returns the object pointer in `EAX`. The return type is `u32` from that `EAX`; the parameter count is from the `RET 4`. The stack word is passed on unchanged.
pub fn fn_00d66b20(e: &mut Engine, this: Ptr, flags: u32) -> u32 {
    e.call(0x00d6_6b60, &args![this.addr().wrapping_sub(0x20), flags])
        .u32()
}

// Translated from 00d66b30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by 0x8 (`SUB ECX,0x8`) and jumps to `00d66b60`. `00d66b60` (`RET 4`) has the shape of a deleting destructor: it tests bit 0 of the flags word and returns the object pointer in `EAX`. The return type is `u32` from that `EAX`; the parameter count is from the `RET 4`. The stack word is passed on unchanged.
pub fn fn_00d66b30(e: &mut Engine, this: Ptr, flags: u32) -> u32 {
    e.call(0x00d6_6b60, &args![this.addr().wrapping_sub(0x8), flags])
        .u32()
}

// Translated from 00d66b40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by 0x28 (`SUB ECX,0x28`) and jumps to `00d66b60`. `00d66b60` (`RET 4`) has the shape of a deleting destructor: it tests bit 0 of the flags word and returns the object pointer in `EAX`. The return type is `u32` from that `EAX`; the parameter count is from the `RET 4`. The stack word is passed on unchanged.
pub fn fn_00d66b40(e: &mut Engine, this: Ptr, flags: u32) -> u32 {
    e.call(0x00d6_6b60, &args![this.addr().wrapping_sub(0x28), flags])
        .u32()
}

// Translated from 00d66b50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by 0x2c (`SUB ECX,0x2c`) and jumps to `00d66b60`. `00d66b60` (`RET 4`) has the shape of a deleting destructor: it tests bit 0 of the flags word and returns the object pointer in `EAX`. The return type is `u32` from that `EAX`; the parameter count is from the `RET 4`. The stack word is passed on unchanged.
pub fn fn_00d66b50(e: &mut Engine, this: Ptr, flags: u32) -> u32 {
    e.call(0x00d6_6b60, &args![this.addr().wrapping_sub(0x2c), flags])
        .u32()
}

/// The process factory object (`01269cd0`, a pointer), `hkProcessFactory::registerProcess` (Xbox
/// PDB, `00dd9380`, thiscall: the name, the creation function; answers the id), the process name
/// `"Phantoms"` (`010c41a4`) and the creation function `00d66770`.
const PROCESS_FACTORY: u32 = 0x0126_9cd0;
const REGISTER_PROCESS: u32 = 0x00dd_9380;
const PHANTOMS_NAME: u32 = 0x010c_41a4;
const PHANTOMS_CREATE: u32 = 0x00d6_6770;
/// The id the registration got (`01268ff0`, answered by `00d66b10`).
const PHANTOMS_PROCESS_ID: u32 = 0x0126_8ff0;
/// `hkpWorldViewerBase`'s constructor (Xbox PDB, `00ddccb0`, thiscall, one argument: the process
/// contexts) and the float the mouse picking viewer starts its `+0x30` field with (`01013974`,
/// 1000.0 in the exe).
const WORLD_VIEWER_CONSTRUCT: u32 = 0x00dd_ccb0;
const MOUSE_PICKING_SPRING: u32 = 0x0101_3974;
/// The vtable pointers of the mouse picking viewer (RTTI: `hkpMousePickingViewer`): `+0`, `+8`
/// and `+0x20`.
const VTABLE_VIEWER: u32 = 0x010d_1ce4;
const VTABLE_PROCESS: u32 = 0x010d_1cc4;
const VTABLE_FILTER: u32 = 0x010d_1cb4;

// Translated from 00d66af0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Registers the creation function `00d66770` as the process `"Phantoms"` with the process
/// factory at `01269cd0` (`hkProcessFactory::registerProcess`) and keeps the answered id at
/// `01268ff0`.
pub fn fn_00d66af0(e: &mut Engine) {
    let factory: u32 = e.global(PROCESS_FACTORY);
    let id = e
        .call(
            REGISTER_PROCESS,
            &args![factory, PHANTOMS_NAME, PHANTOMS_CREATE],
        )
        .u32();
    e.set_global(PHANTOMS_PROCESS_ID, id);
}

// Translated from 00d66b10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Answers the process id kept at `01268ff0`: slot 1 of the vtable `010d1bbc` (RTTI:
/// `hkpPhantomDisplayViewer`); `ECX` unused.
pub fn fn_00d66b10(e: &mut Engine) -> u32 {
    e.global(PHANTOMS_PROCESS_ID)
}

// Translated from 00d66b90 (decompiled, FalloutNV.exe 1.4.0.525)
/// The constructor of the mouse picking viewer (RTTI: `hkpMousePickingViewer`, vtables
/// `010d1ce4`, `010d1cc4` and `010d1cb4`): runs the `hkpWorldViewerBase` constructor
/// (`00ddccb0`, with the process `contexts`), loads the float at `01013974` and stores it at
/// `+0x30` (`FLD` and `FSTP`: a signalling NaN comes out quiet), zeroes `+0x28` and `+0x2c` and
/// stores the three vtable pointers (`+0`, `+8`, `+0x20`). Answers `this`.
pub fn fn_00d66b90(e: &mut Engine, this: Ptr, contexts: Ptr) -> Ptr {
    let at = this.addr();
    e.call(WORLD_VIEWER_CONSTRUCT, &args![this, contexts]);
    let spring = x87_quiet_bits(e.mem.u32(MOUSE_PICKING_SPRING));
    e.mem.set_u32(at.wrapping_add(0x30), spring);
    e.mem.set_u32(at.wrapping_add(0x28), 0);
    e.mem.set_u32(at.wrapping_add(0x2c), 0);
    e.mem.set_u32(at, VTABLE_VIEWER);
    e.mem.set_u32(at.wrapping_add(8), VTABLE_PROCESS);
    e.mem.set_u32(at.wrapping_add(0x20), VTABLE_FILTER);
    this
}

// Translated from 00d66bd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores `0x011b5ff0` through `first` and 3 through `second` (stdcall-like, `RET 8`; `ECX` is
/// the object, ignored; the class is the one of the vtable `010d1cc4`, RTTI:
/// `hkpMousePickingViewer`, slot 2).
pub fn fn_00d66bd0(e: &mut Engine, _this: Ptr, first: Ptr, second: Ptr) {
    e.mem.set_u32(first.addr(), 0x011b_5ff0);
    e.mem.set_u32(second.addr(), 3);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00d6_6af0, fn_00d66af0()),
        entry!(0x00d6_6b10, fn_00d66b10() -> u32),
        entry!(0x00d6_6b90, fn_00d66b90(Ptr, Ptr) -> Ptr),
        entry!(0x00d6_6bd0, fn_00d66bd0(Ptr, Ptr, Ptr)),
        entry!(0x00d6_6b60, deleting_destructor_00d66b60(Ptr, u8) -> Ptr),
        entry!(0x00d6_6b20, fn_00d66b20(Ptr, u32) -> u32),
        entry!(0x00d6_6b30, fn_00d66b30(Ptr, u32) -> u32),
        entry!(0x00d6_6b40, fn_00d66b40(Ptr, u32) -> u32),
        entry!(0x00d6_6b50, fn_00d66b50(Ptr, u32) -> u32),
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
    fn deleting_destructor_00d66b60_frees_only_when_bit_zero_is_set() {
        check(0x00d6_6b60, 0x00d6_6490);
    }
}

#[cfg(test)]
mod le81_thunk_tests {
    use crate::prelude::*;

    /// Calls `thunk` with `this = 0x2000` and `extra` stack words, with a
    /// double over `target` that returns `0xdeadbeef`; checks that the
    /// double ran once, with `this` moved back by `back` and `extra` passed on.
    fn check(thunk: u32, target: u32, back: u32, extra: &[u32]) -> Ret {
        let mut e = Engine::new();
        e.register(target, |_, _| Ret {
            eax: 0xdead_beef,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        let mut words = vec![0x2000u32];
        words.extend_from_slice(extra);
        let result = e.call(thunk, &words);
        let log = e.call_log.take().unwrap();
        let mut expected = vec![0x2000 - back];
        expected.extend_from_slice(extra);
        let calls: Vec<_> = log.iter().filter(|(a, _)| *a == target).collect();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].1, expected);
        result
    }

    #[test]
    fn le81_thunk_00d66b20() {
        assert_eq!(
            check(0x00d6_6b20, 0x00d6_6b60, 0x20, &[0x1]).u32(),
            0xdead_beef
        );
    }

    #[test]
    fn le81_thunk_00d66b30() {
        assert_eq!(
            check(0x00d6_6b30, 0x00d6_6b60, 0x8, &[0x1]).u32(),
            0xdead_beef
        );
    }

    #[test]
    fn le81_thunk_00d66b40() {
        assert_eq!(
            check(0x00d6_6b40, 0x00d6_6b60, 0x28, &[0x1]).u32(),
            0xdead_beef
        );
    }

    #[test]
    fn le81_thunk_00d66b50() {
        assert_eq!(
            check(0x00d6_6b50, 0x00d6_6b60, 0x2c, &[0x1]).u32(),
            0xdead_beef
        );
    }
}

#[cfg(test)]
mod mouse_picking_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    type Seen = Rc<RefCell<Vec<Vec<u32>>>>;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0126_8000, 0x2000);
        e.map(0x0100_0000, 0x0030_0000);
        e
    }

    #[test]
    fn fn_00d66af0_registers_the_phantoms_process_and_keeps_its_id() {
        let mut e = engine();
        e.set_global(PROCESS_FACTORY, 0x1111u32);
        let seen: Seen = Rc::default();
        let log = Rc::clone(&seen);
        e.register_double(REGISTER_PROCESS, move |_, a| {
            log.borrow_mut().push(a.to_vec());
            Ret {
                eax: 7,
                ..Ret::default()
            }
        });
        e.call(0x00d6_6af0, &args![]);
        assert_eq!(*seen.borrow(), vec![vec![0x1111, 0x010c_41a4, 0x00d6_6770]]);
        assert_eq!(e.global::<u32>(PHANTOMS_PROCESS_ID), 7);
        assert_eq!(e.call(0x00d6_6b10, &args![0x55u32]).u32(), 7);
    }

    #[test]
    fn fn_00d66b90_sets_up_the_mouse_picking_viewer() {
        let mut e = engine();
        let seen: Seen = Rc::default();
        let log = Rc::clone(&seen);
        e.register_double(WORLD_VIEWER_CONSTRUCT, move |e, a| {
            log.borrow_mut().push(a.to_vec());
            // The base constructor leaves things in the fields the viewer then sets.
            e.mem.set_u32(a[0] + 0x28, 0xdead);
            e.mem.set_u32(a[0] + 0x2c, 0xbeef);
            Ret::default()
        });
        e.mem.set_u32(MOUSE_PICKING_SPRING, 1000f32.to_bits());
        let this = e.mem.alloc(0x40);
        let got = e.call(0x00d6_6b90, &args![this, 0x1234u32]).ptr::<()>();
        assert_eq!(got.addr(), this);
        assert_eq!(*seen.borrow(), vec![vec![this, 0x1234]]);
        assert_eq!(e.mem.u32(this + 0x30), 1000f32.to_bits());
        assert_eq!(e.mem.u32(this + 0x28), 0);
        assert_eq!(e.mem.u32(this + 0x2c), 0);
        assert_eq!(e.mem.u32(this), VTABLE_VIEWER);
        assert_eq!(e.mem.u32(this + 8), VTABLE_PROCESS);
        assert_eq!(e.mem.u32(this + 0x20), VTABLE_FILTER);
        // A signalling NaN in the constant comes out quiet (FLD / FSTP).
        e.mem.set_u32(MOUSE_PICKING_SPRING, 0x7f80_0001);
        e.call(0x00d6_6b90, &args![this, 0u32]);
        assert_eq!(e.mem.u32(this + 0x30), 0x7fc0_0001);
    }

    #[test]
    fn fn_00d66bd0_stores_the_two_constants() {
        let mut e = Engine::new();
        let first = e.mem.alloc(4);
        let second = e.mem.alloc(4);
        e.call(0x00d6_6bd0, &args![0x99u32, first, second]);
        assert_eq!(e.mem.u32(first), 0x011b_5ff0);
        assert_eq!(e.mem.u32(second), 3);
    }
}

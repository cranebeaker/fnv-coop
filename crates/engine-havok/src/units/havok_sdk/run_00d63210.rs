//! `Havok SDK/run_00d63210` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The memory router (`0x00c8_5750`): its field `+0x10` is the allocator the block is freed through.
const MEMORY_ROUTER: u32 = 0x00c8_5750;

// Translated from 00d63250 (decompiled, FalloutNV.exe 1.4.0.525)
/// Deleting destructor: runs the base destructor (`0x00d6_2fe0`) on `this` and, when bit 0 of
/// `flags` is set, returns the block to the memory router's allocator (slot `8`) with the
/// 16-bit size kept at `this + 4`. Returns `this`. The class is unconfirmed.
pub fn deleting_destructor_00d63250(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    e.call(0x00d6_2fe0, &args![this]);
    if flags & 1 != 0 {
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let size = u32::from(e.mem.u16(this.addr() + 4));
        e.vcall(allocator, 8, &args![this, size]);
    }
    this
}

// Translated from 00d63230 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by 0x8 (`SUB ECX,0x8`) and jumps to `00d63250`. `00d63250` (`RET 4`) has the shape of a deleting destructor: it tests bit 0 of the flags word and returns the object pointer in `EAX`. The return type is `u32` from that `EAX`; the parameter count is from the `RET 4`. The stack word is passed on unchanged.
pub fn fn_00d63230(e: &mut Engine, this: Ptr, flags: u32) -> u32 {
    e.call(0x00d6_3250, &args![this.addr().wrapping_sub(0x8), flags])
        .u32()
}

// Translated from 00d63240 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by 0x20 (`SUB ECX,0x20`) and jumps to `00d63250`. `00d63250` (`RET 4`) has the shape of a deleting destructor: it tests bit 0 of the flags word and returns the object pointer in `EAX`. The return type is `u32` from that `EAX`; the parameter count is from the `RET 4`. The stack word is passed on unchanged.
pub fn fn_00d63240(e: &mut Engine, this: Ptr, flags: u32) -> u32 {
    e.call(0x00d6_3250, &args![this.addr().wrapping_sub(0x20), flags])
        .u32()
}

/// The two process ids the snapshot viewer answers (`01268fc0` when the flag byte is set,
/// `01268fc4` when it is clear).
const SNAPSHOT_PROCESS_ID_SET: u32 = 0x0126_8fc0;
const SNAPSHOT_PROCESS_ID_CLEAR: u32 = 0x0126_8fc4;

// Translated from 00d63210 (decompiled, FalloutNV.exe 1.4.0.525)
/// Answers one of two process ids by the byte at `this + 0x20`: `01268fc0` when it is not
/// zero, `01268fc4` when it is zero. Slot 1 of the vtable `010d1700` (RTTI:
/// `hkpWorldSnapshotViewer`) with `this` the sub-object at `+8` of the viewer, so the byte is the
/// flag the viewer's constructor keeps at `+0x28`.
pub fn fn_00d63210(e: &mut Engine, this: Ptr) -> u32 {
    if e.mem.u8(this.addr().wrapping_add(0x20)) != 0 {
        e.global(SNAPSHOT_PROCESS_ID_SET)
    } else {
        e.global(SNAPSHOT_PROCESS_ID_CLEAR)
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00d6_3210, fn_00d63210(Ptr) -> u32),
        entry!(0x00d6_3250, deleting_destructor_00d63250(Ptr, u8) -> Ptr),
        entry!(0x00d6_3230, fn_00d63230(Ptr, u32) -> u32),
        entry!(0x00d6_3240, fn_00d63240(Ptr, u32) -> u32),
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
    fn deleting_destructor_00d63250_frees_only_when_bit_zero_is_set() {
        check(0x00d6_3250, 0x00d6_2fe0);
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
    fn le81_thunk_00d63230() {
        assert_eq!(
            check(0x00d6_3230, 0x00d6_3250, 0x8, &[0x1]).u32(),
            0xdead_beef
        );
    }

    #[test]
    fn le81_thunk_00d63240() {
        assert_eq!(
            check(0x00d6_3240, 0x00d6_3250, 0x20, &[0x1]).u32(),
            0xdead_beef
        );
    }
}

#[cfg(test)]
mod snapshot_id_tests {
    use super::*;

    #[test]
    fn fn_00d63210_answers_the_id_chosen_by_the_flag_byte() {
        let mut e = Engine::new();
        e.map(0x0126_8000, 0x1000);
        e.set_global(SNAPSHOT_PROCESS_ID_SET, 0x11u32);
        e.set_global(SNAPSHOT_PROCESS_ID_CLEAR, 0x22u32);
        let this = e.mem.alloc(0x40);
        assert_eq!(e.call(0x00d6_3210, &args![this]).u32(), 0x22);
        e.mem.set_u8(this + 0x20, 1);
        assert_eq!(e.call(0x00d6_3210, &args![this]).u32(), 0x11);
        // Any non-zero byte counts.
        e.mem.set_u8(this + 0x20, 0x80);
        assert_eq!(e.call(0x00d6_3210, &args![this]).u32(), 0x11);
    }
}

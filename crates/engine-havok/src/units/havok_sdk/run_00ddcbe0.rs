//! `Havok SDK/run_00ddcbe0` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The memory router (`0x00c8_5750`): its field `+0x10` is the allocator the block is freed through.
const MEMORY_ROUTER: u32 = 0x00c8_5750;

// Translated from 00ddcc10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Deleting destructor: runs the base destructor (`0x00dd_cb50`) on `this` and, when bit 0 of
/// `flags` is set, returns the block to the memory router's allocator (slot `8`) with the
/// 16-bit size kept at `this + 4`. Returns `this`. The class is unconfirmed.
pub fn deleting_destructor_00ddcc10(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    e.call(0x00dd_cb50, &args![this]);
    if flags & 1 != 0 {
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let size = u32::from(e.mem.u16(this.addr() + 4));
        e.vcall(allocator, 8, &args![this, size]);
    }
    this
}

// Translated from 00ddcbf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by `0x20` (`SUB ECX,0x20`) and jumps to `00ddcc10`, which (`RET 4`) has the shape of a scalar deleting destructor: it reads bit 0 of the one stack word. The caller gets its `EAX`; the class is unconfirmed.
pub fn fn_00ddcbf0(e: &mut Engine, this: Ptr, flags: u32) -> u32 {
    e.call(0x00dd_cc10, &args![this.addr().wrapping_sub(0x20), flags])
        .u32()
}

// Translated from 00ddcc00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by `0x8` (`SUB ECX,0x8`) and jumps to `00ddcc10`, which (`RET 4`) has the shape of a scalar deleting destructor: it reads bit 0 of the one stack word. The caller gets its `EAX`; the class is unconfirmed.
pub fn fn_00ddcc00(e: &mut Engine, this: Ptr, flags: u32) -> u32 {
    e.call(0x00dd_cc10, &args![this.addr().wrapping_sub(0x8), flags])
        .u32()
}

/// The word at `01269cdc` (a global the exe keeps; its role is not confirmed).
const GLOBAL_WORD: u32 = 0x0126_9cdc;

// Translated from 00ddcbe0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (no arguments): answers the word at `01269cdc`.
pub fn fn_00ddcbe0(e: &mut Engine) -> u32 {
    e.global(GLOBAL_WORD)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00dd_cc10, deleting_destructor_00ddcc10(Ptr, u8) -> Ptr),
        entry!(0x00dd_cbf0, fn_00ddcbf0(Ptr, u32) -> u32),
        entry!(0x00dd_cc00, fn_00ddcc00(Ptr, u32) -> u32),
        entry!(0x00ddcbe0, fn_00ddcbe0() -> u32),
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
    fn deleting_destructor_00ddcc10_frees_only_when_bit_zero_is_set() {
        check(0x00dd_cc10, 0x00dd_cb50);
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
    fn thunk_00ddcbf0() {
        assert_eq!(check(0x00dd_cbf0, 0x00dd_cc10, 0x20), 0xdead_beef);
    }

    #[test]
    fn thunk_00ddcc00() {
        assert_eq!(check(0x00dd_cc00, 0x00dd_cc10, 0x8), 0xdead_beef);
    }
}

#[cfg(test)]
mod a07_global_tests {
    use super::*;

    #[test]
    fn the_global_word_is_answered() {
        let mut e = Engine::new();
        e.map(GLOBAL_WORD, 4);
        e.set_global(GLOBAL_WORD, 0xbeef_u32);
        assert_eq!(e.call(0x00dd_cbe0, &args![]).u32(), 0xbeef);
    }
}

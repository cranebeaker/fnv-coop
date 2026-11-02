//! `Havok SDK/run_00d0bb20` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The memory router (`0x00c8_5750`): its field `+0x10` is the allocator the block is freed through.
const MEMORY_ROUTER: u32 = 0x00c8_5750;

// Translated from 00d0bbf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Deleting destructor: runs the base destructor (`0x00d0_bb20`) on `this` and, when bit 0 of
/// `flags` is set, returns the block to the memory router's allocator (slot `8`) with the
/// 16-bit size kept at `this + 4`. Returns `this`. The class is unconfirmed.
pub fn deleting_destructor_00d0bbf0(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    e.call(0x00d0_bb20, &args![this]);
    if flags & 1 != 0 {
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let size = u32::from(e.mem.u16(this.addr() + 4));
        e.vcall(allocator, 8, &args![this, size]);
    }
    this
}

// Translated from 00d0bbc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by 0x8 (`SUB ECX,0x8`) and jumps to `00d0bbf0`. `00d0bbf0` (`RET 4`) has the shape of a deleting destructor: it tests bit 0 of the flags word and returns the object pointer in `EAX`. The return type is `u32` from that `EAX`; the parameter count is from the `RET 4`. The stack word is passed on unchanged.
pub fn fn_00d0bbc0(e: &mut Engine, this: Ptr, flags: u32) -> u32 {
    e.call(0x00d0_bbf0, &args![this.addr().wrapping_sub(0x8), flags])
        .u32()
}

// Translated from 00d0bbd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by 0xc (`SUB ECX,0xc`) and jumps to `00d0bbf0`. `00d0bbf0` (`RET 4`) has the shape of a deleting destructor: it tests bit 0 of the flags word and returns the object pointer in `EAX`. The return type is `u32` from that `EAX`; the parameter count is from the `RET 4`. The stack word is passed on unchanged.
pub fn fn_00d0bbd0(e: &mut Engine, this: Ptr, flags: u32) -> u32 {
    e.call(0x00d0_bbf0, &args![this.addr().wrapping_sub(0xc), flags])
        .u32()
}

// Translated from 00d0bbe0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by 0x10 (`SUB ECX,0x10`) and jumps to `00d0bbf0`. `00d0bbf0` (`RET 4`) has the shape of a deleting destructor: it tests bit 0 of the flags word and returns the object pointer in `EAX`. The return type is `u32` from that `EAX`; the parameter count is from the `RET 4`. The stack word is passed on unchanged.
pub fn fn_00d0bbe0(e: &mut Engine, this: Ptr, flags: u32) -> u32 {
    e.call(0x00d0_bbf0, &args![this.addr().wrapping_sub(0x10), flags])
        .u32()
}

// Translated from 00d0bb20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor (`__fastcall`, class unconfirmed): writes the vtable words `+0`, `+8`, `+0xc`,
/// `+0x10`; for each of the six words from `+0x18`, when it is not null calls `00c905b0` on it and
/// clears it; then sets the size at `+0x3c` to 0 and, when the capacity word at `+0x40` has its
/// sign bit clear, frees the array at `+0x38` through slot `0x10` of the memory router's allocator
/// (`(capacity & 0x3fffffff) * 4` bytes); clears `+0x38`, sets `+0x40` to `0x80000000` and writes
/// the base vtable words (`+0x10`, `+0xc`, `+8`, `+0`).
pub fn fn_00d0bb20(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base, 0x010c_d3c4);
    e.mem.set_u32(base + 8, 0x010c_d3b4);
    e.mem.set_u32(base + 0xc, 0x010c_d3a4);
    e.mem.set_u32(base + 0x10, 0x010c_d394);
    for slot in 0..6u32 {
        let at = base + 0x18 + slot * 4;
        let item = e.mem.u32(at);
        if item != 0 {
            e.call(0x00c9_05b0, &args![item]);
            e.mem.set_u32(at, 0);
        }
    }
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let capacity = e.mem.u32(base + 0x40);
    e.mem.set_u32(base + 0x3c, 0);
    if (capacity as i32) >= 0 {
        let data = e.mem.u32(base + 0x38);
        let bytes = (capacity & 0x3fff_ffff) * 4;
        e.vcall(allocator, 0x10, &args![data, bytes]);
    }
    e.mem.set_u32(base + 0x38, 0);
    e.mem.set_u32(base + 0x40, 0x8000_0000);
    e.mem.set_u32(base + 0x10, 0x010c_cf7c);
    e.mem.set_u32(base + 0xc, 0x010c_925c);
    e.mem.set_u32(base + 8, 0x010c_b1c4);
    e.mem.set_u32(base, 0x0102_e388);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00d0_bbf0, deleting_destructor_00d0bbf0(Ptr, u8) -> Ptr),
        entry!(0x00d0_bbc0, fn_00d0bbc0(Ptr, u32) -> u32),
        entry!(0x00d0_bbd0, fn_00d0bbd0(Ptr, u32) -> u32),
        entry!(0x00d0_bbe0, fn_00d0bbe0(Ptr, u32) -> u32),
        entry!(0x00d0_bb20, fn_00d0bb20(Ptr)),
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
    fn deleting_destructor_00d0bbf0_frees_only_when_bit_zero_is_set() {
        check(0x00d0_bbf0, 0x00d0_bb20);
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
    fn le81_thunk_00d0bbc0() {
        assert_eq!(
            check(0x00d0_bbc0, 0x00d0_bbf0, 0x8, &[0x1]).u32(),
            0xdead_beef
        );
    }

    #[test]
    fn le81_thunk_00d0bbd0() {
        assert_eq!(
            check(0x00d0_bbd0, 0x00d0_bbf0, 0xc, &[0x1]).u32(),
            0xdead_beef
        );
    }

    #[test]
    fn le81_thunk_00d0bbe0() {
        assert_eq!(
            check(0x00d0_bbe0, 0x00d0_bbf0, 0x10, &[0x1]).u32(),
            0xdead_beef
        );
    }
}

#[cfg(test)]
mod havok_a04_tests {
    use crate::prelude::*;

    #[test]
    fn fn_00d0bb20_releases_the_items_and_the_array() {
        for (capacity, freed) in [(0x4000_0005u32, true), (0x8000_0005, false)] {
            let mut e = Engine::new();
            let vtable = e.mem.alloc(0x20);
            e.mem.set_u32(vtable + 0x10, 0x0900_0011);
            let allocator = e.mem.alloc(0x10);
            e.mem.set_u32(allocator, vtable);
            let router = e.mem.alloc(0x20);
            e.mem.set_u32(router + 0x10, allocator);
            e.register_double(0x00c8_5750, move |_, _| Ret {
                eax: router,
                ..Ret::default()
            });
            e.register(0x00c9_05b0, |_, _| Ret::default());
            e.register(0x0900_0011, |_, _| Ret::default());
            let this = e.mem.alloc(0x50);
            e.mem.set_u32(this + 0x18, 0x1111);
            e.mem.set_u32(this + 0x24, 0x2222);
            e.mem.set_u32(this + 0x28, 0x3333);
            e.mem.set_u32(this + 0x38, 0x7000);
            e.mem.set_u32(this + 0x3c, 9);
            e.mem.set_u32(this + 0x40, capacity);
            e.call_log = Some(vec![]);
            e.call(0x00d0_bb20, &args![this]);
            let log = e.call_log.take().unwrap();
            let mut expected = vec![
                (0x00c9_05b0, vec![0x1111]),
                (0x00c9_05b0, vec![0x2222]),
                (0x00c9_05b0, vec![0x3333]),
                (0x00c8_5750, vec![]),
            ];
            if freed {
                expected.push((0x0900_0011, vec![allocator, 0x7000, 20]));
            }
            assert_eq!(log[1..], expected[..]);
            for slot in 0..6 {
                assert_eq!(e.mem.u32(this + 0x18 + slot * 4), 0);
            }
            assert_eq!(e.mem.u32(this + 0x38), 0);
            assert_eq!(e.mem.u32(this + 0x3c), 0);
            assert_eq!(e.mem.u32(this + 0x40), 0x8000_0000);
            assert_eq!(e.mem.u32(this), 0x0102_e388);
            assert_eq!(e.mem.u32(this + 8), 0x010c_b1c4);
            assert_eq!(e.mem.u32(this + 0xc), 0x010c_925c);
            assert_eq!(e.mem.u32(this + 0x10), 0x010c_cf7c);
        }
    }
}

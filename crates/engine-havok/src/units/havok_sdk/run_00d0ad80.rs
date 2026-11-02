//! `Havok SDK/run_00d0ad80` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The memory router (`0x00c8_5750`): its field `+0x10` is the allocator the block is freed through.
const MEMORY_ROUTER: u32 = 0x00c8_5750;

// Translated from 00d0adc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Deleting destructor: runs the base destructor (`0x00d0_a4f0`) on `this` and, when bit 0 of
/// `flags` is set, returns the block to the memory router's allocator (slot `8`) with the
/// 16-bit size kept at `this + 4`. Returns `this`. The class is unconfirmed.
pub fn deleting_destructor_00d0adc0(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    e.call(0x00d0_a4f0, &args![this]);
    if flags & 1 != 0 {
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let size = u32::from(e.mem.u16(this.addr() + 4));
        e.vcall(allocator, 8, &args![this, size]);
    }
    this
}

// Translated from 00d0ada0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by 0x14 (`SUB ECX,0x14`) and jumps to `00d0adc0`. `00d0adc0` (`RET 4`) has the shape of a deleting destructor: it tests bit 0 of the flags word and returns the object pointer in `EAX`. The return type is `u32` from that `EAX`; the parameter count is from the `RET 4`. The stack word is passed on unchanged.
pub fn fn_00d0ada0(e: &mut Engine, this: Ptr, flags: u32) -> u32 {
    e.call(0x00d0_adc0, &args![this.addr().wrapping_sub(0x14), flags])
        .u32()
}

// Translated from 00d0adb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by 0x18 (`SUB ECX,0x18`) and jumps to `00d0adc0`. `00d0adc0` (`RET 4`) has the shape of a deleting destructor: it tests bit 0 of the flags word and returns the object pointer in `EAX`. The return type is `u32` from that `EAX`; the parameter count is from the `RET 4`. The stack word is passed on unchanged.
pub fn fn_00d0adb0(e: &mut Engine, this: Ptr, flags: u32) -> u32 {
    e.call(0x00d0_adc0, &args![this.addr().wrapping_sub(0x18), flags])
        .u32()
}

/// `00d9e280` (`this` = the part at `+8`, one word), `00c91750` and `00c91a70` (`this` = the
/// argument object, one word each) and `00c905b0` (`this` only: removes a reference): the calls
/// `00d0ae30` makes.
const PART_CALL: u32 = 0x00d9_e280;
const OBJECT_CALL_FIRST: u32 = 0x00c9_1750;
const OBJECT_CALL_SECOND: u32 = 0x00c9_1a70;
const REMOVE_REFERENCE: u32 = 0x00c9_05b0;

// Translated from 00d0ad80 (decompiled, FalloutNV.exe 1.4.0.525)
/// A virtual method (vtable slot at `010cd288`): true when the byte at `+0x20` is 0.
pub fn fn_00d0ad80(e: &mut Engine, this: Ptr) -> bool {
    e.mem.u8(this.addr() + 0x20) == 0
}

// Translated from 00d0ad90 (decompiled, FalloutNV.exe 1.4.0.525)
/// A virtual method (vtable slot at `010cd28c`): the byte at `+0x20` becomes 1 when the low byte of
/// `flag` is 0 and 0 otherwise.
pub fn fn_00d0ad90(e: &mut Engine, this: Ptr, flag: u8) {
    e.mem.set_u8(this.addr() + 0x20, u8::from(flag == 0));
}

// Translated from 00d0ae30 (decompiled, FalloutNV.exe 1.4.0.525)
/// A virtual method of the second table of a multiply-inheriting object (vtable slot at `010cd310`;
/// `this` is the sub-object at `+0xc` of the object, the class is unconfirmed). With `owner =
/// this - 0xc` and `object` the stack word: `00d9e280` runs on `this + 8` with `object`; then
/// `00c91750` on `object` with `this - 4` (0 when `owner` is 0) and `00c91a70` on `object` with
/// `this` (0 when `owner` is 0); finally a reference is removed from `owner` (`00c905b0`).
pub fn fn_00d0ae30(e: &mut Engine, this: Ptr, object: u32) {
    let base = this.addr();
    e.call(PART_CALL, &args![base.wrapping_add(8), object]);
    let owner = base.wrapping_sub(0xc);
    let first = if owner != 0 { base.wrapping_sub(4) } else { 0 };
    e.call(OBJECT_CALL_FIRST, &args![object, first]);
    let second = if owner != 0 { base } else { 0 };
    e.call(OBJECT_CALL_SECOND, &args![object, second]);
    e.call(REMOVE_REFERENCE, &args![owner]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00d0_ad80, fn_00d0ad80(Ptr) -> bool),
        entry!(0x00d0_ad90, fn_00d0ad90(Ptr, u8)),
        entry!(0x00d0_ae30, fn_00d0ae30(Ptr, u32)),
        entry!(0x00d0_adc0, deleting_destructor_00d0adc0(Ptr, u8) -> Ptr),
        entry!(0x00d0_ada0, fn_00d0ada0(Ptr, u32) -> u32),
        entry!(0x00d0_adb0, fn_00d0adb0(Ptr, u32) -> u32),
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
    fn deleting_destructor_00d0adc0_frees_only_when_bit_zero_is_set() {
        check(0x00d0_adc0, 0x00d0_a4f0);
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
    fn le81_thunk_00d0ada0() {
        assert_eq!(
            check(0x00d0_ada0, 0x00d0_adc0, 0x14, &[0x1]).u32(),
            0xdead_beef
        );
    }

    #[test]
    fn le81_thunk_00d0adb0() {
        assert_eq!(
            check(0x00d0_adb0, 0x00d0_adc0, 0x18, &[0x1]).u32(),
            0xdead_beef
        );
    }
}

#[cfg(test)]
mod tests_00d0ad80 {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    /// The byte flag reads and writes as the exe does (only the low byte of the argument counts).
    #[test]
    fn flag_byte_methods() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x40);
        assert!(e.call(0x00d0_ad80, &args![this]).bool());
        e.call(0x00d0_ad90, &args![this, 0u32]);
        assert_eq!(e.mem.u8(this + 0x20), 1);
        assert!(!e.call(0x00d0_ad80, &args![this]).bool());
        e.call(0x00d0_ad90, &args![this, 0x100u32]);
        assert_eq!(e.mem.u8(this + 0x20), 1);
        e.call(0x00d0_ad90, &args![this, 5u32]);
        assert_eq!(e.mem.u8(this + 0x20), 0);
        assert!(e.call(0x00d0_ad80, &args![this]).bool());
    }

    /// The four calls run in order with the adjusted pointers.
    #[test]
    fn multiple_inheritance_method_adjusts_its_pointers() {
        let mut e = Engine::new();
        let log = Rc::new(RefCell::new(Vec::new()));
        for (address, name) in [
            (PART_CALL, "part"),
            (OBJECT_CALL_FIRST, "first"),
            (OBJECT_CALL_SECOND, "second"),
            (REMOVE_REFERENCE, "remove"),
        ] {
            let sink = Rc::clone(&log);
            e.register_double(address, move |_, a| {
                sink.borrow_mut().push((name, a.to_vec()));
                Ret::default()
            });
        }
        e.call(0x00d0_ae30, &args![0x1000u32, 0x77u32]);
        assert_eq!(
            *log.borrow(),
            vec![
                ("part", vec![0x1008, 0x77]),
                ("first", vec![0x77, 0xffc]),
                ("second", vec![0x77, 0x1000]),
                ("remove", vec![0xff4])
            ]
        );
        // With `this - 0xc` null the two pointers are null.
        log.borrow_mut().clear();
        e.call(0x00d0_ae30, &args![0xcu32, 0x77u32]);
        assert_eq!(
            *log.borrow(),
            vec![
                ("part", vec![0x14, 0x77]),
                ("first", vec![0x77, 0]),
                ("second", vec![0x77, 0]),
                ("remove", vec![0])
            ]
        );
    }
}

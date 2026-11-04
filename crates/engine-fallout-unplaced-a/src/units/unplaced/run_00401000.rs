//! `(unplaced)/run_00401000` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The CRT's `_byteswap_ulong` (`00ec3bad`, cdecl, one word).
const BYTESWAP_ULONG: u32 = 0x00ec_3bad;

// Translated from 00401080 (decompiled, FalloutNV.exe 1.4.0.525)
/// Reverses the byte order of the `u32` at `value` in place through the
/// CRT's `_byteswap_ulong` (`00ec3bad`, cdecl). `cdecl`; its callers push a
/// second word (a flag byte) that the body never reads. The engine map gives
/// it no name.
pub fn fn_00401080(e: &mut Engine, value: Ptr, _unused_1: u32) {
    let word = e.mem.u32(value.addr());
    let swapped = e.call(BYTESWAP_ULONG, &args![word]).u32();
    e.mem.set_u32(value.addr(), swapped);
}

// Translated from 00401050 (decompiled, FalloutNV.exe 1.4.0.525)
/// The compiler's "vector constructor iterator" (`__stdcall`, four words):
/// calls the `__thiscall` constructor at `constructor` once for each of
/// `count` elements of `size` bytes, starting at `array`. A zero or negative
/// `count` constructs nothing. The constructor's result is ignored.
pub fn fn_00401050(e: &mut Engine, array: u32, size: u32, count: i32, constructor: u32) {
    let mut element = array;
    let mut remaining = count;
    loop {
        remaining = remaining.wrapping_sub(1);
        if remaining < 0 {
            break;
        }
        e.call(constructor, &args![element]);
        element = element.wrapping_add(size);
    }
}

// Translated from 00401170 (decompiled, FalloutNV.exe 1.4.0.525)
/// Returns the byte at `+4` of `this` (a plain field getter, zero-extended
/// into `EAX`).
pub fn fn_00401170(e: &mut Engine, this: Ptr) -> u8 {
    e.mem.u8(this.addr() + 4)
}

// Translated from 004013e0 (PC disassembly, FalloutNV.exe 1.4.0.525).
/// Tests mask 8 in the word at +8, returning only the boolean in AL.
/// No Xbox PDB name is mapped to this folded field predicate.
pub fn fn_004013e0(e: &mut Engine, this: Ptr) -> bool {
    e.mem.u32(this.addr().wrapping_add(8)) & 8 != 0
}

// Translated from 004010b0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 004010b0 - the exception-unwinding frame (FS:[0]) and the security-cookie bookkeeping are not translated.
/// Constructor of the form `CreateFormOfType` (`00465110`) makes for form type 0x76 (0x20 bytes); `ECX` is the object.
/// The engine map gives it no name; the RTTI name of the vtable it stores at `+0` (`0x0101_144c`) is `BGSDehydrationStage`, so this is the constructor of `BGSDehydrationStage`.
/// It runs the base and component constructors by address, stores the vtable pointers and sets the fields to their start values.
/// The exception-unwinding frame is not translated.
pub fn fn_004010b0(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(0x0048_3370, &args![this]);
    e.mem.set_u32(base, 0x0101_144c);
    e.call(0x0040_19f0, &args![base + 0x18]);
    e.call(0x004f_15a0, &args![this, 0x76u32]);
    e.call(0x0040_1b20, &args![this]);
    this
}

// Translated from 00401150 (decompiled, FalloutNV.exe 1.4.0.525)
/// A forwarder: it hands `this` (`ECX`) to the byte-getter `00401170` of this unit (reads
/// the byte at `+4`), and returns its result (a `u8`). Parameters: one word, `this`; the
/// disassembly of the function ends in a plain `RET` and passes only `ECX` on. The return
/// type (`u8`, AL) is that of `00401170`'s translated signature `fn_00401170(Ptr) -> u8`;
/// the wrapper's disassembly (`CALL 00401170` then straight to `RET`) leaves EAX untouched.
pub fn fn_00401150(e: &mut Engine, this: Ptr) -> u8 {
    fn_00401170(e, this)
}

/// Whether the word at +8 of `this` has any bit of `mask` (`SETNZ AL`; the upper bytes of
/// `EAX` keep the masked word's, which callers do not read).
fn word_eight_has(e: &mut Engine, this: Ptr, mask: u32) -> bool {
    e.mem.u32(this.addr().wrapping_add(8)) & mask != 0
}

// Translated from 00401190 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`ECX` only, plain `RET`): tests mask 0x400 in the word at +8.
pub fn fn_00401190(e: &mut Engine, this: Ptr) -> bool {
    word_eight_has(e, this, 0x400)
}

// Translated from 004011b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`ECX` only, plain `RET`): tests mask 0x10000 in the word at
/// +8.
pub fn fn_004011b0(e: &mut Engine, this: Ptr) -> bool {
    word_eight_has(e, this, 0x0001_0000)
}

// Translated from 004011d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`ECX` only, plain `RET`): tests mask 0x20000 in the word at
/// +8.
pub fn fn_004011d0(e: &mut Engine, this: Ptr) -> bool {
    word_eight_has(e, this, 0x0002_0000)
}

// Translated from 004011f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`ECX` only, plain `RET`): tests mask 0x80000 in the word at
/// +8.
pub fn fn_004011f0(e: &mut Engine, this: Ptr) -> bool {
    word_eight_has(e, this, 0x0008_0000)
}

// Translated from 00401210 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`ECX` only, plain `RET`): tests mask 0x2000000 in the word at
/// +8.
pub fn fn_00401210(e: &mut Engine, this: Ptr) -> bool {
    word_eight_has(e, this, 0x0200_0000)
}

// Translated from 00401230 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`ECX` only, plain `RET`): tests mask 0x40000000 in the word at
/// +8.
pub fn fn_00401230(e: &mut Engine, this: Ptr) -> bool {
    word_eight_has(e, this, 0x4000_0000)
}

// Translated from 00401250 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`ECX` only, plain `RET`): tests mask 0x200 in the word at +8.
pub fn fn_00401250(e: &mut Engine, this: Ptr) -> bool {
    word_eight_has(e, this, 0x200)
}

// Translated from 00401130 (decompiled, FalloutNV.exe 1.4.0.525)
/// A form vtable slot shared by about a hundred form classes (slot +0x30 of
/// `BGSDehydrationStage`'s vtable `0101144c`, and the same slot of the others;
/// unnamed in the engine map; `ECX` and one stack word, `RET 4`): hands its
/// argument to the object's own vtable slot +0x20 and returns. The Xbox PDB's
/// `TESForm` puts `LoadEdit` at +0x30 and `Load` at +0x20; that the PC
/// layout matches is not confirmed here.
pub fn fn_00401130(e: &mut Engine, this: Ptr, argument: u32) {
    e.vcall(this.addr(), 0x20, &args![argument]);
}

// Translated from 00401270 (decompiled, FalloutNV.exe 1.4.0.525)
/// A form vtable slot shared by about ninety form classes (slot +0x124 of
/// `BGSDehydrationStage`'s vtable `0101144c`; unnamed in the engine map):
/// takes `ECX` and five stack words (`RET 0x14`), reads none of them and
/// answers false (`XOR AL,AL`).
pub fn fn_00401270(
    _e: &mut Engine,
    _this: Ptr,
    _unused_1: u32,
    _unused_2: u32,
    _unused_3: u32,
    _unused_4: u32,
    _unused_5: u32,
) -> bool {
    false
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0040_1130, fn_00401130(Ptr, u32)),
        entry!(0x0040_1270, fn_00401270(Ptr, u32, u32, u32, u32, u32) -> bool),
        entry!(0x0040_1150, fn_00401150(Ptr) -> u8),
        entry!(0x00401050, fn_00401050(u32, u32, i32, u32)),
        entry!(0x00401080, fn_00401080(Ptr, u32)),
        entry!(0x00401170, fn_00401170(Ptr) -> u8),
        entry!(0x004013e0, fn_004013e0(Ptr) -> bool),
        entry!(0x0040_10b0, fn_004010b0(Ptr) -> Ptr),
        entry!(0x0040_1190, fn_00401190(Ptr) -> bool),
        entry!(0x0040_11b0, fn_004011b0(Ptr) -> bool),
        entry!(0x0040_11d0, fn_004011d0(Ptr) -> bool),
        entry!(0x0040_11f0, fn_004011f0(Ptr) -> bool),
        entry!(0x0040_1210, fn_00401210(Ptr) -> bool),
        entry!(0x0040_1230, fn_00401230(Ptr) -> bool),
        entry!(0x0040_1250, fn_00401250(Ptr) -> bool),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    const CONSTRUCTOR: u32 = 0x0060_0000;

    fn constructed(count: i32) -> Vec<u32> {
        let mut e = Engine::new();
        let seen = std::rc::Rc::new(std::cell::RefCell::new(Vec::new()));
        let log = seen.clone();
        e.register_double(CONSTRUCTOR, move |_, a| {
            log.borrow_mut().push(a[0]);
            Ret::default()
        });
        e.call(0x0040_1050, &args![0x1000u32, 0x24u32, count, CONSTRUCTOR]);
        let result = seen.borrow().clone();
        result
    }

    #[test]
    fn constructs_each_element_in_turn() {
        assert_eq!(constructed(3), vec![0x1000, 0x1024, 0x1048]);
    }

    #[test]
    fn constructs_nothing_for_zero_or_negative_counts() {
        assert!(constructed(0).is_empty());
        assert!(constructed(-2).is_empty());
    }

    #[test]
    fn swaps_the_word_in_place() {
        let mut e = Engine::new();
        e.register(BYTESWAP_ULONG, |_, a| a[0].swap_bytes().into_ret());
        let block = e.mem.alloc(8);
        e.mem.set_u32(block, 0x1122_3344);
        e.mem.set_u32(block + 4, 0xaabb_ccdd);
        e.call_log = Some(vec![]);
        e.call(0x0040_1080, &args![block, 1u32]);
        assert_eq!(e.mem.u32(block), 0x4433_2211);
        assert_eq!(e.mem.u32(block + 4), 0xaabb_ccdd);
        let log = e.call_log.take().unwrap();
        let swaps: Vec<_> = log.iter().filter(|(a, _)| *a == BYTESWAP_ULONG).collect();
        assert_eq!(swaps.len(), 1);
        assert_eq!(swaps[0].1, vec![0x1122_3344]);
    }

    #[test]
    fn reads_the_byte_at_4() {
        let mut e = Engine::new();
        let object: Ptr = Ptr::new(e.mem.alloc(0x10));
        e.mem.set_u8(object.addr() + 4, 0x5a);
        // Neighbouring bytes are not part of the result.
        e.mem.set_u8(object.addr() + 5, 0xff);
        assert_eq!(e.call(0x0040_1170, &args![object]).u32(), 0x5a);
    }

    #[test]
    fn flag_predicate_matches_native_al_and_preserves_all_bytes() {
        // Thirty-six guarded native cases, unpacked exe SHA-256
        // 19406942e48724d797300c4ea6be9ac69a32f670b8c35db09279a2258422739f,
        // FCW 027f / MXCSR 1f80: AL follows bit 3, RET 0, no memory writes.
        let values = [0, 7, u32::MAX, !8u32]
            .into_iter()
            .chain((0..32).map(|bit| 1u32 << bit));
        let mut e = Engine::new();
        let block = e.mem.alloc(20);
        for value in values {
            e.mem.write(block, &[0xa5; 20]);
            e.mem.set_u32(block + 12, value);
            let before = e.mem.bytes(block, 20);
            assert_eq!(e.call(0x004013e0, &[block + 4]).bool(), value & 8 != 0);
            assert_eq!(e.mem.bytes(block, 20), before);
        }
    }
}

#[cfg(test)]
mod ctor_tests {
    use crate::units::crt::ctor_test::{self, A};

    #[test]
    fn fn_004010b0_builds_the_object() {
        ctor_test::run(&ctor_test::Case {
            addr: 0x0040_10b0,
            size: 0x20,
            globals: &[],
            returns: &[],
            calls: &[
                (0x0048_3370, &[A::T(0x0)]),
                (0x0040_19f0, &[A::T(0x18)]),
                (0x004f_15a0, &[A::T(0x0), A::V(0x76)]),
                (0x0040_1b20, &[A::T(0x0)]),
            ],
            stores: &[(0x0, 4, 0x0101_144c)],
        });
    }
}

#[cfg(test)]
mod thunk_tests {
    use super::*;

    #[allow(dead_code)]
    fn calls_to(e: &Engine, callee: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == callee)
            .map(|(_, words)| words.clone())
            .collect()
    }

    #[test]
    fn fwd_a03_00401150_returns_the_byte_at_4_through_the_unit_getter() {
        let mut e = Engine::new();
        let object = e.mem.alloc(0x10);
        e.mem.set_u8(object + 4, 0x5a);
        // Neighbouring bytes are not part of the result.
        e.mem.set_u8(object + 5, 0xff);
        assert_eq!(e.call(0x0040_1150, &[object]).u32() & 0xff, 0x5a);
    }
}

#[cfg(test)]
mod flag_word_tests {
    use super::*;

    #[test]
    fn flag_getters_test_their_mask_in_the_word_at_eight() {
        let mut e = Engine::new();
        let form = e.mem.alloc(0x10);
        for (address, mask) in [
            (0x0040_1190u32, 0x400u32),
            (0x0040_11b0, 0x0001_0000),
            (0x0040_11d0, 0x0002_0000),
            (0x0040_11f0, 0x0008_0000),
            (0x0040_1210, 0x0200_0000),
            (0x0040_1230, 0x4000_0000),
            (0x0040_1250, 0x200),
        ] {
            e.mem.set_u32(form + 8, mask);
            assert!(e.call(address, &args![form]).bool(), "{address:08x}");
            e.mem.set_u32(form + 8, !mask);
            assert!(!e.call(address, &args![form]).bool(), "{address:08x}");
            // Only the word at +8 counts.
            e.mem.set_u32(form + 4, mask);
            e.mem.set_u32(form + 0xc, mask);
            assert!(!e.call(address, &args![form]).bool(), "{address:08x}");
            e.mem.set_u32(form + 4, 0);
            e.mem.set_u32(form + 0xc, 0);
        }
    }
}

#[cfg(test)]
mod n075_tests {
    use super::*;

    #[test]
    fn fn_00401130_forwards_its_word_to_slot_0x20() {
        let mut e = Engine::new();
        e.register(0x0060_0020, |_, a| {
            assert_eq!(a.len(), 2);
            Ret::default()
        });
        e.put_vtable(
            0x0070_0000,
            &[0; 8]
                .iter()
                .copied()
                .chain([0x0060_0020])
                .collect::<Vec<_>>(),
        );
        let object = e.mem.alloc(8);
        e.mem.set_u32(object, 0x0070_0000);
        e.call_log = Some(vec![]);
        e.call(0x0040_1130, &args![object, 0x55u32]);
        let log = e.call_log.take().unwrap();
        let slot: Vec<_> = log.iter().filter(|(a, _)| *a == 0x0060_0020).collect();
        assert_eq!(slot.len(), 1);
        assert_eq!(slot[0].1, vec![object, 0x55]);
    }

    #[test]
    fn fn_00401270_answers_false() {
        let mut e = Engine::new();
        let r = e.call(0x0040_1270, &args![0x1000u32, 1u32, 2u32, 3u32, 4u32, 5u32]);
        assert!(!r.bool());
    }
}

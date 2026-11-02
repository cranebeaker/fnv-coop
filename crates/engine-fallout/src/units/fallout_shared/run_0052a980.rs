//! `fallout shared/run_0052a980` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The vtable `0x0102_d5c4` the constructor `0052aaa0` and the destructor
//! `0052ab80` store has the RTTI name `TESClimate` (complete object locator
//! at `0x0102_d5c0`, type descriptor `.?AVTESClimate@@`).

#[allow(unused_imports)]
use crate::prelude::*;

/// `sprintf_s`-style formatter (`00406d00`, cdecl: buffer, size, format, ...).
const FORMAT: u32 = 0x0040_6d00;
/// `"  Temp"`.
const TEMP_TEXT: u32 = 0x0102_d5a8;
/// The vtable of `TESClimate` (RTTI).
const CLIMATE_VTABLE: u32 = 0x0102_d5c4;
/// The game's allocator (`00401000`, cdecl size).
const ALLOCATE: u32 = 0x0040_1000;
/// The game's deallocator (`00401030`, cdecl pointer).
const DEALLOCATE: u32 = 0x0040_1030;

// Translated from 0052a980 (decompiled, FalloutNV.exe 1.4.0.525)
/// Writes `"  Temp"` into `buffer` (0x800 bytes) through the formatter
/// `00406d00`. `this` is not read. The function leaves the formatter's
/// `EAX` as its own result.
pub fn fn_0052a980(e: &mut Engine, _this: Ptr, buffer: u32) -> u32 {
    e.call(FORMAT, &args![buffer, 0x800u32, TEMP_TEXT]).u32()
}

// Translated from 0052a9b0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0052a9b0 - the exception-unwinding frame (FS:[0]) around the node allocation is not translated.
/// Appends `item` to a singly linked list whose nodes hold the item at `+0`
/// and the next node at `+4` (`this` is the first node), unless
/// `00822510` (called on `item` with 0) answers true. It walks to the last
/// node; when that node holds an item (`0052aa80`) it allocates an 8-byte
/// node, builds it with `006317f0(node, item)` and links it after the last
/// one (a failed allocation links null); otherwise it stores the item in
/// the last node through `006e5cc0`.
pub fn fn_0052a9b0(e: &mut Engine, this: Ptr, item: u32) {
    if e.call(0x0082_2510, &args![item, 0u32]).bool() {
        return;
    }
    let mut last = this.addr();
    while e.mem.u32(last + 4) != 0 {
        last = e.mem.u32(last + 4);
    }
    if fn_0052aa80(e, Ptr::new(last), 0) {
        let node = e.call(ALLOCATE, &args![8u32]).u32();
        let built = if node == 0 {
            0
        } else {
            e.call(0x0063_17f0, &args![node, item]).u32()
        };
        e.mem.set_u32(last + 4, built);
    } else {
        e.call(0x006e_5cc0, &args![last, item]);
    }
}

// Translated from 0052aa80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether the word at `this` differs from `value`.
pub fn fn_0052aa80(e: &mut Engine, this: Ptr, value: u32) -> bool {
    e.mem.u32(this.addr()) != value
}

// Translated from 0052aaa0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0052aaa0 - the exception-unwinding frame (FS:[0]) and the security-cookie bookkeeping are not translated.
/// Constructor of the form `CreateFormOfType` (`00465110`) makes for form type 0x36 (0x58 bytes); `ECX` is the object.
/// The vtable it stores at `+0` (`0x0102_d5c4`) has the RTTI name `TESClimate`.
/// It runs the base and component constructors by address, stores the vtable pointers and sets the fields to their start values.
/// The exception-unwinding frame is not translated.
pub fn fn_0052aaa0(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(0x0048_3370, &args![this]);
    e.mem.set_u32(base, CLIMATE_VTABLE);
    e.call(0x0048_8f50, &args![base + 0x18]);
    e.call(0x0058_22c0, &args![base + 0x30]);
    e.call(
        0x00ec_782f,
        &args![base + 0x38, 0xc, 0x2, 0x0048_e270, 0x0048_e2e0],
    );
    e.call(0x004f_15a0, &args![this, 0x36]);
    e.call(0x0052_ac20, &args![this]);
    this
}

// Translated from 0052ab50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor of `TESClimate`: the destructor `0052ab80`,
/// then frees the object when bit 0 of `flags` is set. Returns `this`.
pub fn fn_0052ab50(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_0052ab80(e, this);
    if flags & 1 != 0 {
        e.call(DEALLOCATE, &args![this]);
    }
    this
}

// Translated from 0052ab80 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0052ab80 - the exception-unwinding frame (FS:[0]) and the security-cookie bookkeeping are not translated.
/// Destructor of `TESClimate` (the vtable it stores has that RTTI name):
/// stores the vtable, runs `00483710` on the object, destroys the two
/// 12-byte elements at `+0x38` (`00ec5fce` with `0048e2e0`) and the
/// components at `+0x30` (`00582320`) and `+0x18` (`00489070`), then the
/// `TESForm` base destructor (`00483630`).
pub fn fn_0052ab80(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base, CLIMATE_VTABLE);
    e.call(0x0048_3710, &args![this]);
    e.call(
        0x00ec_5fce,
        &args![base + 0x38, 0xcu32, 0x2u32, 0x0048_e2e0u32],
    );
    e.call(0x0058_2320, &args![base + 0x30]);
    e.call(0x0048_9070, &args![base + 0x18]);
    e.call(0x0048_3630, &args![this]);
}

// Translated from 0052ac20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Clears the 6 bytes at `+0x50` (`00403d30` with 0) and then sets the low
/// six bits of the byte at `+0x55` to 3 (`0052ac50`).
pub fn fn_0052ac20(e: &mut Engine, this: Ptr) {
    e.call(0x0040_3d30, &args![this.addr() + 0x50, 0u32, 6u32]);
    fn_0052ac50(e, this, 3);
}

// Translated from 0052ac50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Replaces the low six bits of the byte at `+0x55`: keeps its top two
/// bits and ORs in the low byte of `004a8f20(value, 0xffffff3f)`.
pub fn fn_0052ac50(e: &mut Engine, this: Ptr, value: u32) {
    let kept = u32::from(e.mem.u8(this.addr() + 0x55)) & 0xc0;
    let bits = e.call(0x004a_8f20, &args![value, 0xffff_ff3fu32]).u32();
    e.mem.set_u8(this.addr() + 0x55, (kept | bits) as u8);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0052_a980, fn_0052a980(Ptr, u32) -> u32),
        entry!(0x0052_a9b0, fn_0052a9b0(Ptr, u32)),
        entry!(0x0052_aa80, fn_0052aa80(Ptr, u32) -> bool),
        entry!(0x0052_aaa0, fn_0052aaa0(Ptr) -> Ptr),
        entry!(0x0052_ab50, fn_0052ab50(Ptr, u32) -> Ptr),
        entry!(0x0052_ab80, fn_0052ab80(Ptr)),
        entry!(0x0052_ac20, fn_0052ac20(Ptr)),
        entry!(0x0052_ac50, fn_0052ac50(Ptr, u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::crt::ctor_test::{self, A};

    fn noop(_: &mut Engine, _: &[u32]) -> Ret {
        Ret::default()
    }

    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    #[test]
    fn fn_0052a980_formats_temp_into_the_buffer() {
        let mut e = Engine::new();
        e.register(FORMAT, |_, _| 0x55u32.into_ret());
        e.call_log = Some(vec![]);
        let r = e.call(0x0052_a980, &args![0x1234u32, 0x9000u32]).u32();
        assert_eq!(r, 0x55);
        assert_eq!(calls_to(&e, FORMAT), vec![vec![0x9000, 0x800, TEMP_TEXT]]);
    }

    /// A chain of nodes (item, next) and the doubles the append calls.
    fn list_engine(refuse: bool) -> Engine {
        let mut e = Engine::new();
        if refuse {
            e.register(0x0082_2510, |_, _| 1u32.into_ret());
        } else {
            e.register(0x0082_2510, |_, _| 0u32.into_ret());
        }
        e.register(ALLOCATE, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(0x0063_17f0, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            e.mem.set_u32(a[0] + 4, 0);
            a[0].into_ret()
        });
        e.register(0x006e_5cc0, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            Ret::default()
        });
        e
    }

    #[test]
    fn fn_0052a9b0_appends_a_node_after_a_full_last_node() {
        let mut e = list_engine(false);
        let first = e.mem.alloc(8);
        let second = e.mem.alloc(8);
        e.mem.set_u32(first, 0x11);
        e.mem.set_u32(first + 4, second);
        e.mem.set_u32(second, 0x22);
        e.mem.set_u32(second + 4, 0);
        e.call_log = Some(vec![]);
        e.call(0x0052_a9b0, &args![first, 0x33u32]);
        assert_eq!(calls_to(&e, 0x0082_2510), vec![vec![0x33, 0]]);
        assert_eq!(calls_to(&e, ALLOCATE), vec![vec![8]]);
        let node = e.mem.u32(second + 4);
        assert_ne!(node, 0);
        assert_eq!(e.mem.u32(node), 0x33);
        assert_eq!(e.mem.u32(first + 4), second);
    }

    #[test]
    fn fn_0052a9b0_fills_an_empty_last_node() {
        let mut e = list_engine(false);
        let first = e.mem.alloc(8);
        e.call_log = Some(vec![]);
        e.call(0x0052_a9b0, &args![first, 0x44u32]);
        assert_eq!(calls_to(&e, 0x006e_5cc0), vec![vec![first, 0x44]]);
        assert!(calls_to(&e, ALLOCATE).is_empty());
        assert_eq!(e.mem.u32(first), 0x44);
    }

    #[test]
    fn fn_0052a9b0_links_null_when_allocation_fails() {
        let mut e = list_engine(false);
        e.register(ALLOCATE, |_, _| 0u32.into_ret());
        let first = e.mem.alloc(8);
        e.mem.set_u32(first, 0x11);
        e.mem.set_u32(first + 4, 0);
        e.call_log = Some(vec![]);
        e.call(0x0052_a9b0, &args![first, 0x33u32]);
        assert!(calls_to(&e, 0x0063_17f0).is_empty());
        assert_eq!(e.mem.u32(first + 4), 0);
    }

    #[test]
    fn fn_0052a9b0_does_nothing_when_the_item_check_answers_true() {
        let mut e = list_engine(true);
        let first = e.mem.alloc(8);
        e.call_log = Some(vec![]);
        e.call(0x0052_a9b0, &args![first, 0x44u32]);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 2);
        assert_eq!(e.mem.u32(first), 0);
    }

    #[test]
    fn fn_0052aa80_compares_the_first_word() {
        let mut e = Engine::new();
        let p = e.mem.alloc(4);
        e.mem.set_u32(p, 7);
        assert!(!e.call(0x0052_aa80, &args![p, 7u32]).bool());
        assert!(e.call(0x0052_aa80, &args![p, 8u32]).bool());
    }

    #[test]
    fn fn_0052aaa0_builds_the_object() {
        ctor_test::run(&ctor_test::Case {
            addr: 0x0052_aaa0,
            size: 0x58,
            globals: &[],
            returns: &[],
            calls: &[
                (0x0048_3370, &[A::T(0x0)]),
                (0x0048_8f50, &[A::T(0x18)]),
                (0x0058_22c0, &[A::T(0x30)]),
                (
                    0x00ec_782f,
                    &[
                        A::T(0x38),
                        A::V(0xc),
                        A::V(0x2),
                        A::V(0x0048_e270),
                        A::V(0x0048_e2e0),
                    ],
                ),
                (0x004f_15a0, &[A::T(0x0), A::V(0x36)]),
                (0x0052_ac20, &[A::T(0x0)]),
            ],
            stores: &[(0x0, 4, 0x0102_d5c4)],
        });
    }

    fn destructor_engine() -> Engine {
        let mut e = Engine::new();
        for a in [
            0x0048_3710,
            0x00ec_5fce,
            0x0058_2320,
            0x0048_9070,
            0x0048_3630,
            DEALLOCATE,
        ] {
            e.register(a, noop);
        }
        e
    }

    #[test]
    fn fn_0052ab80_destroys_the_parts_in_order() {
        let mut e = destructor_engine();
        let this = e.mem.alloc(0x58);
        e.call_log = Some(vec![]);
        e.call(0x0052_ab80, &args![this]);
        let log = e.call_log.take().unwrap();
        let seen: Vec<(u32, Vec<u32>)> = log.into_iter().skip(1).collect();
        assert_eq!(
            seen,
            vec![
                (0x0048_3710, vec![this]),
                (0x00ec_5fce, vec![this + 0x38, 0xc, 2, 0x0048_e2e0]),
                (0x0058_2320, vec![this + 0x30]),
                (0x0048_9070, vec![this + 0x18]),
                (0x0048_3630, vec![this]),
            ]
        );
        assert_eq!(e.mem.u32(this), CLIMATE_VTABLE);
    }

    #[test]
    fn fn_0052ab50_frees_only_with_bit_0() {
        let mut e = destructor_engine();
        let this = e.mem.alloc(0x58);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0052_ab50, &args![this, 0u32]).u32(), this);
        assert!(calls_to(&e, DEALLOCATE).is_empty());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0052_ab50, &args![this, 3u32]).u32(), this);
        assert_eq!(calls_to(&e, DEALLOCATE), vec![vec![this]]);
        assert_eq!(calls_to(&e, 0x0048_3630), vec![vec![this]]);
    }

    #[test]
    fn fn_0052ac20_clears_and_sets_the_flag_bits() {
        let mut e = Engine::new();
        e.register(0x0040_3d30, noop);
        e.register(0x004a_8f20, |_, a| (a[0] & a[1]).into_ret());
        let this = e.mem.alloc(0x58);
        e.mem.set_u8(this + 0x55, 0xff);
        e.call_log = Some(vec![]);
        e.call(0x0052_ac20, &args![this]);
        assert_eq!(calls_to(&e, 0x0040_3d30), vec![vec![this + 0x50, 0, 6]]);
        assert_eq!(calls_to(&e, 0x004a_8f20), vec![vec![3, 0xffff_ff3f]]);
        assert_eq!(e.mem.u8(this + 0x55), 0xc3);
    }

    #[test]
    fn fn_0052ac50_keeps_the_top_bits() {
        let mut e = Engine::new();
        e.register(0x004a_8f20, |_, _| 0x1_0005u32.into_ret());
        let this = e.mem.alloc(0x58);
        e.mem.set_u8(this + 0x55, 0x7f);
        e.call(0x0052_ac50, &args![this, 9u32]);
        assert_eq!(e.mem.u8(this + 0x55), 0x45);
    }
}

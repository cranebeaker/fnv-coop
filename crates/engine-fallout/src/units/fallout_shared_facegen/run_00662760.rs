//! `fallout shared/facegen/run_00662760` (Xbox PDB source unit), subsystem `fallout shared/facegen`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The global `00662760` makes and `006627e0` destroys (`011d5b8c`).
const FACEGEN_GLOBAL_011D5B8C: u32 = 0x011d_5b8c;
/// `00401000` (`cdecl`, one word: the size): the game's `operator new`.
const OPERATOR_NEW: u32 = 0x0040_1000;
/// `00438170` (`thiscall`, one word: a C string): the 4-byte object's constructor.
const FOD_CONSTRUCT_00438170: u32 = 0x0043_8170;
/// `"FOD"`, the text the object is made from.
const FOD_TEXT: u32 = 0x0105_f3c0;
/// `004b9900` (`thiscall`, one word: the delete flags): the object's deleting destructor.
const DELETING_DESTRUCTOR_004B9900: u32 = 0x004b_9900;

// Translated from 00662760 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: no arguments. Allocates 4 bytes (`00401000`) and, when that
/// worked, constructs them from `"FOD"` (`00438170`, whose result is kept); stores the result
/// (or 0) at `011d5b8c`.
// Translation incomplete: 00662760 - the C++ exception frame (FS:[0] chain, unwind state 0 around the constructor) is not translated.
pub fn fn_00662760(e: &mut Engine) {
    let block = e.call(OPERATOR_NEW, &args![4u32]).u32();
    let object = if block != 0 {
        e.call(FOD_CONSTRUCT_00438170, &args![block, FOD_TEXT])
            .u32()
    } else {
        0
    };
    e.set_global(FACEGEN_GLOBAL_011D5B8C, object);
}

// Translated from 006627e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: no arguments. Deletes the object at `011d5b8c` when there is
/// one (`004b9900` with flags 1) and clears the global.
pub fn fn_006627e0(e: &mut Engine) {
    let object = e.global::<u32>(FACEGEN_GLOBAL_011D5B8C);
    if object != 0 {
        e.call(DELETING_DESTRUCTOR_004B9900, &args![object, 1u32]);
    }
    e.set_global(FACEGEN_GLOBAL_011D5B8C, 0u32);
}

// Translated from 00662820 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: `thiscall`, no stack words. The number of 4-byte elements
/// between the begin pointer at +0xC and the end pointer at +0x10 (`SAR` by 2: signed).
pub fn fn_00662820(e: &mut Engine, this: Ptr) -> i32 {
    let begin = e.mem.u32(this.addr().wrapping_add(0xc));
    let end = e.mem.u32(this.addr().wrapping_add(0x10));
    (end.wrapping_sub(begin) as i32) >> 2
}

// Translated from 00662840 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: `thiscall`, no stack words. The number of 0x34-byte elements
/// between the begin pointer at +0xC and the end pointer at +0x10 (`IDIV`: signed,
/// truncating).
pub fn fn_00662840(e: &mut Engine, this: Ptr) -> i32 {
    let begin = e.mem.u32(this.addr().wrapping_add(0xc));
    let end = e.mem.u32(this.addr().wrapping_add(0x10));
    (end.wrapping_sub(begin) as i32) / 0x34
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0066_2760, fn_00662760()),
        entry!(0x0066_27e0, fn_006627e0()),
        entry!(0x0066_2820, fn_00662820(Ptr) -> i32),
        entry!(0x0066_2840, fn_00662840(Ptr) -> i32),
    ]
}

#[cfg(test)]
mod b048_tests {
    use super::*;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    #[test]
    fn fn_00662760_makes_the_object_or_stores_zero() {
        let mut e = Engine::new();
        e.map(FACEGEN_GLOBAL_011D5B8C, 4);
        e.register(OPERATOR_NEW, |_, _| 0x9000u32.into_ret());
        e.register(FOD_CONSTRUCT_00438170, |_, a| (a[0] + 1).into_ret());
        e.call_log = Some(vec![]);
        e.call(0x0066_2760, &args![]);
        assert_eq!(e.global::<u32>(FACEGEN_GLOBAL_011D5B8C), 0x9001);
        assert_eq!(calls(&e, OPERATOR_NEW), vec![vec![4]]);
        assert_eq!(
            calls(&e, FOD_CONSTRUCT_00438170),
            vec![vec![0x9000, FOD_TEXT]]
        );
        e.register(OPERATOR_NEW, |_, _| 0u32.into_ret());
        e.call(0x0066_2760, &args![]);
        assert_eq!(e.global::<u32>(FACEGEN_GLOBAL_011D5B8C), 0);
        assert_eq!(calls(&e, FOD_CONSTRUCT_00438170).len(), 1);
    }

    #[test]
    fn fn_006627e0_deletes_a_set_object_and_clears_the_global() {
        let mut e = Engine::new();
        e.map(FACEGEN_GLOBAL_011D5B8C, 4);
        e.register(DELETING_DESTRUCTOR_004B9900, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0066_27e0, &args![]);
        assert!(calls(&e, DELETING_DESTRUCTOR_004B9900).is_empty());
        e.set_global(FACEGEN_GLOBAL_011D5B8C, 0x9000u32);
        e.call(0x0066_27e0, &args![]);
        assert_eq!(
            calls(&e, DELETING_DESTRUCTOR_004B9900),
            vec![vec![0x9000, 1]]
        );
        assert_eq!(e.global::<u32>(FACEGEN_GLOBAL_011D5B8C), 0);
    }

    #[test]
    fn element_counts_divide_the_span_signed() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x14);
        e.mem.set_u32(this + 0xc, 0x1000);
        e.mem.set_u32(this + 0x10, 0x1000 + 0x34 * 3);
        assert_eq!(e.call(0x0066_2820, &args![this]).i32(), 0x27);
        assert_eq!(e.call(0x0066_2840, &args![this]).i32(), 3);
        // End before begin: negative counts (SAR rounds down, IDIV toward zero).
        e.mem.set_u32(this + 0x10, 0x1000 - 0x35);
        assert_eq!(e.call(0x0066_2820, &args![this]).i32(), -14);
        assert_eq!(e.call(0x0066_2840, &args![this]).i32(), -1);
    }
}

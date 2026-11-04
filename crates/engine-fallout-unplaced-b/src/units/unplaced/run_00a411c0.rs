//! `(unplaced)/run_00a411c0` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `00aa13e0`: `operator new` (`cdecl`: size).
const OPERATOR_NEW: u32 = 0x00aa_13e0;
/// `NiExtraDataController`'s constructor (`00a572a0`; `thiscall`, no stack word).
const EXTRA_DATA_CONTROLLER_CONSTRUCTOR: u32 = 0x00a5_72a0;
/// The `NiFloatExtraDataController` virtual table (`010975fc`).
const FLOAT_EXTRA_DATA_CONTROLLER_VTABLE: u32 = 0x0109_75fc;
/// `NiBlendFloatInterpolator`'s constructor (`00a40520`; `thiscall`: manager-controlled
/// flag, weight threshold, array size).
const BLEND_FLOAT_INTERPOLATOR_CONSTRUCTOR: u32 = 0x00a4_0520;

// Translated from 00a411c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// A `CreateBlendInterpolator` used by many virtual tables (unnamed in the engine map;
/// `thiscall`, five words, `RET 0x14`; `this`, the index and the third word are not read):
/// allocates 0x34 bytes and constructs an `NiBlendFloatInterpolator` (`00a40520`) from the
/// second word, the threshold (fourth word, through the x87 stack) and the fifth word.
/// Answers the constructor's result, null when the allocation fails.
pub fn fn_00a411c0(
    e: &mut Engine,
    _this: u32,
    _unused_1: u32,
    manager_controlled: u32,
    _unused_3: u32,
    threshold: u32,
    array_size: u32,
) -> Ptr {
    let block = e.call(OPERATOR_NEW, &args![0x34u32]).u32();
    if block == 0 {
        return Ptr::new(0);
    }
    e.call(
        BLEND_FLOAT_INTERPOLATOR_CONSTRUCTOR,
        &args![
            block,
            manager_controlled,
            x87_quiet_bits(threshold),
            array_size
        ],
    )
    .ptr()
}

// Translated from 00a411f0 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// `NiFloatExtraDataController`'s create function (not a Ghidra function; the bytes at
/// `00a411f0`, plain `RET`): allocates 0x40 bytes and, when that succeeds, runs
/// `NiExtraDataController`'s constructor (`00a572a0`) and stores the
/// `NiFloatExtraDataController` virtual table (`010975fc`). Answers the object, null when
/// the allocation failed.
pub fn fn_00a411f0(e: &mut Engine) -> Ptr {
    let block = e.call(OPERATOR_NEW, &args![0x40u32]).u32();
    if block == 0 {
        return Ptr::new(0);
    }
    e.call(EXTRA_DATA_CONTROLLER_CONSTRUCTOR, &args![block]);
    e.mem.set_u32(block, FLOAT_EXTRA_DATA_CONTROLLER_VTABLE);
    Ptr::new(block)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a4_11c0, fn_00a411c0(u32, u32, u32, u32, u32, u32) -> Ptr),
        entry!(0x00a4_11f0, fn_00a411f0() -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn blend_interpolator_gets_the_flag_threshold_and_size() {
        let mut e = Engine::new();
        let block = e.mem.alloc(0x34);
        e.register_double(OPERATOR_NEW, move |_, a| {
            assert_eq!(a[0], 0x34);
            block.into_ret()
        });
        e.register(BLEND_FLOAT_INTERPOLATOR_CONSTRUCTOR, |_, a| a[0].into_ret());
        e.call_log = Some(vec![]);
        let answer = e
            .call(
                0x00a4_11c0,
                &args![0x100u32, 7u32, 1u32, 1u32, 0x7f80_0002u32, 4u32],
            )
            .u32();
        assert_eq!(answer, block);
        let log = e.call_log.take().unwrap();
        let built: Vec<_> = log
            .iter()
            .filter(|c| c.0 == BLEND_FLOAT_INTERPOLATOR_CONSTRUCTOR)
            .collect();
        assert_eq!(built[0].1, vec![block, 1, 0x7fc0_0002, 4]);
        e.register(OPERATOR_NEW, |_, _| Ret::default());
        assert_eq!(
            e.call(0x00a4_11c0, &args![0u32, 0u32, 0u32, 0u32, 0u32, 2u32])
                .u32(),
            0
        );
    }

    #[test]
    fn create_function_builds_the_controller() {
        let mut e = Engine::new();
        let block = e.mem.alloc(0x40);
        e.register_double(OPERATOR_NEW, move |_, a| {
            assert_eq!(a[0], 0x40);
            block.into_ret()
        });
        e.register(EXTRA_DATA_CONTROLLER_CONSTRUCTOR, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00a4_11f0, &args![]).u32(), block);
        assert_eq!(e.mem.u32(block), FLOAT_EXTRA_DATA_CONTROLLER_VTABLE);
        let log = e.call_log.take().unwrap();
        assert!(log
            .iter()
            .any(|c| c.0 == EXTRA_DATA_CONTROLLER_CONSTRUCTOR && c.1 == vec![block]));
        e.register(OPERATOR_NEW, |_, _| Ret::default());
        assert_eq!(e.call(0x00a4_11f0, &args![]).u32(), 0);
    }
}

//! `NiAnimation/run_00a40980` (Xbox PDB source unit), subsystem `NiAnimation`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `00a36f80`: the jump target of `00a40a00`.
const JUMP_TARGET_00A40A00: u32 = 0x00a3_6f80;

// Translated from 00a40a00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: a 5-byte jump to `00a36f80` (engine map: `NiBlendInterpolator::RegisterStreamables`), passing on `this` and its one argument; returns the target result. Return type (bool): the final target `00a36f80` is `NiBlendInterpolator::RegisterStreamables` (engine map), `bool` in the Xbox PDB, and its disassembly ends with AL set to 1.
pub fn fn_00a40a00(e: &mut Engine, this: Ptr, stream: Ptr) -> bool {
    e.call(JUMP_TARGET_00A40A00, &args![this, stream]).bool()
}

/// `00a36890` (`thiscall`): `NiBlendInterpolator`'s default constructor.
const BLEND_INTERPOLATOR_CONSTRUCT: u32 = 0x00a3_6890;
/// `NiBlendTransformInterpolator`'s vtable (by its RTTI type descriptor).
const NI_BLEND_TRANSFORM_INTERPOLATOR_VTABLE: u32 = 0x0109_750c;

// Translated from 00a409b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (by its body `NiBlendTransformInterpolator`'s default
/// constructor): `NiBlendInterpolator`'s default constructor (`00a36890`), then the vtable
/// `0109750c`. Answers `this`.
pub fn fn_00a409b0(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(BLEND_INTERPOLATOR_CONSTRUCT, &args![this]);
    e.mem
        .set_u32(this.addr(), NI_BLEND_TRANSFORM_INTERPOLATOR_VTABLE);
    this
}

/// `NiBlendInterpolator::NiBlendInterpolator` (engine map; `thiscall`, three words, the
/// second a float).
const BLEND_INTERPOLATOR_CONSTRUCT_WITH: u32 = 0x00a3_70b0;
/// `00aa13e0` (`cdecl`, the size): the Gamebryo allocation the objects are made with.
const NI_ALLOCATE: u32 = 0x00aa_13e0;

// Translated from 00a40980 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, three words, `RET 0xc`; by its body a
/// `NiBlendTransformInterpolator` constructor): `NiBlendInterpolator`'s constructor
/// (`00a370b0`) with the three words (the second, a float, through the x87 stack), then the
/// vtable `0109750c`. Answers `this`.
pub fn fn_00a40980(e: &mut Engine, this: Ptr, first: u32, second: f32, third: u32) -> Ptr {
    let second = x87_quiet_bits(second.to_bits());
    e.call(
        BLEND_INTERPOLATOR_CONSTRUCT_WITH,
        &args![this, first, second, third],
    );
    e.mem
        .set_u32(this.addr(), NI_BLEND_TRANSFORM_INTERPOLATOR_VTABLE);
    this
}

// Translated from 00a409d0 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (a function Ghidra did not create; no arguments; by its body
/// the creation function `NiBlendTransformInterpolator` registers): 0x30 bytes from
/// `00aa13e0`, built by `NiBlendInterpolator`'s default constructor (`00a36890`) with the
/// vtable `0109750c`. Answers the object, null when the allocation fails.
pub fn fn_00a409d0(e: &mut Engine) -> Ptr {
    let object = e.call(NI_ALLOCATE, &args![0x30u32]).u32();
    if object == 0 {
        return Ptr::NULL;
    }
    e.call(BLEND_INTERPOLATOR_CONSTRUCT, &args![object]);
    e.mem
        .set_u32(object, NI_BLEND_TRANSFORM_INTERPOLATOR_VTABLE);
    Ptr::new(object)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a4_0980, fn_00a40980(Ptr, u32, f32, u32) -> Ptr),
        entry!(0x00a4_09d0, fn_00a409d0() -> Ptr),
        entry!(0x00a4_0a00, fn_00a40a00(Ptr, Ptr) -> bool),
        entry!(0x00a4_09b0, fn_00a409b0(Ptr) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn thunk_00a40a00_forwards_its_words_and_returns_the_target_result() {
        const THUNK: u32 = 0x00a4_0a00;
        const TARGET: u32 = 0x00a3_6f80;
        let mut e = Engine::new();
        e.register(TARGET, |_, _| Ret {
            eax: 1,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        assert_eq!(e.call(THUNK, &args![0x1000u32, 0x2000u32]).u32(), 1);
        let log = e.call_log.take().unwrap();
        let hits: Vec<_> = log.iter().filter(|call| call.0 == TARGET).collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, args![0x1000u32, 0x2000u32]);
        e.register(TARGET, |_, _| Ret::default());
        assert_eq!(e.call(THUNK, &args![0x1000u32, 0x2000u32]).u32(), 0);
    }

    #[test]
    fn fn_00a409b0_constructs_the_base_and_stores_the_vtable() {
        let mut e = Engine::new();
        e.register(BLEND_INTERPOLATOR_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], 0x1234);
            e.mem.set_u32(a[0] + 4, 0x5678);
            a[0].into_ret()
        });
        let this = e.mem.alloc(0x40);
        assert_eq!(e.call(0x00a4_09b0, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), NI_BLEND_TRANSFORM_INTERPOLATOR_VTABLE);
        assert_eq!(e.mem.u32(this + 4), 0x5678);
    }
}

#[cfg(test)]
mod n007_tests {
    use super::*;

    #[test]
    fn constructor_with_words_passes_them_on() {
        let mut e = Engine::new();
        e.register(BLEND_INTERPOLATOR_CONSTRUCT_WITH, |_, a| a[0].into_ret());
        e.call_log = Some(vec![]);
        let this = e.mem.alloc(0x30);
        let r = e
            .call(0x00a4_0980, &args![this, 7u32, 0x7f80_0001u32, 9u32])
            .u32();
        assert_eq!(r, this);
        assert_eq!(e.mem.u32(this), NI_BLEND_TRANSFORM_INTERPOLATOR_VTABLE);
        let log = e.call_log.take().unwrap();
        let call = log
            .iter()
            .find(|c| c.0 == BLEND_INTERPOLATOR_CONSTRUCT_WITH)
            .unwrap();
        assert_eq!(call.1, vec![this, 7, 0x7fc0_0001, 9]);
    }

    #[test]
    fn creation_function_allocates_and_constructs() {
        let mut e = Engine::new();
        e.register(NI_ALLOCATE, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(BLEND_INTERPOLATOR_CONSTRUCT, |_, a| a[0].into_ret());
        let object = e.call(0x00a4_09d0, &[]).u32();
        assert_eq!(e.mem.u32(object), NI_BLEND_TRANSFORM_INTERPOLATOR_VTABLE);
        e.register(NI_ALLOCATE, |_, _| 0u32.into_ret());
        assert_eq!(e.call(0x00a4_09d0, &[]).u32(), 0);
    }
}

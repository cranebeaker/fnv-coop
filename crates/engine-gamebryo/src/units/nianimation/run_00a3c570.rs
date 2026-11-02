//! `NiAnimation/run_00a3c570` (Xbox PDB source unit), subsystem `NiAnimation`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiNew`'s allocator (`00aa13e0`, cdecl, the size in bytes).
const NI_ALLOCATE: u32 = 0x00aa_13e0;
/// `NiKeyBasedInterpolator::NiKeyBasedInterpolator` (`00a3ca10`, engine map).
const NI_KEY_BASED_INTERPOLATOR_CONSTRUCT: u32 = 0x00a3_ca10;
/// The vtable of `NiFloatInterpolator` (RTTI `.?AVNiFloatInterpolator@@`).
const NI_FLOAT_INTERPOLATOR_VTABLE: u32 = 0x0109_6f34;
/// The `float` the object's value starts as (`0109a918`, the same one the
/// `NiFloatInterpolator(NiFloatData*)` constructor `00a3c340` copies).
const INITIAL_FLOAT_VALUE: u32 = 0x0109_a918;
/// Size of an `NiFloatInterpolator` (the allocation, 0x18 bytes).
const NI_FLOAT_INTERPOLATOR_SIZE: u32 = 0x18;
/// `NiFloatInterpolator` (Xbox PDB): `m_fFloatValue` +0x0c,
/// `m_spFloatData` +0x10, `m_uiLastIdx` +0x14.
const FLOAT_VALUE: u32 = 0x0c;
const FLOAT_DATA: u32 = 0x10;
const LAST_INDEX: u32 = 0x14;

// Translated from 00a3c570 (disassembled, FalloutNV.exe 1.4.0.525; Ghidra made no function here)
/// `NiFloatInterpolator::CreateObject` (by the vtable it stores; the code is
/// not a Ghidra function): allocates 0x18 bytes; null when that fails.
/// Otherwise the `NiKeyBasedInterpolator` constructor, the
/// `NiFloatInterpolator` vtable, the value from the `float` at `0109a918`
/// (an x87 `FLD`/`FSTP` copy), no float data and last index 0.
pub fn ni_float_interpolator_create_object(e: &mut Engine) -> Ptr {
    let object = e
        .call(NI_ALLOCATE, &args![NI_FLOAT_INTERPOLATOR_SIZE])
        .u32();
    if object == 0 {
        return Ptr::NULL;
    }
    e.call(NI_KEY_BASED_INTERPOLATOR_CONSTRUCT, &args![object]);
    e.mem.set_u32(object, NI_FLOAT_INTERPOLATOR_VTABLE);
    let value = e.global::<u32>(INITIAL_FLOAT_VALUE);
    e.mem.set_u32(object + FLOAT_VALUE, x87_quiet_bits(value));
    e.mem.set_u32(object + FLOAT_DATA, 0);
    e.mem.set_u32(object + LAST_INDEX, 0);
    Ptr::new(object)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(
        0x00a3_c570,
        ni_float_interpolator_create_object() -> Ptr
    )]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_object_builds_an_empty_float_interpolator_or_gives_null() {
        let mut e = Engine::new();
        e.register(NI_ALLOCATE, |e, a| Ret {
            eax: e.mem.alloc(a[0]),
            ..Ret::default()
        });
        e.register(NI_KEY_BASED_INTERPOLATOR_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], 0x1111);
            e.mem.set_u32(a[0] + FLOAT_DATA, 7);
            e.mem.set_u32(a[0] + LAST_INDEX, 9);
            Ret::default()
        });
        e.map(INITIAL_FLOAT_VALUE, 4);
        // A signalling NaN is quieted by the FLD/FSTP copy.
        e.mem.set_u32(INITIAL_FLOAT_VALUE, 0xffa0_0000);
        e.call_log = Some(vec![]);
        let object = e.call(0x00a3_c570, &args![]).u32();
        assert_ne!(object, 0);
        assert_eq!(e.mem.block_size(object), Some(NI_FLOAT_INTERPOLATOR_SIZE));
        assert_eq!(e.mem.u32(object), NI_FLOAT_INTERPOLATOR_VTABLE);
        assert_eq!(e.mem.u32(object + FLOAT_VALUE), 0xffe0_0000);
        assert_eq!(e.mem.u32(object + FLOAT_DATA), 0);
        assert_eq!(e.mem.u32(object + LAST_INDEX), 0);
        let constructed: Vec<_> = e
            .call_log
            .iter()
            .flatten()
            .filter(|(a, _)| *a == NI_KEY_BASED_INTERPOLATOR_CONSTRUCT)
            .collect();
        assert_eq!(constructed.len(), 1);
        assert_eq!(constructed[0].1, vec![object]);

        // A failed allocation gives null and constructs nothing.
        e.register(NI_ALLOCATE, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00a3_c570, &args![]).u32(), 0);
        assert!(!e
            .call_log
            .iter()
            .flatten()
            .any(|(a, _)| *a == NI_KEY_BASED_INTERPOLATOR_CONSTRUCT));
    }
}

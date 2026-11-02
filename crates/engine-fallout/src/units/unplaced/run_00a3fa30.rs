//! `(unplaced)/run_00a3fa30` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiKeyBasedInterpolator::NiKeyBasedInterpolator` (`thiscall`, no stack words).
const KEY_BASED_INTERPOLATOR_CONSTRUCT: u32 = 0x00a3_ca10;
/// `NiTransformInterpolator`'s vtable (by its RTTI type descriptor).
const NI_TRANSFORM_INTERPOLATOR_VTABLE: u32 = 0x0109_7314;
/// The three words the translation is first copied from, and the four of the rotation.
const TRANSLATE_SOURCE: u32 = 0x011a_8400;
const ROTATE_SOURCE: u32 = 0x011f_3704;
/// The float the scale is first loaded from (`01096b8c`).
const SCALE_SOURCE: u32 = 0x0109_6b8c;
/// The float `-FLT_MAX` (`ff7fffff`) that marks the translation, rotation and scale unset.
const NO_VALUE_FLOAT: u32 = 0x0101_5f5c;
/// `InterlockedIncrement` (import slot, one word: the address).
const INTERLOCKED_INCREMENT: u32 = 0x00fd_f1c0;

// Translated from 00a3fa30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (by its body `NiTransformInterpolator`'s constructor from its
/// data; `thiscall`, one stack word; the field names are the Xbox PDB's): the
/// `NiKeyBasedInterpolator` constructor, the vtable `01097314`, the transform value
/// (`m_kTransformValue` +0xc) copied from the words at `011a8400` (translation) and `011f3704`
/// (rotation) and the float at `01096b8c` (scale), the data counted into `m_spData` (+0x2c),
/// then the translation's first word, the rotation's second word (`m_fX`) and the scale set
/// to `-FLT_MAX` (`01015f5c`), the last translation, rotation and scale key indices (+0x30,
/// +0x32, +0x34) 0 and `bPose` (+0x44) 0. Answers `this`.
pub fn fn_00a3fa30(e: &mut Engine, this: Ptr, data: Ptr) -> Ptr {
    let a = this.addr();
    e.call(KEY_BASED_INTERPOLATOR_CONSTRUCT, &args![this]);
    e.mem.set_u32(a, NI_TRANSFORM_INTERPOLATOR_VTABLE);
    for word in 0..3 {
        let value = e.mem.u32(TRANSLATE_SOURCE + 4 * word);
        e.mem.set_u32(a + 0xc + 4 * word, value);
    }
    for word in 0..4 {
        let value = e.mem.u32(ROTATE_SOURCE + 4 * word);
        e.mem.set_u32(a + 0x18 + 4 * word, value);
    }
    let scale = x87_store32(x87_load32(e.mem.u32(SCALE_SOURCE)));
    e.mem.set_u32(a + 0x28, scale);
    e.mem.set_u32(a + 0x2c, data.addr());
    if data.addr() != 0 {
        e.call(INTERLOCKED_INCREMENT, &args![data.addr() + 4]);
    }
    let no_value = x87_store32(x87_load32(e.mem.u32(NO_VALUE_FLOAT)));
    e.mem.set_u32(a + 0xc, no_value);
    e.mem.set_u32(a + 0x1c, no_value);
    e.mem.set_u16(a + 0x34, 0);
    e.mem.set_u32(a + 0x28, no_value);
    e.mem.set_u16(a + 0x30, 0);
    e.mem.set_u16(a + 0x32, 0);
    e.mem.set_u8(a + 0x44, 0);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00a3_fa30, fn_00a3fa30(Ptr, Ptr) -> Ptr)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fn_00a3fa30_sets_up_the_transform_and_counts_the_data() {
        let mut e = Engine::new();
        e.map(0x011a_8000, 0x1000);
        e.map(0x011f_3000, 0x1000);
        e.map(0x0109_6000, 0x1000);
        e.map(0x0101_5000, 0x1000);
        for word in 0..3 {
            e.mem.set_u32(TRANSLATE_SOURCE + 4 * word, 0x100 + word);
        }
        for word in 0..4 {
            e.mem.set_u32(ROTATE_SOURCE + 4 * word, 0x200 + word);
        }
        e.mem.set_u32(SCALE_SOURCE, 0x3f80_0000);
        e.mem.set_u32(NO_VALUE_FLOAT, 0xff7f_ffff);
        e.register(KEY_BASED_INTERPOLATOR_CONSTRUCT, |_, _| Ret::default());
        e.register(INTERLOCKED_INCREMENT, |e, a| {
            let count = e.mem.u32(a[0]) + 1;
            e.mem.set_u32(a[0], count);
            count.into_ret()
        });
        let this = e.mem.alloc(0x48);
        e.mem.write(this, &[0xee; 0x48]);
        let data = e.mem.alloc(0x10);
        assert_eq!(e.call(0x00a3_fa30, &args![this, data]).u32(), this);
        assert_eq!(e.mem.u32(this), NI_TRANSFORM_INTERPOLATOR_VTABLE);
        assert_eq!(e.mem.u32(this + 0xc), 0xff7f_ffff);
        assert_eq!(e.mem.u32(this + 0x10), 0x101);
        assert_eq!(e.mem.u32(this + 0x14), 0x102);
        assert_eq!(e.mem.u32(this + 0x18), 0x200);
        assert_eq!(e.mem.u32(this + 0x1c), 0xff7f_ffff);
        assert_eq!(e.mem.u32(this + 0x20), 0x202);
        assert_eq!(e.mem.u32(this + 0x24), 0x203);
        assert_eq!(e.mem.u32(this + 0x28), 0xff7f_ffff);
        assert_eq!(e.mem.u32(this + 0x2c), data);
        assert_eq!(e.mem.u32(data + 4), 1);
        assert_eq!(e.mem.u16(this + 0x30), 0);
        assert_eq!(e.mem.u16(this + 0x32), 0);
        assert_eq!(e.mem.u16(this + 0x34), 0);
        assert_eq!(e.mem.u16(this + 0x36), 0xeeee);
        assert_eq!(e.mem.u8(this + 0x44), 0);
        // Without data nothing is counted.
        let other = e.mem.alloc(0x48);
        e.call_log = Some(vec![]);
        e.call(0x00a3_fa30, &args![other, 0u32]);
        let log = e.call_log.take().unwrap();
        assert!(!log.iter().any(|c| c.0 == INTERLOCKED_INCREMENT));
        assert_eq!(e.mem.u32(other + 0x2c), 0);
    }
}

//! `NiMain/niaccumulator.cpp` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiAccumulator::ms_RTTI` (the class's `NiRTTI` record).
const NI_ACCUMULATOR_RTTI: u32 = 0x011f_6060;

// Translated from 00aa0b00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiAccumulator::GetRTTI` (Xbox PDB): the address of the class's `NiRTTI` record.
pub fn ni_accumulator_get_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(NI_ACCUMULATOR_RTTI)
}

/// `NiAccumulator::m_pkCamera` (Xbox PDB) +8.
const ACCUMULATOR_CAMERA: u32 = 0x8;

// Translated from 00aa0fe0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiAccumulator::FinishAccumulating` (Xbox PDB): clears the camera
/// pointer at +8.
pub fn ni_accumulator_finish_accumulating(e: &mut Engine, this: Ptr) {
    e.mem
        .set_u32(this.addr().wrapping_add(ACCUMULATOR_CAMERA), 0);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00aa0b00, ni_accumulator_get_rtti() -> Ptr),
        entry!(0x00aa0fe0, ni_accumulator_finish_accumulating(Ptr)),
        entry!(
            0x00aa_0b10,
            ni_accumulator_scalar_deleting_destructor(Ptr, u8) -> Ptr
        ),
    ]
}

/// `NiAccumulator`'s vtable (`010a2304`, the scalar deleting destructor's), the `NiObject`
/// destructor body (`00a5d3d0`) and the sized free (`00aa1460`, cdecl: block, size).
const NI_ACCUMULATOR_VTABLE: u32 = 0x010a_2304;
const NI_OBJECT_DESTRUCT: u32 = 0x00a5_d3d0;
const NI_FREE_SIZED: u32 = 0x00aa_1460;

// Translated from 00aa0b10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiAccumulator`'s scalar deleting destructor (Xbox PDB; thiscall, `RET 4`): the class's
/// vtable, `NiObject`'s destructor body (`00a5d3d0`) and, with bit 0 of `flags`, the 0xc
/// bytes given back (`00aa1460`). Answers `this`.
pub fn ni_accumulator_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    e.mem.set_u32(this.addr(), NI_ACCUMULATOR_VTABLE);
    e.call(NI_OBJECT_DESTRUCT, &args![this]);
    if flags & 1 != 0 {
        e.call(NI_FREE_SIZED, &args![this, 0xcu32]);
    }
    this
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ni_accumulator_get_rtti_returns_the_class_rtti() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00aa0b00, &args![]).u32(), NI_ACCUMULATOR_RTTI);
    }
}

#[cfg(test)]
mod b014_tests {
    use super::*;

    #[test]
    fn finish_accumulating_clears_the_camera() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x10);
        e.mem.set_u32(this + 8, 0x1234);
        e.mem.set_u32(this + 0xc, 0x5678);
        e.call(0x00aa0fe0, &args![this]);
        assert_eq!(e.mem.u32(this + 8), 0);
        assert_eq!(e.mem.u32(this + 0xc), 0x5678);
    }
}

#[cfg(test)]
mod n014_tests {
    use super::*;

    #[test]
    fn scalar_deleting_destructor_frees_only_on_request() {
        let mut e = Engine::new();
        e.register(NI_OBJECT_DESTRUCT, |_, _| Ret::default());
        e.register(NI_FREE_SIZED, |_, _| Ret::default());
        let this = e.mem.alloc(0xc);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00aa_0b10, &args![this, 0u32]).u32(), this);
        assert_eq!(e.mem.u32(this), NI_ACCUMULATOR_VTABLE);
        e.call(0x00aa_0b10, &args![this, 1u32]);
        let log = e.call_log.take().unwrap();
        let frees: Vec<_> = log.iter().filter(|c| c.0 == NI_FREE_SIZED).collect();
        assert_eq!(frees.len(), 1);
        assert_eq!(frees[0].1, vec![this, 0xc]);
        assert_eq!(log.iter().filter(|c| c.0 == NI_OBJECT_DESTRUCT).count(), 2);
    }
}

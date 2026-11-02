//! `NiMain/niloddata.cpp` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiLODData::ms_RTTI` (the class's `NiRTTI` record).
const NI_LOD_DATA_RTTI: u32 = 0x011f_6050;

// Translated from 00aa09e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiLODData::GetRTTI` (Xbox PDB): the address of the class's `NiRTTI` record.
pub fn ni_lod_data_get_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(NI_LOD_DATA_RTTI)
}

/// `NiLODData`'s vtable (`010a2254`), `NiObject`'s destructor body (`00a5d3d0`, `thiscall`)
/// and the sized delete (`00aa1460`, `cdecl`: pointer, size).
const NI_LOD_DATA_VTABLE: u32 = 0x010a_2254;
const NI_OBJECT_DESTRUCT: u32 = 0x00a5_d3d0;
const SIZED_DELETE: u32 = 0x00aa_1460;

// Translated from 00aa09f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiLODData::_scalar_deleting_destructor_` (Xbox PDB): the vtable, `NiObject`'s destructor
/// body, then the sized delete of 8 bytes when bit 0 of `flags` is set. Answers `this`.
pub fn ni_lod_data_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.mem.set_u32(this.addr(), NI_LOD_DATA_VTABLE);
    e.call(NI_OBJECT_DESTRUCT, &args![this]);
    if flags & 1 != 0 {
        e.call(SIZED_DELETE, &args![this, 8u32]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00aa09e0, ni_lod_data_get_rtti() -> Ptr),
        entry!(
            0x00aa_09f0,
            ni_lod_data_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ni_lod_data_get_rtti_returns_the_class_rtti() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00aa09e0, &args![]).u32(), NI_LOD_DATA_RTTI);
    }
}

#[cfg(test)]
mod a06_tests {
    use super::*;

    #[test]
    fn scalar_deleting_destructor_runs_the_base_and_deletes_on_bit_0() {
        let mut e = Engine::new();
        e.register(NI_OBJECT_DESTRUCT, |_, _| Ret::default());
        e.register(SIZED_DELETE, |_, _| Ret::default());
        let this = e.mem.alloc(8);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00aa_09f0, &args![this, 0u32]).u32(), this);
        assert_eq!(e.mem.u32(this), NI_LOD_DATA_VTABLE);
        e.call(0x00aa_09f0, &args![this, 1u32]);
        let log = e.call_log.take().unwrap();
        let of = |addr: u32| -> Vec<Vec<u32>> {
            log.iter()
                .filter(|c| c.0 == addr)
                .map(|c| c.1.clone())
                .collect()
        };
        assert_eq!(of(NI_OBJECT_DESTRUCT), vec![vec![this]; 2]);
        assert_eq!(of(SIZED_DELETE), vec![vec![this, 8]]);
    }
}

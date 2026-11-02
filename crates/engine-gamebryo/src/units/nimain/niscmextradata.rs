//! `NiMain/niscmextradata.cpp` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiSCMExtraData::ms_RTTI` (the class's `NiRTTI` record).
const NI_SCM_EXTRA_DATA_RTTI: u32 = 0x0127_0908;
/// `NiSCMExtraData`'s vtable (its RTTI names `NiSCMExtraData`).
const NI_SCM_EXTRA_DATA_VTABLE: u32 = 0x0110_129c;
/// `00aa10f0` (`cdecl`, one word): frees a block.
const FREE: u32 = 0x00aa_10f0;
/// `00a7b300` (`thiscall`, unit `niextradata.cpp`): the extra-data base's destructor body.
const EXTRA_DATA_DESTRUCTOR: u32 = 0x00a7_b300;
/// `00aa1460` (`cdecl`, (block, size)): the sized delete.
const SIZED_DELETE: u32 = 0x00aa_1460;

// Translated from 00ee89a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiSCMExtraData::GetRTTI` (Xbox PDB): the address of the class's `NiRTTI` record.
pub fn ni_scm_extra_data_get_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(NI_SCM_EXTRA_DATA_RTTI)
}

// Translated from 00ee89b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiSCMExtraData::_scalar_deleting_destructor_` (Xbox PDB): sets the class's vtable,
/// frees the blocks held at +0x24 and +0x2c (the Xbox PDB's `m_apkEntries` array starts at
/// +0x24; the exe frees only these two words of it), runs the extra-data base's destructor
/// body (`00a7b300`), then the sized delete of 0x30 bytes when bit 0 of `flags` is set.
/// Answers `this`.
pub fn ni_scm_extra_data_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    let first = e.mem.u32(this.addr() + 0x24);
    e.mem.set_u32(this.addr(), NI_SCM_EXTRA_DATA_VTABLE);
    e.call(FREE, &args![first]);
    let third = e.mem.u32(this.addr() + 0x2c);
    e.call(FREE, &args![third]);
    e.call(EXTRA_DATA_DESTRUCTOR, &args![this]);
    if flags & 1 != 0 {
        e.call(SIZED_DELETE, &args![this, 0x30u32]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00ee_89a0, ni_scm_extra_data_get_rtti() -> Ptr),
        entry!(
            0x00ee_89b0,
            ni_scm_extra_data_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .clone()
            .unwrap_or_default()
            .into_iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, words)| words)
            .collect()
    }

    #[test]
    fn get_rtti_returns_the_class_rtti() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00ee_89a0, &args![]).u32(), NI_SCM_EXTRA_DATA_RTTI);
    }

    #[test]
    fn destructor_frees_two_entries_and_deletes_only_on_bit_0() {
        let mut e = Engine::new();
        for double in [FREE, EXTRA_DATA_DESTRUCTOR, SIZED_DELETE] {
            e.register(double, |_, _| Ret::default());
        }
        e.call_log = Some(vec![]);
        let this = e.mem.alloc(0x30);
        e.mem.set_u32(this + 0x24, 0x1111);
        e.mem.set_u32(this + 0x28, 0x2222);
        e.mem.set_u32(this + 0x2c, 0x3333);
        assert_eq!(e.call(0x00ee_89b0, &args![this, 0xfeu32]).u32(), this);
        assert_eq!(e.mem.u32(this), NI_SCM_EXTRA_DATA_VTABLE);
        assert_eq!(calls_to(&e, FREE), vec![vec![0x1111], vec![0x3333]]);
        assert_eq!(calls_to(&e, EXTRA_DATA_DESTRUCTOR), vec![vec![this]]);
        assert!(calls_to(&e, SIZED_DELETE).is_empty());
        e.call(0x00ee_89b0, &args![this, 1u32]);
        assert_eq!(calls_to(&e, SIZED_DELETE), vec![vec![this, 0x30]]);
    }
}

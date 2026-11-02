//! `NiAnimation/run_00a44f90` (Xbox PDB source unit), subsystem `NiAnimation`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiObject`'s constructor (no arguments; `this` = the object).
const OBJECT_CONSTRUCT: u32 = 0x00a5_d3a0;
/// `NiUVData`'s vtable (its `RTTI` locator names the class; the vtable also holds the scalar
/// deleting destructor `00a45200`).
const NI_UV_DATA_VTABLE: u32 = 0x0109_7f84;

layout! {
    /// `NiUVData` (Xbox PDB), 0x4c bytes on PC as on the Xbox: the animation keys of a UV
    /// controller, four tracks (U offset, V offset, U tiling, V tiling) of a count, a key
    /// array and a key type each, then the last values and the key sizes.
    pub struct NiUVData: 0x4c {
        /// `m_uiNumUOffsetKeys` (Xbox PDB).
        0x08 m_uiNumUOffsetKeys: u32,
        /// `m_pkUOffsetKeys` (Xbox PDB).
        0x0C m_pkUOffsetKeys: u32,
        /// `m_eUOffsetType` (Xbox PDB).
        0x10 m_eUOffsetType: u32,
        /// `m_uiNumVOffsetKeys` (Xbox PDB).
        0x14 m_uiNumVOffsetKeys: u32,
        /// `m_pkVOffsetKeys` (Xbox PDB).
        0x18 m_pkVOffsetKeys: u32,
        /// `m_eVOffsetType` (Xbox PDB).
        0x1C m_eVOffsetType: u32,
        /// `m_uiNumUTilingKeys` (Xbox PDB).
        0x20 m_uiNumUTilingKeys: u32,
        /// `m_pkUTilingKeys` (Xbox PDB).
        0x24 m_pkUTilingKeys: u32,
        /// `m_eUTilingType` (Xbox PDB).
        0x28 m_eUTilingType: u32,
        /// `m_uiNumVTilingKeys` (Xbox PDB).
        0x2C m_uiNumVTilingKeys: u32,
        /// `m_pkVTilingKeys` (Xbox PDB).
        0x30 m_pkVTilingKeys: u32,
        /// `m_eVTilingType` (Xbox PDB).
        0x34 m_eVTilingType: u32,
        /// `m_fLastUOffset` (Xbox PDB).
        0x38 m_fLastUOffset: f32,
        /// `m_fLastVOffset` (Xbox PDB).
        0x3C m_fLastVOffset: f32,
        /// `m_fLastUTiling` (Xbox PDB).
        0x40 m_fLastUTiling: f32,
        /// `m_fLastVTiling` (Xbox PDB).
        0x44 m_fLastVTiling: f32,
        /// `m_ucUOffsetSize` (Xbox PDB).
        0x48 m_ucUOffsetSize: u8,
        /// `m_ucVOffsetSize` (Xbox PDB).
        0x49 m_ucVOffsetSize: u8,
        /// `m_ucUTilingSize` (Xbox PDB).
        0x4A m_ucUTilingSize: u8,
        /// `m_ucVTilingSize` (Xbox PDB).
        0x4B m_ucVTilingSize: u8,
    }
}

// Translated from 00a44f90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiUVData::NiUVData` (Xbox PDB; the class is the one whose vtable `01097f84` this
/// stores, and the engine map has no name here): runs `NiObject`'s constructor (`00a5d3a0`),
/// clears the three other tracks' counts, arrays and types and the U offset track's, sets the
/// last U and V offsets to 0.0 and the last U and V tilings to 1.0, clears the four key
/// sizes, stores the vtable last and answers `this`.
pub fn ni_uv_data_constructor(e: &mut Engine, this: Ptr<NiUVData>) -> Ptr<NiUVData> {
    e.call(OBJECT_CONSTRUCT, &args![this]);
    e.set(this, NiUVData::m_fLastUOffset, 0.0);
    e.set(this, NiUVData::m_fLastVOffset, 0.0);
    e.set(this, NiUVData::m_uiNumUOffsetKeys, 0);
    e.set(this, NiUVData::m_pkUOffsetKeys, 0);
    e.set(this, NiUVData::m_fLastUTiling, 1.0);
    e.set(this, NiUVData::m_eUOffsetType, 0);
    e.set(this, NiUVData::m_uiNumVOffsetKeys, 0);
    e.set(this, NiUVData::m_fLastVTiling, 1.0);
    e.set(this, NiUVData::m_pkVOffsetKeys, 0);
    e.set(this, NiUVData::m_eVOffsetType, 0);
    e.set(this, NiUVData::m_uiNumUTilingKeys, 0);
    e.set(this, NiUVData::m_pkUTilingKeys, 0);
    e.set(this, NiUVData::m_eUTilingType, 0);
    e.set(this, NiUVData::m_uiNumVTilingKeys, 0);
    e.set(this, NiUVData::m_pkVTilingKeys, 0);
    e.set(this, NiUVData::m_eVTilingType, 0);
    e.set(this, NiUVData::m_ucUOffsetSize, 0);
    e.set(this, NiUVData::m_ucVOffsetSize, 0);
    e.set(this, NiUVData::m_ucUTilingSize, 0);
    e.set(this, NiUVData::m_ucVTilingSize, 0);
    e.mem.set_u32(this.addr(), NI_UV_DATA_VTABLE);
    this
}

/// `00aa13e0` (`cdecl`, the size): the Gamebryo allocation the objects are made with.
const NI_ALLOCATE: u32 = 0x00aa_13e0;

// Translated from 00a44ff0 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (a function Ghidra did not create; no arguments; by its body
/// `NiUVData`'s creation function): 0x4c bytes from `00aa13e0`, built by
/// [`ni_uv_data_constructor`] (a tail jump). Answers the object, null when the allocation
/// fails.
pub fn fn_00a44ff0(e: &mut Engine) -> Ptr<NiUVData> {
    let object = e.call(NI_ALLOCATE, &args![0x4cu32]).u32();
    if object == 0 {
        return Ptr::new(0);
    }
    ni_uv_data_constructor(e, Ptr::new(object))
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00a44f90,
            ni_uv_data_constructor(Ptr<NiUVData>) -> Ptr<NiUVData>
        ),
        entry!(0x00a4_4ff0, fn_00a44ff0() -> Ptr<NiUVData>),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ni_uv_data_constructor_sets_the_defaults_and_the_vtable() {
        let mut e = Engine::new();
        e.register(OBJECT_CONSTRUCT, |_, _| Ret::default());
        let this = e.new_object::<NiUVData>();
        for offset in (4..0x4c).step_by(4) {
            e.mem.set_u32(this.addr() + offset, 0xdead_beef);
        }
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00a44f90, &args![this]).u32(), this.addr());
        assert_eq!(
            e.call_log.as_ref().unwrap()[1..],
            [(OBJECT_CONSTRUCT, vec![this.addr()])]
        );
        assert_eq!(e.mem.u32(this.addr()), NI_UV_DATA_VTABLE);
        assert_eq!(e.mem.u32(this.addr() + 4), 0xdead_beef);
        for offset in (8..0x38).step_by(4) {
            assert_eq!(e.mem.u32(this.addr() + offset), 0, "offset {offset:#x}");
        }
        assert_eq!(e.get(this, NiUVData::m_fLastUOffset), 0.0);
        assert_eq!(e.get(this, NiUVData::m_fLastVOffset), 0.0);
        assert_eq!(e.get(this, NiUVData::m_fLastUTiling), 1.0);
        assert_eq!(e.get(this, NiUVData::m_fLastVTiling), 1.0);
        assert_eq!(e.mem.u32(this.addr() + 0x48), 0);
    }
}

#[cfg(test)]
mod n007_tests {
    use super::*;

    #[test]
    fn creation_function_allocates_and_constructs() {
        let mut e = Engine::new();
        e.register(OBJECT_CONSTRUCT, |_, _| Ret::default());
        e.register(NI_ALLOCATE, |e, a| e.mem.alloc(a[0]).into_ret());
        let object = e.call(0x00a4_4ff0, &[]).u32();
        assert_eq!(e.mem.u32(object), NI_UV_DATA_VTABLE);
        e.register(NI_ALLOCATE, |_, _| 0u32.into_ret());
        assert_eq!(e.call(0x00a4_4ff0, &[]).u32(), 0);
    }
}

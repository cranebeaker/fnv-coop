//! `BSShader/bsdistanttreeshaderproperty.cpp` (Xbox PDB source unit), subsystem `BSShader`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

layout! {
    /// `BSDistantTreeShaderProperty` (Xbox PDB, 0x70 bytes there), a `BSShaderProperty`. The
    /// PC base class lacks the Xbox's `fLastClearFrame` and `fLastAccumFrame` words, so this
    /// class's own fields sit 8 bytes lower (the offsets the constructor writes).
    pub struct BSDistantTreeShaderProperty: 0x68 {
        /// `NiShadeProperty::iShaderPropertyType` (Xbox PDB).
        0x1C iShaderPropertyType: u32,
        /// `BSShaderProperty::iShader` (Xbox PDB).
        0x58 iShader: u32,
        /// `uiInstanceCount` (Xbox PDB).
        0x60 uiInstanceCount: u32,
        /// `pInstancesA` (Xbox PDB): `BSDistantTreeShader::InstanceData*`, 0x20 bytes each.
        0x64 pInstancesA: u32,
    }
}

/// `BSShaderProperty::BSShaderProperty` (engine map, `00ba8e50`; thiscall, no stack words).
const SHADER_PROPERTY_CONSTRUCT: u32 = 0x00ba_8e50;
/// The class's vtable (`010b9c40`, stored by the constructor).
const VTABLE: u32 = 0x010b_9c40;
/// `BSDistantTreeShader::GetMaxInstances` (engine map, `00bb6030`; no arguments).
const GET_MAX_INSTANCES: u32 = 0x00bb_6030;
/// The memory manager singleton (`011f6238`) and its allocate (`00aa3e40`; thiscall, size).
const MEMORY_MANAGER: u32 = 0x011f_6238;
const MEMORY_ALLOCATE: u32 = 0x00aa_3e40;

// Translated from 00bb7170 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSDistantTreeShaderProperty::BSDistantTreeShaderProperty` (Xbox PDB): thiscall, no stack
/// words; answers `this`. Runs the base constructor (`00ba8e50`), sets the vtable, `iShader`
/// 8, `uiInstanceCount` from `BSDistantTreeShader::GetMaxInstances` and `pInstancesA` to a
/// block of 0x20 bytes per instance from the memory manager (size `0xFFFFFFFF` when the
/// product overflows 32 bits, as the exe's `MUL`/`SETO`/`NEG`/`OR` computes it), and
/// `iShaderPropertyType` 0x18.
// Translation incomplete: 00bb7170 - the C++ exception frame (FS:[0] chain, unwind state 0) is not translated.
pub fn bs_distant_tree_shader_property_bs_distant_tree_shader_property(
    e: &mut Engine,
    this: Ptr<BSDistantTreeShaderProperty>,
) -> Ptr<BSDistantTreeShaderProperty> {
    let base = this.addr();
    e.call(SHADER_PROPERTY_CONSTRUCT, &args![base]);
    e.mem.set_u32(base, VTABLE);
    e.set(this, BSDistantTreeShaderProperty::iShader, 8);
    let count = e.call(GET_MAX_INSTANCES, &args![]).u32();
    e.set(this, BSDistantTreeShaderProperty::uiInstanceCount, count);
    let product = u64::from(count) * 0x20;
    let size = if product >> 32 != 0 {
        0xffff_ffff
    } else {
        product as u32
    };
    let block = e.call(MEMORY_ALLOCATE, &args![MEMORY_MANAGER, size]).u32();
    e.set(this, BSDistantTreeShaderProperty::pInstancesA, block);
    e.set(this, BSDistantTreeShaderProperty::iShaderPropertyType, 0x18);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(
        0x00bb_7170,
        bs_distant_tree_shader_property_bs_distant_tree_shader_property(
            Ptr<BSDistantTreeShaderProperty>,
        ) -> Ptr<
            BSDistantTreeShaderProperty,
        >
    )]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constructor_sizes_the_instance_block_and_saturates_on_overflow() {
        // (instances, size asked): 0x20 bytes each, all ones past 32 bits.
        for (count, size) in [(0x40u32, 0x800u32), (0x0800_0001, 0xffff_ffff)] {
            let mut e = Engine::new();
            e.register(SHADER_PROPERTY_CONSTRUCT, |_, _| Ret::default());
            e.register_double(GET_MAX_INSTANCES, move |_, _| Ret {
                eax: count,
                ..Ret::default()
            });
            e.register(MEMORY_ALLOCATE, |_, _| Ret {
                eax: 0x0e00_4000,
                ..Ret::default()
            });
            let this: Ptr<BSDistantTreeShaderProperty> = e.new_object();
            e.call_log = Some(vec![]);
            assert_eq!(e.call(0x00bb_7170, &args![this]).u32(), this.addr());
            let log = e.call_log.take().unwrap();
            let base: Vec<_> = log
                .iter()
                .filter(|c| c.0 == SHADER_PROPERTY_CONSTRUCT)
                .collect();
            assert_eq!(base[0].1, vec![this.addr()]);
            let alloc: Vec<_> = log.iter().filter(|c| c.0 == MEMORY_ALLOCATE).collect();
            assert_eq!(alloc[0].1, vec![MEMORY_MANAGER, size]);
            assert_eq!(e.mem.u32(this.addr()), VTABLE);
            assert_eq!(e.get(this, BSDistantTreeShaderProperty::iShader), 8);
            assert_eq!(
                e.get(this, BSDistantTreeShaderProperty::uiInstanceCount),
                count
            );
            assert_eq!(
                e.get(this, BSDistantTreeShaderProperty::pInstancesA),
                0x0e00_4000
            );
            assert_eq!(
                e.get(this, BSDistantTreeShaderProperty::iShaderPropertyType),
                0x18
            );
        }
    }
}

//! `(unplaced)/run_00a84ca0` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `MemoryManager::Deallocate` (`00aa4060`: the manager and the block).
const MEMORY_MANAGER_DEALLOCATE: u32 = 0x00aa_4060;
/// The `MemoryManager` singleton (`011f6238`).
const MEMORY_MANAGER: u32 = 0x011f_6238;
/// `NiSkinData` on the PC (`NiTransform` is 0x34 bytes there):
/// `m_pkBoneData` +0x40, `m_uiBones` +0x44; each `BoneData` is 0x4C bytes
/// with `m_pkBoneVertData` at +0x44 and `m_usVerts` at +0x48.
const BONE_DATA: u32 = 0x40;
const BONES: u32 = 0x44;
const BONE_DATA_SIZE: u32 = 0x4c;
const BONE_VERT_DATA: u32 = 0x44;
const BONE_VERTS: u32 = 0x48;

// Translated from 00a84c90 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `NiSkinData::BoneData` constructor (Ghidra made no function of it;
/// `NiSkinData::LoadBinary` passes it to the vector constructor): clears
/// the vertex data pointer and count. Returns `this`.
pub fn fn_00a84c90(e: &mut Engine, this: Ptr) -> Ptr {
    e.mem.set_u32(this.addr() + BONE_VERT_DATA, 0);
    e.mem.set_u16(this.addr() + BONE_VERTS, 0);
    this
}

// Translated from 00a84ca0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Frees every bone's vertex data (`MemoryManager::Deallocate`) and clears
/// the pointers, when the skin data has bone data.
pub fn fn_00a84ca0(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    if e.mem.u32(base + BONE_DATA) == 0 {
        return;
    }
    let mut index = 0u32;
    while index < e.mem.u32(base + BONES) {
        let bone = e
            .mem
            .u32(base + BONE_DATA)
            .wrapping_add(index.wrapping_mul(BONE_DATA_SIZE));
        let block = e.mem.u32(bone + BONE_VERT_DATA);
        e.call(MEMORY_MANAGER_DEALLOCATE, &args![MEMORY_MANAGER, block]);
        let bone = e
            .mem
            .u32(base + BONE_DATA)
            .wrapping_add(index.wrapping_mul(BONE_DATA_SIZE));
        e.mem.set_u32(bone + BONE_VERT_DATA, 0);
        index += 1;
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a84c90, fn_00a84c90(Ptr) -> Ptr),
        entry!(0x00a84ca0, fn_00a84ca0(Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bone_data_constructor_clears_the_vertex_data() {
        let mut e = Engine::new();
        let bone = e.mem.alloc(0x4c);
        e.mem.write(bone, &[0xaa; 0x4c]);
        assert_eq!(e.call(0x00a8_4c90, &args![bone]).u32(), bone);
        assert_eq!(e.mem.u32(bone + 0x44), 0);
        assert_eq!(e.mem.u16(bone + 0x48), 0);
        assert_eq!(e.mem.u16(bone + 0x4a), 0xaaaa);
    }

    #[test]
    fn frees_each_bones_vertex_data() {
        let mut e = Engine::new();
        e.register(MEMORY_MANAGER_DEALLOCATE, |_, _| Ret::default());
        let data = e.mem.alloc(0x48);
        e.call_log = Some(vec![]);
        e.call(0x00a8_4ca0, &args![data]);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
        let bones = e.mem.alloc(0x4c * 2);
        e.mem.set_u32(bones + 0x44, 0x111);
        e.mem.set_u32(bones + 0x4c + 0x44, 0x222);
        e.mem.set_u32(data + 0x40, bones);
        e.mem.set_u32(data + 0x44, 2);
        e.call(0x00a8_4ca0, &args![data]);
        let freed: Vec<_> = e
            .call_log
            .iter()
            .flatten()
            .filter(|(a, _)| *a == MEMORY_MANAGER_DEALLOCATE)
            .map(|(_, w)| w.clone())
            .collect();
        assert_eq!(
            freed,
            vec![vec![MEMORY_MANAGER, 0x111], vec![MEMORY_MANAGER, 0x222]]
        );
        assert_eq!(e.mem.u32(bones + 0x44), 0);
        assert_eq!(e.mem.u32(bones + 0x4c + 0x44), 0);
    }
}

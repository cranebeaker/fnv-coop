//! `BSShader/imagespaceeffect.cpp` (Xbox PDB source unit), subsystem `BSShader`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

layout! {
    /// `ImageSpaceEffect` (Xbox PDB), 0x58 bytes; the PC code reads the same offsets. Its
    /// five members are `NiTPrimitiveArray`s (the shared `NiTArray` layout: vtable, element
    /// pointer, 16-bit counts).
    pub struct ImageSpaceEffect: 0x58 {
        /// `bIsActive` (Xbox PDB).
        0x04 bIsActive: u8,
        /// `bParamsChanged` (Xbox PDB).
        0x05 bParamsChanged: u8,
        /// `pEffectList` (Xbox PDB): `NiTPrimitiveArray<ImageSpaceEffect *>`.
        0x08 pEffectList: Inline<crate::types::NiTArray>,
        /// `pEffectParamList` (Xbox PDB): `NiTPrimitiveArray<ImageSpaceEffectParam *>`.
        0x18 pEffectParamList: Inline<crate::types::NiTArray>,
        /// `pTextures` (Xbox PDB): `NiTPrimitiveArray<ImageSpaceTexture *>`.
        0x28 pTextures: Inline<crate::types::NiTArray>,
        /// `pEffectInputs` (Xbox PDB): `NiTPrimitiveArray<ImageSpaceEffect::EffectInput *>`.
        0x38 pEffectInputs: Inline<crate::types::NiTArray>,
        /// `pEffectOutput` (Xbox PDB): `NiTPrimitiveArray<int>`.
        0x48 pEffectOutput: Inline<crate::types::NiTArray>,
    }
}

/// `ImageSpaceEffect`'s vtable (`010b7fe4`), stored by the destructor.
const VTABLE: u32 = 0x010b_7fe4;
/// The vtable the destructor gives each `NiTPrimitiveArray` member before freeing its
/// elements (`010b9d24`).
const PRIMITIVE_ARRAY_VTABLE: u32 = 0x010b_9d24;
/// `00ba3aa0` (`thiscall`, two words: an index and the address of the value): stores the
/// value in that slot of an `NiTArray`.
const ARRAY_SET_AT: u32 = 0x00ba_3aa0;
/// `00d47a40` (`thiscall`, one word): stores the word at `this + 8`.
const SET_WORD_8: u32 = 0x00d4_7a40;
/// The memory manager object passed as `this` to its deallocation.
const MEMORY_MANAGER: u32 = 0x011f_6238;
/// The memory manager's deallocation (`00aa4060`, `thiscall`, one word).
const MEMORY_DEALLOCATE: u32 = 0x00aa_4060;
/// `00aa10f0` (`cdecl`, one word): releases an array block (null does nothing).
const RELEASE_BLOCK: u32 = 0x00aa_10f0;

// Translated from 00ba3cb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ImageSpaceEffect::SetTexture` (engine map; `thiscall`, `RET 0xc`: an index, a texture,
/// a third word): stores the texture in slot `index` of `pTextures` (`00ba3aa0`, given the
/// address of the texture's stack word) and, when the texture (read back from that word) is
/// not null, stores the third word at its +0x08 (`00d47a40`).
pub fn image_space_effect_set_texture(
    e: &mut Engine,
    this: Ptr<ImageSpaceEffect>,
    index: u32,
    texture: u32,
    value: u32,
) {
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), texture);
        let textures = this.addr() + ImageSpaceEffect::pTextures.off;
        e.call(ARRAY_SET_AT, &args![textures, index, slot]);
        let texture = e.mem.u32(slot.addr());
        if texture != 0 {
            e.call(SET_WORD_8, &args![texture, value]);
        }
    });
}

// Translated from 00ba3d30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ImageSpaceEffect::~ImageSpaceEffect` (engine map; `thiscall`): stores the class's
/// vtable, frees every non-null element of `pEffectInputs` (the memory manager's
/// deallocation) and clears its slot (`00ba3aa0` with a zero word), re-reading the size
/// (`m_usSize`, +0x42) after each element. Then each array member from the last to the
/// first gets the primitive-array vtable and its element block is released (`00aa10f0`).
///
/// The exception frame (`FS:[0]`) is not translated.
// Translation incomplete: 00ba3d30 - the FS:[0] exception frame (unwinding on a throw) is not translated
pub fn image_space_effect_destructor(e: &mut Engine, this: Ptr<ImageSpaceEffect>) {
    let base = this.addr();
    e.mem.set_u32(base, VTABLE);
    let inputs = base + ImageSpaceEffect::pEffectInputs.off;
    let mut index = 0u32;
    if e.mem.u16(inputs + 0xa) != 0 {
        loop {
            let elements = e.mem.u32(inputs + 4);
            let element = e.mem.u32(elements.wrapping_add(index * 4));
            if element != 0 {
                e.call(MEMORY_DEALLOCATE, &args![MEMORY_MANAGER, element]);
                e.with_stack(4, |e, zero| {
                    e.call(ARRAY_SET_AT, &args![inputs, index, zero]);
                });
            }
            index += 1;
            if index >= u32::from(e.mem.u16(inputs + 0xa)) {
                break;
            }
        }
    }
    for member in [0x48u32, 0x38, 0x28, 0x18, 0x08] {
        let elements = e.mem.u32(base + member + 4);
        e.mem.set_u32(base + member, PRIMITIVE_ARRAY_VTABLE);
        e.call(RELEASE_BLOCK, &args![elements]);
    }
}

// Translated from 00ba3ce0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the deleting destructor of an `NiTPrimitiveArray` member
/// (`thiscall`, one word: the flags): the primitive-array vtable `010b9d24`, its element
/// block released (`00aa10f0`), then the array freed through the memory manager when bit 0
/// of the flags is set. Answers `this`.
pub fn fn_00ba3ce0(
    e: &mut Engine,
    this: Ptr<crate::types::NiTArray>,
    flags: u32,
) -> Ptr<crate::types::NiTArray> {
    let elements = e.get(this, crate::types::NiTArray::m_pBase);
    e.mem.set_u32(this.addr(), PRIMITIVE_ARRAY_VTABLE);
    e.call(RELEASE_BLOCK, &args![elements]);
    if flags & 1 != 0 {
        e.call(MEMORY_DEALLOCATE, &args![MEMORY_MANAGER, this]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00ba_3cb0,
            image_space_effect_set_texture(Ptr<ImageSpaceEffect>, u32, u32, u32)
        ),
        entry!(
            0x00ba_3d30,
            image_space_effect_destructor(Ptr<ImageSpaceEffect>)
        ),
        entry!(
            0x00ba_3ce0,
            fn_00ba3ce0(Ptr<crate::types::NiTArray>, u32) -> Ptr<crate::types::NiTArray>
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn set_texture_stores_the_slot_and_the_textures_word_8() {
        let mut e = Engine::new();
        e.register(ARRAY_SET_AT, |e, a| {
            // Records the value the slot word holds.
            let value = e.mem.u32(a[2]);
            e.mem.set_u32(0x0090_0000, value);
            Ret::default()
        });
        e.map(0x0090_0000, 0x10);
        e.register(SET_WORD_8, |_, _| Ret::default());
        let this = e.mem.alloc(0x58);
        e.call_log = Some(vec![]);
        e.call(0x00ba_3cb0, &[this, 3, 0x5000, 0x77]);
        let log = e.call_log.take().unwrap();
        let set: Vec<_> = log.iter().filter(|(a, _)| *a == ARRAY_SET_AT).collect();
        assert_eq!(set[0].1[..2], [this + 0x28, 3]);
        assert_eq!(e.mem.u32(0x0090_0000), 0x5000);
        let word8: Vec<_> = log.iter().filter(|(a, _)| *a == SET_WORD_8).collect();
        assert_eq!(word8[0].1, vec![0x5000, 0x77]);

        // A null texture stores the slot only.
        e.call_log = Some(vec![]);
        e.call(0x00ba_3cb0, &[this, 1, 0, 0x77]);
        let log = e.call_log.take().unwrap();
        assert!(log.iter().any(|(a, _)| *a == ARRAY_SET_AT));
        assert!(!log.iter().any(|(a, _)| *a == SET_WORD_8));
    }

    #[test]
    fn destructor_frees_the_inputs_and_releases_every_array() {
        let mut e = Engine::new();
        e.register(MEMORY_DEALLOCATE, |_, _| Ret::default());
        e.register(ARRAY_SET_AT, |e, a| {
            assert_eq!(e.mem.u32(a[2]), 0);
            Ret::default()
        });
        e.register(RELEASE_BLOCK, |_, _| Ret::default());
        let this = e.mem.alloc(0x58);
        let inputs = e.mem.alloc(12);
        e.mem.set_u32(inputs, 0x111);
        e.mem.set_u32(inputs + 8, 0x333);
        e.mem.set_u32(this + 0x3c, inputs);
        e.mem.set_u16(this + 0x42, 3);
        for (member, block) in [
            (0x08u32, 0xa08u32),
            (0x18, 0xa18),
            (0x28, 0xa28),
            (0x48, 0xa48),
        ] {
            e.mem.set_u32(this + member + 4, block);
        }
        e.call_log = Some(vec![]);
        e.call(0x00ba_3d30, &[this]);
        let log = e.call_log.take().unwrap();
        let freed: Vec<_> = log
            .iter()
            .filter(|(a, _)| *a == MEMORY_DEALLOCATE)
            .map(|(_, w)| w[1])
            .collect();
        assert_eq!(freed, vec![0x111, 0x333]);
        let cleared: Vec<_> = log
            .iter()
            .filter(|(a, _)| *a == ARRAY_SET_AT)
            .map(|(_, w)| (w[0], w[1]))
            .collect();
        assert_eq!(cleared, vec![(this + 0x38, 0), (this + 0x38, 2)]);
        let released: Vec<_> = log
            .iter()
            .filter(|(a, _)| *a == RELEASE_BLOCK)
            .map(|(_, w)| w[0])
            .collect();
        assert_eq!(released, vec![0xa48, inputs, 0xa28, 0xa18, 0xa08]);
        assert_eq!(e.mem.u32(this), VTABLE);
        for member in [0x08u32, 0x18, 0x28, 0x38, 0x48] {
            assert_eq!(e.mem.u32(this + member), PRIMITIVE_ARRAY_VTABLE);
        }

        // An empty input array frees nothing.
        let this = e.mem.alloc(0x58);
        e.call_log = Some(vec![]);
        e.call(0x00ba_3d30, &[this]);
        let log = e.call_log.take().unwrap();
        assert!(!log.iter().any(|(a, _)| *a == MEMORY_DEALLOCATE));
        assert_eq!(log.iter().filter(|(a, _)| *a == RELEASE_BLOCK).count(), 5);
    }
}

#[cfg(test)]
mod n012_tests {
    use super::*;

    #[test]
    fn primitive_array_deleting_destructor_releases_and_frees() {
        let mut e = Engine::new();
        e.register(RELEASE_BLOCK, |_, _| Ret::default());
        e.register(MEMORY_DEALLOCATE, |_, _| Ret::default());
        let array = e.mem.alloc(0x10);
        e.mem.set_u32(array + 4, 0x5150);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00ba_3ce0, &args![array, 0u32]).u32(), array);
        assert_eq!(e.mem.u32(array), PRIMITIVE_ARRAY_VTABLE);
        e.call(0x00ba_3ce0, &args![array, 1u32]);
        let log = e.call_log.take().unwrap();
        let released: Vec<&Vec<u32>> = log
            .iter()
            .filter(|(a, _)| *a == RELEASE_BLOCK)
            .map(|(_, w)| w)
            .collect();
        assert_eq!(released, vec![&vec![0x5150], &vec![0x5150]]);
        let freed: Vec<&Vec<u32>> = log
            .iter()
            .filter(|(a, _)| *a == MEMORY_DEALLOCATE)
            .map(|(_, w)| w)
            .collect();
        assert_eq!(freed, vec![&vec![MEMORY_MANAGER, array]]);
    }
}

//! `BSShader/imagespaceeffectmenubg.cpp` (Xbox PDB source unit), subsystem `BSShader`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

use crate::units::bsshader::geometrydecalshader::shader_inline::{
    release_reference, MEMORY_ALLOCATE, MEMORY_MANAGER,
};

/// The effect-array sizer `00ba3f90` (unnamed; `thiscall`, one word), the `NiTArray` set-size
/// `00a90180` (`thiscall`, one word), the array setter `00ba3aa0` (`thiscall` on an
/// `NiTPrimitiveArray`: index, the address of the word), `BSShaderManager::GetTexture`
/// (`00b55840`, `cdecl`: file name, a word, the address of an `NiPointer`, two words),
/// `ImageSpaceTexture`'s set-texture for a texture that is not rendered (`00ba37f0`),
/// `ImageSpaceShaderParam`'s constructor (`00b8a9e0`), `InitConstantMap` (`00b8ab10`, two
/// words) and `SetPixelConstant` (`00b8a790`, five words), and the input and effect setters
/// `00ba3b50` and `00ba3c70` (unit `run_00ba3aa0`).
const SIZE_ARRAYS_00BA3F90: u32 = 0x00ba_3f90;
const ARRAY_SET_SIZE_00A90180: u32 = 0x00a9_0180;
const ARRAY_SET_AT_00BA3AA0: u32 = 0x00ba_3aa0;
const GET_TEXTURE_00B55840: u32 = 0x00b5_5840;
const TEXTURE_SET_PLAIN_00BA37F0: u32 = 0x00ba_37f0;
const PARAM_CONSTRUCT_00B8A9E0: u32 = 0x00b8_a9e0;
const PARAM_INIT_CONSTANT_MAP_00B8AB10: u32 = 0x00b8_ab10;
const PARAM_SET_PIXEL_CONSTANT_00B8A790: u32 = 0x00b8_a790;
const SET_INPUTS_00BA3B50: u32 = 0x00ba_3b50;
const SET_EFFECT_00BA3C70: u32 = 0x00ba_3c70;
/// The HDR byte (`011f941e`) and the file name `"Data\Textures\Interface\Misc\sepia.dds"`
/// (`010bca84`).
const HDR_ENABLED_011F941E: u32 = 0x011f_941e;
const SEPIA_FILE_010BCA84: u32 = 0x010b_ca84;

// Translated from 00c04730 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c04730 - the C++ exception frame (FS:[0] chain, unwind states 0 and 1 for the texture pointer and the parameter allocation) is not translated.
/// `ImageSpaceEffectMenuBG::Setup` (Xbox PDB), `thiscall`, two words (the manager; a word it
/// does not read). Sizes the arrays to 1 (`00ba3f90`) and `pTextures` to 3; makes an
/// `ImageSpaceTexture` (0x10 bytes from the memory manager, cleared as its constructor does;
/// null when the allocation fails) and keeps it at +0x58. Without the HDR byte `011f941e` the
/// texture takes the sepia texture (`BSShaderManager::GetTexture` of `010bca84` with 1 into a
/// local `NiPointer`, then `00ba37f0`), and the local's reference is dropped. The texture
/// becomes texture 2 of `pTextures` (the array setter given the address +0x58). One parameter
/// (`InitConstantMap(0, 1)`, pixel constant 0 = (0, 0, 0, 0)) is stored as 0; input 0 is
/// textures 1 and 2 (filters 1 and 0) and effect 0 the manager's +0xc4.
pub fn image_space_effect_menu_bg_setup(e: &mut Engine, this: Ptr, manager: u32, _unused_1: u32) {
    let base = this.addr();
    e.call(SIZE_ARRAYS_00BA3F90, &args![base, 1u32]);
    e.call(ARRAY_SET_SIZE_00A90180, &args![base + 0x28, 3u32]);
    let block = e
        .call(MEMORY_ALLOCATE, &args![MEMORY_MANAGER, 0x10u32])
        .u32();
    let texture = if block != 0 {
        for offset in 0..3u32 {
            e.mem.set_u8(block + offset, 0);
        }
        for offset in [4u32, 8, 0xc] {
            e.mem.set_u32(block + offset, 0);
        }
        block
    } else {
        0
    };
    e.mem.set_u32(base + 0x58, texture);
    if e.global::<u8>(HDR_ENABLED_011F941E) == 0 {
        e.with_stack(4, |e, local| {
            let local = local.addr();
            e.call(
                GET_TEXTURE_00B55840,
                &args![SEPIA_FILE_010BCA84, 1u32, local, 0u32, 0u32],
            );
            let loaded = e.mem.u32(local);
            let texture = e.mem.u32(base + 0x58);
            e.call(TEXTURE_SET_PLAIN_00BA37F0, &args![texture, loaded]);
            let loaded = e.mem.u32(local);
            if loaded != 0 {
                release_reference(e, loaded);
            }
        });
    }
    e.call(
        ARRAY_SET_AT_00BA3AA0,
        &args![base + 0x28, 2u32, base + 0x58],
    );
    let block = e
        .call(MEMORY_ALLOCATE, &args![MEMORY_MANAGER, 0x88u32])
        .u32();
    let param = if block != 0 {
        e.call(PARAM_CONSTRUCT_00B8A9E0, &args![block]).u32()
    } else {
        0
    };
    e.call(PARAM_INIT_CONSTANT_MAP_00B8AB10, &args![param, 0u32, 1u32]);
    e.call(
        PARAM_SET_PIXEL_CONSTANT_00B8A790,
        &args![param, 0u32, 0u32, 0u32, 0u32, 0u32],
    );
    e.with_stack(4, |e, local| {
        e.mem.set_u32(local.addr(), param);
        e.call(
            ARRAY_SET_AT_00BA3AA0,
            &args![base + 0x18, 0u32, local.addr()],
        );
    });
    e.call(
        SET_INPUTS_00BA3B50,
        &args![base, 0u32, 1u32, 2u32, 1u32, 0u32],
    );
    let effect = e.mem.u32(e.mem.u32(manager + 8) + 0xc4);
    e.call(SET_EFFECT_00BA3C70, &args![base, 0u32, effect, param, 0u32]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(
        0x00c0_4730,
        image_space_effect_menu_bg_setup(Ptr, u32, u32)
    )]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::bsshader::geometrydecalshader::shader_test_rig::*;

    type Stored = std::rc::Rc<std::cell::RefCell<Vec<(u32, u32, u32)>>>;

    /// Every callee doubled; the array setter records (array, index, the word it is given);
    /// `GetTexture` leaves the pointer null; parameters are 0x0e6c_0000.
    fn engine() -> (Engine, u32, u32, Stored) {
        let mut e = rig();
        e.map(0x011f_9000, 0x1000);
        e.register(PARAM_CONSTRUCT_00B8A9E0, |_, _| 0x0e6c_0000u32.into_ret());
        let stored: Stored = Default::default();
        let log = stored.clone();
        e.register_double(ARRAY_SET_AT_00BA3AA0, move |e, a| {
            log.borrow_mut().push((a[0], a[1], e.mem.u32(a[2])));
            Ret::default()
        });
        for callee in [
            GET_TEXTURE_00B55840,
            SIZE_ARRAYS_00BA3F90,
            ARRAY_SET_SIZE_00A90180,
            TEXTURE_SET_PLAIN_00BA37F0,
            PARAM_INIT_CONSTANT_MAP_00B8AB10,
            PARAM_SET_PIXEL_CONSTANT_00B8A790,
            SET_INPUTS_00BA3B50,
            SET_EFFECT_00BA3C70,
        ] {
            e.register(callee, |_, _| Ret::default());
        }
        let this = e.mem.alloc(0x5c);
        let manager = e.mem.alloc(0x10);
        let effects = e.mem.alloc(0xc8);
        e.mem.set_u32(effects + 0xc4, 0x0e6d_0000);
        e.mem.set_u32(manager + 8, effects);
        (e, this, manager, stored)
    }

    #[test]
    fn setup_loads_the_sepia_texture_without_hdr() {
        let (mut e, this, manager, stored) = engine();
        let loaded = object(&mut e, 1);
        e.register_double(GET_TEXTURE_00B55840, move |e, a| {
            e.mem.set_u32(a[2], loaded);
            Ret::default()
        });
        e.call(0x00c0_4730, &args![this, manager, 0u32]);
        let texture = e.mem.u32(this + 0x58);
        assert_ne!(texture, 0);
        assert_eq!(e.mem.bytes(texture, 0x10), vec![0; 0x10]);
        let gets = calls(&e, GET_TEXTURE_00B55840);
        assert_eq!(gets.len(), 1);
        assert_eq!(gets[0][..2], [SEPIA_FILE_010BCA84, 1]);
        assert_eq!(gets[0][3..], [0, 0]);
        assert_eq!(
            calls(&e, TEXTURE_SET_PLAIN_00BA37F0),
            vec![vec![texture, loaded]]
        );
        // The local pointer's reference dropped: 1 to 0, deleted.
        assert_eq!(calls(&e, REFERENCE_DELETE), vec![vec![loaded]]);
        assert_eq!(
            *stored.borrow(),
            vec![(this + 0x28, 2, texture), (this + 0x18, 0, 0x0e6c_0000)]
        );
        assert_eq!(
            calls(&e, SET_INPUTS_00BA3B50),
            vec![vec![this, 0, 1, 2, 1, 0]]
        );
        assert_eq!(
            calls(&e, SET_EFFECT_00BA3C70),
            vec![vec![this, 0, 0x0e6d_0000, 0x0e6c_0000, 0]]
        );
        assert_eq!(
            calls(&e, PARAM_SET_PIXEL_CONSTANT_00B8A790),
            vec![vec![0x0e6c_0000, 0, 0, 0, 0, 0]]
        );
        assert_eq!(calls(&e, SIZE_ARRAYS_00BA3F90), vec![vec![this, 1]]);
        assert_eq!(
            calls(&e, ARRAY_SET_SIZE_00A90180),
            vec![vec![this + 0x28, 3]]
        );
    }

    #[test]
    fn setup_with_hdr_keeps_the_texture_empty() {
        let (mut e, this, manager, stored) = engine();
        e.set_global(HDR_ENABLED_011F941E, 1u8);
        e.call(0x00c0_4730, &args![this, manager, 0u32]);
        assert!(calls(&e, GET_TEXTURE_00B55840).is_empty());
        assert!(calls(&e, TEXTURE_SET_PLAIN_00BA37F0).is_empty());
        let texture = e.mem.u32(this + 0x58);
        assert_eq!(stored.borrow()[0], (this + 0x28, 2, texture));
        // Without HDR and a failed load: no reference to drop.
        let (mut e, this, manager, _) = engine();
        e.call(0x00c0_4730, &args![this, manager, 0u32]);
        assert!(calls(&e, REFERENCE_DELETE).is_empty());
        assert_eq!(calls(&e, TEXTURE_SET_PLAIN_00BA37F0)[0][1], 0);
    }
}

//! `BSShader/run_00ba3aa0` (Xbox PDB source unit), subsystem `BSShader`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! These set an image-space effect's arrays at one index. `ImageSpaceEffect` (Xbox PDB; the
//! PC code uses the same offsets) keeps `NiTPrimitiveArray`s: `pEffectList` +0x08,
//! `pEffectParamList` +0x18, `pEffectInputs` +0x38 and `pEffectOutput` +0x48. An input is an
//! `ImageSpaceEffect::EffectInput` block (Xbox PDB: `iTexIndex`, `iFilterMode`, 8 bytes each)
//! allocated from the memory manager.

#[allow(unused_imports)]
use crate::prelude::*;

use crate::units::bsshader::geometrydecalshader::shader_inline::{MEMORY_ALLOCATE, MEMORY_MANAGER};

/// The array element setter `00ba3aa0` (unnamed; `thiscall` on an `NiTPrimitiveArray`: the
/// index, the address of the word to store).
const ARRAY_SET_AT_00BA3AA0: u32 = 0x00ba_3aa0;

/// `pairs` inputs allocated in one block (`8 * pairs` bytes, not checked for null), input `i`
/// taking `textures[i]` and `filters[i]`, then stored at `index` of `pEffectInputs` (+0x38)
/// through a local holding the block's address.
fn set_inputs(e: &mut Engine, this: Ptr, index: u32, textures: &[u32], filters: &[u32]) {
    let size = 8 * textures.len() as u32;
    let block = e.call(MEMORY_ALLOCATE, &args![MEMORY_MANAGER, size]).u32();
    for (i, (texture, filter)) in textures.iter().zip(filters).enumerate() {
        e.mem.set_u32(block + 8 * i as u32, *texture);
        e.mem.set_u32(block + 8 * i as u32 + 4, *filter);
    }
    e.with_stack(4, |e, local| {
        e.mem.set_u32(local.addr(), block);
        e.call(
            ARRAY_SET_AT_00BA3AA0,
            &args![this.addr() + 0x38, index, local.addr()],
        );
    });
}

// Translated from 00ba3b10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00ba3aa0`), `thiscall`, three words: one input (`texture`,
/// `filter`) for the effect's input `index`.
pub fn fn_00ba3b10(e: &mut Engine, this: Ptr, index: u32, texture: u32, filter: u32) {
    set_inputs(e, this, index, &[texture], &[filter]);
}

// Translated from 00ba3b50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00ba3aa0`), `thiscall`, five words: two inputs for the
/// effect's input `index`, the two textures first, then the two filter modes.
pub fn fn_00ba3b50(
    e: &mut Engine,
    this: Ptr,
    index: u32,
    texture_0: u32,
    texture_1: u32,
    filter_0: u32,
    filter_1: u32,
) {
    set_inputs(
        e,
        this,
        index,
        &[texture_0, texture_1],
        &[filter_0, filter_1],
    );
}

// Translated from 00ba3ba0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00ba3aa0`), `thiscall`, seven words: three inputs for the
/// effect's input `index`, the three textures first, then the three filter modes.
pub fn fn_00ba3ba0(
    e: &mut Engine,
    this: Ptr,
    index: u32,
    texture_0: u32,
    texture_1: u32,
    texture_2: u32,
    filter_0: u32,
    filter_1: u32,
    filter_2: u32,
) {
    set_inputs(
        e,
        this,
        index,
        &[texture_0, texture_1, texture_2],
        &[filter_0, filter_1, filter_2],
    );
}

// Translated from 00ba3c00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00ba3aa0`), `thiscall`, nine words: four inputs for the
/// effect's input `index`, the four textures first, then the four filter modes.
pub fn fn_00ba3c00(
    e: &mut Engine,
    this: Ptr,
    index: u32,
    texture_0: u32,
    texture_1: u32,
    texture_2: u32,
    texture_3: u32,
    filter_0: u32,
    filter_1: u32,
    filter_2: u32,
    filter_3: u32,
) {
    set_inputs(
        e,
        this,
        index,
        &[texture_0, texture_1, texture_2, texture_3],
        &[filter_0, filter_1, filter_2, filter_3],
    );
}

// Translated from 00ba3c70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00ba3aa0`), `thiscall`, four words: stores `effect`,
/// `parameter` and `output` at `index` of `pEffectList` (+0x08), `pEffectParamList` (+0x18) and
/// `pEffectOutput` (+0x48). The exe passes the addresses of its own three stack arguments;
/// here they are three consecutive words on the stack.
pub fn fn_00ba3c70(
    e: &mut Engine,
    this: Ptr,
    index: u32,
    effect: u32,
    parameter: u32,
    output: u32,
) {
    let base = this.addr();
    e.with_stack(0xc, |e, words| {
        let words = words.addr();
        e.mem.set_u32(words, effect);
        e.mem.set_u32(words + 4, parameter);
        e.mem.set_u32(words + 8, output);
        e.call(ARRAY_SET_AT_00BA3AA0, &args![base + 0x8, index, words]);
        e.call(ARRAY_SET_AT_00BA3AA0, &args![base + 0x18, index, words + 4]);
        e.call(ARRAY_SET_AT_00BA3AA0, &args![base + 0x48, index, words + 8]);
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00ba_3b10, fn_00ba3b10(Ptr, u32, u32, u32)),
        entry!(0x00ba_3b50, fn_00ba3b50(Ptr, u32, u32, u32, u32, u32)),
        entry!(
            0x00ba_3ba0,
            fn_00ba3ba0(Ptr, u32, u32, u32, u32, u32, u32, u32)
        ),
        entry!(
            0x00ba_3c00,
            fn_00ba3c00(Ptr, u32, u32, u32, u32, u32, u32, u32, u32, u32)
        ),
        entry!(0x00ba_3c70, fn_00ba3c70(Ptr, u32, u32, u32, u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    type Stored = std::rc::Rc<std::cell::RefCell<Vec<(u32, u32, u32)>>>;

    /// The allocator handing out fresh blocks and the array setter recording (array, index,
    /// the word its pointer argument points at).
    fn engine() -> (Engine, Stored) {
        let mut e = Engine::new();
        e.register(MEMORY_ALLOCATE, |e, a| e.mem.alloc(a[1]).into_ret());
        let stored: Stored = Default::default();
        let log = stored.clone();
        e.register_double(ARRAY_SET_AT_00BA3AA0, move |e, a| {
            log.borrow_mut().push((a[0], a[1], e.mem.u32(a[2])));
            Ret::default()
        });
        e.call_log = Some(vec![]);
        (e, stored)
    }

    fn allocations(e: &Engine) -> Vec<Vec<u32>> {
        let log = e.call_log.as_ref().unwrap();
        log.iter()
            .filter(|c| c.0 == MEMORY_ALLOCATE)
            .map(|c| c.1.clone())
            .collect()
    }

    /// The effect, the index and the words of the block stored at +0x38.
    fn stored_inputs(e: &Engine, stored: &Stored, this: u32, words: u32) -> (u32, Vec<u32>) {
        let stored = stored.borrow();
        assert_eq!(stored.len(), 1);
        let (array, index, block) = stored[0];
        assert_eq!(array, this + 0x38);
        let values = (0..words).map(|i| e.mem.u32(block + 4 * i)).collect();
        (index, values)
    }

    #[test]
    fn fn_00ba3b10_sets_one_input() {
        let (mut e, stored) = engine();
        let this = e.mem.alloc(0x58);
        e.call(0x00ba_3b10, &args![this, 2u32, 7u32, 1u32]);
        assert_eq!(allocations(&e), vec![vec![MEMORY_MANAGER, 8]]);
        assert_eq!(stored_inputs(&e, &stored, this, 2), (2, vec![7, 1]));
    }

    #[test]
    fn fn_00ba3b50_pairs_textures_with_filters() {
        let (mut e, stored) = engine();
        let this = e.mem.alloc(0x58);
        e.call(0x00ba_3b50, &args![this, 1u32, 10u32, 11u32, 20u32, 21u32]);
        assert_eq!(allocations(&e), vec![vec![MEMORY_MANAGER, 0x10]]);
        assert_eq!(
            stored_inputs(&e, &stored, this, 4),
            (1, vec![10, 20, 11, 21])
        );
    }

    #[test]
    fn fn_00ba3ba0_pairs_textures_with_filters() {
        let (mut e, stored) = engine();
        let this = e.mem.alloc(0x58);
        e.call(
            0x00ba_3ba0,
            &args![this, 0u32, 10u32, 11u32, 12u32, 20u32, 21u32, 22u32],
        );
        assert_eq!(allocations(&e), vec![vec![MEMORY_MANAGER, 0x18]]);
        assert_eq!(
            stored_inputs(&e, &stored, this, 6),
            (0, vec![10, 20, 11, 21, 12, 22])
        );
    }

    #[test]
    fn fn_00ba3c00_pairs_textures_with_filters() {
        let (mut e, stored) = engine();
        let this = e.mem.alloc(0x58);
        e.call(
            0x00ba_3c00,
            &args![this, 3u32, 10u32, 11u32, 12u32, 13u32, 20u32, 21u32, 22u32, 23u32],
        );
        assert_eq!(allocations(&e), vec![vec![MEMORY_MANAGER, 0x20]]);
        assert_eq!(
            stored_inputs(&e, &stored, this, 8),
            (3, vec![10, 20, 11, 21, 12, 22, 13, 23])
        );
    }

    #[test]
    fn fn_00ba3c70_sets_effect_parameter_and_output() {
        let (mut e, stored) = engine();
        let this = e.mem.alloc(0x58);
        e.call(
            0x00ba_3c70,
            &args![this, 4u32, 0x111u32, 0x222u32, 0x333u32],
        );
        assert_eq!(
            *stored.borrow(),
            vec![
                (this + 0x8, 4, 0x111),
                (this + 0x18, 4, 0x222),
                (this + 0x48, 4, 0x333),
            ]
        );
        assert!(allocations(&e).is_empty());
    }
}

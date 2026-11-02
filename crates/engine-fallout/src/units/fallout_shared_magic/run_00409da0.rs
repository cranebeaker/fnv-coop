//! `fallout shared/magic/run_00409da0` (Xbox PDB source unit), subsystem `fallout shared/magic`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 0040a110 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by 0x30 (`SUB ECX,0x30`) and jumps to `00409a70`. `00409a70` (`RET 0x4`) takes one stack word and leaves no deliberate value in `EAX` (its decompiled form is `void`), so the thunk returns nothing; the parameter count is from the `RET`. The stack word is passed on unchanged.
pub fn fn_0040a110(e: &mut Engine, this: Ptr, argument: u32) {
    e.call(
        0x0040_9a70,
        &args![this.addr().wrapping_sub(0x30), argument],
    );
}

// Translated from 0040a120 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by 0x30 (`SUB ECX,0x30`) and jumps to `00409a50`. `00409a50` (plain `RET`) returns 0 or 1 in `EAX` (`MOVZX EAX,AL; NEG EAX; SBB EAX,EAX; ADD EAX,1`), so the thunk returns `bool`; it takes no stack word.
pub fn fn_0040a120(e: &mut Engine, this: Ptr) -> bool {
    e.call(0x0040_9a50, &args![this.addr().wrapping_sub(0x30)])
        .bool()
}

// Translated from 0040a130 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by 0x4c (`SUB ECX,0x4c`) and jumps to `00409b10`. `00409b10` (`RET 4`) has the shape of a deleting destructor: it tests bit 0 of the flags word and returns the object pointer in `EAX`. The return type is `u32` from that `EAX`; the parameter count is from the `RET 4`. The stack word is passed on unchanged.
pub fn fn_0040a130(e: &mut Engine, this: Ptr, flags: u32) -> u32 {
    e.call(0x0040_9b10, &args![this.addr().wrapping_sub(0x4c), flags])
        .u32()
}

/// `TESModel::LoadModelChunk` (`004892d0`, cdecl: model, file).
const TES_MODEL_LOAD_MODEL_CHUNK: u32 = 0x0048_92d0;
/// `TESFile` current chunk size (`00401660`, thiscall, unnamed in the map).
const TES_FILE_CHUNK_SIZE: u32 = 0x0040_1660;
/// `TESFile::GetChunkData_ov3` (`00472890`, thiscall: buffer, size).
const TES_FILE_GET_CHUNK_DATA_SIZED: u32 = 0x0047_2890;
/// `00403570` (thiscall on the file, returns a `u16`; unnamed in the map).
const TES_FILE_HALF_00403570: u32 = 0x0040_3570;
/// `TESModelTextureSwap::UnpackTextureSwapChunkData` (`0048a7b0`, thiscall: data, size, word).
const TES_MODEL_TEXTURE_SWAP_UNPACK: u32 = 0x0048_a7b0;
/// `TESTexture::LoadTextureChunk` (`0048e3d0`, cdecl: texture, file).
const TES_TEXTURE_LOAD_TEXTURE_CHUNK: u32 = 0x0048_e3d0;
/// `TESFile::GetChunkData` (`004727f0`, thiscall: out pointer, `RET 4`).
const TES_FILE_GET_CHUNK_DATA: u32 = 0x0047_27f0;
/// `006ecd40` (thiscall, one word, `RET 4`): the map's name
/// (`CombatProcedureAttackMelee::Initialize`) comes from folded code.
const SET_FORM_VALUE_006ECD40: u32 = 0x006e_cd40;
/// `TESScriptableForm::InitItem` (`0048cd90`, thiscall: owner form).
const TES_SCRIPTABLE_FORM_INIT_ITEM: u32 = 0x0048_cd90;
/// `BGSDestructibleObjectForm::LoadChunk` (`004781e0`, cdecl: form, file).
const BGS_DESTRUCTIBLE_OBJECT_FORM_LOAD_CHUNK: u32 = 0x0047_81e0;
/// `TESForm::LoadData` (`004861f0`, thiscall: file, two words).
const TES_FORM_LOAD_DATA: u32 = 0x0048_61f0;

// Translated from 00409da0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00409da0 - the security-cookie check (00ec408c) is not translated.
/// Chunk loader of the form `fn_004098f0` constructs, called on the component at
/// form + 0x30 (`this`; the form is `this - 0x30`) with the plugin file and the chunk's
/// four-character type (`RET 8`). The engine map gives it no name. By type:
/// `MODL`, `MODD`, `MOSD`, `MODT`: `TESModel::LoadModelChunk` on `this + 0x1c` (null
/// when `this` is 0x30); `MODS`: reads the chunk into a stack buffer of the chunk's size
/// (`_alloca`) and unpacks it with `TESModelTextureSwap::UnpackTextureSwapChunkData` on
/// `this + 0x1c`; `ICON`: `TESTexture::LoadTextureChunk` on `this + 0x3c`; `ETYP`: reads
/// one word and stores it through `006ecd40` on `this + 0x5c`; `SCRI`: reads one word,
/// stores it through `006ecd40` on `this + 0x48`, then `TESScriptableForm::InitItem` there
/// with the form; `DEST`, `DSTD`: `BGSDestructibleObjectForm::LoadChunk` on `this + 0x64`;
/// `DATA`: `TESForm::LoadData` on the form. Other types do nothing.
pub fn fn_00409da0(e: &mut Engine, this: Ptr, file: Ptr, chunk: u32) {
    let base = this.addr();
    // The component pointer the cdecl loaders get: null when `this - 0x30` is null.
    let component = |offset: u32| {
        if base.wrapping_sub(0x30) == 0 {
            0
        } else {
            base.wrapping_add(offset)
        }
    };
    match chunk {
        // MODL, MOSD, MODD, MODT
        0x4c44_4f4d | 0x4453_4f4d | 0x4444_4f4d | 0x5444_4f4d => {
            e.call(TES_MODEL_LOAD_MODEL_CHUNK, &args![component(0x1c), file]);
        }
        // MODS
        0x5344_4f4d => {
            let size = e.call(TES_FILE_CHUNK_SIZE, &args![file]).u32();
            e.with_stack(size, |e, buffer| {
                e.call(TES_FILE_GET_CHUNK_DATA_SIZED, &args![file, buffer, size]);
                let word = u32::from(e.call(TES_FILE_HALF_00403570, &args![file]).u16());
                e.call(
                    TES_MODEL_TEXTURE_SWAP_UNPACK,
                    &args![base.wrapping_add(0x1c), buffer, size, word],
                );
            });
        }
        // ICON
        0x4e4f_4349 => {
            e.call(
                TES_TEXTURE_LOAD_TEXTURE_CHUNK,
                &args![component(0x3c), file],
            );
        }
        // ETYP
        0x5059_5445 => {
            let value = read_chunk_word(e, file);
            e.call(
                SET_FORM_VALUE_006ECD40,
                &args![base.wrapping_add(0x5c), value],
            );
        }
        // SCRI
        0x4952_4353 => {
            let value = read_chunk_word(e, file);
            e.call(
                SET_FORM_VALUE_006ECD40,
                &args![base.wrapping_add(0x48), value],
            );
            e.call(
                TES_SCRIPTABLE_FORM_INIT_ITEM,
                &args![base.wrapping_add(0x48), base.wrapping_sub(0x30)],
            );
        }
        // DSTD, DEST
        0x4454_5344 | 0x5453_4544 => {
            e.call(
                BGS_DESTRUCTIBLE_OBJECT_FORM_LOAD_CHUNK,
                &args![component(0x64), file],
            );
        }
        // DATA
        0x4154_4144 => {
            e.call(
                TES_FORM_LOAD_DATA,
                &args![base.wrapping_sub(0x30), file, 0u32, 0u32],
            );
        }
        _ => {}
    }
}

/// A zeroed stack word filled by `TESFile::GetChunkData`, as `fn_00409da0` reads `ETYP` and `SCRI`.
fn read_chunk_word(e: &mut Engine, file: Ptr) -> u32 {
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), 0);
        e.call(TES_FILE_GET_CHUNK_DATA, &args![file, slot]);
        e.mem.u32(slot.addr())
    })
}

// Translated from 0040a1c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor of a member (the engine map gives it no name; Ghidra shows a library-match
/// name): runs `0046ffb0` on `this + 4`.
pub fn fn_0040a1c0(e: &mut Engine, this: Ptr) {
    e.call(0x0046_ffb0, &args![this.addr() + 4]);
}

// Translated from 0040a140 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0040a140 - the exception-unwinding frame (FS:[0]) is not translated.
/// Destructor body (the engine map gives it no name; the class is unconfirmed): stores
/// the vtables `01011d0c` at +0 and `01011cf4` at +0xc, runs `00406340`
/// (`effectitemlist.cpp`) and `fn_0040a1c0` on the member at +0xc, then the base
/// destructor `00591300` on `this`.
pub fn fn_0040a140(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base, 0x0101_1d0c);
    e.mem.set_u32(base + 0xc, 0x0101_1cf4);
    e.call(0x0040_6340, &args![base + 0xc]);
    fn_0040a1c0(e, Ptr::new(base + 0xc));
    e.call(0x0059_1300, &args![this]);
}

/// `__RTDynamicCast` (CRT), cdecl (object, offset, source type, target type,
/// is-reference), and the type descriptors of `MagicItem` and
/// `IngredientItem` (their decorated names at +8).
const DYNAMIC_CAST: u32 = 0x00ec_43fb;
const TYPE_MAGIC_ITEM: u32 = 0x0118_3140;
const TYPE_INGREDIENT_ITEM: u32 = 0x0118_39dc;
/// Thiscall: the word at +0x9c (folded code).
const WORD_AT_9C: u32 = 0x0058_6150;
/// Thiscall (word), `RET 4`: stores the word at +0x9c
/// (`magicballprojectile.cpp`, folded code).
const SET_WORD_AT_9C: u32 = 0x0081_0570;
/// Thiscall (mask), `RET 4`: whether the flags byte at +0xa0 has a bit of
/// `mask` (`run_004098d0.rs`).
const FLAGS_AT_A0_HAVE: u32 = 0x0040_9a30;
/// `EffectItemList::GetCost` (`effectitemlist.rs`), thiscall (actor), `RET
/// 4`: a float in `ST0`.
const EFFECT_LIST_GET_COST: u32 = 0x0040_63e0;
/// Thiscall: swaps the byte order of the word at `this` (`00503210`).
const SWAP_BYTES_00503210: u32 = 0x0050_3210;

// Translated from 00409fd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall` on the `MagicItem` base at +0x30,
/// `RET 4`): whether `other` (a `MagicItem`) differs from this ingredient:
/// true when it is not an `IngredientItem` (`__RTDynamicCast`), or when its
/// word at +0x9c (`00586150`, `other`'s first) or its flags byte (+0xa0)
/// differs from this form's.
pub fn fn_00409fd0(e: &mut Engine, this: Ptr, other: u32) -> bool {
    let other = e
        .call(
            DYNAMIC_CAST,
            &args![other, 0u32, TYPE_MAGIC_ITEM, TYPE_INGREDIENT_ITEM, 0u32],
        )
        .u32();
    if other == 0 {
        return true;
    }
    let theirs = e.call(WORD_AT_9C, &args![other]).u32();
    let ours = e
        .call(WORD_AT_9C, &args![this.addr().wrapping_sub(0x30)])
        .u32();
    theirs != ours || e.mem.u8(other + 0xa0) != e.mem.u8(this.addr() + 0x70)
}

// Translated from 0040a040 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall` on the `MagicItem` base at +0x30,
/// `RET 4`): when `other` (a `MagicItem`) is an `IngredientItem`
/// (`__RTDynamicCast`), copies its word at +0x9c (`00810570`) and its flags
/// byte (+0xa0) into this form; otherwise nothing.
pub fn fn_0040a040(e: &mut Engine, this: Ptr, other: u32) {
    let other = e
        .call(
            DYNAMIC_CAST,
            &args![other, 0u32, TYPE_MAGIC_ITEM, TYPE_INGREDIENT_ITEM, 0u32],
        )
        .u32();
    if other == 0 {
        return;
    }
    let value = e.call(WORD_AT_9C, &args![other]).u32();
    e.call(
        SET_WORD_AT_9C,
        &args![this.addr().wrapping_sub(0x30), value],
    );
    let flags = e.mem.u8(other + 0xa0);
    e.mem.set_u8(this.addr() + 0x70, flags);
}

// Translated from 0040a0a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall` on the `EffectItemList` at +0x3c,
/// `RET 4`, a float in `ST0`): the ingredient's cost. With flag 1 of the
/// form's flags byte (`00409a30`) it is the word at +0x9c (`FILD`);
/// otherwise `EffectItemList::GetCost(0)` (the argument is not read).
pub fn fn_0040a0a0(e: &mut Engine, this: Ptr, _unused_1: u32) -> f64 {
    let form = this.addr().wrapping_sub(0x3c);
    if e.call(FLAGS_AT_A0_HAVE, &args![form, 1u32]).bool() {
        let value = e.call(WORD_AT_9C, &args![form]).i32();
        f64::from(value)
    } else {
        e.call(EFFECT_LIST_GET_COST, &args![this, 0u32]).f64()
    }
}

// Translated from 0040a0f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`): swaps the byte order of the word
/// at +0x9c (`00503210`).
pub fn fn_0040a0f0(e: &mut Engine, this: Ptr) {
    e.call(SWAP_BYTES_00503210, &args![this.addr() + 0x9c]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0040_a110, fn_0040a110(Ptr, u32)),
        entry!(0x0040_a120, fn_0040a120(Ptr) -> bool),
        entry!(0x0040_a130, fn_0040a130(Ptr, u32) -> u32),
        entry!(0x0040_9da0, fn_00409da0(Ptr, Ptr, u32)),
        entry!(0x0040_a1c0, fn_0040a1c0(Ptr)),
        entry!(0x0040_a140, fn_0040a140(Ptr)),
        entry!(0x0040_9fd0, fn_00409fd0(Ptr, u32) -> bool),
        entry!(0x0040_a040, fn_0040a040(Ptr, u32)),
        entry!(0x0040_a0a0, fn_0040a0a0(Ptr, u32) -> f64),
        entry!(0x0040_a0f0, fn_0040a0f0(Ptr)),
    ]
}

#[cfg(test)]
mod le81_thunk_tests {
    use crate::prelude::*;

    /// Calls `thunk` with `this = 0x2000` and `extra` stack words, with a
    /// double over `target` that returns `0xdeadbeef`; checks that the
    /// double ran once, with `this` moved back by `back` and `extra` passed on.
    fn check(thunk: u32, target: u32, back: u32, extra: &[u32]) -> Ret {
        let mut e = Engine::new();
        e.register(target, |_, _| Ret {
            eax: 0xdead_beef,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        let mut words = vec![0x2000u32];
        words.extend_from_slice(extra);
        let result = e.call(thunk, &words);
        let log = e.call_log.take().unwrap();
        let mut expected = vec![0x2000 - back];
        expected.extend_from_slice(extra);
        let calls: Vec<_> = log.iter().filter(|(a, _)| *a == target).collect();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].1, expected);
        result
    }

    #[test]
    fn le81_thunk_0040a110() {
        check(0x0040_a110, 0x0040_9a70, 0x30, &[0x77]);
    }

    #[test]
    fn le81_thunk_0040a120() {
        assert!(check(0x0040_a120, 0x0040_9a50, 0x30, &[]).bool());
    }

    #[test]
    fn le81_thunk_0040a130() {
        assert_eq!(
            check(0x0040_a130, 0x0040_9b10, 0x4c, &[0x1]).u32(),
            0xdead_beef
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    type Calls = Vec<(u32, Vec<u32>)>;

    fn noop(_: &mut Engine, _: &[u32]) -> Ret {
        Ret::default()
    }

    /// Runs `addr` with `words`, every callee in `callees` a no-op,
    /// and returns the calls other than the top-level one.
    fn run(e: &mut Engine, addr: u32, words: &[u32], callees: &[u32]) -> Calls {
        for c in callees {
            e.register(*c, noop);
        }
        e.call_log = Some(vec![]);
        e.call(addr, words);
        e.call_log
            .take()
            .unwrap()
            .into_iter()
            .filter(|c| c.0 != addr)
            .collect()
    }

    #[test]
    fn fn_00409da0_dispatches_each_chunk_type() {
        let file = 0x5000u32;
        let this = 0x6030u32;
        let loaders = [
            TES_MODEL_LOAD_MODEL_CHUNK,
            TES_TEXTURE_LOAD_TEXTURE_CHUNK,
            SET_FORM_VALUE_006ECD40,
            TES_SCRIPTABLE_FORM_INIT_ITEM,
            BGS_DESTRUCTIBLE_OBJECT_FORM_LOAD_CHUNK,
            TES_FORM_LOAD_DATA,
            TES_FILE_GET_CHUNK_DATA_SIZED,
            TES_MODEL_TEXTURE_SWAP_UNPACK,
        ];
        let cases: [(u32, Calls); 11] = [
            (
                0x4c44_4f4d,
                vec![(TES_MODEL_LOAD_MODEL_CHUNK, vec![this + 0x1c, file])],
            ),
            (
                0x4453_4f4d,
                vec![(TES_MODEL_LOAD_MODEL_CHUNK, vec![this + 0x1c, file])],
            ),
            (
                0x4444_4f4d,
                vec![(TES_MODEL_LOAD_MODEL_CHUNK, vec![this + 0x1c, file])],
            ),
            (
                0x5444_4f4d,
                vec![(TES_MODEL_LOAD_MODEL_CHUNK, vec![this + 0x1c, file])],
            ),
            (
                0x4e4f_4349,
                vec![(TES_TEXTURE_LOAD_TEXTURE_CHUNK, vec![this + 0x3c, file])],
            ),
            (
                0x5059_5445,
                vec![(SET_FORM_VALUE_006ECD40, vec![this + 0x5c, 0x55])],
            ),
            (
                0x4952_4353,
                vec![
                    (SET_FORM_VALUE_006ECD40, vec![this + 0x48, 0x55]),
                    (
                        TES_SCRIPTABLE_FORM_INIT_ITEM,
                        vec![this + 0x48, this - 0x30],
                    ),
                ],
            ),
            (
                0x4454_5344,
                vec![(
                    BGS_DESTRUCTIBLE_OBJECT_FORM_LOAD_CHUNK,
                    vec![this + 0x64, file],
                )],
            ),
            (
                0x5453_4544,
                vec![(
                    BGS_DESTRUCTIBLE_OBJECT_FORM_LOAD_CHUNK,
                    vec![this + 0x64, file],
                )],
            ),
            (
                0x4154_4144,
                vec![(TES_FORM_LOAD_DATA, vec![this - 0x30, file, 0, 0])],
            ),
            (0x4c44_4f4e, vec![]),
        ];
        for (chunk, want) in cases {
            let mut e = Engine::new();
            e.register(TES_FILE_GET_CHUNK_DATA, |e, a| {
                e.mem.set_u32(a[1], 0x55);
                Ret::default()
            });
            let calls = run(&mut e, 0x0040_9da0, &[this, file, chunk], &loaders);
            let calls: Calls = calls
                .into_iter()
                .filter(|c| c.0 != TES_FILE_GET_CHUNK_DATA)
                .collect();
            assert_eq!(calls, want, "chunk {chunk:08x}");
        }
    }

    #[test]
    fn fn_00409da0_passes_null_components_when_the_form_is_null() {
        let mut e = Engine::new();
        let calls = run(
            &mut e,
            0x0040_9da0,
            &[0x30, 0x5000, 0x4c44_4f4d],
            &[TES_MODEL_LOAD_MODEL_CHUNK],
        );
        assert_eq!(calls, vec![(TES_MODEL_LOAD_MODEL_CHUNK, vec![0, 0x5000])]);
        let calls = run(
            &mut e,
            0x0040_9da0,
            &[0x30, 0x5000, 0x4e4f_4349],
            &[TES_TEXTURE_LOAD_TEXTURE_CHUNK],
        );
        assert_eq!(
            calls,
            vec![(TES_TEXTURE_LOAD_TEXTURE_CHUNK, vec![0, 0x5000])]
        );
        let calls = run(
            &mut e,
            0x0040_9da0,
            &[0x30, 0x5000, 0x5453_4544],
            &[BGS_DESTRUCTIBLE_OBJECT_FORM_LOAD_CHUNK],
        );
        assert_eq!(
            calls,
            vec![(BGS_DESTRUCTIBLE_OBJECT_FORM_LOAD_CHUNK, vec![0, 0x5000])]
        );
    }

    #[test]
    fn fn_00409da0_unpacks_the_texture_swap_chunk_from_a_stack_buffer() {
        let mut e = Engine::new();
        e.register(TES_FILE_CHUNK_SIZE, |_, _| Ret {
            eax: 8,
            ..Ret::default()
        });
        e.register(TES_FILE_HALF_00403570, |_, _| Ret {
            eax: 0x0001_0007,
            ..Ret::default()
        });
        let calls = run(
            &mut e,
            0x0040_9da0,
            &[0x6030, 0x5000, 0x5344_4f4d],
            &[TES_FILE_GET_CHUNK_DATA_SIZED, TES_MODEL_TEXTURE_SWAP_UNPACK],
        );
        let read: Vec<_> = calls
            .iter()
            .filter(|c| c.0 == TES_FILE_GET_CHUNK_DATA_SIZED)
            .collect();
        let unpack: Vec<_> = calls
            .iter()
            .filter(|c| c.0 == TES_MODEL_TEXTURE_SWAP_UNPACK)
            .collect();
        assert_eq!(read.len(), 1);
        assert_eq!(unpack.len(), 1);
        let buffer = read[0].1[1];
        assert_eq!(read[0].1, vec![0x5000, buffer, 8]);
        assert_eq!(unpack[0].1, vec![0x6030 + 0x1c, buffer, 8, 7]);
    }

    #[test]
    fn fn_0040a1c0_destroys_the_member_at_four() {
        let mut e = Engine::new();
        let calls = run(&mut e, 0x0040_a1c0, &[0x7000], &[0x0046_ffb0]);
        assert_eq!(calls, vec![(0x0046_ffb0, vec![0x7004])]);
    }

    #[test]
    fn fn_0040a140_restores_vtables_and_destroys_members() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x20);
        let calls = run(
            &mut e,
            0x0040_a140,
            &[this],
            &[0x0040_6340, 0x0046_ffb0, 0x0059_1300],
        );
        assert_eq!(
            calls,
            vec![
                (0x0040_6340, vec![this + 0xc]),
                (0x0046_ffb0, vec![this + 0x10]),
                (0x0059_1300, vec![this]),
            ]
        );
        assert_eq!(e.mem.u32(this), 0x0101_1d0c);
        assert_eq!(e.mem.u32(this + 0xc), 0x0101_1cf4);
    }
}

#[cfg(test)]
mod ingredient_value_tests {
    use super::*;

    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, words)| words.clone())
            .collect()
    }

    /// The cast answers `a[0] - 0x30` unless it is 0x9000.
    fn engine() -> Engine {
        let mut e = Engine::new();
        e.register(DYNAMIC_CAST, |_, a| {
            if a[0] == 0x9000 {
                Ret::default()
            } else {
                (a[0] - 0x30).into_ret()
            }
        });
        e.register(WORD_AT_9C, |e, a| e.mem.u32(a[0] + 0x9c).into_ret());
        e.register(SET_WORD_AT_9C, |e, a| {
            e.mem.set_u32(a[0] + 0x9c, a[1]);
            Ret::default()
        });
        e
    }

    fn form(e: &mut Engine, value: u32, flags: u8) -> u32 {
        let form = e.mem.alloc(0xa4);
        e.mem.set_u32(form + 0x9c, value);
        e.mem.set_u8(form + 0xa0, flags);
        form
    }

    #[test]
    fn ingredients_differ_by_value_or_flags() {
        let mut e = engine();
        let this = form(&mut e, 7, 1);
        let same = form(&mut e, 7, 1);
        let other_value = form(&mut e, 8, 1);
        let other_flags = form(&mut e, 7, 3);
        assert!(!e.call(0x0040_9fd0, &args![this + 0x30, same + 0x30]).bool());
        assert!(fn_00409fd0(
            &mut e,
            Ptr::new(this + 0x30),
            other_value + 0x30
        ));
        assert!(fn_00409fd0(
            &mut e,
            Ptr::new(this + 0x30),
            other_flags + 0x30
        ));
        assert!(fn_00409fd0(&mut e, Ptr::new(this + 0x30), 0x9000));
        e.call_log = Some(vec![]);
        fn_00409fd0(&mut e, Ptr::new(this + 0x30), same + 0x30);
        assert_eq!(
            calls_to(&e, DYNAMIC_CAST),
            vec![vec![
                same + 0x30,
                0,
                TYPE_MAGIC_ITEM,
                TYPE_INGREDIENT_ITEM,
                0
            ]]
        );
    }

    #[test]
    fn copy_takes_value_and_flags() {
        let mut e = engine();
        let this = form(&mut e, 0, 0);
        let other = form(&mut e, 0x44, 0x81);
        e.call(0x0040_a040, &args![this + 0x30, other + 0x30]);
        assert_eq!(e.mem.u32(this + 0x9c), 0x44);
        assert_eq!(e.mem.u8(this + 0xa0), 0x81);
        e.call_log = Some(vec![]);
        fn_0040a040(&mut e, Ptr::new(this + 0x30), 0x9000);
        assert!(calls_to(&e, SET_WORD_AT_9C).is_empty());
    }

    #[test]
    fn cost_is_the_value_or_the_effects_cost() {
        let mut e = engine();
        e.register(FLAGS_AT_A0_HAVE, |e, a| {
            u32::from(u32::from(e.mem.u8(a[0] + 0xa0)) & a[1] != 0).into_ret()
        });
        e.register(EFFECT_LIST_GET_COST, |_, _| Ret {
            st0: 3.25,
            ..Ret::default()
        });
        let this = form(&mut e, (-5i32) as u32, 1);
        assert_eq!(e.call(0x0040_a0a0, &args![this + 0x3c, 9u32]).f64(), -5.0);
        e.mem.set_u8(this + 0xa0, 2);
        e.call_log = Some(vec![]);
        assert_eq!(fn_0040a0a0(&mut e, Ptr::new(this + 0x3c), 9), 3.25);
        assert_eq!(
            calls_to(&e, EFFECT_LIST_GET_COST),
            vec![vec![this + 0x3c, 0]]
        );
    }

    #[test]
    fn swaps_the_word_at_9c() {
        let mut e = Engine::new();
        e.register(SWAP_BYTES_00503210, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0040_a0f0, &args![0x1000u32]);
        assert_eq!(calls_to(&e, SWAP_BYTES_00503210), vec![vec![0x109c]]);
    }
}

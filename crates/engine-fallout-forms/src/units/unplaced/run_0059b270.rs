//! `(unplaced)/run_0059b270` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 0059b880 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0059b880 - the exception-unwinding frame (FS:[0]) and the security-cookie bookkeeping are not translated.
/// Constructor of the form `CreateFormOfType` (`00465110`) makes for form type 0x75 (0x2c bytes); `ECX` is the object.
/// The engine map gives it no name; the RTTI name of the vtable it stores at `+0` (`0x0103_4b4c`) is `TESCaravanDeck`, so this is the constructor of `TESCaravanDeck`.
/// It runs the base and component constructors by address, stores the vtable pointers and sets the fields to their start values.
/// The exception-unwinding frame is not translated.
pub fn fn_0059b880(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(0x0048_3370, &args![this]);
    e.call(0x0040_2d00, &args![base + 0x18]);
    e.mem.set_u32(base, 0x0103_4b4c);
    e.mem.set_u32(base + 0x18, 0x0103_4b38);
    e.call(0x004f_15a0, &args![this, 0x75u32]);
    e.call(0x0059_b9f0, &args![this]);
    this
}

// Translated from 0059b910 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor shape (the engine map gives it no name; the class is unconfirmed): runs the destructor body `0059b940` on `this`, then frees `this` with `00401030` when bit 0 of `flags` is set. Returns `this`.
pub fn fn_0059b910(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x0059_b940, &args![this]);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

use crate::units::fallout_shared::tescaravan::{
    cast_or_null, chunk_word, load_editor_id, CHUNK_DATA, CHUNK_EDID, CHUNK_FULL, CHUNK_ICON,
    CHUNK_MICO, CHUNK_MODD, CHUNK_MODL, CHUNK_MODS, CHUNK_MODT, CHUNK_MOSD, CHUNK_OBND, CHUNK_SIZE,
    CHUNK_YNAM, CHUNK_ZNAM, GET_CHUNK_DATA_SIZED, GET_TES_CHUNK, GET_TES_FORM, LOAD_FORM,
    NEXT_CHUNK, SET_FLAG_00484AB0,
};

/// Chunk types `MOD2` and `MOD3` (the four characters, little-endian).
const CHUNK_MOD2: u32 = 0x3244_4f4d;
const CHUNK_MOD3: u32 = 0x3344_4f4d;

/// The `MOD2`/`MOD3` chunks: the chunk into a stack buffer of its size (read with a size of
/// 0), the buffer given to vtable slot +0x18 of the model at `model`, then `004893e0`
/// (cdecl) on the model with the file.
fn load_extra_model(e: &mut Engine, file: Ptr, model: u32) {
    let size = e.call(CHUNK_SIZE, &args![file]).u32();
    e.with_stack(size, |e, buffer| {
        e.call(GET_CHUNK_DATA_SIZED, &args![file, buffer, 0u32]);
        e.vcall(model, 0x18, &args![buffer]);
    });
    e.call(0x0048_93e0, &args![model, file]);
}

// Translated from 0059b290 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0059b290 - the /GS cookie check (00ec408c) is not translated; the EDID, MODS, MOD2 and MOD3 buffers from __alloca_probe_16 (00ec5ec0) are with_stack blocks of the chunk size (their 16-byte alignment padding, and the stack growing with each such chunk, are not modelled)
/// The `TESCaravanMoney` `Load` (form type 0x74, the type its constructor `0059ae60`
/// gives; unnamed in the engine map; `ECX` and one stack word, the file, `RET 4`): false
/// unless `TESFile::GetTESForm`'s `AL` is 0x74. Otherwise `TESForm::LoadForm`, `00484ab0`
/// with 0, then each chunk:
/// - `EDID`: the editor ID (stack buffer, at most 0x200 bytes, vtable slot +0x134);
///   `OBND`: vtable slot +0xe0 with the file;
/// - `FULL`: `00487050` (cdecl) on +0x30; `MODL`, `MODT`, `MODD`, `MOSD`: `004892d0`
///   (cdecl) on +0x3c; `MODS`: the chunk into a stack buffer of its size, then `0048a7b0`
///   on +0x3c with the buffer, the size and `00403570`'s 16-bit count; `ICON`: `0048e3d0`
///   (cdecl) on +0x5c; `MICO`: `0047a290` (cdecl) on +0x68 (these casts null-checked);
/// - `YNAM`: a word given to `006ecd40` on +0x80; `ZNAM`: a word given to `00403550` on
///   +0x80;
/// - `DATA`: `004861f0` (`TESForm::LoadData`) with the file, 0, 0;
/// - `MOD2` and `MOD3`: the models at +0x8c and +0xac (see `load_extra_model`).
///
/// Then true.
pub fn fn_0059b290(e: &mut Engine, this: Ptr, file: Ptr) -> bool {
    if e.call(GET_TES_FORM, &args![file]).u8() != 0x74 {
        return false;
    }
    let base = this.addr();
    e.call(LOAD_FORM, &args![this, file]);
    e.call(SET_FLAG_00484AB0, &args![this, 0u32]);
    loop {
        let chunk = e.call(GET_TES_CHUNK, &args![file]).u32();
        if chunk == 0 {
            break;
        }
        match chunk {
            CHUNK_MODL | CHUNK_MODD | CHUNK_MOSD | CHUNK_MODT => {
                let model = cast_or_null(this, 0x3c);
                e.call(0x0048_92d0, &args![model, file]);
            }
            CHUNK_MOD2 => load_extra_model(e, file, base + 0x8c),
            CHUNK_MOD3 => load_extra_model(e, file, base + 0xac),
            CHUNK_DATA => {
                e.call(0x0048_61f0, &args![this, file, 0u32, 0u32]);
            }
            CHUNK_EDID => load_editor_id(e, this, file),
            CHUNK_OBND => {
                e.vcall(base, 0xe0, &args![file]);
            }
            CHUNK_ICON => {
                let texture = cast_or_null(this, 0x5c);
                e.call(0x0048_e3d0, &args![texture, file]);
            }
            CHUNK_FULL => {
                let name = cast_or_null(this, 0x30);
                e.call(0x0048_7050, &args![name, file]);
            }
            CHUNK_YNAM => {
                let word = chunk_word(e, file);
                e.call(0x006e_cd40, &args![base + 0x80, word]);
            }
            CHUNK_ZNAM => {
                let word = chunk_word(e, file);
                e.call(0x0040_3550, &args![base + 0x80, word]);
            }
            CHUNK_MICO => {
                let icon = cast_or_null(this, 0x68);
                e.call(0x0047_a290, &args![icon, file]);
            }
            CHUNK_MODS => {
                let size = e.call(CHUNK_SIZE, &args![file]).u32();
                e.with_stack(size, |e, buffer| {
                    e.call(GET_CHUNK_DATA_SIZED, &args![file, buffer, size]);
                    let count = u32::from(e.call(0x0040_3570, &args![file]).u16());
                    e.call(0x0048_a7b0, &args![base + 0x3c, buffer, size, count]);
                });
            }
            _ => {}
        }
        if !e.call(NEXT_CHUNK, &args![file]).bool() {
            break;
        }
    }
    true
}

// Translated from 0059b9c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`ECX` only, plain `RET`): runs `00470470` with the pointer at
/// +0x24 as `this` (also when it is null), then `00403d30` (cdecl) with `this+0x28`, 0 and
/// 4. The `TESCaravanDeck` destructor `0059b940` runs it first.
pub fn fn_0059b9c0(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    let list = e.mem.u32(base + 0x24);
    e.call(0x0047_0470, &args![list]);
    e.call(0x0040_3d30, &args![base + 0x28, 0u32, 4u32]);
}

// Translated from 0059b940 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0059b940 - the exception-unwinding frame (FS:[0] chain, handler 00ef8ac3, state variable) is not translated.
/// Destructor of `TESCaravanDeck` (the class whose vtable `0x0103_4b4c` the constructor
/// `0059b880` stores; unnamed in the engine map; `ECX` only, plain `RET`): stores the
/// vtables at +0 and +0x18 again, runs `0059b9c0`, then `00591300` on +0x18 and
/// `00483630` (the `TESForm` destructor) on the object.
pub fn fn_0059b940(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base, 0x0103_4b4c);
    e.mem.set_u32(base + 0x18, 0x0103_4b38);
    fn_0059b9c0(e, this);
    e.call(0x0059_1300, &args![base + 0x18]);
    e.call(0x0048_3630, &args![this]);
}

// Translated from 0059b9f0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0059b9f0 - the exception-unwinding frame (FS:[0] chain, handler 00eed64b, state variable) is not translated.
/// Unnamed in the engine map (`ECX` only, plain `RET`; the `TESCaravanDeck` constructor
/// `0059b880` runs it last): allocates 8 bytes through `00401000` (cdecl), constructs them
/// with `0096a2d0` (its `EAX` result is kept; 0 when the allocation failed), stores the
/// result at +0x24 and clears +0x28.
pub fn fn_0059b9f0(e: &mut Engine, this: Ptr) {
    let block = e.call(0x0040_1000, &args![8u32]).u32();
    let list = if block == 0 {
        0
    } else {
        e.call(0x0096_a2d0, &args![block]).u32()
    };
    let base = this.addr();
    e.mem.set_u32(base + 0x24, list);
    e.mem.set_u32(base + 0x28, 0);
}

/// RTTI type descriptor `.?AVTESCaravanMoney@@` (the source type is `TESForm`'s, from the
/// shared constants of the caravan unit's loader).
const TYPE_CARAVAN_MONEY: u32 = 0x0118_c5e4;
const TYPE_TES_FORM_DESCRIPTOR: u32 = 0x0118_3028;
/// `__RTDynamicCast` (`00ec43fb`).
const RT_DYNAMIC_CAST: u32 = 0x00ec_43fb;
/// `TESForm::CopyAllComponents` (`004851b0`) and `TESForm::CompareAllComponents`
/// (`00485270`), `ECX` and one stack word.
const COPY_ALL_COMPONENTS: u32 = 0x0048_51b0;
const COMPARE_ALL_COMPONENTS: u32 = 0x0048_5270;
/// `00408b20` (`cdecl`, two words: two strings): nonzero when they differ.
const STRINGS_DIFFER: u32 = 0x0040_8b20;
/// `00401170` (`ECX`): the form type byte; `004f15a0` (`ECX`, one stack word) sets it.
const FORM_TYPE_BYTE: u32 = 0x0040_1170;
const SET_FORM_TYPE: u32 = 0x004f_15a0;

/// The cast of `form` from `TESForm` to `TESCaravanMoney`, or null.
fn as_caravan_money(e: &mut Engine, form: Ptr) -> u32 {
    e.call(
        RT_DYNAMIC_CAST,
        &args![
            form,
            0u32,
            TYPE_TES_FORM_DESCRIPTOR,
            TYPE_CARAVAN_MONEY,
            0u32
        ],
    )
    .u32()
}

// Translated from 0059b270 (decompiled, FalloutNV.exe 1.4.0.525)
/// The address of the 0x20-byte model entry `index` at +0x8c of `this` (unnamed in the
/// engine map; `ECX` and one stack word, `RET 4`; called by `Save`, [`fn_0059b680`] and
/// [`fn_0059b740`]).
pub fn fn_0059b270(_e: &mut Engine, this: Ptr, index: u32) -> Ptr {
    Ptr::new(
        this.addr()
            .wrapping_add(0x8c)
            .wrapping_add(index.wrapping_shl(5)),
    )
}

// Translated from 0059b680 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESCaravanMoney`'s `Copy` slot (+0x108 of its vtable `010349b4`; unnamed in the engine
/// map; `ECX` and one stack word, `RET 4`): when `source` casts to a `TESCaravanMoney`,
/// `TESForm::CopyAllComponents` from it, each of the two model entries' path (the source
/// entry's vtable slot +0x14) set on this one's (+0x8c, +0xac: slot +0x18), and the form
/// type byte (`00401170` of `source`, into `004f15a0`).
pub fn fn_0059b680(e: &mut Engine, this: Ptr, source: Ptr) {
    let money = as_caravan_money(e, source);
    if money == 0 {
        return;
    }
    e.call(COPY_ALL_COMPONENTS, &args![this, money]);
    for (index, offset) in [(0u32, 0x8cu32), (1, 0xac)] {
        let entry = fn_0059b270(e, Ptr::new(money), index).addr();
        let path = e.vcall(entry, 0x14, &args![]).u32();
        e.vcall(this.addr() + offset, 0x18, &args![path]);
    }
    let kind = e.call(FORM_TYPE_BYTE, &args![source]).u32();
    e.call(SET_FORM_TYPE, &args![this, kind]);
}

// Translated from 0059b740 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESCaravanMoney`'s `Compare` slot (+0x10c of its vtable; unnamed in the engine map;
/// `ECX` and one stack word, `RET 4`): true (different) when `other` is not a
/// `TESCaravanMoney`, when `TESForm::CompareAllComponents` answers true, or when a model
/// entry's path differs (`00408b20(this one's, the other's)`, each the entry's slot
/// +0x14); false otherwise. The decompiler hangs the other's path on this one's getter; the
/// getter takes no stack word and the path is `00408b20`'s second argument.
pub fn fn_0059b740(e: &mut Engine, this: Ptr, other: Ptr) -> bool {
    let money = as_caravan_money(e, other);
    if money == 0 {
        return true;
    }
    if e.call(COMPARE_ALL_COMPONENTS, &args![this, money]).u8() != 0 {
        return true;
    }
    for (index, offset) in [(0u32, 0x8cu32), (1, 0xac)] {
        let entry = fn_0059b270(e, Ptr::new(money), index).addr();
        let theirs = e.vcall(entry, 0x14, &args![]).u32();
        let mine = e.vcall(this.addr() + offset, 0x14, &args![]).u32();
        if e.call(STRINGS_DIFFER, &args![mine, theirs]).u32() != 0 {
            return true;
        }
    }
    false
}

// Translated from 0059b820 (decompiled, FalloutNV.exe 1.4.0.525)
/// A vtable slot of three vtables (`01026528`, `010348d8`, `01034b00`; unnamed in the
/// engine map; `ECX` and two stack words, `RET 8`): `TESBoundObject::Clone3D` (`0050ee50`)
/// with the same words (the second as its byte), answering its `EAX`.
pub fn fn_0059b820(e: &mut Engine, this: Ptr, reference: u32, flag: u8) -> u32 {
    e.call(0x0050_ee50, &args![this, reference, u32::from(flag)])
        .u32()
}

// Translated from 0059b850 (decompiled, FalloutNV.exe 1.4.0.525)
/// A `SaveGame` slot of three vtables (`010260bc`, `01026434`, `01034a0c`; also called by
/// `0059bf90`; unnamed in the engine map; `ECX` and one stack word, `RET 4`): the base
/// save (`00484c20`), then `TESValueForm::SaveGame` (`0048e9e0`) on the component at +0x78.
pub fn fn_0059b850(e: &mut Engine, this: Ptr, buffer: u32) {
    e.call(0x0048_4c20, &args![this, buffer]);
    e.call(0x0048_e9e0, &args![this.addr() + 0x78, buffer]);
}

// Translated from 0059ba80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESCaravanDeck`'s form-save slot (+0x2c of its vtable `01034b4c`; unnamed in the engine
/// map; `ECX`, plain `RET`): `TESForm::StartForm`, `TESFullName::Save` on +0x18, a `CARD`
/// chunk (`00485910`) with the `0084e3a0` word of each card of the list `0059bb30`
/// answers (up to the first null one), the 4 bytes at +0x28 (`TESForm::__SaveData`),
/// byte-swapped before and back after when `00401500` says so (`00503210`), then
/// `TESForm::CloseForm`.
pub fn fn_0059ba80(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.call(0x0048_55a0, &args![this]);
    e.call(0x0048_7010, &args![base + 0x18]);
    let mut node = e.call(0x0059_bb30, &args![this]).u32();
    while node != 0 {
        let item = e.call(0x0068_15c0, &args![node]).u32();
        if e.mem.u32(item) == 0 {
            break;
        }
        let item = e.call(0x0068_15c0, &args![node]).u32();
        let card = e.mem.u32(item);
        let word = e.call(0x0084_e3a0, &args![card]).u32();
        e.call(0x0048_5910, &args![0x4452_4143u32, word]);
        node = e.call(0x0072_6070, &args![node]).u32();
    }
    if e.call(0x0040_1500, &args![]).u8() != 0 {
        e.call(0x0050_3210, &args![base + 0x28]);
    }
    e.call(0x0048_5e20, &args![this, base + 0x28, 4u32]);
    if e.call(0x0040_1500, &args![]).u8() != 0 {
        e.call(0x0050_3210, &args![base + 0x28]);
    }
    e.call(0x0048_5680, &args![this]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0059_b270, fn_0059b270(Ptr, u32) -> Ptr),
        entry!(0x0059_b680, fn_0059b680(Ptr, Ptr)),
        entry!(0x0059_b740, fn_0059b740(Ptr, Ptr) -> bool),
        entry!(0x0059_b820, fn_0059b820(Ptr, u32, u8) -> u32),
        entry!(0x0059_b850, fn_0059b850(Ptr, u32)),
        entry!(0x0059_ba80, fn_0059ba80(Ptr)),
        entry!(0x0059_b880, fn_0059b880(Ptr) -> Ptr),
        entry!(0x0059_b910, fn_0059b910(Ptr, u32) -> Ptr),
        entry!(0x0059_b290, fn_0059b290(Ptr, Ptr) -> bool),
        entry!(0x0059_b9c0, fn_0059b9c0(Ptr)),
        entry!(0x0059_b940, fn_0059b940(Ptr)),
        entry!(0x0059_b9f0, fn_0059b9f0(Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use crate::units::crt::ctor_test::{self, A};

    #[test]
    fn fn_0059b880_builds_the_object() {
        ctor_test::run(&ctor_test::Case {
            addr: 0x0059_b880,
            size: 0x2c,
            globals: &[],
            returns: &[],
            calls: &[
                (0x0048_3370, &[A::T(0x0)]),
                (0x0040_2d00, &[A::T(0x18)]),
                (0x004f_15a0, &[A::T(0x0), A::V(0x75)]),
                (0x0059_b9f0, &[A::T(0x0)]),
            ],
            stores: &[(0x0, 4, 0x0103_4b4c), (0x18, 4, 0x0103_4b38)],
        });
    }
    #[test]
    fn at_0059b910_runs_destructor_body_then_deletes_on_flag_one() {
        use crate::prelude::*;

        let mut e = Engine::new();
        let this: Ptr = Ptr::new(e.mem.alloc(16));
        e.register(0x0059_b940, |_, _| Ret::default());
        e.register(0x0040_1030, |_, _| Ret::default());
        for (flags, deletes) in [(0u32, false), (2, false), (1, true), (0xffff_ffff, true)] {
            e.call_log = Some(vec![]);
            let result = e.call(0x0059_b910, &args![this, flags]);
            assert_eq!(result.u32(), this.addr());
            let log = e.call_log.take().unwrap();
            let calls: Vec<(u32, Vec<u32>)> = log
                .into_iter()
                .filter(|(address, _)| *address != 0x0059_b910)
                .collect();
            let mut expected = vec![(0x0059_b940, vec![this.addr()])];
            if deletes {
                expected.push((0x0040_1030, vec![this.addr()]));
            }
            assert_eq!(calls, expected);
        }
    }

    use super::*;
    use std::cell::Cell;
    use std::rc::Rc;

    /// The calls a test made, without the top-level call and the file's chunk-walking calls.
    fn calls_after(e: &mut Engine, top: u32) -> Vec<(u32, Vec<u32>)> {
        e.call_log
            .take()
            .unwrap()
            .into_iter()
            .filter(|(address, _)| {
                ![top, GET_TES_FORM, GET_TES_CHUNK, NEXT_CHUNK].contains(address)
            })
            .collect()
    }

    /// Doubles for a file whose record has `form_type` and the given chunks, and no-op
    /// doubles for the loader's callees.
    fn file_with_chunks(e: &mut Engine, form_type: u32, chunks: Vec<u32>) {
        let position = Rc::new(Cell::new(0usize));
        e.register_double(GET_TES_FORM, move |_, _| Ret {
            eax: form_type,
            ..Ret::default()
        });
        let current = position.clone();
        let list = chunks.clone();
        e.register_double(GET_TES_CHUNK, move |_, _| Ret {
            eax: list.get(current.get()).copied().unwrap_or(0),
            ..Ret::default()
        });
        let count = chunks.len();
        e.register_double(NEXT_CHUNK, move |_, _| {
            position.set(position.get() + 1);
            Ret {
                eax: u32::from(position.get() < count),
                ..Ret::default()
            }
        });
        for address in [
            LOAD_FORM,
            SET_FLAG_00484AB0,
            0x0047_27f0,
            GET_CHUNK_DATA_SIZED,
            0x0048_92d0,
            0x0048_61f0,
            0x0048_e3d0,
            0x006e_cd40,
            0x0047_a290,
            0x0040_3550,
            0x0048_7050,
            0x0048_a7b0,
            0x0048_93e0,
            0x0040_3570,
        ] {
            e.register(address, |_, _| Ret::default());
        }
        e.register(CHUNK_SIZE, |_, _| Ret {
            eax: 0x20,
            ..Ret::default()
        });
    }

    #[test]
    fn caravan_money_load_refuses_another_form_type() {
        let mut e = Engine::new();
        let form = e.mem.alloc(0xcc);
        file_with_chunks(&mut e, 0x73, vec![CHUNK_DATA]);
        e.call_log = Some(vec![]);
        assert!(!e.call(0x0059_b290, &args![form, 0x5000u32]).bool());
        assert!(calls_after(&mut e, 0x0059_b290).is_empty());
    }

    #[test]
    fn caravan_money_load_dispatches_each_chunk() {
        let mut e = Engine::new();
        let form = e.mem.alloc(0xcc);
        let file = 0x5000u32;
        e.put_vtable(0x0300_0000, &[0; 0x80]);
        e.mem.set_u32(0x0300_0000 + 0xe0, 0x0300_10e0);
        e.mem.set_u32(0x0300_0000 + 0x134, 0x0300_1134);
        e.mem.set_u32(form, 0x0300_0000);
        e.put_vtable(0x0300_0400, &[0; 8]);
        e.mem.set_u32(0x0300_0400 + 0x18, 0x0300_1418);
        e.mem.set_u32(form + 0x8c, 0x0300_0400);
        e.mem.set_u32(form + 0xac, 0x0300_0400);
        for address in [0x0300_10e0u32, 0x0300_1134, 0x0300_1418] {
            e.register(address, |_, _| Ret::default());
        }
        file_with_chunks(
            &mut e,
            0x74,
            vec![
                CHUNK_MODL,
                CHUNK_MODT,
                CHUNK_MODD,
                CHUNK_MOSD,
                CHUNK_MOD2,
                CHUNK_MOD3,
                CHUNK_DATA,
                CHUNK_EDID,
                CHUNK_OBND,
                CHUNK_ICON,
                CHUNK_FULL,
                CHUNK_YNAM,
                CHUNK_ZNAM,
                CHUNK_MICO,
                CHUNK_MODS,
                0x5858_5858,
            ],
        );
        e.register(0x0047_27f0, |e, a| {
            e.mem.set_u32(a[1], 0x77);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        assert!(e.call(0x0059_b290, &args![form, file]).bool());
        let calls = calls_after(&mut e, 0x0059_b290);
        let buffers: Vec<_> = calls
            .iter()
            .filter(|call| call.0 == GET_CHUNK_DATA_SIZED)
            .map(|call| (call.1[1], call.1[2]))
            .collect();
        assert_eq!(buffers.len(), 4);
        let calls: Vec<_> = calls
            .into_iter()
            .filter(|call| {
                ![GET_CHUNK_DATA_SIZED, CHUNK_SIZE, 0x0047_27f0, 0x0040_3570].contains(&call.0)
            })
            .collect();
        assert_eq!(
            calls,
            vec![
                (LOAD_FORM, vec![form, file]),
                (SET_FLAG_00484AB0, vec![form, 0]),
                (0x0048_92d0, vec![form + 0x3c, file]),
                (0x0048_92d0, vec![form + 0x3c, file]),
                (0x0048_92d0, vec![form + 0x3c, file]),
                (0x0048_92d0, vec![form + 0x3c, file]),
                (0x0300_1418, vec![form + 0x8c, buffers[0].0]),
                (0x0048_93e0, vec![form + 0x8c, file]),
                (0x0300_1418, vec![form + 0xac, buffers[1].0]),
                (0x0048_93e0, vec![form + 0xac, file]),
                (0x0048_61f0, vec![form, file, 0, 0]),
                (0x0300_1134, vec![form, buffers[2].0]),
                (0x0300_10e0, vec![form, file]),
                (0x0048_e3d0, vec![form + 0x5c, file]),
                (0x0048_7050, vec![form + 0x30, file]),
                (0x006e_cd40, vec![form + 0x80, 0x77]),
                (0x0040_3550, vec![form + 0x80, 0x77]),
                (0x0047_a290, vec![form + 0x68, file]),
                (0x0048_a7b0, vec![form + 0x3c, buffers[3].0, 0x20, 0]),
            ]
        );
        assert_eq!(
            buffers.iter().map(|buffer| buffer.1).collect::<Vec<_>>(),
            vec![0, 0, 0x200, 0x20]
        );
    }

    #[test]
    fn fn_0059b9c0_releases_the_list_and_clears_the_count() {
        for list in [0u32, 0x6000] {
            let mut e = Engine::new();
            let deck = e.mem.alloc(0x2c);
            e.mem.set_u32(deck + 0x24, list);
            e.register(0x0047_0470, |_, _| Ret::default());
            e.register(0x0040_3d30, |_, _| Ret::default());
            e.call_log = Some(vec![]);
            e.call(0x0059_b9c0, &args![deck]);
            assert_eq!(
                calls_after(&mut e, 0x0059_b9c0),
                vec![
                    (0x0047_0470, vec![list]),
                    (0x0040_3d30, vec![deck + 0x28, 0, 4]),
                ]
            );
        }
    }

    #[test]
    fn fn_0059b940_stores_the_vtables_and_runs_the_destructors() {
        let mut e = Engine::new();
        let deck = e.mem.alloc(0x2c);
        e.mem.set_u32(deck + 0x24, 0x6000);
        for address in [0x0047_0470u32, 0x0040_3d30, 0x0059_1300, 0x0048_3630] {
            e.register(address, |_, _| Ret::default());
        }
        e.call_log = Some(vec![]);
        e.call(0x0059_b940, &args![deck]);
        assert_eq!(
            calls_after(&mut e, 0x0059_b940),
            vec![
                (0x0047_0470, vec![0x6000]),
                (0x0040_3d30, vec![deck + 0x28, 0, 4]),
                (0x0059_1300, vec![deck + 0x18]),
                (0x0048_3630, vec![deck]),
            ]
        );
        assert_eq!(e.mem.u32(deck), 0x0103_4b4c);
        assert_eq!(e.mem.u32(deck + 0x18), 0x0103_4b38);
    }

    #[test]
    fn fn_0059b9f0_allocates_and_constructs_the_list() {
        for (block, expected) in [(0u32, 0u32), (0x7000, 0x7004)] {
            let mut e = Engine::new();
            let deck = e.mem.alloc(0x2c);
            e.mem.set_u32(deck + 0x28, 0xdead);
            e.register_double(0x0040_1000, move |_, _| Ret {
                eax: block,
                ..Ret::default()
            });
            e.register(0x0096_a2d0, |_, a| Ret {
                eax: a[0] + 4,
                ..Ret::default()
            });
            e.call_log = Some(vec![]);
            e.call(0x0059_b9f0, &args![deck]);
            let calls = calls_after(&mut e, 0x0059_b9f0);
            assert_eq!(calls[0], (0x0040_1000, vec![8]));
            assert_eq!(calls.len(), if block == 0 { 1 } else { 2 });
            assert_eq!(e.mem.u32(deck + 0x24), expected);
            assert_eq!(e.mem.u32(deck + 0x28), 0);
        }
    }
}

#[cfg(test)]
mod n075_tests {
    use super::*;

    type Log = Vec<(u32, Vec<u32>)>;

    fn calls(log: &Log, callee: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|(a, _)| *a == callee)
            .map(|(_, w)| w.clone())
            .collect()
    }

    #[test]
    fn fn_0059b270_indexes_the_model_entries() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x0059_b270, &args![0x1000u32, 0u32]).u32(), 0x108c);
        assert_eq!(e.call(0x0059_b270, &args![0x1000u32, 1u32]).u32(), 0x10ac);
    }

    /// Two money forms whose model entries' slot +0x14 answer the entry's address plus 1.
    fn engine(cast: bool) -> (Engine, u32, u32) {
        let mut e = Engine::new();
        e.register(
            RT_DYNAMIC_CAST,
            if cast {
                |_, a| a[0].into_ret()
            } else {
                |_, _| 0u32.into_ret()
            },
        );
        e.register(0x0060_0014, |_, a| (a[0] + 1).into_ret());
        e.register(0x0060_0018, |_, _| Ret::default());
        e.put_vtable(0x0070_0000, &[0, 0, 0, 0, 0, 0x0060_0014, 0x0060_0018]);
        let this = e.mem.alloc(0xd0);
        let other = e.mem.alloc(0xd0);
        for form in [this, other] {
            e.mem.set_u32(form + 0x8c, 0x0070_0000);
            e.mem.set_u32(form + 0xac, 0x0070_0000);
        }
        for callee in [COPY_ALL_COMPONENTS, SET_FORM_TYPE] {
            e.register(callee, |_, _| Ret::default());
        }
        e.register(FORM_TYPE_BYTE, |_, _| 0x75u32.into_ret());
        e.call_log = Some(vec![]);
        (e, this, other)
    }

    #[test]
    fn fn_0059b680_copies_the_model_paths() {
        let (mut e, this, other) = engine(true);
        e.call(0x0059_b680, &args![this, other]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls(&log, COPY_ALL_COMPONENTS), vec![vec![this, other]]);
        assert_eq!(
            calls(&log, 0x0060_0018),
            vec![
                vec![this + 0x8c, other + 0x8d],
                vec![this + 0xac, other + 0xad]
            ]
        );
        assert_eq!(calls(&log, SET_FORM_TYPE), vec![vec![this, 0x75]]);
        let (mut e, this, other) = engine(false);
        e.call(0x0059_b680, &args![this, other]);
        assert_eq!(e.call_log.take().unwrap().len(), 2);
    }

    #[test]
    fn fn_0059b740_compares_components_and_paths() {
        let (mut e, this, other) = engine(true);
        e.register(COMPARE_ALL_COMPONENTS, |_, _| 0u32.into_ret());
        e.register(STRINGS_DIFFER, |_, _| 0u32.into_ret());
        assert!(!e.call(0x0059_b740, &args![this, other]).bool());
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls(&log, STRINGS_DIFFER),
            vec![
                vec![this + 0x8d, other + 0x8d],
                vec![this + 0xad, other + 0xad]
            ]
        );
        e.register(STRINGS_DIFFER, |_, a| {
            u32::from(a[0] & 0xff == 0xad).into_ret()
        });
        let differs = e.call(0x0059_b740, &args![this, other]).bool();
        assert_eq!(differs, (this + 0xad) & 0xff == 0xad);
        e.register(COMPARE_ALL_COMPONENTS, |_, _| 1u32.into_ret());
        assert!(e.call(0x0059_b740, &args![this, other]).bool());
        let (mut e, this, other) = engine(false);
        assert!(e.call(0x0059_b740, &args![this, other]).bool());
    }

    #[test]
    fn fn_0059b820_and_0059b850_forward() {
        let mut e = Engine::new();
        e.register(0x0050_ee50, |_, a| (a[0] + a[1] + a[2]).into_ret());
        assert_eq!(
            e.call(0x0059_b820, &args![0x1000u32, 0x20u32, 0x103u32])
                .u32(),
            0x1023
        );
        e.register(0x0048_4c20, |_, _| Ret::default());
        e.register(0x0048_e9e0, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0059_b850, &args![0x1000u32, 0x9000u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log[1..],
            [
                (0x0048_4c20, vec![0x1000, 0x9000]),
                (0x0048_e9e0, vec![0x1078, 0x9000])
            ]
        );
    }

    #[test]
    fn fn_0059ba80_saves_the_deck() {
        for swaps in [false, true] {
            let mut e = Engine::new();
            for callee in [
                0x0048_55a0u32,
                0x0048_7010,
                0x0048_5910,
                0x0050_3210,
                0x0048_5e20,
                0x0048_5680,
            ] {
                e.register(callee, |_, _| Ret::default());
            }
            e.register(
                0x0040_1500,
                if swaps {
                    |_, _| 1u32.into_ret()
                } else {
                    |_, _| 0u32.into_ret()
                },
            );
            let second = e.mem.alloc(8);
            let first = e.mem.alloc(8);
            e.mem.set_u32(first, 0x111);
            e.mem.set_u32(first + 4, second);
            e.register_double(0x0059_bb30, move |_, _| first.into_ret());
            e.register(0x0068_15c0, |_, a| a[0].into_ret());
            e.register(0x0072_6070, |e, a| e.mem.u32(a[0] + 4).into_ret());
            e.register(0x0084_e3a0, |_, a| (a[0] + 1).into_ret());
            e.call_log = Some(vec![]);
            e.call(0x0059_ba80, &args![0x1000u32]);
            let log = e.call_log.take().unwrap();
            assert_eq!(calls(&log, 0x0048_5910), vec![vec![0x4452_4143, 0x112]]);
            assert_eq!(calls(&log, 0x0048_7010), vec![vec![0x1018]]);
            assert_eq!(calls(&log, 0x0048_5e20), vec![vec![0x1000, 0x1028, 4]]);
            assert_eq!(calls(&log, 0x0050_3210).len(), if swaps { 2 } else { 0 });
            assert_eq!(log.last().unwrap(), &(0x0048_5680, vec![0x1000]));
        }
    }
}

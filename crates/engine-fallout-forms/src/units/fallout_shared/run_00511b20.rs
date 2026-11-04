//! `fallout shared/run_00511b20` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00511e30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by 0x18 (`SUB ECX,0x18`) and jumps to `005119e0`. `005119e0` (`RET 4`) has the shape of a deleting destructor: it tests bit 0 of the flags word and returns the object pointer in `EAX`. The return type is `u32` from that `EAX`; the parameter count is from the `RET 4`. The stack word is passed on unchanged.
pub fn fn_00511e30(e: &mut Engine, this: Ptr, flags: u32) -> u32 {
    e.call(0x0051_19e0, &args![this.addr().wrapping_sub(0x18), flags])
        .u32()
}

// Translated from 00511f30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor shape (the engine map gives it no name; the class is unconfirmed): runs the destructor body `00511f60` on `this`, then frees `this` with `00401030` when bit 0 of `flags` is set. Returns `this`.
pub fn fn_00511f30(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x0051_1f60, &args![this]);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

/// `TESFile::GetTESForm` (`00472660`, engine map): the current record's
/// form type byte.
const GET_TES_FORM: u32 = 0x0047_2660;
/// `TESForm::LoadForm` (`00485110`, engine map).
const TES_FORM_LOAD_FORM: u32 = 0x0048_5110;
/// `TESFile::GetTESChunk` (`004726b0`, engine map): 0 at the end.
const GET_TES_CHUNK: u32 = 0x0047_26b0;
/// `004726f0`: moves to the next chunk; false when there is none.
const NEXT_CHUNK: u32 = 0x0047_26f0;
/// `TESFile::GetChunkData` (`004727f0`, engine map): one word.
const GET_CHUNK_WORD: u32 = 0x0047_27f0;
/// `TESFile::GetChunkData_ov3` (`00472890`, engine map): buffer and size.
const GET_CHUNK_DATA: u32 = 0x0047_2890;
/// `00401660`: the current chunk's size.
const CHUNK_SIZE: u32 = 0x0040_1660;
/// `00403570`: the word `UnpackTextureSwapChunkData` takes (`ECX` = the
/// file, `AX`).
const FILE_WORD: u32 = 0x0040_3570;
/// `TESModel::LoadModelChunk` (`004892d0`, engine map): cdecl, model and
/// file.
const LOAD_MODEL_CHUNK: u32 = 0x0048_92d0;
/// `TESModelTextureSwap::UnpackTextureSwapChunkData` (`0048a7b0`, engine
/// map).
const UNPACK_TEXTURE_SWAP: u32 = 0x0048_a7b0;
/// The record type `fn_00511b20` accepts.
const FORM_TYPE_ARMA_RECORD: u32 = 0x4d;

const CHUNK_EDID: u32 = 0x4449_4445;
const CHUNK_DATA: u32 = 0x4154_4144;
const CHUNK_OBND: u32 = 0x444e_424f;
const CHUNK_MODL: u32 = 0x4c44_4f4d;
const CHUNK_MODD: u32 = 0x4444_4f4d;
const CHUNK_MOSD: u32 = 0x4453_4f4d;
const CHUNK_MODT: u32 = 0x5444_4f4d;
const CHUNK_MODS: u32 = 0x5344_4f4d;

/// The `TESObjectARMA` vtables (RTTI `.?AVTESObjectARMA@@` for each) and
/// the offsets they go to.
const ARMA_VTABLES: [(u32, u32); 13] = [
    (0x0, 0x0102_a31c),
    (0x30, 0x0102_a304),
    (0x3c, 0x0102_a2f0),
    (0x48, 0x0102_a2d8),
    (0x58, 0x0102_a2b8),
    (0x60, 0x0102_a2a4),
    (0x68, 0x0102_a28c),
    (0x70, 0x0102_a278),
    (0x14c, 0x0102_a264),
    (0x154, 0x0102_a250),
    (0x15c, 0x0102_a23c),
    (0x164, 0x0102_a228),
    (0x16c, 0x0102_a214),
];

fn store_arma_vtables(e: &mut Engine, this: Ptr) {
    for (offset, vtable) in ARMA_VTABLES {
        e.mem.set_u32(this.addr() + offset, vtable);
    }
}

// Translated from 00511b20 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00511b20 - the /GS security-cookie check (00ec408c) is not translated; the __alloca_probe_16 (00ec5ec0) buffers are with_stack blocks of the chunk size rounded up to 16 bytes.
/// The `TESObjectARMA` record reader (`file`, one stack word): false unless
/// the record's type (`TESFile::GetTESForm`) is 0x4D; then
/// `TESForm::LoadForm` and each chunk until none is left: `EDID` (at most
/// 0x200 bytes) to vtable slot 0x134; `OBND` to slot 0xE0 with the file;
/// `MODL`, `MODD`, `MOSD`, `MODT` to `TESModel::LoadModelChunk` on +0x18
/// (null when `this` is); `MODS` read whole into a stack buffer and
/// unpacked as a texture swap on +0x18 with `00403570`'s word; `DATA`, one
/// word, into +0x38. Returns true.
pub fn fn_00511b20(e: &mut Engine, this: Ptr, file: Ptr) -> bool {
    let base = this.addr();
    if e.call(GET_TES_FORM, &args![file]).u8() as u32 != FORM_TYPE_ARMA_RECORD {
        return false;
    }
    e.call(TES_FORM_LOAD_FORM, &args![this, file]);
    loop {
        let chunk = e.call(GET_TES_CHUNK, &args![file]).u32();
        if chunk == 0 {
            break;
        }
        match chunk {
            CHUNK_EDID => {
                let size = e.call(CHUNK_SIZE, &args![file]).u32();
                let reserved = size.wrapping_add(15) & !15;
                e.with_stack(reserved, |e, buffer| {
                    e.call(GET_CHUNK_DATA, &args![file, buffer, 0x200u32]);
                    e.vcall(base, 0x134, &args![buffer]);
                });
            }
            CHUNK_OBND => {
                e.vcall(base, 0xe0, &args![file]);
            }
            CHUNK_MODL | CHUNK_MODD | CHUNK_MOSD | CHUNK_MODT => {
                let model = if base == 0 { 0 } else { base + 0x18 };
                e.call(LOAD_MODEL_CHUNK, &args![model, file]);
            }
            CHUNK_MODS => {
                let size = e.call(CHUNK_SIZE, &args![file]).u32();
                let reserved = size.wrapping_add(15) & !15;
                e.with_stack(reserved, |e, buffer| {
                    e.call(GET_CHUNK_DATA, &args![file, buffer, size]);
                    let word = e.call(FILE_WORD, &args![file]).u16();
                    e.call(
                        UNPACK_TEXTURE_SWAP,
                        &args![base + 0x18, buffer, size, u32::from(word)],
                    );
                });
            }
            CHUNK_DATA => {
                let value = e.with_stack(4, |e, local| {
                    e.call(GET_CHUNK_WORD, &args![file, local]);
                    e.mem.u32(local.addr())
                });
                e.mem.set_u32(base + 0x38, value);
            }
            _ => {}
        }
        if !e.call(NEXT_CHUNK, &args![file]).bool() {
            break;
        }
    }
    true
}

// Translated from 00511e40 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00511e40 - the exception-unwinding frame (FS:[0]) and the security-cookie word it pushes are not translated.
/// `TESObjectARMA` constructor (the class of every vtable it stores, by
/// their RTTI): the base constructor `005129f0`, the thirteen vtables, the
/// form type 0x60 (`004f15a0`) and [`fn_00512010`]. Returns `this`.
pub fn fn_00511e40(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(0x0051_29f0, &args![this]);
    store_arma_vtables(e, this);
    e.call(0x004f_15a0, &args![this, 0x60u32]);
    fn_00512010(e, this);
    this
}

// Translated from 00511f60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectARMA` destructor body: stores the thirteen vtables back, then
/// the base destructor `00512bd0`.
pub fn fn_00511f60(e: &mut Engine, this: Ptr) {
    store_arma_vtables(e, this);
    e.call(0x0051_2bd0, &args![this]);
}

// Translated from 00512010 (decompiled, FalloutNV.exe 1.4.0.525)
/// Clears the 12 bytes at `this + 0x178` (`00403d30`(dst, 0, 0xC)).
pub fn fn_00512010(e: &mut Engine, this: Ptr) {
    e.call(0x0040_3d30, &args![this.addr() + 0x178, 0u32, 0xcu32]);
}

/// `__RTDynamicCast` (`00ec43fb`, `cdecl`, five words) from `.?AVTESForm@@`
/// (`01183028`) to `.?AVTESObjectANIO@@` (`01186244`).
const RT_DYNAMIC_CAST: u32 = 0x00ec_43fb;
const TYPE_TES_FORM: u32 = 0x0118_3028;
const TYPE_TES_OBJECT_ANIO: u32 = 0x0118_6244;
/// `TESForm::StartForm`, `TESForm::CloseForm` (`004855a0`, `00485680`,
/// `thiscall`), `TESModelTextureSwap::Save` (`0048a520`, three chunk names),
/// `TESForm::CopyAllComponents` and `CompareAllComponents` (`004851b0`,
/// `00485270`, one word) (engine map); `0084e3a0` (`thiscall`) and the
/// chunk writer `00485910` (`cdecl`: chunk name, word).
const START_FORM: u32 = 0x0048_55a0;
const CLOSE_FORM: u32 = 0x0048_5680;
const MODEL_SAVE: u32 = 0x0048_a520;
const COPY_ALL_COMPONENTS: u32 = 0x0048_51b0;
const COMPARE_ALL_COMPONENTS: u32 = 0x0048_5270;
const MEMBER_0084E3A0: u32 = 0x0084_e3a0;
const WRITE_CHUNK_WORD: u32 = 0x0048_5910;

/// `form` cast to `TESObjectANIO`, or 0.
fn cast_to_anio(e: &mut Engine, form: u32) -> u32 {
    e.call(
        RT_DYNAMIC_CAST,
        &args![form, 0u32, TYPE_TES_FORM, TYPE_TES_OBJECT_ANIO, 0u32],
    )
    .u32()
}

// Translated from 00511d10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`; the form's save): `StartForm`,
/// the model at +0x18 saved with the chunk names `MODL`, `MODT`, `MODS`,
/// then, when the word at +0x38 is set, a `DATA` chunk with what
/// `0084e3a0` answers for it; `CloseForm`.
pub fn fn_00511d10(e: &mut Engine, this: Ptr) {
    e.call(START_FORM, &args![this]);
    e.call(
        MODEL_SAVE,
        &args![this.addr() + 0x18, CHUNK_MODL, CHUNK_MODT, CHUNK_MODS],
    );
    let member = e.mem.u32(this.addr() + 0x38);
    if member != 0 {
        let value = e.call(MEMBER_0084E3A0, &args![member]).u32();
        e.call(WRITE_CHUNK_WORD, &args![CHUNK_DATA, value]);
    }
    e.call(CLOSE_FORM, &args![this]);
}

// Translated from 00511d70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, one word, `RET 4`; the form's
/// copy): when `source` casts to `TESObjectANIO`,
/// `TESForm::CopyAllComponents(source)` and the word at +0x38 copied from
/// the cast form.
pub fn fn_00511d70(e: &mut Engine, this: Ptr, source: u32) {
    let other = cast_to_anio(e, source);
    if other == 0 {
        return;
    }
    e.call(COPY_ALL_COMPONENTS, &args![this, source]);
    let value = e.mem.u32(other + 0x38);
    e.mem.set_u32(this.addr() + 0x38, value);
}

// Translated from 00511dc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, one word, `RET 4`): whether the
/// form differs from `form`: true when it does not cast to
/// `TESObjectANIO`, when `TESForm::CompareAllComponents` answers true, or
/// when the words at +0x38 differ.
pub fn fn_00511dc0(e: &mut Engine, this: Ptr, form: u32) -> bool {
    let other = cast_to_anio(e, form);
    if other == 0 {
        return true;
    }
    if e.call(COMPARE_ALL_COMPONENTS, &args![this, other]).bool() {
        return true;
    }
    e.mem.u32(this.addr() + 0x38) != e.mem.u32(other + 0x38)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0051_1e30, fn_00511e30(Ptr, u32) -> u32),
        entry!(0x0051_1f30, fn_00511f30(Ptr, u32) -> Ptr),
        entry!(0x0051_1b20, fn_00511b20(Ptr, Ptr) -> bool),
        entry!(0x0051_1e40, fn_00511e40(Ptr) -> Ptr),
        entry!(0x0051_1f60, fn_00511f60(Ptr)),
        entry!(0x0051_2010, fn_00512010(Ptr)),
        entry!(0x0051_1d10, fn_00511d10(Ptr)),
        entry!(0x0051_1d70, fn_00511d70(Ptr, u32)),
        entry!(0x0051_1dc0, fn_00511dc0(Ptr, u32) -> bool),
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
    fn le81_thunk_00511e30() {
        assert_eq!(
            check(0x0051_1e30, 0x0051_19e0, 0x18, &[0x1]).u32(),
            0xdead_beef
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn at_00511f30_runs_destructor_body_then_deletes_on_flag_one() {
        use crate::prelude::*;

        let mut e = Engine::new();
        let this: Ptr = Ptr::new(e.mem.alloc(16));
        e.register(0x0051_1f60, |_, _| Ret::default());
        e.register(0x0040_1030, |_, _| Ret::default());
        for (flags, deletes) in [(0u32, false), (2, false), (1, true), (0xffff_ffff, true)] {
            e.call_log = Some(vec![]);
            let result = e.call(0x0051_1f30, &args![this, flags]);
            assert_eq!(result.u32(), this.addr());
            let log = e.call_log.take().unwrap();
            let calls: Vec<(u32, Vec<u32>)> = log
                .into_iter()
                .filter(|(address, _)| *address != 0x0051_1f30)
                .collect();
            let mut expected = vec![(0x0051_1f60, vec![this.addr()])];
            if deletes {
                expected.push((0x0040_1030, vec![this.addr()]));
            }
            assert_eq!(calls, expected);
        }
    }
}

#[cfg(test)]
mod b028_tests {
    use super::*;
    use std::cell::Cell;
    use std::rc::Rc;

    /// Chunk reader doubles over a list of (tag, data) chunks.
    fn load_engine(record_type: u32, chunks: Vec<(u32, Vec<u8>)>) -> Engine {
        let mut e = Engine::new();
        let list = Rc::new(chunks);
        let current = Rc::new(Cell::new(0usize));
        e.register_double(GET_TES_FORM, move |_, _| record_type.into_ret());
        e.register(TES_FORM_LOAD_FORM, |_, _| Ret::default());
        let (l, c) = (list.clone(), current.clone());
        e.register_double(GET_TES_CHUNK, move |_, _| {
            l.get(c.get()).map_or(0, |chunk| chunk.0).into_ret()
        });
        let (l, c) = (list.clone(), current.clone());
        e.register_double(NEXT_CHUNK, move |_, _| {
            c.set(c.get() + 1);
            u32::from(c.get() < l.len()).into_ret()
        });
        let (l, c) = (list.clone(), current.clone());
        e.register_double(CHUNK_SIZE, move |_, _| {
            (l[c.get()].1.len() as u32).into_ret()
        });
        let (l, c) = (list.clone(), current.clone());
        e.register_double(GET_CHUNK_DATA, move |e, a| {
            let data = &l[c.get()].1;
            let n = (a[2] as usize).min(data.len());
            e.mem.write(a[1], &data[..n]);
            Ret::default()
        });
        let (l, c) = (list, current);
        e.register_double(GET_CHUNK_WORD, move |e, a| {
            e.mem.write(a[1], &l[c.get()].1[..4]);
            Ret::default()
        });
        e.register(FILE_WORD, |_, _| 0xffff_0102u32.into_ret());
        e.register(LOAD_MODEL_CHUNK, |_, _| Ret::default());
        e.register(UNPACK_TEXTURE_SWAP, |_, _| Ret::default());
        e.put_vtable(0x0200_0000, &[0; 0x80]);
        e.mem.set_u32(0x0200_0000 + 0x134, 0x0090_0000);
        e.mem.set_u32(0x0200_0000 + 0xe0, 0x0090_0010);
        e.register(0x0090_0000, |_, _| Ret::default());
        e.register(0x0090_0010, |_, _| Ret::default());
        e
    }

    #[test]
    fn fn_00511b20_refuses_other_records() {
        let mut e = load_engine(0x4c, vec![(CHUNK_DATA, vec![1, 0, 0, 0])]);
        let this = e.mem.alloc(0x180);
        e.call_log = Some(vec![]);
        assert!(!e.call(0x0051_1b20, &args![this, 0x7000u32]).bool());
        let log = e.call_log.take().unwrap();
        assert!(!log.iter().any(|c| c.0 == TES_FORM_LOAD_FORM));
    }

    #[test]
    fn fn_00511b20_reads_every_chunk() {
        let chunks = vec![
            (CHUNK_EDID, b"Arma\0".to_vec()),
            (CHUNK_OBND, vec![0; 12]),
            (CHUNK_MODL, b"a.nif\0".to_vec()),
            (CHUNK_MODD, vec![0]),
            (CHUNK_MOSD, vec![0]),
            (CHUNK_MODT, vec![0]),
            (CHUNK_MODS, vec![9, 8, 7]),
            (CHUNK_DATA, vec![0x78, 0x56, 0x34, 0x12]),
            (0x1234_5678, vec![]),
        ];
        let mut e = load_engine(0x4d, chunks);
        let this = e.mem.alloc(0x180);
        e.mem.set_u32(this, 0x0200_0000);
        e.call_log = Some(vec![]);
        assert!(e.call(0x0051_1b20, &args![this, 0x7000u32]).bool());
        assert_eq!(e.mem.u32(this + 0x38), 0x1234_5678);
        let log = e.call_log.take().unwrap();
        let of = |addr: u32| -> Vec<Vec<u32>> {
            log.iter()
                .filter(|c| c.0 == addr)
                .map(|c| c.1.clone())
                .collect()
        };
        assert_eq!(of(TES_FORM_LOAD_FORM), vec![vec![this, 0x7000]]);
        assert_eq!(of(0x0090_0010), vec![vec![this, 0x7000]]);
        assert_eq!(of(0x0090_0000).len(), 1);
        assert_eq!(of(LOAD_MODEL_CHUNK), vec![vec![this + 0x18, 0x7000]; 4]);
        let swaps = of(UNPACK_TEXTURE_SWAP);
        assert_eq!(swaps.len(), 1);
        assert_eq!(swaps[0][0], this + 0x18);
        assert_eq!(swaps[0][2..], [3, 0x0102]);
        let reads = of(GET_CHUNK_DATA);
        assert_eq!(reads[0][2], 0x200);
        assert_eq!(reads[1][2], 3);
    }

    #[test]
    fn fn_00511e40_builds_the_arma() {
        let mut e = Engine::new();
        for callee in [0x0051_29f0, 0x004f_15a0, 0x0040_3d30] {
            e.register(callee, |_, _| Ret::default());
        }
        let this = e.mem.alloc(0x190);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0051_1e40, &args![this]).u32(), this);
        for (offset, vtable) in ARMA_VTABLES {
            assert_eq!(e.mem.u32(this + offset), vtable);
        }
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log[1..],
            [
                (0x0051_29f0, vec![this]),
                (0x004f_15a0, vec![this, 0x60]),
                (0x0040_3d30, vec![this + 0x178, 0, 0xc]),
            ]
        );
    }

    #[test]
    fn fn_00511f60_restores_the_vtables_then_destroys_the_base() {
        let mut e = Engine::new();
        e.register(0x0051_2bd0, |_, _| Ret::default());
        let this = e.mem.alloc(0x190);
        e.call_log = Some(vec![]);
        e.call(0x0051_1f60, &args![this]);
        for (offset, vtable) in ARMA_VTABLES {
            assert_eq!(e.mem.u32(this + offset), vtable);
        }
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1..], [(0x0051_2bd0, vec![this])]);
    }

    #[test]
    fn fn_00512010_clears_twelve_bytes_at_0x178() {
        let mut e = Engine::new();
        e.register(0x0040_3d30, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0051_2010, &args![0x4000u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1..], [(0x0040_3d30, vec![0x4178, 0, 0xc])]);
    }
}

#[cfg(test)]
mod n040_tests {
    use super::*;

    fn calls_to(e: &mut Engine, targets: &[u32]) -> Vec<(u32, Vec<u32>)> {
        e.call_log
            .take()
            .unwrap()
            .into_iter()
            .filter(|(a, _)| targets.contains(a))
            .collect()
    }

    /// The cast answers the form when its word at +0xc is zero.
    fn cast(e: &mut Engine) {
        e.register(RT_DYNAMIC_CAST, |e, a| {
            assert_eq!(a[1..], [0, TYPE_TES_FORM, TYPE_TES_OBJECT_ANIO, 0]);
            let form = if e.mem.u32(a[0] + 0xc) == 0 { a[0] } else { 0 };
            Ret {
                eax: form,
                ..Ret::default()
            }
        });
    }

    #[test]
    fn fn_00511d10_saves_the_model_and_the_data_chunk() {
        let mut e = Engine::new();
        let targets = [
            START_FORM,
            MODEL_SAVE,
            MEMBER_0084E3A0,
            WRITE_CHUNK_WORD,
            CLOSE_FORM,
        ];
        for t in targets {
            e.register(t, |_, _| Ret {
                eax: 0x99,
                ..Ret::default()
            });
        }
        let this = e.mem.alloc(0x40);
        e.call_log = Some(vec![]);
        e.call(0x0051_1d10, &[this]);
        let model = vec![this + 0x18, 0x4c44_4f4d, 0x5444_4f4d, 0x5344_4f4d];
        assert_eq!(
            calls_to(&mut e, &targets),
            vec![
                (START_FORM, vec![this]),
                (MODEL_SAVE, model.clone()),
                (CLOSE_FORM, vec![this]),
            ]
        );
        e.mem.set_u32(this + 0x38, 0x5000);
        e.call_log = Some(vec![]);
        e.call(0x0051_1d10, &[this]);
        assert_eq!(
            calls_to(&mut e, &targets),
            vec![
                (START_FORM, vec![this]),
                (MODEL_SAVE, model),
                (MEMBER_0084E3A0, vec![0x5000]),
                (WRITE_CHUNK_WORD, vec![0x4154_4144, 0x99]),
                (CLOSE_FORM, vec![this]),
            ]
        );
    }

    #[test]
    fn fn_00511d70_and_00511dc0_copy_and_compare() {
        let mut e = Engine::new();
        cast(&mut e);
        e.register(COPY_ALL_COMPONENTS, |_, _| Ret::default());
        e.register(COMPARE_ALL_COMPONENTS, |e, a| Ret {
            eax: e.mem.u32(a[1] + 0x10),
            ..Ret::default()
        });
        let this = e.mem.alloc(0x40);
        let other = e.mem.alloc(0x40);
        e.mem.set_u32(other + 0x38, 0x77);
        e.mem.set_u32(other + 0xc, 1);
        assert!(e.call(0x0051_1dc0, &[this, other]).bool());
        e.call(0x0051_1d70, &[this, other]);
        assert_eq!(e.mem.u32(this + 0x38), 0);
        e.mem.set_u32(other + 0xc, 0);
        assert!(e.call(0x0051_1dc0, &[this, other]).bool());
        e.call_log = Some(vec![]);
        e.call(0x0051_1d70, &[this, other]);
        assert_eq!(
            calls_to(&mut e, &[COPY_ALL_COMPONENTS]),
            vec![(COPY_ALL_COMPONENTS, vec![this, other])]
        );
        assert_eq!(e.mem.u32(this + 0x38), 0x77);
        assert!(!e.call(0x0051_1dc0, &[this, other]).bool());
        e.mem.set_u32(other + 0x10, 1);
        assert!(e.call(0x0051_1dc0, &[this, other]).bool());
    }
}

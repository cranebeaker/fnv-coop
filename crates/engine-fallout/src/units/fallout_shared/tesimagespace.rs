//! `fallout shared/tesimagespace.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 0052b310 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0052b310 - the exception-unwinding frame (FS:[0]) and the security-cookie bookkeeping are not translated.
/// Constructor of the form `CreateFormOfType` (`00465110`) makes for form type 0x53 (0xb0 bytes); `ECX` is the object.
/// The engine map names it `TESImageSpace::TESImageSpace` (Xbox PDB); the RTTI name of the vtable it stores at `+0` (`0x0102_d7f4`) is `TESImageSpace`, which agrees.
/// It runs the base and component constructors by address, stores the vtable pointers and sets the fields to their start values.
/// The exception-unwinding frame is not translated.
pub fn fn_0052b310(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(0x0048_3370, &args![this]);
    e.mem.set_u32(base, 0x0102_d7f4);
    e.call(0x004f_15a0, &args![this, 0x53u32]);
    e.call(0x0052_b410, &args![this]);
    this
}

// Translated from 0052b380 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor shape (the engine map gives it no name; the class is unconfirmed): runs the destructor body `0052b3b0` on `this`, then frees `this` with `00401030` when bit 0 of `flags` is set. Returns `this`.
pub fn fn_0052b380(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x0052_b3b0, &args![this]);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

/// The game's `memset` wrapper (`00403d30`, `cdecl`: destination, byte, size).
const MEMSET: u32 = 0x0040_3d30;
/// `TESFile::GetTESForm` (Xbox PDB, `00472660`, `ECX` = file): the record's form type (`AL`).
const GET_TES_FORM: u32 = 0x0047_2660;
/// `TESForm::LoadForm` (Xbox PDB, `00485110`, `ECX` = form, one stack word: the file).
const LOAD_FORM: u32 = 0x0048_5110;
/// `00484ab0` (`ECX`, one stack word): sets or clears form flag 8.
const SET_FORM_FLAG_8: u32 = 0x0048_4ab0;
/// `TESFile::GetTESChunk` (Xbox PDB, `004726b0`, `ECX` = file): the chunk's code, 0 at the end.
const GET_TES_CHUNK: u32 = 0x0047_26b0;
/// `00401660` (`ECX` = file): the current chunk's size.
const CHUNK_SIZE: u32 = 0x0040_1660;
/// `TESFile::GetChunkData_ov3` (Xbox PDB, `00472890`, `ECX` = file, two stack words:
/// destination, size).
const GET_CHUNK_DATA_SIZED: u32 = 0x0047_2890;
/// `004b7580` (`cdecl`): `memcpy_s(destination, destination size, source, count)`.
const MEMCPY_S: u32 = 0x004b_7580;
/// `00401680` (`ECX` = file): whether the file's data needs its byte order swapped (`AL`).
const NEEDS_SWAP: u32 = 0x0040_1680;
/// `ImageSpaceParameterData::Endian` (Xbox PDB, `00b8ad00`, `ECX`).
const IMAGE_SPACE_PARAMETERS_ENDIAN: u32 = 0x00b8_ad00;
/// `004726f0` (`ECX` = file): steps to the next chunk; false at the record's end (`AL`).
const NEXT_CHUNK: u32 = 0x0047_26f0;
/// Chunk codes `TESImageSpace::Load` reads (the four characters, first byte lowest).
const CHUNK_EDID: u32 = 0x4449_4445;
const CHUNK_OBND: u32 = 0x444e_424f;
const CHUNK_DNAM: u32 = 0x4d41_4e44;
/// The image space record's form type.
const FORM_TYPE_IMAGE_SPACE: u8 = 0x53;

// Translated from 0052b410 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESImageSpace`'s vtable (`0102d7f4`) slot +0x14, the `InitializeData` slot of the Xbox
/// PDB's `TESForm` (`ECX`; the constructor `0052b310` runs it): clears the 0x98 bytes of
/// image space data at +0x18 (`memset`).
pub fn fn_0052b410(e: &mut Engine, this: Ptr) {
    e.call(MEMSET, &args![this.addr() + 0x18, 0u32, 0x98u32]);
}

// Translated from 0052b3b0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0052b3b0 - the C++ exception-unwinding frame (FS:[0] chain, handler at 00ef8ae8, state variable) is not translated
/// The `TESImageSpace` destructor body (unnamed in the map; `ECX`, no stack words; the scalar
/// deleting destructor `0052b380` runs it): stores the vtable `0102d7f4`, then `00483710` and
/// `TESForm::~TESForm` (`00483630`) on `this`.
pub fn fn_0052b3b0(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), 0x0102_d7f4);
    e.call(0x0048_3710, &args![this]);
    e.call(0x0048_3630, &args![this]);
}

// Translated from 0052b440 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0052b440 - the /GS cookie check (00ec408c) is not translated; the EDID buffer from __alloca_probe_16 (00ec5ec0) is a with_stack block of the chunk size (its 16-byte alignment padding, and the stack growing with each EDID chunk, are not modelled)
/// `TESImageSpace::Load` (unnamed in the map; `TESImageSpace`'s vtable slot +0x20, the `Load`
/// slot of the Xbox PDB's `TESForm`), `ECX` and one stack word, the file (`RET 4`). A record
/// that is not an image space (`TESFile::GetTESForm` not 0x53) answers false. Otherwise
/// `TESForm::LoadForm`, form flag 8 cleared (`00484ab0`), then each chunk until
/// `GetTESChunk` answers 0 or `004726f0` false: `EDID` (a stack buffer of the chunk's size,
/// read with at most 0x200 bytes, given to vtable slot +0x134), `OBND` (vtable slot +0xe0
/// with the file), `DNAM` (0x98 bytes read into a stack buffer; a chunk shorter than 0x98
/// bytes is the older layout: its first 0x38 bytes go to +0x18, the float at +0x50 is set to
/// 1.0 and the next 0x5c bytes go to +0x54; a full chunk goes to +0x18 whole (`memcpy_s`);
/// then, when the file needs it, `ImageSpaceParameterData::Endian` on +0x18). Then true.
pub fn fn_0052b440(e: &mut Engine, this: Ptr, file: Ptr) -> bool {
    let form_type = e.call(GET_TES_FORM, &args![file]).u32() as u8;
    if form_type != FORM_TYPE_IMAGE_SPACE {
        return false;
    }
    let base = this.addr();
    e.call(LOAD_FORM, &args![this, file]);
    e.call(SET_FORM_FLAG_8, &args![this, 0u32]);
    e.with_stack(0x98, |e, buffer| {
        let buffer = buffer.addr();
        loop {
            let chunk = e.call(GET_TES_CHUNK, &args![file]).u32();
            if chunk == 0 {
                break;
            }
            match chunk {
                CHUNK_EDID => {
                    let size = e.call(CHUNK_SIZE, &args![file]).u32();
                    e.with_stack(size, |e, name| {
                        e.call(GET_CHUNK_DATA_SIZED, &args![file, name, 0x200u32]);
                        e.vcall(base, 0x134, &args![name]);
                    });
                }
                CHUNK_OBND => {
                    e.vcall(base, 0xe0, &args![file]);
                }
                CHUNK_DNAM => {
                    e.call(GET_CHUNK_DATA_SIZED, &args![file, buffer, 0x98u32]);
                    if e.call(CHUNK_SIZE, &args![file]).u32() < 0x98 {
                        e.call(MEMCPY_S, &args![base + 0x18, 0x3cu32, buffer, 0x38u32]);
                        // FLD1; FSTP float.
                        e.mem.set_u32(base + 0x50, x87_store32(1.0));
                        e.call(
                            MEMCPY_S,
                            &args![base + 0x54, 0x5cu32, buffer + 0x38, 0x5cu32],
                        );
                    } else {
                        e.call(MEMCPY_S, &args![base + 0x18, 0x98u32, buffer, 0x98u32]);
                    }
                    if e.call(NEEDS_SWAP, &args![file]).bool() {
                        e.call(IMAGE_SPACE_PARAMETERS_ENDIAN, &args![base + 0x18]);
                    }
                }
                _ => {}
            }
            if !e.call(NEXT_CHUNK, &args![file]).bool() {
                break;
            }
        }
    });
    true
}

/// `TESForm::StartForm` (`004855a0`), `TESForm::CloseForm` (`00485680`),
/// `TESForm::__AddChunkData` (`00485990`, `cdecl (chunk, data, size)`) and
/// the byte `00401500` answers (`011c54ba`: whether saving swaps bytes).
const FORM_START: u32 = 0x0048_55a0;
const FORM_CLOSE: u32 = 0x0048_5680;
const FORM_ADD_CHUNK_DATA: u32 = 0x0048_5990;
const SAVE_SWAPS: u32 = 0x0040_1500;

// Translated from 0052b630 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESImageSpace::Save` (Xbox PDB): starts the form, adds the 0x98-byte
/// parameters at +0x18 as `DNAM` (swapped by `ImageSpaceParameterData::Endian`
/// before and after when `00401500` answers non-zero) and closes the form.
pub fn tes_image_space_save(e: &mut Engine, this: Ptr) {
    let data = this.addr() + 0x18;
    e.call(FORM_START, &args![this]);
    if e.call(SAVE_SWAPS, &args![]).u8() != 0 {
        e.call(IMAGE_SPACE_PARAMETERS_ENDIAN, &args![data]);
    }
    e.call(FORM_ADD_CHUNK_DATA, &args![CHUNK_DNAM, data, 0x98u32]);
    if e.call(SAVE_SWAPS, &args![]).u8() != 0 {
        e.call(IMAGE_SPACE_PARAMETERS_ENDIAN, &args![data]);
    }
    e.call(FORM_CLOSE, &args![this]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0052_b310, fn_0052b310(Ptr) -> Ptr),
        entry!(0x0052_b380, fn_0052b380(Ptr, u32) -> Ptr),
        entry!(0x0052_b410, fn_0052b410(Ptr)),
        entry!(0x0052_b3b0, fn_0052b3b0(Ptr)),
        entry!(0x0052_b440, fn_0052b440(Ptr, Ptr) -> bool),
        entry!(0x0052_b630, tes_image_space_save(Ptr)),
    ]
}

#[cfg(test)]
mod image_space_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        let log = e.call_log.as_ref().unwrap();
        log.iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    #[test]
    fn fn_0052b410_clears_the_data() {
        let mut e = Engine::new();
        e.register(MEMSET, |_, a| a[0].into_ret());
        e.call_log = Some(vec![]);
        e.call(0x0052_b410, &args![0x5000u32]);
        assert_eq!(calls(&e, MEMSET), vec![vec![0x5018, 0, 0x98]]);
    }

    #[test]
    fn fn_0052b3b0_stores_the_vtable_then_destroys() {
        let mut e = Engine::new();
        e.register(0x0048_3710, |e, a| {
            let seen = e.mem.u32(a[0]);
            e.mem.set_u32(a[0] + 4, seen);
            Ret::default()
        });
        e.register(0x0048_3630, |_, _| Ret::default());
        let this = e.mem.alloc(0xb0);
        e.call_log = Some(vec![]);
        e.call(0x0052_b3b0, &args![this]);
        assert_eq!(e.mem.u32(this + 4), 0x0102_d7f4);
        let order: Vec<u32> = e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .skip(1)
            .map(|c| c.0)
            .collect();
        assert_eq!(order, vec![0x0048_3710, 0x0048_3630]);
    }

    type Script = Rc<RefCell<(Vec<(u32, u32)>, usize)>>;

    const FILE: u32 = 0x0e00_0000;

    fn load_rig(chunks: Vec<(u32, u32)>, swap: bool) -> (Engine, u32) {
        let mut e = Engine::new();
        let script: Script = Rc::new(RefCell::new((chunks, 0)));
        e.register(GET_TES_FORM, |_, _| 0x53u32.into_ret());
        e.register(LOAD_FORM, |_, _| Ret::default());
        e.register(SET_FORM_FLAG_8, |_, _| Ret::default());
        let s = script.clone();
        e.register_double(GET_TES_CHUNK, move |_, _| {
            let s = s.borrow();
            s.0.get(s.1).map_or(0, |c| c.0).into_ret()
        });
        let s = script.clone();
        e.register_double(CHUNK_SIZE, move |_, _| {
            let s = s.borrow();
            s.0[s.1].1.into_ret()
        });
        let s = script;
        e.register_double(NEXT_CHUNK, move |_, _| {
            let mut s = s.borrow_mut();
            s.1 += 1;
            (s.1 < s.0.len()).into_ret()
        });
        e.register(GET_CHUNK_DATA_SIZED, |_, _| Ret::default());
        e.register(MEMCPY_S, |_, _| Ret::default());
        e.register_double(NEEDS_SWAP, move |_, _| swap.into_ret());
        e.register(IMAGE_SPACE_PARAMETERS_ENDIAN, |_, _| Ret::default());
        e.put_vtable(0x0f00_c000, &[0; 0x4e]);
        e.mem.set_u32(0x0f00_c0e0, 0x0f00_c100);
        e.mem.set_u32(0x0f00_c134, 0x0f00_c200);
        e.register(0x0f00_c100, |_, _| Ret::default());
        e.register(0x0f00_c200, |_, _| Ret::default());
        let this = e.mem.alloc(0xb0);
        e.mem.set_u32(this, 0x0f00_c000);
        e.call_log = Some(vec![]);
        (e, this)
    }

    #[test]
    fn fn_0052b440_refuses_other_records() {
        let (mut e, this) = load_rig(vec![], false);
        e.register(GET_TES_FORM, |_, _| 0x36u32.into_ret());
        assert!(!e.call(0x0052_b440, &args![this, FILE]).bool());
        assert!(calls(&e, LOAD_FORM).is_empty());
    }

    #[test]
    fn fn_0052b440_reads_the_name_bound_and_a_full_data_chunk() {
        let chunks = vec![
            (CHUNK_EDID, 0x1c),
            (CHUNK_OBND, 0xc),
            (CHUNK_DNAM, 0x98),
            (0x1234, 4),
        ];
        let (mut e, this) = load_rig(chunks, true);
        assert!(e.call(0x0052_b440, &args![this, FILE]).bool());
        assert_eq!(calls(&e, LOAD_FORM), vec![vec![this, FILE]]);
        assert_eq!(calls(&e, SET_FORM_FLAG_8), vec![vec![this, 0]]);
        let edid = calls(&e, 0x0f00_c200);
        let reads = calls(&e, GET_CHUNK_DATA_SIZED);
        assert_eq!(reads[0], vec![FILE, edid[0][1], 0x200]);
        assert_eq!(reads[1][2], 0x98);
        assert_eq!(calls(&e, 0x0f00_c100), vec![vec![this, FILE]]);
        assert_eq!(
            calls(&e, MEMCPY_S),
            vec![vec![this + 0x18, 0x98, reads[1][1], 0x98]]
        );
        assert_eq!(
            calls(&e, IMAGE_SPACE_PARAMETERS_ENDIAN),
            vec![vec![this + 0x18]]
        );
        assert_eq!(calls(&e, NEXT_CHUNK).len(), 4);
    }

    #[test]
    fn fn_0052b440_spreads_a_short_data_chunk() {
        let (mut e, this) = load_rig(vec![(CHUNK_DNAM, 0x94)], false);
        assert!(e.call(0x0052_b440, &args![this, FILE]).bool());
        let buffer = calls(&e, GET_CHUNK_DATA_SIZED)[0][1];
        assert_eq!(
            calls(&e, MEMCPY_S),
            vec![
                vec![this + 0x18, 0x3c, buffer, 0x38],
                vec![this + 0x54, 0x5c, buffer + 0x38, 0x5c]
            ]
        );
        assert_eq!(e.mem.f32(this + 0x50), 1.0);
        assert!(calls(&e, IMAGE_SPACE_PARAMETERS_ENDIAN).is_empty());
    }
}

#[cfg(test)]
mod tests {
    use crate::units::crt::ctor_test::{self, A};

    #[test]
    fn fn_0052b310_builds_the_object() {
        ctor_test::run(&ctor_test::Case {
            addr: 0x0052_b310,
            size: 0xb0,
            globals: &[],
            returns: &[],
            calls: &[
                (0x0048_3370, &[A::T(0x0)]),
                (0x004f_15a0, &[A::T(0x0), A::V(0x53)]),
                (0x0052_b410, &[A::T(0x0)]),
            ],
            stores: &[(0x0, 4, 0x0102_d7f4)],
        });
    }
    #[test]
    fn at_0052b380_runs_destructor_body_then_deletes_on_flag_one() {
        use crate::prelude::*;

        let mut e = Engine::new();
        let this: Ptr = Ptr::new(e.mem.alloc(16));
        e.register(0x0052_b3b0, |_, _| Ret::default());
        e.register(0x0040_1030, |_, _| Ret::default());
        for (flags, deletes) in [(0u32, false), (2, false), (1, true), (0xffff_ffff, true)] {
            e.call_log = Some(vec![]);
            let result = e.call(0x0052_b380, &args![this, flags]);
            assert_eq!(result.u32(), this.addr());
            let log = e.call_log.take().unwrap();
            let calls: Vec<(u32, Vec<u32>)> = log
                .into_iter()
                .filter(|(address, _)| *address != 0x0052_b380)
                .collect();
            let mut expected = vec![(0x0052_b3b0, vec![this.addr()])];
            if deletes {
                expected.push((0x0040_1030, vec![this.addr()]));
            }
            assert_eq!(calls, expected);
        }
    }
}

#[cfg(test)]
mod n043_tests {
    use super::*;

    #[test]
    fn save_adds_dnam_swapped_when_asked() {
        let mut e = Engine::new();
        e.map(0x7300_0000, 4);
        for addr in [
            FORM_START,
            FORM_CLOSE,
            FORM_ADD_CHUNK_DATA,
            IMAGE_SPACE_PARAMETERS_ENDIAN,
        ] {
            e.register(addr, |_, _| Ret::default());
        }
        e.register(SAVE_SWAPS, |e, _| e.mem.u32(0x7300_0000).into_ret());
        e.mem.set_u32(0x7300_0000, 1);
        e.call_log = Some(vec![]);
        e.call(0x0052_b630, &args![0x1000u32]);
        let order: Vec<(u32, Vec<u32>)> = e
            .call_log
            .take()
            .unwrap()
            .into_iter()
            .filter(|c| c.0 != SAVE_SWAPS)
            .skip(1)
            .collect();
        assert_eq!(
            order,
            vec![
                (FORM_START, vec![0x1000]),
                (IMAGE_SPACE_PARAMETERS_ENDIAN, vec![0x1018]),
                (FORM_ADD_CHUNK_DATA, vec![CHUNK_DNAM, 0x1018, 0x98]),
                (IMAGE_SPACE_PARAMETERS_ENDIAN, vec![0x1018]),
                (FORM_CLOSE, vec![0x1000]),
            ]
        );
        e.mem.set_u32(0x7300_0000, 0);
        e.call_log = Some(vec![]);
        e.call(0x0052_b630, &args![0x1000u32]);
        let log = e.call_log.take().unwrap();
        assert!(log.iter().all(|c| c.0 != IMAGE_SPACE_PARAMETERS_ENDIAN));
    }
}

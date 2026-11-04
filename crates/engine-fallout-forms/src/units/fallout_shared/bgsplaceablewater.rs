//! `fallout shared/bgsplaceablewater.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 004fc1b0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 004fc1b0 - the exception-unwinding frame (FS:[0]) and the security-cookie bookkeeping are not translated.
/// `BGSPlaceableWater::BGSPlaceableWater` (Xbox PDB): the form constructor `CreateFormOfType` (`00465110`) runs for form type 0x23 (0x50 bytes).
/// `ECX` is the object; it runs the base and component constructors, stores the vtable pointers of the object and its components, and sets the fields to their start values.
/// The exception-unwinding frame is not translated.
pub fn bgsplaceablewater_bgsplaceablewater(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(0x0050_e820, &args![this]);
    e.call(0x0048_8f50, &args![base + 0x30]);
    e.mem.set_u32(base, 0x0102_4f4c);
    e.mem.set_u32(base + 0x30, 0x0102_4f28);
    e.call(0x0040_19f0, &args![base + 0x48]);
    e.call(0x004f_15a0, &args![this, 0x23]);
    e.call(0x004f_c2e0, &args![this]);
    this
}

// Translated from 004fc240 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor shape (the engine map gives it no name; the class is unconfirmed): runs the destructor body `004fc270` on `this`, then frees `this` with `00401030` when bit 0 of `flags` is set. Returns `this`.
pub fn fn_004fc240(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x004f_c270, &args![this]);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

/// `00624700`: returns `this + 0x48`, the address of the water's flags word
/// (the first of the two words its `DNAM` chunk fills).
const FLAGS_ADDRESS: u32 = 0x0062_4700;
/// `00843170` (thiscall): zeroes the two words at `this` and `this + 4`.
const CLEAR_TWO_WORDS: u32 = 0x0084_3170;
/// `004fc990` (unit `run_004fc860`, thiscall, `RET 8`): sets (`value` nonzero)
/// or clears the bits `mask` of the flags word at `this + 0x48`.
const SET_FLAG_BITS: u32 = 0x004f_c990;
/// `004013e0` (thiscall): bit 3 of the form's flags (`TESForm` +0x08).
const FORM_FLAG_8: u32 = 0x0040_13e0;
/// `TESForm::GetFile` (Xbox PDB, `00484e60`, thiscall, `RET 4`).
const FORM_GET_FILE: u32 = 0x0048_4e60;
/// `TESForm::AddCompileIndex` (Xbox PDB, `00485d50`, cdecl: id pointer, file).
const FORM_ADD_COMPILE_INDEX: u32 = 0x0048_5d50;
/// `004839c0` (cdecl, one word): the form with the form id given.
const LOOKUP_FORM: u32 = 0x0048_39c0;
/// `__RTDynamicCast` (`00ec43fb`, cdecl, five words).
const RT_DYNAMIC_CAST: u32 = 0x00ec_43fb;
/// The `TESForm` RTTI type descriptor (`01183028`), the cast's source type.
const TYPE_TES_FORM: u32 = 0x0118_3028;
/// The RTTI type descriptor the form at +0x4C is cast to (`01184118`; the
/// class is not confirmed here).
const TYPE_PLACEABLE_WATER_FORM: u32 = 0x0118_4118;
/// The global read for a missing +0x4C form (`011ca53c`).
const DEFAULT_WATER_FORM: u32 = 0x011c_a53c;
/// `00484ab0` (`TESForm`, thiscall, `RET 4`): sets or clears a form flag bit.
const FORM_SET_FLAG: u32 = 0x0048_4ab0;
/// `TESFile::GetTESForm` (Xbox PDB, `00472660`): the current record's form type.
const FILE_GET_FORM_TYPE: u32 = 0x0047_2660;
/// `TESForm::LoadForm` (Xbox PDB, `00485110`, thiscall, `RET 4`).
const FORM_LOAD_FORM: u32 = 0x0048_5110;
/// `TESFile::GetTESChunk` (Xbox PDB, `004726b0`): the current chunk's tag, 0 at the end.
const FILE_GET_CHUNK: u32 = 0x0047_26b0;
/// `004726f0` (thiscall): steps to the next chunk; false at the end.
const FILE_NEXT_CHUNK: u32 = 0x0047_26f0;
/// `00401660` (thiscall): the file's word at +0x25C, the current chunk's size.
const FILE_CHUNK_SIZE: u32 = 0x0040_1660;
/// `TESFile::GetChunkData_ov3` (Xbox PDB, `00472890`, thiscall, `RET 8`).
const FILE_GET_CHUNK_DATA: u32 = 0x0047_2890;
/// `00401680` (thiscall): the file's byte at +0x299 (the data is byte-swapped).
const FILE_SWAPS: u32 = 0x0040_1680;
/// `00403570` (thiscall): the file's 16-bit word at +0x254 (the record's version).
const FILE_VERSION: u32 = 0x0040_3570;
/// `TESModel::LoadModelChunk` (Xbox PDB, `004892d0`, cdecl: model, file).
const MODEL_LOAD_MODEL_CHUNK: u32 = 0x0048_92d0;
/// `004fc790` (thiscall): clears the 8-byte `DATA` block (a 16-bit word at +0,
/// a word at +4).
const DATA_BLOCK_INIT: u32 = 0x004f_c790;
/// `TESForm::LoadData` (Xbox PDB, `004861f0`, thiscall, `RET 0xC`: file, buffer, size).
const FORM_LOAD_DATA: u32 = 0x0048_61f0;
/// `00535aa0` (thiscall): byte-swaps the 8-byte `DATA` block.
const DATA_BLOCK_SWAP: u32 = 0x0053_5aa0;
/// `00462230` (thiscall): byte-swaps the two words of the `DNAM` block.
const DNAM_SWAP: u32 = 0x0046_2230;

/// The vtable slot the load calls first (byte offset 0x18, no arguments).
const SLOT_CLEAR: u32 = 0x18;
/// The vtable slot `EDID` passes the editor id to (byte offset 0x134).
const SLOT_SET_EDITOR_ID: u32 = 0x134;
/// The vtable slot `OBND` passes the file to (byte offset 0xE0).
const SLOT_LOAD_BOUND: u32 = 0xe0;

const FORM_TYPE_PLACEABLE_WATER: u32 = 0x23;
const CHUNK_MODL: u32 = 0x4c44_4f4d;
const CHUNK_MODT: u32 = 0x5444_4f4d;
const CHUNK_DATA: u32 = 0x4154_4144;
const CHUNK_EDID: u32 = 0x4449_4445;
const CHUNK_OBND: u32 = 0x444e_424f;
const CHUNK_DNAM: u32 = 0x4d41_4e44;

// Translated from 004fc360 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets (`value` nonzero) or clears flag bit 0x1 of the water through `004fc990`.
pub fn fn_004fc360(e: &mut Engine, this: Ptr, value: u8) {
    e.call(SET_FLAG_BITS, &args![this, 0x1u32, u32::from(value)]);
}

// Translated from 004fc380 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets or clears flag bit 0x4 through `004fc990`.
pub fn fn_004fc380(e: &mut Engine, this: Ptr, value: u8) {
    e.call(SET_FLAG_BITS, &args![this, 0x4u32, u32::from(value)]);
}

// Translated from 004fc3a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets or clears flag bit 0x8 through `004fc990`.
pub fn fn_004fc3a0(e: &mut Engine, this: Ptr, value: u8) {
    e.call(SET_FLAG_BITS, &args![this, 0x8u32, u32::from(value)]);
}

// Translated from 004fc3c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets or clears flag bit 0x40 through `004fc990`.
pub fn fn_004fc3c0(e: &mut Engine, this: Ptr, value: u8) {
    e.call(SET_FLAG_BITS, &args![this, 0x40u32, u32::from(value)]);
}

// Translated from 004fc3e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets or clears flag bit 0x200 through `004fc990`.
pub fn fn_004fc3e0(e: &mut Engine, this: Ptr, value: u8) {
    e.call(SET_FLAG_BITS, &args![this, 0x200u32, u32::from(value)]);
}

// Translated from 004fc400 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets or clears flag bit 0x800 through `004fc990`.
pub fn fn_004fc400(e: &mut Engine, this: Ptr, value: u8) {
    e.call(SET_FLAG_BITS, &args![this, 0x800u32, u32::from(value)]);
}

// Translated from 004fc450 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets or clears flag bit 0x4000_0000 through `004fc990`.
pub fn fn_004fc450(e: &mut Engine, this: Ptr, value: u8) {
    e.call(
        SET_FLAG_BITS,
        &args![this, 0x4000_0000u32, u32::from(value)],
    );
}

// Translated from 004fc470 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets or clears flag bit 0x2000_0000 through `004fc990`.
pub fn fn_004fc470(e: &mut Engine, this: Ptr, value: u8) {
    e.call(
        SET_FLAG_BITS,
        &args![this, 0x2000_0000u32, u32::from(value)],
    );
}

// Translated from 004fc490 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets or clears flag bit 0x1000_0000 through `004fc990`.
pub fn fn_004fc490(e: &mut Engine, this: Ptr, value: u8) {
    e.call(
        SET_FLAG_BITS,
        &args![this, 0x1000_0000u32, u32::from(value)],
    );
}

// Translated from 004fc420 (decompiled, FalloutNV.exe 1.4.0.525)
/// Flag bit 0x8000_0000 with the sense inverted: `value` nonzero clears it,
/// zero sets it (`SETZ` before the call to `004fc990`).
pub fn fn_004fc420(e: &mut Engine, this: Ptr, value: u8) {
    let set = u32::from(value == 0);
    e.call(SET_FLAG_BITS, &args![this, 0x8000_0000u32, set]);
}

// Translated from 004fc2e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The constructor's field set-up: zeroes the two words at +0x48 (through
/// `00624700` and `00843170`), then sets flag bits 0x1, 0x4, 0x8, 0x40, 0x200
/// and 0x800, clears 0x4000_0000 and 0x2000_0000, sets 0x1000_0000 and clears
/// 0x8000_0000 (`004fc420` with 1).
pub fn fn_004fc2e0(e: &mut Engine, this: Ptr) {
    let flags = e.call(FLAGS_ADDRESS, &args![this]).u32();
    e.call(CLEAR_TWO_WORDS, &args![flags]);
    fn_004fc360(e, this, 1);
    fn_004fc380(e, this, 1);
    fn_004fc3a0(e, this, 1);
    fn_004fc3c0(e, this, 1);
    fn_004fc3e0(e, this, 1);
    fn_004fc400(e, this, 1);
    fn_004fc450(e, this, 0);
    fn_004fc470(e, this, 0);
    fn_004fc490(e, this, 1);
    fn_004fc420(e, this, 1);
}

// Translated from 004fc270 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 004fc270 - the exception-unwinding frame (FS:[0]) is not translated.
/// The destructor body `004fc240` runs: stores the object's and the +0x30
/// component's vtables (`01024f4c`, `01024f28`), destroys the component at
/// +0x30 (`00489070`, `tesmodel.cpp`), then runs the base destructor
/// (`0050e9f0`, `tesobject.cpp`).
pub fn fn_004fc270(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base, 0x0102_4f4c);
    e.mem.set_u32(base + 0x30, 0x0102_4f28);
    e.call(0x0048_9070, &args![base + 0x30]);
    e.call(0x0050_e9f0, &args![this]);
}

// Translated from 004fc4b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The water's `InitItem`-style resolution: unless form flag 0x8 is set
/// (`004013e0`), the form id at +0x4C (when nonzero) gets the compile index of
/// the form's file (`GetFile(-1)`, `AddCompileIndex`), is looked up
/// (`004839c0`) and cast with `__RTDynamicCast` (type `01184118`); a missing
/// form becomes the global at `011ca53c`. Then form flag bit 1 is set
/// (`00484ab0` with 1).
pub fn fn_004fc4b0(e: &mut Engine, this: Ptr) {
    if e.call(FORM_FLAG_8, &args![this]).bool() {
        return;
    }
    let base = this.addr();
    let id = e.mem.u32(base + 0x4c);
    if id != 0 {
        let resolved = e.with_stack(4, |e, slot| {
            e.mem.set_u32(slot.addr(), id);
            let file = e.call(FORM_GET_FILE, &args![this, 0xffff_ffffu32]).u32();
            e.call(FORM_ADD_COMPILE_INDEX, &args![slot, file]);
            let id = e.mem.u32(slot.addr());
            let form = e.call(LOOKUP_FORM, &args![id]).u32();
            e.call(
                RT_DYNAMIC_CAST,
                &args![form, 0u32, TYPE_TES_FORM, TYPE_PLACEABLE_WATER_FORM, 0u32],
            )
            .u32()
        });
        e.mem.set_u32(base + 0x4c, resolved);
    }
    if e.mem.u32(base + 0x4c) == 0 {
        let fallback = e.global::<u32>(DEFAULT_WATER_FORM);
        e.mem.set_u32(base + 0x4c, fallback);
    }
    e.call(FORM_SET_FLAG, &args![this, 1u32]);
}

// Translated from 004fc540 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 004fc540 - the compiler's security cookie check (`__security_check_cookie`, 00ec408c) is not translated; the `EDID` alloca buffer is a heap block.
/// The water's `Load` (`RET 4`: the file): false unless the record is form
/// type 0x23. Then the slot-0x18 call, `TESForm::LoadForm`, form flag bit 0
/// cleared (`00484ab0` with 0), and every chunk: `EDID` (a buffer the size of
/// the chunk, at most 0x200 bytes read, to vtable slot 0x134), `OBND` (slot
/// 0xE0 with the file), `MODL`/`MODT` (`TESModel::LoadModelChunk` on the
/// component at +0x30, or on 0 when `this` is null), `DATA` (8 bytes: a
/// cleared block loaded with `TESForm::LoadData`, swapped when the file
/// swaps; its word at +4 goes to +0x4C, +0x48 is cleared, flag bits 0x1, 0x4,
/// 0x8, 0x40, 0x200 and 0x800 are set and 0x4000_0000 cleared), `DNAM` (the
/// 8 bytes at +0x48; 0x4000_0000 cleared when the record version is below 9;
/// swapped when the file swaps). Returns true at the end of the chunks.
pub fn fn_004fc540(e: &mut Engine, this: Ptr, file: Ptr) -> bool {
    let base = this.addr();
    if u32::from(e.call(FILE_GET_FORM_TYPE, &args![file]).u8()) != FORM_TYPE_PLACEABLE_WATER {
        return false;
    }
    e.vcall(base, SLOT_CLEAR, &[]);
    e.call(FORM_LOAD_FORM, &args![this, file]);
    e.call(FORM_SET_FLAG, &args![this, 0u32]);
    loop {
        let chunk = e.call(FILE_GET_CHUNK, &args![file]).u32();
        if chunk == 0 {
            break;
        }
        match chunk {
            CHUNK_EDID => {
                // The game takes a stack buffer as large as the chunk and
                // reads at most 0x200 bytes into it.
                let size = e.call(FILE_CHUNK_SIZE, &args![file]).u32();
                let buffer = e.mem.alloc(size.max(1));
                e.call(FILE_GET_CHUNK_DATA, &args![file, buffer, 0x200u32]);
                e.vcall(base, SLOT_SET_EDITOR_ID, &args![buffer]);
                e.mem.free(buffer);
            }
            CHUNK_OBND => {
                e.vcall(base, SLOT_LOAD_BOUND, &args![file]);
            }
            CHUNK_MODL | CHUNK_MODT => {
                let model = if base == 0 { 0 } else { base + 0x30 };
                e.call(MODEL_LOAD_MODEL_CHUNK, &args![model, file]);
            }
            CHUNK_DATA => {
                let value = e.with_stack(8, |e, block| {
                    e.call(DATA_BLOCK_INIT, &args![block]);
                    e.call(FORM_LOAD_DATA, &args![this, file, block, 8u32]);
                    if e.call(FILE_SWAPS, &args![file]).bool() {
                        e.call(DATA_BLOCK_SWAP, &args![block]);
                    }
                    e.mem.u32(block.addr() + 4)
                });
                e.mem.set_u32(base + 0x4c, value);
                e.mem.set_u32(base + 0x48, 0);
                fn_004fc360(e, this, 1);
                fn_004fc380(e, this, 1);
                fn_004fc3a0(e, this, 1);
                fn_004fc3c0(e, this, 1);
                fn_004fc3e0(e, this, 1);
                fn_004fc400(e, this, 1);
                fn_004fc450(e, this, 0);
            }
            CHUNK_DNAM => {
                e.call(FILE_GET_CHUNK_DATA, &args![file, base + 0x48, 8u32]);
                if e.call(FILE_VERSION, &args![file]).u16() < 9 {
                    fn_004fc450(e, this, 0);
                }
                if e.call(FILE_SWAPS, &args![file]).bool() {
                    e.call(DNAM_SWAP, &args![base + 0x48]);
                }
            }
            _ => {}
        }
        if !e.call(FILE_NEXT_CHUNK, &args![file]).bool() {
            break;
        }
    }
    true
}

/// `TESForm::StartForm` (Xbox PDB, `004855a0`).
const FORM_START_FORM: u32 = 0x0048_55a0;
/// `TESForm::CloseForm` (Xbox PDB, `00485680`).
const FORM_CLOSE_FORM: u32 = 0x0048_5680;
/// `TESModel::Save` (Xbox PDB, `004891d0`; `RET 8`: the two chunk
/// signatures).
const MODEL_SAVE: u32 = 0x0048_91d0;
/// The `DNAM` block's constructor (`004019f0`, `this` only).
const DNAM_BLOCK_INIT: u32 = 0x0040_19f0;
/// Copies bytes (`00401460`, cdecl: destination, source, size).
const COPY_BYTES: u32 = 0x0040_1460;
/// `TESForm::__AddChunkData` (Xbox PDB, `00485990`, cdecl: signature, data,
/// size).
const ADD_CHUNK_DATA: u32 = 0x0048_5990;
/// Whether the data written needs byte swapping (`00401500`, no arguments).
const SAVE_SWAPS: u32 = 0x0040_1500;
/// `TESForm`'s form id getter (`0084e3a0`).
const FORM_ID: u32 = 0x0084_e3a0;

// Translated from 004fc790 (decompiled, FalloutNV.exe 1.4.0.525)
/// Clears the 8-byte `DATA` block (the 16-bit word at +0 and the word at
/// +4; bytes 2 and 3 are left as they are) and returns it.
pub fn fn_004fc790(e: &mut Engine, this: Ptr) -> Ptr {
    e.mem.set_u16(this.addr(), 0);
    e.mem.set_u32(this.addr() + 4, 0);
    this
}

// Translated from 004fc7b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSPlaceableWater::Save` (Xbox PDB): starts the form, saves the model at
/// +0x30 (`MODL`/`MODT`), then a `DNAM` chunk of a stack block (built by
/// `004019f0`, the 8 bytes at +0x48 copied in) whose second word is the form
/// id of the water form at +0x4C (0 without one), swapped (`00462230`)
/// before and after the write when `00401500` says so; closes the form.
pub fn bgs_placeable_water_save(e: &mut Engine, this: Ptr) {
    e.call(FORM_START_FORM, &args![this]);
    e.call(
        MODEL_SAVE,
        &args![this.addr() + 0x30, CHUNK_MODL, CHUNK_MODT],
    );
    e.with_stack(8, |e, block| {
        e.call(DNAM_BLOCK_INIT, &args![block]);
        e.call(COPY_BYTES, &args![block, this.addr() + 0x48, 8u32]);
        let water = e.mem.u32(this.addr() + 0x4c);
        let water_id = if water == 0 {
            0
        } else {
            e.call(FORM_ID, &args![water]).u32()
        };
        e.mem.set_u32(block.addr() + 4, water_id);
        if e.call(SAVE_SWAPS, &args![]).bool() {
            e.call(DNAM_SWAP, &args![block]);
        }
        e.call(ADD_CHUNK_DATA, &args![CHUNK_DNAM, block, 8u32]);
        if e.call(SAVE_SWAPS, &args![]).bool() {
            e.call(DNAM_SWAP, &args![block]);
        }
    });
    e.call(FORM_CLOSE_FORM, &args![this]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x004f_c1b0, bgsplaceablewater_bgsplaceablewater(Ptr) -> Ptr),
        entry!(0x004f_c240, fn_004fc240(Ptr, u32) -> Ptr),
        entry!(0x004f_c360, fn_004fc360(Ptr, u8)),
        entry!(0x004f_c380, fn_004fc380(Ptr, u8)),
        entry!(0x004f_c3a0, fn_004fc3a0(Ptr, u8)),
        entry!(0x004f_c3c0, fn_004fc3c0(Ptr, u8)),
        entry!(0x004f_c3e0, fn_004fc3e0(Ptr, u8)),
        entry!(0x004f_c400, fn_004fc400(Ptr, u8)),
        entry!(0x004f_c450, fn_004fc450(Ptr, u8)),
        entry!(0x004f_c470, fn_004fc470(Ptr, u8)),
        entry!(0x004f_c490, fn_004fc490(Ptr, u8)),
        entry!(0x004f_c420, fn_004fc420(Ptr, u8)),
        entry!(0x004f_c2e0, fn_004fc2e0(Ptr)),
        entry!(0x004f_c270, fn_004fc270(Ptr)),
        entry!(0x004f_c4b0, fn_004fc4b0(Ptr)),
        entry!(0x004f_c540, fn_004fc540(Ptr, Ptr) -> bool),
        entry!(0x004f_c790, fn_004fc790(Ptr) -> Ptr),
        entry!(0x004f_c7b0, bgs_placeable_water_save(Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use crate::units::crt::ctor_test::{self, A};

    #[test]
    fn bgsplaceablewater_bgsplaceablewater_builds_the_object() {
        ctor_test::run(&ctor_test::Case {
            addr: 0x004f_c1b0,
            size: 0x50,
            globals: &[],
            returns: &[],
            calls: &[
                (0x0050_e820, &[A::T(0x0)]),
                (0x0048_8f50, &[A::T(0x30)]),
                (0x0040_19f0, &[A::T(0x48)]),
                (0x004f_15a0, &[A::T(0x0), A::V(0x23)]),
                (0x004f_c2e0, &[A::T(0x0)]),
            ],
            stores: &[(0x0, 4, 0x0102_4f4c), (0x30, 4, 0x0102_4f28)],
        });
    }
    #[test]
    fn at_004fc240_runs_destructor_body_then_deletes_on_flag_one() {
        use crate::prelude::*;

        let mut e = Engine::new();
        let this: Ptr = Ptr::new(e.mem.alloc(16));
        e.register(0x004f_c270, |_, _| Ret::default());
        e.register(0x0040_1030, |_, _| Ret::default());
        for (flags, deletes) in [(0u32, false), (2, false), (1, true), (0xffff_ffff, true)] {
            e.call_log = Some(vec![]);
            let result = e.call(0x004f_c240, &args![this, flags]);
            assert_eq!(result.u32(), this.addr());
            let log = e.call_log.take().unwrap();
            let calls: Vec<(u32, Vec<u32>)> = log
                .into_iter()
                .filter(|(address, _)| *address != 0x004f_c240)
                .collect();
            let mut expected = vec![(0x004f_c270, vec![this.addr()])];
            if deletes {
                expected.push((0x0040_1030, vec![this.addr()]));
            }
            assert_eq!(calls, expected);
        }
    }
}

#[cfg(test)]
mod water_tests {
    use crate::prelude::*;

    type Log = Vec<(u32, Vec<u32>)>;

    fn ok(_: &mut Engine, _: &[u32]) -> Ret {
        Ret::default()
    }

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    fn only(log: &Log, addr: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    fn check_setter(addr: u32, mask: u32, inverted: bool) {
        let mut e = Engine::new();
        e.register(0x004f_c990, ok);
        for (word, low) in [(1u32, 1u32), (0, 0), (0x100, 0), (0x1ff, 0xff)] {
            e.call_log = Some(vec![]);
            e.call(addr, &[0x2000, word]);
            let log = e.call_log.take().unwrap();
            let value = if inverted { u32::from(low == 0) } else { low };
            assert_eq!(only(&log, 0x004f_c990), vec![vec![0x2000, mask, value]]);
        }
    }

    #[test]
    fn at_004fc360_sets_bit_1() {
        check_setter(0x004f_c360, 0x1, false);
    }
    #[test]
    fn at_004fc380_sets_bit_4() {
        check_setter(0x004f_c380, 0x4, false);
    }
    #[test]
    fn at_004fc3a0_sets_bit_8() {
        check_setter(0x004f_c3a0, 0x8, false);
    }
    #[test]
    fn at_004fc3c0_sets_bit_40() {
        check_setter(0x004f_c3c0, 0x40, false);
    }
    #[test]
    fn at_004fc3e0_sets_bit_200() {
        check_setter(0x004f_c3e0, 0x200, false);
    }
    #[test]
    fn at_004fc400_sets_bit_800() {
        check_setter(0x004f_c400, 0x800, false);
    }
    #[test]
    fn at_004fc450_sets_bit_40000000() {
        check_setter(0x004f_c450, 0x4000_0000, false);
    }
    #[test]
    fn at_004fc470_sets_bit_20000000() {
        check_setter(0x004f_c470, 0x2000_0000, false);
    }
    #[test]
    fn at_004fc490_sets_bit_10000000() {
        check_setter(0x004f_c490, 0x1000_0000, false);
    }
    #[test]
    fn at_004fc420_inverts_bit_80000000() {
        check_setter(0x004f_c420, 0x8000_0000, true);
    }

    fn flag_calls(log: &Log) -> Vec<(u32, u32)> {
        only(log, 0x004f_c990)
            .into_iter()
            .map(|w| (w[1], w[2]))
            .collect()
    }

    #[test]
    fn at_004fc2e0_clears_then_sets_the_default_flags() {
        let mut e = Engine::new();
        e.register(0x0062_4700, |_, a| ret(a[0] + 0x48));
        e.register(0x0084_3170, ok);
        e.register(0x004f_c990, ok);
        e.call_log = Some(vec![]);
        e.call(0x004f_c2e0, &[0x3000]);
        let log = e.call_log.take().unwrap();
        assert_eq!(only(&log, 0x0084_3170), vec![vec![0x3048]]);
        assert_eq!(
            flag_calls(&log),
            vec![
                (0x1, 1),
                (0x4, 1),
                (0x8, 1),
                (0x40, 1),
                (0x200, 1),
                (0x800, 1),
                (0x4000_0000, 0),
                (0x2000_0000, 0),
                (0x1000_0000, 1),
                (0x8000_0000, 0),
            ]
        );
        // The clear comes first.
        let clear = log.iter().position(|(a, _)| *a == 0x0084_3170).unwrap();
        let set = log.iter().position(|(a, _)| *a == 0x004f_c990).unwrap();
        assert!(clear < set);
    }

    #[test]
    fn at_004fc270_restores_vtables_and_runs_the_member_and_base_destructors() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x50);
        e.register(0x0048_9070, ok);
        e.register(0x0050_e9f0, ok);
        e.call_log = Some(vec![]);
        e.call(0x004f_c270, &[this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.mem.u32(this), 0x0102_4f4c);
        assert_eq!(e.mem.u32(this + 0x30), 0x0102_4f28);
        let calls: Vec<_> = log.into_iter().skip(1).collect();
        assert_eq!(
            calls,
            vec![(0x0048_9070, vec![this + 0x30]), (0x0050_e9f0, vec![this])]
        );
    }

    fn resolve_engine(flag: bool, found: u32) -> (Engine, u32) {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x50);
        e.map(0x011c_a000, 0x1000);
        e.set_global::<u32>(0x011c_a53c, 0x7777);
        let flag_fn: AbiFn = if flag {
            |_, _| ret(1)
        } else {
            |_, _| ret(0x100)
        };
        e.register(0x0040_13e0, flag_fn);
        e.register(0x0048_4e60, |_, _| ret(0x5000));
        e.register(0x0048_5d50, |e, a| {
            let id = e.mem.u32(a[0]);
            e.mem.set_u32(a[0], id | 0x0100_0000);
            Ret::default()
        });
        e.register(0x0048_39c0, |_, a| ret(a[0] + 1));
        e.register_double(0x00ec_43fb, move |_, a| {
            ret(if a[0] == 0 { 0 } else { found })
        });
        e.register(0x0048_4ab0, ok);
        (e, this)
    }

    #[test]
    fn at_004fc4b0_resolves_the_form_or_falls_back_to_the_global() {
        // Flag 8 set: nothing happens.
        let (mut e, this) = resolve_engine(true, 0x9999);
        e.mem.set_u32(this + 0x4c, 0x42);
        e.call_log = Some(vec![]);
        e.call(0x004f_c4b0, &[this]);
        assert_eq!(e.call_log.take().unwrap().len(), 2);
        assert_eq!(e.mem.u32(this + 0x4c), 0x42);

        // An id: compile index, lookup, cast.
        let (mut e, this) = resolve_engine(false, 0x9999);
        e.mem.set_u32(this + 0x4c, 0x42);
        e.call_log = Some(vec![]);
        e.call(0x004f_c4b0, &[this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(only(&log, 0x0048_4e60), vec![vec![this, 0xffff_ffff]]);
        assert_eq!(only(&log, 0x0048_5d50)[0][1], 0x5000);
        assert_eq!(only(&log, 0x0048_39c0), vec![vec![0x0100_0042]]);
        assert_eq!(
            only(&log, 0x00ec_43fb),
            vec![vec![0x0100_0043, 0, 0x0118_3028, 0x0118_4118, 0]]
        );
        assert_eq!(e.mem.u32(this + 0x4c), 0x9999);
        assert_eq!(only(&log, 0x0048_4ab0), vec![vec![this, 1]]);

        // The cast fails: the global.
        let (mut e, this) = resolve_engine(false, 0);
        e.mem.set_u32(this + 0x4c, 0x42);
        e.call(0x004f_c4b0, &[this]);
        assert_eq!(e.mem.u32(this + 0x4c), 0x7777);

        // No id: no lookup, the global.
        let (mut e, this) = resolve_engine(false, 0x9999);
        e.call_log = Some(vec![]);
        e.call(0x004f_c4b0, &[this]);
        let log = e.call_log.take().unwrap();
        assert!(only(&log, 0x0048_39c0).is_empty());
        assert_eq!(e.mem.u32(this + 0x4c), 0x7777);
        assert_eq!(only(&log, 0x0048_4ab0), vec![vec![this, 1]]);
    }

    fn load_engine(chunks: Vec<u32>, version: u32, swaps: u32) -> (Engine, u32, u32) {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x50);
        let file = e.mem.alloc(0x300);
        e.put_vtable(0x0090_0000, &[0x00a0_0000; 0x50]);
        e.mem.set_u32(0x0090_0018, 0x00a0_0018);
        e.mem.set_u32(0x0090_00e0, 0x00a0_00e0);
        e.mem.set_u32(0x0090_0134, 0x00a0_0134);
        e.mem.set_u32(this, 0x0090_0000);
        for slot in [0x00a0_0018, 0x00a0_00e0, 0x00a0_0134] {
            e.register(slot, ok);
        }
        e.register(0x0047_2660, |_, _| ret(0x123));
        let mut next = 0usize;
        e.register_double(0x0047_26b0, move |_, _| {
            let c = chunks.get(next).copied().unwrap_or(0);
            next += 1;
            ret(c)
        });
        e.register(0x0047_26f0, |_, _| ret(1));
        e.register(0x0040_1660, |_, _| ret(0x10));
        e.register(0x0047_2890, |e, a| {
            if a[2] == 8 {
                e.mem.set_u32(a[1], 0x1111);
                e.mem.set_u32(a[1] + 4, 0x2222);
            }
            Ret::default()
        });
        e.register_double(0x0040_3570, move |_, _| ret(version));
        e.register_double(0x0040_1680, move |_, _| ret(swaps));
        e.register(0x0046_2230, ok);
        e.register(0x0053_5aa0, ok);
        e.register(0x004f_c790, ok);
        e.register(0x0048_61f0, |e, a| {
            e.mem.set_u32(a[2] + 4, 0xabcd);
            Ret::default()
        });
        e.register(0x0048_5110, ok);
        e.register(0x0048_4ab0, ok);
        e.register(0x0048_92d0, ok);
        e.register(0x004f_c990, ok);
        (e, this, file)
    }

    #[test]
    fn at_004fc540_loads_each_chunk() {
        // Wrong form type: false, nothing loaded.
        let (mut e, this, file) = load_engine(vec![], 8, 1);
        e.register(0x0047_2660, |_, _| ret(0x22));
        e.call_log = Some(vec![]);
        assert!(!e.call(0x004f_c540, &[this, file]).bool());
        assert_eq!(e.call_log.take().unwrap().len(), 2);

        let chunks = vec![
            0x4449_4445u32, // EDID
            0x444e_424f,    // OBND
            0x4c44_4f4d,    // MODL
            0x4154_4144,    // DATA
            0x4d41_4e44,    // DNAM
            0x1234_5678,    // not handled
            0x5444_4f4d,    // MODT
        ];
        let (mut e, this, file) = load_engine(chunks, 8, 1);
        e.call_log = Some(vec![]);
        assert!(e.call(0x004f_c540, &[this, file]).bool());
        let log = e.call_log.take().unwrap();
        assert_eq!(only(&log, 0x00a0_0018), vec![vec![this]]);
        assert_eq!(only(&log, 0x0048_5110), vec![vec![this, file]]);
        assert_eq!(only(&log, 0x0048_4ab0), vec![vec![this, 0]]);
        // EDID: at most 0x200 bytes into the buffer, which goes to slot 0x134.
        let reads = only(&log, 0x0047_2890);
        assert_eq!(reads[0][0], file);
        assert_eq!(reads[0][2], 0x200);
        assert_eq!(only(&log, 0x00a0_0134), vec![vec![this, reads[0][1]]]);
        assert_eq!(only(&log, 0x00a0_00e0), vec![vec![this, file]]);
        assert_eq!(
            only(&log, 0x0048_92d0),
            vec![vec![this + 0x30, file], vec![this + 0x30, file]]
        );
        // DATA: loaded and swapped; DNAM then filled +0x48/+0x4C and swapped.
        let load = only(&log, 0x0048_61f0);
        assert_eq!(load[0][..2], [this, file]);
        assert_eq!(load[0][3], 8);
        assert_eq!(only(&log, 0x004f_c790), vec![vec![load[0][2]]]);
        assert_eq!(only(&log, 0x0053_5aa0), vec![vec![load[0][2]]]);
        assert_eq!(reads[1], vec![file, this + 0x48, 8]);
        assert_eq!(e.mem.u32(this + 0x48), 0x1111);
        assert_eq!(e.mem.u32(this + 0x4c), 0x2222);
        assert_eq!(only(&log, 0x0046_2230), vec![vec![this + 0x48]]);
        assert_eq!(
            flag_calls(&log),
            vec![
                (0x1, 1),
                (0x4, 1),
                (0x8, 1),
                (0x40, 1),
                (0x200, 1),
                (0x800, 1),
                (0x4000_0000, 0),
                // DNAM with version 8.
                (0x4000_0000, 0),
            ]
        );
    }

    #[test]
    fn at_004fc540_data_fields_and_dnam_at_version_9() {
        // DATA without a swap: +4 of the block to +0x4C, +0x48 cleared.
        let (mut e, this, file) = load_engine(vec![0x4154_4144], 8, 0x100);
        e.mem.set_u32(this + 0x48, 0x5555);
        e.call_log = Some(vec![]);
        assert!(e.call(0x004f_c540, &[this, file]).bool());
        let log = e.call_log.take().unwrap();
        assert!(only(&log, 0x0053_5aa0).is_empty());
        assert_eq!(e.mem.u32(this + 0x48), 0);
        assert_eq!(e.mem.u32(this + 0x4c), 0xabcd);

        // DNAM with version 9 (the low 16 bits): the flag stays, no swap.
        let (mut e, this, file) = load_engine(vec![0x4d41_4e44], 0x10009, 0);
        e.call_log = Some(vec![]);
        assert!(e.call(0x004f_c540, &[this, file]).bool());
        let log = e.call_log.take().unwrap();
        assert!(flag_calls(&log).is_empty());
        assert!(only(&log, 0x0046_2230).is_empty());

        // The chunk walk stops when the next-chunk step fails.
        let (mut e, this, file) = load_engine(vec![0x444e_424f, 0x444e_424f], 9, 0);
        e.register(0x0047_26f0, |_, _| ret(0x100));
        e.call_log = Some(vec![]);
        assert!(e.call(0x004f_c540, &[this, file]).bool());
        let log = e.call_log.take().unwrap();
        assert_eq!(only(&log, 0x00a0_00e0).len(), 1);
    }
}

#[cfg(test)]
mod n037_tests {
    use super::*;

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    #[test]
    fn data_block_init_clears_the_two_fields_only() {
        let mut e = Engine::new();
        let block = e.mem.alloc(8);
        e.mem.write(block, &[0xcd; 8]);
        assert_eq!(e.call(0x004f_c790, &[block]).u32(), block);
        assert_eq!(e.mem.u32(block), 0xcdcd_0000);
        assert_eq!(e.mem.u32(block + 4), 0);
    }

    type Chunks = std::rc::Rc<std::cell::RefCell<Vec<Vec<u32>>>>;

    fn save_engine(swaps: bool) -> (Engine, Chunks) {
        let mut e = Engine::new();
        for addr in [
            FORM_START_FORM,
            FORM_CLOSE_FORM,
            MODEL_SAVE,
            DNAM_BLOCK_INIT,
        ] {
            e.register(addr, |_, _| Ret::default());
        }
        e.register(COPY_BYTES, |e, a| {
            for i in 0..a[2] {
                let b = e.mem.u8(a[1] + i);
                e.mem.set_u8(a[0] + i, b);
            }
            Ret::default()
        });
        e.register(FORM_ID, |_, a| ret(a[0] + 0xc));
        if swaps {
            e.register(SAVE_SWAPS, |_, _| ret(1));
        } else {
            e.register(SAVE_SWAPS, |_, _| ret(0x100));
        }
        e.register(DNAM_SWAP, |e, a| {
            for i in 0..2 {
                let w = e.mem.u32(a[0] + 4 * i);
                e.mem.set_u32(a[0] + 4 * i, w + 0x1000);
            }
            Ret::default()
        });
        let chunks = Chunks::default();
        let seen = chunks.clone();
        e.register_double(ADD_CHUNK_DATA, move |e, a| {
            seen.borrow_mut()
                .push(vec![a[0], a[2], e.mem.u32(a[1]), e.mem.u32(a[1] + 4)]);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        (e, chunks)
    }

    #[test]
    fn save_writes_model_and_dnam() {
        let (mut e, chunks) = save_engine(false);
        let this = e.mem.alloc(0x50);
        e.mem.set_u32(this + 0x48, 0x11);
        e.mem.set_u32(this + 0x4c, 0x2000);
        e.call(0x004f_c7b0, &[this]);
        e.mem.set_u32(this + 0x4c, 0);
        e.call(0x004f_c7b0, &[this]);
        assert_eq!(
            *chunks.borrow(),
            vec![
                vec![CHUNK_DNAM, 8, 0x11, 0x200c],
                vec![CHUNK_DNAM, 8, 0x11, 0]
            ]
        );
        let models: Vec<Vec<u32>> = e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == MODEL_SAVE)
            .map(|c| c.1.clone())
            .collect();
        assert_eq!(models[0], vec![this + 0x30, CHUNK_MODL, CHUNK_MODT]);
        let (mut e, chunks) = save_engine(true);
        let this = e.mem.alloc(0x50);
        e.call(0x004f_c7b0, &[this]);
        assert_eq!(*chunks.borrow(), vec![vec![CHUNK_DNAM, 8, 0x1000, 0x1000]]);
    }
}

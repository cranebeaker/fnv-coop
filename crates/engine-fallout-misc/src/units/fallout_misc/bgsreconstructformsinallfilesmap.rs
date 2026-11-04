//! `fallout/misc/saveload/bgsreconstructformsinallfilesmap.cpp` (Xbox PDB source unit), subsystem `fallout/misc`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! Functions marked `Translation incomplete` have an exception-unwinding
//! frame (`FS:[0]`) that is not translated; everything else in them is.

#[allow(unused_imports)]
use crate::prelude::*;

/// `00844db0(this, bucket_count)`: the hash-table base initializer below.
/// `00844130` and `008433d0` pass `0x25` as the bucket count.
const BUCKET_COUNT: u32 = 0x25;
/// `008453a0(this, 0, 0)` (`run_00845230`): runs after `00844a90` stores its
/// vtable.
const MAP_INITIALIZER: u32 = 0x0084_53a0;
/// `00aa1070`: the allocator (cdecl, byte count), as `00853270` uses it.
const ALLOCATE: u32 = 0x00aa_1070;
/// `00403d30` (`memset`, cdecl: destination, value, byte count).
const MEMSET: u32 = 0x0040_3d30;

// Translated from 00844db0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: stores the vtable `0107ed1c`, the bucket count
/// at +4 and 0 at +0xc, allocates `4 * count` bytes for the bucket table
/// (+8) and zeroes it. Returns `this`.
pub fn fn_00844db0(e: &mut Engine, this: Ptr, bucket_count: u32) -> Ptr {
    let t = this.addr();
    e.mem.set_u32(t, 0x0107_ed1c);
    e.mem.set_u32(t + 4, bucket_count);
    e.mem.set_u32(t + 0xc, 0);
    let bytes = e.mem.u32(t + 4) << 2;
    let table = e.call(ALLOCATE, &args![bytes]).u32();
    e.mem.set_u32(t + 8, table);
    let bytes = e.mem.u32(t + 4) << 2;
    let table = e.mem.u32(t + 8);
    e.call(MEMSET, &args![table, 0u32, bytes]);
    this
}

// Translated from 00844130 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: `00844db0(this, bucket_count)`, then the vtable
/// `0107ec48` is stored. Returns `this`.
pub fn fn_00844130(e: &mut Engine, this: Ptr, bucket_count: u32) -> Ptr {
    fn_00844db0(e, this, bucket_count);
    e.mem.set_u32(this.addr(), 0x0107_ec48);
    this
}

// Translated from 00844a90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: stores the vtable `0107ed08` and runs
/// `008453a0(this, 0, 0)`. Returns `this`.
pub fn fn_00844a90(e: &mut Engine, this: Ptr) -> Ptr {
    e.mem.set_u32(this.addr(), 0x0107_ed08);
    e.call(MAP_INITIALIZER, &args![this, 0u32, 0u32]);
    this
}

// Translated from 00843470 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: runs `00844a90` on `this` and on the member at
/// +0x10. Returns `this`.
// Translation incomplete: 00843470 - SEH exception unwinding is unsupported.
pub fn fn_00843470(e: &mut Engine, this: Ptr) -> Ptr {
    fn_00844a90(e, this);
    fn_00844a90(e, this.byte_add(0x10).cast());
    this
}

// Translated from 008433d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSReconstructFormsInAllFilesMap::BGSReconstructFormsInAllFilesMap`
/// (Xbox PDB): the base `00844130(this, 0x25)`, the vtable `0107ea74`, the
/// member at +0x10 through `00843470`, and 0 at +0x30. Returns `this`.
// Translation incomplete: 008433d0 - SEH exception unwinding is unsupported.
pub fn bgsreconstructformsinallfilesmap_constructor(e: &mut Engine, this: Ptr) -> Ptr {
    fn_00844130(e, this, BUCKET_COUNT);
    e.mem.set_u32(this.addr(), 0x0107_ea74);
    fn_00843470(e, this.byte_add(0x10).cast());
    e.mem.set_u32(this.addr() + 0x30, 0);
    this
}

// Translated from 00843220 (decompiled, FalloutNV.exe 1.4.0.525)
/// A forwarder: it hands `this` (`ECX`) to `00844ff0`, a destructor with an exception frame
/// that stores the vtable `0107ed3c` and runs `008454f0` and `00844fa0`, and returns
/// nothing. Parameters: one word, `this`; the disassembly of the function ends in a plain
/// `RET` and passes only `ECX` on. `00844ff0`'s disassembly ends with its exception-frame
/// unlink and returns no value (Ghidra types it void).
pub fn fn_00843220(e: &mut Engine, this: Ptr) {
    e.call(0x0084_4ff0, &args![this]);
}

/// The vtable of the `BSSimpleArray` instance with RTTI name `.?AV?$BSSimpleArray@UReconstructFormData@@$0EAA@@` (read at the word before the vtable: its complete object locator, then the type descriptor).
const ARRAY_VTABLE_0107ED08: u32 = 0x0107_ed08;
/// `008454f0` (unnamed in the engine map): `thiscall`, `this` plus one stack word (`RET 4`). When the
/// buffer pointer (+4) is set it runs the instance's element destroy step; with the word non-zero it
/// then calls the array's virtual slot +8 on the buffer and clears the buffer pointer and the reserved
/// size (+0xc); finally it clears the size (+8).
const ARRAY_RELEASE_008454F0: u32 = 0x0084_54f0;

// Translated from 00844ac0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor of the `BSSimpleArray` instance with RTTI name `.?AV?$BSSimpleArray@UReconstructFormData@@$0EAA@@` (`thiscall`): stores the
/// instance's vtable at `this`, then calls `008454f0(this, 1)`.
pub fn fn_00844ac0(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), ARRAY_VTABLE_0107ED08);
    e.call(ARRAY_RELEASE_008454F0, &args![this, 1u32]);
}

/// The `NiTMap` iteration (Xbox PDB name unconfirmed): the first position
/// (`004b9ba0`) and the next entry (`006b7f20`, `RET 0xC`: the position,
/// the key and the value, each by address).
const MAP_FIRST_POSITION: u32 = 0x004b_9ba0;
const MAP_NEXT: u32 = 0x006b_7f20;
/// Empties the map (`00438af0`).
const MAP_REMOVE_ALL: u32 = 0x0043_8af0;
/// The element count of one of the two arrays (`0044ddc0`).
const ARRAY_COUNT: u32 = 0x0044_ddc0;
/// The array's reallocation (`006a2770`, `RET 8`: new size, count kept),
/// its element destruction (`0072ba80`, `RET 8`: first element, count) and
/// construction (`00845300`, `RET 8`: first element, count).
const ARRAY_REALLOCATE: u32 = 0x006a_2770;
const ARRAY_DESTROY: u32 = 0x0072_ba80;
const ARRAY_CONSTRUCT: u32 = 0x0084_5300;
/// The `NiTMap<TESFile *, BGSReconstructFormsInFileMap *>` vtable (stored
/// by `00844130` and `00844e20`) and `BGSReconstructFormsInAllFilesMap`'s.
const FILE_MAP_VTABLE: u32 = 0x0107_ec48;
const ALL_FILES_MAP_VTABLE: u32 = 0x0107_ea74;
/// The TLS word whose bit 4 `00843c10` sets or clears (`+0x294`).
const TLS_FLAGS: u32 = 0x294;

// Translated from 00843150 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the byte at `+0x14`.
pub fn fn_00843150(e: &mut Engine, this: Ptr) -> u8 {
    e.mem.u8(this.addr() + 0x14)
}

// Translated from 00843170 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: clears the words at `+0` and `+4`.
pub fn fn_00843170(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), 0);
    e.mem.set_u32(this.addr() + 4, 0);
}

// Translated from 00843240 (decompiled, FalloutNV.exe 1.4.0.525)
/// Empties both arrays (at `+0` and `+0x10`) and frees their memory
/// (`00844ae0` with 0 and 1).
pub fn fn_00843240(e: &mut Engine, this: Ptr) {
    fn_00844ae0(e, this, 0, 1);
    fn_00844ae0(e, this.byte_add(0x10), 0, 1);
}

// Translated from 00843440 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSReconstructFormsInAllFilesMap::_scalar_deleting_destructor_` (Xbox
/// PDB): the destructor (`008434d0`), then frees `this` (`00401030`) when
/// bit 0 of `flags` is set. Returns `this`.
pub fn bgs_reconstruct_forms_in_all_files_map_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    bgs_reconstruct_forms_in_all_files_map_destructor(e, this);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

// Translated from 008434d0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 008434d0 - the exception-unwinding frame (FS:[0]) is not translated.
/// `BGSReconstructFormsInAllFilesMap::~BGSReconstructFormsInAllFilesMap`
/// (Xbox PDB): stores the vtable `0107ea74`, deletes the per-file maps
/// (`008435a0`), destroys the two arrays at `+0x10` (`00843540`) and runs
/// the `NiTMap` destructor (`00844e20`).
pub fn bgs_reconstruct_forms_in_all_files_map_destructor(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), ALL_FILES_MAP_VTABLE);
    fn_008435a0(e, this);
    fn_00843540(e, this.byte_add(0x10));
    fn_00844e20(e, this);
}

// Translated from 00843540 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00843540 - the exception-unwinding frame (FS:[0]) is not translated.
/// Destroys the two arrays: `00844ac0` on `+0x10`, then on `+0`.
pub fn fn_00843540(e: &mut Engine, this: Ptr) {
    fn_00844ac0(e, this.byte_add(0x10));
    fn_00844ac0(e, this);
}

// Translated from 008435a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Deletes every per-file map: walks the map (`004b9ba0`, `006b7f20` with a
/// cleared key and value), running vtable slot 0 with 1 (the deleting
/// destructor) on each value that is not null; then empties the map
/// (`00438af0`), empties both arrays (`00843240` on `+0x10`) and clears
/// the word at `+0x30`.
pub fn fn_008435a0(e: &mut Engine, this: Ptr) {
    e.with_stack(12, |e, locals| {
        let position_at = locals;
        let key_at = locals.byte_add(4);
        let value_at = locals.byte_add(8);
        let position = e.call(MAP_FIRST_POSITION, &args![this]).u32();
        e.mem.set_u32(position_at.addr(), position);
        while e.mem.u32(position_at.addr()) != 0 {
            e.mem.set_u32(key_at.addr(), 0);
            e.mem.set_u32(value_at.addr(), 0);
            e.call(MAP_NEXT, &args![this, position_at, key_at, value_at]);
            let value = e.mem.u32(value_at.addr());
            if value != 0 {
                e.vcall(value, 0, &args![1u32]);
            }
        }
    });
    e.call(MAP_REMOVE_ALL, &args![this]);
    fn_00843240(e, this.byte_add(0x10));
    e.mem.set_u32(this.addr() + 0x30, 0);
}

// Translated from 008439e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: reconstructs the forms of every file. Sets
/// the TLS flag (`00843c10` with 1 on `*011ddf38`), reserves `+0x30`
/// entries in both arrays (`00843be0`), saves the answer of `004516b0` on
/// `*011c3f2c` and sets it with `00550890(1)`; for every entry of the map
/// (`004b9ba0`, `006b7f20`), `BGSReconstructFormsInFileMap::
/// ReconstructForms` (`008427f0`) runs on the value with the key's
/// thread-safe file (`TESFile::GetThreadSafeFile`, `004739b0`) and the
/// arrays; the saved setting is put back. Then every entry of the arrays
/// (`00843bb0` entries, `008432e0`) whose form answers true to vtable slot
/// 0xF0 but has no base form (`007af430`) is removed (`00843380`) and the
/// index stays; every form without form flag 0x8 (`004013e0`) runs vtable
/// slot 0x88; every entry goes to `00843c90`. Finally the TLS flag is
/// cleared.
pub fn fn_008439e0(e: &mut Engine, this: Ptr) {
    let arrays = this.byte_add(0x10);
    let tls_owner = Ptr::new(e.global::<u32>(0x011d_df38));
    fn_00843c10(e, tls_owner, true);
    let reserve = e.mem.u32(this.addr() + 0x30);
    fn_00843be0(e, arrays, reserve);
    let setting_owner = e.global::<u32>(0x011c_3f2c);
    let saved = e.call(0x0045_16b0, &args![setting_owner]).u8();
    let setting_owner = e.global::<u32>(0x011c_3f2c);
    e.call(0x0055_0890, &args![setting_owner, 1u32]);
    e.with_stack(12, |e, locals| {
        let position_at = locals;
        let key_at = locals.byte_add(4);
        let value_at = locals.byte_add(8);
        let position = e.call(MAP_FIRST_POSITION, &args![this]).u32();
        e.mem.set_u32(position_at.addr(), position);
        while e.mem.u32(position_at.addr()) != 0 {
            e.mem.set_u32(key_at.addr(), 0);
            e.mem.set_u32(value_at.addr(), 0);
            e.call(MAP_NEXT, &args![this, position_at, key_at, value_at]);
            let key = e.mem.u32(key_at.addr());
            let file = e.call(0x0047_39b0, &args![key]).u32();
            let value = e.mem.u32(value_at.addr());
            e.call(0x0084_27f0, &args![value, file, arrays]);
        }
    });
    let setting_owner = e.global::<u32>(0x011c_3f2c);
    e.call(0x0055_0890, &args![setting_owner, u32::from(saved)]);
    let mut index = 0u32;
    while index < fn_00843bb0(e, arrays) {
        let entry = e.call(0x0084_32e0, &args![arrays, index]).u32();
        let form = e.mem.u32(entry);
        if e.vcall(form, 0xf0, &args![]).bool() {
            let form = e.mem.u32(entry);
            if e.call(0x007a_f430, &args![form]).u32() == 0 {
                e.call(0x0084_3380, &args![arrays, index]);
                index = index.wrapping_sub(1);
            }
        }
        index = index.wrapping_add(1);
    }
    let count = fn_00843bb0(e, arrays);
    for index in 0..count {
        let entry = e.call(0x0084_32e0, &args![arrays, index]).u32();
        let form = e.mem.u32(entry);
        if !e.call(0x0040_13e0, &args![form]).bool() {
            let form = e.mem.u32(entry);
            e.vcall(form, 0x88, &args![]);
        }
    }
    for index in 0..count {
        let entry = e.call(0x0084_32e0, &args![arrays, index]).u32();
        e.call(0x0084_3c90, &args![this, entry]);
    }
    let tls_owner = Ptr::new(e.global::<u32>(0x011d_df38));
    fn_00843c10(e, tls_owner, false);
}

// Translated from 00843bb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The entries of both arrays: `0044ddc0` on `+0` plus on `+0x10`.
pub fn fn_00843bb0(e: &mut Engine, this: Ptr) -> u32 {
    let first = e.call(ARRAY_COUNT, &args![this]).u32();
    let second = e.call(ARRAY_COUNT, &args![this.addr() + 0x10]).u32();
    second.wrapping_add(first)
}

// Translated from 00843be0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the allocated size of both arrays (`00844c20` on `+0` and
/// `+0x10`); `RET 4`.
pub fn fn_00843be0(e: &mut Engine, this: Ptr, size: u32) {
    fn_00844c20(e, this, size);
    fn_00844c20(e, this.byte_add(0x10), size);
}

// Translated from 00843c10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets (`set` true) or clears bit 4 of the word at `+0x294` of the
/// calling thread's TLS block; `this` is not read; `RET 4`.
pub fn fn_00843c10(e: &mut Engine, _this: Ptr, set: bool) {
    let tls = e.tls();
    let flags = e.mem.u32(tls + TLS_FLAGS);
    let flags = if set { flags | 4 } else { flags & 0xffff_fffb };
    e.mem.set_u32(tls + TLS_FLAGS, flags);
}

// Translated from 00844ae0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `BSSimpleArray` (8-byte elements: buffer `+4`, size `+8`, allocated
/// size `+0xC`) `SetSize`, `RET 8`: size 0 releases the array
/// (`008454f0` with `free`). Growing past the allocated size allocates
/// (vtable slot 4 with the size, when nothing is allocated) or reallocates
/// (`006a2770` with the size and the old size), then constructs the new
/// elements (`00845300`); growing within it only constructs them;
/// shrinking destroys the dropped elements (`0072ba80`) and, with `free`
/// and the new size at most a quarter of the allocated one, reallocates to
/// the new size.
pub fn fn_00844ae0(e: &mut Engine, this: Ptr, size: u32, free: u8) {
    let t = this.addr();
    if size == 0 {
        e.call(0x0084_54f0, &args![this, u32::from(free)]);
        return;
    }
    let element = |e: &Engine, index: u32| e.mem.u32(t + 4).wrapping_add(index.wrapping_mul(8));
    if size > e.mem.u32(t + 0xc) {
        if e.mem.u32(t + 0xc) == 0 {
            let buffer = e.vcall(t, 4, &args![size]).u32();
            e.mem.set_u32(t + 4, buffer);
            e.mem.set_u32(t + 0xc, size);
        } else {
            let old = e.mem.u32(t + 8);
            e.call(ARRAY_REALLOCATE, &args![this, size, old]);
            e.mem.set_u32(t + 0xc, size);
        }
        let old = e.mem.u32(t + 8);
        let first = element(e, old);
        e.call(ARRAY_CONSTRUCT, &args![this, first, size.wrapping_sub(old)]);
        e.mem.set_u32(t + 8, size);
    } else if size < e.mem.u32(t + 8) {
        let old = e.mem.u32(t + 8);
        let first = element(e, size);
        e.call(ARRAY_DESTROY, &args![this, first, old.wrapping_sub(size)]);
        e.mem.set_u32(t + 8, size);
        if free != 0 && size <= e.mem.u32(t + 0xc) >> 2 {
            e.call(ARRAY_REALLOCATE, &args![this, size, size]);
            e.mem.set_u32(t + 0xc, size);
        }
    } else {
        let old = e.mem.u32(t + 8);
        let first = element(e, old);
        e.call(ARRAY_CONSTRUCT, &args![this, first, size.wrapping_sub(old)]);
        e.mem.set_u32(t + 8, size);
    }
}

// Translated from 00844c20 (decompiled, FalloutNV.exe 1.4.0.525)
/// The same array's allocated-size setter, `RET 4`: nothing when it is
/// already `size`; elements past `size` are destroyed (`0072ba80`) and the
/// size cut; then the buffer is reallocated (`006a2770` with `size` and
/// the size) and the allocated size set.
pub fn fn_00844c20(e: &mut Engine, this: Ptr, size: u32) {
    let t = this.addr();
    if size == e.mem.u32(t + 0xc) {
        return;
    }
    if size < e.mem.u32(t + 8) {
        let old = e.mem.u32(t + 8);
        let first = e.mem.u32(t + 4).wrapping_add(size.wrapping_mul(8));
        e.call(ARRAY_DESTROY, &args![this, first, old.wrapping_sub(size)]);
        e.mem.set_u32(t + 8, size);
    }
    let count = e.mem.u32(t + 8);
    e.call(ARRAY_REALLOCATE, &args![this, size, count]);
    e.mem.set_u32(t + 0xc, size);
}

// Translated from 00844e20 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00844e20 - the exception-unwinding frame (FS:[0]) is not translated.
/// The `NiTMap<TESFile *, BGSReconstructFormsInFileMap *>` destructor
/// (vtable `0107ec48`): empties the map (`00438af0`), then runs the base
/// destructor `00844e80`.
pub fn fn_00844e20(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), FILE_MAP_VTABLE);
    e.call(MAP_REMOVE_ALL, &args![this]);
    e.call(0x0084_4e80, &args![this]);
}

/// `TESForm::GetFile` (engine map, `00484e60`; `thiscall`, one word: the index, -1 for the
/// last file).
const FORM_GET_FILE: u32 = 0x0048_4e60;
/// `00440e30` (`thiscall` on a form, no stack words): the text `AddForm`'s warning prints
/// as the form's type.
const TYPE_NAME_00440E30: u32 = 0x0044_0e30;
/// `0084e3a0` (`thiscall`: the word at +0xC, a form's id).
const FORM_ID: u32 = 0x0084_e3a0;
/// The forms' vtable slot `AddForm` and `AddReference` print as the name (+0x130).
const FORM_NAME_SLOT: u32 = 0x130;
/// `SaveGameWarning` (engine map, `00845cc0`; variadic `cdecl`: format and values).
const SAVE_GAME_WARNING: u32 = 0x0084_5cc0;
/// `"Form %08X %s with type %s and no file passed to
/// BGSReconstructFormsInAllFilesMap::AddForm()"` and `"Cell %08X %s with no file passed to
/// BGSReconstructFormsInAllFilesMap::AddReference()"`.
const NO_FILE_FORM_MESSAGE: u32 = 0x0107_ea90;
const NO_FILE_CELL_MESSAGE: u32 = 0x0107_eaf0;
/// The file map's look-up (`00853130`, `thiscall`, two words: key and the value's
/// address; answers whether found) and `SetAt` (`00844700`, `thiscall`: key, value).
const FILE_MAP_GET_AT: u32 = 0x0085_3130;
const FILE_MAP_SET_AT: u32 = 0x0084_4700;
/// `operator new` (`00401000`, `cdecl`, one word) and
/// `BGSReconstructFormsInFileMap::BGSReconstructFormsInFileMap` (engine map, `00842180`;
/// `thiscall`, no stack words) on a 0x1C-byte block.
const OPERATOR_NEW: u32 = 0x0040_1000;
const FILE_MAP_CONSTRUCT: u32 = 0x0084_2180;
const FILE_MAP_SIZE: u32 = 0x1c;
/// `BGSReconstructFormsInFileMap::AddForm` (engine map, `00842310`; `thiscall`, two words)
/// and `AddCellForm` (`00842600`; `thiscall`, four words).
const FILE_MAP_ADD_FORM: u32 = 0x0084_2310;
const FILE_MAP_ADD_CELL_FORM: u32 = 0x0084_2600;
/// The count of added forms (`+0x30`).
const ADDED_COUNT: u32 = 0x30;
/// `00425fd0` (`thiscall` on a form: a byte), `00460140` (`thiscall`, no stack words),
/// `TESObjectCELL::GetWorldSpace` (engine map, `0054ddd0`; `thiscall`, no stack words) and
/// `BGSReconstructFormsInAllFilesMap::GetFileForFormID` (engine map, `00843fe0`;
/// `thiscall`, two words).
const FORM_TEST_00425FD0: u32 = 0x0042_5fd0;
const FORM_KEY_00460140: u32 = 0x0046_0140;
const CELL_GET_WORLD_SPACE: u32 = 0x0054_ddd0;
const GET_FILE_FOR_FORM_ID: u32 = 0x0084_3fe0;

/// The file's `BGSReconstructFormsInFileMap`: looked up in the map (`00853130`), or a new
/// one (`00842180` on a 0x1C-byte block, null when the allocation fails) set at the file
/// (`00844700`).
fn file_map_for(e: &mut Engine, this: Ptr, file: u32) -> u32 {
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), 0);
        if !e.call(FILE_MAP_GET_AT, &args![this, file, slot]).bool() {
            let block = e.call(OPERATOR_NEW, &args![FILE_MAP_SIZE]).u32();
            let map = if block != 0 {
                e.call(FILE_MAP_CONSTRUCT, &args![block]).u32()
            } else {
                0
            };
            e.mem.set_u32(slot.addr(), map);
            e.call(FILE_MAP_SET_AT, &args![this, file, map]);
        }
        e.mem.u32(slot.addr())
    })
}

/// `+0x30` counts up.
fn count_added(e: &mut Engine, this: Ptr) {
    let count = e.mem.u32(this.addr() + ADDED_COUNT);
    e.mem
        .set_u32(this.addr() + ADDED_COUNT, count.wrapping_add(1));
}

// Translated from 00843630 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00843630 - the compiler's exception frame (FS:[0] chain, handler 00f0b23b, state 0 that frees the 0x1c-byte block when its constructor throws) is not translated
/// `BGSReconstructFormsInAllFilesMap::AddForm` (engine map; `thiscall`, two words: the form
/// and a word handed on; `RET 8`). Nothing for a null form. A form without a file
/// (`TESForm::GetFile(-1)`) gets the warning `"Form %08X %s with type %s and no file ..."`
/// (its id, its name from vtable +0x130 and `00440e30`'s text). Otherwise the form goes to
/// its file's map (found, or made and set) with `BGSReconstructFormsInFileMap::AddForm`,
/// and `+0x30` counts up.
pub fn bgs_reconstruct_forms_in_all_files_map_add_form(
    e: &mut Engine,
    this: Ptr,
    form: u32,
    data: u32,
) {
    if form == 0 {
        return;
    }
    let file = e.call(FORM_GET_FILE, &args![form, u32::MAX]).u32();
    if file == 0 {
        let type_text = e.call(TYPE_NAME_00440E30, &args![form]).u32();
        let name = e.vcall(form, FORM_NAME_SLOT, &args![]).u32();
        let id = e.call(FORM_ID, &args![form]).u32();
        e.call(
            SAVE_GAME_WARNING,
            &args![NO_FILE_FORM_MESSAGE, id, name, type_text],
        );
        return;
    }
    let map = file_map_for(e, this, file);
    e.call(FILE_MAP_ADD_FORM, &args![map, form, data]);
    count_added(e, this);
}

// Translated from 00843750 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00843750 - the compiler's exception frame (FS:[0] chain, handler 00f0b23b, state 0 that frees the 0x1c-byte block when its constructor throws) is not translated
/// `BGSReconstructFormsInAllFilesMap::AddReference` (engine map; `thiscall`, three words: a
/// form, then two words handed on; `RET 0xc`). Nothing for a null form. The file is
/// `GetFileForFormID(00460140(form), first)` when the form's `00425fd0` answers true, else
/// `GetFileForFormID(00460140(GetWorldSpace(form)), first)`. Without a file the warning
/// `"Cell %08X %s with no file ..."` (its id and its name from vtable +0x130); otherwise the
/// form goes to the file's map (found, or made and set) with
/// `BGSReconstructFormsInFileMap::AddCellForm(form, 0, first, second)`, and `+0x30` counts
/// up.
pub fn bgs_reconstruct_forms_in_all_files_map_add_reference(
    e: &mut Engine,
    this: Ptr,
    form: u32,
    first: u32,
    second: u32,
) {
    if form == 0 {
        return;
    }
    let file = if e.call(FORM_TEST_00425FD0, &args![form]).bool() {
        let key = e.call(FORM_KEY_00460140, &args![form]).u32();
        e.call(GET_FILE_FOR_FORM_ID, &args![this, key, first]).u32()
    } else {
        let world = e.call(CELL_GET_WORLD_SPACE, &args![form]).u32();
        let key = e.call(FORM_KEY_00460140, &args![world]).u32();
        e.call(GET_FILE_FOR_FORM_ID, &args![this, key, first]).u32()
    };
    if file == 0 {
        let name = e.vcall(form, FORM_NAME_SLOT, &args![]).u32();
        let id = e.call(FORM_ID, &args![form]).u32();
        e.call(SAVE_GAME_WARNING, &args![NO_FILE_CELL_MESSAGE, id, name]);
        return;
    }
    let map = file_map_for(e, this, file);
    e.call(
        FILE_MAP_ADD_CELL_FORM,
        &args![map, form, 0u32, first, second],
    );
    count_added(e, this);
}

// --- The reconstruct maps (BGSReconstructFormsMap .. BGSReconstructFormsInFileMap) ---

/// `operator delete` (`00401030`, `cdecl`, the block).
const OPERATOR_DELETE: u32 = 0x0040_1030;
/// The classes' vtables, each named by the RTTI type descriptor its complete object locator
/// (the word before the vtable) points at.
/// `.?AVBGSReconstructFormsMap@@`
const FORMS_MAP_VTABLE: u32 = 0x0107_e930;
/// `.?AVBGSReconstructCellsMap@@`
const CELLS_MAP_VTABLE: u32 = 0x0107_e950;
/// `.?AVBGSReconstructCellSubBlocksMap@@`
const SUB_BLOCKS_MAP_VTABLE: u32 = 0x0107_e970;
/// `.?AVBGSReconstructCellBlocksMap@@`
const CELL_BLOCKS_MAP_VTABLE: u32 = 0x0107_e990;
/// `.?AVBGSReconstructWorldSpacesMap@@`
const WORLD_SPACES_MAP_VTABLE: u32 = 0x0107_e9b0;
/// `.?AVBGSReconstructFormsInFileMap@@`
const FORMS_IN_FILE_MAP_VTABLE: u32 = 0x0107_e9d0;
/// `.?AVBGSOffsetDataArray@@`
const OFFSET_DATA_ARRAY_VTABLE: u32 = 0x0107_ea60;
/// `.?AV?$BSScrapArray@UOffsetData@@$0EAA@@@`
const SCRAP_ARRAY_VTABLE: u32 = 0x0107_ed3c;
/// `NiTMap<unsigned int, ReconstructFormData>`, `NiTMap<unsigned int, BGSReconstructFormsMap *>`,
/// `NiTMap<unsigned int, BGSReconstructCellsMap *>`, `NiTMap<unsigned int,
/// BGSReconstructCellSubBlocksMap *>` and `NiTMap<unsigned int, BGSReconstructCellBlocksMap *>`
/// (RTTI).
const FORM_DATA_NITMAP_VTABLE: u32 = 0x0107_eba8;
const FORMS_MAP_NITMAP_VTABLE: u32 = 0x0107_ebc8;
const CELLS_MAP_NITMAP_VTABLE: u32 = 0x0107_ebe8;
const SUB_BLOCKS_MAP_NITMAP_VTABLE: u32 = 0x0107_ec08;
const CELL_BLOCKS_MAP_NITMAP_VTABLE: u32 = 0x0107_ec28;
/// The `NiTMapBase<DFALL<..>, unsigned int, ..>` vtables of `ReconstructFormData`,
/// `BGSReconstructCellSubBlocksMap *` and `BGSReconstructCellBlocksMap *` values (RTTI).
const FORM_DATA_NITMAP_BASE_VTABLE: u32 = 0x0107_ec68;
const SUB_BLOCKS_MAP_NITMAP_BASE_VTABLE: u32 = 0x0107_ecc8;
const CELL_BLOCKS_MAP_NITMAP_BASE_VTABLE: u32 = 0x0107_ece8;
/// The `NiTMapBase` constructors of the `BGSReconstructFormsMap *` and
/// `BGSReconstructCellsMap *` maps (`00844590`, `00844690`; `thiscall`, the bucket count;
/// another unit's range in the engine map).
const FORMS_MAP_NITMAP_BASE_CONSTRUCT: u32 = 0x0084_4590;
const CELLS_MAP_NITMAP_BASE_CONSTRUCT: u32 = 0x0084_4690;
/// The base destructors the destructors here end with (`thiscall`, no stack words; not this
/// unit's): `00844470` (`NiTMap<unsigned int, ReconstructFormData>`), `00844600` (`NiTMap<unsigned
/// int, BGSReconstructFormsMap *>`), `00844840`, `00844940`, `00844a40` (after
/// `CELLS_MAP_NITMAP_VTABLE`, `SUB_BLOCKS_MAP_NITMAP_VTABLE`, `CELL_BLOCKS_MAP_NITMAP_VTABLE`),
/// and the ones the `NiTMapBase` and `BSSimpleArray` deleting destructors run: `008444d0`,
/// `00844660`, `00844e80`, `00844fa0`.
const FORM_DATA_NITMAP_DESTRUCT: u32 = 0x0084_4470;
const FORMS_MAP_NITMAP_DESTRUCT: u32 = 0x0084_4600;
const CELLS_MAP_NITMAP_BASE_DESTRUCT: u32 = 0x0084_4840;
const SUB_BLOCKS_MAP_NITMAP_BASE_DESTRUCT: u32 = 0x0084_4940;
const CELL_BLOCKS_MAP_NITMAP_BASE_DESTRUCT: u32 = 0x0084_4a40;
const FORM_DATA_NITMAP_BASE_DESTRUCT: u32 = 0x0084_44d0;
const FORMS_MAP_NITMAP_BASE_DESTRUCT: u32 = 0x0084_4660;
const FILE_MAP_NITMAP_BASE_DESTRUCT: u32 = 0x0084_4e80;
const OFFSET_DATA_ARRAY_BASE_DESTRUCT: u32 = 0x0084_4fa0;
/// `0096a2d0` (`thiscall` on an 8-byte local, no stack words): the local's constructor.
const PAIR_CONSTRUCT: u32 = 0x0096_a2d0;
/// `004b0680` (`thiscall` on an 8-byte local: a form and a word; `RET 8`): fills it and
/// answers its address.
const FORM_DATA_MAKE: u32 = 0x004b_0680;
/// The `ReconstructFormData` map's set (`00844310`, `thiscall`: key, the value's two words)
/// and look-up (`008443f0`, `thiscall`: key, the value's address; answers whether found).
const FORM_DATA_SET_AT: u32 = 0x0084_4310;
const FORM_DATA_GET_AT: u32 = 0x0084_43f0;
/// `00565450` (`thiscall` on a form, a `bool` in `AL`).
const FORM_TEST_00565450: u32 = 0x0056_5450;
/// `00401170` (`thiscall` on a form): the form's type byte (`TESForm::cFormType`, +4).
const FORM_TYPE: u32 = 0x0040_1170;
/// `TESObjectCELL::GetDataX` / `GetDataY` (engine map, `00544c30` / `00544c60`).
const CELL_GET_DATA_X: u32 = 0x0054_4c30;
const CELL_GET_DATA_Y: u32 = 0x0054_4c60;
/// `00587410` (`cdecl`: x, y), the cell-coordinate key of the world-space unit.
const CELL_COORDINATE_KEY: u32 = 0x0058_7410;
/// `00452420` (`thiscall` on a `BGSReconstructFormsMap`: a mask; a `bool` in `AL`).
const FORMS_MAP_TEST_FLAGS: u32 = 0x0045_2420;
/// The sub-block and block keys of a cell (`00544210`, `005441b0`; `thiscall`) and of
/// exterior coordinates (`TESObjectCELL::CalcExtGroupSubBlockKey` `005442c0`,
/// `CalcExtGroupBlockKey` `00544280`; `cdecl`: x, y).
const CELL_SUB_BLOCK_KEY: u32 = 0x0054_4210;
const CELL_BLOCK_KEY: u32 = 0x0054_41b0;
const EXTERIOR_SUB_BLOCK_KEY: u32 = 0x0054_42c0;
const EXTERIOR_BLOCK_KEY: u32 = 0x0054_4280;
/// The sizes `operator new` gets for the maps.
const FORMS_MAP_SIZE: u32 = 0x14;
const CELLS_MAP_SIZE: u32 = 0x10;
const CELL_BLOCKS_MAP_SIZE: u32 = 0x18;
const WORLD_SPACES_MAP_SIZE: u32 = 0x10;

/// The value of `key` in the `NiTMap` at `map` (`00853130` into a cleared slot), or a new
/// one: `operator new(size)` and `make` on the block (0 when the allocation fails), set at the
/// key (`00844700`).
fn value_or_new(
    e: &mut Engine,
    map: u32,
    key: u32,
    size: u32,
    make: fn(&mut Engine, u32) -> u32,
) -> u32 {
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), 0);
        if !e.call(FILE_MAP_GET_AT, &args![map, key, slot]).bool() {
            let block = e.call(OPERATOR_NEW, &args![size]).u32();
            let value = if block != 0 { make(e, block) } else { 0 };
            e.mem.set_u32(slot.addr(), value);
            e.call(FILE_MAP_SET_AT, &args![map, key, value]);
        }
        e.mem.u32(slot.addr())
    })
}

/// The word at `owner + offset`, made first when it is 0: `operator new(size)` and `make` on
/// the block (0 when the allocation fails).
fn member_or_new(
    e: &mut Engine,
    owner: u32,
    offset: u32,
    size: u32,
    make: fn(&mut Engine, u32) -> u32,
) -> u32 {
    if e.mem.u32(owner + offset) == 0 {
        let block = e.call(OPERATOR_NEW, &args![size]).u32();
        let value = if block != 0 { make(e, block) } else { 0 };
        e.mem.set_u32(owner + offset, value);
    }
    e.mem.u32(owner + offset)
}

fn make_forms_map(e: &mut Engine, block: u32) -> u32 {
    bgs_reconstruct_forms_map_constructor(e, Ptr::new(block)).addr()
}
fn make_cells_map(e: &mut Engine, block: u32) -> u32 {
    fn_00841340(e, Ptr::new(block)).addr()
}
fn make_sub_blocks_map(e: &mut Engine, block: u32) -> u32 {
    fn_008416c0(e, Ptr::new(block)).addr()
}
fn make_interior_blocks_map(e: &mut Engine, block: u32) -> u32 {
    bgs_reconstruct_cell_blocks_map_constructor(e, Ptr::new(block), 0).addr()
}
fn make_exterior_blocks_map(e: &mut Engine, block: u32) -> u32 {
    bgs_reconstruct_cell_blocks_map_constructor(e, Ptr::new(block), 1).addr()
}
fn make_world_spaces_map(e: &mut Engine, block: u32) -> u32 {
    fn_00841ea0(e, Ptr::new(block)).addr()
}

/// Walks the map (`004b9ba0`, `006b7f20` with a cleared key and value) and runs vtable slot
/// 0 with 1 (the deleting destructor) on every value that is not null.
fn delete_values(e: &mut Engine, this: Ptr) {
    e.with_stack(12, |e, locals| {
        let position_at = locals;
        let key_at = locals.byte_add(4);
        let value_at = locals.byte_add(8);
        let position = e.call(MAP_FIRST_POSITION, &args![this]).u32();
        e.mem.set_u32(position_at.addr(), position);
        while e.mem.u32(position_at.addr()) != 0 {
            e.mem.set_u32(key_at.addr(), 0);
            e.mem.set_u32(value_at.addr(), 0);
            e.call(MAP_NEXT, &args![this, position_at, key_at, value_at]);
            let value = e.mem.u32(value_at.addr());
            if value != 0 {
                e.vcall(value, 0, &args![1u32]);
            }
        }
    });
}

/// Walks the map (`004b9ba0`, `006b7f20` with a cleared key and value) and answers true at the
/// first value `test` answers true for.
fn any_value(e: &mut Engine, this: Ptr, test: impl Fn(&mut Engine, u32) -> bool) -> bool {
    e.with_stack(12, |e, locals| {
        let position_at = locals;
        let key_at = locals.byte_add(4);
        let value_at = locals.byte_add(8);
        let position = e.call(MAP_FIRST_POSITION, &args![this]).u32();
        e.mem.set_u32(position_at.addr(), position);
        while e.mem.u32(position_at.addr()) != 0 {
            e.mem.set_u32(key_at.addr(), 0);
            e.mem.set_u32(value_at.addr(), 0);
            e.call(MAP_NEXT, &args![this, position_at, key_at, value_at]);
            let value = e.mem.u32(value_at.addr());
            if test(e, value) {
                return true;
            }
        }
        false
    })
}

/// A scalar deleting destructor's tail: frees `this` (`00401030`) when bit 0 of `flags` is
/// set; answers `this`.
fn free_if(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

/// An `NiTMapBase` constructor's body (as `00844db0`): the vtable, the bucket count at +4, 0
/// at +0xC, a bucket table of `4 * count` bytes (`00aa1070`) at +8, zeroed (`00403d30`).
fn map_base_construct(e: &mut Engine, this: Ptr, vtable: u32, bucket_count: u32) -> Ptr {
    let t = this.addr();
    e.mem.set_u32(t, vtable);
    e.mem.set_u32(t + 4, bucket_count);
    e.mem.set_u32(t + 0xc, 0);
    let bytes = e.mem.u32(t + 4) << 2;
    let table = e.call(ALLOCATE, &args![bytes]).u32();
    e.mem.set_u32(t + 8, table);
    let bytes = e.mem.u32(t + 4) << 2;
    let table = e.mem.u32(t + 8);
    e.call(MEMSET, &args![table, 0u32, bytes]);
    this
}

// Translated from 006db9b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSScrapArray<OffsetData,1024>::_Allocate` (Xbox PDB): `count` 8-byte elements from the
/// array's scrap heap (+0x10), `ScrapHeap::Allocate(count * 8, *010a2720)`; answers the
/// block. `RET 4`.
pub fn bs_scrap_array_offset_data_allocate(e: &mut Engine, this: Ptr, count: u32) -> u32 {
    let alignment = e.global::<u32>(SCRAP_ALIGNMENT);
    let heap = e.mem.u32(this.addr() + 0x10);
    e.call(SCRAP_HEAP_ALLOCATE, &args![heap, count << 3, alignment])
        .u32()
}

// Translated from 00841190 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSReconstructFormsMap::BGSReconstructFormsMap` (Xbox PDB): the `NiTMap<unsigned int,
/// ReconstructFormData>` constructor (`00844040` with 0x25 buckets), its vtable and 0 in the
/// flags at +0x10. Answers `this`.
pub fn bgs_reconstruct_forms_map_constructor(e: &mut Engine, this: Ptr) -> Ptr {
    fn_00844040(e, this, BUCKET_COUNT);
    e.mem.set_u32(this.addr(), FORMS_MAP_VTABLE);
    e.mem.set_u32(this.addr() + 0x10, 0);
    this
}

// Translated from 008411c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSReconstructFormsMap::_scalar_deleting_destructor_` (Xbox PDB): `008411f0`, then frees
/// `this` with bit 0 of `flags`. Answers `this`.
pub fn bgs_reconstruct_forms_map_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    fn_008411f0(e, this);
    free_if(e, this, flags)
}

// Translated from 008411f0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 008411f0 - the exception-unwinding frame (FS:[0]) is not translated.
/// The `BGSReconstructFormsMap` destructor (unnamed in the engine map; the map gives the body
/// several library names, a folded body): its vtable, the map emptied (`00438af0`), then the
/// `NiTMap<unsigned int, ReconstructFormData>` destructor `00844470`.
pub fn fn_008411f0(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), FORMS_MAP_VTABLE);
    e.call(MAP_REMOVE_ALL, &args![this]);
    e.call(FORM_DATA_NITMAP_DESTRUCT, &args![this]);
}

// Translated from 00841250 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSReconstructFormsMap::AddForm` (Xbox PDB; `thiscall`: a form, a form id used when the
/// form is null, a word; `RET 0xC`). The key is the form's id (`0084e3a0`) or `form_id`; the
/// value, made by `004b0680(form, data)` in a local (after `0096a2d0` on another 8-byte
/// local), is set at it (`00844310`). Then flags (`00841320`): 6 for a null form; for a form
/// whose vtable +0xF0 answers true, 2 when `00565450` answers true, else 4; otherwise 4 when
/// the form's type is 0x43.
pub fn bgs_reconstruct_forms_map_add_form(
    e: &mut Engine,
    this: Ptr,
    form: u32,
    form_id: u32,
    data: u32,
) {
    let key = if form != 0 {
        e.call(FORM_ID, &args![form]).u32()
    } else {
        form_id
    };
    // The exe's locals from EBP-0x20: the form data at EBP-0x18 (+8), the other local at
    // EBP-8 (+0x18).
    e.with_stack(0x20, |e, locals| {
        e.call(PAIR_CONSTRUCT, &args![locals.byte_add(0x18)]);
        let made = e
            .call(FORM_DATA_MAKE, &args![locals.byte_add(8), form, data])
            .u32();
        let first = e.mem.u32(made);
        let second = e.mem.u32(made + 4);
        e.call(FORM_DATA_SET_AT, &args![this, key, first, second]);
    });
    if form == 0 {
        fn_00841320(e, this, 6);
    } else if e.vcall(form, 0xf0, &args![]).bool() {
        if e.call(FORM_TEST_00565450, &args![form]).bool() {
            fn_00841320(e, this, 2);
        } else {
            fn_00841320(e, this, 4);
        }
    } else if e.call(FORM_TYPE, &args![form]).u32() == 0x43 {
        fn_00841320(e, this, 4);
    }
}

// Translated from 00841320 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets `bits` in the `BGSReconstructFormsMap`'s flags (+0x10); `RET 4`. The engine map's name
/// (`NiTMapBase<..ReconstructFormData>::GetAt`) is a folded one.
pub fn fn_00841320(e: &mut Engine, this: Ptr, bits: u32) {
    let flags = e.mem.u32(this.addr() + 0x10);
    e.mem.set_u32(this.addr() + 0x10, flags | bits);
}

// Translated from 00841340 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `BGSReconstructCellsMap` constructor (unnamed in the engine map; the vtable's RTTI names
/// the class): `00844070` with 0x25 buckets, then the vtable. Answers `this`.
pub fn fn_00841340(e: &mut Engine, this: Ptr) -> Ptr {
    fn_00844070(e, this, BUCKET_COUNT);
    e.mem.set_u32(this.addr(), CELLS_MAP_VTABLE);
    this
}

// Translated from 00841370 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSReconstructCellsMap::_scalar_deleting_destructor_` (Xbox PDB): `008413a0`, then frees
/// `this` with bit 0 of `flags`. Answers `this`.
pub fn bgs_reconstruct_cells_map_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    fn_008413a0(e, this);
    free_if(e, this, flags)
}

// Translated from 008413a0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 008413a0 - the exception-unwinding frame (FS:[0]) is not translated.
/// The `BGSReconstructCellsMap` destructor (unnamed in the engine map): its vtable, every
/// `BGSReconstructFormsMap` value deleted (vtable slot 0 with 1), then the `NiTMap<unsigned int,
/// BGSReconstructFormsMap *>` destructor `00844600`.
pub fn fn_008413a0(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), CELLS_MAP_VTABLE);
    delete_values(e, this);
    e.call(FORMS_MAP_NITMAP_DESTRUCT, &args![this]);
}

// Translated from 00841460 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00841460 - the compiler's exception frame (FS:[0] chain, state 0 that frees the new map's block when its constructor throws) is not translated.
/// A `BGSReconstructCellsMap` add (unnamed in the engine map; `thiscall`: a cell, a form, a
/// form id, a word; `RET 0x10`). The key is the cell's id (`0084e3a0`) when `00425fd0`
/// answers true, else `00587410(GetDataX, GetDataY)` (`GetDataY` runs first); its
/// `BGSReconstructFormsMap` is found or made (0x14 bytes, `00841190`). When the form is the
/// cell itself the map's flags get 1 and `data`; otherwise the form is added
/// (`00841250(form, form_id, data)`).
pub fn fn_00841460(e: &mut Engine, this: Ptr, cell: u32, form: u32, form_id: u32, data: u32) {
    let key = if e.call(FORM_TEST_00425FD0, &args![cell]).bool() {
        e.call(FORM_ID, &args![cell]).u32()
    } else {
        let y = e.call(CELL_GET_DATA_Y, &args![cell]).u32();
        let x = e.call(CELL_GET_DATA_X, &args![cell]).u32();
        e.call(CELL_COORDINATE_KEY, &args![x, y]).u32()
    };
    let map = value_or_new(e, this.addr(), key, FORMS_MAP_SIZE, make_forms_map);
    if cell == form {
        fn_00841320(e, Ptr::new(map), 1);
        fn_00841320(e, Ptr::new(map), data);
    } else {
        bgs_reconstruct_forms_map_add_form(e, Ptr::new(map), form, form_id, data);
    }
}

// Translated from 00841580 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00841580 - the compiler's exception frame (FS:[0] chain, state 0 that frees the new map's block when its constructor throws) is not translated.
/// A `BGSReconstructCellsMap` add by coordinates (unnamed in the engine map; `thiscall`: a word
/// it does not read, x and y (their low 16 bits), a form, a form id, a word; `RET 0x18`): the
/// key `00587410(x, y)`, its `BGSReconstructFormsMap` found or made (0x14 bytes, `00841190`),
/// and the form added (`00841250`).
#[allow(clippy::too_many_arguments)]
pub fn fn_00841580(
    e: &mut Engine,
    this: Ptr,
    _unused_1: u32,
    x: u32,
    y: u32,
    form: u32,
    form_id: u32,
    data: u32,
) {
    let key = e
        .call(CELL_COORDINATE_KEY, &args![x & 0xffff, y & 0xffff])
        .u32();
    let map = value_or_new(e, this.addr(), key, FORMS_MAP_SIZE, make_forms_map);
    bgs_reconstruct_forms_map_add_form(e, Ptr::new(map), form, form_id, data);
}

// Translated from 00841660 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether any `BGSReconstructFormsMap` of the `BGSReconstructCellsMap` answers true to
/// `00452420(mask)` (unnamed in the engine map; `RET 4`).
pub fn fn_00841660(e: &mut Engine, this: Ptr, mask: u32) -> bool {
    any_value(e, this, |e, map| {
        e.call(FORMS_MAP_TEST_FLAGS, &args![map, mask]).bool()
    })
}

// Translated from 008416c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `BGSReconstructCellSubBlocksMap` constructor (the vtable's RTTI names the class; the
/// engine map's `NiTMapBase<..BGSReconstructCellsMap *>` name is a folded one): `008440a0`
/// with 0x25 buckets, then the vtable. Answers `this`.
pub fn fn_008416c0(e: &mut Engine, this: Ptr) -> Ptr {
    fn_008440a0(e, this, BUCKET_COUNT);
    e.mem.set_u32(this.addr(), SUB_BLOCKS_MAP_VTABLE);
    this
}

// Translated from 008416f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSReconstructCellSubBlocksMap::_scalar_deleting_destructor_` (Xbox PDB): `00841720`,
/// then frees `this` with bit 0 of `flags`. Answers `this`.
pub fn bgs_reconstruct_cell_sub_blocks_map_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    fn_00841720(e, this);
    free_if(e, this, flags)
}

// Translated from 00841720 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00841720 - the exception-unwinding frame (FS:[0]) is not translated.
/// The `BGSReconstructCellSubBlocksMap` destructor (unnamed in the engine map): its vtable,
/// every `BGSReconstructCellsMap` value deleted, then the `NiTMap` destructor `008447e0`.
pub fn fn_00841720(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), SUB_BLOCKS_MAP_VTABLE);
    delete_values(e, this);
    fn_008447e0(e, this);
}

// Translated from 008417e0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 008417e0 - the compiler's exception frame (FS:[0] chain, state 0 that frees the new map's block when its constructor throws) is not translated.
/// A `BGSReconstructCellSubBlocksMap` add (unnamed in the engine map; `thiscall`: a cell, a
/// form, a form id, a word; `RET 0x10`): the cell's sub-block key (`00544210`), its
/// `BGSReconstructCellsMap` found or made (0x10 bytes, `00841340`), then `00841460` with the
/// same words.
pub fn fn_008417e0(e: &mut Engine, this: Ptr, cell: u32, form: u32, form_id: u32, data: u32) {
    let key = e.call(CELL_SUB_BLOCK_KEY, &args![cell]).u32();
    let map = value_or_new(e, this.addr(), key, CELLS_MAP_SIZE, make_cells_map);
    fn_00841460(e, Ptr::new(map), cell, form, form_id, data);
}

// Translated from 008418b0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 008418b0 - the compiler's exception frame (FS:[0] chain, state 0 that frees the new map's block when its constructor throws) is not translated.
/// A `BGSReconstructCellSubBlocksMap` add by coordinates (unnamed in the engine map;
/// `thiscall`: a world space, x, y, a form, a form id, a word; `RET 0x18`): the key
/// `TESObjectCELL::CalcExtGroupSubBlockKey(x, y)`, its `BGSReconstructCellsMap` found or made
/// (0x10 bytes, `00841340`), then `00841580` with the same words.
#[allow(clippy::too_many_arguments)]
pub fn fn_008418b0(
    e: &mut Engine,
    this: Ptr,
    world: u32,
    x: u32,
    y: u32,
    form: u32,
    form_id: u32,
    data: u32,
) {
    let key = e.call(EXTERIOR_SUB_BLOCK_KEY, &args![x, y]).u32();
    let map = value_or_new(e, this.addr(), key, CELLS_MAP_SIZE, make_cells_map);
    fn_00841580(e, Ptr::new(map), world, x, y, form, form_id, data);
}

// Translated from 00841990 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether any `BGSReconstructCellsMap` of the `BGSReconstructCellSubBlocksMap` answers true
/// to `00841660(mask)` (unnamed in the engine map; `RET 4`).
pub fn fn_00841990(e: &mut Engine, this: Ptr, mask: u32) -> bool {
    any_value(e, this, |e, map| fn_00841660(e, Ptr::new(map), mask))
}

// Translated from 008419f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSReconstructCellBlocksMap::BGSReconstructCellBlocksMap` (Xbox PDB; `RET 4`): `008440d0`
/// with 0x25 buckets, the vtable, 0 at +0x10 and the byte at +0x14 (1 from the world-space
/// maps, 0 from `BGSReconstructFormsInFileMap::AddCellForm`). Answers `this`.
pub fn bgs_reconstruct_cell_blocks_map_constructor(e: &mut Engine, this: Ptr, exterior: u8) -> Ptr {
    fn_008440d0(e, this, BUCKET_COUNT);
    e.mem.set_u32(this.addr(), CELL_BLOCKS_MAP_VTABLE);
    e.mem.set_u32(this.addr() + 0x10, 0);
    e.mem.set_u8(this.addr() + 0x14, exterior);
    this
}

// Translated from 00841a30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSReconstructCellBlocksMap::_scalar_deleting_destructor_` (Xbox PDB): `00841a60`, then
/// frees `this` with bit 0 of `flags`. Answers `this`.
pub fn bgs_reconstruct_cell_blocks_map_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    fn_00841a60(e, this);
    free_if(e, this, flags)
}

// Translated from 00841a60 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00841a60 - the exception-unwinding frame (FS:[0]) is not translated.
/// The `BGSReconstructCellBlocksMap` destructor (unnamed in the engine map): its vtable, every
/// `BGSReconstructCellSubBlocksMap` value deleted, then the `NiTMap` destructor `008448e0`.
/// The map at +0x10 is not deleted here.
pub fn fn_00841a60(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), CELL_BLOCKS_MAP_VTABLE);
    delete_values(e, this);
    fn_008448e0(e, this);
}

// Translated from 00841b20 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00841b20 - the compiler's exception frame (FS:[0] chain, states 0 and 1 that free a new map's block when its constructor throws) is not translated.
/// `BGSReconstructCellBlocksMap::AddCellForm` (Xbox PDB; `thiscall`: a cell, a form, a form
/// id, a word; `RET 0x10`). With the byte at +0x14 set and a form whose vtable +0xF0 and
/// `00565450` answer true, the form goes to the map at +0x10 (made first, 0x14 bytes,
/// `00841190`) through `00841250`. Otherwise the cell's block key (`005441b0`) finds or makes
/// its `BGSReconstructCellSubBlocksMap` (0x10 bytes, `008416c0`), and `008417e0` gets the same
/// words.
pub fn bgs_reconstruct_cell_blocks_map_add_cell_form(
    e: &mut Engine,
    this: Ptr,
    cell: u32,
    form: u32,
    form_id: u32,
    data: u32,
) {
    let t = this.addr();
    if e.mem.u8(t + 0x14) != 0
        && form != 0
        && e.vcall(form, 0xf0, &args![]).bool()
        && e.call(FORM_TEST_00565450, &args![form]).bool()
    {
        let map = member_or_new(e, t, 0x10, FORMS_MAP_SIZE, make_forms_map);
        bgs_reconstruct_forms_map_add_form(e, Ptr::new(map), form, form_id, data);
        return;
    }
    let key = e.call(CELL_BLOCK_KEY, &args![cell]).u32();
    let map = value_or_new(e, t, key, CELLS_MAP_SIZE, make_sub_blocks_map);
    fn_008417e0(e, Ptr::new(map), cell, form, form_id, data);
}

// Translated from 00841ca0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00841ca0 - the compiler's exception frame (FS:[0] chain, states 0 and 1 that free a new map's block when its constructor throws) is not translated.
/// `BGSReconstructCellBlocksMap::AddCellForm_ov2` (Xbox PDB; `thiscall`: a world space, x, y,
/// a form, a form id, a word; `RET 0x18`). A form whose vtable +0xF0 and `00565450` answer
/// true goes to the map at +0x10 (made first) through `00841250` (the byte at +0x14 is not
/// read here). Otherwise `TESObjectCELL::CalcExtGroupBlockKey(x, y)` finds or makes the
/// `BGSReconstructCellSubBlocksMap` (0x10 bytes, `008416c0`), and `008418b0` gets the same
/// words.
#[allow(clippy::too_many_arguments)]
pub fn bgs_reconstruct_cell_blocks_map_add_cell_form_ov2(
    e: &mut Engine,
    this: Ptr,
    world: u32,
    x: u32,
    y: u32,
    form: u32,
    form_id: u32,
    data: u32,
) {
    let t = this.addr();
    if form != 0
        && e.vcall(form, 0xf0, &args![]).bool()
        && e.call(FORM_TEST_00565450, &args![form]).bool()
    {
        let map = member_or_new(e, t, 0x10, FORMS_MAP_SIZE, make_forms_map);
        bgs_reconstruct_forms_map_add_form(e, Ptr::new(map), form, form_id, data);
        return;
    }
    let key = e.call(EXTERIOR_BLOCK_KEY, &args![x, y]).u32();
    let map = value_or_new(e, t, key, CELLS_MAP_SIZE, make_sub_blocks_map);
    fn_008418b0(e, Ptr::new(map), world, x, y, form, form_id, data);
}

// Translated from 00841e20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether the `BGSReconstructCellBlocksMap` holds forms answering true to `mask`: its map at
/// +0x10 (`00452420`, when set), else any `BGSReconstructCellSubBlocksMap` value (`00841990`)
/// (unnamed in the engine map; `RET 4`).
pub fn fn_00841e20(e: &mut Engine, this: Ptr, mask: u32) -> bool {
    let own = e.mem.u32(this.addr() + 0x10);
    if own != 0 && e.call(FORMS_MAP_TEST_FLAGS, &args![own, mask]).bool() {
        return true;
    }
    any_value(e, this, |e, map| fn_00841990(e, Ptr::new(map), mask))
}

// Translated from 00841ea0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `BGSReconstructWorldSpacesMap` constructor (the vtable's RTTI names the class; the
/// engine map's `NiTMapBase<..BGSReconstructCellBlocksMap *>` name is a folded one):
/// `00844100` with 0x25 buckets, then the vtable. Answers `this`.
pub fn fn_00841ea0(e: &mut Engine, this: Ptr) -> Ptr {
    fn_00844100(e, this, BUCKET_COUNT);
    e.mem.set_u32(this.addr(), WORLD_SPACES_MAP_VTABLE);
    this
}

// Translated from 00841ed0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSReconstructWorldSpacesMap::_scalar_deleting_destructor_` (Xbox PDB): `00841f00`, then
/// frees `this` with bit 0 of `flags`. Answers `this`.
pub fn bgs_reconstruct_world_spaces_map_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    fn_00841f00(e, this);
    free_if(e, this, flags)
}

// Translated from 00841f00 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00841f00 - the exception-unwinding frame (FS:[0]) is not translated.
/// The `BGSReconstructWorldSpacesMap` destructor (unnamed in the engine map): its vtable,
/// every `BGSReconstructCellBlocksMap` value deleted, then the `NiTMap` destructor `008449e0`.
pub fn fn_00841f00(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), WORLD_SPACES_MAP_VTABLE);
    delete_values(e, this);
    fn_008449e0(e, this);
}

// Translated from 00841fc0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00841fc0 - the compiler's exception frame (FS:[0] chain, state 0 that frees the new map's block when its constructor throws) is not translated.
/// `BGSReconstructWorldSpacesMap::AddCellForm` (Xbox PDB; `thiscall`: a cell, a form, a form
/// id, a word; `RET 0x10`): the key is the id (`0084e3a0`) of the cell's world space
/// (`TESObjectCELL::GetWorldSpace`); its `BGSReconstructCellBlocksMap` is found or made (0x18
/// bytes, `008419f0(1)`), then `00841b20` gets the same words.
pub fn bgs_reconstruct_world_spaces_map_add_cell_form(
    e: &mut Engine,
    this: Ptr,
    cell: u32,
    form: u32,
    form_id: u32,
    data: u32,
) {
    let world = e.call(CELL_GET_WORLD_SPACE, &args![cell]).u32();
    let key = e.call(FORM_ID, &args![world]).u32();
    let map = value_or_new(
        e,
        this.addr(),
        key,
        CELL_BLOCKS_MAP_SIZE,
        make_exterior_blocks_map,
    );
    bgs_reconstruct_cell_blocks_map_add_cell_form(e, Ptr::new(map), cell, form, form_id, data);
}

// Translated from 008420a0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 008420a0 - the compiler's exception frame (FS:[0] chain, state 0 that frees the new map's block when its constructor throws) is not translated.
/// `BGSReconstructWorldSpacesMap::AddCellForm_ov2` (Xbox PDB; `thiscall`: a world space, x,
/// y, a form, a form id, a word; `RET 0x18`): the key is the world space's id (`0084e3a0`);
/// its `BGSReconstructCellBlocksMap` is found or made (0x18 bytes, `008419f0(1)`), then
/// `00841ca0` gets the same words.
#[allow(clippy::too_many_arguments)]
pub fn bgs_reconstruct_world_spaces_map_add_cell_form_ov2(
    e: &mut Engine,
    this: Ptr,
    world: u32,
    x: u32,
    y: u32,
    form: u32,
    form_id: u32,
    data: u32,
) {
    let key = e.call(FORM_ID, &args![world]).u32();
    let map = value_or_new(
        e,
        this.addr(),
        key,
        CELL_BLOCKS_MAP_SIZE,
        make_exterior_blocks_map,
    );
    bgs_reconstruct_cell_blocks_map_add_cell_form_ov2(
        e,
        Ptr::new(map),
        world,
        x,
        y,
        form,
        form_id,
        data,
    );
}

// Translated from 00842180 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSReconstructFormsInFileMap::BGSReconstructFormsInFileMap` (Xbox PDB): `00844070` with
/// 0x25 buckets, the vtable, and 0 in the interior blocks map (+0x10), the world-spaces map
/// (+0x14) and the count (+0x18). Answers `this`.
pub fn bgs_reconstruct_forms_in_file_map_constructor(e: &mut Engine, this: Ptr) -> Ptr {
    fn_00844070(e, this, BUCKET_COUNT);
    let t = this.addr();
    e.mem.set_u32(t, FORMS_IN_FILE_MAP_VTABLE);
    e.mem.set_u32(t + 0x10, 0);
    e.mem.set_u32(t + 0x14, 0);
    e.mem.set_u32(t + 0x18, 0);
    this
}

// Translated from 008421c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSReconstructFormsInFileMap::_scalar_deleting_destructor_` (Xbox PDB): `008421f0`, then
/// frees `this` with bit 0 of `flags`. Answers `this`.
pub fn bgs_reconstruct_forms_in_file_map_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    fn_008421f0(e, this);
    free_if(e, this, flags)
}

// Translated from 008421f0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 008421f0 - the exception-unwinding frame (FS:[0]) is not translated.
/// The `BGSReconstructFormsInFileMap` destructor (unnamed in the engine map): its vtable,
/// every `BGSReconstructFormsMap` value deleted, the maps at +0x10 and +0x14 deleted (vtable
/// slot 0 with 1, when set), then the `NiTMap<unsigned int, BGSReconstructFormsMap *>`
/// destructor `00844600`.
pub fn fn_008421f0(e: &mut Engine, this: Ptr) {
    let t = this.addr();
    e.mem.set_u32(t, FORMS_IN_FILE_MAP_VTABLE);
    delete_values(e, this);
    let interior = e.mem.u32(t + 0x10);
    if interior != 0 {
        e.vcall(interior, 0, &args![1u32]);
    }
    let worlds = e.mem.u32(t + 0x14);
    if worlds != 0 {
        e.vcall(worlds, 0, &args![1u32]);
    }
    e.call(FORMS_MAP_NITMAP_DESTRUCT, &args![this]);
}

/// `ScrapHeap::Allocate` (engine map, `00aa54a0`; `thiscall`: byte count, alignment) and
/// `ScrapHeap::Deallocate` (`00aa5610`; `thiscall`: the block).
const SCRAP_HEAP_ALLOCATE: u32 = 0x00aa_54a0;
const SCRAP_HEAP_DEALLOCATE: u32 = 0x00aa_5610;
/// The alignment word the scrap arrays pass to `ScrapHeap::Allocate` (read at `010a2720`).
const SCRAP_ALIGNMENT: u32 = 0x010a_2720;
/// `00401460` (`cdecl`: destination, source, byte count): the copy `_Reallocate` makes.
const MEMORY_COPY_00401460: u32 = 0x0040_1460;
/// `006815c0` (`thiscall`): answers `this` (here the constructor of a 12-byte local).
const RETURNS_THIS_006815C0: u32 = 0x0068_15c0;
/// `GetLocationCellOrWorld` (engine map, `00846190`; `cdecl`: a word, the cell's and the
/// world space's addresses).
const GET_LOCATION_CELL_OR_WORLD: u32 = 0x0084_6190;
/// `TESObjectREFR::GetWorldSpace` (engine map, `00575d70`) and `008d6f30` (`thiscall`, the
/// reference's other form: the parent cell the save code saves when there is no world space).
const REFR_GET_WORLD_SPACE: u32 = 0x0057_5d70;
const REFR_PARENT_CELL_008D6F30: u32 = 0x008d_6f30;
/// `00406d90` (`cdecl`, a `float`): the integer the cell coordinate is made from (`SAR 12`).
const FLOAT_TO_INT_00406D90: u32 = 0x0040_6d90;
/// `0059bb30` (`thiscall` on a form; the engine map's `D3DTexture_LockRect` is a folded name):
/// the form's cell in `AddForm`'s type-0x43 branch.
const FORM_CELL_0059BB30: u32 = 0x0059_bb30;
/// The record tags (the first word of each 12-byte entry `{name, tag, type}` of the table at
/// `01187004`, filled at run time; the names are the table's): `TES4`, `GRUP`, `CELL`, `WRLD`,
/// `TOFT`. A form type's tag is the word at `01187008 + 0xC * type`.
const TAG_TES4: u32 = 0x0118_7014;
const TAG_GRUP: u32 = 0x0118_7020;
const TAG_CELL: u32 = 0x0118_72b4;
const TAG_WRLD: u32 = 0x0118_7314;
const TAG_TOFT: u32 = 0x0118_73c8;
const TAG_BY_FORM_TYPE: u32 = 0x0118_7008;
/// The cell `ReconstructForms` keeps loaded between records (`011c3f30`) and the
/// `TESDataHandler` singleton pointer (`011c3f2c`).
const RECONSTRUCT_CELL: u32 = 0x011c_3f30;
const DATA_HANDLER: u32 = 0x011c_3f2c;
/// The `TESFile` functions `ReconstructForms` runs (`thiscall` on the file):
/// `TESFile::TESRewind` (`00471f80`, a word), the current record header (`00462270`),
/// `TESFile::NextForm` (`00472150`, a word), `TESFile::GetOptimizedFile` (`00471ca0`, a byte),
/// the group skip (`00473660`), `TESFile::GetTESChunk` (`004726b0`),
/// `TESFile::GetChunkData_ov3` (`00472890`: buffer, size), the next chunk (`004726f0`, a
/// `bool`), `00401680` (a `bool`), the record's end (`00472000`), the position (`00467bb0`) and
/// `TESFile::SetOffset` (`004723a0`, the offset).
const FILE_REWIND: u32 = 0x0047_1f80;
const FILE_CURRENT_FORM: u32 = 0x0046_2270;
const FILE_NEXT_FORM: u32 = 0x0047_2150;
const FILE_GET_OPTIMIZED: u32 = 0x0047_1ca0;
const FILE_SKIP_GROUP: u32 = 0x0047_3660;
const FILE_GET_CHUNK: u32 = 0x0047_26b0;
const FILE_GET_CHUNK_DATA: u32 = 0x0047_2890;
const FILE_NEXT_CHUNK: u32 = 0x0047_26f0;
const FILE_TEST_00401680: u32 = 0x0040_1680;
const FILE_END_FORM: u32 = 0x0047_2000;
const FILE_POSITION: u32 = 0x0046_7bb0;
const FILE_SET_OFFSET: u32 = 0x0047_23a0;
/// `"XCLC"` as a little-endian word: the chunk whose 12 bytes give a cell's coordinates.
const CHUNK_XCLC: u32 = 0x434c_4358;
/// `00540720` (`thiscall`, the 12-byte chunk buffer's constructor) and `00462230`
/// (`thiscall` on it, run when `00401680` answers true).
const CHUNK_BUFFER_CONSTRUCT: u32 = 0x0054_0720;
const CHUNK_BUFFER_00462230: u32 = 0x0046_2230;
/// `TESForm::AddCompileIndex` (engine map, `00485d50`; `cdecl`: the id's address, the file).
const ADD_COMPILE_INDEX: u32 = 0x0048_5d50;
/// `00467bd0` (`thiscall` on the data handler: the file, three words; `RET 0x10`): loads the
/// current record.
const HANDLER_LOAD_RECORD: u32 = 0x0046_7bd0;
/// `004839c0` (`cdecl`, a form id): the loaded form with that id, or 0.
const FORM_BY_ID: u32 = 0x0048_39c0;
/// `0044edb0` (`thiscall`, a word getter; the engine map's
/// `BaseProcess::GetCurrentProcedureIndex` is a folded name).
const WORD_GETTER_0044EDB0: u32 = 0x0044_edb0;
/// `TESForm::GetFormTypeFromFormString` (engine map, `00486890`; `cdecl`, a tag) and
/// `004612b0` (`cdecl`, a form type): the type's name.
const FORM_TYPE_FROM_TAG: u32 = 0x0048_6890;
const FORM_TYPE_NAME: u32 = 0x0046_12b0;
/// `"Cell with Form ID %08X was not able to be reconstructed"` and `"Form %08X with type %s
/// was not able to be reconstructed"`.
const CELL_NOT_RECONSTRUCTED: u32 = 0x0107_ea24;
const FORM_NOT_RECONSTRUCTED: u32 = 0x0107_e9ec;
/// `005ca4f0` (`thiscall` on the offset pair, a `bool`): whether it is set.
const PAIR_IS_SET: u32 = 0x005c_a4f0;

// Translated from 00842310 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00842310 - the compiler's exception frame (FS:[0] chain, state 0 that frees the new map's block when its constructor throws) is not translated.
/// `BGSReconstructFormsInFileMap::AddForm` (Xbox PDB; `thiscall`: a form, a word; `RET 8`).
/// The form's type is read first (vtable +0x8C). A form whose vtable +0xF0 answers true is
/// placed by its location: vtable +0x138 (two 12-byte locals made by `006815c0`, a word, 0);
/// when it answers true `GetLocationCellOrWorld` gives the cell and the world space from the
/// word, else the world space is `TESObjectREFR::GetWorldSpace` (with the 12 bytes of vtable
/// +0x1F4 as the position) or, without one, the cell is `008d6f30`'s. With a world space it
/// goes to `00842730(world, x, y, form, 0, data)` (x and y from `00406d90` of the position's
/// first two floats, shifted right by 12), else with a cell to `00842600(cell, form, 0,
/// data)`. Otherwise by type: 0x43 through its cell (`0059bb30`) and that cell's world space
/// (with `GetDataX`, `GetDataY`), 0x39 as its own cell, 0x41 not at all, any other type into
/// the `BGSReconstructFormsMap` of its type's tag (found or made) with `00841250(form, 0,
/// data)`, counting +0x18 up.
pub fn bgs_reconstruct_forms_in_file_map_add_form(e: &mut Engine, this: Ptr, form: u32, data: u32) {
    let kind = e.vcall(form, 0x8c, &args![]).u32();
    if e.vcall(form, 0xf0, &args![]).bool() {
        // The exe's locals from EBP-0x68: the position at EBP-0x34, the other 12 bytes at
        // EBP-0x28, the word at EBP-0x14, the cell at EBP-0x38, the world space at EBP-0x18.
        e.with_stack(0x5c, |e, frame| {
            let ebp = frame.addr() + 0x68;
            let position = ebp - 0x34;
            let other = ebp - 0x28;
            let word_at = ebp - 0x14;
            let cell_at = ebp - 0x38;
            let world_at = ebp - 0x18;
            e.call(RETURNS_THIS_006815C0, &args![position]);
            e.call(RETURNS_THIS_006815C0, &args![other]);
            e.mem.set_u32(word_at, 0);
            e.mem.set_u32(cell_at, 0);
            e.mem.set_u32(world_at, 0);
            if e.vcall(form, 0x138, &args![position, other, word_at, 0u32])
                .bool()
            {
                let word = e.mem.u32(word_at);
                e.call(GET_LOCATION_CELL_OR_WORLD, &args![word, cell_at, world_at]);
            } else {
                let world = e.call(REFR_GET_WORLD_SPACE, &args![form]).u32();
                e.mem.set_u32(world_at, world);
                if world != 0 {
                    let source = e.vcall(form, 0x1f4, &args![]).u32();
                    for i in 0..3 {
                        let word = e.mem.u32(source + 4 * i);
                        e.mem.set_u32(position + 4 * i, word);
                    }
                } else {
                    let cell = e.call(REFR_PARENT_CELL_008D6F30, &args![form]).u32();
                    e.mem.set_u32(cell_at, cell);
                }
            }
            let world = e.mem.u32(world_at);
            if world != 0 {
                let x_bits = x87_quiet_bits(e.mem.u32(position));
                let x = (e.call(FLOAT_TO_INT_00406D90, &args![x_bits]).i32() >> 12) as u32;
                let y_bits = x87_quiet_bits(e.mem.u32(position + 4));
                let y = (e.call(FLOAT_TO_INT_00406D90, &args![y_bits]).i32() >> 12) as u32;
                bgs_reconstruct_forms_in_file_map_add_cell_form_ov2(
                    e, this, world, x, y, form, 0, data,
                );
            } else {
                let cell = e.mem.u32(cell_at);
                if cell != 0 {
                    bgs_reconstruct_forms_in_file_map_add_cell_form(e, this, cell, form, 0, data);
                }
            }
        });
        return;
    }
    if kind == 0x43 {
        let cell = e.call(FORM_CELL_0059BB30, &args![form]).u32();
        let world = if cell != 0 {
            e.call(CELL_GET_WORLD_SPACE, &args![cell]).u32()
        } else {
            0
        };
        if world != 0 {
            let y = e.call(CELL_GET_DATA_Y, &args![cell]).u32();
            let x = e.call(CELL_GET_DATA_X, &args![cell]).u32();
            bgs_reconstruct_forms_in_file_map_add_cell_form_ov2(
                e, this, world, x, y, form, 0, data,
            );
        } else if cell != 0 {
            bgs_reconstruct_forms_in_file_map_add_cell_form(e, this, cell, form, 0, data);
        }
    } else if kind == 0x39 {
        bgs_reconstruct_forms_in_file_map_add_cell_form(e, this, form, form, 0, data);
    } else if kind != 0x41 {
        let kind = e.vcall(form, 0x8c, &args![]).u32();
        let tag = e.global::<u32>(TAG_BY_FORM_TYPE.wrapping_add(kind.wrapping_mul(0xc)));
        let map = value_or_new(e, this.addr(), tag, FORMS_MAP_SIZE, make_forms_map);
        bgs_reconstruct_forms_map_add_form(e, Ptr::new(map), form, 0, data);
        let count = e.mem.u32(this.addr() + 0x18);
        e.mem.set_u32(this.addr() + 0x18, count.wrapping_add(1));
    }
}

// Translated from 00842600 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00842600 - the compiler's exception frame (FS:[0] chain, states 0 and 1 that free a new map's block when its constructor throws) is not translated.
/// `BGSReconstructFormsInFileMap::AddCellForm` (Xbox PDB; `thiscall`: a cell, a form, a form
/// id, a word; `RET 0x10`): when the cell's `00425fd0` answers true, into the interior
/// `BGSReconstructCellBlocksMap` at +0x10 (made first, 0x18 bytes, `008419f0(0)`) with
/// `00841b20`; else into the `BGSReconstructWorldSpacesMap` at +0x14 (made first, 0x10 bytes,
/// `00841ea0`) with `00841fc0`. The count at +0x18 goes up.
pub fn bgs_reconstruct_forms_in_file_map_add_cell_form(
    e: &mut Engine,
    this: Ptr,
    cell: u32,
    form: u32,
    form_id: u32,
    data: u32,
) {
    let t = this.addr();
    if e.call(FORM_TEST_00425FD0, &args![cell]).bool() {
        let map = member_or_new(e, t, 0x10, CELL_BLOCKS_MAP_SIZE, make_interior_blocks_map);
        bgs_reconstruct_cell_blocks_map_add_cell_form(e, Ptr::new(map), cell, form, form_id, data);
    } else {
        let map = member_or_new(e, t, 0x14, WORLD_SPACES_MAP_SIZE, make_world_spaces_map);
        bgs_reconstruct_world_spaces_map_add_cell_form(e, Ptr::new(map), cell, form, form_id, data);
    }
    let count = e.mem.u32(t + 0x18);
    e.mem.set_u32(t + 0x18, count.wrapping_add(1));
}

// Translated from 00842730 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00842730 - the compiler's exception frame (FS:[0] chain, state 0 that frees the new map's block when its constructor throws) is not translated.
/// `BGSReconstructFormsInFileMap::AddCellForm_ov2` (Xbox PDB; `thiscall`: a world space, x,
/// y, a form, a form id, a word; `RET 0x18`): into the `BGSReconstructWorldSpacesMap` at +0x14
/// (made first, 0x10 bytes, `00841ea0`) with `008420a0`; the count at +0x18 goes up.
#[allow(clippy::too_many_arguments)]
pub fn bgs_reconstruct_forms_in_file_map_add_cell_form_ov2(
    e: &mut Engine,
    this: Ptr,
    world: u32,
    x: u32,
    y: u32,
    form: u32,
    form_id: u32,
    data: u32,
) {
    let t = this.addr();
    let map = member_or_new(e, t, 0x14, WORLD_SPACES_MAP_SIZE, make_world_spaces_map);
    bgs_reconstruct_world_spaces_map_add_cell_form_ov2(
        e,
        Ptr::new(map),
        world,
        x,
        y,
        form,
        form_id,
        data,
    );
    let count = e.mem.u32(t + 0x18);
    e.mem.set_u32(t + 0x18, count.wrapping_add(1));
}

// Translated from 008427f0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 008427f0 - the exception-unwinding frame (FS:[0]) that destroys the offset array is not translated.
/// `BGSReconstructFormsInFileMap::ReconstructForms` (Xbox PDB; `thiscall`: the file, the
/// arrays of `BGSReconstructFormsInAllFilesMap`; `RET 8`). Nothing when the count (+0x18) is
/// 0. Otherwise the file is rewound (`TESRewind(1)`) and its `TES4` records skipped; then,
/// while the current record's tag is not 0 and fewer than the count of forms were
/// reconstructed, the record stream is walked with four maps in locals (the cell blocks map
/// `a`, the sub-blocks map `b`, the cells map `c`, the forms map `d`), the current world space
/// and cell ids, and a `BGSOffsetDataArray` (`00843190`) of group offsets:
/// - a `GRUP` labelled `CELL` or `WRLD` enters the interior blocks map (+0x10) or, for
///   `WRLD`, checks the world-spaces map (+0x14) (skipping the group, `00473660`, when the map
///   is missing); a group of type 1 looks up its world space (its compile index added) in
///   +0x14, and in an optimized file a world space holding forms of mask 5 (`00841e20`) has
///   the group's offset (+0x10) added; types 2/4 and 3/5 look up the next map, type 6 tests
///   the forms map's masks 2 and 4 (adding the offset once for mask 4 in an optimized file
///   outside a world space), 8 mask 2, 9 mask 4; type 0 finds the forms map of its label in
///   `this`; other types and every miss skip the group;
/// - `WRLD` keeps its id; `CELL` finds its forms map in `c` (by its `XCLC` coordinates,
///   `00587410`, when `a`'s byte +0x14 is set), loads it (`00467bd0(file, 0, 0, 0)`) when it
///   holds mask 1 and adds the cell with `0044edb0`'s word to the arrays (`00843270`) or warns;
///   without `c` the forms map is `0044edb0` of `a`;
/// - `TOFT` with -1 or -2 takes the next stored offset (or ends the walk);
/// - any other record found in `d` (after `00841320(0x10000)`) is loaded
///   (`00467bd0(file, 0, form, 1)`, with the current cell made current at `011c3f30`) and its
///   form added to the arrays, or a warning names it.
/// When a group's end is reached (`00467bb0`) the next stored offset is taken (or the walk
/// ends); a set offset pair (`005ca4f0`) moves the file there (`TESFile::SetOffset`) and
/// restarts the maps from the group found there (a world space's group looks its world space
/// up again, a cell's group takes the pair's forms map), then the pair is cleared
/// (`00843170`) and a world space's type-9 group skipped. At the end `011c3f30` is cleared
/// and the offset array destroyed (`00843220`).
pub fn bgs_reconstruct_forms_in_file_map_reconstruct_forms(
    e: &mut Engine,
    this: Ptr,
    file: u32,
    arrays: u32,
) {
    let t = this.addr();
    if e.mem.u32(t + 0x18) == 0 {
        return;
    }
    e.call(FILE_REWIND, &args![file, 1u32]);
    let header = e.call(FILE_CURRENT_FORM, &args![file]).u32();
    while e.mem.u32(header) == e.global::<u32>(TAG_TES4) {
        e.call(FILE_NEXT_FORM, &args![file, 1u32]);
    }
    // The exe's locals from EBP-0xA8.
    e.with_stack(0xa8, |e, frame| {
        let ebp = frame.addr() + 0xa8;
        let blocks_at = ebp - 0x44;
        let sub_blocks_at = ebp - 0x40;
        let cells_at = ebp - 0x10;
        let forms_at = ebp - 0x1c;
        let offsets = ebp - 0x30;
        let pair = ebp - 0x58;
        let id_at = ebp - 0x60;
        let chunk_buffer = ebp - 0x78;
        let cell_data = ebp - 0x84;
        let record = ebp - 0x94;
        let next = |e: &mut Engine| {
            e.call(FILE_NEXT_FORM, &args![file, 1u32]);
        };
        let skip = |e: &mut Engine| {
            e.call(FILE_SKIP_GROUP, &args![file]);
        };
        let found = |e: &mut Engine, map: u32, key: u32, out: u32| -> bool {
            e.call(FILE_MAP_GET_AT, &args![map, key, out]).bool()
        };
        e.mem.set_u32(blocks_at, 0);
        e.mem.set_u32(sub_blocks_at, 0);
        e.mem.set_u32(cells_at, 0);
        e.mem.set_u32(forms_at, 0);
        let mut world_id = 0u32;
        let mut cell_id = 0u32;
        e.set_global(RECONSTRUCT_CELL, 0u32);
        let optimized = e.call(FILE_GET_OPTIMIZED, &args![file]).u8() != 0;
        let mut done = 0u32;
        fn_00843190(e, Ptr::new(offsets));
        let mut index = 0u32;
        let mut end = 0u32;
        while e.mem.u32(header) != 0 && done < e.mem.u32(t + 0x18) {
            e.call(PAIR_CONSTRUCT, &args![pair]);
            let tag = e.mem.u32(header);
            if tag == e.global::<u32>(TAG_GRUP) {
                if e.mem.u32(header + 8) == e.global::<u32>(TAG_CELL)
                    || e.mem.u32(header + 8) == e.global::<u32>(TAG_WRLD)
                {
                    e.mem.set_u32(blocks_at, 0);
                    let is_world = e.mem.u32(header + 8) == e.global::<u32>(TAG_WRLD);
                    if is_world {
                        if e.mem.u32(t + 0x14) != 0 {
                            next(e);
                        } else {
                            skip(e);
                        }
                    } else if e.mem.u32(t + 0x10) != 0 {
                        let interior = e.mem.u32(t + 0x10);
                        e.mem.set_u32(blocks_at, interior);
                        next(e);
                    } else {
                        skip(e);
                    }
                    e.mem.set_u32(sub_blocks_at, 0);
                    e.mem.set_u32(cells_at, 0);
                    e.mem.set_u32(forms_at, 0);
                } else if e.mem.u32(header + 0xc) != 0 {
                    match e.mem.u32(header + 0xc) {
                        1 => {
                            let label = e.mem.u32(header + 8);
                            e.mem.set_u32(id_at, label);
                            e.call(ADD_COMPILE_INDEX, &args![id_at, file]);
                            let worlds = e.mem.u32(t + 0x14);
                            let id = e.mem.u32(id_at);
                            if found(e, worlds, id, blocks_at) {
                                let blocks = e.mem.u32(blocks_at);
                                if optimized && fn_00841e20(e, Ptr::new(blocks), 5) {
                                    let offset = e.mem.u32(header + 0x10);
                                    fn_008431c0(e, Ptr::new(offsets), offset, 0);
                                }
                                next(e);
                            } else {
                                skip(e);
                            }
                            e.mem.set_u32(sub_blocks_at, 0);
                            e.mem.set_u32(cells_at, 0);
                            e.mem.set_u32(forms_at, 0);
                        }
                        2 | 4 => {
                            let blocks = e.mem.u32(blocks_at);
                            let label = e.mem.u32(header + 8);
                            if found(e, blocks, label, sub_blocks_at) {
                                next(e);
                            } else {
                                skip(e);
                            }
                            e.mem.set_u32(cells_at, 0);
                            e.mem.set_u32(forms_at, 0);
                        }
                        3 | 5 => {
                            let sub_blocks = e.mem.u32(sub_blocks_at);
                            let label = e.mem.u32(header + 8);
                            if found(e, sub_blocks, label, cells_at) {
                                next(e);
                            } else {
                                skip(e);
                            }
                            e.mem.set_u32(forms_at, 0);
                        }
                        6 => {
                            let forms = e.mem.u32(forms_at);
                            if forms != 0 {
                                let mask_2 =
                                    e.call(FORMS_MAP_TEST_FLAGS, &args![forms, 2u32]).bool();
                                let mut mask_4 =
                                    e.call(FORMS_MAP_TEST_FLAGS, &args![forms, 4u32]).bool();
                                if optimized && world_id == 0 && mask_4 {
                                    let offset = e.mem.u32(header + 0x10);
                                    fn_008431c0(e, Ptr::new(offsets), offset, forms);
                                    mask_4 = false;
                                }
                                if mask_2 || mask_4 {
                                    next(e);
                                } else {
                                    skip(e);
                                }
                            } else {
                                skip(e);
                            }
                        }
                        8 => {
                            let forms = e.mem.u32(forms_at);
                            if e.call(FORMS_MAP_TEST_FLAGS, &args![forms, 2u32]).bool() {
                                next(e);
                            } else {
                                skip(e);
                            }
                        }
                        9 => {
                            let forms = e.mem.u32(forms_at);
                            if e.call(FORMS_MAP_TEST_FLAGS, &args![forms, 4u32]).bool() {
                                next(e);
                            } else {
                                skip(e);
                            }
                        }
                        // The jump table (`00843120`) sends 7 here; the range check sends 10
                        // and up.
                        _ => skip(e),
                    }
                } else {
                    let label = e.mem.u32(header + 8);
                    if found(e, t, label, forms_at) {
                        next(e);
                    } else {
                        skip(e);
                    }
                }
            } else if tag == e.global::<u32>(TAG_WRLD) {
                world_id = e.mem.u32(header + 0xc);
                next(e);
            } else if tag == e.global::<u32>(TAG_CELL) {
                if e.mem.u32(cells_at) != 0 {
                    cell_id = e.mem.u32(header + 0xc);
                    let mut key = cell_id;
                    let blocks = e.mem.u32(blocks_at);
                    if blocks != 0 && fn_00843150(e, Ptr::new(blocks)) != 0 {
                        e.call(CHUNK_BUFFER_CONSTRUCT, &args![chunk_buffer]);
                        e.call(MEMSET, &args![chunk_buffer, 0u32, 0xcu32]);
                        let mut chunk = e.call(FILE_GET_CHUNK, &args![file]).u32();
                        while chunk != 0 {
                            if chunk == CHUNK_XCLC {
                                e.call(FILE_GET_CHUNK_DATA, &args![file, chunk_buffer, 0xcu32]);
                                if e.call(FILE_TEST_00401680, &args![file]).bool() {
                                    e.call(CHUNK_BUFFER_00462230, &args![chunk_buffer]);
                                }
                                break;
                            }
                            chunk = if e.call(FILE_NEXT_CHUNK, &args![file]).bool() {
                                e.call(FILE_GET_CHUNK, &args![file]).u32()
                            } else {
                                0
                            };
                        }
                        e.call(FILE_END_FORM, &args![file]);
                        let x = u32::from(e.mem.u16(chunk_buffer));
                        let y = u32::from(e.mem.u16(chunk_buffer + 4));
                        key = e.call(CELL_COORDINATE_KEY, &args![x, y]).u32();
                    }
                    e.set_global(RECONSTRUCT_CELL, 0u32);
                    let cells = e.mem.u32(cells_at);
                    if found(e, cells, key, forms_at) {
                        let forms = e.mem.u32(forms_at);
                        if e.call(FORMS_MAP_TEST_FLAGS, &args![forms, 1u32]).bool() {
                            let handler = e.global::<u32>(DATA_HANDLER);
                            e.call(HANDLER_LOAD_RECORD, &args![handler, file, 0u32, 0u32, 0u32]);
                            next(e);
                            let cell = e.call(FORM_BY_ID, &args![cell_id]).u32();
                            if cell != 0 {
                                let forms = e.mem.u32(forms_at);
                                let flags = e.call(WORD_GETTER_0044EDB0, &args![forms]).u32();
                                e.call(FORM_DATA_MAKE, &args![cell_data, cell, flags]);
                                fn_00843270(e, Ptr::new(arrays), cell_data);
                            } else {
                                e.call(SAVE_GAME_WARNING, &args![CELL_NOT_RECONSTRUCTED, cell_id]);
                            }
                            done = done.wrapping_add(1);
                        } else {
                            next(e);
                        }
                    } else {
                        next(e);
                    }
                } else {
                    let blocks = e.mem.u32(blocks_at);
                    let forms = e.call(WORD_GETTER_0044EDB0, &args![blocks]).u32();
                    e.mem.set_u32(forms_at, forms);
                    cell_id = e.mem.u32(header + 0xc);
                    e.set_global(RECONSTRUCT_CELL, 0u32);
                    next(e);
                }
            } else if tag == e.global::<u32>(TAG_TOFT) {
                let value = e.mem.u32(header + 0xc);
                if value == u32::MAX || value == 0xffff_fffe {
                    if e.call(ARRAY_COUNT, &args![offsets]).u32() != 0 {
                        let entry = fn_00844160(e, Ptr::new(offsets), index);
                        let offset = e.mem.u32(entry);
                        let map = e.mem.u32(entry + 4);
                        e.mem.set_u32(pair, offset);
                        e.mem.set_u32(pair + 4, map);
                        index = index.wrapping_add(1);
                        // The exe also sets a local byte here that it never reads.
                    } else {
                        e.mem.set_u32(header, 0);
                    }
                } else {
                    next(e);
                }
            } else {
                e.call(PAIR_CONSTRUCT, &args![record]);
                let id = e.mem.u32(header + 0xc);
                let forms = e.mem.u32(forms_at);
                fn_00841320(e, Ptr::new(forms), 0x10000);
                if e.call(FORM_DATA_GET_AT, &args![forms, id, record]).bool() {
                    if cell_id != 0 {
                        let current = e.global::<u32>(RECONSTRUCT_CELL);
                        if current == 0 || e.call(FORM_ID, &args![current]).u32() != cell_id {
                            let cell = e.call(FORM_BY_ID, &args![cell_id]).u32();
                            e.set_global(RECONSTRUCT_CELL, cell);
                        }
                    }
                    let handler = e.global::<u32>(DATA_HANDLER);
                    let form = e.mem.u32(record);
                    e.call(HANDLER_LOAD_RECORD, &args![handler, file, 0u32, form, 1u32]);
                    next(e);
                    if e.mem.u32(record) == 0 {
                        let form = e.call(FORM_BY_ID, &args![id]).u32();
                        e.mem.set_u32(record, form);
                    }
                    if e.mem.u32(record) != 0 {
                        fn_00843270(e, Ptr::new(arrays), record);
                    } else {
                        let tag = e.mem.u32(header);
                        let kind = e.call(FORM_TYPE_FROM_TAG, &args![tag]).u32();
                        let name = e.call(FORM_TYPE_NAME, &args![kind]).u32();
                        e.call(SAVE_GAME_WARNING, &args![FORM_NOT_RECONSTRUCTED, id, name]);
                    }
                    done = done.wrapping_add(1);
                } else {
                    next(e);
                }
            }
            if end != 0 && end <= e.call(FILE_POSITION, &args![file]).u32() {
                if e.call(ARRAY_COUNT, &args![offsets]).u32() > index {
                    let entry = fn_00844160(e, Ptr::new(offsets), index);
                    let offset = e.mem.u32(entry);
                    let map = e.mem.u32(entry + 4);
                    e.mem.set_u32(pair, offset);
                    e.mem.set_u32(pair + 4, map);
                    index = index.wrapping_add(1);
                } else {
                    e.mem.set_u32(header, 0);
                }
            }
            if e.call(PAIR_IS_SET, &args![pair]).bool() {
                let offset = e.mem.u32(pair);
                e.call(FILE_SET_OFFSET, &args![file, offset]);
                let (world_label, size) = if e.mem.u32(header + 0xc) == 1 {
                    let label = e.mem.u32(header + 8);
                    let size = e.mem.u32(header + 4);
                    next(e);
                    (label, size)
                } else {
                    (0, e.mem.u32(header + 4))
                };
                e.mem.set_u32(blocks_at, 0);
                e.mem.set_u32(sub_blocks_at, 0);
                e.mem.set_u32(cells_at, 0);
                e.mem.set_u32(forms_at, 0);
                cell_id = 0;
                world_id = 0;
                end = e.call(FILE_POSITION, &args![file]).u32().wrapping_add(size);
                if world_label != 0 {
                    world_id = world_label;
                    let worlds = e.mem.u32(t + 0x14);
                    found(e, worlds, world_id, blocks_at);
                } else {
                    let map = e.mem.u32(pair + 4);
                    e.mem.set_u32(forms_at, map);
                    cell_id = e.mem.u32(header + 8);
                }
                fn_00843170(e, Ptr::new(pair));
                if world_id != 0 && e.mem.u32(header + 0xc) == 9 {
                    skip(e);
                }
            }
        }
        e.set_global(RECONSTRUCT_CELL, 0u32);
        fn_00843220(e, Ptr::new(offsets));
    });
}

/// The scrap array's construction (`00844f00`, this unit) and its base constructor
/// (`00845430`, `thiscall`), its set-up (`008453a0`, `thiscall`: two words; also
/// `MAP_INITIALIZER`), its next free index (`00845460`, `thiscall`), `MemoryManager`'s
/// singleton (`00401020`, `cdecl`) and `MemoryManager::GetThreadScrapHeap` (`00aa42e0`).
const SCRAP_ARRAY_BASE_CONSTRUCT: u32 = 0x0084_5430;
const ARRAY_NEXT_INDEX: u32 = 0x0084_5460;
const MEMORY_MANAGER: u32 = 0x0040_1020;
const THREAD_SCRAP_HEAP: u32 = 0x00aa_42e0;
/// The arrays' element address (`006950d0`, `thiscall`: an index).
const ARRAY_ELEMENT: u32 = 0x0069_50d0;
/// The arrays' removal (`00844c90`, `thiscall`: an index, a word passed as 1).
const ARRAY_REMOVE: u32 = 0x0084_4c90;
/// `TESFile::GetFileForTempID` (engine map, `00474060`; `cdecl`, a form id) and
/// `TESDataHandler::GetCompiledFile` (`00465010`; `thiscall`: the compile index).
const FILE_FOR_TEMP_ID: u32 = 0x0047_4060;
const HANDLER_GET_COMPILED_FILE: u32 = 0x0046_5010;
/// `"Worldspace %08X %s with no file passed to BGSReconstructFormsInAllFilesMap::AddReference()"`.
const NO_FILE_WORLD_MESSAGE: u32 = 0x0107_eb48;
/// The callees of `00843c90` (all `thiscall`): `009cdae0` and `TESObjectREFR::GetScale`
/// (`00567400`) give two values in `ST0`, `00598040` (`ST0`) and `00567490` (a `float`) set
/// the scale; `00461580` (a word: the entry's mask test, a `bool`), `007af430` (the base form;
/// the engine map's `BGSSaveFormBuffer::GetForm` is a folded name), `00549580` and
/// `00440da0` (`bool`s), `TESWorldSpace::GetTerrainManager` (`00586170`) and
/// `BGSTerrainManager::HideTree` (`006fcfa0`: the reference, a word); `00437b90` (a `bool`),
/// `MiddleHighProcess::GetSavedAcquireObject` / `SetSavedAcquireObject` (`008d8520` /
/// `00407800`), `MobileObject::GetCurrentProcessType` (`00931850`),
/// `ProcessLists::RemoveReference` (`0096d470`: the reference, the type) on the process lists
/// (`011e0e80`), the 0xB4-byte object's constructor `00906dc0`,
/// `BGSSaveLoadGame::QueueInitPackageLocations` (`0084a520`) on `*011ddf38`, `004181e0` (the
/// object `TESCreature::InitDefaultWorn` `005f9e00` (four words) or `006047c0` (five words)
/// runs on), `0068efb0` and `00693ef0` (a byte).
const SCALE_SOURCE_009CDAE0: u32 = 0x009c_dae0;
const REFR_GET_SCALE: u32 = 0x0056_7400;
const SCALE_VALUE_00598040: u32 = 0x0059_8040;
const REFR_SET_SCALE_00567490: u32 = 0x0056_7490;
const ENTRY_TEST_MASK: u32 = 0x0046_1580;
const REFR_BASE_FORM: u32 = 0x007a_f430;
const FORM_TEST_00549580: u32 = 0x0054_9580;
const FORM_TEST_00440DA0: u32 = 0x0044_0da0;
const WORLD_GET_TERRAIN_MANAGER: u32 = 0x0058_6170;
const TERRAIN_HIDE_TREE: u32 = 0x006f_cfa0;
const REFR_TEST_00437B90: u32 = 0x0043_7b90;
const GET_SAVED_ACQUIRE_OBJECT: u32 = 0x008d_8520;
const SET_SAVED_ACQUIRE_OBJECT: u32 = 0x0040_7800;
const GET_CURRENT_PROCESS_TYPE: u32 = 0x0093_1850;
const PROCESS_LISTS_REMOVE_REFERENCE: u32 = 0x0096_d470;
const PROCESS_LISTS: u32 = 0x011e_0e80;
const ACQUIRE_OBJECT_CONSTRUCT: u32 = 0x0090_6dc0;
const ACQUIRE_OBJECT_SIZE: u32 = 0xb4;
const QUEUE_INIT_PACKAGE_LOCATIONS: u32 = 0x0084_a520;
const SAVE_LOAD_GAME: u32 = 0x011d_df38;
const ACTOR_BASE_004181E0: u32 = 0x0041_81e0;
const INIT_DEFAULT_WORN: u32 = 0x005f_9e00;
const INIT_WORN_006047C0: u32 = 0x0060_47c0;
const FORM_OBJECT_0068EFB0: u32 = 0x0068_efb0;
const OBJECT_SET_00693EF0: u32 = 0x0069_3ef0;

// Translated from 00843190 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `BGSOffsetDataArray` constructor (unnamed in the engine map; the vtable's RTTI names
/// the class): the `BSScrapArray<OffsetData,1024>` constructor `00844f00(0x20, 0, 0)`, then
/// the vtable. Answers `this`.
pub fn fn_00843190(e: &mut Engine, this: Ptr) -> Ptr {
    fn_00844f00(e, this, 0x20, 0, 0);
    e.mem.set_u32(this.addr(), OFFSET_DATA_ARRAY_VTABLE);
    this
}

// Translated from 008431c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Adds an offset entry (unnamed in the engine map; `thiscall`: two words; `RET 8`): an 8-byte
/// local filled by `004b0680(first, second)`, appended with `00844eb0`, whose index it
/// answers (`EAX` passed on).
pub fn fn_008431c0(e: &mut Engine, this: Ptr, first: u32, second: u32) -> u32 {
    e.with_stack(8, |e, entry| {
        e.call(FORM_DATA_MAKE, &args![entry, first, second]);
        fn_00844eb0(e, this, entry.addr())
    })
}

// Translated from 008431f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `BGSOffsetDataArray` scalar deleting destructor (unnamed in the engine map): `00843220`,
/// then frees `this` with bit 0 of `flags`. Answers `this`.
pub fn fn_008431f0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_00843220(e, this);
    free_if(e, this, flags)
}

// Translated from 00843270 (decompiled, FalloutNV.exe 1.4.0.525)
/// Appends a `ReconstructFormData` (`entry`, its form first) to one of the two arrays
/// (unnamed in the engine map; `RET 4`): to the one at +0x10 when the form's vtable +0xF0
/// answers true or its type is 0x39 or 0x41, else to the one at +0; answers `00844eb0`'s
/// index.
pub fn fn_00843270(e: &mut Engine, this: Ptr, entry: u32) -> u32 {
    let form = e.mem.u32(entry);
    let second = e.vcall(form, 0xf0, &args![]).bool()
        || e.call(FORM_TYPE, &args![e.mem.u32(entry)]).u32() == 0x39
        || e.call(FORM_TYPE, &args![e.mem.u32(entry)]).u32() == 0x41;
    if second {
        fn_00844eb0(e, this.byte_add(0x10), entry)
    } else {
        fn_00844eb0(e, this, entry)
    }
}

// Translated from 008432e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The address of entry `index` of both arrays together (unnamed in the engine map; `RET 4`):
/// in the one at +0 below its count (`0044ddc0`), else at `index - count` in the one at +0x10
/// (`00844160`, whose answer it passes on).
pub fn fn_008432e0(e: &mut Engine, this: Ptr, index: u32) -> u32 {
    let count = e.call(ARRAY_COUNT, &args![this]).u32();
    if index < count {
        fn_00844160(e, this, index)
    } else {
        fn_00844160(e, this.byte_add(0x10), index.wrapping_sub(count))
    }
}

// Translated from 00843330 (decompiled, FalloutNV.exe 1.4.0.525)
/// The first word (the form) of entry `index` of both arrays together (unnamed in the engine
/// map; `RET 4`), found as in `008432e0`.
pub fn fn_00843330(e: &mut Engine, this: Ptr, index: u32) -> u32 {
    let count = e.call(ARRAY_COUNT, &args![this]).u32();
    let entry = if index < count {
        fn_00844160(e, this, index)
    } else {
        fn_00844160(e, this.byte_add(0x10), index.wrapping_sub(count))
    };
    e.mem.u32(entry)
}

// Translated from 00843380 (decompiled, FalloutNV.exe 1.4.0.525)
/// Removes entry `index` of both arrays together (unnamed in the engine map; `RET 4`):
/// `00844c90(index, 1)` on the one at +0 below its count, else `00844c90(index - count, 1)` on
/// the one at +0x10. Answers true (`MOV AL,1`).
pub fn fn_00843380(e: &mut Engine, this: Ptr, index: u32) -> bool {
    let count = e.call(ARRAY_COUNT, &args![this]).u32();
    if index < count {
        e.call(ARRAY_REMOVE, &args![this, index, 1u32]);
    } else {
        e.call(
            ARRAY_REMOVE,
            &args![this.addr() + 0x10, index.wrapping_sub(count), 1u32],
        );
    }
    true
}

// Translated from 008438b0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 008438b0 - the compiler's exception frame (FS:[0] chain, state 0 that frees the 0x1c-byte block when its constructor throws) is not translated.
/// `BGSReconstructFormsInAllFilesMap::AddReference_ov2` (Xbox PDB; `thiscall`: a world space,
/// x, y, a form id, a word; `RET 0x14`). Nothing for a null world space. The file is
/// `GetFileForFormID(00460140(world), form_id)`; without one the warning `"Worldspace %08X %s
/// with no file ..."` (the world space's id and its name from vtable +0x130); otherwise the
/// file's map (found, or made and set) gets `BGSReconstructFormsInFileMap::AddCellForm_ov2(world,
/// x, y, 0, form_id, data)`, and +0x30 counts up.
#[allow(clippy::too_many_arguments)]
pub fn bgs_reconstruct_forms_in_all_files_map_add_reference_ov2(
    e: &mut Engine,
    this: Ptr,
    world: u32,
    x: u32,
    y: u32,
    form_id: u32,
    data: u32,
) {
    if world == 0 {
        return;
    }
    let key = e.call(FORM_KEY_00460140, &args![world]).u32();
    let file = bgs_reconstruct_forms_in_all_files_map_get_file_for_form_id(e, this, key, form_id);
    if file == 0 {
        let name = e.vcall(world, FORM_NAME_SLOT, &args![]).u32();
        let id = e.call(FORM_ID, &args![world]).u32();
        e.call(SAVE_GAME_WARNING, &args![NO_FILE_WORLD_MESSAGE, id, name]);
        return;
    }
    let map = file_map_for(e, this, file);
    bgs_reconstruct_forms_in_file_map_add_cell_form_ov2(
        e,
        Ptr::new(map),
        world,
        x,
        y,
        0,
        form_id,
        data,
    );
    count_added(e, this);
}

// Translated from 00843c90 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00843c90 - the compiler's exception frame (FS:[0] chain, state 0 that frees the 0xB4-byte block when its constructor throws) is not translated.
/// Finishes one reconstructed entry (unnamed in the engine map; `thiscall`, `this` not read:
/// the entry, a `ReconstructFormData` whose first word is the form; `RET 4`). For a form whose
/// vtable +0xF0 answers true (a reference): its scale is set again (`00567490` with
/// `00598040`'s value as a `float`) when vtable +0x1D0 has an object whose `009cdae0` (as a
/// `double`) differs from `TESObjectREFR::GetScale` (or either is NaN); with the entry's mask 8
/// (`00461580`) and a base form answering `00549580`, a reference answering `00440da0` in a
/// world space has its tree hidden (`HideTree(refr, 1)` on the world space's terrain manager).
/// When vtable +0xFC answers true: with `00437b90` true, mask 8 and a saved acquire object,
/// the reference is removed from the process lists, the object deleted (slot 0 with 1) and the
/// saved object cleared; with `00437b90` false and no saved object, a new 0xB4-byte one
/// (`00906dc0`) is saved and vtable +0x260 runs. Then, when vtable +0x100 answers true, its
/// package locations are queued, and with mask 8 and a vtable +0x1D0 object either
/// `InitDefaultWorn(refr, 1, 1, 1)` (vtable +0x21C true) or `006047c0(refr, 1, 1, 0, 1)` runs
/// on `004181e0`'s object. Otherwise a form whose vtable +0xF8 answers true gets slot +0x10
/// with 0 on its member at +0x30, and a form of type 0x43 with a `0068efb0` object gets
/// `00693ef0(00440da0(form))` on it.
pub fn fn_00843c90(e: &mut Engine, _this: Ptr, entry: u32) {
    let form = e.mem.u32(entry);
    if e.vcall(form, 0xf0, &args![]).bool() {
        let refr = e.mem.u32(entry);
        let object = e.vcall(refr, 0x1d0, &args![]).u32();
        if object != 0 {
            let saved = e.call(SCALE_SOURCE_009CDAE0, &args![object]).f64();
            let scale = e.call(REFR_GET_SCALE, &args![refr]).f64();
            // FUCOMPP then `TEST AH,0x44; JNP`: the scale is set unless the two are equal.
            if saved != scale {
                let value = e.call(SCALE_VALUE_00598040, &args![refr]).f64();
                e.call(REFR_SET_SCALE_00567490, &args![refr, x87_store32(value)]);
            }
        }
        if e.call(ENTRY_TEST_MASK, &args![entry, 8u32]).bool() {
            let base = e.call(REFR_BASE_FORM, &args![refr]).u32();
            if e.call(FORM_TEST_00549580, &args![base]).bool()
                && e.call(FORM_TEST_00440DA0, &args![refr]).bool()
            {
                let world = e.call(REFR_GET_WORLD_SPACE, &args![refr]).u32();
                if world != 0 {
                    let terrain = e.call(WORLD_GET_TERRAIN_MANAGER, &args![world]).u32();
                    e.call(TERRAIN_HIDE_TREE, &args![terrain, refr, 1u32]);
                }
            }
        }
        if e.vcall(refr, 0xfc, &args![]).bool() {
            if e.call(REFR_TEST_00437B90, &args![refr]).bool() {
                if e.call(ENTRY_TEST_MASK, &args![entry, 8u32]).bool()
                    && e.call(GET_SAVED_ACQUIRE_OBJECT, &args![refr]).u32() != 0
                {
                    let kind = e.call(GET_CURRENT_PROCESS_TYPE, &args![refr]).u32();
                    e.call(
                        PROCESS_LISTS_REMOVE_REFERENCE,
                        &args![PROCESS_LISTS, refr, kind],
                    );
                    let object = e.call(GET_SAVED_ACQUIRE_OBJECT, &args![refr]).u32();
                    if object != 0 {
                        e.vcall(object, 0, &args![1u32]);
                    }
                    e.call(SET_SAVED_ACQUIRE_OBJECT, &args![refr, 0u32]);
                }
            } else if e.call(GET_SAVED_ACQUIRE_OBJECT, &args![refr]).u32() == 0 {
                let block = e.call(OPERATOR_NEW, &args![ACQUIRE_OBJECT_SIZE]).u32();
                let object = if block != 0 {
                    e.call(ACQUIRE_OBJECT_CONSTRUCT, &args![block]).u32()
                } else {
                    0
                };
                e.call(SET_SAVED_ACQUIRE_OBJECT, &args![refr, object]);
                e.vcall(refr, 0x260, &args![]);
            }
            if e.vcall(refr, 0x100, &args![]).bool() {
                let save_load = e.global::<u32>(SAVE_LOAD_GAME);
                e.call(QUEUE_INIT_PACKAGE_LOCATIONS, &args![save_load, refr]);
                if e.call(ENTRY_TEST_MASK, &args![entry, 8u32]).bool()
                    && e.vcall(refr, 0x1d0, &args![]).u32() != 0
                {
                    if e.vcall(refr, 0x21c, &args![]).bool() {
                        let base = e.call(ACTOR_BASE_004181E0, &args![refr]).u32();
                        e.call(INIT_DEFAULT_WORN, &args![base, refr, 1u32, 1u32, 1u32]);
                    } else {
                        let base = e.call(ACTOR_BASE_004181E0, &args![refr]).u32();
                        e.call(
                            INIT_WORN_006047C0,
                            &args![base, refr, 1u32, 1u32, 0u32, 1u32],
                        );
                    }
                }
            }
        }
    } else if e.vcall(e.mem.u32(entry), 0xf8, &args![]).bool() {
        let form = e.mem.u32(entry);
        let member = if form != 0 { form + 0x30 } else { 0 };
        e.vcall(member, 0x10, &args![0u32]);
    } else if e.call(FORM_TYPE, &args![e.mem.u32(entry)]).u32() == 0x43 {
        let form = e.mem.u32(entry);
        if e.call(FORM_OBJECT_0068EFB0, &args![form]).u32() != 0 {
            let byte = u32::from(e.call(FORM_TEST_00440DA0, &args![form]).u8());
            let object = e.call(FORM_OBJECT_0068EFB0, &args![form]).u32();
            e.call(OBJECT_SET_00693EF0, &args![object, byte]);
        }
    }
}

// Translated from 00843fe0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSReconstructFormsInAllFilesMap::GetFileForFormID` (Xbox PDB; `thiscall`: a word it does
/// not read, a form id; `RET 8`): the loaded form's file (`TESForm::GetFile(-1)`), else
/// `TESFile::GetFileForTempID`, else the data handler's compiled file of the id's top byte.
pub fn bgs_reconstruct_forms_in_all_files_map_get_file_for_form_id(
    e: &mut Engine,
    _this: Ptr,
    _unused_1: u32,
    form_id: u32,
) -> u32 {
    let form = e.call(FORM_BY_ID, &args![form_id]).u32();
    if form != 0 {
        return e.call(FORM_GET_FILE, &args![form, u32::MAX]).u32();
    }
    let file = e.call(FILE_FOR_TEMP_ID, &args![form_id]).u32();
    if file != 0 {
        return file;
    }
    let handler = e.global::<u32>(DATA_HANDLER);
    e.call(HANDLER_GET_COMPILED_FILE, &args![handler, form_id >> 24])
        .u32()
}

// Translated from 00844040 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `NiTMap<unsigned int, ReconstructFormData>` constructor (the engine map's
/// `NiTMapBase<..>::NiTMapBase` name is the base's; `RET 4`): `008442a0(count)`, then the
/// vtable. Answers `this`.
pub fn fn_00844040(e: &mut Engine, this: Ptr, bucket_count: u32) -> Ptr {
    fn_008442a0(e, this, bucket_count);
    e.mem.set_u32(this.addr(), FORM_DATA_NITMAP_VTABLE);
    this
}

// Translated from 00844070 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `NiTMap<unsigned int, BGSReconstructFormsMap *>` constructor (`RET 4`): `00844590(count)`,
/// then the vtable. Answers `this`.
pub fn fn_00844070(e: &mut Engine, this: Ptr, bucket_count: u32) -> Ptr {
    e.call(FORMS_MAP_NITMAP_BASE_CONSTRUCT, &args![this, bucket_count]);
    e.mem.set_u32(this.addr(), FORMS_MAP_NITMAP_VTABLE);
    this
}

// Translated from 008440a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `NiTMap<unsigned int, BGSReconstructCellsMap *>` constructor (`RET 4`):
/// `00844690(count)`, then the vtable. Answers `this`.
pub fn fn_008440a0(e: &mut Engine, this: Ptr, bucket_count: u32) -> Ptr {
    e.call(CELLS_MAP_NITMAP_BASE_CONSTRUCT, &args![this, bucket_count]);
    e.mem.set_u32(this.addr(), CELLS_MAP_NITMAP_VTABLE);
    this
}

// Translated from 008440d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `NiTMap<unsigned int, BGSReconstructCellSubBlocksMap *>` constructor (`RET 4`):
/// `00844870(count)`, then the vtable. Answers `this`.
pub fn fn_008440d0(e: &mut Engine, this: Ptr, bucket_count: u32) -> Ptr {
    fn_00844870(e, this, bucket_count);
    e.mem.set_u32(this.addr(), SUB_BLOCKS_MAP_NITMAP_VTABLE);
    this
}

// Translated from 00844100 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `NiTMap<unsigned int, BGSReconstructCellBlocksMap *>` constructor (`RET 4`):
/// `00844970(count)`, then the vtable. Answers `this`.
pub fn fn_00844100(e: &mut Engine, this: Ptr, bucket_count: u32) -> Ptr {
    fn_00844970(e, this, bucket_count);
    e.mem.set_u32(this.addr(), CELL_BLOCKS_MAP_NITMAP_VTABLE);
    this
}

// Translated from 00844160 (decompiled, FalloutNV.exe 1.4.0.525)
/// The address of element `index` (unnamed in the engine map; `RET 4`): `006950d0(index)`,
/// whose answer it passes on.
pub fn fn_00844160(e: &mut Engine, this: Ptr, index: u32) -> u32 {
    e.call(ARRAY_ELEMENT, &args![this, index]).u32()
}

// Translated from 00844180 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTMap<unsigned int,ReconstructFormData>::_scalar_deleting_destructor_` (Xbox PDB):
/// `00844470`, then frees `this` with bit 0 of `flags`. Answers `this`.
pub fn ni_t_map_form_data_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(FORM_DATA_NITMAP_DESTRUCT, &args![this]);
    free_if(e, this, flags)
}

// Translated from 008441b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTMap<unsigned int,BGSReconstructFormsMap *>::_scalar_deleting_destructor_` (Xbox PDB):
/// `00844600`, then frees `this` with bit 0 of `flags`. Answers `this`.
pub fn ni_t_map_forms_map_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(FORMS_MAP_NITMAP_DESTRUCT, &args![this]);
    free_if(e, this, flags)
}

// Translated from 008441e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTMap<unsigned int,BGSReconstructCellsMap *>::_scalar_deleting_destructor_` (Xbox PDB):
/// `008447e0`, then frees `this` with bit 0 of `flags`. Answers `this`.
pub fn ni_t_map_cells_map_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_008447e0(e, this);
    free_if(e, this, flags)
}

// Translated from 00844210 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTMap<unsigned int,BGSReconstructCellSubBlocksMap *>::_scalar_deleting_destructor_` (Xbox
/// PDB): `008448e0`, then frees `this` with bit 0 of `flags`. Answers `this`.
pub fn ni_t_map_sub_blocks_map_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    fn_008448e0(e, this);
    free_if(e, this, flags)
}

// Translated from 00844240 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTMap<unsigned int,BGSReconstructCellBlocksMap *>::_scalar_deleting_destructor_` (Xbox
/// PDB): `008449e0`, then frees `this` with bit 0 of `flags`. Answers `this`.
pub fn ni_t_map_cell_blocks_map_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    fn_008449e0(e, this);
    free_if(e, this, flags)
}

// Translated from 00844270 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTMap<TESFile *,BGSReconstructFormsInFileMap *>::_scalar_deleting_destructor_` (Xbox PDB):
/// `00844e20`, then frees `this` with bit 0 of `flags`. Answers `this`.
pub fn ni_t_map_file_map_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_00844e20(e, this);
    free_if(e, this, flags)
}

// Translated from 008442a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `NiTMapBase<.., unsigned int, ReconstructFormData>` constructor (unnamed in the engine
/// map; `RET 4`): the body of `00844db0` with its own vtable. Answers `this`.
pub fn fn_008442a0(e: &mut Engine, this: Ptr, bucket_count: u32) -> Ptr {
    map_base_construct(e, this, FORM_DATA_NITMAP_BASE_VTABLE, bucket_count)
}

// Translated from 008447e0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 008447e0 - the exception-unwinding frame (FS:[0]) is not translated.
/// The `NiTMap<unsigned int, BGSReconstructCellsMap *>` destructor (unnamed in the engine map):
/// its vtable, the map emptied (`00438af0`), then the base destructor `00844840`.
pub fn fn_008447e0(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), CELLS_MAP_NITMAP_VTABLE);
    e.call(MAP_REMOVE_ALL, &args![this]);
    e.call(CELLS_MAP_NITMAP_BASE_DESTRUCT, &args![this]);
}

// Translated from 00844870 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `NiTMapBase<.., unsigned int, BGSReconstructCellSubBlocksMap *>` constructor (unnamed in
/// the engine map; `RET 4`): the body of `00844db0` with its own vtable. Answers `this`.
pub fn fn_00844870(e: &mut Engine, this: Ptr, bucket_count: u32) -> Ptr {
    map_base_construct(e, this, SUB_BLOCKS_MAP_NITMAP_BASE_VTABLE, bucket_count)
}

// Translated from 008448e0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 008448e0 - the exception-unwinding frame (FS:[0]) is not translated.
/// The `NiTMap<unsigned int, BGSReconstructCellSubBlocksMap *>` destructor (unnamed in the
/// engine map): its vtable, the map emptied (`00438af0`), then the base destructor `00844940`.
pub fn fn_008448e0(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), SUB_BLOCKS_MAP_NITMAP_VTABLE);
    e.call(MAP_REMOVE_ALL, &args![this]);
    e.call(SUB_BLOCKS_MAP_NITMAP_BASE_DESTRUCT, &args![this]);
}

// Translated from 00844970 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `NiTMapBase<.., unsigned int, BGSReconstructCellBlocksMap *>` constructor (unnamed in
/// the engine map; `RET 4`): the body of `00844db0` with its own vtable. Answers `this`.
pub fn fn_00844970(e: &mut Engine, this: Ptr, bucket_count: u32) -> Ptr {
    map_base_construct(e, this, CELL_BLOCKS_MAP_NITMAP_BASE_VTABLE, bucket_count)
}

// Translated from 008449e0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 008449e0 - the exception-unwinding frame (FS:[0]) is not translated.
/// The `NiTMap<unsigned int, BGSReconstructCellBlocksMap *>` destructor (unnamed in the engine
/// map): its vtable, the map emptied (`00438af0`), then the base destructor `00844a40`.
pub fn fn_008449e0(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), CELL_BLOCKS_MAP_NITMAP_VTABLE);
    e.call(MAP_REMOVE_ALL, &args![this]);
    e.call(CELL_BLOCKS_MAP_NITMAP_BASE_DESTRUCT, &args![this]);
}

// Translated from 00844eb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Appends an 8-byte element to the array (unnamed in the engine map; `RET 4`, the source's
/// address): the next index (`00845460`), one element constructed there (`00845300(element,
/// 1)`), the source's two words copied in; answers the index.
pub fn fn_00844eb0(e: &mut Engine, this: Ptr, source: u32) -> u32 {
    let t = this.addr();
    let index = e.call(ARRAY_NEXT_INDEX, &args![this]).u32();
    let element = e.mem.u32(t + 4).wrapping_add(index.wrapping_mul(8));
    e.call(ARRAY_CONSTRUCT, &args![this, element, 1u32]);
    let first = e.mem.u32(source);
    let second = e.mem.u32(source + 4);
    let element = e.mem.u32(t + 4).wrapping_add(index.wrapping_mul(8));
    e.mem.set_u32(element, first);
    e.mem.set_u32(element + 4, second);
    index
}

// Translated from 00844f00 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00844f00 - the exception-unwinding frame (FS:[0]) is not translated.
/// The `BSScrapArray<OffsetData,1024>` constructor (unnamed in the engine map; `RET 0xC`: two
/// words for the set-up, a scrap heap): the base constructor (`00845430`), the vtable, the
/// heap at +0x10 (the given one, or `MemoryManager::GetThreadScrapHeap` of `00401020`'s
/// manager when it is 0), then `008453a0(first, second)`. Answers `this`.
pub fn fn_00844f00(e: &mut Engine, this: Ptr, first: u32, second: u32, heap: u32) -> Ptr {
    e.call(SCRAP_ARRAY_BASE_CONSTRUCT, &args![this]);
    e.mem.set_u32(this.addr(), SCRAP_ARRAY_VTABLE);
    let heap = if heap != 0 {
        heap
    } else {
        let manager = e.call(MEMORY_MANAGER, &args![]).u32();
        e.call(THREAD_SCRAP_HEAP, &args![manager]).u32()
    };
    e.mem.set_u32(this.addr() + 0x10, heap);
    e.call(MAP_INITIALIZER, &args![this, first, second]);
    this
}

// Translated from 00845050 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSScrapArray<OffsetData,1024>::_Reallocate` (Xbox PDB; `RET 8`: the old block, the new
/// element count): a new block from the scrap heap (+0x10, `count * 8` bytes, alignment
/// `*010a2720`), the old elements copied in (`00401460`, `0044ddc0`'s count times 8 bytes), the
/// old block given back (`ScrapHeap::Deallocate`); answers the new block.
pub fn bs_scrap_array_offset_data_reallocate(
    e: &mut Engine,
    this: Ptr,
    old: u32,
    count: u32,
) -> u32 {
    let alignment = e.global::<u32>(SCRAP_ALIGNMENT);
    let heap = e.mem.u32(this.addr() + 0x10);
    let block = e
        .call(SCRAP_HEAP_ALLOCATE, &args![heap, count << 3, alignment])
        .u32();
    let used = e.call(ARRAY_COUNT, &args![this]).u32() << 3;
    e.call(MEMORY_COPY_00401460, &args![block, old, used]);
    let heap = e.mem.u32(this.addr() + 0x10);
    e.call(SCRAP_HEAP_DEALLOCATE, &args![heap, old]);
    block
}

// Translated from 008450b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTMapBase<..ReconstructFormData>::_scalar_deleting_destructor_` (Xbox PDB): `008444d0`,
/// then frees `this` with bit 0 of `flags`. Answers `this`.
pub fn ni_t_map_base_form_data_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    e.call(FORM_DATA_NITMAP_BASE_DESTRUCT, &args![this]);
    free_if(e, this, flags)
}

// Translated from 008450e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTMapBase<..BGSReconstructFormsMap *>::_scalar_deleting_destructor_` (Xbox PDB):
/// `00844660`, then frees `this` with bit 0 of `flags`. Answers `this`.
pub fn ni_t_map_base_forms_map_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    e.call(FORMS_MAP_NITMAP_BASE_DESTRUCT, &args![this]);
    free_if(e, this, flags)
}

// Translated from 00845110 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTMapBase<..BGSReconstructCellsMap *>::_scalar_deleting_destructor_` (Xbox PDB):
/// `00844840`, then frees `this` with bit 0 of `flags`. Answers `this`.
pub fn ni_t_map_base_cells_map_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    e.call(CELLS_MAP_NITMAP_BASE_DESTRUCT, &args![this]);
    free_if(e, this, flags)
}

// Translated from 00845140 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTMapBase<..BGSReconstructCellSubBlocksMap *>::_scalar_deleting_destructor_` (Xbox PDB):
/// `00844940`, then frees `this` with bit 0 of `flags`. Answers `this`.
pub fn ni_t_map_base_sub_blocks_map_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    e.call(SUB_BLOCKS_MAP_NITMAP_BASE_DESTRUCT, &args![this]);
    free_if(e, this, flags)
}

// Translated from 00845170 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTMapBase<..BGSReconstructCellBlocksMap *>::_scalar_deleting_destructor_` (Xbox PDB):
/// `00844a40`, then frees `this` with bit 0 of `flags`. Answers `this`.
pub fn ni_t_map_base_cell_blocks_map_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    e.call(CELL_BLOCKS_MAP_NITMAP_BASE_DESTRUCT, &args![this]);
    free_if(e, this, flags)
}

// Translated from 008451a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<ReconstructFormData,1024>::_scalar_deleting_destructor_` (Xbox PDB):
/// `00844ac0`, then frees `this` with bit 0 of `flags`. Answers `this`.
pub fn bs_simple_array_form_data_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    fn_00844ac0(e, this);
    free_if(e, this, flags)
}

// Translated from 008451d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTMapBase<..TESFile *,BGSReconstructFormsInFileMap *>::_scalar_deleting_destructor_` (Xbox
/// PDB): `00844e80`, then frees `this` with bit 0 of `flags`. Answers `this`.
pub fn ni_t_map_base_file_map_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    e.call(FILE_MAP_NITMAP_BASE_DESTRUCT, &args![this]);
    free_if(e, this, flags)
}

// Translated from 00845200 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<OffsetData,1024>::_scalar_deleting_destructor_` (Xbox PDB): `00844fa0`, then
/// frees `this` with bit 0 of `flags`. Answers `this`.
pub fn bs_simple_array_offset_data_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    e.call(OFFSET_DATA_ARRAY_BASE_DESTRUCT, &args![this]);
    free_if(e, this, flags)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x0084_3630,
            bgs_reconstruct_forms_in_all_files_map_add_form(Ptr, u32, u32)
        ),
        entry!(
            0x0084_3750,
            bgs_reconstruct_forms_in_all_files_map_add_reference(Ptr, u32, u32, u32)
        ),
        entry!(0x0084_3150, fn_00843150(Ptr) -> u8),
        entry!(0x0084_3170, fn_00843170(Ptr)),
        entry!(0x0084_3240, fn_00843240(Ptr)),
        entry!(
            0x0084_3440,
            bgs_reconstruct_forms_in_all_files_map_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(
            0x0084_34d0,
            bgs_reconstruct_forms_in_all_files_map_destructor(Ptr)
        ),
        entry!(0x0084_3540, fn_00843540(Ptr)),
        entry!(0x0084_35a0, fn_008435a0(Ptr)),
        entry!(0x0084_39e0, fn_008439e0(Ptr)),
        entry!(0x0084_3bb0, fn_00843bb0(Ptr) -> u32),
        entry!(0x0084_3be0, fn_00843be0(Ptr, u32)),
        entry!(0x0084_3c10, fn_00843c10(Ptr, bool)),
        entry!(0x0084_4ae0, fn_00844ae0(Ptr, u32, u8)),
        entry!(0x0084_4c20, fn_00844c20(Ptr, u32)),
        entry!(0x0084_4e20, fn_00844e20(Ptr)),
        entry!(0x0084_3220, fn_00843220(Ptr)),
        entry!(0x00844db0, fn_00844db0(Ptr, u32) -> Ptr),
        entry!(0x00844130, fn_00844130(Ptr, u32) -> Ptr),
        entry!(0x00844a90, fn_00844a90(Ptr) -> Ptr),
        entry!(0x00843470, fn_00843470(Ptr) -> Ptr),
        entry!(0x008433d0, bgsreconstructformsinallfilesmap_constructor(Ptr) -> Ptr),
        entry!(0x0084_4ac0, fn_00844ac0(Ptr)),
        entry!(0x006d_b9b0, bs_scrap_array_offset_data_allocate(Ptr, u32) -> u32),
        entry!(0x0084_1190, bgs_reconstruct_forms_map_constructor(Ptr) -> Ptr),
        entry!(0x0084_11c0, bgs_reconstruct_forms_map_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x0084_11f0, fn_008411f0(Ptr)),
        entry!(
            0x0084_1250,
            bgs_reconstruct_forms_map_add_form(Ptr, u32, u32, u32)
        ),
        entry!(0x0084_1320, fn_00841320(Ptr, u32)),
        entry!(0x0084_1340, fn_00841340(Ptr) -> Ptr),
        entry!(0x0084_1370, bgs_reconstruct_cells_map_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x0084_13a0, fn_008413a0(Ptr)),
        entry!(0x0084_1460, fn_00841460(Ptr, u32, u32, u32, u32)),
        entry!(0x0084_1580, fn_00841580(Ptr, u32, u32, u32, u32, u32, u32)),
        entry!(0x0084_1660, fn_00841660(Ptr, u32) -> bool),
        entry!(0x0084_16c0, fn_008416c0(Ptr) -> Ptr),
        entry!(0x0084_16f0, bgs_reconstruct_cell_sub_blocks_map_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x0084_1720, fn_00841720(Ptr)),
        entry!(0x0084_17e0, fn_008417e0(Ptr, u32, u32, u32, u32)),
        entry!(0x0084_18b0, fn_008418b0(Ptr, u32, u32, u32, u32, u32, u32)),
        entry!(0x0084_1990, fn_00841990(Ptr, u32) -> bool),
        entry!(0x0084_19f0, bgs_reconstruct_cell_blocks_map_constructor(Ptr, u8) -> Ptr),
        entry!(0x0084_1a30, bgs_reconstruct_cell_blocks_map_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x0084_1a60, fn_00841a60(Ptr)),
        entry!(
            0x0084_1b20,
            bgs_reconstruct_cell_blocks_map_add_cell_form(Ptr, u32, u32, u32, u32)
        ),
        entry!(
            0x0084_1ca0,
            bgs_reconstruct_cell_blocks_map_add_cell_form_ov2(Ptr, u32, u32, u32, u32, u32, u32)
        ),
        entry!(0x0084_1e20, fn_00841e20(Ptr, u32) -> bool),
        entry!(0x0084_1ea0, fn_00841ea0(Ptr) -> Ptr),
        entry!(0x0084_1ed0, bgs_reconstruct_world_spaces_map_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x0084_1f00, fn_00841f00(Ptr)),
        entry!(
            0x0084_1fc0,
            bgs_reconstruct_world_spaces_map_add_cell_form(Ptr, u32, u32, u32, u32)
        ),
        entry!(
            0x0084_20a0,
            bgs_reconstruct_world_spaces_map_add_cell_form_ov2(Ptr, u32, u32, u32, u32, u32, u32)
        ),
        entry!(0x0084_2180, bgs_reconstruct_forms_in_file_map_constructor(Ptr) -> Ptr),
        entry!(0x0084_21c0, bgs_reconstruct_forms_in_file_map_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x0084_21f0, fn_008421f0(Ptr)),
        entry!(
            0x0084_2310,
            bgs_reconstruct_forms_in_file_map_add_form(Ptr, u32, u32)
        ),
        entry!(
            0x0084_2600,
            bgs_reconstruct_forms_in_file_map_add_cell_form(Ptr, u32, u32, u32, u32)
        ),
        entry!(
            0x0084_2730,
            bgs_reconstruct_forms_in_file_map_add_cell_form_ov2(Ptr, u32, u32, u32, u32, u32, u32)
        ),
        entry!(
            0x0084_27f0,
            bgs_reconstruct_forms_in_file_map_reconstruct_forms(Ptr, u32, u32)
        ),
        entry!(0x0084_3190, fn_00843190(Ptr) -> Ptr),
        entry!(0x0084_31c0, fn_008431c0(Ptr, u32, u32) -> u32),
        entry!(0x0084_31f0, fn_008431f0(Ptr, u32) -> Ptr),
        entry!(0x0084_3270, fn_00843270(Ptr, u32) -> u32),
        entry!(0x0084_32e0, fn_008432e0(Ptr, u32) -> u32),
        entry!(0x0084_3330, fn_00843330(Ptr, u32) -> u32),
        entry!(0x0084_3380, fn_00843380(Ptr, u32) -> bool),
        entry!(
            0x0084_38b0,
            bgs_reconstruct_forms_in_all_files_map_add_reference_ov2(Ptr, u32, u32, u32, u32, u32)
        ),
        entry!(0x0084_3c90, fn_00843c90(Ptr, u32)),
        entry!(0x0084_3fe0, bgs_reconstruct_forms_in_all_files_map_get_file_for_form_id(Ptr, u32, u32) -> u32),
        entry!(0x0084_4040, fn_00844040(Ptr, u32) -> Ptr),
        entry!(0x0084_4070, fn_00844070(Ptr, u32) -> Ptr),
        entry!(0x0084_40a0, fn_008440a0(Ptr, u32) -> Ptr),
        entry!(0x0084_40d0, fn_008440d0(Ptr, u32) -> Ptr),
        entry!(0x0084_4100, fn_00844100(Ptr, u32) -> Ptr),
        entry!(0x0084_4160, fn_00844160(Ptr, u32) -> u32),
        entry!(0x0084_4180, ni_t_map_form_data_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x0084_41b0, ni_t_map_forms_map_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x0084_41e0, ni_t_map_cells_map_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x0084_4210, ni_t_map_sub_blocks_map_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x0084_4240, ni_t_map_cell_blocks_map_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x0084_4270, ni_t_map_file_map_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x0084_42a0, fn_008442a0(Ptr, u32) -> Ptr),
        entry!(0x0084_47e0, fn_008447e0(Ptr)),
        entry!(0x0084_4870, fn_00844870(Ptr, u32) -> Ptr),
        entry!(0x0084_48e0, fn_008448e0(Ptr)),
        entry!(0x0084_4970, fn_00844970(Ptr, u32) -> Ptr),
        entry!(0x0084_49e0, fn_008449e0(Ptr)),
        entry!(0x0084_4eb0, fn_00844eb0(Ptr, u32) -> u32),
        entry!(0x0084_4f00, fn_00844f00(Ptr, u32, u32, u32) -> Ptr),
        entry!(0x0084_5050, bs_scrap_array_offset_data_reallocate(Ptr, u32, u32) -> u32),
        entry!(0x0084_50b0, ni_t_map_base_form_data_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x0084_50e0, ni_t_map_base_forms_map_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x0084_5110, ni_t_map_base_cells_map_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x0084_5140, ni_t_map_base_sub_blocks_map_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x0084_5170, ni_t_map_base_cell_blocks_map_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x0084_51a0, bs_simple_array_form_data_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x0084_51d0, ni_t_map_base_file_map_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x0084_5200, bs_simple_array_offset_data_scalar_deleting_destructor(Ptr, u32) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn doubles(e: &mut Engine) {
        e.register(ALLOCATE, |e, a| Ret {
            eax: e.mem.alloc(a[0]),
            ..Ret::default()
        });
        e.register(MEMSET, |e, a| {
            for i in 0..a[2] {
                e.mem.set_u8(a[0] + i, a[1] as u8);
            }
            Ret::default()
        });
        e.register(MAP_INITIALIZER, |_, _| Ret::default());
    }

    #[test]
    fn fn_00844db0_builds_a_zeroed_bucket_table() {
        let mut e = Engine::new();
        doubles(&mut e);
        let this = e.mem.alloc(0x20);
        e.mem.set_u32(this + 0xc, 77);
        let r = e.call(0x00844db0, &args![this, 5u32]);
        assert_eq!(r.u32(), this);
        assert_eq!(e.mem.u32(this), 0x0107_ed1c);
        assert_eq!(e.mem.u32(this + 4), 5);
        assert_eq!(e.mem.u32(this + 0xc), 0);
        let table = e.mem.u32(this + 8);
        assert_eq!(e.mem.block_size(table), Some(24), "20 bytes, rounded up");
        assert_eq!(e.mem.bytes(table, 20), vec![0; 20]);
    }

    #[test]
    fn fn_00844130_replaces_the_vtable() {
        let mut e = Engine::new();
        doubles(&mut e);
        let this = e.mem.alloc(0x20);
        let r = e.call(0x00844130, &args![this, 3u32]);
        assert_eq!(r.u32(), this);
        assert_eq!(e.mem.u32(this), 0x0107_ec48);
        assert_eq!(e.mem.u32(this + 4), 3);
    }

    #[test]
    fn fn_00844a90_stores_the_vtable_and_initializes() {
        let mut e = Engine::new();
        doubles(&mut e);
        let this = e.mem.alloc(0x10);
        e.call_log = Some(vec![]);
        let r = e.call(0x00844a90, &args![this]);
        assert_eq!(r.u32(), this);
        assert_eq!(e.mem.u32(this), 0x0107_ed08);
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1], (MAP_INITIALIZER, vec![this, 0, 0]));
    }

    #[test]
    fn fn_00843470_initializes_both_members() {
        let mut e = Engine::new();
        doubles(&mut e);
        let this = e.mem.alloc(0x30);
        e.call_log = Some(vec![]);
        let r = e.call(0x00843470, &args![this]);
        assert_eq!(r.u32(), this);
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1], (MAP_INITIALIZER, vec![this, 0, 0]));
        assert_eq!(log[2], (MAP_INITIALIZER, vec![this + 0x10, 0, 0]));
        assert_eq!(e.mem.u32(this), 0x0107_ed08);
        assert_eq!(e.mem.u32(this + 0x10), 0x0107_ed08);
    }

    #[test]
    fn the_constructor_builds_the_base_member_and_clears_the_tail() {
        let mut e = Engine::new();
        doubles(&mut e);
        let this = e.mem.alloc(0x40);
        e.mem.set_u32(this + 0x30, 9);
        let r = e.call(0x008433d0, &args![this]);
        assert_eq!(r.u32(), this);
        assert_eq!(e.mem.u32(this), 0x0107_ea74, "the final vtable");
        assert_eq!(e.mem.u32(this + 4), 0x25, "base bucket count");
        assert_eq!(e.mem.u32(this + 0x10), 0x0107_ed08);
        assert_eq!(e.mem.u32(this + 0x30), 0);
    }
}

#[cfg(test)]
mod thunk_tests {
    use super::*;

    fn calls_to(e: &Engine, callee: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == callee)
            .map(|(_, words)| words.clone())
            .collect()
    }

    #[test]
    fn fwd_a03_00843220_passes_this_to_the_callee() {
        let mut e = Engine::new();
        e.register(0x0084_4ff0, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0084_3220, &args![0x0000_5000u32]);
        assert_eq!(calls_to(&e, 0x0084_4ff0), vec![vec![0x5000]]);
    }
}

#[cfg(test)]
mod array_destructor_tests {
    use super::*;

    fn check_array_destructor(address: u32, vtable: u32, release: u32) {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x10);
        e.register(release, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(address, &args![this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.mem.u32(this), vtable);
        let hits: Vec<_> = log.iter().filter(|c| c.0 == release).collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, vec![this, 1]);
    }

    #[test]
    fn fn_00844ac0_sets_the_vtable_then_releases_the_buffer() {
        check_array_destructor(0x0084_4ac0, ARRAY_VTABLE_0107ED08, ARRAY_RELEASE_008454F0);
    }
}

#[cfg(test)]
mod b023_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    fn calls_to(log: &[(u32, Vec<u32>)], addr: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    fn quiet(e: &mut Engine, addrs: &[u32]) {
        for a in addrs {
            e.register(*a, |_, _| Ret::default());
        }
    }

    #[test]
    fn small_members() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x34);
        e.mem.set_u8(this + 0x14, 0x5a);
        assert_eq!(e.call(0x0084_3150, &[this]).u8(), 0x5a);
        e.mem.set_u32(this, 7);
        e.mem.set_u32(this + 4, 8);
        e.mem.set_u32(this + 8, 9);
        e.call(0x0084_3170, &[this]);
        assert_eq!(e.mem.u32(this), 0);
        assert_eq!(e.mem.u32(this + 4), 0);
        assert_eq!(e.mem.u32(this + 8), 9);
        e.register(ARRAY_COUNT, |e, a| ret(e.mem.u32(a[0] + 8)));
        e.mem.set_u32(this + 8, 3);
        e.mem.set_u32(this + 0x18, 4);
        assert_eq!(e.call(0x0084_3bb0, &[this]).u32(), 7);
        let tls = e.tls();
        e.mem.set_u32(tls + TLS_FLAGS, 0x10);
        e.call(0x0084_3c10, &[0, 0x100]);
        assert_eq!(e.mem.u32(tls + TLS_FLAGS), 0x10);
        e.call(0x0084_3c10, &[0, 1]);
        assert_eq!(e.mem.u32(tls + TLS_FLAGS), 0x14);
        e.call(0x0084_3c10, &[0, 0]);
        assert_eq!(e.mem.u32(tls + TLS_FLAGS), 0x10);
    }

    /// An array (buffer `+4`, size `+8`, allocated `+0xC`) whose vtable slot
    /// 4 is `0x7000_0004`.
    fn array(e: &mut Engine, buffer: u32, size: u32, allocated: u32) -> u32 {
        e.put_vtable(0x0600_0000, &[0, 0x7000_0004]);
        let a = e.mem.alloc(0x10);
        e.mem.set_u32(a, 0x0600_0000);
        e.mem.set_u32(a + 4, buffer);
        e.mem.set_u32(a + 8, size);
        e.mem.set_u32(a + 0xc, allocated);
        a
    }

    #[test]
    fn array_set_size() {
        let mut e = Engine::new();
        quiet(
            &mut e,
            &[
                0x0084_54f0,
                ARRAY_REALLOCATE,
                ARRAY_CONSTRUCT,
                ARRAY_DESTROY,
            ],
        );
        e.register(0x7000_0004, |_, _| ret(0x0800_0000));
        // Size 0: released with the byte as given.
        let a = array(&mut e, 0x1000, 4, 8);
        e.call_log = Some(vec![]);
        e.call(0x0084_4ae0, &[a, 0, 2]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, 0x0084_54f0), vec![vec![a, 2]]);
        // Growing an empty array: allocated through slot 4.
        let a = array(&mut e, 0, 0, 0);
        e.call_log = Some(vec![]);
        e.call(0x0084_4ae0, &[a, 3, 0]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, 0x7000_0004), vec![vec![a, 3]]);
        assert_eq!(
            calls_to(&log, ARRAY_CONSTRUCT),
            vec![vec![a, 0x0800_0000, 3]]
        );
        assert_eq!(
            e.mem.bytes(a + 4, 12),
            [0x0800_0000u32, 3, 3]
                .iter()
                .flat_map(|w| w.to_le_bytes())
                .collect::<Vec<u8>>()
        );
        // Growing past the allocation: reallocated, then constructed.
        let a = array(&mut e, 0x1000, 2, 4);
        e.call_log = Some(vec![]);
        e.call(0x0084_4ae0, &[a, 6, 0]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, ARRAY_REALLOCATE), vec![vec![a, 6, 2]]);
        assert_eq!(calls_to(&log, ARRAY_CONSTRUCT), vec![vec![a, 0x1010, 4]]);
        assert_eq!((e.mem.u32(a + 8), e.mem.u32(a + 0xc)), (6, 6));
        // Growing within it: constructed only.
        let a = array(&mut e, 0x1000, 2, 8);
        e.call_log = Some(vec![]);
        e.call(0x0084_4ae0, &[a, 5, 1]);
        let log = e.call_log.take().unwrap();
        assert!(calls_to(&log, ARRAY_REALLOCATE).is_empty());
        assert_eq!(calls_to(&log, ARRAY_CONSTRUCT), vec![vec![a, 0x1010, 3]]);
        assert_eq!(e.mem.u32(a + 8), 5);
        // Shrinking: destroyed; with the byte set and a quarter or less of
        // the allocation, reallocated to the size.
        let a = array(&mut e, 0x1000, 6, 8);
        e.call_log = Some(vec![]);
        e.call(0x0084_4ae0, &[a, 2, 1]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, ARRAY_DESTROY), vec![vec![a, 0x1010, 4]]);
        assert_eq!(calls_to(&log, ARRAY_REALLOCATE), vec![vec![a, 2, 2]]);
        assert_eq!((e.mem.u32(a + 8), e.mem.u32(a + 0xc)), (2, 2));
        let a = array(&mut e, 0x1000, 6, 8);
        e.call_log = Some(vec![]);
        e.call(0x0084_4ae0, &[a, 3, 1]);
        assert!(calls_to(&e.call_log.take().unwrap(), ARRAY_REALLOCATE).is_empty());
        e.call_log = Some(vec![]);
        e.call(0x0084_4ae0, &[a, 1, 0x100]);
        assert!(calls_to(&e.call_log.take().unwrap(), ARRAY_REALLOCATE).is_empty());
    }

    #[test]
    fn array_allocated_size() {
        let mut e = Engine::new();
        quiet(&mut e, &[ARRAY_REALLOCATE, ARRAY_DESTROY]);
        let a = array(&mut e, 0x1000, 6, 8);
        e.call_log = Some(vec![]);
        e.call(0x0084_4c20, &[a, 8]);
        assert_eq!(e.call_log.take().unwrap().len(), 1);
        e.call_log = Some(vec![]);
        e.call(0x0084_4c20, &[a, 4]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, ARRAY_DESTROY), vec![vec![a, 0x1020, 2]]);
        assert_eq!(calls_to(&log, ARRAY_REALLOCATE), vec![vec![a, 4, 4]]);
        assert_eq!((e.mem.u32(a + 8), e.mem.u32(a + 0xc)), (4, 4));
        e.call_log = Some(vec![]);
        e.call(0x0084_4c20, &[a, 16]);
        let log = e.call_log.take().unwrap();
        assert!(calls_to(&log, ARRAY_DESTROY).is_empty());
        assert_eq!(calls_to(&log, ARRAY_REALLOCATE), vec![vec![a, 16, 4]]);
        // Both arrays of the pair.
        let pair = e.mem.alloc(0x20);
        e.mem.set_u32(pair + 0xc, 1);
        e.mem.set_u32(pair + 0x1c, 1);
        e.call_log = Some(vec![]);
        e.call(0x0084_3be0, &[pair, 5]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, ARRAY_REALLOCATE),
            vec![vec![pair, 5, 0], vec![pair + 0x10, 5, 0]]
        );
        // 00843240 empties both, freeing.
        quiet(&mut e, &[0x0084_54f0]);
        e.call_log = Some(vec![]);
        e.call(0x0084_3240, &[pair]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, 0x0084_54f0),
            vec![vec![pair, 1], vec![pair + 0x10, 1]]
        );
    }

    /// A map of `entries` (key, value) walked through positions 1..=n.
    fn map_doubles(e: &mut Engine, entries: Vec<(u32, u32)>) {
        let n = entries.len() as u32;
        e.register_double(MAP_FIRST_POSITION, move |_, _| ret(u32::from(n > 0)));
        let entries = Rc::new(entries);
        e.register_double(MAP_NEXT, move |e, a| {
            let position = e.mem.u32(a[1]);
            assert_eq!(e.mem.u32(a[2]), 0);
            assert_eq!(e.mem.u32(a[3]), 0);
            let (key, value) = entries[position as usize - 1];
            e.mem.set_u32(a[2], key);
            e.mem.set_u32(a[3], value);
            let next = if position < entries.len() as u32 {
                position + 1
            } else {
                0
            };
            e.mem.set_u32(a[1], next);
            Ret::default()
        });
    }

    #[test]
    fn deleting_the_file_maps_and_the_destructor() {
        let mut e = Engine::new();
        e.put_vtable(0x0610_0000, &[0x7000_0000]);
        quiet(
            &mut e,
            &[
                0x7000_0000,
                MAP_REMOVE_ALL,
                0x0084_54f0,
                0x0084_4e80,
                0x0040_1030,
            ],
        );
        let first = e.mem.alloc(4);
        let second = e.mem.alloc(4);
        e.mem.set_u32(first, 0x0610_0000);
        e.mem.set_u32(second, 0x0610_0000);
        map_doubles(&mut e, vec![(1, first), (2, 0), (3, second)]);
        let this = e.mem.alloc(0x34);
        e.mem.set_u32(this + 0x30, 9);
        e.call_log = Some(vec![]);
        e.call(0x0084_35a0, &[this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, 0x7000_0000),
            vec![vec![first, 1], vec![second, 1]]
        );
        assert_eq!(calls_to(&log, MAP_REMOVE_ALL), vec![vec![this]]);
        assert_eq!(
            calls_to(&log, 0x0084_54f0),
            vec![vec![this + 0x10, 1], vec![this + 0x20, 1]]
        );
        assert_eq!(e.mem.u32(this + 0x30), 0);

        // The destructor: vtables stored, the arrays' destructors, then the
        // map's.
        map_doubles(&mut e, vec![]);
        for (flags, frees) in [(0u32, 0usize), (1, 1)] {
            e.call_log = Some(vec![]);
            assert_eq!(e.call(0x0084_3440, &[this, flags]).u32(), this);
            let log = e.call_log.take().unwrap();
            assert_eq!(calls_to(&log, 0x0040_1030).len(), frees);
            assert_eq!(e.mem.u32(this), FILE_MAP_VTABLE);
            assert_eq!(e.mem.u32(this + 0x10), ARRAY_VTABLE_0107ED08);
            assert_eq!(e.mem.u32(this + 0x20), ARRAY_VTABLE_0107ED08);
            // 00843540 releases +0x20 then +0x10, after 008435a0's two.
            assert_eq!(
                calls_to(&log, 0x0084_54f0),
                vec![
                    vec![this + 0x10, 1],
                    vec![this + 0x20, 1],
                    vec![this + 0x20, 1],
                    vec![this + 0x10, 1],
                ]
            );
            assert_eq!(calls_to(&log, MAP_REMOVE_ALL), vec![vec![this]; 2]);
            assert_eq!(calls_to(&log, 0x0084_4e80), vec![vec![this]]);
        }
    }

    #[test]
    fn reconstruct_every_file() {
        let mut e = Engine::new();
        e.map(0x011c_3000, 0x1000);
        e.map(0x011d_d000, 0x1000);
        e.set_global(0x011c_3f2c, 0x0aaa_0000u32);
        e.set_global(0x011d_df38, 0x0bbb_0000u32);
        let this = e.mem.alloc(0x34);
        e.mem.set_u32(this + 0x30, 0x40);
        // Two arrays of form entries: forms are [vtable, kind]; kind 1
        // answers true to slot 0xF0, kind 2 also has flag 0x8.
        e.put_vtable(0x0620_0000, &{
            let mut s = vec![0u32; 0xf4 / 4];
            s[0x88 / 4] = 0x7000_0088;
            s[0xf0 / 4] = 0x7000_00f0;
            s
        });
        e.register(0x7000_00f0, |e, a| ret(u32::from(e.mem.u32(a[0] + 4) == 1)));
        e.register(0x7000_0088, |_, _| Ret::default());
        e.register(0x0040_13e0, |e, a| ret(u32::from(e.mem.u32(a[0] + 4) == 2)));
        let form = |e: &mut Engine, kind: u32| {
            let f = e.mem.alloc(8);
            e.mem.set_u32(f, 0x0620_0000);
            e.mem.set_u32(f + 4, kind);
            f
        };
        let forms = vec![
            form(&mut e, 0),
            form(&mut e, 1),
            form(&mut e, 2),
            form(&mut e, 1),
        ];
        // Entries are the address of a slot holding the form; the count is
        // the list's length (all in the first array).
        let list = Rc::new(RefCell::new(forms.clone()));
        let slots = e.mem.alloc(0x40);
        let l = list.clone();
        e.register_double(ARRAY_COUNT, move |_, a| {
            ret(if a[0] == this + 0x10 {
                l.borrow().len() as u32
            } else {
                0
            })
        });
        let l = list.clone();
        e.register_double(0x0084_32e0, move |e, a| {
            let f = l.borrow()[a[1] as usize];
            e.mem.set_u32(slots + 4 * a[1], f);
            ret(slots + 4 * a[1])
        });
        let l = list.clone();
        e.register_double(0x0084_3380, move |_, a| {
            l.borrow_mut().remove(a[1] as usize);
            Ret::default()
        });
        // Base forms: only the second kind-1 form has one.
        let keep = forms[3];
        e.register_double(0x007a_f430, move |_, a| ret(u32::from(a[0] == keep)));
        quiet(
            &mut e,
            &[
                ARRAY_REALLOCATE,
                ARRAY_DESTROY,
                0x0055_0890,
                0x0084_27f0,
                0x0084_3c90,
            ],
        );
        e.register(0x0045_16b0, |_, _| ret(0x1203));
        e.register(0x0047_39b0, |_, a| ret(a[0] + 0x100));
        map_doubles(&mut e, vec![(0x10, 0x20), (0x11, 0x21)]);
        e.call_log = Some(vec![]);
        e.call(0x0084_39e0, &[this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, ARRAY_REALLOCATE),
            vec![vec![this + 0x10, 0x40, 0], vec![this + 0x20, 0x40, 0]]
        );
        assert_eq!(
            calls_to(&log, 0x0055_0890),
            vec![vec![0x0aaa_0000, 1], vec![0x0aaa_0000, 3]]
        );
        assert_eq!(
            calls_to(&log, 0x0084_27f0),
            vec![
                vec![0x20, 0x110, this + 0x10],
                vec![0x21, 0x111, this + 0x10]
            ]
        );
        // The first kind-1 form (no base form) was removed.
        assert_eq!(*list.borrow(), vec![forms[0], forms[2], forms[3]]);
        assert_eq!(calls_to(&log, 0x0084_3380), vec![vec![this + 0x10, 1]]);
        // Slot 0x88 for the forms without flag 0x8.
        let initialized: Vec<u32> = calls_to(&log, 0x7000_0088).iter().map(|w| w[0]).collect();
        assert_eq!(initialized, vec![forms[0], forms[3]]);
        let finished: Vec<u32> = calls_to(&log, 0x0084_3c90)
            .iter()
            .map(|w| {
                assert_eq!(w[0], this);
                e.mem.u32(w[1])
            })
            .collect();
        assert_eq!(finished, vec![forms[0], forms[2], forms[3]]);
        let tls = e.tls();
        assert_eq!(e.mem.u32(tls + TLS_FLAGS) & 4, 0);
    }
}

#[cfg(test)]
mod la1_tests {
    use super::*;

    /// The forms' name slot (vtable +0x130) as the tests place it.
    const NAME_SLOT: u32 = 0x0090_0130;

    fn calls(log: &[(u32, Vec<u32>)], address: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|(a, _)| *a == address)
            .map(|(_, w)| w.clone())
            .collect()
    }

    /// `known` is the file map's answer for file 0x7000 (0 when the map has none).
    fn rig(file: u32, known: u32) -> (Engine, Ptr, u32) {
        let mut e = Engine::new();
        e.register_double(FORM_GET_FILE, move |_, _| file.into_ret());
        e.register_double(GET_FILE_FOR_FORM_ID, move |_, _| file.into_ret());
        e.register(TYPE_NAME_00440E30, |_, _| 0x0300_0000u32.into_ret());
        e.register(NAME_SLOT, |_, _| 0x0300_1000u32.into_ret());
        e.register(FORM_ID, |_, _| 0x0001_2345u32.into_ret());
        e.register(FORM_KEY_00460140, |_, a| (a[0] ^ 0xff).into_ret());
        e.register(CELL_GET_WORLD_SPACE, |_, _| 0x0c00_0000u32.into_ret());
        e.register_double(FILE_MAP_GET_AT, move |e, a| {
            e.mem.set_u32(a[2], known);
            u32::from(known != 0).into_ret()
        });
        e.register(OPERATOR_NEW, |_, _| 0x0d00_0000u32.into_ret());
        e.register(FILE_MAP_CONSTRUCT, |_, a| a[0].into_ret());
        for callee in [
            SAVE_GAME_WARNING,
            FILE_MAP_SET_AT,
            FILE_MAP_ADD_FORM,
            FILE_MAP_ADD_CELL_FORM,
        ] {
            e.register(callee, |_, _| Ret::default());
        }
        e.put_vtable(0x0090_1000, &[0; 0x4d]);
        e.mem.set_u32(0x0090_1000 + 0x130, NAME_SLOT);
        let form = e.mem.alloc(0x20);
        e.mem.set_u32(form, 0x0090_1000);
        let this = Ptr::new(e.mem.alloc(0x34));
        (e, this, form)
    }

    #[test]
    fn add_form_adds_to_a_new_file_map() {
        let (mut e, this, form) = rig(0x7000, 0);
        e.call_log = Some(vec![]);
        e.call(0x0084_3630, &args![this, form, 9u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls(&log, FORM_GET_FILE), vec![vec![form, u32::MAX]]);
        assert_eq!(
            calls(&log, FILE_MAP_SET_AT),
            vec![vec![this.addr(), 0x7000, 0x0d00_0000]]
        );
        assert_eq!(
            calls(&log, FILE_MAP_ADD_FORM),
            vec![vec![0x0d00_0000, form, 9]]
        );
        assert_eq!(e.mem.u32(this.addr() + ADDED_COUNT), 1);
    }

    #[test]
    fn add_form_uses_the_known_map_or_warns_without_a_file() {
        let (mut e, this, form) = rig(0x7000, 0x0e00_0000);
        e.call_log = Some(vec![]);
        e.call(0x0084_3630, &args![this, form, 9u32]);
        let log = e.call_log.take().unwrap();
        assert!(calls(&log, OPERATOR_NEW).is_empty());
        assert_eq!(
            calls(&log, FILE_MAP_ADD_FORM),
            vec![vec![0x0e00_0000, form, 9]]
        );
        let (mut e, this, form) = rig(0, 0);
        e.call_log = Some(vec![]);
        e.call(0x0084_3630, &args![this, form, 9u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls(&log, SAVE_GAME_WARNING),
            vec![vec![
                NO_FILE_FORM_MESSAGE,
                0x0001_2345,
                0x0300_1000,
                0x0300_0000
            ]]
        );
        assert!(calls(&log, FILE_MAP_ADD_FORM).is_empty());
        assert_eq!(e.mem.u32(this.addr() + ADDED_COUNT), 0);
        // A null form: nothing at all.
        e.call_log = Some(vec![]);
        e.call(0x0084_3630, &args![this, 0u32, 9u32]);
        assert_eq!(e.call_log.take().unwrap().len(), 1);
    }

    #[test]
    fn add_reference_finds_the_file_by_the_form_or_its_world() {
        let (mut e, this, form) = rig(0x7000, 0x0e00_0000);
        e.register(FORM_TEST_00425FD0, |_, _| 1u32.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x0084_3750, &args![this, form, 5u32, 6u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls(&log, GET_FILE_FOR_FORM_ID),
            vec![vec![this.addr(), form ^ 0xff, 5]]
        );
        assert_eq!(
            calls(&log, FILE_MAP_ADD_CELL_FORM),
            vec![vec![0x0e00_0000, form, 0, 5, 6]]
        );
        assert_eq!(e.mem.u32(this.addr() + ADDED_COUNT), 1);
        // Through the world space when the test answers false.
        e.register(FORM_TEST_00425FD0, |_, _| 0x100u32.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x0084_3750, &args![this, form, 5u32, 6u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls(&log, CELL_GET_WORLD_SPACE), vec![vec![form]]);
        assert_eq!(
            calls(&log, GET_FILE_FOR_FORM_ID),
            vec![vec![this.addr(), 0x0c00_00ff, 5]]
        );
        assert_eq!(e.mem.u32(this.addr() + ADDED_COUNT), 2);
    }

    #[test]
    fn add_reference_warns_without_a_file() {
        let (mut e, this, form) = rig(0, 0);
        e.register(FORM_TEST_00425FD0, |_, _| 1u32.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x0084_3750, &args![this, form, 5u32, 6u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls(&log, SAVE_GAME_WARNING),
            vec![vec![NO_FILE_CELL_MESSAGE, 0x0001_2345, 0x0300_1000]]
        );
        assert!(calls(&log, FILE_MAP_ADD_CELL_FORM).is_empty());
    }
}

#[cfg(test)]
mod n088_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    type Entries = Rc<RefCell<Vec<(u32, u32, u32)>>>;

    /// The forms' vtable in these tests and its doubled slots.
    const FORM_VTABLE: u32 = 0x0700_0000;
    const SLOT_KIND: u32 = 0x7100_008c;
    const SLOT_REFERENCE: u32 = 0x7100_00f0;
    const SLOT_NAME: u32 = 0x7100_0130;
    const SLOT_LOCATION: u32 = 0x7100_0138;
    const SLOT_POSITION: u32 = 0x7100_01f4;

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    fn calls(e: &Engine, address: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == address)
            .map(|(_, w)| w.clone())
            .collect()
    }

    fn quiet(e: &mut Engine, addresses: &[u32]) {
        for a in addresses {
            e.register(*a, |_, _| Ret::default());
        }
    }

    /// Doubles for the maps: a shared table of (map, key, value) behind `00853130` and
    /// `00844700`, `operator new` and the allocator from the engine's heap, the form data
    /// helpers, and forms whose type byte is at +4, id at +0xC, vtable +0xF0 answer at +0x20
    /// and `00565450` answer at +0x24.
    fn rig() -> (Engine, Entries) {
        let mut e = Engine::new();
        let entries: Entries = Rc::new(RefCell::new(vec![]));
        let table = entries.clone();
        e.register_double(FILE_MAP_GET_AT, move |e, a| {
            let hit = table
                .borrow()
                .iter()
                .find(|(m, k, _)| *m == a[0] && *k == a[1])
                .map(|(_, _, v)| *v);
            match hit {
                Some(v) => {
                    e.mem.set_u32(a[2], v);
                    ret(1)
                }
                None => ret(0),
            }
        });
        let table = entries.clone();
        e.register_double(FILE_MAP_SET_AT, move |_, a| {
            table.borrow_mut().push((a[0], a[1], a[2]));
            Ret::default()
        });
        e.register(OPERATOR_NEW, |e, a| ret(e.mem.alloc(a[0])));
        e.register(ALLOCATE, |e, a| ret(e.mem.alloc(a[0].max(4))));
        quiet(
            &mut e,
            &[
                MEMSET,
                FORMS_MAP_NITMAP_BASE_CONSTRUCT,
                CELLS_MAP_NITMAP_BASE_CONSTRUCT,
                PAIR_CONSTRUCT,
                FORM_DATA_SET_AT,
            ],
        );
        e.register(FORM_DATA_MAKE, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            e.mem.set_u32(a[0] + 4, a[2]);
            ret(a[0])
        });
        e.register(FORM_ID, |e, a| ret(e.mem.u32(a[0] + 0xc)));
        e.register(FORM_TYPE, |e, a| ret(u32::from(e.mem.u8(a[0] + 4))));
        e.register(SLOT_KIND, |e, a| ret(u32::from(e.mem.u8(a[0] + 4))));
        e.register(SLOT_REFERENCE, |e, a| ret(e.mem.u32(a[0] + 0x20)));
        e.register(FORM_TEST_00565450, |e, a| ret(e.mem.u32(a[0] + 0x24)));
        let mut slots = vec![0u32; 0x100];
        slots[0x8c / 4] = SLOT_KIND;
        slots[0xf0 / 4] = SLOT_REFERENCE;
        slots[0x130 / 4] = SLOT_NAME;
        slots[0x138 / 4] = SLOT_LOCATION;
        slots[0x1f4 / 4] = SLOT_POSITION;
        e.put_vtable(FORM_VTABLE, &slots);
        (e, entries)
    }

    /// A form of `kind` with id `id`; `reference` is its vtable +0xF0 answer.
    fn form(e: &mut Engine, kind: u8, id: u32, reference: bool) -> u32 {
        let f = e.mem.alloc(0x40);
        e.mem.set_u32(f, FORM_VTABLE);
        e.mem.set_u8(f + 4, kind);
        e.mem.set_u32(f + 0xc, id);
        e.mem.set_u32(f + 0x20, u32::from(reference));
        f
    }

    /// The value the table holds for (map, key).
    fn value(entries: &Entries, map: u32, key: u32) -> Option<u32> {
        entries
            .borrow()
            .iter()
            .find(|(m, k, _)| *m == map && *k == key)
            .map(|(_, _, v)| *v)
    }

    #[test]
    fn scrap_array_allocate_and_reallocate() {
        let mut e = Engine::new();
        e.map(0x010a_2000, 0x1000);
        e.set_global(SCRAP_ALIGNMENT, 16u32);
        e.register(SCRAP_HEAP_ALLOCATE, |_, _| ret(0x5000));
        e.register(ARRAY_COUNT, |_, _| ret(3));
        quiet(&mut e, &[MEMORY_COPY_00401460, SCRAP_HEAP_DEALLOCATE]);
        let this = e.mem.alloc(0x14);
        e.mem.set_u32(this + 0x10, 0x0900_0000);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x006d_b9b0, &[this, 5]).u32(), 0x5000);
        assert_eq!(
            calls(&e, SCRAP_HEAP_ALLOCATE),
            vec![vec![0x0900_0000, 40, 16]]
        );
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0084_5050, &[this, 0x6000, 7]).u32(), 0x5000);
        assert_eq!(
            calls(&e, SCRAP_HEAP_ALLOCATE),
            vec![vec![0x0900_0000, 56, 16]]
        );
        assert_eq!(
            calls(&e, MEMORY_COPY_00401460),
            vec![vec![0x5000, 0x6000, 24]]
        );
        assert_eq!(
            calls(&e, SCRAP_HEAP_DEALLOCATE),
            vec![vec![0x0900_0000, 0x6000]]
        );
    }

    #[test]
    fn constructors_store_their_vtables() {
        let (mut e, _) = rig();
        // The base-constructed maps: vtable, buckets, cleared count, zeroed table.
        for (address, vtable) in [
            (0x0084_42a0u32, FORM_DATA_NITMAP_BASE_VTABLE),
            (0x0084_4870, SUB_BLOCKS_MAP_NITMAP_BASE_VTABLE),
            (0x0084_4970, CELL_BLOCKS_MAP_NITMAP_BASE_VTABLE),
        ] {
            let this = e.mem.alloc(0x20);
            e.mem.set_u32(this + 0xc, 9);
            e.call_log = Some(vec![]);
            assert_eq!(e.call(address, &[this, 3]).u32(), this);
            assert_eq!(e.mem.u32(this), vtable);
            assert_eq!((e.mem.u32(this + 4), e.mem.u32(this + 0xc)), (3, 0));
            let table = e.mem.u32(this + 8);
            assert_eq!(calls(&e, ALLOCATE), vec![vec![12]]);
            assert_eq!(calls(&e, MEMSET), vec![vec![table, 0, 12]]);
        }
        // The NiTMap constructors.
        for (address, vtable, base) in [
            (0x0084_4040u32, FORM_DATA_NITMAP_VTABLE, ALLOCATE),
            (
                0x0084_4070,
                FORMS_MAP_NITMAP_VTABLE,
                FORMS_MAP_NITMAP_BASE_CONSTRUCT,
            ),
            (
                0x0084_40a0,
                CELLS_MAP_NITMAP_VTABLE,
                CELLS_MAP_NITMAP_BASE_CONSTRUCT,
            ),
            (0x0084_40d0, SUB_BLOCKS_MAP_NITMAP_VTABLE, ALLOCATE),
            (0x0084_4100, CELL_BLOCKS_MAP_NITMAP_VTABLE, ALLOCATE),
        ] {
            let this = e.mem.alloc(0x20);
            e.call_log = Some(vec![]);
            assert_eq!(e.call(address, &[this, 0x25]).u32(), this);
            assert_eq!(e.mem.u32(this), vtable);
            assert_eq!(calls(&e, base).len(), 1);
        }
        // The classes.
        let this = e.mem.alloc(0x20);
        e.mem.set_u32(this + 0x10, 7);
        assert_eq!(e.call(0x0084_1190, &[this]).u32(), this);
        assert_eq!(
            (e.mem.u32(this), e.mem.u32(this + 4)),
            (FORMS_MAP_VTABLE, 0x25)
        );
        assert_eq!(e.mem.u32(this + 0x10), 0);
        for (address, vtable) in [
            (0x0084_1340u32, CELLS_MAP_VTABLE),
            (0x0084_16c0, SUB_BLOCKS_MAP_VTABLE),
            (0x0084_1ea0, WORLD_SPACES_MAP_VTABLE),
        ] {
            let this = e.mem.alloc(0x20);
            assert_eq!(e.call(address, &[this]).u32(), this);
            assert_eq!(e.mem.u32(this), vtable);
        }
        let this = e.mem.alloc(0x20);
        e.mem.set_u32(this + 0x10, 7);
        assert_eq!(e.call(0x0084_19f0, &[this, 0x101]).u32(), this);
        assert_eq!(e.mem.u32(this), CELL_BLOCKS_MAP_VTABLE);
        assert_eq!((e.mem.u32(this + 0x10), e.mem.u8(this + 0x14)), (0, 1));
        assert_eq!(e.mem.u32(this + 4), 0x25);
        let this = e.mem.alloc(0x20);
        for offset in [0x10, 0x14, 0x18] {
            e.mem.set_u32(this + offset, 5);
        }
        assert_eq!(e.call(0x0084_2180, &[this]).u32(), this);
        assert_eq!(e.mem.u32(this), FORMS_IN_FILE_MAP_VTABLE);
        assert_eq!(e.mem.bytes(this + 0x10, 12), vec![0; 12]);
    }

    #[test]
    fn destructors_store_their_vtables_and_run_their_bases() {
        let (mut e, _) = rig();
        quiet(
            &mut e,
            &[
                MAP_REMOVE_ALL,
                OPERATOR_DELETE,
                FORM_DATA_NITMAP_DESTRUCT,
                FORMS_MAP_NITMAP_DESTRUCT,
                CELLS_MAP_NITMAP_BASE_DESTRUCT,
                SUB_BLOCKS_MAP_NITMAP_BASE_DESTRUCT,
                CELL_BLOCKS_MAP_NITMAP_BASE_DESTRUCT,
                FORM_DATA_NITMAP_BASE_DESTRUCT,
                FORMS_MAP_NITMAP_BASE_DESTRUCT,
                FILE_MAP_NITMAP_BASE_DESTRUCT,
                OFFSET_DATA_ARRAY_BASE_DESTRUCT,
                0x0084_4e80,
                0x0084_54f0,
                0x0084_4ff0,
                0x7000_0000,
            ],
        );
        e.register(MAP_FIRST_POSITION, |_, _| ret(0));
        // Destructors: (address, vtable left at +0, the base destructor called last).
        for (address, vtable, base) in [
            (0x0084_11f0u32, FORMS_MAP_VTABLE, FORM_DATA_NITMAP_DESTRUCT),
            (0x0084_13a0, CELLS_MAP_VTABLE, FORMS_MAP_NITMAP_DESTRUCT),
            (
                0x0084_1720,
                CELLS_MAP_NITMAP_VTABLE,
                CELLS_MAP_NITMAP_BASE_DESTRUCT,
            ),
            (
                0x0084_1a60,
                SUB_BLOCKS_MAP_NITMAP_VTABLE,
                SUB_BLOCKS_MAP_NITMAP_BASE_DESTRUCT,
            ),
            (
                0x0084_1f00,
                CELL_BLOCKS_MAP_NITMAP_VTABLE,
                CELL_BLOCKS_MAP_NITMAP_BASE_DESTRUCT,
            ),
            (
                0x0084_21f0,
                FORMS_IN_FILE_MAP_VTABLE,
                FORMS_MAP_NITMAP_DESTRUCT,
            ),
            (
                0x0084_47e0,
                CELLS_MAP_NITMAP_VTABLE,
                CELLS_MAP_NITMAP_BASE_DESTRUCT,
            ),
            (
                0x0084_48e0,
                SUB_BLOCKS_MAP_NITMAP_VTABLE,
                SUB_BLOCKS_MAP_NITMAP_BASE_DESTRUCT,
            ),
            (
                0x0084_49e0,
                CELL_BLOCKS_MAP_NITMAP_VTABLE,
                CELL_BLOCKS_MAP_NITMAP_BASE_DESTRUCT,
            ),
        ] {
            let this = e.mem.alloc(0x20);
            e.call_log = Some(vec![]);
            e.call(address, &[this]);
            assert_eq!(e.mem.u32(this), vtable, "{address:08x}");
            let log = e.call_log.take().unwrap();
            assert_eq!(log.last().unwrap(), &(base, vec![this]), "{address:08x}");
        }
        // The deleting destructors: (address, a call their destructor makes).
        for (address, callee) in [
            (0x0084_11c0u32, FORM_DATA_NITMAP_DESTRUCT),
            (0x0084_1370, FORMS_MAP_NITMAP_DESTRUCT),
            (0x0084_16f0, CELLS_MAP_NITMAP_BASE_DESTRUCT),
            (0x0084_1a30, SUB_BLOCKS_MAP_NITMAP_BASE_DESTRUCT),
            (0x0084_1ed0, CELL_BLOCKS_MAP_NITMAP_BASE_DESTRUCT),
            (0x0084_21c0, FORMS_MAP_NITMAP_DESTRUCT),
            (0x0084_4180, FORM_DATA_NITMAP_DESTRUCT),
            (0x0084_41b0, FORMS_MAP_NITMAP_DESTRUCT),
            (0x0084_41e0, CELLS_MAP_NITMAP_BASE_DESTRUCT),
            (0x0084_4210, SUB_BLOCKS_MAP_NITMAP_BASE_DESTRUCT),
            (0x0084_4240, CELL_BLOCKS_MAP_NITMAP_BASE_DESTRUCT),
            (0x0084_4270, 0x0084_4e80),
            (0x0084_50b0, FORM_DATA_NITMAP_BASE_DESTRUCT),
            (0x0084_50e0, FORMS_MAP_NITMAP_BASE_DESTRUCT),
            (0x0084_5110, CELLS_MAP_NITMAP_BASE_DESTRUCT),
            (0x0084_5140, SUB_BLOCKS_MAP_NITMAP_BASE_DESTRUCT),
            (0x0084_5170, CELL_BLOCKS_MAP_NITMAP_BASE_DESTRUCT),
            (0x0084_51a0, 0x0084_54f0),
            (0x0084_51d0, FILE_MAP_NITMAP_BASE_DESTRUCT),
            (0x0084_5200, OFFSET_DATA_ARRAY_BASE_DESTRUCT),
            (0x0084_31f0, 0x0084_4ff0),
        ] {
            for (flags, frees) in [(0u32, 0usize), (3, 1)] {
                let this = e.mem.alloc(0x20);
                e.call_log = Some(vec![]);
                assert_eq!(e.call(address, &[this, flags]).u32(), this);
                assert_eq!(calls(&e, callee).len(), 1, "{address:08x}");
                assert_eq!(calls(&e, OPERATOR_DELETE).len(), frees, "{address:08x}");
            }
        }
        // The map destructors delete every value, and the file map its two members.
        let value = e.mem.alloc(4);
        e.put_vtable(0x0710_0000, &[0x7000_0000]);
        e.mem.set_u32(value, 0x0710_0000);
        e.register(MAP_FIRST_POSITION, |_, _| ret(1));
        e.register_double(MAP_NEXT, move |e, a| {
            e.mem.set_u32(a[1], 0);
            e.mem.set_u32(a[3], value);
            Ret::default()
        });
        let this = e.mem.alloc(0x20);
        e.mem.set_u32(this + 0x10, value);
        e.mem.set_u32(this + 0x14, value);
        e.call_log = Some(vec![]);
        e.call(0x0084_21f0, &[this]);
        assert_eq!(calls(&e, 0x7000_0000), vec![vec![value, 1]; 3]);
    }

    #[test]
    fn forms_map_add_form_keys_and_flags() {
        let (mut e, _) = rig();
        let this = e.mem.alloc(0x14);
        let check = |e: &mut Engine, form: u32, flags: u32, key: u32| {
            e.mem.set_u32(this + 0x10, 0x100);
            e.call_log = Some(vec![]);
            e.call(0x0084_1250, &[this, form, 0x77, 9]);
            assert_eq!(e.mem.u32(this + 0x10), 0x100 | flags);
            let set = calls(e, FORM_DATA_SET_AT);
            assert_eq!(set, vec![vec![this, key, form, 9]]);
        };
        check(&mut e, 0, 6, 0x77);
        let reference = form(&mut e, 0x3a, 0x1234, true);
        e.mem.set_u32(reference + 0x24, 1);
        check(&mut e, reference, 2, 0x1234);
        e.mem.set_u32(reference + 0x24, 0x100);
        check(&mut e, reference, 4, 0x1234);
        let typed = form(&mut e, 0x43, 0x55, false);
        check(&mut e, typed, 4, 0x55);
        let other = form(&mut e, 0x20, 0x56, false);
        check(&mut e, other, 0, 0x56);
        // 00841320 alone.
        e.mem.set_u32(this + 0x10, 1);
        e.call(0x0084_1320, &[this, 0x30]);
        assert_eq!(e.mem.u32(this + 0x10), 0x31);
    }

    #[test]
    fn cell_maps_find_or_make_their_children() {
        let (mut e, entries) = rig();
        e.register(FORM_TEST_00425FD0, |e, a| ret(e.mem.u32(a[0] + 0x28)));
        e.register(CELL_GET_DATA_X, |_, _| ret(3));
        e.register(CELL_GET_DATA_Y, |_, _| ret(4));
        e.register(CELL_COORDINATE_KEY, |_, a| ret(a[0] << 16 | a[1]));
        e.register(CELL_SUB_BLOCK_KEY, |_, _| ret(0x51));
        e.register(EXTERIOR_SUB_BLOCK_KEY, |_, a| ret(0x5200 + a[0] + a[1]));
        let interior = form(&mut e, 0x39, 0x0100_0001, false);
        e.mem.set_u32(interior + 0x28, 1);
        let exterior = form(&mut e, 0x39, 0x0100_0002, false);
        let item = form(&mut e, 0x20, 0x0100_0003, false);
        // 00841460: a new forms map per key; the cell itself sets 1 and the word.
        let cells = e.mem.alloc(0x10);
        e.call(0x0084_1460, &[cells, interior, item, 0, 9]);
        let map = value(&entries, cells, 0x0100_0001).unwrap();
        assert_eq!(e.mem.u32(map), FORMS_MAP_VTABLE);
        e.call(0x0084_1460, &[cells, interior, interior, 0, 0x80]);
        assert_eq!(e.mem.u32(map + 0x10), 0x81);
        e.call_log = Some(vec![]);
        e.call(0x0084_1460, &[cells, exterior, item, 0, 9]);
        let order: Vec<u32> = e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .map(|c| c.0)
            .filter(|a| *a == CELL_GET_DATA_X || *a == CELL_GET_DATA_Y)
            .collect();
        assert_eq!(order, vec![CELL_GET_DATA_Y, CELL_GET_DATA_X]);
        assert!(value(&entries, cells, 0x0003_0004).is_some());
        // 00841580: by the low 16 bits of x and y.
        e.call_log = Some(vec![]);
        e.call(
            0x0084_1580,
            &[cells, 0xdead, 0x0001_0005, 0xffff_fffe, item, 0, 9],
        );
        assert_eq!(calls(&e, CELL_COORDINATE_KEY), vec![vec![5, 0xfffe]]);
        // 008417e0 and 008418b0: a cells map, then the add into it.
        let sub_blocks = e.mem.alloc(0x10);
        e.call(0x0084_17e0, &[sub_blocks, interior, item, 0, 9]);
        let cells = value(&entries, sub_blocks, 0x51).unwrap();
        assert_eq!(e.mem.u32(cells), CELLS_MAP_VTABLE);
        assert!(value(&entries, cells, 0x0100_0001).is_some());
        e.call(0x0084_18b0, &[sub_blocks, 0x0d00_0000, 2, 3, item, 0, 9]);
        let cells = value(&entries, sub_blocks, 0x5205).unwrap();
        assert!(value(&entries, cells, 0x0002_0003).is_some());
        // Known children are used as they are.
        let known = e.mem.alloc(0x10);
        let forms = e.mem.alloc(0x14);
        entries.borrow_mut().push((sub_blocks, 0x5207, known));
        entries.borrow_mut().push((known, 0x0003_0004, forms));
        e.call_log = Some(vec![]);
        e.call(0x0084_18b0, &[sub_blocks, 0, 3, 4, item, 0, 9]);
        assert!(calls(&e, OPERATOR_NEW).is_empty());
        assert_eq!(calls(&e, FORM_DATA_SET_AT)[0][0], forms);
    }

    #[test]
    fn queries_look_through_the_children() {
        let (mut e, _) = rig();
        // Each map's values: maps 0x10..; the flags test answers for map 0x22 with mask 4.
        let values: Rc<Vec<(u32, Vec<u32>)>> = Rc::new(vec![
            (0x10, vec![0x20, 0x21]),
            (0x11, vec![0x22]),
            (0x30, vec![0x10, 0x11]),
            (0x40, vec![0x30]),
        ]);
        let v = values.clone();
        e.register_double(MAP_FIRST_POSITION, move |_, a| {
            ret(u32::from(
                v.iter().any(|(m, l)| *m == a[0] && !l.is_empty()),
            ))
        });
        let v = values.clone();
        e.register_double(MAP_NEXT, move |e, a| {
            let list = &v.iter().find(|(m, _)| *m == a[0]).unwrap().1;
            let position = e.mem.u32(a[1]) as usize;
            e.mem.set_u32(a[3], list[position - 1]);
            let next = if position < list.len() {
                position + 1
            } else {
                0
            };
            e.mem.set_u32(a[1], next as u32);
            Ret::default()
        });
        e.register(FORMS_MAP_TEST_FLAGS, |_, a| {
            ret(u32::from(a[0] == 0x22 && a[1] == 4))
        });
        assert!(!e.call(0x0084_1660, &[0x10, 4]).bool());
        assert!(e.call(0x0084_1660, &[0x11, 4]).bool());
        assert!(e.call(0x0084_1990, &[0x30, 4]).bool());
        assert!(!e.call(0x0084_1990, &[0x30, 2]).bool());
        // 00841e20: its own map first, then the sub-blocks maps.
        let blocks = e.mem.alloc(0x18);
        e.mem.set_u32(blocks + 0x10, 0x22);
        assert!(e.call(0x0084_1e20, &[blocks, 4]).bool());
        e.mem.set_u32(blocks + 0x10, 0);
        assert!(!e.call(0x0084_1e20, &[blocks, 4]).bool());
    }

    #[test]
    fn cell_blocks_maps_add_persistent_forms_to_their_own_map() {
        let (mut e, entries) = rig();
        e.register(FORM_TEST_00425FD0, |_, _| ret(1));
        e.register(CELL_BLOCK_KEY, |_, _| ret(0x61));
        e.register(CELL_SUB_BLOCK_KEY, |_, _| ret(0x62));
        e.register(EXTERIOR_BLOCK_KEY, |_, a| ret(0x6300 + a[0]));
        e.register(EXTERIOR_SUB_BLOCK_KEY, |_, _| ret(0x64));
        e.register(CELL_COORDINATE_KEY, |_, _| ret(0x65));
        let cell = form(&mut e, 0x39, 0x0100_0001, false);
        let persistent = form(&mut e, 0x3a, 0x0100_0002, true);
        e.mem.set_u32(persistent + 0x24, 1);
        let plain = form(&mut e, 0x3a, 0x0100_0003, true);
        // With the byte set: the persistent reference goes to +0x10, made once.
        let blocks = e.mem.alloc(0x18);
        e.mem.set_u8(blocks + 0x14, 1);
        e.call(0x0084_1b20, &[blocks, cell, persistent, 0, 9]);
        let own = e.mem.u32(blocks + 0x10);
        assert_eq!(e.mem.u32(own), FORMS_MAP_VTABLE);
        e.call(0x0084_1b20, &[blocks, cell, persistent, 0, 9]);
        assert_eq!(e.mem.u32(blocks + 0x10), own);
        assert!(entries.borrow().is_empty());
        // Others (or without the byte) go down the block map.
        e.call(0x0084_1b20, &[blocks, cell, plain, 0, 9]);
        let sub_blocks = value(&entries, blocks, 0x61).unwrap();
        assert_eq!(e.mem.u32(sub_blocks), SUB_BLOCKS_MAP_VTABLE);
        let cells = value(&entries, sub_blocks, 0x62).unwrap();
        assert!(value(&entries, cells, 0x0100_0001).is_some());
        let other = e.mem.alloc(0x18);
        e.call(0x0084_1b20, &[other, cell, persistent, 0, 9]);
        assert_eq!(e.mem.u32(other + 0x10), 0);
        assert!(value(&entries, other, 0x61).is_some());
        // The coordinate overload does not read the byte.
        let exterior = e.mem.alloc(0x18);
        e.call(
            0x0084_1ca0,
            &[exterior, 0x0d00_0000, 2, 3, persistent, 0, 9],
        );
        assert_ne!(e.mem.u32(exterior + 0x10), 0);
        e.call(0x0084_1ca0, &[exterior, 0x0d00_0000, 2, 3, 0, 0x55, 9]);
        let sub_blocks = value(&entries, exterior, 0x6302).unwrap();
        let cells = value(&entries, sub_blocks, 0x64).unwrap();
        assert!(value(&entries, cells, 0x65).is_some());
    }

    #[test]
    fn world_spaces_and_file_maps_add_cell_forms() {
        let (mut e, entries) = rig();
        let world = form(&mut e, 0x41, 0x0100_0010, false);
        e.register_double(CELL_GET_WORLD_SPACE, move |_, _| ret(world));
        e.register(FORM_TEST_00425FD0, |e, a| ret(e.mem.u32(a[0] + 0x28)));
        e.register(CELL_BLOCK_KEY, |_, _| ret(0x61));
        e.register(CELL_SUB_BLOCK_KEY, |_, _| ret(0x62));
        e.register(EXTERIOR_BLOCK_KEY, |_, _| ret(0x63));
        e.register(EXTERIOR_SUB_BLOCK_KEY, |_, _| ret(0x64));
        e.register(CELL_COORDINATE_KEY, |_, _| ret(0x65));
        e.register(CELL_GET_DATA_X, |_, _| ret(1));
        e.register(CELL_GET_DATA_Y, |_, _| ret(2));
        let cell = form(&mut e, 0x39, 0x0100_0001, false);
        let item = form(&mut e, 0x20, 0x0100_0003, false);
        // 00841fc0 / 008420a0: an exterior cell blocks map per world space.
        let worlds = e.mem.alloc(0x10);
        e.call(0x0084_1fc0, &[worlds, cell, item, 0, 9]);
        let blocks = value(&entries, worlds, 0x0100_0010).unwrap();
        assert_eq!(e.mem.u32(blocks), CELL_BLOCKS_MAP_VTABLE);
        assert_eq!(e.mem.u8(blocks + 0x14), 1);
        assert!(value(&entries, blocks, 0x61).is_some());
        e.call(0x0084_20a0, &[worlds, world, 1, 2, item, 0, 9]);
        assert!(value(&entries, blocks, 0x63).is_some());
        // 00842600: interior cells into +0x10 (byte 0), others through +0x14; counts.
        let file_map = e.mem.alloc(0x1c);
        e.mem.set_u32(cell + 0x28, 1);
        e.call(0x0084_2600, &[file_map, cell, item, 0, 9]);
        let interior = e.mem.u32(file_map + 0x10);
        assert_eq!(
            (e.mem.u32(interior), e.mem.u8(interior + 0x14)),
            (CELL_BLOCKS_MAP_VTABLE, 0)
        );
        e.mem.set_u32(cell + 0x28, 0);
        e.call(0x0084_2600, &[file_map, cell, item, 0, 9]);
        let worlds = e.mem.u32(file_map + 0x14);
        assert_eq!(e.mem.u32(worlds), WORLD_SPACES_MAP_VTABLE);
        assert!(value(&entries, worlds, 0x0100_0010).is_some());
        e.call(0x0084_2730, &[file_map, world, 1, 2, item, 0, 9]);
        assert_eq!(e.mem.u32(file_map + 0x14), worlds);
        assert_eq!(e.mem.u32(file_map + 0x18), 3);
    }

    #[test]
    fn file_map_add_form_places_each_kind() {
        let (mut e, entries) = rig();
        e.map(0x0118_7000, 0x1000);
        e.set_global(TAG_BY_FORM_TYPE + 0xc * 0x20, 0x4f42_4a53u32);
        let world = form(&mut e, 0x41, 0x0100_0010, false);
        let cell = form(&mut e, 0x39, 0x0100_0001, false);
        e.register_double(CELL_GET_WORLD_SPACE, move |e, a| {
            ret(e.mem.u32(a[0] + 0x2c))
        });
        e.register(FORM_TEST_00425FD0, |_, _| ret(1));
        e.register(CELL_GET_DATA_X, |_, _| ret(1));
        e.register(CELL_GET_DATA_Y, |_, _| ret(2));
        for key in [
            CELL_BLOCK_KEY,
            CELL_SUB_BLOCK_KEY,
            EXTERIOR_BLOCK_KEY,
            EXTERIOR_SUB_BLOCK_KEY,
            CELL_COORDINATE_KEY,
        ] {
            e.register(key, |_, _| ret(0x70));
        }
        e.register(RETURNS_THIS_006815C0, |_, a| ret(a[0]));
        e.register(FORM_CELL_0059BB30, |e, a| ret(e.mem.u32(a[0] + 0x30)));
        e.register(FLOAT_TO_INT_00406D90, |_, a| {
            ret(f32::from_bits(a[0]) as i32 as u32)
        });
        let reference = form(&mut e, 0x3a, 0x0100_0002, true);
        let file_map = e.mem.alloc(0x1c);
        // Without a location from vtable +0x138: the world space and its position.
        e.register(SLOT_LOCATION, |_, _| ret(0));
        e.register_double(REFR_GET_WORLD_SPACE, move |_, _| ret(world));
        let position = e.mem.alloc(12);
        e.mem.set_u32(position, 8192.0f32.to_bits());
        e.mem.set_u32(position + 4, (-4096.5f32).to_bits());
        e.register_double(SLOT_POSITION, move |_, _| ret(position));
        e.call_log = Some(vec![]);
        e.call(0x0084_2310, &[file_map, reference, 9]);
        assert_eq!(
            calls(&e, FLOAT_TO_INT_00406D90),
            vec![vec![8192.0f32.to_bits()], vec![(-4096.5f32).to_bits()]]
        );
        assert_eq!(e.mem.u32(file_map + 0x18), 1);
        assert_ne!(e.mem.u32(file_map + 0x14), 0);
        // Without a world space: the cell from 008d6f30.
        e.register(REFR_GET_WORLD_SPACE, |_, _| ret(0));
        e.register_double(REFR_PARENT_CELL_008D6F30, move |_, _| ret(cell));
        e.call(0x0084_2310, &[file_map, reference, 9]);
        assert_ne!(e.mem.u32(file_map + 0x10), 0);
        assert_eq!(e.mem.u32(file_map + 0x18), 2);
        // With a location: GetLocationCellOrWorld fills the cell; neither: nothing.
        e.register(SLOT_LOCATION, |e, a| {
            e.mem.set_u32(a[3], 0x99);
            ret(1)
        });
        e.register_double(GET_LOCATION_CELL_OR_WORLD, move |e, a| {
            assert_eq!(a[0], 0x99);
            e.mem.set_u32(a[1], cell);
            Ret::default()
        });
        e.call(0x0084_2310, &[file_map, reference, 9]);
        assert_eq!(e.mem.u32(file_map + 0x18), 3);
        e.register(GET_LOCATION_CELL_OR_WORLD, |_, _| Ret::default());
        e.call(0x0084_2310, &[file_map, reference, 9]);
        assert_eq!(e.mem.u32(file_map + 0x18), 3);
        // Type 0x43: through its cell's world space, or the cell alone.
        let typed = form(&mut e, 0x43, 0x0100_0004, false);
        e.mem.set_u32(typed + 0x30, cell);
        e.mem.set_u32(cell + 0x2c, world);
        e.call_log = Some(vec![]);
        e.call(0x0084_2310, &[file_map, typed, 9]);
        assert_eq!(calls(&e, CELL_GET_DATA_X).len(), 1);
        assert_eq!(e.mem.u32(file_map + 0x18), 4);
        e.mem.set_u32(cell + 0x2c, 0);
        e.call(0x0084_2310, &[file_map, typed, 9]);
        assert_eq!(e.mem.u32(file_map + 0x18), 5);
        e.mem.set_u32(typed + 0x30, 0);
        e.call(0x0084_2310, &[file_map, typed, 9]);
        assert_eq!(e.mem.u32(file_map + 0x18), 5);
        // A cell is its own cell; a world space is left out.
        e.call(0x0084_2310, &[file_map, cell, 9]);
        assert_eq!(e.mem.u32(file_map + 0x18), 6);
        e.call(0x0084_2310, &[file_map, world, 9]);
        assert_eq!(e.mem.u32(file_map + 0x18), 6);
        // Other types: the forms map of the type's tag.
        let other = form(&mut e, 0x20, 0x0100_0005, false);
        e.call(0x0084_2310, &[file_map, other, 9]);
        let map = value(&entries, file_map, 0x4f42_4a53).unwrap();
        assert_eq!(e.mem.u32(map), FORMS_MAP_VTABLE);
        assert_eq!(e.mem.u32(file_map + 0x18), 7);
    }

    /// A record stream for `ReconstructForms`: each record is (tag, size, label, kind,
    /// offset word); `TESFile::NextForm` and the group skip step to the next one,
    /// `TESFile::SetOffset` jumps to the record listed for an offset; past the end the header
    /// holds zeros.
    fn stream(e: &mut Engine, records: Vec<[u32; 5]>, jumps: Vec<(u32, usize)>) -> u32 {
        let header = e.mem.alloc(0x14);
        let records = Rc::new(records);
        let at = Rc::new(RefCell::new(0usize));
        let show = {
            let records = records.clone();
            move |e: &mut Engine, index: usize| {
                let words = records.get(index).copied().unwrap_or([0; 5]);
                for (i, w) in words.iter().enumerate() {
                    e.mem.set_u32(header + 4 * i as u32, *w);
                }
            }
        };
        show(e, 0);
        e.register_double(FILE_CURRENT_FORM, move |_, _| ret(header));
        for address in [FILE_NEXT_FORM, FILE_SKIP_GROUP] {
            let at = at.clone();
            let show = show.clone();
            e.register_double(address, move |e, _| {
                let next = *at.borrow() + 1;
                *at.borrow_mut() = next;
                show(e, next);
                Ret::default()
            });
        }
        let at = at.clone();
        e.register_double(FILE_SET_OFFSET, move |e, a| {
            let index = jumps.iter().find(|(o, _)| *o == a[1]).unwrap().1;
            *at.borrow_mut() = index;
            show(e, index);
            Ret::default()
        });
        header
    }

    /// The tags, the data handler and `ReconstructForms`' quiet callees.
    fn reconstruct_rig() -> (Engine, Entries) {
        let (mut e, entries) = rig();
        e.map(0x0118_7000, 0x1000);
        e.map(0x011c_3000, 0x1000);
        for (tag, word) in [
            (TAG_TES4, 0x5434_4553u32),
            (TAG_GRUP, 0x5055_5247),
            (TAG_CELL, 0x4c4c_4543),
            (TAG_WRLD, 0x444c_5257),
            (TAG_TOFT, 0x5446_4f54),
        ] {
            e.set_global(tag, word);
        }
        e.set_global(DATA_HANDLER, 0x0aaa_0000u32);
        quiet(
            &mut e,
            &[
                FILE_REWIND,
                HANDLER_LOAD_RECORD,
                MAP_INITIALIZER,
                0x0084_4ff0,
                ARRAY_CONSTRUCT,
                SAVE_GAME_WARNING,
            ],
        );
        e.register(FILE_GET_OPTIMIZED, |_, _| ret(0));
        e.register(FILE_POSITION, |_, _| ret(0x100));
        e.register(PAIR_IS_SET, |e, a| ret(u32::from(e.mem.u32(a[0]) != 0)));
        e.register(MEMORY_MANAGER, |_, _| ret(0x0bbb_0000));
        e.register(THREAD_SCRAP_HEAP, |_, _| ret(0x0ccc_0000));
        // The arrays' storage: a 0x40-element buffer at +4 from the base constructor, the next
        // index from the size at +8.
        e.register(SCRAP_ARRAY_BASE_CONSTRUCT, |e, a| {
            let buffer = e.mem.alloc(0x200);
            e.mem.set_u32(a[0] + 4, buffer);
            Ret::default()
        });
        e.register(ARRAY_NEXT_INDEX, |e, a| {
            let size = e.mem.u32(a[0] + 8);
            e.mem.set_u32(a[0] + 8, size + 1);
            ret(size)
        });
        e.register(ARRAY_COUNT, |e, a| ret(e.mem.u32(a[0] + 8)));
        e.register(ARRAY_ELEMENT, |e, a| ret(e.mem.u32(a[0] + 4) + 8 * a[1]));
        (e, entries)
    }

    /// Two empty arrays (buffer `+4`, size `+8`) as `BGSReconstructFormsInAllFilesMap` holds them.
    fn arrays(e: &mut Engine) -> u32 {
        let arrays = e.mem.alloc(0x20);
        let first = e.mem.alloc(0x100);
        let second = e.mem.alloc(0x100);
        e.mem.set_u32(arrays + 4, first);
        e.mem.set_u32(arrays + 0x14, second);
        arrays
    }

    #[test]
    fn reconstruct_forms_walks_an_interior_cell() {
        let (mut e, entries) = reconstruct_rig();
        let grup = 0x5055_5247;
        let cell_tag = 0x4c4c_4543;
        let header = stream(
            &mut e,
            vec![
                [0x5434_4553, 0, 0, 0, 0],
                [grup, 0x100, cell_tag, 0, 0],
                [grup, 0x80, 7, 2, 0],
                [grup, 0x60, 8, 3, 0],
                [cell_tag, 0x20, 0, 0x1234, 0],
                [grup, 0x40, 0x1234, 6, 0],
                [0x5246_4552, 0x10, 0, 0x55, 0],
                [0x5246_4552, 0x10, 0, 0x56, 0],
            ],
            vec![],
        );
        let file = 0x0f00_0000;
        let this = e.mem.alloc(0x1c);
        e.mem.set_u32(this + 0x18, 2);
        let blocks = e.mem.alloc(0x18);
        e.mem.set_u32(this + 0x10, blocks);
        let forms = e.mem.alloc(0x14);
        {
            let mut table = entries.borrow_mut();
            table.push((blocks, 7, 0x0e00_0000));
            table.push((0x0e00_0000, 8, 0x0e10_0000));
            table.push((0x0e10_0000, 0x1234, forms));
        }
        e.register_double(FORMS_MAP_TEST_FLAGS, move |_, a| {
            assert_eq!(a[0], forms);
            ret(u32::from(a[1] == 1 || a[1] == 2))
        });
        let cell = form(&mut e, 0x39, 0x1234, false);
        let reference = form(&mut e, 0x20, 0x55, false);
        e.register_double(FORM_BY_ID, move |_, a| {
            ret(if a[0] == 0x1234 { cell } else { 0 })
        });
        e.register(WORD_GETTER_0044EDB0, |_, _| ret(0x10));
        e.register_double(FORM_DATA_GET_AT, move |e, a| {
            assert_eq!((a[0], a[1]), (forms, 0x55));
            e.mem.set_u32(a[2], reference);
            e.mem.set_u32(a[2] + 4, 3);
            ret(1)
        });
        let arrays = arrays(&mut e);
        e.call_log = Some(vec![]);
        e.call(0x0084_27f0, &[this, file, arrays]);
        assert_eq!(
            calls(&e, HANDLER_LOAD_RECORD),
            vec![
                vec![0x0aaa_0000, file, 0, 0, 0],
                vec![0x0aaa_0000, file, 0, reference, 1]
            ]
        );
        // The cell went to the second array with 0044edb0's word, the reference to the first.
        let second = e.mem.u32(arrays + 0x14);
        assert_eq!((e.mem.u32(second), e.mem.u32(second + 4)), (cell, 0x10));
        let first = e.mem.u32(arrays + 4);
        assert_eq!((e.mem.u32(first), e.mem.u32(first + 4)), (reference, 3));
        assert_eq!(e.mem.u32(forms + 0x10), 0x10000);
        assert_eq!(calls(&e, FORM_BY_ID), vec![vec![0x1234], vec![0x1234]]);
        assert_eq!(e.global::<u32>(RECONSTRUCT_CELL), 0);
        assert_eq!(calls(&e, 0x0084_4ff0).len(), 1);
        assert!(calls(&e, FILE_SKIP_GROUP).is_empty());
        assert_eq!(e.mem.u32(header), 0x5246_4552, "stopped after two forms");
        // Nothing at all without forms to reconstruct.
        e.mem.set_u32(this + 0x18, 0);
        e.call_log = Some(vec![]);
        e.call(0x0084_27f0, &[this, file, arrays]);
        assert_eq!(e.call_log.take().unwrap().len(), 1);
    }

    #[test]
    fn reconstruct_forms_stores_and_follows_group_offsets() {
        let (mut e, entries) = reconstruct_rig();
        e.register(FILE_GET_OPTIMIZED, |_, _| ret(1));
        let grup = 0x5055_5247;
        let world_tag = 0x444c_5257;
        let header = stream(
            &mut e,
            vec![
                [grup, 0x100, world_tag, 0, 0],
                [grup, 0x80, 0x3c, 1, 0x9000],
                [0x5446_4f54, 0, 0, u32::MAX, 0],
                [grup, 0x200, 0x77, 7, 0],
                [grup, 0x40, 0x3c, 1, 0],
            ],
            vec![(0x9000, 4)],
        );
        let file = 0x0f00_0000;
        let this = e.mem.alloc(0x1c);
        e.mem.set_u32(this + 0x18, 5);
        let worlds = e.mem.alloc(0x10);
        e.mem.set_u32(this + 0x14, worlds);
        let blocks = e.mem.alloc(0x18);
        e.mem.set_u32(blocks + 0x10, 0x0e20_0000);
        entries.borrow_mut().push((worlds, 0x0100_003c, blocks));
        e.register(ADD_COMPILE_INDEX, |e, a| {
            let id = e.mem.u32(a[0]);
            e.mem.set_u32(a[0], id | 0x0100_0000);
            Ret::default()
        });
        e.register(FORMS_MAP_TEST_FLAGS, |_, a| {
            ret(u32::from(a[0] == 0x0e20_0000 && a[1] == 5))
        });
        let arrays = arrays(&mut e);
        e.call_log = Some(vec![]);
        e.call(0x0084_27f0, &[this, file, arrays]);
        assert_eq!(calls(&e, ADD_COMPILE_INDEX).len(), 1);
        // The world space's group offset was stored, taken at the TOFT, and jumped to.
        assert_eq!(calls(&e, FILE_SET_OFFSET), vec![vec![file, 0x9000]]);
        // There the world space is looked up by its plain label (not found here).
        let lookups: Vec<u32> = calls(&e, FILE_MAP_GET_AT)
            .iter()
            .filter(|w| w[0] == worlds)
            .map(|w| w[1])
            .collect();
        assert_eq!(lookups, vec![0x0100_003c, 0x3c]);
        assert_eq!(e.mem.u32(header), 0);
        assert_eq!(calls(&e, FILE_SKIP_GROUP).len(), 0);
    }

    #[test]
    fn array_helpers() {
        let (mut e, _) = reconstruct_rig();
        // 00844f00 with and without a heap; 00843190 passes 0x20, 0, 0.
        let this = e.mem.alloc(0x14);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0084_4f00, &[this, 5, 6, 0x0ddd_0000]).u32(), this);
        assert_eq!(e.mem.u32(this + 0x10), 0x0ddd_0000);
        assert_eq!(e.mem.u32(this), SCRAP_ARRAY_VTABLE);
        assert_eq!(calls(&e, MAP_INITIALIZER), vec![vec![this, 5, 6]]);
        assert!(calls(&e, THREAD_SCRAP_HEAP).is_empty());
        let offsets = e.mem.alloc(0x14);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0084_3190, &[offsets]).u32(), offsets);
        assert_eq!(e.mem.u32(offsets), OFFSET_DATA_ARRAY_VTABLE);
        assert_eq!(e.mem.u32(offsets + 0x10), 0x0ccc_0000);
        assert_eq!(calls(&e, THREAD_SCRAP_HEAP), vec![vec![0x0bbb_0000]]);
        assert_eq!(calls(&e, MAP_INITIALIZER), vec![vec![offsets, 0x20, 0]]);
        // 008431c0 / 00844eb0 append and answer the index.
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0084_31c0, &[offsets, 0x9000, 0x44]).u32(), 0);
        assert_eq!(e.call(0x0084_31c0, &[offsets, 0x9100, 0x45]).u32(), 1);
        let buffer = e.mem.u32(offsets + 4);
        assert_eq!(e.mem.u32(buffer + 8), 0x9100);
        assert_eq!(e.mem.u32(buffer + 0xc), 0x45);
        assert_eq!(calls(&e, ARRAY_CONSTRUCT)[1], vec![offsets, buffer + 8, 1]);
        // 00844160 passes 006950d0's answer on.
        assert_eq!(e.call(0x0084_4160, &[offsets, 1]).u32(), buffer + 8);
        // 00843270 picks the array by the form.
        let arrays = arrays(&mut e);
        let plain = form(&mut e, 0x20, 1, false);
        let cell = form(&mut e, 0x39, 2, false);
        let world = form(&mut e, 0x41, 3, false);
        let reference = form(&mut e, 0x3a, 4, true);
        let entry = e.mem.alloc(8);
        for (f, index) in [
            (plain, 0u32),
            (cell, 0),
            (world, 1),
            (reference, 2),
            (plain, 1),
        ] {
            e.mem.set_u32(entry, f);
            e.mem.set_u32(entry + 4, 7);
            assert_eq!(e.call(0x0084_3270, &[arrays, entry]).u32(), index);
        }
        assert_eq!((e.mem.u32(arrays + 8), e.mem.u32(arrays + 0x18)), (2, 3));
        // 008432e0 / 00843330 / 00843380 index both arrays together.
        let first = e.mem.u32(arrays + 4);
        let second = e.mem.u32(arrays + 0x14);
        assert_eq!(e.call(0x0084_32e0, &[arrays, 1]).u32(), first + 8);
        assert_eq!(e.call(0x0084_32e0, &[arrays, 3]).u32(), second + 8);
        assert_eq!(e.call(0x0084_3330, &[arrays, 0]).u32(), plain);
        assert_eq!(e.call(0x0084_3330, &[arrays, 4]).u32(), reference);
        quiet(&mut e, &[ARRAY_REMOVE]);
        e.call_log = Some(vec![]);
        assert!(e.call(0x0084_3380, &[arrays, 1]).bool());
        assert!(e.call(0x0084_3380, &[arrays, 4]).bool());
        assert_eq!(
            calls(&e, ARRAY_REMOVE),
            vec![vec![arrays, 1, 1], vec![arrays + 0x10, 2, 1]]
        );
    }

    #[test]
    fn add_reference_ov2_and_the_file_of_a_form_id() {
        let (mut e, entries) = rig();
        e.map(0x011c_3000, 0x1000);
        e.set_global(DATA_HANDLER, 0x0aaa_0000u32);
        // GetFileForFormID: the loaded form's file, the temporary file, the compiled file.
        let loaded = form(&mut e, 0x20, 0x0200_0001, false);
        e.register_double(FORM_BY_ID, move |_, a| {
            ret(if a[0] == 0x0200_0001 { loaded } else { 0 })
        });
        e.register(FORM_GET_FILE, |_, _| ret(0x0f10_0000));
        e.register(FILE_FOR_TEMP_ID, |_, a| {
            ret(if a[0] == 0xff00_0001 { 0x0f20_0000 } else { 0 })
        });
        e.register(HANDLER_GET_COMPILED_FILE, |_, a| ret(0x0f30_0000 + a[1]));
        let this = e.mem.alloc(0x34);
        for (id, file) in [
            (0x0200_0001u32, 0x0f10_0000u32),
            (0xff00_0001, 0x0f20_0000),
            (0x0300_0002, 0x0f30_0003),
        ] {
            e.call_log = Some(vec![]);
            assert_eq!(e.call(0x0084_3fe0, &[this, 0xdead, id]).u32(), file);
        }
        assert_eq!(
            calls(&e, HANDLER_GET_COMPILED_FILE),
            vec![vec![0x0aaa_0000, 3]]
        );
        // AddReference_ov2: nothing for a null world space, a warning without a file.
        e.register(FORM_KEY_00460140, |_, _| ret(0x1111));
        e.register(SLOT_NAME, |_, _| ret(0x0300_1000));
        e.register(SAVE_GAME_WARNING, |_, _| Ret::default());
        for key in [
            EXTERIOR_BLOCK_KEY,
            EXTERIOR_SUB_BLOCK_KEY,
            CELL_COORDINATE_KEY,
        ] {
            e.register(key, |_, _| ret(0x70));
        }
        e.call_log = Some(vec![]);
        e.call(0x0084_38b0, &[this, 0, 1, 2, 0x0200_0001, 9]);
        assert_eq!(e.call_log.take().unwrap().len(), 1);
        let world = form(&mut e, 0x41, 0x0400_0004, false);
        e.call_log = Some(vec![]);
        e.call(0x0084_38b0, &[this, world, 1, 2, 0x0300_0002, 9]);
        assert_eq!(e.mem.u32(this + ADDED_COUNT), 1);
        let file_map = value(&entries, this, 0x0f30_0003).unwrap();
        assert_eq!(e.mem.u32(file_map + 0x18), 1, "AddCellForm_ov2 counted");
        assert_ne!(e.mem.u32(file_map + 0x14), 0);
        assert_eq!(
            calls(&e, HANDLER_GET_COMPILED_FILE),
            vec![vec![0x0aaa_0000, 3]],
            "the key from 00460140 is not what is looked up"
        );
        e.register(HANDLER_GET_COMPILED_FILE, |_, _| ret(0));
        e.call_log = Some(vec![]);
        e.call(0x0084_38b0, &[this, world, 1, 2, 0x0300_0002, 9]);
        assert_eq!(
            calls(&e, SAVE_GAME_WARNING),
            vec![vec![NO_FILE_WORLD_MESSAGE, 0x0400_0004, 0x0300_1000]]
        );
        assert_eq!(e.mem.u32(this + ADDED_COUNT), 1);
    }

    #[test]
    fn finishing_an_entry() {
        let (mut e, _) = rig();
        e.map(0x011d_d000, 0x1000);
        e.set_global(SAVE_LOAD_GAME, 0x0bbb_0000u32);
        let refr = form(&mut e, 0x3a, 1, true);
        let entry = e.mem.alloc(8);
        e.mem.set_u32(entry, refr);
        // The reference's vtable: +0x1D0 (an object), +0xFC, +0x100, +0x21C, +0x260.
        let mut slots = vec![0u32; 0x100];
        slots[0xf0 / 4] = SLOT_REFERENCE;
        for slot in [0x1d0u32, 0xfc, 0x100, 0x21c, 0x260, 0xf8] {
            slots[slot as usize / 4] = 0x7200_0000 + slot;
        }
        e.put_vtable(0x0720_0000, &slots);
        e.mem.set_u32(refr, 0x0720_0000);
        e.register(0x7200_01d0, |e, a| ret(e.mem.u32(a[0] + 0x30)));
        e.register(0x7200_00fc, |e, a| ret(e.mem.u32(a[0] + 0x34)));
        e.register(0x7200_0100, |e, a| ret(e.mem.u32(a[0] + 0x38)));
        e.register(0x7200_021c, |e, a| ret(e.mem.u32(a[0] + 0x3c)));
        e.register(0x7200_00f8, |_, _| ret(0));
        quiet(
            &mut e,
            &[
                0x7200_0260,
                REFR_SET_SCALE_00567490,
                TERRAIN_HIDE_TREE,
                PROCESS_LISTS_REMOVE_REFERENCE,
                SET_SAVED_ACQUIRE_OBJECT,
                QUEUE_INIT_PACKAGE_LOCATIONS,
                INIT_DEFAULT_WORN,
                INIT_WORN_006047C0,
                OBJECT_SET_00693EF0,
                0x7000_0000,
            ],
        );
        e.register(SCALE_SOURCE_009CDAE0, |_, _| Ret {
            st0: 1.5,
            ..Ret::default()
        });
        e.register(REFR_GET_SCALE, |_, _| Ret {
            st0: 1.25,
            ..Ret::default()
        });
        e.register(SCALE_VALUE_00598040, |_, _| Ret {
            st0: 2.0,
            ..Ret::default()
        });
        e.register(ENTRY_TEST_MASK, |_, a| ret(u32::from(a[1] == 8)));
        e.register(REFR_BASE_FORM, |_, _| ret(0x0d00_0000));
        e.register(FORM_TEST_00549580, |_, _| ret(1));
        e.register(FORM_TEST_00440DA0, |_, _| ret(1));
        e.register(REFR_GET_WORLD_SPACE, |_, _| ret(0x0d10_0000));
        e.register(WORLD_GET_TERRAIN_MANAGER, |_, _| ret(0x0d20_0000));
        e.register(REFR_TEST_00437B90, |e, a| ret(e.mem.u32(a[0] + 0x2c)));
        let acquire = e.mem.alloc(4);
        e.put_vtable(0x0730_0000, &[0x7000_0000]);
        e.mem.set_u32(acquire, 0x0730_0000);
        e.register_double(GET_SAVED_ACQUIRE_OBJECT, move |e, a| {
            ret(if e.mem.u32(a[0] + 0x28) != 0 {
                acquire
            } else {
                0
            })
        });
        e.register(GET_CURRENT_PROCESS_TYPE, |_, _| ret(2));
        e.register(ACQUIRE_OBJECT_CONSTRUCT, |_, a| ret(a[0]));
        e.register(ACTOR_BASE_004181E0, |_, _| ret(0x0d30_0000));
        // An object with another scale, a tree to hide, a saved object to drop, an actor.
        e.mem.set_u32(refr + 0x30, 0x0d40_0000);
        e.mem.set_u32(refr + 0x34, 1);
        e.mem.set_u32(refr + 0x38, 1);
        e.mem.set_u32(refr + 0x2c, 1);
        e.mem.set_u32(refr + 0x28, 1);
        e.call_log = Some(vec![]);
        e.call(0x0084_3c90, &[0x1234, entry]);
        assert_eq!(
            calls(&e, REFR_SET_SCALE_00567490),
            vec![vec![refr, 2.0f32.to_bits()]]
        );
        assert_eq!(
            calls(&e, TERRAIN_HIDE_TREE),
            vec![vec![0x0d20_0000, refr, 1]]
        );
        assert_eq!(
            calls(&e, PROCESS_LISTS_REMOVE_REFERENCE),
            vec![vec![PROCESS_LISTS, refr, 2]]
        );
        assert_eq!(calls(&e, 0x7000_0000), vec![vec![acquire, 1]]);
        assert_eq!(calls(&e, SET_SAVED_ACQUIRE_OBJECT), vec![vec![refr, 0]]);
        assert_eq!(
            calls(&e, QUEUE_INIT_PACKAGE_LOCATIONS),
            vec![vec![0x0bbb_0000, refr]]
        );
        assert_eq!(
            calls(&e, INIT_WORN_006047C0),
            vec![vec![0x0d30_0000, refr, 1, 1, 0, 1]]
        );
        // Equal scales, no saved object and 00437b90 false: a new object, slot +0x260;
        // vtable +0x21C true: InitDefaultWorn.
        e.register(REFR_GET_SCALE, |_, _| Ret {
            st0: 1.5,
            ..Ret::default()
        });
        e.mem.set_u32(refr + 0x2c, 0);
        e.mem.set_u32(refr + 0x28, 0);
        e.mem.set_u32(refr + 0x3c, 1);
        e.call_log = Some(vec![]);
        e.call(0x0084_3c90, &[0, entry]);
        assert!(calls(&e, REFR_SET_SCALE_00567490).is_empty());
        let made = calls(&e, SET_SAVED_ACQUIRE_OBJECT);
        assert_eq!(made.len(), 1);
        assert_eq!(calls(&e, OPERATOR_NEW), vec![vec![ACQUIRE_OBJECT_SIZE]]);
        assert_eq!(calls(&e, 0x7200_0260), vec![vec![refr]]);
        assert_eq!(
            calls(&e, INIT_DEFAULT_WORN),
            vec![vec![0x0d30_0000, refr, 1, 1, 1]]
        );
        // A NaN scale is never equal.
        e.register(REFR_GET_SCALE, |_, _| Ret {
            st0: f64::NAN,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x0084_3c90, &[0, entry]);
        assert_eq!(calls(&e, REFR_SET_SCALE_00567490).len(), 1);
        // Not a reference: vtable +0xF8 with the member at +0x30, or type 0x43.
        let other = form(&mut e, 0x43, 2, false);
        e.mem.set_u32(other, 0x0720_0000);
        e.mem.set_u32(other + 0x20, 0);
        e.register(SLOT_REFERENCE, |e, a| ret(e.mem.u32(a[0] + 0x20)));
        e.register(FORM_OBJECT_0068EFB0, |_, _| ret(0x0d50_0000));
        e.register(FORM_TEST_00440DA0, |_, _| ret(0x0201));
        e.mem.set_u32(entry, other);
        e.call_log = Some(vec![]);
        e.call(0x0084_3c90, &[0, entry]);
        assert_eq!(calls(&e, OBJECT_SET_00693EF0), vec![vec![0x0d50_0000, 1]]);
        e.register(0x7200_00f8, |_, _| ret(1));
        e.put_vtable(0x0740_0000, &[0, 0, 0, 0, 0x7400_0010]);
        e.mem.set_u32(other + 0x30, 0x0740_0000);
        e.register(0x7400_0010, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0084_3c90, &[0, entry]);
        assert_eq!(calls(&e, 0x7400_0010), vec![vec![other + 0x30, 0]]);
        assert!(calls(&e, OBJECT_SET_00693EF0).is_empty());
    }
}

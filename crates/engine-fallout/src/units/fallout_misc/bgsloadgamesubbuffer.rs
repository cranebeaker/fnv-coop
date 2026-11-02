//! `fallout/misc/saveload/bgsloadgamesubbuffer.cpp` (Xbox PDB source unit), subsystem `fallout/misc`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00865570 (decompiled, FalloutNV.exe 1.4.0.525)
/// Folded append wrapper: reads position at +0xc and forwards the low
/// byte of the last stack argument to the positioned save-buffer writer.
pub fn append_bytes(e: &mut Engine, this: Ptr, source: Ptr, size: u32, no_separator: u8) {
    let position = e.mem.u32(this.addr().wrapping_add(0xc));
    e.call(
        0x00865ce0,
        &args![this, source, size, position, no_separator],
    );
}

/// `00aa1070`: the allocator (cdecl, byte count).
const ALLOCATE: u32 = 0x00aa_1070;
/// `00403d30` (`memset`, cdecl: destination, value, byte count).
const MEMSET: u32 = 0x0040_3d30;
/// `00438af0(this)`: the base teardown (`fallout shared/run_00438af0`).
const BASE_TEARDOWN: u32 = 0x0043_8af0;
/// `00aa10f0(pointer)`: releases a block (cdecl, one argument).
const FREE_BLOCK: u32 = 0x00aa_10f0;
/// `__ehvec_ctor` (`00ec782f`, CRT): constructs an array of elements.
const VECTOR_CONSTRUCT: u32 = 0x00ec_782f;

// Translated from 008656c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: stores the vtable `010822a8`, the bucket count
/// at +4 and 0 at +0xc, allocates `4 * count` bytes for the table (+8) and
/// zeroes it. Returns `this`.
pub fn fn_008656c0(e: &mut Engine, this: Ptr, bucket_count: u32) -> Ptr {
    let t = this.addr();
    e.mem.set_u32(t, 0x0108_22a8);
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

// Translated from 00865660 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: `008656c0(this, word)`, then the vtable
/// `01082288`. Returns `this`.
pub fn fn_00865660(e: &mut Engine, this: Ptr, word: u32) -> Ptr {
    fn_008656c0(e, this, word);
    e.mem.set_u32(this.addr(), 0x0108_2288);
    this
}

// Translated from 00865200 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: `00865660(this, 0x25)`; `EAX` is whatever that
/// returns (it returns `this`).
pub fn fn_00865200(e: &mut Engine, this: Ptr) -> u32 {
    fn_00865660(e, this, 0x25).addr()
}

// Translated from 00865840 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: stores the vtable `010822a8`, runs
/// `00438af0(this)` and releases the block at +8 (`00aa10f0`). `EAX` is the
/// release call's result, returned here.
pub fn fn_00865840(e: &mut Engine, this: Ptr) -> u32 {
    e.mem.set_u32(this.addr(), 0x0108_22a8);
    e.call(BASE_TEARDOWN, &args![this]);
    let block = e.mem.u32(this.addr() + 8);
    e.call(FREE_BLOCK, &args![block]).u32()
}

// Translated from 008657e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: stores the vtable `01082288`, runs
/// `00438af0(this)` and then `00865840`; `EAX` is the latter's result.
// Translation incomplete: 008657e0 - SEH exception unwinding is unsupported.
pub fn fn_008657e0(e: &mut Engine, this: Ptr) -> u32 {
    e.mem.set_u32(this.addr(), 0x0108_2288);
    e.call(BASE_TEARDOWN, &args![this]);
    fn_00865840(e, this)
}

// Translated from 008651d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: constructs the array of three 0x10-byte
/// elements at `this` with `__ehvec_ctor` (`00ec782f`: element constructor
/// `00865200`, element destructor `008657e0`). Returns `this`.
pub fn fn_008651d0(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(
        VECTOR_CONSTRUCT,
        &args![this, 0x10u32, 3u32, 0x0086_5200u32, 0x0086_57e0u32],
    );
    this
}

/// The function `fn_00865870` calls on the member (`00865900`).
const CALLEE_00865900: u32 = 0x0086_5900;

// Translated from 00865870 (decompiled, FalloutNV.exe 1.4.0.525)
/// Named `NiTMap<TESForm*,BGSLoadGameSubBuffer>::NewItem` (Xbox PDB name in the engine map). Thin wrapper: calls `00865900` with `ECX` = `this + 0xc` and passes on its pointer result.
pub fn fn_00865870(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(CALLEE_00865900, &args![this.byte_add(0xc)]).ptr()
}

/// `004b9ba0` (thiscall): the map's first occupied bucket entry, or 0.
const MAP_FIRST_ITEM: u32 = 0x004b_9ba0;
/// `0044dee0` (thiscall): clears the sub-buffer's data pointer (its
/// constructor).
const SUB_BUFFER_CONSTRUCT: u32 = 0x0044_dee0;
/// `00865730` (thiscall, 3 words): takes the map entry at `*position`,
/// writes its form and sub-buffer to the out pointers and moves `*position`
/// to the next entry.
const MAP_GET_NEXT: u32 = 0x0086_5730;
/// `00435b20` (thiscall): releases the sub-buffer's data and clears it.
const SUB_BUFFER_FREE: u32 = 0x0043_5b20;
/// `0043fcd0` (thiscall): non-zero when the reference's 3D is loaded.
const REFERENCE_LOADED_3D: u32 = 0x0043_fcd0;
/// `00864b00` `BGSLoadGameSubBuffer::LoadAnimation` (Xbox PDB name in the
/// engine map; thiscall, form).
const LOAD_ANIMATION: u32 = 0x0086_4b00;
/// `00864c50` `BGSLoadGameSubBuffer::LoadCharController` (Xbox PDB name in
/// the engine map; thiscall, form).
const LOAD_CHAR_CONTROLLER: u32 = 0x0086_4c50;
/// `00864d80` `BGSLoadGameSubBuffer::LoadHavokData` (Xbox PDB name in the
/// engine map; thiscall, form).
const LOAD_HAVOK_DATA: u32 = 0x0086_4d80;
/// `00559450` (thiscall): the word at +0 (the sub-buffer's data).
const SUB_BUFFER_DATA: u32 = 0x0055_9450;
/// `005d43c0` (thiscall): `this + 0x44` (the reference's extra data list).
const REFERENCE_EXTRA_LIST: u32 = 0x005d_43c0;
/// `00422940` `ExtraDataList::SetSavedAnimation` (Xbox PDB name in the
/// engine map; thiscall, data).
const SET_SAVED_ANIMATION: u32 = 0x0042_2940;
/// `00422ac0` `ExtraDataList::SetSavedHavokData` (Xbox PDB name in the
/// engine map; thiscall, data).
const SET_SAVED_HAVOK_DATA: u32 = 0x0042_2ac0;
/// `0084e3a0` (thiscall): the word at +0xc (a form's id, a save-form
/// buffer's size).
const WORD_AT_C: u32 = 0x0084_e3a0;
/// `00845f70` (cdecl): bytes the variable-sized encoding of a value takes.
const VARIABLE_SIZE_LENGTH: u32 = 0x0084_5f70;
/// `00401000` (cdecl, byte count): allocates a block.
const ALLOCATE_BLOCK: u32 = 0x0040_1000;
/// `008a81c0` `BGSLoadFormBuffer::GetVersion` (Xbox PDB name in the engine
/// map; thiscall, byte result).
const BUFFER_GET_VERSION: u32 = 0x008a_81c0;
/// `00845fc0` `SaveVariableSizedValueToBuffer` (Xbox PDB name in the engine
/// map; cdecl: destination, value).
const SAVE_VARIABLE_SIZED_VALUE: u32 = 0x0084_5fc0;
/// `00726070` (thiscall): the word at +4 (the buffer's data).
const WORD_AT_4: u32 = 0x0072_6070;
/// `00401460` (cdecl: destination, source, byte count): copies bytes.
const COPY_BYTES: u32 = 0x0040_1460;
/// `00846080` `LoadVariableSizedValueFromBuffer` (Xbox PDB name in the
/// engine map; cdecl: source, out value): the bytes it read.
const LOAD_VARIABLE_SIZED_VALUE: u32 = 0x0084_6080;
/// `00865f60` `BGSSaveGameBuffer::SaveVariableSizedValue` (Xbox PDB name in
/// the engine map; thiscall, value).
const SAVE_GAME_VARIABLE_SIZED_VALUE: u32 = 0x0086_5f60;
/// `008659c0` `BGSSaveFormBuffer::BGSSaveFormBuffer` (Xbox PDB name in the
/// engine map; thiscall).
const SAVE_FORM_BUFFER_CONSTRUCT: u32 = 0x0086_59c0;
/// The save-load manager singleton pointer.
const SAVE_LOAD_MANAGER: u32 = 0x011d_e134;
/// `00851110` (thiscall): returns 0x1b.
const MANAGER_VERSION: u32 = 0x0085_1110;
/// `00401170` (thiscall): the byte at +4 (a form's type).
const BYTE_AT_4: u32 = 0x0040_1170;
/// `008c71b0` (thiscall, word): stores the word at +0 and returns `this`
/// (the body is a constructor; the engine map's name
/// `BGSChangeFlags::IsForcedChange` does not fit it).
const CHANGE_FLAGS_CONSTRUCT: u32 = 0x008c_71b0;
/// `00865ad0` (thiscall, 4 words): sets up the save-form buffer.
const SAVE_FORM_BUFFER_INIT: u32 = 0x0086_5ad0;
/// `0050f9c0` (thiscall, form).
const SAVE_FORM_BUFFER_SET_FORM: u32 = 0x0050_f9c0;
/// `00563650` `TESObjectREFR::SaveAnimation` (Xbox PDB name in the engine
/// map; thiscall, buffer).
const REFERENCE_SAVE_ANIMATION: u32 = 0x0056_3650;
/// `00562de0` `TESObjectREFR::SaveHavokData` (Xbox PDB name in the engine
/// map; thiscall, buffer).
const REFERENCE_SAVE_HAVOK_DATA: u32 = 0x0056_2de0;
/// `00847dd0` (thiscall): the save-form buffer's teardown (the engine map
/// names the folded body `BGSSaveLoadManager::GetMinorVersion`).
const SAVE_FORM_BUFFER_DESTRUCT: u32 = 0x0084_7dd0;
/// `00845cc0` `SaveGameWarning` (Xbox PDB name in the engine map; cdecl,
/// format and its arguments).
const SAVE_GAME_WARNING: u32 = 0x0084_5cc0;
/// `"SAVELOAD: Form '%s' %08X passed to
/// BGSLoadGameSubBuffer::SaveAnimation() is not a reference"` (both savers
/// pass it).
const NOT_A_REFERENCE_FORMAT: u32 = 0x0108_2228;
/// The eh vector destructor iterator (`00ec5fce`, stdcall: array, element
/// size, count, element destructor).
const VECTOR_DESTRUCT: u32 = 0x00ec_5fce;

// Translated from 00865390 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSSaveLoadQueuedSubBufferMap::LoadQueuedSubBuffers` (Xbox PDB name in
/// the engine map): walks the map of kind `kind` (`this + 0x10 * kind`;
/// 0 animation, 1 character controller, 2 Havok data). For each queued
/// form whose 3D is loaded (`0043fcd0`), or that is not a reference
/// (vtable +0xF0), the kind's loader loads the sub-buffer; otherwise kind 0
/// frees it when the reference's vtable +0x100 says so or else hands it to
/// the reference's extra data as its saved animation, kind 1 frees it, and
/// kind 2 hands it over as saved Havok data. Ends with the map's teardown
/// (`00438af0`).
pub fn load_queued_sub_buffers(e: &mut Engine, this: Ptr, kind: i32) {
    let map = this.addr().wrapping_add((kind as u32) << 4);
    // Locals: position at +0, form at +4, the sub-buffer at +8.
    e.with_stack(0xc, |e, locals| {
        let position = locals.addr();
        let form_slot = position + 4;
        let sub_buffer = position + 8;
        let first = e.call(MAP_FIRST_ITEM, &args![map]).u32();
        e.mem.set_u32(position, first);
        while e.mem.u32(position) != 0 {
            e.mem.set_u32(form_slot, 0);
            e.call(SUB_BUFFER_CONSTRUCT, &args![sub_buffer]);
            e.call(MAP_GET_NEXT, &args![map, position, form_slot, sub_buffer]);
            let mut load = true;
            let mut reference = 0u32;
            let mut frees = 0u32;
            let form = e.mem.u32(form_slot);
            if e.vcall(form, 0xf0, &[]).bool() {
                reference = e.mem.u32(form_slot);
                load = e.call(REFERENCE_LOADED_3D, &args![reference]).u32() != 0;
                if e.vcall(reference, 0x100, &[]).bool() {
                    frees = reference;
                }
            }
            if load {
                let form = e.mem.u32(form_slot);
                match kind {
                    0 => {
                        e.call(LOAD_ANIMATION, &args![sub_buffer, form]);
                    }
                    1 => {
                        e.call(LOAD_CHAR_CONTROLLER, &args![sub_buffer, form]);
                    }
                    2 => {
                        e.call(LOAD_HAVOK_DATA, &args![sub_buffer, form]);
                    }
                    _ => {}
                }
            } else {
                match kind {
                    0 => {
                        if frees != 0 {
                            e.call(SUB_BUFFER_FREE, &args![sub_buffer]);
                        } else if reference != 0 {
                            let data = e.call(SUB_BUFFER_DATA, &args![sub_buffer]).u32();
                            let list = e.call(REFERENCE_EXTRA_LIST, &args![reference]).u32();
                            e.call(SET_SAVED_ANIMATION, &args![list, data]);
                        }
                    }
                    1 => {
                        e.call(SUB_BUFFER_FREE, &args![sub_buffer]);
                    }
                    2 => {
                        if reference != 0 {
                            let data = e.call(SUB_BUFFER_DATA, &args![sub_buffer]).u32();
                            let list = e.call(REFERENCE_EXTRA_LIST, &args![reference]).u32();
                            e.call(SET_SAVED_HAVOK_DATA, &args![list, data]);
                        }
                    }
                    _ => {}
                }
            }
        }
    });
    e.call(BASE_TEARDOWN, &args![map]);
}

// Translated from 00865360 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: loads the queued sub-buffers of all three
/// kinds (`00865390` with 0, 1, 2).
pub fn fn_00865360(e: &mut Engine, this: Ptr) {
    load_queued_sub_buffers(e, this, 0);
    load_queued_sub_buffers(e, this, 1);
    load_queued_sub_buffers(e, this, 2);
}

// Translated from 00865120 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: copies the save-form buffer `buffer` into a
/// new block stored at `this + 0`: one version byte (`008a81c0`), the
/// buffer's size in the variable-sized encoding (`00845fc0`), then the
/// buffer's bytes. Nothing happens for an empty buffer.
pub fn fn_00865120(e: &mut Engine, this: Ptr, buffer: Ptr) {
    let size = e.call(WORD_AT_C, &args![buffer]).u32();
    if size == 0 {
        return;
    }
    let size_length = e.call(VARIABLE_SIZE_LENGTH, &args![size]).u32();
    let total = size.wrapping_add(size_length).wrapping_add(1);
    let block = e.call(ALLOCATE_BLOCK, &args![total]).u32();
    e.mem.set_u32(this.addr(), block);
    let mut cursor = e.mem.u32(this.addr());
    let version_at = cursor;
    let version = e.call(BUFFER_GET_VERSION, &args![buffer]).u32() as u8;
    e.mem.set_u8(version_at, version);
    cursor = cursor.wrapping_add(1);
    e.call(SAVE_VARIABLE_SIZED_VALUE, &args![cursor, size]);
    cursor = cursor.wrapping_add(size_length);
    let source = e.call(WORD_AT_4, &args![buffer]).u32();
    e.call(COPY_BYTES, &args![cursor, source, size]);
}

// Translated from 00865510 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSLoadGameSubBuffer::SaveGame` (Xbox PDB name in the engine map): reads
/// the size stored after the version byte of the data at `this + 0`
/// (`00846080`), writes it to the save buffer (`00865f60`) and appends the
/// bytes that follow it (`00865570`, without separator).
pub fn bgs_load_game_sub_buffer_save_game(e: &mut Engine, this: Ptr, save_buffer: Ptr) {
    let mut cursor = e.mem.u32(this.addr()).wrapping_add(1);
    let size = e.with_stack(4, |e, value| {
        let read = e
            .call(LOAD_VARIABLE_SIZED_VALUE, &args![cursor, value])
            .u32();
        cursor = read.wrapping_add(cursor);
        e.mem.u32(value.addr())
    });
    e.call(SAVE_GAME_VARIABLE_SIZED_VALUE, &args![save_buffer, size]);
    append_bytes(e, save_buffer, Ptr::new(cursor), size, 1);
}

/// The body `00864f20` and `00865020` share: when `form` is a reference
/// (vtable +0xF0), builds a save-form buffer on the stack for it (version
/// 0x1b, the form's type byte, change flags 4, its id), lets the reference
/// write into it through `save` and copies it into this sub-buffer
/// (`00865120`); otherwise logs the "not a reference" warning with the
/// form's name (vtable +0x130) and id.
fn save_reference_data(e: &mut Engine, this: Ptr, form: Ptr, save: u32) {
    if e.vcall(form.addr(), 0xf0, &[]).bool() {
        let reference = form;
        e.with_stack(0x24, |e, buffer| {
            e.call(SAVE_FORM_BUFFER_CONSTRUCT, &args![buffer]);
            let manager = e.global::<u32>(SAVE_LOAD_MANAGER);
            let version = e.call(MANAGER_VERSION, &args![manager]).u32();
            let form_type = e.call(BYTE_AT_4, &args![form]).u32();
            // The change flags are an object passed by value: built in its
            // argument slot.
            let flags = e.with_stack(4, |e, slot| {
                e.call(CHANGE_FLAGS_CONSTRUCT, &args![slot, 4u32]);
                e.mem.u32(slot.addr())
            });
            let form_id = e.call(WORD_AT_C, &args![form]).u32();
            e.call(
                SAVE_FORM_BUFFER_INIT,
                &args![buffer, form_id, flags, form_type, version],
            );
            e.call(SAVE_FORM_BUFFER_SET_FORM, &args![buffer, form]);
            e.call(save, &args![reference, buffer]);
            fn_00865120(e, this, buffer);
            e.call(SAVE_FORM_BUFFER_DESTRUCT, &args![buffer]);
        });
    } else {
        let form_id = e.call(WORD_AT_C, &args![form]).u32();
        let name = e.vcall(form.addr(), 0x130, &[]).u32();
        e.call(
            SAVE_GAME_WARNING,
            &args![NOT_A_REFERENCE_FORMAT, name, form_id],
        );
    }
}

// Translated from 00864f20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: saves a reference's animation into this
/// sub-buffer (`TESObjectREFR::SaveAnimation`, `00563650`, through a
/// save-form buffer); a form that is not a reference only logs a warning.
// Translation incomplete: 00864f20 - SEH exception unwinding (the stack buffer's destructor on unwind) is unsupported.
pub fn fn_00864f20(e: &mut Engine, this: Ptr, form: Ptr) {
    save_reference_data(e, this, form, REFERENCE_SAVE_ANIMATION);
}

// Translated from 00865020 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: saves a reference's Havok data into this
/// sub-buffer (`TESObjectREFR::SaveHavokData`, `00562de0`, through a
/// save-form buffer); a form that is not a reference only logs the same
/// warning as `00864f20`.
// Translation incomplete: 00865020 - SEH exception unwinding (the stack buffer's destructor on unwind) is unsupported.
pub fn fn_00865020(e: &mut Engine, this: Ptr, form: Ptr) {
    save_reference_data(e, this, form, REFERENCE_SAVE_HAVOK_DATA);
}

// Translated from 00865220 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the destructor of the three queued sub-buffer
/// maps at `this` (0x10 bytes each): frees every queued sub-buffer, tears
/// each map down (`00438af0`), then runs the eh vector destructor iterator
/// over the array with `008657e0`.
// Translation incomplete: 00865220 - SEH exception unwinding is unsupported.
pub fn fn_00865220(e: &mut Engine, this: Ptr) {
    for index in 0u32..3 {
        let map = this.addr().wrapping_add(index << 4);
        e.with_stack(0xc, |e, locals| {
            let position = locals.addr();
            let form_slot = position + 4;
            let sub_buffer = position + 8;
            let first = e.call(MAP_FIRST_ITEM, &args![map]).u32();
            e.mem.set_u32(position, first);
            while e.mem.u32(position) != 0 {
                e.mem.set_u32(form_slot, 0);
                e.call(SUB_BUFFER_CONSTRUCT, &args![sub_buffer]);
                e.call(MAP_GET_NEXT, &args![map, position, form_slot, sub_buffer]);
                e.call(SUB_BUFFER_FREE, &args![sub_buffer]);
            }
        });
        e.call(BASE_TEARDOWN, &args![map]);
    }
    e.call(VECTOR_DESTRUCT, &args![this, 0x10u32, 3u32, 0x0086_57e0u32]);
}

/// The player pointer (`011dea3c`).
const PLAYER: u32 = 0x011d_ea3c;
/// `007af430` (`thiscall`; engine map `BGSSaveFormBuffer::GetForm`, a
/// folded body): the word the animation load compares the form with.
const PLAYER_WORD_007AF430: u32 = 0x007a_f430;
/// `PlayerCharacter::GetAnimation` (`00950a60`, `thiscall`, `RET 4`: a
/// byte).
const PLAYER_GET_ANIMATION: u32 = 0x0095_0a60;
/// `0049b050` (`thiscall`, `RET 4`; engine-map unit `animation.cpp`): loads
/// an animation from the load-form buffer.
const ANIMATION_LOAD: u32 = 0x0049_b050;
/// `005636e0` (`thiscall`, `RET 4`; `tesobjectrefr.cpp`): a reference's
/// animation load from the buffer.
const REFERENCE_LOAD_ANIMATION: u32 = 0x0056_36e0;
/// `0081db60` (`thiscall`, `RET 4`): the load-form buffer's scalar deleting
/// destructor.
const LOAD_FORM_BUFFER_DELETE: u32 = 0x0081_db60;
/// `MobileObject::GetCharController` (`009306d0`), `MiddleHighProcess::
/// GetSavedAcquireObject` (`008d8520`, a folded body: the actor's process)
/// and `MiddleHighProcess::LoadCharController` (`00928a50`, `RET 4`).
const GET_CHAR_CONTROLLER: u32 = 0x0093_06d0;
const ACTOR_PROCESS: u32 = 0x008d_8520;
const PROCESS_LOAD_CHAR_CONTROLLER: u32 = 0x0092_8a50;
/// `TESObjectREFR::LoadHavokData` (`00562ed0`, `thiscall`, `RET 4`).
const REFERENCE_LOAD_HAVOK_DATA: u32 = 0x0056_2ed0;
/// The load warnings (format, name, id).
const WARNING_NOT_A_REFERENCE: u32 = 0x0108_2098;
const WARNING_NO_ANIMATION: u32 = 0x0108_20e0;
const WARNING_NOT_A_REFERENCE_FOR_CONTROLLER: u32 = 0x0108_2130;
const WARNING_NOT_AN_ACTOR: u32 = 0x0108_2178;
const WARNING_NO_CHAR_CONTROLLER: u32 = 0x0108_21d0;
/// `BGSLoadFormBuffer::BGSLoadFormBuffer` (`008643b0`), its form setter
/// (`007037c0`, `RET 4`), version setter (`008a8150`, `RET 4`) and
/// `BGSLoadGameBuffer::SetBuffer` (`008647f0`, `RET 0xc`).
const LOAD_FORM_BUFFER_CONSTRUCT: u32 = 0x0086_43b0;
const LOAD_FORM_BUFFER_SET_FORM: u32 = 0x0070_37c0;
const LOAD_FORM_BUFFER_SET_VERSION: u32 = 0x008a_8150;
const LOAD_GAME_BUFFER_SET_BUFFER: u32 = 0x0086_47f0;
/// `00844700` (`thiscall`, `RET 8`: key, value): the map's `SetAt`.
const MAP_SET_AT: u32 = 0x0084_4700;
/// `BGSLoadGameBuffer::LoadVariableSizedValue` (`00864a60`, `thiscall`).
const BUFFER_LOAD_VARIABLE: u32 = 0x0086_4a60;
/// `BGSUnloadedFormBuffer::BGSUnloadedFormBuffer` (`00537e90`, `thiscall`,
/// `RET 4`: the data).
const UNLOADED_FORM_BUFFER_CONSTRUCT: u32 = 0x0053_7e90;
/// `0084da90` (`thiscall`, `RET 4`): the buffer's current read address.
const BUFFER_CURRENT: u32 = 0x0084_da90;
/// `0049bb50` (`thiscall`, `RET 4`): moves the buffer's position on.
const BUFFER_SKIP: u32 = 0x0049_bb50;
/// `00401030` (cdecl): the game's `operator delete`.
const OPERATOR_DELETE: u32 = 0x0040_1030;
/// `006b8310` (`thiscall`, `RET 4`): returns a map item to the allocator at
/// +0xc.
const MAP_FREE_ITEM: u32 = 0x006b_8310;
/// `00865980` (`thiscall`): the new map item's constructor.
const MAP_ITEM_CONSTRUCT: u32 = 0x0086_5980;

/// The load-form buffer's deleting destructor when there is one
/// (`0081db60` with 1).
fn delete_load_form_buffer(e: &mut Engine, buffer: u32) {
    if buffer != 0 {
        e.call(LOAD_FORM_BUFFER_DELETE, &args![buffer, 1u32]);
    }
}

/// `SaveGameWarning` with the form's name (vtable `+0x130`) and id.
fn warn_with_form(e: &mut Engine, format: u32, form: u32) {
    let id = e.call(WORD_AT_C, &args![form]).u32();
    let name = e.vcall(form, 0x130, &args![]).u32();
    e.call(SAVE_GAME_WARNING, &args![format, name, id]);
}

// Translated from 00864b00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSLoadGameSubBuffer::LoadAnimation` (Xbox PDB; `thiscall`, `RET 4`):
/// a form equal to the player's `007af430` word becomes the player (and
/// marks it). The data goes into a load-form buffer (`00864e20`). For a
/// reference (vtable `+0xf0`): an actor (`+0x100`) loads it into its
/// animation (the player's `GetAnimation` with the mark, others' `+0x1e4`;
/// `0049b050`), warning when it has none; another reference loads it
/// itself (`005636e0`). Not a reference: a warning. The buffer is deleted.
pub fn bgs_load_game_sub_buffer_load_animation(e: &mut Engine, this: Ptr, form: u32) {
    let mut form = form;
    let mut is_player = 0u32;
    let player = e.global::<u32>(PLAYER);
    if form == e.call(PLAYER_WORD_007AF430, &args![player]).u32() {
        is_player = 1;
        form = e.global::<u32>(PLAYER);
    }
    let buffer = bgs_load_game_sub_buffer_create_load_form_buffer(e, this, form);
    if e.vcall(form, 0xf0, &args![]).bool() {
        if e.vcall(form, 0x100, &args![]).bool() {
            let animation = if form == e.global::<u32>(PLAYER) {
                let player = e.global::<u32>(PLAYER);
                e.call(PLAYER_GET_ANIMATION, &args![player, is_player])
                    .u32()
            } else {
                e.vcall(form, 0x1e4, &args![]).u32()
            };
            if animation != 0 {
                e.call(ANIMATION_LOAD, &args![animation, buffer]);
            } else {
                warn_with_form(e, WARNING_NO_ANIMATION, form);
            }
        } else {
            e.call(REFERENCE_LOAD_ANIMATION, &args![form, buffer]);
        }
    } else {
        warn_with_form(e, WARNING_NOT_A_REFERENCE, form);
    }
    delete_load_form_buffer(e, buffer);
}

// Translated from 00864c50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSLoadGameSubBuffer::LoadCharController` (Xbox PDB; `thiscall`,
/// `RET 4`): the data in a load-form buffer (`00864e20`) is loaded by the
/// process of an actor (vtable `+0xf0`, then `+0xfc`) that has a character
/// controller (`009306d0`; `008d8520`, `00928a50`); a warning for an actor
/// without one, a reference that is no actor, or a form that is no
/// reference. The buffer is deleted.
pub fn bgs_load_game_sub_buffer_load_char_controller(e: &mut Engine, this: Ptr, form: u32) {
    let buffer = bgs_load_game_sub_buffer_create_load_form_buffer(e, this, form);
    if e.vcall(form, 0xf0, &args![]).bool() {
        if e.vcall(form, 0xfc, &args![]).bool() {
            if e.call(GET_CHAR_CONTROLLER, &args![form]).u32() != 0 {
                let process = e.call(ACTOR_PROCESS, &args![form]).u32();
                e.call(PROCESS_LOAD_CHAR_CONTROLLER, &args![process, buffer]);
            } else {
                warn_with_form(e, WARNING_NO_CHAR_CONTROLLER, form);
            }
        } else {
            warn_with_form(e, WARNING_NOT_AN_ACTOR, form);
        }
    } else {
        warn_with_form(e, WARNING_NOT_A_REFERENCE_FOR_CONTROLLER, form);
    }
    delete_load_form_buffer(e, buffer);
}

// Translated from 00864d80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSLoadGameSubBuffer::LoadHavokData` (Xbox PDB; `thiscall`, `RET 4`):
/// a reference (vtable `+0xf0`) loads the data from a load-form buffer
/// (`00864e20`, `TESObjectREFR::LoadHavokData`); otherwise a warning. The
/// buffer is deleted.
pub fn bgs_load_game_sub_buffer_load_havok_data(e: &mut Engine, this: Ptr, form: u32) {
    let buffer = bgs_load_game_sub_buffer_create_load_form_buffer(e, this, form);
    if e.vcall(form, 0xf0, &args![]).bool() {
        e.call(REFERENCE_LOAD_HAVOK_DATA, &args![form, buffer]);
    } else {
        warn_with_form(e, WARNING_NOT_A_REFERENCE, form);
    }
    delete_load_form_buffer(e, buffer);
}

// Translated from 00864e20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSLoadGameSubBuffer::CreateLoadFormBuffer` (Xbox PDB; `thiscall`,
/// `RET 4`): a new load-form buffer (0x30 bytes, `00401000`, its
/// constructor; null when the allocation fails) for `form` (`007037c0`)
/// with the data's version byte (`008a8150`), over the data at `this + 0`
/// (`SetBuffer` with the size read after the version byte, `00846080`, and
/// the header length as start). The sub-buffer gives the data up (its word
/// cleared). Answers the buffer.
// Translation incomplete: 00864e20 - the C++ exception frame (FS:[0] chain, unwind state 0 that frees the new buffer) and the /GS cookie word are not translated.
pub fn bgs_load_game_sub_buffer_create_load_form_buffer(
    e: &mut Engine,
    this: Ptr,
    form: u32,
) -> u32 {
    let block = e.call(ALLOCATE_BLOCK, &args![0x30u32]).u32();
    let buffer = if block == 0 {
        0
    } else {
        e.call(LOAD_FORM_BUFFER_CONSTRUCT, &args![block]).u32()
    };
    let data = e.mem.u32(this.addr());
    e.call(LOAD_FORM_BUFFER_SET_FORM, &args![buffer, form]);
    let version = u32::from(e.mem.u8(data));
    e.call(LOAD_FORM_BUFFER_SET_VERSION, &args![buffer, version]);
    e.with_stack(4, |e, size| {
        let data = e.mem.u32(this.addr());
        let read = e
            .call(
                LOAD_VARIABLE_SIZED_VALUE,
                &args![data.wrapping_add(1), size],
            )
            .u32();
        let start = read.wrapping_add(1);
        let total = e.mem.u32(size.addr()).wrapping_add(start);
        let data = e.mem.u32(this.addr());
        e.call(
            LOAD_GAME_BUFFER_SET_BUFFER,
            &args![buffer, data, total, start],
        );
    });
    e.mem.set_u32(this.addr(), 0);
    buffer
}

// Translated from 008652f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSSaveLoadQueuedSubBufferMap::QueueSubBuffer` (Xbox PDB; `thiscall`,
/// `RET 0xc`: kind, load buffer, form): loads a sub-buffer from the load
/// buffer (`008655a0` on a local); when it holds data it is set into the
/// kind's map (`this + 0x10 * kind`, `00844700`) under `form`, or without
/// one under the load buffer's form (vtable `+4`).
pub fn bgs_save_load_queued_sub_buffer_map_queue_sub_buffer(
    e: &mut Engine,
    this: Ptr,
    kind: u32,
    buffer: u32,
    form: u32,
) {
    e.with_stack(4, |e, sub_buffer| {
        e.call(SUB_BUFFER_CONSTRUCT, &args![sub_buffer]);
        bgs_load_game_sub_buffer_load_game(e, sub_buffer, buffer);
        if e.call(SUB_BUFFER_DATA, &args![sub_buffer]).u32() == 0 {
            return;
        }
        let key = if form != 0 {
            form
        } else {
            e.vcall(buffer, 4, &args![]).u32()
        };
        let data = e.mem.u32(sub_buffer.addr());
        let map = (kind << 4).wrapping_add(this.addr());
        e.call(MAP_SET_AT, &args![map, key, data]);
    });
}

// Translated from 008655a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSLoadGameSubBuffer::LoadGame` (Xbox PDB; `thiscall`, `RET 4`): reads
/// a size from the load buffer (`00864a60`); for a non-zero size a block of
/// version byte, size encoding (`00845f70`, `00845fc0`) and data is made
/// (`00401000`, kept with `00537e90`): the version from the buffer's
/// vtable slot 0, the data copied from the buffer's current address
/// (`0084da90`, `00401460`); the buffer then moves past it (`0049bb50`).
pub fn bgs_load_game_sub_buffer_load_game(e: &mut Engine, this: Ptr, buffer: u32) {
    let size = e.call(BUFFER_LOAD_VARIABLE, &args![buffer]).u32();
    if size == 0 {
        return;
    }
    let length = e.call(VARIABLE_SIZE_LENGTH, &args![size]).u32();
    let total = size.wrapping_add(length).wrapping_add(1);
    let block = e.call(ALLOCATE_BLOCK, &args![total]).u32();
    e.call(UNLOADED_FORM_BUFFER_CONSTRUCT, &args![this, block]);
    let version = e.vcall(buffer, 0, &args![]).u8();
    e.mem.set_u8(block, version);
    let mut cursor = block.wrapping_add(1);
    e.call(SAVE_VARIABLE_SIZED_VALUE, &args![cursor, size]);
    cursor = cursor.wrapping_add(length);
    let source = e.call(BUFFER_CURRENT, &args![buffer, size]).u32();
    e.call(COPY_BYTES, &args![cursor, source, size]);
    e.call(BUFFER_SKIP, &args![buffer, size]);
}

// Translated from 00865690 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor (engine map: `NiTMap<TESForm*,
/// BGSLoadGameSubBuffer>`; `RET 4`): `008657e0`, then `operator delete` on
/// flag bit 0. Answers `this`.
pub fn fn_00865690(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_008657e0(e, this);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 00865730 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 0xc`: position, key out,
/// value out): the map entry at `*position` (next, key at +4, value at
/// +8) gives its key and value; the position moves to the next entry of
/// the bucket, or of the following buckets (from the key's bucket, vtable
/// `+4`, plus one, up to the count at +4 of the table at +8), or to 0.
pub fn fn_00865730(e: &mut Engine, this: Ptr, position: Ptr, key: Ptr, value: Ptr) {
    let item = e.mem.u32(position.addr());
    e.mem.set_u32(key.addr(), e.mem.u32(item.wrapping_add(4)));
    e.mem.set_u32(value.addr(), e.mem.u32(item.wrapping_add(8)));
    let next = e.mem.u32(item);
    if next != 0 {
        e.mem.set_u32(position.addr(), next);
        return;
    }
    let item_key = e.mem.u32(item.wrapping_add(4));
    let mut bucket = e
        .vcall(this.addr(), 4, &args![item_key])
        .u32()
        .wrapping_add(1);
    while bucket < e.mem.u32(this.addr().wrapping_add(4)) {
        let table = e.mem.u32(this.addr().wrapping_add(8));
        let entry = e.mem.u32(table.wrapping_add(bucket.wrapping_mul(4)));
        if entry != 0 {
            e.mem.set_u32(position.addr(), entry);
            return;
        }
        bucket = bucket.wrapping_add(1);
    }
    e.mem.set_u32(position.addr(), 0);
}

// Translated from 00865890 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`: a map item): stores a
/// fresh change-flags word (`008c71b0` with 0 on a local) as the item's
/// value (+8) and returns the item to the allocator at +0xc (`006b8310`).
pub fn fn_00865890(e: &mut Engine, this: Ptr, item: u32) {
    let flags = e.with_stack(4, |e, local| {
        e.call(CHANGE_FLAGS_CONSTRUCT, &args![local, 0u32]);
        e.mem.u32(local.addr())
    });
    e.mem.set_u32(item.wrapping_add(8), flags);
    e.call(MAP_FREE_ITEM, &args![this.addr().wrapping_add(0xc), item]);
}

// Translated from 008658d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor of the `NiTMapBase` under `00865690` (engine
/// map name; `RET 4`): `00865840`, then `operator delete` on flag bit 0.
/// Answers `this`.
pub fn fn_008658d0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_00865840(e, this);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 00865900 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`ECX` only, not read): a new 0xc-byte map
/// item (`00401000`, constructed by `00865980`), null when the allocation
/// fails.
// Translation incomplete: 00865900 - the C++ exception frame (FS:[0] chain, unwind state 0 that frees the new item) and the /GS cookie word are not translated.
pub fn fn_00865900(e: &mut Engine, _this: Ptr) -> u32 {
    let block = e.call(ALLOCATE_BLOCK, &args![0xcu32]).u32();
    if block == 0 {
        return 0;
    }
    e.call(MAP_ITEM_CONSTRUCT, &args![block]).u32()
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x0086_4b00,
            bgs_load_game_sub_buffer_load_animation(Ptr, u32)
        ),
        entry!(
            0x0086_4c50,
            bgs_load_game_sub_buffer_load_char_controller(Ptr, u32)
        ),
        entry!(
            0x0086_4d80,
            bgs_load_game_sub_buffer_load_havok_data(Ptr, u32)
        ),
        entry!(
            0x0086_4e20,
            bgs_load_game_sub_buffer_create_load_form_buffer(Ptr, u32) -> u32
        ),
        entry!(
            0x0086_52f0,
            bgs_save_load_queued_sub_buffer_map_queue_sub_buffer(Ptr, u32, u32, u32)
        ),
        entry!(0x0086_55a0, bgs_load_game_sub_buffer_load_game(Ptr, u32)),
        entry!(0x0086_5690, fn_00865690(Ptr, u32) -> Ptr),
        entry!(0x0086_5730, fn_00865730(Ptr, Ptr, Ptr, Ptr)),
        entry!(0x0086_5890, fn_00865890(Ptr, u32)),
        entry!(0x0086_58d0, fn_008658d0(Ptr, u32) -> Ptr),
        entry!(0x0086_5900, fn_00865900(Ptr) -> u32),
        entry!(0x00865570, append_bytes(Ptr, Ptr, u32, u8)),
        entry!(0x008656c0, fn_008656c0(Ptr, u32) -> Ptr),
        entry!(0x00865660, fn_00865660(Ptr, u32) -> Ptr),
        entry!(0x00865200, fn_00865200(Ptr) -> u32),
        entry!(0x00865840, fn_00865840(Ptr) -> u32),
        entry!(0x008657e0, fn_008657e0(Ptr) -> u32),
        entry!(0x008651d0, fn_008651d0(Ptr) -> Ptr),
        entry!(0x0086_5870, fn_00865870(Ptr) -> Ptr),
        entry!(0x0086_5390, load_queued_sub_buffers(Ptr, i32)),
        entry!(0x0086_5360, fn_00865360(Ptr)),
        entry!(0x0086_5120, fn_00865120(Ptr, Ptr)),
        entry!(0x0086_5510, bgs_load_game_sub_buffer_save_game(Ptr, Ptr)),
        entry!(0x0086_4f20, fn_00864f20(Ptr, Ptr)),
        entry!(0x0086_5020, fn_00865020(Ptr, Ptr)),
        entry!(0x0086_5220, fn_00865220(Ptr)),
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
        e.register(BASE_TEARDOWN, |_, _| Ret::default());
        e.register(FREE_BLOCK, |_, a| Ret {
            eax: a[0] + 1,
            ..Ret::default()
        });
        e.register(VECTOR_CONSTRUCT, |_, _| Ret::default());
    }

    #[test]
    fn fn_008656c0_builds_a_zeroed_bucket_table() {
        let mut e = Engine::new();
        doubles(&mut e);
        let this = e.mem.alloc(0x10);
        e.mem.set_u32(this + 0xc, 5);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x008656c0, &args![this, 3u32]).u32(), this);
        assert_eq!(e.mem.u32(this), 0x0108_22a8);
        assert_eq!(e.mem.u32(this + 4), 3);
        assert_eq!(e.mem.u32(this + 0xc), 0);
        let table = e.mem.u32(this + 8);
        assert_eq!(e.mem.bytes(table, 12), vec![0; 12]);
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1], (ALLOCATE, vec![12]));
        assert_eq!(log[2], (MEMSET, vec![table, 0, 12]));
    }

    #[test]
    fn fn_00865660_and_00865200_store_the_vtable() {
        let mut e = Engine::new();
        doubles(&mut e);
        let this = e.mem.alloc(0x10);
        assert_eq!(e.call(0x00865660, &args![this, 9u32]).u32(), this);
        assert_eq!(e.mem.u32(this), 0x0108_2288);
        assert_eq!(e.mem.u32(this + 4), 9, "the base initializer ran");
        e.mem.set_u32(this, 0);
        assert_eq!(e.call(0x00865200, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this + 4), 0x25);
    }

    #[test]
    fn fn_00865840_releases_the_block_at_8_and_returns_the_result() {
        let mut e = Engine::new();
        doubles(&mut e);
        let this = e.mem.alloc(0x10);
        e.mem.set_u32(this + 8, 0x7000);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00865840, &args![this]).u32(), 0x7001);
        assert_eq!(e.mem.u32(this), 0x0108_22a8);
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1], (BASE_TEARDOWN, vec![this]));
        assert_eq!(log[2], (FREE_BLOCK, vec![0x7000]));
    }

    #[test]
    fn fn_008657e0_tears_down_twice_and_ends_on_the_second_vtable() {
        let mut e = Engine::new();
        doubles(&mut e);
        let this = e.mem.alloc(0x10);
        e.call_log = Some(vec![]);
        e.call(0x008657e0, &args![this]);
        let order: Vec<u32> = e
            .call_log
            .take()
            .unwrap()
            .iter()
            .skip(1)
            .map(|c| c.0)
            .collect();
        assert_eq!(order, vec![BASE_TEARDOWN, BASE_TEARDOWN, FREE_BLOCK]);
        assert_eq!(e.mem.u32(this), 0x0108_22a8);
    }

    #[test]
    fn fn_008651d0_constructs_three_elements() {
        let mut e = Engine::new();
        doubles(&mut e);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x008651d0, &args![0x6000u32]).u32(), 0x6000);
        assert_eq!(
            e.call_log.take().unwrap()[1],
            (
                VECTOR_CONSTRUCT,
                vec![0x6000, 0x10, 3, 0x0086_5200, 0x0086_57e0]
            )
        );
    }

    #[test]
    fn l68d_00865870_passes_this_plus_c_to_the_callee() {
        let mut e = Engine::new();
        e.register(CALLEE_00865900, |_, a| Ret {
            eax: a[0] ^ 0x5555,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x0086_5870, &args![0x4100u32]).ptr::<()>().addr(),
            0x1459
        );
        let log = e.call_log.take().unwrap();
        let inner: Vec<_> = log.iter().filter(|c| c.0 == CALLEE_00865900).collect();
        assert_eq!(inner.len(), 1);
        assert_eq!(inner[0].1, vec![0x4100 + 0xc]);
    }

    /// Doubles every callee of the queued-map walkers: `MAP_GET_NEXT`
    /// hands out `forms` in order (the position goes to 0 after the last).
    fn queue_doubles(e: &mut Engine, forms: Vec<u32>) {
        e.register(MAP_FIRST_ITEM, |_, _| Ret {
            eax: 1,
            ..Ret::default()
        });
        let mut next = 0usize;
        e.register_double(MAP_GET_NEXT, move |e, a| {
            e.mem.set_u32(a[2], forms[next]);
            e.mem.set_u32(a[3], 0x5100 + next as u32);
            next += 1;
            e.mem.set_u32(a[1], if next < forms.len() { 1 } else { 0 });
            Ret::default()
        });
        for addr in [
            SUB_BUFFER_CONSTRUCT,
            SUB_BUFFER_FREE,
            LOAD_ANIMATION,
            LOAD_CHAR_CONTROLLER,
            LOAD_HAVOK_DATA,
            SET_SAVED_ANIMATION,
            SET_SAVED_HAVOK_DATA,
            BASE_TEARDOWN,
            VECTOR_DESTRUCT,
        ] {
            e.register(addr, |_, _| Ret::default());
        }
        e.register(SUB_BUFFER_DATA, |e, a| Ret {
            eax: e.mem.u32(a[0]),
            ..Ret::default()
        });
        e.register(REFERENCE_EXTRA_LIST, |_, a| Ret {
            eax: a[0] + 0x44,
            ..Ret::default()
        });
        // A form's byte at +8 says whether it is a reference, +9 whether its
        // 3D is loaded, +10 what vtable +0x100 answers.
        e.register(0x7000_0f00, |e, a| Ret {
            eax: e.mem.u8(a[0] + 8) as u32,
            ..Ret::default()
        });
        e.register(REFERENCE_LOADED_3D, |e, a| Ret {
            eax: e.mem.u8(a[0] + 9) as u32,
            ..Ret::default()
        });
        e.register(0x7000_1000, |e, a| Ret {
            eax: e.mem.u8(a[0] + 10) as u32,
            ..Ret::default()
        });
        e.map(0x9100_0000, 0x200);
        e.mem.set_u32(0x9100_0000 + 0xf0, 0x7000_0f00);
        e.mem.set_u32(0x9100_0000 + 0x100, 0x7000_1000);
    }

    fn queued_form(e: &mut Engine, reference: u8, loaded: u8, frees: u8) -> u32 {
        let form = e.mem.alloc(0x10);
        e.mem.set_u32(form, 0x9100_0000);
        e.mem.set_u8(form + 8, reference);
        e.mem.set_u8(form + 9, loaded);
        e.mem.set_u8(form + 10, frees);
        form
    }

    fn calls_to(e: &mut Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    #[test]
    fn load_queued_sub_buffers_loads_or_hands_over_by_kind() {
        let mut e = Engine::new();
        let plain = queued_form(&mut e, 0, 0, 0);
        let loaded = queued_form(&mut e, 1, 1, 0);
        let unloaded = queued_form(&mut e, 1, 0, 0);
        let freeing = queued_form(&mut e, 1, 0, 1);
        queue_doubles(&mut e, vec![plain, loaded, unloaded, freeing]);
        let this = e.mem.alloc(0x30);
        e.call_log = Some(vec![]);
        e.call(0x0086_5390, &args![this, 0u32]);
        let loads: Vec<u32> = calls_to(&mut e, LOAD_ANIMATION)
            .iter()
            .map(|a| a[1])
            .collect();
        assert_eq!(loads, vec![plain, loaded]);
        assert_eq!(calls_to(&mut e, SET_SAVED_ANIMATION).len(), 1);
        assert_eq!(calls_to(&mut e, SET_SAVED_ANIMATION)[0][0], unloaded + 0x44);
        assert_eq!(calls_to(&mut e, SUB_BUFFER_FREE).len(), 1);
        assert_eq!(calls_to(&mut e, BASE_TEARDOWN), vec![vec![this]]);

        let mut e = Engine::new();
        let loaded = queued_form(&mut e, 1, 1, 0);
        let unloaded = queued_form(&mut e, 1, 0, 0);
        queue_doubles(&mut e, vec![loaded, unloaded]);
        e.call_log = Some(vec![]);
        e.call(0x0086_5390, &args![this, 1u32]);
        assert_eq!(calls_to(&mut e, LOAD_CHAR_CONTROLLER).len(), 1);
        assert_eq!(calls_to(&mut e, SUB_BUFFER_FREE).len(), 1);
        assert_eq!(calls_to(&mut e, MAP_FIRST_ITEM), vec![vec![this + 0x10]]);

        let mut e = Engine::new();
        let loaded = queued_form(&mut e, 1, 1, 0);
        let unloaded = queued_form(&mut e, 1, 0, 1);
        queue_doubles(&mut e, vec![loaded, unloaded]);
        e.call_log = Some(vec![]);
        e.call(0x0086_5390, &args![this, 2u32]);
        assert_eq!(calls_to(&mut e, LOAD_HAVOK_DATA)[0][1], loaded);
        let saved = calls_to(&mut e, SET_SAVED_HAVOK_DATA);
        assert_eq!(saved.len(), 1);
        assert_eq!(saved[0][0], unloaded + 0x44);
        assert_eq!(calls_to(&mut e, BASE_TEARDOWN), vec![vec![this + 0x20]]);
    }

    #[test]
    fn fn_00865360_loads_all_three_kinds() {
        let mut e = Engine::new();
        queue_doubles(&mut e, vec![]);
        e.register(MAP_FIRST_ITEM, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0086_5360, &args![0x6000u32]);
        assert_eq!(
            calls_to(&mut e, BASE_TEARDOWN),
            vec![vec![0x6000], vec![0x6010], vec![0x6020]]
        );
    }

    #[test]
    fn fn_00865120_copies_the_buffer_behind_version_and_size() {
        let mut e = Engine::new();
        e.register(WORD_AT_C, |e, a| Ret {
            eax: e.mem.u32(a[0] + 0xc),
            ..Ret::default()
        });
        e.register(VARIABLE_SIZE_LENGTH, |_, _| Ret {
            eax: 2,
            ..Ret::default()
        });
        e.register(ALLOCATE_BLOCK, |e, a| Ret {
            eax: e.mem.alloc(a[0]),
            ..Ret::default()
        });
        e.register(BUFFER_GET_VERSION, |_, _| Ret {
            eax: 0x1234,
            ..Ret::default()
        });
        e.register(SAVE_VARIABLE_SIZED_VALUE, |_, _| Ret::default());
        e.register(WORD_AT_4, |e, a| Ret {
            eax: e.mem.u32(a[0] + 4),
            ..Ret::default()
        });
        e.register(COPY_BYTES, |_, _| Ret::default());
        let this = e.mem.alloc(4);
        let buffer = e.mem.alloc(0x24);
        e.call_log = Some(vec![]);
        e.call(0x0086_5120, &args![this, buffer]);
        assert_eq!(e.mem.u32(this), 0, "an empty buffer is not copied");
        assert!(calls_to(&mut e, ALLOCATE_BLOCK).is_empty());

        e.mem.set_u32(buffer + 0xc, 5);
        e.mem.set_u32(buffer + 4, 0x4444);
        e.call(0x0086_5120, &args![this, buffer]);
        let block = e.mem.u32(this);
        assert_eq!(calls_to(&mut e, ALLOCATE_BLOCK), vec![vec![8]]);
        assert_eq!(e.mem.u8(block), 0x34);
        assert_eq!(
            calls_to(&mut e, SAVE_VARIABLE_SIZED_VALUE),
            vec![vec![block + 1, 5]]
        );
        assert_eq!(
            calls_to(&mut e, COPY_BYTES),
            vec![vec![block + 3, 0x4444, 5]]
        );
    }

    #[test]
    fn save_game_writes_the_size_then_the_bytes() {
        let mut e = Engine::new();
        e.register(LOAD_VARIABLE_SIZED_VALUE, |e, a| {
            e.mem.set_u32(a[1], 7);
            Ret {
                eax: 2,
                ..Ret::default()
            }
        });
        e.register(SAVE_GAME_VARIABLE_SIZED_VALUE, |_, _| Ret::default());
        e.register(0x00865ce0, |_, _| Ret::default());
        let this = e.mem.alloc(4);
        e.mem.set_u32(this, 0x9000);
        let save = e.mem.alloc(0x10);
        e.mem.set_u32(save + 0xc, 0x33);
        e.call_log = Some(vec![]);
        e.call(0x0086_5510, &args![this, save]);
        assert_eq!(calls_to(&mut e, LOAD_VARIABLE_SIZED_VALUE)[0][0], 0x9001);
        assert_eq!(
            calls_to(&mut e, SAVE_GAME_VARIABLE_SIZED_VALUE),
            vec![vec![save, 7]]
        );
        assert_eq!(
            calls_to(&mut e, 0x00865ce0),
            vec![vec![save, 0x9003, 7, 0x33, 1]]
        );
    }

    fn saver_doubles(e: &mut Engine) {
        for addr in [
            SAVE_FORM_BUFFER_CONSTRUCT,
            SAVE_FORM_BUFFER_INIT,
            SAVE_FORM_BUFFER_SET_FORM,
            REFERENCE_SAVE_ANIMATION,
            REFERENCE_SAVE_HAVOK_DATA,
            SAVE_FORM_BUFFER_DESTRUCT,
            SAVE_GAME_WARNING,
        ] {
            e.register(addr, |_, _| Ret::default());
        }
        e.register(MANAGER_VERSION, |_, _| Ret {
            eax: 0x1b,
            ..Ret::default()
        });
        e.register(BYTE_AT_4, |e, a| Ret {
            eax: e.mem.u8(a[0] + 4) as u32,
            ..Ret::default()
        });
        e.register(CHANGE_FLAGS_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            Ret {
                eax: a[0],
                ..Ret::default()
            }
        });
        e.register(WORD_AT_C, |e, a| Ret {
            eax: e.mem.u32(a[0] + 0xc),
            ..Ret::default()
        });
        e.register(0x7000_0f00, |e, a| Ret {
            eax: e.mem.u8(a[0] + 8) as u32,
            ..Ret::default()
        });
        e.register(0x7000_1300, |_, _| Ret {
            eax: 0x0abc_0000,
            ..Ret::default()
        });
        e.map(0x9100_0000, 0x200);
        e.mem.set_u32(0x9100_0000 + 0xf0, 0x7000_0f00);
        e.mem.set_u32(0x9100_0000 + 0x130, 0x7000_1300);
        e.map(SAVE_LOAD_MANAGER, 4);
        e.set_global(SAVE_LOAD_MANAGER, 0x6600u32);
    }

    #[test]
    fn fn_00864f20_and_00865020_save_through_a_buffer_or_warn() {
        for (addr, save) in [
            (0x0086_4f20u32, REFERENCE_SAVE_ANIMATION),
            (0x0086_5020, REFERENCE_SAVE_HAVOK_DATA),
        ] {
            let mut e = Engine::new();
            saver_doubles(&mut e);
            let form = e.mem.alloc(0x10);
            e.mem.set_u32(form, 0x9100_0000);
            e.mem.set_u8(form + 4, 0x3b);
            e.mem.set_u32(form + 0xc, 0x0001_2345);
            let this = e.mem.alloc(4);
            e.call_log = Some(vec![]);
            e.call(addr, &args![this, form]);
            assert_eq!(
                calls_to(&mut e, SAVE_GAME_WARNING),
                vec![vec![NOT_A_REFERENCE_FORMAT, 0x0abc_0000, 0x0001_2345]]
            );
            assert!(calls_to(&mut e, save).is_empty());

            e.mem.set_u8(form + 8, 1);
            e.call_log = Some(vec![]);
            e.call(addr, &args![this, form]);
            let init = calls_to(&mut e, SAVE_FORM_BUFFER_INIT);
            let buffer = init[0][0];
            assert_eq!(init[0][1..].to_vec(), vec![0x0001_2345, 4, 0x3b, 0x1b]);
            assert_eq!(calls_to(&mut e, MANAGER_VERSION), vec![vec![0x6600]]);
            assert_eq!(calls_to(&mut e, save), vec![vec![form, buffer]]);
            assert_eq!(
                calls_to(&mut e, SAVE_FORM_BUFFER_DESTRUCT),
                vec![vec![buffer]]
            );
            assert!(calls_to(&mut e, SAVE_GAME_WARNING).is_empty());
        }
    }

    #[test]
    fn fn_00865220_frees_every_queued_buffer_and_destroys_the_maps() {
        let mut e = Engine::new();
        queue_doubles(&mut e, vec![0x10, 0x20]);
        // Each map holds two entries: the queue restarts for each map.
        let mut handed = 0u32;
        e.register_double(MAP_GET_NEXT, move |e, a| {
            handed += 1;
            e.mem.set_u32(a[1], handed % 2);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x0086_5220, &args![0x6000u32]);
        assert_eq!(calls_to(&mut e, SUB_BUFFER_FREE).len(), 6);
        assert_eq!(
            calls_to(&mut e, BASE_TEARDOWN),
            vec![vec![0x6000], vec![0x6010], vec![0x6020]]
        );
        assert_eq!(
            calls_to(&mut e, VECTOR_DESTRUCT),
            vec![vec![0x6000, 0x10, 3, 0x0086_57e0]]
        );
    }
}

#[cfg(test)]
mod n072_tests {
    use super::*;

    fn calls(e: &Engine, address: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == address)
            .map(|c| c.1.clone())
            .collect()
    }

    const FORM_VTABLE: u32 = 0x0150_0000;
    const IS_REFERENCE: u32 = 0x0150_10f0;
    const IS_ACTOR_FC: u32 = 0x0150_10fc;
    const IS_ACTOR_100: u32 = 0x0150_1100;
    const NAME: u32 = 0x0150_1130;
    const ANIMATION_1E4: u32 = 0x0150_11e4;

    /// A form whose slot answers are the words at +4 (`+0xf0`), +8
    /// (`+0xfc`, `+0x100`) and +0xc (`+0x1e4`).
    fn form(e: &mut Engine, reference: u32, actor: u32, animation: u32) -> u32 {
        let f = e.mem.alloc(0x10);
        e.mem.set_u32(f, FORM_VTABLE);
        e.mem.set_u32(f + 4, reference);
        e.mem.set_u32(f + 8, actor);
        e.mem.set_u32(f + 0xc, animation);
        f
    }

    /// Doubles for the load-form buffer creation (the buffer is the new
    /// block) and everything the loads call.
    fn engine() -> Engine {
        let mut e = Engine::new();
        let mut slots = vec![0u32; 0x80];
        slots[0xf0 / 4] = IS_REFERENCE;
        slots[0xfc / 4] = IS_ACTOR_FC;
        slots[0x100 / 4] = IS_ACTOR_100;
        slots[0x130 / 4] = NAME;
        slots[0x1e4 / 4] = ANIMATION_1E4;
        e.put_vtable(FORM_VTABLE, &slots);
        e.register(IS_REFERENCE, |e, a| e.mem.u32(a[0] + 4).into_ret());
        e.register(IS_ACTOR_FC, |e, a| e.mem.u32(a[0] + 8).into_ret());
        e.register(IS_ACTOR_100, |e, a| e.mem.u32(a[0] + 8).into_ret());
        e.register(NAME, |_, _| 0x0160_0000u32.into_ret());
        e.register(ANIMATION_1E4, |e, a| e.mem.u32(a[0] + 0xc).into_ret());
        e.register(WORD_AT_C, |_, _| 0x77u32.into_ret());
        e.register(ALLOCATE_BLOCK, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(LOAD_FORM_BUFFER_CONSTRUCT, |_, a| a[0].into_ret());
        e.register(LOAD_VARIABLE_SIZED_VALUE, |e, a| {
            e.mem.set_u32(a[1], 0x20);
            2u32.into_ret()
        });
        e.map(0x011d_e000, 0x1000);
        e.set_global::<u32>(PLAYER, 0x0170_0000);
        e.register(PLAYER_WORD_007AF430, |_, _| 0x0171_0000u32.into_ret());
        for a in [
            LOAD_FORM_BUFFER_SET_FORM,
            LOAD_FORM_BUFFER_SET_VERSION,
            LOAD_GAME_BUFFER_SET_BUFFER,
            LOAD_FORM_BUFFER_DELETE,
            SAVE_GAME_WARNING,
            ANIMATION_LOAD,
            REFERENCE_LOAD_ANIMATION,
            PROCESS_LOAD_CHAR_CONTROLLER,
            REFERENCE_LOAD_HAVOK_DATA,
        ] {
            e.register(a, |_, _| Ret::default());
        }
        e.call_log = Some(vec![]);
        e
    }

    fn sub_buffer(e: &mut Engine) -> u32 {
        let data = e.mem.alloc(0x30);
        e.mem.set_u8(data, 0x0e);
        let this = e.mem.alloc(4);
        e.mem.set_u32(this, data);
        this
    }

    #[test]
    fn create_load_form_buffer_hands_the_data_over() {
        let mut e = engine();
        let this = sub_buffer(&mut e);
        let data = e.mem.u32(this);
        let buffer = e.call(0x0086_4e20, &args![this, 0x4444u32]).u32();
        assert_eq!(calls(&e, ALLOCATE_BLOCK), vec![vec![0x30]]);
        assert_eq!(
            calls(&e, LOAD_FORM_BUFFER_SET_FORM),
            vec![vec![buffer, 0x4444]]
        );
        assert_eq!(
            calls(&e, LOAD_FORM_BUFFER_SET_VERSION),
            vec![vec![buffer, 0x0e]]
        );
        assert_eq!(calls(&e, LOAD_VARIABLE_SIZED_VALUE)[0][0], data + 1);
        assert_eq!(
            calls(&e, LOAD_GAME_BUFFER_SET_BUFFER),
            vec![vec![buffer, data, 0x23, 3]]
        );
        assert_eq!(e.mem.u32(this), 0);
    }

    #[test]
    fn load_animation_by_kind_of_form() {
        let mut e = engine();
        let this = sub_buffer(&mut e);
        // An actor with an animation.
        let actor = form(&mut e, 1, 1, 0x5500);
        e.call(0x0086_4b00, &args![this, actor]);
        let buffer = calls(&e, ANIMATION_LOAD)[0][1];
        assert_eq!(calls(&e, ANIMATION_LOAD), vec![vec![0x5500, buffer]]);
        assert_eq!(calls(&e, LOAD_FORM_BUFFER_DELETE), vec![vec![buffer, 1]]);
        // An actor without one, a plain reference, not a reference.
        let lone = form(&mut e, 1, 1, 0);
        let reference = form(&mut e, 1, 0, 0);
        let other = form(&mut e, 0, 0, 0);
        for f in [lone, reference, other] {
            let this = sub_buffer(&mut e);
            e.call(0x0086_4b00, &args![this, f]);
        }
        assert_eq!(
            calls(&e, SAVE_GAME_WARNING),
            vec![
                vec![WARNING_NO_ANIMATION, 0x0160_0000, 0x77],
                vec![WARNING_NOT_A_REFERENCE, 0x0160_0000, 0x77]
            ]
        );
        assert_eq!(calls(&e, REFERENCE_LOAD_ANIMATION)[0][0], reference);
    }

    #[test]
    fn load_animation_of_the_player() {
        let mut e = engine();
        // The player is the form; its animation through GetAnimation with
        // the mark set when the form was the player's `007af430` word.
        let player = form(&mut e, 1, 1, 0);
        e.set_global::<u32>(PLAYER, player);
        e.register(PLAYER_GET_ANIMATION, |_, a| (0x6600 + a[1]).into_ret());
        let this = sub_buffer(&mut e);
        e.call(0x0086_4b00, &args![this, 0x0171_0000u32]);
        let this = sub_buffer(&mut e);
        e.call(0x0086_4b00, &args![this, player]);
        assert_eq!(
            calls(&e, PLAYER_GET_ANIMATION),
            vec![vec![player, 1], vec![player, 0]]
        );
        assert_eq!(calls(&e, ANIMATION_LOAD)[0][0], 0x6601);
    }

    #[test]
    fn load_char_controller_needs_an_actor_with_a_controller() {
        let mut e = engine();
        e.register(GET_CHAR_CONTROLLER, |e, a| e.mem.u32(a[0] + 0xc).into_ret());
        e.register(ACTOR_PROCESS, |_, _| 0x9900u32.into_ret());
        let with = form(&mut e, 1, 1, 1);
        let without = form(&mut e, 1, 1, 0);
        let reference = form(&mut e, 1, 0, 0);
        let other = form(&mut e, 0, 0, 0);
        for f in [with, without, reference, other] {
            let this = sub_buffer(&mut e);
            e.call(0x0086_4c50, &args![this, f]);
        }
        assert_eq!(calls(&e, PROCESS_LOAD_CHAR_CONTROLLER).len(), 1);
        assert_eq!(calls(&e, PROCESS_LOAD_CHAR_CONTROLLER)[0][0], 0x9900);
        let warnings: Vec<u32> = calls(&e, SAVE_GAME_WARNING).iter().map(|w| w[0]).collect();
        assert_eq!(
            warnings,
            vec![
                WARNING_NO_CHAR_CONTROLLER,
                WARNING_NOT_AN_ACTOR,
                WARNING_NOT_A_REFERENCE_FOR_CONTROLLER
            ]
        );
        assert_eq!(calls(&e, LOAD_FORM_BUFFER_DELETE).len(), 4);
    }

    #[test]
    fn load_havok_data_needs_a_reference() {
        let mut e = engine();
        let reference = form(&mut e, 1, 0, 0);
        let other = form(&mut e, 0, 0, 0);
        for f in [reference, other] {
            let this = sub_buffer(&mut e);
            e.call(0x0086_4d80, &args![this, f]);
        }
        assert_eq!(calls(&e, REFERENCE_LOAD_HAVOK_DATA)[0][0], reference);
        assert_eq!(
            calls(&e, SAVE_GAME_WARNING),
            vec![vec![WARNING_NOT_A_REFERENCE, 0x0160_0000, 0x77]]
        );
        // A failed allocation leaves no buffer to delete.
        e.register(ALLOCATE_BLOCK, |_, _| 0u32.into_ret());
        e.call_log = Some(vec![]);
        let this = sub_buffer(&mut e);
        e.call(0x0086_4d80, &args![this, reference]);
        assert!(calls(&e, LOAD_FORM_BUFFER_DELETE).is_empty());
    }

    /// The load buffer: vtable slot 0 the version 0x0e, slot 4 its form.
    fn load_buffer(e: &mut Engine, size: u32) -> u32 {
        e.put_vtable(0x0152_0000, &[0x0152_1000, 0x0152_1004]);
        e.register(0x0152_1000, |_, _| 0x0eu32.into_ret());
        e.register(0x0152_1004, |_, _| 0x8888u32.into_ret());
        e.register_double(BUFFER_LOAD_VARIABLE, move |_, _| size.into_ret());
        e.register(VARIABLE_SIZE_LENGTH, |_, _| 2u32.into_ret());
        e.register(UNLOADED_FORM_BUFFER_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            Ret::default()
        });
        e.register(SAVE_VARIABLE_SIZED_VALUE, |_, _| Ret::default());
        e.register(BUFFER_CURRENT, |_, _| 0x0153_0000u32.into_ret());
        e.register(COPY_BYTES, |_, _| Ret::default());
        e.register(BUFFER_SKIP, |_, _| Ret::default());
        e.register(SUB_BUFFER_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], 0);
            Ret::default()
        });
        e.register(SUB_BUFFER_DATA, |e, a| e.mem.u32(a[0]).into_ret());
        e.register(MAP_SET_AT, |_, _| Ret::default());
        let b = e.mem.alloc(4);
        e.mem.set_u32(b, 0x0152_0000);
        b
    }

    #[test]
    fn load_game_copies_the_data_after_a_header() {
        let mut e = engine();
        let buffer = load_buffer(&mut e, 0x10);
        let this = e.mem.alloc(4);
        e.call(0x0086_55a0, &args![this, buffer]);
        let block = e.mem.u32(this);
        assert_eq!(calls(&e, ALLOCATE_BLOCK), vec![vec![0x13]]);
        assert_eq!(e.mem.u8(block), 0x0e);
        assert_eq!(
            calls(&e, SAVE_VARIABLE_SIZED_VALUE),
            vec![vec![block + 1, 0x10]]
        );
        assert_eq!(calls(&e, BUFFER_CURRENT), vec![vec![buffer, 0x10]]);
        assert_eq!(
            calls(&e, COPY_BYTES),
            vec![vec![block + 3, 0x0153_0000, 0x10]]
        );
        assert_eq!(calls(&e, BUFFER_SKIP), vec![vec![buffer, 0x10]]);
    }

    #[test]
    fn queue_sub_buffer_sets_the_data_under_the_form() {
        let mut e = engine();
        let buffer = load_buffer(&mut e, 0x10);
        let this = e.mem.alloc(0x30);
        e.call(0x0086_52f0, &args![this, 2u32, buffer, 0x4444u32]);
        e.call(0x0086_52f0, &args![this, 1u32, buffer, 0u32]);
        let set = calls(&e, MAP_SET_AT);
        assert_eq!(set[0][..2], [this + 0x20, 0x4444]);
        assert_eq!(set[1][..2], [this + 0x10, 0x8888]);
        // No data: nothing is queued.
        let empty = load_buffer(&mut e, 0);
        e.call_log = Some(vec![]);
        e.call(0x0086_52f0, &args![this, 0u32, empty, 0x4444u32]);
        assert!(calls(&e, MAP_SET_AT).is_empty());
    }

    #[test]
    fn the_scalar_deleting_destructors() {
        for address in [0x0086_5690u32, 0x0086_58d0] {
            for flags in [0u32, 1] {
                let mut e = Engine::new();
                for a in [BASE_TEARDOWN, FREE_BLOCK, OPERATOR_DELETE] {
                    e.register(a, |_, _| Ret::default());
                }
                let this = e.mem.alloc(0x10);
                e.call_log = Some(vec![]);
                assert_eq!(e.call(address, &args![this, flags]).u32(), this);
                assert_eq!(e.mem.u32(this), 0x0108_22a8);
                assert_eq!(calls(&e, OPERATOR_DELETE).len() as u32, flags);
            }
        }
    }

    #[test]
    fn fn_00865730_walks_to_the_next_entry() {
        let mut e = Engine::new();
        e.put_vtable(0x0154_0000, &[0, 0x0154_1004]);
        e.register(0x0154_1004, |_, a| (a[1] % 4).into_ret());
        let table = e.mem.alloc(16);
        let map = e.mem.alloc(0x10);
        e.mem.set_u32(map, 0x0154_0000);
        e.mem.set_u32(map + 4, 4);
        e.mem.set_u32(map + 8, table);
        let item = |e: &mut Engine, next: u32, key: u32, value: u32| {
            let i = e.mem.alloc(0xc);
            e.mem.set_u32(i, next);
            e.mem.set_u32(i + 4, key);
            e.mem.set_u32(i + 8, value);
            i
        };
        let last = item(&mut e, 0, 3, 0x33);
        e.mem.set_u32(table + 12, last);
        let second = item(&mut e, 0, 1, 0x11);
        let first = item(&mut e, second, 5, 0x55);
        e.mem.set_u32(table + 4, first);
        let slots = e.mem.alloc(12);
        let (position, key, value) = (slots, slots + 4, slots + 8);
        e.mem.set_u32(position, first);
        let mut seen = vec![];
        while e.mem.u32(position) != 0 {
            e.call(0x0086_5730, &args![map, position, key, value]);
            seen.push((e.mem.u32(key), e.mem.u32(value)));
        }
        assert_eq!(seen, vec![(5, 0x55), (1, 0x11), (3, 0x33)]);
    }

    #[test]
    fn fn_00865890_clears_the_value_and_frees_the_item() {
        let mut e = Engine::new();
        e.register(CHANGE_FLAGS_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], a[1] + 0x40);
            Ret::default()
        });
        e.register(MAP_FREE_ITEM, |_, _| Ret::default());
        let this = e.mem.alloc(0x10);
        let item = e.mem.alloc(0xc);
        e.mem.set_u32(item + 8, 0xffff);
        e.call_log = Some(vec![]);
        e.call(0x0086_5890, &args![this, item]);
        assert_eq!(e.mem.u32(item + 8), 0x40);
        assert_eq!(calls(&e, MAP_FREE_ITEM), vec![vec![this + 0xc, item]]);
    }

    #[test]
    fn fn_00865900_makes_an_item() {
        let mut e = Engine::new();
        e.register(ALLOCATE_BLOCK, |_, _| 0x5000u32.into_ret());
        e.register(MAP_ITEM_CONSTRUCT, |_, a| a[0].into_ret());
        assert_eq!(e.call(0x0086_5900, &args![0x10u32]).u32(), 0x5000);
        e.register(ALLOCATE_BLOCK, |_, _| 0u32.into_ret());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0086_5900, &args![0x10u32]).u32(), 0);
        assert!(calls(&e, MAP_ITEM_CONSTRUCT).is_empty());
    }
}

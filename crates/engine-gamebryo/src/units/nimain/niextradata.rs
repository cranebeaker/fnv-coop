//! `NiMain/niextradata.cpp` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `NiExtraData` (vtable `0109da1c`, RTTI `NiExtraData`): an `NiObject` with its name, a
//! fixed-string handle (count at `handle - 8`), at `+0x08` (`m_kName`, Xbox PDB).

#[allow(unused_imports)]
use crate::prelude::*;

use crate::units::nimain::nimainsdm::{
    acquire_fixed_string, release_fixed_string, ALLOCATE, EMPTY_FIXED_STRING,
};

/// The `NiExtraData` vtable (RTTI `.?AVNiExtraData@@`).
const EXTRA_DATA_VTABLE: u32 = 0x0109_da1c;
/// `00a5d3a0` (`thiscall`, unit `niobject.cpp`): the `NiObject` constructor.
const OBJECT_CONSTRUCTOR: u32 = 0x00a5_d3a0;
/// `00a5d3d0` (`thiscall`, unit `niobject.cpp`): the `NiObject` destructor body.
const OBJECT_DESTRUCTOR: u32 = 0x00a5_d3d0;
/// `00a5d450` (`thiscall`, `RET 8`, unit `niobject.cpp`): the base member copy.
const OBJECT_COPY_MEMBERS: u32 = 0x00a5_d450;
/// `NiObject::LoadBinary` (`00a5d610`, `thiscall`, `RET 4`).
const OBJECT_LOAD_BINARY: u32 = 0x00a5_d610;
/// `NiStream::GetObjectFromLinkID` (`00a63430`, `thiscall` on the stream, no stack word).
const GET_OBJECT_FROM_LINK_ID: u32 = 0x00a6_3430;
/// `00a645b0` (`thiscall` on the stream, no stack word, unit `nistream.cpp`).
const STREAM_00A645B0: u32 = 0x00a6_45b0;
/// `00aa1070` (`cdecl`, one word): allocates a block of that size.
const ALLOCATE_BLOCK: u32 = 0x00aa_1070;
/// `NiStreamLoadBinary<unsigned char>` (`00a471d0`, `cdecl`: stream, buffer, count).
const LOAD_BYTES: u32 = 0x00a4_71d0;
/// `00539f50` (`thiscall`, `RET 8`; engine-map unit `tesland.cpp`, a body shared by
/// identical-code folding): run on the new 0x14-byte block with the size and the buffer.
const BLOCK_00539F50: u32 = 0x0053_9f50;
/// `NiStream::LoadCStringAsFixedString` (`00a64ea0`) and `NiStream::LoadFixedString`
/// (`00a64df0`), both `thiscall` on the stream with the address of the handle.
const LOAD_C_STRING_AS_FIXED_STRING: u32 = 0x00a6_4ea0;
const LOAD_FIXED_STRING: u32 = 0x00a6_4df0;
/// The record `GetRTTI` (vtable `+8`) answers for an `NiExtraData` itself (`011f4a80`).
const EXTRA_DATA_RTTI: u32 = 0x011f_4a80;
/// The stream's file version (`NiStream`, `+0xd8`) and its reader (`+0x24c`).
const STREAM_VERSION: u32 = 0xd8;
const STREAM_READER: u32 = 0x24c;

// Translated from 00a7b2e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map; by its vtable the `NiExtraData` default constructor
/// (`thiscall`): the `NiObject` constructor (`00a5d3a0`), the vtable `0109da1c`, and the
/// name set to the empty string's handle (`0109b220`, not counted). Answers `this`.
pub fn fn_00a7b2e0(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(OBJECT_CONSTRUCTOR, &args![this]);
    e.mem.set_u32(this.addr(), EXTRA_DATA_VTABLE);
    let empty = e.mem.u32(EMPTY_FIXED_STRING);
    e.mem.set_u32(this.addr() + 8, empty);
    this
}

// Translated from 00a7b300 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map; the `NiExtraData` destructor body (`thiscall`): sets the
/// vtable `0109da1c`, releases the name (`InterlockedDecrement` at `handle - 8` unless it
/// is the empty string's) and ends in the `NiObject` destructor (`00a5d3d0`).
pub fn fn_00a7b300(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), EXTRA_DATA_VTABLE);
    let name = e.mem.u32(this.addr() + 8);
    release_fixed_string(e, name);
    e.call(OBJECT_DESTRUCTOR, &args![this]);
}

// Translated from 00a7b330 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map; the `NiExtraData` member copy (`thiscall`, `RET 8`): runs
/// the base copy `00a5d450` with `(dest, cloning)`, then gives `dest` the name of `this`
/// (counted, and the old name of `dest` released) when the two differ.
pub fn fn_00a7b330(e: &mut Engine, this: Ptr, dest: Ptr, cloning: u32) {
    e.call(OBJECT_COPY_MEMBERS, &args![this, dest, cloning]);
    let name = e.mem.u32(this.addr() + 8);
    let held = dest.addr() + 8;
    if e.mem.u32(held) != name {
        acquire_fixed_string(e, name);
        let old = e.mem.u32(held);
        release_fixed_string(e, old);
        e.mem.set_u32(held, name);
    }
}

// Translated from 00a7b3b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiExtraData::LinkObject` (Xbox PDB; `thiscall`, `RET 4`): for a stream older than
/// version `0x0500000b` the word at `+8` becomes the stream's next linked object
/// (`NiStream::GetObjectFromLinkID`, `00a63430`); newer streams change nothing.
pub fn ni_extra_data_link_object(e: &mut Engine, this: Ptr, stream: Ptr) {
    if e.mem.u32(stream.addr() + STREAM_VERSION) < 0x0500_000b {
        let object = e.call(GET_OBJECT_FROM_LINK_ID, &args![stream]).u32();
        e.mem.set_u32(this.addr() + 8, object);
    }
}

// Translated from 00a7b440 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map; the `NiExtraData` constructor from a name (`thiscall`,
/// `RET 4`, the address of a fixed-string handle): the `NiObject` constructor, the vtable
/// `0109da1c`, the empty name, then the given name (counted) when it is not the empty
/// string's. Answers `this`.
pub fn fn_00a7b440(e: &mut Engine, this: Ptr, name: Ptr) -> Ptr {
    e.call(OBJECT_CONSTRUCTOR, &args![this]);
    e.mem.set_u32(this.addr(), EXTRA_DATA_VTABLE);
    let empty = e.mem.u32(EMPTY_FIXED_STRING);
    let held = this.addr() + 8;
    e.mem.set_u32(held, empty);
    let handle = e.mem.u32(name.addr());
    if empty != handle {
        acquire_fixed_string(e, handle);
        let old = e.mem.u32(held);
        release_fixed_string(e, old);
        e.mem.set_u32(held, handle);
    }
    this
}

// Translated from 00a7b4a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiExtraData::LoadBinary` (Xbox PDB; `thiscall`, `RET 4`): `NiObject::LoadBinary`
/// first. For a stream older than `0x0500000b`: `00a645b0` on the stream, then the reader
/// at `stream + 0x24c` reads one 4-byte size (the function pointer at its `+8`, `cdecl`
/// `(reader, buffer, 4, &4, 1)`, the answer added to the reader's word at `+4`; the buffer
/// is the stack slot of the stream argument). When the size is not 0, `this` is not null
/// and `this`'s `GetRTTI` (slot `+8`) answers `011f4a80`: a buffer of that size
/// (`00aa1070`) is filled by `NiStreamLoadBinary<unsigned char>` (`00a471d0`), a 0x14-byte
/// block is allocated (`00aa13e0`) and, when it is not null, `00539f50` runs on it with
/// `(size, buffer)`; a non-null answer goes to the stream's slot `+0x28`. Streams from
/// `0x0500000b` read the name: as a C string (`00a64ea0`) before `0x14010001`, as a fixed
/// string (`00a64df0`) from it.
pub fn ni_extra_data_load_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(OBJECT_LOAD_BINARY, &args![this, stream]);
    let version = e.mem.u32(stream.addr() + STREAM_VERSION);
    if version >= 0x0500_000b {
        let name = this.addr().wrapping_add(8);
        if version < 0x1401_0001 {
            e.call(LOAD_C_STRING_AS_FIXED_STRING, &args![stream, name]);
        } else {
            e.call(LOAD_FIXED_STRING, &args![stream, name]);
        }
        return;
    }
    e.call(STREAM_00A645B0, &args![stream]);
    let reader = e.mem.u32(stream.addr() + STREAM_READER);
    let size = e.with_stack(8, |e, slots| {
        // The argument slot (still holding the stream pointer) and the local count.
        let buffer = slots.addr();
        let count = buffer + 4;
        e.mem.set_u32(buffer, stream.addr());
        e.mem.set_u32(count, 4);
        let read_function = e.mem.u32(reader + 8);
        let read = e
            .call(read_function, &args![reader, buffer, 4u32, count, 1u32])
            .u32();
        let total = e.mem.u32(reader + 4).wrapping_add(read);
        e.mem.set_u32(reader + 4, total);
        e.mem.u32(buffer)
    });
    if size == 0 || this.is_null() {
        return;
    }
    if e.vcall(this.addr(), 8, &[]).u32() != EXTRA_DATA_RTTI {
        return;
    }
    let buffer = e.call(ALLOCATE_BLOCK, &args![size]).u32();
    e.call(LOAD_BYTES, &args![stream, buffer, size]);
    let block = e.call(ALLOCATE, &args![0x14u32]).u32();
    if block == 0 {
        return;
    }
    let made = e.call(BLOCK_00539F50, &args![block, size, buffer]).u32();
    if made != 0 {
        e.vcall(stream.addr(), 0x28, &args![made]);
    }
}

/// `NiObject::RegisterStreamables` (`00a5d2f0`, the stream) and `NiObject::IsEqual`
/// (`00a5d470`, the other object), both `thiscall`.
const OBJECT_REGISTER_STREAMABLES: u32 = 0x00a5_d2f0;
const OBJECT_IS_EQUAL: u32 = 0x00a5_d470;
/// `00a89af0` (unnamed and unplaced in the engine map; `thiscall`, the stream): the base
/// save `NiExtraData::SaveBinary` starts with.
const OBJECT_SAVE_BINARY_00A89AF0: u32 = 0x00a8_9af0;
/// `NiStream::SaveFixedString` (`00a641f0`, `thiscall` on the stream, the address of the
/// handle).
const SAVE_FIXED_STRING: u32 = 0x00a6_41f0;
/// The sized delete (`00aa1460`, `cdecl`: block, size).
const SIZED_DELETE: u32 = 0x00aa_1460;
/// `NiGetViewerString` (`00a7d330`, the class name) and `NiGetViewerString_ov11`
/// (`00a7d5e0`, a name and a fixed-string handle), both `cdecl`, answering a new string.
const VIEWER_STRING_CLASS: u32 = 0x00a7_d330;
const VIEWER_STRING_FIXED_STRING: u32 = 0x00a7_d5e0;
/// The string array's grow (`0096ad30`, `thiscall`, the new size) and set-at (`0096ae90`,
/// `thiscall`, (index, address of the string)), both unnamed in the engine map.
const STRINGS_GROW: u32 = 0x0096_ad30;
const STRINGS_SET_AT: u32 = 0x0096_ae90;
/// The literal `"m_pcName"`.
const TEXT_M_PC_NAME: u32 = 0x0109_7790;

/// Appends `text` to a viewer-string array as the exe's inlined `Add` does: when the 16-bit
/// size (+0xa) has reached the capacity (+0x8) the array grows to its step (+0xe) plus the
/// size, then the string is stored at the old size (through a stack word holding it).
fn push_viewer_string(e: &mut Engine, strings: Ptr, text: u32) {
    let size = u32::from(e.mem.u16(strings.addr() + 0xa));
    if size >= u32::from(e.mem.u16(strings.addr() + 8)) {
        let step = u32::from(e.mem.u16(strings.addr() + 0xe));
        e.call(STRINGS_GROW, &args![strings, step + size]);
    }
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), text);
        e.call(STRINGS_SET_AT, &args![strings, size, slot]);
    });
}

// Translated from 00a7b290 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiExtraData::RegisterStreamables` (Xbox PDB): false when
/// `NiObject::RegisterStreamables` is false; otherwise whether the stream's virtual slot
/// +0x20 (with the address of the name, +0x8) answers nonzero.
pub fn ni_extra_data_register_streamables(e: &mut Engine, this: Ptr, stream: Ptr) -> bool {
    if !e
        .call(OBJECT_REGISTER_STREAMABLES, &args![this, stream])
        .bool()
    {
        return false;
    }
    e.vcall(stream.addr(), 0x20, &args![this.addr() + 8]).bool()
}

// Translated from 00a7b2c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiExtraData::SaveBinary` (Xbox PDB): the base save (`00a89af0`), then the name
/// (`NiStream::SaveFixedString` with its address, +0x8).
pub fn ni_extra_data_save_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(OBJECT_SAVE_BINARY_00A89AF0, &args![this, stream]);
    e.call(SAVE_FIXED_STRING, &args![stream, this.addr() + 8]);
}

// Translated from 00a7b380 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (a function Ghidra did not create; by its body the creation
/// function of `NiExtraData`): 0xc bytes (`00aa13e0`) with the default constructor's body
/// inlined (the `NiObject` constructor, the vtable `0109da1c`, the empty name, not
/// counted), or null.
pub fn fn_00a7b380(e: &mut Engine) -> Ptr {
    let block = e.call(ALLOCATE, &args![0xcu32]).u32();
    if block == 0 {
        return Ptr::new(0);
    }
    e.call(OBJECT_CONSTRUCTOR, &args![block]);
    e.mem.set_u32(block, EXTRA_DATA_VTABLE);
    let empty = e.mem.u32(EMPTY_FIXED_STRING);
    e.mem.set_u32(block + 8, empty);
    Ptr::new(block)
}

// Translated from 00a7b3d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiExtraData::IsEqual` (Xbox PDB): `NiObject::IsEqual`, then whether the two name
/// handles (+0x8) are the same.
pub fn ni_extra_data_is_equal(e: &mut Engine, this: Ptr, other: Ptr) -> bool {
    if !e.call(OBJECT_IS_EQUAL, &args![this, other]).bool() {
        return false;
    }
    e.mem.u32(this.addr() + 8) == e.mem.u32(other.addr().wrapping_add(8))
}

// Translated from 00a7b400 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiExtraData::_scalar_deleting_destructor_` (Xbox PDB): the destructor body (`00a7b300`:
/// vtable, the name released, the `NiObject` destructor) inlined, then the sized delete of
/// 0xc bytes when bit 0 of `flags` is set. Answers `this`.
pub fn ni_extra_data_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_00a7b300(e, this);
    if flags & 1 != 0 {
        e.call(SIZED_DELETE, &args![this, 0xcu32]);
    }
    this
}

// Translated from 00a7b580 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiExtraData::GetViewerStrings` (Xbox PDB): the class name (`NiGetViewerString` of
/// `ms_RTTI`'s name word, `011f4a80`) and `"m_pcName"` with the name handle
/// (`NiGetViewerString_ov11`), appended to the array. No base class strings.
pub fn ni_extra_data_get_viewer_strings(e: &mut Engine, this: Ptr, strings: Ptr) {
    let name = e.mem.u32(EXTRA_DATA_RTTI);
    let text = e.call(VIEWER_STRING_CLASS, &args![name]).u32();
    push_viewer_string(e, strings, text);
    let handle = e.mem.u32(this.addr() + 8);
    let text = e
        .call(VIEWER_STRING_FIXED_STRING, &args![TEXT_M_PC_NAME, handle])
        .u32();
    push_viewer_string(e, strings, text);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a7_b2e0, fn_00a7b2e0(Ptr) -> Ptr),
        entry!(0x00a7_b300, fn_00a7b300(Ptr)),
        entry!(0x00a7_b330, fn_00a7b330(Ptr, Ptr, u32)),
        entry!(0x00a7_b3b0, ni_extra_data_link_object(Ptr, Ptr)),
        entry!(0x00a7_b440, fn_00a7b440(Ptr, Ptr) -> Ptr),
        entry!(0x00a7_b4a0, ni_extra_data_load_binary(Ptr, Ptr)),
        entry!(
            0x00a7_b290,
            ni_extra_data_register_streamables(Ptr, Ptr) -> bool
        ),
        entry!(0x00a7_b2c0, ni_extra_data_save_binary(Ptr, Ptr)),
        entry!(0x00a7_b380, fn_00a7b380() -> Ptr),
        entry!(0x00a7_b3d0, ni_extra_data_is_equal(Ptr, Ptr) -> bool),
        entry!(
            0x00a7_b400,
            ni_extra_data_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(0x00a7_b580, ni_extra_data_get_viewer_strings(Ptr, Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::nimain::nimainsdm::test_support::*;

    const EMPTY: u32 = 0x0109_b228;

    /// A counted fixed-string handle: the count word sits 8 bytes before it.
    fn handle(e: &mut Engine, count: u32) -> u32 {
        let block = e.mem.alloc(0x10);
        e.mem.set_u32(block, count);
        block + 8
    }

    fn count(e: &Engine, handle: u32) -> u32 {
        e.mem.u32(handle - 8)
    }

    #[test]
    fn default_construction_sets_the_vtable_and_the_empty_name() {
        let mut e = engine_with_counting();
        e.register(OBJECT_CONSTRUCTOR, |_, a| a[0].into_ret());
        e.call_log = Some(vec![]);
        let this = e.mem.alloc(0xc);
        assert_eq!(e.call(0x00a7_b2e0, &args![this]).u32(), this);
        assert_eq!(calls_to(&e, OBJECT_CONSTRUCTOR), vec![vec![this]]);
        assert_eq!(e.mem.u32(this), EXTRA_DATA_VTABLE);
        assert_eq!(e.mem.u32(this + 8), EMPTY);
    }

    #[test]
    fn destruction_releases_a_counted_name_only() {
        let mut e = engine_with_counting();
        e.register(OBJECT_DESTRUCTOR, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        let name = handle(&mut e, 2);
        let this = e.mem.alloc(0xc);
        e.mem.set_u32(this + 8, name);
        e.call(0x00a7_b300, &args![this]);
        assert_eq!(count(&e, name), 1);
        assert_eq!(e.mem.u32(this), EXTRA_DATA_VTABLE);
        e.mem.set_u32(this + 8, EMPTY);
        e.call(0x00a7_b300, &args![this]);
        assert_eq!(calls_to(&e, OBJECT_DESTRUCTOR), vec![vec![this]; 2]);
        assert_eq!(calls_to(&e, 0x00fd_f1c4).len(), 1);
    }

    #[test]
    fn copying_gives_the_destination_the_name() {
        let mut e = engine_with_counting();
        e.register(OBJECT_COPY_MEMBERS, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        let mine = handle(&mut e, 1);
        let theirs = handle(&mut e, 1);
        let this = e.mem.alloc(0xc);
        let dest = e.mem.alloc(0xc);
        e.mem.set_u32(this + 8, mine);
        e.mem.set_u32(dest + 8, theirs);
        e.call(0x00a7_b330, &args![this, dest, 7u32]);
        assert_eq!(calls_to(&e, OBJECT_COPY_MEMBERS), vec![vec![this, dest, 7]]);
        assert_eq!(e.mem.u32(dest + 8), mine);
        assert_eq!(count(&e, mine), 2);
        assert_eq!(count(&e, theirs), 0);
        // The same name again: nothing counted.
        e.call(0x00a7_b330, &args![this, dest, 7u32]);
        assert_eq!(count(&e, mine), 2);
    }

    #[test]
    fn linking_reads_the_object_only_from_old_streams() {
        let mut e = engine_with_counting();
        e.register(GET_OBJECT_FROM_LINK_ID, |_, _| 0x4444u32.into_ret());
        let this = e.mem.alloc(0xc);
        let stream = e.mem.alloc(0x250);
        e.mem.set_u32(stream + STREAM_VERSION, 0x0500_000a);
        e.call(0x00a7_b3b0, &args![this, stream]);
        assert_eq!(e.mem.u32(this + 8), 0x4444);
        e.mem.set_u32(this + 8, 0);
        e.mem.set_u32(stream + STREAM_VERSION, 0x0500_000b);
        e.call(0x00a7_b3b0, &args![this, stream]);
        assert_eq!(e.mem.u32(this + 8), 0);
    }

    #[test]
    fn construction_from_a_name_counts_it() {
        let mut e = engine_with_counting();
        e.register(OBJECT_CONSTRUCTOR, |_, a| a[0].into_ret());
        let name = handle(&mut e, 1);
        let holder = e.mem.alloc(4);
        e.mem.set_u32(holder, name);
        let this = e.mem.alloc(0xc);
        assert_eq!(e.call(0x00a7_b440, &args![this, holder]).u32(), this);
        assert_eq!(e.mem.u32(this), EXTRA_DATA_VTABLE);
        assert_eq!(e.mem.u32(this + 8), name);
        assert_eq!(count(&e, name), 2);
        e.mem.set_u32(holder, EMPTY);
        e.call(0x00a7_b440, &args![this, holder]);
        assert_eq!(e.mem.u32(this + 8), EMPTY);
    }

    const READ: u32 = 0x0300_1000;
    const THIS_VTABLE: u32 = 0x0300_2000;
    const GET_RTTI: u32 = 0x0300_3000;
    const STREAM_VTABLE: u32 = 0x0300_4000;
    const REGISTER_OBJECT: u32 = 0x0300_5000;

    /// An old-version stream whose reader answers `size` (and 4 bytes read).
    fn load_fixture(size: u32, rtti: u32, made: u32) -> (Engine, u32, u32) {
        let mut e = engine_with_counting();
        e.register(OBJECT_LOAD_BINARY, |_, _| Ret::default());
        e.register(STREAM_00A645B0, |_, _| Ret::default());
        e.register(ALLOCATE_BLOCK, |_, _| 0x6000u32.into_ret());
        e.register(LOAD_BYTES, |_, _| Ret::default());
        e.register(REGISTER_OBJECT, |_, _| Ret::default());
        e.register_double(BLOCK_00539F50, move |_, _| made.into_ret());
        e.register_double(GET_RTTI, move |_, _| rtti.into_ret());
        e.register_double(READ, move |e, a| {
            e.mem.set_u32(a[1], size);
            4u32.into_ret()
        });
        e.put_vtable(THIS_VTABLE, &[0, 0, GET_RTTI]);
        let mut slots = vec![0u32; 11];
        slots[10] = REGISTER_OBJECT;
        e.put_vtable(STREAM_VTABLE, &slots);
        let reader = e.mem.alloc(0xc);
        e.mem.set_u32(reader + 8, READ);
        e.mem.set_u32(reader + 4, 0x10);
        let stream = e.mem.alloc(0x250);
        e.mem.set_u32(stream, STREAM_VTABLE);
        e.mem.set_u32(stream + STREAM_VERSION, 0x0400_0002);
        e.mem.set_u32(stream + STREAM_READER, reader);
        let this = e.mem.alloc(0xc);
        e.mem.set_u32(this, THIS_VTABLE);
        e.call_log = Some(vec![]);
        (e, this, stream)
    }

    #[test]
    fn loading_from_an_old_stream_reads_the_size_and_the_bytes() {
        let (mut e, this, stream) = load_fixture(0x20, EXTRA_DATA_RTTI, 0x7000);
        e.call(0x00a7_b4a0, &args![this, stream]);
        let reader = e.mem.u32(stream + STREAM_READER);
        assert_eq!(e.mem.u32(reader + 4), 0x14);
        let reads = calls_to(&e, READ);
        assert_eq!(reads.len(), 1);
        assert_eq!((reads[0][0], reads[0][2], reads[0][4]), (reader, 4, 1));
        assert_eq!(calls_to(&e, LOAD_BYTES), vec![vec![stream, 0x6000, 0x20]]);
        let blocks = calls_to(&e, BLOCK_00539F50);
        assert_eq!(blocks.len(), 1);
        assert_eq!((blocks[0][1], blocks[0][2]), (0x20, 0x6000));
        assert_eq!(calls_to(&e, REGISTER_OBJECT), vec![vec![stream, 0x7000]]);
    }

    #[test]
    fn loading_stops_on_a_zero_size_another_class_or_a_null_object() {
        let (mut e, this, stream) = load_fixture(0, EXTRA_DATA_RTTI, 0x7000);
        e.call(0x00a7_b4a0, &args![this, stream]);
        assert!(calls_to(&e, GET_RTTI).is_empty());
        let (mut e, this, stream) = load_fixture(8, 0x1234, 0x7000);
        e.call(0x00a7_b4a0, &args![this, stream]);
        assert!(calls_to(&e, ALLOCATE_BLOCK).is_empty());
        let (mut e, this, stream) = load_fixture(8, EXTRA_DATA_RTTI, 0);
        e.call(0x00a7_b4a0, &args![this, stream]);
        assert!(calls_to(&e, REGISTER_OBJECT).is_empty());
        let (mut e, _, stream) = load_fixture(8, EXTRA_DATA_RTTI, 0x7000);
        e.call(0x00a7_b4a0, &args![0u32, stream]);
        assert!(calls_to(&e, GET_RTTI).is_empty());
    }

    #[test]
    fn loading_from_newer_streams_reads_the_name() {
        let mut e = engine_with_counting();
        e.register(OBJECT_LOAD_BINARY, |_, _| Ret::default());
        e.register(LOAD_C_STRING_AS_FIXED_STRING, |_, _| Ret::default());
        e.register(LOAD_FIXED_STRING, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        let this = e.mem.alloc(0xc);
        let stream = e.mem.alloc(0x250);
        e.mem.set_u32(stream + STREAM_VERSION, 0x1401_0000);
        e.call(0x00a7_b4a0, &args![this, stream]);
        e.mem.set_u32(stream + STREAM_VERSION, 0x1401_0001);
        e.call(0x00a7_b4a0, &args![this, stream]);
        assert_eq!(
            calls_to(&e, LOAD_C_STRING_AS_FIXED_STRING),
            vec![vec![stream, this + 8]]
        );
        assert_eq!(
            calls_to(&e, LOAD_FIXED_STRING),
            vec![vec![stream, this + 8]]
        );
        assert_eq!(
            calls_to(&e, OBJECT_LOAD_BINARY),
            vec![vec![this, stream]; 2]
        );
    }
}

#[cfg(test)]
mod n016_tests {
    use super::*;
    use crate::units::nimain::nimainsdm::test_support::*;

    fn engine() -> Engine {
        let mut e = engine_with_counting();
        for double in [
            OBJECT_CONSTRUCTOR,
            OBJECT_DESTRUCTOR,
            OBJECT_SAVE_BINARY_00A89AF0,
            SAVE_FIXED_STRING,
            SIZED_DELETE,
            STRINGS_GROW,
            STRINGS_SET_AT,
        ] {
            e.register(double, |_, _| Ret::default());
        }
        e.call_log = Some(vec![]);
        e
    }

    #[test]
    fn register_streamables_asks_the_stream_about_the_name() {
        let mut e = engine();
        e.register(OBJECT_REGISTER_STREAMABLES, |_, _| 1u32.into_ret());
        let mut slots = vec![0u32; 0x10];
        slots[0x20 / 4] = 0x0e00_0020;
        e.put_vtable(0x0e10_0000, &slots);
        e.register(0x0e00_0020, |_, _| 1u32.into_ret());
        let stream = e.mem.alloc(4);
        e.mem.set_u32(stream, 0x0e10_0000);
        let this = e.mem.alloc(0xc);
        assert!(e.call(0x00a7_b290, &args![this, stream]).bool());
        assert_eq!(calls_to(&e, 0x0e00_0020), vec![vec![stream, this + 8]]);
        // Only the low byte of the answer counts.
        e.register(0x0e00_0020, |_, _| 0x100u32.into_ret());
        assert!(!e.call(0x00a7_b290, &args![this, stream]).bool());
        e.register(OBJECT_REGISTER_STREAMABLES, |_, _| 0u32.into_ret());
        assert!(!e.call(0x00a7_b290, &args![this, stream]).bool());
        assert_eq!(calls_to(&e, 0x0e00_0020).len(), 2);
    }

    #[test]
    fn save_binary_saves_the_base_then_the_name() {
        let mut e = engine();
        let this = e.mem.alloc(0xc);
        e.call(0x00a7_b2c0, &args![this, 0x0e20_0000u32]);
        assert_eq!(
            calls_to(&e, OBJECT_SAVE_BINARY_00A89AF0),
            vec![vec![this, 0x0e20_0000]]
        );
        assert_eq!(
            calls_to(&e, SAVE_FIXED_STRING),
            vec![vec![0x0e20_0000, this + 8]]
        );
    }

    #[test]
    fn create_object_builds_an_empty_named_extra_data() {
        let mut e = engine();
        let object = e.call(0x00a7_b380, &args![]).u32();
        assert_eq!(calls_to(&e, ALLOCATE), vec![vec![0xc]]);
        assert_eq!(calls_to(&e, OBJECT_CONSTRUCTOR), vec![vec![object]]);
        assert_eq!(e.mem.u32(object), EXTRA_DATA_VTABLE);
        assert_eq!(e.mem.u32(object + 8), 0x0109_b228);
        assert_eq!(e.mem.u32(0x0109_b220), 0x0109_b228);
        e.register(ALLOCATE, |_, _| 0u32.into_ret());
        assert_eq!(e.call(0x00a7_b380, &args![]).u32(), 0);
    }

    #[test]
    fn is_equal_compares_the_names_after_the_base() {
        let mut e = engine();
        e.register(OBJECT_IS_EQUAL, |_, _| 1u32.into_ret());
        let this = e.mem.alloc(0xc);
        let other = e.mem.alloc(0xc);
        e.mem.set_u32(this + 8, 0x0e00_0010);
        e.mem.set_u32(other + 8, 0x0e00_0010);
        assert!(e.call(0x00a7_b3d0, &args![this, other]).bool());
        e.mem.set_u32(other + 8, 0x0e00_0020);
        assert!(!e.call(0x00a7_b3d0, &args![this, other]).bool());
        e.register(OBJECT_IS_EQUAL, |_, _| 0u32.into_ret());
        assert!(!e.call(0x00a7_b3d0, &args![this, this]).bool());
    }

    #[test]
    fn scalar_deleting_destructor_releases_the_name_and_deletes_on_bit_0() {
        let mut e = engine();
        let handle = e.mem.alloc(0x10) + 8;
        e.mem.set_u32(handle - 8, 2);
        let this = e.mem.alloc(0xc);
        e.mem.set_u32(this + 8, handle);
        assert_eq!(e.call(0x00a7_b400, &args![this, 1u32]).u32(), this);
        assert_eq!(e.mem.u32(this), EXTRA_DATA_VTABLE);
        assert_eq!(e.mem.u32(handle - 8), 1);
        assert_eq!(calls_to(&e, OBJECT_DESTRUCTOR), vec![vec![this]]);
        assert_eq!(calls_to(&e, SIZED_DELETE), vec![vec![this, 0xc]]);
        e.call(0x00a7_b400, &args![this, 0u32]);
        assert_eq!(calls_to(&e, SIZED_DELETE).len(), 1);
    }

    #[test]
    fn get_viewer_strings_adds_the_class_and_the_name() {
        let mut e = engine();
        e.mem.set_u32(EXTRA_DATA_RTTI, 0x0123_0000);
        e.register(VIEWER_STRING_CLASS, |_, _| 0x0e00_0001u32.into_ret());
        e.register(VIEWER_STRING_FIXED_STRING, |_, _| 0x0e00_0002u32.into_ret());
        let this = e.mem.alloc(0xc);
        e.mem.set_u32(this + 8, 0x0e00_0040);
        let strings = e.mem.alloc(0x10);
        e.call(0x00a7_b580, &args![this, strings]);
        assert_eq!(calls_to(&e, VIEWER_STRING_CLASS), vec![vec![0x0123_0000]]);
        assert_eq!(
            calls_to(&e, VIEWER_STRING_FIXED_STRING),
            vec![vec![TEXT_M_PC_NAME, 0x0e00_0040]]
        );
        // Capacity 0: both appends grow (step 0 plus the size 0).
        assert_eq!(calls_to(&e, STRINGS_GROW), vec![vec![strings, 0]; 2]);
        assert_eq!(calls_to(&e, STRINGS_SET_AT).len(), 2);
    }
}

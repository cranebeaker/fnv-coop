//! `NiAnimation/bstreadtransfinterpolator.cpp` (Xbox PDB source unit), subsystem `NiAnimation`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `BSTreadTransfInterpolator::ms_RTTI` (the class's `NiRTTI` record).
const BS_TREAD_TRANSF_INTERPOLATOR_RTTI: u32 = 0x011f_3ed8;

// Translated from 00a43360 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSTreadTransfInterpolator::GetRTTI` (Xbox PDB): the address of the class's `NiRTTI` record.
pub fn bs_tread_transf_interpolator_get_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(BS_TREAD_TRANSF_INTERPOLATOR_RTTI)
}

/// `00c85800`: the jump target of `00a43340`.
const JUMP_TARGET_00A43340: u32 = 0x00c8_5800;

// Translated from 00a43340 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSTreadTransfInterpolator::IsEqual` (Xbox PDB): a 5-byte jump to `00c85800`, passing on `this` and its one argument; returns the target result. Return type (bool): Xbox PDB public name `?IsEqual@BSTreadTransfInterpolator@@UAA_NPAVNiObject@@@Z`.
pub fn bs_tread_transf_interpolator_is_equal(e: &mut Engine, this: Ptr, other: Ptr) -> bool {
    e.call(JUMP_TARGET_00A43340, &args![this, other]).bool()
}

/// `BSTreadTransfInterpolator`'s vtable (`010979f4`).
const VTABLE: u32 = 0x0109_79f4;
/// `operator new` (`00aa13e0`, `cdecl (size)`) and the sized delete
/// (`00aa1460`, `cdecl (block, size)`).
const OPERATOR_NEW: u32 = 0x00aa_13e0;
const SIZED_DELETE: u32 = 0x00aa_1460;
/// `NiKeyBasedInterpolator`'s constructor (`00a3ca10`, `thiscall`), its
/// destructor (`00a3ca30`, `thiscall`), `CopyMembers` (`00aa0ab0`, `thiscall
/// (this, dest, cloning process)`) and `LoadBinary` (`00aa0ac0`, `thiscall
/// (this, stream)`).
const BASE_CONSTRUCTOR: u32 = 0x00a3_ca10;
const BASE_DESTRUCTOR: u32 = 0x00a3_ca30;
const BASE_COPY_MEMBERS: u32 = 0x00aa_0ab0;
const BASE_LOAD_BINARY: u32 = 0x00aa_0ac0;
/// The memory manager (`011f6238`) with its allocate (`00aa3e40`) and free
/// (`00aa4060`), both `thiscall`, one word.
const MEMORY_MANAGER: u32 = 0x011f_6238;
const MEMORY_MANAGER_ALLOCATE: u32 = 0x00aa_3e40;
const MEMORY_MANAGER_FREE: u32 = 0x00aa_4060;
/// The compiler's vector constructor iterator (`00401050`, `stdcall (array,
/// element size, count, constructor)`).
const VECTOR_CONSTRUCTOR_ITERATOR: u32 = 0x0040_1050;
/// `NiStream::LoadFixedString` (`00a64df0`, `thiscall (stream, &string)`),
/// `NiStream::ResolveLinkID` (`00a64620`, `thiscall (stream)`) and
/// `NiQuatTransform::LoadBinary` (`00a38bb0`, `thiscall (transform,
/// stream)`).
const STREAM_LOAD_FIXED_STRING: u32 = 0x00a6_4df0;
const STREAM_RESOLVE_LINK_ID: u32 = 0x00a6_4620;
const QUAT_TRANSFORM_LOAD_BINARY: u32 = 0x00a3_8bb0;
/// `InterlockedIncrement` and `InterlockedDecrement` (import slots, one word:
/// the address).
const INTERLOCKED_INCREMENT: u32 = 0x00fd_f1c0;
const INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;
/// The pooled empty `NiFixedString` handle (`0109b220`), never counted.
const FIXED_STRING_EMPTY: u32 = 0x0109_b220;
/// Words a fresh `LinkInfo` copies: a translation (3 words, `011a8400`), a
/// rotation (4 words, `011f3704`) and the scale `float` (`01096b8c`).
const FRESH_TRANSLATE: u32 = 0x011a_8400;
const FRESH_ROTATE: u32 = 0x011f_3704;
const FRESH_SCALE: u32 = 0x0109_6b8c;

/// `BSTreadTransfInterpolator` fields (Xbox PDB, same offsets on PC):
/// `Links` +0xC (`LinkInfo*`, the element count in the word before),
/// `uiNumLinks` +0x10, `fCurrLerp` +0x14, `fDiffLerp` +0x18, `uiLastLerpIdx`
/// +0x1C, `spLerpData` +0x20 (`NiPointer<NiFloatData>`).
const LINKS: u32 = 0xc;
const NUM_LINKS: u32 = 0x10;
const CURR_LERP: u32 = 0x14;
const DIFF_LERP: u32 = 0x18;
const LAST_LERP_INDEX: u32 = 0x1c;
const LERP_DATA: u32 = 0x20;
/// `BSTreadTransfInterpolator::LinkInfo` (Xbox PDB), 0x44 bytes: `Start`
/// +0 and `End` +0x20 (`NiQuatTransform`, 8 words each), `Name` +0x40
/// (`NiFixedString`).
const LINK_INFO_SIZE: u32 = 0x44;
const LINK_END: u32 = 0x20;
const LINK_NAME: u32 = 0x40;
/// The size of `BSTreadTransfInterpolator` (`operator new` and delete).
const OBJECT_SIZE: u32 = 0x24;

/// The byte count of a counted `new[]` of `count` elements of `size`
/// bytes: `count * size` (all ones on overflow) plus the count word (all
/// ones on a carry).
fn array_bytes(count: u32, size: u32) -> u32 {
    let product = count.checked_mul(size).unwrap_or(u32::MAX);
    product.checked_add(4).unwrap_or(u32::MAX)
}

fn word(e: &Engine, address: u32) -> u32 {
    e.mem.u32(address)
}

/// Releases a pooled string: `InterlockedDecrement` of its count
/// (`handle - 8`) unless it is the empty string.
fn release_fixed_string(e: &mut Engine, handle: u32) {
    if handle != e.global::<u32>(FIXED_STRING_EMPTY) {
        e.call(INTERLOCKED_DECREMENT, &args![handle.wrapping_sub(8)]);
    }
}

/// Releases a reference-counted object: `InterlockedDecrement` of its count
/// at +4 and, when that reaches zero, `DeleteThis` (vtable +4).
fn release_object(e: &mut Engine, object: u32) {
    if e.call(INTERLOCKED_DECREMENT, &args![object.wrapping_add(4)])
        .u32()
        == 0
    {
        e.vcall(object, 4, &args![]);
    }
}

/// The inline `NiPointer` assignment of `object` to the pointer at `slot`:
/// nothing when it already holds it; otherwise the old object released,
/// the new one stored and counted (`InterlockedIncrement` of its +4).
fn assign_object(e: &mut Engine, slot: u32, object: u32) {
    let old = word(e, slot);
    if old == object {
        return;
    }
    if old != 0 {
        release_object(e, old);
    }
    e.mem.set_u32(slot, object);
    if object != 0 {
        e.call(INTERLOCKED_INCREMENT, &args![object.wrapping_add(4)]);
    }
}

/// Destroys a counted `LinkInfo` array (`delete[]`): each name released,
/// last first, then the block (from its count word) freed.
fn delete_link_array(e: &mut Engine, links: u32) {
    let count = word(e, links.wrapping_sub(4));
    let mut index = count.wrapping_sub(1);
    while index as i32 >= 0 {
        let item = links.wrapping_add(index.wrapping_mul(LINK_INFO_SIZE));
        let name = word(e, item.wrapping_add(LINK_NAME));
        release_fixed_string(e, name);
        index = index.wrapping_sub(1);
    }
    e.call(
        MEMORY_MANAGER_FREE,
        &args![MEMORY_MANAGER, links.wrapping_sub(4)],
    );
}

/// A counted `new[]` of `count` `LinkInfo`s, each built by `00a433b0`;
/// null when the allocation fails.
fn new_link_array(e: &mut Engine, count: u32) -> u32 {
    let block = e
        .call(
            MEMORY_MANAGER_ALLOCATE,
            &args![MEMORY_MANAGER, array_bytes(count, LINK_INFO_SIZE)],
        )
        .u32();
    if block == 0 {
        return 0;
    }
    e.mem.set_u32(block, count);
    let links = block.wrapping_add(4);
    e.call(
        VECTOR_CONSTRUCTOR_ITERATOR,
        &args![links, LINK_INFO_SIZE, count, 0x00a4_33b0u32],
    );
    links
}

/// The fields a new `BSTreadTransfInterpolator` gets after the base
/// constructor: no links, both lerp floats 0.0 (one `FLDZ` stored twice),
/// the vtable, no lerp data.
fn init_fields(e: &mut Engine, object: u32) {
    e.mem.set_f32(object + CURR_LERP, 0.0);
    e.mem.set_u32(object + LINKS, 0);
    e.mem.set_u32(object + NUM_LINKS, 0);
    e.mem.set_f32(object + DIFF_LERP, 0.0);
    e.mem.set_u32(object + LAST_LERP_INDEX, 0);
    e.mem.set_u32(object, VTABLE);
    e.mem.set_u32(object + LERP_DATA, 0);
}

// Translated from 00a43370 (decompiled, FalloutNV.exe 1.4.0.525)
/// The class factory (no Ghidra function; the engine map has no name):
/// `operator new(0x24)`, the `NiKeyBasedInterpolator` constructor, then no
/// links, both lerp floats 0.0, no lerp data and the class's vtable. Null
/// when the allocation fails.
pub fn fn_00a43370(e: &mut Engine) -> Ptr {
    let object = e.call(OPERATOR_NEW, &args![OBJECT_SIZE]).u32();
    if object == 0 {
        return Ptr::new(0);
    }
    e.call(BASE_CONSTRUCTOR, &args![object]);
    init_fields(e, object);
    Ptr::new(object)
}

// Translated from 00a433b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `LinkInfo` constructor (no Ghidra function; given to the vector
/// constructor iterator): `Start` and `End` each take the fresh
/// translation, rotation and scale (the scale through the FPU), and `Name`
/// takes the empty string's handle word without counting it. Answers
/// `this`.
pub fn fn_00a433b0(e: &mut Engine, this: Ptr) -> Ptr {
    let item = this.addr();
    for transform in [item, item.wrapping_add(LINK_END)] {
        for i in 0..3 {
            let value = word(e, FRESH_TRANSLATE + 4 * i);
            e.mem.set_u32(transform.wrapping_add(4 * i), value);
        }
        for i in 0..4 {
            let value = word(e, FRESH_ROTATE + 4 * i);
            e.mem.set_u32(transform.wrapping_add(0xc + 4 * i), value);
        }
        let scale = x87_quiet_bits(word(e, FRESH_SCALE));
        e.mem.set_u32(transform.wrapping_add(0x1c), scale);
    }
    let empty = word(e, FIXED_STRING_EMPTY);
    e.mem.set_u32(item.wrapping_add(LINK_NAME), empty);
    this
}

/// Copies the 8 words of a `NiQuatTransform` (`MOVSD.REP`).
fn copy_transform(e: &mut Engine, source: u32, target: u32) {
    for i in 0..8 {
        let value = word(e, source.wrapping_add(4 * i));
        e.mem.set_u32(target.wrapping_add(4 * i), value);
    }
}

// Translated from 00a43690 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSTreadTransfInterpolator::CopyMembers` (body; the engine map has no
/// name): the base class's `CopyMembers`, then `dest`'s link array
/// destroyed and rebuilt with this one's count, each link's name assigned
/// (counted) and its two transforms copied word for word, the lerp data
/// pointer assigned and both lerp floats copied (through the FPU).
pub fn fn_00a43690(e: &mut Engine, this: Ptr, dest: Ptr, cloning: Ptr) {
    let this = this.addr();
    let dest = dest.addr();
    e.call(BASE_COPY_MEMBERS, &args![this, dest, cloning]);
    let old_links = word(e, dest.wrapping_add(LINKS));
    if old_links != 0 {
        delete_link_array(e, old_links);
    }
    let count = word(e, this + NUM_LINKS);
    e.mem.set_u32(dest.wrapping_add(NUM_LINKS), count);
    let count = word(e, this + NUM_LINKS);
    let links = new_link_array(e, count);
    e.mem.set_u32(dest.wrapping_add(LINKS), links);
    let mut index = 0u32;
    let mut offset = 0u32;
    while index < word(e, this + NUM_LINKS) {
        let source = word(e, this + LINKS).wrapping_add(offset);
        let target = word(e, dest.wrapping_add(LINKS)).wrapping_add(offset);
        let name = word(e, source.wrapping_add(LINK_NAME));
        let target_name = target.wrapping_add(LINK_NAME);
        if word(e, target_name) != name {
            if name != e.global::<u32>(FIXED_STRING_EMPTY) {
                e.call(INTERLOCKED_INCREMENT, &args![name.wrapping_sub(8)]);
            }
            let old = word(e, target_name);
            release_fixed_string(e, old);
            e.mem.set_u32(target_name, name);
        }
        let target = word(e, dest.wrapping_add(LINKS)).wrapping_add(offset);
        let source = word(e, this + LINKS).wrapping_add(offset);
        copy_transform(e, source, target);
        let target = word(e, dest.wrapping_add(LINKS)).wrapping_add(offset + LINK_END);
        let source = word(e, this + LINKS).wrapping_add(offset + LINK_END);
        copy_transform(e, source, target);
        index = index.wrapping_add(1);
        offset = offset.wrapping_add(LINK_INFO_SIZE);
    }
    let data = word(e, this + LERP_DATA);
    assign_object(e, dest.wrapping_add(LERP_DATA), data);
    let curr = x87_quiet_bits(word(e, this + CURR_LERP));
    e.mem.set_u32(dest.wrapping_add(CURR_LERP), curr);
    let diff = x87_quiet_bits(word(e, this + DIFF_LERP));
    e.mem.set_u32(dest.wrapping_add(DIFF_LERP), diff);
}

// Translated from 00a43840 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSTreadTransfInterpolator::~BSTreadTransfInterpolator` (body; the
/// engine map has no name): the class's vtable, the link array destroyed
/// (names released, block freed), the lerp data released, then the
/// `NiKeyBasedInterpolator` destructor (a tail jump).
pub fn fn_00a43840(e: &mut Engine, this: Ptr) {
    let object = this.addr();
    e.mem.set_u32(object, VTABLE);
    let links = word(e, object + LINKS);
    if links != 0 {
        delete_link_array(e, links);
    }
    let data = word(e, object + LERP_DATA);
    if data != 0 {
        release_object(e, data);
    }
    e.call(BASE_DESTRUCTOR, &args![object]);
}

// Translated from 00a438d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSTreadTransfInterpolator::LoadBinary` (Xbox PDB): the base class's
/// `LoadBinary`; the link count read (one 4-byte word through the stream
/// reader's function at its +8, `cdecl (reader, this + 0x10, 4, &4, 1)`,
/// the answer added to the reader's +4); a new link array of that count
/// (the old one is not freed), each link's name, `Start` and `End` loaded;
/// then the lerp data resolved from its link id and assigned.
pub fn bs_tread_transf_interpolator_load_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    let object = this.addr();
    let stream = stream.addr();
    e.call(BASE_LOAD_BINARY, &args![object, stream]);
    let reader = word(e, stream.wrapping_add(0x24c));
    let read_function = word(e, reader.wrapping_add(8));
    let read = e.with_stack(4, |e, size| {
        e.mem.set_u32(size.addr(), 4);
        e.call(
            read_function,
            &args![reader, object + NUM_LINKS, 4u32, size, 1u32],
        )
        .u32()
    });
    let position = word(e, reader.wrapping_add(4)).wrapping_add(read);
    e.mem.set_u32(reader.wrapping_add(4), position);
    let count = word(e, object + NUM_LINKS);
    let links = new_link_array(e, count);
    e.mem.set_u32(object + LINKS, links);
    let mut index = 0u32;
    let mut offset = 0u32;
    while index < word(e, object + NUM_LINKS) {
        let item = word(e, object + LINKS).wrapping_add(offset);
        e.call(
            STREAM_LOAD_FIXED_STRING,
            &args![stream, item.wrapping_add(LINK_NAME)],
        );
        let item = word(e, object + LINKS).wrapping_add(offset);
        e.call(QUAT_TRANSFORM_LOAD_BINARY, &args![item, stream]);
        let item = word(e, object + LINKS).wrapping_add(offset);
        e.call(
            QUAT_TRANSFORM_LOAD_BINARY,
            &args![item.wrapping_add(LINK_END), stream],
        );
        index = index.wrapping_add(1);
        offset = offset.wrapping_add(LINK_INFO_SIZE);
    }
    let data = e.call(STREAM_RESOLVE_LINK_ID, &args![stream]).u32();
    assign_object(e, object + LERP_DATA, data);
}

// Translated from 00a439e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSTreadTransfInterpolator::CreateClone` (Xbox PDB): a new object built
/// as the factory builds it (`operator new(0x24)`, base constructor, empty
/// fields, the vtable), then this one's members copied into it
/// (`00a43690`); when the allocation fails the copy still runs, into null,
/// and null is answered.
pub fn bs_tread_transf_interpolator_create_clone(e: &mut Engine, this: Ptr, cloning: Ptr) -> Ptr {
    let object = e.call(OPERATOR_NEW, &args![OBJECT_SIZE]).u32();
    if object != 0 {
        e.call(BASE_CONSTRUCTOR, &args![object]);
        init_fields(e, object);
    }
    fn_00a43690(e, this, Ptr::new(object), cloning);
    Ptr::new(object)
}

// Translated from 00a43a50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSTreadTransfInterpolator::scalar deleting destructor` (Xbox PDB): the
/// destructor (`00a43840`), then the sized delete of 0x24 bytes when bit 0
/// of `flags` is set. Answers `this`.
pub fn bs_tread_transf_interpolator_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    fn_00a43840(e, this);
    if flags & 1 != 0 {
        e.call(SIZED_DELETE, &args![this, OBJECT_SIZE]);
    }
    this
}

/// `NiKeyBasedInterpolator::RegisterStreamables` (`00aa0ad0`, `thiscall
/// (this, stream)`) and the base `SaveBinary` this class's runs first
/// (`00a95ca0`, `thiscall (this, stream)`).
const BASE_REGISTER_STREAMABLES: u32 = 0x00aa_0ad0;
const BASE_SAVE_BINARY: u32 = 0x00a9_5ca0;
/// `NiStream`'s `RegisterFixedString` (vtable +0x20, one word: the string's
/// address) and `SaveLinkID` (+0x30, one word: the object); the objects'
/// `RegisterStreamables` (+0x54).
const STREAM_SLOT_REGISTER_FIXED_STRING: u32 = 0x20;
const STREAM_SLOT_SAVE_LINK_ID: u32 = 0x30;
const SLOT_REGISTER_STREAMABLES: u32 = 0x54;
/// `NiStream::SaveFixedString` (`00a641f0`, `thiscall (stream, &string)`)
/// and `NiQuatTransform::SaveBinary` (`00a38b60`, `thiscall (transform,
/// stream)`).
const STREAM_SAVE_FIXED_STRING: u32 = 0x00a6_41f0;
const QUAT_TRANSFORM_SAVE_BINARY: u32 = 0x00a3_8b60;
/// `NiFloatKey::GenInterp` (`00a26b40`, `cdecl (time, keys, type, count,
/// &last index, key size)`, the value in `ST0`).
const FLOAT_KEY_GEN_INTERP: u32 = 0x00a2_6b40;
/// `NiInterpolator::m_fLastTime` (Xbox PDB) +8.
const LAST_TIME: u32 = 0x8;
/// `NiFloatData` (Xbox PDB): `m_uiNumKeys` +8, `m_pkKeys` +0xC, `m_eType`
/// +0x10, `m_ucKeySize` +0x14.
const DATA_NUM_KEYS: u32 = 0x8;
const DATA_KEYS: u32 = 0xc;
const DATA_TYPE: u32 = 0x10;
const DATA_KEY_SIZE: u32 = 0x14;
/// The virtual slot `GetChannelPosed` calls (+0xE0, one word: the channel).
const SLOT_CHANNEL_QUERY: u32 = 0xe0;

// Translated from 00a43350 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSTreadTransfInterpolator::GetKeyContent` (Xbox PDB; `thiscall`, one
/// stack word): 6 for any channel but 0, 0 for channel 0 (`NEG`/`SBB`/`AND
/// 6`); `this` is not read.
pub fn bs_tread_transf_interpolator_get_key_content(
    _e: &mut Engine,
    _this: Ptr,
    channel: u16,
) -> u32 {
    if channel != 0 {
        6
    } else {
        0
    }
}

// Translated from 00a43450 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSTreadTransfInterpolator::RegisterStreamables` (Xbox PDB): the base
/// `RegisterStreamables` (`00aa0ad0`); when it is true, each link's name is
/// registered with the stream (vtable +0x20) and the lerp data registers
/// itself (vtable +0x54) when there is one. Answers the base's answer (kept
/// in the stream argument's stack byte).
pub fn bs_tread_transf_interpolator_register_streamables(
    e: &mut Engine,
    this: Ptr,
    stream: Ptr,
) -> bool {
    let this = this.addr();
    let answer = e
        .call(BASE_REGISTER_STREAMABLES, &args![this, stream])
        .bool();
    if !answer {
        return false;
    }
    let mut index = 0u32;
    while index < word(e, this + NUM_LINKS) {
        let links = word(e, this + LINKS);
        let name = links.wrapping_add(index.wrapping_mul(LINK_INFO_SIZE) + LINK_NAME);
        e.vcall(
            stream.addr(),
            STREAM_SLOT_REGISTER_FIXED_STRING,
            &args![name],
        );
        index = index.wrapping_add(1);
    }
    let data = word(e, this + LERP_DATA);
    if data != 0 {
        e.vcall(data, SLOT_REGISTER_STREAMABLES, &args![stream]);
    }
    answer
}

// Translated from 00a434b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSTreadTransfInterpolator::Update` (Xbox PDB; `thiscall`, three stack
/// words, `RET 0xC`; the object and value words are not read): with lerp
/// keys, the lerp value `v` at `time` (`NiFloatKey::GenInterp`, the last
/// index kept); when `time` went back (before the last time) and `v` is
/// below `fCurrLerp`, the lerp wrapped: `fDiffLerp = (1 - fCurrLerp) + v`;
/// otherwise `fDiffLerp = v - fCurrLerp`; `fCurrLerp = v`. The last time
/// takes `time` in every case; true is answered. The key-size argument is
/// the data's byte in a word whose upper bytes the exe leaves as they were
/// on its stack; `GenInterp` reads only the byte (`MOVZX` at `00a26b70`).
pub fn bs_tread_transf_interpolator_update(
    e: &mut Engine,
    this: Ptr,
    time: f32,
    _unused_2: u32,
    _unused_3: u32,
) -> bool {
    let this = this.addr();
    let time_bits = x87_quiet_bits(time.to_bits());
    let data = word(e, this + LERP_DATA);
    let count = word(e, data + DATA_NUM_KEYS);
    let key_size = u32::from(e.mem.u8(data + DATA_KEY_SIZE));
    let key_type = word(e, data + DATA_TYPE);
    let keys = word(e, data + DATA_KEYS);
    if count != 0 {
        let lerp = e.with_stack(4, |e, last_index| {
            let last_index = last_index.addr();
            let index = word(e, this + LAST_LERP_INDEX);
            e.mem.set_u32(last_index, index);
            let value = e
                .call(
                    FLOAT_KEY_GEN_INTERP,
                    &args![time_bits, keys, key_type, count, last_index, key_size],
                )
                .f64();
            let index = word(e, last_index);
            e.mem.set_u32(this + LAST_LERP_INDEX, index);
            x87_store32(value)
        });
        let value = x87_load32(lerp);
        let last = x87_load32(word(e, this + LAST_TIME));
        let current = x87_load32(word(e, this + CURR_LERP));
        if x87_load32(time_bits) < last && current > value {
            let one_minus = x87_sub(1.0, current);
            let diff = x87_add(one_minus, value);
            e.mem.set_u32(this + DIFF_LERP, x87_store32(diff));
        } else {
            let diff = x87_sub(value, x87_m32(word(e, this + CURR_LERP)));
            e.mem.set_u32(this + DIFF_LERP, x87_store32(diff));
        }
        e.mem.set_u32(this + CURR_LERP, x87_store32(value));
    }
    e.mem.set_u32(this + LAST_TIME, time_bits);
    true
}

// Translated from 00a43560 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSTreadTransfInterpolator::GetKeyCount` (Xbox PDB): for channel 0 with
/// lerp data its key count; 0 otherwise.
pub fn bs_tread_transf_interpolator_get_key_count(e: &mut Engine, this: Ptr, channel: u16) -> u32 {
    let data = word(e, this.addr() + LERP_DATA);
    if channel != 0 || data == 0 {
        return 0;
    }
    word(e, data + DATA_NUM_KEYS)
}

// Translated from 00a43580 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSTreadTransfInterpolator::GetKeyType` (Xbox PDB): for channel 0 with
/// lerp data its key type; 0 otherwise.
pub fn bs_tread_transf_interpolator_get_key_type(e: &mut Engine, this: Ptr, channel: u16) -> u32 {
    let data = word(e, this.addr() + LERP_DATA);
    if channel != 0 || data == 0 {
        return 0;
    }
    word(e, data + DATA_TYPE)
}

// Translated from 00a435a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSTreadTransfInterpolator::GetKeyArray` (Xbox PDB): for channel 0 with
/// lerp data its keys; null otherwise.
pub fn bs_tread_transf_interpolator_get_key_array(e: &mut Engine, this: Ptr, channel: u16) -> u32 {
    let data = word(e, this.addr() + LERP_DATA);
    if channel != 0 || data == 0 {
        return 0;
    }
    word(e, data + DATA_KEYS)
}

// Translated from 00a435c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSTreadTransfInterpolator::GetKeyStride` (Xbox PDB): for channel 0 with
/// lerp data its key size byte; 0 otherwise (`AL` only).
pub fn bs_tread_transf_interpolator_get_key_stride(e: &mut Engine, this: Ptr, channel: u16) -> u8 {
    let data = word(e, this.addr() + LERP_DATA);
    if channel != 0 || data == 0 {
        return 0;
    }
    e.mem.u8(data + DATA_KEY_SIZE)
}

// Translated from 00a435e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSTreadTransfInterpolator::GetChannelPosed` (Xbox PDB): with lerp data
/// its own virtual slot +0xE0 runs on the channel word (its answer is
/// dropped); always false.
pub fn bs_tread_transf_interpolator_get_channel_posed(
    e: &mut Engine,
    this: Ptr,
    channel: u32,
) -> bool {
    if word(e, this.addr() + LERP_DATA) != 0 {
        e.vcall(this.addr(), SLOT_CHANNEL_QUERY, &args![channel]);
    }
    false
}

// Translated from 00a43600 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSTreadTransfInterpolator::SaveBinary` (Xbox PDB): the base
/// `SaveBinary` (`00a95ca0`), the link count (4 bytes; the component size in
/// the stream argument's stack word, which then holds the loop counter),
/// each link's name (`SaveFixedString`) and its two transforms
/// (`NiQuatTransform::SaveBinary`), then the lerp data's link (`SaveLinkID`,
/// the stream's vtable +0x30).
pub fn bs_tread_transf_interpolator_save_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    use crate::units::nianimation::niblendinterpolator::stream_write;
    let this = this.addr();
    e.call(BASE_SAVE_BINARY, &args![this, stream]);
    e.with_stack(4, |e, argument| {
        let argument = argument.addr();
        e.mem.set_u32(argument, stream.addr());
        stream_write(e, stream, this + NUM_LINKS, 4, argument);
        e.mem.set_u32(argument, 0);
        let mut offset = 0u32;
        while word(e, argument) < word(e, this + NUM_LINKS) {
            let links = word(e, this + LINKS);
            let name = links.wrapping_add(offset + LINK_NAME);
            e.call(STREAM_SAVE_FIXED_STRING, &args![stream, name]);
            let start = word(e, this + LINKS).wrapping_add(offset);
            e.call(QUAT_TRANSFORM_SAVE_BINARY, &args![start, stream]);
            let end = word(e, this + LINKS).wrapping_add(offset + LINK_END);
            e.call(QUAT_TRANSFORM_SAVE_BINARY, &args![end, stream]);
            let counter = word(e, argument).wrapping_add(1);
            offset = offset.wrapping_add(LINK_INFO_SIZE);
            e.mem.set_u32(argument, counter);
        }
    });
    let data = word(e, this + LERP_DATA);
    e.vcall(stream.addr(), STREAM_SLOT_SAVE_LINK_ID, &args![data]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a43360, bs_tread_transf_interpolator_get_rtti() -> Ptr),
        entry!(0x00a4_3340, bs_tread_transf_interpolator_is_equal(Ptr, Ptr) -> bool),
        entry!(0x00a4_3370, fn_00a43370() -> Ptr),
        entry!(0x00a4_33b0, fn_00a433b0(Ptr) -> Ptr),
        entry!(0x00a4_3690, fn_00a43690(Ptr, Ptr, Ptr)),
        entry!(0x00a4_3840, fn_00a43840(Ptr)),
        entry!(
            0x00a4_38d0,
            bs_tread_transf_interpolator_load_binary(Ptr, Ptr)
        ),
        entry!(0x00a4_39e0, bs_tread_transf_interpolator_create_clone(Ptr, Ptr) -> Ptr),
        entry!(0x00a4_3a50, bs_tread_transf_interpolator_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(
            0x00a4_3350,
            bs_tread_transf_interpolator_get_key_content(Ptr, u16) -> u32
        ),
        entry!(
            0x00a4_3450,
            bs_tread_transf_interpolator_register_streamables(Ptr, Ptr) -> bool
        ),
        entry!(
            0x00a4_34b0,
            bs_tread_transf_interpolator_update(Ptr, f32, u32, u32) -> bool
        ),
        entry!(
            0x00a4_3560,
            bs_tread_transf_interpolator_get_key_count(Ptr, u16) -> u32
        ),
        entry!(
            0x00a4_3580,
            bs_tread_transf_interpolator_get_key_type(Ptr, u16) -> u32
        ),
        entry!(
            0x00a4_35a0,
            bs_tread_transf_interpolator_get_key_array(Ptr, u16) -> u32
        ),
        entry!(
            0x00a4_35c0,
            bs_tread_transf_interpolator_get_key_stride(Ptr, u16) -> u8
        ),
        entry!(
            0x00a4_35e0,
            bs_tread_transf_interpolator_get_channel_posed(Ptr, u32) -> bool
        ),
        entry!(
            0x00a4_3600,
            bs_tread_transf_interpolator_save_binary(Ptr, Ptr)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bs_tread_transf_interpolator_get_rtti_returns_the_class_rtti() {
        let mut e = Engine::new();
        assert_eq!(
            e.call(0x00a43360, &args![]).u32(),
            BS_TREAD_TRANSF_INTERPOLATOR_RTTI
        );
    }

    #[test]
    fn thunk_00a43340_forwards_its_words_and_returns_the_target_result() {
        const THUNK: u32 = 0x00a4_3340;
        const TARGET: u32 = 0x00c8_5800;
        let mut e = Engine::new();
        e.register(TARGET, |_, _| Ret {
            eax: 1,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        assert_eq!(e.call(THUNK, &args![0x1000u32, 0x2000u32]).u32(), 1);
        let log = e.call_log.take().unwrap();
        let hits: Vec<_> = log.iter().filter(|call| call.0 == TARGET).collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, args![0x1000u32, 0x2000u32]);
        e.register(TARGET, |_, _| Ret::default());
        assert_eq!(e.call(THUNK, &args![0x1000u32, 0x2000u32]).u32(), 0);
    }

    type CallList = Vec<Vec<u32>>;

    fn tread_calls_to(e: &Engine, addr: u32) -> CallList {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|call| call.0 == addr)
            .map(|call| call.1.clone())
            .collect()
    }

    /// An engine whose allocator, vector constructor and base-class
    /// functions are doubles; the vector constructor runs the element
    /// constructor it is given on each element.
    fn tread_engine() -> Engine {
        let mut e = Engine::new();
        e.map(FIXED_STRING_EMPTY, 4);
        e.set_global(FIXED_STRING_EMPTY, 0x0500_0008u32);
        e.map(FRESH_TRANSLATE, 0xc);
        e.map(FRESH_ROTATE, 0x10);
        e.map(FRESH_SCALE, 4);
        e.register(OPERATOR_NEW, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(SIZED_DELETE, |_, _| Ret::default());
        e.register(MEMORY_MANAGER_ALLOCATE, |e, a| e.mem.alloc(a[1]).into_ret());
        e.register(MEMORY_MANAGER_FREE, |_, _| Ret::default());
        e.register(INTERLOCKED_INCREMENT, |e, a| {
            let count = e.mem.u32(a[0]).wrapping_add(1);
            e.mem.set_u32(a[0], count);
            count.into_ret()
        });
        e.register(INTERLOCKED_DECREMENT, |e, a| {
            let count = e.mem.u32(a[0]).wrapping_sub(1);
            e.mem.set_u32(a[0], count);
            count.into_ret()
        });
        e.register(VECTOR_CONSTRUCTOR_ITERATOR, |e, a| {
            for i in 0..a[2] {
                e.call(a[3], &[a[0] + i * a[1]]);
            }
            Ret::default()
        });
        for addr in [
            BASE_CONSTRUCTOR,
            BASE_DESTRUCTOR,
            BASE_COPY_MEMBERS,
            BASE_LOAD_BINARY,
            STREAM_LOAD_FIXED_STRING,
            QUAT_TRANSFORM_LOAD_BINARY,
        ] {
            e.register(addr, |_, _| Ret::default());
        }
        e.call_log = Some(vec![]);
        e
    }

    /// A pooled string handle with its count word at `handle - 8`.
    fn tread_string(e: &mut Engine, count: u32) -> u32 {
        let block = e.mem.alloc(0x10);
        e.mem.set_u32(block, count);
        block + 8
    }

    /// A reference-counted object whose `DeleteThis` (vtable +4) is `delete_this`.
    fn tread_object(e: &mut Engine, count: u32, delete_this: u32) -> u32 {
        let vtable = e.mem.alloc(8);
        e.mem.set_u32(vtable + 4, delete_this);
        let object = e.mem.alloc(8);
        e.mem.set_u32(object, vtable);
        e.mem.set_u32(object + 4, count);
        object
    }

    #[test]
    fn tread_factory_builds_an_empty_interpolator_or_answers_null() {
        let mut e = tread_engine();
        let object = e.call(0x00a4_3370, &args![]).u32();
        assert_ne!(object, 0);
        assert_eq!(tread_calls_to(&e, OPERATOR_NEW), vec![vec![0x24]]);
        assert_eq!(tread_calls_to(&e, BASE_CONSTRUCTOR), vec![vec![object]]);
        assert_eq!(e.mem.u32(object), VTABLE);
        for offset in [
            LINKS,
            NUM_LINKS,
            CURR_LERP,
            DIFF_LERP,
            LAST_LERP_INDEX,
            LERP_DATA,
        ] {
            assert_eq!(e.mem.u32(object + offset), 0);
        }
        e.register(OPERATOR_NEW, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00a4_3370, &args![]).u32(), 0);
        assert!(tread_calls_to(&e, BASE_CONSTRUCTOR).is_empty());
    }

    #[test]
    fn tread_link_info_constructor_copies_the_fresh_transform_twice() {
        let mut e = tread_engine();
        for i in 0..3 {
            e.set_global(FRESH_TRANSLATE + 4 * i, 0x100 + i);
        }
        for i in 0..4 {
            e.set_global(FRESH_ROTATE + 4 * i, 0x200 + i);
        }
        // A signalling NaN comes out quiet (FLD/FSTP).
        e.set_global(FRESH_SCALE, 0x7f80_0001u32);
        let item = e.mem.alloc(LINK_INFO_SIZE);
        assert_eq!(e.call(0x00a4_33b0, &args![item]).u32(), item);
        for base in [item, item + LINK_END] {
            for i in 0..3 {
                assert_eq!(e.mem.u32(base + 4 * i), 0x100 + i);
            }
            for i in 0..4 {
                assert_eq!(e.mem.u32(base + 0xc + 4 * i), 0x200 + i);
            }
            assert_eq!(e.mem.u32(base + 0x1c), 0x7fc0_0001);
        }
        assert_eq!(e.mem.u32(item + LINK_NAME), 0x0500_0008);
    }

    #[test]
    fn tread_copy_members_rebuilds_the_links_and_shares_names_and_data() {
        const DELETE_THIS: u32 = 0x0300_0010;
        let mut e = tread_engine();
        e.register(DELETE_THIS, |_, _| Ret::default());
        let empty = 0x0500_0008u32;
        // Source: two links, the first named, the second unnamed.
        let name = tread_string(&mut e, 1);
        let this = e.mem.alloc(0x24);
        let source = e.mem.alloc(4 + 2 * LINK_INFO_SIZE) + 4;
        e.mem.set_u32(source - 4, 2);
        for i in 0..16 {
            e.mem.set_u32(source + 4 * i, 0x1000 + i);
            e.mem.set_u32(source + LINK_INFO_SIZE + 4 * i, 0x2000 + i);
        }
        e.mem.set_u32(source + LINK_NAME, name);
        e.mem.set_u32(source + LINK_INFO_SIZE + LINK_NAME, empty);
        e.mem.set_u32(this + LINKS, source);
        e.mem.set_u32(this + NUM_LINKS, 2);
        e.mem.set_u32(this + CURR_LERP, 0x7f80_0002);
        e.mem.set_f32(this + DIFF_LERP, 0.25);
        let data = tread_object(&mut e, 1, DELETE_THIS);
        e.mem.set_u32(this + LERP_DATA, data);
        // Destination: one old link holding a counted name, old lerp data.
        let old_name = tread_string(&mut e, 2);
        let dest = e.mem.alloc(0x24);
        let old_links = e.mem.alloc(4 + LINK_INFO_SIZE) + 4;
        e.mem.set_u32(old_links - 4, 1);
        e.mem.set_u32(old_links + LINK_NAME, old_name);
        e.mem.set_u32(dest + LINKS, old_links);
        let old_data = tread_object(&mut e, 1, DELETE_THIS);
        e.mem.set_u32(dest + LERP_DATA, old_data);
        e.call(0x00a4_3690, &args![this, dest, 0x7000u32]);
        assert_eq!(
            tread_calls_to(&e, BASE_COPY_MEMBERS),
            vec![vec![this, dest, 0x7000]]
        );
        assert_eq!(e.mem.u32(old_name - 8), 1);
        assert_eq!(
            tread_calls_to(&e, MEMORY_MANAGER_FREE),
            vec![vec![MEMORY_MANAGER, old_links - 4]]
        );
        assert_eq!(
            tread_calls_to(&e, MEMORY_MANAGER_ALLOCATE),
            vec![vec![MEMORY_MANAGER, 4 + 2 * LINK_INFO_SIZE]]
        );
        let links = e.mem.u32(dest + LINKS);
        assert_eq!(e.mem.u32(links - 4), 2);
        assert_eq!(e.mem.u32(dest + NUM_LINKS), 2);
        for i in 0..16 {
            assert_eq!(e.mem.u32(links + 4 * i), 0x1000 + i);
            assert_eq!(e.mem.u32(links + LINK_INFO_SIZE + 4 * i), 0x2000 + i);
        }
        assert_eq!(e.mem.u32(links + LINK_NAME), name);
        assert_eq!(e.mem.u32(name - 8), 2);
        assert_eq!(e.mem.u32(links + LINK_INFO_SIZE + LINK_NAME), empty);
        // The old lerp data reaches zero and deletes itself; the new one is counted.
        assert_eq!(tread_calls_to(&e, DELETE_THIS), vec![vec![old_data]]);
        assert_eq!(e.mem.u32(dest + LERP_DATA), data);
        assert_eq!(e.mem.u32(data + 4), 2);
        assert_eq!(e.mem.u32(dest + CURR_LERP), 0x7fc0_0002);
        assert_eq!(e.mem.f32(dest + DIFF_LERP), 0.25);
    }

    #[test]
    fn tread_destructor_releases_links_and_data_then_runs_the_base() {
        const DELETE_THIS: u32 = 0x0300_0010;
        let mut e = tread_engine();
        e.register(DELETE_THIS, |_, _| Ret::default());
        let first = tread_string(&mut e, 3);
        let second = tread_string(&mut e, 1);
        let this = e.mem.alloc(0x24);
        let links = e.mem.alloc(4 + 2 * LINK_INFO_SIZE) + 4;
        e.mem.set_u32(links - 4, 2);
        e.mem.set_u32(links + LINK_NAME, first);
        e.mem.set_u32(links + LINK_INFO_SIZE + LINK_NAME, second);
        e.mem.set_u32(this + LINKS, links);
        let data = tread_object(&mut e, 2, DELETE_THIS);
        e.mem.set_u32(this + LERP_DATA, data);
        e.call(0x00a4_3840, &args![this]);
        assert_eq!(e.mem.u32(this), VTABLE);
        assert_eq!(e.mem.u32(first - 8), 2);
        assert_eq!(e.mem.u32(second - 8), 0);
        assert_eq!(
            tread_calls_to(&e, MEMORY_MANAGER_FREE),
            vec![vec![MEMORY_MANAGER, links - 4]]
        );
        assert_eq!(e.mem.u32(data + 4), 1);
        assert!(tread_calls_to(&e, DELETE_THIS).is_empty());
        assert_eq!(tread_calls_to(&e, BASE_DESTRUCTOR), vec![vec![this]]);
        // Without links or data only the vtable and the base destructor remain.
        let bare = e.mem.alloc(0x24);
        e.call_log = Some(vec![]);
        e.call(0x00a4_3840, &args![bare]);
        assert!(tread_calls_to(&e, MEMORY_MANAGER_FREE).is_empty());
        assert_eq!(tread_calls_to(&e, BASE_DESTRUCTOR), vec![vec![bare]]);
    }

    #[test]
    fn tread_load_binary_reads_the_count_then_each_link_and_the_data() {
        const READ: u32 = 0x0300_0000;
        let mut e = tread_engine();
        e.register(READ, |e, a| {
            assert_eq!(e.mem.u32(a[3]), 4);
            e.mem.set_u32(a[1], 2);
            4u32.into_ret()
        });
        let data = tread_object(&mut e, 0, 0);
        e.register_double(STREAM_RESOLVE_LINK_ID, move |_, _| data.into_ret());
        let reader = e.mem.alloc(0xc);
        e.mem.set_u32(reader + 4, 0x10);
        e.mem.set_u32(reader + 8, READ);
        let stream = e.mem.alloc(0x250);
        e.mem.set_u32(stream + 0x24c, reader);
        let this = e.mem.alloc(0x24);
        e.call(0x00a4_38d0, &args![this, stream]);
        assert_eq!(
            tread_calls_to(&e, BASE_LOAD_BINARY),
            vec![vec![this, stream]]
        );
        let reads = tread_calls_to(&e, READ);
        assert_eq!(
            (reads[0][0], reads[0][1], reads[0][2], reads[0][4]),
            (reader, this + NUM_LINKS, 4, 1)
        );
        assert_eq!(e.mem.u32(reader + 4), 0x14);
        let links = e.mem.u32(this + LINKS);
        assert_eq!(e.mem.u32(links - 4), 2);
        assert_eq!(
            tread_calls_to(&e, STREAM_LOAD_FIXED_STRING),
            vec![
                vec![stream, links + LINK_NAME],
                vec![stream, links + LINK_INFO_SIZE + LINK_NAME]
            ]
        );
        assert_eq!(
            tread_calls_to(&e, QUAT_TRANSFORM_LOAD_BINARY),
            vec![
                vec![links, stream],
                vec![links + LINK_END, stream],
                vec![links + LINK_INFO_SIZE, stream],
                vec![links + LINK_INFO_SIZE + LINK_END, stream]
            ]
        );
        assert_eq!(e.mem.u32(this + LERP_DATA), data);
        assert_eq!(e.mem.u32(data + 4), 1);
    }

    #[test]
    fn tread_create_clone_copies_into_a_new_object_or_into_null() {
        let mut e = tread_engine();
        let this = e.mem.alloc(0x24);
        let clone = e.call(0x00a4_39e0, &args![this, 0x7000u32]).u32();
        assert_ne!(clone, 0);
        assert_eq!(e.mem.u32(clone), VTABLE);
        assert_eq!(tread_calls_to(&e, BASE_CONSTRUCTOR), vec![vec![clone]]);
        assert_eq!(
            tread_calls_to(&e, BASE_COPY_MEMBERS),
            vec![vec![this, clone, 0x7000]]
        );
        // A failed allocation still copies, into address 0.
        e.map(0, 0x40);
        e.register(OPERATOR_NEW, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00a4_39e0, &args![this, 0x7000u32]).u32(), 0);
        assert!(tread_calls_to(&e, BASE_CONSTRUCTOR).is_empty());
        assert_eq!(
            tread_calls_to(&e, BASE_COPY_MEMBERS),
            vec![vec![this, 0, 0x7000]]
        );
    }

    #[test]
    fn tread_scalar_deleting_destructor_frees_only_with_bit_0() {
        let mut e = tread_engine();
        let this = e.mem.alloc(0x24);
        assert_eq!(e.call(0x00a4_3a50, &args![this, 2u32]).u32(), this);
        assert!(tread_calls_to(&e, SIZED_DELETE).is_empty());
        assert_eq!(tread_calls_to(&e, BASE_DESTRUCTOR), vec![vec![this]]);
        e.call(0x00a4_3a50, &args![this, 1u32]);
        assert_eq!(tread_calls_to(&e, SIZED_DELETE), vec![vec![this, 0x24]]);
    }
}

#[cfg(test)]
mod a07_tests {
    use super::*;
    use crate::units::nianimation::niblendinterpolator::a07_test_support::{calls, stream};

    const TEST_VTABLE: u32 = 0x7400_0000;
    fn slot(offset: u32) -> u32 {
        0x7500_0000 + offset
    }

    fn engine() -> Engine {
        let mut e = Engine::new();
        let slots: Vec<u32> = (0..0x40).map(|i| slot(4 * i)).collect();
        e.put_vtable(TEST_VTABLE, &slots);
        for i in 0..0x40 {
            e.register(slot(4 * i), |_, _| Ret::default());
        }
        e
    }

    /// An interpolator with `links` links and lerp data (`count` keys of type
    /// 1, key size 8) unless `count` is `None`.
    fn tread(e: &mut Engine, links: u32, count: Option<u32>) -> u32 {
        let this = e.mem.alloc(OBJECT_SIZE);
        e.mem.set_u32(this, TEST_VTABLE);
        let array = e.mem.alloc(LINK_INFO_SIZE * links.max(1));
        e.mem.set_u32(this + LINKS, array);
        e.mem.set_u32(this + NUM_LINKS, links);
        if let Some(count) = count {
            let data = e.mem.alloc(0x18);
            e.mem.set_u32(data, TEST_VTABLE);
            e.mem.set_u32(data + DATA_NUM_KEYS, count);
            e.mem.set_u32(data + DATA_KEYS, 0x4000);
            e.mem.set_u32(data + DATA_TYPE, 1);
            e.mem.set_u8(data + DATA_KEY_SIZE, 8);
            e.mem.set_u32(this + LERP_DATA, data);
        }
        this
    }

    #[test]
    fn get_key_content_is_6_for_channels_but_0() {
        let mut e = engine();
        assert_eq!(e.call(0x00a4_3350, &args![0u32, 0u32]).u32(), 0);
        assert_eq!(e.call(0x00a4_3350, &args![0u32, 1u32]).u32(), 6);
        assert_eq!(e.call(0x00a4_3350, &args![0u32, 0x1_0000u32]).u32(), 0);
    }

    #[test]
    fn register_streamables_registers_names_and_the_data() {
        let mut e = engine();
        e.register(BASE_REGISTER_STREAMABLES, |e, a| {
            (e.mem.u32(a[0] + 4) == 0).into_ret()
        });
        let this = tread(&mut e, 2, Some(1));
        let s = e.mem.alloc(0x10);
        e.mem.set_u32(s, TEST_VTABLE);
        e.call_log = Some(vec![]);
        assert!(e.call(0x00a4_3450, &args![this, s]).bool());
        let links = e.mem.u32(this + LINKS);
        assert_eq!(
            calls(&e, slot(STREAM_SLOT_REGISTER_FIXED_STRING)),
            vec![
                vec![s, links + LINK_NAME],
                vec![s, links + LINK_INFO_SIZE + LINK_NAME]
            ]
        );
        let data = e.mem.u32(this + LERP_DATA);
        assert_eq!(
            calls(&e, slot(SLOT_REGISTER_STREAMABLES)),
            vec![vec![data, s]]
        );
        e.mem.set_u32(this + 4, 1);
        e.call_log = Some(vec![]);
        assert!(!e.call(0x00a4_3450, &args![this, s]).bool());
        assert!(calls(&e, slot(STREAM_SLOT_REGISTER_FIXED_STRING)).is_empty());
    }

    #[test]
    fn update_tracks_the_lerp_and_its_wrap() {
        let mut e = engine();
        e.register(FLOAT_KEY_GEN_INTERP, |e, a| {
            e.mem.set_u32(a[4], 3);
            (f64::from(f32::from_bits(a[0])) / 10.0).into_ret()
        });
        let this = tread(&mut e, 0, Some(2));
        e.call_log = Some(vec![]);
        // Forward: 0.25 - 0 (the lerp value is time / 10).
        assert!(e.call(0x00a4_34b0, &args![this, 2.5f32, 0u32, 0u32]).bool());
        assert_eq!(e.mem.f32(this + DIFF_LERP), 0.25);
        assert_eq!(e.mem.f32(this + CURR_LERP), 0.25);
        assert_eq!(e.mem.f32(this + LAST_TIME), 2.5);
        assert_eq!(e.mem.u32(this + LAST_LERP_INDEX), 3);
        let gen = calls(&e, FLOAT_KEY_GEN_INTERP);
        assert_eq!(gen[0][..4], [2.5f32.to_bits(), 0x4000, 1, 2]);
        assert_eq!(gen[0][5], 8);
        // Back in time with a smaller lerp: it wrapped, (1 - 0.25) + 0.125.
        e.call(0x00a4_34b0, &args![this, 1.25f32, 0u32, 0u32]);
        assert_eq!(e.mem.f32(this + DIFF_LERP), 0.875);
        assert_eq!(e.mem.f32(this + CURR_LERP), 0.125);
        // No keys: only the last time.
        let bare = tread(&mut e, 0, Some(0));
        e.mem.set_f32(bare + CURR_LERP, 0.5);
        e.call(0x00a4_34b0, &args![bare, 4.0f32, 0u32, 0u32]);
        assert_eq!(e.mem.f32(bare + CURR_LERP), 0.5);
        assert_eq!(e.mem.f32(bare + LAST_TIME), 4.0);
    }

    #[test]
    fn key_queries_answer_only_for_channel_0_with_data() {
        let mut e = engine();
        let this = tread(&mut e, 0, Some(5));
        assert_eq!(e.call(0x00a4_3560, &args![this, 0u32]).u32(), 5);
        assert_eq!(e.call(0x00a4_3580, &args![this, 0u32]).u32(), 1);
        assert_eq!(e.call(0x00a4_35a0, &args![this, 0u32]).u32(), 0x4000);
        assert_eq!(e.call(0x00a4_35c0, &args![this, 0u32]).u8(), 8);
        for addr in [0x00a4_3560u32, 0x00a4_3580, 0x00a4_35a0, 0x00a4_35c0] {
            assert_eq!(e.call(addr, &args![this, 1u32]).u8(), 0);
        }
        let bare = tread(&mut e, 0, None);
        for addr in [0x00a4_3560u32, 0x00a4_3580, 0x00a4_35a0, 0x00a4_35c0] {
            assert_eq!(e.call(addr, &args![bare, 0u32]).u8(), 0);
        }
    }

    #[test]
    fn get_channel_posed_asks_its_slot_and_answers_false() {
        let mut e = engine();
        e.register(slot(SLOT_CHANNEL_QUERY), |_, _| 1u32.into_ret());
        let this = tread(&mut e, 0, Some(1));
        e.call_log = Some(vec![]);
        assert!(!e.call(0x00a4_35e0, &args![this, 0x0001_0002u32]).bool());
        assert_eq!(
            calls(&e, slot(SLOT_CHANNEL_QUERY)),
            vec![vec![this, 0x0001_0002]]
        );
        let bare = tread(&mut e, 0, None);
        e.call_log = Some(vec![]);
        assert!(!e.call(0x00a4_35e0, &args![bare, 0u32]).bool());
        assert!(calls(&e, slot(SLOT_CHANNEL_QUERY)).is_empty());
    }

    #[test]
    fn save_binary_writes_the_links_and_the_data_link() {
        let mut e = engine();
        e.register(BASE_SAVE_BINARY, |_, _| Ret::default());
        e.register(STREAM_SAVE_FIXED_STRING, |_, _| Ret::default());
        e.register(QUAT_TRANSFORM_SAVE_BINARY, |_, _| Ret::default());
        let (s, written) = stream(&mut e, 0, &[]);
        e.mem.set_u32(s.addr(), TEST_VTABLE);
        let this = tread(&mut e, 2, Some(1));
        e.call_log = Some(vec![]);
        e.call(0x00a4_3600, &args![this, s]);
        assert_eq!(*written.borrow(), 2u32.to_le_bytes().to_vec());
        let links = e.mem.u32(this + LINKS);
        assert_eq!(
            calls(&e, STREAM_SAVE_FIXED_STRING),
            vec![
                vec![s.addr(), links + LINK_NAME],
                vec![s.addr(), links + LINK_INFO_SIZE + LINK_NAME]
            ]
        );
        let saved = calls(&e, QUAT_TRANSFORM_SAVE_BINARY);
        assert_eq!(saved.len(), 4);
        assert_eq!(saved[1], vec![links + LINK_END, s.addr()]);
        let data = e.mem.u32(this + LERP_DATA);
        assert_eq!(
            calls(&e, slot(STREAM_SLOT_SAVE_LINK_ID)),
            vec![vec![s.addr(), data]]
        );
    }
}

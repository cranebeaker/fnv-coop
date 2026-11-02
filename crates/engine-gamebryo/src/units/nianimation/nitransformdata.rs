//! `NiAnimation/nitransformdata.cpp` (Xbox PDB source unit), subsystem `NiAnimation`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::nimain::nimainsdm::{interlocked_decrement, interlocked_increment};
use crate::units::nimain::nistream::{NiBinaryStream, NiStream};

/// `NiTransformData::ms_RTTI` (the class's `NiRTTI` record).
const NI_TRANSFORM_DATA_RTTI: u32 = 0x011f_3fbc;

// Translated from 00a4e0a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTransformData::GetRTTI` (Xbox PDB): the address of the class's `NiRTTI` record.
pub fn ni_transform_data_get_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(NI_TRANSFORM_DATA_RTTI)
}

// Translated from 00a4e5b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTransformData::_scalar_deleting_destructor_` (Xbox PDB): runs the destructor body at `00a4e270`,
/// then the sized delete (`00aa1460`, `(this, size)`) of 0x2c bytes when bit 0 of
/// `flags` is set. Answers `this`.
pub fn ni_transform_data_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    const DESTRUCTOR_BODY: u32 = 0x00a4_e270;
    const SIZED_DELETE: u32 = 0x00aa_1460;
    e.call(DESTRUCTOR_BODY, &args![this]);
    if flags & 1 != 0 {
        e.call(SIZED_DELETE, &args![this, 0x2cu32]);
    }
    this
}

layout! {
    /// `NiTransformData` (Xbox PDB), 0x2c bytes on PC as on the Xbox: an `NiObject` and the
    /// rotation, position and scale key lists (counts, key types, key sizes, keys).
    pub struct NiTransformData: 0x2C {
        /// `m_usNumRotKeys` (Xbox PDB).
        0x08 m_usNumRotKeys: u16,
        /// `m_usNumPosKeys` (Xbox PDB).
        0x0A m_usNumPosKeys: u16,
        /// `m_usNumScaleKeys` (Xbox PDB).
        0x0C m_usNumScaleKeys: u16,
        /// `m_eRotType` (Xbox PDB): `NiAnimationKey::KeyType`.
        0x10 m_eRotType: u32,
        /// `m_ePosType` (Xbox PDB).
        0x14 m_ePosType: u32,
        /// `m_eScaleType` (Xbox PDB).
        0x18 m_eScaleType: u32,
        /// `m_ucRotSize` (Xbox PDB).
        0x1C m_ucRotSize: u8,
        /// `m_ucPosSize` (Xbox PDB).
        0x1D m_ucPosSize: u8,
        /// `m_ucScaleSize` (Xbox PDB).
        0x1E m_ucScaleSize: u8,
        /// `m_pkRotKeys` (Xbox PDB): `NiRotKey *`.
        0x20 m_pkRotKeys: u32,
        /// `m_pkPosKeys` (Xbox PDB): `NiPosKey *`.
        0x24 m_pkPosKeys: u32,
        /// `m_pkScaleKeys` (Xbox PDB): `NiFloatKey *`.
        0x28 m_pkScaleKeys: u32,
    }
}

/// `NiTransformData`'s vtable.
const NI_TRANSFORM_DATA_VTABLE: u32 = 0x0109_9004;
/// `operator new` (`cdecl`, the size).
const OPERATOR_NEW: u32 = 0x00aa_13e0;
/// `NiObject`'s constructor and destructor body (`thiscall`, no stack words) and its
/// `LoadBinary` (`thiscall`, the stream).
const OBJECT_CONSTRUCT: u32 = 0x00a5_d3a0;
const OBJECT_DESTRUCT: u32 = 0x00a5_d3d0;
const OBJECT_LOAD_BINARY: u32 = 0x00a5_d610;
/// `00a29ad0` (`thiscall` on the keys): what the rotation keys of type 4 (Euler keys) go
/// through before they are deleted.
const EULER_KEYS_RELEASE: u32 = 0x00a2_9ad0;
/// The animation key function tables, one word per key type: the delete functions
/// (`cdecl`, the keys) of the rotation (`011f38d0`), position (`011f38b8`) and scale
/// (`011f38a0`) keys; the create functions (`cdecl`, (stream, count), answer the keys) at
/// `011f3de0`, `011f3dc8` and `011f3db0`; the load functions (`cdecl`, (keys, count, size))
/// at `011f37b0`, `011f3798` and `011f3780`; and the key size bytes at `011f3764`, `011f375e`
/// and `011f3758`.
const ROT_DELETE: u32 = 0x011f_38d0;
const POS_DELETE: u32 = 0x011f_38b8;
const SCALE_DELETE: u32 = 0x011f_38a0;
const ROT_CREATE: u32 = 0x011f_3de0;
const POS_CREATE: u32 = 0x011f_3dc8;
const SCALE_CREATE: u32 = 0x011f_3db0;
const ROT_LOAD: u32 = 0x011f_37b0;
const POS_LOAD: u32 = 0x011f_3798;
const SCALE_LOAD: u32 = 0x011f_3780;
const ROT_SIZE: u32 = 0x011f_3764;
const POS_SIZE: u32 = 0x011f_375e;
const SCALE_SIZE: u32 = 0x011f_3758;
/// `00a29680` (`cdecl`, the message): the warning the loader gives for more than 65535 keys,
/// with the messages `"Transform data loading has more than 65535 rotation keys!"`
/// (`01099104`), `"... position keys!"` (`010990c8`) and `"... scale keys!"` (`01099090`).
const WARNING: u32 = 0x00a2_9680;
const TOO_MANY_ROTATION_KEYS: u32 = 0x0109_9104;
const TOO_MANY_POSITION_KEYS: u32 = 0x0109_90c8;
const TOO_MANY_SCALE_KEYS: u32 = 0x0109_9090;

/// Deletes the rotation keys through the delete function of their type (keys of type 4
/// first through `00a29ad0`), when there are keys.
fn delete_rotation_keys(e: &mut Engine, this: Ptr<NiTransformData>) {
    let keys = e.get(this, NiTransformData::m_pkRotKeys);
    if keys != 0 {
        if e.get(this, NiTransformData::m_eRotType) == 4 {
            e.call(EULER_KEYS_RELEASE, &args![keys]);
        }
        let kind = e.get(this, NiTransformData::m_eRotType);
        let delete = e.mem.u32(ROT_DELETE.wrapping_add(kind.wrapping_mul(4)));
        e.call(delete, &args![keys]);
    }
}

/// Deletes the keys at `keys_field` through the delete function of the type at `type_field`
/// in `table`, when there are keys.
fn delete_keys(
    e: &mut Engine,
    this: Ptr<NiTransformData>,
    keys_field: Field<NiTransformData, u32>,
    type_field: Field<NiTransformData, u32>,
    table: u32,
) {
    let keys = e.get(this, keys_field);
    if keys != 0 {
        let kind = e.get(this, type_field);
        let delete = e.mem.u32(table.wrapping_add(kind.wrapping_mul(4)));
        e.call(delete, &args![keys]);
    }
}

// Translated from 00a4e040 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (a function Ghidra did not create; by its body the creation
/// function of `NiTransformData`): allocates 0x2c bytes (`operator new`); when that
/// succeeds, the `NiObject` constructor, every key list empty (counts, types, sizes and key
/// pointers 0) and the vtable `01099004`. Answers the object, or null.
pub fn fn_00a4e040(e: &mut Engine) -> Ptr<NiTransformData> {
    let block = e.call(OPERATOR_NEW, &args![0x2cu32]).u32();
    if block == 0 {
        return Ptr::new(0);
    }
    e.call(OBJECT_CONSTRUCT, &args![block]);
    let this = Ptr::<NiTransformData>::new(block);
    e.set(this, NiTransformData::m_usNumRotKeys, 0);
    e.set(this, NiTransformData::m_pkRotKeys, 0);
    e.set(this, NiTransformData::m_eRotType, 0);
    e.set(this, NiTransformData::m_ucRotSize, 0);
    e.set(this, NiTransformData::m_pkPosKeys, 0);
    e.set(this, NiTransformData::m_ePosType, 0);
    e.set(this, NiTransformData::m_ucPosSize, 0);
    e.set(this, NiTransformData::m_pkScaleKeys, 0);
    e.set(this, NiTransformData::m_eScaleType, 0);
    e.set(this, NiTransformData::m_ucScaleSize, 0);
    e.mem.set_u32(block, NI_TRANSFORM_DATA_VTABLE);
    e.set(this, NiTransformData::m_usNumPosKeys, 0);
    e.set(this, NiTransformData::m_usNumScaleKeys, 0);
    this
}

// Translated from 00a4e270 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `nitransformdata.cpp`; the destructor body): stores the
/// vtable `01099004`, deletes the rotation, position and scale keys through their types'
/// delete functions (rotation keys of type 4 first through `00a29ad0`) and runs the
/// `NiObject` destructor body (`00a5d3d0`).
pub fn fn_00a4e270(e: &mut Engine, this: Ptr<NiTransformData>) {
    e.mem.set_u32(this.addr(), NI_TRANSFORM_DATA_VTABLE);
    delete_rotation_keys(e, this);
    delete_keys(
        e,
        this,
        NiTransformData::m_pkPosKeys,
        NiTransformData::m_ePosType,
        POS_DELETE,
    );
    delete_keys(
        e,
        this,
        NiTransformData::m_pkScaleKeys,
        NiTransformData::m_eScaleType,
        SCALE_DELETE,
    );
    e.call(OBJECT_DESTRUCT, &args![this]);
}

// Translated from 00a4e2e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `nitransformdata.cpp`; by its body the rotation key
/// replacement): deletes the current rotation keys (as the destructor does) and, when `keys`
/// and `count` are both non-zero, takes them with `kind` and that type's key size
/// (`011f3764`); otherwise the rotation list is left empty (count, keys, type and size 0).
pub fn fn_00a4e2e0(e: &mut Engine, this: Ptr<NiTransformData>, keys: u32, count: u32, kind: u32) {
    delete_rotation_keys(e, this);
    if keys != 0 && count != 0 {
        e.set(this, NiTransformData::m_usNumRotKeys, count as u16);
        e.set(this, NiTransformData::m_pkRotKeys, keys);
        e.set(this, NiTransformData::m_eRotType, kind);
        let size = e.mem.u8(kind.wrapping_add(ROT_SIZE));
        e.set(this, NiTransformData::m_ucRotSize, size);
    } else {
        e.set(this, NiTransformData::m_usNumRotKeys, 0);
        e.set(this, NiTransformData::m_pkRotKeys, 0);
        e.set(this, NiTransformData::m_eRotType, 0);
        e.set(this, NiTransformData::m_ucRotSize, 0);
    }
}

// Translated from 00a4e350 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `nitransformdata.cpp`; by its body the position key
/// replacement): deletes the current position keys through their type's delete function
/// and, when `keys` and `count` are both non-zero, takes them with `kind` and that type's key
/// size (`011f375e`); otherwise the position list is left empty.
pub fn fn_00a4e350(e: &mut Engine, this: Ptr<NiTransformData>, keys: u32, count: u32, kind: u32) {
    delete_keys(
        e,
        this,
        NiTransformData::m_pkPosKeys,
        NiTransformData::m_ePosType,
        POS_DELETE,
    );
    if keys != 0 && count != 0 {
        e.set(this, NiTransformData::m_usNumPosKeys, count as u16);
        e.set(this, NiTransformData::m_pkPosKeys, keys);
        e.set(this, NiTransformData::m_ePosType, kind);
        let size = e.mem.u8(kind.wrapping_add(POS_SIZE));
        e.set(this, NiTransformData::m_ucPosSize, size);
    } else {
        e.set(this, NiTransformData::m_usNumPosKeys, 0);
        e.set(this, NiTransformData::m_pkPosKeys, 0);
        e.set(this, NiTransformData::m_ePosType, 0);
        e.set(this, NiTransformData::m_ucPosSize, 0);
    }
}

// Translated from 00a4e3b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `nitransformdata.cpp`; by its body the scale key
/// replacement): deletes the current scale keys through their type's delete function and,
/// when `keys` and `count` are both non-zero, takes them with `kind` and that type's key size
/// (`011f3758`); otherwise the scale list is left empty.
pub fn fn_00a4e3b0(e: &mut Engine, this: Ptr<NiTransformData>, keys: u32, count: u32, kind: u32) {
    delete_keys(
        e,
        this,
        NiTransformData::m_pkScaleKeys,
        NiTransformData::m_eScaleType,
        SCALE_DELETE,
    );
    if keys != 0 && count != 0 {
        e.set(this, NiTransformData::m_usNumScaleKeys, count as u16);
        e.set(this, NiTransformData::m_pkScaleKeys, keys);
        e.set(this, NiTransformData::m_eScaleType, kind);
        let size = e.mem.u8(kind.wrapping_add(SCALE_SIZE));
        e.set(this, NiTransformData::m_ucScaleSize, size);
    } else {
        e.set(this, NiTransformData::m_usNumScaleKeys, 0);
        e.set(this, NiTransformData::m_pkScaleKeys, 0);
        e.set(this, NiTransformData::m_ucScaleSize, 0);
        e.set(this, NiTransformData::m_eScaleType, 0);
    }
}

/// Reads 4 bytes from the stream's input into `destination`, the byte count word at
/// `count` set to 4 first (`function(input, destination, 4, count, 1)`; the answer is added
/// to the input's position).
fn read_word(e: &mut Engine, stream: Ptr<NiStream>, destination: u32, count: u32) {
    let input = Ptr::<NiBinaryStream>::new(e.get(stream, NiStream::m_pkIstr));
    let function = e.get(input, NiBinaryStream::m_pfnRead);
    e.mem.set_u32(count, 4);
    let done = e
        .call(function, &args![input, destination, 4u32, count, 1u32])
        .u32();
    let position = e.get(input, NiBinaryStream::m_uiAbsoluteCurrentPos);
    e.set(
        input,
        NiBinaryStream::m_uiAbsoluteCurrentPos,
        position.wrapping_add(done),
    );
}

/// One key list of `LoadBinary`: the stack words it uses (the key count in the stream
/// argument's slot, the type and the byte-count word), its tables and its message.
struct KeyList {
    kind_slot: u32,
    count_slot: u32,
    /// The local whose low byte the key size byte is stored into (the type's own slot for
    /// rotations, the byte-count word otherwise).
    size_slot: u32,
    size_table: u32,
    create_table: u32,
    load_table: u32,
    message: u32,
    replace: fn(&mut Engine, Ptr<NiTransformData>, u32, u32, u32),
}

/// Reads one key list: its count; when not 0 its type, the keys made by the type's create
/// function (`(stream, count)`) and filled by its load function (`(keys, count, size
/// word)`, the size word being the slot the key size byte was stored into), the count cut to
/// 65535 with a warning, and the list replaced.
fn load_key_list(
    e: &mut Engine,
    this: Ptr<NiTransformData>,
    stream: Ptr<NiStream>,
    count_word: u32,
    list: &KeyList,
) {
    read_word(e, stream, count_word, list.count_slot);
    if e.mem.u32(count_word) == 0 {
        return;
    }
    read_word(e, stream, list.kind_slot, list.count_slot);
    let kind = e.mem.u32(list.kind_slot);
    let size = e.mem.u8(kind.wrapping_add(list.size_table));
    let count = e.mem.u32(count_word);
    let size_slot = list.size_slot;
    e.mem.set_u8(size_slot, size);
    let create = e
        .mem
        .u32(list.create_table.wrapping_add(kind.wrapping_mul(4)));
    let keys = e.call(create, &args![stream, count]).u32();
    let size_word = e.mem.u32(size_slot);
    let load = e
        .mem
        .u32(list.load_table.wrapping_add(kind.wrapping_mul(4)));
    e.call(load, &args![keys, count, size_word]);
    if e.mem.u32(count_word) > 0xffff {
        e.mem.set_u32(count_word, 0xffff);
        e.call(WARNING, &args![list.message]);
    }
    let count = e.mem.u32(count_word);
    (list.replace)(e, this, keys, count, kind);
}

// Translated from 00a4e5d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTransformData::LoadBinary` (Xbox PDB): `NiObject::LoadBinary`, then the rotation,
/// position and scale key lists in turn, each as a 4-byte count and, when not 0, a 4-byte key
/// type; the keys are made and read through the key type's create and load functions
/// (`011f3de0`/`011f37b0`, `011f3dc8`/`011f3798`, `011f3db0`/`011f3780`), the count is cut to
/// 65535 with a warning (`00a29680`) and the list replaced (`00a4e2e0`, `00a4e350`,
/// `00a4e3b0`). The rotation list's load function is given the key size byte over the type
/// word's low byte; the other two the size byte alone (their byte-count word was 4).
pub fn ni_transform_data_load_binary(
    e: &mut Engine,
    this: Ptr<NiTransformData>,
    stream: Ptr<NiStream>,
) {
    e.call(OBJECT_LOAD_BINARY, &args![this, stream]);
    // Stack words: the stream argument's slot (the counts) and two locals.
    e.with_stack(0xc, |e, slots| {
        let count_word = slots.addr();
        let first = slots.addr() + 4;
        let second = slots.addr() + 8;
        let rotation = KeyList {
            kind_slot: second,
            count_slot: first,
            size_slot: second,
            size_table: ROT_SIZE,
            create_table: ROT_CREATE,
            load_table: ROT_LOAD,
            message: TOO_MANY_ROTATION_KEYS,
            replace: fn_00a4e2e0,
        };
        let position = KeyList {
            kind_slot: first,
            count_slot: second,
            size_slot: second,
            size_table: POS_SIZE,
            create_table: POS_CREATE,
            load_table: POS_LOAD,
            message: TOO_MANY_POSITION_KEYS,
            replace: fn_00a4e350,
        };
        let scale = KeyList {
            kind_slot: first,
            count_slot: second,
            size_slot: second,
            size_table: SCALE_SIZE,
            create_table: SCALE_CREATE,
            load_table: SCALE_LOAD,
            message: TOO_MANY_SCALE_KEYS,
            replace: fn_00a4e3b0,
        };
        load_key_list(e, this, stream, count_word, &rotation);
        load_key_list(e, this, stream, count_word, &position);
        load_key_list(e, this, stream, count_word, &scale);
    });
}

/// `NiAnimationKey::GuaranteeKeyAtStartAndEnd` (`00a3d930`, `cdecl`, (content, type, &keys,
/// &count, begin, end)) and `NiAnimationKey::CopySequence` (`00a3d630`, `cdecl`, (content,
/// type, keys, count, begin, end, &new keys, &new count)); the content is 2 for rotation, 1
/// for position and 0 for scale keys as these calls pass it.
const GUARANTEE_KEY_AT_START_AND_END: u32 = 0x00a3_d930;
const COPY_SEQUENCE: u32 = 0x00a3_d630;
/// `NiObject::IsEqual` (`00a5d470`), `NiObject::CreateDeepCopy` (`00a5d510`, `thiscall`, the
/// `NiPointer` slot to fill), `NiObject::GetViewerStrings` (`00a5d6e0`) and the base
/// `SaveBinary` this class calls (`00a89af0`, unnamed in the engine map), each `thiscall`
/// with one word.
const OBJECT_IS_EQUAL: u32 = 0x00a5_d470;
const OBJECT_CREATE_DEEP_COPY: u32 = 0x00a5_d510;
const OBJECT_GET_VIEWER_STRINGS: u32 = 0x00a5_d6e0;
const BASE_SAVE_BINARY: u32 = 0x00a8_9af0;
/// The key equal functions (`cdecl`, two keys) and save functions (`cdecl`, (stream, keys,
/// count)) per key type: rotation `011f3b10` / `011f3a80`, position `011f3af8` / `011f3a68`,
/// scale `011f3ae0` / `011f3a50`.
const ROT_EQUAL: u32 = 0x011f_3b10;
const POS_EQUAL: u32 = 0x011f_3af8;
const SCALE_EQUAL: u32 = 0x011f_3ae0;
const ROT_SAVE: u32 = 0x011f_3a80;
const POS_SAVE: u32 = 0x011f_3a68;
const SCALE_SAVE: u32 = 0x011f_3a50;
/// `NiGetViewerString` (`00a7d330`, `cdecl`, the text) and its `(name, unsigned short)`
/// overload (`00a7d490`), and the viewer-string array's grow (`0096ad30`, `thiscall`, the new
/// size) and set (`0096ae90`, `thiscall`, (index, &text)).
const NI_GET_VIEWER_STRING: u32 = 0x00a7_d330;
const NI_GET_VIEWER_STRING_USHORT: u32 = 0x00a7_d490;
const STRINGS_GROW: u32 = 0x0096_ad30;
const STRINGS_SET_AT: u32 = 0x0096_ae90;
/// The viewer-string names `"m_usNumRotKeys"` (`01099164`), `"m_usNumPosKeys"` (`01099154`)
/// and `"m_usNumScaleKeys"` (`01099140`).
const NAME_ROT_KEYS: u32 = 0x0109_9164;
const NAME_POS_KEYS: u32 = 0x0109_9154;
const NAME_SCALE_KEYS: u32 = 0x0109_9140;

/// One key list's fields in the order the class's functions visit them: rotation, position,
/// scale.
struct Channel {
    content: u32,
    count: Field<NiTransformData, u16>,
    kind: Field<NiTransformData, u32>,
    size: Field<NiTransformData, u8>,
    keys: Field<NiTransformData, u32>,
}

const CHANNELS: [Channel; 3] = [
    Channel {
        content: 2,
        count: NiTransformData::m_usNumRotKeys,
        kind: NiTransformData::m_eRotType,
        size: NiTransformData::m_ucRotSize,
        keys: NiTransformData::m_pkRotKeys,
    },
    Channel {
        content: 1,
        count: NiTransformData::m_usNumPosKeys,
        kind: NiTransformData::m_ePosType,
        size: NiTransformData::m_ucPosSize,
        keys: NiTransformData::m_pkPosKeys,
    },
    Channel {
        content: 0,
        count: NiTransformData::m_usNumScaleKeys,
        kind: NiTransformData::m_eScaleType,
        size: NiTransformData::m_ucScaleSize,
        keys: NiTransformData::m_pkScaleKeys,
    },
];

/// Writes 4 bytes from `source` to the stream's output, the byte-count word at `count` set to
/// 4 first; the answer is added to the output's position.
fn write_word(e: &mut Engine, stream: Ptr<NiStream>, source: u32, count: u32) {
    let output = Ptr::<NiBinaryStream>::new(e.get(stream, NiStream::m_pkOstr));
    let function = e.get(output, NiBinaryStream::m_pfnWrite);
    e.mem.set_u32(count, 4);
    let done = e
        .call(function, &args![output, source, 4u32, count, 1u32])
        .u32();
    let position = e.get(output, NiBinaryStream::m_uiAbsoluteCurrentPos);
    e.set(
        output,
        NiBinaryStream::m_uiAbsoluteCurrentPos,
        position.wrapping_add(done),
    );
}

/// Releases the counted object `object` when there is one (`InterlockedDecrement` of its count
/// at +4; its slot +4 at zero).
fn release(e: &mut Engine, object: u32) {
    if object != 0 && interlocked_decrement(e, object + 4) == 0 {
        e.vcall(object, 4, &args![]);
    }
}

// Translated from 00a4df40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTransformData::GuaranteeKeysAtStartAndEnd` (Xbox PDB; `thiscall`, (begin, end)): for each
/// key list that has keys (rotation, position, scale), `NiAnimationKey::GuaranteeKeyAtStartAndEnd`
/// (`00a3d930`) with the list's content (2, 1, 0), type, keys and count in two stack words and
/// the times (through the x87 stack); the keys and the count's low 16 bits it leaves there are
/// stored back.
pub fn ni_transform_data_guarantee_keys_at_start_and_end(
    e: &mut Engine,
    this: Ptr<NiTransformData>,
    begin: f32,
    end: f32,
) {
    let begin = x87_quiet_bits(begin.to_bits());
    let end = x87_quiet_bits(end.to_bits());
    // The stack locals: the count (ESP-8) and the keys (ESP-4).
    e.with_stack(8, |e, locals| {
        let count_word = locals.addr();
        let keys_word = count_word + 4;
        for channel in &CHANNELS {
            let count = e.get(this, channel.count);
            if count == 0 {
                continue;
            }
            let keys = e.get(this, channel.keys);
            e.mem.set_u32(count_word, u32::from(count));
            e.mem.set_u32(keys_word, keys);
            let kind = e.get(this, channel.kind);
            e.call(
                GUARANTEE_KEY_AT_START_AND_END,
                &args![channel.content, kind, keys_word, count_word, begin, end],
            );
            let count = e.mem.u16(count_word);
            e.set(this, channel.count, count);
            let keys = e.mem.u32(keys_word);
            e.set(this, channel.keys, keys);
        }
    });
}

// Translated from 00a4e0b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, (keys, count, type)): takes position keys without
/// deleting the current ones: when `keys` and `count` are both non-zero, the count's low 16
/// bits, the keys, `kind` and that type's key size (`011f375e`); otherwise all four zero.
pub fn fn_00a4e0b0(e: &mut Engine, this: Ptr<NiTransformData>, keys: u32, count: u32, kind: u32) {
    if keys != 0 && count != 0 {
        e.set(this, NiTransformData::m_usNumPosKeys, count as u16);
        e.set(this, NiTransformData::m_pkPosKeys, keys);
        e.set(this, NiTransformData::m_ePosType, kind);
        let size = e.mem.u8(kind.wrapping_add(POS_SIZE));
        e.set(this, NiTransformData::m_ucPosSize, size);
    } else {
        e.set(this, NiTransformData::m_usNumPosKeys, 0);
        e.set(this, NiTransformData::m_pkPosKeys, 0);
        e.set(this, NiTransformData::m_ePosType, 0);
        e.set(this, NiTransformData::m_ucPosSize, 0);
    }
}

// Translated from 00a4e0f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, (keys, count, type)): takes scale keys without
/// deleting the current ones, as `00a4e0b0` for the scale list (sizes `011f3758`).
pub fn fn_00a4e0f0(e: &mut Engine, this: Ptr<NiTransformData>, keys: u32, count: u32, kind: u32) {
    if keys != 0 && count != 0 {
        e.set(this, NiTransformData::m_usNumScaleKeys, count as u16);
        e.set(this, NiTransformData::m_pkScaleKeys, keys);
        e.set(this, NiTransformData::m_eScaleType, kind);
        let size = e.mem.u8(kind.wrapping_add(SCALE_SIZE));
        e.set(this, NiTransformData::m_ucScaleSize, size);
    } else {
        e.set(this, NiTransformData::m_usNumScaleKeys, 0);
        e.set(this, NiTransformData::m_pkScaleKeys, 0);
        e.set(this, NiTransformData::m_ucScaleSize, 0);
        e.set(this, NiTransformData::m_eScaleType, 0);
    }
}

// Translated from 00a4e130 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTransformData::IsEqual` (Xbox PDB; `thiscall`, the other object): equal when
/// `NiObject::IsEqual` (`00a5d470`) is, the three lists' counts and types and the three size
/// bytes are equal, and every key of each list equals the other's (the equal function of its
/// type: rotation `011f3b10`, position `011f3af8`, scale `011f3ae0`; `this`'s count and size).
pub fn ni_transform_data_is_equal(
    e: &mut Engine,
    this: Ptr<NiTransformData>,
    other: Ptr<NiTransformData>,
) -> bool {
    if !e.call(OBJECT_IS_EQUAL, &args![this, other]).bool() {
        return false;
    }
    let same_lists = CHANNELS.iter().all(|c| {
        e.get(this, c.count) == e.get(other, c.count) && e.get(this, c.kind) == e.get(other, c.kind)
    });
    let same_sizes = CHANNELS
        .iter()
        .all(|c| e.get(this, c.size) == e.get(other, c.size));
    if !same_lists || !same_sizes {
        return false;
    }
    for (channel, table) in CHANNELS.iter().zip([ROT_EQUAL, POS_EQUAL, SCALE_EQUAL]) {
        let kind = e.get(this, channel.kind);
        let equal = e.mem.u32(table.wrapping_add(kind.wrapping_mul(4)));
        let mut i = 0u32;
        while i < u32::from(e.get(this, channel.count)) {
            let offset = u32::from(e.get(this, channel.size)).wrapping_mul(i);
            let mine = e.get(this, channel.keys).wrapping_add(offset);
            let theirs = e.get(other, channel.keys).wrapping_add(offset);
            if !e.call(equal, &args![mine, theirs]).bool() {
                return false;
            }
            i += 1;
        }
    }
    true
}

// Translated from 00a4e410 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTransformData::GetSequenceData` (Xbox PDB; `thiscall`, (begin, end, &data)): a deep copy
/// (`NiObject::CreateDeepCopy`, `00a5d510`, into a stack `NiPointer` that starts null) held
/// as transform data (counted), and for each list with keys (rotation, position, scale) the
/// keys between the times (`NiAnimationKey::CopySequence`, `00a3d630`, into two stack words)
/// given to the copy's list (`00a4e2e0`, `00a4e350`, `00a4e3b0`, with the list's type). The
/// result goes to `data` (the counted `NiPointer` assignment); then both stack pointers are
/// released (the held one skipped when it is null).
pub fn ni_transform_data_get_sequence_data(
    e: &mut Engine,
    this: Ptr<NiTransformData>,
    begin: f32,
    end: f32,
    data: Ptr,
) {
    let begin = x87_quiet_bits(begin.to_bits());
    let end = x87_quiet_bits(end.to_bits());
    // The stack locals: the new count (ESP-0xc), the new keys (ESP-8), the copy (ESP-4).
    e.with_stack(0xc, |e, locals| {
        let count_word = locals.addr();
        let keys_word = count_word + 4;
        let copy_slot = count_word + 8;
        e.mem.set_u32(copy_slot, 0);
        e.call(OBJECT_CREATE_DEEP_COPY, &args![this, copy_slot]);
        let copy = e.mem.u32(copy_slot);
        if copy != 0 {
            interlocked_increment(e, copy + 4);
        }
        e.mem.set_u32(keys_word, 0);
        e.mem.set_u32(count_word, 0);
        type Replace = fn(&mut Engine, Ptr<NiTransformData>, u32, u32, u32);
        let replaces: [Replace; 3] = [fn_00a4e2e0, fn_00a4e350, fn_00a4e3b0];
        for (channel, replace) in CHANNELS.iter().zip(replaces) {
            let count = e.get(this, channel.count);
            if count == 0 {
                continue;
            }
            let kind = e.get(this, channel.kind);
            let keys = e.get(this, channel.keys);
            e.call(
                COPY_SEQUENCE,
                &args![
                    channel.content,
                    kind,
                    keys,
                    u32::from(count),
                    begin,
                    end,
                    keys_word,
                    count_word
                ],
            );
            let kind = e.get(this, channel.kind);
            let new_keys = e.mem.u32(keys_word);
            let new_count = e.mem.u32(count_word);
            replace(e, Ptr::new(copy), new_keys, new_count, kind);
        }
        let old = e.mem.u32(data.addr());
        let mut release_held = true;
        if old != copy {
            release(e, old);
            e.mem.set_u32(data.addr(), copy);
            if copy == 0 {
                release_held = false;
            } else {
                interlocked_increment(e, copy + 4);
            }
        }
        if release_held {
            release(e, copy);
        }
        let held = e.mem.u32(copy_slot);
        release(e, held);
    });
}

// Translated from 00a4e810 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTransformData::SaveBinary` (Xbox PDB; `thiscall`, the stream): the base `SaveBinary`
/// (`00a89af0`), then for each list (rotation, position, scale) its count as a 4-byte word
/// (put in the stream argument's stack word) and, when it is non-zero, its 4-byte type and its
/// keys through the type's save function (`011f3a80`, `011f3a68`, `011f3a50`; (stream, keys,
/// count)). The byte-count word is the one stack local.
pub fn ni_transform_data_save_binary(
    e: &mut Engine,
    this: Ptr<NiTransformData>,
    stream: Ptr<NiStream>,
) {
    e.call(BASE_SAVE_BINARY, &args![this, stream]);
    // The stack words: the byte-count local (ESP-4) and the stream argument.
    e.with_stack(8, |e, words| {
        let count_word = words.addr();
        let argument = count_word + 4;
        for (channel, table) in CHANNELS.iter().zip([ROT_SAVE, POS_SAVE, SCALE_SAVE]) {
            let count = u32::from(e.get(this, channel.count));
            e.mem.set_u32(argument, count);
            write_word(e, stream, argument, count_word);
            if e.get(this, channel.count) == 0 {
                continue;
            }
            write_word(e, stream, this.addr() + channel.kind.off, count_word);
            let count = u32::from(e.get(this, channel.count));
            let keys = e.get(this, channel.keys);
            e.mem.set_u32(argument, count);
            let kind = e.get(this, channel.kind);
            let save = e.mem.u32(table.wrapping_add(kind.wrapping_mul(4)));
            e.call(save, &args![stream, keys, count]);
        }
    });
}

// Translated from 00a4e990 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTransformData::GetViewerStrings` (Xbox PDB; `thiscall`, the strings): `NiObject`'s
/// strings (`00a5d6e0`), then appended: the class name (the first word of the `NiRTTI` record
/// at `011f3fbc`, through `NiGetViewerString`) and the three key counts as `(name, count)`
/// strings (`00a7d490`): rotation, position, scale. Each append grows the array to the step
/// (+0xE) plus the size when the 16-bit size (+0xA) has reached the capacity (+0x8); each text
/// goes in through the strings argument's stack word.
pub fn ni_transform_data_get_viewer_strings(
    e: &mut Engine,
    this: Ptr<NiTransformData>,
    strings: u32,
) {
    e.call(OBJECT_GET_VIEWER_STRINGS, &args![this, strings]);
    e.with_stack(4, |e, slot| {
        let append = |e: &mut Engine, text: u32| {
            e.mem.set_u32(slot.addr(), text);
            let size = u32::from(e.mem.u16(strings + 0xa));
            if size >= u32::from(e.mem.u16(strings + 8)) {
                let step = u32::from(e.mem.u16(strings + 0xe));
                e.call(STRINGS_GROW, &args![strings, step + size]);
            }
            e.call(STRINGS_SET_AT, &args![strings, size, slot]);
        };
        let class_name = e.mem.u32(NI_TRANSFORM_DATA_RTTI);
        let text = e.call(NI_GET_VIEWER_STRING, &args![class_name]).u32();
        append(e, text);
        for (name, channel) in [NAME_ROT_KEYS, NAME_POS_KEYS, NAME_SCALE_KEYS]
            .iter()
            .zip(&CHANNELS)
        {
            let count = u32::from(e.get(this, channel.count));
            let text = e
                .call(NI_GET_VIEWER_STRING_USHORT, &args![*name, count])
                .u32();
            append(e, text);
        }
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a4e0a0, ni_transform_data_get_rtti() -> Ptr),
        entry!(0x00a4e5b0, ni_transform_data_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x00a4_e040, fn_00a4e040() -> Ptr<NiTransformData>),
        entry!(0x00a4_e270, fn_00a4e270(Ptr<NiTransformData>)),
        entry!(
            0x00a4_e2e0,
            fn_00a4e2e0(Ptr<NiTransformData>, u32, u32, u32)
        ),
        entry!(
            0x00a4_e350,
            fn_00a4e350(Ptr<NiTransformData>, u32, u32, u32)
        ),
        entry!(
            0x00a4_e3b0,
            fn_00a4e3b0(Ptr<NiTransformData>, u32, u32, u32)
        ),
        entry!(
            0x00a4_e5d0,
            ni_transform_data_load_binary(Ptr<NiTransformData>, Ptr<NiStream>)
        ),
        entry!(
            0x00a4_df40,
            ni_transform_data_guarantee_keys_at_start_and_end(Ptr<NiTransformData>, f32, f32)
        ),
        entry!(
            0x00a4_e0b0,
            fn_00a4e0b0(Ptr<NiTransformData>, u32, u32, u32)
        ),
        entry!(
            0x00a4_e0f0,
            fn_00a4e0f0(Ptr<NiTransformData>, u32, u32, u32)
        ),
        entry!(
            0x00a4_e130,
            ni_transform_data_is_equal(Ptr<NiTransformData>, Ptr<NiTransformData>) -> bool
        ),
        entry!(
            0x00a4_e410,
            ni_transform_data_get_sequence_data(Ptr<NiTransformData>, f32, f32, Ptr)
        ),
        entry!(
            0x00a4_e810,
            ni_transform_data_save_binary(Ptr<NiTransformData>, Ptr<NiStream>)
        ),
        entry!(
            0x00a4_e990,
            ni_transform_data_get_viewer_strings(Ptr<NiTransformData>, u32)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ni_transform_data_get_rtti_returns_the_class_rtti() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00a4e0a0, &args![]).u32(), NI_TRANSFORM_DATA_RTTI);
    }

    #[test]
    fn ni_transform_data_scalar_deleting_destructor_deletes_only_on_bit_0() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register(0x00a4_e270, |_, _| Ret::default());
        e.register(0x00aa_1460, |_, _| Ret::default());
        let this = 0x5000u32;
        let calls_to = |e: &Engine, addr: u32| -> Vec<Vec<u32>> {
            e.call_log
                .as_ref()
                .unwrap()
                .iter()
                .filter(|c| c.0 == addr)
                .map(|c| c.1.clone())
                .collect()
        };
        assert_eq!(e.call(0x00a4e5b0, &args![this, 0xfffe_u32]).u32(), this);
        assert!(calls_to(&e, 0x00aa_1460).is_empty());
        assert_eq!(e.call(0x00a4e5b0, &args![this, 1u32]).u32(), this);
        assert_eq!(calls_to(&e, 0x00aa_1460), vec![vec![this, 0x2c]]);
        assert_eq!(calls_to(&e, 0x00a4_e270), vec![vec![this]; 2]);
    }
}

#[cfg(test)]
mod key_list_tests {
    use super::*;

    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    /// The delete functions of key types 0 to 4 for each list: `0x0300_0r0t` with `r` the
    /// list (1 rotation, 2 position, 3 scale) and `t` the type; the key size bytes are
    /// `0x10 + type` for every list.
    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x011f_3000, 0x2000);
        for kind in 0..5u32 {
            for (list, table) in [(1u32, ROT_DELETE), (2, POS_DELETE), (3, SCALE_DELETE)] {
                let double = 0x0300_0000 + (list << 8) + kind;
                e.mem.set_u32(table + 4 * kind, double);
                e.register(double, |_, _| Ret::default());
            }
            for table in [ROT_SIZE, POS_SIZE, SCALE_SIZE] {
                e.mem.set_u8(table + kind, 0x10 + kind as u8);
            }
        }
        e.register(EULER_KEYS_RELEASE, |_, _| Ret::default());
        e.register(OBJECT_DESTRUCT, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e
    }

    #[test]
    fn create_object_00a4e040_builds_empty_data() {
        let mut e = engine();
        e.register(OPERATOR_NEW, |e, a| {
            let block = e.mem.alloc(a[0]);
            e.mem.write(block, &[0xee; 0x2c]);
            block.into_ret()
        });
        e.register(OBJECT_CONSTRUCT, |_, _| Ret::default());
        let data = e.call(0x00a4_e040, &args![]).u32();
        assert_eq!(e.mem.u32(data), NI_TRANSFORM_DATA_VTABLE);
        assert_eq!(e.mem.u32(data + 4), 0xeeee_eeee);
        for offset in (8..0x2c).step_by(4) {
            let expected = match offset {
                0xc => 0xeeee_0000,
                0x1c => 0xee00_0000,
                _ => 0,
            };
            assert_eq!(e.mem.u32(data + offset), expected, "{offset:#x}");
        }
        e.register(OPERATOR_NEW, |_, _| 0u32.into_ret());
        assert_eq!(e.call(0x00a4_e040, &args![]).u32(), 0);
    }

    #[test]
    fn destructor_body_deletes_every_key_list() {
        let mut e = engine();
        let this = e.new_object::<NiTransformData>();
        e.set(this, NiTransformData::m_pkRotKeys, 0x100);
        e.set(this, NiTransformData::m_eRotType, 4);
        e.set(this, NiTransformData::m_pkScaleKeys, 0x300);
        e.set(this, NiTransformData::m_eScaleType, 2);
        e.call(0x00a4_e270, &args![this]);
        assert_eq!(e.mem.u32(this.addr()), NI_TRANSFORM_DATA_VTABLE);
        assert_eq!(calls_to(&e, EULER_KEYS_RELEASE), vec![vec![0x100]]);
        assert_eq!(calls_to(&e, 0x0300_0104), vec![vec![0x100]]);
        assert_eq!(calls_to(&e, 0x0300_0302), vec![vec![0x300]]);
        assert!(calls_to(&e, 0x0300_0200).is_empty());
        assert_eq!(calls_to(&e, OBJECT_DESTRUCT), vec![vec![this.addr()]]);
    }

    #[test]
    fn replace_rotation_keys_deletes_and_takes_or_empties() {
        let mut e = engine();
        let this = e.new_object::<NiTransformData>();
        e.set(this, NiTransformData::m_pkRotKeys, 0x100);
        e.set(this, NiTransformData::m_eRotType, 1);
        e.call(0x00a4_e2e0, &args![this, 0x500u32, 0x1_0003u32, 3u32]);
        assert!(calls_to(&e, EULER_KEYS_RELEASE).is_empty());
        assert_eq!(calls_to(&e, 0x0300_0101), vec![vec![0x100]]);
        assert_eq!(e.get(this, NiTransformData::m_usNumRotKeys), 3);
        assert_eq!(e.get(this, NiTransformData::m_pkRotKeys), 0x500);
        assert_eq!(e.get(this, NiTransformData::m_eRotType), 3);
        assert_eq!(e.get(this, NiTransformData::m_ucRotSize), 0x13);
        e.call(0x00a4_e2e0, &args![this, 0x600u32, 0u32, 2u32]);
        assert_eq!(calls_to(&e, 0x0300_0103), vec![vec![0x500]]);
        assert_eq!(e.get(this, NiTransformData::m_pkRotKeys), 0);
        assert_eq!(e.get(this, NiTransformData::m_eRotType), 0);
        assert_eq!(e.get(this, NiTransformData::m_ucRotSize), 0);
    }

    #[test]
    fn replace_position_keys_deletes_and_takes_or_empties() {
        let mut e = engine();
        let this = e.new_object::<NiTransformData>();
        e.set(this, NiTransformData::m_pkPosKeys, 0x200);
        e.set(this, NiTransformData::m_ePosType, 2);
        e.call(0x00a4_e350, &args![this, 0x700u32, 2u32, 1u32]);
        assert_eq!(calls_to(&e, 0x0300_0202), vec![vec![0x200]]);
        assert_eq!(e.get(this, NiTransformData::m_usNumPosKeys), 2);
        assert_eq!(e.get(this, NiTransformData::m_pkPosKeys), 0x700);
        assert_eq!(e.get(this, NiTransformData::m_ePosType), 1);
        assert_eq!(e.get(this, NiTransformData::m_ucPosSize), 0x11);
        e.call(0x00a4_e350, &args![this, 0u32, 2u32, 1u32]);
        assert_eq!(e.get(this, NiTransformData::m_usNumPosKeys), 0);
        assert_eq!(e.get(this, NiTransformData::m_pkPosKeys), 0);
        assert_eq!(e.get(this, NiTransformData::m_ucPosSize), 0);
    }

    #[test]
    fn replace_scale_keys_deletes_and_takes_or_empties() {
        let mut e = engine();
        let this = e.new_object::<NiTransformData>();
        e.call(0x00a4_e3b0, &args![this, 0x800u32, 4u32, 2u32]);
        assert!(calls_to(&e, 0x0300_0300).is_empty());
        assert_eq!(e.get(this, NiTransformData::m_usNumScaleKeys), 4);
        assert_eq!(e.get(this, NiTransformData::m_pkScaleKeys), 0x800);
        assert_eq!(e.get(this, NiTransformData::m_eScaleType), 2);
        assert_eq!(e.get(this, NiTransformData::m_ucScaleSize), 0x12);
        e.call(0x00a4_e3b0, &args![this, 0x900u32, 0u32, 2u32]);
        assert_eq!(calls_to(&e, 0x0300_0302), vec![vec![0x800]]);
        assert_eq!(e.get(this, NiTransformData::m_pkScaleKeys), 0);
        assert_eq!(e.get(this, NiTransformData::m_eScaleType), 0);
    }

    #[test]
    fn load_binary_reads_the_three_key_lists() {
        let mut e = engine();
        e.register(OBJECT_LOAD_BINARY, |_, _| Ret::default());
        e.register(WARNING, |_, _| Ret::default());
        // Create functions answer 0x7000 + list; load functions record their words.
        for (list, create, load) in [
            (1u32, ROT_CREATE, ROT_LOAD),
            (2, POS_CREATE, POS_LOAD),
            (3, SCALE_CREATE, SCALE_LOAD),
        ] {
            for kind in 0..5u32 {
                e.mem.set_u32(create + 4 * kind, 0x0310_0000 + list);
                e.mem.set_u32(load + 4 * kind, 0x0320_0000 + list);
            }
            e.register_double(0x0310_0000 + list, move |_, _| (0x7000 + list).into_ret());
            e.register(0x0320_0000 + list, |_, _| Ret::default());
        }
        // The stream answers: rotation count 0x10002, type 0x103 (size byte 0x13 over it);
        // position count 0; scale count 5, type 2.
        let stream = e.new_object::<NiStream>();
        let input = e.new_object::<NiBinaryStream>();
        e.set(stream, NiStream::m_pkIstr, input.addr());
        e.set(input, NiBinaryStream::m_pfnRead, 0x0330_0000);
        let mut answers = std::collections::VecDeque::from(vec![0x1_0002u32, 0x103, 0, 5, 2]);
        e.register_double(0x0330_0000, move |e, a| {
            assert_eq!((a[2], e.mem.u32(a[3]), a[4]), (4, 4, 1));
            e.mem.set_u32(a[1], answers.pop_front().unwrap());
            4u32.into_ret()
        });
        e.mem.set_u8(ROT_SIZE + 0x103, 0x13);
        e.mem.set_u32(ROT_CREATE + 4 * 0x103, 0x0310_0001);
        e.mem.set_u32(ROT_LOAD + 4 * 0x103, 0x0320_0001);
        e.mem.set_u32(ROT_DELETE + 4 * 0x103, 0x0300_0100);
        let this = e.new_object::<NiTransformData>();
        e.call(0x00a4_e5d0, &args![this, stream]);
        assert_eq!(
            calls_to(&e, 0x0310_0001),
            vec![vec![stream.addr(), 0x1_0002]]
        );
        assert_eq!(
            calls_to(&e, 0x0320_0001),
            vec![vec![0x7001, 0x1_0002, 0x113]]
        );
        assert_eq!(calls_to(&e, WARNING), vec![vec![TOO_MANY_ROTATION_KEYS]]);
        assert_eq!(e.get(this, NiTransformData::m_usNumRotKeys), 0xffff);
        assert_eq!(e.get(this, NiTransformData::m_pkRotKeys), 0x7001);
        assert_eq!(e.get(this, NiTransformData::m_eRotType), 0x103);
        assert!(calls_to(&e, 0x0310_0002).is_empty());
        assert_eq!(calls_to(&e, 0x0320_0003), vec![vec![0x7003, 5, 0x12]]);
        assert_eq!(e.get(this, NiTransformData::m_usNumScaleKeys), 5);
        assert_eq!(e.get(this, NiTransformData::m_pkScaleKeys), 0x7003);
        assert_eq!(e.get(input, NiBinaryStream::m_uiAbsoluteCurrentPos), 20);
    }
}

#[cfg(test)]
mod n006_tests {
    use super::*;

    const LOG: u32 = 0x0300_0600;
    const EQUAL: u32 = 0x0300_0610;
    const SAVE: u32 = 0x0300_0620;
    const WRITE: u32 = 0x0300_0630;

    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    /// Counting stand-ins for the imported `InterlockedIncrement` and `InterlockedDecrement`.
    fn counting(e: &mut Engine) {
        e.register(0x00fd_f1c0, |e, a| {
            let count = e.mem.u32(a[0]).wrapping_add(1);
            e.mem.set_u32(a[0], count);
            count.into_ret()
        });
        e.register(0x00fd_f1c4, |e, a| {
            let count = e.mem.u32(a[0]).wrapping_sub(1);
            e.mem.set_u32(a[0], count);
            count.into_ret()
        });
    }

    /// Data with 1, 2 and 3 rotation, position and scale keys (types 4, 5, 6; sizes 0x10,
    /// 0x14, 0x18; keys at 0x7000, 0x7100, 0x7200).
    fn data(e: &mut Engine) -> Ptr<NiTransformData> {
        let this = e.new_object::<NiTransformData>();
        for (i, channel) in CHANNELS.iter().enumerate() {
            let i = i as u32;
            e.set(this, channel.count, 1 + i as u16);
            e.set(this, channel.kind, 4 + i);
            e.set(this, channel.size, 0x10 + 4 * i as u8);
            e.set(this, channel.keys, 0x7000 + 0x100 * i);
        }
        this
    }

    #[test]
    fn guarantee_keys_runs_on_each_list_with_keys() {
        let mut e = Engine::new();
        e.register(GUARANTEE_KEY_AT_START_AND_END, |e, a| {
            // Grows the list by one and moves it.
            let count = e.mem.u32(a[3]);
            e.mem.set_u32(a[3], count + 0x1_0001);
            let keys = e.mem.u32(a[2]);
            e.mem.set_u32(a[2], keys + 0x10);
            Ret::default()
        });
        let this = data(&mut e);
        e.set(this, NiTransformData::m_usNumPosKeys, 0);
        e.call_log = Some(vec![]);
        e.call(0x00a4_df40, &args![this, 0.5f32, 2.5f32]);
        let calls = calls_to(&e, GUARANTEE_KEY_AT_START_AND_END);
        assert_eq!(calls.len(), 2);
        assert_eq!((calls[0][0], calls[0][1]), (2, 4));
        assert_eq!((calls[1][0], calls[1][1]), (0, 6));
        assert_eq!(calls[0][4..], [0.5f32.to_bits(), 2.5f32.to_bits()]);
        // Only the count's low 16 bits come back.
        assert_eq!(e.get(this, NiTransformData::m_usNumRotKeys), 2);
        assert_eq!(e.get(this, NiTransformData::m_pkRotKeys), 0x7010);
        assert_eq!(e.get(this, NiTransformData::m_usNumScaleKeys), 4);
        assert_eq!(e.get(this, NiTransformData::m_pkScaleKeys), 0x7210);
        assert_eq!(e.get(this, NiTransformData::m_pkPosKeys), 0x7100);
    }

    #[test]
    fn the_plain_key_setters_take_or_clear_their_list() {
        let mut e = Engine::new();
        e.map(POS_SIZE, 0x10);
        e.map(SCALE_SIZE, 0x10);
        e.mem.set_u8(POS_SIZE + 2, 0x40);
        e.mem.set_u8(SCALE_SIZE + 2, 0x20);
        let this = data(&mut e);
        e.call_log = Some(vec![]);
        e.call(0x00a4_e0b0, &args![this, 0x9000u32, 0x1_0007u32, 2u32]);
        assert_eq!(e.get(this, NiTransformData::m_usNumPosKeys), 7);
        assert_eq!(e.get(this, NiTransformData::m_pkPosKeys), 0x9000);
        assert_eq!(e.get(this, NiTransformData::m_ePosType), 2);
        assert_eq!(e.get(this, NiTransformData::m_ucPosSize), 0x40);
        e.call(0x00a4_e0f0, &args![this, 0x9100u32, 3u32, 2u32]);
        assert_eq!(e.get(this, NiTransformData::m_usNumScaleKeys), 3);
        assert_eq!(e.get(this, NiTransformData::m_pkScaleKeys), 0x9100);
        assert_eq!(e.get(this, NiTransformData::m_ucScaleSize), 0x20);
        // Nothing is deleted, and no keys clears the list.
        assert_eq!(e.call_log.as_ref().unwrap().len(), 2);
        e.call(0x00a4_e0b0, &args![this, 0x9000u32, 0u32, 2u32]);
        e.call(0x00a4_e0f0, &args![this, 0u32, 3u32, 2u32]);
        assert_eq!(e.get(this, NiTransformData::m_usNumPosKeys), 0);
        assert_eq!(e.get(this, NiTransformData::m_pkScaleKeys), 0);
        assert_eq!(e.get(this, NiTransformData::m_eScaleType), 0);
        assert_eq!(e.get(this, NiTransformData::m_ucPosSize), 0);
    }

    #[test]
    fn is_equal_compares_fields_then_every_key() {
        let mut e = Engine::new();
        e.map(0x011f_3a00, 0x200);
        for table in [ROT_EQUAL, POS_EQUAL, SCALE_EQUAL] {
            for kind in 4..7 {
                e.mem.set_u32(table + 4 * kind, EQUAL);
            }
        }
        e.register(OBJECT_IS_EQUAL, |_, _| 1u32.into_ret());
        e.register(EQUAL, |_, a| (a[1] != 0x7230).into_ret());
        let a = data(&mut e);
        let b = data(&mut e);
        e.call_log = Some(vec![]);
        assert!(!e.call(0x00a4_e130, &args![a, b]).bool());
        let keys: Vec<u32> = calls_to(&e, EQUAL).iter().map(|c| c[0]).collect();
        assert_eq!(keys, vec![0x7000, 0x7100, 0x7114, 0x7200, 0x7218, 0x7230]);
        e.register(EQUAL, |_, _| 1u32.into_ret());
        assert!(e.call(0x00a4_e130, &args![a, b]).bool());
        e.set(b, NiTransformData::m_ucScaleSize, 1);
        e.call_log = Some(vec![]);
        assert!(!e.call(0x00a4_e130, &args![a, b]).bool());
        assert!(calls_to(&e, EQUAL).is_empty());
    }

    /// A counted object (count `count`) deleting through slot +4 into the log.
    fn counted(e: &mut Engine, size: u32, count: u32) -> u32 {
        let vtable = e.mem.alloc(0x10);
        e.mem.set_u32(vtable + 4, LOG);
        let object = e.mem.alloc(size);
        e.mem.set_u32(object, vtable);
        e.mem.set_u32(object + 4, count);
        object
    }

    #[test]
    fn get_sequence_data_copies_each_list_into_the_copy() {
        let mut e = Engine::new();
        counting(&mut e);
        e.register(LOG, |_, _| Ret::default());
        e.map(ROT_SIZE, 0x10);
        e.map(POS_SIZE, 0x10);
        e.map(SCALE_SIZE, 0x10);
        let copy = counted(&mut e, 0x2c, 0);
        e.register_double(OBJECT_CREATE_DEEP_COPY, move |e, a| {
            // The copy goes into the slot counted, as an `NiPointer` assignment does.
            e.mem.set_u32(a[1], copy);
            e.mem.set_u32(copy + 4, 1);
            Ret::default()
        });
        e.register(COPY_SEQUENCE, |e, a| {
            e.mem.set_u32(a[6], 0x8000 + a[0]);
            e.mem.set_u32(a[7], 10 + a[0]);
            Ret::default()
        });
        let this = data(&mut e);
        let old = counted(&mut e, 0x10, 1);
        let out = e.mem.alloc(4);
        e.mem.set_u32(out, old);
        e.call_log = Some(vec![]);
        e.call(0x00a4_e410, &args![this, 0.25f32, 0.75f32, out]);
        let copies = calls_to(&e, COPY_SEQUENCE);
        assert_eq!(copies.len(), 3);
        assert_eq!(
            copies[0][..6],
            [2, 4, 0x7000, 1, 0.25f32.to_bits(), 0.75f32.to_bits()]
        );
        assert_eq!(copies[1][..4], [1, 5, 0x7100, 2]);
        assert_eq!(copies[2][..4], [0, 6, 0x7200, 3]);
        let copy = Ptr::<NiTransformData>::new(copy);
        assert_eq!(e.get(copy, NiTransformData::m_pkRotKeys), 0x8002);
        assert_eq!(e.get(copy, NiTransformData::m_usNumRotKeys), 12);
        assert_eq!(e.get(copy, NiTransformData::m_ePosType), 5);
        assert_eq!(e.get(copy, NiTransformData::m_usNumScaleKeys), 10);
        assert_eq!(e.mem.u32(out), copy.addr());
        // The old object released and deleted; the copy counted by the result alone.
        assert_eq!(calls_to(&e, LOG), vec![vec![old]]);
        assert_eq!(e.mem.u32(copy.addr() + 4), 1);
    }

    #[test]
    fn get_sequence_data_keeps_an_equal_result_and_skips_empty_lists() {
        let mut e = Engine::new();
        counting(&mut e);
        e.register(LOG, |_, _| Ret::default());
        let copy = counted(&mut e, 0x2c, 1);
        e.register_double(OBJECT_CREATE_DEEP_COPY, move |e, a| {
            e.mem.set_u32(a[1], copy);
            let count = e.mem.u32(copy + 4);
            e.mem.set_u32(copy + 4, count + 1);
            Ret::default()
        });
        e.register(COPY_SEQUENCE, |_, _| Ret::default());
        let this = e.new_object::<NiTransformData>();
        let out = e.mem.alloc(4);
        e.mem.set_u32(out, copy);
        e.call_log = Some(vec![]);
        e.call(0x00a4_e410, &args![this, 0.25f32, 0.75f32, out]);
        assert!(calls_to(&e, COPY_SEQUENCE).is_empty());
        assert_eq!(e.mem.u32(out), copy);
        assert_eq!(e.mem.u32(copy + 4), 1);
        assert!(calls_to(&e, LOG).is_empty());
    }

    #[test]
    fn save_binary_writes_each_list() {
        let mut e = Engine::new();
        e.map(0x011f_3a00, 0x200);
        for table in [ROT_SAVE, POS_SAVE, SCALE_SAVE] {
            for kind in 4..7 {
                e.mem.set_u32(table + 4 * kind, SAVE);
            }
        }
        for f in [BASE_SAVE_BINARY, SAVE, LOG] {
            e.register(f, |_, _| Ret::default());
        }
        e.register(WRITE, |e, a| {
            assert_eq!((a[2], e.mem.u32(a[3]), a[4]), (4, 4, 1));
            let word = e.mem.u32(a[1]);
            e.call(LOG, &args![word]);
            4u32.into_ret()
        });
        let stream = e.new_object::<NiStream>();
        let output = e.new_object::<NiBinaryStream>();
        e.set(stream, NiStream::m_pkOstr, output.addr());
        e.set(output, NiBinaryStream::m_pfnWrite, WRITE);
        let this = data(&mut e);
        e.set(this, NiTransformData::m_usNumPosKeys, 0);
        e.call_log = Some(vec![]);
        e.call(0x00a4_e810, &args![this, stream]);
        let written: Vec<u32> = calls_to(&e, LOG).iter().map(|c| c[0]).collect();
        assert_eq!(written, vec![1, 4, 0, 3, 6]);
        assert_eq!(
            calls_to(&e, SAVE),
            vec![
                vec![stream.addr(), 0x7000, 1],
                vec![stream.addr(), 0x7200, 3]
            ]
        );
        assert_eq!(e.mem.u32(output.addr() + 4), 20);
    }

    #[test]
    fn get_viewer_strings_adds_the_class_and_counts() {
        let mut e = Engine::new();
        e.map(NI_TRANSFORM_DATA_RTTI, 4);
        e.mem.set_u32(NI_TRANSFORM_DATA_RTTI, 0x0108_0000);
        for f in [OBJECT_GET_VIEWER_STRINGS, STRINGS_GROW, LOG] {
            e.register(f, |_, _| Ret::default());
        }
        e.register(NI_GET_VIEWER_STRING, |_, a| a[0].into_ret());
        e.register(NI_GET_VIEWER_STRING_USHORT, |_, a| (a[0] + a[1]).into_ret());
        e.register(STRINGS_SET_AT, |e, a| {
            let size = e.mem.u16(a[0] + 0xa);
            e.mem.set_u16(a[0] + 0xa, size + 1);
            let text = e.mem.u32(a[2]);
            e.call(LOG, &args![text]);
            Ret::default()
        });
        let this = data(&mut e);
        let strings = e.mem.alloc(0x10);
        e.mem.set_u16(strings + 8, 8);
        e.call_log = Some(vec![]);
        e.call(0x00a4_e990, &args![this, strings]);
        let texts: Vec<u32> = calls_to(&e, LOG).iter().map(|c| c[0]).collect();
        assert_eq!(
            texts,
            vec![
                0x0108_0000,
                NAME_ROT_KEYS + 1,
                NAME_POS_KEYS + 2,
                NAME_SCALE_KEYS + 3
            ]
        );
        assert!(calls_to(&e, STRINGS_GROW).is_empty());
    }
}

//! `NiMain/bsdismemberskininstance.cpp` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `00a86380`: the jump target of `00a6c670`.
const JUMP_TARGET_00A6C670: u32 = 0x00a8_6380;

// Translated from 00a6c670 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSDismemberSkinInstance::LinkObject` (Xbox PDB): a 5-byte jump to `00a86380` (engine map: `NiSkinInstance::LinkObject`), passing on `this` and its one argument. Return type (void): Xbox PDB public name `?LinkObject@BSDismemberSkinInstance@@UAAXAAVNiStream@@@Z`.
pub fn bs_dismember_skin_instance_link_object(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(JUMP_TARGET_00A6C670, &args![this, stream]);
}

/// `00aa13e0`: `operator new` (`cdecl`: size).
const OPERATOR_NEW: u32 = 0x00aa_13e0;
/// `00a5d3a0`: the `NiObject` constructor (engine map; `thiscall`, no stack word).
const OBJECT_CONSTRUCTOR: u32 = 0x00a5_d3a0;
/// The `BSDismemberSkinInstance` virtual table (`01069a84`).
const DISMEMBER_SKIN_INSTANCE_VTABLE: u32 = 0x0106_9a84;
/// The size of a `BSDismemberSkinInstance` (0x40 bytes).
const DISMEMBER_SKIN_INSTANCE_SIZE: u32 = 0x40;
/// `NiSkinInstance`'s member copy (`00a86660`; `thiscall` on the source: clone, cloning
/// process).
const SKIN_INSTANCE_COPY_MEMBERS: u32 = 0x00a8_6660;
/// `NiSkinInstance::LoadBinary` (`00a86340`; `thiscall`: stream).
const SKIN_INSTANCE_LOAD_BINARY: u32 = 0x00a8_6340;
/// The cloning process's map lookup (`00a53c30`; `thiscall` on the map at the cloning
/// process's `+0`: key, address of the result word; answers whether the key was found).
const CLONE_MAP_LOOKUP: u32 = 0x00a5_3c30;
/// `00aa1070`: the `cdecl` allocate (size).
const ALLOCATE: u32 = 0x00aa_1070;
/// The memory manager (`011f6238`) and its allocate (`00aa3e40`; `thiscall`: size).
const MEMORY_MANAGER: u32 = 0x011f_6238;
const MEMORY_MANAGER_ALLOCATE: u32 = 0x00aa_3e40;

layout! {
    /// `BSDismemberSkinInstance` (Xbox PDB), 0x40 bytes: `NiSkinInstance` (0x34) and the
    /// partition array. Offsets confirmed by the PC code of this unit.
    pub struct BSDismemberSkinInstance: 0x40 {
        /// `m_spSkinData` (Xbox PDB, `NiSkinInstance`): `NiSkinData*`; its word at `+0x44`
        /// is the bone count the clone's bone array is sized by.
        0x08 m_spSkinData: u32,
        /// `m_spSkinPartition` (Xbox PDB, `NiSkinInstance`).
        0x0C m_spSkinPartition: u32,
        /// `m_pkRootParent` (Xbox PDB, `NiSkinInstance`).
        0x10 m_pkRootParent: u32,
        /// `m_ppkBones` (Xbox PDB, `NiSkinInstance`).
        0x14 m_ppkBones: u32,
        /// `m_uiFrameID` (Xbox PDB, `NiSkinInstance`).
        0x18 m_uiFrameID: u32,
        /// `uiPartitionDataArraySize` (Xbox PDB).
        0x34 uiPartitionDataArraySize: u32,
        /// `pPartitionDataArray` (Xbox PDB): 4-byte `PartitionData` items (two bytes, then
        /// a 16-bit word).
        0x38 pPartitionDataArray: u32,
        /// `bVisiblePartitions` (Xbox PDB).
        0x3C bVisiblePartitions: u8,
    }
}

/// The bytes of `count` 4-byte items as the code asks the memory manager for them: the
/// `MUL`/`SETO`/`NEG`/`OR` size computation gives all ones when the product overflows.
fn partition_bytes(count: u32) -> u32 {
    count.checked_mul(4).unwrap_or(u32::MAX)
}

/// The fields the `BSDismemberSkinInstance` constructor inlined here sets: after the
/// `NiObject` constructor, the words `+0x08`..`+0x14`, `+0x1C`..`+0x28`, `+0x30`..`+0x38`
/// zero, the frame id `-1` and the class's virtual table (the word at `+0x2C` is not
/// written).
fn construct_in_place(e: &mut Engine, object: u32) {
    e.call(OBJECT_CONSTRUCTOR, &args![object]);
    for offset in [0x08, 0x0c, 0x10, 0x14] {
        e.mem.set_u32(object + offset, 0);
    }
    e.mem.set_u32(object + 0x18, 0xffff_ffff);
    for offset in [0x1c, 0x20, 0x24, 0x28, 0x30] {
        e.mem.set_u32(object + offset, 0);
    }
    e.mem.set_u32(object, DISMEMBER_SKIN_INSTANCE_VTABLE);
    e.mem.set_u32(object + 0x34, 0);
    e.mem.set_u32(object + 0x38, 0);
}

/// The cloning process's lookup of `key` (`00a53c30`) into the word at `result`.
fn clone_lookup(e: &mut Engine, cloning: u32, key: u32, result: Ptr) -> bool {
    let map = e.mem.u32(cloning);
    e.call(CLONE_MAP_LOOKUP, &args![map, key, result]).bool()
}

// Translated from 00a6c720 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSDismemberSkinInstance::CopyMembers` (unnamed in the engine map; `thiscall` on the
/// source, `RET 8`: clone, cloning process). Runs `NiSkinInstance`'s copy (`00a86660`),
/// copies the partition count and, for a count above zero, a new array from the memory
/// manager (`00aa3e40`, count * 4 bytes, all ones on overflow) filled item by item (two
/// bytes, then the 16-bit word; the count read again for each item); with no partitions the
/// clone's array is null. Last the visible-partitions byte.
pub fn bs_dismember_skin_instance_copy_members(
    e: &mut Engine,
    this: Ptr<BSDismemberSkinInstance>,
    clone: Ptr<BSDismemberSkinInstance>,
    cloning: Ptr,
) {
    e.call(SKIN_INSTANCE_COPY_MEMBERS, &args![this, clone, cloning]);
    let count = e.get(this, BSDismemberSkinInstance::uiPartitionDataArraySize);
    e.set(
        clone,
        BSDismemberSkinInstance::uiPartitionDataArraySize,
        count,
    );
    if e.get(this, BSDismemberSkinInstance::uiPartitionDataArraySize) == 0 {
        e.set(clone, BSDismemberSkinInstance::pPartitionDataArray, 0);
    } else {
        let count = e.get(this, BSDismemberSkinInstance::uiPartitionDataArraySize);
        let array = e
            .call(
                MEMORY_MANAGER_ALLOCATE,
                &args![MEMORY_MANAGER, partition_bytes(count)],
            )
            .u32();
        e.set(clone, BSDismemberSkinInstance::pPartitionDataArray, array);
        let mut index = 0u32;
        while index < e.get(this, BSDismemberSkinInstance::uiPartitionDataArraySize) {
            let offset = index.wrapping_mul(4);
            let source = e.get(this, BSDismemberSkinInstance::pPartitionDataArray);
            let value = e.mem.u8(source.wrapping_add(offset));
            let target = e.get(clone, BSDismemberSkinInstance::pPartitionDataArray);
            e.mem.set_u8(target.wrapping_add(offset), value);
            let source = e.get(this, BSDismemberSkinInstance::pPartitionDataArray);
            let value = e.mem.u8(source.wrapping_add(offset).wrapping_add(1));
            let target = e.get(clone, BSDismemberSkinInstance::pPartitionDataArray);
            e.mem
                .set_u8(target.wrapping_add(offset).wrapping_add(1), value);
            let source = e.get(this, BSDismemberSkinInstance::pPartitionDataArray);
            let value = e.mem.u16(source.wrapping_add(offset).wrapping_add(2));
            let target = e.get(clone, BSDismemberSkinInstance::pPartitionDataArray);
            e.mem
                .set_u16(target.wrapping_add(offset).wrapping_add(2), value);
            index = index.wrapping_add(1);
        }
    }
    let visible = e.get(this, BSDismemberSkinInstance::bVisiblePartitions);
    e.set(clone, BSDismemberSkinInstance::bVisiblePartitions, visible);
}

// Translated from 00a6c7c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSDismemberSkinInstance::CreateClone` (Xbox PDB; `thiscall`, `RET 4`: cloning
/// process). Allocates and constructs a 0x40-byte instance (null when the allocation
/// fails), copies the members into it ([`bs_dismember_skin_instance_copy_members`]), then
/// maps the root parent and every bone through the cloning process (`00a53c30`): the clone
/// found, else the original pointer. The bone array (`00aa1070`) holds as many pointers as
/// the skin data's word at `+0x44`; the lookup's result word is not reset between bones.
pub fn bs_dismember_skin_instance_create_clone(
    e: &mut Engine,
    this: Ptr<BSDismemberSkinInstance>,
    cloning: Ptr,
) -> Ptr<BSDismemberSkinInstance> {
    let block = e
        .call(OPERATOR_NEW, &args![DISMEMBER_SKIN_INSTANCE_SIZE])
        .u32();
    if block != 0 {
        construct_in_place(e, block);
    }
    let clone = Ptr::<BSDismemberSkinInstance>::new(block);
    bs_dismember_skin_instance_copy_members(e, this, clone, cloning);
    e.with_stack(4, |e, found| {
        e.mem.set_u32(found.addr(), 0);
        let root = e.get(this, BSDismemberSkinInstance::m_pkRootParent);
        let root = if clone_lookup(e, cloning.addr(), root, found) {
            e.mem.u32(found.addr())
        } else {
            e.get(this, BSDismemberSkinInstance::m_pkRootParent)
        };
        e.set(clone, BSDismemberSkinInstance::m_pkRootParent, root);
        let skin_data = e.get(this, BSDismemberSkinInstance::m_spSkinData);
        let bones = e.mem.u32(skin_data.wrapping_add(0x44));
        let array = e.call(ALLOCATE, &args![bones.wrapping_mul(4)]).u32();
        e.set(clone, BSDismemberSkinInstance::m_ppkBones, array);
        let mut index = 0u32;
        while index < bones {
            let source = e.get(this, BSDismemberSkinInstance::m_ppkBones);
            let bone = e.mem.u32(source.wrapping_add(index.wrapping_mul(4)));
            let bone = if clone_lookup(e, cloning.addr(), bone, found) {
                e.mem.u32(found.addr())
            } else {
                let source = e.get(this, BSDismemberSkinInstance::m_ppkBones);
                e.mem.u32(source.wrapping_add(index.wrapping_mul(4)))
            };
            let target = e.get(clone, BSDismemberSkinInstance::m_ppkBones);
            e.mem
                .set_u32(target.wrapping_add(index.wrapping_mul(4)), bone);
            index = index.wrapping_add(1);
        }
    });
    clone
}

// Translated from 00a6c8a0 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// The class's create function (not a Ghidra function; the bytes at `00a6c8a0`, plain
/// `RET`): allocates 0x40 bytes and, when that succeeds, constructs a
/// `BSDismemberSkinInstance` in them (the `NiObject` constructor `00a5d3a0`, the cleared
/// fields, frame id `-1`, the virtual table `01069a84`; the word at `+0x2C` is not written).
/// Answers the object, null when the allocation failed.
pub fn fn_00a6c8a0(e: &mut Engine) -> Ptr<BSDismemberSkinInstance> {
    let block = e
        .call(OPERATOR_NEW, &args![DISMEMBER_SKIN_INSTANCE_SIZE])
        .u32();
    if block == 0 {
        return Ptr::new(0);
    }
    construct_in_place(e, block);
    Ptr::new(block)
}

// Translated from 00a6c900 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSDismemberSkinInstance::LoadBinary` (Xbox PDB; `thiscall`, `RET 4`: stream). Runs
/// `NiSkinInstance::LoadBinary` (`00a86340`), reads the partition count (one 4-byte word)
/// and the partition items (count * 4 bytes, component sizes 1, 1, 2) through the stream's
/// input object (`NiStream` `+0x24C`; read function at its `+8`, `cdecl` (input,
/// destination, bytes, sizes, size count); the answer added to its word at `+4`), the array
/// from the memory manager (`00aa3e40`, all ones on overflow). The visible-partitions byte
/// becomes the OR of every item's first byte.
pub fn bs_dismember_skin_instance_load_binary(
    e: &mut Engine,
    this: Ptr<BSDismemberSkinInstance>,
    stream: Ptr,
) {
    e.call(SKIN_INSTANCE_LOAD_BINARY, &args![this, stream]);
    let count_field = this.addr() + 0x34;
    let input = e.mem.u32(stream.addr() + 0x24c);
    let read = e.mem.u32(input + 8);
    let answer = e.with_stack(4, |e, size| {
        e.mem.set_u32(size.addr(), 4);
        e.call(read, &args![input, count_field, 4u32, size, 1u32])
            .u32()
    });
    let total = e.mem.u32(input + 4).wrapping_add(answer);
    e.mem.set_u32(input + 4, total);
    let count = e.get(this, BSDismemberSkinInstance::uiPartitionDataArraySize);
    let array = e
        .call(
            MEMORY_MANAGER_ALLOCATE,
            &args![MEMORY_MANAGER, partition_bytes(count)],
        )
        .u32();
    let bytes = e
        .get(this, BSDismemberSkinInstance::uiPartitionDataArraySize)
        .wrapping_mul(4);
    e.set(this, BSDismemberSkinInstance::pPartitionDataArray, array);
    let input = e.mem.u32(stream.addr() + 0x24c);
    let read = e.mem.u32(input + 8);
    let answer = e.with_stack(12, |e, sizes| {
        e.mem.set_u32(sizes.addr(), 1);
        e.mem.set_u32(sizes.addr() + 4, 1);
        e.mem.set_u32(sizes.addr() + 8, 2);
        e.call(read, &args![input, array, bytes, sizes, 3u32]).u32()
    });
    let total = e.mem.u32(input + 4).wrapping_add(answer);
    e.mem.set_u32(input + 4, total);
    e.set(this, BSDismemberSkinInstance::bVisiblePartitions, 0);
    let mut item = e.get(this, BSDismemberSkinInstance::pPartitionDataArray);
    let mut index = 0u32;
    while index < e.get(this, BSDismemberSkinInstance::uiPartitionDataArraySize) {
        let visible = e.get(this, BSDismemberSkinInstance::bVisiblePartitions) | e.mem.u8(item);
        e.set(this, BSDismemberSkinInstance::bVisiblePartitions, visible);
        index = index.wrapping_add(1);
        item = item.wrapping_add(4);
    }
}

use crate::types::NiTArray;

/// `NiSkinInstance::RegisterStreamables` (`00a86460`; `thiscall`: stream).
const SKIN_INSTANCE_REGISTER_STREAMABLES: u32 = 0x00a8_6460;
/// `NiSkinInstance::IsEqual` (`00a864d0`; `thiscall`: other object).
const SKIN_INSTANCE_IS_EQUAL: u32 = 0x00a8_64d0;
/// `NiSkinInstance::SaveBinary` (`00a867a0`; `thiscall`: stream).
const SKIN_INSTANCE_SAVE_BINARY: u32 = 0x00a8_67a0;
/// `NiSkinInstance::GetViewerStrings` (`00a86920`; `thiscall`: string array).
const SKIN_INSTANCE_GET_VIEWER_STRINGS: u32 = 0x00a8_6920;
/// `NiGetViewerString` (`00a7d330`; `cdecl`: text): a copy of the text.
const VIEWER_STRING_NAME: u32 = 0x00a7_d330;
/// `NiGetViewerString_ov2` (`00a7d370`; `cdecl`: label, boolean word).
const VIEWER_STRING_BOOL: u32 = 0x00a7_d370;
/// `NiGetViewerString_ov8` (`00a7d510`; `cdecl`: label, unsigned word).
const VIEWER_STRING_UNSIGNED: u32 = 0x00a7_d510;
/// The string array's `SetSize` (`0096ad30`; `thiscall`: new capacity) and `SetAt`
/// (`0096ae90`; `thiscall`: index, address of the element).
const STRING_ARRAY_SET_SIZE: u32 = 0x0096_ad30;
const STRING_ARRAY_SET_AT: u32 = 0x0096_ae90;
/// `00406d00`: the bounded `sprintf` (`cdecl`: buffer, size, format, ...).
const FORMAT_BOUNDED: u32 = 0x0040_6d00;
/// The word at `011f49d8`: the class name the viewer strings start with (the class's
/// `NiRTTI` name pointer).
const CLASS_NAME: u32 = 0x011f_49d8;
/// `"Num Partitions"`, `"Visible"`.
const TEXT_NUM_PARTITIONS: u32 = 0x0109_c7f8;
const TEXT_VISIBLE: u32 = 0x0109_c7f0;
/// `", State Change"` and the empty string `""` (`01011584`).
const TEXT_STATE_CHANGE: u32 = 0x0109_c7e0;
const TEXT_EMPTY: u32 = 0x0101_1584;
/// `"Enabled"`, `"Disabled"`.
const TEXT_ENABLED: u32 = 0x0109_c7d8;
const TEXT_DISABLED: u32 = 0x0109_c7cc;
/// `"%d = BodyPart %d [%s%s]"`.
const TEXT_PARTITION_FORMAT: u32 = 0x0109_c7b4;
/// `NiStream::m_pkOstr` (the output object, `+0x250`; its write function at `+0xc`, `cdecl`:
/// output, source, bytes, component sizes, component count; the answer is added to its word
/// at `+4`) and the stream's byte at `+0x1e4` (1: the items are written as they are, else
/// swapped first and swapped back after).
const STREAM_OUTPUT: u32 = 0x250;
const STREAM_NO_SWAP: u32 = 0x1e4;

// Translated from 00a6c680 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSDismemberSkinInstance::RegisterStreamables` (Xbox PDB; `thiscall`, `RET 4`: stream):
/// `NiSkinInstance::RegisterStreamables` (`00a86460`), its answer as a boolean.
pub fn bs_dismember_skin_instance_register_streamables(
    e: &mut Engine,
    this: Ptr<BSDismemberSkinInstance>,
    stream: Ptr,
) -> bool {
    e.call(SKIN_INSTANCE_REGISTER_STREAMABLES, &args![this, stream])
        .u8()
        != 0
}

// Translated from 00a6c6a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSDismemberSkinInstance::IsEqual` (Xbox PDB; `thiscall`, `RET 4`: other object): false
/// when `NiSkinInstance::IsEqual` (`00a864d0`) is, when the partition counts differ, or at
/// the first partition item whose first byte, second byte or 16-bit word differs.
pub fn bs_dismember_skin_instance_is_equal(
    e: &mut Engine,
    this: Ptr<BSDismemberSkinInstance>,
    other: Ptr<BSDismemberSkinInstance>,
) -> bool {
    if e.call(SKIN_INSTANCE_IS_EQUAL, &args![this, other]).u8() == 0 {
        return false;
    }
    let count = e.get(this, BSDismemberSkinInstance::uiPartitionDataArraySize);
    if count != e.get(other, BSDismemberSkinInstance::uiPartitionDataArraySize) {
        return false;
    }
    let mut index = 0u32;
    if count != 0 {
        let mut mine = e.get(this, BSDismemberSkinInstance::pPartitionDataArray);
        let mut theirs = e.get(other, BSDismemberSkinInstance::pPartitionDataArray);
        loop {
            if e.mem.u8(mine) != e.mem.u8(theirs)
                || e.mem.u8(mine.wrapping_add(1)) != e.mem.u8(theirs.wrapping_add(1))
                || e.mem.u16(mine.wrapping_add(2)) != e.mem.u16(theirs.wrapping_add(2))
            {
                return false;
            }
            index = index.wrapping_add(1);
            theirs = theirs.wrapping_add(4);
            mine = mine.wrapping_add(4);
            if index >= count {
                break;
            }
        }
    }
    true
}

/// One write through the stream's output object (`NiStream` `+0x250`, write function at
/// `+0xc`) with one component of `size_word`'s size, the answer added to the output's word
/// at `+4`.
fn stream_write(e: &mut Engine, stream: Ptr, source: u32, bytes: u32, size_word: u32) {
    let output = e.mem.u32(stream.addr().wrapping_add(STREAM_OUTPUT));
    let write = e.mem.u32(output.wrapping_add(0xc));
    let answer = e
        .call(write, &args![output, source, bytes, size_word, 1u32])
        .u32();
    let total = e.mem.u32(output.wrapping_add(4)).wrapping_add(answer);
    e.mem.set_u32(output.wrapping_add(4), total);
}

/// Swaps the two bytes of the 16-bit word at `address`.
fn swap_word(e: &mut Engine, address: u32) {
    let word = e.mem.u16(address);
    e.mem.set_u16(address, word.swap_bytes());
}

/// `BSWAP` of the word at `address`.
fn swap_dword(e: &mut Engine, address: u32) {
    let word = e.mem.u32(address);
    e.mem.set_u32(address, word.swap_bytes());
}

// Translated from 00a6c9c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSDismemberSkinInstance::SaveBinary` (Xbox PDB; `thiscall`, `RET 4`: stream). After
/// `NiSkinInstance::SaveBinary` (`00a867a0`), writes the partition count (4 bytes, one
/// 4-byte component) and the items (count * 4 bytes, one 4-byte component). Unless the
/// stream's byte at `+0x1e4` is 1, each item is swapped before the write (its 16-bit word at
/// `+2`, then the whole item with `BSWAP`) and swapped back after it (`BSWAP`, then the word
/// at `+2`). The component-size word of each write is a stack slot set to 4 (the first in
/// the function's local word, the second in its stream argument word).
pub fn bs_dismember_skin_instance_save_binary(
    e: &mut Engine,
    this: Ptr<BSDismemberSkinInstance>,
    stream: Ptr,
) {
    e.call(SKIN_INSTANCE_SAVE_BINARY, &args![this, stream]);
    let count_field = this.addr().wrapping_add(0x34);
    e.with_stack(8, |e, frame| {
        let local = frame.addr();
        let argument = frame.addr() + 4;
        e.mem.set_u32(argument, stream.addr());
        e.mem.set_u32(local, 4);
        stream_write(e, stream, count_field, 4, local);
        if e.mem.u8(stream.addr().wrapping_add(STREAM_NO_SWAP)) != 1 {
            let mut index = 0u32;
            while index < e.mem.u32(count_field) {
                let offset = index.wrapping_mul(4);
                let array = e.get(this, BSDismemberSkinInstance::pPartitionDataArray);
                swap_word(e, array.wrapping_add(offset).wrapping_add(2));
                index = index.wrapping_add(1);
                let array = e.get(this, BSDismemberSkinInstance::pPartitionDataArray);
                swap_dword(e, array.wrapping_add(offset));
            }
        }
        let bytes = e.mem.u32(count_field).wrapping_mul(4);
        let array = e.get(this, BSDismemberSkinInstance::pPartitionDataArray);
        e.mem.set_u32(argument, 4);
        stream_write(e, stream, array, bytes, argument);
        if e.mem.u8(stream.addr().wrapping_add(STREAM_NO_SWAP)) != 1 {
            let mut index = 0u32;
            while index < e.mem.u32(count_field) {
                let offset = index.wrapping_mul(4);
                let array = e.get(this, BSDismemberSkinInstance::pPartitionDataArray);
                swap_dword(e, array.wrapping_add(offset));
                let array = e.get(this, BSDismemberSkinInstance::pPartitionDataArray);
                index = index.wrapping_add(1);
                swap_word(e, array.wrapping_add(offset).wrapping_add(2));
            }
        }
    });
}

/// Adds the string word at `slot` at the array's end: `SetSize` (`0096ad30`) to the size
/// plus the grow-by step when the array is full, then `SetAt` (`0096ae90`) at the old size.
fn add_viewer_string(e: &mut Engine, strings: Ptr<NiTArray>, slot: u32) {
    let index = e.get(strings, NiTArray::m_usSize) as u32;
    if index >= e.get(strings, NiTArray::m_usMaxSize) as u32 {
        let grow = e.get(strings, NiTArray::m_usGrowBy) as u32;
        e.call(
            STRING_ARRAY_SET_SIZE,
            &args![strings, grow.wrapping_add(index)],
        );
    }
    e.call(STRING_ARRAY_SET_AT, &args![strings, index, slot]);
}

// Translated from 00a6cac0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00a6cac0 - the /GS stack cookie (011c16bc, __security_check_cookie 00ec408c) is not translated
/// `BSDismemberSkinInstance::GetViewerStrings` (Xbox PDB; `thiscall`, `RET 4`: string
/// array). After `NiSkinInstance::GetViewerStrings` (`00a86920`), adds the class name (the
/// word at `011f49d8`), `"Num Partitions"` with the count and `"Visible"` with the
/// visible-partitions byte (each through `SetSize`/`SetAt`), then for each partition
/// `"%d = BodyPart %d [%s%s]"` formatted into a 64-byte buffer (`00406d00`: the index, the
/// item's 16-bit word, `"Enabled"` or `"Disabled"` by its first byte, `", State Change"` or
/// `""` by its second) and copied (`00a7d330`), stored with `SetAt` inlined (grow, then the
/// size and the non-null count kept as `SetAt` keeps them).
pub fn bs_dismember_skin_instance_get_viewer_strings(
    e: &mut Engine,
    this: Ptr<BSDismemberSkinInstance>,
    strings: Ptr<NiTArray>,
) {
    // The frame: the string word (+0) and the 64-byte format buffer (+4).
    e.with_stack(0x44, |e, frame| {
        let slot = frame.addr();
        let buffer = frame.addr() + 4;
        e.call(SKIN_INSTANCE_GET_VIEWER_STRINGS, &args![this, strings]);
        let class_name = e.global::<u32>(CLASS_NAME);
        let text = e.call(VIEWER_STRING_NAME, &args![class_name]).u32();
        e.mem.set_u32(slot, text);
        add_viewer_string(e, strings, slot);
        let count = e.get(this, BSDismemberSkinInstance::uiPartitionDataArraySize);
        let text = e
            .call(VIEWER_STRING_UNSIGNED, &args![TEXT_NUM_PARTITIONS, count])
            .u32();
        e.mem.set_u32(slot, text);
        add_viewer_string(e, strings, slot);
        let visible = e.get(this, BSDismemberSkinInstance::bVisiblePartitions) as u32;
        let text = e
            .call(VIEWER_STRING_BOOL, &args![TEXT_VISIBLE, visible])
            .u32();
        e.mem.set_u32(slot, text);
        add_viewer_string(e, strings, slot);
        let mut index = 0u32;
        while index < e.get(this, BSDismemberSkinInstance::uiPartitionDataArraySize) {
            let array = e.get(this, BSDismemberSkinInstance::pPartitionDataArray);
            let item = array.wrapping_add(index.wrapping_mul(4));
            let change = if e.mem.u8(item.wrapping_add(1)) != 0 {
                TEXT_STATE_CHANGE
            } else {
                TEXT_EMPTY
            };
            let state = if e.mem.u8(item) != 0 {
                TEXT_ENABLED
            } else {
                TEXT_DISABLED
            };
            let body_part = e.mem.u16(item.wrapping_add(2)) as u32;
            e.call(
                FORMAT_BOUNDED,
                &args![
                    buffer,
                    0x40u32,
                    TEXT_PARTITION_FORMAT,
                    index,
                    body_part,
                    state,
                    change
                ],
            );
            let text = e.call(VIEWER_STRING_NAME, &args![buffer]).u32();
            let at = e.get(strings, NiTArray::m_usSize);
            if at >= e.get(strings, NiTArray::m_usMaxSize) {
                let grow = e.get(strings, NiTArray::m_usGrowBy) as u32;
                e.call(
                    STRING_ARRAY_SET_SIZE,
                    &args![strings, grow.wrapping_add(at as u32)],
                );
            }
            let base = e.get(strings, NiTArray::m_pBase);
            let element = base.wrapping_add((at as u32).wrapping_mul(4));
            if at >= e.get(strings, NiTArray::m_usSize) {
                e.set(strings, NiTArray::m_usSize, at.wrapping_add(1));
                if text != 0 {
                    let used = e.get(strings, NiTArray::m_usESize).wrapping_add(1);
                    e.set(strings, NiTArray::m_usESize, used);
                }
            } else if text != 0 {
                if e.mem.u32(element) == 0 {
                    let used = e.get(strings, NiTArray::m_usESize).wrapping_add(1);
                    e.set(strings, NiTArray::m_usESize, used);
                }
            } else if e.mem.u32(element) != 0 {
                let used = e.get(strings, NiTArray::m_usESize).wrapping_add(0xffff);
                e.set(strings, NiTArray::m_usESize, used);
            }
            let base = e.get(strings, NiTArray::m_pBase);
            index = index.wrapping_add(1);
            e.mem
                .set_u32(base.wrapping_add((at as u32).wrapping_mul(4)), text);
        }
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00a6_c680,
            bs_dismember_skin_instance_register_streamables(
                Ptr<BSDismemberSkinInstance>,
                Ptr,
            ) -> bool
        ),
        entry!(
            0x00a6_c6a0,
            bs_dismember_skin_instance_is_equal(
                Ptr<BSDismemberSkinInstance>,
                Ptr<BSDismemberSkinInstance>,
            ) -> bool
        ),
        entry!(
            0x00a6_c9c0,
            bs_dismember_skin_instance_save_binary(Ptr<BSDismemberSkinInstance>, Ptr)
        ),
        entry!(
            0x00a6_cac0,
            bs_dismember_skin_instance_get_viewer_strings(
                Ptr<BSDismemberSkinInstance>,
                Ptr<NiTArray>,
            )
        ),
        entry!(
            0x00a6_c670,
            bs_dismember_skin_instance_link_object(Ptr, Ptr)
        ),
        entry!(
            0x00a6_c720,
            bs_dismember_skin_instance_copy_members(
                Ptr<BSDismemberSkinInstance>,
                Ptr<BSDismemberSkinInstance>,
                Ptr,
            )
        ),
        entry!(
            0x00a6_c7c0,
            bs_dismember_skin_instance_create_clone(
                Ptr<BSDismemberSkinInstance>,
                Ptr,
            ) -> Ptr<BSDismemberSkinInstance>
        ),
        entry!(0x00a6_c8a0, fn_00a6c8a0() -> Ptr<BSDismemberSkinInstance>),
        entry!(
            0x00a6_c900,
            bs_dismember_skin_instance_load_binary(Ptr<BSDismemberSkinInstance>, Ptr)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn thunk_00a6c670_forwards_its_words_to_the_jump_target() {
        const THUNK: u32 = 0x00a6_c670;
        const TARGET: u32 = 0x00a8_6380;
        let mut e = Engine::new();
        e.register(TARGET, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(THUNK, &args![0x1000u32, 0x2000u32]);
        let log = e.call_log.take().unwrap();
        let hits: Vec<_> = log.iter().filter(|call| call.0 == TARGET).collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, args![0x1000u32, 0x2000u32]);
    }
}

#[cfg(test)]
mod b037_tests {
    use super::*;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.register(SKIN_INSTANCE_COPY_MEMBERS, |_, _| Ret::default());
        e.register(SKIN_INSTANCE_LOAD_BINARY, |_, _| Ret::default());
        e.register(OBJECT_CONSTRUCTOR, |_, _| Ret::default());
        e.register(MEMORY_MANAGER_ALLOCATE, |e, a| e.mem.alloc(a[1]).into_ret());
        e.register(ALLOCATE, |e, a| e.mem.alloc(a[0].max(4)).into_ret());
        e.call_log = Some(vec![]);
        e
    }

    #[test]
    fn copy_members_copies_every_partition_item() {
        let mut e = engine();
        let source = e.mem.alloc(0x40);
        let target = e.mem.alloc(0x40);
        let items = e.mem.alloc(8);
        e.mem.set_u32(items, 0x1234_0201);
        e.mem.set_u32(items + 4, 0x5678_0403);
        e.mem.set_u32(source + 0x34, 2);
        e.mem.set_u32(source + 0x38, items);
        e.mem.set_u8(source + 0x3c, 1);
        e.call(0x00a6_c720, &args![source, target, 0x9000u32]);
        assert_eq!(
            calls(&e, SKIN_INSTANCE_COPY_MEMBERS),
            vec![vec![source, target, 0x9000]]
        );
        assert_eq!(
            calls(&e, MEMORY_MANAGER_ALLOCATE),
            vec![vec![MEMORY_MANAGER, 8]]
        );
        let copy = e.mem.u32(target + 0x38);
        assert_ne!(copy, items);
        assert_eq!(e.mem.u32(copy), 0x1234_0201);
        assert_eq!(e.mem.u32(copy + 4), 0x5678_0403);
        assert_eq!(e.mem.u32(target + 0x34), 2);
        assert_eq!(e.mem.u8(target + 0x3c), 1);
    }

    #[test]
    fn copy_members_without_partitions_clears_the_array() {
        let mut e = engine();
        let source = e.mem.alloc(0x40);
        let target = e.mem.alloc(0x40);
        e.mem.set_u32(target + 0x38, 0x4444);
        e.call(0x00a6_c720, &args![source, target, 0x9000u32]);
        assert_eq!(e.mem.u32(target + 0x38), 0);
        assert!(calls(&e, MEMORY_MANAGER_ALLOCATE).is_empty());
        assert_eq!(partition_bytes(0x4000_0000), u32::MAX);
    }

    #[test]
    fn create_clone_maps_the_root_and_the_bones() {
        let mut e = engine();
        let block = e.mem.alloc(0x40);
        e.mem.set_u32(block + 0x2c, 0x2c2c);
        e.register_double(OPERATOR_NEW, move |_, _| block.into_ret());
        // The lookup finds the root (0x100 -> 0x111) and bone 0x200, not bone 0x300.
        e.register(CLONE_MAP_LOOKUP, |e, a| {
            let found = match a[1] {
                0x100 => 0x111,
                0x200 => 0x222,
                _ => return Ret::default(),
            };
            e.mem.set_u32(a[2], found);
            1u32.into_ret()
        });
        let source = e.mem.alloc(0x40);
        let skin_data = e.mem.alloc(0x48);
        e.mem.set_u32(skin_data + 0x44, 2);
        let bones = e.mem.alloc(8);
        e.mem.set_u32(bones, 0x200);
        e.mem.set_u32(bones + 4, 0x300);
        e.mem.set_u32(source + 0x08, skin_data);
        e.mem.set_u32(source + 0x10, 0x100);
        e.mem.set_u32(source + 0x14, bones);
        let cloning = e.mem.alloc(4);
        e.mem.set_u32(cloning, 0x7777);
        let clone = e.call(0x00a6_c7c0, &args![source, cloning]).u32();
        assert_eq!(clone, block);
        assert_eq!(e.mem.u32(block), DISMEMBER_SKIN_INSTANCE_VTABLE);
        assert_eq!(e.mem.u32(block + 0x18), 0xffff_ffff);
        assert_eq!(e.mem.u32(block + 0x2c), 0x2c2c);
        assert_eq!(e.mem.u32(block + 0x10), 0x111);
        let cloned = e.mem.u32(block + 0x14);
        assert_eq!(e.mem.u32(cloned), 0x222);
        assert_eq!(e.mem.u32(cloned + 4), 0x300);
        assert_eq!(calls(&e, ALLOCATE), vec![vec![8]]);
        let lookups = calls(&e, CLONE_MAP_LOOKUP);
        assert_eq!(lookups.len(), 3);
        assert!(lookups.iter().all(|c| c[0] == 0x7777));
        assert_eq!(
            calls(&e, SKIN_INSTANCE_COPY_MEMBERS),
            vec![vec![source, block, cloning]]
        );
    }

    #[test]
    fn create_function_constructs_or_answers_null() {
        let mut e = engine();
        let block = e.mem.alloc(0x40);
        e.mem.set_u32(block + 0x2c, 0x2c2c);
        e.mem.set_u32(block + 0x08, 0x0808);
        e.register_double(OPERATOR_NEW, move |_, a| {
            assert_eq!(a[0], 0x40);
            block.into_ret()
        });
        assert_eq!(e.call(0x00a6_c8a0, &args![]).u32(), block);
        assert_eq!(calls(&e, OBJECT_CONSTRUCTOR), vec![vec![block]]);
        assert_eq!(e.mem.u32(block), DISMEMBER_SKIN_INSTANCE_VTABLE);
        assert_eq!(e.mem.u32(block + 0x08), 0);
        assert_eq!(e.mem.u32(block + 0x18), 0xffff_ffff);
        assert_eq!(e.mem.u32(block + 0x2c), 0x2c2c);
        e.register(OPERATOR_NEW, |_, _| Ret::default());
        assert_eq!(e.call(0x00a6_c8a0, &args![]).u32(), 0);
    }

    #[test]
    fn load_binary_reads_the_partitions_and_ors_the_visible_bytes() {
        let mut e = engine();
        const READ: u32 = 0x0300_0000;
        e.register(READ, |e, a| {
            if a[4] == 1 {
                assert_eq!(e.mem.u32(a[3]), 4);
                e.mem.set_u32(a[1], 3);
            } else {
                assert_eq!(
                    (e.mem.u32(a[3]), e.mem.u32(a[3] + 4), e.mem.u32(a[3] + 8)),
                    (1, 1, 2)
                );
                for (index, first) in [2u8, 0, 4].iter().enumerate() {
                    e.mem.set_u8(a[1] + 4 * index as u32, *first);
                }
            }
            a[2].into_ret()
        });
        let input = e.mem.alloc(0xc);
        e.mem.set_u32(input + 4, 0x10);
        e.mem.set_u32(input + 8, READ);
        let stream = e.mem.alloc(0x250);
        e.mem.set_u32(stream + 0x24c, input);
        let this = e.mem.alloc(0x40);
        e.call(0x00a6_c900, &args![this, stream]);
        assert_eq!(
            calls(&e, SKIN_INSTANCE_LOAD_BINARY),
            vec![vec![this, stream]]
        );
        assert_eq!(e.mem.u32(this + 0x34), 3);
        assert_eq!(
            calls(&e, MEMORY_MANAGER_ALLOCATE),
            vec![vec![MEMORY_MANAGER, 12]]
        );
        assert_eq!(e.mem.u8(this + 0x3c), 6);
        assert_eq!(e.mem.u32(input + 4), 0x10 + 4 + 12);
        let reads = calls(&e, READ);
        assert_eq!(
            (reads[0][1], reads[0][2], reads[1][1], reads[1][2]),
            (this + 0x34, 4, e.mem.u32(this + 0x38), 12)
        );
    }
}

#[cfg(test)]
mod n014_tests {
    use super::*;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    #[test]
    fn register_streamables_answers_the_base_byte() {
        let mut e = Engine::new();
        e.register(SKIN_INSTANCE_REGISTER_STREAMABLES, |_, a| {
            // Only the low byte counts.
            (if a[1] == 0x2000 { 0x100u32 } else { 0x101 }).into_ret()
        });
        e.call_log = Some(vec![]);
        assert!(!e.call(0x00a6_c680, &args![0x1000u32, 0x2000u32]).bool());
        assert!(e.call(0x00a6_c680, &args![0x1000u32, 0x3000u32]).bool());
        assert_eq!(
            calls(&e, SKIN_INSTANCE_REGISTER_STREAMABLES)[0],
            vec![0x1000, 0x2000]
        );
    }

    #[test]
    fn is_equal_compares_the_base_count_and_every_item() {
        const BASE_ANSWER: u32 = 0x0300_0000;
        let mut e = Engine::new();
        e.register(SKIN_INSTANCE_IS_EQUAL, |e, _| {
            e.mem.u32(BASE_ANSWER).into_ret()
        });
        e.mem.map(BASE_ANSWER, 4);
        e.mem.set_u32(BASE_ANSWER, 1);
        let a = e.mem.alloc(0x40);
        let b = e.mem.alloc(0x40);
        let items_a = e.mem.alloc(8);
        let items_b = e.mem.alloc(8);
        for items in [items_a, items_b] {
            e.mem.set_u32(items, 0x1234_0201);
            e.mem.set_u32(items + 4, 0x5678_0403);
        }
        for (this, items) in [(a, items_a), (b, items_b)] {
            e.mem.set_u32(this + 0x34, 2);
            e.mem.set_u32(this + 0x38, items);
        }
        assert!(e.call(0x00a6_c6a0, &args![a, b]).bool());
        // The first byte, the second byte and the word of the second item.
        for value in [0x5678_0400u32, 0x5678_0003, 0x5600_0403] {
            e.mem.set_u32(items_b + 4, value);
            assert!(!e.call(0x00a6_c6a0, &args![a, b]).bool());
        }
        e.mem.set_u32(items_b + 4, 0x5678_0403);
        e.mem.set_u32(b + 0x34, 1);
        assert!(!e.call(0x00a6_c6a0, &args![a, b]).bool());
        // No partitions on either side: equal without reading the arrays.
        e.mem.set_u32(a + 0x34, 0);
        e.mem.set_u32(b + 0x34, 0);
        e.mem.set_u32(a + 0x38, 0);
        assert!(e.call(0x00a6_c6a0, &args![a, b]).bool());
        e.mem.set_u32(BASE_ANSWER, 0x100);
        assert!(!e.call(0x00a6_c6a0, &args![a, b]).bool());
    }

    #[test]
    fn save_binary_swaps_the_items_around_the_write_unless_told_not_to() {
        const WRITE: u32 = 0x0310_0000;
        const WRITTEN: u32 = 0x0320_0000;
        for no_swap in [0u8, 1] {
            let mut e = Engine::new();
            e.register(SKIN_INSTANCE_SAVE_BINARY, |_, _| Ret::default());
            e.register(WRITE, |e, a| {
                assert_eq!((e.mem.u32(a[3]), a[4]), (4, 1));
                if a[2] == 8 {
                    // The items as written.
                    let first = e.mem.u32(a[1]);
                    let second = e.mem.u32(a[1] + 4);
                    e.mem.set_u32(WRITTEN, first);
                    e.mem.set_u32(WRITTEN + 4, second);
                }
                a[2].into_ret()
            });
            e.mem.map(WRITTEN, 8);
            let output = e.mem.alloc(0x10);
            e.mem.set_u32(output + 4, 0x10);
            e.mem.set_u32(output + 0xc, WRITE);
            let stream = e.mem.alloc(0x260);
            e.mem.set_u32(stream + 0x250, output);
            e.mem.set_u8(stream + 0x1e4, no_swap);
            let this = e.mem.alloc(0x40);
            let items = e.mem.alloc(8);
            e.mem.set_u32(items, 0x4433_2211);
            e.mem.set_u32(items + 4, 0x8877_6655);
            e.mem.set_u32(this + 0x34, 2);
            e.mem.set_u32(this + 0x38, items);
            e.call_log = Some(vec![]);
            e.call(0x00a6_c9c0, &args![this, stream]);
            assert_eq!(
                calls(&e, SKIN_INSTANCE_SAVE_BINARY),
                vec![vec![this, stream]]
            );
            let writes = calls(&e, WRITE);
            assert_eq!(writes.len(), 2);
            assert_eq!((writes[0][1], writes[0][2]), (this + 0x34, 4));
            assert_eq!((writes[1][1], writes[1][2]), (items, 8));
            assert_eq!(e.mem.u32(output + 4), 0x10 + 4 + 8);
            let written = (e.mem.u32(WRITTEN), e.mem.u32(WRITTEN + 4));
            if no_swap == 1 {
                assert_eq!(written, (0x4433_2211, 0x8877_6655));
            } else {
                // The word at +2 swapped (11 22 44 33), then the whole item reversed.
                assert_eq!(written, (0x1122_4433, 0x5566_8877));
            }
            assert_eq!(e.mem.u32(items), 0x4433_2211);
            assert_eq!(e.mem.u32(items + 4), 0x8877_6655);
        }
    }

    #[test]
    fn get_viewer_strings_lists_the_count_the_flag_and_each_partition() {
        let mut e = Engine::new();
        e.mem.map(CLASS_NAME, 4);
        e.mem.set_u32(CLASS_NAME, 0x0109_0000);
        e.register(SKIN_INSTANCE_GET_VIEWER_STRINGS, |_, _| Ret::default());
        e.register(VIEWER_STRING_NAME, |_, a| a[0].wrapping_add(1).into_ret());
        e.register(VIEWER_STRING_UNSIGNED, |_, a| (a[1] + 0x100).into_ret());
        e.register(VIEWER_STRING_BOOL, |_, a| (a[1] + 0x200).into_ret());
        e.register(FORMAT_BOUNDED, |_, _| Ret::default());
        e.register(STRING_ARRAY_SET_SIZE, |e, a| {
            let base = e.mem.alloc(4 * a[1]);
            let old = e.mem.u32(a[0] + 4);
            let size = e.mem.u16(a[0] + 0xa) as u32;
            for index in 0..size {
                let value = e.mem.u32(old + 4 * index);
                e.mem.set_u32(base + 4 * index, value);
            }
            e.mem.set_u32(a[0] + 4, base);
            e.mem.set_u16(a[0] + 8, a[1] as u16);
            Ret::default()
        });
        e.register(STRING_ARRAY_SET_AT, |e, a| {
            let base = e.mem.u32(a[0] + 4);
            let value = e.mem.u32(a[2]);
            e.mem.set_u32(base + 4 * a[1], value);
            e.mem.set_u16(a[0] + 0xa, a[1] as u16 + 1);
            let used = e.mem.u16(a[0] + 0xc) + 1;
            e.mem.set_u16(a[0] + 0xc, used);
            Ret::default()
        });
        let strings = e.mem.alloc(0x10);
        e.mem.set_u16(strings + 0xe, 2);
        let this = e.mem.alloc(0x40);
        let items = e.mem.alloc(8);
        e.mem.set_u32(items, 0x0007_0001);
        e.mem.set_u32(items + 4, 0x0009_0100);
        e.mem.set_u32(this + 0x34, 2);
        e.mem.set_u32(this + 0x38, items);
        e.mem.set_u8(this + 0x3c, 1);
        e.call_log = Some(vec![]);
        e.call(0x00a6_cac0, &args![this, strings]);
        assert_eq!(
            calls(&e, SKIN_INSTANCE_GET_VIEWER_STRINGS),
            vec![vec![this, strings]]
        );
        let formats = calls(&e, FORMAT_BOUNDED);
        assert_eq!(formats.len(), 2);
        assert_eq!(
            formats[0][1..],
            [0x40, TEXT_PARTITION_FORMAT, 0, 7, TEXT_ENABLED, TEXT_EMPTY]
        );
        assert_eq!(
            formats[1][1..],
            [
                0x40,
                TEXT_PARTITION_FORMAT,
                1,
                9,
                TEXT_DISABLED,
                TEXT_STATE_CHANGE
            ]
        );
        assert_eq!(
            calls(&e, VIEWER_STRING_UNSIGNED),
            vec![vec![TEXT_NUM_PARTITIONS, 2]]
        );
        assert_eq!(calls(&e, VIEWER_STRING_BOOL), vec![vec![TEXT_VISIBLE, 1]]);
        // Five strings: the class name, the count, the flag and the two partitions.
        assert_eq!(e.mem.u16(strings + 0xa), 5);
        assert_eq!(e.mem.u16(strings + 0xc), 5);
        let base = e.mem.u32(strings + 4);
        assert_eq!(e.mem.u32(base), 0x0109_0001);
        assert_eq!(e.mem.u32(base + 4), 0x102);
        assert_eq!(e.mem.u32(base + 8), 0x201);
        assert_eq!(e.mem.u32(base + 12), formats[0][0] + 1);
        assert_eq!(e.mem.u32(base + 16), formats[1][0] + 1);
    }
}

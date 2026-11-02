//! `NiAnimation/nimorphdata.cpp` (Xbox PDB source unit), subsystem `NiAnimation`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiMorphData::ms_RTTI` (the class's `NiRTTI` record).
const NI_MORPH_DATA_RTTI: u32 = 0x011f_3e74;

// Translated from 00a415e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiMorphData::GetRTTI` (Xbox PDB): the address of the class's `NiRTTI` record.
pub fn ni_morph_data_get_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(NI_MORPH_DATA_RTTI)
}

/// The vtable of `NiMorphData` (the deleting destructor stores it).
const NI_MORPH_DATA_VTABLE: u32 = 0x0109_76fc;
/// `NiMorphData` size (Xbox PDB and the sized delete).
const NI_MORPH_DATA_SIZE: u32 = 0x18;
/// `NiMorphData` fields (Xbox PDB names; PC offsets as the code uses them):
/// `m_uiNumTargets` +0x08, `m_uiNumVertsPerTarget` +0x0c, `m_aMorphTargets`
/// +0x10 and `m_bRelativeTargets` +0x14.
const NUM_TARGETS: u32 = 0x08;
const NUM_VERTS_PER_TARGET: u32 = 0x0c;
const MORPH_TARGETS: u32 = 0x10;
const RELATIVE_TARGETS: u32 = 0x14;
/// `NiMorphData::MorphTarget` (16 bytes; offsets as the code uses them):
/// the vertex array +0x00, the name (an `NiFixedString`) +0x04, the weight
/// float +0x08 and the interpolator pointer +0x0c.
const MORPH_TARGET_SIZE: u32 = 0x10;
const TARGET_VERTICES: u32 = 0x00;
const TARGET_NAME: u32 = 0x04;
const TARGET_WEIGHT: u32 = 0x08;
const TARGET_INTERPOLATOR: u32 = 0x0c;

/// The fixed-string handle of the empty string (never counted).
const EMPTY_FIXED_STRING: u32 = 0x0109_b220;
/// The memory manager singleton and its `Allocate` (`00aa3e40`) and
/// `Deallocate` (`00aa4060`), `thiscall`, one word.
const MEMORY_MANAGER: u32 = 0x011f_6238;
const MEMORY_MANAGER_ALLOCATE: u32 = 0x00aa_3e40;
const MEMORY_MANAGER_DEALLOCATE: u32 = 0x00aa_4060;
/// `00a41610` (unit `nimorphdata.cpp`): the `MorphTarget` destructor.
const MORPH_TARGET_DESTRUCT: u32 = 0x00a4_1610;
/// The `MorphTarget` constructor (`00a415f0`, in this file), the address
/// `LoadBinary` hands the vector constructor iterator.
const MORPH_TARGET_CONSTRUCT: u32 = 0x00a4_15f0;
/// `00401050` (`stdcall`, `(array, element size, count, constructor)`):
/// the compiler's vector constructor iterator.
const VECTOR_CONSTRUCTOR_ITERATOR: u32 = 0x0040_1050;
/// `NiObject::LoadBinary` (`00a5d610`) and `NiObject`'s destructor body
/// (`00a5d3d0`).
const NI_OBJECT_LOAD_BINARY: u32 = 0x00a5_d610;
const NI_OBJECT_DESTRUCT: u32 = 0x00a5_d3d0;
/// `00aa1460` (`cdecl`, `(block, size)`): the sized delete.
const NI_SIZED_DELETE: u32 = 0x00aa_1460;
/// `NiStream::LoadCStringAsFixedString` (`00a64ea0`) and
/// `NiStream::LoadFixedString` (`00a64df0`), `thiscall` on the stream with
/// the string's address.
const LOAD_CSTRING_AS_FIXED_STRING: u32 = 0x00a6_4ea0;
const LOAD_FIXED_STRING: u32 = 0x00a6_4df0;
/// The float-key tables `LoadBinary` uses for old files (indexed by key
/// type, as in `nifloatdata.cpp`): key size bytes `011f3758`, load
/// functions `011f3db0` (`cdecl` `(stream, count)`), after-load functions
/// `011f3780` (`cdecl` `(keys, count, key size)`).
const FLOAT_KEY_SIZE_TABLE: u32 = 0x011f_3758;
const FLOAT_KEY_LOAD_TABLE: u32 = 0x011f_3db0;
const FLOAT_KEY_AFTER_LOAD_TABLE: u32 = 0x011f_3780;
/// `00aa13e0` (`cdecl`, one word): allocates from the Ni allocator.
const NI_ALLOCATE: u32 = 0x00aa_13e0;
/// `00a3c340` (unnamed in the engine map; `thiscall`, one word): the
/// `NiFloatInterpolator` constructor taking float data.
const FLOAT_INTERPOLATOR_CONSTRUCT: u32 = 0x00a3_c340;
/// `NiFloatInterpolator::ReplaceKeys` (`0052ce40`, `thiscall`, `(keys,
/// count, key type)`).
const FLOAT_INTERPOLATOR_REPLACE_KEYS: u32 = 0x0052_ce40;
/// `NiSprintf` (`00a305b0`, `cdecl`) and its format `"MT %d"`.
const NI_SPRINTF: u32 = 0x00a3_05b0;
const TARGET_NAME_FORMAT: u32 = 0x0109_7788;
/// `00a5b690` (`cdecl`, one word): the fixed-string handle for a C string.
const GET_FIXED_STRING: u32 = 0x00a5_b690;
/// `NiColor::LoadBinary` (`00a59be0`, `thiscall` on the element).
const COLOR_LOAD_BINARY: u32 = 0x00a5_9be0;
/// The interpolator's virtual slot `0xbc` (no arguments) run on the one
/// made for an old file's keys.
const SLOT_INTERPOLATOR_COLLAPSE: u32 = 0xbc;
/// `InterlockedIncrement` / `InterlockedDecrement` (import slots) and
/// `NiRefObject::DeleteThis` (virtual slot 1).
const INTERLOCKED_INCREMENT: u32 = 0x00fd_f1c0;
const INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;
const SLOT_DELETE_THIS: u32 = 0x04;

/// `NiStream::m_uiNifFileVersion` +0xd8 and `m_pkIstr` +0x24c (Xbox PDB
/// names; PC offsets), and the input stream's position (+4) and read
/// function (+8, `cdecl` `(stream, buffer, size, &component size, component
/// count)`, returns the bytes read).
const STREAM_VERSION: u32 = 0xd8;
const STREAM_INPUT: u32 = 0x24c;
const BINARY_STREAM_POSITION: u32 = 0x4;
const BINARY_STREAM_READ: u32 = 0x8;

/// One read through the stream's read function into `buffer`: the word at
/// `slot` is set to `component` and passed as the component size; the
/// position advances by the bytes read.
fn stream_read(e: &mut Engine, stream: u32, buffer: u32, size: u32, component: u32, slot: u32) {
    let input = e.mem.u32(stream.wrapping_add(STREAM_INPUT));
    let read = e.mem.u32(input.wrapping_add(BINARY_STREAM_READ));
    e.mem.set_u32(slot, component);
    let bytes = e.call(read, &args![input, buffer, size, slot, 1u32]).u32();
    let position = e.mem.u32(input.wrapping_add(BINARY_STREAM_POSITION));
    e.mem.set_u32(
        input.wrapping_add(BINARY_STREAM_POSITION),
        position.wrapping_add(bytes),
    );
}

/// The inline `NiPointer` release: `InterlockedDecrement` of the count at
/// `+4` and, when it reaches zero, the object's `DeleteThis`.
fn release(e: &mut Engine, object: u32) {
    if e.call(INTERLOCKED_DECREMENT, &args![object.wrapping_add(4)])
        .u32()
        == 0
    {
        e.vcall(object, SLOT_DELETE_THIS, &args![]);
    }
}

/// `MemoryManager::Allocate` of an array of `count` elements of `size`
/// bytes behind a count word: the size all ones when `count * size`
/// overflows or when adding the 4 bytes carries.
fn allocate_counted_array(e: &mut Engine, count: u32, size: u32) -> u32 {
    let bytes = count.checked_mul(size).unwrap_or(u32::MAX);
    let bytes = bytes.checked_add(4).unwrap_or(u32::MAX);
    e.call(MEMORY_MANAGER_ALLOCATE, &args![MEMORY_MANAGER, bytes])
        .u32()
}

// Translated from 00a415f0 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// `NiMorphData::MorphTarget::MorphTarget` (Ghidra has no function here;
/// the constructor `LoadBinary` hands the vector constructor iterator): no
/// vertices, the empty fixed string as the name, weight 0.0 and no
/// interpolator. Returns `this`.
pub fn ni_morph_data_morph_target_construct(e: &mut Engine, this: Ptr) -> Ptr {
    let target = this.addr();
    e.mem.set_u32(target.wrapping_add(TARGET_VERTICES), 0);
    let empty = e.mem.u32(EMPTY_FIXED_STRING);
    e.mem.set_u32(target.wrapping_add(TARGET_NAME), empty);
    e.mem.set_f32(target.wrapping_add(TARGET_WEIGHT), 0.0);
    e.mem.set_u32(target.wrapping_add(TARGET_INTERPOLATOR), 0);
    this
}

// Translated from 00a41660 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: `NiMorphData::MorphTarget`'s vector deleting
/// destructor. With bit 1 of `flags`, `this` is an array whose count is the
/// word before it: from the last element to the first, the vertex array
/// goes back to the memory manager, the interpolator is released and the
/// name's handle uncounted (unless it is the empty string's); then, with
/// bit 0, the block (from the count word) is deallocated, and the block's
/// address is returned. Without bit 1 the one element's destructor
/// (`00a41610`) runs and, with bit 0, `this` is deallocated; `this` is
/// returned.
pub fn fn_00a41660(e: &mut Engine, this: Ptr, flags: u32) -> u32 {
    let array = this.addr();
    if flags & 2 == 0 {
        e.call(MORPH_TARGET_DESTRUCT, &args![this]);
        if flags & 1 != 0 {
            e.call(MEMORY_MANAGER_DEALLOCATE, &args![MEMORY_MANAGER, array]);
        }
        return array;
    }
    let block = array.wrapping_sub(4);
    let count = e.mem.u32(block);
    let mut element = count.wrapping_shl(4).wrapping_add(array);
    let mut remaining = count as i32;
    loop {
        remaining = remaining.wrapping_sub(1);
        if remaining < 0 {
            break;
        }
        element = element.wrapping_sub(MORPH_TARGET_SIZE);
        let vertices = e.mem.u32(element.wrapping_add(TARGET_VERTICES));
        e.call(MEMORY_MANAGER_DEALLOCATE, &args![MEMORY_MANAGER, vertices]);
        let interpolator = e.mem.u32(element.wrapping_add(TARGET_INTERPOLATOR));
        if interpolator != 0 {
            release(e, interpolator);
        }
        let name = e.mem.u32(element.wrapping_add(TARGET_NAME));
        if name != e.mem.u32(EMPTY_FIXED_STRING) {
            e.call(INTERLOCKED_DECREMENT, &args![name.wrapping_sub(8)]);
        }
    }
    if flags & 1 != 0 {
        e.call(MEMORY_MANAGER_DEALLOCATE, &args![MEMORY_MANAGER, block]);
    }
    block
}

// Translated from 00a41770 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `nimorphdata.cpp`; `cdecl`): reads
/// `count` floats into `floats` in one read of `count * 4` bytes, component
/// size 4 (in the first argument's own stack word).
pub fn fn_00a41770(e: &mut Engine, stream: Ptr, floats: u32, count: u32) {
    let stream = stream.addr();
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), stream);
        let size = count.wrapping_mul(4);
        stream_read(e, stream, floats, size, 4, slot.addr());
    });
}

// Translated from 00a41940 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiMorphData::MorphTarget::LoadBinary` (Xbox PDB): the name (a C string
/// made a fixed string before file version `0x14010001`, a fixed string
/// from then on), the weight (4 bytes) for versions `0x0a010068` up to
/// `0x14010002`, then `vertex_count` vertices (12 bytes each) into a new
/// memory-manager array that becomes the target's, in one read. The
/// component size word is the first argument's own stack word.
pub fn ni_morph_data_morph_target_load_binary(
    e: &mut Engine,
    this: Ptr,
    stream: Ptr,
    vertex_count: u32,
) {
    let target = this.addr();
    let stream = stream.addr();
    let version = e.mem.u32(stream.wrapping_add(STREAM_VERSION));
    let name = target.wrapping_add(TARGET_NAME);
    if version < 0x1401_0001 {
        e.call(LOAD_CSTRING_AS_FIXED_STRING, &args![stream, name]);
    } else {
        e.call(LOAD_FIXED_STRING, &args![stream, name]);
    }
    e.with_stack(4, |e, slot| {
        let slot = slot.addr();
        e.mem.set_u32(slot, stream);
        let version = e.mem.u32(stream.wrapping_add(STREAM_VERSION));
        if (0x0a01_0068..0x1401_0003).contains(&version) {
            stream_read(e, stream, target.wrapping_add(TARGET_WEIGHT), 4, 4, slot);
        }
        let bytes = vertex_count.checked_mul(12).unwrap_or(u32::MAX);
        let vertices = e
            .call(MEMORY_MANAGER_ALLOCATE, &args![MEMORY_MANAGER, bytes])
            .u32();
        e.mem
            .set_u32(target.wrapping_add(TARGET_VERTICES), vertices);
        let size = vertex_count.wrapping_mul(12);
        stream_read(e, stream, vertices, size, 4, slot);
    });
}

// Translated from 00a41a00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiMorphData::_scalar_deleting_destructor_` (Xbox PDB): stores the
/// vtable, destroys the morph target array (`00a41660` with flags 3) when
/// there is one, runs `NiObject`'s destructor body (`00a5d3d0`), then the
/// sized delete of 0x18 bytes when bit 0 of `flags` is set. Returns `this`.
pub fn ni_morph_data_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    let object = this.addr();
    let targets = e.mem.u32(object.wrapping_add(MORPH_TARGETS));
    e.mem.set_u32(object, NI_MORPH_DATA_VTABLE);
    if targets != 0 {
        fn_00a41660(e, Ptr::new(targets), 3);
    }
    e.call(NI_OBJECT_DESTRUCT, &args![this]);
    if flags & 1 != 0 {
        e.call(NI_SIZED_DELETE, &args![this, NI_MORPH_DATA_SIZE]);
    }
    this
}

// Translated from 00a41a40 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00a41a40 - the /GS cookie check (00ec408c) that guards the 256-byte name buffer is not modelled
/// `NiMorphData::LoadBinary` (Xbox PDB): `NiObject::LoadBinary`, the target
/// count, the vertex count per target (4 bytes each) and the relative
/// targets byte (`== 1`), then a memory-manager array of that many morph
/// targets behind a count word, each constructed (`00a415f0` through the
/// vector constructor iterator; null when the allocation fails). Each
/// target then loads: from file version `0x0a010068` on with
/// `MorphTarget::LoadBinary`; before, from the old layout: a key count and
/// key type (4 bytes each); with keys, the type's load and after-load
/// functions (the float-key tables) make the keys, a new
/// `NiFloatInterpolator` (`00a3c340` with no data; null when the
/// allocation fails) takes them (`ReplaceKeys`) and becomes the target's
/// interpolator (counted; the old one released), and its slot `0xbc` runs;
/// then the name `"MT <index>"` (`NiSprintf` into a 256-byte stack buffer,
/// made a fixed string, counted into the target, the old name uncounted,
/// then the temporary's count dropped), and the vertices: a new 12-byte
/// array (`NiColor::LoadBinary` per vertex) replacing the target's (the
/// old one deallocated).
///
/// The key size is pushed as a stack word whose low byte is the size; the
/// three bytes above it are a stack slot this function never writes, so
/// they are passed as zero here.
pub fn ni_morph_data_load_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(NI_OBJECT_LOAD_BINARY, &args![this, stream]);
    let object = this.addr();
    let stream = stream.addr();
    // Frame (the exe's stack words from ESP+0x10 after its pushes): the
    // first component size word +0 (the loop index later), the loop's
    // component size word +4, the relative-targets byte +0x0b, the key count
    // +0x10, the key type +0x14, then the 256-byte name buffer +0x20.
    e.with_stack(0x120, |e, frame| {
        let component = frame.addr();
        let loop_component = component + 4;
        let relative_byte = component + 0x0b;
        let key_count_word = component + 0x10;
        let key_type_word = component + 0x14;
        let buffer = component + 0x20;
        stream_read(e, stream, object.wrapping_add(NUM_TARGETS), 4, 4, component);
        stream_read(
            e,
            stream,
            object.wrapping_add(NUM_VERTS_PER_TARGET),
            4,
            4,
            component,
        );
        stream_read(e, stream, relative_byte, 1, 1, component);
        let count = e.mem.u32(object.wrapping_add(NUM_TARGETS));
        let relative = u8::from(e.mem.u8(relative_byte) == 1);
        e.mem
            .set_u8(object.wrapping_add(RELATIVE_TARGETS), relative);
        let block = allocate_counted_array(e, count, MORPH_TARGET_SIZE);
        let targets = if block == 0 {
            0
        } else {
            e.mem.set_u32(block, count);
            let array = block.wrapping_add(4);
            e.call(
                VECTOR_CONSTRUCTOR_ITERATOR,
                &args![array, MORPH_TARGET_SIZE, count, MORPH_TARGET_CONSTRUCT],
            );
            array
        };
        e.mem.set_u32(object.wrapping_add(MORPH_TARGETS), targets);
        let mut index = 0u32;
        let mut offset = 0u32;
        while index < e.mem.u32(object.wrapping_add(NUM_TARGETS)) {
            if e.mem.u32(stream.wrapping_add(STREAM_VERSION)) >= 0x0a01_0068 {
                let target = e
                    .mem
                    .u32(object.wrapping_add(MORPH_TARGETS))
                    .wrapping_add(offset);
                let vertex_count = e.mem.u32(object.wrapping_add(NUM_VERTS_PER_TARGET));
                ni_morph_data_morph_target_load_binary(
                    e,
                    Ptr::new(target),
                    Ptr::new(stream),
                    vertex_count,
                );
            } else {
                stream_read(e, stream, key_count_word, 4, 4, loop_component);
                stream_read(e, stream, key_type_word, 4, 4, loop_component);
                let key_count = e.mem.u32(key_count_word);
                if key_count != 0 {
                    let key_type = e.mem.u32(key_type_word);
                    let key_size = e.mem.u8(FLOAT_KEY_SIZE_TABLE.wrapping_add(key_type));
                    let load = e
                        .mem
                        .u32(FLOAT_KEY_LOAD_TABLE.wrapping_add(key_type.wrapping_mul(4)));
                    let keys = e.call(load, &args![stream, key_count]).u32();
                    let after_load = e
                        .mem
                        .u32(FLOAT_KEY_AFTER_LOAD_TABLE.wrapping_add(key_type.wrapping_mul(4)));
                    e.call(after_load, &args![keys, key_count, u32::from(key_size)]);
                    let made = e.call(NI_ALLOCATE, &args![0x18u32]).u32();
                    let interpolator = if made == 0 {
                        0
                    } else {
                        e.call(FLOAT_INTERPOLATOR_CONSTRUCT, &args![made, 0u32])
                            .u32()
                    };
                    e.call(
                        FLOAT_INTERPOLATOR_REPLACE_KEYS,
                        &args![interpolator, keys, key_count, key_type],
                    );
                    let slot = e
                        .mem
                        .u32(object.wrapping_add(MORPH_TARGETS))
                        .wrapping_add(offset)
                        .wrapping_add(TARGET_INTERPOLATOR);
                    let old = e.mem.u32(slot);
                    if old != interpolator {
                        if old != 0 {
                            release(e, old);
                        }
                        e.mem.set_u32(slot, interpolator);
                        if interpolator != 0 {
                            e.call(INTERLOCKED_INCREMENT, &args![interpolator.wrapping_add(4)]);
                        }
                    }
                    e.vcall(interpolator, SLOT_INTERPOLATOR_COLLAPSE, &args![]);
                }
                e.call(
                    NI_SPRINTF,
                    &args![buffer, 0x100u32, TARGET_NAME_FORMAT, index],
                );
                let name = e.call(GET_FIXED_STRING, &args![buffer]).u32();
                let name_slot = e
                    .mem
                    .u32(object.wrapping_add(MORPH_TARGETS))
                    .wrapping_add(offset)
                    .wrapping_add(TARGET_NAME);
                if e.mem.u32(name_slot) != name {
                    if name != e.mem.u32(EMPTY_FIXED_STRING) {
                        e.call(INTERLOCKED_INCREMENT, &args![name.wrapping_sub(8)]);
                    }
                    let old = e.mem.u32(name_slot);
                    if old != e.mem.u32(EMPTY_FIXED_STRING) {
                        e.call(INTERLOCKED_DECREMENT, &args![old.wrapping_sub(8)]);
                    }
                    e.mem.set_u32(name_slot, name);
                }
                if name != e.mem.u32(EMPTY_FIXED_STRING) {
                    e.call(INTERLOCKED_DECREMENT, &args![name.wrapping_sub(8)]);
                }
                let vertex_count = e.mem.u32(object.wrapping_add(NUM_VERTS_PER_TARGET));
                let bytes = vertex_count.checked_mul(12).unwrap_or(u32::MAX);
                let vertices = e
                    .call(MEMORY_MANAGER_ALLOCATE, &args![MEMORY_MANAGER, bytes])
                    .u32();
                let mut vertex = 0u32;
                while vertex < e.mem.u32(object.wrapping_add(NUM_VERTS_PER_TARGET)) {
                    e.call(
                        COLOR_LOAD_BINARY,
                        &args![vertices.wrapping_add(vertex.wrapping_mul(12)), stream],
                    );
                    vertex += 1;
                }
                let target = e
                    .mem
                    .u32(object.wrapping_add(MORPH_TARGETS))
                    .wrapping_add(offset);
                let old = e.mem.u32(target.wrapping_add(TARGET_VERTICES));
                e.call(MEMORY_MANAGER_DEALLOCATE, &args![MEMORY_MANAGER, old]);
                e.mem
                    .set_u32(target.wrapping_add(TARGET_VERTICES), vertices);
            }
            offset = offset.wrapping_add(MORPH_TARGET_SIZE);
            index += 1;
        }
    });
}

// Translated from 00a41610 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiMorphData::MorphTarget::~MorphTarget` (unnamed in the engine map; the address
/// `00a41660` calls for one element): the vertex array goes back to the memory manager, the
/// interpolator is released and the name's handle uncounted unless it is the empty string's.
pub fn ni_morph_data_morph_target_destruct(e: &mut Engine, this: Ptr) {
    let target = this.addr();
    let vertices = e.mem.u32(target.wrapping_add(TARGET_VERTICES));
    e.call(MEMORY_MANAGER_DEALLOCATE, &args![MEMORY_MANAGER, vertices]);
    let interpolator = e.mem.u32(target.wrapping_add(TARGET_INTERPOLATOR));
    if interpolator != 0 {
        release(e, interpolator);
    }
    let name = e.mem.u32(target.wrapping_add(TARGET_NAME));
    if name != e.mem.u32(EMPTY_FIXED_STRING) {
        e.call(INTERLOCKED_DECREMENT, &args![name.wrapping_sub(8)]);
    }
}

/// `NiObject::RegisterStreamables` (`00a5d2f0`), `NiObject::IsEqual` (`00a5d470`),
/// `NiObject::GetViewerStrings` (`00a5d6e0`) and the base `SaveBinary` (`00a89af0`), all
/// thiscall with one word.
const NI_OBJECT_REGISTER_STREAMABLES: u32 = 0x00a5_d2f0;
const NI_OBJECT_IS_EQUAL: u32 = 0x00a5_d470;
const NI_OBJECT_GET_VIEWER_STRINGS: u32 = 0x00a5_d6e0;
const NI_OBJECT_SAVE_BINARY: u32 = 0x00a8_9af0;
/// The stream's `RegisterFixedString` (virtual slot `0x20`, the string's address) and
/// `NiStream::SaveFixedString` (`00a641f0`, thiscall, the string's address).
const SLOT_REGISTER_FIXED_STRING: u32 = 0x20;
const SAVE_FIXED_STRING: u32 = 0x00a6_41f0;
/// `NiStream::m_pkOstr` (Xbox PDB name, PC offset +0x250) and the output stream's write
/// function (+0xc, `cdecl` like the read function).
const STREAM_OUTPUT: u32 = 0x250;
const BINARY_STREAM_WRITE: u32 = 0xc;
/// Viewer strings: `NiGetViewerString` (`00a7d330`), its unsigned form (`00a7d510`) and its
/// fixed-string form (`00a7d5e0`), all cdecl; the array's resize (`0096ad30`) and `SetAt`
/// (`0096ae90`), thiscall.
const NI_GET_VIEWER_STRING: u32 = 0x00a7_d330;
const NI_GET_VIEWER_STRING_UNSIGNED: u32 = 0x00a7_d510;
const NI_GET_VIEWER_STRING_FIXED: u32 = 0x00a7_d5e0;
const VIEWER_STRINGS_GROW: u32 = 0x0096_ad30;
const VIEWER_STRINGS_SET_AT: u32 = 0x0096_ae90;
/// The labels `"m_uiNumTargets"`, `"m_uiNumVertsPerTarget"`, `"MorphTargets"` and
/// `"m_pcName"`.
const TEXT_NUM_TARGETS: u32 = 0x0109_77c4;
const TEXT_NUM_VERTS_PER_TARGET: u32 = 0x0109_77ac;
const TEXT_MORPH_TARGETS: u32 = 0x0109_779c;
const TEXT_NAME: u32 = 0x0109_7790;

/// One write through the stream's output stream's write function from `buffer`: the word at
/// `slot` is set to `component` and passed as the component size; the position advances by
/// the bytes written.
fn stream_write(e: &mut Engine, stream: u32, buffer: u32, size: u32, component: u32, slot: u32) {
    let output = e.mem.u32(stream.wrapping_add(STREAM_OUTPUT));
    let write = e.mem.u32(output.wrapping_add(BINARY_STREAM_WRITE));
    e.mem.set_u32(slot, component);
    let bytes = e
        .call(write, &args![output, buffer, size, slot, 1u32])
        .u32();
    let position = e.mem.u32(output.wrapping_add(BINARY_STREAM_POSITION));
    e.mem.set_u32(
        output.wrapping_add(BINARY_STREAM_POSITION),
        position.wrapping_add(bytes),
    );
}

/// The morph target at `index`, or 0 when `index` is not below the target count.
fn target_at(e: &Engine, data: u32, index: u32, offset: u32) -> u32 {
    if index < e.mem.u32(data.wrapping_add(NUM_TARGETS)) {
        e.mem
            .u32(data.wrapping_add(MORPH_TARGETS))
            .wrapping_add(offset)
    } else {
        0
    }
}

// Translated from 00a41710 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiMorphData::RegisterStreamables` (Xbox PDB): false when `NiObject::RegisterStreamables`
/// is; otherwise each morph target's name registers with the stream (its slot `0x20`, the
/// name's address; the count re-read after each). True.
pub fn ni_morph_data_register_streamables(e: &mut Engine, this: Ptr, stream: Ptr) -> bool {
    if !e
        .call(NI_OBJECT_REGISTER_STREAMABLES, &args![this, stream])
        .bool()
    {
        return false;
    }
    let data = this.addr();
    let mut index = 0u32;
    let mut offset = 0u32;
    while index < e.mem.u32(data.wrapping_add(NUM_TARGETS)) {
        let target = target_at(e, data, index, offset);
        e.vcall(
            stream.addr(),
            SLOT_REGISTER_FIXED_STRING,
            &args![target.wrapping_add(TARGET_NAME)],
        );
        index += 1;
        offset = offset.wrapping_add(MORPH_TARGET_SIZE);
    }
    true
}

// Translated from 00a417b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `nimorphdata.cpp`; `cdecl`): writes `count` floats from
/// `floats` in one write of `count * 4` bytes, component size 4 (in the first argument's own
/// stack word).
pub fn fn_00a417b0(e: &mut Engine, stream: Ptr, floats: u32, count: u32) {
    let stream = stream.addr();
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), stream);
        let size = count.wrapping_mul(4);
        stream_write(e, stream, floats, size, 4, slot.addr());
    });
}

// Translated from 00a417f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiMorphData::MorphTarget::IsEqual` (Xbox PDB): every one of `vertex_count` vertices of
/// `other` equals this target's (x, y and z, unordered is unequal), and the name handles are
/// the same.
pub fn ni_morph_data_morph_target_is_equal(
    e: &mut Engine,
    this: Ptr,
    other: Ptr,
    vertex_count: u32,
) -> bool {
    // The vertex arrays are read only when there are vertices to compare.
    let (ours, theirs) = if vertex_count != 0 {
        (
            e.mem.u32(this.addr().wrapping_add(TARGET_VERTICES)),
            e.mem.u32(other.addr().wrapping_add(TARGET_VERTICES)),
        )
    } else {
        (0, 0)
    };
    for vertex in 0..vertex_count {
        let offset = vertex.wrapping_mul(12);
        for lane in [0u32, 4, 8] {
            let mine = x87_load32(e.mem.u32(ours.wrapping_add(offset + lane)));
            let other = x87_load32(e.mem.u32(theirs.wrapping_add(offset + lane)));
            if other != mine {
                return false;
            }
        }
    }
    let name = e.mem.u32(this.addr().wrapping_add(TARGET_NAME));
    name == e.mem.u32(other.addr().wrapping_add(TARGET_NAME))
}

// Translated from 00a418a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiMorphData::IsEqual` (Xbox PDB): `NiObject::IsEqual`, the same target count, vertex
/// count and relative-targets byte, then each target `IsEqual` the other's (`00a417f0` on
/// this data's target with the other's and this data's vertex count; a target index past a
/// count gives a null target).
pub fn ni_morph_data_is_equal(e: &mut Engine, this: Ptr, other: Ptr) -> bool {
    if !e.call(NI_OBJECT_IS_EQUAL, &args![this, other]).bool() {
        return false;
    }
    let data = this.addr();
    let other = other.addr();
    let other_count = e.mem.u32(other.wrapping_add(NUM_TARGETS));
    let count = e.mem.u32(data.wrapping_add(NUM_TARGETS));
    if count != other_count
        || e.mem.u32(data.wrapping_add(NUM_VERTS_PER_TARGET))
            != e.mem.u32(other.wrapping_add(NUM_VERTS_PER_TARGET))
        || e.mem.u8(data.wrapping_add(RELATIVE_TARGETS))
            != e.mem.u8(other.wrapping_add(RELATIVE_TARGETS))
    {
        return false;
    }
    let mut offset = 0u32;
    for index in 0..count {
        let theirs = if index < other_count {
            e.mem
                .u32(other.wrapping_add(MORPH_TARGETS))
                .wrapping_add(offset)
        } else {
            0
        };
        let ours = if index < count {
            e.mem
                .u32(data.wrapping_add(MORPH_TARGETS))
                .wrapping_add(offset)
        } else {
            0
        };
        let vertex_count = e.mem.u32(data.wrapping_add(NUM_VERTS_PER_TARGET));
        if !ni_morph_data_morph_target_is_equal(e, Ptr::new(ours), Ptr::new(theirs), vertex_count) {
            return false;
        }
        offset = offset.wrapping_add(MORPH_TARGET_SIZE);
    }
    true
}

// Translated from 00a41d90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiMorphData::SaveBinary` (Xbox PDB): the base's `SaveBinary` (`00a89af0`), the target
/// count and vertex count (4 bytes each), the relative-targets byte (from the argument's own
/// stack word), then per target its name (`NiStream::SaveFixedString`) and, with vertices
/// per target, its vertices (12 bytes each, component size 4).
pub fn ni_morph_data_save_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(NI_OBJECT_SAVE_BINARY, &args![this, stream]);
    let data = this.addr();
    let stream = stream.addr();
    // Frame: the component size words +0 and +4, the argument word +8.
    e.with_stack(12, |e, frame| {
        let component = frame.addr();
        let vertex_component = component + 4;
        let argument = component + 8;
        e.mem.set_u32(argument, stream);
        stream_write(e, stream, data.wrapping_add(NUM_TARGETS), 4, 4, component);
        let verts = data.wrapping_add(NUM_VERTS_PER_TARGET);
        stream_write(e, stream, verts, 4, 4, component);
        let relative = e.mem.u8(data.wrapping_add(RELATIVE_TARGETS));
        e.mem.set_u8(argument, relative);
        stream_write(e, stream, argument, 1, 1, component);
        let mut index = 0u32;
        let mut offset = 0u32;
        while index < e.mem.u32(data.wrapping_add(NUM_TARGETS)) {
            let target = e
                .mem
                .u32(data.wrapping_add(MORPH_TARGETS))
                .wrapping_add(offset);
            let vertex_count = e.mem.u32(data.wrapping_add(NUM_VERTS_PER_TARGET));
            e.call(
                SAVE_FIXED_STRING,
                &args![stream, target.wrapping_add(TARGET_NAME)],
            );
            if vertex_count != 0 {
                let vertices = e.mem.u32(target.wrapping_add(TARGET_VERTICES));
                let size = vertex_count.wrapping_mul(12);
                stream_write(e, stream, vertices, size, 4, vertex_component);
            }
            offset = offset.wrapping_add(MORPH_TARGET_SIZE);
            index += 1;
        }
    });
}

/// The viewer-string array append the compiler inlined with the string in the stack word
/// `slot`: grows the array (16-bit size +0xA, capacity +8, grow-by +0xE) when full, then
/// `SetAt` at the old size with the slot's address.
fn add_viewer_string(e: &mut Engine, strings: u32, slot: u32) {
    let size = u32::from(e.mem.u16(strings.wrapping_add(0xa)));
    if size >= u32::from(e.mem.u16(strings.wrapping_add(8))) {
        let step = u32::from(e.mem.u16(strings.wrapping_add(0xe)));
        e.call(VIEWER_STRINGS_GROW, &args![strings, step + size]);
    }
    e.call(VIEWER_STRINGS_SET_AT, &args![strings, size, slot]);
}

/// The inline `NiTPrimitiveArray::SetAtGrow` of `string` at the array's size (data +4, 16-bit
/// size +0xA, capacity +8, used count +0xC, grow-by +0xE): grows when full; a slot at or past
/// the (new) size makes the size one more, the used count counting a non-null string; inside
/// it the used count follows the change between null and non-null; then the slot is set.
fn set_at_grow(e: &mut Engine, strings: u32, string: u32) {
    let index = u32::from(e.mem.u16(strings.wrapping_add(0xa)));
    if index >= u32::from(e.mem.u16(strings.wrapping_add(8))) {
        let step = u32::from(e.mem.u16(strings.wrapping_add(0xe)));
        e.call(VIEWER_STRINGS_GROW, &args![strings, step + index]);
    }
    let used = strings.wrapping_add(0xc);
    if index >= u32::from(e.mem.u16(strings.wrapping_add(0xa))) {
        e.mem.set_u16(strings.wrapping_add(0xa), (index + 1) as u16);
        if string != 0 {
            e.mem.set_u16(used, e.mem.u16(used).wrapping_add(1));
        }
    } else if string != 0 {
        let data = e.mem.u32(strings.wrapping_add(4));
        if e.mem.u32(data.wrapping_add(index * 4)) == 0 {
            e.mem.set_u16(used, e.mem.u16(used).wrapping_add(1));
        }
    } else {
        let data = e.mem.u32(strings.wrapping_add(4));
        if e.mem.u32(data.wrapping_add(index * 4)) != 0 {
            e.mem.set_u16(used, e.mem.u16(used).wrapping_add(0xffff));
        }
    }
    let data = e.mem.u32(strings.wrapping_add(4));
    e.mem.set_u32(data.wrapping_add(index * 4), string);
}

// Translated from 00a41ea0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiMorphData::GetViewerStrings` (Xbox PDB): `NiObject::GetViewerStrings`, the class name
/// (`011f3e74`), `"m_uiNumTargets"`, `"m_uiNumVertsPerTarget"` and `"MorphTargets"` (held in
/// the argument's own stack word), then per target `"m_pcName"` with its name handle
/// (`00a7d5e0`) stored with the inline `SetAtGrow` (a target index past the count reads
/// through a null target, as the exe does).
pub fn ni_morph_data_get_viewer_strings(e: &mut Engine, this: Ptr, strings: Ptr) {
    e.call(NI_OBJECT_GET_VIEWER_STRINGS, &args![this, strings]);
    let data = this.addr();
    let strings = strings.addr();
    e.with_stack(4, |e, slot| {
        let slot = slot.addr();
        e.mem.set_u32(slot, strings);
        let class_name = e.mem.u32(NI_MORPH_DATA_RTTI);
        let text = e.call(NI_GET_VIEWER_STRING, &args![class_name]).u32();
        e.mem.set_u32(slot, text);
        add_viewer_string(e, strings, slot);
        let count = e.mem.u32(data.wrapping_add(NUM_TARGETS));
        let text = e
            .call(
                NI_GET_VIEWER_STRING_UNSIGNED,
                &args![TEXT_NUM_TARGETS, count],
            )
            .u32();
        e.mem.set_u32(slot, text);
        add_viewer_string(e, strings, slot);
        let verts = e.mem.u32(data.wrapping_add(NUM_VERTS_PER_TARGET));
        let text = e
            .call(
                NI_GET_VIEWER_STRING_UNSIGNED,
                &args![TEXT_NUM_VERTS_PER_TARGET, verts],
            )
            .u32();
        e.mem.set_u32(slot, text);
        add_viewer_string(e, strings, slot);
        let text = e
            .call(NI_GET_VIEWER_STRING, &args![TEXT_MORPH_TARGETS])
            .u32();
        e.mem.set_u32(slot, text);
        add_viewer_string(e, strings, slot);
        let mut index = 0u32;
        let mut offset = 0u32;
        while index < e.mem.u32(data.wrapping_add(NUM_TARGETS)) {
            let target = target_at(e, data, index, offset);
            let name = e.mem.u32(target.wrapping_add(TARGET_NAME));
            let text = e
                .call(NI_GET_VIEWER_STRING_FIXED, &args![TEXT_NAME, name])
                .u32();
            set_at_grow(e, strings, text);
            offset = offset.wrapping_add(MORPH_TARGET_SIZE);
            index += 1;
        }
    });
}

// Translated from 00a57850 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiMorphData::MorphTarget::RegisterStreamables` (Xbox PDB): a jump to the stream's
/// `RegisterFixedString` (its slot `0x20`) with the name's address (`this + 4`, written into
/// the argument's stack word); its answer is returned.
pub fn ni_morph_data_morph_target_register_streamables(
    e: &mut Engine,
    this: Ptr,
    stream: Ptr,
) -> bool {
    let name = this.addr().wrapping_add(TARGET_NAME);
    e.vcall(stream.addr(), SLOT_REGISTER_FIXED_STRING, &args![name])
        .bool()
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a415e0, ni_morph_data_get_rtti() -> Ptr),
        entry!(0x00a4_15f0, ni_morph_data_morph_target_construct(Ptr) -> Ptr),
        entry!(0x00a4_1660, fn_00a41660(Ptr, u32) -> u32),
        entry!(0x00a4_1770, fn_00a41770(Ptr, u32, u32)),
        entry!(
            0x00a4_1940,
            ni_morph_data_morph_target_load_binary(Ptr, Ptr, u32)
        ),
        entry!(0x00a4_1a00, ni_morph_data_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x00a4_1a40, ni_morph_data_load_binary(Ptr, Ptr)),
        entry!(0x00a4_1610, ni_morph_data_morph_target_destruct(Ptr)),
        entry!(
            0x00a4_1710,
            ni_morph_data_register_streamables(Ptr, Ptr) -> bool
        ),
        entry!(0x00a4_17b0, fn_00a417b0(Ptr, u32, u32)),
        entry!(
            0x00a4_17f0,
            ni_morph_data_morph_target_is_equal(Ptr, Ptr, u32) -> bool
        ),
        entry!(0x00a4_18a0, ni_morph_data_is_equal(Ptr, Ptr) -> bool),
        entry!(0x00a4_1d90, ni_morph_data_save_binary(Ptr, Ptr)),
        entry!(0x00a4_1ea0, ni_morph_data_get_viewer_strings(Ptr, Ptr)),
        entry!(
            0x00a5_7850,
            ni_morph_data_morph_target_register_streamables(Ptr, Ptr) -> bool
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ni_morph_data_get_rtti_returns_the_class_rtti() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00a415e0, &args![]).u32(), NI_MORPH_DATA_RTTI);
    }

    type Log = std::rc::Rc<std::cell::RefCell<Vec<(u32, Vec<u32>)>>>;

    const READ: u32 = 0x00ee_0400;
    const DELETE_THIS: u32 = 0x00ee_0500;
    const COLLAPSE: u32 = 0x00ee_0600;
    const LOAD_KEYS: u32 = 0x00ee_0700;
    const AFTER_LOAD: u32 = 0x00ee_0800;

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    fn logged(e: &mut Engine, log: &Log, address: u32, eax: u32) {
        let log = log.clone();
        e.register_double(address, move |_, a| {
            log.borrow_mut().push((address, a.to_vec()));
            ret(eax)
        });
    }

    fn calls(log: &Log, address: u32) -> Vec<Vec<u32>> {
        log.borrow()
            .iter()
            .filter(|c| c.0 == address)
            .map(|c| c.1.clone())
            .collect()
    }

    /// The interlocked import slots, logged and counting in memory.
    fn interlocked(e: &mut Engine, log: &Log) {
        for (address, step) in [
            (INTERLOCKED_INCREMENT, 1u32),
            (INTERLOCKED_DECREMENT, u32::MAX),
        ] {
            let log = log.clone();
            e.register_double(address, move |e, a| {
                log.borrow_mut().push((address, a.to_vec()));
                let count = e.mem.u32(a[0]).wrapping_add(step);
                e.mem.set_u32(a[0], count);
                ret(count)
            });
        }
    }

    fn counted(e: &mut Engine, count: u32) -> u32 {
        let vtable = e.mem.alloc(0x100);
        e.mem.set_u32(vtable + SLOT_DELETE_THIS, DELETE_THIS);
        e.mem.set_u32(vtable + SLOT_INTERPOLATOR_COLLAPSE, COLLAPSE);
        let object = e.mem.alloc(0x18);
        e.mem.set_u32(object, vtable);
        e.mem.set_u32(object + 4, count);
        object
    }

    #[test]
    fn morph_target_construct_clears_the_target() {
        let mut e = Engine::new();
        e.map(EMPTY_FIXED_STRING, 4);
        e.mem.set_u32(EMPTY_FIXED_STRING, 0x5550);
        let target = e.mem.alloc(0x10);
        e.mem.write(target, &[0xaa; 0x10]);
        assert_eq!(e.call(0x00a4_15f0, &args![target]).u32(), target);
        assert_eq!(e.mem.u32(target), 0);
        assert_eq!(e.mem.u32(target + 4), 0x5550);
        assert_eq!(e.mem.u32(target + 8), 0);
        assert_eq!(e.mem.u32(target + 0xc), 0);
    }

    #[test]
    fn vector_deleting_destructor_destroys_each_target_from_the_last() {
        let mut e = Engine::new();
        let log = Log::default();
        interlocked(&mut e, &log);
        logged(&mut e, &log, MEMORY_MANAGER_DEALLOCATE, 0);
        logged(&mut e, &log, MORPH_TARGET_DESTRUCT, 0);
        logged(&mut e, &log, DELETE_THIS, 0);
        e.map(EMPTY_FIXED_STRING, 4);
        e.mem.set_u32(EMPTY_FIXED_STRING, 0x5550);
        let block = e.mem.alloc(4 + 0x20);
        let array = block + 4;
        e.mem.set_u32(block, 2);
        let interpolator = counted(&mut e, 1);
        // Element 0: vertices 0x100, empty name; element 1: vertices
        // 0x200, a counted name, an interpolator.
        let name = e.mem.alloc(0x10) + 8;
        e.mem.set_u32(name - 8, 3);
        e.mem.set_u32(array, 0x100);
        e.mem.set_u32(array + 4, 0x5550);
        e.mem.set_u32(array + 0x10, 0x200);
        e.mem.set_u32(array + 0x14, name);
        e.mem.set_u32(array + 0x1c, interpolator);
        assert_eq!(e.call(0x00a4_1660, &args![array, 3u32]).u32(), block);
        assert_eq!(
            *log.borrow(),
            [
                (MEMORY_MANAGER_DEALLOCATE, vec![MEMORY_MANAGER, 0x200]),
                (INTERLOCKED_DECREMENT, vec![interpolator + 4]),
                (DELETE_THIS, vec![interpolator]),
                (INTERLOCKED_DECREMENT, vec![name - 8]),
                (MEMORY_MANAGER_DEALLOCATE, vec![MEMORY_MANAGER, 0x100]),
                (MEMORY_MANAGER_DEALLOCATE, vec![MEMORY_MANAGER, block]),
            ]
        );
        // One element: its destructor, then the delete on bit 0.
        log.borrow_mut().clear();
        assert_eq!(e.call(0x00a4_1660, &args![array, 1u32]).u32(), array);
        assert_eq!(
            *log.borrow(),
            [
                (MORPH_TARGET_DESTRUCT, vec![array]),
                (MEMORY_MANAGER_DEALLOCATE, vec![MEMORY_MANAGER, array]),
            ]
        );
    }

    /// A stream of file `version` whose reads log (buffer, size, component
    /// size, count) and fill their buffer with the next word's bytes.
    fn stream(e: &mut Engine, log: &Log, version: u32, words: &'static [u32]) -> u32 {
        let stream = e.mem.alloc(0x5c4);
        let input = e.mem.alloc(0x10);
        e.mem.set_u32(stream + STREAM_INPUT, input);
        e.mem.set_u32(stream + STREAM_VERSION, version);
        e.mem.set_u32(input + BINARY_STREAM_READ, READ);
        let l = log.clone();
        let mut next = words.iter();
        e.register_double(READ, move |e, a| {
            l.borrow_mut()
                .push((READ, vec![a[1], a[2], e.mem.u32(a[3]), a[4]]));
            let word = next.next().unwrap().to_le_bytes();
            for b in 0..a[2].min(4) {
                e.mem.set_u8(a[1] + b, word[b as usize]);
            }
            ret(a[2])
        });
        stream
    }

    #[test]
    fn fn_00a41770_reads_the_floats_in_one_read() {
        let mut e = Engine::new();
        let log = Log::default();
        let stream = stream(&mut e, &log, 0, &[0x3f80_0000]);
        let floats = e.mem.alloc(12);
        e.call(0x00a4_1770, &args![stream, floats, 3u32]);
        assert_eq!(calls(&log, READ), [vec![floats, 12, 4, 1]]);
        assert_eq!(e.mem.f32(floats), 1.0);
    }

    #[test]
    fn morph_target_load_binary_reads_name_weight_and_vertices() {
        let mut e = Engine::new();
        let log = Log::default();
        let vertices = e.mem.alloc(36);
        logged(&mut e, &log, MEMORY_MANAGER_ALLOCATE, vertices);
        logged(&mut e, &log, LOAD_FIXED_STRING, 0);
        logged(&mut e, &log, LOAD_CSTRING_AS_FIXED_STRING, 0);
        let new = stream(&mut e, &log, 0x1401_0002, &[0x3f00_0000, 7]);
        let target = e.mem.alloc(0x10);
        e.call(0x00a4_1940, &args![target, new, 3u32]);
        assert_eq!(
            *log.borrow(),
            [
                (LOAD_FIXED_STRING, vec![new, target + 4]),
                (READ, vec![target + 8, 4, 4, 1]),
                (MEMORY_MANAGER_ALLOCATE, vec![MEMORY_MANAGER, 36]),
                (READ, vec![vertices, 36, 4, 1]),
            ]
        );
        assert_eq!(e.mem.f32(target + 8), 0.5);
        assert_eq!(e.mem.u32(target), vertices);
        // Older files: a C string name and no weight.
        log.borrow_mut().clear();
        let old = stream(&mut e, &log, 0x0a01_0067, &[7]);
        e.call(0x00a4_1940, &args![target, old, 3u32]);
        assert_eq!(
            log.borrow()[0],
            (LOAD_CSTRING_AS_FIXED_STRING, vec![old, target + 4])
        );
        assert_eq!(calls(&log, READ).len(), 1);
    }

    #[test]
    fn scalar_deleting_destructor_destroys_the_targets() {
        let mut e = Engine::new();
        let log = Log::default();
        logged(&mut e, &log, NI_OBJECT_DESTRUCT, 0);
        logged(&mut e, &log, NI_SIZED_DELETE, 0);
        logged(&mut e, &log, MEMORY_MANAGER_DEALLOCATE, 0);
        let block = e.mem.alloc(4);
        let this = e.mem.alloc(0x18);
        e.mem.set_u32(this + MORPH_TARGETS, block + 4);
        assert_eq!(e.call(0x00a4_1a00, &args![this, 1u32]).u32(), this);
        assert_eq!(e.mem.u32(this), NI_MORPH_DATA_VTABLE);
        assert_eq!(
            *log.borrow(),
            [
                (MEMORY_MANAGER_DEALLOCATE, vec![MEMORY_MANAGER, block]),
                (NI_OBJECT_DESTRUCT, vec![this]),
                (NI_SIZED_DELETE, vec![this, 0x18]),
            ]
        );
        log.borrow_mut().clear();
        e.mem.set_u32(this + MORPH_TARGETS, 0);
        e.call(0x00a4_1a00, &args![this, 0u32]);
        assert_eq!(*log.borrow(), [(NI_OBJECT_DESTRUCT, vec![this])]);
    }

    /// `LoadBinary` of two targets of three vertices from a stream of file
    /// `version`.
    fn load(version: u32, words: &'static [u32]) -> (Engine, Log, u32, u32, u32) {
        let mut e = Engine::new();
        let log = Log::default();
        interlocked(&mut e, &log);
        e.map(EMPTY_FIXED_STRING, 4);
        e.mem.set_u32(EMPTY_FIXED_STRING, 0x5550);
        let stream = stream(&mut e, &log, version, words);
        let block = e.mem.alloc(0x24);
        let l = log.clone();
        let mut first = Some(block);
        e.register_double(MEMORY_MANAGER_ALLOCATE, move |e, a| {
            l.borrow_mut().push((MEMORY_MANAGER_ALLOCATE, a.to_vec()));
            ret(first.take().unwrap_or_else(|| e.mem.alloc(a[1])))
        });
        let l = log.clone();
        e.register_double(VECTOR_CONSTRUCTOR_ITERATOR, move |e, a| {
            l.borrow_mut()
                .push((VECTOR_CONSTRUCTOR_ITERATOR, a.to_vec()));
            for i in 0..a[2] {
                e.mem.set_u32(a[0] + 0x10 * i + 4, 0x5550);
            }
            Ret::default()
        });
        for address in [
            NI_OBJECT_LOAD_BINARY,
            LOAD_FIXED_STRING,
            LOAD_CSTRING_AS_FIXED_STRING,
            MEMORY_MANAGER_DEALLOCATE,
            COLOR_LOAD_BINARY,
            NI_SPRINTF,
            AFTER_LOAD,
            FLOAT_INTERPOLATOR_REPLACE_KEYS,
            COLLAPSE,
        ] {
            logged(&mut e, &log, address, 0);
        }
        logged(&mut e, &log, LOAD_KEYS, 0x6000);
        let interpolator = counted(&mut e, 0);
        logged(&mut e, &log, NI_ALLOCATE, interpolator);
        logged(&mut e, &log, FLOAT_INTERPOLATOR_CONSTRUCT, interpolator);
        let name = e.mem.alloc(0x10) + 8;
        logged(&mut e, &log, GET_FIXED_STRING, name);
        e.map(FLOAT_KEY_SIZE_TABLE, 0x10);
        e.map(FLOAT_KEY_LOAD_TABLE, 0x40);
        e.map(FLOAT_KEY_AFTER_LOAD_TABLE, 0x40);
        e.mem.set_u8(FLOAT_KEY_SIZE_TABLE + 2, 0x0c);
        e.mem.set_u32(FLOAT_KEY_LOAD_TABLE + 8, LOAD_KEYS);
        e.mem.set_u32(FLOAT_KEY_AFTER_LOAD_TABLE + 8, AFTER_LOAD);
        let this = e.mem.alloc(0x18);
        e.call(0x00a4_1a40, &args![this, stream]);
        (e, log, this, stream, interpolator)
    }

    #[test]
    fn load_binary_loads_new_targets_through_morph_target_load_binary() {
        let (e, log, this, stream, _) = load(0x1402_0007, &[2, 3, 1, 0, 0]);
        assert_eq!(e.mem.u32(this + NUM_TARGETS), 2);
        assert_eq!(e.mem.u32(this + NUM_VERTS_PER_TARGET), 3);
        assert_eq!(e.mem.u8(this + RELATIVE_TARGETS), 1);
        let array = e.mem.u32(this + MORPH_TARGETS);
        assert_eq!(
            calls(&log, VECTOR_CONSTRUCTOR_ITERATOR),
            [vec![array, 0x10, 2, MORPH_TARGET_CONSTRUCT]]
        );
        assert_eq!(e.mem.u32(array - 4), 2);
        assert_eq!(
            calls(&log, LOAD_FIXED_STRING),
            [vec![stream, array + 4], vec![stream, array + 0x14]]
        );
        assert_eq!(
            calls(&log, MEMORY_MANAGER_ALLOCATE),
            [
                vec![MEMORY_MANAGER, 0x24],
                vec![MEMORY_MANAGER, 36],
                vec![MEMORY_MANAGER, 36]
            ]
        );
    }

    #[test]
    fn load_binary_builds_old_targets_from_keys() {
        // Two targets, three vertices, relative byte 2; target 0 has two
        // keys of type 2, target 1 none.
        let (e, log, this, stream, interpolator) = load(0x0a01_0067, &[2, 3, 2, 2, 2, 0, 0]);
        assert_eq!(e.mem.u8(this + RELATIVE_TARGETS), 0);
        let array = e.mem.u32(this + MORPH_TARGETS);
        assert_eq!(calls(&log, LOAD_KEYS), [vec![stream, 2]]);
        assert_eq!(calls(&log, AFTER_LOAD), [vec![0x6000, 2, 0x0c]]);
        assert_eq!(
            calls(&log, FLOAT_INTERPOLATOR_REPLACE_KEYS),
            [vec![interpolator, 0x6000, 2, 2]]
        );
        assert_eq!(calls(&log, COLLAPSE), [vec![interpolator]]);
        assert_eq!(e.mem.u32(array + TARGET_INTERPOLATOR), interpolator);
        assert_eq!(e.mem.u32(interpolator + 4), 1);
        let sprintf = calls(&log, NI_SPRINTF);
        assert_eq!(sprintf.len(), 2);
        assert_eq!(sprintf[0][1..], [0x100, TARGET_NAME_FORMAT, 0]);
        assert_eq!(sprintf[1][3], 1);
        let name = e.mem.u32(array + TARGET_NAME);
        assert_eq!(e.mem.u32(array + 0x14), name);
        // Counted twice into the targets, the temporary dropped each time.
        assert_eq!(e.mem.u32(name - 8), 0);
        assert_eq!(calls(&log, COLOR_LOAD_BINARY).len(), 6);
        assert_eq!(calls(&log, MEMORY_MANAGER_DEALLOCATE).len(), 2);
        assert_ne!(e.mem.u32(array + TARGET_VERTICES), 0);
    }
}

#[cfg(test)]
mod n004_tests {
    use super::*;
    use std::cell::{Cell, RefCell};
    use std::rc::Rc;

    type Log = Rc<RefCell<Vec<(u32, Vec<u32>)>>>;
    type Writes = Rc<RefCell<Vec<(Vec<u8>, u32, u32)>>>;

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    fn logged(e: &mut Engine, log: &Log, address: u32, eax: u32) {
        let log = log.clone();
        e.register_double(address, move |_, a| {
            log.borrow_mut().push((address, a.to_vec()));
            ret(eax)
        });
    }

    fn calls(log: &Log, address: u32) -> Vec<Vec<u32>> {
        log.borrow()
            .iter()
            .filter(|c| c.0 == address)
            .map(|c| c.1.clone())
            .collect()
    }

    /// Morph data with `count` targets (names 0x100 + index, 12-byte vertices).
    fn data(e: &mut Engine, count: u32, verts: u32) -> u32 {
        let data = e.mem.alloc(0x18);
        let targets = e.mem.alloc(0x10 * count.max(1));
        for i in 0..count {
            let vertices = e.mem.alloc(12 * verts.max(1));
            for k in 0..3 * verts {
                e.mem.set_f32(vertices + 4 * k, k as f32);
            }
            e.mem.set_u32(targets + 0x10 * i, vertices);
            e.mem.set_u32(targets + 0x10 * i + TARGET_NAME, 0x100 + i);
        }
        e.mem.set_u32(data + NUM_TARGETS, count);
        e.mem.set_u32(data + NUM_VERTS_PER_TARGET, verts);
        e.mem.set_u32(data + MORPH_TARGETS, targets);
        data
    }

    #[test]
    fn morph_target_destructor_frees_releases_and_uncounts() {
        let mut e = Engine::new();
        let log = Log::default();
        e.map(EMPTY_FIXED_STRING, 4);
        e.mem.set_u32(EMPTY_FIXED_STRING, 0x0900_0008);
        logged(&mut e, &log, MEMORY_MANAGER_DEALLOCATE, 0);
        logged(&mut e, &log, INTERLOCKED_DECREMENT, 0);
        logged(&mut e, &log, 0x00ee_3000, 0);
        let vtable = e.mem.alloc(8);
        e.mem.set_u32(vtable + SLOT_DELETE_THIS, 0x00ee_3000);
        let interpolator = e.mem.alloc(8);
        e.mem.set_u32(interpolator, vtable);
        let target = e.mem.alloc(0x10);
        e.mem.set_u32(target, 0x4000);
        e.mem.set_u32(target + TARGET_NAME, 0x0800_0008);
        e.mem.set_u32(target + TARGET_INTERPOLATOR, interpolator);
        e.call(0x00a4_1610, &args![target]);
        assert_eq!(
            calls(&log, MEMORY_MANAGER_DEALLOCATE),
            [vec![MEMORY_MANAGER, 0x4000]]
        );
        assert_eq!(
            calls(&log, INTERLOCKED_DECREMENT),
            [vec![interpolator + 4], vec![0x0800_0000]]
        );
        assert_eq!(calls(&log, 0x00ee_3000), [vec![interpolator]]);
        // The empty string's handle and a missing interpolator are left alone.
        e.mem.set_u32(target + TARGET_NAME, 0x0900_0008);
        e.mem.set_u32(target + TARGET_INTERPOLATOR, 0);
        e.call(0x00a4_1610, &args![target]);
        assert_eq!(calls(&log, INTERLOCKED_DECREMENT).len(), 2);
    }

    #[test]
    fn register_streamables_registers_every_target_name() {
        let mut e = Engine::new();
        let log = Log::default();
        let base = Rc::new(Cell::new(0u32));
        let answer = base.clone();
        e.register_double(
            NI_OBJECT_REGISTER_STREAMABLES,
            move |_, _| ret(answer.get()),
        );
        logged(&mut e, &log, 0x00ee_3100, 1);
        let vtable = e.mem.alloc(0x40);
        e.mem
            .set_u32(vtable + SLOT_REGISTER_FIXED_STRING, 0x00ee_3100);
        let stream = e.mem.alloc(8);
        e.mem.set_u32(stream, vtable);
        let this = data(&mut e, 2, 1);
        assert!(!e.call(0x00a4_1710, &args![this, stream]).bool());
        base.set(1);
        assert!(e.call(0x00a4_1710, &args![this, stream]).bool());
        let targets = e.mem.u32(this + MORPH_TARGETS);
        assert_eq!(
            calls(&log, 0x00ee_3100),
            [vec![stream, targets + 4], vec![stream, targets + 0x14]]
        );
    }

    /// A stream whose output stream records each write.
    fn output_stream(e: &mut Engine, writes: &Writes) -> u32 {
        let record = writes.clone();
        e.register_double(0x00ee_3200, move |e, a| {
            let bytes = e.mem.bytes(a[1], a[2]);
            record.borrow_mut().push((bytes, e.mem.u32(a[3]), a[4]));
            ret(a[2])
        });
        let output = e.mem.alloc(0x10);
        e.mem.set_u32(output + BINARY_STREAM_WRITE, 0x00ee_3200);
        let stream = e.mem.alloc(0x260);
        e.mem.set_u32(stream + STREAM_OUTPUT, output);
        stream
    }

    #[test]
    fn fn_00a417b0_writes_the_floats_in_one_write() {
        let mut e = Engine::new();
        let writes = Writes::default();
        let stream = output_stream(&mut e, &writes);
        let floats = e.mem.alloc(8);
        e.mem.set_f32(floats, 1.0);
        e.mem.set_f32(floats + 4, 2.0);
        e.call(0x00a4_17b0, &args![stream, floats, 2u32]);
        let mut bytes = 1.0f32.to_le_bytes().to_vec();
        bytes.extend(2.0f32.to_le_bytes());
        assert_eq!(*writes.borrow(), [(bytes, 4, 1)]);
        let output = e.mem.u32(stream + STREAM_OUTPUT);
        assert_eq!(e.mem.u32(output + BINARY_STREAM_POSITION), 8);
    }

    #[test]
    fn morph_target_is_equal_compares_vertices_and_names() {
        let mut e = Engine::new();
        let a = data(&mut e, 1, 2);
        let b = data(&mut e, 1, 2);
        let ta = e.mem.u32(a + MORPH_TARGETS);
        let tb = e.mem.u32(b + MORPH_TARGETS);
        assert!(e.call(0x00a4_17f0, &args![ta, tb, 2u32]).bool());
        let vb = e.mem.u32(tb);
        e.mem.set_f32(vb + 0x14, 9.0);
        assert!(!e.call(0x00a4_17f0, &args![ta, tb, 2u32]).bool());
        // Only the first vertex compared.
        assert!(e.call(0x00a4_17f0, &args![ta, tb, 1u32]).bool());
        e.mem.set_u32(tb + TARGET_NAME, 0x999);
        assert!(!e.call(0x00a4_17f0, &args![ta, tb, 0u32]).bool());
        // A NaN never equals itself.
        e.mem.set_u32(tb + TARGET_NAME, 0x100);
        let va = e.mem.u32(ta);
        e.mem.set_f32(va, f32::NAN);
        e.mem.set_f32(vb, f32::NAN);
        assert!(!e.call(0x00a4_17f0, &args![ta, tb, 1u32]).bool());
    }

    #[test]
    fn is_equal_checks_counts_flags_and_targets() {
        let mut e = Engine::new();
        let base = Rc::new(Cell::new(1u32));
        let answer = base.clone();
        e.register_double(NI_OBJECT_IS_EQUAL, move |_, _| ret(answer.get()));
        let a = data(&mut e, 2, 1);
        let b = data(&mut e, 2, 1);
        assert!(e.call(0x00a4_18a0, &args![a, b]).bool());
        e.mem.set_u8(b + RELATIVE_TARGETS, 1);
        assert!(!e.call(0x00a4_18a0, &args![a, b]).bool());
        e.mem.set_u8(b + RELATIVE_TARGETS, 0);
        let tb = e.mem.u32(b + MORPH_TARGETS);
        e.mem.set_u32(tb + 0x10 + TARGET_NAME, 0x555);
        assert!(!e.call(0x00a4_18a0, &args![a, b]).bool());
        e.mem.set_u32(b + NUM_VERTS_PER_TARGET, 2);
        assert!(!e.call(0x00a4_18a0, &args![a, b]).bool());
        e.mem.set_u32(b + NUM_TARGETS, 1);
        assert!(!e.call(0x00a4_18a0, &args![a, b]).bool());
        base.set(0);
        assert!(!e.call(0x00a4_18a0, &args![a, a]).bool());
    }

    #[test]
    fn save_binary_writes_counts_flag_names_and_vertices() {
        let mut e = Engine::new();
        let log = Log::default();
        logged(&mut e, &log, NI_OBJECT_SAVE_BINARY, 0);
        logged(&mut e, &log, SAVE_FIXED_STRING, 0);
        let writes = Writes::default();
        let stream = output_stream(&mut e, &writes);
        let this = data(&mut e, 2, 1);
        e.mem.set_u8(this + RELATIVE_TARGETS, 1);
        e.call(0x00a4_1d90, &args![this, stream]);
        let targets = e.mem.u32(this + MORPH_TARGETS);
        assert_eq!(calls(&log, NI_OBJECT_SAVE_BINARY), [vec![this, stream]]);
        assert_eq!(
            calls(&log, SAVE_FIXED_STRING),
            [vec![stream, targets + 4], vec![stream, targets + 0x14]]
        );
        let vertex: Vec<u8> = [0.0f32, 1.0, 2.0]
            .iter()
            .flat_map(|v| v.to_le_bytes())
            .collect();
        assert_eq!(
            *writes.borrow(),
            [
                (vec![2, 0, 0, 0], 4, 1),
                (vec![1, 0, 0, 0], 4, 1),
                (vec![1], 1, 1),
                (vertex.clone(), 4, 1),
                (vertex, 4, 1),
            ]
        );
        // Without vertices only the names are written.
        e.mem.set_u32(this + NUM_VERTS_PER_TARGET, 0);
        e.call(0x00a4_1d90, &args![this, stream]);
        assert_eq!(writes.borrow().len(), 8);
    }

    #[test]
    fn get_viewer_strings_lists_counts_and_names() {
        let mut e = Engine::new();
        let log = Log::default();
        e.map(NI_MORPH_DATA_RTTI, 4);
        e.mem.set_u32(NI_MORPH_DATA_RTTI, 0x0155_0000);
        logged(&mut e, &log, NI_OBJECT_GET_VIEWER_STRINGS, 0);
        logged(&mut e, &log, NI_GET_VIEWER_STRING, 0x0166_0000);
        logged(&mut e, &log, NI_GET_VIEWER_STRING_UNSIGNED, 0x0177_0000);
        e.register(NI_GET_VIEWER_STRING_FIXED, |_, a| ret(a[1] << 4));
        logged(&mut e, &log, VIEWER_STRINGS_SET_AT, 0);
        let grows = log.clone();
        e.register_double(VIEWER_STRINGS_GROW, move |e, a| {
            grows.borrow_mut().push((VIEWER_STRINGS_GROW, a.to_vec()));
            // The grown array holds three entries already.
            e.mem.set_u16(a[0] + 8, 8);
            e.mem.set_u16(a[0] + 0xa, 3);
            Ret::default()
        });
        let this = data(&mut e, 2, 1);
        let strings = e.mem.alloc(0x10);
        let slots = e.mem.alloc(0x10);
        e.mem.set_u32(strings + 4, slots);
        e.mem.set_u16(strings + 8, 1);
        e.mem.set_u16(strings + 0xe, 2);
        // Slot 1 is already used, so filling it keeps the used count.
        e.mem.set_u32(slots + 4, 0x77);
        e.call(0x00a4_1ea0, &args![this, strings]);
        assert_eq!(
            calls(&log, NI_OBJECT_GET_VIEWER_STRINGS),
            [vec![this, strings]]
        );
        assert_eq!(
            calls(&log, NI_GET_VIEWER_STRING),
            [vec![0x0155_0000], vec![TEXT_MORPH_TARGETS]]
        );
        assert_eq!(
            calls(&log, NI_GET_VIEWER_STRING_UNSIGNED),
            [
                vec![TEXT_NUM_TARGETS, 2],
                vec![TEXT_NUM_VERTS_PER_TARGET, 1]
            ]
        );
        assert_eq!(calls(&log, VIEWER_STRINGS_SET_AT).len(), 4);
        assert_eq!(calls(&log, VIEWER_STRINGS_GROW), [vec![strings, 3]]);
        assert_eq!(e.mem.u32(slots), 0x1000);
        assert_eq!(e.mem.u32(slots + 4), 0x1010);
        assert_eq!(e.mem.u16(strings + 0xa), 3);
        assert_eq!(e.mem.u16(strings + 0xc), 1);
    }

    #[test]
    fn morph_target_register_streamables_registers_the_name() {
        let mut e = Engine::new();
        let log = Log::default();
        logged(&mut e, &log, 0x00ee_3300, 1);
        let vtable = e.mem.alloc(0x40);
        e.mem
            .set_u32(vtable + SLOT_REGISTER_FIXED_STRING, 0x00ee_3300);
        let stream = e.mem.alloc(8);
        e.mem.set_u32(stream, vtable);
        assert!(e.call(0x00a5_7850, &args![0x7000u32, stream]).bool());
        assert_eq!(calls(&log, 0x00ee_3300), [vec![stream, 0x7004]]);
    }
}

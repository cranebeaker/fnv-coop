//! `NiMain/nitexturingproperty.cpp` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00a6ab90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTexturingProperty::_scalar_deleting_destructor_` (Xbox PDB): runs the destructor body at `00a6a0b0`,
/// then the sized delete (`00aa1460`, `(this, size)`) of 0x30 bytes when bit 0 of
/// `flags` is set. Answers `this`.
pub fn ni_texturing_property_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    const DESTRUCTOR_BODY: u32 = 0x00a6_a0b0;
    const SIZED_DELETE: u32 = 0x00aa_1460;
    e.call(DESTRUCTOR_BODY, &args![this]);
    if flags & 1 != 0 {
        e.call(SIZED_DELETE, &args![this, 0x30u32]);
    }
    this
}

use crate::units::nimain::nimainsdm::{assign_ref_counted, set_literal_fixed_string, ALLOCATE};

/// `011f49ac`: the pointer `00a6b410` stores the object it builds in.
const TEXTURING_PROPERTY_SLOT: u32 = 0x011f_49ac;
/// `00a6aa40` (`thiscall`, unit `nitexturingproperty.cpp`): constructs the object in the
/// block and answers it.
const TEXTURING_PROPERTY_CONSTRUCTOR: u32 = 0x00a6_aa40;
/// The static fixed strings `00a6b410` sets: (global, address of the literal), in order.
const TEXTURE_SLOT_NAMES: [(u32, u32); 10] = [
    (0x011f_49b0, 0x0108_03dc),
    (0x011f_49b4, 0x0103_c800),
    (0x011f_49b8, 0x0109_c5e4),
    (0x011f_49bc, 0x0109_c5dc),
    (0x011f_49c0, 0x0109_c5d4),
    (0x011f_49c4, 0x0109_c5cc),
    (0x011f_49c8, 0x0101_c29c),
    (0x011f_49cc, 0x0109_c5c0),
    (0x011f_49d0, 0x0101_e47c),
    (0x011f_49d4, 0x0109_c5b8),
];

// Translated from 00a6b410 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTexturingProperty::_SDMInit` (Xbox PDB; run by the start-up `00a89cc0`, no
/// arguments): allocates 0x30 bytes (`00aa13e0`), constructs a texturing property in them
/// with `00a6aa40` and stores the object it answers, counted, in `011f49ac`, releasing the
/// object held there before (a failed allocation stores null); then sets ten static fixed
/// strings from literals (`TEXTURE_SLOT_NAMES`), each only when the global does not hold
/// the literal's address already.
pub fn ni_texturing_property_sdm_init(e: &mut Engine) {
    let block = e.call(ALLOCATE, &args![0x30u32]).u32();
    let object = if block == 0 {
        0
    } else {
        e.call(TEXTURING_PROPERTY_CONSTRUCTOR, &args![block]).u32()
    };
    assign_ref_counted(e, TEXTURING_PROPERTY_SLOT, object);
    for (slot, literal) in TEXTURE_SLOT_NAMES {
        set_literal_fixed_string(e, slot, literal);
    }
}

/// `0096ae90` (`thiscall`, `RET 8`, no unit in the engine map): called with the part at
/// `+0x1c`, an index and the address of a word holding 0.
const SET_ELEMENT: u32 = 0x0096_ae90;
/// `00aa1070` (`cdecl`, one word): allocates a block of that size.
const ALLOCATE_BLOCK: u32 = 0x00aa_1070;

// Translated from 00a6aa40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `nitexturingproperty.cpp`; the body of a constructor,
/// `thiscall`, no stack word): runs the base constructor `00a5c200`, sets the vtable
/// `0109c514` and the part at `this + 0x1c` (vtable `0109c394`, then `0109c5ac`, count
/// words 9, 0, 0 and 1, a 0x24-byte buffer from `00aa1070` at `+0x20`), clears the word at
/// `+0x2c`, calls `0096ae90` for the indexes 0 to 8 with a word holding 0, then sets the
/// flag word at `+0x18`: bit 0 when one of the entries of the buffer from index 1 up to
/// the count at `+0x26` is non-zero, bit 2 always, and bits 1 and 3 cleared.
pub fn fn_00a6aa40(e: &mut Engine, this: Ptr) -> Ptr {
    let object = this.addr();
    e.call(
        crate::units::nimain::nimainsdm::OBJECT_BASE_CONSTRUCTOR,
        &args![this],
    );
    e.mem.set_u32(object, 0x0109_c514);
    let part = object + 0x1c;
    e.mem.set_u16(object + 0x18, 0);
    e.mem.set_u16(part + 8, 9);
    e.mem.set_u32(part, 0x0109_c394);
    e.mem.set_u16(part + 0xe, 1);
    e.mem.set_u16(part + 0xa, 0);
    e.mem.set_u16(part + 0xc, 0);
    let buffer = e.call(ALLOCATE_BLOCK, &args![0x24u32]).u32();
    e.mem.set_u32(part + 4, buffer);
    e.mem.set_u32(part, 0x0109_c5ac);
    e.mem.set_u32(object + 0x2c, 0);
    for index in 0..9u32 {
        e.with_stack(4, |e, local| {
            e.mem.set_u32(local.addr(), 0);
            e.call(SET_ELEMENT, &args![part, index, local]);
        });
    }
    let mut flags = u32::from(e.mem.u16(object + 0x18) & 0xf00f);
    e.mem.set_u16(object + 0x18, flags as u16);
    flags = u32::from(e.mem.u16(object + 0x18));
    let count = u32::from(e.mem.u16(object + 0x26));
    let mut found = false;
    if count > 1 {
        let mut slot = e.mem.u32(object + 0x20).wrapping_add(4);
        let mut index = 1u32;
        loop {
            if e.mem.u32(slot) != 0 {
                found = true;
                break;
            }
            index += 1;
            slot = slot.wrapping_add(4);
            if index >= count {
                break;
            }
        }
    }
    if found {
        flags |= 1;
    } else {
        flags &= 0xfffe;
    }
    let word = (flags as u16 & 0xfff5) | 4;
    e.mem.set_u16(object + 0x18, flags as u16);
    e.mem.set_u16(object + 0x18, word);
    this
}

// ---- Batch b012: maps, cloning, loading and linking ------------------------------------

use crate::types::NiTArray;
use crate::units::nimain::nimainsdm::{release_fixed_string, GET_FIXED_STRING};

/// `NiTexturingProperty` (Xbox PDB) fields the code below uses: the flag word `m_uFlags`
/// (`+0x18`), the map array `m_kMaps` (an `NiTPrimitiveArray<Map *>`, `+0x1c`, its
/// elements at `+0x20`) and `m_pkShaderMaps` (a pointer to another such array, `+0x2c`).
const FLAGS: u32 = 0x18;
const MAPS: u32 = 0x1c;
const SHADER_MAPS: u32 = 0x2c;
/// The vtables: `NiTexturingProperty` (`0109c514`), its map array (`0109c394`, then
/// `0109c5ac`), a shader-map array (`0109c5b4`), `NiTexturingProperty::Map` (`0109c4c4`)
/// and the map kinds made for slot 5 (`0109c4d8`), slot 7 (`0109c500`) and the shader maps
/// (`0109c4ec`).
const TEXTURING_PROPERTY_VTABLE: u32 = 0x0109_c514;
const MAP_ARRAY_BASE_VTABLE: u32 = 0x0109_c394;
const MAP_ARRAY_VTABLE: u32 = 0x0109_c5ac;
const SHADER_MAP_ARRAY_VTABLE: u32 = 0x0109_c5b4;
const MAP_VTABLE: u32 = 0x0109_c4c4;
const SLOT_5_MAP_VTABLE: u32 = 0x0109_c4d8;
const SLOT_7_MAP_VTABLE: u32 = 0x0109_c500;
const SHADER_MAP_VTABLE: u32 = 0x0109_c4ec;
/// `NiTexturingProperty::GetRTTI` answers this record (`011f49a4`).
const TEXTURING_PROPERTY_RTTI: u32 = 0x011f_49a4;
/// The memory manager object (`011f6238`) and its `thiscall` allocate (`00aa3e40`, one
/// word) and free (`00aa4060`, one word).
const MEMORY_MANAGER: u32 = 0x011f_6238;
const MEMORY_ALLOCATE: u32 = 0x00aa_3e40;
const MEMORY_FREE: u32 = 0x00aa_4060;
/// `00aa10f0` (`cdecl`, one word): frees a block from `00aa1070`.
const FREE_BLOCK: u32 = 0x00aa_10f0;
/// `00a5c240` (`thiscall`, unit `niobjectnet.cpp`): the `NiObjectNET` destructor body.
const OBJECT_NET_DESTRUCTOR: u32 = 0x00a5_c240;
/// `00a5d450` (`thiscall`, `RET 8`, unit `niobject.cpp`); `00a5d7a0` (`thiscall`, `RET 8`, a
/// jump to `00a5c290`): the member copies `CreateClone` and `00a6bb00` run first.
const OBJECT_COPY_MEMBERS: u32 = 0x00a5_d450;
const OBJECT_NET_COPY_MEMBERS: u32 = 0x00a5_d7a0;
/// `00a90180` (`thiscall`, `RET 4`; engine-map unit `nioldscreenspace.cpp`, a shared
/// body): sets the capacity of an `NiTPrimitiveArray`.
const ARRAY_SET_SIZE: u32 = 0x00a9_0180;
/// `NiSourceTexture::Create_ov2` (`00a5ff10`, `cdecl`, three words).
const SOURCE_TEXTURE_CREATE: u32 = 0x00a5_ff10;
/// `00a88eb0` (`thiscall`, `RET 0x14`): builds a texture transform from a translation, a
/// rotation (a float word), a scale, a centre and a method; `00a88f00` (`thiscall`,
/// `RET 4`): loads one from the stream.
const TEXTURE_TRANSFORM_CONSTRUCTOR: u32 = 0x00a8_8eb0;
const TEXTURE_TRANSFORM_LOAD_BINARY: u32 = 0x00a8_8f00;
/// The two floats `00a69b00` copies as a new transform's translation (`011f4980`) and the
/// float its centre starts at (`01016248`).
const ZERO_POINT_2: u32 = 0x011f_4980;
const TRANSFORM_CENTRE: u32 = 0x0101_6248;
/// `NiProperty::LoadBinary` (`00a5d7f0`, `RET 4`), `00a645b0` (`thiscall` on the stream),
/// `NiStream::SetNumberOfLinkIDs` (`00a64700`, engine-map name, `RET 4`),
/// `NiStream::GetNumberOfLinkIDs` (`00a63460`) and `NiStream::GetObjectFromLinkID`
/// (`00a63430`), both `thiscall` on the stream without a stack word.
const PROPERTY_LOAD_BINARY: u32 = 0x00a5_d7f0;
const STREAM_00A645B0: u32 = 0x00a6_45b0;
const SET_NUMBER_OF_LINK_IDS: u32 = 0x00a6_4700;
const GET_NUMBER_OF_LINK_IDS: u32 = 0x00a6_3460;
const GET_OBJECT_FROM_LINK_ID: u32 = 0x00a6_3430;
/// `00a47360` (`thiscall`, `RET 4`; engine map: `NiSequenceStreamHelper::LinkObject`, a body
/// shared by folding): the base link step.
const BASE_LINK_OBJECT: u32 = 0x00a4_7360;
/// Constructors in this unit outside the batch: the slot-5 map (`00a69fb0`, 0x28 bytes), the
/// slot-7 map (`00a6a010`, 0x14 bytes) and the shader-map array (`00a6a990`, `RET 8`:
/// capacity and grow step).
const SLOT_5_MAP_CONSTRUCTOR: u32 = 0x00a6_9fb0;
const SLOT_7_MAP_CONSTRUCTOR: u32 = 0x00a6_a010;
const SHADER_MAP_ARRAY_CONSTRUCTOR: u32 = 0x00a6_a990;
/// `InterlockedIncrement` / `InterlockedDecrement` import slots.
const INTERLOCKED_INCREMENT: u32 = 0x00fd_f1c0;
const INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;

/// `MemoryManager::Allocate` on the manager object (`00aa3e40`).
fn memory_allocate(e: &mut Engine, size: u32) -> u32 {
    e.call(MEMORY_ALLOCATE, &args![MEMORY_MANAGER, size]).u32()
}

/// `InterlockedDecrement` of the count at `object + 4`; `DeleteThis` (slot `+4`) at zero.
fn release(e: &mut Engine, object: u32) {
    if e.call(INTERLOCKED_DECREMENT, &args![object + 4]).i32() == 0 {
        e.vcall(object, 4, &[]);
    }
}

/// `NiPointer::operator=` on the holder at `holder`.
fn assign(e: &mut Engine, holder: u32, value: u32) {
    let old = e.mem.u32(holder);
    if old != value {
        if old != 0 {
            release(e, old);
        }
        e.mem.set_u32(holder, value);
        if value != 0 {
            e.call(INTERLOCKED_INCREMENT, &args![value + 4]);
        }
    }
}

/// One read through the stream's reader (`stream + 0x24c`): the function pointer at the
/// reader's `+8`, `cdecl (reader, buffer, size, count, 1)` with the word at `count` set to
/// `size` first; the answer is added to the reader's word at `+4`.
fn stream_read(e: &mut Engine, stream: u32, buffer: u32, size: u32, count: u32) {
    let reader = e.mem.u32(stream + 0x24c);
    let read_function = e.mem.u32(reader + 8);
    e.mem.set_u32(count, size);
    let read = e
        .call(read_function, &args![reader, buffer, size, count, 1u32])
        .u32();
    let total = e.mem.u32(reader + 4).wrapping_add(read);
    e.mem.set_u32(reader + 4, total);
}

/// The inlined `NiTPrimitiveArray::SetAt` after its capacity check: past the size the size
/// becomes `index + 1` (and a non-null value is counted); inside it the count of non-null
/// elements follows the change; then the element is stored.
fn array_set_counted(e: &mut Engine, array: Ptr<NiTArray>, index: u32, value: u32) {
    let size = u32::from(e.get(array, NiTArray::m_usSize));
    let used = e.get(array, NiTArray::m_usESize);
    if index >= size {
        e.set(array, NiTArray::m_usSize, index.wrapping_add(1) as u16);
        if value != 0 {
            e.set(array, NiTArray::m_usESize, used.wrapping_add(1));
        }
    } else {
        let current = e.mem.u32(e.get(array, NiTArray::m_pBase) + 4 * index);
        if value != 0 {
            if current == 0 {
                e.set(array, NiTArray::m_usESize, used.wrapping_add(1));
            }
        } else if current != 0 {
            e.set(array, NiTArray::m_usESize, used.wrapping_add(0xffff));
        }
    }
    let base = e.get(array, NiTArray::m_pBase);
    e.mem.set_u32(base + 4 * index, value);
}

/// The capacity check before a `SetAt` at `index`: grown to `grow step + index` when
/// `index` is not below the capacity (`00a90180`).
fn array_reserve(e: &mut Engine, array: Ptr<NiTArray>, index: u32) {
    if index >= u32::from(e.get(array, NiTArray::m_usMaxSize)) {
        let step = u32::from(e.get(array, NiTArray::m_usGrowBy));
        e.call(ARRAY_SET_SIZE, &args![array, step.wrapping_add(index)]);
    }
}

/// The map count in bits 4..11 of the flag word, plus one.
fn count_one_more_map(e: &mut Engine, this: u32) {
    let flags = u32::from(e.mem.u16(this + FLAGS));
    let count = ((flags >> 4) & 0xff) + 1;
    e.mem
        .set_u16(this + FLAGS, ((count << 4) | (flags & 0xf00f)) as u16);
}

// Translated from 00a69dd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map; by its vtable (`0109c4c4`) the `NiTexturingProperty::Map`
/// default constructor (`ECX` only): flags cleared then set to `(flags & 0xf100) | 0x3100`,
/// no texture and no transform. Answers `this` (`MOV EAX,ECX` first, 00a69dd0).
pub fn fn_00a69dd0(e: &mut Engine, this: Ptr) -> Ptr {
    let map = this.addr();
    e.mem.set_u16(map + 4, 0);
    e.mem.set_u32(map, MAP_VTABLE);
    e.mem.set_u32(map + 8, 0);
    let flags = e.mem.u16(map + 4);
    e.mem.set_u32(map + 0xc, 0);
    e.mem.set_u16(map + 4, (flags & 0xf100) | 0x3100);
    this
}

// Translated from 00a69e00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map; the `NiTexturingProperty::Map` constructor from its parts
/// (`thiscall`, `RET 0x14`): the texture (counted), then the flag word
/// `((((filter << 4) | uv_set) << 8) | (flags & 0xc0ff)) & 0xff00 | clamp` (16 bits; the
/// flags cleared before) and the transform. Answers `this`.
pub fn fn_00a69e00(
    e: &mut Engine,
    this: Ptr,
    texture: u32,
    clamp: u32,
    filter: u32,
    uv_set: u32,
    transform: u32,
) -> Ptr {
    let map = this.addr();
    e.mem.set_u16(map + 4, 0);
    e.mem.set_u32(map, MAP_VTABLE);
    e.mem.set_u32(map + 8, texture);
    if texture != 0 {
        e.call(INTERLOCKED_INCREMENT, &args![texture + 4]);
    }
    let high = ((filter << 4) | uv_set) << 8;
    let old = e.mem.u16(map + 4);
    e.mem.set_u32(map + 0xc, transform);
    let flags = ((high as u16 | (old & 0xc0ff)) & 0xff00) | clamp as u16;
    e.mem.set_u16(map + 4, flags);
    this
}

// Translated from 00a69b00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `nitexturingproperty.cpp`; `ECX` only, answers `this`);
/// its fields match `NiTextureTransform` (Xbox PDB; `m_eMethod` at `+0x44` on PC): the
/// translation from the two words at `011f4980`, rotation 0, scale 1 and 1, centre twice
/// the float at `01016248` (through the x87 stack), the dirty byte 1 and the method 2.
pub fn fn_00a69b00(e: &mut Engine, this: Ptr) -> Ptr {
    let at = this.addr();
    let x = e.mem.u32(ZERO_POINT_2);
    e.mem.set_u32(at, x);
    let y = e.mem.u32(ZERO_POINT_2 + 4);
    e.mem.set_u32(at + 8, x87_store32(0.0));
    e.mem.set_u32(at + 4, y);
    e.mem.set_u32(at + 0xc, x87_store32(1.0));
    e.mem.set_u32(at + 0x10, x87_store32(1.0));
    let centre = x87_quiet_bits(e.mem.u32(TRANSFORM_CENTRE));
    e.mem.set_u32(at + 0x14, centre);
    e.mem.set_u32(at + 0x18, centre);
    e.mem.set_u8(at + 0x1c, 1);
    e.mem.set_u32(at + 0x44, 2);
    this
}

// Translated from 00a6a000 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: a virtual slot (referenced only from the vtables at
/// `010973f0`, `01097bd0` and `01099658`; `this` in `ECX`, not read) that answers 3.
pub fn fn_00a6a000(_e: &mut Engine, _this: Ptr) -> u32 {
    3
}

// Translated from 00a6a140 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTexturingProperty::GetRTTI` (Xbox PDB): the record at `011f49a4`.
pub fn ni_texturing_property_get_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(TEXTURING_PROPERTY_RTTI)
}

// Translated from 00a6ad40 (decompiled, FalloutNV.exe 1.4.0.525)
/// No function in Ghidra's project (read from the bytes; the class factory, `cdecl`, no
/// argument): allocates 0x30 bytes (`00aa13e0`) and runs the constructor `00a6aa40` on
/// them (a jump: its answer is the answer); a failed allocation answers null.
pub fn fn_00a6ad40(e: &mut Engine) -> Ptr {
    let block = e.call(ALLOCATE, &args![0x30u32]).u32();
    if block == 0 {
        return Ptr::NULL;
    }
    fn_00a6aa40(e, Ptr::new(block))
}

// Translated from 00a6a0b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map; the `NiTexturingProperty` destructor body (`ECX` only): sets
/// the vtable, deletes every map of the map array (their slot 0 with 1; the count at `+0x26`
/// read again each time), then, with a shader-map array, every shader map and the array
/// itself (slot 0 with 1); the map array's vtable becomes `0109c394`, its elements are freed
/// (`00aa10f0`) and the `NiObjectNET` destructor body (`00a5c240`) ends it.
pub fn fn_00a6a0b0(e: &mut Engine, this: Ptr) {
    let at = this.addr();
    e.mem.set_u32(at, TEXTURING_PROPERTY_VTABLE);
    let mut index = 0u32;
    while index < u32::from(e.mem.u16(at + 0x26)) {
        let map = e.mem.u32(e.mem.u32(at + 0x20) + 4 * index);
        if map != 0 {
            e.vcall(map, 0, &args![1u32]);
        }
        index += 1;
    }
    if e.mem.u32(at + SHADER_MAPS) != 0 {
        let mut index = 0u32;
        while index < u32::from(e.mem.u16(e.mem.u32(at + SHADER_MAPS) + 0xa)) {
            let base = e.mem.u32(e.mem.u32(at + SHADER_MAPS) + 4);
            let map = e.mem.u32(base + 4 * index);
            if map != 0 {
                e.vcall(map, 0, &args![1u32]);
            }
            index += 1;
        }
        let array = e.mem.u32(at + SHADER_MAPS);
        if array != 0 {
            e.vcall(array, 0, &args![1u32]);
        }
    }
    let elements = e.mem.u32(at + 0x20);
    e.mem.set_u32(at + MAPS, MAP_ARRAY_BASE_VTABLE);
    e.call(FREE_BLOCK, &args![elements]);
    e.call(OBJECT_NET_DESTRUCTOR, &args![this]);
}

// Translated from 00a6a6f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `nitexturingproperty.cpp`; the `_SDMShutdown` of
/// `00a6b410`, no argument): releases the default texturing property held at `011f49ac`
/// and clears it, then each of the ten static fixed strings that is not null gets the
/// handle `00a5b690(0)` and its old handle is released (unless it is the empty string's).
pub fn fn_00a6a6f0(e: &mut Engine) {
    let object = e.mem.u32(TEXTURING_PROPERTY_SLOT);
    if object != 0 {
        release(e, object);
        e.mem.set_u32(TEXTURING_PROPERTY_SLOT, 0);
    }
    for (slot, _) in TEXTURE_SLOT_NAMES {
        let old = e.mem.u32(slot);
        if old != 0 {
            let handle = e.call(GET_FIXED_STRING, &args![0u32]).u32();
            e.mem.set_u32(slot, handle);
            release_fixed_string(e, old);
        }
    }
}

// Translated from 00a6a8b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map; the `NiTexturingProperty::Map` copy constructor (`thiscall`,
/// `RET 4`): the vtable, the flag word and the texture (counted) of `other`; with a
/// transform in `other` a 0x48-byte block from the memory manager is built by `00a88eb0`
/// from a copy of its translation, rotation (through the x87 stack), scale, centre and
/// method, and becomes the transform (null when the allocation fails or `other` has none).
/// Answers `this`.
pub fn fn_00a6a8b0(e: &mut Engine, this: Ptr, other: Ptr) -> Ptr {
    let (map, source) = (this.addr(), other.addr());
    e.mem.set_u32(map, MAP_VTABLE);
    let flags = e.mem.u16(source + 4);
    e.mem.set_u16(map + 4, flags);
    let texture = e.mem.u32(source + 8);
    e.mem.set_u32(map + 8, texture);
    if texture != 0 {
        e.call(INTERLOCKED_INCREMENT, &args![texture + 4]);
    }
    if e.mem.u32(source + 0xc) == 0 {
        e.mem.set_u32(map + 0xc, 0);
        return this;
    }
    let block = memory_allocate(e, 0x48);
    if block == 0 {
        e.mem.set_u32(map + 0xc, 0);
        return this;
    }
    let transform = e.mem.u32(source + 0xc);
    let made = e.with_stack(0x18, |e, locals| {
        // The centre, the scale and the translation, copied as words.
        let (centre, scale, translate) = (locals.addr(), locals.addr() + 8, locals.addr() + 0x10);
        for (to, from) in [(centre, 0x14), (scale, 0xc), (translate, 0)] {
            let first = e.mem.u32(transform + from);
            let second = e.mem.u32(transform + from + 4);
            e.mem.set_u32(to, first);
            e.mem.set_u32(to + 4, second);
        }
        let method = e.mem.u32(transform + 0x44);
        let rotation = x87_quiet_bits(e.mem.u32(transform + 8));
        e.call(
            TEXTURE_TRANSFORM_CONSTRUCTOR,
            &args![block, translate, rotation, scale, centre, method],
        )
        .u32()
    });
    e.mem.set_u32(map + 0xc, made);
    this
}

// Translated from 00a6a9f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map; the `NiTexturingProperty::Map` scalar deleting destructor
/// (`thiscall`, `RET 4`): the vtable, the transform freed through the memory manager
/// (`00aa4060`), the texture released, then the map itself freed the same way when bit 0
/// of `flags` is set. Answers `this`.
pub fn fn_00a6a9f0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    let map = this.addr();
    let transform = e.mem.u32(map + 0xc);
    e.mem.set_u32(map, MAP_VTABLE);
    e.call(MEMORY_FREE, &args![MEMORY_MANAGER, transform]);
    let texture = e.mem.u32(map + 8);
    if texture != 0 {
        release(e, texture);
    }
    if flags & 1 != 0 {
        e.call(MEMORY_FREE, &args![MEMORY_MANAGER, this]);
    }
    this
}

// Translated from 00a6abb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map; the `NiTexturingProperty` constructor from a texture file
/// (`thiscall`, `RET 0xc`): the same set-up as `00a6aa40`, then the texture
/// `NiSourceTexture::Create_ov2(a1, *a2, a3)` (`00a5ff10`) in a new map (0x10 bytes from the
/// memory manager, `00a69e00` with clamp 0, filter 3, UV set 1 and no transform; null when
/// the allocation fails) at index 0 and nothing at the indexes 1 to 8 (`0096ae90`), then
/// the flag word as `00a6aa40` leaves it. Answers `this`.
pub fn fn_00a6abb0(e: &mut Engine, this: Ptr, a1: u32, a2: Ptr, a3: u32) -> Ptr {
    let object = this.addr();
    e.call(
        crate::units::nimain::nimainsdm::OBJECT_BASE_CONSTRUCTOR,
        &args![this],
    );
    e.mem.set_u32(object, TEXTURING_PROPERTY_VTABLE);
    let part = object + MAPS;
    e.mem.set_u16(object + FLAGS, 0);
    e.mem.set_u16(part + 8, 9);
    e.mem.set_u32(part, MAP_ARRAY_BASE_VTABLE);
    e.mem.set_u16(part + 0xe, 1);
    e.mem.set_u16(part + 0xa, 0);
    e.mem.set_u16(part + 0xc, 0);
    let buffer = e.call(ALLOCATE_BLOCK, &args![0x24u32]).u32();
    e.mem.set_u32(part + 4, buffer);
    e.mem.set_u32(part, MAP_ARRAY_VTABLE);
    e.mem.set_u32(object + SHADER_MAPS, 0);
    let name = e.mem.u32(a2.addr());
    let texture = e.call(SOURCE_TEXTURE_CREATE, &args![a1, name, a3]).u32();
    let block = memory_allocate(e, 0x10);
    let map = if block == 0 {
        0
    } else {
        fn_00a69e00(e, Ptr::new(block), texture, 0, 3, 1, 0).addr()
    };
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), map);
        e.call(SET_ELEMENT, &args![part, 0u32, slot]);
        for index in 1..9u32 {
            e.mem.set_u32(slot.addr(), 0);
            e.call(SET_ELEMENT, &args![part, index, slot]);
        }
    });
    let count = u32::from(e.mem.u16(object + 0x26));
    let masked = e.mem.u16(object + FLAGS) & 0xf00f;
    e.mem.set_u16(object + FLAGS, masked);
    let mut flags = e.mem.u16(object + FLAGS);
    let mut found = false;
    let mut index = 1u32;
    while index < count {
        if e.mem.u32(e.mem.u32(object + 0x20) + 4 * index) != 0 {
            found = true;
            break;
        }
        index += 1;
    }
    if found {
        flags |= 1;
    } else {
        flags &= 0xfffe;
    }
    e.mem.set_u16(object + FLAGS, flags);
    e.mem.set_u16(object + FLAGS, (flags & 0xfff5) | 4);
    this
}

// Translated from 00a6c610 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTexturingProperty::CreateClone` (Xbox PDB; `thiscall`, `RET 4`): without controllers
/// (`+0x0c` null) the property is its own clone: `00a5d450(this, this, cloning)` and `this`
/// answered. Otherwise a new property (0x30 bytes from `00aa13e0`, `00a6aa40`) gets the
/// members (`00a6bb00`, also run with a null clone when the allocation fails) and is
/// answered.
pub fn ni_texturing_property_create_clone(e: &mut Engine, this: Ptr, cloning: u32) -> Ptr {
    if e.mem.u32(this.addr() + 0xc) == 0 {
        e.call(OBJECT_COPY_MEMBERS, &args![this, this, cloning]);
        return this;
    }
    let block = e.call(ALLOCATE, &args![0x30u32]).u32();
    let clone = if block == 0 {
        Ptr::NULL
    } else {
        fn_00a6aa40(e, Ptr::new(block))
    };
    fn_00a6bb00(e, this, clone, cloning);
    clone
}

// Translated from 00a6bb00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map; the `NiTexturingProperty` member copy (`thiscall`, `RET 8`):
/// the `NiObjectNET` copy (`00a5d7a0`), the flag word, the capacity of the map array of
/// `dest` set to the count of `this` (`00a90180`), then for each non-null map of `this`
/// a copy (`00a6a8b0`) set into `dest` at the same index (`0096ae90`, after growing the
/// array when the index is not below its capacity): slot 5 as a 0x28-byte map (vtable
/// `0109c4d8`, six more floats at `+0x10..+0x24` through the x87 stack), slot 7 as a
/// 0x14-byte map (vtable `0109c500`, one float at `+0x10`), the others as plain 0x10-byte
/// maps (all from the memory manager; a failed allocation sets null). With shader maps in
/// `this`, `dest` gets an empty shader-map array when it has none, its capacity set to the
/// count of `this`, and a 0x14-byte copy (vtable `0109c4ec`, the word at `+0x10`) of each
/// non-null shader map set at the same index (grown first, counted inline).
pub fn fn_00a6bb00(e: &mut Engine, this: Ptr, dest: Ptr, cloning: u32) {
    let (from, to) = (this.addr(), dest.addr());
    e.call(OBJECT_NET_COPY_MEMBERS, &args![this, dest, cloning]);
    let flags = e.mem.u16(from + FLAGS);
    e.mem.set_u16(to.wrapping_add(FLAGS), flags);
    let count = u32::from(e.mem.u16(from + 0x26));
    let array: Ptr<NiTArray> = Ptr::new(to.wrapping_add(MAPS));
    e.call(ARRAY_SET_SIZE, &args![array, count]);
    e.with_stack(8, |e, slots| {
        // The local the slot-5 and slot-7 copies go through, and the stack slot of the
        // `cloning` argument, which holds the index and the plain copies.
        let (local, index_slot) = (slots.addr(), slots.addr() + 4);
        e.mem.set_u32(index_slot, 0);
        let mut index = 0u32;
        while index < count {
            if e.mem.u32(e.mem.u32(from + 0x20) + 4 * index) != 0 {
                match index {
                    5 => {
                        let block = memory_allocate(e, 0x28);
                        if block == 0 {
                            e.mem.set_u32(local, 0);
                        } else {
                            let source = e.mem.u32(e.mem.u32(from + 0x20) + 0x14);
                            fn_00a6a8b0(e, Ptr::new(block), Ptr::new(source));
                            e.mem.set_u32(block, SLOT_5_MAP_VTABLE);
                            for at in (0x10..0x28).step_by(4) {
                                let value = x87_quiet_bits(e.mem.u32(source + at));
                                e.mem.set_u32(block + at, value);
                                if at == 0x10 {
                                    e.mem.set_u32(local, block);
                                }
                            }
                        }
                        array_reserve(e, array, 5);
                        e.call(SET_ELEMENT, &args![array, 5u32, local]);
                    }
                    7 => {
                        let block = memory_allocate(e, 0x14);
                        if block == 0 {
                            e.mem.set_u32(local, 0);
                        } else {
                            let source = e.mem.u32(e.mem.u32(from + 0x20) + 0x1c);
                            fn_00a6a8b0(e, Ptr::new(block), Ptr::new(source));
                            e.mem.set_u32(block, SLOT_7_MAP_VTABLE);
                            let value = x87_quiet_bits(e.mem.u32(source + 0x10));
                            e.mem.set_u32(block + 0x10, value);
                            e.mem.set_u32(local, block);
                        }
                        array_reserve(e, array, 7);
                        e.call(SET_ELEMENT, &args![array, 7u32, local]);
                    }
                    _ => {
                        let block = memory_allocate(e, 0x10);
                        let copy = if block == 0 {
                            0
                        } else {
                            let source = e.mem.u32(e.mem.u32(from + 0x20) + 4 * index);
                            fn_00a6a8b0(e, Ptr::new(block), Ptr::new(source)).addr()
                        };
                        e.mem.set_u32(index_slot, copy);
                        array_reserve(e, array, index);
                        e.call(SET_ELEMENT, &args![array, index, index_slot]);
                    }
                }
            }
            index += 1;
            e.mem.set_u32(index_slot, index);
        }
    });
    if e.mem.u32(from + SHADER_MAPS) == 0 {
        return;
    }
    if e.mem.u32(to.wrapping_add(SHADER_MAPS)) == 0 {
        let block = memory_allocate(e, 0x10);
        if block != 0 {
            e.mem.set_u16(block + 0xe, 1);
            e.mem.set_u16(block + 8, 0);
            e.mem.set_u16(block + 0xa, 0);
            e.mem.set_u16(block + 0xc, 0);
            e.mem.set_u32(block + 4, 0);
            e.mem.set_u32(block, SHADER_MAP_ARRAY_VTABLE);
        }
        e.mem.set_u32(to.wrapping_add(SHADER_MAPS), block);
    }
    let count = u32::from(e.mem.u16(e.mem.u32(from + SHADER_MAPS) + 0xa));
    let target = e.mem.u32(to.wrapping_add(SHADER_MAPS));
    e.call(ARRAY_SET_SIZE, &args![target, count]);
    for index in 0..count {
        let base = e.mem.u32(e.mem.u32(from + SHADER_MAPS) + 4);
        if e.mem.u32(base + 4 * index) == 0 {
            continue;
        }
        let block = memory_allocate(e, 0x14);
        let copy = if block == 0 {
            0
        } else {
            let base = e.mem.u32(e.mem.u32(from + SHADER_MAPS) + 4);
            let source = e.mem.u32(base + 4 * index);
            fn_00a6a8b0(e, Ptr::new(block), Ptr::new(source));
            e.mem.set_u32(block, SHADER_MAP_VTABLE);
            let word = e.mem.u32(source + 0x10);
            e.mem.set_u32(block + 0x10, word);
            block
        };
        let target: Ptr<NiTArray> = Ptr::new(e.mem.u32(to.wrapping_add(SHADER_MAPS)));
        array_reserve(e, target, index);
        array_set_counted(e, target, index, copy);
    }
}

// Translated from 00a6b640 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTexturingProperty::Map::LoadBinary` (Xbox PDB; `thiscall`, `RET 4`): `00a645b0` on
/// the stream, then by stream version (`+0xd8`): before `0x14010002` three 4-byte words
/// whose low bits become the flag word's bits 12-13 (`<< 12`, mask `0xcfff`), 8-11
/// (`<< 8`, mask `0xf0ff`) and 0-7 (mask `0xff00`); from it the flag word itself (2 bytes).
/// Before `0x0a030004` two 2-byte words and before `0x04010010` one more are read and
/// dropped. From `0x0a00010a` the transform is cleared and one byte read: when it is 1 a
/// transform (0x48 bytes from the memory manager, `00a69b00`; null when that fails) is made
/// and loaded (`00a88f00`). Every read goes through the reader's function pointer; the
/// stack slot of the stream argument serves as count or buffer as in the exe.
pub fn ni_texturing_property_map_load_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    let (map, source) = (this.addr(), stream.addr());
    e.call(STREAM_00A645B0, &args![stream]);
    let version = |e: &Engine| e.mem.u32(source + 0xd8);
    e.with_stack(0xc, |e, frame| {
        // Two locals and the stack slot of the stream argument (holding the pointer).
        let (first, second, argument) = (frame.addr(), frame.addr() + 4, frame.addr() + 8);
        e.mem.set_u32(argument, source);
        if version(e) < 0x1401_0002 {
            stream_read(e, source, first, 4, argument);
            let value = e.mem.u32(first);
            let flags = (e.mem.u16(map + 4) & 0xcfff) | (value << 12) as u16;
            e.mem.set_u16(map + 4, flags);
            stream_read(e, source, first, 4, argument);
            let value = e.mem.u32(first);
            let flags = (e.mem.u16(map + 4) & 0xf0ff) | (value << 8) as u16;
            e.mem.set_u16(map + 4, flags);
            stream_read(e, source, second, 4, argument);
            let flags = (e.mem.u16(map + 4) & 0xff00) | e.mem.u16(second);
            e.mem.set_u16(map + 4, flags);
        } else {
            stream_read(e, source, map + 4, 2, argument);
        }
        if version(e) < 0x0a03_0004 {
            stream_read(e, source, argument, 2, second);
            stream_read(e, source, argument, 2, second);
        }
        if version(e) < 0x0401_0010 {
            stream_read(e, source, argument, 2, second);
        }
        if version(e) >= 0x0a00_010a {
            e.mem.set_u32(map + 0xc, 0);
            stream_read(e, source, argument, 1, second);
            if e.mem.u8(argument) == 1 {
                let block = memory_allocate(e, 0x48);
                let transform = if block == 0 {
                    0
                } else {
                    fn_00a69b00(e, Ptr::new(block)).addr()
                };
                e.mem.set_u32(map + 0xc, transform);
                e.call(TEXTURE_TRANSFORM_LOAD_BINARY, &args![transform, stream]);
            }
        }
    });
}

// Translated from 00a6a240 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTexturingProperty::LinkObject` (Xbox PDB; `thiscall`, `RET 4`): the base link step
/// (`00a47360`), then the map count (flag bits 4-11) cleared; for each of the stream's link
/// ids (`GetNumberOfLinkIDs`, asked again each time; nothing for an id when it answers 0)
/// the map at that index gets the linked object as its texture (`GetObjectFromLinkID`,
/// counted), and from index 8 on the map count goes up by one. Streams before
/// `0x14020005` then move the old slot-6 and slot-7 maps behind the others: with either
/// present and a map count, the maps from the last one down to index 8 move up by the
/// number present (`0096ae90` with the address of the element, after growing the array);
/// the slot-6 map goes to index 8 (index 6 cleared, map count + 1) or index 8 is cleared;
/// the slot-7 map goes to index 9 (index 7 cleared, map count + 1), or index 9 is cleared
/// when two were present (which cannot happen without a slot-7 map; kept as in the exe).
/// Streams from `0x05000011` link the shader maps' textures the same way.
pub fn ni_texturing_property_link_object(e: &mut Engine, this: Ptr, stream: Ptr) {
    let at = this.addr();
    e.call(BASE_LINK_OBJECT, &args![this, stream]);
    let links = e.call(GET_NUMBER_OF_LINK_IDS, &args![stream]).u32();
    let masked = e.mem.u16(at + FLAGS) & 0xf00f;
    e.mem.set_u16(at + FLAGS, masked);
    for index in 0..links {
        if e.call(GET_NUMBER_OF_LINK_IDS, &args![stream]).u32() == 0 {
            continue;
        }
        let slot = e.mem.u32(at + 0x20).wrapping_add(4 * index);
        let object = e.call(GET_OBJECT_FROM_LINK_ID, &args![stream]).u32();
        let map = e.mem.u32(slot);
        assign(e, map + 8, object);
        if index >= 8 {
            count_one_more_map(e, at);
        }
    }
    if e.mem.u32(stream.addr() + 0xd8) < 0x1402_0005 {
        e.with_stack(0xc, |e, frame| {
            let (sixth, seventh, cleared) = (frame.addr(), frame.addr() + 4, frame.addr() + 8);
            let size = e.mem.u16(at + 0x26);
            e.mem.set_u32(sixth, 0);
            e.mem.set_u32(seventh, 0);
            if size > 6 {
                let map = e.mem.u32(e.mem.u32(at + 0x20) + 0x18);
                e.mem.set_u32(sixth, map);
            }
            if size > 7 {
                let map = e.mem.u32(e.mem.u32(at + 0x20) + 0x1c);
                e.mem.set_u32(seventh, map);
            }
            let (has_sixth, has_seventh) = (e.mem.u32(sixth) != 0, e.mem.u32(seventh) != 0);
            if !has_sixth && !has_seventh {
                return;
            }
            let present = u32::from(has_sixth) + u32::from(has_seventh);
            let array: Ptr<NiTArray> = Ptr::new(at + MAPS);
            if e.mem.u16(at + FLAGS) & 0xff0 != 0 && present != 0 {
                let mut from = u32::from(size).wrapping_sub(1);
                if from >= 8 {
                    let mut to = from.wrapping_add(present);
                    loop {
                        let element = e.mem.u32(at + 0x20).wrapping_add(4 * from);
                        array_reserve(e, array, to);
                        e.call(SET_ELEMENT, &args![array, to, element]);
                        from = from.wrapping_sub(1);
                        to = to.wrapping_sub(1);
                        if from < 8 {
                            break;
                        }
                    }
                }
            }
            if has_sixth {
                array_reserve(e, array, 8);
                e.call(SET_ELEMENT, &args![array, 8u32, sixth]);
                e.mem.set_u32(cleared, 0);
                e.call(SET_ELEMENT, &args![array, 6u32, cleared]);
                count_one_more_map(e, at);
            } else {
                e.mem.set_u32(cleared, 0);
                e.call(SET_ELEMENT, &args![array, 8u32, cleared]);
            }
            if has_seventh {
                array_reserve(e, array, 9);
                e.call(SET_ELEMENT, &args![array, 9u32, seventh]);
                e.mem.set_u32(cleared, 0);
                e.call(SET_ELEMENT, &args![array, 7u32, cleared]);
                count_one_more_map(e, at);
            } else if present == 2 {
                e.mem.set_u32(cleared, 0);
                e.call(SET_ELEMENT, &args![array, 9u32, cleared]);
            }
        });
    }
    if e.mem.u32(stream.addr() + 0xd8) >= 0x0500_0011 {
        let links = e.call(GET_NUMBER_OF_LINK_IDS, &args![stream]).u32();
        for index in 0..links {
            if e.call(GET_NUMBER_OF_LINK_IDS, &args![stream]).u32() == 0 {
                continue;
            }
            let base = e.mem.u32(e.mem.u32(at + SHADER_MAPS) + 4);
            let slot = base.wrapping_add(4 * index);
            let object = e.call(GET_OBJECT_FROM_LINK_ID, &args![stream]).u32();
            let map = e.mem.u32(slot);
            assign(e, map + 8, object);
        }
    }
}

// Translated from 00a6ad60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTexturingProperty::LoadBinary` (Xbox PDB; `thiscall`, `RET 4`):
/// `NiProperty::LoadBinary` (`00a5d7f0`); the flag word (before stream version
/// `0x14010002` a 4-byte word put in bits 1-3, `(flags & 0xfff1) | word * 2`; from it the
/// 2-byte word itself, then masked with `0xf00f`); the map count (4 bytes), the map array's
/// capacity set to it (plus 2 for streams before `0x14020005` with more than 6) and
/// `SetNumberOfLinkIDs(count)` (`+1` before `0x0303000d`). For each map a presence flag (a
/// 4-byte word tested for non-zero before `0x04010000`, a byte from it) goes to
/// `SetNumberOfLinkIDs`; a present map is made and loaded (its slot `+4` with the stream; a
/// failed allocation still calls through null, as the exe does). Before `0x0303000d` the
/// maps from index 5 on are stored one index higher, after a `SetNumberOfLinkIDs(0)`.
/// From `0x0303000d` index 5 is a 0x28-byte map (`00a69fb0`) and, from `0x14020005`, index 7
/// a 0x14-byte map (`00a6a010`), both set with `0096ae90`; every other index is a plain
/// 0x10-byte map (initialized inline as `00a69dd0` does) stored inline (capacity checked,
/// counted). From `0x05000011` the shader maps follow: their count (4 bytes, to
/// `SetNumberOfLinkIDs`), a new shader-map array (`00a6a990(count, 1)`, null when the
/// allocation fails) and for each a presence byte and a 0x14-byte shader map (vtable
/// `0109c4ec`) loaded the same way and stored inline (no capacity check). Last, bit 0 of
/// the flag word tells whether a map from index 1 on is present.
pub fn ni_texturing_property_load_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    let (at, source) = (this.addr(), stream.addr());
    e.call(PROPERTY_LOAD_BINARY, &args![this, stream]);
    let version = |e: &Engine| e.mem.u32(source + 0xd8);
    let flags_at = at + FLAGS;
    let array: Ptr<NiTArray> = Ptr::new(at + MAPS);
    e.with_stack(0x14, |e, frame| {
        // The locals of the exe's frame: the map being made, the count, a scratch word and
        // a count slot, then the stack slot of the stream argument (holding the pointer).
        let made = frame.addr();
        let count_word = frame.addr() + 4;
        let scratch = frame.addr() + 8;
        let count_slot = frame.addr() + 0xc;
        let argument = frame.addr() + 0x10;
        e.mem.set_u32(argument, source);
        if version(e) < 0x1401_0002 {
            stream_read(e, source, scratch, 4, argument);
            let value = e.mem.u32(scratch);
            let flags = (e.mem.u16(flags_at) & 0xfff1) | value.wrapping_add(value) as u16;
            e.mem.set_u16(flags_at, flags);
        } else {
            stream_read(e, source, flags_at, 2, argument);
            let flags = e.mem.u16(flags_at) & 0xf00f;
            e.mem.set_u16(flags_at, flags);
        }
        stream_read(e, source, count_word, 4, argument);
        let count = e.mem.u32(count_word);
        let capacity = if version(e) < 0x1402_0005 && count > 6 {
            count.wrapping_add(2)
        } else {
            count
        };
        e.call(ARRAY_SET_SIZE, &args![array, capacity]);
        let links = if version(e) < 0x0303_000d {
            e.mem.u32(count_word).wrapping_add(1)
        } else {
            e.mem.u32(count_word)
        };
        e.call(SET_NUMBER_OF_LINK_IDS, &args![stream, links]);
        let mut index = 0u32;
        while index < e.mem.u32(count_word) {
            let mut slot = index;
            e.mem.set_u32(made, 0);
            let present = if version(e) < 0x0401_0000 {
                stream_read(e, source, made, 4, count_slot);
                let present = u8::from(e.mem.u32(made) != 0);
                e.mem.set_u8(argument, present);
                present
            } else {
                stream_read(e, source, argument, 1, count_slot);
                e.mem.u8(argument)
            };
            let current = version(e);
            let mut plain = true;
            if current < 0x0303_000d {
                if index >= 5 {
                    e.call(SET_NUMBER_OF_LINK_IDS, &args![stream, 0u32]);
                    slot = index + 1;
                }
            } else if index == 5 || (index == 7 && current >= 0x1402_0005) {
                plain = false;
                e.call(SET_NUMBER_OF_LINK_IDS, &args![stream, u32::from(present)]);
                if e.mem.u8(argument) != 0 {
                    let (size, constructor) = if index == 5 {
                        (0x28, SLOT_5_MAP_CONSTRUCTOR)
                    } else {
                        (0x14, SLOT_7_MAP_CONSTRUCTOR)
                    };
                    let block = memory_allocate(e, size);
                    let map = if block == 0 {
                        0
                    } else {
                        e.call(constructor, &args![block]).u32()
                    };
                    e.mem.set_u32(made, map);
                    e.vcall(map, 4, &args![stream]);
                }
                e.call(SET_ELEMENT, &args![array, index, made]);
            }
            if plain {
                e.call(SET_NUMBER_OF_LINK_IDS, &args![stream, u32::from(present)]);
                if e.mem.u8(argument) != 0 {
                    let block = memory_allocate(e, 0x10);
                    if block != 0 {
                        e.mem.set_u16(block + 4, 0);
                        e.mem.set_u32(block, MAP_VTABLE);
                        e.mem.set_u32(block + 8, 0);
                        e.mem.set_u32(block + 0xc, 0);
                        let flags = (e.mem.u16(block + 4) & 0xf100) | 0x3100;
                        e.mem.set_u16(block + 4, flags);
                    }
                    e.mem.set_u32(made, block);
                    e.vcall(block, 4, &args![stream]);
                }
                array_reserve(e, array, slot);
                let map = e.mem.u32(made);
                array_set_counted(e, array, slot, map);
            }
            index += 1;
        }
        if version(e) >= 0x0500_0011 {
            stream_read(e, source, count_word, 4, argument);
            let count = e.mem.u32(count_word);
            e.call(SET_NUMBER_OF_LINK_IDS, &args![stream, count]);
            if count != 0 {
                let block = memory_allocate(e, 0x10);
                let shader_maps = if block == 0 {
                    0
                } else {
                    e.call(SHADER_MAP_ARRAY_CONSTRUCTOR, &args![block, count, 1u32])
                        .u32()
                };
                e.mem.set_u32(at + SHADER_MAPS, shader_maps);
                let mut index = 0u32;
                while index < e.mem.u32(count_word) {
                    stream_read(e, source, argument, 1, count_slot);
                    let present = u32::from(e.mem.u8(argument));
                    e.call(SET_NUMBER_OF_LINK_IDS, &args![stream, present]);
                    let mut map = 0u32;
                    if e.mem.u8(argument) != 0 {
                        let block = memory_allocate(e, 0x14);
                        if block != 0 {
                            e.mem.set_u16(block + 4, 0);
                            e.mem.set_u32(block + 8, 0);
                            let flags = (e.mem.u16(block + 4) & 0xf100) | 0x3100;
                            e.mem.set_u32(block + 0xc, 0);
                            e.mem.set_u16(block + 4, flags);
                            e.mem.set_u32(block, SHADER_MAP_VTABLE);
                            e.mem.set_u32(block + 0x10, 0);
                        }
                        map = block;
                        e.vcall(map, 4, &args![stream]);
                    }
                    let target: Ptr<NiTArray> = Ptr::new(e.mem.u32(at + SHADER_MAPS));
                    array_set_counted(e, target, index, map);
                    index += 1;
                }
            }
        }
    });
    let count = u32::from(e.mem.u16(at + 0x26));
    let mut index = 1u32;
    while index < count {
        if e.mem.u32(e.mem.u32(at + 0x20) + 4 * index) != 0 {
            let flags = e.mem.u16(flags_at) | 1;
            e.mem.set_u16(flags_at, flags);
            return;
        }
        index += 1;
    }
    let flags = e.mem.u16(flags_at) & 0xfffe;
    e.mem.set_u16(flags_at, flags);
}

/// `NiTexturingProperty::Map` fields (Xbox PDB, PC offsets): the flag word `m_usFlags`
/// (+0x04: UV set in bits 0-7, filter in bits 8-11, clamp in bits 12-13), `m_spTexture`
/// (+0x08) and `m_pkTextureTransform` (+0x0C); the word or float a derived map adds at
/// +0x10; the bump map's six floats at +0x10..+0x24.
const MAP_FLAGS: u32 = 0x04;
const MAP_TEXTURE: u32 = 0x08;
const MAP_TRANSFORM: u32 = 0x0c;
const MAP_EXTRA: u32 = 0x10;
/// The property's map array (`m_kMaps`, an `NiTArray` at +0x1C): `m_pBase` +0x20,
/// `m_usMaxSize` +0x24, `m_usSize` +0x26.
const MAP_ELEMENTS: u32 = 0x20;
const MAP_CAPACITY: u32 = 0x24;
const MAP_COUNT: u32 = 0x26;
/// `NiNew`'s companions: `NiStricmp` (`00aa1360`, cdecl: two strings; 0 when equal) and
/// `NiSprintf` (`00a305b0`, cdecl: buffer, size, format, then the format words).
const NI_STRICMP: u32 = 0x00aa_1360;
const NI_SPRINTF: u32 = 0x00a3_05b0;
/// The base `RegisterStreamables` (`00a62130`), `IsEqual` (`00a62140`) and `SaveBinary`
/// (`00a5d7b0`): five-byte bodies the engine map names after `NiSequenceStreamHelper`
/// (folded); `NiProperty::GetViewerStrings` (`00a5d850`). All thiscall, one word.
const BASE_REGISTER_STREAMABLES_00A62130: u32 = 0x00a6_2130;
const BASE_IS_EQUAL_00A62140: u32 = 0x00a6_2140;
const BASE_SAVE_BINARY_00A5D7B0: u32 = 0x00a5_d7b0;
const PROPERTY_GET_VIEWER_STRINGS: u32 = 0x00a5_d850;
/// The transform helpers: the difference test `00a88c60` (unnamed; thiscall on the
/// transform, the other; nonzero when they differ) and `SaveBinary` `00a88f80` (unnamed;
/// thiscall on the transform, the stream).
const TRANSFORM_NOT_EQUAL_00A88C60: u32 = 0x00a8_8c60;
const TRANSFORM_SAVE_BINARY_00A88F80: u32 = 0x00a8_8f80;
/// The stream slot +0x30 (`SaveLinkID` in the Xbox PDB order) and the texture slots
/// `RegisterStreamables` +0x54 and `IsEqual` +0x5C; the map slot +0x08 (`SaveBinary`).
const STREAM_SAVE_LINK_ID_SLOT: u32 = 0x30;
const TEXTURE_REGISTER_STREAMABLES_SLOT: u32 = 0x54;
const TEXTURE_IS_EQUAL_SLOT: u32 = 0x5c;
const MAP_SAVE_BINARY_SLOT: u32 = 0x08;
/// The viewer-string helpers: `NiGetViewerString` (cdecl, the class name),
/// `NiGetViewerString_ov7` (a name and an `int`), `_ov8` (a name and an unsigned value),
/// `_ov9` (a name and a `float`) and `_ov10` (a name and a pointer); the strings array
/// `SetSize` (`0096ad30`) and its add `0096a610` (unnamed; thiscall on the array, the
/// address of the element).
const VIEWER_STRING_NAME: u32 = 0x00a7_d330;
const VIEWER_STRING_SIGNED: u32 = 0x00a7_d4d0;
const VIEWER_STRING_UNSIGNED: u32 = 0x00a7_d510;
const VIEWER_STRING_FLOAT: u32 = 0x00a7_d550;
const VIEWER_STRING_POINTER: u32 = 0x00a7_d5a0;
const STRING_ARRAY_SET_SIZE: u32 = 0x0096_ad30;
const STRING_ARRAY_ADD_0096A610: u32 = 0x0096_a610;
/// The clamp-mode formats (`"%s = CLAMP_S_CLAMP_T"`, `"%s = CLAMP_S_WRAP_T"`,
/// `"%s = WRAP_S_CLAMP_T"`, `"%s = WRAP_S_WRAP_T"`), the filter-mode formats
/// (`"%s = FILTER_NEAREST"` ... `"%s = FILTER_MAGLERP_MINANISO"`) and `"%s = UNKNOWN!!!"`.
const CLAMP_FORMATS: [u32; 4] = [0x0109_c3dc, 0x0109_c3c8, 0x0109_c3b4, 0x0109_c3a0];
const FILTER_FORMATS: [u32; 8] = [
    0x0109_c4ac,
    0x0109_c498,
    0x0109_c484,
    0x0109_c464,
    0x0109_c448,
    0x0109_c428,
    0x0109_c414,
    0x0109_c3f4,
];
const FORMAT_UNKNOWN: u32 = 0x0109_b2ec;
/// The viewer-string texts: `"ApplyMode "`, the map names of slots 0 to 7
/// (`"MultiTexture Base Map"` ... `"MultiTexture Parallax Map"`), the bump map floats
/// (`"m_fLumaScale"`, `"m_fLumaOffset"`, `"m_fBumpMat00"`, `"m_fBumpMat01"`,
/// `"m_fBumpMat10"`, `"m_fBumpMat11"`), `"m_fParallaxOffset"`,
/// `"MultiTexture Decal Map %d"`, `"m_spTexture "`, `"Clamp Mode "`, `"Filter Mode"`,
/// `"Texture Coord Index "`, `"Shader Map"` and `"m_uiID"`.
const TEXT_APPLY_MODE: u32 = 0x0109_c794;
const TEXT_MAP_NAMES: [u32; 8] = [
    0x0109_c77c,
    0x0109_c764,
    0x0109_c74c,
    0x0109_c734,
    0x0109_c71c,
    0x0109_c704,
    0x0109_c68c,
    0x0109_c670,
];
const TEXT_BUMP_FLOATS: [u32; 6] = [
    0x0109_c6f4,
    0x0109_c6e4,
    0x0109_c6d4,
    0x0109_c6c4,
    0x0109_c6b4,
    0x0109_c6a4,
];
const TEXT_PARALLAX_OFFSET: u32 = 0x0109_c65c;
const TEXT_DECAL_MAP: u32 = 0x0109_c640;
const TEXT_TEXTURE: u32 = 0x0109_c630;
const TEXT_CLAMP_MODE: u32 = 0x0109_c624;
const TEXT_FILTER_MODE: u32 = 0x0109_c618;
const TEXT_COORD_INDEX: u32 = 0x0109_c600;
const TEXT_SHADER_MAP: u32 = 0x0109_c5f4;
const TEXT_ID: u32 = 0x0109_c5ec;

/// The length of the zero-terminated string at `text` (the code inline `strlen`).
fn string_length(e: &Engine, text: u32) -> u32 {
    let mut length = 0u32;
    while e.mem.u8(text.wrapping_add(length)) != 0 {
        length = length.wrapping_add(1);
    }
    length
}

/// One write through the stream's writer (`stream + 0x250`): its function pointer at +0xC,
/// `cdecl (writer, buffer, size, count, 1)` with the word at `count` set to `size` first;
/// the answer is added to the writer's word at +4.
fn stream_write(e: &mut Engine, stream: u32, buffer: u32, size: u32, count: u32) {
    let writer = e.mem.u32(stream + 0x250);
    let write_function = e.mem.u32(writer + 0xc);
    e.mem.set_u32(count, size);
    let written = e
        .call(write_function, &args![writer, buffer, size, count, 1u32])
        .u32();
    let total = e.mem.u32(writer + 4).wrapping_add(written);
    e.mem.set_u32(writer + 4, total);
}

/// Whether two `float` words compare equal as `FLD`/`FLD`/`FUCOMPP` with `TEST AH,0x44`
/// finds them (a NaN is never equal; +0 equals -0).
fn floats_equal(a: u32, b: u32) -> bool {
    f32::from_bits(a) == f32::from_bits(b)
}

/// Adds the string word at `slot` at the end of the strings array (`SetSize` to the size
/// plus the grow-by step when the array is full, then `SetAt`).
fn add_viewer_string(e: &mut Engine, strings: Ptr<NiTArray>, slot: u32) {
    let index = u32::from(e.get(strings, NiTArray::m_usSize));
    if index >= u32::from(e.get(strings, NiTArray::m_usMaxSize)) {
        let grow = u32::from(e.get(strings, NiTArray::m_usGrowBy));
        e.call(STRING_ARRAY_SET_SIZE, &args![strings, grow + index]);
    }
    e.call(SET_ELEMENT, &args![strings, index, slot]);
}

/// Adds `text` at the end of the strings array with its `SetAt` inlined (`SetSize` when
/// full, then [`array_set_counted`] at the old size).
fn add_viewer_string_inline(e: &mut Engine, strings: Ptr<NiTArray>, text: u32) {
    let index = u32::from(e.get(strings, NiTArray::m_usSize));
    if index >= u32::from(e.get(strings, NiTArray::m_usMaxSize)) {
        let grow = u32::from(e.get(strings, NiTArray::m_usGrowBy));
        e.call(STRING_ARRAY_SET_SIZE, &args![strings, grow + index]);
    }
    array_set_counted(e, strings, index, text);
}

// Translated from 00a69b40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (thiscall, `RET 4`: a map word): sets flag bit 0 of the
/// property when the word is nonzero or any map after index 0 is set (`m_usSize` read once),
/// clears it otherwise.
pub fn fn_00a69b40(e: &mut Engine, this: Ptr, map: u32) {
    let at = this.addr();
    let mut any = map != 0;
    if !any {
        let count = u32::from(e.mem.u16(at + MAP_COUNT));
        let elements = e.mem.u32(at + MAP_ELEMENTS);
        any = (1..count).any(|i| e.mem.u32(elements + 4 * i) != 0);
    }
    let flags = e.mem.u16(at + FLAGS);
    e.mem
        .set_u16(at + FLAGS, if any { flags | 1 } else { flags & 0xfffe });
}

// Translated from 00a69b90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTexturingProperty::GetViewerString` (Xbox PDB; cdecl: a name and the clamp mode): a
/// new `NiAlloc` buffer of the name length plus 0x14, filled by `NiSprintf` with the mode
/// name for modes 0 to 3 (the code jump table at `00a69c38`) or `"%s = UNKNOWN!!!"`.
/// Returns the buffer.
pub fn ni_texturing_property_get_viewer_string(e: &mut Engine, name: u32, mode: u32) -> u32 {
    let size = string_length(e, name).wrapping_add(0x14);
    let buffer = e.call(ALLOCATE_BLOCK, &args![size]).u32();
    let format = CLAMP_FORMATS
        .get(mode as usize)
        .copied()
        .unwrap_or(FORMAT_UNKNOWN);
    e.call(NI_SPRINTF, &args![buffer, size, format, name]);
    buffer
}

// Translated from 00a69c50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTexturingProperty::GetViewerString_ov2` (Xbox PDB; cdecl: a name and the filter
/// mode): a new `NiAlloc` buffer of the name length plus 0x1E, filled by `NiSprintf` with
/// the mode name for modes 0 to 7 (the code jump table at `00a69d54`) or
/// `"%s = UNKNOWN!!!"`. Returns the buffer.
pub fn ni_texturing_property_get_viewer_string_ov2(e: &mut Engine, name: u32, mode: u32) -> u32 {
    let size = string_length(e, name).wrapping_add(0x1e);
    let buffer = e.call(ALLOCATE_BLOCK, &args![size]).u32();
    let format = FILTER_FORMATS
        .get(mode as usize)
        .copied()
        .unwrap_or(FORMAT_UNKNOWN);
    e.call(NI_SPRINTF, &args![buffer, size, format, name]);
    buffer
}

// Translated from 00a69d80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl: the address of a string pointer, an out index): finds
/// the string among the ten static fixed strings `_SDMInit` sets (`011f49b0` onwards, the
/// map slot names): the same pointer, or both non-null and equal ignoring case
/// (`NiStricmp`). Stores the index and answers true when found; false otherwise.
pub fn fn_00a69d80(e: &mut Engine, text: u32, index: u32) -> bool {
    for (i, (slot, _)) in TEXTURE_SLOT_NAMES.iter().enumerate() {
        let wanted = e.mem.u32(text);
        let known = e.mem.u32(*slot);
        let found = known == wanted
            || (wanted != 0 && known != 0 && e.call(NI_STRICMP, &args![known, wanted]).u32() == 0);
        if found {
            e.mem.set_u32(index, i as u32);
            return true;
        }
    }
    false
}

// Translated from 00a69e60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map; the body compares two `NiTexturingProperty::Map`s (thiscall,
/// `RET 4`: the other map): the textures both null or equal (the texture's slot +0x5C),
/// the same flag word, and the transforms both null or not different (`00a88c60`).
pub fn fn_00a69e60(e: &mut Engine, this: Ptr, other: Ptr) -> bool {
    let (map, theirs) = (this.addr(), other.addr());
    let texture = e.mem.u32(map + MAP_TEXTURE);
    if texture == 0 {
        if e.mem.u32(theirs + MAP_TEXTURE) != 0 {
            return false;
        }
    } else {
        let other_texture = e.mem.u32(theirs + MAP_TEXTURE);
        if e.vcall(texture, TEXTURE_IS_EQUAL_SLOT, &args![other_texture])
            .u32() as u8
            == 0
        {
            return false;
        }
    }
    if e.mem.u16(map + MAP_FLAGS) != e.mem.u16(theirs + MAP_FLAGS) {
        return false;
    }
    let transform = e.mem.u32(map + MAP_TRANSFORM);
    let other_transform = e.mem.u32(theirs + MAP_TRANSFORM);
    if transform == 0 || other_transform == 0 {
        return transform == 0 && other_transform == 0;
    }
    e.call(
        TRANSFORM_NOT_EQUAL_00A88C60,
        &args![transform, other_transform],
    )
    .u32() as u8
        == 0
}

// Translated from 00a69ed0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map; the body compares two bump maps (the slot-5 maps of vtable
/// `0109c4d8`; thiscall, `RET 4`): [`fn_00a69e60`] and the six floats at +0x10..+0x24
/// (`FUCOMPP`: a NaN differs).
pub fn fn_00a69ed0(e: &mut Engine, this: Ptr, other: Ptr) -> bool {
    if !fn_00a69e60(e, this, other) {
        return false;
    }
    (0..6).all(|i| {
        let offset = MAP_EXTRA + 4 * i;
        floats_equal(
            e.mem.u32(this.addr() + offset),
            e.mem.u32(other.addr() + offset),
        )
    })
}

// Translated from 00a69f50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map; the body compares two shader maps (vtable `0109c4ec`;
/// thiscall, `RET 4`): [`fn_00a69e60`] and the same word at +0x10.
pub fn fn_00a69f50(e: &mut Engine, this: Ptr, other: Ptr) -> bool {
    if !fn_00a69e60(e, this, other) {
        return false;
    }
    e.mem.u32(this.addr() + MAP_EXTRA) == e.mem.u32(other.addr() + MAP_EXTRA)
}

// Translated from 00a69f80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map; the body compares two slot-7 maps (vtable `0109c500`;
/// thiscall, `RET 4`): [`fn_00a69e60`] and the float at +0x10 (`FUCOMPP`: a NaN differs).
pub fn fn_00a69f80(e: &mut Engine, this: Ptr, other: Ptr) -> bool {
    fn_00a69e60(e, this, other)
        && floats_equal(
            e.mem.u32(this.addr() + MAP_EXTRA),
            e.mem.u32(other.addr() + MAP_EXTRA),
        )
}

// Translated from 00a69fb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map; by its vtable (`0109c4d8`) the slot-5 (bump) map default
/// constructor (`ECX` only): flags cleared then `(flags & 0xf100) | 0x3100`, no texture and
/// no transform, and the floats 1 (+0x10), 0 (+0x14), the float at `01016248` (+0x18), 0
/// (+0x1C), 0 (+0x20) and the float at `01016248` (+0x24), each through the x87 stack.
/// Returns `this`.
pub fn fn_00a69fb0(e: &mut Engine, this: Ptr) -> Ptr {
    let map = this.addr();
    e.mem.set_u16(map + MAP_FLAGS, 0);
    e.mem.set_u32(map + MAP_TEXTURE, 0);
    e.mem.set_u32(map + 0x10, x87_store32(1.0));
    let zero = x87_store32(0.0);
    let flags = e.mem.u16(map + MAP_FLAGS);
    e.mem.set_u32(map + 0x14, zero);
    e.mem.set_u32(map + MAP_TRANSFORM, 0);
    let half = x87_quiet_bits(e.mem.u32(TRANSFORM_CENTRE));
    e.mem.set_u32(map + 0x18, half);
    e.mem.set_u32(map + 0x24, half);
    e.mem.set_u16(map + MAP_FLAGS, (flags & 0xf100) | 0x3100);
    e.mem.set_u32(map + 0x1c, zero);
    e.mem.set_u32(map, SLOT_5_MAP_VTABLE);
    e.mem.set_u32(map + 0x20, zero);
    this
}

// Translated from 00a6a010 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map; by its vtable (`0109c500`) the slot-7 map default
/// constructor (`ECX` only): flags cleared then `(flags & 0xf100) | 0x3100`, no texture and
/// no transform, and the float at `01016248` (through the x87 stack) at +0x10. Returns
/// `this`.
pub fn fn_00a6a010(e: &mut Engine, this: Ptr) -> Ptr {
    let map = this.addr();
    let half = x87_quiet_bits(e.mem.u32(TRANSFORM_CENTRE));
    e.mem.set_u16(map + MAP_FLAGS, 0);
    e.mem.set_u32(map + MAP_TEXTURE, 0);
    e.mem.set_u32(map + 0x10, half);
    let flags = e.mem.u16(map + MAP_FLAGS);
    e.mem.set_u32(map + MAP_TRANSFORM, 0);
    e.mem.set_u16(map + MAP_FLAGS, (flags & 0xf100) | 0x3100);
    e.mem.set_u32(map, SLOT_7_MAP_VTABLE);
    this
}

/// The two map arrays' deleting destructors: the vtable given, the element block freed
/// (`00aa10f0`), the array itself given back to the memory manager (`00aa4060`) when bit 0
/// of `flags` is set.
fn delete_map_array(e: &mut Engine, this: Ptr, flags: u32, vtable: u32) -> Ptr {
    let elements = e.mem.u32(this.addr() + 4);
    e.mem.set_u32(this.addr(), vtable);
    e.call(FREE_BLOCK, &args![elements]);
    if flags & 1 != 0 {
        e.call(MEMORY_FREE, &args![MEMORY_MANAGER, this]);
    }
    this
}

// Translated from 00a6a050 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (referenced from the map-array vtables `0109c394` and
/// `0109c5ac`): the map array scalar deleting destructor (thiscall, `RET 4`) with the base
/// vtable `0109c394`. Returns `this`.
pub fn fn_00a6a050(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    delete_map_array(e, this, flags, MAP_ARRAY_BASE_VTABLE)
}

// Translated from 00a6a080 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (referenced from the vtables `0109c39c` and `0109c5b4`): the
/// shader-map array scalar deleting destructor (thiscall, `RET 4`) with the base vtable
/// `0109c39c`. Returns `this`.
pub fn fn_00a6a080(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    delete_map_array(e, this, flags, SHADER_MAP_ARRAY_BASE_VTABLE)
}

/// The shader-map array base vtable (`0109c39c`).
const SHADER_MAP_ARRAY_BASE_VTABLE: u32 = 0x0109_c39c;

/// The map count (flag bits 4-11) less one, kept to 16 bits as the code stores it.
fn count_one_less_map(e: &mut Engine, this: u32) {
    let flags = u32::from(e.mem.u16(this + FLAGS));
    let count = ((flags >> 4) & 0xff).wrapping_sub(1);
    e.mem
        .set_u16(this + FLAGS, ((count << 4) | (flags & 0xf00f)) as u16);
}

// Translated from 00a6a150 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map; the body sets the map at slot `index + 8` (thiscall, `RET 8`:
/// the index and the map; the decal slots follow the eight fixed ones). Inside the array the
/// map count (flag bits 4-11) follows a null/non-null change, the map is set (`0096ae90`
/// with the map argument's stack word) and an old map is deleted (its slot 0 with 1). Past
/// the size the array grows when the slot is not below its capacity (`00a90180`), the map
/// is set and a non-null one counted. Either way [`fn_00a69b40`] then updates flag bit 0.
pub fn fn_00a6a150(e: &mut Engine, this: Ptr, index: u32, map: u32) {
    let at = this.addr();
    let slot_index = index.wrapping_add(8);
    e.with_stack(4, |e, argument| {
        e.mem.set_u32(argument.addr(), map);
        let maps = Ptr::<NiTArray>::new(at + MAPS);
        if slot_index < u32::from(e.mem.u16(at + MAP_COUNT)) {
            let old = e.mem.u32(e.mem.u32(at + MAP_ELEMENTS) + 4 * slot_index);
            if map != 0 {
                if old == 0 {
                    count_one_more_map(e, at);
                }
            } else if old != 0 {
                count_one_less_map(e, at);
            }
            e.call(SET_ELEMENT, &args![maps, slot_index, argument]);
            if old != 0 {
                e.vcall(old, 0, &args![1u32]);
            }
            fn_00a69b40(e, this, map);
            return;
        }
        if slot_index >= u32::from(e.mem.u16(at + MAP_CAPACITY)) {
            let grow = u32::from(e.get(maps, NiTArray::m_usGrowBy));
            e.call(ARRAY_SET_SIZE, &args![maps, grow.wrapping_add(slot_index)]);
        }
        e.call(SET_ELEMENT, &args![maps, slot_index, argument]);
        let map = e.mem.u32(argument.addr());
        if map != 0 {
            count_one_more_map(e, at);
        }
        fn_00a69b40(e, this, map);
    });
}

// Translated from 00a6a560 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTexturingProperty::RegisterStreamables` (Xbox PDB; thiscall, `RET 4`): false when the
/// base `RegisterStreamables` (`00a62130`) is; otherwise the texture of every map, then of
/// every shader map, registers (its slot +0x54; counts read again each time) and the answer
/// is true.
pub fn ni_texturing_property_register_streamables(e: &mut Engine, this: Ptr, stream: Ptr) -> bool {
    if e.call(BASE_REGISTER_STREAMABLES_00A62130, &args![this, stream])
        .u32() as u8
        == 0
    {
        return false;
    }
    let at = this.addr();
    let register = |e: &mut Engine, map: u32| {
        if map != 0 {
            let texture = e.mem.u32(map + MAP_TEXTURE);
            if texture != 0 {
                e.vcall(texture, TEXTURE_REGISTER_STREAMABLES_SLOT, &args![stream]);
            }
        }
    };
    let mut i = 0u32;
    while i < u32::from(e.mem.u16(at + MAP_COUNT)) {
        let map = e.mem.u32(e.mem.u32(at + MAP_ELEMENTS) + 4 * i);
        register(e, map);
        i += 1;
    }
    if e.mem.u32(at + SHADER_MAPS) != 0 {
        let mut i = 0u32;
        while i < u32::from(e.mem.u16(e.mem.u32(at + SHADER_MAPS) + 0xa)) {
            let map = e
                .mem
                .u32(e.mem.u32(e.mem.u32(at + SHADER_MAPS) + 4) + 4 * i);
            register(e, map);
            i += 1;
        }
    }
    true
}

// Translated from 00a6a5f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTexturingProperty::IsEqual` (Xbox PDB; thiscall, `RET 4`): the base `IsEqual`
/// (`00a62140`), the same flag word and map count, each map pair both null or equal (slot 5
/// by [`fn_00a69ed0`], slot 7 by [`fn_00a69f80`], the others by [`fn_00a69e60`], each run on
/// this property's map), and the shader-map arrays both absent or of one size with each pair
/// both null or equal ([`fn_00a69f50`]).
pub fn ni_texturing_property_is_equal(e: &mut Engine, this: Ptr, other: Ptr) -> bool {
    if e.call(BASE_IS_EQUAL_00A62140, &args![this, other]).u32() as u8 == 0 {
        return false;
    }
    let (mine, theirs) = (this.addr(), other.addr());
    if e.mem.u16(mine + FLAGS) != e.mem.u16(theirs + FLAGS) {
        return false;
    }
    let count = u32::from(e.mem.u16(mine + MAP_COUNT));
    if count != u32::from(e.mem.u16(theirs + MAP_COUNT)) {
        return false;
    }
    for i in 0..count {
        let a = e.mem.u32(e.mem.u32(mine + MAP_ELEMENTS) + 4 * i);
        let b = e.mem.u32(e.mem.u32(theirs + MAP_ELEMENTS) + 4 * i);
        if a == 0 {
            if b != 0 {
                return false;
            }
            continue;
        }
        if b == 0 {
            return false;
        }
        let equal = match i {
            5 => fn_00a69ed0(e, Ptr::new(a), Ptr::new(b)),
            7 => fn_00a69f80(e, Ptr::new(a), Ptr::new(b)),
            _ => fn_00a69e60(e, Ptr::new(a), Ptr::new(b)),
        };
        if !equal {
            return false;
        }
    }
    let shader = e.mem.u32(mine + SHADER_MAPS);
    if shader == 0 {
        return e.mem.u32(theirs + SHADER_MAPS) == 0;
    }
    let other_shader = e.mem.u32(theirs + SHADER_MAPS);
    if other_shader == 0 {
        return false;
    }
    let count = u32::from(e.mem.u16(shader + 0xa));
    if count != u32::from(e.mem.u16(other_shader + 0xa)) {
        return false;
    }
    for i in 0..count {
        let a = e
            .mem
            .u32(e.mem.u32(e.mem.u32(mine + SHADER_MAPS) + 4) + 4 * i);
        let b = e
            .mem
            .u32(e.mem.u32(e.mem.u32(theirs + SHADER_MAPS) + 4) + 4 * i);
        if a == 0 {
            if b != 0 {
                return false;
            }
            continue;
        }
        if b == 0 || !fn_00a69f50(e, Ptr::new(a), Ptr::new(b)) {
            return false;
        }
    }
    true
}

// Translated from 00a6a990 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map; the shader-map array constructor (thiscall, `RET 8`: the
/// capacity and the grow step, 16 bits each): the grow step, the base vtable `0109c39c`,
/// the capacity, size and count 0, an element block of four bytes per element (`NiAlloc`;
/// none for capacity 0), then the vtable `0109c5b4`. Returns `this`.
pub fn fn_00a6a990(e: &mut Engine, this: Ptr<NiTArray>, capacity: u32, grow: u32) -> Ptr<NiTArray> {
    e.set(this, NiTArray::m_usGrowBy, grow as u16);
    e.mem.set_u32(this.addr(), SHADER_MAP_ARRAY_BASE_VTABLE);
    let capacity = capacity as u16;
    e.set(this, NiTArray::m_usMaxSize, capacity);
    e.set(this, NiTArray::m_usSize, 0);
    e.set(this, NiTArray::m_usESize, 0);
    let elements = if capacity != 0 {
        e.call(ALLOCATE_BLOCK, &args![u32::from(capacity) * 4])
            .u32()
    } else {
        0
    };
    e.set(this, NiTArray::m_pBase, elements);
    e.mem.set_u32(this.addr(), SHADER_MAP_ARRAY_VTABLE);
    this
}

// Translated from 00a6b250 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTexturingProperty::SaveBinary` (Xbox PDB; thiscall, `RET 4`): the base `SaveBinary`
/// (`00a5d7b0`), the flag word (2 bytes), the map count (4 bytes), for each map a byte
/// saying whether it is there followed by the map own `SaveBinary` (its slot +8), then the
/// shader-map count (0 without an array) and the same for each shader map. The stream
/// argument stack word holds the component size or the presence byte, as in the code.
pub fn ni_texturing_property_save_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    let (at, out) = (this.addr(), stream.addr());
    e.call(BASE_SAVE_BINARY_00A5D7B0, &args![this, stream]);
    // The three locals and the stream argument stack word.
    e.with_stack(0x10, |e, frame| {
        let (count, sizes, argument) = (frame.addr(), frame.addr() + 8, frame.addr() + 0xc);
        e.mem.set_u32(argument, out);
        stream_write(e, out, at + FLAGS, 2, argument);
        let maps = u32::from(e.mem.u16(at + MAP_COUNT));
        e.mem.set_u32(count, maps);
        stream_write(e, out, count, 4, argument);
        let save_each = |e: &mut Engine, map: u32| {
            e.mem.set_u8(argument, u8::from(map != 0));
            stream_write(e, out, argument, 1, sizes);
            if e.mem.u8(argument) != 0 {
                e.vcall(map, MAP_SAVE_BINARY_SLOT, &args![stream]);
            }
        };
        for i in 0..e.mem.u32(count) {
            let map = e.mem.u32(e.mem.u32(at + MAP_ELEMENTS) + 4 * i);
            save_each(e, map);
        }
        e.mem.set_u32(argument, 4);
        let shader = e.mem.u32(at + SHADER_MAPS);
        if shader == 0 {
            e.mem.set_u32(count, 0);
            stream_write(e, out, count, 4, argument);
            return;
        }
        e.mem.set_u32(count, u32::from(e.mem.u16(shader + 0xa)));
        stream_write(e, out, count, 4, argument);
        for i in 0..e.mem.u32(count) {
            let map = e
                .mem
                .u32(e.mem.u32(e.mem.u32(at + SHADER_MAPS) + 4) + 4 * i);
            save_each(e, map);
        }
    });
}

// Translated from 00a6b830 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTexturingProperty::Map::SaveBinary` (Xbox PDB; thiscall, `RET 4`): the link to the
/// texture (stream slot +0x30), the flag word (2 bytes), a byte saying whether there is a
/// transform (written over the low byte of the stream argument stack word, then from
/// there), then the transform own `SaveBinary` (`00a88f80`) when there is one.
pub fn ni_texturing_property_map_save_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    let (map, out) = (this.addr(), stream.addr());
    let texture = e.mem.u32(map + MAP_TEXTURE);
    e.vcall(out, STREAM_SAVE_LINK_ID_SLOT, &args![texture]);
    // The local (`PUSH ECX`) and the stream argument stack word.
    e.with_stack(8, |e, frame| {
        let (local, argument) = (frame.addr(), frame.addr() + 4);
        e.mem.set_u32(argument, out);
        stream_write(e, out, map + MAP_FLAGS, 2, local);
        let has_transform = e.mem.u32(map + MAP_TRANSFORM) != 0;
        e.mem.set_u8(argument, u8::from(has_transform));
        stream_write(e, out, argument, 1, local);
    });
    let transform = e.mem.u32(map + MAP_TRANSFORM);
    if transform != 0 {
        e.call(TRANSFORM_SAVE_BINARY_00A88F80, &args![transform, stream]);
    }
}

// Translated from 00a6b8c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTexturingProperty::BumpMap::LoadBinary` (Xbox PDB; thiscall, `RET 4`):
/// [`ni_texturing_property_map_load_binary`], then the six floats at +0x10..+0x24, four
/// bytes each (the component size in the stream argument stack word).
pub fn ni_texturing_property_bump_map_load_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    ni_texturing_property_map_load_binary(e, this, stream);
    e.with_stack(4, |e, argument| {
        for i in 0..6 {
            let field = this.addr() + MAP_EXTRA + 4 * i;
            stream_read(e, stream.addr(), field, 4, argument.addr());
        }
    });
}

// Translated from 00a6b9a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTexturingProperty::BumpMap::SaveBinary` (Xbox PDB; thiscall, `RET 4`):
/// [`ni_texturing_property_map_save_binary`], then the six floats at +0x10..+0x24, four
/// bytes each (the component size in the stream argument stack word).
pub fn ni_texturing_property_bump_map_save_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    ni_texturing_property_map_save_binary(e, this, stream);
    e.with_stack(4, |e, argument| {
        for i in 0..6 {
            let field = this.addr() + MAP_EXTRA + 4 * i;
            stream_write(e, stream.addr(), field, 4, argument.addr());
        }
    });
}

// Translated from 00a6ba80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (the `LoadBinary` slot of the slot-7 and shader-map vtables
/// `0109c500`, `0109c4ec`; thiscall, `RET 4`): [`ni_texturing_property_map_load_binary`],
/// then the word at +0x10 (4 bytes; the component size in the stream argument stack word).
pub fn fn_00a6ba80(e: &mut Engine, this: Ptr, stream: Ptr) {
    ni_texturing_property_map_load_binary(e, this, stream);
    e.with_stack(4, |e, argument| {
        stream_read(
            e,
            stream.addr(),
            this.addr() + MAP_EXTRA,
            4,
            argument.addr(),
        );
    });
}

// Translated from 00a6bac0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (the `SaveBinary` slot of the slot-7 and shader-map vtables;
/// thiscall, `RET 4`): [`ni_texturing_property_map_save_binary`], then the word at +0x10
/// (4 bytes; the component size in the stream argument stack word).
pub fn fn_00a6bac0(e: &mut Engine, this: Ptr, stream: Ptr) {
    ni_texturing_property_map_save_binary(e, this, stream);
    e.with_stack(4, |e, argument| {
        stream_write(
            e,
            stream.addr(),
            this.addr() + MAP_EXTRA,
            4,
            argument.addr(),
        );
    });
}

/// The shared tail of each map's viewer strings in [`ni_texturing_property_get_viewer_strings`]:
/// `"m_spTexture "`, `"Clamp Mode "`, `"Filter Mode"` and `"Texture Coord Index "`, each
/// formatted into the 64-byte buffer `buffer` first, then the string made from it and the
/// map's field added (`add` stores it).
fn map_strings(e: &mut Engine, map: u32, buffer: u32, add: &mut dyn FnMut(&mut Engine, u32)) {
    e.call(NI_SPRINTF, &args![buffer, 0x40u32, TEXT_TEXTURE]);
    let texture = e.mem.u32(map + MAP_TEXTURE);
    let text = e.call(VIEWER_STRING_POINTER, &args![buffer, texture]).u32();
    add(e, text);
    e.call(NI_SPRINTF, &args![buffer, 0x40u32, TEXT_CLAMP_MODE]);
    let clamp = (u32::from(e.mem.u16(map + MAP_FLAGS)) >> 12) & 3;
    let text = ni_texturing_property_get_viewer_string(e, buffer, clamp);
    add(e, text);
    e.call(NI_SPRINTF, &args![buffer, 0x40u32, TEXT_FILTER_MODE]);
    let filter = u32::from(e.mem.u8(map + MAP_FLAGS + 1)) & 0xf;
    let text = ni_texturing_property_get_viewer_string_ov2(e, buffer, filter);
    add(e, text);
    e.call(NI_SPRINTF, &args![buffer, 0x40u32, TEXT_COORD_INDEX]);
    let coordinates = u32::from(e.mem.u16(map + MAP_FLAGS)) & 0xff;
    let text = e
        .call(VIEWER_STRING_UNSIGNED, &args![buffer, coordinates])
        .u32();
    add(e, text);
}

// Translated from 00a6bdd0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00a6bdd0 - the /GS stack cookie (011c16bc, __security_check_cookie 00ec408c) is not translated
/// `NiTexturingProperty::GetViewerStrings` (Xbox PDB; thiscall, `RET 4`):
/// `NiProperty::GetViewerStrings`, the class name, `"ApplyMode "` (formatted into a local
/// buffer) with flag bits 1-3; then for each map present its name (slots 0 to 7 by the code
/// jump table at `00a6c5f0`, added through `0096a610`; the bump map also its six floats and
/// the slot-7 map its float, pushed through `FLD`/`FSTP`; past slot 7
/// `"MultiTexture Decal Map %d"` formatted with the slot less 8 and added through
/// `SetSize`/`SetAt`) and the map strings (texture, clamp, filter, coordinate index); then
/// for each shader map present `"Shader Map"` with its index, `"m_uiID"` with its word at
/// +0x10 and the map strings, all added with the array `SetAt` inlined. The locals are
/// kept as in the code: the class-name and count words, the result word, three 64-byte
/// buffers.
pub fn ni_texturing_property_get_viewer_strings(e: &mut Engine, this: Ptr, strings: Ptr<NiTArray>) {
    let at = this.addr();
    e.call(PROPERTY_GET_VIEWER_STRINGS, &args![this, strings]);
    // Locals: the result word, the count word, the saved `this`, then the three buffers.
    e.with_stack(0xcc, |e, frame| {
        let result = frame.addr();
        let count_word = frame.addr() + 4;
        let texture_buffer = frame.addr() + 0xc;
        let apply_buffer = frame.addr() + 0x4c;
        let decal_buffer = frame.addr() + 0x8c;
        let class_name = e.global::<u32>(TEXTURING_PROPERTY_RTTI);
        let text = e.call(VIEWER_STRING_NAME, &args![class_name]).u32();
        e.mem.set_u32(count_word, text);
        add_viewer_string(e, strings, count_word);
        let count = u32::from(e.mem.u16(at + MAP_COUNT));
        e.call(NI_SPRINTF, &args![apply_buffer, 0x40u32, TEXT_APPLY_MODE]);
        e.mem.set_u32(count_word, count);
        let apply = (u32::from(e.mem.u16(at + FLAGS)) >> 1) & 7;
        let text = e
            .call(VIEWER_STRING_SIGNED, &args![apply_buffer, apply])
            .u32();
        e.mem.set_u32(result, text);
        add_viewer_string(e, strings, result);
        let mut add_through_set_at = |e: &mut Engine, text: u32| {
            e.mem.set_u32(result, text);
            add_viewer_string(e, strings, result);
        };
        let mut i = 0u32;
        while i < e.mem.u32(count_word) {
            let map = e.mem.u32(e.mem.u32(at + MAP_ELEMENTS) + 4 * i);
            if map != 0 {
                if i <= 7 {
                    let text = e
                        .call(
                            VIEWER_STRING_UNSIGNED,
                            &args![TEXT_MAP_NAMES[i as usize], i],
                        )
                        .u32();
                    e.mem.set_u32(result, text);
                    e.call(STRING_ARRAY_ADD_0096A610, &args![strings, result]);
                    let floats: &[(u32, u32)] = match i {
                        5 => &[
                            (TEXT_BUMP_FLOATS[0], 0x10),
                            (TEXT_BUMP_FLOATS[1], 0x14),
                            (TEXT_BUMP_FLOATS[2], 0x18),
                            (TEXT_BUMP_FLOATS[3], 0x1c),
                            (TEXT_BUMP_FLOATS[4], 0x20),
                            (TEXT_BUMP_FLOATS[5], 0x24),
                        ],
                        7 => &[(TEXT_PARALLAX_OFFSET, 0x10)],
                        _ => &[],
                    };
                    for (name, offset) in floats {
                        // `FLD`/`FSTP` into the result word, then again onto the stack.
                        let value = x87_quiet_bits(e.mem.u32(map + offset));
                        e.mem.set_u32(result, value);
                        let pushed = x87_quiet_bits(e.mem.u32(result));
                        let text = e.call(VIEWER_STRING_FLOAT, &args![*name, pushed]).u32();
                        e.mem.set_u32(result, text);
                        e.call(STRING_ARRAY_ADD_0096A610, &args![strings, result]);
                    }
                } else {
                    e.call(
                        NI_SPRINTF,
                        &args![decal_buffer, 0x40u32, TEXT_DECAL_MAP, i.wrapping_sub(8)],
                    );
                    let text = e
                        .call(VIEWER_STRING_UNSIGNED, &args![decal_buffer, i])
                        .u32();
                    add_through_set_at(e, text);
                }
                map_strings(e, map, texture_buffer, &mut add_through_set_at);
            }
            i += 1;
        }
        let shader = e.mem.u32(at + SHADER_MAPS);
        if shader == 0 {
            return;
        }
        let shader_count = u32::from(e.mem.u16(shader + 0xa));
        e.mem.set_u32(count_word, shader_count);
        e.mem.set_u32(result, 0);
        let mut add_inline = |e: &mut Engine, text: u32| add_viewer_string_inline(e, strings, text);
        while e.mem.u32(result) < e.mem.u32(count_word) {
            let index = e.mem.u32(result);
            let map = e
                .mem
                .u32(e.mem.u32(e.mem.u32(at + SHADER_MAPS) + 4) + 4 * index);
            if map != 0 {
                let text = e
                    .call(VIEWER_STRING_UNSIGNED, &args![TEXT_SHADER_MAP, index])
                    .u32();
                add_inline(e, text);
                let id = e.mem.u32(map + MAP_EXTRA);
                let text = e.call(VIEWER_STRING_UNSIGNED, &args![TEXT_ID, id]).u32();
                add_inline(e, text);
                map_strings(e, map, texture_buffer, &mut add_inline);
            }
            e.mem.set_u32(result, index.wrapping_add(1));
        }
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a6ab90, ni_texturing_property_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x00a6_b410, ni_texturing_property_sdm_init()),
        entry!(0x00a6_aa40, fn_00a6aa40(Ptr) -> Ptr),
        entry!(0x00a6_9dd0, fn_00a69dd0(Ptr) -> Ptr),
        entry!(0x00a6_9e00, fn_00a69e00(Ptr, u32, u32, u32, u32, u32) -> Ptr),
        entry!(0x00a6_9b00, fn_00a69b00(Ptr) -> Ptr),
        entry!(0x00a6_a000, fn_00a6a000(Ptr) -> u32),
        entry!(0x00a6_a140, ni_texturing_property_get_rtti() -> Ptr),
        entry!(0x00a6_ad40, fn_00a6ad40() -> Ptr),
        entry!(0x00a6_a0b0, fn_00a6a0b0(Ptr)),
        entry!(0x00a6_a6f0, fn_00a6a6f0()),
        entry!(0x00a6_a8b0, fn_00a6a8b0(Ptr, Ptr) -> Ptr),
        entry!(0x00a6_a9f0, fn_00a6a9f0(Ptr, u32) -> Ptr),
        entry!(0x00a6_abb0, fn_00a6abb0(Ptr, u32, Ptr, u32) -> Ptr),
        entry!(0x00a6_c610, ni_texturing_property_create_clone(Ptr, u32) -> Ptr),
        entry!(0x00a6_bb00, fn_00a6bb00(Ptr, Ptr, u32)),
        entry!(0x00a6_b640, ni_texturing_property_map_load_binary(Ptr, Ptr)),
        entry!(0x00a6_a240, ni_texturing_property_link_object(Ptr, Ptr)),
        entry!(0x00a6_ad60, ni_texturing_property_load_binary(Ptr, Ptr)),
        entry!(0x00a6_9b40, fn_00a69b40(Ptr, u32)),
        entry!(
            0x00a6_9b90,
            ni_texturing_property_get_viewer_string(u32, u32) -> u32
        ),
        entry!(
            0x00a6_9c50,
            ni_texturing_property_get_viewer_string_ov2(u32, u32) -> u32
        ),
        entry!(0x00a6_9d80, fn_00a69d80(u32, u32) -> bool),
        entry!(0x00a6_9e60, fn_00a69e60(Ptr, Ptr) -> bool),
        entry!(0x00a6_9ed0, fn_00a69ed0(Ptr, Ptr) -> bool),
        entry!(0x00a6_9f50, fn_00a69f50(Ptr, Ptr) -> bool),
        entry!(0x00a6_9f80, fn_00a69f80(Ptr, Ptr) -> bool),
        entry!(0x00a6_9fb0, fn_00a69fb0(Ptr) -> Ptr),
        entry!(0x00a6_a010, fn_00a6a010(Ptr) -> Ptr),
        entry!(0x00a6_a050, fn_00a6a050(Ptr, u32) -> Ptr),
        entry!(0x00a6_a080, fn_00a6a080(Ptr, u32) -> Ptr),
        entry!(0x00a6_a150, fn_00a6a150(Ptr, u32, u32)),
        entry!(
            0x00a6_a560,
            ni_texturing_property_register_streamables(Ptr, Ptr) -> bool
        ),
        entry!(0x00a6_a5f0, ni_texturing_property_is_equal(Ptr, Ptr) -> bool),
        entry!(
            0x00a6_a990,
            fn_00a6a990(Ptr<NiTArray>, u32, u32) -> Ptr<NiTArray>
        ),
        entry!(0x00a6_b250, ni_texturing_property_save_binary(Ptr, Ptr)),
        entry!(0x00a6_b830, ni_texturing_property_map_save_binary(Ptr, Ptr)),
        entry!(
            0x00a6_b8c0,
            ni_texturing_property_bump_map_load_binary(Ptr, Ptr)
        ),
        entry!(
            0x00a6_b9a0,
            ni_texturing_property_bump_map_save_binary(Ptr, Ptr)
        ),
        entry!(0x00a6_ba80, fn_00a6ba80(Ptr, Ptr)),
        entry!(0x00a6_bac0, fn_00a6bac0(Ptr, Ptr)),
        entry!(
            0x00a6_bdd0,
            ni_texturing_property_get_viewer_strings(Ptr, Ptr<NiTArray>)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ni_texturing_property_scalar_deleting_destructor_deletes_only_on_bit_0() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register(0x00a6_a0b0, |_, _| Ret::default());
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
        assert_eq!(e.call(0x00a6ab90, &args![this, 0xfffe_u32]).u32(), this);
        assert!(calls_to(&e, 0x00aa_1460).is_empty());
        assert_eq!(e.call(0x00a6ab90, &args![this, 1u32]).u32(), this);
        assert_eq!(calls_to(&e, 0x00aa_1460), vec![vec![this, 0x30]]);
        assert_eq!(calls_to(&e, 0x00a6_a0b0), vec![vec![this]; 2]);
    }
}

#[cfg(test)]
mod start_up_tests {
    use super::*;
    use crate::units::nimain::nimainsdm::test_support::*;

    #[test]
    fn builds_the_object_and_sets_the_ten_static_strings() {
        let run = run_default_instance(0x00a6_b410, TEXTURING_PROPERTY_SLOT, |e| {
            e.register(TEXTURING_PROPERTY_CONSTRUCTOR, |_, a| a[0].into_ret());
            e.register(GET_FIXED_STRING, |_, a| (a[0] + 0x1000).into_ret());
            let empty = e.mem.u32(EMPTY_FIXED_STRING);
            for (slot, _) in TEXTURE_SLOT_NAMES {
                e.mem.set_u32(slot, empty);
            }
            // The fourth global already holds its literal's address.
            e.mem.set_u32(0x011f_49bc, 0x0109_c5dc);
        });
        assert_ne!(run.object, 0);
        assert_eq!(run.engine.mem.u32(run.object + 4), 1);
        assert_eq!(*run.freed.borrow(), vec![run.previous]);
        for (slot, literal) in TEXTURE_SLOT_NAMES {
            let expected = if slot == 0x011f_49bc {
                literal
            } else {
                literal + 0x1000
            };
            assert_eq!(run.engine.mem.u32(slot), expected);
        }
    }

    #[test]
    fn a_failed_allocation_stores_null() {
        let run = run_default_instance(0x00a6_b410, TEXTURING_PROPERTY_SLOT, |e| {
            e.register(ALLOCATE, |_, _| 0u32.into_ret());
            e.register(GET_FIXED_STRING, |_, a| a[0].into_ret());
            let empty = e.mem.u32(EMPTY_FIXED_STRING);
            for (slot, _) in TEXTURE_SLOT_NAMES {
                e.mem.set_u32(slot, empty);
            }
        });
        assert_eq!(run.object, 0);
    }
}

#[cfg(test)]
mod constructor_tests {
    use super::*;
    use crate::units::nimain::nimainsdm::test_support::*;

    /// Runs the constructor with `0096ae90` setting the count word to `count` and the
    /// buffer entries to `entries` (from index 1).
    fn construct(count: u16, entries: &[u32]) -> (Engine, u32, Vec<Vec<u32>>) {
        let mut e = engine_with_counting();
        e.register(ALLOCATE_BLOCK, |e, a| e.mem.alloc(a[0]).into_ret());
        let this = e.mem.alloc(0x40);
        e.mem.write(this, &[0xcc; 0x40]);
        let entries = entries.to_vec();
        e.register_double(SET_ELEMENT, move |e, a| {
            if a[1] == 8 {
                // The last call: the count and entries become visible.
                e.mem.set_u16(a[0] + 0xa, count);
                let buffer = e.mem.u32(a[0] + 4);
                for (index, value) in entries.iter().enumerate() {
                    e.mem.set_u32(buffer + 4 * (index as u32 + 1), *value);
                }
            }
            Ret::default()
        });
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00a6_aa40, &args![this]).u32(), this);
        let calls = calls_to(&e, SET_ELEMENT);
        (e, this, calls)
    }

    #[test]
    fn sets_the_part_and_calls_the_element_setter_for_nine_indexes() {
        let (e, this, calls) = construct(1, &[]);
        assert_eq!(calls_to(&e, OBJECT_BASE_CONSTRUCTOR), vec![vec![this]]);
        assert_eq!(e.mem.u32(this), 0x0109_c514);
        assert_eq!(e.mem.u32(this + 0x1c), 0x0109_c5ac);
        assert_eq!(e.mem.u16(this + 0x1c + 8), 9);
        assert_eq!(e.mem.u16(this + 0x1c + 0xe), 1);
        assert_eq!(e.mem.u32(this + 0x2c), 0);
        let indexes: Vec<u32> = calls.iter().map(|c| c[1]).collect();
        assert_eq!(indexes, (0..9).collect::<Vec<u32>>());
        assert!(calls.iter().all(|c| c[0] == this + 0x1c));
    }

    #[test]
    fn flag_word_follows_the_buffer_entries() {
        // count 1: nothing to scan: bit 0 cleared, bit 2 set.
        let (e, this, _) = construct(1, &[]);
        assert_eq!(e.mem.u16(this + 0x18), 0x0004);
        // count 3, all entries zero: same.
        let (e, this, _) = construct(3, &[0, 0]);
        assert_eq!(e.mem.u16(this + 0x18), 0x0004);
        // count 3, the second entry set: bit 0 set.
        let (e, this, _) = construct(3, &[0, 7]);
        assert_eq!(e.mem.u16(this + 0x18), 0x0005);
    }
}

#[cfg(test)]
mod batch_tests {
    use super::*;
    use crate::units::nimain::nimainsdm::test_support::*;
    use std::cell::RefCell;
    use std::collections::VecDeque;
    use std::rc::Rc;

    const READ: u32 = 0x0300_0000;
    const MAP_LOAD: u32 = 0x0300_1000;
    const DELETE: u32 = 0x0300_2000;
    const TEST_VTABLE: u32 = 0x0300_3000;

    /// The engine with the memory manager, the element setter (it stores the word its third
    /// argument points to) and the other callees doubled.
    fn engine() -> Engine {
        let mut e = engine_with_counting();
        e.map(0x0101_6000, 0x1000);
        e.register(MEMORY_ALLOCATE, |e, a| e.mem.alloc(a[1]).into_ret());
        e.register(ALLOCATE_BLOCK, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(SET_ELEMENT, |e, a| {
            let value = e.mem.u32(a[2]);
            let base = e.mem.u32(a[0] + 4);
            e.mem.set_u32(base + 4 * a[1], value);
            Ret::default()
        });
        for address in [
            MEMORY_FREE,
            FREE_BLOCK,
            OBJECT_NET_DESTRUCTOR,
            OBJECT_COPY_MEMBERS,
            OBJECT_NET_COPY_MEMBERS,
            ARRAY_SET_SIZE,
            TEXTURE_TRANSFORM_LOAD_BINARY,
            PROPERTY_LOAD_BINARY,
            STREAM_00A645B0,
            SET_NUMBER_OF_LINK_IDS,
            BASE_LINK_OBJECT,
            MAP_LOAD,
            DELETE,
        ] {
            e.register(address, |_, _| Ret::default());
        }
        e.register(TEXTURE_TRANSFORM_CONSTRUCTOR, |_, a| a[0].into_ret());
        e.register(SOURCE_TEXTURE_CREATE, |_, _| 0x4400u32.into_ret());
        e.put_vtable(TEST_VTABLE, &[DELETE, DELETE]);
        e.put_vtable(MAP_VTABLE, &[DELETE, MAP_LOAD]);
        e.put_vtable(SHADER_MAP_VTABLE, &[DELETE, MAP_LOAD]);
        e.call_log = Some(vec![]);
        e
    }

    /// A counted object (vtable `TEST_VTABLE`, count `count`).
    fn counted(e: &mut Engine, count: u32) -> u32 {
        let at = e.mem.alloc(0x10);
        e.mem.set_u32(at, TEST_VTABLE);
        e.mem.set_u32(at + 4, count);
        at
    }

    /// A stream of `version` whose reader hands out `words` in order (each read stores the
    /// first `size` bytes of the next word and answers `size`).
    fn stream(e: &mut Engine, version: u32, words: &[u32]) -> u32 {
        let queue = Rc::new(RefCell::new(
            words.iter().copied().collect::<VecDeque<u32>>(),
        ));
        e.register_double(READ, move |e, a| {
            let word = queue.borrow_mut().pop_front().expect("a word to read");
            let bytes = word.to_le_bytes();
            for (offset, byte) in bytes.iter().take(a[2] as usize).enumerate() {
                e.mem.set_u8(a[1] + offset as u32, *byte);
            }
            a[2].into_ret()
        });
        let reader = e.mem.alloc(0xc);
        e.mem.set_u32(reader + 8, READ);
        let stream = e.mem.alloc(0x250);
        e.mem.set_u32(stream + 0xd8, version);
        e.mem.set_u32(stream + 0x24c, reader);
        stream
    }

    /// A texturing property with a map array of capacity `capacity` holding `maps`.
    fn property(e: &mut Engine, capacity: u16, maps: &[u32]) -> u32 {
        let this = e.mem.alloc(0x30);
        let elements = e.mem.alloc(4 * u32::from(capacity).max(1));
        for (index, map) in maps.iter().enumerate() {
            e.mem.set_u32(elements + 4 * index as u32, *map);
        }
        e.mem.set_u32(this + MAPS, MAP_ARRAY_VTABLE);
        e.mem.set_u32(this + 0x20, elements);
        e.mem.set_u16(this + 0x24, capacity);
        e.mem.set_u16(this + 0x26, maps.len() as u16);
        e.mem
            .set_u16(this + 0x28, maps.iter().filter(|m| **m != 0).count() as u16);
        e.mem.set_u16(this + 0x2a, 1);
        this
    }

    fn map(e: &mut Engine, texture: u32, transform: u32, size: u32) -> u32 {
        let at = e.mem.alloc(size);
        e.mem.set_u32(at, MAP_VTABLE);
        e.mem.set_u16(at + 4, 0x1234);
        e.mem.set_u32(at + 8, texture);
        e.mem.set_u32(at + 0xc, transform);
        at
    }

    /// The link-id count double: 9 for the first ten questions (the map loop), then 0.
    fn nine_link_ids_then_none(e: &mut Engine) {
        let asked = Rc::new(std::cell::Cell::new(0u32));
        e.register_double(GET_NUMBER_OF_LINK_IDS, move |_, _| {
            asked.set(asked.get() + 1);
            if asked.get() <= 10 { 9u32 } else { 0u32 }.into_ret()
        });
    }

    /// The array set-size double: fresh zeroed elements of the asked capacity.
    fn growing_arrays(e: &mut Engine) {
        e.register(ARRAY_SET_SIZE, |e, a| {
            let elements = e.mem.alloc(4 * a[1].max(1));
            e.mem.set_u32(a[0] + 4, elements);
            e.mem.set_u16(a[0] + 8, a[1] as u16);
            Ret::default()
        });
    }

    #[test]
    fn the_map_constructors_set_the_flag_word() {
        let mut e = engine();
        let at = e.mem.alloc(0x10);
        e.mem.write(at, &[0xff; 0x10]);
        e.call(0x00a6_9dd0, &args![at]);
        assert_eq!(e.mem.u32(at), MAP_VTABLE);
        assert_eq!(e.mem.u16(at + 4), 0x3100);
        assert_eq!((e.mem.u32(at + 8), e.mem.u32(at + 0xc)), (0, 0));
        let texture = counted(&mut e, 1);
        let answer = e
            .call(0x00a6_9e00, &args![at, texture, 2u32, 3u32, 1u32, 0x77u32])
            .u32();
        assert_eq!(answer, at);
        assert_eq!(e.mem.u16(at + 4), 0x3102);
        assert_eq!(e.mem.u32(at + 8), texture);
        assert_eq!(e.mem.u32(texture + 4), 2);
        assert_eq!(e.mem.u32(at + 0xc), 0x77);
        e.call(0x00a6_9e00, &args![at, 0u32, 0u32, 0u32, 0u32, 0u32]);
        assert_eq!((e.mem.u32(at + 8), e.mem.u16(at + 4)), (0, 0));
    }

    #[test]
    fn a_new_transform_has_its_defaults() {
        let mut e = engine();
        e.mem.set_u32(ZERO_POINT_2, 0x11);
        e.mem.set_u32(ZERO_POINT_2 + 4, 0x22);
        e.mem.set_f32(TRANSFORM_CENTRE, 0.5);
        let at = e.mem.alloc(0x48);
        assert_eq!(e.call(0x00a6_9b00, &args![at]).u32(), at);
        assert_eq!((e.mem.u32(at), e.mem.u32(at + 4)), (0x11, 0x22));
        let floats: Vec<f32> = (2..7).map(|i| e.mem.f32(at + 4 * i)).collect();
        assert_eq!(floats, vec![0.0, 1.0, 1.0, 0.5, 0.5]);
        assert_eq!(e.mem.u8(at + 0x1c), 1);
        assert_eq!(e.mem.u32(at + 0x44), 2);
    }

    #[test]
    fn the_constant_slots() {
        let mut e = engine();
        assert_eq!(e.call(0x00a6_a000, &args![0x5000u32]).u32(), 3);
        assert_eq!(e.call(0x00a6_a140, &args![]).u32(), TEXTURING_PROPERTY_RTTI);
    }

    #[test]
    fn the_factory_constructs_a_property() {
        let mut e = engine();
        let object = e.call(0x00a6_ad40, &args![]).u32();
        assert_ne!(object, 0);
        assert_eq!(e.mem.u32(object), TEXTURING_PROPERTY_VTABLE);
        assert_eq!(e.mem.u16(object + FLAGS), 4);
        e.register(ALLOCATE, |_, _| 0u32.into_ret());
        assert_eq!(e.call(0x00a6_ad40, &args![]).u32(), 0);
    }

    #[test]
    fn the_destructor_deletes_the_maps_and_the_shader_maps() {
        let mut e = engine();
        let first = map(&mut e, 0, 0, 0x10);
        let second = map(&mut e, 0, 0, 0x10);
        let this = property(&mut e, 4, &[first, 0, second]);
        let shader = map(&mut e, 0, 0, 0x14);
        let shader_array = e.mem.alloc(0x10);
        let shader_elements = e.mem.alloc(8);
        e.mem.set_u32(shader_array, TEST_VTABLE);
        e.mem.set_u32(shader_array + 4, shader_elements);
        e.mem.set_u16(shader_array + 0xa, 2);
        e.mem.set_u32(shader_elements + 4, shader);
        e.mem.set_u32(this + SHADER_MAPS, shader_array);
        let elements = e.mem.u32(this + 0x20);
        e.call(0x00a6_a0b0, &args![this]);
        let deleted: Vec<u32> = calls_to(&e, DELETE).iter().map(|c| c[0]).collect();
        assert_eq!(deleted, vec![first, second, shader, shader_array]);
        assert!(calls_to(&e, DELETE).iter().all(|c| c[1] == 1));
        assert_eq!(e.mem.u32(this), TEXTURING_PROPERTY_VTABLE);
        assert_eq!(e.mem.u32(this + MAPS), MAP_ARRAY_BASE_VTABLE);
        assert_eq!(calls_to(&e, FREE_BLOCK), vec![vec![elements]]);
        assert_eq!(calls_to(&e, OBJECT_NET_DESTRUCTOR), vec![vec![this]]);
    }

    #[test]
    fn the_shutdown_releases_the_default_and_the_strings() {
        let mut e = engine();
        e.register(GET_FIXED_STRING, |_, _| 0x0109_b228u32.into_ret());
        let object = counted(&mut e, 1);
        e.mem.set_u32(TEXTURING_PROPERTY_SLOT, object);
        let handle = e.mem.alloc(0x10) + 8;
        e.mem.set_u32(handle - 8, 30);
        for (index, (slot, _)) in TEXTURE_SLOT_NAMES.iter().enumerate() {
            let value = if index == 2 { 0 } else { handle };
            e.mem.set_u32(*slot, value);
        }
        e.call(0x00a6_a6f0, &args![]);
        assert_eq!(e.mem.u32(TEXTURING_PROPERTY_SLOT), 0);
        assert_eq!(calls_to(&e, DELETE), vec![vec![object]]);
        assert_eq!(calls_to(&e, GET_FIXED_STRING), vec![vec![0]; 9]);
        assert_eq!(e.mem.u32(TEXTURE_SLOT_NAMES[2].0), 0);
        assert_eq!(e.mem.u32(TEXTURE_SLOT_NAMES[0].0), 0x0109_b228);
        assert_eq!(e.mem.u32(handle - 8), 21);
    }

    #[test]
    fn the_map_copy_copies_the_transform_through_its_constructor() {
        let mut e = engine();
        let texture = counted(&mut e, 1);
        let transform = e.mem.alloc(0x48);
        for word in 0..0x12 {
            e.mem.set_u32(transform + 4 * word, 0x100 + word);
        }
        e.mem.set_f32(transform + 8, 0.25);
        let source = map(&mut e, texture, transform, 0x10);
        let this = e.mem.alloc(0x10);
        e.register(TEXTURE_TRANSFORM_CONSTRUCTOR, |e, a| {
            // translate, rotation, scale, centre, method
            assert_eq!((e.mem.u32(a[1]), e.mem.u32(a[1] + 4)), (0x100, 0x101));
            assert_eq!(f32::from_bits(a[2]), 0.25);
            assert_eq!((e.mem.u32(a[3]), e.mem.u32(a[3] + 4)), (0x103, 0x104));
            assert_eq!((e.mem.u32(a[4]), e.mem.u32(a[4] + 4)), (0x105, 0x106));
            assert_eq!(a[5], 0x111);
            0x9900u32.into_ret()
        });
        assert_eq!(e.call(0x00a6_a8b0, &args![this, source]).u32(), this);
        assert_eq!(e.mem.u32(this), MAP_VTABLE);
        assert_eq!(e.mem.u16(this + 4), 0x1234);
        assert_eq!(e.mem.u32(this + 8), texture);
        assert_eq!(e.mem.u32(texture + 4), 2);
        assert_eq!(e.mem.u32(this + 0xc), 0x9900);
        // Without a transform, or when the allocation fails: none.
        let plain = map(&mut e, 0, 0, 0x10);
        e.call(0x00a6_a8b0, &args![this, plain]);
        assert_eq!(e.mem.u32(this + 0xc), 0);
        e.register(MEMORY_ALLOCATE, |_, _| 0u32.into_ret());
        e.call(0x00a6_a8b0, &args![this, source]);
        assert_eq!(e.mem.u32(this + 0xc), 0);
    }

    #[test]
    fn the_map_deleting_destructor_frees_through_the_memory_manager() {
        let mut e = engine();
        let texture = counted(&mut e, 1);
        let this = map(&mut e, texture, 0x6600, 0x10);
        assert_eq!(e.call(0x00a6_a9f0, &args![this, 1u32]).u32(), this);
        assert_eq!(
            calls_to(&e, MEMORY_FREE),
            vec![vec![MEMORY_MANAGER, 0x6600], vec![MEMORY_MANAGER, this]]
        );
        assert_eq!(calls_to(&e, DELETE), vec![vec![texture]]);
        e.mem.set_u32(this + 8, 0);
        e.call(0x00a6_a9f0, &args![this, 0u32]);
        assert_eq!(calls_to(&e, MEMORY_FREE).len(), 3);
    }

    #[test]
    fn the_file_constructor_puts_the_texture_in_map_0() {
        let mut e = engine();
        let name = e.mem.alloc(4);
        e.mem.set_u32(name, 0x5150);
        let this = e.mem.alloc(0x30);
        let texture = counted(&mut e, 1);
        e.register_double(SOURCE_TEXTURE_CREATE, move |_, _| texture.into_ret());
        // The setter also leaves the size at 9 as the real one would.
        e.register(SET_ELEMENT, |e, a| {
            let value = e.mem.u32(a[2]);
            let base = e.mem.u32(a[0] + 4);
            e.mem.set_u32(base + 4 * a[1], value);
            e.mem.set_u16(a[0] + 0xa, 9);
            Ret::default()
        });
        let answer = e.call(0x00a6_abb0, &args![this, 7u32, name, 9u32]).u32();
        assert_eq!(answer, this);
        assert_eq!(
            calls_to(&e, SOURCE_TEXTURE_CREATE),
            vec![vec![7, 0x5150, 9]]
        );
        let first = e.mem.u32(e.mem.u32(this + 0x20));
        assert_ne!(first, 0);
        assert_eq!(e.mem.u32(first + 8), texture);
        assert_eq!(e.mem.u16(first + 4), 0x3100);
        let indexes: Vec<u32> = calls_to(&e, SET_ELEMENT).iter().map(|c| c[1]).collect();
        assert_eq!(indexes, (0..9).collect::<Vec<u32>>());
        // Only map 0 is set: bit 0 stays clear.
        assert_eq!(e.mem.u16(this + FLAGS), 4);
        assert_eq!(e.mem.u32(this), TEXTURING_PROPERTY_VTABLE);
    }

    #[test]
    fn a_property_without_controllers_is_its_own_clone() {
        let mut e = engine();
        let this = property(&mut e, 2, &[]);
        e.mem.set_u32(this + 0xc, 0);
        assert_eq!(e.call(0x00a6_c610, &args![this, 3u32]).u32(), this);
        assert_eq!(calls_to(&e, OBJECT_COPY_MEMBERS), vec![vec![this, this, 3]]);
    }

    #[test]
    fn a_clone_copies_the_maps_by_kind() {
        let mut e = engine();
        growing_arrays(&mut e);
        let texture = counted(&mut e, 1);
        let plain = map(&mut e, texture, 0, 0x10);
        let fifth = map(&mut e, 0, 0, 0x28);
        for at in (0x10..0x28).step_by(4) {
            e.mem.set_f32(fifth + at, at as f32);
        }
        let seventh = map(&mut e, 0, 0, 0x14);
        e.mem.set_f32(seventh + 0x10, 7.5);
        let this = property(&mut e, 8, &[plain, 0, 0, 0, 0, fifth, 0, seventh]);
        e.mem.set_u32(this + 0xc, 0x1111);
        let shader = map(&mut e, 0, 0, 0x14);
        e.mem.set_u32(shader + 0x10, 0xabc);
        let shader_array = e.mem.alloc(0x10);
        let shader_elements = e.mem.alloc(8);
        e.mem.set_u32(shader_array + 4, shader_elements);
        e.mem.set_u16(shader_array + 0xa, 2);
        e.mem.set_u32(shader_elements + 4, shader);
        e.mem.set_u32(this + SHADER_MAPS, shader_array);
        let clone = e.call(0x00a6_c610, &args![this, 3u32]).u32();
        assert_ne!(clone, this);
        assert_eq!(e.mem.u32(clone), TEXTURING_PROPERTY_VTABLE);
        assert_eq!(e.mem.u16(clone + FLAGS), e.mem.u16(this + FLAGS));
        let elements = e.mem.u32(clone + 0x20);
        let copies: Vec<u32> = (0..8).map(|i| e.mem.u32(elements + 4 * i)).collect();
        assert_eq!(e.mem.u32(copies[0] + 8), texture);
        assert_eq!(e.mem.u32(copies[0]), MAP_VTABLE);
        assert_eq!(e.mem.u32(copies[5]), SLOT_5_MAP_VTABLE);
        assert_eq!(e.mem.f32(copies[5] + 0x24), 36.0);
        assert_eq!(e.mem.u32(copies[7]), SLOT_7_MAP_VTABLE);
        assert_eq!(e.mem.f32(copies[7] + 0x10), 7.5);
        assert_eq!(copies[1], 0);
        let shader_copy_array = e.mem.u32(clone + SHADER_MAPS);
        assert_ne!(shader_copy_array, 0);
        assert_eq!(e.mem.u16(shader_copy_array + 0xa), 2);
        assert_eq!(e.mem.u16(shader_copy_array + 0xc), 1);
        let copy = e.mem.u32(e.mem.u32(shader_copy_array + 4) + 4);
        assert_eq!(e.mem.u32(copy), SHADER_MAP_VTABLE);
        assert_eq!(e.mem.u32(copy + 0x10), 0xabc);
    }

    #[test]
    fn a_map_from_an_old_stream_builds_its_flags_from_three_words() {
        let mut e = engine();
        let at = map(&mut e, 0, 0x55, 0x10);
        e.mem.set_u16(at + 4, 0xffff);
        let s = stream(&mut e, 0x0400_0000, &[0x1, 0x7, 0x42, 0, 0, 0]);
        e.call(0x00a6_b640, &args![at, s]);
        assert_eq!(e.mem.u16(at + 4), 0xd742);
        assert_eq!(e.mem.u32(at + 0xc), 0x55);
        assert_eq!(calls_to(&e, READ).len(), 6);
    }

    #[test]
    fn a_map_from_a_new_stream_reads_its_flags_and_maybe_a_transform() {
        let mut e = engine();
        let at = map(&mut e, 0, 0x55, 0x10);
        let s = stream(&mut e, 0x1402_0007, &[0x2233, 1]);
        e.call(0x00a6_b640, &args![at, s]);
        assert_eq!(e.mem.u16(at + 4), 0x2233);
        let transform = e.mem.u32(at + 0xc);
        assert_ne!(transform, 0);
        assert_eq!(e.mem.u32(transform + 0x44), 2);
        assert_eq!(
            calls_to(&e, TEXTURE_TRANSFORM_LOAD_BINARY),
            vec![vec![transform, s]]
        );
        let s = stream(&mut e, 0x1402_0007, &[0x2233, 0]);
        e.call(0x00a6_b640, &args![at, s]);
        assert_eq!(e.mem.u32(at + 0xc), 0);
    }

    #[test]
    fn linking_gives_each_map_its_texture_and_counts_the_extra_maps() {
        let mut e = engine();
        let textures: Vec<u32> = (0..9).map(|_| counted(&mut e, 1)).collect();
        let maps: Vec<u32> = (0..9).map(|_| map(&mut e, 0, 0, 0x10)).collect();
        let this = property(&mut e, 9, &maps);
        e.mem.set_u16(this + FLAGS, 0xffff);
        let s = stream(&mut e, 0x1402_0005, &[]);
        let next = Rc::new(RefCell::new(textures.clone().into_iter()));
        e.register_double(GET_OBJECT_FROM_LINK_ID, move |_, _| {
            next.borrow_mut().next().unwrap().into_ret()
        });
        nine_link_ids_then_none(&mut e);
        e.call(0x00a6_a240, &args![this, s]);
        for (map, texture) in maps.iter().zip(&textures) {
            assert_eq!(e.mem.u32(map + 8), *texture);
            assert_eq!(e.mem.u32(texture + 4), 2);
        }
        // Index 8 counted once in bits 4-11.
        assert_eq!(e.mem.u16(this + FLAGS), 0xf01f);
    }

    #[test]
    fn linking_an_old_stream_moves_the_slot_6_and_7_maps() {
        let mut e = engine();
        let maps: Vec<u32> = (0..9).map(|_| map(&mut e, 0, 0, 0x10)).collect();
        let this = property(&mut e, 12, &maps);
        let s = stream(&mut e, 0x1402_0004, &[]);
        // Nine link ids, all null textures: index 8 counts one extra map.
        nine_link_ids_then_none(&mut e);
        e.register(GET_OBJECT_FROM_LINK_ID, |_, _| 0u32.into_ret());
        e.call(0x00a6_a240, &args![this, s]);
        let sets: Vec<u32> = calls_to(&e, SET_ELEMENT).iter().map(|c| c[1]).collect();
        // Map 8 moves up by two (to 10), map 6 goes to 8 (6 cleared) and map 7 to 9
        // (7 cleared).
        assert_eq!(sets, vec![10, 8, 6, 9, 7]);
        let moved = calls_to(&e, SET_ELEMENT)[0][2];
        assert_eq!(moved, e.mem.u32(this + 0x20) + 0x20);
        let elements = e.mem.u32(this + 0x20);
        assert_eq!(e.mem.u32(elements + 0x28), maps[8]);
        assert_eq!(e.mem.u32(elements + 0x20), maps[6]);
        assert_eq!(e.mem.u32(elements + 0x24), maps[7]);
        assert_eq!(e.mem.u32(elements + 0x18), 0);
        assert_eq!(e.mem.u16(this + FLAGS), 0x0030);
    }

    #[test]
    fn loading_reads_the_maps_and_the_shader_maps() {
        let mut e = engine();
        growing_arrays(&mut e);
        let this = property(&mut e, 0, &[]);
        e.register(SHADER_MAP_ARRAY_CONSTRUCTOR, |e, a| {
            let elements = e.mem.alloc(4 * a[1]);
            e.mem.set_u32(a[0] + 4, elements);
            e.mem.set_u16(a[0] + 8, a[1] as u16);
            a[0].into_ret()
        });
        // Flags, two maps (the first absent, the second present), two shader maps (the
        // second present).
        let s = stream(&mut e, 0x1402_0007, &[0xff7f, 2, 0, 1, 2, 0, 1]);
        e.call(0x00a6_ad60, &args![this, s]);
        assert_eq!(calls_to(&e, PROPERTY_LOAD_BINARY), vec![vec![this, s]]);
        let links: Vec<u32> = calls_to(&e, SET_NUMBER_OF_LINK_IDS)
            .iter()
            .map(|c| c[1])
            .collect();
        assert_eq!(links, vec![2, 0, 1, 2, 0, 1]);
        let elements = e.mem.u32(this + 0x20);
        let second = e.mem.u32(elements + 4);
        assert_eq!(e.mem.u32(elements), 0);
        assert_eq!(e.mem.u32(second), MAP_VTABLE);
        assert_eq!(e.mem.u16(this + 0x26), 2);
        assert_eq!(e.mem.u16(this + 0x28), 1);
        let shader_array = e.mem.u32(this + SHADER_MAPS);
        assert_eq!(e.mem.u16(shader_array + 0xa), 2);
        assert_eq!(e.mem.u16(shader_array + 0xc), 1);
        let shader = e.mem.u32(e.mem.u32(shader_array + 4) + 4);
        assert_eq!(e.mem.u32(shader), SHADER_MAP_VTABLE);
        let loads: Vec<u32> = calls_to(&e, MAP_LOAD).iter().map(|c| c[0]).collect();
        assert_eq!(loads, vec![second, shader]);
        // Flags masked with 0xf00f, then bit 0 set for the map at index 1.
        assert_eq!(e.mem.u16(this + FLAGS), 0xf00f);
    }

    #[test]
    fn loading_an_old_stream_puts_the_flag_word_in_bits_1_to_3() {
        let mut e = engine();
        growing_arrays(&mut e);
        let this = property(&mut e, 0, &[]);
        e.mem.set_u16(this + FLAGS, 0xffff);
        // Flags 5, one map (absent, a 4-byte word in a stream before 0x04010000).
        let s = stream(&mut e, 0x0303_0000, &[5, 1, 0]);
        e.call(0x00a6_ad60, &args![this, s]);
        let links: Vec<u32> = calls_to(&e, SET_NUMBER_OF_LINK_IDS)
            .iter()
            .map(|c| c[1])
            .collect();
        assert_eq!(links, vec![2, 0]);
        assert_eq!(e.mem.u16(this + FLAGS), 0xfffa);
        assert_eq!(e.mem.u16(this + 0x26), 1);
    }
}

#[cfg(test)]
mod n020_tests {
    use super::*;
    use crate::units::nimain::nimainsdm::test_support::{calls_to, engine_with_counting};

    const VTABLE: u32 = 0x0710_0000;
    const DELETE_DOUBLE: u32 = 0x0720_0000;
    const SAVE_DOUBLE: u32 = 0x0720_0010;
    const REGISTER_DOUBLE: u32 = 0x0720_0020;
    const TEXTURE_EQUAL_DOUBLE: u32 = 0x0720_0030;
    const SAVE_LINK_DOUBLE: u32 = 0x0720_0040;
    const TRANSFER: u32 = 0x0720_0050;
    const READ: u32 = 0x0720_0060;

    /// The counting engine with one vtable for maps, textures and streams (slot 0 deletes,
    /// +8 saves, +0x30 saves a link, +0x54 registers, +0x5C compares the first words).
    fn engine() -> Engine {
        let mut e = engine_with_counting();
        let mut slots = vec![0u32; 0x60 / 4];
        slots[0] = DELETE_DOUBLE;
        slots[MAP_SAVE_BINARY_SLOT as usize / 4] = SAVE_DOUBLE;
        slots[STREAM_SAVE_LINK_ID_SLOT as usize / 4] = SAVE_LINK_DOUBLE;
        slots[TEXTURE_REGISTER_STREAMABLES_SLOT as usize / 4] = REGISTER_DOUBLE;
        slots[TEXTURE_IS_EQUAL_SLOT as usize / 4] = TEXTURE_EQUAL_DOUBLE;
        e.put_vtable(VTABLE, &slots);
        for addr in [
            DELETE_DOUBLE,
            SAVE_DOUBLE,
            REGISTER_DOUBLE,
            SAVE_LINK_DOUBLE,
        ] {
            e.register(addr, |_, _| Ret::default());
        }
        e.register(TEXTURE_EQUAL_DOUBLE, |e, a| {
            u32::from(a[1] != 0 && e.mem.u32(a[0] + 0x10) == e.mem.u32(a[1] + 0x10)).into_ret()
        });
        e.register(TRANSFER, |e, a| {
            assert_eq!(e.mem.u32(a[3]), a[2]);
            a[2].into_ret()
        });
        e.register(READ, |e, a| {
            assert_eq!(e.mem.u32(a[3]), a[2]);
            for i in 0..a[2] {
                e.mem.set_u8(a[1] + i, 0);
            }
            a[2].into_ret()
        });
        e.map(0x0101_6000, 0x1000);
        e.mem.set_u32(TRANSFORM_CENTRE, 0x3f00_0000);
        e
    }

    fn object(e: &mut Engine, size: u32) -> u32 {
        let block = e.mem.alloc(size);
        e.mem.set_u32(block, VTABLE);
        block
    }

    /// A property with `count` map slots holding `maps`.
    fn property(e: &mut Engine, maps: &[u32]) -> u32 {
        let this = object(e, 0x30);
        let elements = e.mem.alloc(4 * maps.len() as u32 + 4);
        for (i, m) in maps.iter().enumerate() {
            e.mem.set_u32(elements + 4 * i as u32, *m);
        }
        e.mem.set_u32(this + MAP_ELEMENTS, elements);
        e.mem.set_u16(this + MAP_CAPACITY, maps.len() as u16);
        e.mem.set_u16(this + MAP_COUNT, maps.len() as u16);
        e.mem.set_u16(this + MAPS + 0xe, 2);
        this
    }

    fn stream(e: &mut Engine) -> u32 {
        let stream = object(e, 0x5c4);
        let binary = e.mem.alloc(0x10);
        e.mem.set_u32(binary + 8, READ);
        e.mem.set_u32(binary + 0xc, TRANSFER);
        e.mem.set_u32(stream + 0x24c, binary);
        e.mem.set_u32(stream + 0x250, binary);
        stream
    }

    #[test]
    fn flag_bit_0_follows_the_word_and_the_later_maps() {
        let mut e = engine();
        let this = property(&mut e, &[1, 0, 0]);
        e.mem.set_u16(this + FLAGS, 0x10);
        e.call(0x00a6_9b40, &args![this, 0u32]);
        assert_eq!(e.mem.u16(this + FLAGS), 0x10);
        e.call(0x00a6_9b40, &args![this, 7u32]);
        assert_eq!(e.mem.u16(this + FLAGS), 0x11);
        e.call(0x00a6_9b40, &args![this, 0u32]);
        assert_eq!(e.mem.u16(this + FLAGS), 0x10);
        e.mem.set_u32(e.mem.u32(this + MAP_ELEMENTS) + 8, 5);
        e.call(0x00a6_9b40, &args![this, 0u32]);
        assert_eq!(e.mem.u16(this + FLAGS), 0x11);
    }

    #[test]
    fn mode_strings_pick_their_formats() {
        let mut e = engine();
        e.register(ALLOCATE_BLOCK, |_, a| (0x0c00_0000 + a[0]).into_ret());
        e.register(NI_SPRINTF, |_, _| Ret::default());
        let name = e.mem.alloc(4);
        e.mem.write(name, b"abc");
        e.call_log = Some(vec![]);
        for mode in 0..5u32 {
            e.call(0x00a6_9b90, &args![name, mode]);
        }
        for mode in [0u32, 7, 8] {
            e.call(0x00a6_9c50, &args![name, mode]);
        }
        let formats: Vec<u32> = calls_to(&e, NI_SPRINTF).iter().map(|w| w[2]).collect();
        let mut expected = CLAMP_FORMATS.to_vec();
        expected.extend([
            FORMAT_UNKNOWN,
            FILTER_FORMATS[0],
            FILTER_FORMATS[7],
            FORMAT_UNKNOWN,
        ]);
        assert_eq!(formats, expected);
        let sizes: Vec<u32> = calls_to(&e, ALLOCATE_BLOCK).iter().map(|w| w[0]).collect();
        assert_eq!(sizes, vec![0x17, 0x17, 0x17, 0x17, 0x17, 0x21, 0x21, 0x21]);
    }

    #[test]
    fn slot_name_lookup_matches_pointers_or_names() {
        let mut e = engine();
        e.register(NI_STRICMP, |e, a| {
            u32::from(e.mem.u32(a[0]) != e.mem.u32(a[1])).into_ret()
        });
        for (i, (slot, _)) in TEXTURE_SLOT_NAMES.iter().enumerate() {
            let s = e.mem.alloc(4);
            e.mem.set_u32(s, 0x41 + i as u32);
            e.mem.set_u32(*slot, s);
        }
        let holder = e.mem.alloc(8);
        let copy = e.mem.alloc(4);
        e.mem.set_u32(copy, 0x41 + 9);
        e.mem.set_u32(holder, copy);
        assert_eq!(
            e.call(0x00a6_9d80, &args![holder, holder + 4]).u32() & 0xff,
            1
        );
        assert_eq!(e.mem.u32(holder + 4), 9);
        e.mem.set_u32(holder, e.mem.u32(TEXTURE_SLOT_NAMES[4].0));
        assert_eq!(
            e.call(0x00a6_9d80, &args![holder, holder + 4]).u32() & 0xff,
            1
        );
        assert_eq!(e.mem.u32(holder + 4), 4);
        e.mem.set_u32(copy, 0x20);
        e.mem.set_u32(holder, copy);
        assert_eq!(
            e.call(0x00a6_9d80, &args![holder, holder + 4]).u32() & 0xff,
            0
        );
    }

    #[test]
    fn map_equality_checks_texture_flags_and_transform() {
        let mut e = engine();
        e.register(TRANSFORM_NOT_EQUAL_00A88C60, |e, a| {
            u32::from(e.mem.u32(a[0]) != e.mem.u32(a[1])).into_ret()
        });
        let a = object(&mut e, 0x28);
        let b = object(&mut e, 0x28);
        assert_eq!(e.call(0x00a6_9e60, &args![a, b]).u32() & 0xff, 1);
        let ta = object(&mut e, 0x14);
        let tb = object(&mut e, 0x14);
        e.mem.set_u32(a + MAP_TEXTURE, ta);
        assert_eq!(e.call(0x00a6_9e60, &args![a, b]).u32() & 0xff, 0);
        e.mem.set_u32(b + MAP_TEXTURE, tb);
        assert_eq!(e.call(0x00a6_9e60, &args![a, b]).u32() & 0xff, 1);
        e.mem.set_u32(tb + 0x10, 1);
        assert_eq!(e.call(0x00a6_9e60, &args![a, b]).u32() & 0xff, 0);
        e.mem.set_u32(tb + 0x10, 0);
        e.mem.set_u16(b + MAP_FLAGS, 3);
        assert_eq!(e.call(0x00a6_9e60, &args![a, b]).u32() & 0xff, 0);
        e.mem.set_u16(b + MAP_FLAGS, 0);
        let xa = e.mem.alloc(4);
        let xb = e.mem.alloc(4);
        e.mem.set_u32(a + MAP_TRANSFORM, xa);
        assert_eq!(e.call(0x00a6_9e60, &args![a, b]).u32() & 0xff, 0);
        e.mem.set_u32(b + MAP_TRANSFORM, xb);
        assert_eq!(e.call(0x00a6_9e60, &args![a, b]).u32() & 0xff, 1);
        e.mem.set_u32(xb, 9);
        assert_eq!(e.call(0x00a6_9e60, &args![a, b]).u32() & 0xff, 0);
        // The derived maps add their own fields.
        e.mem.set_u32(xb, 0);
        e.mem.set_u32(a + 0x24, 0x7fc0_0000);
        e.mem.set_u32(b + 0x24, 0x7fc0_0000);
        assert_eq!(e.call(0x00a6_9ed0, &args![a, b]).u32() & 0xff, 0);
        e.mem.set_u32(a + 0x24, 0x8000_0000);
        e.mem.set_u32(b + 0x24, 0);
        assert_eq!(e.call(0x00a6_9ed0, &args![a, b]).u32() & 0xff, 1);
        assert_eq!(e.call(0x00a6_9f80, &args![a, b]).u32() & 0xff, 1);
        e.mem.set_u32(a + 0x10, 0x3f80_0000);
        assert_eq!(e.call(0x00a6_9f80, &args![a, b]).u32() & 0xff, 0);
        assert_eq!(e.call(0x00a6_9f50, &args![a, b]).u32() & 0xff, 0);
        e.mem.set_u32(b + 0x10, 0x3f80_0000);
        assert_eq!(e.call(0x00a6_9f50, &args![a, b]).u32() & 0xff, 1);
    }

    #[test]
    fn map_constructors_set_their_defaults() {
        let mut e = engine();
        let bump = e.mem.alloc(0x28);
        e.mem.set_u16(bump + MAP_FLAGS, 0xffff);
        assert_eq!(e.call(0x00a6_9fb0, &args![bump]).u32(), bump);
        assert_eq!(e.mem.u32(bump), SLOT_5_MAP_VTABLE);
        assert_eq!(e.mem.u16(bump + MAP_FLAGS), 0x3100);
        let floats: Vec<u32> = (0..6).map(|i| e.mem.u32(bump + 0x10 + 4 * i)).collect();
        assert_eq!(floats, vec![0x3f80_0000, 0, 0x3f00_0000, 0, 0, 0x3f00_0000]);
        let seven = e.mem.alloc(0x14);
        assert_eq!(e.call(0x00a6_a010, &args![seven]).u32(), seven);
        assert_eq!(e.mem.u32(seven), SLOT_7_MAP_VTABLE);
        assert_eq!(e.mem.u32(seven + 0x10), 0x3f00_0000);
        assert_eq!(e.mem.u16(seven + MAP_FLAGS), 0x3100);
    }

    #[test]
    fn map_array_destructors_free_the_elements() {
        let mut e = engine();
        e.register(FREE_BLOCK, |_, _| Ret::default());
        e.register(MEMORY_FREE, |_, _| Ret::default());
        let array = e.mem.alloc(0x10);
        e.mem.set_u32(array + 4, 0x0abc_0000);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00a6_a050, &args![array, 1u32]).u32(), array);
        assert_eq!(e.mem.u32(array), MAP_ARRAY_BASE_VTABLE);
        e.call(0x00a6_a080, &args![array, 0u32]);
        assert_eq!(e.mem.u32(array), SHADER_MAP_ARRAY_BASE_VTABLE);
        assert_eq!(calls_to(&e, FREE_BLOCK), vec![vec![0x0abc_0000]; 2]);
        assert_eq!(calls_to(&e, MEMORY_FREE), vec![vec![MEMORY_MANAGER, array]]);
    }

    #[test]
    fn decal_maps_count_replace_and_grow() {
        let mut e = engine();
        e.register(SET_ELEMENT, |e, a| {
            let base = e.mem.u32(a[0] + 4);
            let value = e.mem.u32(a[2]);
            e.mem.set_u32(base + 4 * a[1], value);
            if a[1] >= u32::from(e.mem.u16(a[0] + 0xa)) {
                e.mem.set_u16(a[0] + 0xa, a[1] as u16 + 1);
            }
            Ret::default()
        });
        e.register(ARRAY_SET_SIZE, |e, a| {
            let base = e.mem.alloc(4 * a[1]);
            let old = e.mem.u32(a[0] + 4);
            for i in 0..u32::from(e.mem.u16(a[0] + 0xa)) {
                let v = e.mem.u32(old + 4 * i);
                e.mem.set_u32(base + 4 * i, v);
            }
            e.mem.set_u32(a[0] + 4, base);
            e.mem.set_u16(a[0] + 8, a[1] as u16);
            Ret::default()
        });
        let old = object(&mut e, 0x10);
        let this = property(&mut e, &[0, 0, 0, 0, 0, 0, 0, 0, old]);
        e.mem.set_u16(this + FLAGS, 0x0010);
        e.call_log = Some(vec![]);
        // Replace decal 0 with nothing: the count drops, the old map is deleted.
        e.call(0x00a6_a150, &args![this, 0u32, 0u32]);
        assert_eq!(e.mem.u16(this + FLAGS), 0x0000);
        assert_eq!(calls_to(&e, DELETE_DOUBLE), vec![vec![old, 1]]);
        // A new decal 1 past the size: the array grows, the count goes up, bit 0 is set.
        let new = object(&mut e, 0x10);
        e.call(0x00a6_a150, &args![this, 1u32, new]);
        assert_eq!(e.mem.u16(this + FLAGS), 0x0011);
        assert_eq!(e.mem.u32(e.mem.u32(this + MAP_ELEMENTS) + 4 * 9), new);
        assert_eq!(calls_to(&e, ARRAY_SET_SIZE), vec![vec![this + MAPS, 2 + 9]]);
        // A null decal 0 over a null keeps the count.
        e.call(0x00a6_a150, &args![this, 0u32, 0u32]);
        assert_eq!(e.mem.u16(this + FLAGS), 0x0011);
    }

    #[test]
    fn register_streamables_walks_maps_and_shader_maps() {
        let mut e = engine();
        e.register(BASE_REGISTER_STREAMABLES_00A62130, |_, _| 1u32.into_ret());
        let texture = object(&mut e, 0x14);
        let map = object(&mut e, 0x10);
        e.mem.set_u32(map + MAP_TEXTURE, texture);
        let bare = object(&mut e, 0x10);
        let this = property(&mut e, &[map, 0, bare]);
        let shader_texture = object(&mut e, 0x14);
        let shader_map = object(&mut e, 0x14);
        e.mem.set_u32(shader_map + MAP_TEXTURE, shader_texture);
        let shaders = e.mem.alloc(0x10);
        let shader_elements = e.mem.alloc(4);
        e.mem.set_u32(shader_elements, shader_map);
        e.mem.set_u32(shaders + 4, shader_elements);
        e.mem.set_u16(shaders + 0xa, 1);
        e.mem.set_u32(this + SHADER_MAPS, shaders);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00a6_a560, &args![this, 0x50u32]).u32() & 0xff, 1);
        assert_eq!(
            calls_to(&e, REGISTER_DOUBLE),
            vec![vec![texture, 0x50], vec![shader_texture, 0x50]]
        );
        e.register(BASE_REGISTER_STREAMABLES_00A62130, |_, _| 0u32.into_ret());
        assert_eq!(e.call(0x00a6_a560, &args![this, 0x50u32]).u32() & 0xff, 0);
    }

    #[test]
    fn property_equality_uses_the_slot_comparisons() {
        let mut e = engine();
        e.register(BASE_IS_EQUAL_00A62140, |_, _| 1u32.into_ret());
        let make = |e: &mut Engine, extra: u32| {
            let m = object(e, 0x28);
            e.mem.set_u32(m + 0x10, extra);
            m
        };
        let a_maps: Vec<u32> = (0..8)
            .map(|i| if i == 1 { 0 } else { make(&mut e, 0) })
            .collect();
        let b_maps: Vec<u32> = (0..8)
            .map(|i| if i == 1 { 0 } else { make(&mut e, 0) })
            .collect();
        let a = property(&mut e, &a_maps);
        let b = property(&mut e, &b_maps);
        assert_eq!(e.call(0x00a6_a5f0, &args![a, b]).u32() & 0xff, 1);
        // Slot 7 compares its float.
        e.mem.set_u32(b_maps[7] + 0x10, 0x3f80_0000);
        assert_eq!(e.call(0x00a6_a5f0, &args![a, b]).u32() & 0xff, 0);
        e.mem.set_u32(b_maps[7] + 0x10, 0);
        // Slot 5 compares its six floats.
        e.mem.set_u32(b_maps[5] + 0x20, 0x3f80_0000);
        assert_eq!(e.call(0x00a6_a5f0, &args![a, b]).u32() & 0xff, 0);
        e.mem.set_u32(b_maps[5] + 0x20, 0);
        // Only one side has shader maps.
        let shaders = e.mem.alloc(0x10);
        e.mem.set_u32(a + SHADER_MAPS, shaders);
        assert_eq!(e.call(0x00a6_a5f0, &args![a, b]).u32() & 0xff, 0);
        let other_shaders = e.mem.alloc(0x10);
        e.mem.set_u32(b + SHADER_MAPS, other_shaders);
        assert_eq!(e.call(0x00a6_a5f0, &args![a, b]).u32() & 0xff, 1);
        e.mem.set_u16(b + FLAGS, 1);
        assert_eq!(e.call(0x00a6_a5f0, &args![a, b]).u32() & 0xff, 0);
    }

    #[test]
    fn shader_map_array_constructor_allocates_its_elements() {
        let mut e = engine();
        e.register(ALLOCATE_BLOCK, |_, a| (0x0d00_0000 + a[0]).into_ret());
        let array = e.mem.alloc(0x10);
        e.mem.set_u16(array + 0xa, 9);
        assert_eq!(
            e.call(0x00a6_a990, &args![array, 0x1_0003u32, 2u32]).u32(),
            array
        );
        assert_eq!(e.mem.u32(array), SHADER_MAP_ARRAY_VTABLE);
        assert_eq!(e.mem.u32(array + 4), 0x0d00_000c);
        assert_eq!(
            (
                e.mem.u16(array + 8),
                e.mem.u16(array + 0xa),
                e.mem.u16(array + 0xe)
            ),
            (3, 0, 2)
        );
        e.call(0x00a6_a990, &args![array, 0x1_0000u32, 2u32]);
        assert_eq!(e.mem.u32(array + 4), 0);
    }

    #[test]
    fn save_binary_writes_flags_counts_and_present_maps() {
        let mut e = engine();
        e.register(BASE_SAVE_BINARY_00A5D7B0, |_, _| Ret::default());
        let map = object(&mut e, 0x10);
        let this = property(&mut e, &[map, 0]);
        let s = stream(&mut e);
        e.call_log = Some(vec![]);
        e.call(0x00a6_b250, &args![this, s]);
        let sizes: Vec<u32> = calls_to(&e, TRANSFER).iter().map(|w| w[2]).collect();
        // Flags, map count, two presence bytes, shader count (no array).
        assert_eq!(sizes, vec![2, 4, 1, 1, 4]);
        assert_eq!(calls_to(&e, SAVE_DOUBLE), vec![vec![map, s]]);
    }

    #[test]
    fn map_save_binary_writes_link_flags_and_transform_byte() {
        let mut e = engine();
        e.register(TRANSFORM_SAVE_BINARY_00A88F80, |_, _| Ret::default());
        let map = object(&mut e, 0x10);
        e.mem.set_u32(map + MAP_TEXTURE, 0x0abc_0000);
        let s = stream(&mut e);
        e.call_log = Some(vec![]);
        e.call(0x00a6_b830, &args![map, s]);
        assert_eq!(calls_to(&e, SAVE_LINK_DOUBLE), vec![vec![s, 0x0abc_0000]]);
        assert!(calls_to(&e, TRANSFORM_SAVE_BINARY_00A88F80).is_empty());
        e.mem.set_u32(map + MAP_TRANSFORM, 0x0def_0000);
        e.call(0x00a6_b830, &args![map, s]);
        assert_eq!(
            calls_to(&e, TRANSFORM_SAVE_BINARY_00A88F80),
            vec![vec![0x0def_0000, s]]
        );
        let sizes: Vec<u32> = calls_to(&e, TRANSFER).iter().map(|w| w[2]).collect();
        assert_eq!(sizes, vec![2, 1, 2, 1]);
    }

    #[test]
    fn derived_maps_load_and_save_their_extra_fields() {
        let mut e = engine();
        e.register(STREAM_00A645B0, |_, _| Ret::default());
        e.register(TRANSFORM_SAVE_BINARY_00A88F80, |_, _| Ret::default());
        let s = stream(&mut e);
        // A current version: the flag word, then no transform.
        e.mem.set_u32(s + 0xd8, 0x1402_0007);
        let map = object(&mut e, 0x28);
        e.call_log = Some(vec![]);
        e.call(0x00a6_b8c0, &args![map, s]);
        e.call(0x00a6_b9a0, &args![map, s]);
        e.call(0x00a6_ba80, &args![map, s]);
        e.call(0x00a6_bac0, &args![map, s]);
        let fields: Vec<(u32, u32)> = e
            .call_log
            .iter()
            .flatten()
            .filter(|(a, _)| *a == READ || *a == TRANSFER)
            .map(|(_, w)| w)
            .filter(|w| (map + 0x10..map + 0x28).contains(&w[1]))
            .map(|w| (w[1] - map, w[2]))
            .collect();
        let six: Vec<(u32, u32)> = (0..6).map(|i| (0x10 + 4 * i, 4)).collect();
        let mut expected = six.clone();
        expected.extend(six);
        expected.extend([(0x10, 4), (0x10, 4)]);
        assert_eq!(fields, expected);
    }

    #[test]
    fn viewer_strings_list_the_maps_and_shader_maps() {
        let mut e = engine();
        e.mem.set_u32(TEXTURING_PROPERTY_RTTI, 0x0104_0000);
        e.map(0x0109_c000, 0x1000);
        e.register(PROPERTY_GET_VIEWER_STRINGS, |_, _| Ret::default());
        e.register(NI_SPRINTF, |_, _| Ret::default());
        e.register(VIEWER_STRING_NAME, |_, a| (a[0] + 1).into_ret());
        e.register(VIEWER_STRING_SIGNED, |_, a| (0x0100_0000 + a[1]).into_ret());
        e.register(VIEWER_STRING_UNSIGNED, |_, a| {
            (0x0200_0000 + (a[1] & 0xffff)).into_ret()
        });
        e.register(VIEWER_STRING_FLOAT, |_, a| {
            (0x0300_0000 + (a[1] & 0xffff)).into_ret()
        });
        e.register(VIEWER_STRING_POINTER, |_, a| {
            (0x0400_0000 + (a[1] & 0xffff)).into_ret()
        });
        e.register(ALLOCATE_BLOCK, |_, a| (0x0500_0000 + a[0]).into_ret());
        e.register(STRING_ARRAY_SET_SIZE, |e, a| {
            let base = e.mem.alloc(4 * a[1]);
            let old = e.mem.u32(a[0] + 4);
            for i in 0..u32::from(e.mem.u16(a[0] + 0xa)) {
                let v = e.mem.u32(old + 4 * i);
                e.mem.set_u32(base + 4 * i, v);
            }
            e.mem.set_u32(a[0] + 4, base);
            e.mem.set_u16(a[0] + 8, a[1] as u16);
            Ret::default()
        });
        let add = |e: &mut Engine, a: &[u32], index: u32| {
            let base = e.mem.u32(a[0] + 4);
            let value = e.mem.u32(a[a.len() - 1]);
            e.mem.set_u32(base + 4 * index, value);
            e.mem.set_u16(a[0] + 0xa, index as u16 + 1);
        };
        e.register_double(SET_ELEMENT, move |e, a| {
            add(e, a, a[1]);
            Ret::default()
        });
        e.register_double(STRING_ARRAY_ADD_0096A610, move |e, a| {
            let index = u32::from(e.mem.u16(a[0] + 0xa));
            if index >= u32::from(e.mem.u16(a[0] + 8)) {
                let base = e.mem.alloc(4 * (index + 8));
                let old = e.mem.u32(a[0] + 4);
                for i in 0..index {
                    let v = e.mem.u32(old + 4 * i);
                    e.mem.set_u32(base + 4 * i, v);
                }
                e.mem.set_u32(a[0] + 4, base);
                e.mem.set_u16(a[0] + 8, index as u16 + 8);
            }
            add(e, a, index);
            Ret::default()
        });
        let strings = e.mem.alloc(0x10);
        let first_block = e.mem.alloc(4);
        e.mem.set_u32(strings + 4, first_block);
        e.mem.set_u16(strings + 8, 1);
        e.mem.set_u16(strings + 0xe, 4);
        // Slot 7 (one float) and slot 9 (a decal), then one shader map.
        let seven = object(&mut e, 0x14);
        e.mem.set_u32(seven + 0x10, 0x3f80_0007);
        e.mem.set_u16(seven + MAP_FLAGS, 0x2301);
        let decal = object(&mut e, 0x10);
        let mut maps = vec![0u32; 10];
        maps[7] = seven;
        maps[9] = decal;
        let this = property(&mut e, &maps);
        e.mem.set_u16(this + FLAGS, 0b1010);
        let shader = object(&mut e, 0x14);
        e.mem.set_u32(shader + 0x10, 0x55);
        let shaders = e.mem.alloc(0x10);
        let elements = e.mem.alloc(4);
        e.mem.set_u32(elements, shader);
        e.mem.set_u32(shaders + 4, elements);
        e.mem.set_u16(shaders + 0xa, 1);
        e.mem.set_u32(this + SHADER_MAPS, shaders);
        e.call(0x00a6_bdd0, &args![this, strings]);
        let count = u32::from(e.mem.u16(strings + 0xa));
        let base = e.mem.u32(strings + 4);
        let words: Vec<u32> = (0..count).map(|i| e.mem.u32(base + 4 * i)).collect();
        // The clamp and filter strings are buffers of the (empty) name length plus 0x14
        // and 0x1e.
        let tail = |coordinates: u32| {
            vec![
                0x0400_0000,
                0x0500_0014,
                0x0500_001e,
                0x0200_0000 + coordinates,
            ]
        };
        let mut expected = vec![0x0104_0001, 0x0100_0005, 0x0200_0007, 0x0300_0007];
        expected.extend(tail(1));
        expected.push(0x0200_0009);
        expected.extend(tail(0));
        expected.extend([0x0200_0000, 0x0200_0055]);
        expected.extend(tail(0));
        assert_eq!(words, expected);
    }
}

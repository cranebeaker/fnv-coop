//! `fallout shared/tesobjecttree.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 0051cfe0 (decompiled, FalloutNV.exe 1.4.0.525)
/// A forwarder: it hands `this` (`ECX`) to `00a5e930`, a destructor that stores the vtable
/// `0109b5ac`, releases the references that vtable slot +0xec returns for each of the
/// entries counted by the half-word at `+0xa6`, and tail-jumps into the base destructor
/// `00a5a390`, and returns nothing. Parameters: one word, `this`; the disassembly of the
/// function ends in a plain `RET` and passes only `ECX` on. `00a5e930` ends in `JMP
/// 00a5a390` (which itself ends in `JMP 00a5c240` and then `JMP 00a5d3d0`); no value is
/// computed for the caller and Ghidra types it void.
pub fn fn_0051cfe0(e: &mut Engine, this: Ptr) {
    e.call(0x00a5_e930, &args![this]);
}

/// `TESForm::LoadForm` (Xbox PDB name of `00485110`; `ECX` = form, the file).
const TES_FORM_LOAD_FORM: u32 = 0x0048_5110;
/// `TESFile::GetTESForm` (Xbox PDB name of `00472660`; `ECX` = file): the form
/// type of the record, in `AL`.
const GET_TES_FORM: u32 = 0x0047_2660;
/// `TESFile::GetTESChunk` (Xbox PDB name of `004726b0`; `ECX` = file).
const GET_TES_CHUNK: u32 = 0x0047_26b0;
/// `004726f0` (`ECX` = file): moves to the next chunk; true when there is one.
const NEXT_CHUNK: u32 = 0x0047_26f0;
/// The current chunk size (`00401660`, `TESFile` `+0x25C`).
const CHUNK_SIZE: u32 = 0x0040_1660;
/// `00401680` (`ECX` = file): the byte at `TESFile` `+0x299`, the swap flag.
const SWAP_FLAG: u32 = 0x0040_1680;
/// `TESFile::GetChunkData` (Xbox PDB overload `_ov3`, `00472890`): `thiscall`
/// (file; buffer, size).
const GET_CHUNK_DATA: u32 = 0x0047_2890;
/// `TESModel::LoadModelChunk` (`004892d0`, `cdecl`: model, file).
const LOAD_MODEL_CHUNK: u32 = 0x0048_92d0;
/// `TESTexture::LoadTextureChunk` (`0048e3d0`, `cdecl`: texture, file).
const LOAD_TEXTURE_CHUNK: u32 = 0x0048_e3d0;
/// `BGSDestructibleObjectForm::LoadChunk` (`004781e0`, `cdecl`: component, file).
const LOAD_DESTRUCTIBLE_CHUNK: u32 = 0x0047_81e0;
/// `00414290` (`ECX` = the `CNAM` block at `+0x6c`): its swap routine.
const SWAP_CNAM: u32 = 0x0041_4290;
/// `00401080` (`cdecl`: address, flag byte): byte-swaps the word at the address.
const SWAP_4_BYTES: u32 = 0x0040_1080;
/// `00401000` (`cdecl`, one word): allocates a block of that size.
const ALLOCATE: u32 = 0x0040_1000;
/// `00401030` (`cdecl`, one word): frees a block from `00401000`.
const FREE: u32 = 0x0040_1030;
/// `00403d30` (`cdecl`: destination, value, count): forwards to `_memset`.
const MEMSET: u32 = 0x0040_3d30;
/// `005e03d0` (`ECX` = the list at `+0x5c`): clears it before the `SNAM` entries.
const LIST_CLEAR: u32 = 0x005e_03d0;
/// `0096a610` (`ECX` = the list at `+0x5c`, one word: the address of an entry).
const LIST_ADD: u32 = 0x0096_a610;

/// The form type of the record this loader accepts (`0x25`).
const FORM_TYPE: u8 = 0x25;

/// Chunk ids, as the little-endian words of their four letters.
const CHUNK_CNAM: u32 = 0x4d41_4e43;
const CHUNK_BNAM: u32 = 0x4d41_4e42;
const CHUNK_SNAM: u32 = 0x4d41_4e53;
const CHUNK_DSTD: u32 = 0x4454_5344;
const CHUNK_DEST: u32 = 0x5453_4544;
const CHUNK_EDID: u32 = 0x4449_4445;
const CHUNK_OBND: u32 = 0x444e_424f;
const CHUNK_MODL: u32 = 0x4c44_4f4d;
const CHUNK_MODT: u32 = 0x5444_4f4d;
const CHUNK_ICON: u32 = 0x4e4f_4349;

/// The object's virtual slot called with the file for an `OBND` chunk.
const SLOT_LOAD_BOUNDS: u32 = 0xe0;
/// The object's virtual slot called with the text of an `EDID` chunk.
const SLOT_SET_EDITOR_ID: u32 = 0x134;
/// The most an `EDID` chunk is read with.
const EDITOR_ID_MAXIMUM: u32 = 0x200;

/// Offsets from `this` of the members the chunks go to.
const MODEL_OFFSET: u32 = 0x30;
const TEXTURE_OFFSET: u32 = 0x48;
const DESTRUCTIBLE_OFFSET: u32 = 0x54;
const LIST_OFFSET: u32 = 0x5c;
const CNAM_OFFSET: u32 = 0x6c;
/// The signed word the `CNAM` chunk handling raises to `0x80` when negative.
const CNAM_LIMIT_OFFSET: u32 = 0x80;
const BNAM_OFFSET: u32 = 0x8c;
const BNAM_SECOND_OFFSET: u32 = 0x90;

/// A member address, or null when the object is null.
fn component(object: u32, offset: u32) -> u32 {
    if object == 0 {
        0
    } else {
        object + offset
    }
}

// Translated from 0051bbc0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0051bbc0 - the /GS stack cookie check (00ec408c) is not translated.
/// `TESObjectTREE::Load` (Xbox PDB; `thiscall`, `RET 4`, `file`): the `Load`
/// slot of the virtual table `0102bc94` (the form type `0x25` constructor
/// `0051b6e0` stores it). Answers false when the record is not of form type
/// `0x25`; otherwise it runs `TESForm::LoadForm`, reads every chunk and answers
/// true:
/// - `MODL`, `MODT`: `TESModel::LoadModelChunk` on `this + 0x30`;
/// - `ICON`: `TESTexture::LoadTextureChunk` on `this + 0x48`;
/// - `DEST`, `DSTD`: `BGSDestructibleObjectForm::LoadChunk` on `this + 0x54`
///   (each of these three is null when `this` is null);
/// - `OBND`: virtual slot `0xe0` with the file; `EDID`: read into a stack buffer
///   (at most 0x200 bytes) and passed to virtual slot `0x134`;
/// - `CNAM`: only when the chunk is 0x20 bytes: read into `this + 0x6c`, swapped
///   by `00414290` when the swap flag of the file is set, and the signed word at
///   `+0x80` is set to `0x80` if negative;
/// - `BNAM`: only when the chunk is 8 bytes: read into `this + 0x8c`, and the
///   words at `+0x8c` and `+0x90` are byte-swapped when the swap flag is set;
/// - `SNAM`: when the size is a non-zero multiple of 4, the words are read into
///   a new block (zeroed first), the list at `this + 0x5c` is cleared, and each
///   non-zero word (swapped first when the swap flag is set) is added to it by
///   address (`0096a610`); the block is freed.
///
/// Other chunks are skipped. The stack cookie check is not translated.
pub fn tes_object_tree_load(e: &mut Engine, this: Ptr, file: Ptr) -> bool {
    let object = this.addr();
    if e.call(GET_TES_FORM, &args![file]).u8() != FORM_TYPE {
        return false;
    }
    e.call(TES_FORM_LOAD_FORM, &args![this, file]);
    loop {
        let chunk = e.call(GET_TES_CHUNK, &args![file]).u32();
        if chunk == 0 {
            break;
        }
        match chunk {
            CHUNK_CNAM => {
                let size = e.call(CHUNK_SIZE, &args![file]).u32();
                if size == 0x20 {
                    e.call(GET_CHUNK_DATA, &args![file, object + CNAM_OFFSET, 0x20u32]);
                    if e.call(SWAP_FLAG, &args![file]).bool() {
                        e.call(SWAP_CNAM, &args![object + CNAM_OFFSET]);
                    }
                    if (e.mem.u32(object + CNAM_LIMIT_OFFSET) as i32) < 0 {
                        e.mem.set_u32(object + CNAM_LIMIT_OFFSET, 0x80);
                    }
                }
            }
            CHUNK_BNAM => {
                let size = e.call(CHUNK_SIZE, &args![file]).u32();
                if size == 8 {
                    e.call(GET_CHUNK_DATA, &args![file, object + BNAM_OFFSET, 8u32]);
                    if e.call(SWAP_FLAG, &args![file]).bool() {
                        e.call(SWAP_4_BYTES, &args![object + BNAM_OFFSET, 0u32]);
                        e.call(SWAP_4_BYTES, &args![object + BNAM_SECOND_OFFSET, 0u32]);
                    }
                }
            }
            CHUNK_SNAM => {
                let size = e.call(CHUNK_SIZE, &args![file]).u32();
                if size != 0 && size % 4 == 0 {
                    let count = size >> 2;
                    // The allocation size is `count * 4`, which cannot overflow.
                    let block = e.call(ALLOCATE, &args![count * 4]).u32();
                    e.call(MEMSET, &args![block, 0u32, size]);
                    e.call(GET_CHUNK_DATA, &args![file, block, size]);
                    e.call(LIST_CLEAR, &args![object + LIST_OFFSET]);
                    for index in 0..count {
                        let entry = block + index * 4;
                        if e.call(SWAP_FLAG, &args![file]).bool() {
                            e.call(SWAP_4_BYTES, &args![entry, 0u32]);
                        }
                        if e.mem.u32(entry) != 0 {
                            e.call(LIST_ADD, &args![object + LIST_OFFSET, entry]);
                        }
                    }
                    e.call(FREE, &args![block]);
                }
            }
            CHUNK_EDID => {
                let size = e.call(CHUNK_SIZE, &args![file]).u32();
                // `_alloca_probe_16`: the buffer is the size rounded up to 16 bytes.
                let reserved = size.wrapping_add(15) & !15;
                e.with_stack(reserved, |e, buffer| {
                    e.call(GET_CHUNK_DATA, &args![file, buffer, EDITOR_ID_MAXIMUM]);
                    e.vcall(object, SLOT_SET_EDITOR_ID, &args![buffer]);
                });
            }
            CHUNK_OBND => {
                e.vcall(object, SLOT_LOAD_BOUNDS, &args![file]);
            }
            CHUNK_MODL | CHUNK_MODT => {
                let model = component(object, MODEL_OFFSET);
                e.call(LOAD_MODEL_CHUNK, &args![model, file]);
            }
            CHUNK_ICON => {
                let texture = component(object, TEXTURE_OFFSET);
                e.call(LOAD_TEXTURE_CHUNK, &args![texture, file]);
            }
            CHUNK_DSTD | CHUNK_DEST => {
                let target = component(object, DESTRUCTIBLE_OFFSET);
                e.call(LOAD_DESTRUCTIBLE_CHUNK, &args![target, file]);
            }
            _ => {}
        }
        if !e.call(NEXT_CHUNK, &args![file]).bool() {
            break;
        }
    }
    true
}

// Translated from 0051d150 (PC disassembly, FalloutNV.exe 1.4.0.525).
/// Answers the constant address 0120044c without reading the receiver. The map names this body `BSShaderTextureSet::GetRTTI`; it takes only `this` (plain `RET`).
pub fn fn_0051d150(_e: &mut Engine, _this: Ptr) -> u32 {
    0x0120_044c
}

// Translated from 0051d1b0 (PC disassembly, FalloutNV.exe 1.4.0.525).
/// Answers the constant address 01200444 without reading the receiver. The map names this body `BSTextureSet::GetRTTI`; it takes only `this` (plain `RET`).
pub fn fn_0051d1b0(_e: &mut Engine, _this: Ptr) -> u32 {
    0x0120_0444
}

/// `00a5ecb0` (`NiMain/ninode.cpp`, `thiscall`, one word): the `NiNode`
/// constructor body the billboard constructors run first.
const NI_NODE_CONSTRUCT: u32 = 0x00a5_ecb0;
/// The vtable `0102bf44` (RTTI `.?AVNiBillboardNode@@`).
const BILLBOARD_NODE_VTABLE: u32 = 0x0102_bf44;
/// `NiBillboardNode`'s flag word (`u16`; the low three bits are the mode,
/// bit 3 a flag) and its float at `+0xb0`.
const BILLBOARD_FLAGS_OFFSET: u32 = 0xac;
const BILLBOARD_FLOAT_OFFSET: u32 = 0xb0;
/// `00aa1460` (`cdecl`: block, size): gives back a block of the given size.
const FREE_SIZED: u32 = 0x00aa_1460;
/// `00a5d3a0` (`NiMain/niobject.cpp`, `thiscall`): the base constructor
/// `BSTextureSet`'s constructor runs.
const NI_OBJECT_CONSTRUCT: u32 = 0x00a5_d3a0;
/// The vtables `0102c0ec` (RTTI `.?AVBSTextureSet@@`) and `0102c04c` (RTTI
/// `.?AVBSShaderTextureSet@@`).
const TEXTURE_SET_VTABLE: u32 = 0x0102_c0ec;
const SHADER_TEXTURE_SET_VTABLE: u32 = 0x0102_c04c;
/// `00ec782f` (`cdecl`: array, element size, count, constructor,
/// destructor): the compiler's array constructor helper.
const ARRAY_CONSTRUCT: u32 = 0x00ec_782f;
/// `00ec5fce` (`eh vector destructor iterator`, `cdecl`: array, element
/// size, count, destructor).
const ARRAY_DESTRUCT: u32 = 0x00ec_5fce;
/// The 8-byte string's constructor (`004037b0`) and destructor (`004037d0`),
/// passed to the array helpers by address.
const STRING_CONSTRUCT: u32 = 0x0040_37b0;
const STRING_DESTRUCT: u32 = 0x0040_37d0;
/// `004a7090` (`thiscall`): the base destructor `BSShaderTextureSet`'s
/// destructor ends with.
const TEXTURE_SET_BASE_DESTRUCT: u32 = 0x004a_7090;
/// Where `BSShaderTextureSet`'s six strings start, and their size and count.
const TEXTURE_NAMES_OFFSET: u32 = 0x8;
const TEXTURE_NAME_SIZE: u32 = 8;
const TEXTURE_NAME_COUNT: u32 = 6;
/// The text `GetBillboardTextureFilename` starts the name with (`0102bed0`).
const BILLBOARD_TEXTURE_PREFIX: u32 = 0x0102_bed0;
/// The model component's virtual slot that answers the model's file name.
const SLOT_MODEL_NAME: u32 = 0x14;
/// The three doubles `0051d250` compares the two `BNAM` floats with.
const BNAM_LOWER: u32 = 0x0101_2060;
const BNAM_FIRST_UPPER: u32 = 0x0101_79e0;
const BNAM_SECOND_UPPER: u32 = 0x0102_c188;

/// The address `offset` bytes into the object at `object`.
fn field_addr(object: Ptr, offset: u32) -> u32 {
    object.addr().wrapping_add(offset)
}

// Translated from 0051cf00 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0051cf00 - the C++ exception-unwinding frame (FS:[0]) is not translated.
/// An `NiBillboardNode` constructor (unnamed in the engine map; `thiscall`,
/// plain `RET`): runs the `NiNode` constructor body `00a5ecb0` with 0, stores
/// the `NiBillboardNode` vtable `0102bf44`, clears the flag word at `+0xac`
/// and stores 0.0 at `+0xb0`, then sets the mode bits to 1 (`0051d000`) and
/// flag bit 3 (`0051d060` with 1). Returns `this`.
pub fn fn_0051cf00(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(NI_NODE_CONSTRUCT, &args![this, 0u32]);
    e.mem.set_u32(this.addr(), BILLBOARD_NODE_VTABLE);
    e.mem.set_u16(field_addr(this, BILLBOARD_FLAGS_OFFSET), 0);
    // FLDZ; FSTP float.
    e.mem
        .set_u32(field_addr(this, BILLBOARD_FLOAT_OFFSET), x87_store32(0.0));
    fn_0051d000(e, this, 1);
    fn_0051d060(e, this, 1);
    this
}

// Translated from 0051d000 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`): sets the low three bits
/// of the billboard flag word at `+0xac` to `mode` (`0051d020` with mask 7,
/// shift 0).
pub fn fn_0051d000(e: &mut Engine, this: Ptr, mode: u16) {
    fn_0051d020(e, this, mode, 7, 0);
}

// Translated from 0051d020 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 0xc`): the flag word at
/// `+0xac` becomes `(flags & !mask) | (value << shift)`, kept to 16 bits; the
/// shift count is taken modulo 32 as `SHL` does.
pub fn fn_0051d020(e: &mut Engine, this: Ptr, value: u16, mask: u16, shift: u16) {
    let at = field_addr(this, BILLBOARD_FLAGS_OFFSET);
    let kept = u32::from(e.mem.u16(at)) & !u32::from(mask);
    let bits = u32::from(value) << (u32::from(shift) & 31);
    e.mem.set_u16(at, (kept | bits) as u16);
}

// Translated from 0051d060 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`): sets (`on` non-zero) or
/// clears bit 3 of the flag word at `+0xac` (`0051d080` with 8).
pub fn fn_0051d060(e: &mut Engine, this: Ptr, on: u8) {
    fn_0051d080(e, this, on, 8);
}

// Translated from 0051d080 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 8`): ORs `bits` into the flag
/// word at `+0xac` when `on` is non-zero, clears them otherwise.
pub fn fn_0051d080(e: &mut Engine, this: Ptr, on: u8, bits: u16) {
    let at = field_addr(this, BILLBOARD_FLAGS_OFFSET);
    let flags = e.mem.u16(at);
    if on != 0 {
        e.mem.set_u16(at, flags | bits);
    } else {
        e.mem.set_u16(at, flags & !bits);
    }
}

// Translated from 0051d190 (decompiled, FalloutNV.exe 1.4.0.525)
/// A `BSTextureSet` constructor (unnamed in the engine map; `thiscall`, plain
/// `RET`): runs the base constructor `00a5d3a0` and stores the vtable
/// `0102c0ec` (RTTI `.?AVBSTextureSet@@`). Returns `this`.
pub fn fn_0051d190(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(NI_OBJECT_CONSTRUCT, &args![this]);
    e.mem.set_u32(this.addr(), TEXTURE_SET_VTABLE);
    this
}

// Translated from 0051d0d0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0051d0d0 - the C++ exception-unwinding frame (FS:[0]) is not translated.
/// `BSShaderTextureSet::BSShaderTextureSet` (Xbox PDB; `thiscall`, plain
/// `RET`): the `BSTextureSet` constructor (`0051d190`), the vtable `0102c04c`,
/// then the array helper `00ec782f` constructs the six 8-byte strings at
/// `+0x8` (constructor `004037b0`, destructor `004037d0`). Returns `this`.
pub fn bs_shader_texture_set_bs_shader_texture_set(e: &mut Engine, this: Ptr) -> Ptr {
    fn_0051d190(e, this);
    e.mem.set_u32(this.addr(), SHADER_TEXTURE_SET_VTABLE);
    e.call(
        ARRAY_CONSTRUCT,
        &args![
            field_addr(this, TEXTURE_NAMES_OFFSET),
            TEXTURE_NAME_SIZE,
            TEXTURE_NAME_COUNT,
            STRING_CONSTRUCT,
            STRING_DESTRUCT
        ],
    );
    this
}

// Translated from 0051cf90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBillboardNode::GetRTTI` (Xbox PDB; `thiscall`, plain `RET`): answers
/// the constant address `011f4aa0` without reading `this`.
pub fn ni_billboard_node_get_rtti(_e: &mut Engine, _this: Ptr) -> u32 {
    0x011f_4aa0
}

// Translated from 0051d160 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSShaderTextureSet::scalar deleting destructor` (Xbox PDB; `thiscall`,
/// `RET 4`): the destructor `0051d1c0`, then, when bit 0 of `flags` is set,
/// gives the 0x38-byte object back (`00aa1460`). Returns `this`.
pub fn bs_shader_texture_set_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    fn_0051d1c0(e, this);
    if flags & 1 != 0 {
        e.call(FREE_SIZED, &args![this, 0x38u32]);
    }
    this
}

// Translated from 0051d1c0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0051d1c0 - the C++ exception-unwinding frame (FS:[0]) is not translated.
/// The `BSShaderTextureSet` destructor (unnamed in the engine map;
/// `thiscall`, plain `RET`): destroys the six 8-byte strings at `+0x8` with
/// the array helper `00ec5fce` (destructor `004037d0`), then runs the base
/// destructor `004a7090`.
pub fn fn_0051d1c0(e: &mut Engine, this: Ptr) {
    e.call(
        ARRAY_DESTRUCT,
        &args![
            field_addr(this, TEXTURE_NAMES_OFFSET),
            TEXTURE_NAME_SIZE,
            TEXTURE_NAME_COUNT,
            STRING_DESTRUCT
        ],
    );
    e.call(TEXTURE_SET_BASE_DESTRUCT, &args![this]);
}

// Translated from 0051c3b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectTREE::GetBillboardTextureFilename` (Xbox PDB; `thiscall`,
/// `RET 4`): writes into `buffer` the text at `0102bed0`, then the model's
/// file name (virtual slot `0x14` of the model component at `this + 0x30`)
/// up to its first `.` or its end; when it stopped at a `.` it appends
/// `.dds`. The result is terminated with a zero byte.
pub fn tes_object_tree_get_billboard_texture_filename(e: &mut Engine, this: Ptr, buffer: Ptr) {
    let mut out = buffer.addr();
    let mut prefix = BILLBOARD_TEXTURE_PREFIX;
    while e.mem.u8(prefix) != 0 {
        let byte = e.mem.u8(prefix);
        e.mem.set_u8(out, byte);
        prefix = prefix.wrapping_add(1);
        out = out.wrapping_add(1);
    }
    let mut name = e
        .vcall(field_addr(this, MODEL_OFFSET), SLOT_MODEL_NAME, &[])
        .u32();
    while e.mem.u8(name) != b'.' && e.mem.u8(name) != 0 {
        let byte = e.mem.u8(name);
        e.mem.set_u8(out, byte);
        name = name.wrapping_add(1);
        out = out.wrapping_add(1);
    }
    if e.mem.u8(name) == b'.' {
        for byte in *b".dds" {
            e.mem.set_u8(out, byte);
            out = out.wrapping_add(1);
        }
    }
    e.mem.set_u8(out, 0);
}

// Translated from 0051d250 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, plain `RET`): tests the two `BNAM`
/// floats at `+0x8c` and `+0x90` against the doubles at `01012060` (lower),
/// `010179e0` and `0102c188` (uppers). False when the first is above the
/// lower and at most its upper; otherwise true unless the second is above
/// the lower and at most its upper. A NaN compares as the exe's flag tests
/// do: it is never "above" nor "at most".
pub fn fn_0051d250(e: &mut Engine, this: Ptr) -> bool {
    let lower: f64 = e.global(BNAM_LOWER);
    let first = x87_load32(e.mem.u32(field_addr(this, BNAM_OFFSET)));
    if first > lower && first <= e.global::<f64>(BNAM_FIRST_UPPER) {
        return false;
    }
    let second = x87_load32(e.mem.u32(field_addr(this, BNAM_SECOND_OFFSET)));
    if !(second > lower) {
        return true;
    }
    !(second <= e.global::<f64>(BNAM_SECOND_UPPER))
}

// Translated from 0051cfa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBillboardNode::scalar deleting destructor` (Xbox PDB; `thiscall`,
/// `RET 4`): the destructor `0051cfe0`, then, when bit 0 of `flags` is set,
/// gives the 0xb4-byte object back (`00aa1460`). Returns `this`.
pub fn ni_billboard_node_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_0051cfe0(e, this);
    if flags & 1 != 0 {
        e.call(FREE_SIZED, &args![this, 0xb4u32]);
    }
    this
}

/// `TESForm::StartForm` (`004855a0`) and `TESForm::CloseForm` (`00485680`),
/// `thiscall` on the form, no stack words.
const START_FORM: u32 = 0x0048_55a0;
const CLOSE_FORM: u32 = 0x0048_5680;
/// The component saves (engine map; `thiscall`): `TESModel::Save`
/// (`004891d0`, two chunk tags), `TESTexture::Save` (`0048e370`, a tag) and
/// `BGSDestructibleObjectForm::Save` (`00477f20`).
const MODEL_SAVE: u32 = 0x0048_91d0;
const TEXTURE_SAVE: u32 = 0x0048_e370;
const DESTRUCTIBLE_SAVE: u32 = 0x0047_7f20;
/// `TESForm::AddChunkArray_ov2` (`00485710`, cdecl(chunk, data, count)) and
/// `TESForm::__AddChunkData` (`00485990`, cdecl(chunk, data, size)).
const ADD_CHUNK_ARRAY_OV2: u32 = 0x0048_5710;
const ADD_CHUNK_DATA: u32 = 0x0048_5990;
/// `00401500`: the byte at `011c54ba` (whether saved words are byte-swapped).
const SAVE_SWAPS_BYTES: u32 = 0x0040_1500;
/// The tree's word list at `+0x5c` (`thiscall` on it): its count
/// (`00658930`), the address of an element (`00877a30`, `RET 4`: the index)
/// and the add of an element by address (`009938d0`, `RET 4`).
const LIST_COUNT: u32 = 0x0065_8930;
const LIST_ELEMENT: u32 = 0x0087_7a30;
const LIST_APPEND: u32 = 0x0099_38d0;
/// `__RTDynamicCast` (`00ec43fb`, cdecl, five words) and the type
/// descriptors `TESForm` (`01183028`) and `TESObjectTREE` (`01186060`).
const RT_DYNAMIC_CAST: u32 = 0x00ec_43fb;
const TYPE_TES_FORM: u32 = 0x0118_3028;
const TYPE_TES_OBJECT_TREE: u32 = 0x0118_6060;
/// `TESForm::CopyAllComponents` (`004851b0`, `RET 4`) and
/// `TESForm::CompareAllComponents` (`00485270`, `RET 4`).
const COPY_ALL_COMPONENTS: u32 = 0x0048_51b0;
const COMPARE_ALL_COMPONENTS: u32 = 0x0048_5270;
/// `_memcmp` (`00ec4835`, cdecl) and `memcpy(dest, source, count)`
/// (`00401460`, cdecl).
const MEMCMP: u32 = 0x00ec_4835;
const MEMCPY: u32 = 0x0040_1460;
/// The tree's vtable slot `+0x18c` (no stack words): the address of its
/// 0x20 bytes of `CNAM` data.
const SLOT_CNAM_DATA: u32 = 0x18c;
/// `00452df0` (`thiscall` on a float pair, `RET 4`: the other pair):
/// whether they are equal.
const PAIR_EQUALS: u32 = 0x0045_2df0;
/// The random number (`00487f50`).
const RANDOM: u32 = 0x0048_7f50;
/// `00664840` (cdecl, a byte): the object whose `+0xc` string `0051cee0`
/// answers the first word of (`00559450`).
const OBJECT_00664840: u32 = 0x0066_4840;
const FIRST_WORD: u32 = 0x0055_9450;
/// `004a7090` (`thiscall`): `BSTextureSet`'s destructor body.
const TEXTURE_SET_DESTRUCT: u32 = 0x004a_7090;
/// The compiler's array constructor (`00401050`, `stdcall`: array, element
/// size, count, constructor), the do-nothing element constructors it is
/// given (`006815c0`, `004a7800`), `NiAlloc` (`00aa1070`, cdecl) and the
/// `NiObject` allocation (`00aa13e0`, cdecl).
const ARRAY_CONSTRUCT_STDCALL: u32 = 0x0040_1050;
const POINT_CONSTRUCT: u32 = 0x0068_15c0;
const COLOR_CONSTRUCT: u32 = 0x004a_7800;
const NI_ALLOC: u32 = 0x00aa_1070;
const NI_NEW: u32 = 0x00aa_13e0;
/// The scope guard the game keeps on its stack (`00404eb0`: tag, flag,
/// file, line; `00404ee0`), its tag and the source file
/// (`"D:\_Fallout3\...\TESObjectTREE.cpp"`).
const SCOPE_GUARD_CONSTRUCT: u32 = 0x0040_4eb0;
const SCOPE_GUARD_DESTRUCT: u32 = 0x0040_4ee0;
const SCOPE_GUARD_TAG: u32 = 0x23;
const TREE_SOURCE_FILE: u32 = 0x0102_bef0;
/// Vector, texture-coordinate and colour setters (`thiscall` on a stack
/// temporary, answering it): `00416870` (`RET 0xc`, three floats),
/// `00452dc0` (`RET 8`, two floats), `00414430` (`RET 0x10`, four floats).
const POINT_SET: u32 = 0x0041_6870;
const TEXTURE_COORDINATE_SET: u32 = 0x0045_2dc0;
const COLOR_SET: u32 = 0x0041_4430;
/// The size used when the tree's `+0x90` float is not above 0.0 (1400.0 at
/// `0102beec`), the 0.0 it is compared with (`01012060`), the 2.0 it is
/// divided by (`01011590`) and the -1.0 of the normals (`01012054`).
const BILLBOARD_DEFAULT_SIZE: u32 = 0x0102_beec;
const ZERO_DOUBLE: u32 = 0x0101_2060;
const TWO_DOUBLE: u32 = 0x0101_1590;
const MINUS_ONE: u32 = 0x0101_2054;
/// `NiTriShapeData::NiTriShapeData` (`00a7b630`, `thiscall`, nine words),
/// `NiTriShape::NiTriShape_ov2` (`00a74480`, `RET 4`), the `NiPointer`
/// constructor (`00633c90`, `RET 4`), assignment (`006e5cc0`, `RET 4`),
/// release (`0045cec0`) and comparison (`00822510`, `RET 4`: whether it
/// holds the given object).
const TRI_SHAPE_DATA_CONSTRUCT: u32 = 0x00a7_b630;
const TRI_SHAPE_CONSTRUCT: u32 = 0x00a7_4480;
const POINTER_CONSTRUCT: u32 = 0x0063_3c90;
const POINTER_ASSIGN: u32 = 0x006e_5cc0;
const POINTER_RELEASE: u32 = 0x0045_cec0;
const POINTER_HOLDS: u32 = 0x0082_2510;
/// `BuildDistant3D`'s callees: `NiAVObject::AttachProperty` (`00439410`,
/// `RET 4`), the property constructor `00ba8630`,
/// `BSShaderPPLightingProperty::LoadTextureSet` (`00b66ad0`, `RET 8`) and
/// the word at `0102bf34` it is given.
const ATTACH_PROPERTY: u32 = 0x0043_9410;
const LIGHTING_PROPERTY_CONSTRUCT: u32 = 0x00ba_8630;
const LOAD_TEXTURE_SET: u32 = 0x00b6_6ad0;
const LOAD_TEXTURE_SET_ARGUMENT: u32 = 0x0102_bf34;
/// `CreateDistant3D`'s callees: `0051d740` (no arguments), `00b58720`
/// (cdecl, the form id), `00af4200` (cdecl: name, buffer, size),
/// `BSTexturePalette::GetTexture_ov2` (`00a61b90`, cdecl: name, pointer),
/// the model loader (the word at `011c3b3c`) and its `00447980` (`RET 8`),
/// the record constructor `00b57e70` and `00b59ae0` (cdecl, seven words).
const DISTANT_CHECK_0051D740: u32 = 0x0051_d740;
const DISTANT_FORM_CHECK: u32 = 0x00b5_8720;
const TEXTURE_PATH: u32 = 0x00af_4200;
const GET_TEXTURE: u32 = 0x00a6_1b90;
const MODEL_LOADER: u32 = 0x011c_3b3c;
const MODEL_LOADER_QUEUE: u32 = 0x0044_7980;
const DISTANT_RECORD_CONSTRUCT: u32 = 0x00b5_7e70;
const DISTANT_ADD: u32 = 0x00b5_9ae0;
/// The form-id getter (`0084e3a0`, `thiscall`: the word at +0xc).
const FORM_ID: u32 = 0x0084_e3a0;

// Translated from 0051bfb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectTREE::Save` (engine map; no stack words): `StartForm`, the
/// model (+0x30; `MODL`, `MODT`), the icon (+0x48, `ICON`), the destructible
/// data (+0x54); with words in the list at +0x5c, a block of `4 * count`
/// bytes (all ones on overflow; zeroed; never freed) gets each non-zero
/// word at its index and is added as `SNAM` (`AddChunkArray_ov2` with the
/// count). A negative word at +0x80 becomes 0x80. The 0x20 bytes at +0x6c
/// as `CNAM` (byte-swapped before and back after, `00414290`, when
/// `00401500` says so), the 2 words at +0x8c as `BNAM` (`AddChunkArray_ov2`
/// with 2), `CloseForm`.
pub fn tes_object_tree_save(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.call(START_FORM, &args![this]);
    e.call(
        MODEL_SAVE,
        &args![base + MODEL_OFFSET, CHUNK_MODL, CHUNK_MODT],
    );
    e.call(TEXTURE_SAVE, &args![base + TEXTURE_OFFSET, CHUNK_ICON]);
    e.call(DESTRUCTIBLE_SAVE, &args![base + DESTRUCTIBLE_OFFSET]);
    let list = base + LIST_OFFSET;
    if e.call(LIST_COUNT, &args![list]).u32() != 0 {
        let count = e.call(LIST_COUNT, &args![list]).u32();
        let bytes = count << 2;
        let (size, overflow) = count.overflowing_mul(4);
        let request = if overflow { 0xffff_ffff } else { size };
        let block = e.call(ALLOCATE, &args![request]).u32();
        e.call(MEMSET, &args![block, 0u32, bytes]);
        let mut index = 0u32;
        while index < count {
            let at = e.call(LIST_ELEMENT, &args![list, index]).u32();
            let word = e.mem.u32(at);
            if word != 0 {
                e.mem
                    .set_u32(block.wrapping_add(index.wrapping_mul(4)), word);
            }
            index += 1;
        }
        e.call(ADD_CHUNK_ARRAY_OV2, &args![CHUNK_SNAM, block, count]);
    }
    if e.mem.i32(base + CNAM_LIMIT_OFFSET) < 0 {
        e.mem.set_u32(base + CNAM_LIMIT_OFFSET, 0x80);
    }
    if e.call(SAVE_SWAPS_BYTES, &[]).u32() as u8 != 0 {
        e.call(SWAP_CNAM, &args![base + CNAM_OFFSET]);
    }
    e.call(
        ADD_CHUNK_DATA,
        &args![CHUNK_CNAM, base + CNAM_OFFSET, 0x20u32],
    );
    if e.call(SAVE_SWAPS_BYTES, &[]).u32() as u8 != 0 {
        e.call(SWAP_CNAM, &args![base + CNAM_OFFSET]);
    }
    e.call(
        ADD_CHUNK_ARRAY_OV2,
        &args![CHUNK_BNAM, base + BNAM_OFFSET, 2u32],
    );
    e.call(CLOSE_FORM, &args![this]);
}

/// `other` as a `TESObjectTREE` (`__RTDynamicCast` from `TESForm`), or 0.
fn as_tree(e: &mut Engine, other: u32) -> u32 {
    e.call(
        RT_DYNAMIC_CAST,
        &args![other, 0u32, TYPE_TES_FORM, TYPE_TES_OBJECT_TREE, 0u32],
    )
    .u32()
}

// Translated from 0051c140 (decompiled, FalloutNV.exe 1.4.0.525)
/// The tree's copy (engine map: no name; `RET 4`): when `other` is a
/// `TESObjectTREE`, copies all components (`TESForm::CopyAllComponents`
/// with `other`), empties this word list (`005e03d0`) and appends each of
/// the other's elements (`009938d0` with its address), copies the 0x20
/// bytes the other's vtable slot `+0x18c` points at into +0x6c and the two
/// `BNAM` words (+0x8c, +0x90).
pub fn fn_0051c140(e: &mut Engine, this: Ptr, other: u32) {
    let source = as_tree(e, other);
    if source == 0 {
        return;
    }
    e.call(COPY_ALL_COMPONENTS, &args![this, other]);
    let count = e.call(LIST_COUNT, &args![source + LIST_OFFSET]).u32();
    e.call(LIST_CLEAR, &args![this.addr() + LIST_OFFSET]);
    let mut index = 0u32;
    while index < count {
        let element = e
            .call(LIST_ELEMENT, &args![source + LIST_OFFSET, index])
            .u32();
        e.call(LIST_APPEND, &args![this.addr() + LIST_OFFSET, element]);
        index += 1;
    }
    let data = e.vcall(source, SLOT_CNAM_DATA, &[]).u32();
    for offset in (0..0x20).step_by(4) {
        let word = e.mem.u32(data + offset);
        e.mem.set_u32(this.addr() + CNAM_OFFSET + offset, word);
    }
    let first = e.mem.u32(source + BNAM_OFFSET);
    let second = e.mem.u32(source + BNAM_SECOND_OFFSET);
    e.mem.set_u32(this.addr() + BNAM_OFFSET, first);
    e.mem.set_u32(this.addr() + BNAM_SECOND_OFFSET, second);
}

// Translated from 0051c220 (decompiled, FalloutNV.exe 1.4.0.525)
/// The tree's compare (engine map: no name; `RET 4`): true when `other` is
/// no `TESObjectTREE`; when the `CNAM` data differs (`_memcmp` of 0x20 bytes
/// with the other's vtable slot `+0x18c`); when the `BNAM` pairs differ
/// (`fn_0051c330`); when the word lists differ in count or in any word; or
/// when `TESForm::CompareAllComponents` (with the cast form) says so.
pub fn fn_0051c220(e: &mut Engine, this: Ptr, other: u32) -> bool {
    let source = as_tree(e, other);
    if source == 0 {
        return true;
    }
    let data = e.vcall(source, SLOT_CNAM_DATA, &[]).u32();
    if e.call(MEMCMP, &args![this.addr() + CNAM_OFFSET, data, 0x20u32])
        .u32()
        != 0
    {
        return true;
    }
    if fn_0051c330(e, Ptr::new(this.addr() + BNAM_OFFSET), source + BNAM_OFFSET) {
        return true;
    }
    let count = e.call(LIST_COUNT, &args![this.addr() + LIST_OFFSET]).u32();
    if count != e.call(LIST_COUNT, &args![source + LIST_OFFSET]).u32() {
        return true;
    }
    let mut index = 0u32;
    while index < count {
        let mine = e
            .call(LIST_ELEMENT, &args![this.addr() + LIST_OFFSET, index])
            .u32();
        let theirs = e
            .call(LIST_ELEMENT, &args![source + LIST_OFFSET, index])
            .u32();
        if e.mem.u32(mine) != e.mem.u32(theirs) {
            return true;
        }
        index += 1;
    }
    e.call(COMPARE_ALL_COMPONENTS, &args![this, source]).u32() as u8 != 0
}

// Translated from 0051c330 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether two float pairs differ (engine map: no name; `RET 4`): the
/// negation of `00452df0`.
pub fn fn_0051c330(e: &mut Engine, this: Ptr, other: u32) -> bool {
    e.call(PAIR_EQUALS, &args![this, other]).u32() as u8 == 0
}

// Translated from 0051c360 (decompiled, FalloutNV.exe 1.4.0.525)
/// A random word of the tree's list (engine map: no name; no stack words):
/// 0 when the list is empty, else the word at `00487f50() % count`.
pub fn fn_0051c360(e: &mut Engine, this: Ptr) -> u32 {
    let list = this.addr() + LIST_OFFSET;
    let count = e.call(LIST_COUNT, &args![list]).u32();
    if count == 0 {
        return 0;
    }
    let index = e.call(RANDOM, &[]).u32() % count;
    let at = e.call(LIST_ELEMENT, &args![list, index]).u32();
    e.mem.u32(at)
}

/// The constant the exe loads for each of the billboard's twelve normals'
/// floats: 0.0 (`FLDZ`), 1.0 (`FLD1`) or the -1.0 at `01012054`.
#[derive(Clone, Copy)]
enum NormalFloat {
    Zero,
    One,
    MinusOne,
}

/// The billboard's normals, in the exe's order: four (0, 1, 0), four
/// (0, 0, -1), four (1, 0, 0).
const BILLBOARD_NORMALS: [NormalFloat; 36] = {
    use NormalFloat::{MinusOne as M, One as I, Zero as Z};
    [
        Z, I, Z, Z, I, Z, Z, I, Z, Z, I, Z, Z, Z, M, Z, Z, M, Z, Z, M, Z, Z, M, I, Z, Z, I, Z, Z,
        I, Z, Z, I, Z, Z,
    ]
};

/// `00416870` on `temporary` with three float words; its answer's three
/// words are copied to `to`.
fn set_point(e: &mut Engine, temporary: u32, to: u32, (x, y, z): (u32, u32, u32)) {
    let at = e.call(POINT_SET, &args![temporary, x, y, z]).u32();
    for offset in [0u32, 4, 8] {
        let word = e.mem.u32(at + offset);
        e.mem.set_u32(to + offset, word);
    }
}

/// `00401000(size)` and, when that is not null, `00401050(block, element,
/// count, constructor)`; the block (0 when the allocation failed).
fn new_array(e: &mut Engine, size: u32, element: u32, count: u32, constructor: u32) -> u32 {
    let block = e.call(ALLOCATE, &args![size]).u32();
    if block != 0 {
        e.call(
            ARRAY_CONSTRUCT_STDCALL,
            &args![block, element, count, constructor],
        );
        block
    } else {
        0
    }
}

// Translated from 0051c490 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0051c490 - the FS:[0] exception frame (it frees the arrays and releases the pointer if a step throws) is not translated.
/// `TESObjectTREE::CreateBillboardGeomData` (engine map; `RET 8`: a byte,
/// the address of an `NiPointer`): inside a scope guard (tag 0x23, line
/// 0x199), builds a four-vertex quad of the tree's size (the float at
/// +0x90; 1400.0 when not above 0.0): its vertices (-half, 0, 0), then
/// (-half, 0, size) and (half, 0, size) when `upright` is set, else
/// (-half, size, 0) and (half, size, 0), then (half, 0, 0); twelve normals
/// copied from the stack table (`BILLBOARD_NORMALS`); texture coordinates
/// (1, 1), (1, 0), (0, 0), (0, 1); four colours (1, 1, 1, 0); the triangles
/// 0, 2, 1 and 0, 3, 2 (`NiAlloc`). An `NiTriShapeData` (0x58 bytes) made of
/// them (4 vertices, 1, 0x1000, 2 triangles) is stored into `*geometry`
/// through a temporary `NiPointer`. Each array is `00401000` plus the
/// compiler's array constructor; the stack temporaries the vector setters
/// fill are one block here.
pub fn tes_object_tree_create_billboard_geom_data(
    e: &mut Engine,
    this: Ptr,
    upright: u8,
    geometry: u32,
) {
    e.with_stack(0xc8, |e, locals| {
        let guard = locals.addr();
        let pointer = locals.addr() + 4;
        let temporary = locals.addr() + 8;
        let normals_table = locals.addr() + 0x38;
        e.call(
            SCOPE_GUARD_CONSTRUCT,
            &args![guard, SCOPE_GUARD_TAG, 1u32, TREE_SOURCE_FILE, 0x199u32],
        );
        let minus_one = x87_quiet_bits(e.global::<u32>(MINUS_ONE));
        for (i, value) in BILLBOARD_NORMALS.iter().enumerate() {
            let bits = match value {
                NormalFloat::Zero => 0,
                NormalFloat::One => 1.0f32.to_bits(),
                NormalFloat::MinusOne => minus_one,
            };
            e.mem.set_u32(normals_table + 4 * i as u32, bits);
        }
        let vertices = new_array(e, 0x30, 0xc, 4, POINT_CONSTRUCT);
        let mut size = e.mem.u32(this.addr() + BNAM_SECOND_OFFSET);
        if x87_load32(size) <= e.global::<f64>(ZERO_DOUBLE) {
            size = x87_quiet_bits(e.global::<u32>(BILLBOARD_DEFAULT_SIZE));
        }
        let half = x87_store32(x87_div(x87_load32(size), e.global::<f64>(TWO_DOUBLE)));
        let minus_half = x87_store32(-x87_load32(half));
        let size = x87_quiet_bits(size);
        let half = x87_quiet_bits(half);
        set_point(e, temporary, vertices, (minus_half, 0, 0));
        set_point(e, temporary, vertices + 0x24, (half, 0, 0));
        if upright != 0 {
            set_point(e, temporary, vertices + 0xc, (minus_half, 0, size));
            set_point(e, temporary, vertices + 0x18, (half, 0, size));
        } else {
            set_point(e, temporary, vertices + 0xc, (minus_half, size, 0));
            set_point(e, temporary, vertices + 0x18, (half, size, 0));
        }
        let normals = new_array(e, 0x90, 0xc, 0xc, POINT_CONSTRUCT);
        e.call(MEMCPY, &args![normals, normals_table, 0x90u32]);
        let coordinates = new_array(e, 0x20, 8, 4, POINT_CONSTRUCT);
        let one = 1.0f32.to_bits();
        for (i, (u, v)) in [(one, one), (one, 0), (0, 0), (0, one)].iter().enumerate() {
            let at = e
                .call(TEXTURE_COORDINATE_SET, &args![temporary, *u, *v])
                .u32();
            let (first, second) = (e.mem.u32(at), e.mem.u32(at + 4));
            e.mem.set_u32(coordinates + 8 * i as u32, first);
            e.mem.set_u32(coordinates + 8 * i as u32 + 4, second);
        }
        let colors = new_array(e, 0x40, 0x10, 4, COLOR_CONSTRUCT);
        for i in 0..4u32 {
            let at = e
                .call(COLOR_SET, &args![temporary, one, one, one, 0u32])
                .u32();
            for offset in [0u32, 4, 8, 0xc] {
                let word = e.mem.u32(at + offset);
                e.mem.set_u32(colors + 0x10 * i + offset, word);
            }
        }
        let triangles = e.call(NI_ALLOC, &args![0xcu32]).u32();
        for (i, index) in [0u16, 2, 1, 0, 3, 2].iter().enumerate() {
            e.mem.set_u16(triangles + 2 * i as u32, *index);
        }
        let block = e.call(NI_NEW, &args![0x58u32]).u32();
        let data = if block != 0 {
            e.call(
                TRI_SHAPE_DATA_CONSTRUCT,
                &args![
                    block,
                    4u32,
                    vertices,
                    normals,
                    colors,
                    coordinates,
                    1u32,
                    0x1000u32,
                    2u32,
                    triangles
                ],
            )
            .u32()
        } else {
            0
        };
        e.call(POINTER_CONSTRUCT, &args![pointer, data]);
        e.call(POINTER_ASSIGN, &args![geometry, pointer]);
        e.call(POINTER_RELEASE, &args![pointer]);
        e.call(SCOPE_GUARD_DESTRUCT, &args![guard]);
    });
}

// Translated from 0051cba0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0051cba0 - the FS:[0] exception frame (it releases the pointers if a step throws) and the /GS cookie check (00ec408c) are not translated.
/// `TESObjectTREE::BuildDistant3D` (engine map; `RET 4`, a byte): 0 unless
/// the `BNAM` sizes pass (`fn_0051d250`). Inside a scope guard (tag 0x23,
/// line 0x1d6): the billboard geometry (`CreateBillboardGeomData` with
/// `upright` into a null `NiPointer`), an `NiTriShape` of it (0xc4 bytes,
/// `NiTriShape_ov2`), the property `fn_0051cee0` names attached when there
/// is one, a lighting property (0x104 bytes, `00ba8630`) attached, a
/// `BSShaderTextureSet` (0x38 bytes) whose texture 0 is the billboard
/// texture name (vtable slot `+0x94`) loaded into the property
/// (`LoadTextureSet` with `0102bf34`), the property's vtable slot `+0xe4`
/// with 0. When the texture set's texture 0 (vtable slot `+0x90` into a
/// null `NiPointer`) is still null (`00822510`), answers 0; otherwise an
/// `NiBillboardNode` (0xb4 bytes, `fn_0051cf00`) in mode 5 gets the shape
/// as a child (vtable slot `+0xdc` with 1) and is answered. Allocations that
/// fail give null objects, which are used without a check as the exe does.
/// The pointers, the guard and the 0x104-byte name the game keeps on its
/// stack are a block here.
pub fn tes_object_tree_build_distant_3d(e: &mut Engine, this: Ptr, upright: u8) -> u32 {
    if !fn_0051d250(e, this) {
        return 0;
    }
    e.with_stack(0x118, |e, locals| {
        let guard = locals.addr();
        let geometry = locals.addr() + 4;
        let texture = locals.addr() + 8;
        let name = locals.addr() + 0xc;
        e.call(
            SCOPE_GUARD_CONSTRUCT,
            &args![guard, SCOPE_GUARD_TAG, 1u32, TREE_SOURCE_FILE, 0x1d6u32],
        );
        e.call(POINTER_CONSTRUCT, &args![geometry, 0u32]);
        tes_object_tree_create_billboard_geom_data(e, this, upright, geometry);
        let block = e.call(NI_NEW, &args![0xc4u32]).u32();
        let shape = if block != 0 {
            let data = e.call(FIRST_WORD, &args![geometry]).u32();
            e.call(TRI_SHAPE_CONSTRUCT, &args![block, data]).u32()
        } else {
            0
        };
        let extra = fn_0051cee0(e);
        if extra != 0 {
            e.call(ATTACH_PROPERTY, &args![shape, extra]);
        }
        let block = e.call(NI_NEW, &args![0x104u32]).u32();
        let property = if block != 0 {
            e.call(LIGHTING_PROPERTY_CONSTRUCT, &args![block]).u32()
        } else {
            0
        };
        e.call(ATTACH_PROPERTY, &args![shape, property]);
        tes_object_tree_get_billboard_texture_filename(e, this, Ptr::new(name));
        let block = e.call(NI_NEW, &args![0x38u32]).u32();
        let set = if block != 0 {
            bs_shader_texture_set_bs_shader_texture_set(e, Ptr::new(block)).addr()
        } else {
            0
        };
        e.vcall(set, 0x94, &args![0u32, name]);
        e.call(
            LOAD_TEXTURE_SET,
            &args![property, LOAD_TEXTURE_SET_ARGUMENT, set],
        );
        e.vcall(property, 0xe4, &args![0u32]);
        e.call(POINTER_CONSTRUCT, &args![texture, 0u32]);
        e.vcall(set, 0x90, &args![0u32, texture]);
        let node = if e.call(POINTER_HOLDS, &args![texture, 0u32]).u32() as u8 != 0 {
            0
        } else {
            let block = e.call(NI_NEW, &args![0xb4u32]).u32();
            let node = if block != 0 {
                fn_0051cf00(e, Ptr::new(block)).addr()
            } else {
                0
            };
            fn_0051d000(e, Ptr::new(node), 5);
            e.vcall(node, 0xdc, &args![shape, 1u32]);
            node
        };
        e.call(POINTER_RELEASE, &args![texture]);
        e.call(POINTER_RELEASE, &args![geometry]);
        e.call(SCOPE_GUARD_DESTRUCT, &args![guard]);
        node
    })
}

// Translated from 0051cee0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The first word (`00559450`) of the string at `+0xc` of the object
/// `00664840(1)` answers (engine map: no name; cdecl, no arguments).
pub fn fn_0051cee0(e: &mut Engine) -> u32 {
    let object = e.call(OBJECT_00664840, &args![1u32]).u32();
    e.call(FIRST_WORD, &args![object.wrapping_add(0xc)]).u32()
}

// Translated from 0051d220 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSTextureSet` scalar deleting destructor (engine map; `RET 4`): the
/// destructor body `004a7090`, then, when bit 0 of `flags` is set, gives
/// the 8-byte object back (`00aa1460`). Returns `this`.
pub fn bs_texture_set_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(TEXTURE_SET_DESTRUCT, &args![this]);
    if flags & 1 != 0 {
        e.call(FREE_SIZED, &args![this, 8u32]);
    }
    this
}

// Translated from 0051d2c0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0051d2c0 - the FS:[0] exception frame (it frees the point array if its construction throws) is not translated.
/// The distant-tree record's constructor (engine map: no name; `RET 0x1c`):
/// stores `tree`, `first`, `second`, `third` and `count` at +0, +4, +8, +0xc,
/// +0x10; an array of `count` points (`count * 12` bytes, all ones on
/// overflow, constructed with `006815c0`; 0 when the allocation fails) at
/// +0x14 gets `count * 12` bytes of `points` (`memcpy`); a block of
/// `count` words (all ones on overflow) at +0x18 gets `count * 4` bytes of
/// `words`. Returns `this`.
#[allow(clippy::too_many_arguments)]
pub fn fn_0051d2c0(
    e: &mut Engine,
    this: Ptr,
    tree: u32,
    first: u32,
    second: u32,
    third: u32,
    count: u32,
    points: u32,
    words: u32,
) -> Ptr {
    let base = this.addr();
    e.mem.set_u32(base, tree);
    e.mem.set_u32(base + 4, first);
    e.mem.set_u32(base + 8, second);
    e.mem.set_u32(base + 0xc, third);
    e.mem.set_u32(base + 0x10, count);
    let count = e.mem.u32(base + 0x10);
    let (size, overflow) = count.overflowing_mul(0xc);
    let request = if overflow { 0xffff_ffff } else { size };
    let array = new_array(e, request, 0xc, count, POINT_CONSTRUCT);
    e.mem.set_u32(base + 0x14, array);
    let bytes = e.mem.u32(base + 0x10).wrapping_mul(0xc);
    let array = e.mem.u32(base + 0x14);
    e.call(MEMCPY, &args![array, points, bytes]);
    let count = e.mem.u32(base + 0x10);
    let (size, overflow) = count.overflowing_mul(4);
    let request = if overflow { 0xffff_ffff } else { size };
    let block = e.call(ALLOCATE, &args![request]).u32();
    e.mem.set_u32(base + 0x18, block);
    let bytes = e.mem.u32(base + 0x10) << 2;
    let block = e.mem.u32(base + 0x18);
    e.call(MEMCPY, &args![block, words, bytes]);
    this
}

// Translated from 0051d400 (decompiled, FalloutNV.exe 1.4.0.525)
/// The distant-tree record's destructor body (engine map: no name; no
/// stack words): frees the point array (+0x14) and the word block (+0x18).
pub fn fn_0051d400(e: &mut Engine, this: Ptr) {
    let points = e.mem.u32(this.addr() + 0x14);
    e.call(FREE, &args![points]);
    let words = e.mem.u32(this.addr() + 0x18);
    e.call(FREE, &args![words]);
}

// Translated from 0051d440 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0051d440 - the FS:[0] exception frame (it releases the texture pointer and ends the scope guard if a step throws) and the /GS cookie check (00ec408c) are not translated.
/// `TESObjectTREE::CreateDistant3D` (engine map; `RET 0x20`, eight words;
/// the second and the eighth are not read): inside a scope guard (tag 0x23,
/// line 0x228), nothing unless `seventh`, `points`, `words` and `count` are
/// all set, `0051d740` answers true and the `BNAM` sizes pass
/// (`fn_0051d250`). When `00b58720` answers false for the tree's form id:
/// the billboard texture's name (`GetBillboardTextureFilename`) becomes a
/// path (`00af4200`, 0x104 bytes) whose texture is looked up
/// (`BSTexturePalette::GetTexture_ov2` into a null `NiPointer`); without
/// one, a distant-tree record (0x1c bytes, `fn_0051d2c0` with the tree,
/// `fifth`, `sixth`, `seventh`, `count`, `points`, `words`; null when the
/// allocation fails) is queued on the model loader (`00447980` with the
/// path) and nothing more happens; with one, the node `BuildDistant3D(1)`
/// builds is needed (nothing more without it). Then a record (`00b57e70`)
/// of 0, the node and the form id is passed to `00b59ae0` with `fifth`,
/// `sixth`, `seventh`, its address, `points`, `words` and the low 16 bits of
/// `count`; the node it leaves there, when set, is deleted (vtable slot 0
/// with 1). The guard, pointer, record and names the game keeps on its
/// stack are a block here.
#[allow(clippy::too_many_arguments)]
pub fn tes_object_tree_create_distant_3d(
    e: &mut Engine,
    this: Ptr,
    points: u32,
    _unused_2: u32,
    words: u32,
    count: u32,
    fifth: u32,
    sixth: u32,
    seventh: u32,
    _unused_8: u32,
) {
    e.with_stack(0x220, |e, locals| {
        let guard = locals.addr();
        let texture = locals.addr() + 4;
        let record = locals.addr() + 8;
        let name = locals.addr() + 0x14;
        let path = locals.addr() + 0x118;
        e.call(
            SCOPE_GUARD_CONSTRUCT,
            &args![guard, SCOPE_GUARD_TAG, 1u32, TREE_SOURCE_FILE, 0x228u32],
        );
        if seventh == 0 || points == 0 || words == 0 || count == 0 {
            e.call(SCOPE_GUARD_DESTRUCT, &args![guard]);
            return;
        }
        if e.call(DISTANT_CHECK_0051D740, &[]).u32() as u8 == 0 {
            e.call(SCOPE_GUARD_DESTRUCT, &args![guard]);
            return;
        }
        if !fn_0051d250(e, this) {
            e.call(SCOPE_GUARD_DESTRUCT, &args![guard]);
            return;
        }
        let mut node = 0u32;
        let id = e.call(FORM_ID, &args![this]).u32();
        if e.call(DISTANT_FORM_CHECK, &args![id]).u32() as u8 == 0 {
            tes_object_tree_get_billboard_texture_filename(e, this, Ptr::new(name));
            e.call(TEXTURE_PATH, &args![name, path, 0x104u32]);
            e.call(POINTER_CONSTRUCT, &args![texture, 0u32]);
            e.call(GET_TEXTURE, &args![path, texture]);
            if e.call(FIRST_WORD, &args![texture]).u32() == 0 {
                let block = e.call(ALLOCATE, &args![0x1cu32]).u32();
                let queued = if block != 0 {
                    fn_0051d2c0(
                        e,
                        Ptr::new(block),
                        this.addr(),
                        fifth,
                        sixth,
                        seventh,
                        count,
                        points,
                        words,
                    )
                    .addr()
                } else {
                    0
                };
                let loader = e.global::<u32>(MODEL_LOADER);
                e.call(MODEL_LOADER_QUEUE, &args![loader, path, queued]);
                e.call(POINTER_RELEASE, &args![texture]);
                e.call(SCOPE_GUARD_DESTRUCT, &args![guard]);
                return;
            }
            node = tes_object_tree_build_distant_3d(e, this, 1);
            if node == 0 {
                e.call(POINTER_RELEASE, &args![texture]);
                e.call(SCOPE_GUARD_DESTRUCT, &args![guard]);
                return;
            }
            e.call(POINTER_RELEASE, &args![texture]);
        }
        e.call(DISTANT_RECORD_CONSTRUCT, &args![record]);
        let id = e.call(FORM_ID, &args![this]).u32();
        e.mem.set_u32(record + 8, id);
        e.mem.set_u32(record + 4, node);
        e.mem.set_u32(record, 0);
        e.call(
            DISTANT_ADD,
            &args![fifth, sixth, seventh, record, points, words, count & 0xffff],
        );
        let left = e.mem.u32(record + 4);
        if left != 0 {
            e.vcall(left, 0, &args![1u32]);
        }
        e.call(SCOPE_GUARD_DESTRUCT, &args![guard]);
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0051_d150, fn_0051d150(Ptr) -> u32),
        entry!(0x0051_d1b0, fn_0051d1b0(Ptr) -> u32),
        entry!(0x0051_cfe0, fn_0051cfe0(Ptr)),
        entry!(0x0051_bbc0, tes_object_tree_load(Ptr, Ptr) -> bool),
        entry!(0x0051_cf00, fn_0051cf00(Ptr) -> Ptr),
        entry!(0x0051_d000, fn_0051d000(Ptr, u16)),
        entry!(0x0051_d020, fn_0051d020(Ptr, u16, u16, u16)),
        entry!(0x0051_d060, fn_0051d060(Ptr, u8)),
        entry!(0x0051_d080, fn_0051d080(Ptr, u8, u16)),
        entry!(0x0051_d190, fn_0051d190(Ptr) -> Ptr),
        entry!(0x0051_d0d0, bs_shader_texture_set_bs_shader_texture_set(Ptr) -> Ptr),
        entry!(0x0051_cf90, ni_billboard_node_get_rtti(Ptr) -> u32),
        entry!(
            0x0051_d160,
            bs_shader_texture_set_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(0x0051_d1c0, fn_0051d1c0(Ptr)),
        entry!(
            0x0051_c3b0,
            tes_object_tree_get_billboard_texture_filename(Ptr, Ptr)
        ),
        entry!(0x0051_d250, fn_0051d250(Ptr) -> bool),
        entry!(
            0x0051_cfa0,
            ni_billboard_node_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(0x0051_bfb0, tes_object_tree_save(Ptr)),
        entry!(0x0051_c140, fn_0051c140(Ptr, u32)),
        entry!(0x0051_c220, fn_0051c220(Ptr, u32) -> bool),
        entry!(0x0051_c330, fn_0051c330(Ptr, u32) -> bool),
        entry!(0x0051_c360, fn_0051c360(Ptr) -> u32),
        entry!(
            0x0051_c490,
            tes_object_tree_create_billboard_geom_data(Ptr, u8, u32)
        ),
        entry!(0x0051_cba0, tes_object_tree_build_distant_3d(Ptr, u8) -> u32),
        entry!(0x0051_cee0, fn_0051cee0() -> u32),
        entry!(
            0x0051_d220,
            bs_texture_set_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(
            0x0051_d2c0,
            fn_0051d2c0(Ptr, u32, u32, u32, u32, u32, u32, u32) -> Ptr
        ),
        entry!(0x0051_d400, fn_0051d400(Ptr)),
        entry!(
            0x0051_d440,
            tes_object_tree_create_distant_3d(Ptr, u32, u32, u32, u32, u32, u32, u32, u32)
        ),
    ]
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
    fn fwd_a03_0051cfe0_passes_this_to_the_callee() {
        let mut e = Engine::new();
        e.register(0x00a5_e930, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0051_cfe0, &args![0x0000_5000u32]);
        assert_eq!(calls_to(&e, 0x00a5_e930), vec![vec![0x5000]]);
    }
}

#[cfg(test)]
mod load_tests {
    use super::*;
    use std::cell::Cell;
    use std::rc::Rc;

    const FILE: u32 = 0x6000;
    const OBJECT_TABLE: u32 = 0x0700_0000;
    const TARGET_LOAD_BOUNDS: u32 = 0x0050_0001;
    const TARGET_SET_EDITOR_ID: u32 = 0x0050_0002;

    type CallLog = Vec<(u32, Vec<u32>)>;

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    fn calls_to(log: &CallLog, addr: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    struct Record {
        chunks: Vec<u32>,
        form_type: u32,
        null_this: bool,
        chunk_size: u32,
        swap: bool,
        /// Words the chunk data call writes at its buffer.
        data: Vec<u32>,
    }

    impl Record {
        fn of(chunks: Vec<u32>) -> Record {
            Record {
                chunks,
                form_type: 0x25,
                null_this: false,
                chunk_size: 0,
                swap: false,
                data: vec![],
            }
        }
    }

    /// Runs the loader; returns the engine, the call log (without the top-level
    /// call), the address of `this` and the result.
    fn run(record: Record) -> (Engine, CallLog, u32, bool) {
        let mut e = Engine::new();
        let position = Rc::new(Cell::new(0usize));
        let (list, at) = (record.chunks.clone(), position.clone());
        e.register_double(GET_TES_CHUNK, move |_, _| {
            ret(list.get(at.get()).copied().unwrap_or(0))
        });
        let total = record.chunks.len();
        e.register_double(NEXT_CHUNK, move |_, _| {
            position.set(position.get() + 1);
            ret(u32::from(position.get() < total))
        });
        let form_type = record.form_type;
        e.register_double(GET_TES_FORM, move |_, _| ret(0x7700 | form_type));
        for addr in [
            TES_FORM_LOAD_FORM,
            LOAD_MODEL_CHUNK,
            LOAD_TEXTURE_CHUNK,
            LOAD_DESTRUCTIBLE_CHUNK,
            SWAP_CNAM,
            SWAP_4_BYTES,
            FREE,
            LIST_CLEAR,
            LIST_ADD,
            TARGET_LOAD_BOUNDS,
            TARGET_SET_EDITOR_ID,
        ] {
            e.register(addr, |_, _| Ret::default());
        }
        e.register(ALLOCATE, |e, a| ret(e.mem.alloc(a[0])));
        e.register(MEMSET, |_, _| Ret::default());
        let data = record.data.clone();
        e.register_double(GET_CHUNK_DATA, move |e, a| {
            for (i, word) in data.iter().enumerate() {
                e.mem.set_u32(a[1] + 4 * i as u32, *word);
            }
            Ret::default()
        });
        let chunk_size = record.chunk_size;
        e.register_double(CHUNK_SIZE, move |_, _| ret(chunk_size));
        let swap = record.swap;
        e.register_double(SWAP_FLAG, move |_, _| ret(0x5500 | u32::from(swap)));
        let mut table = vec![0u32; 0x4e];
        table[0xe0 / 4] = TARGET_LOAD_BOUNDS;
        table[0x134 / 4] = TARGET_SET_EDITOR_ID;
        e.put_vtable(OBJECT_TABLE, &table);
        let this = if record.null_this {
            0
        } else {
            let object = e.mem.alloc(0xa0);
            e.mem.set_u32(object, OBJECT_TABLE);
            object
        };
        e.call_log = Some(vec![]);
        let result = e.call(0x0051_bbc0, &args![this, FILE]).bool();
        let log = e.call_log.take().unwrap();
        (e, log.into_iter().skip(1).collect(), this, result)
    }

    #[test]
    fn a_record_of_another_form_type_is_refused() {
        let mut record = Record::of(vec![CHUNK_ICON]);
        record.form_type = 0x26;
        let (_, log, _, result) = run(record);
        assert!(!result);
        assert_eq!(log, vec![(GET_TES_FORM, vec![FILE])]);
    }

    #[test]
    fn the_header_is_read_before_the_chunks() {
        let (_, log, this, result) = run(Record::of(vec![]));
        assert!(result);
        assert_eq!(log[0], (GET_TES_FORM, vec![FILE]));
        assert_eq!(log[1], (TES_FORM_LOAD_FORM, vec![this, FILE]));
        assert_eq!(log[2], (GET_TES_CHUNK, vec![FILE]));
        assert_eq!(log.len(), 3);
    }

    #[test]
    fn the_virtual_chunks_go_to_the_objects_slots() {
        let mut record = Record::of(vec![CHUNK_OBND, CHUNK_EDID, 0x5555_5555]);
        record.chunk_size = 5;
        let (_, log, this, _) = run(record);
        assert_eq!(calls_to(&log, TARGET_LOAD_BOUNDS), vec![vec![this, FILE]]);
        let data = calls_to(&log, GET_CHUNK_DATA);
        assert_eq!(data.len(), 1);
        assert_eq!((data[0][0], data[0][2]), (FILE, 0x200));
        assert_eq!(
            calls_to(&log, TARGET_SET_EDITOR_ID),
            vec![vec![this, data[0][1]]]
        );
        assert_eq!(calls_to(&log, NEXT_CHUNK).len(), 3);
    }

    #[test]
    fn the_components_get_their_chunks() {
        let chunks = vec![CHUNK_MODL, CHUNK_MODT, CHUNK_ICON, CHUNK_DSTD, CHUNK_DEST];
        let (_, log, this, _) = run(Record::of(chunks));
        assert_eq!(
            calls_to(&log, LOAD_MODEL_CHUNK),
            vec![vec![this + 0x30, FILE]; 2]
        );
        assert_eq!(
            calls_to(&log, LOAD_TEXTURE_CHUNK),
            vec![vec![this + 0x48, FILE]]
        );
        assert_eq!(
            calls_to(&log, LOAD_DESTRUCTIBLE_CHUNK),
            vec![vec![this + 0x54, FILE]; 2]
        );
    }

    #[test]
    fn a_null_object_has_null_components() {
        let mut record = Record::of(vec![CHUNK_MODL, CHUNK_ICON, CHUNK_DEST]);
        record.null_this = true;
        let (_, log, _, _) = run(record);
        assert_eq!(calls_to(&log, LOAD_MODEL_CHUNK), vec![vec![0, FILE]]);
        assert_eq!(calls_to(&log, LOAD_TEXTURE_CHUNK), vec![vec![0, FILE]]);
        assert_eq!(calls_to(&log, LOAD_DESTRUCTIBLE_CHUNK), vec![vec![0, FILE]]);
    }

    #[test]
    fn cnam_needs_exactly_0x20_bytes_and_raises_a_negative_limit() {
        let mut record = Record::of(vec![CHUNK_CNAM]);
        record.chunk_size = 0x1f;
        let (_, log, _, _) = run(record);
        assert!(calls_to(&log, GET_CHUNK_DATA).is_empty());

        let mut record = Record::of(vec![CHUNK_CNAM]);
        record.chunk_size = 0x20;
        record.data = vec![0; 8];
        let (e, log, this, _) = run(record);
        assert_eq!(
            calls_to(&log, GET_CHUNK_DATA),
            vec![vec![FILE, this + 0x6c, 0x20]]
        );
        assert!(calls_to(&log, SWAP_CNAM).is_empty());
        // The block is 0x20 bytes at +0x6c, so +0x80 is its sixth word.
        assert_eq!(e.mem.u32(this + 0x80), 0);

        let mut record = Record::of(vec![CHUNK_CNAM]);
        record.chunk_size = 0x20;
        record.swap = true;
        record.data = vec![1, 2, 3, 4, 5, 0xffff_fff0];
        let (e, log, this, _) = run(record);
        assert_eq!(calls_to(&log, SWAP_CNAM), vec![vec![this + 0x6c]]);
        assert_eq!(e.mem.u32(this + 0x80), 0x80);

        let mut record = Record::of(vec![CHUNK_CNAM]);
        record.chunk_size = 0x20;
        record.data = vec![1, 2, 3, 4, 5, 0x7fff_ffff];
        let (e, _, this, _) = run(record);
        assert_eq!(e.mem.u32(this + 0x80), 0x7fff_ffff);
    }

    #[test]
    fn bnam_needs_exactly_8_bytes_and_swaps_both_words_on_request() {
        let mut record = Record::of(vec![CHUNK_BNAM]);
        record.chunk_size = 4;
        let (_, log, _, _) = run(record);
        assert!(calls_to(&log, GET_CHUNK_DATA).is_empty());

        let mut record = Record::of(vec![CHUNK_BNAM]);
        record.chunk_size = 8;
        let (_, log, this, _) = run(record);
        assert_eq!(
            calls_to(&log, GET_CHUNK_DATA),
            vec![vec![FILE, this + 0x8c, 8]]
        );
        assert!(calls_to(&log, SWAP_4_BYTES).is_empty());

        let mut record = Record::of(vec![CHUNK_BNAM]);
        record.chunk_size = 8;
        record.swap = true;
        let (_, log, this, _) = run(record);
        assert_eq!(
            calls_to(&log, SWAP_4_BYTES),
            vec![vec![this + 0x8c, 0], vec![this + 0x90, 0]]
        );
    }

    #[test]
    fn snam_adds_the_non_zero_words_and_frees_the_block() {
        for size in [0, 6] {
            let mut record = Record::of(vec![CHUNK_SNAM]);
            record.chunk_size = size;
            let (_, log, _, _) = run(record);
            assert!(calls_to(&log, ALLOCATE).is_empty(), "size {size}");
        }

        let mut record = Record::of(vec![CHUNK_SNAM]);
        record.chunk_size = 12;
        record.data = vec![0x11, 0, 0x33];
        let (_, log, this, _) = run(record);
        assert_eq!(calls_to(&log, ALLOCATE), vec![vec![12]]);
        let block = calls_to(&log, GET_CHUNK_DATA)[0][1];
        assert_eq!(calls_to(&log, MEMSET), vec![vec![block, 0, 12]]);
        assert_eq!(calls_to(&log, LIST_CLEAR), vec![vec![this + 0x5c]]);
        assert_eq!(
            calls_to(&log, LIST_ADD),
            vec![vec![this + 0x5c, block], vec![this + 0x5c, block + 8]]
        );
        assert!(calls_to(&log, SWAP_4_BYTES).is_empty());
        assert_eq!(calls_to(&log, FREE), vec![vec![block]]);

        let mut record = Record::of(vec![CHUNK_SNAM]);
        record.chunk_size = 8;
        record.swap = true;
        record.data = vec![1, 2];
        let (_, log, _, _) = run(record);
        let block = calls_to(&log, GET_CHUNK_DATA)[0][1];
        assert_eq!(
            calls_to(&log, SWAP_4_BYTES),
            vec![vec![block, 0], vec![block + 4, 0]]
        );
    }
}

#[cfg(test)]
mod constant_getter_tests {
    use super::*;

    #[test]
    fn fn_0051d150_answers_its_constant_address() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x0051_d150, &args![0x1000u32]).u32(), 0x0120_044c);
        assert_eq!(e.call(0x0051_d150, &args![0u32]).u32(), 0x0120_044c);
    }

    #[test]
    fn fn_0051d1b0_answers_its_constant_address() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x0051_d1b0, &args![0x1000u32]).u32(), 0x0120_0444);
        assert_eq!(e.call(0x0051_d1b0, &args![0u32]).u32(), 0x0120_0444);
    }
}

#[cfg(test)]
mod b015_tests {
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

    fn noop(e: &mut Engine, addrs: &[u32]) {
        for a in addrs {
            e.register(*a, |_, _| Ret::default());
        }
    }

    #[test]
    fn billboard_constructor_sets_vtable_mode_one_and_flag_bit_3() {
        let mut e = Engine::new();
        noop(&mut e, &[NI_NODE_CONSTRUCT]);
        let node: Ptr = Ptr::new(e.mem.alloc(0xb4));
        e.mem.set_u16(node.addr() + 0xac, 0xfff0);
        e.mem.set_u32(node.addr() + 0xb0, 0x7fc0_0000);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0051_cf00, &args![node]).u32(), node.addr());
        assert_eq!(calls_to(&e, NI_NODE_CONSTRUCT), vec![vec![node.addr(), 0]]);
        assert_eq!(e.mem.u32(node.addr()), BILLBOARD_NODE_VTABLE);
        assert_eq!(e.mem.u16(node.addr() + 0xac), 0x9);
        assert_eq!(e.mem.u32(node.addr() + 0xb0), 0);
    }

    #[test]
    fn mode_setter_replaces_the_low_three_bits() {
        let mut e = Engine::new();
        let node: Ptr = Ptr::new(e.mem.alloc(0xb4));
        e.mem.set_u16(node.addr() + 0xac, 0xff0f);
        e.call(0x0051_d000, &args![node, 0x0002u32]);
        assert_eq!(e.mem.u16(node.addr() + 0xac), 0xff0a);
    }

    #[test]
    fn masked_setter_shifts_modulo_32_and_keeps_16_bits() {
        let mut e = Engine::new();
        let node: Ptr = Ptr::new(e.mem.alloc(0xb4));
        e.mem.set_u16(node.addr() + 0xac, 0xffff);
        e.call(0x0051_d020, &args![node, 0x3u32, 0x00f0u32, 4u32]);
        assert_eq!(e.mem.u16(node.addr() + 0xac), 0xff3f);
        // A shift of 36 is 4 for SHL; bits shifted past 16 are lost.
        e.mem.set_u16(node.addr() + 0xac, 0);
        e.call(0x0051_d020, &args![node, 0xf001u32, 0u32, 36u32]);
        assert_eq!(e.mem.u16(node.addr() + 0xac), 0x0010);
    }

    #[test]
    fn flag_bit_3_setter_and_bit_helper_set_and_clear() {
        let mut e = Engine::new();
        let node: Ptr = Ptr::new(e.mem.alloc(0xb4));
        e.mem.set_u16(node.addr() + 0xac, 0x0101);
        e.call(0x0051_d060, &args![node, 0x100u32]);
        // Only the low byte of the flag counts: 0x100 is "off".
        assert_eq!(e.mem.u16(node.addr() + 0xac), 0x0101);
        e.call(0x0051_d060, &args![node, 1u32]);
        assert_eq!(e.mem.u16(node.addr() + 0xac), 0x0109);
        e.call(0x0051_d080, &args![node, 0u32, 0x0101u32]);
        assert_eq!(e.mem.u16(node.addr() + 0xac), 0x0008);
        e.call(0x0051_d080, &args![node, 7u32, 0x8000u32]);
        assert_eq!(e.mem.u16(node.addr() + 0xac), 0x8008);
    }

    #[test]
    fn texture_set_constructor_runs_the_base_and_sets_its_vtable() {
        let mut e = Engine::new();
        noop(&mut e, &[NI_OBJECT_CONSTRUCT]);
        let set: Ptr = Ptr::new(e.mem.alloc(0x38));
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0051_d190, &args![set]).u32(), set.addr());
        assert_eq!(calls_to(&e, NI_OBJECT_CONSTRUCT), vec![vec![set.addr()]]);
        assert_eq!(e.mem.u32(set.addr()), TEXTURE_SET_VTABLE);
    }

    #[test]
    fn shader_texture_set_constructor_builds_six_strings() {
        let mut e = Engine::new();
        noop(&mut e, &[NI_OBJECT_CONSTRUCT, ARRAY_CONSTRUCT]);
        let set: Ptr = Ptr::new(e.mem.alloc(0x38));
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0051_d0d0, &args![set]).u32(), set.addr());
        assert_eq!(e.mem.u32(set.addr()), SHADER_TEXTURE_SET_VTABLE);
        assert_eq!(calls_to(&e, NI_OBJECT_CONSTRUCT).len(), 1);
        assert_eq!(
            calls_to(&e, ARRAY_CONSTRUCT),
            vec![vec![set.addr() + 8, 8, 6, 0x0040_37b0, 0x0040_37d0]]
        );
    }

    #[test]
    fn billboard_rtti_is_a_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x0051_cf90, &args![0u32]).u32(), 0x011f_4aa0);
    }

    #[test]
    fn shader_texture_set_destructors_free_on_flag_bit_0() {
        let mut e = Engine::new();
        noop(
            &mut e,
            &[ARRAY_DESTRUCT, TEXTURE_SET_BASE_DESTRUCT, FREE_SIZED],
        );
        let set: Ptr = Ptr::new(e.mem.alloc(0x38));
        for (flags, frees) in [(0u32, false), (2, false), (3, true)] {
            e.call_log = Some(vec![]);
            assert_eq!(e.call(0x0051_d160, &args![set, flags]).u32(), set.addr());
            assert_eq!(
                calls_to(&e, ARRAY_DESTRUCT),
                vec![vec![set.addr() + 8, 8, 6, 0x0040_37d0]]
            );
            assert_eq!(
                calls_to(&e, TEXTURE_SET_BASE_DESTRUCT),
                vec![vec![set.addr()]]
            );
            let freed = calls_to(&e, FREE_SIZED);
            if frees {
                assert_eq!(freed, vec![vec![set.addr(), 0x38]]);
            } else {
                assert!(freed.is_empty());
            }
        }
    }

    #[test]
    fn billboard_deleting_destructor_frees_0xb4_bytes_on_flag_bit_0() {
        let mut e = Engine::new();
        noop(&mut e, &[0x00a5_e930, FREE_SIZED]);
        for (flags, frees) in [(0u32, false), (1, true)] {
            e.call_log = Some(vec![]);
            assert_eq!(e.call(0x0051_cfa0, &args![0x5000u32, flags]).u32(), 0x5000);
            assert_eq!(calls_to(&e, 0x00a5_e930), vec![vec![0x5000]]);
            assert_eq!(calls_to(&e, FREE_SIZED).len(), usize::from(frees));
        }
    }

    fn texture_name(model_name: &[u8]) -> Vec<u8> {
        let mut e = Engine::new();
        e.map(BILLBOARD_TEXTURE_PREFIX, 0x10);
        e.mem.set_cstr(BILLBOARD_TEXTURE_PREFIX, b"Trees/");
        let tree: Ptr = Ptr::new(e.mem.alloc(0x100));
        e.put_vtable(0x0090_0000, &[0, 0, 0, 0, 0, 0x0090_1000]);
        e.mem.set_u32(tree.addr() + 0x30, 0x0090_0000);
        let name = e.mem.alloc(0x40);
        e.mem.set_cstr(name, model_name);
        e.register_double(0x0090_1000, move |_, _| name.into_ret());
        let buffer = e.mem.alloc(0x80);
        e.mem.write(buffer, &[0xaa; 0x80]);
        e.call(0x0051_c3b0, &args![tree, buffer]);
        e.mem.cstr(buffer)
    }

    #[test]
    fn billboard_texture_name_swaps_the_extension_for_dds() {
        assert_eq!(
            texture_name(b"Joshua01.nif"),
            b"Trees/Joshua01.dds".to_vec()
        );
        assert_eq!(texture_name(b"a.b.nif"), b"Trees/a.dds".to_vec());
        // No dot: the name is copied whole, without an extension.
        assert_eq!(texture_name(b"Plain"), b"Trees/Plain".to_vec());
    }

    #[test]
    fn bnam_test_follows_the_flag_tests() {
        let mut e = Engine::new();
        for (addr, value) in [
            (BNAM_LOWER, 0.0f64),
            (BNAM_FIRST_UPPER, 10.0),
            (BNAM_SECOND_UPPER, 20.0),
        ] {
            e.map(addr, 8);
            e.set_global(addr, value);
        }
        let tree: Ptr = Ptr::new(e.mem.alloc(0x100));
        let mut check = |first: f32, second: f32| {
            e.mem.set_f32(tree.addr() + 0x8c, first);
            e.mem.set_f32(tree.addr() + 0x90, second);
            e.call(0x0051_d250, &args![tree]).bool()
        };
        // First in (0, 10]: false whatever the second is.
        assert!(!check(5.0, 100.0));
        assert!(!check(10.0, 100.0));
        // First outside: true unless the second is in (0, 20].
        assert!(!check(11.0, 20.0));
        assert!(check(11.0, 0.0));
        assert!(check(0.0, 21.0));
        assert!(check(f32::NAN, f32::NAN));
        assert!(!check(f32::NAN, 1.0));
    }
}

#[cfg(test)]
mod save_distant_tests {
    use super::*;

    type Log = Vec<(u32, Vec<u32>)>;

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

    fn run(e: &mut Engine, addr: u32, words: &[u32]) -> (Ret, Log) {
        e.call_log = Some(vec![]);
        let r = e.call(addr, words);
        (r, e.call_log.take().unwrap())
    }

    fn quiet(e: &mut Engine, addrs: &[u32]) {
        for a in addrs {
            e.register(*a, |_, _| Ret::default());
        }
    }

    /// The word list doubles: a list at `+0x5c` is `[count, words...]` held
    /// at the address its first word gives.
    fn list_engine() -> Engine {
        let mut e = Engine::new();
        e.register(LIST_COUNT, |e, a| {
            let words = e.mem.u32(a[0]);
            ret(if words == 0 { 0 } else { e.mem.u32(words) })
        });
        e.register(LIST_ELEMENT, |e, a| ret(e.mem.u32(a[0]) + 4 + 4 * a[1]));
        e.register(ALLOCATE, |e, a| ret(e.mem.alloc(a[0])));
        e.register(MEMSET, |e, a| {
            for i in 0..a[2] {
                e.mem.set_u8(a[0] + i, a[1] as u8);
            }
            Ret::default()
        });
        e.register(MEMCPY, |e, a| {
            for i in 0..a[2] {
                let b = e.mem.u8(a[1] + i);
                e.mem.set_u8(a[0] + i, b);
            }
            Ret::default()
        });
        e.register(MEMCMP, |e, a| {
            let differ = (0..a[2]).any(|i| e.mem.u8(a[0] + i) != e.mem.u8(a[1] + i));
            ret(u32::from(differ))
        });
        e.register(RT_DYNAMIC_CAST, |_, a| ret(a[0]));
        e
    }

    /// A tree (0x98 bytes) whose list holds `words` and whose vtable slot
    /// `+0x18c` answers its `+0x6c` data.
    fn tree(e: &mut Engine, words: &[u32]) -> u32 {
        let this = e.mem.alloc(0x98);
        let vtable = e.mem.alloc(0x190);
        e.mem.set_u32(this, vtable);
        e.mem.set_u32(vtable + SLOT_CNAM_DATA, 0x0f49_018c);
        e.register(0x0f49_018c, |_, a| ret(a[0] + CNAM_OFFSET));
        let list = e.mem.alloc(4 + 4 * words.len() as u32);
        e.mem.set_u32(list, words.len() as u32);
        for (i, w) in words.iter().enumerate() {
            e.mem.set_u32(list + 4 + 4 * i as u32, *w);
        }
        e.mem.set_u32(this + LIST_OFFSET, list);
        this
    }

    #[test]
    fn save_writes_components_words_and_data() {
        let mut e = list_engine();
        quiet(
            &mut e,
            &[
                START_FORM,
                CLOSE_FORM,
                MODEL_SAVE,
                TEXTURE_SAVE,
                DESTRUCTIBLE_SAVE,
                ADD_CHUNK_ARRAY_OV2,
                ADD_CHUNK_DATA,
                SWAP_CNAM,
            ],
        );
        e.register(SAVE_SWAPS_BYTES, |_, _| ret(1));
        let this = tree(&mut e, &[7, 0, 9]);
        e.mem.set_u32(this + CNAM_LIMIT_OFFSET, 0xffff_fff0);
        let (_, log) = run(&mut e, 0x0051_bfb0, &args![this]);
        assert_eq!(
            only(&log, MODEL_SAVE),
            vec![vec![this + 0x30, CHUNK_MODL, CHUNK_MODT]]
        );
        assert_eq!(
            only(&log, TEXTURE_SAVE),
            vec![vec![this + 0x48, CHUNK_ICON]]
        );
        assert_eq!(only(&log, DESTRUCTIBLE_SAVE), vec![vec![this + 0x54]]);
        let arrays = only(&log, ADD_CHUNK_ARRAY_OV2);
        assert_eq!((arrays[0][0], arrays[0][2]), (CHUNK_SNAM, 3));
        let block = arrays[0][1];
        assert_eq!([0, 4, 8].map(|o| e.mem.u32(block + o)), [7, 0, 9]);
        assert_eq!(arrays[1], vec![CHUNK_BNAM, this + 0x8c, 2]);
        assert_eq!(e.mem.u32(this + CNAM_LIMIT_OFFSET), 0x80);
        assert_eq!(
            only(&log, ADD_CHUNK_DATA),
            vec![vec![CHUNK_CNAM, this + 0x6c, 0x20]]
        );
        assert_eq!(only(&log, SWAP_CNAM).len(), 2);
        let empty = tree(&mut e, &[]);
        e.register(SAVE_SWAPS_BYTES, |_, _| ret(0));
        let (_, log) = run(&mut e, 0x0051_bfb0, &args![empty]);
        assert_eq!(only(&log, ADD_CHUNK_ARRAY_OV2).len(), 1);
        assert!(only(&log, SWAP_CNAM).is_empty());
        assert_eq!(e.mem.u32(empty + CNAM_LIMIT_OFFSET), 0);
    }

    #[test]
    fn copy_compare_and_random_pick() {
        let mut e = list_engine();
        quiet(&mut e, &[COPY_ALL_COMPONENTS]);
        e.register(COMPARE_ALL_COMPONENTS, |_, _| ret(0));
        e.register(LIST_CLEAR, |e, a| {
            let list = e.mem.alloc(0x40);
            e.mem.set_u32(a[0], list);
            Ret::default()
        });
        e.register(LIST_APPEND, |e, a| {
            let list = e.mem.u32(a[0]);
            let n = e.mem.u32(list);
            let value = e.mem.u32(a[1]);
            e.mem.set_u32(list + 4 + 4 * n, value);
            e.mem.set_u32(list, n + 1);
            Ret::default()
        });
        e.register(PAIR_EQUALS, |e, a| {
            ret(u32::from(
                e.mem.u32(a[0]) == e.mem.u32(a[1]) && e.mem.u32(a[0] + 4) == e.mem.u32(a[1] + 4),
            ))
        });
        let other = tree(&mut e, &[0x21, 0x22]);
        for i in 0..8 {
            e.mem.set_u32(other + 0x6c + 4 * i, 0x100 + i);
        }
        e.mem.set_u32(other + 0x8c, 3);
        e.mem.set_u32(other + 0x90, 4);
        let this = tree(&mut e, &[0x99]);
        assert!(e.call(0x0051_c220, &args![this, other]).bool());
        let (_, log) = run(&mut e, 0x0051_c140, &args![this, other]);
        assert_eq!(only(&log, COPY_ALL_COMPONENTS), vec![vec![this, other]]);
        assert_eq!(e.mem.u32(this + 0x6c + 0x1c), 0x107);
        assert_eq!((e.mem.u32(this + 0x8c), e.mem.u32(this + 0x90)), (3, 4));
        assert!(!e.call(0x0051_c220, &args![this, other]).bool());
        e.mem.set_u32(this + 0x90, 5);
        assert!(e.call(0x0051_c220, &args![this, other]).bool());
        e.mem.set_u32(this + 0x90, 4);
        let list = e.mem.u32(this + LIST_OFFSET);
        e.mem.set_u32(list + 8, 0x23);
        assert!(e.call(0x0051_c220, &args![this, other]).bool());
        e.mem.set_u32(list + 8, 0x22);
        e.register(COMPARE_ALL_COMPONENTS, |_, _| ret(1));
        assert!(e.call(0x0051_c220, &args![this, other]).bool());
        e.register(RT_DYNAMIC_CAST, |_, _| ret(0));
        assert!(e.call(0x0051_c220, &args![this, other]).bool());

        e.register(RANDOM, |_, _| ret(7));
        assert_eq!(e.call(0x0051_c360, &args![other]).u32(), 0x22);
        let empty = tree(&mut e, &[]);
        let (r, log) = run(&mut e, 0x0051_c360, &args![empty]);
        assert_eq!(r.eax, 0);
        assert!(only(&log, RANDOM).is_empty());
    }

    #[test]
    fn small_helpers() {
        let mut e = Engine::new();
        e.register(PAIR_EQUALS, |_, a| ret(u32::from(a[1] == 1) * 0x101));
        assert!(e.call(0x0051_c330, &args![0x10u32, 2u32]).bool());
        assert!(!e.call(0x0051_c330, &args![0x10u32, 1u32]).bool());
        e.register(OBJECT_00664840, |_, a| ret(0x3000 + a[0]));
        e.register(FIRST_WORD, |_, a| ret(a[0] * 2));
        assert_eq!(e.call(0x0051_cee0, &[]).u32(), (0x3001 + 0xc) * 2);
        quiet(&mut e, &[TEXTURE_SET_DESTRUCT, FREE_SIZED]);
        let (r, log) = run(&mut e, 0x0051_d220, &args![0x5000u32, 1u32]);
        assert_eq!(r.eax, 0x5000);
        assert_eq!(only(&log, FREE_SIZED), vec![vec![0x5000, 8]]);
        let (_, log) = run(&mut e, 0x0051_d220, &args![0x5000u32, 2u32]);
        assert!(only(&log, FREE_SIZED).is_empty());
        assert_eq!(only(&log, TEXTURE_SET_DESTRUCT), vec![vec![0x5000]]);
    }

    #[test]
    fn distant_record_copies_points_and_words() {
        let mut e = list_engine();
        quiet(&mut e, &[ARRAY_CONSTRUCT_STDCALL, FREE]);
        let points = e.mem.alloc(0x18);
        for i in 0..6 {
            e.mem.set_u32(points + 4 * i, 0x10 + i);
        }
        let words = e.mem.alloc(8);
        e.mem.set_u32(words, 0xaa);
        e.mem.set_u32(words + 4, 0xbb);
        let this = e.mem.alloc(0x1c);
        let (r, log) = run(
            &mut e,
            0x0051_d2c0,
            &args![this, 0x700u32, 1u32, 2u32, 3u32, 2u32, points, words],
        );
        assert_eq!(r.eax, this);
        assert_eq!(
            [0, 4, 8, 0xc, 0x10].map(|o| e.mem.u32(this + o)),
            [0x700, 1, 2, 3, 2]
        );
        let array = e.mem.u32(this + 0x14);
        assert_eq!(
            only(&log, ARRAY_CONSTRUCT_STDCALL),
            vec![vec![array, 0xc, 2, POINT_CONSTRUCT]]
        );
        assert_eq!(e.mem.u32(array + 0x14), 0x15);
        let block = e.mem.u32(this + 0x18);
        assert_eq!((e.mem.u32(block), e.mem.u32(block + 4)), (0xaa, 0xbb));
        let (_, log) = run(&mut e, 0x0051_d400, &args![this]);
        assert_eq!(only(&log, FREE), vec![vec![array], vec![block]]);
    }

    /// Doubles for the billboard builder: the setters write their float
    /// words into the temporary and answer it.
    fn billboard_engine() -> Engine {
        let mut e = list_engine();
        quiet(
            &mut e,
            &[
                SCOPE_GUARD_CONSTRUCT,
                SCOPE_GUARD_DESTRUCT,
                ARRAY_CONSTRUCT_STDCALL,
                POINTER_ASSIGN,
                POINTER_RELEASE,
                POINTER_CONSTRUCT,
            ],
        );
        e.register(POINT_SET, |e, a| {
            for i in 0..3 {
                e.mem.set_u32(a[0] + 4 * i, a[1 + i as usize]);
            }
            ret(a[0])
        });
        e.register(TEXTURE_COORDINATE_SET, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            e.mem.set_u32(a[0] + 4, a[2]);
            ret(a[0])
        });
        e.register(COLOR_SET, |e, a| {
            for i in 0..4 {
                e.mem.set_u32(a[0] + 4 * i, a[1 + i as usize]);
            }
            ret(a[0])
        });
        e.register(NI_ALLOC, |e, a| ret(e.mem.alloc(a[0])));
        e.register(NI_NEW, |e, a| ret(e.mem.alloc(a[0])));
        e.register(TRI_SHAPE_DATA_CONSTRUCT, |_, a| ret(a[0]));
        for (at, bits) in [
            (MINUS_ONE, (-1.0f32).to_bits()),
            (BILLBOARD_DEFAULT_SIZE, 1400.0f32.to_bits()),
        ] {
            e.map(at & !0xfff, 0x1000);
            e.set_global::<u32>(at, bits);
        }
        for (at, value) in [(ZERO_DOUBLE, 0.0f64), (TWO_DOUBLE, 2.0)] {
            e.map(at & !0xfff, 0x1000);
            e.set_global::<f64>(at, value);
        }
        e
    }

    fn floats(e: &Engine, at: u32, n: u32) -> Vec<f32> {
        (0..n)
            .map(|i| f32::from_bits(e.mem.u32(at + 4 * i)))
            .collect()
    }

    #[test]
    fn billboard_geometry_is_a_quad_of_the_tree_size() {
        let mut e = billboard_engine();
        let this = e.mem.alloc(0x98);
        e.mem.set_u32(this + 0x90, 100.0f32.to_bits());
        let (_, log) = run(&mut e, 0x0051_c490, &args![this, 1u32, 0x7777u32]);
        let data = only(&log, TRI_SHAPE_DATA_CONSTRUCT);
        assert_eq!(data.len(), 1);
        let d = &data[0];
        assert_eq!((d[1], d[6], d[7], d[8]), (4, 1, 0x1000, 2));
        assert_eq!(
            floats(&e, d[2], 12),
            vec![-50.0, 0.0, 0.0, -50.0, 0.0, 100.0, 50.0, 0.0, 100.0, 50.0, 0.0, 0.0]
        );
        let normals = floats(&e, d[3], 36);
        assert_eq!(normals[..3], [0.0, 1.0, 0.0]);
        assert_eq!(normals[12..15], [0.0, 0.0, -1.0]);
        assert_eq!(normals[33..], [1.0, 0.0, 0.0]);
        assert_eq!(floats(&e, d[4], 4), vec![1.0, 1.0, 1.0, 0.0]);
        assert_eq!(
            floats(&e, d[5], 8),
            vec![1.0, 1.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0]
        );
        let triangles: Vec<u16> = (0..6).map(|i| e.mem.u16(d[9] + 2 * i)).collect();
        assert_eq!(triangles, vec![0, 2, 1, 0, 3, 2]);
        let pointer = only(&log, POINTER_CONSTRUCT)[0].clone();
        assert_eq!(pointer[1], d[0]);
        assert_eq!(only(&log, POINTER_ASSIGN), vec![vec![0x7777, pointer[0]]]);
        assert_eq!(
            only(&log, SCOPE_GUARD_CONSTRUCT)[0][1..],
            [SCOPE_GUARD_TAG, 1, TREE_SOURCE_FILE, 0x199]
        );
    }

    #[test]
    fn billboard_geometry_lies_flat_and_defaults_its_size() {
        let mut e = billboard_engine();
        let this = e.mem.alloc(0x98);
        e.mem.set_u32(this + 0x90, (-3.0f32).to_bits());
        let (_, log) = run(&mut e, 0x0051_c490, &args![this, 0u32, 0x7777u32]);
        let d = &only(&log, TRI_SHAPE_DATA_CONSTRUCT)[0];
        assert_eq!(
            floats(&e, d[2], 12),
            vec![-700.0, 0.0, 0.0, -700.0, 1400.0, 0.0, 700.0, 1400.0, 0.0, 700.0, 0.0, 0.0]
        );
        e.register(NI_NEW, |_, _| ret(0));
        let (_, log) = run(&mut e, 0x0051_c490, &args![this, 0u32, 0x7777u32]);
        assert!(only(&log, TRI_SHAPE_DATA_CONSTRUCT).is_empty());
        assert_eq!(only(&log, POINTER_CONSTRUCT)[0][1], 0);
    }

    /// Doubles for `BuildDistant3D` and `CreateDistant3D`: objects come from
    /// `NI_NEW` with a shared vtable whose slots `+0x90` (fills the
    /// pointer with `texture`), `+0x94`, `+0xdc`, `+0xe4` and `+0` are
    /// doubles.
    fn distant_engine(texture: u32) -> Engine {
        let mut e = billboard_engine();
        let vtable = e.mem.alloc(0x100);
        let slots: Vec<u32> = (0..0x40u32).map(|i| 0x0f4a_0000 + 4 * i).collect();
        for (i, target) in slots.iter().enumerate() {
            e.mem.set_u32(vtable + 4 * i as u32, *target);
            e.register(*target, |_, _| Ret::default());
        }
        sizes_pass(&mut e);
        e.put_vtable(BILLBOARD_NODE_VTABLE, &slots);
        e.put_vtable(SHADER_TEXTURE_SET_VTABLE, &slots);
        let object = e.mem.alloc(0x10);
        e.mem.set_u32(object + 0xc, 0x4444);
        e.register_double(0x0f4a_0090, move |e, a| {
            e.mem.set_u32(a[2], texture);
            Ret::default()
        });
        e.register_double(NI_NEW, move |e, a| {
            let block = e.mem.alloc(a[0].max(0x100));
            e.mem.set_u32(block, vtable);
            ret(block)
        });
        e.register(POINTER_HOLDS, |e, a| {
            ret(u32::from(e.mem.u32(a[0]) == a[1]))
        });
        e.register(FIRST_WORD, |e, a| ret(e.mem.u32(a[0])));
        e.register(TRI_SHAPE_CONSTRUCT, |_, a| ret(a[0]));
        e.register(LIGHTING_PROPERTY_CONSTRUCT, |_, a| ret(a[0]));
        e.register_double(OBJECT_00664840, move |_, _| ret(object));
        quiet(&mut e, &[ATTACH_PROPERTY, LOAD_TEXTURE_SET]);
        e.register(NI_NODE_CONSTRUCT, |_, _| Ret::default());
        e.register(NI_OBJECT_CONSTRUCT, |_, _| Ret::default());
        e.register(ARRAY_CONSTRUCT, |_, _| Ret::default());
        e
    }

    /// A tree whose `BNAM` sizes pass `fn_0051d250` and whose model's name
    /// (slot `+0x14` of the model at `+0x30`) is "t.nif".
    fn distant_tree(e: &mut Engine) -> u32 {
        let this = e.mem.alloc(0x98);
        e.mem.set_u32(this + 0x8c, 1.0f32.to_bits());
        let model_vtable = e.mem.alloc(0x20);
        e.mem.set_u32(this + 0x30, model_vtable);
        e.mem.set_u32(model_vtable + 0x14, 0x0f4b_0014);
        let name = e.mem.alloc(8);
        e.mem.write(name, b"t.nif\0");
        e.register_double(0x0f4b_0014, move |_, _| ret(name));
        e.mem.set_u32(this + 0xc, 0x0000_1234);
        e.register(FORM_ID, |e, a| ret(e.mem.u32(a[0] + 0xc)));
        this
    }

    fn sizes_pass(e: &mut Engine) {
        for (at, value) in [
            (BNAM_LOWER, 0.0f64),
            (BNAM_FIRST_UPPER, 0.5),
            (BNAM_SECOND_UPPER, 0.5),
        ] {
            e.map(at & !0xfff, 0x1000);
            e.set_global::<f64>(at, value);
        }
        e.map(BILLBOARD_TEXTURE_PREFIX & !0xfff, 0x1000);
    }

    #[test]
    fn build_distant_3d_makes_a_billboard_node() {
        let mut e = distant_engine(0x6000);
        let this = distant_tree(&mut e);
        let (r, log) = run(&mut e, 0x0051_cba0, &args![this, 1u32]);
        assert_ne!(r.eax, 0);
        let shape = only(&log, TRI_SHAPE_CONSTRUCT)[0][0];
        let attached = only(&log, ATTACH_PROPERTY);
        assert_eq!(attached[0], vec![shape, 0x4444]);
        assert_eq!(attached.len(), 2);
        assert_eq!(only(&log, 0x0f4a_00dc), vec![vec![r.eax, shape, 1]]);
        assert_eq!(e.mem.u16(r.eax + BILLBOARD_FLAGS_OFFSET) & 7, 5);
        assert_eq!(only(&log, POINTER_RELEASE).len(), 3);
        let (r, _) = {
            let mut e = distant_engine(0);
            let this = distant_tree(&mut e);
            run(&mut e, 0x0051_cba0, &args![this, 1u32])
        };
        assert_eq!(r.eax, 0);
        e.mem.set_u32(this + 0x8c, 0.25f32.to_bits());
        let (r, log) = run(&mut e, 0x0051_cba0, &args![this, 1u32]);
        assert_eq!((r.eax, log.len()), (0, 1));
    }

    #[test]
    fn create_distant_3d_queues_or_adds() {
        let mut e = distant_engine(0x6000);
        let this = distant_tree(&mut e);
        e.register(DISTANT_CHECK_0051D740, |_, _| ret(1));
        e.register(DISTANT_FORM_CHECK, |_, _| ret(0));
        quiet(
            &mut e,
            &[TEXTURE_PATH, MODEL_LOADER_QUEUE, DISTANT_RECORD_CONSTRUCT],
        );
        e.register(DISTANT_ADD, |_, _| Ret::default());
        e.register(GET_TEXTURE, |e, a| {
            e.mem.set_u32(a[1], 0x6000);
            Ret::default()
        });
        let words = [this, 0x11, 0x22, 0x33, 0x10002, 0x55, 0x66, 0x77, 0x88];
        let (_, log) = run(&mut e, 0x0051_d440, &words);
        let added = only(&log, DISTANT_ADD);
        assert_eq!(added.len(), 1);
        assert_eq!(added[0][..3], [0x55, 0x66, 0x77]);
        assert_eq!(added[0][4..], [0x11, 0x33, 2]);
        let record = added[0][3];
        assert_eq!(e.mem.u32(record + 8), 0x1234);
        let node = e.mem.u32(record + 4);
        assert_ne!(node, 0);
        assert_eq!(only(&log, 0x0f4a_0000), vec![vec![node, 1]]);

        e.register(GET_TEXTURE, |_, _| Ret::default());
        e.register(FIRST_WORD, |_, _| ret(0));
        quiet(&mut e, &[ARRAY_CONSTRUCT_STDCALL]);
        e.map(MODEL_LOADER & !0xfff, 0x1000);
        e.set_global::<u32>(MODEL_LOADER, 0x0c00_0000);
        let points = e.mem.alloc(0x18);
        let ids = e.mem.alloc(8);
        let words = [this, points, 0, ids, 2, 0x55, 0x66, 0x77, 0];
        let (_, log) = run(&mut e, 0x0051_d440, &words);
        let queued = only(&log, MODEL_LOADER_QUEUE);
        assert_eq!(queued.len(), 1);
        assert_eq!(queued[0][0], 0x0c00_0000);
        let record = queued[0][2];
        assert_eq!(
            [0, 4, 8, 0xc, 0x10].map(|o| e.mem.u32(record + o)),
            [this, 0x55, 0x66, 0x77, 2]
        );
        assert!(only(&log, DISTANT_ADD).is_empty());

        e.register(DISTANT_FORM_CHECK, |_, _| ret(1));
        let (_, log) = run(&mut e, 0x0051_d440, &words);
        let added = only(&log, DISTANT_ADD);
        assert_eq!(e.mem.u32(added[0][3] + 4), 0);
        assert!(only(&log, TEXTURE_PATH).is_empty());

        for missing in [1usize, 3, 4, 7] {
            let mut short = words;
            short[missing] = 0;
            let (_, log) = run(&mut e, 0x0051_d440, &short);
            assert_eq!(only(&log, SCOPE_GUARD_DESTRUCT).len(), 1);
            assert!(only(&log, DISTANT_ADD).is_empty());
        }
        e.register(DISTANT_CHECK_0051D740, |_, _| ret(0x100));
        let (_, log) = run(&mut e, 0x0051_d440, &words);
        assert!(only(&log, FORM_ID).is_empty());
    }
}

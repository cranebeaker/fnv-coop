//! `SpeedTree/leafinfo.cpp` (Xbox PDB source unit), subsystem `SpeedTree`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `SIdvLeafInfo` describes a tree's leaves: the leaf textures, the blossom
//! and dimming settings and the generated leaf tables (vertexes, texture
//! coordinates and time offsets). The PC layout is the Xbox layout with the
//! vector at +0x10 grown by 8 bytes (the PC `std::vector` carries two more
//! words in front of its pointers), so every field from the spacing
//! tolerance on sits 8 bytes later than in the Xbox PDB.

#[allow(unused_imports)]
use crate::prelude::*;

use super::billboardleaf::CVec3;

/// `operator new(size)` (`cdecl`).
const OPERATOR_NEW: u32 = 0x0040_1000;
/// `std::basic_string<char>::basic_string(const char*)` (`thiscall`).
const STRING_FROM_C_STRING: u32 = 0x0044_a4c0;
/// `std::basic_string` copy constructor (`thiscall`: `this`, the source).
const STRING_COPY: u32 = 0x0044_a500;
/// `std::basic_string::assign(const string&, pos, count)` (`thiscall`).
const STRING_ASSIGN: u32 = 0x0044_a580;
/// `std::basic_string::_Tidy(built, new_size)` (`thiscall`).
const STRING_TIDY: u32 = 0x0044_a730;
/// The exception constructor `Parse` throws with (`thiscall`: the string
/// and a `bool`; unnamed in the engine map, `SpeedTree/speedtreert.cpp`).
const PARSE_ERROR_FROM_STRING: u32 = 0x00b0_9480;
/// `_CxxThrowException(object, throw_info)`.
const CXX_THROW_EXCEPTION: u32 = 0x00ec_7894;
/// The CRT's invalid-parameter handler (`cdecl`, no arguments) that the
/// vector's iterator checks call.
const INVALID_PARAMETER: u32 = 0x00ec_7c56;
/// `CIdvRandom::CIdvRandom` (`thiscall`, `00b3f100`).
const RANDOM_CONSTRUCTOR: u32 = 0x00b3_f100;
/// `CIdvRandom::GetUniform` (`thiscall`: `this`, from, to; `float` in ST0).
const RANDOM_GET_UNIFORM: u32 = 0x00b3_f130;
/// The `CIdvRandom` destructor (`thiscall`, unnamed in the engine map).
const RANDOM_DESTRUCTOR: u32 = 0x00aa_7830;
/// `CTreeFileAccess::ParseInt` (Xbox PDB, `thiscall`).
const PARSE_INT: u32 = 0x00b3_ed80;
/// `CTreeFileAccess::ParseFloat` (Xbox PDB, `thiscall`; `float` in ST0).
const PARSE_FLOAT: u32 = 0x00b3_ed20;
/// `CTreeFileAccess::ParseBool` (`thiscall`; the byte in `AL`, 0 or 1;
/// unnamed in the engine map).
const PARSE_BOOL: u32 = 0x00b0_6180;
/// `CTreeFileAccess::ParseVector3` (Xbox PDB, `thiscall`: `this`, the
/// result buffer; returns it).
const PARSE_VECTOR3: u32 = 0x00b3_ef80;
/// `CTreeFileAccess::ParseString` (Xbox PDB, `thiscall`: `this`, the result
/// string; returns it).
const PARSE_STRING: u32 = 0x00b3_ee40;
/// `CIdvFilename::NoPath` (Xbox PDB, `thiscall`: the filename, the result
/// string; returns it).
const FILENAME_NO_PATH: u32 = 0x00b0_9f90;
/// `std::vector<SIdvLeafTexture>::clear()` (`thiscall`; an erase of the
/// whole range, unnamed in the engine map).
const TEXTURE_VECTOR_CLEAR: u32 = 0x00b2_df90;
/// `SIdvLeafTexture` copy construction into raw memory (`thiscall`: the
/// destination; the source; unnamed in the engine map).
const TEXTURE_COPY: u32 = 0x00b2_e680;
/// The vector's insert-n-copies (`thiscall`: iterator proxy, iterator
/// position, count, the address of the value; unnamed in the engine map).
const TEXTURE_VECTOR_INSERT: u32 = 0x00b3_dbd0;
/// The vector iterator's `operator+=` (`thiscall`: the iterator, the count
/// of elements; unnamed in the engine map).
const TEXTURE_ITERATOR_ADVANCE: u32 = 0x00b3_e390;
/// The vector's erase of a range (`thiscall`: the result iterator, first
/// proxy and position, last proxy and position; unnamed in the engine map).
const TEXTURE_VECTOR_ERASE: u32 = 0x00b2_e250;

/// `10000.0f`: the end of the range the time offsets are drawn from.
const TIME_OFFSET_RANGE: u32 = 0x0102_2958;
/// `0.8f`: the default leaf colour.
const DEFAULT_COLOR: u32 = 0x0103_0ff0;
/// `0.2f`: the default colour variance.
const DEFAULT_COLOR_VARIANCE: u32 = 0x0101_df44;
/// `0.5f`: the default origin's x.
const DEFAULT_ORIGIN_X: u32 = 0x0101_6248;
/// The default size (x and y).
const DEFAULT_SIZE: u32 = 0x010a_587c;
/// `10.0f`: the default used size (x and y).
const DEFAULT_SIZE_USED: u32 = 0x0101_7b78;
/// `std::string::npos`.
const STRING_NPOS: u32 = 0x0101_73f0;
/// `"malformed general leaf information"`.
const MESSAGE_MALFORMED_GENERAL: u32 = 0x010a_5834;
/// `"malformed single leaf information"`.
const MESSAGE_MALFORMED_SINGLE: u32 = 0x010a_5858;
/// The throw information of the parse-error exception.
const PARSE_THROW_INFO: u32 = 0x0117_0384;

layout! {
    /// `SIdvLeafTexture` (Xbox PDB), 0x54 bytes; the PC layout is the same.
    /// The filename (`CIdvFilename`, a `std::string`) is at +0x14.
    pub struct SIdvLeafTexture: 0x54 {
        /// `m_bBlossom` (Xbox PDB).
        0x00 m_bBlossom: u8,
        /// `m_cColor` (Xbox PDB).
        0x04 m_cColor: Inline<CVec3>,
        /// `m_fColorVariance` (Xbox PDB).
        0x10 m_fColorVariance: f32,
        /// `m_cOrigin` (Xbox PDB).
        0x30 m_cOrigin: Inline<CVec3>,
        /// `m_cSize` (Xbox PDB).
        0x3C m_cSize: Inline<CVec3>,
        /// `m_cSizeUsed` (Xbox PDB).
        0x48 m_cSizeUsed: Inline<CVec3>,
    }

    /// The PC `std::vector<SIdvLeafTexture>` as the code reads it: the
    /// iterator-debugging proxy word, then the first, last and end
    /// pointers.
    pub struct LeafTextureVector: 0x18 {
        /// The container proxy (iterator checking).
        0x00 proxy: u32,
        /// `_Myfirst`.
        0x0C first: u32,
        /// `_Mylast`.
        0x10 last: u32,
        /// `_Myend`.
        0x14 end: u32,
    }

    /// `SIdvLeafInfo` (Xbox PDB) on PC, 0x58 bytes; the fields from
    /// `m_fSpacingTolerance` on are 8 bytes after the Xbox offsets.
    pub struct SIdvLeafInfo: 0x58 {
        /// `m_bDimming` (Xbox PDB).
        0x00 m_bDimming: u8,
        /// `m_fDimmingScalar` (Xbox PDB).
        0x04 m_fDimmingScalar: f32,
        /// `m_eCollisionType` (Xbox PDB).
        0x0C m_eCollisionType: i32,
        /// `m_vLeafTextures` (Xbox PDB).
        0x10 m_vLeafTextures: Inline<LeafTextureVector>,
        /// `m_fSpacingTolerance` (Xbox PDB, +0x20).
        0x28 m_fSpacingTolerance: f32,
        /// `m_fBlossomDistance` (Xbox PDB, +0x24).
        0x2C m_fBlossomDistance: f32,
        /// `m_fBlossomWeighting` (Xbox PDB, +0x28).
        0x30 m_fBlossomWeighting: f32,
        /// `m_nBlossomLevel` (Xbox PDB, +0x2c).
        0x34 m_nBlossomLevel: i32,
        /// `m_nNumRockingGroups` (Xbox PDB, +0x38).
        0x40 m_nNumRockingGroups: i32,
        /// `m_nNumLeafLodLevels` (Xbox PDB, +0x3c).
        0x44 m_nNumLeafLodLevels: i32,
        /// `m_nNumTextures` (Xbox PDB, +0x40).
        0x48 m_nNumTextures: i32,
        /// `m_pLeafVertexes` (Xbox PDB, +0x44): `float**`, one table per
        /// leaf LOD level.
        0x4C m_pLeafVertexes: Ptr,
        /// `m_pLeafTexCoords` (Xbox PDB, +0x48): `float*`.
        0x50 m_pLeafTexCoords: Ptr,
        /// `m_pTimeOffsets` (Xbox PDB, +0x4c): `float*`.
        0x54 m_pTimeOffsets: Ptr,
    }
}

/// An `SIdvLeafTexture` passed by value: its 21 words, as the code pushes
/// them.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LeafTextureArg(pub [u32; 21]);

impl Arg for LeafTextureArg {
    const WORDS: usize = 21;
    fn take(words: &[u32], i: &mut usize) -> Self {
        let mut value = [0u32; 21];
        for word in value.iter_mut() {
            *word = u32::take(words, i);
        }
        LeafTextureArg(value)
    }
    fn put(self, out: &mut Vec<u32>) {
        out.extend_from_slice(&self.0);
    }
}

/// `count * element` for `operator new`: the product, or all ones when it
/// overflows 32 bits (`MUL`, `SETO`, `NEG`, `OR`).
fn array_size(count: u32, element: u32) -> u32 {
    count.checked_mul(element).unwrap_or(0xffff_ffff)
}

/// Copies `words` 32-bit words from `from` to `to`.
fn copy_words(e: &mut Engine, from: u32, to: u32, words: u32) {
    for i in 0..words {
        let word = e.mem.u32(from.wrapping_add(4 * i));
        e.mem.set_u32(to.wrapping_add(4 * i), word);
    }
}

/// Stores the float at `global` (`FLD`, `FSTP`: a signalling NaN comes back
/// quiet) to `to`.
fn store_global_float(e: &mut Engine, global: u32, to: u32) {
    let bits = x87_quiet_bits(e.mem.u32(global));
    e.mem.set_u32(to, bits);
}

// Translated from 00b3cf40 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b3cf40 - exception unwinding frame (FS:[0], /GS cookie) is not translated.
/// `SIdvLeafInfo::InitTables` (identified from the Xbox PDB layout it
/// fills; the engine map has no name): records the texture count and builds
/// the leaf tables.
///
/// * the time offsets: one uniform draw in `[0, 10000)` per rocking group;
/// * the texture coordinates: for each of `2 * count` quads, the 8 floats
///   0, 1, 1, 1, 1, 0, 0, 0;
/// * the leaf vertexes: one table per leaf LOD level of
///   `2 * count * rocking groups * 16` floats, all zero.
///
/// Every allocation size is the element count times 4, or all ones when
/// that overflows. A local `CIdvRandom` is made for the draws and destroyed
/// at the end.
pub fn sidv_leaf_info_init_tables(e: &mut Engine, this: Ptr<SIdvLeafInfo>, count: i32) {
    e.set(this, SIdvLeafInfo::m_nNumTextures, count);
    let rocking_groups = e.get(this, SIdvLeafInfo::m_nNumRockingGroups);
    let size = array_size(rocking_groups as u32, 4);
    let time_offsets = e.call(OPERATOR_NEW, &args![size]).u32();
    e.set(this, SIdvLeafInfo::m_pTimeOffsets, Ptr::new(time_offsets));

    // The exe's frame (`EBP` frame, exception record, `SUB ESP,0x30`, the cookie): 0x40 bytes,
    // the random generator (one byte) at +0x2b (`EBP-0x15`).
    e.with_stack(0x40, |e, frame| {
        let random = frame.addr() + 0x2b;
        e.call(RANDOM_CONSTRUCTOR, &args![random]);
        let mut i: i32 = 0;
        while i < e.get(this, SIdvLeafInfo::m_nNumRockingGroups) {
            let end = x87_quiet(e.global::<f32>(TIME_OFFSET_RANGE));
            let drawn = e
                .call(RANDOM_GET_UNIFORM, &args![random, 0.0f32, end])
                .f32();
            let table = e.get(this, SIdvLeafInfo::m_pTimeOffsets).addr();
            e.mem
                .set_f32(table.wrapping_add((i as u32).wrapping_mul(4)), drawn);
            i = i.wrapping_add(1);
        }

        let size = array_size((count as u32) << 4, 4);
        let tex_coords = e.call(OPERATOR_NEW, &args![size]).u32();
        e.set(this, SIdvLeafInfo::m_pLeafTexCoords, Ptr::new(tex_coords));
        let mut cursor = tex_coords;
        let mut i: i32 = 0;
        while i < count.wrapping_shl(1) {
            for value in [0.0f32, 1.0, 1.0, 1.0, 1.0, 0.0, 0.0, 0.0] {
                e.mem.set_f32(cursor, value);
                cursor = cursor.wrapping_add(4);
            }
            i = i.wrapping_add(1);
        }

        let lod_levels = e.get(this, SIdvLeafInfo::m_nNumLeafLodLevels);
        let size = array_size(lod_levels as u32, 4);
        let vertexes = e.call(OPERATOR_NEW, &args![size]).u32();
        e.set(this, SIdvLeafInfo::m_pLeafVertexes, Ptr::new(vertexes));
        let rocking_groups = e.get(this, SIdvLeafInfo::m_nNumRockingGroups);
        let floats = (count.wrapping_shl(1).wrapping_mul(rocking_groups)).wrapping_shl(4);
        let mut level: i32 = 0;
        while level < e.get(this, SIdvLeafInfo::m_nNumLeafLodLevels) {
            let size = array_size(floats as u32, 4);
            let table = e.call(OPERATOR_NEW, &args![size]).u32();
            let tables = e.get(this, SIdvLeafInfo::m_pLeafVertexes).addr();
            e.mem
                .set_u32(tables.wrapping_add((level as u32).wrapping_mul(4)), table);
            let mut j: i32 = 0;
            while j < floats {
                let tables = e.get(this, SIdvLeafInfo::m_pLeafVertexes).addr();
                let table = e
                    .mem
                    .u32(tables.wrapping_add((level as u32).wrapping_mul(4)));
                e.mem
                    .set_f32(table.wrapping_add((j as u32).wrapping_mul(4)), 0.0);
                j = j.wrapping_add(1);
            }
            level = level.wrapping_add(1);
        }
        e.call(RANDOM_DESTRUCTOR, &args![random]);
    });
}

// Translated from 00b3d720 (decompiled, FalloutNV.exe 1.4.0.525)
/// `SIdvLeafTexture::SIdvLeafTexture` (identified from the Xbox PDB layout
/// it fills; the engine map has no name): not a blossom, colour (0.8, 0.8,
/// 0.8), colour variance 0.2, an empty filename, origin (0.5, 1, 0), size
/// (`DEFAULT_SIZE`, `DEFAULT_SIZE`, 0) and used size (10, 10, 0). Returns
/// `this`.
pub fn fn_00b3d720(e: &mut Engine, this: Ptr<SIdvLeafTexture>) -> Ptr<SIdvLeafTexture> {
    e.set(this, SIdvLeafTexture::m_bBlossom, 0);
    let color = this.at(SIdvLeafTexture::m_cColor);
    store_global_float(e, DEFAULT_COLOR, color.addr());
    store_global_float(e, DEFAULT_COLOR, color.addr() + 4);
    store_global_float(e, DEFAULT_COLOR, color.addr() + 8);
    let variance = x87_quiet(e.global::<f32>(DEFAULT_COLOR_VARIANCE));
    e.set(this, SIdvLeafTexture::m_fColorVariance, variance);
    e.call(STRING_TIDY, &args![this.byte_add(0x14), false, 0u32]);
    let origin = this.at(SIdvLeafTexture::m_cOrigin);
    store_global_float(e, DEFAULT_ORIGIN_X, origin.addr());
    e.set(origin, CVec3::y, 1.0);
    e.set(origin, CVec3::z, 0.0);
    let size = this.at(SIdvLeafTexture::m_cSize);
    store_global_float(e, DEFAULT_SIZE, size.addr());
    store_global_float(e, DEFAULT_SIZE, size.addr() + 4);
    e.set(size, CVec3::z, 0.0);
    let used = this.at(SIdvLeafTexture::m_cSizeUsed);
    store_global_float(e, DEFAULT_SIZE_USED, used.addr());
    store_global_float(e, DEFAULT_SIZE_USED, used.addr() + 4);
    e.set(used, CVec3::z, 0.0);
    this
}

/// The vector's iterator check (`_Orphan`/range check of the debug
/// runtime): the invalid-parameter handler runs when the vector is null,
/// the position is before the first element or after the last. The handler
/// does not return in the exe; the code after it is the normal path.
fn check_iterator(e: &mut Engine, vector: u32, position: u32) {
    let first = e.mem.u32(vector.wrapping_add(0x0c));
    let last = e.mem.u32(vector.wrapping_add(0x10));
    if vector == 0 || first > position || position > last {
        e.call(INVALID_PARAMETER, &args![]);
    }
}

/// The number of elements in the vector at `vector`: `(last - first) / 0x54`
/// with a signed division.
fn texture_vector_size(e: &mut Engine, vector: u32) -> u32 {
    let first = e.mem.u32(vector.wrapping_add(0x0c));
    let last = e.mem.u32(vector.wrapping_add(0x10));
    ((last.wrapping_sub(first) as i32) / 0x54) as u32
}

// Translated from 00b3d910 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b3d910 - exception unwinding frame (FS:[0], /GS cookie) is not translated.
/// `std::vector<SIdvLeafTexture>::resize(new_size, value)` (`thiscall`, the
/// value passed by value; unnamed in the engine map): grows with copies of
/// `value` (the insert at the end) or erases the elements from `new_size`
/// on. The sizes are compared as unsigned numbers, so a vector whose last
/// pointer is before its first one takes the erase path.
pub fn texture_vector_resize(e: &mut Engine, this: Ptr, new_size: u32, value: LeafTextureArg) {
    // The exe's frame (`EBP` frame, exception record, `SUB ESP,0xc8`, the cookie): 0xd8 bytes
    // below `EBP`, then the saved `EBP`, the return address, `new_size` (+0xe0) and the texture
    // passed by value (+0xe4, `EBP+0xc`), which is the copy inserted. The advanced iterator at
    // +0x74 (`EBP-0x64`) and the erase result at +0x88 (`EBP-0x50`).
    e.with_stack(0x138, |e, frame| {
        resize_in_frame(e, this, new_size, value, frame.addr())
    });
}

/// [`texture_vector_resize`] in the exe's frame (`frame` is its lowest address).
fn resize_in_frame(e: &mut Engine, this: Ptr, new_size: u32, value: LeafTextureArg, frame: u32) {
    let vector = this.addr();
    e.mem.set_u32(frame + 0xe0, new_size);
    for (i, word) in value.0.iter().enumerate() {
        e.mem.set_u32(frame + 0xe4 + 4 * i as u32, *word);
    }
    if texture_vector_size(e, vector) < new_size {
        let last = e.mem.u32(vector.wrapping_add(0x10));
        check_iterator(e, vector, last);
        let proxy = e.mem.u32(vector);
        let missing = new_size.wrapping_sub(texture_vector_size(e, vector));
        let copy = frame + 0xe4;
        e.call(
            TEXTURE_VECTOR_INSERT,
            &args![this, proxy, last, missing, copy],
        );
    } else if new_size < texture_vector_size(e, vector) {
        let last = e.mem.u32(vector.wrapping_add(0x10));
        check_iterator(e, vector, last);
        let last_proxy = e.mem.u32(vector);
        let first = e.mem.u32(vector.wrapping_add(0x0c));
        check_iterator(e, vector, first);
        let first_proxy = e.mem.u32(vector);
        let iterator = frame + 0x74;
        e.mem.set_u32(iterator, first_proxy);
        e.mem.set_u32(iterator + 4, first);
        e.call(TEXTURE_ITERATOR_ADVANCE, &args![iterator, new_size]);
        let erase_proxy = e.mem.u32(iterator);
        let erase_position = e.mem.u32(iterator + 4);
        let result = frame + 0x88;
        e.call(
            TEXTURE_VECTOR_ERASE,
            &args![this, result, erase_proxy, erase_position, last_proxy, last],
        );
    }
}

/// Throws the parse-error exception with `message` (`"malformed ..."`):
/// the message string, the exception built from it, `_CxxThrowException`.
/// `text` and `exception` are the two locals of the throw site in `00b3d1b0`'s frame.
fn throw_malformed(e: &mut Engine, message: u32, text: u32, exception: u32) -> ! {
    e.call(STRING_FROM_C_STRING, &args![text, message]);
    e.call(PARSE_ERROR_FROM_STRING, &args![exception, text, 0u32]);
    e.call(CXX_THROW_EXCEPTION, &args![exception, PARSE_THROW_INFO]);
    unreachable!("__CxxThrowException does not return (00ec7894)")
}

/// Reads a `CVec3` from the file into the three words at `to`, through the local `buffer`.
fn parse_vector3_into(e: &mut Engine, file: Ptr, to: u32, buffer: u32) {
    let parsed = e.call(PARSE_VECTOR3, &args![file, buffer]).u32();
    copy_words(e, parsed, to, 3);
}

/// The offsets in `00b3d1b0`'s frame, which starts at `EBP-0x384` (see
/// [`sidv_leaf_info_parse`]).
mod parse_frame {
    /// The texture being parsed (`EBP-0x74`).
    pub const TEXTURE: u32 = 0x310;
    /// The vector buffers of the colour, origin, size and used size (`EBP-0x80`, `-0xc8`,
    /// `-0xd4`, `-0xe0`).
    pub const COLOR: u32 = 0x304;
    pub const ORIGIN: u32 = 0x2bc;
    pub const SIZE: u32 = 0x2b0;
    pub const USED_SIZE: u32 = 0x2a4;
    /// The parsed filename, its copy and the name without the path (`EBP-0x1e0`, `-0x9c`,
    /// `-0xbc`).
    pub const PARSED: u32 = 0x1a4;
    pub const COPY: u32 = 0x2e8;
    pub const NAME: u32 = 0x2c8;
    /// The texture throw's message and exception (`EBP-0x124`, `-0x108`).
    pub const TEXTURE_TEXT: u32 = 0x260;
    pub const TEXTURE_EXCEPTION: u32 = 0x27c;
    /// The general throw's message and exception (`EBP-0x168`, `-0x14c`).
    pub const GENERAL_TEXT: u32 = 0x21c;
    pub const GENERAL_EXCEPTION: u32 = 0x238;
}

/// One leaf texture's tag/value pairs, read from the file into the texture
/// at `texture` (a default-constructed local) until the tag 0x3f0. The
/// first value read is a discarded index, the second the first tag. Tags
/// 0xfa0 (the blossom flag), 0xfa1 (colour), 0xfa2 (colour variance), 0xfa3
/// (the filename, reduced to its name without the path), 0xfa4 (origin),
/// 0xfa5 (size) and 0xfa6 (used size) fill the fields; 0xfa7 reads a float
/// and drops it. Any other tag throws `"malformed single leaf information"`.
/// `frame` is `00b3d1b0`'s frame ([`parse_frame`]).
fn parse_texture(e: &mut Engine, file: Ptr, texture: Ptr<SIdvLeafTexture>, frame: u32) {
    let _discarded = e.call(PARSE_INT, &args![file]).u32();
    let mut tag = e.call(PARSE_INT, &args![file]).u32();
    loop {
        match tag.wrapping_sub(0xfa0) {
            0 => {
                let blossom = e.call(PARSE_BOOL, &args![file]).u8();
                e.set(texture, SIdvLeafTexture::m_bBlossom, blossom);
            }
            1 => parse_vector3_into(e, file, texture.addr() + 0x04, frame + parse_frame::COLOR),
            2 => {
                let variance = e.call(PARSE_FLOAT, &args![file]).f32();
                e.set(texture, SIdvLeafTexture::m_fColorVariance, variance);
            }
            3 => {
                let npos = e.mem.u32(STRING_NPOS);
                let filename = texture.addr() + 0x14;
                let parsed = frame + parse_frame::PARSED;
                e.call(PARSE_STRING, &args![file, parsed]);
                let copy = frame + parse_frame::COPY;
                e.call(STRING_COPY, &args![copy, parsed]);
                e.call(STRING_TIDY, &args![parsed, true, 0u32]);
                e.call(STRING_ASSIGN, &args![filename, copy, 0u32, npos]);
                e.call(STRING_TIDY, &args![copy, true, 0u32]);
                let name = frame + parse_frame::NAME;
                let reduced = e.call(FILENAME_NO_PATH, &args![filename, name]).u32();
                e.call(STRING_ASSIGN, &args![filename, reduced, 0u32, npos]);
                e.call(STRING_TIDY, &args![name, true, 0u32]);
            }
            4 => parse_vector3_into(e, file, texture.addr() + 0x30, frame + parse_frame::ORIGIN),
            5 => parse_vector3_into(e, file, texture.addr() + 0x3c, frame + parse_frame::SIZE),
            6 => parse_vector3_into(
                e,
                file,
                texture.addr() + 0x48,
                frame + parse_frame::USED_SIZE,
            ),
            7 => {
                e.call(PARSE_FLOAT, &args![file]);
            }
            _ => throw_malformed(
                e,
                MESSAGE_MALFORMED_SINGLE,
                frame + parse_frame::TEXTURE_TEXT,
                frame + parse_frame::TEXTURE_EXCEPTION,
            ),
        }
        tag = e.call(PARSE_INT, &args![file]).u32();
        if tag == 0x3f0 {
            break;
        }
    }
}

/// The texture block (tag 0x3f1): a discarded value and the texture count,
/// the vector cleared and resized to the count with default textures, then
/// each texture parsed into a default local and copied into place; one more
/// value (the block's end tag) is read and dropped.
/// `frame` is `00b3d1b0`'s frame ([`parse_frame`]); the default texture is built in the
/// outgoing argument area 0x54 bytes below it (`SUB ESP,0x54`, 00b3d34d), under the pushed
/// count.
fn parse_textures(e: &mut Engine, this: Ptr<SIdvLeafInfo>, file: Ptr, frame: u32) {
    let _discarded = e.call(PARSE_INT, &args![file]).u32();
    let count = e.call(PARSE_INT, &args![file]).i32();
    let vector = this.at(SIdvLeafInfo::m_vLeafTextures);
    e.call(TEXTURE_VECTOR_CLEAR, &args![vector]);
    let default = frame.wrapping_sub(0x54);
    fn_00b3d720(e, Ptr::new(default));
    let mut value = [0u32; 21];
    for (i, word) in value.iter_mut().enumerate() {
        *word = e.mem.u32(default + 4 * i as u32);
    }
    texture_vector_resize(e, vector.cast(), count as u32, LeafTextureArg(value));

    let mut i: i32 = 0;
    while i < count {
        let local = Ptr::<SIdvLeafTexture>::new(frame + parse_frame::TEXTURE);
        fn_00b3d720(e, local);
        parse_texture(e, file, local, frame);
        let textures = vector.addr();
        let size = texture_vector_size(e, textures);
        if (i as u32) >= size {
            e.call(INVALID_PARAMETER, &args![]);
        }
        let destination = (i as u32)
            .wrapping_mul(0x54)
            .wrapping_add(e.mem.u32(textures.wrapping_add(0x0c)));
        e.call(TEXTURE_COPY, &args![destination, local]);
        e.call(STRING_TIDY, &args![local.byte_add(0x14), true, 0u32]);
        i = i.wrapping_add(1);
    }
    let _end = e.call(PARSE_INT, &args![file]).u32();
}

// Translated from 00b3d1b0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b3d1b0 - exception unwinding frame (FS:[0], /GS cookie) is not translated; the throws are.
/// `SIdvLeafInfo::Parse` (Xbox PDB, `thiscall` with the file as the one
/// stack argument): reads the leaf block from the tree file into `this`.
/// The first tag is processed before any end test; after each value the
/// next tag is read and 0x3ed ends the block.
///
/// * 0xbb8 reads a float into the blossom distance; 0xbb9 an integer into the
///   blossom level; 0xbba a float into the blossom weighting; 0xbbf a float
///   into the spacing tolerance; 0xbc0 an integer into the collision type;
///   0xbc1 a byte into the dimming flag; 0xbc2 a float into the dimming
///   scalar;
/// * 0xbbb and 0xbbe read a byte, 0xbbc and 0xbbd a float, and drop them;
/// * 0x3f1 is the texture block ([`parse_textures`]);
/// * any other tag throws `"malformed general leaf information"`.
pub fn sidv_leaf_info_parse(e: &mut Engine, this: Ptr<SIdvLeafInfo>, file: Ptr) {
    // The exe's frame (`EBP` frame, exception record, `SUB ESP,0x374`, the cookie): 0x384 bytes,
    // with 0x58 bytes of outgoing arguments below it for the texture resize.
    e.with_stack(0x58 + 0x384, |e, raw| {
        parse_in_frame(e, this, file, raw.addr() + 0x58)
    });
}

/// [`sidv_leaf_info_parse`] in the exe's frame (`frame` is its lowest address).
fn parse_in_frame(e: &mut Engine, this: Ptr<SIdvLeafInfo>, file: Ptr, frame: u32) {
    let mut tag = e.call(PARSE_INT, &args![file]).u32();
    loop {
        match tag {
            0xbb8 => {
                let value = e.call(PARSE_FLOAT, &args![file]).f32();
                e.set(this, SIdvLeafInfo::m_fBlossomDistance, value);
            }
            0x3f1 => parse_textures(e, this, file, frame),
            0xbb9 => {
                let value = e.call(PARSE_INT, &args![file]).i32();
                e.set(this, SIdvLeafInfo::m_nBlossomLevel, value);
            }
            0xbba => {
                let value = e.call(PARSE_FLOAT, &args![file]).f32();
                e.set(this, SIdvLeafInfo::m_fBlossomWeighting, value);
            }
            0xbbb | 0xbbe => {
                e.call(PARSE_BOOL, &args![file]);
            }
            0xbbc | 0xbbd => {
                e.call(PARSE_FLOAT, &args![file]);
            }
            0xbbf => {
                let value = e.call(PARSE_FLOAT, &args![file]).f32();
                e.set(this, SIdvLeafInfo::m_fSpacingTolerance, value);
            }
            0xbc0 => {
                let value = e.call(PARSE_INT, &args![file]).i32();
                e.set(this, SIdvLeafInfo::m_eCollisionType, value);
            }
            0xbc1 => {
                let value = e.call(PARSE_BOOL, &args![file]).u8();
                e.set(this, SIdvLeafInfo::m_bDimming, value);
            }
            0xbc2 => {
                let value = e.call(PARSE_FLOAT, &args![file]).f32();
                e.set(this, SIdvLeafInfo::m_fDimmingScalar, value);
            }
            _ => throw_malformed(
                e,
                MESSAGE_MALFORMED_GENERAL,
                frame + parse_frame::GENERAL_TEXT,
                frame + parse_frame::GENERAL_EXCEPTION,
            ),
        }
        tag = e.call(PARSE_INT, &args![file]).u32();
        if tag == 0x3ed {
            break;
        }
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00b3cf40,
            sidv_leaf_info_init_tables(Ptr<SIdvLeafInfo>, i32)
        ),
        entry!(0x00b3d1b0, sidv_leaf_info_parse(Ptr<SIdvLeafInfo>, Ptr)),
        entry!(
            0x00b3d720,
            fn_00b3d720(Ptr<SIdvLeafTexture>) -> Ptr<SIdvLeafTexture>
        ),
        entry!(0x00b3d910, texture_vector_resize(Ptr, u32, LeafTextureArg)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    type Calls = Rc<RefCell<Vec<(u32, Vec<u32>)>>>;

    /// Records the calls to `addresses`; each double answers `Ret::default()`.
    fn recorded(e: &mut Engine, addresses: &[u32]) -> Calls {
        let calls: Calls = Rc::new(RefCell::new(Vec::new()));
        for &address in addresses {
            let record = calls.clone();
            e.register_double(address, move |_, a| {
                record.borrow_mut().push((address, a.to_vec()));
                Ret::default()
            });
        }
        calls
    }

    /// An engine with the globals the unit reads.
    fn leaf_engine() -> Engine {
        let mut e = Engine::new();
        for page in [
            0x0101_6000,
            0x0101_7000,
            0x0101_d000,
            0x0102_2000,
            0x0103_0000,
            0x010a_5000,
        ] {
            e.map(page, 0x1000);
        }
        e.set_global(TIME_OFFSET_RANGE, 10000.0f32);
        e.set_global(DEFAULT_COLOR, 0.8f32);
        e.set_global(DEFAULT_COLOR_VARIANCE, 0.2f32);
        e.set_global(DEFAULT_ORIGIN_X, 0.5f32);
        e.set_global(DEFAULT_SIZE, 0.12f32);
        e.set_global(DEFAULT_SIZE_USED, 10.0f32);
        e.set_global(STRING_NPOS, u32::MAX);
        e
    }

    fn words(e: &Engine, address: u32, count: u32) -> Vec<u32> {
        (0..count).map(|i| e.mem.u32(address + 4 * i)).collect()
    }

    #[test]
    fn the_layout_matches_the_pc_offsets() {
        assert_eq!(SIdvLeafInfo::SIZE, 0x58);
        assert_eq!(SIdvLeafTexture::SIZE, 0x54);
        assert_eq!(SIdvLeafInfo::m_nNumRockingGroups.off, 0x40);
        assert_eq!(SIdvLeafInfo::m_pTimeOffsets.off, 0x54);
    }

    #[test]
    fn the_default_texture_is_filled_in() {
        let mut e = leaf_engine();
        let calls = recorded(&mut e, &[STRING_TIDY]);
        let texture = e.new_object::<SIdvLeafTexture>();
        for i in 0..21 {
            e.mem.set_u32(texture.addr() + 4 * i, 0xdead_beef);
        }
        let returned = e
            .call(0x00b3_d720, &args![texture])
            .ptr::<SIdvLeafTexture>();
        assert_eq!(returned, texture);
        let base = texture.addr();
        assert_eq!(e.mem.u8(base), 0);
        assert_eq!(f32s(&e, base + 4, 3), [0.8, 0.8, 0.8]);
        assert_eq!(f32s(&e, base + 0x10, 1), [0.2]);
        // The filename is only touched through its constructor.
        assert_eq!(e.mem.u32(base + 0x14), 0xdead_beef);
        assert_eq!(
            *calls.borrow(),
            vec![(STRING_TIDY, vec![base + 0x14, 0, 0])]
        );
        assert_eq!(f32s(&e, base + 0x30, 3), [0.5, 1.0, 0.0]);
        assert_eq!(f32s(&e, base + 0x3c, 3), [0.12, 0.12, 0.0]);
        assert_eq!(f32s(&e, base + 0x48, 3), [10.0, 10.0, 0.0]);
    }

    fn f32s(e: &Engine, address: u32, count: u32) -> Vec<f32> {
        (0..count)
            .map(|i| f32::from_bits(e.mem.u32(address + 4 * i)))
            .collect()
    }

    /// An engine for `InitTables`: `operator new` hands out blocks and logs
    /// the sizes, `GetUniform` answers 1, 2, 3, ...
    fn tables_engine() -> (Engine, Calls, Rc<RefCell<Vec<u32>>>) {
        let mut e = leaf_engine();
        let sizes = Rc::new(RefCell::new(Vec::new()));
        let log = sizes.clone();
        e.register_double(OPERATOR_NEW, move |e, a| {
            log.borrow_mut().push(a[0]);
            let block = e.mem.alloc(a[0].clamp(4, 0x4000));
            for i in 0..a[0].clamp(4, 0x4000) / 4 {
                e.mem.set_u32(block + 4 * i, 0xdead_beef);
            }
            Ret {
                eax: block,
                ..Ret::default()
            }
        });
        let calls = recorded(&mut e, &[RANDOM_CONSTRUCTOR, RANDOM_DESTRUCTOR]);
        let log = calls.clone();
        let mut next = 0.0;
        e.register_double(RANDOM_GET_UNIFORM, move |_, a| {
            log.borrow_mut().push((RANDOM_GET_UNIFORM, a.to_vec()));
            next += 1.0;
            Ret {
                st0: next,
                ..Ret::default()
            }
        });
        (e, calls, sizes)
    }

    #[test]
    fn init_tables_builds_the_three_tables() {
        let (mut e, calls, sizes) = tables_engine();
        let info = e.new_object::<SIdvLeafInfo>();
        e.set(info, SIdvLeafInfo::m_nNumRockingGroups, 3);
        e.set(info, SIdvLeafInfo::m_nNumLeafLodLevels, 2);
        e.call(0x00b3_cf40, &args![info, 2u32]);
        assert_eq!(e.get(info, SIdvLeafInfo::m_nNumTextures), 2);
        // 3 offsets, 2 * 16 coordinates, 2 table pointers, then two tables
        // of 2 * 2 * 3 * 16 = 192 floats.
        assert_eq!(*sizes.borrow(), vec![12, 128, 8, 768, 768]);
        let offsets = e.get(info, SIdvLeafInfo::m_pTimeOffsets).addr();
        assert_eq!(f32s(&e, offsets, 3), [1.0, 2.0, 3.0]);
        let coordinates = e.get(info, SIdvLeafInfo::m_pLeafTexCoords).addr();
        let quad = [0.0f32, 1.0, 1.0, 1.0, 1.0, 0.0, 0.0, 0.0];
        for q in 0..4 {
            assert_eq!(f32s(&e, coordinates + 32 * q, 8), quad);
        }
        let tables = e.get(info, SIdvLeafInfo::m_pLeafVertexes).addr();
        for level in 0..2 {
            let table = e.mem.u32(tables + 4 * level);
            assert!(words(&e, table, 192).iter().all(|&w| w == 0));
        }
        // The draws are in [0, 10000) from the local generator, which is
        // made first and destroyed last.
        let log = calls.borrow();
        assert_eq!(log.first().unwrap().0, RANDOM_CONSTRUCTOR);
        assert_eq!(log.last().unwrap().0, RANDOM_DESTRUCTOR);
        let draws: Vec<_> = log.iter().filter(|c| c.0 == RANDOM_GET_UNIFORM).collect();
        assert_eq!(draws.len(), 3);
        assert_eq!(draws[0].1[1..], [0.0f32.to_bits(), 10000.0f32.to_bits()]);
        assert_eq!(draws[0].1[0], log.first().unwrap().1[0]);
    }

    #[test]
    fn init_tables_allocation_size_saturates_when_it_overflows() {
        let (mut e, _, sizes) = tables_engine();
        let info = e.new_object::<SIdvLeafInfo>();
        e.set(info, SIdvLeafInfo::m_nNumLeafLodLevels, 1);
        // The doubled count is negative, so no quad is written, but the
        // shifted count times 4 overflows.
        e.call(0x00b3_cf40, &args![info, 0x4fff_ffffu32]);
        assert_eq!(*sizes.borrow(), vec![0, 0xffff_ffff, 4, 0]);
        // No rocking groups: no draws and an empty table.
        assert_eq!(e.get(info, SIdvLeafInfo::m_nNumTextures), 0x4fff_ffff);
    }

    /// A vector of `size` elements at fresh memory, with room for 8.
    fn texture_vector(e: &mut Engine, size: u32) -> (u32, u32) {
        let vector = e.mem.alloc(0x18);
        let elements = e.mem.alloc(0x54 * 8);
        e.mem.set_u32(vector, 0x7000);
        e.mem.set_u32(vector + 0x0c, elements);
        e.mem.set_u32(vector + 0x10, elements + 0x54 * size);
        e.mem.set_u32(vector + 0x14, elements + 0x54 * 8);
        (vector, elements)
    }

    fn marker_value() -> LeafTextureArg {
        let mut value = [0u32; 21];
        for (i, word) in value.iter_mut().enumerate() {
            *word = 0x100 + i as u32;
        }
        LeafTextureArg(value)
    }

    #[test]
    fn resize_grows_with_copies_of_the_value() {
        let mut e = leaf_engine();
        let (vector, elements) = texture_vector(&mut e, 1);
        let seen = Rc::new(RefCell::new(Vec::new()));
        let log = seen.clone();
        e.register_double(TEXTURE_VECTOR_INSERT, move |e, a| {
            let value: Vec<u32> = (0..21).map(|i| e.mem.u32(a[4] + 4 * i)).collect();
            log.borrow_mut().push((a.to_vec(), value));
            Ret::default()
        });
        let calls = recorded(
            &mut e,
            &[
                TEXTURE_VECTOR_ERASE,
                TEXTURE_ITERATOR_ADVANCE,
                INVALID_PARAMETER,
            ],
        );
        e.call(0x00b3_d910, &args![vector, 4u32, marker_value()]);
        let seen = seen.borrow();
        assert_eq!(seen.len(), 1);
        // this, proxy, position (the end), the three missing elements.
        assert_eq!(seen[0].0[..4], [vector, 0x7000, elements + 0x54, 3]);
        assert_eq!(seen[0].1, marker_value().0.to_vec());
        assert!(calls.borrow().is_empty());
    }

    #[test]
    fn resize_erases_from_the_new_size_on() {
        let mut e = leaf_engine();
        let (vector, elements) = texture_vector(&mut e, 3);
        let calls = recorded(
            &mut e,
            &[
                TEXTURE_VECTOR_ERASE,
                TEXTURE_VECTOR_INSERT,
                INVALID_PARAMETER,
            ],
        );
        e.register_double(TEXTURE_ITERATOR_ADVANCE, |e, a| {
            let position = e.mem.u32(a[0] + 4);
            e.mem.set_u32(a[0] + 4, position + a[1] * 0x54);
            Ret::default()
        });
        e.call(0x00b3_d910, &args![vector, 1u32, marker_value()]);
        let log = calls.borrow();
        assert_eq!(log.len(), 1);
        let expected = vec![
            vector,
            log[0].1[1],
            0x7000,
            elements + 0x54,
            0x7000,
            elements + 0x54 * 3,
        ];
        assert_eq!(log[0].1, expected);
    }

    #[test]
    fn resize_to_the_same_size_does_nothing() {
        let mut e = leaf_engine();
        let (vector, _) = texture_vector(&mut e, 2);
        let calls = recorded(
            &mut e,
            &[
                TEXTURE_VECTOR_ERASE,
                TEXTURE_VECTOR_INSERT,
                TEXTURE_ITERATOR_ADVANCE,
                INVALID_PARAMETER,
            ],
        );
        e.call(0x00b3_d910, &args![vector, 2u32, marker_value()]);
        assert!(calls.borrow().is_empty());
    }

    #[test]
    fn resize_checks_the_iterators_and_a_reversed_vector_erases() {
        let mut e = leaf_engine();
        let (vector, elements) = texture_vector(&mut e, 0);
        // The last pointer before the first: a negative size, which the
        // unsigned compare sees as huge, so the erase path runs and its
        // iterator check fails.
        e.mem.set_u32(vector + 0x10, elements - 0x54 * 2);
        let calls = recorded(
            &mut e,
            &[
                TEXTURE_VECTOR_ERASE,
                TEXTURE_VECTOR_INSERT,
                TEXTURE_ITERATOR_ADVANCE,
                INVALID_PARAMETER,
            ],
        );
        e.call(0x00b3_d910, &args![vector, 0u32, marker_value()]);
        let addresses: Vec<u32> = calls.borrow().iter().map(|c| c.0).collect();
        assert_eq!(
            addresses,
            [
                INVALID_PARAMETER,
                INVALID_PARAMETER,
                TEXTURE_ITERATOR_ADVANCE,
                TEXTURE_VECTOR_ERASE
            ]
        );
    }

    /// Scripts the file: `ints` for `ParseInt`, `floats` for `ParseFloat`,
    /// `flags` for the bool read, `vectors` for `ParseVector3`. The string,
    /// vector and exception callees are recorded.
    fn parse_engine(
        ints: Vec<u32>,
        floats: Vec<f32>,
        flags: Vec<u8>,
        vectors: Vec<[f32; 3]>,
    ) -> (Engine, Calls) {
        let mut e = leaf_engine();
        let calls = recorded(
            &mut e,
            &[
                STRING_FROM_C_STRING,
                PARSE_ERROR_FROM_STRING,
                STRING_COPY,
                STRING_ASSIGN,
                STRING_TIDY,
                TEXTURE_VECTOR_CLEAR,
                TEXTURE_VECTOR_INSERT,
                INVALID_PARAMETER,
            ],
        );
        let mut next = 0;
        e.register_double(PARSE_INT, move |_, _| {
            next += 1;
            Ret {
                eax: ints[next - 1],
                ..Ret::default()
            }
        });
        let mut next = 0;
        e.register_double(PARSE_FLOAT, move |_, _| {
            next += 1;
            Ret {
                st0: floats[next - 1] as f64,
                ..Ret::default()
            }
        });
        let mut next = 0;
        e.register_double(PARSE_BOOL, move |_, _| {
            next += 1;
            Ret {
                eax: flags[next - 1] as u32,
                ..Ret::default()
            }
        });
        let mut next = 0;
        e.register_double(PARSE_VECTOR3, move |e, a| {
            next += 1;
            for (i, v) in vectors[next - 1].iter().enumerate() {
                e.mem.set_f32(a[1] + 4 * i as u32, *v);
            }
            Ret {
                eax: a[1],
                ..Ret::default()
            }
        });
        let log = calls.clone();
        e.register_double(PARSE_STRING, move |_, a| {
            log.borrow_mut().push((PARSE_STRING, a.to_vec()));
            Ret {
                eax: a[1],
                ..Ret::default()
            }
        });
        let log = calls.clone();
        e.register_double(FILENAME_NO_PATH, move |_, a| {
            log.borrow_mut().push((FILENAME_NO_PATH, a.to_vec()));
            Ret {
                eax: a[1],
                ..Ret::default()
            }
        });
        (e, calls)
    }

    #[test]
    fn parse_reads_the_general_fields() {
        let ints = vec![
            0xbb8, 0xbb9, 7, 0xbba, 0xbbb, 0xbbc, 0xbbd, 0xbbe, 0xbbf, 0xbc0, 3, 0xbc1, 0xbc2,
            0x3ed,
        ];
        let floats = vec![1.5, 0.25, 9.0, 8.0, 0.75, 0.5];
        let (mut e, calls) = parse_engine(ints, floats, vec![0, 0, 1], vec![]);
        let info = e.new_object::<SIdvLeafInfo>();
        let file = Ptr::<()>::new(e.mem.alloc(0x20));
        e.call(0x00b3_d1b0, &args![info, file]);
        assert_eq!(e.get(info, SIdvLeafInfo::m_fBlossomDistance), 1.5);
        assert_eq!(e.get(info, SIdvLeafInfo::m_nBlossomLevel), 7);
        assert_eq!(e.get(info, SIdvLeafInfo::m_fBlossomWeighting), 0.25);
        assert_eq!(e.get(info, SIdvLeafInfo::m_fSpacingTolerance), 0.75);
        assert_eq!(e.get(info, SIdvLeafInfo::m_eCollisionType), 3);
        assert_eq!(e.get(info, SIdvLeafInfo::m_bDimming), 1);
        assert_eq!(e.get(info, SIdvLeafInfo::m_fDimmingScalar), 0.5);
        // Nothing but the file reads happened.
        assert!(calls.borrow().is_empty());
    }

    #[test]
    fn parse_reads_the_texture_block() {
        let ints = vec![
            0x3f1, 99, 2, // the block: a dropped value, two textures
            5, 0xfa0, 0xfa1, 0xfa2, 0xfa3, 0xfa4, 0xfa5, 0xfa6, 0xfa7, 0x3f0, // texture 0
            6, 0xfa3, 0x3f0, // texture 1
            0x3f2, 0x3ed, // the dropped end tag, the end
        ];
        let vectors = vec![
            [1.0, 2.0, 3.0],
            [4.0, 5.0, 6.0],
            [7.0, 8.0, 9.0],
            [10.0, 11.0, 12.0],
        ];
        let (mut e, calls) = parse_engine(ints, vec![0.125, 9.5], vec![1], vectors);
        // The insert double makes room for the two textures.
        let elements = e.mem.alloc(0x54 * 2);
        let info = e.new_object::<SIdvLeafInfo>();
        let vector = info.addr() + 0x10;
        e.mem.set_u32(vector + 0x0c, elements);
        e.mem.set_u32(vector + 0x10, elements);
        let insert_log = calls.clone();
        e.register_double(TEXTURE_VECTOR_INSERT, move |e, a| {
            insert_log
                .borrow_mut()
                .push((TEXTURE_VECTOR_INSERT, a.to_vec()));
            let last = e.mem.u32(a[0] + 0x10);
            e.mem.set_u32(a[0] + 0x10, last + a[3] * 0x54);
            Ret::default()
        });
        let copies = Rc::new(RefCell::new(Vec::new()));
        let log = copies.clone();
        e.register_double(TEXTURE_COPY, move |e, a| {
            let source: Vec<u32> = (0..21).map(|i| e.mem.u32(a[1] + 4 * i)).collect();
            log.borrow_mut().push((a[0], source));
            Ret::default()
        });
        let file = Ptr::<()>::new(e.mem.alloc(0x20));
        e.call(0x00b3_d1b0, &args![info, file]);

        let copies = copies.borrow();
        assert_eq!(copies.len(), 2);
        assert_eq!(copies[0].0, elements);
        assert_eq!(copies[1].0, elements + 0x54);
        let first = &copies[0].1;
        assert_eq!(first[0] & 0xff, 1);
        let as_f32 = |range: std::ops::Range<usize>| -> Vec<f32> {
            first[range].iter().map(|w| f32::from_bits(*w)).collect()
        };
        assert_eq!(as_f32(1..4), [1.0, 2.0, 3.0]);
        assert_eq!(as_f32(4..5), [0.125]);
        assert_eq!(as_f32(12..15), [4.0, 5.0, 6.0]);
        assert_eq!(as_f32(15..18), [7.0, 8.0, 9.0]);
        assert_eq!(as_f32(18..21), [10.0, 11.0, 12.0]);
        // The second texture keeps its defaults (the byte is clear).
        let second = &copies[1].1;
        assert_eq!(second[0] & 0xff, 0);
        assert_eq!(f32::from_bits(second[1]), 0.8);

        let log = calls.borrow();
        let addresses: Vec<u32> = log.iter().map(|c| c.0).collect();
        // The vector is cleared and resized with the default texture's
        // constructor (the filename `_Tidy`) first; then per texture: its
        // own default, and for a filename tag the parsed string copied,
        // assigned, reduced to the file name and assigned again, every
        // temporary destroyed; the local filename is destroyed after the
        // copy.
        let tidy = STRING_TIDY;
        let file_tag = [
            PARSE_STRING,
            STRING_COPY,
            tidy,
            STRING_ASSIGN,
            tidy,
            FILENAME_NO_PATH,
            STRING_ASSIGN,
            tidy,
        ];
        let mut expected = vec![TEXTURE_VECTOR_CLEAR, tidy, TEXTURE_VECTOR_INSERT, tidy];
        expected.extend(file_tag);
        expected.push(tidy);
        expected.push(tidy);
        expected.extend(file_tag);
        expected.push(tidy);
        assert_eq!(addresses, expected);
        // The vector is resized to the count (2 missing) with the default.
        let insert = &log[2];
        assert_eq!(insert.1[3], 2);
        // The filename assigned is the reduced name with `npos` as the count.
        let assign = log.iter().find(|c| c.0 == STRING_ASSIGN).unwrap();
        assert_eq!(assign.1[2..], [0, u32::MAX]);
    }

    #[test]
    fn parse_throws_on_an_unknown_general_tag() {
        for tag in [0, 0x3f0, 0xbb7, 0xbc3, 0xfa0] {
            let (mut e, calls) = parse_engine(vec![tag], vec![], vec![], vec![]);
            e.register(CXX_THROW_EXCEPTION, |_, _| panic!("thrown"));
            let info = e.new_object::<SIdvLeafInfo>();
            let file = Ptr::<()>::new(e.mem.alloc(0x20));
            let thrown = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                e.call(0x00b3_d1b0, &args![info, file]);
            }));
            assert!(thrown.is_err());
            let log = calls.borrow();
            let string = log.iter().find(|c| c.0 == STRING_FROM_C_STRING).unwrap();
            assert_eq!(string.1[1], MESSAGE_MALFORMED_GENERAL);
            let error = log.iter().find(|c| c.0 == PARSE_ERROR_FROM_STRING).unwrap();
            assert_eq!(error.1[1..], [string.1[0], 0]);
        }
    }

    #[test]
    fn parse_throws_on_an_unknown_texture_tag() {
        for tag in [0xf9f, 0xfa8, 0x3f1] {
            let ints = vec![0x3f1, 0, 1, 0, tag];
            let (mut e, calls) = parse_engine(ints, vec![], vec![], vec![]);
            e.register(CXX_THROW_EXCEPTION, |_, _| panic!("thrown"));
            let elements = e.mem.alloc(0x54);
            let info = e.new_object::<SIdvLeafInfo>();
            e.mem.set_u32(info.addr() + 0x1c, elements);
            e.mem.set_u32(info.addr() + 0x20, elements);
            e.register_double(TEXTURE_VECTOR_INSERT, |_, _| Ret::default());
            let file = Ptr::<()>::new(e.mem.alloc(0x20));
            let thrown = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                e.call(0x00b3_d1b0, &args![info, file]);
            }));
            assert!(thrown.is_err());
            let log = calls.borrow();
            let string = log.iter().find(|c| c.0 == STRING_FROM_C_STRING).unwrap();
            assert_eq!(string.1[1], MESSAGE_MALFORMED_SINGLE);
        }
    }

    #[test]
    fn parse_texture_index_check_calls_the_invalid_parameter_handler() {
        // The vector double does not grow the vector, so the bounds check
        // after the tags fails before the copy.
        let ints = vec![0x3f1, 0, 1, 0, 0xfa7, 0x3f0, 0x3f2, 0x3ed];
        let (mut e, calls) = parse_engine(ints, vec![1.0], vec![], vec![]);
        let elements = e.mem.alloc(0x54);
        let info = e.new_object::<SIdvLeafInfo>();
        e.mem.set_u32(info.addr() + 0x1c, elements);
        e.mem.set_u32(info.addr() + 0x20, elements);
        let copies = recorded(&mut e, &[TEXTURE_COPY]);
        let file = Ptr::<()>::new(e.mem.alloc(0x20));
        e.call(0x00b3_d1b0, &args![info, file]);
        assert!(calls.borrow().iter().any(|c| c.0 == INVALID_PARAMETER));
        // The handler returns in the harness, so the copy still goes ahead.
        assert_eq!(copies.borrow().len(), 1);
    }

    // ---- the exe's stack frames ----

    /// The calls made to `address` while the call log was on.
    fn logged(e: &Engine, address: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == address)
            .map(|c| c.1.clone())
            .collect()
    }

    /// The local the next `with_stack(size)` on the running thread takes (as the engine
    /// answers it), with the call log switched on.
    fn next_frame(e: &mut Engine, size: u32) -> u32 {
        let frame = e.next_stack_local(size);
        e.call_log = Some(vec![]);
        frame
    }

    #[test]
    fn init_tables_keeps_the_random_generator_at_frame_offset_0x2b() {
        let (mut e, _, _) = tables_engine();
        let info = e.new_object::<SIdvLeafInfo>();
        e.set(info, SIdvLeafInfo::m_nNumRockingGroups, 1);
        let frame = next_frame(&mut e, 0x40);
        e.call(0x00b3_cf40, &args![info, 1u32]);
        assert_eq!(logged(&e, RANDOM_CONSTRUCTOR), vec![vec![frame + 0x2b]]);
        assert_eq!(logged(&e, RANDOM_GET_UNIFORM)[0][0], frame + 0x2b);
        assert_eq!(logged(&e, RANDOM_DESTRUCTOR), vec![vec![frame + 0x2b]]);
    }

    #[test]
    fn resize_uses_its_argument_slot_and_the_exe_frame_offsets() {
        let mut e = leaf_engine();
        let (vector, _) = texture_vector(&mut e, 1);
        recorded(
            &mut e,
            &[
                TEXTURE_VECTOR_INSERT,
                TEXTURE_VECTOR_ERASE,
                TEXTURE_ITERATOR_ADVANCE,
            ],
        );
        let frame = next_frame(&mut e, 0x138);
        e.call(0x00b3_d910, &args![vector, 4u32, marker_value()]);
        // The copy inserted is the by-value argument at +0xe4.
        assert_eq!(logged(&e, TEXTURE_VECTOR_INSERT)[0][4], frame + 0xe4);
        assert_eq!(e.mem.u32(frame + 0xe4), 0x100);
        let (vector, _) = texture_vector(&mut e, 3);
        let frame = next_frame(&mut e, 0x138);
        e.call(0x00b3_d910, &args![vector, 1u32, marker_value()]);
        assert_eq!(logged(&e, TEXTURE_ITERATOR_ADVANCE)[0][0], frame + 0x74);
        assert_eq!(logged(&e, TEXTURE_VECTOR_ERASE)[0][1], frame + 0x88);
    }

    #[test]
    fn parse_keeps_every_local_at_the_exe_frame_offsets() {
        let ints = vec![
            0x3f1, 99, 1, // the block: a dropped value, one texture
            5, 0xfa1, 0xfa3, 0xfa4, 0xfa5, 0xfa6, 0x3f0, // the texture
            0x3f2, 0x3ed, // the dropped end tag, the end
        ];
        let vectors = vec![[1.0, 2.0, 3.0]; 4];
        let (mut e, _) = parse_engine(ints, vec![], vec![], vectors);
        let elements = e.mem.alloc(0x54);
        let info = e.new_object::<SIdvLeafInfo>();
        let vector = info.addr() + 0x10;
        e.mem.set_u32(vector + 0x0c, elements);
        e.mem.set_u32(vector + 0x10, elements);
        e.register_double(TEXTURE_VECTOR_INSERT, |e, a| {
            let last = e.mem.u32(a[0] + 0x10);
            e.mem.set_u32(a[0] + 0x10, last + a[3] * 0x54);
            Ret::default()
        });
        recorded(&mut e, &[TEXTURE_COPY]);
        let file = Ptr::<()>::new(e.mem.alloc(0x20));
        let raw = next_frame(&mut e, 0x58 + 0x384);
        let frame = raw + 0x58;
        e.call(0x00b3_d1b0, &args![info, file]);
        // The default texture in the outgoing argument area (its filename at +0x14).
        assert_eq!(logged(&e, STRING_TIDY)[0], vec![frame - 0x54 + 0x14, 0, 0]);
        let buffers: Vec<u32> = logged(&e, PARSE_VECTOR3).iter().map(|c| c[1]).collect();
        assert_eq!(
            buffers,
            vec![
                frame + parse_frame::COLOR,
                frame + parse_frame::ORIGIN,
                frame + parse_frame::SIZE,
                frame + parse_frame::USED_SIZE
            ]
        );
        assert_eq!(logged(&e, PARSE_STRING)[0][1], frame + parse_frame::PARSED);
        assert_eq!(
            logged(&e, STRING_COPY)[0][..2],
            [frame + parse_frame::COPY, frame + parse_frame::PARSED]
        );
        assert_eq!(
            logged(&e, FILENAME_NO_PATH)[0][1],
            frame + parse_frame::NAME
        );
        assert_eq!(logged(&e, TEXTURE_COPY)[0][1], frame + parse_frame::TEXTURE);
    }

    #[test]
    fn the_two_throws_use_their_own_frame_locals() {
        let (mut e, _) = parse_engine(vec![0], vec![], vec![], vec![]);
        e.register(CXX_THROW_EXCEPTION, |_, _| panic!("thrown"));
        let info = e.new_object::<SIdvLeafInfo>();
        let file = Ptr::<()>::new(e.mem.alloc(0x20));
        let frame = next_frame(&mut e, 0x58 + 0x384) + 0x58;
        let thrown = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            e.call(0x00b3_d1b0, &args![info, file]);
        }));
        assert!(thrown.is_err());
        assert_eq!(
            logged(&e, STRING_FROM_C_STRING)[0][0],
            frame + parse_frame::GENERAL_TEXT
        );
        assert_eq!(
            logged(&e, PARSE_ERROR_FROM_STRING)[0][..2],
            [
                frame + parse_frame::GENERAL_EXCEPTION,
                frame + parse_frame::GENERAL_TEXT
            ]
        );
        let (mut e, _) = parse_engine(vec![0x3f1, 0, 1, 0, 0xfa8], vec![], vec![], vec![]);
        e.register(CXX_THROW_EXCEPTION, |_, _| panic!("thrown"));
        let elements = e.mem.alloc(0x54);
        let info = e.new_object::<SIdvLeafInfo>();
        e.mem.set_u32(info.addr() + 0x1c, elements);
        e.mem.set_u32(info.addr() + 0x20, elements);
        e.register_double(TEXTURE_VECTOR_INSERT, |_, _| Ret::default());
        let file = Ptr::<()>::new(e.mem.alloc(0x20));
        let frame = next_frame(&mut e, 0x58 + 0x384) + 0x58;
        let thrown = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            e.call(0x00b3_d1b0, &args![info, file]);
        }));
        assert!(thrown.is_err());
        assert_eq!(
            logged(&e, STRING_FROM_C_STRING)[0][0],
            frame + parse_frame::TEXTURE_TEXT
        );
        assert_eq!(
            logged(&e, PARSE_ERROR_FROM_STRING)[0][0],
            frame + parse_frame::TEXTURE_EXCEPTION
        );
    }
}

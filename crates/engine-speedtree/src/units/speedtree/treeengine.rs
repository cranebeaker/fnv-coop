//! `SpeedTree/treeengine.cpp` (Xbox PDB source unit), subsystem `SpeedTree`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! SpeedTreeRT's tree engine: `CTreeEngine` reads a tree file (`Parse` and the parsers of its
//! sections), computes the branch tree (`Compute`: builds the root branch and the leaf lod
//! vectors, computes the branches and the branch and leaf reductions) and frees the transient
//! data again (`FreeTransientData`). The unit also holds the MSVC `std::vector` instantiations
//! it needs for its own element types (the leaf texture vector and the vector of 8-byte
//! elements) and the vector deleting destructor of the leaf lod array.
//!
//! The PC `std::vector` is 0x18 bytes (see `branch.rs`), so `CTreeEngine` is 0x128 bytes on the PC
//! and its fields after the first vector are 8 bytes (and, after the second, 0x10 bytes; after the
//! leaf info, 0x18 bytes) higher than in the Xbox PDB.
//!
//! x87 note: the exe computes in extended precision and stores `float` results; the
//! translations compute in `f64` through the shared x87 helpers and round at each `float` store.
//! Functions that carry an `FS:[0]` exception frame are marked `Translation incomplete` (the
//! unwinding handler is not translated; the throws are).

#[allow(unused_imports)]
use crate::prelude::*;

use super::branch::{
    ftol, vector_len, CVec3, StlVector, GET_UNIFORM, INVALID_PARAMETER, OPERATOR_DELETE,
    OPERATOR_NEW,
};
use super::leafinfo::SIdvLeafInfo;
use super::run_00b32410::{first_of, fn_00b355d0, last_of, span};

/// `CTreeFileAccess::ParseInt` (Xbox PDB, `thiscall`, `int` in `EAX`), `ParseFloat` (`float` in
/// `ST0`), `ParseString` (the string to fill) and the peek of the next tag (`00b3efe0`).
const PARSE_INT: u32 = 0x00b3_ed80;
const PARSE_FLOAT: u32 = 0x00b3_ed20;
const PARSE_STRING: u32 = 0x00b3_ee40;
const PEEK_INT: u32 = 0x00b3_efe0;
/// `00b06180`: reads a `bool` (one byte of the file's byte vector) from the tree file
/// (`thiscall`, `AL`).
const PARSE_BOOL: u32 = 0x00b0_6180;
/// `std::basic_string<char>::basic_string(const char*)` (`thiscall`), the string tidy
/// (`0044a730`: a flag and a size), the assign (`0044a580`: the source, a position and a count)
/// and the compare (`00ac9da0`, `int`).
const STRING_FROM_C_STRING: u32 = 0x0044_a4c0;
const STRING_TIDY: u32 = 0x0044_a730;
const STRING_ASSIGN: u32 = 0x0044_a580;
const STRING_COMPARE: u32 = 0x00ac_9da0;
/// `std::string::npos` as the exe reads it (`010173f0`).
const STRING_NPOS: u32 = 0x0101_73f0;
/// `CIdvFilename::NoPath` (`00b09f90`, `thiscall`: the string to fill; answers it).
const FILENAME_NO_PATH: u32 = 0x00b0_9f90;
/// The exception constructor the parsers throw with (`00b09480`: the text and a flag), the
/// throw information of the parse exceptions and `_CxxThrowException`.
const PARSE_ERROR_FROM_STRING: u32 = 0x00b0_9480;
const PARSE_THROW_INFO: u32 = 0x0117_0384;
const CXX_THROW_EXCEPTION: u32 = 0x00ec_7894;
/// The pointer to the text the file's header string is compared with (`011ad36c`).
const HEADER_TEXT_POINTER: u32 = 0x011a_d36c;
/// `00b32350`: `clear` of a 32-bit element vector; `00b15390`: its `push_back`.
const VECTOR_CLEAR: u32 = 0x00b3_2350;
const VECTOR_PUSH_BACK: u32 = 0x00b1_5390;
/// The 32-bit element vector helpers (`run_00b169c0`): the default constructor and the tidy.
const WORD_DEFAULT_CONSTRUCT: u32 = 0x00b1_7db0;
const WORD_TIDY: u32 = 0x00b1_7f50;
/// `00b33370`: the destructor of a 32-bit element vector (the array element destructor).
const WORD_DESTRUCTOR: u32 = 0x00b3_3370;
/// `00b36330`: the assignment of a 32-bit element vector (`this`, the source).
const WORD_ASSIGN: u32 = 0x00b3_6330;
/// `00b2e330`: advances an iterator of a 32-bit element vector (`thiscall`: the iterator, a
/// count) and `00b42980`: constructs one (`thiscall`: the iterator, the position, the vector).
const ADVANCE_ITERATOR: u32 = 0x00b2_e330;
const ITERATOR_CONSTRUCT: u32 = 0x00b4_2980;
/// `00b156b0`: inserts one 32-bit element before an iterator (`thiscall`: the vector, `out`, the
/// iterator's proxy and position, the address of the value).
const WORD_INSERT_ONE: u32 = 0x00b1_56b0;
/// `00b138c0`: a helper of the branch pointer vector (`cdecl`: the vector), `00b13820`: collects
/// the branches under a branch into a vector (`thiscall`), `00b135a0`: places a branch in a lod's
/// geometry (`thiscall`: the lod, the geometry).
const BRANCH_VECTOR_PREPARE: u32 = 0x00b1_38c0;
const BRANCH_COLLECT: u32 = 0x00b1_3820;
const BRANCH_PLACE: u32 = 0x00b1_35a0;
/// `CIdvBranch::CIdvBranch` (`00b10dc0`), `~CIdvBranch` (`00b10e90`) and `Compute` (`00b11050`).
const BRANCH_CONSTRUCT: u32 = 0x00b1_0dc0;
const BRANCH_DESTRUCT: u32 = 0x00b1_0e90;
const BRANCH_COMPUTE: u32 = 0x00b1_1050;
/// `CIndexedGeometry::SetNumLodLevels` (`00b2fb30`), `ResetStripCounter` (`00b306d0`), the
/// level preparation `00b30c80` (`thiscall`, one word) and the empty level fill `00b2fdc0`
/// (`thiscall`, three words).
const GEOMETRY_SET_NUM_LOD_LEVELS: u32 = 0x00b2_fb30;
const GEOMETRY_RESET_STRIP_COUNTER: u32 = 0x00b3_06d0;
const GEOMETRY_PREPARE_LEVEL: u32 = 0x00b3_0c80;
const GEOMETRY_FILL_EMPTY: u32 = 0x00b2_fdc0;
/// `CIdvRandom::CIdvRandom` (`00b3f100`), `Reseed` (`00b3f160`) and the destructor (`00aa7830`).
const RANDOM_CONSTRUCT: u32 = 0x00b3_f100;
const RANDOM_RESEED: u32 = 0x00b3_f160;
const RANDOM_DESTRUCT: u32 = 0x00aa_7830;
/// `CIdvCamera::~CIdvCamera` (`00b3f0a0`: stores its vtable) and `SIdvLeafInfo::~SIdvLeafInfo`
/// (`00b3ce30`).
const CAMERA_DESTRUCT: u32 = 0x00b3_f0a0;
const LEAF_INFO_DESTRUCT: u32 = 0x00b3_ce30;
/// `SIdvBranchInfo::~SIdvBranchInfo` (`00b3f620`), `SIdvBranchInfo::Parse` (`00b3f8e0`, `cdecl`),
/// `SIdvLeafInfo::Parse` (`00b3d1b0`), `SIdvWindInfo::Parse` (`00b408b0`) and
/// `CBillboardLeaf::Parse` (`00b405c0`, `cdecl`).
const BRANCH_INFO_DESTRUCT: u32 = 0x00b3_f620;
const BRANCH_INFO_PARSE: u32 = 0x00b3_f8e0;
const LEAF_INFO_PARSE: u32 = 0x00b3_d1b0;
const WIND_INFO_PARSE: u32 = 0x00b4_08b0;
const BILLBOARD_LEAF_PARSE: u32 = 0x00b4_05c0;
/// The leaf reducer of the lod builder: the constructor `00b40ad0` (`thiscall`: the leaf info and three floats) and the application `00b40b30` (`thiscall`: `out` and the vector to reduce passed by value, six words).
const LEAF_REDUCER_CONSTRUCT: u32 = 0x00b4_0ad0;
const LEAF_REDUCER_APPLY: u32 = 0x00b4_0b30;
/// `CVec3::Distance` (`00b04d40`, `thiscall`: the static camera position at `011f8e9c`, the
/// other position) and that static.
const VEC3_DISTANCE: u32 = 0x00b0_4d40;
const CAMERA_POSITION: u32 = 0x011f_8e9c;
/// The leaf texture vector helpers (`thiscall`): the clear `00b2df90`, the destruction of a range
/// `00b2e140`, the buy `00b2e080` (`bool`), and the `cdecl` range copies `00b2e500` and
/// `00b2e3f0` (assigning) and `00b2e570` (constructing).
const TEXTURES_CLEAR: u32 = 0x00b2_df90;
const TEXTURES_DESTROY_RANGE: u32 = 0x00b2_e140;
const TEXTURES_BUY: u32 = 0x00b2_e080;
const TEXTURES_ASSIGN_CHECKED: u32 = 0x00b2_e500;
const TEXTURES_ASSIGN_RANGE: u32 = 0x00b2_e3f0;
const TEXTURES_CONSTRUCT_RANGE: u32 = 0x00b2_e570;
/// `00b42e50`: copies `[first, last)` to `dest` and answers the end (`cdecl`), of 32-bit
/// elements.
const WORD_COPY: u32 = 0x00b4_2e50;
/// `00ec782f`: `__ehvec_ctor` (`array`, element size, count, constructor, destructor), `00ec5fce`:
/// `__ehvec_dtor` (`array`, element size, count, destructor), `srand` (`00ecada6`) and `_time64`
/// (`00ecadda`).
const EH_VECTOR_CONSTRUCT: u32 = 0x00ec_782f;
const EH_VECTOR_DESTRUCT: u32 = 0x00ec_5fce;
const SRAND: u32 = 0x00ec_ada6;
const TIME64: u32 = 0x00ec_adda;

/// The float constants the code loads (addresses in the exe's data): `-1.0f` (`01012054`), `0.0`
/// and `-1.0` as `double` (`01012060`, `0101a6b0`), `1.0` as `double` (`01012070`), the `float`
/// `10.0` (`01017b78`), the `double` the leaf lod scale is multiplied by (`01020758`), the
/// `float` seed range ends (`010162c0`, `0108cb00`), the `float` default of the leaf reduction
/// (`0101e2bc`).
const MINUS_ONE_FLOAT: u32 = 0x0101_2054;
const ZERO_DOUBLE: u32 = 0x0101_2060;
const ONE_DOUBLE: u32 = 0x0101_2070;
const MINUS_ONE_DOUBLE: u32 = 0x0101_a6b0;
const TEN_FLOAT: u32 = 0x0101_7b78;
const LOD_SCALE_DOUBLE: u32 = 0x0102_0758;
const SEED_LOW_FLOAT: u32 = 0x0101_62c0;
const SEED_HIGH_FLOAT: u32 = 0x0108_cb00;
const DEFAULT_LEAF_REDUCTION: u32 = 0x0101_e2bc;
/// The two wind constants `00b2bc70` multiplies by (`010a5664`, `010a5668`).
const WIND_SCALE_FIRST: u32 = 0x010a_5664;
const WIND_SCALE_SECOND: u32 = 0x010a_5668;

/// The statics the engine sets: `CIdvBranch`'s flag that lets the last level be expanded
/// (`011ad148`, a byte), the weight level marker (`011ad14c`), the pointer to the leaf info
/// (`011f8bf4`), the vector of branch infos the branches read (`011f8c50`) and the seed counter
/// (`011f8bfc`).
const EXPAND_LAST_FLAG: u32 = 0x011a_d148;
const LEVEL_MARKER: u32 = 0x011a_d14c;
const LEAF_INFO_POINTER: u32 = 0x011f_8bf4;
const BRANCH_INFO_LIST: u32 = 0x011f_8c50;
const BRANCH_COUNTER: u32 = 0x011f_8bfc;

/// The texts the parsers throw with.
const MESSAGE_MISSING_BEGIN: u32 = 0x010a_56b8;
const MESSAGE_NOT_SPT: u32 = 0x010a_5698;
const MESSAGE_MALFORMED_SPT: u32 = 0x010a_5678;
const MESSAGE_MALFORMED_TREE_INFO: u32 = 0x010a_56d4;
const MESSAGE_MALFORMED_BRANCH: u32 = 0x010a_54cc;
const MESSAGE_TOO_MANY_LODS: u32 = 0x010a_5710;
const MESSAGE_MALFORMED_LEAF_LOD: u32 = 0x010a_56f8;
const MESSAGE_MALFORMED_ENGINE_LOD: u32 = 0x010a_572c;
const MESSAGE_MALFORMED_TEXTURES: u32 = 0x010a_5748;
const MESSAGE_MALFORMED_FLARE: u32 = 0x010a_5764;

layout! {
    /// `CTreeEngine` (Xbox PDB), 0x128 bytes on the PC (`0x110` on the Xbox): a `CIdvCamera` (vtable
    /// and position) followed by the tree's data. The vectors are 0x18 bytes on the PC, so the
    /// offsets after `m_vBranchInfo` are higher than the PDB's.
    pub struct CTreeEngine: 0x128 {
        /// The vtable (`CIdvCamera` base).
        0x00 vtable: u32,
        /// `m_cPosition` (Xbox PDB `CIdvCamera`): three `float`s.
        0x04 m_cPosition: Inline<CVec3>,
        /// `m_fBillboardSize` (Xbox PDB).
        0x10 m_fBillboardSize: f32,
        /// `m_fLod` (Xbox PDB).
        0x14 m_fLod: f32,
        /// `m_fOverrideTreeSize` (Xbox PDB).
        0x18 m_fOverrideTreeSize: f32,
        /// `m_fOverrideTreeVariance` (Xbox PDB).
        0x1C m_fOverrideTreeVariance: f32,
        /// `m_cRandom` (Xbox PDB), a one-byte `CIdvRandom`.
        0x20 m_cRandom: u8,
        /// `m_bTransientDataIntact` (Xbox PDB).
        0x21 m_bTransientDataIntact: u8,
        /// `m_sTreeInfo.m_strBranchTextureFilename` (Xbox PDB `SIdvTreeInfo`, a 0x1C-byte string).
        0x24 m_strBranchTextureFilename: u32,
        /// `m_sTreeInfo.m_fFar` (Xbox PDB `SIdvTreeInfo +0x1C`).
        0x40 m_fFar: f32,
        /// `m_sTreeInfo.m_fNear` (Xbox PDB `SIdvTreeInfo +0x20`).
        0x44 m_fNear: f32,
        /// `m_sTreeInfo.m_nSeed` (Xbox PDB `SIdvTreeInfo +0x24`).
        0x48 m_nSeed: i32,
        /// `m_sTreeInfo.m_fSize` (Xbox PDB `SIdvTreeInfo +0x28`).
        0x4C m_fSize: f32,
        /// `m_sTreeInfo.m_fSizeVariance` (Xbox PDB `SIdvTreeInfo +0x2C`).
        0x50 m_fSizeVariance: f32,
        /// `m_sTreeInfo.m_nFlareSeed` (Xbox PDB `SIdvTreeInfo +0x30`).
        0x54 m_nFlareSeed: i32,
        /// `m_pBranch` (Xbox PDB).
        0x58 m_pBranch: u32,
        /// `m_pBranchGeometry` (Xbox PDB): the `CIndexedGeometry`.
        0x5C m_pBranchGeometry: u32,
        /// `m_vBranchInfo` (Xbox PDB `+0x60`): a `std::vector<SIdvBranchInfo *>`.
        0x60 m_vBranchInfo: Inline<StlVector>,
        /// `m_nNumBranchLodLevels` (Xbox PDB `+0x70`).
        0x78 m_nNumBranchLodLevels: u32,
        /// `m_vBillboardLeaves` (Xbox PDB `+0x74`): a `std::vector<CBillboardLeaf *>`.
        0x7C m_vBillboardLeaves: Inline<StlVector>,
        /// `m_sLeafInfo` (Xbox PDB `+0x84`).
        0x94 m_sLeafInfo: Inline<SIdvLeafInfo>,
        /// `m_vLeafLods` (Xbox PDB `+0xD4`): an array of `std::vector<CBillboardLeaf *>`, its
        /// element count in the word before it.
        0xEC m_vLeafLods: u32,
        /// `m_bParsedLeafLods` (Xbox PDB `+0xD8`).
        0xF0 m_bParsedLeafLods: u8,
        /// `m_fMinBranchVolumePercent` (Xbox PDB `+0xDC`).
        0xF4 m_fMinBranchVolumePercent: f32,
        /// `m_fMaxBranchVolumePercent` (Xbox PDB `+0xE0`).
        0xF8 m_fMaxBranchVolumePercent: f32,
        /// `m_fLeafReductionPercentage` (Xbox PDB `+0xE4`).
        0xFC m_fLeafReductionPercentage: f32,
        /// `m_fBranchReductionFuzziness` (Xbox PDB `+0xE8`).
        0x100 m_fBranchReductionFuzziness: f32,
        /// `m_fLargeBranchPercent` (Xbox PDB `+0xEC`).
        0x104 m_fLargeBranchPercent: f32,
        /// `m_nBranchesWeightLevel` (Xbox PDB `+0xF0`).
        0x108 m_nBranchesWeightLevel: i32,
        /// `m_sWindInfo` (Xbox PDB `+0xF4`): `SIdvWindInfo`, 0x1C bytes.
        0x10C m_sWindInfo: u32,
    }
}

/// Throws the parse exception for the text at `message`: the string, an exception built from it
/// and a `false` flag, `_CxxThrowException`. Does not return.
fn throw_parse_error(e: &mut Engine, message: u32) -> ! {
    e.with_stack(0x1c, |e, text| {
        e.call(STRING_FROM_C_STRING, &args![text, message]);
        e.with_stack(0x28, |e, exception| {
            e.call(PARSE_ERROR_FROM_STRING, &args![exception, text, 0u32]);
            e.call(CXX_THROW_EXCEPTION, &args![exception, PARSE_THROW_INFO]);
        });
    });
    unreachable!("__CxxThrowException does not return (00ec7894)")
}

/// `ParseInt` of the tree file.
fn parse_int(e: &mut Engine, file: Ptr) -> i32 {
    e.call(PARSE_INT, &args![file]).i32()
}

/// `ParseFloat` of the tree file (the value `ST0` holds, as the next `FSTP dword` stores it).
fn parse_float(e: &mut Engine, file: Ptr) -> f32 {
    e.call(PARSE_FLOAT, &args![file]).f32()
}

/// The count of 32-bit elements of the vector at `vector`: `(last - first) >> 2` (arithmetic
/// shift), compared unsigned.
fn word_count(e: &Engine, vector: u32) -> u32 {
    (last_of(e, vector).wrapping_sub(first_of(e, vector)) as i32 >> 2) as u32
}

/// The checked element access of a 32-bit element vector: an index not below the count takes the
/// invalid-parameter path. Answers the address of the element.
fn word_slot(e: &mut Engine, vector: u32, index: u32) -> u32 {
    if index >= word_count(e, vector) {
        e.call(INVALID_PARAMETER, &args![]);
    }
    first_of(e, vector).wrapping_add(index.wrapping_mul(4))
}

/// The size in bytes the array allocation asks `operator new` for: `count * 0x18 + 4`, all ones
/// when either step overflows 32 bits (`MUL` with `SETO`, then `ADD` with `SETC`).
fn array_bytes(count: u32) -> u32 {
    u32::try_from(u64::from(count) * 0x18)
        .unwrap_or(u32::MAX)
        .saturating_add(4)
}

/// Allocates the array of `count` empty 32-bit element vectors (0x18 bytes each, the count in
/// the word before the first one, constructed with `__ehvec_ctor`) and answers the address of
/// the first one, or zero when the allocation fails.
fn construct_vector_array(e: &mut Engine, count: u32) -> u32 {
    let block = e.call(OPERATOR_NEW, &args![array_bytes(count)]).u32();
    if block != 0 {
        e.mem.set_u32(block, count);
        e.call(
            EH_VECTOR_CONSTRUCT,
            &args![
                block.wrapping_add(4),
                0x18u32,
                count,
                WORD_DEFAULT_CONSTRUCT,
                WORD_DESTRUCTOR
            ],
        );
        block.wrapping_add(4)
    } else {
        0
    }
}

// Translated from 00b2ae60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CTreeEngine::_scalar_deleting_destructor_` (Xbox PDB): destroys the engine (`00b2ae90`) and,
/// when bit 0 of `flags` is set, frees it. Returns `this`.
pub fn c_tree_engine_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<CTreeEngine>,
    flags: u32,
) -> Ptr<CTreeEngine> {
    fn_00b2ae90(e, this);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 00b2ae90 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b2ae90 - the FS:[0] exception frame is not translated.
/// The destructor of `CTreeEngine` (the engine map gives it no name): destroys the leaf info
/// (`00b3ce30`), tidies the billboard leaf vector (`00b17f50`) and frees its proxy, does the same
/// for the branch info vector, tidies the branch texture file name string (`0044a730`), runs the
/// `CIdvRandom` destructor (`00aa7830`) and the `CIdvCamera` base destructor (`00b3f0a0`).
pub fn fn_00b2ae90(e: &mut Engine, this: Ptr<CTreeEngine>) {
    let t = this.addr();
    e.call(LEAF_INFO_DESTRUCT, &args![t + 0x94]);
    e.call(WORD_TIDY, &args![t + 0x7C]);
    let leaves_proxy = e.mem.u32(t + 0x7C);
    e.call(OPERATOR_DELETE, &args![leaves_proxy]);
    e.call(WORD_TIDY, &args![t + 0x60]);
    let infos_proxy = e.mem.u32(t + 0x60);
    e.call(OPERATOR_DELETE, &args![infos_proxy]);
    e.call(STRING_TIDY, &args![t + 0x24, 1u32, 0u32]);
    e.call(RANDOM_DESTRUCT, &args![t + 0x20]);
    e.call(CAMERA_DESTRUCT, &args![this]);
}

// Translated from 00b2af80 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b2af80 - the FS:[0] exception frame is not translated.
/// `CTreeEngine::Compute` (Xbox PDB; the one stack argument is the step the leaf lod scale grows
/// by): sets the expand-last-level flag (`011ad148`, set when the leaf lods are not parsed),
/// makes the root `CIdvBranch` (`00b10dc0`), unless the leaf lods were parsed makes the array of
/// leaf lod vectors (`m_vLeafLods`, one per leaf lod level), sets the weight level marker
/// (`011ad14c`), refills the static branch info list (`011f8c50`) from `m_vBranchInfo` and points
/// the static leaf info (`011f8bf4`) at `m_sLeafInfo`. Then it seeds the random generator
/// (`Reseed`), draws the tree size from `size - variance` to `size + variance`, seeds `srand`
/// with the flare seed and, for a positive size, scales the size and the used size of every
/// leaf texture (`+0x48`, `+0x4C` from `+0x3C`, `+0x40`). It reseeds again, sets the geometry's
/// lod level count and strip counter, stores the seed in the static counter (`011f8bfc`),
/// computes the branch tree (`CIdvBranch::Compute`, `00b11050`: the identity matrix, the
/// direction (0, 0, 1) and the billboard leaf vector), reduces the branches and leaves
/// (`00b2c130`) and, unless the leaf lods were parsed, builds the leaf lods (`00b2c890`).
/// Finally `srand(time(0))`.
pub fn c_tree_engine_compute(e: &mut Engine, this: Ptr<CTreeEngine>, lod_step: f32) {
    let t = this.addr();
    let expand = u8::from(e.mem.u8(t + 0xF0) == 0);
    e.mem.set_u8(EXPAND_LAST_FLAG, expand);
    let block = e.call(OPERATOR_NEW, &args![0x50u32]).u32();
    let branch = if block != 0 {
        e.call(BRANCH_CONSTRUCT, &args![block, 0u32]).u32()
    } else {
        0
    };
    e.mem.set_u32(t + 0x58, branch);
    if e.mem.u8(t + 0xF0) == 0 {
        let count = e.mem.u32(t + 0xD8);
        let array = construct_vector_array(e, count);
        e.mem.set_u32(t + 0xEC, array);
    }
    let level = e.mem.u32(t + 0x108);
    e.mem.set_u32(LEVEL_MARKER, level);
    e.call(VECTOR_CLEAR, &args![BRANCH_INFO_LIST]);
    e.mem.set_u32(LEAF_INFO_POINTER, t + 0x94);
    let mut index = 0u32;
    while index < word_count(e, t + 0x60) {
        let slot = word_slot(e, t + 0x60, index);
        let info = e.mem.u32(slot);
        e.with_stack(4, |e, copy| {
            e.mem.set_u32(copy.addr(), info);
            e.call(VECTOR_PUSH_BACK, &args![BRANCH_INFO_LIST, copy]);
        });
        index += 1;
    }
    let seed = e.mem.u32(t + 0x48);
    e.call(RANDOM_RESEED, &args![t + 0x20, seed]);
    let size_bits = e.mem.u32(t + 0x4C);
    let variance_bits = e.mem.u32(t + 0x50);
    let high = x87_store32(x87_add(x87_load32(size_bits), x87_m32(variance_bits)));
    let low = x87_store32(x87_sub(x87_load32(size_bits), x87_m32(variance_bits)));
    let drawn = e
        .call(
            GET_UNIFORM,
            &args![
                t + 0x20,
                f32::from_bits(x87_quiet_bits(low)),
                f32::from_bits(x87_quiet_bits(high))
            ],
        )
        .f64();
    let tree_size = x87_store32(drawn);
    let flare_seed = e.mem.u32(t + 0x54);
    e.call(SRAND, &args![flare_seed]);
    let size = f64::from(f32::from_bits(e.mem.u32(t + 0x4C)));
    if size > e.global::<f64>(ZERO_DOUBLE) {
        let mut texture = 0u32;
        while texture < vector_len(e, t + 0xA4, 0x54) {
            if texture >= vector_len(e, t + 0xA4, 0x54) {
                e.call(INVALID_PARAMETER, &args![]);
            }
            let element = texture
                .wrapping_mul(0x54)
                .wrapping_add(first_of(e, t + 0xA4));
            let size_bits = e.mem.u32(t + 0x4C);
            let scaled_x = x87_store32(x87_mul(
                x87_load32(size_bits),
                x87_m32(e.mem.u32(element + 0x3C)),
            ));
            e.mem.set_u32(element + 0x48, x87_quiet_bits(scaled_x));
            let scaled_y = x87_store32(x87_mul(
                x87_load32(size_bits),
                x87_m32(e.mem.u32(element + 0x40)),
            ));
            e.mem.set_u32(element + 0x4C, x87_quiet_bits(scaled_y));
            texture += 1;
        }
    }
    let seed = e.mem.u32(t + 0x48);
    e.call(RANDOM_RESEED, &args![t + 0x20, seed]);
    let geometry = e.mem.u32(t + 0x5C);
    let lod_levels = e.mem.u32(t + 0x78) & 0xffff;
    e.call(GEOMETRY_SET_NUM_LOD_LEVELS, &args![geometry, lod_levels]);
    let geometry = e.mem.u32(t + 0x5C);
    e.call(GEOMETRY_RESET_STRIP_COUNTER, &args![geometry, 0u32]);
    let seed = e.mem.u32(t + 0x48);
    e.mem.set_u32(BRANCH_COUNTER, seed);
    e.with_stack(0x24, |e, matrix| {
        e.with_stack(0xC, |e, direction| {
            let identity = [1.0f32, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0];
            for (i, value) in identity.iter().enumerate() {
                e.mem.set_f32(matrix.addr() + 4 * i as u32, *value);
            }
            e.mem.set_f32(direction.addr(), 0.0);
            e.mem.set_f32(direction.addr() + 4, 0.0);
            e.mem.set_f32(direction.addr() + 8, 1.0);
            let minus_one = x87_quiet(e.global::<f32>(MINUS_ONE_FLOAT));
            let seed = e.mem.u32(t + 0x48);
            let geometry = e.mem.u32(t + 0x5C);
            let branch = e.mem.u32(t + 0x58);
            e.call(
                BRANCH_COMPUTE,
                &args![
                    branch,
                    seed,
                    f32::from_bits(x87_quiet_bits(tree_size)),
                    0u32,
                    t + 4,
                    0.0f32,
                    0.0f32,
                    matrix,
                    direction,
                    geometry,
                    t + 0x7C,
                    1.0f32,
                    seed,
                    minus_one
                ],
            );
        });
    });
    fn_00b2c130(e, this);
    if e.mem.u8(t + 0xF0) == 0 {
        fn_00b2c890(e, this, x87_quiet(lod_step));
    }
    let now = e.call(TIME64, &args![0u32]).u32();
    e.call(SRAND, &args![now]);
}

// Translated from 00b2b490 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b2b490 - the FS:[0] exception frame is not translated.
/// `std::vector<CBillboardLeaf *>::_vector_deleting_destructor_` (Xbox PDB name; the body is
/// the vector deleting destructor of the 0x18-byte `std::vector` of 32-bit elements): with bit
/// 1 of `flags` set `this` is the first of an array whose count is the word before it, so each
/// element is destroyed (`__ehvec_dtor`, 0x18 bytes each, with `00b33370`) and, with bit 0
/// also set, the block (the array start, 4 bytes before) is freed; the answer is the array
/// start. Without bit 1 the one vector is tidied (`00b17f50`), its proxy freed and, with bit 0,
/// the vector itself. Returns the (adjusted) pointer.
pub fn std_vector_c_billboard_leaf_p_vector_deleting_destructor(
    e: &mut Engine,
    this: Ptr<StlVector>,
    flags: u32,
) -> Ptr<StlVector> {
    let v = this.addr();
    if flags & 2 != 0 {
        let count = e.mem.u32(v.wrapping_sub(4));
        e.call(
            EH_VECTOR_DESTRUCT,
            &args![this, 0x18u32, count, WORD_DESTRUCTOR],
        );
        if flags & 1 != 0 {
            e.call(OPERATOR_DELETE, &args![v.wrapping_sub(4)]);
        }
        Ptr::new(v.wrapping_sub(4))
    } else {
        e.call(WORD_TIDY, &args![this]);
        let proxy = e.mem.u32(v);
        e.call(OPERATOR_DELETE, &args![proxy]);
        if flags & 1 != 0 {
            e.call(OPERATOR_DELETE, &args![this]);
        }
        this
    }
}

// Translated from 00b2b550 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b2b550 - the FS:[0] exception frame and the /GS cookie check
// (00ec408c) are not translated; the throws are.
/// `CTreeEngine::Parse` (Xbox PDB): parses a tree file. The first tag must be `0x3E8` (else
/// `"missing begin_file token"` is thrown); the file's header string (`ParseString`) must equal
/// the text `011ad36c` points at (else `"not a valid SpeedTree SPT file"`). Then tags until
/// `0x3E9`: `0x3EA` the tree info (`ParseTreeInfo`, `00b2bde0`), `0x3EC` the leaf info
/// (`SIdvLeafInfo::Parse` on `m_sLeafInfo`), `0x3F3` the wind info (`SIdvWindInfo::Parse` on
/// `m_sWindInfo`); any other tag throws `"malformed SpeedTree SPT file"`. When the file has more
/// bytes and the next tag is `7000` (`0x1B58`) the leaf clusters are read (`ParseLeafCluster`).
/// The branch texture file name is then reassigned through a copy (`00b2b8d0`, `00b2b910`), the
/// leaf texture vector is assigned to itself (`00b2db10`) and, when the override tree size is
/// above -1, the size is set with `SetSize`. Returns true.
pub fn c_tree_engine_parse(e: &mut Engine, this: Ptr<CTreeEngine>, file: Ptr) -> bool {
    let t = this.addr();
    let f = file.addr();
    e.with_stack(0x1c, |e, header| {
        if parse_int(e, file) != 0x3e8 {
            throw_parse_error(e, MESSAGE_MISSING_BEGIN);
        }
        e.call(PARSE_STRING, &args![file, header]);
        let different = e.with_stack(0x1c, |e, expected| {
            let text = e.mem.u32(HEADER_TEXT_POINTER);
            e.call(STRING_FROM_C_STRING, &args![expected, text]);
            let result = e.call(STRING_COMPARE, &args![header, expected]).i32() != 0;
            e.call(STRING_TIDY, &args![expected, 1u32, 0u32]);
            result
        });
        if different {
            throw_parse_error(e, MESSAGE_NOT_SPT);
        }
        let mut tag = parse_int(e, file);
        loop {
            match tag {
                0x3ea => c_tree_engine_parse_tree_info(e, this, file),
                0x3ec => {
                    e.call(LEAF_INFO_PARSE, &args![t + 0x94, file]);
                }
                0x3f3 => {
                    e.call(WIND_INFO_PARSE, &args![t + 0x10C, file]);
                }
                _ => throw_parse_error(e, MESSAGE_MALFORMED_SPT),
            }
            tag = parse_int(e, file);
            if tag == 0x3e9 {
                break;
            }
        }
        let size = last_of(e, f + 4).wrapping_sub(first_of(e, f + 4));
        if e.mem.u32(f) < size && e.call(PEEK_INT, &args![file]).i32() == 0x1b58 {
            parse_int(e, file);
            c_tree_engine_parse_leaf_cluster(e, this, file);
        }
        e.with_stack(0x1c, |e, copy| {
            let copy = fn_00b2b8d0(e, Ptr::new(copy.addr()), Ptr::new(t + 0x24));
            fn_00b2b910(e, Ptr::new(t + 0x24), copy);
            e.call(STRING_TIDY, &args![copy, 1u32, 0u32]);
        });
        fn_00b2db10(e, Ptr::new(t + 0xA4), Ptr::new(t + 0xA4));
        let override_size = f64::from(f32::from_bits(e.mem.u32(t + 0x18)));
        if override_size > e.global::<f64>(MINUS_ONE_DOUBLE) {
            let size = x87_quiet(e.mem.f32(t + 0x18));
            let variance = x87_quiet(e.mem.f32(t + 0x1C));
            c_tree_engine_set_size(e, this, size, variance);
        }
        e.call(STRING_TIDY, &args![header, 1u32, 0u32]);
    });
    true
}

// Translated from 00b2b8d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Assigns the string `other` to the string `this` after emptying `this` (the engine map gives
/// it no name): tidies `this` (`0044a730`, flag 0, size 0) and assigns all of `other`
/// (`0044a580`: from position 0 to `npos`). Returns `this`.
pub fn fn_00b2b8d0(e: &mut Engine, this: Ptr, other: Ptr) -> Ptr {
    e.call(STRING_TIDY, &args![this, 0u32, 0u32]);
    let npos = e.mem.u32(STRING_NPOS);
    e.call(STRING_ASSIGN, &args![this, other, 0u32, npos]);
    this
}

// Translated from 00b2b910 (decompiled, FalloutNV.exe 1.4.0.525)
/// Assigns all of the string `other` to the string `this` (`0044a580`: from position 0 to
/// `npos`; the engine map gives it no name). Returns `this`.
pub fn fn_00b2b910(e: &mut Engine, this: Ptr, other: Ptr) -> Ptr {
    let npos = e.mem.u32(STRING_NPOS);
    e.call(STRING_ASSIGN, &args![this, other, 0u32, npos]);
    this
}

// Translated from 00b2b940 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CTreeEngine::FreeTransientData` (Xbox PDB): when the transient data is intact
/// (`m_bTransientDataIntact`), destroys and frees the root branch (`00b10e90`) and clears the
/// pointer; for each leaf lod level (when the leaf lod array exists) deletes every billboard
/// leaf through its virtual destructor (slot 0 with flag 1), nulls the entries and clears the
/// vector (`00b32350`); then deletes the leaf lod array (`00b2b490` with flags 3) and clears
/// its pointer, destroys and frees every branch info (`00b3f620`), nulls the entries, clears
/// `m_vBranchInfo` and clears the flag.
pub fn c_tree_engine_free_transient_data(e: &mut Engine, this: Ptr<CTreeEngine>) {
    let t = this.addr();
    if e.mem.u8(t + 0x21) == 0 {
        return;
    }
    let branch = e.mem.u32(t + 0x58);
    if branch != 0 {
        e.call(BRANCH_DESTRUCT, &args![branch]);
        e.call(OPERATOR_DELETE, &args![branch]);
    }
    e.mem.set_u32(t + 0x58, 0);
    let mut level = 0i32;
    while level < e.mem.i32(t + 0xD8) {
        if e.mem.u32(t + 0xEC) != 0 {
            let mut leaf_index = 0u32;
            loop {
                let list = (level as u32)
                    .wrapping_mul(0x18)
                    .wrapping_add(e.mem.u32(t + 0xEC));
                if leaf_index >= word_count(e, list) {
                    break;
                }
                let slot = word_slot(e, list, leaf_index);
                let leaf = e.mem.u32(slot);
                if leaf != 0 {
                    e.vcall(leaf, 0, &args![1u32]);
                }
                let list = (level as u32)
                    .wrapping_mul(0x18)
                    .wrapping_add(e.mem.u32(t + 0xEC));
                let slot = word_slot(e, list, leaf_index);
                e.mem.set_u32(slot, 0);
                leaf_index += 1;
            }
            let list = (level as u32)
                .wrapping_mul(0x18)
                .wrapping_add(e.mem.u32(t + 0xEC));
            e.call(VECTOR_CLEAR, &args![list]);
        }
        level += 1;
    }
    let array = e.mem.u32(t + 0xEC);
    if array != 0 {
        std_vector_c_billboard_leaf_p_vector_deleting_destructor(e, Ptr::new(array), 3);
    }
    e.mem.set_u32(t + 0xEC, 0);
    let mut info_index = 0u32;
    while info_index < word_count(e, t + 0x60) {
        let slot = word_slot(e, t + 0x60, info_index);
        let info = e.mem.u32(slot);
        if info != 0 {
            e.call(BRANCH_INFO_DESTRUCT, &args![info]);
            e.call(OPERATOR_DELETE, &args![info]);
        }
        let slot = word_slot(e, t + 0x60, info_index);
        e.mem.set_u32(slot, 0);
        info_index += 1;
    }
    e.call(VECTOR_CLEAR, &args![t + 0x60]);
    e.mem.set_u8(t + 0x21, 0);
}

// Translated from 00b2bc70 (decompiled, FalloutNV.exe 1.4.0.525)
/// A wind strength setter of `CTreeEngine` (the engine map gives it no name): stores the
/// strength in `m_sWindInfo.m_fStrength` (`+0x124`) and sets the leaf oscillation vector
/// (`+0x118`): `x = -(c1 * strength * leafFactors.x)`, `y = c1 * strength * leafFactors.x` (both
/// from the constant at `010a5668`) and `z = c0 * strength * leafFactors.y` (the constant at
/// `010a5664`); the sign flip is the x87 `FCHS` of the stored product.
pub fn fn_00b2bc70(e: &mut Engine, this: Ptr<CTreeEngine>, strength: f32) {
    let t = this.addr();
    let strength_bits = strength.to_bits();
    e.mem.set_u32(t + 0x124, x87_quiet_bits(strength_bits));
    let scale = x87_load32(e.mem.u32(WIND_SCALE_FIRST));
    let z = x87_store32(x87_mul(
        x87_mul(scale, x87_m32(strength_bits)),
        x87_m32(e.mem.u32(t + 0x110)),
    ));
    let scale = x87_load32(e.mem.u32(WIND_SCALE_SECOND));
    let y = x87_store32(x87_mul(
        x87_mul(scale, x87_m32(strength_bits)),
        x87_m32(e.mem.u32(t + 0x10C)),
    ));
    e.mem.set_u32(t + 0x118, x87_quiet_bits(y) ^ 0x8000_0000);
    e.mem.set_u32(t + 0x11C, x87_quiet_bits(y));
    e.mem.set_u32(t + 0x120, x87_quiet_bits(z));
}

// Translated from 00b2bcf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The getter matching `CTreeEngine::SetSize` (the engine map gives it no name): writes the
/// tree size (`+0x4C`) to `size` and the size variance (`+0x50`) to `variance`, copying the
/// floats through the x87 stack.
pub fn fn_00b2bcf0(e: &mut Engine, this: Ptr<CTreeEngine>, size: Ptr, variance: Ptr) {
    let t = this.addr();
    let size_bits = x87_quiet_bits(e.mem.u32(t + 0x4C));
    e.mem.set_u32(size.addr(), size_bits);
    let variance_bits = x87_quiet_bits(e.mem.u32(t + 0x50));
    e.mem.set_u32(variance.addr(), variance_bits);
}

// Translated from 00b2bd20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CTreeEngine::SetSize` (Xbox PDB): stores the tree size (`+0x4C`) and the size variance
/// (`+0x50`), copying the floats through the x87 stack.
pub fn c_tree_engine_set_size(e: &mut Engine, this: Ptr<CTreeEngine>, size: f32, variance: f32) {
    let t = this.addr();
    e.mem.set_u32(t + 0x4C, x87_quiet_bits(size.to_bits()));
    e.mem.set_u32(t + 0x50, x87_quiet_bits(variance.to_bits()));
}

// Translated from 00b2bd40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CTreeEngine::ComputeLod` (Xbox PDB): the distance (`CVec3::Distance`, `00b04d40`) from the
/// static camera position (`011f8e9c`) to the engine's position; `m_fLod` becomes `1 -
/// (distance - near) / (far - near)`, set to 1 when it is above 1 and to 0 when it is below 0
/// (NaN stays). Returns the lod in `ST0`.
pub fn c_tree_engine_compute_lod(e: &mut Engine, this: Ptr<CTreeEngine>) -> f32 {
    let t = this.addr();
    let distance = e.call(VEC3_DISTANCE, &args![CAMERA_POSITION, t + 4]).f64();
    let distance = x87_store32(distance);
    let numerator = x87_sub(x87_load32(distance), x87_m32(e.mem.u32(t + 0x44)));
    let denominator = x87_sub(
        x87_load32(e.mem.u32(t + 0x40)),
        x87_m32(e.mem.u32(t + 0x44)),
    );
    let ratio = x87_div(numerator, denominator);
    e.mem.set_u32(t + 0x14, x87_store32(x87_sub(1.0, ratio)));
    let lod = f64::from(f32::from_bits(x87_quiet_bits(e.mem.u32(t + 0x14))));
    if lod > e.global::<f64>(ONE_DOUBLE) {
        e.mem.set_f32(t + 0x14, 1.0);
    } else {
        let lod = f64::from(f32::from_bits(x87_quiet_bits(e.mem.u32(t + 0x14))));
        if lod < e.global::<f64>(ZERO_DOUBLE) {
            e.mem.set_f32(t + 0x14, 0.0);
        }
    }
    f32::from_bits(x87_quiet_bits(e.mem.u32(t + 0x14)))
}

// Translated from 00b2bdc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The setter of the lod distances of `CTreeEngine` (the engine map gives it no name): the first
/// argument becomes the near distance (`+0x44`), the second the far distance (`+0x40`), copied
/// through the x87 stack.
pub fn fn_00b2bdc0(e: &mut Engine, this: Ptr<CTreeEngine>, near: f32, far: f32) {
    let t = this.addr();
    e.mem.set_u32(t + 0x44, x87_quiet_bits(near.to_bits()));
    e.mem.set_u32(t + 0x40, x87_quiet_bits(far.to_bits()));
}

// Translated from 00b2bde0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b2bde0 - the FS:[0] exception frame is not translated; the throw
// is.
/// `CTreeEngine::ParseTreeInfo` (Xbox PDB): reads the tree info section tag by tag until
/// `0x3EB`. `2000` (`0x7D0`) the branch texture file name (a string, copied through two
/// temporaries, then reduced with `CIdvFilename::NoPath` `00b09f90` and assigned to
/// `m_strBranchTextureFilename`); `0x3F6` the branch infos (`00b2c790`); `0x7D1` a float into
/// the far distance (`+0x40`); `0x7D2` a skipped `bool`; `0x7D3` a float into the near distance
/// (`+0x44`); `0x7D4` a skipped int; `0x7D5` the seed (`SetSeed`); `0x7D6`, `0x7D7` floats into
/// the size (`+0x4C`) and its variance (`+0x50`); any other tag throws `"malformed general tree
/// information"`.
pub fn c_tree_engine_parse_tree_info(e: &mut Engine, this: Ptr<CTreeEngine>, file: Ptr) {
    let t = this.addr();
    let mut tag = parse_int(e, file);
    loop {
        if tag > 0x7d0 {
            match (tag - 0x7d1) as u32 {
                0 => {
                    let value = parse_float(e, file);
                    e.mem.set_f32(t + 0x40, value);
                }
                1 => {
                    e.call(PARSE_BOOL, &args![file]);
                }
                2 => {
                    let value = parse_float(e, file);
                    e.mem.set_f32(t + 0x44, value);
                }
                3 => {
                    parse_int(e, file);
                }
                4 => {
                    let seed = parse_int(e, file);
                    c_tree_engine_set_seed(e, this, seed as u32);
                }
                5 => {
                    let value = parse_float(e, file);
                    e.mem.set_f32(t + 0x4C, value);
                }
                6 => {
                    let value = parse_float(e, file);
                    e.mem.set_f32(t + 0x50, value);
                }
                _ => throw_parse_error(e, MESSAGE_MALFORMED_TREE_INFO),
            }
        } else if tag == 0x7d0 {
            e.with_stack(0x1c, |e, parsed| {
                e.call(PARSE_STRING, &args![file, parsed]);
                e.with_stack(0x1c, |e, copy| {
                    e.call(STRING_TIDY, &args![copy, 0u32, 0u32]);
                    let npos = e.mem.u32(STRING_NPOS);
                    e.call(STRING_ASSIGN, &args![copy, parsed, 0u32, npos]);
                    e.call(STRING_TIDY, &args![parsed, 1u32, 0u32]);
                    let npos = e.mem.u32(STRING_NPOS);
                    e.call(STRING_ASSIGN, &args![t + 0x24, copy, 0u32, npos]);
                    e.call(STRING_TIDY, &args![copy, 1u32, 0u32]);
                });
            });
            e.with_stack(0x1c, |e, trimmed| {
                let trimmed_name = e.call(FILENAME_NO_PATH, &args![t + 0x24, trimmed]).u32();
                let npos = e.mem.u32(STRING_NPOS);
                e.call(STRING_ASSIGN, &args![t + 0x24, trimmed_name, 0u32, npos]);
                e.call(STRING_TIDY, &args![trimmed, 1u32, 0u32]);
            });
        } else if tag == 0x3f6 {
            fn_00b2c790(e, this, file);
        } else {
            throw_parse_error(e, MESSAGE_MALFORMED_TREE_INFO);
        }
        tag = parse_int(e, file);
        if tag == 0x3eb {
            return;
        }
    }
}

// Translated from 00b2c0c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CTreeEngine::SetSeed` (Xbox PDB): a seed of 0 draws one (the generator reseeded with -1,
/// then a uniform float between the constants at `010162c0` and `0108cb00`, truncated with
/// `_ftol2`) into `m_nSeed` (`+0x48`); 1 leaves it unchanged; any other value is stored.
pub fn c_tree_engine_set_seed(e: &mut Engine, this: Ptr<CTreeEngine>, seed: u32) {
    let t = this.addr();
    if seed == 0 {
        e.call(RANDOM_RESEED, &args![t + 0x20, 0xffff_ffffu32]);
        let high = x87_quiet(e.global::<f32>(SEED_HIGH_FLOAT));
        let low = x87_quiet(e.global::<f32>(SEED_LOW_FLOAT));
        let drawn = e.call(GET_UNIFORM, &args![t + 0x20, low, high]).f64();
        let drawn = ftol(e, drawn);
        e.mem.set_i32(t + 0x48, drawn);
    } else if seed != 1 {
        e.mem.set_u32(t + 0x48, seed);
    }
}

// Translated from 00b2c130 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b2c130 - the FS:[0] exception frame is not translated.
/// The branch and leaf reduction of `CTreeEngine::Compute` (the engine map gives it no name).
/// Collects the branches under the root (`00b13820`, when the geometry holds at least one
/// triangle's worth of the 16-bit count `(last - first) / 3` of its `+0x88` vector) into a
/// vector, sums their volumes (`+0x30`) and finds the largest. Then every branch whose volume
/// is above `largest * (1 - m_fLargeBranchPercent)` is moved to a second vector (and erased
/// from the first), while each other branch gets the reduction volume
/// `max(0, (1 - r) * volume + r * largest)` (`+0x34`) for `r` a random draw below
/// `m_fBranchReductionFuzziness`. The moved branches are put back at the front of the first
/// vector. Finally for each branch lod level, the lod's share of the total volume
/// (`m_fMaxBranchVolumePercent` down to `m_fMinBranchVolumePercent`, 1 for a single level) selects
/// how many leading branches are placed in the level (`00b135a0`), after the geometry's strip
/// counter is reset (`00b306d0`); an empty first level is filled with the empty level fill
/// (`00b2fdc0`).
pub fn fn_00b2c130(e: &mut Engine, this: Ptr<CTreeEngine>) {
    let t = this.addr();
    e.with_stack(0x18, |e, branches| {
        e.with_stack(0x18, |e, moved| {
            e.with_stack(8, |e, random| {
                let a = branches.addr();
                let b = moved.addr();
                let r = random.addr();
                e.call(WORD_DEFAULT_CONSTRUCT, &args![branches]);
                if e.mem.u32(t + 0x58) != 0 {
                    let geometry = e.mem.u32(t + 0x5C);
                    let triangles = (last_of(e, geometry + 0x88)
                        .wrapping_sub(first_of(e, geometry + 0x88))
                        as i32
                        >> 2) as u32
                        / 3;
                    if (triangles as u16) != 0 {
                        let root = e.mem.u32(t + 0x58);
                        e.call(BRANCH_COLLECT, &args![root, branches]);
                    }
                }
                let mut total = 0u32;
                let mut largest = 0u32;
                let mut index = 0u32;
                while index < word_count(e, a) {
                    let slot = word_slot(e, a, index);
                    let branch = e.mem.u32(slot);
                    let volume = x87_quiet_bits(e.mem.u32(branch + 0x30));
                    total = x87_store32(x87_add(x87_load32(total), x87_m32(volume)));
                    if f64::from(f32::from_bits(largest)) < f64::from(f32::from_bits(volume)) {
                        largest = x87_quiet_bits(volume);
                    }
                    index += 1;
                }
                e.call(RANDOM_CONSTRUCT, &args![random]);
                let fuzz_complement = x87_store32(x87_sub(1.0, x87_load32(e.mem.u32(t + 0x104))));
                e.call(WORD_DEFAULT_CONSTRUCT, &args![moved]);
                let mut index = 0u32;
                while index < word_count(e, a) {
                    let slot = word_slot(e, a, index);
                    let branch = e.mem.u32(slot);
                    let volume = x87_quiet_bits(x87_quiet_bits(e.mem.u32(branch + 0x30)));
                    let limit = x87_mul(x87_load32(largest), x87_m32(fuzz_complement));
                    if f64::from(f32::from_bits(volume)) > limit {
                        let slot = word_slot(e, a, index);
                        e.call(VECTOR_PUSH_BACK, &args![moved, slot]);
                        let begin = first_of(e, a);
                        let (proxy, position) = e.with_stack(8, |e, iterator| {
                            e.call(ITERATOR_CONSTRUCT, &args![iterator, begin, branches]);
                            e.call(ADVANCE_ITERATOR, &args![iterator, index]);
                            (e.mem.u32(iterator.addr()), e.mem.u32(iterator.addr() + 4))
                        });
                        e.with_stack(8, |e, out| {
                            fn_00b2def0(e, branches.cast(), out, proxy, position);
                        });
                        index = index.wrapping_sub(1);
                    } else {
                        let fuzziness = x87_quiet(e.mem.f32(t + 0x100));
                        let drawn = e.call(GET_UNIFORM, &args![r, 0.0f32, fuzziness]).f64();
                        let u = x87_store32(drawn);
                        let w = x87_store32(x87_sub(1.0, x87_load32(u)));
                        let blended = x87_store32(x87_add(
                            x87_mul(x87_load32(w), x87_m32(volume)),
                            x87_mul(x87_load32(u), x87_m32(largest)),
                        ));
                        let blended_value = f64::from(f32::from_bits(blended));
                        let clamped = if 0.0 <= blended_value || blended_value.is_nan() {
                            blended
                        } else {
                            0.0f32.to_bits()
                        };
                        let clamped = x87_quiet_bits(clamped);
                        let slot = word_slot(e, a, index);
                        let branch = e.mem.u32(slot);
                        e.mem.set_u32(branch + 0x34, x87_quiet_bits(clamped));
                    }
                    index = index.wrapping_add(1);
                }
                e.call(BRANCH_VECTOR_PREPARE, &args![branches]);
                e.call(BRANCH_VECTOR_PREPARE, &args![moved]);
                let mut pending = 0u32;
                while pending < word_count(e, b) {
                    let begin = first_of(e, a);
                    let (proxy, position) = e.with_stack(8, |e, iterator| {
                        e.call(ITERATOR_CONSTRUCT, &args![iterator, begin, branches]);
                        (e.mem.u32(iterator.addr()), e.mem.u32(iterator.addr() + 4))
                    });
                    let slot = word_slot(e, b, pending);
                    e.with_stack(8, |e, out| {
                        e.call(
                            WORD_INSERT_ONE,
                            &args![branches, out, proxy, position, slot],
                        );
                    });
                    pending += 1;
                }
                let levels = e.mem.u32(t + 0x78);
                let mut level = 0u32;
                while level < levels {
                    if level == 0 {
                        let geometry = e.mem.u32(t + 0x5C);
                        e.call(GEOMETRY_PREPARE_LEVEL, &args![geometry, 0u32]);
                    }
                    let factor = if (levels as i32) < 2 {
                        1.0f32.to_bits()
                    } else {
                        let low = e.mem.u32(t + 0xF8);
                        let ratio = x87_store32(x87_div(
                            f64::from(level),
                            f64::from(levels.wrapping_sub(1) as i32),
                        ));
                        let span = x87_sub(x87_m32(e.mem.u32(t + 0xF4)), x87_load32(low));
                        x87_store32(x87_add(x87_mul(x87_load32(ratio), span), x87_load32(low)))
                    };
                    let share = x87_quiet_bits(factor);
                    let target = x87_store32(x87_mul(x87_load32(share), x87_m32(total)));
                    let mut count = 0i32;
                    let mut accumulated = 0u32;
                    let mut taken = 0u32;
                    while taken < word_count(e, a)
                        && f64::from(f32::from_bits(accumulated))
                            < f64::from(f32::from_bits(target))
                    {
                        let slot = word_slot(e, a, taken);
                        let branch = e.mem.u32(slot);
                        let volume = x87_quiet_bits(e.mem.u32(branch + 0x30));
                        accumulated =
                            x87_store32(x87_add(x87_load32(accumulated), x87_m32(volume)));
                        count += 1;
                        taken += 1;
                    }
                    let geometry = e.mem.u32(t + 0x5C);
                    e.call(
                        GEOMETRY_RESET_STRIP_COUNTER,
                        &args![geometry, level & 0xffff],
                    );
                    if level == 0 && count == 0 {
                        let geometry = e.mem.u32(t + 0x5C);
                        e.call(GEOMETRY_FILL_EMPTY, &args![geometry, 0u32, 0u32, 0u32]);
                    } else {
                        let mut placed = 0i32;
                        while placed < count {
                            let slot = word_slot(e, a, placed as u32);
                            let branch = e.mem.u32(slot);
                            let geometry = e.mem.u32(t + 0x5C);
                            e.call(BRANCH_PLACE, &args![branch, level, geometry]);
                            placed += 1;
                        }
                    }
                    level += 1;
                }
                e.call(WORD_TIDY, &args![moved]);
                let moved_proxy = e.mem.u32(b);
                e.call(OPERATOR_DELETE, &args![moved_proxy]);
                e.call(RANDOM_DESTRUCT, &args![random]);
                e.call(WORD_TIDY, &args![branches]);
                let branches_proxy = e.mem.u32(a);
                e.call(OPERATOR_DELETE, &args![branches_proxy]);
            });
        });
    });
}

// Translated from 00b2c790 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b2c790 - the FS:[0] exception frame is not translated; the throw
// is.
/// The branch info section of `CTreeEngine::ParseTreeInfo` (the engine map gives it no name):
/// clears `m_vBranchInfo` (`00b32350`), reads a count and that many branch infos
/// (`SIdvBranchInfo::Parse`, appended with `00b15390`), then the end tag `0x3F7` (else
/// `"malformed branch data"` is thrown).
pub fn fn_00b2c790(e: &mut Engine, this: Ptr<CTreeEngine>, file: Ptr) {
    let t = this.addr();
    e.call(VECTOR_CLEAR, &args![t + 0x60]);
    let count = parse_int(e, file);
    let mut index = 0i32;
    while index < count {
        let info = e.call(BRANCH_INFO_PARSE, &args![file]).u32();
        e.with_stack(4, |e, slot| {
            e.mem.set_u32(slot.addr(), info);
            e.call(VECTOR_PUSH_BACK, &args![t + 0x60, slot]);
        });
        index += 1;
    }
    if parse_int(e, file) != 0x3f7 {
        throw_parse_error(e, MESSAGE_MALFORMED_BRANCH);
    }
}

// Translated from 00b2c890 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b2c890 - the FS:[0] exception frame is not translated.
/// The leaf lod builder of `CTreeEngine::Compute` (the engine map gives it no name; the one
/// stack argument is the lod step): finds the largest leaf size (the larger of each leaf
/// texture's used size `+0x48`, `+0x4C`, times the leaf info's spacing tolerance `+0xBC`; 10
/// when none is found, the value stays -1). Then, when the leaf lod array exists, copies the
/// billboard leaves into its first vector (`00b36330`) and for each further lod level builds
/// a leaf reducer from the leaf info and `1 + level * step`, the leaf reduction percentage
/// (`+0xFC`) and the largest size times the constant at `01020758` (`00b40ad0`), applies it to
/// a copy of the previous level's vector (`00b40b30`, the copy passed by value and torn down
/// by the callee) and assigns the result to the level's vector.
pub fn fn_00b2c890(e: &mut Engine, this: Ptr<CTreeEngine>, lod_step: f32) {
    let t = this.addr();
    let mut largest = x87_quiet_bits(e.mem.u32(MINUS_ONE_FLOAT));
    let mut index = 0u32;
    while index < vector_len(e, t + 0xA4, 0x54) {
        if index >= vector_len(e, t + 0xA4, 0x54) {
            e.call(INVALID_PARAMETER, &args![]);
        }
        let element = index.wrapping_mul(0x54).wrapping_add(first_of(e, t + 0xA4));
        let width = e.mem.u32(element + 0x48);
        let height = e.mem.u32(element + 0x4C);
        let chosen = if f64::from(f32::from_bits(height)) < f64::from(f32::from_bits(width)) {
            width
        } else {
            height
        };
        let chosen = x87_quiet_bits(x87_quiet_bits(chosen));
        let product = x87_mul(x87_load32(chosen), x87_m32(e.mem.u32(t + 0xBC)));
        if f64::from(f32::from_bits(largest)) < product {
            largest = x87_store32(product);
        }
        index += 1;
    }
    if f64::from(f32::from_bits(largest)) == e.global::<f64>(MINUS_ONE_DOUBLE) {
        largest = x87_quiet_bits(e.mem.u32(TEN_FLOAT));
    }
    if e.mem.u32(t + 0xEC) != 0 {
        let lods = e.mem.u32(t + 0xEC);
        e.call(WORD_ASSIGN, &args![lods, t + 0x7C]);
        let mut level = 1i32;
        while level < e.mem.i32(t + 0xD8) {
            let step_bits = lod_step.to_bits();
            let level_scale =
                x87_store32(x87_add(x87_mul(f64::from(level), x87_m32(step_bits)), 1.0));
            let reduction = x87_quiet_bits(e.mem.u32(t + 0xFC));
            let scaled_size = x87_store32(x87_mul(
                x87_load32(largest),
                e.global::<f64>(LOD_SCALE_DOUBLE),
            ));
            // The leaf reducer is 0x28 bytes: 00b40ad0 runs its base constructor and stores the
            // floats and the word at +0x18 to +0x24 (EBP-0x4c in the exe).
            e.with_stack(0x28, |e, reducer| {
                e.call(
                    LEAF_REDUCER_CONSTRUCT,
                    &args![
                        reducer,
                        t + 0x94,
                        f32::from_bits(x87_quiet_bits(scaled_size)),
                        f32::from_bits(reduction),
                        f32::from_bits(x87_quiet_bits(level_scale))
                    ],
                );
                e.with_stack(0x18, |e, previous| {
                    let source = ((level - 1) as u32)
                        .wrapping_mul(0x18)
                        .wrapping_add(e.mem.u32(t + 0xEC));
                    fn_00b355d0(e, previous.cast(), Ptr::new(source));
                    e.with_stack(0x18, |e, result| {
                        let words = [
                            e.mem.u32(previous.addr()),
                            e.mem.u32(previous.addr() + 4),
                            e.mem.u32(previous.addr() + 8),
                            e.mem.u32(previous.addr() + 0xC),
                            e.mem.u32(previous.addr() + 0x10),
                            e.mem.u32(previous.addr() + 0x14),
                        ];
                        let produced = e
                            .call(
                                LEAF_REDUCER_APPLY,
                                &args![
                                    reducer, result, words[0], words[1], words[2], words[3],
                                    words[4], words[5]
                                ],
                            )
                            .u32();
                        let target = (level as u32)
                            .wrapping_mul(0x18)
                            .wrapping_add(e.mem.u32(t + 0xEC));
                        e.call(WORD_ASSIGN, &args![target, produced]);
                        e.call(WORD_TIDY, &args![result]);
                        let result_proxy = e.mem.u32(result.addr());
                        e.call(OPERATOR_DELETE, &args![result_proxy]);
                    });
                });
                fn_00b2e1b0(e, reducer.cast());
                let reducer_proxy = e.mem.u32(reducer.addr());
                e.call(OPERATOR_DELETE, &args![reducer_proxy]);
            });
            level += 1;
        }
    }
}

// Translated from 00b2cb50 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b2cb50 - the FS:[0] exception frame is not translated.
/// The destructor of a vector of 8-byte elements (the engine map gives it no name): tidies it
/// (`00b2e1b0`) and frees its proxy block.
pub fn fn_00b2cb50(e: &mut Engine, this: Ptr<StlVector>) {
    fn_00b2e1b0(e, this);
    let proxy = e.mem.u32(this.addr());
    e.call(OPERATOR_DELETE, &args![proxy]);
}

// Translated from 00b2cbc0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b2cbc0 - the FS:[0] exception frame is not translated; the throws
// are.
/// `CTreeEngine::ParseLeafCluster` (Xbox PDB): reads the number of leaf lod levels into
/// `m_sLeafInfo.m_nNumLeafLodLevels` (`+0xD8`), deletes the old leaf lod array (`00b2b490`, flags
/// 3) and makes a new one (an array of that many empty vectors). Then tags until `0x1B59`:
/// `0x1B5A` starts a level (more levels than announced throw `"too many leaf lod levels"`),
/// `0x1B5C` is a leaf (`CBillboardLeaf::Parse`, appended to the level's vector) until `0x1B5B`
/// ends the level; anything else throws `"malformed leaf lod data"`. The end sets
/// `m_bParsedLeafLods` (`+0xF0`).
pub fn c_tree_engine_parse_leaf_cluster(e: &mut Engine, this: Ptr<CTreeEngine>, file: Ptr) {
    let t = this.addr();
    let mut level = 0i32;
    let count = parse_int(e, file);
    e.mem.set_i32(t + 0xD8, count);
    let old = e.mem.u32(t + 0xEC);
    if old != 0 {
        std_vector_c_billboard_leaf_p_vector_deleting_destructor(e, Ptr::new(old), 3);
    }
    let count = e.mem.u32(t + 0xD8);
    let array = construct_vector_array(e, count);
    e.mem.set_u32(t + 0xEC, array);
    let mut tag = parse_int(e, file);
    loop {
        if tag == 0x1b59 {
            e.mem.set_u8(t + 0xF0, 1);
            return;
        }
        if level >= e.mem.i32(t + 0xD8) {
            throw_parse_error(e, MESSAGE_TOO_MANY_LODS);
        }
        if tag != 0x1b5a {
            throw_parse_error(e, MESSAGE_MALFORMED_LEAF_LOD);
        }
        tag = parse_int(e, file);
        while tag != 0x1b5b {
            if tag != 0x1b5c {
                throw_parse_error(e, MESSAGE_MALFORMED_LEAF_LOD);
            }
            let leaf = e.call(BILLBOARD_LEAF_PARSE, &args![file]).u32();
            let list = (level as u32)
                .wrapping_mul(0x18)
                .wrapping_add(e.mem.u32(t + 0xEC));
            e.with_stack(4, |e, slot| {
                e.mem.set_u32(slot.addr(), leaf);
                e.call(VECTOR_PUSH_BACK, &args![list, slot]);
            });
            tag = parse_int(e, file);
        }
        level += 1;
        tag = parse_int(e, file);
    }
}

// Translated from 00b2ceb0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b2ceb0 - the FS:[0] exception frame is not translated; the throw
// is.
/// `CTreeEngine::ParseLodInfo` (Xbox PDB; the engine map's stack argument is the tree file):
/// reads tags until `0x232E`: `0x232F` `m_nNumBranchLodLevels` (`+0x78`), `0x2330`
/// `m_fMinBranchVolumePercent` (`+0xF4`), `0x2332` `m_fLeafReductionPercentage` (`+0xFC`; a
/// value of zero becomes the default at `0101e2bc`), `0x2333` the leaf lod level count
/// (`+0xD8`), `0x2334` `m_fMaxBranchVolumePercent` (`+0xF8`), `0x2335`
/// `m_fBranchReductionFuzziness` (`+0x100`), `0x2336` `m_fLargeBranchPercent` (`+0x104`); any
/// other tag (`0x2331` included) throws `"malformed engine lod data"`.
pub fn c_tree_engine_parse_lod_info(e: &mut Engine, this: Ptr<CTreeEngine>, file: Ptr) {
    let t = this.addr();
    let mut tag = parse_int(e, file);
    loop {
        match tag.wrapping_sub(0x232f) as u32 {
            0 => {
                let value = parse_int(e, file);
                e.mem.set_i32(t + 0x78, value);
            }
            1 => {
                let value = parse_float(e, file);
                e.mem.set_f32(t + 0xF4, value);
            }
            3 => {
                let value = parse_float(e, file);
                e.mem.set_f32(t + 0xFC, value);
                if f64::from(e.mem.f32(t + 0xFC)) == e.global::<f64>(ZERO_DOUBLE) {
                    let default = x87_quiet_bits(e.mem.u32(DEFAULT_LEAF_REDUCTION));
                    e.mem.set_u32(t + 0xFC, default);
                }
            }
            4 => {
                let value = parse_int(e, file);
                e.mem.set_i32(t + 0xD8, value);
            }
            5 => {
                let value = parse_float(e, file);
                e.mem.set_f32(t + 0xF8, value);
            }
            6 => {
                let value = parse_float(e, file);
                e.mem.set_f32(t + 0x100, value);
            }
            7 => {
                let value = parse_float(e, file);
                e.mem.set_f32(t + 0x104, value);
            }
            _ => throw_parse_error(e, MESSAGE_MALFORMED_ENGINE_LOD),
        }
        tag = parse_int(e, file);
        if tag == 0x232e {
            return;
        }
    }
}

// Translated from 00b2d090 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b2d090 - the FS:[0] exception frame is not translated; the throws
// are.
/// `CTreeEngine::ParseTextureControls` (Xbox PDB; the stack argument is the tree file): for each
/// branch info the tag `0x3A9A`, a `bool` into `m_bRandomTCoordOffset` (`+0x1E`), the tag
/// `0x3A9B` and a float into `m_fTwist` (`+0x20`); then the end tag `0x3A99`. Any other tag
/// throws `"malformed texture controls"`.
pub fn c_tree_engine_parse_texture_controls(e: &mut Engine, this: Ptr<CTreeEngine>, file: Ptr) {
    let t = this.addr();
    let mut index = 0u32;
    loop {
        if index >= word_count(e, t + 0x60) {
            if parse_int(e, file) != 0x3a99 {
                throw_parse_error(e, MESSAGE_MALFORMED_TEXTURES);
            }
            return;
        }
        if parse_int(e, file) != 0x3a9a {
            throw_parse_error(e, MESSAGE_MALFORMED_TEXTURES);
        }
        let slot = word_slot(e, t + 0x60, index);
        let flag = e.call(PARSE_BOOL, &args![file]).u8();
        let info = e.mem.u32(slot);
        e.mem.set_u8(info + 0x1E, flag);
        if parse_int(e, file) != 0x3a9b {
            throw_parse_error(e, MESSAGE_MALFORMED_TEXTURES);
        }
        let slot = word_slot(e, t + 0x60, index);
        let value = parse_float(e, file);
        let info = e.mem.u32(slot);
        e.mem.set_f32(info + 0x20, value);
        index += 1;
    }
}

// Translated from 00b2d2e0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b2d2e0 - the FS:[0] exception frame is not translated; the throws
// are.
/// `CTreeEngine::ParseFlareInfo` (Xbox PDB; the stack argument is the tree file): for each
/// branch info the tags `0x3E82` (a float into `+0x24`, `m_fSegmentPackingExponent`), `0x3E83`
/// (an int into `+0x28`, `m_nNumFlares`), `0x3E84` (`+0x2C`), `0x3E85` (`+0x30`), `0x3E86`
/// (`+0x34`), `0x3E87` (`+0x38`), `0x3E88` (`+0x3C`), `0x3E89` (`+0x40`), `0x3E8A` (`+0x44`),
/// `0x3E8B` (`+0x48`) and `0x3E8C` (`+0x4C`), each followed by a float (the first seven stored
/// through the element slot taken before the read, the last four through the checked access
/// `00b31230` taken after it); then the end tag `0x3E81`. Any other tag throws `"malformed
/// flare info"`.
pub fn c_tree_engine_parse_flare_info(e: &mut Engine, this: Ptr<CTreeEngine>, file: Ptr) {
    let t = this.addr();
    let mut index = 0u32;
    loop {
        if index >= word_count(e, t + 0x60) {
            if parse_int(e, file) != 0x3e81 {
                throw_parse_error(e, MESSAGE_MALFORMED_FLARE);
            }
            return;
        }
        if parse_int(e, file) != 0x3e82 {
            throw_parse_error(e, MESSAGE_MALFORMED_FLARE);
        }
        let slot = word_slot(e, t + 0x60, index);
        let value = parse_float(e, file);
        let info = e.mem.u32(slot);
        e.mem.set_f32(info + 0x24, value);
        if parse_int(e, file) != 0x3e83 {
            throw_parse_error(e, MESSAGE_MALFORMED_FLARE);
        }
        let slot = word_slot(e, t + 0x60, index);
        let count = parse_int(e, file);
        let info = e.mem.u32(slot);
        e.mem.set_i32(info + 0x28, count);
        // The flare floats with their tags: the first five through a slot taken before the read.
        if parse_int(e, file) != 0x3e84 {
            throw_parse_error(e, MESSAGE_MALFORMED_FLARE);
        }
        for (field, next_tag) in [
            (0x2cu32, 0x3e85),
            (0x30, 0x3e86),
            (0x34, 0x3e87),
            (0x38, 0x3e88),
            (0x3c, 0x3e89),
        ] {
            let slot = word_slot(e, t + 0x60, index);
            let value = parse_float(e, file);
            let info = e.mem.u32(slot);
            e.mem.set_f32(info + field, value);
            if parse_int(e, file) != next_tag {
                throw_parse_error(e, MESSAGE_MALFORMED_FLARE);
            }
        }
        for (field, next_tag) in [(0x40u32, 0x3e8a), (0x44, 0x3e8b), (0x48, 0x3e8c)] {
            let value = parse_float(e, file);
            let stored = x87_store32(f64::from(value));
            let slot = word_slot(e, t + 0x60, index);
            let info = e.mem.u32(slot);
            e.mem.set_u32(info + field, x87_quiet_bits(stored));
            if parse_int(e, file) != next_tag {
                throw_parse_error(e, MESSAGE_MALFORMED_FLARE);
            }
        }
        let value = parse_float(e, file);
        let stored = x87_store32(f64::from(value));
        let slot = word_slot(e, t + 0x60, index);
        let info = e.mem.u32(slot);
        e.mem.set_u32(info + 0x4C, x87_quiet_bits(stored));
        index += 1;
    }
}

// Translated from 00b2db10 (decompiled, FalloutNV.exe 1.4.0.525)
/// The assignment of the leaf texture vector (`std::vector<SIdvLeafTexture>::operator=`; the
/// elements are 0x54 bytes; the engine map gives it no name): nothing for itself; an empty
/// source clears (`00b2df90`); a source with more elements than the destination either fits the
/// capacity (the first part is assigned over the existing elements, `00b2e3f0`, the rest
/// constructed after them, `00b2e570`) or needs new storage (the old elements destroyed,
/// `00b2e140`, and freed, the buy `00b2e080` and the construction `00b2e570`); a source no
/// larger is assigned over the start (`00b2e500`), the surplus destroyed (`00b2e140`) and the
/// end set. Returns `this`.
pub fn fn_00b2db10(e: &mut Engine, this: Ptr<StlVector>, other: Ptr<StlVector>) -> Ptr<StlVector> {
    let v = this.addr();
    let o = other.addr();
    if v == o {
        return this;
    }
    let other_count = vector_len(e, o, 0x54);
    if other_count == 0 {
        e.call(TEXTURES_CLEAR, &args![this]);
        return this;
    }
    if vector_len(e, v, 0x54) < other_count {
        let capacity = if first_of(e, v) == 0 {
            0
        } else {
            span(e, v, 0x14, 0x54)
        };
        if other_count <= capacity {
            let middle = vector_len(e, v, 0x54)
                .wrapping_mul(0x54)
                .wrapping_add(first_of(e, o));
            let (other_first, dest) = (first_of(e, o), first_of(e, v));
            e.call(TEXTURES_ASSIGN_RANGE, &args![other_first, middle, dest]);
            let (other_last, last) = (last_of(e, o), last_of(e, v));
            let end = e
                .call(
                    TEXTURES_CONSTRUCT_RANGE,
                    &args![middle, other_last, last, v + 8, 0u32, 0u32],
                )
                .u32();
            e.mem.set_u32(v + 0x10, end);
        } else {
            if first_of(e, v) != 0 {
                let (first, last) = (first_of(e, v), last_of(e, v));
                e.call(TEXTURES_DESTROY_RANGE, &args![this, first, last]);
                let block = first_of(e, v);
                e.call(OPERATOR_DELETE, &args![block]);
            }
            let count = vector_len(e, o, 0x54);
            if e.call(TEXTURES_BUY, &args![this, count]).bool() {
                let (other_first, other_last) = (first_of(e, o), last_of(e, o));
                let dest = first_of(e, v);
                let end = e
                    .call(
                        TEXTURES_CONSTRUCT_RANGE,
                        &args![other_first, other_last, dest, v + 8, 0u32, 0u32],
                    )
                    .u32();
                e.mem.set_u32(v + 0x10, end);
            }
        }
    } else {
        let (other_first, other_last, dest) = (first_of(e, o), last_of(e, o), first_of(e, v));
        let end = e
            .call(
                TEXTURES_ASSIGN_CHECKED,
                &args![other_first, other_last, dest, 0u32, 0u32, 0u32],
            )
            .u32();
        let last = last_of(e, v);
        e.call(TEXTURES_DESTROY_RANGE, &args![this, end, last]);
        let new_last = vector_len(e, o, 0x54)
            .wrapping_mul(0x54)
            .wrapping_add(first_of(e, v));
        e.mem.set_u32(v + 0x10, new_last);
    }
    this
}

// Translated from 00b2def0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The checked erase of the one 32-bit element at the iterator position `position` (the engine
/// map gives it no name; the iterator's proxy word, at word 2, is not read): moves the later
/// elements down (`00b42e50`: from `position + 4` to the end, to `position`), shortens the
/// vector by one element and writes the iterator at the same position to `out` (a position
/// outside the shortened vector takes the invalid-parameter path). Returns `out`.
pub fn fn_00b2def0(
    e: &mut Engine,
    this: Ptr<StlVector>,
    out: Ptr,
    _unused_2: u32,
    position: u32,
) -> Ptr {
    let v = this.addr();
    let end = last_of(e, v);
    e.call(WORD_COPY, &args![position.wrapping_add(4), end, position]);
    let end = last_of(e, v);
    e.mem.set_u32(v + 0x10, end.wrapping_sub(4));
    e.mem.set_u32(out.addr(), 0);
    if v == 0 || first_of(e, v) > position || position > last_of(e, v) {
        e.call(INVALID_PARAMETER, &args![]);
    }
    let proxy = e.mem.u32(v);
    e.mem.set_u32(out.addr(), proxy);
    e.mem.set_u32(out.addr() + 4, position);
    out
}

// Translated from 00b2e1b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The tidy of a vector of 8-byte elements (the engine map gives it no name): with storage,
/// walks the elements (nothing to destroy) and frees the block, then zeroes the first, last and
/// end pointers. (The element count the exe also computes is not used.)
pub fn fn_00b2e1b0(e: &mut Engine, this: Ptr<StlVector>) {
    let v = this.addr();
    if first_of(e, v) != 0 {
        let mut cursor = first_of(e, v);
        while cursor != last_of(e, v) {
            cursor = cursor.wrapping_add(8);
        }
        let block = first_of(e, v);
        e.call(OPERATOR_DELETE, &args![block]);
    }
    e.mem.set_u32(v + 0x0C, 0);
    e.mem.set_u32(v + 0x10, 0);
    e.mem.set_u32(v + 0x14, 0);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00b2ae60,
            c_tree_engine_scalar_deleting_destructor(Ptr<CTreeEngine>, u32) -> Ptr<CTreeEngine>
        ),
        entry!(0x00b2ae90, fn_00b2ae90(Ptr<CTreeEngine>)),
        entry!(0x00b2af80, c_tree_engine_compute(Ptr<CTreeEngine>, f32)),
        entry!(
            0x00b2b490,
            std_vector_c_billboard_leaf_p_vector_deleting_destructor(
                Ptr<StlVector>,
                u32,
            ) -> Ptr<StlVector>
        ),
        entry!(
            0x00b2b550,
            c_tree_engine_parse(Ptr<CTreeEngine>, Ptr) -> bool
        ),
        entry!(0x00b2b8d0, fn_00b2b8d0(Ptr, Ptr) -> Ptr),
        entry!(0x00b2b910, fn_00b2b910(Ptr, Ptr) -> Ptr),
        entry!(
            0x00b2b940,
            c_tree_engine_free_transient_data(Ptr<CTreeEngine>)
        ),
        entry!(0x00b2bc70, fn_00b2bc70(Ptr<CTreeEngine>, f32)),
        entry!(0x00b2bcf0, fn_00b2bcf0(Ptr<CTreeEngine>, Ptr, Ptr)),
        entry!(
            0x00b2bd20,
            c_tree_engine_set_size(Ptr<CTreeEngine>, f32, f32)
        ),
        entry!(
            0x00b2bd40,
            c_tree_engine_compute_lod(Ptr<CTreeEngine>) -> f32
        ),
        entry!(0x00b2bdc0, fn_00b2bdc0(Ptr<CTreeEngine>, f32, f32)),
        entry!(
            0x00b2bde0,
            c_tree_engine_parse_tree_info(Ptr<CTreeEngine>, Ptr)
        ),
        entry!(0x00b2c0c0, c_tree_engine_set_seed(Ptr<CTreeEngine>, u32)),
        entry!(0x00b2c130, fn_00b2c130(Ptr<CTreeEngine>)),
        entry!(0x00b2c790, fn_00b2c790(Ptr<CTreeEngine>, Ptr)),
        entry!(0x00b2c890, fn_00b2c890(Ptr<CTreeEngine>, f32)),
        entry!(0x00b2cb50, fn_00b2cb50(Ptr<StlVector>)),
        entry!(
            0x00b2cbc0,
            c_tree_engine_parse_leaf_cluster(Ptr<CTreeEngine>, Ptr)
        ),
        entry!(
            0x00b2ceb0,
            c_tree_engine_parse_lod_info(Ptr<CTreeEngine>, Ptr)
        ),
        entry!(
            0x00b2d090,
            c_tree_engine_parse_texture_controls(Ptr<CTreeEngine>, Ptr)
        ),
        entry!(
            0x00b2d2e0,
            c_tree_engine_parse_flare_info(Ptr<CTreeEngine>, Ptr)
        ),
        entry!(
            0x00b2db10,
            fn_00b2db10(Ptr<StlVector>, Ptr<StlVector>) -> Ptr<StlVector>
        ),
        entry!(
            0x00b2def0,
            fn_00b2def0(Ptr<StlVector>, Ptr, u32, u32) -> Ptr
        ),
        entry!(0x00b2e1b0, fn_00b2e1b0(Ptr<StlVector>)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::collections::VecDeque;
    use std::rc::Rc;

    /// What the doubled tree file answers (`ParseInt`, `ParseFloat`, `ParseBool`, the peek and the
    /// header string), in the order the code reads it.
    struct Tags {
        ints: VecDeque<i32>,
        floats: VecDeque<f32>,
        bytes: VecDeque<u8>,
        peek: i32,
        header: u32,
    }

    type Script = Rc<RefCell<Tags>>;

    /// The word the header text pointer points at, and a word that differs from it.
    const HEADER_EQUAL: u32 = 0x5555;
    const HEADER_OTHER: u32 = 0x6666;
    /// The destructor the doubled billboard leaves' vtable points at.
    const DESTRUCTOR: u32 = 0x00c0_0000;
    /// The proxy constructor and the checked word copy the copy constructor of 32-bit vectors calls.
    const PROXY_CONSTRUCT_FOR_COPY: u32 = 0x00b2_0ba0;
    const WORD_UNINIT_COPY_FOR_COPY: u32 = 0x00b2_4960;

    fn ret(value: u32) -> Ret {
        Ret {
            eax: value,
            ..Ret::default()
        }
    }

    fn ret_float(value: f64) -> Ret {
        Ret {
            st0: value,
            ..Ret::default()
        }
    }

    /// Makes room for one more element in the 32-bit element vector at `vector`.
    fn make_room(e: &mut Engine, vector: u32) {
        let (first, last, end) = (
            first_of(e, vector),
            last_of(e, vector),
            e.mem.u32(vector + 0x14),
        );
        if end.wrapping_sub(last) >= 4 && first != 0 {
            return;
        }
        let used = last.wrapping_sub(first);
        let storage = e.mem.alloc(used + 64);
        if used > 0 {
            let bytes = e.mem.bytes(first, used);
            e.mem.write(storage, &bytes);
        }
        e.mem.set_u32(vector + 0x0C, storage);
        e.mem.set_u32(vector + 0x10, storage + used);
        e.mem.set_u32(vector + 0x14, storage + used + 64);
    }

    /// `push_back` of the doubled vectors.
    fn append(e: &mut Engine, vector: u32, word: u32) {
        make_room(e, vector);
        let last = last_of(e, vector);
        e.mem.set_u32(last, word);
        e.mem.set_u32(vector + 0x10, last + 4);
    }

    /// An engine with the exe's data range mapped, the exe's constants stored and every callee
    /// outside this file doubled; the calls are logged.
    fn engine(script: &Script) -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0020_0000);
        e.call_log = Some(Vec::new());
        e.set_global::<f32>(MINUS_ONE_FLOAT, -1.0);
        e.set_global::<f64>(ZERO_DOUBLE, 0.0);
        e.set_global::<f64>(ONE_DOUBLE, 1.0);
        e.set_global::<f64>(MINUS_ONE_DOUBLE, -1.0);
        e.set_global::<f32>(TEN_FLOAT, 10.0);
        e.set_global::<f64>(LOD_SCALE_DOUBLE, 0.5);
        e.set_global::<f32>(SEED_LOW_FLOAT, 1.0);
        e.set_global::<f32>(SEED_HIGH_FLOAT, 9.0);
        e.set_global::<f32>(DEFAULT_LEAF_REDUCTION, 0.2);
        e.set_global::<f32>(WIND_SCALE_FIRST, 2.0);
        e.set_global::<f32>(WIND_SCALE_SECOND, 3.0);
        e.set_global::<u32>(STRING_NPOS, 0xffff_ffff);
        e.set_global::<u32>(HEADER_TEXT_POINTER, HEADER_EQUAL);
        for address in [
            STRING_TIDY,
            STRING_ASSIGN,
            WORD_DEFAULT_CONSTRUCT,
            WORD_TIDY,
            WORD_DESTRUCTOR,
            WORD_ASSIGN,
            BRANCH_VECTOR_PREPARE,
            BRANCH_COLLECT,
            BRANCH_PLACE,
            BRANCH_DESTRUCT,
            BRANCH_COMPUTE,
            GEOMETRY_SET_NUM_LOD_LEVELS,
            GEOMETRY_RESET_STRIP_COUNTER,
            GEOMETRY_PREPARE_LEVEL,
            GEOMETRY_FILL_EMPTY,
            RANDOM_RESEED,
            RANDOM_DESTRUCT,
            CAMERA_DESTRUCT,
            LEAF_INFO_DESTRUCT,
            BRANCH_INFO_DESTRUCT,
            BRANCH_INFO_PARSE,
            BILLBOARD_LEAF_PARSE,
            LEAF_INFO_PARSE,
            WIND_INFO_PARSE,
            LEAF_REDUCER_CONSTRUCT,
            TEXTURES_CLEAR,
            TEXTURES_DESTROY_RANGE,
            TEXTURES_BUY,
            TEXTURES_ASSIGN_CHECKED,
            TEXTURES_ASSIGN_RANGE,
            TEXTURES_CONSTRUCT_RANGE,
            SRAND,
            EH_VECTOR_CONSTRUCT,
            EH_VECTOR_DESTRUCT,
            DESTRUCTOR,
        ] {
            e.register(address, |_, _| Ret::default());
        }
        e.register(OPERATOR_NEW, |e, a| ret(e.mem.alloc(a[0])));
        e.register(OPERATOR_DELETE, |e, a| {
            if e.mem.block_size(a[0]).is_some() {
                e.mem.free(a[0]);
            }
            Ret::default()
        });
        e.register(INVALID_PARAMETER, |_, _| -> Ret {
            panic!("invalid parameter")
        });
        e.register(CXX_THROW_EXCEPTION, |_, _| -> Ret { panic!("thrown") });
        e.register(STRING_FROM_C_STRING, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            ret(a[0])
        });
        e.register(PARSE_ERROR_FROM_STRING, |e, a| -> Ret {
            panic!("parse error {:#x}", e.mem.u32(a[1]))
        });
        e.register(STRING_COMPARE, |e, a| {
            ret(u32::from(e.mem.u32(a[0]) != e.mem.u32(a[1])))
        });
        e.register(FILENAME_NO_PATH, |_, a| ret(a[1]));
        e.register(BRANCH_CONSTRUCT, |_, a| ret(a[0]));
        e.register(RANDOM_CONSTRUCT, |_, a| ret(a[0]));
        e.register(LEAF_REDUCER_APPLY, |_, a| ret(a[1]));
        e.register(TIME64, |_, _| ret(0x1234_5678));
        e.register(GET_UNIFORM, |_, a| {
            let (low, high) = (f32::from_bits(a[1]), f32::from_bits(a[2]));
            ret_float((f64::from(low) + f64::from(high)) / 2.0)
        });
        e.register(super::super::branch::FTOL, |_, a| {
            let value = f64::from_bits(u64::from(a[0]) | u64::from(a[1]) << 32);
            ret(value as i32 as u32)
        });
        e.register(VECTOR_CLEAR, |e, a| {
            let first = first_of(e, a[0]);
            e.mem.set_u32(a[0] + 0x10, first);
            Ret::default()
        });
        e.register_double(VECTOR_PUSH_BACK, |e, a| {
            let word = e.mem.u32(a[1]);
            append(e, a[0], word);
            Ret::default()
        });
        e.register(WORD_COPY, |e, a| {
            if a[1] > a[0] {
                let bytes = e.mem.bytes(a[0], a[1] - a[0]);
                e.mem.write(a[2], &bytes);
                ret(a[2] + bytes.len() as u32)
            } else {
                ret(a[2])
            }
        });
        e.register(ITERATOR_CONSTRUCT, |e, a| {
            let proxy = e.mem.u32(a[2]);
            e.mem.set_u32(a[0], proxy);
            e.mem.set_u32(a[0] + 4, a[1]);
            ret(a[0])
        });
        e.register(ADVANCE_ITERATOR, |e, a| {
            let position = e.mem.u32(a[0] + 4).wrapping_add(a[1].wrapping_mul(4));
            e.mem.set_u32(a[0] + 4, position);
            ret(a[0])
        });
        e.register(WORD_INSERT_ONE, |e, a| {
            let value = e.mem.u32(a[4]);
            let index = a[3].wrapping_sub(first_of(e, a[0])) / 4;
            make_room(e, a[0]);
            let first = first_of(e, a[0]);
            let last = last_of(e, a[0]);
            let mut at = last;
            while at > first + 4 * index {
                let moved = e.mem.u32(at - 4);
                e.mem.set_u32(at, moved);
                at -= 4;
            }
            e.mem.set_u32(first + 4 * index, value);
            e.mem.set_u32(a[0] + 0x10, last + 4);
            e.mem.set_u32(a[1], a[2]);
            e.mem.set_u32(a[1] + 4, first + 4 * index);
            ret(a[1])
        });
        e.register(PROXY_CONSTRUCT_FOR_COPY, |e, a| {
            let proxy = e.mem.alloc(4);
            e.mem.set_u32(proxy, a[0]);
            e.mem.set_u32(a[0], proxy);
            ret(a[0])
        });
        e.register(WORD_UNINIT_COPY_FOR_COPY, |e, a| {
            let bytes = e.mem.bytes(a[1], a[3].wrapping_sub(a[1]));
            e.mem.write(a[4], &bytes);
            ret(a[4].wrapping_add(bytes.len() as u32))
        });
        let tags = script.clone();
        e.register_double(PARSE_INT, move |_, _| {
            let value = tags
                .borrow_mut()
                .ints
                .pop_front()
                .expect("the tree file ran out of ints");
            ret(value as u32)
        });
        let tags = script.clone();
        e.register_double(PARSE_FLOAT, move |_, _| {
            let value = tags
                .borrow_mut()
                .floats
                .pop_front()
                .expect("the tree file ran out of floats");
            ret_float(f64::from(value))
        });
        let tags = script.clone();
        e.register_double(PARSE_BOOL, move |_, _| {
            let value = tags
                .borrow_mut()
                .bytes
                .pop_front()
                .expect("the tree file ran out of bytes");
            ret(u32::from(value))
        });
        let tags = script.clone();
        e.register_double(PEEK_INT, move |_, _| ret(tags.borrow().peek as u32));
        let tags = script.clone();
        e.register_double(PARSE_STRING, move |e, a| {
            let header = tags.borrow().header;
            e.mem.set_u32(a[1], header);
            ret(a[1])
        });
        e
    }

    fn setup(ints: &[i32], floats: &[f32], bytes: &[u8]) -> (Engine, Script) {
        let script = Rc::new(RefCell::new(Tags {
            ints: ints.iter().copied().collect(),
            floats: floats.iter().copied().collect(),
            bytes: bytes.iter().copied().collect(),
            peek: 0x1b58,
            header: HEADER_EQUAL,
        }));
        (engine(&script), script)
    }

    /// Checks the whole script was read.
    fn finished(script: &Script) {
        let tags = script.borrow();
        assert!(
            tags.ints.is_empty() && tags.floats.is_empty() && tags.bytes.is_empty(),
            "the tree file was not read to the end"
        );
    }

    /// The argument words of every call to `address`.
    fn calls(e: &Engine, address: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(called, _)| *called == address)
            .map(|(_, words)| words.clone())
            .collect()
    }

    /// The argument words of the first call to `address`.
    fn first_call(e: &Engine, address: u32) -> Vec<u32> {
        calls(e, address).remove(0)
    }

    /// The calls to any of `addresses`, in order.
    fn trace(e: &Engine, addresses: &[u32]) -> Vec<(u32, Vec<u32>)> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(called, _)| addresses.contains(called))
            .cloned()
            .collect()
    }

    /// Gives the vector at `at` storage for `capacity` elements of `size` bytes, `count` of them
    /// in use.
    fn place_vector(e: &mut Engine, at: u32, count: u32, capacity: u32, size: u32) {
        if capacity > 0 {
            let storage = e.mem.alloc(capacity * size);
            e.mem.set_u32(at + 0x0C, storage);
            e.mem.set_u32(at + 0x10, storage + count * size);
            e.mem.set_u32(at + 0x14, storage + capacity * size);
        }
    }

    /// Makes the vector at `at` hold exactly `words`.
    fn word_vector(e: &mut Engine, at: u32, words: &[u32]) {
        let count = words.len() as u32;
        place_vector(e, at, count, count, 4);
        for (i, word) in words.iter().enumerate() {
            let first = first_of(e, at);
            e.mem.set_u32(first + 4 * i as u32, *word);
        }
    }

    fn words_of(e: &Engine, at: u32) -> Vec<u32> {
        let first = first_of(e, at);
        (0..word_count(e, at))
            .map(|i| e.mem.u32(first + 4 * i))
            .collect()
    }

    fn tree(e: &mut Engine) -> u32 {
        e.mem.alloc(0x128)
    }

    /// A tree file object: a position word and a byte vector of `size` bytes.
    fn file_object(e: &mut Engine, size: u32) -> u32 {
        let file = e.mem.alloc(0x20);
        place_vector(e, file + 4, size, size, 1);
        file
    }

    // ---- destruction ----------------------------------------------------------------------

    #[test]
    fn the_scalar_deleting_destructor_frees_the_engine_only_with_bit_zero() {
        let (mut e, _) = setup(&[], &[], &[]);
        let t = tree(&mut e);
        assert_eq!(e.call(0x00b2_ae60, &args![t, 0u32]).u32(), t);
        assert_eq!(calls(&e, CAMERA_DESTRUCT), vec![vec![t]]);
        assert!(!calls(&e, OPERATOR_DELETE).contains(&vec![t]));
        assert!(e.mem.block_size(t).is_some());
        assert_eq!(e.call(0x00b2_ae60, &args![t, 1u32]).u32(), t);
        assert!(calls(&e, OPERATOR_DELETE).contains(&vec![t]));
        assert!(e.mem.block_size(t).is_none());
    }

    #[test]
    fn the_destructor_destroys_the_members_in_order() {
        let (mut e, _) = setup(&[], &[], &[]);
        let t = tree(&mut e);
        e.mem.set_u32(t + 0x7C, 0x111);
        e.mem.set_u32(t + 0x60, 0x222);
        e.call(0x00b2_ae90, &args![t]);
        let members = [
            LEAF_INFO_DESTRUCT,
            WORD_TIDY,
            OPERATOR_DELETE,
            STRING_TIDY,
            RANDOM_DESTRUCT,
            CAMERA_DESTRUCT,
        ];
        assert_eq!(
            trace(&e, &members),
            vec![
                (LEAF_INFO_DESTRUCT, vec![t + 0x94]),
                (WORD_TIDY, vec![t + 0x7C]),
                (OPERATOR_DELETE, vec![0x111]),
                (WORD_TIDY, vec![t + 0x60]),
                (OPERATOR_DELETE, vec![0x222]),
                (STRING_TIDY, vec![t + 0x24, 1, 0]),
                (RANDOM_DESTRUCT, vec![t + 0x20]),
                (CAMERA_DESTRUCT, vec![t]),
            ]
        );
    }

    #[test]
    fn the_vector_deleting_destructor_destroys_an_array_or_one_vector() {
        let (mut e, _) = setup(&[], &[], &[]);
        let block = e.mem.alloc(0x30);
        e.mem.set_u32(block, 2);
        let array = block + 4;
        assert_eq!(e.call(0x00b2_b490, &args![array, 2u32]).u32(), block);
        assert_eq!(
            calls(&e, EH_VECTOR_DESTRUCT),
            vec![vec![array, 0x18, 2, WORD_DESTRUCTOR]]
        );
        assert!(e.mem.block_size(block).is_some());
        assert_eq!(e.call(0x00b2_b490, &args![array, 3u32]).u32(), block);
        assert!(calls(&e, OPERATOR_DELETE).contains(&vec![block]));
        assert!(e.mem.block_size(block).is_none());

        let vector = e.mem.alloc(0x18);
        e.mem.set_u32(vector, 0x333);
        assert_eq!(e.call(0x00b2_b490, &args![vector, 0u32]).u32(), vector);
        assert_eq!(calls(&e, WORD_TIDY), vec![vec![vector]]);
        assert!(calls(&e, OPERATOR_DELETE).contains(&vec![0x333]));
        assert!(e.mem.block_size(vector).is_some());
        assert_eq!(e.call(0x00b2_b490, &args![vector, 1u32]).u32(), vector);
        assert!(e.mem.block_size(vector).is_none());
    }

    #[test]
    fn free_transient_data_destroys_the_branches_leaves_and_infos() {
        let (mut e, _) = setup(&[], &[], &[]);
        let t = tree(&mut e);
        e.mem.set_u8(t + 0x21, 1);
        let root = e.mem.alloc(8);
        e.mem.set_u32(t + 0x58, root);
        let leaf_object = |e: &mut Engine| {
            let vtable = e.mem.alloc(8);
            e.mem.set_u32(vtable, DESTRUCTOR);
            let leaf = e.mem.alloc(8);
            e.mem.set_u32(leaf, vtable);
            leaf
        };
        let (l1, l2) = (leaf_object(&mut e), leaf_object(&mut e));
        let block = e.mem.alloc(0x28);
        e.mem.set_u32(block, 1);
        let array = block + 4;
        word_vector(&mut e, array, &[l1, 0, l2]);
        let leaf_storage = first_of(&e, array);
        e.mem.set_u32(t + 0xD8, 1);
        e.mem.set_u32(t + 0xEC, array);
        word_vector(&mut e, t + 0x60, &[0x1111, 0, 0x2222]);
        let info_storage = first_of(&e, t + 0x60);
        e.call(0x00b2_b940, &args![t]);
        let observed = [
            BRANCH_DESTRUCT,
            OPERATOR_DELETE,
            DESTRUCTOR,
            VECTOR_CLEAR,
            EH_VECTOR_DESTRUCT,
            BRANCH_INFO_DESTRUCT,
        ];
        assert_eq!(
            trace(&e, &observed),
            vec![
                (BRANCH_DESTRUCT, vec![root]),
                (OPERATOR_DELETE, vec![root]),
                (DESTRUCTOR, vec![l1, 1]),
                (DESTRUCTOR, vec![l2, 1]),
                (VECTOR_CLEAR, vec![array]),
                (EH_VECTOR_DESTRUCT, vec![array, 0x18, 1, WORD_DESTRUCTOR]),
                (OPERATOR_DELETE, vec![array - 4]),
                (BRANCH_INFO_DESTRUCT, vec![0x1111]),
                (OPERATOR_DELETE, vec![0x1111]),
                (BRANCH_INFO_DESTRUCT, vec![0x2222]),
                (OPERATOR_DELETE, vec![0x2222]),
                (VECTOR_CLEAR, vec![t + 0x60]),
            ]
        );
        assert_eq!(e.mem.u32(t + 0x58), 0);
        assert_eq!(e.mem.u32(t + 0xEC), 0);
        assert_eq!(e.mem.u8(t + 0x21), 0);
        for i in 0..3 {
            assert_eq!(e.mem.u32(leaf_storage + 4 * i), 0);
            assert_eq!(e.mem.u32(info_storage + 4 * i), 0);
        }
    }

    #[test]
    fn free_transient_data_does_nothing_once_the_data_is_gone() {
        let (mut e, _) = setup(&[], &[], &[]);
        let t = tree(&mut e);
        e.mem.set_u32(t + 0x58, 0x1234);
        e.call(0x00b2_b940, &args![t]);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
        assert_eq!(e.mem.u32(t + 0x58), 0x1234);
    }

    #[test]
    fn the_vector_of_eight_byte_elements_is_tidied_and_its_proxy_freed() {
        let (mut e, _) = setup(&[], &[], &[]);
        let v = e.mem.alloc(0x18);
        e.mem.set_u32(v, 0x4444);
        place_vector(&mut e, v, 2, 2, 8);
        let storage = first_of(&e, v);
        e.call(0x00b2_cb50, &args![v]);
        assert_eq!(
            trace(&e, &[OPERATOR_DELETE]),
            vec![
                (OPERATOR_DELETE, vec![storage]),
                (OPERATOR_DELETE, vec![0x4444])
            ]
        );
        assert_eq!(
            (first_of(&e, v), last_of(&e, v), e.mem.u32(v + 0x14)),
            (0, 0, 0)
        );
    }

    #[test]
    fn the_tidy_of_eight_byte_elements_frees_only_existing_storage() {
        let (mut e, _) = setup(&[], &[], &[]);
        let v = e.mem.alloc(0x18);
        place_vector(&mut e, v, 2, 3, 8);
        let storage = first_of(&e, v);
        e.call(0x00b2_e1b0, &args![v]);
        assert_eq!(calls(&e, OPERATOR_DELETE), vec![vec![storage]]);
        assert_eq!(
            (first_of(&e, v), last_of(&e, v), e.mem.u32(v + 0x14)),
            (0, 0, 0)
        );
        e.call(0x00b2_e1b0, &args![v]);
        assert_eq!(calls(&e, OPERATOR_DELETE).len(), 1);
    }

    // ---- small accessors ------------------------------------------------------------------

    #[test]
    fn the_wind_strength_setter_stores_the_strength_and_the_oscillation_vector() {
        let (mut e, _) = setup(&[], &[], &[]);
        let t = tree(&mut e);
        e.mem.set_f32(t + 0x10C, 2.0);
        e.mem.set_f32(t + 0x110, 0.5);
        e.call(0x00b2_bc70, &args![t, 4.0f32]);
        assert_eq!(e.mem.f32(t + 0x124), 4.0);
        assert_eq!(e.mem.f32(t + 0x118), -24.0);
        assert_eq!(e.mem.f32(t + 0x11C), 24.0);
        assert_eq!(e.mem.f32(t + 0x120), 4.0);
    }

    #[test]
    fn the_size_getter_copies_the_size_and_the_variance() {
        let (mut e, _) = setup(&[], &[], &[]);
        let t = tree(&mut e);
        e.mem.set_f32(t + 0x4C, 12.5);
        e.mem.set_f32(t + 0x50, 1.5);
        let out = e.mem.alloc(8);
        e.call(0x00b2_bcf0, &args![t, out, out + 4]);
        assert_eq!(e.mem.f32(out), 12.5);
        assert_eq!(e.mem.f32(out + 4), 1.5);
    }

    #[test]
    fn set_size_stores_the_size_and_the_variance() {
        let (mut e, _) = setup(&[], &[], &[]);
        let t = tree(&mut e);
        e.call(0x00b2_bd20, &args![t, 30.0f32, 4.0f32]);
        assert_eq!(e.mem.f32(t + 0x4C), 30.0);
        assert_eq!(e.mem.f32(t + 0x50), 4.0);
    }

    #[test]
    fn the_lod_distance_setter_stores_near_then_far() {
        let (mut e, _) = setup(&[], &[], &[]);
        let t = tree(&mut e);
        e.call(0x00b2_bdc0, &args![t, 10.0f32, 30.0f32]);
        assert_eq!(e.mem.f32(t + 0x44), 10.0);
        assert_eq!(e.mem.f32(t + 0x40), 30.0);
    }

    fn lod_at(distance: f64) -> f32 {
        let (mut e, _) = setup(&[], &[], &[]);
        e.register_double(VEC3_DISTANCE, move |_, _| ret_float(distance));
        let t = tree(&mut e);
        e.mem.set_f32(t + 0x44, 10.0);
        e.mem.set_f32(t + 0x40, 30.0);
        let lod = e.call(0x00b2_bd40, &args![t]).f32();
        assert_eq!(calls(&e, VEC3_DISTANCE), vec![vec![CAMERA_POSITION, t + 4]]);
        assert_eq!(e.mem.f32(t + 0x14), lod);
        lod
    }

    #[test]
    fn compute_lod_is_the_position_between_near_and_far_clamped_to_zero_and_one() {
        assert_eq!(lod_at(20.0), 0.5);
        assert_eq!(lod_at(5.0), 1.0);
        assert_eq!(lod_at(40.0), 0.0);
    }

    #[test]
    fn set_seed_draws_a_seed_for_zero_keeps_it_for_one_and_stores_others() {
        let (mut e, _) = setup(&[], &[], &[]);
        let t = tree(&mut e);
        e.mem.set_u32(t + 0x48, 77);
        e.call(0x00b2_c0c0, &args![t, 1u32]);
        assert_eq!(e.mem.u32(t + 0x48), 77);
        e.call(0x00b2_c0c0, &args![t, 42u32]);
        assert_eq!(e.mem.u32(t + 0x48), 42);
        assert!(calls(&e, RANDOM_RESEED).is_empty());
        e.call(0x00b2_c0c0, &args![t, 0u32]);
        assert_eq!(e.mem.i32(t + 0x48), 5);
        assert_eq!(calls(&e, RANDOM_RESEED), vec![vec![t + 0x20, 0xffff_ffff]]);
        assert_eq!(
            calls(&e, GET_UNIFORM),
            vec![vec![t + 0x20, 1.0f32.to_bits(), 9.0f32.to_bits()]]
        );
    }

    #[test]
    fn the_string_assignments_empty_the_destination_first_only_for_b8d0() {
        let (mut e, _) = setup(&[], &[], &[]);
        let (this, other) = (e.mem.alloc(0x1c), e.mem.alloc(0x1c));
        assert_eq!(e.call(0x00b2_b8d0, &args![this, other]).u32(), this);
        assert_eq!(
            trace(&e, &[STRING_TIDY, STRING_ASSIGN]),
            vec![
                (STRING_TIDY, vec![this, 0, 0]),
                (STRING_ASSIGN, vec![this, other, 0, 0xffff_ffff]),
            ]
        );
        assert_eq!(e.call(0x00b2_b910, &args![this, other]).u32(), this);
        assert_eq!(calls(&e, STRING_TIDY).len(), 1);
        assert_eq!(calls(&e, STRING_ASSIGN).len(), 2);
    }

    // ---- compute --------------------------------------------------------------------------

    #[test]
    fn compute_builds_the_branch_tree_and_the_leaf_lod_array() {
        let (mut e, _) = setup(&[], &[], &[]);
        let t = tree(&mut e);
        let geometry = e.mem.alloc(0x100);
        e.mem.set_u32(t + 0x5C, geometry);
        e.mem.set_u32(t + 0x48, 5);
        e.mem.set_f32(t + 0x4C, 10.0);
        e.mem.set_f32(t + 0x50, 2.0);
        e.mem.set_u32(t + 0x54, 7);
        e.mem.set_u32(t + 0x78, 1);
        e.mem.set_u32(t + 0x108, 7);
        e.mem.set_u32(t + 0xD8, 1);
        word_vector(&mut e, t + 0x60, &[0xa1, 0xa2]);
        place_vector(&mut e, t + 0xA4, 1, 1, 0x54);
        let texture = first_of(&e, t + 0xA4);
        e.mem.set_f32(texture + 0x3C, 4.0);
        e.mem.set_f32(texture + 0x40, 8.0);
        e.call(0x00b2_af80, &args![t, 0.5f32]);

        assert_eq!(e.mem.u8(EXPAND_LAST_FLAG), 1);
        let branch = e.mem.u32(t + 0x58);
        assert_ne!(branch, 0);
        assert_eq!(calls(&e, BRANCH_CONSTRUCT), vec![vec![branch, 0]]);
        let array = e.mem.u32(t + 0xEC);
        assert_ne!(array, 0);
        assert_eq!(e.mem.u32(array - 4), 1);
        assert_eq!(calls(&e, OPERATOR_NEW), vec![vec![0x50], vec![0x1c]]);
        assert_eq!(
            calls(&e, EH_VECTOR_CONSTRUCT),
            vec![vec![
                array,
                0x18,
                1,
                WORD_DEFAULT_CONSTRUCT,
                WORD_DESTRUCTOR
            ]]
        );
        assert_eq!(e.mem.u32(LEVEL_MARKER), 7);
        assert_eq!(e.mem.u32(LEAF_INFO_POINTER), t + 0x94);
        assert_eq!(words_of(&e, BRANCH_INFO_LIST), vec![0xa1, 0xa2]);
        assert_eq!(
            calls(&e, RANDOM_RESEED),
            vec![vec![t + 0x20, 5], vec![t + 0x20, 5]]
        );
        assert_eq!(
            calls(&e, GET_UNIFORM),
            vec![vec![t + 0x20, 8.0f32.to_bits(), 12.0f32.to_bits()]]
        );
        assert_eq!(calls(&e, SRAND), vec![vec![7], vec![0x1234_5678]]);
        assert_eq!(calls(&e, TIME64), vec![vec![0]]);
        assert_eq!(e.mem.f32(texture + 0x48), 40.0);
        assert_eq!(e.mem.f32(texture + 0x4C), 80.0);
        assert_eq!(
            calls(&e, GEOMETRY_SET_NUM_LOD_LEVELS),
            vec![vec![geometry, 1]]
        );
        assert_eq!(e.mem.u32(BRANCH_COUNTER), 5);
        let compute = first_call(&e, BRANCH_COMPUTE);
        assert_eq!(compute.len(), 14);
        assert_eq!(compute[0], branch);
        assert_eq!(compute[1], 5);
        assert_eq!(compute[2], 10.0f32.to_bits());
        assert_eq!(compute[3], 0);
        assert_eq!(compute[4], t + 4);
        assert_eq!((compute[5], compute[6]), (0, 0));
        assert_eq!(compute[9], geometry);
        assert_eq!(compute[10], t + 0x7C);
        assert_eq!(compute[11], 1.0f32.to_bits());
        assert_eq!(compute[12], 5);
        assert_eq!(compute[13], (-1.0f32).to_bits());
        // The reduction ran (an empty tree fills the empty first level) and the lod builder
        // copied the billboard leaves into the first level.
        assert_eq!(
            calls(&e, GEOMETRY_FILL_EMPTY),
            vec![vec![geometry, 0, 0, 0]]
        );
        assert_eq!(calls(&e, WORD_ASSIGN), vec![vec![array, t + 0x7C]]);
    }

    #[test]
    fn compute_with_parsed_leaf_lods_keeps_them_and_builds_no_lods() {
        let (mut e, _) = setup(&[], &[], &[]);
        e.mem.set_u8(EXPAND_LAST_FLAG, 0xff);
        let t = tree(&mut e);
        let geometry = e.mem.alloc(0x100);
        e.mem.set_u32(t + 0x5C, geometry);
        e.mem.set_u8(t + 0xF0, 1);
        e.mem.set_u32(t + 0x78, 1);
        e.call(0x00b2_af80, &args![t, 0.5f32]);
        assert_eq!(e.mem.u8(EXPAND_LAST_FLAG), 0);
        assert_eq!(e.mem.u32(t + 0xEC), 0);
        assert!(calls(&e, EH_VECTOR_CONSTRUCT).is_empty());
        assert!(calls(&e, WORD_ASSIGN).is_empty());
        assert_eq!(calls(&e, SRAND), vec![vec![0], vec![0x1234_5678]]);
    }

    // ---- the branch reduction -------------------------------------------------------------

    #[test]
    fn the_reduction_moves_large_branches_to_the_front_and_places_them_by_level() {
        let (mut e, _) = setup(&[], &[], &[]);
        let t = tree(&mut e);
        let geometry = e.mem.alloc(0x100);
        place_vector(&mut e, geometry + 0x88, 9, 9, 4);
        e.mem.set_u32(t + 0x5C, geometry);
        let root = e.mem.alloc(8);
        e.mem.set_u32(t + 0x58, root);
        e.mem.set_f32(t + 0x104, 0.25);
        e.mem.set_f32(t + 0x100, 0.5);
        e.mem.set_u32(t + 0x78, 2);
        e.mem.set_f32(t + 0xF4, 0.5);
        e.mem.set_f32(t + 0xF8, 1.0);
        let mut branches = Vec::new();
        for volume in [1.0f32, 4.0, 2.0] {
            let branch = e.mem.alloc(0x40);
            e.mem.set_f32(branch + 0x30, volume);
            branches.push(branch);
        }
        let collected = branches.clone();
        e.register_double(BRANCH_COLLECT, move |e, a| {
            for branch in &collected {
                append(e, a[1], *branch);
            }
            Ret::default()
        });
        e.call(0x00b2_c130, &args![t]);

        let (b1, b2, b3) = (branches[0], branches[1], branches[2]);
        assert_eq!(calls(&e, BRANCH_COLLECT)[0][0], root);
        assert_eq!(e.mem.f32(b1 + 0x34), 1.75);
        assert_eq!(e.mem.f32(b3 + 0x34), 2.5);
        assert_eq!(e.mem.f32(b2 + 0x34), 0.0);
        let draws = calls(&e, GET_UNIFORM);
        assert_eq!(draws.len(), 2);
        assert!(draws
            .iter()
            .all(|draw| draw[1] == 0.0f32.to_bits() && draw[2] == 0.5f32.to_bits()));
        assert_eq!(
            calls(&e, BRANCH_PLACE),
            vec![
                vec![b2, 0, geometry],
                vec![b1, 0, geometry],
                vec![b3, 0, geometry],
                vec![b2, 1, geometry],
            ]
        );
        assert_eq!(calls(&e, GEOMETRY_PREPARE_LEVEL), vec![vec![geometry, 0]]);
        assert_eq!(
            calls(&e, GEOMETRY_RESET_STRIP_COUNTER),
            vec![vec![geometry, 0], vec![geometry, 1]]
        );
        assert!(calls(&e, GEOMETRY_FILL_EMPTY).is_empty());
        assert_eq!(calls(&e, BRANCH_VECTOR_PREPARE).len(), 2);
        assert_eq!(calls(&e, RANDOM_CONSTRUCT).len(), 1);
        assert_eq!(calls(&e, RANDOM_DESTRUCT).len(), 1);
    }

    #[test]
    fn the_reduction_of_a_tree_without_branches_fills_the_empty_first_level() {
        let (mut e, _) = setup(&[], &[], &[]);
        let t = tree(&mut e);
        let geometry = e.mem.alloc(0x100);
        e.mem.set_u32(t + 0x5C, geometry);
        e.mem.set_u32(t + 0x78, 1);
        e.call(0x00b2_c130, &args![t]);
        assert!(calls(&e, BRANCH_COLLECT).is_empty());
        assert_eq!(calls(&e, GEOMETRY_PREPARE_LEVEL), vec![vec![geometry, 0]]);
        assert_eq!(
            calls(&e, GEOMETRY_RESET_STRIP_COUNTER),
            vec![vec![geometry, 0]]
        );
        assert_eq!(
            calls(&e, GEOMETRY_FILL_EMPTY),
            vec![vec![geometry, 0, 0, 0]]
        );
        assert!(calls(&e, BRANCH_PLACE).is_empty());
    }

    #[test]
    fn the_checked_erase_moves_the_later_elements_down_and_answers_an_iterator() {
        let (mut e, _) = setup(&[], &[], &[]);
        let v = e.mem.alloc(0x18);
        e.mem.set_u32(v, 0x4444);
        word_vector(&mut e, v, &[10, 20, 30, 40]);
        let first = first_of(&e, v);
        let out = e.mem.alloc(8);
        assert_eq!(
            e.call(0x00b2_def0, &args![v, out, 0u32, first + 4]).u32(),
            out
        );
        assert_eq!(words_of(&e, v), vec![10, 30, 40]);
        assert_eq!(e.mem.u32(out), 0x4444);
        assert_eq!(e.mem.u32(out + 4), first + 4);
        e.call(0x00b2_def0, &args![v, out, 0u32, first + 8]);
        assert_eq!(words_of(&e, v), vec![10, 30]);
        assert_eq!(e.mem.u32(out + 4), first + 8);
    }

    #[test]
    #[should_panic(expected = "invalid parameter")]
    fn the_checked_erase_rejects_a_position_past_the_end() {
        let (mut e, _) = setup(&[], &[], &[]);
        let v = e.mem.alloc(0x18);
        word_vector(&mut e, v, &[10, 20, 30, 40]);
        let first = first_of(&e, v);
        let out = e.mem.alloc(8);
        e.call(0x00b2_def0, &args![v, out, 0u32, first + 16]);
    }

    // ---- the leaf lod builder -------------------------------------------------------------

    /// A tree with a two-level leaf lod array whose first level holds two words.
    fn lod_scene(e: &mut Engine) -> (u32, u32) {
        let t = tree(e);
        let block = e.mem.alloc(0x40);
        e.mem.set_u32(block, 2);
        let array = block + 4;
        word_vector(e, array, &[0x111, 0x222]);
        e.mem.set_u32(t + 0xEC, array);
        e.mem.set_u32(t + 0xD8, 2);
        e.mem.set_f32(t + 0xFC, 0.4);
        (t, array)
    }

    #[test]
    fn the_leaf_lod_builder_reduces_each_further_level_from_the_previous_one() {
        let (mut e, _) = setup(&[], &[], &[]);
        let (t, array) = lod_scene(&mut e);
        place_vector(&mut e, t + 0xA4, 1, 1, 0x54);
        let texture = first_of(&e, t + 0xA4);
        e.mem.set_f32(texture + 0x48, 3.0);
        e.mem.set_f32(texture + 0x4C, 5.0);
        e.mem.set_f32(t + 0xBC, 2.0);
        let seen: Rc<RefCell<Vec<u32>>> = Rc::new(RefCell::new(Vec::new()));
        let sink = seen.clone();
        e.register_double(LEAF_REDUCER_APPLY, move |e, a| {
            sink.borrow_mut()
                .extend([e.mem.u32(a[5]), e.mem.u32(a[5] + 4), a[6] - a[5]]);
            ret(a[1])
        });
        e.call(0x00b2_c890, &args![t, 0.25f32]);

        let construct = first_call(&e, LEAF_REDUCER_CONSTRUCT);
        assert_eq!(
            construct[1..],
            [
                t + 0x94,
                5.0f32.to_bits(),
                0.4f32.to_bits(),
                1.25f32.to_bits()
            ]
        );
        let apply = first_call(&e, LEAF_REDUCER_APPLY);
        assert_eq!(apply.len(), 8);
        assert_eq!(apply[0], construct[0]);
        assert_eq!(*seen.borrow(), vec![0x111, 0x222, 8]);
        assert_eq!(
            calls(&e, WORD_ASSIGN),
            vec![vec![array, t + 0x7C], vec![array + 0x18, apply[1]]]
        );
        assert_eq!(calls(&e, WORD_TIDY), vec![vec![apply[1]]]);
    }

    #[test]
    fn the_leaf_lod_builder_takes_ten_as_the_largest_size_without_leaf_textures() {
        let (mut e, _) = setup(&[], &[], &[]);
        let (t, _) = lod_scene(&mut e);
        e.set_global::<f64>(LOD_SCALE_DOUBLE, 0.25);
        e.call(0x00b2_c890, &args![t, 0.5f32]);
        let construct = first_call(&e, LEAF_REDUCER_CONSTRUCT);
        assert_eq!(
            construct[1..],
            [
                t + 0x94,
                2.5f32.to_bits(),
                0.4f32.to_bits(),
                1.5f32.to_bits()
            ]
        );
    }

    #[test]
    fn the_leaf_lod_builder_leaves_a_missing_array_alone() {
        let (mut e, _) = setup(&[], &[], &[]);
        let t = tree(&mut e);
        e.call(0x00b2_c890, &args![t, 0.5f32]);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
    }

    // ---- the parsers ----------------------------------------------------------------------

    #[test]
    fn parse_reads_the_sections_and_applies_the_override_size() {
        let (mut e, script) = setup(
            &[0x3e8, 0x3ea, 0x7d1, 0x3eb, 0x3ec, 0x3f3, 0x3e9],
            &[50.0],
            &[],
        );
        let t = tree(&mut e);
        e.mem.set_f32(t + 0x18, 20.0);
        e.mem.set_f32(t + 0x1C, 3.0);
        let f = file_object(&mut e, 0);
        assert!(e.call(0x00b2_b550, &args![t, f]).bool());
        finished(&script);
        assert_eq!(e.mem.f32(t + 0x40), 50.0);
        assert_eq!(calls(&e, LEAF_INFO_PARSE), vec![vec![t + 0x94, f]]);
        assert_eq!(calls(&e, WIND_INFO_PARSE), vec![vec![t + 0x10C, f]]);
        assert_eq!(e.mem.f32(t + 0x4C), 20.0);
        assert_eq!(e.mem.f32(t + 0x50), 3.0);
        assert_eq!(calls(&e, STRING_COMPARE).len(), 1);
    }

    #[test]
    fn parse_reads_the_leaf_clusters_when_the_file_has_more_bytes() {
        let (mut e, script) = setup(&[0x3e8, 0x3ec, 0x3e9, 0x1b58, 0, 0x1b59], &[], &[]);
        let t = tree(&mut e);
        e.mem.set_f32(t + 0x18, -1.0);
        let f = file_object(&mut e, 8);
        assert!(e.call(0x00b2_b550, &args![t, f]).bool());
        finished(&script);
        assert_eq!(e.mem.u8(t + 0xF0), 1);
        assert_eq!(e.mem.u32(t + 0xD8), 0);
        assert_ne!(e.mem.u32(t + 0xEC), 0);
        assert_eq!(e.mem.f32(t + 0x4C), 0.0);
        assert_eq!(calls(&e, PEEK_INT), vec![vec![f]]);
        // The branch texture file name is reassigned through a copy and the leaf texture vector
        // is assigned to itself (nothing to do).
        assert_eq!(calls(&e, STRING_ASSIGN).len(), 2);
    }

    #[test]
    #[should_panic(expected = "parse error 0x10a56b8")]
    fn parse_rejects_a_file_that_does_not_begin_with_the_begin_tag() {
        let (mut e, _) = setup(&[0x3e7], &[], &[]);
        let t = tree(&mut e);
        let f = file_object(&mut e, 0);
        e.call(0x00b2_b550, &args![t, f]);
    }

    #[test]
    #[should_panic(expected = "parse error 0x10a5698")]
    fn parse_rejects_a_file_with_another_header() {
        let (mut e, script) = setup(&[0x3e8], &[], &[]);
        script.borrow_mut().header = HEADER_OTHER;
        let t = tree(&mut e);
        let f = file_object(&mut e, 0);
        e.call(0x00b2_b550, &args![t, f]);
    }

    #[test]
    #[should_panic(expected = "parse error 0x10a5678")]
    fn parse_rejects_an_unknown_section() {
        let (mut e, _) = setup(&[0x3e8, 0x1234], &[], &[]);
        let t = tree(&mut e);
        let f = file_object(&mut e, 0);
        e.call(0x00b2_b550, &args![t, f]);
    }

    #[test]
    fn parse_tree_info_reads_every_tag() {
        let (mut e, script) = setup(
            &[
                0x7d0, 0x7d1, 0x7d2, 0x7d3, 0x7d4, 99, 0x7d5, 777, 0x7d6, 0x7d7, 0x3f6, 1, 0x3f7,
                0x3eb,
            ],
            &[50.0, 5.0, 30.0, 4.0],
            &[1],
        );
        e.register(BRANCH_INFO_PARSE, |_, _| ret(0xb1));
        let t = tree(&mut e);
        let f = file_object(&mut e, 0);
        e.call(0x00b2_bde0, &args![t, f]);
        finished(&script);
        assert_eq!(e.mem.f32(t + 0x40), 50.0);
        assert_eq!(e.mem.f32(t + 0x44), 5.0);
        assert_eq!(e.mem.f32(t + 0x4C), 30.0);
        assert_eq!(e.mem.f32(t + 0x50), 4.0);
        assert_eq!(e.mem.u32(t + 0x48), 777);
        assert_eq!(calls(&e, PARSE_BOOL), vec![vec![f]]);
        assert_eq!(words_of(&e, t + 0x60), vec![0xb1]);
        let parsed = first_call(&e, PARSE_STRING);
        assert_eq!(parsed[0], f);
        let assigns = calls(&e, STRING_ASSIGN);
        assert_eq!(assigns.len(), 3);
        assert_eq!(assigns[0][1], parsed[1]);
        assert_eq!(assigns[1][0], t + 0x24);
        assert_eq!(assigns[1][1], assigns[0][0]);
        assert_eq!(assigns[2][0], t + 0x24);
        let no_path = first_call(&e, FILENAME_NO_PATH);
        assert_eq!(no_path[0], t + 0x24);
        assert_eq!(assigns[2][1], no_path[1]);
    }

    #[test]
    #[should_panic(expected = "parse error 0x10a56d4")]
    fn parse_tree_info_rejects_an_unknown_tag() {
        let (mut e, _) = setup(&[0x1], &[], &[]);
        let t = tree(&mut e);
        let f = file_object(&mut e, 0);
        e.call(0x00b2_bde0, &args![t, f]);
    }

    #[test]
    #[should_panic(expected = "parse error 0x10a56d4")]
    fn parse_tree_info_rejects_a_tag_past_the_last_one() {
        let (mut e, _) = setup(&[0x7d8], &[], &[]);
        let t = tree(&mut e);
        let f = file_object(&mut e, 0);
        e.call(0x00b2_bde0, &args![t, f]);
    }

    #[test]
    fn the_branch_info_section_appends_each_parsed_info() {
        let (mut e, script) = setup(&[2, 0x3f7], &[], &[]);
        let mut next = 0xb0u32;
        e.register_double(BRANCH_INFO_PARSE, move |_, _| {
            next += 1;
            ret(next)
        });
        let t = tree(&mut e);
        word_vector(&mut e, t + 0x60, &[0x99]);
        let f = file_object(&mut e, 0);
        e.call(0x00b2_c790, &args![t, f]);
        finished(&script);
        assert_eq!(calls(&e, BRANCH_INFO_PARSE), vec![vec![f], vec![f]]);
        assert_eq!(words_of(&e, t + 0x60), vec![0xb1, 0xb2]);
    }

    #[test]
    #[should_panic(expected = "parse error 0x10a54cc")]
    fn the_branch_info_section_needs_its_end_tag() {
        let (mut e, _) = setup(&[0, 0x3f8], &[], &[]);
        let t = tree(&mut e);
        let f = file_object(&mut e, 0);
        e.call(0x00b2_c790, &args![t, f]);
    }

    #[test]
    fn the_leaf_cluster_parser_builds_the_lod_array_and_fills_the_levels() {
        let (mut e, script) = setup(
            &[2, 0x1b5a, 0x1b5c, 0x1b5c, 0x1b5b, 0x1b5a, 0x1b5b, 0x1b59],
            &[],
            &[],
        );
        let mut next = 0xc0u32;
        e.register_double(BILLBOARD_LEAF_PARSE, move |_, _| {
            next += 1;
            ret(next)
        });
        let t = tree(&mut e);
        let block = e.mem.alloc(0x30);
        e.mem.set_u32(block, 3);
        let old = block + 4;
        e.mem.set_u32(t + 0xEC, old);
        let f = file_object(&mut e, 0);
        e.call(0x00b2_cbc0, &args![t, f]);
        finished(&script);
        assert_eq!(
            calls(&e, EH_VECTOR_DESTRUCT),
            vec![vec![old, 0x18, 3, WORD_DESTRUCTOR]]
        );
        assert!(calls(&e, OPERATOR_DELETE).contains(&vec![block]));
        assert_eq!(e.mem.u32(t + 0xD8), 2);
        assert_eq!(e.mem.u8(t + 0xF0), 1);
        let array = e.mem.u32(t + 0xEC);
        assert_eq!(e.mem.u32(array - 4), 2);
        assert_eq!(
            calls(&e, EH_VECTOR_CONSTRUCT),
            vec![vec![
                array,
                0x18,
                2,
                WORD_DEFAULT_CONSTRUCT,
                WORD_DESTRUCTOR
            ]]
        );
        assert_eq!(words_of(&e, array), vec![0xc1, 0xc2]);
        assert_eq!(words_of(&e, array + 0x18), Vec::<u32>::new());
    }

    #[test]
    #[should_panic(expected = "parse error 0x10a5710")]
    fn the_leaf_cluster_parser_rejects_more_levels_than_announced() {
        let (mut e, _) = setup(&[1, 0x1b5a, 0x1b5b, 0x1b5a], &[], &[]);
        let t = tree(&mut e);
        let f = file_object(&mut e, 0);
        e.call(0x00b2_cbc0, &args![t, f]);
    }

    #[test]
    #[should_panic(expected = "parse error 0x10a56f8")]
    fn the_leaf_cluster_parser_rejects_an_unknown_tag() {
        let (mut e, _) = setup(&[1, 0x7], &[], &[]);
        let t = tree(&mut e);
        let f = file_object(&mut e, 0);
        e.call(0x00b2_cbc0, &args![t, f]);
    }

    #[test]
    fn the_lod_info_parser_reads_every_tag_and_defaults_a_zero_leaf_reduction() {
        let (mut e, script) = setup(
            &[
                0x232f, 3, 0x2330, 0x2332, 0x2333, 4, 0x2334, 0x2335, 0x2336, 0x232e,
            ],
            &[0.25, 0.0, 0.9, 0.1, 0.3],
            &[],
        );
        let t = tree(&mut e);
        let f = file_object(&mut e, 0);
        e.call(0x00b2_ceb0, &args![t, f]);
        finished(&script);
        assert_eq!(e.mem.u32(t + 0x78), 3);
        assert_eq!(e.mem.f32(t + 0xF4), 0.25);
        assert_eq!(e.mem.f32(t + 0xFC), 0.2);
        assert_eq!(e.mem.u32(t + 0xD8), 4);
        assert_eq!(e.mem.f32(t + 0xF8), 0.9);
        assert_eq!(e.mem.f32(t + 0x100), 0.1);
        assert_eq!(e.mem.f32(t + 0x104), 0.3);
    }

    #[test]
    fn the_lod_info_parser_keeps_a_nonzero_leaf_reduction() {
        let (mut e, script) = setup(&[0x2332, 0x232e], &[0.6], &[]);
        let t = tree(&mut e);
        let f = file_object(&mut e, 0);
        e.call(0x00b2_ceb0, &args![t, f]);
        finished(&script);
        assert_eq!(e.mem.f32(t + 0xFC), 0.6);
    }

    #[test]
    #[should_panic(expected = "parse error 0x10a572c")]
    fn the_lod_info_parser_rejects_an_unknown_tag() {
        let (mut e, _) = setup(&[0x2331], &[], &[]);
        let t = tree(&mut e);
        let f = file_object(&mut e, 0);
        e.call(0x00b2_ceb0, &args![t, f]);
    }

    #[test]
    fn the_texture_controls_parser_fills_each_branch_info() {
        let (mut e, script) = setup(
            &[0x3a9a, 0x3a9b, 0x3a9a, 0x3a9b, 0x3a99],
            &[0.5, 1.5],
            &[1, 0],
        );
        let t = tree(&mut e);
        let (i1, i2) = (e.mem.alloc(0x50), e.mem.alloc(0x50));
        word_vector(&mut e, t + 0x60, &[i1, i2]);
        e.mem.set_u8(i2 + 0x1E, 0xff);
        let f = file_object(&mut e, 0);
        e.call(0x00b2_d090, &args![t, f]);
        finished(&script);
        assert_eq!(e.mem.u8(i1 + 0x1E), 1);
        assert_eq!(e.mem.f32(i1 + 0x20), 0.5);
        assert_eq!(e.mem.u8(i2 + 0x1E), 0);
        assert_eq!(e.mem.f32(i2 + 0x20), 1.5);
    }

    #[test]
    #[should_panic(expected = "parse error 0x10a5748")]
    fn the_texture_controls_parser_rejects_an_unknown_tag() {
        let (mut e, _) = setup(&[0x1], &[], &[]);
        let t = tree(&mut e);
        let info = e.mem.alloc(0x50);
        word_vector(&mut e, t + 0x60, &[info]);
        let f = file_object(&mut e, 0);
        e.call(0x00b2_d090, &args![t, f]);
    }

    #[test]
    fn the_flare_info_parser_fills_each_branch_info() {
        let (mut e, script) = setup(
            &[
                0x3e82, 0x3e83, 7, 0x3e84, 0x3e85, 0x3e86, 0x3e87, 0x3e88, 0x3e89, 0x3e8a, 0x3e8b,
                0x3e8c, 0x3e81,
            ],
            &[1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0],
            &[],
        );
        let t = tree(&mut e);
        let info = e.mem.alloc(0x50);
        word_vector(&mut e, t + 0x60, &[info]);
        let f = file_object(&mut e, 0);
        e.call(0x00b2_d2e0, &args![t, f]);
        finished(&script);
        assert_eq!(e.mem.f32(info + 0x24), 1.0);
        assert_eq!(e.mem.i32(info + 0x28), 7);
        for (field, value) in [
            (0x2cu32, 2.0f32),
            (0x30, 3.0),
            (0x34, 4.0),
            (0x38, 5.0),
            (0x3c, 6.0),
            (0x40, 7.0),
            (0x44, 8.0),
            (0x48, 9.0),
            (0x4c, 10.0),
        ] {
            assert_eq!(e.mem.f32(info + field), value, "field {field:#x}");
        }
    }

    #[test]
    #[should_panic(expected = "parse error 0x10a5764")]
    fn the_flare_info_parser_rejects_an_unknown_tag() {
        let (mut e, _) = setup(&[0x1], &[], &[]);
        let t = tree(&mut e);
        let info = e.mem.alloc(0x50);
        word_vector(&mut e, t + 0x60, &[info]);
        let f = file_object(&mut e, 0);
        e.call(0x00b2_d2e0, &args![t, f]);
    }

    // ---- the leaf texture vector assignment -----------------------------------------------

    fn texture_vector(e: &mut Engine, count: u32, capacity: u32) -> u32 {
        let v = e.mem.alloc(0x18);
        place_vector(e, v, count, capacity, 0x54);
        v
    }

    const TEXTURE: u32 = 0x54;
    const CONSTRUCTED_END: u32 = 0x7777_0000;

    #[test]
    fn assigning_a_texture_vector_to_itself_or_from_an_empty_one() {
        let (mut e, _) = setup(&[], &[], &[]);
        let v = texture_vector(&mut e, 2, 2);
        assert_eq!(e.call(0x00b2_db10, &args![v, v]).u32(), v);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
        let empty = texture_vector(&mut e, 0, 0);
        assert_eq!(e.call(0x00b2_db10, &args![v, empty]).u32(), v);
        assert_eq!(calls(&e, TEXTURES_CLEAR), vec![vec![v]]);
    }

    #[test]
    fn assigning_a_larger_texture_vector_within_the_capacity() {
        let (mut e, _) = setup(&[], &[], &[]);
        e.register(TEXTURES_CONSTRUCT_RANGE, |_, _| ret(CONSTRUCTED_END));
        let v = texture_vector(&mut e, 1, 3);
        let o = texture_vector(&mut e, 3, 3);
        let (vf, vl) = (first_of(&e, v), last_of(&e, v));
        let (of, ol) = (first_of(&e, o), last_of(&e, o));
        assert_eq!(e.call(0x00b2_db10, &args![v, o]).u32(), v);
        assert_eq!(
            calls(&e, TEXTURES_ASSIGN_RANGE),
            vec![vec![of, of + TEXTURE, vf]]
        );
        assert_eq!(
            calls(&e, TEXTURES_CONSTRUCT_RANGE),
            vec![vec![of + TEXTURE, ol, vl, v + 8, 0, 0]]
        );
        assert_eq!(e.mem.u32(v + 0x10), CONSTRUCTED_END);
    }

    #[test]
    fn assigning_a_larger_texture_vector_beyond_the_capacity_buys_new_storage() {
        let (mut e, _) = setup(&[], &[], &[]);
        e.register(TEXTURES_CONSTRUCT_RANGE, |_, _| ret(CONSTRUCTED_END));
        e.register(TEXTURES_BUY, |e, a| {
            e.mem.set_u32(a[0] + 0x0C, 0x8888_0000);
            ret(1)
        });
        let v = texture_vector(&mut e, 1, 1);
        let o = texture_vector(&mut e, 3, 3);
        let (vf, vl) = (first_of(&e, v), last_of(&e, v));
        let (of, ol) = (first_of(&e, o), last_of(&e, o));
        assert_eq!(e.call(0x00b2_db10, &args![v, o]).u32(), v);
        assert_eq!(calls(&e, TEXTURES_DESTROY_RANGE), vec![vec![v, vf, vl]]);
        assert_eq!(calls(&e, OPERATOR_DELETE), vec![vec![vf]]);
        assert_eq!(calls(&e, TEXTURES_BUY), vec![vec![v, 3]]);
        assert_eq!(
            calls(&e, TEXTURES_CONSTRUCT_RANGE),
            vec![vec![of, ol, 0x8888_0000, v + 8, 0, 0]]
        );
        assert_eq!(e.mem.u32(v + 0x10), CONSTRUCTED_END);
    }

    #[test]
    fn assigning_a_smaller_texture_vector_assigns_over_the_start_and_destroys_the_rest() {
        let (mut e, _) = setup(&[], &[], &[]);
        let v = texture_vector(&mut e, 3, 3);
        let o = texture_vector(&mut e, 2, 2);
        let (vf, vl) = (first_of(&e, v), last_of(&e, v));
        let (of, ol) = (first_of(&e, o), last_of(&e, o));
        let end = vf + 2 * TEXTURE;
        e.register_double(TEXTURES_ASSIGN_CHECKED, move |_, _| ret(end));
        assert_eq!(e.call(0x00b2_db10, &args![v, o]).u32(), v);
        assert_eq!(
            calls(&e, TEXTURES_ASSIGN_CHECKED),
            vec![vec![of, ol, vf, 0, 0, 0]]
        );
        assert_eq!(calls(&e, TEXTURES_DESTROY_RANGE), vec![vec![v, end, vl]]);
        assert_eq!(e.mem.u32(v + 0x10), end);
    }
}

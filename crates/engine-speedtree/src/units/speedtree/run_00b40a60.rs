//! `SpeedTree/run_00b40a60` (Xbox PDB source unit), subsystem `SpeedTree`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! SpeedTreeRT's leaf level-of-detail engine and the `std` code it uses: the tree-info
//! constructor (`SIdvTreeInfo`), the engine's constructor (`CLeafLodEngine`), its `Compute`
//! (which merges close pairs of leaves into one, a random share of them), the per-leaf pairing
//! search over a `std::vector<bool>`, and the MSVC instantiations of `std::vector<SLodEntry>`
//! (8-byte elements, the leaf and its match) and `std::vector<bool>` (its bit words are a
//! `std::vector<unsigned long>` at `+0x0C`).
//!
//! The PC build's `std::vector` is 0x18 bytes ([`StlVector`]: the checked-iterator proxy at
//! `+0`, the first, last and end pointers at `+0x0C`, `+0x10`, `+0x14`), so `CLeafLodEngine`
//! (Xbox PDB 0x20 bytes) is 0x28 bytes here. A bit-vector iterator is three words (proxy, word
//! address, bit offset); a vector iterator two (proxy, element address). The allocator and the
//! two flag bytes of the exe's `std::vector` code are passed on the stack by the compiler; the
//! flag bytes are left uninitialised in the exe and are passed here as zeros. Functions that
//! carry an `FS:[0]` exception frame (the handlers that unwind a half-built vector are not
//! translated) are marked `Translation incomplete`.

#[allow(unused_imports)]
use crate::prelude::*;

use super::branch::{
    checked_iterator, copy_float, vector_at, StlVector, INVALID_PARAMETER, OPERATOR_DELETE,
};

/// `std::string::_Tidy(this, built, new size)` (`0044a730`; `(0, 0)` makes the string empty).
const STRING_TIDY: u32 = 0x0044_a730;
/// The proxy allocation of the checked-iterator `std::vector` constructors (`0044d670`,
/// `cdecl`, `(1, 0)`) and the container proxy allocation of the bit vector (`0044abd0`,
/// `thiscall`, `RET 4`).
const ALLOCATE_PROXY: u32 = 0x0044_d670;
const ALLOCATE_BOOL_PROXY: u32 = 0x0044_abd0;
/// The length error the `std::vector` code raises for a request above the maximum size (it does
/// not return) and `std::vector<bool>`'s own (`00b429d0`, `leaflod.rs`).
const LENGTH_ERROR: u32 = 0x00b0_fb80;
const BOOL_LENGTH_ERROR: u32 = 0x00b4_29d0;
/// The element allocator of the `SLodEntry` vector (`00b42df0`: count, 0), the construction of
/// elements past the end (`00b42f40`: destination, count, value, the allocator, two flags), the
/// copy of a range (`00b431e0`: first, last, destination, the allocator), the copy up to an end
/// (`00b43010`: first, last, destination end, the allocator) and the move up (`00b42da0`: first,
/// last, end of the destination), all `cdecl` in `run_00b42a60`.
const ENTRY_ALLOCATE: u32 = 0x00b4_2df0;
const ENTRY_CONSTRUCT: u32 = 0x00b4_2f40;
const ENTRY_COPY_RANGE: u32 = 0x00b4_31e0;
const ENTRY_TAIL_COPY: u32 = 0x00b4_3010;
const ENTRY_MOVE_UP: u32 = 0x00b4_2da0;
/// `std::vector<unsigned long>` helpers: the proxy construction (`00b326e0`, `RET 4`), the
/// fill construction (`00b32600`: count, the address of the value; `RET 8`), the tidy
/// (`00b17f50`), the iterator advance (`00b2e330`, `RET 4`) and the checked range erase
/// (`00b335c0`: this, out, first proxy, first, last proxy, last).
const WORDS_CONSTRUCT_PROXY: u32 = 0x00b3_26e0;
const WORDS_CONSTRUCT_FILL: u32 = 0x00b3_2600;
const VECTOR_TIDY: u32 = 0x00b1_7f50;
const WORDS_ITERATOR_ADVANCE: u32 = 0x00b2_e330;
const WORDS_ERASE: u32 = 0x00b3_35c0;
/// The bit-vector iterator helpers of `run_00b42a60`: the advance (`00b42a60`, `RET 4`: a bit
/// count), the reference constructor from an iterator (`00b42be0`, `RET 4`), the base word of a
/// reference (`00b42c60`), the iterator constructor (`00b42980`, `RET 8`: the element address,
/// the container) and the fill (`00b42ed0`: two iterators and the address of the value).
const BIT_ITERATOR_ADVANCE: u32 = 0x00b4_2a60;
const BIT_REFERENCE_CONSTRUCT: u32 = 0x00b4_2be0;
const BIT_REFERENCE_BASE: u32 = 0x00b4_2c60;
const ITERATOR_CONSTRUCT: u32 = 0x00b4_2980;
const BIT_FILL: u32 = 0x00b4_2ed0;
/// The leaf-vector iterator helpers (`run_00b2ab50`): the element address (`00b2ab50`), the
/// increment (`00b2abc0`) and the equality test (`00b2ac30`, `RET 4`, a flag in `AL`).
const ITERATOR_ELEMENT: u32 = 0x00b2_ab50;
const ITERATOR_INCREMENT: u32 = 0x00b2_abc0;
const ITERATORS_EQUAL: u32 = 0x00b2_ac30;
/// The pair vector's own checked increment (`00b41d40`, below; `00b40c80` calls it at
/// `00b40d2c`): one 8-byte `SLodEntry` on, where `00b2abc0` steps 4 bytes.
const PAIR_ITERATOR_INCREMENT: u32 = 0x00b4_1d40;
/// The leaf pointer vector's constructor (`00b17db0`), copy constructor (`00b355d0`, `RET 4`),
/// `operator=` (`00b36330`, `RET 4`) and `push_back` (`00b15390`, in `branch.rs`).
const LEAF_VECTOR_CONSTRUCT: u32 = 0x00b1_7db0;
const LEAF_VECTOR_COPY_CONSTRUCT: u32 = 0x00b3_55d0;
const LEAF_VECTOR_ASSIGN: u32 = 0x00b3_6330;
const LEAF_VECTOR_PUSH: u32 = 0x00b1_5390;
/// `CBillboardLeaf`: the copy of a leaf (`00b403a0`, `thiscall`, `this` only; returns the new
/// leaf), `GetColor` (`00b40430`, `RET 4`: a result slot, three floats) and `SetColor`
/// (`00b404c0`, `RET 8`: three floats and a flag).
const LEAF_CLONE: u32 = 0x00b4_03a0;
const LEAF_GET_COLOR: u32 = 0x00b4_0430;
const LEAF_SET_COLOR: u32 = 0x00b4_04c0;
/// `CVec3::Distance` (`00b04d40`, `thiscall`, `RET 4`; the distance in `ST0`).
const VEC3_DISTANCE: u32 = 0x00b0_4d40;
/// `CIdvRandom::CIdvRandom` (`00b3f100`), `GetUniform` (`00b3f130`: low, high; the result in `ST0`),
/// the function-local static generator of `00b40c80` (`011f8f04`) and its guard word
/// (`011f8f08`), and `_atexit` (`00ec658f`) with the destructor the static registers
/// (`00fdc740`).
const RANDOM_CONSTRUCT: u32 = 0x00b3_f100;
const GET_UNIFORM: u32 = 0x00b3_f130;
const RANDOM_STATIC: u32 = 0x011f_8f04;
const RANDOM_GUARD: u32 = 0x011f_8f08;
const ATEXIT: u32 = 0x00ec_658f;
const RANDOM_DESTRUCTOR: u32 = 0x00fd_c740;
/// Constants the code reads: the floats `[0107d9a0]`, `[01016410]` (100.0) and `[0101b268]` of the
/// tree-info constructor, `[01016248]` (0.5), `[01016970]` (the largest float; the nearest-leaf
/// search starts from it) and the double `[01011588]` of `00b40c80`.
const TREE_FAR_DEFAULT: u32 = 0x0107_d9a0;
const TREE_NEAR_DEFAULT: u32 = 0x0101_6410;
const TREE_SIZE_DEFAULT: u32 = 0x0101_b268;
const HALF_FLOAT: u32 = 0x0101_6248;
const LARGEST_FLOAT: u32 = 0x0101_6970;
const SIZE_SCALE_DOUBLE: u32 = 0x0101_1588;

layout! {
    /// `SIdvTreeInfo` (Xbox PDB), 0x34 bytes (a `CIdvFilename`, the 0x1C-byte string, at `+0`).
    pub struct SIdvTreeInfo: 0x34 {
        /// `m_fFar` (Xbox PDB).
        0x1C m_fFar: f32,
        /// `m_fNear` (Xbox PDB).
        0x20 m_fNear: f32,
        /// `m_nSeed` (Xbox PDB).
        0x24 m_nSeed: i32,
        /// `m_fSize` (Xbox PDB).
        0x28 m_fSize: f32,
        /// `m_fSizeVariance` (Xbox PDB).
        0x2C m_fSizeVariance: f32,
        /// `m_nFlareSeed` (Xbox PDB).
        0x30 m_nFlareSeed: i32,
    }

    /// `CLeafLodEngine` (Xbox PDB), 0x28 bytes on the PC (the vector is 8 bytes larger than the
    /// Xbox's, so the fields after it are 8 higher than in the PDB).
    pub struct CLeafLodEngine: 0x28 {
        /// `m_vPairs` (Xbox PDB): a `std::vector<SLodEntry>` (`m_pLeaf`, `m_pLeafMatch`).
        0x00 m_vPairs: Inline<StlVector>,
        /// `m_fLeafSpacingTolerance` (Xbox PDB `+0x10`).
        0x18 m_fLeafSpacingTolerance: f32,
        /// `m_fLeafReductionPercentage` (Xbox PDB `+0x14`).
        0x1C m_fLeafReductionPercentage: f32,
        /// `m_sLeafInfo` (Xbox PDB `+0x18`): a `const SIdvLeafInfo *`.
        0x20 m_sLeafInfo: Ptr,
        /// `m_fLeafSizeIncreaseFactor` (Xbox PDB `+0x1C`).
        0x24 m_fLeafSizeIncreaseFactor: f32,
    }
}

// Translated from 00b40a60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `SIdvTreeInfo::SIdvTreeInfo` (Xbox PDB layout; `fastcall`, `this` only): makes the texture
/// file name string empty (`0044a730`, `(0, 0)`) and sets `m_fFar` to the float at `[0107d9a0]`,
/// `m_fNear` to the float at `[01016410]`, `m_nSeed` to 1200, `m_fSize` to the float at
/// `[0101b268]`, `m_fSizeVariance` to 0 and `m_nFlareSeed` to 444. Returns `this`.
pub fn fn_00b40a60(e: &mut Engine, this: Ptr<SIdvTreeInfo>) -> Ptr<SIdvTreeInfo> {
    let t = this.addr();
    e.call(STRING_TIDY, &args![t, 0u32, 0u32]);
    copy_float(e, t + 0x1C, TREE_FAR_DEFAULT);
    copy_float(e, t + 0x20, TREE_NEAR_DEFAULT);
    e.set(this, SIdvTreeInfo::m_nSeed, 0x4B0);
    copy_float(e, t + 0x28, TREE_SIZE_DEFAULT);
    e.set(this, SIdvTreeInfo::m_fSizeVariance, 0.0);
    e.set(this, SIdvTreeInfo::m_nFlareSeed, 0x1BC);
    this
}

// Translated from 00b40ad0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CLeafLodEngine::CLeafLodEngine(const SIdvLeafInfo *, float, float, float)` (Xbox PDB layout;
/// `thiscall`, four arguments, `RET 0x10`): constructs the pair vector (`00b416d0`), then stores
/// the spacing tolerance (the first float, `+0x18`), the reduction percentage (the second,
/// `+0x1C`), the leaf info pointer (`+0x20`) and the size increase factor (the third, `+0x24`);
/// the floats go through the x87 stack. Returns `this`.
pub fn fn_00b40ad0(
    e: &mut Engine,
    this: Ptr<CLeafLodEngine>,
    leaf_info: Ptr,
    spacing_tolerance: f32,
    reduction_percentage: f32,
    size_increase_factor: f32,
) -> Ptr<CLeafLodEngine> {
    let t = this.addr();
    fn_00b416d0(e, this.cast());
    e.mem
        .set_u32(t + 0x18, x87_quiet_bits(spacing_tolerance.to_bits()));
    e.mem
        .set_u32(t + 0x1C, x87_quiet_bits(reduction_percentage.to_bits()));
    e.set(this, CLeafLodEngine::m_sLeafInfo, leaf_info);
    e.mem
        .set_u32(t + 0x24, x87_quiet_bits(size_increase_factor.to_bits()));
    this
}

// Translated from 00b40b30 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b40b30 - the FS:[0] exception frame (the handlers that tidy the
// vectors if a call throws) is not translated.
/// `CLeafLodEngine::Compute` (the engine map gives it no name; `thiscall`, `RET 0x1C`: the
/// pointer to the returned `std::vector<CBillboardLeaf *>` and the vector of the leaves passed
/// by value, six words): makes a local leaf vector (`00b17db0`), copy constructs the argument
/// (`00b355d0`) in place in the argument area of `00b41110` (`00b40b78`..`00b40b84`: its proxy
/// names that area) for `00b41110` (which consumes it) to pair up close leaves, runs `00b40c80`
/// to merge the pairs into new leaves and assigns its result to the local vector
/// (`00b36330`); the temporary result is tidied (`00b17f50`) and its proxy deleted; the local
/// vector is copied into the returned vector (`00b355d0`) and tidied and deleted too, as is
/// the by-value argument. Returns the returned vector.
pub fn fn_00b40b30(
    e: &mut Engine,
    this: Ptr<CLeafLodEngine>,
    returned: Ptr<StlVector>,
    leaves: [u32; 6],
) -> Ptr<StlVector> {
    e.with_stack(0x18, |e, argument| {
        let arg = argument.addr();
        for (i, word) in leaves.iter().enumerate() {
            e.mem.set_u32(arg + 4 * i as u32, *word);
        }
        e.with_stack(0x18, |e, local| {
            let local = local.addr();
            e.call(LEAF_VECTOR_CONSTRUCT, &args![local]);
            // 00b40b78: `SUB ESP,0x18; MOV ECX,ESP`: the copy is built in the outgoing argument
            // area, which stays in place as `00b41110`'s own `[EBP+8]` (its `RET 0x18` frees it).
            e.with_stack(0x18, |e, outgoing| {
                e.call(LEAF_VECTOR_COPY_CONSTRUCT, &args![outgoing, arg]); // 00b40b84
                fn_00b41110(e, this, outgoing.cast()); // 00b40b95: the area itself, not a copy
            });
            e.with_stack(0x18, |e, result| {
                let result = result.addr();
                let produced = fn_00b40c80(e, this, Ptr::new(result)).addr();
                e.call(LEAF_VECTOR_ASSIGN, &args![local, produced]);
                e.call(VECTOR_TIDY, &args![result]);
                let proxy = e.mem.u32(result);
                e.call(OPERATOR_DELETE, &args![proxy]);
            });
            e.call(LEAF_VECTOR_COPY_CONSTRUCT, &args![returned, local]);
            e.call(VECTOR_TIDY, &args![local]);
            let proxy = e.mem.u32(local);
            e.call(OPERATOR_DELETE, &args![proxy]);
        });
        e.call(VECTOR_TIDY, &args![arg]);
        let proxy = e.mem.u32(arg);
        e.call(OPERATOR_DELETE, &args![proxy]);
    });
    returned
}

// Translated from 00b40c80 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b40c80 - the FS:[0] exception frame (the handlers that tidy the
// result vector and the static's registration if a call throws) is not translated.
/// Merges the engine's leaf pairs (the engine map gives it no name; `thiscall`, `RET 4`: the
/// pointer to the returned `std::vector<CBillboardLeaf *>`): the first time, constructs the
/// function-local `CIdvRandom` (`00b3f100` on `011f8f04`, guard bit 0 of `[011f8f08]`) and
/// registers its destructor (`_atexit`). With an empty local result vector (`00b17db0`), for
/// each pair of `m_vPairs` (the 8-byte `SLodEntry`s, iterated with the checked iterator helpers
/// `00b42980`, `00b2ac30`, `00b2ab50`, `00b41d40`) where a uniform random number (`00b3f130`,
/// 0 to 1) exceeds the reduction percentage (`+0x1C`): the leaf is copied (`00b403a0`), its
/// position set to half the sum of the pair's two positions (`0.5 * (a + b)`, per axis) with `z`
/// lifted by `(origin.y - 0.5) * (size factor * size.y * 0.5)` of the leaf's texture (the
/// texture byte `/ 2` indexes the leaf info's texture vector; range-checked) — with the double
/// at `[01011588]` as the 0.5 — and its colour set to half the sum of the pair's colours
/// (`00b40430`, `00b404c0`); it is appended to the result (`00b15390`). The result is copied
/// into the returned vector (`00b355d0`) and the local vector tidied and deleted. Returns the
/// returned vector.
pub fn fn_00b40c80(
    e: &mut Engine,
    this: Ptr<CLeafLodEngine>,
    returned: Ptr<StlVector>,
) -> Ptr<StlVector> {
    let t = this.addr();
    if e.mem.u32(RANDOM_GUARD) & 1 == 0 {
        e.mem.set_u32(RANDOM_GUARD, e.mem.u32(RANDOM_GUARD) | 1);
        e.call(RANDOM_CONSTRUCT, &args![RANDOM_STATIC]);
        e.call(ATEXIT, &args![RANDOM_DESTRUCTOR]);
    }
    e.with_stack(0x18 + 8 + 8 + 8, |e, frame| {
        let local = frame.addr();
        let it_a = local + 0x18;
        let it_b = it_a + 8;
        let new_leaf_slot = it_b + 8;
        e.call(LEAF_VECTOR_CONSTRUCT, &args![local]);
        let first = e.mem.u32(t + 0x0C);
        e.call(ITERATOR_CONSTRUCT, &args![it_a, first, this]);
        loop {
            let last = e.mem.u32(t + 0x10);
            e.call(ITERATOR_CONSTRUCT, &args![it_b, last, this]);
            if e.call(ITERATORS_EQUAL, &args![it_a, it_b]).bool() {
                break;
            }
            let random = e
                .call(GET_UNIFORM, &args![RANDOM_STATIC, 0.0f32, 1.0f32])
                .f64();
            if x87_load32(e.mem.u32(t + 0x1C)) < random {
                merge_pair(e, this, it_a, new_leaf_slot, local);
            }
            e.call(PAIR_ITERATOR_INCREMENT, &args![it_a]); // 00b40d2c: one 8-byte pair on
        }
        e.call(LEAF_VECTOR_COPY_CONSTRUCT, &args![returned, local]);
        e.call(VECTOR_TIDY, &args![local]);
        let proxy = e.mem.u32(local);
        e.call(OPERATOR_DELETE, &args![proxy]);
    });
    returned
}

/// The body of `00b40c80`'s loop for the pair at the iterator `it`: builds the merged leaf in
/// the slot `leaf_slot` and appends it to the vector at `result`.
fn merge_pair(e: &mut Engine, this: Ptr<CLeafLodEngine>, it: u32, leaf_slot: u32, result: u32) {
    let t = this.addr();
    let entry = e.call(ITERATOR_ELEMENT, &args![it]).u32();
    let leaf = e.mem.u32(entry);
    let new_leaf = e.call(LEAF_CLONE, &args![leaf]).u32();
    e.mem.set_u32(leaf_slot, new_leaf);
    let entry = e.call(ITERATOR_ELEMENT, &args![it]).u32();
    let match_position = e.mem.u32(entry + 4).wrapping_add(4);
    let entry = e.call(ITERATOR_ELEMENT, &args![it]).u32();
    let leaf_position = e.mem.u32(entry).wrapping_add(4);
    // `half * (a + b)` per axis, each sum and product stored as a float.
    let mut middle = [0u32; 3];
    for (axis, slot) in middle.iter_mut().enumerate() {
        let offset = 4 * axis as u32;
        let sum = x87_store32(x87_add(
            x87_load32(e.mem.u32(match_position + offset)),
            x87_m32(e.mem.u32(leaf_position + offset)),
        ));
        let sum = x87_quiet_bits(sum);
        let half = x87_load32(e.mem.u32(HALF_FLOAT));
        *slot = x87_store32(x87_mul(half, x87_m32(sum)));
    }
    let middle = middle.map(x87_quiet_bits);
    let entry = e.call(ITERATOR_ELEMENT, &args![it]).u32();
    let texture = u32::from(e.mem.u8(e.mem.u32(entry) + 0x40));
    let index = (texture as i32 / 2) as u32;
    let info = e.mem.u32(t + 0x20);
    let texture_at = vector_at(e, info + 0x10, index, 0x54);
    let scale: f64 = e.global(SIZE_SCALE_DOUBLE);
    let origin = x87_sub(x87_load32(e.mem.u32(texture_at + 0x34)), scale);
    let factor = x87_mul(
        x87_mul(
            x87_load32(e.mem.u32(t + 0x24)),
            x87_m32(e.mem.u32(texture_at + 0x4C)),
        ),
        scale,
    );
    let lift = x87_store32(x87_mul(origin, factor));
    let lifted = x87_store32(x87_add(x87_load32(middle[2]), x87_m32(lift)));
    e.mem.set_u32(new_leaf + 4, middle[0]);
    e.mem.set_u32(new_leaf + 8, middle[1]);
    e.mem.set_u32(new_leaf + 12, lifted);
    let first_entry = e.call(ITERATOR_ELEMENT, &args![it]).u32();
    let second_entry = e.call(ITERATOR_ELEMENT, &args![it]).u32();
    let colors = e.with_stack(0x18, |e, slots| {
        let first_slot = slots.addr();
        let second_slot = first_slot + 0xC;
        let match_leaf = e.mem.u32(first_entry + 4);
        let first_color = e.call(LEAF_GET_COLOR, &args![match_leaf, first_slot]).u32();
        let leaf = e.mem.u32(second_entry);
        let second_color = e.call(LEAF_GET_COLOR, &args![leaf, second_slot]).u32();
        let mut sums = [0u32; 3];
        for (axis, slot) in sums.iter_mut().enumerate() {
            let offset = 4 * axis as u32;
            *slot = x87_store32(x87_add(
                x87_load32(e.mem.u32(first_color + offset)),
                x87_m32(e.mem.u32(second_color + offset)),
            ));
        }
        sums
    });
    let mut halves = [0u32; 3];
    for (axis, slot) in halves.iter_mut().enumerate() {
        let sum = x87_quiet_bits(colors[axis]);
        let half = x87_load32(e.mem.u32(HALF_FLOAT));
        *slot = x87_quiet_bits(x87_store32(x87_mul(half, x87_m32(sum))));
    }
    e.with_stack(0xC, |e, color| {
        for (axis, word) in halves.iter().enumerate() {
            e.mem.set_u32(color.addr() + 4 * axis as u32, *word);
        }
        e.call(LEAF_SET_COLOR, &args![new_leaf, color, 0u32]);
    });
    e.call(LEAF_VECTOR_PUSH, &args![result, leaf_slot]);
}

// Translated from 00b41110 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b41110 - the FS:[0] exception frame (the handlers that tidy the
// bit vector and the argument vector if a call throws) is not translated.
/// Finds, for every leaf, its nearest unpaired later leaf (the engine map gives it no name;
/// `thiscall`, `RET 0x18`: the vector of `CBillboardLeaf *` passed by value, six words that
/// the function destroys): a `std::vector<bool>` with one bit per leaf is made (`00b418b0`) and
/// cleared with `std::fill` (`00b42ed0`). For each leaf `a` (its own bit is not tested), for each later
/// leaf `b` (its index counted with `00b420a0` and advanced with `00b2e330`): when `b`'s bit is
/// set the search stops; otherwise the distance (`00b04d40`) between the two leaves' positions
/// (`+4`) is kept if it is below the spacing tolerance (`+0x18`) and below the best so far (a
/// float starting at `[01016970]`). When a match was found its bit is set (`00b41ae0` on a bit
/// reference made with `00b42a60` and `00b42be0`) and the pair is appended to `m_vPairs`
/// (`00b417b0`). The bit vector is destroyed (`00b419c0`) and the argument vector tidied
/// (`00b17f50`) and its proxy deleted.
///
/// `vector` is the address of the argument area that holds the by-value vector (the exe's
/// `[EBP+8]`): the caller built the copy there (`00b40b78`..`00b40b84`), so its proxy names
/// that area, and the checked iterators made on it (`00b42980` with `LEA ECX,[EBP+8]`) find
/// their container through that proxy (`00b2ab50`, `00b2e330`).
pub fn fn_00b41110(e: &mut Engine, this: Ptr<CLeafLodEngine>, vector: Ptr<StlVector>) {
    let t = this.addr();
    {
        let v = vector.addr(); // 00b41145: the vector is `[EBP+8]` itself, worked on in place
        let count = (e.mem.u32(v + 0x10).wrapping_sub(e.mem.u32(v + 0x0C)) as i32 >> 2) as u32;
        e.with_stack(0x24 + 0x60, |e, frame| {
            let bits = frame.addr();
            // Scratch objects: iterators and references of three words, the pair, two vector
            // iterators of two words, the flag and the bit-vector helpers' temporaries.
            let advanced = bits + 0x24;
            let fresh = advanced + 0xC;
            let second = fresh + 0xC;
            let reference = second + 0xC;
            let pair = reference + 0xC;
            let it_a = pair + 8;
            let it_b = it_a + 8;
            let it_end = it_b + 8;
            let flag = it_end + 8;
            fn_00b418b0(e, Ptr::new(bits), count, 0);
            e.mem.set_u8(flag, 0);
            fn_00b41a50(e, Ptr::new(bits), Ptr::new(advanced));
            let size = e.mem.u32(bits + 8);
            if size != 0 {
                e.call(BIT_ITERATOR_ADVANCE, &args![advanced, size]);
            }
            let end_words = [
                e.mem.u32(advanced),
                e.mem.u32(advanced + 4),
                e.mem.u32(advanced + 8),
            ];
            let first_iterator = fn_00b41a50(e, Ptr::new(bits), Ptr::new(fresh)).addr();
            let begin_words = [
                e.mem.u32(first_iterator),
                e.mem.u32(first_iterator + 4),
                e.mem.u32(first_iterator + 8),
            ];
            e.call(
                BIT_FILL,
                &args![
                    begin_words[0],
                    begin_words[1],
                    begin_words[2],
                    end_words[0],
                    end_words[1],
                    end_words[2],
                    flag
                ],
            );
            let first = e.mem.u32(v + 0x0C);
            e.call(ITERATOR_CONSTRUCT, &args![it_a, first, v]);
            loop {
                let last = e.mem.u32(v + 0x10);
                e.call(ITERATOR_CONSTRUCT, &args![it_end, last, v]);
                if e.call(ITERATORS_EQUAL, &args![it_a, it_end]).bool() {
                    break;
                }
                let entry = e.call(ITERATOR_ELEMENT, &args![it_a]).u32();
                let leaf_a = e.mem.u32(entry);
                e.mem.set_u32(pair, leaf_a);
                e.mem.set_u32(pair + 4, 0);
                let mut best = x87_quiet_bits(e.mem.u32(LARGEST_FLOAT));
                let mut best_index = -1i32;
                let first = e.mem.u32(v + 0x0C);
                e.call(ITERATOR_CONSTRUCT, &args![second, first, v]);
                let mut index = fn_00b420a0(e, Ptr::new(it_a), Ptr::new(second)).wrapping_add(1);
                // The second iterator starts one past the first.
                let (proxy, position) = (e.mem.u32(it_a), e.mem.u32(it_a + 4));
                e.mem.set_u32(it_b, proxy);
                e.mem.set_u32(it_b + 4, position);
                e.call(WORDS_ITERATOR_ADVANCE, &args![it_b, 1u32]);
                loop {
                    let last = e.mem.u32(v + 0x10);
                    e.call(ITERATOR_CONSTRUCT, &args![it_end, last, v]);
                    if e.call(ITERATORS_EQUAL, &args![it_b, it_end]).bool() {
                        break;
                    }
                    // The bit of the candidate: stops the search when set.
                    if bit_at(e, bits, advanced, reference, index as u32) {
                        break;
                    }
                    if !bit_at(e, bits, advanced, reference, index as u32) {
                        let candidate = e.call(ITERATOR_ELEMENT, &args![it_b]).u32();
                        let candidate_position = e.mem.u32(candidate).wrapping_add(4);
                        let entry = e.call(ITERATOR_ELEMENT, &args![it_a]).u32();
                        let leaf_position = e.mem.u32(entry).wrapping_add(4);
                        let distance = e
                            .call(VEC3_DISTANCE, &args![leaf_position, candidate_position])
                            .f64();
                        let distance = x87_store32(distance);
                        let tolerance = x87_load32(e.mem.u32(t + 0x18));
                        if x87_load32(distance) < tolerance
                            && x87_load32(distance) < x87_load32(best)
                        {
                            best = x87_quiet_bits(distance);
                            let candidate = e.call(ITERATOR_ELEMENT, &args![it_b]).u32();
                            let matched = e.mem.u32(candidate);
                            e.mem.set_u32(pair + 4, matched);
                            best_index = index;
                        }
                    }
                    index = index.wrapping_add(1);
                    e.call(ITERATOR_INCREMENT, &args![it_b]);
                }
                if e.mem.u32(pair + 4) != 0 {
                    set_bit_at(e, bits, advanced, reference, best_index as u32);
                    fn_00b417b0(e, this.cast(), Ptr::new(pair));
                }
                e.call(ITERATOR_INCREMENT, &args![it_a]);
            }
            fn_00b419c0(e, Ptr::new(bits));
        });
        e.call(VECTOR_TIDY, &args![v]);
        let proxy = e.mem.u32(v);
        e.call(OPERATOR_DELETE, &args![proxy]);
    } // 00b416c9: `RET 0x18`, the argument area goes with the caller's frame
}

/// Makes the bit reference of bit `index` of the bit vector at `bits` the way `00b41110` does:
/// the begin iterator (`00b41a50`) is copied, advanced by `index` (`00b42a60`) and given to the
/// reference constructor (`00b42be0`). `iterator` and `reference` are scratch objects of three
/// words; returns the reference's bit offset word.
fn bit_reference(e: &mut Engine, bits: u32, iterator: u32, reference: u32, index: u32) -> u32 {
    let begin = fn_00b41a50(e, Ptr::new(bits), Ptr::new(iterator)).addr();
    let words = [e.mem.u32(begin), e.mem.u32(begin + 4), e.mem.u32(begin + 8)];
    for (i, word) in words.iter().enumerate() {
        e.mem.set_u32(iterator + 4 * i as u32, *word);
    }
    e.call(BIT_ITERATOR_ADVANCE, &args![iterator, index]);
    e.call(BIT_REFERENCE_CONSTRUCT, &args![reference, iterator]);
    e.mem.u32(reference + 8)
}

/// Whether bit `index` is set, as `00b41110` tests it: `1 << (offset & 31)` against the word
/// the reference gives (`00b420f0`).
fn bit_at(e: &mut Engine, bits: u32, iterator: u32, reference: u32, index: u32) -> bool {
    let offset = bit_reference(e, bits, iterator, reference, index);
    let mask = 1u32 << (offset & 31);
    let word = fn_00b420f0(e, Ptr::new(reference));
    e.mem.u32(word) & mask != 0
}

/// Sets bit `index` (`00b41ae0` with `true` on its reference).
fn set_bit_at(e: &mut Engine, bits: u32, iterator: u32, reference: u32, index: u32) {
    bit_reference(e, bits, iterator, reference, index);
    fn_00b41ae0(e, Ptr::new(reference), 1);
}

// Translated from 00b416d0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b416d0 - the FS:[0] exception frame is not translated.
/// The default constructor of the pair vector (a `std::vector<SLodEntry>`; the engine map
/// gives it no name; `fastcall`, `this` only): allocates the 8-byte proxy (`0044d670`,
/// `(1, 0)`), stores `this` in its first word when it was allocated, stores the proxy (or zero) in
/// the vector's first word, and buys room for no elements (`00b41c80`). Returns `this`.
pub fn fn_00b416d0(e: &mut Engine, this: Ptr<StlVector>) -> Ptr<StlVector> {
    let proxy = e.call(ALLOCATE_PROXY, &args![1u32, 0u32]).u32();
    let stored = if proxy == 0 {
        0
    } else {
        e.mem.set_u32(proxy, this.addr());
        proxy
    };
    e.mem.set_u32(this.addr(), stored);
    fn_00b41c80(e, this, 0);
    this
}

// Translated from 00b417b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `push_back` of the pair vector (the engine map gives it no name; `thiscall`, one argument,
/// `RET 4`): with spare capacity (the sizes in 8-byte elements, shifted arithmetically) the
/// element at `value` is constructed at the end (`00b42f40`; two of the words it takes are bytes
/// left uninitialised on the exe's stack and are passed as zero) and the end advanced by 8;
/// otherwise the checked insert `00b41b50` at the end is used.
pub fn fn_00b417b0(e: &mut Engine, this: Ptr<StlVector>, value: Ptr) {
    let base = this.addr();
    let size = (e.mem.u32(base + 0x10).wrapping_sub(e.mem.u32(base + 0x0C)) as i32 >> 3) as u32;
    let capacity = entry_capacity(e, base);
    if size < capacity {
        let last = e.mem.u32(base + 0x10);
        e.call(
            ENTRY_CONSTRUCT,
            &args![last, 1u32, value, base.wrapping_add(8), 0u32, 0u32],
        );
        e.mem.set_u32(base + 0x10, 8u32.wrapping_add(last));
    } else {
        let last = e.mem.u32(base + 0x10);
        let proxy = checked_iterator(e, base, last);
        e.with_stack(8, |e, out| {
            fn_00b41b50(e, this, out, proxy, last, value);
        });
    }
}

/// The capacity of the 8-byte-element vector at `vector`: zero without storage, else
/// `(end - first) >> 3` (arithmetic shift).
fn entry_capacity(e: &Engine, vector: u32) -> u32 {
    if e.mem.u32(vector + 0x0C) == 0 {
        0
    } else {
        (e.mem
            .u32(vector + 0x14)
            .wrapping_sub(e.mem.u32(vector + 0x0C)) as i32
            >> 3) as u32
    }
}

// Translated from 00b418b0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b418b0 - the FS:[0] exception frame is not translated.
/// `std::vector<bool>::vector(size_type n, bool value)` (the engine map gives it no name;
/// `thiscall`, two arguments, `RET 8`; the value is a byte): allocates the container proxy
/// (`0044abd0` on `this + 4`, argument 1), stores `this` in its first word when it was allocated
/// and the proxy (or zero) in the vector's first word, clears the bit count (`+8`), builds the
/// bit words (`+0x0C`, `(n + 31) >> 5` words, all ones for `true`; `00b42030`) and sets the size
/// (`00b41db0`). Returns `this`.
pub fn fn_00b418b0(e: &mut Engine, this: Ptr, n: u32, value: u8) -> Ptr {
    let t = this.addr();
    let proxy = e.call(ALLOCATE_BOOL_PROXY, &args![t + 4, 1u32]).u32();
    let stored = if proxy == 0 {
        0
    } else {
        e.mem.set_u32(proxy, t);
        proxy
    };
    e.mem.set_u32(t, stored);
    e.mem.set_u32(t + 8, 0);
    // `NEG` of the byte and `SBB`: all ones when the value is not zero.
    let fill = if value != 0 { u32::MAX } else { 0 };
    e.with_stack(4, |e, word| {
        e.mem.set_u32(word.addr(), fill);
        fn_00b42030(
            e,
            Ptr::new(t + 0x0C),
            n.wrapping_add(0x1F) >> 5,
            word.addr(),
        );
    });
    fn_00b41db0(e, this, n);
    this
}

// Translated from 00b419c0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b419c0 - the FS:[0] exception frame is not translated.
/// The destructor of the bit vector (`fastcall`, `this` only): clears the bit count (`+8`),
/// tidies the bit words (`00b17f50` on `this + 0xC`), deletes their proxy (the word at `+0xC`) and
/// then the container proxy (the word at `+0`).
pub fn fn_00b419c0(e: &mut Engine, this: Ptr) {
    let t = this.addr();
    e.mem.set_u32(t + 8, 0);
    e.call(VECTOR_TIDY, &args![t + 0x0C]);
    let words_proxy = e.mem.u32(t + 0x0C);
    e.call(OPERATOR_DELETE, &args![words_proxy]);
    let proxy = e.mem.u32(t);
    e.call(OPERATOR_DELETE, &args![proxy]);
}

// Translated from 00b41a50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `std::vector<bool>::begin` (the engine map gives it no name; `thiscall`, one argument,
/// `RET 4`: where to write the iterator): the iterator at the first bit word (`+0x18`, the word
/// vector's `_Myfirst`, with the range checks of the checked iterator on the vector at `+0xC`)
/// is built, and `out` is set to (the container proxy, the first word's address, bit 0); a
/// null `this` takes the invalid-parameter path (`00ec7c56`). Returns `out`.
pub fn fn_00b41a50(e: &mut Engine, this: Ptr, out: Ptr) -> Ptr {
    let t = this.addr();
    let words = t.wrapping_add(0x0C);
    let first = e.mem.u32(words + 0x0C);
    checked_iterator(e, words, first);
    e.mem.set_u32(out.addr(), 0);
    e.mem.set_u32(out.addr() + 4, first);
    e.mem.set_u32(out.addr() + 8, 0);
    if t == 0 {
        e.call(INVALID_PARAMETER, &args![]);
    }
    let proxy = e.mem.u32(t);
    e.mem.set_u32(out.addr(), proxy);
    out
}

// Translated from 00b41ae0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `std::vector<bool>::reference::operator=(bool)` (the engine map gives it no name; `thiscall`,
/// one argument, `RET 4`; `this` is the reference: a proxy, a word address and a bit offset
/// at `+8`): sets the bit (`1 << (offset & 31)`, ORed into the word that `00b420f0` finds) for
/// a nonzero value, else clears it. Returns `this`.
pub fn fn_00b41ae0(e: &mut Engine, this: Ptr, value: u8) -> Ptr {
    let word = fn_00b420f0(e, this);
    let mask = 1u32 << (e.mem.u32(this.addr() + 8) & 31);
    let current = e.mem.u32(word);
    if value != 0 {
        e.mem.set_u32(word, mask | current);
    } else {
        e.mem.set_u32(word, !mask & current);
    }
    this
}

// Translated from 00b41b50 (decompiled, FalloutNV.exe 1.4.0.525)
/// The checked insert of one pair before an iterator (the engine map gives it no name;
/// `thiscall`, four arguments, `RET 0x10`: where to write the iterator, the iterator's proxy and
/// position, and the pair to insert): with elements present, the iterator at the first element
/// is checked and the position must belong to this vector (its proxy nonzero and equal to the
/// vector's); the offset of the position in 8-byte elements is kept (0 for an empty vector).
/// `00b42180` inserts one copy; then the iterator at the new first element is advanced by the
/// offset (`00b42ce0`) and written to `out` (proxy, position). Returns `out`.
pub fn fn_00b41b50(
    e: &mut Engine,
    this: Ptr<StlVector>,
    out: Ptr,
    proxy: u32,
    position: u32,
    value: Ptr,
) -> Ptr {
    let t = this.addr();
    let offset = if (e.mem.u32(t + 0x10).wrapping_sub(e.mem.u32(t + 0x0C)) as i32 >> 3) == 0 {
        0
    } else {
        let first = e.mem.u32(t + 0x0C);
        let first_proxy = checked_iterator(e, t, first);
        if proxy == 0 || proxy != first_proxy {
            e.call(INVALID_PARAMETER, &args![]);
        }
        (position.wrapping_sub(first) as i32 >> 3) as u32
    };
    fn_00b42180(e, this, proxy, position, 1, value);
    let first = e.mem.u32(t + 0x0C);
    let first_proxy = checked_iterator(e, t, first);
    e.with_stack(8, |e, iterator| {
        e.mem.set_u32(iterator.addr(), first_proxy);
        e.mem.set_u32(iterator.addr() + 4, first);
        e.call(ENTRY_ITERATOR_ADVANCE, &args![iterator, offset]);
        let (proxy_out, position_out) =
            (e.mem.u32(iterator.addr()), e.mem.u32(iterator.addr() + 4));
        e.mem.set_u32(out.addr(), proxy_out);
        e.mem.set_u32(out.addr() + 4, position_out);
    });
    out
}

/// The checked iterator advance of the pair vector (`00b42ce0`, `thiscall`, `RET 4`).
const ENTRY_ITERATOR_ADVANCE: u32 = 0x00b4_2ce0;

// Translated from 00b41c80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `_Buy(count)` of the pair vector (the engine map gives it no name; `thiscall`, one argument,
/// `RET 4`; the result in `AL`): clears the first, last and end pointers; a count of zero
/// buys nothing (false); above `0x1FFFFFFF` raises the length error (`00b0fb80`); otherwise
/// allocates (`00b42df0`) room for `count` 8-byte pairs, sets last to first and end past the
/// room. True when it did not buy zero.
pub fn fn_00b41c80(e: &mut Engine, this: Ptr<StlVector>, count: u32) -> bool {
    let t = this.addr();
    e.mem.set_u32(t + 0x0C, 0);
    e.mem.set_u32(t + 0x10, 0);
    e.mem.set_u32(t + 0x14, 0);
    if count == 0 {
        return false;
    }
    // The maximum size, tested for zero by the exe (the one never used).
    let limit = 0x1fff_ffffu32;
    let limit = if limit > 0 { limit } else { 1 };
    if limit < count {
        e.call(LENGTH_ERROR, &args![]);
    } else {
        let storage = e.call(ENTRY_ALLOCATE, &args![count, 0u32]).u32();
        e.mem.set_u32(t + 0x0C, storage);
        let first = e.mem.u32(t + 0x0C);
        e.mem.set_u32(t + 0x10, first);
        e.mem.set_u32(
            t + 0x14,
            e.mem.u32(t + 0x0C).wrapping_add(count.wrapping_mul(8)),
        );
    }
    true
}

// Translated from 00b41d40 (decompiled, FalloutNV.exe 1.4.0.525)
/// The checked increment of a pair-vector iterator (the engine map gives it no name; `fastcall`,
/// `this` only; the iterator is a proxy and a position): the proxy must be nonzero (else the
/// invalid-parameter path, `00ec7c56`) and the position must be below the owner vector's last
/// pointer (the owner is the word the proxy points to); then it advances by 8. Returns `this`.
pub fn fn_00b41d40(e: &mut Engine, this: Ptr) -> Ptr {
    let t = this.addr();
    if e.mem.u32(t) == 0 {
        e.call(INVALID_PARAMETER, &args![]);
    }
    let owner = if e.mem.u32(t) == 0 {
        0
    } else {
        e.mem.u32(e.mem.u32(t))
    };
    if e.mem.u32(t + 4) >= e.mem.u32(owner + 0x10) {
        e.call(INVALID_PARAMETER, &args![]);
    }
    let next = e.mem.u32(t + 4).wrapping_add(8);
    e.mem.set_u32(t + 4, next);
    this
}

// Translated from 00b41db0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the size of a bit vector in bits and trims the unused bits (the engine map gives it no
/// name; `thiscall`, one argument, `RET 4`): the largest size is `0x3FFFFFFF` words of 32 bits
/// (`>= 0x7FFFFFF` makes it `0xFFFFFFFF`); a larger request calls the bit vector's length error
/// (`00b429d0`; never reached). When the word vector (`+0xC`) holds more than `(n + 31) >> 5`
/// words, the surplus is erased (`00b335c0` from the iterator advanced by that many words, with the
/// range checks); the bit count (`+8`) is set; and with `n % 32` bits in the last word, the
/// bits above them are masked off. Returns `n / 32` when `n % 32` is zero, else the address
/// of the word that was masked.
pub fn fn_00b41db0(e: &mut Engine, this: Ptr, n: u32) -> u32 {
    let t = this.addr();
    // The exe tests the constant maximum against zero and against `0x07FFFFFF`; the other
    // branches never run.
    let limit = 0x3fff_ffffu32;
    let limit = if limit > 0 { limit } else { 1 };
    let max_bits = if limit < 0x07ff_ffff {
        limit << 5
    } else {
        0xffff_ffff
    };
    if max_bits < n {
        e.call(BOOL_LENGTH_ERROR, &args![this]);
    }
    let words = n.wrapping_add(0x1F) >> 5;
    let vector = t + 0x0C;
    let held = (e
        .mem
        .u32(vector + 0x10)
        .wrapping_sub(e.mem.u32(vector + 0x0C)) as i32
        >> 2) as u32;
    if words < held {
        let last = e.mem.u32(vector + 0x10);
        let last_proxy = checked_iterator(e, vector, last);
        let first = e.mem.u32(vector + 0x0C);
        let first_proxy = checked_iterator(e, vector, first);
        let (advanced_proxy, advanced) = e.with_stack(8, |e, iterator| {
            e.mem.set_u32(iterator.addr(), first_proxy);
            e.mem.set_u32(iterator.addr() + 4, first);
            e.call(WORDS_ITERATOR_ADVANCE, &args![iterator, words]);
            (e.mem.u32(iterator.addr()), e.mem.u32(iterator.addr() + 4))
        });
        e.with_stack(8, |e, out| {
            e.call(
                WORDS_ERASE,
                &args![vector, out, advanced_proxy, advanced, last_proxy, last],
            );
        });
    }
    e.mem.set_u32(t + 8, n);
    let remainder = n % 0x20;
    if remainder == 0 {
        return n / 0x20;
    }
    let index = words.wrapping_sub(1);
    let word = vector_at(e, vector, index, 4);
    let mask = (1u32 << (remainder & 31)).wrapping_sub(1);
    let current = e.mem.u32(word);
    e.mem.set_u32(word, mask & current);
    word
}

// Translated from 00b42030 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b42030 - the FS:[0] exception frame is not translated.
/// The constructor of a `std::vector<unsigned long>` filled with copies of a word (the engine
/// map gives it no name; `thiscall`, two arguments, `RET 8`: the count and the address of the
/// value): builds the proxy (`00b326e0`, with a dummy stack word) and fills the vector
/// (`00b32600`). Returns `this`.
pub fn fn_00b42030(e: &mut Engine, this: Ptr, count: u32, value: u32) -> Ptr {
    e.call(WORDS_CONSTRUCT_PROXY, &args![this, 0u32]);
    e.call(WORDS_CONSTRUCT_FILL, &args![this, count, value]);
    this
}

// Translated from 00b420a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The difference of two word-vector iterators (the engine map gives it no name; `thiscall`,
/// one argument, `RET 4`; each is a proxy and an address): both need the same nonzero proxy (else
/// the invalid-parameter path, `00ec7c56`); the result is `(this.position - other.position) >> 2`
/// (arithmetic).
pub fn fn_00b420a0(e: &mut Engine, this: Ptr, other: Ptr) -> i32 {
    let (a, b) = (this.addr(), other.addr());
    if e.mem.u32(a) == 0 || e.mem.u32(a) != e.mem.u32(b) {
        e.call(INVALID_PARAMETER, &args![]);
    }
    e.mem.u32(a + 4).wrapping_sub(e.mem.u32(b + 4)) as i32 >> 2
}

// Translated from 00b420f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The word of a bit reference (the engine map gives it no name; `fastcall`, `this` only; the
/// reference is a proxy, a word address and a bit offset at `+8`): the proxy and the address must
/// be nonzero (else the invalid-parameter path, `00ec7c56`); the bit position is the offset plus
/// 32 bits for each word between the address and the base word of the reference (`00b42c60`),
/// and it must be below the bit count (`+8`) of the vector the proxy belongs to (else the same
/// path). Returns the word address.
pub fn fn_00b420f0(e: &mut Engine, this: Ptr) -> u32 {
    let t = this.addr();
    if e.mem.u32(t) == 0 || e.mem.u32(t + 4) == 0 {
        e.call(INVALID_PARAMETER, &args![]);
    }
    let offset = e.mem.u32(t + 8);
    let base = e.call(BIT_REFERENCE_BASE, &args![this]).u32();
    let position =
        ((e.mem.u32(t + 4).wrapping_sub(base) as i32 >> 2) << 5).wrapping_add(offset as i32) as u32;
    let owner = if e.mem.u32(t) == 0 {
        0
    } else {
        e.mem.u32(e.mem.u32(t))
    };
    if position >= e.mem.u32(owner + 8) {
        e.call(INVALID_PARAMETER, &args![]);
    }
    e.mem.u32(t + 4)
}

/// The `_Insert_n` prologue for the pair vector (8-byte elements): the same decisions as the
/// other instances, in shifted sizes.
enum Room {
    Nothing,
    TooLong,
    Grow(u32),
    Fits,
}

fn insert_room(e: &Engine, vector: u32, limit: u32, count: u32) -> Room {
    // The exe tests `limit > 0` before using the limit (a constant, so the 1 is never used).
    let limit = limit.max(1);
    let capacity = entry_capacity(e, vector);
    if count == 0 {
        return Room::Nothing;
    }
    let len = (e
        .mem
        .u32(vector + 0x10)
        .wrapping_sub(e.mem.u32(vector + 0x0C)) as i32
        >> 3) as u32;
    if limit.wrapping_sub(len) < count {
        return Room::TooLong;
    }
    let required = len.wrapping_add(count);
    if capacity >= required {
        return Room::Fits;
    }
    let mut grown = if limit.wrapping_sub(capacity >> 1) < capacity {
        0
    } else {
        (capacity >> 1).wrapping_add(capacity)
    };
    if grown < required {
        grown = required;
    }
    Room::Grow(grown)
}

// Translated from 00b42180 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b42180 - the FS:[0] exception frame (the handler that frees the new
// storage if a copy throws) is not translated.
/// `_Insert_n` of the pair vector (8-byte elements, two words; the engine map gives it no name;
/// `thiscall`, four arguments, the first, a proxy, is not read): inserts `count` copies of the
/// pair at `value` before `position`. A count of zero does nothing; a result above the maximum
/// size (`0x1FFFFFFF` elements) raises the length error (`00b0fb80`). Without room the vector
/// grows (`00b42df0`): the elements before the position are copied to the new storage
/// (`00b431e0`), the copies of the value made (`00b42f40`) and the later elements copied after
/// them, the old block freed. With room, the value is first copied to a local (two words);
/// when fewer elements follow the position than are inserted the tail moves up (`00b431e0`),
/// copies of the value are made past the old end and the range is overwritten with the value;
/// otherwise the last `count` elements are copied past the end (`00b43010`), the others moved
/// up (`00b42da0`) and the range overwritten.
pub fn fn_00b42180(
    e: &mut Engine,
    this: Ptr<StlVector>,
    _unused_1: u32,
    position: u32,
    count: u32,
    value: Ptr,
) {
    let t = this.addr();
    match insert_room(e, t, 0x1fff_ffff, count) {
        Room::Nothing => {}
        Room::TooLong => {
            e.call(LENGTH_ERROR, &args![]);
        }
        Room::Grow(capacity) => {
            let storage = e.call(ENTRY_ALLOCATE, &args![capacity, 0u32]).u32();
            let first = e.mem.u32(t + 0x0C);
            let copied = e
                .call(ENTRY_COPY_RANGE, &args![first, position, storage, t + 8])
                .u32();
            e.call(
                ENTRY_CONSTRUCT,
                &args![copied, count, value, t + 8, 0u32, 0u32],
            );
            let filled = copied.wrapping_add(count.wrapping_mul(8));
            let last = e.mem.u32(t + 0x10);
            e.call(ENTRY_COPY_RANGE, &args![position, last, filled, t + 8]);
            let len = (e.mem.u32(t + 0x10).wrapping_sub(e.mem.u32(t + 0x0C)) as i32 >> 3) as u32;
            let new_len = len.wrapping_add(count);
            if e.mem.u32(t + 0x0C) != 0 {
                // The loop that destroys the old elements has nothing to run.
                let mut element = e.mem.u32(t + 0x0C);
                let last = e.mem.u32(t + 0x10);
                while element != last {
                    element = element.wrapping_add(8);
                }
                let block = e.mem.u32(t + 0x0C);
                e.call(OPERATOR_DELETE, &args![block]);
            }
            e.mem
                .set_u32(t + 0x14, storage.wrapping_add(capacity.wrapping_mul(8)));
            e.mem
                .set_u32(t + 0x10, storage.wrapping_add(new_len.wrapping_mul(8)));
            e.mem.set_u32(t + 0x0C, storage);
        }
        Room::Fits => {
            let after = (e.mem.u32(t + 0x10).wrapping_sub(position) as i32 >> 3) as u32;
            let temporary = [e.mem.u32(value.addr()), e.mem.u32(value.addr() + 4)];
            if after < count {
                let last = e.mem.u32(t + 0x10);
                e.call(
                    ENTRY_COPY_RANGE,
                    &args![
                        position,
                        last,
                        position.wrapping_add(count.wrapping_mul(8)),
                        t + 8
                    ],
                );
                e.with_stack(8, |e, copy| {
                    for (i, word) in temporary.iter().enumerate() {
                        e.mem.set_u32(copy.addr() + 4 * i as u32, *word);
                    }
                    let last = e.mem.u32(t + 0x10);
                    let surplus =
                        count.wrapping_sub((last.wrapping_sub(position) as i32 >> 3) as u32);
                    e.call(
                        ENTRY_CONSTRUCT,
                        &args![last, surplus, copy, t + 8, 0u32, 0u32],
                    );
                });
                let end = e.mem.u32(t + 0x10).wrapping_add(count.wrapping_mul(8));
                e.mem.set_u32(t + 0x10, end);
                let stop = e.mem.u32(t + 0x10).wrapping_sub(count.wrapping_mul(8));
                let mut element = position;
                while element != stop {
                    for (i, word) in temporary.iter().enumerate() {
                        e.mem.set_u32(element + 4 * i as u32, *word);
                    }
                    element = element.wrapping_add(8);
                }
            } else {
                let old_last = e.mem.u32(t + 0x10);
                let tail = old_last.wrapping_sub(count.wrapping_mul(8));
                let this_last = e.mem.u32(t + 0x10);
                let end = e
                    .call(ENTRY_TAIL_COPY, &args![tail, old_last, this_last, t + 8])
                    .u32();
                e.mem.set_u32(t + 0x10, end);
                e.call(ENTRY_MOVE_UP, &args![position, tail, old_last]);
                let stop = position.wrapping_add(count.wrapping_mul(8));
                let mut element = position;
                while element != stop {
                    for (i, word) in temporary.iter().enumerate() {
                        e.mem.set_u32(element + 4 * i as u32, *word);
                    }
                    element = element.wrapping_add(8);
                }
            }
        }
    }
}

/// An entry of the pair vector's `00b42180`/`00b417b0` registration: the entry for the two
/// functions that take their words by value in the exe (`00b40b30`, `00b41110`).
fn entry_00b40b30(e: &mut Engine, a: &[u32]) -> Ret {
    let mut leaves = [0u32; 6];
    leaves.copy_from_slice(&a[2..8]);
    let result = fn_00b40b30(e, Ptr::new(a[0]), Ptr::new(a[1]), leaves);
    RetVal::into_ret(result)
}

/// Through the uniform ABI the six words of `00b41110`'s by-value vector arrive as values: they
/// are laid out in an area that stands for its argument area (`[EBP+8]`). The exe's one caller,
/// `00b40b30` (`00b40b95`), calls `fn_00b41110` with the area it built the copy in.
fn entry_00b41110(e: &mut Engine, a: &[u32]) -> Ret {
    e.with_stack(0x18, |e, area| {
        for (i, word) in a[1..7].iter().enumerate() {
            e.mem.set_u32(area.addr() + 4 * i as u32, *word); // 00b41110's [EBP+8]..[EBP+0x1C]
        }
        fn_00b41110(e, Ptr::new(a[0]), area.cast());
    });
    Ret::default()
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00b40a60,
            fn_00b40a60(Ptr<SIdvTreeInfo>) -> Ptr<SIdvTreeInfo>
        ),
        entry!(
            0x00b40ad0,
            fn_00b40ad0(Ptr<CLeafLodEngine>, Ptr, f32, f32, f32) -> Ptr<CLeafLodEngine>
        ),
        (0x00b40b30, entry_00b40b30 as AbiFn),
        entry!(
            0x00b40c80,
            fn_00b40c80(Ptr<CLeafLodEngine>, Ptr<StlVector>) -> Ptr<StlVector>
        ),
        (0x00b41110, entry_00b41110 as AbiFn),
        entry!(0x00b416d0, fn_00b416d0(Ptr<StlVector>) -> Ptr<StlVector>),
        entry!(0x00b417b0, fn_00b417b0(Ptr<StlVector>, Ptr)),
        entry!(0x00b418b0, fn_00b418b0(Ptr, u32, u8) -> Ptr),
        entry!(0x00b419c0, fn_00b419c0(Ptr)),
        entry!(0x00b41a50, fn_00b41a50(Ptr, Ptr) -> Ptr),
        entry!(0x00b41ae0, fn_00b41ae0(Ptr, u8) -> Ptr),
        entry!(
            0x00b41b50,
            fn_00b41b50(Ptr<StlVector>, Ptr, u32, u32, Ptr) -> Ptr
        ),
        entry!(0x00b41c80, fn_00b41c80(Ptr<StlVector>, u32) -> bool),
        entry!(0x00b41d40, fn_00b41d40(Ptr) -> Ptr),
        entry!(0x00b41db0, fn_00b41db0(Ptr, u32) -> u32),
        entry!(0x00b42030, fn_00b42030(Ptr, u32, u32) -> Ptr),
        entry!(0x00b420a0, fn_00b420a0(Ptr, Ptr) -> i32),
        entry!(0x00b420f0, fn_00b420f0(Ptr) -> u32),
        entry!(0x00b42180, fn_00b42180(Ptr<StlVector>, u32, u32, u32, Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    type Log = Vec<(u32, Vec<u32>)>;

    /// The proxy word the test vectors carry; the checked iterators of the doubles carry another.
    const PROXY: u32 = 0x0090_0000;
    const ITERATOR_PROXY: u32 = 0x0090_beef;

    /// Callees outside this file; every test runs them as do-nothing doubles unless it supplies
    /// its own.
    const EXTERNAL: &[u32] = &[
        STRING_TIDY,
        ALLOCATE_PROXY,
        ALLOCATE_BOOL_PROXY,
        LENGTH_ERROR,
        BOOL_LENGTH_ERROR,
        ENTRY_ALLOCATE,
        ENTRY_CONSTRUCT,
        ENTRY_COPY_RANGE,
        ENTRY_TAIL_COPY,
        ENTRY_MOVE_UP,
        WORDS_CONSTRUCT_PROXY,
        WORDS_CONSTRUCT_FILL,
        VECTOR_TIDY,
        WORDS_ITERATOR_ADVANCE,
        WORDS_ERASE,
        BIT_ITERATOR_ADVANCE,
        BIT_REFERENCE_CONSTRUCT,
        BIT_REFERENCE_BASE,
        ITERATOR_CONSTRUCT,
        BIT_FILL,
        ITERATOR_ELEMENT,
        ITERATOR_INCREMENT,
        ITERATORS_EQUAL,
        LEAF_VECTOR_CONSTRUCT,
        LEAF_VECTOR_COPY_CONSTRUCT,
        LEAF_VECTOR_ASSIGN,
        LEAF_VECTOR_PUSH,
        LEAF_CLONE,
        LEAF_GET_COLOR,
        LEAF_SET_COLOR,
        VEC3_DISTANCE,
        RANDOM_CONSTRUCT,
        GET_UNIFORM,
        ATEXIT,
        ENTRY_ITERATOR_ADVANCE,
        INVALID_PARAMETER,
    ];

    /// An engine with the exe's data range mapped as zeroes and every outside callee of the file
    /// silenced.
    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0020_0000);
        for addr in EXTERNAL {
            e.register(*addr, |_, _| Ret::default());
        }
        e
    }

    /// The calls of `addr` in the log, argument words by call.
    fn calls(log: &Log, addr: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    /// A double that records its argument words (the log is returned) and answers zero.
    fn spy(e: &mut Engine, addr: u32) -> Rc<RefCell<Vec<Vec<u32>>>> {
        let log: Rc<RefCell<Vec<Vec<u32>>>> = Rc::new(RefCell::new(Vec::new()));
        let sink = log.clone();
        e.register_double(addr, move |_, a| {
            sink.borrow_mut().push(a.to_vec());
            Ret::default()
        });
        log
    }

    /// `count` words from `at`.
    fn words(e: &Engine, at: u32, count: u32) -> Vec<u32> {
        (0..count).map(|i| e.mem.u32(at + 4 * i)).collect()
    }

    /// Writes `values` as floats from `at`.
    fn put_floats(e: &mut Engine, at: u32, values: &[f32]) {
        for (i, value) in values.iter().enumerate() {
            e.mem.set_f32(at + 4 * i as u32, *value);
        }
    }

    /// `count` floats from `at`.
    fn floats(e: &Engine, at: u32, count: u32) -> Vec<f32> {
        (0..count).map(|i| e.mem.f32(at + 4 * i)).collect()
    }

    /// The header of a `std::vector` at `at`: the proxy word, first, last and end.
    fn header(e: &mut Engine, at: u32, first: u32, last: u32, end: u32) {
        e.mem.set_u32(at, PROXY);
        e.mem.set_u32(at + 0x0C, first);
        e.mem.set_u32(at + 0x10, last);
        e.mem.set_u32(at + 0x14, end);
    }

    /// A vector of 8-byte pairs at `at` with room for `capacity` and holding `elements`; returns
    /// the storage.
    fn pair_vector(e: &mut Engine, at: u32, elements: &[(u32, u32)], capacity: u32) -> u32 {
        let data = e.mem.alloc(capacity * 8);
        for (i, (a, b)) in elements.iter().enumerate() {
            e.mem.set_u32(data + 8 * i as u32, *a);
            e.mem.set_u32(data + 8 * i as u32 + 4, *b);
        }
        header(
            e,
            at,
            data,
            data + 8 * elements.len() as u32,
            data + 8 * capacity,
        );
        data
    }

    /// The pairs of the vector at `at`.
    fn pairs(e: &Engine, at: u32) -> Vec<(u32, u32)> {
        let first = e.mem.u32(at + 0x0C);
        let count = (e.mem.u32(at + 0x10) - first) / 8;
        (0..count)
            .map(|i| (e.mem.u32(first + 8 * i), e.mem.u32(first + 8 * i + 4)))
            .collect()
    }

    /// A vector of words at `at` holding `values`; returns the storage.
    fn word_vector(e: &mut Engine, at: u32, values: &[u32]) -> u32 {
        let data = e.mem.alloc(4 * values.len() as u32);
        for (i, value) in values.iter().enumerate() {
            e.mem.set_u32(data + 4 * i as u32, *value);
        }
        header(
            e,
            at,
            data,
            data + 4 * values.len() as u32,
            data + 4 * values.len() as u32,
        );
        data
    }

    /// Moves `len` bytes (a no-op for none).
    fn move_bytes(e: &mut Engine, from: u32, to: u32, len: u32) {
        if len > 0 {
            let data = e.mem.bytes(from, len);
            e.mem.write(to, &data);
        }
    }

    /// Doubles for the pair storage of `std::vector<SLodEntry>`: the allocation, the copies, the
    /// moves, the construction of copies and the iterator advance.
    fn install_pair_storage(e: &mut Engine) {
        e.register_double(ENTRY_ALLOCATE, |e, a| Ret {
            eax: e.mem.alloc(a[0] * 8),
            ..Ret::default()
        });
        let copy = |e: &mut Engine, a: &[u32]| {
            let len = a[1].wrapping_sub(a[0]);
            move_bytes(e, a[0], a[2], len);
            Ret {
                eax: a[2].wrapping_add(len),
                ..Ret::default()
            }
        };
        e.register_double(ENTRY_COPY_RANGE, copy);
        e.register_double(ENTRY_TAIL_COPY, copy);
        e.register_double(ENTRY_MOVE_UP, |e, a| {
            let len = a[1].wrapping_sub(a[0]);
            move_bytes(e, a[0], a[2].wrapping_sub(len), len);
            Ret::default()
        });
        e.register_double(ENTRY_CONSTRUCT, |e, a| {
            for i in 0..a[1] {
                for w in 0..2 {
                    let word = e.mem.u32(a[2] + 4 * w);
                    e.mem.set_u32(a[0] + 8 * i + 4 * w, word);
                }
            }
            Ret::default()
        });
        e.register_double(ENTRY_ITERATOR_ADVANCE, |e, a| {
            let position = e.mem.u32(a[0] + 4).wrapping_add(8 * a[1]);
            e.mem.set_u32(a[0] + 4, position);
            Ret {
                eax: a[0],
                ..Ret::default()
            }
        });
    }

    /// Doubles for the checked iterators over a vector with `step`-byte elements: an iterator is
    /// built from a position, compared by position, dereferenced to its position and advanced by
    /// one element.
    fn install_iterators(e: &mut Engine, step: u32) {
        e.register_double(ITERATOR_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], ITERATOR_PROXY);
            e.mem.set_u32(a[0] + 4, a[1]);
            Ret {
                eax: a[0],
                ..Ret::default()
            }
        });
        e.register_double(ITERATORS_EQUAL, |e, a| Ret {
            eax: u32::from(e.mem.u32(a[0] + 4) == e.mem.u32(a[1] + 4)),
            ..Ret::default()
        });
        e.register_double(ITERATOR_ELEMENT, |e, a| Ret {
            eax: e.mem.u32(a[0] + 4),
            ..Ret::default()
        });
        e.register_double(ITERATOR_INCREMENT, move |e, a| {
            let position = e.mem.u32(a[0] + 4).wrapping_add(step);
            e.mem.set_u32(a[0] + 4, position);
            Ret {
                eax: a[0],
                ..Ret::default()
            }
        });
        e.register_double(WORDS_ITERATOR_ADVANCE, |e, a| {
            let position = e.mem.u32(a[0] + 4).wrapping_add(4 * a[1]);
            e.mem.set_u32(a[0] + 4, position);
            Ret {
                eax: a[0],
                ..Ret::default()
            }
        });
        // The pair vector's increment (00b41d40): one 8-byte pair on.
        e.register_double(PAIR_ITERATOR_INCREMENT, |e, a| {
            let position = e.mem.u32(a[0] + 4).wrapping_add(8);
            e.mem.set_u32(a[0] + 4, position);
            Ret {
                eax: a[0],
                ..Ret::default()
            }
        });
    }

    /// Puts the real translations at `addrs` back over the doubles (from their units' lists).
    fn restore(e: &mut Engine, addrs: &[u32]) {
        let units = [
            funcs(),
            super::super::frondengine::funcs(),
            super::super::run_00b2ab50::funcs(),
            super::super::run_00b2e250::funcs(),
        ];
        for (addr, f) in units.into_iter().flatten() {
            if addrs.contains(&addr) {
                e.register(addr, f);
            }
        }
    }

    /// The checked iterators of the leaf and pair vectors (00b42980, 00b2ac30, 00b2ab50,
    /// 00b2abc0, 00b2e330, 00b41d40), which find their container through the proxy.
    const CHECKED_ITERATORS: [u32; 6] = [
        ITERATOR_CONSTRUCT,
        ITERATORS_EQUAL,
        ITERATOR_ELEMENT,
        ITERATOR_INCREMENT,
        WORDS_ITERATOR_ADVANCE,
        PAIR_ITERATOR_INCREMENT,
    ];

    /// Gives the vector at `at` a proxy (8 bytes) that names it, as its constructors do.
    fn own_proxy(e: &mut Engine, at: u32) -> u32 {
        let proxy = e.mem.alloc(8);
        e.mem.set_u32(proxy, at);
        e.mem.set_u32(at, proxy);
        proxy
    }

    #[test]
    fn the_tree_info_gets_its_defaults() {
        let mut e = engine();
        e.mem.set_f32(TREE_FAR_DEFAULT, 4000.0);
        e.mem.set_f32(TREE_NEAR_DEFAULT, 100.0);
        e.mem.set_u32(TREE_SIZE_DEFAULT, 0x7f80_0001);
        let tidy = spy(&mut e, STRING_TIDY);
        let this = e.new_object::<SIdvTreeInfo>();
        for word in 0..13 {
            e.mem.set_u32(this.addr() + 4 * word, 0xdead_beef);
        }
        let result = e.call(0x00b4_0a60, &args![this]).ptr::<SIdvTreeInfo>();
        assert_eq!(result, this);
        assert_eq!(*tidy.borrow(), vec![vec![this.addr(), 0, 0]]);
        assert_eq!(e.get(this, SIdvTreeInfo::m_fFar), 4000.0);
        assert_eq!(e.get(this, SIdvTreeInfo::m_fNear), 100.0);
        assert_eq!(e.get(this, SIdvTreeInfo::m_nSeed), 1200);
        // The size goes through the x87 stack, which quiets a signalling NaN.
        assert_eq!(e.mem.u32(this.addr() + 0x28), 0x7fc0_0001);
        assert_eq!(e.get(this, SIdvTreeInfo::m_fSizeVariance), 0.0);
        assert_eq!(e.get(this, SIdvTreeInfo::m_nFlareSeed), 444);
    }

    #[test]
    fn the_lod_engine_stores_its_settings_after_building_the_pair_vector() {
        let mut e = engine();
        let this = e.new_object::<CLeafLodEngine>();
        let result = e
            .call(
                0x00b4_0ad0,
                &args![
                    this,
                    0x1234u32,
                    0.25f32,
                    0.5f32,
                    f32::from_bits(0x7f80_0001)
                ],
            )
            .ptr::<CLeafLodEngine>();
        assert_eq!(result, this);
        let t = this.addr();
        assert_eq!(e.mem.f32(t + 0x18), 0.25);
        assert_eq!(e.mem.f32(t + 0x1C), 0.5);
        assert_eq!(e.mem.u32(t + 0x20), 0x1234);
        assert_eq!(e.mem.u32(t + 0x24), 0x7fc0_0001);
        assert_eq!(e.mem.u32(t), 0);
        assert_eq!(e.mem.u32(t + 0x0C), 0);
    }

    #[test]
    fn computing_runs_the_pairing_then_the_merge_over_a_copy_of_the_leaves() {
        let mut e = engine();
        // Empty leaf vectors make the pairing and the merge run through without leaves; the
        // doubles below record the order of their vector helpers.
        pairing_setup(&mut e, &[], 1.0);
        let order: Rc<RefCell<Vec<(&'static str, Vec<u32>)>>> = Rc::new(RefCell::new(Vec::new()));
        for (name, addr) in [
            ("construct", LEAF_VECTOR_CONSTRUCT),
            ("assign", LEAF_VECTOR_ASSIGN),
            ("tidy", VECTOR_TIDY),
        ] {
            let sink = order.clone();
            e.register_double(addr, move |_, a| {
                sink.borrow_mut().push((name, a.to_vec()));
                Ret::default()
            });
        }
        let sink = order.clone();
        e.register_double(LEAF_VECTOR_COPY_CONSTRUCT, move |e, a| {
            // The copy takes the six words of the source and remembers them.
            let source = words(e, a[1], 6);
            for (i, word) in source.iter().enumerate() {
                e.mem.set_u32(a[0] + 4 * i as u32, *word);
            }
            let mut recorded = a.to_vec();
            recorded.extend(source);
            sink.borrow_mut().push(("copy", recorded));
            Ret::default()
        });
        let this = e.new_object::<CLeafLodEngine>();
        pair_vector(&mut e, this.addr(), &[], 1);
        let returned = e.mem.alloc(0x18);
        let result = e
            .call(
                0x00b4_0b30,
                &args![this, returned, 0u32, 0xAAAAu32, 0xBBBBu32, 0u32, 0u32, 0u32],
            )
            .u32();
        assert_eq!(result, returned);
        let order = order.borrow();
        let names: Vec<&str> = order.iter().map(|(n, _)| *n).collect();
        assert_eq!(
            names,
            [
                "construct",
                "copy",
                "tidy",
                "tidy",
                "construct",
                "copy",
                "tidy",
                "assign",
                "tidy",
                "copy",
                "tidy",
                "tidy"
            ]
        );
        // The leaf vector the result is collected in.
        let local = order[0].1[0];
        // A copy of the argument (six words by value) is made first.
        assert_eq!(order[1].1[2..], [0, 0xAAAA, 0xBBBB, 0, 0, 0]);
        let argument = order[1].1[1];
        // The pairing tidies its bit vector and the copied argument; the merge then builds its
        // own local result, copies it into the temporary and tidies it.
        assert_eq!(order[3].1.len(), 1);
        let merge_local = order[4].1[0];
        let temporary = order[5].1[0];
        assert_eq!(order[5].1[1], merge_local);
        assert_eq!(order[6].1, vec![merge_local]);
        // The merge's result is assigned to the local vector, then the temporary is tidied.
        assert_eq!(order[7].1, vec![local, temporary]);
        assert_eq!(order[8].1, vec![temporary]);
        // The local vector is copied into the returned vector and tidied; the argument is
        // tidied last.
        assert_eq!(order[9].1[..2], [returned, local]);
        assert_eq!(order[10].1, vec![local]);
        assert_eq!(order[11].1, vec![argument]);
    }

    /// The doubles and objects of the merge test: two pairs of leaves, the first pair's leaf at
    /// (3, 4, 5) and its match at (1, 2, 3).
    struct Merge {
        this: Ptr<CLeafLodEngine>,
        leaves: Vec<u32>,
    }

    fn merge_setup(e: &mut Engine, uniform: Vec<f64>) -> Merge {
        install_iterators(e, 8);
        e.mem.set_f32(HALF_FLOAT, 0.5);
        e.mem.set_f64(SIZE_SCALE_DOUBLE, 0.5);
        let this = e.new_object::<CLeafLodEngine>();
        let t = this.addr();
        // Four leaves of 0x60 bytes: position at `+4`, the texture byte at `+0x40`, colour at
        // `+0x4C` (the colour double reads it from there).
        let mut leaves = Vec::new();
        for _ in 0..4 {
            let leaf = e.mem.alloc(0x60);
            e.mem.set_u8(leaf + 0x40, 3);
            leaves.push(leaf);
        }
        put_floats(e, leaves[0] + 4, &[3.0, 4.0, 5.0]);
        put_floats(e, leaves[1] + 4, &[1.0, 2.0, 3.0]);
        put_floats(e, leaves[2] + 4, &[9.0, 9.0, 9.0]);
        put_floats(e, leaves[3] + 4, &[8.0, 8.0, 8.0]);
        put_floats(e, leaves[0] + 0x4C, &[0.4, 0.2, 0.0]);
        put_floats(e, leaves[1] + 0x4C, &[0.2, 0.4, 0.6]);
        pair_vector(e, t, &[(leaves[0], leaves[1]), (leaves[2], leaves[3])], 2);
        e.mem.set_f32(t + 0x1C, 0.5);
        e.mem.set_f32(t + 0x24, 2.0);
        // The leaf info: two textures of 0x54 bytes in the vector at `+0x10`; texture 1 has the
        // origin `y` 0.75 (`+0x34`) and the size `y` 4.0 (`+0x4C`).
        let info = e.mem.alloc(0x30);
        let textures = e.mem.alloc(0x54 * 2);
        header(
            e,
            info + 0x10,
            textures,
            textures + 0x54 * 2,
            textures + 0x54 * 2,
        );
        e.mem.set_f32(textures + 0x54 + 0x34, 0.75);
        e.mem.set_f32(textures + 0x54 + 0x4C, 4.0);
        e.mem.set_u32(t + 0x20, info);
        let queue = Rc::new(RefCell::new(uniform));
        e.register_double(GET_UNIFORM, move |_, _| Ret {
            st0: queue.borrow_mut().remove(0),
            ..Ret::default()
        });
        e.register_double(LEAF_CLONE, |e, a| {
            let copy = e.mem.alloc(0x60);
            let data = e.mem.bytes(a[0], 0x60);
            e.mem.write(copy, &data);
            Ret {
                eax: copy,
                ..Ret::default()
            }
        });
        e.register_double(LEAF_GET_COLOR, |e, a| {
            for i in 0..3 {
                let word = e.mem.u32(a[0] + 0x4C + 4 * i);
                e.mem.set_u32(a[1] + 4 * i, word);
            }
            Ret {
                eax: a[1],
                ..Ret::default()
            }
        });
        Merge { this, leaves }
    }

    #[test]
    fn leaf_pairs_above_the_reduction_percentage_are_merged_into_one_leaf() {
        let mut e = engine();
        let merge = merge_setup(&mut e, vec![0.9, 0.1, 0.9, 0.1]);
        let colors = Rc::new(RefCell::new(Vec::new()));
        let sink = colors.clone();
        e.register_double(LEAF_SET_COLOR, move |e, a| {
            sink.borrow_mut().push((a[0], floats(e, a[1], 3), a[2]));
            Ret::default()
        });
        let pushed = Rc::new(RefCell::new(Vec::new()));
        let sink = pushed.clone();
        e.register_double(LEAF_VECTOR_PUSH, move |e, a| {
            sink.borrow_mut().push((a[0], e.mem.u32(a[1])));
            Ret::default()
        });
        let constructed = spy(&mut e, RANDOM_CONSTRUCT);
        let registered = spy(&mut e, ATEXIT);
        let copied = spy(&mut e, LEAF_VECTOR_COPY_CONSTRUCT);
        e.call_log = Some(vec![]);
        let returned = e.mem.alloc(0x18);
        let result = e.call(0x00b4_0c80, &args![merge.this, returned]).u32();
        assert_eq!(result, returned);
        let log = e.call_log.take().unwrap();
        // The random generator was built and its destructor registered the first time only.
        assert_eq!(*constructed.borrow(), vec![vec![RANDOM_STATIC]]);
        assert_eq!(*registered.borrow(), vec![vec![RANDOM_DESTRUCTOR]]);
        assert_eq!(e.mem.u32(RANDOM_GUARD) & 1, 1);
        // Each pair draws a number between 0 and 1.
        assert_eq!(
            calls(&log, GET_UNIFORM),
            vec![
                vec![RANDOM_STATIC, 0.0f32.to_bits(), 1.0f32.to_bits()],
                vec![RANDOM_STATIC, 0.0f32.to_bits(), 1.0f32.to_bits()]
            ]
        );
        // Only the first pair (0.9 above 0.5) was merged: a copy of its leaf.
        let clones = calls(&log, LEAF_CLONE);
        assert_eq!(clones, vec![vec![merge.leaves[0]]]);
        let pushed = pushed.borrow();
        assert_eq!(pushed.len(), 1);
        let new_leaf = pushed[0].1;
        assert_ne!(new_leaf, merge.leaves[0]);
        // Half the sum of the positions, with `z` lifted by `(0.75 - 0.5) * (2 * 4 * 0.5)`.
        assert_eq!(floats(&e, new_leaf + 4, 3), vec![2.0, 3.0, 5.0]);
        let colors = colors.borrow();
        assert_eq!(colors.len(), 1);
        assert_eq!(colors[0].0, new_leaf);
        let expect = [
            (0.2f32 + 0.4) * 0.5,
            (0.4f32 + 0.2) * 0.5,
            (0.6f32 + 0.0) * 0.5,
        ];
        assert_eq!(colors[0].1, expect.to_vec());
        assert_eq!(colors[0].2, 0);
        // The local result is copied to the returned vector.
        assert_eq!(copied.borrow().len(), 1);
        assert_eq!(copied.borrow()[0][0], returned);
        drop(colors);
        drop(pushed);
        // A second run does not build the generator again.
        e.call(0x00b4_0c80, &args![merge.this, returned]);
        assert_eq!(constructed.borrow().len(), 1);
        assert_eq!(registered.borrow().len(), 1);
    }

    #[test]
    fn a_draw_not_above_the_percentage_merges_nothing() {
        let mut e = engine();
        let merge = merge_setup(&mut e, vec![0.5, 0.5]);
        let clones = spy(&mut e, LEAF_CLONE);
        let returned = e.mem.alloc(0x18);
        e.call(0x00b4_0c80, &args![merge.this, returned]);
        assert!(clones.borrow().is_empty());
    }

    #[test]
    fn the_merge_moves_on_one_pair_at_a_time() {
        // 00b40d29..00b40d2c: the merge loop steps with the pair vector's increment (00b41d40,
        // 8 bytes), not the 4-byte one of the leaf vectors (00b2abc0), so each pair draws once
        // and a merge copies the pair's leaf, never its match.
        let mut e = engine();
        let merge = merge_setup(&mut e, vec![0.9, 0.9]);
        restore(&mut e, &CHECKED_ITERATORS);
        e.register_double(INVALID_PARAMETER, |_, _| -> Ret {
            panic!("invalid parameter")
        });
        own_proxy(&mut e, merge.this.addr());
        e.call_log = Some(vec![]);
        let returned = e.mem.alloc(0x18);
        e.call(0x00b4_0c80, &args![merge.this, returned]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls(&log, GET_UNIFORM).len(), 2);
        assert_eq!(
            calls(&log, LEAF_CLONE),
            vec![vec![merge.leaves[0]], vec![merge.leaves[2]]]
        );
        assert_eq!(calls(&log, PAIR_ITERATOR_INCREMENT).len(), 2);
        assert!(calls(&log, ITERATOR_INCREMENT).is_empty());
    }

    /// The leaves of the pairing test (positions along `x`), the vector of pointers to them and
    /// the leaf engine with its tolerance.
    struct Pairing {
        this: Ptr<CLeafLodEngine>,
        leaves: Vec<u32>,
        bit_words: Rc<RefCell<Vec<u32>>>,
        pushed: Rc<RefCell<Vec<(u32, u32)>>>,
    }

    fn pairing_setup(e: &mut Engine, xs: &[f32], tolerance: f32) -> Pairing {
        install_iterators(e, 4);
        e.mem.set_f32(LARGEST_FLOAT, f32::MAX);
        e.register_double(ALLOCATE_BOOL_PROXY, |e, _| Ret {
            eax: e.mem.alloc(8),
            ..Ret::default()
        });
        e.register_double(WORDS_CONSTRUCT_FILL, |e, a| {
            let data = e.mem.alloc((a[1] * 4).max(4));
            let fill = e.mem.u32(a[2]);
            for i in 0..a[1] {
                e.mem.set_u32(data + 4 * i, fill);
            }
            e.mem.set_u32(a[0] + 0x0C, data);
            e.mem.set_u32(a[0] + 0x10, data + 4 * a[1]);
            e.mem.set_u32(a[0] + 0x14, data + 4 * a[1]);
            Ret::default()
        });
        e.register_double(BIT_ITERATOR_ADVANCE, |e, a| {
            let total = e.mem.u32(a[0] + 8) + a[1];
            let word = e.mem.u32(a[0] + 4) + 4 * (total / 32);
            e.mem.set_u32(a[0] + 4, word);
            e.mem.set_u32(a[0] + 8, total % 32);
            Ret::default()
        });
        e.register_double(BIT_REFERENCE_CONSTRUCT, |e, a| {
            for i in 0..3 {
                let word = e.mem.u32(a[1] + 4 * i);
                e.mem.set_u32(a[0] + 4 * i, word);
            }
            Ret {
                eax: a[0],
                ..Ret::default()
            }
        });
        e.register_double(BIT_REFERENCE_BASE, |e, a| {
            let owner = e.mem.u32(e.mem.u32(a[0]));
            Ret {
                eax: e.mem.u32(owner + 0x18),
                ..Ret::default()
            }
        });
        e.register_double(VEC3_DISTANCE, |e, a| {
            let sum: f64 = (0..3)
                .map(|i| {
                    let d = f64::from(e.mem.f32(a[0] + 4 * i)) - f64::from(e.mem.f32(a[1] + 4 * i));
                    d * d
                })
                .sum();
            Ret {
                st0: sum.sqrt(),
                ..Ret::default()
            }
        });
        // The first tidy is the bit vector's word vector: its first word is the bit set.
        let bit_words = Rc::new(RefCell::new(Vec::new()));
        let sink = bit_words.clone();
        e.register_double(VECTOR_TIDY, move |e, a| {
            if sink.borrow().is_empty() {
                let first = e.mem.u32(a[0] + 0x0C);
                let last = e.mem.u32(a[0] + 0x10);
                let words = (last - first) / 4;
                *sink.borrow_mut() = (0..words).map(|i| e.mem.u32(first + 4 * i)).collect();
            }
            Ret::default()
        });
        let pushed = Rc::new(RefCell::new(Vec::new()));
        let sink = pushed.clone();
        e.register_double(ENTRY_CONSTRUCT, move |e, a| {
            sink.borrow_mut()
                .push((e.mem.u32(a[2]), e.mem.u32(a[2] + 4)));
            for w in 0..2 {
                let word = e.mem.u32(a[2] + 4 * w);
                e.mem.set_u32(a[0] + 4 * w, word);
            }
            Ret::default()
        });
        let this = e.new_object::<CLeafLodEngine>();
        pair_vector(e, this.addr(), &[], 4);
        e.mem.set_f32(this.addr() + 0x18, tolerance);
        let mut leaves = Vec::new();
        for x in xs {
            let leaf = e.mem.alloc(0x4C);
            put_floats(e, leaf + 4, &[*x, 0.0, 0.0]);
            leaves.push(leaf);
        }
        Pairing {
            this,
            leaves,
            bit_words,
            pushed,
        }
    }

    /// Runs the pairing on the leaves (a vector of pointers handed by value).
    fn run_pairing(e: &mut Engine, setup: &Pairing) {
        let block = e.mem.alloc(4 * setup.leaves.len() as u32);
        for (i, leaf) in setup.leaves.iter().enumerate() {
            e.mem.set_u32(block + 4 * i as u32, *leaf);
        }
        let end = block + 4 * setup.leaves.len() as u32;
        e.call(
            0x00b4_1110,
            &args![setup.this, 0u32, 0u32, 0u32, block, end, end],
        );
    }

    #[test]
    fn each_leaf_is_paired_with_its_nearest_unpaired_later_leaf_within_the_tolerance() {
        let mut e = engine();
        let setup = pairing_setup(&mut e, &[0.0, 1.0, 5.0], 2.0);
        run_pairing(&mut e, &setup);
        // Leaf 0 pairs with leaf 1 (distance 1); leaf 1 and leaf 2 are 4 apart.
        assert_eq!(
            *setup.pushed.borrow(),
            vec![(setup.leaves[0], setup.leaves[1])]
        );
        // Bit 1 (the matched leaf) is set in the three-bit vector.
        assert_eq!(*setup.bit_words.borrow(), vec![0b010]);
        let t = setup.this.addr();
        assert_eq!(e.mem.u32(t + 0x10) - e.mem.u32(t + 0x0C), 8);
    }

    #[test]
    fn the_search_for_a_match_stops_at_a_leaf_that_is_already_paired() {
        let mut e = engine();
        // Leaf 0 prefers leaf 2 (0.5 away) over leaf 1 (1 away); leaf 1 then finds leaf 2
        // taken and stops.
        let setup = pairing_setup(&mut e, &[0.0, 1.0, 0.5], 2.0);
        run_pairing(&mut e, &setup);
        assert_eq!(
            *setup.pushed.borrow(),
            vec![(setup.leaves[0], setup.leaves[2])]
        );
        assert_eq!(*setup.bit_words.borrow(), vec![0b100]);
    }

    #[test]
    fn leaves_beyond_the_tolerance_make_no_pairs() {
        let mut e = engine();
        let setup = pairing_setup(&mut e, &[0.0, 3.0, 6.0], 2.0);
        run_pairing(&mut e, &setup);
        assert!(setup.pushed.borrow().is_empty());
        assert_eq!(*setup.bit_words.borrow(), vec![0]);
    }

    #[test]
    fn compute_pairs_the_leaves_in_the_area_it_built_their_copy_in() {
        // 00b40b78..00b40b95: 00b40b30 builds the copy of the leaves in 00b41110's argument
        // area, so the copy's proxy names the vector the pairing's checked iterators walk. With
        // freed blocks held back and filled (the battery's NV_BLOCK_CHECK=store,freed,log), a
        // copy built in an area freed before the pairing ran left 00b2e330 no container behind
        // the proxy: the invalid-parameter path (Goodsprings, thread 14).
        let mut e = engine();
        e.mem.enable_freed_check(false);
        let setup = pairing_setup(&mut e, &[0.0, 1.0, 5.0], 2.0);
        restore(&mut e, &CHECKED_ITERATORS);
        e.register_double(INVALID_PARAMETER, |_, _| -> Ret {
            panic!("invalid parameter")
        });
        let deleted = spy(&mut e, OPERATOR_DELETE);
        // 00b355d0: the copy gets a proxy of its own that names it.
        let built: Rc<RefCell<Vec<(u32, u32)>>> = Rc::new(RefCell::new(Vec::new()));
        let sink = built.clone();
        e.register_double(LEAF_VECTOR_COPY_CONSTRUCT, move |e, a| {
            let proxy = own_proxy(e, a[0]);
            for offset in [0x0Cu32, 0x10, 0x14] {
                let word = e.mem.u32(a[1] + offset);
                e.mem.set_u32(a[0] + offset, word);
            }
            sink.borrow_mut().push((a[0], proxy));
            Ret {
                eax: a[0],
                ..Ret::default()
            }
        });
        own_proxy(&mut e, setup.this.addr());
        // The leaves as the caller (00b2c890) hands them: a vector whose proxy names it.
        let caller = e.mem.alloc(0x18);
        let block = e.mem.alloc(4 * 3);
        for (i, leaf) in setup.leaves.iter().enumerate() {
            e.mem.set_u32(block + 4 * i as u32, *leaf);
        }
        header(&mut e, caller, block, block + 12, block + 12);
        own_proxy(&mut e, caller);
        let w = words(&e, caller, 6);
        let returned = e.mem.alloc(0x18);
        e.call_log = Some(vec![]);
        e.call(
            0x00b4_0b30,
            &args![setup.this, returned, w[0], w[1], w[2], w[3], w[4], w[5]],
        );
        let log = e.call_log.take().unwrap();
        // Leaf 0 pairs with leaf 1 (distance 1); leaf 1 and leaf 2 are 4 apart.
        assert_eq!(
            *setup.pushed.borrow(),
            vec![(setup.leaves[0], setup.leaves[1])]
        );
        // The pairing's first iterator is made on the area the copy was built in, which the
        // pairing tidies at the end and whose proxy it deletes.
        let (area, area_proxy) = built.borrow()[0];
        let made = calls(&log, ITERATOR_CONSTRUCT);
        assert_eq!(made[0][1..], [block, area]);
        assert!(calls(&log, VECTOR_TIDY).contains(&vec![area]));
        assert!(deleted.borrow().contains(&vec![area_proxy]));
    }

    #[test]
    fn the_pair_vector_constructor_allocates_its_proxy_and_buys_nothing() {
        let mut e = engine();
        let log = Rc::new(RefCell::new(Vec::new()));
        let sink = log.clone();
        let proxy = e.mem.alloc(8);
        e.register_double(ALLOCATE_PROXY, move |_, a| {
            sink.borrow_mut().push(a.to_vec());
            Ret {
                eax: proxy,
                ..Ret::default()
            }
        });
        let this = e.mem.alloc(0x18);
        for word in 0..6 {
            e.mem.set_u32(this + 4 * word, 0xdead_beef);
        }
        let result = e.call(0x00b4_16d0, &args![this]).u32();
        assert_eq!(result, this);
        assert_eq!(*log.borrow(), vec![vec![1, 0]]);
        assert_eq!(e.mem.u32(this), proxy);
        assert_eq!(e.mem.u32(proxy), this);
        assert_eq!(words(&e, this + 0x0C, 3), vec![0, 0, 0]);
        // Without a proxy the vector's first word is zero.
        reply(&mut e, ALLOCATE_PROXY, 0);
        e.mem.set_u32(this, 0xdead_beef);
        e.call(0x00b4_16d0, &args![this]);
        assert_eq!(e.mem.u32(this), 0);
    }

    /// A double that answers `result` in `EAX`.
    fn reply(e: &mut Engine, addr: u32, result: u32) {
        e.register_double(addr, move |_, _| Ret {
            eax: result,
            ..Ret::default()
        });
    }

    #[test]
    fn pushing_a_pair_constructs_it_at_the_end_when_there_is_room() {
        let mut e = engine();
        install_pair_storage(&mut e);
        let constructed = spy(&mut e, ENTRY_CONSTRUCT);
        let this = e.mem.alloc(0x18);
        let data = pair_vector(&mut e, this, &[(1, 10)], 4);
        let value = e.mem.alloc(8);
        e.call(0x00b4_17b0, &args![this, value]);
        assert_eq!(
            *constructed.borrow(),
            vec![vec![data + 8, 1, value, this + 8, 0, 0]]
        );
        assert_eq!(e.mem.u32(this + 0x10), data + 16);
    }

    #[test]
    fn pushing_a_pair_without_room_grows_the_vector_through_the_checked_insert() {
        let mut e = engine();
        install_pair_storage(&mut e);
        let this = e.mem.alloc(0x18);
        let data = pair_vector(&mut e, this, &[(1, 10), (2, 20)], 2);
        let value = e.mem.alloc(8);
        e.mem.set_u32(value, 9);
        e.mem.set_u32(value + 4, 90);
        e.call(0x00b4_17b0, &args![this, value]);
        assert_eq!(pairs(&e, this), vec![(1, 10), (2, 20), (9, 90)]);
        assert_eq!(e.mem.block_size(data), None);
        // The capacity grew by half again (3 pairs).
        assert_eq!(e.mem.u32(this + 0x14) - e.mem.u32(this + 0x0C), 24);
    }

    #[test]
    fn the_bit_vector_constructor_builds_the_words_and_sets_the_size() {
        let mut e = engine();
        let this = e.mem.alloc(0x30);
        let fills = Rc::new(RefCell::new(Vec::new()));
        let sink = fills.clone();
        e.register_double(WORDS_CONSTRUCT_FILL, move |e, a| {
            let data = e.mem.alloc((a[1] * 4).max(4));
            let fill = e.mem.u32(a[2]);
            sink.borrow_mut().push((a[0], a[1], fill));
            for i in 0..a[1] {
                e.mem.set_u32(data + 4 * i, fill);
            }
            header(e, a[0], data, data + 4 * a[1], data + 4 * a[1]);
            Ret::default()
        });
        let proxy_calls = Rc::new(RefCell::new(Vec::new()));
        let sink = proxy_calls.clone();
        let proxy = e.mem.alloc(8);
        e.register_double(ALLOCATE_BOOL_PROXY, move |_, a| {
            sink.borrow_mut().push(a.to_vec());
            Ret {
                eax: proxy,
                ..Ret::default()
            }
        });
        let result = e.call(0x00b4_18b0, &args![this, 40u32, 1u32]).u32();
        assert_eq!(result, this);
        assert_eq!(*proxy_calls.borrow(), vec![vec![this + 4, 1]]);
        assert_eq!(e.mem.u32(this), proxy);
        assert_eq!(e.mem.u32(proxy), this);
        assert_eq!(*fills.borrow(), vec![(this + 0x0C, 2, u32::MAX)]);
        assert_eq!(e.mem.u32(this + 8), 40);
        // The bits above the 40th are masked off.
        let first = e.mem.u32(this + 0x18);
        assert_eq!(words(&e, first, 2), vec![u32::MAX, 0xFF]);
        // A false value fills with zeros; a missing proxy leaves the first word zero.
        reply(&mut e, ALLOCATE_BOOL_PROXY, 0);
        e.mem.set_u32(this, 0xdead_beef);
        fills.borrow_mut().clear();
        e.call(0x00b4_18b0, &args![this, 32u32, 0u32]);
        assert_eq!(e.mem.u32(this), 0);
        assert_eq!(*fills.borrow(), vec![(this + 0x0C, 1, 0)]);
        assert_eq!(e.mem.u32(this + 8), 32);
    }

    #[test]
    fn destroying_a_bit_vector_frees_both_proxies() {
        let mut e = engine();
        let tidy = spy(&mut e, VECTOR_TIDY);
        let this = e.mem.alloc(0x20);
        let (container_proxy, words_proxy) = (e.mem.alloc(8), e.mem.alloc(8));
        e.mem.set_u32(this, container_proxy);
        e.mem.set_u32(this + 8, 77);
        e.mem.set_u32(this + 0x0C, words_proxy);
        e.call(0x00b4_19c0, &args![this]);
        assert_eq!(e.mem.u32(this + 8), 0);
        assert_eq!(*tidy.borrow(), vec![vec![this + 0x0C]]);
        assert_eq!(e.mem.block_size(container_proxy), None);
        assert_eq!(e.mem.block_size(words_proxy), None);
    }

    #[test]
    fn the_begin_iterator_of_a_bit_vector_is_the_proxy_and_the_first_word() {
        let mut e = engine();
        let this = e.mem.alloc(0x30);
        e.mem.set_u32(this, 0x1234);
        header(&mut e, this + 0x0C, 0x2000, 0x2008, 0x2008);
        let out = e.mem.alloc(12);
        let result = e.call(0x00b4_1a50, &args![this, out]).u32();
        assert_eq!(result, out);
        assert_eq!(words(&e, out, 3), vec![0x1234, 0x2000, 0]);
        // A null vector takes the invalid-parameter path (the low page is readable here).
        let invalid = spy(&mut e, INVALID_PARAMETER);
        e.map(0, 0x1000);
        e.call(0x00b4_1a50, &args![0u32, out]);
        assert_eq!(invalid.borrow().len(), 1);
    }

    /// A bit vector of `bits` bits with the given words, and a reference to bit `offset` of the
    /// word at index `word`: returns the vector, the reference and the words' storage.
    fn bit_reference_at(
        e: &mut Engine,
        bits: u32,
        values: &[u32],
        word: u32,
        offset: u32,
    ) -> (u32, u32, u32) {
        let vector = e.mem.alloc(0x30);
        let proxy = e.mem.alloc(8);
        e.mem.set_u32(proxy, vector);
        e.mem.set_u32(vector, proxy);
        e.mem.set_u32(vector + 8, bits);
        let data = word_vector(e, vector + 0x0C, values);
        let reference = e.mem.alloc(12);
        e.mem.set_u32(reference, proxy);
        e.mem.set_u32(reference + 4, data + 4 * word);
        e.mem.set_u32(reference + 8, offset);
        e.register_double(BIT_REFERENCE_BASE, move |_, _| Ret {
            eax: data,
            ..Ret::default()
        });
        (vector, reference, data)
    }

    #[test]
    fn a_bit_reference_names_its_word_when_the_bit_is_inside_the_vector() {
        let mut e = engine();
        let invalid = spy(&mut e, INVALID_PARAMETER);
        let (_, reference, data) = bit_reference_at(&mut e, 40, &[0, 0], 1, 3);
        let word = e.call(0x00b4_20f0, &args![reference]).u32();
        assert_eq!(word, data + 4);
        assert!(invalid.borrow().is_empty());
        // Bit 40 is past the 40 bits.
        e.mem.set_u32(reference + 8, 8);
        e.call(0x00b4_20f0, &args![reference]);
        assert_eq!(invalid.borrow().len(), 1);
        // No proxy: the invalid-parameter path twice (the proxy test and the bit count read from
        // the zero page).
        e.map(0, 0x1000);
        e.mem.set_u32(reference, 0);
        e.call(0x00b4_20f0, &args![reference]);
        assert_eq!(invalid.borrow().len(), 3);
    }

    #[test]
    fn assigning_a_bit_reference_sets_or_clears_its_bit() {
        let mut e = engine();
        let (_, reference, data) = bit_reference_at(&mut e, 64, &[0xF000, 0x1], 0, 5);
        let result = e.call(0x00b4_1ae0, &args![reference, 1u32]).u32();
        assert_eq!(result, reference);
        assert_eq!(e.mem.u32(data), 0xF000 | (1 << 5));
        e.call(0x00b4_1ae0, &args![reference, 0u32]);
        assert_eq!(e.mem.u32(data), 0xF000);
        // Bits of other words are untouched.
        assert_eq!(e.mem.u32(data + 4), 1);
    }

    #[test]
    fn the_checked_insert_of_a_pair_returns_an_iterator_at_the_new_element() {
        let mut e = engine();
        install_pair_storage(&mut e);
        let this = e.mem.alloc(0x18);
        let data = pair_vector(&mut e, this, &[(1, 10), (2, 20)], 2);
        let value = e.mem.alloc(8);
        e.mem.set_u32(value, 9);
        e.mem.set_u32(value + 4, 90);
        let out = e.mem.alloc(8);
        let result = e
            .call(0x00b4_1b50, &args![this, out, PROXY, data + 8, value])
            .u32();
        assert_eq!(result, out);
        assert_eq!(pairs(&e, this), vec![(1, 10), (9, 90), (2, 20)]);
        let storage = e.mem.u32(this + 0x0C);
        assert_eq!(words(&e, out, 2), vec![PROXY, storage + 8]);
        // An empty vector inserts at offset zero.
        let empty = e.mem.alloc(0x18);
        e.mem.set_u32(empty, PROXY);
        let result = e
            .call(0x00b4_1b50, &args![empty, out, PROXY, 0u32, value])
            .u32();
        assert_eq!(result, out);
        assert_eq!(pairs(&e, empty), vec![(9, 90)]);
        assert_eq!(words(&e, out, 2), vec![PROXY, e.mem.u32(empty + 0x0C)]);
        // An iterator without a proxy takes the invalid-parameter path.
        let invalid = spy(&mut e, INVALID_PARAMETER);
        let this = e.mem.alloc(0x18);
        let data = pair_vector(&mut e, this, &[(1, 10)], 4);
        e.call(0x00b4_1b50, &args![this, out, 0u32, data, value]);
        assert_eq!(invalid.borrow().len(), 1);
    }

    #[test]
    fn buying_room_for_pairs_covers_zero_too_many_and_a_normal_count() {
        let mut e = engine();
        let this = e.mem.alloc(0x18);
        for word in 3..6 {
            e.mem.set_u32(this + 4 * word, 0xdead_beef);
        }
        assert_eq!(e.call(0x00b4_1c80, &args![this, 0u32]).u32() & 0xFF, 0);
        assert_eq!(words(&e, this + 0x0C, 3), vec![0, 0, 0]);
        // Too many pairs raise the length error.
        let length = spy(&mut e, LENGTH_ERROR);
        let bought = e.call(0x00b4_1c80, &args![this, 0x2000_0000u32]).u32() & 0xFF;
        assert_eq!(bought, 1);
        assert_eq!(length.borrow().len(), 1);
        assert_eq!(words(&e, this + 0x0C, 3), vec![0, 0, 0]);
        // A normal count allocates `count` pairs.
        let allocate = Rc::new(RefCell::new(Vec::new()));
        let sink = allocate.clone();
        let storage = e.mem.alloc(40);
        e.register_double(ENTRY_ALLOCATE, move |_, a| {
            sink.borrow_mut().push(a.to_vec());
            Ret {
                eax: storage,
                ..Ret::default()
            }
        });
        let bought = e.call(0x00b4_1c80, &args![this, 5u32]).u32() & 0xFF;
        assert_eq!(bought, 1);
        assert_eq!(*allocate.borrow(), vec![vec![5, 0]]);
        assert_eq!(
            words(&e, this + 0x0C, 3),
            vec![storage, storage, storage + 40]
        );
    }

    #[test]
    fn advancing_a_pair_iterator_checks_the_proxy_and_the_end() {
        let mut e = engine();
        let invalid = spy(&mut e, INVALID_PARAMETER);
        let owner = e.mem.alloc(0x18);
        let data = pair_vector(&mut e, owner, &[(1, 2), (3, 4)], 2);
        let proxy = e.mem.alloc(8);
        e.mem.set_u32(proxy, owner);
        let iterator = e.mem.alloc(8);
        e.mem.set_u32(iterator, proxy);
        e.mem.set_u32(iterator + 4, data);
        let result = e.call(0x00b4_1d40, &args![iterator]).u32();
        assert_eq!(result, iterator);
        assert_eq!(e.mem.u32(iterator + 4), data + 8);
        assert!(invalid.borrow().is_empty());
        // At the end: the position is not below `last`.
        e.mem.set_u32(iterator + 4, data + 16);
        e.call(0x00b4_1d40, &args![iterator]);
        assert_eq!(invalid.borrow().len(), 1);
        assert_eq!(e.mem.u32(iterator + 4), data + 24);
        // Without a proxy both checks fail (the owner is read as zero).
        e.map(0, 0x1000);
        e.mem.set_u32(iterator, 0);
        e.mem.set_u32(iterator + 4, 0x2000);
        e.call(0x00b4_1d40, &args![iterator]);
        assert_eq!(invalid.borrow().len(), 3);
        assert_eq!(e.mem.u32(iterator + 4), 0x2008);
    }

    /// A bit vector object holding `values` as its words.
    fn bit_object(e: &mut Engine, values: &[u32]) -> (u32, u32) {
        let this = e.mem.alloc(0x30);
        let data = word_vector(e, this + 0x0C, values);
        (this, data)
    }

    #[test]
    fn setting_the_bit_count_erases_surplus_words_and_masks_the_last_one() {
        let mut e = engine();
        install_iterators(&mut e, 4);
        let erase = spy(&mut e, WORDS_ERASE);
        let (this, data) = bit_object(&mut e, &[u32::MAX; 3]);
        let word = e.call(0x00b4_1db0, &args![this, 40u32]).u32();
        // Two words are needed; the surplus third is erased from the second word's end.
        assert_eq!(
            *erase.borrow(),
            vec![vec![
                this + 0x0C,
                erase.borrow()[0][1],
                PROXY,
                data + 8,
                PROXY,
                data + 12
            ]]
        );
        assert_eq!(e.mem.u32(this + 8), 40);
        assert_eq!(word, data + 4);
        assert_eq!(e.mem.u32(data + 4), 0xFF);
        assert_eq!(e.mem.u32(data), u32::MAX);
        // A multiple of 32 bits needs no mask and answers the word count.
        let (this, data) = bit_object(&mut e, &[u32::MAX; 2]);
        erase.borrow_mut().clear();
        let result = e.call(0x00b4_1db0, &args![this, 64u32]).u32();
        assert_eq!(result, 2);
        assert!(erase.borrow().is_empty());
        assert_eq!(words(&e, data, 2), vec![u32::MAX, u32::MAX]);
        assert_eq!(e.mem.u32(this + 8), 64);
        // Zero bits erase every word.
        let (this, data) = bit_object(&mut e, &[7]);
        let result = e.call(0x00b4_1db0, &args![this, 0u32]).u32();
        assert_eq!(result, 0);
        assert_eq!(erase.borrow()[0][3], data);
    }

    #[test]
    fn a_word_vector_is_constructed_with_a_proxy_and_filled() {
        let mut e = engine();
        let proxy = spy(&mut e, WORDS_CONSTRUCT_PROXY);
        let fill = spy(&mut e, WORDS_CONSTRUCT_FILL);
        let this = e.mem.alloc(0x18);
        let result = e.call(0x00b4_2030, &args![this, 3u32, 0x1234u32]).u32();
        assert_eq!(result, this);
        assert_eq!(*proxy.borrow(), vec![vec![this, 0]]);
        assert_eq!(*fill.borrow(), vec![vec![this, 3, 0x1234]]);
    }

    #[test]
    fn the_difference_of_two_word_iterators_is_in_words() {
        let mut e = engine();
        let invalid = spy(&mut e, INVALID_PARAMETER);
        let (a, b) = (e.mem.alloc(8), e.mem.alloc(8));
        for (iterator, position) in [(a, 0x3010u32), (b, 0x3000)] {
            e.mem.set_u32(iterator, PROXY);
            e.mem.set_u32(iterator + 4, position);
        }
        assert_eq!(e.call(0x00b4_20a0, &args![a, b]).i32(), 4);
        assert_eq!(e.call(0x00b4_20a0, &args![b, a]).i32(), -4);
        assert!(invalid.borrow().is_empty());
        // Different proxies, or none, take the invalid-parameter path.
        e.mem.set_u32(b, PROXY + 8);
        e.call(0x00b4_20a0, &args![a, b]);
        assert_eq!(invalid.borrow().len(), 1);
        e.mem.set_u32(a, 0);
        e.mem.set_u32(b, 0);
        e.call(0x00b4_20a0, &args![a, b]);
        assert_eq!(invalid.borrow().len(), 2);
    }

    #[test]
    fn the_insert_changes_nothing_for_a_count_of_zero_and_reports_too_long_a_result() {
        let mut e = engine();
        install_pair_storage(&mut e);
        let this = e.mem.alloc(0x18);
        let data = pair_vector(&mut e, this, &[(1, 10)], 4);
        let value = e.mem.alloc(8);
        let length = spy(&mut e, LENGTH_ERROR);
        e.call(0x00b4_2180, &args![this, PROXY, data, 0u32, value]);
        assert_eq!(pairs(&e, this), vec![(1, 10)]);
        assert!(length.borrow().is_empty());
        e.call(
            0x00b4_2180,
            &args![this, PROXY, data, 0x2000_0000u32, value],
        );
        assert_eq!(length.borrow().len(), 1);
        assert_eq!(pairs(&e, this), vec![(1, 10)]);
    }

    fn insert(e: &mut Engine, at: u32, index: u32, count: u32, value: (u32, u32)) {
        let first = e.mem.u32(at + 0x0C);
        let slot = e.mem.alloc(8);
        e.mem.set_u32(slot, value.0);
        e.mem.set_u32(slot + 4, value.1);
        e.call(
            0x00b4_2180,
            &args![at, PROXY, first + 8 * index, count, slot],
        );
    }

    #[test]
    fn inserting_into_a_full_vector_allocates_copies_around_the_value_and_frees_the_old_block() {
        let mut e = engine();
        install_pair_storage(&mut e);
        let this = e.mem.alloc(0x18);
        let data = pair_vector(&mut e, this, &[(1, 10), (2, 20)], 2);
        insert(&mut e, this, 1, 1, (9, 90));
        assert_eq!(pairs(&e, this), vec![(1, 10), (9, 90), (2, 20)]);
        assert_eq!(e.mem.block_size(data), None);
        // 2 + 2 / 2 = 3 pairs of capacity.
        assert_eq!(e.mem.u32(this + 0x14) - e.mem.u32(this + 0x0C), 24);
        // An empty vector without storage needs one pair.
        let empty = e.mem.alloc(0x18);
        e.mem.set_u32(empty, PROXY);
        e.call(
            0x00b4_2180,
            &args![empty, PROXY, 0u32, 1u32, e.mem.u32(this + 0x0C)],
        );
        assert_eq!(pairs(&e, empty), vec![(1, 10)]);
        assert_eq!(e.mem.u32(empty + 0x14) - e.mem.u32(empty + 0x0C), 8);
    }

    #[test]
    fn inserting_with_room_moves_the_tail_up_when_it_is_at_least_as_long_as_the_insert() {
        let mut e = engine();
        install_pair_storage(&mut e);
        let this = e.mem.alloc(0x18);
        pair_vector(&mut e, this, &[(1, 10), (2, 20)], 4);
        insert(&mut e, this, 0, 1, (7, 70));
        assert_eq!(pairs(&e, this), vec![(7, 70), (1, 10), (2, 20)]);
        // Inserting two at the end copies nothing but the new pairs.
        insert(&mut e, this, 3, 1, (8, 80));
        assert_eq!(pairs(&e, this), vec![(7, 70), (1, 10), (2, 20), (8, 80)]);
    }

    #[test]
    fn inserting_more_than_the_tail_holds_constructs_the_surplus_past_the_old_end() {
        let mut e = engine();
        install_pair_storage(&mut e);
        let this = e.mem.alloc(0x18);
        pair_vector(&mut e, this, &[(1, 10)], 4);
        insert(&mut e, this, 0, 2, (7, 70));
        assert_eq!(pairs(&e, this), vec![(7, 70), (7, 70), (1, 10)]);
        // Three new pairs after one element into room for four.
        let other = e.mem.alloc(0x18);
        pair_vector(&mut e, other, &[(5, 50)], 4);
        insert(&mut e, other, 1, 3, (6, 60));
        assert_eq!(pairs(&e, other), vec![(5, 50), (6, 60), (6, 60), (6, 60)]);
    }
}

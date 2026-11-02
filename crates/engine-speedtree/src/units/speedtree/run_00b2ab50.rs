//! `SpeedTree/run_00b2ab50` (Xbox PDB source unit), subsystem `SpeedTree`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The checked iterator of a `std::vector` of 32-bit elements (dereference, increment and
//! comparison, as the exe instantiates them for the tree engine's vectors) and the
//! `CTreeEngine` constructor.
//!
//! Functions that carry an `FS:[0]` exception frame are marked `Translation incomplete` (the
//! unwinding handler is not translated).

#[allow(unused_imports)]
use crate::prelude::*;

use super::branch::INVALID_PARAMETER;
use super::treeengine::CTreeEngine;

/// `CIdvCamera::CIdvCamera` (`00b3f030`), `CIdvRandom::CIdvRandom` (`00b3f100`),
/// `SIdvTreeInfo::SIdvTreeInfo` (`00b40a60`), the 32-bit element vector's default constructor
/// (`00b17db0`), `SIdvLeafInfo::SIdvLeafInfo` (`00b3cd60`) and `SIdvWindInfo::SIdvWindInfo`
/// (`00b40850`), each `thiscall` with the object as `this`.
const CAMERA_CONSTRUCT: u32 = 0x00b3_f030;
const RANDOM_CONSTRUCT: u32 = 0x00b3_f100;
const TREE_INFO_CONSTRUCT: u32 = 0x00b4_0a60;
const WORD_VECTOR_CONSTRUCT: u32 = 0x00b1_7db0;
const LEAF_INFO_CONSTRUCT: u32 = 0x00b3_cd60;
const WIND_INFO_CONSTRUCT: u32 = 0x00b4_0850;
/// The wind strength setter of `CTreeEngine` (`00b2bc70`, `thiscall`, one `float`).
const SET_WIND_STRENGTH: u32 = 0x00b2_bc70;
/// The vtable of `CTreeEngine` (`010a5670`).
const TREE_ENGINE_VTABLE: u32 = 0x010a_5670;
/// The `float` constants the constructor stores: `-1.0` (`01012054`), `0.5` (`01016248`), `0.3`
/// (`01017870`), `0.05` (`010181bc`) and the starting wind strength `0.25` (`0101622c`).
const MINUS_ONE: u32 = 0x0101_2054;
const HALF: u32 = 0x0101_6248;
const THREE_TENTHS: u32 = 0x0101_7870;
const FIVE_HUNDREDTHS: u32 = 0x0101_81bc;
const QUARTER: u32 = 0x0101_622c;

// Translated from 00b2ab50 (decompiled, FalloutNV.exe 1.4.0.525)
/// The checked dereference of an iterator of a 32-bit element vector (`operator*`, as the exe
/// instantiates it; the engine map gives it no name; `this` = the iterator: proxy at `+0`,
/// position at `+4`): a null proxy takes the invalid-parameter path; so does a position that is
/// not below the vector's last pointer (the vector is the first word of the proxy). Returns the
/// position (the element's address).
pub fn fn_00b2ab50(e: &mut Engine, this: Ptr) -> u32 {
    let it = this.addr();
    if e.mem.u32(it) == 0 {
        e.call(INVALID_PARAMETER, &args![]);
    }
    let container = if e.mem.u32(it) != 0 {
        e.mem.u32(e.mem.u32(it))
    } else {
        0
    };
    if e.mem.u32(it + 4) >= e.mem.u32(container.wrapping_add(0x10)) {
        e.call(INVALID_PARAMETER, &args![]);
    }
    e.mem.u32(it + 4)
}

// Translated from 00b2abc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The checked increment of an iterator of a 32-bit element vector (`operator++`, as the exe
/// instantiates it; the engine map gives it no name): the checks of `00b2ab50`, then the
/// position moves on by one element (4 bytes). Returns `this`.
pub fn fn_00b2abc0(e: &mut Engine, this: Ptr) -> Ptr {
    let it = this.addr();
    if e.mem.u32(it) == 0 {
        e.call(INVALID_PARAMETER, &args![]);
    }
    let container = if e.mem.u32(it) != 0 {
        e.mem.u32(e.mem.u32(it))
    } else {
        0
    };
    if e.mem.u32(it + 4) >= e.mem.u32(container.wrapping_add(0x10)) {
        e.call(INVALID_PARAMETER, &args![]);
    }
    let position = e.mem.u32(it + 4).wrapping_add(4);
    e.mem.set_u32(it + 4, position);
    this
}

// Translated from 00b2ac30 (decompiled, FalloutNV.exe 1.4.0.525)
/// The comparison of two iterators of a 32-bit element vector (`operator==`, as the exe
/// instantiates it; the engine map gives it no name): a null proxy in `this`, or proxies that
/// differ, take the invalid-parameter path; the answer is whether the positions are equal.
pub fn fn_00b2ac30(e: &mut Engine, this: Ptr, other: Ptr) -> bool {
    let it = this.addr();
    let rhs = other.addr();
    if e.mem.u32(it) == 0 || e.mem.u32(it) != e.mem.u32(rhs) {
        e.call(INVALID_PARAMETER, &args![]);
    }
    e.mem.u32(it + 4) == e.mem.u32(rhs + 4)
}

// Translated from 00b2ac80 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b2ac80 - the FS:[0] exception frame (the handler that destroys the
// constructed members if a later step throws) is not translated.
/// `CTreeEngine::CTreeEngine` (the engine map gives it no name; the one argument is the
/// `CIndexedGeometry` the engine writes into): constructs the `CIdvCamera` base (`00b3f030`)
/// and sets the vtable, a billboard size and lod of 1.0, the override tree size of -1.0 and
/// variance 0, constructs the `CIdvRandom` (`00b3f100`), sets `m_bTransientDataIntact`,
/// constructs the tree info (`00b40a60`), clears the branch and geometry pointers, constructs
/// the branch info vector (`00b17db0`), sets 6 branch lod levels, constructs the billboard leaf
/// vector (`00b17db0`) and the leaf info (`00b3cd60`), clears the leaf lod array and its
/// parsed flag, sets the volume percents (0.5 and 1.0), the leaf reduction percentage (0.3), the
/// branch reduction fuzziness (0), the large branch percent (0.05) and the weight level (1),
/// constructs the wind info (`00b40850`), stores the geometry and sets the wind strength to
/// 0.25 (`00b2bc70`). Returns `this`.
pub fn fn_00b2ac80(e: &mut Engine, this: Ptr<CTreeEngine>, geometry: u32) -> Ptr<CTreeEngine> {
    let t = this.addr();
    e.call(CAMERA_CONSTRUCT, &args![this]);
    e.mem.set_u32(t, TREE_ENGINE_VTABLE);
    e.mem.set_f32(t + 0x10, 1.0);
    e.mem.set_f32(t + 0x14, 1.0);
    let minus_one = x87_quiet_bits(e.mem.u32(MINUS_ONE));
    e.mem.set_u32(t + 0x18, minus_one);
    e.mem.set_f32(t + 0x1C, 0.0);
    e.call(RANDOM_CONSTRUCT, &args![t + 0x20]);
    e.mem.set_u8(t + 0x21, 1);
    e.call(TREE_INFO_CONSTRUCT, &args![t + 0x24]);
    e.mem.set_u32(t + 0x58, 0);
    e.mem.set_u32(t + 0x5C, 0);
    e.call(WORD_VECTOR_CONSTRUCT, &args![t + 0x60]);
    e.mem.set_u32(t + 0x78, 6);
    e.call(WORD_VECTOR_CONSTRUCT, &args![t + 0x7C]);
    e.call(LEAF_INFO_CONSTRUCT, &args![t + 0x94]);
    e.mem.set_u32(t + 0xEC, 0);
    e.mem.set_u8(t + 0xF0, 0);
    let half = x87_quiet_bits(e.mem.u32(HALF));
    e.mem.set_u32(t + 0xF4, half);
    e.mem.set_f32(t + 0xF8, 1.0);
    let three_tenths = x87_quiet_bits(e.mem.u32(THREE_TENTHS));
    e.mem.set_u32(t + 0xFC, three_tenths);
    e.mem.set_f32(t + 0x100, 0.0);
    let five_hundredths = x87_quiet_bits(e.mem.u32(FIVE_HUNDREDTHS));
    e.mem.set_u32(t + 0x104, five_hundredths);
    e.mem.set_u32(t + 0x108, 1);
    e.call(WIND_INFO_CONSTRUCT, &args![t + 0x10C]);
    e.mem.set_u32(t + 0x5C, geometry);
    let quarter = x87_quiet(e.global::<f32>(QUARTER));
    e.call(SET_WIND_STRENGTH, &args![this, quarter]);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00b2ab50, fn_00b2ab50(Ptr) -> u32),
        entry!(0x00b2abc0, fn_00b2abc0(Ptr) -> Ptr),
        entry!(0x00b2ac30, fn_00b2ac30(Ptr, Ptr) -> bool),
        entry!(
            0x00b2ac80,
            fn_00b2ac80(Ptr<CTreeEngine>, u32) -> Ptr<CTreeEngine>
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    /// An engine whose `INVALID_PARAMETER` panics.
    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0020_0000);
        e.call_log = Some(Vec::new());
        e.register(INVALID_PARAMETER, |_, _| -> Ret {
            panic!("invalid parameter")
        });
        e
    }

    /// A vector whose last pointer is `last`, a proxy that points at it and an iterator (proxy,
    /// position); answers the iterator.
    fn iterator(e: &mut Engine, last: u32, position: u32) -> u32 {
        let vector = e.mem.alloc(0x18);
        e.mem.set_u32(vector + 0x10, last);
        let proxy = e.mem.alloc(4);
        e.mem.set_u32(proxy, vector);
        let it = e.mem.alloc(8);
        e.mem.set_u32(it, proxy);
        e.mem.set_u32(it + 4, position);
        it
    }

    #[test]
    fn the_dereference_answers_the_position_below_the_last_pointer() {
        let mut e = engine();
        let it = iterator(&mut e, 0x2000, 0x1ffc);
        assert_eq!(e.call(0x00b2_ab50, &args![it]).u32(), 0x1ffc);
    }

    #[test]
    #[should_panic(expected = "invalid parameter")]
    fn the_dereference_rejects_the_end_position() {
        let mut e = engine();
        let it = iterator(&mut e, 0x2000, 0x2000);
        e.call(0x00b2_ab50, &args![it]);
    }

    #[test]
    #[should_panic(expected = "invalid parameter")]
    fn the_dereference_rejects_an_iterator_without_a_proxy() {
        let mut e = engine();
        let it = e.mem.alloc(8);
        e.call(0x00b2_ab50, &args![it]);
    }

    #[test]
    fn the_increment_moves_the_position_on_by_one_element() {
        let mut e = engine();
        let it = iterator(&mut e, 0x2000, 0x1ff8);
        assert_eq!(e.call(0x00b2_abc0, &args![it]).u32(), it);
        assert_eq!(e.mem.u32(it + 4), 0x1ffc);
    }

    #[test]
    #[should_panic(expected = "invalid parameter")]
    fn the_increment_rejects_the_end_position() {
        let mut e = engine();
        let it = iterator(&mut e, 0x2000, 0x2000);
        e.call(0x00b2_abc0, &args![it]);
    }

    #[test]
    fn the_equality_compares_the_positions_of_iterators_of_one_vector() {
        let mut e = engine();
        let it = iterator(&mut e, 0x2000, 0x1ff8);
        let same = e.mem.alloc(8);
        let proxy = e.mem.u32(it);
        e.mem.set_u32(same, proxy);
        e.mem.set_u32(same + 4, 0x1ff8);
        assert!(e.call(0x00b2_ac30, &args![it, same]).bool());
        e.mem.set_u32(same + 4, 0x1ffc);
        assert!(!e.call(0x00b2_ac30, &args![it, same]).bool());
    }

    #[test]
    #[should_panic(expected = "invalid parameter")]
    fn the_equality_rejects_iterators_of_different_vectors() {
        let mut e = engine();
        let it = iterator(&mut e, 0x2000, 0x1ff8);
        let other = iterator(&mut e, 0x2000, 0x1ff8);
        e.call(0x00b2_ac30, &args![it, other]);
    }

    #[test]
    fn the_constructor_builds_the_members_and_sets_the_defaults() {
        let mut e = engine();
        for address in [
            CAMERA_CONSTRUCT,
            RANDOM_CONSTRUCT,
            TREE_INFO_CONSTRUCT,
            WORD_VECTOR_CONSTRUCT,
            LEAF_INFO_CONSTRUCT,
            WIND_INFO_CONSTRUCT,
            SET_WIND_STRENGTH,
        ] {
            e.register(address, |_, _| Ret::default());
        }
        e.set_global::<f32>(MINUS_ONE, -1.0);
        e.set_global::<f32>(HALF, 0.5);
        e.set_global::<f32>(THREE_TENTHS, 0.3);
        e.set_global::<f32>(FIVE_HUNDREDTHS, 0.05);
        e.set_global::<f32>(QUARTER, 0.25);
        let t = e.mem.alloc(0x128);
        for offset in (0x10..0x128).step_by(4) {
            e.mem.set_u32(t + offset, 0xdead_beef);
        }
        assert_eq!(e.call(0x00b2_ac80, &args![t, 0x7000u32]).u32(), t);
        assert_eq!(e.mem.u32(t), TREE_ENGINE_VTABLE);
        assert_eq!(e.mem.f32(t + 0x10), 1.0);
        assert_eq!(e.mem.f32(t + 0x14), 1.0);
        assert_eq!(e.mem.f32(t + 0x18), -1.0);
        assert_eq!(e.mem.f32(t + 0x1C), 0.0);
        assert_eq!(e.mem.u8(t + 0x21), 1);
        assert_eq!(e.mem.u32(t + 0x58), 0);
        assert_eq!(e.mem.u32(t + 0x5C), 0x7000);
        assert_eq!(e.mem.u32(t + 0x78), 6);
        assert_eq!(e.mem.u32(t + 0xEC), 0);
        assert_eq!(e.mem.u8(t + 0xF0), 0);
        assert_eq!(e.mem.f32(t + 0xF4), 0.5);
        assert_eq!(e.mem.f32(t + 0xF8), 1.0);
        assert_eq!(e.mem.f32(t + 0xFC), 0.3);
        assert_eq!(e.mem.f32(t + 0x100), 0.0);
        assert_eq!(e.mem.f32(t + 0x104), 0.05);
        assert_eq!(e.mem.u32(t + 0x108), 1);
        let order: Vec<(u32, Vec<u32>)> = e.call_log.clone().unwrap().into_iter().skip(1).collect();
        assert_eq!(
            order,
            vec![
                (CAMERA_CONSTRUCT, vec![t]),
                (RANDOM_CONSTRUCT, vec![t + 0x20]),
                (TREE_INFO_CONSTRUCT, vec![t + 0x24]),
                (WORD_VECTOR_CONSTRUCT, vec![t + 0x60]),
                (WORD_VECTOR_CONSTRUCT, vec![t + 0x7C]),
                (LEAF_INFO_CONSTRUCT, vec![t + 0x94]),
                (WIND_INFO_CONSTRUCT, vec![t + 0x10C]),
                (SET_WIND_STRENGTH, vec![t, 0.25f32.to_bits()]),
            ]
        );
    }
}

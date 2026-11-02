//! `Havok SDK/hkplaneequationutil.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `hkgpConvexHull::hkgpConvexHull` (Xbox PDB; `this` = the 0x14-byte hull).
const CONVEX_HULL_CONSTRUCT: u32 = 0x00d7_6ca0;
/// `hkgpConvexHull::BuildConfig::BuildConfig` (Xbox PDB; `this` = the configuration, answers
/// `this`).
const BUILD_CONFIG_CONSTRUCT: u32 = 0x00d7_4d40;
/// `hkgpConvexHull::buildFromPlanes` (Xbox PDB; `this` = the hull, then `(planes, count,
/// configuration)`; answers -1 when it fails).
const BUILD_FROM_PLANES: u32 = 0x00d7_74a0;
/// `hkgpConvexHull::fetchPositions` (Xbox PDB; `this` = the hull, then `(1, out array)`).
const FETCH_POSITIONS: u32 = 0x00d7_6120;
/// `hkgpConvexHull::~hkgpConvexHull` (Xbox PDB; `this` = the hull).
const CONVEX_HULL_DESTRUCT: u32 = 0x00d7_6940;

// Translated from 00dd6270 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkPlaneEquationUtil::getConvexVerticesFromPlaneEquations` (Xbox PDB): `planes` is an
/// `hkArray` of plane equations (data at `+0`, count at `+4`). With fewer than 4 planes the
/// answer is 1. Otherwise builds a convex hull (a local `hkgpConvexHull` with a default
/// `BuildConfig`, `buildFromPlanes`); when that fails (-1) the hull is destroyed and the answer
/// is 1; when it works the hull's vertices are fetched into the array `out`
/// (`fetchPositions(1, out)`), the hull is destroyed and the answer is 0.
pub fn get_convex_vertices_from_plane_equations(e: &mut Engine, planes: Ptr, out: Ptr) -> u32 {
    let count = e.mem.i32(planes.addr() + 4);
    if count < 4 {
        return 1;
    }
    // The exe's frame: the build configuration (0x10 bytes) below the hull (0x14 bytes).
    e.with_stack(0x24, |e, frame| {
        let config = frame.addr();
        let hull = frame.addr() + 0x10;
        e.call(CONVEX_HULL_CONSTRUCT, &args![hull]);
        let data = e.mem.u32(planes.addr());
        let count = e.mem.u32(planes.addr() + 4);
        let config = e.call(BUILD_CONFIG_CONSTRUCT, &args![config]).u32();
        let built = e
            .call(BUILD_FROM_PLANES, &args![hull, data, count, config])
            .i32();
        if built == -1 {
            e.call(CONVEX_HULL_DESTRUCT, &args![hull]);
            return 1;
        }
        e.call(FETCH_POSITIONS, &args![hull, 1u32, out]);
        e.call(CONVEX_HULL_DESTRUCT, &args![hull]);
        0
    })
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(
        0x00dd_6270,
        get_convex_vertices_from_plane_equations(Ptr, Ptr) -> u32
    )]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    type Calls = Rc<RefCell<Vec<(u32, Vec<u32>)>>>;

    /// Doubles for the hull functions; `build_result` is what `buildFromPlanes` answers.
    fn hull_doubles(e: &mut Engine, build_result: i32) -> Calls {
        let calls: Calls = Rc::default();
        for address in [
            CONVEX_HULL_CONSTRUCT,
            BUILD_FROM_PLANES,
            FETCH_POSITIONS,
            CONVEX_HULL_DESTRUCT,
        ] {
            let sink = Rc::clone(&calls);
            e.register_double(address, move |_, a| {
                sink.borrow_mut().push((address, a.to_vec()));
                Ret {
                    eax: if address == BUILD_FROM_PLANES {
                        build_result as u32
                    } else {
                        0
                    },
                    ..Ret::default()
                }
            });
        }
        let sink = Rc::clone(&calls);
        e.register_double(BUILD_CONFIG_CONSTRUCT, move |_, a| {
            sink.borrow_mut().push((BUILD_CONFIG_CONSTRUCT, a.to_vec()));
            Ret {
                eax: a[0],
                ..Ret::default()
            }
        });
        calls
    }

    fn planes(e: &mut Engine, count: u32) -> u32 {
        let planes = e.mem.alloc(0x10);
        e.mem.set_u32(planes, 0x7000);
        e.mem.set_u32(planes + 4, count);
        planes
    }

    #[test]
    fn fewer_than_four_planes_are_refused_without_building_a_hull() {
        let mut e = Engine::new();
        let calls = hull_doubles(&mut e, 0);
        let planes = planes(&mut e, 3);
        assert_eq!(e.call(0x00dd_6270, &args![planes, 0x8000u32]).u32(), 1);
        assert!(calls.borrow().is_empty());
    }

    #[test]
    fn a_built_hull_hands_over_its_vertices() {
        let mut e = Engine::new();
        let calls = hull_doubles(&mut e, 5);
        let planes = planes(&mut e, 4);
        assert_eq!(e.call(0x00dd_6270, &args![planes, 0x8000u32]).u32(), 0);
        let calls = calls.borrow();
        let order: Vec<u32> = calls.iter().map(|(address, _)| *address).collect();
        assert_eq!(
            order,
            vec![
                CONVEX_HULL_CONSTRUCT,
                BUILD_CONFIG_CONSTRUCT,
                BUILD_FROM_PLANES,
                FETCH_POSITIONS,
                CONVEX_HULL_DESTRUCT
            ]
        );
        let hull = calls[0].1[0];
        let config = calls[1].1[0];
        assert_eq!(hull, config + 0x10);
        assert_eq!(calls[2].1, vec![hull, 0x7000, 4, config]);
        assert_eq!(calls[3].1, vec![hull, 1, 0x8000]);
        assert_eq!(calls[4].1, vec![hull]);
    }

    #[test]
    fn a_failed_build_destroys_the_hull_and_answers_one() {
        let mut e = Engine::new();
        let calls = hull_doubles(&mut e, -1);
        let planes = planes(&mut e, 6);
        assert_eq!(e.call(0x00dd_6270, &args![planes, 0x8000u32]).u32(), 1);
        let order: Vec<u32> = calls.borrow().iter().map(|(address, _)| *address).collect();
        assert_eq!(
            order,
            vec![
                CONVEX_HULL_CONSTRUCT,
                BUILD_CONFIG_CONSTRUCT,
                BUILD_FROM_PLANES,
                CONVEX_HULL_DESTRUCT
            ]
        );
    }
}

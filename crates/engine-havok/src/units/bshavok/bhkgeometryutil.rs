//! `BSHavok/bhkgeometryutil.obj` (Xbox PDB source unit), subsystem `BSHavok`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The one function carries an `FS:[0]` exception-unwinding frame that is not modeled.

#[allow(unused_imports)]
use crate::prelude::*;

/// `hkGeometryUtility::createConvexGeometry_ov2` (Xbox PDB, `00d5a630`, cdecl `(input, geometry,
/// word)`) and `hkGeometry::~hkGeometry` (`00cce530`, `this` = the geometry).
const CREATE_CONVEX_GEOMETRY: u32 = 0x00d5_a630;
const GEOMETRY_DESTRUCTOR: u32 = 0x00cc_e530;

// Translated from 00d5a510 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00d5a510 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkGeometryUtil::ComputePlanesFromVertices` (Xbox PDB), cdecl `(vertices, word, planes)`:
/// builds an input record on the stack from the two words at `vertices` and the value 0x10
/// and a geometry made of two empty arrays (data 0, size 0, capacity word 0x80000000), runs
/// `createConvexGeometry_ov2` (`00d5a630`) on `(input, geometry, word)`, swaps the three
/// words of the geometry's first array with the three words at `planes` and runs the
/// geometry's destructor (`00cce530`), which therefore releases what `planes` held before.
pub fn bhk_geometry_util_compute_planes_from_vertices(
    e: &mut Engine,
    vertices: Ptr,
    word: u32,
    planes: Ptr,
) {
    e.with_stack(0x30, |e, frame| {
        let input = frame.addr();
        let geometry = frame.addr() + 0xc;
        let first = e.mem.u32(vertices.addr());
        let second = e.mem.u32(vertices.addr() + 4);
        e.mem.set_u32(input, first);
        e.mem.set_u32(input + 4, second);
        e.mem.set_u32(input + 8, 0x10);
        for array in [geometry, geometry + 0xc] {
            e.mem.set_u32(array, 0);
            e.mem.set_u32(array + 4, 0);
            e.mem.set_u32(array + 8, 0x8000_0000);
        }
        e.call(CREATE_CONVEX_GEOMETRY, &args![input, geometry, word]);
        for offset in [0u32, 4, 8] {
            let made = e.mem.u32(geometry + offset);
            let old = e.mem.u32(planes.addr() + offset);
            e.mem.set_u32(geometry + offset, old);
            e.mem.set_u32(planes.addr() + offset, made);
        }
        e.call(GEOMETRY_DESTRUCTOR, &args![geometry]);
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(
        0x00d5a510,
        bhk_geometry_util_compute_planes_from_vertices(Ptr, u32, Ptr)
    )]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    type Seen = Rc<RefCell<Vec<Vec<u32>>>>;

    #[test]
    fn compute_planes_swaps_the_made_array_with_the_old_planes() {
        let mut e = Engine::new();
        let vertices = e.mem.alloc(8);
        e.mem.set_u32(vertices, 0xaaa0);
        e.mem.set_u32(vertices + 4, 7);
        let planes = e.mem.alloc(12);
        for (lane, value) in [0x11u32, 0x22, 0x33].into_iter().enumerate() {
            e.mem.set_u32(planes + 4 * lane as u32, value);
        }
        let seen: Seen = Rc::new(RefCell::new(vec![]));
        let seen_make = seen.clone();
        e.register_double(CREATE_CONVEX_GEOMETRY, move |e, a| {
            let mut row = vec![a[2]];
            for offset in 0..3u32 {
                row.push(e.mem.u32(a[0] + 4 * offset));
            }
            // Both arrays of the geometry start empty.
            for offset in 0..6u32 {
                row.push(e.mem.u32(a[1] + 4 * offset));
            }
            seen_make.borrow_mut().push(row);
            for (lane, value) in [0x101u32, 0x202, 0x303].into_iter().enumerate() {
                e.mem.set_u32(a[1] + 4 * lane as u32, value);
            }
            Ret::default()
        });
        let seen_free = seen.clone();
        e.register_double(GEOMETRY_DESTRUCTOR, move |e, a| {
            let row = (0..3u32).map(|lane| e.mem.u32(a[0] + 4 * lane)).collect();
            seen_free.borrow_mut().push(row);
            Ret::default()
        });
        e.call(0x00d5a510, &args![vertices, 0x55u32, planes]);
        let rows = seen.borrow();
        assert_eq!(
            rows[0],
            [0x55, 0xaaa0, 7, 0x10, 0, 0, 0x8000_0000, 0, 0, 0x8000_0000]
        );
        assert_eq!(rows[1], [0x11, 0x22, 0x33]);
        for (lane, value) in [0x101u32, 0x202, 0x303].into_iter().enumerate() {
            assert_eq!(e.mem.u32(planes + 4 * lane as u32), value);
        }
    }
}

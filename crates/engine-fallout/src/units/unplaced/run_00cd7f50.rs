//! `(unplaced)/run_00cd7f50` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// Per-thread word (`TLS + 0x2b4`) that the allocating functions set to
/// 0x10 around their work.
const SCOPE_WORD: u32 = 0x2b4;
/// `float` 0.1 (`010c72bc`), the radius the local shape description starts
/// with.
const DEFAULT_RADIUS: u32 = 0x010c_72bc;
/// `bhkGeometryUtil::ComputePlanesFromVertices` (engine map), `00d5a510`
/// (`cdecl`: source, planes array, vertices array).
const COMPUTE_PLANES: u32 = 0x00d5_a510;
/// The game's allocator (`00aa13e0`, size).
const ALLOCATE: u32 = 0x00aa_13e0;
/// The shape constructor (`00cd7e80`, `bhkcharcontrollershape.obj`;
/// `this`, description).
const SHAPE_CONSTRUCT: u32 = 0x00cd_7e80;
/// The description's destructor (`00cce290`, `bhkconvexverticesshape.obj`).
const DESCRIPTION_DESTROY: u32 = 0x00cc_e290;

// Translated from 00cd7f50 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00cd7f50 - the compiler's exception frame (FS:[0], with the /GS cookie mixed into its record) and the unwind states it tracks are not translated
/// Builds a convex shape from a source of vertices (`cdecl`; no Xbox PDB
/// name): with the thread's scope word set to 0x10, a local description
/// (0x20 bytes: a zero word, the radius 0.1 from `010c72bc`, then two
/// empty Havok arrays, each data 0, size 0, capacity flags `0x80000000`
/// at +8 and +0x14) is filled when `source` +4 is not null:
/// `ComputePlanesFromVertices(source, &array at +0x14, &array at +8)`,
/// then a 0x14-byte block is allocated and, if not null, constructed by
/// `00cd7e80` from the description. The description is destroyed
/// (`00cce290`), the scope word restored, and the shape (or 0) returned.
pub fn fn_00cd7f50(e: &mut Engine, source: Ptr) -> u32 {
    let tls = e.tls();
    let saved = e.mem.u32(tls + SCOPE_WORD);
    e.mem.set_u32(tls + SCOPE_WORD, 0x10);
    let shape = e.with_stack(0x20, |e, description| {
        let d = description.addr();
        e.mem.set_u32(d, 0);
        e.mem.set_u32(d + 8, 0);
        e.mem.set_u32(d + 0xc, 0);
        e.mem.set_u32(d + 0x10, 0x8000_0000);
        e.mem.set_u32(d + 0x14, 0);
        e.mem.set_u32(d + 0x18, 0);
        e.mem.set_u32(d + 0x1c, 0x8000_0000);
        let radius = x87_store32(x87_load32(e.mem.u32(DEFAULT_RADIUS)));
        e.mem.set_u32(d + 4, radius);
        let mut shape = 0;
        if e.mem.u32(source.addr() + 4) != 0 {
            e.call(COMPUTE_PLANES, &args![source, d + 0x14, d + 8]);
            let block = e.call(ALLOCATE, &args![0x14u32]).u32();
            shape = if block == 0 {
                0
            } else {
                e.call(SHAPE_CONSTRUCT, &args![block, d]).u32()
            };
        }
        e.call(DESCRIPTION_DESTROY, &args![d]);
        shape
    });
    e.mem.set_u32(tls + SCOPE_WORD, saved);
    shape
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00cd7f50, fn_00cd7f50(Ptr) -> u32)]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x010c_7000, 0x1000);
        e.set_global(DEFAULT_RADIUS, 0x3dcc_cccdu32);
        e.register(COMPUTE_PLANES, |_, _| Ret::default());
        e.register(DESCRIPTION_DESTROY, |_, _| Ret::default());
        e.register(SHAPE_CONSTRUCT, |e, a| {
            // The description as the constructor sees it.
            assert_eq!(e.mem.u32(a[1] + 4), 0x3dcc_cccd);
            assert_eq!(e.mem.u32(a[1] + 0x10), 0x8000_0000);
            assert_eq!(e.mem.u32(a[1] + 0x1c), 0x8000_0000);
            Ret {
                eax: a[0],
                ..Ret::default()
            }
        });
        e
    }

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        let log = e.call_log.as_ref().unwrap();
        log.iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    #[test]
    fn builds_the_shape_from_the_planes_of_the_vertices() {
        let mut e = engine();
        e.register(ALLOCATE, |_, _| Ret {
            eax: 0x7000,
            ..Ret::default()
        });
        let source: Ptr = Ptr::new(e.mem.alloc(8));
        e.mem.set_u32(source.addr() + 4, 1);
        let tls = e.tls();
        e.mem.set_u32(tls + SCOPE_WORD, 5);
        e.call_log = Some(vec![]);
        let shape = e.call(0x00cd_7f50, &args![source]).u32();
        assert_eq!(shape, 0x7000);
        assert_eq!(e.mem.u32(tls + SCOPE_WORD), 5);
        let planes = calls(&e, COMPUTE_PLANES);
        let d = calls(&e, DESCRIPTION_DESTROY)[0][0];
        assert_eq!(planes, vec![vec![source.addr(), d + 0x14, d + 8]]);
        assert_eq!(calls(&e, ALLOCATE), vec![vec![0x14]]);
        assert_eq!(calls(&e, SHAPE_CONSTRUCT), vec![vec![0x7000, d]]);
    }

    #[test]
    fn a_failed_allocation_returns_null() {
        let mut e = engine();
        e.register(ALLOCATE, |_, _| Ret::default());
        let source: Ptr = Ptr::new(e.mem.alloc(8));
        e.mem.set_u32(source.addr() + 4, 1);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00cd_7f50, &args![source]).u32(), 0);
        assert!(calls(&e, SHAPE_CONSTRUCT).is_empty());
    }

    #[test]
    fn without_vertices_only_the_description_is_destroyed() {
        let mut e = engine();
        let source: Ptr = Ptr::new(e.mem.alloc(8));
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00cd_7f50, &args![source]).u32(), 0);
        assert!(calls(&e, COMPUTE_PLANES).is_empty());
        assert_eq!(calls(&e, DESCRIPTION_DESTROY).len(), 1);
    }
}

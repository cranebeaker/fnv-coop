//! `fallout shared/pathfinding/run_006f30e0` (Xbox PDB source unit), subsystem `fallout shared/pathfinding`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! Constructors of three `BSSimpleArray<T *, 1024>` instances of the path
//! smoother (the RTTI names of their vtables) and the array's "sparse enough
//! to shrink" test.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::types::BSSimpleArray;

/// `006b3eb0(this, capacity, size)`: constructor body of the
/// `BSSimpleArray<T *, 1024>`.
const ARRAY_INIT: u32 = 0x006b_3eb0;
/// Vtable of `BSSimpleArray<PathSmoother::PathEdge *, 1024>` (RTTI name).
const VTABLE_PATH_EDGE: u32 = 0x0106_d800;
/// Vtable of `BSSimpleArray<PathSmoother::UniqueVert *, 1024>` (RTTI name).
const VTABLE_UNIQUE_VERT: u32 = 0x0106_d828;
/// Vtable of `BSSimpleArray<PathSmoother::BoundsEdge *, 1024>` (RTTI name).
const VTABLE_BOUNDS_EDGE: u32 = 0x0106_d850;

/// Stores the vtable and runs the array constructor body with an empty
/// capacity and size.
fn construct(e: &mut Engine, this: Ptr<BSSimpleArray>, vtable: u32) -> Ptr<BSSimpleArray> {
    e.mem.set_u32(this.addr(), vtable);
    e.call(ARRAY_INIT, &args![this, 0u32, 0u32]);
    this
}

// Translated from 006f30e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of `BSSimpleArray<PathSmoother::PathEdge *, 1024>` (the RTTI
/// name of the vtable it stores; no Xbox PDB name is mapped). Returns `this`.
pub fn fn_006f30e0(e: &mut Engine, this: Ptr<BSSimpleArray>) -> Ptr<BSSimpleArray> {
    construct(e, this, VTABLE_PATH_EDGE)
}

// Translated from 006f3110 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of `BSSimpleArray<PathSmoother::UniqueVert *, 1024>` (the RTTI
/// name of the vtable it stores; no Xbox PDB name is mapped). Returns `this`.
pub fn fn_006f3110(e: &mut Engine, this: Ptr<BSSimpleArray>) -> Ptr<BSSimpleArray> {
    construct(e, this, VTABLE_UNIQUE_VERT)
}

// Translated from 006f3140 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of `BSSimpleArray<PathSmoother::BoundsEdge *, 1024>` (the RTTI
/// name of the vtable it stores; no Xbox PDB name is mapped). Returns `this`.
pub fn fn_006f3140(e: &mut Engine, this: Ptr<BSSimpleArray>) -> Ptr<BSSimpleArray> {
    construct(e, this, VTABLE_BOUNDS_EDGE)
}

// Translated from 006f3170 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether the array is sparse enough to shrink. With more than 0x400
/// elements reserved: the size is at most the reserved size minus 0x800;
/// otherwise: the size is at most a quarter of the reserved size (unsigned
/// comparisons).
pub fn fn_006f3170(e: &mut Engine, this: Ptr<BSSimpleArray>) -> bool {
    let reserved = e.get(this, BSSimpleArray::iReservedSize);
    let size = e.get(this, BSSimpleArray::iSize);
    if reserved > 0x400 {
        size <= reserved.wrapping_sub(0x800)
    } else {
        size <= reserved >> 2
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x006f30e0,
            fn_006f30e0(Ptr<BSSimpleArray>) -> Ptr<BSSimpleArray>
        ),
        entry!(
            0x006f3110,
            fn_006f3110(Ptr<BSSimpleArray>) -> Ptr<BSSimpleArray>
        ),
        entry!(
            0x006f3140,
            fn_006f3140(Ptr<BSSimpleArray>) -> Ptr<BSSimpleArray>
        ),
        entry!(0x006f3170, fn_006f3170(Ptr<BSSimpleArray>) -> bool),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constructors_store_their_vtable_and_run_the_array_constructor() {
        for (address, vtable) in [
            (0x006f30e0, VTABLE_PATH_EDGE),
            (0x006f3110, VTABLE_UNIQUE_VERT),
            (0x006f3140, VTABLE_BOUNDS_EDGE),
        ] {
            let mut e = Engine::new();
            e.register(ARRAY_INIT, |_, _| Ret::default());
            let this: Ptr<BSSimpleArray> = e.new_object();
            e.call_log = Some(vec![]);
            let result = e.call(address, &args![this]).ptr::<BSSimpleArray>();
            assert_eq!(result, this);
            assert_eq!(e.mem.u32(this.addr()), vtable);
            assert_eq!(
                e.call_log.clone().unwrap()[1],
                (ARRAY_INIT, vec![this.addr(), 0, 0])
            );
        }
    }

    fn can_shrink(size: u32, reserved: u32) -> bool {
        let mut e = Engine::new();
        let this: Ptr<BSSimpleArray> = e.new_object();
        e.set(this, BSSimpleArray::iSize, size);
        e.set(this, BSSimpleArray::iReservedSize, reserved);
        e.call(0x006f3170, &args![this]).bool()
    }

    #[test]
    fn small_arrays_shrink_at_a_quarter() {
        assert!(can_shrink(256, 1024));
        assert!(!can_shrink(257, 1024));
        assert!(can_shrink(0, 0));
    }

    #[test]
    fn large_arrays_shrink_when_0x800_are_free() {
        assert!(can_shrink(1, 0x801));
        assert!(!can_shrink(2, 0x801));
        assert!(!can_shrink(0x1000, 0x1800 - 1));
        assert!(can_shrink(0x1000, 0x1800));
    }
}

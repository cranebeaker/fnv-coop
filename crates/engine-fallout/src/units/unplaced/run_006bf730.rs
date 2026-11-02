//! `(unplaced)/run_006bf730` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The `BSSimpleArray<PortalEdgeSwap,1024>` (Xbox PDB) element size and buffer (`pBuffer`,
/// +4).
const ELEMENT_SIZE: u32 = 0x1c;
const BUFFER: u32 = 4;
/// The next free index, growing the buffer when needed (`006bfd40`, thiscall), the
/// element-range constructor (`006bfdd0`, thiscall: first element, count), the element's
/// assignment (`006bfc40`, thiscall: the source; in `navmeshobstaclecutter.cpp` by the
/// engine map), `operator new` (`00401000`, cdecl size) and the reallocation helper
/// `0042f5d0` (cdecl: block, size).
const NEXT_INDEX: u32 = 0x006b_fd40;
const CONSTRUCT_ELEMENTS: u32 = 0x006b_fdd0;
const ELEMENT_ASSIGN: u32 = 0x006b_fc40;
const OPERATOR_NEW: u32 = 0x0040_1000;
const REALLOCATE_0042F5D0: u32 = 0x0042_f5d0;

/// The address of element `index`.
fn element(e: &Engine, this: Ptr, index: u32) -> u32 {
    index
        .wrapping_mul(ELEMENT_SIZE)
        .wrapping_add(e.mem.u32(this.addr() + BUFFER))
}

// Translated from 006bf730 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the map (`thiscall`, `RET 4`; called from `006be950`): appends `value` to a
/// `BSSimpleArray<PortalEdgeSwap,1024>`: takes the next index (`006bfd40`), constructs one
/// element there (`006bfdd0`) and assigns `value` to it (`006bfc40`). Answers the index.
pub fn fn_006bf730(e: &mut Engine, this: Ptr, value: u32) -> u32 {
    let index = e.call(NEXT_INDEX, &args![this]).u32();
    let at = element(e, this, index);
    e.call(CONSTRUCT_ELEMENTS, &args![this, at, 1u32]);
    let at = element(e, this, index);
    e.call(ELEMENT_ASSIGN, &args![at, value]);
    index
}

// Translated from 006bf780 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<PortalEdgeSwap,1024>::_Allocate` (Xbox PDB, vtable +0x04; `thiscall`,
/// `RET 4`): `operator new(count * 0x1C)`.
pub fn bs_simple_array_portal_edge_swap_allocate(e: &mut Engine, _this: Ptr, count: u32) -> u32 {
    e.call(OPERATOR_NEW, &args![count.wrapping_mul(ELEMENT_SIZE)])
        .u32()
}

// Translated from 006bf7b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<PortalEdgeSwap,1024>::_Reallocate` (Xbox PDB, vtable +0x0C; `thiscall`,
/// `RET 8`): `0042f5d0(buffer, count * 0x1C)`, whose `EAX` is left as the result.
pub fn bs_simple_array_portal_edge_swap_reallocate(
    e: &mut Engine,
    _this: Ptr,
    buffer: u32,
    count: u32,
) -> u32 {
    e.call(
        REALLOCATE_0042F5D0,
        &args![buffer, count.wrapping_mul(ELEMENT_SIZE)],
    )
    .u32()
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x006b_f730, fn_006bf730(Ptr, u32) -> u32),
        entry!(
            0x006b_f780,
            bs_simple_array_portal_edge_swap_allocate(Ptr, u32) -> u32
        ),
        entry!(
            0x006b_f7b0,
            bs_simple_array_portal_edge_swap_reallocate(Ptr, u32, u32) -> u32
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn calls_to(e: &Engine, callee: u32) -> Vec<Vec<u32>> {
        let log = e.call_log.as_ref().unwrap();
        log.iter()
            .filter(|(address, _)| *address == callee)
            .map(|(_, words)| words.clone())
            .collect()
    }

    #[test]
    fn append_constructs_and_assigns_at_the_next_index() {
        let mut e = Engine::new();
        // The next index grows the buffer: the element is found through the new one.
        e.register(NEXT_INDEX, |e, a| {
            e.mem.set_u32(a[0] + 4, 0x8000);
            2u32.into_ret()
        });
        e.register(CONSTRUCT_ELEMENTS, |_, _| Ret::default());
        e.register(ELEMENT_ASSIGN, |_, a| a[0].into_ret());
        let array = e.mem.alloc(0x10);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x006b_f730, &args![array, 0x7000u32]).u32(), 2);
        assert_eq!(
            calls_to(&e, CONSTRUCT_ELEMENTS),
            vec![vec![array, 0x8038, 1]]
        );
        assert_eq!(calls_to(&e, ELEMENT_ASSIGN), vec![vec![0x8038, 0x7000]]);
    }

    #[test]
    fn allocate_and_reallocate_size_by_0x1c() {
        let mut e = Engine::new();
        e.register(OPERATOR_NEW, |_, a| (a[0] + 1).into_ret());
        e.register(REALLOCATE_0042F5D0, |_, a| (a[0] + a[1]).into_ret());
        assert_eq!(e.call(0x006b_f780, &args![0u32, 4u32]).u32(), 0x71);
        assert_eq!(
            e.call(0x006b_f7b0, &args![0u32, 0x1000u32, 2u32]).u32(),
            0x1038
        );
    }
}

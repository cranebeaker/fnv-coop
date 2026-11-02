//! `BSMain/run_00c46530` (Xbox PDB source unit), subsystem `BSMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiTArray<NiPointer<NiRefObject>, ..>` vtable (RTTI).
const VTABLE_TARRAY_REF_OBJECT: u32 = 0x010c_1c5c;
/// The array's allocation (`00c46170`, cdecl: the element count).
const ALLOCATE_ELEMENTS: u32 = 0x00c4_6170;

// Translated from 00c46530 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTArray<NiPointer<NiRefObject>, ..>` constructor (class by the vtable's RTTI):
/// `m_usMaxSize` (+8) = `max_size`, `m_usGrowBy` (+0xe) = `grow_by`, `m_usSize` (+0xa) and
/// `m_usESize` (+0xc) = 0; `m_pBase` (+4) is `00c46170(max_size)`, or null when `max_size`
/// (read back from +8) is 0. Returns `this`.
pub fn ni_t_array_ref_object_constructor(
    e: &mut Engine,
    this: Ptr,
    max_size: u16,
    grow_by: u16,
) -> Ptr {
    let base = this.addr();
    e.mem.set_u32(base, VTABLE_TARRAY_REF_OBJECT);
    e.mem.set_u16(base + 8, max_size);
    e.mem.set_u16(base + 0xe, grow_by);
    e.mem.set_u16(base + 0xa, 0);
    e.mem.set_u16(base + 0xc, 0);
    let count = u32::from(e.mem.u16(base + 8));
    if count == 0 {
        e.mem.set_u32(base + 4, 0);
    } else {
        let block = e.call(ALLOCATE_ELEMENTS, &args![count]).u32();
        e.mem.set_u32(base + 4, block);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(
        0x00c4_6530,
        ni_t_array_ref_object_constructor(Ptr, u16, u16) -> Ptr
    )]
}

#[cfg(test)]
mod b004_tests {
    use super::*;

    #[test]
    fn constructor_allocates_unless_the_size_is_zero() {
        let mut e = Engine::new();
        e.register(ALLOCATE_ELEMENTS, |_, a| (0x6000 + a[0]).into_ret());
        let this = e.mem.alloc(0x10);
        e.call_log = Some(vec![]);
        let r = e.call(0x00c4_6530, &args![this, 0x1111_0004u32, 0x2222_0002u32]);
        assert_eq!(r.u32(), this);
        let allocations: Vec<_> = e
            .call_log
            .take()
            .unwrap()
            .into_iter()
            .filter(|(a, _)| *a == ALLOCATE_ELEMENTS)
            .collect();
        assert_eq!(allocations, vec![(ALLOCATE_ELEMENTS, vec![4])]);
        assert_eq!(e.mem.u32(this), VTABLE_TARRAY_REF_OBJECT);
        assert_eq!(e.mem.u32(this + 4), 0x6004);
        assert_eq!(e.mem.u32(this + 8), 4);
        assert_eq!(e.mem.u32(this + 0xc), 0x0002_0000);
        e.register(ALLOCATE_ELEMENTS, |_, _| panic!("no allocation for 0"));
        e.call(0x00c4_6530, &args![this, 0u32, 0u32]);
        assert_eq!(e.mem.u32(this + 4), 0);
    }
}

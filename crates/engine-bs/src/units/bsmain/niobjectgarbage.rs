//! `BSMain/niobjectgarbage.obj` (Xbox PDB source unit), subsystem `BSMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The `NiTObjectArray<NiPointer<NiRefObject>>` virtual table (`010c1c5c`).
const OBJECT_ARRAY_VTABLE: u32 = 0x010c_1c5c;
/// The elements' vector deleting destructor (`00c46420`, `thiscall` on the element block,
/// `RET 4`: flags).
const ELEMENTS_VECTOR_DELETE: u32 = 0x00c4_6420;
/// The memory manager (`011f6238`) and its free (`00aa4060`, `thiscall`, `RET 4`).
const MEMORY_MANAGER: u32 = 0x011f_6238;
const MEMORY_MANAGER_FREE: u32 = 0x00aa_4060;

// Translated from 00c464c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTObjectArray<NiPointer<NiRefObject>>::_scalar_deleting_destructor_` (engine map;
/// `thiscall`, `RET 4`): the class's virtual table, the element block (`m_pBase`, +4) deleted
/// with its vector deleting destructor (`00c46420` with 3) when there is one, and, when bit 0
/// of `flags` is set, `this` given back to the memory manager. Answers `this`.
pub fn ni_t_object_array_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.mem.set_u32(this.addr(), OBJECT_ARRAY_VTABLE);
    let elements = e.mem.u32(this.addr().wrapping_add(4));
    if elements != 0 {
        e.call(ELEMENTS_VECTOR_DELETE, &args![elements, 3u32]);
    }
    if flags & 1 != 0 {
        e.call(MEMORY_MANAGER_FREE, &args![MEMORY_MANAGER, this]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(
        0x00c4_64c0,
        ni_t_object_array_scalar_deleting_destructor(Ptr, u32) -> Ptr
    )]
}

#[cfg(test)]
mod n069_tests {
    use super::*;

    #[test]
    fn the_elements_are_deleted_and_this_only_with_bit_zero() {
        let mut e = Engine::new();
        e.register(ELEMENTS_VECTOR_DELETE, |_, _| Ret::default());
        e.register(MEMORY_MANAGER_FREE, |_, _| Ret::default());
        let this = e.mem.alloc(0x10);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00c4_64c0, &args![this, 0u32]).u32(), this);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.mem.u32(this), OBJECT_ARRAY_VTABLE);
        assert!(!log.iter().any(|c| c.0 == ELEMENTS_VECTOR_DELETE));
        assert!(!log.iter().any(|c| c.0 == MEMORY_MANAGER_FREE));
        e.mem.set_u32(this + 4, 0x7000);
        e.call_log = Some(vec![]);
        e.call(0x00c4_64c0, &args![this, 1u32]);
        let log = e.call_log.take().unwrap();
        assert!(log.contains(&(ELEMENTS_VECTOR_DELETE, vec![0x7000, 3])));
        assert!(log.contains(&(MEMORY_MANAGER_FREE, vec![MEMORY_MANAGER, this])));
    }
}

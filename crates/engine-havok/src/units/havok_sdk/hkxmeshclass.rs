//! `Havok SDK/hkxmeshclass.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The memory router (`00c85750`): its field `+0x10` is the allocator whose slot `+0x10`
/// (`bufFree(block, size)`) gives blocks back.
const MEMORY_ROUTER: u32 = 0x00c8_5750;

/// `hkStringPtr::~hkStringPtr` (`00cb0c00`, `thiscall`).
const STRING_PTR_DESTRUCT: u32 = 0x00cb_0c00;

/// `hkxAttributeHolder::~hkxAttributeHolder` (`00e3eaa0`).
const HOLDER_DESTRUCT: u32 = 0x00e3_eaa0;

/// The allocator behind the memory router (`00c85750`, its field `+0x10`).
fn router_allocator(e: &mut Engine) -> u32 {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    e.mem.u32(router + 0x10)
}

// Translated from 00e01ef0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkxMesh::UserChannelInfo::_scalar_deleting_destructor_` (Xbox PDB), `thiscall`, `RET 4`
/// (`flags`): destroys the `hkStringPtr`s at `+0x18` and `+0x14` (`00cb0c00`), runs the
/// attribute holder's destructor (`00e3eaa0`) and, when bit 0 of `flags` is set, gives the object
/// back (allocator slot `+8`, with the 16-bit size at `+4`). Returns `this`.
pub fn hkx_mesh_user_channel_info_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u8,
) -> Ptr {
    let base = this.addr();
    e.call(STRING_PTR_DESTRUCT, &args![base + 0x18]);
    e.call(STRING_PTR_DESTRUCT, &args![base + 0x14]);
    e.call(HOLDER_DESTRUCT, &args![this]);
    if flags & 1 != 0 {
        let allocator = router_allocator(e);
        let size = u32::from(e.mem.u16(base + 4));
        e.vcall(allocator, 8, &args![this, size]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00e01ef0, hkx_mesh_user_channel_info_scalar_deleting_destructor(Ptr, u8) -> Ptr)]
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::cell::RefCell;
    use std::rc::Rc;

    type Log = Rc<RefCell<Vec<Vec<u32>>>>;

    const ALLOCATOR_OBJECT: u32 = 0x0060_0000;
    const ALLOCATOR_TABLE: u32 = 0x0060_1000;
    const DELETE_DOUBLE: u32 = 0x0060_2008;
    const FREE_DOUBLE: u32 = 0x0060_2010;

    /// A memory router (`00c85750`) whose allocator logs `[block, size]` for slot `+0x10`
    /// (`bufFree`, the first log) and for slot `+8` (the second log).
    fn install_memory(e: &mut Engine) -> (Log, Log) {
        e.map(ALLOCATOR_OBJECT, 0x100);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, ALLOCATOR_OBJECT);
        e.mem.set_u32(ALLOCATOR_OBJECT, ALLOCATOR_TABLE);
        let mut table = vec![0u32; 5];
        table[2] = DELETE_DOUBLE;
        table[4] = FREE_DOUBLE;
        e.put_vtable(ALLOCATOR_TABLE, &table);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        let frees: Log = Rc::new(RefCell::new(Vec::new()));
        let deletes: Log = Rc::new(RefCell::new(Vec::new()));
        let log = frees.clone();
        e.register_double(FREE_DOUBLE, move |_, a| {
            log.borrow_mut().push(vec![a[1], a[2]]);
            Ret::default()
        });
        let log = deletes.clone();
        e.register_double(DELETE_DOUBLE, move |_, a| {
            log.borrow_mut().push(vec![a[1], a[2]]);
            Ret::default()
        });
        (frees, deletes)
    }

    #[test]
    fn the_destructor_destroys_both_names_then_the_holder() {
        let mut e = Engine::new();
        let (_, deletes) = install_memory(&mut e);
        let events = Rc::new(RefCell::new(Vec::new()));
        for (address, tag) in [(STRING_PTR_DESTRUCT, 1u32), (HOLDER_DESTRUCT, 2)] {
            let kept = events.clone();
            e.register_double(address, move |_, a| {
                kept.borrow_mut().push((tag, a[0]));
                Ret::default()
            });
        }
        let this = e.mem.alloc(0x30);
        e.mem.set_u16(this + 4, 0x2c);
        assert_eq!(e.call(0x00e0_1ef0, &args![this, 0u8]).u32(), this);
        assert_eq!(
            *events.borrow(),
            vec![(1, this + 0x18), (1, this + 0x14), (2, this)]
        );
        assert!(deletes.borrow().is_empty());
        assert_eq!(e.call(0x00e0_1ef0, &args![this, 1u8]).u32(), this);
        assert_eq!(*deletes.borrow(), vec![vec![this, 0x2c]]);
    }
}

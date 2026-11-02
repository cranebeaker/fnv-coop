//! `NiXenonRenderer/run_00e94810` (Xbox PDB source unit), subsystem `NiXenonRenderer`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The vertex buffer set map's tables (by their RTTI): `NiTMapBase<..., unsigned int,
/// NiVBSet *>` (`010f0f94`) and `NiTPointerMap<unsigned int, NiVBSet *>` (`010f0fb4`); the
/// map clean-up `00b601a0` (unnamed; `thiscall`), the free `00aa10f0` (`cdecl`, a block), the
/// memory manager singleton (`011f6238`) and its free (`00aa4060`, `thiscall`, block).
const VB_SET_MAP_BASE_VTABLE: u32 = 0x010f_0f94;
const VB_SET_MAP_VTABLE: u32 = 0x010f_0fb4;
const MAP_00B601A0: u32 = 0x00b6_01a0;
const FREE_00AA10F0: u32 = 0x00aa_10f0;
const MEMORY_MANAGER: u32 = 0x011f_6238;
const MEMORY_DEALLOCATE: u32 = 0x00aa_4060;

// Translated from 00e94810 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the scalar deleting destructor of the vertex buffer set map's
/// base (`thiscall`, the flags): its table (`010f0f94`), `00b601a0`, the bucket array (+8)
/// freed (`00aa10f0`), and with bit 0 the block goes back to the memory manager (`00aa4060`).
/// Answers `this`.
pub fn fn_00e94810(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    let base = this.addr();
    e.mem.set_u32(base, VB_SET_MAP_BASE_VTABLE);
    e.call(MAP_00B601A0, &args![base]);
    let buckets = e.mem.u32(base.wrapping_add(8));
    e.call(FREE_00AA10F0, &args![buckets]);
    if flags & 1 != 0 {
        e.call(MEMORY_DEALLOCATE, &args![MEMORY_MANAGER, base]);
    }
    this
}

// Translated from 00e94850 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the scalar deleting destructor of the vertex buffer set map
/// (`thiscall`, the flags): its table (`010f0fb4`) and `00b601a0`, the base's table
/// (`010f0f94`) and `00b601a0` again, the bucket array (+8) freed (`00aa10f0`), and with bit 0
/// the block goes back to the memory manager (`00aa4060`). Answers `this`.
pub fn fn_00e94850(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    let base = this.addr();
    e.mem.set_u32(base, VB_SET_MAP_VTABLE);
    e.call(MAP_00B601A0, &args![base]);
    e.mem.set_u32(base, VB_SET_MAP_BASE_VTABLE);
    e.call(MAP_00B601A0, &args![base]);
    let buckets = e.mem.u32(base.wrapping_add(8));
    e.call(FREE_00AA10F0, &args![buckets]);
    if flags & 1 != 0 {
        e.call(MEMORY_DEALLOCATE, &args![MEMORY_MANAGER, base]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00e9_4810, fn_00e94810(Ptr, u32) -> Ptr),
        entry!(0x00e9_4850, fn_00e94850(Ptr, u32) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    #[test]
    fn vb_set_map_deleting_destructors_free_on_bit_zero() {
        for (address, tables) in [
            (0x00e9_4810u32, vec![VB_SET_MAP_BASE_VTABLE]),
            (0x00e9_4850, vec![VB_SET_MAP_VTABLE, VB_SET_MAP_BASE_VTABLE]),
        ] {
            for (flags, frees) in [(0u32, false), (1, true)] {
                let mut e = Engine::new();
                let seen = Rc::new(RefCell::new(vec![]));
                let tables_seen = seen.clone();
                e.register_double(MAP_00B601A0, move |e, a| {
                    tables_seen.borrow_mut().push(e.mem.u32(a[0]));
                    Ret::default()
                });
                e.register(FREE_00AA10F0, |_, _| Ret::default());
                e.register(MEMORY_DEALLOCATE, |_, _| Ret::default());
                let this = e.mem.alloc(0x10);
                e.mem.set_u32(this + 8, 0x0e0b_0000);
                e.call_log = Some(vec![]);
                assert_eq!(e.call(address, &args![this, flags]).u32(), this);
                assert_eq!(*seen.borrow(), tables);
                let log = e.call_log.take().unwrap();
                let freed: Vec<_> = log.iter().filter(|c| c.0 == FREE_00AA10F0).collect();
                assert_eq!(freed[0].1, vec![0x0e0b_0000]);
                let released = log.iter().filter(|c| c.0 == MEMORY_DEALLOCATE).count();
                assert_eq!(released, usize::from(frees));
            }
        }
    }
}

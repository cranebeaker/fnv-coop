//! `fallout shared/distant terrain system/run_006fa160` (Xbox PDB source unit), subsystem `fallout shared/distant terrain system`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The vtable at `0106dfe0`. The complete object locator before it names
/// the type descriptor `.?AV?$BSSimpleArray@UTreeType@BGSDistantTreeBlock@@
/// $0EAA@@@`, that is `BSSimpleArray<BGSDistantTreeBlock::TreeType,1024>`.
const TREE_TYPE_ARRAY_VTABLE: u32 = 0x0106_dfe0;
/// `bgsdistanttreeblock.cpp`: with its second argument nonzero, releases the
/// array's buffer (`pBuffer` at `+4`, `iReservedSize` at `+0xc`) and clears
/// the size at `+8`.
const ARRAY_RELEASE: u32 = 0x006f_9c70;

// Translated from 006fa160 (decompiled, FalloutNV.exe 1.4.0.525)
/// The destructor body of `BSSimpleArray<BGSDistantTreeBlock::TreeType,
/// 1024>` (the vtable it stores is that class's, by its run-time type
/// information): stores the vtable and releases the buffer through
/// `006f9c70(this, 1)`.
pub fn fn_006fa160(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), TREE_TYPE_ARRAY_VTABLE);
    e.call(ARRAY_RELEASE, &args![this, 1u32]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x006fa160, fn_006fa160(Ptr))]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn destructor_body_stores_the_vtable_and_releases_the_buffer() {
        let mut e = Engine::new();
        e.register(ARRAY_RELEASE, |_, _| Ret::default());
        let array = Ptr::<()>::new(e.mem.alloc(0x10));
        e.mem.set_u32(array.addr(), 0x1111);
        e.call_log = Some(vec![]);
        e.call(0x006f_a160, &args![array]);
        assert_eq!(e.mem.u32(array.addr()), TREE_TYPE_ARRAY_VTABLE);
        let log = e.call_log.take().unwrap();
        assert_eq!(log.len(), 2);
        assert_eq!(log[1], (ARRAY_RELEASE, vec![array.addr(), 1]));
    }
}

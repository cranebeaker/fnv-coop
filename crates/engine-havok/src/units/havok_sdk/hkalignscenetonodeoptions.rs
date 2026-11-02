//! `Havok SDK/hkalignscenetonodeoptions.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `hkStringPtr::~hkStringPtr` (`00cb0c00`, thiscall, no stack words).
const STRING_DESTRUCT: u32 = 0x00cb_0c00;
/// The vtable the destructors of this family leave in the object (`0102e388`).
const BASE_VTABLE: u32 = 0x0102_e388;
/// The memory router (`00c85750`): its field `+0x10` is the allocator a deleted block goes
/// back to.
const MEMORY_ROUTER: u32 = 0x00c8_5750;

// Translated from 00e4afa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkAlignSceneToNodeOptions::_scalar_deleting_destructor_` (Xbox PDB; thiscall, one stack word
/// `flags`): destroys the string pointer at +0x14, leaves the base vtable and, when bit 0 of
/// `flags` is set, returns the block to the memory router's allocator (function at +8) with
/// the 16-bit size kept at +4. Returns `this`.
pub fn hk_align_scene_to_node_options_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u8,
) -> Ptr {
    let me = this.addr();
    e.call(STRING_DESTRUCT, &args![me + 0x14]);
    e.mem.set_u32(me, BASE_VTABLE);
    if flags & 1 != 0 {
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let size = u32::from(e.mem.u16(me + 4));
        e.vcall(allocator, 8, &args![me, size]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00e4_afa0, hk_align_scene_to_node_options_scalar_deleting_destructor(Ptr, u8) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    type Log = Rc<RefCell<Vec<(u32, Vec<u32>)>>>;

    const FREE_BLOCK: u32 = 0x00cc_0008;

    /// An engine with the memory router, an allocator whose function at +8 is a logging double,
    /// and a logging double for the string destructor. Returns the log and the allocator.
    fn setup() -> (Engine, Log, u32) {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        let log: Log = Rc::default();
        let vtable = e.mem.alloc(0x20);
        let allocator = e.mem.alloc(0x10);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(allocator, vtable);
        e.mem.set_u32(vtable + 8, FREE_BLOCK);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        for address in [FREE_BLOCK, STRING_DESTRUCT] {
            let log = Rc::clone(&log);
            e.register_double(address, move |_, a| {
                log.borrow_mut().push((address, a.to_vec()));
                Ret::default()
            });
        }
        (e, log, allocator)
    }

    #[test]
    fn destroys_the_string_and_keeps_the_block_when_bit_zero_is_clear() {
        let (mut e, log, _) = setup();
        let this = e.mem.alloc(0x20);
        let result =
            hk_align_scene_to_node_options_scalar_deleting_destructor(&mut e, Ptr::new(this), 0xfe);
        assert_eq!(result, Ptr::new(this));
        assert_eq!(*log.borrow(), vec![(STRING_DESTRUCT, vec![this + 0x14])]);
        assert_eq!(e.mem.u32(this), BASE_VTABLE);
    }

    #[test]
    fn frees_the_block_with_its_stored_size_when_bit_zero_is_set() {
        let (mut e, log, allocator) = setup();
        let this = e.mem.alloc(0x20);
        e.mem.set_u16(this + 4, 0x1c);
        let result =
            hk_align_scene_to_node_options_scalar_deleting_destructor(&mut e, Ptr::new(this), 1);
        assert_eq!(result, Ptr::new(this));
        assert_eq!(
            *log.borrow(),
            vec![
                (STRING_DESTRUCT, vec![this + 0x14]),
                (FREE_BLOCK, vec![allocator, this, 0x1c]),
            ]
        );
        assert_eq!(e.mem.u32(this), BASE_VTABLE);
    }
}

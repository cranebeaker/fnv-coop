//! `Havok SDK/hklocalframeclass.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The memory router (`0x00c8_5750`): its field `+0x10` is the allocator the block is freed through.
const MEMORY_ROUTER: u32 = 0x00c8_5750;

// Translated from 00da0860 (decompiled, FalloutNV.exe 1.4.0.525)
/// Deleting destructor: runs the base destructor (`0x00df_67d0`) on `this` and, when bit 0 of
/// `flags` is set, returns the block to the memory router's allocator (slot `8`) with the
/// 16-bit size kept at `this + 4`. Returns `this`. The class is unconfirmed.
pub fn deleting_destructor_00da0860(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    e.call(0x00df_67d0, &args![this]);
    if flags & 1 != 0 {
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let size = u32::from(e.mem.u16(this.addr() + 4));
        e.vcall(allocator, 8, &args![this, size]);
    }
    this
}

/// `hkStringPtr::~hkStringPtr` (`00cb0c00`, `thiscall`, no word).
const STRING_PTR_DESTRUCTOR: u32 = 0x00cb_0c00;
/// The virtual table stored by the group's deleting destructor.
const LOCAL_FRAME_GROUP_VTABLE: u32 = 0x0102_e388;

// Translated from 00da0800 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkLocalFrameGroup::_scalar_deleting_destructor_` (Xbox PDB, from the engine map's
/// vtable entry), `thiscall`, `RET 4`: destroys the string pointer at `+8`
/// (`00cb0c00`), stores the virtual table (`0102e388`) and, when bit 0 of `flags` is set,
/// returns the block to the memory router's allocator (slot 8) with the 16-bit size kept
/// at `this + 4`. Returns `this`.
pub fn hk_localframe_group_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    e.call(STRING_PTR_DESTRUCTOR, &args![this.addr() + 8]);
    e.mem.set_u32(this.addr(), LOCAL_FRAME_GROUP_VTABLE);
    if flags & 1 != 0 {
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let size = u32::from(e.mem.u16(this.addr() + 4));
        let allocator = e.mem.u32(router + 0x10);
        e.vcall(allocator, 8, &args![this, size]);
    }
    this
}

// Translated from 00da0840 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkSimpleLocalFrame::getParentFrame` (Xbox PDB), `thiscall`: the word at `+0x5c`.
pub fn hk_simple_localframe_get_parent_frame(e: &mut Engine, this: Ptr) -> u32 {
    e.mem.u32(this.addr() + 0x5c)
}

// Translated from 00da0850 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkSimpleLocalFrame::setParentFrame` (Xbox PDB), `thiscall`, `RET 4`: stores `frame` at
/// `+0x5c`.
pub fn hk_simple_localframe_set_parent_frame(e: &mut Engine, this: Ptr, frame: u32) {
    e.mem.set_u32(this.addr() + 0x5c, frame);
}

// Translated from 00da0890 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkSimpleLocalFrame::getName` (Xbox PDB), `thiscall`: the string pointer at `+0x64` with
/// its low flag bit cleared (`hkStringPtr::cString`).
pub fn hk_simple_localframe_get_name(e: &mut Engine, this: Ptr) -> u32 {
    e.mem.u32(this.addr() + 0x64) & 0xffff_fffe
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00da_0860, deleting_destructor_00da0860(Ptr, u8) -> Ptr),
        entry!(0x00da0800, hk_localframe_group_scalar_deleting_destructor(Ptr, u8) -> Ptr),
        entry!(0x00da0840, hk_simple_localframe_get_parent_frame(Ptr) -> u32),
        entry!(0x00da0850, hk_simple_localframe_set_parent_frame(Ptr, u32)),
        entry!(0x00da0890, hk_simple_localframe_get_name(Ptr) -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    const FREE_SLOT_DOUBLE: u32 = 0x0900_0011;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.map(0x00fd_f000, 0x1000);
        e
    }

    /// Installs a base-destructor double at `base` and a memory router whose allocator
    /// frees through a double; both push their arguments (tagged 1 and 2) on the log.
    fn install(e: &mut Engine, base: u32) -> Rc<RefCell<Vec<u32>>> {
        let log = Rc::new(RefCell::new(Vec::new()));
        let base_log = Rc::clone(&log);
        e.register_double(base, move |_, a| {
            base_log.borrow_mut().extend([1, a[0]]);
            Ret::default()
        });
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 8, FREE_SLOT_DOUBLE);
        let allocator = e.mem.alloc(0x10);
        e.mem.set_u32(allocator, vtable);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        let free_log = Rc::clone(&log);
        e.register_double(FREE_SLOT_DOUBLE, move |_, a| {
            free_log.borrow_mut().extend([2, a[1], a[2]]);
            Ret::default()
        });
        log
    }

    /// Without bit 0 only the base destructor runs; with it the block is also freed with
    /// its stored size. Either way `this` is returned.
    fn check(address: u32, base: u32) {
        let mut e = engine();
        let log = install(&mut e, base);
        let this = e.mem.alloc(0x40);
        e.mem.set_u16(this + 4, 0x1234);
        let got = e.call(address, &args![this, 0xffff_fffeu32]).u32();
        assert_eq!(got, this);
        assert_eq!(*log.borrow(), vec![1, this]);
        log.borrow_mut().clear();
        let got = e.call(address, &args![this, 1u32]).u32();
        assert_eq!(got, this);
        assert_eq!(*log.borrow(), vec![1, this, 2, this, 0x1234]);
    }

    #[test]
    fn deleting_destructor_00da0860_frees_only_when_bit_zero_is_set() {
        check(0x00da_0860, 0x00df_67d0);
    }

    #[test]
    fn the_group_destructor_destroys_its_string_and_frees_on_request() {
        let mut e = engine();
        e.register(0x00cb_0c00, |_, _| Ret::default());
        let object = e.mem.alloc(0x20);
        e.mem.set_u16(object + 4, 0x20);
        e.put_vtable(0x4400_3000, &[0, 0, 0x4400_3100]);
        e.register(0x4400_3100, |_, _| Ret::default());
        let allocator = e.mem.alloc(8);
        e.mem.set_u32(allocator, 0x4400_3000);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00da_0800, &args![object, 0u8]).u32(), object);
        assert_eq!(e.mem.u32(object), 0x0102_e388);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 2);
        assert_eq!(
            e.call_log.as_ref().unwrap()[1],
            (0x00cb_0c00, vec![object + 8])
        );
        e.call_log = Some(vec![]);
        e.call(0x00da_0800, &args![object, 1u8]);
        let freed: Vec<Vec<u32>> = e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == 0x4400_3100)
            .map(|c| c.1.clone())
            .collect();
        assert_eq!(freed, vec![vec![allocator, object, 0x20]]);
    }

    #[test]
    fn the_simple_localframe_accessors_read_and_write_their_words() {
        let mut e = Engine::new();
        let frame = e.mem.alloc(0x80);
        e.call(0x00da_0850, &args![frame, 0x1234u32]);
        assert_eq!(e.mem.u32(frame + 0x5c), 0x1234);
        assert_eq!(e.call(0x00da_0840, &args![frame]).u32(), 0x1234);
        e.mem.set_u32(frame + 0x64, 0x7001);
        assert_eq!(e.call(0x00da_0890, &args![frame]).u32(), 0x7000);
    }
}

//! `(unplaced)/run_006ec410` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The destructors of two message-queue classes the engine map has no name
//! for. The vtables' RTTI names them: `0106d6f0` is
//! `BSTCommonMessageQueue<NiPointer<NiAVObject>>` and `0106d710` its base
//! `BSTMessageQueue<NiPointer<NiAVObject>>` (Xbox PDB: 8 and 4 bytes).

#[allow(unused_imports)]
use crate::prelude::*;

/// `BSTCommonMessageQueue<NiPointer<NiAVObject>>`'s vtable (RTTI).
const COMMON_MESSAGE_QUEUE_VTABLE: u32 = 0x0106_d6f0;
/// `BSTMessageQueue<NiPointer<NiAVObject>>`'s vtable (RTTI).
const MESSAGE_QUEUE_VTABLE: u32 = 0x0106_d710;
/// `00401030`: frees a block (cdecl, one word).
const FREE: u32 = 0x0040_1030;

// Translated from 006ec410 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor body of `BSTCommonMessageQueue<NiPointer<NiAVObject>>`: puts
/// its vtable back, then runs the base class's destructor (`fn_006ec430`).
pub fn fn_006ec410(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), COMMON_MESSAGE_QUEUE_VTABLE);
    fn_006ec430(e, this);
}

// Translated from 006ec430 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor body of `BSTMessageQueue<NiPointer<NiAVObject>>`: puts its
/// vtable back.
pub fn fn_006ec430(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), MESSAGE_QUEUE_VTABLE);
}

// Translated from 006ec450 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor of `BSTCommonMessageQueue<NiPointer<NiAVObject>>`:
/// destroys the queue and frees it when bit 0 of `flags` is set; returns
/// `this`.
pub fn fn_006ec450(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_006ec410(e, this);
    if flags & 1 != 0 {
        e.call(FREE, &args![this]);
    }
    this
}

// Translated from 006ec480 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor of `BSTMessageQueue<NiPointer<NiAVObject>>`:
/// destroys the queue and frees it when bit 0 of `flags` is set; returns
/// `this`.
pub fn fn_006ec480(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_006ec430(e, this);
    if flags & 1 != 0 {
        e.call(FREE, &args![this]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x006ec410, fn_006ec410(Ptr)),
        entry!(0x006ec430, fn_006ec430(Ptr)),
        entry!(0x006ec450, fn_006ec450(Ptr, u32) -> Ptr),
        entry!(0x006ec480, fn_006ec480(Ptr, u32) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.register(FREE, |_, _| Ret::default());
        e
    }

    #[test]
    fn common_queue_destructor_ends_with_the_base_vtable() {
        let mut e = engine();
        let queue: Ptr = Ptr::new(e.mem.alloc(8));
        e.call(0x006e_c410, &args![queue]);
        assert_eq!(e.mem.u32(queue.addr()), MESSAGE_QUEUE_VTABLE);
    }

    #[test]
    fn base_queue_destructor_puts_its_vtable() {
        let mut e = engine();
        let queue: Ptr = Ptr::new(e.mem.alloc(4));
        e.call(0x006e_c430, &args![queue]);
        assert_eq!(e.mem.u32(queue.addr()), MESSAGE_QUEUE_VTABLE);
    }

    fn frees(e: &mut Engine, addr: u32, flags: u32) -> Vec<Vec<u32>> {
        let queue: Ptr = Ptr::new(e.mem.alloc(8));
        e.call_log = Some(vec![]);
        let back = e.call(addr, &args![queue, flags]).ptr::<()>();
        assert_eq!(back, queue);
        assert_eq!(e.mem.u32(queue.addr()), MESSAGE_QUEUE_VTABLE);
        let log = e.call_log.take().unwrap();
        log.into_iter()
            .filter(|(a, _)| *a == FREE)
            .map(|(_, w)| {
                assert_eq!(w, vec![queue.addr()]);
                w
            })
            .collect()
    }

    #[test]
    fn common_queue_deleting_destructor_frees_only_with_bit_0() {
        let mut e = engine();
        assert_eq!(frees(&mut e, 0x006e_c450, 1).len(), 1);
        assert_eq!(frees(&mut e, 0x006e_c450, 2).len(), 0);
    }

    #[test]
    fn base_queue_deleting_destructor_frees_only_with_bit_0() {
        let mut e = engine();
        assert_eq!(frees(&mut e, 0x006e_c480, 3).len(), 1);
        assert_eq!(frees(&mut e, 0x006e_c480, 0).len(), 0);
    }
}

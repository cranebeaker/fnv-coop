//! `Havok SDK/hkwin32semaphore.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The KERNEL32 import slots the exe calls: `CreateSemaphoreA` (`00fdf0b4`), `CloseHandle`
/// (`00fdf07c`), `WaitForSingleObject` (`00fdf1c8`) and `ReleaseSemaphore` (`00fdf1b4`).
const CREATE_SEMAPHORE_A: u32 = 0x00fd_f0b4;
const CLOSE_HANDLE: u32 = 0x00fd_f07c;
const WAIT_FOR_SINGLE_OBJECT: u32 = 0x00fd_f1c8;
const RELEASE_SEMAPHORE: u32 = 0x00fd_f1b4;

// Translated from 00d04350 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkSemaphore::hkSemaphore` (Xbox PDB), `thiscall`, `RET 8` (`initial`, `maximum`):
/// `CreateSemaphoreA(NULL, initial, maximum, NULL)`, the handle stored at `this`. Returns
/// `this`.
pub fn hk_semaphore_hk_semaphore(e: &mut Engine, this: Ptr, initial: u32, maximum: u32) -> Ptr {
    let handle = e
        .call(CREATE_SEMAPHORE_A, &args![0u32, initial, maximum, 0u32])
        .u32();
    e.mem.set_u32(this.addr(), handle);
    this
}

// Translated from 00d04370 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkSemaphore::~hkSemaphore` (Xbox PDB), `thiscall`, no stack words: `CloseHandle` of the
/// handle at `this`.
pub fn hk_semaphore_destructor(e: &mut Engine, this: Ptr) {
    let handle = e.mem.u32(this.addr());
    e.call(CLOSE_HANDLE, &args![handle]);
}

// Translated from 00d04380 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkSemaphore::acquire` (Xbox PDB), `thiscall`, no stack words: `WaitForSingleObject` of the
/// handle at `this` with the timeout `0xffffffff` (infinite).
pub fn hk_semaphore_acquire(e: &mut Engine, this: Ptr) {
    let handle = e.mem.u32(this.addr());
    e.call(WAIT_FOR_SINGLE_OBJECT, &args![handle, 0xffff_ffffu32]);
}

// Translated from 00d04390 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkSemaphore::release` (Xbox PDB), `thiscall`, `RET 4` (`count`): `ReleaseSemaphore` of the
/// handle at `this` by `count`, the previous count not asked for (a null pointer).
pub fn hk_semaphore_release(e: &mut Engine, this: Ptr, count: u32) {
    let handle = e.mem.u32(this.addr());
    e.call(RELEASE_SEMAPHORE, &args![handle, count, 0u32]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00d04350, hk_semaphore_hk_semaphore(Ptr, u32, u32) -> Ptr),
        entry!(0x00d04370, hk_semaphore_destructor(Ptr)),
        entry!(0x00d04380, hk_semaphore_acquire(Ptr)),
        entry!(0x00d04390, hk_semaphore_release(Ptr, u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::cell::RefCell;
    use std::rc::Rc;

    /// Doubles for the four imports that log `(slot, arguments)`; `CreateSemaphoreA` answers
    /// the handle 0x1234.
    fn imports(e: &mut Engine) -> Rc<RefCell<Vec<(u32, Vec<u32>)>>> {
        let log = Rc::new(RefCell::new(Vec::new()));
        for slot in [
            CREATE_SEMAPHORE_A,
            CLOSE_HANDLE,
            WAIT_FOR_SINGLE_OBJECT,
            RELEASE_SEMAPHORE,
        ] {
            let log = log.clone();
            e.register_double(slot, move |_, a| {
                log.borrow_mut().push((slot, a.to_vec()));
                Ret {
                    eax: 0x1234,
                    ..Ret::default()
                }
            });
        }
        log
    }

    #[test]
    fn the_constructor_creates_a_semaphore_and_keeps_the_handle() {
        let mut e = Engine::new();
        let log = imports(&mut e);
        let this = e.mem.alloc(8);
        assert_eq!(e.call(0x00d0_4350, &args![this, 2u32, 5u32]).u32(), this);
        assert_eq!(e.mem.u32(this), 0x1234);
        // CreateSemaphoreA(security, initial, maximum, name).
        assert_eq!(*log.borrow(), vec![(CREATE_SEMAPHORE_A, vec![0, 2, 5, 0])]);
    }

    #[test]
    fn the_other_three_use_the_handle_at_this() {
        let mut e = Engine::new();
        let log = imports(&mut e);
        let this = e.mem.alloc(8);
        e.mem.set_u32(this, 0x77);
        e.call(0x00d0_4380, &args![this]);
        e.call(0x00d0_4390, &args![this, 3u32]);
        e.call(0x00d0_4370, &args![this]);
        assert_eq!(
            *log.borrow(),
            vec![
                (WAIT_FOR_SINGLE_OBJECT, vec![0x77, 0xffff_ffff]),
                (RELEASE_SEMAPHORE, vec![0x77, 3, 0]),
                (CLOSE_HANDLE, vec![0x77]),
            ]
        );
    }
}

//! `Havok SDK/hkpbroadphase.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The memory router (`0x00c8_5750`): its field `+0x10` is the allocator the block is freed through.
const MEMORY_ROUTER: u32 = 0x00c8_5750;
/// `EnterCriticalSection` and `LeaveCriticalSection` import slots.
const ENTER_CRITICAL_SECTION: u32 = 0x00fd_f05c;
const LEAVE_CRITICAL_SECTION: u32 = 0x00fd_f100;

// Translated from 00d97050 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpBroadPhase::lockImplementation` (Xbox PDB): enters the critical section whose pointer
/// is at +0x10 of the broad phase (`EnterCriticalSection` import slot `00fdf05c`).
pub fn hkp_broad_phase_lock_implementation(e: &mut Engine, this: Ptr) {
    let section = e.mem.u32(this.addr() + 0x10);
    e.call(ENTER_CRITICAL_SECTION, &args![section]);
}

// Translated from 00d97060 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpBroadPhase::unlockImplementation` (Xbox PDB): leaves the critical section whose pointer
/// is at +0x10 of the broad phase (`LeaveCriticalSection` import slot `00fdf100`).
pub fn hkp_broad_phase_unlock_implementation(e: &mut Engine, this: Ptr) {
    let section = e.mem.u32(this.addr() + 0x10);
    e.call(LEAVE_CRITICAL_SECTION, &args![section]);
}

// Translated from 00d97100 (decompiled, FalloutNV.exe 1.4.0.525)
/// Deleting destructor: runs the base destructor (`0x00d9_70c0`) on `this` and, when bit 0 of
/// `flags` is set, returns the block to the memory router's allocator (slot `8`) with the
/// 16-bit size kept at `this + 4`. Returns `this`. The class is unconfirmed.
pub fn deleting_destructor_00d97100(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    e.call(0x00d9_70c0, &args![this]);
    if flags & 1 != 0 {
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let size = u32::from(e.mem.u16(this.addr() + 4));
        e.vcall(allocator, 8, &args![this, size]);
    }
    this
}

/// `hkCriticalSection::hkCriticalSection` (`00cf6da0`, `this` = the block, `(argument)`) and the
/// call `00cf6d30` made on the broad phase's `+8` sub-object once the section is in place.
const CRITICAL_SECTION_CONSTRUCT: u32 = 0x00cf_6da0;
const AFTER_SECTION_CALL: u32 = 0x00cf_6d30;
/// `DeleteCriticalSection` import slot.
const DELETE_CRITICAL_SECTION: u32 = 0x00fd_f058;

// Translated from 00d97070 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpBroadPhase::enableMultiThreading` (named in the engine map): when the broad phase has
/// no critical section yet (`+0x10`), takes `0x18` bytes from the memory router's allocator
/// (slot 4), constructs the section in them with `argument` (`00cf6da0`; 0 when the
/// allocation failed) and stores it, then calls `00cf6d30` on the sub-object at `this + 8`.
pub fn hkp_broad_phase_enable_multi_threading(e: &mut Engine, this: Ptr, argument: u32) {
    let base = this.addr();
    if e.mem.u32(base + 0x10) != 0 {
        return;
    }
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let block = e.vcall(allocator, 4, &args![0x18u32]).u32();
    let section = if block != 0 {
        e.call(CRITICAL_SECTION_CONSTRUCT, &args![block, argument])
            .u32()
    } else {
        0
    };
    e.mem.set_u32(base + 0x10, section);
    e.call(AFTER_SECTION_CALL, &args![base + 8]);
}

// Translated from 00d970c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpBroadPhase::~hkpBroadPhase` (named in the engine map): writes the vtable `010d3f7c`; a
/// critical section at `+0x10` is deleted (`DeleteCriticalSection`, import slot `00fdf058`),
/// its `0x18` bytes are returned to the memory router's allocator (slot 8) and the pointer
/// cleared; finally the base vtable `0102e388` is written.
pub fn hkp_broad_phase_destruct(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    let section = e.mem.u32(base + 0x10);
    e.mem.set_u32(base, 0x010d_3f7c);
    if section != 0 {
        e.call(DELETE_CRITICAL_SECTION, &args![section]);
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        e.vcall(allocator, 8, &args![section, 0x18u32]);
        e.mem.set_u32(base + 0x10, 0);
    }
    e.mem.set_u32(base, 0x0102_e388);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00d9_7070,
            hkp_broad_phase_enable_multi_threading(Ptr, u32)
        ),
        entry!(0x00d9_70c0, hkp_broad_phase_destruct(Ptr)),
        entry!(0x00d9_7050, hkp_broad_phase_lock_implementation(Ptr)),
        entry!(0x00d9_7060, hkp_broad_phase_unlock_implementation(Ptr)),
        entry!(0x00d9_7100, deleting_destructor_00d97100(Ptr, u8) -> Ptr),
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
    fn deleting_destructor_00d97100_frees_only_when_bit_zero_is_set() {
        check(0x00d9_7100, 0x00d9_70c0);
    }

    #[test]
    fn lock_and_unlock_enter_and_leave_the_critical_section_at_0x10() {
        let mut e = engine();
        e.call_log = Some(vec![]);
        for slot in [ENTER_CRITICAL_SECTION, LEAVE_CRITICAL_SECTION] {
            e.register(slot, |_, _| Ret::default());
        }
        let this = e.mem.alloc(0x20);
        e.mem.set_u32(this + 0x10, 0x1234_0000);
        e.call(0x00d9_7050, &args![this]);
        e.call(0x00d9_7060, &args![this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log,
            vec![
                (0x00d9_7050, vec![this]),
                (ENTER_CRITICAL_SECTION, vec![0x1234_0000]),
                (0x00d9_7060, vec![this]),
                (LEAVE_CRITICAL_SECTION, vec![0x1234_0000]),
            ]
        );
    }

    const TAKE_SLOT_DOUBLE: u32 = 0x0900_0012;

    /// The argument lists of every logged call to `addr`.
    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, words)| words.clone())
            .collect()
    }

    /// A memory router whose allocator takes through a double answering `take_result` (slot
    /// 4) and frees through another (slot 8); returns the allocator.
    fn install_allocator(e: &mut Engine, take_result: u32) -> u32 {
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 4, TAKE_SLOT_DOUBLE);
        e.mem.set_u32(vtable + 8, FREE_SLOT_DOUBLE);
        let allocator = e.mem.alloc(0x10);
        e.mem.set_u32(allocator, vtable);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        e.register_double(TAKE_SLOT_DOUBLE, move |_, _| Ret {
            eax: take_result,
            ..Ret::default()
        });
        e.register_double(FREE_SLOT_DOUBLE, |_, _| Ret::default());
        allocator
    }

    #[test]
    fn enable_multi_threading_builds_the_critical_section_once() {
        let mut e = engine();
        e.call_log = Some(vec![]);
        let allocator = install_allocator(&mut e, 0x4444_0000);
        e.register_double(CRITICAL_SECTION_CONSTRUCT, |_, a| Ret {
            eax: a[0] + 1,
            ..Ret::default()
        });
        e.register_double(AFTER_SECTION_CALL, |_, _| Ret::default());
        let this = e.mem.alloc(0x20);
        e.call(0x00d9_7070, &args![this, 77u32]);
        assert_eq!(e.mem.u32(this + 0x10), 0x4444_0001);
        assert_eq!(calls(&e, TAKE_SLOT_DOUBLE), vec![vec![allocator, 0x18]]);
        assert_eq!(
            calls(&e, CRITICAL_SECTION_CONSTRUCT),
            vec![vec![0x4444_0000, 77]]
        );
        assert_eq!(calls(&e, AFTER_SECTION_CALL), vec![vec![this + 8]]);
        // With a section in place nothing more happens.
        e.call(0x00d9_7070, &args![this, 77u32]);
        assert_eq!(calls(&e, AFTER_SECTION_CALL).len(), 1);
        assert_eq!(calls(&e, TAKE_SLOT_DOUBLE).len(), 1);
    }

    #[test]
    fn enable_multi_threading_stores_null_when_the_allocation_fails() {
        let mut e = engine();
        e.call_log = Some(vec![]);
        install_allocator(&mut e, 0);
        e.register_double(AFTER_SECTION_CALL, |_, _| Ret::default());
        let this = e.mem.alloc(0x20);
        e.call(0x00d9_7070, &args![this, 77u32]);
        assert_eq!(e.mem.u32(this + 0x10), 0);
        assert!(calls(&e, CRITICAL_SECTION_CONSTRUCT).is_empty());
        assert_eq!(calls(&e, AFTER_SECTION_CALL), vec![vec![this + 8]]);
    }

    #[test]
    fn destruct_deletes_and_returns_the_critical_section() {
        let mut e = engine();
        e.call_log = Some(vec![]);
        let allocator = install_allocator(&mut e, 0);
        e.register_double(DELETE_CRITICAL_SECTION, |_, _| Ret::default());
        let this = e.mem.alloc(0x20);
        e.mem.set_u32(this + 0x10, 0x5555_0000);
        e.call(0x00d9_70c0, &args![this]);
        assert_eq!(e.mem.u32(this), 0x0102_e388);
        assert_eq!(e.mem.u32(this + 0x10), 0);
        assert_eq!(calls(&e, DELETE_CRITICAL_SECTION), vec![vec![0x5555_0000]]);
        assert_eq!(
            calls(&e, FREE_SLOT_DOUBLE),
            vec![vec![allocator, 0x5555_0000, 0x18]]
        );
        // Without a section only the vtables are written.
        e.call(0x00d9_70c0, &args![this]);
        assert_eq!(calls(&e, DELETE_CRITICAL_SECTION).len(), 1);
    }
}

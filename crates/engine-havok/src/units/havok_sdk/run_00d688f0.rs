//! `Havok SDK/run_00d688f0` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The memory router (`0x00c8_5750`): its field `+0x10` is the allocator the block is freed through.
const MEMORY_ROUTER: u32 = 0x00c8_5750;

// Translated from 00d68910 (decompiled, FalloutNV.exe 1.4.0.525)
/// Deleting destructor: runs the base destructor (`0x00de_2a40`) on `this` and, when bit 0 of
/// `flags` is set, returns the block to the memory router's allocator (slot `8`) with the
/// 16-bit size kept at `this + 4`. Returns `this`. The class is unconfirmed.
pub fn deleting_destructor_00d68910(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    e.call(0x00de_2a40, &args![this]);
    if flags & 1 != 0 {
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let size = u32::from(e.mem.u16(this.addr() + 4));
        e.vcall(allocator, 8, &args![this, size]);
    }
    this
}

// Translated from 00d68900 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by `0x20` (`SUB ECX,0x20`) and jumps to `00d68910`, which (`RET 4`) has the shape of a scalar deleting destructor: it reads bit 0 of the one stack word. The caller gets its `EAX`; the class is unconfirmed.
pub fn fn_00d68900(e: &mut Engine, this: Ptr, flags: u32) -> u32 {
    e.call(0x00d6_8910, &args![this.addr().wrapping_sub(0x20), flags])
        .u32()
}

/// The process id the active contact point viewer answers (`0126900c`) and the word
/// `00d68940` hands to the display handler (`01269010`).
const ACTIVE_PROCESS_ID: u32 = 0x0126_900c;
const CONTACT_POINT_TAG: u32 = 0x0126_9010;

// Translated from 00d688f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Answers the process id kept at `0126900c`: slot 1 of the vtable `010d201c` (RTTI:
/// `hkpActiveContactPointViewer`); `ECX` unused.
pub fn fn_00d688f0(e: &mut Engine) -> u32 {
    e.global(ACTIVE_PROCESS_ID)
}

// Translated from 00d68940 (decompiled, FalloutNV.exe 1.4.0.525)
/// Calls the method at vtable offset 0x10 of the object at `[this - 0x14]` with `(contact +
/// 0xf0, contact + 0x13, the word at 01269010)`, where `contact` is the one stack word.
pub fn fn_00d68940(e: &mut Engine, this: Ptr, contact: u32) {
    let tag: u32 = e.global(CONTACT_POINT_TAG);
    let handler = e.mem.u32(this.addr().wrapping_sub(0x14));
    e.vcall(
        handler,
        0x10,
        &args![contact.wrapping_add(0xf0), contact.wrapping_add(0x13), tag],
    );
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00d6_88f0, fn_00d688f0() -> u32),
        entry!(0x00d6_8940, fn_00d68940(Ptr, u32)),
        entry!(0x00d6_8910, deleting_destructor_00d68910(Ptr, u8) -> Ptr),
        entry!(0x00d6_8900, fn_00d68900(Ptr, u32) -> u32),
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
    fn deleting_destructor_00d68910_frees_only_when_bit_zero_is_set() {
        check(0x00d6_8910, 0x00de_2a40);
    }
}

#[cfg(test)]
mod a07_thunk_tests {
    use crate::prelude::*;

    /// Calls `thunk` with `this = 0x2000` and one flags word, with a double
    /// over `target` that returns `0xdeadbeef`; checks that the double ran
    /// once, with `this` moved back by `back` and the flags passed on, and
    /// returns the thunk's `EAX`.
    fn check(thunk: u32, target: u32, back: u32) -> u32 {
        let mut e = Engine::new();
        e.register(target, |_, _| Ret {
            eax: 0xdead_beef,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        let result = e.call(thunk, &[0x2000u32, 0x1]);
        let log = e.call_log.take().unwrap();
        let calls: Vec<_> = log.iter().filter(|(a, _)| *a == target).collect();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].1, vec![0x2000 - back, 0x1]);
        result.u32()
    }

    #[test]
    fn thunk_00d68900() {
        assert_eq!(check(0x00d6_8900, 0x00d6_8910, 0x20), 0xdead_beef);
    }
}

#[cfg(test)]
mod active_viewer_id_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    type Seen = Rc<RefCell<Vec<Vec<u32>>>>;

    #[test]
    fn fn_00d688f0_answers_the_registered_process_id() {
        let mut e = Engine::new();
        e.map(0x0126_8000, 0x2000);
        e.set_global(ACTIVE_PROCESS_ID, 0x4242u32);
        assert_eq!(e.call(0x00d6_88f0, &args![0x99u32]).u32(), 0x4242);
    }

    #[test]
    fn fn_00d68940_hands_the_transform_the_id_and_the_tag_to_the_handler() {
        let mut e = Engine::new();
        e.map(0x0126_8000, 0x2000);
        e.map(0x0900_0000, 0x100);
        e.set_global(CONTACT_POINT_TAG, 0x55u32);
        // The handler object at 0x09000040 has its vtable at 0x09000000 (slot 0x10 below).
        e.mem.set_u32(0x0900_0040, 0x0900_0000);
        e.mem.set_u32(0x0900_0010, 0x0900_0021);
        let seen: Seen = Rc::default();
        let log = Rc::clone(&seen);
        e.register_double(0x0900_0021, move |_, a| {
            log.borrow_mut().push(a.to_vec());
            Ret::default()
        });
        let this = e.mem.alloc(0x40) + 0x20;
        e.mem.set_u32(this - 0x14, 0x0900_0040);
        e.call(0x00d6_8940, &args![this, 0x2000u32]);
        assert_eq!(
            *seen.borrow(),
            vec![vec![0x0900_0040, 0x20f0, 0x2013, 0x55]]
        );
    }
}

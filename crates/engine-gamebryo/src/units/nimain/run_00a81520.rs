//! `NiMain/run_00a81520` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

use crate::units::nimain::nimainsdm::{assign_ref_counted, ALLOCATE, OBJECT_BASE_CONSTRUCTOR};

/// `011f5ae8`: the pointer `00a81520` stores the object it builds in.
const SHADE_PROPERTY_SLOT: u32 = 0x011f_5ae8;

// Translated from 00a81520 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (range inside the NiMain subsystem; run by
/// the start-up `00a89cc0`, no arguments): builds a 0x20-byte object (allocation
/// `00aa13e0`, base constructor `00a5c200`, vtable `0109e274`, the word at `+0x1c` set to
/// -1, the word at `+0x18` to 1) and stores it, counted, in `011f5ae8`, releasing the
/// object held there before. A failed allocation stores null.
pub fn fn_00a81520(e: &mut Engine) {
    let block = e.call(ALLOCATE, &args![0x20u32]).u32();
    let object = if block == 0 {
        0
    } else {
        e.call(OBJECT_BASE_CONSTRUCTOR, &args![block]);
        e.mem.set_u32(block, 0x0109_e274);
        e.mem.set_u32(block + 0x1c, 0xffff_ffff);
        e.mem.set_u16(block + 0x18, 1);
        block
    };
    assign_ref_counted(e, SHADE_PROPERTY_SLOT, object);
}

/// Import slot of `InterlockedDecrement` (`stdcall`, one word: the address of the count).
const INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;

// Translated from 00a81590 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (no arguments; one of the `NiMain` shutdown steps of
/// `00a8a280`): releases the object that `00a81520` stores in `011f5ae8`. When it is not null, drops one reference
/// (`InterlockedDecrement` of the count at +4; at zero it
/// calls the object's vtable slot +4) and clears the pointer.
pub fn fn_00a81590(e: &mut Engine) {
    let object = e.mem.u32(SHADE_PROPERTY_SLOT);
    if object != 0 {
        let count = e
            .call(INTERLOCKED_DECREMENT, &args![object.wrapping_add(4)])
            .u32();
        if count == 0 {
            e.vcall(object, 4, &args![]);
        }
        e.mem.set_u32(SHADE_PROPERTY_SLOT, 0);
    }
}

// Translated from 00a814e0 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (a function Ghidra did not create; its address is in the
/// stream loader table `00a89cc0` builds, and by its body the creation function of
/// `NiShadeProperty`, the class its vtable `0109e274` names): a new 0x20-byte object
/// (allocation `00aa13e0`, base constructor `00a5c200`, the 16-bit flags at +0x18 set to 1,
/// the vtable, the word at +0x1c set to -1), or null when the allocation fails.
pub fn fn_00a814e0(e: &mut Engine) -> Ptr {
    let block = e.call(ALLOCATE, &args![0x20u32]).u32();
    if block == 0 {
        return Ptr::new(0);
    }
    e.call(OBJECT_BASE_CONSTRUCTOR, &args![block]);
    e.mem.set_u16(block + 0x18, 1);
    e.mem.set_u32(block, 0x0109_e274);
    e.mem.set_u32(block + 0x1c, 0xffff_ffff);
    Ptr::new(block)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a8_1520, fn_00a81520()),
        entry!(0x00a8_1590, fn_00a81590()),
        entry!(0x00a8_14e0, fn_00a814e0() -> Ptr),
    ]
}

#[cfg(test)]
mod start_up_tests {
    use super::*;
    use crate::units::nimain::nimainsdm::test_support::*;

    #[test]
    fn builds_the_object_and_replaces_the_held_one() {
        let run = run_default_instance(0x00a8_1520, SHADE_PROPERTY_SLOT, |_| {});
        let e = &run.engine;
        assert_eq!(e.mem.u32(run.object), 0x0109_e274);
        assert_eq!(e.mem.u32(run.object + 0x1c), 0xffff_ffff);
        assert_eq!(e.mem.u16(run.object + 0x18), 1);
        assert_eq!(e.mem.u32(run.object + 4), 1);
        assert_eq!(*run.freed.borrow(), vec![run.previous]);
    }

    #[test]
    fn a_failed_allocation_stores_null() {
        let run = run_default_instance(0x00a8_1520, SHADE_PROPERTY_SLOT, |e| {
            e.register(ALLOCATE, |_, _| 0u32.into_ret());
        });
        assert_eq!(run.object, 0);
    }
}

#[cfg(test)]
mod tests_b043 {
    use super::*;

    #[test]
    fn fn_00a81590_releases_and_clears_the_held_object() {
        let mut e = Engine::new();
        e.map(SHADE_PROPERTY_SLOT & !0xfff, 0x1000);
        e.put_vtable(0x0150_0000, &[0, 0x0150_1000]);
        e.register(0x0150_1000, |_, _| Ret::default());
        let object = e.mem.alloc(8);
        e.mem.set_u32(object, 0x0150_0000);
        let remaining = std::rc::Rc::new(std::cell::Cell::new(1u32));
        let counter = remaining.clone();
        e.register_double(INTERLOCKED_DECREMENT, move |_, _| {
            counter.set(counter.get() - 1);
            counter.get().into_ret()
        });
        // Still referenced elsewhere: no destructor.
        remaining.set(2);
        e.mem.set_u32(SHADE_PROPERTY_SLOT, object);
        e.call_log = Some(vec![]);
        e.call(0x00a8_1590, &args![]);
        let log = e.call_log.take().unwrap();
        assert!(log.contains(&(INTERLOCKED_DECREMENT, vec![object + 4])));
        assert!(log.iter().all(|call| call.0 != 0x0150_1000));
        assert_eq!(e.mem.u32(SHADE_PROPERTY_SLOT), 0);
        // Last reference: vtable slot +4 runs on the object.
        e.mem.set_u32(SHADE_PROPERTY_SLOT, object);
        e.call_log = Some(vec![]);
        e.call(0x00a8_1590, &args![]);
        let log = e.call_log.take().unwrap();
        assert!(log.contains(&(0x0150_1000, vec![object])));
        assert_eq!(e.mem.u32(SHADE_PROPERTY_SLOT), 0);
        // Null: nothing.
        e.call_log = Some(vec![]);
        e.call(0x00a8_1590, &args![]);
        assert_eq!(e.call_log.take().unwrap().len(), 1);
    }
}

#[cfg(test)]
mod create_tests {
    use super::*;
    use crate::units::nimain::nimainsdm::test_support::{calls_to, engine_with_counting};

    #[test]
    fn creates_the_shade_property_or_null() {
        let mut e = engine_with_counting();
        e.call_log = Some(vec![]);
        let object = e.call(0x00a8_14e0, &args![]).u32();
        assert_eq!(calls_to(&e, ALLOCATE), vec![vec![0x20]]);
        assert_eq!(calls_to(&e, OBJECT_BASE_CONSTRUCTOR), vec![vec![object]]);
        assert_eq!(e.mem.u32(object), 0x0109_e274);
        assert_eq!(e.mem.u16(object + 0x18), 1);
        assert_eq!(e.mem.u32(object + 0x1c), 0xffff_ffff);
        e.register(ALLOCATE, |_, _| 0u32.into_ret());
        assert_eq!(e.call(0x00a8_14e0, &args![]).u32(), 0);
    }
}

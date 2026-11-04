//! `fallout shared/run_0044f950` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! Besides the critical-section helpers: two walks over the `BSSimpleList`
//! at `+0x10C` of an object (its class is not confirmed) that pass each
//! element to one of the object's virtual functions.

#[allow(unused_imports)]
use crate::prelude::*;

/// `InitializeCriticalSection` (import slot).
const INITIALIZE_CRITICAL_SECTION: u32 = 0x00fd_f054;
/// `DeleteCriticalSection` (import slot, Ghidra's name for it).
const DELETE_CRITICAL_SECTION: u32 = 0x00fd_f058;
/// `00404d30`: the object's own check before a walk (a `bool`).
const OBJECT_CHECK: u32 = 0x0040_4d30;
/// `006815c0`: returns its `this` (the node; its first word is the element).
const NODE_SELF: u32 = 0x0068_15c0;
/// `00726070`: a `BSSimpleList` node's next node.
const NODE_NEXT: u32 = 0x0072_6070;
/// The memory-context guard (`00404eb0` with four words, `00404ee0`).
const CONTEXT_GUARD_ENTER: u32 = 0x0040_4eb0;
const CONTEXT_GUARD_LEAVE: u32 = 0x0040_4ee0;
/// The source-file string the guarded walk passes.
const SOURCE_FILE: u32 = 0x0101_608c;

// Translated from 0044fae0 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// A critical section's constructor (`thiscall`): `InitializeCriticalSection`
/// on `this` (the import, `00fdf054`), then returns `this`.
pub fn fn_0044fae0(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(INITIALIZE_CRITICAL_SECTION, &args![this]);
    this
}

// Translated from 0044f950 (decompiled, FalloutNV.exe 1.4.0.525)
/// When the object's check (`00404d30`) passes, hands every element of the
/// list at `+0x10C` to the object's vtable +0x10, and-ing their results
/// into the check's; returns that.
pub fn fn_0044f950(e: &mut Engine, this: Ptr) -> u8 {
    let mut result = e.call(OBJECT_CHECK, &args![this]).u8();
    if result != 0 {
        let mut node = fn_0044faa0(e, this);
        while node != 0 {
            let slot = e.call(NODE_SELF, &args![node]).u32();
            let element = e.mem.u32(slot);
            result &= e.vcall(this.addr(), 0x10, &args![element]).u8();
            node = fn_0044fac0(e, this, node);
        }
    }
    result
}

// Translated from 0044f9c0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0044f9c0 - the compiler's exception-unwinding frame (FS:[0]) is not translated
/// As [`fn_0044f950`] with the object's vtable +0x14, skipping null
/// elements, under a memory-context guard (`00404eb0` with `9, 1, <source
/// file>, 0x357`).
pub fn fn_0044f9c0(e: &mut Engine, this: Ptr) -> u8 {
    e.with_stack(4, |e, guard| {
        e.call(
            CONTEXT_GUARD_ENTER,
            &args![guard, 9u32, 1u32, SOURCE_FILE, 0x357u32],
        );
        let mut result = e.call(OBJECT_CHECK, &args![this]).u8();
        if result != 0 {
            let mut node = fn_0044faa0(e, this);
            while node != 0 {
                let slot = e.call(NODE_SELF, &args![node]).u32();
                if e.mem.u32(slot) != 0 {
                    let slot = e.call(NODE_SELF, &args![node]).u32();
                    let element = e.mem.u32(slot);
                    result &= e.vcall(this.addr(), 0x14, &args![element]).u8();
                }
                node = fn_0044fac0(e, this, node);
            }
        }
        e.call(CONTEXT_GUARD_LEAVE, &args![guard]);
        result
    })
}

// Translated from 0044faa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The address of the list at `+0x10C` (its first node).
pub fn fn_0044faa0(_e: &mut Engine, this: Ptr) -> u32 {
    this.addr().wrapping_add(0x10c)
}

// Translated from 0044fac0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The node after `node` (`00726070` on it); `this` is not read.
pub fn fn_0044fac0(e: &mut Engine, _this: Ptr, node: u32) -> u32 {
    e.call(NODE_NEXT, &args![node]).u32()
}

// Translated from 0044fb00 (decompiled, FalloutNV.exe 1.4.0.525)
/// A critical section's destructor (`thiscall`): `DeleteCriticalSection` on
/// `this` (the import, `00fdf058`).
pub fn fn_0044fb00(e: &mut Engine, this: Ptr) {
    e.call(DELETE_CRITICAL_SECTION, &args![this]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0044fae0, fn_0044fae0(Ptr) -> Ptr),
        entry!(0x0044_f950, fn_0044f950(Ptr) -> u8),
        entry!(0x0044_f9c0, fn_0044f9c0(Ptr) -> u8),
        entry!(0x0044_faa0, fn_0044faa0(Ptr) -> u32),
        entry!(0x0044_fac0, fn_0044fac0(Ptr, u32) -> u32),
        entry!(0x0044_fb00, fn_0044fb00(Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn initializes_the_section_and_returns_it() {
        let mut e = Engine::new();
        e.register(INITIALIZE_CRITICAL_SECTION, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0044fae0, &[0x5000]).u32(), 0x5000);
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1], (INITIALIZE_CRITICAL_SECTION, vec![0x5000]));
    }

    fn calls_to(e: &Engine, target: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == target)
            .map(|(_, words)| words.clone())
            .collect()
    }

    /// An object whose list at `+0x10C` holds the elements 5, 0 and 7 and
    /// whose slots +0x10 and +0x14 return 1 except for element 7.
    fn walk_engine(check: u8) -> (Engine, u32) {
        let mut e = Engine::new();
        e.register(NODE_SELF, |_, a| a[0].into_ret());
        e.register(NODE_NEXT, |e, a| e.mem.u32(a[0] + 4).into_ret());
        e.register(CONTEXT_GUARD_ENTER, |_, a| a[0].into_ret());
        e.register(CONTEXT_GUARD_LEAVE, |_, _| Ret::default());
        e.register_double(OBJECT_CHECK, move |_, _| check.into_ret());
        e.put_vtable(0x0060_0000, &[0, 0, 0, 0, 0x0061_0010, 0x0061_0014]);
        e.register(0x0061_0010, |_, a| u8::from(a[1] != 7).into_ret());
        e.register(0x0061_0014, |_, a| u8::from(a[1] != 7).into_ret());
        let this = e.mem.alloc(0x114);
        e.mem.set_u32(this, 0x0060_0000);
        let second = e.mem.alloc(8);
        let third = e.mem.alloc(8);
        e.mem.set_u32(this + 0x10c, 5);
        e.mem.set_u32(this + 0x110, second);
        e.mem.set_u32(second + 4, third);
        e.mem.set_u32(third, 7);
        (e, this)
    }

    #[test]
    fn walk_passes_every_element_to_slot_0x10() {
        let (mut e, this) = walk_engine(1);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0044_f950, &args![this]).u8(), 0);
        let seen: Vec<u32> = calls_to(&e, 0x0061_0010).iter().map(|w| w[1]).collect();
        assert_eq!(seen, vec![5, 0, 7]);
        let (mut e, this) = walk_engine(0);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0044_f950, &args![this]).u8(), 0);
        assert!(calls_to(&e, 0x0061_0010).is_empty());
    }

    #[test]
    fn guarded_walk_skips_null_elements() {
        let (mut e, this) = walk_engine(1);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0044_f9c0, &args![this]).u8(), 0);
        let seen: Vec<u32> = calls_to(&e, 0x0061_0014).iter().map(|w| w[1]).collect();
        assert_eq!(seen, vec![5, 7]);
        let guards = calls_to(&e, CONTEXT_GUARD_ENTER);
        assert_eq!(guards[0][1..], [9, 1, SOURCE_FILE, 0x357]);
        assert_eq!(calls_to(&e, CONTEXT_GUARD_LEAVE).len(), 1);
    }

    #[test]
    fn list_helpers_and_section_destructor() {
        let mut e = Engine::new();
        e.register(NODE_NEXT, |_, a| (a[0] + 1).into_ret());
        e.register(DELETE_CRITICAL_SECTION, |_, _| Ret::default());
        assert_eq!(e.call(0x0044_faa0, &args![0x1000u32]).u32(), 0x110c);
        assert_eq!(e.call(0x0044_fac0, &args![0x1000u32, 0x20u32]).u32(), 0x21);
        e.call_log = Some(vec![]);
        e.call(0x0044_fb00, &args![0x5000u32]);
        assert_eq!(calls_to(&e, DELETE_CRITICAL_SECTION), vec![vec![0x5000]]);
    }
}

//! `fallout/misc/run_00866690` (Xbox PDB source unit), subsystem `fallout/misc`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `operator new(size)` (cdecl).
const OPERATOR_NEW: u32 = 0x0040_1000;
/// The 4-byte object's constructor (`thiscall`, no arguments).
const INNER_CONSTRUCT: u32 = 0x00aa_2230;
/// The 8-byte holder's constructor (`thiscall(holder, inner)`).
const HOLDER_CONSTRUCT: u32 = 0x00aa_21d0;
/// The global the holder is stored in (the allocator start-up's holder,
/// docs/ENGINE_CRATE.md).
const HOLDER_GLOBAL: u32 = 0x011f_6118;
/// The next start-up step (no arguments).
const NEXT_STEP: u32 = 0x00aa_2020;
/// `BSSystemUtility::QInstance` (engine map): the singleton, whose vtable
/// slot `+4` is called with three zero words.
const SYSTEM_UTILITY_INSTANCE: u32 = 0x00af_2640;

// Translated from 00866690 (decompiled, FalloutNV.exe 1.4.0.525)
/// The allocator start-up `MemoryManager::MemoryManager` (`00aa3880`)
/// runs (`thiscall`; `this` is saved but not read): makes an 8-byte holder
/// (`00aa21d0`) around a new 4-byte object (`00aa2230`, or null when its
/// allocation fails), stores it (or null) at `011f6118`, runs `00aa2020`,
/// then calls vtable slot `+4` of `BSSystemUtility::QInstance()` with
/// `(0, 0, 0)`.
// Translation incomplete: 00866690 - the FS:[0] exception frame (frees the blocks if a constructor throws) is not translated
pub fn fn_00866690(e: &mut Engine, _unused_this: Ptr) {
    let block = e.call(OPERATOR_NEW, &args![8u32]).u32();
    let holder = if block == 0 {
        0
    } else {
        let inner_block = e.call(OPERATOR_NEW, &args![4u32]).u32();
        let inner = if inner_block == 0 {
            0
        } else {
            e.call(INNER_CONSTRUCT, &args![inner_block]).u32()
        };
        e.call(HOLDER_CONSTRUCT, &args![block, inner]).u32()
    };
    e.set_global(HOLDER_GLOBAL, holder);
    e.call(NEXT_STEP, &[]);
    let utility = e.call(SYSTEM_UTILITY_INSTANCE, &[]).u32();
    e.vcall(utility, 4, &args![0u32, 0u32, 0u32]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00866690, fn_00866690(Ptr))]
}

#[cfg(test)]
mod tests {
    use super::*;

    const SLOT: u32 = 0x0f20_0004;

    fn engine() -> (Engine, u32) {
        let mut e = Engine::new();
        e.map(0x011f_6000, 0x1000);
        e.register(INNER_CONSTRUCT, |_, a| a[0].into_ret());
        e.register(HOLDER_CONSTRUCT, |_, a| a[0].into_ret());
        e.register(NEXT_STEP, |_, _| Ret::default());
        e.register(SLOT, |_, _| Ret::default());
        let vtable = e.mem.alloc(8);
        e.mem.set_u32(vtable + 4, SLOT);
        let utility = e.mem.alloc(4);
        e.mem.set_u32(utility, vtable);
        e.register_double(SYSTEM_UTILITY_INSTANCE, move |_, _| utility.into_ret());
        (e, utility)
    }

    #[test]
    fn stores_the_holder_and_calls_the_utility() {
        let (mut e, utility) = engine();
        e.register(OPERATOR_NEW, |e, a| e.mem.alloc(a[0]).into_ret());
        e.call_log = Some(vec![]);
        e.call(0x0086_6690, &args![0u32]);
        let log = e.call_log.take().unwrap();
        let holder = log.iter().find(|(a, _)| *a == HOLDER_CONSTRUCT).unwrap();
        let inner = log.iter().find(|(a, _)| *a == INNER_CONSTRUCT).unwrap();
        assert_eq!(holder.1[1], inner.1[0]);
        assert_eq!(e.global::<u32>(HOLDER_GLOBAL), holder.1[0]);
        let slot = log.iter().find(|(a, _)| *a == SLOT).unwrap();
        assert_eq!(slot.1, vec![utility, 0, 0, 0]);
        let order: Vec<u32> = log
            .iter()
            .map(|(a, _)| *a)
            .filter(|a| [NEXT_STEP, SYSTEM_UTILITY_INSTANCE].contains(a))
            .collect();
        assert_eq!(order, vec![NEXT_STEP, SYSTEM_UTILITY_INSTANCE]);
    }

    #[test]
    fn failed_allocations_give_null() {
        // The holder's block fails: null holder, no constructors.
        let (mut e, _) = engine();
        e.register(OPERATOR_NEW, |_, _| 0u32.into_ret());
        e.set_global(HOLDER_GLOBAL, 5u32);
        e.call_log = Some(vec![]);
        e.call(0x0086_6690, &args![0u32]);
        let log = e.call_log.take().unwrap();
        assert!(!log.iter().any(|(a, _)| *a == HOLDER_CONSTRUCT));
        assert_eq!(e.global::<u32>(HOLDER_GLOBAL), 0);
        // Only the inner block fails: the holder gets null.
        let (mut e, _) = engine();
        e.register(OPERATOR_NEW, |e, a| {
            if a[0] == 8 {
                e.mem.alloc(8).into_ret()
            } else {
                0u32.into_ret()
            }
        });
        e.call_log = Some(vec![]);
        e.call(0x0086_6690, &args![0u32]);
        let log = e.call_log.take().unwrap();
        assert!(!log.iter().any(|(a, _)| *a == INNER_CONSTRUCT));
        let holder = log.iter().find(|(a, _)| *a == HOLDER_CONSTRUCT).unwrap();
        assert_eq!(holder.1[1], 0);
    }
}

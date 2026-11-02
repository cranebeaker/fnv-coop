//! `fallout/misc/run_008798e0` (Xbox PDB source unit), subsystem `fallout/misc`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The `SeenData` vtable (`008799a0` stores the same address).
const SEEN_DATA_VTABLE: u32 = 0x0108_3fc4;
/// `memset`-shaped helper (`00403d30`, `cdecl`: destination, value, count).
const MEMSET_WRAPPER: u32 = 0x0040_3d30;

// Translated from 00879940 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `SeenData` constructor (no name in the engine map; the vtable is the
/// one `SeenData`'s destructor body `008799a0` restores): stores the vtable,
/// clears the 0x20 bytes of seen bits at +4 (`00403d30(this + 4, 0, 0x20)`)
/// and returns `this`.
pub fn fn_00879940(e: &mut Engine, this: Ptr) -> Ptr {
    e.mem.set_u32(this.addr(), SEEN_DATA_VTABLE);
    e.call(
        MEMSET_WRAPPER,
        &args![this.addr().wrapping_add(4), 0u32, 0x20u32],
    );
    this
}

/// `00717e50` (`thiscall`; engine map
/// `BaseProcess::GetActorPackageThatIsRunning`): the word `008798e0` tests.
const RUNNING_PACKAGE: u32 = 0x0071_7e50;
/// `005e0830` (`thiscall`, `RET 8`: that word, 1).
const PACKAGE_TEST_005E0830: u32 = 0x005e_0830;

// Translated from 008798e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`ECX` only): false without the word from
/// `00717e50`; otherwise `005e0830` with it and 1, and when that answers
/// true the answer of vtable `+0x20`, after which vtable `+0x1c` runs.
pub fn fn_008798e0(e: &mut Engine, this: Ptr) -> bool {
    let package = e.call(RUNNING_PACKAGE, &args![this]).u32();
    if package == 0 {
        return false;
    }
    if !e
        .call(PACKAGE_TEST_005E0830, &args![this, package, 1u32])
        .bool()
    {
        return false;
    }
    let result = e.vcall(this.addr(), 0x20, &args![]).bool();
    e.vcall(this.addr(), 0x1c, &args![]);
    result
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0087_9940, fn_00879940(Ptr) -> Ptr),
        entry!(0x0087_98e0, fn_008798e0(Ptr) -> bool),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fn_00879940_sets_the_vtable_and_clears_the_bits() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x24);
        e.register(MEMSET_WRAPPER, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0087_9940, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), SEEN_DATA_VTABLE);
        let log = e.call_log.take().unwrap();
        let calls: Vec<_> = log.iter().filter(|(a, _)| *a == MEMSET_WRAPPER).collect();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].1, vec![this + 4, 0, 0x20]);
    }
}

#[cfg(test)]
mod n072_tests {
    use super::*;

    fn run(package: u32, test: u32, slot: u32) -> (bool, usize) {
        let mut e = Engine::new();
        let mut slots = vec![0u32; 9];
        slots[7] = 0x0150_101c;
        slots[8] = 0x0150_1020;
        e.put_vtable(0x0150_0000, &slots);
        e.register_double(0x0150_1020, move |_, _| slot.into_ret());
        e.register(0x0150_101c, |_, _| 0u32.into_ret());
        e.register_double(RUNNING_PACKAGE, move |_, _| package.into_ret());
        e.register_double(PACKAGE_TEST_005E0830, move |_, a| {
            assert_eq!(a[1..], [package, 1]);
            test.into_ret()
        });
        let this = e.mem.alloc(4);
        e.mem.set_u32(this, 0x0150_0000);
        e.call_log = Some(vec![]);
        let result = e.call(0x0087_98e0, &args![this]).bool();
        let log = e.call_log.take().unwrap();
        (result, log.iter().filter(|c| c.0 == 0x0150_101c).count())
    }

    #[test]
    fn fn_008798e0_answers_the_slot_after_both_tests() {
        assert_eq!(run(0, 1, 1), (false, 0));
        assert_eq!(run(0x40, 0, 1), (false, 0));
        assert_eq!(run(0x40, 1, 1), (true, 1));
        assert_eq!(run(0x40, 1, 0x100), (false, 1));
    }
}

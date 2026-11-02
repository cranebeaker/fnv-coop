//! `Havok SDK/run_00d9e250` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `hkpWorld::addWorldPostSimulationListener` (`00c93920`, engine map name): `thiscall`, one word,
/// the listener.
const WORLD_ADD_POST_SIMULATION_LISTENER: u32 = 0x00c9_3920;
/// The removal paired with `00c93920` (`00c918e0`; the map has no name for it).
const WORLD_REMOVE_POST_SIMULATION_LISTENER: u32 = 0x00c9_18e0;

// Translated from 00d9e250 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thiscall with one stack word, `RET 4`: calls `hkpWorld::addWorldPostSimulationListener`
/// (`00c93920`) on the world `arg` with the listener `this + 8`, or null when `this` is null. The
/// class is unconfirmed (the embedded post-simulation listener of a viewer).
pub fn fn_00d9e250(e: &mut Engine, this: Ptr, arg: u32) {
    let listener = if this.addr() == 0 {
        0
    } else {
        this.addr().wrapping_add(8)
    };
    e.call(WORLD_ADD_POST_SIMULATION_LISTENER, &args![arg, listener]);
}

// Translated from 00d9e280 (decompiled, FalloutNV.exe 1.4.0.525)
/// As `00d9e250` through the removal `00c918e0`.
pub fn fn_00d9e280(e: &mut Engine, this: Ptr, arg: u32) {
    let listener = if this.addr() == 0 {
        0
    } else {
        this.addr().wrapping_add(8)
    };
    e.call(WORLD_REMOVE_POST_SIMULATION_LISTENER, &args![arg, listener]);
}

// Translated from 00d9e2b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thiscall with two stack words `(out, other)`, `RET 8`: the byte at `out` becomes 1 when the
/// first three words of `other` equal those of `this`, otherwise 0 (a hidden-return `hkBool`).
/// Returns `out`. The class is unconfirmed (`hkpendofstepcallbackutil.rs` calls it as
/// `ENTRIES_EQUAL`).
pub fn fn_00d9e2b0(e: &mut Engine, this: Ptr, out: u32, other: u32) -> u32 {
    let this = this.addr();
    let equal = (0..3u32).all(|word| e.mem.u32(other + 4 * word) == e.mem.u32(this + 4 * word));
    e.mem.set_u8(out, u8::from(equal));
    out
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00d9_e250, fn_00d9e250(Ptr, u32)),
        entry!(0x00d9_e280, fn_00d9e280(Ptr, u32)),
        entry!(0x00d9_e2b0, fn_00d9e2b0(Ptr, u32, u32) -> u32),
    ]
}

#[cfg(test)]
mod a09_tests {
    use super::*;

    #[test]
    fn the_listener_functions_pass_this_plus_8_or_null() {
        let mut e = Engine::new();
        e.register(WORLD_ADD_POST_SIMULATION_LISTENER, |_, _| Ret::default());
        e.register(WORLD_REMOVE_POST_SIMULATION_LISTENER, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x00d9_e250, &args![0x3000u32, 0x41u32]);
        e.call(0x00d9_e250, &args![0u32, 0x42u32]);
        e.call(0x00d9_e280, &args![0x4000u32, 0x43u32]);
        e.call(0x00d9_e280, &args![0u32, 0x44u32]);
        let log = e.call_log.take().unwrap();
        let calls: Vec<_> = log
            .iter()
            .filter(|(a, _)| {
                *a == WORLD_ADD_POST_SIMULATION_LISTENER
                    || *a == WORLD_REMOVE_POST_SIMULATION_LISTENER
            })
            .collect();
        assert_eq!(calls.len(), 4);
        assert_eq!(
            *calls[0],
            (WORLD_ADD_POST_SIMULATION_LISTENER, vec![0x41, 0x3008])
        );
        assert_eq!(
            *calls[1],
            (WORLD_ADD_POST_SIMULATION_LISTENER, vec![0x42, 0])
        );
        assert_eq!(
            *calls[2],
            (WORLD_REMOVE_POST_SIMULATION_LISTENER, vec![0x43, 0x4008])
        );
        assert_eq!(
            *calls[3],
            (WORLD_REMOVE_POST_SIMULATION_LISTENER, vec![0x44, 0])
        );
    }

    #[test]
    fn the_comparison_wants_all_three_words_equal() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x10);
        let other = e.mem.alloc(0x10);
        let out = e.mem.alloc(8);
        for word in 0..3 {
            e.mem.set_u32(this + 4 * word, 10 + word);
            e.mem.set_u32(other + 4 * word, 10 + word);
        }
        e.mem.set_u32(this + 0xc, 1);
        e.mem.set_u32(other + 0xc, 2);
        e.mem.set_u8(out, 0xee);
        assert_eq!(e.call(0x00d9_e2b0, &args![this, out, other]).u32(), out);
        assert_eq!(e.mem.u8(out), 1);
        for word in 0..3 {
            e.mem.set_u32(other + 4 * word, 99);
            e.call(0x00d9_e2b0, &args![this, out, other]);
            assert_eq!(e.mem.u8(out), 0);
            e.mem.set_u32(other + 4 * word, 10 + word);
        }
        // Only the byte at `out` is written.
        e.mem.set_u32(out, 0x1234_5678);
        e.call(0x00d9_e2b0, &args![this, out, other]);
        assert_eq!(e.mem.u32(out), 0x1234_5601);
    }
}

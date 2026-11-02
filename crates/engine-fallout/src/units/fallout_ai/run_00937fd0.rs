//! `fallout/ai/run_00937fd0` (Xbox PDB source unit), subsystem `fallout/ai`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `0066e920` (`cdecl`, one word): the actor value's info record (0 outside
/// `0..0x4d`).
const ACTOR_VALUE_INFO: u32 = 0x0066_e920;
/// The slot table (0x4D words, one per actor value), its slot count and the
/// byte that says it is built.
const FAST_SLOT_INDEX: u32 = 0x011e_0618;
const FAST_SLOT_COUNT: u32 = 0x011e_0750;
const FAST_SLOTS_BUILT: u32 = 0x011e_074c;
/// The number of actor values.
const ACTOR_VALUE_COUNT: i32 = 0x4d;

// Translated from 00937fd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (no arguments): builds, once, the table at
/// `011e0618` that gives each of the 0x4D actor values whose info record
/// (`0066e920`) has bit 0 of its word at +0x44 set the next slot number
/// (0, 1, ...), and every other value 0x4D; the slot count goes to
/// `011e0750` and the built flag at `011e074c` is set.
pub fn fn_00937fd0(e: &mut Engine) {
    if e.mem.u8(FAST_SLOTS_BUILT) != 0 {
        return;
    }
    let mut slots = 0i32;
    for index in 0..ACTOR_VALUE_COUNT {
        let info = e.call(ACTOR_VALUE_INFO, &args![index]).u32();
        let at = FAST_SLOT_INDEX.wrapping_add((index as u32).wrapping_mul(4));
        if info != 0 && e.mem.u32(info + 0x44) & 1 != 0 {
            e.mem.set_u32(at, slots as u32);
            slots += 1;
        } else {
            e.mem.set_u32(at, ACTOR_VALUE_COUNT as u32);
        }
    }
    e.mem.set_u32(FAST_SLOT_COUNT, slots as u32);
    e.mem.set_u8(FAST_SLOTS_BUILT, 1);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00937fd0, fn_00937fd0())]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn slots_go_to_values_with_bit_0_once() {
        let mut e = Engine::new();
        e.map(0x011e_0000, 0x1000);
        let flagged = e.mem.alloc(0x48);
        e.mem.set_u32(flagged + 0x44, 1);
        let plain = e.mem.alloc(0x48);
        e.mem.set_u32(plain + 0x44, 2);
        e.register_double(ACTOR_VALUE_INFO, move |_, a| {
            match a[0] {
                3 | 10 => flagged,
                4 => plain,
                _ => 0,
            }
            .into_ret()
        });
        e.call_log = Some(vec![]);
        e.call(0x0093_7fd0, &[]);
        assert_eq!(e.mem.u32(FAST_SLOT_INDEX + 3 * 4), 0);
        assert_eq!(e.mem.u32(FAST_SLOT_INDEX + 10 * 4), 1);
        assert_eq!(e.mem.u32(FAST_SLOT_INDEX + 4 * 4), 0x4d);
        assert_eq!(e.mem.u32(FAST_SLOT_INDEX), 0x4d);
        assert_eq!(e.mem.u32(FAST_SLOT_INDEX + 0x4c * 4), 0x4d);
        assert_eq!(e.mem.u32(FAST_SLOT_COUNT), 2);
        assert_eq!(e.mem.u8(FAST_SLOTS_BUILT), 1);
        let asked = e
            .call_log
            .take()
            .unwrap()
            .iter()
            .filter(|c| c.0 == ACTOR_VALUE_INFO)
            .count();
        assert_eq!(asked, 0x4d);
        // Built: nothing runs again.
        e.call_log = Some(vec![]);
        e.mem.set_u32(FAST_SLOT_COUNT, 9);
        e.call(0x0093_7fd0, &[]);
        assert_eq!(e.mem.u32(FAST_SLOT_COUNT), 9);
    }
}

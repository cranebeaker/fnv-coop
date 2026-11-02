//! `BSShader/run_00b64d60` (Xbox PDB source unit), subsystem `BSShader`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// thiscall (object, index): the per-entry update `00b64d60` runs on each
/// active entry (`00b63540`, in `BSShader/bsshaderaccumulator.cpp` by the
/// engine map).
const ENTRY_UPDATE_B63540: u32 = 0x00b6_3540;

// Translated from 00b64d60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (its class is not confirmed), thiscall
/// without stack words. The object holds three 0x14-byte entries from
/// `+0x4c` (an active byte at +0, a float at +4, a count at +8 and a
/// counter at +0xc), a busy byte at `+0x40`, a float at `+0x44` and a
/// count at `+0x3c`. Each active entry is updated (`00b63540(index)`);
/// one still active afterwards bumps its counter, sets the busy byte and
/// marks the pass as not settled. Entries whose float is above 0 or whose
/// count is not 0 take part in the smallest count (from 0x7fffffff) and the
/// largest float (from 0.0). When the pass is settled, clears the busy
/// byte, stores the largest float at `+0x44` (1.0 when the smallest count
/// is below 10), the smallest count at `+0x3c`, and clears the three
/// entries' floats and counts. NaN floats compare neither above nor below.
pub fn fn_00b64d60(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    let mut smallest = 0x7fff_ffffu32;
    let mut largest = 0.0f32.to_bits();
    let mut settled = true;
    for index in 0..3u32 {
        let entry = base + 0x4c + 0x14 * index;
        if e.mem.u8(entry) != 0 {
            e.call(ENTRY_UPDATE_B63540, &args![base, index]);
            if e.mem.u8(entry) != 0 {
                let counter = e.mem.u32(entry + 0xc).wrapping_add(1);
                e.mem.set_u32(entry + 0xc, counter);
                e.mem.set_u8(base + 0x40, 1);
                settled = false;
            }
        }
        let above_zero = 0.0 < x87_m32(e.mem.u32(entry + 4));
        if above_zero || e.mem.u32(entry + 8) != 0 {
            let count = e.mem.u32(entry + 8);
            if count < smallest {
                smallest = count;
            }
            let value = x87_quiet_bits(e.mem.u32(entry + 4));
            if x87_load32(largest) < x87_load32(value) {
                largest = value;
            }
        }
    }
    if !settled {
        return;
    }
    e.mem.set_u8(base + 0x40, 0);
    e.mem.set_u32(base + 0x44, x87_quiet_bits(largest));
    if smallest < 0xa {
        e.mem.set_u32(base + 0x44, 1.0f32.to_bits());
    }
    e.mem.set_u32(base + 0x3c, smallest);
    for index in 0..3u32 {
        let entry = base + 0x4c + 0x14 * index;
        e.mem.set_u32(entry + 4, 0.0f32.to_bits());
    }
    for index in 0..3u32 {
        let entry = base + 0x4c + 0x14 * index;
        e.mem.set_u32(entry + 8, 0);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00b64d60, fn_00b64d60(Ptr))]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn object(e: &mut Engine, entries: [(u8, f32, u32); 3]) -> u32 {
        let this = e.mem.alloc(0x90);
        for (index, (active, value, count)) in entries.into_iter().enumerate() {
            let entry = this + 0x4c + 0x14 * index as u32;
            e.mem.set_u8(entry, active);
            e.mem.set_f32(entry + 4, value);
            e.mem.set_u32(entry + 8, count);
        }
        this
    }

    #[test]
    fn a_settled_pass_keeps_the_extremes_and_clears_the_entries() {
        let mut e = Engine::new();
        // The update leaves entry 1 inactive.
        e.register(ENTRY_UPDATE_B63540, |e, a| {
            e.mem.set_u8(a[0] + 0x4c + 0x14 * a[1], 0);
            Ret::default()
        });
        let this = object(&mut e, [(0, 0.5, 12), (1, 0.75, 20), (0, -1.0, 0)]);
        e.call_log = Some(vec![]);
        e.call(0x00b6_4d60, &args![this]);
        let log = e.call_log.take().unwrap();
        let updates: Vec<_> = log.iter().filter(|c| c.0 == ENTRY_UPDATE_B63540).collect();
        assert_eq!(updates.len(), 1);
        assert_eq!(updates[0].1, vec![this, 1]);
        assert_eq!(e.mem.u8(this + 0x40), 0);
        assert_eq!(e.mem.f32(this + 0x44), 0.75);
        assert_eq!(e.mem.u32(this + 0x3c), 12);
        for index in 0..3 {
            assert_eq!(e.mem.u32(this + 0x50 + 0x14 * index), 0);
            assert_eq!(e.mem.u32(this + 0x54 + 0x14 * index), 0);
        }
        // A smallest count below 10 gives 1.0.
        let this = object(&mut e, [(0, 0.5, 3), (0, 0.0, 0), (0, f32::NAN, 0)]);
        e.call(0x00b6_4d60, &args![this]);
        assert_eq!(e.mem.f32(this + 0x44), 1.0);
        assert_eq!(e.mem.u32(this + 0x3c), 3);
    }

    #[test]
    fn an_entry_still_active_keeps_the_pass_open() {
        let mut e = Engine::new();
        e.register(ENTRY_UPDATE_B63540, |_, _| Ret::default());
        let this = object(&mut e, [(1, 0.5, 12), (0, 0.0, 0), (0, 0.0, 0)]);
        e.mem.set_u32(this + 0x58, 4);
        e.mem.set_u32(this + 0x3c, 0x99);
        e.call(0x00b6_4d60, &args![this]);
        assert_eq!(e.mem.u32(this + 0x58), 5);
        assert_eq!(e.mem.u8(this + 0x40), 1);
        assert_eq!(e.mem.u32(this + 0x3c), 0x99);
        assert_eq!(e.mem.f32(this + 0x50), 0.5);
    }
}

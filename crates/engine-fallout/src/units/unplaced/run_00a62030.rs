//! `(unplaced)/run_00a62030` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// Source of the 0x44 bytes (17 words) `00a62110` copies, `011aa2a0`.
const COPY_SOURCE: u32 = 0x011a_a2a0;
/// Destination of the copy, `011f4600`.
const COPY_DESTINATION: u32 = 0x011f_4600;

// Translated from 00a62110 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (no unit in the engine map, next to `00a62030` and `00a62090`; run by the
/// start-up `00a89cc0`, no arguments): copies 17 words from `011aa2a0` to `011f4600`
/// (`REP MOVSD`).
pub fn fn_00a62110(e: &mut Engine) {
    for index in 0..0x11u32 {
        let word = e.mem.u32(COPY_SOURCE + 4 * index);
        e.mem.set_u32(COPY_DESTINATION + 4 * index, word);
    }
}

/// The memory manager singleton and its deallocate (`00aa4060`, `thiscall`, one word).
const MEMORY_MANAGER: u32 = 0x011f_6238;
const DEALLOCATE: u32 = 0x00aa_4060;
/// `011f4468`: the object `00a62030` and `00a62090` work on (`thiscall` object of
/// `00a615c0`).
const PALETTE_OBJECT: u32 = 0x011f_4468;
/// `011f4464`: the array of 0xb28-byte entries (count in the word in front of it).
const PALETTE_ENTRIES: u32 = 0x011f_4464;
/// `011f4480`: the object `0040fbf0` runs on; its word at +4 (`011f4484`) is a count
/// `00a62030` decrements, clearing `011f4480` when it reaches 0.
const PALETTE_LOCK: u32 = 0x011f_4480;
/// `0040fbf0` (`thiscall`, `RET 4`).
const LOCK_ENTER: u32 = 0x0040_fbf0;
/// `00a615c0` and `00a61ec0` (`thiscall`, no stack word; unit `NiMain/bstexturepalette.cpp`).
const PALETTE_CLEAR: u32 = 0x00a6_15c0;
const PALETTE_ENTRY_CLEAR: u32 = 0x00a6_1ec0;
/// Size of one entry of the array at `011f4464`.
const ENTRY_SIZE: u32 = 0xb28;

// Translated from 00a62030 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (no arguments): when `011f4468` holds the object, runs
/// `0040fbf0(0)` on `011f4480`, `00a615c0` on the object, and `00a61ec0` on the first seven
/// 0xb28-byte entries of the array at `011f4464` (the pointer read again for each); then
/// decrements the word at `011f4484` and clears `011f4480` when it reaches 0.
pub fn fn_00a62030(e: &mut Engine) {
    if e.mem.u32(PALETTE_OBJECT) == 0 {
        return;
    }
    e.call(LOCK_ENTER, &args![PALETTE_LOCK, 0u32]);
    let object = e.mem.u32(PALETTE_OBJECT);
    e.call(PALETTE_CLEAR, &args![object]);
    let mut offset = 0u32;
    while offset < 0x4e18 {
        let entries = e.mem.u32(PALETTE_ENTRIES);
        e.call(PALETTE_ENTRY_CLEAR, &args![offset.wrapping_add(entries)]);
        offset += ENTRY_SIZE;
    }
    let count = e.mem.u32(PALETTE_LOCK + 4).wrapping_sub(1);
    e.mem.set_u32(PALETTE_LOCK + 4, count);
    if count == 0 {
        e.mem.set_u32(PALETTE_LOCK, 0);
    }
}

// Translated from 00a62090 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (no arguments): runs [`fn_00a62030`]; when `011f4468` holds the
/// object, `00a615c0` runs on it and it goes back to the memory manager; the pointer is
/// cleared. Then, when the array at `011f4464` is set, `00a61ec0` runs on each of its
/// 0xb28-byte entries from the last to the first (count in the word in front of it) and the
/// block, count word included, goes back to the memory manager; the pointer is cleared.
pub fn fn_00a62090(e: &mut Engine) {
    fn_00a62030(e);
    let object = e.mem.u32(PALETTE_OBJECT);
    if object != 0 {
        e.call(PALETTE_CLEAR, &args![object]);
        e.call(DEALLOCATE, &args![MEMORY_MANAGER, object]);
    }
    let entries = e.mem.u32(PALETTE_ENTRIES);
    e.mem.set_u32(PALETTE_OBJECT, 0);
    if entries != 0 {
        let block = entries.wrapping_sub(4);
        let count = e.mem.u32(block);
        let mut entry = count.wrapping_mul(ENTRY_SIZE).wrapping_add(entries);
        let mut remaining = count.wrapping_sub(1) as i32;
        while remaining >= 0 {
            entry = entry.wrapping_sub(ENTRY_SIZE);
            e.call(PALETTE_ENTRY_CLEAR, &args![entry]);
            remaining = remaining.wrapping_sub(1);
        }
        e.call(DEALLOCATE, &args![MEMORY_MANAGER, block]);
    }
    e.mem.set_u32(PALETTE_ENTRIES, 0);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a6_2110, fn_00a62110()),
        entry!(0x00a6_2030, fn_00a62030()),
        entry!(0x00a6_2090, fn_00a62090()),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn copies_seventeen_words() {
        let mut e = Engine::new();
        e.map(0x011a_a000, 0x1000);
        e.map(0x011f_4000, 0x1000);
        for index in 0..0x12u32 {
            e.mem.set_u32(COPY_SOURCE + 4 * index, 0x1000 + index);
        }
        e.call(0x00a6_2110, &args![]);
        for index in 0..0x11u32 {
            assert_eq!(e.mem.u32(COPY_DESTINATION + 4 * index), 0x1000 + index);
        }
        assert_eq!(e.mem.u32(COPY_DESTINATION + 0x44), 0);
    }
}

#[cfg(test)]
mod tests_b043 {
    use super::*;

    fn fixture() -> Engine {
        let mut e = Engine::new();
        e.map(PALETTE_OBJECT & !0xfff, 0x1000);
        for callee in [LOCK_ENTER, PALETTE_CLEAR, PALETTE_ENTRY_CLEAR, DEALLOCATE] {
            e.register(callee, |_, _| Ret::default());
        }
        e
    }

    #[test]
    fn fn_00a62030_clears_seven_entries_and_counts_down() {
        let mut e = fixture();
        e.call_log = Some(vec![]);
        e.call(0x00a6_2030, &args![]);
        assert_eq!(e.call_log.take().unwrap().len(), 1);
        e.mem.set_u32(PALETTE_OBJECT, 0x7000);
        e.mem.set_u32(PALETTE_ENTRIES, 0x10_0000);
        e.mem.set_u32(PALETTE_LOCK, 0x99);
        e.mem.set_u32(PALETTE_LOCK + 4, 2);
        e.call_log = Some(vec![]);
        e.call(0x00a6_2030, &args![]);
        let log = e.call_log.take().unwrap();
        let mut expected = vec![
            (LOCK_ENTER, vec![PALETTE_LOCK, 0]),
            (PALETTE_CLEAR, vec![0x7000]),
        ];
        for index in 0..7 {
            expected.push((PALETTE_ENTRY_CLEAR, vec![0x10_0000 + index * ENTRY_SIZE]));
        }
        assert_eq!(log[1..], expected[..]);
        assert_eq!(e.mem.u32(PALETTE_LOCK + 4), 1);
        assert_eq!(e.mem.u32(PALETTE_LOCK), 0x99);
        e.call(0x00a6_2030, &args![]);
        assert_eq!(e.mem.u32(PALETTE_LOCK + 4), 0);
        assert_eq!(e.mem.u32(PALETTE_LOCK), 0);
    }

    #[test]
    fn fn_00a62090_frees_the_object_and_the_entries_last_first() {
        let mut e = fixture();
        let block = e.mem.alloc(4 + 2 * ENTRY_SIZE);
        e.mem.set_u32(block, 2);
        e.mem.set_u32(PALETTE_ENTRIES, block + 4);
        e.call_log = Some(vec![]);
        e.call(0x00a6_2090, &args![]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log[1..],
            [
                (PALETTE_ENTRY_CLEAR, vec![block + 4 + ENTRY_SIZE]),
                (PALETTE_ENTRY_CLEAR, vec![block + 4]),
                (DEALLOCATE, vec![MEMORY_MANAGER, block]),
            ]
        );
        assert_eq!(e.mem.u32(PALETTE_ENTRIES), 0);

        // With the object: fn_00a62030's work, then the object is cleared and freed.
        e.mem.set_u32(PALETTE_OBJECT, 0x7000);
        e.mem.set_u32(PALETTE_LOCK + 4, 1);
        e.call_log = Some(vec![]);
        e.call(0x00a6_2090, &args![]);
        let log = e.call_log.take().unwrap();
        let tail: Vec<_> = log.iter().skip(log.len() - 2).cloned().collect();
        assert_eq!(
            tail,
            [
                (PALETTE_CLEAR, vec![0x7000]),
                (DEALLOCATE, vec![MEMORY_MANAGER, 0x7000]),
            ]
        );
        assert_eq!(log.iter().filter(|c| c.0 == PALETTE_ENTRY_CLEAR).count(), 7);
        assert_eq!(e.mem.u32(PALETTE_OBJECT), 0);
    }
}

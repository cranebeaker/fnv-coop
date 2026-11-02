//! `Havok SDK/hkmemutil.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! All three are cdecl `(destination, source, bytes)`.

#[allow(unused_imports)]
use crate::prelude::*;

/// `memcpy` (`00ec44d0`).
const MEMCPY: u32 = 0x00ec_44d0;
/// `memset` (`00ec61c0`).
const MEMSET: u32 = 0x00ec_61c0;

// Translated from 00d2cab0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkMemUtil::memSet` (Xbox PDB), cdecl `(destination, value, bytes)`: a jump to the C
/// runtime's `memset`, whose result (the destination) it returns.
pub fn hk_mem_util_mem_set(e: &mut Engine, destination: Ptr, value: i32, bytes: u32) -> u32 {
    e.call(MEMSET, &args![destination, value, bytes]).u32()
}

// Translated from 00d2caf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map. Takes its inputs in registers: `destination` in `ECX`,
/// `source` in `EDX`, `bytes` in `EAX` (nothing on the stack). Copies `bytes / 4` whole words
/// from `source` to `destination`, last word first (so that a destination above an
/// overlapping source is preserved); nothing for fewer than 4 bytes.
pub fn fn_00d2caf0(e: &mut Engine, destination: u32, source: u32, bytes: u32) {
    if bytes < 4 {
        return;
    }
    let words = bytes >> 2;
    let delta = source.wrapping_sub(destination);
    let mut address = destination
        .wrapping_add(words.wrapping_mul(4))
        .wrapping_sub(4);
    for _ in 0..words {
        let word = e.mem.u32(address.wrapping_add(delta));
        e.mem.set_u32(address, word);
        address = address.wrapping_sub(4);
    }
}

// Translated from 00d2cb20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map. Takes its inputs in registers: `destination` in `ECX`,
/// `source` in `EDX`, `bytes` in `EAX` (nothing on the stack). Copies `bytes / 2` 16-bit
/// units from `source` to `destination`, last unit first; nothing for fewer than 2 bytes.
pub fn fn_00d2cb20(e: &mut Engine, destination: u32, source: u32, bytes: u32) {
    if bytes < 2 {
        return;
    }
    let halves = bytes >> 1;
    let delta = source.wrapping_sub(destination);
    let mut address = destination
        .wrapping_add(halves.wrapping_mul(2))
        .wrapping_sub(2);
    for _ in 0..halves {
        let half = e.mem.u16(address.wrapping_add(delta));
        e.mem.set_u16(address, half);
        address = address.wrapping_sub(2);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00d2cab0, hk_mem_util_mem_set(Ptr, i32, u32) -> u32),
        entry!(0x00d2caf0, fn_00d2caf0(u32, u32, u32)),
        entry!(0x00d2cb20, fn_00d2cb20(u32, u32, u32)),
        entry!(0x00d2caa0, hk_mem_util_mem_cpy(Ptr, Ptr, u32) -> u32),
        entry!(0x00d2cb70, fn_00d2cb70(u32, u32, u32)),
        entry!(0x00d2cc70, hk_mem_util_mem_move(u32, u32, u32)),
    ]
}

// Translated from 00d2caa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkMemUtil::memCpy` (Xbox PDB): a jump to the C runtime's `memcpy`, whose result
/// (the destination) it returns.
pub fn hk_mem_util_mem_cpy(e: &mut Engine, destination: Ptr, source: Ptr, bytes: u32) -> u32 {
    e.call(MEMCPY, &args![destination, source, bytes]).u32()
}

// Translated from 00d2cb70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map. Copies `bytes` bytes from `source` to `destination` starting
/// at the high end, so that overlapping blocks with the destination above the source are
/// preserved. It uses the widest unit that divides the byte count and both addresses: 8
/// bytes (as two words), 4, 2, else single bytes; a unit size is tried only when the count
/// is a nonzero multiple of it and both addresses are aligned to it, and then it copies
/// everything.
pub fn fn_00d2cb70(e: &mut Engine, destination: u32, source: u32, bytes: u32) {
    for unit in [8u32, 4, 2] {
        let whole = bytes & !(unit - 1);
        if whole != 0
            && bytes & (unit - 1) == 0
            && destination & (unit - 1) == 0
            && source & (unit - 1) == 0
        {
            let mut offset = whole;
            while offset != 0 {
                offset -= unit;
                match unit {
                    8 => {
                        let low = e.mem.u32(source + offset);
                        e.mem.set_u32(destination + offset, low);
                        let high = e.mem.u32(source + offset + 4);
                        e.mem.set_u32(destination + offset + 4, high);
                    }
                    4 => {
                        let word = e.mem.u32(source + offset);
                        e.mem.set_u32(destination + offset, word);
                    }
                    _ => {
                        let half = e.mem.u16(source + offset);
                        e.mem.set_u16(destination + offset, half);
                    }
                }
            }
            return;
        }
    }
    for offset in (0..bytes).rev() {
        let byte = e.mem.u8(source + offset);
        e.mem.set_u8(destination + offset, byte);
    }
}

// Translated from 00d2cc70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkMemUtil::memMove` (Xbox PDB): nothing when the addresses are equal, the backward copy
/// (`00d2cb70`) when the destination is above the source (unsigned comparison), otherwise
/// `memCpy`.
pub fn hk_mem_util_mem_move(e: &mut Engine, destination: u32, source: u32, bytes: u32) {
    if destination > source {
        fn_00d2cb70(e, destination, source, bytes);
    } else if destination < source {
        hk_mem_util_mem_cpy(e, Ptr::new(destination), Ptr::new(source), bytes);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn rig() -> (Engine, u32) {
        let mut e = Engine::new();
        e.call_log = Some(Vec::new());
        let block = e.mem.alloc(64);
        for i in 0..64 {
            e.mem.set_u8(block + i, i as u8);
        }
        (e, block)
    }

    #[test]
    fn mem_cpy_forwards_to_the_runtime_and_returns_its_result() {
        let (mut e, block) = rig();
        e.register(MEMCPY, |_, a| a[0].into_ret());
        let result = e.call(0x00d2_caa0, &args![block + 32, block, 4u32]).u32();
        assert_eq!(result, block + 32);
        assert_eq!(
            e.call_log.as_ref().unwrap().last().unwrap().1,
            vec![block + 32, block, 4]
        );
    }

    #[test]
    fn backward_copy_overlaps_correctly_at_every_unit_size() {
        // Destination above the source by `shift` bytes: the result must equal the source's
        // original contents, as a move does. The pairs reach the 8, 4, 2 and 1 byte copies.
        for (shift, bytes) in [(8u32, 16u32), (4, 12), (2, 6), (1, 5), (8, 7), (8, 0)] {
            let (mut e, block) = rig();
            let base = (block + 7) & !7;
            let original: Vec<u8> = (0..bytes).map(|i| e.mem.u8(base + i)).collect();
            e.call(0x00d2_cb70, &args![base + shift, base, bytes]);
            let copied: Vec<u8> = (0..bytes).map(|i| e.mem.u8(base + shift + i)).collect();
            assert_eq!(copied, original, "shift {shift} bytes {bytes}");
        }
    }

    #[test]
    fn mem_move_picks_the_direction() {
        let (mut e, block) = rig();
        e.register(MEMCPY, |e, a| {
            for i in 0..a[2] {
                let byte = e.mem.u8(a[1] + i);
                e.mem.set_u8(a[0] + i, byte);
            }
            a[0].into_ret()
        });
        // Destination below the source: the runtime copy.
        e.call(0x00d2_cc70, &args![block, block + 8, 8u32]);
        assert_eq!(e.mem.u8(block), 8);
        assert!(e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .any(|(a, _)| *a == MEMCPY));
        // Equal addresses: nothing.
        let before = e.call_log.as_ref().unwrap().len();
        e.call(0x00d2_cc70, &args![block, block, 8u32]);
        assert_eq!(e.call_log.as_ref().unwrap().len(), before + 1);
        // Destination above the source: the backward copy, no runtime call.
        let base = (block + 7) & !7;
        let before = e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == MEMCPY)
            .count();
        e.call(0x00d2_cc70, &args![base + 8, base, 16u32]);
        let after = e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == MEMCPY)
            .count();
        assert_eq!(before, after);
        assert_eq!(e.mem.u8(base + 8 + 15), (base - block + 15) as u8);
    }

    #[test]
    fn mem_set_forwards_to_the_runtime_and_returns_its_result() {
        let (mut e, block) = rig();
        e.register(MEMSET, |_, a| a[0].into_ret());
        let result = e.call(0x00d2_cab0, &args![block, -1i32, 24u32]).u32();
        assert_eq!(result, block);
        assert_eq!(
            e.call_log.as_ref().unwrap().last().unwrap().1,
            vec![block, 0xffff_ffff, 24]
        );
    }

    #[test]
    fn word_and_half_word_copies_go_backwards_and_keep_the_remainder() {
        // (the unit in bytes, the function, shift of the destination above the source, bytes)
        for (unit, address, shift, bytes) in [
            (4u32, 0x00d2_caf0u32, 4u32, 12u32),
            (4, 0x00d2_caf0, 8, 7),
            (4, 0x00d2_caf0, 4, 3),
            (4, 0x00d2_caf0, 4, 0),
            (2, 0x00d2_cb20, 2, 6),
            (2, 0x00d2_cb20, 4, 5),
            (2, 0x00d2_cb20, 2, 1),
        ] {
            let (mut e, block) = rig();
            let base = (block + 7) & !7;
            let original: Vec<u8> = (0..64).map(|i| e.mem.u8(base + i)).collect();
            e.call(address, &args![base + shift, base, bytes]);
            let copied = bytes - bytes % unit;
            // the whole units are the original source bytes, whatever the overlap
            for i in 0..copied {
                assert_eq!(
                    e.mem.u8(base + shift + i),
                    original[i as usize],
                    "unit {unit} shift {shift} bytes {bytes} byte {i}"
                );
            }
            // the bytes past the whole units and before the destination are untouched
            for i in 0..shift.min(64) {
                assert_eq!(e.mem.u8(base + i), original[i as usize]);
            }
            for i in (shift + copied)..32 {
                assert_eq!(e.mem.u8(base + i), original[i as usize]);
            }
        }
    }
}

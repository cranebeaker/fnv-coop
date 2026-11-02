//! `Havok SDK/run_00e31510` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00e31510 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: steps over a value of `size` bytes in a buffer (the address is
/// the word stored at `cursor`) and byte-swaps it in place when the size is 2 (the two bytes
/// exchanged) or 4 (the bytes 0 and 3, and 1 and 2, exchanged); any other size is only skipped.
/// The word at `cursor` moves on by `size`. The function takes two stack words and pops none
/// (`RET`) and returns nothing.
pub fn fn_00e31510(e: &mut Engine, size: u32, cursor: Ptr) {
    match size {
        2 => {
            let at = e.mem.u32(cursor.addr());
            swap_bytes_at(e, at, at.wrapping_add(1));
        }
        4 => {
            let at = e.mem.u32(cursor.addr());
            swap_bytes_at(e, at, at.wrapping_add(3));
            let at = e.mem.u32(cursor.addr());
            swap_bytes_at(e, at.wrapping_add(1), at.wrapping_add(2));
        }
        _ => {}
    }
    let at = e.mem.u32(cursor.addr());
    e.mem.set_u32(cursor.addr(), at.wrapping_add(size));
}

/// Exchanges the bytes at the two addresses.
fn swap_bytes_at(e: &mut Engine, first: u32, second: u32) {
    let a = e.mem.u8(first);
    let b = e.mem.u8(second);
    e.mem.set_u8(first, b);
    e.mem.set_u8(second, a);
}

// Translated from 00e31570 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: reads a length-prefixed block header. When the high nibble of the
/// low byte of `flags` is clear the result is `0xffffffff` and nothing else happens. Otherwise a
/// 16-bit value is stepped over and swapped (`00e31510` with size 2); the result is that value
/// (read back at `cursor - 2`), and the word at `cursor` moves on past the byte it now points to
/// and that many more bytes plus 3. The function takes two stack words and pops none (`RET`).
pub fn fn_00e31570(e: &mut Engine, flags: u8, cursor: Ptr) -> u32 {
    if flags & 0xf0 == 0 {
        return 0xffff_ffff;
    }
    fn_00e31510(e, 2, cursor);
    let at = e.mem.u32(cursor.addr());
    let skip = u32::from(e.mem.u8(at));
    let length = u32::from(e.mem.u16(at.wrapping_sub(2)));
    e.mem.set_u32(
        cursor.addr(),
        at.wrapping_add(skip).wrapping_add(length).wrapping_add(3),
    );
    length
}

// Translated from 00e315a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: steps over the dword fields that the bits of `flags` select.
/// Nothing happens when the low byte of `flags` is zero. The offset at `cursor` is rounded up
/// to 4; then for each of the bits 0, 1 and 2: when the bit four places up (4, 5, 6) is set the
/// field is two dwords (`00e31510` with size 4, twice) and counted; otherwise when the bit
/// itself is set, one dword. When any field was counted and `count` is not negative, `count + 1`
/// passes follow, each stepping over every counted field once with `00e31510`, size 2 when
/// `wide` is not zero and 1 otherwise. The function takes four stack words and pops none
/// (`RET`) and returns nothing.
pub fn fn_00e315a0(e: &mut Engine, flags: u8, count: i32, wide: u32, cursor: Ptr) {
    if flags == 0 {
        return;
    }
    let at = e.mem.u32(cursor.addr());
    e.mem
        .set_u32(cursor.addr(), at.wrapping_add(3) & 0xffff_fffc);
    let mut counted: i32 = 0;
    let mut bit: u32 = 1;
    for _ in 0..3 {
        if u32::from(flags) & bit.rotate_left(4) != 0 {
            fn_00e31510(e, 4, cursor);
            fn_00e31510(e, 4, cursor);
            counted += 1;
        } else if u32::from(flags) & bit != 0 {
            fn_00e31510(e, 4, cursor);
        }
        bit = bit.rotate_left(1);
    }
    if counted == 0 {
        return;
    }
    let size = u32::from(wide != 0) + 1;
    if count < 0 {
        return;
    }
    let mut passes = count.wrapping_add(1);
    loop {
        // The exe re-tests the counter here; it is positive at this point, so the branch that
        // skips the inner loop is unreachable.
        if counted > 0 {
            for _ in 0..counted {
                fn_00e31510(e, size, cursor);
            }
        }
        passes = passes.wrapping_sub(1);
        if passes == 0 {
            break;
        }
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00e3_1510, fn_00e31510(u32, Ptr)),
        entry!(0x00e3_1570, fn_00e31570(u8, Ptr) -> u32),
        entry!(0x00e3_15a0, fn_00e315a0(u8, i32, u32, Ptr)),
    ]
}

#[cfg(test)]
mod tests_a05 {
    use crate::prelude::*;

    /// A buffer of the bytes 0x10, 0x11, ... and a cursor word pointing at offset `at`.
    fn buffer(e: &mut Engine, at: u32) -> (u32, u32) {
        let data = e.mem.alloc(0x40);
        for index in 0..0x40u32 {
            e.mem.set_u8(data + index, 0x10 + index as u8);
        }
        let cursor = e.mem.alloc(4);
        e.mem.set_u32(cursor, data + at);
        (data, cursor)
    }

    fn bytes(e: &Engine, data: u32, count: u32) -> Vec<u8> {
        (0..count).map(|index| e.mem.u8(data + index)).collect()
    }

    /// Sizes 1 and 3 only move the cursor, 2 and 4 swap the bytes first.
    #[test]
    fn fn_00e31510_swaps_two_and_four_byte_values() {
        let mut e = Engine::new();
        let (data, cursor) = buffer(&mut e, 0);
        e.call(0x00e3_1510, &args![1u32, cursor]);
        assert_eq!(e.mem.u32(cursor), data + 1);
        e.call(0x00e3_1510, &args![3u32, cursor]);
        assert_eq!(e.mem.u32(cursor), data + 4);
        assert_eq!(bytes(&e, data, 4), vec![0x10, 0x11, 0x12, 0x13]);
        e.call(0x00e3_1510, &args![2u32, cursor]);
        assert_eq!(e.mem.u32(cursor), data + 6);
        assert_eq!(bytes(&e, data + 4, 2), vec![0x15, 0x14]);
        e.call(0x00e3_1510, &args![4u32, cursor]);
        assert_eq!(e.mem.u32(cursor), data + 10);
        assert_eq!(bytes(&e, data + 6, 4), vec![0x19, 0x18, 0x17, 0x16]);
        assert_eq!(bytes(&e, data + 10, 1), vec![0x1a]);
    }

    /// Without a high nibble the answer is -1; otherwise the swapped 16-bit length comes back
    /// and the cursor skips the length-and-a-byte block.
    #[test]
    fn fn_00e31570_reads_the_length() {
        let mut e = Engine::new();
        let (data, cursor) = buffer(&mut e, 0);
        assert_eq!(
            e.call(0x00e3_1570, &args![0x0fu32, cursor]).u32(),
            0xffff_ffff
        );
        assert_eq!(e.mem.u32(cursor), data);
        e.mem.set_u8(data, 0x01);
        e.mem.set_u8(data + 1, 0x02);
        e.mem.set_u8(data + 2, 5);
        // The swapped 16-bit value is 0x0102 (bytes 02 01 after the swap): 0x0102 = 258.
        let length = e.call(0x00e3_1570, &args![0x10u32, cursor]).u32();
        assert_eq!(length, 0x0102);
        assert_eq!(e.mem.u32(cursor), data + 2 + 5 + 0x0102 + 3);
    }

    /// Flags 0: nothing. Flags 0x01 and 0x20: one dword for bit 0, two (counted) for bit 1.
    #[test]
    fn fn_00e315a0_steps_over_the_selected_fields() {
        let mut e = Engine::new();
        let (data, cursor) = buffer(&mut e, 1);
        e.call(0x00e3_15a0, &args![0u32, 0i32, 0u32, cursor]);
        assert_eq!(e.mem.u32(cursor), data + 1);
        // Align to 4, bit 0 (one dword), bit 1 via bit 5 (two dwords, counted), no passes
        // for a negative count.
        e.call(0x00e3_15a0, &args![0x21u32, -1i32, 0u32, cursor]);
        assert_eq!(e.mem.u32(cursor), data + 4 + 4 + 8);
    }

    /// With a counted field, `count + 1` passes follow, each stepping over the counted field
    /// with 1 byte or, when `wide`, 2 bytes (swapped).
    #[test]
    fn fn_00e315a0_runs_count_plus_one_passes() {
        let mut e = Engine::new();
        let (data, cursor) = buffer(&mut e, 0);
        // Bit 4 set: one counted field of two dwords; two passes of one byte each.
        e.call(0x00e3_15a0, &args![0x10u32, 1i32, 0u32, cursor]);
        assert_eq!(e.mem.u32(cursor), data + 8 + 2);
        let (data, cursor) = buffer(&mut e, 0);
        // The same with wide: two passes of two bytes, each swapped in place.
        e.call(0x00e3_15a0, &args![0x10u32, 1i32, 1u32, cursor]);
        assert_eq!(e.mem.u32(cursor), data + 8 + 4);
        assert_eq!(bytes(&e, data + 8, 4), vec![0x19, 0x18, 0x1b, 0x1a]);
        // Two counted fields count twice per pass.
        let (data, cursor) = buffer(&mut e, 0);
        e.call(0x00e3_15a0, &args![0x30u32, 0i32, 0u32, cursor]);
        assert_eq!(e.mem.u32(cursor), data + 16 + 2);
    }
}

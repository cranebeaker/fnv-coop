//! `Havok SDK/hkpsavecontactpointsendianutil.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The functions swap the byte order of the saved contact point records in place. All of them are
//! cdecl and none has a /GS cookie or an exception frame.

#[allow(unused_imports)]
use crate::prelude::*;

/// The 16-byte swapper the first two functions tail-jump to (cdecl, one word: the start of four
/// dwords whose bytes it reverses).
const SWAP_FOUR_DWORDS: u32 = 0x00e0_5de0;
/// `hkpSaveContactPointsUtil::getSerializedAgentType` (`00dcd4d0`, cdecl, one word).
const GET_SERIALIZED_AGENT_TYPE: u32 = 0x00dc_d4d0;

/// Swaps the bytes at `base + first` and `base + second`.
fn swap_bytes(e: &mut Engine, base: u32, first: i32, second: i32) {
    let first = first as u32;
    let second = second as u32;
    let held = e.mem.u8(base.wrapping_add(first));
    let other = e.mem.u8(base.wrapping_add(second));
    e.mem.set_u8(base.wrapping_add(first), other);
    e.mem.set_u8(base.wrapping_add(second), held);
}

// Translated from 00e05e20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (cdecl, one word): reverses the 4 bytes at `+0x18` and then, as a tail jump, the four
/// dwords at `+0x20`.
pub fn fn_00e05e20(e: &mut Engine, block: Ptr) {
    let block = block.addr();
    swap_bytes(e, block, 0x18, 0x1b);
    swap_bytes(e, block, 0x19, 0x1a);
    e.call(SWAP_FOUR_DWORDS, &args![block.wrapping_add(0x20)]);
}

// Translated from 00e05e50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (cdecl, one word): reverses the three dwords at `+4`, `+8` and `+0xC` and then, as a
/// tail jump, the four dwords at `+0x10`.
pub fn fn_00e05e50(e: &mut Engine, block: Ptr) {
    let block = block.addr();
    for dword in [4, 8, 0xc] {
        swap_bytes(e, block, dword, dword + 3);
        swap_bytes(e, block, dword + 1, dword + 2);
    }
    e.call(SWAP_FOUR_DWORDS, &args![block.wrapping_add(0x10)]);
}

// Translated from 00e05eb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (cdecl, one word): reverses the two dwords at `+4` and `+8`.
pub fn fn_00e05eb0(e: &mut Engine, block: Ptr) {
    let block = block.addr();
    for dword in [4, 8] {
        swap_bytes(e, block, dword, dword + 3);
        swap_bytes(e, block, dword + 1, dword + 2);
    }
}

// Translated from 00e05ef0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (cdecl, one word): swaps the two bytes of each 16-bit word at `+2 + 4 * i` for the
/// `byte [+0x21]` words (the count is read again after every word), reverses the four dwords at
/// `+0x30` and then the dword at `+0x24`.
pub fn fn_00e05ef0(e: &mut Engine, block: Ptr) {
    let block = block.addr();
    if e.mem.u8(block + 0x21) != 0 {
        let mut index: i32 = 0;
        let mut at = block + 2;
        loop {
            swap_bytes(e, at, 0, 1);
            index += 1;
            at = at.wrapping_add(4);
            if index >= i32::from(e.mem.u8(block + 0x21)) {
                break;
            }
        }
    }
    let mut at = block + 0x33;
    for _ in 0..4 {
        swap_bytes(e, at, -3, 0);
        swap_bytes(e, at, -2, -1);
        at = at.wrapping_add(4);
    }
    swap_bytes(e, block, 0x24, 0x27);
    swap_bytes(e, block, 0x25, 0x26);
}

// Translated from 00e05f60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (cdecl, one word): swaps the two bytes of the three 16-bit words at `+0`, `+2` and
/// `+4`, reverses the three dwords at `+8`, `+0xC` and `+0x10`, then the dwords at `+0x14` and
/// `+0x18`.
pub fn fn_00e05f60(e: &mut Engine, block: Ptr) {
    let block = block.addr();
    let mut at = block + 0xb;
    for index in 0..3 {
        swap_bytes(e, block, index * 2, index * 2 + 1);
        swap_bytes(e, at, -3, 0);
        swap_bytes(e, at, -2, -1);
        at = at.wrapping_add(4);
    }
    for dword in [0x14, 0x18] {
        swap_bytes(e, block, dword, dword + 3);
        swap_bytes(e, block, dword + 1, dword + 2);
    }
}

// Translated from 00e05fe0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (cdecl, one word): swaps the two bytes of each of the four 16-bit words at `+0`.
pub fn fn_00e05fe0(e: &mut Engine, block: Ptr) {
    let block = block.addr();
    for index in 0..4 {
        swap_bytes(e, block, index * 2, index * 2 + 1);
    }
}

// Translated from 00e06010 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (cdecl, one word): the block starts with three counts (`byte [+0]`, `byte [+1]`,
/// `byte [+2]`). Swaps the two bytes of the 16-bit word at `+6 + 8 * i` for each of the
/// `byte [+2]` records (the count is read again after every record), then the two bytes of each
/// of the `byte [+0] + byte [+1]` words that follow the records, from `+4 + 8 * byte [+2]`.
pub fn fn_00e06010(e: &mut Engine, block: Ptr) {
    let block = block.addr();
    if e.mem.u8(block + 2) != 0 {
        let mut index: i32 = 0;
        let mut at = block + 6;
        loop {
            swap_bytes(e, at, 0, 1);
            index += 1;
            at = at.wrapping_add(8);
            if index >= i32::from(e.mem.u8(block + 2)) {
                break;
            }
        }
    }
    let words = i32::from(e.mem.u8(block + 1)) + i32::from(e.mem.u8(block));
    let start = block + u32::from(e.mem.u8(block + 2)) * 8 + 4;
    if words > 0 {
        let mut index: i32 = 0;
        loop {
            swap_bytes(e, start, index * 2, index * 2 + 1);
            index += 1;
            if index >= words {
                break;
            }
        }
    }
}

// Translated from 00e06070 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpSaveContactPointsEndianUtil::swapEndianTypeInEntry` (map name), cdecl with six words
/// `(context, entry, track, larger_layout, size, swapped_track)`: writes 0 to the byte at
/// `swapped_track` and dispatches on the entry's type byte (`byte [entry]`).
///
/// * Types 0 and 1: the size is `0x10` and nothing else happens.
/// * Types 2, 3, 10, 11: with the low byte of `larger_layout` zero, `00e05eb0` is applied to the
///   entry and the payload starts at `+0x10`; otherwise at `+0x20` with no swap.
/// * Types 4, 5, 6, 12, 13, 14: with zero `00e05e50` is applied and the payload starts at
///   `+0x20`; otherwise `00e05e20` and `+0x30`.
/// * Types 7, 8, 9 and everything above 14: nothing is done and the size is not written.
///
/// For the first two groups the size is `byte [entry + 3]` and the payload is swapped according to
/// `getSerializedAgentType` of the word at `[context] + 0x16dc + 0x50 * byte [entry + 1]`: type 1
/// `00e05ef0`, type 2 `00e05f60`, types 3 and 4 `00e05fe0` then `00e06010` at `+0xC`, type 5
/// the same when bit 5 of `byte [payload + 0xB]` is set and otherwise `swapEndianTypeIn1nTrack`
/// `(context, track)` with the byte at `swapped_track` set to 1, types 6 to 9 the latter, other
/// types nothing. Only the low byte of `larger_layout` is looked at.
pub fn hkp_save_contact_points_endian_util_swap_endian_type_in_entry(
    e: &mut Engine,
    context: Ptr,
    entry: Ptr,
    track: Ptr,
    larger_layout: u8,
    size: Ptr,
    swapped_track: Ptr,
) {
    let entry = entry.addr();
    e.mem.set_u8(swapped_track.addr(), 0);
    let payload = match e.mem.u8(entry) {
        0 | 1 => {
            e.mem.set_u32(size.addr(), 0x10);
            return;
        }
        2 | 3 | 10 | 11 => {
            if larger_layout == 0 {
                e.call(0x00e0_5eb0, &args![entry]);
                entry.wrapping_add(0x10)
            } else {
                entry.wrapping_add(0x20)
            }
        }
        4..=6 | 12..=14 => {
            if larger_layout != 0 {
                e.call(0x00e0_5e20, &args![entry]);
                entry.wrapping_add(0x30)
            } else {
                e.call(0x00e0_5e50, &args![entry]);
                entry.wrapping_add(0x20)
            }
        }
        _ => return,
    };
    let agent = u32::from(e.mem.u8(entry + 1)).wrapping_mul(0x50);
    let agent = e.mem.u32(
        e.mem
            .u32(context.addr())
            .wrapping_add(agent)
            .wrapping_add(0x16dc),
    );
    let kind = e.call(GET_SERIALIZED_AGENT_TYPE, &args![agent]).u32();
    e.mem.set_u32(size.addr(), u32::from(e.mem.u8(entry + 3)));
    match kind.wrapping_sub(1) {
        0 => {
            e.call(0x00e0_5ef0, &args![payload]);
        }
        1 => {
            e.call(0x00e0_5f60, &args![payload]);
        }
        2 | 3 => {
            e.call(0x00e0_5fe0, &args![payload]);
            e.call(0x00e0_6010, &args![payload.wrapping_add(0xc)]);
        }
        4 => {
            if e.mem.u8(payload + 0xb) & 0x20 != 0 {
                e.call(0x00e0_5fe0, &args![payload]);
                e.call(0x00e0_6010, &args![payload.wrapping_add(0xc)]);
            } else {
                e.call(0x00e0_61c0, &args![context, track]);
                e.mem.set_u8(swapped_track.addr(), 1);
            }
        }
        5..=8 => {
            e.call(0x00e0_61c0, &args![context, track]);
            e.mem.set_u8(swapped_track.addr(), 1);
        }
        _ => {}
    }
}

// Translated from 00e061c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpSaveContactPointsEndianUtil::swapEndianTypeIn1nTrack` (map name), cdecl with two words
/// `(context, track)`. The track keeps `+0` an array of items, `+4` their count, `+0xC` an array
/// of block pointers and `+0x10` its count. Each item has its size in its first word, and is
/// followed after `0x10` bytes by entries. Every entry goes through `swapEndianTypeInEntry` with
/// the current block (the first block to start with; the next block when the current one is for
/// another item or ends before the entry's offset in the item, or none when there are no more),
/// a zero layout flag, and the next entry is found by adding the size it wrote.
///
/// An entry that does not write its size (types 7 to 9, above 14) or writes zero keeps the exe in
/// an endless loop when the block does not change; that loop is translated as it is.
pub fn hkp_save_contact_points_endian_util_swap_endian_type_in_1n_track(
    e: &mut Engine,
    context: Ptr,
    track: Ptr,
) {
    let track = track.addr();
    let (mut cursor, mut block): (i32, u32) = if e.mem.u32(track + 0x10) != 0 {
        (0, e.mem.u32(e.mem.u32(track + 0xc)))
    } else {
        (-1, 0)
    };
    if e.mem.i32(track + 4) <= 0 {
        return;
    }
    // The exe's frame (`SUB ESP,0x18`, four pushes) taken with the arguments: the entry size
    // at +0x1c (00e0626b) and the second out word in the `track` argument's own slot, +0x30
    // past the return address at +0x28 (00e06262).
    e.with_stack(0x34, |e, frame| {
        let frame = frame.addr();
        e.mem.set_u32(frame + 0x2c, context.addr());
        e.mem.set_u32(frame + 0x30, track);
        let (size_at, second_at) = (frame + 0x1c, frame + 0x30);
        let mut item_index: i32 = 0;
        loop {
            let item = e.mem.u32(
                e.mem
                    .u32(track)
                    .wrapping_add((item_index as u32).wrapping_mul(4)),
            );
            let end = e.mem.u32(item).wrapping_add(item).wrapping_add(0x10);
            let mut entry = item.wrapping_add(0x10);
            if entry < end {
                let mut next = item_index.wrapping_add(1);
                loop {
                    e.mem.set_u32(size_at, 0);
                    if block != 0 {
                        let offset = entry.wrapping_sub(item).wrapping_sub(0x10) as i32;
                        if e.mem.i32(block + 0x18) != item_index || e.mem.i32(block + 0x1c) < offset
                        {
                            if next < e.mem.i32(track + 0x10) {
                                let list = e.mem.u32(track + 0xc);
                                block = e.mem.u32(
                                    list.wrapping_add((cursor as u32).wrapping_mul(4))
                                        .wrapping_add(4),
                                );
                                cursor += 1;
                                next += 1;
                            } else {
                                block = 0;
                            }
                        }
                    }
                    e.call(
                        0x00e0_6070,
                        &args![context, entry, block, 0u32, size_at, second_at],
                    );
                    entry = entry.wrapping_add(e.mem.u32(size_at));
                    if entry >= end {
                        break;
                    }
                }
            }
            item_index = item_index.wrapping_add(1);
            if item_index >= e.mem.i32(track + 4) {
                break;
            }
        }
    });
}

// Translated from 00e062a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpSaveContactPointsEndianUtil::swapEndianTypeInCollisionEntry` (map name), cdecl with two
/// words `(context, entry)`: runs `swapEndianTypeInEntry` on the entry at `entry + 0x74` with the
/// track at `entry + 0x114` and layout flag 1, then divides the word at `entry + 0x54` by
/// `byte [entry + 0x2a]` (the stride; a zero stride is the processor's divide error and panics
/// here), stores `0x103` at `entry + 0x12c`, and for that many records of the stride from the
/// pointer at `entry + 0x50` reverses the dwords at `+0x10`, `+0`, `+4` and `+8`, then
/// `2 * byte [entry + 0x28]` dwords from `+0x14`.
pub fn hkp_save_contact_points_endian_util_swap_endian_type_in_collision_entry(
    e: &mut Engine,
    context: Ptr,
    entry: Ptr,
) {
    let entry = entry.addr();
    // The exe's frame (`PUSH ECX`, two pushes) taken with the arguments: the first out word in
    // the pushed slot at +0x8 (00e062ac) and the second in the `entry` argument's own slot,
    // +0x14 past the return address at +0xc (00e062a7).
    e.with_stack(0x18, |e, frame| {
        let frame = frame.addr();
        e.mem.set_u32(frame + 0x10, context.addr());
        e.mem.set_u32(frame + 0x14, entry);
        // 00e062b1: the flag byte 1 is built in the first out word.
        e.mem.set_u8(frame + 8, 1);
        e.call(
            0x00e0_6070,
            &args![
                context,
                entry.wrapping_add(0x74),
                entry.wrapping_add(0x114),
                1u32,
                frame + 8,
                frame + 0x14
            ],
        );
    });
    let stride = u32::from(e.mem.u8(entry + 0x2a));
    assert!(
        stride != 0,
        "00e062a0: the stride at +0x2a is zero (divide error)"
    );
    let mut remaining = e.mem.i32(entry + 0x54) / stride as i32;
    e.mem.set_u32(entry + 0x12c, 0x103);
    if remaining <= 0 {
        return;
    }
    let mut offset: u32 = 0;
    loop {
        let record = e.mem.u32(entry + 0x50).wrapping_add(offset);
        for dword in [0x10, 0, 4, 8] {
            swap_bytes(e, record, dword, dword + 3);
            swap_bytes(e, record, dword + 1, dword + 2);
        }
        if e.mem.u8(entry + 0x28) != 0 {
            let mut index: i32 = 0;
            let mut at = record.wrapping_add(0x17);
            loop {
                swap_bytes(e, at, -3, 0);
                swap_bytes(e, at, -2, -1);
                index += 1;
                at = at.wrapping_add(4);
                if index >= i32::from(e.mem.u8(entry + 0x28)) * 2 {
                    break;
                }
            }
        }
        offset = offset.wrapping_add(stride);
        remaining -= 1;
        if remaining == 0 {
            break;
        }
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00e0_5e20, fn_00e05e20(Ptr)),
        entry!(0x00e0_5e50, fn_00e05e50(Ptr)),
        entry!(0x00e0_5eb0, fn_00e05eb0(Ptr)),
        entry!(0x00e0_5ef0, fn_00e05ef0(Ptr)),
        entry!(0x00e0_5f60, fn_00e05f60(Ptr)),
        entry!(0x00e0_5fe0, fn_00e05fe0(Ptr)),
        entry!(0x00e0_6010, fn_00e06010(Ptr)),
        entry!(
            0x00e0_6070,
            hkp_save_contact_points_endian_util_swap_endian_type_in_entry(
                Ptr, Ptr, Ptr, u8, Ptr, Ptr
            )
        ),
        entry!(
            0x00e0_61c0,
            hkp_save_contact_points_endian_util_swap_endian_type_in_1n_track(Ptr, Ptr)
        ),
        entry!(
            0x00e0_62a0,
            hkp_save_contact_points_endian_util_swap_endian_type_in_collision_entry(Ptr, Ptr)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::havok_sdk::hkpbvagent::test_support::{
        calls, engine, log_double, new_log, Log,
    };
    use std::rc::Rc;

    /// A block of `len` bytes holding 1, 2, 3, ... (and some guard bytes after it).
    fn numbered(e: &mut Engine, len: u32) -> u32 {
        let block = e.mem.alloc(len + 0x10);
        for index in 0..len {
            e.mem.set_u8(block + index, (index + 1) as u8);
        }
        block
    }

    fn bytes(e: &Engine, at: u32, len: u32) -> Vec<u8> {
        (0..len).map(|index| e.mem.u8(at + index)).collect()
    }

    /// `data` with each `(start, length)` group reversed.
    fn reversed(data: &[u8], groups: &[(usize, usize)]) -> Vec<u8> {
        let mut data = data.to_vec();
        for (start, length) in groups {
            data[*start..*start + *length].reverse();
        }
        data
    }

    #[test]
    fn the_first_swapper_hands_the_next_dwords_on_00e05e20() {
        let mut e = engine();
        let log = new_log();
        log_double(&mut e, &log, SWAP_FOUR_DWORDS, 0);
        let block = numbered(&mut e, 0x30);
        let before = bytes(&e, block, 0x30);
        e.call(0x00e0_5e20, &args![block]);
        assert_eq!(bytes(&e, block, 0x30), reversed(&before, &[(0x18, 4)]));
        assert_eq!(calls(&log, SWAP_FOUR_DWORDS), vec![vec![block + 0x20]]);
    }

    #[test]
    fn the_second_swapper_swaps_three_dwords_and_hands_the_next_on_00e05e50() {
        let mut e = engine();
        let log = new_log();
        log_double(&mut e, &log, SWAP_FOUR_DWORDS, 0);
        let block = numbered(&mut e, 0x20);
        let before = bytes(&e, block, 0x20);
        e.call(0x00e0_5e50, &args![block]);
        assert_eq!(
            bytes(&e, block, 0x20),
            reversed(&before, &[(4, 4), (8, 4), (0xc, 4)])
        );
        assert_eq!(calls(&log, SWAP_FOUR_DWORDS), vec![vec![block + 0x10]]);
    }

    #[test]
    fn the_third_swapper_swaps_two_dwords_00e05eb0() {
        let mut e = engine();
        let block = numbered(&mut e, 0x10);
        let before = bytes(&e, block, 0x10);
        e.call(0x00e0_5eb0, &args![block]);
        assert_eq!(bytes(&e, block, 0x10), reversed(&before, &[(4, 4), (8, 4)]));
    }

    #[test]
    fn the_word_and_dword_swapper_follows_the_word_count_00e05ef0() {
        for count in [0u8, 1, 3, 8] {
            let mut e = engine();
            let block = numbered(&mut e, 0x40);
            e.mem.set_u8(block + 0x21, count);
            let before = bytes(&e, block, 0x40);
            e.call(0x00e0_5ef0, &args![block]);
            let mut groups: Vec<(usize, usize)> =
                (0..usize::from(count)).map(|i| (2 + 4 * i, 2)).collect();
            groups.extend([(0x30, 4), (0x34, 4), (0x38, 4), (0x3c, 4), (0x24, 4)]);
            assert_eq!(bytes(&e, block, 0x40), reversed(&before, &groups));
            assert_eq!(e.mem.u8(block + 0x21), count);
        }
    }

    #[test]
    fn the_three_words_and_five_dwords_are_swapped_00e05f60() {
        let mut e = engine();
        let block = numbered(&mut e, 0x20);
        let before = bytes(&e, block, 0x20);
        e.call(0x00e0_5f60, &args![block]);
        assert_eq!(
            bytes(&e, block, 0x20),
            reversed(
                &before,
                &[
                    (0, 2),
                    (2, 2),
                    (4, 2),
                    (8, 4),
                    (0xc, 4),
                    (0x10, 4),
                    (0x14, 4),
                    (0x18, 4)
                ]
            )
        );
    }

    #[test]
    fn four_words_are_swapped_00e05fe0() {
        let mut e = engine();
        let block = numbered(&mut e, 0x10);
        let before = bytes(&e, block, 0x10);
        e.call(0x00e0_5fe0, &args![block]);
        assert_eq!(
            bytes(&e, block, 0x10),
            reversed(&before, &[(0, 2), (2, 2), (4, 2), (6, 2)])
        );
    }

    #[test]
    fn the_counted_block_swaps_records_then_trailing_words_00e06010() {
        // (first count, second count, record count) -> the groups swapped.
        let cases: [(u8, u8, u8, Vec<(usize, usize)>); 3] = [
            (
                1,
                2,
                2,
                vec![(6, 2), (0xe, 2), (0x14, 2), (0x16, 2), (0x18, 2)],
            ),
            (1, 2, 0, vec![(4, 2), (6, 2), (8, 2)]),
            (0, 0, 0, vec![]),
        ];
        for (first, second, records, groups) in cases {
            let mut e = engine();
            let block = numbered(&mut e, 0x40);
            e.mem.set_u8(block, first);
            e.mem.set_u8(block + 1, second);
            e.mem.set_u8(block + 2, records);
            let before = bytes(&e, block, 0x40);
            e.call(0x00e0_6010, &args![block]);
            assert_eq!(bytes(&e, block, 0x40), reversed(&before, &groups));
        }
    }

    const LEAVES: [u32; 7] = [
        0x00e0_5eb0,
        0x00e0_5e50,
        0x00e0_5e20,
        0x00e0_5ef0,
        0x00e0_5f60,
        0x00e0_5fe0,
        0x00e0_6010,
    ];

    /// Doubles for everything `swapEndianTypeInEntry` calls. The agent type function answers the
    /// word it is given.
    fn entry_doubles(e: &mut Engine, log: &Log) {
        for address in LEAVES {
            log_double(e, log, address, 0);
        }
        log_double(e, log, 0x00e0_61c0, 0);
        let logged = Rc::clone(log);
        e.register_double(GET_SERIALIZED_AGENT_TYPE, move |_, a| {
            logged
                .borrow_mut()
                .push((GET_SERIALIZED_AGENT_TYPE, a.to_vec()));
            Ret {
                eax: a[0],
                ..Ret::default()
            }
        });
    }

    /// What one `swapEndianTypeInEntry` call did.
    struct EntryRun {
        log: Log,
        entry: u32,
        context: u32,
        track: u32,
        size: u32,
        swapped_track: u8,
    }

    /// Runs the entry function on an entry of type `kind` whose agent index is 3 and whose
    /// agent record holds `agent`; the byte at `payload_offset + 0xB` (the payload's
    /// byte `+0xB`) holds `flags`.
    fn run_entry(kind: u8, layout: u32, agent: u32, payload_offset: u32, flags: u8) -> EntryRun {
        let mut e = engine();
        let log = new_log();
        entry_doubles(&mut e, &log);
        let area = e.mem.alloc(0x2000);
        e.mem.set_u32(area + 0x16dc + 3 * 0x50, agent);
        let context = e.mem.alloc(8);
        e.mem.set_u32(context, area);
        let entry = e.mem.alloc(0x60);
        e.mem.write(entry, &[0; 0x60]);
        e.mem.set_u8(entry, kind);
        e.mem.set_u8(entry + 1, 3);
        e.mem.set_u8(entry + 3, 0x24);
        e.mem.set_u8(entry + payload_offset + 0xb, flags);
        let track = e.mem.alloc(0x20);
        let out = e.mem.alloc(8);
        e.mem.set_u32(out, 0xcdcd_cdcd);
        e.mem.set_u8(out + 4, 0xee);
        e.call(
            0x00e0_6070,
            &args![context, entry, track, layout, out, out + 4],
        );
        EntryRun {
            log,
            entry,
            context,
            track,
            size: e.mem.u32(out),
            swapped_track: e.mem.u8(out + 4),
        }
    }

    fn logged(run: &EntryRun) -> Vec<(u32, Vec<u32>)> {
        run.log.borrow().clone()
    }

    #[test]
    fn the_smallest_entry_types_only_report_their_size_00e06070() {
        for kind in [0u8, 1] {
            let run = run_entry(kind, 0, 1, 0, 0);
            assert_eq!(run.size, 0x10);
            assert_eq!(run.swapped_track, 0);
            assert!(logged(&run).is_empty());
        }
    }

    #[test]
    fn unhandled_entry_types_do_nothing_00e06070() {
        for kind in [7u8, 8, 9, 15, 0x80, 0xff] {
            let run = run_entry(kind, 0, 1, 0, 0);
            assert_eq!(run.size, 0xcdcd_cdcd, "type {kind}");
            assert_eq!(run.swapped_track, 0);
            assert!(logged(&run).is_empty());
        }
    }

    #[test]
    fn the_short_group_takes_its_head_and_payload_offset_from_the_layout_00e06070() {
        for kind in [2u8, 3, 10, 11] {
            let run = run_entry(kind, 0, 1, 0x10, 0);
            let payload = run.entry + 0x10;
            assert_eq!(
                logged(&run),
                vec![
                    (0x00e0_5eb0, vec![run.entry]),
                    (GET_SERIALIZED_AGENT_TYPE, vec![1]),
                    (0x00e0_5ef0, vec![payload]),
                ]
            );
            assert_eq!(run.size, 0x24);
            // A nonzero low byte keeps the head and moves the payload.
            let run = run_entry(kind, 0x201, 2, 0x20, 0);
            assert_eq!(
                logged(&run),
                vec![
                    (GET_SERIALIZED_AGENT_TYPE, vec![2]),
                    (0x00e0_5f60, vec![run.entry + 0x20]),
                ]
            );
            // Only the low byte of the layout counts.
            let run = run_entry(kind, 0x100, 1, 0x10, 0);
            assert_eq!(logged(&run)[0].0, 0x00e0_5eb0);
        }
    }

    #[test]
    fn the_long_group_takes_its_head_and_payload_offset_from_the_layout_00e06070() {
        for kind in [4u8, 5, 6, 12, 13, 14] {
            let run = run_entry(kind, 0, 1, 0x20, 0);
            assert_eq!(
                logged(&run),
                vec![
                    (0x00e0_5e50, vec![run.entry]),
                    (GET_SERIALIZED_AGENT_TYPE, vec![1]),
                    (0x00e0_5ef0, vec![run.entry + 0x20]),
                ]
            );
            let run = run_entry(kind, 1, 1, 0x30, 0);
            assert_eq!(
                logged(&run),
                vec![
                    (0x00e0_5e20, vec![run.entry]),
                    (GET_SERIALIZED_AGENT_TYPE, vec![1]),
                    (0x00e0_5ef0, vec![run.entry + 0x30]),
                ]
            );
        }
    }

    #[test]
    fn the_agent_type_picks_the_payload_swapper_00e06070() {
        // Types 3 and 4 swap the words, then the counted block behind them.
        for agent in [3u32, 4] {
            let run = run_entry(2, 1, agent, 0x20, 0);
            let payload = run.entry + 0x20;
            assert_eq!(
                logged(&run)[1..],
                [
                    (0x00e0_5fe0, vec![payload]),
                    (0x00e0_6010, vec![payload + 0xc])
                ]
            );
            assert_eq!(run.swapped_track, 0);
        }
        // Type 5 does the same when bit 5 of the byte at +0xB is set, and otherwise goes
        // through the track.
        let run = run_entry(2, 1, 5, 0x20, 0x20);
        assert_eq!(logged(&run).len(), 3);
        assert_eq!(run.swapped_track, 0);
        let run = run_entry(2, 1, 5, 0x20, 0xdf);
        assert_eq!(
            logged(&run)[1..],
            [(0x00e0_61c0, vec![run.context, run.track])]
        );
        assert_eq!(run.swapped_track, 1);
        for agent in 6u32..=9 {
            let run = run_entry(2, 1, agent, 0x20, 0x20);
            assert_eq!(
                logged(&run)[1..],
                [(0x00e0_61c0, vec![run.context, run.track])]
            );
            assert_eq!(run.swapped_track, 1);
        }
        // Other types leave the payload alone, but the size is still written.
        for agent in [0u32, 10, 0x1234, 0xffff_ffff] {
            let run = run_entry(2, 1, agent, 0x20, 0);
            assert_eq!(logged(&run).len(), 1);
            assert_eq!(run.size, 0x24);
            assert_eq!(run.swapped_track, 0);
        }
    }

    /// A track of two items, three entries (of 0x10 bytes) in the first and two in the second,
    /// and the given blocks: `(item, last offset)` each. Returns the track, the items, the
    /// context.
    fn track_with(e: &mut Engine, blocks: &[(u32, u32)]) -> (u32, [u32; 2], u32) {
        let first = e.mem.alloc(0x50);
        let second = e.mem.alloc(0x40);
        e.mem.set_u32(first, 0x30);
        e.mem.set_u32(second, 0x20);
        let items = e.mem.alloc(8);
        e.mem.set_u32(items, first);
        e.mem.set_u32(items + 4, second);
        let list = e.mem.alloc(0x20);
        for (index, (item, limit)) in blocks.iter().enumerate() {
            let block = e.mem.alloc(0x30);
            e.mem.set_u32(block + 0x18, *item);
            e.mem.set_u32(block + 0x1c, *limit);
            e.mem.set_u32(list + 4 * index as u32, block);
        }
        let track = e.mem.alloc(0x20);
        e.mem.set_u32(track, items);
        e.mem.set_u32(track + 4, 2);
        e.mem.set_u32(track + 0xc, list);
        e.mem.set_u32(track + 0x10, blocks.len() as u32);
        (track, [first, second], 0x7777)
    }

    /// The `(entry, block)` pair of every call to the entry function, with sizes of 0x10.
    fn track_run(blocks: &[(u32, u32)], sizes: &[u32]) -> (Vec<(u32, u32)>, Vec<u32>, [u32; 2]) {
        let mut e = engine();
        let seen = Rc::new(std::cell::RefCell::new(Vec::new()));
        let log = Rc::clone(&seen);
        let sizes = sizes.to_vec();
        let mut call = 0;
        // The out words: +0x1c and the `track` argument's slot (+0x30) of the exe's frame.
        let frame = e.next_stack_local(0x34);
        e.register_double(0x00e0_6070, move |e, a| {
            assert_eq!(a[3] & 0xff, 0);
            assert_eq!(a[4..6], [frame + 0x1c, frame + 0x30]);
            log.borrow_mut().push((a[1], a[2], a[0]));
            e.mem.set_u32(a[4], sizes[call.min(sizes.len() - 1)]);
            call += 1;
            Ret::default()
        });
        let (track, items, context) = track_with(&mut e, blocks);
        let list = e.mem.u32(track + 0xc);
        let block_ptrs: Vec<u32> = (0..blocks.len() as u32)
            .map(|index| e.mem.u32(list + 4 * index))
            .collect();
        e.call(0x00e0_61c0, &args![context, track]);
        let seen = seen.borrow();
        assert!(seen.iter().all(|(_, _, ctx)| *ctx == context));
        (
            seen.iter()
                .map(|(entry, block, _)| (*entry, *block))
                .collect(),
            block_ptrs,
            items,
        )
    }

    #[test]
    fn entries_are_visited_with_the_block_that_covers_them_00e061c0() {
        let (seen, blocks, items) = track_run(&[(0, 0x10), (0, 0x2f), (1, 0x1f)], &[0x10]);
        let [first, second] = items;
        assert_eq!(
            seen,
            vec![
                (first + 0x10, blocks[0]),
                (first + 0x20, blocks[0]),
                (first + 0x30, blocks[1]),
                (second + 0x10, blocks[2]),
                (second + 0x20, blocks[2]),
            ]
        );
    }

    #[test]
    fn without_more_blocks_the_entries_get_none_00e061c0() {
        let (seen, blocks, items) = track_run(&[(0, 0x10)], &[0x10]);
        let [first, second] = items;
        assert_eq!(
            seen,
            vec![
                (first + 0x10, blocks[0]),
                (first + 0x20, blocks[0]),
                (first + 0x30, 0),
                (second + 0x10, 0),
                (second + 0x20, 0),
            ]
        );
        let (seen, _, _) = track_run(&[], &[0x10]);
        assert_eq!(seen.len(), 5);
        assert!(seen.iter().all(|(_, block)| *block == 0));
    }

    #[test]
    fn the_size_an_entry_reports_moves_to_the_next_one_00e061c0() {
        let (seen, _, items) = track_run(&[], &[0x20, 0x10, 0x30, 0x10, 0x10]);
        let [first, second] = items;
        // The first item ends at +0x40: entries at +0x10 and +0x30. The second ends at +0x30: its one entry.
        assert_eq!(
            seen.iter().map(|(entry, _)| *entry).collect::<Vec<_>>(),
            vec![first + 0x10, first + 0x30, second + 0x10]
        );
    }

    #[test]
    fn a_track_without_items_is_left_alone_00e061c0() {
        let mut e = engine();
        let log = new_log();
        log_double(&mut e, &log, 0x00e0_6070, 0);
        let track = e.mem.alloc(0x20);
        e.call(0x00e0_61c0, &args![0u32, track]);
        assert!(log.borrow().is_empty());
    }

    /// A collision entry with three records of 0x20 bytes at `[+0x50]`, two extra dwords each
    /// and the stride and byte count given.
    fn collision_entry(e: &mut Engine, stride: u8, size: i32, dwords: u8) -> (u32, u32) {
        let entry = e.mem.alloc(0x140);
        e.mem.write(entry, &[0; 0x140]);
        let records = numbered(e, 0x80);
        e.mem.set_u32(entry + 0x50, records);
        e.mem.set_u8(entry + 0x2a, stride);
        e.mem.set_i32(entry + 0x54, size);
        e.mem.set_u8(entry + 0x28, dwords);
        (entry, records)
    }

    #[test]
    fn the_collision_entry_swaps_every_record_00e062a0() {
        let mut e = engine();
        let seen = Rc::new(std::cell::RefCell::new(Vec::new()));
        let log = Rc::clone(&seen);
        e.register_double(0x00e0_6070, move |_, a| {
            log.borrow_mut().push(a.to_vec());
            Ret::default()
        });
        // 0x5f / 0x20 = 2 whole records.
        let (entry, records) = collision_entry(&mut e, 0x20, 0x5f, 1);
        let before = bytes(&e, records, 0x80);
        // The out words: +0x8 and the `entry` argument's slot (+0x14) of the exe's frame.
        let frame = e.next_stack_local(0x18);
        e.call(0x00e0_62a0, &args![0x7777u32, entry]);
        let seen = seen.borrow();
        assert_eq!(seen.len(), 1);
        assert_eq!(seen[0][..3], [0x7777, entry + 0x74, entry + 0x114]);
        assert_eq!(seen[0][4..6], [frame + 8, frame + 0x14]);
        assert_eq!(seen[0][3] & 0xff, 1);
        assert_eq!(e.mem.u32(entry + 0x12c), 0x103);
        let mut groups = Vec::new();
        for record in 0..2 {
            let base = 0x20 * record;
            groups.extend([
                (base + 0x10, 4),
                (base, 4),
                (base + 4, 4),
                (base + 8, 4),
                (base + 0x14, 4),
                (base + 0x18, 4),
            ]);
        }
        assert_eq!(bytes(&e, records, 0x80), reversed(&before, &groups));
    }

    #[test]
    fn without_extra_dwords_only_the_fixed_ones_are_swapped_00e062a0() {
        let mut e = engine();
        log_double(&mut e, &new_log(), 0x00e0_6070, 0);
        let (entry, records) = collision_entry(&mut e, 0x40, 0x40, 0);
        let before = bytes(&e, records, 0x80);
        e.call(0x00e0_62a0, &args![0u32, entry]);
        assert_eq!(
            bytes(&e, records, 0x80),
            reversed(&before, &[(0x10, 4), (0, 4), (4, 4), (8, 4)])
        );
    }

    #[test]
    fn no_records_are_swapped_when_the_count_is_not_positive_00e062a0() {
        for size in [0, 0x1f, -0x40] {
            let mut e = engine();
            log_double(&mut e, &new_log(), 0x00e0_6070, 0);
            let (entry, records) = collision_entry(&mut e, 0x20, size, 2);
            let before = bytes(&e, records, 0x80);
            e.call(0x00e0_62a0, &args![0u32, entry]);
            assert_eq!(bytes(&e, records, 0x80), before);
            assert_eq!(e.mem.u32(entry + 0x12c), 0x103);
        }
    }

    #[test]
    #[should_panic(expected = "00e062a0")]
    fn a_zero_stride_is_a_divide_error_00e062a0() {
        let mut e = engine();
        log_double(&mut e, &new_log(), 0x00e0_6070, 0);
        let (entry, _) = collision_entry(&mut e, 0, 0x40, 0);
        e.call(0x00e0_62a0, &args![0u32, entry]);
    }
}

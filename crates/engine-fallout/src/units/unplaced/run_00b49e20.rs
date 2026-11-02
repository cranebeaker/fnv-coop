//! `(unplaced)/run_00b49e20` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00b49e20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, three words: a running checksum, a
/// buffer and its length). A null buffer answers 1. Otherwise the low and
/// high halves of the checksum sum the bytes and the running sums, in blocks
/// of at most 0x15b0 bytes (16 at a time while at least 16 remain, then one
/// by one), each block ending with both halves taken modulo 0xfff1. Answers
/// `high << 16 | low`.
pub fn fn_00b49e20(e: &mut Engine, checksum: u32, buffer: u32, length: u32) -> u32 {
    let mut low = checksum & 0xffff;
    let mut high = (checksum >> 16) & 0xffff;
    if buffer == 0 {
        return 1;
    }
    let mut at = buffer;
    let mut length = length;
    while length != 0 {
        let mut block = if length < 0x15b0 { length } else { 0x15b0 };
        length -= block;
        while (block as i32) >= 0x10 {
            for i in 0..0x10 {
                low = low.wrapping_add(u32::from(e.mem.u8(at + i)));
                high = high.wrapping_add(low);
            }
            at = at.wrapping_add(0x10);
            block -= 0x10;
        }
        while block != 0 {
            low = low.wrapping_add(u32::from(e.mem.u8(at)));
            at = at.wrapping_add(1);
            high = high.wrapping_add(low);
            block -= 1;
        }
        low %= 0xfff1;
        high %= 0xfff1;
    }
    (high << 16) | low
}

/// The CRC table `00b4a070`'s byte-at-a-time branch reads (256 words in
/// `.rdata`; the exe's words are the reflected CRC-32 table, `00000000`,
/// `77073096`, `ee0e612c`, ...).
const CRC_TABLE: u32 = 0x010a_6550;
/// The two word-at-a-time CRC helpers `00b4a070` chooses between (`cdecl`,
/// the same three words): `00b4a270` (the branch the exe takes) and
/// `00b4a670`.
const CRC_TAKEN_HELPER: u32 = 0x00b4_a270;
const CRC_OTHER_HELPER: u32 = 0x00b4_a670;
/// The three further 256-word tables that follow `CRC_TABLE` in `.rdata`,
/// which `00b4a270`'s word steps index with the CRC's bytes 2, 1 and 0
/// (`CRC_TABLE` takes byte 3).
const CRC_TABLE_1: u32 = 0x010a_6950;
const CRC_TABLE_2: u32 = 0x010a_6d50;
const CRC_TABLE_3: u32 = 0x010a_7150;
/// The four 256-word tables after those, which `00b4a670` reads (its byte
/// steps the first; its word steps the four, with the CRC's bytes 0 to 3).
/// The words sampled from the exe are those of `CRC_TABLE` and its three
/// followers with their bytes reversed.
const CRC_SWAPPED_TABLE_0: u32 = 0x010a_7550;
const CRC_SWAPPED_TABLE_1: u32 = 0x010a_7950;
const CRC_SWAPPED_TABLE_2: u32 = 0x010a_7d50;
const CRC_SWAPPED_TABLE_3: u32 = 0x010a_8150;

/// The messages `00b49070` leaves in the stream's message word (`+0x18`):
/// "invalid distance too far back", "invalid distance code" and
/// "invalid literal/length code" (`.rdata`).
const MESSAGE_DISTANCE_TOO_FAR: u32 = 0x011a_d71c;
const MESSAGE_DISTANCE_CODE: u32 = 0x011a_d73c;
const MESSAGE_LITERAL_LENGTH_CODE: u32 = 0x011a_d754;

/// The 16-bit base and extra-bit tables `00b497d0` reads for its kinds 1 and
/// 2 (`.rdata`): kind 1 indexes the first two from symbol 257 (the exe
/// subtracts 0x202 bytes from each address), kind 2 the last two from 0.
const LENGTH_BASE: u32 = 0x010a_6450;
const LENGTH_EXTRA: u32 = 0x010a_6490;
const DISTANCE_BASE: u32 = 0x010a_64d0;
const DISTANCE_EXTRA: u32 = 0x010a_6510;

/// The address of the 16-bit element `index` of the array at `base`.
fn half_word_at(base: u32, index: u32) -> u32 {
    base.wrapping_add(index.wrapping_mul(2))
}

/// A decoding-table entry as the exe stores it: the operation byte, the bit
/// count byte and the 16-bit value.
fn table_entry(operation: u8, bits: u8, value: u16) -> u32 {
    u32::from(operation) | (u32::from(bits) << 8) | (u32::from(value) << 16)
}

/// Reads the next input byte into the bit buffer (the input pointer is
/// moved first, then the byte at it is added shifted by the bit count).
fn fast_pull_byte(e: &mut Engine, input: &mut u32, hold: &mut u32, bits: &mut u32) {
    *input = input.wrapping_add(1);
    let byte = u32::from(e.mem.u8(*input));
    *hold = byte.wrapping_shl(*bits).wrapping_add(*hold);
    *bits = bits.wrapping_add(8);
}

/// One byte copied in `00b49070`'s copy loops: both pointers move first,
/// then the byte at the source goes to the output.
fn fast_copy_byte(e: &mut Engine, from: &mut u32, out: &mut u32) {
    *from = from.wrapping_add(1);
    *out = out.wrapping_add(1);
    let byte = e.mem.u8(*from);
    e.mem.set_u8(*out, byte);
}

/// Copies `count` bytes (at least one: the exe's loop tests after copying).
fn fast_copy_run(e: &mut Engine, from: &mut u32, out: &mut u32, count: u32) {
    let mut count = count;
    loop {
        fast_copy_byte(e, from, out);
        count = count.wrapping_sub(1);
        if count == 0 {
            break;
        }
    }
}

// Translated from 00b49070 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `zLib/inflate.c`; `cdecl`, two words:
/// the stream record and the output position the caller started at). The
/// body is a fast Huffman decoding loop over the stream's buffers, the shape
/// of zlib's `inflate_fast` (the identity is by the body). From the stream
/// it reads the input pointer and count (`+0x0`, `+0x4`) and the output
/// pointer and count (`+0xc`, `+0x10`); from the state (`+0x1c`) the window
/// size, fill, write position and buffer (`+0x20` .. `+0x2c`), the bit
/// buffer and its count (`+0x30`, `+0x34`), the literal/length and distance
/// tables (`+0x44`, `+0x48`) and their index bits (`+0x4c`, `+0x50`).
///
/// While at least 6 input bytes and 258 output bytes remain it tops the bit
/// buffer up two bytes at a time below 15 bits, looks the code up and drops
/// its bits; an operation of 0 is a literal byte; bit 0x10 is a length (base
/// plus the operation's low four bits of extra bits) followed by a distance
/// code from the distance table, then a copy from the output or, beyond what
/// was output, from the window (wrapping at its write position); a
/// distance beyond the window's fill sets the stream's message to "invalid
/// distance too far back" and the mode (`state +0x0`) to 0x1b. Without bit
/// 0x40 an operation names a second-level entry (its value plus the next
/// operation-many bits); with it a length code ends the block (bit 0x20:
/// mode 0xb) or is invalid (message, mode 0x1b), and a distance code is
/// invalid (message, mode 0x1b). On leaving, the whole unused bytes go
/// back to the input, the pointers and counts are written back and the bit
/// buffer keeps only its remaining bits.
pub fn fn_00b49070(e: &mut Engine, stream: u32, start: u32) {
    let state = e.mem.u32(stream + 0x1c);
    let mut input = e.mem.u32(stream).wrapping_sub(1);
    let last = input.wrapping_add(e.mem.u32(stream + 0x4)).wrapping_sub(5);
    let mut out = e.mem.u32(stream + 0xc).wrapping_sub(1);
    let begin = out.wrapping_sub(start.wrapping_sub(e.mem.u32(stream + 0x10)));
    let end = out
        .wrapping_add(e.mem.u32(stream + 0x10))
        .wrapping_sub(0x101);
    let window_size = e.mem.u32(state + 0x20);
    let window_have = e.mem.u32(state + 0x24);
    let window_write = e.mem.u32(state + 0x28);
    let window = e.mem.u32(state + 0x2c);
    let mut hold = e.mem.u32(state + 0x30);
    let mut bits = e.mem.u32(state + 0x34);
    let length_codes = e.mem.u32(state + 0x44);
    let distance_codes = e.mem.u32(state + 0x48);
    let length_mask = 1u32.wrapping_shl(e.mem.u32(state + 0x4c)).wrapping_sub(1);
    let distance_mask = 1u32.wrapping_shl(e.mem.u32(state + 0x50)).wrapping_sub(1);
    'decode: loop {
        if bits < 0xf {
            fast_pull_byte(e, &mut input, &mut hold, &mut bits);
            fast_pull_byte(e, &mut input, &mut hold, &mut bits);
        }
        let mut code = e
            .mem
            .u32(length_codes.wrapping_add((hold & length_mask).wrapping_mul(4)));
        // The literal/length code (the exe's 00b49193).
        loop {
            let drop = (code >> 8) & 0xff;
            hold = hold.wrapping_shr(drop);
            bits = bits.wrapping_sub(drop);
            let operation = code & 0xff;
            if operation == 0 {
                out = out.wrapping_add(1);
                e.mem.set_u8(out, (code >> 16) as u8);
                break;
            }
            if operation & 0x10 != 0 {
                let mut length = code >> 16;
                let extra = operation & 0xf;
                if extra != 0 {
                    if bits < extra {
                        fast_pull_byte(e, &mut input, &mut hold, &mut bits);
                    }
                    length = (1u32.wrapping_shl(extra).wrapping_sub(1) & hold).wrapping_add(length);
                    hold = hold.wrapping_shr(extra);
                    bits = bits.wrapping_sub(extra);
                }
                if bits < 0xf {
                    fast_pull_byte(e, &mut input, &mut hold, &mut bits);
                    fast_pull_byte(e, &mut input, &mut hold, &mut bits);
                }
                code = e
                    .mem
                    .u32(distance_codes.wrapping_add((hold & distance_mask).wrapping_mul(4)));
                // The distance code (the exe's 00b4929f).
                loop {
                    let drop = (code >> 8) & 0xff;
                    hold = hold.wrapping_shr(drop);
                    bits = bits.wrapping_sub(drop);
                    let operation = code & 0xff;
                    if operation & 0x10 != 0 {
                        let mut distance = code >> 16;
                        let extra = operation & 0xf;
                        if bits < extra {
                            fast_pull_byte(e, &mut input, &mut hold, &mut bits);
                            if bits < extra {
                                fast_pull_byte(e, &mut input, &mut hold, &mut bits);
                            }
                        }
                        distance = (1u32.wrapping_shl(extra).wrapping_sub(1) & hold)
                            .wrapping_add(distance);
                        hold = hold.wrapping_shr(extra);
                        bits = bits.wrapping_sub(extra);
                        let mut copy = out.wrapping_sub(begin);
                        if distance > copy {
                            copy = distance.wrapping_sub(copy);
                            if copy > window_have {
                                e.mem.set_u32(stream + 0x18, MESSAGE_DISTANCE_TOO_FAR);
                                e.mem.set_u32(state, 0x1b);
                                break 'decode;
                            }
                            let mut from = window.wrapping_sub(1);
                            if window_write == 0 {
                                from = window_size.wrapping_sub(copy).wrapping_add(from);
                                if copy < length {
                                    length = length.wrapping_sub(copy);
                                    fast_copy_run(e, &mut from, &mut out, copy);
                                    from = out.wrapping_sub(distance);
                                }
                            } else if window_write < copy {
                                from = window_size
                                    .wrapping_add(window_write)
                                    .wrapping_sub(copy)
                                    .wrapping_add(from);
                                copy = copy.wrapping_sub(window_write);
                                if copy < length {
                                    length = length.wrapping_sub(copy);
                                    fast_copy_run(e, &mut from, &mut out, copy);
                                    from = window.wrapping_sub(1);
                                    if window_write < length {
                                        copy = window_write;
                                        length = length.wrapping_sub(copy);
                                        fast_copy_run(e, &mut from, &mut out, copy);
                                        from = out.wrapping_sub(distance);
                                    }
                                }
                            } else {
                                from = window_write.wrapping_sub(copy).wrapping_add(from);
                                if copy < length {
                                    length = length.wrapping_sub(copy);
                                    fast_copy_run(e, &mut from, &mut out, copy);
                                    from = out.wrapping_sub(distance);
                                }
                            }
                            while length > 2 {
                                fast_copy_byte(e, &mut from, &mut out);
                                fast_copy_byte(e, &mut from, &mut out);
                                fast_copy_byte(e, &mut from, &mut out);
                                length = length.wrapping_sub(3);
                            }
                            if length != 0 {
                                fast_copy_byte(e, &mut from, &mut out);
                                if length > 1 {
                                    fast_copy_byte(e, &mut from, &mut out);
                                }
                            }
                        } else {
                            // Copy from the output itself (three bytes at
                            // least: the exe's loop tests after copying).
                            let mut from = out.wrapping_sub(distance);
                            loop {
                                fast_copy_byte(e, &mut from, &mut out);
                                fast_copy_byte(e, &mut from, &mut out);
                                fast_copy_byte(e, &mut from, &mut out);
                                length = length.wrapping_sub(3);
                                if length <= 2 {
                                    break;
                                }
                            }
                            if length != 0 {
                                fast_copy_byte(e, &mut from, &mut out);
                                if length > 1 {
                                    fast_copy_byte(e, &mut from, &mut out);
                                }
                            }
                        }
                        break;
                    }
                    if operation & 0x40 == 0 {
                        let index = (1u32.wrapping_shl(operation).wrapping_sub(1) & hold)
                            .wrapping_add(code >> 16);
                        code = e
                            .mem
                            .u32(distance_codes.wrapping_add(index.wrapping_mul(4)));
                        continue;
                    }
                    e.mem.set_u32(stream + 0x18, MESSAGE_DISTANCE_CODE);
                    e.mem.set_u32(state, 0x1b);
                    break 'decode;
                }
                break;
            }
            if operation & 0x40 == 0 {
                let index =
                    (1u32.wrapping_shl(operation).wrapping_sub(1) & hold).wrapping_add(code >> 16);
                code = e.mem.u32(length_codes.wrapping_add(index.wrapping_mul(4)));
                continue;
            }
            if operation & 0x20 != 0 {
                e.mem.set_u32(state, 0xb);
            } else {
                e.mem.set_u32(stream + 0x18, MESSAGE_LITERAL_LENGTH_CODE);
                e.mem.set_u32(state, 0x1b);
            }
            break 'decode;
        }
        // The exe's 00b496f8: go on while input and output remain.
        if input >= last || out >= end {
            break;
        }
    }
    // The exe's 00b4970c: give back the unused whole bytes.
    let unused = bits >> 3;
    input = input.wrapping_sub(unused);
    bits = bits.wrapping_sub(unused << 3);
    hold &= 1u32.wrapping_shl(bits).wrapping_sub(1);
    e.mem.set_u32(stream, input.wrapping_add(1));
    e.mem.set_u32(stream + 0xc, out.wrapping_add(1));
    let input_left = if input < last {
        last.wrapping_sub(input).wrapping_add(5)
    } else {
        5u32.wrapping_sub(input.wrapping_sub(last))
    };
    e.mem.set_u32(stream + 0x4, input_left);
    let output_left = if out < end {
        end.wrapping_sub(out).wrapping_add(0x101)
    } else {
        0x101u32.wrapping_sub(out.wrapping_sub(end))
    };
    e.mem.set_u32(stream + 0x10, output_left);
    e.mem.set_u32(state + 0x30, hold);
    e.mem.set_u32(state + 0x34, bits);
}

// Translated from 00b497d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `zLib/inflate.c`; `cdecl`, six words: a
/// kind, an array of 16-bit code lengths, their count, the address of the
/// table pointer, the address of the root bit count and a 16-bit work
/// array). The body builds a canonical Huffman decoding table, the shape of
/// zlib's `inflate_table` (the identity is by the body). It counts the
/// lengths, clamps the root bits to the longest and shortest lengths used,
/// and answers -1 when no length is used, when the lengths over-subscribe,
/// or when they are incomplete unless the kind is not 0 and exactly one
/// code is used. It sorts the symbols by length into the work array and
/// fills the table: kind 0 entries carry the symbol itself (operation 0)
/// below 19, 19 as operation 0x60 and above it the work array's own words
/// as base and extra, kind 1 literals below 256 (operation 0), 256 as operation
/// 0x60 and the rest from the length tables, any other kind every symbol
/// from the distance tables. Codes longer than the root go to second-level
/// tables after the root table, linked from it (operation = the sub-table's
/// bits, bits = root, value = its offset in entries). Kind 1 answers 1 when
/// the entries used reach 0x506. An incomplete table is padded with
/// invalid entries (operation 0x40). On success the table pointer moves
/// past the entries used, the root bit count is written back and it
/// answers 0.
// Translation incomplete: 00b497d0 - the /GS stack cookie check (00ec408c) is not translated.
pub fn fn_00b497d0(
    e: &mut Engine,
    kind: u32,
    lens: u32,
    codes: u32,
    table: u32,
    bits: u32,
    work: u32,
) -> i32 {
    // The exe keeps its 16 length counts at EBP-0x24 and its 16 offsets at
    // EBP-0x78 of a 0x94-byte frame; a length index beyond 15 reaches the
    // rest of that memory, so both arrays live in a block laid out like it.
    e.with_stack(0x94, |e, frame| {
        let count = frame.addr() + 0x70;
        let offsets = frame.addr() + 0x1c;
        let mut index = 0u32;
        while index <= 0xf {
            e.mem.set_u16(half_word_at(count, index), 0);
            index += 1;
        }
        let mut symbol = 0u32;
        while symbol < codes {
            let length = u32::from(e.mem.u16(half_word_at(lens, symbol)));
            let counted = e.mem.u16(half_word_at(count, length)).wrapping_add(1);
            e.mem.set_u16(half_word_at(count, length), counted);
            symbol += 1;
        }
        let mut root = e.mem.u32(bits);
        let mut max = 0xfu32;
        while max >= 1 {
            if e.mem.u16(half_word_at(count, max)) != 0 {
                break;
            }
            max -= 1;
        }
        if root > max {
            root = max;
        }
        if max == 0 {
            return -1;
        }
        let mut min = 1u32;
        while min <= 0xf {
            if e.mem.u16(half_word_at(count, min)) != 0 {
                break;
            }
            min += 1;
        }
        if root < min {
            root = min;
        }
        let mut left = 1i32;
        let mut length = 1u32;
        while length <= 0xf {
            left <<= 1;
            left = left.wrapping_sub(i32::from(e.mem.u16(half_word_at(count, length))));
            if left < 0 {
                return -1;
            }
            length += 1;
        }
        if left > 0
            && (kind == 0 || codes.wrapping_sub(u32::from(e.mem.u16(half_word_at(count, 0)))) != 1)
        {
            return -1;
        }
        e.mem.set_u16(half_word_at(offsets, 1), 0);
        let mut length = 1u32;
        while length < 0xf {
            let next = e
                .mem
                .u16(half_word_at(offsets, length))
                .wrapping_add(e.mem.u16(half_word_at(count, length)));
            e.mem.set_u16(half_word_at(offsets, length + 1), next);
            length += 1;
        }
        let mut symbol = 0u32;
        while symbol < codes {
            let length = u32::from(e.mem.u16(half_word_at(lens, symbol)));
            if length != 0 {
                let slot = u32::from(e.mem.u16(half_word_at(offsets, length)));
                e.mem.set_u16(half_word_at(work, slot), symbol as u16);
                let moved = e.mem.u16(half_word_at(offsets, length)).wrapping_add(1);
                e.mem.set_u16(half_word_at(offsets, length), moved);
            }
            symbol += 1;
        }
        let (base, extra, end): (u32, u32, i32) = match kind {
            0 => (work, work, 0x13),
            1 => (LENGTH_BASE - 0x202, LENGTH_EXTRA - 0x202, 0x100),
            _ => (DISTANCE_BASE, DISTANCE_EXTRA, -1),
        };
        let mut huff = 0u32;
        let mut symbol = 0u32;
        let mut length = min;
        let mut next = e.mem.u32(table);
        let mut current = root;
        let mut drop = 0u32;
        let mut low = u32::MAX;
        let mut used = 1u32.wrapping_shl(root);
        let mask = used.wrapping_sub(1);
        if kind == 1 && used >= 0x506 {
            return 1;
        }
        // The exe's 00b49ac4: one entry (replicated) per symbol.
        loop {
            let entry_bits = length.wrapping_sub(drop) as u8;
            let sorted = e.mem.u16(half_word_at(work, symbol));
            let (operation, value) = if i32::from(sorted) < end {
                (0u8, sorted)
            } else if i32::from(sorted) > end {
                (
                    e.mem.u8(half_word_at(extra, u32::from(sorted))),
                    e.mem.u16(half_word_at(base, u32::from(sorted))),
                )
            } else {
                (0x60u8, 0u16)
            };
            let entry = table_entry(operation, entry_bits, value);
            let increment = 1u32.wrapping_shl(length.wrapping_sub(drop));
            let mut fill = 1u32.wrapping_shl(current);
            loop {
                fill = fill.wrapping_sub(increment);
                let at = huff.wrapping_shr(drop).wrapping_add(fill);
                e.mem.set_u32(next.wrapping_add(at.wrapping_mul(4)), entry);
                if fill == 0 {
                    break;
                }
            }
            let mut increment = 1u32.wrapping_shl(length.wrapping_sub(1));
            while huff & increment != 0 {
                increment >>= 1;
            }
            if increment != 0 {
                huff = (increment.wrapping_sub(1) & huff).wrapping_add(increment);
            } else {
                huff = 0;
            }
            symbol = symbol.wrapping_add(1);
            let left_of_length = e.mem.u16(half_word_at(count, length)).wrapping_sub(1);
            e.mem.set_u16(half_word_at(count, length), left_of_length);
            if left_of_length == 0 {
                if length == max {
                    break;
                }
                let sorted = u32::from(e.mem.u16(half_word_at(work, symbol)));
                length = u32::from(e.mem.u16(half_word_at(lens, sorted)));
            }
            if length > root && (huff & mask) != low {
                if drop == 0 {
                    drop = root;
                }
                next = next.wrapping_add(1u32.wrapping_shl(current).wrapping_mul(4));
                current = length.wrapping_sub(drop);
                let mut left = 1i32.wrapping_shl(current);
                while current.wrapping_add(drop) < max {
                    let counted = e.mem.u16(half_word_at(count, current.wrapping_add(drop)));
                    left = left.wrapping_sub(i32::from(counted));
                    if left <= 0 {
                        break;
                    }
                    current = current.wrapping_add(1);
                    left = left.wrapping_shl(1);
                }
                used = used.wrapping_add(1u32.wrapping_shl(current));
                if kind == 1 && used >= 0x506 {
                    return 1;
                }
                low = huff & mask;
                let root_table = e.mem.u32(table);
                e.mem
                    .set_u8(root_table.wrapping_add(low.wrapping_mul(4)), current as u8);
                let root_table = e.mem.u32(table);
                e.mem.set_u8(
                    root_table.wrapping_add(low.wrapping_mul(4)).wrapping_add(1),
                    root as u8,
                );
                let offset = (next.wrapping_sub(e.mem.u32(table)) as i32) >> 2;
                let root_table = e.mem.u32(table);
                e.mem.set_u16(
                    root_table.wrapping_add(low.wrapping_mul(4)).wrapping_add(2),
                    offset as u16,
                );
            }
        }
        // The exe's 00b49d2e: pad an incomplete code with invalid entries.
        let mut entry_bits = length.wrapping_sub(drop) as u8;
        while huff != 0 {
            if drop != 0 && (huff & mask) != low {
                // Back to the root table (the exe also sets its current
                // sub-table bits to the root here; nothing reads them after).
                drop = 0;
                length = root;
                next = e.mem.u32(table);
                entry_bits = length as u8;
            }
            let entry = table_entry(0x40, entry_bits, 0);
            let at = huff.wrapping_shr(drop);
            e.mem.set_u32(next.wrapping_add(at.wrapping_mul(4)), entry);
            let mut increment = 1u32.wrapping_shl(length.wrapping_sub(1));
            while huff & increment != 0 {
                increment >>= 1;
            }
            if increment != 0 {
                huff = (increment.wrapping_sub(1) & huff).wrapping_add(increment);
            } else {
                huff = 0;
            }
        }
        let moved = e.mem.u32(table).wrapping_add(used.wrapping_mul(4));
        e.mem.set_u32(table, moved);
        e.mem.set_u32(bits, root);
        0
    })
}

/// One step of `00b4a070`'s byte-at-a-time branch: the CRC shifted down a
/// byte, xor the table word the byte and the CRC's low byte select.
fn crc_table_step(e: &mut Engine, crc: u32, at: u32) -> u32 {
    let index = (u32::from(e.mem.u8(at)) ^ crc) & 0xff;
    (crc >> 8) ^ e.mem.u32(CRC_TABLE.wrapping_add(index.wrapping_mul(4)))
}

// Translated from 00b4a070 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, three words: a running CRC, a buffer
/// and its length): a CRC-32 (the identity is by the table its other branch
/// reads, the exe's reflected CRC-32 table at `010a6550`). A null buffer
/// answers 0. The exe then tests the constant 1 and the low byte of a stack
/// word it set to 1 (a byte-order probe), both always true, so it answers
/// `00b4a270(crc, buffer, length)`; the probe's other answer calls
/// `00b4a670` with the same words, and the constant's other answer runs the
/// CRC here a byte at a time (complemented before and after, eight bytes per
/// round while at least eight remain, then one by one).
pub fn fn_00b4a070(e: &mut Engine, crc: u32, buffer: u32, length: u32) -> u32 {
    if buffer == 0 {
        return 0;
    }
    // MOV EAX,1; TEST EAX,EAX: the exe tests a constant (always taken).
    let constant = 1u32;
    if constant != 0 {
        // The word 1 stored at EBP-0x4 and its first (low) byte read back.
        let probe = 1u32;
        if probe.to_le_bytes()[0] != 0 {
            return e.call(CRC_TAKEN_HELPER, &args![crc, buffer, length]).u32();
        }
        return e.call(CRC_OTHER_HELPER, &args![crc, buffer, length]).u32();
    }
    // Not reached (the constant above): the byte-at-a-time CRC.
    let mut crc = crc ^ 0xffff_ffff;
    let mut at = buffer;
    let mut length = length;
    while length >= 8 {
        for _ in 0..8 {
            crc = crc_table_step(e, crc, at);
            at = at.wrapping_add(1);
        }
        length -= 8;
    }
    while length != 0 {
        crc = crc_table_step(e, crc, at);
        at = at.wrapping_add(1);
        length -= 1;
    }
    crc ^ 0xffff_ffff
}

/// The word `index` of the 256-word table at `table`.
fn crc_word(e: &mut Engine, table: u32, index: u32) -> u32 {
    e.mem.u32(table.wrapping_add(index.wrapping_mul(4)))
}

/// One word step of `00b4a270` (after the word is xored in): the CRC's
/// bytes 0, 1, 2 and 3 index `CRC_TABLE_3`, `_2`, `_1` and `CRC_TABLE`, the
/// four words xored in the exe's order.
fn crc_word_step(e: &mut Engine, crc: u32) -> u32 {
    let mut next = crc_word(e, CRC_TABLE_3, crc & 0xff);
    next ^= crc_word(e, CRC_TABLE_2, (crc >> 8) & 0xff);
    next ^= crc_word(e, CRC_TABLE_1, (crc >> 0x10) & 0xff);
    next ^ crc_word(e, CRC_TABLE, crc >> 0x18)
}

// Translated from 00b4a270 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, three words: a running CRC, a buffer
/// and its length; the branch `00b4a070` takes). The CRC is complemented,
/// then the bytes before the first 4-byte-aligned address go through one at
/// a time (`CRC_TABLE`, as `00b4a070`'s own byte branch), then words read
/// little-endian are xored in and stepped through the four tables, eight
/// per round while at least 32 bytes remain, then one per round while at
/// least four remain, then the last bytes one at a time; answers the CRC
/// complemented again.
pub fn fn_00b4a270(e: &mut Engine, crc: u32, buffer: u32, length: u32) -> u32 {
    let mut crc = !crc;
    let mut buffer = buffer;
    let mut length = length;
    while length != 0 && buffer & 3 != 0 {
        crc = crc_table_step(e, crc, buffer);
        buffer = buffer.wrapping_add(1);
        length -= 1;
    }
    let mut at = buffer;
    while length >= 0x20 {
        for _ in 0..8 {
            crc ^= e.mem.u32(at);
            at = at.wrapping_add(4);
            crc = crc_word_step(e, crc);
        }
        length -= 0x20;
    }
    while length >= 4 {
        crc ^= e.mem.u32(at);
        at = at.wrapping_add(4);
        crc = crc_word_step(e, crc);
        length -= 4;
    }
    let mut buffer = at;
    while length != 0 {
        crc = crc_table_step(e, crc, buffer);
        buffer = buffer.wrapping_add(1);
        length -= 1;
    }
    !crc
}

/// The word with its four bytes reversed, as `00b4a670` builds it: the four
/// bytes shifted into place and added.
fn crc_bytes_reversed(value: u32) -> u32 {
    (value >> 0x18) + ((value >> 8) & 0xff00) + ((value & 0xff00) << 8) + ((value & 0xff) << 0x18)
}

/// One byte step of `00b4a670`: the CRC shifted up a byte, xor the word of
/// `CRC_SWAPPED_TABLE_0` the CRC's top byte xor the buffer's byte selects.
fn crc_swapped_byte_step(e: &mut Engine, crc: u32, at: u32) -> u32 {
    let index = (crc >> 0x18) ^ u32::from(e.mem.u8(at));
    (crc << 8) ^ crc_word(e, CRC_SWAPPED_TABLE_0, index)
}

/// One word step of `00b4a670` (after the word is xored in): the CRC's
/// bytes 0, 1, 2 and 3 index `CRC_SWAPPED_TABLE_0` to `_3`, the four words
/// xored in the exe's order.
fn crc_swapped_word_step(e: &mut Engine, crc: u32) -> u32 {
    let mut next = crc_word(e, CRC_SWAPPED_TABLE_0, crc & 0xff);
    next ^= crc_word(e, CRC_SWAPPED_TABLE_1, (crc >> 8) & 0xff);
    next ^= crc_word(e, CRC_SWAPPED_TABLE_2, (crc >> 0x10) & 0xff);
    next ^ crc_word(e, CRC_SWAPPED_TABLE_3, crc >> 0x18)
}

// Translated from 00b4a670 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, three words: a running CRC, a buffer
/// and its length; the branch `00b4a070` does not take). It works on the
/// CRC with its bytes reversed, complemented: the bytes before the first
/// 4-byte-aligned address one at a time (`CRC_SWAPPED_TABLE_0`), then words
/// (read little-endian, as every load on this CPU) xored in and stepped
/// through the four swapped tables, eight per round while at least 32 bytes
/// remain, then one per round while at least four remain, then the last
/// bytes one at a time; answers the result complemented with its bytes
/// reversed back. Its word pointer starts four bytes before the aligned
/// address and moves before each read, as the exe's does.
pub fn fn_00b4a670(e: &mut Engine, crc: u32, buffer: u32, length: u32) -> u32 {
    let mut crc = !crc_bytes_reversed(crc);
    let mut buffer = buffer;
    let mut length = length;
    while length != 0 && buffer & 3 != 0 {
        crc = crc_swapped_byte_step(e, crc, buffer);
        buffer = buffer.wrapping_add(1);
        length -= 1;
    }
    let mut at = buffer.wrapping_sub(4);
    while length >= 0x20 {
        for _ in 0..8 {
            at = at.wrapping_add(4);
            crc ^= e.mem.u32(at);
            crc = crc_swapped_word_step(e, crc);
        }
        length -= 0x20;
    }
    while length >= 4 {
        at = at.wrapping_add(4);
        crc ^= e.mem.u32(at);
        crc = crc_swapped_word_step(e, crc);
        length -= 4;
    }
    let mut buffer = at.wrapping_add(4);
    while length != 0 {
        crc = crc_swapped_byte_step(e, crc, buffer);
        buffer = buffer.wrapping_add(1);
        length -= 1;
    }
    crc_bytes_reversed(!crc)
}

// The compressor's tree half: zlib's `trees.c`, `compress.c` and `crc32.c` shapes by their
// bodies (the engine map names none of them). The compressor state's words they use:
/// The pending buffer and count, the data-type byte and the level.
const TS_PENDING_BUFFER: u32 = 0x08;
const TS_PENDING: u32 = 0x14;
const TS_DATA_TYPE: u32 = 0x1c;
const TS_LEVEL: u32 = 0x7c;
/// The literal/length, distance and bit-length trees (4-byte entries: frequency or code
/// `u16` at +0, parent or length `u16` at +2).
const TS_LITERAL_TREE: u32 = 0x8c;
const TS_DISTANCE_TREE: u32 = 0x980;
const TS_BIT_LENGTH_TREE: u32 = 0xa74;
/// The three tree descriptors (tree, largest code, static descriptor).
const TS_LITERAL_DESCRIPTOR: u32 = 0xb10;
const TS_DISTANCE_DESCRIPTOR: u32 = 0xb1c;
const TS_BIT_LENGTH_DESCRIPTOR: u32 = 0xb28;
/// The bit-length counts (16 `u16`), the heap (573 words) with its length and top, and
/// the depths (573 bytes).
const TS_BIT_LENGTH_COUNT: u32 = 0xb34;
const TS_HEAP: u32 = 0xb54;
const TS_HEAP_LENGTH: u32 = 0x1448;
const TS_HEAP_MAX: u32 = 0x144c;
const TS_DEPTH: u32 = 0x1450;
/// The literal buffer and its size, the literal count, the distance buffer, the optimal
/// and static lengths, the match count, the last end-of-block length and the bit buffer.
const TS_LITERAL_BUFFER: u32 = 0x1690;
const TS_LITERAL_BUFFER_SIZE: u32 = 0x1694;
const TS_LAST_LITERAL: u32 = 0x1698;
const TS_DISTANCE_BUFFER: u32 = 0x169c;
const TS_OPTIMAL_LENGTH: u32 = 0x16a0;
const TS_STATIC_LENGTH: u32 = 0x16a4;
const TS_MATCHES: u32 = 0x16a8;
const TS_LAST_END_LENGTH: u32 = 0x16ac;
const TS_BIT_BUFFER: u32 = 0x16b0;
const TS_BIT_COUNT: u32 = 0x16b4;
/// The heap's size (`2 * 286 + 1`).
const HEAP_SIZE: i32 = 0x23d;
/// The static tree descriptors (`.data`) and trees, the bit-length order, the length and
/// distance code tables and their extra bits and bases (`.rdata`).
const STATIC_LITERAL_DESCRIPTOR: u32 = 0x011a_d770;
const STATIC_DISTANCE_DESCRIPTOR: u32 = 0x011a_d784;
const STATIC_BIT_LENGTH_DESCRIPTOR: u32 = 0x011a_d798;
const STATIC_LITERAL_TREE: u32 = 0x010a_86a0;
const STATIC_DISTANCE_TREE: u32 = 0x010a_8b20;
const BIT_LENGTH_ORDER: u32 = 0x010a_85c4;
const LENGTH_CODE: u32 = 0x010a_8d98;
const DISTANCE_CODE: u32 = 0x010a_8b98;
const LENGTH_EXTRA_BITS: u32 = 0x010a_8550;
const LENGTH_BASES: u32 = 0x010a_8e98;
const DISTANCE_EXTRA_BITS: u32 = 0x010a_85d8;
const DISTANCE_BASES: u32 = 0x010a_8f10;
/// The compressor's `deflateInit_` (`00b46360`), `deflate` (`00b46b70`) and `deflateEnd`
/// (`00b47370`), all in `run_00b45c00.rs`; the version string `compress2` passes
/// (`011ad7ac`).
const DEFLATE_INIT: u32 = 0x00b4_6360;
const DEFLATE: u32 = 0x00b4_6b70;
const DEFLATE_END: u32 = 0x00b4_7370;
const COMPRESS_VERSION: u32 = 0x011a_d7ac;

/// A pending byte.
fn tree_put_byte(e: &mut Engine, state: u32, byte: u32) {
    let buffer = e.mem.u32(state + TS_PENDING_BUFFER);
    let pending = e.mem.u32(state + TS_PENDING);
    e.mem.set_u8(buffer.wrapping_add(pending), byte as u8);
    e.mem.set_u32(state + TS_PENDING, pending.wrapping_add(1));
}

/// The exe's inlined bit writer: with more bits held than fit with `length` more, the
/// value shifted to the held count fills the 16-bit buffer, both its bytes go out and the
/// buffer keeps the value's (16-bit) rest; otherwise the value is only added and counted.
fn send_bits(e: &mut Engine, state: u32, value: u32, length: i32) {
    let held = e.mem.i32(state + TS_BIT_COUNT);
    if held > 16i32.wrapping_sub(length) {
        let buffer = u32::from(e.mem.u16(state + TS_BIT_BUFFER)) | value.wrapping_shl(held as u32);
        e.mem.set_u16(state + TS_BIT_BUFFER, buffer as u16);
        let buffer = u32::from(e.mem.u16(state + TS_BIT_BUFFER));
        tree_put_byte(e, state, buffer & 0xff);
        let buffer = u32::from(e.mem.u16(state + TS_BIT_BUFFER));
        tree_put_byte(e, state, buffer >> 8);
        let shift = 16i32.wrapping_sub(e.mem.i32(state + TS_BIT_COUNT)) as u32;
        let rest = (value & 0xffff).wrapping_shr(shift);
        e.mem.set_u16(state + TS_BIT_BUFFER, rest as u16);
        let held = e.mem.i32(state + TS_BIT_COUNT);
        e.mem.set_i32(
            state + TS_BIT_COUNT,
            length.wrapping_add(held).wrapping_sub(16),
        );
    } else {
        let buffer = u32::from(e.mem.u16(state + TS_BIT_BUFFER)) | value.wrapping_shl(held as u32);
        e.mem.set_u16(state + TS_BIT_BUFFER, buffer as u16);
        let held = e.mem.i32(state + TS_BIT_COUNT);
        e.mem
            .set_i32(state + TS_BIT_COUNT, held.wrapping_add(length));
    }
}

/// The exe's inlined code writer: the tree entry's code with its length (read first).
fn send_code(e: &mut Engine, state: u32, tree: u32, code: u32) {
    let entry = tree.wrapping_add(code.wrapping_mul(4));
    let length = i32::from(e.mem.u16(entry.wrapping_add(2)));
    let value = u32::from(e.mem.u16(entry));
    send_bits(e, state, value, length);
}

// Translated from 00b4a060 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map; zlib's `get_crc_table` by its body (no words): the address of
/// the CRC table `010a6550`.
pub fn fn_00b4a060(_e: &mut Engine) -> u32 {
    CRC_TABLE
}

// Translated from 00b4ab70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map; zlib's `tr_static_init` with the static trees built in (no
/// words): does nothing.
pub fn fn_00b4ab70(_e: &mut Engine) {}

// Translated from 00b4ab80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map; zlib's `init_block` by its body (`cdecl`, one word: the
/// compressor state): clears the frequencies of the 286 literal/length, 30 distance and 19
/// bit-length codes, counts the end-of-block code once, and clears the optimal and static
/// lengths, the match and the literal counts.
pub fn fn_00b4ab80(e: &mut Engine, state: u32) {
    for (tree, count) in [
        (TS_LITERAL_TREE, 0x11eu32),
        (TS_DISTANCE_TREE, 0x1e),
        (TS_BIT_LENGTH_TREE, 0x13),
    ] {
        for code in 0..count {
            e.mem.set_u16(state + tree + 4 * code, 0);
        }
    }
    e.mem.set_u16(state + TS_LITERAL_TREE + 4 * 0x100, 1);
    e.mem.set_u32(state + TS_STATIC_LENGTH, 0);
    e.mem.set_u32(state + TS_OPTIMAL_LENGTH, 0);
    e.mem.set_u32(state + TS_MATCHES, 0);
    e.mem.set_u32(state + TS_LAST_LITERAL, 0);
}

// Translated from 00b4aad0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map; zlib's `_tr_init` by its body (`cdecl`, one word: the
/// compressor state): `fn_00b4ab70`, the three descriptors pointed at their trees and the
/// static descriptors (`011ad770`, `011ad784`, `011ad798`), the bit buffer emptied, the
/// last end-of-block length 8, and `fn_00b4ab80`.
pub fn fn_00b4aad0(e: &mut Engine, state: u32) {
    fn_00b4ab70(e);
    for (descriptor, tree, statics) in [
        (
            TS_LITERAL_DESCRIPTOR,
            TS_LITERAL_TREE,
            STATIC_LITERAL_DESCRIPTOR,
        ),
        (
            TS_DISTANCE_DESCRIPTOR,
            TS_DISTANCE_TREE,
            STATIC_DISTANCE_DESCRIPTOR,
        ),
        (
            TS_BIT_LENGTH_DESCRIPTOR,
            TS_BIT_LENGTH_TREE,
            STATIC_BIT_LENGTH_DESCRIPTOR,
        ),
    ] {
        e.mem.set_u32(state + descriptor, state + tree);
        e.mem.set_u32(state + descriptor + 8, statics);
    }
    e.mem.set_u16(state + TS_BIT_BUFFER, 0);
    e.mem.set_u32(state + TS_BIT_COUNT, 0);
    e.mem.set_u32(state + TS_LAST_END_LENGTH, 8);
    fn_00b4ab80(e, state);
}

// Translated from 00b4ac50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map; zlib's `_tr_stored_block` by its body (`cdecl`, four words:
/// the compressor state, a buffer, its length and the last-block flag): the stored block
/// type (`flag` in 3 bits), then `fn_00b4dca0` with the header.
pub fn fn_00b4ac50(e: &mut Engine, state: u32, buffer: u32, length: u32, last: u32) {
    send_bits(e, state, last, 3);
    fn_00b4dca0(e, state, buffer, length, 1);
}

// Translated from 00b4ad90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map; zlib's `_tr_align` by its body (`cdecl`, one word: the
/// compressor state): a static block type (2 in 3 bits) and the static end-of-block code
/// (0 in 7 bits), then `fn_00b4daf0`; when the last end-of-block length plus 11 less the
/// bits held is below 9, the same pair and flush again. The last end-of-block length
/// becomes 7.
pub fn fn_00b4ad90(e: &mut Engine, state: u32) {
    send_bits(e, state, 2, 3);
    send_bits(e, state, 0, 7);
    fn_00b4daf0(e, state);
    let room = e
        .mem
        .i32(state + TS_LAST_END_LENGTH)
        .wrapping_add(0xb)
        .wrapping_sub(e.mem.i32(state + TS_BIT_COUNT));
    if room < 9 {
        send_bits(e, state, 2, 3);
        send_bits(e, state, 0, 7);
        fn_00b4daf0(e, state);
    }
    e.mem.set_u32(state + TS_LAST_END_LENGTH, 7);
}

// Translated from 00b4b250 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map; zlib's `_tr_flush_block` by its body (`cdecl`, four words:
/// the compressor state, the block's bytes or null, its length and the last-block flag).
/// At a level above 0: the data type is guessed (`fn_00b4da00`) while unknown (2), both
/// trees are built (`fn_00b4b600`) and the bit-length tree (`fn_00b4bf10`); the optimal
/// and static byte counts are `(bits + 10) / 8` and the smaller counts. At level 0 both
/// are the length plus 5. With bytes and a length plus 4 within that count the block is
/// stored (`fn_00b4ac50`); else a static block (type `2 + last`, `fn_00b4d210` with the
/// static trees) when that is as small, otherwise a dynamic one (type `4 + last`, the
/// trees sent by `fn_00b4c170`, the data by `fn_00b4d210`). Then `fn_00b4ab80`, and for the
/// last block `fn_00b4dbe0`.
pub fn fn_00b4b250(e: &mut Engine, state: u32, buffer: u32, length: u32, last: u32) {
    let mut max_index = 0i32;
    let (optimal, fixed);
    if e.mem.i32(state + TS_LEVEL) > 0 {
        if e.mem.u8(state + TS_DATA_TYPE) == 2 {
            fn_00b4da00(e, state);
        }
        fn_00b4b600(e, state, state + TS_LITERAL_DESCRIPTOR);
        fn_00b4b600(e, state, state + TS_DISTANCE_DESCRIPTOR);
        max_index = fn_00b4bf10(e, state);
        let mut best = e.mem.u32(state + TS_OPTIMAL_LENGTH).wrapping_add(0xa) >> 3;
        let static_bytes = e.mem.u32(state + TS_STATIC_LENGTH).wrapping_add(0xa) >> 3;
        if static_bytes <= best {
            best = static_bytes;
        }
        optimal = best;
        fixed = static_bytes;
    } else {
        fixed = length.wrapping_add(5);
        optimal = fixed;
    }
    if length.wrapping_add(4) <= optimal && buffer != 0 {
        fn_00b4ac50(e, state, buffer, length, last);
    } else if fixed == optimal {
        send_bits(e, state, last.wrapping_add(2), 3);
        fn_00b4d210(e, state, STATIC_LITERAL_TREE, STATIC_DISTANCE_TREE);
    } else {
        send_bits(e, state, last.wrapping_add(4), 3);
        let literal_codes = e.mem.i32(state + TS_LITERAL_DESCRIPTOR + 4).wrapping_add(1);
        let distance_codes = e
            .mem
            .i32(state + TS_DISTANCE_DESCRIPTOR + 4)
            .wrapping_add(1);
        fn_00b4c170(
            e,
            state,
            literal_codes,
            distance_codes,
            max_index.wrapping_add(1),
        );
        fn_00b4d210(e, state, state + TS_LITERAL_TREE, state + TS_DISTANCE_TREE);
    }
    fn_00b4ab80(e, state);
    if last != 0 {
        fn_00b4dbe0(e, state);
    }
}

/// The heap entry at `index`.
fn heap_at(e: &Engine, state: u32, index: i32) -> i32 {
    e.mem.i32(state + TS_HEAP + (index as u32).wrapping_mul(4))
}

fn set_heap_at(e: &mut Engine, state: u32, index: i32, value: i32) {
    e.mem
        .set_i32(state + TS_HEAP + (index as u32).wrapping_mul(4), value);
}

/// A node's frequency and depth.
fn frequency(e: &Engine, tree: u32, node: i32) -> u32 {
    u32::from(e.mem.u16(tree.wrapping_add((node as u32).wrapping_mul(4))))
}

fn depth(e: &Engine, state: u32, node: i32) -> u32 {
    u32::from(e.mem.u8(state + TS_DEPTH + node as u32))
}

/// The heap order: a smaller frequency, or an equal one at no greater depth.
fn smaller(e: &Engine, state: u32, tree: u32, n: i32, m: i32) -> bool {
    frequency(e, tree, n) < frequency(e, tree, m)
        || (frequency(e, tree, n) == frequency(e, tree, m)
            && depth(e, state, n) <= depth(e, state, m))
}

// Translated from 00b4b9a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map; zlib's `pqdownheap` by its body (`cdecl`, three words: the
/// compressor state, a tree and a heap index): sifts the entry at the index down the heap,
/// swapping with the smaller child while that is smaller.
pub fn fn_00b4b9a0(e: &mut Engine, state: u32, tree: u32, index: i32) {
    let mut k = index;
    let value = heap_at(e, state, k);
    let mut j = k.wrapping_shl(1);
    while j <= e.mem.i32(state + TS_HEAP_LENGTH) {
        if j < e.mem.i32(state + TS_HEAP_LENGTH)
            && smaller(
                e,
                state,
                tree,
                heap_at(e, state, j + 1),
                heap_at(e, state, j),
            )
        {
            j += 1;
        }
        if smaller(e, state, tree, value, heap_at(e, state, j)) {
            break;
        }
        let child = heap_at(e, state, j);
        set_heap_at(e, state, k, child);
        k = j;
        j = j.wrapping_shl(1);
    }
    set_heap_at(e, state, k, value);
}

// Translated from 00b4b600 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map; zlib's `build_tree` by its body (`cdecl`, two words: the
/// compressor state and a tree descriptor). Puts every used code on the heap (unused ones
/// get length 0), forces at least two (frequency 1, lowering the optimal and, with a static
/// tree, the static length), keeps the largest used code, heapifies (`fn_00b4b9a0`), and
/// combines the two smallest nodes into new ones (from the element count up, depth one
/// more than the deeper) until one is left, the nodes saved at the heap's top. Then the
/// lengths (`fn_00b4bb30`) and codes (`fn_00b4be40`).
pub fn fn_00b4b600(e: &mut Engine, state: u32, descriptor: u32) {
    let tree = e.mem.u32(descriptor);
    let statics = e.mem.u32(descriptor + 8);
    let static_tree = e.mem.u32(statics);
    let elements = e.mem.i32(statics + 0xc);
    let mut max_code = -1i32;
    e.mem.set_i32(state + TS_HEAP_LENGTH, 0);
    e.mem.set_i32(state + TS_HEAP_MAX, HEAP_SIZE);
    let mut n = 0i32;
    while n < elements {
        if frequency(e, tree, n) != 0 {
            max_code = n;
            let length = e.mem.i32(state + TS_HEAP_LENGTH).wrapping_add(1);
            e.mem.set_i32(state + TS_HEAP_LENGTH, length);
            set_heap_at(e, state, length, max_code);
            e.mem.set_u8(state + TS_DEPTH + n as u32, 0);
        } else {
            e.mem.set_u16(
                tree.wrapping_add((n as u32).wrapping_mul(4))
                    .wrapping_add(2),
                0,
            );
        }
        n += 1;
    }
    while e.mem.i32(state + TS_HEAP_LENGTH) < 2 {
        let node = if max_code < 2 {
            max_code += 1;
            max_code
        } else {
            0
        };
        let length = e.mem.i32(state + TS_HEAP_LENGTH).wrapping_add(1);
        e.mem.set_i32(state + TS_HEAP_LENGTH, length);
        set_heap_at(e, state, length, node);
        e.mem
            .set_u16(tree.wrapping_add((node as u32).wrapping_mul(4)), 1);
        e.mem.set_u8(state + TS_DEPTH + node as u32, 0);
        let optimal = e.mem.u32(state + TS_OPTIMAL_LENGTH).wrapping_sub(1);
        e.mem.set_u32(state + TS_OPTIMAL_LENGTH, optimal);
        if static_tree != 0 {
            let static_length = u32::from(
                e.mem.u16(
                    static_tree
                        .wrapping_add((node as u32).wrapping_mul(4))
                        .wrapping_add(2),
                ),
            );
            let value = e
                .mem
                .u32(state + TS_STATIC_LENGTH)
                .wrapping_sub(static_length);
            e.mem.set_u32(state + TS_STATIC_LENGTH, value);
        }
    }
    e.mem.set_i32(descriptor + 4, max_code);
    let mut n = e.mem.i32(state + TS_HEAP_LENGTH) / 2;
    while n >= 1 {
        fn_00b4b9a0(e, state, tree, n);
        n -= 1;
    }
    let mut node = elements;
    loop {
        let n = heap_at(e, state, 1);
        let last = heap_at(e, state, e.mem.i32(state + TS_HEAP_LENGTH));
        set_heap_at(e, state, 1, last);
        let length = e.mem.i32(state + TS_HEAP_LENGTH).wrapping_sub(1);
        e.mem.set_i32(state + TS_HEAP_LENGTH, length);
        fn_00b4b9a0(e, state, tree, 1);
        let m = heap_at(e, state, 1);
        let top = e.mem.i32(state + TS_HEAP_MAX).wrapping_sub(1);
        e.mem.set_i32(state + TS_HEAP_MAX, top);
        set_heap_at(e, state, top, n);
        let top = e.mem.i32(state + TS_HEAP_MAX).wrapping_sub(1);
        e.mem.set_i32(state + TS_HEAP_MAX, top);
        set_heap_at(e, state, top, m);
        let sum = frequency(e, tree, n).wrapping_add(frequency(e, tree, m));
        e.mem
            .set_u16(tree.wrapping_add((node as u32).wrapping_mul(4)), sum as u16);
        let deeper = if depth(e, state, n) >= depth(e, state, m) {
            depth(e, state, n)
        } else {
            depth(e, state, m)
        };
        e.mem
            .set_u8(state + TS_DEPTH + node as u32, deeper.wrapping_add(1) as u8);
        for child in [m, n] {
            e.mem.set_u16(
                tree.wrapping_add((child as u32).wrapping_mul(4))
                    .wrapping_add(2),
                node as u16,
            );
        }
        set_heap_at(e, state, 1, node);
        node += 1;
        fn_00b4b9a0(e, state, tree, 1);
        if e.mem.i32(state + TS_HEAP_LENGTH) < 2 {
            break;
        }
    }
    let top = e.mem.i32(state + TS_HEAP_MAX).wrapping_sub(1);
    e.mem.set_i32(state + TS_HEAP_MAX, top);
    let root = heap_at(e, state, 1);
    set_heap_at(e, state, top, root);
    fn_00b4bb30(e, state, descriptor);
    fn_00b4be40(e, tree, max_code, state + TS_BIT_LENGTH_COUNT);
}

// Translated from 00b4bb30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map; zlib's `gen_bitlen` by its body (`cdecl`, two words: the
/// compressor state and a tree descriptor). Clears the bit-length counts; walks the heap
/// from its top giving each node its parent's length plus one (at most the descriptor's
/// limit, counting the overflows), and for each code (not above the largest used one)
/// counts its length and adds `frequency * (length + extra bits)` to the optimal length
/// (and the static tree's length to the static one). With overflows it moves counts down
/// from the limit (one less there, two more one level deeper) until none are left, then
/// gives the codes, from the heap's end, the corrected lengths, the optimal length taking
/// the differences.
pub fn fn_00b4bb30(e: &mut Engine, state: u32, descriptor: u32) {
    let tree = e.mem.u32(descriptor);
    let max_code = e.mem.i32(descriptor + 4);
    let statics = e.mem.u32(descriptor + 8);
    let static_tree = e.mem.u32(statics);
    let extra = e.mem.u32(statics + 4);
    let base = e.mem.i32(statics + 8);
    let max_length = e.mem.i32(statics + 0x10);
    let length_of = |e: &Engine, node: u32| {
        u32::from(
            e.mem
                .u16(tree.wrapping_add(node.wrapping_mul(4)).wrapping_add(2)),
        )
    };
    let count_at = |bits: i32| state + TS_BIT_LENGTH_COUNT + (bits as u32).wrapping_mul(2);
    let mut overflow = 0i32;
    let mut bits = 0i32;
    while bits <= 0xf {
        e.mem.set_u16(count_at(bits), 0);
        bits += 1;
    }
    let root = heap_at(e, state, e.mem.i32(state + TS_HEAP_MAX)) as u32;
    e.mem
        .set_u16(tree.wrapping_add(root.wrapping_mul(4)).wrapping_add(2), 0);
    let mut h = e.mem.i32(state + TS_HEAP_MAX).wrapping_add(1);
    while h < HEAP_SIZE {
        let n = heap_at(e, state, h) as u32;
        let parent = length_of(e, n);
        let mut length = length_of(e, parent) as i32 + 1;
        if length > max_length {
            length = max_length;
            overflow += 1;
        }
        e.mem.set_u16(
            tree.wrapping_add(n.wrapping_mul(4)).wrapping_add(2),
            length as u16,
        );
        if (n as i32) <= max_code {
            let count = e.mem.u16(count_at(length)).wrapping_add(1);
            e.mem.set_u16(count_at(length), count);
            let mut extra_bits = 0i32;
            if (n as i32) >= base {
                extra_bits = e
                    .mem
                    .i32(extra.wrapping_add((n as i32).wrapping_sub(base) as u32 * 4));
            }
            let f = frequency(e, tree, n as i32);
            let optimal = f
                .wrapping_mul(length.wrapping_add(extra_bits) as u32)
                .wrapping_add(e.mem.u32(state + TS_OPTIMAL_LENGTH));
            e.mem.set_u32(state + TS_OPTIMAL_LENGTH, optimal);
            if static_tree != 0 {
                let static_length = u32::from(
                    e.mem
                        .u16(static_tree.wrapping_add(n.wrapping_mul(4)).wrapping_add(2)),
                );
                let value = f
                    .wrapping_mul(static_length.wrapping_add(extra_bits as u32))
                    .wrapping_add(e.mem.u32(state + TS_STATIC_LENGTH));
                e.mem.set_u32(state + TS_STATIC_LENGTH, value);
            }
        }
        h += 1;
    }
    if overflow == 0 {
        return;
    }
    loop {
        let mut bits = max_length - 1;
        while e.mem.u16(count_at(bits)) == 0 {
            bits -= 1;
        }
        let count = e.mem.u16(count_at(bits)).wrapping_sub(1);
        e.mem.set_u16(count_at(bits), count);
        let count = e.mem.u16(count_at(bits + 1)).wrapping_add(2);
        e.mem.set_u16(count_at(bits + 1), count);
        let count = e.mem.u16(count_at(max_length)).wrapping_sub(1);
        e.mem.set_u16(count_at(max_length), count);
        overflow -= 2;
        if overflow <= 0 {
            break;
        }
    }
    let mut bits = max_length;
    while bits != 0 {
        let mut n = u32::from(e.mem.u16(count_at(bits)));
        while n != 0 {
            h -= 1;
            let m = heap_at(e, state, h);
            if m > max_code {
                continue;
            }
            let length = length_of(e, m as u32);
            if length != bits as u32 {
                let optimal = (bits as u32)
                    .wrapping_sub(length)
                    .wrapping_mul(frequency(e, tree, m))
                    .wrapping_add(e.mem.u32(state + TS_OPTIMAL_LENGTH));
                e.mem.set_u32(state + TS_OPTIMAL_LENGTH, optimal);
                e.mem.set_u16(
                    tree.wrapping_add((m as u32).wrapping_mul(4))
                        .wrapping_add(2),
                    bits as u16,
                );
            }
            n -= 1;
        }
        bits -= 1;
    }
}

// Translated from 00b4be40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map; zlib's `gen_codes` by its body (`cdecl`, three words: a
/// tree, its largest code and the bit-length counts): the first code of each length
/// (`(previous + count) * 2`, 16-bit) into a 16-entry local, then each code with a length
/// takes the next code of its length reversed (`fn_00b4dab0`).
// Translation incomplete: 00b4be40 - the /GS stack cookie (011c16bc, checked by 00ec408c) is not translated.
pub fn fn_00b4be40(e: &mut Engine, tree: u32, max_code: i32, counts: u32) {
    e.with_stack(0x20, |e, next| {
        let next = next.addr();
        let mut code = 0u16;
        let mut bits = 1u32;
        while bits <= 0xf {
            let count = e.mem.u16(counts.wrapping_add(bits * 2).wrapping_sub(2));
            code = code.wrapping_add(count).wrapping_shl(1);
            e.mem.set_u16(next + bits * 2, code);
            bits += 1;
        }
        let mut n = 0i32;
        while n <= max_code {
            let entry = tree.wrapping_add((n as u32).wrapping_mul(4));
            let length = u32::from(e.mem.u16(entry.wrapping_add(2)));
            if length != 0 {
                let slot = next.wrapping_add(length.wrapping_mul(2));
                let value = u32::from(e.mem.u16(slot));
                let advanced = e.mem.u16(slot).wrapping_add(1);
                e.mem.set_u16(slot, advanced);
                let reversed = fn_00b4dab0(e, value, length as i32);
                e.mem.set_u16(entry, reversed as u16);
            }
            n += 1;
        }
    });
}

// Translated from 00b4bf10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map; zlib's `build_bl_tree` by its body (`cdecl`, one word: the
/// compressor state): scans both trees (`fn_00b4bfd0`), builds the bit-length tree, finds
/// the last bit-length code in transmit order (`010a85c4`, from 18 down to 3) with a
/// length, adds `3 * (index + 1) + 14` to the optimal length and answers the index.
pub fn fn_00b4bf10(e: &mut Engine, state: u32) -> i32 {
    let max = e.mem.i32(state + TS_LITERAL_DESCRIPTOR + 4);
    fn_00b4bfd0(e, state, state + TS_LITERAL_TREE, max);
    let max = e.mem.i32(state + TS_DISTANCE_DESCRIPTOR + 4);
    fn_00b4bfd0(e, state, state + TS_DISTANCE_TREE, max);
    fn_00b4b600(e, state, state + TS_BIT_LENGTH_DESCRIPTOR);
    let mut index = 0x12i32;
    while index >= 3 {
        let code = u32::from(e.mem.u8(BIT_LENGTH_ORDER + index as u32));
        if e.mem.u16(state + TS_BIT_LENGTH_TREE + 2 + code * 4) != 0 {
            break;
        }
        index -= 1;
    }
    let optimal = (index.wrapping_add(1).wrapping_mul(3) as u32)
        .wrapping_add(e.mem.u32(state + TS_OPTIMAL_LENGTH))
        .wrapping_add(0xe);
    e.mem.set_u32(state + TS_OPTIMAL_LENGTH, optimal);
    index
}

/// The run limits `fn_00b4bfd0` and `fn_00b4c670` switch between after a run: 138/3 before
/// a zero length, 6/3 for a repeated length, 7/4 otherwise.
fn run_limits(current: u32, next: u32) -> (i32, i32) {
    if next == 0 {
        (0x8a, 3)
    } else if current == next {
        (6, 3)
    } else {
        (7, 4)
    }
}

// Translated from 00b4bfd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map; zlib's `scan_tree` by its body (`cdecl`, three words: the
/// compressor state, a tree and its largest code). Sets the length after the largest code
/// to 0xffff (a guard) and counts the bit-length codes the run-length coding of the lengths
/// needs: short runs count their length that often, zero runs code 17 (up to 10) or 18,
/// other runs code 16 (after the length itself when it changed).
pub fn fn_00b4bfd0(e: &mut Engine, state: u32, tree: u32, max_code: i32) {
    let length_at = |e: &Engine, n: i32| {
        u32::from(
            e.mem.u16(
                tree.wrapping_add((n as u32).wrapping_mul(4))
                    .wrapping_add(2),
            ),
        )
    };
    let bl_frequency = |code: u32| state + TS_BIT_LENGTH_TREE + code * 4;
    let mut previous = u32::MAX;
    let first = length_at(e, 0);
    let mut count = 0i32;
    let (mut max_count, mut min_count) = (7i32, 4i32);
    if first == 0 {
        max_count = 0x8a;
        min_count = 3;
    }
    e.mem.set_u16(
        tree.wrapping_add((max_code as u32).wrapping_mul(4))
            .wrapping_add(6),
        0xffff,
    );
    let mut current = first;
    let mut n = 0i32;
    while n <= max_code {
        let next = length_at(e, n + 1);
        count += 1;
        if count < max_count && current == next {
            current = next;
            n += 1;
            continue;
        }
        if count < min_count {
            let value = e.mem.u16(bl_frequency(current)).wrapping_add(count as u16);
            e.mem.set_u16(bl_frequency(current), value);
        } else if current != 0 {
            if current != previous {
                let value = e.mem.u16(bl_frequency(current)).wrapping_add(1);
                e.mem.set_u16(bl_frequency(current), value);
            }
            let value = e.mem.u16(bl_frequency(16)).wrapping_add(1);
            e.mem.set_u16(bl_frequency(16), value);
        } else if count < 0xb {
            let value = e.mem.u16(bl_frequency(17)).wrapping_add(1);
            e.mem.set_u16(bl_frequency(17), value);
        } else {
            let value = e.mem.u16(bl_frequency(18)).wrapping_add(1);
            e.mem.set_u16(bl_frequency(18), value);
        }
        count = 0;
        previous = current;
        (max_count, min_count) = run_limits(current, next);
        current = next;
        n += 1;
    }
}

// Translated from 00b4c670 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map; zlib's `send_tree` by its body (`cdecl`, three words: the
/// compressor state, a tree and its largest code): sends the tree's lengths run-length
/// coded with the bit-length tree's codes: short runs as the length repeated, other runs
/// as code 16 (after the length when it changed) and `count - 3` in 2 bits, zero runs as
/// code 17 and `count - 3` in 3 bits (up to 10) or code 18 and `count - 11` in 7 bits.
pub fn fn_00b4c670(e: &mut Engine, state: u32, tree: u32, max_code: i32) {
    let length_at = |e: &Engine, n: i32| {
        u32::from(
            e.mem.u16(
                tree.wrapping_add((n as u32).wrapping_mul(4))
                    .wrapping_add(2),
            ),
        )
    };
    let bl_tree = state + TS_BIT_LENGTH_TREE;
    let mut previous = u32::MAX;
    let first = length_at(e, 0);
    let mut count = 0i32;
    let (mut max_count, mut min_count) = (7i32, 4i32);
    if first == 0 {
        max_count = 0x8a;
        min_count = 3;
    }
    let mut current = first;
    let mut n = 0i32;
    while n <= max_code {
        let next = length_at(e, n + 1);
        count += 1;
        if count < max_count && current == next {
            current = next;
            n += 1;
            continue;
        }
        if count < min_count {
            loop {
                send_code(e, state, bl_tree, current);
                count -= 1;
                if count == 0 {
                    break;
                }
            }
        } else if current != 0 {
            if current != previous {
                send_code(e, state, bl_tree, current);
                count -= 1;
            }
            send_code(e, state, bl_tree, 16);
            send_bits(e, state, count.wrapping_sub(3) as u32, 2);
        } else if count < 0xb {
            send_code(e, state, bl_tree, 17);
            send_bits(e, state, count.wrapping_sub(3) as u32, 3);
        } else {
            send_code(e, state, bl_tree, 18);
            send_bits(e, state, count.wrapping_sub(0xb) as u32, 7);
        }
        count = 0;
        previous = current;
        (max_count, min_count) = run_limits(current, next);
        current = next;
        n += 1;
    }
}

// Translated from 00b4c170 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map; zlib's `send_all_trees` by its body (`cdecl`, four words: the
/// compressor state and the literal, distance and bit-length code counts): the counts less
/// 257, 1 and 4 (5, 5 and 4 bits), the bit-length codes' lengths in transmit order (3 bits
/// each), then both trees (`fn_00b4c670`).
pub fn fn_00b4c170(
    e: &mut Engine,
    state: u32,
    literal_codes: i32,
    distance_codes: i32,
    bl_codes: i32,
) {
    send_bits(e, state, literal_codes.wrapping_sub(0x101) as u32, 5);
    send_bits(e, state, distance_codes.wrapping_sub(1) as u32, 5);
    send_bits(e, state, bl_codes.wrapping_sub(4) as u32, 4);
    let mut rank = 0i32;
    while rank < bl_codes {
        let code = u32::from(e.mem.u8(BIT_LENGTH_ORDER + rank as u32));
        let length = u32::from(e.mem.u16(state + TS_BIT_LENGTH_TREE + 2 + code * 4));
        send_bits(e, state, length, 3);
        rank += 1;
    }
    fn_00b4c670(
        e,
        state,
        state + TS_LITERAL_TREE,
        literal_codes.wrapping_sub(1),
    );
    fn_00b4c670(
        e,
        state,
        state + TS_DISTANCE_TREE,
        distance_codes.wrapping_sub(1),
    );
}

/// The distance code of a (0-based) distance.
fn distance_code(e: &Engine, distance: u32) -> u32 {
    if distance < 0x100 {
        u32::from(e.mem.u8(DISTANCE_CODE + distance))
    } else {
        u32::from(e.mem.u8(DISTANCE_CODE + 0x100 + (distance >> 7)))
    }
}

// Translated from 00b4d0e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map; zlib's `_tr_tally` by its body (`cdecl`, three words: the
/// compressor state, a distance (0 for a literal) and the literal or length less 3): both
/// into the buffers; a literal's frequency counted, or for a match the match count, the
/// length code's and the distance code's frequencies. Answers whether the literal buffer
/// is full (one short of its size).
pub fn fn_00b4d0e0(e: &mut Engine, state: u32, distance: u32, value: u32) -> bool {
    let last = e.mem.u32(state + TS_LAST_LITERAL);
    let distances = e.mem.u32(state + TS_DISTANCE_BUFFER);
    e.mem.set_u16(
        distances.wrapping_add(last.wrapping_mul(2)),
        distance as u16,
    );
    let literals = e.mem.u32(state + TS_LITERAL_BUFFER);
    let last = e.mem.u32(state + TS_LAST_LITERAL);
    e.mem.set_u8(literals.wrapping_add(last), value as u8);
    let last = e.mem.u32(state + TS_LAST_LITERAL).wrapping_add(1);
    e.mem.set_u32(state + TS_LAST_LITERAL, last);
    if distance == 0 {
        let at = state + TS_LITERAL_TREE + value.wrapping_mul(4);
        e.mem.set_u16(at, e.mem.u16(at).wrapping_add(1));
    } else {
        let matches = e.mem.u32(state + TS_MATCHES).wrapping_add(1);
        e.mem.set_u32(state + TS_MATCHES, matches);
        let code = u32::from(e.mem.u8(LENGTH_CODE.wrapping_add(value)));
        let at = state + TS_LITERAL_TREE + (code + 0x101) * 4;
        e.mem.set_u16(at, e.mem.u16(at).wrapping_add(1));
        let code = distance_code(e, distance.wrapping_sub(1));
        let at = state + TS_DISTANCE_TREE + code * 4;
        e.mem.set_u16(at, e.mem.u16(at).wrapping_add(1));
    }
    e.mem.u32(state + TS_LAST_LITERAL) == e.mem.u32(state + TS_LITERAL_BUFFER_SIZE).wrapping_sub(1)
}

// Translated from 00b4d210 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map; zlib's `compress_block` by its body (`cdecl`, three words: the
/// compressor state, the literal/length tree and the distance tree): sends each buffered
/// literal as its code, each match as its length code with the length's extra bits
/// (`010a8550`/`010a8e98`) and its distance code with the distance's extra bits
/// (`010a85d8`/`010a8f10`); then the end-of-block code, whose length becomes the last
/// end-of-block length.
pub fn fn_00b4d210(e: &mut Engine, state: u32, literal_tree: u32, distance_tree: u32) {
    let mut index = 0u32;
    if e.mem.u32(state + TS_LAST_LITERAL) != 0 {
        loop {
            let distances = e.mem.u32(state + TS_DISTANCE_BUFFER);
            let mut distance = u32::from(e.mem.u16(distances.wrapping_add(index * 2)));
            let literals = e.mem.u32(state + TS_LITERAL_BUFFER);
            let mut value = u32::from(e.mem.u8(literals.wrapping_add(index)));
            index += 1;
            if distance == 0 {
                send_code(e, state, literal_tree, value);
            } else {
                let code = u32::from(e.mem.u8(LENGTH_CODE.wrapping_add(value)));
                send_code(e, state, literal_tree, code + 0x101);
                let extra = e.mem.i32(LENGTH_EXTRA_BITS + code * 4);
                if extra != 0 {
                    value = value.wrapping_sub(e.mem.u32(LENGTH_BASES + code * 4));
                    send_bits(e, state, value, extra);
                }
                distance = distance.wrapping_sub(1);
                let code = distance_code(e, distance);
                send_code(e, state, distance_tree, code);
                let extra = e.mem.i32(DISTANCE_EXTRA_BITS + code * 4);
                if extra != 0 {
                    distance = distance.wrapping_sub(e.mem.u32(DISTANCE_BASES + code * 4));
                    send_bits(e, state, distance, extra);
                }
            }
            if index >= e.mem.u32(state + TS_LAST_LITERAL) {
                break;
            }
        }
    }
    send_code(e, state, literal_tree, 0x100);
    let length = u32::from(e.mem.u16(literal_tree + 0x100 * 4 + 2));
    e.mem.set_u32(state + TS_LAST_END_LENGTH, length);
}

// Translated from 00b4da00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map; zlib's `set_data_type` by its body (`cdecl`, one word: the
/// compressor state): sums the literal frequencies of bytes 0..7 and 128..256 (binary) and
/// 7..128 (text); the data-type byte becomes 0 when the binary sum exceeds a quarter of
/// the text sum, else 1.
pub fn fn_00b4da00(e: &mut Engine, state: u32) {
    let mut text = 0u32;
    let mut binary = 0u32;
    let at = |n: u32| state + TS_LITERAL_TREE + n * 4;
    let mut n = 0u32;
    while n < 7 {
        binary = u32::from(e.mem.u16(at(n))).wrapping_add(binary);
        n += 1;
    }
    while n < 0x80 {
        text = u32::from(e.mem.u16(at(n))).wrapping_add(text);
        n += 1;
    }
    while n < 0x100 {
        binary = u32::from(e.mem.u16(at(n))).wrapping_add(binary);
        n += 1;
    }
    let data_type = 1u8.wrapping_sub(u8::from(text >> 2 < binary));
    e.mem.set_u8(state + TS_DATA_TYPE, data_type);
}

// Translated from 00b4dab0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map; zlib's `bi_reverse` by its body (`cdecl`, two words: a code
/// and its length): the code's low `length` bits reversed (at least one bit is taken).
pub fn fn_00b4dab0(_e: &mut Engine, code: u32, length: i32) -> u32 {
    let mut code = code;
    let mut length = length;
    let mut result = 0u32;
    loop {
        result |= code & 1;
        code >>= 1;
        result <<= 1;
        length -= 1;
        if length <= 0 {
            break;
        }
    }
    result >> 1
}

// Translated from 00b4daf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map; zlib's `bi_flush` by its body (`cdecl`, one word: the
/// compressor state): a full bit buffer (16) goes out as two bytes and empties; at least 8
/// bits held send the low byte and keep the rest.
pub fn fn_00b4daf0(e: &mut Engine, state: u32) {
    if e.mem.i32(state + TS_BIT_COUNT) == 0x10 {
        let buffer = u32::from(e.mem.u16(state + TS_BIT_BUFFER));
        tree_put_byte(e, state, buffer & 0xff);
        let buffer = u32::from(e.mem.u16(state + TS_BIT_BUFFER));
        tree_put_byte(e, state, buffer >> 8);
        e.mem.set_u16(state + TS_BIT_BUFFER, 0);
        e.mem.set_u32(state + TS_BIT_COUNT, 0);
    } else if e.mem.i32(state + TS_BIT_COUNT) > 7 {
        let byte = u32::from(e.mem.u8(state + TS_BIT_BUFFER));
        tree_put_byte(e, state, byte);
        let buffer = e.mem.u16(state + TS_BIT_BUFFER) >> 8;
        e.mem.set_u16(state + TS_BIT_BUFFER, buffer);
        let held = e.mem.i32(state + TS_BIT_COUNT).wrapping_sub(8);
        e.mem.set_i32(state + TS_BIT_COUNT, held);
    }
}

// Translated from 00b4dbe0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map; zlib's `bi_windup` by its body (`cdecl`, one word: the
/// compressor state): more than 8 bits held send both bytes, 1..8 the low one; the bit
/// buffer then empties.
pub fn fn_00b4dbe0(e: &mut Engine, state: u32) {
    if e.mem.i32(state + TS_BIT_COUNT) > 8 {
        let buffer = u32::from(e.mem.u16(state + TS_BIT_BUFFER));
        tree_put_byte(e, state, buffer & 0xff);
        let buffer = u32::from(e.mem.u16(state + TS_BIT_BUFFER));
        tree_put_byte(e, state, buffer >> 8);
    } else if e.mem.i32(state + TS_BIT_COUNT) > 0 {
        let byte = u32::from(e.mem.u8(state + TS_BIT_BUFFER));
        tree_put_byte(e, state, byte);
    }
    e.mem.set_u16(state + TS_BIT_BUFFER, 0);
    e.mem.set_u32(state + TS_BIT_COUNT, 0);
}

// Translated from 00b4dca0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map; zlib's `copy_block` by its body (`cdecl`, four words: the
/// compressor state, a buffer, its length and a header flag): `fn_00b4dbe0`, the last
/// end-of-block length 8, with the flag the length and its complement (16 bits each, low
/// byte first), then the bytes, one by one into the pending buffer.
pub fn fn_00b4dca0(e: &mut Engine, state: u32, buffer: u32, length: u32, header: u32) {
    fn_00b4dbe0(e, state);
    e.mem.set_u32(state + TS_LAST_END_LENGTH, 8);
    if header != 0 {
        tree_put_byte(e, state, length & 0xff);
        tree_put_byte(e, state, (length >> 8) & 0xff);
        tree_put_byte(e, state, !length & 0xff);
        tree_put_byte(e, state, !(length >> 8) & 0xff);
    }
    let mut at = buffer;
    let mut left = length;
    while left != 0 {
        let byte = u32::from(e.mem.u8(at));
        tree_put_byte(e, state, byte);
        at = at.wrapping_add(1);
        left -= 1;
    }
}

// Translated from 00b4ddb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map; zlib's `compress2` by its body (`cdecl`, five words: the
/// destination, the address of its size, the source, its length and the level). A local
/// stream record (input the source, output the destination with its size, default
/// allocator) is set up with `deflateInit_` (`00b46360`, version `011ad7ac`; its error
/// answered), finished in one `deflate` call (`00b46b70` with 4): anything but the end
/// takes the stream down (`00b47370`) and answers -5 for 0, else the answer; the end
/// stores the total out as the size and answers `deflateEnd`'s answer.
pub fn fn_00b4ddb0(
    e: &mut Engine,
    destination: u32,
    size: u32,
    source: u32,
    length: u32,
    level: i32,
) -> i32 {
    e.with_stack(0x38, |e, stream| {
        let stream = stream.addr();
        e.mem.set_u32(stream, source);
        e.mem.set_u32(stream + 4, length);
        e.mem.set_u32(stream + 0xc, destination);
        let room = e.mem.u32(size);
        e.mem.set_u32(stream + 0x10, room);
        if e.mem.u32(stream + 0x10) != e.mem.u32(size) {
            return -5;
        }
        e.mem.set_u32(stream + 0x20, 0);
        e.mem.set_u32(stream + 0x24, 0);
        e.mem.set_u32(stream + 0x28, 0);
        let answer = e
            .call(
                DEFLATE_INIT,
                &args![stream, level, COMPRESS_VERSION, 0x38u32],
            )
            .i32();
        if answer != 0 {
            return answer;
        }
        let answer = e.call(DEFLATE, &args![stream, 4u32]).i32();
        if answer != 1 {
            e.call(DEFLATE_END, &args![stream]);
            return if answer == 0 { -5 } else { answer };
        }
        let total = e.mem.u32(stream + 0x14);
        e.mem.set_u32(size, total);
        e.call(DEFLATE_END, &args![stream]).i32()
    })
}

// Translated from 00b4de80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map; zlib's `compress` by its body (`cdecl`, four words): the same
/// words to `fn_00b4ddb0` with level -1; its answer.
pub fn fn_00b4de80(e: &mut Engine, destination: u32, size: u32, source: u32, length: u32) -> i32 {
    fn_00b4ddb0(e, destination, size, source, length, -1)
}

// Translated from 00b4dea0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map; zlib's `compressBound` by its body (`cdecl`, one word: a
/// length): `length + (length >> 12) + (length >> 14) + 11`.
pub fn fn_00b4dea0(_e: &mut Engine, length: u32) -> u32 {
    (length >> 12)
        .wrapping_add(length)
        .wrapping_add(length >> 14)
        .wrapping_add(0xb)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00b4_9e20, fn_00b49e20(u32, u32, u32) -> u32),
        entry!(0x00b4_9070, fn_00b49070(u32, u32)),
        entry!(0x00b4_97d0, fn_00b497d0(u32, u32, u32, u32, u32, u32) -> i32),
        entry!(0x00b4_a070, fn_00b4a070(u32, u32, u32) -> u32),
        entry!(0x00b4_a270, fn_00b4a270(u32, u32, u32) -> u32),
        entry!(0x00b4_a670, fn_00b4a670(u32, u32, u32) -> u32),
        entry!(0x00b4_a060, fn_00b4a060() -> u32),
        entry!(0x00b4_aad0, fn_00b4aad0(u32)),
        entry!(0x00b4_ab70, fn_00b4ab70()),
        entry!(0x00b4_ab80, fn_00b4ab80(u32)),
        entry!(0x00b4_ac50, fn_00b4ac50(u32, u32, u32, u32)),
        entry!(0x00b4_ad90, fn_00b4ad90(u32)),
        entry!(0x00b4_b250, fn_00b4b250(u32, u32, u32, u32)),
        entry!(0x00b4_b600, fn_00b4b600(u32, u32)),
        entry!(0x00b4_b9a0, fn_00b4b9a0(u32, u32, i32)),
        entry!(0x00b4_bb30, fn_00b4bb30(u32, u32)),
        entry!(0x00b4_be40, fn_00b4be40(u32, i32, u32)),
        entry!(0x00b4_bf10, fn_00b4bf10(u32) -> i32),
        entry!(0x00b4_bfd0, fn_00b4bfd0(u32, u32, i32)),
        entry!(0x00b4_c170, fn_00b4c170(u32, i32, i32, i32)),
        entry!(0x00b4_c670, fn_00b4c670(u32, u32, i32)),
        entry!(0x00b4_d0e0, fn_00b4d0e0(u32, u32, u32) -> bool),
        entry!(0x00b4_d210, fn_00b4d210(u32, u32, u32)),
        entry!(0x00b4_da00, fn_00b4da00(u32)),
        entry!(0x00b4_dab0, fn_00b4dab0(u32, i32) -> u32),
        entry!(0x00b4_daf0, fn_00b4daf0(u32)),
        entry!(0x00b4_dbe0, fn_00b4dbe0(u32)),
        entry!(0x00b4_dca0, fn_00b4dca0(u32, u32, u32, u32)),
        entry!(0x00b4_ddb0, fn_00b4ddb0(u32, u32, u32, u32, i32) -> i32),
        entry!(0x00b4_de80, fn_00b4de80(u32, u32, u32, u32) -> i32),
        entry!(0x00b4_dea0, fn_00b4dea0(u32) -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The same sums written out directly, for the check.
    fn expected(text: &[u8]) -> u32 {
        let (mut a, mut b) = (1u32, 0u32);
        for &c in text {
            a = (a + u32::from(c)) % 0xfff1;
            b = (b + a) % 0xfff1;
        }
        (b << 16) | a
    }

    #[test]
    fn fn_00b49e20_sums_short_and_long_buffers() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00b4_9e20, &args![5u32, 0u32, 3u32]).u32(), 1);
        let text = b"Wikipedia";
        let at = e.mem.alloc(16);
        e.mem.write(at, text);
        assert_eq!(
            e.call(0x00b4_9e20, &args![1u32, at, 9u32]).u32(),
            0x11e6_0398
        );
        let long: Vec<u8> = (0..0x2000u32).map(|i| (i * 7 + 3) as u8).collect();
        let big = e.mem.alloc(0x2000);
        e.mem.write(big, &long);
        assert_eq!(
            e.call(0x00b4_9e20, &args![1u32, big, 0x2000u32]).u32(),
            expected(&long)
        );
    }

    /// Packs (value, bit count) fields into `len` bytes, least significant
    /// bit first.
    fn pack_bits(fields: &[(u32, u32)], len: usize) -> Vec<u8> {
        let mut bytes = vec![0u8; len];
        let mut at = 0usize;
        for &(value, count) in fields {
            for bit in 0..count {
                if (value >> bit) & 1 != 0 {
                    bytes[at / 8] |= 1 << (at % 8);
                }
                at += 1;
            }
        }
        bytes
    }

    /// Literal/length entries (two index bits): 0 a length of 3 plus two
    /// extra bits, 1 the block's end, 2 the literal 'A', 3 a link to 4 and 5
    /// (one more bit): the literal 'B' and an invalid code.
    const LENGTH_CODES: [u32; 6] = [
        0x0003_0212,
        0x0000_0260,
        0x0041_0200,
        0x0004_0201,
        0x0042_0100,
        0x0000_0140,
    ];
    /// Distance entries (two index bits): 0 a distance of 1 plus three extra
    /// bits, 1 an invalid code, 2 a link to 4 and 5 (one more bit): a
    /// distance of 2 and an invalid code.
    const DISTANCE_CODES: [u32; 6] = [
        0x0001_0213,
        0x0000_0240,
        0x0004_0201,
        0,
        0x0002_0110,
        0x0000_0140,
    ];

    /// What a run of 00b49070 left: the stream, the state, the input and the
    /// output buffers.
    struct FastRun {
        stream: u32,
        state: u32,
        input: u32,
        output: u32,
    }

    /// Runs 00b49070 over `input` (`available` bytes counted) with the
    /// given tables (two index bits each), the window "01234567" (eight
    /// bytes, `have` of them filled, write position `write`) and 300 bytes
    /// of output; the mode starts at 7.
    fn fast_run(
        e: &mut Engine,
        tables: (&[u32], &[u32]),
        input: &[u8],
        available: u32,
        have: u32,
        write: u32,
    ) -> FastRun {
        let stream = e.mem.alloc(0x38);
        let state = e.mem.alloc(0x60);
        let input_at = e.mem.alloc(64);
        e.mem.write(input_at, input);
        let output = e.mem.alloc(300);
        let window = e.mem.alloc(8);
        e.mem.write(window, b"01234567");
        let mut codes = [0u32; 2];
        for (slot, words) in [tables.0, tables.1].iter().enumerate() {
            let at = e.mem.alloc(4 * words.len() as u32);
            for (i, word) in words.iter().enumerate() {
                e.mem.set_u32(at + 4 * i as u32, *word);
            }
            codes[slot] = at;
        }
        e.mem.set_u32(stream, input_at);
        e.mem.set_u32(stream + 0x4, available);
        e.mem.set_u32(stream + 0xc, output);
        e.mem.set_u32(stream + 0x10, 300);
        e.mem.set_u32(stream + 0x1c, state);
        e.mem.set_u32(state, 7);
        e.mem.set_u32(state + 0x20, 8);
        e.mem.set_u32(state + 0x24, have);
        e.mem.set_u32(state + 0x28, write);
        e.mem.set_u32(state + 0x2c, window);
        e.mem.set_u32(state + 0x44, codes[0]);
        e.mem.set_u32(state + 0x48, codes[1]);
        e.mem.set_u32(state + 0x4c, 2);
        e.mem.set_u32(state + 0x50, 2);
        e.call(0x00b4_9070, &args![stream, 300u32]);
        FastRun {
            stream,
            state,
            input: input_at,
            output,
        }
    }

    #[test]
    fn fn_00b49070_decodes_literals_copies_and_errors() {
        let mut e = Engine::new();
        let tables = (&LENGTH_CODES[..], &DISTANCE_CODES[..]);
        // 'A', 'B' through the link, a length of 5 at distance 2 (through
        // the distance link) from the output itself, the block's end.
        let bits = pack_bits(
            &[
                (2, 2),
                (3, 2),
                (0, 1),
                (0, 2),
                (2, 2),
                (2, 2),
                (0, 1),
                (1, 2),
            ],
            16,
        );
        let run = fast_run(&mut e, tables, &bits, 16, 8, 0);
        assert_eq!(e.mem.bytes(run.output, 8), b"ABABABA\0");
        assert_eq!(e.mem.u32(run.state), 0xb);
        assert_eq!(e.mem.u32(run.stream), run.input + 2);
        assert_eq!(e.mem.u32(run.stream + 0x4), 14);
        assert_eq!(e.mem.u32(run.stream + 0xc), run.output + 7);
        assert_eq!(e.mem.u32(run.stream + 0x10), 293);
        assert_eq!(e.mem.u32(run.stream + 0x18), 0);
        assert_eq!(e.mem.u32(run.state + 0x30), 0);
        assert_eq!(e.mem.u32(run.state + 0x34), 2);

        // From the window: write position 0 (length 4, distance 2), behind
        // the write position (length 6, distance 5, wrapping to the
        // window's start), and before it (length 6, distance 2).
        let cases: [(u32, u32, u32, &[u8]); 3] = [
            (1, 1, 0, b"6767"),
            (3, 4, 3, b"670126"),
            (3, 1, 3, b"121212"),
        ];
        for (length_extra, distance_extra, write, expected) in cases {
            let bits = pack_bits(
                &[
                    (0, 2),
                    (length_extra, 2),
                    (0, 2),
                    (distance_extra, 3),
                    (1, 2),
                ],
                16,
            );
            let run = fast_run(&mut e, tables, &bits, 16, 8, write);
            assert_eq!(e.mem.bytes(run.output, expected.len() as u32), expected);
            assert_eq!(e.mem.u32(run.state), 0xb);
        }

        // Errors: a distance beyond the window's fill, an invalid distance
        // code, an invalid literal/length code (second level).
        let errors: [(&[(u32, u32)], u32, u32); 3] = [
            (
                &[(0, 2), (0, 2), (2, 2), (0, 1)],
                1,
                MESSAGE_DISTANCE_TOO_FAR,
            ),
            (&[(0, 2), (0, 2), (1, 2)], 8, MESSAGE_DISTANCE_CODE),
            (&[(3, 2), (1, 1)], 8, MESSAGE_LITERAL_LENGTH_CODE),
        ];
        for (fields, have, message) in errors {
            let bits = pack_bits(fields, 16);
            let run = fast_run(&mut e, tables, &bits, 16, have, 0);
            assert_eq!(e.mem.u32(run.stream + 0x18), message);
            assert_eq!(e.mem.u32(run.state), 0x1b);
            assert_eq!(e.mem.u8(run.output), 0);
        }

        // Extra bits that need one more byte (length) and two more bytes
        // (distance, after a code dropping 20 bits); the copy comes from the
        // window's last byte.
        let wide = (
            &[0x0003_021f, 0x0000_0260, 0, 0][..],
            &[0x0001_141f, 0, 0, 0][..],
        );
        let bits = pack_bits(&[(0, 2), (0, 15), (0, 20), (0, 15), (1, 2)], 16);
        let run = fast_run(&mut e, wide, &bits, 20, 8, 0);
        assert_eq!(e.mem.bytes(run.output, 4), b"777\0");
        assert_eq!(e.mem.u32(run.state), 0xb);
        assert_eq!(e.mem.u32(run.stream), run.input + 7);
        assert_eq!(e.mem.u32(run.stream + 0x4), 13);
        assert_eq!(e.mem.u32(run.stream + 0x10), 297);
        assert_eq!(e.mem.u32(run.state + 0x34), 2);

        // The input runs out after one literal: the mode stays.
        let bits = pack_bits(&[(2, 2)], 16);
        let run = fast_run(&mut e, tables, &bits, 6, 8, 0);
        assert_eq!(e.mem.bytes(run.output, 2), b"A\0");
        assert_eq!(e.mem.u32(run.state), 7);
        assert_eq!(e.mem.u32(run.stream), run.input + 1);
        assert_eq!(e.mem.u32(run.stream + 0x4), 5);
        assert_eq!(e.mem.u32(run.state + 0x30), 0);
        assert_eq!(e.mem.u32(run.state + 0x34), 6);
    }

    /// Runs 00b497d0 of `kind` on `lengths` with root bits `root`; answers
    /// its result, the first `size` table words, how many entries the table
    /// pointer moved and the root bits written back.
    fn build_table(
        e: &mut Engine,
        kind: u32,
        lengths: &[u16],
        root: u32,
        size: u32,
    ) -> (i32, Vec<u32>, u32, u32) {
        let lens = e.mem.alloc(2 * lengths.len() as u32);
        for (i, length) in lengths.iter().enumerate() {
            e.mem.set_u16(lens + 2 * i as u32, *length);
        }
        let area = e.mem.alloc(4 * size + 0x40);
        let table = e.mem.alloc(4);
        e.mem.set_u32(table, area);
        let bits = e.mem.alloc(4);
        e.mem.set_u32(bits, root);
        let work = e.mem.alloc(0x240);
        let result = e
            .call(
                0x00b4_97d0,
                &args![kind, lens, lengths.len() as u32, table, bits, work],
            )
            .u32() as i32;
        let words = (0..size).map(|i| e.mem.u32(area + 4 * i)).collect();
        let moved = (e.mem.u32(table) - area) / 4;
        (result, words, moved, e.mem.u32(bits))
    }

    #[test]
    fn fn_00b497d0_builds_tables_and_refuses_bad_lengths() {
        let mut e = Engine::new();
        // Kind 0: lengths 2, 1, 3, 3 with three root bits.
        assert_eq!(
            build_table(&mut e, 0, &[2, 1, 3, 3], 3, 8),
            (
                0,
                vec![
                    0x0001_0100,
                    0x0000_0200,
                    0x0001_0100,
                    0x0002_0300,
                    0x0001_0100,
                    0x0000_0200,
                    0x0001_0100,
                    0x0003_0300,
                ],
                8,
                3
            )
        );
        // Kind 1: symbol 0 of length 1, 256 and 257 of length 2 with one
        // root bit: a second-level table for 256 (operation 0x60) and 257
        // (from the length tables, marked here).
        e.map(LENGTH_BASE, 0x100);
        e.mem.set_u16(LENGTH_BASE, 0x1234);
        e.mem.set_u8(LENGTH_EXTRA, 0x13);
        let mut lengths = vec![0u16; 258];
        lengths[0] = 1;
        lengths[256] = 2;
        lengths[257] = 2;
        assert_eq!(
            build_table(&mut e, 1, &lengths, 1, 4),
            (
                0,
                vec![0x0000_0100, 0x0002_0101, 0x0000_0160, 0x1234_0113],
                4,
                1
            )
        );
        // Kind 2: one code (symbol 1, length 1), root bits clamped from 2 to
        // 1, the other entry padded as invalid.
        e.map(DISTANCE_BASE, 0x80);
        e.mem.set_u16(DISTANCE_BASE + 2, 0x0abc);
        e.mem.set_u8(DISTANCE_EXTRA + 2, 0x11);
        assert_eq!(
            build_table(&mut e, 2, &[0, 1], 2, 2),
            (0, vec![0x0abc_0111, 0x0000_0140], 2, 1)
        );
        // No lengths, over-subscribed, incomplete for kind 0: -1, nothing
        // moved or written back.
        for lengths in [&[0u16, 0][..], &[1, 1, 1][..], &[1][..]] {
            let (result, _, moved, bits) = build_table(&mut e, 0, lengths, 9, 0);
            assert_eq!((result, moved, bits), (-1, 0, 9));
        }
        // Kind 1 with 11 root bits: 2048 entries reach the limit, answers 1.
        let (result, _, moved, bits) = build_table(&mut e, 1, &[11], 11, 0);
        assert_eq!((result, moved, bits), (1, 0, 11));
    }

    #[test]
    fn fn_00b4a070_answers_zero_or_the_taken_helper() {
        let mut e = Engine::new();
        e.register(CRC_TAKEN_HELPER, |_, a| (a[0] ^ a[1] ^ a[2]).into_ret());
        e.register(CRC_OTHER_HELPER, |_, _| 0xdead_beefu32.into_ret());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00b4_a070, &args![5u32, 0u32, 3u32]).u32(), 0);
        assert_eq!(
            e.call(0x00b4_a070, &args![0x1111u32, 0x2000u32, 7u32])
                .u32(),
            0x1111 ^ 0x2000 ^ 7
        );
        let log = e.call_log.take().unwrap();
        let calls = |addr: u32| -> Vec<Vec<u32>> {
            log.iter()
                .filter(|(at, _)| *at == addr)
                .map(|(_, a)| a.clone())
                .collect()
        };
        assert_eq!(calls(CRC_TAKEN_HELPER), vec![vec![0x1111, 0x2000, 7]]);
        assert!(calls(CRC_OTHER_HELPER).is_empty());
    }

    /// The reflected CRC-32 table (polynomial `edb88320`) and the three
    /// tables that step it a further byte each, built here (tests run without
    /// the exe).
    fn crc_tables() -> Vec<[u32; 256]> {
        let mut first = [0u32; 256];
        for (n, word) in first.iter_mut().enumerate() {
            let mut c = n as u32;
            for _ in 0..8 {
                c = if c & 1 != 0 {
                    (c >> 1) ^ 0xedb8_8320
                } else {
                    c >> 1
                };
            }
            *word = c;
        }
        let mut tables = vec![first];
        for _ in 1..4 {
            let prior = tables[tables.len() - 1];
            let mut next = [0u32; 256];
            for (word, &before) in next.iter_mut().zip(prior.iter()) {
                *word = (before >> 8) ^ first[(before & 0xff) as usize];
            }
            tables.push(next);
        }
        tables
    }

    /// An engine with the eight tables where the exe keeps them: the four
    /// above, then the same four with each word's bytes reversed.
    fn crc_engine() -> Engine {
        let mut e = Engine::new();
        e.map(CRC_TABLE, 0x2000);
        let tables = crc_tables();
        for (k, table) in tables.iter().enumerate() {
            for (n, &word) in table.iter().enumerate() {
                let at = CRC_TABLE + (k as u32) * 0x400 + (n as u32) * 4;
                e.mem.set_u32(at, word);
                e.mem.set_u32(at + 0x1000, word.swap_bytes());
            }
        }
        e
    }

    /// CRC-32 a byte at a time, for the check.
    fn crc32(crc: u32, bytes: &[u8]) -> u32 {
        let first = crc_tables()[0];
        let mut c = !crc;
        for &b in bytes {
            c = (c >> 8) ^ first[((c ^ u32::from(b)) & 0xff) as usize];
        }
        !c
    }

    /// Writes `data` `offset` bytes past an aligned allocation; its address.
    fn place(e: &mut Engine, data: &[u8], offset: u32) -> u32 {
        let base = e.mem.alloc(data.len() as u32 + 8);
        assert_eq!(base & 3, 0);
        e.mem.write(base + offset, data);
        base + offset
    }

    #[test]
    fn fn_00b4a270_matches_crc32_through_every_loop() {
        let mut e = crc_engine();
        // Leaves the CRC as it is for an empty buffer (complemented twice).
        assert_eq!(
            e.call(0x00b4_a270, &args![0x1234_5678u32, 0u32, 0u32])
                .u32(),
            0x1234_5678
        );
        // The check value, from an unaligned start (bytes, words, bytes).
        let check = place(&mut e, b"123456789", 1);
        assert_eq!(
            e.call(0x00b4_a270, &args![0u32, check, 9u32]).u32(),
            0xcbf4_3926
        );
        // 78 bytes from +1: 3 alignment bytes, two 32-byte rounds, two words,
        // 3 bytes left; and from an aligned start; and chained in two calls.
        let data: Vec<u8> = (0..78u32).map(|i| (i * 37 + 11) as u8).collect();
        for offset in [0u32, 1, 2, 3] {
            let at = place(&mut e, &data, offset);
            assert_eq!(
                e.call(0x00b4_a270, &args![0u32, at, 78u32]).u32(),
                crc32(0, &data)
            );
            let head = e.call(0x00b4_a270, &args![0u32, at, 5u32]).u32();
            assert_eq!(
                e.call(0x00b4_a270, &args![head, at + 5, 73u32]).u32(),
                crc32(0, &data)
            );
        }
    }

    #[test]
    fn fn_00b4a670_steps_words_as_their_bytes_reversed() {
        let mut e = crc_engine();
        assert_eq!(
            e.call(0x00b4_a670, &args![0x1234_5678u32, 0u32, 0u32])
                .u32(),
            0x1234_5678
        );
        // Bytes only (three from +1): the plain CRC-32.
        let short = place(&mut e, b"abc", 1);
        assert_eq!(
            e.call(0x00b4_a670, &args![0u32, short, 3u32]).u32(),
            crc32(0, b"abc")
        );
        // With word steps, each aligned word's bytes count in reverse order
        // (the words are loaded little-endian, the tables step big-endian).
        let data: Vec<u8> = (0..78u32).map(|i| (i * 53 + 7) as u8).collect();
        for offset in [0u32, 1, 3] {
            let at = place(&mut e, &data, offset);
            let head = ((4 - offset) & 3) as usize;
            let words = (78 - head) / 4 * 4;
            let mut seen = data.clone();
            for group in seen[head..head + words].chunks_mut(4) {
                group.reverse();
            }
            assert_eq!(
                e.call(0x00b4_a670, &args![0x0bad_cafeu32, at, 78u32]).u32(),
                crc32(0x0bad_cafe, &seen)
            );
        }
    }
}

#[cfg(test)]
mod n080_tests {
    use super::*;

    fn answer(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    fn calls_to(e: &Engine, address: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == address)
            .map(|(_, w)| w.clone())
            .collect()
    }

    /// A compressor state with a pending buffer, and the exe's table pages mapped.
    fn state_engine() -> (Engine, u32) {
        let mut e = Engine::new();
        e.map(0x010a_8000, 0x1000);
        e.map(0x011a_d000, 0x1000);
        let state = e.mem.alloc(0x16b8);
        let pending = e.mem.alloc(0x100);
        e.mem.set_u32(state + TS_PENDING_BUFFER, pending);
        (e, state)
    }

    fn pending_bytes(e: &Engine, state: u32) -> Vec<u8> {
        let buffer = e.mem.u32(state + TS_PENDING_BUFFER);
        e.mem.bytes(buffer, e.mem.u32(state + TS_PENDING))
    }

    /// Sets a tree entry's code (or frequency) and length.
    fn set_entry(e: &mut Engine, tree: u32, n: u32, code: u16, length: u16) {
        e.mem.set_u16(tree + 4 * n, code);
        e.mem.set_u16(tree + 4 * n + 2, length);
    }

    /// The bits of `(value, length)` pairs packed low bit first.
    fn packed(bits: &[(u32, u32)]) -> (u128, u32) {
        let mut all = 0u128;
        let mut count = 0u32;
        for (value, length) in bits {
            all |= u128::from(value & ((1 << length) - 1)) << count;
            count += length;
        }
        (all, count)
    }

    #[test]
    fn small_helpers() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00b4_a060, &[]).u32(), CRC_TABLE);
        e.call(0x00b4_ab70, &[]);
        assert_eq!(
            e.call(0x00b4_dea0, &args![0x10000u32]).u32(),
            0x10000 + 0x10 + 4 + 0xb
        );
        assert_eq!(e.call(0x00b4_dab0, &args![0b1101u32, 4i32]).u32(), 0b1011);
        assert_eq!(e.call(0x00b4_dab0, &args![0b1u32, 0i32]).u32(), 1);
    }

    #[test]
    fn init_resets_the_trees_and_bits() {
        let (mut e, state) = state_engine();
        e.mem.set_u16(state + TS_LITERAL_TREE + 4 * 7, 9);
        e.mem.set_u16(state + TS_BIT_LENGTH_TREE + 4 * 18, 9);
        e.mem.set_u32(state + TS_LAST_LITERAL, 5);
        e.mem.set_u16(state + TS_BIT_BUFFER, 0x55);
        e.call(0x00b4_aad0, &args![state]);
        assert_eq!(e.mem.u16(state + TS_LITERAL_TREE + 4 * 7), 0);
        assert_eq!(e.mem.u16(state + TS_BIT_LENGTH_TREE + 4 * 18), 0);
        assert_eq!(e.mem.u16(state + TS_LITERAL_TREE + 4 * 0x100), 1);
        assert_eq!(e.mem.u32(state + TS_LAST_LITERAL), 0);
        assert_eq!(e.mem.u16(state + TS_BIT_BUFFER), 0);
        assert_eq!(e.mem.u32(state + TS_LAST_END_LENGTH), 8);
        assert_eq!(
            e.mem.u32(state + TS_DISTANCE_DESCRIPTOR),
            state + TS_DISTANCE_TREE
        );
        assert_eq!(
            e.mem.u32(state + TS_BIT_LENGTH_DESCRIPTOR + 8),
            STATIC_BIT_LENGTH_DESCRIPTOR
        );
    }

    #[test]
    fn stored_block_writes_header_and_bytes() {
        let (mut e, state) = state_engine();
        let data = e.mem.alloc(4);
        e.mem.set_u8(data, b'A');
        e.mem.set_u8(data + 1, b'B');
        e.call(0x00b4_ac50, &args![state, data, 2u32, 1u32]);
        assert_eq!(pending_bytes(&e, state), [1, 2, 0, 0xfd, 0xff, b'A', b'B']);
        assert_eq!(e.mem.u32(state + TS_BIT_COUNT), 0);
        assert_eq!(e.mem.u32(state + TS_LAST_END_LENGTH), 8);
    }

    #[test]
    fn align_adds_a_second_empty_block_when_short() {
        let (mut e, state) = state_engine();
        e.mem.set_u32(state + TS_LAST_END_LENGTH, 8);
        e.call(0x00b4_ad90, &args![state]);
        assert_eq!(pending_bytes(&e, state), [2]);
        assert_eq!(e.mem.u32(state + TS_BIT_COUNT), 2);
        assert_eq!(e.mem.u32(state + TS_LAST_END_LENGTH), 7);
        let (mut e, state) = state_engine();
        e.mem.set_u32(state + TS_LAST_END_LENGTH, 0);
        e.mem.set_u32(state + TS_BIT_COUNT, 1);
        e.call(0x00b4_ad90, &args![state]);
        assert_eq!(pending_bytes(&e, state), [4, 0x10]);
        assert_eq!(e.mem.u32(state + TS_BIT_COUNT), 5);
    }

    #[test]
    fn bit_flush_and_windup() {
        let (mut e, state) = state_engine();
        e.mem.set_u16(state + TS_BIT_BUFFER, 0x1234);
        e.mem.set_u32(state + TS_BIT_COUNT, 16);
        e.call(0x00b4_daf0, &args![state]);
        assert_eq!(pending_bytes(&e, state), [0x34, 0x12]);
        e.mem.set_u16(state + TS_BIT_BUFFER, 0x1ab);
        e.mem.set_u32(state + TS_BIT_COUNT, 9);
        e.call(0x00b4_daf0, &args![state]);
        assert_eq!(e.mem.u16(state + TS_BIT_BUFFER), 1);
        assert_eq!(e.mem.u32(state + TS_BIT_COUNT), 1);
        e.call(0x00b4_dbe0, &args![state]);
        assert_eq!(pending_bytes(&e, state), [0x34, 0x12, 0xab, 1]);
        e.mem.set_u16(state + TS_BIT_BUFFER, 0x203);
        e.mem.set_u32(state + TS_BIT_COUNT, 10);
        e.call(0x00b4_dbe0, &args![state]);
        assert_eq!(pending_bytes(&e, state)[4..], [3, 2]);
        assert_eq!(e.mem.u32(state + TS_BIT_COUNT), 0);
    }

    #[test]
    fn tally_counts_literals_and_matches() {
        let (mut e, state) = state_engine();
        let distances = e.mem.alloc(8);
        let literals = e.mem.alloc(8);
        e.mem.set_u32(state + TS_DISTANCE_BUFFER, distances);
        e.mem.set_u32(state + TS_LITERAL_BUFFER, literals);
        e.mem.set_u32(state + TS_LITERAL_BUFFER_SIZE, 3);
        e.mem.set_u8(LENGTH_CODE + 10, 9);
        e.mem.set_u8(DISTANCE_CODE + 0x100 + 2, 16);
        assert!(!e.call(0x00b4_d0e0, &args![state, 0u32, 0x41u32]).bool());
        assert!(e.call(0x00b4_d0e0, &args![state, 0x101u32, 10u32]).bool());
        assert_eq!(e.mem.u16(state + TS_LITERAL_TREE + 4 * 0x41), 1);
        assert_eq!(e.mem.u16(state + TS_LITERAL_TREE + 4 * (9 + 0x101)), 1);
        assert_eq!(e.mem.u16(state + TS_DISTANCE_TREE + 4 * 16), 1);
        assert_eq!(e.mem.u32(state + TS_MATCHES), 1);
        assert_eq!(e.mem.u16(distances + 2), 0x101);
        assert_eq!(e.mem.u8(literals + 1), 10);
    }

    #[test]
    fn data_type_guess() {
        let (mut e, state) = state_engine();
        e.mem.set_u16(state + TS_LITERAL_TREE + 4 * b'a' as u32, 8);
        e.mem.set_u16(state + TS_LITERAL_TREE + 4 * 0x90, 2);
        e.call(0x00b4_da00, &args![state]);
        assert_eq!(e.mem.u8(state + TS_DATA_TYPE), 1);
        e.mem.set_u16(state + TS_LITERAL_TREE + 4 * 3, 1);
        e.call(0x00b4_da00, &args![state]);
        assert_eq!(e.mem.u8(state + TS_DATA_TYPE), 0);
    }

    #[test]
    fn codes_follow_the_lengths() {
        let (mut e, _) = state_engine();
        let tree = e.mem.alloc(0x20);
        for (n, length) in [2u16, 1, 3, 3].iter().enumerate() {
            set_entry(&mut e, tree, n as u32, 0, *length);
        }
        let counts = e.mem.alloc(0x20);
        e.mem.set_u16(counts + 2, 1);
        e.mem.set_u16(counts + 4, 1);
        e.mem.set_u16(counts + 6, 2);
        e.call(0x00b4_be40, &args![tree, 3i32, counts]);
        let codes: Vec<u16> = (0..4).map(|n| e.mem.u16(tree + 4 * n)).collect();
        assert_eq!(codes, vec![1, 0, 3, 7]);
    }

    /// A static descriptor without a static tree: extra bits (all 0 but those given), a
    /// base, the element count and the length limit.
    fn descriptor(e: &mut Engine, extras: &[(u32, u32)], elements: u32, limit: u32) -> u32 {
        let extra = e.mem.alloc(4 * elements);
        for (n, bits) in extras {
            e.mem.set_u32(extra + 4 * n, *bits);
        }
        let statics = e.mem.alloc(0x14);
        e.mem.set_u32(statics + 4, extra);
        e.mem.set_u32(statics + 0xc, elements);
        e.mem.set_u32(statics + 0x10, limit);
        statics
    }

    #[test]
    fn tree_building_gives_huffman_lengths() {
        let (mut e, state) = state_engine();
        let statics = descriptor(&mut e, &[], 4, 15);
        let tree = state + TS_LITERAL_TREE;
        for (n, f) in [5u16, 9, 1, 1].iter().enumerate() {
            set_entry(&mut e, tree, n as u32, *f, 0);
        }
        let desc = state + TS_LITERAL_DESCRIPTOR;
        e.mem.set_u32(desc, tree);
        e.mem.set_u32(desc + 8, statics);
        e.call(0x00b4_b600, &args![state, desc]);
        let lengths: Vec<u16> = (0..4).map(|n| e.mem.u16(tree + 4 * n + 2)).collect();
        let codes: Vec<u16> = (0..4).map(|n| e.mem.u16(tree + 4 * n)).collect();
        assert_eq!(lengths, vec![2, 1, 3, 3]);
        assert_eq!(codes, vec![1, 0, 3, 7]);
        assert_eq!(e.mem.i32(desc + 4), 3);
        assert_eq!(e.mem.u32(state + TS_OPTIMAL_LENGTH), 25);
    }

    #[test]
    fn length_limit_redistributes_the_overflow() {
        let (mut e, state) = state_engine();
        let statics = descriptor(&mut e, &[], 4, 2);
        let tree = state + TS_LITERAL_TREE;
        for (n, f) in [1u16, 1, 2, 4].iter().enumerate() {
            set_entry(&mut e, tree, n as u32, *f, 0);
        }
        let desc = state + TS_LITERAL_DESCRIPTOR;
        e.mem.set_u32(desc, tree);
        e.mem.set_u32(desc + 8, statics);
        e.call(0x00b4_b600, &args![state, desc]);
        let lengths: Vec<u16> = (0..4).map(|n| e.mem.u16(tree + 4 * n + 2)).collect();
        let codes: Vec<u16> = (0..4).map(|n| e.mem.u16(tree + 4 * n)).collect();
        assert_eq!(lengths, vec![2, 2, 2, 2]);
        assert_eq!(codes, vec![0, 2, 1, 3]);
        assert_eq!(e.mem.u32(state + TS_OPTIMAL_LENGTH), 16);
    }

    #[test]
    fn heap_sift_orders_by_frequency_then_depth() {
        let (mut e, state) = state_engine();
        let tree = e.mem.alloc(0x20);
        for (n, f) in [7u16, 3, 3, 1].iter().enumerate() {
            set_entry(&mut e, tree, n as u32, *f, 0);
        }
        e.mem.set_u8(state + TS_DEPTH + 1, 2);
        for (i, node) in [0i32, 1, 2, 3].iter().enumerate() {
            set_heap_at(&mut e, state, i as i32 + 1, *node);
        }
        e.mem.set_i32(state + TS_HEAP_LENGTH, 4);
        e.call(0x00b4_b9a0, &args![state, tree, 1i32]);
        // Node 0 (7) moves below the smaller child: node 2 (3, depth 0) beats node 1.
        assert_eq!(
            (1..=4).map(|i| heap_at(&e, state, i)).collect::<Vec<_>>(),
            vec![2, 1, 0, 3]
        );
    }

    /// Lengths 3,3,3,3,3,0,0,0 for codes 0..=7.
    fn run_tree(e: &mut Engine, tree: u32) {
        for n in 0..8 {
            set_entry(e, tree, n, 0, if n < 5 { 3 } else { 0 });
        }
    }

    #[test]
    fn scan_counts_the_run_codes() {
        let (mut e, state) = state_engine();
        let tree = e.mem.alloc(0x40);
        run_tree(&mut e, tree);
        e.call(0x00b4_bfd0, &args![state, tree, 7i32]);
        let bl = |e: &Engine, n: u32| e.mem.u16(state + TS_BIT_LENGTH_TREE + 4 * n);
        assert_eq!(e.mem.u16(tree + 4 * 8 + 2), 0xffff);
        assert_eq!(
            [bl(&e, 3), bl(&e, 16), bl(&e, 17), bl(&e, 18)],
            [1, 1, 1, 0]
        );
    }

    #[test]
    fn send_tree_codes_the_runs() {
        let (mut e, state) = state_engine();
        let tree = e.mem.alloc(0x40);
        run_tree(&mut e, tree);
        e.mem.set_u16(tree + 4 * 8 + 2, 0xffff);
        let bl_tree = state + TS_BIT_LENGTH_TREE;
        set_entry(&mut e, bl_tree, 3, 0, 1);
        set_entry(&mut e, bl_tree, 16, 1, 2);
        set_entry(&mut e, bl_tree, 17, 3, 2);
        e.call(0x00b4_c670, &args![state, tree, 7i32]);
        let (bits, count) = packed(&[(0, 1), (1, 2), (1, 2), (3, 2), (0, 3)]);
        assert_eq!(u128::from(e.mem.u16(state + TS_BIT_BUFFER)), bits);
        assert_eq!(e.mem.u32(state + TS_BIT_COUNT), count);
    }

    #[test]
    fn all_trees_header_lengths_and_runs() {
        let (mut e, state) = state_engine();
        for (i, code) in [16u8, 17, 18, 0].iter().enumerate() {
            e.mem.set_u8(BIT_LENGTH_ORDER + i as u32, *code);
        }
        let bl_tree = state + TS_BIT_LENGTH_TREE;
        set_entry(&mut e, bl_tree, 0, 0, 1);
        set_entry(&mut e, bl_tree, 18, 1, 1);
        e.mem.set_u16(state + TS_LITERAL_TREE + 4 * 257 + 2, 0xffff);
        e.mem.set_u16(state + TS_DISTANCE_TREE + 4 + 2, 0xffff);
        e.call(0x00b4_c170, &args![state, 257i32, 1i32, 4i32]);
        let (bits, count) = packed(&[
            (0, 5),
            (0, 5),
            (0, 4),
            (0, 3),
            (0, 3),
            (1, 3),
            (1, 3),
            (1, 1),
            (127, 7),
            (1, 1),
            (108, 7),
            (0, 1),
        ]);
        assert_eq!(count, 43);
        let bytes: Vec<u8> = (0..4).map(|i| (bits >> (8 * i)) as u8).collect();
        assert_eq!(pending_bytes(&e, state), bytes);
        assert_eq!(u128::from(e.mem.u16(state + TS_BIT_BUFFER)), bits >> 32);
        assert_eq!(e.mem.u32(state + TS_BIT_COUNT), 11);
    }

    #[test]
    fn bit_length_tree_and_its_last_index() {
        let (mut e, state) = state_engine();
        let order = [
            16u8, 17, 18, 0, 8, 7, 9, 6, 10, 5, 11, 4, 12, 3, 13, 2, 14, 1, 15,
        ];
        for (i, code) in order.iter().enumerate() {
            e.mem.set_u8(BIT_LENGTH_ORDER + i as u32, *code);
        }
        let statics = descriptor(&mut e, &[(16, 2), (17, 3), (18, 7)], 19, 7);
        e.mem
            .set_u32(state + TS_BIT_LENGTH_DESCRIPTOR, state + TS_BIT_LENGTH_TREE);
        e.mem.set_u32(state + TS_BIT_LENGTH_DESCRIPTOR + 8, statics);
        run_tree(&mut e, state + TS_LITERAL_TREE);
        e.mem.set_i32(state + TS_LITERAL_DESCRIPTOR + 4, 7);
        set_entry(&mut e, state + TS_DISTANCE_TREE, 0, 0, 1);
        e.mem.set_i32(state + TS_DISTANCE_DESCRIPTOR + 4, 0);
        assert_eq!(e.call(0x00b4_bf10, &args![state]).i32(), 17);
        let bl = |e: &Engine, n: u32| e.mem.u16(state + TS_BIT_LENGTH_TREE + 4 * n + 2);
        assert_eq!([bl(&e, 1), bl(&e, 3), bl(&e, 16), bl(&e, 17)], [2, 2, 2, 2]);
        assert_eq!(e.mem.u32(state + TS_OPTIMAL_LENGTH), 13 + 68);
    }

    #[test]
    fn compress_block_sends_literals_matches_and_the_end() {
        let (mut e, state) = state_engine();
        e.mem.set_u8(LENGTH_CODE + 10, 9);
        e.mem.set_u32(LENGTH_EXTRA_BITS + 4 * 9, 1);
        e.mem.set_u32(LENGTH_BASES + 4 * 9, 10);
        e.mem.set_u8(DISTANCE_CODE + 4, 4);
        e.mem.set_u32(DISTANCE_EXTRA_BITS + 4 * 4, 1);
        e.mem.set_u32(DISTANCE_BASES + 4 * 4, 4);
        let distances = e.mem.alloc(8);
        let literals = e.mem.alloc(8);
        e.mem.set_u16(distances + 2, 5);
        e.mem.set_u8(literals, b'A');
        e.mem.set_u8(literals + 1, 10);
        e.mem.set_u32(state + TS_DISTANCE_BUFFER, distances);
        e.mem.set_u32(state + TS_LITERAL_BUFFER, literals);
        e.mem.set_u32(state + TS_LAST_LITERAL, 2);
        let ltree = e.mem.alloc(0x500);
        let dtree = e.mem.alloc(0x80);
        set_entry(&mut e, ltree, u32::from(b'A'), 2, 2);
        set_entry(&mut e, ltree, 266, 1, 1);
        set_entry(&mut e, ltree, 256, 3, 3);
        set_entry(&mut e, dtree, 4, 0, 1);
        e.call(0x00b4_d210, &args![state, ltree, dtree]);
        assert_eq!(e.mem.u16(state + TS_BIT_BUFFER), 0xc6);
        assert_eq!(e.mem.u32(state + TS_BIT_COUNT), 9);
        assert_eq!(e.mem.u32(state + TS_LAST_END_LENGTH), 3);
    }

    #[test]
    fn flush_block_stores_or_sends_static() {
        let (mut e, state) = state_engine();
        let data = e.mem.alloc(4);
        e.mem.set_u8(data, b'h');
        e.mem.set_u8(data + 1, b'i');
        e.call(0x00b4_b250, &args![state, data, 2u32, 1u32]);
        assert_eq!(pending_bytes(&e, state), [1, 2, 0, 0xfd, 0xff, b'h', b'i']);
        assert_eq!(e.mem.u16(state + TS_LITERAL_TREE + 4 * 0x100), 1);
        // Without the bytes: a static block with only the end code.
        let (mut e, state) = state_engine();
        set_entry(&mut e, STATIC_LITERAL_TREE, 256, 0, 7);
        e.call(0x00b4_b250, &args![state, 0u32, 2u32, 0u32]);
        assert_eq!(e.mem.u16(state + TS_BIT_BUFFER), 2);
        assert_eq!(e.mem.u32(state + TS_BIT_COUNT), 10);
        assert_eq!(e.mem.u32(state + TS_LAST_END_LENGTH), 7);
    }

    #[test]
    fn compress_runs_one_deflate() {
        let mut e = Engine::new();
        e.register(DEFLATE_INIT, |e, a| {
            // `compress` passes level -1, the later calls level 6.
            assert!(a[1] as i32 == -1 || a[1] == 6);
            assert_eq!(a[2..], [COMPRESS_VERSION, 0x38]);
            e.mem.set_u32(a[0] + 0x14, 0x33);
            answer(0)
        });
        e.register(DEFLATE, |e, a| answer(e.mem.u32(a[0] + 4) & 1));
        e.register(DEFLATE_END, |_, _| answer(0));
        let size = e.mem.alloc(4);
        e.mem.set_u32(size, 0x100);
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x00b4_de80, &args![0x5000u32, size, 0x6000u32, 9u32])
                .i32(),
            0
        );
        assert_eq!(e.mem.u32(size), 0x33);
        assert_eq!(calls_to(&e, DEFLATE).len(), 1);
        // A deflate that answers 0 (output full) gives -5 after taking the stream down.
        assert_eq!(
            e.call(0x00b4_ddb0, &args![0x5000u32, size, 0x6000u32, 8u32, 6i32])
                .i32(),
            -5
        );
        assert_eq!(calls_to(&e, DEFLATE_END).len(), 2);
        e.register(DEFLATE_INIT, |_, _| answer((-2i32) as u32));
        assert_eq!(
            e.call(0x00b4_ddb0, &args![0x5000u32, size, 0x6000u32, 8u32, 6i32])
                .i32(),
            -2
        );
    }
}

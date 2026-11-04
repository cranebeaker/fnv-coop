//! `(unplaced)/run_00b45c00` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// CRT `malloc` (`00ecd1c7`), `free` (`00ecd291`) and `memcpy` (`00ec44d0`),
/// all `cdecl`.
const MALLOC: u32 = 0x00ec_d1c7;
const FREE: u32 = 0x00ec_d291;
const MEMCPY: u32 = 0x00ec_44d0;

// Translated from 00b49020 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the default allocator `00b43e70` installs
/// (`cdecl`, three words: an opaque word, a count and a size). With a
/// non-zero opaque word the count goes through `size - size + count`; then
/// `malloc(count * size)` is answered.
pub fn fn_00b49020(e: &mut Engine, opaque: u32, count: u32, size: u32) -> u32 {
    let mut count = count;
    if opaque != 0 {
        count = size.wrapping_sub(size).wrapping_add(count);
    }
    e.call(MALLOC, &args![count.wrapping_mul(size)]).u32()
}

// Translated from 00b49050 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the default free (`cdecl`, two words: an
/// opaque word, not read, and the block): `free(block)`.
pub fn fn_00b49050(e: &mut Engine, _unused_opaque: u32, block: u32) {
    e.call(FREE, &args![block]);
}

// Translated from 00b45db0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, one word: the stream record): takes
/// the stream down. Without a stream, a state (`+0x1c`) or a free function
/// (`+0x24`) answers -2. Frees the state's window (`+0x2c`) when there is
/// one, then the state when it was made by the set-up (`+0x34`), both
/// through the free function with the opaque word (`+0x28`); clears `+0x1c`
/// and answers 0.
pub fn fn_00b45db0(e: &mut Engine, stream: u32) -> i32 {
    if stream == 0 || e.mem.u32(stream + 0x1c) == 0 || e.mem.u32(stream + 0x24) == 0 {
        return -2;
    }
    let state = e.mem.u32(stream + 0x1c);
    let window = e.mem.u32(state + 0x2c);
    if window != 0 {
        let free = e.mem.u32(stream + 0x24);
        let opaque = e.mem.u32(stream + 0x28);
        e.call(free, &args![opaque, window]);
    }
    if e.mem.u32(stream + 0x34) != 0 {
        let free = e.mem.u32(stream + 0x24);
        let opaque = e.mem.u32(stream + 0x28);
        let state = e.mem.u32(stream + 0x1c);
        e.call(free, &args![opaque, state]);
    }
    e.mem.set_u32(stream + 0x1c, 0);
    0
}

// Translated from 00b45c00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, two words: the stream record and the
/// output position): keeps the last output in the state's window. The window
/// (`+0x2c`, `1 << bits` bytes from the allocator with the opaque word) is
/// made on first use (none answers 1); its size (`+0x20`) set with the write
/// position (`+0x28`) and fill (`+0x24`) cleared. The new output is the
/// position minus the stream's `+0x10`: at least a window's worth copies the
/// window's size from the output's end, write position 0, fill the size;
/// less is copied at the write position up to the window's end and the rest
/// wraps to its start (then the write position is the rest and the fill the
/// size), or the write position moves on (wrapping at the size) and the fill
/// grows while below the size. Copies go through `memcpy`; answers 0.
pub fn fn_00b45c00(e: &mut Engine, stream: u32, position: u32) -> i32 {
    let state = e.mem.u32(stream + 0x1c);
    if e.mem.u32(state + 0x2c) == 0 {
        let allocate = e.mem.u32(stream + 0x20);
        let opaque = e.mem.u32(stream + 0x28);
        let size = 1u32.wrapping_shl(e.mem.u32(state + 0x1c));
        let window = e.call(allocate, &args![opaque, size, 1u32]).u32();
        e.mem.set_u32(state + 0x2c, window);
        if e.mem.u32(state + 0x2c) == 0 {
            return 1;
        }
    }
    if e.mem.u32(state + 0x20) == 0 {
        let size = 1u32.wrapping_shl(e.mem.u32(state + 0x1c));
        e.mem.set_u32(state + 0x20, size);
        e.mem.set_u32(state + 0x28, 0);
        e.mem.set_u32(state + 0x24, 0);
    }
    let copy = position.wrapping_sub(e.mem.u32(stream + 0x10));
    let size = e.mem.u32(state + 0x20);
    let window = e.mem.u32(state + 0x2c);
    let end = e.mem.u32(stream + 0xc);
    if copy >= size {
        e.call(MEMCPY, &args![window, end.wrapping_sub(size), size]);
        e.mem.set_u32(state + 0x28, 0);
        let size = e.mem.u32(state + 0x20);
        e.mem.set_u32(state + 0x24, size);
        return 0;
    }
    let write = e.mem.u32(state + 0x28);
    let mut part = size.wrapping_sub(write);
    if part > copy {
        part = copy;
    }
    e.call(
        MEMCPY,
        &args![window.wrapping_add(write), end.wrapping_sub(copy), part],
    );
    let rest = copy.wrapping_sub(part);
    if rest != 0 {
        let window = e.mem.u32(state + 0x2c);
        let end = e.mem.u32(stream + 0xc);
        e.call(MEMCPY, &args![window, end.wrapping_sub(rest), rest]);
        e.mem.set_u32(state + 0x28, rest);
        let size = e.mem.u32(state + 0x20);
        e.mem.set_u32(state + 0x24, size);
    } else {
        let write = e.mem.u32(state + 0x28).wrapping_add(part);
        e.mem.set_u32(state + 0x28, write);
        if e.mem.u32(state + 0x28) == e.mem.u32(state + 0x20) {
            e.mem.set_u32(state + 0x28, 0);
        }
        if e.mem.u32(state + 0x24) < e.mem.u32(state + 0x20) {
            let fill = e.mem.u32(state + 0x24).wrapping_add(part);
            e.mem.set_u32(state + 0x24, fill);
        }
    }
    0
}

/// The fixed code tables `fn_00b45bd0` installs (`.rdata`): 512 length
/// codes, then 32 distance codes. Each code is a word: kind bits (byte 0),
/// bit count (byte 1) and value (high half).
const FIXED_LENGTH_CODES: u32 = 0x010a_5a98;
const FIXED_DISTANCE_CODES: u32 = 0x010a_6298;

// Translated from 00b45bd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, one word: the decompressor state):
/// points the state's length codes (`+0x44`) at the fixed table `010a5a98`
/// with 9 index bits (`+0x4c`), and its distance codes (`+0x48`) at
/// `010a6298` with 5 (`+0x50`).
pub fn fn_00b45bd0(e: &mut Engine, state: u32) {
    e.mem.set_u32(state + 0x44, FIXED_LENGTH_CODES);
    e.mem.set_u32(state + 0x4c, 9);
    e.mem.set_u32(state + 0x48, FIXED_DISTANCE_CODES);
    e.mem.set_u32(state + 0x50, 5);
}

/// `00b4a070` (`cdecl`, three words: a running value, a buffer and its
/// length; answers the new value): the checksum `inflate` keeps over the
/// bytes of a header of the `1f 8b` kind (not translated here).
const HEADER_CHECKSUM: u32 = 0x00b4_a070;
/// `00b49e20` (`run_00b49e20.rs`, `cdecl`, same three words): the sum
/// checksum of the stream data; a null buffer answers its start value 1.
const DATA_CHECKSUM: u32 = 0x00b4_9e20;
/// `00b497d0` (`cdecl`, six words: the table kind 0/1/2, the code lengths,
/// their count, the address of the next free code slot, the address of the
/// index bit count, a work area; answers 0 when the table was built).
const TABLE_BUILDER: u32 = 0x00b4_97d0;
/// `00b49070` (`cdecl`, two words: the stream record and the output left at
/// the start of the call): the decoder `inflate` hands a block's codes to
/// while at least 6 input bytes and 258 output bytes are left.
const FAST_DECODER: u32 = 0x00b4_9070;

/// The order the code-length code lengths of a coded block arrive in (19
/// `u16` symbol numbers, `.rdata`).
const CODE_LENGTH_ORDER: u32 = 0x010a_6318;

/// The messages `inflate` leaves in the stream's message word (`+0x18`).
const MSG_INCORRECT_HEADER_CHECK: u32 = 0x011a_d4b4;
const MSG_UNKNOWN_METHOD_IN_HEADER: u32 = 0x011a_d4cc;
const MSG_INVALID_WINDOW_SIZE: u32 = 0x011a_d4e8;
const MSG_UNKNOWN_METHOD_IN_FLAGS: u32 = 0x011a_d4fc;
const MSG_UNKNOWN_HEADER_FLAGS: u32 = 0x011a_d518;
const MSG_HEADER_CRC_MISMATCH: u32 = 0x011a_d534;
const MSG_INVALID_BLOCK_TYPE: u32 = 0x011a_d548;
const MSG_INVALID_STORED_LENGTHS: u32 = 0x011a_d55c;
const MSG_TOO_MANY_SYMBOLS: u32 = 0x011a_d57c;
const MSG_INVALID_CODE_LENGTHS: u32 = 0x011a_d5a0;
const MSG_REPEAT_WITHOUT_LENGTH: u32 = 0x011a_d5bc;
const MSG_REPEAT_TOO_LONG: u32 = 0x011a_d5d8;
const MSG_INVALID_LENGTH_SET: u32 = 0x011a_d5f4;
const MSG_INVALID_DISTANCE_SET: u32 = 0x011a_d610;
const MSG_INVALID_LENGTH_CODE: u32 = 0x011a_d628;
const MSG_INVALID_DISTANCE_CODE: u32 = 0x011a_d644;
const MSG_DISTANCE_TOO_FAR: u32 = 0x011a_d65c;
const MSG_INCORRECT_LENGTH: u32 = 0x011a_d67c;

/// The stream record's words `inflate` uses.
const STREAM_INPUT: u32 = 0x00;
const STREAM_INPUT_LEFT: u32 = 0x04;
const STREAM_TOTAL_IN: u32 = 0x08;
const STREAM_OUTPUT: u32 = 0x0c;
const STREAM_OUTPUT_LEFT: u32 = 0x10;
const STREAM_TOTAL_OUT: u32 = 0x14;
const STREAM_MESSAGE: u32 = 0x18;
const STREAM_STATE: u32 = 0x1c;
/// Summary `inflate` leaves on return: bits held, 0x40 on the last block,
/// 0x80 between blocks.
const STREAM_BLOCK_SUMMARY: u32 = 0x2c;
const STREAM_CHECKSUM: u32 = 0x30;

/// The decompressor state's words `inflate` uses (the window words
/// `+0x1c`..`+0x2c` are the ones `fn_00b45c00` keeps).
const STATE_MODE: u32 = 0x00;
const STATE_LAST_BLOCK: u32 = 0x04;
/// Header kinds accepted: bit 0 the `78 xx` kind, bit 1 the `1f 8b` kind.
const STATE_WRAP: u32 = 0x08;
const STATE_HAVE_DICTIONARY: u32 = 0x0c;
const STATE_HEADER_FLAGS: u32 = 0x10;
const STATE_CHECKSUM: u32 = 0x14;
const STATE_TOTAL: u32 = 0x18;
const STATE_WINDOW_BITS: u32 = 0x1c;
const STATE_WINDOW_SIZE: u32 = 0x20;
const STATE_WINDOW_FILL: u32 = 0x24;
const STATE_WINDOW_WRITE: u32 = 0x28;
const STATE_WINDOW: u32 = 0x2c;
const STATE_HOLD: u32 = 0x30;
const STATE_BITS: u32 = 0x34;
const STATE_LENGTH: u32 = 0x38;
const STATE_DISTANCE: u32 = 0x3c;
const STATE_EXTRA: u32 = 0x40;
const STATE_LENGTH_CODES: u32 = 0x44;
const STATE_DISTANCE_CODES: u32 = 0x48;
const STATE_LENGTH_INDEX_BITS: u32 = 0x4c;
const STATE_DISTANCE_INDEX_BITS: u32 = 0x50;
const STATE_CODE_COUNT: u32 = 0x54;
const STATE_LENGTH_COUNT: u32 = 0x58;
const STATE_DISTANCE_COUNT: u32 = 0x5c;
const STATE_HAVE: u32 = 0x60;
const STATE_NEXT_CODE: u32 = 0x64;
/// 320 `u16` code lengths.
const STATE_LENGTHS: u32 = 0x68;
const STATE_WORK: u32 = 0x2e8;
const STATE_CODES: u32 = 0x528;

/// The modes `inflate` names more than once.
const MODE_BLOCK_START: u32 = 0x0b;
const MODE_BLOCK_HEADER: u32 = 0x0c;
const MODE_CODES: u32 = 0x12;
const MODE_BAD: u32 = 0x1b;
const MODE_NO_MEMORY: u32 = 0x1c;

/// `inflate`'s working copies of the input position and count and of the
/// bit buffer (the stream's `+0x00`/`+0x04`, the state's `+0x30`/`+0x34`).
struct Input {
    next: u32,
    have: u32,
    hold: u32,
    bits: u32,
}

impl Input {
    /// Takes one input byte into the bit buffer above the bits held; false
    /// (nothing taken) when no input is left.
    fn pull(&mut self, e: &Engine) -> bool {
        if self.have == 0 {
            return false;
        }
        self.have -= 1;
        let byte = u32::from(e.mem.u8(self.next));
        self.hold = byte.wrapping_shl(self.bits).wrapping_add(self.hold);
        self.next = self.next.wrapping_add(1);
        self.bits = self.bits.wrapping_add(8);
        true
    }

    /// Pulls bytes while fewer than `count` bits are held; false when the
    /// input ran out first.
    fn need(&mut self, e: &Engine, count: u32) -> bool {
        while self.bits < count {
            if !self.pull(e) {
                return false;
            }
        }
        true
    }

    /// Drops the low `count` bits.
    fn drop_bits(&mut self, count: u32) {
        self.hold = self.hold.wrapping_shr(count);
        self.bits = self.bits.wrapping_sub(count);
    }

    /// Empties the bit buffer.
    fn clear(&mut self) {
        self.hold = 0;
        self.bits = 0;
    }

    /// The low `count` bits held.
    fn low(&self, count: u32) -> u32 {
        1u32.wrapping_shl(count).wrapping_sub(1) & self.hold
    }
}

/// A code word's kind bits, bit count and value.
fn code_kind(code: u32) -> u32 {
    code & 0xff
}
fn code_bits(code: u32) -> u32 {
    (code >> 8) & 0xff
}
fn code_value(code: u32) -> u32 {
    code >> 16
}

/// The code word at `index` of the table at `table`.
fn code_at(e: &Engine, table: u32, index: u32) -> u32 {
    e.mem.u32(table.wrapping_add(index.wrapping_mul(4)))
}

/// Writes the working copies back: output position and count to the
/// stream, input position and count to the stream, the bit buffer to the
/// state.
fn inflate_store(e: &mut Engine, stream: u32, state: u32, put: u32, left: u32, input: &Input) {
    e.mem.set_u32(stream + STREAM_OUTPUT, put);
    e.mem.set_u32(stream + STREAM_OUTPUT_LEFT, left);
    e.mem.set_u32(stream + STREAM_INPUT, input.next);
    e.mem.set_u32(stream + STREAM_INPUT_LEFT, input.have);
    e.mem.set_u32(state + STATE_HOLD, input.hold);
    e.mem.set_u32(state + STATE_BITS, input.bits);
}

/// Leaves `message` in the stream and puts the state in the failed mode.
fn inflate_fail(e: &mut Engine, stream: u32, state: u32, message: u32) {
    e.mem.set_u32(stream + STREAM_MESSAGE, message);
    e.mem.set_u32(state + STATE_MODE, MODE_BAD);
}

/// Runs the header checksum over the low `count` bytes of `hold`, stored
/// low byte first in a stack buffer as `inflate` stores them.
fn inflate_checksum_held(e: &mut Engine, state: u32, hold: u32, count: u32) {
    let sum = e.with_stack(4, |e, buffer| {
        for i in 0..count {
            e.mem.set_u8(buffer.addr() + i, (hold >> (8 * i)) as u8);
        }
        let running = e.mem.u32(state + STATE_CHECKSUM);
        e.call(HEADER_CHECKSUM, &args![running, buffer.addr(), count])
            .u32()
    });
    e.mem.set_u32(state + STATE_CHECKSUM, sum);
}

/// Decodes one code of the table in the state word `table` (index bit count
/// in `index_bits`) the way `inflate`'s length and distance modes do: the
/// first-level code once enough bits are held, then, when its kind bits
/// have no high nibble (and, with `nonzero_kind`, are not zero), the code of
/// the sub-table it points to, dropping the first code's bits. The answered
/// code's own bits are not dropped. `None` when the input ran out.
fn inflate_decode(
    e: &Engine,
    state: u32,
    table: u32,
    index_bits: u32,
    input: &mut Input,
    nonzero_kind: bool,
) -> Option<u32> {
    let first = loop {
        let index = input.low(e.mem.u32(state + index_bits));
        let code = code_at(e, e.mem.u32(state + table), index);
        if code_bits(code) <= input.bits {
            break code;
        }
        if !input.pull(e) {
            return None;
        }
    };
    if (nonzero_kind && code_kind(first) == 0) || code_kind(first) & 0xf0 != 0 {
        return Some(first);
    }
    let code = loop {
        let index = code_value(first).wrapping_add(
            input
                .low(code_bits(first) + code_kind(first))
                .wrapping_shr(code_bits(first)),
        );
        let code = code_at(e, e.mem.u32(state + table), index);
        if code_bits(first) + code_bits(code) <= input.bits {
            break code;
        }
        if !input.pull(e) {
            return None;
        }
    };
    input.drop_bits(code_bits(first));
    Some(code)
}

// Translated from 00b44000 (decompiled, FalloutNV.exe 1.4.0.525)
/// `inflate` (engine map; unit `zLib/inflate.c`; `cdecl`, two words: the
/// stream record and the flush kind). Answers -2 without a stream, a state
/// (`+0x1c`) or an output (`+0x0c`), or with input left but no input
/// pointer. Otherwise runs the state's mode machine over the stream's input
/// and output, keeping working copies of the input, output and bit buffer:
/// header checks (`78 xx` and `1f 8b` kinds, with their flags, extra field,
/// name, comment and header checksum), the dictionary id (answers 2 when no
/// dictionary is set), block headers, stored blocks (copied with `memcpy`),
/// coded blocks (code-length, length and distance tables through
/// `00b497d0`, the fixed ones through `fn_00b45bd0`), literal and match
/// output (through `00b49070` while at least 6 input and 258 output bytes
/// are left), and the trailer. A bad stream leaves a message (`+0x18`) and
/// answers -3; the end answers 1. When the input or output runs out, or
/// after the end, the copies are written back, the window is updated
/// (`fn_00b45c00`, -4 and the no-memory mode when it fails), the totals
/// grow and the block summary (`+0x2c`) is set; no progress (or flush kind
/// 4) with nothing else to report answers -5.
///
/// As the exe has it: the trailer's checksum words are read but never
/// compared, and the data checksum is cleared (state `+0x14`, stream
/// `+0x30`) wherever output was counted; the name and comment fields go
/// through the header checksum only under flag 0x2000, which the flags mode
/// refuses; and a bad repeat in the code lengths still goes on to build
/// the tables, whose success puts the codes mode over the failed one.
pub fn inflate(e: &mut Engine, stream: u32, flush: i32) -> i32 {
    if stream == 0
        || e.mem.u32(stream + STREAM_STATE) == 0
        || e.mem.u32(stream + STREAM_OUTPUT) == 0
        || (e.mem.u32(stream + STREAM_INPUT) == 0 && e.mem.u32(stream + STREAM_INPUT_LEFT) != 0)
    {
        return -2;
    }
    let state = e.mem.u32(stream + STREAM_STATE);
    if e.mem.u32(state + STATE_MODE) == MODE_BLOCK_START {
        e.mem.set_u32(state + STATE_MODE, MODE_BLOCK_HEADER);
    }
    let mut put = e.mem.u32(stream + STREAM_OUTPUT);
    let mut left = e.mem.u32(stream + STREAM_OUTPUT_LEFT);
    let mut input = Input {
        next: e.mem.u32(stream + STREAM_INPUT),
        have: e.mem.u32(stream + STREAM_INPUT_LEFT),
        hold: e.mem.u32(state + STATE_HOLD),
        bits: e.mem.u32(state + STATE_BITS),
    };
    let input_start = input.have;
    let mut output_start = left;
    let mut ret = 0i32;
    'run: loop {
        let mode = e.mem.u32(state + STATE_MODE);
        match mode {
            // Header.
            0x00 => {
                if e.mem.u32(state + STATE_WRAP) == 0 {
                    e.mem.set_u32(state + STATE_MODE, MODE_BLOCK_HEADER);
                    continue 'run;
                }
                if !input.need(e, 0x10) {
                    break 'run;
                }
                if e.mem.u32(state + STATE_WRAP) & 2 != 0 && input.hold == 0x8b1f {
                    let start = e.call(HEADER_CHECKSUM, &args![0u32, 0u32, 0u32]).u32();
                    e.mem.set_u32(state + STATE_CHECKSUM, start);
                    inflate_checksum_held(e, state, input.hold, 2);
                    input.clear();
                    e.mem.set_u32(state + STATE_MODE, 1);
                    continue 'run;
                }
                e.mem.set_u32(state + STATE_HEADER_FLAGS, 0);
                if e.mem.u32(state + STATE_WRAP) & 1 == 0
                    || ((input.hold & 0xff) << 8).wrapping_add(input.hold >> 8) % 0x1f != 0
                {
                    inflate_fail(e, stream, state, MSG_INCORRECT_HEADER_CHECK);
                    continue 'run;
                }
                if input.hold & 0xf != 8 {
                    inflate_fail(e, stream, state, MSG_UNKNOWN_METHOD_IN_HEADER);
                    continue 'run;
                }
                input.drop_bits(4);
                if (input.hold & 0xf) + 8 > e.mem.u32(state + STATE_WINDOW_BITS) {
                    inflate_fail(e, stream, state, MSG_INVALID_WINDOW_SIZE);
                    continue 'run;
                }
                let start = e.call(DATA_CHECKSUM, &args![0u32, 0u32, 0u32]).u32();
                e.mem.set_u32(state + STATE_CHECKSUM, start);
                let check = e.mem.u32(state + STATE_CHECKSUM);
                e.mem.set_u32(stream + STREAM_CHECKSUM, check);
                let next = if input.hold & 0x200 != 0 {
                    9
                } else {
                    MODE_BLOCK_START
                };
                e.mem.set_u32(state + STATE_MODE, next);
                input.clear();
            }
            // Header flags (method byte and flag byte).
            0x01 => {
                if !input.need(e, 0x10) {
                    break 'run;
                }
                e.mem.set_u32(state + STATE_HEADER_FLAGS, input.hold);
                if e.mem.u32(state + STATE_HEADER_FLAGS) & 0xff != 8 {
                    inflate_fail(e, stream, state, MSG_UNKNOWN_METHOD_IN_FLAGS);
                    continue 'run;
                }
                if e.mem.u32(state + STATE_HEADER_FLAGS) & 0xe000 != 0 {
                    inflate_fail(e, stream, state, MSG_UNKNOWN_HEADER_FLAGS);
                    continue 'run;
                }
                if e.mem.u32(state + STATE_HEADER_FLAGS) & 0x200 != 0 {
                    inflate_checksum_held(e, state, input.hold, 2);
                }
                input.clear();
                e.mem.set_u32(state + STATE_MODE, 2);
            }
            // Header time word.
            0x02 => {
                if !input.need(e, 0x20) {
                    break 'run;
                }
                if e.mem.u32(state + STATE_HEADER_FLAGS) & 0x200 != 0 {
                    inflate_checksum_held(e, state, input.hold, 4);
                }
                input.clear();
                e.mem.set_u32(state + STATE_MODE, 3);
            }
            // Header extra-flags and system bytes.
            0x03 => {
                if !input.need(e, 0x10) {
                    break 'run;
                }
                if e.mem.u32(state + STATE_HEADER_FLAGS) & 0x200 != 0 {
                    inflate_checksum_held(e, state, input.hold, 2);
                }
                input.clear();
                e.mem.set_u32(state + STATE_MODE, 4);
            }
            // Extra field length.
            0x04 => {
                if e.mem.u32(state + STATE_HEADER_FLAGS) & 0x400 != 0 {
                    if !input.need(e, 0x10) {
                        break 'run;
                    }
                    e.mem.set_u32(state + STATE_LENGTH, input.hold);
                    if e.mem.u32(state + STATE_HEADER_FLAGS) & 0x200 != 0 {
                        inflate_checksum_held(e, state, input.hold, 2);
                    }
                    input.clear();
                }
                e.mem.set_u32(state + STATE_MODE, 5);
            }
            // Extra field.
            0x05 => {
                if e.mem.u32(state + STATE_HEADER_FLAGS) & 0x400 != 0 {
                    let mut copy = e.mem.u32(state + STATE_LENGTH);
                    if copy > input.have {
                        copy = input.have;
                    }
                    if copy != 0 {
                        if e.mem.u32(state + STATE_HEADER_FLAGS) & 0x200 != 0 {
                            let running = e.mem.u32(state + STATE_CHECKSUM);
                            let sum = e
                                .call(HEADER_CHECKSUM, &args![running, input.next, copy])
                                .u32();
                            e.mem.set_u32(state + STATE_CHECKSUM, sum);
                        }
                        input.have = input.have.wrapping_sub(copy);
                        input.next = input.next.wrapping_add(copy);
                        let length = e.mem.u32(state + STATE_LENGTH).wrapping_sub(copy);
                        e.mem.set_u32(state + STATE_LENGTH, length);
                    }
                    if e.mem.u32(state + STATE_LENGTH) != 0 {
                        break 'run;
                    }
                }
                e.mem.set_u32(state + STATE_MODE, 6);
            }
            // Name (0x800) and comment (0x1000): bytes up to a zero.
            0x06 | 0x07 => {
                let flag = if mode == 0x06 { 0x800 } else { 0x1000 };
                if e.mem.u32(state + STATE_HEADER_FLAGS) & flag != 0 {
                    if input.have == 0 {
                        break 'run;
                    }
                    let mut copy = 0u32;
                    let last = loop {
                        let byte = u32::from(e.mem.u8(input.next.wrapping_add(copy)));
                        copy = copy.wrapping_add(1);
                        if byte == 0 || copy >= input.have {
                            break byte;
                        }
                    };
                    if e.mem.u32(state + STATE_HEADER_FLAGS) & 0x2000 != 0 {
                        let running = e.mem.u32(state + STATE_CHECKSUM);
                        let sum = e
                            .call(HEADER_CHECKSUM, &args![running, input.next, copy])
                            .u32();
                        e.mem.set_u32(state + STATE_CHECKSUM, sum);
                    }
                    input.have = input.have.wrapping_sub(copy);
                    input.next = input.next.wrapping_add(copy);
                    if last != 0 {
                        break 'run;
                    }
                }
                e.mem.set_u32(state + STATE_MODE, mode + 1);
            }
            // Header checksum.
            0x08 => {
                if e.mem.u32(state + STATE_HEADER_FLAGS) & 0x200 != 0 {
                    if !input.need(e, 0x10) {
                        break 'run;
                    }
                    if input.hold != e.mem.u32(state + STATE_CHECKSUM) & 0xffff {
                        inflate_fail(e, stream, state, MSG_HEADER_CRC_MISMATCH);
                        continue 'run;
                    }
                    input.clear();
                }
                let start = e.call(HEADER_CHECKSUM, &args![0u32, 0u32, 0u32]).u32();
                e.mem.set_u32(state + STATE_CHECKSUM, start);
                let check = e.mem.u32(state + STATE_CHECKSUM);
                e.mem.set_u32(stream + STREAM_CHECKSUM, check);
                e.mem.set_u32(state + STATE_MODE, MODE_BLOCK_START);
            }
            // Dictionary id (high byte first).
            0x09 => {
                if !input.need(e, 0x20) {
                    break 'run;
                }
                let hold = input.hold;
                let id = ((hold >> 24) & 0xff)
                    .wrapping_add((hold >> 8) & 0xff00)
                    .wrapping_add((hold & 0xff00) << 8)
                    .wrapping_add((hold & 0xff) << 24);
                e.mem.set_u32(state + STATE_CHECKSUM, id);
                let check = e.mem.u32(state + STATE_CHECKSUM);
                e.mem.set_u32(stream + STREAM_CHECKSUM, check);
                input.clear();
                e.mem.set_u32(state + STATE_MODE, 0x0a);
            }
            // Dictionary wanted.
            0x0a => {
                if e.mem.u32(state + STATE_HAVE_DICTIONARY) == 0 {
                    inflate_store(e, stream, state, put, left, &input);
                    return 2;
                }
                let start = e.call(DATA_CHECKSUM, &args![0u32, 0u32, 0u32]).u32();
                e.mem.set_u32(state + STATE_CHECKSUM, start);
                let check = e.mem.u32(state + STATE_CHECKSUM);
                e.mem.set_u32(stream + STREAM_CHECKSUM, check);
                e.mem.set_u32(state + STATE_MODE, MODE_BLOCK_START);
            }
            // Block start (stops here for flush kind 5) and block header.
            0x0b | 0x0c => {
                if mode == MODE_BLOCK_START && flush == 5 {
                    break 'run;
                }
                if e.mem.u32(state + STATE_LAST_BLOCK) != 0 {
                    input.drop_bits(input.bits & 7);
                    e.mem.set_u32(state + STATE_MODE, 0x18);
                    continue 'run;
                }
                if !input.need(e, 3) {
                    break 'run;
                }
                e.mem.set_u32(state + STATE_LAST_BLOCK, input.hold & 1);
                input.drop_bits(1);
                // The exe's switch on the two type bits; its default (above
                // 3) cannot be reached and skips to the same drop.
                match input.hold & 3 {
                    0 => e.mem.set_u32(state + STATE_MODE, 0x0d),
                    1 => {
                        fn_00b45bd0(e, state);
                        e.mem.set_u32(state + STATE_MODE, MODE_CODES);
                    }
                    2 => e.mem.set_u32(state + STATE_MODE, 0x0f),
                    _ => inflate_fail(e, stream, state, MSG_INVALID_BLOCK_TYPE),
                }
                input.drop_bits(2);
            }
            // Stored block lengths.
            0x0d => {
                input.drop_bits(input.bits & 7);
                if !input.need(e, 0x20) {
                    break 'run;
                }
                if input.hold & 0xffff != (input.hold >> 16) ^ 0xffff {
                    inflate_fail(e, stream, state, MSG_INVALID_STORED_LENGTHS);
                    continue 'run;
                }
                e.mem.set_u32(state + STATE_LENGTH, input.hold & 0xffff);
                input.clear();
                e.mem.set_u32(state + STATE_MODE, 0x0e);
            }
            // Stored block copy.
            0x0e => {
                let mut copy = e.mem.u32(state + STATE_LENGTH);
                if copy == 0 {
                    e.mem.set_u32(state + STATE_MODE, MODE_BLOCK_START);
                    continue 'run;
                }
                if copy > input.have {
                    copy = input.have;
                }
                if copy > left {
                    copy = left;
                }
                if copy == 0 {
                    break 'run;
                }
                e.call(MEMCPY, &args![put, input.next, copy]);
                input.have = input.have.wrapping_sub(copy);
                input.next = input.next.wrapping_add(copy);
                left = left.wrapping_sub(copy);
                put = put.wrapping_add(copy);
                let length = e.mem.u32(state + STATE_LENGTH).wrapping_sub(copy);
                e.mem.set_u32(state + STATE_LENGTH, length);
            }
            // Coded block counts.
            0x0f => {
                if !input.need(e, 0x0e) {
                    break 'run;
                }
                e.mem
                    .set_u32(state + STATE_LENGTH_COUNT, (input.hold & 0x1f) + 0x101);
                input.drop_bits(5);
                e.mem
                    .set_u32(state + STATE_DISTANCE_COUNT, (input.hold & 0x1f) + 1);
                input.drop_bits(5);
                e.mem
                    .set_u32(state + STATE_CODE_COUNT, (input.hold & 0xf) + 4);
                input.drop_bits(4);
                if e.mem.u32(state + STATE_LENGTH_COUNT) > 0x11e
                    || e.mem.u32(state + STATE_DISTANCE_COUNT) > 0x1e
                {
                    inflate_fail(e, stream, state, MSG_TOO_MANY_SYMBOLS);
                    continue 'run;
                }
                e.mem.set_u32(state + STATE_HAVE, 0);
                e.mem.set_u32(state + STATE_MODE, 0x10);
            }
            // Code-length code lengths, then their table.
            0x10 => {
                while e.mem.u32(state + STATE_HAVE) < e.mem.u32(state + STATE_CODE_COUNT) {
                    if !input.need(e, 3) {
                        break 'run;
                    }
                    let have = e.mem.u32(state + STATE_HAVE);
                    let symbol = u32::from(
                        e.mem
                            .u16(CODE_LENGTH_ORDER.wrapping_add(have.wrapping_mul(2))),
                    );
                    e.mem.set_u16(
                        state
                            .wrapping_add(symbol.wrapping_mul(2))
                            .wrapping_add(STATE_LENGTHS),
                        (input.hold & 7) as u16,
                    );
                    e.mem.set_u32(state + STATE_HAVE, have.wrapping_add(1));
                    input.drop_bits(3);
                }
                while e.mem.u32(state + STATE_HAVE) < 0x13 {
                    let have = e.mem.u32(state + STATE_HAVE);
                    let symbol = u32::from(
                        e.mem
                            .u16(CODE_LENGTH_ORDER.wrapping_add(have.wrapping_mul(2))),
                    );
                    e.mem.set_u16(
                        state
                            .wrapping_add(symbol.wrapping_mul(2))
                            .wrapping_add(STATE_LENGTHS),
                        0,
                    );
                    e.mem.set_u32(state + STATE_HAVE, have + 1);
                }
                e.mem.set_u32(state + STATE_NEXT_CODE, state + STATE_CODES);
                let next = e.mem.u32(state + STATE_NEXT_CODE);
                e.mem.set_u32(state + STATE_LENGTH_CODES, next);
                e.mem.set_u32(state + STATE_LENGTH_INDEX_BITS, 7);
                ret = e
                    .call(
                        TABLE_BUILDER,
                        &args![
                            0u32,
                            state + STATE_LENGTHS,
                            0x13u32,
                            state + STATE_NEXT_CODE,
                            state + STATE_LENGTH_INDEX_BITS,
                            state + STATE_WORK
                        ],
                    )
                    .i32();
                if ret != 0 {
                    inflate_fail(e, stream, state, MSG_INVALID_CODE_LENGTHS);
                    continue 'run;
                }
                e.mem.set_u32(state + STATE_HAVE, 0);
                e.mem.set_u32(state + STATE_MODE, 0x11);
            }
            // Code lengths of the length and distance codes, then their
            // tables.
            0x11 => {
                'lengths: while e.mem.u32(state + STATE_HAVE)
                    < e.mem
                        .u32(state + STATE_LENGTH_COUNT)
                        .wrapping_add(e.mem.u32(state + STATE_DISTANCE_COUNT))
                {
                    let code = loop {
                        let index = input.low(e.mem.u32(state + STATE_LENGTH_INDEX_BITS));
                        let code = code_at(e, e.mem.u32(state + STATE_LENGTH_CODES), index);
                        if code_bits(code) <= input.bits {
                            break code;
                        }
                        if !input.pull(e) {
                            break 'run;
                        }
                    };
                    let value = code_value(code);
                    if value < 0x10 {
                        if !input.need(e, code_bits(code)) {
                            break 'run;
                        }
                        input.drop_bits(code_bits(code));
                        let have = e.mem.u32(state + STATE_HAVE);
                        e.mem.set_u16(
                            state
                                .wrapping_add(have.wrapping_mul(2))
                                .wrapping_add(STATE_LENGTHS),
                            value as u16,
                        );
                        e.mem.set_u32(state + STATE_HAVE, have.wrapping_add(1));
                        continue 'lengths;
                    }
                    let (length, copy) = if value == 0x10 {
                        if !input.need(e, code_bits(code) + 2) {
                            break 'run;
                        }
                        input.drop_bits(code_bits(code));
                        let have = e.mem.u32(state + STATE_HAVE);
                        if have == 0 {
                            inflate_fail(e, stream, state, MSG_REPEAT_WITHOUT_LENGTH);
                            break 'lengths;
                        }
                        let previous = u32::from(
                            e.mem
                                .u16(state.wrapping_add(have.wrapping_mul(2)).wrapping_add(0x66)),
                        );
                        let copy = (input.hold & 3) + 3;
                        input.drop_bits(2);
                        (previous, copy)
                    } else if value == 0x11 {
                        if !input.need(e, code_bits(code) + 3) {
                            break 'run;
                        }
                        input.drop_bits(code_bits(code));
                        let copy = (input.hold & 7) + 3;
                        input.drop_bits(3);
                        (0, copy)
                    } else {
                        if !input.need(e, code_bits(code) + 7) {
                            break 'run;
                        }
                        input.drop_bits(code_bits(code));
                        let copy = (input.hold & 0x7f) + 0xb;
                        input.drop_bits(7);
                        (0, copy)
                    };
                    let total = e
                        .mem
                        .u32(state + STATE_LENGTH_COUNT)
                        .wrapping_add(e.mem.u32(state + STATE_DISTANCE_COUNT));
                    if e.mem.u32(state + STATE_HAVE).wrapping_add(copy) > total {
                        inflate_fail(e, stream, state, MSG_REPEAT_TOO_LONG);
                        break 'lengths;
                    }
                    for _ in 0..copy {
                        let have = e.mem.u32(state + STATE_HAVE);
                        e.mem.set_u16(
                            state
                                .wrapping_add(have.wrapping_mul(2))
                                .wrapping_add(STATE_LENGTHS),
                            length as u16,
                        );
                        e.mem.set_u32(state + STATE_HAVE, have.wrapping_add(1));
                    }
                }
                // The exe goes on here after a bad repeat too (see above).
                e.mem.set_u32(state + STATE_NEXT_CODE, state + STATE_CODES);
                let next = e.mem.u32(state + STATE_NEXT_CODE);
                e.mem.set_u32(state + STATE_LENGTH_CODES, next);
                e.mem.set_u32(state + STATE_LENGTH_INDEX_BITS, 9);
                let length_count = e.mem.u32(state + STATE_LENGTH_COUNT);
                ret = e
                    .call(
                        TABLE_BUILDER,
                        &args![
                            1u32,
                            state + STATE_LENGTHS,
                            length_count,
                            state + STATE_NEXT_CODE,
                            state + STATE_LENGTH_INDEX_BITS,
                            state + STATE_WORK
                        ],
                    )
                    .i32();
                if ret != 0 {
                    inflate_fail(e, stream, state, MSG_INVALID_LENGTH_SET);
                    continue 'run;
                }
                let next = e.mem.u32(state + STATE_NEXT_CODE);
                e.mem.set_u32(state + STATE_DISTANCE_CODES, next);
                e.mem.set_u32(state + STATE_DISTANCE_INDEX_BITS, 6);
                let distance_count = e.mem.u32(state + STATE_DISTANCE_COUNT);
                let length_count = e.mem.u32(state + STATE_LENGTH_COUNT);
                ret = e
                    .call(
                        TABLE_BUILDER,
                        &args![
                            2u32,
                            state
                                .wrapping_add(length_count.wrapping_mul(2))
                                .wrapping_add(STATE_LENGTHS),
                            distance_count,
                            state + STATE_NEXT_CODE,
                            state + STATE_DISTANCE_INDEX_BITS,
                            state + STATE_WORK
                        ],
                    )
                    .i32();
                if ret != 0 {
                    inflate_fail(e, stream, state, MSG_INVALID_DISTANCE_SET);
                    continue 'run;
                }
                e.mem.set_u32(state + STATE_MODE, MODE_CODES);
            }
            // Length (or literal) code.
            0x12 => {
                if input.have >= 6 && left >= 0x102 {
                    inflate_store(e, stream, state, put, left, &input);
                    e.call(FAST_DECODER, &args![stream, output_start]);
                    put = e.mem.u32(stream + STREAM_OUTPUT);
                    left = e.mem.u32(stream + STREAM_OUTPUT_LEFT);
                    input.next = e.mem.u32(stream + STREAM_INPUT);
                    input.have = e.mem.u32(stream + STREAM_INPUT_LEFT);
                    input.hold = e.mem.u32(state + STATE_HOLD);
                    input.bits = e.mem.u32(state + STATE_BITS);
                    continue 'run;
                }
                let Some(code) = inflate_decode(
                    e,
                    state,
                    STATE_LENGTH_CODES,
                    STATE_LENGTH_INDEX_BITS,
                    &mut input,
                    true,
                ) else {
                    break 'run;
                };
                input.drop_bits(code_bits(code));
                e.mem.set_u32(state + STATE_LENGTH, code_value(code));
                let kind = code_kind(code);
                if kind == 0 {
                    e.mem.set_u32(state + STATE_MODE, 0x17);
                } else if kind & 0x20 != 0 {
                    e.mem.set_u32(state + STATE_MODE, MODE_BLOCK_START);
                } else if kind & 0x40 != 0 {
                    inflate_fail(e, stream, state, MSG_INVALID_LENGTH_CODE);
                } else {
                    e.mem.set_u32(state + STATE_EXTRA, kind & 0xf);
                    e.mem.set_u32(state + STATE_MODE, 0x13);
                }
            }
            // Length extra bits.
            0x13 => {
                let extra = e.mem.u32(state + STATE_EXTRA);
                if extra != 0 {
                    if !input.need(e, extra) {
                        break 'run;
                    }
                    let length = input
                        .low(extra)
                        .wrapping_add(e.mem.u32(state + STATE_LENGTH));
                    e.mem.set_u32(state + STATE_LENGTH, length);
                    input.drop_bits(extra);
                }
                e.mem.set_u32(state + STATE_MODE, 0x14);
            }
            // Distance code.
            0x14 => {
                let Some(code) = inflate_decode(
                    e,
                    state,
                    STATE_DISTANCE_CODES,
                    STATE_DISTANCE_INDEX_BITS,
                    &mut input,
                    false,
                ) else {
                    break 'run;
                };
                input.drop_bits(code_bits(code));
                if code_kind(code) & 0x40 != 0 {
                    inflate_fail(e, stream, state, MSG_INVALID_DISTANCE_CODE);
                    continue 'run;
                }
                e.mem.set_u32(state + STATE_DISTANCE, code_value(code));
                e.mem.set_u32(state + STATE_EXTRA, code_kind(code) & 0xf);
                e.mem.set_u32(state + STATE_MODE, 0x15);
            }
            // Distance extra bits and the distance check.
            0x15 => {
                let extra = e.mem.u32(state + STATE_EXTRA);
                if extra != 0 {
                    if !input.need(e, extra) {
                        break 'run;
                    }
                    let distance = input
                        .low(extra)
                        .wrapping_add(e.mem.u32(state + STATE_DISTANCE));
                    e.mem.set_u32(state + STATE_DISTANCE, distance);
                    input.drop_bits(extra);
                }
                let reach = e
                    .mem
                    .u32(state + STATE_WINDOW_FILL)
                    .wrapping_add(output_start)
                    .wrapping_sub(left);
                if e.mem.u32(state + STATE_DISTANCE) > reach {
                    inflate_fail(e, stream, state, MSG_DISTANCE_TOO_FAR);
                    continue 'run;
                }
                e.mem.set_u32(state + STATE_MODE, 0x16);
            }
            // Match copy (from the window or the output), byte by byte.
            0x16 => {
                if left == 0 {
                    break 'run;
                }
                let made = output_start.wrapping_sub(left);
                let distance = e.mem.u32(state + STATE_DISTANCE);
                let length = e.mem.u32(state + STATE_LENGTH);
                let (mut from, mut copy) = if distance > made {
                    let mut copy = distance.wrapping_sub(made);
                    let write = e.mem.u32(state + STATE_WINDOW_WRITE);
                    let window = e.mem.u32(state + STATE_WINDOW);
                    let from = if copy > write {
                        copy = copy.wrapping_sub(write);
                        e.mem
                            .u32(state + STATE_WINDOW_SIZE)
                            .wrapping_sub(copy)
                            .wrapping_add(window)
                    } else {
                        write.wrapping_sub(copy).wrapping_add(window)
                    };
                    if copy > length {
                        copy = length;
                    }
                    (from, copy)
                } else {
                    (put.wrapping_sub(distance), length)
                };
                if copy > left {
                    copy = left;
                }
                left = left.wrapping_sub(copy);
                e.mem
                    .set_u32(state + STATE_LENGTH, length.wrapping_sub(copy));
                loop {
                    let byte = e.mem.u8(from);
                    e.mem.set_u8(put, byte);
                    put = put.wrapping_add(1);
                    from = from.wrapping_add(1);
                    copy = copy.wrapping_sub(1);
                    if copy == 0 {
                        break;
                    }
                }
                if e.mem.u32(state + STATE_LENGTH) == 0 {
                    e.mem.set_u32(state + STATE_MODE, MODE_CODES);
                }
            }
            // Literal.
            0x17 => {
                if left == 0 {
                    break 'run;
                }
                let literal = e.mem.u8(state + STATE_LENGTH);
                e.mem.set_u8(put, literal);
                put = put.wrapping_add(1);
                left -= 1;
                e.mem.set_u32(state + STATE_MODE, MODE_CODES);
            }
            // Trailer checksum word (read, not compared).
            0x18 => {
                if e.mem.u32(state + STATE_WRAP) != 0 {
                    if !input.need(e, 0x20) {
                        break 'run;
                    }
                    output_start = output_start.wrapping_sub(left);
                    let total_out = e
                        .mem
                        .u32(stream + STREAM_TOTAL_OUT)
                        .wrapping_add(output_start);
                    e.mem.set_u32(stream + STREAM_TOTAL_OUT, total_out);
                    let total = e.mem.u32(state + STATE_TOTAL).wrapping_add(output_start);
                    e.mem.set_u32(state + STATE_TOTAL, total);
                    if output_start != 0 {
                        e.mem.set_u32(state + STATE_CHECKSUM, 0);
                        e.mem.set_u32(stream + STREAM_CHECKSUM, 0);
                    }
                    output_start = left;
                    input.clear();
                }
                e.mem.set_u32(state + STATE_MODE, 0x19);
            }
            // Trailer length word (with header flags only).
            0x19 => {
                if e.mem.u32(state + STATE_WRAP) != 0 && e.mem.u32(state + STATE_HEADER_FLAGS) != 0
                {
                    if !input.need(e, 0x20) {
                        break 'run;
                    }
                    if input.hold != e.mem.u32(state + STATE_TOTAL) {
                        inflate_fail(e, stream, state, MSG_INCORRECT_LENGTH);
                        continue 'run;
                    }
                    input.clear();
                }
                e.mem.set_u32(state + STATE_MODE, 0x1a);
            }
            // Done.
            0x1a => {
                ret = 1;
                break 'run;
            }
            // Failed.
            0x1b => {
                ret = -3;
                break 'run;
            }
            MODE_NO_MEMORY => return -4,
            _ => return -2,
        }
    }
    inflate_store(e, stream, state, put, left, &input);
    if (e.mem.u32(state + STATE_WINDOW_SIZE) != 0
        || ((e.mem.u32(state + STATE_MODE) as i32) < 0x18
            && output_start != e.mem.u32(stream + STREAM_OUTPUT_LEFT)))
        && fn_00b45c00(e, stream, output_start) != 0
    {
        e.mem.set_u32(state + STATE_MODE, MODE_NO_MEMORY);
        return -4;
    }
    let input_used = input_start.wrapping_sub(e.mem.u32(stream + STREAM_INPUT_LEFT));
    let output_made = output_start.wrapping_sub(e.mem.u32(stream + STREAM_OUTPUT_LEFT));
    let total_in = e.mem.u32(stream + STREAM_TOTAL_IN).wrapping_add(input_used);
    e.mem.set_u32(stream + STREAM_TOTAL_IN, total_in);
    let total_out = e
        .mem
        .u32(stream + STREAM_TOTAL_OUT)
        .wrapping_add(output_made);
    e.mem.set_u32(stream + STREAM_TOTAL_OUT, total_out);
    let total = e.mem.u32(state + STATE_TOTAL).wrapping_add(output_made);
    e.mem.set_u32(state + STATE_TOTAL, total);
    if e.mem.u32(state + STATE_WRAP) != 0 && output_made != 0 {
        e.mem.set_u32(state + STATE_CHECKSUM, 0);
        e.mem.set_u32(stream + STREAM_CHECKSUM, 0);
    }
    let last = if e.mem.u32(state + STATE_LAST_BLOCK) != 0 {
        0x40
    } else {
        0
    };
    let between = if e.mem.u32(state + STATE_MODE) == MODE_BLOCK_START {
        0x80
    } else {
        0
    };
    let summary = e
        .mem
        .u32(state + STATE_BITS)
        .wrapping_add(last)
        .wrapping_add(between);
    e.mem.set_u32(stream + STREAM_BLOCK_SUMMARY, summary);
    if ((input_used == 0 && output_made == 0) || flush == 4) && ret == 0 {
        ret = -5;
    }
    ret
}

/// `00b43db0` (`cdecl`, one word: the stream record): the decompressor reset `fn_00b45f50`
/// runs (not in this file).
const INFLATE_RESET: u32 = 0x00b4_3db0;
/// The modes `fn_00b45e30`, `fn_00b45f50` and `fn_00b46170` test or set.
const MODE_DICTIONARY: u32 = 0x0a;
const MODE_STORED_LENGTHS: u32 = 0x0d;
const MODE_SYNC: u32 = 0x1d;
/// The decompressor state's size (`fn_00b461c0` copies it whole) and its sync count
/// (`+0x60`, the `have` word `fn_00b460e0` keeps between calls).
const INFLATE_STATE_SIZE: u32 = 0x1ba8;
/// The stream record's allocator, free function and opaque word.
const STREAM_ALLOCATE: u32 = 0x20;
const STREAM_FREE: u32 = 0x24;
const STREAM_OPAQUE: u32 = 0x28;

// Translated from 00b45e30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map; zlib's `inflateSetDictionary` by its body (`cdecl`, three
/// words: the stream record, a dictionary and its length). Answers -2 without a stream
/// or state, or unless the state waits for a dictionary (mode 10); -3 when the sum
/// checksum `00b49e20` of the dictionary (from its start value) differs from the one the
/// header gave (state `+0x14`); -4 (and the no-memory mode) when the window can't be made
/// (`fn_00b45c00` with the output left). Otherwise the dictionary's last window's worth
/// (or all of it, at the window's end) is copied into the window, the fill set, and the
/// have-dictionary word (`+0xc`) set to 1; answers 0.
pub fn fn_00b45e30(e: &mut Engine, stream: u32, dictionary: u32, length: u32) -> i32 {
    if stream == 0 || e.mem.u32(stream + STREAM_STATE) == 0 {
        return -2;
    }
    let state = e.mem.u32(stream + STREAM_STATE);
    if e.mem.u32(state + STATE_MODE) != MODE_DICTIONARY {
        return -2;
    }
    let start = e.call(DATA_CHECKSUM, &args![0u32, 0u32, 0u32]).u32();
    let checksum = e
        .call(DATA_CHECKSUM, &args![start, dictionary, length])
        .u32();
    if checksum != e.mem.u32(state + STATE_CHECKSUM) {
        return -3;
    }
    let left = e.mem.u32(stream + STREAM_OUTPUT_LEFT);
    if fn_00b45c00(e, stream, left) != 0 {
        e.mem.set_u32(state + STATE_MODE, MODE_NO_MEMORY);
        return -4;
    }
    let size = e.mem.u32(state + STATE_WINDOW_SIZE);
    if length > size {
        let window = e.mem.u32(state + STATE_WINDOW);
        let from = dictionary.wrapping_add(length).wrapping_sub(size);
        e.call(MEMCPY, &args![window, from, size]);
        let size = e.mem.u32(state + STATE_WINDOW_SIZE);
        e.mem.set_u32(state + STATE_WINDOW_FILL, size);
    } else {
        let window = e.mem.u32(state + STATE_WINDOW);
        let to = window.wrapping_add(size).wrapping_sub(length);
        e.call(MEMCPY, &args![to, dictionary, length]);
        e.mem.set_u32(state + STATE_WINDOW_FILL, length);
    }
    e.mem.set_u32(state + STATE_HAVE_DICTIONARY, 1);
    0
}

// Translated from 00b460e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map; zlib's `syncsearch` by its body (`cdecl`, three words: the
/// address of the found count, a buffer and its length). Scans the buffer while the count
/// is below 4: a byte equal to 0 (count below 2) or 0xff (otherwise) raises the count,
/// another zero byte makes it `4 - count`, any other byte clears it. Stores the count and
/// answers the bytes scanned.
pub fn fn_00b460e0(e: &mut Engine, found: u32, buffer: u32, length: u32) -> u32 {
    let mut count = e.mem.u32(found);
    let mut index = 0u32;
    while index < length && count < 4 {
        let byte = u32::from(e.mem.u8(buffer.wrapping_add(index)));
        let wanted = if count < 2 { 0 } else { 0xff };
        if byte == wanted {
            count = count.wrapping_add(1);
        } else if byte != 0 {
            count = 0;
        } else {
            count = 4u32.wrapping_sub(count);
        }
        index = index.wrapping_add(1);
    }
    e.mem.set_u32(found, count);
    index
}

// Translated from 00b45f50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map; zlib's `inflateSync` by its body (`cdecl`, one word: the
/// stream record). Answers -2 without a stream or state, -5 with no input and fewer than
/// 8 bits held. Entering the sync mode (0x1d) it aligns the bit buffer to a byte, moves
/// the whole bytes held into a 4-byte local and searches them (`fn_00b460e0`, count
/// cleared first); then searches the input, taking what it scanned. Without the full
/// `00 00 ff ff` marker answers -3; with it runs the reset `00b43db0` keeping the total
/// in and out, enters the block-start mode and answers 0.
pub fn fn_00b45f50(e: &mut Engine, stream: u32) -> i32 {
    if stream == 0 || e.mem.u32(stream + STREAM_STATE) == 0 {
        return -2;
    }
    let state = e.mem.u32(stream + STREAM_STATE);
    if e.mem.u32(stream + STREAM_INPUT_LEFT) == 0 && e.mem.u32(state + STATE_BITS) < 8 {
        return -5;
    }
    if e.mem.u32(state + STATE_MODE) != MODE_SYNC {
        e.mem.set_u32(state + STATE_MODE, MODE_SYNC);
        let bits = e.mem.u32(state + STATE_BITS);
        let hold = e.mem.u32(state + STATE_HOLD).wrapping_shl(bits & 7);
        e.mem.set_u32(state + STATE_HOLD, hold);
        let bits = e.mem.u32(state + STATE_BITS);
        e.mem
            .set_u32(state + STATE_BITS, bits.wrapping_sub(bits & 7));
        e.with_stack(4, |e, buffer| {
            let buffer = buffer.addr();
            let mut count = 0u32;
            while e.mem.u32(state + STATE_BITS) >= 8 {
                let byte = e.mem.u8(state + STATE_HOLD);
                e.mem.set_u8(buffer.wrapping_add(count), byte);
                count = count.wrapping_add(1);
                let hold = e.mem.u32(state + STATE_HOLD) >> 8;
                e.mem.set_u32(state + STATE_HOLD, hold);
                let bits = e.mem.u32(state + STATE_BITS).wrapping_sub(8);
                e.mem.set_u32(state + STATE_BITS, bits);
            }
            e.mem.set_u32(state + STATE_HAVE, 0);
            fn_00b460e0(e, state + STATE_HAVE, buffer, count);
        });
    }
    let have = e.mem.u32(stream + STREAM_INPUT_LEFT);
    let input = e.mem.u32(stream + STREAM_INPUT);
    let scanned = fn_00b460e0(e, state + STATE_HAVE, input, have);
    let left = e.mem.u32(stream + STREAM_INPUT_LEFT).wrapping_sub(scanned);
    e.mem.set_u32(stream + STREAM_INPUT_LEFT, left);
    let next = e.mem.u32(stream + STREAM_INPUT).wrapping_add(scanned);
    e.mem.set_u32(stream + STREAM_INPUT, next);
    let total = e.mem.u32(stream + STREAM_TOTAL_IN).wrapping_add(scanned);
    e.mem.set_u32(stream + STREAM_TOTAL_IN, total);
    if e.mem.u32(state + STATE_HAVE) != 4 {
        return -3;
    }
    let total_in = e.mem.u32(stream + STREAM_TOTAL_IN);
    let total_out = e.mem.u32(stream + STREAM_TOTAL_OUT);
    e.call(INFLATE_RESET, &args![stream]);
    e.mem.set_u32(stream + STREAM_TOTAL_IN, total_in);
    e.mem.set_u32(stream + STREAM_TOTAL_OUT, total_out);
    e.mem.set_u32(state + STATE_MODE, MODE_BLOCK_START);
    0
}

// Translated from 00b46170 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map; zlib's `inflateSyncPoint` by its body (`cdecl`, one word:
/// the stream record): -2 without a stream or state; 1 when the state is in the
/// stored-lengths mode (0xd) with no bits held, else 0.
pub fn fn_00b46170(e: &mut Engine, stream: u32) -> i32 {
    if stream == 0 || e.mem.u32(stream + STREAM_STATE) == 0 {
        return -2;
    }
    let state = e.mem.u32(stream + STREAM_STATE);
    i32::from(
        e.mem.u32(state + STATE_MODE) == MODE_STORED_LENGTHS && e.mem.u32(state + STATE_BITS) == 0,
    )
}

/// `(pointer - base) >> 2` (arithmetic) times 4 plus `new_base`: the exe's rebasing of a
/// pointer into the state's code area.
fn rebase(pointer: u32, base: u32, new_base: u32) -> u32 {
    let words = (pointer.wrapping_sub(base) as i32) >> 2;
    new_base.wrapping_add((words as u32).wrapping_mul(4))
}

// Translated from 00b461c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map; zlib's `inflateCopy` by its body (`cdecl`, two words: the
/// destination and source stream records). Answers -2 when either is null or the source
/// has no state, allocator or free function. Allocates a state (0x1ba8 bytes, count 1,
/// through the source's allocator with its opaque word; none answers -4) and, when the
/// source has a window, a window (`1 << bits` bytes; none frees the state and answers -4).
/// Copies the 14 words of the stream record and the state (`memcpy`), rebases the three
/// code pointers (+0x44, +0x48, +0x64) into the new state's code area (+0x528), copies the
/// window, stores it and the state in the destination; answers 0.
pub fn fn_00b461c0(e: &mut Engine, destination: u32, source: u32) -> i32 {
    if destination == 0
        || source == 0
        || e.mem.u32(source + STREAM_STATE) == 0
        || e.mem.u32(source + STREAM_ALLOCATE) == 0
        || e.mem.u32(source + STREAM_FREE) == 0
    {
        return -2;
    }
    let state = e.mem.u32(source + STREAM_STATE);
    let allocate = e.mem.u32(source + STREAM_ALLOCATE);
    let opaque = e.mem.u32(source + STREAM_OPAQUE);
    let copy = e
        .call(allocate, &args![opaque, 1u32, INFLATE_STATE_SIZE])
        .u32();
    if copy == 0 {
        return -4;
    }
    let mut window = 0u32;
    if e.mem.u32(state + STATE_WINDOW) != 0 {
        let size = 1u32.wrapping_shl(e.mem.u32(state + STATE_WINDOW_BITS));
        let allocate = e.mem.u32(source + STREAM_ALLOCATE);
        let opaque = e.mem.u32(source + STREAM_OPAQUE);
        window = e.call(allocate, &args![opaque, size, 1u32]).u32();
        if window == 0 {
            let free = e.mem.u32(source + STREAM_FREE);
            let opaque = e.mem.u32(source + STREAM_OPAQUE);
            e.call(free, &args![opaque, copy]);
            return -4;
        }
    }
    for word in 0..14 {
        let value = e.mem.u32(source + 4 * word);
        e.mem.set_u32(destination + 4 * word, value);
    }
    e.call(MEMCPY, &args![copy, state, INFLATE_STATE_SIZE]);
    for offset in [STATE_LENGTH_CODES, STATE_DISTANCE_CODES, STATE_NEXT_CODE] {
        let pointer = e.mem.u32(state + offset);
        let rebased = rebase(pointer, state + STATE_CODES, copy + STATE_CODES);
        e.mem.set_u32(copy + offset, rebased);
    }
    if window != 0 {
        let size = 1u32.wrapping_shl(e.mem.u32(state + STATE_WINDOW_BITS));
        let from = e.mem.u32(state + STATE_WINDOW);
        e.call(MEMCPY, &args![window, from, size]);
    }
    e.mem.set_u32(copy + STATE_WINDOW, window);
    e.mem.set_u32(destination + STREAM_STATE, copy);
    0
}

// The compressor half of this unit: zlib's `deflate.c` shapes by their bodies (the
// engine map names none of them). The state is 0x16b8 bytes; its words used here:
/// The compressor state's size and its words.
const DEFLATE_STATE_SIZE: u32 = 0x16b8;
const DS_STREAM: u32 = 0x00;
const DS_STATUS: u32 = 0x04;
const DS_PENDING_BUFFER: u32 = 0x08;
const DS_PENDING_BUFFER_SIZE: u32 = 0x0c;
const DS_PENDING_OUT: u32 = 0x10;
const DS_PENDING: u32 = 0x14;
/// Header kind: 0 none, 1 the `78 xx` kind, 2 the `1f 8b` kind; negated once the trailer
/// is written.
const DS_WRAP: u32 = 0x18;
/// The method byte (8).
const DS_METHOD: u32 = 0x1d;
const DS_LAST_FLUSH: u32 = 0x20;
const DS_WINDOW_SIZE: u32 = 0x24;
const DS_WINDOW_BITS: u32 = 0x28;
const DS_WINDOW_MASK: u32 = 0x2c;
const DS_WINDOW: u32 = 0x30;
const DS_PREVIOUS: u32 = 0x38;
const DS_HEAD: u32 = 0x3c;
const DS_INSERT_HASH: u32 = 0x40;
const DS_HASH_SIZE: u32 = 0x44;
const DS_HASH_BITS: u32 = 0x48;
const DS_HASH_MASK: u32 = 0x4c;
const DS_HASH_SHIFT: u32 = 0x50;
const DS_BLOCK_START: u32 = 0x54;
const DS_STRING_START: u32 = 0x64;
const DS_LOOKAHEAD: u32 = 0x6c;
const DS_MAX_CHAIN: u32 = 0x74;
const DS_MAX_LAZY: u32 = 0x78;
const DS_LEVEL: u32 = 0x7c;
const DS_STRATEGY: u32 = 0x80;
const DS_GOOD_MATCH: u32 = 0x84;
const DS_NICE_MATCH: u32 = 0x88;
const DS_LITERAL_BUFFER: u32 = 0x1690;
const DS_LITERAL_BUFFER_SIZE: u32 = 0x1694;
const DS_DISTANCE_BUFFER: u32 = 0x169c;
const DS_BIT_BUFFER: u32 = 0x16b0;
const DS_BIT_COUNT: u32 = 0x16b4;
/// The status words: initial, busy, finished.
const STATUS_INIT: u32 = 0x2a;
const STATUS_BUSY: u32 = 0x71;
const STATUS_FINISH: u32 = 0x29a;
/// The stream's adler/crc word and the data-type word set by the reset.
const STREAM_ADLER: u32 = 0x30;
const STREAM_DATA_TYPE: u32 = 0x2c;
/// The version string's first character `fn_00b46390` checks against (`.rdata`).
const VERSION_STRING: u32 = 0x010a_63f0;
/// The message table's entries for the stream error (-2), the memory error (-4) and the
/// buffer error (-5) (`.data` words holding message addresses).
const MESSAGE_STREAM_ERROR: u32 = 0x010a_6408;
const MESSAGE_MEMORY_ERROR: u32 = 0x010a_6410;
const MESSAGE_BUFFER_ERROR: u32 = 0x010a_6414;
/// The level table (`.rdata`, 12-byte rows: good length, lazy, nice length, chain
/// (`u16` each), then the compress function).
const LEVEL_TABLE: u32 = 0x010a_6378;
/// The checksum `00b4a070` (`run_00b49e20.rs`; the `1f 8b` kind's), the trees' set-up
/// `00b4aad0`, the stored block `00b4ac50` (four words), the alignment `00b4ad90`, the
/// bound helper `00b4dea0` (one word) and `memset` (`00ec61c0`), all `cdecl`.
const HEADER_CHECKSUM_00B4A070: u32 = 0x00b4_a070;
const TREES_INIT: u32 = 0x00b4_aad0;
const TREES_STORED_BLOCK: u32 = 0x00b4_ac50;
const TREES_ALIGN: u32 = 0x00b4_ad90;
const BOUND_00B4DEA0: u32 = 0x00b4_dea0;
const MEMSET: u32 = 0x00ec_61c0;

/// Appends a byte to the pending buffer.
fn put_byte(e: &mut Engine, state: u32, byte: u32) {
    let buffer = e.mem.u32(state + DS_PENDING_BUFFER);
    let pending = e.mem.u32(state + DS_PENDING);
    e.mem.set_u8(buffer.wrapping_add(pending), byte as u8);
    e.mem.set_u32(state + DS_PENDING, pending.wrapping_add(1));
}

/// A row of the level table.
fn level_row(level: u32) -> u32 {
    LEVEL_TABLE.wrapping_add(level.wrapping_mul(12))
}

// Translated from 00b46360 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map; zlib's `deflateInit_` by its body (`cdecl`, four words: the
/// stream record, the level, the version string and the record's size): `fn_00b46390`
/// with method 8, 15 window bits, memory level 8 and strategy 0; its answer.
pub fn fn_00b46360(e: &mut Engine, stream: u32, level: i32, version: u32, size: i32) -> i32 {
    fn_00b46390(e, stream, level, 8, 15, 8, 0, version, size)
}

// Translated from 00b46390 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map; zlib's `deflateInit2_` by its body (`cdecl`, eight words:
/// the stream record, level, method, window bits, memory level, strategy, version string,
/// record size). -6 unless the version's first character matches `010a63f0` and the size
/// is 0x38; -2 without a stream. Clears the message, installs the default allocator
/// (`fn_00b49020`, opaque 0) and free (`fn_00b49050`) where missing; level -1 means 6;
/// negative window bits mean no header, above 15 the `1f 8b` kind. -2 for a memory level
/// outside 1..=9, a method other than 8, window bits outside 8..=15, a level outside
/// 0..=9 or a strategy outside 0..=3; 8 window bits become 9. Allocates the state (-4
/// when it can't), the window, previous and head chains (two bytes each) and the pending
/// buffer (four bytes per literal); when any of those fails the status is the finished
/// one, the message the memory one, the state taken down (`fn_00b47370`) and -4
/// answered. Otherwise lays the literal and distance buffers over the pending one, keeps
/// the level, strategy and method, and answers the reset (`fn_00b46880`).
#[allow(clippy::too_many_arguments)]
pub fn fn_00b46390(
    e: &mut Engine,
    stream: u32,
    level: i32,
    method: i32,
    window_bits: i32,
    memory_level: i32,
    strategy: i32,
    version: u32,
    size: i32,
) -> i32 {
    let mut wrap = 1u32;
    if version == 0 || e.mem.u8(version) as i8 != e.mem.u8(VERSION_STRING) as i8 || size != 0x38 {
        return -6;
    }
    if stream == 0 {
        return -2;
    }
    e.mem.set_u32(stream + STREAM_MESSAGE, 0);
    if e.mem.u32(stream + STREAM_ALLOCATE) == 0 {
        e.mem.set_u32(stream + STREAM_ALLOCATE, 0x00b4_9020);
        e.mem.set_u32(stream + STREAM_OPAQUE, 0);
    }
    if e.mem.u32(stream + STREAM_FREE) == 0 {
        e.mem.set_u32(stream + STREAM_FREE, 0x00b4_9050);
    }
    let mut level = level;
    if level == -1 {
        level = 6;
    }
    let mut window_bits = window_bits;
    if window_bits < 0 {
        wrap = 0;
        window_bits = window_bits.wrapping_neg();
    } else if window_bits > 15 {
        wrap = 2;
        window_bits = window_bits.wrapping_sub(16);
    }
    if !(1..=9).contains(&memory_level)
        || method != 8
        || !(8..=15).contains(&window_bits)
        || !(0..=9).contains(&level)
        || !(0..=3).contains(&strategy)
    {
        return -2;
    }
    if window_bits == 8 {
        window_bits = 9;
    }
    let allocate = |e: &mut Engine, count: u32, item: u32| {
        let function = e.mem.u32(stream + STREAM_ALLOCATE);
        let opaque = e.mem.u32(stream + STREAM_OPAQUE);
        e.call(function, &args![opaque, count, item]).u32()
    };
    let state = allocate(e, 1, DEFLATE_STATE_SIZE);
    if state == 0 {
        return -4;
    }
    e.mem.set_u32(stream + STREAM_STATE, state);
    e.mem.set_u32(state + DS_STREAM, stream);
    e.mem.set_u32(state + DS_WRAP, wrap);
    e.mem.set_u32(state + DS_WINDOW_BITS, window_bits as u32);
    let window_size = 1u32.wrapping_shl(e.mem.u32(state + DS_WINDOW_BITS));
    e.mem.set_u32(state + DS_WINDOW_SIZE, window_size);
    let mask = e.mem.u32(state + DS_WINDOW_SIZE).wrapping_sub(1);
    e.mem.set_u32(state + DS_WINDOW_MASK, mask);
    e.mem
        .set_u32(state + DS_HASH_BITS, (memory_level as u32).wrapping_add(7));
    let hash_size = 1u32.wrapping_shl(e.mem.u32(state + DS_HASH_BITS));
    e.mem.set_u32(state + DS_HASH_SIZE, hash_size);
    let mask = e.mem.u32(state + DS_HASH_SIZE).wrapping_sub(1);
    e.mem.set_u32(state + DS_HASH_MASK, mask);
    let shift = e.mem.u32(state + DS_HASH_BITS).wrapping_add(2) / 3;
    e.mem.set_u32(state + DS_HASH_SHIFT, shift);
    let count = e.mem.u32(state + DS_WINDOW_SIZE);
    let window = allocate(e, count, 2);
    e.mem.set_u32(state + DS_WINDOW, window);
    let count = e.mem.u32(state + DS_WINDOW_SIZE);
    let previous = allocate(e, count, 2);
    e.mem.set_u32(state + DS_PREVIOUS, previous);
    let count = e.mem.u32(state + DS_HASH_SIZE);
    let head = allocate(e, count, 2);
    e.mem.set_u32(state + DS_HEAD, head);
    e.mem.set_u32(
        state + DS_LITERAL_BUFFER_SIZE,
        1u32.wrapping_shl((memory_level as u32).wrapping_add(6)),
    );
    let count = e.mem.u32(state + DS_LITERAL_BUFFER_SIZE);
    let overlay = allocate(e, count, 4);
    e.mem.set_u32(state + DS_PENDING_BUFFER, overlay);
    let literals = e.mem.u32(state + DS_LITERAL_BUFFER_SIZE);
    e.mem
        .set_u32(state + DS_PENDING_BUFFER_SIZE, literals.wrapping_shl(2));
    if e.mem.u32(state + DS_WINDOW) == 0
        || e.mem.u32(state + DS_PREVIOUS) == 0
        || e.mem.u32(state + DS_HEAD) == 0
        || e.mem.u32(state + DS_PENDING_BUFFER) == 0
    {
        e.mem.set_u32(state + DS_STATUS, STATUS_FINISH);
        let message = e.mem.u32(MESSAGE_MEMORY_ERROR);
        e.mem.set_u32(stream + STREAM_MESSAGE, message);
        fn_00b47370(e, stream);
        return -4;
    }
    let literals = e.mem.u32(state + DS_LITERAL_BUFFER_SIZE);
    e.mem.set_u32(
        state + DS_DISTANCE_BUFFER,
        overlay.wrapping_add((literals >> 1).wrapping_mul(2)),
    );
    let literals = e.mem.u32(state + DS_LITERAL_BUFFER_SIZE);
    let pending = e.mem.u32(state + DS_PENDING_BUFFER);
    e.mem.set_u32(
        state + DS_LITERAL_BUFFER,
        literals.wrapping_mul(3).wrapping_add(pending),
    );
    e.mem.set_u32(state + DS_LEVEL, level as u32);
    e.mem.set_u32(state + DS_STRATEGY, strategy as u32);
    e.mem.set_u8(state + DS_METHOD, method as u8);
    fn_00b46880(e, stream)
}

// Translated from 00b466b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map; zlib's `deflateSetDictionary` by its body (`cdecl`, three
/// words: the stream record, a dictionary and its length). -2 without a stream, state or
/// dictionary, for the `1f 8b` kind, or for the `78 xx` kind once started. With a header
/// the stream's checksum takes the dictionary (`00b49e20`). Fewer than 3 bytes answer 0;
/// at most the window size minus 0x106 of its last bytes are copied to the window
/// (`memcpy`), the string and block starts set to their count, the insert hash primed with
/// the first two bytes and each 3-byte string hashed into the head and previous chains.
/// Answers 0.
pub fn fn_00b466b0(e: &mut Engine, stream: u32, dictionary: u32, length: u32) -> i32 {
    let mut count = length;
    let mut dictionary = dictionary;
    if stream == 0 || e.mem.u32(stream + STREAM_STATE) == 0 || dictionary == 0 {
        return -2;
    }
    let state = e.mem.u32(stream + STREAM_STATE);
    if e.mem.u32(state + DS_WRAP) == 2
        || (e.mem.u32(state + DS_WRAP) == 1 && e.mem.u32(state + DS_STATUS) != STATUS_INIT)
    {
        return -2;
    }
    if e.mem.u32(state + DS_WRAP) != 0 {
        let adler = e.mem.u32(stream + STREAM_ADLER);
        let adler = e
            .call(DATA_CHECKSUM, &args![adler, dictionary, length])
            .u32();
        e.mem.set_u32(stream + STREAM_ADLER, adler);
    }
    if count < 3 {
        return 0;
    }
    let limit = e.mem.u32(state + DS_WINDOW_SIZE).wrapping_sub(0x106);
    if count > limit {
        count = e.mem.u32(state + DS_WINDOW_SIZE).wrapping_sub(0x106);
        dictionary = length.wrapping_sub(count).wrapping_add(dictionary);
    }
    let window = e.mem.u32(state + DS_WINDOW);
    e.call(MEMCPY, &args![window, dictionary, count]);
    e.mem.set_u32(state + DS_STRING_START, count);
    e.mem.set_u32(state + DS_BLOCK_START, count);
    let window = e.mem.u32(state + DS_WINDOW);
    e.mem
        .set_u32(state + DS_INSERT_HASH, u32::from(e.mem.u8(window)));
    let hash = e
        .mem
        .u32(state + DS_INSERT_HASH)
        .wrapping_shl(e.mem.u32(state + DS_HASH_SHIFT));
    let window = e.mem.u32(state + DS_WINDOW);
    let hash =
        (hash ^ u32::from(e.mem.u8(window.wrapping_add(1)))) & e.mem.u32(state + DS_HASH_MASK);
    e.mem.set_u32(state + DS_INSERT_HASH, hash);
    let mut index = 0u32;
    while index <= count.wrapping_sub(3) {
        let hash = e
            .mem
            .u32(state + DS_INSERT_HASH)
            .wrapping_shl(e.mem.u32(state + DS_HASH_SHIFT));
        let window = e.mem.u32(state + DS_WINDOW);
        let byte = u32::from(e.mem.u8(window.wrapping_add(index).wrapping_add(2)));
        let hash = (hash ^ byte) & e.mem.u32(state + DS_HASH_MASK);
        e.mem.set_u32(state + DS_INSERT_HASH, hash);
        let hash = e.mem.u32(state + DS_INSERT_HASH);
        let head = e.mem.u32(state + DS_HEAD);
        let slot = index & e.mem.u32(state + DS_WINDOW_MASK);
        let previous = e.mem.u32(state + DS_PREVIOUS);
        let value = e.mem.u16(head.wrapping_add(hash.wrapping_mul(2)));
        e.mem
            .set_u16(previous.wrapping_add(slot.wrapping_mul(2)), value);
        // The exe reads the entry back into a local it then clears (no other use).
        let slot = index & e.mem.u32(state + DS_WINDOW_MASK);
        let previous = e.mem.u32(state + DS_PREVIOUS);
        let _ = e.mem.u16(previous.wrapping_add(slot.wrapping_mul(2)));
        let hash = e.mem.u32(state + DS_INSERT_HASH);
        let head = e.mem.u32(state + DS_HEAD);
        e.mem
            .set_u16(head.wrapping_add(hash.wrapping_mul(2)), index as u16);
        index = index.wrapping_add(1);
    }
    0
}

// Translated from 00b46880 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map; zlib's `deflateReset` by its body (`cdecl`, one word: the
/// stream record). -2 without a stream, state, allocator or free function. Clears the
/// totals and message, sets the data type to 2, empties the pending buffer, makes the
/// header kind positive, the status initial (with a header) or busy, the stream's checksum
/// the start value of the kind's checksum (`00b4a070` for the `1f 8b` kind, `00b49e20`
/// otherwise), clears the last flush, and runs the trees' set-up (`00b4aad0`) and
/// `fn_00b476f0`. Answers 0.
pub fn fn_00b46880(e: &mut Engine, stream: u32) -> i32 {
    if stream == 0
        || e.mem.u32(stream + STREAM_STATE) == 0
        || e.mem.u32(stream + STREAM_ALLOCATE) == 0
        || e.mem.u32(stream + STREAM_FREE) == 0
    {
        return -2;
    }
    e.mem.set_u32(stream + STREAM_TOTAL_OUT, 0);
    e.mem.set_u32(stream + STREAM_TOTAL_IN, 0);
    e.mem.set_u32(stream + STREAM_MESSAGE, 0);
    e.mem.set_u32(stream + STREAM_DATA_TYPE, 2);
    let state = e.mem.u32(stream + STREAM_STATE);
    e.mem.set_u32(state + DS_PENDING, 0);
    let buffer = e.mem.u32(state + DS_PENDING_BUFFER);
    e.mem.set_u32(state + DS_PENDING_OUT, buffer);
    if (e.mem.u32(state + DS_WRAP) as i32) < 0 {
        let wrap = e.mem.u32(state + DS_WRAP).wrapping_neg();
        e.mem.set_u32(state + DS_WRAP, wrap);
    }
    let status = if e.mem.u32(state + DS_WRAP) != 0 {
        STATUS_INIT
    } else {
        STATUS_BUSY
    };
    e.mem.set_u32(state + DS_STATUS, status);
    let start = if e.mem.u32(state + DS_WRAP) == 2 {
        e.call(HEADER_CHECKSUM_00B4A070, &args![0u32, 0u32, 0u32])
            .u32()
    } else {
        e.call(DATA_CHECKSUM, &args![0u32, 0u32, 0u32]).u32()
    };
    e.mem.set_u32(stream + STREAM_ADLER, start);
    e.mem.set_u32(state + DS_LAST_FLUSH, 0);
    e.call(TREES_INIT, &args![state]);
    fn_00b476f0(e, state);
    0
}

// Translated from 00b46990 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map; zlib's `deflatePrime` by its body (`cdecl`, three words: the
/// stream record, a bit count and a value): -2 without a stream or state; else the bit
/// count (+0x16b4) and the value's low bits (`u16` at +0x16b0) become the bit buffer;
/// answers 0.
pub fn fn_00b46990(e: &mut Engine, stream: u32, bits: u32, value: u32) -> i32 {
    if stream == 0 || e.mem.u32(stream + STREAM_STATE) == 0 {
        return -2;
    }
    let state = e.mem.u32(stream + STREAM_STATE);
    e.mem.set_u32(state + DS_BIT_COUNT, bits);
    let low = 1u32.wrapping_shl(bits).wrapping_sub(1) & value;
    let state = e.mem.u32(stream + STREAM_STATE);
    e.mem.set_u16(state + DS_BIT_BUFFER, low as u16);
    0
}

// Translated from 00b469e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map; zlib's `deflateParams` by its body (`cdecl`, three words: the
/// stream record, level and strategy). -2 without a stream or state, or for a level
/// outside 0..=9 (after -1 becomes 6) or a strategy outside 0..=3. When the level table's
/// compress function changes (the old level's row against the new one's) and input was
/// taken, the stream is flushed (`fn_00b46b70` with 1) and that answer kept. A new level
/// takes its row's lazy, good, nice and chain values; the strategy is stored. Answers the
/// kept value (0 without a flush).
pub fn fn_00b469e0(e: &mut Engine, stream: u32, level: i32, strategy: i32) -> i32 {
    let mut answer = 0;
    if stream == 0 || e.mem.u32(stream + STREAM_STATE) == 0 {
        return -2;
    }
    let state = e.mem.u32(stream + STREAM_STATE);
    let mut level = level;
    if level == -1 {
        level = 6;
    }
    if !(0..=9).contains(&level) || !(0..=3).contains(&strategy) {
        return -2;
    }
    let old = e.mem.u32(level_row(e.mem.u32(state + DS_LEVEL)) + 8);
    if old != e.mem.u32(level_row(level as u32) + 8) && e.mem.u32(stream + STREAM_TOTAL_IN) != 0 {
        answer = fn_00b46b70(e, stream, 1);
    }
    if e.mem.u32(state + DS_LEVEL) != level as u32 {
        e.mem.set_u32(state + DS_LEVEL, level as u32);
        let row = level_row(level as u32);
        e.mem
            .set_u32(state + DS_MAX_LAZY, u32::from(e.mem.u16(row + 2)));
        e.mem
            .set_u32(state + DS_GOOD_MATCH, u32::from(e.mem.u16(row)));
        e.mem
            .set_u32(state + DS_NICE_MATCH, u32::from(e.mem.u16(row + 4)));
        e.mem
            .set_u32(state + DS_MAX_CHAIN, u32::from(e.mem.u16(row + 6)));
    }
    e.mem.set_u32(state + DS_STRATEGY, strategy as u32);
    answer
}

// Translated from 00b46b00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map; zlib's `deflateBound` by its body (`cdecl`, two words: the
/// stream record and a source length): `length + (length + 7) / 8 + (length + 63) / 64 +
/// 11`, unless a state with 15 window and 15 hash bits is there: then `00b4dea0(length)`.
pub fn fn_00b46b00(e: &mut Engine, stream: u32, length: u32) -> u32 {
    let bound = (length.wrapping_add(7) >> 3)
        .wrapping_add(length)
        .wrapping_add(length.wrapping_add(0x3f) >> 6)
        .wrapping_add(0xb);
    if stream == 0 || e.mem.u32(stream + STREAM_STATE) == 0 {
        return bound;
    }
    let state = e.mem.u32(stream + STREAM_STATE);
    if e.mem.u32(state + DS_WINDOW_BITS) != 0xf || e.mem.u32(state + DS_HASH_BITS) != 0xf {
        return bound;
    }
    e.call(BOUND_00B4DEA0, &args![length]).u32()
}

// Translated from 00b47250 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map; zlib's `putShortMSB` by its body (`cdecl`, two words: the
/// compressor state and a value): its bits 8..15, then its low byte, to the pending buffer.
pub fn fn_00b47250(e: &mut Engine, state: u32, value: u32) {
    put_byte(e, state, value >> 8);
    put_byte(e, state, value & 0xff);
}

// Translated from 00b472a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map; zlib's `flush_pending` by its body (`cdecl`, one word: the
/// stream record): copies as much of the pending bytes as the output has room for
/// (`memcpy`), moving the output, the pending position, the total out and both counts; an
/// emptied pending buffer starts over.
pub fn fn_00b472a0(e: &mut Engine, stream: u32) {
    let state = e.mem.u32(stream + STREAM_STATE);
    let mut length = e.mem.u32(state + DS_PENDING);
    if length > e.mem.u32(stream + STREAM_OUTPUT_LEFT) {
        length = e.mem.u32(stream + STREAM_OUTPUT_LEFT);
    }
    if length == 0 {
        return;
    }
    let output = e.mem.u32(stream + STREAM_OUTPUT);
    let from = e.mem.u32(state + DS_PENDING_OUT);
    e.call(MEMCPY, &args![output, from, length]);
    let output = e.mem.u32(stream + STREAM_OUTPUT).wrapping_add(length);
    e.mem.set_u32(stream + STREAM_OUTPUT, output);
    let state = e.mem.u32(stream + STREAM_STATE);
    let out = e.mem.u32(state + DS_PENDING_OUT).wrapping_add(length);
    e.mem.set_u32(state + DS_PENDING_OUT, out);
    let total = e.mem.u32(stream + STREAM_TOTAL_OUT).wrapping_add(length);
    e.mem.set_u32(stream + STREAM_TOTAL_OUT, total);
    let left = e.mem.u32(stream + STREAM_OUTPUT_LEFT).wrapping_sub(length);
    e.mem.set_u32(stream + STREAM_OUTPUT_LEFT, left);
    let state = e.mem.u32(stream + STREAM_STATE);
    let pending = e.mem.u32(state + DS_PENDING).wrapping_sub(length);
    e.mem.set_u32(state + DS_PENDING, pending);
    let state = e.mem.u32(stream + STREAM_STATE);
    if e.mem.u32(state + DS_PENDING) == 0 {
        let buffer = e.mem.u32(state + DS_PENDING_BUFFER);
        e.mem.set_u32(state + DS_PENDING_OUT, buffer);
    }
}

// Translated from 00b47370 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map; zlib's `deflateEnd` by its body (`cdecl`, one word: the
/// stream record). -2 without a stream or state, or with a status other than initial,
/// busy or finished. Frees the pending buffer, head, previous chain and window (each when
/// there is one) and the state through the stream's free function and opaque word, clears
/// the state word, and answers -3 when it was busy, else 0.
pub fn fn_00b47370(e: &mut Engine, stream: u32) -> i32 {
    if stream == 0 || e.mem.u32(stream + STREAM_STATE) == 0 {
        return -2;
    }
    let status = e.mem.u32(e.mem.u32(stream + STREAM_STATE) + DS_STATUS);
    if status != STATUS_INIT && status != STATUS_BUSY && status != STATUS_FINISH {
        return -2;
    }
    for offset in [DS_PENDING_BUFFER, DS_HEAD, DS_PREVIOUS, DS_WINDOW] {
        let state = e.mem.u32(stream + STREAM_STATE);
        let block = e.mem.u32(state + offset);
        if block != 0 {
            let free = e.mem.u32(stream + STREAM_FREE);
            let opaque = e.mem.u32(stream + STREAM_OPAQUE);
            e.call(free, &args![opaque, block]);
        }
    }
    let state = e.mem.u32(stream + STREAM_STATE);
    let free = e.mem.u32(stream + STREAM_FREE);
    let opaque = e.mem.u32(stream + STREAM_OPAQUE);
    e.call(free, &args![opaque, state]);
    e.mem.set_u32(stream + STREAM_STATE, 0);
    if status == STATUS_BUSY {
        -3
    } else {
        0
    }
}

// Translated from 00b46b70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map; zlib's `deflate` by its body (`cdecl`, two words: the stream
/// record and the flush kind 0..=4). -2 without a stream or state or for another flush
/// kind; with no output, or input left without an input pointer, or a finished stream
/// flushed other than with 4, the stream-error message and -2; with no room the
/// buffer-error message and -5. An initial status writes the header: for the `1f 8b`
/// kind `1f 8b 08 00 00 00 00 00`, the level byte (2 at level 9, 4 for strategy 2+ or
/// level below 2, else 0) and `ff`, with the checksum restarted (`00b4a070`); otherwise
/// the `78 xx` header with the level flags and the preset-dictionary bit (and the
/// dictionary's checksum after it), the checksum restarted (`00b49e20`); then busy.
/// Pending bytes are flushed (`fn_00b472a0`; no room left keeps the last flush at -1 and
/// answers 0); without new input or a stronger flush than last time (4 aside) the
/// buffer error. A finished stream with input left is a buffer error too. With input, a
/// lookahead or a flush on an unfinished stream the level's compress function runs:
/// finishing kinds (2, 3) finish the stream; need-more and finish-started (0, 2) answer
/// 0 (last flush -1 without room); a block done (1) aligns (`00b4ad90`, flush 1) or adds an
/// empty stored block (`00b4ac50`; flush 3 also clears the head chain with `memset`), then
/// flushes. Other flushes than 4 answer 0; no header answers 1; else the trailer (the
/// `1f 8b` kind's checksum and total in, little-endian; otherwise the checksum's two
/// halves), a flush, the header kind negated, and 1 when nothing is left pending.
pub fn fn_00b46b70(e: &mut Engine, stream: u32, flush: i32) -> i32 {
    if stream == 0 || e.mem.u32(stream + STREAM_STATE) == 0 || !(0..=4).contains(&flush) {
        return -2;
    }
    let state = e.mem.u32(stream + STREAM_STATE);
    if e.mem.u32(stream + STREAM_OUTPUT) == 0
        || (e.mem.u32(stream + STREAM_INPUT) == 0 && e.mem.u32(stream + STREAM_INPUT_LEFT) != 0)
        || (e.mem.u32(state + DS_STATUS) == STATUS_FINISH && flush != 4)
    {
        let message = e.mem.u32(MESSAGE_STREAM_ERROR);
        e.mem.set_u32(stream + STREAM_MESSAGE, message);
        return -2;
    }
    if e.mem.u32(stream + STREAM_OUTPUT_LEFT) == 0 {
        let message = e.mem.u32(MESSAGE_BUFFER_ERROR);
        e.mem.set_u32(stream + STREAM_MESSAGE, message);
        return -5;
    }
    e.mem.set_u32(state + DS_STREAM, stream);
    let old_flush = e.mem.u32(state + DS_LAST_FLUSH) as i32;
    e.mem.set_u32(state + DS_LAST_FLUSH, flush as u32);
    if e.mem.u32(state + DS_STATUS) == STATUS_INIT {
        if e.mem.u32(state + DS_WRAP) == 2 {
            for byte in [0x1fu32, 0x8b, 8, 0, 0, 0, 0, 0] {
                put_byte(e, state, byte);
            }
            let level = e.mem.u32(state + DS_LEVEL) as i32;
            let flags = if level == 9 {
                2
            } else if (e.mem.u32(state + DS_STRATEGY) as i32) < 2 && level >= 2 {
                0
            } else {
                4
            };
            put_byte(e, state, flags);
            put_byte(e, state, 0xff);
            e.mem.set_u32(state + DS_STATUS, STATUS_BUSY);
            let start = e
                .call(HEADER_CHECKSUM_00B4A070, &args![0u32, 0u32, 0u32])
                .u32();
            e.mem.set_u32(stream + STREAM_ADLER, start);
        } else {
            let mut header = (e
                .mem
                .u32(state + DS_WINDOW_BITS)
                .wrapping_sub(8)
                .wrapping_shl(4)
                .wrapping_add(8))
            .wrapping_shl(8);
            let level = e.mem.u32(state + DS_LEVEL) as i32;
            let level_flags = if (e.mem.u32(state + DS_STRATEGY) as i32) >= 2 || level < 2 {
                0
            } else if level < 6 {
                1
            } else if level == 6 {
                2
            } else {
                3
            };
            header |= level_flags << 6;
            if e.mem.u32(state + DS_STRING_START) != 0 {
                header |= 0x20;
            }
            header = 0x1fu32.wrapping_sub(header % 0x1f).wrapping_add(header);
            e.mem.set_u32(state + DS_STATUS, STATUS_BUSY);
            fn_00b47250(e, state, header);
            if e.mem.u32(state + DS_STRING_START) != 0 {
                let adler = e.mem.u32(stream + STREAM_ADLER);
                fn_00b47250(e, state, adler >> 16);
                let adler = e.mem.u32(stream + STREAM_ADLER);
                fn_00b47250(e, state, adler & 0xffff);
            }
            let start = e.call(DATA_CHECKSUM, &args![0u32, 0u32, 0u32]).u32();
            e.mem.set_u32(stream + STREAM_ADLER, start);
        }
    }
    if e.mem.u32(state + DS_PENDING) != 0 {
        fn_00b472a0(e, stream);
        if e.mem.u32(stream + STREAM_OUTPUT_LEFT) == 0 {
            e.mem.set_u32(state + DS_LAST_FLUSH, u32::MAX);
            return 0;
        }
    } else if e.mem.u32(stream + STREAM_INPUT_LEFT) == 0 && flush <= old_flush && flush != 4 {
        let message = e.mem.u32(MESSAGE_BUFFER_ERROR);
        e.mem.set_u32(stream + STREAM_MESSAGE, message);
        return -5;
    }
    if e.mem.u32(state + DS_STATUS) == STATUS_FINISH && e.mem.u32(stream + STREAM_INPUT_LEFT) != 0 {
        let message = e.mem.u32(MESSAGE_BUFFER_ERROR);
        e.mem.set_u32(stream + STREAM_MESSAGE, message);
        return -5;
    }
    if e.mem.u32(stream + STREAM_INPUT_LEFT) != 0
        || e.mem.u32(state + DS_LOOKAHEAD) != 0
        || (flush != 0 && e.mem.u32(state + DS_STATUS) != STATUS_FINISH)
    {
        let function = e.mem.u32(level_row(e.mem.u32(state + DS_LEVEL)) + 8);
        let block_state = e.call(function, &args![state, flush as u32]).u32();
        if block_state == 2 || block_state == 3 {
            e.mem.set_u32(state + DS_STATUS, STATUS_FINISH);
        }
        if block_state == 0 || block_state == 2 {
            if e.mem.u32(stream + STREAM_OUTPUT_LEFT) == 0 {
                e.mem.set_u32(state + DS_LAST_FLUSH, u32::MAX);
            }
            return 0;
        }
        if block_state == 1 {
            if flush == 1 {
                e.call(TREES_ALIGN, &args![state]);
            } else {
                e.call(TREES_STORED_BLOCK, &args![state, 0u32, 0u32, 0u32]);
                if flush == 3 {
                    let size = e.mem.u32(state + DS_HASH_SIZE);
                    let head = e.mem.u32(state + DS_HEAD);
                    e.mem
                        .set_u16(head.wrapping_add(size.wrapping_mul(2)).wrapping_sub(2), 0);
                    let size = e.mem.u32(state + DS_HASH_SIZE);
                    let head = e.mem.u32(state + DS_HEAD);
                    e.call(
                        MEMSET,
                        &args![head, 0u32, size.wrapping_mul(2).wrapping_sub(2)],
                    );
                }
            }
            fn_00b472a0(e, stream);
            if e.mem.u32(stream + STREAM_OUTPUT_LEFT) == 0 {
                e.mem.set_u32(state + DS_LAST_FLUSH, u32::MAX);
                return 0;
            }
        }
    }
    if flush != 4 {
        return 0;
    }
    if (e.mem.u32(state + DS_WRAP) as i32) <= 0 {
        return 1;
    }
    if e.mem.u32(state + DS_WRAP) == 2 {
        let adler = e.mem.u32(stream + STREAM_ADLER);
        for shift in [0u32, 8, 16, 24] {
            put_byte(e, state, (adler >> shift) & 0xff);
        }
        let total = e.mem.u32(stream + STREAM_TOTAL_IN);
        for shift in [0u32, 8, 16, 24] {
            put_byte(e, state, (total >> shift) & 0xff);
        }
    } else {
        let adler = e.mem.u32(stream + STREAM_ADLER);
        fn_00b47250(e, state, adler >> 16);
        let adler = e.mem.u32(stream + STREAM_ADLER);
        fn_00b47250(e, state, adler & 0xffff);
    }
    fn_00b472a0(e, stream);
    if (e.mem.u32(state + DS_WRAP) as i32) > 0 {
        let wrap = e.mem.u32(state + DS_WRAP).wrapping_neg();
        e.mem.set_u32(state + DS_WRAP, wrap);
    }
    i32::from(e.mem.u32(state + DS_PENDING) == 0)
}

/// More compressor state words: the window's double size, the previous match and lazy
/// state, and the literal count; the frequency tables (4-byte entries, the count a `u16`
/// at +0): literals and lengths at +0x8c, distances at +0x980.
const DS_WINDOW_DOUBLE: u32 = 0x34;
const DS_MATCH_LENGTH: u32 = 0x58;
const DS_PREVIOUS_MATCH: u32 = 0x5c;
const DS_MATCH_AVAILABLE: u32 = 0x60;
const DS_MATCH_START: u32 = 0x68;
const DS_PREVIOUS_LENGTH: u32 = 0x70;
const DS_LITERAL_FREQUENCIES: u32 = 0x8c;
const DS_LENGTH_FREQUENCIES: u32 = 0x490;
const DS_DISTANCE_FREQUENCIES: u32 = 0x980;
const DS_LAST_LITERAL: u32 = 0x1698;
/// The tree descriptors' table pointers `fn_00b47490` rebases (+0xb10, +0xb1c, +0xb28 point
/// at the frequency tables +0x8c, +0x980 and +0xa74).
const DS_LITERAL_DESCRIPTOR: u32 = 0xb10;
const DS_DISTANCE_DESCRIPTOR: u32 = 0xb1c;
const DS_BIT_LENGTH_DESCRIPTOR: u32 = 0xb28;
const DS_BIT_LENGTH_FREQUENCIES: u32 = 0xa74;
/// The length-code and distance-code byte tables (`.rdata`; the distance codes of
/// distances from 256 on are at +0x100, by `distance >> 7`).
const LENGTH_CODE: u32 = 0x010a_8d98;
const DISTANCE_CODE: u32 = 0x010a_8b98;
/// The block flush `00b4b250` (`run_00b49e20.rs`; `cdecl`, four words: state, block start
/// or null, length, last-block flag).
const TREES_FLUSH_BLOCK: u32 = 0x00b4_b250;
/// The window margin (`0x106`) and the largest match (`0x102`).
const MIN_LOOKAHEAD: u32 = 0x106;
const MAX_MATCH: u32 = 0x102;

/// The exe's inlined string insert: the insert hash takes the byte two past the string
/// start; the previous chain at the start (masked) takes the head of that hash, the head
/// takes the start. Answers the chain entry read back (the hash head).
fn insert_string(e: &mut Engine, state: u32) -> u32 {
    let hash = e
        .mem
        .u32(state + DS_INSERT_HASH)
        .wrapping_shl(e.mem.u32(state + DS_HASH_SHIFT));
    let start = e.mem.u32(state + DS_STRING_START);
    let window = e.mem.u32(state + DS_WINDOW);
    let byte = u32::from(e.mem.u8(window.wrapping_add(start).wrapping_add(2)));
    let hash = (hash ^ byte) & e.mem.u32(state + DS_HASH_MASK);
    e.mem.set_u32(state + DS_INSERT_HASH, hash);
    let hash = e.mem.u32(state + DS_INSERT_HASH);
    let head = e.mem.u32(state + DS_HEAD);
    let slot = e.mem.u32(state + DS_STRING_START) & e.mem.u32(state + DS_WINDOW_MASK);
    let previous = e.mem.u32(state + DS_PREVIOUS);
    let value = e.mem.u16(head.wrapping_add(hash.wrapping_mul(2)));
    e.mem
        .set_u16(previous.wrapping_add(slot.wrapping_mul(2)), value);
    let slot = e.mem.u32(state + DS_STRING_START) & e.mem.u32(state + DS_WINDOW_MASK);
    let previous = e.mem.u32(state + DS_PREVIOUS);
    let hash_head = u32::from(e.mem.u16(previous.wrapping_add(slot.wrapping_mul(2))));
    let hash = e.mem.u32(state + DS_INSERT_HASH);
    let head = e.mem.u32(state + DS_HEAD);
    let start = e.mem.u16(state + DS_STRING_START);
    e.mem
        .set_u16(head.wrapping_add(hash.wrapping_mul(2)), start);
    hash_head
}

/// The insert hash restarted from the two bytes at the string start.
fn restart_hash(e: &mut Engine, state: u32) {
    let window = e.mem.u32(state + DS_WINDOW);
    let start = e.mem.u32(state + DS_STRING_START);
    e.mem.set_u32(
        state + DS_INSERT_HASH,
        u32::from(e.mem.u8(window.wrapping_add(start))),
    );
    let hash = e
        .mem
        .u32(state + DS_INSERT_HASH)
        .wrapping_shl(e.mem.u32(state + DS_HASH_SHIFT));
    let start = e.mem.u32(state + DS_STRING_START);
    let window = e.mem.u32(state + DS_WINDOW);
    let byte = u32::from(e.mem.u8(window.wrapping_add(start).wrapping_add(1)));
    let hash = (hash ^ byte) & e.mem.u32(state + DS_HASH_MASK);
    e.mem.set_u32(state + DS_INSERT_HASH, hash);
}

/// One more count in a frequency table entry (`u16`, wrapping).
fn count_frequency(e: &mut Engine, address: u32) {
    let count = e.mem.u16(address).wrapping_add(1);
    e.mem.set_u16(address, count);
}

/// The exe's inlined literal tally: distance 0 and the byte into the buffers, its
/// frequency counted; answers whether the literal buffer is now full (one short of its
/// size).
fn tally_literal(e: &mut Engine, state: u32, byte: u8) -> bool {
    let last = e.mem.u32(state + DS_LAST_LITERAL);
    let distances = e.mem.u32(state + DS_DISTANCE_BUFFER);
    e.mem
        .set_u16(distances.wrapping_add(last.wrapping_mul(2)), 0);
    let literals = e.mem.u32(state + DS_LITERAL_BUFFER);
    let last = e.mem.u32(state + DS_LAST_LITERAL);
    e.mem.set_u8(literals.wrapping_add(last), byte);
    let last = e.mem.u32(state + DS_LAST_LITERAL).wrapping_add(1);
    e.mem.set_u32(state + DS_LAST_LITERAL, last);
    count_frequency(e, state + DS_LITERAL_FREQUENCIES + 4 * u32::from(byte));
    e.mem.u32(state + DS_LAST_LITERAL) == e.mem.u32(state + DS_LITERAL_BUFFER_SIZE).wrapping_sub(1)
}

/// The exe's inlined match tally: the distance (`u16`) and the length less 3 (a byte) into
/// the buffers, the length code's and distance code's frequencies counted; answers
/// whether the literal buffer is now full.
fn tally_distance(e: &mut Engine, state: u32, distance: u16, length: u8) -> bool {
    let last = e.mem.u32(state + DS_LAST_LITERAL);
    let distances = e.mem.u32(state + DS_DISTANCE_BUFFER);
    e.mem
        .set_u16(distances.wrapping_add(last.wrapping_mul(2)), distance);
    let literals = e.mem.u32(state + DS_LITERAL_BUFFER);
    let last = e.mem.u32(state + DS_LAST_LITERAL);
    e.mem.set_u8(literals.wrapping_add(last), length);
    let last = e.mem.u32(state + DS_LAST_LITERAL).wrapping_add(1);
    e.mem.set_u32(state + DS_LAST_LITERAL, last);
    let distance = distance.wrapping_sub(1);
    let code = u32::from(e.mem.u8(LENGTH_CODE + u32::from(length)));
    count_frequency(e, state + DS_LENGTH_FREQUENCIES + 4 * code);
    let code = if distance < 0x100 {
        u32::from(e.mem.u8(DISTANCE_CODE + u32::from(distance)))
    } else {
        u32::from(e.mem.u8(DISTANCE_CODE + 0x100 + (u32::from(distance) >> 7)))
    };
    count_frequency(e, state + DS_DISTANCE_FREQUENCIES + 4 * code);
    e.mem.u32(state + DS_LAST_LITERAL) == e.mem.u32(state + DS_LITERAL_BUFFER_SIZE).wrapping_sub(1)
}

/// The exe's inlined block flush: `00b4b250` with the window at the block start (null for a
/// negative start), the length since it and the last-block flag; the block start moves to
/// the string start and the pending bytes are flushed (`fn_00b472a0`). Answers whether the
/// output still has room.
fn flush_block(e: &mut Engine, state: u32, last: bool) -> bool {
    let start = e.mem.u32(state + DS_BLOCK_START);
    let block = if (start as i32) < 0 {
        0
    } else {
        e.mem.u32(state + DS_WINDOW).wrapping_add(start)
    };
    let length = e
        .mem
        .u32(state + DS_STRING_START)
        .wrapping_sub(e.mem.u32(state + DS_BLOCK_START));
    e.call(
        TREES_FLUSH_BLOCK,
        &args![state, block, length, u32::from(last)],
    );
    let start = e.mem.u32(state + DS_STRING_START);
    e.mem.set_u32(state + DS_BLOCK_START, start);
    let stream = e.mem.u32(state + DS_STREAM);
    fn_00b472a0(e, stream);
    let stream = e.mem.u32(state + DS_STREAM);
    e.mem.u32(stream + STREAM_OUTPUT_LEFT) != 0
}

/// The compress functions' answer after the final block: finish-started (2) or need-more
/// (0) without room, finish-done (3) or block-done (1) with it, by whether the flush is 4.
fn finish_answer(room: bool, flush: u32) -> u32 {
    match (room, flush == 4) {
        (false, true) => 2,
        (false, false) => 0,
        (true, true) => 3,
        (true, false) => 1,
    }
}

// Translated from 00b47490 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map; zlib's `deflateCopy` by its body (`cdecl`, two words: the
/// destination and source stream records). -2 when either is null or the source has no
/// state. Copies the 14 words of the record, allocates a state (through the copied
/// allocator; -4 when it can't) and copies the source's into it (`memcpy`), pointing it at
/// the destination; allocates the window, previous and head chains and the pending buffer
/// (any missing takes the copy down with `fn_00b47370` and answers -4) and copies them;
/// rebases the pending position, the literal and distance buffers and the three tree
/// descriptors' tables into the copy. Answers 0.
pub fn fn_00b47490(e: &mut Engine, destination: u32, source: u32) -> i32 {
    if source == 0 || destination == 0 || e.mem.u32(source + STREAM_STATE) == 0 {
        return -2;
    }
    let old = e.mem.u32(source + STREAM_STATE);
    for word in 0..14 {
        let value = e.mem.u32(source + 4 * word);
        e.mem.set_u32(destination + 4 * word, value);
    }
    let allocate = |e: &mut Engine, count: u32, item: u32| {
        let function = e.mem.u32(destination + STREAM_ALLOCATE);
        let opaque = e.mem.u32(destination + STREAM_OPAQUE);
        e.call(function, &args![opaque, count, item]).u32()
    };
    let state = allocate(e, 1, DEFLATE_STATE_SIZE);
    if state == 0 {
        return -4;
    }
    e.mem.set_u32(destination + STREAM_STATE, state);
    e.call(MEMCPY, &args![state, old, DEFLATE_STATE_SIZE]);
    e.mem.set_u32(state + DS_STREAM, destination);
    let count = e.mem.u32(state + DS_WINDOW_SIZE);
    let window = allocate(e, count, 2);
    e.mem.set_u32(state + DS_WINDOW, window);
    let count = e.mem.u32(state + DS_WINDOW_SIZE);
    let previous = allocate(e, count, 2);
    e.mem.set_u32(state + DS_PREVIOUS, previous);
    let count = e.mem.u32(state + DS_HASH_SIZE);
    let head = allocate(e, count, 2);
    e.mem.set_u32(state + DS_HEAD, head);
    let count = e.mem.u32(state + DS_LITERAL_BUFFER_SIZE);
    let overlay = allocate(e, count, 4);
    e.mem.set_u32(state + DS_PENDING_BUFFER, overlay);
    if e.mem.u32(state + DS_WINDOW) == 0
        || e.mem.u32(state + DS_PREVIOUS) == 0
        || e.mem.u32(state + DS_HEAD) == 0
        || e.mem.u32(state + DS_PENDING_BUFFER) == 0
    {
        fn_00b47370(e, destination);
        return -4;
    }
    for (offset, size) in [
        (DS_WINDOW, DS_WINDOW_SIZE),
        (DS_PREVIOUS, DS_WINDOW_SIZE),
        (DS_HEAD, DS_HASH_SIZE),
    ] {
        let length = e.mem.u32(state + size).wrapping_shl(1);
        let from = e.mem.u32(old + offset);
        let to = e.mem.u32(state + offset);
        e.call(MEMCPY, &args![to, from, length]);
    }
    let length = e.mem.u32(state + DS_PENDING_BUFFER_SIZE);
    let from = e.mem.u32(old + DS_PENDING_BUFFER);
    let to = e.mem.u32(state + DS_PENDING_BUFFER);
    e.call(MEMCPY, &args![to, from, length]);
    let out = e
        .mem
        .u32(old + DS_PENDING_OUT)
        .wrapping_sub(e.mem.u32(old + DS_PENDING_BUFFER))
        .wrapping_add(e.mem.u32(state + DS_PENDING_BUFFER));
    e.mem.set_u32(state + DS_PENDING_OUT, out);
    let literals = e.mem.u32(state + DS_LITERAL_BUFFER_SIZE);
    e.mem.set_u32(
        state + DS_DISTANCE_BUFFER,
        overlay.wrapping_add((literals >> 1).wrapping_mul(2)),
    );
    let literals = e.mem.u32(state + DS_LITERAL_BUFFER_SIZE);
    let pending = e.mem.u32(state + DS_PENDING_BUFFER);
    e.mem.set_u32(
        state + DS_LITERAL_BUFFER,
        literals.wrapping_mul(3).wrapping_add(pending),
    );
    e.mem.set_u32(
        state + DS_LITERAL_DESCRIPTOR,
        state + DS_LITERAL_FREQUENCIES,
    );
    e.mem.set_u32(
        state + DS_DISTANCE_DESCRIPTOR,
        state + DS_DISTANCE_FREQUENCIES,
    );
    e.mem.set_u32(
        state + DS_BIT_LENGTH_DESCRIPTOR,
        state + DS_BIT_LENGTH_FREQUENCIES,
    );
    0
}

// Translated from 00b476f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map; zlib's `lm_init` by its body (`cdecl`, one word: the
/// compressor state): the double window size, the head chain cleared (its last entry,
/// then the rest with `memset`), the level's lazy, good, nice and chain values from the
/// level table, the string and block starts and the lookahead 0, the match and previous
/// lengths 2, no match available and the insert hash 0.
pub fn fn_00b476f0(e: &mut Engine, state: u32) {
    let double = e.mem.u32(state + DS_WINDOW_SIZE).wrapping_shl(1);
    e.mem.set_u32(state + DS_WINDOW_DOUBLE, double);
    let size = e.mem.u32(state + DS_HASH_SIZE);
    let head = e.mem.u32(state + DS_HEAD);
    e.mem
        .set_u16(head.wrapping_add(size.wrapping_mul(2)).wrapping_sub(2), 0);
    let size = e.mem.u32(state + DS_HASH_SIZE);
    let head = e.mem.u32(state + DS_HEAD);
    e.call(
        MEMSET,
        &args![head, 0u32, size.wrapping_mul(2).wrapping_sub(2)],
    );
    let row = level_row(e.mem.u32(state + DS_LEVEL));
    e.mem
        .set_u32(state + DS_MAX_LAZY, u32::from(e.mem.u16(row + 2)));
    let row = level_row(e.mem.u32(state + DS_LEVEL));
    e.mem
        .set_u32(state + DS_GOOD_MATCH, u32::from(e.mem.u16(row)));
    let row = level_row(e.mem.u32(state + DS_LEVEL));
    e.mem
        .set_u32(state + DS_NICE_MATCH, u32::from(e.mem.u16(row + 4)));
    let row = level_row(e.mem.u32(state + DS_LEVEL));
    e.mem
        .set_u32(state + DS_MAX_CHAIN, u32::from(e.mem.u16(row + 6)));
    e.mem.set_u32(state + DS_STRING_START, 0);
    e.mem.set_u32(state + DS_BLOCK_START, 0);
    e.mem.set_u32(state + DS_LOOKAHEAD, 0);
    e.mem.set_u32(state + DS_PREVIOUS_LENGTH, 2);
    e.mem.set_u32(state + DS_MATCH_LENGTH, 2);
    e.mem.set_u32(state + DS_MATCH_AVAILABLE, 0);
    e.mem.set_u32(state + DS_INSERT_HASH, 0);
}

// Translated from 00b477e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map; zlib's `deflate_stored` by its body (`cdecl`, two words: the
/// compressor state and the flush kind). Blocks are at most 0xffff bytes (or the pending
/// buffer size less 5). It fills the window (`fn_00b47a20`) while at most one byte is
/// ahead (nothing and no flush answers need-more, 0; nothing with a flush ends the loop),
/// takes the lookahead into the string, and emits a stored block (`00b4b250`, then
/// `fn_00b472a0`; no room answers 0) when the string reaches the block's limit (or is at
/// 0) or the window margin. Then the final block with the last flag for flush 4; answers
/// as `finish_answer`.
pub fn fn_00b477e0(e: &mut Engine, state: u32, flush: u32) -> u32 {
    let mut limit = 0xffffu32;
    if limit > e.mem.u32(state + DS_PENDING_BUFFER_SIZE).wrapping_sub(5) {
        limit = e.mem.u32(state + DS_PENDING_BUFFER_SIZE).wrapping_sub(5);
    }
    loop {
        if e.mem.u32(state + DS_LOOKAHEAD) <= 1 {
            fn_00b47a20(e, state);
            if e.mem.u32(state + DS_LOOKAHEAD) == 0 && flush == 0 {
                return 0;
            }
            if e.mem.u32(state + DS_LOOKAHEAD) == 0 {
                break;
            }
        }
        let start = e
            .mem
            .u32(state + DS_STRING_START)
            .wrapping_add(e.mem.u32(state + DS_LOOKAHEAD));
        e.mem.set_u32(state + DS_STRING_START, start);
        e.mem.set_u32(state + DS_LOOKAHEAD, 0);
        let max_start = e.mem.u32(state + DS_BLOCK_START).wrapping_add(limit);
        let start = e.mem.u32(state + DS_STRING_START);
        if start == 0 || start >= max_start {
            let ahead = e.mem.u32(state + DS_STRING_START).wrapping_sub(max_start);
            e.mem.set_u32(state + DS_LOOKAHEAD, ahead);
            e.mem.set_u32(state + DS_STRING_START, max_start);
            if !flush_block(e, state, false) {
                return 0;
            }
        }
        let since = e
            .mem
            .u32(state + DS_STRING_START)
            .wrapping_sub(e.mem.u32(state + DS_BLOCK_START));
        if since
            >= e.mem
                .u32(state + DS_WINDOW_SIZE)
                .wrapping_sub(MIN_LOOKAHEAD)
            && !flush_block(e, state, false)
        {
            return 0;
        }
    }
    let room = flush_block(e, state, flush == 4);
    finish_answer(room, flush)
}

// Translated from 00b47a20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map; zlib's `fill_window` by its body (`cdecl`, one word: the
/// compressor state). Repeats while the lookahead is below 0x106 and input is left: when
/// the string start reaches the window size plus the window size less 0x106, the upper
/// half slides down (`memcpy`), the match, string and block starts move back and every
/// head and previous-chain entry drops by the window size (0 below it); then input is read
/// (`fn_00b47c50`) after the lookahead and, with at least 3 bytes ahead, the insert hash
/// restarts. The exe's 16-bit-int branch (`sizeof(int) <= 2`, compiled as `XOR EAX,EAX;
/// JZ`) never runs and is left out.
pub fn fn_00b47a20(e: &mut Engine, state: u32) {
    let size = e.mem.u32(state + DS_WINDOW_SIZE);
    loop {
        let mut more = e
            .mem
            .u32(state + DS_WINDOW_DOUBLE)
            .wrapping_sub(e.mem.u32(state + DS_LOOKAHEAD))
            .wrapping_sub(e.mem.u32(state + DS_STRING_START));
        let limit = size
            .wrapping_add(e.mem.u32(state + DS_WINDOW_SIZE))
            .wrapping_sub(MIN_LOOKAHEAD);
        if e.mem.u32(state + DS_STRING_START) >= limit {
            let window = e.mem.u32(state + DS_WINDOW);
            let window_upper = e.mem.u32(state + DS_WINDOW).wrapping_add(size);
            e.call(MEMCPY, &args![window, window_upper, size]);
            for offset in [DS_MATCH_START, DS_STRING_START, DS_BLOCK_START] {
                let value = e.mem.u32(state + offset).wrapping_sub(size);
                e.mem.set_u32(state + offset, value);
            }
            for (table, count) in [
                (e.mem.u32(state + DS_HEAD), e.mem.u32(state + DS_HASH_SIZE)),
                (e.mem.u32(state + DS_PREVIOUS), size),
            ] {
                let mut entry = table.wrapping_add(count.wrapping_mul(2));
                let mut left = count;
                loop {
                    entry = entry.wrapping_sub(2);
                    let value = u32::from(e.mem.u16(entry));
                    let slid = if value >= size {
                        value.wrapping_sub(size)
                    } else {
                        0
                    };
                    e.mem.set_u16(entry, slid as u16);
                    left = left.wrapping_sub(1);
                    if left == 0 {
                        break;
                    }
                }
            }
            more = more.wrapping_add(size);
        }
        let stream = e.mem.u32(state + DS_STREAM);
        if e.mem.u32(stream + STREAM_INPUT_LEFT) == 0 {
            return;
        }
        let at = e
            .mem
            .u32(state + DS_WINDOW)
            .wrapping_add(e.mem.u32(state + DS_STRING_START))
            .wrapping_add(e.mem.u32(state + DS_LOOKAHEAD));
        let stream = e.mem.u32(state + DS_STREAM);
        let read = fn_00b47c50(e, stream, at, more);
        let ahead = e.mem.u32(state + DS_LOOKAHEAD).wrapping_add(read);
        e.mem.set_u32(state + DS_LOOKAHEAD, ahead);
        if e.mem.u32(state + DS_LOOKAHEAD) >= 3 {
            restart_hash(e, state);
        }
        let stream = e.mem.u32(state + DS_STREAM);
        if e.mem.u32(state + DS_LOOKAHEAD) >= MIN_LOOKAHEAD
            || e.mem.u32(stream + STREAM_INPUT_LEFT) == 0
        {
            return;
        }
    }
}

// Translated from 00b47c50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map; zlib's `read_buf` by its body (`cdecl`, three words: the
/// stream record, a buffer and its size): takes up to the size from the input; the
/// stream's checksum takes the bytes (`00b49e20` for the `78 xx` kind, `00b4a070` for the
/// `1f 8b` kind) before they are copied (`memcpy`); moves the input and the total in.
/// Answers the count (0 with nothing to take).
pub fn fn_00b47c50(e: &mut Engine, stream: u32, buffer: u32, size: u32) -> u32 {
    let mut length = e.mem.u32(stream + STREAM_INPUT_LEFT);
    if length > size {
        length = size;
    }
    if length == 0 {
        return 0;
    }
    let left = e.mem.u32(stream + STREAM_INPUT_LEFT).wrapping_sub(length);
    e.mem.set_u32(stream + STREAM_INPUT_LEFT, left);
    let wrap = e.mem.u32(e.mem.u32(stream + STREAM_STATE) + DS_WRAP);
    if wrap == 1 {
        let adler = e.mem.u32(stream + STREAM_ADLER);
        let input = e.mem.u32(stream + STREAM_INPUT);
        let adler = e.call(DATA_CHECKSUM, &args![adler, input, length]).u32();
        e.mem.set_u32(stream + STREAM_ADLER, adler);
    } else if e.mem.u32(e.mem.u32(stream + STREAM_STATE) + DS_WRAP) == 2 {
        let crc = e.mem.u32(stream + STREAM_ADLER);
        let input = e.mem.u32(stream + STREAM_INPUT);
        let crc = e
            .call(HEADER_CHECKSUM_00B4A070, &args![crc, input, length])
            .u32();
        e.mem.set_u32(stream + STREAM_ADLER, crc);
    }
    let input = e.mem.u32(stream + STREAM_INPUT);
    e.call(MEMCPY, &args![buffer, input, length]);
    let next = e.mem.u32(stream + STREAM_INPUT).wrapping_add(length);
    e.mem.set_u32(stream + STREAM_INPUT, next);
    let total = e.mem.u32(stream + STREAM_TOTAL_IN).wrapping_add(length);
    e.mem.set_u32(stream + STREAM_TOTAL_IN, total);
    length
}

/// The match search the fast and lazy compressors share: `fn_00b482b0` for strategies
/// below 2, `fn_00b485c0` for strategy 3 when the match is one byte back; others keep the
/// match length.
fn search_match(e: &mut Engine, state: u32, hash_head: u32) {
    let strategy = e.mem.u32(state + DS_STRATEGY) as i32;
    if strategy < 2 {
        let length = fn_00b482b0(e, state, hash_head);
        e.mem.set_u32(state + DS_MATCH_LENGTH, length);
    } else if strategy == 3 && e.mem.u32(state + DS_STRING_START).wrapping_sub(hash_head) == 1 {
        let length = fn_00b485c0(e, state, hash_head);
        e.mem.set_u32(state + DS_MATCH_LENGTH, length);
    }
}

// Translated from 00b47d20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map; zlib's `deflate_fast` by its body (`cdecl`, two words: the
/// compressor state and the flush kind). Each step fills the window below 0x106 bytes
/// ahead (still short without a flush answers 0; nothing ahead ends), inserts the string
/// (with 3 or more ahead), searches a match within the window distance, and tallies it
/// (inserting the matched strings when the match is at most the lazy limit and 3 or more
/// bytes stay ahead, else restarting the hash past it) or a literal; a full literal buffer
/// flushes a block (no room answers 0). The final block carries the last flag for flush 4;
/// answers as `finish_answer`.
pub fn fn_00b47d20(e: &mut Engine, state: u32, flush: u32) -> u32 {
    let mut hash_head = 0u32;
    loop {
        if e.mem.u32(state + DS_LOOKAHEAD) < MIN_LOOKAHEAD {
            fn_00b47a20(e, state);
            if e.mem.u32(state + DS_LOOKAHEAD) < MIN_LOOKAHEAD && flush == 0 {
                return 0;
            }
            if e.mem.u32(state + DS_LOOKAHEAD) == 0 {
                break;
            }
        }
        if e.mem.u32(state + DS_LOOKAHEAD) >= 3 {
            hash_head = insert_string(e, state);
        }
        if hash_head != 0
            && e.mem.u32(state + DS_STRING_START).wrapping_sub(hash_head)
                <= e.mem
                    .u32(state + DS_WINDOW_SIZE)
                    .wrapping_sub(MIN_LOOKAHEAD)
        {
            search_match(e, state, hash_head);
        }
        let full;
        if e.mem.u32(state + DS_MATCH_LENGTH) >= 3 {
            let length = e.mem.u32(state + DS_MATCH_LENGTH).wrapping_sub(3) as u8;
            let distance = e
                .mem
                .u32(state + DS_STRING_START)
                .wrapping_sub(e.mem.u32(state + DS_MATCH_START)) as u16;
            full = tally_distance(e, state, distance, length);
            let ahead = e
                .mem
                .u32(state + DS_LOOKAHEAD)
                .wrapping_sub(e.mem.u32(state + DS_MATCH_LENGTH));
            e.mem.set_u32(state + DS_LOOKAHEAD, ahead);
            if e.mem.u32(state + DS_MATCH_LENGTH) <= e.mem.u32(state + DS_MAX_LAZY)
                && e.mem.u32(state + DS_LOOKAHEAD) >= 3
            {
                let length = e.mem.u32(state + DS_MATCH_LENGTH).wrapping_sub(1);
                e.mem.set_u32(state + DS_MATCH_LENGTH, length);
                loop {
                    let start = e.mem.u32(state + DS_STRING_START).wrapping_add(1);
                    e.mem.set_u32(state + DS_STRING_START, start);
                    hash_head = insert_string(e, state);
                    let length = e.mem.u32(state + DS_MATCH_LENGTH).wrapping_sub(1);
                    e.mem.set_u32(state + DS_MATCH_LENGTH, length);
                    if e.mem.u32(state + DS_MATCH_LENGTH) == 0 {
                        break;
                    }
                }
                let start = e.mem.u32(state + DS_STRING_START).wrapping_add(1);
                e.mem.set_u32(state + DS_STRING_START, start);
            } else {
                let start = e
                    .mem
                    .u32(state + DS_STRING_START)
                    .wrapping_add(e.mem.u32(state + DS_MATCH_LENGTH));
                e.mem.set_u32(state + DS_STRING_START, start);
                e.mem.set_u32(state + DS_MATCH_LENGTH, 0);
                restart_hash(e, state);
            }
        } else {
            let window = e.mem.u32(state + DS_WINDOW);
            let start = e.mem.u32(state + DS_STRING_START);
            let byte = e.mem.u8(window.wrapping_add(start));
            full = tally_literal(e, state, byte);
            let ahead = e.mem.u32(state + DS_LOOKAHEAD).wrapping_sub(1);
            e.mem.set_u32(state + DS_LOOKAHEAD, ahead);
            let start = e.mem.u32(state + DS_STRING_START).wrapping_add(1);
            e.mem.set_u32(state + DS_STRING_START, start);
        }
        if full && !flush_block(e, state, false) {
            return 0;
        }
    }
    let room = flush_block(e, state, flush == 4);
    finish_answer(room, flush)
}

// Translated from 00b482b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map; zlib's `longest_match` by its body (`cdecl`, two words: the
/// compressor state and the chain head). Walks the previous chain from the head while it
/// stays above the window limit and the chain budget (a quarter of it once the previous
/// length reaches the good length) lasts: a candidate whose bytes at the best length and
/// one before, and whose first two bytes, agree with the string is compared on (in steps
/// of 8 up to 0x102 bytes); a longer one becomes the match start and best length, ending
/// the walk at the nice length (itself at most the lookahead). Answers the best length,
/// at most the lookahead.
pub fn fn_00b482b0(e: &mut Engine, state: u32, head: u32) -> u32 {
    let mut chain = e.mem.u32(state + DS_MAX_CHAIN);
    let mut scan = e
        .mem
        .u32(state + DS_WINDOW)
        .wrapping_add(e.mem.u32(state + DS_STRING_START));
    let mut best = e.mem.u32(state + DS_PREVIOUS_LENGTH);
    let mut nice = e.mem.u32(state + DS_NICE_MATCH);
    let limit = if e.mem.u32(state + DS_STRING_START)
        > e.mem
            .u32(state + DS_WINDOW_SIZE)
            .wrapping_sub(MIN_LOOKAHEAD)
    {
        e.mem.u32(state + DS_STRING_START).wrapping_sub(
            e.mem
                .u32(state + DS_WINDOW_SIZE)
                .wrapping_sub(MIN_LOOKAHEAD),
        )
    } else {
        0
    };
    let previous = e.mem.u32(state + DS_PREVIOUS);
    let mask = e.mem.u32(state + DS_WINDOW_MASK);
    let end = e
        .mem
        .u32(state + DS_WINDOW)
        .wrapping_add(e.mem.u32(state + DS_STRING_START))
        .wrapping_add(MAX_MATCH);
    let mut end_before = e.mem.u8(scan.wrapping_add(best).wrapping_sub(1));
    let mut end_byte = e.mem.u8(scan.wrapping_add(best));
    if e.mem.u32(state + DS_PREVIOUS_LENGTH) >= e.mem.u32(state + DS_GOOD_MATCH) {
        chain >>= 2;
    }
    if nice > e.mem.u32(state + DS_LOOKAHEAD) {
        nice = e.mem.u32(state + DS_LOOKAHEAD);
    }
    let mut current = head;
    loop {
        let mut candidate = e.mem.u32(state + DS_WINDOW).wrapping_add(current);
        let starts_alike = e.mem.u8(candidate.wrapping_add(best)) == end_byte
            && e.mem.u8(candidate.wrapping_add(best).wrapping_sub(1)) == end_before
            && e.mem.u8(candidate) == e.mem.u8(scan)
            && {
                candidate = candidate.wrapping_add(1);
                e.mem.u8(candidate) == e.mem.u8(scan.wrapping_add(1))
            };
        if starts_alike {
            scan = scan.wrapping_add(2);
            candidate = candidate.wrapping_add(1);
            'compare: loop {
                for _ in 0..8 {
                    scan = scan.wrapping_add(1);
                    candidate = candidate.wrapping_add(1);
                    if e.mem.u8(candidate) != e.mem.u8(scan) {
                        break 'compare;
                    }
                }
                if scan >= end {
                    break;
                }
            }
            let length = MAX_MATCH.wrapping_sub(end.wrapping_sub(scan)) as i32;
            scan = end.wrapping_sub(MAX_MATCH);
            if length > best as i32 {
                e.mem.set_u32(state + DS_MATCH_START, current);
                best = length as u32;
                if length >= nice as i32 {
                    break;
                }
                end_before = e.mem.u8(scan.wrapping_add(best).wrapping_sub(1));
                end_byte = e.mem.u8(scan.wrapping_add(best));
            }
        }
        current = u32::from(
            e.mem
                .u16(previous.wrapping_add((current & mask).wrapping_mul(2))),
        );
        if current <= limit {
            break;
        }
        chain = chain.wrapping_sub(1);
        if chain == 0 {
            break;
        }
    }
    if best <= e.mem.u32(state + DS_LOOKAHEAD) {
        best
    } else {
        e.mem.u32(state + DS_LOOKAHEAD)
    }
}

// Translated from 00b485c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map; zlib's `longest_match_fast` by its body (`cdecl`, two words:
/// the compressor state and the candidate): 2 unless the candidate's first two bytes match
/// the string's; then compares on (steps of 8 up to 0x102 bytes); below 3 answers 2, else
/// the candidate becomes the match start and the length (at most the lookahead) is
/// answered.
pub fn fn_00b485c0(e: &mut Engine, state: u32, candidate: u32) -> u32 {
    let mut scan = e
        .mem
        .u32(state + DS_WINDOW)
        .wrapping_add(e.mem.u32(state + DS_STRING_START));
    let end = e
        .mem
        .u32(state + DS_WINDOW)
        .wrapping_add(e.mem.u32(state + DS_STRING_START))
        .wrapping_add(MAX_MATCH);
    let mut matched = e.mem.u32(state + DS_WINDOW).wrapping_add(candidate);
    if e.mem.u8(matched) != e.mem.u8(scan)
        || e.mem.u8(matched.wrapping_add(1)) != e.mem.u8(scan.wrapping_add(1))
    {
        return 2;
    }
    scan = scan.wrapping_add(2);
    matched = matched.wrapping_add(2);
    'compare: loop {
        for _ in 0..8 {
            scan = scan.wrapping_add(1);
            matched = matched.wrapping_add(1);
            if e.mem.u8(scan) != e.mem.u8(matched) {
                break 'compare;
            }
        }
        if scan >= end {
            break;
        }
    }
    let length = MAX_MATCH.wrapping_sub(end.wrapping_sub(scan)) as i32;
    if length < 3 {
        return 2;
    }
    e.mem.set_u32(state + DS_MATCH_START, candidate);
    let length = length as u32;
    if length <= e.mem.u32(state + DS_LOOKAHEAD) {
        length
    } else {
        e.mem.u32(state + DS_LOOKAHEAD)
    }
}

// Translated from 00b487b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map; zlib's `deflate_slow` by its body (`cdecl`, two words: the
/// compressor state and the flush kind). Like `fn_00b47d20` but lazy: each step keeps the
/// previous match, searches a new one while the previous is below the lazy limit (a match
/// of at most 5 is dropped for strategy 1, or a 3-byte one more than 0x1000 back); a
/// previous match at least as long as the new one is tallied (its strings inserted up to
/// the last full string, the lazy state cleared), otherwise the previous byte goes out as
/// a literal when one was waiting (a full buffer flushing a block) or a match becomes
/// waiting. Full buffers flush blocks (no room answers 0). At the end a waiting literal is
/// tallied and the final block flushed with the last flag for flush 4; answers as
/// `finish_answer`.
pub fn fn_00b487b0(e: &mut Engine, state: u32, flush: u32) -> u32 {
    let mut hash_head = 0u32;
    loop {
        if e.mem.u32(state + DS_LOOKAHEAD) < MIN_LOOKAHEAD {
            fn_00b47a20(e, state);
            if e.mem.u32(state + DS_LOOKAHEAD) < MIN_LOOKAHEAD && flush == 0 {
                return 0;
            }
            if e.mem.u32(state + DS_LOOKAHEAD) == 0 {
                break;
            }
        }
        if e.mem.u32(state + DS_LOOKAHEAD) >= 3 {
            hash_head = insert_string(e, state);
        }
        let length = e.mem.u32(state + DS_MATCH_LENGTH);
        e.mem.set_u32(state + DS_PREVIOUS_LENGTH, length);
        let start = e.mem.u32(state + DS_MATCH_START);
        e.mem.set_u32(state + DS_PREVIOUS_MATCH, start);
        e.mem.set_u32(state + DS_MATCH_LENGTH, 2);
        if hash_head != 0
            && e.mem.u32(state + DS_PREVIOUS_LENGTH) < e.mem.u32(state + DS_MAX_LAZY)
            && e.mem.u32(state + DS_STRING_START).wrapping_sub(hash_head)
                <= e.mem
                    .u32(state + DS_WINDOW_SIZE)
                    .wrapping_sub(MIN_LOOKAHEAD)
        {
            search_match(e, state, hash_head);
            if e.mem.u32(state + DS_MATCH_LENGTH) <= 5
                && (e.mem.u32(state + DS_STRATEGY) == 1
                    || (e.mem.u32(state + DS_MATCH_LENGTH) == 3
                        && e.mem
                            .u32(state + DS_STRING_START)
                            .wrapping_sub(e.mem.u32(state + DS_MATCH_START))
                            > 0x1000))
            {
                e.mem.set_u32(state + DS_MATCH_LENGTH, 2);
            }
        }
        if e.mem.u32(state + DS_PREVIOUS_LENGTH) >= 3
            && e.mem.u32(state + DS_MATCH_LENGTH) <= e.mem.u32(state + DS_PREVIOUS_LENGTH)
        {
            let max_insert = e
                .mem
                .u32(state + DS_STRING_START)
                .wrapping_add(e.mem.u32(state + DS_LOOKAHEAD))
                .wrapping_sub(3);
            let length = e.mem.u32(state + DS_PREVIOUS_LENGTH).wrapping_sub(3) as u8;
            let distance =
                e.mem
                    .u32(state + DS_STRING_START)
                    .wrapping_sub(1)
                    .wrapping_sub(e.mem.u32(state + DS_PREVIOUS_MATCH)) as u16;
            let full = tally_distance(e, state, distance, length);
            let ahead = e
                .mem
                .u32(state + DS_LOOKAHEAD)
                .wrapping_sub(e.mem.u32(state + DS_PREVIOUS_LENGTH).wrapping_sub(1));
            e.mem.set_u32(state + DS_LOOKAHEAD, ahead);
            let length = e.mem.u32(state + DS_PREVIOUS_LENGTH).wrapping_sub(2);
            e.mem.set_u32(state + DS_PREVIOUS_LENGTH, length);
            loop {
                let start = e.mem.u32(state + DS_STRING_START).wrapping_add(1);
                e.mem.set_u32(state + DS_STRING_START, start);
                if e.mem.u32(state + DS_STRING_START) <= max_insert {
                    hash_head = insert_string(e, state);
                }
                let length = e.mem.u32(state + DS_PREVIOUS_LENGTH).wrapping_sub(1);
                e.mem.set_u32(state + DS_PREVIOUS_LENGTH, length);
                if e.mem.u32(state + DS_PREVIOUS_LENGTH) == 0 {
                    break;
                }
            }
            e.mem.set_u32(state + DS_MATCH_AVAILABLE, 0);
            e.mem.set_u32(state + DS_MATCH_LENGTH, 2);
            let start = e.mem.u32(state + DS_STRING_START).wrapping_add(1);
            e.mem.set_u32(state + DS_STRING_START, start);
            if full && !flush_block(e, state, false) {
                return 0;
            }
        } else if e.mem.u32(state + DS_MATCH_AVAILABLE) != 0 {
            let window = e.mem.u32(state + DS_WINDOW);
            let start = e.mem.u32(state + DS_STRING_START);
            let byte = e.mem.u8(window.wrapping_add(start).wrapping_sub(1));
            if tally_literal(e, state, byte) {
                flush_block(e, state, false);
            }
            let start = e.mem.u32(state + DS_STRING_START).wrapping_add(1);
            e.mem.set_u32(state + DS_STRING_START, start);
            let ahead = e.mem.u32(state + DS_LOOKAHEAD).wrapping_sub(1);
            e.mem.set_u32(state + DS_LOOKAHEAD, ahead);
            let stream = e.mem.u32(state + DS_STREAM);
            if e.mem.u32(stream + STREAM_OUTPUT_LEFT) == 0 {
                return 0;
            }
        } else {
            e.mem.set_u32(state + DS_MATCH_AVAILABLE, 1);
            let start = e.mem.u32(state + DS_STRING_START).wrapping_add(1);
            e.mem.set_u32(state + DS_STRING_START, start);
            let ahead = e.mem.u32(state + DS_LOOKAHEAD).wrapping_sub(1);
            e.mem.set_u32(state + DS_LOOKAHEAD, ahead);
        }
    }
    if e.mem.u32(state + DS_MATCH_AVAILABLE) != 0 {
        let window = e.mem.u32(state + DS_WINDOW);
        let start = e.mem.u32(state + DS_STRING_START);
        let byte = e.mem.u8(window.wrapping_add(start).wrapping_sub(1));
        tally_literal(e, state, byte);
        e.mem.set_u32(state + DS_MATCH_AVAILABLE, 0);
    }
    let room = flush_block(e, state, flush == 4);
    finish_answer(room, flush)
}

// Translated from 00b48ee0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (no callers found); zlib's `zlibVersion` by its body (no
/// words): the address of the version string `011ad714`.
pub fn fn_00b48ee0(_e: &mut Engine) -> u32 {
    0x011a_d714
}

/// One size field of `fn_00b48ef0`: nothing for 2 bytes, `1 << shift` for 4, `2 << shift`
/// for 8, `3 << shift` otherwise.
fn size_flags(size: u32, shift: u32) -> u32 {
    match size {
        2 => 0,
        4 => 1 << shift,
        8 => 2 << shift,
        _ => 3 << shift,
    }
}

// Translated from 00b48ef0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (no callers found); zlib's `zlibCompileFlags` by its body (no
/// words): the size fields of four 4-byte types (the exe tests each size against 2, 4 and
/// 8 at run time) at bits 0, 2, 4 and 6: 0x55.
pub fn fn_00b48ef0(_e: &mut Engine) -> u32 {
    let mut flags = 0u32;
    for shift in [0u32, 2, 4, 6] {
        let size = 4u32;
        flags = flags.wrapping_add(size_flags(size, shift));
    }
    flags
}

/// The message table (`.data`, one message address per error, from 2 down).
const MESSAGE_TABLE: u32 = 0x010a_63f8;

// Translated from 00b49000 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (no callers found); zlib's `zError` by its body (`cdecl`, one
/// word: an error number): the message address at `010a63f8 + (2 - number) * 4`.
pub fn fn_00b49000(e: &mut Engine, error: i32) -> u32 {
    let index = 2i32.wrapping_sub(error) as u32;
    e.mem.u32(MESSAGE_TABLE.wrapping_add(index.wrapping_mul(4)))
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00b4_9020, fn_00b49020(u32, u32, u32) -> u32),
        entry!(0x00b4_9050, fn_00b49050(u32, u32)),
        entry!(0x00b4_5db0, fn_00b45db0(u32) -> i32),
        entry!(0x00b4_5c00, fn_00b45c00(u32, u32) -> i32),
        entry!(0x00b4_5bd0, fn_00b45bd0(u32)),
        entry!(0x00b4_4000, inflate(u32, i32) -> i32),
        entry!(0x00b4_5e30, fn_00b45e30(u32, u32, u32) -> i32),
        entry!(0x00b4_5f50, fn_00b45f50(u32) -> i32),
        entry!(0x00b4_60e0, fn_00b460e0(u32, u32, u32) -> u32),
        entry!(0x00b4_6170, fn_00b46170(u32) -> i32),
        entry!(0x00b4_61c0, fn_00b461c0(u32, u32) -> i32),
        entry!(0x00b4_6360, fn_00b46360(u32, i32, u32, i32) -> i32),
        entry!(0x00b4_6390, fn_00b46390(u32, i32, i32, i32, i32, i32, u32, i32) -> i32),
        entry!(0x00b4_66b0, fn_00b466b0(u32, u32, u32) -> i32),
        entry!(0x00b4_6880, fn_00b46880(u32) -> i32),
        entry!(0x00b4_6990, fn_00b46990(u32, u32, u32) -> i32),
        entry!(0x00b4_69e0, fn_00b469e0(u32, i32, i32) -> i32),
        entry!(0x00b4_6b00, fn_00b46b00(u32, u32) -> u32),
        entry!(0x00b4_6b70, fn_00b46b70(u32, i32) -> i32),
        entry!(0x00b4_7250, fn_00b47250(u32, u32)),
        entry!(0x00b4_72a0, fn_00b472a0(u32)),
        entry!(0x00b4_7370, fn_00b47370(u32) -> i32),
        entry!(0x00b4_7490, fn_00b47490(u32, u32) -> i32),
        entry!(0x00b4_76f0, fn_00b476f0(u32)),
        entry!(0x00b4_77e0, fn_00b477e0(u32, u32) -> u32),
        entry!(0x00b4_7a20, fn_00b47a20(u32)),
        entry!(0x00b4_7c50, fn_00b47c50(u32, u32, u32) -> u32),
        entry!(0x00b4_7d20, fn_00b47d20(u32, u32) -> u32),
        entry!(0x00b4_82b0, fn_00b482b0(u32, u32) -> u32),
        entry!(0x00b4_85c0, fn_00b485c0(u32, u32) -> u32),
        entry!(0x00b4_87b0, fn_00b487b0(u32, u32) -> u32),
        entry!(0x00b4_8ee0, fn_00b48ee0() -> u32),
        entry!(0x00b4_8ef0, fn_00b48ef0() -> u32),
        entry!(0x00b4_9000, fn_00b49000(i32) -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_allocator_and_free() {
        let mut e = Engine::new();
        e.register(MALLOC, |_, a| (a[0] + 0x1000).into_ret());
        e.register(FREE, |_, _| Ret::default());
        assert_eq!(e.call(0x00b4_9020, &args![0u32, 3u32, 4u32]).u32(), 0x100c);
        assert_eq!(e.call(0x00b4_9020, &args![1u32, 3u32, 4u32]).u32(), 0x100c);
        e.call_log = Some(vec![]);
        e.call(0x00b4_9050, &args![9u32, 0x500u32]);
        assert_eq!(e.call_log.take().unwrap()[1], (FREE, vec![0x500]));
    }

    const ALLOCATE: u32 = 0x013b_0000;
    const RELEASE: u32 = 0x013b_0010;

    fn engine() -> (Engine, u32, u32) {
        let mut e = Engine::new();
        e.register(ALLOCATE, |e, a| e.mem.alloc(a[1] * a[2]).into_ret());
        e.register(RELEASE, |_, _| Ret::default());
        e.register(MEMCPY, |e, a| {
            for i in 0..a[2] {
                let b = e.mem.u8(a[1] + i);
                e.mem.set_u8(a[0] + i, b);
            }
            a[0].into_ret()
        });
        let stream = e.mem.alloc(0x38);
        let state = e.mem.alloc(0x40);
        e.mem.set_u32(stream + 0x1c, state);
        e.mem.set_u32(stream + 0x20, ALLOCATE);
        e.mem.set_u32(stream + 0x24, RELEASE);
        e.mem.set_u32(state + 0x1c, 3);
        (e, stream, state)
    }

    #[test]
    fn fn_00b45db0_frees_what_was_made() {
        let (mut e, stream, state) = engine();
        e.mem.set_u32(state + 0x2c, 0x7000);
        e.mem.set_u32(stream + 0x34, 1);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00b4_5db0, &args![stream]).i32(), 0);
        let frees: Vec<u32> = e
            .call_log
            .take()
            .unwrap()
            .into_iter()
            .filter(|c| c.0 == RELEASE)
            .map(|c| c.1[1])
            .collect();
        assert_eq!(frees, vec![0x7000, state]);
        assert_eq!(e.mem.u32(stream + 0x1c), 0);
        assert_eq!(e.call(0x00b4_5db0, &args![stream]).i32(), -2);
    }

    #[test]
    fn fn_00b45c00_keeps_the_last_output() {
        let (mut e, stream, state) = engine();
        let output = e.mem.alloc(0x20);
        for i in 0..0x20 {
            e.mem.set_u8(output + i, i as u8);
        }
        // 5 bytes of output into an empty 8-byte window.
        e.mem.set_u32(stream + 0xc, output + 5);
        e.mem.set_u32(stream + 0x10, 0x100);
        assert_eq!(e.call(0x00b4_5c00, &args![stream, 0x105u32]).i32(), 0);
        let window = e.mem.u32(state + 0x2c);
        assert_eq!(e.mem.u32(state + 0x20), 8);
        assert_eq!((e.mem.u32(state + 0x28), e.mem.u32(state + 0x24)), (5, 5));
        assert_eq!(e.mem.u8(window + 4), 4);
        // 6 more: 3 fill the window, 3 wrap to its start.
        e.mem.set_u32(stream + 0xc, output + 11);
        assert_eq!(e.call(0x00b4_5c00, &args![stream, 0x106u32]).i32(), 0);
        assert_eq!((e.mem.u32(state + 0x28), e.mem.u32(state + 0x24)), (3, 8));
        assert_eq!(e.mem.u8(window + 7), 7);
        assert_eq!(e.mem.u8(window), 8);
        // A window's worth or more: the last 8 bytes, from the start.
        e.mem.set_u32(stream + 0xc, output + 0x20);
        assert_eq!(e.call(0x00b4_5c00, &args![stream, 0x120u32]).i32(), 0);
        assert_eq!(e.mem.u8(window), 0x18);
        assert_eq!((e.mem.u32(state + 0x28), e.mem.u32(state + 0x24)), (0, 8));
    }

    #[test]
    fn fn_00b45c00_fails_without_a_window() {
        let (mut e, stream, _) = engine();
        e.register(ALLOCATE, |_, _| Ret::default());
        assert_eq!(e.call(0x00b4_5c00, &args![stream, 0u32]).i32(), 1);
    }

    #[test]
    fn fn_00b45bd0_installs_the_fixed_tables() {
        let mut e = Engine::new();
        let state = e.mem.alloc(0x60);
        e.call(0x00b4_5bd0, &args![state]);
        assert_eq!(e.mem.u32(state + 0x44), 0x010a_5a98);
        assert_eq!(e.mem.u32(state + 0x48), 0x010a_6298);
        assert_eq!((e.mem.u32(state + 0x4c), e.mem.u32(state + 0x50)), (9, 5));
    }

    /// The test's model of the header checksum double.
    fn header_sum(running: u32, bytes: &[u8]) -> u32 {
        bytes.iter().fold(running.wrapping_mul(31) ^ 0x55, |s, &b| {
            s.wrapping_mul(3).wrapping_add(u32::from(b))
        })
    }

    /// Bits written low bit first, after whole prefix bytes.
    struct BitWriter {
        bytes: Vec<u8>,
        bit: u32,
    }

    impl BitWriter {
        fn new(prefix: &[u8]) -> Self {
            BitWriter {
                bytes: prefix.to_vec(),
                bit: 0,
            }
        }
        fn put(&mut self, value: u32, count: u32) {
            for i in 0..count {
                if self.bit % 8 == 0 {
                    self.bytes.push(0);
                }
                if (value >> i) & 1 != 0 {
                    *self.bytes.last_mut().unwrap() |= 1 << (self.bit % 8);
                }
                self.bit += 1;
            }
        }
        fn finish(mut self, suffix: &[u8]) -> Vec<u8> {
            self.bytes.extend_from_slice(suffix);
            self.bytes
        }
    }

    /// An engine with a stream over `input`, an output of `size` bytes, a
    /// zeroed state (window bits 15, header kind `78 xx`) and doubles for
    /// the callees outside this file. Answers (engine, stream, state, output).
    fn inflater(input: &[u8], size: u32) -> (Engine, u32, u32, u32) {
        let mut e = Engine::new();
        e.register(MEMCPY, |e, a| {
            for i in 0..a[2] {
                let b = e.mem.u8(a[1] + i);
                e.mem.set_u8(a[0] + i, b);
            }
            a[0].into_ret()
        });
        e.register(DATA_CHECKSUM, |_, a| {
            assert_eq!(a, [0, 0, 0]);
            1u32.into_ret()
        });
        e.register(HEADER_CHECKSUM, |e, a| {
            let bytes: Vec<u8> = (0..a[2]).map(|i| e.mem.u8(a[1] + i)).collect();
            if a[1] == 0 {
                0u32.into_ret()
            } else {
                header_sum(a[0], &bytes).into_ret()
            }
        });
        e.register(ALLOCATE, |e, a| e.mem.alloc(a[1] * a[2]).into_ret());
        e.mem.map(0x010a_5000, 0x2000);
        let stream = e.mem.alloc(0x38);
        let state = e.mem.alloc(0x2528);
        let at = e.mem.alloc(input.len() as u32 + 1);
        e.mem.write(at, input);
        let output = e.mem.alloc(size);
        e.mem.set_u32(stream, at);
        e.mem.set_u32(stream + 4, input.len() as u32);
        e.mem.set_u32(stream + 0xc, output);
        e.mem.set_u32(stream + 0x10, size);
        e.mem.set_u32(stream + 0x1c, state);
        e.mem.set_u32(stream + 0x20, ALLOCATE);
        e.mem.set_u32(stream + 0x24, RELEASE);
        e.mem.set_u32(state + 8, 1);
        e.mem.set_u32(state + 0x1c, 15);
        (e, stream, state, output)
    }

    fn output_bytes(e: &Engine, output: u32, count: u32) -> Vec<u8> {
        (0..count).map(|i| e.mem.u8(output + i)).collect()
    }

    #[test]
    fn inflate_refuses_a_missing_stream_and_stops_without_progress() {
        let (mut e, stream, state, _) = inflater(&[], 16);
        assert_eq!(e.call(0x00b4_4000, &args![0u32, 0i32]).i32(), -2);
        e.mem.set_u32(stream, 0);
        e.mem.set_u32(stream + 4, 1);
        assert_eq!(e.call(0x00b4_4000, &args![stream, 0i32]).i32(), -2);
        e.mem.set_u32(stream + 4, 0);
        // No input: nothing happens, -5; an unknown mode answers -2.
        assert_eq!(e.call(0x00b4_4000, &args![stream, 0i32]).i32(), -5);
        assert_eq!(e.mem.u32(state), 0);
        e.mem.set_u32(state, 0x1d);
        assert_eq!(e.call(0x00b4_4000, &args![stream, 0i32]).i32(), -2);
    }

    #[test]
    fn inflate_copies_a_stored_block() {
        let mut input = vec![0x78, 0x01, 0x01, 5, 0, 0xfa, 0xff];
        input.extend_from_slice(b"hello");
        input.extend_from_slice(&[9, 9, 9, 9]);
        let (mut e, stream, state, output) = inflater(&input, 16);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00b4_4000, &args![stream, 0i32]).i32(), 1);
        assert_eq!(output_bytes(&e, output, 5), b"hello");
        let copies: Vec<Vec<u32>> = e
            .call_log
            .take()
            .unwrap()
            .into_iter()
            .filter(|c| c.0 == MEMCPY)
            .map(|c| c.1)
            .collect();
        assert_eq!(copies.len(), 1);
        assert_eq!(copies[0][2], 5);
        assert_eq!(e.mem.u32(state), 0x1a);
        assert_eq!((e.mem.u32(stream + 4), e.mem.u32(stream + 0x10)), (0, 11));
        assert_eq!(e.mem.u32(stream + 8), 16);
        assert_eq!(e.mem.u32(stream + 0x14), 5);
        assert_eq!(e.mem.u32(state + 0x18), 5);
        // The trailer is read, not compared; the checksum is cleared.
        assert_eq!((e.mem.u32(state + 0x14), e.mem.u32(stream + 0x30)), (0, 0));
        assert_eq!(e.mem.u32(stream + 0x2c), 0x40);
        // Again: the done mode answers 1 at once.
        assert_eq!(e.call(0x00b4_4000, &args![stream, 0i32]).i32(), 1);
    }

    #[test]
    fn inflate_fails_bad_headers_and_lengths() {
        let (mut e, stream, state, _) = inflater(&[0x78, 0x02], 16);
        assert_eq!(e.call(0x00b4_4000, &args![stream, 0i32]).i32(), -3);
        assert_eq!(e.mem.u32(stream + 0x18), MSG_INCORRECT_HEADER_CHECK);
        assert_eq!(e.mem.u32(state), 0x1b);
        // A window too large for the state's 15 bits.
        let (mut e, stream, _, _) = inflater(&[0x88, 0x1c], 16);
        assert_eq!(e.call(0x00b4_4000, &args![stream, 0i32]).i32(), -3);
        assert_eq!(e.mem.u32(stream + 0x18), MSG_INVALID_WINDOW_SIZE);
        // A stored block whose lengths disagree.
        let (mut e, stream, _, _) = inflater(&[0x78, 0x01, 0x01, 5, 0, 0xfa, 0xfe], 16);
        assert_eq!(e.call(0x00b4_4000, &args![stream, 0i32]).i32(), -3);
        assert_eq!(e.mem.u32(stream + 0x18), MSG_INVALID_STORED_LENGTHS);
        // Block type 3.
        let (mut e, stream, _, _) = inflater(&[0x78, 0x01, 0x07], 16);
        assert_eq!(e.call(0x00b4_4000, &args![stream, 0i32]).i32(), -3);
        assert_eq!(e.mem.u32(stream + 0x18), MSG_INVALID_BLOCK_TYPE);
    }

    #[test]
    fn inflate_answers_2_for_a_dictionary() {
        let (mut e, stream, state, _) = inflater(&[0x78, 0xbb, 0x12, 0x34, 0x56, 0x78], 16);
        assert_eq!(e.call(0x00b4_4000, &args![stream, 0i32]).i32(), 2);
        assert_eq!(e.mem.u32(state), 0x0a);
        assert_eq!(e.mem.u32(stream + 0x30), 0x1234_5678);
        assert_eq!(e.mem.u32(stream + 4), 0);
        // With a dictionary set the data checksum starts and the block
        // header waits for input; flush kind 5 stops at the block start.
        e.mem.set_u32(state + 0xc, 1);
        assert_eq!(e.call(0x00b4_4000, &args![stream, 5i32]).i32(), -5);
        assert_eq!(e.mem.u32(state), 0x0b);
        assert_eq!(e.mem.u32(stream + 0x30), 1);
        assert_eq!(e.mem.u32(stream + 0x2c), 0x80);
    }

    #[test]
    fn inflate_reads_a_header_with_flags() {
        // 1f 8b, method 8, flags: header checksum, extra field, name.
        let mut input = vec![0x1f, 0x8b, 0x08, 0x0e, 1, 2, 3, 4, 0, 3, 2, 0, b'a', b'b'];
        input.extend_from_slice(b"nm\0");
        let mut sum = header_sum(0, &[0x1f, 0x8b]);
        sum = header_sum(sum, &[0x08, 0x0e]);
        sum = header_sum(sum, &[1, 2, 3, 4]);
        sum = header_sum(sum, &[0, 3]);
        sum = header_sum(sum, &[2, 0]);
        sum = header_sum(sum, b"ab");
        input.extend_from_slice(&[sum as u8, (sum >> 8) as u8]);
        let (mut e, stream, state, _) = inflater(&input, 16);
        e.mem.set_u32(state + 8, 2);
        assert_eq!(e.call(0x00b4_4000, &args![stream, 0i32]).i32(), 0);
        assert_eq!(e.mem.u32(state + 0x10), 0x0e08);
        // The header checksum matched; the next checksum starts at 0.
        assert_eq!(e.mem.u32(state), 0x0b);
        assert_eq!(e.mem.u32(stream + 0x30), 0);
        assert_eq!(e.mem.u32(stream + 8), input.len() as u32);
        // A wrong header checksum fails.
        let last = input.len() - 1;
        input[last] ^= 1;
        let (mut e, stream, state, _) = inflater(&input, 16);
        e.mem.set_u32(state + 8, 2);
        assert_eq!(e.call(0x00b4_4000, &args![stream, 0i32]).i32(), -3);
        assert_eq!(e.mem.u32(stream + 0x18), MSG_HEADER_CRC_MISMATCH);
        // Unknown flags.
        let (mut e, stream, state, _) = inflater(&[0x1f, 0x8b, 0x08, 0x20], 16);
        e.mem.set_u32(state + 8, 2);
        assert_eq!(e.call(0x00b4_4000, &args![stream, 0i32]).i32(), -3);
        assert_eq!(e.mem.u32(stream + 0x18), MSG_UNKNOWN_HEADER_FLAGS);
    }

    /// Synthetic fixed tables: a length code for every low byte of the
    /// index (literals below 0xf0; 0xf0 a length 3 with 2 extra bits; 0xf1
    /// a link to a one-bit sub-table at 0xf2 holding `A`/`B`; 0xfe invalid;
    /// 0xff the block end), and distance codes `d + 1` with one extra bit.
    fn fixed_tables(e: &mut Engine) {
        for i in 0..0x200u32 {
            let low = i & 0xff;
            let code = match low {
                0xf0 => 0x0003_0812,
                0xf1 => 0x00f2_0801,
                0xfe => 0x0000_0840,
                0xff => 0x0000_0820,
                _ => (low << 16) | 0x0800,
            };
            e.mem.set_u32(0x010a_5a98 + i * 4, code);
        }
        e.mem.set_u32(0x010a_5a98 + 0xf2 * 4, 0x0041_0100);
        e.mem.set_u32(0x010a_5a98 + 0xf3 * 4, 0x0042_0100);
        for d in 0..0x20u32 {
            e.mem.set_u32(0x010a_6298 + d * 4, ((d + 1) << 16) | 0x0511);
        }
    }

    /// `78 01`, a last fixed block: `h`, `i`, length 4 at distance 2, the
    /// sub-table's `B`, block end; then a trailer.
    fn fixed_stream() -> Vec<u8> {
        let mut w = BitWriter::new(&[0x78, 0x01]);
        w.put(0b011, 3);
        w.put(u32::from(b'h'), 8);
        w.put(u32::from(b'i'), 8);
        w.put(0xf0, 8);
        w.put(1, 2);
        w.put(1, 5);
        w.put(0, 1);
        w.put(0xf1, 8);
        w.put(1, 1);
        w.put(0xff, 8);
        w.finish(&[1, 2, 3, 4])
    }

    #[test]
    fn inflate_decodes_a_fixed_block() {
        let input = fixed_stream();
        let (mut e, stream, state, output) = inflater(&input, 64);
        fixed_tables(&mut e);
        assert_eq!(e.call(0x00b4_4000, &args![stream, 0i32]).i32(), 1);
        assert_eq!(output_bytes(&e, output, 7), b"hihihiB");
        assert_eq!(e.mem.u32(state), 0x1a);
        assert_eq!(e.mem.u32(stream + 0x14), 7);
        assert_eq!(e.mem.u32(stream + 0x10), 57);
        // An invalid length code fails.
        let mut w = BitWriter::new(&[0x78, 0x01]);
        w.put(0b011, 3);
        w.put(0xfe, 8);
        let (mut e, stream, _, _) = inflater(&w.finish(&[]), 64);
        fixed_tables(&mut e);
        assert_eq!(e.call(0x00b4_4000, &args![stream, 0i32]).i32(), -3);
        assert_eq!(e.mem.u32(stream + 0x18), MSG_INVALID_LENGTH_CODE);
        // Too far back: distance 2 after one byte.
        let mut w = BitWriter::new(&[0x78, 0x01]);
        w.put(0b011, 3);
        w.put(u32::from(b'h'), 8);
        w.put(0xf0, 8);
        w.put(0, 2);
        w.put(1, 5);
        w.put(0, 1);
        let (mut e, stream, _, _) = inflater(&w.finish(&[]), 64);
        fixed_tables(&mut e);
        assert_eq!(e.call(0x00b4_4000, &args![stream, 0i32]).i32(), -3);
        assert_eq!(e.mem.u32(stream + 0x18), MSG_DISTANCE_TOO_FAR);
    }

    #[test]
    fn inflate_keeps_the_window_or_fails_without_one() {
        let input = fixed_stream();
        // Stopped after `h` and `i`: the output goes to a new window.
        let (mut e, stream, state, _) = inflater(&input[..5], 64);
        fixed_tables(&mut e);
        assert_eq!(e.call(0x00b4_4000, &args![stream, 0i32]).i32(), 0);
        let window = e.mem.u32(state + 0x2c);
        assert_ne!(window, 0);
        assert_eq!(e.mem.u32(state + 0x20), 0x8000);
        assert_eq!(output_bytes(&e, window, 2), b"hi");
        // Without memory for it: the no-memory mode, -4, and -4 again.
        let (mut e, stream, state, _) = inflater(&input[..5], 64);
        fixed_tables(&mut e);
        e.register(ALLOCATE, |_, _| Ret::default());
        assert_eq!(e.call(0x00b4_4000, &args![stream, 0i32]).i32(), -4);
        assert_eq!(e.mem.u32(state), 0x1c);
        assert_eq!(e.call(0x00b4_4000, &args![stream, 0i32]).i32(), -4);
    }

    #[test]
    fn inflate_hands_long_runs_to_the_fast_decoder() {
        let input = fixed_stream();
        let (mut e, stream, state, output) = inflater(&input, 0x200);
        fixed_tables(&mut e);
        let seen = std::rc::Rc::new(std::cell::RefCell::new(Vec::new()));
        let log = seen.clone();
        e.register_double(FAST_DECODER, move |e, a| {
            let state = e.mem.u32(a[0] + 0x1c);
            log.borrow_mut().push((
                a.to_vec(),
                e.mem.u32(a[0] + 4),
                e.mem.u32(state + 0x34),
                e.mem.u32(a[0] + 0xc),
            ));
            // Writes one byte and ends the stream.
            let put = e.mem.u32(a[0] + 0xc);
            e.mem.set_u8(put, b'z');
            e.mem.set_u32(a[0] + 0xc, put + 1);
            let left = e.mem.u32(a[0] + 0x10);
            e.mem.set_u32(a[0] + 0x10, left - 1);
            e.mem.set_u32(state, 0x1a);
            Ret::default()
        });
        assert_eq!(e.call(0x00b4_4000, &args![stream, 0i32]).i32(), 1);
        let seen = seen.borrow();
        assert_eq!(seen.len(), 1);
        // Called with the output left at the start, after the copies were
        // written back (block header taken: 3 of the byte's 8 bits used).
        assert_eq!(seen[0].0, vec![stream, 0x200]);
        assert_eq!(seen[0].1, input.len() as u32 - 3);
        assert_eq!(seen[0].2, 5);
        assert_eq!(seen[0].3, output);
        assert_eq!(e.mem.u8(output), b'z');
        assert_eq!(e.mem.u32(stream + 0x10), 0x1ff);
        assert_eq!(e.mem.u32(state), 0x1a);
    }

    /// `78 01`, a last coded block: 257 + 1 symbols, 4 code-length code
    /// lengths (1..4), then the code-length symbols `symbols` (two bits each
    /// in the test's code-length table, with their extra bits).
    fn coded_engine(symbols: &[(u32, u32, u32)]) -> (Engine, u32, u32, TableCalls) {
        let mut w = BitWriter::new(&[0x78, 0x01]);
        w.put(0b101, 3);
        w.put(0, 5);
        w.put(0, 5);
        w.put(0, 4);
        for length in 1..=4 {
            w.put(length, 3);
        }
        for &(symbol, extra, extra_bits) in symbols {
            w.put(symbol, 2);
            w.put(extra, extra_bits);
        }
        w.put(0, 1);
        let (mut e, stream, state, _) = inflater(&w.finish(&[]), 64);
        for i in 0..0x13u32 {
            e.mem.set_u16(0x010a_6318 + i * 2, i as u16);
        }
        let calls: TableCalls = Default::default();
        let log = calls.clone();
        e.register_double(TABLE_BUILDER, move |e, a| {
            log.borrow_mut().push(a.to_vec());
            let next = e.mem.u32(a[3]);
            match a[0] {
                // Code lengths: index & 3 -> length 8, repeat, zeros, zeros.
                0 => {
                    for i in 0..0x80u32 {
                        let value = [8u32, 16, 17, 18][(i & 3) as usize];
                        e.mem.set_u32(next + i * 4, (value << 16) | 0x0200);
                    }
                }
                // Lengths: every code ends the block.
                1 => {
                    for i in 0..0x200u32 {
                        e.mem.set_u32(next + i * 4, 0x0000_0120);
                    }
                    e.mem.set_u32(a[3], next + 0x800);
                }
                _ => {}
            }
            Ret::default()
        });
        (e, stream, state, calls)
    }

    type TableCalls = std::rc::Rc<std::cell::RefCell<Vec<Vec<u32>>>>;

    #[test]
    fn inflate_builds_the_tables_of_a_coded_block() {
        // 8, repeat 6, 138 zeros, 113 zeros: 258 lengths.
        let (mut e, stream, state, calls) =
            coded_engine(&[(0, 0, 0), (1, 3, 2), (3, 127, 7), (3, 102, 7)]);
        assert_eq!(e.call(0x00b4_4000, &args![stream, 0i32]).i32(), 0);
        let lengths = |e: &Engine, i: u32| e.mem.u16(state + 0x68 + i * 2);
        // The code-length code lengths, in the test's order.
        let calls = calls.borrow();
        assert_eq!(
            calls[0],
            vec![
                0,
                state + 0x68,
                0x13,
                state + 0x64,
                state + 0x4c,
                state + 0x2e8
            ]
        );
        assert_eq!(
            calls[1],
            vec![
                1,
                state + 0x68,
                0x101,
                state + 0x64,
                state + 0x4c,
                state + 0x2e8
            ]
        );
        assert_eq!(
            calls[2],
            vec![
                2,
                state + 0x68 + 0x202,
                1,
                state + 0x64,
                state + 0x50,
                state + 0x2e8
            ]
        );
        for i in 0..7 {
            assert_eq!(lengths(&e, i), 8);
        }
        assert_eq!(lengths(&e, 7), 0);
        assert_eq!(e.mem.u32(state + 0x60), 0x102);
        assert_eq!(e.mem.u32(state + 0x44), state + 0x528);
        assert_eq!(e.mem.u32(state + 0x48), state + 0x528 + 0x800);
        assert_eq!((e.mem.u32(state + 0x4c), e.mem.u32(state + 0x50)), (9, 6));
        // The block ended; the trailer is still to come.
        assert_eq!(e.mem.u32(state), 0x18);
    }

    #[test]
    fn inflate_goes_on_after_a_repeat_without_a_length() {
        // A repeat first: the message is left, yet the tables are built and
        // decoding goes on, as in the exe.
        let (mut e, stream, state, calls) = coded_engine(&[(1, 0, 2)]);
        assert_eq!(e.call(0x00b4_4000, &args![stream, 0i32]).i32(), 0);
        assert_eq!(e.mem.u32(stream + 0x18), MSG_REPEAT_WITHOUT_LENGTH);
        assert_eq!(calls.borrow().len(), 3);
        assert_eq!(e.mem.u32(state), 0x18);
        // A repeat past the 258 lengths.
        let (mut e, stream, _, _) = coded_engine(&[(0, 0, 0), (3, 127, 7), (3, 127, 7)]);
        assert_eq!(e.call(0x00b4_4000, &args![stream, 0i32]).i32(), 0);
        assert_eq!(e.mem.u32(stream + 0x18), MSG_REPEAT_TOO_LONG);
    }
}

#[cfg(test)]
mod n080_tests {
    use super::*;

    const ALLOCATE: u32 = 0x013c_0000;
    const RELEASE: u32 = 0x013c_0010;

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

    /// An engine with an allocating allocator, a recording free, copying `memcpy` and
    /// filling `memset` doubles, and a stream record using them.
    fn engine() -> (Engine, u32) {
        let mut e = Engine::new();
        e.register(ALLOCATE, |e, a| answer(e.mem.alloc(a[1] * a[2])));
        e.register(RELEASE, |_, _| Ret::default());
        e.register(MEMCPY, |e, a| {
            for i in 0..a[2] {
                let b = e.mem.u8(a[1] + i);
                e.mem.set_u8(a[0] + i, b);
            }
            answer(a[0])
        });
        e.register(MEMSET, |e, a| {
            for i in 0..a[2] {
                e.mem.set_u8(a[0] + i, a[1] as u8);
            }
            answer(a[0])
        });
        let stream = e.mem.alloc(0x38);
        e.mem.set_u32(stream + STREAM_ALLOCATE, ALLOCATE);
        e.mem.set_u32(stream + STREAM_FREE, RELEASE);
        (e, stream)
    }

    fn bytes(e: &mut Engine, data: &[u8]) -> u32 {
        let at = e.mem.alloc(data.len() as u32 + 0x10);
        for (i, b) in data.iter().enumerate() {
            e.mem.set_u8(at + i as u32, *b);
        }
        at
    }

    #[test]
    fn set_dictionary_fills_the_window() {
        let (mut e, stream) = engine();
        e.register(DATA_CHECKSUM, |_, a| {
            answer(if a[1] == 0 { 1 } else { 0x77 })
        });
        let state = e.mem.alloc(0x60);
        e.mem.set_u32(stream + STREAM_STATE, state);
        assert_eq!(e.call(0x00b4_5e30, &args![stream, 0u32, 0u32]).i32(), -2);
        e.mem.set_u32(state + STATE_MODE, MODE_DICTIONARY);
        e.mem.set_u32(state + STATE_CHECKSUM, 0x76);
        let dictionary = bytes(&mut e, b"0123456789");
        assert_eq!(
            e.call(0x00b4_5e30, &args![stream, dictionary, 10u32]).i32(),
            -3
        );
        e.mem.set_u32(state + STATE_CHECKSUM, 0x77);
        let window = e.mem.alloc(8);
        e.mem.set_u32(state + STATE_WINDOW, window);
        e.mem.set_u32(state + STATE_WINDOW_SIZE, 8);
        e.mem.set_u32(stream + STREAM_OUTPUT, window);
        e.mem.set_u32(stream + STREAM_OUTPUT_LEFT, 0);
        assert_eq!(
            e.call(0x00b4_5e30, &args![stream, dictionary, 10u32]).i32(),
            0
        );
        assert_eq!(e.mem.bytes(window, 8), b"23456789");
        assert_eq!(e.mem.u32(state + STATE_WINDOW_FILL), 8);
        assert_eq!(e.mem.u32(state + STATE_HAVE_DICTIONARY), 1);
        e.call(0x00b4_5e30, &args![stream, dictionary, 3u32]);
        assert_eq!(e.mem.bytes(window + 5, 3), b"012");
        assert_eq!(e.mem.u32(state + STATE_WINDOW_FILL), 3);
    }

    #[test]
    fn sync_search_counts_the_marker() {
        let (mut e, _) = engine();
        let found = e.mem.alloc(4);
        let data = bytes(&mut e, &[7, 0, 0, 0xff, 0xff, 9]);
        assert_eq!(e.call(0x00b4_60e0, &args![found, data, 6u32]).u32(), 5);
        assert_eq!(e.mem.u32(found), 4);
        e.mem.set_u32(found, 1);
        let data = bytes(&mut e, &[5, 0]);
        e.call(0x00b4_60e0, &args![found, data, 2u32]);
        assert_eq!(e.mem.u32(found), 1);
        e.mem.set_u32(found, 3);
        let data = bytes(&mut e, &[0]);
        e.call(0x00b4_60e0, &args![found, data, 1u32]);
        assert_eq!(e.mem.u32(found), 1);
    }

    #[test]
    fn sync_finds_the_marker_across_held_bits() {
        let (mut e, stream) = engine();
        e.register(INFLATE_RESET, |_, _| Ret::default());
        let state = e.mem.alloc(0x70);
        e.mem.set_u32(stream + STREAM_STATE, state);
        assert_eq!(e.call(0x00b4_5f50, &args![stream]).i32(), -5);
        // 19 bits held: three dropped to the byte boundary, two zero bytes kept.
        e.mem.set_u32(state + STATE_BITS, 19);
        e.mem.set_u32(state + STATE_HOLD, 0);
        let input = bytes(&mut e, &[0xff, 0xff, 1]);
        e.mem.set_u32(stream + STREAM_INPUT, input);
        e.mem.set_u32(stream + STREAM_INPUT_LEFT, 3);
        e.mem.set_u32(stream + STREAM_TOTAL_IN, 10);
        e.mem.set_u32(stream + STREAM_TOTAL_OUT, 20);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00b4_5f50, &args![stream]).i32(), 0);
        assert_eq!(calls_to(&e, INFLATE_RESET), vec![vec![stream]]);
        assert_eq!(e.mem.u32(stream + STREAM_INPUT_LEFT), 1);
        assert_eq!(e.mem.u32(stream + STREAM_TOTAL_IN), 12);
        assert_eq!(e.mem.u32(stream + STREAM_TOTAL_OUT), 20);
        assert_eq!(e.mem.u32(state + STATE_MODE), MODE_BLOCK_START);
        assert_eq!(e.mem.u32(state + STATE_BITS), 0);
        // In sync mode without the marker: -3.
        e.mem.set_u32(state + STATE_MODE, MODE_SYNC);
        e.mem.set_u32(state + STATE_HAVE, 0);
        e.mem.set_u32(stream + STREAM_INPUT_LEFT, 1);
        assert_eq!(e.call(0x00b4_5f50, &args![stream]).i32(), -3);
    }

    #[test]
    fn sync_point_is_the_stored_lengths_mode_without_bits() {
        let (mut e, stream) = engine();
        assert_eq!(e.call(0x00b4_6170, &args![stream]).i32(), -2);
        let state = e.mem.alloc(0x40);
        e.mem.set_u32(stream + STREAM_STATE, state);
        e.mem.set_u32(state + STATE_MODE, MODE_STORED_LENGTHS);
        assert_eq!(e.call(0x00b4_6170, &args![stream]).i32(), 1);
        e.mem.set_u32(state + STATE_BITS, 1);
        assert_eq!(e.call(0x00b4_6170, &args![stream]).i32(), 0);
    }

    #[test]
    fn inflate_copy_duplicates_state_and_window() {
        let (mut e, stream) = engine();
        let destination = e.mem.alloc(0x38);
        assert_eq!(e.call(0x00b4_61c0, &args![destination, stream]).i32(), -2);
        let state = e.mem.alloc(INFLATE_STATE_SIZE);
        e.mem.set_u32(stream + STREAM_STATE, state);
        e.mem.set_u32(stream + STREAM_TOTAL_IN, 0x55);
        let window = bytes(&mut e, b"abcd");
        e.mem.set_u32(state + STATE_WINDOW, window);
        e.mem.set_u32(state + STATE_WINDOW_BITS, 2);
        e.mem
            .set_u32(state + STATE_LENGTH_CODES, state + STATE_CODES + 0x10);
        e.mem
            .set_u32(state + STATE_DISTANCE_CODES, state + STATE_CODES + 0x20);
        e.mem
            .set_u32(state + STATE_NEXT_CODE, state + STATE_CODES + 0x30);
        assert_eq!(e.call(0x00b4_61c0, &args![destination, stream]).i32(), 0);
        let copy = e.mem.u32(destination + STREAM_STATE);
        assert_ne!(copy, state);
        assert_eq!(e.mem.u32(destination + STREAM_TOTAL_IN), 0x55);
        assert_eq!(
            e.mem.u32(copy + STATE_LENGTH_CODES),
            copy + STATE_CODES + 0x10
        );
        assert_eq!(e.mem.u32(copy + STATE_NEXT_CODE), copy + STATE_CODES + 0x30);
        let new_window = e.mem.u32(copy + STATE_WINDOW);
        assert_ne!(new_window, window);
        assert_eq!(e.mem.bytes(new_window, 4), b"abcd");
        // A failed window allocation frees the state copy.
        e.register(ALLOCATE, |e, a| {
            answer(if a[2] == 1 {
                0
            } else {
                e.mem.alloc(a[1] * a[2])
            })
        });
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00b4_61c0, &args![destination, stream]).i32(), -4);
        assert_eq!(calls_to(&e, RELEASE).len(), 1);
    }

    /// A stream with a compressor state set up by `deflateInit2_` with its callees
    /// doubled.
    fn deflate_engine(window_bits: i32) -> (Engine, u32, u32) {
        let (mut e, stream) = engine();
        e.map(0x010a_6000, 0x3000);
        e.mem.set_u8(VERSION_STRING, b'1');
        e.register(DATA_CHECKSUM, |_, _| answer(1));
        e.register(HEADER_CHECKSUM_00B4A070, |_, _| answer(0));
        e.register(TREES_INIT, |_, _| Ret::default());
        let version = bytes(&mut e, b"1.2.2");
        let result = e
            .call(
                0x00b4_6390,
                &args![
                    stream,
                    6i32,
                    8i32,
                    window_bits,
                    8i32,
                    0i32,
                    version,
                    0x38i32
                ],
            )
            .i32();
        assert_eq!(result, 0);
        let state = e.mem.u32(stream + STREAM_STATE);
        (e, stream, state)
    }

    #[test]
    fn deflate_init_lays_out_the_state() {
        let (mut e, stream, state) = deflate_engine(15);
        assert_eq!(e.mem.u32(state + DS_STREAM), stream);
        assert_eq!(e.mem.u32(state + DS_WRAP), 1);
        assert_eq!(e.mem.u32(state + DS_WINDOW_SIZE), 0x8000);
        assert_eq!(e.mem.u32(state + DS_HASH_SIZE), 0x8000);
        assert_eq!(e.mem.u32(state + DS_HASH_SHIFT), 5);
        assert_eq!(e.mem.u32(state + DS_LITERAL_BUFFER_SIZE), 0x4000);
        let pending = e.mem.u32(state + DS_PENDING_BUFFER);
        assert_eq!(e.mem.u32(state + DS_DISTANCE_BUFFER), pending + 0x4000);
        assert_eq!(e.mem.u32(state + DS_LITERAL_BUFFER), pending + 0xc000);
        assert_eq!(e.mem.u32(state + DS_STATUS), STATUS_INIT);
        assert_eq!(e.mem.u32(state + DS_PREVIOUS_LENGTH), 2);
        assert_eq!(e.mem.u8(state + DS_METHOD), 8);
        assert_eq!(e.mem.u32(stream + STREAM_DATA_TYPE), 2);
        // deflateInit_ fills in the defaults; bad arguments.
        let version = bytes(&mut e, b"1");
        assert_eq!(
            e.call(0x00b4_6360, &args![stream, -1i32, version, 0x38i32])
                .i32(),
            0
        );
        let state = e.mem.u32(stream + STREAM_STATE);
        assert_eq!(e.mem.u32(state + DS_LEVEL), 6);
        assert_eq!(
            e.call(0x00b4_6360, &args![stream, 6i32, version, 0x30i32])
                .i32(),
            -6
        );
        assert_eq!(
            e.call(
                0x00b4_6390,
                &args![stream, 10i32, 8i32, 15i32, 8i32, 0i32, version, 0x38i32]
            )
            .i32(),
            -2
        );
        // A failed buffer allocation: the finished status, the memory message, -4.
        e.mem.set_u32(MESSAGE_MEMORY_ERROR, 0x4444);
        e.register(ALLOCATE, |e, a| {
            answer(if a[2] == 4 {
                0
            } else {
                e.mem.alloc(a[1] * a[2])
            })
        });
        assert_eq!(
            e.call(
                0x00b4_6390,
                &args![stream, 6i32, 8i32, -9i32, 1i32, 0i32, version, 0x38i32]
            )
            .i32(),
            -4
        );
        assert_eq!(e.mem.u32(stream + STREAM_MESSAGE), 0x4444);
        assert_eq!(e.mem.u32(stream + STREAM_STATE), 0);
    }

    #[test]
    fn deflate_reset_restarts_the_gzip_checksum() {
        let (mut e, stream, state) = deflate_engine(31);
        assert_eq!(e.mem.u32(state + DS_WRAP), 2);
        e.mem.set_u32(state + DS_WRAP, (-2i32) as u32);
        e.register(HEADER_CHECKSUM_00B4A070, |_, _| answer(0x99));
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00b4_6880, &args![stream]).i32(), 0);
        assert_eq!(e.mem.u32(state + DS_WRAP), 2);
        assert_eq!(e.mem.u32(stream + STREAM_ADLER), 0x99);
        assert_eq!(calls_to(&e, TREES_INIT), vec![vec![state]]);
        e.mem.set_u32(stream + STREAM_FREE, 0);
        assert_eq!(e.call(0x00b4_6880, &args![stream]).i32(), -2);
    }

    #[test]
    fn deflate_set_dictionary_hashes_the_strings() {
        let (mut e, stream, state) = deflate_engine(15);
        e.register(DATA_CHECKSUM, |_, a| answer(a[0] + a[2]));
        let dictionary = bytes(&mut e, b"abcd");
        assert_eq!(
            e.call(0x00b4_66b0, &args![stream, dictionary, 4u32]).i32(),
            0
        );
        assert_eq!(e.mem.u32(stream + STREAM_ADLER), 5);
        let window = e.mem.u32(state + DS_WINDOW);
        assert_eq!(e.mem.bytes(window, 4), b"abcd");
        assert_eq!(e.mem.u32(state + DS_STRING_START), 4);
        let hash = |a: u8, b: u8, c: u8| {
            (((u32::from(a) << 5 ^ u32::from(b)) << 5) ^ u32::from(c)) & 0x7fff
        };
        let head = e.mem.u32(state + DS_HEAD);
        assert_eq!(e.mem.u16(head + 2 * hash(b'b', b'c', b'd')), 1);
        assert_eq!(e.mem.u32(state + DS_INSERT_HASH), hash(b'b', b'c', b'd'));
        assert_eq!(
            e.call(0x00b4_66b0, &args![stream, dictionary, 2u32]).i32(),
            0
        );
        assert_eq!(e.call(0x00b4_66b0, &args![stream, 0u32, 2u32]).i32(), -2);
        e.mem.set_u32(state + DS_STATUS, STATUS_BUSY);
        assert_eq!(
            e.call(0x00b4_66b0, &args![stream, dictionary, 2u32]).i32(),
            -2
        );
    }

    #[test]
    fn prime_sets_the_bit_buffer() {
        let (mut e, stream, state) = deflate_engine(15);
        assert_eq!(e.call(0x00b4_6990, &args![stream, 3u32, 0xffu32]).i32(), 0);
        assert_eq!(e.mem.u32(state + DS_BIT_COUNT), 3);
        assert_eq!(e.mem.u16(state + DS_BIT_BUFFER), 7);
    }

    #[test]
    fn params_flush_and_take_the_level_row() {
        let (mut e, stream, state) = deflate_engine(15);
        for level in 0..10u32 {
            let row = level_row(level);
            for (i, value) in [4u16, 5, 6, 7].iter().enumerate() {
                e.mem.set_u16(row + 2 * i as u32, value + level as u16);
            }
            e.mem
                .set_u32(row + 8, if level < 4 { 0x1000 } else { 0x2000 });
        }
        e.mem.set_u32(stream + STREAM_TOTAL_IN, 1);
        e.mem.set_u32(stream + STREAM_OUTPUT, 0x100);
        e.mem.set_u32(stream + STREAM_OUTPUT_LEFT, 0);
        e.mem.set_u32(MESSAGE_BUFFER_ERROR, 0x5555);
        // The flush through deflate answers the buffer error here (no room).
        assert_eq!(e.call(0x00b4_69e0, &args![stream, 1i32, 2i32]).i32(), -5);
        assert_eq!(e.mem.u32(state + DS_LEVEL), 1);
        assert_eq!(e.mem.u32(state + DS_GOOD_MATCH), 5);
        assert_eq!(e.mem.u32(state + DS_MAX_LAZY), 6);
        assert_eq!(e.mem.u32(state + DS_NICE_MATCH), 7);
        assert_eq!(e.mem.u32(state + DS_MAX_CHAIN), 8);
        assert_eq!(e.mem.u32(state + DS_STRATEGY), 2);
        assert_eq!(e.call(0x00b4_69e0, &args![stream, 2i32, 0i32]).i32(), 0);
        assert_eq!(e.call(0x00b4_69e0, &args![stream, 2i32, 4i32]).i32(), -2);
    }

    #[test]
    fn bound_uses_the_helper_for_default_sizes() {
        let (mut e, stream, state) = deflate_engine(15);
        e.register(BOUND_00B4DEA0, |_, a| answer(a[0] * 2));
        assert_eq!(e.call(0x00b4_6b00, &args![stream, 100u32]).u32(), 200);
        e.mem.set_u32(state + DS_HASH_BITS, 14);
        assert_eq!(
            e.call(0x00b4_6b00, &args![stream, 100u32]).u32(),
            100 + 13 + 2 + 11
        );
        assert_eq!(
            e.call(0x00b4_6b00, &args![0u32, 64u32]).u32(),
            64 + 8 + 1 + 11
        );
    }

    #[test]
    fn short_and_pending_writes() {
        let (mut e, stream, state) = deflate_engine(15);
        e.call(0x00b4_7250, &args![state, 0x1234u32]);
        let buffer = e.mem.u32(state + DS_PENDING_BUFFER);
        assert_eq!(e.mem.bytes(buffer, 2), [0x12, 0x34]);
        assert_eq!(e.mem.u32(state + DS_PENDING), 2);
        let output = e.mem.alloc(4);
        e.mem.set_u32(stream + STREAM_OUTPUT, output);
        e.mem.set_u32(stream + STREAM_OUTPUT_LEFT, 1);
        e.call(0x00b4_72a0, &args![stream]);
        assert_eq!(e.mem.u8(output), 0x12);
        assert_eq!(e.mem.u32(state + DS_PENDING), 1);
        assert_eq!(e.mem.u32(state + DS_PENDING_OUT), buffer + 1);
        e.mem.set_u32(stream + STREAM_OUTPUT_LEFT, 3);
        e.call(0x00b4_72a0, &args![stream]);
        assert_eq!(e.mem.u8(output + 1), 0x34);
        assert_eq!(e.mem.u32(state + DS_PENDING_OUT), buffer);
        assert_eq!(e.mem.u32(stream + STREAM_TOTAL_OUT), 2);
        assert_eq!(e.mem.u32(stream + STREAM_OUTPUT_LEFT), 2);
    }

    #[test]
    fn deflate_end_frees_everything() {
        let (mut e, stream, state) = deflate_engine(15);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00b4_7370, &args![stream]).i32(), 0);
        let freed: Vec<u32> = calls_to(&e, RELEASE).iter().map(|w| w[1]).collect();
        assert_eq!(freed.len(), 5);
        assert_eq!(freed[4], state);
        assert_eq!(e.mem.u32(stream + STREAM_STATE), 0);
        let (mut e, stream, state) = deflate_engine(15);
        e.mem.set_u32(state + DS_STATUS, STATUS_BUSY);
        assert_eq!(e.call(0x00b4_7370, &args![stream]).i32(), -3);
        let (mut e, stream, state) = deflate_engine(15);
        e.mem.set_u32(state + DS_STATUS, 5);
        assert_eq!(e.call(0x00b4_7370, &args![stream]).i32(), -2);
    }

    #[test]
    fn deflate_copy_rebases_the_buffers() {
        let (mut e, stream, state) = deflate_engine(15);
        let pending = e.mem.u32(state + DS_PENDING_BUFFER);
        e.mem.set_u32(state + DS_PENDING_OUT, pending + 3);
        let window = e.mem.u32(state + DS_WINDOW);
        e.mem.set_u8(window + 7, 0x42);
        let destination = e.mem.alloc(0x38);
        assert_eq!(e.call(0x00b4_7490, &args![destination, stream]).i32(), 0);
        let copy = e.mem.u32(destination + STREAM_STATE);
        assert_ne!(copy, state);
        assert_eq!(e.mem.u32(copy + DS_STREAM), destination);
        let new_pending = e.mem.u32(copy + DS_PENDING_BUFFER);
        assert_eq!(e.mem.u32(copy + DS_PENDING_OUT), new_pending + 3);
        assert_eq!(e.mem.u32(copy + DS_LITERAL_BUFFER), new_pending + 0xc000);
        assert_eq!(e.mem.u32(copy + DS_LITERAL_DESCRIPTOR), copy + 0x8c);
        assert_eq!(e.mem.u8(e.mem.u32(copy + DS_WINDOW) + 7), 0x42);
        assert_eq!(e.call(0x00b4_7490, &args![0u32, stream]).i32(), -2);
    }

    #[test]
    fn deflate_writes_header_and_trailer() {
        let (mut e, stream, state) = deflate_engine(15);
        // Level 6's compress function: a double that finishes the stream.
        e.register(0x013c_0100, |_, _| answer(3));
        e.mem.set_u32(level_row(6) + 8, 0x013c_0100);
        e.register(DATA_CHECKSUM, |_, _| answer(1));
        let output = e.mem.alloc(0x20);
        e.mem.set_u32(stream + STREAM_OUTPUT, output);
        e.mem.set_u32(stream + STREAM_OUTPUT_LEFT, 0x20);
        e.mem.set_u32(stream + STREAM_ADLER, 0x0102_0304);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00b4_6b70, &args![stream, 4i32]).i32(), 1);
        assert_eq!(calls_to(&e, 0x013c_0100), vec![vec![state, 4]]);
        // 78 9c, then the checksum (restarted to 1 after the header): 00 00 00 01.
        assert_eq!(e.mem.bytes(output, 6), [0x78, 0x9c, 0, 0, 0, 1]);
        assert_eq!(e.mem.u32(state + DS_WRAP), (-1i32) as u32);
        assert_eq!(e.mem.u32(state + DS_STATUS), STATUS_FINISH);
        // A finished stream without input: no header now, answers 1 again.
        assert_eq!(e.call(0x00b4_6b70, &args![stream, 4i32]).i32(), 1);
        assert_eq!(e.call(0x00b4_6b70, &args![stream, 5i32]).i32(), -2);
        e.mem.set_u32(MESSAGE_STREAM_ERROR, 0x5151);
        assert_eq!(e.call(0x00b4_6b70, &args![stream, 0i32]).i32(), -2);
        assert_eq!(e.mem.u32(stream + STREAM_MESSAGE), 0x5151);
    }

    #[test]
    fn deflate_gzip_header_and_block_done_flushes() {
        let (mut e, stream, state) = deflate_engine(31);
        e.register(0x013c_0200, |_, _| answer(1));
        e.mem.set_u32(level_row(6) + 8, 0x013c_0200);
        e.register(TREES_STORED_BLOCK, |_, _| Ret::default());
        e.register(TREES_ALIGN, |_, _| Ret::default());
        let output = e.mem.alloc(0x20);
        e.mem.set_u32(stream + STREAM_OUTPUT, output);
        e.mem.set_u32(stream + STREAM_OUTPUT_LEFT, 0x20);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00b4_6b70, &args![stream, 3i32]).i32(), 0);
        assert_eq!(
            e.mem.bytes(output, 10),
            [0x1f, 0x8b, 8, 0, 0, 0, 0, 0, 0, 0xff]
        );
        assert_eq!(calls_to(&e, TREES_STORED_BLOCK), vec![vec![state, 0, 0, 0]]);
        assert!(calls_to(&e, MEMSET).len() == 1);
        e.call_log = Some(vec![]);
        e.mem.set_u32(stream + STREAM_INPUT, 0x100);
        e.mem.set_u32(stream + STREAM_INPUT_LEFT, 1);
        assert_eq!(e.call(0x00b4_6b70, &args![stream, 1i32]).i32(), 0);
        assert_eq!(calls_to(&e, TREES_ALIGN), vec![vec![state]]);
    }

    /// A compressor state over `text` as its window, ready to compress it from the start
    /// (hashes primed, buffers allocated), with the block flush doubled.
    fn compress_engine(text: &[u8], max_lazy: u32) -> (Engine, u32, u32) {
        let (mut e, stream) = engine();
        e.map(0x010a_8000, 0x1000);
        e.mem.set_u8(LENGTH_CODE, 0);
        e.mem.set_u8(DISTANCE_CODE + 2, 2);
        e.register(TREES_FLUSH_BLOCK, |_, _| Ret::default());
        let state = e.mem.alloc(DEFLATE_STATE_SIZE);
        e.mem.set_u32(stream + STREAM_STATE, state);
        e.mem.set_u32(stream + STREAM_OUTPUT_LEFT, 0x10);
        e.mem.set_u32(state + DS_STREAM, stream);
        let window = e.mem.alloc(0x10000);
        for (i, b) in text.iter().enumerate() {
            e.mem.set_u8(window + i as u32, *b);
        }
        e.mem.set_u32(state + DS_WINDOW, window);
        e.mem.set_u32(state + DS_WINDOW_SIZE, 0x8000);
        e.mem.set_u32(state + DS_WINDOW_DOUBLE, 0x10000);
        e.mem.set_u32(state + DS_WINDOW_MASK, 0x7fff);
        e.mem.set_u32(state + DS_HASH_MASK, 0x7fff);
        e.mem.set_u32(state + DS_HASH_SHIFT, 5);
        let previous = e.mem.alloc(0x10000);
        let head = e.mem.alloc(0x10000);
        e.mem.set_u32(state + DS_PREVIOUS, previous);
        e.mem.set_u32(state + DS_HEAD, head);
        let distances = e.mem.alloc(0x100);
        let literals = e.mem.alloc(0x100);
        e.mem.set_u32(state + DS_DISTANCE_BUFFER, distances);
        e.mem.set_u32(state + DS_LITERAL_BUFFER, literals);
        e.mem.set_u32(state + DS_LITERAL_BUFFER_SIZE, 0x4000);
        e.mem.set_u32(state + DS_LOOKAHEAD, text.len() as u32);
        e.mem.set_u32(state + DS_MATCH_LENGTH, 2);
        e.mem.set_u32(state + DS_PREVIOUS_LENGTH, 2);
        e.mem.set_u32(state + DS_MAX_CHAIN, 4);
        e.mem.set_u32(state + DS_MAX_LAZY, max_lazy);
        e.mem.set_u32(state + DS_GOOD_MATCH, 8);
        e.mem.set_u32(state + DS_NICE_MATCH, 0x102);
        let hash = ((u32::from(text[0]) << 5) ^ u32::from(text[1])) & 0x7fff;
        e.mem.set_u32(state + DS_INSERT_HASH, hash);
        (e, stream, state)
    }

    fn check_abcabcabc(e: &Engine, state: u32) {
        // Four literals, then a 5-byte match 3 back (its length byte 5 - 3).
        assert_eq!(e.mem.u32(state + DS_LAST_LITERAL), 5);
        let literals = e.mem.u32(state + DS_LITERAL_BUFFER);
        let distances = e.mem.u32(state + DS_DISTANCE_BUFFER);
        assert_eq!(e.mem.bytes(literals, 5), b"abca\x02");
        assert_eq!(e.mem.u16(distances + 8), 3);
        assert_eq!(
            e.mem
                .u16(state + DS_LITERAL_FREQUENCIES + 4 * u32::from(b'a')),
            2
        );
        assert_eq!(e.mem.u16(state + DS_LENGTH_FREQUENCIES), 1);
        assert_eq!(e.mem.u16(state + DS_DISTANCE_FREQUENCIES + 8), 1);
        assert_eq!(e.mem.u32(state + DS_STRING_START), 9);
    }

    #[test]
    fn fast_compression_finds_the_repeat() {
        let (mut e, _, state) = compress_engine(b"abcabcabc", 4);
        let window = e.mem.u32(state + DS_WINDOW);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00b4_7d20, &args![state, 4u32]).u32(), 3);
        check_abcabcabc(&e, state);
        assert_eq!(
            calls_to(&e, TREES_FLUSH_BLOCK),
            vec![vec![state, window, 9, 1]]
        );
        // Without a flush and too little ahead: need more.
        let (mut e, _, state) = compress_engine(b"abcabcabc", 4);
        assert_eq!(e.call(0x00b4_7d20, &args![state, 0u32]).u32(), 0);
    }

    #[test]
    fn lazy_compression_finds_the_repeat() {
        let (mut e, _, state) = compress_engine(b"abcabcabc", 4);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00b4_87b0, &args![state, 4u32]).u32(), 3);
        check_abcabcabc(&e, state);
        assert_eq!(e.mem.u32(state + DS_MATCH_AVAILABLE), 0);
        let (mut e, stream, state) = compress_engine(b"abcabcabc", 4);
        e.mem.set_u32(stream + STREAM_OUTPUT_LEFT, 0);
        assert_eq!(e.call(0x00b4_87b0, &args![state, 4u32]).u32(), 0);
    }

    #[test]
    fn stored_compression_emits_one_block() {
        let (mut e, _, state) = compress_engine(b"abcabcabc", 4);
        e.mem.set_u32(state + DS_PENDING_BUFFER_SIZE, 0x1000);
        let window = e.mem.u32(state + DS_WINDOW);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00b4_77e0, &args![state, 4u32]).u32(), 3);
        assert_eq!(
            calls_to(&e, TREES_FLUSH_BLOCK),
            vec![vec![state, window, 9, 1]]
        );
        let (mut e, _, state) = compress_engine(b"abcabcabc", 4);
        e.mem.set_u32(state + DS_PENDING_BUFFER_SIZE, 0x1000);
        assert_eq!(e.call(0x00b4_77e0, &args![state, 1u32]).u32(), 1);
    }

    #[test]
    fn longest_match_compares_the_chain() {
        let (mut e, _, state) = compress_engine(b"abcdabcdxabcd", 4);
        e.mem.set_u32(state + DS_STRING_START, 9);
        e.mem.set_u32(state + DS_LOOKAHEAD, 4);
        let previous = e.mem.u32(state + DS_PREVIOUS);
        e.mem.set_u16(previous + 2 * 4, 0);
        assert_eq!(e.call(0x00b4_82b0, &args![state, 4u32]).u32(), 4);
        assert_eq!(e.mem.u32(state + DS_MATCH_START), 4);
        // The fast form for a one-byte-back run.
        let (mut e, _, state) = compress_engine(b"aaaaaa", 4);
        e.mem.set_u32(state + DS_STRING_START, 1);
        e.mem.set_u32(state + DS_LOOKAHEAD, 5);
        assert_eq!(e.call(0x00b4_85c0, &args![state, 0u32]).u32(), 5);
        assert_eq!(e.call(0x00b4_85c0, &args![state, 7u32]).u32(), 2);
    }

    #[test]
    fn fill_window_slides_and_reads() {
        let (mut e, stream, state) = compress_engine(b"xy", 4);
        e.mem.set_u32(state + DS_WINDOW_SIZE, 0x100);
        e.mem.set_u32(state + DS_WINDOW_DOUBLE, 0x200);
        e.mem.set_u32(state + DS_HASH_SIZE, 4);
        e.mem.set_u32(state + DS_LOOKAHEAD, 0);
        e.mem.set_u32(state + DS_STRING_START, 0x1fa);
        e.mem.set_u32(state + DS_WRAP, 1);
        let head = e.mem.u32(state + DS_HEAD);
        e.mem.set_u16(head, 0x150);
        e.mem.set_u16(head + 2, 0x50);
        let window = e.mem.u32(state + DS_WINDOW);
        e.mem.set_u8(window + 0x100, 0x33);
        e.register(DATA_CHECKSUM, |_, _| answer(9));
        let input = bytes(&mut e, b"hello");
        e.mem.set_u32(stream + STREAM_INPUT, input);
        e.mem.set_u32(stream + STREAM_INPUT_LEFT, 5);
        e.call(0x00b4_7a20, &args![state]);
        assert_eq!(e.mem.u8(window), 0x33);
        assert_eq!(e.mem.u32(state + DS_STRING_START), 0xfa);
        assert_eq!(e.mem.u16(head), 0x50);
        assert_eq!(e.mem.u16(head + 2), 0);
        assert_eq!(e.mem.u32(state + DS_LOOKAHEAD), 5);
        assert_eq!(e.mem.bytes(window + 0xfa, 5), b"hello");
        assert_eq!(e.mem.u32(stream + STREAM_INPUT_LEFT), 0);
        assert_eq!(e.mem.u32(stream + STREAM_ADLER), 9);
        assert_eq!(
            e.mem.u32(state + DS_INSERT_HASH),
            ((u32::from(b'h') << 5) ^ u32::from(b'e')) & 0x7fff
        );
        assert_eq!(e.call(0x00b4_7c50, &args![stream, window, 4u32]).u32(), 0);
    }

    #[test]
    fn lm_init_takes_the_level_row() {
        let (mut e, _, state) = deflate_engine(15);
        e.mem.set_u32(state + DS_LEVEL, 2);
        for (i, value) in [10u16, 11, 12, 13].iter().enumerate() {
            e.mem.set_u16(level_row(2) + 2 * i as u32, *value);
        }
        e.mem.set_u32(state + DS_STRING_START, 7);
        e.call(0x00b4_76f0, &args![state]);
        assert_eq!(e.mem.u32(state + DS_GOOD_MATCH), 10);
        assert_eq!(e.mem.u32(state + DS_MAX_LAZY), 11);
        assert_eq!(e.mem.u32(state + DS_NICE_MATCH), 12);
        assert_eq!(e.mem.u32(state + DS_MAX_CHAIN), 13);
        assert_eq!(e.mem.u32(state + DS_STRING_START), 0);
        assert_eq!(e.mem.u32(state + DS_WINDOW_DOUBLE), 0x10000);
    }

    #[test]
    fn version_flags_and_messages() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00b4_8ee0, &[]).u32(), 0x011a_d714);
        assert_eq!(e.call(0x00b4_8ef0, &[]).u32(), 0x55);
        e.map(0x010a_6000, 0x1000);
        e.mem.set_u32(MESSAGE_TABLE + 4 * 4, 0x4242);
        assert_eq!(e.call(0x00b4_9000, &args![-2i32]).u32(), 0x4242);
    }
}

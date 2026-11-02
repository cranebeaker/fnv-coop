//! `Havok SDK/run_00d965b0` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The thread's memory router (`00c85750`); its `+0x10` is the allocator whose slot `+8` frees a
/// block (pointer, size).
const MEMORY_ROUTER: u32 = 0x00c8_5750;
/// `00df2640` (no unit in the engine map): `thiscall`, two stack words, `RET 8`.
const TABLE_FALLBACK: u32 = 0x00df_2640;
/// `hkJobQueue::addJobQueueLocked` (`00d04b00`, engine map name): `thiscall`, three stack
/// words `(queue-word, job, flag)`, `RET 0xc`.
const ADD_JOB_QUEUE_LOCKED: u32 = 0x00d0_4b00;
/// `00cf6d20` (no unit in the engine map): `thiscall`, no stack words.
const BASE_INIT: u32 = 0x00cf_6d20;
/// The vtable `00d97010` installs.
const VTABLE_00D97010: u32 = 0x010d_3f7c;

/// Frees a block through the router's allocator (slot `+8`, cdecl-like `(block, size)`).
fn router_free(e: &mut Engine, block: u32, size: u32) {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    e.vcall(allocator, 8, &args![block, size]);
}

// Translated from 00d965b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thiscall, no stack words: `this` is a counted block: a dword count at `+0`, the size
/// argument at `+8` and the pointers from `+0xc` on. Each of the `count` pointers is freed
/// through the router's allocator (slot `+8`, with `(pointer, [this + 8])`), then the block
/// itself with `(this, sign-extended 16-bit word at +0, times 4, plus 0xc)`. The class is
/// unconfirmed (the call site is the post-collide job).
pub fn fn_00d965b0(e: &mut Engine, this: Ptr) {
    let this = this.addr();
    let mut index = 0i32;
    if e.mem.i32(this) > 0 {
        loop {
            let size = e.mem.u32(this + 8);
            let pointer = e.mem.u32(
                this.wrapping_add(0xc)
                    .wrapping_add((index as u32).wrapping_mul(4)),
            );
            router_free(e, pointer, size);
            index = index.wrapping_add(1);
            if index >= e.mem.i32(this) {
                break;
            }
        }
    }
    let words = i32::from(e.mem.i16(this)) as u32;
    let bytes = words.wrapping_mul(4).wrapping_add(0xc);
    router_free(e, this, bytes);
}

// Translated from 00d96600 (decompiled, FalloutNV.exe 1.4.0.525)
/// Cdecl, four stack words of which the first two are never read (`_unused_0`, `_unused_1`):
/// `(_, _, source, destination)`; a callback in a table (`00d2cca0` stores its address). Copies
/// the first 0x80 bytes of `source` to `destination` word by word and sets `destination + 0x14`
/// to `[[[source + 0x1c]] + 0x28][16-bit word at destination + 0x10]`; then by the byte kind at
/// `source` it steps the source along and answers 1, or answers 0 (kinds 2, 5, 13 and every kind
/// above 14 answer 0 at once):
/// - 0: when `[source + 0x20] > 1`: the 16-bit word at `+0x10` goes up, `[+0x20]` down and
///   `[destination + 0x20]` becomes 1;
/// - 1: when the 16-bit word at `+0x26` is above 0x80: it loses 0x80, the one at `+0x24` gains
///   it and the 16-bit word at `destination + 0x26` is 0x80;
/// - 3, 6 and 14: the 0x80 bytes are copied again, answer 0;
/// - 4 and 10: when `[[source + 0x20]]` is not null, `[source + 0x20]` becomes it;
/// - 7 and 9: when `[source + 0x20] + 0x4000` is below `[source + 0x24]` it is stored there and
///   at `destination + 0x24`;
/// - 8: when the byte at `[source + 0x2c] + 0x1c` is zero, `[source + 0x2c]` becomes the first
///   word of that node;
/// - 11: as 1 on the words at `+0x34`, `+0x36` and `destination + 0x34`; otherwise, with
///   `[source + 0x20]` not zero, the kind byte becomes 10;
/// - 12: `00df2640` on `source` with `[[source + 0x1c]] + 0x28` and `destination`; its answer.
/// The class is unconfirmed.
pub fn fn_00d96600(
    e: &mut Engine,
    _unused_0: u32,
    _unused_1: u32,
    source: u32,
    destination: u32,
) -> u32 {
    for block in 0..8u32 {
        for word in 0..4u32 {
            let value = e.mem.u32(source.wrapping_add(16 * block + 4 * word));
            e.mem
                .set_u32(destination.wrapping_add(16 * block + 4 * word), value);
        }
    }
    let outer = e.mem.u32(source.wrapping_add(0x1c));
    let table_owner = e.mem.u32(outer);
    let index = u32::from(e.mem.u16(destination.wrapping_add(0x10)));
    let table = e.mem.u32(table_owner.wrapping_add(0x28));
    let entry = e.mem.u32(table.wrapping_add(index.wrapping_mul(4)));
    let table_start = table_owner.wrapping_add(0x28);
    e.mem.set_u32(destination.wrapping_add(0x14), entry);
    let kind = e.mem.u8(source);
    match kind {
        0 => {
            let remaining = e.mem.i32(source.wrapping_add(0x20));
            if remaining > 1 {
                let word = e.mem.u16(source.wrapping_add(0x10)).wrapping_add(1);
                e.mem.set_u16(source.wrapping_add(0x10), word);
                e.mem
                    .set_u32(source.wrapping_add(0x20), (remaining - 1) as u32);
                e.mem.set_u32(destination.wrapping_add(0x20), 1);
                return 1;
            }
            0
        }
        1 => {
            let size = e.mem.u16(source.wrapping_add(0x26));
            if size > 0x80 {
                e.mem
                    .set_u16(source.wrapping_add(0x26), size.wrapping_sub(0x80));
                let word = e.mem.u16(source.wrapping_add(0x24)).wrapping_add(0x80);
                e.mem.set_u16(source.wrapping_add(0x24), word);
                e.mem.set_u16(destination.wrapping_add(0x26), 0x80);
                return 1;
            }
            0
        }
        3 | 6 | 14 => {
            for word in 0..0x20u32 {
                let value = e.mem.u32(source.wrapping_add(4 * word));
                e.mem.set_u32(destination.wrapping_add(4 * word), value);
            }
            0
        }
        4 | 10 => {
            let next = e.mem.u32(source.wrapping_add(0x20));
            if e.mem.u32(next) != 0 {
                let value = e.mem.u32(next);
                e.mem.set_u32(source.wrapping_add(0x20), value);
                return 1;
            }
            0
        }
        7 | 9 => {
            let advanced = e.mem.u32(source.wrapping_add(0x20)).wrapping_add(0x4000);
            if advanced < e.mem.u32(source.wrapping_add(0x24)) {
                e.mem.set_u32(source.wrapping_add(0x20), advanced);
                e.mem.set_u32(destination.wrapping_add(0x24), advanced);
                return 1;
            }
            0
        }
        8 => {
            let node = e.mem.u32(source.wrapping_add(0x2c));
            if e.mem.u8(node.wrapping_add(0x1c)) == 0 {
                let next = e.mem.u32(node);
                e.mem.set_u32(source.wrapping_add(0x2c), next);
                return 1;
            }
            0
        }
        11 => {
            let size = e.mem.u16(source.wrapping_add(0x34));
            if size > 0x80 {
                e.mem
                    .set_u16(source.wrapping_add(0x34), size.wrapping_sub(0x80));
                let word = e.mem.u16(source.wrapping_add(0x36)).wrapping_add(0x80);
                e.mem.set_u16(source.wrapping_add(0x36), word);
                e.mem.set_u16(destination.wrapping_add(0x34), 0x80);
                return 1;
            }
            if e.mem.u32(source.wrapping_add(0x20)) != 0 {
                e.mem.set_u8(source, 0xa);
                return 1;
            }
            0
        }
        12 => e
            .call(TABLE_FALLBACK, &args![source, table_start, destination])
            .u32(),
        _ => 0,
    }
}

/// Starts the job record at `out + 0x10`: the byte `ty`, a zero byte, the byte `flag`, the
/// 16-bit size at `+4`, `0xffff` at `+6`, and the 16-bit word at `+0x10` and the words at
/// `+0x14`, `+0x18` and `+0x1c` copied from `source`. A record address of 0 (`out` =
/// `0xfffffff0`) is skipped, as the exe's `LEA`/`TEST` does; the answer is the record address.
fn start_record(e: &mut Engine, out: u32, ty: u8, flag: u8, size: u16, source: u32) -> u32 {
    let record = out.wrapping_add(0x10);
    if record == 0 {
        return 0;
    }
    e.mem.set_u8(record, ty);
    e.mem.set_u8(record + 1, 0);
    e.mem.set_u8(record + 2, flag);
    e.mem.set_u16(record + 4, size);
    e.mem.set_u16(record + 6, 0xffff);
    let half = e.mem.u16(source.wrapping_add(0x10));
    e.mem.set_u16(record + 0x10, half);
    for offset in [0x14u32, 0x18, 0x1c] {
        let word = e.mem.u32(source.wrapping_add(offset));
        e.mem.set_u32(record + offset, word);
    }
    record
}

// Translated from 00d96870 (decompiled, FalloutNV.exe 1.4.0.525)
/// Register inputs `ecx`, `edx`, `edi` (no stack words): finishes a job. A record of kind 4
/// (flag 1, size 0x30, `0xffff` at `+6`) is started at `edi + 0x10` from the source `edx`
/// (the 16-bit word at `+0x10` and the words at `+0x14`, `+0x18`, `+0x1c` copied) and gets
/// `[ecx + 0x68]` at `+0x20`, `[ecx + 0x4c]` at `+0x24` and the byte at `ecx + 0x58` at `+0x28`
/// (skipped when `edi + 0x10` is 0). Then `[ecx] = [ecx + 0x6c]`, `[edi] = 0` and the byte at
/// `edi + 0x12` is 1. Returns `[ecx + 0x6c]` (`EAX`). The class is unconfirmed.
pub fn fn_00d96870(e: &mut Engine, ecx: u32, edx: u32, edi: u32) -> u32 {
    let record = start_record(e, edi, 4, 1, 0x30, edx);
    if record != 0 {
        let word = e.mem.u32(ecx.wrapping_add(0x68));
        let second = e.mem.u32(ecx.wrapping_add(0x4c));
        e.mem.set_u32(record + 0x24, second);
        let flag = e.mem.u8(ecx.wrapping_add(0x58));
        e.mem.set_u32(record + 0x20, word);
        e.mem.set_u8(record + 0x28, flag);
    }
    let value = e.mem.u32(ecx.wrapping_add(0x6c));
    e.mem.set_u32(ecx, value);
    e.mem.set_u32(edi, 0);
    e.mem.set_u8(edi.wrapping_add(0x12), 1);
    value
}

/// Queues a job record of kind 5 on the job queue (`hkJobQueue::addJobQueueLocked`): a 0x20-byte
/// stack record with the bytes `5, 0, 2` at `+0`, `0x20` and `0xffff` as 16-bit words at `+4` and
/// `+6`, and the 16-bit word at `+0x10` and the words at `+0x14`, `+0x18`, `+0x1c` copied from
/// `source`. The other bytes of the exe's stack record (`+3`, `+8..+0xf`, `+0x12..+0x13`) are
/// never written there and are zero here.
fn queue_job(e: &mut Engine, queue: u32, second: u32, source: u32) {
    e.with_stack(0x20, |e, record| {
        let record = record.addr();
        e.mem.set_u16(record + 4, 0x20);
        let half = e.mem.u16(source.wrapping_add(0x10));
        e.mem.set_u16(record + 6, 0xffff);
        let word = e.mem.u32(source.wrapping_add(0x14));
        e.mem.set_u16(record + 0x10, half);
        e.mem.set_u32(record + 0x14, word);
        let word = e.mem.u32(source.wrapping_add(0x18));
        e.mem.set_u32(record + 0x18, word);
        let word = e.mem.u32(source.wrapping_add(0x1c));
        e.mem.set_u8(record, 5);
        e.mem.set_u8(record + 1, 0);
        e.mem.set_u8(record + 2, 2);
        e.mem.set_u32(record + 0x1c, word);
        e.call(ADD_JOB_QUEUE_LOCKED, &args![queue, second, record, 0u32]);
    });
}

// Translated from 00d96970 (decompiled, FalloutNV.exe 1.4.0.525)
/// Register inputs `ecx` and `eax`, then the two stack words `queue` and `second`: when the
/// byte at `ecx + 0x21` is not zero, a kind-5 job record built from `eax` (see `queue_job`) is
/// queued with `hkJobQueue::addJobQueueLocked`. The class is unconfirmed.
pub fn fn_00d96970(e: &mut Engine, ecx: u32, eax: u32, queue: u32, second: u32) {
    if e.mem.u8(ecx.wrapping_add(0x21)) != 0 {
        queue_job(e, queue, second, eax);
    }
}

// Translated from 00d969e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Cdecl `(queue, second, source, out)`: a job-completion callback. `source` is a job record
/// (byte kind at `+0`, the pointer `p` at `+0x18`), `out` the record that follows. Kinds 1 to 11
/// and 13 are handled (every other kind answers 1); most count `[p]` down and answer 1 until it
/// reaches zero, then start the follow-up record at `out + 0x10` (kind and layout per case, all
/// starting as in `start_record`), clear `[out]` and set the marker byte at `out + 0x12` (1, or 2
/// for the second kind-4 variant when the byte at `p + 0x59` is set) and store the next count
/// in `[p]`; the answer is 0 when a follow-up was made. Kinds 1 and 2 start a kind-3 record
/// (or finish with `00d96870` when `[p + 0x74] <= 0`), kind 3 finishes with `00d96870`, kind 4
/// a kind-6 or kind-7 record and, when the byte at `p + 0x21` is set, queues a kind-5 job from
/// it, kinds 5 and 11 count `[[source + 0x18] + 4]` down and start a kind-12 record, kind 6 sets
/// the byte at `p + 0x20`, kind 7 starts a kind-8 record, kind 8 a kind-8 or kind-9 record,
/// kind 9 a kind-8 or kind-11 record, kind 10 re-links a node and kind 13 starts a kind-14
/// record. A record address of 0 skips that record only. The bytes of the exe's stack record
/// for the queued job that it never writes are zero here. The class is unconfirmed.
pub fn fn_00d969e0(e: &mut Engine, queue: u32, second: u32, source: u32, out: u32) -> u32 {
    let kind = e.mem.u8(source);
    let p = e.mem.u32(source.wrapping_add(0x18));
    match kind {
        1 | 2 => {
            let count = e.mem.u32(p).wrapping_sub(1);
            e.mem.set_u32(p, count);
            if count != 0 {
                return 1;
            }
            if e.mem.i32(p.wrapping_add(0x74)) <= 0 {
                fn_00d96870(e, p, source, out);
                return 0;
            }
            start_record(e, out, 3, 2, 0x20, source);
            e.mem.set_u32(out, 0);
            0
        }
        3 => {
            fn_00d96870(e, p, source, out);
            0
        }
        4 => {
            let count = e.mem.u32(p).wrapping_sub(1);
            e.mem.set_u32(p, count);
            if count != 0 {
                return 1;
            }
            let record = if e.mem.u8(p.wrapping_add(0x58)) != 0 {
                let record = start_record(e, out, 6, 1, 0x40, source);
                if record != 0 {
                    let start = e.mem.u32(p.wrapping_add(0x14));
                    e.mem.set_u32(record + 0x20, start);
                    let word = e.mem.u32(p.wrapping_add(0x30)).wrapping_sub(start);
                    e.mem.set_u32(record + 0x28, word);
                    let word = e
                        .mem
                        .u32(p.wrapping_add(0x3c))
                        .wrapping_sub(e.mem.u32(record + 0x20));
                    e.mem.set_u32(record + 0x2c, word);
                    let word = e
                        .mem
                        .u32(p.wrapping_add(0x40))
                        .wrapping_sub(e.mem.u32(record + 0x20));
                    e.mem.set_u32(record + 0x30, word);
                    let word = e.mem.u32(p.wrapping_add(0x18));
                    e.mem.set_u32(record + 0x24, word);
                    let word = e.mem.u32(p.wrapping_add(0x44));
                    e.mem.set_u32(record + 0x34, word);
                    let word = e.mem.u32(p.wrapping_add(0x48));
                    e.mem.set_u32(record + 0x38, word);
                }
                e.mem.set_u32(out, 0);
                let marker = u8::from(e.mem.u8(p.wrapping_add(0x59)) != 0).wrapping_add(1);
                e.mem.set_u8(out.wrapping_add(0x12), marker);
                record
            } else {
                let record = start_record(e, out, 7, 1, 0x30, source);
                if record != 0 {
                    let word = e.mem.u32(p.wrapping_add(0x30));
                    e.mem.set_u32(record + 0x20, word);
                    let word = e.mem.u32(p.wrapping_add(0x34));
                    e.mem.set_u32(record + 0x24, word);
                }
                e.mem.set_u32(out, 0);
                e.mem.set_u8(out.wrapping_add(0x12), 1);
                let value = u32::from(e.mem.u16(p.wrapping_add(0x38)));
                e.mem.set_u32(p, value);
                record
            };
            if e.mem.u8(p.wrapping_add(0x21)) != 0 {
                queue_job(e, queue, second, record);
            }
            0
        }
        5 => finish_stage(e, source, p, out, 1),
        6 => {
            e.mem.set_u8(p.wrapping_add(0x20), 1);
            1
        }
        7 => {
            let count = e.mem.u32(p).wrapping_sub(1);
            e.mem.set_u32(p, count);
            if count != 0 {
                return 1;
            }
            let record = start_record(e, out, 8, 1, 0x30, source);
            if record != 0 {
                let word = e.mem.u32(p.wrapping_add(0x78));
                e.mem.set_u32(record + 0x2c, word);
                e.mem.set_u32(record + 0x20, 0);
                e.mem.set_u32(record + 0x28, 0);
            }
            e.mem.set_u32(out, 0);
            e.mem.set_u8(out.wrapping_add(0x12), 1);
            let value = u32::from(e.mem.u16(p.wrapping_add(0x7c)));
            e.mem.set_u32(p, value);
            0
        }
        8 => follow_stage_eight(e, source, p, out),
        9 => {
            let count = e.mem.u32(p).wrapping_sub(1);
            e.mem.set_u32(p, count);
            if count != 0 {
                return 1;
            }
            if e.mem.u8(source.wrapping_add(0x2c)) == 0 {
                let record = start_record(e, out, 8, 1, 0x30, source);
                if record != 0 {
                    let word = e.mem.u32(p.wrapping_add(0x78));
                    e.mem.set_u32(record + 0x2c, word);
                    let word = e.mem.u32(source.wrapping_add(0x28)).wrapping_add(1);
                    e.mem.set_u32(record + 0x20, word);
                    e.mem.set_u32(record + 0x28, 0);
                }
                let value = u32::from(e.mem.u16(p.wrapping_add(0x7c)));
                e.mem.set_u32(out, 0);
                e.mem.set_u8(out.wrapping_add(0x12), 1);
                e.mem.set_u32(p, value);
            } else {
                let record = start_record(e, out, 0xb, 2, 0x40, source);
                if record != 0 {
                    let word = e.mem.u32(p.wrapping_add(0x78));
                    e.mem.set_u32(record + 0x20, word);
                    e.mem.set_u16(record + 0x36, 0);
                    let half = e.mem.u16(p.wrapping_add(0x54));
                    e.mem.set_u16(record + 0x34, half);
                    let word = e.mem.u32(p.wrapping_add(0x14));
                    e.mem.set_u32(record + 0x30, word);
                    e.mem.set_u8(record + 0x3c, 0);
                }
                let value = e.mem.u32(p.wrapping_add(0x6c));
                e.mem.set_u32(out, 0);
                e.mem.set_u8(out.wrapping_add(0x12), 1);
                e.mem.set_u32(p, value);
            }
            0
        }
        10 => {
            let count = e.mem.u32(p).wrapping_sub(1);
            e.mem.set_u32(p, count);
            let holder = e.mem.u32(source.wrapping_add(0x20));
            let node = e.mem.u32(holder.wrapping_add(0xc));
            if e.mem.u32(node) != 0 {
                let head = e.mem.u32(p.wrapping_add(0x24));
                e.mem.set_u32(node + 4, head);
                e.mem.set_u32(p.wrapping_add(0x24), node);
            }
            if e.mem.u32(p) == 0 {
                e.mem.set_u8(p.wrapping_add(0x20), 1);
            }
            1
        }
        11 => {
            if e.mem.i32(source.wrapping_add(0x38)) <= 5 {
                e.mem.set_u32(p.wrapping_add(8), 0);
            }
            let size = u32::from(e.mem.u16(source.wrapping_add(0x34)));
            let units = (size.wrapping_sub(1) >> 7).wrapping_add(1);
            finish_stage(e, source, p, out, units)
        }
        13 => {
            let node = e.mem.u32(source.wrapping_add(0x20));
            if node == 0 {
                return 1;
            }
            let count = e.mem.u32(node.wrapping_add(4)).wrapping_sub(1);
            e.mem.set_u32(node.wrapping_add(4), count);
            if count != 0 {
                return 1;
            }
            let record = start_record(e, out, 0xe, 2, 0x30, source);
            if record != 0 {
                let half = e.mem.u16(source.wrapping_add(0x10));
                e.mem.set_u16(record + 0x10, half);
                let word = e.mem.u32(source.wrapping_add(0x20));
                e.mem.set_u32(record + 0x20, word);
            }
            e.mem.set_u32(out, 0);
            0
        }
        _ => 1,
    }
}

/// The shared tail of kinds 5 and 11 of `00d969e0`: `units` is taken off `[p + 4]`... where
/// `p` is the pointer at `source + 0x18`: when it reaches 0 a kind-12 record is started at
/// `out + 0x10` (flag 2, size 0x30, `[source + 0x18]` at `+0x18`, the 16-bit word 1 at `+0x20`)
/// and `[out]` is cleared; the answer is then 0, otherwise 1.
fn finish_stage(e: &mut Engine, source: u32, p: u32, out: u32, units: u32) -> u32 {
    let remaining = e.mem.u32(p.wrapping_add(4)).wrapping_sub(units);
    e.mem.set_u32(p.wrapping_add(4), remaining);
    if remaining != 0 {
        return 1;
    }
    let record = out.wrapping_add(0x10);
    if record != 0 {
        let held = e.mem.u32(source.wrapping_add(0x18));
        e.mem.set_u8(record, 0xc);
        e.mem.set_u8(record + 1, 0);
        e.mem.set_u8(record + 2, 2);
        e.mem.set_u16(record + 4, 0x30);
        e.mem.set_u16(record + 6, 0xffff);
        let half = e.mem.u16(source.wrapping_add(0x10));
        e.mem.set_u16(record + 0x10, half);
        let word = e.mem.u32(source.wrapping_add(0x14));
        e.mem.set_u32(record + 0x14, word);
        let word = e.mem.u32(source.wrapping_add(0x18));
        e.mem.set_u32(record + 0x18, word);
        let word = e.mem.u32(source.wrapping_add(0x1c));
        e.mem.set_u32(record + 0x18, held);
        e.mem.set_u32(record + 0x1c, word);
        e.mem.set_u16(record + 0x20, 1);
    }
    e.mem.set_u32(out, 0);
    0
}

/// Kind 8 of `00d969e0`: counts `[p]` down (a non-zero count answers 1), then by the 16-bit word
/// at `[[source + 0x2c] + 0x1e]`: non-zero starts a kind-8 record that is a copy of the first
/// 0x30 bytes of `source` with `[[source + 0x2c] + 0x20]` at `+0x2c` and answers with that
/// word; zero compares `[source + 0x28] + 1` with `[source + 0x24]`: below it a kind-8 record
/// again copies `source` (with `[+0x28]` one up and `[p + 0x78]` at `+0x2c`) and the next count is
/// the 16-bit word at `p + 0x7c`; otherwise a kind-9 record is started (`[p + 0x30]`,
/// `[p + 0x34]`, `[source + 0x20]` and a zero byte at `+0x2c`) and the count is the 16-bit word at
/// `p + 0x3a`. The count is stored at `[p]`, `[out]` cleared and the byte at `out + 0x12` set; the
/// answer is 0.
fn follow_stage_eight(e: &mut Engine, source: u32, p: u32, out: u32) -> u32 {
    let count = e.mem.u32(p).wrapping_sub(1);
    e.mem.set_u32(p, count);
    if count != 0 {
        return 1;
    }
    let stage = e.mem.u32(source.wrapping_add(0x2c));
    let width = u32::from(e.mem.u16(stage.wrapping_add(0x1e)));
    let next_count;
    if width != 0 {
        let record = start_record(e, out, 8, 1, 0x30, source);
        if record != 0 {
            copy_prefix(e, source, record);
            let stage = e.mem.u32(source.wrapping_add(0x2c));
            let word = e.mem.u32(stage.wrapping_add(0x20));
            e.mem.set_u32(record + 0x2c, word);
        }
        next_count = width;
    } else if e.mem.u32(source.wrapping_add(0x28)).wrapping_add(1)
        < e.mem.u32(source.wrapping_add(0x24))
    {
        let record = start_record(e, out, 8, 1, 0x30, source);
        if record == 0 {
            // The exe's `INC dword [0x28]` on a null record address faults.
            let value = e.mem.u32(0x28).wrapping_add(1);
            e.mem.set_u32(0x28, value);
            let word = e.mem.u32(p.wrapping_add(0x78));
            e.mem.set_u32(0x2c, word);
            next_count = u32::from(e.mem.u16(p.wrapping_add(0x7c)));
        } else {
            copy_prefix(e, source, record);
            let stage = e.mem.u32(source.wrapping_add(0x2c));
            let word = e.mem.u32(stage.wrapping_add(0x20));
            e.mem.set_u32(record + 0x2c, word);
            let value = e.mem.u32(record + 0x28).wrapping_add(1);
            e.mem.set_u32(record + 0x28, value);
            let word = e.mem.u32(p.wrapping_add(0x78));
            e.mem.set_u32(record + 0x2c, word);
            next_count = u32::from(e.mem.u16(p.wrapping_add(0x7c)));
        }
    } else {
        let record = start_record(e, out, 9, 1, 0x30, source);
        if record != 0 {
            let word = e.mem.u32(p.wrapping_add(0x30));
            e.mem.set_u32(record + 0x20, word);
            let word = e.mem.u32(p.wrapping_add(0x34));
            e.mem.set_u32(record + 0x24, word);
            let word = e.mem.u32(source.wrapping_add(0x20));
            e.mem.set_u32(record + 0x28, word);
            e.mem.set_u8(record + 0x2c, 0);
        }
        next_count = u32::from(e.mem.u16(p.wrapping_add(0x3a)));
    }
    e.mem.set_u32(out, 0);
    e.mem.set_u8(out.wrapping_add(0x12), 1);
    e.mem.set_u32(p, next_count);
    0
}

/// `REP MOVSD` of 12 words (0x30 bytes) from `source` to `record`.
fn copy_prefix(e: &mut Engine, source: u32, record: u32) {
    for word in 0..12u32 {
        let value = e.mem.u32(source.wrapping_add(4 * word));
        e.mem.set_u32(record.wrapping_add(4 * word), value);
    }
}

// Translated from 00d97010 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor (the `hkp3AxisSweep` base, from `hkp3axissweep.rs`): `this + 6` the 16-bit word
/// 1, the vtable `010d3f7c` at `+0`, `-15` at `+8`, the 16-bit word `0x8000` at `+0xc`, zero at
/// `+0x10`, then `00cf6d20` on `this + 8`. Returns `this`. The class is unconfirmed.
pub fn fn_00d97010(e: &mut Engine, this: Ptr) -> Ptr {
    let this = this.addr();
    e.mem.set_u16(this + 6, 1);
    e.mem.set_u32(this, VTABLE_00D97010);
    e.mem.set_u32(this + 8, 0xffff_fff1);
    e.mem.set_u16(this + 0xc, 0x8000);
    e.mem.set_u32(this + 0x10, 0);
    e.call(BASE_INIT, &args![this + 8]);
    Ptr::new(this)
}

// Translated from 00d967a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Cdecl, four stack words of which the first two are never read (`_unused_0`, `_unused_1`):
/// `(_, _, source, destination)`; a callback in a table (`00d2cca0` stores its address, with
/// `00d968e0`), the sibling of `00d96600`. Copies the first 0x80 bytes of `source` to
/// `destination` word by word and sets `destination + 0x14` to `[[[source + 0x1c]] +
/// 0x28][16-bit word at destination + 0x10]`; then by the kind byte at `source`, with `n` the
/// 16-bit word at `source + 0x28` and `m` the one at `source + 0x2a`:
/// - 0xd: when `n > m` (unsigned) the 16-bit word at `+0x24` goes up, `+0x28` becomes `n - m`,
///   `[source + 0x40]` gains `m`, the 16-bit words `destination + 0x44` and `destination + 0x28`
///   become 0x200 and `m`; answers 1;
/// - 0xe: the 0x80 bytes are copied again (the word at `destination + 0x14` becomes the source's
///   own); answers 0;
/// - 0xf: when `n > m` the 16-bit word at `+0x24` goes up, `+0x28` becomes `n - m` and
///   `destination + 0x28` becomes `m`; answers 1;
/// - every other kind, and the cases above that do not apply: answers 0.
///
/// The class is unconfirmed.
pub fn fn_00d967a0(
    e: &mut Engine,
    _unused_0: u32,
    _unused_1: u32,
    source: u32,
    destination: u32,
) -> u32 {
    for block in 0..8u32 {
        for word in 0..4u32 {
            let value = e.mem.u32(source.wrapping_add(16 * block + 4 * word));
            e.mem
                .set_u32(destination.wrapping_add(16 * block + 4 * word), value);
        }
    }
    let outer = e.mem.u32(source.wrapping_add(0x1c));
    let table_owner = e.mem.u32(outer);
    let index = u32::from(e.mem.u16(destination.wrapping_add(0x10)));
    let table = e.mem.u32(table_owner.wrapping_add(0x28));
    let entry = e.mem.u32(table.wrapping_add(index.wrapping_mul(4)));
    e.mem.set_u32(destination.wrapping_add(0x14), entry);
    match e.mem.u8(source) {
        0xd => {
            let total = e.mem.u16(source.wrapping_add(0x28));
            let part = e.mem.u16(source.wrapping_add(0x2a));
            if total <= part {
                return 0;
            }
            let counter = e.mem.u16(source.wrapping_add(0x24)).wrapping_add(1);
            e.mem.set_u16(source.wrapping_add(0x24), counter);
            e.mem
                .set_u16(source.wrapping_add(0x28), total.wrapping_sub(part));
            let moved = e.mem.u32(source.wrapping_add(0x40));
            e.mem.set_u32(
                source.wrapping_add(0x40),
                moved.wrapping_add(u32::from(part)),
            );
            e.mem.set_u16(destination.wrapping_add(0x44), 0x200);
            e.mem.set_u16(destination.wrapping_add(0x28), part);
            1
        }
        0xe => {
            for word in 0..0x20u32 {
                let value = e.mem.u32(source.wrapping_add(4 * word));
                e.mem.set_u32(destination.wrapping_add(4 * word), value);
            }
            0
        }
        0xf => {
            let total = e.mem.u16(source.wrapping_add(0x28));
            let part = e.mem.u16(source.wrapping_add(0x2a));
            if total <= part {
                return 0;
            }
            let counter = e.mem.u16(source.wrapping_add(0x24)).wrapping_add(1);
            e.mem.set_u16(source.wrapping_add(0x24), counter);
            e.mem
                .set_u16(source.wrapping_add(0x28), total.wrapping_sub(part));
            e.mem.set_u16(destination.wrapping_add(0x28), part);
            1
        }
        _ => 0,
    }
}

// Translated from 00d968e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Cdecl, four stack words of which the first two are never read (`_unused_0`, `_unused_1`):
/// `(_, _, source, destination)`; a callback in a table (`00d2cca0` stores its address, with
/// `00d967a0`). Answers 1 for every kind of `source` (the byte at `+0`) except 0xd. For kind 0xd
/// with a pointer at `source + 0x20`: it takes one off the count at the pointer's `+4`; while the
/// count stays above 0 it answers 1. At 0 it starts the record of kind 0xe at `destination +
/// 0x10` (flag 2, size 0x30, `0xffff` at `+6`, the 16-bit word at `+0x10` and the words at `+0x14`,
/// `+0x18`, `+0x1c` copied from `source`) with the source's word at `+0x20` added at `+0x20`
/// (skipped when `destination + 0x10` is 0); then it clears the word at `destination` and
/// answers 0. The class is unconfirmed.
pub fn fn_00d968e0(
    e: &mut Engine,
    _unused_0: u32,
    _unused_1: u32,
    source: u32,
    destination: u32,
) -> u32 {
    if e.mem.u8(source) != 0xd {
        return 1;
    }
    let shared = e.mem.u32(source.wrapping_add(0x20));
    if shared == 0 {
        return 1;
    }
    let count = e.mem.u32(shared.wrapping_add(4)).wrapping_sub(1);
    e.mem.set_u32(shared.wrapping_add(4), count);
    if count != 0 {
        return 1;
    }
    let record = start_record(e, destination, 0xe, 2, 0x30, source);
    if record != 0 {
        let word = e.mem.u32(source.wrapping_add(0x20));
        e.mem.set_u32(record.wrapping_add(0x20), word);
    }
    e.mem.set_u32(destination, 0);
    0
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00d967a0, fn_00d967a0(u32, u32, u32, u32) -> u32),
        entry!(0x00d968e0, fn_00d968e0(u32, u32, u32, u32) -> u32),
        entry!(0x00d9_65b0, fn_00d965b0(Ptr)),
        entry!(0x00d9_6600, fn_00d96600(u32, u32, u32, u32) -> u32),
        entry!(0x00d9_6870, fn_00d96870(u32, u32, u32) -> u32),
        entry!(0x00d9_6970, fn_00d96970(u32, u32, u32, u32)),
        entry!(0x00d9_69e0, fn_00d969e0(u32, u32, u32, u32) -> u32),
        entry!(0x00d9_7010, fn_00d97010(Ptr) -> Ptr),
    ]
}

#[cfg(test)]
mod a09_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    /// A source of kind `kind` for `00d967a0`: index 2 in the word at `+0x10`, the table entry
    /// 0xbeef behind `[[+0x1c]] + 0x28`, `n` and `m` at `+0x28` and `+0x2a`.
    fn source_00d967a0(e: &mut Engine, kind: u8, total: u16, part: u16) -> u32 {
        let source = e.mem.alloc(0x100);
        let table = e.mem.alloc(0x40);
        e.mem.set_u32(table + 8, 0xbeef);
        let owner = e.mem.alloc(0x40);
        e.mem.set_u32(owner + 0x28, table);
        let outer = e.mem.alloc(8);
        e.mem.set_u32(outer, owner);
        e.mem.set_u8(source, kind);
        e.mem.set_u16(source + 0x10, 2);
        e.mem.set_u32(source + 0x14, 0x1414);
        e.mem.set_u32(source + 0x1c, outer);
        e.mem.set_u16(source + 0x24, 7);
        e.mem.set_u16(source + 0x28, total);
        e.mem.set_u16(source + 0x2a, part);
        e.mem.set_u32(source + 0x40, 0x1000);
        source
    }

    #[test]
    fn fn_00d967a0_splits_kind_0xd_by_the_part_size() {
        let mut e = Engine::new();
        let source = source_00d967a0(&mut e, 0xd, 10, 3);
        let destination = e.mem.alloc(0x100);
        let answer = e
            .call(0x00d9_67a0, &args![0u32, 0u32, source, destination])
            .u32();
        assert_eq!(answer, 1);
        assert_eq!(e.mem.u8(destination), 0xd);
        assert_eq!(e.mem.u32(destination + 0x14), 0xbeef);
        assert_eq!(e.mem.u16(source + 0x24), 8);
        assert_eq!(e.mem.u16(source + 0x28), 7);
        assert_eq!(e.mem.u32(source + 0x40), 0x1003);
        assert_eq!(e.mem.u16(destination + 0x44), 0x200);
        assert_eq!(e.mem.u16(destination + 0x28), 3);
    }

    #[test]
    fn fn_00d967a0_splits_kind_0xf_without_the_offset_and_size_words() {
        let mut e = Engine::new();
        let source = source_00d967a0(&mut e, 0xf, 10, 3);
        let destination = e.mem.alloc(0x100);
        let answer = e
            .call(0x00d9_67a0, &args![0u32, 0u32, source, destination])
            .u32();
        assert_eq!(answer, 1);
        assert_eq!(e.mem.u16(source + 0x24), 8);
        assert_eq!(e.mem.u16(source + 0x28), 7);
        assert_eq!(e.mem.u32(source + 0x40), 0x1000);
        assert_eq!(e.mem.u16(destination + 0x44), 0);
        assert_eq!(e.mem.u16(destination + 0x28), 3);
    }

    #[test]
    fn fn_00d967a0_answers_zero_when_the_part_is_not_smaller() {
        for kind in [0xd, 0xf] {
            let mut e = Engine::new();
            let source = source_00d967a0(&mut e, kind, 5, 5);
            let destination = e.mem.alloc(0x100);
            let answer = e
                .call(0x00d9_67a0, &args![0u32, 0u32, source, destination])
                .u32();
            assert_eq!(answer, 0);
            assert_eq!(e.mem.u16(source + 0x24), 7);
            assert_eq!(e.mem.u32(destination + 0x14), 0xbeef);
        }
    }

    #[test]
    fn fn_00d967a0_copies_kind_0xe_again_and_answers_zero() {
        let mut e = Engine::new();
        let source = source_00d967a0(&mut e, 0xe, 10, 3);
        let destination = e.mem.alloc(0x100);
        let answer = e
            .call(0x00d9_67a0, &args![0u32, 0u32, source, destination])
            .u32();
        assert_eq!(answer, 0);
        // The table entry is overwritten by the second copy.
        assert_eq!(e.mem.u32(destination + 0x14), 0x1414);
        assert_eq!(e.mem.u8(destination), 0xe);
    }

    #[test]
    fn fn_00d967a0_answers_zero_for_other_kinds() {
        let mut e = Engine::new();
        let source = source_00d967a0(&mut e, 1, 10, 3);
        let destination = e.mem.alloc(0x100);
        let answer = e
            .call(0x00d9_67a0, &args![0u32, 0u32, source, destination])
            .u32();
        assert_eq!(answer, 0);
        assert_eq!(e.mem.u32(destination + 0x14), 0xbeef);
        assert_eq!(e.mem.u16(source + 0x24), 7);
    }

    /// A source of kind `kind` whose pointer at `+0x20` has the count `count` at `+4`.
    fn shared_source_00d968e0(e: &mut Engine, kind: u8, count: u32) -> (u32, u32) {
        let source = e.mem.alloc(0x40);
        let shared = e.mem.alloc(8);
        e.mem.set_u32(shared + 4, count);
        e.mem.set_u8(source, kind);
        e.mem.set_u16(source + 0x10, 0x1234);
        e.mem.set_u32(source + 0x14, 1);
        e.mem.set_u32(source + 0x18, 2);
        e.mem.set_u32(source + 0x1c, 3);
        e.mem.set_u32(source + 0x20, shared);
        (source, shared)
    }

    #[test]
    fn fn_00d968e0_answers_one_for_other_kinds_and_for_a_shared_count_above_one() {
        let mut e = Engine::new();
        let destination = e.mem.alloc(0x80);
        let (other, _) = shared_source_00d968e0(&mut e, 4, 1);
        assert_eq!(
            e.call(0x00d9_68e0, &args![0u32, 0u32, other, destination])
                .u32(),
            1
        );
        let (busy, shared) = shared_source_00d968e0(&mut e, 0xd, 3);
        assert_eq!(
            e.call(0x00d9_68e0, &args![0u32, 0u32, busy, destination])
                .u32(),
            1
        );
        assert_eq!(e.mem.u32(shared + 4), 2);
        let unshared = e.mem.alloc(0x40);
        e.mem.set_u8(unshared, 0xd);
        assert_eq!(
            e.call(0x00d9_68e0, &args![0u32, 0u32, unshared, destination])
                .u32(),
            1
        );
    }

    #[test]
    fn fn_00d968e0_starts_the_record_when_the_last_user_lets_go() {
        let mut e = Engine::new();
        let destination = e.mem.alloc(0x80);
        e.mem.set_u32(destination, 0x55);
        let (source, shared) = shared_source_00d968e0(&mut e, 0xd, 1);
        let answer = e
            .call(0x00d9_68e0, &args![0u32, 0u32, source, destination])
            .u32();
        assert_eq!(answer, 0);
        assert_eq!(e.mem.u32(shared + 4), 0);
        assert_eq!(e.mem.u32(destination), 0);
        let record = destination + 0x10;
        assert_eq!(e.mem.u8(record), 0xe);
        assert_eq!(e.mem.u8(record + 1), 0);
        assert_eq!(e.mem.u8(record + 2), 2);
        assert_eq!(e.mem.u16(record + 4), 0x30);
        assert_eq!(e.mem.u16(record + 6), 0xffff);
        assert_eq!(e.mem.u16(record + 0x10), 0x1234);
        assert_eq!(e.mem.u32(record + 0x14), 1);
        assert_eq!(e.mem.u32(record + 0x18), 2);
        assert_eq!(e.mem.u32(record + 0x1c), 3);
        assert_eq!(e.mem.u32(record + 0x20), shared);
    }

    const FREE_SLOT: u32 = 0x0900_0008;

    type Calls = Rc<RefCell<Vec<(u32, Vec<u32>)>>>;

    fn record(e: &mut Engine, addresses: &[u32]) -> Calls {
        let calls: Calls = Rc::new(RefCell::new(Vec::new()));
        for address in addresses {
            let log = Rc::clone(&calls);
            let address = *address;
            e.register_double(address, move |_, a| {
                log.borrow_mut().push((address, a.to_vec()));
                Ret::default()
            });
        }
        calls
    }

    fn router(e: &mut Engine) {
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 8, FREE_SLOT);
        let allocator = e.mem.alloc(0x10);
        e.mem.set_u32(allocator, vtable);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
    }

    #[test]
    fn the_counted_block_frees_each_pointer_and_then_itself() {
        let mut e = Engine::new();
        router(&mut e);
        let frees = record(&mut e, &[FREE_SLOT]);
        let this = e.mem.alloc(0x40);
        e.mem.set_u32(this, 2);
        e.mem.set_u32(this + 8, 0x77);
        e.mem.set_u32(this + 0xc, 0xa000);
        e.mem.set_u32(this + 0x10, 0xb000);
        e.call(0x00d9_65b0, &args![this]);
        let calls = frees.borrow();
        let allocator_this = calls[0].1[0];
        assert_eq!(calls.len(), 3);
        assert_eq!(calls[0].1, vec![allocator_this, 0xa000, 0x77]);
        assert_eq!(calls[1].1, vec![allocator_this, 0xb000, 0x77]);
        assert_eq!(calls[2].1, vec![allocator_this, this, 2 * 4 + 0xc]);
    }

    #[test]
    fn an_empty_block_and_a_negative_word_free_only_the_block() {
        let mut e = Engine::new();
        router(&mut e);
        let frees = record(&mut e, &[FREE_SLOT]);
        let this = e.mem.alloc(0x40);
        // Count 0 in the dword but the 16-bit word -3 in the low half is not what the loop
        // reads: the loop reads the dword, the size reads the signed word.
        e.mem.set_u32(this, 0xffff_fffd);
        e.call(0x00d9_65b0, &args![this]);
        let calls = frees.borrow();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].1[1..], [this, 0]);
    }

    /// A source of `size` bytes with distinct words, and a destination.
    fn pair(e: &mut Engine) -> (u32, u32) {
        let owner = e.mem.alloc(0x40);
        let table = e.mem.alloc(0x40);
        e.mem.set_u32(owner + 0x28, table);
        for slot in 0..8u32 {
            e.mem.set_u32(table + 4 * slot, 0xe000 + slot);
        }
        let outer = e.mem.alloc(0x10);
        e.mem.set_u32(outer, owner);
        let source = e.mem.alloc(0x90);
        for word in 0..0x24u32 {
            e.mem.set_u32(source + 4 * word, 0x1000_0000 + word);
        }
        e.mem.set_u32(source + 0x1c, outer);
        let destination = e.mem.alloc(0x90);
        e.mem.set_u16(source + 0x10, 3);
        (source, destination)
    }

    fn set_kind(e: &mut Engine, source: u32, kind: u8) {
        e.mem.set_u8(source, kind);
    }

    #[test]
    fn the_callback_copies_the_record_and_looks_up_the_table_entry() {
        let mut e = Engine::new();
        let (source, destination) = pair(&mut e);
        set_kind(&mut e, source, 2);
        assert_eq!(
            e.call(0x00d9_6600, &args![0u32, 0u32, source, destination])
                .u32(),
            0
        );
        for word in 0..0x20u32 {
            if word == 5 {
                continue;
            }
            assert_eq!(
                e.mem.u32(destination + 4 * word),
                e.mem.u32(source + 4 * word),
                "{word}"
            );
        }
        // The word at destination + 0x10 (3, copied) indexes the table.
        assert_eq!(e.mem.u32(destination + 0x14), 0xe003);
    }

    #[test]
    fn kinds_0_and_1_step_the_counters() {
        let mut e = Engine::new();
        let (source, destination) = pair(&mut e);
        set_kind(&mut e, source, 0);
        e.mem.set_u32(source + 0x20, 1);
        assert_eq!(
            e.call(0x00d9_6600, &args![0u32, 0u32, source, destination])
                .u32(),
            0
        );
        e.mem.set_u32(source + 0x20, 5);
        assert_eq!(
            e.call(0x00d9_6600, &args![0u32, 0u32, source, destination])
                .u32(),
            1
        );
        assert_eq!(e.mem.u16(source + 0x10), 4);
        assert_eq!(e.mem.u32(source + 0x20), 4);
        assert_eq!(e.mem.u32(destination + 0x20), 1);
        set_kind(&mut e, source, 1);
        e.mem.set_u16(source + 0x26, 0x80);
        assert_eq!(
            e.call(0x00d9_6600, &args![0u32, 0u32, source, destination])
                .u32(),
            0
        );
        e.mem.set_u16(source + 0x26, 0x180);
        e.mem.set_u16(source + 0x24, 0x10);
        assert_eq!(
            e.call(0x00d9_6600, &args![0u32, 0u32, source, destination])
                .u32(),
            1
        );
        assert_eq!(e.mem.u16(source + 0x26), 0x100);
        assert_eq!(e.mem.u16(source + 0x24), 0x90);
        assert_eq!(e.mem.u16(destination + 0x26), 0x80);
    }

    #[test]
    fn kinds_3_6_and_14_copy_the_whole_record_again_and_answer_zero() {
        let mut e = Engine::new();
        let (source, destination) = pair(&mut e);
        for kind in [3u8, 6, 14] {
            set_kind(&mut e, source, kind);
            assert_eq!(
                e.call(0x00d9_6600, &args![0u32, 0u32, source, destination])
                    .u32(),
                0
            );
            // Even the table entry word is the plain copy now.
            assert_eq!(e.mem.u32(destination + 0x14), e.mem.u32(source + 0x14));
        }
    }

    #[test]
    fn list_kinds_follow_their_links() {
        let mut e = Engine::new();
        let (source, destination) = pair(&mut e);
        let node_a = e.mem.alloc(0x20);
        let node_b = e.mem.alloc(0x20);
        e.mem.set_u32(node_a, node_b);
        e.mem.set_u32(node_b, 0);
        for kind in [4u8, 10] {
            set_kind(&mut e, source, kind);
            e.mem.set_u32(source + 0x20, node_a);
            assert_eq!(
                e.call(0x00d9_6600, &args![0u32, 0u32, source, destination])
                    .u32(),
                1
            );
            assert_eq!(e.mem.u32(source + 0x20), node_b);
            assert_eq!(
                e.call(0x00d9_6600, &args![0u32, 0u32, source, destination])
                    .u32(),
                0
            );
        }
        // Kind 8: the node's byte at +0x1c must be zero.
        set_kind(&mut e, source, 8);
        e.mem.set_u32(source + 0x2c, node_a);
        assert_eq!(
            e.call(0x00d9_6600, &args![0u32, 0u32, source, destination])
                .u32(),
            1
        );
        assert_eq!(e.mem.u32(source + 0x2c), node_b);
        e.mem.set_u8(node_b + 0x1c, 1);
        assert_eq!(
            e.call(0x00d9_6600, &args![0u32, 0u32, source, destination])
                .u32(),
            0
        );
    }

    #[test]
    fn kinds_7_9_11_and_12_have_their_own_rules() {
        let mut e = Engine::new();
        let (source, destination) = pair(&mut e);
        for kind in [7u8, 9] {
            set_kind(&mut e, source, kind);
            e.mem.set_u32(source + 0x20, 0x1000);
            e.mem.set_u32(source + 0x24, 0x6000);
            assert_eq!(
                e.call(0x00d9_6600, &args![0u32, 0u32, source, destination])
                    .u32(),
                1
            );
            assert_eq!(e.mem.u32(source + 0x20), 0x5000);
            assert_eq!(e.mem.u32(destination + 0x24), 0x5000);
            e.mem.set_u32(source + 0x20, 0x3000);
            assert_eq!(
                e.call(0x00d9_6600, &args![0u32, 0u32, source, destination])
                    .u32(),
                0
            );
        }
        set_kind(&mut e, source, 11);
        e.mem.set_u16(source + 0x34, 0x100);
        e.mem.set_u16(source + 0x36, 1);
        assert_eq!(
            e.call(0x00d9_6600, &args![0u32, 0u32, source, destination])
                .u32(),
            1
        );
        assert_eq!(e.mem.u16(source + 0x34), 0x80);
        assert_eq!(e.mem.u16(source + 0x36), 0x81);
        assert_eq!(e.mem.u16(destination + 0x34), 0x80);
        e.mem.set_u32(source + 0x20, 9);
        assert_eq!(
            e.call(0x00d9_6600, &args![0u32, 0u32, source, destination])
                .u32(),
            1
        );
        assert_eq!(e.mem.u8(source), 0xa);
        set_kind(&mut e, source, 11);
        e.mem.set_u32(source + 0x20, 0);
        assert_eq!(
            e.call(0x00d9_6600, &args![0u32, 0u32, source, destination])
                .u32(),
            0
        );
        // Kind 12 hands over to 00df2640 and returns its answer.
        e.register(TABLE_FALLBACK, |_, _| Ret {
            eax: 0x4242,
            ..Ret::default()
        });
        set_kind(&mut e, source, 12);
        assert_eq!(
            e.call(0x00d9_6600, &args![0u32, 0u32, source, destination])
                .u32(),
            0x4242
        );
    }

    #[test]
    fn the_big_job_callback_is_one_for_unlisted_kinds() {
        let mut e = Engine::new();
        let (source, _) = pair(&mut e);
        let out = e.mem.alloc(0x80);
        let p = e.mem.alloc(0x100);
        e.mem.set_u32(source + 0x18, p);
        for kind in [0u8, 12, 14, 15, 16, 17, 200] {
            set_kind(&mut e, source, kind);
            assert_eq!(
                e.call(0x00d9_69e0, &args![1u32, 2u32, source, out]).u32(),
                1
            );
        }
        // Kind 6 raises a flag byte; kind 10 re-links a node.
        set_kind(&mut e, source, 6);
        assert_eq!(
            e.call(0x00d9_69e0, &args![1u32, 2u32, source, out]).u32(),
            1
        );
        assert_eq!(e.mem.u8(p + 0x20), 1);
    }

    /// The job record fixture: a source of kind `kind` whose `p` has the count `count` and
    /// recognisable words; returns `(source, p, out)`.
    fn job(e: &mut Engine, kind: u8, count: u32) -> (u32, u32, u32) {
        let source = e.mem.alloc(0x80);
        for word in 0..0x20u32 {
            e.mem.set_u32(source + 4 * word, 0x2000_0000 + word);
        }
        e.mem.set_u8(source, kind);
        let p = e.mem.alloc(0x100);
        for word in 0..0x40u32 {
            e.mem.set_u32(p + 4 * word, 0x3000_0000 + word);
        }
        e.mem.set_u32(p, count);
        e.mem.set_u32(source + 0x18, p);
        let out = e.mem.alloc(0x80);
        e.mem.set_u32(out, 0x5555);
        (source, p, out)
    }

    fn header(e: &Engine, out: u32) -> (u8, u8, u8, u16, u16) {
        let r = out + 0x10;
        (
            e.mem.u8(r),
            e.mem.u8(r + 1),
            e.mem.u8(r + 2),
            e.mem.u16(r + 4),
            e.mem.u16(r + 6),
        )
    }

    #[test]
    fn kinds_1_and_2_count_down_then_make_a_kind_3_record_or_finish() {
        for kind in [1u8, 2] {
            let mut e = Engine::new();
            let (source, p, out) = job(&mut e, kind, 2);
            assert_eq!(
                e.call(0x00d9_69e0, &args![1u32, 2u32, source, out]).u32(),
                1
            );
            assert_eq!(e.mem.u32(p), 1);
            // Last count with `[p + 0x74] > 0`: a kind-3 record of size 0x20.
            e.mem.set_u32(p + 0x74, 1);
            assert_eq!(
                e.call(0x00d9_69e0, &args![1u32, 2u32, source, out]).u32(),
                0
            );
            assert_eq!(header(&e, out), (3, 0, 2, 0x20, 0xffff));
            assert_eq!(e.mem.u16(out + 0x20), e.mem.u16(source + 0x10));
            assert_eq!(e.mem.u32(out + 0x24), e.mem.u32(source + 0x14));
            assert_eq!(e.mem.u32(out + 0x28), e.mem.u32(source + 0x18));
            assert_eq!(e.mem.u32(out + 0x2c), e.mem.u32(source + 0x1c));
            assert_eq!(e.mem.u32(out), 0);
            // With `[p + 0x74] <= 0` the 00d96870 path runs instead.
            let (source, p, out) = job(&mut e, kind, 1);
            e.mem.set_u32(p + 0x74, 0);
            e.mem.set_u32(p + 0x6c, 0x99);
            assert_eq!(
                e.call(0x00d9_69e0, &args![1u32, 2u32, source, out]).u32(),
                0
            );
            assert_eq!(header(&e, out), (4, 0, 1, 0x30, 0xffff));
            assert_eq!(e.mem.u32(p), 0x99);
            assert_eq!(e.mem.u8(out + 0x12), 1);
        }
    }

    #[test]
    fn kind_3_finishes_through_00d96870_without_counting() {
        let mut e = Engine::new();
        let (source, p, out) = job(&mut e, 3, 7);
        e.mem.set_u32(p + 0x6c, 0x1234);
        assert_eq!(
            e.call(0x00d9_69e0, &args![1u32, 2u32, source, out]).u32(),
            0
        );
        assert_eq!(e.mem.u32(p), 0x1234);
        assert_eq!(header(&e, out), (4, 0, 1, 0x30, 0xffff));
        assert_eq!(e.mem.u32(out + 0x30), e.mem.u32(p + 0x68));
        assert_eq!(e.mem.u32(out + 0x34), e.mem.u32(p + 0x4c));
        assert_eq!(e.mem.u8(out + 0x38), e.mem.u8(p + 0x58));
    }

    #[test]
    fn kind_4_with_the_flag_byte_makes_a_kind_6_record_and_queues_a_job() {
        let mut e = Engine::new();
        let queued = record(&mut e, &[ADD_JOB_QUEUE_LOCKED]);
        let (source, p, out) = job(&mut e, 4, 1);
        e.mem.set_u8(p + 0x58, 1);
        e.mem.set_u8(p + 0x59, 1);
        e.mem.set_u8(p + 0x21, 1);
        e.mem.set_u32(p + 0x14, 100);
        e.mem.set_u32(p + 0x30, 160);
        e.mem.set_u32(p + 0x3c, 170);
        e.mem.set_u32(p + 0x40, 180);
        assert_eq!(
            e.call(0x00d9_69e0, &args![0x11u32, 0x22u32, source, out])
                .u32(),
            0
        );
        assert_eq!(header(&e, out), (6, 0, 2, 0x40, 0xffff));
        assert_eq!(e.mem.u32(out + 0x30), 100);
        assert_eq!(e.mem.u32(out + 0x38), 60);
        assert_eq!(e.mem.u32(out + 0x3c), 70);
        assert_eq!(e.mem.u32(out + 0x40), 80);
        assert_eq!(e.mem.u32(out + 0x34), e.mem.u32(p + 0x18));
        assert_eq!(e.mem.u8(out + 0x12), 2);
        assert_eq!(e.mem.u32(out), 0);
        let queued = queued.borrow();
        assert_eq!(queued.len(), 1);
        assert_eq!(queued[0].1[0], 0x11);
        assert_eq!(queued[0].1[1], 0x22);
        assert_eq!(queued[0].1[3], 0);
    }

    #[test]
    fn kind_4_without_the_flag_makes_a_kind_7_record_and_reloads_the_count() {
        let mut e = Engine::new();
        let queued = record(&mut e, &[ADD_JOB_QUEUE_LOCKED]);
        let (source, p, out) = job(&mut e, 4, 1);
        e.mem.set_u8(p + 0x58, 0);
        e.mem.set_u8(p + 0x21, 0);
        e.mem.set_u16(p + 0x38, 0x1234);
        assert_eq!(
            e.call(0x00d9_69e0, &args![0x11u32, 0x22u32, source, out])
                .u32(),
            0
        );
        assert_eq!(header(&e, out), (7, 0, 1, 0x30, 0xffff));
        assert_eq!(e.mem.u32(out + 0x30), e.mem.u32(p + 0x30));
        assert_eq!(e.mem.u32(out + 0x34), e.mem.u32(p + 0x34));
        assert_eq!(e.mem.u32(p), 0x1234);
        assert_eq!(e.mem.u8(out + 0x12), 1);
        assert!(queued.borrow().is_empty());
        // Not the last count: nothing but the decrement.
        let (source, p, out) = job(&mut e, 4, 3);
        assert_eq!(
            e.call(0x00d9_69e0, &args![0x11u32, 0x22u32, source, out])
                .u32(),
            1
        );
        assert_eq!(e.mem.u32(p), 2);
    }

    #[test]
    fn kinds_5_and_11_count_the_shared_stage_down_to_a_kind_12_record() {
        let mut e = Engine::new();
        let (source, p, out) = job(&mut e, 5, 9);
        e.mem.set_u32(p + 4, 2);
        assert_eq!(
            e.call(0x00d9_69e0, &args![0u32, 0u32, source, out]).u32(),
            1
        );
        assert_eq!(e.mem.u32(p + 4), 1);
        assert_eq!(
            e.call(0x00d9_69e0, &args![0u32, 0u32, source, out]).u32(),
            0
        );
        assert_eq!(header(&e, out), (0xc, 0, 2, 0x30, 0xffff));
        assert_eq!(e.mem.u32(out + 0x28), p);
        assert_eq!(e.mem.u16(out + 0x30), 1);
        assert_eq!(e.mem.u32(out), 0);
        // Kind 11: 0x100 bytes are two 0x80 units; a small `[source + 0x38]` clears `[p + 8]`.
        let (source, p, out) = job(&mut e, 11, 9);
        e.mem.set_u16(source + 0x34, 0x100);
        e.mem.set_u32(source + 0x38, 3);
        e.mem.set_u32(p + 4, 5);
        assert_eq!(
            e.call(0x00d9_69e0, &args![0u32, 0u32, source, out]).u32(),
            1
        );
        assert_eq!(e.mem.u32(p + 4), 3);
        assert_eq!(e.mem.u32(p + 8), 0);
        e.mem.set_u32(source + 0x38, 9);
        e.mem.set_u32(p + 8, 0x77);
        e.mem.set_u32(p + 4, 2);
        assert_eq!(
            e.call(0x00d9_69e0, &args![0u32, 0u32, source, out]).u32(),
            0
        );
        assert_eq!(e.mem.u32(p + 8), 0x77);
        assert_eq!(e.mem.u32(p + 4), 0);
    }

    #[test]
    fn kind_7_makes_a_kind_8_record_and_reloads_the_count() {
        let mut e = Engine::new();
        let (source, p, out) = job(&mut e, 7, 2);
        assert_eq!(
            e.call(0x00d9_69e0, &args![0u32, 0u32, source, out]).u32(),
            1
        );
        e.mem.set_u16(p + 0x7c, 0x55);
        e.mem.set_u32(p + 0x78, 0xaa);
        assert_eq!(
            e.call(0x00d9_69e0, &args![0u32, 0u32, source, out]).u32(),
            0
        );
        assert_eq!(header(&e, out), (8, 0, 1, 0x30, 0xffff));
        assert_eq!(e.mem.u32(out + 0x3c), 0xaa);
        assert_eq!(e.mem.u32(out + 0x30), 0);
        assert_eq!(e.mem.u32(out + 0x38), 0);
        assert_eq!(e.mem.u32(p), 0x55);
        assert_eq!(e.mem.u8(out + 0x12), 1);
    }

    /// A kind-8 job whose stage record at `[source + 0x2c]` has the width word `width`.
    fn kind_eight(e: &mut Engine, width: u16) -> (u32, u32, u32, u32) {
        let (source, p, out) = job(e, 8, 1);
        let stage = e.mem.alloc(0x40);
        e.mem.set_u16(stage + 0x1e, width);
        e.mem.set_u32(stage + 0x20, 0x9090);
        e.mem.set_u32(source + 0x2c, stage);
        (source, p, out, stage)
    }

    #[test]
    fn kind_8_with_a_width_copies_the_source_into_a_new_kind_8_record() {
        let mut e = Engine::new();
        let (source, p, out, _) = kind_eight(&mut e, 5);
        assert_eq!(
            e.call(0x00d9_69e0, &args![0u32, 0u32, source, out]).u32(),
            0
        );
        // The record is a copy of the first 0x30 bytes of the source, then the stage's word.
        let source_word = e.mem.u32(source);
        // The marker byte at `out + 0x12` is set to 1 after the copy.
        assert_eq!(
            e.mem.u32(out + 0x10),
            (source_word & 0xff00_ffff) | 0x0001_0000
        );
        assert_eq!(e.mem.u32(out + 0x14), e.mem.u32(source + 4));
        assert_eq!(e.mem.u32(out + 0x20), e.mem.u32(source + 0x10));
        assert_eq!(e.mem.u32(out + 0x3c), 0x9090);
        assert_eq!(e.mem.u32(p), 5);
        assert_eq!(e.mem.u32(out), 0);
        assert_eq!(e.mem.u8(out + 0x12), 1);
    }

    #[test]
    fn kind_8_without_a_width_continues_with_a_kind_8_or_a_kind_9_record() {
        let mut e = Engine::new();
        // `[source + 0x28] + 1 < [source + 0x24]`: another kind-8 record, one step further.
        let (source, p, out, _) = kind_eight(&mut e, 0);
        e.mem.set_u32(source + 0x28, 3);
        e.mem.set_u32(source + 0x24, 10);
        e.mem.set_u32(p + 0x78, 0x66);
        e.mem.set_u16(p + 0x7c, 0x44);
        assert_eq!(
            e.call(0x00d9_69e0, &args![0u32, 0u32, source, out]).u32(),
            0
        );
        assert_eq!(e.mem.u32(out + 0x38), 4);
        assert_eq!(e.mem.u32(out + 0x3c), 0x66);
        assert_eq!(e.mem.u32(p), 0x44);
        // Otherwise a kind-9 record.
        let (source, p, out, _) = kind_eight(&mut e, 0);
        e.mem.set_u32(source + 0x28, 9);
        e.mem.set_u32(source + 0x24, 10);
        e.mem.set_u32(p + 0x30, 0x31);
        e.mem.set_u32(p + 0x34, 0x32);
        e.mem.set_u16(p + 0x3a, 0x33);
        assert_eq!(
            e.call(0x00d9_69e0, &args![0u32, 0u32, source, out]).u32(),
            0
        );
        assert_eq!(header(&e, out), (9, 0, 1, 0x30, 0xffff));
        assert_eq!(e.mem.u32(out + 0x30), 0x31);
        assert_eq!(e.mem.u32(out + 0x34), 0x32);
        assert_eq!(e.mem.u32(out + 0x38), e.mem.u32(source + 0x20));
        assert_eq!(e.mem.u8(out + 0x3c), 0);
        assert_eq!(e.mem.u32(p), 0x33);
        // Not the last count: the answer is 1 and nothing is written.
        let (source, p, out, _) = kind_eight(&mut e, 0);
        e.mem.set_u32(p, 4);
        assert_eq!(
            e.call(0x00d9_69e0, &args![0u32, 0u32, source, out]).u32(),
            1
        );
        assert_eq!(e.mem.u32(p), 3);
        assert_eq!(e.mem.u32(out), 0x5555);
    }

    #[test]
    fn kind_9_makes_a_kind_8_or_a_kind_11_record() {
        let mut e = Engine::new();
        let (source, p, out) = job(&mut e, 9, 1);
        e.mem.set_u8(source + 0x2c, 0);
        e.mem.set_u32(source + 0x28, 6);
        e.mem.set_u32(p + 0x78, 0x71);
        e.mem.set_u16(p + 0x7c, 0x72);
        assert_eq!(
            e.call(0x00d9_69e0, &args![0u32, 0u32, source, out]).u32(),
            0
        );
        assert_eq!(header(&e, out), (8, 0, 1, 0x30, 0xffff));
        assert_eq!(e.mem.u32(out + 0x3c), 0x71);
        assert_eq!(e.mem.u32(out + 0x30), 7);
        assert_eq!(e.mem.u32(out + 0x38), 0);
        assert_eq!(e.mem.u32(p), 0x72);
        let (source, p, out) = job(&mut e, 9, 1);
        e.mem.set_u8(source + 0x2c, 1);
        e.mem.set_u32(p + 0x78, 0x73);
        e.mem.set_u16(p + 0x54, 0x74);
        e.mem.set_u32(p + 0x14, 0x75);
        e.mem.set_u32(p + 0x6c, 0x76);
        assert_eq!(
            e.call(0x00d9_69e0, &args![0u32, 0u32, source, out]).u32(),
            0
        );
        assert_eq!(header(&e, out), (0xb, 0, 1, 0x40, 0xffff));
        assert_eq!(e.mem.u32(out + 0x30), 0x73);
        assert_eq!(e.mem.u16(out + 0x46), 0);
        assert_eq!(e.mem.u16(out + 0x44), 0x74);
        assert_eq!(e.mem.u32(out + 0x40), 0x75);
        assert_eq!(e.mem.u8(out + 0x4c), 0);
        assert_eq!(e.mem.u32(p), 0x76);
    }

    #[test]
    fn kind_10_relinks_a_node_and_raises_the_flag_at_zero() {
        let mut e = Engine::new();
        let (source, p, out) = job(&mut e, 10, 1);
        let holder = e.mem.alloc(0x20);
        let node = e.mem.alloc(0x10);
        e.mem.set_u32(node, 0x1);
        e.mem.set_u32(holder + 0xc, node);
        e.mem.set_u32(source + 0x20, holder);
        e.mem.set_u32(p + 0x24, 0x88);
        assert_eq!(
            e.call(0x00d9_69e0, &args![0u32, 0u32, source, out]).u32(),
            1
        );
        assert_eq!(e.mem.u32(node + 4), 0x88);
        assert_eq!(e.mem.u32(p + 0x24), node);
        assert_eq!(e.mem.u32(p), 0);
        assert_eq!(e.mem.u8(p + 0x20), 1);
    }

    #[test]
    fn kind_13_counts_the_node_down_to_a_kind_14_record() {
        let mut e = Engine::new();
        let (source, _, out) = job(&mut e, 13, 1);
        e.mem.set_u32(source + 0x20, 0);
        assert_eq!(
            e.call(0x00d9_69e0, &args![0u32, 0u32, source, out]).u32(),
            1
        );
        let node = e.mem.alloc(0x20);
        e.mem.set_u32(source + 0x20, node);
        e.mem.set_u32(node + 4, 2);
        assert_eq!(
            e.call(0x00d9_69e0, &args![0u32, 0u32, source, out]).u32(),
            1
        );
        assert_eq!(e.mem.u32(node + 4), 1);
        assert_eq!(
            e.call(0x00d9_69e0, &args![0u32, 0u32, source, out]).u32(),
            0
        );
        assert_eq!(header(&e, out), (0xe, 0, 2, 0x30, 0xffff));
        assert_eq!(e.mem.u32(out + 0x30), node);
        assert_eq!(e.mem.u32(out), 0);
    }

    #[test]
    fn the_queue_helper_builds_the_kind_5_record() {
        let mut e = Engine::new();
        e.map(0, 0x1000);
        let seen = Rc::new(RefCell::new(Vec::new()));
        let log = Rc::clone(&seen);
        e.register_double(ADD_JOB_QUEUE_LOCKED, move |e, a| {
            let words: Vec<u32> = (0..8).map(|i| e.mem.u32(a[2] + 4 * i)).collect();
            log.borrow_mut().push((
                a.to_vec(),
                words,
                e.mem.u8(a[2]),
                e.mem.u8(a[2] + 1),
                e.mem.u8(a[2] + 2),
            ));
            Ret::default()
        });
        let source = e.mem.alloc(0x40);
        e.mem.set_u16(source + 0x10, 0x1111);
        e.mem.set_u32(source + 0x14, 0x14);
        e.mem.set_u32(source + 0x18, 0x18);
        e.mem.set_u32(source + 0x1c, 0x1c);
        let ecx = e.mem.alloc(0x40);
        e.mem.set_u8(ecx + 0x21, 1);
        e.call(0x00d9_6970, &args![ecx, source, 0x31u32, 0x32u32]);
        e.mem.set_u8(ecx + 0x21, 0);
        e.call(0x00d9_6970, &args![ecx, source, 0x33u32, 0x34u32]);
        let seen = seen.borrow();
        assert_eq!(seen.len(), 1);
        let (arguments, words, b0, b1, b2) = &seen[0];
        assert_eq!((arguments[0], arguments[1], arguments[3]), (0x31, 0x32, 0));
        assert_eq!((*b0, *b1, *b2), (5, 0, 2));
        assert_eq!(words[1] & 0xffff, 0x20);
        assert_eq!(words[1] >> 16, 0xffff);
        assert_eq!(words[4] & 0xffff, 0x1111);
        assert_eq!(words[5], 0x14);
        assert_eq!(words[6], 0x18);
        assert_eq!(words[7], 0x1c);
    }

    #[test]
    fn the_constructor_fills_the_header_and_runs_the_base_init() {
        let mut e = Engine::new();
        let calls = record(&mut e, &[BASE_INIT]);
        let this = e.mem.alloc(0x20);
        assert_eq!(e.call(0x00d9_7010, &args![this]).u32(), this);
        assert_eq!(e.mem.u16(this + 6), 1);
        assert_eq!(e.mem.u32(this), VTABLE_00D97010);
        assert_eq!(e.mem.u32(this + 8), 0xffff_fff1);
        assert_eq!(e.mem.u16(this + 0xc), 0x8000);
        assert_eq!(e.mem.u32(this + 0x10), 0);
        assert_eq!(*calls.borrow(), [(BASE_INIT, vec![this + 8])]);
    }
}

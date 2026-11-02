//! `Havok SDK/run_00df7760` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

use super::run_00df06a0::{movaps, mulps, store_xmm, subps, sum3};

/// The float at `0104da48`: the start value of the second field of the two result records.
const FIRST_FLOAT: u32 = 0x0104_da48;
/// The float at `0104da4c`: the start value of the first field of the third record.
const SECOND_FLOAT: u32 = 0x0104_da4c;
/// The constant stored in the first word of the two result records (`0108f288`).
const RECORD_VTABLE: u32 = 0x0108_f288;
/// `InterlockedExchangeAdd` import slot (stdcall: the counter, the addend; answers the old value).
const INTERLOCKED_EXCHANGE_ADD: u32 = 0x00fd_f1ec;
/// Releases a semaphore (`00d043c0`, cdecl `(semaphore, count)`).
const SEMAPHORE_RELEASE: u32 = 0x00d0_43c0;

/// The loop the exe repeats twice: `chunks` 16-byte pieces from `from` to `to`, a dword at a time.
fn copy_chunks(e: &mut Engine, mut from: u32, mut to: u32, mut chunks: i32) {
    while chunks > 0 {
        for lane in 0..4u32 {
            let word = e.mem.u32(from.wrapping_add(4 * lane));
            e.mem.set_u32(to.wrapping_add(4 * lane), word);
        }
        chunks -= 1;
        from = from.wrapping_add(0x10);
        to = to.wrapping_add(0x10);
    }
}

// Translated from 00df7760 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (it runs the dispatcher table of `a2` over a list of 0x40-byte
/// segment records, the swept relative of `00df6e00`; class unconfirmed). Cdecl, four words:
/// `a1` (floats at `+0x20`, `+0x24`, `+0x28` and the word at `+0x2c` are copied into the frame
/// records), `a2` (its first word is the dispatcher table, the word at `+0xc` is copied), `a3`
/// (`count` records) and `count`.
///
/// A record is two vectors `from` at `+0` and `to` at `+0x10`, the two bodies at `+0x20` and
/// `+0x24`, an output buffer at `+0x28`, a word at `+0x2c`, the first result count stored at `+0x30`,
/// a second output buffer at `+0x34`, a word at `+0x38` and the second result count stored at
/// `+0x3c`. For each it writes `to - from` at frame `+0xb0` and its length (`SQRTSS` of the lane
/// sum) at `+0xc4`, fills three argument records of the 0xec4-byte frame (at `+0x1c`, `+0x34` and
/// `+0x60`), picks the handler from the table at `*a2 + 0x9ac` (20-byte stride) by the byte at
/// `*a2 + ((type(a) + 0xd) << 5) + type(b)` (the type is the word at `+0xc` of the first word of
/// each body) and calls it (cdecl, five words: both bodies, the record at `+0x60`, the record at
/// `+0x1c` and the record at `+0x34` when the second output buffer is set, else 0). The 0x70-byte
/// results (16-bit counts at `+0x30` and `+0x48`) are copied from the buffers at `+0xd0` and
/// `+0x7d0` to the output buffers (the second only when its pointer is set).
pub fn fn_00df7760(e: &mut Engine, a1: Ptr, a2: Ptr, a3: Ptr, count: i32) {
    e.with_stack(0xed0, |e, frame| {
        let b = frame.addr();
        let at = |offset: u32| b.wrapping_add(offset);
        let mut eps = x87_quiet_bits(e.mem.u32(FIRST_FLOAT));
        e.mem.set_u32(at(0x20), eps);
        e.mem.set_u32(at(0x38), eps);
        e.mem.set_u32(at(0x28), at(0xd0));
        e.mem.set_u32(at(0x1c), RECORD_VTABLE);
        e.mem.set_u32(at(0x34), RECORD_VTABLE);
        e.mem.set_u32(at(0x2c), 0x10);
        e.mem.set_u32(at(0x44), 0x10);
        e.mem.set_u32(at(0x3c), at(0x7d0));
        e.mem.set_u32(at(0x40), at(0x7d0));
        let word = e.mem.u32(a1.addr().wrapping_add(0x2c));
        let float = x87_quiet_bits(e.mem.u32(a1.addr().wrapping_add(0x28)));
        e.mem.set_u32(at(0x54), word);
        e.mem.set_u32(at(0x50), float);
        let float = x87_quiet_bits(e.mem.u32(SECOND_FLOAT));
        e.mem.set_u32(at(0xc0), float);
        let float = x87_quiet_bits(e.mem.u32(a1.addr().wrapping_add(0x20)));
        let dispatcher = e.mem.u32(a2.addr());
        e.mem.set_u32(at(0x68), float);
        let word = e.mem.u32(a2.addr().wrapping_add(0xc));
        e.mem.set_u32(at(0x6c), word);
        e.mem.set_u32(at(0x24), at(0xd0));
        e.mem.set_u32(at(0xc8), at(0x50));
        e.mem.set_u32(at(0x30), 0);
        e.mem.set_u32(at(0x48), 0);
        e.mem.set_u32(at(0x64), 0);
        e.mem.set_u32(at(0x60), dispatcher);
        if count <= 0 {
            return;
        }
        let mut rec = a3.addr().wrapping_add(0x2c);
        e.mem.set_u32(at(0x18), count as u32);
        let mut first_pass = true;
        loop {
            if !first_pass {
                eps = x87_quiet_bits(e.mem.u32(FIRST_FLOAT));
            }
            first_pass = false;
            let body_a = e.mem.u32(rec.wrapping_sub(0xc));
            let shape_a = e.mem.u32(body_a);
            let body_b = e.mem.u32(rec.wrapping_sub(8));
            let shape_b = e.mem.u32(body_b);
            let from = movaps(e, rec.wrapping_sub(0x2c));
            let to = movaps(e, rec.wrapping_sub(0x1c));
            let delta = subps(to, from);
            store_xmm(e, at(0xb0), delta);
            let length = sum3(mulps(delta, delta)).sqrt();
            e.mem.set_f32(at(0x4c), length);
            e.mem.set_u32(at(0xc4), x87_quiet_bits(length.to_bits()));
            e.mem.set_u32(at(0x1c), RECORD_VTABLE);
            let float = x87_quiet_bits(e.mem.u32(a1.addr().wrapping_add(0x20)));
            e.mem.set_u32(at(0x30), 0);
            e.mem.set_u32(at(0x68), float);
            let float = x87_quiet_bits(e.mem.u32(a1.addr().wrapping_add(0x24)));
            e.mem.set_u32(at(0x24), at(0xd0));
            e.mem.set_u32(at(0xc0), float);
            e.mem.set_u32(at(0x28), at(0xd0));
            let word = e.mem.u32(rec);
            e.mem.set_u32(at(0x20), eps);
            e.mem.set_u32(at(0x2c), word);
            let second_output = e.mem.u32(rec.wrapping_add(8));
            if second_output != 0 {
                e.mem.set_u32(at(0x38), eps);
                e.mem.set_u32(at(0x3c), at(0x7d0));
                e.mem.set_u32(at(0x40), at(0x7d0));
                let word = e.mem.u32(rec.wrapping_add(0xc));
                e.mem.set_u32(at(0x34), RECORD_VTABLE);
                e.mem.set_u32(at(0x44), word);
                e.mem.set_u32(at(0x48), 0);
            }
            let row = e.mem.u32(shape_a.wrapping_add(0xc)).wrapping_add(0xd) << 5;
            let dispatcher = e.mem.u32(a2.addr());
            let row = row.wrapping_add(e.mem.u32(shape_b.wrapping_add(0xc)));
            let kind = u32::from(e.mem.u8(row.wrapping_add(dispatcher)));
            let extra = if second_output != 0 { at(0x34) } else { 0 };
            let handler = e.mem.u32(
                dispatcher
                    .wrapping_add(kind.wrapping_mul(5).wrapping_mul(4))
                    .wrapping_add(0x9ac),
            );
            e.call(handler, &args![body_a, body_b, at(0x60), at(0x1c), extra]);
            let first_count = u32::from(e.mem.u16(at(0x30)));
            let second_count = u32::from(e.mem.u16(at(0x48)));
            e.mem.set_u32(rec.wrapping_add(4), first_count);
            e.mem.set_u32(rec.wrapping_add(0x10), second_count);
            if first_count as i32 > 0 {
                let to = e.mem.u32(rec.wrapping_sub(4));
                copy_chunks(e, at(0xd0), to, first_count.wrapping_mul(7) as i32);
            }
            let second_output = e.mem.u32(rec.wrapping_add(8));
            if second_output != 0 {
                let second_count = e.mem.u32(rec.wrapping_add(0x10));
                if second_count as i32 > 0 {
                    copy_chunks(
                        e,
                        at(0x7d0),
                        second_output,
                        second_count.wrapping_mul(7) as i32,
                    );
                }
            }
            rec = rec.wrapping_add(0x40);
            let remaining = e.mem.u32(at(0x18)).wrapping_sub(1);
            e.mem.set_u32(at(0x18), remaining);
            if remaining == 0 {
                break;
            }
        }
    })
}

// Translated from 00df7a60 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (class unconfirmed). Thiscall, no stack words: adds 1 to the counter
/// at `*(this + 0x1c)` with `InterlockedExchangeAdd` when that pointer is set, then releases the
/// semaphore at `*(this + 0x10)` once (`00d043c0`, `(semaphore, 1)`) when that pointer is set.
/// Returns `EAX` as the exe leaves it: the last call's result, else the counter pointer (0).
pub fn fn_00df7a60(e: &mut Engine, this: Ptr) -> u32 {
    let mut eax = e.mem.u32(this.addr().wrapping_add(0x1c));
    if eax != 0 {
        eax = e.call(INTERLOCKED_EXCHANGE_ADD, &args![eax, 1u32]).u32();
    }
    let semaphore = e.mem.u32(this.addr().wrapping_add(0x10));
    if semaphore != 0 {
        eax = e.call(SEMAPHORE_RELEASE, &args![semaphore, 1u32]).u32();
    }
    eax
}

// Translated from 00df7d00 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (class unconfirmed). Cdecl, two words `source` and `dest`: copies the
/// dwords at `+0x14`, `+0x1c`, `+0x10`, `+0x20`, `+0x24`, `+0x38`, `+0x28`, `+0x2c`, `+0x30`,
/// `+0x3c`, `+0x34` and `+0x40` from `source` to `dest`, in that order.
pub fn fn_00df7d00(e: &mut Engine, source: Ptr, dest: Ptr) {
    for offset in [
        0x14u32, 0x1c, 0x10, 0x20, 0x24, 0x38, 0x28, 0x2c, 0x30, 0x3c, 0x34, 0x40,
    ] {
        let word = e.mem.u32(source.addr().wrapping_add(offset));
        e.mem.set_u32(dest.addr().wrapping_add(offset), word);
    }
}

// Translated from 00df79d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (cdecl, two words `queue`, `job`; a job function: `00da3400` stores its address
/// at `00da3429`; descriptions, not confirmed). Between the profiling records `010dba08` and
/// `010c41f8` it runs `00df7760` (this unit) with `(job, [job + 0x18], [job + 0x34], [job +
/// 0x38])`, then asks the job queue for the next job: `00d04e80` (`thiscall` on `queue`) with
/// `(job, job, 0)`. Answers what that answers (0 to go on with the next job).
pub fn fn_00df79d0(e: &mut Engine, queue: Ptr, job: Ptr) -> u32 {
    let at = job.addr();
    crate::units::havok_sdk::hkadeltacompressedanimation::profile_record(e, 0x010d_ba08);
    let first = e.mem.u32(at.wrapping_add(0x18));
    let second = e.mem.u32(at.wrapping_add(0x34));
    let count = e.mem.u32(at.wrapping_add(0x38));
    fn_00df7760(e, job, Ptr::new(first), Ptr::new(second), count as i32);
    crate::units::havok_sdk::hkadeltacompressedanimation::profile_record(e, 0x010c_41f8);
    e.call(0x00d0_4e80, &args![queue, job, job, 0u32]).u32()
}

// Translated from 00df7c00 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (cdecl, two words `queue`, `job`; a job function: `00da3480` stores its address
/// at `00da34ef`; descriptions, not confirmed). Between the profiling records `010dba24` and
/// `010c41f8` it clears the words at `job + 0x48` and `job + 0x50`, then calls one of two
/// functions with the job's words and the addresses of those two (the floats at `+0x38` and
/// `+0x3c` go through `FLD`/`FSTP`, a signalling NaN quieted). When the word at `job + 0x20` is
/// not zero: `00e29cd0` with `([+0x20], [+0x28], [+0x2c], [+0x30], [+0x34], float +0x38, float
/// +0x3c, [+0x40], [+0x44], job + 0x48, [+0x4c], job + 0x50)`. Otherwise `00e2a010` with `([+0x24],
/// [+0x2c], [+0x30], [+0x34], float +0x38, float +0x3c, [+0x40], [+0x44], job + 0x48, [+0x4c],
/// job + 0x50)`. Then, as `00df79d0` does, `00d04e80` on `queue` with `(job, job, 0)`; answers
/// what it answers.
pub fn fn_00df7c00(e: &mut Engine, queue: Ptr, job: Ptr) -> u32 {
    let at = job.addr();
    let word = |e: &Engine, offset: u32| e.mem.u32(at.wrapping_add(offset));
    crate::units::havok_sdk::hkadeltacompressedanimation::profile_record(e, 0x010d_ba24);
    let selector = word(e, 0x20);
    let third_float = x87_quiet_bits(word(e, 0x3c));
    e.mem.set_u32(at.wrapping_add(0x48), 0);
    e.mem.set_u32(at.wrapping_add(0x50), 0);
    let second_float = x87_quiet_bits(word(e, 0x38));
    if selector != 0 {
        let words = [
            selector,
            word(e, 0x28),
            word(e, 0x2c),
            word(e, 0x30),
            word(e, 0x34),
            second_float,
            third_float,
            word(e, 0x40),
            word(e, 0x44),
            at.wrapping_add(0x48),
            word(e, 0x4c),
            at.wrapping_add(0x50),
        ];
        e.call(0x00e2_9cd0, &words);
    } else {
        let words = [
            word(e, 0x24),
            word(e, 0x2c),
            word(e, 0x30),
            word(e, 0x34),
            second_float,
            third_float,
            word(e, 0x40),
            word(e, 0x44),
            at.wrapping_add(0x48),
            word(e, 0x4c),
            at.wrapping_add(0x50),
        ];
        e.call(0x00e2_a010, &words);
    }
    crate::units::havok_sdk::hkadeltacompressedanimation::profile_record(e, 0x010c_41f8);
    e.call(0x00d0_4e80, &args![queue, job, job, 0u32]).u32()
}

// Translated from 00df7d60 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (cdecl, two words `queue`, `job`; a job function: `00da3480` stores its address
/// at `00da34bd`; descriptions, not confirmed). Between the profiling records `010dba3c` and
/// `010c41f8` it builds a 16-byte block on the stack (word 0 cleared, `[job + 0x38]` at `+8`,
/// `[job + 0x3c]` at `+0xc`) and runs `00d9d880` (cdecl, seven words) with `(block + 8, [+0x24],
/// [+0x28], [+0x30], [+0x20], [+0x34], block)`; the word the block's first word holds afterwards is
/// stored where `[job + 0x2c]` points. Then, as `00df79d0` does, `00d04e80` on `queue` with
/// `(job, job, 0)`; answers what it answers.
pub fn fn_00df7d60(e: &mut Engine, queue: Ptr, job: Ptr) -> u32 {
    let at = job.addr();
    crate::units::havok_sdk::hkadeltacompressedanimation::profile_record(e, 0x010d_ba3c);
    e.with_stack(0x10, |e, block| {
        let block = block.addr();
        let first = e.mem.u32(at.wrapping_add(0x38));
        let second = e.mem.u32(at.wrapping_add(0x3c));
        e.mem.set_u32(block.wrapping_add(8), first);
        e.mem.set_u32(block.wrapping_add(0xc), second);
        e.mem.set_u32(block, 0);
        let words = [
            block.wrapping_add(8),
            e.mem.u32(at.wrapping_add(0x24)),
            e.mem.u32(at.wrapping_add(0x28)),
            e.mem.u32(at.wrapping_add(0x30)),
            e.mem.u32(at.wrapping_add(0x20)),
            e.mem.u32(at.wrapping_add(0x34)),
            block,
        ];
        e.call(0x00d9_d880, &words);
        let target = e.mem.u32(at.wrapping_add(0x2c));
        let result = e.mem.u32(block);
        e.mem.set_u32(target, result);
    });
    crate::units::havok_sdk::hkadeltacompressedanimation::profile_record(e, 0x010c_41f8);
    e.call(0x00d0_4e80, &args![queue, job, job, 0u32]).u32()
}

// Translated from 00df7a90 (disassembled, FalloutNV.exe 1.4.0.525)
/// Unnamed (Ghidra made no function; 00df7a90 to the `RET` at 00df7ab5, its jump table after
/// it): the job-finished function `00da3400` registers for job type 2, `cdecl` with three words
/// (only the third, the job, is read). For job kinds 0 to 5 (byte `+0`; the jump table at
/// `00df7ab8` sends all six to the same code) the counter at `*(job + 0x14)` drops by one and at
/// 0 [`fn_00df7a60`] runs on the job. Answers 1.
pub fn fn_00df7a90(e: &mut Engine, _unused_1: u32, _unused_2: u32, job: Ptr) -> u32 {
    let kind = e.mem.u8(job.addr());
    let counter = e.mem.u32(job.addr().wrapping_add(0x14));
    if kind <= 5 {
        let left = e.mem.u32(counter).wrapping_sub(1);
        e.mem.set_u32(counter, left);
        if left == 0 {
            e.call(0x00df_7a60, &args![job]);
        }
    }
    1
}

/// One split case of `00df7ad0`: the job's item count word, its remaining total word, its item
/// pointer word and the item size (a shift or a multiply in the exe).
type SplitCase = (u32, u32, u32, u32);

/// `00df7ad0`'s jump table at `00df7be0` (read from the exe) as the cases its five targets run:
/// 0 (`00df7b71`), 1 (`00df7b95`), 2 (`00df7bb6`), 3 (`00df7b24`), 4 (`00df7b49`).
const SPLIT_CASES_00DF7BE0: [SplitCase; 5] = [
    (0x34, 0x3c, 0x38, 0x30),
    (0x24, 0x2c, 0x28, 0x20),
    (0x30, 0x38, 0x34, 0x10),
    (0x30, 0x38, 0x34, 0x40),
    (0x20, 0x28, 0x24, 0x30),
];

// Translated from 00df7ad0 (disassembled, FalloutNV.exe 1.4.0.525)
/// Unnamed (Ghidra made no function; 00df7ad0 to the `RET` at 00df7bdd, its jump table after it):
/// the job-split function `00da3400` registers for job type 2, `cdecl` with four words (the
/// third the job, the fourth the new job; the first two not read). Copies the job's 0x80 bytes to
/// the new job (word by word, in order), then by the job's kind (byte `+0`, 0 to 4): when the
/// remaining total is above the item count (signed), the new job takes the count as its total,
/// the job's total drops by the count and its item pointer moves past `count` items, and the
/// answer is 1; otherwise (or for another kind) 0.
pub fn fn_00df7ad0(e: &mut Engine, _unused_1: u32, _unused_2: u32, job: Ptr, split: Ptr) -> u32 {
    let job = job.addr();
    let split = split.addr();
    for offset in (0..0x80u32).step_by(4) {
        let value = e.mem.u32(job.wrapping_add(offset));
        e.mem.set_u32(split.wrapping_add(offset), value);
    }
    let kind = e.mem.u8(job);
    if kind > 4 {
        return 0;
    }
    let (count, total, items, size) = SPLIT_CASES_00DF7BE0[usize::from(kind)];
    let first = e.mem.u32(job.wrapping_add(count));
    if (e.mem.u32(job.wrapping_add(total)) as i32) <= first as i32 {
        return 0;
    }
    e.mem.set_u32(split.wrapping_add(total), first);
    let count_now = e.mem.u32(job.wrapping_add(count));
    let left = e.mem.u32(job.wrapping_add(total)).wrapping_sub(count_now);
    e.mem.set_u32(job.wrapping_add(total), left);
    let moved = e
        .mem
        .u32(job.wrapping_add(items))
        .wrapping_add(count_now.wrapping_mul(size));
    e.mem.set_u32(job.wrapping_add(items), moved);
    1
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00df79d0, fn_00df79d0(Ptr, Ptr) -> u32),
        entry!(0x00df7c00, fn_00df7c00(Ptr, Ptr) -> u32),
        entry!(0x00df7d60, fn_00df7d60(Ptr, Ptr) -> u32),
        entry!(0x00df_7760, fn_00df7760(Ptr, Ptr, Ptr, i32)),
        entry!(0x00df_7a60, fn_00df7a60(Ptr) -> u32),
        entry!(0x00df_7a90, fn_00df7a90(u32, u32, Ptr) -> u32),
        entry!(0x00df_7ad0, fn_00df7ad0(u32, u32, Ptr, Ptr) -> u32),
        entry!(0x00df_7d00, fn_00df7d00(Ptr, Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    /// The thread's record stream (TLS index 3) for the profiling records and a double for the
    /// job queue's next-job function that answers 7. Returns the stream and the records.
    fn profiled_engine_00df79d0() -> (Engine, u32, u32) {
        let mut e = Engine::new();
        e.map(0x0126_8000, 0x1000);
        e.set_global(0x0126_835c, 3u32);
        let stream = e.mem.alloc(0x20);
        let records = e.mem.alloc(0x40);
        e.mem.set_u32(stream + 4, records);
        e.mem.set_u32(stream + 0xc, records + 0x40);
        e.register_double(0x00fd_f044, move |_, _| Ret {
            eax: stream,
            ..Ret::default()
        });
        e.register(0x00d0_4e80, |_, _| Ret {
            eax: 7,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        (e, stream, records)
    }

    fn labels_00df79d0(e: &Engine, stream: u32, records: u32) -> Vec<u32> {
        let end = e.mem.u32(stream + 4);
        (records..end)
            .step_by(12)
            .map(|record| e.mem.u32(record))
            .collect()
    }

    fn logged_00df79d0(e: &Engine, address: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(called, _)| *called == address)
            .map(|(_, words)| words.clone())
            .collect()
    }

    /// A job whose word at each offset `0x20 + 4 * n` is `0x100 + n`, with the floats at `+0x38`
    /// and `+0x3c` given.
    fn job_00df79d0(e: &mut Engine, floats: (u32, u32)) -> u32 {
        let job = e.mem.alloc(0x100);
        for n in 0..16u32 {
            e.mem.set_u32(job + 0x20 + 4 * n, 0x100 + n);
        }
        e.mem.set_u32(job + 0x38, floats.0);
        e.mem.set_u32(job + 0x3c, floats.1);
        job
    }

    #[test]
    fn fn_00df79d0_runs_the_sweep_between_two_records_and_asks_for_the_next_job() {
        let (mut e, stream, records) = profiled_engine_00df79d0();
        e.map(0x0104_d000, 0x1000);
        let job = job_00df79d0(&mut e, (0, 0));
        // `00df7760` reads the two objects the job points to; with a count of 0 it stops there.
        let first = e.mem.alloc(0x20);
        let second = e.mem.alloc(0x20);
        e.mem.set_u32(job + 0x18, first);
        e.mem.set_u32(job + 0x34, second);
        let answer = e.call(0x00df_79d0, &args![0x1234u32, job]).u32();
        assert_eq!(answer, 7);
        assert_eq!(
            labels_00df79d0(&e, stream, records),
            vec![0x010d_ba08, 0x010c_41f8]
        );
        assert_eq!(
            logged_00df79d0(&e, 0x00d0_4e80),
            vec![vec![0x1234, job, job, 0]]
        );
    }

    #[test]
    fn fn_00df7c00_with_a_selector_calls_00e29cd0_with_twelve_words() {
        let (mut e, stream, records) = profiled_engine_00df79d0();
        e.register(0x00e2_9cd0, |_, _| Ret::default());
        e.register(0x00e2_a010, |_, _| Ret::default());
        let job = job_00df79d0(&mut e, (0x3f80_0000, 0x7fa0_0001));
        e.mem.set_u32(job + 0x48, 0x99);
        e.mem.set_u32(job + 0x50, 0x98);
        let answer = e.call(0x00df_7c00, &args![0x1234u32, job]).u32();
        assert_eq!(answer, 7);
        assert_eq!(e.mem.u32(job + 0x48), 0);
        assert_eq!(e.mem.u32(job + 0x50), 0);
        assert_eq!(
            logged_00df79d0(&e, 0x00e2_9cd0),
            vec![vec![
                0x100,
                0x102,
                0x103,
                0x104,
                0x105,
                0x3f80_0000,
                0x7fe0_0001,
                0x108,
                0x109,
                job + 0x48,
                0x10b,
                job + 0x50,
            ]]
        );
        assert!(logged_00df79d0(&e, 0x00e2_a010).is_empty());
        assert_eq!(
            labels_00df79d0(&e, stream, records),
            vec![0x010d_ba24, 0x010c_41f8]
        );
    }

    #[test]
    fn fn_00df7c00_without_a_selector_calls_00e2a010_with_eleven_words() {
        let (mut e, _, _) = profiled_engine_00df79d0();
        e.register(0x00e2_9cd0, |_, _| Ret::default());
        e.register(0x00e2_a010, |_, _| Ret::default());
        let job = job_00df79d0(&mut e, (0x3f80_0000, 0x4000_0000));
        e.mem.set_u32(job + 0x20, 0);
        e.call(0x00df_7c00, &args![0x1234u32, job]);
        assert_eq!(
            logged_00df79d0(&e, 0x00e2_a010),
            vec![vec![
                0x101,
                0x103,
                0x104,
                0x105,
                0x3f80_0000,
                0x4000_0000,
                0x108,
                0x109,
                job + 0x48,
                0x10b,
                job + 0x50,
            ]]
        );
        assert!(logged_00df79d0(&e, 0x00e2_9cd0).is_empty());
    }

    #[test]
    fn fn_00df7d60_stores_the_helpers_first_output_where_the_job_says() {
        let (mut e, stream, records) = profiled_engine_00df79d0();
        let job = job_00df79d0(&mut e, (0x38, 0x3c));
        let target = e.mem.alloc(8);
        e.mem.set_u32(job + 0x2c, target);
        // The helper sees the block: its words at +8 and +0xc, and fills its first word.
        e.register_double(0x00d9_d880, |e, a| {
            assert_eq!(a[0], a[6] + 8);
            assert_eq!(e.mem.u32(a[0]), 0x38);
            assert_eq!(e.mem.u32(a[0] + 4), 0x3c);
            assert_eq!(e.mem.u32(a[6]), 0);
            e.mem.set_u32(a[6], 0xcafe);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        let answer = e.call(0x00df_7d60, &args![0x1234u32, job]).u32();
        assert_eq!(answer, 7);
        assert_eq!(e.mem.u32(target), 0xcafe);
        let helper = logged_00df79d0(&e, 0x00d9_d880);
        assert_eq!(helper.len(), 1);
        assert_eq!(
            [
                helper[0][1],
                helper[0][2],
                helper[0][3],
                helper[0][4],
                helper[0][5]
            ],
            [0x101, 0x102, 0x104, 0x100, 0x105]
        );
        assert_eq!(
            labels_00df79d0(&e, stream, records),
            vec![0x010d_ba3c, 0x010c_41f8]
        );
        assert_eq!(
            logged_00df79d0(&e, 0x00d0_4e80),
            vec![vec![0x1234, job, job, 0]]
        );
    }

    type CallLog = Rc<RefCell<Vec<Vec<u32>>>>;

    const HANDLER: u32 = 0x0a00_0200;

    struct Rig {
        e: Engine,
        a1: u32,
        a2: u32,
        list: u32,
        dispatcher: u32,
        bodies: [u32; 2],
        log: CallLog,
    }

    /// A dispatcher table whose byte for the (1, 2) type pair picks the handler at index 1
    /// (table offset `0x9ac + 20`); each call writes `first` results into the buffer at record
    /// `+0x24` of its first record and `second` into the second, and reports both counts.
    fn rig(first: u16, second: u16) -> Rig {
        let mut e = Engine::new();
        let a1 = e.mem.alloc(0x40);
        let a2 = e.mem.alloc(0x20);
        let dispatcher = e.mem.alloc(0x1000);
        let shape_a = e.mem.alloc(0x20);
        let shape_b = e.mem.alloc(0x20);
        let body_a = e.mem.alloc(0x10);
        let body_b = e.mem.alloc(0x10);
        let list = e.mem.alloc(0x40 * 3);
        e.map(0x0100_0000, 0x0030_0000);
        e.mem.set_u32(0x0104_da48, 0x3f80_0000);
        e.mem.set_u32(0x0104_da4c, 0x7f80_0001);
        for (lane, value) in [1.0f32, 2.0, 3.0, 0.0].iter().enumerate() {
            e.mem.set_f32(a1 + 0x20 + 4 * lane as u32, *value);
        }
        e.mem.set_u32(a2, dispatcher);
        e.mem.set_u32(a2 + 0xc, 0xcafe);
        e.mem.set_u32(body_a, shape_a);
        e.mem.set_u32(body_b, shape_b);
        e.mem.set_u32(shape_a + 0xc, 1);
        e.mem.set_u32(shape_b + 0xc, 2);
        e.mem.set_u8(dispatcher + ((1 + 0xd) << 5) + 2, 1);
        e.mem.set_u32(dispatcher + 0x9ac + 20, HANDLER);
        let log: CallLog = Rc::default();
        {
            let log = log.clone();
            e.register_double(HANDLER, move |e, a| {
                let mut words = a.to_vec();
                for offset in [0u32, 4, 8, 0xc, 0x10, 0x14, 0x18] {
                    words.push(e.mem.u32(a[3] + offset));
                }
                for offset in [0u32, 8, 0xc, 0x50, 0x54, 0x58, 0x5c, 0x60, 0x64, 0x68] {
                    words.push(e.mem.u32(a[2] + offset));
                }
                log.borrow_mut().push(words);
                let low = e.mem.u32(a[3] + 0xc);
                for r in 0..u32::from(first) {
                    for k in 0..0x1cu32 {
                        e.mem
                            .set_u32(low + r * 0x70 + 4 * k, 0x1000 + k + (r << 16));
                    }
                }
                e.mem.set_u8(a[3] + 0x14, first as u8);
                if a[4] != 0 {
                    let high = e.mem.u32(a[4] + 0xc);
                    for r in 0..u32::from(second) {
                        for k in 0..0x1cu32 {
                            e.mem
                                .set_u32(high + r * 0x70 + 4 * k, 0x2000 + k + (r << 16));
                        }
                    }
                    e.mem.set_u8(a[4] + 0x14, second as u8);
                }
                Ret::default()
            });
        }
        Rig {
            e,
            a1,
            a2,
            list,
            dispatcher,
            bodies: [body_a, body_b],
            log,
        }
    }

    /// Fills record `index`: the segment from `(0, 0, 0)` to `(3, 4, 12)`.
    fn record(r: &mut Rig, index: u32, output: u32, second_output: u32) {
        let at = r.list + 0x40 * index;
        for (lane, value) in [0.0f32, 0.0, 0.0, 0.0].iter().enumerate() {
            r.e.mem.set_f32(at + 4 * lane as u32, *value);
        }
        for (lane, value) in [3.0f32, 4.0, 12.0, 0.0].iter().enumerate() {
            r.e.mem.set_f32(at + 0x10 + 4 * lane as u32, *value);
        }
        r.e.mem.set_u32(at + 0x20, r.bodies[0]);
        r.e.mem.set_u32(at + 0x24, r.bodies[1]);
        r.e.mem.set_u32(at + 0x28, output);
        r.e.mem.set_u32(at + 0x2c, 9);
        r.e.mem.set_u32(at + 0x30, 0xdead_beef);
        r.e.mem.set_u32(at + 0x34, second_output);
        r.e.mem.set_u32(at + 0x38, 11);
        r.e.mem.set_u32(at + 0x3c, 0xdead_beef);
    }

    #[test]
    fn the_handler_gets_the_records_and_both_result_sets_are_copied() {
        let mut r = rig(2, 1);
        let first = r.e.mem.alloc(0x400);
        let second = r.e.mem.alloc(0x400);
        record(&mut r, 0, first, second);
        r.e.call(0x00df_7760, &args![r.a1, r.a2, r.list, 1i32]);
        let log = r.log.borrow();
        assert_eq!(log.len(), 1);
        let call = &log[0];
        assert_eq!(call[0], r.bodies[0]);
        assert_eq!(call[1], r.bodies[1]);
        // The first record: vtable, the float, the buffer pointers, the word, a cleared count.
        assert_eq!(call[5], 0x0108_f288);
        assert_eq!(call[6], 1.0f32.to_bits());
        assert_eq!(call[7], call[8]);
        assert_eq!(call[7], call[2] + 0x70);
        assert_eq!(call[9], 9);
        assert_eq!(call[10], 0);
        assert_eq!(call[11], 0x0108_f288);
        // The record at +0x60: the dispatcher, the float, the word.
        assert_eq!(call[12], r.dispatcher);
        assert_eq!(call[13], 1.0f32.to_bits());
        assert_eq!(call[14], 0xcafe);
        // The fifth word is the second record, whose count was cleared and word set.
        assert_ne!(call[4], 0);
        for result in 0..2u32 {
            for k in 0..0x1cu32 {
                assert_eq!(
                    r.e.mem.u32(first + result * 0x70 + 4 * k),
                    0x1000 + k + (result << 16)
                );
            }
        }
        assert_eq!(r.e.mem.u32(first + 0xe0), 0);
        assert_eq!(r.e.mem.u32(second), 0x2000);
        assert_eq!(r.e.mem.u32(second + 0x70), 0);
        assert_eq!(r.e.mem.u32(r.list + 0x30), 2);
        assert_eq!(r.e.mem.u32(r.list + 0x3c), 1);
    }

    #[test]
    fn without_a_second_buffer_the_fifth_word_is_zero_and_nothing_is_copied_there() {
        let mut r = rig(1, 3);
        let first = r.e.mem.alloc(0x400);
        record(&mut r, 0, first, 0);
        r.e.call(0x00df_7760, &args![r.a1, r.a2, r.list, 1i32]);
        assert_eq!(r.log.borrow()[0][4], 0);
        assert_eq!(r.e.mem.u32(first), 0x1000);
        assert_eq!(r.e.mem.u32(r.list + 0x30), 1);
        assert_eq!(r.e.mem.u32(r.list + 0x3c), 0);
    }

    #[test]
    fn the_segment_vector_and_length_are_left_in_the_frame() {
        let mut r = rig(0, 0);
        let first = r.e.mem.alloc(0x100);
        record(&mut r, 0, first, 0);
        r.e.call(0x00df_7760, &args![r.a1, r.a2, r.list, 1i32]);
        let log = r.log.borrow();
        let call = &log[0];
        // The difference vector at +0xb0 and the length at +0xc4 sit 0x50 and 0x64 above the
        // record the handler gets at +0x60; +0xc0 holds the float at +0x24 of the first argument.
        let delta: Vec<u32> = call[15..19].to_vec();
        let expected: Vec<u32> = [3.0f32, 4.0, 12.0, 0.0]
            .iter()
            .map(|v| v.to_bits())
            .collect();
        assert_eq!(delta, expected);
        assert_eq!(call[19], 2.0f32.to_bits());
        assert_eq!(call[20], 13.0f32.to_bits());
        assert_eq!(call[21], call[2] - 0x10);
    }

    #[test]
    fn every_record_runs_and_a_zero_count_runs_none() {
        let mut r = rig(1, 0);
        let first = r.e.mem.alloc(0x100);
        let second = r.e.mem.alloc(0x100);
        record(&mut r, 0, first, 0);
        record(&mut r, 1, second, 0);
        r.e.call(0x00df_7760, &args![r.a1, r.a2, r.list, 2i32]);
        assert_eq!(r.log.borrow().len(), 2);
        assert_eq!(r.e.mem.u32(second), 0x1000);
        r.e.call(0x00df_7760, &args![r.a1, r.a2, r.list, 0i32]);
        r.e.call(0x00df_7760, &args![r.a1, r.a2, r.list, -3i32]);
        assert_eq!(r.log.borrow().len(), 2);
    }

    #[test]
    fn the_completion_adds_one_and_releases_the_semaphore() {
        let mut e = Engine::new();
        let log: CallLog = Rc::default();
        for (address, eax) in [(INTERLOCKED_EXCHANGE_ADD, 5u32), (SEMAPHORE_RELEASE, 8)] {
            let log = log.clone();
            e.register_double(address, move |_, a| {
                let mut words = vec![address];
                words.extend_from_slice(a);
                log.borrow_mut().push(words);
                Ret {
                    eax,
                    ..Ret::default()
                }
            });
        }
        let this = e.mem.alloc(0x30);
        assert_eq!(e.call(0x00df_7a60, &args![this]).u32(), 0);
        assert!(log.borrow().is_empty());
        e.mem.set_u32(this + 0x1c, 0x7001);
        assert_eq!(e.call(0x00df_7a60, &args![this]).u32(), 5);
        e.mem.set_u32(this + 0x10, 0x7002);
        assert_eq!(e.call(0x00df_7a60, &args![this]).u32(), 8);
        assert_eq!(
            *log.borrow(),
            vec![
                vec![INTERLOCKED_EXCHANGE_ADD, 0x7001, 1],
                vec![INTERLOCKED_EXCHANGE_ADD, 0x7001, 1],
                vec![SEMAPHORE_RELEASE, 0x7002, 1],
            ]
        );
    }

    #[test]
    fn the_copy_moves_exactly_the_listed_words() {
        let mut e = Engine::new();
        let source = e.mem.alloc(0x50);
        let dest = e.mem.alloc(0x50);
        for offset in (0..0x50).step_by(4) {
            e.mem.set_u32(source + offset, 0x100 + offset);
            e.mem.set_u32(dest + offset, 0x9000 + offset);
        }
        e.call(0x00df_7d00, &args![source, dest]);
        let copied = [
            0x10u32, 0x14, 0x1c, 0x20, 0x24, 0x28, 0x2c, 0x30, 0x34, 0x38, 0x3c, 0x40,
        ];
        for offset in (0..0x50).step_by(4) {
            let expected = if copied.contains(&offset) {
                0x100 + offset
            } else {
                0x9000 + offset
            };
            assert_eq!(e.mem.u32(dest + offset), expected, "offset {offset:#x}");
        }
    }
}

#[cfg(test)]
mod gap1_tests {
    use super::*;

    #[test]
    fn fn_00df7a90_drops_the_counter_and_finishes_at_zero() {
        let mut e = Engine::new();
        e.register(0x00df_7a60, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        let counter = e.mem.alloc(4);
        e.mem.set_u32(counter, 2);
        let job = e.mem.alloc(0x20);
        e.mem.set_u32(job + 0x14, counter);
        let finished = |e: &Engine| {
            e.call_log
                .as_ref()
                .unwrap()
                .iter()
                .filter(|(a, _)| *a == 0x00df_7a60)
                .count()
        };
        assert_eq!(e.call(0x00df_7a90, &args![0u32, 0u32, job]).u32(), 1);
        assert_eq!((e.mem.u32(counter), finished(&e)), (1, 0));
        e.mem.set_u8(job, 5);
        assert_eq!(e.call(0x00df_7a90, &args![0u32, 0u32, job]).u32(), 1);
        assert_eq!((e.mem.u32(counter), finished(&e)), (0, 1));
        // Kinds above 5: nothing.
        e.mem.set_u8(job, 6);
        assert_eq!(e.call(0x00df_7a90, &args![0u32, 0u32, job]).u32(), 1);
        assert_eq!((e.mem.u32(counter), finished(&e)), (0, 1));
    }

    #[test]
    fn fn_00df7ad0_copies_the_job_and_splits_it() {
        let mut e = Engine::new();
        let job = e.mem.alloc(0x80);
        let split = e.mem.alloc(0x80);
        for (kind, (count, total, items, size)) in SPLIT_CASES_00DF7BE0.iter().enumerate() {
            for offset in (0..0x80u32).step_by(4) {
                e.mem.set_u32(job + offset, 0x100 + offset);
            }
            e.mem.set_u8(job, kind as u8);
            e.mem.set_u32(job + count, 3);
            e.mem.set_u32(job + total, 10);
            e.mem.set_u32(job + items, 0x1000);
            assert_eq!(e.call(0x00df_7ad0, &args![0u32, 0u32, job, split]).u32(), 1);
            assert_eq!(e.mem.u32(split + 4), 0x104);
            assert_eq!(e.mem.u32(split + count), 3);
            assert_eq!(e.mem.u32(split + items), 0x1000);
            assert_eq!(e.mem.u32(split + total), 3);
            assert_eq!(e.mem.u32(job + total), 7);
            assert_eq!(e.mem.u32(job + items), 0x1000 + 3 * size);
            // A total not above the count (signed): no split.
            e.mem.set_u32(job + total, 3);
            assert_eq!(e.call(0x00df_7ad0, &args![0u32, 0u32, job, split]).u32(), 0);
            e.mem.set_u32(job + total, 0xffff_fff0);
            assert_eq!(e.call(0x00df_7ad0, &args![0u32, 0u32, job, split]).u32(), 0);
        }
        e.mem.set_u8(job, 5);
        assert_eq!(e.call(0x00df_7ad0, &args![0u32, 0u32, job, split]).u32(), 0);
        assert_eq!(e.mem.u8(split), 5);
    }
}

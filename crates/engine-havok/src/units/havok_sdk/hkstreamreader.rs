//! `Havok SDK/hkstreamreader.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `hkStreamReader` is the abstract reader: its vtable (Xbox PDB) has `isOk` (+0xc), `read`
//! (+0x10, `(buffer, size)` answering the number of bytes read), `skip` (+0x14), `markSupported`
//! (+0x18), `setMark` (+0x1c), `rewindToMark` (+0x20), `seekTellSupported` (+0x24), `seek`
//! (+0x28) and `tell` (+0x2c). The functions here are the base class's defaults.

#[allow(unused_imports)]
use crate::prelude::*;

/// Slot `+0x10` of the reader's vtable: `read(buffer, size)`, answering the bytes read.
const READ_SLOT: u32 = 0x10;
/// The chunk `skip` reads into its stack buffer at a time (the buffer is `0x200` bytes).
const SKIP_CHUNK: i32 = 0x200;

// Translated from 00cb14f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkStreamReader::skip` (Xbox PDB), thiscall, one stack word `count`: reads and throws away
/// up to `count` bytes through the virtual `read` (slot `0x10`) into a `0x200`-byte stack buffer,
/// in chunks of at most `0x200` (a signed compare). It stops when `read` answers 0 or when
/// the remaining count is exactly 0 (a negative remainder keeps going). Returns the number of
/// bytes skipped, `count - remaining`.
pub fn hk_stream_reader_skip(e: &mut Engine, this: Ptr, count: i32) -> i32 {
    let mut remaining = count;
    if remaining != 0 {
        e.with_stack(SKIP_CHUNK as u32, |e, buffer| loop {
            let chunk = if remaining > SKIP_CHUNK {
                SKIP_CHUNK
            } else {
                remaining
            };
            let read = e.vcall(this.addr(), READ_SLOT, &args![buffer, chunk]).i32();
            if read == 0 {
                break;
            }
            remaining = remaining.wrapping_sub(read);
            if remaining == 0 {
                break;
            }
        });
    }
    count.wrapping_sub(remaining)
}

// Translated from 00cb1550 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (reads one byte), thiscall, no stack words: calls the virtual
/// `read` (slot `0x10`) for one byte into a stack word and, when it answered non-zero,
/// returns that byte sign-extended; otherwise -1. The byte is the top byte of the stack
/// word the code reserves (`ESP + 3`), as the exe passes it.
pub fn fn_00cb1550(e: &mut Engine, this: Ptr) -> i32 {
    e.with_stack(4, |e, slot| {
        let byte_address = slot.addr() + 3;
        let read = e
            .vcall(this.addr(), READ_SLOT, &args![byte_address, 1u32])
            .u32();
        if read != 0 {
            i32::from(e.mem.i8(byte_address))
        } else {
            -1
        }
    })
}

// Translated from 00cb1570 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkStreamReader::setMark` (Xbox PDB), thiscall, one stack word: neither `this` nor the
/// word is read; answers 1.
pub fn hk_stream_reader_set_mark(_e: &mut Engine, _this: Ptr, _unused_0: u32) -> u32 {
    1
}

// Translated from 00cb1580 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkStreamReader::rewindToMark` (Xbox PDB), thiscall, no stack words: answers 1.
pub fn hk_stream_reader_rewind_to_mark(_e: &mut Engine, _this: Ptr) -> u32 {
    1
}

// Translated from 00cb1590 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkStreamReader::seek` (Xbox PDB), thiscall, two stack words (offset and origin, neither
/// read): answers 1.
pub fn hk_stream_reader_seek(_e: &mut Engine, _this: Ptr, _unused_0: u32, _unused_1: u32) -> u32 {
    1
}

// Translated from 00cb15a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkStreamReader::tell` (Xbox PDB), thiscall, no stack words: answers -1.
pub fn hk_stream_reader_tell(_e: &mut Engine, _this: Ptr) -> i32 {
    -1
}

// Translated from 00cb15b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkStreamReader::markSupported` (Xbox PDB), thiscall, one stack word (the `hkBool`
/// result; `this` is not read): stores false. Returns the `hkBool` pointer.
pub fn hk_stream_reader_mark_supported(e: &mut Engine, _this: Ptr, out: Ptr) -> Ptr {
    e.mem.set_u8(out.addr(), 0);
    out
}

// Translated from 00cb15c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkStreamReader::seekTellSupported` (Xbox PDB), thiscall, one stack word (the `hkBool`
/// result; `this` is not read): stores false. Returns the `hkBool` pointer.
pub fn hk_stream_reader_seek_tell_supported(e: &mut Engine, _this: Ptr, out: Ptr) -> Ptr {
    e.mem.set_u8(out.addr(), 0);
    out
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00cb_14f0, hk_stream_reader_skip(Ptr, i32) -> i32),
        entry!(0x00cb_1550, fn_00cb1550(Ptr) -> i32),
        entry!(0x00cb_1570, hk_stream_reader_set_mark(Ptr, u32) -> u32),
        entry!(0x00cb_1580, hk_stream_reader_rewind_to_mark(Ptr) -> u32),
        entry!(0x00cb_1590, hk_stream_reader_seek(Ptr, u32, u32) -> u32),
        entry!(0x00cb_15a0, hk_stream_reader_tell(Ptr) -> i32),
        entry!(0x00cb_15b0, hk_stream_reader_mark_supported(Ptr, Ptr) -> Ptr),
        entry!(0x00cb_15c0, hk_stream_reader_seek_tell_supported(Ptr, Ptr) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    type Reads = Rc<RefCell<Vec<(u32, i32)>>>;

    const READ_DOUBLE: u32 = 0x0900_0020;

    /// A reader whose `read` answers the given counts in turn (then 0) and logs
    /// `(buffer, size)` of every call.
    fn reader(e: &mut Engine, answers: Vec<u32>) -> (u32, Reads) {
        let vtable = e.mem.alloc(0x30);
        e.mem.set_u32(vtable + 0x10, READ_DOUBLE);
        let this = e.mem.alloc(8);
        e.mem.set_u32(this, vtable);
        let log: Reads = Rc::new(RefCell::new(Vec::new()));
        let seen = Rc::clone(&log);
        let mut remaining = answers.into_iter();
        e.register_double(READ_DOUBLE, move |_, a| {
            seen.borrow_mut().push((a[1], a[2] as i32));
            Ret {
                eax: remaining.next().unwrap_or(0),
                ..Ret::default()
            }
        });
        (this, log)
    }

    #[test]
    fn skip_zero_reads_nothing() {
        let mut e = Engine::new();
        let (this, log) = reader(&mut e, vec![5]);
        assert_eq!(e.call(0x00cb_14f0, &args![this, 0i32]).i32(), 0);
        assert!(log.borrow().is_empty());
    }

    #[test]
    fn skip_reads_in_chunks_of_at_most_0x200_until_the_count_is_used_up() {
        let mut e = Engine::new();
        let (this, log) = reader(&mut e, vec![0x200, 0x200, 0x64]);
        assert_eq!(e.call(0x00cb_14f0, &args![this, 0x464i32]).i32(), 0x464);
        let sizes: Vec<i32> = log.borrow().iter().map(|r| r.1).collect();
        assert_eq!(sizes, vec![0x200, 0x200, 0x64]);
        // The buffer is the same stack block each time.
        let buffers: Vec<u32> = log.borrow().iter().map(|r| r.0).collect();
        assert!(buffers.iter().all(|b| *b == buffers[0]));
    }

    #[test]
    fn skip_stops_when_read_answers_zero() {
        let mut e = Engine::new();
        let (this, log) = reader(&mut e, vec![0x10, 0]);
        assert_eq!(e.call(0x00cb_14f0, &args![this, 0x100i32]).i32(), 0x10);
        assert_eq!(log.borrow().len(), 2);
        assert_eq!(log.borrow()[1].1, 0xf0);
    }

    #[test]
    fn skip_keeps_going_past_zero_when_read_overshoots() {
        // The loop ends on exactly zero only: a read of more than asked leaves a negative
        // remainder, which is asked for next as a (negative) size.
        let mut e = Engine::new();
        let (this, log) = reader(&mut e, vec![0x14, 0]);
        // count minus the remainder: 0x10 - (-4).
        assert_eq!(e.call(0x00cb_14f0, &args![this, 0x10i32]).i32(), 0x14);
        let sizes: Vec<i32> = log.borrow().iter().map(|r| r.1).collect();
        assert_eq!(sizes, vec![0x10, -4]);
    }

    #[test]
    fn skip_of_a_negative_count_asks_for_that_size() {
        let mut e = Engine::new();
        let (this, log) = reader(&mut e, vec![0]);
        assert_eq!(e.call(0x00cb_14f0, &args![this, -3i32]).i32(), 0);
        assert_eq!(log.borrow()[0].1, -3);
    }

    #[test]
    fn fn_00cb1550_returns_the_byte_read_sign_extended() {
        let mut e = Engine::new();
        let vtable = e.mem.alloc(0x30);
        e.mem.set_u32(vtable + 0x10, READ_DOUBLE);
        let this = e.mem.alloc(8);
        e.mem.set_u32(this, vtable);
        e.register_double(READ_DOUBLE, |e, a| {
            assert_eq!(a[2], 1);
            e.mem.set_u8(a[1], 0xf0);
            Ret {
                eax: 1,
                ..Ret::default()
            }
        });
        assert_eq!(e.call(0x00cb_1550, &args![this]).i32(), -16);
        e.register_double(READ_DOUBLE, |e, a| {
            e.mem.set_u8(a[1], 0x41);
            Ret {
                eax: 7,
                ..Ret::default()
            }
        });
        assert_eq!(e.call(0x00cb_1550, &args![this]).i32(), 0x41);
    }

    #[test]
    fn fn_00cb1550_answers_minus_one_when_nothing_was_read() {
        let mut e = Engine::new();
        let (this, log) = reader(&mut e, vec![0]);
        assert_eq!(e.call(0x00cb_1550, &args![this]).i32(), -1);
        assert_eq!(log.borrow().len(), 1);
    }

    #[test]
    fn the_default_stubs_answer_their_constants() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00cb_1570, &args![0u32, 9u32]).u32(), 1);
        assert_eq!(e.call(0x00cb_1580, &args![0u32]).u32(), 1);
        assert_eq!(e.call(0x00cb_1590, &args![0u32, 1u32, 2u32]).u32(), 1);
        assert_eq!(e.call(0x00cb_15a0, &args![0u32]).i32(), -1);
    }

    #[test]
    fn the_support_queries_store_false_and_return_the_result_pointer() {
        let mut e = Engine::new();
        let out = e.mem.alloc(4);
        for address in [0x00cb_15b0u32, 0x00cb_15c0] {
            e.mem.set_u8(out, 1);
            assert_eq!(e.call(address, &args![0u32, out]).u32(), out);
            assert_eq!(e.mem.u8(out), 0);
        }
    }
}

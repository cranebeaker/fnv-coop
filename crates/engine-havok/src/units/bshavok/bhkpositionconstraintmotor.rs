//! `BSHavok/bhkpositionconstraintmotor.obj` (Xbox PDB source unit), subsystem `BSHavok`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The functions of the motor object read and write four consecutive words of it (`+0x14..+0x24`)
//! after the base class has done its own two (`+0xC`, `+0x10`).

#[allow(unused_imports)]
use crate::prelude::*;

use super::bhknicollisionobject::{enter_context, push_viewer_string};

/// The base class's stream functions (cdecl `(stream, this)`; they move the words at `+0xC` and
/// `+0x10`): reading `00d86100`, writing `00d86160`.
const BASE_READ: u32 = 0x00d8_6100;
const BASE_WRITE: u32 = 0x00d8_6160;
/// `NiRTTI` pointer word (`01268730`) that `GetViewerStrings` shows as the class name, and the
/// name builder (`00a7d330`, cdecl).
const CLASS_RTTI: u32 = 0x0126_8730;
const NI_GET_VIEWER_STRING: u32 = 0x00a7_d330;

layout! {
    /// The motor object as far as these functions know it: the four words they move.
    pub struct BhkPositionConstraintMotor: 0x24 {
        /// The first of the four words.
        0x14 firstWord: u32,
        /// The second.
        0x18 secondWord: u32,
        /// The third.
        0x1c thirdWord: u32,
        /// The fourth.
        0x20 fourthWord: u32,
    }
}

/// One stream transfer of four bytes at `address`: the stream's input object (the pointer at
/// `stream + 0x24c`, read function in slot `+8`) when reading, its output object (`stream +
/// 0x250`, write function in slot `+0xC`) when writing; the function is called cdecl as `(io,
/// address, 4, &4, 1)` with the 4 in a local, and the byte count it answers is added to the
/// counter at `io + 4`.
fn transfer_word(e: &mut Engine, stream: u32, io_pointer: u32, function_slot: u32, address: u32) {
    let io = e.mem.u32(stream + io_pointer);
    let function = e.mem.u32(io + function_slot);
    let transferred = e.with_stack(4, |e, size| {
        e.mem.set_u32(size.addr(), 4);
        e.call(function, &args![io, address, 4u32, size, 1u32])
            .u32()
    });
    let total = e.mem.u32(io + 4);
    e.mem.set_u32(io + 4, total.wrapping_add(transferred));
}

// Translated from 00cf41c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `fn_00cf41c0` (unconfirmed identity, the stream reader of the motor): runs the base reader
/// (`00d86100(stream, this)`), then reads the words at `+0x14`, `+0x18`, `+0x1C` and `+0x20`
/// from the stream's input object (the pointer at `stream + 0x24c`, whose read function is in slot
/// `+8`; each call is `(io, address, 4, &4, 1)` and the byte count it answers is added to the
/// counter at `io + 4`).
pub fn fn_00cf41c0(e: &mut Engine, this: Ptr<BhkPositionConstraintMotor>, stream: Ptr) {
    e.call(BASE_READ, &args![stream, this]);
    for offset in [0x14u32, 0x18, 0x1c, 0x20] {
        transfer_word(e, stream.addr(), 0x24c, 8, this.addr() + offset);
    }
}

// Translated from 00cf4260 (decompiled, FalloutNV.exe 1.4.0.525)
/// `fn_00cf4260` (unconfirmed identity, the stream writer of the motor): runs the base writer
/// (`00d86160(stream, this)`), then writes the words at `+0x14`, `+0x18`, `+0x1C` and `+0x20` to
/// the stream's output object (the pointer at `stream + 0x250`, whose write function is in slot
/// `+0xC`; each call is `(io, address, 4, &4, 1)` and the byte count it answers is added to the
/// counter at `io + 4`).
pub fn fn_00cf4260(e: &mut Engine, this: Ptr<BhkPositionConstraintMotor>, stream: Ptr) {
    e.call(BASE_WRITE, &args![stream, this]);
    for offset in [0x14u32, 0x18, 0x1c, 0x20] {
        transfer_word(e, stream.addr(), 0x250, 0xc, this.addr() + offset);
    }
}

// Translated from 00cf4300 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00cf4300 - C++ exception unwinding (SEH frame state) is not translated.
/// `bhkPositionConstraintMotor::GetViewerStrings` (engine map): with the thread block's `+0x2b4`
/// word at 0x10, appends to `strings` the class name (`00a7d330` on the `NiRTTI` pointer at
/// `01268730`). The motor has no base strings of its own to add.
pub fn bhk_position_constraint_motor_get_viewer_strings(
    e: &mut Engine,
    _this: Ptr<BhkPositionConstraintMotor>,
    strings: Ptr,
) {
    let (word, saved) = enter_context(e);
    let class_name = e.mem.u32(CLASS_RTTI);
    let text = e.call(NI_GET_VIEWER_STRING, &args![class_name]).u32();
    push_viewer_string(e, strings.addr(), text);
    e.mem.set_u32(word, saved);
}

// Translated from 00cf41b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkPositionConstraintMotor::GetRTTI` (Xbox PDB): takes no arguments and returns the constant 0x0126_8730.
pub fn bhk_position_constraint_motor_get_r_t_t_i(_e: &mut Engine) -> u32 {
    0x0126_8730
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00cf41b0, bhk_position_constraint_motor_get_r_t_t_i() -> u32),
        entry!(
            0x00cf41c0,
            fn_00cf41c0(Ptr<BhkPositionConstraintMotor>, Ptr)
        ),
        entry!(
            0x00cf4260,
            fn_00cf4260(Ptr<BhkPositionConstraintMotor>, Ptr)
        ),
        entry!(
            0x00cf4300,
            bhk_position_constraint_motor_get_viewer_strings(Ptr<BhkPositionConstraintMotor>, Ptr)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    type Words = Vec<u32>;

    const STRINGS_GROW: u32 = 0x0096_ad30;
    const STRINGS_SET_AT: u32 = 0x0096_ae90;

    fn calls_to(e: &Engine, addr: u32) -> Vec<Words> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    fn ret(value: u32) -> Ret {
        Ret {
            eax: value,
            ..Ret::default()
        }
    }

    /// A stream whose input object (`+0x24c`) and output object (`+0x250`) are one block `io`
    /// with the byte counter at `+4` set to 10 and the read function at `+8`, write at `+0xC`.
    fn stream_fixture(e: &mut Engine, read: u32, write: u32) -> (u32, u32) {
        let stream = e.mem.alloc(0x260);
        let io = e.mem.alloc(0x20);
        e.mem.set_u32(stream + 0x24c, io);
        e.mem.set_u32(stream + 0x250, io);
        e.mem.set_u32(io + 4, 10);
        e.mem.set_u32(io + 8, read);
        e.mem.set_u32(io + 0xc, write);
        (stream, io)
    }

    #[test]
    fn get_r_t_t_i_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00cf_41b0, &args![]).u32(), 0x0126_8730);
    }

    #[test]
    fn the_reader_reads_the_four_words_through_the_input_object() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register(BASE_READ, |_, _| Ret::default());
        // The read function stores a word per call and answers 4 bytes.
        let counter = Rc::new(RefCell::new(0u32));
        let calls = counter.clone();
        e.register_double(0x7000_0008, move |e, a| {
            let mut count = calls.borrow_mut();
            *count += 1;
            // (io, address, size, &size, count): the size cell holds 4.
            assert_eq!(e.mem.u32(a[3]), 4);
            e.mem.set_u32(a[1], 0x1000 + *count);
            ret(4)
        });
        let (stream, io) = stream_fixture(&mut e, 0x7000_0008, 0x7000_000c);
        let this = e.new_object::<BhkPositionConstraintMotor>();
        e.call(0x00cf41c0, &args![this, stream]);
        assert_eq!(e.get(this, BhkPositionConstraintMotor::firstWord), 0x1001);
        assert_eq!(e.get(this, BhkPositionConstraintMotor::secondWord), 0x1002);
        assert_eq!(e.get(this, BhkPositionConstraintMotor::thirdWord), 0x1003);
        assert_eq!(e.get(this, BhkPositionConstraintMotor::fourthWord), 0x1004);
        assert_eq!(e.mem.u32(io + 4), 26);
        assert_eq!(calls_to(&e, BASE_READ), vec![vec![stream, this.addr()]]);
        let reads = calls_to(&e, 0x7000_0008);
        let addresses: Vec<u32> = reads.iter().map(|c| c[1]).collect();
        assert_eq!(
            addresses,
            [
                this.addr() + 0x14,
                this.addr() + 0x18,
                this.addr() + 0x1c,
                this.addr() + 0x20
            ]
        );
        for call in &reads {
            assert_eq!((call[0], call[2], call[4]), (io, 4, 1));
        }
        assert!(calls_to(&e, 0x7000_000c).is_empty());
    }

    #[test]
    fn the_writer_writes_the_four_words_through_the_output_object() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register(BASE_WRITE, |_, _| Ret::default());
        let written = Rc::new(RefCell::new(Vec::new()));
        let sink = written.clone();
        e.register_double(0x7000_000c, move |e, a| {
            assert_eq!(e.mem.u32(a[3]), 4);
            sink.borrow_mut().push(e.mem.u32(a[1]));
            // Report fewer bytes on the third word to see the counter follow the answers.
            ret(if sink.borrow().len() == 3 { 2 } else { 4 })
        });
        let (stream, io) = stream_fixture(&mut e, 0x7000_0008, 0x7000_000c);
        let this = e.new_object::<BhkPositionConstraintMotor>();
        e.set(this, BhkPositionConstraintMotor::firstWord, 11);
        e.set(this, BhkPositionConstraintMotor::secondWord, 22);
        e.set(this, BhkPositionConstraintMotor::thirdWord, 33);
        e.set(this, BhkPositionConstraintMotor::fourthWord, 44);
        e.call(0x00cf4260, &args![this, stream]);
        assert_eq!(*written.borrow(), [11, 22, 33, 44]);
        assert_eq!(e.mem.u32(io + 4), 10 + 4 + 4 + 2 + 4);
        assert_eq!(calls_to(&e, BASE_WRITE), vec![vec![stream, this.addr()]]);
        assert!(calls_to(&e, 0x7000_0008).is_empty());
    }

    #[test]
    fn get_viewer_strings_appends_the_class_name() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.map(CLASS_RTTI, 4);
        e.mem.set_u32(CLASS_RTTI, 0x5151_0001);
        e.register(NI_GET_VIEWER_STRING, |_, _| ret(0xa000));
        e.register(STRINGS_GROW, |_, _| Ret::default());
        let stored = Rc::new(RefCell::new(Vec::new()));
        let sink = stored.clone();
        e.register_double(STRINGS_SET_AT, move |e, a| {
            sink.borrow_mut().push((a[1], e.mem.u32(a[2])));
            Ret::default()
        });
        let strings = e.mem.alloc(0x10);
        e.mem.set_u16(strings + 8, 2);
        e.mem.set_u16(strings + 0xa, 2);
        e.mem.set_u16(strings + 0xe, 6);
        let word = e.tls() + 0x2b4;
        e.mem.set_u32(word, 0x42);
        e.call(0x00cf4300, &args![0x5000u32, strings]);
        assert_eq!(calls_to(&e, NI_GET_VIEWER_STRING), vec![vec![0x5151_0001]]);
        // The array was full: grown by the step first.
        assert_eq!(calls_to(&e, STRINGS_GROW), vec![vec![strings, 8]]);
        assert_eq!(*stored.borrow(), vec![(2, 0xa000)]);
        assert_eq!(e.mem.u32(word), 0x42);
    }
}

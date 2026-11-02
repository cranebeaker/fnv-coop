//! `BSHavok/bhkspringdamperconstraintmotor.obj` (Xbox PDB source unit), subsystem `BSHavok`: its
//! functions in FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The stream functions of the motor read and write two words of it (`+0x14`, `+0x18`) after
//! the base class has done its own.
//!
//! The function marked `Translation incomplete` carries an `FS:[0]` exception-unwinding frame,
//! which is not modeled.

#[allow(unused_imports)]
use crate::prelude::*;

/// The base class's stream functions (cdecl `(stream, this)`): reading `00d86100`, writing
/// `00d86160`.
const BASE_READ: u32 = 0x00d8_6100;
const BASE_WRITE: u32 = 0x00d8_6160;
/// `NiGetViewerString` (`00a7d330`, cdecl: name) and the string array's resize (`0096ad30`)
/// and store (`0096ae90`).
const GET_VIEWER_STRING: u32 = 0x00a7_d330;
const STRING_ARRAY_SET_SIZE: u32 = 0x0096_ad30;
const STRING_ARRAY_SET_AT: u32 = 0x0096_ae90;
/// The class name word (`01268740`) `GetViewerStrings` passes to `NiGetViewerString`.
const CLASS_RTTI: u32 = 0x0126_8740;
/// Per-thread word the Havok-allocating functions set to 0x10 (`TLS + 0x2b4`).
const SCOPE_WORD: u32 = 0x2b4;

layout! {
    /// The motor object as far as these functions know it: the two words they move.
    pub struct BhkSpringDamperConstraintMotor: 0x1c {
        /// The first of the two words.
        0x14 firstWord: u32,
        /// The second.
        0x18 secondWord: u32,
    }
}

/// One stream transfer of four bytes at `address`: the stream's input object (the pointer at
/// `stream + 0x24c`, read function in slot `+8`) when reading, its output object (`stream +
/// 0x250`, write function in slot `+0xC`) when writing; the function is called cdecl as `(io,
/// address, 4, &4, 1)` with the 4 in a local (the exe uses the stream argument's own stack
/// slot), and the byte count it answers is added to the counter at `io + 4`.
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

// Translated from 00cf4600 (decompiled, FalloutNV.exe 1.4.0.525)
/// `fn_00cf4600` (unconfirmed identity, the stream reader of the motor): runs the base reader
/// (`00d86100(stream, this)`), then reads the words at `+0x14` and `+0x18` from the stream's
/// input object (the pointer at `stream + 0x24c`, whose read function is in slot `+8`; each
/// call is `(io, address, 4, &4, 1)` and the byte count it answers is added to the counter at
/// `io + 4`).
pub fn fn_00cf4600(e: &mut Engine, this: Ptr<BhkSpringDamperConstraintMotor>, stream: Ptr) {
    e.call(BASE_READ, &args![stream, this]);
    for offset in [0x14u32, 0x18] {
        transfer_word(e, stream.addr(), 0x24c, 8, this.addr() + offset);
    }
}

// Translated from 00cf4670 (decompiled, FalloutNV.exe 1.4.0.525)
/// `fn_00cf4670` (unconfirmed identity, the stream writer of the motor): runs the base writer
/// (`00d86160(stream, this)`), then writes the words at `+0x14` and `+0x18` to the stream's
/// output object (the pointer at `stream + 0x250`, whose write function is in slot `+0xC`;
/// each call is `(io, address, 4, &4, 1)` and the byte count it answers is added to the
/// counter at `io + 4`).
pub fn fn_00cf4670(e: &mut Engine, this: Ptr<BhkSpringDamperConstraintMotor>, stream: Ptr) {
    e.call(BASE_WRITE, &args![stream, this]);
    for offset in [0x14u32, 0x18] {
        transfer_word(e, stream.addr(), 0x250, 0xc, this.addr() + offset);
    }
}

// Translated from 00cf46e0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00cf46e0 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkSpringDamperConstraintMotor::GetViewerStrings` (engine map name), `(strings)`: ignores
/// `this`; inside the `TLS + 0x2b4 = 0x10` scope, stores the text `NiGetViewerString` makes of
/// the word at `01268740` in the string array (grown by `0096ad30` when its size, `+0xa`, has
/// reached its capacity, `+8`; stored by `0096ae90`). The motor has no base strings of its
/// own to add.
pub fn bhk_spring_damper_constraint_motor_get_viewer_strings(
    e: &mut Engine,
    _this: Ptr<BhkSpringDamperConstraintMotor>,
    strings: Ptr,
) {
    let thread = e.tls();
    let saved = e.mem.u32(thread + SCOPE_WORD);
    e.mem.set_u32(thread + SCOPE_WORD, 0x10);
    let name = e.mem.u32(CLASS_RTTI);
    let text = e.call(GET_VIEWER_STRING, &args![name]).u32();
    let array = strings.addr();
    let size = u32::from(e.mem.u16(array + 0xa));
    if size >= u32::from(e.mem.u16(array + 8)) {
        let grow_by = u32::from(e.mem.u16(array + 0xe));
        e.call(STRING_ARRAY_SET_SIZE, &args![array, grow_by + size]);
    }
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), text);
        e.call(STRING_ARRAY_SET_AT, &args![array, size, slot]);
    });
    e.mem.set_u32(thread + SCOPE_WORD, saved);
}

// Translated from 00cf45f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkSpringDamperConstraintMotor::GetRTTI` (engine map name): the address of the class's
/// RTTI record (`01268740`, the same word `GetViewerStrings` reads); `this` is not read.
pub fn bhk_spring_damper_constraint_motor_get_rtti(
    _e: &mut Engine,
    _this: Ptr<BhkSpringDamperConstraintMotor>,
) -> u32 {
    CLASS_RTTI
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00cf45f0,
            bhk_spring_damper_constraint_motor_get_rtti(Ptr<BhkSpringDamperConstraintMotor>) -> u32
        ),
        entry!(
            0x00cf4600,
            fn_00cf4600(Ptr<BhkSpringDamperConstraintMotor>, Ptr)
        ),
        entry!(
            0x00cf4670,
            fn_00cf4670(Ptr<BhkSpringDamperConstraintMotor>, Ptr)
        ),
        entry!(
            0x00cf46e0,
            bhk_spring_damper_constraint_motor_get_viewer_strings(
                Ptr<BhkSpringDamperConstraintMotor>,
                Ptr,
            )
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    const READ_FILE: u32 = 0x7000_0008;
    const WRITE_FILE: u32 = 0x7000_000c;

    fn logged_engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.call_log = Some(vec![]);
        e
    }

    fn calls_to(e: &Engine, address: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == address)
            .map(|c| c.1.clone())
            .collect()
    }

    /// A stream whose input object (`+0x24c`) and output object (`+0x250`) are one block `io`
    /// with the byte counter at `+4` set to 10 and the read function at `+8`, write at `+0xC`.
    fn stream_fixture(e: &mut Engine) -> (u32, u32) {
        let stream = e.mem.alloc(0x260);
        let io = e.mem.alloc(0x20);
        e.mem.set_u32(stream + 0x24c, io);
        e.mem.set_u32(stream + 0x250, io);
        e.mem.set_u32(io + 4, 10);
        e.mem.set_u32(io + 8, READ_FILE);
        e.mem.set_u32(io + 0xc, WRITE_FILE);
        (stream, io)
    }

    #[test]
    fn fn_00cf4600_reads_the_two_words_through_the_input_object() {
        let mut e = logged_engine();
        e.register(BASE_READ, |_, _| Ret::default());
        let mut reads_so_far = 0u32;
        e.register_double(READ_FILE, move |e, a| {
            // (io, address, size, &size, count): the size cell holds 4.
            assert_eq!(e.mem.u32(a[3]), 4);
            reads_so_far += 1;
            e.mem.set_u32(a[1], 0x1000 + reads_so_far);
            4u32.into_ret()
        });
        let (stream, io) = stream_fixture(&mut e);
        let this = e.new_object::<BhkSpringDamperConstraintMotor>();
        e.call(0x00cf_4600, &args![this, stream]);
        assert_eq!(
            e.get(this, BhkSpringDamperConstraintMotor::firstWord),
            0x1001
        );
        assert_eq!(
            e.get(this, BhkSpringDamperConstraintMotor::secondWord),
            0x1002
        );
        assert_eq!(e.mem.u32(io + 4), 18);
        assert_eq!(calls_to(&e, BASE_READ), vec![vec![stream, this.addr()]]);
        let reads = calls_to(&e, READ_FILE);
        assert_eq!(reads.len(), 2);
        assert_eq!(reads[0][..3], [io, this.addr() + 0x14, 4]);
        assert_eq!(reads[1][..3], [io, this.addr() + 0x18, 4]);
        assert!(reads.iter().all(|call| call[4] == 1));
        assert!(calls_to(&e, WRITE_FILE).is_empty());
    }

    #[test]
    fn fn_00cf4670_writes_the_two_words_through_the_output_object() {
        let mut e = logged_engine();
        e.register(BASE_WRITE, |_, _| Ret::default());
        let mut writes_so_far = 0u32;
        e.register_double(WRITE_FILE, move |e, a| {
            assert_eq!(e.mem.u32(a[3]), 4);
            writes_so_far += 1;
            // Report fewer bytes for the second word to see the counter follow the answers.
            (if writes_so_far == 2 { 2u32 } else { 4 }).into_ret()
        });
        let (stream, io) = stream_fixture(&mut e);
        let this = e.new_object::<BhkSpringDamperConstraintMotor>();
        e.set(this, BhkSpringDamperConstraintMotor::firstWord, 11);
        e.set(this, BhkSpringDamperConstraintMotor::secondWord, 22);
        e.call(0x00cf_4670, &args![this, stream]);
        assert_eq!(calls_to(&e, BASE_WRITE), vec![vec![stream, this.addr()]]);
        let writes = calls_to(&e, WRITE_FILE);
        assert_eq!(writes.len(), 2);
        assert_eq!(writes[0][..3], [io, this.addr() + 0x14, 4]);
        assert_eq!(writes[1][..3], [io, this.addr() + 0x18, 4]);
        assert_eq!(e.mem.u32(io + 4), 10 + 4 + 2);
        assert!(calls_to(&e, READ_FILE).is_empty());
    }

    #[test]
    fn bhk_spring_damper_constraint_motor_get_viewer_strings_appends_the_class_name() {
        let mut e = logged_engine();
        e.register(GET_VIEWER_STRING, |_, _| 0xa000u32.into_ret());
        e.register(STRING_ARRAY_SET_SIZE, |_, _| Ret::default());
        e.register(STRING_ARRAY_SET_AT, |e, a| {
            let base = e.mem.u32(a[0] + 4);
            let value = e.mem.u32(a[2]);
            e.mem.set_u32(base + a[1] * 4, value);
            Ret::default()
        });
        e.mem.set_u32(CLASS_RTTI, 0x5151_0001);
        let strings = e.mem.alloc(0x10);
        let base = e.mem.alloc(0x40);
        e.mem.set_u32(strings + 4, base);
        e.mem.set_u16(strings + 8, 2);
        e.mem.set_u16(strings + 0xa, 2);
        e.mem.set_u16(strings + 0xe, 6);
        let thread = e.tls();
        e.mem.set_u32(thread + SCOPE_WORD, 0x42);
        e.call(0x00cf_46e0, &args![0x5000u32, strings]);
        assert_eq!(calls_to(&e, GET_VIEWER_STRING), vec![vec![0x5151_0001]]);
        // The array was full: grown by the step first.
        assert_eq!(calls_to(&e, STRING_ARRAY_SET_SIZE), vec![vec![strings, 8]]);
        assert_eq!(e.mem.u32(base + 8), 0xa000);
        assert_eq!(e.mem.u32(thread + SCOPE_WORD), 0x42);
    }

    #[test]
    fn bhk_spring_damper_constraint_motor_get_rtti_returns_the_class_record() {
        let mut e = logged_engine();
        assert_eq!(e.call(0x00cf_45f0, &args![0x5000u32]).u32(), CLASS_RTTI);
    }

    #[test]
    fn bhk_spring_damper_constraint_motor_get_viewer_strings_does_not_grow_a_roomy_array() {
        let mut e = logged_engine();
        e.register(GET_VIEWER_STRING, |_, _| 0xa000u32.into_ret());
        e.register(STRING_ARRAY_SET_SIZE, |_, _| Ret::default());
        e.register(STRING_ARRAY_SET_AT, |_, _| Ret::default());
        let strings = e.mem.alloc(0x10);
        e.mem.set_u16(strings + 8, 4);
        e.mem.set_u16(strings + 0xa, 2);
        e.call(0x00cf_46e0, &args![0x5000u32, strings]);
        assert!(calls_to(&e, STRING_ARRAY_SET_SIZE).is_empty());
        assert_eq!(calls_to(&e, STRING_ARRAY_SET_AT).len(), 1);
        assert_eq!(calls_to(&e, STRING_ARRAY_SET_AT)[0][..2], [strings, 2]);
    }
}

//! `BSHavok/bhkvelocityconstraintmotor.obj` (Xbox PDB source unit), subsystem `BSHavok`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The two stream functions move the motor's three fields (`+0x14` and `+0x18`, 4 bytes each,
//! `+0x1C`, 1 byte) through the stream's file object: the base-class function runs first, then
//! every field goes through the file's transfer function (function pointer at `+8` of the file at
//! `[stream + 0x24C]` when loading, the one at `+0xC` of the file at `[stream + 0x250]` when saving),
//! and the file's byte counter at `+4` grows by what was transferred.

use super::bhkconstraint::{add_viewer_string, enter_scope, leave_scope, stream_transfer};
#[allow(unused_imports)]
use crate::prelude::*;

/// The base-class functions the two stream functions run first (cdecl `(stream, motor)`).
const BASE_LOAD: u32 = 0x00d8_6100;
const BASE_SAVE: u32 = 0x00d8_6160;
/// The viewer string of a type description (`00a7d330`, one word) and the global holding the
/// type description word it is given.
const VIEWER_STRING: u32 = 0x00a7_d330;
const VIEWER_TYPE: u32 = 0x0126_8738;

// Translated from 00cf43f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkVelocityConstraintMotor::GetRTTI` (Xbox PDB): takes no arguments and returns the constant 0x0126_8738.
pub fn bhk_velocity_constraint_motor_get_r_t_t_i(_e: &mut Engine) -> u32 {
    0x0126_8738
}

// Translated from 00cf4400 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name. `ECX` = the motor, stack `(stream)`: the base-class load (`00d86100`), then the
/// fields at `+0x14` (4 bytes), `+0x18` (4 bytes) and `+0x1C` (1 byte) read through the file at
/// `[stream + 0x24C]` (looked up again for every field).
pub fn fn_00cf4400(e: &mut Engine, this: Ptr, stream: u32) {
    let me = this.addr();
    e.call(BASE_LOAD, &args![stream, me]);
    for (field, size) in [(0x14, 4), (0x18, 4), (0x1c, 1)] {
        let file = e.mem.u32(stream + 0x24c);
        stream_transfer(e, file, 8, me + field, size);
    }
}

// Translated from 00cf4490 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name. `ECX` = the motor, stack `(stream)`: the base-class save (`00d86160`), then the
/// same three fields written through the file at `[stream + 0x250]` (slot `+0xC`).
pub fn fn_00cf4490(e: &mut Engine, this: Ptr, stream: u32) {
    let me = this.addr();
    e.call(BASE_SAVE, &args![stream, me]);
    for (field, size) in [(0x14, 4), (0x18, 4), (0x1c, 1)] {
        let file = e.mem.u32(stream + 0x250);
        stream_transfer(e, file, 0xc, me + field, size);
    }
}

// Translated from 00cf4520 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00cf4520 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkVelocityConstraintMotor::GetViewerStrings` (Xbox PDB), stack word `(list)`: in
/// the allocation scope (`TLS + 0x2b4` = 0x10), appends the viewer string of the type
/// description `01268738` to the viewer string list.
/// Virtual method: the first word is `this` (`ECX` in the exe), which the body never reads.
pub fn bhk_velocity_constraint_motor_get_viewer_strings(e: &mut Engine, _this: u32, list: u32) {
    let old = enter_scope(e);
    let description = e.global::<u32>(VIEWER_TYPE);
    let text = e.call(VIEWER_STRING, &args![description]).u32();
    add_viewer_string(e, list, text);
    leave_scope(e, old);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00cf43f0, bhk_velocity_constraint_motor_get_r_t_t_i() -> u32),
        entry!(0x00cf4400, fn_00cf4400(Ptr, u32)),
        entry!(0x00cf4490, fn_00cf4490(Ptr, u32)),
        entry!(
            0x00cf4520,
            bhk_velocity_constraint_motor_get_viewer_strings(u32, u32)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The `this` word of the virtual calls: these functions never read `ECX`.
    const VCALL_THIS: u32 = 0x7777_0000;

    type Log = Vec<Vec<u32>>;

    const READ_D: u32 = 0x0900_0100;
    const WRITE_D: u32 = 0x0900_0104;
    const ARRAY_GROW: u32 = 0x0096_ad30;
    const ARRAY_SET_AT: u32 = 0x0096_ae90;
    /// Words the viewer string doubles leave what they saw in.
    const SEEN_SCOPE: u32 = 0x0126_873c;
    const SEEN_TEXT: u32 = 0x0126_8740;

    fn calls_to(e: &Engine, addr: u32) -> Log {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|call| call.0 == addr)
            .map(|call| call.1.clone())
            .collect()
    }

    /// A file object with the given transfer function stored at the given offset.
    fn file(e: &mut Engine, slot: u32, target: u32) -> u32 {
        let file = e.mem.alloc(0x20);
        e.mem.set_u32(file + slot, target);
        file
    }

    /// A stream whose read file (`+0x24C`, function at `+8`) and write file (`+0x250`, function at `+0xC`)
    /// are different objects; answers `(stream, read file, write file)`.
    fn stream(e: &mut Engine) -> (u32, u32, u32) {
        e.call_log = Some(vec![]);
        e.register(BASE_LOAD, |_, _| Ret::default());
        e.register(BASE_SAVE, |_, _| Ret::default());
        // The read double fills the buffer with 0x5a bytes; both answer the size.
        e.register(READ_D, |e, a| {
            e.mem.write(a[1], &vec![0x5a; a[2] as usize]);
            Ret {
                eax: a[2],
                ..Ret::default()
            }
        });
        e.register(WRITE_D, |_, a| Ret {
            eax: a[2],
            ..Ret::default()
        });
        let stream = e.mem.alloc(0x300);
        let reader = file(e, 8, READ_D);
        let writer = file(e, 0xc, WRITE_D);
        e.mem.set_u32(stream + 0x24c, reader);
        e.mem.set_u32(stream + 0x250, writer);
        (stream, reader, writer)
    }

    #[test]
    fn loading_reads_the_three_fields_through_the_read_file() {
        let mut e = Engine::new();
        let (stream, reader, writer) = stream(&mut e);
        let motor = e.mem.alloc(0x40);
        e.call(0x00cf_4400, &args![motor, stream]);
        assert_eq!(calls_to(&e, BASE_LOAD), vec![vec![stream, motor]]);
        let reads = calls_to(&e, READ_D);
        assert_eq!(reads.len(), 3);
        let sizes: Vec<u32> = reads.iter().map(|call| call[2]).collect();
        assert_eq!(sizes, vec![4, 4, 1]);
        let buffers: Vec<u32> = reads.iter().map(|call| call[1]).collect();
        assert_eq!(buffers, vec![motor + 0x14, motor + 0x18, motor + 0x1c]);
        assert!(reads.iter().all(|call| call[0] == reader && call[4] == 1));
        assert_eq!(
            [0x14, 0x18, 0x1c, 0x1d].map(|offset| e.mem.u8(motor + offset)),
            [0x5a, 0x5a, 0x5a, 0]
        );
        // The file counted what it transferred; the write file is untouched.
        assert_eq!(e.mem.u32(reader + 4), 9);
        assert_eq!(e.mem.u32(writer + 4), 0);
        assert!(calls_to(&e, WRITE_D).is_empty());
    }

    #[test]
    fn saving_writes_the_three_fields_through_the_write_file() {
        let mut e = Engine::new();
        let (stream, reader, writer) = stream(&mut e);
        let motor = e.mem.alloc(0x40);
        e.call(0x00cf_4490, &args![motor, stream]);
        assert_eq!(calls_to(&e, BASE_SAVE), vec![vec![stream, motor]]);
        let writes = calls_to(&e, WRITE_D);
        let transfers: Vec<(u32, u32)> = writes.iter().map(|call| (call[1], call[2])).collect();
        assert_eq!(
            transfers,
            vec![(motor + 0x14, 4), (motor + 0x18, 4), (motor + 0x1c, 1)]
        );
        assert!(writes.iter().all(|call| call[0] == writer));
        assert_eq!(e.mem.u32(writer + 4), 9);
        assert_eq!(e.mem.u32(reader + 4), 0);
        assert!(calls_to(&e, READ_D).is_empty());
    }

    #[test]
    fn viewer_strings_append_the_type_description_text_in_the_allocation_scope() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.map(VIEWER_TYPE, 0x20);
        e.set_global(VIEWER_TYPE, 0x5eedu32);
        e.register(VIEWER_STRING, |e, _| {
            let tls = e.tls();
            let scope = e.mem.u32(tls + 0x2b4);
            e.mem.set_u32(SEEN_SCOPE, scope);
            Ret {
                eax: 0x7e57,
                ..Ret::default()
            }
        });
        e.register(ARRAY_GROW, |_, _| Ret::default());
        e.register(ARRAY_SET_AT, |e, a| {
            let text = e.mem.u32(a[2]);
            e.mem.set_u32(SEEN_TEXT, text);
            Ret::default()
        });
        let tls = e.tls();
        e.mem.set_u32(tls + 0x2b4, 0x77);
        let list = e.mem.alloc(0x20);
        e.mem.set_u16(list + 8, 4);
        e.mem.set_u16(list + 0xa, 2);
        e.call(0x00cf_4520, &args![VCALL_THIS, list]);
        assert_eq!(calls_to(&e, VIEWER_STRING), vec![vec![0x5eed]]);
        assert_eq!(e.mem.u32(SEEN_SCOPE), 0x10);
        assert_eq!(e.mem.u32(tls + 0x2b4), 0x77);
        assert_eq!(calls_to(&e, ARRAY_SET_AT)[0][..2], [list, 2]);
        assert_eq!(e.mem.u32(SEEN_TEXT), 0x7e57);
        assert!(calls_to(&e, ARRAY_GROW).is_empty());
        // A full list grows first, by its growth step plus the index.
        e.mem.set_u16(list + 0xa, 4);
        e.mem.set_u16(list + 0xe, 3);
        e.call(0x00cf_4520, &args![VCALL_THIS, list]);
        assert_eq!(calls_to(&e, ARRAY_GROW), vec![vec![list, 7]]);
    }

    #[test]
    fn bhk_velocity_constraint_motor_get_r_t_t_i_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00cf_43f0, &args![]).u32(), 0x0126_8738);
    }
}

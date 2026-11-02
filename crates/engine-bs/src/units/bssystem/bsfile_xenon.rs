//! `BSSystem/bsfile_xenon.cpp` (Xbox PDB source unit), subsystem `BSSystem`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md). `BSFile` itself is platform
//! (`engine-core`'s `platform.rs`, `open_memory_file`).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00b004c0 (disassembled, FalloutNV.exe 1.4.0.525)
/// No label (`thiscall`, two words it does not read, `RET 8`): answers 1 (`AL`).
pub fn fn_00b004c0(_e: &mut Engine, _this: u32, _unused_1: u32, _unused_2: u32) -> u8 {
    1
}

// Translated from 00b004d0 (disassembled, FalloutNV.exe 1.4.0.525)
/// No label (`thiscall`, plain `RET`): does nothing.
pub fn fn_00b004d0(_e: &mut Engine, _this: u32) {}

/// `_memset` (`00ec61c0`, `cdecl`) and the KERNEL32 import slots `FindFirstFileA`
/// (`00fdf070`), `FindClose` (`00fdf064`), `FileTimeToSystemTime` (`00fdf098`) and
/// `SystemTimeToTzSpecificLocalTime` (`00fdf134`), `stdcall`.
const MEMSET: u32 = 0x00ec_61c0;
const FIND_FIRST_FILE_A: u32 = 0x00fd_f070;
const FIND_CLOSE: u32 = 0x00fd_f064;
const FILE_TIME_TO_SYSTEM_TIME: u32 = 0x00fd_f098;
const SYSTEM_TIME_TO_LOCAL: u32 = 0x00fd_f134;
/// The file's slot +0x18 (`thiscall`, no words: answers its name) and its word at +0x24.
const SLOT_NAME: u32 = 0x18;
/// The frame's locals by their offset in a block standing for `EBP - 0x174`: the
/// `WIN32_FIND_DATAA` (`EBP - 0x170`, its `ftLastWriteTime` at `EBP - 0x15c`), the local
/// `SYSTEMTIME` (`EBP - 0x2c`), the UTC `SYSTEMTIME` (`EBP - 0x1c`) and the `FILETIME`
/// (`EBP - 8`).
const FRAME_SIZE: u32 = 0x174;
const FIND_DATA: u32 = 0x4;
const FIND_LAST_WRITE: u32 = 0x18;
const LOCAL_TIME: u32 = 0x148;
const UTC_TIME: u32 = 0x158;
const FILE_TIME: u32 = 0x16c;

// Translated from 00b003c0 (disassembled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b003c0 - the /GS stack cookie check (00ec408c) is not translated
/// The file's last write time as local time (no label; `thiscall`, one word: the address of
/// the 16-byte `SYSTEMTIME` answered, `RET 4`; called from `ReadSaveGameDataOLD`). With the
/// word at +0x24 zero, a zeroed `SYSTEMTIME` (`_memset` on a local) is copied out. Otherwise
/// the file is searched for by its name (slot +0x18) with `FindFirstFileA`; when the search
/// handle is not -1 its `ftLastWriteTime` is kept in a local `FILETIME`; the handle is closed
/// with `FindClose` whatever it is (-1 included); the `FILETIME` becomes a UTC `SYSTEMTIME`
/// (`FileTimeToSystemTime`) and that the local time (`SystemTimeToTzSpecificLocalTime` with
/// no zone), which is copied out. The answers of the calls are not checked: a local a failed
/// call did not write is copied as the frame holds it. Answers the out address.
pub fn fn_00b003c0(e: &mut Engine, this: u32, out: u32) -> u32 {
    e.with_stack(FRAME_SIZE, |e, frame| {
        let frame = frame.addr();
        let local = frame + LOCAL_TIME;
        if e.mem.u32(this.wrapping_add(0x24)) == 0 {
            e.call(MEMSET, &args![local, 0u32, 0x10u32]);
        } else {
            let name = e.vcall(this, SLOT_NAME, &args![]).u32();
            let handle = e
                .call(FIND_FIRST_FILE_A, &args![name, frame + FIND_DATA])
                .u32();
            if handle != 0xffff_ffff {
                let low = e.mem.u32(frame + FIND_LAST_WRITE);
                let high = e.mem.u32(frame + FIND_LAST_WRITE + 4);
                e.mem.set_u32(frame + FILE_TIME, low);
                e.mem.set_u32(frame + FILE_TIME + 4, high);
            }
            e.call(FIND_CLOSE, &args![handle]);
            e.call(
                FILE_TIME_TO_SYSTEM_TIME,
                &args![frame + FILE_TIME, frame + UTC_TIME],
            );
            e.call(SYSTEM_TIME_TO_LOCAL, &args![0u32, frame + UTC_TIME, local]);
        }
        for i in 0..4 {
            let word = e.mem.u32(local + 4 * i);
            e.mem.set_u32(out.wrapping_add(4 * i), word);
        }
    });
    out
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00b0_03c0, fn_00b003c0(u32, u32) -> u32),
        entry!(0x00b0_04c0, fn_00b004c0(u32, u32, u32) -> u8),
        entry!(0x00b0_04d0, fn_00b004d0(u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn answers_one_and_does_nothing() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00b0_04c0, &args![1u32, 2u32, 3u32]).u8(), 1);
        let this = e.mem.alloc(4);
        e.mem.set_u32(this, 9);
        e.call(0x00b0_04d0, &args![this]);
        assert_eq!(e.mem.u32(this), 9);
    }

    const NAME_FN: u32 = 0x0e00_0100;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    /// A file object (word +0x24 `open`, its name from slot +0x18), the search answering
    /// `found` with a last write time of 0x01dc_0000_1234_5678, and the time calls doubled:
    /// the UTC time's words are the `FILETIME`'s low word split, the local one each word + 1.
    fn engine(open: u32, found: bool) -> (Engine, u32, u32) {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x28);
        let vtable = e.mem.alloc(0x20);
        let name = e.mem.alloc(8);
        e.mem.set_u32(this, vtable);
        e.mem.set_u32(this + 0x24, open);
        e.mem.set_u32(vtable + SLOT_NAME, NAME_FN);
        e.register_double(NAME_FN, move |_, _| name.into_ret());
        e.register(MEMSET, |e, a| {
            e.mem.write(a[0], &vec![a[1] as u8; a[2] as usize]);
            a[0].into_ret()
        });
        e.register_double(FIND_FIRST_FILE_A, move |e, a| {
            if !found {
                return 0xffff_ffffu32.into_ret();
            }
            e.mem.set_u32(a[1] + 0x14, 0x1234_5678);
            e.mem.set_u32(a[1] + 0x18, 0x01dc_0000);
            0x40u32.into_ret()
        });
        e.register(FIND_CLOSE, |_, _| 1u32.into_ret());
        e.register(FILE_TIME_TO_SYSTEM_TIME, |e, a| {
            let low = e.mem.u32(a[0]);
            e.mem.set_u32(a[1], low);
            e.mem.set_u32(a[1] + 4, low ^ 0xffff_ffff);
            e.mem.set_u32(a[1] + 8, 3);
            e.mem.set_u32(a[1] + 12, 4);
            1u32.into_ret()
        });
        e.register(SYSTEM_TIME_TO_LOCAL, |e, a| {
            for i in 0..4 {
                let w = e.mem.u32(a[1] + 4 * i);
                e.mem.set_u32(a[2] + 4 * i, w.wrapping_add(1));
            }
            1u32.into_ret()
        });
        e.call_log = Some(vec![]);
        let out = e.mem.alloc(16);
        e.mem.write(out, &[0xaa; 16]);
        (e, this, out)
    }

    #[test]
    fn a_closed_file_answers_a_zeroed_time() {
        let (mut e, this, out) = engine(0, true);
        assert_eq!(e.call(0x00b0_03c0, &args![this, out]).u32(), out);
        assert_eq!(e.mem.bytes(out, 16), vec![0; 16]);
        assert!(calls(&e, FIND_FIRST_FILE_A).is_empty());
        assert_eq!(calls(&e, MEMSET)[0][1..], [0, 0x10]);
    }

    #[test]
    fn the_last_write_time_goes_through_utc_to_local() {
        let (mut e, this, out) = engine(1, true);
        assert_eq!(e.call(0x00b0_03c0, &args![this, out]).u32(), out);
        let words: Vec<u32> = (0..4).map(|i| e.mem.u32(out + 4 * i)).collect();
        assert_eq!(words, [0x1234_5679, 0xedcb_a988, 4, 5]);
        assert_eq!(calls(&e, FIND_CLOSE), vec![vec![0x40]]);
        let local = calls(&e, SYSTEM_TIME_TO_LOCAL);
        assert_eq!(local[0][0], 0);
        assert_eq!(local[0][1], calls(&e, FILE_TIME_TO_SYSTEM_TIME)[0][1]);
    }

    #[test]
    fn a_failed_search_closes_minus_one_and_converts_the_frame_s_time() {
        let (mut e, this, out) = engine(1, false);
        e.call(0x00b0_03c0, &args![this, out]);
        assert_eq!(calls(&e, FIND_CLOSE), vec![vec![0xffff_ffff]]);
        // The FILETIME local was never written: the frame block's zeros go through.
        let words: Vec<u32> = (0..4).map(|i| e.mem.u32(out + 4 * i)).collect();
        assert_eq!(words, [1, 0, 4, 5]);
    }
}

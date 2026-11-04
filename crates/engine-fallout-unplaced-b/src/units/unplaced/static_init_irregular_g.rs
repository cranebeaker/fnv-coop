//! `(unplaced)/static_init_irregular_g` (no source unit: the compiler's dynamic initializers),
//! subsystem `compiler: static init/exit`: 33 initializers of FalloutNV.exe 1.4.0.525 that
//! `research/initgen` refused and start-up skipped. Part 7 of the list. Decoded from
//! `read_memory` bytes (Ghidra made no function of them), every instruction checked against
//! one of two shapes:
//!
//! - 21 global arrays: `PUSH destructor; PUSH constructor; PUSH count; PUSH size; PUSH array;
//!   CALL 00ec782f` (`` `eh vector constructor iterator' ``, stdcall, five words), then
//!   `PUSH exit_fn; CALL _atexit; POP ECX; RET`.
//! - 12 critical sections: `PUSH section; CALL dword ptr [00fdf054]` (`InitializeCriticalSection`,
//!   stdcall, one word), then `PUSH exit_fn; CALL _atexit; POP ECX; RET`.
//!
//! No frame, no `/GS` check. They are cdecl functions with no parameters; `_initterm` ignores
//! `EAX`.

#[allow(unused_imports)]
use crate::prelude::*;

/// `_atexit` (`00ec658f`, cdecl, one word: the function to run at exit).
const ATEXIT: u32 = 0x00ec_658f;
/// `` `eh vector constructor iterator' `` (`00ec782f`, stdcall: array, size, count,
/// constructor, destructor).
const VECTOR_CONSTRUCTOR_ITERATOR: u32 = 0x00ec_782f;
/// The `InitializeCriticalSection` import slot (`00fdf054`, stdcall, one word).
const INITIALIZE_CRITICAL_SECTION: u32 = 0x00fd_f054;

/// (initializer, array, size, count, constructor, destructor, exit function), in table order.
type ArrayRow = (u32, u32, u32, u32, u32, u32, u32);

const ARRAYS: [ArrayRow; 21] = [
    (
        0x00fb_96e0,
        0x011f_9548,
        0x0000_0004,
        0x0000_0023,
        0x00b5_4620,
        0x00b6_6f20,
        0x00fd_cb40,
    ),
    (
        0x00fb_99a0,
        0x011f_a024,
        0x0000_0004,
        0x0000_000a,
        0x00b5_4620,
        0x00b6_6f20,
        0x00fd_ccd0,
    ),
    (
        0x00fb_9b60,
        0x011f_da48,
        0x0000_0004,
        0x0000_00d0,
        0x00b5_4620,
        0x00b6_6f20,
        0x00fd_cd00,
    ),
    (
        0x00fb_9b90,
        0x011f_dd88,
        0x0000_0004,
        0x0000_009c,
        0x00b5_4620,
        0x00b6_6f20,
        0x00fd_cd20,
    ),
    (
        0x00fb_9bc0,
        0x011f_dff8,
        0x0000_0004,
        0x0000_02f7,
        0x00b5_4620,
        0x00b7_9930,
        0x00fd_cd80,
    ),
    (
        0x00fb_9bf0,
        0x011f_ebd8,
        0x0000_0004,
        0x0000_0016,
        0x00b5_4620,
        0x0045_cec0,
        0x00fd_cd40,
    ),
    (
        0x00fb_9c20,
        0x011f_ec30,
        0x0000_0004,
        0x0000_0016,
        0x00b5_4620,
        0x0045_cec0,
        0x00fd_cd60,
    ),
    (
        0x00fb_9ce0,
        0x011f_f4a0,
        0x0000_0004,
        0x0000_0003,
        0x00b5_4620,
        0x0045_cec0,
        0x00fd_cdd0,
    ),
    (
        0x00fb_9d10,
        0x011f_f4ac,
        0x0000_0004,
        0x0000_0006,
        0x00b5_4620,
        0x0045_cec0,
        0x00fd_cdf0,
    ),
    (
        0x00fb_a480,
        0x0120_05c8,
        0x0000_0004,
        0x0000_0002,
        0x00b5_4620,
        0x00b6_6f20,
        0x00fd_cf30,
    ),
    (
        0x00fb_a4b0,
        0x0120_05d0,
        0x0000_0004,
        0x0000_0004,
        0x00b5_4620,
        0x00b6_6f20,
        0x00fd_cf50,
    ),
    (
        0x00fb_a4e0,
        0x0120_05e0,
        0x0000_0004,
        0x0000_0001,
        0x00b5_4620,
        0x00b7_9930,
        0x00fd_cf70,
    ),
    (
        0x00fb_a7a0,
        0x0120_0c48,
        0x0000_0004,
        0x0000_0032,
        0x00b5_4620,
        0x00b6_6f20,
        0x00fd_d0d0,
    ),
    (
        0x00fb_a7d0,
        0x0120_0d10,
        0x0000_0004,
        0x0000_0032,
        0x00b5_4620,
        0x00b6_6f20,
        0x00fd_d0f0,
    ),
    (
        0x00fb_a800,
        0x0120_0dd8,
        0x0000_0004,
        0x0000_0039,
        0x00b5_4620,
        0x00b6_6f20,
        0x00fd_d110,
    ),
    (
        0x00fb_a830,
        0x0120_0ec0,
        0x0000_0004,
        0x0000_0070,
        0x00b5_4620,
        0x00b7_9930,
        0x00fd_d170,
    ),
    (
        0x00fb_a860,
        0x0120_1080,
        0x0000_0004,
        0x0000_0011,
        0x00b5_4620,
        0x0045_cec0,
        0x00fd_d130,
    ),
    (
        0x00fb_a890,
        0x0120_10c4,
        0x0000_0004,
        0x0000_000a,
        0x00b5_4620,
        0x0045_cec0,
        0x00fd_d150,
    ),
    (
        0x00fb_ac90,
        0x0120_2230,
        0x0000_0004,
        0x0000_0021,
        0x00b5_4620,
        0x00b7_9930,
        0x00fd_d310,
    ),
    (
        0x00fb_ace0,
        0x0120_22c0,
        0x0000_0004,
        0x0000_001a,
        0x00b5_4620,
        0x00b7_9930,
        0x00fd_d330,
    ),
    (
        0x00fb_ad90,
        0x0120_2350,
        0x0000_0004,
        0x0000_001e,
        0x00b5_4620,
        0x00b7_9930,
        0x00fd_d350,
    ),
];

/// (initializer, critical section, exit function), in table order.
#[cfg(test)]
const CRITICAL_SECTIONS: [(u32, u32, u32); 12] = [
    (0x00fb_54e0, 0x011f_4180, 0x00fd_ae40),
    (0x00fb_5670, 0x011f_4380, 0x00fd_ae60),
    (0x00fb_5800, 0x011f_4580, 0x00fd_af00),
    (0x00fb_5890, 0x011f_4800, 0x00fd_af50),
    (0x00fb_5910, 0x011f_4900, 0x00fd_af70),
    (0x00fb_6090, 0x011f_5c80, 0x00fd_b0c0),
    (0x00fb_65e0, 0x011f_600c, 0x00fd_b260),
    (0x00fb_b110, 0x0120_2600, 0x00fd_d410),
    (0x00fb_d590, 0x0126_7e48, 0x00fd_e140),
    (0x00fb_d6d0, 0x0126_813c, 0x00fd_e160),
    (0x00fb_d9c0, 0x0126_820c, 0x00fd_e1a0),
    (0x00fc_73e0, 0x0126_f600, 0x00fd_e330),
];

/// The array shape.
fn array_at_exit(e: &mut Engine, row: &ArrayRow) {
    let (_, array, size, count, constructor, destructor, exit_fn) = *row;
    e.call(
        VECTOR_CONSTRUCTOR_ITERATOR,
        &args![array, size, count, constructor, destructor],
    );
    e.call(ATEXIT, &args![exit_fn]);
}

/// The critical-section shape.
fn critical_section_at_exit(e: &mut Engine, section: u32, exit_fn: u32) {
    e.call(INITIALIZE_CRITICAL_SECTION, &args![section]);
    e.call(ATEXIT, &args![exit_fn]);
}

// Translated from 00fb96e0 (disassembled, FalloutNV.exe 1.4.0.525)
/// The array at `11f9548` (0x23 items of 0x4 bytes, constructor `b54620`, destructor `b66f20`);
/// `_atexit(fdcb40)`.
pub fn fn_00fb96e0(e: &mut Engine) {
    array_at_exit(e, &ARRAYS[0]);
}

// Translated from 00fb99a0 (disassembled, FalloutNV.exe 1.4.0.525)
/// The array at `11fa024` (0xa items of 0x4 bytes, constructor `b54620`, destructor `b66f20`);
/// `_atexit(fdccd0)`.
pub fn fn_00fb99a0(e: &mut Engine) {
    array_at_exit(e, &ARRAYS[1]);
}

// Translated from 00fb9b60 (disassembled, FalloutNV.exe 1.4.0.525)
/// The array at `11fda48` (0xd0 items of 0x4 bytes, constructor `b54620`, destructor `b66f20`);
/// `_atexit(fdcd00)`.
pub fn fn_00fb9b60(e: &mut Engine) {
    array_at_exit(e, &ARRAYS[2]);
}

// Translated from 00fb9b90 (disassembled, FalloutNV.exe 1.4.0.525)
/// The array at `11fdd88` (0x9c items of 0x4 bytes, constructor `b54620`, destructor `b66f20`);
/// `_atexit(fdcd20)`.
pub fn fn_00fb9b90(e: &mut Engine) {
    array_at_exit(e, &ARRAYS[3]);
}

// Translated from 00fb9bc0 (disassembled, FalloutNV.exe 1.4.0.525)
/// The array at `11fdff8` (0x2f7 items of 0x4 bytes, constructor `b54620`, destructor `b79930`);
/// `_atexit(fdcd80)`.
pub fn fn_00fb9bc0(e: &mut Engine) {
    array_at_exit(e, &ARRAYS[4]);
}

// Translated from 00fb9bf0 (disassembled, FalloutNV.exe 1.4.0.525)
/// The array at `11febd8` (0x16 items of 0x4 bytes, constructor `b54620`, destructor `45cec0`);
/// `_atexit(fdcd40)`.
pub fn fn_00fb9bf0(e: &mut Engine) {
    array_at_exit(e, &ARRAYS[5]);
}

// Translated from 00fb9c20 (disassembled, FalloutNV.exe 1.4.0.525)
/// The array at `11fec30` (0x16 items of 0x4 bytes, constructor `b54620`, destructor `45cec0`);
/// `_atexit(fdcd60)`.
pub fn fn_00fb9c20(e: &mut Engine) {
    array_at_exit(e, &ARRAYS[6]);
}

// Translated from 00fb9ce0 (disassembled, FalloutNV.exe 1.4.0.525)
/// The array at `11ff4a0` (0x3 items of 0x4 bytes, constructor `b54620`, destructor `45cec0`);
/// `_atexit(fdcdd0)`.
pub fn fn_00fb9ce0(e: &mut Engine) {
    array_at_exit(e, &ARRAYS[7]);
}

// Translated from 00fb9d10 (disassembled, FalloutNV.exe 1.4.0.525)
/// The array at `11ff4ac` (0x6 items of 0x4 bytes, constructor `b54620`, destructor `45cec0`);
/// `_atexit(fdcdf0)`.
pub fn fn_00fb9d10(e: &mut Engine) {
    array_at_exit(e, &ARRAYS[8]);
}

// Translated from 00fba480 (disassembled, FalloutNV.exe 1.4.0.525)
/// The array at `12005c8` (0x2 items of 0x4 bytes, constructor `b54620`, destructor `b66f20`);
/// `_atexit(fdcf30)`.
pub fn fn_00fba480(e: &mut Engine) {
    array_at_exit(e, &ARRAYS[9]);
}

// Translated from 00fba4b0 (disassembled, FalloutNV.exe 1.4.0.525)
/// The array at `12005d0` (0x4 items of 0x4 bytes, constructor `b54620`, destructor `b66f20`);
/// `_atexit(fdcf50)`.
pub fn fn_00fba4b0(e: &mut Engine) {
    array_at_exit(e, &ARRAYS[10]);
}

// Translated from 00fba4e0 (disassembled, FalloutNV.exe 1.4.0.525)
/// The array at `12005e0` (0x1 items of 0x4 bytes, constructor `b54620`, destructor `b79930`);
/// `_atexit(fdcf70)`.
pub fn fn_00fba4e0(e: &mut Engine) {
    array_at_exit(e, &ARRAYS[11]);
}

// Translated from 00fba7a0 (disassembled, FalloutNV.exe 1.4.0.525)
/// The array at `1200c48` (0x32 items of 0x4 bytes, constructor `b54620`, destructor `b66f20`);
/// `_atexit(fdd0d0)`.
pub fn fn_00fba7a0(e: &mut Engine) {
    array_at_exit(e, &ARRAYS[12]);
}

// Translated from 00fba7d0 (disassembled, FalloutNV.exe 1.4.0.525)
/// The array at `1200d10` (0x32 items of 0x4 bytes, constructor `b54620`, destructor `b66f20`);
/// `_atexit(fdd0f0)`.
pub fn fn_00fba7d0(e: &mut Engine) {
    array_at_exit(e, &ARRAYS[13]);
}

// Translated from 00fba800 (disassembled, FalloutNV.exe 1.4.0.525)
/// The array at `1200dd8` (0x39 items of 0x4 bytes, constructor `b54620`, destructor `b66f20`);
/// `_atexit(fdd110)`.
pub fn fn_00fba800(e: &mut Engine) {
    array_at_exit(e, &ARRAYS[14]);
}

// Translated from 00fba830 (disassembled, FalloutNV.exe 1.4.0.525)
/// The array at `1200ec0` (0x70 items of 0x4 bytes, constructor `b54620`, destructor `b79930`);
/// `_atexit(fdd170)`.
pub fn fn_00fba830(e: &mut Engine) {
    array_at_exit(e, &ARRAYS[15]);
}

// Translated from 00fba860 (disassembled, FalloutNV.exe 1.4.0.525)
/// The array at `1201080` (0x11 items of 0x4 bytes, constructor `b54620`, destructor `45cec0`);
/// `_atexit(fdd130)`.
pub fn fn_00fba860(e: &mut Engine) {
    array_at_exit(e, &ARRAYS[16]);
}

// Translated from 00fba890 (disassembled, FalloutNV.exe 1.4.0.525)
/// The array at `12010c4` (0xa items of 0x4 bytes, constructor `b54620`, destructor `45cec0`);
/// `_atexit(fdd150)`.
pub fn fn_00fba890(e: &mut Engine) {
    array_at_exit(e, &ARRAYS[17]);
}

// Translated from 00fbac90 (disassembled, FalloutNV.exe 1.4.0.525)
/// The array at `1202230` (0x21 items of 0x4 bytes, constructor `b54620`, destructor `b79930`);
/// `_atexit(fdd310)`.
pub fn fn_00fbac90(e: &mut Engine) {
    array_at_exit(e, &ARRAYS[18]);
}

// Translated from 00fbace0 (disassembled, FalloutNV.exe 1.4.0.525)
/// The array at `12022c0` (0x1a items of 0x4 bytes, constructor `b54620`, destructor `b79930`);
/// `_atexit(fdd330)`.
pub fn fn_00fbace0(e: &mut Engine) {
    array_at_exit(e, &ARRAYS[19]);
}

// Translated from 00fbad90 (disassembled, FalloutNV.exe 1.4.0.525)
/// The array at `1202350` (0x1e items of 0x4 bytes, constructor `b54620`, destructor `b79930`);
/// `_atexit(fdd350)`.
pub fn fn_00fbad90(e: &mut Engine) {
    array_at_exit(e, &ARRAYS[20]);
}

// Translated from 00fb54e0 (disassembled, FalloutNV.exe 1.4.0.525)
/// `InitializeCriticalSection(011f4180)`; `_atexit(fdae40)`.
pub fn fn_00fb54e0(e: &mut Engine) {
    critical_section_at_exit(e, 0x011f_4180, 0x00fd_ae40);
}

// Translated from 00fb5670 (disassembled, FalloutNV.exe 1.4.0.525)
/// `InitializeCriticalSection(011f4380)`; `_atexit(fdae60)`.
pub fn fn_00fb5670(e: &mut Engine) {
    critical_section_at_exit(e, 0x011f_4380, 0x00fd_ae60);
}

// Translated from 00fb5800 (disassembled, FalloutNV.exe 1.4.0.525)
/// `InitializeCriticalSection(011f4580)`; `_atexit(fdaf00)`.
pub fn fn_00fb5800(e: &mut Engine) {
    critical_section_at_exit(e, 0x011f_4580, 0x00fd_af00);
}

// Translated from 00fb5890 (disassembled, FalloutNV.exe 1.4.0.525)
/// `InitializeCriticalSection(011f4800)`; `_atexit(fdaf50)`.
pub fn fn_00fb5890(e: &mut Engine) {
    critical_section_at_exit(e, 0x011f_4800, 0x00fd_af50);
}

// Translated from 00fb5910 (disassembled, FalloutNV.exe 1.4.0.525)
/// `InitializeCriticalSection(011f4900)`; `_atexit(fdaf70)`.
pub fn fn_00fb5910(e: &mut Engine) {
    critical_section_at_exit(e, 0x011f_4900, 0x00fd_af70);
}

// Translated from 00fb6090 (disassembled, FalloutNV.exe 1.4.0.525)
/// `InitializeCriticalSection(011f5c80)`; `_atexit(fdb0c0)`.
pub fn fn_00fb6090(e: &mut Engine) {
    critical_section_at_exit(e, 0x011f_5c80, 0x00fd_b0c0);
}

// Translated from 00fb65e0 (disassembled, FalloutNV.exe 1.4.0.525)
/// `InitializeCriticalSection(011f600c)`; `_atexit(fdb260)`.
pub fn fn_00fb65e0(e: &mut Engine) {
    critical_section_at_exit(e, 0x011f_600c, 0x00fd_b260);
}

// Translated from 00fbb110 (disassembled, FalloutNV.exe 1.4.0.525)
/// `InitializeCriticalSection(01202600)`; `_atexit(fdd410)`.
pub fn fn_00fbb110(e: &mut Engine) {
    critical_section_at_exit(e, 0x0120_2600, 0x00fd_d410);
}

// Translated from 00fbd590 (disassembled, FalloutNV.exe 1.4.0.525)
/// `InitializeCriticalSection(01267e48)`; `_atexit(fde140)`.
pub fn fn_00fbd590(e: &mut Engine) {
    critical_section_at_exit(e, 0x0126_7e48, 0x00fd_e140);
}

// Translated from 00fbd6d0 (disassembled, FalloutNV.exe 1.4.0.525)
/// `InitializeCriticalSection(0126813c)`; `_atexit(fde160)`.
pub fn fn_00fbd6d0(e: &mut Engine) {
    critical_section_at_exit(e, 0x0126_813c, 0x00fd_e160);
}

// Translated from 00fbd9c0 (disassembled, FalloutNV.exe 1.4.0.525)
/// `InitializeCriticalSection(0126820c)`; `_atexit(fde1a0)`.
pub fn fn_00fbd9c0(e: &mut Engine) {
    critical_section_at_exit(e, 0x0126_820c, 0x00fd_e1a0);
}

// Translated from 00fc73e0 (disassembled, FalloutNV.exe 1.4.0.525)
/// `InitializeCriticalSection(0126f600)`; `_atexit(fde330)`.
pub fn fn_00fc73e0(e: &mut Engine) {
    critical_section_at_exit(e, 0x0126_f600, 0x00fd_e330);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00fb_96e0, fn_00fb96e0()),
        entry!(0x00fb_99a0, fn_00fb99a0()),
        entry!(0x00fb_9b60, fn_00fb9b60()),
        entry!(0x00fb_9b90, fn_00fb9b90()),
        entry!(0x00fb_9bc0, fn_00fb9bc0()),
        entry!(0x00fb_9bf0, fn_00fb9bf0()),
        entry!(0x00fb_9c20, fn_00fb9c20()),
        entry!(0x00fb_9ce0, fn_00fb9ce0()),
        entry!(0x00fb_9d10, fn_00fb9d10()),
        entry!(0x00fb_a480, fn_00fba480()),
        entry!(0x00fb_a4b0, fn_00fba4b0()),
        entry!(0x00fb_a4e0, fn_00fba4e0()),
        entry!(0x00fb_a7a0, fn_00fba7a0()),
        entry!(0x00fb_a7d0, fn_00fba7d0()),
        entry!(0x00fb_a800, fn_00fba800()),
        entry!(0x00fb_a830, fn_00fba830()),
        entry!(0x00fb_a860, fn_00fba860()),
        entry!(0x00fb_a890, fn_00fba890()),
        entry!(0x00fb_ac90, fn_00fbac90()),
        entry!(0x00fb_ace0, fn_00fbace0()),
        entry!(0x00fb_ad90, fn_00fbad90()),
        entry!(0x00fb_54e0, fn_00fb54e0()),
        entry!(0x00fb_5670, fn_00fb5670()),
        entry!(0x00fb_5800, fn_00fb5800()),
        entry!(0x00fb_5890, fn_00fb5890()),
        entry!(0x00fb_5910, fn_00fb5910()),
        entry!(0x00fb_6090, fn_00fb6090()),
        entry!(0x00fb_65e0, fn_00fb65e0()),
        entry!(0x00fb_b110, fn_00fbb110()),
        entry!(0x00fb_d590, fn_00fbd590()),
        entry!(0x00fb_d6d0, fn_00fbd6d0()),
        entry!(0x00fb_d9c0, fn_00fbd9c0()),
        entry!(0x00fc_73e0, fn_00fc73e0()),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    type Calls = Vec<(u32, Vec<u32>)>;

    fn run(entry: u32) -> Calls {
        let mut e = Engine::new();
        e.register(VECTOR_CONSTRUCTOR_ITERATOR, |_, _| Ret::default());
        e.register(INITIALIZE_CRITICAL_SECTION, |_, _| Ret::default());
        e.register(ATEXIT, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(entry, &args![]);
        e.call_log
            .take()
            .unwrap()
            .into_iter()
            .filter(|(a, _)| *a != entry)
            .collect()
    }

    #[test]
    fn arrays_are_constructed_then_their_exit_function_registered() {
        for row in ARRAYS {
            let (entry, array, size, count, constructor, destructor, exit_fn) = row;
            assert_eq!(
                run(entry),
                vec![
                    (
                        VECTOR_CONSTRUCTOR_ITERATOR,
                        vec![array, size, count, constructor, destructor]
                    ),
                    (ATEXIT, vec![exit_fn]),
                ],
                "{entry:08x}"
            );
        }
        // 00fb96e0 as its bytes push them: dtor b66f20, ctor b54620, count 0x23, size 4.
        assert_eq!(
            ARRAYS[0],
            (
                0x00fb_96e0,
                0x011f_9548,
                4,
                0x23,
                0x00b5_4620,
                0x00b6_6f20,
                0x00fd_cb40
            )
        );
    }

    #[test]
    fn critical_sections_are_initialized_then_their_exit_function_registered() {
        for (entry, section, exit_fn) in CRITICAL_SECTIONS {
            assert_eq!(
                run(entry),
                vec![
                    (INITIALIZE_CRITICAL_SECTION, vec![section]),
                    (ATEXIT, vec![exit_fn]),
                ],
                "{entry:08x}"
            );
        }
        assert_eq!(funcs().len(), 33);
    }
}

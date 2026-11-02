//! `(unplaced)/static_init_irregular_h` (no source unit: the compiler's dynamic initializers),
//! subsystem `compiler: static init/exit`: 34 small initializers of FalloutNV.exe 1.4.0.525
//! that `research/initgen` refused (a load not stored next, or a call whose answer is kept
//! after an `ADD ESP`) and start-up skipped. Part 8 of the list. Decoded from `read_memory`
//! bytes (Ghidra made no function of them), every instruction checked against one of five
//! shapes, in the code's order of loads and stores:
//!
//! - 11 two-word copies: `MOV EAX,[a]; MOV ECX,[b]; MOV [c],EAX; MOV [d],ECX`.
//! - 7 four-word copies: `MOV EAX,[a]; MOV ECX,[b]; MOV EDX,[c]; MOV [d],EAX; MOV EAX,[e];
//!   MOV [f],ECX; MOV [g],EDX; MOV [h],EAX`.
//! - 6 pairs of zero words: `XOR EAX,EAX; MOV [a],EAX; MOV [b],EAX`.
//! - 3 copies with an `_atexit`: `MOV EAX,[a]; PUSH destructor; MOV [g],EAX; CALL _atexit;
//!   POP ECX`.
//! - 7 colours: `PUSH d; PUSH c; PUSH b; PUSH a; CALL rgbFromChars (00cabb00, cdecl); ADD
//!   ESP,0x10; MOV [g],EAX`.
//!
//! No frame, no `/GS` check. They are cdecl functions with no parameters; `_initterm` ignores
//! `EAX`.

#[allow(unused_imports)]
use crate::prelude::*;

/// `_atexit` (`00ec658f`, cdecl, one word: the function to run at exit).
const ATEXIT: u32 = 0x00ec_658f;
/// `rgbFromChars` (Ghidra, `00cabb00`, cdecl, four words; answers the colour word).
const RGB_FROM_CHARS: u32 = 0x00ca_bb00;

/// `MOV EAX,[a]; MOV ECX,[b]; MOV [c],EAX; MOV [d],ECX`.
fn copy2(e: &mut Engine, a: u32, b: u32, c: u32, d: u32) {
    let eax = e.global::<u32>(a);
    let ecx = e.global::<u32>(b);
    e.set_global::<u32>(c, eax);
    e.set_global::<u32>(d, ecx);
}

/// `MOV EAX,[a]; MOV ECX,[b]; MOV EDX,[c]; MOV [d],EAX; MOV EAX,[e]; MOV [f],ECX; MOV [g],EDX;
/// MOV [h],EAX`: `w` is `[a, b, c, d, e, f, g, h]`.
fn copy4(e: &mut Engine, w: [u32; 8]) {
    let eax = e.global::<u32>(w[0]);
    let ecx = e.global::<u32>(w[1]);
    let edx = e.global::<u32>(w[2]);
    e.set_global::<u32>(w[3], eax);
    let eax = e.global::<u32>(w[4]);
    e.set_global::<u32>(w[5], ecx);
    e.set_global::<u32>(w[6], edx);
    e.set_global::<u32>(w[7], eax);
}

/// `XOR EAX,EAX; MOV [a],EAX; MOV [b],EAX`.
fn zero2(e: &mut Engine, a: u32, b: u32) {
    e.set_global::<u32>(a, 0);
    e.set_global::<u32>(b, 0);
}

/// `MOV EAX,[a]; PUSH destructor; MOV [g],EAX; CALL _atexit; POP ECX`.
fn copy_at_exit(e: &mut Engine, a: u32, destructor: u32, g: u32) {
    let eax = e.global::<u32>(a);
    e.set_global::<u32>(g, eax);
    e.call(ATEXIT, &args![destructor]);
}

/// `PUSH p3; PUSH p2; PUSH p1; PUSH p0; CALL 00cabb00; ADD ESP,0x10; MOV [g],EAX`.
fn colour(e: &mut Engine, p: [u32; 4], g: u32) {
    let c = e.call(RGB_FROM_CHARS, &args![p[0], p[1], p[2], p[3]]).u32();
    e.set_global::<u32>(g, c);
}

// Translated from 00fb9740 (disassembled, FalloutNV.exe 1.4.0.525)
/// `011f95dc` = `011a9b50`, `011f95e0` = `011a9b54` (loaded first, then stored).
pub fn fn_00fb9740(e: &mut Engine) {
    copy2(e, 0x011a_9b50, 0x011a_9b54, 0x011f_95dc, 0x011f_95e0);
}

// Translated from 00fb9810 (disassembled, FalloutNV.exe 1.4.0.525)
/// `011f962c` = `011f4980`, `011f9630` = `011f4984` (loaded first, then stored).
pub fn fn_00fb9810(e: &mut Engine) {
    copy2(e, 0x011f_4980, 0x011f_4984, 0x011f_962c, 0x011f_9630);
}

// Translated from 00fb9830 (disassembled, FalloutNV.exe 1.4.0.525)
/// `011f9634` = `011f4980`, `011f9638` = `011f4984` (loaded first, then stored).
pub fn fn_00fb9830(e: &mut Engine) {
    copy2(e, 0x011f_4980, 0x011f_4984, 0x011f_9634, 0x011f_9638);
}

// Translated from 00fbae50 (disassembled, FalloutNV.exe 1.4.0.525)
/// `012023f4` = `011f4980`, `012023f8` = `011f4984` (loaded first, then stored).
pub fn fn_00fbae50(e: &mut Engine) {
    copy2(e, 0x011f_4980, 0x011f_4984, 0x0120_23f4, 0x0120_23f8);
}

// Translated from 00fbef60 (disassembled, FalloutNV.exe 1.4.0.525)
/// `011b5258` = `011b51c8`, `011b5270` = `011b51c4` (loaded first, then stored).
pub fn fn_00fbef60(e: &mut Engine) {
    copy2(e, 0x011b_51c8, 0x011b_51c4, 0x011b_5258, 0x011b_5270);
}

// Translated from 00fbf1a0 (disassembled, FalloutNV.exe 1.4.0.525)
/// `011b56e0` = `011b5658`, `011b56f8` = `011b565c` (loaded first, then stored).
pub fn fn_00fbf1a0(e: &mut Engine) {
    copy2(e, 0x011b_5658, 0x011b_565c, 0x011b_56e0, 0x011b_56f8);
}

// Translated from 00fbf4a0 (disassembled, FalloutNV.exe 1.4.0.525)
/// `011b62d0` = `011b6238`, `011b6300` = `011b6248` (loaded first, then stored).
pub fn fn_00fbf4a0(e: &mut Engine) {
    copy2(e, 0x011b_6238, 0x011b_6248, 0x011b_62d0, 0x011b_6300);
}

// Translated from 00fc1160 (disassembled, FalloutNV.exe 1.4.0.525)
/// `011b99a0` = `011b995c`, `011b9a18` = `011b9958` (loaded first, then stored).
pub fn fn_00fc1160(e: &mut Engine) {
    copy2(e, 0x011b_995c, 0x011b_9958, 0x011b_99a0, 0x011b_9a18);
}

// Translated from 00fc11b0 (disassembled, FalloutNV.exe 1.4.0.525)
/// `011b9ac0` = `011b995c`, `011b9af0` = `011b9958` (loaded first, then stored).
pub fn fn_00fc11b0(e: &mut Engine) {
    copy2(e, 0x011b_995c, 0x011b_9958, 0x011b_9ac0, 0x011b_9af0);
}

// Translated from 00fc3140 (disassembled, FalloutNV.exe 1.4.0.525)
/// `011bb5f8` = `011b7fe0`, `011bb610` = `011bb528` (loaded first, then stored).
pub fn fn_00fc3140(e: &mut Engine) {
    copy2(e, 0x011b_7fe0, 0x011b_b528, 0x011b_b5f8, 0x011b_b610);
}

// Translated from 00fc3270 (disassembled, FalloutNV.exe 1.4.0.525)
/// `011bb928` = `011bb8d0`, `011bb940` = `011bb8d4` (loaded first, then stored).
pub fn fn_00fc3270(e: &mut Engine) {
    copy2(e, 0x011b_b8d0, 0x011b_b8d4, 0x011b_b928, 0x011b_b940);
}

// Translated from 00fb4a20 (disassembled, FalloutNV.exe 1.4.0.525)
/// `011f366c`, `011f3670`, `011f3674`, `011f3678` = `011a9ea4`, `011a9ea8`, `011a9eac`, `011a9eb0` (in the loads and stores order of [`copy4`]).
pub fn fn_00fb4a20(e: &mut Engine) {
    copy4(
        e,
        [
            0x011a_9ea4,
            0x011a_9ea8,
            0x011a_9eac,
            0x011f_366c,
            0x011a_9eb0,
            0x011f_3670,
            0x011f_3674,
            0x011f_3678,
        ],
    );
}

// Translated from 00fb4a50 (disassembled, FalloutNV.exe 1.4.0.525)
/// `011f3680`, `011f3684`, `011f3688`, `011f368c` = `011a9bd0`, `011a9bd4`, `011a9bd8`, `011a9bdc` (in the loads and stores order of [`copy4`]).
pub fn fn_00fb4a50(e: &mut Engine) {
    copy4(
        e,
        [
            0x011a_9bd0,
            0x011a_9bd4,
            0x011a_9bd8,
            0x011f_3680,
            0x011a_9bdc,
            0x011f_3684,
            0x011f_3688,
            0x011f_368c,
        ],
    );
}

// Translated from 00fb97a0 (disassembled, FalloutNV.exe 1.4.0.525)
/// `011f95f4`, `011f95f8`, `011f95fc`, `011f9600` = `011a9bd0`, `011a9bd4`, `011a9bd8`, `011a9bdc` (in the loads and stores order of [`copy4`]).
pub fn fn_00fb97a0(e: &mut Engine) {
    copy4(
        e,
        [
            0x011a_9bd0,
            0x011a_9bd4,
            0x011a_9bd8,
            0x011f_95f4,
            0x011a_9bdc,
            0x011f_95f8,
            0x011f_95fc,
            0x011f_9600,
        ],
    );
}

// Translated from 00fb97e0 (disassembled, FalloutNV.exe 1.4.0.525)
/// `011f961c`, `011f9620`, `011f9624`, `011f9628` = `011a9bd0`, `011a9bd4`, `011a9bd8`, `011a9bdc` (in the loads and stores order of [`copy4`]).
pub fn fn_00fb97e0(e: &mut Engine) {
    copy4(
        e,
        [
            0x011a_9bd0,
            0x011a_9bd4,
            0x011a_9bd8,
            0x011f_961c,
            0x011a_9bdc,
            0x011f_9620,
            0x011f_9624,
            0x011f_9628,
        ],
    );
}

// Translated from 00fba040 (disassembled, FalloutNV.exe 1.4.0.525)
/// `0120002c`, `01200030`, `01200034`, `01200038` = `011a9bd0`, `011a9bd4`, `011a9bd8`, `011a9bdc` (in the loads and stores order of [`copy4`]).
pub fn fn_00fba040(e: &mut Engine) {
    copy4(
        e,
        [
            0x011a_9bd0,
            0x011a_9bd4,
            0x011a_9bd8,
            0x0120_002c,
            0x011a_9bdc,
            0x0120_0030,
            0x0120_0034,
            0x0120_0038,
        ],
    );
}

// Translated from 00fba070 (disassembled, FalloutNV.exe 1.4.0.525)
/// `0120003c`, `01200040`, `01200044`, `01200048` = `011a9bd0`, `011a9bd4`, `011a9bd8`, `011a9bdc` (in the loads and stores order of [`copy4`]).
pub fn fn_00fba070(e: &mut Engine) {
    copy4(
        e,
        [
            0x011a_9bd0,
            0x011a_9bd4,
            0x011a_9bd8,
            0x0120_003c,
            0x011a_9bdc,
            0x0120_0040,
            0x0120_0044,
            0x0120_0048,
        ],
    );
}

// Translated from 00fbf580 (disassembled, FalloutNV.exe 1.4.0.525)
/// `011b6438`, `011b64b0`, `011b65a0`, `011b6858` = `011b63dc`, `011b63e0`, `011b63d8`, `011b63d4` (in the loads and stores order of [`copy4`]).
pub fn fn_00fbf580(e: &mut Engine) {
    copy4(
        e,
        [
            0x011b_63dc,
            0x011b_63e0,
            0x011b_63d8,
            0x011b_6438,
            0x011b_63d4,
            0x011b_64b0,
            0x011b_65a0,
            0x011b_6858,
        ],
    );
}

// Translated from 00fb4d90 (disassembled, FalloutNV.exe 1.4.0.525)
/// `011f3ea0` = 0, `011f3ea4` = 0.
pub fn fn_00fb4d90(e: &mut Engine) {
    zero2(e, 0x011f_3ea0, 0x011f_3ea4);
}

// Translated from 00fb5640 (disassembled, FalloutNV.exe 1.4.0.525)
/// `011f42a0` = 0, `011f42a4` = 0.
pub fn fn_00fb5640(e: &mut Engine) {
    zero2(e, 0x011f_42a0, 0x011f_42a4);
}

// Translated from 00fb57d0 (disassembled, FalloutNV.exe 1.4.0.525)
/// `011f4480` = 0, `011f4484` = 0.
pub fn fn_00fb57d0(e: &mut Engine) {
    zero2(e, 0x011f_4480, 0x011f_4484);
}

// Translated from 00fb98f0 (disassembled, FalloutNV.exe 1.4.0.525)
/// `011f9ea0` = 0, `011f9ea4` = 0.
pub fn fn_00fb98f0(e: &mut Engine) {
    zero2(e, 0x011f_9ea0, 0x011f_9ea4);
}

// Translated from 00fb9900 (disassembled, FalloutNV.exe 1.4.0.525)
/// `011f9ec0` = 0, `011f9ec4` = 0.
pub fn fn_00fb9900(e: &mut Engine) {
    zero2(e, 0x011f_9ec0, 0x011f_9ec4);
}

// Translated from 00fbcc80 (disassembled, FalloutNV.exe 1.4.0.525)
/// `01267b80` = 0, `01267b84` = 0.
pub fn fn_00fbcc80(e: &mut Engine) {
    zero2(e, 0x0126_7b80, 0x0126_7b84);
}

// Translated from 00fb4d50 (disassembled, FalloutNV.exe 1.4.0.525)
/// `011f3ec0` = `0109b220`; `_atexit(00fdadd0)`.
pub fn fn_00fb4d50(e: &mut Engine) {
    copy_at_exit(e, 0x0109_b220, 0x00fd_add0, 0x011f_3ec0);
}

// Translated from 00fb4d70 (disassembled, FalloutNV.exe 1.4.0.525)
/// `011f3ec4` = `0109b220`; `_atexit(00fdadf0)`.
pub fn fn_00fb4d70(e: &mut Engine) {
    copy_at_exit(e, 0x0109_b220, 0x00fd_adf0, 0x011f_3ec4);
}

// Translated from 00fb5a10 (disassembled, FalloutNV.exe 1.4.0.525)
/// `011f49e0` = `0109b220`; `_atexit(00fdafe0)`.
pub fn fn_00fb5a10(e: &mut Engine) {
    copy_at_exit(e, 0x0109_b220, 0x00fd_afe0, 0x011f_49e0);
}

// Translated from 00fbf300 (disassembled, FalloutNV.exe 1.4.0.525)
/// `01268fd8` = `rgbFromChars(0xc8, 0xc8, 0xff, 0x50)`.
pub fn fn_00fbf300(e: &mut Engine) {
    colour(e, [0xc8, 0xc8, 0xff, 0x50], 0x0126_8fd8);
}

// Translated from 00fbf320 (disassembled, FalloutNV.exe 1.4.0.525)
/// `01268fd4` = `rgbFromChars(0xc8, 0xff, 0xc8, 0x96)`.
pub fn fn_00fbf320(e: &mut Engine) {
    colour(e, [0xc8, 0xff, 0xc8, 0x96], 0x0126_8fd4);
}

// Translated from 00fbf350 (disassembled, FalloutNV.exe 1.4.0.525)
/// `01268fe4` = `rgbFromChars(0xf0, 0xf0, 0xf0, 0xff)`.
pub fn fn_00fbf350(e: &mut Engine) {
    colour(e, [0xf0, 0xf0, 0xf0, 0xff], 0x0126_8fe4);
}

// Translated from 00fbf380 (disassembled, FalloutNV.exe 1.4.0.525)
/// `01268fe8` = `rgbFromChars(0x78, 0x78, 0x78, 0xff)`.
pub fn fn_00fbf380(e: &mut Engine) {
    colour(e, [0x78, 0x78, 0x78, 0xff], 0x0126_8fe8);
}

// Translated from 00fbf3a0 (disassembled, FalloutNV.exe 1.4.0.525)
/// `01268ff4` = `rgbFromChars(0xf0, 0xc8, 0x0, 0xc8)`.
pub fn fn_00fbf3a0(e: &mut Engine) {
    colour(e, [0xf0, 0xc8, 0x0, 0xc8], 0x0126_8ff4);
}

// Translated from 00fbf3e0 (disassembled, FalloutNV.exe 1.4.0.525)
/// `01269014` = `rgbFromChars(0x8c, 0xf0, 0x8c, 0x8c)`.
pub fn fn_00fbf3e0(e: &mut Engine) {
    colour(e, [0x8c, 0xf0, 0x8c, 0x8c], 0x0126_9014);
}

// Translated from 00fbf410 (disassembled, FalloutNV.exe 1.4.0.525)
/// `01269018` = `rgbFromChars(0x46, 0xc8, 0x46, 0x8c)`.
pub fn fn_00fbf410(e: &mut Engine) {
    colour(e, [0x46, 0xc8, 0x46, 0x8c], 0x0126_9018);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00fb_9740, fn_00fb9740()),
        entry!(0x00fb_9810, fn_00fb9810()),
        entry!(0x00fb_9830, fn_00fb9830()),
        entry!(0x00fb_ae50, fn_00fbae50()),
        entry!(0x00fb_ef60, fn_00fbef60()),
        entry!(0x00fb_f1a0, fn_00fbf1a0()),
        entry!(0x00fb_f4a0, fn_00fbf4a0()),
        entry!(0x00fc_1160, fn_00fc1160()),
        entry!(0x00fc_11b0, fn_00fc11b0()),
        entry!(0x00fc_3140, fn_00fc3140()),
        entry!(0x00fc_3270, fn_00fc3270()),
        entry!(0x00fb_4a20, fn_00fb4a20()),
        entry!(0x00fb_4a50, fn_00fb4a50()),
        entry!(0x00fb_97a0, fn_00fb97a0()),
        entry!(0x00fb_97e0, fn_00fb97e0()),
        entry!(0x00fb_a040, fn_00fba040()),
        entry!(0x00fb_a070, fn_00fba070()),
        entry!(0x00fb_f580, fn_00fbf580()),
        entry!(0x00fb_4d90, fn_00fb4d90()),
        entry!(0x00fb_5640, fn_00fb5640()),
        entry!(0x00fb_57d0, fn_00fb57d0()),
        entry!(0x00fb_98f0, fn_00fb98f0()),
        entry!(0x00fb_9900, fn_00fb9900()),
        entry!(0x00fb_cc80, fn_00fbcc80()),
        entry!(0x00fb_4d50, fn_00fb4d50()),
        entry!(0x00fb_4d70, fn_00fb4d70()),
        entry!(0x00fb_5a10, fn_00fb5a10()),
        entry!(0x00fb_f300, fn_00fbf300()),
        entry!(0x00fb_f320, fn_00fbf320()),
        entry!(0x00fb_f350, fn_00fbf350()),
        entry!(0x00fb_f380, fn_00fbf380()),
        entry!(0x00fb_f3a0, fn_00fbf3a0()),
        entry!(0x00fb_f3e0, fn_00fbf3e0()),
        entry!(0x00fb_f410, fn_00fbf410()),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    type Calls = Vec<(u32, Vec<u32>)>;

    /// (initializer, the words it reads and writes in its code order) per shape.
    const COPY2: &[(u32, [u32; 4])] = &[
        (
            0x00fb_9740,
            [0x011a_9b50, 0x011a_9b54, 0x011f_95dc, 0x011f_95e0],
        ),
        (
            0x00fb_9810,
            [0x011f_4980, 0x011f_4984, 0x011f_962c, 0x011f_9630],
        ),
        (
            0x00fb_9830,
            [0x011f_4980, 0x011f_4984, 0x011f_9634, 0x011f_9638],
        ),
        (
            0x00fb_ae50,
            [0x011f_4980, 0x011f_4984, 0x0120_23f4, 0x0120_23f8],
        ),
        (
            0x00fb_ef60,
            [0x011b_51c8, 0x011b_51c4, 0x011b_5258, 0x011b_5270],
        ),
        (
            0x00fb_f1a0,
            [0x011b_5658, 0x011b_565c, 0x011b_56e0, 0x011b_56f8],
        ),
        (
            0x00fb_f4a0,
            [0x011b_6238, 0x011b_6248, 0x011b_62d0, 0x011b_6300],
        ),
        (
            0x00fc_1160,
            [0x011b_995c, 0x011b_9958, 0x011b_99a0, 0x011b_9a18],
        ),
        (
            0x00fc_11b0,
            [0x011b_995c, 0x011b_9958, 0x011b_9ac0, 0x011b_9af0],
        ),
        (
            0x00fc_3140,
            [0x011b_7fe0, 0x011b_b528, 0x011b_b5f8, 0x011b_b610],
        ),
        (
            0x00fc_3270,
            [0x011b_b8d0, 0x011b_b8d4, 0x011b_b928, 0x011b_b940],
        ),
    ];
    const COPY4: &[(u32, [u32; 8])] = &[
        (
            0x00fb_4a20,
            [
                0x011a_9ea4,
                0x011a_9ea8,
                0x011a_9eac,
                0x011f_366c,
                0x011a_9eb0,
                0x011f_3670,
                0x011f_3674,
                0x011f_3678,
            ],
        ),
        (
            0x00fb_4a50,
            [
                0x011a_9bd0,
                0x011a_9bd4,
                0x011a_9bd8,
                0x011f_3680,
                0x011a_9bdc,
                0x011f_3684,
                0x011f_3688,
                0x011f_368c,
            ],
        ),
        (
            0x00fb_97a0,
            [
                0x011a_9bd0,
                0x011a_9bd4,
                0x011a_9bd8,
                0x011f_95f4,
                0x011a_9bdc,
                0x011f_95f8,
                0x011f_95fc,
                0x011f_9600,
            ],
        ),
        (
            0x00fb_97e0,
            [
                0x011a_9bd0,
                0x011a_9bd4,
                0x011a_9bd8,
                0x011f_961c,
                0x011a_9bdc,
                0x011f_9620,
                0x011f_9624,
                0x011f_9628,
            ],
        ),
        (
            0x00fb_a040,
            [
                0x011a_9bd0,
                0x011a_9bd4,
                0x011a_9bd8,
                0x0120_002c,
                0x011a_9bdc,
                0x0120_0030,
                0x0120_0034,
                0x0120_0038,
            ],
        ),
        (
            0x00fb_a070,
            [
                0x011a_9bd0,
                0x011a_9bd4,
                0x011a_9bd8,
                0x0120_003c,
                0x011a_9bdc,
                0x0120_0040,
                0x0120_0044,
                0x0120_0048,
            ],
        ),
        (
            0x00fb_f580,
            [
                0x011b_63dc,
                0x011b_63e0,
                0x011b_63d8,
                0x011b_6438,
                0x011b_63d4,
                0x011b_64b0,
                0x011b_65a0,
                0x011b_6858,
            ],
        ),
    ];
    const ZERO2: &[(u32, [u32; 2])] = &[
        (0x00fb_4d90, [0x011f_3ea0, 0x011f_3ea4]),
        (0x00fb_5640, [0x011f_42a0, 0x011f_42a4]),
        (0x00fb_57d0, [0x011f_4480, 0x011f_4484]),
        (0x00fb_98f0, [0x011f_9ea0, 0x011f_9ea4]),
        (0x00fb_9900, [0x011f_9ec0, 0x011f_9ec4]),
        (0x00fb_cc80, [0x0126_7b80, 0x0126_7b84]),
    ];
    const COPY_AT_EXIT: &[(u32, [u32; 3])] = &[
        (0x00fb_4d50, [0x0109_b220, 0x00fd_add0, 0x011f_3ec0]),
        (0x00fb_4d70, [0x0109_b220, 0x00fd_adf0, 0x011f_3ec4]),
        (0x00fb_5a10, [0x0109_b220, 0x00fd_afe0, 0x011f_49e0]),
    ];
    const COLOURS: &[(u32, [u32; 4], u32)] = &[
        (0x00fb_f300, [0xc8, 0xc8, 0xff, 0x50], 0x0126_8fd8),
        (0x00fb_f320, [0xc8, 0xff, 0xc8, 0x96], 0x0126_8fd4),
        (0x00fb_f350, [0xf0, 0xf0, 0xf0, 0xff], 0x0126_8fe4),
        (0x00fb_f380, [0x78, 0x78, 0x78, 0xff], 0x0126_8fe8),
        (0x00fb_f3a0, [0xf0, 0xc8, 0x0, 0xc8], 0x0126_8ff4),
        (0x00fb_f3e0, [0x8c, 0xf0, 0x8c, 0x8c], 0x0126_9014),
        (0x00fb_f410, [0x46, 0xc8, 0x46, 0x8c], 0x0126_9018),
    ];

    /// A fresh engine with every word of `words` mapped, each source preset to a distinct value.
    fn engine(words: &[u32]) -> Engine {
        let mut e = Engine::new();
        for &w in words {
            if !e.mem.is_mapped(w) {
                e.map(w, 4);
            }
        }
        for (i, &w) in words.iter().enumerate() {
            e.mem.set_u32(w, 0x1111_0000 + i as u32);
        }
        e.register(ATEXIT, |_, _| Ret::default());
        e
    }

    fn run(e: &mut Engine, entry: u32) -> Calls {
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
    fn two_word_copies() {
        for &(entry, w) in COPY2 {
            let mut e = engine(&w);
            let (a, b) = (e.mem.u32(w[0]), e.mem.u32(w[1]));
            assert!(run(&mut e, entry).is_empty());
            assert_eq!((e.mem.u32(w[2]), e.mem.u32(w[3])), (a, b), "{entry:08x}");
        }
    }

    #[test]
    fn four_word_copies_follow_the_load_and_store_order() {
        for &(entry, w) in COPY4 {
            let mut e = engine(&w);
            let (a, b, c) = (e.mem.u32(w[0]), e.mem.u32(w[1]), e.mem.u32(w[2]));
            let e4 = e.mem.u32(w[4]);
            assert!(run(&mut e, entry).is_empty());
            // [e] is read after [d] is written.
            let after_d = if w[4] == w[3] { a } else { e4 };
            assert_eq!(
                [e.mem.u32(w[5]), e.mem.u32(w[6]), e.mem.u32(w[7])],
                [b, c, after_d],
                "{entry:08x}"
            );
        }
    }

    #[test]
    fn zero_pairs() {
        for &(entry, w) in ZERO2 {
            let mut e = engine(&w);
            assert!(run(&mut e, entry).is_empty());
            assert_eq!((e.mem.u32(w[0]), e.mem.u32(w[1])), (0, 0), "{entry:08x}");
        }
    }

    #[test]
    fn copies_with_an_exit_function() {
        for &(entry, [a, destructor, g]) in COPY_AT_EXIT {
            let mut e = engine(&[a, g]);
            let v = e.mem.u32(a);
            assert_eq!(run(&mut e, entry), vec![(ATEXIT, vec![destructor])]);
            assert_eq!(e.mem.u32(g), v, "{entry:08x}");
        }
    }

    #[test]
    fn colours_keep_the_answer() {
        for &(entry, p, g) in COLOURS {
            let mut e = engine(&[g]);
            e.register(RGB_FROM_CHARS, |_, a| Ret {
                eax: a[0] ^ (a[1] << 8) ^ (a[2] << 16) ^ (a[3] << 24),
                ..Ret::default()
            });
            assert_eq!(run(&mut e, entry), vec![(RGB_FROM_CHARS, p.to_vec())]);
            assert_eq!(
                e.mem.u32(g),
                p[0] ^ (p[1] << 8) ^ (p[2] << 16) ^ (p[3] << 24),
                "{entry:08x}"
            );
        }
        assert_eq!(
            funcs().len(),
            COPY2.len() + COPY4.len() + ZERO2.len() + COPY_AT_EXIT.len() + COLOURS.len()
        );
    }
}

//! `(unplaced)/static_init_irregular_f` (no source unit: the compiler's dynamic initializers),
//! subsystem `compiler: static init/exit`: 68 initializers of FalloutNV.exe 1.4.0.525 that only
//! give a destructor to `_atexit` (`PUSH destructor; CALL 00ec658f; POP ECX; RET`: the
//! globals they belong to need no construction), refused by `research/initgen` (no shape of
//! its table is a bare `_atexit`) and skipped by start-up. Part 6 of the list. Decoded from
//! `read_memory` bytes (Ghidra made no function of them), every instruction checked. They are
//! cdecl functions with no parameters; `EAX` (what `_atexit` answered) is ignored by
//! `_initterm`, so they return nothing.

#[allow(unused_imports)]
use crate::prelude::*;

/// `_atexit` (`00ec658f`, cdecl, one word: the function to run at exit).
const ATEXIT: u32 = 0x00ec_658f;

/// (initializer, destructor), in table order (the tests check each function against it).
#[cfg(test)]
const AT_EXIT_ONLY: [(u32, u32); 68] = [
    (0x00fc_793c, 0x00fd_e581),
    (0x00fb_5500, 0x00fd_ae50),
    (0x00fb_56b0, 0x00fd_ae70),
    (0x00fb_5750, 0x00fd_aea0),
    (0x00fb_5780, 0x00fd_aed0),
    (0x00fb_5990, 0x00fd_afb0),
    (0x00fb_5ae0, 0x00fd_b000),
    (0x00fb_5bf0, 0x00fd_b030),
    (0x00fb_5c40, 0x00fd_b060),
    (0x00fb_5db0, 0x00fd_b090),
    (0x00fb_6150, 0x00fd_b0d0),
    (0x00fb_6180, 0x00fd_b100),
    (0x00fb_6340, 0x00fd_b1a0),
    (0x00fb_64d0, 0x00fd_b1d0),
    (0x00fb_6520, 0x00fd_b200),
    (0x00fb_65d0, 0x00fd_b270),
    (0x00fb_9510, 0x00fd_c750),
    (0x00fb_95f0, 0x00fd_cb10),
    (0x00fb_9600, 0x00fd_c7b0),
    (0x00fb_9610, 0x00fd_c7e0),
    (0x00fb_9620, 0x00fd_c810),
    (0x00fb_9630, 0x00fd_c840),
    (0x00fb_9640, 0x00fd_c870),
    (0x00fb_9650, 0x00fd_c8a0),
    (0x00fb_9660, 0x00fd_c8d0),
    (0x00fb_9670, 0x00fd_c900),
    (0x00fb_9680, 0x00fd_c930),
    (0x00fb_9690, 0x00fd_c960),
    (0x00fb_96a0, 0x00fd_c990),
    (0x00fb_96b0, 0x00fd_c9c0),
    (0x00fb_96c0, 0x00fd_c9f0),
    (0x00fb_96d0, 0x00fd_ca20),
    (0x00fb_9710, 0x00fd_ca50),
    (0x00fb_9720, 0x00fd_ca80),
    (0x00fb_9730, 0x00fd_cb60),
    (0x00fb_9760, 0x00fd_cab0),
    (0x00fb_9770, 0x00fd_cae0),
    (0x00fb_9780, 0x00fd_cb90),
    (0x00fb_9790, 0x00fd_cbc0),
    (0x00fb_9950, 0x00fd_cca0),
    (0x00fb_9c70, 0x00fd_cda0),
    (0x00fb_a020, 0x00fd_ce40),
    (0x00fb_a030, 0x00fd_ce80),
    (0x00fb_a100, 0x00fd_cef0),
    (0x00fb_a2d0, 0x00fd_cf20),
    (0x00fb_a6d0, 0x00fd_cf90),
    (0x00fb_a700, 0x00fd_cfc0),
    (0x00fb_a750, 0x00fd_cfe0),
    (0x00fb_a760, 0x00fd_d010),
    (0x00fb_a770, 0x00fd_d040),
    (0x00fb_a780, 0x00fd_d070),
    (0x00fb_a790, 0x00fd_d0a0),
    (0x00fb_a940, 0x00fd_d190),
    (0x00fb_a950, 0x00fd_d1c0),
    (0x00fb_a960, 0x00fd_d1f0),
    (0x00fb_a970, 0x00fd_d220),
    (0x00fb_aaf0, 0x00fd_d250),
    (0x00fb_ab20, 0x00fd_d280),
    (0x00fb_ac50, 0x00fd_d2b0),
    (0x00fb_ac60, 0x00fd_d2e0),
    (0x00fb_b130, 0x00fd_d420),
    (0x00fb_cd40, 0x00fd_de30),
    (0x00fb_dc70, 0x00fd_e220),
    (0x00fb_ffd0, 0x00fd_e2b0),
    (0x00fc_7190, 0x00fd_e2d0),
    (0x00fc_72c0, 0x00fd_e320),
    (0x00fc_7400, 0x00fd_e340),
    (0x00fc_78bc, 0x00fd_e54f),
];

// Translated from 00fc793c (disassembled, FalloutNV.exe 1.4.0.525)
/// `_atexit(00fde581)`.
pub fn fn_00fc793c(e: &mut Engine) {
    e.call(ATEXIT, &args![0x00fd_e581u32]);
}

// Translated from 00fb5500 (disassembled, FalloutNV.exe 1.4.0.525)
/// `_atexit(00fdae50)`.
pub fn fn_00fb5500(e: &mut Engine) {
    e.call(ATEXIT, &args![0x00fd_ae50u32]);
}

// Translated from 00fb56b0 (disassembled, FalloutNV.exe 1.4.0.525)
/// `_atexit(00fdae70)`.
pub fn fn_00fb56b0(e: &mut Engine) {
    e.call(ATEXIT, &args![0x00fd_ae70u32]);
}

// Translated from 00fb5750 (disassembled, FalloutNV.exe 1.4.0.525)
/// `_atexit(00fdaea0)`.
pub fn fn_00fb5750(e: &mut Engine) {
    e.call(ATEXIT, &args![0x00fd_aea0u32]);
}

// Translated from 00fb5780 (disassembled, FalloutNV.exe 1.4.0.525)
/// `_atexit(00fdaed0)`.
pub fn fn_00fb5780(e: &mut Engine) {
    e.call(ATEXIT, &args![0x00fd_aed0u32]);
}

// Translated from 00fb5990 (disassembled, FalloutNV.exe 1.4.0.525)
/// `_atexit(00fdafb0)`.
pub fn fn_00fb5990(e: &mut Engine) {
    e.call(ATEXIT, &args![0x00fd_afb0u32]);
}

// Translated from 00fb5ae0 (disassembled, FalloutNV.exe 1.4.0.525)
/// `_atexit(00fdb000)`.
pub fn fn_00fb5ae0(e: &mut Engine) {
    e.call(ATEXIT, &args![0x00fd_b000u32]);
}

// Translated from 00fb5bf0 (disassembled, FalloutNV.exe 1.4.0.525)
/// `_atexit(00fdb030)`.
pub fn fn_00fb5bf0(e: &mut Engine) {
    e.call(ATEXIT, &args![0x00fd_b030u32]);
}

// Translated from 00fb5c40 (disassembled, FalloutNV.exe 1.4.0.525)
/// `_atexit(00fdb060)`.
pub fn fn_00fb5c40(e: &mut Engine) {
    e.call(ATEXIT, &args![0x00fd_b060u32]);
}

// Translated from 00fb5db0 (disassembled, FalloutNV.exe 1.4.0.525)
/// `_atexit(00fdb090)`.
pub fn fn_00fb5db0(e: &mut Engine) {
    e.call(ATEXIT, &args![0x00fd_b090u32]);
}

// Translated from 00fb6150 (disassembled, FalloutNV.exe 1.4.0.525)
/// `_atexit(00fdb0d0)`.
pub fn fn_00fb6150(e: &mut Engine) {
    e.call(ATEXIT, &args![0x00fd_b0d0u32]);
}

// Translated from 00fb6180 (disassembled, FalloutNV.exe 1.4.0.525)
/// `_atexit(00fdb100)`.
pub fn fn_00fb6180(e: &mut Engine) {
    e.call(ATEXIT, &args![0x00fd_b100u32]);
}

// Translated from 00fb6340 (disassembled, FalloutNV.exe 1.4.0.525)
/// `_atexit(00fdb1a0)`.
pub fn fn_00fb6340(e: &mut Engine) {
    e.call(ATEXIT, &args![0x00fd_b1a0u32]);
}

// Translated from 00fb64d0 (disassembled, FalloutNV.exe 1.4.0.525)
/// `_atexit(00fdb1d0)`.
pub fn fn_00fb64d0(e: &mut Engine) {
    e.call(ATEXIT, &args![0x00fd_b1d0u32]);
}

// Translated from 00fb6520 (disassembled, FalloutNV.exe 1.4.0.525)
/// `_atexit(00fdb200)`.
pub fn fn_00fb6520(e: &mut Engine) {
    e.call(ATEXIT, &args![0x00fd_b200u32]);
}

// Translated from 00fb65d0 (disassembled, FalloutNV.exe 1.4.0.525)
/// `_atexit(00fdb270)`.
pub fn fn_00fb65d0(e: &mut Engine) {
    e.call(ATEXIT, &args![0x00fd_b270u32]);
}

// Translated from 00fb9510 (disassembled, FalloutNV.exe 1.4.0.525)
/// `_atexit(00fdc750)`.
pub fn fn_00fb9510(e: &mut Engine) {
    e.call(ATEXIT, &args![0x00fd_c750u32]);
}

// Translated from 00fb95f0 (disassembled, FalloutNV.exe 1.4.0.525)
/// `_atexit(00fdcb10)`.
pub fn fn_00fb95f0(e: &mut Engine) {
    e.call(ATEXIT, &args![0x00fd_cb10u32]);
}

// Translated from 00fb9600 (disassembled, FalloutNV.exe 1.4.0.525)
/// `_atexit(00fdc7b0)`.
pub fn fn_00fb9600(e: &mut Engine) {
    e.call(ATEXIT, &args![0x00fd_c7b0u32]);
}

// Translated from 00fb9610 (disassembled, FalloutNV.exe 1.4.0.525)
/// `_atexit(00fdc7e0)`.
pub fn fn_00fb9610(e: &mut Engine) {
    e.call(ATEXIT, &args![0x00fd_c7e0u32]);
}

// Translated from 00fb9620 (disassembled, FalloutNV.exe 1.4.0.525)
/// `_atexit(00fdc810)`.
pub fn fn_00fb9620(e: &mut Engine) {
    e.call(ATEXIT, &args![0x00fd_c810u32]);
}

// Translated from 00fb9630 (disassembled, FalloutNV.exe 1.4.0.525)
/// `_atexit(00fdc840)`.
pub fn fn_00fb9630(e: &mut Engine) {
    e.call(ATEXIT, &args![0x00fd_c840u32]);
}

// Translated from 00fb9640 (disassembled, FalloutNV.exe 1.4.0.525)
/// `_atexit(00fdc870)`.
pub fn fn_00fb9640(e: &mut Engine) {
    e.call(ATEXIT, &args![0x00fd_c870u32]);
}

// Translated from 00fb9650 (disassembled, FalloutNV.exe 1.4.0.525)
/// `_atexit(00fdc8a0)`.
pub fn fn_00fb9650(e: &mut Engine) {
    e.call(ATEXIT, &args![0x00fd_c8a0u32]);
}

// Translated from 00fb9660 (disassembled, FalloutNV.exe 1.4.0.525)
/// `_atexit(00fdc8d0)`.
pub fn fn_00fb9660(e: &mut Engine) {
    e.call(ATEXIT, &args![0x00fd_c8d0u32]);
}

// Translated from 00fb9670 (disassembled, FalloutNV.exe 1.4.0.525)
/// `_atexit(00fdc900)`.
pub fn fn_00fb9670(e: &mut Engine) {
    e.call(ATEXIT, &args![0x00fd_c900u32]);
}

// Translated from 00fb9680 (disassembled, FalloutNV.exe 1.4.0.525)
/// `_atexit(00fdc930)`.
pub fn fn_00fb9680(e: &mut Engine) {
    e.call(ATEXIT, &args![0x00fd_c930u32]);
}

// Translated from 00fb9690 (disassembled, FalloutNV.exe 1.4.0.525)
/// `_atexit(00fdc960)`.
pub fn fn_00fb9690(e: &mut Engine) {
    e.call(ATEXIT, &args![0x00fd_c960u32]);
}

// Translated from 00fb96a0 (disassembled, FalloutNV.exe 1.4.0.525)
/// `_atexit(00fdc990)`.
pub fn fn_00fb96a0(e: &mut Engine) {
    e.call(ATEXIT, &args![0x00fd_c990u32]);
}

// Translated from 00fb96b0 (disassembled, FalloutNV.exe 1.4.0.525)
/// `_atexit(00fdc9c0)`.
pub fn fn_00fb96b0(e: &mut Engine) {
    e.call(ATEXIT, &args![0x00fd_c9c0u32]);
}

// Translated from 00fb96c0 (disassembled, FalloutNV.exe 1.4.0.525)
/// `_atexit(00fdc9f0)`.
pub fn fn_00fb96c0(e: &mut Engine) {
    e.call(ATEXIT, &args![0x00fd_c9f0u32]);
}

// Translated from 00fb96d0 (disassembled, FalloutNV.exe 1.4.0.525)
/// `_atexit(00fdca20)`.
pub fn fn_00fb96d0(e: &mut Engine) {
    e.call(ATEXIT, &args![0x00fd_ca20u32]);
}

// Translated from 00fb9710 (disassembled, FalloutNV.exe 1.4.0.525)
/// `_atexit(00fdca50)`.
pub fn fn_00fb9710(e: &mut Engine) {
    e.call(ATEXIT, &args![0x00fd_ca50u32]);
}

// Translated from 00fb9720 (disassembled, FalloutNV.exe 1.4.0.525)
/// `_atexit(00fdca80)`.
pub fn fn_00fb9720(e: &mut Engine) {
    e.call(ATEXIT, &args![0x00fd_ca80u32]);
}

// Translated from 00fb9730 (disassembled, FalloutNV.exe 1.4.0.525)
/// `_atexit(00fdcb60)`.
pub fn fn_00fb9730(e: &mut Engine) {
    e.call(ATEXIT, &args![0x00fd_cb60u32]);
}

// Translated from 00fb9760 (disassembled, FalloutNV.exe 1.4.0.525)
/// `_atexit(00fdcab0)`.
pub fn fn_00fb9760(e: &mut Engine) {
    e.call(ATEXIT, &args![0x00fd_cab0u32]);
}

// Translated from 00fb9770 (disassembled, FalloutNV.exe 1.4.0.525)
/// `_atexit(00fdcae0)`.
pub fn fn_00fb9770(e: &mut Engine) {
    e.call(ATEXIT, &args![0x00fd_cae0u32]);
}

// Translated from 00fb9780 (disassembled, FalloutNV.exe 1.4.0.525)
/// `_atexit(00fdcb90)`.
pub fn fn_00fb9780(e: &mut Engine) {
    e.call(ATEXIT, &args![0x00fd_cb90u32]);
}

// Translated from 00fb9790 (disassembled, FalloutNV.exe 1.4.0.525)
/// `_atexit(00fdcbc0)`.
pub fn fn_00fb9790(e: &mut Engine) {
    e.call(ATEXIT, &args![0x00fd_cbc0u32]);
}

// Translated from 00fb9950 (disassembled, FalloutNV.exe 1.4.0.525)
/// `_atexit(00fdcca0)`.
pub fn fn_00fb9950(e: &mut Engine) {
    e.call(ATEXIT, &args![0x00fd_cca0u32]);
}

// Translated from 00fb9c70 (disassembled, FalloutNV.exe 1.4.0.525)
/// `_atexit(00fdcda0)`.
pub fn fn_00fb9c70(e: &mut Engine) {
    e.call(ATEXIT, &args![0x00fd_cda0u32]);
}

// Translated from 00fba020 (disassembled, FalloutNV.exe 1.4.0.525)
/// `_atexit(00fdce40)`.
pub fn fn_00fba020(e: &mut Engine) {
    e.call(ATEXIT, &args![0x00fd_ce40u32]);
}

// Translated from 00fba030 (disassembled, FalloutNV.exe 1.4.0.525)
/// `_atexit(00fdce80)`.
pub fn fn_00fba030(e: &mut Engine) {
    e.call(ATEXIT, &args![0x00fd_ce80u32]);
}

// Translated from 00fba100 (disassembled, FalloutNV.exe 1.4.0.525)
/// `_atexit(00fdcef0)`.
pub fn fn_00fba100(e: &mut Engine) {
    e.call(ATEXIT, &args![0x00fd_cef0u32]);
}

// Translated from 00fba2d0 (disassembled, FalloutNV.exe 1.4.0.525)
/// `_atexit(00fdcf20)`.
pub fn fn_00fba2d0(e: &mut Engine) {
    e.call(ATEXIT, &args![0x00fd_cf20u32]);
}

// Translated from 00fba6d0 (disassembled, FalloutNV.exe 1.4.0.525)
/// `_atexit(00fdcf90)`.
pub fn fn_00fba6d0(e: &mut Engine) {
    e.call(ATEXIT, &args![0x00fd_cf90u32]);
}

// Translated from 00fba700 (disassembled, FalloutNV.exe 1.4.0.525)
/// `_atexit(00fdcfc0)`.
pub fn fn_00fba700(e: &mut Engine) {
    e.call(ATEXIT, &args![0x00fd_cfc0u32]);
}

// Translated from 00fba750 (disassembled, FalloutNV.exe 1.4.0.525)
/// `_atexit(00fdcfe0)`.
pub fn fn_00fba750(e: &mut Engine) {
    e.call(ATEXIT, &args![0x00fd_cfe0u32]);
}

// Translated from 00fba760 (disassembled, FalloutNV.exe 1.4.0.525)
/// `_atexit(00fdd010)`.
pub fn fn_00fba760(e: &mut Engine) {
    e.call(ATEXIT, &args![0x00fd_d010u32]);
}

// Translated from 00fba770 (disassembled, FalloutNV.exe 1.4.0.525)
/// `_atexit(00fdd040)`.
pub fn fn_00fba770(e: &mut Engine) {
    e.call(ATEXIT, &args![0x00fd_d040u32]);
}

// Translated from 00fba780 (disassembled, FalloutNV.exe 1.4.0.525)
/// `_atexit(00fdd070)`.
pub fn fn_00fba780(e: &mut Engine) {
    e.call(ATEXIT, &args![0x00fd_d070u32]);
}

// Translated from 00fba790 (disassembled, FalloutNV.exe 1.4.0.525)
/// `_atexit(00fdd0a0)`.
pub fn fn_00fba790(e: &mut Engine) {
    e.call(ATEXIT, &args![0x00fd_d0a0u32]);
}

// Translated from 00fba940 (disassembled, FalloutNV.exe 1.4.0.525)
/// `_atexit(00fdd190)`.
pub fn fn_00fba940(e: &mut Engine) {
    e.call(ATEXIT, &args![0x00fd_d190u32]);
}

// Translated from 00fba950 (disassembled, FalloutNV.exe 1.4.0.525)
/// `_atexit(00fdd1c0)`.
pub fn fn_00fba950(e: &mut Engine) {
    e.call(ATEXIT, &args![0x00fd_d1c0u32]);
}

// Translated from 00fba960 (disassembled, FalloutNV.exe 1.4.0.525)
/// `_atexit(00fdd1f0)`.
pub fn fn_00fba960(e: &mut Engine) {
    e.call(ATEXIT, &args![0x00fd_d1f0u32]);
}

// Translated from 00fba970 (disassembled, FalloutNV.exe 1.4.0.525)
/// `_atexit(00fdd220)`.
pub fn fn_00fba970(e: &mut Engine) {
    e.call(ATEXIT, &args![0x00fd_d220u32]);
}

// Translated from 00fbaaf0 (disassembled, FalloutNV.exe 1.4.0.525)
/// `_atexit(00fdd250)`.
pub fn fn_00fbaaf0(e: &mut Engine) {
    e.call(ATEXIT, &args![0x00fd_d250u32]);
}

// Translated from 00fbab20 (disassembled, FalloutNV.exe 1.4.0.525)
/// `_atexit(00fdd280)`.
pub fn fn_00fbab20(e: &mut Engine) {
    e.call(ATEXIT, &args![0x00fd_d280u32]);
}

// Translated from 00fbac50 (disassembled, FalloutNV.exe 1.4.0.525)
/// `_atexit(00fdd2b0)`.
pub fn fn_00fbac50(e: &mut Engine) {
    e.call(ATEXIT, &args![0x00fd_d2b0u32]);
}

// Translated from 00fbac60 (disassembled, FalloutNV.exe 1.4.0.525)
/// `_atexit(00fdd2e0)`.
pub fn fn_00fbac60(e: &mut Engine) {
    e.call(ATEXIT, &args![0x00fd_d2e0u32]);
}

// Translated from 00fbb130 (disassembled, FalloutNV.exe 1.4.0.525)
/// `_atexit(00fdd420)`.
pub fn fn_00fbb130(e: &mut Engine) {
    e.call(ATEXIT, &args![0x00fd_d420u32]);
}

// Translated from 00fbcd40 (disassembled, FalloutNV.exe 1.4.0.525)
/// `_atexit(00fdde30)`.
pub fn fn_00fbcd40(e: &mut Engine) {
    e.call(ATEXIT, &args![0x00fd_de30u32]);
}

// Translated from 00fbdc70 (disassembled, FalloutNV.exe 1.4.0.525)
/// `_atexit(00fde220)`.
pub fn fn_00fbdc70(e: &mut Engine) {
    e.call(ATEXIT, &args![0x00fd_e220u32]);
}

// Translated from 00fbffd0 (disassembled, FalloutNV.exe 1.4.0.525)
/// `_atexit(00fde2b0)`.
pub fn fn_00fbffd0(e: &mut Engine) {
    e.call(ATEXIT, &args![0x00fd_e2b0u32]);
}

// Translated from 00fc7190 (disassembled, FalloutNV.exe 1.4.0.525)
/// `_atexit(00fde2d0)`.
pub fn fn_00fc7190(e: &mut Engine) {
    e.call(ATEXIT, &args![0x00fd_e2d0u32]);
}

// Translated from 00fc72c0 (disassembled, FalloutNV.exe 1.4.0.525)
/// `_atexit(00fde320)`.
pub fn fn_00fc72c0(e: &mut Engine) {
    e.call(ATEXIT, &args![0x00fd_e320u32]);
}

// Translated from 00fc7400 (disassembled, FalloutNV.exe 1.4.0.525)
/// `_atexit(00fde340)`.
pub fn fn_00fc7400(e: &mut Engine) {
    e.call(ATEXIT, &args![0x00fd_e340u32]);
}

// Translated from 00fc78bc (disassembled, FalloutNV.exe 1.4.0.525)
/// `_atexit(00fde54f)`.
pub fn fn_00fc78bc(e: &mut Engine) {
    e.call(ATEXIT, &args![0x00fd_e54fu32]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00fc_793c, fn_00fc793c()),
        entry!(0x00fb_5500, fn_00fb5500()),
        entry!(0x00fb_56b0, fn_00fb56b0()),
        entry!(0x00fb_5750, fn_00fb5750()),
        entry!(0x00fb_5780, fn_00fb5780()),
        entry!(0x00fb_5990, fn_00fb5990()),
        entry!(0x00fb_5ae0, fn_00fb5ae0()),
        entry!(0x00fb_5bf0, fn_00fb5bf0()),
        entry!(0x00fb_5c40, fn_00fb5c40()),
        entry!(0x00fb_5db0, fn_00fb5db0()),
        entry!(0x00fb_6150, fn_00fb6150()),
        entry!(0x00fb_6180, fn_00fb6180()),
        entry!(0x00fb_6340, fn_00fb6340()),
        entry!(0x00fb_64d0, fn_00fb64d0()),
        entry!(0x00fb_6520, fn_00fb6520()),
        entry!(0x00fb_65d0, fn_00fb65d0()),
        entry!(0x00fb_9510, fn_00fb9510()),
        entry!(0x00fb_95f0, fn_00fb95f0()),
        entry!(0x00fb_9600, fn_00fb9600()),
        entry!(0x00fb_9610, fn_00fb9610()),
        entry!(0x00fb_9620, fn_00fb9620()),
        entry!(0x00fb_9630, fn_00fb9630()),
        entry!(0x00fb_9640, fn_00fb9640()),
        entry!(0x00fb_9650, fn_00fb9650()),
        entry!(0x00fb_9660, fn_00fb9660()),
        entry!(0x00fb_9670, fn_00fb9670()),
        entry!(0x00fb_9680, fn_00fb9680()),
        entry!(0x00fb_9690, fn_00fb9690()),
        entry!(0x00fb_96a0, fn_00fb96a0()),
        entry!(0x00fb_96b0, fn_00fb96b0()),
        entry!(0x00fb_96c0, fn_00fb96c0()),
        entry!(0x00fb_96d0, fn_00fb96d0()),
        entry!(0x00fb_9710, fn_00fb9710()),
        entry!(0x00fb_9720, fn_00fb9720()),
        entry!(0x00fb_9730, fn_00fb9730()),
        entry!(0x00fb_9760, fn_00fb9760()),
        entry!(0x00fb_9770, fn_00fb9770()),
        entry!(0x00fb_9780, fn_00fb9780()),
        entry!(0x00fb_9790, fn_00fb9790()),
        entry!(0x00fb_9950, fn_00fb9950()),
        entry!(0x00fb_9c70, fn_00fb9c70()),
        entry!(0x00fb_a020, fn_00fba020()),
        entry!(0x00fb_a030, fn_00fba030()),
        entry!(0x00fb_a100, fn_00fba100()),
        entry!(0x00fb_a2d0, fn_00fba2d0()),
        entry!(0x00fb_a6d0, fn_00fba6d0()),
        entry!(0x00fb_a700, fn_00fba700()),
        entry!(0x00fb_a750, fn_00fba750()),
        entry!(0x00fb_a760, fn_00fba760()),
        entry!(0x00fb_a770, fn_00fba770()),
        entry!(0x00fb_a780, fn_00fba780()),
        entry!(0x00fb_a790, fn_00fba790()),
        entry!(0x00fb_a940, fn_00fba940()),
        entry!(0x00fb_a950, fn_00fba950()),
        entry!(0x00fb_a960, fn_00fba960()),
        entry!(0x00fb_a970, fn_00fba970()),
        entry!(0x00fb_aaf0, fn_00fbaaf0()),
        entry!(0x00fb_ab20, fn_00fbab20()),
        entry!(0x00fb_ac50, fn_00fbac50()),
        entry!(0x00fb_ac60, fn_00fbac60()),
        entry!(0x00fb_b130, fn_00fbb130()),
        entry!(0x00fb_cd40, fn_00fbcd40()),
        entry!(0x00fb_dc70, fn_00fbdc70()),
        entry!(0x00fb_ffd0, fn_00fbffd0()),
        entry!(0x00fc_7190, fn_00fc7190()),
        entry!(0x00fc_72c0, fn_00fc72c0()),
        entry!(0x00fc_7400, fn_00fc7400()),
        entry!(0x00fc_78bc, fn_00fc78bc()),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn each_gives_its_destructor_to_atexit() {
        for (entry, destructor) in AT_EXIT_ONLY {
            let mut e = Engine::new();
            e.register(ATEXIT, |_, _| Ret::default());
            e.call_log = Some(vec![]);
            e.call(entry, &args![]);
            let calls: Vec<(u32, Vec<u32>)> = e
                .call_log
                .take()
                .unwrap()
                .into_iter()
                .filter(|(a, _)| *a != entry)
                .collect();
            assert_eq!(calls, vec![(ATEXIT, vec![destructor])], "{entry:08x}");
        }
        assert_eq!(funcs().len(), AT_EXIT_ONLY.len());
    }
}

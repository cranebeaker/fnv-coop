//! The C runtime's floating-point to text conversions (MSVC 2008 LIBCMT, statically linked
//! into FalloutNV.exe 1.4.0.525), translated from the exe: `__gcvt_s` (`00ecdbf6`, the body
//! `_gcvt` `00ecdd16` wraps), the `%e` and `%f` builders it picks between, `__cftoe`
//! (`00ed1d87`) and `__cftof` (`00ed22cc`) with their locale forms `__cftoe_l` (`00ed1cb7`),
//! `__cftof_l` (`00ed2211`) and the layout halves `__cftoe2_l` (`00ed1b48`) and `__cftof2_l`
//! (`00ed211a`), the rounding to a digit count `__fptostr` (`00ee18cb`), the in-place string
//! move `__shift` (`00ed1b03`), and the digit generation: `__fltout2` (`00edf36b`), which widens
//! the double to the 80-bit form with `___dtold` (`00edf2ae`) and has `$I10_OUTPUT`
//! (`00ee47cb`) produce the decimal digits. The names are Ghidra's function-ID labels.
//!
//! `$I10_OUTPUT` is the exe's own decimal conversion. It works on the CRT's 12-byte numbers
//! (a 16-bit rounding word, an 80-bit significand with the binary point above its top bit,
//! and a 15-bit exponent biased by `0x3ffe` with the sign above it): it estimates the decimal
//! exponent from the binary one, scales by powers of ten from the tables at `011c2cb0`
//! (10^1..10^7, 10^8..10^56, 10^64..10^448, 10^512..10^3584) and `011c2e10` (their
//! reciprocals), multiplies by 0.1 once more when the result is 1 or more, and then takes one
//! digit per multiplication by ten of the 88-bit fraction. The 12-byte multiplication is
//! inlined twice in the exe with the same instructions on different registers; it is written
//! once here ([`ld12_mul`]) on a model of the frame ([`Frame`]) at the frame's own offsets,
//! because the code reads its numbers through overlapping word and dword accesses.
//!
//! Not modelled: the `/GS` cookie checks (`00ec408c`) of `__gcvt_s`, `__fltout2`,
//! `$I10_OUTPUT`, `__cftoe_l` and `__cftof_l`.

use super::*;

/// `_LocaleUpdate::_LocaleUpdate` (`00ec660e`, thiscall: the object, the locale or 0), `_errno`
/// (`00ec85e3`), the invalid-parameter report (`00ec7c30`, five words), `__invoke_watson`
/// (`00ec7b08`, five words, does not return), `strcpy_s` (`00ec65a6`), `memmove`
/// (`00ec7230`), `memset` (`00ec61c0`) and `strlen` (`00ec6130`): translated elsewhere.
const LOCALE_UPDATE_ADDR: u32 = 0x00ec_660e;
const ERRNO_ADDR: u32 = 0x00ec_85e3;
const INVALID_PARAMETER_ADDR: u32 = 0x00ec_7c30;
const INVOKE_WATSON_ADDR: u32 = 0x00ec_7b08;
const STRCPY_S_ADDR: u32 = 0x00ec_65a6;
const MEMMOVE_ADDR: u32 = 0x00ec_7230;
const MEMSET_ADDR: u32 = 0x00ec_61c0;
const STRLEN_ADDR: u32 = 0x00ec_6130;
/// The functions of this file the exe calls (`CALL`).
const FLTOUT2: u32 = 0x00ed_f36b;
const DTOLD: u32 = 0x00ed_f2ae;
const I10_OUTPUT: u32 = 0x00ee_47cb;
const FPTOSTR: u32 = 0x00ee_18cb;
const CFTOF: u32 = 0x00ed_22cc;
const CFTOF_L: u32 = 0x00ed_2211;
const CFTOF2_L: u32 = 0x00ed_211a;
const CFTOE: u32 = 0x00ed_1d87;
const CFTOE_L: u32 = 0x00ed_1cb7;
const CFTOE2_L: u32 = 0x00ed_1b48;
const SHIFT: u32 = 0x00ed_1b03;

/// The power-of-ten tables `$I10_OUTPUT` walks (12-byte numbers, seven to a group): the
/// positive powers from `011c2cb0` and the negative ones from `011c2e10`. The code starts each
/// walk 0x60 bytes before them and steps 0x54 (a group) before the first entry.
const POW10_POSITIVE: u32 = 0x011c_2cb0;
const POW10_NEGATIVE: u32 = 0x011c_2e10;
/// The texts `$I10_OUTPUT` copies for an exponent of all ones: `"1#SNAN"` (`011008ac`),
/// `"1#IND"` (`011008a4`), `"1#INF"` (`0110089c`), `"1#QNAN"` (`01100894`).
const TEXT_SNAN: u32 = 0x0110_08ac;
const TEXT_IND: u32 = 0x0110_08a4;
const TEXT_INF: u32 = 0x0110_089c;
const TEXT_QNAN: u32 = 0x0110_0894;
/// `"e+000"` (`010f2a48`), the exponent `__cftoe2_l` fills in.
const EXPONENT_TEMPLATE: u32 = 0x010f_2a48;
/// The byte `__cftoe2_l` tests (bit 0 set: a three-digit exponent with a leading zero loses
/// it; the `_set_output_format` flag, no label in the map).
const OUTPUT_FORMAT: u32 = 0x0127_0800;

/// The `_strflt` the conversions fill (`__fltout2`) and read: the sign (`'-'` or `' '`, a
/// dword), the decimal point position, the flag `$I10_OUTPUT` answers, the digits' address.
const FLT_SIGN: u32 = 0;
const FLT_DECPT: u32 = 4;
const FLT_FLAG: u32 = 8;
const FLT_MANTISSA: u32 = 0xc;

/// `*_errno() = code` then the invalid-parameter report with five zeros, the exe's order.
fn report(e: &mut Engine, code: u32) {
    let slot = e.call(ERRNO_ADDR, &[]).u32();
    e.mem.set_u32(slot, code);
    e.call(INVALID_PARAMETER_ADDR, &[0, 0, 0, 0, 0]);
}

/// `__invoke_watson(0, 0, 0, 0, 0)` when a `strcpy_s` answered nonzero (it does not return).
fn watson_unless_zero(e: &mut Engine, result: u32) {
    if result != 0 {
        e.call(INVOKE_WATSON_ADDR, &[0, 0, 0, 0, 0]);
    }
}

/// The decimal point of the locale the `_LocaleUpdate` object at `update` holds: its locale
/// info (+0), that one's `lconv` (+0xbc), its `decimal_point` (+0), the first character.
fn decimal_point(e: &Engine, update: u32) -> u8 {
    let info = e.mem.u32(update);
    let lconv = e.mem.u32(info + 0xbc);
    let point = e.mem.u32(lconv);
    e.mem.u8(point)
}

// Translated from 00ecdbf6 (disassembled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ecdbf6 - the /GS stack cookie check (00ec408c) is not translated
/// `__gcvt_s` (cdecl: the buffer, its size, the double, the significant digits; answers an
/// `errno_t`). Takes the thread's locale (`_LocaleUpdate`, the object at `EBP-0x2c`). A null
/// buffer or a size of 0 sets `errno` to `EINVAL` (0x16), reports an invalid parameter and
/// answers 0x16; otherwise the buffer's first byte is cleared and a digit count that is not
/// below the size gives `ERANGE` (0x22) the same way. Then `__fltout2` (the `_strflt` at
/// `EBP-0x3c`, its 0x16-byte digit buffer at `EBP-0x1c`) gives the decimal point position:
/// with the exponent (`decpt - 1`) below -1 or above `digits - 1` the text is `__cftoe` with
/// `digits - 1` decimals (small `e`), otherwise `__cftof` with `digits - decpt` decimals; both
/// get the address of the double argument. A nonzero answer is stored in `errno` and
/// answered. Otherwise trailing zeros are removed from the digits after the decimal point
/// (the locale's character), up to an `e` or the end, the rest moved down; the point itself
/// stays.
pub fn gcvt_s(e: &mut Engine, buffer: Ptr, size: u32, value: f64, digits: u32) -> i32 {
    // The frame from EBP-0x3c up to the arguments: the `_strflt` (EBP-0x3c), the
    // `_LocaleUpdate` object (EBP-0x2c), the digit buffer (EBP-0x1c), the cookie (EBP-0x4),
    // the saved EBP and return address, and the arguments from EBP+0x8 (the double at
    // EBP+0x10, whose address the conversions get).
    e.with_stack(0x54, |e, frame| {
        let ebp = frame.addr() + 0x3c;
        let value_at = ebp + 0x10;
        e.mem.set_u64(value_at, value.to_bits());
        let flt = ebp - 0x3c;
        let update = ebp - 0x2c;
        let digit_buffer = ebp - 0x1c;
        let to = buffer.addr();
        e.call(LOCALE_UPDATE_ADDR, &[update, 0]);
        let result = 'body: {
            // CMP EDI,EBX; JNZ, then CMP [EBP+0xc],EBX; JBE.
            if to == 0 || size == 0 {
                report(e, 0x16);
                break 'body 0x16;
            }
            e.mem.set_u8(to, 0);
            // CMP ESI,[EBP+0xc]; JC: unsigned.
            if digits >= size {
                report(e, 0x22);
                break 'body 0x22;
            }
            let answered = e
                .call(
                    FLTOUT2,
                    &[
                        value.to_bits() as u32,
                        (value.to_bits() >> 32) as u32,
                        flt,
                        digit_buffer,
                        0x16,
                    ],
                )
                .u32();
            let decpt = e.mem.u32(answered + FLT_DECPT) as i32;
            let exponent = decpt.wrapping_sub(1);
            let count = digits as i32;
            // CMP ECX,-1; JL, then CMP ECX,EDX; JG: signed.
            let error = if exponent < -1 || exponent > count.wrapping_sub(1) {
                e.call(
                    CFTOE,
                    &[value_at, to, size, count.wrapping_sub(1) as u32, 0],
                )
                .u32()
            } else {
                e.call(
                    CFTOF,
                    &[value_at, to, size, count.wrapping_sub(decpt) as u32],
                )
                .u32()
            };
            if error != 0 {
                let slot = e.call(ERRNO_ADDR, &[]).u32();
                e.mem.set_u32(slot, error);
                break 'body error;
            }
            // 00ecdca9: find the decimal point (or the end).
            let mut at = to;
            let mut c = e.mem.u8(at);
            if c != 0 {
                let point = decimal_point(e, update);
                while c != point {
                    at = at.wrapping_add(1);
                    c = e.mem.u8(at);
                    if c == 0 {
                        break;
                    }
                }
            }
            // 00ecdcc9: past the point; nothing to do without one.
            let c = e.mem.u8(at);
            at = at.wrapping_add(1);
            if c == 0 {
                break 'body 0;
            }
            // 00ecdcd8: on to an `e` or the end.
            loop {
                let c = e.mem.u8(at);
                if c == 0 || c == b'e' {
                    break;
                }
                at = at.wrapping_add(1);
            }
            // 00ecdcde: back over the zeros, then move the rest (with its terminator) down.
            let mut from = at;
            let mut last = at;
            loop {
                last = last.wrapping_sub(1);
                if e.mem.u8(last) != b'0' {
                    break;
                }
            }
            loop {
                let c = e.mem.u8(from);
                last = last.wrapping_add(1);
                from = from.wrapping_add(1);
                e.mem.set_u8(last, c);
                if c == 0 {
                    break;
                }
            }
            0
        };
        locale_update_release(e, update);
        result as i32
    })
}

// Translated from 00edf36b (disassembled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00edf36b - the /GS stack cookie check (00ec408c) is not translated
/// `__fltout2` (cdecl: the double as two words, the `_strflt`, the digit buffer, its size;
/// answers the `_strflt`). `___dtold` widens the double (the argument's own slot) to the
/// 80-bit form at `EBP-0x10`; `$I10_OUTPUT` gets its ten bytes as three words (the upper half
/// of the third is the stack slot's other half, the upper half of this frame's `EBP` that
/// `___dtold`'s `PUSH EBP` left there; `$I10_OUTPUT` copies only ten bytes), 17 digits, flags
/// 0 and the `_FloatOutStruct` at `EBP-0x2c`. The `_strflt` gets the answer as its flag, the
/// sign byte and the decimal exponent sign-extended (`MOVSX`), and the digits are copied with
/// `strcpy_s` into the buffer (a failure is `__invoke_watson`), which becomes its mantissa.
pub fn fltout2(e: &mut Engine, low: u32, high: u32, flt: Ptr, result: Ptr, size: u32) -> Ptr {
    // The frame from EBP-0x30 up to the double at EBP+0x8.
    e.with_stack(0x40, |e, frame| {
        let ebp = frame.addr() + 0x30;
        let value_at = ebp + 8;
        e.mem.set_u32(value_at, low);
        e.mem.set_u32(value_at + 4, high);
        let wide = ebp - 0x10;
        let fos = ebp - 0x2c;
        e.call(DTOLD, &[wide, value_at]);
        let third = u32::from(e.mem.u16(wide + 8)) | (ebp & 0xffff_0000);
        let words = [e.mem.u32(wide), e.mem.u32(wide + 4), third, 0x11, 0, fos];
        let flag = e.call(I10_OUTPUT, &words).u32();
        let to = flt.addr();
        e.mem.set_u32(to + FLT_FLAG, flag);
        e.mem
            .set_u32(to + FLT_SIGN, e.mem.i8(fos + 2) as i32 as u32);
        e.mem.set_u32(to + FLT_DECPT, e.mem.i16(fos) as i32 as u32);
        let copied = e.call(STRCPY_S_ADDR, &[result.addr(), size, fos + 4]).u32();
        watson_unless_zero(e, copied);
        e.mem.set_u32(to + FLT_MANTISSA, result.addr());
        flt
    })
}

// Translated from 00edf2ae (disassembled, FalloutNV.exe 1.4.0.525)
/// `___dtold` (cdecl: the 10-byte result, the double; answers the result's address). The
/// double's exponent is rebiased by `0x3c00` with the integer bit set; an exponent of all ones
/// becomes `0x7fff` (integer bit set); a zero exponent with a zero fraction gives zero with the
/// sign; a denormal gets `0x3c01` without the integer bit and is shifted up until the top bit
/// is set, the exponent counting down (16-bit). The significand is the fraction shifted up by
/// 11; the sign goes into the exponent word (+8).
pub fn dtold(e: &mut Engine, out: Ptr, input: Ptr) -> Ptr {
    let from = input.addr();
    let to = out.addr();
    let top = u32::from(e.mem.u16(from + 6));
    let sign = top & 0x8000;
    let exponent = (top >> 4) & 0x7ff;
    let high = e.mem.u32(from + 4) & 0xfffff;
    let low = e.mem.u32(from);
    let mut integer_bit = 0x8000_0000u32;
    let mut biased: u32;
    if exponent == 0 {
        if high == 0 && low == 0 {
            e.mem.set_u32(to + 4, 0);
            e.mem.set_u32(to, 0);
            e.mem.set_u16(to + 8, sign as u16);
            return out;
        }
        // 00edf316
        biased = (exponent + 0x3c01) & 0xffff;
        integer_bit = 0;
    } else if exponent == 0x7ff {
        biased = 0x7fff;
    } else {
        biased = (exponent + 0x3c00) & 0xffff;
    }
    // 00edf322
    let mut significand_high = (low >> 21) | (high << 11) | integer_bit;
    let mut significand_low = low << 11;
    e.mem.set_u32(to + 4, significand_high);
    e.mem.set_u32(to, significand_low);
    if significand_high & 0x8000_0000 == 0 {
        loop {
            significand_high = (significand_high << 1) | (significand_low >> 31);
            significand_low <<= 1;
            // ADD EDI,0xffff: the low word counts down.
            biased = biased.wrapping_add(0xffff);
            e.mem.set_u32(to + 4, significand_high);
            e.mem.set_u32(to, significand_low);
            if significand_high & 0x8000_0000 != 0 {
                break;
            }
        }
    }
    e.mem.set_u16(to + 8, (sign | biased) as u16);
    out
}

/// `$I10_OUTPUT`'s frame, `EBP-0x74` up to `EBP` (the cookie at `EBP-0x4`), addressed by the
/// `EBP` offsets the code uses. The bytes the code never writes start as zero; it only reads
/// them in the halves of dwords it masks off (`EBP-0x6`, `EBP-0x14`, `EBP-0x24`).
struct Frame([u8; 0x74]);

impl Frame {
    fn at(offset: i32) -> usize {
        (0x74 + offset) as usize
    }

    fn u8(&self, offset: i32) -> u8 {
        self.0[Self::at(offset)]
    }

    fn set_u8(&mut self, offset: i32, value: u8) {
        self.0[Self::at(offset)] = value;
    }

    fn u16(&self, offset: i32) -> u16 {
        let i = Self::at(offset);
        u16::from_le_bytes([self.0[i], self.0[i + 1]])
    }

    fn set_u16(&mut self, offset: i32, value: u16) {
        let i = Self::at(offset);
        self.0[i..i + 2].copy_from_slice(&value.to_le_bytes());
    }

    fn u32(&self, offset: i32) -> u32 {
        let i = Self::at(offset);
        u32::from_le_bytes([self.0[i], self.0[i + 1], self.0[i + 2], self.0[i + 3]])
    }

    fn set_u32(&mut self, offset: i32, value: u32) {
        let i = Self::at(offset);
        self.0[i..i + 4].copy_from_slice(&value.to_le_bytes());
    }

    fn twelve(&self, offset: i32) -> [u8; 12] {
        let i = Self::at(offset);
        let mut out = [0u8; 12];
        out.copy_from_slice(&self.0[i..i + 12]);
        out
    }

    fn set_twelve(&mut self, offset: i32, value: &[u8; 12]) {
        let i = Self::at(offset);
        self.0[i..i + 12].copy_from_slice(value);
    }

    /// The three dwords from `offset` as one 96-bit number.
    fn u96(&self, offset: i32) -> u128 {
        u128::from(self.u32(offset))
            | u128::from(self.u32(offset + 4)) << 32
            | u128::from(self.u32(offset + 8)) << 64
    }

    fn set_u96(&mut self, offset: i32, value: u128) {
        self.set_u32(offset, value as u32);
        self.set_u32(offset + 4, (value >> 32) as u32);
        self.set_u32(offset + 8, (value >> 64) as u32);
    }

    /// `SHL [o],1` with the carries into `[o+4]` and `[o+8]` (`SHR ..,0x1f; ADD; OR`): the
    /// 96-bit number one bit up, the top bit lost.
    fn shift_left(&mut self, offset: i32) {
        let v = self.u96(offset);
        self.set_u96(offset, (v << 1) & MASK_96);
    }

    /// `SHR [o+8],1` with the bits carried down into `[o+4]` and `[o]` (`SHL ..,0x1f; OR`).
    fn shift_right(&mut self, offset: i32) {
        let v = self.u96(offset);
        self.set_u96(offset, v >> 1);
    }
}

const MASK_96: u128 = (1 << 96) - 1;

/// The 12-byte multiplication `$I10_OUTPUT` inlines (`00ee49d4`-`00ee4cd8` for the powers
/// of ten, `00ee4c3b`-`00ee4ed8` for 0.1, the same instructions on other registers): the
/// number at `EBP-0x20` times `m` (its rounding word at +0, the significand from +2, the sign
/// and exponent at +0xa), the product built at `EBP-0x10`.
///
/// The exponents' sum (16-bit, each without its sign) at `0x7fff` or more in either operand
/// or above `0xbffd` gives infinity with the product's sign (the dword at `EBP-0x18`
/// `0x7fff8000` or `0xffff8000`, the two below zero); at `0x3fbf` or less zero. An exponent of
/// 0 counts one more, and a zero operand gives zero (the number's case clears only its
/// exponent word; its significand is zero already). Otherwise the significand words are
/// multiplied in five rows: row `i` adds the 32-bit products of the number's word `i + j` and
/// the multiplier's word `4 - j` into the dword at `EBP-0x10 + 2i`, a carry incrementing the
/// word above it (row 4's would be the cookie's low word at `EBP-0x4`: the sum of the rows
/// never carries out of the product's six words, the truncated product of two 80-bit
/// significands being below 2^96 in those units). The exponent less `0x3ffe` (`ADD ..,0xc002`)
/// is normalised: while positive and the product's top bit is clear, one bit up and one less;
/// then, unless still positive, one less again and, when that is negative, the product moves
/// down to an exponent of 0, a 1 shifted out of the rounding word's low bit at any step
/// leaving that bit set. The rounding word above `0x8000`, or exactly `0x8000` with the
/// significand's low bit set, rounds up (a carry out of the top word gives `0x8000` and one
/// more in the exponent). An exponent below `0x7fff` stores the significand (`EBP-0xe`, ten
/// bytes) and the exponent with the sign; otherwise infinity.
fn ld12_mul(f: &mut Frame, m: &[u8; 12]) {
    let word = |i: usize| u16::from_le_bytes([m[2 * i], m[2 * i + 1]]);
    let dword = |o: usize| u32::from_le_bytes([m[o], m[o + 1], m[o + 2], m[o + 3]]);
    let multiplier_exponent = word(5);
    let mut sticky = 0u32;
    f.set_u32(-0x10, 0);
    f.set_u32(-0xc, 0);
    f.set_u32(-0x8, 0);
    // MOV ECX,dword [EBP-0x16]: the upper half (EBP-0x14) is masked off.
    let number_exponent = f.u16(-0x16);
    let sign = (multiplier_exponent ^ number_exponent) & 0x8000;
    let a = number_exponent & 0x7fff;
    let b = multiplier_exponent & 0x7fff;
    let mut exponent = a + b;
    let infinity = |f: &mut Frame| {
        f.set_u32(-0x18, if sign == 0 { 0x7fff_8000 } else { 0xffff_8000 });
        f.set_u32(-0x1c, 0);
        f.set_u32(-0x20, 0);
    };
    let zero = |f: &mut Frame| {
        f.set_u32(-0x18, 0);
        f.set_u32(-0x1c, 0);
        f.set_u32(-0x20, 0);
    };
    if a >= 0x7fff || b >= 0x7fff || exponent > 0xbffd {
        infinity(f);
        return;
    }
    if exponent <= 0x3fbf {
        zero(f);
        return;
    }
    if a == 0 {
        exponent += 1;
        if f.u32(-0x18) & 0x7fff_ffff == 0 && f.u32(-0x1c) == 0 && f.u32(-0x20) == 0 {
            f.set_u16(-0x16, 0);
            return;
        }
    }
    if b == 0 {
        // Unreachable for the 0.1 multiplication (exponent 0x3ffb), as Ghidra notes; the
        // tables hold no zero either.
        exponent += 1;
        if dword(8) & 0x7fff_ffff == 0 && dword(4) == 0 && dword(0) == 0 {
            zero(f);
            return;
        }
    }
    // The rows; the carry word of row 4 is the cookie's low word (EBP-0x4).
    for i in 0..5i32 {
        for j in 0..(5 - i) {
            let product = u32::from(f.u16(-0x20 + 2 * (i + j))) * u32::from(word((4 - j) as usize));
            let at = -0x10 + 2 * i;
            let before = f.u32(at);
            let sum = before.wrapping_add(product);
            f.set_u32(at, sum);
            if sum < before || sum < product {
                let carry_at = -0xc + 2 * i;
                f.set_u16(carry_at, f.u16(carry_at).wrapping_add(1));
            }
        }
    }
    exponent = exponent.wrapping_add(0xc002);
    let mut normalised = false;
    if exponent as i16 > 0 {
        loop {
            if f.u32(-0x8) & 0x8000_0000 != 0 {
                break;
            }
            f.shift_left(-0x10);
            exponent = exponent.wrapping_add(0xffff);
            if exponent as i16 <= 0 {
                break;
            }
        }
        normalised = exponent as i16 > 0;
    }
    if !normalised {
        exponent = exponent.wrapping_add(0xffff);
        if (exponent as i16) < 0 {
            let steps = exponent.wrapping_neg();
            exponent = exponent.wrapping_add(steps);
            for _ in 0..steps {
                if f.u8(-0x10) & 1 != 0 {
                    sticky += 1;
                }
                f.shift_right(-0x10);
            }
            if sticky != 0 {
                f.set_u16(-0x10, f.u16(-0x10) | 1);
            }
        }
    }
    // 00ee4b94: rounding.
    if f.u16(-0x10) > 0x8000 || f.u32(-0x10) & 0x1ffff == 0x18000 {
        if f.u32(-0xe) == 0xffff_ffff {
            f.set_u32(-0xe, 0);
            if f.u32(-0xa) == 0xffff_ffff {
                f.set_u32(-0xa, 0);
                if f.u16(-0x6) == 0xffff {
                    f.set_u16(-0x6, 0x8000);
                    exponent = exponent.wrapping_add(1);
                } else {
                    f.set_u16(-0x6, f.u16(-0x6).wrapping_add(1));
                }
            } else {
                f.set_u32(-0xa, f.u32(-0xa).wrapping_add(1));
            }
        } else {
            f.set_u32(-0xe, f.u32(-0xe).wrapping_add(1));
        }
    }
    if exponent < 0x7fff {
        f.set_u16(-0x20, f.u16(-0xe));
        f.set_u32(-0x1e, f.u32(-0xc));
        f.set_u32(-0x1a, f.u32(-0x8));
        f.set_u16(-0x16, exponent | sign);
    } else {
        infinity(f);
    }
}

/// The zero answer of `$I10_OUTPUT` (`00ee485e`, `00ee4ef4`, `00ee50c3`): decimal exponent 0,
/// the sign `'-'` for a negative input (`CMP ..,0x8000; SETNZ; DEC; AND 0xd; ADD 0x20`), one
/// digit `'0'` at `digit` (the first digit place) and the terminator at +5.
fn zero_digits(e: &mut Engine, fos: u32, sign: u16, digit: u32) {
    e.mem.set_u16(fos, 0);
    e.mem
        .set_u8(fos + 2, if sign == 0x8000 { b'-' } else { b' ' });
    e.mem.set_u8(fos + 3, 1);
    e.mem.set_u8(digit, b'0');
    e.mem.set_u8(fos + 5, 0);
}

// Translated from 00ee47cb (disassembled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ee47cb - the /GS stack cookie check (00ec408c) is not translated
/// `$I10_OUTPUT` (cdecl: a 12-byte number as three words, of which ten bytes are read, the
/// digit count, flags, the `_FloatOutStruct`; answers 1, or 0 for an infinity or NaN). The
/// struct gets the decimal exponent (word +0), the sign (`'-'` or `' '`, +2), the digit count
/// (+3) and the digits with a terminator (+4).
///
/// Zero gives `"0"`. An exponent of all ones gives `"1#SNAN"` (integer bit and quiet bit clear
/// unless it is the plain infinity), `"1#IND"` (negative, `0xc0000000` and a zero low dword),
/// `"1#INF"` (`0x80000000` and zero) or `"1#QNAN"`, copied with `strcpy_s` (a failure is
/// `__invoke_watson`), with the exponent 1. Otherwise the decimal exponent is estimated from
/// the binary one and the top byte of the significand (`((e >> 8) + (hi >> 24) * 2) * 0x4d +
/// e * 0x4d10 - 0x134312f4`, arithmetic shift by 16, 16-bit), the number (without its sign)
/// is multiplied by ten to the minus that through the tables, three bits of the power at a
/// time (an entry whose rounding word is `0x8000` or more is copied to `EBP-0x3c` and has the
/// dword at its +2 decreased by one first), and by 0.1 (`EBP-0x30`) when it is 1 or more, the
/// exponent one more. With flags bit 0 the digit count is added to the exponent, and a count
/// of 0 or less gives `"0"`. At most 21 digits plus one more are produced, each the top byte
/// of the 88-bit fraction times ten; the last decides the rounding: below `'5'` (signed) it
/// goes with any trailing zeros (all zeros give `"0"`), otherwise the digits round up, a carry
/// past the first making it `'1'` with one more in the exponent. The count is the last digit's
/// place less the struct's address less 3 (byte arithmetic).
pub fn i10_output(
    e: &mut Engine,
    word0: u32,
    word1: u32,
    word2: u32,
    digits: i32,
    flags: u32,
    fos: Ptr,
) -> i32 {
    let mut f = Frame([0; 0x74]);
    // MOVSD; MOVSD; MOVSW: ten bytes of the argument to EBP-0x10.
    f.set_u32(-0x10, word0);
    f.set_u32(-0xc, word1);
    f.set_u16(-0x8, word2 as u16);
    // MOV EDX,dword [EBP-0x8]: the upper half (EBP-0x6) is masked off.
    let sign_exponent = f.u16(-0x8);
    let sign = sign_exponent & 0x8000;
    let exponent = sign_exponent & 0x7fff;
    let fos = fos.addr();
    // 0.1: ten bytes 0xcc and the exponent 0x3ffb (EBP-0x30).
    for offset in -0x30..-0x26 {
        f.set_u8(offset, 0xcc);
    }
    f.set_u16(-0x26, 0x3ffb);
    let answer = 1; // EBP-0x74
    e.mem.set_u8(fos + 2, if sign != 0 { b'-' } else { b' ' });
    let high = f.u32(-0xc);
    let low = f.u32(-0x10);
    if exponent == 0 && high == 0 && low == 0 {
        zero_digits(e, fos, sign, fos + 4);
        return 1;
    }
    if exponent == 0x7fff {
        e.mem.set_u16(fos, 1);
        let infinity = high == 0x8000_0000 && low == 0;
        let (text, length) = if !infinity && high & 0x4000_0000 == 0 {
            (TEXT_SNAN, 6)
        } else if sign != 0 && high == 0xc000_0000 {
            if low != 0 {
                (TEXT_QNAN, 6)
            } else {
                (TEXT_IND, 5)
            }
        } else if infinity {
            (TEXT_INF, 5)
        } else {
            (TEXT_QNAN, 6)
        };
        let copied = e.call(STRCPY_S_ADDR, &[fos + 4, 0x16, text]).u32();
        watson_unless_zero(e, copied);
        e.mem.set_u8(fos + 3, length);
        return 0;
    }
    // 00ee4932: the decimal exponent's estimate (EBP-0x4c, zero-extended).
    let estimate = (u32::from(exponent) >> 8)
        .wrapping_add((high >> 24) * 2)
        .wrapping_mul(0x4d)
        .wrapping_add(u32::from(exponent).wrapping_mul(0x4d10))
        .wrapping_add(0xecbc_ed0c);
    let mut decimal = ((estimate as i32) >> 16) as u32 & 0xffff;
    f.set_u16(-0x20, 0);
    f.set_u16(-0x16, exponent);
    f.set_u32(-0x1a, high);
    f.set_u32(-0x1e, low);
    let mut table = POW10_POSITIVE - 0x60;
    let mut power = (decimal as u16 as i16 as i32).wrapping_neg();
    if power != 0 {
        if power < 0 {
            table = POW10_NEGATIVE - 0x60;
            power = power.wrapping_neg();
        }
        while power != 0 {
            table += 0x54;
            let index = (power & 7) as u32;
            power >>= 3;
            if index != 0 {
                let entry = table + index * 12;
                let mut m = [0u8; 12];
                e.mem.read(entry, &mut m);
                // CMP word [EAX],0x8000; JC
                if e.mem.u16(entry) >= 0x8000 {
                    f.set_twelve(-0x3c, &m);
                    f.set_u32(-0x3a, f.u32(-0x3a).wrapping_sub(1));
                    m = f.twelve(-0x3c);
                }
                ld12_mul(&mut f, &m);
            }
        }
    }
    // 00ee4c1f: 1 or more is multiplied by 0.1.
    if f.u16(-0x16) >= 0x3fff {
        decimal = decimal.wrapping_add(1);
        let m = f.twelve(-0x30);
        ld12_mul(&mut f, &m);
    }
    // 00ee4edb
    e.mem.set_u16(fos, decimal as u16);
    let mut count = digits;
    if flags & 1 != 0 {
        count = count.wrapping_add(i32::from(decimal as u16 as i16));
        if count <= 0 {
            zero_digits(e, fos, sign, fos + 4);
            return 1;
        }
    }
    if count > 0x15 {
        count = 0x15;
    }
    // The binary point moves to the top byte: eight bits up, then down by the exponent's
    // distance below 0x3ffe (the low byte of it).
    let distance = u32::from(f.u16(-0x16)).wrapping_sub(0x3ffe) as i32;
    f.set_u16(-0x16, 0);
    for _ in 0..8 {
        f.shift_left(-0x20);
    }
    if distance < 0 {
        let steps = distance.wrapping_neg() & 0xff;
        for _ in 0..steps {
            f.shift_right(-0x20);
        }
    }
    // 00ee4f9e: the digits. Each step copies the number to EBP-0x3c and forms ten times it
    // from four times plus once (with the carries) and once more up (all mod 2^96); the top
    // byte is the digit and is cleared.
    let start = fos + 4;
    let mut at = start;
    let mut left = count.wrapping_add(1);
    if left > 0 {
        loop {
            let v = f.u96(-0x20);
            let copy = f.twelve(-0x20);
            f.set_twelve(-0x3c, &copy);
            let five = (((v << 2) & MASK_96) + v) & MASK_96;
            let ten = (five << 1) & MASK_96;
            f.set_u96(-0x20, ten);
            e.mem.set_u8(at, ((ten >> 88) as u8).wrapping_add(0x30));
            at = at.wrapping_add(1);
            left -= 1;
            f.set_u8(-0x15, 0);
            if left <= 0 {
                break;
            }
        }
    }
    // 00ee5067: the last digit rounds.
    let mut last = at.wrapping_sub(1);
    let rounding = e.mem.i8(last);
    last = last.wrapping_sub(1);
    if rounding < 0x35 {
        while last >= start && e.mem.u8(last) == b'0' {
            last = last.wrapping_sub(1);
        }
        if last < start {
            zero_digits(e, fos, sign, start);
            return 1;
        }
    } else {
        while last >= start {
            if e.mem.u8(last) != b'9' {
                break;
            }
            e.mem.set_u8(last, b'0');
            last = last.wrapping_sub(1);
        }
        if last < start {
            last = last.wrapping_add(1);
            e.mem.set_u16(fos, e.mem.u16(fos).wrapping_add(1));
        }
        e.mem.set_u8(last, e.mem.u8(last).wrapping_add(1));
    }
    // 00ee5090
    let length = (last as u8).wrapping_sub(fos as u8).wrapping_sub(3);
    e.mem.set_u8(fos + 3, length);
    let end = (length as i8 as i32 as u32)
        .wrapping_add(fos)
        .wrapping_add(4);
    e.mem.set_u8(end, 0);
    answer
}

// Translated from 00ee18cb (disassembled, FalloutNV.exe 1.4.0.525)
/// `__fptostr` (cdecl: the buffer, its size, the digit count, the `_strflt`; answers an
/// `errno_t`). A null buffer or size 0 is `EINVAL` (0x16) with the invalid-parameter report;
/// otherwise the first byte is cleared and a size not above `max(count, 0) + 1` is `ERANGE`
/// (0x22) the same way. The buffer gets `'0'` (the carry place) and `count` digits of the
/// mantissa, `'0'` once it runs out, and the terminator; with a count of 0 or more and the
/// next mantissa digit `'5'` or above (signed) the digits round up through the nines. A carry
/// into the first place (`'1'`) adds one to the decimal point position; otherwise the digits
/// move down over it (`memmove` of `strlen + 1` bytes).
pub fn fptostr(e: &mut Engine, buffer: Ptr, size: u32, count: i32, flt: Ptr) -> i32 {
    let to = buffer.addr();
    let flt = flt.addr();
    let mut mantissa = e.mem.u32(flt + FLT_MANTISSA);
    if to == 0 || size == 0 {
        report(e, 0x16);
        return 0x16;
    }
    e.mem.set_u8(to, 0);
    let need = (if count > 0 { count } else { 0 }).wrapping_add(1) as u32;
    // CMP [EBP+0xc],EAX; JA: unsigned.
    if size <= need {
        report(e, 0x22);
        return 0x22;
    }
    e.mem.set_u8(to, b'0');
    let mut at = to.wrapping_add(1);
    let mut left = count;
    if left > 0 {
        loop {
            let c = e.mem.u8(mantissa);
            let c = if c != 0 {
                mantissa = mantissa.wrapping_add(1);
                c
            } else {
                b'0'
            };
            e.mem.set_u8(at, c);
            at = at.wrapping_add(1);
            left -= 1;
            if left <= 0 {
                break;
            }
        }
    }
    e.mem.set_u8(at, 0);
    if left >= 0 && e.mem.i8(mantissa) >= 0x35 {
        loop {
            at = at.wrapping_sub(1);
            if e.mem.u8(at) != b'9' {
                break;
            }
            e.mem.set_u8(at, b'0');
        }
        e.mem.set_u8(at, e.mem.u8(at).wrapping_add(1));
    }
    if e.mem.u8(to) == b'1' {
        e.mem
            .set_u32(flt + FLT_DECPT, e.mem.u32(flt + FLT_DECPT).wrapping_add(1));
    } else {
        let rest = to.wrapping_add(1);
        let length = e.call(STRLEN_ADDR, &[rest]).u32();
        e.call(MEMMOVE_ADDR, &[to, rest, length.wrapping_add(1)]);
    }
    0
}

// Translated from 00ed22cc (disassembled, FalloutNV.exe 1.4.0.525)
/// `__cftof` (cdecl: the double's address, the buffer, its size, the decimals): `__cftof_l`
/// with locale 0.
pub fn cftof(e: &mut Engine, value: Ptr, buffer: Ptr, size: u32, decimals: i32) -> i32 {
    e.call(
        CFTOF_L,
        &[value.addr(), buffer.addr(), size, decimals as u32, 0],
    )
    .i32()
}

// Translated from 00ed2211 (disassembled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ed2211 - the /GS stack cookie check (00ec408c) is not translated
/// `__cftof_l` (cdecl: the double's address, the buffer, its size, the decimals, the locale).
/// `__fltout2` fills the `_strflt` (`EBP-0x2c`) and its digits (`EBP-0x1c`, 0x16 bytes); then
/// a null buffer or size 0 is `EINVAL` (0x16) with the invalid-parameter report. `__fptostr`
/// rounds to `decpt + decimals` digits into the buffer after the sign's place (the size less
/// that place, unless it is -1); its error clears the buffer's first byte and is answered.
/// Otherwise `__cftof2_l` lays the text out (no `%g` flag).
pub fn cftof_l(
    e: &mut Engine,
    value: Ptr,
    buffer: Ptr,
    size: u32,
    decimals: i32,
    locale: u32,
) -> i32 {
    e.with_stack(0x2c, |e, frame| {
        let flt = frame.addr();
        let digit_buffer = flt + 0x10;
        let from = value.addr();
        let words = [
            e.mem.u32(from),
            e.mem.u32(from + 4),
            flt,
            digit_buffer,
            0x16,
        ];
        e.call(FLTOUT2, &words);
        let to = buffer.addr();
        if to == 0 || size == 0 {
            report(e, 0x16);
            return 0x16;
        }
        let room = if size == u32::MAX {
            size
        } else {
            size.wrapping_sub(u32::from(e.mem.u32(flt + FLT_SIGN) == 0x2d))
        };
        let count = e.mem.u32(flt + FLT_DECPT).wrapping_add(decimals as u32);
        let place = u32::from(e.mem.u32(flt + FLT_SIGN) == 0x2d).wrapping_add(to);
        let error = e.call(FPTOSTR, &[place, room, count, flt]).u32();
        if error != 0 {
            e.mem.set_u8(to, 0);
            return error as i32;
        }
        e.call(CFTOF2_L, &[flt, to, size, decimals as u32, 0, locale])
            .i32()
    })
}

// Translated from 00ed211a (disassembled, FalloutNV.exe 1.4.0.525)
/// `__cftof2_l` (`EAX` the `_strflt`, `ECX` the buffer; on the stack the size, the decimals,
/// the `%g` flag byte and the locale; answers an `errno_t`), the `%f` layout of digits
/// `__fptostr` left in the buffer after the sign's place. Takes the locale (`_LocaleUpdate`,
/// `EBP-0x10`). A null buffer or size 0 is `EINVAL` (0x16) with the invalid-parameter report.
/// With the flag and `decpt - 1` equal to the decimals, `"0"` is appended after the digits.
/// A negative number gets its `'-'`; a point position of 0 or less inserts a `'0'` before the
/// digits (`__shift` by one), else the position is skipped. With decimals above 0 the
/// locale's decimal point is inserted there and, for a negative position, `-decpt` zeros
/// (with the flag, or when not more than the decimals; otherwise the decimals) after it.
pub fn cftof2_l(
    e: &mut Engine,
    flt: u32,
    buffer: u32,
    size: u32,
    decimals: i32,
    g_flag: u32,
    locale: u32,
) -> i32 {
    e.with_stack(0x10, |e, frame| {
        let update = frame.addr();
        let place = e.mem.u32(flt + FLT_DECPT).wrapping_sub(1);
        e.call(LOCALE_UPDATE_ADDR, &[update, locale]);
        if buffer == 0 || size == 0 {
            report(e, 0x16);
            locale_update_release(e, update);
            return 0x16;
        }
        let g_flag = g_flag as u8 != 0;
        if g_flag && place == decimals as u32 {
            let at = u32::from(e.mem.u32(flt + FLT_SIGN) == 0x2d)
                .wrapping_add(place)
                .wrapping_add(buffer);
            e.mem.set_u8(at, b'0');
            e.mem.set_u8(at.wrapping_add(1), 0);
        }
        let mut at = buffer;
        if e.mem.u32(flt + FLT_SIGN) == 0x2d {
            e.mem.set_u8(buffer, b'-');
            at = buffer.wrapping_add(1);
        }
        let decpt = e.mem.u32(flt + FLT_DECPT);
        if decpt as i32 > 0 {
            at = at.wrapping_add(decpt);
        } else {
            e.call(SHIFT, &[at, 1]);
            e.mem.set_u8(at, b'0');
            at = at.wrapping_add(1);
        }
        let mut decimals = decimals;
        if decimals > 0 {
            e.call(SHIFT, &[at, 1]);
            let point = decimal_point(e, update);
            e.mem.set_u8(at, point);
            let position = e.mem.u32(flt + FLT_DECPT) as i32;
            at = at.wrapping_add(1);
            if position < 0 {
                let zeros = position.wrapping_neg();
                // CMP byte [EBP+0x10],0; JNZ, then CMP [EBP+0xc],EBX; JL.
                if g_flag || decimals >= zeros {
                    decimals = zeros;
                }
                e.call(SHIFT, &[at, decimals as u32]);
                e.call(MEMSET_ADDR, &[at, 0x30, decimals as u32]);
            }
        }
        locale_update_release(e, update);
        0
    })
}

// Translated from 00ed1d87 (disassembled, FalloutNV.exe 1.4.0.525)
/// `__cftoe` (cdecl: the double's address, the buffer, its size, the decimals, capitals):
/// `__cftoe_l` with locale 0.
pub fn cftoe(
    e: &mut Engine,
    value: Ptr,
    buffer: Ptr,
    size: u32,
    decimals: i32,
    capitals: u32,
) -> i32 {
    e.call(
        CFTOE_L,
        &[
            value.addr(),
            buffer.addr(),
            size,
            decimals as u32,
            capitals,
            0,
        ],
    )
    .i32()
}

// Translated from 00ed1cb7 (disassembled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ed1cb7 - the /GS stack cookie check (00ec408c) is not translated
/// `__cftoe_l` (cdecl: the double's address, the buffer, its size, the decimals, capitals, the
/// locale). `__fltout2` fills the `_strflt` (`EBP-0x2c`) and its digits (`EBP-0x1c`, 0x16
/// bytes); then a null buffer or size 0 is `EINVAL` (0x16) with the invalid-parameter report.
/// `__fptostr` rounds to `decimals + 1` digits into the buffer after the sign's place and,
/// with decimals above 0, the point's (the size less those places, unless it is -1); its error
/// clears the buffer's first byte and is answered. Otherwise `__cftoe2_l` lays the text out
/// (no `%g` flag).
pub fn cftoe_l(
    e: &mut Engine,
    value: Ptr,
    buffer: Ptr,
    size: u32,
    decimals: i32,
    capitals: u32,
    locale: u32,
) -> i32 {
    e.with_stack(0x2c, |e, frame| {
        let flt = frame.addr();
        let digit_buffer = flt + 0x10;
        let from = value.addr();
        let words = [
            e.mem.u32(from),
            e.mem.u32(from + 4),
            flt,
            digit_buffer,
            0x16,
        ];
        e.call(FLTOUT2, &words);
        let to = buffer.addr();
        if to == 0 || size == 0 {
            report(e, 0x16);
            return 0x16;
        }
        let point = u32::from(decimals > 0);
        let room = if size == u32::MAX {
            u32::MAX
        } else {
            size.wrapping_sub(u32::from(e.mem.u32(flt + FLT_SIGN) == 0x2d))
                .wrapping_sub(point)
        };
        let place = u32::from(e.mem.u32(flt + FLT_SIGN) == 0x2d)
            .wrapping_add(to)
            .wrapping_add(point);
        let count = (decimals as u32).wrapping_add(1);
        let error = e.call(FPTOSTR, &[place, room, count, flt]).u32();
        if error != 0 {
            e.mem.set_u8(to, 0);
            return error as i32;
        }
        e.call(
            CFTOE2_L,
            &[to, size, decimals as u32, capitals, flt, 0, locale],
        )
        .i32()
    })
}

// Translated from 00ed1b48 (disassembled, FalloutNV.exe 1.4.0.525)
/// `__cftoe2_l` (`EAX` the buffer; on the stack the size, the decimals, capitals, the
/// `_strflt`, the `%g` flag byte and the locale; answers an `errno_t`), the `%e` layout of the
/// digits `__fptostr` left in the buffer after the sign's and point's places. Takes the
/// locale (`_LocaleUpdate`, `EBP-0x10`). A null buffer or size 0 is `EINVAL` (0x16), a size not
/// above `max(decimals, 0) + 9` `ERANGE` (0x22), each with the invalid-parameter report. With
/// the flag the digits first move up by the point's place (`__shift`). A negative number gets
/// its `'-'`; with decimals the first digit moves down into the point's place and the locale's
/// decimal point follows it. After the decimals (and one more place without the flag)
/// `"e+000"` is copied with `strcpy_s` (the room left of the size, or -1; a failure is
/// `__invoke_watson`), `'E'` for capitals; unless the first digit is `'0'` the exponent
/// `decpt - 1` fills it (`'-'` for a negative one, hundreds, tens and units added to the
/// template's digits). With bit 0 of the output-format byte (`01270800`) a leading `'0'` of
/// the three exponent digits is removed (`memmove` of 3 bytes).
pub fn cftoe2_l(
    e: &mut Engine,
    buffer: u32,
    size: u32,
    decimals: i32,
    capitals: u32,
    flt: u32,
    g_flag: u32,
    locale: u32,
) -> i32 {
    e.with_stack(0x10, |e, frame| {
        let update = frame.addr();
        e.call(LOCALE_UPDATE_ADDR, &[update, locale]);
        let fail = |e: &mut Engine, code: u32| {
            report(e, code);
            locale_update_release(e, update);
            code as i32
        };
        if buffer == 0 || size == 0 {
            return fail(e, 0x16);
        }
        let need = (if decimals > 0 { decimals } else { 0 }).wrapping_add(9) as u32;
        // CMP [EBP+0x8],EAX; JA: unsigned.
        if size <= need {
            return fail(e, 0x22);
        }
        let g_flag = g_flag as u8 != 0;
        if g_flag {
            let at = u32::from(e.mem.u32(flt + FLT_SIGN) == 0x2d).wrapping_add(buffer);
            e.call(SHIFT, &[at, u32::from(decimals > 0)]);
        }
        let mut at = buffer;
        if e.mem.u32(flt + FLT_SIGN) == 0x2d {
            e.mem.set_u8(buffer, b'-');
            at = buffer.wrapping_add(1);
        }
        if decimals > 0 {
            let next = at.wrapping_add(1);
            e.mem.set_u8(at, e.mem.u8(next));
            at = next;
            let point = decimal_point(e, update);
            e.mem.set_u8(at, point);
        }
        at = at
            .wrapping_add(u32::from(!g_flag))
            .wrapping_add(decimals as u32);
        let room = if size == u32::MAX {
            u32::MAX
        } else {
            buffer.wrapping_sub(at).wrapping_add(size)
        };
        let copied = e.call(STRCPY_S_ADDR, &[at, room, EXPONENT_TEMPLATE]).u32();
        watson_unless_zero(e, copied);
        let exponent_digits = at.wrapping_add(2);
        if capitals != 0 {
            e.mem.set_u8(at, b'E');
        }
        let mantissa = e.mem.u32(flt + FLT_MANTISSA);
        at = at.wrapping_add(1);
        if e.mem.u8(mantissa) != b'0' {
            let mut exponent = (e.mem.u32(flt + FLT_DECPT) as i32).wrapping_sub(1);
            if exponent < 0 {
                exponent = exponent.wrapping_neg();
                e.mem.set_u8(at, b'-');
            }
            at = at.wrapping_add(1);
            if exponent >= 100 {
                e.mem
                    .set_u8(at, e.mem.u8(at).wrapping_add((exponent / 100) as u8));
                exponent %= 100;
            }
            at = at.wrapping_add(1);
            if exponent >= 10 {
                e.mem
                    .set_u8(at, e.mem.u8(at).wrapping_add((exponent / 10) as u8));
                exponent %= 10;
            }
            let units = at.wrapping_add(1);
            e.mem
                .set_u8(units, e.mem.u8(units).wrapping_add(exponent as u8));
        }
        if e.mem.u8(OUTPUT_FORMAT) & 1 != 0 && e.mem.u8(exponent_digits) == b'0' {
            e.call(
                MEMMOVE_ADDR,
                &[exponent_digits, exponent_digits.wrapping_add(1), 3],
            );
        }
        locale_update_release(e, update);
        0
    })
}

// Translated from 00ed1b03 (disassembled, FalloutNV.exe 1.4.0.525)
/// `__shift` (`EAX` the string, `EDI` the distance; answers `EAX`): with a distance, the
/// string and its terminator move up by it (`memmove(s + n, s, strlen(s) + 1)`, whose answer
/// is left in `EAX`); without one nothing happens and `EAX` is the string.
pub fn shift(e: &mut Engine, text: u32, distance: u32) -> u32 {
    if distance == 0 {
        return text;
    }
    let length = e.call(STRLEN_ADDR, &[text]).u32();
    e.call(
        MEMMOVE_ADDR,
        &[text.wrapping_add(distance), text, length.wrapping_add(1)],
    )
    .u32()
}

// ---- Text to double, and the x87 double to 64-bit integer -------------------------------------

/// `_isctype_l` (`00ed41e7`, cdecl: character, mask, `_LocaleUpdate`) and `_fltin2`
/// (`00edb0db`, cdecl: the `_flt` to fill, string, length, 0, 0, `_LocaleUpdate`; answers the
/// `_flt`, its double at `+0x10`).
const ISCTYPE_L_ADDR: u32 = 0x00ed_41e7;
const FLTIN2: u32 = 0x00ed_b0db;
/// `_SPACE`, the ctype bit `__atof_l` skips.
const CTYPE_SPACE: u32 = 8;

// Translated from 00eca4c8 (disassembled, FalloutNV.exe 1.4.0.525)
/// `__atof_l` (Ghidra's function ID), cdecl: (string, locale), the result in `ST0`. A
/// `_LocaleUpdate` for the locale is made on the stack. A null string sets `errno` `EINVAL`
/// (0x16), calls the invalid-parameter handler and gives 0.0. Otherwise leading white space is
/// skipped (`_SPACE`: through `_isctype_l` when the locale's `mb_cur_max` (+0xac, signed) is
/// above 1, else from its ctype table (+0xc8, a word per byte)), and `_fltin2(&flt, string,
/// strlen(string), 0, 0, &update)` converts the rest: its double (`+0x10`) is the result
/// (`FLD`). Either way an update that set the thread's own-locale bit clears it again (bit 1 of
/// the thread data's +0x70).
pub fn atof_l(e: &mut Engine, string: Ptr, locale: u32) -> f64 {
    // The frame from EBP-0x28: the 0x18-byte `_flt`, then the `_LocaleUpdate` at EBP-0x10.
    e.with_stack(0x28, |e, frame| {
        let flt = frame.addr();
        let update = flt + 0x18;
        e.call(LOCALE_UPDATE_ADDR, &args![update, locale]);
        let mut at = string.addr();
        let value = if at == 0 {
            report(e, 0x16);
            0.0
        } else {
            loop {
                let info = e.mem.u32(update);
                let c = u32::from(e.mem.u8(at));
                let space = if e.mem.i32(info.wrapping_add(0xac)) > 1 {
                    e.call(ISCTYPE_L_ADDR, &args![c, CTYPE_SPACE, update]).u32()
                } else {
                    let table = e.mem.u32(info.wrapping_add(0xc8));
                    u32::from(e.mem.u16(table.wrapping_add(2 * c))) & CTYPE_SPACE
                };
                if space == 0 {
                    break;
                }
                at = at.wrapping_add(1);
            }
            let length = e.call(STRLEN_ADDR, &args![at]).u32();
            let result = e
                .call(FLTIN2, &args![flt, at, length, 0u32, 0u32, update])
                .u32();
            x87_load64(e.mem.u64(result.wrapping_add(0x10)))
        };
        if e.mem.u8(update + 0xc) != 0 {
            let thread = e.mem.u32(update + 8);
            let own = e.mem.u32(thread.wrapping_add(0x70));
            e.mem.set_u32(thread.wrapping_add(0x70), own & !2);
        }
        value
    })
}

/// `FILD qword`: the 64-bit integer `n` in a register, exact (the 64-bit significand holds it).
/// The x87 module has no helper for it.
fn fild64(n: i64) -> X87Ext {
    if n == 0 {
        return X87Ext::from_parts(0, 0);
    }
    let sign = if n < 0 { 0x8000 } else { 0 };
    let magnitude = n.unsigned_abs();
    let zeros = magnitude.leading_zeros();
    X87Ext::from_parts(sign | (0x3fff + 63 - zeros) as u16, magnitude << zeros)
}

// Translated from 00ec62f6 (disassembled, FalloutNV.exe 1.4.0.525)
/// `_ftol2`'s x87 body (unnamed in the engine map; the entry `00ec62c0` jumps here without
/// SSE2): `ST0` in, the 64-bit integer in `EDX:EAX` out, `ST0` popped. The value is stored as
/// a float (`FST`; only its sign is read) and as an integer rounded by the control word
/// (`FISTP qword`). A low word of 0 with a high word of 0 or 0x80000000 (zero, or the integer
/// indefinite) is the answer as it is, and with a low word of 0 the sign tested below is the
/// high word's rather than the float's. Otherwise the integer is loaded back (`FILD`) and the
/// value minus it (`FSUBP`) stored as a float: for a negative sign, a positive nonzero
/// difference adds 1; for a non-negative sign, a negative nonzero difference takes 1 off (the
/// carry of `ADD ECX,0x7fffffff` on the float bits, the sign bit flipped first for a negative
/// value): the value truncated toward zero.
pub fn ftol2_x87(_e: &mut Engine, value: f64) -> u64 {
    let st0 = X87Ext::load64(value);
    let float_bits = st0.store32();
    let integer = x87_fistp64(st0);
    let bits = integer as u64;
    let (low, high) = (bits as u32, (bits >> 32) as u32);
    let mut sign = float_bits;
    if low == 0 {
        sign = high;
        if high & 0x7fff_ffff == 0 {
            return bits;
        }
    }
    let difference = crate::x87::x87_fsub(st0, fild64(integer)).store32();
    if (sign as i32) < 0 {
        let flipped = difference ^ 0x8000_0000;
        let carry = flipped.checked_add(0x7fff_ffff).is_none();
        bits.wrapping_add(u64::from(carry))
    } else {
        let borrow = difference.checked_add(0x7fff_ffff).is_none();
        bits.wrapping_sub(u64::from(borrow))
    }
}

pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00ec_dbf6, gcvt_s(Ptr, u32, f64, u32) -> i32),
        entry!(0x00ed_f36b, fltout2(u32, u32, Ptr, Ptr, u32) -> Ptr),
        entry!(0x00ed_f2ae, dtold(Ptr, Ptr) -> Ptr),
        entry!(0x00ee_47cb, i10_output(u32, u32, u32, i32, u32, Ptr) -> i32),
        entry!(0x00ee_18cb, fptostr(Ptr, u32, i32, Ptr) -> i32),
        entry!(0x00ed_22cc, cftof(Ptr, Ptr, u32, i32) -> i32),
        entry!(0x00ed_2211, cftof_l(Ptr, Ptr, u32, i32, u32) -> i32),
        entry!(0x00ed_211a, cftof2_l(u32, u32, u32, i32, u32, u32) -> i32),
        entry!(0x00ed_1d87, cftoe(Ptr, Ptr, u32, i32, u32) -> i32),
        entry!(0x00ed_1cb7, cftoe_l(Ptr, Ptr, u32, i32, u32, u32) -> i32),
        entry!(0x00ed_1b48, cftoe2_l(u32, u32, i32, u32, u32, u32, u32) -> i32),
        entry!(0x00ed_1b03, shift(u32, u32) -> u32),
        entry!(0x00ec_a4c8, atof_l(Ptr, u32) -> f64),
        entry!(0x00ec_62f6, ftol2_x87(f64) -> u64),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    type Calls = Vec<Vec<u32>>;

    /// A big unsigned number in 32-bit limbs, least significant first.
    type Big = Vec<u32>;

    fn times_small(a: &mut Big, m: u32) {
        let mut carry = 0u64;
        for limb in a.iter_mut() {
            let v = u64::from(*limb) * u64::from(m) + carry;
            *limb = v as u32;
            carry = v >> 32;
        }
        if carry != 0 {
            a.push(carry as u32);
        }
    }

    fn bit_length(a: &Big) -> usize {
        let top = a.iter().rposition(|&l| l != 0).unwrap();
        32 * top + (32 - a[top].leading_zeros() as usize)
    }

    fn bit(a: &Big, i: usize) -> bool {
        a.get(i / 32).is_some_and(|l| l >> (i % 32) & 1 != 0)
    }

    /// `a >= b` for numbers of the same limb count.
    fn at_least(a: &Big, b: &Big) -> bool {
        for (x, y) in a.iter().rev().zip(b.iter().rev()) {
            if x != y {
                return x > y;
            }
        }
        true
    }

    fn subtract(a: &mut Big, b: &Big) {
        let mut borrow = 0i64;
        for (x, y) in a.iter_mut().zip(b.iter()) {
            let v = i64::from(*x) - i64::from(*y) - borrow;
            *x = v as u32;
            borrow = i64::from(v < 0);
        }
    }

    fn double(a: &mut Big) {
        let mut carry = 0u32;
        for limb in a.iter_mut() {
            let next = *limb >> 31;
            *limb = *limb << 1 | carry;
            carry = next;
        }
    }

    /// The first 81 bits of 10^k from its leading one and the binary exponent `e` with
    /// 10^k = 0.b0b1.. * 2^e.
    fn power_bits(k: i32) -> ([bool; 81], i32) {
        let mut ten: Big = vec![1];
        for _ in 0..k.unsigned_abs() {
            times_small(&mut ten, 10);
        }
        let mut out = [false; 81];
        if k >= 0 {
            let length = bit_length(&ten);
            for (i, b) in out.iter_mut().enumerate() {
                *b = i < length && bit(&ten, length - 1 - i);
            }
            return (out, length as i32);
        }
        // 1 / 10^-k by long division, one binary place at a time.
        ten.push(0);
        let mut rest: Big = vec![0; ten.len()];
        rest[0] = 1;
        let mut place = 0i32;
        let mut first = None;
        let mut taken = 0;
        while taken < out.len() {
            double(&mut rest);
            place += 1;
            let one = at_least(&rest, &ten);
            if one {
                subtract(&mut rest, &ten);
                first.get_or_insert(place);
            }
            if first.is_some() {
                out[taken] = one;
                taken += 1;
            }
        }
        (out, 1 - first.unwrap())
    }

    /// One table entry: the top 64 bits rounded to nearest (bytes 2-9), the next 16 bits rounded
    /// to nearest on their own (bytes 0-1), the exponent biased by 0x3ffe (bytes 10-11).
    fn entry(k: i32) -> [u8; 12] {
        let (bits, exponent) = power_bits(k);
        let take =
            |range: std::ops::Range<usize>| range.fold(0u64, |v, i| v << 1 | u64::from(bits[i]));
        let high = take(0..64).wrapping_add(u64::from(bits[64]));
        let low = (take(64..80) as u16).wrapping_add(u16::from(bits[80]));
        let mut out = [0u8; 12];
        out[0..2].copy_from_slice(&low.to_le_bytes());
        out[2..10].copy_from_slice(&high.to_le_bytes());
        out[10..12].copy_from_slice(&((exponent + 0x3ffe) as u16).to_le_bytes());
        out
    }

    /// The power-of-ten tables `$I10_OUTPUT` reads at `011c2cb0` and `011c2e10`, computed from
    /// the exact powers (the exe's entries, checked byte for byte against the exe by
    /// [`computed_tables_are_the_exes`]): 10^(j * 8^g) for g 0..4 and j 1..7, then 10^4096 (the
    /// five groups' entries the 80-bit range reaches), positive and negative. The top 64 bits are
    /// rounded to nearest and the next 16 rounded on their own, so where the low word's top bit
    /// is set the top 64 bits were rounded up, which the code's dword decrement takes back.
    fn power_tables() -> (Vec<[u8; 12]>, Vec<[u8; 12]>) {
        let mut powers = Vec::new();
        for g in 0..4 {
            for j in 1..=7 {
                powers.push(j * 8i32.pow(g));
            }
        }
        powers.push(4096);
        (
            powers.iter().map(|&k| entry(k)).collect(),
            powers.iter().map(|&k| entry(-k)).collect(),
        )
    }

    /// The texts the exponent of all ones gives (at their addresses in the exe) and the `%e`
    /// exponent template.
    const TEXTS: [(u32, &[u8]); 5] = [
        (TEXT_QNAN, b"1#QNAN\0"),
        (TEXT_INF, b"1#INF\0"),
        (TEXT_IND, b"1#IND\0"),
        (TEXT_SNAN, b"1#SNAN\0"),
        (EXPONENT_TEMPLATE, b"e+000\0"),
    ];

    fn unhex(s: &str) -> Vec<u8> {
        (0..s.len() / 2)
            .map(|i| u8::from_str_radix(&s[2 * i..2 * i + 2], 16).unwrap())
            .collect()
    }

    fn fnv(h: &mut u64, data: &[u8]) {
        for &b in data {
            *h ^= u64::from(b);
            *h = h.wrapping_mul(0x0000_0100_0000_01b3);
        }
    }

    const FNV_START: u64 = 0xcbf2_9ce4_8422_2325;

    fn calls(e: &Engine, addr: u32) -> Calls {
        let log = e.call_log.as_ref().unwrap();
        log.iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    /// The `_LocaleUpdate` the doubles hand out, on the heap: the locale info (its `lconv` at
    /// +0xbc, whose `decimal_point` is `point`) and a multibyte info word.
    struct Locale {
        info: u32,
        mbc: u32,
    }

    fn locale(e: &mut Engine, point: u8) -> Locale {
        let info = e.mem.alloc(0x100);
        let lconv = e.mem.alloc(0x10);
        let text = e.mem.alloc(2);
        e.mem.set_u32(info + 0xbc, lconv);
        e.mem.set_u32(lconv, text);
        e.mem.set_u8(text, point);
        let mbc = e.mem.alloc(4);
        Locale { info, mbc }
    }

    /// An engine with the computed power-of-ten tables ([`power_tables`]) and the texts at
    /// their addresses, the output-format byte clear, `_errno` doubled to one slot, the
    /// invalid-parameter report and `__invoke_watson` doubled to do nothing, and
    /// `_LocaleUpdate` doubled to fill the object with a locale whose point is `'.'` and leave
    /// its "updated" byte clear (as the oracle runs did). The call log is off.
    fn engine() -> (Engine, u32) {
        let mut e = Engine::new();
        let (positive, negative) = power_tables();
        for (table, entries) in [(POW10_POSITIVE, positive), (POW10_NEGATIVE, negative)] {
            e.map(table, 12 * entries.len() as u32);
            for (i, entry) in entries.iter().enumerate() {
                e.mem.write(table + 12 * i as u32, entry);
            }
        }
        for (at, text) in TEXTS {
            e.map(at, text.len() as u32);
            e.mem.write(at, text);
        }
        e.map(OUTPUT_FORMAT, 4);
        let slot = e.mem.alloc(4);
        e.register_double(ERRNO_ADDR, move |_, _| Ret {
            eax: slot,
            ..Ret::default()
        });
        e.register_double(INVALID_PARAMETER_ADDR, |_, _| Ret::default());
        e.register_double(INVOKE_WATSON_ADDR, |_, _| Ret::default());
        let Locale { info, mbc } = locale(&mut e, b'.');
        e.register_double(LOCALE_UPDATE_ADDR, move |e, a| {
            e.mem.set_u32(a[0], info);
            e.mem.set_u32(a[0] + 4, mbc);
            e.mem.set_u8(a[0] + 0xc, 0);
            Ret {
                eax: a[0],
                ..Ret::default()
            }
        });
        (e, slot)
    }

    /// A heap block of `len` bytes of `fill`.
    fn block(e: &mut Engine, len: u32, fill: u8) -> u32 {
        let at = e.mem.alloc(len);
        e.mem.write(at, &vec![fill; len as usize]);
        at
    }

    /// A xorshift generator for spread test values.
    struct Rng(u64);

    impl Rng {
        fn next(&mut self) -> u64 {
            self.0 ^= self.0 << 13;
            self.0 ^= self.0 >> 7;
            self.0 ^= self.0 << 17;
            self.0
        }
    }

    /// The doubles the sweeps convert (bit patterns): signed zeros, denormals, the extremes,
    /// infinities and NaNs of every class, powers of two and ten with their neighbours, numbers
    /// whose decimal expansion ends in a 5 (halfway at some digit), classic rounding cases, and
    /// random doubles over every exponent band.
    fn special_doubles() -> Vec<u64> {
        let mut out: Vec<u64> = vec![
            0,
            1 << 63,
            1,
            2,
            3,
            0x000f_ffff_ffff_ffff,
            0x8000_0000_0000_0001,
            0x0010_0000_0000_0000,
            0x0010_0000_0000_0001,
            0x7fef_ffff_ffff_ffff,
            0xffef_ffff_ffff_ffff,
            0x7ff0_0000_0000_0000,
            0xfff0_0000_0000_0000,
            0x7ff8_0000_0000_0000,
            0xfff8_0000_0000_0000,
            0xfff8_0000_0000_0001,
            0x7ff8_0000_0000_0001,
            0x7ff0_0000_0000_0001,
            0xfff0_0000_0000_0001,
            0x7ff4_0000_0000_0000,
            0x7fff_ffff_ffff_ffff,
            0xffff_ffff_ffff_ffff,
        ];
        let literals: [f64; 48] = [
            0.1,
            0.2,
            0.3,
            0.5,
            1.0,
            1.5,
            2.5,
            3.5,
            0.125,
            0.375,
            0.15,
            0.25,
            0.35,
            0.45,
            1.005,
            2.675,
            9.5,
            99.5,
            999.5,
            0.95,
            0.995,
            0.9995,
            9.995,
            1.0e23,
            0.7,
            123_456_789.0,
            0.000123456789,
            1.0e-5,
            1.0e-4,
            5.0e-5,
            100.0,
            1.0e15,
            1.0e16,
            1.0e17,
            4.35,
            0.045,
            1.125,
            2.0 / 3.0,
            1.0 / 3.0,
            7.0e-10,
            6.02e23,
            1.6e-19,
            12.5,
            1.25e-7,
            0.0005,
            5.5,
            4_503_599_627_370_497.0,
            0.0625,
        ];
        for v in literals {
            out.push(v.to_bits());
            out.push((-v).to_bits());
        }
        out
    }

    /// The rest of [`doubles`]: powers of two and ten with their neighbours, integers with a 5
    /// at the 17th or 18th significant digit, and random doubles over every exponent band.
    fn spread_doubles() -> Vec<u64> {
        let mut out = Vec::new();
        let mut k = -1074i32;
        while k <= 1023 {
            out.push(if k >= -1022 {
                ((k + 1023) as u64) << 52
            } else {
                1u64 << (k + 1074)
            });
            k += 13;
        }
        for p in -30..=30 {
            let v: f64 = format!("1e{p}").parse().unwrap();
            out.push(v.to_bits());
            out.push(v.to_bits() + 1);
            out.push(v.to_bits() - 1);
        }
        // Integers and halves with a 5 at the 17th or 18th significant digit.
        for v in [
            12_345_678_901_234_565u64,
            12_345_678_901_234_575,
            99_999_999_999_999_995,
            10_000_000_000_000_005,
            1_152_921_504_606_846_976,
        ] {
            out.push((v as f64).to_bits());
        }
        let mut rng = Rng(0x2545_f491_4f6c_dd1d);
        for (low, high, count) in [
            (0x000u64, 0x7feu64, 220),
            (0x3c0, 0x440, 160),
            (0x3fd, 0x404, 80),
            (0x000, 0x001, 20),
        ] {
            for _ in 0..count {
                let exponent = low + rng.next() % (high - low + 1);
                let fraction = rng.next() & 0x000f_ffff_ffff_ffff;
                let sign = rng.next() & (1 << 63);
                out.push(sign | exponent << 52 | fraction);
            }
        }
        out
    }

    /// `$I10_OUTPUT`'s direct inputs: the three words (the third's upper half random, which it
    /// does not read), the digit count and the flags, over zero, every exponent band of the
    /// 80-bit form (denormal, below and above the double range, the largest), unnormal
    /// significands, the NaN and infinity patterns, and digit counts from -3 to 24.
    fn i10_inputs() -> Vec<(u32, u32, u32, i32, u32)> {
        let mut out = Vec::new();
        let mut rng = Rng(0x9e37_79b9_7f4a_7c15);
        let specials: [(u32, u32, u16); 12] = [
            (0, 0, 0),
            (0, 0, 0x8000),
            (0, 0x8000_0000, 0x7fff),
            (0, 0x8000_0000, 0xffff),
            (0, 0xc000_0000, 0xffff),
            (1, 0xc000_0000, 0xffff),
            (0, 0xc000_0000, 0x7fff),
            (0, 0xa000_0000, 0x7fff),
            (5, 0x8000_0000, 0x7fff),
            (0, 0, 0x7fff),
            (0, 0x4000_0000, 0x7fff),
            (0x1234, 0x0000_0001, 0xffff),
        ];
        for (i, (w0, w1, w2)) in specials.iter().enumerate() {
            out.push((*w0, *w1, u32::from(*w2), 0x11, (i & 1) as u32));
        }
        let bands: [(u32, u32, u32); 7] = [
            (0x0000, 0x0000, 30),
            (0x0001, 0x3bcc, 60),
            (0x3bcd, 0x43fe, 160),
            (0x3ff0, 0x4010, 60),
            (0x43ff, 0x7ffe, 60),
            (0x7ff0, 0x7ffe, 30),
            (0x3ffe, 0x3fff, 40),
        ];
        for (low, high, count) in bands {
            for n in 0..count {
                let exponent = low + (rng.next() % u64::from(high - low + 1)) as u32;
                let sign = (rng.next() as u32) & 0x8000;
                let w0 = rng.next() as u32;
                let mut w1 = rng.next() as u32;
                // Every fifth an unnormal (integer bit clear) unless the exponent is zero.
                if n % 5 != 0 && exponent != 0 {
                    w1 |= 0x8000_0000;
                } else if exponent != 0 {
                    w1 &= 0x7fff_ffff;
                }
                let upper = (rng.next() as u32) & 0xffff_0000;
                let digits = (rng.next() % 28) as i32 - 3;
                let flags = (rng.next() & 1) as u32;
                out.push((w0, w1, upper | sign | exponent, digits, flags));
            }
        }
        out
    }

    /// Every double the `__fltout2` sweep converts.
    fn doubles() -> Vec<u64> {
        let mut out = special_doubles();
        out.extend(spread_doubles());
        out
    }

    /// The doubles the layout sweeps (`__gcvt_s`, `__cftoe_l`, `__cftof_l`) convert at every
    /// precision: the special ones and every seventh of the rest.
    fn sweep_doubles() -> Vec<u64> {
        let mut out = special_doubles();
        out.extend(spread_doubles().into_iter().step_by(7));
        out
    }

    /// One direct call of `__cftoe2_l` or `__cftof2_l`: the sign (`'-'` or `' '`), the decimal
    /// point position, the decimals, the `%g` flag, capitals and whether the digits are `"0"`.
    type Layout = (u32, i32, i32, u32, u32, bool);

    /// The direct layout calls: both signs, point positions around zero, decimals from -2 to 6,
    /// with and without the `%g` flag and capitals, and a zero mantissa.
    fn layout_inputs() -> Vec<Layout> {
        let mut out = Vec::new();
        for sign in [0x20, 0x2d] {
            for decpt in [-3, -1, 0, 1, 2, 5] {
                for decimals in [-2, -1, 0, 1, 2, 3, 6] {
                    for g_flag in [0, 1] {
                        out.push((
                            sign,
                            decpt,
                            decimals,
                            g_flag,
                            (decpt & 1) as u32,
                            decpt == 2,
                        ));
                    }
                }
            }
        }
        out
    }

    /// The buffer a layout call starts from, as `__cftoe_l` / `__cftof_l` leave it for one
    /// (`exponent` true for `%e`): a place for the sign, for `%e` one for the point, the digits
    /// (`decimals + 1` for `%e`, `decpt + decimals` for `%f`, of `"1234567890123456789"`), the
    /// terminator, then 0x55.
    fn layout_buffer(input: &Layout, exponent: bool) -> Vec<u8> {
        let (sign, decpt, decimals, _, _, zero) = *input;
        let mut out = Vec::new();
        if sign == 0x2d {
            out.push(b'S');
        }
        if exponent && decimals > 0 {
            out.push(b'P');
        }
        let count = if exponent {
            decimals + 1
        } else {
            decpt + decimals
        };
        let digits: &[u8] = if zero {
            b"0000000000000000000"
        } else {
            b"1234567890123456789"
        };
        out.extend_from_slice(&digits[..count.max(0) as usize]);
        out.push(0);
        out.resize(0x40, 0x55);
        out
    }

    /// The hashes of the exe's own results over the sweeps below (FNV-1a 64 of each vector's
    /// record in order), measured with `nv-call` on FalloutNV.exe 1.4.0.525 (SHA-256
    /// 19406942..739f) under control word 0x027f, with `_LocaleUpdate` patched in the harness
    /// to hand out a locale whose decimal point is `'.'` and the output-format byte clear.
    const FLTOUT2_HASH: u64 = 0xd0db_05f6_213c_de6d;
    const I10_OUTPUT_HASH: u64 = 0xf1b9_7288_94af_3be9;
    const GCVT_S_HASH: u64 = 0x8fb3_40a8_26d9_70ac;
    const CFTOE_L_HASH: u64 = 0x92b9_5962_b335_76af;
    const CFTOF_L_HASH: u64 = 0x80c1_8026_799e_0db3;
    const CFTOE2_L_HASH: u64 = 0xf24d_8904_8175_eb13;
    const CFTOF2_L_HASH: u64 = 0xfd48_579c_8d8b_3f4d;

    const GCVT_S: u32 = 0x00ec_dbf6;

    /// With `NV_EXE` set to FalloutNV.exe 1.4.0.525: the computed tables and the texts are the
    /// bytes the exe has at their addresses.
    #[test]
    fn computed_tables_are_the_exes() {
        let Ok(path) = std::env::var("NV_EXE") else {
            return;
        };
        let e = Engine::with_exe(std::path::Path::new(&path)).unwrap();
        let (positive, negative) = power_tables();
        for (table, entries) in [(POW10_POSITIVE, positive), (POW10_NEGATIVE, negative)] {
            for (i, entry) in entries.iter().enumerate() {
                let at = table + 12 * i as u32;
                assert_eq!(e.mem.bytes(at, 12), entry.to_vec(), "{at:08x}");
            }
        }
        for (at, text) in TEXTS {
            assert_eq!(
                e.mem.bytes(at, text.len() as u32),
                text.to_vec(),
                "{at:08x}"
            );
        }
    }

    #[test]
    fn fltout2_gives_the_exes_digits_for_every_double() {
        let (mut e, _) = engine();
        let mut h = FNV_START;
        let all = doubles();
        assert_eq!(all.len(), 948);
        for bits in all {
            let flt = block(&mut e, 16, 0x55);
            let digits = block(&mut e, 24, 0x55);
            let words = [bits as u32, (bits >> 32) as u32, flt, digits, 22];
            assert_eq!(e.call(FLTOUT2, &words).u32(), flt);
            assert_eq!(e.mem.u32(flt + FLT_MANTISSA), digits);
            fnv(&mut h, &e.mem.bytes(flt, 12));
            fnv(&mut h, &e.mem.bytes(digits, 24));
            e.mem.free(flt);
            e.mem.free(digits);
        }
        assert_eq!(h, FLTOUT2_HASH);
    }

    #[test]
    fn i10_output_gives_the_exes_digits_over_every_band_and_count() {
        let (mut e, _) = engine();
        let mut h = FNV_START;
        let all = i10_inputs();
        assert_eq!(all.len(), 452);
        for (w0, w1, w2, digits, flags) in all {
            let fos = block(&mut e, 28, 0xa5);
            let words = [w0, w1, w2, digits as u32, flags, fos];
            let answer = e.call(I10_OUTPUT, &words).u32();
            fnv(&mut h, &answer.to_le_bytes());
            fnv(&mut h, &e.mem.bytes(fos, 28));
            e.mem.free(fos);
        }
        assert_eq!(h, I10_OUTPUT_HASH);
    }

    #[test]
    fn gcvt_s_gives_the_exes_text_at_every_precision() {
        let (mut e, _) = engine();
        let mut h = FNV_START;
        let all = sweep_doubles();
        assert_eq!(all.len() * 18, 4266);
        for bits in all {
            for digits in 0..=17u32 {
                let out = block(&mut e, 64, 0x55);
                let words = args![out, 64u32, f64::from_bits(bits), digits];
                let answer = e.call(GCVT_S, &words).u32();
                fnv(&mut h, &answer.to_le_bytes());
                fnv(&mut h, &e.mem.bytes(out, 64));
                e.mem.free(out);
            }
        }
        assert_eq!(h, GCVT_S_HASH);
    }

    /// The `%e` and `%f` sweeps: every double of [`sweep_doubles`] with -1 to 17 decimals
    /// (capitals for odd counts in `%e`), into 512 bytes of 0x55, locale 0.
    fn layout_sweep(exponent: bool) -> u64 {
        let (mut e, _) = engine();
        let mut h = FNV_START;
        for bits in sweep_doubles() {
            for decimals in -1..=17i32 {
                let value = e.mem.alloc(8);
                e.mem.set_u64(value, bits);
                let out = block(&mut e, 512, 0x55);
                let answer = if exponent {
                    let words = [value, out, 512, decimals as u32, (decimals & 1) as u32, 0];
                    e.call(CFTOE_L, &words).u32()
                } else {
                    e.call(CFTOF_L, &[value, out, 512, decimals as u32, 0])
                        .u32()
                };
                fnv(&mut h, &answer.to_le_bytes());
                fnv(&mut h, &e.mem.bytes(out, 512));
                e.mem.free(out);
                e.mem.free(value);
            }
        }
        h
    }

    #[test]
    fn cftoe_l_gives_the_exes_text_at_every_precision() {
        assert_eq!(layout_sweep(true), CFTOE_L_HASH);
    }

    #[test]
    fn cftof_l_gives_the_exes_text_at_every_precision() {
        assert_eq!(layout_sweep(false), CFTOF_L_HASH);
    }

    /// The direct layout calls of [`layout_inputs`] on one block laid out as in the oracle
    /// runs: the digits (32 bytes), the `_strflt` (16), 16 bytes of 0x55 that negative
    /// decimals write into, the buffer (64).
    fn direct_layout_sweep(exponent: bool) -> u64 {
        let (mut e, _) = engine();
        let mut h = FNV_START;
        for input in layout_inputs() {
            let (sign, decpt, decimals, g_flag, capitals, zero) = input;
            let base = block(&mut e, 0x80, 0);
            let digits: &[u8] = if zero {
                b"0000000000000000000"
            } else {
                b"1234567890123456789"
            };
            e.mem.write(base, digits);
            let flt = base + 0x20;
            e.mem.set_u32(flt + FLT_SIGN, sign);
            e.mem.set_u32(flt + FLT_DECPT, decpt as u32);
            e.mem.set_u32(flt + FLT_FLAG, 1);
            e.mem.set_u32(flt + FLT_MANTISSA, base);
            e.mem.write(base + 0x30, &[0x55; 16]);
            let out = base + 0x40;
            e.mem.write(out, &layout_buffer(&input, exponent));
            let answer = if exponent {
                let words = [out, 64, decimals as u32, capitals, flt, g_flag, 0];
                e.call(CFTOE2_L, &words).u32()
            } else {
                let words = [flt, out, 64, decimals as u32, g_flag, 0];
                e.call(CFTOF2_L, &words).u32()
            };
            fnv(&mut h, &answer.to_le_bytes());
            fnv(&mut h, &e.mem.bytes(base + 0x30, 16));
            fnv(&mut h, &e.mem.bytes(out, 64));
            e.mem.free(base);
        }
        h
    }

    #[test]
    fn cftoe2_l_lays_out_as_the_exe_with_and_without_the_g_flag() {
        assert_eq!(direct_layout_sweep(true), CFTOE2_L_HASH);
    }

    #[test]
    fn cftof2_l_lays_out_as_the_exe_with_and_without_the_g_flag() {
        assert_eq!(direct_layout_sweep(false), CFTOF2_L_HASH);
    }

    /// `__gcvt_s` of `value` with `digits` into 64 bytes: the answer and the text.
    fn gcvt_text(e: &mut Engine, value: f64, digits: u32) -> (u32, String) {
        let out = block(e, 64, 0x55);
        let answer = e.call(GCVT_S, &args![out, 64u32, value, digits]).u32();
        let text = String::from_utf8(e.mem.cstr(out)).unwrap();
        e.mem.free(out);
        (answer, text)
    }

    #[test]
    fn gcvt_s_examples_from_the_exe() {
        let (mut e, _) = engine();
        let cases: [(u64, u32, &str); 24] = [
            (0, 0, "0"),
            (0, 1, "0."),
            (0, 17, "0."),
            (0x7ff0_0000_0000_0000, 0, "e+000"),
            (0x7ff0_0000_0000_0000, 1, "1"),
            (0x7ff0_0000_0000_0000, 6, "1.#INF"),
            (0xfff8_0000_0000_0000, 1, "-1"),
            (0xfff8_0000_0000_0000, 17, "-1.#IND"),
            (0x7ff0_0000_0000_0001, 6, "1.#SNAN"),
            (0.1f64.to_bits(), 0, "0"),
            (0.1f64.to_bits(), 1, "0.1"),
            (0.1f64.to_bits(), 17, "0.10000000000000001"),
            ((-0.1f64).to_bits(), 17, "-0.10000000000000001"),
            (0.5f64.to_bits(), 0, "1"),
            (0.5f64.to_bits(), 17, "0.5"),
            (2.675f64.to_bits(), 1, "3"),
            (2.675f64.to_bits(), 17, "2.6749999999999998"),
            (999.5f64.to_bits(), 0, "e+003"),
            (999.5f64.to_bits(), 1, "1e+003"),
            (999.5f64.to_bits(), 6, "999.5"),
            (1.0e23f64.to_bits(), 6, "1.e+023"),
            (1.0e23f64.to_bits(), 17, "9.9999999999999992e+022"),
            (0.7f64.to_bits(), 17, "0.69999999999999996"),
            (1.125f64.to_bits(), 6, "1.125"),
        ];
        for (bits, digits, text) in cases {
            assert_eq!(
                gcvt_text(&mut e, f64::from_bits(bits), digits),
                (0, text.to_string()),
                "{bits:016x} {digits}"
            );
        }
    }

    #[test]
    fn fltout2_examples_from_the_exe() {
        let (mut e, _) = engine();
        // (double, sign, decpt, flag, digits)
        type Case = (u64, u8, i32, u32, &'static [u8]);
        let cases: [Case; 8] = [
            (0, b' ', 0, 1, b"0"),
            (1 << 63, b'-', 0, 1, b"0"),
            (1, b' ', -323, 1, b"49406564584124654"),
            (0x7ff0_0000_0000_0000, b' ', 1, 0, b"1#INF"),
            (0x7ff8_0000_0000_0000, b' ', 1, 0, b"1#QNAN"),
            (0xfff8_0000_0000_0000, b'-', 1, 0, b"1#IND"),
            (0x7ff0_0000_0000_0001, b' ', 1, 0, b"1#SNAN"),
            (0.1f64.to_bits(), b' ', 0, 1, b"10000000000000001"),
        ];
        for (bits, sign, decpt, flag, digits) in cases {
            let flt = block(&mut e, 16, 0x55);
            let out = block(&mut e, 24, 0x55);
            e.call(FLTOUT2, &[bits as u32, (bits >> 32) as u32, flt, out, 22]);
            assert_eq!(e.mem.u32(flt + FLT_SIGN), u32::from(sign), "{bits:016x}");
            assert_eq!(e.mem.u32(flt + FLT_DECPT), decpt as u32, "{bits:016x}");
            assert_eq!(e.mem.u32(flt + FLT_FLAG), flag, "{bits:016x}");
            assert_eq!(e.mem.cstr(out), digits.to_vec(), "{bits:016x}");
        }
    }

    #[test]
    fn i10_output_specials_from_the_exe() {
        let (mut e, _) = engine();
        // (words, digits, flags, answer, the struct's first bytes)
        type Case = ([u32; 3], i32, u32, u32, &'static str);
        let cases: [Case; 6] = [
            ([0, 0, 0], 0x11, 0, 1, "000020013000a5"),
            ([0, 0, 0x8000], 0x11, 1, 1, "00002d013000a5"),
            (
                [0, 0x8000_0000, 0x7fff],
                0x11,
                0,
                0,
                "010020053123494e4600a5",
            ),
            (
                [0, 0xc000_0000, 0xffff],
                0x11,
                0,
                0,
                "01002d053123494e4400a5",
            ),
            (
                [1, 0xc000_0000, 0xffff],
                0x11,
                1,
                0,
                "01002d063123514e414e00a5",
            ),
            (
                [5, 0x8000_0000, 0x7fff],
                0x11,
                1,
                0,
                "010020063123534e414e00a5",
            ),
        ];
        for (words, digits, flags, answer, start) in cases {
            let fos = block(&mut e, 28, 0xa5);
            let all = [words[0], words[1], words[2], digits as u32, flags, fos];
            assert_eq!(e.call(I10_OUTPUT, &all).u32(), answer, "{words:x?}");
            let expected = unhex(start);
            assert_eq!(
                e.mem.bytes(fos, expected.len() as u32),
                expected,
                "{words:x?}"
            );
        }
    }

    #[test]
    fn i10_output_with_no_digits_to_make() {
        let (mut e, _) = engine();
        let one =
            |digits: i32, flags: u32, fos: u32| [0, 0x8000_0000, 0x3fff, digits as u32, flags, fos];
        // Flags bit 0: the count plus the exponent (-5 + 1) is not positive: "0".
        let fos = block(&mut e, 28, 0xa5);
        assert_eq!(e.call(I10_OUTPUT, &one(-5, 1, fos)).u32(), 1);
        assert_eq!(e.mem.bytes(fos, 7), unhex("000020013000a5"));
        // Without the flag a count of -1 makes no digit: the rounding reads the struct's
        // count byte (+3, 0xa5 here, negative as a signed byte): all zeros, "0".
        let fos = block(&mut e, 28, 0xa5);
        assert_eq!(e.call(I10_OUTPUT, &one(-1, 0, fos)).u32(), 1);
        assert_eq!(e.mem.bytes(fos, 7), unhex("000020013000a5"));
        // A count byte of '5' or more there rounds "up" through it: exponent 2, count 0.
        let fos = block(&mut e, 28, 0xa5);
        e.mem.set_u8(fos + 3, b'7');
        assert_eq!(e.call(I10_OUTPUT, &one(-1, 0, fos)).u32(), 1);
        assert_eq!(e.mem.bytes(fos, 6), unhex("0200200000a5"));
    }

    #[test]
    fn dtold_is_the_x87_load_but_keeps_signalling_nans() {
        let (mut e, _) = engine();
        let input = e.mem.alloc(8);
        let out = e.mem.alloc(12);
        for bits in doubles() {
            e.mem.set_u64(input, bits);
            e.mem.write(out, &[0x55; 12]);
            assert_eq!(e.call(DTOLD, &[out, input]).u32(), out);
            let fraction = bits & 0x000f_ffff_ffff_ffff;
            let signalling =
                (bits >> 52) & 0x7ff == 0x7ff && bits & (1 << 51) == 0 && fraction != 0;
            let expected = if signalling {
                // FLD would set the quiet bit; ___dtold copies the fraction as it is.
                let sign = ((bits >> 48) & 0x8000) as u16;
                X87Ext::from_parts(sign | 0x7fff, 1 << 63 | fraction << 11)
            } else {
                X87Ext::load64(f64::from_bits(bits))
            };
            assert_eq!(e.mem.bytes(out, 10), expected.0.to_vec(), "{bits:016x}");
            assert_eq!(e.mem.bytes(out + 10, 2), vec![0x55, 0x55]);
        }
    }

    #[test]
    fn gcvt_s_reports_a_bad_buffer_size_or_count() {
        let (mut e, slot) = engine();
        e.call_log = Some(vec![]);
        let value = 1.5f64;
        // A null buffer: EINVAL, the report, nothing written.
        assert_eq!(e.call(GCVT_S, &args![0u32, 8u32, value, 3u32]).u32(), 0x16);
        assert_eq!(e.mem.u32(slot), 0x16);
        // A size of 0: the same, the buffer untouched.
        let out = block(&mut e, 8, 0x55);
        assert_eq!(e.call(GCVT_S, &args![out, 0u32, value, 3u32]).u32(), 0x16);
        assert_eq!(e.mem.u8(out), 0x55);
        // A count not below the size: ERANGE, the first byte cleared.
        e.mem.set_u32(slot, 0);
        assert_eq!(e.call(GCVT_S, &args![out, 8u32, value, 8u32]).u32(), 0x22);
        assert_eq!(e.mem.u32(slot), 0x22);
        assert_eq!(e.mem.bytes(out, 2), vec![0, 0x55]);
        assert_eq!(calls(&e, INVALID_PARAMETER_ADDR), vec![vec![0; 5]; 3]);
        assert!(calls(&e, FLTOUT2).is_empty());
        // Every path took the locale.
        assert_eq!(calls(&e, LOCALE_UPDATE_ADDR).len(), 3);
    }

    #[test]
    fn gcvt_s_passes_on_a_conversion_error_and_strips_zeros_after_the_locales_point() {
        let (mut e, slot) = engine();
        let Locale { info, mbc } = locale(&mut e, b',');
        let thread = e.mem.alloc(0x80);
        // This `_LocaleUpdate` says it updated the thread (the release clears bit 1).
        e.register_double(LOCALE_UPDATE_ADDR, move |e, a| {
            e.mem.set_u32(a[0], info);
            e.mem.set_u32(a[0] + 4, mbc);
            e.mem.set_u32(a[0] + 8, thread);
            e.mem.set_u8(a[0] + 0xc, 1);
            Ret {
                eax: a[0],
                ..Ret::default()
            }
        });
        let texts: [(&[u8], &str); 5] = [
            (b"12,500", "12,5"),
            (b"100,000", "100,"),
            (b"1,500e+005", "1,5e+005"),
            (b"12.500", "12.500"),
            (b"", ""),
        ];
        for (made, kept) in texts {
            let made = made.to_vec();
            e.register_double(CFTOF, move |e, a| {
                e.mem.write(a[1], &made);
                e.mem.set_u8(a[1] + made.len() as u32, 0);
                Ret::default()
            });
            e.mem.set_u32(thread + 0x70, 0xff);
            assert_eq!(gcvt_text(&mut e, 12.5, 6), (0, kept.to_string()));
            assert_eq!(e.mem.u32(thread + 0x70), 0xfd);
        }
        // An error from the conversion is stored in errno and answered.
        e.register_double(CFTOF, |_, _| Ret {
            eax: 0x22,
            ..Ret::default()
        });
        e.mem.set_u32(slot, 0);
        assert_eq!(gcvt_text(&mut e, 12.5, 6).0, 0x22);
        assert_eq!(e.mem.u32(slot), 0x22);
        // The exponent decides: 1e20 with 6 digits goes to __cftoe with 5 decimals and the
        // address of the argument.
        let seen = e.mem.alloc(8);
        e.register_double(CFTOE, move |e, a| {
            let value = e.mem.u64(a[0]);
            e.mem.set_u64(seen, value);
            e.mem.set_u8(a[1], 0);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        gcvt_text(&mut e, 1.0e20, 6);
        let made = calls(&e, CFTOE);
        assert_eq!(made.len(), 1);
        assert_eq!(made[0][2..], [64, 5, 0]);
        assert_eq!(e.mem.u64(seen), 1.0e20f64.to_bits());
    }

    #[test]
    fn fptostr_rounds_carries_and_reports() {
        let (mut e, slot) = engine();
        let flt = block(&mut e, 16, 0);
        let digits = e.mem.alloc(16);
        e.mem.set_u32(flt + FLT_MANTISSA, digits);
        let out = block(&mut e, 16, 0x55);
        // (mantissa, count, text, decimal point moved)
        let cases: [(&[u8], i32, &str, u32); 5] = [
            (b"9996", 3, "1000", 1),
            (b"12", 4, "1200", 0),
            (b"1249", 3, "125", 0),
            (b"5", 0, "1", 1),
            (b"7", -1, "", 0),
        ];
        for (mantissa, count, text, carried) in cases {
            e.mem.write(digits, &[0; 16]);
            e.mem.write(digits, mantissa);
            e.mem.set_u32(flt + FLT_DECPT, 4);
            assert_eq!(e.call(FPTOSTR, &[out, 16, count as u32, flt]).u32(), 0);
            assert_eq!(
                e.mem.cstr(out),
                text.as_bytes().to_vec(),
                "{mantissa:?} {count}"
            );
            assert_eq!(
                e.mem.u32(flt + FLT_DECPT),
                4 + carried,
                "{mantissa:?} {count}"
            );
        }
        e.call_log = Some(vec![]);
        assert_eq!(e.call(FPTOSTR, &[0, 16, 3, flt]).u32(), 0x16);
        assert_eq!(e.call(FPTOSTR, &[out, 0, 3, flt]).u32(), 0x16);
        assert_eq!(e.mem.u32(slot), 0x16);
        e.mem.set_u8(out, 0x55);
        // A size not above count + 1: ERANGE, the first byte cleared.
        assert_eq!(e.call(FPTOSTR, &[out, 4, 3, flt]).u32(), 0x22);
        assert_eq!(e.mem.u32(slot), 0x22);
        assert_eq!(e.mem.u8(out), 0);
        assert_eq!(calls(&e, INVALID_PARAMETER_ADDR).len(), 3);
    }

    #[test]
    fn cftoe_and_cftof_report_and_pass_on_errors() {
        let (mut e, slot) = engine();
        let value = e.mem.alloc(8);
        e.mem.set_u64(value, 150_000.0f64.to_bits());
        let out = block(&mut e, 64, 0x55);
        e.call_log = Some(vec![]);
        // Null buffers and size 0: EINVAL after the digits were made.
        assert_eq!(e.call(CFTOE, &[value, 0, 64, 2, 0]).u32(), 0x16);
        assert_eq!(e.call(CFTOF, &[value, 0, 64, 2]).u32(), 0x16);
        assert_eq!(e.call(CFTOE, &[value, out, 0, 2, 0]).u32(), 0x16);
        assert_eq!(calls(&e, FLTOUT2).len(), 3);
        assert_eq!(calls(&e, CFTOE_L)[0], vec![value, 0, 64, 2, 0, 0]);
        assert_eq!(calls(&e, CFTOF_L)[0], vec![value, 0, 64, 2, 0]);
        // __fptostr's ERANGE: the buffer's first byte cleared, the code answered.
        assert_eq!(e.call(CFTOF, &[value, out, 7, 2]).u32(), 0x22);
        assert_eq!(e.mem.u8(out), 0);
        // __cftoe2_l's ERANGE: the size must be above decimals + 9.
        assert_eq!(e.call(CFTOE, &[value, out, 11, 2, 0]).u32(), 0x22);
        assert_eq!(e.mem.u32(slot), 0x22);
        // With the output-format bit the exponent loses its leading zero.
        assert_eq!(e.call(CFTOE, &[value, out, 64, 2, 1]).u32(), 0);
        assert_eq!(e.mem.cstr(out), b"1.50E+005".to_vec());
        e.mem.set_u8(OUTPUT_FORMAT, 1);
        assert_eq!(e.call(CFTOE, &[value, out, 64, 2, 1]).u32(), 0);
        assert_eq!(e.mem.cstr(out), b"1.50E+05".to_vec());
        assert_eq!(e.call(CFTOF, &[value, out, 64, 2]).u32(), 0);
        assert_eq!(e.mem.cstr(out), b"150000.00".to_vec());
    }

    #[test]
    fn cftof2_l_and_cftoe2_l_report_a_null_buffer() {
        let (mut e, slot) = engine();
        let flt = block(&mut e, 16, 0);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(CFTOF2_L, &[flt, 0, 64, 2, 0, 0]).u32(), 0x16);
        assert_eq!(e.call(CFTOE2_L, &[0, 64, 2, 0, flt, 0, 0]).u32(), 0x16);
        assert_eq!(e.mem.u32(slot), 0x16);
        assert_eq!(calls(&e, LOCALE_UPDATE_ADDR).len(), 2);
        assert_eq!(calls(&e, INVALID_PARAMETER_ADDR).len(), 2);
    }

    #[test]
    fn shift_moves_the_string_up_or_leaves_it() {
        let (mut e, _) = engine();
        let text = block(&mut e, 16, 0x55);
        e.mem.write(text, b"abc\0");
        e.call_log = Some(vec![]);
        assert_eq!(e.call(SHIFT, &[text, 0]).u32(), text);
        assert!(calls(&e, MEMMOVE_ADDR).is_empty());
        assert_eq!(e.call(SHIFT, &[text, 2]).u32(), text + 2);
        assert_eq!(calls(&e, MEMMOVE_ADDR), vec![vec![text + 2, text, 4]]);
        assert_eq!(e.mem.bytes(text, 6), b"ababc\0".to_vec());
    }

    #[test]
    fn atof_l_skips_white_space_and_converts_through_fltin2() {
        let (mut e, slot) = engine();
        // The `_LocaleUpdate` double: the locale info (mb_cur_max 1, a ctype table with
        // `_SPACE` on the blank and the tab), a thread block, and the "updated" byte set.
        let info = block(&mut e, 0x100, 0);
        let table = block(&mut e, 0x200, 0);
        e.mem.set_u16(table + 2 * 0x20, 8);
        e.mem.set_u16(table + 2 * 9, 8);
        e.mem.set_u32(info + 0xc8, table);
        e.mem.set_u32(info + 0xac, 1);
        let thread = block(&mut e, 0x80, 0);
        e.mem.set_u32(thread + 0x70, 0xff);
        e.register_double(LOCALE_UPDATE_ADDR, move |e, a| {
            e.mem.set_u32(a[0], info);
            e.mem.set_u32(a[0] + 8, thread);
            e.mem.set_u8(a[0] + 0xc, 1);
            a[0].into_ret()
        });
        e.register(FLTIN2, |e, a| {
            assert_eq!(e.mem.cstr(a[1]).len() as u32, a[2]);
            e.mem.set_u64(a[0] + 0x10, 2.5f64.to_bits());
            a[0].into_ret()
        });
        e.call_log = Some(vec![]);
        let s = block(&mut e, 16, 0);
        e.mem.write(s, b" \t2.5x\0");
        let r = e.call(0x00ec_a4c8, &args![s, 0x1234u32]);
        assert_eq!(r.st0, 2.5);
        let fltin = calls(&e, FLTIN2);
        assert_eq!(fltin[0][1..5].to_vec(), vec![s + 2, 4, 0, 0]);
        assert_eq!(calls(&e, LOCALE_UPDATE_ADDR)[0][1], 0x1234);
        assert_eq!(e.mem.u32(thread + 0x70), 0xfd);
        // A null string: EINVAL with the handler, 0.0, the bit cleared.
        e.mem.set_u32(thread + 0x70, 0xff);
        assert_eq!(e.call(0x00ec_a4c8, &args![0u32, 0u32]).st0, 0.0);
        assert_eq!(e.mem.u32(slot), 0x16);
        assert_eq!(calls(&e, INVALID_PARAMETER_ADDR).len(), 1);
        assert_eq!(e.mem.u32(thread + 0x70), 0xfd);
        // A multibyte locale asks `_isctype_l` (character, `_SPACE`, the update).
        e.mem.set_u32(info + 0xac, 2);
        e.register(ISCTYPE_L_ADDR, |_, a| u32::from(a[0] == 0x23).into_ret());
        e.mem.write(s, b"##7\0");
        e.call(0x00ec_a4c8, &args![s, 0u32]);
        let asked = calls(&e, ISCTYPE_L_ADDR);
        assert_eq!(asked.len(), 3);
        assert_eq!(asked[2][..2].to_vec(), vec![0x37, 8]);
        assert_eq!(calls(&e, FLTIN2)[1][1], s + 2);
    }

    #[test]
    fn ftol2_x87_truncates_toward_zero() {
        let mut e = Engine::new();
        let mut f = |v: f64| e.call(0x00ec_62f6, &args![v]).u64() as i64;
        assert_eq!(f(2.5), 2);
        assert_eq!(f(2.7), 2);
        assert_eq!(f(-2.5), -2);
        assert_eq!(f(-2.7), -2);
        assert_eq!(f(3.0), 3);
        assert_eq!(f(-3.0), -3);
        assert_eq!(f(0.4), 0);
        assert_eq!(f(0.6), 0);
        assert_eq!(f(-0.6), 0);
        assert_eq!(f(4_294_967_296.5), 4_294_967_296);
        assert_eq!(f(-4_294_967_296.5), -4_294_967_296);
        assert_eq!(f(4_294_967_295.7), 4_294_967_295);
        assert_eq!(f(1e300), i64::MIN);
        assert_eq!(f(f64::NAN), i64::MIN);
    }
}

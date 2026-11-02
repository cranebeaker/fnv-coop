//! `fallout shared/run_00647960` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `00403e20` (`thiscall` on a game setting, no stack words): the address of
/// the setting's float value.
const SETTING_VALUE: u32 = 0x0040_3e20;

/// The float a game setting holds (`FLD dword` of the address `00403e20`
/// answers): its bit pattern.
fn setting_bits(e: &mut Engine, setting: u32) -> u32 {
    let at = e.call(SETTING_VALUE, &args![setting]).u32();
    e.mem.u32(at)
}

// Translated from 00647b70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, two words): the larger of two signed
/// integers (the second when they are equal).
pub fn fn_00647b70(_e: &mut Engine, first: i32, second: i32) -> i32 {
    if first > second {
        first
    } else {
        second
    }
}

// Translated from 00647ba0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, three words: the address of a signed
/// integer, a low and a high limit): clamps the integer, the high limit
/// tested first.
pub fn fn_00647ba0(e: &mut Engine, value: Ptr, low: i32, high: i32) {
    let at = value.addr();
    if (e.mem.u32(at) as i32) > high {
        e.mem.set_u32(at, high as u32);
    } else if (e.mem.u32(at) as i32) < low {
        e.mem.set_u32(at, low as u32);
    }
}

/// `006a78f0` (`thiscall`, no stack words; a float in `ST0`).
const VALUE_OF_006A78F0: u32 = 0x006a_78f0;
/// The two settings `006479e0` adds (`011cfb5c`, `011cf264`).
const SETTING_011CFB5C: u32 = 0x011c_fb5c;
const SETTING_011CF264: u32 = 0x011c_f264;

// Translated from 006479e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, one word: an object or 0): the
/// object's `006a78f0` float (0.0 without an object) times the sum of the
/// settings `011cf264` and `011cfb5c`, in x87 order with `float` stores.
pub fn fn_006479e0(e: &mut Engine, object: u32) -> f32 {
    let value = if object != 0 {
        x87_store32(e.call(VALUE_OF_006A78F0, &args![object]).f64())
    } else {
        x87_store32(0.0)
    };
    let local = x87_quiet_bits(value);
    let first = setting_bits(e, SETTING_011CFB5C);
    let second = setting_bits(e, SETTING_011CF264);
    let sum = x87_add(x87_load32(second), x87_m32(first));
    let result = x87_store32(x87_mul(sum, x87_m32(local)));
    f32::from_bits(result)
}

/// The doubles `00647c00` reads: 10.0 (`01020758`), 0.0 (`01012060`) and
/// 1.0 (`01012070`); the float 10.0 (`01017b78`).
const DOUBLE_TEN: u32 = 0x0102_0758;
const DOUBLE_ZERO: u32 = 0x0101_2060;
const DOUBLE_ONE: u32 = 0x0101_2070;
const FLOAT_TEN: u32 = 0x0101_7b78;
/// The settings `00647c00` reads (`011d003c` twice, `011d0138`, `011d0c00`).
const SETTING_011D003C: u32 = 0x011d_003c;
const SETTING_011D0138: u32 = 0x011d_0138;
const SETTING_011D0C00: u32 = 0x011d_0c00;
/// `004dd130` (`cdecl`, two floats; a float in `ST0`), `00408860` (`cdecl`,
/// one float; `ST0`) and `004bd510` (`cdecl`, two floats; `ST0`).
const CURVE_004DD130: u32 = 0x004d_d130;
const MAGNITUDE_00408860: u32 = 0x0040_8860;
const LIMIT_004BD510: u32 = 0x004b_d510;

/// The double at `address` as `FLD qword` sees it.
fn double_at(e: &Engine, address: u32) -> f64 {
    x87_load64(e.mem.u64(address))
}

// Translated from 00647c00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, two floats; the answer in `ST0`). The
/// scale is `second / 10.0`, 10.0 (`01017b78`) when below 0.0. A first value
/// above 0.0 becomes `first * s1 + curve(scale, s3) * (s2 * ((1 - s1) * first))`
/// with the settings `s1` (`011d003c`), `s2` (`011d0138`) and `s3`
/// (`011d0c00`) and the curve `004dd130`, each product stored as a double
/// and the sum as a float. When `00408860` of the value is not below 1.0 (and
/// not unordered) the answer is `004bd510(value, 1.0)`, else the value.
pub fn fn_00647c00(e: &mut Engine, first: f32, second: f32) -> f32 {
    let mut value = x87_quiet_bits(first.to_bits());
    let mut scale = x87_store32(x87_div(x87_m32(second.to_bits()), double_at(e, DOUBLE_TEN)));
    if x87_m32(scale) < double_at(e, DOUBLE_ZERO) {
        scale = x87_quiet_bits(e.mem.u32(FLOAT_TEN));
    }
    if x87_m32(value) > double_at(e, DOUBLE_ZERO) {
        let s1 = setting_bits(e, SETTING_011D003C);
        let part = x87_mul(x87_sub(1.0, x87_load32(s1)), x87_m32(value));
        let s2 = setting_bits(e, SETTING_011D0138);
        let part = x87_mul(x87_load32(s2), part);
        let s3 = x87_quiet_bits(setting_bits(e, SETTING_011D0C00));
        let curve = e
            .call(CURVE_004DD130, &args![x87_quiet_bits(scale), s3])
            .f64();
        let part = x87_mul(curve, part);
        let s1 = setting_bits(e, SETTING_011D003C);
        let total = x87_add(x87_mul(x87_load32(value), x87_m32(s1)), part);
        value = x87_store32(total);
    }
    let size = e
        .call(MAGNITUDE_00408860, &args![x87_quiet_bits(value)])
        .f64();
    let below = size < double_at(e, DOUBLE_ONE) || size.is_nan();
    let answer = if below {
        x87_quiet_bits(value)
    } else {
        let limited = e
            .call(
                LIMIT_004BD510,
                &args![x87_quiet_bits(value), x87_store32(1.0)],
            )
            .f64();
        x87_store32(limited)
    };
    f32::from_bits(answer)
}

/// `00514410` (`thiscall` on the third object; a byte answer), `00646cb0`
/// (`thiscall` on the second; a byte) and `00446390` (`thiscall` on the
/// second; a word), and `CombatFormulas::GetBodyPartCondition` (`00646800`,
/// `cdecl`: the actor, the body part's actor value, 0; `ST0`).
const KIND_OF_00514410: u32 = 0x0051_4410;
const TEST_00646CB0: u32 = 0x0064_6cb0;
const KIND_OF_00446390: u32 = 0x0044_6390;
const BODY_PART_CONDITION: u32 = 0x0064_6800;
/// The settings `00647d10` reads.
const SETTING_011D0EAC: u32 = 0x011d_0eac;
const SETTING_011D0024: u32 = 0x011d_0024;
const SETTING_011D09AC: u32 = 0x011d_09ac;
const SETTING_011D08BC: u32 = 0x011d_08bc;
const SETTING_011D11B8: u32 = 0x011d_11b8;
const SETTING_011D0268: u32 = 0x011d_0268;
const SETTING_011D0448: u32 = 0x011d_0448;
const SETTING_011D0C30: u32 = 0x011d_0c30;
const SETTING_011CFFF4: u32 = 0x011c_fff4;
/// The doubles 2.0 (`01011590`) and 0.01 (`01031148`).
const DOUBLE_TWO: u32 = 0x0101_1590;
const DOUBLE_HUNDREDTH: u32 = 0x0103_1148;

/// `total + setting` with `float` stores (`FLD; FADD dword; FSTP`).
fn add_setting(e: &mut Engine, total: u32, setting: u32) -> u32 {
    let bits = setting_bits(e, setting);
    x87_store32(x87_add(x87_load32(total), x87_m32(bits)))
}

/// `value * setting` with `float` stores (`FLD; FMUL dword; FSTP`).
fn scale_by_setting(e: &mut Engine, value: u32, setting: u32) -> u32 {
    let bits = setting_bits(e, setting);
    x87_store32(x87_mul(x87_load32(value), x87_m32(bits)))
}

// Translated from 00647d10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, seven words: an actor, a second and a
/// third object, and four bytes; the answer in `ST0`). A penalty starts at
/// 0.0: the third object's kind (`00514410`) 1 or 2 adds `011d0eac` or
/// `011d0024`; with the second object, the fifth byte clear and `00646cb0`
/// true, its kind (`00446390`, asked again for each test) 8 or 9 adds
/// `011d09ac`, any other but 2 adds `011d08bc`. The two body-part conditions
/// (actor values 0x1d and 0x1e) give `(1 - c2) + (1 - c1)`: 1.0 picks the
/// factor `011d11b8`, 2.0 `011d0268`, else 1.0; with the seventh byte clear
/// the actor's vtable slot `+8` with 0x48 answering non-zero makes it 1.0.
/// The value is the actor's slot `+0xc` (0x15) times 0.01 times `011d0448`
/// times the factor, times `011d0c30` with the sixth and fifth bytes set,
/// times `1 - penalty`, times `011cfff4` with the fourth byte set; below 0.0
/// answers 0.0.
#[allow(clippy::too_many_arguments)] // the exe's seven stack words
pub fn fn_00647d10(
    e: &mut Engine,
    actor: u32,
    second: u32,
    third: u32,
    fourth_flag: u8,
    fifth_flag: u8,
    sixth_flag: u8,
    seventh_flag: u8,
) -> f32 {
    let mut penalty = x87_store32(0.0);
    if third != 0 {
        let kind = e.call(KIND_OF_00514410, &args![third]).u32() as u8;
        if kind == 1 {
            penalty = add_setting(e, penalty, SETTING_011D0EAC);
        } else if kind == 2 {
            penalty = add_setting(e, penalty, SETTING_011D0024);
        }
    }
    if second != 0 && fifth_flag == 0 && e.call(TEST_00646CB0, &args![second]).u32() as u8 != 0 {
        if e.call(KIND_OF_00446390, &args![second]).u32() == 8
            || e.call(KIND_OF_00446390, &args![second]).u32() == 9
        {
            penalty = add_setting(e, penalty, SETTING_011D09AC);
        } else if e.call(KIND_OF_00446390, &args![second]).u32() != 2 {
            penalty = add_setting(e, penalty, SETTING_011D08BC);
        }
    }
    let first_condition = e
        .call(BODY_PART_CONDITION, &args![actor, 0x1du32, 0u32])
        .f64();
    let first_part = x87_sub(1.0, first_condition);
    let second_condition = e
        .call(BODY_PART_CONDITION, &args![actor, 0x1eu32, 0u32])
        .f64();
    let crippled = x87_store32(x87_add(x87_sub(1.0, second_condition), first_part));
    let mut factor = x87_store32(1.0);
    if x87_m32(crippled) == double_at(e, DOUBLE_ONE) {
        factor = x87_quiet_bits(setting_bits(e, SETTING_011D11B8));
    } else if x87_m32(crippled) == double_at(e, DOUBLE_TWO) {
        factor = x87_quiet_bits(setting_bits(e, SETTING_011D0268));
    }
    if seventh_flag == 0 && e.vcall(actor, 8, &args![0x48u32]).u32() != 0 {
        factor = x87_store32(1.0);
    }
    let skill = e.vcall(actor, 0xc, &args![0x15u32]).f64();
    let base = x87_store32(x87_mul(skill, double_at(e, DOUBLE_HUNDREDTH)));
    let scaled = setting_bits(e, SETTING_011D0448);
    let product = x87_mul(x87_mul(x87_load32(base), x87_m32(scaled)), x87_m32(factor));
    let mut value = x87_quiet_bits(x87_store32(product));
    if sixth_flag != 0 && fifth_flag != 0 {
        value = scale_by_setting(e, value, SETTING_011D0C30);
    }
    value = x87_store32(x87_mul(x87_sub(1.0, x87_load32(penalty)), x87_m32(value)));
    if fourth_flag != 0 {
        value = scale_by_setting(e, value, SETTING_011CFFF4);
    }
    if x87_m32(value) < double_at(e, DOUBLE_ZERO) {
        0.0
    } else {
        f32::from_bits(x87_quiet_bits(value))
    }
}

/// The setting `00647f00` scales by (`011d0898`).
const SETTING_011D0898: u32 = 0x011d_0898;

// Translated from 00647f00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, seven words like `00647d10`: three
/// objects and four bytes; the answer in `ST0`): `00647d10` with the bytes
/// reordered (the sixth, fourth, fifth and seventh as its fourth to seventh),
/// plus 0.0, times the setting `011d0898`, with `float` stores.
#[allow(clippy::too_many_arguments)] // the exe's seven stack words
pub fn fn_00647f00(
    e: &mut Engine,
    actor: u32,
    second: u32,
    third: u32,
    fourth_flag: u8,
    fifth_flag: u8,
    sixth_flag: u8,
    seventh_flag: u8,
) -> f32 {
    let zero = x87_store32(0.0);
    let value = fn_00647d10(
        e,
        actor,
        second,
        third,
        sixth_flag,
        fourth_flag,
        fifth_flag,
        seventh_flag,
    );
    let sum = x87_store32(x87_add(f64::from(value), x87_m32(zero)));
    let result = scale_by_setting(e, sum, SETTING_011D0898);
    f32::from_bits(x87_quiet_bits(result))
}

/// `004579e0` (`cdecl`, one float; a float in `ST0`): the curve
/// `00647960` applies.
const CURVE_004579E0: u32 = 0x0045_79e0;
/// `RandomFloat` (engine map name of `00476b70`; `cdecl`, two floats: the
/// low and high limit; `ST0`).
const RANDOM_FLOAT: u32 = 0x0047_6b70;
/// `_ftol2` (`00ec62c0`): `ST0` truncated to an integer.
const FTOL2: u32 = 0x00ec_62c0;
/// The doubles `00647a30` scales by (`01016408`, `01017a40`) (`00647fd0` scales by the
/// hundredth at `01031148`).
const SCALE_01016408: u32 = 0x0101_6408;
const HUNDRED_01017A40: u32 = 0x0101_7a40;

/// The x87 compare `FCOMP` of `a` with `b` as `FNSTSW AX; TEST AH,5; JP`
/// takes the jump: neither less nor unordered.
fn not_below(a: f64, b: f64) -> bool {
    a >= b
}

// Translated from 00647960 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, two floats; the answer in `ST0`):
/// `t = 1 - second` (a `float`); 0.0 when `t` is below the `double` at
/// `01012060` (or unordered), else `004579e0(t) * first` stored as a
/// `float`.
pub fn fn_00647960(e: &mut Engine, first: f32, second: f32) -> f32 {
    let t = x87_store32(x87_sub(1.0, x87_load32(second.to_bits())));
    if !not_below(x87_load32(t), double_at(e, DOUBLE_ZERO)) {
        return 0.0;
    }
    let curve = e.call(CURVE_004579E0, &args![x87_quiet_bits(t)]).f64();
    f32::from_bits(x87_store32(x87_mul(curve, x87_m32(first.to_bits()))))
}

// Translated from 006479a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, three floats; `ST0`): `t = third /
/// second` (a `float`); 0.0 when `t` is above the `double` 1.0 at
/// `01012070`, else `00647960(first, t)` (also for an unordered `t`).
pub fn fn_006479a0(e: &mut Engine, first: f32, second: f32, third: f32) -> f32 {
    let t = x87_store32(x87_div(
        x87_load32(third.to_bits()),
        x87_m32(second.to_bits()),
    ));
    let value = x87_load32(t);
    let one = double_at(e, DOUBLE_ONE);
    // `TEST AH,0x41; JNZ`: less, equal or unordered.
    if !(value <= one || value.is_nan()) {
        return 0.0;
    }
    fn_00647960(
        e,
        f32::from_bits(x87_quiet_bits(first.to_bits())),
        f32::from_bits(x87_quiet_bits(t)),
    )
}

// Translated from 00647a30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`: an object, a second object, a
/// `float`; `ST0`): 0.0 unless both objects are set. `rate` is the second
/// object's byte (`00647ac0`) times the `double` at `01016408`, `value` the
/// first object's virtual slot 4 with 0x10 (both stored as `float`s). A
/// `value` at most 0.0 answers the `float`; otherwise, with `rate` above
/// 0.0, `amount / (rate * value)` times the `double` at `01017a40`, else
/// 0.0.
pub fn fn_00647a30(e: &mut Engine, object: u32, other: Ptr, amount: f32) -> f32 {
    let mut result = x87_store32(0.0);
    if object != 0 && !other.is_null() {
        let byte = fn_00647ac0(e, other);
        let rate = x87_store32(x87_mul(f64::from(byte), double_at(e, SCALE_01016408)));
        let value = x87_store32(e.vcall(object, 4, &args![0x10u32]).f64());
        let zero = double_at(e, DOUBLE_ZERO);
        let loaded = x87_load32(value);
        if loaded <= zero {
            result = x87_quiet_bits(amount.to_bits());
        } else {
            let rate_loaded = x87_load32(rate);
            if !(rate_loaded <= zero || rate_loaded.is_nan()) {
                let product = x87_mul(rate_loaded, x87_m32(value));
                let quotient = x87_div(x87_m32(amount.to_bits()), product);
                result = x87_store32(x87_mul(quotient, double_at(e, HUNDRED_01017A40)));
            }
        }
    }
    f32::from_bits(x87_quiet_bits(result))
}

// Translated from 00647ac0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The byte at +0x62.
pub fn fn_00647ac0(e: &mut Engine, this: Ptr) -> u8 {
    e.mem.u8(this.addr() + 0x62)
}

// Translated from 00647ae0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, one float): `s1 + (value * s3 + s2)
/// + RandomFloat(-s4, s4)` with the settings `s1` `011cee24`, `s2`
/// `011cf314`, `s3` `011ced28` and `s4` `011ceb38` (each part stored as a
/// `float`), truncated (`_ftol2`) and at least 0 (`00647b70`).
pub fn fn_00647ae0(e: &mut Engine, value: f32) -> i32 {
    let base = x87_quiet_bits(setting_bits(e, 0x011c_ee24));
    let offset = setting_bits(e, 0x011c_f314);
    let factor = setting_bits(e, 0x011c_ed28);
    let scaled = x87_store32(x87_add(
        x87_mul(x87_load32(value.to_bits()), x87_m32(factor)),
        x87_m32(offset),
    ));
    let high = x87_quiet_bits(setting_bits(e, 0x011c_eb38));
    let low = x87_quiet_bits(setting_bits(e, 0x011c_eb38)) ^ 0x8000_0000;
    let random = x87_store32(e.call(RANDOM_FLOAT, &args![low, high]).f64());
    let sum = x87_add(x87_add(x87_load32(base), x87_m32(scaled)), x87_m32(random));
    let whole = e.call(FTOL2, &args![sum]).i32();
    fn_00647b70(e, 0, whole)
}

// Translated from 00647bd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`: the address of an unsigned integer,
/// a low and a high limit): clamps the integer, the high limit tested
/// first.
pub fn fn_00647bd0(e: &mut Engine, value: Ptr, low: u32, high: u32) {
    let at = value.addr();
    if e.mem.u32(at) > high {
        e.mem.set_u32(at, high);
    } else if e.mem.u32(at) < low {
        e.mem.set_u32(at, low);
    }
}

// Translated from 00647cf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, one word): the word as an unsigned
/// number (`FILD qword` of it with a zero high word), in `ST0`.
pub fn fn_00647cf0(_e: &mut Engine, value: u32) -> f64 {
    f64::from(value)
}

// Translated from 00647f50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, six words): `00647d10` with the
/// three words, 0 and the three bytes, stored as a `float`.
pub fn fn_00647f50(
    e: &mut Engine,
    actor: u32,
    second: u32,
    third: u32,
    fourth: u8,
    fifth: u8,
    sixth: u8,
) -> f32 {
    let value = fn_00647d10(e, actor, second, third, 0, fourth, fifth, sixth);
    f32::from_bits(x87_quiet_bits(value.to_bits()))
}

// Translated from 00647f90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, six words): `00647f00` with the
/// three words, the first two bytes, 0 and the third byte, stored as a
/// `float`.
pub fn fn_00647f90(
    e: &mut Engine,
    actor: u32,
    second: u32,
    third: u32,
    fourth: u8,
    fifth: u8,
    sixth: u8,
) -> f32 {
    let value = fn_00647f00(e, actor, second, third, fourth, fifth, 0, sixth);
    f32::from_bits(x87_quiet_bits(value.to_bits()))
}

// Translated from 00647fd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, one word: an actor value owner):
/// `s1 + (s2 - s1) * (value * k)` with the owner's virtual slot 0xC for
/// 0x15 as `value`, the `double` `k` at `01031148` and the settings `s1`
/// `011d0cc0` and `s2` `011d0e1c` (each part stored as a `float`); 0.0
/// when that is below the `double` at `01012060` (or unordered).
pub fn fn_00647fd0(e: &mut Engine, owner: u32) -> f32 {
    let value = e.vcall(owner, 0xc, &args![0x15u32]).f64();
    let scaled = x87_store32(x87_mul(value, double_at(e, DOUBLE_HUNDREDTH)));
    let start = x87_quiet_bits(setting_bits(e, 0x011d_0cc0));
    let end = setting_bits(e, 0x011d_0e1c);
    let start_again = setting_bits(e, 0x011d_0cc0);
    let span = x87_sub(x87_load32(end), x87_m32(start_again));
    let total = x87_store32(x87_add(x87_mul(span, x87_m32(scaled)), x87_m32(start)));
    if !not_below(x87_load32(total), double_at(e, DOUBLE_ZERO)) {
        return 0.0;
    }
    f32::from_bits(x87_quiet_bits(total))
}

// Translated from 00648050 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, one float): `value * 10.0 (the
/// double at 01020758) * s2 + s1` with the settings `s1` `011d05ec` and
/// `s2` `011d0fd8`, stored as a `float`.
pub fn fn_00648050(e: &mut Engine, value: f32) -> f32 {
    let offset = setting_bits(e, 0x011d_05ec);
    let factor = setting_bits(e, 0x011d_0fd8);
    let scaled = x87_mul(x87_load32(value.to_bits()), double_at(e, DOUBLE_TEN));
    let total = x87_add(x87_mul(scaled, x87_m32(factor)), x87_m32(offset));
    f32::from_bits(x87_quiet_bits(x87_store32(total)))
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0064_7b70, fn_00647b70(i32, i32) -> i32),
        entry!(0x0064_7ba0, fn_00647ba0(Ptr, i32, i32)),
        entry!(0x0064_79e0, fn_006479e0(u32) -> f32),
        entry!(0x0064_7c00, fn_00647c00(f32, f32) -> f32),
        entry!(0x0064_7d10, fn_00647d10(u32, u32, u32, u8, u8, u8, u8) -> f32),
        entry!(0x0064_7f00, fn_00647f00(u32, u32, u32, u8, u8, u8, u8) -> f32),
        entry!(0x0064_7960, fn_00647960(f32, f32) -> f32),
        entry!(0x0064_79a0, fn_006479a0(f32, f32, f32) -> f32),
        entry!(0x0064_7a30, fn_00647a30(u32, Ptr, f32) -> f32),
        entry!(0x0064_7ac0, fn_00647ac0(Ptr) -> u8),
        entry!(0x0064_7ae0, fn_00647ae0(f32) -> i32),
        entry!(0x0064_7bd0, fn_00647bd0(Ptr, u32, u32)),
        entry!(0x0064_7cf0, fn_00647cf0(u32) -> f64),
        entry!(0x0064_7f50, fn_00647f50(u32, u32, u32, u8, u8, u8) -> f32),
        entry!(0x0064_7f90, fn_00647f90(u32, u32, u32, u8, u8, u8) -> f32),
        entry!(0x0064_7fd0, fn_00647fd0(u32) -> f32),
        entry!(0x0064_8050, fn_00648050(f32) -> f32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Maps the setting objects at `settings` with their float values: each
    /// object's value word sits right after it (`00403e20` answers +4).
    fn engine(settings: &[(u32, f32)]) -> Engine {
        let mut e = Engine::new();
        e.register(SETTING_VALUE, |_, a| (a[0] + 4).into_ret());
        for (setting, value) in settings {
            e.map(*setting, 8);
            e.mem.set_f32(setting + 4, *value);
        }
        e
    }

    fn doubles(e: &mut Engine) {
        for (at, value) in [
            (DOUBLE_TEN, 10.0f64),
            (DOUBLE_ZERO, 0.0),
            (DOUBLE_ONE, 1.0),
            (DOUBLE_TWO, 2.0),
            (DOUBLE_HUNDREDTH, 0.01),
        ] {
            e.map(at, 8);
            e.mem.set_u64(at, value.to_bits());
        }
        e.map(FLOAT_TEN, 4);
        e.mem.set_f32(FLOAT_TEN, 10.0);
    }

    #[test]
    fn fn_00647b70_answers_the_larger() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x0064_7b70, &args![5i32, -3i32]).i32(), 5);
        assert_eq!(e.call(0x0064_7b70, &args![-5i32, -3i32]).i32(), -3);
    }

    #[test]
    fn fn_00647ba0_clamps() {
        let mut e = Engine::new();
        let v = e.mem.alloc(4);
        for (start, end) in [(12i32, 10i32), (-4, 0), (5, 5)] {
            e.mem.set_u32(v, start as u32);
            e.call(0x0064_7ba0, &args![v, 0i32, 10i32]);
            assert_eq!(e.mem.u32(v) as i32, end);
        }
    }

    #[test]
    fn fn_006479e0_scales_the_value() {
        let mut e = engine(&[(SETTING_011CFB5C, 0.25), (SETTING_011CF264, 0.5)]);
        e.register(VALUE_OF_006A78F0, |_, _| Ret {
            st0: 4.0,
            ..Ret::default()
        });
        assert_eq!(e.call(0x0064_79e0, &args![0x100u32]).f32(), 3.0);
        assert_eq!(e.call(0x0064_79e0, &args![0u32]).f32(), 0.0);
    }

    #[test]
    fn fn_00647c00_grows_and_limits() {
        let mut e = engine(&[
            (SETTING_011D003C, 0.5),
            (SETTING_011D0138, 2.0),
            (SETTING_011D0C00, 3.0),
        ]);
        doubles(&mut e);
        e.register(CURVE_004DD130, |_, a| {
            // The scale (here 20 / 10) and the third setting come as floats.
            assert_eq!(f32::from_bits(a[1]), 3.0);
            Ret {
                st0: f64::from(f32::from_bits(a[0])),
                ..Ret::default()
            }
        });
        e.register(MAGNITUDE_00408860, |_, a| Ret {
            st0: f64::from(f32::from_bits(a[0])).abs(),
            ..Ret::default()
        });
        e.register(LIMIT_004BD510, |_, a| Ret {
            st0: f64::from(f32::from_bits(a[0])) - 1.0,
            ..Ret::default()
        });
        // 0.25 * 0.5 + 2 * (2 * (0.5 * 0.25)) = 0.625: below 1, kept.
        assert_eq!(e.call(0x0064_7c00, &args![0.25f32, 20.0f32]).f32(), 0.625);
        // A negative second makes the scale 10: 1 * 0.5 + 10 * (2 * 0.5) = 10.5,
        // limited by 004bd510.
        assert_eq!(e.call(0x0064_7c00, &args![1.0f32, -5.0f32]).f32(), 9.5);
        // Not above 0: unchanged (magnitude 0.5 is below 1).
        assert_eq!(e.call(0x0064_7c00, &args![-0.5f32, 20.0f32]).f32(), -0.5);
    }

    /// `00647d10`'s engine: the actor's vtable slots `+8` and `+0xc`, the body
    /// part conditions `conditions`, the third object's kind and the second's.
    fn penalty_engine(
        conditions: (f64, f64),
        kind: u32,
        second_kind: u32,
        perk: u32,
    ) -> (Engine, u32) {
        let mut e = engine(&[
            (SETTING_011D0EAC, 0.25),
            (SETTING_011D0024, 0.5),
            (SETTING_011D09AC, 0.125),
            (SETTING_011D08BC, 0.0625),
            (SETTING_011D11B8, 0.5),
            (SETTING_011D0268, 0.25),
            (SETTING_011D0448, 2.0),
            (SETTING_011D0C30, 0.5),
            (SETTING_011CFFF4, 4.0),
            (SETTING_011D0898, 3.0),
        ]);
        doubles(&mut e);
        e.register_double(KIND_OF_00514410, move |_, _| kind.into_ret());
        e.register(TEST_00646CB0, |_, _| 1u32.into_ret());
        e.register_double(KIND_OF_00446390, move |_, _| second_kind.into_ret());
        e.register_double(BODY_PART_CONDITION, move |_, a| Ret {
            st0: if a[1] == 0x1d {
                conditions.0
            } else {
                conditions.1
            },
            ..Ret::default()
        });
        const SLOT_8: u32 = 0x0136_0000;
        const SLOT_C: u32 = 0x0136_0010;
        e.register_double(SLOT_8, move |_, _| perk.into_ret());
        e.register(SLOT_C, |_, _| Ret {
            st0: 50.0,
            ..Ret::default()
        });
        let table = e.mem.alloc(0x10);
        e.mem.set_u32(table + 8, SLOT_8);
        e.mem.set_u32(table + 0xc, SLOT_C);
        let actor = e.mem.alloc(4);
        e.mem.set_u32(actor, table);
        (e, actor)
    }

    fn penalty(e: &mut Engine, actor: u32, flags: [u32; 4]) -> f32 {
        e.call(
            0x0064_7d10,
            &args![actor, 0x200u32, 0x300u32, flags[0], flags[1], flags[2], flags[3]],
        )
        .f32()
    }

    #[test]
    fn fn_00647d10_combines_the_factors() {
        // Healthy arms (factor 1): 50 * 0.01 * 2 = 1; penalty 0.25 + 0.0625.
        let (mut e, actor) = penalty_engine((1.0, 1.0), 1, 3, 0);
        assert_eq!(penalty(&mut e, actor, [0, 0, 0, 1]), 1.0 - 0.3125);
        // One crippled arm: factor 0.5; kind 8 adds 0.125; the fourth byte
        // multiplies by 4.
        let (mut e, actor) = penalty_engine((0.0, 1.0), 2, 8, 0);
        assert_eq!(
            penalty(&mut e, actor, [1, 0, 0, 1]),
            0.5 * (1.0 - 0.625) * 4.0
        );
        // Both crippled (factor 0.25) but the perk check (seventh byte clear)
        // resets it to 1; the fifth and sixth bytes multiply by 0.5 and skip
        // the second object's penalty.
        let (mut e, actor) = penalty_engine((0.0, 0.0), 0, 8, 1);
        assert_eq!(penalty(&mut e, actor, [0, 1, 1, 0]), 0.5);
    }

    #[test]
    fn fn_00647d10_is_never_negative() {
        let (mut e, actor) = penalty_engine((1.0, 1.0), 2, 3, 0);
        e.mem.set_f32(SETTING_011D0024 + 4, 3.0);
        assert_eq!(penalty(&mut e, actor, [0, 0, 0, 1]), 0.0);
    }

    #[test]
    fn fn_00647f00_reorders_and_scales() {
        let (mut e, actor) = penalty_engine((1.0, 1.0), 0, 2, 0);
        e.call_log = Some(vec![]);
        // Its fourth byte is 00647d10's fifth: set, the second object's
        // penalty is skipped; the sixth (00647d10's fourth) multiplies by 4.
        let value = e
            .call(
                0x0064_7f00,
                &args![actor, 0x200u32, 0x300u32, 1u32, 0u32, 1u32, 1u32],
            )
            .f32();
        assert_eq!(value, 1.0 * 4.0 * 3.0);
        let log = e.call_log.take().unwrap();
        assert!(log.iter().all(|c| c.0 != TEST_00646CB0));
    }
}

#[cfg(test)]
mod tests_n042 {
    use super::*;

    fn engine() -> Engine {
        let mut e = Engine::new();
        for (address, value) in [
            (DOUBLE_ZERO, 0.0f64),
            (DOUBLE_ONE, 1.0),
            (DOUBLE_TEN, 10.0),
            (SCALE_01016408, 0.5),
            (HUNDRED_01017A40, 2.0),
            (DOUBLE_HUNDREDTH, 0.01),
        ] {
            e.map(address, 8);
            e.set_global::<f64>(address, value);
        }
        e
    }

    /// Settings: each setting's value lives in a block the double hands
    /// out; `values` maps setting addresses to floats.
    fn settings(e: &mut Engine, values: &[(u32, f32)]) {
        let block = e.mem.alloc(4 * values.len() as u32);
        let mut table = Vec::new();
        for (i, (setting, value)) in values.iter().enumerate() {
            let at = block + 4 * i as u32;
            e.mem.set_u32(at, value.to_bits());
            table.push((*setting, at));
        }
        e.register_double(SETTING_VALUE, move |_, a| {
            table.iter().find(|(s, _)| *s == a[0]).unwrap().1.into_ret()
        });
    }

    fn ret_f64(value: f64) -> Ret {
        Ret {
            st0: value,
            ..Ret::default()
        }
    }

    #[test]
    fn curve_of_one_minus() {
        let mut e = engine();
        e.register(CURVE_004579E0, |_, a| {
            ret_f64(f64::from(f32::from_bits(a[0])) * 4.0)
        });
        let r = e.call(0x0064_7960, &args![3.0f32, 0.25f32]).f32();
        assert_eq!(r, 9.0);
        // 1 - 2 is below 0: 0.0 without the curve.
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0064_7960, &args![3.0f32, 2.0f32]).f32(), 0.0);
        let log = e.call_log.take().unwrap();
        assert!(log.iter().all(|c| c.0 != CURVE_004579E0));
        // A NaN second makes t unordered: 0.0.
        assert_eq!(e.call(0x0064_7960, &args![3.0f32, f32::NAN]).f32(), 0.0);
    }

    #[test]
    fn ratio_at_most_one_goes_through_the_curve() {
        let mut e = engine();
        e.register(CURVE_004579E0, |_, a| {
            ret_f64(f64::from(f32::from_bits(a[0])))
        });
        // t = 1 / 4, curve(1 - t) * 2.
        let r = e.call(0x0064_79a0, &args![2.0f32, 4.0f32, 1.0f32]).f32();
        assert_eq!(r, 1.5);
        // t = 4 / 2 above 1: 0.0.
        assert_eq!(
            e.call(0x0064_79a0, &args![2.0f32, 2.0f32, 4.0f32]).f32(),
            0.0
        );
        // t exactly 1: 1 - 1 = 0, not below 0, curve(0) * 2 = 0.
        assert_eq!(
            e.call(0x0064_79a0, &args![2.0f32, 2.0f32, 2.0f32]).f32(),
            0.0
        );
    }

    #[test]
    fn rate_formula() {
        let mut e = engine();
        e.register(0x0f00_1004, |e, a| {
            ret_f64(f64::from(f32::from_bits(e.mem.u32(a[0] + 4))))
        });
        e.put_vtable(0x0f00_0000, &[0, 0x0f00_1004]);
        let object = e.mem.alloc(8);
        e.mem.set_u32(object, 0x0f00_0000);
        e.mem.set_u32(object + 4, 4.0f32.to_bits());
        let other = e.mem.alloc(0x70);
        e.mem.set_u8(other + 0x62, 6);
        // rate 3, value 4: 12 / (3 * 4) * 2.
        let r = e.call(0x0064_7a30, &args![object, other, 12.0f32]).f32();
        assert_eq!(r, 2.0);
        // value 0: the amount.
        e.mem.set_u32(object + 4, 0.0f32.to_bits());
        assert_eq!(
            e.call(0x0064_7a30, &args![object, other, 12.0f32]).f32(),
            12.0
        );
        // rate 0 with a positive value: 0.0.
        e.mem.set_u32(object + 4, 4.0f32.to_bits());
        e.mem.set_u8(other + 0x62, 0);
        assert_eq!(
            e.call(0x0064_7a30, &args![object, other, 12.0f32]).f32(),
            0.0
        );
        // No object: 0.0.
        assert_eq!(e.call(0x0064_7a30, &args![0u32, other, 12.0f32]).f32(), 0.0);
        assert_eq!(
            e.call(0x0064_7a30, &args![object, 0u32, 12.0f32]).f32(),
            0.0
        );
        assert_eq!(e.call(0x0064_7ac0, &args![other]).u32() & 0xff, 0);
    }

    #[test]
    fn random_total_is_truncated_and_not_negative() {
        let mut e = engine();
        settings(
            &mut e,
            &[
                (0x011c_ee24, 1.0),
                (0x011c_f314, 2.0),
                (0x011c_ed28, 3.0),
                (0x011c_eb38, 0.5),
            ],
        );
        e.register(RANDOM_FLOAT, |_, a| {
            assert_eq!(a[0], (-0.5f32).to_bits());
            assert_eq!(a[1], 0.5f32.to_bits());
            ret_f64(0.25)
        });
        e.register(FTOL2, |_, a| {
            let value = f64::from_bits(u64::from(a[0]) | u64::from(a[1]) << 32);
            (value as i32 as u32).into_ret()
        });
        // 1 + (2 * 3 + 2) + 0.25 = 9.25.
        assert_eq!(e.call(0x0064_7ae0, &args![2.0f32]).i32(), 9);
        assert_eq!(e.call(0x0064_7ae0, &args![-10.0f32]).i32(), 0);
    }

    #[test]
    fn unsigned_clamp_and_word_as_number() {
        let mut e = engine();
        let value = e.mem.alloc(4);
        for (start, expected) in [(5u32, 5u32), (20, 10), (1, 3), (0x8000_0000, 10)] {
            e.mem.set_u32(value, start);
            e.call(0x0064_7bd0, &args![value, 3u32, 10u32]);
            assert_eq!(e.mem.u32(value), expected);
        }
        assert_eq!(
            e.call(0x0064_7cf0, &args![0xffff_ffffu32]).f64(),
            4294967295.0
        );
    }

    #[test]
    fn wrappers_pass_their_bytes_in_place() {
        let mut e = engine();
        settings(
            &mut e,
            &[
                (SETTING_011D0448, 1.0),
                (SETTING_011D0C30, 2.0),
                (SETTING_011CFFF4, 3.0),
                (SETTING_011D0898, 1.0),
            ],
        );
        e.map(DOUBLE_TWO, 8);
        e.set_global::<f64>(DOUBLE_TWO, 2.0);
        // Healthy body parts, no slot-8 effect, a skill of 100.
        e.register(BODY_PART_CONDITION, |_, _| ret_f64(1.0));
        e.register(0x0f00_1008, |_, _| Ret::default());
        e.register(0x0f00_100c, |_, _| ret_f64(100.0));
        e.put_vtable(0x0f00_0000, &[0, 0, 0x0f00_1008, 0x0f00_100c]);
        let actor = e.mem.alloc(4);
        e.mem.set_u32(actor, 0x0f00_0000);
        let call = |e: &mut Engine, addr: u32, d: u32, f: u32| {
            e.call(addr, &args![actor, 0u32, 0u32, d, f, 0u32]).f32()
        };
        // 00647f50: its bytes are 00647d10's fifth and sixth (the fourth
        // is 0); both set double the value (011d0c30).
        assert_eq!(call(&mut e, 0x0064_7f50, 1, 1), 2.0);
        assert_eq!(call(&mut e, 0x0064_7f50, 0, 1), 1.0);
        // 00647f90: 00647f00 with (d, f, 0, ...), whose 00647d10 gets them
        // as its fifth and sixth again.
        assert_eq!(call(&mut e, 0x0064_7f90, 1, 1), 2.0);
        assert_eq!(call(&mut e, 0x0064_7f90, 1, 0), 1.0);
    }

    #[test]
    fn owner_formula_with_floor() {
        let mut e = engine();
        settings(&mut e, &[(0x011d_0cc0, 1.0), (0x011d_0e1c, 3.0)]);
        e.register(0x0f00_100c, |e, a| {
            ret_f64(f64::from(f32::from_bits(e.mem.u32(a[0] + 4))))
        });
        e.put_vtable(0x0f00_0000, &[0, 0, 0, 0x0f00_100c]);
        let owner = e.mem.alloc(8);
        e.mem.set_u32(owner, 0x0f00_0000);
        e.mem.set_u32(owner + 4, 50.0f32.to_bits());
        // 1 + (3 - 1) * (50 * 0.01) = 2.
        assert_eq!(e.call(0x0064_7fd0, &args![owner]).f32(), 2.0);
        e.mem.set_u32(owner + 4, (-200.0f32).to_bits());
        assert_eq!(e.call(0x0064_7fd0, &args![owner]).f32(), 0.0);
    }

    #[test]
    fn scaled_setting_sum() {
        let mut e = engine();
        settings(&mut e, &[(0x011d_05ec, 1.0), (0x011d_0fd8, 0.5)]);
        // 3 * 10 * 0.5 + 1.
        assert_eq!(e.call(0x0064_8050, &args![3.0f32]).f32(), 16.0);
    }
}

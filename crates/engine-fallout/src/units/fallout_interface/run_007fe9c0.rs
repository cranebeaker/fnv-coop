//! `fallout/interface/run_007fe9c0` (Xbox PDB source unit), subsystem `fallout/interface`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The functions here work on a `FORenderedMenuRaceSex` (the RTTI of the vtable its
//! destructor stores): the fields at +0x150..+0x168 time a flicker and a fade of the
//! rendered head, written into shader constants at `0120081c..01200838`, `011ae600..08`.

#[allow(unused_imports)]
use crate::prelude::*;

/// `00507e10` (`thiscall`): the float at +0x8c, in ST0.
const FLOAT_8C_00507E10: u32 = 0x0050_7e10;
/// `00403e20` (`thiscall`): the address of a `Setting`'s value.
const SETTING_VALUE: u32 = 0x0040_3e20;
/// The `Setting`s at `011db520`, `011db464` and `011db5b4`.
const SETTING_011DB520: u32 = 0x011d_b520;
const SETTING_011DB464: u32 = 0x011d_b464;
const SETTING_011DB5B4: u32 = 0x011d_b5b4;
/// `0043d4d0` (`thiscall`) on the `Setting` at `011db4f8`: the address of its integer.
const SETTING_INT_0043D4D0: u32 = 0x0043_d4d0;
const SETTING_011DB4F8: u32 = 0x011d_b4f8;
/// `007feb40` (`thiscall`, one word: the result): a colour of the menu (four words).
const COLOR_007FEB40: u32 = 0x007f_eb40;
/// `00414430` (`thiscall`, four `float`s): builds a colour, answers its address.
const MAKE_COLOR_00414430: u32 = 0x0041_4430;
/// The shader constants written: three floats and a colour, and three more floats.
const CONSTANT_011AE600: u32 = 0x011a_e600;
const CONSTANT_0120081C: u32 = 0x0120_081c;
const CONSTANT_011AE604: u32 = 0x011a_e604;
const CONSTANT_01200820: u32 = 0x0120_0820;
const CONSTANT_0120082C: u32 = 0x0120_082c;
const CONSTANT_01200824: u32 = 0x0120_0824;
const CONSTANT_011AE608: u32 = 0x011a_e608;
const CONSTANT_01200828: u32 = 0x0120_0828;
/// `-1.0` (`float`, `01012054`), `0.001` (`double`, `01016978`), `1.0` (`double`,
/// `01012070`).
const MINUS_ONE: u32 = 0x0101_2054;
const THOUSANDTH: u32 = 0x0101_6978;
const ONE: u32 = 0x0101_2070;
/// `0043c4b0` and `004e3270` (no words): two singletons (the second the image-space
/// manager `00b97550` runs on).
const OBJECT_0043C4B0: u32 = 0x0043_c4b0;
const IMAGE_SPACE_MANAGER_004E3270: u32 = 0x004e_3270;
/// `00b97550` (`imagespacemanager.cpp`, `thiscall`, six words).
const RENDER_EFFECT_00B97550: u32 = 0x00b9_7550;
/// `_rand` (`00ecadb8`).
const RAND: u32 = 0x00ec_adb8;
/// `00825c00` (`thiscall`) on `011f6394`: its word +0x14 (milliseconds).
const TIME_00825C00: u32 = 0x0082_5c00;
const TIMER_011F6394: u32 = 0x011f_6394;
/// `FORenderedMenuRaceSex`'s vtable (`010759ec`).
const VTABLE: u32 = 0x0107_59ec;
/// The `NiPointer` assignment `0066b0d0` (`thiscall`, one word) and destructor
/// `0045cec0` (`thiscall`).
const POINTER_ASSIGN: u32 = 0x0066_b0d0;
const POINTER_DESTRUCTOR: u32 = 0x0045_cec0;
/// The vector destructor iterator (`00ec5fce`, `stdcall`, four words).
const VECTOR_DESTRUCTOR: u32 = 0x00ec_5fce;
/// The base destructor `007fad10` (`thiscall`).
const BASE_DESTRUCTOR_007FAD10: u32 = 0x007f_ad10;

/// The value of a `Setting` as an `FLD`/`FSTP` copy leaves it.
fn setting_bits(e: &mut Engine, setting: u32) -> u32 {
    let value = e.call(SETTING_VALUE, &args![setting]).u32();
    x87_quiet_bits(e.mem.u32(value))
}

/// Copies the four words at `source` to `CONSTANT_0120082C`.
fn store_color(e: &mut Engine, source: u32) {
    for i in 0..4 {
        let word = e.mem.u32(source + 4 * i);
        e.mem.set_u32(CONSTANT_0120082C + 4 * i, word);
    }
}

// Translated from 007fe9c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 0xc`: two words passed on, an unread word).
/// A local flag the code sets to 1 and at once to 0 chooses the shader constants: with it
/// (never, as compiled) they come from the menu (`00507e10`, the `Setting`s at `011db520`
/// and `011db464`, the float +0x154, the colour of `007feb40`); without it they are 0.0,
/// 0.0, -1.0 (`01012054`), 1.0 and the colour (1, 1, 1, 1) (`00414430`). A second set comes
/// from the settings and +0x160 when the byte +0x158 and the flag are set, else 0.0, 0.0,
/// -1.0. Then the image-space manager (`004e3270`) runs `00b97550` with 0x1e, the object of
/// `0043c4b0`, the two words, 0 and 1.
pub fn fn_007fe9c0(e: &mut Engine, this: Ptr, first: u32, second: u32, _unused_3: u32) {
    // MOV byte [EBP-1],1; MOV byte [EBP-1],0: the flag is always clear here, but both
    // branches are kept as the exe has them.
    let flag = std::hint::black_box(false);
    e.with_stack(0x20, |e, colors| {
        if flag {
            let value = e.call(FLOAT_8C_00507E10, &args![this]).f64();
            e.mem.set_u32(CONSTANT_011AE600, x87_store32(value));
            let bits = setting_bits(e, SETTING_011DB520);
            e.mem.set_u32(CONSTANT_0120081C, bits);
            let bits = setting_bits(e, SETTING_011DB464);
            e.mem.set_u32(CONSTANT_011AE604, bits);
            let bits = x87_quiet_bits(e.mem.u32(this.addr() + 0x154));
            e.mem.set_u32(CONSTANT_01200820, bits);
            let color = e
                .call(COLOR_007FEB40, &args![this, colors.addr() + 0x10])
                .u32();
            store_color(e, color);
        } else {
            e.mem.set_u32(CONSTANT_0120081C, 0);
            e.mem.set_u32(CONSTANT_011AE604, 0);
            let bits = x87_quiet_bits(e.mem.u32(MINUS_ONE));
            e.mem.set_u32(CONSTANT_01200820, bits);
            e.mem.set_f32(CONSTANT_011AE600, 1.0);
            let color = e
                .call(
                    MAKE_COLOR_00414430,
                    &args![colors, 1.0f32, 1.0f32, 1.0f32, 1.0f32],
                )
                .u32();
            store_color(e, color);
        }
    });
    if e.mem.u8(this.addr() + 0x158) != 0 && flag {
        let bits = setting_bits(e, SETTING_011DB520);
        e.mem.set_u32(CONSTANT_01200824, bits);
        let bits = setting_bits(e, SETTING_011DB464);
        e.mem.set_u32(CONSTANT_011AE608, bits);
        let bits = x87_quiet_bits(e.mem.u32(this.addr() + 0x160));
        e.mem.set_u32(CONSTANT_01200828, bits);
    } else {
        e.mem.set_u32(CONSTANT_01200824, 0);
        e.mem.set_u32(CONSTANT_011AE608, 0);
        let bits = x87_quiet_bits(e.mem.u32(MINUS_ONE));
        e.mem.set_u32(CONSTANT_01200828, bits);
    }
    let object = e.call(OBJECT_0043C4B0, &args![]).u32();
    let manager = e.call(IMAGE_SPACE_MANAGER_004E3270, &args![]).u32();
    e.call(
        RENDER_EFFECT_00B97550,
        &args![manager, 0x1eu32, object, first, second, 0u32, 1u32],
    );
}

// Translated from 007feb80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`: the time). When more than the value of
/// the `Setting` at `011db5b4` has passed since +0x150 (not when NaN), +0x150 takes the time
/// and +0x154 a random `rand() % 1000 * 0.001`. While the byte +0x158 is set the menu's
/// slot +0x54 gets the time; otherwise, once the time passes +0x168, the fade starts
/// ([`fn_007fecc0`]).
pub fn fn_007feb80(e: &mut Engine, this: Ptr, time: u32) {
    let base = this.addr();
    let elapsed = x87_sub(x87_load32(time), x87_m32(e.mem.u32(base + 0x150)));
    let interval = e.call(SETTING_VALUE, &args![SETTING_011DB5B4]).u32();
    let interval = x87_load32(e.mem.u32(interval));
    // FCOMP double; TEST AH,5; JP: goes on only for interval < elapsed.
    if interval < elapsed {
        e.mem.set_u32(base + 0x150, x87_quiet_bits(time));
        let random = e.call(RAND, &args![]).i32();
        let remainder = random % 1000;
        let thousandth = e.mem.f64(THOUSANDTH);
        let value = x87_store32(x87_mul(f64::from(remainder), thousandth));
        e.mem.set_u32(base + 0x154, value);
    }
    if e.mem.u8(base + 0x158) != 0 {
        e.vcall(base, 0x54, &args![x87_quiet_bits(time)]);
    } else {
        let start = x87_load32(e.mem.u32(base + 0x168));
        // FLD time; FLD +0x168; FCOMPP; TEST AH,5; JP: only for +0x168 < time.
        if start < x87_load32(time) {
            fn_007fecc0(e, this);
        }
    }
}

// Translated from 007fec30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`: the time): the fade (+0x160) is the
/// time since its start (+0x15c) over its length (+0x164). Once it passes the value of the
/// `Setting` at `011db520` plus 1.0 (`01012070`; not when NaN), the fade ends (+0x158
/// cleared) and the next one is due at the time plus `rand() %` the integer of the
/// `Setting` at `011db4f8` (+0x168).
pub fn fn_007fec30(e: &mut Engine, this: Ptr, time: u32) {
    let base = this.addr();
    let since = x87_store32(x87_sub(x87_load32(time), x87_m32(e.mem.u32(base + 0x15c))));
    let fade = x87_store32(x87_div(x87_load32(since), x87_m32(e.mem.u32(base + 0x164))));
    e.mem.set_u32(base + 0x160, fade);
    let limit = e.call(SETTING_VALUE, &args![SETTING_011DB520]).u32();
    let one = e.mem.f64(ONE);
    let limit = x87_add(x87_load32(e.mem.u32(limit)), one);
    let fade = x87_load32(e.mem.u32(base + 0x160));
    // FCOMPP; TEST AH,0x41; JNZ: goes on only for fade > limit.
    if fade > limit {
        e.mem.set_u8(base + 0x158, 0);
        let random = e.call(RAND, &args![]).i32();
        let range = e.call(SETTING_INT_0043D4D0, &args![SETTING_011DB4F8]).u32();
        let remainder = random % e.mem.i32(range);
        let next = x87_store32(x87_add(f64::from(remainder), x87_m32(time)));
        e.mem.set_u32(base + 0x168, next);
    }
}

// Translated from 007fecc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`): starts the fade: sets the byte +0x158, takes
/// the start time (+0x15c) from `00825c00` on `011f6394` (an unsigned word as a float) and
/// clears the fade (+0x160).
pub fn fn_007fecc0(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u8(base + 0x158, 1);
    let time = e.call(TIME_00825C00, &args![TIMER_011F6394]).u32();
    // FILD qword of the word with a zero high word.
    e.mem.set_u32(base + 0x15c, x87_store32(f64::from(time)));
    e.mem.set_u32(base + 0x160, 0);
}

// Translated from 007fed10 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `FORenderedMenuRaceSex` destructor body (unnamed in the engine map; `thiscall`):
/// stores the vtable, assigns null to the eight `NiPointer`s of the two arrays at +0xd8
/// and +0xe8 (in pairs) and to the six at +0xf8..+0x10c, destroys those six (last first)
/// and the two arrays (the vector destructor iterator `00ec5fce` with 4, 4, `0045cec0`),
/// then runs the base destructor `007fad10`.
///
/// The exception frame (`FS:[0]`) is not translated.
// Translation incomplete: 007fed10 - the FS:[0] exception frame (unwinding on a throw) is not translated
pub fn fn_007fed10(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base, VTABLE);
    for i in 0..4u32 {
        e.call(POINTER_ASSIGN, &args![base + 0xd8 + 4 * i, 0u32]);
        e.call(POINTER_ASSIGN, &args![base + 0xe8 + 4 * i, 0u32]);
    }
    for offset in [0xf8u32, 0xfc, 0x100, 0x104, 0x108, 0x10c] {
        e.call(POINTER_ASSIGN, &args![base + offset, 0u32]);
    }
    for offset in [0x10cu32, 0x108, 0x104, 0x100, 0xfc, 0xf8] {
        e.call(POINTER_DESTRUCTOR, &args![base + offset]);
    }
    for offset in [0xe8u32, 0xd8] {
        e.call(
            VECTOR_DESTRUCTOR,
            &args![base + offset, 4u32, 4u32, POINTER_DESTRUCTOR],
        );
    }
    e.call(BASE_DESTRUCTOR_007FAD10, &args![this]);
}

// Translated from 007feb40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`thiscall`, `RET 4`: the result's address; a four-float value returned by
/// value): copies the four words at +0xc0 into `out`. Answers `out`.
pub fn fn_007feb40(e: &mut Engine, this: Ptr, out: u32) -> u32 {
    for i in 0..4 {
        let word = e.mem.u32(this.addr() + 0xc0 + 4 * i);
        e.mem.set_u32(out + 4 * i, word);
    }
    out
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x007f_e9c0, fn_007fe9c0(Ptr, u32, u32, u32)),
        entry!(0x007f_eb80, fn_007feb80(Ptr, u32)),
        entry!(0x007f_ec30, fn_007fec30(Ptr, u32)),
        entry!(0x007f_ecc0, fn_007fecc0(Ptr)),
        entry!(0x007f_ed10, fn_007fed10(Ptr)),
        entry!(0x007f_eb40, fn_007feb40(Ptr, u32) -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn called(e: &Engine, target: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == target)
            .map(|(_, w)| w.clone())
            .collect()
    }

    fn engine() -> Engine {
        let mut e = Engine::new();
        for page in [
            0x0101_2000,
            0x0101_6000,
            0x011a_e000,
            0x011d_b000,
            0x011f_6000,
            0x0120_0000,
        ] {
            e.map(page, 0x1000);
        }
        e.mem.set_f32(MINUS_ONE, -1.0);
        e.mem.set_f64(THOUSANDTH, 0.001);
        e.mem.set_f64(ONE, 1.0);
        e.call_log = Some(vec![]);
        e
    }

    #[test]
    fn shader_constants_take_the_defaults() {
        let mut e = engine();
        e.register(MAKE_COLOR_00414430, |e, a| {
            for i in 0..4 {
                e.mem.set_u32(a[0] + 4 * i, a[1 + i as usize]);
            }
            a[0].into_ret()
        });
        e.register(OBJECT_0043C4B0, |_, _| 0x4b0u32.into_ret());
        e.register(IMAGE_SPACE_MANAGER_004E3270, |_, _| 0x3270u32.into_ret());
        e.register(RENDER_EFFECT_00B97550, |_, _| Ret::default());
        let this = e.mem.alloc(0x170);
        e.mem.set_u8(this + 0x158, 1);
        e.call(0x007f_e9c0, &[this, 0x11, 0x22, 0x33]);
        assert_eq!(e.mem.f32(CONSTANT_0120081C), 0.0);
        assert_eq!(e.mem.f32(CONSTANT_011AE604), 0.0);
        assert_eq!(e.mem.f32(CONSTANT_01200820), -1.0);
        assert_eq!(e.mem.f32(CONSTANT_011AE600), 1.0);
        for i in 0..4 {
            assert_eq!(e.mem.f32(CONSTANT_0120082C + 4 * i), 1.0);
        }
        assert_eq!(e.mem.f32(CONSTANT_01200824), 0.0);
        assert_eq!(e.mem.f32(CONSTANT_011AE608), 0.0);
        assert_eq!(e.mem.f32(CONSTANT_01200828), -1.0);
        assert_eq!(
            called(&e, RENDER_EFFECT_00B97550),
            vec![vec![0x3270, 0x1e, 0x4b0, 0x11, 0x22, 0, 1]]
        );
    }

    #[test]
    fn flicker_rerolls_after_the_interval_and_starts_the_fade() {
        let mut e = engine();
        let interval = e.mem.alloc(4);
        e.mem.set_f32(interval, 0.5);
        e.register_double(SETTING_VALUE, move |_, _| interval.into_ret());
        e.register(RAND, |_, _| 12345u32.into_ret());
        e.register(TIME_00825C00, |_, _| 7000u32.into_ret());
        let this = e.mem.alloc(0x170);
        e.mem.set_f32(this + 0x150, 1.0);
        e.mem.set_f32(this + 0x168, 5.0);
        // 2.0 - 1.0 > 0.5: rerolled; 2.0 is not past +0x168.
        e.call(0x007f_eb80, &[this, 2.0f32.to_bits()]);
        assert_eq!(e.mem.f32(this + 0x150), 2.0);
        assert_eq!(e.mem.f32(this + 0x154), (345.0f64 * 0.001) as f32);
        assert_eq!(e.mem.u8(this + 0x158), 0);
        // 2.25 - 2.0 is not above 0.5: kept; 6.0 passes +0x168: the fade starts.
        e.call(0x007f_eb80, &[this, 2.25f32.to_bits()]);
        assert_eq!(e.mem.f32(this + 0x150), 2.0);
        e.call(0x007f_eb80, &[this, 6.0f32.to_bits()]);
        assert_eq!(e.mem.f32(this + 0x150), 6.0);
        assert_eq!(e.mem.u8(this + 0x158), 1);
        assert_eq!(e.mem.f32(this + 0x15c), 7000.0);
        // While fading the menu's slot +0x54 gets the time.
        let vtable = e.mem.alloc(0x58);
        e.mem.set_u32(vtable + 0x54, 0x7054);
        e.register(0x7054, |_, _| Ret::default());
        e.mem.set_u32(this, vtable);
        e.call(0x007f_eb80, &[this, 6.25f32.to_bits()]);
        assert_eq!(called(&e, 0x7054), vec![vec![this, 6.25f32.to_bits()]]);
    }

    #[test]
    fn fade_ends_past_the_limit() {
        let mut e = engine();
        let limit = e.mem.alloc(4);
        e.mem.set_f32(limit, 1.0);
        e.register_double(SETTING_VALUE, move |_, _| limit.into_ret());
        let range = e.mem.alloc(4);
        e.mem.set_i32(range, 100);
        e.register_double(SETTING_INT_0043D4D0, move |_, _| range.into_ret());
        e.register(RAND, |_, _| 250u32.into_ret());
        let this = e.mem.alloc(0x170);
        e.mem.set_u8(this + 0x158, 1);
        e.mem.set_f32(this + 0x15c, 10.0);
        e.mem.set_f32(this + 0x164, 2.0);
        // (13 - 10) / 2 = 1.5, not above 2.0.
        e.call(0x007f_ec30, &[this, 13.0f32.to_bits()]);
        assert_eq!(e.mem.f32(this + 0x160), 1.5);
        assert_eq!(e.mem.u8(this + 0x158), 1);
        // (15 - 10) / 2 = 2.5: the fade ends, the next is 15 + 250 % 100.
        e.call(0x007f_ec30, &[this, 15.0f32.to_bits()]);
        assert_eq!(e.mem.f32(this + 0x160), 2.5);
        assert_eq!(e.mem.u8(this + 0x158), 0);
        assert_eq!(e.mem.f32(this + 0x168), 65.0);
    }

    #[test]
    fn fade_start_takes_the_timer() {
        let mut e = engine();
        e.register(TIME_00825C00, |_, _| 0xffff_ffffu32.into_ret());
        let this = e.mem.alloc(0x170);
        e.mem.set_f32(this + 0x160, 3.0);
        e.call(0x007f_ecc0, &[this]);
        assert_eq!(e.mem.u8(this + 0x158), 1);
        assert_eq!(e.mem.f32(this + 0x15c), 4_294_967_296.0);
        assert_eq!(e.mem.f32(this + 0x160), 0.0);
        assert_eq!(called(&e, TIME_00825C00), vec![vec![TIMER_011F6394]]);
    }

    #[test]
    fn destructor_releases_every_pointer() {
        let mut e = engine();
        for address in [
            POINTER_ASSIGN,
            POINTER_DESTRUCTOR,
            VECTOR_DESTRUCTOR,
            BASE_DESTRUCTOR_007FAD10,
        ] {
            e.register(address, |_, _| Ret::default());
        }
        let this = e.mem.alloc(0x170);
        e.call(0x007f_ed10, &[this]);
        assert_eq!(e.mem.u32(this), VTABLE);
        let assigned: Vec<u32> = called(&e, POINTER_ASSIGN)
            .iter()
            .map(|w| w[0] - this)
            .collect();
        assert_eq!(
            assigned,
            vec![
                0xd8, 0xe8, 0xdc, 0xec, 0xe0, 0xf0, 0xe4, 0xf4, 0xf8, 0xfc, 0x100, 0x104, 0x108,
                0x10c
            ]
        );
        let destroyed: Vec<u32> = called(&e, POINTER_DESTRUCTOR)
            .iter()
            .map(|w| w[0] - this)
            .collect();
        assert_eq!(destroyed, vec![0x10c, 0x108, 0x104, 0x100, 0xfc, 0xf8]);
        assert_eq!(
            called(&e, VECTOR_DESTRUCTOR),
            vec![
                vec![this + 0xe8, 4, 4, POINTER_DESTRUCTOR],
                vec![this + 0xd8, 4, 4, POINTER_DESTRUCTOR]
            ]
        );
        assert_eq!(called(&e, BASE_DESTRUCTOR_007FAD10), vec![vec![this]]);
    }
}

#[cfg(test)]
mod n059_tests {
    use super::*;

    #[test]
    fn copies_the_four_words() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0xd0);
        for i in 0..4 {
            e.mem.set_u32(this + 0xc0 + 4 * i, 0x100 + i);
        }
        let out = e.mem.alloc(0x10);
        assert_eq!(e.call(0x007f_eb40, &args![this, out]).u32(), out);
        assert_eq!(
            [0, 4, 8, 0xc].map(|o| e.mem.u32(out + o)),
            [0x100, 0x101, 0x102, 0x103]
        );
    }
}

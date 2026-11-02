//! `fallout/interface/run_007f7770` (Xbox PDB source unit), subsystem `fallout/interface`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! Most functions here belong to `FOPipboyManager` (vtable `010780ac`, RTTI
//! `.?AVFOPipboyManager@@`; an `FORenderedMenu` with the Pip-Boy's nodes,
//! buttons, knobs and hum sound): its constructor, destructor and reset.
//! The PC code uses the Xbox PDB's offsets of the class.

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 007f7770 (decompiled, FalloutNV.exe 1.4.0.525)
/// A forwarder: it hands `this` (`ECX`) to `007004e0` of unit run_00700480, which is itself
/// a forwarder to `00700500`, and returns nothing. Parameters: one word, `this`; the
/// disassembly of the function ends in a plain `RET` and passes only `ECX` on. `007004e0` is
/// translated in this family as `fn_007004e0(Ptr)` and returns nothing.
pub fn fn_007f7770(e: &mut Engine, this: Ptr) {
    e.call(0x0070_04e0, &args![this]);
}

/// `FOPipboyManager`'s vtable (`010780ac`).
const FO_PIPBOY_MANAGER_VTABLE: u32 = 0x0107_80ac;
/// `FORenderedMenu`'s constructor (`007faa20`), destructor (`007fad10`)
/// and clean-up (`007fadf0`) (unit run_007fa8d0).
const RENDERED_MENU_CONSTRUCTOR: u32 = 0x007f_aa20;
const RENDERED_MENU_DESTRUCTOR: u32 = 0x007f_ad10;
const RENDERED_MENU_CLEAN_UP: u32 = 0x007f_adf0;
/// The `NiPointer` constructor (`00633c90`, thiscall: the pointer),
/// assignment (`0066b0d0`), destructor (`0045cec0`) and the array element
/// constructor (`006694e0`).
const NI_POINTER_CONSTRUCT: u32 = 0x0063_3c90;
const NI_POINTER_ASSIGN: u32 = 0x0066_b0d0;
const NI_POINTER_DESTRUCT: u32 = 0x0045_cec0;
const NI_POINTER_ELEMENT_CONSTRUCT: u32 = 0x0066_94e0;
/// `` `eh vector constructor iterator' `` (`00ec782f`) and `` `eh vector
/// destructor iterator' `` (`00ec5fce`) (stdcall).
const VECTOR_CONSTRUCTOR_ITERATOR: u32 = 0x00ec_782f;
const VECTOR_DESTRUCTOR_ITERATOR: u32 = 0x00ec_5fce;
/// The `BSSoundHandle` constructor (`0041a250`) and destructor
/// (`00483710`).
const SOUND_HANDLE_CONSTRUCTOR: u32 = 0x0041_a250;
const SOUND_HANDLE_DESTRUCTOR: u32 = 0x0048_3710;
/// `Interface::IsInGameLoadingMenuOpen` (`00705ea0`) and
/// `Interface::HideMenus` (`00703610`) (Xbox PDB names, cdecl).
const IS_IN_GAME_LOADING_MENU_OPEN: u32 = 0x0070_5ea0;
const HIDE_MENUS: u32 = 0x0070_3610;
/// `operator delete` (`00401030`, cdecl).
const OPERATOR_DELETE: u32 = 0x0040_1030;
/// The game time in milliseconds (`00825c00`, thiscall on `011f6394`).
const TIME_GLOBALS: u32 = 0x011f_6394;
const GET_MILLISECONDS: u32 = 0x0082_5c00;
/// The step `007f7830` runs with the time (`007f7970`, thiscall: a float).
const TIMED_STEP: u32 = 0x007f_7970;

/// The Pip-Boy's three button, glow and knob pointer arrays.
const BUTTONS: u32 = 0xe8;
const BUTTON_GLOWS: u32 = 0xf4;
const KNOBS: u32 = 0x100;
/// The knobs' positions, targets and rates (three floats each).
const KNOB_POSITIONS: u32 = 0x10c;
const KNOB_TARGETS: u32 = 0x118;
const KNOB_RATES: u32 = 0x124;

// Translated from 007f7790 (decompiled, FalloutNV.exe 1.4.0.525)
/// A destructor (no PDB name): drops the pointer at +0, deletes the object
/// at +0x50 (virtual slot 0, flag 1) when there is one, destroys the
/// member at +4 ([`fn_007f7770`]) and the pointer at +0.
// Translation incomplete: 007f7790 - the compiler's exception frame (FS:[0] chain, cookie 011c16bc) is not translated
pub fn fn_007f7790(e: &mut Engine, this: Ptr) {
    e.call(NI_POINTER_ASSIGN, &args![this, 0u32]);
    let owned = e.mem.u32(this.addr() + 0x50);
    if owned != 0 {
        e.vcall(owned, 0, &args![1u32]);
    }
    fn_007f7770(e, this.byte_add(4));
    e.call(NI_POINTER_DESTRUCT, &args![this]);
}

// Translated from 007f7830 (decompiled, FalloutNV.exe 1.4.0.525)
/// When the menu is initialized (+0x28), runs `007f7970` with the game time
/// in milliseconds as a float.
pub fn fn_007f7830(e: &mut Engine, this: Ptr) {
    let now = e.call(GET_MILLISECONDS, &args![TIME_GLOBALS]).u32();
    let now = x87_store32(f64::from(now));
    if e.mem.u8(this.addr() + 0x28) != 0 {
        e.call(TIMED_STEP, &args![this, x87_quiet_bits(now)]);
    }
}

// Translated from 007f7a70 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `FOPipboyManager` constructor (no PDB name): the `FORenderedMenu`
/// constructor, the vtable, the Pip-Boy nodes (+0xD8 .. +0xE4) and the
/// button, glow and knob arrays (three pointers each) empty, the hum sound
/// handle (+0x154); all of them dropped again, the knob positions, targets
/// and rates 0, the flags +0x144, +0x150, +0x160, +0x161 and +0x162
/// cleared, partial-screen geometry used ([`fn_007f7c90`] with 1) and the
/// third-person screen (+0xE0) dropped. Returns `this`.
// Translation incomplete: 007f7a70 - the compiler's exception frame (FS:[0] chain, cookie 011c16bc) is not translated
pub fn fn_007f7a70(e: &mut Engine, this: Ptr) -> Ptr {
    let t = this.addr();
    e.call(RENDERED_MENU_CONSTRUCTOR, &args![this]);
    e.mem.set_u32(t, FO_PIPBOY_MANAGER_VTABLE);
    for offset in [0xd8u32, 0xdc, 0xe0, 0xe4] {
        e.call(NI_POINTER_CONSTRUCT, &args![t + offset, 0u32]);
    }
    for array in [BUTTONS, BUTTON_GLOWS, KNOBS] {
        e.call(
            VECTOR_CONSTRUCTOR_ITERATOR,
            &args![
                t + array,
                4u32,
                3u32,
                NI_POINTER_ELEMENT_CONSTRUCT,
                NI_POINTER_DESTRUCT
            ],
        );
    }
    e.call(SOUND_HANDLE_CONSTRUCTOR, &args![t + 0x154]);
    for offset in [0xd8u32, 0xdc, 0xe4] {
        e.call(NI_POINTER_ASSIGN, &args![t + offset, 0u32]);
    }
    for i in 0..3u32 {
        for array in [BUTTONS, BUTTON_GLOWS, KNOBS] {
            e.call(NI_POINTER_ASSIGN, &args![t + array + 4 * i, 0u32]);
        }
        for array in [KNOB_POSITIONS, KNOB_TARGETS, KNOB_RATES] {
            e.mem.set_f32(t + array + 4 * i, 0.0);
        }
    }
    for offset in [0x144u32, 0x150, 0x160, 0x161] {
        e.mem.set_u8(t + offset, 0);
    }
    fn_007f7c90(e, this, 1);
    e.mem.set_u8(t + 0x162, 0);
    e.call(NI_POINTER_ASSIGN, &args![t + 0xe0, 0u32]);
    this
}

// Translated from 007f7c90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets `bUsePartialScreenGeometry` (+0x2A, `FORenderedMenu`).
pub fn fn_007f7c90(e: &mut Engine, this: Ptr, value: u8) {
    e.mem.set_u8(this.addr() + 0x2a, value);
}

// Translated from 007f7cb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `FOPipboyManager` scalar deleting destructor (no PDB name): the
/// destructor ([`fn_007f7ce0`]) and `operator delete` when `flags & 1`.
/// Returns `this`.
pub fn fn_007f7cb0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_007f7ce0(e, this);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 007f7ce0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `FOPipboyManager` destructor (no PDB name): hides the menus when the
/// in-game loading menu is open, destroys the hum sound handle, the knob,
/// glow and button arrays and the node pointers, then runs the
/// `FORenderedMenu` destructor.
// Translation incomplete: 007f7ce0 - the compiler's exception frame (FS:[0] chain, cookie 011c16bc) is not translated
pub fn fn_007f7ce0(e: &mut Engine, this: Ptr) {
    let t = this.addr();
    e.mem.set_u32(t, FO_PIPBOY_MANAGER_VTABLE);
    if e.call(IS_IN_GAME_LOADING_MENU_OPEN, &args![]).bool() {
        e.call(HIDE_MENUS, &args![]);
    }
    e.call(SOUND_HANDLE_DESTRUCTOR, &args![t + 0x154]);
    for array in [KNOBS, BUTTON_GLOWS, BUTTONS] {
        e.call(
            VECTOR_DESTRUCTOR_ITERATOR,
            &args![t + array, 4u32, 3u32, NI_POINTER_DESTRUCT],
        );
    }
    for offset in [0xe4u32, 0xe0, 0xdc, 0xd8] {
        e.call(NI_POINTER_DESTRUCT, &args![t + offset]);
    }
    e.call(RENDERED_MENU_DESTRUCTOR, &args![this]);
}

// Translated from 007f7e00 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `FOPipboyManager` reset (no PDB name): drops the Pip-Boy nodes, the
/// buttons, glows and knobs, zeroes the knob positions, targets and rates,
/// drops the third-person screen and runs the `FORenderedMenu` clean-up.
pub fn fn_007f7e00(e: &mut Engine, this: Ptr) {
    let t = this.addr();
    for offset in [0xd8u32, 0xdc, 0xe4] {
        e.call(NI_POINTER_ASSIGN, &args![t + offset, 0u32]);
    }
    for i in 0..3u32 {
        for array in [BUTTONS, BUTTON_GLOWS, KNOBS] {
            e.call(NI_POINTER_ASSIGN, &args![t + array + 4 * i, 0u32]);
        }
        for array in [KNOB_POSITIONS, KNOB_TARGETS, KNOB_RATES] {
            e.mem.set_f32(t + array + 4 * i, 0.0);
        }
    }
    e.call(NI_POINTER_ASSIGN, &args![t + 0xe0, 0u32]);
    e.call(RENDERED_MENU_CLEAN_UP, &args![this]);
}

/// Two float statics (`012007d0`, `012007d4`) and the byte static marking them set
/// (`011ae5d4`), the float pair `011f4980` copied as two words, the wave function
/// `004e44d0` (cdecl, a `float`, `ST0`).
const OFFSET_X_012007D0: u32 = 0x0120_07d0;
const OFFSET_Y_012007D4: u32 = 0x0120_07d4;
const OFFSET_SET_011AE5D4: u32 = 0x011a_e5d4;
const PAIR_011F4980: u32 = 0x011f_4980;
const WAVE_004E44D0: u32 = 0x004e_44d0;
/// `FORenderedMenu::OnIdle` (`007faf10`), `007f8320` (`RET 4`, a `float`) and
/// `007fa220` (`fopipboymanager.cpp`).
const RENDERED_MENU_ON_IDLE: u32 = 0x007f_af10;
const PIPBOY_007F8320: u32 = 0x007f_8320;
const PIPBOY_007FA220: u32 = 0x007f_a220;

/// The game time in milliseconds as a `float` (`FILD qword` of the zero-extended word,
/// `FSTP dword`).
fn milliseconds_as_float(e: &mut Engine) -> u32 {
    let now = e.call(GET_MILLISECONDS, &args![TIME_GLOBALS]).u32();
    x87_store32(f64::from(now))
}

// Translated from 007f7880 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`thiscall`, `RET 0xc`: three `float`s; starts the timed wave of `007f7970`):
/// marks it running (+0x28), both durations (+0x30, +0x34) the first float, the first
/// amplitude (+0x38) the second, the first frequency (+0x40) the third; the start
/// (+0x2c) the game time in milliseconds, the first output (+0x48) and the second
/// amplitude (+0x3c) 0.0.
pub fn fn_007f7880(e: &mut Engine, this: Ptr, duration: f32, amplitude: f32, frequency: f32) {
    let base = this.addr();
    e.mem.set_u8(base + 0x28, 1);
    let duration = x87_quiet_bits(duration.to_bits());
    e.mem.set_u32(base + 0x30, duration);
    e.mem.set_u32(base + 0x34, duration);
    e.mem
        .set_u32(base + 0x38, x87_quiet_bits(amplitude.to_bits()));
    e.mem
        .set_u32(base + 0x40, x87_quiet_bits(frequency.to_bits()));
    let start = milliseconds_as_float(e);
    e.mem.set_u32(base + 0x2c, start);
    e.mem.set_u32(base + 0x48, x87_store32(0.0));
    e.mem.set_u32(base + 0x3c, x87_store32(0.0));
}

// Translated from 007f78f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`thiscall`, `RET 0xc`: three pointers to float pairs): marks the wave
/// running (+0x28); the durations (+0x30), amplitudes (+0x38) and frequencies (+0x40)
/// copied from the pairs (as words), the start (+0x2c) the game time in milliseconds,
/// and the outputs (+0x48) the pair at `011f4980`.
pub fn fn_007f78f0(e: &mut Engine, this: Ptr, durations: u32, amplitudes: u32, frequencies: u32) {
    let base = this.addr();
    e.mem.set_u8(base + 0x28, 1);
    for (source, offset) in [
        (durations, 0x30u32),
        (amplitudes, 0x38),
        (frequencies, 0x40),
    ] {
        let first = e.mem.u32(source);
        let second = e.mem.u32(source + 4);
        e.mem.set_u32(base + offset, first);
        e.mem.set_u32(base + offset + 4, second);
    }
    let start = milliseconds_as_float(e);
    e.mem.set_u32(base + 0x2c, start);
    let first = e.global::<u32>(PAIR_011F4980);
    e.mem.set_u32(base + 0x48, first);
    let second = e.global::<u32>(PAIR_011F4980 + 4);
    e.mem.set_u32(base + 0x4c, second);
}

// Translated from 007f7970 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`thiscall`, `RET 4`: the time in milliseconds as a `float`): the elapsed
/// time since the start (+0x2c). Past both durations (+0x30, +0x34) the wave stops
/// (+0x28 cleared, `007f7a50(0, 0)`). Otherwise each output (+0x48, +0x4c) is the wave
/// `004e44d0(elapsed * frequency)` times the amplitude times what is left of the
/// duration (`1 - elapsed / duration`).
pub fn fn_007f7970(e: &mut Engine, this: Ptr, now: f32) {
    let base = this.addr();
    let elapsed = x87_store32(x87_sub(
        x87_load32(now.to_bits()),
        x87_m32(e.mem.u32(base + 0x2c)),
    ));
    // FCOMPP of each duration against the elapsed time; TEST AH,5: done only when
    // both are below it.
    let done = x87_load32(e.mem.u32(base + 0x30)) < x87_load32(elapsed)
        && x87_load32(e.mem.u32(base + 0x34)) < x87_load32(elapsed);
    if done {
        e.mem.set_u8(base + 0x28, 0);
        let zero = x87_store32(0.0);
        fn_007f7a50(e, f32::from_bits(zero), f32::from_bits(zero));
        return;
    }
    // The fractions left at `EBP - 0xc` and `EBP - 8`.
    let left = |e: &Engine, duration: u32| {
        let quotient = x87_div(x87_load32(elapsed), x87_m32(e.mem.u32(base + duration)));
        x87_store32(x87_sub(1.0, quotient))
    };
    let first_left = left(e, 0x30);
    let second_left = left(e, 0x34);
    for (frequency, amplitude, fraction, output) in [
        (0x40u32, 0x38u32, first_left, 0x48u32),
        (0x44, 0x3c, second_left, 0x4c),
    ] {
        let phase = x87_store32(x87_mul(
            x87_load32(elapsed),
            x87_m32(e.mem.u32(base + frequency)),
        ));
        let wave = e.call(WAVE_004E44D0, &args![x87_quiet_bits(phase)]).f64();
        let value = x87_mul(
            x87_mul(wave, x87_m32(e.mem.u32(base + amplitude))),
            x87_m32(fraction),
        );
        e.mem.set_u32(base + output, x87_store32(value));
    }
}

// Translated from 007f7a50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (cdecl, two `float`s): stores them in `012007d0` and `012007d4` and sets the
/// byte `011ae5d4`.
pub fn fn_007f7a50(e: &mut Engine, x: f32, y: f32) {
    e.set_global(OFFSET_X_012007D0, x87_quiet_bits(x.to_bits()));
    e.set_global(OFFSET_Y_012007D4, x87_quiet_bits(y.to_bits()));
    e.set_global(OFFSET_SET_011AE5D4, 1u8);
}

// Translated from 007f7ee0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`thiscall`; an idle step): false when `FORenderedMenu::OnIdle` answers
/// false; otherwise `007f8320` with the game time in milliseconds as a `float`, then
/// `007fa220`, and true.
pub fn fn_007f7ee0(e: &mut Engine, this: Ptr) -> bool {
    if !e.call(RENDERED_MENU_ON_IDLE, &args![this]).bool() {
        return false;
    }
    let now = milliseconds_as_float(e);
    e.call(PIPBOY_007F8320, &args![this, x87_quiet_bits(now)]);
    e.call(PIPBOY_007FA220, &args![this]);
    true
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x007f_7770, fn_007f7770(Ptr)),
        entry!(0x007f_7790, fn_007f7790(Ptr)),
        entry!(0x007f_7830, fn_007f7830(Ptr)),
        entry!(0x007f_7a70, fn_007f7a70(Ptr) -> Ptr),
        entry!(0x007f_7c90, fn_007f7c90(Ptr, u8)),
        entry!(0x007f_7cb0, fn_007f7cb0(Ptr, u32) -> Ptr),
        entry!(0x007f_7ce0, fn_007f7ce0(Ptr)),
        entry!(0x007f_7e00, fn_007f7e00(Ptr)),
        entry!(0x007f_7880, fn_007f7880(Ptr, f32, f32, f32)),
        entry!(0x007f_78f0, fn_007f78f0(Ptr, u32, u32, u32)),
        entry!(0x007f_7970, fn_007f7970(Ptr, f32)),
        entry!(0x007f_7a50, fn_007f7a50(f32, f32)),
        entry!(0x007f_7ee0, fn_007f7ee0(Ptr) -> bool),
    ]
}

#[cfg(test)]
mod thunk_tests {
    use super::*;

    fn calls_to(e: &Engine, callee: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == callee)
            .map(|(_, words)| words.clone())
            .collect()
    }

    #[test]
    fn fwd_a03_007f7770_passes_this_to_the_callee() {
        let mut e = Engine::new();
        e.register(0x0070_04e0, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x007f_7770, &args![0x0000_5000u32]);
        assert_eq!(calls_to(&e, 0x0070_04e0), vec![vec![0x5000]]);
    }
}

#[cfg(test)]
mod pipboy_tests {
    use super::*;

    fn calls_to(e: &Engine, callee: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == callee)
            .map(|(_, words)| words.clone())
            .collect()
    }

    fn quiet(e: &mut Engine, addresses: &[u32]) {
        for &address in addresses {
            e.register(address, |_, _| Ret::default());
        }
    }

    fn offsets(e: &Engine, callee: u32, base: u32) -> Vec<u32> {
        calls_to(e, callee).iter().map(|w| w[0] - base).collect()
    }

    #[test]
    fn fn_007f7790_releases_the_members() {
        let mut e = Engine::new();
        quiet(
            &mut e,
            &[
                NI_POINTER_ASSIGN,
                NI_POINTER_DESTRUCT,
                0x0070_04e0,
                0x0000_1000,
            ],
        );
        e.put_vtable(0x0003_0000, &[0x0000_1000]);
        let owned = e.mem.alloc(8);
        e.mem.set_u32(owned, 0x0003_0000);
        let this = e.mem.alloc(0x60);
        e.mem.set_u32(this + 0x50, owned);
        e.call_log = Some(vec![]);
        e.call(0x007f_7790, &args![this]);
        assert_eq!(calls_to(&e, NI_POINTER_ASSIGN), vec![vec![this, 0]]);
        assert_eq!(calls_to(&e, 0x0000_1000), vec![vec![owned, 1]]);
        assert_eq!(calls_to(&e, 0x0070_04e0), vec![vec![this + 4]]);
        assert_eq!(calls_to(&e, NI_POINTER_DESTRUCT), vec![vec![this]]);
        e.mem.set_u32(this + 0x50, 0);
        e.call_log = Some(vec![]);
        e.call(0x007f_7790, &args![this]);
        assert!(calls_to(&e, 0x0000_1000).is_empty());
    }

    #[test]
    fn fn_007f7830_steps_an_initialized_menu() {
        let mut e = Engine::new();
        quiet(&mut e, &[TIMED_STEP]);
        e.register(GET_MILLISECONDS, |_, _| Ret {
            eax: 1234,
            ..Ret::default()
        });
        let this = e.mem.alloc(0x30);
        e.call_log = Some(vec![]);
        e.call(0x007f_7830, &args![this]);
        assert!(calls_to(&e, TIMED_STEP).is_empty());
        e.mem.set_u8(this + 0x28, 1);
        e.call(0x007f_7830, &args![this]);
        assert_eq!(
            calls_to(&e, TIMED_STEP),
            vec![vec![this, 1234.0f32.to_bits()]]
        );
    }

    #[test]
    fn fn_007f7a70_constructs_the_pipboy_manager() {
        let mut e = Engine::new();
        quiet(
            &mut e,
            &[
                RENDERED_MENU_CONSTRUCTOR,
                NI_POINTER_CONSTRUCT,
                NI_POINTER_ASSIGN,
                VECTOR_CONSTRUCTOR_ITERATOR,
                SOUND_HANDLE_CONSTRUCTOR,
            ],
        );
        let this = e.mem.alloc(0x170);
        e.mem.set_f32(this + KNOB_RATES + 8, 3.0);
        e.mem.set_u8(this + 0x161, 1);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x007f_7a70, &args![this]).u32(), this);
        assert_eq!(calls_to(&e, RENDERED_MENU_CONSTRUCTOR), vec![vec![this]]);
        assert_eq!(e.mem.u32(this), FO_PIPBOY_MANAGER_VTABLE);
        assert_eq!(
            offsets(&e, NI_POINTER_CONSTRUCT, this),
            vec![0xd8, 0xdc, 0xe0, 0xe4]
        );
        assert_eq!(
            calls_to(&e, VECTOR_CONSTRUCTOR_ITERATOR)[0],
            vec![
                this + 0xe8,
                4,
                3,
                NI_POINTER_ELEMENT_CONSTRUCT,
                NI_POINTER_DESTRUCT
            ]
        );
        let dropped = offsets(&e, NI_POINTER_ASSIGN, this);
        assert_eq!(dropped[..6], [0xd8, 0xdc, 0xe4, 0xe8, 0xf4, 0x100]);
        assert_eq!(dropped.len(), 13);
        assert_eq!(dropped[12], 0xe0);
        assert_eq!(e.mem.f32(this + KNOB_RATES + 8), 0.0);
        assert_eq!(e.mem.u8(this + 0x161), 0);
        assert_eq!(e.mem.u8(this + 0x2a), 1);
    }

    #[test]
    fn fn_007f7cb0_destroys_and_frees() {
        let mut e = Engine::new();
        quiet(
            &mut e,
            &[
                IS_IN_GAME_LOADING_MENU_OPEN,
                HIDE_MENUS,
                SOUND_HANDLE_DESTRUCTOR,
                VECTOR_DESTRUCTOR_ITERATOR,
                NI_POINTER_DESTRUCT,
                RENDERED_MENU_DESTRUCTOR,
                OPERATOR_DELETE,
            ],
        );
        let this = e.mem.alloc(0x170);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x007f_7cb0, &args![this, 1u32]).u32(), this);
        assert!(calls_to(&e, HIDE_MENUS).is_empty());
        assert_eq!(
            calls_to(&e, SOUND_HANDLE_DESTRUCTOR),
            vec![vec![this + 0x154]]
        );
        assert_eq!(
            offsets(&e, VECTOR_DESTRUCTOR_ITERATOR, this),
            vec![0x100, 0xf4, 0xe8]
        );
        assert_eq!(
            offsets(&e, NI_POINTER_DESTRUCT, this),
            vec![0xe4, 0xe0, 0xdc, 0xd8]
        );
        assert_eq!(calls_to(&e, RENDERED_MENU_DESTRUCTOR), vec![vec![this]]);
        assert_eq!(calls_to(&e, OPERATOR_DELETE), vec![vec![this]]);
        e.register(IS_IN_GAME_LOADING_MENU_OPEN, |_, _| Ret {
            eax: 1,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x007f_7cb0, &args![this, 0u32]);
        assert_eq!(calls_to(&e, HIDE_MENUS).len(), 1);
        assert!(calls_to(&e, OPERATOR_DELETE).is_empty());
    }

    #[test]
    fn fn_007f7e00_resets_the_pipboy_members() {
        let mut e = Engine::new();
        quiet(&mut e, &[NI_POINTER_ASSIGN, RENDERED_MENU_CLEAN_UP]);
        let this = e.mem.alloc(0x170);
        e.mem.set_f32(this + KNOB_POSITIONS, 1.0);
        e.call_log = Some(vec![]);
        e.call(0x007f_7e00, &args![this]);
        let dropped = offsets(&e, NI_POINTER_ASSIGN, this);
        assert_eq!(dropped.len(), 13);
        assert_eq!(dropped[3..6], [0xe8, 0xf4, 0x100]);
        assert_eq!(dropped[12], 0xe0);
        assert_eq!(e.mem.f32(this + KNOB_POSITIONS), 0.0);
        assert_eq!(calls_to(&e, RENDERED_MENU_CLEAN_UP), vec![vec![this]]);
    }
}

#[cfg(test)]
mod n059_tests {
    use super::*;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x011f_4000, 0x1000);
        e.map(0x0120_0000, 0x1000);
        e.map(0x011a_e000, 0x1000);
        e.register(GET_MILLISECONDS, |_, _| 1500u32.into_ret());
        e
    }

    #[test]
    fn starts_the_wave() {
        let mut e = engine();
        let this = e.mem.alloc(0x50);
        e.mem.set_f32(this + 0x48, 9.0);
        e.call(
            0x007f_7880,
            &args![this, 2.0f32.to_bits(), 3.0f32.to_bits(), 4.0f32.to_bits()],
        );
        assert_eq!(e.mem.u8(this + 0x28), 1);
        assert_eq!(
            [0x2c, 0x30, 0x34, 0x38, 0x3c, 0x40, 0x48].map(|o| e.mem.f32(this + o)),
            [1500.0, 2.0, 2.0, 3.0, 0.0, 4.0, 0.0]
        );
        let pairs = e.mem.alloc(0x18);
        for i in 0..6 {
            e.mem.set_f32(pairs + 4 * i, i as f32);
        }
        e.set_global(PAIR_011F4980, 7.0f32);
        e.set_global(PAIR_011F4980 + 4, 8.0f32);
        e.call(0x007f_78f0, &args![this, pairs, pairs + 8, pairs + 0x10]);
        assert_eq!(
            [0x30, 0x34, 0x38, 0x3c, 0x40, 0x44, 0x48, 0x4c].map(|o| e.mem.f32(this + o)),
            [0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 7.0, 8.0]
        );
    }

    #[test]
    fn steps_and_stops_the_wave() {
        let mut e = engine();
        e.register(WAVE_004E44D0, |_, a| Ret {
            st0: f64::from(f32::from_bits(a[0])),
            ..Ret::default()
        });
        let this = e.mem.alloc(0x50);
        for (offset, value) in [
            (0x2c, 100.0f32),
            (0x30, 400.0),
            (0x34, 200.0),
            (0x38, 2.0),
            (0x3c, 3.0),
            (0x40, 0.5),
            (0x44, 0.25),
        ] {
            e.mem.set_f32(this + offset, value);
        }
        e.mem.set_u8(this + 0x28, 1);
        // 100 elapsed: the first output 50 * 2 * 0.75, the second 25 * 3 * 0.5.
        e.call(0x007f_7970, &args![this, 200.0f32.to_bits()]);
        assert_eq!(e.mem.f32(this + 0x48), 75.0);
        assert_eq!(e.mem.f32(this + 0x4c), 37.5);
        // 300 elapsed: past only the second duration, so still running.
        e.call(0x007f_7970, &args![this, 400.0f32.to_bits()]);
        assert_eq!(e.mem.u8(this + 0x28), 1);
        e.call(0x007f_7970, &args![this, 600.0f32.to_bits()]);
        assert_eq!(e.mem.u8(this + 0x28), 0);
        assert_eq!(e.global::<u8>(OFFSET_SET_011AE5D4), 1);
        assert_eq!(e.global::<f32>(OFFSET_X_012007D0), 0.0);
    }

    #[test]
    fn idle_step_runs_after_the_menu() {
        let mut e = engine();
        e.register(RENDERED_MENU_ON_IDLE, |_, a| {
            u32::from(a[0] == 0x5000).into_ret()
        });
        e.register(PIPBOY_007F8320, |_, _| Ret::default());
        e.register(PIPBOY_007FA220, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        assert!(!e.call(0x007f_7ee0, &args![0x6000u32]).bool());
        assert!(e.call(0x007f_7ee0, &args![0x5000u32]).bool());
        let log = e.call_log.unwrap();
        assert!(log.contains(&(PIPBOY_007F8320, vec![0x5000, 1500.0f32.to_bits()])));
        assert!(log.contains(&(PIPBOY_007FA220, vec![0x5000])));
        assert!(!log.contains(&(PIPBOY_007FA220, vec![0x6000])));
    }
}

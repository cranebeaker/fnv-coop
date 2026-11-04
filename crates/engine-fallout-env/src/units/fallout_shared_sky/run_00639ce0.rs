//! `fallout shared/sky/run_00639ce0` (Xbox PDB source unit), subsystem `fallout shared/sky`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The `Sky` constructor (it stores `Sky`'s vtable `0104edec`) and the setter of flag 0x20
//! of `Sky::uiFlags`, which the linker placed before `sky.cpp`. The `Sky` fields are written
//! at the offsets `sky.rs` declares for its layout, with their Xbox PDB names.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::fallout_shared_sky::skyobject::{pointer_assign, NI_POINTER_FROM_RAW};

/// `Sky::vftable` (`0104edec`).
const SKY_VTABLE: u32 = 0x0104_edec;
/// `vector constructor iterator` (`00401050`, `stdcall`: array, size, count, constructor)
/// with the `NiColor` element constructor of `clouds.cpp` (`00633e00`).
const VECTOR_CONSTRUCT: u32 = 0x0040_1050;
const COLOR_ELEMENT_CONSTRUCT: u32 = 0x0063_3e00;
/// `NiColor::NiColor(r, g, b)` (`00416870`, `thiscall`, three `float`s).
const COLOR_CONSTRUCT: u32 = 0x0041_6870;
/// The `float` constants the constructor loads: `01017b78` (`fCurrentGameHour`),
/// `01018f7c` (`fHighNoon`) and `0104ede4` (the `float` at +0x130).
const INITIAL_GAME_HOUR: u32 = 0x0101_7b78;
const INITIAL_HIGH_NOON: u32 = 0x0101_8f7c;
const INITIAL_FLOAT_130: u32 = 0x0104_ede4;
/// The `NiColor` every sky colour starts as (`011a9b7c`, three words).
const INITIAL_COLOR: u32 = 0x011a_9b7c;
/// `operator new` (`00401000`, `cdecl`) and the sound list's constructor (`0096a2d0`,
/// `thiscall`; 8 bytes).
const OPERATOR_NEW: u32 = 0x0040_1000;
const LIST_CONSTRUCT: u32 = 0x0096_a2d0;
/// The setting (`011ccca8`) whose `float` (`00403e20`, a pointer to the value) is the
/// initial `+0x10c`.
const SETTING_011CCCA8: u32 = 0x011c_cca8;
const SETTING_FLOAT_VALUE: u32 = 0x0040_3e20;

// Translated from 00639d40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Sky::Sky` (by the vtable it stores; unnamed in the engine map; `thiscall`, no stack
/// words): the vtable; `spRoot` and `spMoonsRoot` constructed from 0; the ten `SkyColor`s
/// constructed (`00633e00`), `WaterFogColor` and `SunSpecularColor` built as (0, 0, 0); no
/// components (`+0x20` to `+0x38`); both roots assigned 0; `fCurrentGameHour` from
/// `01017b78`; mode 4; no climate or weathers; `fLastWeatherUpdate` 0; no image-space
/// instances; every sky colour and `SunSpecularColor` from `011a9b7c`; `fWindSpeed` 0, the
/// `float` at +0xd0 1, `fCurrentWeatherPct` 1, `+0x110` 0; a new sound list (8 bytes,
/// `0096a2d0`; 0 when the allocation fails) as `pSkySoundList`; `fFlash` 0, no moon phase
/// update; `+0x10c` from the setting at `011ccca8`; no flags, then flag 0x20 set
/// (`0063a030`); `fHighNoon` from `01018f7c`, `+0x130` from `0104ede4`, `+0x134` 0. Answers
/// `this`.
// Translation incomplete: 00639d40 - the C++ exception frame (FS:[0] chain, unwind states 0 to 2 for the roots and the list allocation) is not translated.
pub fn fn_00639d40(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    let float_from = |e: &Engine, address: u32| x87_quiet_bits(e.mem.u32(address));
    e.mem.set_u32(base, SKY_VTABLE);
    e.call(NI_POINTER_FROM_RAW, &args![base + 4, 0u32]);
    e.call(NI_POINTER_FROM_RAW, &args![base + 8, 0u32]);
    e.call(
        VECTOR_CONSTRUCT,
        &args![base + 0x3c, 0xcu32, 0xau32, COLOR_ELEMENT_CONSTRUCT],
    );
    let zero = 0.0f32.to_bits();
    for offset in [0xb4u32, 0xc0] {
        e.call(COLOR_CONSTRUCT, &args![base + offset, zero, zero, zero]);
    }
    for offset in [0x20u32, 0x24, 0x2c, 0x28, 0x30, 0x34, 0x38] {
        e.mem.set_u32(base + offset, 0);
    }
    pointer_assign(e, base + 4, 0);
    pointer_assign(e, base + 8, 0);
    let hour = float_from(e, INITIAL_GAME_HOUR);
    e.mem.set_u32(base + 0xec, hour);
    e.mem.set_u32(base + 0xf8, 4);
    for offset in [0xcu32, 0x1c, 0x18, 0x14, 0x10] {
        e.mem.set_u32(base + offset, 0);
    }
    e.mem.set_f32(base + 0xf0, 0.0);
    for offset in [0x11cu32, 0x124, 0x120, 0x128] {
        e.mem.set_u32(base + offset, 0);
    }
    for color in 0..10u32 {
        for word in 0..3 {
            let value = e.mem.u32(INITIAL_COLOR + 4 * word);
            e.mem.set_u32(base + 0x3c + 0xc * color + 4 * word, value);
        }
    }
    for word in 0..3 {
        let value = e.mem.u32(INITIAL_COLOR + 4 * word);
        e.mem.set_u32(base + 0xc0 + 4 * word, value);
    }
    e.mem.set_f32(base + 0xcc, 0.0);
    e.mem.set_f32(base + 0xd0, 1.0);
    e.mem.set_f32(base + 0xf4, 1.0);
    e.mem.set_f32(base + 0x110, 0.0);
    let block = e.call(OPERATOR_NEW, &args![8u32]).u32();
    let list = if block != 0 {
        e.call(LIST_CONSTRUCT, &args![block]).u32()
    } else {
        0
    };
    e.mem.set_u32(base + 0xfc, list);
    e.mem.set_f32(base + 0x100, 0.0);
    e.mem.set_u32(base + 0x108, 0);
    let setting = e.call(SETTING_FLOAT_VALUE, &args![SETTING_011CCCA8]).u32();
    let value = float_from(e, setting);
    e.mem.set_u32(base + 0x10c, value);
    e.mem.set_u32(base + 0x118, 0);
    fn_0063a030(e, base, true);
    let noon = float_from(e, INITIAL_HIGH_NOON);
    e.mem.set_u32(base + 0x12c, noon);
    let value = float_from(e, INITIAL_FLOAT_130);
    e.mem.set_u32(base + 0x130, value);
    e.mem.set_f32(base + 0x134, 0.0);
    this
}

// Translated from 0063a030 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall` on the `Sky`, one byte): sets flag 0x20 of
/// `uiFlags` (`+0x118`) when the byte is not 0, else clears it.
pub fn fn_0063a030(e: &mut Engine, sky: u32, on: bool) {
    let flags = e.mem.u32(sky + 0x118);
    let flags = if on {
        flags | 0x20
    } else {
        flags & 0xffff_ffdf
    };
    e.mem.set_u32(sky + 0x118, flags);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0063_9d40, fn_00639d40(Ptr) -> Ptr),
        entry!(0x0063_a030, fn_0063a030(u32, bool)),
    ]
    .into_iter()
    .chain(funcs_n086())
    .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::fallout_shared_sky::skyobject::test_support::*;

    #[test]
    fn constructor_sets_the_sky_defaults() {
        let mut e = Engine::new();
        plain_pointers(&mut e);
        quiet(&mut e, &[VECTOR_CONSTRUCT, COLOR_CONSTRUCT]);
        for (address, value) in [
            (INITIAL_GAME_HOUR, 10.0f32),
            (INITIAL_HIGH_NOON, 13.0),
            (INITIAL_FLOAT_130, 0.5),
        ] {
            e.map(address, 4);
            e.set_global(address, value);
        }
        e.map(INITIAL_COLOR, 12);
        for word in 0..3u32 {
            e.mem.set_u32(INITIAL_COLOR + 4 * word, 0x10 + word);
        }
        let setting = e.mem.alloc(8);
        e.mem.set_f32(setting + 4, 7.0);
        e.register_double(SETTING_FLOAT_VALUE, move |_, _| answer(setting + 4));
        e.register(OPERATOR_NEW, |_, _| answer(0x0000_0800));
        e.register(LIST_CONSTRUCT, |_, a| answer(a[0]));
        let this = e.mem.alloc(0x138);
        for offset in (0..0x138).step_by(4) {
            e.mem.set_u32(this + offset, 0xdead_beef);
        }
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0063_9d40, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), SKY_VTABLE);
        assert_eq!(
            calls(&e, VECTOR_CONSTRUCT),
            vec![vec![this + 0x3c, 0xc, 0xa, COLOR_ELEMENT_CONSTRUCT]]
        );
        assert_eq!(
            calls(&e, COLOR_CONSTRUCT),
            vec![vec![this + 0xb4, 0, 0, 0], vec![this + 0xc0, 0, 0, 0]]
        );
        assert_eq!(e.mem.u32(this + 4), 0);
        assert_eq!(e.mem.u32(this + 0x20), 0);
        assert_eq!(e.mem.f32(this + 0xec), 10.0);
        assert_eq!(e.mem.u32(this + 0xf8), 4);
        assert_eq!(e.mem.u32(this + 0x3c + 0xc * 9 + 8), 0x12);
        assert_eq!(e.mem.u32(this + 0xc0), 0x10);
        // WaterFogColor is left as its constructor made it.
        assert_eq!(e.mem.u32(this + 0xb4), 0xdead_beef);
        assert_eq!(e.mem.f32(this + 0xd0), 1.0);
        assert_eq!(e.mem.f32(this + 0xf4), 1.0);
        assert_eq!(e.mem.u32(this + 0xfc), 0x0800);
        assert_eq!(e.mem.f32(this + 0x10c), 7.0);
        assert_eq!(e.mem.u32(this + 0x118), 0x20);
        assert_eq!(e.mem.f32(this + 0x12c), 13.0);
        assert_eq!(e.mem.f32(this + 0x130), 0.5);
        assert_eq!(e.mem.f32(this + 0x134), 0.0);
        // A failed list allocation leaves no list.
        e.register(OPERATOR_NEW, |_, _| answer(0));
        e.call(0x0063_9d40, &args![this]);
        assert_eq!(e.mem.u32(this + 0xfc), 0);
    }

    #[test]
    fn flag_0x20_setter() {
        let mut e = Engine::new();
        let sky = e.mem.alloc(0x138);
        e.mem.set_u32(sky + 0x118, 3);
        e.call(0x0063_a030, &args![sky, 1u32]);
        assert_eq!(e.mem.u32(sky + 0x118), 0x23);
        e.call(0x0063_a030, &args![sky, 0x100u32]);
        assert_eq!(e.mem.u32(sky + 0x118), 3);
    }
}

// Translated from 00639ce0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, plain `RET`; the class is
/// unconfirmed): the low 31 bits of the word at `+0x18`.
pub fn fn_00639ce0(e: &mut Engine, this: Ptr) -> u32 {
    e.mem.u32(this.addr().wrapping_add(0x18)) & 0x7fff_ffff
}

// Translated from 00639d00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`): keeps the top bit of
/// the word at `+0x18` and puts the low 31 bits of `value` under it (the
/// word written twice, as the exe does).
pub fn fn_00639d00(e: &mut Engine, this: Ptr, value: u32) {
    let slot = this.addr().wrapping_add(0x18);
    let top = e.mem.u32(slot) & 0x8000_0000;
    e.mem.set_u32(slot, top);
    let word = (value & 0x7fff_ffff) | e.mem.u32(slot);
    e.mem.set_u32(slot, word);
}

/// This unit's n086 translations, by exe address.
fn funcs_n086() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0063_9ce0, fn_00639ce0(Ptr) -> u32),
        entry!(0x0063_9d00, fn_00639d00(Ptr, u32)),
    ]
}

#[cfg(test)]
mod n086_tests {
    use super::*;

    #[test]
    fn the_low_31_bits_of_word_0x18() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x1c);
        e.mem.set_u32(this + 0x18, 0x8000_0005);
        assert_eq!(e.call(0x0063_9ce0, &args![this]).u32(), 5);
        e.call(0x0063_9d00, &args![this, 0xffff_fff0u32]);
        assert_eq!(e.mem.u32(this + 0x18), 0xffff_fff0);
        e.mem.set_u32(this + 0x18, 0x7fff_ffff);
        e.call(0x0063_9d00, &args![this, 0x8000_0003u32]);
        assert_eq!(e.mem.u32(this + 0x18), 3);
    }
}

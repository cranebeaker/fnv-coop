//! `fallout shared/run_006151d0` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 006153c0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006153c0 - the exception-unwinding frame (FS:[0]) and the security-cookie bookkeeping are not translated.
/// Constructor of the form `CreateFormOfType` (`00465110`) makes for form type 0x68 (0x50 bytes); `ECX` is the object.
/// The engine map gives it no name; the RTTI name of the vtable it stores at `+0` (`0x0104_ba24`) is `TESReputation`, so this is the constructor of `TESReputation`.
/// It runs the base and component constructors by address, stores the vtable pointers and sets the fields to their start values.
/// The exception-unwinding frame is not translated.
pub fn fn_006153c0(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(0x0048_3370, &args![this]);
    e.call(0x0040_2d00, &args![base + 0x18]);
    e.call(0x0040_2c20, &args![base + 0x24]);
    e.call(0x0047_a130, &args![base + 0x30]);
    e.mem.set_u32(base, 0x0104_ba24);
    e.mem.set_u32(base + 0x18, 0x0104_ba0c);
    e.mem.set_u32(base + 0x24, 0x0104_b9ec);
    e.mem.set_u32(base + 0x30, 0x0104_b9d8);
    e.call(0x0061_5560, &args![this]);
    e.call(0x004f_15a0, &args![this, 0x68u32]);
    this
}

/// The race's body texture for a sex and index (`00613a20`, tesrace.cpp,
/// `__thiscall`, `RET 8`) and whether a texture has a path (`0048cee0`).
const BODY_TEXTURE: u32 = 0x0061_3a20;
const HAS_PATH: u32 = 0x0048_cee0;

// Translated from 00615560 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the two floats at `this + 0x44` and `this + 0x48` to 0.0
/// (`FLDZ`/`FSTP`; `ECX`, no stack words). The constructor `fn_006153c0`
/// runs it. The engine map gives it no name.
pub fn fn_00615560(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr() + 0x44, x87_store32(0.0));
    e.mem.set_u32(this.addr() + 0x48, x87_store32(0.0));
}

// Translated from 006155c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The float at `this + 0x44` when `which` is 1, else the one at
/// `this + 0x48`, returned in ST0 (`FLD`, which quiets a signalling NaN).
/// `__thiscall`, `RET 4`. The engine map gives it no name.
pub fn fn_006155c0(e: &mut Engine, this: Ptr, which: i32) -> f64 {
    let at = if which == 1 { 0x44 } else { 0x48 };
    x87_load32(e.mem.u32(this.addr() + at))
}

// Translated from 006154b0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006154b0 - the exception-unwinding frame (FS:[0]) and the security-cookie word it pushes are not translated.
/// The `TESReputation` destructor body (the scalar deleting destructor
/// `fn_00615480` runs it): stores the vtables back (`0104ba24`, and the
/// components' at +0x18, +0x24, +0x30), runs `00483710` on `this` (an
/// empty function in the exe), the component destructors `0047a1a0`
/// (+0x30), `00402c40` (+0x24), `00591300` (+0x18) and
/// `TESForm::~TESForm` (`00483630`). `ECX`, no stack words. The
/// exception-unwinding frame is not translated.
pub fn fn_006154b0(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base, 0x0104_ba24);
    e.mem.set_u32(base + 0x18, 0x0104_ba0c);
    e.mem.set_u32(base + 0x24, 0x0104_b9ec);
    e.mem.set_u32(base + 0x30, 0x0104_b9d8);
    e.call(0x0048_3710, &args![this]);
    e.call(0x0047_a1a0, &args![base + 0x30]);
    e.call(0x0040_2c40, &args![base + 0x24]);
    e.call(0x0059_1300, &args![base + 0x18]);
    e.call(0x0048_3630, &args![this]);
}

// Translated from 00615480 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `TESReputation` scalar deleting destructor: runs `fn_006154b0` on
/// `this`, then frees `this` with `00401030` when bit 0 of `flags` is set.
/// Returns `this`. `__thiscall`, `RET 4`. The engine map gives it no name.
pub fn fn_00615480(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_006154b0(e, this);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

// Translated from 006151d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The word `0043d4d0` gives the address of for the object at `011d5a50`
/// (`0043d4d0` is `__thiscall` on it, modelloader.cpp). No arguments. The
/// engine map gives it no name.
pub fn fn_006151d0(e: &mut Engine) -> u32 {
    let at = e.call(0x0043_d4d0, &args![0x011d_5a50u32]).u32();
    e.mem.u32(at)
}

// Translated from 00615300 (decompiled, FalloutNV.exe 1.4.0.525)
/// The body texture index of a body part: `part - 2` for parts 2..=4,
/// -1 for any other (cdecl). The engine map gives it no name.
pub fn fn_00615300(_e: &mut Engine, part: i32) -> i32 {
    let index = part.wrapping_sub(2);
    // `SUB` then `JS`, then a signed compare with 3.
    if !(0..3).contains(&index) {
        -1
    } else {
        index
    }
}

/// The body texture of `sex` and `index` when it has a path.
fn body_texture_with_path(e: &mut Engine, this: Ptr, sex: i32, index: i32) -> Option<u32> {
    if e.call(BODY_TEXTURE, &args![this, sex, index]).u32() == 0 {
        return None;
    }
    let texture = e.call(BODY_TEXTURE, &args![this, sex, index]).u32();
    if e.call(HAS_PATH, &args![texture]).u32() == 0 {
        return None;
    }
    Some(e.call(BODY_TEXTURE, &args![this, sex, index]).u32())
}

// Translated from 006151f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The race's body texture for `sex` (0 or 1) and body part `part`
/// (`fn_00615300` gives the index) that has a path (`0048cee0`), falling
/// back to the other sex's; 0 when neither has one or the arguments are
/// out of range. `__thiscall`, `RET 8`. The engine map gives it no name.
pub fn fn_006151f0(e: &mut Engine, this: Ptr, sex: i32, part: i32) -> u32 {
    let index = fn_00615300(e, part);
    if !(0..2).contains(&sex) || !(0..3).contains(&index) {
        return 0;
    }
    if let Some(texture) = body_texture_with_path(e, this, sex, index) {
        return texture;
    }
    let other = match sex {
        0 => 1,
        1 => 0,
        _ => return 0,
    };
    body_texture_with_path(e, this, other, index).unwrap_or(0)
}

// Translated from 00615330 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00615330 - the /GS stack cookie check (00ec408c) is not translated.
/// Sets the race's string at `this + 0x50c` to `text` when it differs:
/// the text is copied into a stack buffer (`0044a670` its length,
/// `00406d30`), compared with `BSStringT<char>::StrCmp` (case-insensitive
/// flag 1); an equal string answers false, otherwise `004037f0` sets it and
/// it answers true. `__thiscall`, `RET 4`. The engine map gives it no
/// name. The stack cookie check is not translated.
pub fn fn_00615330(e: &mut Engine, this: Ptr, text: u32) -> bool {
    let length = e.call(0x0044_a670, &args![text]).u32().wrapping_add(1);
    // `_alloca_probe_16`: the buffer is the size rounded up to 16 bytes.
    let reserved = length.wrapping_add(15) & !15;
    e.with_stack(reserved, |e, copy| {
        e.call(0x0040_6d30, &args![copy, length, text]);
        let string = this.addr() + 0x50c;
        if e.call(0x0040_8a80, &args![string, copy, 1u32]).u32() == 0 {
            return false;
        }
        e.call(0x0040_37f0, &args![string, copy, 0u32]);
        true
    })
}

/// `00403df0` (`ECX` = setting): the setting's string.
const SETTING_STRING: u32 = 0x0040_3df0;
/// `00403e20` (`ECX` = setting): the address of the setting's `float`.
const SETTING_FLOAT: u32 = 0x0040_3e20;
/// `00408da0` (`ECX` = string): its characters.
const STRING_TEXT: u32 = 0x0040_8da0;
/// `sprintf_s` wrapper (`00406d00`, cdecl: buffer, size, format, ...).
const SPRINTF: u32 = 0x0040_6d00;
/// `"%s\n%s"` (`0104bb5c`).
const FORMAT_TWO_LINES: u32 = 0x0104_bb5c;
/// `00616950` (`ECX` = reputation, which; `RET 4`): the reputation's
/// level byte for fame (1) or infamy (0).
const REPUTATION_LEVEL: u32 = 0x0061_6950;
/// `007052f0` (cdecl, six words): shows the text as a corner message.
const SHOW_MESSAGE: u32 = 0x0070_52f0;
/// The message's display time (`010162c0`, a `float`).
const MESSAGE_TIME: u32 = 0x0101_62c0;
/// The player singleton pointer (`011dea3c`).
const PLAYER: u32 = 0x011d_ea3c;
/// `00953c50` (`ECX` = player, a word; `RET 4`): whether the player is in
/// combat (`PlayerCharacter::IsPlayerCharacterInCombat` in the engine
/// map).
const PLAYER_IN_COMBAT: u32 = 0x0095_3c50;
/// `Interface::InDialog` (engine map name of `007050d0`; no arguments).
const IN_DIALOG: u32 = 0x0070_50d0;
/// The reputation's floor for a decrease (`01012060`, a `double`).
const REPUTATION_FLOOR: u32 = 0x0101_2060;
/// The settings giving the amount for the change levels 1..=5 (the five
/// words `00615c90` and `00615fa0` build on the stack).
const LEVEL_SETTINGS: [u32; 5] = [
    0x011c_bc88,
    0x011c_bcc8,
    0x011c_ba70,
    0x011c_bc54,
    0x011c_bb70,
];

/// Where a reputation change gets its operand: the `float` argument, or
/// the setting of a change level (`00403e20`, read after the first level
/// check).
#[derive(Clone, Copy)]
enum ChangeAmount {
    Float(u32),
    Level(i32),
}

/// The common body of `00615730`, `00615a00`, `00615c90` and `00615fa0`:
/// fame (`which` 1, +0x44) or infamy (+0x48) is raised (clamped to the
/// maximum at +0x40) or lowered (clamped to the `double` at `01012060`),
/// the change is shown (`"%s\n%s"` of the setting text and the name at
/// +0x18, then `007052f0` with the message setting, 0 and the `float` at
/// `010162c0`), and when the level (`00616950`) changed the flag at +0x4C is
/// set and, outside combat and dialog, the message box (`006155f0`) shows,
/// else the player gets `006159e0`; last the form is marked changed
/// (virtual slot 0x48 with 2).
fn change_reputation(e: &mut Engine, this: Ptr, which: i32, amount: ChangeAmount, raise: bool) {
    let base = this.addr();
    e.with_stack(0x100, |e, text| {
        let before = e.call(REPUTATION_LEVEL, &args![this, which as u32]).u8();
        let fame = which == 1;
        let operand = match amount {
            ChangeAmount::Float(bits) => bits,
            ChangeAmount::Level(level) => {
                let setting = LEVEL_SETTINGS[(level - 1) as usize];
                let at = e.call(SETTING_FLOAT, &args![setting]).u32();
                e.mem.u32(at)
            }
        };
        let field = base + if fame { 0x44 } else { 0x48 };
        let current = x87_load32(e.mem.u32(field));
        let changed = if raise {
            x87_add(current, x87_m32(operand))
        } else {
            x87_sub(current, x87_m32(operand))
        };
        e.mem.set_u32(field, x87_store32(changed));
        let value = x87_load32(e.mem.u32(field));
        let clamped = if raise {
            let maximum = x87_load32(e.mem.u32(base + 0x40));
            if value <= maximum {
                e.mem.u32(field)
            } else {
                e.mem.u32(base + 0x40)
            }
        } else {
            let floor: f64 = e.global(REPUTATION_FLOOR);
            if value >= floor {
                e.mem.u32(field)
            } else {
                0
            }
        };
        e.mem.set_u32(field, x87_quiet_bits(clamped));
        let (text_setting, message_setting, flag) = match (fame, raise) {
            (true, true) => (0x011c_bac0u32, 0x011c_bd5cu32, 1u32),
            (false, true) => (0x011c_ba58, 0x011c_bad0, 0),
            (true, false) => (0x011c_bb98, 0x011c_bc34, 0),
            (false, false) => (0x011c_ba94, 0x011c_ba00, 1),
        };
        let line = e.call(SETTING_STRING, &args![text_setting]).u32();
        let name = e.call(STRING_TEXT, &args![base + 0x18]).u32();
        e.call(SPRINTF, &args![text, 0xfau32, FORMAT_TWO_LINES, name, line]);
        let after = e.call(REPUTATION_LEVEL, &args![this, u32::from(fame)]).u8();
        let mut level_changed = false;
        if after != before {
            e.mem.set_u32(base + 0x4c, flag);
            level_changed = true;
        }
        let message = e.call(SETTING_STRING, &args![message_setting]).u32();
        let time = x87_quiet_bits(e.global::<u32>(MESSAGE_TIME));
        e.call(SHOW_MESSAGE, &args![text, 0u32, message, 0u32, time, 0u32]);
        if level_changed {
            let player = e.global::<u32>(PLAYER);
            if e.call(PLAYER_IN_COMBAT, &args![player, 0u32]).u8() == 0
                && e.call(IN_DIALOG, &[]).u8() == 0
            {
                fn_006155f0(e, this);
            } else {
                let player = e.global::<u32>(PLAYER);
                fn_006159e0(e, Ptr::new(player), this.addr());
            }
        }
    });
    e.vcall(base, 0x48, &args![2u32]);
}

// Translated from 00615580 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets fame (+0x44, `which` 1) or infamy (+0x48) to `value` (`FLD`/`FSTP`)
/// and marks the form changed (virtual slot 0x48 with 2; `RET 8`).
pub fn fn_00615580(e: &mut Engine, this: Ptr, which: i32, value: f32) {
    let field = this.addr() + if which == 1 { 0x44 } else { 0x48 };
    e.mem.set_u32(field, x87_quiet_bits(value.to_bits()));
    e.vcall(this.addr(), 0x48, &args![2u32]);
}

// Translated from 006155f0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006155f0 - the /GS stack cookie check (00ec408c) is not translated
/// Shows the reputation level message box: `"%s\n%s"`-style text
/// (`0104bb5c`) of the title (`00616710`) and the level text (`00616890`)
/// in a 0x200-byte buffer, `00718630` on the tutorial manager (`004b7210`
/// + 0x4D4) with 0x27, 0 and 0x1F4, then the message box (`00703f10`) with
/// the text, `006167d0`, the name at +0x18, the button setting `011cbcb0`
/// (the flag at +0x4C is 1) or `011cba30`, the callback `00615720`, 1,
/// 0x17, two zero floats, the setting `011d38b8` and 0.
pub fn fn_006155f0(e: &mut Engine, this: Ptr) {
    e.with_stack(0x200, |e, text| {
        let level_text = e.call(0x0061_6890, &args![this]).u32();
        let title = e.call(0x0061_6710, &args![this]).u32();
        e.call(
            SPRINTF,
            &args![text, 0x200u32, FORMAT_TWO_LINES, title, level_text],
        );
        let manager = e.call(0x004b_7210, &[]).u32();
        e.call(
            0x0071_8630,
            &args![manager.wrapping_add(0x4d4), 0x27u32, 0u32, 0x1f4u32],
        );
        let button_setting = if e.mem.u32(this.addr() + 0x4c) == 1 {
            0x011c_bcb0u32
        } else {
            0x011c_ba30u32
        };
        let last = e.call(SETTING_STRING, &args![0x011d_38b8u32]).u32();
        let button = e.call(SETTING_STRING, &args![button_setting]).u32();
        let name = e.call(STRING_TEXT, &args![this.addr() + 0x18]).u32();
        let image = e.call(0x0061_67d0, &args![this]).u32();
        e.call(
            0x0070_3f10,
            &args![
                text,
                image,
                name,
                button,
                0x0061_5720u32,
                1u32,
                0x17u32,
                0u32,
                0u32,
                last,
                0u32
            ],
        );
    });
}

// Translated from 00615720 (decompiled, FalloutNV.exe 1.4.0.525)
/// The message box's callback (`cdecl`, no arguments): stores the box's
/// result byte (`Interface::GetMessageMenuResult`, `00703fa0`) at
/// `0119a0bc`.
pub fn fn_00615720(e: &mut Engine) {
    let result = e.call(0x0070_3fa0, &[]).u8();
    e.set_global::<u8>(0x0119_a0bc, result);
}

// Translated from 00615730 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00615730 - the /GS stack cookie check (00ec408c) is not translated
/// Raises fame (`which` 1) or infamy by `amount` (`RET 8`), clamped to
/// the maximum at +0x40 (`change_reputation`).
pub fn fn_00615730(e: &mut Engine, this: Ptr, which: i32, amount: f32) {
    change_reputation(e, this, which, ChangeAmount::Float(amount.to_bits()), true);
}

// Translated from 006159e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores a word at +0xE0C (`RET 4`; on the player: the reputation whose
/// level changed while the message box could not show).
pub fn fn_006159e0(e: &mut Engine, this: Ptr, value: u32) {
    e.mem.set_u32(this.addr() + 0xe0c, value);
}

// Translated from 00615a00 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00615a00 - the /GS stack cookie check (00ec408c) is not translated
/// Lowers fame (`which` 1) or infamy by `amount` (`RET 8`), not below the
/// `double` at `01012060` (`change_reputation`).
pub fn fn_00615a00(e: &mut Engine, this: Ptr, which: i32, amount: f32) {
    change_reputation(e, this, which, ChangeAmount::Float(amount.to_bits()), false);
}

// Translated from 00615c90 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00615c90 - the /GS stack cookie check (00ec408c) is not translated
/// Raises fame (`which` 1) or infamy by the amount of change `level` 1..=5
/// (the settings `011cbc88`, `011cbcc8`, `011cba70`, `011cbc54`,
/// `011cbb70`; `RET 8`); any other level does nothing.
pub fn fn_00615c90(e: &mut Engine, this: Ptr, which: i32, level: i32) {
    if level <= 0 || level >= 6 {
        return;
    }
    change_reputation(e, this, which, ChangeAmount::Level(level), true);
}

// Translated from 00615fa0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00615fa0 - the /GS stack cookie check (00ec408c) is not translated
/// Lowers fame (`which` 1) or infamy by the amount of change `level`
/// 1..=5 (as `00615c90`; `RET 8`); any other level does nothing.
pub fn fn_00615fa0(e: &mut Engine, this: Ptr, which: i32, level: i32) {
    if level <= 0 || level >= 6 {
        return;
    }
    change_reputation(e, this, which, ChangeAmount::Level(level), false);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0061_53c0, fn_006153c0(Ptr) -> Ptr),
        entry!(0x0061_5560, fn_00615560(Ptr)),
        entry!(0x0061_55c0, fn_006155c0(Ptr, i32) -> f64),
        entry!(0x0061_54b0, fn_006154b0(Ptr)),
        entry!(0x0061_5480, fn_00615480(Ptr, u32) -> Ptr),
        entry!(0x0061_51d0, fn_006151d0() -> u32),
        entry!(0x0061_5300, fn_00615300(i32) -> i32),
        entry!(0x0061_51f0, fn_006151f0(Ptr, i32, i32) -> u32),
        entry!(0x0061_5330, fn_00615330(Ptr, u32) -> bool),
        entry!(0x0061_5580, fn_00615580(Ptr, i32, f32)),
        entry!(0x0061_55f0, fn_006155f0(Ptr)),
        entry!(0x0061_5720, fn_00615720()),
        entry!(0x0061_5730, fn_00615730(Ptr, i32, f32)),
        entry!(0x0061_59e0, fn_006159e0(Ptr, u32)),
        entry!(0x0061_5a00, fn_00615a00(Ptr, i32, f32)),
        entry!(0x0061_5c90, fn_00615c90(Ptr, i32, i32)),
        entry!(0x0061_5fa0, fn_00615fa0(Ptr, i32, i32)),
    ]
}

#[cfg(test)]
mod tests {
    use crate::units::crt::ctor_test::{self, A};

    #[test]
    fn fn_006153c0_builds_the_object() {
        ctor_test::run(&ctor_test::Case {
            addr: 0x0061_53c0,
            size: 0x50,
            globals: &[],
            returns: &[],
            calls: &[
                (0x0048_3370, &[A::T(0x0)]),
                (0x0040_2d00, &[A::T(0x18)]),
                (0x0040_2c20, &[A::T(0x24)]),
                (0x0047_a130, &[A::T(0x30)]),
                (0x0061_5560, &[A::T(0x0)]),
                (0x004f_15a0, &[A::T(0x0), A::V(0x68)]),
            ],
            stores: &[
                (0x0, 4, 0x0104_ba24),
                (0x18, 4, 0x0104_ba0c),
                (0x24, 4, 0x0104_b9ec),
                (0x30, 4, 0x0104_b9d8),
            ],
        });
    }
}

#[cfg(test)]
mod b022_tests {
    use super::*;

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    #[test]
    fn the_two_floats() {
        let mut e = Engine::new();
        let object = e.mem.alloc(0x50);
        e.mem.set_f32(object + 0x44, 3.5);
        e.mem.set_f32(object + 0x48, -2.0);
        assert_eq!(e.call(0x0061_55c0, &[object, 1]).f64(), 3.5);
        assert_eq!(e.call(0x0061_55c0, &[object, 0]).f64(), -2.0);
        assert_eq!(e.call(0x0061_55c0, &[object, 2]).f64(), -2.0);
        // A signalling NaN comes back quiet.
        e.mem.set_u32(object + 0x44, 0x7f80_0001);
        let nan = e.call(0x0061_55c0, &[object, 1]).f64();
        assert_eq!((nan as f32).to_bits(), 0x7fc0_0001);
        e.call(0x0061_5560, &[object]);
        assert_eq!(e.mem.u32(object + 0x44), 0);
        assert_eq!(e.mem.u32(object + 0x48), 0);
    }

    #[test]
    fn the_destructor_runs_the_bases_and_deletes_on_flag_one() {
        let mut e = Engine::new();
        let callees = [
            0x0048_3710,
            0x0047_a1a0,
            0x0040_2c40,
            0x0059_1300,
            0x0048_3630,
            0x0040_1030,
        ];
        for callee in callees {
            e.register(callee, |_, _| Ret::default());
        }
        let object = e.mem.alloc(0x50);
        for flags in [0u32, 1] {
            e.call_log = Some(vec![]);
            assert_eq!(e.call(0x0061_5480, &[object, flags]).u32(), object);
            let log = e.call_log.take().unwrap();
            let mut expected = vec![
                (0x0048_3710, vec![object]),
                (0x0047_a1a0, vec![object + 0x30]),
                (0x0040_2c40, vec![object + 0x24]),
                (0x0059_1300, vec![object + 0x18]),
                (0x0048_3630, vec![object]),
            ];
            if flags == 1 {
                expected.push((0x0040_1030, vec![object]));
            }
            assert_eq!(log[1..], expected[..]);
            assert_eq!(e.mem.u32(object), 0x0104_ba24);
            assert_eq!(e.mem.u32(object + 0x30), 0x0104_b9d8);
        }
    }

    #[test]
    fn the_word_of_the_global_object() {
        let mut e = Engine::new();
        let word = e.mem.alloc(4);
        e.mem.set_u32(word, 0x77);
        e.register_double(0x0043_d4d0, move |_, _| ret(word));
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0061_51d0, &[]).u32(), 0x77);
        assert_eq!(calls_to(&e, 0x0043_d4d0), vec![vec![0x011d_5a50]]);
    }

    #[test]
    fn body_part_indices() {
        let mut e = Engine::new();
        let index = |e: &mut Engine, part: i32| e.call(0x0061_5300, &args![part]).i32();
        assert_eq!(index(&mut e, 1), -1);
        assert_eq!(index(&mut e, 2), 0);
        assert_eq!(index(&mut e, 4), 2);
        assert_eq!(index(&mut e, 5), -1);
        assert_eq!(index(&mut e, i32::MIN), -1);
    }

    /// The race double: texture `(sex, index)` is `0x100 * (sex + 1) +
    /// index`; textures listed in `with_path` have a path.
    fn race(with_path: &'static [u32]) -> Engine {
        let mut e = Engine::new();
        e.register(BODY_TEXTURE, |_, a| ret(0x100 * (a[1] + 1) + a[2]));
        e.register_double(HAS_PATH, move |_, a| {
            ret(u32::from(with_path.contains(&a[0])))
        });
        e
    }

    #[test]
    fn body_textures_fall_back_to_the_other_sex() {
        let mut e = race(&[0x101, 0x202]);
        let get = |e: &mut Engine, sex: i32, part: i32| {
            e.call(0x0061_51f0, &args![0x1000u32, sex, part]).u32()
        };
        assert_eq!(get(&mut e, 0, 3), 0x101);
        assert_eq!(get(&mut e, 1, 3), 0x101);
        assert_eq!(get(&mut e, 0, 4), 0x202);
        assert_eq!(get(&mut e, 0, 2), 0);
        assert_eq!(get(&mut e, 2, 3), 0);
        assert_eq!(get(&mut e, -1, 3), 0);
        assert_eq!(get(&mut e, 0, 9), 0);
    }

    #[test]
    fn the_name_string_is_set_when_it_differs() {
        let mut e = Engine::new();
        e.register(0x0044_a670, |e, a| ret(e.mem.cstr(a[0]).len() as u32));
        e.register(0x0040_6d30, |e, a| {
            let text = e.mem.cstr(a[2]);
            e.mem.set_cstr(a[0], &text);
            Ret::default()
        });
        e.register(0x0040_37f0, |_, _| Ret::default());
        let text = e.mem.alloc(8);
        e.mem.set_cstr(text, b"Nord");
        let race = e.mem.alloc(0x524);
        for (equal, changed) in [(0u32, false), (1, true)] {
            e.register_double(0x0040_8a80, move |_, _| ret(equal));
            e.call_log = Some(vec![]);
            assert_eq!(e.call(0x0061_5330, &[race, text]).bool(), changed);
            let compares = calls_to(&e, 0x0040_8a80);
            assert_eq!(compares[0][0], race + 0x50c);
            assert_eq!(compares[0][2], 1);
            assert_eq!(calls_to(&e, 0x0040_6d30)[0][1], 5);
            let sets = calls_to(&e, 0x0040_37f0);
            assert_eq!(sets.len(), usize::from(changed));
            if changed {
                assert_eq!(sets[0], vec![race + 0x50c, compares[0][1], 0]);
            }
        }
    }
}

#[cfg(test)]
mod tests_n042 {
    use super::*;

    type Log = Vec<(u32, Vec<u32>)>;

    fn run(e: &mut Engine, addr: u32, words: &[u32]) -> (Ret, Log) {
        e.call_log = Some(vec![]);
        let result = e.call(addr, words);
        let log = e
            .call_log
            .take()
            .unwrap()
            .into_iter()
            .filter(|(a, _)| *a != addr)
            .collect();
        (result, log)
    }

    fn calls(log: &Log, addr: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    const MARK_CHANGED: u32 = 0x0f00_1000;

    /// A reputation with a vtable whose slot 0x48 is a logged double.
    fn engine() -> (Engine, u32) {
        let mut e = Engine::new();
        e.register(MARK_CHANGED, |_, _| Ret::default());
        let mut slots = vec![0u32; 0x4c / 4];
        slots[0x48 / 4] = MARK_CHANGED;
        e.put_vtable(0x0f00_0000, &slots);
        let this = e.mem.alloc(0x50);
        e.mem.set_u32(this, 0x0f00_0000);
        (e, this)
    }

    #[test]
    fn set_value_and_small_helpers() {
        let (mut e, this) = engine();
        let (_, log) = run(&mut e, 0x0061_5580, &[this, 1, 0x7f80_0001]);
        assert_eq!(e.mem.u32(this + 0x44), 0x7fc0_0001);
        assert_eq!(log, vec![(MARK_CHANGED, vec![this, 2])]);
        e.call(0x0061_5580, &args![this, 0u32, 2.5f32]);
        assert_eq!(e.mem.u32(this + 0x48), 2.5f32.to_bits());
        let player = e.mem.alloc(0xe10);
        e.call(0x0061_59e0, &args![player, 0x1234u32]);
        assert_eq!(e.mem.u32(player + 0xe0c), 0x1234);
        e.register(0x0070_3fa0, |_, _| 0x1_0003u32.into_ret());
        e.map(0x0119_a0bc, 4);
        e.call(0x0061_5720, &[]);
        assert_eq!(e.mem.u8(0x0119_a0bc), 3);
    }

    fn change_engine(level_before: u32, level_after: u32) -> (Engine, u32) {
        let (mut e, this) = engine();
        e.register(SETTING_STRING, |_, a| (a[0] + 1).into_ret());
        e.register(STRING_TEXT, |_, _| 0x7100u32.into_ret());
        e.register(SPRINTF, |_, _| Ret::default());
        e.register(SHOW_MESSAGE, |_, _| Ret::default());
        let calls_seen = e.mem.alloc(4);
        e.register_double(REPUTATION_LEVEL, move |e, _| {
            let n = e.mem.u32(calls_seen);
            e.mem.set_u32(calls_seen, n + 1);
            if n % 2 == 0 {
                level_before
            } else {
                level_after
            }
            .into_ret()
        });
        e.map(MESSAGE_TIME, 4);
        e.mem.set_u32(MESSAGE_TIME, 3.0f32.to_bits());
        e.map(REPUTATION_FLOOR, 8);
        e.set_global::<f64>(REPUTATION_FLOOR, 0.0);
        e.map(PLAYER, 4);
        e.mem.set_u32(PLAYER, 0x4400);
        e.register(PLAYER_IN_COMBAT, |_, _| Ret::default());
        e.register(IN_DIALOG, |_, _| Ret::default());
        // The message box's callees.
        for addr in [
            0x0061_6890,
            0x0061_6710,
            0x004b_7210,
            0x0071_8630,
            0x0061_67d0,
            0x0070_3f10,
        ] {
            e.register(addr, |_, _| Ret::default());
        }
        e.mem.set_u32(this + 0x40, 100.0f32.to_bits());
        (e, this)
    }

    #[test]
    fn raising_fame_clamps_to_the_maximum_and_shows_the_change() {
        let (mut e, this) = change_engine(2, 2);
        e.mem.set_u32(this + 0x44, 90.0f32.to_bits());
        let (_, log) = run(&mut e, 0x0061_5730, &[this, 1, 25.0f32.to_bits()]);
        assert_eq!(f32::from_bits(e.mem.u32(this + 0x44)), 100.0);
        let text = calls(&log, SPRINTF)[0][0];
        assert_eq!(
            calls(&log, SPRINTF),
            vec![vec![text, 0xfa, FORMAT_TWO_LINES, 0x7100, 0x011c_bac1]]
        );
        assert_eq!(
            calls(&log, REPUTATION_LEVEL),
            vec![vec![this, 1], vec![this, 1]]
        );
        assert_eq!(
            calls(&log, SHOW_MESSAGE),
            vec![vec![text, 0, 0x011c_bd5d, 0, 3.0f32.to_bits(), 0]]
        );
        assert!(calls(&log, 0x0070_3f10).is_empty());
        assert_eq!(log.last().unwrap(), &(MARK_CHANGED, vec![this, 2]));
        // Below the maximum the sum stays.
        e.mem.set_u32(this + 0x44, 10.0f32.to_bits());
        e.call(0x0061_5730, &args![this, 1u32, 25.0f32]);
        assert_eq!(f32::from_bits(e.mem.u32(this + 0x44)), 35.0);
    }

    #[test]
    fn a_level_change_shows_the_box_or_tells_the_player() {
        let (mut e, this) = change_engine(1, 2);
        e.mem.set_u32(this + 0x48, 10.0f32.to_bits());
        e.mem.set_u32(this + 0x4c, 1);
        let (_, log) = run(&mut e, 0x0061_5730, &[this, 0, 5.0f32.to_bits()]);
        assert_eq!(f32::from_bits(e.mem.u32(this + 0x48)), 15.0);
        // Infamy raised: the flag goes to 0, the box shows with 011cba30.
        assert_eq!(e.mem.u32(this + 0x4c), 0);
        assert_eq!(
            calls(&log, REPUTATION_LEVEL),
            vec![vec![this, 0], vec![this, 0]]
        );
        let shown = calls(&log, 0x0070_3f10);
        assert_eq!(shown.len(), 1);
        assert_eq!(shown[0][3], 0x011c_ba31);
        assert_eq!(shown[0][4], 0x0061_5720);
        assert_eq!(shown[0][9], 0x011d_38b9);
        assert_eq!(calls(&log, 0x0071_8630), vec![vec![0x4d4, 0x27, 0, 0x1f4]]);
        // In combat the player gets the reputation instead.
        e.register(PLAYER_IN_COMBAT, |_, _| 1u32.into_ret());
        let player = e.mem.alloc(0xe10);
        e.mem.set_u32(PLAYER, player);
        let (_, log) = run(&mut e, 0x0061_5730, &[this, 0, 5.0f32.to_bits()]);
        assert!(calls(&log, 0x0070_3f10).is_empty());
        assert_eq!(e.mem.u32(player + 0xe0c), this);
    }

    #[test]
    fn lowering_clamps_to_the_floor() {
        let (mut e, this) = change_engine(3, 2);
        e.register(PLAYER_IN_COMBAT, |_, _| 1u32.into_ret());
        let player = e.mem.alloc(0xe10);
        e.mem.set_u32(PLAYER, player);
        e.mem.set_u32(this + 0x44, 10.0f32.to_bits());
        let (_, log) = run(&mut e, 0x0061_5a00, &[this, 1, 25.0f32.to_bits()]);
        assert_eq!(e.mem.u32(this + 0x44), 0);
        assert_eq!(e.mem.u32(this + 0x4c), 0);
        assert_eq!(calls(&log, SPRINTF)[0][4], 0x011c_bb99);
        assert_eq!(calls(&log, SHOW_MESSAGE)[0][2], 0x011c_bc35);
        e.mem.set_u32(this + 0x48, 30.0f32.to_bits());
        let (_, log) = run(&mut e, 0x0061_5a00, &[this, 0, 25.0f32.to_bits()]);
        assert_eq!(f32::from_bits(e.mem.u32(this + 0x48)), 5.0);
        assert_eq!(e.mem.u32(this + 0x4c), 1);
        assert_eq!(calls(&log, SPRINTF)[0][4], 0x011c_ba95);
        assert_eq!(calls(&log, SHOW_MESSAGE)[0][2], 0x011c_ba01);
        // A NaN result is clamped to 0.
        e.mem.set_u32(this + 0x48, 0x7fc0_0000);
        e.call(0x0061_5a00, &args![this, 0u32, 1.0f32]);
        assert_eq!(e.mem.u32(this + 0x48), 0);
    }

    #[test]
    fn level_changes_take_the_amount_from_the_settings() {
        let (mut e, this) = change_engine(2, 2);
        let amount = e.mem.alloc(4);
        e.mem.set_u32(amount, 7.0f32.to_bits());
        e.register_double(SETTING_FLOAT, move |_, _| amount.into_ret());
        e.mem.set_u32(this + 0x44, 1.0f32.to_bits());
        let (_, log) = run(&mut e, 0x0061_5c90, &[this, 1, 3]);
        assert_eq!(f32::from_bits(e.mem.u32(this + 0x44)), 8.0);
        assert_eq!(calls(&log, SETTING_FLOAT), vec![vec![0x011c_ba70]]);
        let (_, log) = run(&mut e, 0x0061_5fa0, &[this, 1, 5]);
        assert_eq!(f32::from_bits(e.mem.u32(this + 0x44)), 1.0);
        assert_eq!(calls(&log, SETTING_FLOAT), vec![vec![0x011c_bb70]]);
        for level in [0u32, 6, 0xffff_ffff] {
            let (_, log) = run(&mut e, 0x0061_5c90, &[this, 1, level]);
            assert!(log.is_empty());
            let (_, log) = run(&mut e, 0x0061_5fa0, &[this, 1, level]);
            assert!(log.is_empty());
        }
    }
}

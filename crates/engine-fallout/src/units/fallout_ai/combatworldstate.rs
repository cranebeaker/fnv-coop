//! `fallout/ai/combat/combatworldstate.cpp` (Xbox PDB source unit), subsystem `fallout/ai`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

layout! {
    /// `CombatWorldStateArray` (Xbox PDB), 0x5C bytes: twenty `int`
    /// values by world state, then three "specified variables".
    pub struct CombatWorldStateArray: 0x5c {
        /// `iValues` (Xbox PDB): the first of 20 words.
        0x00 iValues: u32,
        /// `iSpecifiedVariables` (Xbox PDB): the first of 3 words.
        0x50 iSpecifiedVariables: u32,
    }
}

/// `CombatWorldStateArray::IsTrue`'s table of world-state kinds (8 bytes
/// per state; the byte at +0 says whether the state's value is a bit mask).
const STATE_IS_MASK_TABLE: u32 = 0x011a_55b4;
/// The array's value getter (`005bd5c0`, thiscall(state): the word at
/// `this + state * 4`).
const GET_VALUE: u32 = 0x005b_d5c0;
/// `memcpy` (`00401460`, cdecl(dest, source, size)).
const MEMCPY: u32 = 0x0040_1460;

// Translated from 009aad00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets a world state's value (thiscall(`state`, `value`), `RET 8`):
/// `iValues[state] = value` (no bounds check).
pub fn fn_009aad00(e: &mut Engine, this: Ptr<CombatWorldStateArray>, state: u32, value: u32) {
    let address = this.addr().wrapping_add(state.wrapping_mul(4));
    e.mem.set_u32(address, value);
}

// Translated from 009aad20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets a specified variable (thiscall(`variable`, `value`), `RET 8`):
/// the word at `this + variable * 4 + 0x3FFF8` (32-bit wrap), so the
/// variables 0xFFFF0016 .. 0xFFFF0018 the update passes land on
/// `iSpecifiedVariables[0..3]` (+0x50 .. +0x58).
pub fn fn_009aad20(e: &mut Engine, this: Ptr<CombatWorldStateArray>, variable: u32, value: u32) {
    let address = this
        .addr()
        .wrapping_add(variable.wrapping_mul(4))
        .wrapping_add(0x3fff8);
    e.mem.set_u32(address, value);
}

// Translated from 009aad40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether the signed byte at +0x73 of `this` is not zero (thiscall, no
/// stack words; `NEG`/`SBB`/`NEG`, so 0 or 1). Called on the combat
/// controller.
pub fn fn_009aad40(e: &mut Engine, this: Ptr) -> bool {
    e.mem.u8(this.addr().wrapping_add(0x73)) != 0
}

// Translated from 009aad60 (decompiled, FalloutNV.exe 1.4.0.525)
/// The address `this + index * 8 + 0x1AC` (thiscall(`index`), `RET 4`).
/// Called on the combat controller.
pub fn fn_009aad60(_e: &mut Engine, this: Ptr, index: u32) -> u32 {
    this.addr()
        .wrapping_add(index.wrapping_mul(8))
        .wrapping_add(0x1ac)
}

// Translated from 009aad80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Copies a `CombatWorldStateArray` (thiscall(`source`), `RET 4`): `memcpy`
/// of the 0x50 bytes of `iValues`, then of the 0xC bytes of
/// `iSpecifiedVariables`.
pub fn fn_009aad80(
    e: &mut Engine,
    this: Ptr<CombatWorldStateArray>,
    source: Ptr<CombatWorldStateArray>,
) {
    e.call(MEMCPY, &args![this, source, 0x50u32]);
    e.call(
        MEMCPY,
        &args![this.addr() + 0x50, source.addr().wrapping_add(0x50), 0xcu32],
    );
}

// Translated from 009aadc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatWorldStateArray::IsTrue` (Xbox PDB; thiscall(`state`, `value`),
/// `RET 8`): for a state whose table byte (`011a55b4 + state * 8`) is set,
/// whether bit `value` (mod 32, `SHL` by `CL`) of its value (`005bd5c0`) is
/// set; otherwise whether its value equals `value`.
pub fn combat_world_state_array_is_true(
    e: &mut Engine,
    this: Ptr<CombatWorldStateArray>,
    state: u32,
    value: u32,
) -> bool {
    let kind = e
        .mem
        .u8(state.wrapping_mul(8).wrapping_add(STATE_IS_MASK_TABLE));
    if kind != 0 {
        let current = e.call(GET_VALUE, &args![this, state]).u32();
        current & (1u32 << (value & 0x1f)) != 0
    } else {
        let current = e.call(GET_VALUE, &args![this, state]).u32();
        current == value
    }
}

// Translated from 00dcb0c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The body the engine map names `CombatWorldState::CombatWorldState`
/// (identical-code folding; its only reference is slot +0xC of the vtable
/// at `010d7494`, whose RTTI is `.?AUObject@StructArrayImplementation@@`):
/// stores `ECX` (`this`) at `*out` and `value` at `out + 4`, and returns
/// `out` (`RET 8`; it reads no frame).
pub fn fn_00dcb0c0(e: &mut Engine, this: u32, out: Ptr, value: u32) -> Ptr {
    e.mem.set_u32(out.addr(), this);
    e.mem.set_u32(out.addr().wrapping_add(4), value);
    out
}

/// `0.1` (`double`, the `float` 0.1 widened): the height scale of the
/// cover-distance check.
const COVER_HEIGHT_SCALE: u32 = 0x0101_ffa0;
/// `1024.0` (`double`): the squared distance under which the cover state
/// bit is set.
const COVER_DISTANCE_SQUARED: u32 = 0x0101_ecb8;
/// `589824.0` (`double`, 768 squared): the squared distance to the target
/// under which the movement check runs.
const TARGET_DISTANCE_SQUARED: u32 = 0x0108_cb08;
/// `100.0` (`double`): health fraction to percent.
const PERCENT: u32 = 0x0101_7a40;
/// The setting whose `float` the threat score is compared with (state 0xE).
const THREAT_SETTING: u32 = 0x011c_fbcc;
/// The setting whose `float` the time since the target was last detected
/// is compared with (state 3).
const DETECTION_TIME_SETTING: u32 = 0x011c_e528;
/// The integer setting the health percentage is compared with.
const HEALTH_PERCENT_SETTING: u32 = 0x011c_e0b4;
/// `"COMBAT: Unsupported target ref type %i in world state update"`.
const UNSUPPORTED_TARGET_MESSAGE: u32 = 0x0108_e9cc;
/// A `float` setting's value address (`00403e20`, thiscall).
const SETTING_VALUE: u32 = 0x0040_3e20;
/// An integer setting's value address (`0043d4d0`, thiscall).
const INT_SETTING_VALUE: u32 = 0x0043_d4d0;
/// `TESObjectREFR::GetWorldLocation` (`00575650`, thiscall(out)): fills a
/// 16-byte world location and returns its address.
const GET_WORLD_LOCATION: u32 = 0x0057_5650;
/// The procedure's state test `[+8] == 1` (`0097e310`, thiscall, byte).
const PROCEDURE_STATE_IS_1: u32 = 0x0097_e310;

/// Sets bit `bit` (mod 32) of `flags`.
fn with_bit(flags: u32, bit: u32) -> u32 {
    flags | 1u32 << (bit & 0x1f)
}

// Translated from 009aa4e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatWorldStateArray::Update` (Xbox PDB; thiscall(`controller`), `RET
/// 4`): clears the array (`009aa4b0`), checks the controller's weapon
/// (`CombatController::CheckCurrentWeapon`), then fills the world states
/// from the controller (PC offsets; Xbox PDB names 0x10 higher):
/// `combat_state` (+0x9C, `00586150`), the attacker (+0xBC, `0097ae90`),
/// the target (+0xC0, `004030b0`), the combat group (+0x80, `004fb070`)
/// and the action and movement procedures (+0x84 `004fd3c0`, +0x88
/// `004fd3e0`).
///
/// State 1 is a bit set (state 2 its complement): bit (cover location +4)
/// for a cover location (`0097b050`) whose `009373f0` byte is clear, when
/// the movement procedure is not of type 7 (virtual +0x34) and the cover
/// positions (`CombatUtilities::CalculateCoverPositions`) lie within a
/// squared distance of 1024 of the attacker (height scaled by 0.1), or
/// when it is of type 7 and not in state 1 (`0097e310`); bits 0x11 + i for
/// the combat items (`0097cfc0(i)`, i < 5) whose first word is the index
/// of an action procedure of type 8 not in state 1 (`0044edb0`); bit 2
/// when the target is within a squared distance of 589824, `0097fd40`
/// answers false and `CombatState::CheckMovement(target location, 1, 0)`
/// true; bits 0x80, 0x100, 0x200 and 0x10000 always, one cleared by the
/// location target (`007058c0`, when its byte +1 is clear) of type 0 .. 3
/// (another type is reported through `005b5e40`); 0x1000 for flee state 2
/// (`00662a80`); 0x400 for a movement procedure of type 9 in neither state
/// 1 nor 2 (`00666f00`).
///
/// Then: 0xE whether the threat score (`00644790`) exceeds the setting at
/// `011cfbcc`; 8 whether the state's +0x78 word is set; without a target
/// 3 = 1, else 4 (`0097ee90`), 5 (`00998980` and not
/// `bTargetSpaceDifferent`), 6 (`bTargetBlocked`), 7
/// (`bTargetSpaceDifferent`), 3 (the time since the last detection,
/// `0097fae0`/`00435e00`, exceeds the setting at `011ce528`) and 9
/// (`cTargetFlushed`); 0xB and specified variable 2 the combat weapon type
/// (6 without a weapon, 7 when the weapon is not the state's weapon of
/// that type), 0xA the same unless the attacker's process (`008d8520`)
/// virtual +0x454 answers false (then 7); 0xC `iHasCombatWeapon`;
/// specified variable 0 the first combat item index + 0x11; 0xD
/// `iHasCombatItem`; specified variable 1 the first usable combat item
/// (`0097d110(i)`, i < 2) whose timer (`009aad60`, `008d7f80`) is ready
/// and that applies (item 0: health percent below the integer setting at
/// `011ce0b4`; item 1 always), + 0x11; 0xF `009a1380`; 0x13 the combat
/// group's `0098a310(attacker)` when its +0x28 word is set; 0x12 the
/// combat permissions (`00516bf0`) with 0x10000 (`0097ef30` on the group),
/// 0x20000 (the controller's byte +0xD5, `009a0240`) and 0x40000 unless
/// the target's base object's +0x30 component answers true (virtual
/// +0x14).
pub fn combat_world_state_array_update(
    e: &mut Engine,
    this: Ptr<CombatWorldStateArray>,
    controller: u32,
) {
    // The function's frame: locals at `ebp - x` as the exe keeps them.
    e.with_stack(0xd8, |e, frame| {
        let ebp = frame.addr() + 0xd8;
        let attacker_location = ebp - 0x40;
        let target_location = ebp - 0x18;
        let cover_point = ebp - 0x6c;
        let offset = ebp - 0x60;

        e.call(0x009a_a4b0, &args![this]);
        e.call(0x0097_f470, &args![controller]);
        let state = e.call(0x0058_6150, &args![controller]).u32();
        let attacker = e.call(0x0097_ae90, &args![controller]).u32();
        let target = e.call(0x0040_30b0, &args![controller]).u32();
        e.call(GET_WORLD_LOCATION, &args![attacker, attacker_location]);
        e.call(0x0097_ede0, &args![target_location]);
        if target != 0 {
            let location = e.call(GET_WORLD_LOCATION, &args![target, ebp - 0x94]).u32();
            for word in 0..4 {
                let value = e.mem.u32(location.wrapping_add(4 * word));
                e.mem.set_u32(target_location + 4 * word, value);
            }
        }
        let group = e.call(0x004f_b070, &args![controller]).u32();
        let mut flags = 0u32;
        let action = e.call(0x004f_d3c0, &args![controller]).u32();
        let movement = e.call(0x004f_d3e0, &args![controller]).u32();
        let cover = e.call(0x0097_b050, &args![state]).u32();
        if cover != 0 && !e.call(0x0093_73f0, &args![cover]).bool() {
            if movement != 0 && e.vcall(movement, 0x34, &args![]).u32() == 7 {
                if !e.call(PROCEDURE_STATE_IS_1, &args![movement]).bool() {
                    flags = with_bit(flags, e.mem.u32(cover.wrapping_add(4)));
                }
            } else {
                e.call(0x0068_15c0, &args![cover_point]);
                e.call(0x009a_9370, &args![controller, cover, cover_point, 0u32]);
                let point = e.call(0x0068_15c0, &args![attacker_location]).u32();
                e.call(0x0043_9ef0, &args![cover_point, offset, point]);
                let height = e.mem.u32(offset + 8);
                let scale = e.global::<f64>(COVER_HEIGHT_SCALE);
                let scaled = x87_store32(x87_mul(x87_load32(height), scale));
                e.mem.set_u32(offset + 8, scaled);
                let squared = e.call(0x004a_7290, &args![offset]).f64();
                // `FCOMP`/`TEST AH,5`/`JP`: only an ordered "below" sets it.
                if squared < e.global::<f64>(COVER_DISTANCE_SQUARED) {
                    flags = with_bit(flags, e.mem.u32(cover.wrapping_add(4)));
                }
            }
        }
        if action != 0
            && !e.call(PROCEDURE_STATE_IS_1, &args![action]).bool()
            && e.vcall(action, 0x34, &args![]).u32() == 8
            && e.call(0x0044_edb0, &args![action]).u32() != 0
        {
            for item in 0..5u32 {
                let entry = e.call(0x0097_cfc0, &args![state, item]).u32();
                if entry != 0 {
                    let index = e.call(0x0044_edb0, &args![action]).u32();
                    if e.mem.u32(entry) == index {
                        flags = with_bit(flags, item + 0x11);
                    }
                }
            }
        }
        if target != 0 {
            let squared = e
                .call(0x0052_7a40, &args![target_location, attacker_location])
                .f64();
            if squared < e.global::<f64>(TARGET_DISTANCE_SQUARED)
                && !e.call(0x0097_fd40, &args![state]).bool()
                && e.call(0x009a_02a0, &args![state, target_location, 1u32, 0u32])
                    .bool()
            {
                flags |= 2;
            }
        }
        flags |= 0x80;
        flags |= 0x100;
        flags |= 0x200;
        flags |= 0x10000;
        let location_target = e.call(0x0070_58c0, &args![state]).u32();
        if location_target != 0 && e.mem.u8(location_target.wrapping_add(1)) == 0 {
            // The exe's jump table (`009aacf0`) for types 0 .. 3.
            match e.mem.u8(location_target) {
                0 => flags &= 0xffff_ff7f,
                1 => flags &= 0xffff_feff,
                2 => flags &= 0xffff_fdff,
                3 => flags &= 0xfffe_ffff,
                kind => {
                    e.call(
                        0x005b_5e40,
                        &args![UNSUPPORTED_TARGET_MESSAGE, u32::from(kind)],
                    );
                }
            }
        }
        if e.call(0x0066_2a80, &args![state]).u8() == 2 {
            flags |= 0x1000;
        }
        if movement != 0
            && e.vcall(movement, 0x34, &args![]).u32() == 9
            && !e.call(PROCEDURE_STATE_IS_1, &args![movement]).bool()
            && !e.call(0x0066_6f00, &args![movement]).bool()
        {
            flags |= 0x400;
        }
        fn_009aad00(e, this, 1, flags);
        fn_009aad00(e, this, 2, !flags);

        let threat = e.call(0x0064_4790, &args![state]).f64();
        let setting = e.call(SETTING_VALUE, &args![THREAT_SETTING]).u32();
        // `FLD` the setting, `FCOMP` the stored `double`, `TEST AH,5`/`JP`.
        let above = x87_m32(e.mem.u32(setting)) < threat;
        fn_009aad00(e, this, 0xe, u32::from(above));
        let word = e.call(0x0040_7820, &args![state]).u32();
        fn_009aad00(e, this, 8, u32::from(word != 0));
        if target == 0 {
            fn_009aad00(e, this, 3, 1);
        } else {
            let value = e.call(0x0097_ee90, &args![state]).u8();
            fn_009aad00(e, this, 4, u32::from(value));
            let value = e.call(0x0099_8980, &args![state]).bool()
                && !e.call(0x008b_3c10, &args![state]).bool();
            fn_009aad00(e, this, 5, u32::from(value));
            let value = e.call(0x008b_3c90, &args![state]).u8();
            fn_009aad00(e, this, 6, u32::from(value));
            let value = e.call(0x008b_3c10, &args![state]).u8();
            fn_009aad00(e, this, 7, u32::from(value));
            let stamp = e.call(0x0097_fae0, &args![state, ebp - 0x98]).u32();
            let elapsed = e.call(0x0043_5e00, &args![stamp]).f64();
            let setting = e.call(SETTING_VALUE, &args![DETECTION_TIME_SETTING]).u32();
            let above = x87_m32(e.mem.u32(setting)) < elapsed;
            fn_009aad00(e, this, 3, u32::from(above));
            let flushed = fn_009aad40(e, Ptr::new(state));
            fn_009aad00(e, this, 9, u32::from(flushed));
        }
        let weapon = e.call(0x008a_1710, &args![attacker]).u32();
        let kind = if weapon != 0 {
            e.call(0x0052_2c80, &args![weapon]).u32()
        } else {
            6
        };
        let mut weapon_type = kind;
        if weapon != 0 && weapon != e.call(0x0097_c150, &args![state, kind]).u32() {
            weapon_type = 7;
        }
        fn_009aad00(e, this, 0xb, weapon_type);
        fn_009aad20(e, this, 0xffff_0018, weapon_type);
        let process = e.call(0x008d_8520, &args![attacker]).u32();
        let value = if e.vcall(process, 0x454, &args![]).bool() {
            weapon_type
        } else {
            7
        };
        fn_009aad00(e, this, 10, value);
        let value = e.call(0x009e_32d0, &args![state]).u32();
        fn_009aad00(e, this, 0xc, value);
        for item in 0..5u32 {
            if e.call(0x0097_cfc0, &args![state, item]).u32() != 0 {
                fn_009aad20(e, this, 0xffff_0016, item + 0x11);
                break;
            }
        }
        let value = e.call(0x008d_8160, &args![state]).u32();
        fn_009aad00(e, this, 0xd, value);
        for item in 0..2u32 {
            if e.call(0x0097_d110, &args![state, item]).u32() == 0 {
                continue;
            }
            let timer = fn_009aad60(e, Ptr::new(state), item);
            if !e.call(0x008d_7f80, &args![timer]).bool() {
                continue;
            }
            let mut applies = false;
            match item + 0x11 {
                0x11 => {
                    let health = e.call(0x0089_3590, &args![attacker]).f64();
                    let percent = x87_mul(health, e.global::<f64>(PERCENT));
                    let setting = e
                        .call(INT_SETTING_VALUE, &args![HEALTH_PERCENT_SETTING])
                        .u32();
                    let limit = f64::from(e.mem.i32(setting));
                    // `FILD`, `FCOMP` the stored `double`, `TEST AH,0x41`:
                    // only an ordered "above" applies.
                    if limit > percent {
                        applies = true;
                    }
                }
                0x12 => applies = true,
                _ => {}
            }
            if applies {
                fn_009aad20(e, this, 0xffff_0017, item + 0x11);
                break;
            }
        }
        let value = e.call(0x009a_1380, &args![state]).u8();
        fn_009aad00(e, this, 0xf, u32::from(value));
        if e.call(0x0045_cd60, &args![group]).u32() != 0 {
            let value = e.call(0x0098_a310, &args![group, attacker]).u32();
            fn_009aad00(e, this, 0x13, value);
        }
        let mut permissions = e.call(0x0051_6bf0, &args![state]).u32();
        if e.call(0x0097_ef30, &args![group]).bool() {
            permissions |= 0x10000;
        }
        if e.call(0x009a_0240, &args![controller]).bool() {
            permissions |= 0x20000;
        }
        let keeps = target != 0 && {
            let base = e.call(0x0041_81e0, &args![target]).u32();
            e.vcall(base.wrapping_add(0x30), 0x14, &args![]).bool()
        };
        if !keeps {
            permissions |= 0x40000;
        }
        fn_009aad00(e, this, 0x12, permissions);
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x009a_a4e0,
            combat_world_state_array_update(Ptr<CombatWorldStateArray>, u32)
        ),
        entry!(
            0x009a_ad00,
            fn_009aad00(Ptr<CombatWorldStateArray>, u32, u32)
        ),
        entry!(
            0x009a_ad20,
            fn_009aad20(Ptr<CombatWorldStateArray>, u32, u32)
        ),
        entry!(0x009a_ad40, fn_009aad40(Ptr) -> bool),
        entry!(0x009a_ad60, fn_009aad60(Ptr, u32) -> u32),
        entry!(
            0x009a_ad80,
            fn_009aad80(Ptr<CombatWorldStateArray>, Ptr<CombatWorldStateArray>)
        ),
        entry!(
            0x009a_adc0,
            combat_world_state_array_is_true(Ptr<CombatWorldStateArray>, u32, u32) -> bool
        ),
        entry!(0x00dc_b0c0, fn_00dcb0c0(u32, Ptr, u32) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    type CallLog = [(u32, Vec<u32>)];

    fn calls(log: &CallLog, addr: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    fn st0(value: f64) -> Ret {
        Ret {
            st0: value,
            ..Ret::default()
        }
    }

    #[test]
    fn values_and_specified_variables_are_stored_by_index() {
        let mut e = Engine::new();
        let this = e.new_object::<CombatWorldStateArray>();
        e.call(0x009a_ad00, &args![this, 3u32, 0x1234u32]);
        e.call(0x009a_ad20, &args![this, 0xffff_0017u32, 0x5678u32]);
        assert_eq!(e.mem.u32(this.addr() + 0xc), 0x1234);
        assert_eq!(e.mem.u32(this.addr() + 0x54), 0x5678);
        e.call(0x009a_ad20, &args![this, 0xffff_0016u32, 1u32]);
        e.call(0x009a_ad20, &args![this, 0xffff_0018u32, 2u32]);
        assert_eq!(e.mem.u32(this.addr() + 0x50), 1);
        assert_eq!(e.mem.u32(this.addr() + 0x58), 2);
    }

    #[test]
    fn controller_byte_and_timer_address() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x200);
        assert!(!e.call(0x009a_ad40, &args![this]).bool());
        e.mem.set_u8(this + 0x73, 0x80);
        assert_eq!(e.call(0x009a_ad40, &args![this]).u32(), 1);
        assert_eq!(
            e.call(0x009a_ad60, &args![this, 1u32]).u32(),
            this + 0x1ac + 8
        );
    }

    #[test]
    fn copy_moves_both_parts() {
        let mut e = Engine::new();
        e.register(MEMCPY, |e, a| {
            for i in 0..a[2] {
                let byte = e.mem.u8(a[1] + i);
                e.mem.set_u8(a[0] + i, byte);
            }
            Ret::default()
        });
        let this = e.new_object::<CombatWorldStateArray>();
        let source = e.new_object::<CombatWorldStateArray>();
        for i in 0..0x17u32 {
            e.mem.set_u32(source.addr() + 4 * i, i + 1);
        }
        e.call_log = Some(vec![]);
        e.call(0x009a_ad80, &args![this, source]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls(&log, MEMCPY),
            vec![
                vec![this.addr(), source.addr(), 0x50],
                vec![this.addr() + 0x50, source.addr() + 0x50, 0xc]
            ]
        );
        assert_eq!(e.mem.u32(this.addr() + 0x58), 0x17);
    }

    #[test]
    fn is_true_tests_bits_or_equality() {
        let mut e = Engine::new();
        e.map(STATE_IS_MASK_TABLE & !0xfff, 0x2000);
        e.mem.set_u8(STATE_IS_MASK_TABLE + 8, 1);
        e.register(GET_VALUE, |e, a| e.mem.u32(a[0] + 4 * a[1]).into_ret());
        let this = e.new_object::<CombatWorldStateArray>();
        e.mem.set_u32(this.addr() + 4, 0x0000_0024);
        e.mem.set_u32(this.addr() + 8, 5);
        // State 1 is a mask: bits 2 and 5 (and 37 mod 32 = 5).
        assert!(e.call(0x009a_adc0, &args![this, 1u32, 2u32]).bool());
        assert!(e.call(0x009a_adc0, &args![this, 1u32, 37u32]).bool());
        assert!(!e.call(0x009a_adc0, &args![this, 1u32, 3u32]).bool());
        // State 2 is a plain value.
        assert!(e.call(0x009a_adc0, &args![this, 2u32, 5u32]).bool());
        assert!(!e.call(0x009a_adc0, &args![this, 2u32, 4u32]).bool());
    }

    #[test]
    fn folded_pair_store_writes_ecx_and_the_value() {
        let mut e = Engine::new();
        let out = e.mem.alloc(8);
        let back = e.call(0x00dc_b0c0, &args![0x1111u32, out, 0x2222u32]);
        assert_eq!(back.u32(), out);
        assert_eq!(e.mem.u32(out), 0x1111);
        assert_eq!(e.mem.u32(out + 4), 0x2222);
    }

    const ATTACKER: u32 = 0x0a00_0000;
    const GROUP: u32 = 0x0b00_0000;
    const WEAPON: u32 = 0x0c00_0000;
    const PROCEDURE_VTABLE: u32 = 0x0d00_0000;
    const PROCEDURE_TYPE: u32 = 0x0d00_1000;

    struct World {
        target: bool,
        /// 0 for no movement procedure.
        movement_type: u32,
        movement_state: u32,
        action_type: u32,
        location_kind: u8,
        health: f64,
        process_answer: u32,
        same_weapon: bool,
        group_word: u32,
        group_strength: u32,
        detection_byte: u32,
    }

    fn procedure(e: &mut Engine, kind: u32, state: u32, index: u32) -> u32 {
        let object = e.mem.alloc(0x20);
        e.mem.set_u32(object, PROCEDURE_VTABLE);
        e.mem.set_u32(object + 4, kind);
        e.mem.set_u32(object + 8, state);
        e.mem.set_u32(object + 0x10, index);
        object
    }

    fn object_with_slot(e: &mut Engine, vtable: u32, slot: u32, target: u32) -> u32 {
        let mut slots = vec![0u32; slot as usize / 4];
        slots.push(target);
        e.put_vtable(vtable, &slots);
        let object = e.mem.alloc(0x40);
        e.mem.set_u32(object, vtable);
        object
    }

    fn setting(e: &mut Engine, address: u32, bits: u32) {
        e.mem.set_u32(address + 4, bits);
    }

    fn update(world: World) -> (Engine, Ptr<CombatWorldStateArray>, Vec<(u32, Vec<u32>)>) {
        let mut e = Engine::new();
        for (address, value) in [
            (COVER_HEIGHT_SCALE, f64::from(0.1f32)),
            (COVER_DISTANCE_SQUARED, 1024.0),
            (TARGET_DISTANCE_SQUARED, 589_824.0),
            (PERCENT, 100.0),
        ] {
            e.map(address & !0xfff, 0x1000);
            e.set_global(address, value);
        }
        e.map(0x011c_e000, 0x2000);
        setting(&mut e, THREAT_SETTING, 1.0f32.to_bits());
        setting(&mut e, DETECTION_TIME_SETTING, 5.0f32.to_bits());
        setting(&mut e, HEALTH_PERCENT_SETTING, 50);
        e.register(SETTING_VALUE, |_, a| (a[0] + 4).into_ret());
        e.register(INT_SETTING_VALUE, |_, a| (a[0] + 4).into_ret());

        let this = e.new_object::<CombatWorldStateArray>();
        let controller = e.mem.alloc(0x100);
        let state = e.mem.alloc(0x200);
        e.mem.set_u8(state + 0x73, 0x80);
        let cover = e.mem.alloc(8);
        e.mem.set_u32(cover + 4, 5);
        // Combat items 1 and 3; the action procedure's index is 9.
        let item_one = e.mem.alloc(4);
        e.mem.set_u32(item_one, 9);
        let item_three = e.mem.alloc(4);
        e.mem.set_u32(item_three, 4);
        e.mem.set_u32(state + 0x190 + 4, item_one);
        e.mem.set_u32(state + 0x190 + 12, item_three);
        // Both usable items exist.
        e.mem.set_u32(state + 0x1a4, 0x0e00_0000);
        e.mem.set_u32(state + 0x1a8, 0x0e00_0001);
        // The state's weapon of type 3.
        let listed = if world.same_weapon {
            WEAPON
        } else {
            0x0c00_0001
        };
        e.mem.set_u32(state + 0xc + 3 * 4, listed);

        e.put_vtable(
            PROCEDURE_VTABLE,
            &[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, PROCEDURE_TYPE],
        );
        e.register(PROCEDURE_TYPE, |e, a| e.mem.u32(a[0] + 4).into_ret());
        let action = procedure(&mut e, world.action_type, 0, 9);
        let movement = if world.movement_type != 0 {
            procedure(&mut e, world.movement_type, world.movement_state, 0)
        } else {
            0
        };
        let process = object_with_slot(&mut e, 0x0d00_2000, 0x454, 0x0d00_3000);
        let answer = world.process_answer;
        e.register_double(0x0d00_3000, move |_, _| answer.into_ret());
        let base = e.mem.alloc(0x40);
        let component = object_with_slot(&mut e, 0x0d00_4000, 0x14, 0x0d00_5000);
        for offset in (0..0x40).step_by(4) {
            let word = e.mem.u32(component + offset);
            e.mem.set_u32(base + 0x30 + offset, word);
        }
        e.register(0x0d00_5000, |_, _| Ret::default());
        let location_target = e.mem.alloc(4);
        e.mem.set_u8(location_target, world.location_kind);
        let target = if world.target { 0x0f00_0000u32 } else { 0 };

        for address in [0x009a_a4b0, 0x0097_f470, 0x0097_ede0, 0x005b_5e40] {
            e.register(address, |_, _| Ret::default());
        }
        e.register_double(0x0058_6150, move |_, _| state.into_ret());
        e.register(0x0097_ae90, |_, _| ATTACKER.into_ret());
        e.register_double(0x0040_30b0, move |_, _| target.into_ret());
        e.register(GET_WORLD_LOCATION, |e, a| {
            let point = if a[0] == ATTACKER {
                [1.0f32, 2.0, 3.0]
            } else {
                [4.0, 5.0, 6.0]
            };
            for (i, value) in point.iter().enumerate() {
                e.mem.set_f32(a[1] + 4 * i as u32, *value);
            }
            e.mem.set_u32(a[1] + 12, 0x77);
            a[1].into_ret()
        });
        e.register(0x004f_b070, |_, _| GROUP.into_ret());
        e.register_double(0x004f_d3c0, move |_, _| action.into_ret());
        e.register_double(0x004f_d3e0, move |_, _| movement.into_ret());
        e.register_double(0x0097_b050, move |_, _| cover.into_ret());
        e.register(0x0093_73f0, |_, _| Ret::default());
        e.register(PROCEDURE_STATE_IS_1, |e, a| {
            u32::from(e.mem.u32(a[0] + 8) == 1).into_ret()
        });
        e.register(0x0066_6f00, |e, a| {
            u32::from(e.mem.u32(a[0] + 8) == 2).into_ret()
        });
        e.register(0x0068_15c0, |_, a| a[0].into_ret());
        e.register(0x009a_9370, |e, a| {
            for (i, value) in [10.0f32, 20.0, 30.0].iter().enumerate() {
                e.mem.set_f32(a[2] + 4 * i as u32, *value);
            }
            Ret::default()
        });
        e.register(0x0043_9ef0, |e, a| {
            for i in 0..3 {
                let value = e.mem.f32(a[0] + 4 * i) - e.mem.f32(a[2] + 4 * i);
                e.mem.set_f32(a[1] + 4 * i, value);
            }
            a[1].into_ret()
        });
        e.register(0x004a_7290, |e, a| {
            let squared: f64 = (0..3)
                .map(|i| f64::from(e.mem.f32(a[0] + 4 * i)).powi(2))
                .sum();
            st0(squared)
        });
        e.register(0x0044_edb0, |e, a| e.mem.u32(a[0] + 0x10).into_ret());
        e.register(0x0097_cfc0, |e, a| {
            e.mem.u32(a[0] + 0x190 + 4 * a[1]).into_ret()
        });
        e.register(0x0097_d110, |e, a| {
            e.mem.u32(a[0] + 0x1a4 + 4 * a[1]).into_ret()
        });
        e.register(0x0097_c150, |e, a| {
            e.mem.u32(a[0] + 0xc + 4 * a[1]).into_ret()
        });
        e.register(0x0052_7a40, |_, _| st0(100.0));
        e.register(0x0097_fd40, |_, _| Ret::default());
        e.register(0x009a_02a0, |_, _| 1u32.into_ret());
        e.register_double(0x0070_58c0, move |_, _| location_target.into_ret());
        e.register(0x0066_2a80, |_, _| 0x302u32.into_ret());
        e.register(0x0064_4790, |_, _| st0(2.0));
        e.register(0x0040_7820, |_, _| 5u32.into_ret());
        e.register(0x0097_ee90, |_, _| 1u32.into_ret());
        e.register(0x0099_8980, |_, _| 1u32.into_ret());
        e.register(0x008b_3c10, |_, _| 0x100u32.into_ret());
        e.register(0x008b_3c90, |_, _| 1u32.into_ret());
        e.register(0x0097_fae0, |_, a| a[1].into_ret());
        e.register(0x0043_5e00, |_, _| st0(10.0));
        let weapon = if world.target { WEAPON } else { 0 };
        e.register_double(0x008a_1710, move |_, _| weapon.into_ret());
        e.register(0x0052_2c80, |_, _| 3u32.into_ret());
        e.register_double(0x008d_8520, move |_, _| process.into_ret());
        e.register(0x009e_32d0, |_, _| 0x44u32.into_ret());
        e.register(0x008d_8160, |_, _| 0x55u32.into_ret());
        e.register(0x008d_7f80, |_, _| 1u32.into_ret());
        let health = world.health;
        e.register_double(0x0089_3590, move |_, _| st0(health));
        e.register(0x009a_1380, |_, _| 1u32.into_ret());
        let group_word = world.group_word;
        e.register_double(0x0045_cd60, move |_, _| group_word.into_ret());
        e.register(0x0098_a310, |_, _| 0x66u32.into_ret());
        e.register(0x0051_6bf0, |_, _| 3u32.into_ret());
        let strength = world.group_strength;
        e.register_double(0x0097_ef30, move |_, _| strength.into_ret());
        let detection = world.detection_byte;
        e.register_double(0x009a_0240, move |_, _| detection.into_ret());
        e.register_double(0x0041_81e0, move |_, _| base.into_ret());

        e.call_log = Some(vec![]);
        e.call(0x009a_a4e0, &args![this, controller]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls(&log, 0x009a_a4b0), vec![vec![this.addr()]]);
        assert_eq!(calls(&log, 0x0097_f470), vec![vec![controller]]);
        (e, this, log)
    }

    fn values(e: &Engine, this: Ptr<CombatWorldStateArray>) -> Vec<u32> {
        (0..0x17).map(|i| e.mem.u32(this.addr() + 4 * i)).collect()
    }

    #[test]
    fn update_with_a_target() {
        let (e, this, log) = update(World {
            target: true,
            movement_type: 7,
            movement_state: 0,
            action_type: 8,
            location_kind: 1,
            health: 0.25,
            process_answer: 0,
            same_weapon: true,
            group_word: 1,
            group_strength: 1,
            detection_byte: 0,
        });
        let flags = 0x20 | 1 << 0x12 | 2 | 0x80 | 0x200 | 0x10000 | 0x1000;
        let v = values(&e, this);
        assert_eq!(v[1], flags);
        assert_eq!(v[2], !flags);
        assert_eq!(v[0xe], 1);
        assert_eq!(v[8], 1);
        assert_eq!(v[3..10], [1, 1, 1, 1, 0, 1, 1]);
        assert_eq!(v[0xb], 3);
        assert_eq!(v[10], 7);
        assert_eq!(v[0xc], 0x44);
        assert_eq!(v[0xd], 0x55);
        assert_eq!(v[0xf], 1);
        assert_eq!(v[0x12], 0x50003);
        assert_eq!(v[0x13], 0x66);
        assert_eq!(v[0x14..0x17], [0x12, 0x11, 3]);
        // The movement check gets the target's location.
        let checked = calls(&log, 0x009a_02a0);
        assert_eq!(checked[0][2..], [1, 0]);
        assert_eq!(e.mem.f32(checked[0][1]), 4.0);
        assert!(calls(&log, 0x009a_9370).is_empty());
        assert!(calls(&log, 0x005b_5e40).is_empty());
    }

    #[test]
    fn update_without_a_target_measures_the_cover() {
        let (e, this, log) = update(World {
            target: false,
            movement_type: 0,
            movement_state: 0,
            action_type: 3,
            location_kind: 7,
            health: 0.25,
            process_answer: 1,
            same_weapon: true,
            group_word: 0,
            group_strength: 0,
            detection_byte: 1,
        });
        let flags = 0x20 | 0x80 | 0x100 | 0x200 | 0x10000 | 0x1000;
        let v = values(&e, this);
        assert_eq!(v[1], flags);
        assert_eq!(v[3], 1);
        assert_eq!(v[4..8], [0, 0, 0, 0]);
        assert_eq!(v[9], 0);
        assert_eq!(v[0xb], 6);
        assert_eq!(v[10], 6);
        assert_eq!(v[0x12], 0x60003);
        assert_eq!(v[0x13], 0);
        assert_eq!(v[0x14..0x17], [0x12, 0x11, 6]);
        let cover = calls(&log, 0x009a_9370);
        assert_eq!(cover.len(), 1);
        assert_eq!(cover[0][3], 0);
        // (10, 20, 30) - (1, 2, 3), the height scaled by 0.1.
        let offset = calls(&log, 0x004a_7290)[0][0];
        assert_eq!(e.mem.f32(offset), 9.0);
        assert_eq!(
            e.mem.u32(offset + 8),
            x87_store32(x87_mul(27.0, f64::from(0.1f32)))
        );
        assert_eq!(
            calls(&log, 0x005b_5e40),
            vec![vec![UNSUPPORTED_TARGET_MESSAGE, 7]]
        );
        assert!(calls(&log, 0x009a_02a0).is_empty());
    }

    #[test]
    fn update_movement_type_nine_and_the_health_threshold() {
        let (e, this, _) = update(World {
            target: true,
            movement_type: 9,
            movement_state: 0,
            action_type: 8,
            location_kind: 3,
            health: 0.75,
            process_answer: 1,
            same_weapon: false,
            group_word: 1,
            group_strength: 0,
            detection_byte: 0,
        });
        let v = values(&e, this);
        // Cover measured (type 9 is not 7): (10,20,30) - (1,2,3) is near.
        let flags = 0x20 | 1 << 0x12 | 2 | 0x80 | 0x100 | 0x200 | 0x1000 | 0x400;
        assert_eq!(v[1], flags);
        // The weapon is not the state's weapon of its type.
        assert_eq!(v[0xb], 7);
        assert_eq!(v[10], 7);
        // 75 percent is not below 50: the second usable item applies.
        assert_eq!(v[0x15], 0x12);

        let (e, this, _) = update(World {
            target: true,
            movement_type: 7,
            movement_state: 1,
            action_type: 8,
            location_kind: 0,
            health: 0.25,
            process_answer: 1,
            same_weapon: true,
            group_word: 1,
            group_strength: 0,
            detection_byte: 0,
        });
        // Movement type 7 in state 1: no cover bit, no measurement.
        let v = values(&e, this);
        assert_eq!(v[1], 1 << 0x12 | 2 | 0x100 | 0x200 | 0x10000 | 0x1000);
        assert_eq!(v[10], 3);
    }
}

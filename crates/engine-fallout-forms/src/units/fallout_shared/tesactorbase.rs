//! `fallout shared/tesactorbase.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `00401170`: thiscall getter, the byte at `this + 4`.
const GET_TYPE_BYTE: u32 = 0x0040_1170;
/// `00461580`: thiscall (`RET 4`), true when `this[4] & mask` is non-zero.
const TEST_FLAGS: u32 = 0x0046_1580;
/// The type byte the function accepts.
const ACCEPTED_TYPE: u32 = 0x2b;
/// The offset, from `this`, of the object whose flags are tested.
const FLAGS_OBJECT_OFFSET: u32 = 0x30;
/// The mask passed to the flag test.
const FLAG_MASK: u32 = 0x20;

// Translated from 005f0c40 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (the map places it in `tesactorbase.cpp`). `__fastcall`: false
/// unless `00401170(this)` returns `0x2b`; then whether `00461580` on
/// `this + 0x30` with mask `0x20` is true.
pub fn fn_005f0c40(e: &mut Engine, this: Ptr) -> bool {
    if e.call(GET_TYPE_BYTE, &args![this]).u32() != ACCEPTED_TYPE {
        return false;
    }
    let flags_object = this.addr() + FLAGS_OBJECT_OFFSET;
    e.call(TEST_FLAGS, &args![flags_object, FLAG_MASK]).bool()
}

/// The form type byte `TESActorBase::IsImmobile` accepts.
const IMMOBILE_TYPE: u32 = 0x2b;
/// The mask `TESActorBase::IsImmobile` tests.
const IMMOBILE_FLAG_MASK: u32 = 0x0080_0000;
/// The type byte `TESActorBase::GetSex` accepts.
const SEX_TYPE: u32 = 0x2a;
/// The mask `TESActorBase::GetSex` tests.
const SEX_FLAG_MASK: u32 = 1;

// Translated from 005f0c80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESActorBase::IsImmobile` (Xbox PDB): `__fastcall`. False unless `00401170(this)` returns
/// `0x2b`; then whether `00461580` on `this + 0x30` with mask `0x800000` is true.
pub fn tes_actor_base_is_immobile(e: &mut Engine, this: Ptr) -> bool {
    if e.call(GET_TYPE_BYTE, &args![this]).u32() != IMMOBILE_TYPE {
        return false;
    }
    let flags_object = this.addr().wrapping_add(FLAGS_OBJECT_OFFSET);
    e.call(TEST_FLAGS, &args![flags_object, IMMOBILE_FLAG_MASK])
        .bool()
}

// Translated from 005f0cc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESActorBase::GetSex` (Xbox PDB): `__fastcall`. `-1` unless `00401170(this)` returns
/// `0x2a`; then `1` when `00461580` on `this + 0x30` with mask `1` is true, else `0`.
pub fn tes_actor_base_get_sex(e: &mut Engine, this: Ptr) -> i32 {
    if e.call(GET_TYPE_BYTE, &args![this]).u32() != SEX_TYPE {
        return -1;
    }
    let flags_object = this.addr().wrapping_add(FLAGS_OBJECT_OFFSET);
    i32::from(
        e.call(TEST_FLAGS, &args![flags_object, SEX_FLAG_MASK])
            .bool(),
    )
}

/// `00480160` (`TESAttributes`, thiscall, `RET 4`: index): the byte at
/// `this + index - 1` (in `AL`).
const ATTRIBUTE_GET: u32 = 0x0048_0160;
/// `TESAttributes::SetAttributeValue` (Xbox PDB, `00480180`, thiscall,
/// `RET 0xC`: index, value byte, flag byte).
const ATTRIBUTE_SET: u32 = 0x0048_0180;
/// `TESActorBaseData::SetFlagBit` (Xbox PDB, `0047dd50`, thiscall, `RET 0xC`:
/// mask, value byte, flag byte).
const DATA_SET_FLAG_BIT: u32 = 0x0047_dd50;
/// The player singleton pointer (`011dea3c`) and `004181e0` on it (the
/// player's base form).
const PLAYER: u32 = 0x011d_ea3c;
const PLAYER_BASE_FORM: u32 = 0x0041_81e0;
/// The singleton at `011c3f2c` (`TES`) and the save/load game singleton at
/// `011ddf38`.
const TES: u32 = 0x011c_3f2c;
const SAVE_LOAD_GAME: u32 = 0x011d_df38;
/// `__RTDynamicCast` (`00ec43fb`, cdecl, five words) and the RTTI type
/// descriptors: `TESForm` (`01183028`), `TESAmmo` (`011840c4`),
/// `TESObjectWEAP` (`01183998`), `TESObjectCLOT` (`01186fcc`),
/// `TESObjectARMO` (`01183a34`).
const RT_DYNAMIC_CAST: u32 = 0x00ec_43fb;
const TYPE_TES_FORM: u32 = 0x0118_3028;
const TYPE_TES_AMMO: u32 = 0x0118_40c4;
const TYPE_TES_OBJECT_WEAP: u32 = 0x0118_3998;
const TYPE_TES_OBJECT_CLOT: u32 = 0x0118_6fcc;
const TYPE_TES_OBJECT_ARMO: u32 = 0x0118_3a34;
/// `TESHealthForm::GetFormHealth` (Xbox PDB, `004873d0`, cdecl).
const GET_FORM_HEALTH: u32 = 0x0048_73d0;
/// `_ftol2` (`00ec62c0`): ST0 to an integer.
const FTOL2: u32 = 0x00ec_62c0;

// Translated from 005f11f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Attribute index 5 of a `TESAttributes` (`00480160`: the byte at
/// `this + 4`); `EAX` as the callee leaves it.
pub fn fn_005f11f0(e: &mut Engine, this: Ptr) -> u32 {
    e.call(ATTRIBUTE_GET, &args![this, 5u32]).u32()
}

// Translated from 005f1210 (decompiled, FalloutNV.exe 1.4.0.525)
/// Attribute index 6 (`00480160` with 6).
pub fn fn_005f1210(e: &mut Engine, this: Ptr) -> u32 {
    e.call(ATTRIBUTE_GET, &args![this, 6u32]).u32()
}

// Translated from 005f1230 (decompiled, FalloutNV.exe 1.4.0.525)
/// Attribute index 7 (`00480160` with 7).
pub fn fn_005f1230(e: &mut Engine, this: Ptr) -> u32 {
    e.call(ATTRIBUTE_GET, &args![this, 7u32]).u32()
}

// Translated from 005f1250 (decompiled, FalloutNV.exe 1.4.0.525)
/// Attribute index 8 (`00480160` with 8).
pub fn fn_005f1250(e: &mut Engine, this: Ptr) -> u32 {
    e.call(ATTRIBUTE_GET, &args![this, 8u32]).u32()
}

// Translated from 005f1270 (decompiled, FalloutNV.exe 1.4.0.525)
/// Attribute index 9 (`00480160` with 9).
pub fn fn_005f1270(e: &mut Engine, this: Ptr) -> u32 {
    e.call(ATTRIBUTE_GET, &args![this, 9u32]).u32()
}

// Translated from 005f1290 (decompiled, FalloutNV.exe 1.4.0.525)
/// Attribute index 0xA (`00480160` with 0xA).
pub fn fn_005f1290(e: &mut Engine, this: Ptr) -> u32 {
    e.call(ATTRIBUTE_GET, &args![this, 0xau32]).u32()
}

// Translated from 005f12b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Attribute index 0xB (`00480160` with 0xB).
pub fn fn_005f12b0(e: &mut Engine, this: Ptr) -> u32 {
    e.call(ATTRIBUTE_GET, &args![this, 0xbu32]).u32()
}

// Translated from 005f1570 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets attribute index 0xB of a `TESAttributes` to `value` with the flag 1
/// (`TESAttributes::SetAttributeValue`, `RET 4`).
pub fn fn_005f1570(e: &mut Engine, this: Ptr, value: u8) {
    e.call(ATTRIBUTE_SET, &args![this, 0xbu32, u32::from(value), 1u32]);
}

// Translated from 005f0b00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESActorBase::GetHealth` (Xbox PDB): vtable slot 0x10 of the
/// `TESHealthForm` part at +0xB0 (`GetFormHealth`).
pub fn tes_actor_base_get_health(e: &mut Engine, this: Ptr) -> u32 {
    e.vcall(this.addr() + 0xb0, 0x10, &[]).u32()
}

// Translated from 005f0d00 (decompiled, FalloutNV.exe 1.4.0.525)
/// False unless the form type (`00401170`) is 0x2A; then flag 0x10 of the
/// actor base data at +0x30 (`00461580`).
pub fn fn_005f0d00(e: &mut Engine, this: Ptr) -> bool {
    if e.call(GET_TYPE_BYTE, &args![this]).u32() != SEX_TYPE {
        return false;
    }
    e.call(
        TEST_FLAGS,
        &args![this.addr() + FLAGS_OBJECT_OFFSET, 0x10u32],
    )
    .bool()
}

// Translated from 005f0d30 (decompiled, FalloutNV.exe 1.4.0.525)
/// For form type 0x2A only: sets flag 0x10 of the actor base data to
/// `value` (`TESActorBaseData::SetFlagBit` with the flag byte 1; `RET 4`).
pub fn fn_005f0d30(e: &mut Engine, this: Ptr, value: u8) {
    if e.call(GET_TYPE_BYTE, &args![this]).u32() == SEX_TYPE {
        e.call(
            DATA_SET_FLAG_BIT,
            &args![
                this.addr() + FLAGS_OBJECT_OFFSET,
                0x10u32,
                u32::from(value),
                1u32
            ],
        );
    }
}

// Translated from 005f0f80 (decompiled, FalloutNV.exe 1.4.0.525)
/// The word at `011caff0` when it is nonzero and `this` is the player's base
/// form (`004181e0` on the player), otherwise the word at `011cafec`.
pub fn fn_005f0f80(e: &mut Engine, this: Ptr) -> u32 {
    if e.global::<u32>(0x011c_aff0) != 0 {
        let player = e.global::<u32>(PLAYER);
        if this.addr() == e.call(PLAYER_BASE_FORM, &args![player]).u32() {
            return e.global::<u32>(0x011c_aff0);
        }
    }
    e.global::<u32>(0x011c_afec)
}

// Translated from 00565210 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESActorBase::SetStartsDead` (Xbox PDB, `RET 4`): the form's flags
/// (`0044ddc0`) with bit 0x80000 set or cleared, written back (`00403550`).
pub fn tes_actor_base_set_starts_dead(e: &mut Engine, this: Ptr, value: u8) {
    let mut flags = e.call(0x0044_ddc0, &args![this]).u32();
    if value != 0 {
        flags |= 0x0008_0000;
    } else {
        flags &= 0xfff7_ffff;
    }
    e.call(0x0040_3550, &args![this, flags]);
}

// Translated from 005f0b50 (decompiled, FalloutNV.exe 1.4.0.525)
/// True for a form type other than 0x2B; for 0x2B, whether flag 0x10 or
/// flag 0x1 of the actor base data is set (`00461580`, 0x10 first).
pub fn fn_005f0b50(e: &mut Engine, this: Ptr) -> bool {
    if e.call(GET_TYPE_BYTE, &args![this]).u32() != ACCEPTED_TYPE {
        return true;
    }
    let data = this.addr() + FLAGS_OBJECT_OFFSET;
    e.call(TEST_FLAGS, &args![data, 0x10u32]).bool()
        || e.call(TEST_FLAGS, &args![data, 1u32]).bool()
}

// Translated from 005f0bf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// True for a form type other than 0x2B; for 0x2B, whether flag 0x40 or
/// flag 0x1 of the actor base data is set (0x40 first).
pub fn fn_005f0bf0(e: &mut Engine, this: Ptr) -> bool {
    if e.call(GET_TYPE_BYTE, &args![this]).u32() != ACCEPTED_TYPE {
        return true;
    }
    let data = this.addr() + FLAGS_OBJECT_OFFSET;
    e.call(TEST_FLAGS, &args![data, 0x40u32]).bool()
        || e.call(TEST_FLAGS, &args![data, 1u32]).bool()
}

// Translated from 005f1630 (decompiled, FalloutNV.exe 1.4.0.525)
/// `RET 4` (`value`): when `00469860` on the `TES` singleton says yes for the
/// form's id (`0084e3a0`), and the save/load game's `0084a620` (with `this`
/// and 0) says yes too, passes `value` to `0047e340` on the actor base data;
/// when `00469860` says no, to `00484b60` on the form.
pub fn fn_005f1630(e: &mut Engine, this: Ptr, value: u32) {
    let id = e.call(0x0084_e3a0, &args![this]).u32();
    let tes = e.global::<u32>(TES);
    if e.call(0x0046_9860, &args![tes, id]).bool() {
        let game = e.global::<u32>(SAVE_LOAD_GAME);
        if e.call(0x0084_a620, &args![game, this, 0u32]).bool() {
            e.call(
                0x0047_e340,
                &args![this.addr() + FLAGS_OBJECT_OFFSET, value],
            );
        }
    } else {
        e.call(0x0048_4b60, &args![this, value]);
    }
}

// Translated from 005f1690 (decompiled, FalloutNV.exe 1.4.0.525)
/// As `005f1630` with `0084a740` (with `this`), `0047e360` on the actor base
/// data and `00484bc0` on the form; here a no from `0084a740` also goes to
/// `00484bc0`.
pub fn fn_005f1690(e: &mut Engine, this: Ptr, value: u32) {
    let id = e.call(0x0084_e3a0, &args![this]).u32();
    let tes = e.global::<u32>(TES);
    if e.call(0x0046_9860, &args![tes, id]).bool() {
        let game = e.global::<u32>(SAVE_LOAD_GAME);
        if e.call(0x0084_a740, &args![game, this]).bool() {
            e.call(
                0x0047_e360,
                &args![this.addr() + FLAGS_OBJECT_OFFSET, value],
            );
            return;
        }
    }
    e.call(0x0048_4bc0, &args![this, value]);
}

// Translated from 005f0fb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The actor value `index` of an actor base, read through its
/// `ActorValueOwner` part (`this`, at +0x100 of the actor base; `RET 4`):
/// 0..4 from the `TESAIForm` part (`this - 0x70`: `004f1540`, `004f6de0`,
/// `005bb4d0`, `0047ef30`, `0062f2f0`, bytes); 5..0xB the attributes
/// (`005f11f0`..`005f12b0` on `this - 0x48`, bytes); 0x10 the health
/// (`GetHealth` on `this - 0x100`); 0x15 `008f21d0` and 0x16 vtable slot 0x60
/// of the actor base data (`this - 0xD0`, 16-bit); 0x17 slot 0x64 of it
/// (a float through `_ftol2`); 0x39 `0047ee30` on the AI form (byte); any
/// other index 0.
pub fn fn_005f0fb0(e: &mut Engine, this: Ptr, index: u32) -> u32 {
    let base = this.addr();
    let ai = base.wrapping_sub(0x70);
    let attributes = Ptr::new(base.wrapping_sub(0x48));
    let data = base.wrapping_sub(0xd0);
    match index {
        0 => u32::from(e.call(0x004f_1540, &args![ai]).u8()),
        1 => u32::from(e.call(0x004f_6de0, &args![ai]).u8()),
        2 => u32::from(e.call(0x005b_b4d0, &args![ai]).u8()),
        3 => u32::from(e.call(0x0047_ef30, &args![ai]).u8()),
        4 => u32::from(e.call(0x0062_f2f0, &args![ai]).u8()),
        5 => fn_005f11f0(e, attributes) & 0xff,
        6 => fn_005f1210(e, attributes) & 0xff,
        7 => fn_005f1230(e, attributes) & 0xff,
        8 => fn_005f1250(e, attributes) & 0xff,
        9 => fn_005f1270(e, attributes) & 0xff,
        0xa => fn_005f1290(e, attributes) & 0xff,
        0xb => fn_005f12b0(e, attributes) & 0xff,
        0x10 => tes_actor_base_get_health(e, Ptr::new(base.wrapping_sub(0x100))),
        0x15 => u32::from(e.call(0x008f_21d0, &args![data]).u16()),
        0x16 => u32::from(e.vcall(data, 0x60, &[]).u16()),
        0x17 => {
            let value = e.vcall(data, 0x64, &[]).f64();
            e.call(FTOL2, &args![value]).u32()
        }
        0x39 => u32::from(e.call(0x0047_ee30, &args![ai]).u8()),
        _ => 0,
    }
}

// Translated from 005f12d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the actor value `index` of an actor base (`RET 8`): 0..3 through the
/// `TESAIForm` part at +0x90 (`0047ee00`, `0047eed0`, `0047ef00`, `0047ef50`
/// with the low byte of `value` and 1), 4 through `0047ef80` (the byte);
/// 5..0xB the attributes at +0xB8 (`005f14b0`, `005f14d0`, `005f14f0`,
/// `005f1510`, `005f1530`, `005f1550`, `005f1570`, the byte); 0x10 the health
/// (`005f0b30`, the word); 0x15 `0047e270` on the actor base data (the low
/// 16 bits); 0x17 `0047e1c0` on it with `value` as a float (`FILD`/`FSTP`);
/// any other index nothing.
pub fn fn_005f12d0(e: &mut Engine, this: Ptr, index: u32, value: u32) {
    let base = this.addr();
    let ai = base + 0x90;
    let attributes = base + 0xb8;
    let byte = value & 0xff;
    match index {
        0 => {
            e.call(0x0047_ee00, &args![ai, byte, 1u32]);
        }
        1 => {
            e.call(0x0047_eed0, &args![ai, byte, 1u32]);
        }
        2 => {
            e.call(0x0047_ef00, &args![ai, byte, 1u32]);
        }
        3 => {
            e.call(0x0047_ef50, &args![ai, byte, 1u32]);
        }
        4 => {
            e.call(0x0047_ef80, &args![ai, byte]);
        }
        5 => {
            e.call(0x005f_14b0, &args![attributes, byte]);
        }
        6 => {
            e.call(0x005f_14d0, &args![attributes, byte]);
        }
        7 => {
            e.call(0x005f_14f0, &args![attributes, byte]);
        }
        8 => {
            e.call(0x005f_1510, &args![attributes, byte]);
        }
        9 => {
            e.call(0x005f_1530, &args![attributes, byte]);
        }
        0xa => {
            e.call(0x005f_1550, &args![attributes, byte]);
        }
        0xb => fn_005f1570(e, Ptr::new(attributes), value as u8),
        0x10 => {
            e.call(0x005f_0b30, &args![this, value]);
        }
        0x15 => {
            e.call(
                0x0047_e270,
                &args![base + FLAGS_OBJECT_OFFSET, value & 0xffff],
            );
        }
        0x17 => {
            let bits = x87_store32(f64::from(value as i32));
            e.call(0x0047_e1c0, &args![base + FLAGS_OBJECT_OFFSET, bits]);
        }
        _ => {}
    }
}

// Translated from 005f0d60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESActorBase::GetDesirability` (Xbox PDB, `RET 4`: the item form; the
/// result in ST0): 0 for a null form; by its form type:
/// - 0x18 (`TESObjectARMO`): with health (`GetFormHealth`), `004be180`
///   plus `CombatFormulas::CalcArmorRating` of the truncated low 16 bits of
///   `004be060` with 1.0 (`FADD` of the double, then the float);
/// - 0x1A (`TESObjectCLOT`): `00647cf0` of the word at +4 of +0x58;
/// - 0x1E: the constant at `01016410`;
/// - 0x28 (`TESObjectWEAP`): with health, `CombatFormulas::CalcWeaponDamage`
///   for the actor value owner part (+0x100; 0 for a null `this`), the
///   weapon, 1.0, 1.0 and four zeros;
/// - 0x29 (`TESAmmo`): 1.0;
///
/// anything else (or a failed cast, or no health) 0.
pub fn tes_actor_base_get_desirability(e: &mut Engine, this: Ptr, form: u32) -> f32 {
    let mut result = 0.0f32;
    if form == 0 {
        return result;
    }
    let kind = e.call(GET_TYPE_BYTE, &args![form]).u32().wrapping_sub(0x18);
    let cast = |e: &mut Engine, target: u32| {
        e.call(
            RT_DYNAMIC_CAST,
            &args![form, 0u32, TYPE_TES_FORM, target, 0u32],
        )
        .u32()
    };
    match kind {
        0x11 => {
            if cast(e, TYPE_TES_AMMO) != 0 {
                result = 1.0;
            }
        }
        0x10 => {
            let weapon = cast(e, TYPE_TES_OBJECT_WEAP);
            if weapon != 0 && e.call(GET_FORM_HEALTH, &args![weapon]).u32() != 0 {
                let owner = if this.addr() == 0 {
                    0
                } else {
                    this.addr() + 0x100
                };
                let one = 1.0f32.to_bits();
                let damage = e
                    .call(
                        0x0064_4ce0,
                        &args![owner, weapon, one, one, 0u32, 0u32, 0u32, 0u32],
                    )
                    .f64();
                result = f32::from_bits(x87_store32(damage));
            }
        }
        2 => {
            let clothing = cast(e, TYPE_TES_OBJECT_CLOT);
            if clothing != 0 {
                let value = e.call(0x0072_6070, &args![clothing + 0x58]).u32();
                let rating = e.call(0x0064_7cf0, &args![value]).f64();
                result = f32::from_bits(x87_store32(rating));
            }
        }
        0 => {
            let armor = cast(e, TYPE_TES_OBJECT_ARMO);
            if armor != 0 && e.call(GET_FORM_HEALTH, &args![armor]).u32() != 0 {
                // CalcArmorRating's arguments are pushed right to left: the
                // 1.0 first, then the truncated `004be060` result.
                let one = 1.0f32.to_bits();
                let base_rating = e.call(0x004b_e060, &args![armor]).f64();
                let truncated = crate::x87::fistp_chop(base_rating) as u32 & 0xffff;
                let rating = e.call(0x0064_6360, &args![truncated, one]).f64();
                let extra = e.call(0x004b_e180, &args![armor]).f64();
                result = f32::from_bits(x87_store32(x87_add(extra, rating)));
            }
        }
        6 => {
            result = x87_quiet(f32::from_bits(e.global::<u32>(0x0101_6410)));
        }
        _ => {}
    }
    result
}

// Translated from 005f1590 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESActorBase::ShouldRunInitDefaultWorn` (Xbox PDB, `RET 4`: the
/// reference): true unless the save/load game has a change for the
/// reference (`BGSSaveLoadGame::GetChange`) under the change flags made by
/// `008c71b0`: 0x0800_0020 when `0042ce10` on the save/load game is set and
/// `00576d30` on the reference says yes; 0x400 when it is clear and the
/// reference's vtable slot 0x22C (with 0) says yes.
pub fn tes_actor_base_should_run_init_default_worn(
    e: &mut Engine,
    _this: Ptr,
    reference: u32,
) -> bool {
    let mut result = true;
    let game = e.global::<u32>(SAVE_LOAD_GAME);
    let flags = if e.call(0x0042_ce10, &args![game]).bool() {
        if e.call(0x0057_6d30, &args![reference]).bool() {
            Some(0x0800_0020u32)
        } else {
            None
        }
    } else if e.vcall(reference, 0x22c, &args![0u32]).bool() {
        Some(0x400)
    } else {
        None
    };
    if let Some(flags) = flags {
        // The change flags object is built in the argument slot and passed
        // by value.
        let word = e.with_stack(4, |e, slot| {
            e.call(0x008c_71b0, &args![slot, flags]);
            e.mem.u32(slot.addr())
        });
        let game = e.global::<u32>(SAVE_LOAD_GAME);
        if e.call(0x0084_a6d0, &args![game, reference, word]).bool() {
            result = false;
        }
    }
    result
}

// Translated from 005f1f30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Saves the actor base to the save buffer (`RET 4`): the form (`00484d60`),
/// the actor base data (+0x30, `0047ea30`), the spell list (+0x7C,
/// `0048e040`), the attributes (+0xB8, `00480270`), the AI form (+0x90,
/// `0047f940`), then, when the buffer's flag 0x20 is set (`00428110`,
/// `004280f0`), the name string of +0xD0 (`00408da0`) with
/// `BGSSaveGameBuffer::SaveString` (and 0).
pub fn fn_005f1f30(e: &mut Engine, this: Ptr, buffer: u32) {
    let base = this.addr();
    e.call(0x0048_4d60, &args![this, buffer]);
    e.call(0x0047_ea30, &args![base + 0x30, buffer]);
    e.call(0x0048_e040, &args![base + 0x7c, buffer]);
    e.call(0x0048_0270, &args![base + 0xb8, buffer]);
    e.call(0x0047_f940, &args![base + 0x90, buffer]);
    let saves = e.with_stack(4, |e, local| {
        let flags = e.call(0x0042_8110, &args![buffer, local]).u32();
        e.call(0x0042_80f0, &args![flags, 0x20u32]).bool()
    });
    if saves {
        let name = e.call(0x0040_8da0, &args![base + 0xd0]).u32();
        e.call(0x0086_5e70, &args![buffer, name, 0u32]);
    }
}

/// `006ecd40` (`ECX` = object, one word; `RET 4`): stores the word at the
/// object's +4 (the engine map's name,
/// `CombatProcedureAttackMelee::Initialize`, is a folded body).
const SET_WORD_4: u32 = 0x006e_cd40;
/// `0044ddc0` (`ECX` = form): its flags word (+8).
const FORM_FLAGS: u32 = 0x0044_ddc0;
/// The save/load game object (`011de45c`, a pointer).
const SAVE_LOAD_GAME_OBJECT: u32 = 0x011d_e45c;
/// `TESSaveLoadGame::UseSaveGameBlocks` (Xbox PDB name of `00862110`;
/// `ECX` = save/load game).
const USE_SAVE_GAME_BLOCKS: u32 = 0x0086_2110;
/// `008df040` (`ECX` = save/load game): the save's version byte.
const SAVE_VERSION: u32 = 0x008d_f040;
/// `00825c00` (`ECX` = save/load game): the current position in the save
/// data.
const SAVE_POSITION: u32 = 0x0082_5c00;
/// `008579b0` (`ECX` = save/load game; buffer, size; `RET 8`): writes bytes.
const SAVE_WRITE: u32 = 0x0085_79b0;
/// `008579e0` (`ECX` = save/load game; buffer, size; `RET 8`): reads bytes.
const LOAD_READ: u32 = 0x0085_79e0;
/// `00408d60` (`ECX` = setting): the address of the setting's value.
const SETTING_VALUE: u32 = 0x0040_8d60;
/// The boolean setting whose value turns on the save size reports
/// (`011de4e8`).
const REPORT_SAVE_SIZES: u32 = 0x011d_e4e8;
/// `004fd3e0` (`ECX` = save/load game): the record of the form being
/// saved (form id at +0, flags at +5, a byte at +9), or 0.
const SAVING_RECORD: u32 = 0x004f_d3e0;
/// `004fd3c0` (`ECX` = save/load game): the record of the form being
/// loaded, or 0.
const LOADING_RECORD: u32 = 0x004f_d3c0;
/// `004839c0` (cdecl: form id): the form with that id.
const LOOKUP_FORM: u32 = 0x0048_39c0;
/// `Error` (engine map name of `0040fbe0`; cdecl, a format and its
/// arguments).
const ERROR: u32 = 0x0040_fbe0;
/// `005b5e40` (cdecl, a format and its arguments): writes a warning line.
const LOG_WARNING: u32 = 0x005b_5e40;
/// The source file name the save size reports pass (`01047628`).
const SOURCE_FILE: u32 = 0x0104_7628;
/// The block tag written before an actor base's save data (`KOLB`).
const BLOCK_TAG: u32 = 0x424c_4f4b;
/// `00408da0` (`ECX` = string): its characters.
const STRING_TEXT: u32 = 0x0040_8da0;
/// `strlen` wrapper (`0044a670`, cdecl).
const STRING_LENGTH: u32 = 0x0044_a670;
/// `TESForm::SaveGameDataOLD` (Xbox PDB name of `00484ce0`; `ECX` = form;
/// buffer, size).
const SAVE_FORM_DATA: u32 = 0x0048_4ce0;
/// `00484d00` (`ECX` = form; buffer, size; `RET 8`): reads form data.
const LOAD_FORM_DATA: u32 = 0x0048_4d00;
/// `TESSoundFile::SetSoundFile` (engine map name of `00489100`; `ECX` =
/// the name string at +0xD0, the characters; `RET 4`).
const SET_NAME: u32 = 0x0048_9100;
/// The vtable `005f2150` stores (`01047670`, slot 0 `005f2170`).
const RESET_3D_ITERATOR_VTABLE: u32 = 0x0104_7670;
/// The list `005f20a0` walks with its iterator (`011e0e80`).
const ITERATED_LIST: u32 = 0x011e_0e80;
/// `0096bbc0` (`ECX` = the list at `011e0e80`; the iterator; `RET 4`).
const ITERATE: u32 = 0x0096_bbc0;
/// `008d8520` (`ECX` = reference): its process (the engine map's name,
/// `MiddleHighProcess::GetSavedAcquireObject`, is a folded body).
const REFERENCE_PROCESS: u32 = 0x008d_8520;
/// `007af430` (`ECX` = reference): its base object.
const REFERENCE_BASE_OBJECT: u32 = 0x007a_f430;

/// Sets attribute `index` to the byte `value` with the flag 1
/// (`TESAttributes::SetAttributeValue`).
fn set_attribute(e: &mut Engine, this: Ptr, index: u32, value: u8) {
    e.call(ATTRIBUTE_SET, &args![this, index, u32::from(value), 1u32]);
}

// Translated from 005f0b30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the health (`RET 4`): the word goes to the health form at +0xB0
/// (`006ecd40`, which stores it at its +4).
pub fn fn_005f0b30(e: &mut Engine, this: Ptr, health: u32) {
    e.call(SET_WORD_4, &args![this.addr() + 0xb0, health]);
}

// Translated from 005f0ba0 (decompiled, FalloutNV.exe 1.4.0.525)
/// False when `005f0b50` is false; otherwise true unless the form type is
/// 0x2B and flag 0x40000 of the actor base data (+0x30, `00461580`) is set.
pub fn fn_005f0ba0(e: &mut Engine, this: Ptr) -> bool {
    if !fn_005f0b50(e, this) {
        return false;
    }
    if e.call(GET_TYPE_BYTE, &args![this]).u32() == ACCEPTED_TYPE
        && e.call(
            TEST_FLAGS,
            &args![this.addr() + FLAGS_OBJECT_OFFSET, 0x0004_0000u32],
        )
        .bool()
    {
        return false;
    }
    true
}

// Translated from 005f0f50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether flag 0x80000 of the form's flags (`0044ddc0`) is set (the flag
/// `TESActorBase::SetStartsDead` writes).
pub fn fn_005f0f50(e: &mut Engine, this: Ptr) -> bool {
    e.call(FORM_FLAGS, &args![this]).u32() & 0x0008_0000 != 0
}

// Translated from 005f14b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets attribute index 5 of a `TESAttributes` to `value` with the flag 1
/// (`TESAttributes::SetAttributeValue`, `RET 4`).
pub fn fn_005f14b0(e: &mut Engine, this: Ptr, value: u8) {
    set_attribute(e, this, 5, value);
}

// Translated from 005f14d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets attribute index 6 (as `005f14b0`).
pub fn fn_005f14d0(e: &mut Engine, this: Ptr, value: u8) {
    set_attribute(e, this, 6, value);
}

// Translated from 005f14f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets attribute index 7 (as `005f14b0`).
pub fn fn_005f14f0(e: &mut Engine, this: Ptr, value: u8) {
    set_attribute(e, this, 7, value);
}

// Translated from 005f1510 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets attribute index 8 (as `005f14b0`).
pub fn fn_005f1510(e: &mut Engine, this: Ptr, value: u8) {
    set_attribute(e, this, 8, value);
}

// Translated from 005f1530 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets attribute index 9 (as `005f14b0`).
pub fn fn_005f1530(e: &mut Engine, this: Ptr, value: u8) {
    set_attribute(e, this, 9, value);
}

// Translated from 005f1550 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets attribute index 0xA (as `005f14b0`).
pub fn fn_005f1550(e: &mut Engine, this: Ptr, value: u8) {
    set_attribute(e, this, 0xa, value);
}

/// The save size reports' check: whether the boolean setting at
/// `011de4e8` is set.
fn reports_save_sizes(e: &mut Engine) -> bool {
    let value = e.call(SETTING_VALUE, &args![REPORT_SAVE_SIZES]).u32();
    e.mem.u8(value) != 0
}

/// The form of a save record (`004839c0` on its form id) and its name
/// (virtual slot 0x130 of the form).
fn record_form_name(e: &mut Engine, record: u32) -> u32 {
    let id = e.mem.u32(record);
    let form = e.call(LOOKUP_FORM, &args![id]).u32();
    e.vcall(form, 0x130, &[]).u32()
}

// Translated from 005f16f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The actor base's save size for the change `flags` (`RET 4`; a `u16`
/// sum): 6 when the save uses blocks (`UseSaveGameBlocks`), the form's
/// (`00484bf0`) for a save version of 0x6D or more, the attributes'
/// (+0xB8 `004801e0`), the actor base data's (+0x30 `0047e380`), the spell
/// list's (+0x7C `TESSpellList::GetSaveSize`) and the AI form's (+0x90
/// `TESAIForm::GetSaveSize`), and with flag 0x20 the length of the name at
/// +0xD0 plus 1. When the setting at `011de4e8` is set it reports the size
/// (`Error`, line 0x226) with the record of the form being saved when
/// there is one. The 4 bytes for the health are under a mask the compiler
/// folded to 0 (`AND EAX,0`): never added.
// The exe tests a mask the compiler folded to 0; the branch is kept as written.
#[allow(clippy::bad_bit_mask, clippy::erasing_op)]
pub fn fn_005f16f0(e: &mut Engine, this: Ptr, flags: u32) -> u16 {
    let base = this.addr();
    let mut size: u16 = 0;
    let start = size;
    let game = e.global::<u32>(SAVE_LOAD_GAME_OBJECT);
    if e.call(USE_SAVE_GAME_BLOCKS, &args![game]).u8() != 0 {
        size = size.wrapping_add(4);
        size = size.wrapping_add(2);
    }
    let game = e.global::<u32>(SAVE_LOAD_GAME_OBJECT);
    if e.call(SAVE_VERSION, &args![game]).u8() >= 0x6d {
        let part = e.call(0x0048_4bf0, &args![this, flags]).u16();
        size = size.wrapping_add(part);
    }
    for (offset, getter) in [
        (0xb8, 0x0048_01e0),
        (0x30, 0x0047_e380),
        (0x7c, 0x0048_d8f0),
        (0x90, 0x0047_f7e0),
    ] {
        let part = e.call(getter, &args![base + offset, flags]).u16();
        size = size.wrapping_add(part);
    }
    // Folded mask (always false in the exe): the health's 4 bytes.
    if flags & 0 != 0 {
        size = size.wrapping_add(4);
    }
    if flags & 0x20 != 0 {
        size = size.wrapping_add(1);
        let text = e.call(STRING_TEXT, &args![base + 0xd0]).u32();
        let length = e.call(STRING_LENGTH, &args![text]).u32();
        size = size.wrapping_add(length as u16);
    }
    if reports_save_sizes(e) {
        let game = e.global::<u32>(SAVE_LOAD_GAME_OBJECT);
        let record = e.call(SAVING_RECORD, &args![game]).u32();
        let reported = u32::from(size).wrapping_sub(u32::from(start));
        if record != 0 {
            let name = record_form_name(e, record);
            let id = e.mem.u32(record);
            let record_flags = e.mem.u32(record + 5);
            e.call(
                ERROR,
                &args![
                    0x0101_2cb0u32,
                    reported,
                    id,
                    name,
                    record_flags,
                    0x226u32,
                    SOURCE_FILE
                ],
            );
        } else {
            e.call(
                ERROR,
                &args![0x0101_2c78u32, reported, 0x226u32, SOURCE_FILE],
            );
        }
    }
    size
}

// Translated from 005f18c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The actor base's save (`RET 4`: the change flags). With save blocks
/// (`UseSaveGameBlocks`) it writes the tag `KOLB` and a 16-bit size
/// placeholder (4 and 2 bytes, `008579b0`), remembering the position before
/// the placeholder (`00825c00`). Then the form (`00484c20`, save version
/// 0x6D or more), the attributes (+0xB8 `00480210`), the actor base data
/// (+0x30 `0047e4a0`), the spell list (+0x7C `TESSpellList::SaveGame`), the
/// AI form (+0x90 `TESAIForm::SaveGame`) and, with flag 0x20, the name at
/// +0xD0 as a length byte and its characters (`TESForm::SaveGameDataOLD`).
/// When the setting at `011de4e8` is set the size written is reported
/// (`Error`, line 0x24B). With save blocks the size since the placeholder
/// is written into it, with a warning when it is more than 0xFFFF. The
/// health (+0xB0) is saved under a mask the compiler folded to 0
/// (`AND EDX,0`): never.
// The exe tests a mask the compiler folded to 0; the branch is kept as written.
#[allow(clippy::bad_bit_mask, clippy::erasing_op)]
pub fn fn_005f18c0(e: &mut Engine, this: Ptr, flags: u32) {
    let base = this.addr();
    e.with_stack(0x34, |e, frame| {
        // The exe's locals, at their offsets from `EBP`.
        let ebp = frame.addr() + 0x34;
        let size_slot = ebp - 8;
        let tag_slot = ebp - 0x10;
        let health_slot = ebp - 0x14;
        let length_slot = ebp - 0x19;
        e.mem.set_u16(size_slot, 0);
        let mut placeholder = 0u32;
        let game = e.global::<u32>(SAVE_LOAD_GAME_OBJECT);
        let mut start = e.call(SAVE_POSITION, &args![game]).u32();
        if reports_save_sizes(e) {
            let game = e.global::<u32>(SAVE_LOAD_GAME_OBJECT);
            start = e.call(SAVE_POSITION, &args![game]).u32();
        }
        let game = e.global::<u32>(SAVE_LOAD_GAME_OBJECT);
        if e.call(USE_SAVE_GAME_BLOCKS, &args![game]).u8() != 0 {
            e.mem.set_u32(tag_slot, BLOCK_TAG);
            let game = e.global::<u32>(SAVE_LOAD_GAME_OBJECT);
            e.call(SAVE_WRITE, &args![game, tag_slot, 4u32]);
            let game = e.global::<u32>(SAVE_LOAD_GAME_OBJECT);
            placeholder = e.call(SAVE_POSITION, &args![game]).u32();
            let game = e.global::<u32>(SAVE_LOAD_GAME_OBJECT);
            e.call(SAVE_WRITE, &args![game, size_slot, 2u32]);
        }
        let game = e.global::<u32>(SAVE_LOAD_GAME_OBJECT);
        if e.call(SAVE_VERSION, &args![game]).u8() >= 0x6d {
            e.call(0x0048_4c20, &args![this, flags]);
        }
        e.call(0x0048_0210, &args![base + 0xb8, flags]);
        e.call(0x0047_e4a0, &args![base + 0x30, flags]);
        e.call(0x0048_da20, &args![base + 0x7c, flags]);
        e.call(0x0047_f840, &args![base + 0x90, flags]);
        // Folded mask (always false in the exe): the health.
        if flags & 0 != 0 {
            let health = e.call(0x0072_6070, &args![base + 0xb0]).u32();
            e.mem.set_u32(health_slot, health);
            e.call(SAVE_FORM_DATA, &args![this, health_slot, 4u32]);
        }
        if flags & 0x20 != 0 {
            let text = e.call(STRING_TEXT, &args![base + 0xd0]).u32();
            let length = e.call(STRING_LENGTH, &args![text]).u32() as u8;
            e.mem.set_u8(length_slot, length);
            e.call(SAVE_FORM_DATA, &args![this, length_slot, 1u32]);
            if e.mem.u8(length_slot) != 0 {
                let length = u32::from(e.mem.u8(length_slot));
                e.call(SAVE_FORM_DATA, &args![this, text, length]);
            }
        }
        if reports_save_sizes(e) {
            let game = e.global::<u32>(SAVE_LOAD_GAME_OBJECT);
            let end = e.call(SAVE_POSITION, &args![game]).u32();
            let game = e.global::<u32>(SAVE_LOAD_GAME_OBJECT);
            let record = e.call(SAVING_RECORD, &args![game]).u32();
            let written = end.wrapping_sub(start);
            if record != 0 {
                let name = record_form_name(e, record);
                let id = e.mem.u32(record);
                let record_flags = e.mem.u32(record + 5);
                e.call(
                    ERROR,
                    &args![
                        0x0101_53a0u32,
                        written,
                        id,
                        name,
                        record_flags,
                        0x24bu32,
                        SOURCE_FILE
                    ],
                );
            } else {
                e.call(
                    ERROR,
                    &args![0x0101_536cu32, written, 0x24bu32, SOURCE_FILE],
                );
            }
        }
        let game = e.global::<u32>(SAVE_LOAD_GAME_OBJECT);
        if e.call(USE_SAVE_GAME_BLOCKS, &args![game]).u8() != 0 {
            let game = e.global::<u32>(SAVE_LOAD_GAME_OBJECT);
            let end = e.call(SAVE_POSITION, &args![game]).u32();
            if end > placeholder.wrapping_add(0xffff) {
                e.call(LOG_WARNING, &args![0x0101_5318u32, SOURCE_FILE, 0x24bu32]);
            }
            e.mem
                .set_u16(placeholder, end.wrapping_sub(placeholder) as u16);
        }
    });
}

// Translated from 005f1b30 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 005f1b30 - the /GS stack cookie check (00ec408c) is not translated
/// The actor base's load (`RET 8`: the change flags and a second word,
/// both passed on). With save blocks (`UseSaveGameBlocks`) it reads the
/// 4-byte tag (`008579e0`) and warns (line 0x254, with the record of the
/// form being loaded when there is one, else the save version) when it is
/// not `KOLB`, then remembers the position (`00825c00`) and reads the
/// 16-bit block size. Then the form (`00484c50`, save version 0x6D or
/// more), the attributes (+0xB8 `00480240`), the actor base data (+0x30
/// `0047e6f0`), the spell list (+0x7C `0048dca0`), the AI form (+0x90
/// `TESAIForm::LoadGame`) and, with flag 0x20, the name: a cleared
/// 0x104-byte buffer, a length byte and that many characters
/// (`00484d00`), set as the name at +0xD0 (`00489100`). With save blocks
/// it warns (line 0x274) when the position is past or before the block's
/// end. The health is loaded under a mask the compiler folded to 0
/// (`AND ECX,0`): never.
// The exe tests a mask the compiler folded to 0; the branch is kept as written.
#[allow(clippy::bad_bit_mask, clippy::erasing_op)]
pub fn fn_005f1b30(e: &mut Engine, this: Ptr, flags: u32, second: u32) {
    let base = this.addr();
    e.with_stack(0x13c, |e, frame| {
        // The exe's locals, at their offsets from `EBP`.
        let ebp = frame.addr() + 0x13c;
        let size_slot = ebp - 4;
        let tag_slot = ebp - 0xc;
        let health_slot = ebp - 0x18;
        let name_buffer = ebp - 0x128;
        let length_slot = ebp - 0x129;
        e.mem.set_u16(size_slot, 0);
        let mut block_start = 0u32;
        let game = e.global::<u32>(SAVE_LOAD_GAME_OBJECT);
        if e.call(USE_SAVE_GAME_BLOCKS, &args![game]).u8() != 0 {
            let game = e.global::<u32>(SAVE_LOAD_GAME_OBJECT);
            e.call(LOAD_READ, &args![game, tag_slot, 4u32]);
            if e.mem.u32(tag_slot) != BLOCK_TAG {
                let game = e.global::<u32>(SAVE_LOAD_GAME_OBJECT);
                let record = e.call(LOADING_RECORD, &args![game]).u32();
                if record != 0 {
                    let name = record_form_name(e, record);
                    let id = e.mem.u32(record);
                    let kind = u32::from(e.mem.u8(record + 9));
                    let record_flags = e.mem.u32(record + 5);
                    e.call(
                        LOG_WARNING,
                        &args![
                            0x0101_5718u32,
                            SOURCE_FILE,
                            0x254u32,
                            id,
                            name,
                            kind,
                            record_flags
                        ],
                    );
                } else {
                    let game = e.global::<u32>(SAVE_LOAD_GAME_OBJECT);
                    let version = u32::from(e.call(SAVE_VERSION, &args![game]).u8());
                    e.call(
                        LOG_WARNING,
                        &args![0x0101_56a8u32, SOURCE_FILE, 0x254u32, version],
                    );
                }
            }
            let game = e.global::<u32>(SAVE_LOAD_GAME_OBJECT);
            block_start = e.call(SAVE_POSITION, &args![game]).u32();
            let game = e.global::<u32>(SAVE_LOAD_GAME_OBJECT);
            e.call(LOAD_READ, &args![game, size_slot, 2u32]);
        }
        let game = e.global::<u32>(SAVE_LOAD_GAME_OBJECT);
        if e.call(SAVE_VERSION, &args![game]).u8() >= 0x6d {
            e.call(0x0048_4c50, &args![this, flags, second]);
        }
        e.call(0x0048_0240, &args![base + 0xb8, flags, second]);
        e.call(0x0047_e6f0, &args![base + 0x30, flags, second]);
        e.call(0x0048_dca0, &args![base + 0x7c, flags, second]);
        e.call(0x0047_f8c0, &args![base + 0x90, flags, second]);
        // Folded mask (always false in the exe): the health.
        if flags & 0 != 0 {
            e.call(LOAD_FORM_DATA, &args![this, health_slot, 4u32]);
            let health = e.mem.u32(health_slot);
            e.call(SET_WORD_4, &args![base + 0xb0, health]);
        }
        if flags & 0x20 != 0 {
            e.call(0x0040_3d30, &args![name_buffer, 0u32, 0x104u32]);
            e.call(LOAD_FORM_DATA, &args![this, length_slot, 1u32]);
            if e.mem.u8(length_slot) != 0 {
                let length = u32::from(e.mem.u8(length_slot));
                e.call(LOAD_FORM_DATA, &args![this, name_buffer, length]);
            }
            e.call(SET_NAME, &args![base + 0xd0, name_buffer]);
        }
        let game = e.global::<u32>(SAVE_LOAD_GAME_OBJECT);
        if e.call(USE_SAVE_GAME_BLOCKS, &args![game]).u8() == 0 {
            return;
        }
        let game = e.global::<u32>(SAVE_LOAD_GAME_OBJECT);
        let position = e.call(SAVE_POSITION, &args![game]).u32();
        let game = e.global::<u32>(SAVE_LOAD_GAME_OBJECT);
        let record = e.call(LOADING_RECORD, &args![game]).u32();
        let end = u32::from(e.mem.u16(size_slot)).wrapping_add(block_start);
        if record != 0 {
            let id = e.mem.u32(record);
            let form = e.call(LOOKUP_FORM, &args![id]).u32();
            let (format, difference) = if position > end {
                (0x0101_5588u32, position.wrapping_sub(end))
            } else if position < end {
                (0x0101_5500u32, end.wrapping_sub(position))
            } else {
                return;
            };
            let record_flags = e.mem.u32(record + 5);
            let kind = u32::from(e.mem.u8(record + 9));
            let name = e.vcall(form, 0x130, &[]).u32();
            let id = e.mem.u32(record);
            e.call(
                LOG_WARNING,
                &args![
                    format,
                    difference,
                    SOURCE_FILE,
                    0x274u32,
                    id,
                    name,
                    kind,
                    record_flags
                ],
            );
        } else {
            let (format, difference) = if position > end {
                (0x0101_54a0u32, position.wrapping_sub(end))
            } else if position < end {
                (0x0101_5440u32, end.wrapping_sub(position))
            } else {
                return;
            };
            let game = e.global::<u32>(SAVE_LOAD_GAME_OBJECT);
            let version = u32::from(e.call(SAVE_VERSION, &args![game]).u8());
            e.call(
                LOG_WARNING,
                &args![format, difference, SOURCE_FILE, 0x274u32, version],
            );
        }
    });
}

// Translated from 005f1fd0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 005f1fd0 - the /GS stack cookie check (00ec408c) is not translated
/// Loads the actor base from the load buffer (`RET 4`): the form
/// (`00484da0`), the actor base data (+0x30, `0047ea70`), the spell list
/// (+0x7C, `0048e140`), the attributes (+0xB8, `004802b0`), the AI form
/// (+0x90, `0047f980`), then, when the buffer's flag 0x20 is set
/// (`00428110`, `004280f0`), a name read into a stack buffer
/// (`BGSLoadGameBuffer::LoadString`) and set as the name at +0xD0
/// (`00489100`).
pub fn fn_005f1fd0(e: &mut Engine, this: Ptr, buffer: u32) {
    let base = this.addr();
    e.call(0x0048_4da0, &args![this, buffer]);
    e.call(0x0047_ea70, &args![base + 0x30, buffer]);
    e.call(0x0048_e140, &args![base + 0x7c, buffer]);
    e.call(0x0048_02b0, &args![base + 0xb8, buffer]);
    e.call(0x0047_f980, &args![base + 0x90, buffer]);
    e.with_stack(0x118, |e, frame| {
        // The exe's locals, at their offsets from `EBP`.
        let ebp = frame.addr() + 0x118;
        let flags_slot = ebp - 0x114;
        let name = ebp - 0x110;
        let flags = e.call(0x0042_8110, &args![buffer, flags_slot]).u32();
        if e.call(0x0042_80f0, &args![flags, 0x20u32]).u8() != 0 {
            e.call(0x0086_49a0, &args![buffer, name]);
            e.call(SET_NAME, &args![base + 0xd0, name]);
        }
    });
}

// Translated from 005f20a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Nothing for a zero byte; otherwise runs a stack `Reset3DMobIterator`
/// (`005f2150`; +8 this actor base, +0xC the byte) over the list at
/// `011e0e80` (`0096bbc0`) twice, with its +4 word 0 and then 1
/// (`006ecd40`), and when the player's base form (`004181e0`) is this one
/// and the player has a process (`008d8520`), passes the byte to the
/// process's virtual slot 0x468 (`RET 4`).
pub fn fn_005f20a0(e: &mut Engine, this: Ptr, value: u8) {
    if value == 0 {
        return;
    }
    e.with_stack(0x10, |e, iterator| {
        fn_005f2150(e, iterator);
        e.mem.set_u32(iterator.addr() + 8, this.addr());
        e.mem.set_u8(iterator.addr() + 0xc, value);
        e.call(SET_WORD_4, &args![iterator, 0u32]);
        e.call(ITERATE, &args![ITERATED_LIST, iterator]);
        e.call(SET_WORD_4, &args![iterator, 1u32]);
        e.call(ITERATE, &args![ITERATED_LIST, iterator]);
    });
    let player = e.global::<u32>(PLAYER);
    if e.call(PLAYER_BASE_FORM, &args![player]).u32() != this.addr() {
        return;
    }
    let player = e.global::<u32>(PLAYER);
    if e.call(REFERENCE_PROCESS, &args![player]).u32() == 0 {
        return;
    }
    let player = e.global::<u32>(PLAYER);
    let process = e.call(REFERENCE_PROCESS, &args![player]).u32();
    e.vcall(process, 0x468, &args![u32::from(value)]);
}

// Translated from 005f2150 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `Reset3DMobIterator` constructor (the vtable `01047670`, whose slot
/// 0 is `Reset3DMobIterator::Operate`): its base (`005d8b10`), then the
/// vtable. Returns `this`.
pub fn fn_005f2150(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(0x005d_8b10, &args![this]);
    e.mem.set_u32(this.addr(), RESET_3D_ITERATOR_VTABLE);
    this
}

// Translated from 005f2170 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Reset3DMobIterator::Operate` (Xbox PDB; the reference; `RET 4`): when
/// the reference's base object (`007af430`) is the iterator's actor base
/// (+8) and the reference has a process (`008d8520`), passes the
/// iterator's byte (+0xC) to the process's virtual slot 0x468. Returns
/// nothing the callers read (`EAX` is left as the last call leaves it).
pub fn reset_3d_mob_iterator_operate(e: &mut Engine, this: Ptr, reference: Ptr) {
    let object = e.call(REFERENCE_BASE_OBJECT, &args![reference]).u32();
    if object != e.mem.u32(this.addr() + 8) {
        return;
    }
    if e.call(REFERENCE_PROCESS, &args![reference]).u32() == 0 {
        return;
    }
    let process = e.call(REFERENCE_PROCESS, &args![reference]).u32();
    let value = u32::from(e.mem.u8(this.addr() + 0xc));
    e.vcall(process, 0x468, &args![value]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x005f_0c80, tes_actor_base_is_immobile(Ptr) -> bool),
        entry!(0x005f_0cc0, tes_actor_base_get_sex(Ptr) -> i32),
        entry!(0x005f0c40, fn_005f0c40(Ptr) -> bool),
        entry!(0x005f_11f0, fn_005f11f0(Ptr) -> u32),
        entry!(0x005f_1210, fn_005f1210(Ptr) -> u32),
        entry!(0x005f_1230, fn_005f1230(Ptr) -> u32),
        entry!(0x005f_1250, fn_005f1250(Ptr) -> u32),
        entry!(0x005f_1270, fn_005f1270(Ptr) -> u32),
        entry!(0x005f_1290, fn_005f1290(Ptr) -> u32),
        entry!(0x005f_12b0, fn_005f12b0(Ptr) -> u32),
        entry!(0x005f_1570, fn_005f1570(Ptr, u8)),
        entry!(0x005f_0b00, tes_actor_base_get_health(Ptr) -> u32),
        entry!(0x005f_0d00, fn_005f0d00(Ptr) -> bool),
        entry!(0x005f_0d30, fn_005f0d30(Ptr, u8)),
        entry!(0x005f_0f80, fn_005f0f80(Ptr) -> u32),
        entry!(0x0056_5210, tes_actor_base_set_starts_dead(Ptr, u8)),
        entry!(0x005f_0b50, fn_005f0b50(Ptr) -> bool),
        entry!(0x005f_0bf0, fn_005f0bf0(Ptr) -> bool),
        entry!(0x005f_1630, fn_005f1630(Ptr, u32)),
        entry!(0x005f_1690, fn_005f1690(Ptr, u32)),
        entry!(0x005f_0fb0, fn_005f0fb0(Ptr, u32) -> u32),
        entry!(0x005f_12d0, fn_005f12d0(Ptr, u32, u32)),
        entry!(0x005f_0d60, tes_actor_base_get_desirability(Ptr, u32) -> f32),
        entry!(0x005f_1590, tes_actor_base_should_run_init_default_worn(Ptr, u32) -> bool),
        entry!(0x005f_1f30, fn_005f1f30(Ptr, u32)),
        entry!(0x005f_0b30, fn_005f0b30(Ptr, u32)),
        entry!(0x005f_0ba0, fn_005f0ba0(Ptr) -> bool),
        entry!(0x005f_0f50, fn_005f0f50(Ptr) -> bool),
        entry!(0x005f_14b0, fn_005f14b0(Ptr, u8)),
        entry!(0x005f_14d0, fn_005f14d0(Ptr, u8)),
        entry!(0x005f_14f0, fn_005f14f0(Ptr, u8)),
        entry!(0x005f_1510, fn_005f1510(Ptr, u8)),
        entry!(0x005f_1530, fn_005f1530(Ptr, u8)),
        entry!(0x005f_1550, fn_005f1550(Ptr, u8)),
        entry!(0x005f_16f0, fn_005f16f0(Ptr, u32) -> u16),
        entry!(0x005f_18c0, fn_005f18c0(Ptr, u32)),
        entry!(0x005f_1b30, fn_005f1b30(Ptr, u32, u32)),
        entry!(0x005f_1fd0, fn_005f1fd0(Ptr, u32)),
        entry!(0x005f_20a0, fn_005f20a0(Ptr, u8)),
        entry!(0x005f_2150, fn_005f2150(Ptr) -> Ptr),
        entry!(0x005f_2170, reset_3d_mob_iterator_operate(Ptr, Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn engine(type_byte: u32, flags_result: bool) -> Engine {
        let mut e = Engine::new();
        e.register_double(GET_TYPE_BYTE, move |_, _| Ret {
            eax: type_byte,
            ..Ret::default()
        });
        e.register_double(TEST_FLAGS, move |_, _| Ret {
            eax: flags_result as u32,
            ..Ret::default()
        });
        e
    }

    #[test]
    fn other_types_are_false_without_testing_the_flags() {
        let mut e = engine(0x2a, true);
        e.call_log = Some(vec![]);
        assert!(!e.call(0x005f_0c40, &args![Ptr::<()>::new(0x1000)]).bool());
        let log = e.call_log.take().unwrap();
        assert!(log.iter().all(|(addr, _)| *addr != TEST_FLAGS));
    }

    #[test]
    fn type_0x2b_returns_the_flag_test_on_the_embedded_object() {
        let mut e = engine(0x2b, true);
        e.call_log = Some(vec![]);
        assert!(e.call(0x005f_0c40, &args![Ptr::<()>::new(0x1000)]).bool());
        let log = e.call_log.take().unwrap();
        let test = log.iter().find(|(addr, _)| *addr == TEST_FLAGS).unwrap();
        assert_eq!(test.1, vec![0x1030, 0x20]);
        let mut e = engine(0x2b, false);
        assert!(!e.call(0x005f_0c40, &args![Ptr::<()>::new(0x1000)]).bool());
    }
}

#[cfg(test)]
mod physics_work_ahead_tests {
    use super::*;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    fn rig(kind: u32, flag_set: bool) -> Engine {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register_double(GET_TYPE_BYTE, move |_, _| kind.into_ret());
        e.register_double(TEST_FLAGS, move |_, _| u32::from(flag_set).into_ret());
        e
    }

    #[test]
    fn at_005f0c80_is_immobile_only_for_type_0x2b_with_the_flag() {
        let mut e = rig(0x2b, true);
        assert!(e.call(0x005f_0c80, &args![0x4000u32]).bool());
        assert_eq!(calls(&e, TEST_FLAGS), vec![vec![0x4030, 0x0080_0000]]);
        let mut e = rig(0x2b, false);
        assert!(!e.call(0x005f_0c80, &args![0x4000u32]).bool());
        let mut e = rig(0x2a, true);
        assert!(!e.call(0x005f_0c80, &args![0x4000u32]).bool());
        assert!(calls(&e, TEST_FLAGS).is_empty());
    }

    #[test]
    fn at_005f0cc0_answers_the_sex_for_type_0x2a_and_minus_one_otherwise() {
        let mut e = rig(0x2a, true);
        assert_eq!(e.call(0x005f_0cc0, &args![0x4000u32]).i32(), 1);
        assert_eq!(calls(&e, TEST_FLAGS), vec![vec![0x4030, 1]]);
        let mut e = rig(0x2a, false);
        assert_eq!(e.call(0x005f_0cc0, &args![0x4000u32]).i32(), 0);
        let mut e = rig(0x2b, true);
        assert_eq!(e.call(0x005f_0cc0, &args![0x4000u32]).i32(), -1);
        assert_eq!(e.call(0x005f_0cc0, &args![0x4000u32]).u32(), 0xffff_ffff);
        assert!(calls(&e, TEST_FLAGS).is_empty());
    }
}

#[cfg(test)]
mod b024_tests {
    use crate::prelude::*;

    type Log = Vec<(u32, Vec<u32>)>;

    fn ok(_: &mut Engine, _: &[u32]) -> Ret {
        Ret::default()
    }

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    fn st0(value: f64) -> Ret {
        Ret {
            st0: value,
            ..Ret::default()
        }
    }

    fn only(log: &Log, addr: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    fn logged(e: &mut Engine, addr: u32, words: &[u32]) -> (Ret, Log) {
        e.call_log = Some(vec![]);
        let r = e.call(addr, words);
        (r, e.call_log.take().unwrap())
    }

    #[test]
    fn attribute_getters_pass_their_index() {
        for (addr, index) in [
            (0x005f_11f0u32, 5u32),
            (0x005f_1210, 6),
            (0x005f_1230, 7),
            (0x005f_1250, 8),
            (0x005f_1270, 9),
            (0x005f_1290, 0xa),
            (0x005f_12b0, 0xb),
        ] {
            let mut e = Engine::new();
            e.register(0x0048_0160, |_, a| ret(0x1200 + a[1]));
            let (r, log) = logged(&mut e, addr, &[0x3000]);
            assert_eq!(only(&log, 0x0048_0160), vec![vec![0x3000, index]]);
            assert_eq!(r.u32(), 0x1200 + index);
        }
    }

    #[test]
    fn at_005f11f0_returns_the_callee_eax() {
        let mut e = Engine::new();
        e.register(0x0048_0160, |_, _| ret(0xabcd_ef12));
        assert_eq!(e.call(0x005f_11f0, &[0x3000]).u32(), 0xabcd_ef12);
    }

    #[test]
    fn at_005f1570_sets_attribute_b() {
        let mut e = Engine::new();
        e.register(0x0048_0180, ok);
        let (_, log) = logged(&mut e, 0x005f_1570, &[0x3000, 0x1ff]);
        assert_eq!(only(&log, 0x0048_0180), vec![vec![0x3000, 0xb, 0xff, 1]]);
    }

    #[test]
    fn at_005f0b00_asks_the_health_part() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x120);
        e.put_vtable(0x0090_0000, &[0x00a0_0000; 8]);
        e.mem.set_u32(this + 0xb0, 0x0090_0000);
        e.mem.set_u32(0x0090_0010, 0x00a0_0010);
        e.register(0x00a0_0010, |_, a| ret(a[0]));
        assert_eq!(e.call(0x005f_0b00, &[this]).u32(), this + 0xb0);
    }

    fn typed(kind: u32, flags: u32) -> Engine {
        let mut e = Engine::new();
        e.register_double(0x0040_1170, move |_, _| ret(kind));
        e.register_double(0x0046_1580, move |_, a| ret(u32::from(a[1] & flags != 0)));
        e.register(0x0047_dd50, ok);
        e
    }

    #[test]
    fn at_005f0d00_tests_flag_10_for_type_2a() {
        let mut e = typed(0x2a, 0x10);
        let (r, log) = logged(&mut e, 0x005f_0d00, &[0x3000]);
        assert!(r.bool());
        assert_eq!(only(&log, 0x0046_1580), vec![vec![0x3030, 0x10]]);
        let mut e = typed(0x2a, 0x1);
        assert!(!e.call(0x005f_0d00, &[0x3000]).bool());
        let mut e = typed(0x2b, 0x10);
        let (r, log) = logged(&mut e, 0x005f_0d00, &[0x3000]);
        assert!(!r.bool());
        assert!(only(&log, 0x0046_1580).is_empty());
    }

    #[test]
    fn at_005f0d30_sets_flag_10_for_type_2a() {
        let mut e = typed(0x2a, 0);
        let (_, log) = logged(&mut e, 0x005f_0d30, &[0x3000, 0x101]);
        assert_eq!(only(&log, 0x0047_dd50), vec![vec![0x3030, 0x10, 1, 1]]);
        let mut e = typed(0x2b, 0);
        let (_, log) = logged(&mut e, 0x005f_0d30, &[0x3000, 1]);
        assert!(only(&log, 0x0047_dd50).is_empty());
    }

    #[test]
    fn at_005f0f80_picks_the_player_global() {
        let mut e = Engine::new();
        e.map(0x011c_a000, 0x1000);
        e.map(0x011d_e000, 0x1000);
        e.set_global::<u32>(0x011d_ea3c, 0x0055_0000);
        e.set_global::<u32>(0x011c_afec, 0x1111);
        e.register(0x0041_81e0, |_, _| ret(0x3000));
        // The player word is 0: the other one.
        assert_eq!(e.call(0x005f_0f80, &[0x3000]).u32(), 0x1111);
        e.set_global::<u32>(0x011c_aff0, 0x2222);
        assert_eq!(e.call(0x005f_0f80, &[0x3000]).u32(), 0x2222);
        assert_eq!(e.call(0x005f_0f80, &[0x4000]).u32(), 0x1111);
    }

    #[test]
    fn at_00565210_sets_or_clears_starts_dead() {
        for (value, expected) in [
            (1u32, 0x0008_0001u32),
            (0x100, 0x0000_0001),
            (0, 0x0000_0001),
        ] {
            let mut e = Engine::new();
            e.register(0x0044_ddc0, |_, _| ret(0x0008_0001));
            e.register(0x0040_3550, ok);
            let mut e2 = Engine::new();
            e2.register(0x0044_ddc0, |_, _| ret(0x0000_0001));
            e2.register(0x0040_3550, ok);
            let (_, log) = logged(&mut e, 0x0056_5210, &[0x3000, value]);
            assert_eq!(only(&log, 0x0040_3550), vec![vec![0x3000, expected]]);
            let (_, log) = logged(&mut e2, 0x0056_5210, &[0x3000, value]);
            assert_eq!(only(&log, 0x0040_3550), vec![vec![0x3000, expected]]);
        }
    }

    #[test]
    fn at_005f0b50_tests_flags_10_then_1() {
        assert!(typed(0x2a, 0).call(0x005f_0b50, &[0x3000]).bool());
        assert!(typed(0x2b, 0x10).call(0x005f_0b50, &[0x3000]).bool());
        assert!(typed(0x2b, 0x1).call(0x005f_0b50, &[0x3000]).bool());
        let mut e = typed(0x2b, 0x40);
        let (r, log) = logged(&mut e, 0x005f_0b50, &[0x3000]);
        assert!(!r.bool());
        assert_eq!(
            only(&log, 0x0046_1580),
            vec![vec![0x3030, 0x10], vec![0x3030, 1]]
        );
    }

    #[test]
    fn at_005f0bf0_tests_flags_40_then_1() {
        assert!(typed(0x2a, 0).call(0x005f_0bf0, &[0x3000]).bool());
        assert!(typed(0x2b, 0x40).call(0x005f_0bf0, &[0x3000]).bool());
        assert!(typed(0x2b, 0x1).call(0x005f_0bf0, &[0x3000]).bool());
        let mut e = typed(0x2b, 0x10);
        let (r, log) = logged(&mut e, 0x005f_0bf0, &[0x3000]);
        assert!(!r.bool());
        assert_eq!(
            only(&log, 0x0046_1580),
            vec![vec![0x3030, 0x40], vec![0x3030, 1]]
        );
    }

    fn change_engine(known: u32, change: u32) -> Engine {
        let mut e = Engine::new();
        e.map(0x011c_3000, 0x1000);
        e.map(0x011d_d000, 0x1000);
        e.set_global::<u32>(0x011c_3f2c, 0x0066_0000);
        e.set_global::<u32>(0x011d_df38, 0x0077_0000);
        e.register(0x0084_e3a0, |_, _| ret(0x42));
        e.register_double(0x0046_9860, move |_, _| ret(known));
        e.register_double(0x0084_a620, move |_, _| ret(change));
        e.register_double(0x0084_a740, move |_, _| ret(change));
        for addr in [0x0047_e340u32, 0x0048_4b60, 0x0047_e360, 0x0048_4bc0] {
            e.register(addr, ok);
        }
        e
    }

    #[test]
    fn at_005f1630_routes_the_value() {
        let mut e = change_engine(1, 1);
        let (_, log) = logged(&mut e, 0x005f_1630, &[0x3000, 9]);
        assert_eq!(only(&log, 0x0046_9860), vec![vec![0x0066_0000, 0x42]]);
        assert_eq!(only(&log, 0x0084_a620), vec![vec![0x0077_0000, 0x3000, 0]]);
        assert_eq!(only(&log, 0x0047_e340), vec![vec![0x3030, 9]]);
        let mut e = change_engine(1, 0);
        let (_, log) = logged(&mut e, 0x005f_1630, &[0x3000, 9]);
        assert!(only(&log, 0x0047_e340).is_empty());
        assert!(only(&log, 0x0048_4b60).is_empty());
        let mut e = change_engine(0, 1);
        let (_, log) = logged(&mut e, 0x005f_1630, &[0x3000, 9]);
        assert_eq!(only(&log, 0x0048_4b60), vec![vec![0x3000, 9]]);
        assert!(only(&log, 0x0084_a620).is_empty());
    }

    #[test]
    fn at_005f1690_routes_the_value() {
        let mut e = change_engine(1, 1);
        let (_, log) = logged(&mut e, 0x005f_1690, &[0x3000, 9]);
        assert_eq!(only(&log, 0x0084_a740), vec![vec![0x0077_0000, 0x3000]]);
        assert_eq!(only(&log, 0x0047_e360), vec![vec![0x3030, 9]]);
        assert!(only(&log, 0x0048_4bc0).is_empty());
        let mut e = change_engine(1, 0);
        let (_, log) = logged(&mut e, 0x005f_1690, &[0x3000, 9]);
        assert_eq!(only(&log, 0x0048_4bc0), vec![vec![0x3000, 9]]);
        let mut e = change_engine(0, 1);
        let (_, log) = logged(&mut e, 0x005f_1690, &[0x3000, 9]);
        assert_eq!(only(&log, 0x0048_4bc0), vec![vec![0x3000, 9]]);
        assert!(only(&log, 0x0084_a740).is_empty());
    }

    #[test]
    fn at_005f0fb0_reads_each_actor_value() {
        let mut e = Engine::new();
        let base = e.mem.alloc(0x120);
        let owner = base + 0x100;
        // Byte getters answer 0x1nn with nn their address's low byte.
        for addr in [
            0x004f_1540u32,
            0x004f_6de0,
            0x005b_b4d0,
            0x0047_ef30,
            0x0062_f2f0,
            0x0047_ee30,
        ] {
            e.register_double(addr, move |_, _| ret(0x100 | (addr & 0xff)));
        }
        e.register(0x0048_0160, |_, a| ret(0xff00 | a[1]));
        e.put_vtable(0x0090_0000, &[0x00a0_0000; 0x20]);
        e.mem.set_u32(base + 0xb0, 0x0090_0000);
        e.mem.set_u32(base + 0x30, 0x0090_0000);
        e.mem.set_u32(0x0090_0010, 0x00a0_0010);
        e.mem.set_u32(0x0090_0060, 0x00a0_0060);
        e.mem.set_u32(0x0090_0064, 0x00a0_0064);
        e.register(0x00a0_0010, |_, _| ret(77));
        e.register(0x00a0_0060, |_, _| ret(0x0001_0005));
        e.register(0x00a0_0064, |_, _| st0(12.75));
        e.register(0x008f_21d0, |_, _| ret(0x0002_0006));
        e.register(0x00ec_62c0, |_, a| {
            let v = f64::from_bits(u64::from(a[0]) | (u64::from(a[1]) << 32));
            ret(v.trunc() as i32 as u32)
        });
        let read = |e: &mut Engine, index: u32| e.call(0x005f_0fb0, &[owner, index]).u32();
        assert_eq!(read(&mut e, 0), 0x40);
        assert_eq!(read(&mut e, 1), 0xe0);
        assert_eq!(read(&mut e, 2), 0xd0);
        assert_eq!(read(&mut e, 3), 0x30);
        assert_eq!(read(&mut e, 4), 0xf0);
        for index in 5..=0xb {
            assert_eq!(read(&mut e, index), index);
        }
        assert_eq!(read(&mut e, 0x10), 77);
        assert_eq!(read(&mut e, 0x15), 6);
        assert_eq!(read(&mut e, 0x16), 5);
        assert_eq!(read(&mut e, 0x17), 12);
        assert_eq!(read(&mut e, 0x39), 0x30);
        for index in [0xc, 0xf, 0x11, 0x14, 0x18, 0x38, 0x3a, 0xffff_ffff] {
            assert_eq!(read(&mut e, index), 0, "index {index:#x}");
        }
        e.call_log = Some(vec![]);
        read(&mut e, 0);
        read(&mut e, 5);
        let log = e.call_log.take().unwrap();
        assert_eq!(only(&log, 0x004f_1540), vec![vec![base + 0x90]]);
        assert_eq!(only(&log, 0x0048_0160), vec![vec![base + 0xb8, 5]]);
    }

    #[test]
    fn at_005f12d0_sets_each_actor_value() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x120);
        let setters = [
            0x0047_ee00u32,
            0x0047_eed0,
            0x0047_ef00,
            0x0047_ef50,
            0x0047_ef80,
            0x005f_14b0,
            0x005f_14d0,
            0x005f_14f0,
            0x005f_1510,
            0x005f_1530,
            0x005f_1550,
            0x0048_0180,
            0x005f_0b30,
            0x0047_e270,
            0x0047_e1c0,
        ];
        for addr in setters {
            e.register(addr, ok);
        }
        let set = |e: &mut Engine, index: u32, value: u32| {
            e.call_log = Some(vec![]);
            e.call(0x005f_12d0, &[this, index, value]);
            e.call_log
                .take()
                .unwrap()
                .into_iter()
                .skip(1)
                .collect::<Log>()
        };
        assert_eq!(
            set(&mut e, 0, 0x1234),
            vec![(0x0047_ee00, vec![this + 0x90, 0x34, 1])]
        );
        assert_eq!(
            set(&mut e, 3, 0x1234),
            vec![(0x0047_ef50, vec![this + 0x90, 0x34, 1])]
        );
        assert_eq!(
            set(&mut e, 4, 0x1234),
            vec![(0x0047_ef80, vec![this + 0x90, 0x34])]
        );
        assert_eq!(
            set(&mut e, 5, 0x1234),
            vec![(0x005f_14b0, vec![this + 0xb8, 0x34])]
        );
        assert_eq!(
            set(&mut e, 0xa, 0x1234),
            vec![(0x005f_1550, vec![this + 0xb8, 0x34])]
        );
        assert_eq!(
            set(&mut e, 0xb, 0x1234),
            vec![(0x0048_0180, vec![this + 0xb8, 0xb, 0x34, 1])]
        );
        assert_eq!(
            set(&mut e, 0x10, 0x1234),
            vec![(0x005f_0b30, vec![this, 0x1234])]
        );
        assert_eq!(
            set(&mut e, 0x15, 0x0012_3456),
            vec![(0x0047_e270, vec![this + 0x30, 0x3456])]
        );
        assert_eq!(
            set(&mut e, 0x17, 0xffff_fffe),
            vec![(0x0047_e1c0, vec![this + 0x30, (-2.0f32).to_bits()])]
        );
        for index in [0xc, 0x11, 0x16, 0x18, 0xffff_ffff] {
            assert!(set(&mut e, index, 1).is_empty(), "index {index:#x}");
        }
    }

    fn desirability_engine(kind: u32) -> Engine {
        let mut e = Engine::new();
        e.register_double(0x0040_1170, move |_, _| ret(kind));
        e.register(0x00ec_43fb, |_, a| ret(a[0] + (a[3] & 0xf)));
        e.register(0x0048_73d0, |_, _| ret(1));
        e
    }

    #[test]
    fn at_005f0d60_rates_each_item_type() {
        let mut e = desirability_engine(0x18);
        assert_eq!(e.call(0x005f_0d60, &[0x3000, 0]).f64(), 0.0);
        // Armour: 004be180 + CalcArmorRating(chop(004be060) & 0xffff, 1.0).
        e.register(0x004b_e060, |_, _| st0(65537.9));
        e.register(0x0064_6360, |_, a| {
            st0(f64::from(a[0]) + f64::from(f32::from_bits(a[1])))
        });
        e.register(0x004b_e180, |_, _| st0(0.25));
        let (r, log) = logged(&mut e, 0x005f_0d60, &[0x3000, 0x5000]);
        assert_eq!(r.f64(), 2.25);
        assert_eq!(only(&log, 0x0064_6360), vec![vec![1, 0x3f80_0000]]);
        assert_eq!(
            only(&log, 0x00ec_43fb)[0],
            vec![0x5000, 0, 0x0118_3028, 0x0118_3a34, 0]
        );
        // No health: 0.
        e.register(0x0048_73d0, |_, _| ret(0));
        assert_eq!(e.call(0x005f_0d60, &[0x3000, 0x5000]).f64(), 0.0);

        // Clothing: 00647cf0 of the word at +4 of +0x58.
        let mut e = desirability_engine(0x1a);
        e.register(0x0072_6070, |_, a| ret(a[0] + 1));
        e.register(0x0064_7cf0, |_, a| st0(f64::from(a[0])));
        let (r, log) = logged(&mut e, 0x005f_0d60, &[0x3000, 0x5000]);
        assert_eq!(r.f64(), f64::from(0x5000u32 + 0xc + 0x58 + 1));
        assert_eq!(only(&log, 0x00ec_43fb)[0][3], 0x0118_6fcc);

        // Type 0x1E: the constant.
        let mut e = desirability_engine(0x1e);
        e.map(0x0101_6000, 0x1000);
        e.set_global::<u32>(0x0101_6410, 100.0f32.to_bits());
        assert_eq!(e.call(0x005f_0d60, &[0x3000, 0x5000]).f64(), 100.0);

        // Weapon: CalcWeaponDamage for this + 0x100 (0 for a null this).
        let mut e = desirability_engine(0x28);
        e.register(0x0064_4ce0, |_, _| st0(1.0 / 3.0));
        let (r, log) = logged(&mut e, 0x005f_0d60, &[0x3000, 0x5000]);
        assert_eq!(r.f64(), f64::from(1.0f32 / 3.0));
        let one = 1.0f32.to_bits();
        assert_eq!(
            only(&log, 0x0064_4ce0),
            vec![vec![0x3100, 0x5008, one, one, 0, 0, 0, 0]]
        );
        let (_, log) = logged(&mut e, 0x005f_0d60, &[0, 0x5000]);
        assert_eq!(only(&log, 0x0064_4ce0)[0][0], 0);

        // Ammunition: 1.0; a failed cast 0.
        let mut e = desirability_engine(0x29);
        assert_eq!(e.call(0x005f_0d60, &[0x3000, 0x5000]).f64(), 1.0);
        e.register(0x00ec_43fb, |_, _| ret(0));
        assert_eq!(e.call(0x005f_0d60, &[0x3000, 0x5000]).f64(), 0.0);

        // Other types: 0, no cast.
        for kind in [0x17u32, 0x19, 0x2a, 0x1c] {
            let mut e = desirability_engine(kind);
            let (r, log) = logged(&mut e, 0x005f_0d60, &[0x3000, 0x5000]);
            assert_eq!(r.f64(), 0.0);
            assert!(only(&log, 0x00ec_43fb).is_empty());
        }
    }

    fn worn_engine(flag: u32, query: u32, change: u32) -> (Engine, u32) {
        let mut e = Engine::new();
        e.map(0x011d_d000, 0x1000);
        e.set_global::<u32>(0x011d_df38, 0x0077_0000);
        e.register_double(0x0042_ce10, move |_, _| ret(flag));
        e.register_double(0x0057_6d30, move |_, _| ret(query));
        e.register(0x008c_71b0, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            ret(a[0])
        });
        e.register_double(0x0084_a6d0, move |_, _| ret(change));
        let reference = e.mem.alloc(0x10);
        e.put_vtable(0x0090_0000, &[0x00a0_0000; 0x90]);
        e.mem.set_u32(0x0090_022c, 0x00a0_022c);
        e.mem.set_u32(reference, 0x0090_0000);
        e.register_double(0x00a0_022c, move |_, _| ret(query));
        (e, reference)
    }

    #[test]
    fn at_005f1590_checks_the_change_flags() {
        let (mut e, r) = worn_engine(1, 1, 1);
        let (v, log) = logged(&mut e, 0x005f_1590, &[0x3000, r]);
        assert!(!v.bool());
        assert_eq!(
            only(&log, 0x0084_a6d0),
            vec![vec![0x0077_0000, r, 0x0800_0020]]
        );
        assert!(only(&log, 0x00a0_022c).is_empty());
        let (mut e, r) = worn_engine(1, 1, 0);
        assert!(e.call(0x005f_1590, &[0x3000, r]).bool());
        let (mut e, r) = worn_engine(1, 0, 1);
        let (v, log) = logged(&mut e, 0x005f_1590, &[0x3000, r]);
        assert!(v.bool());
        assert!(only(&log, 0x0084_a6d0).is_empty());
        // The save/load flag clear: the reference's slot 0x22C and 0x400.
        let (mut e, r) = worn_engine(0, 1, 1);
        let (v, log) = logged(&mut e, 0x005f_1590, &[0x3000, r]);
        assert!(!v.bool());
        assert_eq!(only(&log, 0x00a0_022c), vec![vec![r, 0]]);
        assert_eq!(only(&log, 0x0084_a6d0), vec![vec![0x0077_0000, r, 0x400]]);
        let (mut e, r) = worn_engine(0, 0, 1);
        assert!(e.call(0x005f_1590, &[0x3000, r]).bool());
    }

    #[test]
    fn at_005f1f30_saves_the_parts() {
        for (flag, name) in [(0x20u32, true), (0xdf, false)] {
            let mut e = Engine::new();
            for addr in [
                0x0048_4d60u32,
                0x0047_ea30,
                0x0048_e040,
                0x0048_0270,
                0x0047_f940,
                0x0086_5e70,
            ] {
                e.register(addr, ok);
            }
            e.register(0x0042_8110, |e, a| {
                e.mem.set_u32(a[1], 0x99);
                ret(a[1])
            });
            e.register_double(0x0042_80f0, move |_, a| ret(u32::from(a[1] & flag != 0)));
            e.register(0x0040_8da0, |_, a| ret(a[0] + 1));
            let (_, log) = logged(&mut e, 0x005f_1f30, &[0x3000, 0x8000]);
            let calls: Vec<u32> = log.iter().skip(1).map(|(a, _)| *a).collect();
            let mut expected = vec![
                0x0048_4d60,
                0x0047_ea30,
                0x0048_e040,
                0x0048_0270,
                0x0047_f940,
                0x0042_8110,
                0x0042_80f0,
            ];
            if name {
                expected.extend([0x0040_8da0, 0x0086_5e70]);
            }
            assert_eq!(calls, expected);
            assert_eq!(only(&log, 0x0047_ea30), vec![vec![0x3030, 0x8000]]);
            assert_eq!(only(&log, 0x0048_e040), vec![vec![0x307c, 0x8000]]);
            assert_eq!(only(&log, 0x0048_0270), vec![vec![0x30b8, 0x8000]]);
            assert_eq!(only(&log, 0x0047_f940), vec![vec![0x3090, 0x8000]]);
            if name {
                assert_eq!(only(&log, 0x0086_5e70), vec![vec![0x8000, 0x30d1, 0]]);
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

    fn zero(e: &mut Engine, addrs: &[u32]) {
        for addr in addrs {
            e.register(*addr, |_, _| Ret::default());
        }
    }

    #[test]
    fn health_setter_and_attribute_setters() {
        let mut e = Engine::new();
        zero(&mut e, &[SET_WORD_4, ATTRIBUTE_SET]);
        let (_, log) = run(&mut e, 0x005f_0b30, &[0x5000, 77]);
        assert_eq!(log, vec![(SET_WORD_4, vec![0x50b0, 77])]);
        for (addr, index) in [
            (0x005f_14b0, 5),
            (0x005f_14d0, 6),
            (0x005f_14f0, 7),
            (0x005f_1510, 8),
            (0x005f_1530, 9),
            (0x005f_1550, 0xa),
        ] {
            let (_, log) = run(&mut e, addr, &[0x5000, 0x1ff]);
            assert_eq!(log, vec![(ATTRIBUTE_SET, vec![0x5000, index, 0xff, 1])]);
        }
    }

    #[test]
    fn flag_checks() {
        let mut e = Engine::new();
        e.register(GET_TYPE_BYTE, |e, a| e.mem.u32(a[0] + 0x40).into_ret());
        e.register(TEST_FLAGS, |e, a| {
            u32::from(e.mem.u32(a[0]) & a[1] != 0).into_ret()
        });
        e.register(FORM_FLAGS, |e, a| e.mem.u32(a[0] + 8).into_ret());
        let this = e.mem.alloc(0x100);
        // Not type 0x2b: 005f0b50 true, so true.
        e.mem.set_u32(this + 0x40, 0x2a);
        assert!(e.call(0x005f_0ba0, &args![this]).bool());
        e.mem.set_u32(this + 0x40, 0x2b);
        // 0x2b without flags 0x10 or 1: false.
        assert!(!e.call(0x005f_0ba0, &args![this]).bool());
        e.mem.set_u32(this + 0x30, 0x10);
        assert!(e.call(0x005f_0ba0, &args![this]).bool());
        e.mem.set_u32(this + 0x30, 0x0004_0001);
        assert!(!e.call(0x005f_0ba0, &args![this]).bool());
        e.mem.set_u32(this + 8, 0x0008_0000);
        assert!(e.call(0x005f_0f50, &args![this]).bool());
        e.mem.set_u32(this + 8, 0x0007_ffff);
        assert!(!e.call(0x005f_0f50, &args![this]).bool());
    }

    fn save_engine(blocks: u32, version: u32, report: u8) -> Engine {
        let mut e = Engine::new();
        e.map(SAVE_LOAD_GAME_OBJECT, 4);
        e.mem.set_u32(SAVE_LOAD_GAME_OBJECT, 0x7700);
        e.register_double(USE_SAVE_GAME_BLOCKS, move |_, _| blocks.into_ret());
        e.register_double(SAVE_VERSION, move |_, _| version.into_ret());
        let setting = e.mem.alloc(4);
        e.mem.set_u8(setting, report);
        e.register_double(SETTING_VALUE, move |_, _| setting.into_ret());
        e.register(STRING_TEXT, |e, a| e.mem.u32(a[0]).into_ret());
        e.register(STRING_LENGTH, |_, _| 5u32.into_ret());
        e.register(LOOKUP_FORM, |_, a| (a[0] + 0x10).into_ret());
        e.register(0x0f00_1000, |_, _| 0x6600u32.into_ret());
        let mut slots = vec![0u32; 0x134 / 4];
        slots[0x130 / 4] = 0x0f00_1000;
        e.put_vtable(0x0f00_0000, &slots);
        zero(&mut e, &[ERROR, LOG_WARNING]);
        e
    }

    /// A save record at a new block: form id `0x2000` (whose form, at
    /// `0x2010`, has the vtable `0f000000`), flags at +5, a byte at +9.
    fn record(e: &mut Engine) -> u32 {
        e.map(0x2010, 4);
        e.mem.set_u32(0x2010, 0x0f00_0000);
        let record = e.mem.alloc(0x10);
        e.mem.set_u32(record, 0x2000);
        e.mem.set_u32(record + 5, 0xabcd_ef01);
        e.mem.set_u8(record + 9, 0x2a);
        record
    }

    #[test]
    fn save_size_sums_the_parts() {
        let mut e = save_engine(1, 0x6d, 0);
        e.register(0x0048_4bf0, |_, _| 0x10u32.into_ret());
        e.register(0x0048_01e0, |_, _| 0x7u32.into_ret());
        e.register(0x0047_e380, |_, _| 0x1_0020u32.into_ret());
        e.register(0x0048_d8f0, |_, _| 0x3u32.into_ret());
        e.register(0x0047_f7e0, |_, _| 0x5u32.into_ret());
        let this = e.mem.alloc(0x100);
        let (ret, log) = run(&mut e, 0x005f_16f0, &[this, 0x20]);
        assert_eq!(ret.u32() & 0xffff, 6 + 0x10 + 7 + 0x20 + 3 + 5 + 1 + 5);
        assert_eq!(calls(&log, 0x0048_01e0), vec![vec![this + 0xb8, 0x20]]);
        assert_eq!(calls(&log, 0x0047_f7e0), vec![vec![this + 0x90, 0x20]]);
        assert!(calls(&log, ERROR).is_empty());
        let mut e2 = save_engine(0, 0x6c, 1);
        for getter in [0x0048_01e0, 0x0047_e380, 0x0048_d8f0, 0x0047_f7e0] {
            e2.register(getter, |_, _| 1u32.into_ret());
        }
        e2.register(SAVING_RECORD, |_, _| Ret::default());
        let (ret, log) = run(&mut e2, 0x005f_16f0, &[this, 0xffff_ffdf]);
        assert_eq!(ret.u32() & 0xffff, 4);
        assert!(calls(&log, 0x0048_4bf0).is_empty());
        assert_eq!(
            calls(&log, ERROR),
            vec![vec![0x0101_2c78, 4, 0x226, SOURCE_FILE]]
        );
        let rec = record(&mut e2);
        e2.register_double(SAVING_RECORD, move |_, _| rec.into_ret());
        let (_, log) = run(&mut e2, 0x005f_16f0, &[this, 0]);
        assert_eq!(
            calls(&log, ERROR),
            vec![vec![
                0x0101_2cb0,
                4,
                0x2000,
                0x6600,
                0xabcd_ef01,
                0x226,
                SOURCE_FILE
            ]]
        );
    }

    #[test]
    fn save_game_writes_the_block_and_its_size() {
        let mut e = save_engine(1, 0x6d, 1);
        zero(
            &mut e,
            &[
                0x0048_4c20,
                0x0048_0210,
                0x0047_e4a0,
                0x0048_da20,
                0x0047_f840,
                SAVE_FORM_DATA,
            ],
        );
        let block = e.mem.alloc(0x20);
        // The position: the block start, then 0x30 bytes later.
        let position = e.mem.alloc(4);
        e.mem.set_u32(position, block);
        e.register_double(SAVE_POSITION, move |e, _| e.mem.u32(position).into_ret());
        e.register_double(SAVE_WRITE, move |e, a| {
            if a[2] == 4 {
                assert_eq!(e.mem.u32(a[1]), BLOCK_TAG);
            }
            let at = e.mem.u32(position);
            e.mem.set_u32(position, at + a[2]);
            Ret::default()
        });
        e.register_double(0x0047_f840, move |e, _| {
            let at = e.mem.u32(position);
            e.mem.set_u32(position, at + 0x30);
            Ret::default()
        });
        e.register(SAVING_RECORD, |_, _| Ret::default());
        let this = e.mem.alloc(0x100);
        e.mem.set_u32(this + 0xd0, 0x9000);
        let (_, log) = run(&mut e, 0x005f_18c0, &[this, 0x20]);
        assert_eq!(calls(&log, 0x0048_4c20), vec![vec![this, 0x20]]);
        let saved = calls(&log, SAVE_FORM_DATA);
        assert_eq!(saved.len(), 2);
        assert_eq!((saved[0][0], saved[0][2]), (this, 1));
        assert_eq!(saved[1], vec![this, 0x9000, 5]);
        // The placeholder (after the tag) holds the bytes since it: 2 + 0x30.
        assert_eq!(e.mem.u16(block + 4), 0x32);
        assert_eq!(
            calls(&log, ERROR),
            vec![vec![0x0101_536c, 0x36, 0x24b, SOURCE_FILE]]
        );
        assert!(calls(&log, LOG_WARNING).is_empty());
        // Without blocks nothing is written for the block.
        let mut e = save_engine(0, 0x10, 0);
        zero(
            &mut e,
            &[
                0x0048_0210,
                0x0047_e4a0,
                0x0048_da20,
                0x0047_f840,
                SAVE_POSITION,
                SAVE_WRITE,
            ],
        );
        let (_, log) = run(&mut e, 0x005f_18c0, &[this, 0]);
        assert!(calls(&log, SAVE_WRITE).is_empty());
        assert!(calls(&log, 0x0048_4c20).is_empty());
        assert!(calls(&log, SAVE_FORM_DATA).is_empty());
    }

    #[test]
    fn save_game_warns_about_a_block_over_0xffff() {
        let mut e = save_engine(1, 0x10, 0);
        zero(
            &mut e,
            &[
                0x0048_0210,
                0x0047_e4a0,
                0x0048_da20,
                0x0047_f840,
                SAVE_WRITE,
            ],
        );
        let block = e.mem.alloc(0x20);
        let calls_seen = e.mem.alloc(4);
        e.register_double(SAVE_POSITION, move |e, _| {
            let n = e.mem.u32(calls_seen);
            e.mem.set_u32(calls_seen, n + 1);
            // The second call (after the tag) is the placeholder; the last
            // is 0x10000 bytes past it.
            if n < 2 { block } else { block + 0x1_0000 }.into_ret()
        });
        let this = e.mem.alloc(0x100);
        let (_, log) = run(&mut e, 0x005f_18c0, &[this, 0]);
        assert_eq!(
            calls(&log, LOG_WARNING),
            vec![vec![0x0101_5318, SOURCE_FILE, 0x24b]]
        );
        assert_eq!(e.mem.u16(block), 0);
    }

    fn load_engine(blocks: u32, version: u32) -> Engine {
        let mut e = save_engine(blocks, version, 0);
        zero(
            &mut e,
            &[
                0x0048_4c50,
                0x0048_0240,
                0x0047_e6f0,
                0x0048_dca0,
                0x0047_f8c0,
                0x0040_3d30,
                SET_NAME,
            ],
        );
        e
    }

    #[test]
    fn load_game_reads_the_block_parts_and_name() {
        let mut e = load_engine(1, 0x6d);
        e.register(LOAD_READ, |e, a| {
            if a[2] == 4 {
                e.mem.set_u32(a[1], BLOCK_TAG);
            } else {
                e.mem.set_u16(a[1], 0x40);
            }
            Ret::default()
        });
        e.register(SAVE_POSITION, |_, _| 0x1000u32.into_ret());
        e.register(LOADING_RECORD, |_, _| Ret::default());
        e.register(LOAD_FORM_DATA, |e, a| {
            if a[2] == 1 {
                e.mem.set_u8(a[1], 3);
            }
            Ret::default()
        });
        let this = e.mem.alloc(0x100);
        let (_, log) = run(&mut e, 0x005f_1b30, &[this, 0x20, 9]);
        assert_eq!(calls(&log, 0x0048_4c50), vec![vec![this, 0x20, 9]]);
        assert_eq!(calls(&log, 0x0047_f8c0), vec![vec![this + 0x90, 0x20, 9]]);
        let reads = calls(&log, LOAD_FORM_DATA);
        assert_eq!(reads.len(), 2);
        let buffer = reads[1][1];
        assert_eq!(reads[1], vec![this, buffer, 3]);
        assert_eq!(calls(&log, 0x0040_3d30), vec![vec![buffer, 0, 0x104]]);
        assert_eq!(calls(&log, SET_NAME), vec![vec![this + 0xd0, buffer]]);
        // The position 0x1000 is 0x40 before the block's end.
        assert_eq!(
            calls(&log, LOG_WARNING),
            vec![vec![0x0101_5440, 0x40, SOURCE_FILE, 0x274, 0x6d]]
        );
    }

    #[test]
    fn load_game_warns_about_a_bad_tag_and_a_long_block_with_the_record() {
        let mut e = load_engine(1, 0x10);
        let rec = record(&mut e);
        e.register_double(LOADING_RECORD, move |_, _| rec.into_ret());
        e.register(LOAD_READ, |e, a| {
            if a[2] == 4 {
                e.mem.set_u32(a[1], 0x1234);
            } else {
                e.mem.set_u16(a[1], 0x10);
            }
            Ret::default()
        });
        let position = e.mem.alloc(4);
        e.mem.set_u32(position, 0x1000);
        e.register_double(SAVE_POSITION, move |e, _| e.mem.u32(position).into_ret());
        e.register_double(0x0047_f8c0, move |e, _| {
            e.mem.set_u32(position, 0x1018);
            Ret::default()
        });
        let this = e.mem.alloc(0x100);
        let (_, log) = run(&mut e, 0x005f_1b30, &[this, 0, 0]);
        assert!(calls(&log, 0x0048_4c50).is_empty());
        assert!(calls(&log, SET_NAME).is_empty());
        assert_eq!(
            calls(&log, LOG_WARNING),
            vec![
                vec![
                    0x0101_5718,
                    SOURCE_FILE,
                    0x254,
                    0x2000,
                    0x6600,
                    0x2a,
                    0xabcd_ef01
                ],
                vec![
                    0x0101_5588,
                    8,
                    SOURCE_FILE,
                    0x274,
                    0x2000,
                    0x6600,
                    0x2a,
                    0xabcd_ef01
                ]
            ]
        );
        // Exactly at the end: no warning.
        e.mem.set_u32(position, 0x1000);
        e.register_double(0x0047_f8c0, move |e, _| {
            e.mem.set_u32(position, 0x1010);
            Ret::default()
        });
        e.register(LOAD_READ, |e, a| {
            if a[2] == 4 {
                e.mem.set_u32(a[1], BLOCK_TAG);
            } else {
                e.mem.set_u16(a[1], 0x10);
            }
            Ret::default()
        });
        let (_, log) = run(&mut e, 0x005f_1b30, &[this, 0, 0]);
        assert!(calls(&log, LOG_WARNING).is_empty());
    }

    #[test]
    fn load_game_without_blocks_reads_only_the_parts() {
        let mut e = load_engine(0, 0x6c);
        zero(&mut e, &[LOAD_READ, SAVE_POSITION, LOADING_RECORD]);
        let this = e.mem.alloc(0x100);
        let (_, log) = run(&mut e, 0x005f_1b30, &[this, 0, 0]);
        assert!(calls(&log, LOAD_READ).is_empty());
        assert!(calls(&log, LOADING_RECORD).is_empty());
        assert_eq!(calls(&log, 0x0048_0240), vec![vec![this + 0xb8, 0, 0]]);
    }

    #[test]
    fn buffer_load_reads_parts_and_the_name_with_flag_0x20() {
        let mut e = Engine::new();
        zero(
            &mut e,
            &[
                0x0048_4da0,
                0x0047_ea70,
                0x0048_e140,
                0x0048_02b0,
                0x0047_f980,
                0x0086_49a0,
                SET_NAME,
            ],
        );
        e.register(0x0042_8110, |_, _| 0x20u32.into_ret());
        e.register(0x0042_80f0, |_, a| u32::from(a[0] & a[1] != 0).into_ret());
        let (_, log) = run(&mut e, 0x005f_1fd0, &[0x5000, 0x8800]);
        assert_eq!(
            log[..5],
            [
                (0x0048_4da0, vec![0x5000, 0x8800]),
                (0x0047_ea70, vec![0x5030, 0x8800]),
                (0x0048_e140, vec![0x507c, 0x8800]),
                (0x0048_02b0, vec![0x50b8, 0x8800]),
                (0x0047_f980, vec![0x5090, 0x8800]),
            ]
        );
        let name = calls(&log, 0x0086_49a0)[0][1];
        assert_eq!(calls(&log, SET_NAME), vec![vec![0x50d0, name]]);
        e.register(0x0042_8110, |_, _| 0x10u32.into_ret());
        let (_, log) = run(&mut e, 0x005f_1fd0, &[0x5000, 0x8800]);
        assert!(calls(&log, SET_NAME).is_empty());
    }

    #[test]
    fn reset_3d_runs_the_iterator_twice_and_the_players_process() {
        let mut e = Engine::new();
        zero(&mut e, &[0x005d_8b10]);
        e.register(SET_WORD_4, |e, a| {
            e.mem.set_u32(a[0] + 4, a[1]);
            Ret::default()
        });
        e.register(ITERATE, |e, a| {
            let it = a[1];
            (e.mem.u32(it) ^ e.mem.u32(it + 4) ^ e.mem.u32(it + 8) ^ e.mem.u32(it + 0xc)).into_ret()
        });
        e.map(PLAYER, 4);
        e.mem.set_u32(PLAYER, 0x4400);
        e.register(PLAYER_BASE_FORM, |_, _| 0x5000u32.into_ret());
        e.register(REFERENCE_PROCESS, |_, _| 0x6000u32.into_ret());
        e.map(0x6000, 4);
        e.mem.set_u32(0x6000, 0x0f00_0000);
        e.register(0x0f00_1000, |_, _| Ret::default());
        let mut slots = vec![0u32; 0x46c / 4];
        slots[0x468 / 4] = 0x0f00_1000;
        e.put_vtable(0x0f00_0000, &slots);
        let (_, log) = run(&mut e, 0x005f_20a0, &[0x5000, 0x101]);
        let iterated = calls(&log, ITERATE);
        assert_eq!(iterated.len(), 2);
        assert_eq!(iterated[0][0], ITERATED_LIST);
        let it = iterated[0][1];
        assert_eq!(calls(&log, SET_WORD_4), vec![vec![it, 0], vec![it, 1]]);
        assert_eq!(calls(&log, 0x0f00_1000), vec![vec![0x6000, 1]]);
        e.register(PLAYER_BASE_FORM, |_, _| 0x5001u32.into_ret());
        let (_, log) = run(&mut e, 0x005f_20a0, &[0x5000, 1]);
        assert!(calls(&log, 0x0f00_1000).is_empty());
        let (_, log) = run(&mut e, 0x005f_20a0, &[0x5000, 0x100]);
        assert!(log.is_empty());
    }

    #[test]
    fn iterator_constructor_and_operate() {
        let mut e = Engine::new();
        zero(&mut e, &[0x005d_8b10]);
        let it = e.mem.alloc(0x10);
        assert_eq!(e.call(0x005f_2150, &args![it]).u32(), it);
        assert_eq!(e.mem.u32(it), RESET_3D_ITERATOR_VTABLE);
        e.mem.set_u32(it + 8, 0x5000);
        e.mem.set_u8(it + 0xc, 7);
        e.register(REFERENCE_BASE_OBJECT, |e, a| e.mem.u32(a[0]).into_ret());
        e.register(REFERENCE_PROCESS, |e, a| e.mem.u32(a[0] + 4).into_ret());
        e.register(0x0f00_1000, |_, _| Ret::default());
        let mut slots = vec![0u32; 0x46c / 4];
        slots[0x468 / 4] = 0x0f00_1000;
        e.put_vtable(0x0f00_0000, &slots);
        let process = e.mem.alloc(4);
        e.mem.set_u32(process, 0x0f00_0000);
        let reference = e.mem.alloc(8);
        e.mem.set_u32(reference, 0x5000);
        e.mem.set_u32(reference + 4, process);
        let (_, log) = run(&mut e, 0x005f_2170, &[it, reference]);
        assert_eq!(calls(&log, 0x0f00_1000), vec![vec![process, 7]]);
        e.mem.set_u32(reference + 4, 0);
        let (_, log) = run(&mut e, 0x005f_2170, &[it, reference]);
        assert!(calls(&log, 0x0f00_1000).is_empty());
        e.mem.set_u32(reference, 0x5001);
        let (_, log) = run(&mut e, 0x005f_2170, &[it, reference]);
        assert_eq!(log.len(), 1);
    }
}

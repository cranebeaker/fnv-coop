//! `fallout shared/magic/enchantmentitem.cpp` (Xbox PDB source unit), subsystem `fallout shared/magic`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00409080 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00409080 - the exception-unwinding frame (FS:[0]) and the security-cookie bookkeeping are not translated.
/// `EnchantmentItem::EnchantmentItem` (Xbox PDB): the form constructor `CreateFormOfType` (`00465110`) runs for form type 0x13 (0x44 bytes).
/// `ECX` is the object; it runs the base and component constructors, stores the vtable pointers of the object and its components, and sets the fields to their start values.
/// The exception-unwinding frame is not translated.
pub fn enchantmentitem_enchantmentitem(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(0x0040_9190, &args![this]);
    e.mem.set_u32(base, 0x0101_2ea4);
    e.mem.set_u32(base + 0x18, 0x0101_2e5c);
    e.mem.set_u32(base + 0x24, 0x0101_2e44);
    e.call(0x004f_15a0, &args![this, 0x13]);
    e.call(0x0040_9310, &args![this]);
    this
}

/// The function `fn_00409340` calls on the member (`00406340`).
const CALLEE_00406340: u32 = 0x0040_6340;

// Translated from 00409340 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thin wrapper: calls `00406340` with `ECX` = `this + 0x24` and passes on nothing (the callee returns no value).
pub fn fn_00409340(e: &mut Engine, this: Ptr) {
    e.call(CALLEE_00406340, &args![this.byte_add(0x24)]);
}

// Translated from 00409160 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor shape (the engine map gives it no name; the class is unconfirmed): runs the destructor body `004092a0` on `this`, then frees `this` with `00401030` when bit 0 of `flags` is set. Returns `this`.
pub fn fn_00409160(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x0040_92a0, &args![this]);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

// Translated from 00409270 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor shape (the engine map gives it no name; the class is unconfirmed): runs the destructor body `00409210` on `this`, then frees `this` with `00401030` when bit 0 of `flags` is set. Returns `this`.
pub fn fn_00409270(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x0040_9210, &args![this]);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

/// `__RTDynamicCast` (CRT), cdecl (object, offset, source type, target type,
/// is-reference).
const DYNAMIC_CAST: u32 = 0x00ec_43fb;
/// The RTTI type descriptors the casts use: `TESForm`, `TESObject`,
/// `TESBoundObject`, `TESBipedModelForm`, `TESObjectWEAP` and
/// `TESEnchantableForm` (their decorated names at +8).
const TYPE_TES_FORM: u32 = 0x0118_3028;
const TYPE_TES_OBJECT: u32 = 0x0118_3128;
const TYPE_TES_BOUND_OBJECT: u32 = 0x0118_3108;
const TYPE_TES_BIPED_MODEL_FORM: u32 = 0x0118_3978;
const TYPE_TES_OBJECT_WEAP: u32 = 0x0118_3998;
const TYPE_TES_ENCHANTABLE_FORM: u32 = 0x0118_39b4;
/// Thiscall on a form: its type byte (+4).
const FORM_TYPE: u32 = 0x0040_1170;
/// Thiscall: the word at +0x34 (here `EnchantmentItemData::eCastingType`;
/// the engine map's name `ActorMover::GetPreferredMoveMode` is folded code).
const WORD_AT_34: u32 = 0x005f_36f0;
/// Thiscall on an `EffectItemList` (mask), `RET 4`: `AL`, whether one of its
/// effects has the flag (`effectitemlist.rs`).
const LIST_HAS_EFFECT_FLAG: u32 = 0x0040_6140;
/// Thiscall on a weapon: `AL`, bit 0 of the byte at +0xc0.
const WEAPON_FLAG_00409700: u32 = 0x0040_9700;
/// Thiscall on a biped model form: `AL` (`tesactorbasedata.cpp`; the body
/// returns 0).
const BIPED_FLAG_0047C850: u32 = 0x0047_c850;
/// Thiscall on an effect setting (flag mask), `RET 4`: `AL`.
const EFFECT_HAS_FLAG: u32 = 0x0040_3e60;
/// `ExtraDataList::GetCanNotWear` (engine map), thiscall: `AL`.
const EXTRA_GET_CAN_NOT_WEAR: u32 = 0x0041_8b10;
/// `TESForm::TESForm` and `TESForm::~TESForm` (engine map), thiscall.
const FORM_CONSTRUCT: u32 = 0x0048_3370;
const FORM_DESTRUCT: u32 = 0x0048_3630;
/// The `MagicItem` constructor (`effectitemlist.rs`) and destructor
/// (`0040a140`), thiscall on the `MagicItem` base (+0x18).
const MAGIC_ITEM_CONSTRUCT: u32 = 0x0040_2c90;
const MAGIC_ITEM_DESTRUCT: u32 = 0x0040_a140;
/// `MagicItemForm`'s three vtables (+0, +0x18, +0x24; RTTI
/// `.?AVMagicItemForm@@`).
const MAGIC_ITEM_FORM_VTABLES: [u32; 3] = [0x0101_3044, 0x0101_2ffc, 0x0101_2fe4];
/// `EnchantmentItem`'s three vtables (RTTI `.?AVEnchantmentItem@@`).
const ENCHANTMENT_ITEM_VTABLES: [u32; 3] = [0x0101_2ea4, 0x0101_2e5c, 0x0101_2e44];

/// Stores the three vtables of a magic item form (`+0`, `+0x18`, `+0x24`).
fn store_vtables(e: &mut Engine, this: Ptr, vtables: [u32; 3]) {
    e.mem.set_u32(this.addr(), vtables[0]);
    e.mem.set_u32(this.addr() + 0x18, vtables[1]);
    e.mem.set_u32(this.addr() + 0x24, vtables[2]);
}

// Translated from 00409130 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`): the chunk tag `0x54494e45`
/// (`"ENIT"`).
pub fn fn_00409130(e: &mut Engine, this: Ptr) -> u32 {
    let _ = (e, this);
    0x5449_4e45
}

// Translated from 00409140 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`): `this + 0x1c` (on a magic item
/// form that is `TESFullName::cFullName`, Xbox PDB +0x2c).
pub fn fn_00409140(e: &mut Engine, this: Ptr) -> u32 {
    let _ = e;
    this.addr().wrapping_add(0x1c)
}

// Translated from 00409190 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00409190 - the C++ exception frame (FS:[0] chain, unwind state 0 after the TESForm base) is not translated.
/// `MagicItemForm::MagicItemForm` (Xbox PDB): the `TESForm` base
/// (`00483370`), the `MagicItem` base at +0x18 (`00402c90`), then the three
/// `MagicItemForm` vtables. Returns `this`.
pub fn magic_item_form_magic_item_form(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(FORM_CONSTRUCT, &args![this]);
    e.call(MAGIC_ITEM_CONSTRUCT, &args![this.addr() + 0x18]);
    store_vtables(e, this, MAGIC_ITEM_FORM_VTABLES);
    this
}

// Translated from 00409210 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00409210 - the C++ exception frame (FS:[0] chain, unwind state 0 around the MagicItem base) is not translated.
/// The `MagicItemForm` destructor body (unnamed in the engine map; the
/// decompiler's library match is wrong): the `MagicItem` base (+0x18,
/// `0040a140`), then `TESForm::~TESForm`.
pub fn fn_00409210(e: &mut Engine, this: Ptr) {
    e.call(MAGIC_ITEM_DESTRUCT, &args![this.addr() + 0x18]);
    e.call(FORM_DESTRUCT, &args![this]);
}

// Translated from 004092a0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 004092a0 - the C++ exception frame (FS:[0] chain, unwind state 0 around the effect list's clean-up) is not translated.
/// The `EnchantmentItem` destructor body (unnamed in the engine map): the
/// three `EnchantmentItem` vtables, the effects removed (`00409340`), then
/// the `MagicItemForm` destructor body.
pub fn fn_004092a0(e: &mut Engine, this: Ptr) {
    store_vtables(e, this, ENCHANTMENT_ITEM_VTABLES);
    fn_00409340(e, this);
    fn_00409210(e, this);
}

// Translated from 00409310 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the `EnchantmentItemData` start values
/// (Xbox PDB names, PC offsets): `iCostOverride` (+0x3c) and
/// `iChargeOverride` (+0x38) -1, `iFlags` (+0x40) 0, `eCastingType` (+0x34)
/// 2.
pub fn fn_00409310(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr() + 0x3c, 0xffff_ffff);
    e.mem.set_u32(this.addr() + 0x38, 0xffff_ffff);
    e.mem.set_u8(this.addr() + 0x40, 0);
    e.mem.set_u32(this.addr() + 0x34, 2);
}

// Translated from 00409480 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: sets `EnchantmentItemData::iCostOverride`
/// (+0x3c).
pub fn fn_00409480(e: &mut Engine, this: Ptr, cost: u32) {
    e.mem.set_u32(this.addr() + 0x3c, cost);
}

// Translated from 004094f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`): whether the enchantment
/// suits `form`, by the form's type (a `TESObject`, through
/// `__RTDynamicCast`) and the casting type (`+0x34`): type 0x18 when no
/// effect has flag 0x20000 and the casting type is 3; 0x19 (bit 0 of +0xc0,
/// `00409700`) for casting type 0; 0x1a (a `TESBipedModelForm`) for casting
/// type 3 unless `0047c850` is clear and an effect has flag 0x20000; 0x28 (a
/// `TESObjectWEAP`), 0x29, 0x2a and 0x2b for casting type 2; 0x51 always;
/// every other type never.
pub fn fn_004094f0(e: &mut Engine, this: Ptr, form: u32) -> bool {
    let object = e
        .call(
            DYNAMIC_CAST,
            &args![form, 0u32, TYPE_TES_FORM, TYPE_TES_OBJECT, 0u32],
        )
        .u32();
    if object == 0 {
        return false;
    }
    let effects = this.addr() + 0x24;
    // The compiler's switch: a byte table (`004096bc`) maps `type - 0x18` (0
    // to 0x39, compared unsigned) to the cases of `0040969c`.
    match e.call(FORM_TYPE, &args![object]).u32().wrapping_sub(0x18) {
        0 => {
            !e.call(LIST_HAS_EFFECT_FLAG, &args![effects, 0x20000u32])
                .bool()
                && e.call(WORD_AT_34, &args![this]).u32() == 3
        }
        1 => {
            e.call(WEAPON_FLAG_00409700, &args![object]).bool()
                && e.call(WORD_AT_34, &args![this]).u32() == 0
        }
        2 => {
            let biped = e
                .call(
                    DYNAMIC_CAST,
                    &args![form, 0u32, TYPE_TES_FORM, TYPE_TES_BIPED_MODEL_FORM, 0u32],
                )
                .u32();
            if biped == 0 {
                return false;
            }
            if !e.call(BIPED_FLAG_0047C850, &args![biped]).bool()
                && e.call(LIST_HAS_EFFECT_FLAG, &args![effects, 0x20000u32])
                    .bool()
            {
                return false;
            }
            e.call(WORD_AT_34, &args![this]).u32() == 3
        }
        0x10 => {
            let weapon = e
                .call(
                    DYNAMIC_CAST,
                    &args![object, 0u32, TYPE_TES_OBJECT, TYPE_TES_OBJECT_WEAP, 0u32],
                )
                .u32();
            weapon != 0 && e.call(WORD_AT_34, &args![this]).u32() == 2
        }
        0x11..=0x13 => e.call(WORD_AT_34, &args![this]).u32() == 2,
        0x39 => true,
        _ => false,
    }
}

// Translated from 00409720 (decompiled, FalloutNV.exe 1.4.0.525)
/// `EnchantmentItem::IsValidEffectForEnchantment` (Xbox PDB; cdecl, three
/// words): false without a form or an effect; true for form type 0x2b;
/// false unless the form is a `TESBoundObject` that is a
/// `TESEnchantableForm`. Types 0x18 and 0x1a need effect flag 0x10 and none
/// of 0x40000, 0x80 and 0x100, no "can not wear" extra (`extra` when given),
/// a `TESBipedModelForm`, and not flag 0x20000 unless `0047c850` is set;
/// types 0x28 and 0x29 need flag 0x20 and no "can not wear" extra; every
/// other type is valid.
pub fn enchantment_item_is_valid_effect_for_enchantment(
    e: &mut Engine,
    form: u32,
    extra: u32,
    effect: u32,
) -> bool {
    if form == 0 || effect == 0 {
        return false;
    }
    if e.call(FORM_TYPE, &args![form]).u32() == 0x2b {
        return true;
    }
    if e.call(
        DYNAMIC_CAST,
        &args![
            form,
            0u32,
            TYPE_TES_BOUND_OBJECT,
            TYPE_TES_ENCHANTABLE_FORM,
            0u32
        ],
    )
    .u32()
        == 0
    {
        return false;
    }
    let has = |e: &mut Engine, mask: u32| e.call(EFFECT_HAS_FLAG, &args![effect, mask]).bool();
    // The compiler's switch: a byte table (`004098bc`) maps `type - 0x18` (0
    // to 0x11, compared unsigned) to the cases of `004098b0`.
    match e.call(FORM_TYPE, &args![form]).u32().wrapping_sub(0x18) {
        0 | 2 => {
            if !has(e, 0x10) || has(e, 0x40000) || has(e, 0x80) || has(e, 0x100) {
                return false;
            }
            if extra != 0 && e.call(EXTRA_GET_CAN_NOT_WEAR, &args![extra]).bool() {
                return false;
            }
            let biped = e
                .call(
                    DYNAMIC_CAST,
                    &args![
                        form,
                        0u32,
                        TYPE_TES_BOUND_OBJECT,
                        TYPE_TES_BIPED_MODEL_FORM,
                        0u32
                    ],
                )
                .u32();
            if biped == 0 {
                return false;
            }
            e.call(BIPED_FLAG_0047C850, &args![biped]).bool() || !has(e, 0x20000)
        }
        0x10 | 0x11 => {
            if !has(e, 0x20) {
                return false;
            }
            !(extra != 0 && e.call(EXTRA_GET_CAN_NOT_WEAR, &args![extra]).bool())
        }
        _ => true,
    }
}

/// Thiscall on a spell or enchantment form (two words), `RET 8`
/// (`spellitem.cpp`).
const SPELL_ITEM_0040E230: u32 = 0x0040_e230;
/// The RTTI type descriptors of `MagicItem` and `EnchantmentItem` (their
/// decorated names at +8).
const TYPE_MAGIC_ITEM: u32 = 0x0118_3140;
const TYPE_ENCHANTMENT_ITEM: u32 = 0x0118_3594;
/// Thiscall: the word at +0x38 (`EnchantmentItemData::iChargeOverride`,
/// Xbox PDB name, PC offset; folded code).
const WORD_AT_38: u32 = 0x009e_32d0;
/// Thiscall: the word at +0x3c (`EnchantmentItemData::iCostOverride`; folded
/// code).
const WORD_AT_3C: u32 = 0x0063_9b40;
/// Thiscall (word), `RET 4`: stores the word at +0x38 (`spellitem.cpp`).
const SET_WORD_AT_38: u32 = 0x0040_e500;
/// Thiscall (word), `RET 4`: stores the word at +0x34 (folded code named
/// `LowProcess::SetItemBeingUsed` in the engine map).
const SET_WORD_AT_34: u32 = 0x008d_7dc0;
/// Thiscall (mask), `RET 4`: whether the byte at +0x40 has a bit of `mask`
/// (`spellitem.cpp`).
const FLAGS_AT_40_HAVE: u32 = 0x0040_e210;
/// `EffectItemList::GetCost` (`effectitemlist.rs`), thiscall (actor), `RET
/// 4`: a float in `ST0`.
const EFFECT_LIST_GET_COST: u32 = 0x0040_63e0;

// Translated from 00409100 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall` on the `MagicItem` base at +0x18,
/// `RET 4`): `0040e230` on the whole form with 1 and whether the byte
/// `flag` is zero.
pub fn fn_00409100(e: &mut Engine, this: Ptr, flag: u8) {
    let form = this.addr().wrapping_sub(0x18);
    e.call(
        SPELL_ITEM_0040E230,
        &args![form, 1u32, u32::from(flag == 0)],
    );
}

// Translated from 00409360 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall` on the `MagicItem` base at +0x18,
/// `RET 4`): whether `other` (a `MagicItem`) differs from this
/// enchantment: true when it is not an `EnchantmentItem`
/// (`__RTDynamicCast`), or when its charge override (+0x38), cost override
/// (+0x3c), casting type (+0x34, each through its getter, `other`'s first)
/// or flags byte (+0x40) differs from this form's.
pub fn fn_00409360(e: &mut Engine, this: Ptr, other: u32) -> bool {
    let other = e
        .call(
            DYNAMIC_CAST,
            &args![other, 0u32, TYPE_MAGIC_ITEM, TYPE_ENCHANTMENT_ITEM, 0u32],
        )
        .u32();
    if other == 0 {
        return true;
    }
    let form = this.addr().wrapping_sub(0x18);
    for getter in [WORD_AT_38, WORD_AT_3C, WORD_AT_34] {
        let theirs = e.call(getter, &args![other]).u32();
        let ours = e.call(getter, &args![form]).u32();
        if theirs != ours {
            return true;
        }
    }
    e.mem.u8(other + 0x40) != e.mem.u8(this.addr() + 0x28)
}

// Translated from 00409400 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall` on the `MagicItem` base at +0x18,
/// `RET 4`): when `other` (a `MagicItem`) is an `EnchantmentItem`
/// (`__RTDynamicCast`), copies its charge override (+0x38, `0040e500`),
/// cost override (+0x3c, `00409480`), casting type (+0x34, `008d7dc0`) and
/// flags byte (+0x40) into this form; otherwise nothing.
pub fn fn_00409400(e: &mut Engine, this: Ptr, other: u32) {
    let other = e
        .call(
            DYNAMIC_CAST,
            &args![other, 0u32, TYPE_MAGIC_ITEM, TYPE_ENCHANTMENT_ITEM, 0u32],
        )
        .u32();
    if other == 0 {
        return;
    }
    let form = this.addr().wrapping_sub(0x18);
    let charge = e.call(WORD_AT_38, &args![other]).u32();
    e.call(SET_WORD_AT_38, &args![form, charge]);
    let cost = e.call(WORD_AT_3C, &args![other]).u32();
    fn_00409480(e, Ptr::new(form), cost);
    let casting = e.call(WORD_AT_34, &args![other]).u32();
    e.call(SET_WORD_AT_34, &args![form, casting]);
    let flags = e.mem.u8(other + 0x40);
    e.mem.set_u8(this.addr() + 0x28, flags);
}

// Translated from 004094a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall` on the `EffectItemList` at +0x24,
/// `RET 4`, a float in `ST0`): the enchantment's cost. With flag 1 of the
/// byte at +0x40 (`0040e210`) it is the cost override (+0x3c, `FILD`);
/// otherwise `EffectItemList::GetCost(0)` (the argument is not read).
pub fn fn_004094a0(e: &mut Engine, this: Ptr, _unused_1: u32) -> f64 {
    let form = this.addr().wrapping_sub(0x24);
    if e.call(FLAGS_AT_40_HAVE, &args![form, 1u32]).bool() {
        let cost = e.call(WORD_AT_3C, &args![form]).i32();
        f64::from(cost)
    } else {
        e.call(EFFECT_LIST_GET_COST, &args![this, 0u32]).f64()
    }
}

// Translated from 00409700 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall` on a weapon): whether bit 0 of
/// the byte at +0xc0 is set.
pub fn fn_00409700(e: &mut Engine, this: Ptr) -> bool {
    e.mem.u8(this.addr() + 0xc0) & 1 != 0
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0040_9080, enchantmentitem_enchantmentitem(Ptr) -> Ptr),
        entry!(0x0040_9340, fn_00409340(Ptr)),
        entry!(0x0040_9160, fn_00409160(Ptr, u32) -> Ptr),
        entry!(0x0040_9270, fn_00409270(Ptr, u32) -> Ptr),
        entry!(0x0040_9130, fn_00409130(Ptr) -> u32),
        entry!(0x0040_9140, fn_00409140(Ptr) -> u32),
        entry!(0x0040_9190, magic_item_form_magic_item_form(Ptr) -> Ptr),
        entry!(0x0040_9210, fn_00409210(Ptr)),
        entry!(0x0040_92a0, fn_004092a0(Ptr)),
        entry!(0x0040_9310, fn_00409310(Ptr)),
        entry!(0x0040_9480, fn_00409480(Ptr, u32)),
        entry!(0x0040_94f0, fn_004094f0(Ptr, u32) -> bool),
        entry!(
            0x0040_9720,
            enchantment_item_is_valid_effect_for_enchantment(u32, u32, u32) -> bool
        ),
        entry!(0x0040_9100, fn_00409100(Ptr, u8)),
        entry!(0x0040_9360, fn_00409360(Ptr, u32) -> bool),
        entry!(0x0040_9400, fn_00409400(Ptr, u32)),
        entry!(0x0040_94a0, fn_004094a0(Ptr, u32) -> f64),
        entry!(0x0040_9700, fn_00409700(Ptr) -> bool),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::crt::ctor_test::{self, A};

    #[test]
    fn enchantmentitem_enchantmentitem_builds_the_object() {
        ctor_test::run(&ctor_test::Case {
            addr: 0x0040_9080,
            size: 0x44,
            globals: &[],
            returns: &[],
            calls: &[
                (0x0040_9190, &[A::T(0x0)]),
                (0x004f_15a0, &[A::T(0x0), A::V(0x13)]),
                (0x0040_9310, &[A::T(0x0)]),
            ],
            stores: &[
                (0x0, 4, 0x0101_2ea4),
                (0x18, 4, 0x0101_2e5c),
                (0x24, 4, 0x0101_2e44),
            ],
        });
    }

    #[test]
    fn l68d_00409340_passes_this_plus_24_to_the_callee() {
        let mut e = Engine::new();
        e.register(CALLEE_00406340, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0040_9340, &args![0x4100u32]);
        let log = e.call_log.take().unwrap();
        let inner: Vec<_> = log.iter().filter(|c| c.0 == CALLEE_00406340).collect();
        assert_eq!(inner.len(), 1);
        assert_eq!(inner[0].1, vec![0x4100 + 0x24]);
    }
    #[test]
    fn at_00409160_runs_destructor_body_then_deletes_on_flag_one() {
        use crate::prelude::*;

        let mut e = Engine::new();
        let this: Ptr = Ptr::new(e.mem.alloc(16));
        e.register(0x0040_92a0, |_, _| Ret::default());
        e.register(0x0040_1030, |_, _| Ret::default());
        for (flags, deletes) in [(0u32, false), (2, false), (1, true), (0xffff_ffff, true)] {
            e.call_log = Some(vec![]);
            let result = e.call(0x0040_9160, &args![this, flags]);
            assert_eq!(result.u32(), this.addr());
            let log = e.call_log.take().unwrap();
            let calls: Vec<(u32, Vec<u32>)> = log
                .into_iter()
                .filter(|(address, _)| *address != 0x0040_9160)
                .collect();
            let mut expected = vec![(0x0040_92a0, vec![this.addr()])];
            if deletes {
                expected.push((0x0040_1030, vec![this.addr()]));
            }
            assert_eq!(calls, expected);
        }
    }

    #[test]
    fn at_00409270_runs_destructor_body_then_deletes_on_flag_one() {
        use crate::prelude::*;

        let mut e = Engine::new();
        let this: Ptr = Ptr::new(e.mem.alloc(16));
        e.register(0x0040_9210, |_, _| Ret::default());
        e.register(0x0040_1030, |_, _| Ret::default());
        for (flags, deletes) in [(0u32, false), (2, false), (1, true), (0xffff_ffff, true)] {
            e.call_log = Some(vec![]);
            let result = e.call(0x0040_9270, &args![this, flags]);
            assert_eq!(result.u32(), this.addr());
            let log = e.call_log.take().unwrap();
            let calls: Vec<(u32, Vec<u32>)> = log
                .into_iter()
                .filter(|(address, _)| *address != 0x0040_9270)
                .collect();
            let mut expected = vec![(0x0040_9210, vec![this.addr()])];
            if deletes {
                expected.push((0x0040_1030, vec![this.addr()]));
            }
            assert_eq!(calls, expected);
        }
    }
}

#[cfg(test)]
mod enchantment_tests {
    use super::*;

    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, words)| words.clone())
            .collect()
    }

    fn quiet(e: &mut Engine, addrs: &[u32]) {
        for &addr in addrs {
            e.register(addr, |_, _| Ret::default());
        }
    }

    fn returns(e: &mut Engine, addr: u32, value: u32) {
        e.register_double(addr, move |_, _| Ret {
            eax: value,
            ..Ret::default()
        });
    }

    #[test]
    fn chunk_tag_and_name_address() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x0040_9130, &args![0x1000u32]).u32(), 0x5449_4e45);
        assert_eq!(e.call(0x0040_9140, &args![0x1000u32]).u32(), 0x101c);
    }

    #[test]
    fn magic_item_form_constructor_builds_the_bases_then_the_vtables() {
        let mut e = Engine::new();
        quiet(&mut e, &[FORM_CONSTRUCT, MAGIC_ITEM_CONSTRUCT]);
        let this = e.mem.alloc(0x34);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0040_9190, &args![this]).u32(), this);
        assert_eq!(calls_to(&e, FORM_CONSTRUCT), vec![vec![this]]);
        assert_eq!(calls_to(&e, MAGIC_ITEM_CONSTRUCT), vec![vec![this + 0x18]]);
        assert_eq!(e.mem.u32(this), 0x0101_3044);
        assert_eq!(e.mem.u32(this + 0x18), 0x0101_2ffc);
        assert_eq!(e.mem.u32(this + 0x24), 0x0101_2fe4);
    }

    #[test]
    fn destructors_unwind_the_bases() {
        let mut e = Engine::new();
        quiet(
            &mut e,
            &[MAGIC_ITEM_DESTRUCT, FORM_DESTRUCT, CALLEE_00406340],
        );
        let this = e.mem.alloc(0x44);
        e.call_log = Some(vec![]);
        e.call(0x0040_9210, &args![this]);
        assert_eq!(calls_to(&e, MAGIC_ITEM_DESTRUCT), vec![vec![this + 0x18]]);
        assert_eq!(calls_to(&e, FORM_DESTRUCT), vec![vec![this]]);
        e.call_log = Some(vec![]);
        e.call(0x0040_92a0, &args![this]);
        assert_eq!(e.mem.u32(this), 0x0101_2ea4);
        assert_eq!(e.mem.u32(this + 0x18), 0x0101_2e5c);
        assert_eq!(e.mem.u32(this + 0x24), 0x0101_2e44);
        assert_eq!(calls_to(&e, CALLEE_00406340), vec![vec![this + 0x24]]);
        assert_eq!(calls_to(&e, FORM_DESTRUCT), vec![vec![this]]);
    }

    #[test]
    fn data_start_values_and_cost_override() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x44);
        e.mem.set_u8(this + 0x40, 7);
        e.call(0x0040_9310, &args![this]);
        assert_eq!(e.mem.u32(this + 0x34), 2);
        assert_eq!(e.mem.u32(this + 0x38), 0xffff_ffff);
        assert_eq!(e.mem.u32(this + 0x3c), 0xffff_ffff);
        assert_eq!(e.mem.u8(this + 0x40), 0);
        e.call(0x0040_9480, &args![this, 25u32]);
        assert_eq!(e.mem.u32(this + 0x3c), 25);
    }

    /// Doubles for the type checks: the cast answers `cast` for every pair of
    /// types listed, 0 for others; the form type is `kind`.
    fn type_doubles(e: &mut Engine, kind: u32, casts: &'static [(u32, u32)]) {
        e.register_double(DYNAMIC_CAST, move |_, a| Ret {
            eax: if casts.contains(&(a[2], a[3])) {
                0x6000
            } else {
                0
            },
            ..Ret::default()
        });
        returns(e, FORM_TYPE, kind);
    }

    #[test]
    fn suits_form_by_type_and_casting_type() {
        const OBJECT: &[(u32, u32)] = &[(TYPE_TES_FORM, TYPE_TES_OBJECT)];
        const BIPED: &[(u32, u32)] = &[
            (TYPE_TES_FORM, TYPE_TES_OBJECT),
            (TYPE_TES_FORM, TYPE_TES_BIPED_MODEL_FORM),
        ];
        const WEAPON: &[(u32, u32)] = &[
            (TYPE_TES_FORM, TYPE_TES_OBJECT),
            (TYPE_TES_OBJECT, TYPE_TES_OBJECT_WEAP),
        ];
        // (form type, casts, casting type, effect flag, 00409700, 0047c850, result)
        let cases: [(u32, &'static [(u32, u32)], u32, u32, u32, u32, bool); 11] = [
            (0x18, OBJECT, 3, 0, 0, 0, true),
            (0x18, OBJECT, 3, 1, 0, 0, false),
            (0x18, OBJECT, 2, 0, 0, 0, false),
            (0x19, OBJECT, 0, 0, 1, 0, true),
            (0x19, OBJECT, 0, 0, 0, 0, false),
            (0x1a, BIPED, 3, 1, 0, 0, false),
            (0x1a, BIPED, 3, 1, 0, 1, true),
            (0x1a, OBJECT, 3, 0, 0, 0, false),
            (0x28, WEAPON, 2, 0, 0, 0, true),
            (0x2b, OBJECT, 2, 0, 0, 0, true),
            (0x51, OBJECT, 9, 0, 0, 0, true),
        ];
        for (kind, casts, casting, flag, weapon, biped, result) in cases {
            let mut e = Engine::new();
            type_doubles(&mut e, kind, casts);
            returns(&mut e, WORD_AT_34, casting);
            returns(&mut e, LIST_HAS_EFFECT_FLAG, flag);
            returns(&mut e, WEAPON_FLAG_00409700, weapon);
            returns(&mut e, BIPED_FLAG_0047C850, biped);
            assert_eq!(
                e.call(0x0040_94f0, &args![0x5000u32, 0x7000u32]).bool(),
                result,
                "type {kind:x}"
            );
        }
        // Not a `TESObject`, and an unlisted type.
        let mut e = Engine::new();
        type_doubles(&mut e, 0x18, &[]);
        assert!(!e.call(0x0040_94f0, &args![0x5000u32, 0x7000u32]).bool());
        type_doubles(&mut e, 0x30, OBJECT);
        e.call_log = Some(vec![]);
        assert!(!e.call(0x0040_94f0, &args![0x5000u32, 0x7000u32]).bool());
        assert_eq!(
            calls_to(&e, DYNAMIC_CAST),
            vec![vec![0x7000, 0, TYPE_TES_FORM, TYPE_TES_OBJECT, 0]]
        );
    }

    #[test]
    fn valid_effects_for_enchantment() {
        const ENCHANTABLE: &[(u32, u32)] = &[(TYPE_TES_BOUND_OBJECT, TYPE_TES_ENCHANTABLE_FORM)];
        const BIPED: &[(u32, u32)] = &[
            (TYPE_TES_BOUND_OBJECT, TYPE_TES_ENCHANTABLE_FORM),
            (TYPE_TES_BOUND_OBJECT, TYPE_TES_BIPED_MODEL_FORM),
        ];
        // (form type, casts, effect flags, can-not-wear, result)
        let cases: [(u32, &'static [(u32, u32)], u32, u32, bool); 9] = [
            (0x2b, &[], 0, 0, true),
            (0x18, &[], 0x10, 0, false),
            (0x18, BIPED, 0x10, 0, true),
            (0x1a, BIPED, 0, 0, false),
            (0x1a, BIPED, 0x10 | 0x80, 0, false),
            (0x18, BIPED, 0x10 | 0x20000, 0, false),
            (0x18, BIPED, 0x10, 1, false),
            (0x29, ENCHANTABLE, 0x20, 0, true),
            (0x30, ENCHANTABLE, 0, 0, true),
        ];
        for (kind, casts, flags, wear, result) in cases {
            let mut e = Engine::new();
            type_doubles(&mut e, kind, casts);
            e.register_double(EFFECT_HAS_FLAG, move |_, a| Ret {
                eax: u32::from(flags & a[1] != 0),
                ..Ret::default()
            });
            returns(&mut e, EXTRA_GET_CAN_NOT_WEAR, wear);
            returns(&mut e, BIPED_FLAG_0047C850, 0);
            assert_eq!(
                e.call(0x0040_9720, &args![0x7000u32, 0x7100u32, 0x7200u32])
                    .bool(),
                result,
                "type {kind:x} flags {flags:x}"
            );
        }
        let mut e = Engine::new();
        assert!(!e.call(0x0040_9720, &args![0u32, 0u32, 0x7200u32]).bool());
        assert!(!e.call(0x0040_9720, &args![0x7000u32, 0u32, 0u32]).bool());
    }
}

#[cfg(test)]
mod enchantment_copy_tests {
    use super::*;

    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, words)| words.clone())
            .collect()
    }

    /// Getters and setters as the game's; the cast answers the object at
    /// `a[0] - 0x18` unless it is 0x9000 (not an enchantment).
    fn engine() -> Engine {
        let mut e = Engine::new();
        e.register(DYNAMIC_CAST, |_, a| {
            if a[0] == 0x9000 {
                Ret::default()
            } else {
                (a[0] - 0x18).into_ret()
            }
        });
        e.register(WORD_AT_38, |e, a| e.mem.u32(a[0] + 0x38).into_ret());
        e.register(WORD_AT_3C, |e, a| e.mem.u32(a[0] + 0x3c).into_ret());
        e.register(WORD_AT_34, |e, a| e.mem.u32(a[0] + 0x34).into_ret());
        e.register(SET_WORD_AT_38, |e, a| {
            e.mem.set_u32(a[0] + 0x38, a[1]);
            Ret::default()
        });
        e.register(SET_WORD_AT_34, |e, a| {
            e.mem.set_u32(a[0] + 0x34, a[1]);
            Ret::default()
        });
        e
    }

    fn form(e: &mut Engine, words: [u32; 3], flags: u8) -> u32 {
        let form = e.mem.alloc(0x44);
        e.mem.set_u32(form + 0x34, words[0]);
        e.mem.set_u32(form + 0x38, words[1]);
        e.mem.set_u32(form + 0x3c, words[2]);
        e.mem.set_u8(form + 0x40, flags);
        form
    }

    #[test]
    fn flag_call_passes_one_and_the_inverted_byte() {
        let mut e = Engine::new();
        e.register(SPELL_ITEM_0040E230, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0040_9100, &args![0x1018u32, 0x100u32]);
        e.call(0x0040_9100, &args![0x1018u32, 1u32]);
        assert_eq!(
            calls_to(&e, SPELL_ITEM_0040E230),
            vec![vec![0x1000, 1, 1], vec![0x1000, 1, 0]]
        );
    }

    #[test]
    fn enchantments_differ_by_any_value() {
        let mut e = engine();
        let this = form(&mut e, [1, 2, 3], 4);
        let same = form(&mut e, [1, 2, 3], 4);
        assert!(!e.call(0x0040_9360, &args![this + 0x18, same + 0x18]).bool());
        assert!(fn_00409360(&mut e, Ptr::new(this + 0x18), 0x9000));
        for (words, flags) in [
            ([1, 2, 9], 4),
            ([1, 9, 3], 4),
            ([9, 2, 3], 4),
            ([1, 2, 3], 5),
        ] {
            let other = form(&mut e, words, flags);
            assert!(fn_00409360(&mut e, Ptr::new(this + 0x18), other + 0x18));
        }
    }

    #[test]
    fn copy_takes_the_other_enchantments_values() {
        let mut e = engine();
        let this = form(&mut e, [0, 0, 0], 0);
        let other = form(&mut e, [3, 0x55, 0x66], 0x81);
        e.call(0x0040_9400, &args![this + 0x18, other + 0x18]);
        assert_eq!(e.mem.u32(this + 0x34), 3);
        assert_eq!(e.mem.u32(this + 0x38), 0x55);
        assert_eq!(e.mem.u32(this + 0x3c), 0x66);
        assert_eq!(e.mem.u8(this + 0x40), 0x81);
        e.call_log = Some(vec![]);
        fn_00409400(&mut e, Ptr::new(this + 0x18), 0x9000);
        assert!(calls_to(&e, SET_WORD_AT_38).is_empty());
    }

    #[test]
    fn cost_is_the_override_or_the_effects_cost() {
        let mut e = Engine::new();
        e.register(FLAGS_AT_40_HAVE, |e, a| {
            u32::from(u32::from(e.mem.u8(a[0] + 0x40)) & a[1] != 0).into_ret()
        });
        e.register(WORD_AT_3C, |e, a| e.mem.u32(a[0] + 0x3c).into_ret());
        e.register(EFFECT_LIST_GET_COST, |_, _| Ret {
            st0: 12.5,
            ..Ret::default()
        });
        let form = e.mem.alloc(0x44);
        e.mem.set_u32(form + 0x3c, (-0x0100_0001i32) as u32);
        e.mem.set_u8(form + 0x40, 1);
        assert_eq!(
            e.call(0x0040_94a0, &args![form + 0x24, 7u32]).f64(),
            -16_777_217.0
        );
        e.mem.set_u8(form + 0x40, 2);
        e.call_log = Some(vec![]);
        assert_eq!(fn_004094a0(&mut e, Ptr::new(form + 0x24), 7), 12.5);
        assert_eq!(
            calls_to(&e, EFFECT_LIST_GET_COST),
            vec![vec![form + 0x24, 0]]
        );
    }

    #[test]
    fn weapon_flag_is_bit_zero_of_byte_c0() {
        let mut e = Engine::new();
        let weapon = e.mem.alloc(0xc4);
        e.mem.set_u8(weapon + 0xc0, 0xfe);
        assert!(!e.call(0x0040_9700, &args![weapon]).bool());
        e.mem.set_u8(weapon + 0xc0, 0x01);
        assert!(e.call(0x0040_9700, &args![weapon]).bool());
    }
}

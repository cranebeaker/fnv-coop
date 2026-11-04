//! `fallout shared/magic/spellitem.cpp` (Xbox PDB source unit), subsystem `fallout shared/magic`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 0040e160 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0040e160 - the exception-unwinding frame (FS:[0]) and the security-cookie bookkeeping are not translated.
/// `SpellItem::SpellItem` (Xbox PDB): the form constructor `CreateFormOfType` (`00465110`) runs for form type 0x14 (0x44 bytes).
/// `ECX` is the object; it runs the base and component constructors, stores the vtable pointers of the object and its components, and sets the fields to their start values.
/// The exception-unwinding frame is not translated.
pub fn spellitem_spellitem(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(0x0040_9190, &args![this]);
    e.mem.set_u32(base, 0x0101_3f8c);
    e.mem.set_u32(base + 0x18, 0x0101_3f44);
    e.mem.set_u32(base + 0x24, 0x0101_3f28);
    e.call(0x004f_15a0, &args![this, 0x14]);
    e.call(0x0040_e330, &args![this]);
    this
}

/// `0061e780` (named `TESTopicInfo::GetResponseList` in the engine map): the base body the
/// wrapper forwards to; `thiscall`, `this` plus one stack word (`RET 4`).
const RESPONSE_LIST_TARGET_0061E780: u32 = 0x0061_e780;

// Translated from 0061e760 (decompiled, FalloutNV.exe 1.4.0.525)
/// The engine map names this `TESTopicInfo::CloneResponseList` but the body forwards `this`
/// and one pointer to `0061e780` (named `TESTopicInfo::GetResponseList`; Xbox PDB
/// `?GetResponseList@TESTopicInfo@@IAAPAVResponseListWrapper@@PAV2@@Z` takes and returns a
/// `ResponseListWrapper *`). The callee returns its argument when it is non-null and the
/// address `011cc07c` otherwise (the two stores to its result slot before `MOV EAX`); the
/// wrapper returns that pointer.
pub fn fn_0061e760(e: &mut Engine, this: Ptr, wrapper: Ptr) -> Ptr {
    e.call(RESPONSE_LIST_TARGET_0061E780, &args![this, wrapper])
        .ptr()
}

// Translated from 0040e290 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor shape (the engine map gives it no name; the class is unconfirmed): runs the destructor body `0040e2c0` on `this`, then frees `this` with `00401030` when bit 0 of `flags` is set. Returns `this`.
pub fn fn_0040e290(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x0040_e2c0, &args![this]);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

// Translated from 0058e890 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor shape (the engine map gives it no name; the class is unconfirmed): runs the destructor body `0058e8c0` on `this`, then frees `this` with `00401030` when bit 0 of `flags` is set. Returns `this`.
pub fn fn_0058e890(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x0058_e8c0, &args![this]);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

// Translated from 005e6440 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor shape (the engine map gives it no name; the class is unconfirmed): runs the destructor body `005e6470` on `this`, then frees `this` with `00401030` when bit 0 of `flags` is set. Returns `this`.
pub fn fn_005e6440(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x005e_6470, &args![this]);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

// Translated from 005e7370 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor shape (the engine map gives it no name; the class is unconfirmed): runs the destructor body `005e73a0` on `this`, then frees `this` with `00401030` when bit 0 of `flags` is set. Returns `this`.
pub fn fn_005e7370(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x005e_73a0, &args![this]);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

// Translated from 005e8140 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor shape (the engine map gives it no name; the class is unconfirmed): runs the destructor body `005e8170` on `this`, then frees `this` with `00401030` when bit 0 of `flags` is set. Returns `this`.
pub fn fn_005e8140(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x005e_8170, &args![this]);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

// Translated from 005e9560 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor shape (the engine map gives it no name; the class is unconfirmed): runs the destructor body `005e9590` on `this`, then frees `this` with `00401030` when bit 0 of `flags` is set. Returns `this`.
pub fn fn_005e9560(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x005e_9590, &args![this]);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

// Translated from 005e9830 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor shape (the engine map gives it no name; the class is unconfirmed): runs the destructor body `005e9860` on `this`, then frees `this` with `00401030` when bit 0 of `flags` is set. Returns `this`.
pub fn fn_005e9830(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x005e_9860, &args![this]);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

/// The effect list clean-up (`enchantmentitem.rs`, `00409340`) and the
/// `MagicItemForm` destructor body (`00409210`), thiscall.
const REMOVE_EFFECTS_00409340: u32 = 0x0040_9340;
const MAGIC_ITEM_FORM_DESTRUCT: u32 = 0x0040_9210;

// Translated from 0040e210 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`): whether
/// `SpellItemData::iFlags` (+0x40) has any bit of `mask` (the whole stack
/// word is ANDed with the byte).
pub fn fn_0040e210(e: &mut Engine, this: Ptr, mask: u32) -> bool {
    u32::from(e.mem.u8(this.addr() + 0x40)) & mask != 0
}

// Translated from 0040e280 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`): the chunk tag `0x54495053`
/// (`"SPIT"`).
pub fn fn_0040e280(e: &mut Engine, this: Ptr) -> u32 {
    let _ = (e, this);
    0x5449_5053
}

// Translated from 0040e2c0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0040e2c0 - the C++ exception frame (FS:[0] chain, unwind state 0 around the effect list's clean-up) is not translated.
/// The `SpellItem` destructor body (unnamed in the engine map): the three
/// `SpellItem` vtables (RTTI `.?AVSpellItem@@`), the effects removed
/// (`00409340`), then the `MagicItemForm` destructor body (`00409210`).
pub fn fn_0040e2c0(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base, 0x0101_3f8c);
    e.mem.set_u32(base + 0x18, 0x0101_3f44);
    e.mem.set_u32(base + 0x24, 0x0101_3f28);
    e.call(REMOVE_EFFECTS_00409340, &args![this]);
    e.call(MAGIC_ITEM_FORM_DESTRUCT, &args![this]);
}

// Translated from 0040e330 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the `SpellItemData` start values (Xbox PDB
/// names, PC offsets): `iSpellType` (+0x34) 0, `iCostOverride` (+0x38) -1,
/// `eLevelOverride` (+0x3c) 0, `iFlags` (+0x40) 0.
pub fn fn_0040e330(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr() + 0x34, 0);
    e.mem.set_u32(this.addr() + 0x38, 0xffff_ffff);
    e.mem.set_u32(this.addr() + 0x3c, 0);
    e.mem.set_u8(this.addr() + 0x40, 0);
}

// Translated from 0040e500 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: sets `SpellItemData::iCostOverride` (+0x38).
pub fn fn_0040e500(e: &mut Engine, this: Ptr, cost: u32) {
    e.mem.set_u32(this.addr() + 0x38, cost);
}

/// No arguments: `AL`, the byte at `011c54ba` (whether saved data has its
/// byte order swapped).
const SWAP_ON_SAVE_00401500: u32 = 0x0040_1500;
/// Thiscall on the 0x10 bytes at +0x1c (`005a6b10`, `tesloadscreen.cpp`
/// range): run before and after the chunk when saved data is byte-swapped.
const SWAP_DATA_005A6B10: u32 = 0x005a_6b10;
/// `TESForm::__AddChunkData` (engine map), cdecl (chunk tag, data, size).
const ADD_CHUNK_DATA: u32 = 0x0048_5990;
/// `__RTDynamicCast` (CRT), cdecl, and the type descriptors of `MagicItem`
/// and `SpellItem`.
const DYNAMIC_CAST: u32 = 0x00ec_43fb;
const TYPE_MAGIC_ITEM: u32 = 0x0118_3140;
const TYPE_SPELL_ITEM: u32 = 0x0118_3060;
/// The `MagicItem` virtual slot +0x18: a word (the spell type; `GetCost`
/// treats 2 and 3 apart).
const SLOT_MAGIC_ITEM_TYPE: u32 = 0x18;
/// Thiscall: the word at +0x38 (`iCostOverride`) and at +0x3c
/// (`eLevelOverride`) (folded code).
const WORD_AT_38: u32 = 0x009e_32d0;
const WORD_AT_3C: u32 = 0x0063_9b40;
/// Thiscall (word), `RET 4`: stores the word at +0x34 (folded code named
/// `LowProcess::SetItemBeingUsed` in the engine map) and at +0x3c
/// (`00409480`, `enchantmentitem.rs`).
const SET_WORD_AT_34: u32 = 0x008d_7dc0;
const SET_WORD_AT_3C: u32 = 0x0040_9480;
/// `ActorValueOwner::GetClampedActorValue` (engine map), thiscall on the
/// actor's value owner (+0xa4) (actor value), `RET 4`: an integer.
const GET_CLAMPED_ACTOR_VALUE: u32 = 0x0066_ef20;
/// `EffectItemList::GetAssociatedSkill` (`004065e0`) and
/// `EffectItemList::GetCost` (`004063e0`, actor, `RET 4`, `ST0`)
/// (`effectitemlist.rs`), thiscall.
const EFFECT_LIST_GET_ASSOCIATED_SKILL: u32 = 0x0040_65e0;
const EFFECT_LIST_GET_COST: u32 = 0x0040_63e0;
/// `MagicFormulas::AdjustMagicSpellCost` (engine map), cdecl (cost, skill,
/// value of actor value 0xb): a float in `ST0`.
const ADJUST_MAGIC_SPELL_COST: u32 = 0x0064_9180;

// Translated from 0040e1e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall` on the `MagicItem` base at +0x18):
/// whether the spell's flags (+0x40) lack flag 1 (`0040e210`).
pub fn fn_0040e1e0(e: &mut Engine, this: Ptr) -> bool {
    !fn_0040e210(e, Ptr::new(this.addr().wrapping_sub(0x18)), 1)
}

// Translated from 0040e230 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 8`): sets (when the byte
/// `set` is non-zero) or clears the bits of `mask` in `SpellItemData::iFlags`
/// (+0x40).
pub fn fn_0040e230(e: &mut Engine, this: Ptr, mask: u32, set: u8) {
    let flags = u32::from(e.mem.u8(this.addr() + 0x40));
    let flags = if set != 0 {
        flags | mask
    } else {
        flags & !mask
    };
    e.mem.set_u8(this.addr() + 0x40, flags as u8);
}

// Translated from 0040e360 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, plain `RET`): adds the 0x10 bytes
/// at +0x1c (the spell data) as a chunk (`TESForm::__AddChunkData`) tagged
/// with what the vtable slot +0x24 returns; when `00401500` says saved data
/// is byte-swapped, `005a6b10` runs on them before and after.
pub fn fn_0040e360(e: &mut Engine, this: Ptr) {
    let data = this.addr() + 0x1c;
    if e.call(SWAP_ON_SAVE_00401500, &args![]).bool() {
        e.call(SWAP_DATA_005A6B10, &args![data]);
    }
    let tag = e.vcall(this.addr(), 0x24, &args![]).u32();
    e.call(ADD_CHUNK_DATA, &args![tag, data, 0x10u32]);
    if e.call(SWAP_ON_SAVE_00401500, &args![]).bool() {
        e.call(SWAP_DATA_005A6B10, &args![data]);
    }
}

/// `other` (a `MagicItem`) as a `SpellItem` (`__RTDynamicCast`), or 0.
fn as_spell_item(e: &mut Engine, other: u32) -> u32 {
    e.call(
        DYNAMIC_CAST,
        &args![other, 0u32, TYPE_MAGIC_ITEM, TYPE_SPELL_ITEM, 0u32],
    )
    .u32()
}

// Translated from 0040e3c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall` on the `MagicItem` base at +0x18,
/// `RET 4`): whether `other` (a `MagicItem`) differs from this spell: true
/// when it is not a `SpellItem`, or when its spell type (the `MagicItem`
/// virtual slot +0x18), cost override (+0x38), level override (+0x3c; each
/// `other`'s first) or flags byte (+0x40) differs from this spell's.
pub fn fn_0040e3c0(e: &mut Engine, this: Ptr, other: u32) -> bool {
    let other = as_spell_item(e, other);
    if other == 0 {
        return true;
    }
    let theirs = e
        .vcall(other.wrapping_add(0x18), SLOT_MAGIC_ITEM_TYPE, &args![])
        .u32();
    let ours = e.vcall(this.addr(), SLOT_MAGIC_ITEM_TYPE, &args![]).u32();
    if theirs != ours {
        return true;
    }
    let form = this.addr().wrapping_sub(0x18);
    for getter in [WORD_AT_38, WORD_AT_3C] {
        let theirs = e.call(getter, &args![other]).u32();
        let ours = e.call(getter, &args![form]).u32();
        if theirs != ours {
            return true;
        }
    }
    e.mem.u8(other + 0x40) != e.mem.u8(this.addr() + 0x28)
}

// Translated from 0040e470 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall` on the `MagicItem` base at +0x18,
/// `RET 4`): when `other` (a `MagicItem`) is a `SpellItem`, copies its
/// spell type (the `MagicItem` virtual slot +0x18, stored at +0x34 by
/// `008d7dc0`), cost override (+0x38, `0040e500`), level override (+0x3c,
/// `00409480`) and flags byte (+0x40) into this spell; otherwise nothing.
pub fn fn_0040e470(e: &mut Engine, this: Ptr, other: u32) {
    let other = as_spell_item(e, other);
    if other == 0 {
        return;
    }
    let form = this.addr().wrapping_sub(0x18);
    let kind = e
        .vcall(other.wrapping_add(0x18), SLOT_MAGIC_ITEM_TYPE, &args![])
        .u32();
    e.call(SET_WORD_AT_34, &args![form, kind]);
    let cost = e.call(WORD_AT_38, &args![other]).u32();
    fn_0040e500(e, Ptr::new(form), cost);
    let level = e.call(WORD_AT_3C, &args![other]).u32();
    e.call(SET_WORD_AT_3C, &args![form, level]);
    let flags = e.mem.u8(other + 0x40);
    e.mem.set_u8(this.addr() + 0x28, flags);
}

// Translated from 0040e520 (decompiled, FalloutNV.exe 1.4.0.525)
/// `SpellItem::GetCost` (engine map; `thiscall` on the `EffectItemList` at
/// +0x24, `RET 4`, a float in `ST0`): the actor counts only when the spell
/// type (the `MagicItem` virtual slot +0x18, `this - 0xc`) is neither 2 nor
/// 3. With flag 1 of the spell's flags (`0040e210`) the cost is the cost
/// override (+0x38, `FILD`, stored as a `float`), adjusted by
/// `MagicFormulas::AdjustMagicSpellCost` with the actor's clamped values of
/// the list's associated skill and of actor value 0xb when there is an
/// actor; otherwise it is `EffectItemList::GetCost(actor)`.
pub fn spell_item_get_cost(e: &mut Engine, this: Ptr, actor: u32) -> f32 {
    let magic_item = this.addr().wrapping_sub(0xc);
    let actor = if e.vcall(magic_item, SLOT_MAGIC_ITEM_TYPE, &args![]).u32() == 2
        || e.vcall(magic_item, SLOT_MAGIC_ITEM_TYPE, &args![]).u32() == 3
    {
        0
    } else {
        actor
    };
    let form = this.addr().wrapping_sub(0x24);
    let cost = if fn_0040e210(e, Ptr::new(form), 1) {
        let override_cost = e.call(WORD_AT_38, &args![form]).i32();
        let mut cost = x87_store32(f64::from(override_cost));
        if actor != 0 {
            let values = actor.wrapping_add(0xa4);
            let value_b = e
                .call(GET_CLAMPED_ACTOR_VALUE, &args![values, 0xbu32])
                .u32();
            let skill = e.call(EFFECT_LIST_GET_ASSOCIATED_SKILL, &args![this]).u32();
            let skill_value = e.call(GET_CLAMPED_ACTOR_VALUE, &args![values, skill]).u32();
            let adjusted = e
                .call(
                    ADJUST_MAGIC_SPELL_COST,
                    &args![x87_quiet_bits(cost), skill_value, value_b],
                )
                .f64();
            cost = x87_store32(adjusted);
        }
        cost
    } else {
        let total = e.call(EFFECT_LIST_GET_COST, &args![this, actor]).f64();
        x87_store32(total)
    };
    f32::from_bits(x87_quiet_bits(cost))
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0040_e160, spellitem_spellitem(Ptr) -> Ptr),
        entry!(0x0061_e760, fn_0061e760(Ptr, Ptr) -> Ptr),
        entry!(0x0040_e290, fn_0040e290(Ptr, u32) -> Ptr),
        entry!(0x0058_e890, fn_0058e890(Ptr, u32) -> Ptr),
        entry!(0x005e_6440, fn_005e6440(Ptr, u32) -> Ptr),
        entry!(0x005e_7370, fn_005e7370(Ptr, u32) -> Ptr),
        entry!(0x005e_8140, fn_005e8140(Ptr, u32) -> Ptr),
        entry!(0x005e_9560, fn_005e9560(Ptr, u32) -> Ptr),
        entry!(0x005e_9830, fn_005e9830(Ptr, u32) -> Ptr),
        entry!(0x0040_e210, fn_0040e210(Ptr, u32) -> bool),
        entry!(0x0040_e280, fn_0040e280(Ptr) -> u32),
        entry!(0x0040_e2c0, fn_0040e2c0(Ptr)),
        entry!(0x0040_e330, fn_0040e330(Ptr)),
        entry!(0x0040_e500, fn_0040e500(Ptr, u32)),
        entry!(0x0040_e1e0, fn_0040e1e0(Ptr) -> bool),
        entry!(0x0040_e230, fn_0040e230(Ptr, u32, u8)),
        entry!(0x0040_e360, fn_0040e360(Ptr)),
        entry!(0x0040_e3c0, fn_0040e3c0(Ptr, u32) -> bool),
        entry!(0x0040_e470, fn_0040e470(Ptr, u32)),
        entry!(0x0040_e520, spell_item_get_cost(Ptr, u32) -> f32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::crt::ctor_test::{self, A};

    #[test]
    fn spellitem_spellitem_builds_the_object() {
        ctor_test::run(&ctor_test::Case {
            addr: 0x0040_e160,
            size: 0x44,
            globals: &[],
            returns: &[],
            calls: &[
                (0x0040_9190, &[A::T(0x0)]),
                (0x004f_15a0, &[A::T(0x0), A::V(0x14)]),
                (0x0040_e330, &[A::T(0x0)]),
            ],
            stores: &[
                (0x0, 4, 0x0101_3f8c),
                (0x18, 4, 0x0101_3f44),
                (0x24, 4, 0x0101_3f28),
            ],
        });
    }

    #[test]
    fn forwarder_fn_0061e760_forwards_this_and_the_argument() {
        let mut e = Engine::new();
        e.register(RESPONSE_LIST_TARGET_0061E780, |_, a| {
            (0x4000_0000 | a[1]).into_ret()
        });
        e.call_log = Some(vec![]);
        let result = e.call(0x0061_e760, &args![0x1000u32, 0x2000u32]);
        let log = e.call_log.take().unwrap();
        let hits: Vec<_> = log
            .iter()
            .filter(|call| call.0 == RESPONSE_LIST_TARGET_0061E780)
            .collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, vec![0x1000u32, 0x2000]);
        assert_eq!(result.u32(), 0x4000_2000);
    }
    #[test]
    fn at_0040e290_runs_destructor_body_then_deletes_on_flag_one() {
        use crate::prelude::*;

        let mut e = Engine::new();
        let this: Ptr = Ptr::new(e.mem.alloc(16));
        e.register(0x0040_e2c0, |_, _| Ret::default());
        e.register(0x0040_1030, |_, _| Ret::default());
        for (flags, deletes) in [(0u32, false), (2, false), (1, true), (0xffff_ffff, true)] {
            e.call_log = Some(vec![]);
            let result = e.call(0x0040_e290, &args![this, flags]);
            assert_eq!(result.u32(), this.addr());
            let log = e.call_log.take().unwrap();
            let calls: Vec<(u32, Vec<u32>)> = log
                .into_iter()
                .filter(|(address, _)| *address != 0x0040_e290)
                .collect();
            let mut expected = vec![(0x0040_e2c0, vec![this.addr()])];
            if deletes {
                expected.push((0x0040_1030, vec![this.addr()]));
            }
            assert_eq!(calls, expected);
        }
    }

    #[test]
    fn at_0058e890_runs_destructor_body_then_deletes_on_flag_one() {
        use crate::prelude::*;

        let mut e = Engine::new();
        let this: Ptr = Ptr::new(e.mem.alloc(16));
        e.register(0x0058_e8c0, |_, _| Ret::default());
        e.register(0x0040_1030, |_, _| Ret::default());
        for (flags, deletes) in [(0u32, false), (2, false), (1, true), (0xffff_ffff, true)] {
            e.call_log = Some(vec![]);
            let result = e.call(0x0058_e890, &args![this, flags]);
            assert_eq!(result.u32(), this.addr());
            let log = e.call_log.take().unwrap();
            let calls: Vec<(u32, Vec<u32>)> = log
                .into_iter()
                .filter(|(address, _)| *address != 0x0058_e890)
                .collect();
            let mut expected = vec![(0x0058_e8c0, vec![this.addr()])];
            if deletes {
                expected.push((0x0040_1030, vec![this.addr()]));
            }
            assert_eq!(calls, expected);
        }
    }

    #[test]
    fn at_005e6440_runs_destructor_body_then_deletes_on_flag_one() {
        use crate::prelude::*;

        let mut e = Engine::new();
        let this: Ptr = Ptr::new(e.mem.alloc(16));
        e.register(0x005e_6470, |_, _| Ret::default());
        e.register(0x0040_1030, |_, _| Ret::default());
        for (flags, deletes) in [(0u32, false), (2, false), (1, true), (0xffff_ffff, true)] {
            e.call_log = Some(vec![]);
            let result = e.call(0x005e_6440, &args![this, flags]);
            assert_eq!(result.u32(), this.addr());
            let log = e.call_log.take().unwrap();
            let calls: Vec<(u32, Vec<u32>)> = log
                .into_iter()
                .filter(|(address, _)| *address != 0x005e_6440)
                .collect();
            let mut expected = vec![(0x005e_6470, vec![this.addr()])];
            if deletes {
                expected.push((0x0040_1030, vec![this.addr()]));
            }
            assert_eq!(calls, expected);
        }
    }

    #[test]
    fn at_005e7370_runs_destructor_body_then_deletes_on_flag_one() {
        use crate::prelude::*;

        let mut e = Engine::new();
        let this: Ptr = Ptr::new(e.mem.alloc(16));
        e.register(0x005e_73a0, |_, _| Ret::default());
        e.register(0x0040_1030, |_, _| Ret::default());
        for (flags, deletes) in [(0u32, false), (2, false), (1, true), (0xffff_ffff, true)] {
            e.call_log = Some(vec![]);
            let result = e.call(0x005e_7370, &args![this, flags]);
            assert_eq!(result.u32(), this.addr());
            let log = e.call_log.take().unwrap();
            let calls: Vec<(u32, Vec<u32>)> = log
                .into_iter()
                .filter(|(address, _)| *address != 0x005e_7370)
                .collect();
            let mut expected = vec![(0x005e_73a0, vec![this.addr()])];
            if deletes {
                expected.push((0x0040_1030, vec![this.addr()]));
            }
            assert_eq!(calls, expected);
        }
    }

    #[test]
    fn at_005e8140_runs_destructor_body_then_deletes_on_flag_one() {
        use crate::prelude::*;

        let mut e = Engine::new();
        let this: Ptr = Ptr::new(e.mem.alloc(16));
        e.register(0x005e_8170, |_, _| Ret::default());
        e.register(0x0040_1030, |_, _| Ret::default());
        for (flags, deletes) in [(0u32, false), (2, false), (1, true), (0xffff_ffff, true)] {
            e.call_log = Some(vec![]);
            let result = e.call(0x005e_8140, &args![this, flags]);
            assert_eq!(result.u32(), this.addr());
            let log = e.call_log.take().unwrap();
            let calls: Vec<(u32, Vec<u32>)> = log
                .into_iter()
                .filter(|(address, _)| *address != 0x005e_8140)
                .collect();
            let mut expected = vec![(0x005e_8170, vec![this.addr()])];
            if deletes {
                expected.push((0x0040_1030, vec![this.addr()]));
            }
            assert_eq!(calls, expected);
        }
    }

    #[test]
    fn at_005e9560_runs_destructor_body_then_deletes_on_flag_one() {
        use crate::prelude::*;

        let mut e = Engine::new();
        let this: Ptr = Ptr::new(e.mem.alloc(16));
        e.register(0x005e_9590, |_, _| Ret::default());
        e.register(0x0040_1030, |_, _| Ret::default());
        for (flags, deletes) in [(0u32, false), (2, false), (1, true), (0xffff_ffff, true)] {
            e.call_log = Some(vec![]);
            let result = e.call(0x005e_9560, &args![this, flags]);
            assert_eq!(result.u32(), this.addr());
            let log = e.call_log.take().unwrap();
            let calls: Vec<(u32, Vec<u32>)> = log
                .into_iter()
                .filter(|(address, _)| *address != 0x005e_9560)
                .collect();
            let mut expected = vec![(0x005e_9590, vec![this.addr()])];
            if deletes {
                expected.push((0x0040_1030, vec![this.addr()]));
            }
            assert_eq!(calls, expected);
        }
    }

    #[test]
    fn at_005e9830_runs_destructor_body_then_deletes_on_flag_one() {
        use crate::prelude::*;

        let mut e = Engine::new();
        let this: Ptr = Ptr::new(e.mem.alloc(16));
        e.register(0x005e_9860, |_, _| Ret::default());
        e.register(0x0040_1030, |_, _| Ret::default());
        for (flags, deletes) in [(0u32, false), (2, false), (1, true), (0xffff_ffff, true)] {
            e.call_log = Some(vec![]);
            let result = e.call(0x005e_9830, &args![this, flags]);
            assert_eq!(result.u32(), this.addr());
            let log = e.call_log.take().unwrap();
            let calls: Vec<(u32, Vec<u32>)> = log
                .into_iter()
                .filter(|(address, _)| *address != 0x005e_9830)
                .collect();
            let mut expected = vec![(0x005e_9860, vec![this.addr()])];
            if deletes {
                expected.push((0x0040_1030, vec![this.addr()]));
            }
            assert_eq!(calls, expected);
        }
    }
}

#[cfg(test)]
mod spell_data_tests {
    use super::*;

    #[test]
    fn flag_test_ands_the_whole_word() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x44);
        e.mem.set_u8(this + 0x40, 0x05);
        assert!(e.call(0x0040_e210, &args![this, 4u32]).bool());
        assert!(!e.call(0x0040_e210, &args![this, 0x102u32]).bool());
        assert!(e.call(0x0040_e210, &args![this, 0x101u32]).bool());
    }

    #[test]
    fn chunk_tag() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x0040_e280, &args![0u32]).u32(), 0x5449_5053);
    }

    #[test]
    fn destructor_body_sets_the_vtables_and_unwinds() {
        let mut e = Engine::new();
        e.register(REMOVE_EFFECTS_00409340, |_, _| Ret::default());
        e.register(MAGIC_ITEM_FORM_DESTRUCT, |_, _| Ret::default());
        let this = e.mem.alloc(0x44);
        e.call_log = Some(vec![]);
        e.call(0x0040_e2c0, &args![this]);
        assert_eq!(e.mem.u32(this), 0x0101_3f8c);
        assert_eq!(e.mem.u32(this + 0x18), 0x0101_3f44);
        assert_eq!(e.mem.u32(this + 0x24), 0x0101_3f28);
        let log = e.call_log.take().unwrap();
        let order: Vec<u32> = log.iter().skip(1).map(|c| c.0).collect();
        assert_eq!(
            order,
            vec![REMOVE_EFFECTS_00409340, MAGIC_ITEM_FORM_DESTRUCT]
        );
        assert!(log.iter().skip(1).all(|c| c.1 == vec![this]));
    }

    #[test]
    fn data_start_values_and_cost_override() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x44);
        e.mem.set_u32(this + 0x34, 9);
        e.mem.set_u8(this + 0x40, 9);
        e.call(0x0040_e330, &args![this]);
        assert_eq!(e.mem.u32(this + 0x34), 0);
        assert_eq!(e.mem.u32(this + 0x38), 0xffff_ffff);
        assert_eq!(e.mem.u32(this + 0x3c), 0);
        assert_eq!(e.mem.u8(this + 0x40), 0);
        e.call(0x0040_e500, &args![this, 40u32]);
        assert_eq!(e.mem.u32(this + 0x38), 40);
    }
}

#[cfg(test)]
mod spell_n050_tests {
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

    const TYPE_SLOT_FUNCTION: u32 = 0x00f2_0000;

    /// A spell form: the `MagicItem` base at +0x18 with a vtable whose slot
    /// +0x18 answers the word at form +0x80; the given data words.
    fn spell(e: &mut Engine, kind: u32, cost: u32, level: u32, flags: u8) -> u32 {
        let form = e.mem.alloc(0x90);
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 0x18, TYPE_SLOT_FUNCTION);
        e.mem.set_u32(form + 0x18, vtable);
        e.mem.set_u32(form + 0x80, kind);
        e.mem.set_u32(form + 0x38, cost);
        e.mem.set_u32(form + 0x3c, level);
        e.mem.set_u8(form + 0x40, flags);
        form
    }

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.register(TYPE_SLOT_FUNCTION, |e, a| {
            e.mem.u32(a[0] - 0x18 + 0x80).into_ret()
        });
        e.register(WORD_AT_38, |e, a| e.mem.u32(a[0] + 0x38).into_ret());
        e.register(WORD_AT_3C, |e, a| e.mem.u32(a[0] + 0x3c).into_ret());
        e.register(SET_WORD_AT_34, |e, a| {
            e.mem.set_u32(a[0] + 0x34, a[1]);
            Ret::default()
        });
        e.register(SET_WORD_AT_3C, |e, a| {
            e.mem.set_u32(a[0] + 0x3c, a[1]);
            Ret::default()
        });
        e.register(DYNAMIC_CAST, |_, a| {
            if a[0] == 0x9000 {
                Ret::default()
            } else {
                (a[0] - 0x18).into_ret()
            }
        });
        e
    }

    #[test]
    fn lacks_flag_one() {
        let mut e = Engine::new();
        let form = e.mem.alloc(0x44);
        e.mem.set_u8(form + 0x40, 0xfe);
        assert!(e.call(0x0040_e1e0, &args![form + 0x18]).bool());
        e.mem.set_u8(form + 0x40, 0x01);
        assert!(!fn_0040e1e0(&mut e, Ptr::new(form + 0x18)));
    }

    #[test]
    fn flag_updates() {
        let mut e = Engine::new();
        let form = e.mem.alloc(0x44);
        e.mem.set_u8(form + 0x40, 0x10);
        e.call(0x0040_e230, &args![form, 0x0103u32, 0x200u32]);
        assert_eq!(e.mem.u8(form + 0x40), 0x10);
        e.call(0x0040_e230, &args![form, 0x0103u32, 1u32]);
        assert_eq!(e.mem.u8(form + 0x40), 0x13);
        fn_0040e230(&mut e, Ptr::new(form), 0x11, 0);
        assert_eq!(e.mem.u8(form + 0x40), 0x02);
    }

    #[test]
    fn data_chunk_with_and_without_swapping() {
        let mut e = Engine::new();
        let vtable = e.mem.alloc(0x40);
        e.mem.set_u32(vtable + 0x24, 0x0056_0000);
        e.register(0x0056_0000, |_, _| 0x5449_5053u32.into_ret());
        e.register(ADD_CHUNK_DATA, |_, _| Ret::default());
        e.register(SWAP_DATA_005A6B10, |_, _| Ret::default());
        e.register(SWAP_ON_SAVE_00401500, |_, _| Ret::default());
        let this = e.mem.alloc(0x44);
        e.mem.set_u32(this, vtable);
        e.call_log = Some(vec![]);
        e.call(0x0040_e360, &args![this]);
        assert_eq!(
            calls(&e, ADD_CHUNK_DATA),
            vec![vec![0x5449_5053, this + 0x1c, 0x10]]
        );
        assert!(calls(&e, SWAP_DATA_005A6B10).is_empty());
        e.register(SWAP_ON_SAVE_00401500, |_, _| 1u32.into_ret());
        e.call_log = Some(vec![]);
        fn_0040e360(&mut e, Ptr::new(this));
        assert_eq!(
            calls(&e, SWAP_DATA_005A6B10),
            vec![vec![this + 0x1c], vec![this + 0x1c]]
        );
    }

    #[test]
    fn spells_differ_by_any_value() {
        let mut e = engine();
        let this = spell(&mut e, 1, 2, 3, 4);
        let same = spell(&mut e, 1, 2, 3, 4);
        assert!(!e.call(0x0040_e3c0, &args![this + 0x18, same + 0x18]).bool());
        assert!(fn_0040e3c0(&mut e, Ptr::new(this + 0x18), 0x9000));
        for (kind, cost, level, flags) in [(9, 2, 3, 4), (1, 9, 3, 4), (1, 2, 9, 4), (1, 2, 3, 9)] {
            let other = spell(&mut e, kind, cost, level, flags);
            assert!(fn_0040e3c0(&mut e, Ptr::new(this + 0x18), other + 0x18));
        }
    }

    #[test]
    fn copy_takes_the_other_spells_values() {
        let mut e = engine();
        let this = spell(&mut e, 0, 0, 0, 0);
        let other = spell(&mut e, 3, 0x55, 0x66, 0x81);
        e.call(0x0040_e470, &args![this + 0x18, other + 0x18]);
        assert_eq!(e.mem.u32(this + 0x34), 3);
        assert_eq!(e.mem.u32(this + 0x38), 0x55);
        assert_eq!(e.mem.u32(this + 0x3c), 0x66);
        assert_eq!(e.mem.u8(this + 0x40), 0x81);
        e.call_log = Some(vec![]);
        fn_0040e470(&mut e, Ptr::new(this + 0x18), 0x9000);
        assert!(calls(&e, SET_WORD_AT_34).is_empty());
    }

    #[test]
    fn cost_override_adjusted_for_the_actor_or_effects_cost() {
        let mut e = engine();
        e.register(GET_CLAMPED_ACTOR_VALUE, |_, a| (a[1] * 10).into_ret());
        e.register(EFFECT_LIST_GET_ASSOCIATED_SKILL, |_, _| 0x20u32.into_ret());
        e.register(ADJUST_MAGIC_SPELL_COST, |_, a| Ret {
            st0: f64::from(f32::from_bits(a[0])) + f64::from(a[1]) + f64::from(a[2]) / 1000.0,
            ..Ret::default()
        });
        e.register(EFFECT_LIST_GET_COST, |_, a| Ret {
            st0: f64::from(a[1]) + 0.5,
            ..Ret::default()
        });
        let actor = 0x4000u32;
        let form = spell(&mut e, 0, 16_777_217, 0, 1);
        // The override is stored as a float (2^24 + 1 rounds to 2^24).
        e.call_log = Some(vec![]);
        let cost = e.call(0x0040_e520, &args![form + 0x24, actor]).f32();
        assert_eq!(cost, 16_777_216.0 + 320.0);
        assert_eq!(
            calls(&e, GET_CLAMPED_ACTOR_VALUE),
            vec![vec![actor + 0xa4, 0xb], vec![actor + 0xa4, 0x20]]
        );
        let adjusted = calls(&e, ADJUST_MAGIC_SPELL_COST);
        assert_eq!(adjusted[0][0], 16_777_216.0f32.to_bits());
        // Spell types 2 and 3 ignore the actor: no adjustment.
        for kind in [2, 3] {
            e.mem.set_u32(form + 0x80, kind);
            e.call_log = Some(vec![]);
            assert_eq!(
                spell_item_get_cost(&mut e, Ptr::new(form + 0x24), actor),
                16_777_216.0
            );
            assert!(calls(&e, ADJUST_MAGIC_SPELL_COST).is_empty());
        }
        // Without the override: the effects' cost with the actor (or 0).
        e.mem.set_u8(form + 0x40, 0);
        e.call_log = Some(vec![]);
        assert_eq!(
            spell_item_get_cost(&mut e, Ptr::new(form + 0x24), actor),
            0.5
        );
        assert_eq!(calls(&e, EFFECT_LIST_GET_COST), vec![vec![form + 0x24, 0]]);
        e.mem.set_u32(form + 0x80, 1);
        assert_eq!(spell_item_get_cost(&mut e, Ptr::new(form + 0x24), 7), 7.5);
    }
}

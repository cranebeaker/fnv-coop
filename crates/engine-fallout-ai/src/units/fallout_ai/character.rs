//! `fallout/ai/character.cpp` (Xbox PDB source unit), subsystem `fallout/ai`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

use crate::units::fallout_ai::actor::Actor;

/// `Actor::Actor` (`0087d4a0`): the base constructor, one stack word
/// (`add_to_process_lists`).
const ACTOR_CONSTRUCT: u32 = 0x0087_d4a0;
/// Writes `TESForm::cFormType` (`004f15a0`, one stack word).
const SET_FORM_TYPE: u32 = 0x004f_15a0;
/// Sets the armour rating and threshold to -1.0 (`008d4e80`).
const RESET_ARMOUR: u32 = 0x008d_4e80;
/// The `float` constant -1.0 the code loads from the exe's data.
const MINUS_ONE: u32 = 0x0101_2054;

/// The vtables `Character` installs: its own (+0x00), the `TESChildCell`
/// part (+0x18), `MagicCaster` (+0x88), `MagicTarget` (+0x94),
/// `ActorValueOwner` (+0xa4) and `CachedValuesOwner` (+0xa8), Xbox PDB part
/// names at the PC offsets.
const VTABLES: [(u32, u32); 6] = [
    (0x00, 0x0108_6a6c),
    (0x18, 0x0108_6a60),
    (0x88, 0x0108_6a0c),
    (0x94, 0x0108_69d4),
    (0xa4, 0x0108_69a4),
    (0xa8, 0x0108_695c),
];
/// The form type of a `Character` (`ACHR`).
const FORM_TYPE_CHARACTER: u32 = 0x3b;

layout! {
    /// `Character` (Xbox PDB), 0x1c8 bytes on PC (the Xbox PDB's size);
    /// derived from [`Actor`], whose fields come first (use the `Actor`
    /// layout for them). Only the fields the constructor sets are listed.
    pub struct Character: 0x1c8 {
        /// `pBiped` (Xbox PDB): `BipedAnim*`.
        0x1B4 pBiped: Ptr,
        /// `fArmorRating` (Xbox PDB).
        0x1B8 fArmorRating: f32,
        /// `fArmorThreshold` (Xbox PDB).
        0x1BC fArmorThreshold: f32,
        /// `bTrespassing` (Xbox PDB).
        0x1C0 bTrespassing: bool,
        /// `bGuard` (Xbox PDB).
        0x1C1 bGuard: bool,
        /// `fequippedweight` (Xbox PDB).
        0x1C4 fequippedweight: f32,
    }
}

// Translated from 008d1d30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Character::Character` (Xbox PDB): constructs the `Actor` base (with
/// `add_to_process_lists` 1), installs the six vtables, sets the form type to
/// 0x3b, clears `pBiped`, sets the armour rating and threshold to -1.0
/// (`008d4e80`), clears `bGuard`, sets `fequippedweight` to -1.0, clears
/// `bTrespassing` and the actor's crime fields (`bAttackOnNextTheft`,
/// `iThiefCrimeStamp`, `iMinorCrimes`, `iMajorCrimes`, `bIgnoreCrime`).
/// Returns `this`.
// Translation incomplete: 008d1d30 - SEH exception unwinding and the stack security cookie are unsupported.
pub fn character_character(e: &mut Engine, this: Ptr<Character>) -> Ptr<Character> {
    e.call(ACTOR_CONSTRUCT, &args![this, 1u32]);
    for (offset, vtable) in VTABLES {
        e.mem.set_u32(this.addr() + offset, vtable);
    }
    e.call(SET_FORM_TYPE, &args![this, FORM_TYPE_CHARACTER]);
    e.set(this, Character::pBiped, Ptr::NULL);
    e.call(RESET_ARMOUR, &args![this]);
    e.set(this, Character::bGuard, false);
    let minus_one: f32 = e.global(MINUS_ONE);
    e.set(this, Character::fequippedweight, minus_one);
    e.set(this, Character::bTrespassing, false);
    let actor = this.cast::<Actor>();
    e.set(actor, Actor::bAttackOnNextTheft, false);
    e.set(actor, Actor::iThiefCrimeStamp, 0);
    e.set(actor, Actor::iMinorCrimes, 0);
    e.set(actor, Actor::iMajorCrimes, 0);
    e.set(actor, Actor::bIgnoreCrime, false);
    this
}

// ---------------------------------------------------------------------------
// Callees outside this file, by exe address.
// ---------------------------------------------------------------------------

/// The player singleton pointer (`Actor` global the code compares `this` to).
const PLAYER_SINGLETON: u32 = 0x011d_ea3c;
/// The `double` 0.0 the code compares floats against.
const ZERO_DOUBLE: u32 = 0x0101_2060;
/// `operator new(size)` (`00401000`, cdecl).
const OPERATOR_NEW: u32 = 0x0040_1000;
/// `operator delete(block)` (`00401030`, cdecl).
const OPERATOR_DELETE: u32 = 0x0040_1030;
/// `__RTDynamicCast(object, 0, source type, target type, 0)` (`00ec43fb`).
const RT_DYNAMIC_CAST: u32 = 0x00ec_43fb;
/// The compiler's `_ftol2` (`00ec62c0`): the value in `ST0` as a leading
/// `f64` argument, the truncated `int` in `EAX`.
const FLOAT_TO_INT: u32 = 0x00ec_62c0;
/// The process of an actor (`008d8520`: the word at `+0x68`; the engine map
/// names it `MiddleHighProcess::GetSavedAcquireObject`, but the body is the
/// folded getter of `Actor::pCurrentProcess`).
const ACTOR_PROCESS: u32 = 0x008d_8520;
/// `MobileObject::GetCurrentPackage` (Xbox PDB name, `009344a0`).
const CURRENT_PACKAGE: u32 = 0x0093_44a0;
/// `TESPackage::IsInterruptPackage` (Xbox PDB name, `00678610`).
const IS_INTERRUPT_PACKAGE: u32 = 0x0067_8610;
/// `Actor::EndInterruptPackage` (Xbox PDB name, `00881680`): `(actor, flag)`.
const END_INTERRUPT_PACKAGE: u32 = 0x0088_1680;
/// `MobileObject::IsinDialogue` (Xbox PDB name, `009336c0`).
const IS_IN_DIALOGUE: u32 = 0x0093_36c0;
/// `Actor::ShouldSkipFallOutBehavior` (Xbox PDB name, `008a78f0`).
const SHOULD_SKIP_FALL_OUT_BEHAVIOR: u32 = 0x008a_78f0;
/// `Actor::IsAlarmed` (Xbox PDB name, `008a61b0`).
const IS_ALARMED: u32 = 0x008a_61b0;
/// `Actor::IsFleeing` (Xbox PDB name, `008a6650`): `(actor, flag)`.
const IS_FLEEING: u32 = 0x008a_6650;
/// A predicate on an actor (`00493bb0`, `fallout shared/animation.cpp` range
/// of the map, no name).
const ACTOR_PREDICATE_493BB0: u32 = 0x0049_3bb0;
/// The word at `+0x08` of an object (`0044ddc0`; a folded getter, used here
/// for the form of a worn item and for the count of a list).
const WORD_AT_8: u32 = 0x0044_ddc0;
/// The getter of the extra-data list of a reference (`005d43c0`: `this + 0x44`).
const EXTRA_DATA_LIST: u32 = 0x005d_43c0;
/// `ExtraDataList::GetContainerChanges` (Xbox PDB name, `00418520`).
const GET_CONTAINER_CHANGES: u32 = 0x0041_8520;
/// `InventoryChanges::GetWornItem` (Xbox PDB name, `004c8c10`):
/// `(changes, slot, 0)`; the result is released with [`RELEASE_WORN_ITEM`].
const GET_WORN_ITEM: u32 = 0x004c_8c10;
/// The release of a worn-item result (`004459e0`, `(item, 1)`).
const RELEASE_WORN_ITEM: u32 = 0x0044_59e0;
/// The value of a worn item toward the damage threshold (`004be0b0`,
/// `(item, owner)`, result in `ST0`).
const WORN_ITEM_THRESHOLD: u32 = 0x004b_e0b0;
/// The value of a worn item toward the armour rating (`004bdf90`,
/// `(item, owner)`, result in `ST0`).
const WORN_ITEM_RATING: u32 = 0x004b_df90;
/// `ItemChange::GetItemValue` (Xbox PDB name, `004bd400`), result in `ST0`.
const ITEM_VALUE: u32 = 0x004b_d400;
/// The pointer to a `Setting`'s value (`00403e20`: `setting + 4`, or a
/// zeroed static when the setting is null).
const SETTING_VALUE: u32 = 0x0040_3e20;
/// `min`-like helper `(a, b)` of two `float`s (`0040ebd0`): `b` unless
/// `b > a`.
const MIN_OF_TWO: u32 = 0x0040_ebd0;
/// `CombatFormulas::CalcHandToHandDamage` (Xbox PDB name, `00646310`):
/// `(actor value owner, 0, &out, &out)`.
const CALC_HAND_TO_HAND_DAMAGE: u32 = 0x0064_6310;
/// The setting object whose value caps the armour rating (`011d1020`).
const SETTING_ARMOUR_CAP: u32 = 0x011d_1020;
/// The settings the clothing value uses (armour: scale, base; clothing:
/// scale, base): `011d0ca8`, `011d1104`, `011d0d14`, `011d043c`.
const SETTINGS_ARMOUR_VALUE: [u32; 2] = [0x011d_0ca8, 0x011d_1104];
const SETTINGS_CLOTHING_VALUE: [u32; 2] = [0x011d_0d14, 0x011d_043c];
/// The setting of `008d3290` (`011cd0fc`).
const SETTING_OF_8D3290: u32 = 0x011c_d0fc;
/// RTTI type descriptor of `TESBoundObject` (`01183108`), the source type of
/// the casts; of `TESObjectARMO` (`01183a34`); of `TESObjectCLOT`
/// (`01186fcc`). All three names read from the descriptors' decorated names.
const TYPE_BOUND_OBJECT: u32 = 0x0118_3108;
const TYPE_ARMOUR: u32 = 0x0118_3a34;
const TYPE_CLOTHING: u32 = 0x0118_6fcc;
/// The cap of the clothing value (`double` `01017a40`) and the `float` it is
/// then set to (`01016410`).
const CLOTHING_CAP_COMPARE: u32 = 0x0101_7a40;
const CLOTHING_CAP_VALUE: u32 = 0x0101_6410;
/// The first word at `+4` of the structure (`00726070`).
const READ_WORD_AT_4: u32 = 0x0072_6070;
/// The word at `+0x20` of a reference (`007af430`).
const WORD_AT_20: u32 = 0x007a_f430;
/// A list element getter (`00559450`): the word at `+0` of its `this`.
const WORD_AT_0: u32 = 0x0055_9450;

/// Offset of the structure inside the form that `fn_008d1e50` reads from.
const FORM_SUBSTRUCTURE: u32 = 0x10c;

fn process_of(e: &Engine, this: Ptr<Character>) -> u32 {
    e.get(this.cast::<Actor>(), Actor::pCurrentProcess).addr()
}

fn acquire_process(e: &mut Engine, this: Ptr<Character>) -> u32 {
    e.call(ACTOR_PROCESS, &args![this]).u32()
}

// Translated from 008d1e50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (a virtual slot of the `Character` vtable):
/// the word at `+4` of the structure at `+0x10c` of the form at `this + 0x20`
/// (`007af430`, then `00726070`).
pub fn fn_008d1e50(e: &mut Engine, this: Ptr<Character>) -> u32 {
    let form = e.call(WORD_AT_20, &args![this]).u32();
    e.call(READ_WORD_AT_4, &args![form + FORM_SUBSTRUCTURE])
        .u32()
}

// Translated from 008d1e70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: `bTrespassing` (Xbox PDB field `+0x1c0`).
pub fn fn_008d1e70(e: &mut Engine, this: Ptr<Character>) -> bool {
    e.get(this, Character::bTrespassing)
}

// Translated from 008d1e90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: stores `fequippedweight` (`+0x1c4`).
pub fn fn_008d1e90(e: &mut Engine, this: Ptr<Character>, value: f32) {
    e.set(this, Character::fequippedweight, x87_quiet(value));
}

// Translated from 008d1eb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: reads `fequippedweight` (`+0x1c4`).
pub fn fn_008d1eb0(e: &mut Engine, this: Ptr<Character>) -> f32 {
    e.get(this, Character::fequippedweight)
}

// Translated from 008d1ed0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: reads `bGuard` (`+0x1c1`).
pub fn fn_008d1ed0(e: &mut Engine, this: Ptr<Character>) -> bool {
    e.get(this, Character::bGuard)
}

// Translated from 008d1ef0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: stores the low byte of its argument into `bGuard` (`+0x1c1`).
pub fn fn_008d1ef0(e: &mut Engine, this: Ptr<Character>, value: u8) {
    e.mem.set_u8(this.addr() + 0x1c1, value);
}

// Translated from 008d1f10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: the scalar deleting destructor shape: runs the body of
/// `008d2060`, then releases the block (`operator delete`, `00401030`) when
/// bit 0 of the flags is set. Returns `this`.
pub fn fn_008d1f10(e: &mut Engine, this: Ptr<Character>, flags: u32) -> Ptr<Character> {
    fn_008d2060(e, this);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 008d1f40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Character::Character` second overload (Xbox PDB name `Character_ov2`): the
/// same body as `008d1d30`, but the argument `add_to_process_lists` (one
/// byte) is passed on to the `Actor` constructor instead of the constant 1.
/// Returns `this`.
// Translation incomplete: 008d1f40 - SEH exception unwinding (FS:[0] frame, states -1/0) is unsupported.
pub fn character_character_ov2(
    e: &mut Engine,
    this: Ptr<Character>,
    add_to_process_lists: u8,
) -> Ptr<Character> {
    e.call(ACTOR_CONSTRUCT, &args![this, add_to_process_lists]);
    for (offset, vtable) in VTABLES {
        e.mem.set_u32(this.addr() + offset, vtable);
    }
    e.call(SET_FORM_TYPE, &args![this, FORM_TYPE_CHARACTER]);
    e.set(this, Character::pBiped, Ptr::NULL);
    e.call(RESET_ARMOUR, &args![this]);
    e.set(this, Character::bGuard, false);
    let minus_one: f32 = e.global(MINUS_ONE);
    e.set(this, Character::fequippedweight, minus_one);
    e.set(this, Character::bTrespassing, false);
    let actor = this.cast::<Actor>();
    e.set(actor, Actor::bAttackOnNextTheft, false);
    e.set(actor, Actor::iThiefCrimeStamp, 0);
    e.set(actor, Actor::iMinorCrimes, 0);
    e.set(actor, Actor::iMajorCrimes, 0);
    e.set(actor, Actor::bIgnoreCrime, false);
    this
}

// Translated from 008d2060 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (the destructor body of `Character`, called by `008d1f10`):
/// restores the six `Character` vtables, releases the 3D (`TESObjectREFR::Set3D`
/// with two zero words, `005702e0`) unless bit `0x4000` of the word at `+8`
/// is set (`004077c0`), then runs the `Actor` destructor (`0087dcb0`).
// Translation incomplete: 008d2060 - SEH exception unwinding (FS:[0] frame) is unsupported.
pub fn fn_008d2060(e: &mut Engine, this: Ptr<Character>) {
    for (offset, vtable) in VTABLES {
        e.mem.set_u32(this.addr() + offset, vtable);
    }
    if !e.call(0x0040_77c0, &args![this]).bool() {
        e.call(0x0057_02e0, &args![this, 0u32, 0u32]);
    }
    e.call(0x0087_dcb0, &args![this]);
}

/// Walks the 20 worn-item slots as `008d2110` and `008d22b0` do: every worn
/// item that casts to the armour type and was not seen before (compared to
/// the up-to-20 casts already collected) adds `per_item(item, owner)` to the
/// float field at `total`; each item is released afterwards.
fn add_worn_armour(
    e: &mut Engine,
    this: Ptr<Character>,
    total: Field<Character, f32>,
    per_item: u32,
) {
    let mut seen: Vec<u32> = Vec::new();
    e.set(this, total, 0.0);
    let list = e.call(EXTRA_DATA_LIST, &args![this]).u32();
    let changes = e.call(GET_CONTAINER_CHANGES, &args![list]).u32();
    if changes == 0 {
        return;
    }
    for slot in 0..0x14i32 {
        let item = e.call(GET_WORN_ITEM, &args![changes, slot, 0u32]).u32();
        if item == 0 {
            continue;
        }
        let form = e.call(WORD_AT_8, &args![item]).u32();
        let armour = e
            .call(
                RT_DYNAMIC_CAST,
                &args![
                    Ptr::<()>::new(form),
                    0i32,
                    Ptr::<()>::new(TYPE_BOUND_OBJECT),
                    Ptr::<()>::new(TYPE_ARMOUR),
                    0i32
                ],
            )
            .u32();
        if armour != 0 && !seen.contains(&armour) {
            // `this + 0xa4` (the actor value owner), or 0 when `this` is null.
            let owner = if this.is_null() {
                0
            } else {
                this.addr() + 0xa4
            };
            let value = e.call(per_item, &args![item, owner]).f64();
            let sum = value + f64::from(e.get(this, total));
            e.set(this, total, sum as f32);
            seen.push(armour);
        }
        e.call(RELEASE_WORN_ITEM, &args![item, 1u32]);
    }
}

// Translated from 008d2110 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (the damage threshold of the worn armour): when `fArmorThreshold`
/// (`+0x1bc`) is below 0.0 it is recomputed, as the sum of `004be0b0` over
/// the distinct armour pieces among the 20 worn-item slots; returns the
/// field.
pub fn fn_008d2110(e: &mut Engine, this: Ptr<Character>) -> f32 {
    let zero: f64 = e.global(ZERO_DOUBLE);
    if f64::from(e.get(this, Character::fArmorThreshold)) < zero {
        add_worn_armour(e, this, Character::fArmorThreshold, WORN_ITEM_THRESHOLD);
    }
    e.get(this, Character::fArmorThreshold)
}

// Translated from 008d22b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (the armour rating of the worn armour): when `fArmorRating`
/// (`+0x1b8`) is below 0.0 it is recomputed as the sum of `004bdf90` over the
/// distinct armour pieces; the result is capped by the setting at `011d1020`
/// (`min` helper `0040ebd0`) when that setting's value is above 0.0.
pub fn fn_008d22b0(e: &mut Engine, this: Ptr<Character>) -> f32 {
    let zero: f64 = e.global(ZERO_DOUBLE);
    if f64::from(e.get(this, Character::fArmorRating)) < zero {
        add_worn_armour(e, this, Character::fArmorRating, WORN_ITEM_RATING);
    }
    let cap_at = e.call(SETTING_VALUE, &args![SETTING_ARMOUR_CAP]).u32();
    let cap: f32 = e.global(cap_at);
    if f64::from(cap) > zero {
        let cap_at = e.call(SETTING_VALUE, &args![SETTING_ARMOUR_CAP]).u32();
        let cap: f32 = e.global(cap_at);
        let rating = e.get(this, Character::fArmorRating);
        e.call(MIN_OF_TWO, &args![rating, cap]).f32()
    } else {
        e.get(this, Character::fArmorRating)
    }
}

// Translated from 008d24a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: the hand-to-hand damage of this actor, truncated to an `int`
/// (`CombatFormulas::CalcHandToHandDamage` with the actor value owner
/// `this + 0xa4` (0 for a null `this`) and two zeroed `float` outputs, then
/// `_ftol2` of the first output).
pub fn fn_008d24a0(e: &mut Engine, this: Ptr<Character>) -> i32 {
    // The two stack floats: `[ebp-8]` at +0 and `[ebp-4]` at +4.
    let first = e.with_stack(8, |e, block| {
        let owner = if this.is_null() {
            0
        } else {
            this.addr() + 0xa4
        };
        e.call(
            CALC_HAND_TO_HAND_DAMAGE,
            &args![owner, 0u32, block.addr() + 4, block.addr()],
        );
        e.mem.f32(block.addr() + 4)
    });
    e.call(FLOAT_TO_INT, &args![f64::from(first)]).i32()
}

/// `true` when the form-type helper (`009611e0`) says the object has the type
/// 0xf, 0x13 or 0x20.
fn is_excluded_type(e: &mut Engine, object: u32) -> bool {
    e.call(FORM_TYPE_OF, &args![object]).u32() == 0xf
        || e.call(FORM_TYPE_OF, &args![object]).u32() == 0x13
        || e.call(FORM_TYPE_OF, &args![object]).u32() == 0x20
}
/// The helper of `playercharacter.cpp` at `009611e0` (unnamed in the map).
const FORM_TYPE_OF: u32 = 0x0096_11e0;
/// The package type getter (`0041ca90`, `extradatalist.cpp` range, unnamed).
const PACKAGE_TYPE: u32 = 0x0041_ca90;
/// `SpectatorPackage::SpectatorPackage` (Xbox PDB name, `009f72f0`).
const SPECTATOR_PACKAGE_CONSTRUCT: u32 = 0x009f_72f0;
/// The helper `(package, current package)` called on the new spectator
/// package (`0067a1b0`, a `fallout shared` range-sub function).
const COPY_FROM_PACKAGE: u32 = 0x0067_a1b0;
/// `579670` (`tesobjectrefr.cpp` range, unnamed), a predicate on the actor.
const ACTOR_PREDICATE_579670: u32 = 0x0057_9670;

// Translated from 008d24f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: puts the actor on a new `SpectatorPackage` unless one of the
/// guard conditions holds (current package type 0x16, `00493bb0`, in
/// dialogue with the player as the speaker, actor state virtual `+0x214`
/// not 0/4/9, `Actor::ShouldSkipFallOutBehavior(2)`, no saved acquire
/// object, or the acquire object's two forms of type 0xf/0x13/0x20); then
/// resets the process (virtual `+0x214`, `+0x644(1)`), ends an interrupt
/// package, installs the package (`+0x2f4`) and tells the process.
// Translation incomplete: 008d24f0 - SEH exception unwinding (FS:[0] frame, state 0 around the SpectatorPackage allocation) is unsupported.
pub fn fn_008d24f0(e: &mut Engine, this: Ptr<Character>) {
    let t = this.addr();
    if e.call(CURRENT_PACKAGE, &args![this]).u32() != 0 {
        let package = e.call(CURRENT_PACKAGE, &args![this]).u32();
        if e.call(PACKAGE_TYPE, &args![package]).u32() == 0x16 {
            return;
        }
    }
    if e.call(ACTOR_PREDICATE_493BB0, &args![this]).bool() {
        return;
    }
    if e.call(IS_IN_DIALOGUE, &args![this]).bool()
        && e.vcall(t, 0x2c8, &args![]).u32() == e.global::<u32>(PLAYER_SINGLETON)
    {
        return;
    }
    if !(e.vcall(t, 0x214, &args![]).u32() == 0
        || e.vcall(t, 0x214, &args![]).u32() == 4
        || e.vcall(t, 0x214, &args![]).u32() == 9)
    {
        return;
    }
    if e.call(SHOULD_SKIP_FALL_OUT_BEHAVIOR, &args![this, 2u32])
        .bool()
    {
        return;
    }
    let acquired = acquire_process(e, this);
    if acquired == 0 {
        return;
    }
    let first = e.vcall(acquired, 0x20c, &args![]).u32();
    if first != 0 && is_excluded_type(e, first) {
        return;
    }
    let second = e.vcall(acquired, 0x22c, &args![]).u32();
    if second != 0 && is_excluded_type(e, second) {
        return;
    }
    let process = process_of(e, this);
    e.vcall(process, 0x214, &args![]);
    let acquired = acquire_process(e, this);
    e.vcall(acquired, 0x644, &args![1u32]);
    let process = process_of(e, this);
    if e.vcall(process, 0x4bc, &args![]).u32() == 4
        || e.call(ACTOR_PREDICATE_579670, &args![this]).bool()
    {
        e.vcall(t, 0x418, &args![]);
    }
    let block = e.call(OPERATOR_NEW, &args![0xb4u32]).u32();
    let package = if block != 0 {
        e.call(SPECTATOR_PACKAGE_CONSTRUCT, &args![block]).u32()
    } else {
        0
    };
    let acquired = acquire_process(e, this);
    e.vcall(acquired, 0x28, &args![]);
    if e.call(CURRENT_PACKAGE, &args![this]).u32() != 0 {
        let current = e.call(CURRENT_PACKAGE, &args![this]).u32();
        e.call(COPY_FROM_PACKAGE, &args![package, current]);
    }
    if acquire_process(e, this) != 0 {
        let acquired = acquire_process(e, this);
        if e.vcall(acquired, 0x27c, &args![]).u32() != 0 {
            let acquired = acquire_process(e, this);
            let queued = e.vcall(acquired, 0x27c, &args![]).u32();
            if e.call(IS_INTERRUPT_PACKAGE, &args![queued]).bool() {
                e.call(END_INTERRUPT_PACKAGE, &args![this, 0u32]);
            }
        }
    }
    let acquired = acquire_process(e, this);
    e.vcall(acquired, 0x710, &args![this]);
    e.vcall(t, 0x2f4, &args![package, 0u32, 1u32]);
}

// Translated from 008d2800 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: puts the actor on the given package when it is allowed to (not
/// `00493bb0`, not virtual `+0x22c(0)`, not alarmed, not fleeing, not
/// virtual `+0x448`; `00810fd0(package)` at most 1; actor state virtual
/// `+0x214` 0, 4 or 9; a non-null package). Resets the process, ends an
/// interrupt package, installs the package (`+0x2f4(package, 0, 0)`),
/// `00881830(package, 1)`, process `+0x284(0)`. Returns 1 when it did, else 0.
pub fn fn_008d2800(e: &mut Engine, this: Ptr<Character>, package: Ptr) -> bool {
    let t = this.addr();
    if e.call(ACTOR_PREDICATE_493BB0, &args![this]).bool() {
        return false;
    }
    if e.vcall(t, 0x22c, &args![0u32]).bool() {
        return false;
    }
    if e.call(IS_ALARMED, &args![this]).bool() {
        return false;
    }
    if e.call(IS_FLEEING, &args![this, 0u32]).bool() {
        return false;
    }
    if e.vcall(t, 0x448, &args![]).bool() {
        return false;
    }
    if !package.is_null() && e.call(0x0081_0fd0, &args![package]).i32() > 1 {
        return false;
    }
    if !(e.vcall(t, 0x214, &args![]).u32() == 0
        || e.vcall(t, 0x214, &args![]).u32() == 4
        || e.vcall(t, 0x214, &args![]).u32() == 9)
    {
        return false;
    }
    if package.is_null() {
        return false;
    }
    let process = process_of(e, this);
    e.vcall(process, 0x214, &args![]);
    let acquired = acquire_process(e, this);
    e.vcall(acquired, 0x644, &args![1u32]);
    if e.call(CURRENT_PACKAGE, &args![this]).u32() != 0 {
        let current = e.call(CURRENT_PACKAGE, &args![this]).u32();
        if e.call(IS_INTERRUPT_PACKAGE, &args![current]).bool() {
            e.call(END_INTERRUPT_PACKAGE, &args![this, 0u32]);
        }
    }
    let acquired = acquire_process(e, this);
    e.vcall(acquired, 0x28, &args![]);
    let acquired = acquire_process(e, this);
    e.vcall(acquired, 0x710, &args![this]);
    e.vcall(t, 0x2f4, &args![package, 0u32, 0u32]);
    e.call(0x0088_1830, &args![package, 1u32]);
    let process = process_of(e, this);
    e.vcall(process, 0x284, &args![0u32]);
    true
}

// Translated from 008d29b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: replaces `pBiped` (`+0x1b4`). The old `BipedAnim` is released
/// first (unless it is the same pointer, which returns at once): through its
/// deleting destructor (`00418e00(old, 1)`) when `this` is the player,
/// otherwise through `GarbageCollector::Add_ov4` (`00868560`).
pub fn fn_008d29b0(e: &mut Engine, this: Ptr<Character>, biped: Ptr) {
    let old = e.get(this, Character::pBiped);
    if !old.is_null() {
        if old == biped {
            return;
        }
        if this.addr() == e.global::<u32>(PLAYER_SINGLETON) {
            e.call(0x0041_8e00, &args![old, 1u32]);
        } else {
            e.call(0x0086_8560, &args![old]);
        }
    }
    e.set(this, Character::pBiped, biped);
}

// Translated from 008d2a40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: sets `bTrespassing` (`+0x1c0`) from the byte returned by
/// `00546da0(cell, this)` where the cell comes from `008d6f30`; false when
/// there is no such cell.
pub fn fn_008d2a40(e: &mut Engine, this: Ptr<Character>) {
    let cell = e.call(0x008d_6f30, &args![this]).u32();
    let inside = if cell == 0 {
        0
    } else {
        e.call(0x0054_6da0, &args![cell, this]).u8()
    };
    e.mem.set_u8(this.addr() + 0x1c0, inside);
}

/// The package builder shared by `008d2a80` and `008d2db0` (the two bodies
/// are identical up to the package type, the target type word, and the call
/// of virtual `+0x540` that only `008d2db0` makes). Returns nothing.
fn start_interrupt_package(
    e: &mut Engine,
    this: Ptr<Character>,
    package_type: u32,
    target_flag: u32,
    target: Ptr,
    extra: Option<(u32, u8, u32)>,
) {
    let t = this.addr();
    if e.call(ACTOR_PREDICATE_493BB0, &args![this]).bool() {
        return;
    }
    let process = process_of(e, this);
    e.vcall(process, 0x210, &args![0u32, this]);
    let acquired = acquire_process(e, this);
    e.vcall(acquired, 0x644, &args![0u32]);
    let acquired = acquire_process(e, this);
    e.vcall(acquired, 0x22c, &args![]);
    if e.call(CURRENT_PACKAGE, &args![this]).u32() != 0 {
        let current = e.call(CURRENT_PACKAGE, &args![this]).u32();
        if e.call(IS_INTERRUPT_PACKAGE, &args![current]).bool() {
            e.vcall(t, 0x288, &args![]);
        }
    }
    // `TESPackage::CreatePackage(type)` (cdecl).
    let package = e.call(0x0067_0b90, &args![package_type]).u32();
    e.call(0x0082_6b40, &args![package, 1u32]);
    e.call(0x0082_6b90, &args![package, 1u32]);
    // `PackageLocation` (12 bytes): location type 0, location reference this.
    let block = e.call(OPERATOR_NEW, &args![0xcu32]).u32();
    let location = if block != 0 {
        e.call(0x0067_f030, &args![block]).u32()
    } else {
        0
    };
    e.call(0x0067_f140, &args![location, 0u32]);
    e.call(0x0067_f3c0, &args![location, this]);
    e.call(0x0067_1d30, &args![package, location]);
    if location != 0 {
        e.call(0x0067_0b30, &args![location, 1u32]);
    }
    // `PackageTarget` (16 bytes).
    let block = e.call(OPERATOR_NEW, &args![0x10u32]).u32();
    let package_target = if block != 0 {
        e.call(0x0067_ff70, &args![block]).u32()
    } else {
        0
    };
    e.call(0x0067_2fc0, &args![package, package_target]);
    if package_target != 0 {
        e.call(0x007b_3fa0, &args![package_target, 1u32]);
    }
    e.call(0x0098_4f60, &args![package, target_flag]);
    let package_target = e.call(0x0067_1d10, &args![package]).u32();
    e.call(0x0068_00b0, &args![package_target, 0u32]);
    let package_target = e.call(0x0067_1d10, &args![package]).u32();
    e.call(0x0068_0110, &args![package_target, target]);
    let package_target = e.call(0x0067_1d10, &args![package]).u32();
    e.call(0x0040_3550, &args![package_target, 0u32]);
    let acquired = acquire_process(e, this);
    e.vcall(acquired, 0x28, &args![]);
    let process = process_of(e, this);
    if e.vcall(process, 0x22c, &args![]).u32() != 0 {
        let process = process_of(e, this);
        let a = e.vcall(process, 0x5a4, &args![]).u32() & 0xff;
        // Slots 0x5a4/0x4e8/0x11c/0x128/0x23c/0x22c take no stack words: each
        // pushed result is a word of `SetPackageExtra` below (008d2cf4..008d2d78).
        let b = e.vcall(process, 0x4e8, &args![]).u32() & 0xff;
        let c = e.vcall(process, 0x11c, &args![]).u32() & 0xff;
        let d = e.vcall(process, 0x128, &args![]).u32();
        let f = e.vcall(process, 0x23c, &args![]).u32();
        let g = e.vcall(process, 0x22c, &args![]).u32();
        let list = e.call(EXTRA_DATA_LIST, &args![this]).u32();
        // `ExtraDataList::SetPackageExtra` (Xbox PDB name, `0041c930`).
        e.call(0x0041_c930, &args![list, g, f, d, c, b, a]);
    }
    if let Some((first, flag, second)) = extra {
        let process = process_of(e, this);
        e.vcall(process, 0x540, &args![first, flag, second]);
    }
    e.vcall(t, 0x2f4, &args![package, 0u32, 1u32]);
    e.call(0x008a_8060, &args![target]);
}

// Translated from 008d2a80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: makes a package of type 0x26 targeting `target` (a reference),
/// located on this actor, with the actor's package data copied over from its
/// process (`SetPackageExtra`), installs it through virtual `+0x2f4`, and
/// ends with `008a8060(target)`. Nothing happens when `00493bb0` is true.
// Translation incomplete: 008d2a80 - SEH exception unwinding (FS:[0] frame, states 0/1 around the PackageLocation / PackageTarget allocations) is unsupported.
pub fn fn_008d2a80(e: &mut Engine, this: Ptr<Character>, target: Ptr) {
    start_interrupt_package(e, this, 0x26, 0x2f, target, None);
}

// Translated from 008d2db0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: the same as `008d2a80` with package type 0x25 and target word
/// 0x1d, plus one call of process virtual `+0x540` with the last three
/// arguments (`second`, `flag` as a byte, `third`) before the package is
/// installed.
// Translation incomplete: 008d2db0 - SEH exception unwinding (FS:[0] frame, states 0/1 around the PackageLocation / PackageTarget allocations) is unsupported.
pub fn fn_008d2db0(
    e: &mut Engine,
    this: Ptr<Character>,
    target: Ptr,
    second: u32,
    third: u32,
    flag: u8,
) {
    start_interrupt_package(e, this, 0x25, 0x1d, target, Some((second, flag, third)));
}

// Translated from 008d3110 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Character::GetClothingValue` (Xbox PDB): the sum over the 20 worn-item
/// slots of `setting_base + setting_scale * ItemChange::GetItemValue(item)`
/// for each worn armour piece (settings `011d0ca8` scale, `011d1104` base)
/// or, if it is not armour, each worn clothing piece (`011d0d14`, `011d043c`);
/// anything above the `double` at `01017a40` becomes the `float` at
/// `01016410`. Each item result is released afterwards.
pub fn character_get_clothing_value(e: &mut Engine, this: Ptr<Character>) -> f32 {
    let mut total = 0.0f32;
    let list = e.call(EXTRA_DATA_LIST, &args![this]).u32();
    let changes = e.call(GET_CONTAINER_CHANGES, &args![list]).u32();
    if changes != 0 {
        for slot in 0..0x14i32 {
            let item = e.call(GET_WORN_ITEM, &args![changes, slot, 0u32]).u32();
            if item == 0 {
                continue;
            }
            let form = e.call(WORD_AT_8, &args![item]).u32();
            let armour = e
                .call(
                    RT_DYNAMIC_CAST,
                    &args![
                        Ptr::<()>::new(form),
                        0i32,
                        Ptr::<()>::new(TYPE_BOUND_OBJECT),
                        Ptr::<()>::new(TYPE_ARMOUR),
                        0i32
                    ],
                )
                .u32();
            let form = e.call(WORD_AT_8, &args![item]).u32();
            let clothing = e
                .call(
                    RT_DYNAMIC_CAST,
                    &args![
                        Ptr::<()>::new(form),
                        0i32,
                        Ptr::<()>::new(TYPE_BOUND_OBJECT),
                        Ptr::<()>::new(TYPE_CLOTHING),
                        0i32
                    ],
                )
                .u32();
            let settings = if armour != 0 {
                Some(SETTINGS_ARMOUR_VALUE)
            } else if clothing != 0 {
                Some(SETTINGS_CLOTHING_VALUE)
            } else {
                None
            };
            if let Some([scale, base]) = settings {
                let value = e.call(ITEM_VALUE, &args![item]).f64();
                let scale_at = e.call(SETTING_VALUE, &args![scale]).u32();
                // `FSTP double`: the product is rounded to a double.
                let product = f64::from(e.global::<f32>(scale_at)) * value;
                let base_at = e.call(SETTING_VALUE, &args![base]).u32();
                let sum = f64::from(e.global::<f32>(base_at)) + product;
                total = (sum + f64::from(total)) as f32;
            }
            e.call(RELEASE_WORN_ITEM, &args![item, 1u32]);
        }
    }
    let cap: f64 = e.global(CLOTHING_CAP_COMPARE);
    if f64::from(total) > cap {
        total = e.global(CLOTHING_CAP_VALUE);
    }
    total
}

// Translated from 008d3290 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: the `float` value of the setting at `011cd0fc`.
pub fn fn_008d3290(e: &mut Engine, _this: Ptr<Character>) -> f32 {
    let at = e.call(SETTING_VALUE, &args![SETTING_OF_8D3290]).u32();
    e.global(at)
}

// Translated from 008d32b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: forwards its argument to `008a9020` and returns the 16-bit result
/// (added to a zeroed 16-bit accumulator).
pub fn fn_008d32b0(e: &mut Engine, this: Ptr<Character>, argument: u32) -> u16 {
    let result = e.call(0x008a_9020, &args![this, argument]).u16();
    0u16.wrapping_add(result)
}

// Translated from 008d32f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: forwards its argument to `008a9380`.
pub fn fn_008d32f0(e: &mut Engine, this: Ptr<Character>, argument: u32) {
    e.call(0x008a_9380, &args![this, argument]);
}

// Translated from 008d3310 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: forwards its two arguments to `008a9940`.
pub fn fn_008d3310(e: &mut Engine, this: Ptr<Character>, first: u32, second: u32) {
    e.call(0x008a_9940, &args![this, first, second]);
}

// Translated from 008d3330 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: runs `008aa210(this, argument)`, then, when `0055f5b0` of the
/// object in the global `011de45c` says so, resets `fArmorRating` and
/// `fArmorThreshold` to -1.0 (the `float` at `01012054`).
pub fn fn_008d3330(e: &mut Engine, this: Ptr<Character>, argument: u32) {
    e.call(0x008a_a210, &args![this, argument]);
    let object = e.global::<u32>(0x011d_e45c);
    if e.call(0x0055_f5b0, &args![object]).bool() {
        let minus_one: f32 = e.global(MINUS_ONE);
        e.set(this, Character::fArmorRating, minus_one);
        let minus_one: f32 = e.global(MINUS_ONE);
        e.set(this, Character::fArmorThreshold, minus_one);
    }
}

// Translated from 008d3380 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: forwards its two arguments to `008aa4c0`.
pub fn fn_008d3380(e: &mut Engine, this: Ptr<Character>, first: u32, second: u32) {
    e.call(0x008a_a4c0, &args![this, first, second]);
}

// Translated from 008d33a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (the save-game writer): `Actor::SaveGame(this, buffer)`, then the
/// two bytes `bTrespassing` and `bGuard` written to the buffer
/// (`00865e50(buffer, &field, 1, 0)`).
pub fn fn_008d33a0(e: &mut Engine, this: Ptr<Character>, buffer: Ptr) {
    e.call(0x008a_af40, &args![this, buffer]);
    e.call(0x0086_5e50, &args![buffer, this.addr() + 0x1c0, 1u32, 0u32]);
    e.call(0x0086_5e50, &args![buffer, this.addr() + 0x1c1, 1u32, 0u32]);
}

// Translated from 008d33f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (the save-game reader): `008ab3a0(this, buffer)`, then when the
/// buffer's virtual `+0` (a version byte) is below 9 reads and discards a
/// byte and a 4-byte word, reads `bTrespassing` and `bGuard` (one byte
/// each), and when the version is still below 9 reads and discards a byte
/// and two 4-byte words (`00864980(buffer, destination, size)`).
pub fn fn_008d33f0(e: &mut Engine, this: Ptr<Character>, buffer: Ptr) {
    const READ: u32 = 0x0086_4980;
    e.call(0x008a_b3a0, &args![this, buffer]);
    // The discarded stack locals: a byte at `[ebp-1]`, a word at `[ebp-8]`,
    // then a byte at `[ebp-9]`, words at `[ebp-0x10]` and `[ebp-0x14]`.
    if e.vcall(buffer.addr(), 0, &args![]).u8() < 9 {
        e.with_stack(1, |e, p| e.call(READ, &args![buffer, p.addr(), 1u32]));
        e.with_stack(4, |e, p| e.call(READ, &args![buffer, p.addr(), 4u32]));
    }
    e.call(READ, &args![buffer, this.addr() + 0x1c0, 1u32]);
    e.call(READ, &args![buffer, this.addr() + 0x1c1, 1u32]);
    if e.vcall(buffer.addr(), 0, &args![]).u8() < 9 {
        e.with_stack(1, |e, p| e.call(READ, &args![buffer, p.addr(), 1u32]));
        e.with_stack(4, |e, p| e.call(READ, &args![buffer, p.addr(), 4u32]));
        e.with_stack(4, |e, p| e.call(READ, &args![buffer, p.addr(), 4u32]));
    }
}

// Translated from 008d34b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: forwards its argument to `008abc40`.
pub fn fn_008d34b0(e: &mut Engine, this: Ptr<Character>, argument: u32) {
    e.call(0x008a_bc40, &args![this, argument]);
}

// Translated from 008d34d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: forwards its argument to `008abfe0`.
pub fn fn_008d34d0(e: &mut Engine, this: Ptr<Character>, argument: u32) {
    e.call(0x008a_bfe0, &args![this, argument]);
}

// Translated from 008d34f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: runs `008ac1e0(this, argument)`, then resets `fArmorRating`,
/// `fArmorThreshold` and `fequippedweight` to -1.0 (the `float` at
/// `01012054`, read for each) and clears `bTrespassing` and `bGuard`.
pub fn fn_008d34f0(e: &mut Engine, this: Ptr<Character>, argument: u32) {
    e.call(0x008a_c1e0, &args![this, argument]);
    let minus_one: f32 = e.global(MINUS_ONE);
    e.set(this, Character::fArmorRating, minus_one);
    let minus_one: f32 = e.global(MINUS_ONE);
    e.set(this, Character::fArmorThreshold, minus_one);
    let minus_one: f32 = e.global(MINUS_ONE);
    e.set(this, Character::fequippedweight, minus_one);
    e.set(this, Character::bTrespassing, false);
    e.set(this, Character::bGuard, false);
}

/// The byte `index` of the name string (the pointer comes from `0043b1b0`,
/// asked again for every read as the game does), as a signed `char`.
fn name_char(e: &mut Engine, name: u32, index: u32) -> i8 {
    let text = e.call(0x0043_b1b0, &args![name]).u32();
    e.mem.i8(text + index)
}

/// The classification in the middle of `008d3550`: the first characters of
/// an actor-value name pick one of three index families, `(first, second,
/// third)` with -1 for "not set": `first` goes to the `+0xdc` call, `second`
/// to `+0xe0`, `third` to `+0xe4`. The tables are the compiler's switch
/// tables of `008d3d9c`, `008d3df8`/`008d3e0c`, `008d3e34`/`008d3e48`,
/// `008d3e74`/`008d3e88` and `008d3e9c`/`008d3ebc`, read from the exe.
fn classify_name(e: &mut Engine, name: u32) -> (i32, i32, i32) {
    let mut first = -1;
    let mut second = -1;
    let mut third = -1;
    let c0 = i32::from(name_char(e, name, 0));
    if (c0 - 0x41) as u32 <= 0x16 {
        match c0 as u8 {
            b'A' => {
                if name_char(e, name, 1) == b'n' as i8 {
                    first = 0;
                } else {
                    third = 0;
                }
            }
            b'B' => match name_char(e, name, 1) as u8 {
                b'M' => third = 2,
                b'i' => third = 1,
                b'l' => second = i32::from(name_char(e, name, 5) != b'L' as i8),
                b'r' => match name_char(e, name, 4) as u8 {
                    b'D' => {
                        second = if name_char(e, name, 8) == b'L' as i8 {
                            2
                        } else {
                            3
                        }
                    }
                    b'I' => {
                        second = if name_char(e, name, 6) == b'L' as i8 {
                            4
                        } else {
                            5
                        }
                    }
                    b'U' => {
                        second = if name_char(e, name, 6) == b'L' as i8 {
                            6
                        } else {
                            7
                        }
                    }
                    _ => {}
                },
                _ => {}
            },
            b'C' => {
                if name_char(e, name, 1) == b'o' as i8 {
                    first = 0xe;
                } else {
                    third = 3;
                }
            }
            b'D' => third = 4,
            b'E' => {
                third = if name_char(e, name, 1) == b'e' as i8 {
                    5
                } else {
                    6
                };
            }
            b'F' => {
                if name_char(e, name, 1) == b'e' as i8 {
                    first = 1;
                } else {
                    third = 7;
                }
            }
            b'H' => match name_char(e, name, 4) as u8 {
                b'P' => second = 0xe,
                b'R' => second = 0xf,
                b'Y' => second = 0x10,
                b'y' => first = 2,
                _ => {}
            },
            b'I' => third = 8,
            b'K' => third = 9,
            b'L' => match name_char(e, name, 4) as u8 {
                b'D' => second = 8,
                b'L' => second = 9,
                b'R' => second = 10,
                b'U' => second = 11,
                _ => {}
            },
            b'M' => match name_char(e, name, 5) as u8 {
                b'a' => first = 0xc,
                b'e' => first = 5,
                b'f' => first = 6,
                b'l' => first = 10,
                b'n' => {
                    first = if name_char(e, name, 6) == b'n' as i8 {
                        7
                    } else {
                        0xb
                    };
                }
                b'o' => first = 8,
                b'r' => first = 9,
                _ => {}
            },
            b'N' => third = 10,
            b'O' => {
                third = if name_char(e, name, 1) == b'h' as i8 {
                    0xb
                } else {
                    0xc
                };
            }
            b'P' => first = 0xd,
            b'R' => third = 0xd,
            b'S' => match name_char(e, name, 1) as u8 {
                b'a' => first = 3,
                b'q' => {
                    second = if name_char(e, name, 6) == b'L' as i8 {
                        0xc
                    } else {
                        0xd
                    };
                }
                b'u' => first = 4,
                _ => {}
            },
            b'T' => third = 0xe,
            b'W' => third = 0xf,
            _ => {}
        }
    }
    (first, second, third)
}

// Translated from 008d3550 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (the per-frame update of a `Character`, a virtual slot): runs the
/// `Actor` update (`00888b50(this, delta)`), then, unless this is the player
/// and `004eaf60(player)` is false, or the base form (virtual `+0x1d0`) or the
/// process is missing, makes sure the process has its three animation
/// objects (process virtuals `+0x790/+0x798/+0x7a0`, created from the
/// globals `011c6240/44/48` with `004ade00`/`00653270` and stored through
/// `+0x794/+0x79c/+0x7a4` when missing) and, when the first two pass their
/// checks and the third object's type is not frozen (`456610(009611e0(c))`),
/// walks the controllers of the third object (`00a5c570`): each controller
/// whose element casts (`0045bad0`) and whose value (`00508100`) is not below
/// 0.0 and whose name (`00a41590`, copied by `0048af40`) is not empty is
/// classified by its first characters ([`classify_name`]) and the value is
/// handed to both of the first two objects' virtual `+0x100` result, through
/// its virtual `+0xdc`, `+0xe0` or `+0xe4`.
// Translation incomplete: 008d3550 - SEH exception unwinding (FS:[0] frame, state 0 while the name string temporary lives) is unsupported.
pub fn fn_008d3550(e: &mut Engine, this: Ptr<Character>, delta: f32) {
    let t = this.addr();
    e.call(0x0088_8b50, &args![this, delta]);
    let player = e.global::<u32>(PLAYER_SINGLETON);
    if t == player && !e.call(0x004e_af60, &args![player]).bool() {
        return;
    }
    let base_form = e.vcall(t, 0x1d0, &args![]).u32();
    if base_form == 0 || process_of(e, this) == 0 {
        return;
    }
    let process = process_of(e, this);
    let mut a = e.vcall(process, 0x790, &args![]).u32();
    let process = process_of(e, this);
    let mut b = e.vcall(process, 0x798, &args![]).u32();
    let process = process_of(e, this);
    let mut c = e.vcall(process, 0x7a0, &args![]).u32();
    if a == 0 {
        let word = fn_008d3f70(e);
        let made = e.call(0x004a_de00, &args![base_form, word]).u32();
        a = e.call(0x0065_3270, &args![0x011d_5bf8u32, made]).u32();
        let process = process_of(e, this);
        e.vcall(process, 0x794, &args![a]);
    }
    if b == 0 {
        let word = fn_008d3f80(e);
        let made = e.call(0x004a_de00, &args![base_form, word]).u32();
        b = e.call(0x0065_3270, &args![0x011d_5bf8u32, made]).u32();
        let process = process_of(e, this);
        e.vcall(process, 0x79c, &args![b]);
    }
    if c == 0 {
        let word = fn_008d3f90(e);
        c = e.call(0x004a_de00, &args![base_form, word]).u32();
        let process = process_of(e, this);
        e.vcall(process, 0x7a4, &args![c]);
    }
    if a != 0 && !e.call(0x004f_0140, &args![a]).bool() {
        return;
    }
    if b != 0 && e.call(0x0045_6610, &args![b]).bool() {
        return;
    }
    if c == 0 || a == 0 || b == 0 {
        return;
    }
    let kind = e.call(FORM_TYPE_OF, &args![c]).u32();
    let frozen = e.call(0x0045_6610, &args![kind]).u8();
    let target = e.vcall(a, 0x100, &args![]).u32();
    e.call(0x0064_cc90, &args![target, u32::from(frozen)]);
    let target = e.vcall(b, 0x100, &args![]).u32();
    e.call(0x0064_cc90, &args![target, u32::from(frozen)]);
    if frozen != 0 {
        return;
    }
    let controllers = e.call(0x00a5_c570, &args![c, 0x011f_3728u32]).u32();
    let count = fn_008d3ed0(e, Ptr::new(controllers));
    let zero: f64 = e.global(ZERO_DOUBLE);
    let mut index = 0u32;
    while index < count {
        let entry = fn_008d3f10(e, Ptr::new(controllers), index);
        let element = e.vcall(controllers, 0xc4, &args![index & 0xffff]).u32();
        let mut cast = 0;
        if e.call(0x0045_bad0, &args![0x011f_3e58u32, element]).bool() {
            cast = element;
        }
        if cast != 0 {
            let value = e.call(0x0050_8100, &args![cast]).f32();
            let name_source = e.call(0x00a4_1590, &args![entry.addr()]).u32();
            // The name string temporary (one word) on the stack.
            e.with_stack(4, |e, name| {
                let name = name.addr();
                e.call(0x0048_af40, &args![name, name_source]);
                if f64::from(value) >= zero && name_char(e, name, 0) != 0 {
                    let (first, second, third) = classify_name(e, name);
                    let slot_and_index = if first != -1 {
                        Some((0xdcu32, first))
                    } else if second != -1 {
                        Some((0xe0, second))
                    } else if third != -1 {
                        Some((0xe4, third))
                    } else {
                        None
                    };
                    if let Some((slot, which)) = slot_and_index {
                        for object in [a, b] {
                            let target = e.vcall(object, 0x100, &args![]).u32();
                            e.vcall(target, slot, &args![which, value]);
                        }
                    }
                }
                e.call(0x0043_81b0, &args![name]);
            });
        }
        index += 1;
    }
}

// Translated from 008d3ed0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: the count of the array at `this + 0x38`: zero when its head
/// (`00559450`, the word at `+0`) is null, else the word at `+8` of the head.
pub fn fn_008d3ed0(e: &mut Engine, this: Ptr) -> u32 {
    if e.call(WORD_AT_0, &args![this.addr() + 0x38]).u32() == 0 {
        0
    } else {
        let head = e.call(WORD_AT_0, &args![this.addr() + 0x38]).u32();
        e.call(WORD_AT_8, &args![head]).u32()
    }
}

// Translated from 008d3f10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: the element `index` of the array at `this + 0x38`
/// (`00559450` for the head, then `008d3f40`). The game pushes `index`
/// before the head getter, which does not pop it; the second call reads it.
pub fn fn_008d3f10(e: &mut Engine, this: Ptr, index: u32) -> Ptr {
    let head = e.call(WORD_AT_0, &args![this.addr() + 0x38]).u32();
    fn_008d3f40(e, Ptr::new(head), index)
}

// Translated from 008d3f40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: the address of element `index` (16 bytes each, array at `+0x10`)
/// when `index` is below the count at `+8`, else null.
pub fn fn_008d3f40(e: &mut Engine, this: Ptr, index: u32) -> Ptr {
    if index < e.mem.u32(this.addr() + 8) {
        Ptr::new(
            index
                .wrapping_mul(0x10)
                .wrapping_add(e.mem.u32(this.addr() + 0x10)),
        )
    } else {
        Ptr::NULL
    }
}

// Translated from 008d3f70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: the word in the global `011c6240`.
pub fn fn_008d3f70(e: &mut Engine) -> u32 {
    e.global(0x011c_6240)
}

// Translated from 008d3f80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: the word in the global `011c6244`.
pub fn fn_008d3f80(e: &mut Engine) -> u32 {
    e.global(0x011c_6244)
}

// Translated from 008d3f90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: the word in the global `011c6248`.
pub fn fn_008d3f90(e: &mut Engine) -> u32 {
    e.global(0x011c_6248)
}

// Translated from 008d3fa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Character::Reset3D` (Xbox PDB): nothing when `0043fcd0(this)` is null.
/// When `008c7aa0()` is true the reset is queued (`004537b0()` returns the
/// task queue, `0087b5c0(queue, this)`). Otherwise, for the player the
/// `00705990()` object's virtual `+0x28` runs first and `+0x24` last; in
/// between the process's virtual `+0x468` is called with 1, 2, 4 and 0x10,
/// then `+0x464(this)`; finally, when `0045cd60` of the process is 0,
/// `008854b0(process, 0.0)`.
pub fn character_reset_3d(e: &mut Engine, this: Ptr<Character>) {
    if e.call(0x0043_fcd0, &args![this]).u32() == 0 {
        return;
    }
    let player = e.global::<u32>(PLAYER_SINGLETON);
    if e.call(0x008c_7aa0, &args![]).bool() {
        let queue = e.call(0x0045_37b0, &args![]).u32();
        e.call(0x0087_b5c0, &args![queue, this]);
        return;
    }
    if this.addr() == player {
        let pipboy = e.call(0x0070_5990, &args![]).u32();
        e.vcall(pipboy, 0x28, &args![]);
    }
    for mask in [1u32, 2, 4, 0x10] {
        let acquired = acquire_process(e, this);
        e.vcall(acquired, 0x468, &args![mask]);
    }
    let acquired = acquire_process(e, this);
    e.vcall(acquired, 0x464, &args![this]);
    if this.addr() == e.global::<u32>(PLAYER_SINGLETON) {
        let pipboy = e.call(0x0070_5990, &args![]).u32();
        e.vcall(pipboy, 0x24, &args![]);
    }
    let process = process_of(e, this);
    if e.call(0x0045_cd60, &args![process]).u32() == 0 {
        e.call(0x0088_54b0, &args![process, 0.0f32]);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x008d1d30,
            character_character(Ptr<Character>) -> Ptr<Character>
        ),
        entry!(0x008d1e50, fn_008d1e50(Ptr<Character>) -> u32),
        entry!(0x008d1e70, fn_008d1e70(Ptr<Character>) -> bool),
        entry!(0x008d1e90, fn_008d1e90(Ptr<Character>, f32)),
        entry!(0x008d1eb0, fn_008d1eb0(Ptr<Character>) -> f32),
        entry!(0x008d1ed0, fn_008d1ed0(Ptr<Character>) -> bool),
        entry!(0x008d1ef0, fn_008d1ef0(Ptr<Character>, u8)),
        entry!(
            0x008d1f10,
            fn_008d1f10(Ptr<Character>, u32) -> Ptr<Character>
        ),
        entry!(
            0x008d1f40,
            character_character_ov2(Ptr<Character>, u8) -> Ptr<Character>
        ),
        entry!(0x008d2060, fn_008d2060(Ptr<Character>)),
        entry!(0x008d2110, fn_008d2110(Ptr<Character>) -> f32),
        entry!(0x008d22b0, fn_008d22b0(Ptr<Character>) -> f32),
        entry!(0x008d24a0, fn_008d24a0(Ptr<Character>) -> i32),
        entry!(0x008d24f0, fn_008d24f0(Ptr<Character>)),
        entry!(0x008d2800, fn_008d2800(Ptr<Character>, Ptr) -> bool),
        entry!(0x008d29b0, fn_008d29b0(Ptr<Character>, Ptr)),
        entry!(0x008d2a40, fn_008d2a40(Ptr<Character>)),
        entry!(0x008d2a80, fn_008d2a80(Ptr<Character>, Ptr)),
        entry!(0x008d2db0, fn_008d2db0(Ptr<Character>, Ptr, u32, u32, u8)),
        entry!(
            0x008d3110,
            character_get_clothing_value(Ptr<Character>) -> f32
        ),
        entry!(0x008d3290, fn_008d3290(Ptr<Character>) -> f32),
        entry!(0x008d32b0, fn_008d32b0(Ptr<Character>, u32) -> u16),
        entry!(0x008d32f0, fn_008d32f0(Ptr<Character>, u32)),
        entry!(0x008d3310, fn_008d3310(Ptr<Character>, u32, u32)),
        entry!(0x008d3330, fn_008d3330(Ptr<Character>, u32)),
        entry!(0x008d3380, fn_008d3380(Ptr<Character>, u32, u32)),
        entry!(0x008d33a0, fn_008d33a0(Ptr<Character>, Ptr)),
        entry!(0x008d33f0, fn_008d33f0(Ptr<Character>, Ptr)),
        entry!(0x008d34b0, fn_008d34b0(Ptr<Character>, u32)),
        entry!(0x008d34d0, fn_008d34d0(Ptr<Character>, u32)),
        entry!(0x008d34f0, fn_008d34f0(Ptr<Character>, u32)),
        entry!(0x008d3550, fn_008d3550(Ptr<Character>, f32)),
        entry!(0x008d3ed0, fn_008d3ed0(Ptr) -> u32),
        entry!(0x008d3f10, fn_008d3f10(Ptr, u32) -> Ptr),
        entry!(0x008d3f40, fn_008d3f40(Ptr, u32) -> Ptr),
        entry!(0x008d3f70, fn_008d3f70() -> u32),
        entry!(0x008d3f80, fn_008d3f80() -> u32),
        entry!(0x008d3f90, fn_008d3f90() -> u32),
        entry!(0x008d3fa0, character_reset_3d(Ptr<Character>)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The vtable every test object gets: slot `s` holds `slot_target(s)`,
    /// a double that logs its call and returns 0 unless a test replaces it.
    const VT: u32 = 0x0300_0000;
    fn slot_target(slot: u32) -> u32 {
        0x7000_0000 + slot
    }
    fn ret(v: u32) -> Ret {
        Ret {
            eax: v,
            ..Ret::default()
        }
    }
    fn retf(v: f64) -> Ret {
        Ret {
            st0: v,
            ..Ret::default()
        }
    }
    fn rig() -> Engine {
        let mut e = Engine::new();
        e.map(VT, 0x1000);
        for slot in (0..0x800).step_by(4) {
            e.mem.set_u32(VT + slot, slot_target(slot));
            e.register(slot_target(slot), |_, _| Ret::default());
        }
        e.map(PLAYER_SINGLETON, 4);
        e.map(ZERO_DOUBLE, 8);
        e.map(MINUS_ONE, 4);
        e.set_global(MINUS_ONE, -1.0f32);
        e
    }
    /// A zeroed object of `size` bytes whose vtable is the test vtable.
    fn object(e: &mut Engine, size: u32) -> u32 {
        let p = e.mem.alloc(size);
        e.mem.set_u32(p, VT);
        p
    }
    fn character(e: &mut Engine) -> Ptr<Character> {
        let p = object(e, 0x1c8);
        Ptr::new(p)
    }
    /// Registers doubles returning 0 for every address.
    fn zeros(e: &mut Engine, addrs: &[u32]) {
        for a in addrs {
            e.register(*a, |_, _| Ret::default());
        }
    }
    /// Starts the log; call [`logged`] after the call under test.
    fn start(e: &mut Engine) {
        e.call_log = Some(vec![]);
    }
    /// The calls made by the function under test (the top-level call dropped).
    fn logged(e: &mut Engine) -> Vec<(u32, Vec<u32>)> {
        let mut log = e.call_log.take().unwrap();
        log.remove(0);
        log
    }
    /// The process object (`this + 0x68`) of a character.
    fn with_process(e: &mut Engine, c: Ptr<Character>) -> u32 {
        let p = object(e, 0x40);
        e.set(c.cast::<Actor>(), Actor::pCurrentProcess, Ptr::new(p));
        p
    }
    fn acquire_double(e: &mut Engine) {
        e.register(ACTOR_PROCESS, |e, a| ret(e.mem.u32(a[0] + 0x68)));
    }

    #[test]
    fn constructor_sets_every_field_in_the_games_order() {
        let mut e = Engine::new();
        e.register(ACTOR_CONSTRUCT, |_, a| Ret {
            eax: a[0],
            ..Ret::default()
        });
        e.register(SET_FORM_TYPE, |e, a| {
            e.mem.set_u8(a[0] + 4, a[1] as u8);
            Ret::default()
        });
        // The armour reset is a real function of this crate (`008d4e80`);
        // double it to see that it is called and to keep the test local.
        e.register(RESET_ARMOUR, |e, a| {
            e.mem.set_f32(a[0] + 0x1b8, -1.0);
            e.mem.set_f32(a[0] + 0x1bc, -1.0);
            Ret::default()
        });
        e.map(MINUS_ONE & !0xfff, 0x1000);
        e.set_global(MINUS_ONE, -1.0f32);

        let character: Ptr<Character> = e.new_object();
        let actor = character.cast::<Actor>();
        // Dirty values the constructor must overwrite.
        e.set(character, Character::pBiped, Ptr::new(0x1234));
        e.set(character, Character::bGuard, true);
        e.set(character, Character::bTrespassing, true);
        e.set(character, Character::fequippedweight, 5.0);
        e.set(actor, Actor::bAttackOnNextTheft, true);
        e.set(actor, Actor::iThiefCrimeStamp, 9);
        e.set(actor, Actor::iMinorCrimes, 8);
        e.set(actor, Actor::iMajorCrimes, 7);
        e.set(actor, Actor::bIgnoreCrime, true);

        e.call_log = Some(vec![]);
        let back = e.call(0x008d_1d30, &args![character]).ptr::<Character>();
        assert_eq!(back, character);
        let base = character.addr();
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log[1..],
            [
                (ACTOR_CONSTRUCT, vec![base, 1]),
                (SET_FORM_TYPE, vec![base, 0x3b]),
                (RESET_ARMOUR, vec![base]),
            ]
        );
        for (offset, vtable) in VTABLES {
            assert_eq!(e.mem.u32(base + offset), vtable);
        }
        assert_eq!(e.mem.u8(base + 4), 0x3b);
        assert_eq!(e.get(character, Character::pBiped), Ptr::NULL);
        assert_eq!(e.get(character, Character::fArmorRating), -1.0);
        assert_eq!(e.get(character, Character::fArmorThreshold), -1.0);
        assert!(!e.get(character, Character::bGuard));
        assert!(!e.get(character, Character::bTrespassing));
        assert_eq!(e.get(character, Character::fequippedweight), -1.0);
        assert!(!e.get(actor, Actor::bAttackOnNextTheft));
        assert_eq!(e.get(actor, Actor::iThiefCrimeStamp), 0);
        assert_eq!(e.get(actor, Actor::iMinorCrimes), 0);
        assert_eq!(e.get(actor, Actor::iMajorCrimes), 0);
        assert!(!e.get(actor, Actor::bIgnoreCrime));
    }

    #[test]
    fn second_constructor_passes_its_byte_to_the_actor_constructor() {
        let mut e = Engine::new();
        e.register(ACTOR_CONSTRUCT, |_, a| ret(a[0]));
        e.register(SET_FORM_TYPE, |_, _| Ret::default());
        e.register(RESET_ARMOUR, |_, _| Ret::default());
        e.map(MINUS_ONE & !0xfff, 0x1000);
        e.set_global(MINUS_ONE, -1.0f32);
        let c: Ptr<Character> = e.new_object();
        e.set(c, Character::bGuard, true);
        e.set(c, Character::bTrespassing, true);
        e.set(c.cast::<Actor>(), Actor::iMajorCrimes, 7);
        start(&mut e);
        let back = e.call(0x008d_1f40, &args![c, 0u8]).ptr::<Character>();
        assert_eq!(back, c);
        assert_eq!(
            logged(&mut e),
            [
                (ACTOR_CONSTRUCT, vec![c.addr(), 0]),
                (SET_FORM_TYPE, vec![c.addr(), 0x3b]),
                (RESET_ARMOUR, vec![c.addr()]),
            ]
        );
        for (offset, vtable) in VTABLES {
            assert_eq!(e.mem.u32(c.addr() + offset), vtable);
        }
        assert!(!e.get(c, Character::bGuard));
        assert!(!e.get(c, Character::bTrespassing));
        assert_eq!(e.get(c, Character::fequippedweight), -1.0);
        assert_eq!(e.get(c.cast::<Actor>(), Actor::iMajorCrimes), 0);
        // A non-zero byte is passed unchanged.
        start(&mut e);
        e.call(0x008d_1f40, &args![c, 1u8]);
        assert_eq!(logged(&mut e)[0], (ACTOR_CONSTRUCT, vec![c.addr(), 1]));
    }

    #[test]
    fn reads_and_writes_of_the_character_fields() {
        let mut e = rig();
        let c = character(&mut e);
        e.set(c, Character::bTrespassing, true);
        assert!(e.call(0x008d_1e70, &args![c]).bool());
        e.set(c, Character::bTrespassing, false);
        assert!(!e.call(0x008d_1e70, &args![c]).bool());
    }

    #[test]
    fn equipped_weight_is_stored_and_read_back() {
        let mut e = rig();
        let c = character(&mut e);
        e.call(0x008d_1e90, &args![c, 12.5f32]);
        assert_eq!(e.mem.f32(c.addr() + 0x1c4), 12.5);
        assert_eq!(e.call(0x008d_1eb0, &args![c]).f32(), 12.5);
    }

    #[test]
    fn guard_flag_is_read_and_stores_the_byte() {
        let mut e = rig();
        let c = character(&mut e);
        assert!(!e.call(0x008d_1ed0, &args![c]).bool());
        e.call(0x008d_1ef0, &args![c, 2u8]);
        assert_eq!(e.mem.u8(c.addr() + 0x1c1), 2);
        assert!(e.call(0x008d_1ed0, &args![c]).bool());
    }

    #[test]
    fn form_substructure_word_is_read_through_the_form() {
        let mut e = rig();
        let c = character(&mut e);
        e.register(WORD_AT_20, |_, _| ret(0x5000));
        e.register(READ_WORD_AT_4, |_, a| ret(a[0] ^ 0xff));
        start(&mut e);
        assert_eq!(
            e.call(0x008d_1e50, &args![c]).u32(),
            (0x5000 + 0x10c) ^ 0xff
        );
        assert_eq!(
            logged(&mut e),
            [
                (WORD_AT_20, vec![c.addr()]),
                (READ_WORD_AT_4, vec![0x5000 + 0x10c])
            ]
        );
    }

    #[test]
    fn destructor_body_releases_the_3d_only_without_the_flag() {
        let mut e = rig();
        let c = character(&mut e);
        e.register(0x0040_77c0, |_, _| ret(0));
        zeros(&mut e, &[0x0057_02e0, 0x0087_dcb0]);
        start(&mut e);
        e.call(0x008d_2060, &args![c]);
        assert_eq!(
            logged(&mut e),
            [
                (0x0040_77c0, vec![c.addr()]),
                (0x0057_02e0, vec![c.addr(), 0, 0]),
                (0x0087_dcb0, vec![c.addr()]),
            ]
        );
        for (offset, vtable) in VTABLES {
            assert_eq!(e.mem.u32(c.addr() + offset), vtable);
        }
        e.register(0x0040_77c0, |_, _| ret(1));
        start(&mut e);
        e.call(0x008d_2060, &args![c]);
        assert_eq!(
            logged(&mut e),
            [(0x0040_77c0, vec![c.addr()]), (0x0087_dcb0, vec![c.addr()])]
        );
    }

    #[test]
    fn deleting_destructor_frees_only_with_bit_0() {
        let mut e = rig();
        let c = character(&mut e);
        e.register(0x0040_77c0, |_, _| ret(1));
        zeros(&mut e, &[0x0087_dcb0, OPERATOR_DELETE]);
        start(&mut e);
        let back = e.call(0x008d_1f10, &args![c, 1u32]).ptr::<Character>();
        assert_eq!(back, c);
        assert_eq!(
            logged(&mut e),
            [
                (0x0040_77c0, vec![c.addr()]),
                (0x0087_dcb0, vec![c.addr()]),
                (OPERATOR_DELETE, vec![c.addr()]),
            ]
        );
        start(&mut e);
        e.call(0x008d_1f10, &args![c, 2u32]);
        assert!(logged(&mut e).iter().all(|(a, _)| *a != OPERATOR_DELETE));
    }

    /// Doubles for the worn-item walk: slots 0, 1 and 2 hold items 0x100,
    /// 0x200 and 0x300; the forms are item + 1; forms 0x101 and 0x201 cast to
    /// the same armour 0xa1, form 0x301 is not armour.
    fn worn_items(e: &mut Engine) {
        e.register(EXTRA_DATA_LIST, |_, a| ret(a[0] + 0x44));
        e.register(GET_CONTAINER_CHANGES, |_, a| ret(a[0] + 1));
        e.register(GET_WORN_ITEM, |_, a| {
            ret(if a[1] < 3 { 0x100 * (a[1] + 1) } else { 0 })
        });
        e.register(WORD_AT_8, |_, a| ret(a[0] + 1));
        e.register(RT_DYNAMIC_CAST, |_, a| {
            assert_eq!(a[1..], [0, TYPE_BOUND_OBJECT, TYPE_ARMOUR, 0]);
            ret(match a[0] {
                0x101 | 0x201 => 0xa1,
                _ => 0,
            })
        });
        e.register(WORN_ITEM_THRESHOLD, |_, a| {
            retf(if a[0] == 0x100 { 2.5 } else { 9.0 })
        });
        e.register(WORN_ITEM_RATING, |_, a| {
            retf(if a[0] == 0x100 { 4.0 } else { 9.0 })
        });
        zeros(e, &[RELEASE_WORN_ITEM]);
    }

    #[test]
    fn threshold_is_recomputed_once_per_distinct_armour_piece() {
        let mut e = rig();
        worn_items(&mut e);
        let c = character(&mut e);
        e.set(c, Character::fArmorThreshold, -1.0);
        start(&mut e);
        assert_eq!(e.call(0x008d_2110, &args![c]).f32(), 2.5);
        let log = logged(&mut e);
        // Item 0x200 casts to the same armour as item 0x100 and is skipped.
        let owner = c.addr() + 0xa4;
        let per_item: Vec<_> = log
            .iter()
            .filter(|(a, _)| *a == WORN_ITEM_THRESHOLD)
            .collect();
        assert_eq!(per_item.len(), 1);
        assert_eq!(per_item[0].1, vec![0x100, owner]);
        // All three worn items are released, with 1.
        let released: Vec<_> = log
            .iter()
            .filter(|(a, _)| *a == RELEASE_WORN_ITEM)
            .map(|(_, a)| a.clone())
            .collect();
        assert_eq!(released, [vec![0x100, 1], vec![0x200, 1], vec![0x300, 1]]);
        // Not recomputed when the field is not negative.
        e.set(c, Character::fArmorThreshold, 3.0);
        start(&mut e);
        assert_eq!(e.call(0x008d_2110, &args![c]).f32(), 3.0);
        assert!(logged(&mut e).is_empty());
    }

    #[test]
    fn rating_is_capped_by_the_setting_when_it_is_positive() {
        let mut e = rig();
        worn_items(&mut e);
        e.register(SETTING_VALUE, |_, a| ret(a[0] + 4));
        e.register(MIN_OF_TWO, |_, a| {
            retf(f64::from(f32::from_bits(a[0]).min(f32::from_bits(a[1]))))
        });
        e.map(SETTING_ARMOUR_CAP, 8);
        let c = character(&mut e);
        e.set(c, Character::fArmorRating, -1.0);
        e.set_global(SETTING_ARMOUR_CAP + 4, 3.0f32);
        start(&mut e);
        // Recomputed to 4.0, capped to 3.0 by the helper.
        assert_eq!(e.call(0x008d_22b0, &args![c]).f32(), 3.0);
        let log = logged(&mut e);
        assert!(log.contains(&(MIN_OF_TWO, vec![4.0f32.to_bits(), 3.0f32.to_bits()])));
        assert_eq!(e.get(c, Character::fArmorRating), 4.0);
        // A cap that is not positive leaves the rating alone.
        e.set_global(SETTING_ARMOUR_CAP + 4, 0.0f32);
        start(&mut e);
        assert_eq!(e.call(0x008d_22b0, &args![c]).f32(), 4.0);
        assert!(logged(&mut e).iter().all(|(a, _)| *a != MIN_OF_TWO));
    }

    #[test]
    fn hand_to_hand_damage_is_truncated() {
        let mut e = rig();
        e.register(CALC_HAND_TO_HAND_DAMAGE, |e, a| {
            e.mem.set_f32(a[2], 3.75);
            Ret::default()
        });
        e.register(FLOAT_TO_INT, |_, a| {
            ret(f64::from_bits(u64::from(a[0]) | u64::from(a[1]) << 32) as i32 as u32)
        });
        let c = character(&mut e);
        start(&mut e);
        assert_eq!(e.call(0x008d_24a0, &args![c]).i32(), 3);
        let log = logged(&mut e);
        assert_eq!(log[0].0, CALC_HAND_TO_HAND_DAMAGE);
        assert_eq!(log[0].1[..2], [c.addr() + 0xa4, 0]);
        // The first output sits 4 bytes above the second.
        assert_eq!(log[0].1[2], log[0].1[3] + 4);
    }

    /// The doubles of the spectator-package path of `008d24f0`: every
    /// predicate false, every package null.
    fn quiet_actor(e: &mut Engine) {
        acquire_double(e);
        zeros(
            e,
            &[
                CURRENT_PACKAGE,
                PACKAGE_TYPE,
                ACTOR_PREDICATE_493BB0,
                IS_IN_DIALOGUE,
                SHOULD_SKIP_FALL_OUT_BEHAVIOR,
                FORM_TYPE_OF,
                ACTOR_PREDICATE_579670,
                COPY_FROM_PACKAGE,
                IS_INTERRUPT_PACKAGE,
                END_INTERRUPT_PACKAGE,
                SPECTATOR_PACKAGE_CONSTRUCT,
            ],
        );
        e.register(OPERATOR_NEW, |_, _| ret(0x4000));
        e.register(SPECTATOR_PACKAGE_CONSTRUCT, |_, a| ret(a[0]));
    }

    #[test]
    fn spectator_package_is_installed_on_the_plain_path() {
        let mut e = rig();
        quiet_actor(&mut e);
        let c = character(&mut e);
        let process = with_process(&mut e, c);
        e.register(slot_target(0x20c), |_, _| ret(0));
        start(&mut e);
        e.call(0x008d_24f0, &args![c]);
        let t = c.addr();
        let v = slot_target;
        assert_eq!(
            logged(&mut e),
            [
                (CURRENT_PACKAGE, vec![t]),
                (ACTOR_PREDICATE_493BB0, vec![t]),
                (IS_IN_DIALOGUE, vec![t]),
                (v(0x214), vec![t]),
                (SHOULD_SKIP_FALL_OUT_BEHAVIOR, vec![t, 2]),
                (ACTOR_PROCESS, vec![t]),
                (v(0x20c), vec![process]),
                (v(0x22c), vec![process]),
                (v(0x214), vec![process]),
                (ACTOR_PROCESS, vec![t]),
                (v(0x644), vec![process, 1]),
                (v(0x4bc), vec![process]),
                (ACTOR_PREDICATE_579670, vec![t]),
                (OPERATOR_NEW, vec![0xb4]),
                (SPECTATOR_PACKAGE_CONSTRUCT, vec![0x4000]),
                (ACTOR_PROCESS, vec![t]),
                (v(0x28), vec![process]),
                (CURRENT_PACKAGE, vec![t]),
                (ACTOR_PROCESS, vec![t]),
                (ACTOR_PROCESS, vec![t]),
                (v(0x27c), vec![process]),
                (ACTOR_PROCESS, vec![t]),
                (v(0x710), vec![process, t]),
                (v(0x2f4), vec![t, 0x4000, 0, 1]),
            ]
        );
    }

    #[test]
    fn spectator_package_is_skipped_for_the_guard_conditions() {
        let mut e = rig();
        quiet_actor(&mut e);
        let c = character(&mut e);
        with_process(&mut e, c);
        // Current package of type 0x16.
        e.register(CURRENT_PACKAGE, |_, _| ret(0x9000));
        e.register(PACKAGE_TYPE, |_, _| ret(0x16));
        start(&mut e);
        e.call(0x008d_24f0, &args![c]);
        assert_eq!(logged(&mut e).len(), 3);
        // Actor state 5 (virtual +0x214 returns 5): stops after the checks.
        e.register(CURRENT_PACKAGE, |_, _| ret(0));
        e.register(slot_target(0x214), |_, _| ret(5));
        start(&mut e);
        e.call(0x008d_24f0, &args![c]);
        let log = logged(&mut e);
        assert_eq!(log.last().unwrap().0, slot_target(0x214));
        assert_eq!(
            log.iter().filter(|(a, _)| *a == slot_target(0x214)).count(),
            3
        );
        // An excluded form type on the acquired object's first form.
        e.register(slot_target(0x214), |_, _| ret(4));
        e.register(slot_target(0x20c), |_, _| ret(0x777));
        e.register(FORM_TYPE_OF, |_, _| ret(0x13));
        start(&mut e);
        e.call(0x008d_24f0, &args![c]);
        let log = logged(&mut e);
        assert!(log.iter().all(|(a, _)| *a != OPERATOR_NEW));
        assert!(log.contains(&(FORM_TYPE_OF, vec![0x777])));
    }

    #[test]
    fn package_request_runs_the_process_changes_in_order() {
        let mut e = rig();
        quiet_actor(&mut e);
        zeros(
            &mut e,
            &[0x0081_0fd0, 0x0088_1830, 0x008a_61b0, 0x008a_6650],
        );
        let c = character(&mut e);
        let process = with_process(&mut e, c);
        let t = c.addr();
        let v = slot_target;
        start(&mut e);
        assert!(e.call(0x008d_2800, &args![c, 0x6000u32]).bool());
        assert_eq!(
            logged(&mut e),
            [
                (ACTOR_PREDICATE_493BB0, vec![t]),
                (v(0x22c), vec![t, 0]),
                (IS_ALARMED, vec![t]),
                (IS_FLEEING, vec![t, 0]),
                (v(0x448), vec![t]),
                (0x0081_0fd0, vec![0x6000]),
                (v(0x214), vec![t]),
                (v(0x214), vec![process]),
                (ACTOR_PROCESS, vec![t]),
                (v(0x644), vec![process, 1]),
                (CURRENT_PACKAGE, vec![t]),
                (ACTOR_PROCESS, vec![t]),
                (v(0x28), vec![process]),
                (ACTOR_PROCESS, vec![t]),
                (v(0x710), vec![process, t]),
                (v(0x2f4), vec![t, 0x6000, 0, 0]),
                (0x0088_1830, vec![0x6000, 1]),
                (v(0x284), vec![process, 0]),
            ]
        );
        // No package: nothing happens (after the checks).
        start(&mut e);
        assert!(!e.call(0x008d_2800, &args![c, 0u32]).bool());
        assert_eq!(logged(&mut e).len(), 6);
        // A busy package (result above 1) or a fleeing actor refuses.
        e.register(0x0081_0fd0, |_, _| ret(2));
        assert!(!e.call(0x008d_2800, &args![c, 0x6000u32]).bool());
        e.register(0x0081_0fd0, |_, _| ret(0));
        e.register(IS_FLEEING, |_, _| ret(1));
        start(&mut e);
        assert!(!e.call(0x008d_2800, &args![c, 0x6000u32]).bool());
        assert_eq!(logged(&mut e).len(), 4);
    }

    #[test]
    fn replacing_the_biped_releases_the_old_one_the_players_way_or_not() {
        let mut e = rig();
        zeros(&mut e, &[0x0041_8e00, 0x0086_8560]);
        let c = character(&mut e);
        // No old biped: just stored.
        start(&mut e);
        e.call(0x008d_29b0, &args![c, Ptr::<()>::new(0x1000)]);
        assert!(logged(&mut e).is_empty());
        assert_eq!(e.get(c, Character::pBiped), Ptr::new(0x1000));
        // The same biped: nothing.
        e.call(0x008d_29b0, &args![c, Ptr::<()>::new(0x1000)]);
        // Another actor: the garbage collector.
        start(&mut e);
        e.call(0x008d_29b0, &args![c, Ptr::<()>::new(0x2000)]);
        assert_eq!(logged(&mut e), [(0x0086_8560, vec![0x1000])]);
        assert_eq!(e.get(c, Character::pBiped), Ptr::new(0x2000));
        // The player: the deleting destructor.
        e.set_global(PLAYER_SINGLETON, c.addr());
        start(&mut e);
        e.call(0x008d_29b0, &args![c, Ptr::<()>::new(0x3000)]);
        assert_eq!(logged(&mut e), [(0x0041_8e00, vec![0x2000, 1])]);
        assert_eq!(e.get(c, Character::pBiped), Ptr::new(0x3000));
    }

    #[test]
    fn trespassing_comes_from_the_cell_test() {
        let mut e = rig();
        let c = character(&mut e);
        e.register(0x008d_6f30, |_, _| ret(0));
        e.register(0x0054_6da0, |_, a| {
            ret(if a[0] == 0x7000 { 0x1fe } else { 0 })
        });
        e.mem.set_u8(c.addr() + 0x1c0, 1);
        e.call(0x008d_2a40, &args![c]);
        assert!(!e.get(c, Character::bTrespassing));
        e.register(0x008d_6f30, |_, _| ret(0x7000));
        start(&mut e);
        e.call(0x008d_2a40, &args![c]);
        assert_eq!(
            logged(&mut e),
            [
                (0x008d_6f30, vec![c.addr()]),
                (0x0054_6da0, vec![0x7000, c.addr()])
            ]
        );
        // Only the low byte of the result is kept.
        assert_eq!(e.mem.u8(c.addr() + 0x1c0), 0xfe);
    }

    /// Doubles for `008d2a80` and `008d2db0`.
    fn package_builder(e: &mut Engine) {
        quiet_actor(e);
        zeros(
            e,
            &[
                0x0067_0b90,
                0x0082_6b40,
                0x0082_6b90,
                0x0067_f030,
                0x0067_f140,
                0x0067_f3c0,
                0x0067_1d30,
                0x0067_0b30,
                0x0067_ff70,
                0x0067_2fc0,
                0x007b_3fa0,
                0x0098_4f60,
                0x0067_1d10,
                0x0068_00b0,
                0x0068_0110,
                0x0040_3550,
                0x0041_c930,
                0x008a_8060,
                EXTRA_DATA_LIST,
            ],
        );
        e.register(0x0067_0b90, |_, _| ret(0x5000));
        e.register(0x0067_f030, |_, a| ret(a[0]));
        e.register(0x0067_ff70, |_, a| ret(a[0] + 0x10));
        e.register(0x0067_1d10, |_, _| ret(0x5030));
        e.register(EXTRA_DATA_LIST, |_, a| ret(a[0] + 0x44));
    }

    #[test]
    fn package_of_type_0x26_targets_the_reference() {
        let mut e = rig();
        package_builder(&mut e);
        let c = character(&mut e);
        let process = with_process(&mut e, c);
        let t = c.addr();
        let v = slot_target;
        // A process that has package data: virtual +0x22c is non-zero.
        e.register(slot_target(0x22c), |_, _| ret(0x44));
        start(&mut e);
        e.call(0x008d_2a80, &args![c, 0x6000u32]);
        assert_eq!(
            logged(&mut e),
            [
                (ACTOR_PREDICATE_493BB0, vec![t]),
                (v(0x210), vec![process, 0, t]),
                (ACTOR_PROCESS, vec![t]),
                (v(0x644), vec![process, 0]),
                (ACTOR_PROCESS, vec![t]),
                (v(0x22c), vec![process]),
                (CURRENT_PACKAGE, vec![t]),
                (0x0067_0b90, vec![0x26]),
                (0x0082_6b40, vec![0x5000, 1]),
                (0x0082_6b90, vec![0x5000, 1]),
                (OPERATOR_NEW, vec![0xc]),
                (0x0067_f030, vec![0x4000]),
                (0x0067_f140, vec![0x4000, 0]),
                (0x0067_f3c0, vec![0x4000, t]),
                (0x0067_1d30, vec![0x5000, 0x4000]),
                (0x0067_0b30, vec![0x4000, 1]),
                (OPERATOR_NEW, vec![0x10]),
                (0x0067_ff70, vec![0x4000]),
                (0x0067_2fc0, vec![0x5000, 0x4010]),
                (0x007b_3fa0, vec![0x4010, 1]),
                (0x0098_4f60, vec![0x5000, 0x2f]),
                (0x0067_1d10, vec![0x5000]),
                (0x0068_00b0, vec![0x5030, 0]),
                (0x0067_1d10, vec![0x5000]),
                (0x0068_0110, vec![0x5030, 0x6000]),
                (0x0067_1d10, vec![0x5000]),
                (0x0040_3550, vec![0x5030, 0]),
                (ACTOR_PROCESS, vec![t]),
                (v(0x28), vec![process]),
                (v(0x22c), vec![process]),
                (v(0x5a4), vec![process]),
                (v(0x4e8), vec![process]),
                (v(0x11c), vec![process]),
                (v(0x128), vec![process]),
                (v(0x23c), vec![process]),
                (v(0x22c), vec![process]),
                (EXTRA_DATA_LIST, vec![t]),
                (0x0041_c930, vec![t + 0x44, 0x44, 0, 0, 0, 0, 0]),
                (v(0x2f4), vec![t, 0x5000, 0, 1]),
                (0x008a_8060, vec![0x6000]),
            ]
        );
        // `00493bb0` true: nothing else happens.
        e.register(ACTOR_PREDICATE_493BB0, |_, _| ret(1));
        start(&mut e);
        e.call(0x008d_2a80, &args![c, 0x6000u32]);
        assert_eq!(logged(&mut e).len(), 1);
    }

    #[test]
    fn package_of_type_0x25_adds_the_process_call() {
        let mut e = rig();
        package_builder(&mut e);
        let c = character(&mut e);
        let process = with_process(&mut e, c);
        let t = c.addr();
        let v = slot_target;
        start(&mut e);
        e.call(0x008d_2db0, &args![c, 0x6000u32, 0x11u32, 0x22u32, 0x33u8]);
        let log = logged(&mut e);
        assert!(log.contains(&(0x0067_0b90, vec![0x25])));
        assert!(log.contains(&(0x0098_4f60, vec![0x5000, 0x1d])));
        // No SetPackageExtra: the process virtual +0x22c answers 0.
        assert!(log.iter().all(|(a, _)| *a != 0x0041_c930));
        // Argument order of the extra call: second, byte, third.
        let n = log.len();
        assert_eq!(log[n - 3], (v(0x540), vec![process, 0x11, 0x33, 0x22]));
        assert_eq!(log[n - 2], (v(0x2f4), vec![t, 0x5000, 0, 1]));
        assert_eq!(log[n - 1], (0x008a_8060, vec![0x6000]));
    }

    #[test]
    fn clothing_value_sums_armour_and_clothing_pieces_and_caps() {
        let mut e = rig();
        e.register(EXTRA_DATA_LIST, |_, a| ret(a[0] + 0x44));
        e.register(GET_CONTAINER_CHANGES, |_, a| ret(a[0] + 1));
        e.register(GET_WORN_ITEM, |_, a| {
            ret(if a[1] < 3 { 0x100 * (a[1] + 1) } else { 0 })
        });
        e.register(WORD_AT_8, |_, a| ret(a[0] + 1));
        // Item 0x100 is armour, item 0x200 clothing, 0x300 neither.
        e.register(RT_DYNAMIC_CAST, |_, a| {
            ret(match (a[0], a[3]) {
                (0x101, TYPE_ARMOUR) | (0x201, TYPE_CLOTHING) => 0xa1,
                _ => 0,
            })
        });
        e.register(ITEM_VALUE, |_, a| retf(f64::from(a[0] / 0x100) * 10.0));
        e.register(SETTING_VALUE, |_, a| ret(a[0] + 4));
        zeros(&mut e, &[RELEASE_WORN_ITEM]);
        for setting in SETTINGS_ARMOUR_VALUE.iter().chain(&SETTINGS_CLOTHING_VALUE) {
            e.map(*setting, 8);
        }
        e.map(CLOTHING_CAP_COMPARE, 8);
        e.map(CLOTHING_CAP_VALUE, 4);
        // armour: 0.5 * value + 2.0 ; clothing: 0.25 * value + 1.0
        e.set_global(SETTINGS_ARMOUR_VALUE[0] + 4, 0.5f32);
        e.set_global(SETTINGS_ARMOUR_VALUE[1] + 4, 2.0f32);
        e.set_global(SETTINGS_CLOTHING_VALUE[0] + 4, 0.25f32);
        e.set_global(SETTINGS_CLOTHING_VALUE[1] + 4, 1.0f32);
        e.set_global(CLOTHING_CAP_COMPARE, 100.0f64);
        e.set_global(CLOTHING_CAP_VALUE, 99.0f32);
        let c = character(&mut e);
        start(&mut e);
        // 2 + 0.5 * 10 = 7, then 1 + 0.25 * 20 = 6: 13.
        assert_eq!(e.call(0x008d_3110, &args![c]).f32(), 13.0);
        let released = logged(&mut e)
            .iter()
            .filter(|(a, _)| *a == RELEASE_WORN_ITEM)
            .count();
        assert_eq!(released, 3);
        // Above the cap: the cap value.
        e.set_global(SETTINGS_ARMOUR_VALUE[1] + 4, 500.0f32);
        assert_eq!(e.call(0x008d_3110, &args![c]).f32(), 99.0);
        // Exactly the cap is not above it.
        e.set_global(CLOTHING_CAP_COMPARE, 511.0f64);
        assert_eq!(e.call(0x008d_3110, &args![c]).f32(), 511.0);
    }

    #[test]
    fn setting_getter_reads_through_the_setting_value() {
        let mut e = rig();
        e.register(SETTING_VALUE, |_, a| ret(a[0] + 4));
        e.map(SETTING_OF_8D3290, 8);
        e.set_global(SETTING_OF_8D3290 + 4, 7.5f32);
        let c = character(&mut e);
        start(&mut e);
        assert_eq!(e.call(0x008d_3290, &args![c]).f32(), 7.5);
        assert_eq!(logged(&mut e), [(SETTING_VALUE, vec![SETTING_OF_8D3290])]);
    }

    #[test]
    fn forwarders_pass_their_words_to_the_actor_functions() {
        let mut e = rig();
        let c = character(&mut e);
        zeros(
            &mut e,
            &[
                0x008a_9380,
                0x008a_9940,
                0x008a_a4c0,
                0x008a_bc40,
                0x008a_bfe0,
            ],
        );
        e.register(0x008a_9020, |_, a| ret(0xabcd_0000 | a[1]));
        let t = c.addr();
        start(&mut e);
        // The 16-bit result only.
        assert_eq!(e.call(0x008d_32b0, &args![c, 0x34u32]).u16(), 0x34);
        e.call(0x008d_32f0, &args![c, 5u32]);
        e.call(0x008d_3310, &args![c, 6u32, 7u32]);
        e.call(0x008d_3380, &args![c, 8u32, 9u32]);
        e.call(0x008d_34b0, &args![c, 10u32]);
        e.call(0x008d_34d0, &args![c, 11u32]);
        let log = e.call_log.take().unwrap();
        let inner: Vec<_> = log
            .into_iter()
            .filter(|(a, _)| *a < 0x008d_0000 || *a > 0x008e_0000)
            .collect();
        assert_eq!(
            inner,
            [
                (0x008a_9020, vec![t, 0x34]),
                (0x008a_9380, vec![t, 5]),
                (0x008a_9940, vec![t, 6, 7]),
                (0x008a_a4c0, vec![t, 8, 9]),
                (0x008a_bc40, vec![t, 10]),
                (0x008a_bfe0, vec![t, 11]),
            ]
        );
    }

    #[test]
    fn armour_fields_reset_after_the_actor_call_when_the_object_says_so() {
        let mut e = rig();
        let c = character(&mut e);
        e.register(0x008a_a210, |_, _| Ret::default());
        e.map(0x011d_e45c, 4);
        e.set_global(0x011d_e45c, 0x5555u32);
        e.register(0x0055_f5b0, |_, a| ret(u32::from(a[0] == 0x5555)));
        e.set(c, Character::fArmorRating, 3.0);
        e.set(c, Character::fArmorThreshold, 4.0);
        start(&mut e);
        e.call(0x008d_3330, &args![c, 0x42u32]);
        assert_eq!(
            logged(&mut e),
            [
                (0x008a_a210, vec![c.addr(), 0x42]),
                (0x0055_f5b0, vec![0x5555])
            ]
        );
        assert_eq!(e.get(c, Character::fArmorRating), -1.0);
        assert_eq!(e.get(c, Character::fArmorThreshold), -1.0);
        // The predicate false: untouched.
        e.set(c, Character::fArmorRating, 3.0);
        e.register(0x0055_f5b0, |_, _| ret(0));
        e.call(0x008d_3330, &args![c, 0x42u32]);
        assert_eq!(e.get(c, Character::fArmorRating), 3.0);
    }

    #[test]
    fn save_writes_the_actor_then_the_two_flag_bytes() {
        let mut e = rig();
        let c = character(&mut e);
        zeros(&mut e, &[0x008a_af40, 0x0086_5e50]);
        start(&mut e);
        e.call(0x008d_33a0, &args![c, Ptr::<()>::new(0x9000)]);
        let t = c.addr();
        assert_eq!(
            logged(&mut e),
            [
                (0x008a_af40, vec![t, 0x9000]),
                (0x0086_5e50, vec![0x9000, t + 0x1c0, 1, 0]),
                (0x0086_5e50, vec![0x9000, t + 0x1c1, 1, 0]),
            ]
        );
    }

    #[test]
    fn load_reads_extra_words_only_for_old_versions() {
        let mut e = rig();
        let c = character(&mut e);
        zeros(&mut e, &[0x008a_b3a0, 0x0086_4980]);
        let buffer = object(&mut e, 0x40);
        let t = c.addr();
        // Version 9: only the two flag bytes.
        e.register(slot_target(0), |_, _| ret(9));
        start(&mut e);
        e.call(0x008d_33f0, &args![c, Ptr::<()>::new(buffer)]);
        assert_eq!(
            logged(&mut e),
            [
                (0x008a_b3a0, vec![t, buffer]),
                (slot_target(0), vec![buffer]),
                (0x0086_4980, vec![buffer, t + 0x1c0, 1]),
                (0x0086_4980, vec![buffer, t + 0x1c1, 1]),
                (slot_target(0), vec![buffer]),
            ]
        );
        // Version 8: a byte and a word before, a byte and two words after.
        e.register(slot_target(0), |_, _| ret(8));
        start(&mut e);
        e.call(0x008d_33f0, &args![c, Ptr::<()>::new(buffer)]);
        let sizes: Vec<u32> = logged(&mut e)
            .iter()
            .filter(|(a, _)| *a == 0x0086_4980)
            .map(|(_, a)| a[2])
            .collect();
        assert_eq!(sizes, [1, 4, 1, 1, 1, 4, 4]);
    }

    #[test]
    fn reset_clears_the_armour_cache_and_flags() {
        let mut e = rig();
        let c = character(&mut e);
        e.register(0x008a_c1e0, |_, _| Ret::default());
        e.map(MINUS_ONE & !0xfff, 0x1000);
        e.set_global(MINUS_ONE, -1.0f32);
        e.set(c, Character::fArmorRating, 3.0);
        e.set(c, Character::fArmorThreshold, 4.0);
        e.set(c, Character::fequippedweight, 5.0);
        e.set(c, Character::bTrespassing, true);
        e.set(c, Character::bGuard, true);
        start(&mut e);
        e.call(0x008d_34f0, &args![c, 0x42u32]);
        assert_eq!(logged(&mut e), [(0x008a_c1e0, vec![c.addr(), 0x42])]);
        assert_eq!(e.get(c, Character::fArmorRating), -1.0);
        assert_eq!(e.get(c, Character::fArmorThreshold), -1.0);
        assert_eq!(e.get(c, Character::fequippedweight), -1.0);
        assert!(!e.get(c, Character::bTrespassing));
        assert!(!e.get(c, Character::bGuard));
    }

    /// Writes `text` at a scratch address and registers `0043b1b0` to return it.
    fn name_text(e: &mut Engine, text: &[u8]) {
        e.map(0x0500_0000, 0x1000);
        e.mem.set_cstr(0x0500_0000, text);
        e.register(0x0043_b1b0, |_, _| ret(0x0500_0000));
    }

    #[test]
    fn names_are_classified_by_their_first_characters() {
        let mut e = rig();
        type Classes = (i32, i32, i32);
        let cases: [(&[u8], Classes); 24] = [
            (b"An", (0, -1, -1)),
            (b"Ag", (-1, -1, 0)),
            (b"Bl123X", (-1, 1, -1)),
            (b"Bl123L", (-1, 0, -1)),
            (b"Bi", (-1, -1, 1)),
            (b"BM", (-1, -1, 2)),
            (b"Br12DxxxL", (-1, 2, -1)),
            (b"Br12Dxxxx", (-1, 3, -1)),
            (b"Br12I1L", (-1, 4, -1)),
            (b"Br12U1x", (-1, 7, -1)),
            (b"Co", (0xe, -1, -1)),
            (b"Fe", (1, -1, -1)),
            (b"Ex", (-1, -1, 6)),
            (b"Hxxxy", (2, -1, -1)),
            (b"HxxxY", (-1, 0x10, -1)),
            (b"LxxxR", (-1, 10, -1)),
            (b"Mxxxxn", (0xb, -1, -1)),
            (b"Mxxxxnn", (7, -1, -1)),
            (b"Mxxxxa", (0xc, -1, -1)),
            (b"Oh", (-1, -1, 0xb)),
            (b"Sa", (3, -1, -1)),
            (b"Sq1234L", (-1, 0xc, -1)),
            (b"Su", (4, -1, -1)),
            (b"Gx", (-1, -1, -1)),
        ];
        for (text, expected) in cases {
            name_text(&mut e, text);
            assert_eq!(
                classify_name(&mut e, 0x1000),
                expected,
                "{}",
                String::from_utf8_lossy(text)
            );
        }
        // Single-letter families and the characters outside the table.
        for (letter, expected) in [
            (b'D', (-1, -1, 4)),
            (b'I', (-1, -1, 8)),
            (b'K', (-1, -1, 9)),
            (b'N', (-1, -1, 10)),
            (b'P', (0xd, -1, -1)),
            (b'R', (-1, -1, 0xd)),
            (b'T', (-1, -1, 0xe)),
            (b'W', (-1, -1, 0xf)),
            (b'Z', (-1, -1, -1)),
            (b'a', (-1, -1, -1)),
        ] {
            name_text(&mut e, &[letter, b'x']);
            assert_eq!(
                classify_name(&mut e, 0x1000),
                expected,
                "{}",
                letter as char
            );
        }
    }

    /// Everything `008d3550` reaches, with one controller named "Agility".
    struct UpdateRig {
        e: Engine,
        c: Ptr<Character>,
        process: u32,
    }
    const OBJ_A: u32 = 0x0400_0000;
    const OBJ_B: u32 = 0x0400_0100;
    const OBJ_C: u32 = 0x0400_0200;
    const OBJ_TARGET: u32 = 0x0400_0300;
    const OBJ_LIST: u32 = 0x0400_0500;
    const OBJ_HEAD: u32 = 0x0400_0700;
    const OBJ_ENTRIES: u32 = 0x0400_0800;

    fn update_rig() -> UpdateRig {
        let mut e = rig();
        e.map(0x0400_0000, 0x2000);
        for o in [OBJ_A, OBJ_B, OBJ_C, OBJ_TARGET, OBJ_LIST] {
            e.mem.set_u32(o, VT);
        }
        let c = character(&mut e);
        let process = with_process(&mut e, c);
        e.register(slot_target(0x1d0), |_, _| ret(0x6000));
        e.register(slot_target(0x790), |_, _| ret(OBJ_A));
        e.register(slot_target(0x798), |_, _| ret(OBJ_B));
        e.register(slot_target(0x7a0), |_, _| ret(OBJ_C));
        e.register(slot_target(0x100), |_, _| ret(OBJ_TARGET));
        e.register(slot_target(0xc4), |_, _| ret(0x0400_0900));
        // The list: head at +0x38, one 16-byte entry.
        e.mem.set_u32(OBJ_LIST + 0x38, OBJ_HEAD);
        e.mem.set_u32(OBJ_HEAD + 8, 1);
        e.mem.set_u32(OBJ_HEAD + 0x10, OBJ_ENTRIES);
        e.register(WORD_AT_0, |e, a| ret(e.mem.u32(a[0])));
        e.register(WORD_AT_8, |e, a| ret(e.mem.u32(a[0] + 8)));
        acquire_double(&mut e);
        zeros(
            &mut e,
            &[
                0x0088_8b50,
                0x004e_af60,
                0x0064_cc90,
                0x0048_af40,
                0x0043_81b0,
                0x004a_de00,
                0x0065_3270,
            ],
        );
        e.register(0x004f_0140, |_, _| ret(1));
        e.register(0x0045_6610, |_, _| ret(0));
        e.register(FORM_TYPE_OF, |_, _| ret(0x77));
        e.register(0x00a5_c570, |_, _| ret(OBJ_LIST));
        e.register(0x0045_bad0, |_, a| {
            assert_eq!(a[0], 0x011f_3e58);
            ret(1)
        });
        e.register(0x0050_8100, |_, _| retf(1.25));
        e.register(0x00a4_1590, |_, a| ret(a[0] + 4));
        e.map(0x011c_6000, 0x1000);
        e.set_global(0x011c_6240, 0x11u32);
        e.set_global(0x011c_6244, 0x22u32);
        e.set_global(0x011c_6248, 0x33u32);
        name_text(&mut e, b"Agility");
        UpdateRig { e, c, process }
    }

    #[test]
    fn update_hands_the_named_value_to_both_animation_objects() {
        let UpdateRig { mut e, c, process } = update_rig();
        let t = c.addr();
        start(&mut e);
        e.call(0x008d_3550, &args![c, 0.5f32]);
        let log = logged(&mut e);
        let v = slot_target;
        assert_eq!(log[0], (0x0088_8b50, vec![t, 0.5f32.to_bits()]));
        // "Agility": 'A' not followed by 'n' sets the third family (index 0).
        let value = 1.25f32.to_bits();
        let applied: Vec<_> = log
            .iter()
            .filter(|(a, _)| *a == v(0xe4) || *a == v(0xdc) || *a == v(0xe0))
            .collect();
        assert_eq!(applied.len(), 2);
        assert_eq!(*applied[0], (v(0xe4), vec![OBJ_TARGET, 0, value]));
        assert_eq!(*applied[1], (v(0xe4), vec![OBJ_TARGET, 0, value]));
        // The frozen flag goes to both targets, from the form type of C.
        let freezes: Vec<_> = log.iter().filter(|(a, _)| *a == 0x0064_cc90).collect();
        assert_eq!(freezes.len(), 2);
        assert_eq!(*freezes[0], (0x0064_cc90, vec![OBJ_TARGET, 0]));
        assert!(log.contains(&(FORM_TYPE_OF, vec![OBJ_C])));
        assert!(log.contains(&(0x00a5_c570, vec![OBJ_C, 0x011f_3728])));
        assert!(log.contains(&(v(0xc4), vec![OBJ_LIST, 0])));
        // The name temporary is built from the entry's name (entry + 4) and
        // destroyed.
        assert!(log.contains(&(0x00a4_1590, vec![OBJ_ENTRIES])));
        assert!(log.iter().any(|(a, _)| *a == 0x0043_81b0));
        let _ = process;
    }

    #[test]
    fn update_creates_missing_animation_objects_and_stops_when_frozen() {
        let UpdateRig { mut e, c, process } = update_rig();
        let t = c.addr();
        // The process has none of the three objects.
        e.register(slot_target(0x790), |_, _| ret(0));
        e.register(slot_target(0x798), |_, _| ret(0));
        e.register(slot_target(0x7a0), |_, _| ret(0));
        e.register(0x004a_de00, |_, a| {
            ret(match a[1] {
                0x11 => OBJ_A,
                0x22 => OBJ_B,
                _ => OBJ_C,
            })
        });
        e.register(0x0065_3270, |_, a| {
            assert_eq!(a[0], 0x011d_5bf8);
            ret(a[1])
        });
        start(&mut e);
        e.call(0x008d_3550, &args![c, 0.0f32]);
        let log = logged(&mut e);
        let v = slot_target;
        assert!(log.contains(&(0x004a_de00, vec![0x6000, 0x11])));
        assert!(log.contains(&(0x0065_3270, vec![0x011d_5bf8, OBJ_A])));
        assert!(log.contains(&(v(0x794), vec![process, OBJ_A])));
        assert!(log.contains(&(v(0x79c), vec![process, OBJ_B])));
        assert!(log.contains(&(v(0x7a4), vec![process, OBJ_C])));
        // Frozen (456610 of the form type is true): flags passed, then stop.
        e.register(0x0045_6610, |_, a| ret(u32::from(a[0] == 0x77)));
        // b's own check also uses 456610 (with b): keep it false for b.
        start(&mut e);
        e.call(0x008d_3550, &args![c, 0.0f32]);
        let log = logged(&mut e);
        assert!(log.iter().all(|(a, _)| *a != 0x00a5_c570));
        // The player without the flag stops right after the actor update.
        e.set_global(PLAYER_SINGLETON, t);
        start(&mut e);
        e.call(0x008d_3550, &args![c, 0.0f32]);
        assert_eq!(logged(&mut e).len(), 2);
    }

    #[test]
    fn update_skips_negative_values_and_empty_names() {
        let UpdateRig { mut e, c, .. } = update_rig();
        let v = slot_target;
        e.register(0x0050_8100, |_, _| retf(-0.5));
        start(&mut e);
        e.call(0x008d_3550, &args![c, 0.0f32]);
        assert!(logged(&mut e).iter().all(|(a, _)| *a != v(0xe4)));
        e.register(0x0050_8100, |_, _| retf(f64::NAN));
        start(&mut e);
        e.call(0x008d_3550, &args![c, 0.0f32]);
        assert!(logged(&mut e).iter().all(|(a, _)| *a != v(0xe4)));
        // An empty name.
        e.register(0x0050_8100, |_, _| retf(1.0));
        name_text(&mut e, b"");
        start(&mut e);
        e.call(0x008d_3550, &args![c, 0.0f32]);
        assert!(logged(&mut e).iter().all(|(a, _)| *a != v(0xe4)));
        // A value of exactly 0.0 is applied (the test is "not below 0").
        e.register(0x0050_8100, |_, _| retf(0.0));
        name_text(&mut e, b"Agility");
        start(&mut e);
        e.call(0x008d_3550, &args![c, 0.0f32]);
        assert_eq!(
            logged(&mut e).iter().filter(|(a, _)| *a == v(0xe4)).count(),
            2
        );
    }

    #[test]
    fn list_helpers_read_the_array_at_0x38() {
        let mut e = rig();
        e.register(WORD_AT_0, |e, a| ret(e.mem.u32(a[0])));
        e.register(WORD_AT_8, |e, a| ret(e.mem.u32(a[0] + 8)));
        e.map(0x0400_0000, 0x1000);
        let this = 0x0400_0000;
        // No head: count 0.
        assert_eq!(e.call(0x008d_3ed0, &args![Ptr::<()>::new(this)]).u32(), 0);
        e.mem.set_u32(this + 0x38, this + 0x100);
        e.mem.set_u32(this + 0x108, 3);
        e.mem.set_u32(this + 0x110, this + 0x200);
        assert_eq!(e.call(0x008d_3ed0, &args![Ptr::<()>::new(this)]).u32(), 3);
        // Element access: 16 bytes each, null past the count.
        let at =
            |e: &mut Engine, i: u32| e.call(0x008d_3f10, &args![Ptr::<()>::new(this), i]).u32();
        assert_eq!(at(&mut e, 0), this + 0x200);
        assert_eq!(at(&mut e, 2), this + 0x220);
        assert_eq!(at(&mut e, 3), 0);
        let element = e
            .call(0x008d_3f40, &args![Ptr::<()>::new(this + 0x100), 1u32])
            .u32();
        assert_eq!(element, this + 0x210);
        assert_eq!(
            e.call(0x008d_3f40, &args![Ptr::<()>::new(this + 0x100), 3u32])
                .u32(),
            0
        );
    }

    #[test]
    fn global_getters_return_their_words() {
        let mut e = rig();
        e.map(0x011c_6000, 0x1000);
        e.set_global(0x011c_6240, 1u32);
        e.set_global(0x011c_6244, 2u32);
        e.set_global(0x011c_6248, 3u32);
        assert_eq!(e.call(0x008d_3f70, &args![]).u32(), 1);
        assert_eq!(e.call(0x008d_3f80, &args![]).u32(), 2);
        assert_eq!(e.call(0x008d_3f90, &args![]).u32(), 3);
    }

    #[test]
    fn reset_3d_is_queued_or_run_through_the_process() {
        let mut e = rig();
        acquire_double(&mut e);
        e.register(0x0043_fcd0, |_, _| ret(0));
        e.register(0x008c_7aa0, |_, _| ret(1));
        zeros(
            &mut e,
            &[
                0x0045_37b0,
                0x0087_b5c0,
                0x0070_5990,
                0x0088_54b0,
                0x0045_cd60,
            ],
        );
        e.register(0x0045_37b0, |_, _| ret(0x8000));
        e.register(0x0070_5990, |_, _| ret(OBJ_A));
        e.map(OBJ_A, 0x100);
        e.mem.set_u32(OBJ_A, VT);
        let c = character(&mut e);
        let process = with_process(&mut e, c);
        let t = c.addr();
        let v = slot_target;
        // No 3D: nothing.
        start(&mut e);
        e.call(0x008d_3fa0, &args![c]);
        assert_eq!(logged(&mut e), [(0x0043_fcd0, vec![t])]);
        // 3D present, tasks queued.
        e.register(0x0043_fcd0, |_, _| ret(1));
        start(&mut e);
        e.call(0x008d_3fa0, &args![c]);
        assert_eq!(
            logged(&mut e),
            [
                (0x0043_fcd0, vec![t]),
                (0x008c_7aa0, vec![]),
                (0x0045_37b0, vec![]),
                (0x0087_b5c0, vec![0x8000, t]),
            ]
        );
        // Run directly, as the player.
        e.register(0x008c_7aa0, |_, _| ret(0));
        e.set_global(PLAYER_SINGLETON, t);
        start(&mut e);
        e.call(0x008d_3fa0, &args![c]);
        assert_eq!(
            logged(&mut e),
            [
                (0x0043_fcd0, vec![t]),
                (0x008c_7aa0, vec![]),
                (0x0070_5990, vec![]),
                (v(0x28), vec![OBJ_A]),
                (ACTOR_PROCESS, vec![t]),
                (v(0x468), vec![process, 1]),
                (ACTOR_PROCESS, vec![t]),
                (v(0x468), vec![process, 2]),
                (ACTOR_PROCESS, vec![t]),
                (v(0x468), vec![process, 4]),
                (ACTOR_PROCESS, vec![t]),
                (v(0x468), vec![process, 0x10]),
                (ACTOR_PROCESS, vec![t]),
                (v(0x464), vec![process, t]),
                (0x0070_5990, vec![]),
                (v(0x24), vec![OBJ_A]),
                (0x0045_cd60, vec![process]),
                (0x0088_54b0, vec![process, 0]),
            ]
        );
        // Not the player, and the process flag set: no pipboy, no final call.
        e.set_global(PLAYER_SINGLETON, 0u32);
        e.register(0x0045_cd60, |_, _| ret(1));
        start(&mut e);
        e.call(0x008d_3fa0, &args![c]);
        let log = logged(&mut e);
        assert!(log
            .iter()
            .all(|(a, _)| *a != 0x0070_5990 && *a != 0x0088_54b0));
    }
}

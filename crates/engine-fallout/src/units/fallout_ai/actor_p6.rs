//! `fallout/ai/actor.cpp` (Xbox PDB source unit), part 6: its functions from `008b8e90` up to
//! (not including) `008bec80` in FalloutNV.exe 1.4.0.525, translated
//! (docs/ENGINE_CRATE.md). The unit's shared layouts and helpers are in
//! [`super::actor`]; anything public there may be used here.
//!
//! Session 2 covers `008bb630` to `008bcda0` (40 functions: iron sights, the use-weapon
//! package, the greeting, follower and crime-list helpers, fades). Session 1 covers
//! `008b8e90` to `008bb5c0` (40 functions): the
//! faction test, the `ActorValueOwner` change callbacks (condition, mobility,
//! endurance and the limb/stat callbacks that mark `CachedValues` stale), the
//! `CachedValuesOwner` virtuals that refresh the cached stats of a process,
//! `QueueReplacementLocomotion`, the process-flag getters, the animation
//! idle tests, the big handler of the process's pending animation/equip
//! flags (`008ba600`) and the two rotate requests.
//!
//! Conventions: the `ActorValueOwner` subobject sits at `Actor + 0xa4` and
//! the `CachedValuesOwner` at `Actor + 0xa8` on PC; the callbacks receive
//! the `ActorValueOwner` pointer and subtract `0xa4` (after a null test) to
//! get the actor. x87 note: the game computes in extended precision and
//! stores `float` results; the translations compute in `f32` where the code
//! stores a `float` and compare through the same ordered/unordered rules.

#[allow(unused_imports)]
use super::actor::*;
#[allow(unused_imports)]
use crate::prelude::*;

/// `TESObjectREFR` base form getter (`this + 0x20` through `007af430`).
const GET_BASE_FORM: u32 = 0x0041_81e0;
/// Returns the `ExtraDataList` embedded in a reference (`reference + 0x44`).
const EXTRA_LIST_OF_REFERENCE: u32 = 0x005d_43c0;
/// `ExtraDataList` lookup of the extra data of type `0x5e` (the code treats
/// the result as an `ExtraFactionChanges`).
const EXTRA_LIST_GET_FACTION_CHANGES: u32 = 0x0042_e800;
/// `ExtraDataList` call that adds the extra data `0042e800` looks up (called
/// when the lookup found none).
const EXTRA_LIST_ADD_FACTION_CHANGES: u32 = 0x0042_e760;
/// `ECX` = faction list of the base form (`base form + 0x30`), one word
/// (the faction): true when the list holds the faction.
const BASE_FACTION_LIST_CONTAINS: u32 = 0x0047_ebf0;
/// `ExtraFactionChanges::GetIsExpelled` (named by the decompiler): `ECX` =
/// the extra data, one word (the faction).
const FACTION_CHANGES_IS_EXPELLED: u32 = 0x0043_7080;
/// `ExtraFactionChanges::GetIsInFaction` (named by the decompiler).
const FACTION_CHANGES_IS_IN_FACTION: u32 = 0x0043_7010;
/// `ECX` = extra data (`ExtraFactionChanges`), one word: the call
/// `008b8f20` makes with its argument once the extra data exists.
const FACTION_CHANGES_ADD: u32 = 0x0043_6e10;
/// `MobileObject::GetCharController` (Xbox PDB).
const GET_CHAR_CONTROLLER: u32 = 0x0093_06d0;
/// Tells whether a list node (`ECX`) is empty: both words `+0` and `+4`
/// are zero.
const LIST_NODE_IS_EMPTY: u32 = 0x0082_56d0;
/// Returns its `this` (`ECX`): the address of the list node's data word.
const LIST_NODE_DATA_ADDRESS: u32 = 0x0068_15c0;
/// Advances the list node in `ECX` and returns the next one (the list
/// iterator step; also used to read a count).
const LIST_NODE_NEXT: u32 = 0x0072_6070;
/// `NiPointer`/smart pointer dereference: returns the word `ECX` points to.
const POINTER_GET: u32 = 0x0055_9450;
/// `bhkCharacterProxy::operator*` (map name `operatorP`): `ECX` = the
/// proxy, returns the object `004b5a20` takes.
const PROXY_OBJECT: u32 = 0x004a_e750;
/// `cdecl`, one word: resolves a Havok object to the one `0044ddc0` reads.
const HAVOK_TO_BODY: u32 = 0x004b_5a20;
/// `ECX` = an object (the havok body above, or an item entry): returns the
/// word it holds at `+0x08`.
const ENTRY_OBJECT: u32 = 0x0044_ddc0;
/// `TESObjectREFR::FindReferenceFor3D` (Xbox PDB), `cdecl`, one word.
const FIND_REFERENCE_FOR_3D: u32 = 0x0056_f930;
/// `ECX` = the player character: the reference the contact test compares
/// against.
const PLAYER_EXCLUDED_REFERENCE: u32 = 0x0089_f4e0;
/// `ECX` = contact entry `+0x1c`, `float` in ST0.
const CONTACT_VALUE_A: u32 = 0x0045_7990;
/// `ECX` = the object behind the contact entry, `float` in ST0.
const CONTACT_VALUE_B: u32 = 0x004b_5400;
/// `cdecl`, two `float` arguments, `float` in ST0: the impact damage.
const IMPACT_DAMAGE: u32 = 0x0062_be90;
/// `operator new` (`cdecl`, size).
const OPERATOR_NEW: u32 = 0x0040_1000;
/// `HitData::HitData` (map name): `ECX` = the block.
const HIT_DATA_CONSTRUCTOR: u32 = 0x009b_4d90;
/// `NiPointer<HitData>` constructor from a raw pointer (`ECX` = the slot).
const HIT_DATA_POINTER_CONSTRUCTOR: u32 = 0x008c_1c30;
/// `NiPointer<HitData>` destructor (`ECX` = the slot).
const HIT_DATA_POINTER_DESTRUCTOR: u32 = 0x008c_1c60;
/// `HitData::InitializeImpactData` (Xbox PDB), `cdecl`: hit data, struck
/// reference, actor, damage, contact entry.
const INITIALIZE_IMPACT_DATA: u32 = 0x009b_58d0;
/// `Actor::HitMe` (Xbox PDB): hit data and a flag.
const ACTOR_HIT_ME: u32 = 0x0089_a760;
/// `ECX` = reference: seven words, the character controller and two vectors.
const REACT_TO_CONTACT: u32 = 0x009b_4bf0;

/// The player character pointer global.
const PLAYER_POINTER: u32 = 0x011d_ea3c;
/// The HUD main menu pointer global (`HUDMainMenu` singleton).
const HUD_MAIN_MENU_POINTER: u32 = 0x011d_96c0;

/// Tells whether the process (`ECX`) has a cached-values block.
const HAS_CACHED_VALUES: u32 = 0x0088_4920;
/// `ECX` = process, one word: forwards a `CachedValues::iFlags` mask to the
/// cached-values block.
const PROCESS_FORWARD_FLAG_MASK: u32 = 0x0088_4940;
/// `ECX` = cached values, one word: ORs the mask into `iFlags`.
const CACHED_VALUES_ADD_FLAGS: u32 = 0x0088_48c0;
/// `ECX` = actor: marks the actor's limb-dependent cached values stale.
const REFRESH_AFTER_LIMB_CHANGE: u32 = 0x0088_48e0;
/// `ECX` = actor: recomputes the actor's mobility state before the
/// replacement locomotion is queued.
const REFRESH_BEFORE_QUEUE: u32 = 0x0088_4f80;
/// `Actor::RestoreActorValue` (Xbox PDB): `ECX` = actor, actor value index
/// and a `float`.
const RESTORE_ACTOR_VALUE: u32 = 0x0088_b740;
/// `CombatFormulas::GetBodyPartCondition` (Xbox PDB), `cdecl`: the
/// `ActorValueOwner`, an actor value index and a flag, `float` in ST0.
const GET_BODY_PART_CONDITION: u32 = 0x0064_6800;
/// `cdecl`, `ActorValueOwner` and a `float`, `float` in ST0 (called by the
/// endurance callback with the summed value).
const ENDURANCE_ADJUSTMENT: u32 = 0x0064_36c0;
/// `TESActorBase::GetHealth` (Xbox PDB): `ECX` = the base form.
const GET_BASE_HEALTH: u32 = 0x005f_0b00;
/// `cdecl`, `ActorValueOwner`, `float` in ST0: the value `008b9b70` caches.
const CACHED_VALUE_MEDICINE: u32 = 0x0064_9530;
/// `cdecl`, `ActorValueOwner`, `float` in ST0: the value `008b9c10` caches.
const CACHED_VALUE_SURVIVAL: u32 = 0x0064_9580;
/// `SurgeryMenu::ResetLimb` (Xbox PDB), `cdecl`: the limb actor value.
const SURGERY_MENU_RESET_LIMB: u32 = 0x007e_5810;
/// `HUDMainMenu::ShowCrippledGuy` (Xbox PDB): `ECX` = the HUD menu.
const SHOW_CRIPPLED_GUY: u32 = 0x0077_eaf0;
/// `Actor::TriggerPain` (Xbox PDB): two words.
const TRIGGER_PAIN: u32 = 0x008a_7d50;
/// `MiscStatManager::Increment` (Xbox PDB), `cdecl`: the statistic index.
const MISC_STAT_INCREMENT: u32 = 0x004d_5c60;
/// `strcpy`-like helper, `cdecl`: destination and source.
const COPY_STRING: u32 = 0x0040_46f0;
/// `snprintf`-like helper, `cdecl`: buffer, size, format, arguments.
const FORMAT_STRING: u32 = 0x0040_6d00;
/// `BGSBodyPartData::GetBodyPart_ov2` (Xbox PDB): `ECX` = the data, one word
/// (the actor value index).
const GET_BODY_PART: u32 = 0x005e_5130;
/// `ECX` = body part, returns the name pointer at `+0x1c` (through
/// `00559450`).
const BODY_PART_NAME: u32 = 0x0068_38b0;
/// `cdecl`: actor value index, body part name, buffer, size: writes the
/// limb text.
const BUILD_LIMB_TEXT: u32 = 0x0064_95d0;
/// `ECX` = actor: returns the actor's display name through `00891170` and
/// `TESFullName::GetFullName`.
const ACTOR_NAME: u32 = 0x0055_d520;
/// `ECX` = the HUD menu: shows a message (text, flag, icon path, sound
/// name, `float` time, flag).
const SHOW_MESSAGE: u32 = 0x0077_5380;
/// `Actor::QueueEquipObject` (Xbox PDB).
const QUEUE_EQUIP_OBJECT: u32 = 0x0088_c650;
/// `ECX` = process, mask and flag (`008acf70`): sets or clears the mask in
/// the cached-values block when it exists.
const PROCESS_SET_STATE_FLAG: u32 = 0x008a_cf70;
/// `ECX` = actor, a flag and a word (`008b7360`): the locomotion change
/// `QueueReplacementLocomotion` requests.
const REQUEST_LOCOMOTION_CHANGE: u32 = 0x008b_7360;
/// Returns the actor's process (`this + 0x68`): the map names the body
/// `MiddleHighProcess::GetSavedAcquireObject`, identical-code folding put
/// that name on this getter.
const ACTOR_PROCESS: u32 = 0x008d_8520;
/// `ECX` = actor: returns the combat state `005c1ae0` takes.
const ACTOR_COMBAT_CONTROLLER_STATE: u32 = 0x008a_0330;
/// `ECX` = the object `008a0330` returned.
const COMBAT_CONTROLLER_APPLY: u32 = 0x005c_1ae0;
/// `ECX` = actor (`008c4400`).
const ACTOR_CALLBACK_008C4400: u32 = 0x008c_4400;
/// `PlayerCharacter::IsGodMode` (Xbox PDB), no arguments.
const PLAYER_IS_GOD_MODE: u32 = 0x0095_26b0;
/// `ECX` = player: returns a `float` in ST0 (`008a0c20`).
const PLAYER_VALUE_008A0C20: u32 = 0x008a_0c20;
/// Object the message call `004c69f0` uses as `this`.
const MESSAGE_QUEUE: u32 = 0x011d_4618;
/// `ECX` = `0x011d4618`: builds a message (`0`, icon path, `0`, `float`
/// time, `0`).
const MESSAGE_QUEUE_ADD: u32 = 0x004c_69f0;
/// `cdecl`, the message `004c69f0` returned.
const MESSAGE_QUEUE_SHOW: u32 = 0x0070_52f0;

/// Format string `"%s %s"`.
const FORMAT_TWO_STRINGS: u32 = 0x0101_2058;
/// The empty string the message buffer is initialised with.
const EMPTY_STRING: u32 = 0x0101_1584;
/// Icon path `Interface\Icons\Message Icons\glow_message_vaultboy_very_happy.dds`.
const ICON_VERY_HAPPY: u32 = 0x0102_5cc4;
/// Sound name `UIPopUpMessageGeneral`.
const SOUND_POPUP_GENERAL: u32 = 0x0103_a830;
/// Icon path `Interface\Icons\Message Icons\glow_message_vaultboy_surprised.dds`.
const ICON_SURPRISED: u32 = 0x0104_9638;
/// `float` display time of the messages.
const MESSAGE_TIME: u32 = 0x0101_62c0;
/// `float` the limb callback passes to virtual `+0x394` (100.0).
const FULL_VALUE: u32 = 0x0101_6410;
/// The `double` the replacement-locomotion test compares against (2.0).
const DOUBLE_TWO: u32 = 0x0101_1590;
/// `1.0` as a `double`.
const DOUBLE_ONE: u32 = 0x0101_2070;

/// How far the `ActorValueOwner` subobject sits inside an `Actor` on PC.
const ACTOR_VALUE_OWNER_OFFSET: u32 = 0xa4;
/// `Actor` virtual `+0x360`: a test the condition callbacks use on the actor
/// (meaning not confirmed).
const VSLOT_ACTOR_TEST_360: u32 = 0x360;
/// `Actor` virtual `+0x364`: the last test of the ragdoll update's enabled byte (008bd287).
const VSLOT_ACTOR_0X364: u32 = 0x364;
/// `Actor` virtual `+0x394`: two words (`0` and a `float`).
const VSLOT_ACTOR_0X394: u32 = 0x394;
/// `Actor` virtual `+0x428`: `GetCombatController` (Xbox PDB `+0x424`).
const VSLOT_GET_COMBAT_CONTROLLER: u32 = 0x428;
/// Virtual `+0x180` of the base form (the result of `004181e0`): returns
/// the body part data.
const VSLOT_BASE_BODY_PART_DATA: u32 = 0x180;
/// `ActorValueOwner` virtual `+0x0c`: the current value of an actor value,
/// `float` in ST0.
const OWNER_VSLOT_VALUE: u32 = 0xc;
/// `ActorValueOwner` virtual `+0x20`: a `float` read by `008b9960`.
const OWNER_VSLOT_0X20: u32 = 0x20;
/// `ActorValueOwner` virtual `+0x08`: one word, a non-zero answer stops
/// `008b9830`.
const OWNER_VSLOT_0X08: u32 = 0x08;
/// `ActorValueOwner` virtuals `+0x10`, `+0x14`, `+0x18`: `float`s the
/// endurance callback sums.
const OWNER_VSLOT_0X10: u32 = 0x10;
const OWNER_VSLOT_0X14: u32 = 0x14;
const OWNER_VSLOT_0X18: u32 = 0x18;

/// `Actor` field `pCurrentProcess` at `+0x68`.
fn process_of(e: &Engine, actor: u32) -> u32 {
    e.get(Ptr::<Actor>::new(actor), Actor::pCurrentProcess)
        .addr()
}

/// The actor an `ActorValueOwner` pointer belongs to (the callbacks test the
/// owner for null first).
fn actor_of_owner(owner: u32) -> u32 {
    if owner == 0 {
        0
    } else {
        owner.wrapping_sub(ACTOR_VALUE_OWNER_OFFSET)
    }
}

/// The `ActorValueOwner` of an actor (`actor + 0xa4`, null for null).
fn owner_of_actor(actor: u32) -> u32 {
    if actor == 0 {
        0
    } else {
        actor.wrapping_add(ACTOR_VALUE_OWNER_OFFSET)
    }
}

/// The tail every limb and stat callback shares: when the actor has a
/// process with a cached-values block, forward `mask` to it so the cached
/// values are recomputed.
fn forward_cached_flags(e: &mut Engine, actor: u32, mask: u32) {
    let process = process_of(e, actor);
    if process != 0 && e.call(HAS_CACHED_VALUES, &args![process]).bool() {
        e.call(PROCESS_FORWARD_FLAG_MASK, &args![process, mask]);
    }
}

/// Body shared by the `CachedValuesOwner` virtuals: the cached value was
/// just computed; when the process (at `this - 0x40`, the `Actor + 0x68`
/// of an owner at `Actor + 0xa8`) has a cached-values block, hand the value
/// to `store`.
fn store_cached_value(e: &mut Engine, this: u32, value: f32, store: fn(&mut Engine, Ptr, f32)) {
    let process = e.mem.u32(this.wrapping_sub(0x40));
    if process != 0 && e.call(HAS_CACHED_VALUES, &args![process]).bool() {
        store(e, Ptr::new(process), value);
    }
}

/// Writes `value` at `cached + offset` of the process's cached-values block
/// and ORs `mask` into its `iFlags`, when the block exists.
fn set_cached_value(e: &mut Engine, process: Ptr, offset: u32, value: f32, mask: u32) {
    // BaseProcess::pCachedValues (Xbox PDB) +0x2c
    let cached = e.mem.u32(process.addr() + 0x2c);
    if cached != 0 {
        e.mem.set_f32(cached + offset, value);
        e.call(CACHED_VALUES_ADD_FLAGS, &args![cached, mask]);
    }
}

// Translated from 008b8e90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Actor::IsInFaction` (Xbox PDB): true when the actor is in `faction`,
/// through its base form's faction list and the reference's
/// `ExtraFactionChanges`. Without the extra data the base answer stands;
/// with it, a base membership counts unless the extra data expels the
/// actor, and otherwise the extra data's own membership decides.
pub fn actor_is_in_faction(e: &mut Engine, this: Ptr<Actor>, faction: Ptr) -> bool {
    let list = e.call(EXTRA_LIST_OF_REFERENCE, &args![this]).u32();
    let changes = e.call(EXTRA_LIST_GET_FACTION_CHANGES, &args![list]).u32();
    let base = e.call(GET_BASE_FORM, &args![this]).u32();
    let in_base = e
        .call(BASE_FACTION_LIST_CONTAINS, &args![base + 0x30, faction])
        .bool();
    if changes == 0 {
        return in_base;
    }
    if in_base
        && !e
            .call(FACTION_CHANGES_IS_EXPELLED, &args![changes, faction])
            .bool()
    {
        return true;
    }
    e.call(FACTION_CHANGES_IS_IN_FACTION, &args![changes, faction])
        .bool()
}

// Translated from 008b8f20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Calls the actor's virtual `+0x48` with `0x80000000`, then makes sure the
/// reference has its `ExtraFactionChanges` (adding it when the lookup finds
/// none) and hands `argument` to it (`00436e10`).
pub fn fn_008b8f20(e: &mut Engine, this: Ptr<Actor>, argument: u32) {
    e.vcall(this.addr(), 0x48, &args![0x8000_0000u32]);
    let list = e.call(EXTRA_LIST_OF_REFERENCE, &args![this]).u32();
    let mut changes = e.call(EXTRA_LIST_GET_FACTION_CHANGES, &args![list]).u32();
    if changes == 0 {
        let list = e.call(EXTRA_LIST_OF_REFERENCE, &args![this]).u32();
        e.call(EXTRA_LIST_ADD_FACTION_CHANGES, &args![list]);
        let list = e.call(EXTRA_LIST_OF_REFERENCE, &args![this]).u32();
        changes = e.call(EXTRA_LIST_GET_FACTION_CHANGES, &args![list]).u32();
    }
    if changes != 0 {
        e.call(FACTION_CHANGES_ADD, &args![changes, argument]);
    }
}

// Translated from 008b8f90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Walks the contacts the actor's character controller recorded (the list
/// head sits at controller `+0x648`): for each contact with a reference
/// other than the player's, a reference answering true to virtual `+0x224`
/// whose virtual `+0x304` returns 3 gets `009b4bf0` (the controller and the
/// two vectors of the contact); any other reference takes impact damage
/// (`0062be90` of two values read from the contact) when it is positive,
/// through a new `HitData` and `Actor::HitMe`. C++ exception unwinding of
/// the `NiPointer<HitData>` local is left out.
pub fn fn_008b8f90(e: &mut Engine, this: Ptr<Actor>) {
    if e.call(GET_CHAR_CONTROLLER, &args![this]).u32() == 0 {
        return;
    }
    let controller = e.call(GET_CHAR_CONTROLLER, &args![this]).u32();
    let mut node = fn_008b9220(e, Ptr::new(controller)).addr();
    while node != 0 && !e.call(LIST_NODE_IS_EMPTY, &args![node]).bool() {
        let data_address = e.call(LIST_NODE_DATA_ADDRESS, &args![node]).u32();
        let contact = e.mem.u32(data_address);
        contact_event(e, this, contact);
        node = e.call(LIST_NODE_NEXT, &args![node]).u32();
    }
}

/// One iteration of the loop of `008b8f90`.
fn contact_event(e: &mut Engine, this: Ptr<Actor>, contact: u32) {
    let body = if e.call(POINTER_GET, &args![contact]).u32() != 0 {
        let proxy = e.call(POINTER_GET, &args![contact]).u32();
        let object = e.call(PROXY_OBJECT, &args![proxy]).u32();
        e.call(HAVOK_TO_BODY, &args![object]).u32()
    } else {
        0
    };
    let object = if body != 0 {
        e.call(ENTRY_OBJECT, &args![body]).u32()
    } else {
        0
    };
    let reference = if object != 0 {
        e.call(FIND_REFERENCE_FOR_3D, &args![object]).u32()
    } else {
        0
    };
    if reference == 0 {
        return;
    }
    let player = e.global::<u32>(PLAYER_POINTER);
    if reference == e.call(PLAYER_EXCLUDED_REFERENCE, &args![player]).u32() {
        return;
    }
    if e.vcall(reference, 0x224, &args![]).bool() {
        if e.vcall(reference, 0x304, &args![]).u32() == 3 {
            let controller = e.call(GET_CHAR_CONTROLLER, &args![this]).u32();
            let vector_a = [
                e.mem.u32(contact + 0x10),
                e.mem.u32(contact + 0x14),
                e.mem.u32(contact + 0x18),
            ];
            let vector_b = [
                e.mem.u32(contact + 4),
                e.mem.u32(contact + 8),
                e.mem.u32(contact + 0xc),
            ];
            e.call(
                REACT_TO_CONTACT,
                &args![
                    reference,
                    controller,
                    vector_b[0],
                    vector_b[1],
                    vector_b[2],
                    vector_a[0],
                    vector_a[1],
                    vector_a[2]
                ],
            );
        }
        return;
    }
    let value_a = e.call(CONTACT_VALUE_A, &args![contact + 0x1c]).f32();
    let inner = e.call(POINTER_GET, &args![contact]).u32();
    let value_b = e.call(CONTACT_VALUE_B, &args![inner]).f32();
    let damage = e.call(IMPACT_DAMAGE, &args![value_b, value_a]).f32();
    if damage > 0.0 {
        let block = e.call(OPERATOR_NEW, &args![0x64u32]).u32();
        let hit_data = if block != 0 {
            e.call(HIT_DATA_CONSTRUCTOR, &args![block]).u32()
        } else {
            0
        };
        e.with_stack(4, |e, slot| {
            e.call(HIT_DATA_POINTER_CONSTRUCTOR, &args![slot, hit_data]);
            let hit = e.call(POINTER_GET, &args![slot]).u32();
            e.call(
                INITIALIZE_IMPACT_DATA,
                &args![hit, reference, this, damage, contact],
            );
            let hit = e.call(POINTER_GET, &args![slot]).u32();
            e.call(ACTOR_HIT_ME, &args![this, hit, 0u32]);
            e.call(HIT_DATA_POINTER_DESTRUCTOR, &args![slot]);
        });
    }
}

// Translated from 008b9220 (decompiled, FalloutNV.exe 1.4.0.525)
/// Address of the contact list head inside a character controller
/// (`controller + 0x648`).
pub fn fn_008b9220(_e: &mut Engine, this: Ptr) -> Ptr {
    Ptr::new(this.addr().wrapping_add(0x648))
}

// Translated from 008b9240 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Actor::ConditionModifiedCallback` (Xbox PDB), `cdecl`: called when the
/// condition actor value `index` of the `ActorValueOwner` `owner` changed
/// by `change` from `previous`; `source` is the `ActorValueOwner` of whoever
/// caused it. Resets the surgery limb for the player, shows the crippled
/// limb HUD effect for an actor passing virtual `+0x360`, reports a limb
/// crippled by the player, queues the replacement locomotion for the legs
/// (index `0x48` is the combined mobility value), flags the cached values
/// stale and refreshes them after a limb change.
pub fn actor_condition_modified_callback(
    e: &mut Engine,
    owner: Ptr,
    index: i32,
    previous: f32,
    change: f32,
    source: Ptr,
) {
    if owner.addr() == 0 {
        return;
    }
    let actor = actor_of_owner(owner.addr());
    let source_actor = actor_of_owner(source.addr());
    if actor == 0 {
        return;
    }
    let value = e
        .vcall(owner.addr(), OWNER_VSLOT_VALUE, &args![index])
        .f32();
    let player = e.global::<u32>(PLAYER_POINTER);
    if e.vcall(actor, VSLOT_ACTOR_TEST_360, &args![]).bool()
        && (0x19..=0x1e).contains(&index)
        && actor == player
    {
        e.call(SURGERY_MENU_RESET_LIMB, &args![index]);
    }
    let crippled = value <= 0.0 && previous > 0.0;
    if e.vcall(actor, VSLOT_ACTOR_TEST_360, &args![]).bool() && change < 0.0 && index != 0x48 {
        if crippled {
            let hud = e.global::<u32>(HUD_MAIN_MENU_POINTER);
            e.call(SHOW_CRIPPLED_GUY, &args![hud]);
            e.call(TRIGGER_PAIN, &args![actor, 1u32, 0u32]);
            e.call(MISC_STAT_INCREMENT, &args![0x1eu32]);
        }
    } else if change < 0.0 && ((0x19..=0x1e).contains(&index) || index == 0x1f) {
        if index == 0x1f && crippled {
            let full = e.global::<f32>(FULL_VALUE);
            e.vcall(actor, VSLOT_ACTOR_0X394, &args![0u32, full]);
        }
        let player = e.global::<u32>(PLAYER_POINTER);
        if crippled
            && actor != player
            && source_actor == player
            && !e.vcall(actor, 0x22c, &args![0u32]).bool()
            && e.global::<u32>(HUD_MAIN_MENU_POINTER) != 0
        {
            show_limb_message(e, actor, index);
        }
    }
    if index == 0x48 {
        let actor_owner = owner_of_actor(actor);
        let left = e
            .call(GET_BODY_PART_CONDITION, &args![actor_owner, 0x1du32, 0u32])
            .f32();
        let right = e
            .call(GET_BODY_PART_CONDITION, &args![actor_owner, 0x1eu32, 0u32])
            .f32();
        if crippled {
            if left <= 0.0 {
                actor_queue_replacement_locomotion(e, Ptr::new(actor), 1.0, 0.0, 1.0);
            }
            if right <= 0.0 {
                actor_queue_replacement_locomotion(e, Ptr::new(actor), 1.0, 0.0, 1.0);
            }
        } else if previous == 0.0 && change != 0.0 {
            if left <= 0.0 {
                actor_queue_replacement_locomotion(e, Ptr::new(actor), 0.0, 1.0, 1.0);
            }
            if right <= 0.0 {
                actor_queue_replacement_locomotion(e, Ptr::new(actor), 0.0, 1.0, 1.0);
            }
        }
    }
    if index == 0x19 && change != 0.0 {
        forward_cached_flags(e, actor, 0x100);
    }
    if index == 0x1d || index == 0x1e || index == 0x48 {
        e.call(REFRESH_AFTER_LIMB_CHANGE, &args![actor]);
    }
}

/// The message `008b9240` shows when the player cripples a limb of another
/// actor: `"<actor name> <limb text>"` with the happy Vault Boy icon.
fn show_limb_message(e: &mut Engine, actor: u32, index: i32) {
    e.with_stack(200, |e, limb_text| {
        e.with_stack(500, |e, message| {
            e.call(COPY_STRING, &args![limb_text, EMPTY_STRING]);
            let base = e.call(GET_BASE_FORM, &args![actor]).u32();
            let body_part_data = e.vcall(base, VSLOT_BASE_BODY_PART_DATA, &args![]).u32();
            let mut part_name = 0u32;
            if body_part_data != 0 {
                let part = e.call(GET_BODY_PART, &args![body_part_data, index]).u32();
                if part != 0 {
                    part_name = e.call(BODY_PART_NAME, &args![part]).u32();
                }
            }
            e.call(BUILD_LIMB_TEXT, &args![index, part_name, limb_text, 200u32]);
            let name = e.call(ACTOR_NAME, &args![actor]).u32();
            e.call(
                FORMAT_STRING,
                &args![message, 500u32, FORMAT_TWO_STRINGS, name, limb_text],
            );
            let time = e.global::<f32>(MESSAGE_TIME);
            let hud = e.global::<u32>(HUD_MAIN_MENU_POINTER);
            e.call(
                SHOW_MESSAGE,
                &args![
                    hud,
                    message,
                    1u32,
                    ICON_VERY_HAPPY,
                    SOUND_POPUP_GENERAL,
                    time,
                    0u32
                ],
            );
        });
    });
}

// Translated from 008b9790 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CachedValuesOwner` virtual (`this` = `Actor + 0xa8`): reads actor value
/// `0x19` from the `ActorValueOwner` and, when the process has a
/// cached-values block, stores it as the cached perception condition
/// (`008b97f0`). Returns the value.
pub fn fn_008b9790(e: &mut Engine, this: Ptr) -> f32 {
    let owner = this.addr().wrapping_sub(4);
    let value = e.vcall(owner, OWNER_VSLOT_VALUE, &args![0x19u32]).f32();
    store_cached_value(e, this.addr(), value, fn_008b97f0);
    value
}

// Translated from 008b97f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores `value` as `CachedValues::fCachedPerceptionCondition` (Xbox PDB,
/// `+0x28`) with flag mask `0x100`, when the process has the block.
pub fn fn_008b97f0(e: &mut Engine, this: Ptr, value: f32) {
    set_cached_value(e, this, 0x28, value, 0x100);
}

// Translated from 008b9830 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Actor::MobilityConditionModifiedCallback` (Xbox PDB), `cdecl`: runs the
/// general condition callback (`008b9240`); for the leg condition values
/// `0x1d` and `0x1e`, unless the owner's virtual `+0x08` answers non-zero
/// for `0x48`, queues the replacement locomotion from the sign of
/// `previous` and the leg conditions.
pub fn actor_mobility_condition_modified_callback(
    e: &mut Engine,
    owner: Ptr,
    index: i32,
    previous: f32,
    change: f32,
    source: Ptr,
) {
    if owner.addr() == 0 {
        return;
    }
    actor_condition_modified_callback(e, owner, index, previous, change, source);
    if index != 0x1d && index != 0x1e {
        return;
    }
    if e.vcall(owner.addr(), OWNER_VSLOT_0X08, &args![0x48u32])
        .u32()
        != 0
    {
        return;
    }
    let actor = actor_of_owner(owner.addr());
    let flag: f32 = if previous > 0.0 || previous.is_nan() {
        1.0
    } else {
        0.0
    };
    let other_index: u32 = if index == 0x1d { 0x1e } else { 0x1d };
    let other = e
        .call(
            GET_BODY_PART_CONDITION,
            &args![owner_of_actor(actor), other_index, 0u32],
        )
        .f32();
    e.call(REFRESH_BEFORE_QUEUE, &args![actor]);
    let current = e
        .call(
            GET_BODY_PART_CONDITION,
            &args![owner_of_actor(actor), index, 0u32],
        )
        .f32();
    actor_queue_replacement_locomotion(e, Ptr::new(actor), flag, current, other);
}

// Translated from 008b9960 (decompiled, FalloutNV.exe 1.4.0.525)
/// `cdecl` callback for actor values `0x10` and `0x16`: when the actor has a
/// process, compares the owner's virtual `+0x20` value with `previous +
/// change`; a drop below it (negative `change`, value above the sum) sets
/// the process flag `0x20000000` (index `0x10`) or `0x40000000` (index
/// `0x16`), and a rise (positive `change`, value at or below the sum)
/// clears it (`008acf70`).
pub fn fn_008b9960(e: &mut Engine, owner: Ptr, index: i32, previous: f32, change: f32) {
    if owner.addr() == 0 || (index != 0x10 && index != 0x16) {
        return;
    }
    let actor = actor_of_owner(owner.addr());
    if process_of(e, actor) == 0 {
        return;
    }
    let sum = previous + change;
    let current = e.vcall(owner.addr(), OWNER_VSLOT_0X20, &args![index]).f32();
    let drop = change < 0.0 && current > sum;
    let rise = change > 0.0 && current <= sum;
    let mask = if index == 0x10 {
        0x2000_0000u32
    } else {
        0x4000_0000
    };
    if drop {
        let process = process_of(e, actor);
        e.call(PROCESS_SET_STATE_FLAG, &args![process, mask, 1u32]);
    } else if rise {
        let process = process_of(e, actor);
        e.call(PROCESS_SET_STATE_FLAG, &args![process, mask, 0u32]);
    }
}

// Translated from 008b9ab0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `cdecl` callback: asks the actor's process to recompute the cached
/// values flagged `0x10`.
pub fn fn_008b9ab0(e: &mut Engine, owner: Ptr) {
    if owner.addr() == 0 {
        return;
    }
    forward_cached_flags(e, actor_of_owner(owner.addr()), 0x10);
}

// Translated from 008b9b10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `cdecl` callback: asks the actor's process to recompute the cached
/// values flagged `0x80000000`.
pub fn fn_008b9b10(e: &mut Engine, owner: Ptr) {
    if owner.addr() == 0 {
        return;
    }
    forward_cached_flags(e, actor_of_owner(owner.addr()), 0x8000_0000);
}

// Translated from 008b9b70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CachedValuesOwner` virtual (`this` = `Actor + 0xa8`): computes the
/// medicine effectiveness value (`00649530` of the `ActorValueOwner`) and
/// stores it (`008b9be0`) when the process has a cached-values block.
/// Returns the value.
pub fn fn_008b9b70(e: &mut Engine, this: Ptr) -> f32 {
    // The compiler's null guard: `this - 0xa8 == 0` gives null.
    let owner = if this.addr().wrapping_sub(0xa8) == 0 {
        0
    } else {
        this.addr().wrapping_sub(4)
    };
    let value = e.call(CACHED_VALUE_MEDICINE, &args![owner]).f32();
    store_cached_value(e, this.addr(), value, fn_008b9be0);
    value
}

// Translated from 008b9be0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores `value` as `CachedValues::fCachedMedicineEffectivenessMult`
/// (Xbox PDB, `+0x14`) with flag mask `0x10`, when the process has the
/// block.
pub fn fn_008b9be0(e: &mut Engine, this: Ptr, value: f32) {
    set_cached_value(e, this, 0x14, value, 0x10);
}

// Translated from 008b9c10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CachedValuesOwner` virtual (`this` = `Actor + 0xa8`): computes the
/// survival effectiveness value (`00649580`) and stores it (`008b9c80`).
/// Returns the value.
pub fn fn_008b9c10(e: &mut Engine, this: Ptr) -> f32 {
    let owner = if this.addr().wrapping_sub(0xa8) == 0 {
        0
    } else {
        this.addr().wrapping_sub(4)
    };
    let value = e.call(CACHED_VALUE_SURVIVAL, &args![owner]).f32();
    store_cached_value(e, this.addr(), value, fn_008b9c80);
    value
}

// Translated from 008b9c80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores `value` as `CachedValues::fCachedSurvivalEffectivenessMult`
/// (Xbox PDB, `+0x18`) with flag mask `0x80000000`.
pub fn fn_008b9c80(e: &mut Engine, this: Ptr, value: f32) {
    set_cached_value(e, this, 0x18, value, 0x8000_0000);
}

// Translated from 008b9cc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `cdecl` callback: asks the actor's process to recompute the cached
/// values flagged `0x20`.
pub fn fn_008b9cc0(e: &mut Engine, owner: Ptr) {
    if owner.addr() == 0 {
        return;
    }
    forward_cached_flags(e, actor_of_owner(owner.addr()), 0x20);
}

// Translated from 008b9d20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CachedValuesOwner` virtual (`this` = `Actor + 0xa8`): reads actor value
/// `0x2f` and stores it (`008b9d80`). Returns the value.
pub fn fn_008b9d20(e: &mut Engine, this: Ptr) -> f32 {
    let owner = this.addr().wrapping_sub(4);
    let value = e.vcall(owner, OWNER_VSLOT_VALUE, &args![0x2fu32]).f32();
    store_cached_value(e, this.addr(), value, fn_008b9d80);
    value
}

// Translated from 008b9d80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores `value` as `CachedValues::fCachedParalysis` (Xbox PDB, `+0x1c`)
/// with flag mask `0x20`.
pub fn fn_008b9d80(e: &mut Engine, this: Ptr, value: f32) {
    set_cached_value(e, this, 0x1c, value, 0x20);
}

// Translated from 008b9db0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `cdecl` callback: asks the actor's process to recompute the cached
/// values flagged `0x40`.
pub fn fn_008b9db0(e: &mut Engine, owner: Ptr) {
    if owner.addr() == 0 {
        return;
    }
    forward_cached_flags(e, actor_of_owner(owner.addr()), 0x40);
}

// Translated from 008b9e10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CachedValuesOwner` virtual (`this` = `Actor + 0xa8`): reads actor value
/// `0x0f` and stores it as the healing rate (`008b9e70`). Returns the
/// value.
pub fn fn_008b9e10(e: &mut Engine, this: Ptr) -> f32 {
    let owner = this.addr().wrapping_sub(4);
    let value = e.vcall(owner, OWNER_VSLOT_VALUE, &args![0xfu32]).f32();
    store_cached_value(e, this.addr(), value, fn_008b9e70);
    value
}

// Translated from 008b9e70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores `value` as `CachedValues::fCachedHealingRate` (Xbox PDB, `+0x20`)
/// with flag mask `0x40`.
pub fn fn_008b9e70(e: &mut Engine, this: Ptr, value: f32) {
    set_cached_value(e, this, 0x20, value, 0x40);
}

// Translated from 008b9ea0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Actor::EnduranceModifiedCallback` (Xbox PDB), `cdecl`: asks the
/// process to recompute the cached values flagged `0x80`; when `change` is
/// negative and the owner's virtual `+0x14` for actor value `0x10` is
/// negative, sums that value, the base health, the owner's virtuals `+0x18`
/// and `+0x10` for actor value `0x10` and the adjustment `006436c0` of
/// `previous + change`, and restores actor value `0x10` by `1 - sum` when
/// the sum is negative.
pub fn actor_endurance_modified_callback(
    e: &mut Engine,
    owner: Ptr,
    _unused_1: u32,
    previous: f32,
    change: f32,
) {
    if owner.addr() == 0 {
        return;
    }
    let actor = actor_of_owner(owner.addr());
    forward_cached_flags(e, actor, 0x80);
    if change >= 0.0 || change.is_nan() {
        return;
    }
    let actor_owner = owner_of_actor(actor);
    let value = e
        .vcall(actor_owner, OWNER_VSLOT_0X14, &args![0x10u32])
        .f32();
    if value >= 0.0 || value.is_nan() {
        return;
    }
    let sum = previous + change;
    let adjustment = e.call(ENDURANCE_ADJUSTMENT, &args![owner, sum]).f32();
    let base = e.call(GET_BASE_FORM, &args![actor]).u32();
    let health = e.call(GET_BASE_HEALTH, &args![base]).i32() as f64;
    let second = e
        .vcall(actor_owner, OWNER_VSLOT_0X18, &args![0x10u32])
        .f32() as f64;
    let first = e
        .vcall(actor_owner, OWNER_VSLOT_0X10, &args![0x10u32])
        .f32() as f64;
    // x87 order: the two partial sums are spilled as doubles, the total is
    // stored as a float.
    let total = ((first + (second + health)) + value as f64 + adjustment as f64) as f32;
    if total < 0.0 {
        let one = e.global::<f64>(DOUBLE_ONE);
        let amount = (-(total as f64) + one) as f32;
        e.call(RESTORE_ACTOR_VALUE, &args![actor, 0x10u32, amount]);
    }
}

// Translated from 008b9ff0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CachedValuesOwner` virtual (`this` = `Actor + 0xa8`): reads actor value
/// `0x0f` and stores it as the cached endurance (`008ba050`). Returns the
/// value.
pub fn fn_008b9ff0(e: &mut Engine, this: Ptr) -> f32 {
    let owner = this.addr().wrapping_sub(4);
    let value = e.vcall(owner, OWNER_VSLOT_VALUE, &args![0xfu32]).f32();
    store_cached_value(e, this.addr(), value, fn_008ba050);
    value
}

// Translated from 008ba050 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores `value` as `CachedValues::fCachedEndurance` (Xbox PDB, `+0x24`)
/// with flag mask `0x80`.
pub fn fn_008ba050(e: &mut Engine, this: Ptr, value: f32) {
    set_cached_value(e, this, 0x24, value, 0x80);
}

// Translated from 008ba090 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Actor::QueueReplacementLocomotion` (Xbox PDB), `cdecl`: with
/// `delta = b - a` and `sum = a + c`, a negative `delta` with `sum == 2.0`
/// and `b == 0` asks `008b7360(actor, 1, 0)`; a positive `delta` with `sum
/// < 2.0`, `b != 0` and `c > 0` asks `008b7360(actor, 0, 0)`.
pub fn actor_queue_replacement_locomotion(
    e: &mut Engine,
    this: Ptr<Actor>,
    a: f32,
    b: f32,
    c: f32,
) {
    let delta = b - a;
    let sum = (a + c) as f64;
    let two = e.global::<f64>(DOUBLE_TWO);
    if delta < 0.0 {
        if sum == two && b == 0.0 {
            e.call(REQUEST_LOCOMOTION_CHANGE, &args![this, 1u32, 0u32]);
        }
    } else if delta > 0.0 && sum < two && b != 0.0 && c > 0.0 {
        e.call(REQUEST_LOCOMOTION_CHANGE, &args![this, 0u32, 0u32]);
    }
}

// Translated from 008ba140 (decompiled, FalloutNV.exe 1.4.0.525)
/// `cdecl` callback: when the actor has a process, a combat controller
/// (virtual `+0x428`) gets `008a0330`/`005c1ae0` applied and the process's
/// virtuals `+0x68` and `+0x6c` are called with the actor; then asks the
/// process to recompute the cached values flagged `0x400`.
pub fn fn_008ba140(e: &mut Engine, owner: Ptr) {
    if owner.addr() == 0 {
        return;
    }
    let actor = actor_of_owner(owner.addr());
    if e.call(ACTOR_PROCESS, &args![actor]).u32() != 0 {
        if e.vcall(actor, VSLOT_GET_COMBAT_CONTROLLER, &args![]).u32() != 0 {
            let state = e.call(ACTOR_COMBAT_CONTROLLER_STATE, &args![actor]).u32();
            e.call(COMBAT_CONTROLLER_APPLY, &args![state]);
        }
        let process = e.call(ACTOR_PROCESS, &args![actor]).u32();
        e.vcall(process, 0x68, &args![actor]);
        let process = e.call(ACTOR_PROCESS, &args![actor]).u32();
        e.vcall(process, 0x6c, &args![actor]);
    }
    forward_cached_flags(e, actor, 0x400);
}

// Translated from 008ba210 (decompiled, FalloutNV.exe 1.4.0.525)
/// `cdecl` callback: asks the actor's process to recompute the cached
/// values flagged `0x800`.
pub fn fn_008ba210(e: &mut Engine, owner: Ptr) {
    if owner.addr() == 0 {
        return;
    }
    forward_cached_flags(e, actor_of_owner(owner.addr()), 0x800);
}

// Translated from 008ba270 (decompiled, FalloutNV.exe 1.4.0.525)
/// `cdecl` callback: calls the actor's virtual `+0x3c4`.
pub fn fn_008ba270(e: &mut Engine, owner: Ptr) {
    if owner.addr() == 0 {
        return;
    }
    let actor = actor_of_owner(owner.addr());
    e.vcall(actor, 0x3c4, &args![]);
}

// Translated from 008ba2b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `cdecl` callback: calls `008c4400` on the actor.
pub fn fn_008ba2b0(e: &mut Engine, owner: Ptr) {
    if owner.addr() == 0 {
        return;
    }
    let actor = actor_of_owner(owner.addr());
    e.call(ACTOR_CALLBACK_008C4400, &args![actor]);
}

// Translated from 008ba2f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `cdecl` callback (the second word is never read): when the actor's
/// virtual `+0x360` answers true, the player's virtual `+0x358` is asked and
/// the player is checked for god mode (`009526b0`) and for a value
/// (`008a0c20`) below `threshold`; when the player answered true and is
/// neither in god mode nor below the threshold, the "surprised" Vault Boy
/// message is shown (`004c69f0`, `007052f0`).
pub fn fn_008ba2f0(e: &mut Engine, owner: Ptr, _unused_1: u32, threshold: f32) {
    if owner.addr() == 0 {
        return;
    }
    let actor = actor_of_owner(owner.addr());
    if !e.vcall(actor, VSLOT_ACTOR_TEST_360, &args![]).bool() {
        return;
    }
    let player = e.global::<u32>(PLAYER_POINTER);
    let answered = e.vcall(player, 0x358, &args![]).bool();
    let below = if e.call(PLAYER_IS_GOD_MODE, &args![]).bool() {
        false
    } else {
        let value = e.call(PLAYER_VALUE_008A0C20, &args![player]).f32();
        threshold > value
    };
    if answered && !below {
        let time = e.global::<f32>(MESSAGE_TIME);
        // 008ba3ac: `004c69f0` takes only ECX; the five pushed words are the
        // rest of the cdecl `007052f0` (ADD ESP,0x18).
        let message = e.call(MESSAGE_QUEUE_ADD, &args![MESSAGE_QUEUE]).u32();
        e.call(
            MESSAGE_QUEUE_SHOW,
            &args![message, 0u32, ICON_SURPRISED, 0u32, time, 0u32],
        );
    }
}

// Translated from 008ba3e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The process's virtual `+0x6f4` (a flag test), or false without a
/// process.
pub fn fn_008ba3e0(e: &mut Engine, this: Ptr<Actor>) -> bool {
    process_flag_test(e, this, 0x6f4)
}

// Translated from 008ba410 (decompiled, FalloutNV.exe 1.4.0.525)
/// The process's virtual `+0x6ec` (a flag test), or false without a
/// process.
pub fn fn_008ba410(e: &mut Engine, this: Ptr<Actor>) -> bool {
    process_flag_test(e, this, 0x6ec)
}

// Translated from 008ba440 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Actor::IsHeavyBodyArmorWorn` (Xbox PDB): the process's virtual `+0x6e8`,
/// or false without a process.
pub fn actor_is_heavy_body_armor_worn(e: &mut Engine, this: Ptr<Actor>) -> bool {
    process_flag_test(e, this, 0x6e8)
}

// Translated from 008ba470 (decompiled, FalloutNV.exe 1.4.0.525)
/// The process's virtual `+0x6f0` (a flag test), or false without a
/// process.
pub fn fn_008ba470(e: &mut Engine, this: Ptr<Actor>) -> bool {
    process_flag_test(e, this, 0x6f0)
}

/// The four getters above: false without a process, else the process's
/// virtual `slot` (a `bool` in `AL`).
fn process_flag_test(e: &mut Engine, this: Ptr<Actor>, slot: u32) -> bool {
    let process = process_of(e, this.addr());
    if process == 0 {
        return false;
    }
    e.vcall(process, slot, &args![]).bool()
}

/// The pending-animation bits `IsAnimIdleQuequed` tests: `0x4`, `0x8`,
/// `0x10`, `0x40`, `0x80`, `0x100`, `0x400`, `0x800`, `0x1000`, `0x2000`.
const QUEUED_IDLE_MASK: u32 =
    0x4 | 0x8 | 0x10 | 0x40 | 0x80 | 0x100 | 0x400 | 0x800 | 0x1000 | 0x2000;

// Translated from 008ba4a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Actor::IsAnimIdleQuequed` (Xbox PDB): true when the process's pending
/// animation flags (virtual `+0x618`) include any bit of
/// [`QUEUED_IDLE_MASK`]. The process is not tested for null.
pub fn actor_is_anim_idle_quequed(e: &mut Engine, this: Ptr<Actor>) -> bool {
    let process = process_of(e, this.addr());
    let flags = e.vcall(process, 0x618, &args![]).u32();
    flags & QUEUED_IDLE_MASK != 0
}

/// `Animation::SpecialIdlePlaying` (Xbox PDB): `ECX` = animation.
const ANIMATION_SPECIAL_IDLE_PLAYING: u32 = 0x0049_85b0;
/// `ECX` = an anim group object: returns its type at `+0x14`.
const ANIM_GROUP_TYPE: u32 = 0x0082_5c00;

// Translated from 008ba530 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Actor::IsPlayingLowerBodySpecialIdle` (Xbox PDB): with an animation
/// (actor virtual `+0x1e4`), takes the type of the group in the
/// animation's `+0x128` slot when an idle is queued, else of the one in its
/// `+0x124` slot when a special idle is playing; true for types `1`, `7`
/// and `0x14`.
pub fn actor_is_playing_lower_body_special_idle(e: &mut Engine, this: Ptr<Actor>) -> bool {
    let animation = e.vcall(this.addr(), 0x1e4, &args![]).u32();
    if animation == 0 {
        return false;
    }
    let mut group_type: i32 = -1;
    if actor_is_anim_idle_quequed(e, this)
        && e.call(POINTER_GET, &args![animation + 0x128]).u32() != 0
    {
        let group = e.call(POINTER_GET, &args![animation + 0x128]).u32();
        group_type = e.call(ANIM_GROUP_TYPE, &args![group]).i32();
    } else if e
        .call(ANIMATION_SPECIAL_IDLE_PLAYING, &args![animation])
        .bool()
        && e.call(POINTER_GET, &args![animation + 0x124]).u32() != 0
    {
        let group = e.call(POINTER_GET, &args![animation + 0x124]).u32();
        group_type = e.call(ANIM_GROUP_TYPE, &args![group]).i32();
    }
    group_type == 1 || group_type == 7 || group_type == 0x14
}

/// `Actor::GetAnimGroup` (Xbox PDB): `ECX` = actor, four words, returns the
/// group number in `AX`.
const GET_ANIM_GROUP: u32 = 0x0089_7910;
/// `Animation::PlayGroup` (Xbox PDB): `ECX` = animation, four words (the
/// group number in the low word, then three more).
const PLAY_GROUP: u32 = 0x0049_4740;
/// `Actor::StartAttack` (Xbox PDB): `ECX` = actor, the queued attack.
const START_ATTACK: u32 = 0x0089_3a40;
/// `Animation::SpecialIdleWorking_ov3` (Xbox PDB): `ECX` = animation.
const SPECIAL_IDLE_WORKING: u32 = 0x0049_8f80;
/// `ECX` = actor (`008b0ac0`), called when the pending flag `0x1000` is set.
const ACTOR_HANDLE_FLAG_1000: u32 = 0x008b_0ac0;
/// `ECX` = actor (`008a7a90`), called when the pending flag `0x400` is set.
const ACTOR_HANDLE_FLAG_400: u32 = 0x008a_7a90;
/// `ECX` = actor (`00894940`), called when the pending flag `0x8000` is set.
const ACTOR_HANDLE_FLAG_8000: u32 = 0x0089_4940;
/// `PlayerCharacter::GetAnimation` (Xbox PDB): `ECX` = player, one word.
const PLAYER_GET_ANIMATION: u32 = 0x0095_0a60;
/// Named `Animation::ZeroGlobalTransform` in the engine map (folded body):
/// `ECX` = the object process virtual `+0x3e8` returned.
const ANIMATION_STEP: u32 = 0x0048_f7f0;
/// `ECX` = the object `0048f7f0` returned: a kind number (index into the
/// table at `011977e4`, entries `0x24` bytes).
const ANIMATION_KIND: u32 = 0x005f_2420;
/// Table indexed by [`ANIMATION_KIND`] (entries of `0x24` bytes).
const ANIMATION_KIND_TABLE: u32 = 0x0119_77e4;
/// `ECX` = actor: the actor's flag word (low 16 bits used).
const GET_FLAG_WORD: u32 = 0x0088_46e0;
/// `ItemChange::HasModEffectActive_ov2` (Xbox PDB): `ECX` = item entry, the
/// effect number; `bool` in `AL`.
const HAS_MOD_EFFECT_ACTIVE: u32 = 0x004b_da70;
/// `TESObjectWEAP::GetFormClipRounds` (Xbox PDB): `ECX` = the weapon form,
/// one word.
const GET_FORM_CLIP_ROUNDS: u32 = 0x004f_e160;
/// `InventoryChanges::GetInventoryChanges` (Xbox PDB), `cdecl`: the actor.
const GET_INVENTORY_CHANGES: u32 = 0x004b_f220;
/// `InventoryChanges::GetObjectCount` (Xbox PDB): `ECX` = inventory changes,
/// one word (the object).
const GET_OBJECT_COUNT: u32 = 0x004c_8f30;
/// `TESAnimGroup::AnimGroup` (Xbox PDB), `cdecl`, four words: packs a group
/// number into `AX`.
const PACK_ANIM_GROUP: u32 = 0x005f_2370;
/// `Animation::PickBestAnimation` (Xbox PDB): `ECX` = animation, two words.
const PICK_BEST_ANIMATION: u32 = 0x0049_5740;
/// `ECX` = weapon form, one word: the animation type of the weapon.
const WEAPON_ANIM_TYPE: u32 = 0x0051_e2a0;
/// `TESAnimGroup::GetType` (Xbox PDB), `cdecl`: the group number.
const ANIM_GROUP_GET_TYPE: u32 = 0x005f_2440;
/// `Actor::SetAnimAction` (Xbox PDB): `ECX` = actor, an action and an
/// animation slot.
const SET_ANIM_ACTION: u32 = 0x008a_73e0;
/// `ECX` = animation, one word (a slot number): the slot's entry.
const ANIMATION_SLOT: u32 = 0x0049_1040;
/// `LowProcess::GetGenericLocation` (Xbox PDB): `ECX` = the slot's entry.
const GET_GENERIC_LOCATION: u32 = 0x0080_41a0;
/// `Actor::GetOutofFurnitureQuick` (Xbox PDB): `ECX` = actor.
const GET_OUT_OF_FURNITURE_QUICK: u32 = 0x0088_d640;
/// `TESIdleForm::GetIdleAnimGroupSection` (Xbox PDB): `ECX` = the idle,
/// one word.
const GET_IDLE_ANIM_GROUP_SECTION: u32 = 0x005f_f160;
/// `ECX` = animation: plays an idle (the idle, the actor, the section).
const PLAY_IDLE_SECTION: u32 = 0x0049_7f20;
/// `NiPointer` constructor from a raw pointer (`ECX` = slot, one word).
const IDLE_POINTER_CONSTRUCTOR: u32 = 0x0063_3c90;
/// `NiPointer` destructor (`ECX` = slot).
const IDLE_POINTER_DESTRUCTOR: u32 = 0x0045_cec0;
/// `AnimIdle::Loaded` (Xbox PDB): `ECX` = the idle, one word.
const ANIM_IDLE_LOADED: u32 = 0x0049_6cd0;
/// `Actor::GetCurrentWeapon` (Xbox PDB): `ECX` = actor.
const GET_CURRENT_WEAPON: u32 = 0x008a_1710;
/// `ECX` = the current weapon: its anim type.
const CURRENT_WEAPON_ANIM_TYPE: u32 = 0x0051_f610;
/// `Actor::SetHavokWeapon` (Xbox PDB): `ECX` = actor.
const SET_HAVOK_WEAPON: u32 = 0x008a_5eb0;
/// `TESObjectWEAP::ExpelShellCasing` (Xbox PDB): `ECX` = the weapon form,
/// one word (the actor).
const EXPEL_SHELL_CASING: u32 = 0x0052_4db0;
/// `Actor::SetIronSights` (Xbox PDB): `ECX` = actor, three words (the sights flag, `force`, `skip_if_kind_one`).
const SET_IRON_SIGHTS: u32 = 0x008b_b650;
/// `ECX` = weapon form: the test that splits the `0x1` block (the decompiler
/// names no function).
const WEAPON_FORM_TEST: u32 = 0x004c_0c30;
/// `ECX` = weapon form, one word (the actor).
const WEAPON_FORM_PREPARE: u32 = 0x0052_3150;
/// `ECX` = weapon form, two flag words: returns a word for `008bc240`.
const WEAPON_FORM_STATE: u32 = 0x0052_5620;
/// `ECX` = actor, one word (`008bc240`).
const ACTOR_APPLY_WEAPON_STATE: u32 = 0x008b_c240;
/// `ECX` = actor: tells whether the actor is in the state that makes the `0x1`
/// block take its first branch.
const ACTOR_STATE_TEST_493BB0: u32 = 0x0049_3bb0;
/// `ECX` = actor, two words (`00899200`).
const ACTOR_UPDATE_00899200: u32 = 0x0089_9200;
/// `ECX` = the object process virtual `+0x27c` returned: a type number.
const OBJECT_TYPE: u32 = 0x0041_ca90;
/// `Script::SetActionFlag` (Xbox PDB), `cdecl`: form, value, flag word.
const SET_ACTION_FLAG: u32 = 0x005a_c750;
/// Process virtual `+0x148`: the entry of the weapon in hand (its form is
/// read with `0044ddc0`).
const PROCESS_VSLOT_EQUIPPED: u32 = 0x148;
/// Process virtual `+0x14c`: the entry of the ammunition.
const PROCESS_VSLOT_AMMO: u32 = 0x14c;
/// Process virtual `+0x614`: takes a bit mask (the pending flag to clear).
const PROCESS_VSLOT_CLEAR_PENDING: u32 = 0x614;
/// Process virtual `+0x618`: the pending animation/equip flag word.
const PROCESS_VSLOT_PENDING_FLAGS: u32 = 0x618;
/// Process virtual `+0x708`: one word.
const PROCESS_VSLOT_0X708: u32 = 0x708;

/// The process of the actor, read afresh (the virtuals the handler calls may
/// change it), and its virtual `slot` called with `arguments`.
fn process_vcall(e: &mut Engine, actor: u32, slot: u32, arguments: &[u32]) -> Ret {
    let process = process_of(e, actor);
    e.vcall(process, slot, arguments)
}

// Translated from 008ba600 (decompiled, FalloutNV.exe 1.4.0.525)
/// Handles the pending animation/equip flags of the actor's process
/// (virtual `+0x618`), in order: `0x20` plays group `0xe0` and calls
/// process virtual `+0x294`; stops when actor virtual `+0x22c(0)` answers
/// true; `0x1000`; the queued attack (`StartAttack`, cleared to `0xff` when
/// the process says it is over); `0x800`; `0x400`; `0x8000`; `0x40000`
/// (equip or reload: picks and plays the animation group, queues the ammo
/// equip); `0x1` (weapon form setup); `0x2` (equip the weapon); `0x4` and
/// `0x8` (furniture); `0x200`; `0x80` (play an idle); `0x10`; `0x2000`
/// (loaded idles); `0x40`; `0x100`; `0x4000`; `0x20000` (shell casing);
/// `0x10000` (iron sights). C++ exception unwinding of the `NiPointer`
/// local is left out.
pub fn fn_008ba600(e: &mut Engine, this: Ptr<Actor>) {
    let actor = this.addr();
    if process_of(e, actor) == 0 {
        return;
    }
    let animation = e.vcall(actor, 0x1e4, &args![]).u32();
    let mut flags = process_vcall(e, actor, PROCESS_VSLOT_PENDING_FLAGS, &args![]).u32();
    process_vcall(e, actor, 0x61c, &args![]);
    if animation == 0 {
        return;
    }
    if flags & 0x20 != 0 {
        let group = e
            .call(GET_ANIM_GROUP, &args![actor, 0xe0u32, 0u32, 0u32, 0u32])
            .u16();
        e.call(
            PLAY_GROUP,
            &args![
                animation,
                group as u32,
                1u32,
                0xffff_ffffu32,
                0xffff_ffffu32
            ],
        );
        process_vcall(e, actor, 0x294, &args![actor]);
    }
    if e.vcall(actor, 0x22c, &args![0u32]).bool() {
        return;
    }
    if flags & 0x1000 != 0 {
        e.call(ACTOR_HANDLE_FLAG_1000, &args![actor]);
    }
    let equipped = process_vcall(e, actor, PROCESS_VSLOT_EQUIPPED, &args![]).u32();
    let equipped_form = if equipped != 0 {
        e.call(ENTRY_OBJECT, &args![equipped]).u32()
    } else {
        0
    };
    // Actor::eQueuedattack (Xbox PDB) +0x110
    let queued_attack = e.get(this, Actor::eQueuedattack);
    if queued_attack != 0xff {
        e.call(START_ATTACK, &args![actor, queued_attack]);
        let over = !process_vcall(e, actor, 0x3f0, &args![]).bool()
            || process_vcall(e, actor, 0x444, &args![]).f32() == 0.0;
        if over {
            process_vcall(e, actor, 0x3f4, &args![0u32]);
            e.set(this, Actor::eQueuedattack, 0xff);
        }
    }
    if flags & 0x800 != 0 {
        if process_of(e, actor) == 0
            || animation == 0
            || e.call(SPECIAL_IDLE_WORKING, &args![animation]).bool()
        {
            let process = e.call(ACTOR_PROCESS, &args![actor]).u32();
            e.vcall(process, PROCESS_VSLOT_CLEAR_PENDING, &args![0x800u32]);
        } else {
            process_vcall(e, actor, 0xd8, &args![actor]);
        }
    }
    if flags & 0x400 != 0 {
        e.call(ACTOR_HANDLE_FLAG_400, &args![actor]);
    }
    if flags & 0x8000 != 0 {
        e.call(ACTOR_HANDLE_FLAG_8000, &args![actor]);
    }
    if flags & 0x40000 != 0 {
        equip_or_reload(e, this, animation, equipped_form);
    }
    if flags & 0x1 != 0 {
        weapon_form_setup(e, this);
    }
    if flags & 0x2 != 0 && equipped_form != 0 && equipped != 0 {
        equip_weapon(e, this, animation, equipped, equipped_form);
    }
    if flags & 0x4 != 0 {
        if process_vcall(e, actor, 0x4d0, &args![]).u32() > 0xc8 {
            e.call(GET_OUT_OF_FURNITURE_QUICK, &args![actor]);
        } else if !process_vcall(e, actor, 0x2b0, &args![actor]).bool() {
            process_vcall(e, actor, PROCESS_VSLOT_0X708, &args![0u32]);
        }
        flags &= !0x10;
    }
    if flags & 0x8 != 0 {
        if !process_vcall(e, actor, 0x2b8, &args![actor]).bool() {
            process_vcall(e, actor, PROCESS_VSLOT_0X708, &args![0u32]);
        }
        flags &= !0x10;
    }
    if flags & 0x200 != 0 {
        e.call(GET_OUT_OF_FURNITURE_QUICK, &args![actor]);
    }
    if flags & 0x80 != 0 {
        let idle = process_vcall(e, actor, 0x718, &args![]).u32();
        if idle != 0 {
            let section = e.call(GET_IDLE_ANIM_GROUP_SECTION, &args![idle]).u32();
            // 008bb1d5: the pushed 3 is the last word of `00497f20` (RET 0x10);
            // `005ff160` takes no stack word.
            e.call(
                PLAY_IDLE_SECTION,
                &args![animation, idle, actor, section, 3u32],
            );
        }
    }
    if flags & 0x10 != 0 {
        process_vcall(e, actor, 0x70c, &args![actor]);
    }
    if flags & 0x2000 != 0 {
        while process_vcall(e, actor, 0x724, &args![]).u32() != 0 {
            let queued = process_vcall(e, actor, 0x724, &args![]).u32();
            e.with_stack(4, |e, slot| {
                e.call(IDLE_POINTER_CONSTRUCTOR, &args![slot, queued]);
                let idle = process_vcall(e, actor, 0x728, &args![]).u32();
                if e.call(POINTER_GET, &args![slot]).u32() != 0 && idle != 0 {
                    let held = e.call(POINTER_GET, &args![slot]).u32();
                    e.call(ANIM_IDLE_LOADED, &args![held, idle]);
                }
                process_vcall(e, actor, 0x720, &args![0u32, 0u32]);
                e.call(IDLE_POINTER_DESTRUCTOR, &args![slot]);
            });
        }
    }
    if flags & 0x40 != 0 {
        let flag = process_vcall(e, actor, 0x454, &args![]).bool();
        let not_equipped = u32::from(flag && equipped == 0);
        let slot_entry = if flag { equipped } else { 0xffff_ffff };
        let group = e
            .call(
                GET_ANIM_GROUP,
                &args![actor, 0u32, slot_entry, not_equipped, animation],
            )
            .u16();
        e.call(
            PLAY_GROUP,
            &args![
                animation,
                group as u32,
                1u32,
                0xffff_ffffu32,
                0xffff_ffffu32
            ],
        );
    }
    if flags & 0x100 != 0 && e.call(GET_CURRENT_WEAPON, &args![actor]).u32() != 0 {
        let weapon = e.call(GET_CURRENT_WEAPON, &args![actor]).u32();
        let weapon_type = e.call(CURRENT_WEAPON_ANIM_TYPE, &args![weapon]).u32();
        let entry = process_vcall(e, actor, PROCESS_VSLOT_EQUIPPED, &args![]).u32();
        let group = e
            .call(
                GET_ANIM_GROUP,
                &args![actor, weapon_type, entry, 0u32, animation],
            )
            .u16();
        e.call(
            PLAY_GROUP,
            &args![
                animation,
                group as u32,
                1u32,
                0xffff_ffffu32,
                0xffff_ffffu32
            ],
        );
    }
    if flags & 0x4000 != 0 {
        e.call(SET_HAVOK_WEAPON, &args![actor]);
    }
    if flags & 0x20000 != 0 {
        let entry = process_vcall(e, actor, PROCESS_VSLOT_EQUIPPED, &args![]).u32();
        let form = if entry != 0 {
            let entry = process_vcall(e, actor, PROCESS_VSLOT_EQUIPPED, &args![]).u32();
            e.call(ENTRY_OBJECT, &args![entry]).u32()
        } else {
            0
        };
        if form != 0 {
            e.call(EXPEL_SHELL_CASING, &args![form, actor]);
        }
    }
    if flags & 0x10000 != 0 {
        let sights = process_vcall(e, actor, 0x404, &args![]).u8();
        e.call(SET_IRON_SIGHTS, &args![actor, sights, 1u32, 0u32]);
    }
}

/// The `0x40000` block of `008ba600`: picks the equip or reload animation
/// group for the weapon and ammunition in hand, queues the equip of the
/// next ammunition entry and plays the group.
fn equip_or_reload(e: &mut Engine, this: Ptr<Actor>, animation: u32, equipped_form: u32) {
    let actor = this.addr();
    let player = e.global::<u32>(PLAYER_POINTER);
    let mut current_animation = e.vcall(actor, 0x1e4, &args![]).u32();
    if actor == player {
        current_animation = e.call(PLAYER_GET_ANIMATION, &args![player, 0u32]).u32();
    }
    let weapon_entry = process_vcall(e, actor, PROCESS_VSLOT_EQUIPPED, &args![]).u32();
    let ammo_entry = process_vcall(e, actor, PROCESS_VSLOT_AMMO, &args![]).u32();
    let mut ammo_index: i32 = 0;
    let mut clip_rounds: i32 = 0;
    let animation_object = process_vcall(e, actor, 0x3e8, &args![]).u32();
    let stepped = e.call(ANIMATION_STEP, &args![animation_object]).u32();
    let kind = e.call(ANIMATION_KIND, &args![stepped]).u32();
    // The entry of the kind table is read and never used afterwards.
    let _kind_entry = e
        .mem
        .u32(ANIMATION_KIND_TABLE.wrapping_add(kind.wrapping_mul(0x24)));
    let state = e.call(GET_FLAG_WORD, &args![actor]).u16();
    let mut slot_kind: u32 = 0;
    if state & 0x800 != 0 {
        slot_kind = 2;
    } else if state & 0x2000 != 0 {
        slot_kind = 3;
    } else if state & 0x400 != 0 {
        slot_kind = 1;
    }
    let mut group_index: u32 = 0xff;
    let table: [u32; 4] = if state & 0x200 != 0 {
        [7, 8, 9, 10]
    } else {
        [3, 4, 5, 6]
    };
    for (bit, value) in [1u16, 2, 4, 8].into_iter().zip(table) {
        if state & bit != 0 {
            group_index = value;
            break;
        }
    }
    if weapon_entry != 0 && equipped_form != 0 {
        let modded = e
            .call(HAS_MOD_EFFECT_ACTIVE, &args![weapon_entry, 2u32])
            .u8();
        clip_rounds = e
            .call(GET_FORM_CLIP_ROUNDS, &args![equipped_form, modded as u32])
            .i32();
    }
    let inventory = e.call(GET_INVENTORY_CHANGES, &args![actor]).u32();
    let mut ammo_count: i32 = 0;
    if inventory != 0 && ammo_entry != 0 {
        let ammo_form = e.call(ENTRY_OBJECT, &args![ammo_entry]).u32();
        ammo_count = e.call(GET_OBJECT_COUNT, &args![inventory, ammo_form]).i32();
    }
    if ammo_entry != 0 {
        ammo_index = e
            .call(LIST_NODE_NEXT, &args![ammo_entry])
            .i32()
            .wrapping_add(1);
    }
    if ammo_entry != 0 && ammo_index <= ammo_count {
        let ammo_form = e.call(ENTRY_OBJECT, &args![ammo_entry]).u32();
        e.call(
            QUEUE_EQUIP_OBJECT,
            &args![actor, ammo_form, ammo_index, 0u32, 1u32, 0u32, 0u32],
        );
    }
    if ammo_index < clip_rounds && ammo_count != ammo_index && ammo_count != 0 {
        // Reload: the group of the weapon's own animation type.
        let weapon_form = e.call(ENTRY_OBJECT, &args![weapon_entry]).u32();
        let weapon_type = e.call(WEAPON_ANIM_TYPE, &args![weapon_form, 0u32]).u32();
        let group = e
            .call(
                GET_ANIM_GROUP,
                &args![actor, weapon_type, weapon_entry, 0u32, current_animation],
            )
            .u16();
        let slot = e
            .call(ANIMATION_SLOT, &args![current_animation, 4u32])
            .u32();
        e.call(SET_ANIM_ACTION, &args![actor, 0x11u32, slot]);
        e.call(
            PLAY_GROUP,
            &args![current_animation, group as u32, 1u32, 1u32, 4u32],
        );
        if actor == player {
            e.vcall(player, 0x4b0, &args![group as u32, 1u32]);
        }
        return;
    }
    let first = fn_008ba3e0(e, this);
    let flag = first || fn_008ba410(e, this);
    let packed = e
        .call(
            PACK_ANIM_GROUP,
            &args![slot_kind, 0u32, group_index, u32::from(flag)],
        )
        .u16();
    let picked = e
        .call(
            PICK_BEST_ANIMATION,
            &args![current_animation, packed as u32, 0u32],
        )
        .u16();
    if picked == 0xff || current_animation == 0 {
        return;
    }
    e.call(
        PLAY_GROUP,
        &args![current_animation, picked as u32, 1u32, 1u32, 4u32],
    );
    if actor != player {
        let weapon_type = e.call(WEAPON_ANIM_TYPE, &args![equipped_form, 0u32]).u32();
        let group = e
            .call(GET_ANIM_GROUP, &args![actor, weapon_type, 0u32, 0u32, 0u32])
            .u16();
        let group_type = e.call(ANIM_GROUP_GET_TYPE, &args![group as u32]).u32();
        if group_type == weapon_type && animation != 0 {
            e.call(
                PLAY_GROUP,
                &args![
                    animation,
                    group as u32,
                    1u32,
                    0xffff_ffffu32,
                    0xffff_ffffu32
                ],
            );
            e.vcall(actor, 0x4b0, &args![group as u32, 1u32]);
            let slot = e.call(ANIMATION_SLOT, &args![animation, 4u32]).u32();
            e.call(SET_ANIM_ACTION, &args![actor, 9u32, slot]);
        }
    }
    if actor == player {
        e.vcall(player, 0x4b0, &args![picked as u32, 0u32]);
        e.call(SET_ANIM_ACTION, &args![player, 0xffff_ffffu32, 0u32]);
    }
}

/// The `0x1` block of `008ba600`: sets up the form of the weapon in hand.
fn weapon_form_setup(e: &mut Engine, this: Ptr<Actor>) {
    let actor = this.addr();
    let entry = process_vcall(e, actor, PROCESS_VSLOT_EQUIPPED, &args![]).u32();
    let form = if entry != 0 {
        let entry = process_vcall(e, actor, PROCESS_VSLOT_EQUIPPED, &args![]).u32();
        e.call(ENTRY_OBJECT, &args![entry]).u32()
    } else {
        0
    };
    if form == 0 {
        return;
    }
    if e.call(WEAPON_FORM_TEST, &args![form]).bool() {
        e.call(WEAPON_FORM_PREPARE, &args![form, actor]);
        let entry = process_vcall(e, actor, PROCESS_VSLOT_EQUIPPED, &args![]).u32();
        let mut first = 0u8;
        let mut second = 0u8;
        if entry != 0 {
            first = e.call(HAS_MOD_EFFECT_ACTIVE, &args![entry, 0xbu32]).u8();
            second = e.call(HAS_MOD_EFFECT_ACTIVE, &args![entry, 0x10u32]).u8();
        }
        let state = e
            .call(WEAPON_FORM_STATE, &args![form, first as u32, second as u32])
            .u32();
        e.call(ACTOR_APPLY_WEAPON_STATE, &args![actor, state]);
    } else {
        let player = e.global::<u32>(PLAYER_POINTER);
        if actor == player || e.call(ACTOR_STATE_TEST_493BB0, &args![actor]).bool() {
            e.call(ACTOR_UPDATE_00899200, &args![actor, 0u32, 1u32]);
        } else {
            let process = e.call(ACTOR_PROCESS, &args![actor]).u32();
            let object = e.vcall(process, 0x27c, &args![]).u32();
            if object != 0
                && (e.call(OBJECT_TYPE, &args![object]).u32() == 8
                    || e.call(OBJECT_TYPE, &args![object]).u32() == 0x10)
            {
                e.call(ACTOR_UPDATE_00899200, &args![actor, 0u32, 1u32]);
            } else {
                e.call(ACTOR_UPDATE_00899200, &args![actor, 0u32, 0u32]);
            }
        }
    }
    if e.call(LIST_NODE_NEXT, &args![form + 0x68]).u32() != 0 {
        let entry = process_vcall(e, actor, PROCESS_VSLOT_EQUIPPED, &args![]).u32();
        if entry != 0 {
            let entry = process_vcall(e, actor, PROCESS_VSLOT_EQUIPPED, &args![]).u32();
            if e.call(POINTER_GET, &args![entry]).u32() != 0 {
                let entry = process_vcall(e, actor, PROCESS_VSLOT_EQUIPPED, &args![]).u32();
                let held = e.call(POINTER_GET, &args![entry]).u32();
                let data = e.call(LIST_NODE_DATA_ADDRESS, &args![held]).u32();
                let value = e.mem.u32(data);
                let entry = process_vcall(e, actor, PROCESS_VSLOT_EQUIPPED, &args![]).u32();
                let target = e.call(ENTRY_OBJECT, &args![entry]).u32();
                e.call(SET_ACTION_FLAG, &args![target, value, 0x0040_0000u32]);
            }
        }
    }
}

/// The `0x2` block of `008ba600`: equips the weapon in hand, or clears the
/// pending flag `0x2` when one of the animation's slots `0`, `1` or `4` is
/// not in generic location `1`.
fn equip_weapon(e: &mut Engine, this: Ptr<Actor>, animation: u32, equipped: u32, form: u32) {
    let actor = this.addr();
    let mut all_generic = true;
    for slot in [0u32, 1, 4] {
        if e.call(ANIMATION_SLOT, &args![animation, slot]).u32() != 0 {
            let entry = e.call(ANIMATION_SLOT, &args![animation, slot]).u32();
            if e.call(GET_GENERIC_LOCATION, &args![entry]).u32() != 1 {
                all_generic = false;
                break;
            }
        }
    }
    if all_generic {
        let modded = e.call(HAS_MOD_EFFECT_ACTIVE, &args![equipped, 2u32]).u8();
        e.vcall(actor, 0x3ec, &args![form, 2u32, modded as u32, 0u32]);
    } else {
        let process = e.call(ACTOR_PROCESS, &args![actor]).u32();
        e.vcall(process, PROCESS_VSLOT_CLEAR_PENDING, &args![2u32]);
    }
}

// Translated from 008bb520 (decompiled, FalloutNV.exe 1.4.0.525)
/// Requests the actor mover to rotate the actor towards the vector
/// (`ActorMover::RequestRotateActor`), unless the actor cannot move
/// (`Actor::CanMove`; answers true) or its process has an entry with
/// `008bb5a0` set (answers true).
pub fn fn_008bb520(e: &mut Engine, this: Ptr<Actor>, x: f32, y: f32, z: f32, flag: bool) -> bool {
    if !e.call(ACTOR_CAN_MOVE, &args![this]).bool() {
        return true;
    }
    let process = e.call(ACTOR_PROCESS, &args![this]).u32();
    if process != 0
        && e.call(PROCESS_FLAG_0X28, &args![process]).u32() == 0
        && fn_008bb5a0(e, Ptr::new(process)) != 0
    {
        return true;
    }
    let mover = e.get(this, Actor::pActorMover);
    e.call(REQUEST_ROTATE_ACTOR, &args![mover, x, y, z, flag])
        .bool()
}

/// `Actor::CanMove` (Xbox PDB).
const ACTOR_CAN_MOVE: u32 = 0x0088_43a0;
/// `ECX` = process: returns its word at `+0x28`.
const PROCESS_FLAG_0X28: u32 = 0x0045_cd60;
/// `ActorMover::RequestRotateActor` (Xbox PDB): `ECX` = mover, a 12-byte
/// vector and a flag.
const REQUEST_ROTATE_ACTOR: u32 = 0x009d_ce20;
/// `ActorMover::RequestRotateActor_ov2` (Xbox PDB): `ECX` = mover, a
/// `float` and a flag.
const REQUEST_ROTATE_ACTOR_OV2: u32 = 0x009d_ce80;

// Translated from 008bb5a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Returns the process word at `+0x2ac`.
pub fn fn_008bb5a0(e: &mut Engine, this: Ptr) -> u32 {
    e.mem.u32(this.addr() + 0x2ac)
}

// Translated from 008bb5c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Like `008bb520` with an angle instead of a vector:
/// `ActorMover::RequestRotateActor_ov2`.
pub fn fn_008bb5c0(e: &mut Engine, this: Ptr<Actor>, angle: f32, flag: bool) -> bool {
    if !e.call(ACTOR_CAN_MOVE, &args![this]).bool() {
        return true;
    }
    let process = e.call(ACTOR_PROCESS, &args![this]).u32();
    if process != 0
        && e.call(PROCESS_FLAG_0X28, &args![process]).u32() == 0
        && fn_008bb5a0(e, Ptr::new(process)) != 0
    {
        return true;
    }
    let mover = e.get(this, Actor::pActorMover);
    e.call(REQUEST_ROTATE_ACTOR_OV2, &args![mover, angle, flag])
        .bool()
}

// ---------------------------------------------------------------------------
// 008bb630 .. 008bcda0 (second session of this part)

/// `ECX` = the actor mover (`Actor + 0x190`), one word: the flee request
/// `Actor::SetPathfindingFlee` forwards.
const ACTOR_MOVER_SET_FLEE: u32 = 0x009d_ba30;
/// `ECX` = a one-byte setting: returns the address of its value byte.
const SETTING_BYTE_POINTER: u32 = 0x0040_8d60;
/// `ECX` = a `float` setting: returns the address of its value.
const SETTING_FLOAT_POINTER: u32 = 0x0040_3e20;
/// The one-byte setting `008bb650` tests before it looks up the sighting node.
const SIGHTING_NODE_SETTING: u32 = 0x011d_898c;
/// The text `##SightingNode`.
const SIGHTING_NODE_NAME: u32 = 0x0108_4fa0;
/// `ECX` = the player, one word (`1`): a 3D node of the player.
const PLAYER_GET_NODE: u32 = 0x0095_0bb0;
/// `cdecl`, a node and a name: the child node of that name.
const FIND_CHILD_BY_NAME: u32 = 0x004a_ae30;
/// `ECX` = actor: the actor's animation.
const ACTOR_ANIMATION: u32 = 0x008b_70d0;
/// The object whose word `0044ddc0` reads for the HUD mode test of
/// `008bb650` (the mode number `4` skips the whole block).
const HUD_MODE_OBJECT: u32 = 0x011f_2250;
/// `HUDMainMenu::SetMenuMode` (decompiler name), `cdecl`, one word.
const HUD_SET_MENU_MODE: u32 = 0x0077_1700;
/// `ECX` = the object `0044ddc0` returned for the held item.
const ITEM_STEP_00504E60: u32 = 0x0050_4e60;
/// `ECX` = the object `00504e60` returned: non-zero lets `008bb650` go on.
const ITEM_STEP_0048CEE0: u32 = 0x0048_cee0;
/// `ECX` = the player: a byte compared with the answer of
/// [`PLAYER_FLAG_B`].
const PLAYER_FLAG_A: u32 = 0x0057_37e0;
/// `ECX` = the player: the byte compared with the answer of
/// [`PLAYER_FLAG_A`].
const PLAYER_FLAG_B: u32 = 0x0052_4d10;
/// `ECX` = the object `0044ddc0` returned for the held item; `bool` in `AL`.
const ITEM_TEST_004AD030: u32 = 0x004a_d030;
/// `ItemChange::HasModEffectActive` (decompiler name): `ECX` = the held
/// item, an effect number and the address of a `float`; `bool` in `AL`.
const HAS_MOD_EFFECT_ACTIVE_VALUE: u32 = 0x004b_d8d0;
/// `ECX` = the object `0044ddc0` returned: an index into [`WOBBLE_TABLE`].
const ITEM_WOBBLE_INDEX: u32 = 0x0044_6390;
/// Table of words (one per index of `00446390`) that `008bb650` passes to
/// [`CLEAR_GUN_WOBBLE`].
const WOBBLE_TABLE: u32 = 0x0118_a838;
/// `ClearGunWobble` (decompiler name), `cdecl`: a table word and a node.
const CLEAR_GUN_WOBBLE: u32 = 0x008d_6a80;
/// `PlayerCharacter::ForceTemp1stPerson` (decompiler name): `ECX` = the
/// player, one word; `bool` in `AL`.
const FORCE_TEMP_FIRST_PERSON: u32 = 0x0095_0460;
/// `ECX` = a node of the player, one word: the request `008bb650` makes
/// after resetting the gun wobble.
const NODE_REQUEST_00450F90: u32 = 0x0045_0f90;
/// `ECX` = the object `0044ddc0` returned for the held item; `bool` in
/// `AL` (first test that decides whether the sights group is added back).
const ITEM_SIGHTS_TEST_A: u32 = 0x008b_2c10;
/// The same test for the player's own view; `bool` in `AL`.
const ITEM_SIGHTS_TEST_B: u32 = 0x008b_2bf0;
/// `TESAnimGroup::IsIronSightsAction_ov2` (decompiler name), `cdecl`, one
/// word: the animation group kind; `bool` in `AL`.
const ANIM_GROUP_IS_IRON_SIGHTS: u32 = 0x005f_2750;
/// `ECX` = the object `0048f7f0` returned: the current group number (`AX`).
const ANIMATION_STEP_GROUP: u32 = 0x0047_d3b0;
/// `Animation::BlendOut` (decompiler name): `ECX` = the animation, two words.
const ANIMATION_BLEND_OUT: u32 = 0x0049_94f0;

/// `NiObjectNET::GetExtraData` (decompiler name): `ECX` = the object, one
/// word (a name handle); returns the extra data.
const GET_EXTRA_DATA_BY_NAME: u32 = 0x00a5_bdd0;
/// The text `008bbc40` looks the extra data up by.
const EXTRA_DATA_NAME: u32 = 0x010c_8afc;
/// `ECX` = a temporary, one word (the text): builds the name handle that
/// `00a5bdd0` takes; returns the temporary.
const NAME_HANDLE_BUILD: u32 = 0x0043_8170;
/// `ECX` = the temporary: releases the name handle.
const NAME_HANDLE_RELEASE: u32 = 0x0043_81b0;
/// `TESObjectREFR::GetScale` (decompiler name): `ECX` = the reference,
/// `float` in ST0.
const GET_SCALE: u32 = 0x0056_7400;
/// `ECX` = a list element: the key word the virtual `+0x9c` takes.
const ELEMENT_KEY: u32 = 0x0041_3f40;
/// `ECX` = a list element, the object `+0x9c` returned and a `float`.
const ELEMENT_APPLY_SCALE: u32 = 0x00cb_a6f0;
/// `ECX` = a list (`this + 0x0c`), one word (the index): the list node.
const LIST_NODE_AT: u32 = 0x006a_7ad0;

/// `TESPackage::CreatePackage` (decompiler name), `cdecl`, one word.
const PACKAGE_CREATE: u32 = 0x0067_0b90;
/// `TESPackage::SetPackType` (decompiler name): `ECX` = package, one word.
const PACKAGE_SET_TYPE: u32 = 0x0067_0fc0;
/// `ECX` = package, one word (`0x2c`).
const PACKAGE_SET_FLAGS_WORD: u32 = 0x0098_4f60;
/// `ECX` = package, one word (`1`).
const PACKAGE_SET_FLAG: u32 = 0x0082_6b40;
/// `PackageLocation::PackageLocation` (decompiler name): `ECX` = the block.
const PACKAGE_LOCATION_CONSTRUCTOR: u32 = 0x0067_f030;
/// `PackageLocation::SetLocReference` (decompiler name): `ECX` = the
/// location, one word.
const PACKAGE_LOCATION_SET_REFERENCE: u32 = 0x0067_f3c0;
/// `TESPackage::SetPackageLocation` (decompiler name): `ECX` = package,
/// the location.
const PACKAGE_SET_LOCATION: u32 = 0x0067_1d30;
/// `PackageLocation::~PackageLocation` (decompiler name): `ECX` = the
/// location, a deletion flag.
const PACKAGE_LOCATION_DESTRUCTOR: u32 = 0x0067_0b30;
/// `PackageTarget::PackageTarget` (decompiler name): `ECX` = the block.
const PACKAGE_TARGET_CONSTRUCTOR: u32 = 0x0067_ff70;
/// `PackageTarget::SetTargType` (decompiler name): `ECX` = the target, one
/// word.
const PACKAGE_TARGET_SET_TYPE: u32 = 0x0068_00b0;
/// `ECX` = the target, one word: the first target's reference.
const PACKAGE_TARGET_SET_FIRST: u32 = 0x0068_0140;
/// `PackageTarget::SetTargReference` (decompiler name): `ECX` = target,
/// one word.
const PACKAGE_TARGET_SET_REFERENCE: u32 = 0x0068_0110;
/// `TESPackage::SetPackageTarget` (decompiler name): `ECX` = package, the
/// target.
const PACKAGE_SET_TARGET: u32 = 0x0067_2fc0;
/// `ECX` = the target, a deletion flag (`PackageTarget` destructor).
const PACKAGE_TARGET_DESTRUCTOR: u32 = 0x007b_3fa0;
/// `ECX` = the stack target: its destructor.
const PACKAGE_TARGET_STACK_DESTRUCTOR: u32 = 0x0048_3710;
/// `Actor::GetCurrentEditorPackage` (decompiler name): `ECX` = actor.
const ACTOR_CURRENT_EDITOR_PACKAGE: u32 = 0x0088_14b0;
/// `ECX` = package, the editor package.
const PACKAGE_COPY_EDITOR: u32 = 0x0067_a1b0;
/// `TESPackage::SetPackageSecondLocation` (decompiler name): `ECX` =
/// package, the location.
const PACKAGE_SET_SECOND_LOCATION: u32 = 0x0067_1e10;
/// `TESPackage::GetOrCreateUseWeaponPackageData` (decompiler name): `ECX`
/// = package.
const PACKAGE_USE_WEAPON_DATA: u32 = 0x0067_2d10;
/// `TESUseWeaponPackageData::SetAttackTarget` (decompiler name): `ECX` =
/// the data, the address of a `PackageTarget`.
const USE_WEAPON_SET_ATTACK_TARGET: u32 = 0x0067_d470;

/// The `float` setting `008bc240` copies (`ECX` of `00403e20`).
const ACTION_TIME_SETTING: u32 = 0x011c_d8d8;
/// `ECX` = the other actor: a value `008bc270` hands on.
const OTHER_VALUE_0044EDB0: u32 = 0x0044_edb0;
/// `ECX` = the other actor: a kind number (`1` and `2` are tested).
const OTHER_KIND_009611E0: u32 = 0x0096_11e0;
/// `ECX` = the actor's `+0xac` object, the value and `1`.
const OBJECT_SET_00C74820: u32 = 0x00c7_4820;
/// `ECX` = the actor's `+0xac` object, the value and `1`.
const OBJECT_SET_008978F0: u32 = 0x0089_78f0;
/// `TESActorBaseData::GetKarma` (decompiler name): `ECX` = the other
/// actor, `float` in ST0.
const OTHER_KARMA: u32 = 0x0047_c860;
/// `ECX` = the actor's `+0xac` object, the value and a `float`.
const OBJECT_SET_KARMA: u32 = 0x00c7_4870;

/// `ECX` = a value, two words (`1`, the callback): registers the callback.
const CALLBACK_REGISTER: u32 = 0x004b_05d0;
/// `ECX` = actor: the first object the lookup of `008bc300` starts from
/// (the player's branch).
const PLAYER_LOOKUP_OBJECT: u32 = 0x0043_fcd0;
/// `ECX` = an object: the key `00653270` looks up.
const LOOKUP_KEY: u32 = 0x0043_b230;
/// `cdecl`, a table and a key: the entry or zero.
const TABLE_FIND: u32 = 0x0065_3270;
/// The table `008bc300` looks the key up in.
const LOOKUP_TABLE: u32 = 0x011f_36ac;
/// `ECX` = the entry: returns the object `00a41520` is called on.
const ENTRY_TARGET: u32 = 0x0070_ec90;
/// `ECX` = the target object, two words (`1`, the value of the actor's
/// virtual `+0x238`).
const TARGET_NOTIFY: u32 = 0x00a4_1520;

/// `ECX` = the other actor: its owner.
const REFERENCE_OWNER: u32 = 0x005d_8a70;
/// `ECX` = the owner: non-zero when the greeting is not allowed.
const OWNER_BLOCKS_GREETING: u32 = 0x0082_56d0;
/// `ECX` = actor, one word (the player): `008bc3d0` calls it before it
/// starts the greeting.
const GREETING_PREPARE: u32 = 0x0057_bd60;
/// The byte setting passed as the greeting's fifth argument (`ECX` of
/// `00408d60`).
const GREETING_SETTING: u32 = 0x011e_05ec;
/// The current time as a word (`ECX` is not used).
const CURRENT_TIME_WORD: u32 = 0x0045_7fe0;
/// The `double` the player's timer at `+0xe24` is compared with.
const GREETING_TIMER_LIMIT: u32 = 0x0101_2060;

/// The global actor pointer `008bc590` resets.
const TRACKED_ACTOR_POINTER: u32 = 0x011d_f680;
/// The global `float` `008bc590` writes.
const TRACKED_ACTOR_TIME: u32 = 0x011d_951c;
/// The `float` setting `008bc590` copies into [`TRACKED_ACTOR_TIME`] (`ECX`
/// of `00403e20`).
const TRACKED_TIME_SETTING: u32 = 0x011d_32c4;
/// `ECX` = the object the actor's virtual `+0x1e4` returned, one `float`.
const TRACKED_OBJECT_SET_VALUE: u32 = 0x0049_bdc0;

/// `ECX` = the actor's combat target list head (`+0x12c`), three words
/// (the address of the other actor, `0`, a comparison function).
const LIST_FIND: u32 = 0x0071_9b20;
/// The comparison function `008bc700` hands to [`LIST_FIND`].
const COMBAT_TARGET_COMPARE: u32 = 0x009a_3830;
/// `ECX` = the reference's extra data list, two words (a word, a byte):
/// the first extra data call of `008bc750`.
const EXTRA_LIST_ADD_00422480: u32 = 0x0042_2480;
/// `ECX` = the extra data list, two words.
const EXTRA_LIST_ADD_0041D700: u32 = 0x0041_d700;
/// `ECX` = the extra data list, one word.
const EXTRA_LIST_REMOVE_00422550: u32 = 0x0042_2550;
/// `ECX` = the package: its type number (`1` and `7` are tested).
const PACKAGE_TYPE: u32 = 0x0041_ca90;
/// `ECX` = the extra data list: its package extra.
const EXTRA_LIST_GET_PACKAGE: u32 = 0x0041_cb10;
/// `ECX` = process virtual `+0x428` result, four words.
const SPEECH_START: u32 = 0x0097_f820;
/// `ECX` = the other actor: non-zero when it could follow (`bool`).
const OTHER_TEST_0047BCF0: u32 = 0x0047_bcf0;
/// `ECX` = the base form: the form the commanding test looks at.
const BASE_COMMANDING_FORM: u32 = 0x005f_9b50;
/// `ECX` = the commanding form, one pointer to a word: the object
/// `005f65d0` takes.
const COMMANDING_ENTRY: u32 = 0x0050_0940;
/// `ECX` = the object `00500940` returned, one word (the address of a
/// word holding the other actor); `bool` in `AL`.
const COMMANDING_TEST: u32 = 0x005f_65d0;
/// `ECX` = the player, one word (the teammate): the call that registers a
/// new teammate.
const PLAYER_ADD_TEAMMATE: u32 = 0x008a_bfa0;
/// `ECX` = the player's list at `+0x5fc`, one word (the address of a word
/// holding the teammate).
const TEAMMATE_LIST_REMOVE: u32 = 0x0090_5330;
/// `ECX` = the reference's extra data list: the player crime list.
const EXTRA_LIST_GET_CRIME_LIST: u32 = 0x0041_cf00;
/// `ECX` = the extra data list, two words (the list, a `float`).
const EXTRA_LIST_ADD_CRIME_LIST: u32 = 0x0041_ce50;
/// `ECX` = the list element's holder: the iterator over its elements.
const CRIME_LIST_ITERATOR: u32 = 0x009e_32d0;
/// `ECX` = a form, one word: the call made on every crime list element.
const CRIME_ELEMENT_MARK: u32 = 0x0047_eb90;
/// `ECX` = a pointer to a word: returns a `float` in ST0.
const CRIME_STAMP_VALUE: u32 = 0x006a_7f50;
/// `ECX` = the process lists singleton, one word.
const PROCESS_LISTS_NOTIFY: u32 = 0x0097_21f0;
/// The `ProcessLists` singleton object.
const PROCESS_LISTS_OBJECT: u32 = 0x011e_0e80;

/// The `float` setting `008bcc80` compares the actor value `0x14` with
/// (`ECX` of `00403e20`).
const RADIATION_LIMIT_SETTING: u32 = 0x011c_dcc8;
/// `MobileObject::GetCurrentProcessType` (decompiler name): `ECX` = actor;
/// `0` is the full (high) process.
const GET_PROCESS_TYPE: u32 = 0x0093_1850;
/// `HighProcess::FadeIn` (decompiler name): `ECX` = the process, the actor
/// and `0`.
const HIGH_PROCESS_FADE_IN: u32 = 0x008f_e8f0;
/// `HighProcess::FadeOut` (decompiler name): `ECX` = the process, the actor
/// and two zeros.
const HIGH_PROCESS_FADE_OUT: u32 = 0x008f_e960;
/// `ECX` = the actor's process: the `float` (ST0) `008bcda0` returns.
const FADE_VALUE: u32 = 0x008b_cdd0;
/// `MobileObject::GetCurrentPackage` (decompiler name): `ECX` = actor.
const ACTOR_CURRENT_PACKAGE: u32 = 0x0093_44a0;
/// `ECX` = package: first test of `008bcc80`; `bool` in `AL`.
const PACKAGE_AVOID_TEST_A: u32 = 0x0067_a380;
/// `ECX` = package: second test of `008bcc80`; `bool` in `AL`.
const PACKAGE_AVOID_TEST_B: u32 = 0x0067_ac50;

/// Reads the player pointer global.
fn player_pointer(e: &Engine) -> u32 {
    e.global::<u32>(PLAYER_POINTER)
}

// Translated from 008bb630 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Actor::SetPathfindingFlee` (Xbox PDB; `bool` there): forwards its word to
/// the actor mover (`Actor + 0x190`) and answers the mover's value (the exe
/// ends on that call and leaves its `EAX`; the callers test `AL`).
pub fn actor_set_pathfinding_flee(e: &mut Engine, this: Ptr<Actor>, value: u32) -> bool {
    let mover = e.get(this, Actor::pActorMover);
    e.call(ACTOR_MOVER_SET_FLEE, &args![mover, value]).bool()
}

// Translated from 008bbbd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Returns the byte at `+0x650` (a player flag; the iron sights code tests it
/// on the player).
pub fn fn_008bbbd0(e: &mut Engine, this: Ptr) -> u8 {
    e.mem.u8(this.addr() + 0x650)
}

// Translated from 008bbbf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores the word at `+0xe34` (the player's sighting node, set by
/// `Actor::SetIronSights`).
pub fn fn_008bbbf0(e: &mut Engine, this: Ptr, value: u32) {
    e.mem.set_u32(this.addr() + 0xe34, value);
}

// Translated from 008bbc10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Actor::GetIronSights` (Xbox PDB): process virtual `+0x404`, or zero
/// without a process.
pub fn actor_get_iron_sights(e: &mut Engine, this: Ptr<Actor>) -> u8 {
    let process = process_of(e, this.addr());
    if process == 0 {
        0
    } else {
        e.vcall(process, 0x404, &args![]).u8()
    }
}

// Translated from 008bb650 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Actor::SetIronSights` (Xbox PDB): switches the actor's weapon to or from
/// the iron sights through process virtual `+0x400`. Without a process, when
/// the actor's virtual `+0x21c` answers true, or when nothing changes (and
/// `force` is zero) it does nothing. For the player it also sets the sighting
/// node, the HUD mode and the gun wobble, and it re-selects the animation
/// group of the first and third person weapon animation. With
/// `skip_if_kind_one` set it stops before the switch when the current
/// animation kind is `1`.
pub fn actor_set_iron_sights(
    e: &mut Engine,
    this: Ptr<Actor>,
    sights: u8,
    force: u8,
    skip_if_kind_one: u8,
) {
    let actor = this.addr();
    if process_of(e, actor) == 0 {
        return;
    }
    if e.vcall(actor, 0x21c, &args![]).bool() {
        return;
    }
    if force == 0 && actor_get_iron_sights(e, this) == sights {
        return;
    }
    if actor == player_pointer(e) {
        if sights != 0 {
            let setting = e
                .call(SETTING_BYTE_POINTER, &args![SIGHTING_NODE_SETTING])
                .u32();
            if e.mem.u8(setting) != 0 {
                let player = player_pointer(e);
                let node = e.call(PLAYER_GET_NODE, &args![player, 1u32]).u32();
                let found = e
                    .call(FIND_CHILD_BY_NAME, &args![node, SIGHTING_NODE_NAME])
                    .u32();
                let player = player_pointer(e);
                fn_008bbbf0(e, Ptr::new(player), found);
            }
        } else {
            let player = player_pointer(e);
            fn_008bbbf0(e, Ptr::new(player), 0);
        }
    }
    let animation = e.call(ACTOR_ANIMATION, &args![actor]).u32();
    let step = if animation != 0 {
        e.call(ANIMATION_SLOT, &args![animation, 4u32]).u32()
    } else {
        0
    };
    let mut kind = 0xbu32;
    if step != 0 && e.call(GET_GENERIC_LOCATION, &args![step]).u32() != 3 {
        let object = e.call(ANIMATION_STEP, &args![step]).u32();
        let index = e.call(ANIMATION_KIND, &args![object]).u32();
        kind = e
            .mem
            .u32(ANIMATION_KIND_TABLE.wrapping_add(index.wrapping_mul(0x24)));
    }
    if skip_if_kind_one != 0 && kind == 1 {
        return;
    }
    process_vcall(e, actor, 0x400, &args![sights]);
    if actor == player_pointer(e) && e.call(ENTRY_OBJECT, &args![HUD_MODE_OBJECT]).u32() != 4 {
        iron_sights_player_view(e, actor, sights);
    }
    if kind != 1 {
        return;
    }
    if step != 0 {
        let object = e.call(ANIMATION_STEP, &args![step]).u32();
        let mut group = e.call(ANIMATION_KIND, &args![object]).i32();
        if e.call(ANIM_GROUP_IS_IRON_SIGHTS, &args![group]).bool() {
            group = group.wrapping_sub(3);
        }
        if actor_get_iron_sights(e, this) != 0 {
            let held = process_vcall(e, actor, PROCESS_VSLOT_EQUIPPED, &args![]).u32();
            let add = if held != 0 {
                let held = process_vcall(e, actor, PROCESS_VSLOT_EQUIPPED, &args![]).u32();
                let object = e.call(ENTRY_OBJECT, &args![held]).u32();
                e.call(ITEM_SIGHTS_TEST_A, &args![object]).bool()
            } else {
                true
            };
            if add {
                group = group.wrapping_add(3);
            }
        }
        reselect_animation_group(e, actor, group, animation, step);
    }
    if actor != player_pointer(e) {
        return;
    }
    let player = player_pointer(e);
    let animation = e.call(PLAYER_GET_ANIMATION, &args![player, 1u32]).u32();
    let step = if animation != 0 {
        e.call(ANIMATION_SLOT, &args![animation, 4u32]).u32()
    } else {
        0
    };
    if step == 0 {
        return;
    }
    let object = e.call(ANIMATION_STEP, &args![step]).u32();
    let mut group = e.call(ANIMATION_KIND, &args![object]).i32();
    if e.call(ANIM_GROUP_IS_IRON_SIGHTS, &args![group]).bool() {
        group = group.wrapping_sub(3);
    }
    let held = process_vcall(e, actor, PROCESS_VSLOT_EQUIPPED, &args![]).u32();
    if actor_get_iron_sights(e, this) != 0 {
        let add = if held == 0 {
            true
        } else {
            let current = process_vcall(e, actor, PROCESS_VSLOT_EQUIPPED, &args![]).u32();
            let object = e.call(ENTRY_OBJECT, &args![current]).u32();
            if !e.call(ITEM_SIGHTS_TEST_B, &args![object]).bool() {
                false
            } else {
                let object = e.call(ENTRY_OBJECT, &args![held]).u32();
                if e.call(ITEM_TEST_004AD030, &args![object]).bool() {
                    !e.call(HAS_MOD_EFFECT_ACTIVE, &args![held, 0xeu32]).bool()
                } else {
                    true
                }
            }
        };
        if add {
            group = group.wrapping_add(3);
        }
    }
    reselect_animation_group(e, actor, group, animation, step);
}

/// The tail both animation blocks of `008bb650` share: picks the actor's
/// animation group for `group` and, when its type is `group` and differs
/// from the step's current group, blends the animation out (`4`, `1`).
fn reselect_animation_group(e: &mut Engine, actor: u32, group: i32, animation: u32, step: u32) {
    let picked = e
        .call(GET_ANIM_GROUP, &args![actor, group, 0u32, 0u32, animation])
        .u16();
    let group_type = e.call(ANIM_GROUP_GET_TYPE, &args![picked as u32]).i32();
    if group_type == group {
        let object = e.call(ANIMATION_STEP, &args![step]).u32();
        let current = e.call(ANIMATION_STEP_GROUP, &args![object]).u16();
        if picked != current {
            e.call(ANIMATION_BLEND_OUT, &args![animation, 4u32, 1u32]);
        }
    }
}

/// The player block of `008bb650`: sets the HUD menu mode, and when the held
/// weapon passes the tests resets the gun wobble and the view.
fn iron_sights_player_view(e: &mut Engine, actor: u32, sights: u8) {
    e.call(HUD_SET_MENU_MODE, &args![1u32]);
    e.with_stack(4, |e, local| {
        e.mem.set_f32(local.addr(), 1.0);
        if !process_vcall(e, actor, 0x454, &args![]).bool() {
            return;
        }
        if process_vcall(e, actor, PROCESS_VSLOT_EQUIPPED, &args![]).u32() == 0 {
            return;
        }
        let held = process_vcall(e, actor, PROCESS_VSLOT_EQUIPPED, &args![]).u32();
        let object = e.call(ENTRY_OBJECT, &args![held]).u32();
        let object = e.call(ITEM_STEP_00504E60, &args![object]).u32();
        if e.call(ITEM_STEP_0048CEE0, &args![object]).u32() == 0 {
            return;
        }
        let player = player_pointer(e);
        let flag_a = e.call(PLAYER_FLAG_A, &args![player]).u8();
        let player = player_pointer(e);
        let flag_b = e.call(PLAYER_FLAG_B, &args![player]).u8();
        if flag_a != flag_b {
            return;
        }
        let held = process_vcall(e, actor, PROCESS_VSLOT_EQUIPPED, &args![]).u32();
        let object = e.call(ENTRY_OBJECT, &args![held]).u32();
        if e.call(ITEM_TEST_004AD030, &args![object]).bool() {
            let held = process_vcall(e, actor, PROCESS_VSLOT_EQUIPPED, &args![]).u32();
            if !e
                .call(HAS_MOD_EFFECT_ACTIVE_VALUE, &args![held, 0xeu32, local])
                .bool()
            {
                return;
            }
        }
        let player = player_pointer(e);
        if sights != 0 {
            let node = e.call(PLAYER_GET_NODE, &args![player, 1u32]).u32();
            let held = process_vcall(e, actor, PROCESS_VSLOT_EQUIPPED, &args![]).u32();
            let object = e.call(ENTRY_OBJECT, &args![held]).u32();
            let index = e.call(ITEM_WOBBLE_INDEX, &args![object]).u32();
            let word = e.mem.u32(WOBBLE_TABLE.wrapping_add(index.wrapping_mul(4)));
            e.call(CLEAR_GUN_WOBBLE, &args![word, node]);
            let player = player_pointer(e);
            if !e.call(FORCE_TEMP_FIRST_PERSON, &args![player, 1u32]).bool() {
                let player = player_pointer(e);
                let node = e.call(PLAYER_GET_NODE, &args![player, 1u32]).u32();
                e.call(NODE_REQUEST_00450F90, &args![node, 1u32]);
            }
        } else {
            let node = e.call(PLAYER_GET_NODE, &args![player, 1u32]).u32();
            e.call(CLEAR_GUN_WOBBLE, &args![0u32, node]);
            let player = player_pointer(e);
            if fn_008bbbd0(e, Ptr::new(player)) == 0 {
                let player = player_pointer(e);
                let node = e.call(PLAYER_GET_NODE, &args![player, 1u32]).u32();
                e.call(NODE_REQUEST_00450F90, &args![node, 0u32]);
            }
        }
    });
}

// Translated from 008bbc40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Actor::ApplyConstrTemplate` (Xbox PDB): looks up the extra data named by
/// the text at `010c8afc` on the 3D object `node`, then asks the actor's
/// virtual `+0x1d0` object for the form of every element and applies the
/// actor's scale to it. Answers true when both the object and the extra data
/// exist (the exception frame is not translated).
pub fn actor_apply_constr_template(e: &mut Engine, this: Ptr<Actor>, node: u32) -> bool {
    let actor = this.addr();
    let object = e.vcall(actor, 0x1d0, &args![]).u32();
    let extra = e.with_stack(8, |e, handle| {
        let built = e
            .call(NAME_HANDLE_BUILD, &args![handle, EXTRA_DATA_NAME])
            .u32();
        let extra = e.call(GET_EXTRA_DATA_BY_NAME, &args![node, built]).u32();
        e.call(NAME_HANDLE_RELEASE, &args![handle]);
        extra
    });
    let scale = e.call(GET_SCALE, &args![actor]).f32();
    if object == 0 || extra == 0 {
        return false;
    }
    let mut index = 0u32;
    while index < fn_008bbd90(e, Ptr::new(extra)) {
        let element = fn_008bbd60(e, Ptr::new(extra), index);
        let key = e.call(ELEMENT_KEY, &args![element]).u32();
        let target = e.vcall(object, 0x9c, &args![key]).u32();
        if target != 0 {
            e.call(ELEMENT_APPLY_SCALE, &args![element, target, scale]);
        }
        index += 1;
    }
    true
}

// Translated from 008bbd60 (decompiled, FalloutNV.exe 1.4.0.525)
/// The element at `index` of the list embedded at `+0x0c`: the list node is
/// dereferenced through `00559450`.
pub fn fn_008bbd60(e: &mut Engine, this: Ptr, index: u32) -> u32 {
    let node = e
        .call(LIST_NODE_AT, &args![this.addr() + 0x0c, index])
        .u32();
    e.call(POINTER_GET, &args![node]).u32()
}

// Translated from 008bbd90 (decompiled, FalloutNV.exe 1.4.0.525)
/// The number of elements of the list embedded at `+0x0c` (`0044ddc0`).
pub fn fn_008bbd90(e: &mut Engine, this: Ptr) -> u32 {
    e.call(ENTRY_OBJECT, &args![this.addr() + 0x0c]).u32()
}

// Translated from 008bbdb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the four `float`s at `+0x178`, `+0x17c`, `+0x180` and `+0x184` to the
/// constant at `01012054`.
pub fn fn_008bbdb0(e: &mut Engine, this: Ptr) {
    for offset in [0x178u32, 0x17c, 0x180, 0x184] {
        let value = e.global::<f32>(0x0101_2054);
        e.mem.set_f32(this.addr() + offset, value);
    }
}

// Translated from 008bbe00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the `float` at `+0x188` to the constant at `01012054`.
pub fn fn_008bbe00(e: &mut Engine, this: Ptr) {
    let value = e.global::<f32>(0x0101_2054);
    e.mem.set_f32(this.addr() + 0x188, value);
}

// Translated from 008bbe20 (decompiled, FalloutNV.exe 1.4.0.525)
/// A callback with `this` at `Actor + 0xa8` (the `CachedValuesOwner`): asks
/// the `ActorValueOwner` before it (`this - 4`, virtual `+0x8`, argument `0`)
/// and, when the actor's process (`this - 0x40`) has a cached-values block,
/// stores the answer into it through `008bbe70`. Returns the answer.
pub fn fn_008bbe20(e: &mut Engine, this: Ptr) -> u32 {
    let value = e.vcall(this.addr() - 4, 0x8, &args![0u32]).u32();
    let process = e.mem.u32(this.addr() - 0x40);
    if process != 0 && e.call(HAS_CACHED_VALUES, &args![process]).bool() {
        fn_008bbe70(e, Ptr::new(process), value);
    }
    value
}

// Translated from 008bbe70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores `value` at `+0x30` of the process's cached-values block (`+0x2c`)
/// and marks the flag `0x400` stale, when the block exists.
pub fn fn_008bbe70(e: &mut Engine, this: Ptr, value: u32) {
    let block = e.mem.u32(this.addr() + 0x2c);
    if block != 0 {
        e.mem.set_u32(block + 0x30, value);
        let block = e.mem.u32(this.addr() + 0x2c);
        e.call(CACHED_VALUES_ADD_FLAGS, &args![block, 0x400u32]);
    }
}

// Translated from 008bbeb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Like `008bbe20` with the `ActorValueOwner` argument `0x39` and the
/// writer `008bbf00`.
pub fn fn_008bbeb0(e: &mut Engine, this: Ptr) -> u32 {
    let value = e.vcall(this.addr() - 4, 0x8, &args![0x39u32]).u32();
    let process = e.mem.u32(this.addr() - 0x40);
    if process != 0 && e.call(HAS_CACHED_VALUES, &args![process]).bool() {
        fn_008bbf00(e, Ptr::new(process), value);
    }
    value
}

// Translated from 008bbf00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores `value` at `+0x34` of the process's cached-values block (`+0x2c`)
/// and marks the flag `0x800` stale, when the block exists.
pub fn fn_008bbf00(e: &mut Engine, this: Ptr, value: u32) {
    let block = e.mem.u32(this.addr() + 0x2c);
    if block != 0 {
        e.mem.set_u32(block + 0x34, value);
        let block = e.mem.u32(this.addr() + 0x2c);
        e.call(CACHED_VALUES_ADD_FLAGS, &args![block, 0x800u32]);
    }
}

// Translated from 008bbf40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Builds a package of type `0x10` (use weapon) for the actor and starts it
/// (actor virtual `+0x2f4`): the first target is `target_reference`, the
/// package location `location_reference`, the stack target (which becomes
/// the weapon's attack target) `attack_reference`, and a second location
/// `second_location_reference` when it is not zero. The weapon data takes
/// the four flag bytes and `range` twice. Does nothing without a process.
/// The exception frame is not translated.
#[allow(clippy::too_many_arguments)]
pub fn fn_008bbf40(
    e: &mut Engine,
    this: Ptr<Actor>,
    target_reference: u32,
    location_reference: u32,
    attack_reference: u32,
    range: u16,
    second_location_reference: u32,
    flag_0e: u8,
    flag_0f: u8,
    flag_0c: u8,
    flag_0d: u8,
) {
    let actor = this.addr();
    if process_of(e, actor) == 0 {
        return;
    }
    let package = e.call(PACKAGE_CREATE, &args![0x10u32]).u32();
    e.call(PACKAGE_SET_TYPE, &args![package, 0x10u32]);
    e.call(PACKAGE_SET_FLAGS_WORD, &args![package, 0x2cu32]);
    e.call(PACKAGE_SET_FLAG, &args![package, 1u32]);
    let location = new_package_location(e);
    e.call(
        PACKAGE_LOCATION_SET_REFERENCE,
        &args![location, location_reference],
    );
    e.call(PACKAGE_SET_LOCATION, &args![package, location]);
    if location != 0 {
        e.call(PACKAGE_LOCATION_DESTRUCTOR, &args![location, 1u32]);
    }
    let block = e.call(OPERATOR_NEW, &args![0x10u32]).u32();
    let target = if block != 0 {
        e.call(PACKAGE_TARGET_CONSTRUCTOR, &args![block]).u32()
    } else {
        0
    };
    e.call(PACKAGE_TARGET_SET_TYPE, &args![target, 1u32]);
    e.call(PACKAGE_TARGET_SET_FIRST, &args![target, target_reference]);
    e.call(PACKAGE_SET_TARGET, &args![package, target]);
    if target != 0 {
        e.call(PACKAGE_TARGET_DESTRUCTOR, &args![target, 1u32]);
    }
    e.with_stack(0x10, |e, attack_target| {
        e.call(PACKAGE_TARGET_CONSTRUCTOR, &args![attack_target]);
        e.call(PACKAGE_TARGET_SET_TYPE, &args![attack_target, 0u32]);
        e.call(
            PACKAGE_TARGET_SET_REFERENCE,
            &args![attack_target, attack_reference],
        );
        let editor_package = e.call(ACTOR_CURRENT_EDITOR_PACKAGE, &args![actor]).u32();
        if editor_package != 0 {
            e.call(PACKAGE_COPY_EDITOR, &args![package, editor_package]);
        }
        if second_location_reference != 0 {
            let second = new_package_location(e);
            e.call(
                PACKAGE_LOCATION_SET_REFERENCE,
                &args![second, second_location_reference],
            );
            e.call(PACKAGE_SET_SECOND_LOCATION, &args![package, second]);
            if second != 0 {
                e.call(PACKAGE_LOCATION_DESTRUCTOR, &args![second, 1u32]);
            }
        }
        let data = e.call(PACKAGE_USE_WEAPON_DATA, &args![package]).u32();
        e.mem.set_u8(data + 0xe, flag_0e);
        e.mem.set_u8(data + 0xf, flag_0f);
        e.mem.set_u8(data + 0xc, flag_0c);
        e.mem.set_u8(data + 0xd, flag_0d);
        e.mem.set_u8(data + 0x10, 1);
        e.mem.set_u8(data + 0x11, 0);
        e.mem.set_u16(data + 0x12, 1);
        e.call(USE_WEAPON_SET_ATTACK_TARGET, &args![data, attack_target]);
        e.mem.set_u16(data + 0x14, range);
        e.mem.set_u16(data + 0x16, range);
        let process = e.call(ACTOR_PROCESS, &args![actor]).u32();
        e.vcall(process, 0x28, &args![]);
        e.vcall(actor, 0x2f4, &args![package, 1u32, 1u32]);
        e.call(PACKAGE_TARGET_STACK_DESTRUCTOR, &args![attack_target]);
    });
}

/// Allocates (`operator new`, `0xc` bytes) and constructs a
/// `PackageLocation`; zero when the allocation fails.
fn new_package_location(e: &mut Engine) -> u32 {
    let block = e.call(OPERATOR_NEW, &args![0xcu32]).u32();
    if block != 0 {
        e.call(PACKAGE_LOCATION_CONSTRUCTOR, &args![block]).u32()
    } else {
        0
    }
}

// Translated from 008bc240 (decompiler name unknown, FalloutNV.exe 1.4.0.525)
/// Stores `value` into `Actor::iActionValue` and the `float` setting at
/// `011cd8d8` into `Actor::fTimeronAction`.
pub fn fn_008bc240(e: &mut Engine, this: Ptr<Actor>, value: u32) {
    e.set(this, Actor::iActionValue, value);
    let setting = e
        .call(SETTING_FLOAT_POINTER, &args![ACTION_TIME_SETTING])
        .u32();
    let time = e.mem.f32(setting);
    e.set(this, Actor::fTimeronAction, time);
}

// Translated from 008bc270 (decompiled, FalloutNV.exe 1.4.0.525)
/// A callback (`cdecl`, two words: the actor and another actor) used by
/// `008bc300`. When the actor has a `+0xac` object: calls `0044edb0` and
/// `009611e0` on the other actor and, for kind `1`, `00c74820(.., 1)` and,
/// for kind `2`, `008978f0(.., 1)` on the object; then always hands the other
/// actor's karma to `00c74870`.
pub fn fn_008bc270(e: &mut Engine, actor: Ptr, other: Ptr) {
    let object = e.mem.u32(actor.addr() + 0xac);
    if object == 0 {
        return;
    }
    let value = e.call(OTHER_VALUE_0044EDB0, &args![other]).u32();
    if e.call(OTHER_KIND_009611E0, &args![other]).i32() == 1 {
        let object = e.mem.u32(actor.addr() + 0xac);
        e.call(OBJECT_SET_00C74820, &args![object, value, 1u32]);
    }
    if e.call(OTHER_KIND_009611E0, &args![other]).i32() == 2 {
        let object = e.mem.u32(actor.addr() + 0xac);
        e.call(OBJECT_SET_008978F0, &args![object, value, 1u32]);
    }
    let karma = e.call(OTHER_KARMA, &args![other]).f32();
    let object = e.mem.u32(actor.addr() + 0xac);
    e.call(OBJECT_SET_KARMA, &args![object, value, karma]);
}

// Translated from 008bc300 (decompiled, FalloutNV.exe 1.4.0.525)
/// Registers the callback `008bc270` (`004b05d0`) on the value the actor's
/// virtual `+0x238` returns, then (for the player through `0043fcd0`, for
/// every other actor through virtual `+0x1d0`) looks the actor's key up in
/// the table at `011f36ac` and, when found, notifies the entry's target.
pub fn fn_008bc300(e: &mut Engine, this: Ptr<Actor>) {
    let actor = this.addr();
    let value = e.vcall(actor, 0x238, &args![]).u32();
    e.call(CALLBACK_REGISTER, &args![value, 1u32, 0x008b_c270u32]);
    let object = if actor == player_pointer(e) {
        e.call(PLAYER_LOOKUP_OBJECT, &args![actor]).u32()
    } else {
        e.vcall(actor, 0x1d0, &args![]).u32()
    };
    let key = e.call(LOOKUP_KEY, &args![object]).u32();
    let entry = e.call(TABLE_FIND, &args![LOOKUP_TABLE, key]).u32();
    if entry != 0 {
        let target = e.call(ENTRY_TARGET, &args![entry]).u32();
        e.call(TARGET_NOTIFY, &args![target, 1u32, value]);
    }
}

// Translated from 008bc3d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Actor::StartGreetingPlayer` (Xbox PDB): when `other` has an owner that
/// is not blocking and the actor has a process, tells the process (virtual
/// `+0x628`) about the player; if the player's greeting timer (`+0xe24`) is
/// below the limit starts the greeting (process virtual `+0x2a4`) and
/// restarts the timer; then, when the process does not answer on virtual
/// `+0x4bc` but does on `+0x30c`, calls virtual `+0x1dc`.
pub fn actor_start_greeting_player(e: &mut Engine, this: Ptr<Actor>, other: u32) {
    let actor = this.addr();
    if other == 0 {
        return;
    }
    if e.call(REFERENCE_OWNER, &args![other]).u32() == 0 {
        return;
    }
    let owner = e.call(REFERENCE_OWNER, &args![other]).u32();
    if e.call(OWNER_BLOCKS_GREETING, &args![owner]).bool() {
        return;
    }
    if process_of(e, actor) == 0 {
        return;
    }
    let player = player_pointer(e);
    process_vcall(e, actor, 0x628, &args![player]);
    let player = player_pointer(e);
    if fn_008bc520(e, Ptr::new(player)) {
        let player = player_pointer(e);
        e.call(GREETING_PREPARE, &args![actor, player]);
        process_vcall(e, actor, 0x310, &args![1u32]);
        let setting = e.call(SETTING_BYTE_POINTER, &args![GREETING_SETTING]).u32();
        let byte = e.mem.u8(setting);
        process_vcall(
            e,
            actor,
            0x2a4,
            &args![actor, other, 0u32, 0u32, byte, 0u32],
        );
        let player = player_pointer(e);
        fn_008bc560(e, Ptr::new(player));
    }
    if process_vcall(e, actor, 0x4bc, &args![]).u32() == 0
        && process_vcall(e, actor, 0x30c, &args![]).bool()
    {
        process_vcall(e, actor, 0x1dc, &args![actor, 1u32]);
    }
}

// Translated from 008bc520 (decompiled, FalloutNV.exe 1.4.0.525)
/// True when the `float` at `+0xe24` (the player's greeting timer) is below
/// the `double` at `01012060`.
pub fn fn_008bc520(e: &mut Engine, this: Ptr) -> bool {
    let timer = e.mem.f32(this.addr() + 0xe24) as f64;
    timer < e.global::<f64>(GREETING_TIMER_LIMIT)
}

// Translated from 008bc560 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores the current time word (`00457fe0`) as a `float` at `+0xe24`.
pub fn fn_008bc560(e: &mut Engine, this: Ptr) {
    let now = e.call(CURRENT_TIME_WORD, &args![]).u32();
    e.mem.set_f32(this.addr() + 0xe24, now as f32);
}

// Translated from 008bc590 (decompiled, FalloutNV.exe 1.4.0.525)
/// `cdecl`, two arguments. Resets the actor held in the global at
/// `011df680`: clears its byte `+0x7d`, sets `+0x7c`, sets `+0x7e` when
/// `flag` is not zero; if its process answers on virtual `+0x748` and the
/// actor on virtual `+0x1e4`, zeroes that object's value and tells the
/// process (`+0x744`, `+0x74c` with `0`); then either asks the process
/// (virtual `+0x44`) or clears `+0x7e`; when `keep_time` is zero copies the
/// `float` setting at `011d32c4` to `011d951c`. Finally clears the global.
pub fn fn_008bc590(e: &mut Engine, flag: u32, keep_time: u8) {
    if e.global::<u32>(TRACKED_ACTOR_POINTER) != 0 {
        let actor = e.global::<u32>(TRACKED_ACTOR_POINTER);
        e.mem.set_u8(actor + 0x7d, 0);
        let actor = e.global::<u32>(TRACKED_ACTOR_POINTER);
        e.mem.set_u8(actor + 0x7c, 1);
        if flag != 0 {
            let actor = e.global::<u32>(TRACKED_ACTOR_POINTER);
            e.mem.set_u8(actor + 0x7e, 1);
        }
        let actor = e.global::<u32>(TRACKED_ACTOR_POINTER);
        if e.call(ACTOR_PROCESS, &args![actor]).u32() != 0 {
            let actor = e.global::<u32>(TRACKED_ACTOR_POINTER);
            let process = e.call(ACTOR_PROCESS, &args![actor]).u32();
            if e.vcall(process, 0x748, &args![]).u32() != 0 {
                let actor = e.global::<u32>(TRACKED_ACTOR_POINTER);
                if e.vcall(actor, 0x1e4, &args![]).u32() != 0 {
                    let actor = e.global::<u32>(TRACKED_ACTOR_POINTER);
                    let object = e.vcall(actor, 0x1e4, &args![]).u32();
                    e.call(TRACKED_OBJECT_SET_VALUE, &args![object, 0.0f32]);
                    let actor = e.global::<u32>(TRACKED_ACTOR_POINTER);
                    let process = e.call(ACTOR_PROCESS, &args![actor]).u32();
                    e.vcall(process, 0x744, &args![0u32]);
                    let actor = e.global::<u32>(TRACKED_ACTOR_POINTER);
                    let process = e.call(ACTOR_PROCESS, &args![actor]).u32();
                    e.vcall(process, 0x74c, &args![0u32]);
                }
            }
        }
        let actor = e.global::<u32>(TRACKED_ACTOR_POINTER);
        let mut clear = e.mem.u8(actor + 0x7e) != 0;
        if !clear {
            let actor = e.global::<u32>(TRACKED_ACTOR_POINTER);
            if e.call(ACTOR_PROCESS, &args![actor]).u32() != 0 {
                let actor = e.global::<u32>(TRACKED_ACTOR_POINTER);
                let process = e.call(ACTOR_PROCESS, &args![actor]).u32();
                let actor = e.global::<u32>(TRACKED_ACTOR_POINTER);
                e.vcall(process, 0x44, &args![actor, 0u32, 2u32, 0u32, 1u32, 1u32]);
            } else {
                clear = true;
            }
        }
        if clear {
            let actor = e.global::<u32>(TRACKED_ACTOR_POINTER);
            e.mem.set_u8(actor + 0x7e, 0);
        }
        if keep_time == 0 {
            let setting = e
                .call(SETTING_FLOAT_POINTER, &args![TRACKED_TIME_SETTING])
                .u32();
            let time = e.mem.f32(setting);
            e.set_global(TRACKED_ACTOR_TIME, time);
        }
    }
    e.set_global(TRACKED_ACTOR_POINTER, 0u32);
}

// Translated from 008bc700 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Actor::IsInCombatWithActor` (Xbox PDB): the actor passes `00493bb0`,
/// has a combat target list (`+0x12c`) and the list contains `other` (the
/// search `00719b20` over the address of `other`).
pub fn actor_is_in_combat_with_actor(e: &mut Engine, this: Ptr<Actor>, other: u32) -> bool {
    if !e.call(ACTOR_STATE_TEST_493BB0, &args![this]).bool() {
        return false;
    }
    let list = e.get(this, Actor::pCurrentCombatTargetArray);
    if list.addr() == 0 {
        return false;
    }
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), other);
        e.call(LIST_FIND, &args![list, slot, 0u32, COMBAT_TARGET_COMPARE])
            .i32()
            != -1
    })
}

// Translated from 008bc750 (decompiled, FalloutNV.exe 1.4.0.525)
/// Adds the word and byte to the reference's extra data list (`0041d700`)
/// and then runs the actor's virtual `+0x48` with `0x80000000`.
pub fn fn_008bc750(e: &mut Engine, this: Ptr, value: u32, flag: u8) {
    let list = e.call(EXTRA_LIST_OF_REFERENCE, &args![this]).u32();
    e.call(EXTRA_LIST_ADD_0041D700, &args![list, value, flag]);
    e.vcall(this.addr(), 0x48, &args![0x8000_0000u32]);
}

// Translated from 008bc790 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Actor::AddFollower` (Xbox PDB): `ExtraDataList::AddFollower` on the
/// reference's extra data list, then the actor's virtual `+0x48` with
/// `0x800`.
pub fn actor_add_follower(e: &mut Engine, this: Ptr<Actor>, follower: u32) {
    let list = e.call(EXTRA_LIST_OF_REFERENCE, &args![this]).u32();
    e.call(EXTRA_LIST_ADD_00422480, &args![list, follower]);
    e.vcall(this.addr(), 0x48, &args![0x800u32]);
}

// Translated from 008bc7d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Calls `00422550` with the word on the reference's extra data list and
/// answers its value (the exe ends on that call and leaves its `EAX`; the
/// callers test `AL`).
pub fn fn_008bc7d0(e: &mut Engine, this: Ptr, value: u32) -> bool {
    let list = e.call(EXTRA_LIST_OF_REFERENCE, &args![this]).u32();
    e.call(EXTRA_LIST_REMOVE_00422550, &args![list, value])
        .bool()
}

/// True when the package's type number (`0041ca90`) is `1` or `7` (the
/// numbers `Actor::IsFollowing_ov2` and `CouldBeFollowing` accept).
fn package_type_is_follow(e: &mut Engine, package: u32) -> bool {
    e.call(PACKAGE_TYPE, &args![package]).u32() == 1
        || e.call(PACKAGE_TYPE, &args![package]).u32() == 7
}

// Translated from 008bc7f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Actor::IsFollowing_ov2` (Xbox PDB): the actor has a process, its current
/// package (`009344a0`) has type `1` or `7`, and the process's virtual
/// `+0x128` answers `other`.
pub fn actor_is_following_ov2(e: &mut Engine, this: Ptr<Actor>, other: u32) -> bool {
    let actor = this.addr();
    if process_of(e, actor) == 0 {
        return false;
    }
    let package = e.call(ACTOR_CURRENT_PACKAGE, &args![actor]).u32();
    package != 0
        && package_type_is_follow(e, package)
        && process_vcall(e, actor, 0x128, &args![]).u32() == other
}

// Translated from 008bc860 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Actor::CouldBeFollowing` (Xbox PDB): like `IsFollowing_ov2`, and also
/// true when the package of type `1` or `7` is the one the process's
/// virtual `+0x22c` or `+0x20c` returns, or the package extra of the
/// reference's extra data list.
pub fn actor_could_be_following(e: &mut Engine, this: Ptr<Actor>, other: u32) -> bool {
    let actor = this.addr();
    if process_of(e, actor) == 0 {
        return false;
    }
    let package = e.call(ACTOR_CURRENT_PACKAGE, &args![actor]).u32();
    if package != 0
        && package_type_is_follow(e, package)
        && process_vcall(e, actor, 0x128, &args![]).u32() == other
    {
        return true;
    }
    for slot in [0x22cu32, 0x20c] {
        let package = process_vcall(e, actor, slot, &args![]).u32();
        if package != 0 && package_type_is_follow(e, package) {
            return true;
        }
    }
    let list = e.call(EXTRA_LIST_OF_REFERENCE, &args![actor]).u32();
    let package = e.call(EXTRA_LIST_GET_PACKAGE, &args![list]).u32();
    package != 0 && package_type_is_follow(e, package)
}

// Translated from 008bc980 (decompiled, FalloutNV.exe 1.4.0.525)
/// When the actor's virtual `+0x428` returns an object, calls `0097f820` on
/// it with the four arguments.
pub fn fn_008bc980(e: &mut Engine, this: Ptr<Actor>, a: u32, b: u32, c: u32, d: u8) {
    let object = e.vcall(this.addr(), 0x428, &args![]).u32();
    if object != 0 {
        e.call(SPEECH_START, &args![object, a, b, c, d]);
    }
}

// Translated from 008bc9d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Tells whether the actor can be commanded by `other`: false when `other`
/// has bit 0 set in its `+0x12c` word (`008bca70`); with the actor's
/// virtual `+0x218` true the answer is true when `force` is set or
/// `0047bcf0(other)` holds; otherwise true when the commanding test
/// `005f65d0` of the base form's entry (`005f9b50`, `00500940`) holds for
/// `other`.
pub fn fn_008bc9d0(e: &mut Engine, this: Ptr<Actor>, other: Ptr<Actor>, force: u8) -> bool {
    if fn_008bca70(e, other) {
        return false;
    }
    if e.vcall(this.addr(), 0x218, &args![]).bool() {
        return force != 0 || e.call(OTHER_TEST_0047BCF0, &args![other]).bool();
    }
    let base = e.call(GET_BASE_FORM, &args![this]).u32();
    let form = e.call(BASE_COMMANDING_FORM, &args![base]).u32();
    if form == 0 {
        return false;
    }
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), other.addr());
        let entry = e.call(COMMANDING_ENTRY, &args![form]).u32();
        e.call(COMMANDING_TEST, &args![entry, slot]).bool()
    })
}

// Translated from 008bca70 (decompiled, FalloutNV.exe 1.4.0.525)
/// True when bit 0 of the word at `+0x12c` is set.
pub fn fn_008bca70(e: &mut Engine, this: Ptr<Actor>) -> bool {
    e.get(this, Actor::pCurrentCombatTargetArray).addr() & 1 != 0
}

// Translated from 008bca90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Actor::SetPlayerTeammate` (Xbox PDB): unless the actor is the player,
/// registers it with the player (`008abfa0`) when it becomes a teammate or
/// removes it (`008bcb00`) when it stops being one, and stores the flag at
/// `+0x18d`.
pub fn actor_set_player_teammate(e: &mut Engine, this: Ptr<Actor>, flag: u8) {
    let actor = this.addr();
    if actor == player_pointer(e) {
        return;
    }
    if flag != 0 && e.mem.u8(actor + 0x18d) == 0 {
        let player = player_pointer(e);
        e.call(PLAYER_ADD_TEAMMATE, &args![player, actor]);
    } else if flag == 0 && e.mem.u8(actor + 0x18d) != 0 {
        let player = player_pointer(e);
        fn_008bcb00(e, Ptr::new(player), actor);
    }
    e.mem.set_u8(actor + 0x18d, flag);
}

// Translated from 008bcb00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Decrements the player's teammate count at `+0xd68` and removes the
/// teammate (passed by address) from the list at `+0x5fc` (`00905330`).
pub fn fn_008bcb00(e: &mut Engine, this: Ptr, teammate: u32) {
    let count = e.mem.u32(this.addr() + 0xd68);
    e.mem.set_u32(this.addr() + 0xd68, count.wrapping_sub(1));
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), teammate);
        e.call(TEAMMATE_LIST_REMOVE, &args![this.addr() + 0x5fc, slot]);
    });
}

// Translated from 008bcb40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Walks the player crime list of the reference's extra data list: for every
/// node whose data word is not zero, walks its elements until one is empty
/// (`008256d0`) and calls `0047eb90(element, 0)` on each.
pub fn fn_008bcb40(e: &mut Engine, this: Ptr) {
    let extra = e.call(EXTRA_LIST_OF_REFERENCE, &args![this]).u32();
    let mut node = e.call(EXTRA_LIST_GET_CRIME_LIST, &args![extra]).u32();
    while node != 0 {
        let data = e.call(LIST_NODE_DATA_ADDRESS, &args![node]).u32();
        if e.mem.u32(data) == 0 {
            return;
        }
        let data = e.call(LIST_NODE_DATA_ADDRESS, &args![node]).u32();
        let holder = e.mem.u32(data);
        let mut element = e.call(CRIME_LIST_ITERATOR, &args![holder]).u32();
        while element != 0 && !e.call(LIST_NODE_IS_EMPTY, &args![element]).bool() {
            let data = e.call(LIST_NODE_DATA_ADDRESS, &args![element]).u32();
            let form = e.mem.u32(data);
            e.call(CRIME_ELEMENT_MARK, &args![form, 0u32]);
            element = e.call(LIST_NODE_NEXT, &args![element]).u32();
        }
        node = e.call(LIST_NODE_NEXT, &args![node]).u32();
    }
}

// Translated from 008bcbd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Adds `crime` to the player crime list of the reference's extra data list
/// with the `float` `006a7f50` computes from its `+0x34` word (taken
/// through `008bcc60`), calls `0047eb90(element, 1)` on the elements of
/// the crime up to the first empty one, and tells the process lists
/// (`009721f0`).
pub fn fn_008bcbd0(e: &mut Engine, this: Ptr, crime: Ptr) {
    let stamp = e.with_stack(4, |e, local| {
        let holder = fn_008bcc60(e, crime, local);
        e.call(CRIME_STAMP_VALUE, &args![holder]).f32()
    });
    let extra = e.call(EXTRA_LIST_OF_REFERENCE, &args![this]).u32();
    e.call(EXTRA_LIST_ADD_CRIME_LIST, &args![extra, crime, stamp]);
    let mut element = e.call(CRIME_LIST_ITERATOR, &args![crime]).u32();
    while element != 0 && !e.call(LIST_NODE_IS_EMPTY, &args![element]).bool() {
        let data = e.call(LIST_NODE_DATA_ADDRESS, &args![element]).u32();
        let form = e.mem.u32(data);
        e.call(CRIME_ELEMENT_MARK, &args![form, 1u32]);
        element = e.call(LIST_NODE_NEXT, &args![element]).u32();
    }
    e.call(PROCESS_LISTS_NOTIFY, &args![PROCESS_LISTS_OBJECT, crime]);
}

// Translated from 008bcc60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Copies the word at `+0x34` to `*out` and returns `out`.
pub fn fn_008bcc60(e: &mut Engine, this: Ptr, out: Ptr) -> Ptr {
    let value = e.mem.u32(this.addr() + 0x34);
    e.mem.set_u32(out.addr(), value);
    out
}

// Translated from 008bcc80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Actor::ShouldActorAvoidRadiation` (Xbox PDB): false when `00493bb0`
/// holds or the `float` setting at `011cdcc8` is below the actor value `0x14`;
/// otherwise false when the current package passes `0067a380` and fails
/// `0067ac50`; true in every other case.
pub fn actor_should_actor_avoid_radiation(e: &mut Engine, this: Ptr<Actor>) -> bool {
    let actor = this.addr();
    if e.call(ACTOR_STATE_TEST_493BB0, &args![actor]).bool() {
        return false;
    }
    let value = e
        .vcall(
            actor + ACTOR_VALUE_OWNER_OFFSET,
            OWNER_VSLOT_VALUE,
            &args![0x14u32],
        )
        .f64();
    let setting = e
        .call(SETTING_FLOAT_POINTER, &args![RADIATION_LIMIT_SETTING])
        .u32();
    if (e.mem.f32(setting) as f64) < value {
        return false;
    }
    let package = e.call(ACTOR_CURRENT_PACKAGE, &args![actor]).u32();
    if package != 0
        && e.call(PACKAGE_AVOID_TEST_A, &args![package]).bool()
        && !e.call(PACKAGE_AVOID_TEST_B, &args![package]).bool()
    {
        return false;
    }
    true
}

// Translated from 008bcd20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Actor::FadeIn` (Xbox PDB): for the actor of process type `0`, fades the
/// actor's process in (`HighProcess::FadeIn(actor, 0)`).
pub fn actor_fade_in(e: &mut Engine, this: Ptr<Actor>) {
    let actor = this.addr();
    if e.call(GET_PROCESS_TYPE, &args![actor]).u32() == 0 {
        let process = e.call(ACTOR_PROCESS, &args![actor]).u32();
        e.call(HIGH_PROCESS_FADE_IN, &args![process, actor, 0u32]);
    }
}

// Translated from 008bcd60 (decompiled, FalloutNV.exe 1.4.0.525)
/// The fade-out twin of `Actor::FadeIn`: `HighProcess::FadeOut(actor, 0, 0)`
/// (decompiler name) for an actor of process type `0`.
pub fn fn_008bcd60(e: &mut Engine, this: Ptr<Actor>) {
    let actor = this.addr();
    if e.call(GET_PROCESS_TYPE, &args![actor]).u32() == 0 {
        let process = e.call(ACTOR_PROCESS, &args![actor]).u32();
        e.call(HIGH_PROCESS_FADE_OUT, &args![process, actor, 0u32, 0u32]);
    }
}

// Translated from 008bcda0 (decompiled, FalloutNV.exe 1.4.0.525)
/// For an actor of process type `0` the `float` `008bcdd0(actor)` computes
/// from the process; `1.0` for every other actor.
pub fn fn_008bcda0(e: &mut Engine, this: Ptr<Actor>) -> f32 {
    let actor = this.addr();
    if e.call(GET_PROCESS_TYPE, &args![actor]).u32() == 0 {
        let process = e.call(ACTOR_PROCESS, &args![actor]).u32();
        e.call(FADE_VALUE, &args![process]).f32()
    } else {
        1.0
    }
}

// ---------------------------------------------------------------------------
// 008bcdd0 .. 008bec20 (third session of this part)

/// `bhkRagdollController` (the actor's `+0xac` object): reads its
/// `bInitRagdollAnim` flag (`0087ea20`).
const RAGDOLL_INIT_TEST: u32 = 0x0087_ea20;
/// `ECX` = the player: a test the ragdoll update makes (meaning not
/// confirmed).
const PLAYER_TEST_004EAF60: u32 = 0x004e_af60;
/// `ECX` = the player: a second test of the ragdoll update (meaning not
/// confirmed).
const PLAYER_TEST_00524D10: u32 = 0x0052_4d10;
/// `ECX` = an object: returns the address of its three-float position.
const OBJECT_POSITION: u32 = 0x0045_bb80;
/// Global pointers to the two objects whose position the ragdoll update uses
/// first (the first only while `004eaf60` holds for the player).
const POSITION_SOURCE_A: u32 = 0x011e_07d4;
const POSITION_SOURCE_B: u32 = 0x011e_07d0;
/// The vector the eye height is scaled along (third argument of
/// `004a3760`).
const EYE_DIRECTION: u32 = 0x011a_9484;
/// `NiPoint3` constructor from `ECX`'s side: cdecl `(out, scalar, vector)`,
/// writes `scalar * vector` to `out` and returns `out`.
const VECTOR_TIMES_SCALAR: u32 = 0x004a_3760;
/// `ECX` = a reference: returns `ECX + 0x30` (the reference's position
/// triple on PC; takes no stack words).
const REFERENCE_POSITION: u32 = 0x0043_6aa0;
/// `NiPoint3::operator+` (by its body): `ECX` = left, `(out, right)`; writes
/// the sum to `out` and returns `out`.
const VECTOR_ADD: u32 = 0x0043_9e90;
/// `NiPoint3::operator-` (by its body): `ECX` = left, `(out, right)`; writes
/// the difference to `out` and returns `out`.
const VECTOR_SUBTRACT: u32 = 0x0043_9ef0;
/// `NiPoint3::Length` (by its body): `ECX` = the vector, `float` in ST0.
const VECTOR_LENGTH: u32 = 0x0045_7990;
/// `NiAVObject::GetWorldBound` (decompiler name): `ECX` = the 3D object,
/// returns the address of the sphere (centre and radius, four floats).
const WORLD_BOUND: u32 = 0x0043_d450;
/// `ECX` = a bound sphere: its radius, `float` in ST0.
const BOUND_RADIUS: u32 = 0x0084_d030;
/// The `double` `1.25` the bound radius is scaled by.
const BOUND_RADIUS_FACTOR: u32 = 0x0102_1798;
/// Virtual `+0x22c` of the actor, called with `0` before the ragdoll
/// animation is enabled (meaning not confirmed).
const VSLOT_ACTOR_0X22C: u32 = 0x22c;
/// `ECX` = the ragdoll controller: a test (meaning not confirmed).
const RAGDOLL_TEST_00552490: u32 = 0x0055_2490;
/// `ECX` = the ragdoll controller: a second test (meaning not confirmed).
const RAGDOLL_TEST_008A3BD0: u32 = 0x008a_3bd0;
/// `bhkRagdollController::EnableRagdollAnim` (decompiler name): `ECX` = the
/// controller.
const RAGDOLL_ENABLE_ANIM: u32 = 0x00c7_a8d0;
/// `Actor::SetHavokWeapon` (decompiler name): `ECX` = the actor.
const ACTOR_SET_HAVOK_WEAPON: u32 = 0x008a_5eb0;
/// `ECX` = the actor: a flag word (`& 0xf` and `& 0x8500` are tested).
const ACTOR_FLAGS_008846E0: u32 = 0x0088_46e0;
/// Virtual `+0x1e4` of the actor: the 3D data whose animation the ragdoll
/// update inspects.
const VSLOT_ACTOR_ANIMATION: u32 = 0x1e4;
/// `ECX` = the animation data, one word (`1`): returns a sub-object (zero
/// when there is none).
const ANIMATION_SUB_OBJECT: u32 = 0x0049_1040;
/// `ECX` = the sub-object `00491040` returned: the object `005f4d60` and
/// `005f2420` read.
const ANIMATION_STATE_OBJECT: u32 = 0x0048_f7f0;
/// `ECX` = the state object: a test (meaning not confirmed).
const ANIMATION_STATE_TEST: u32 = 0x005f_4d60;
/// `ECX` = the state object: a number (`0xe3`, `0xe5` and `0xf1..=0xf4` make
/// the ragdoll update treat the actor as animating).
const ANIMATION_STATE_CODE: u32 = 0x005f_2420;
/// Virtual `+0x214` of the actor (a word, zero allows the ragdoll update).
const VSLOT_ACTOR_0X214: u32 = 0x214;
/// `ECX` = the ragdoll controller: a test (meaning not confirmed).
const RAGDOLL_TEST_00C78090: u32 = 0x00c7_8090;
/// `ECX` = the ragdoll controller, one flag.
const RAGDOLL_SET_005BA130: u32 = 0x005b_a130;
/// `ECX` = the ragdoll controller, one flag.
const RAGDOLL_SET_00C747D0: u32 = 0x00c7_47d0;
/// The object the ragdoll update asks for its `+0x08` word (`ENTRY_OBJECT`);
/// compared with `4`.
const RAGDOLL_MODE_OBJECT: u32 = 0x011f_2250;
/// `ECX` = the ragdoll controller: taken when the mode word is `4`.
const RAGDOLL_MODE_FOUR: u32 = 0x00c7_5910;
/// `ECX` = the ragdoll controller: taken for any other mode word.
const RAGDOLL_MODE_OTHER: u32 = 0x00c7_59f0;
/// `Actor::GetFaceAnimationData` (decompiler name): `ECX` = the actor.
const ACTOR_FACE_ANIMATION_DATA: u32 = 0x008a_dcb0;
/// Virtual `+0x9c` of the face animation data: writes four words to the
/// address it is given.
const VSLOT_FACE_DATA_VALUE: u32 = 0x9c;
/// The four-word default `fn_008bd5e0` receives without face data.
const FACE_DATA_DEFAULT: u32 = 0x011a_9ea4;
/// Virtual `+0x4bc` of the actor's process: a kind number (`5` and `10` are
/// tested).
const VSLOT_PROCESS_KIND: u32 = 0x4bc;
/// `ECX` = the ragdoll controller, one flag.
const RAGDOLL_SET_008A3BF0: u32 = 0x008a_3bf0;
/// `ECX` = the process: a test (meaning not confirmed).
const PROCESS_TEST_0045CD60: u32 = 0x0045_cd60;
/// `ECX` = the process: returns the actor mover (zero when none).
const PROCESS_ACTOR_MOVER: u32 = 0x0089_d620;
/// `ECX` = the actor mover: returns a word the update compares with `1`
/// (the engine map names it `ActorMover::GetPreferredMoveMode`, but the body
/// is folded with `TES::pInteriorCell`'s getter).
const MOVER_MODE_005F36F0: u32 = 0x005f_36f0;
/// `ECX` = the ragdoll controller: a test (meaning not confirmed).
const RAGDOLL_TEST_0089D690: u32 = 0x0089_d690;
/// `bhkRagdollController::SetRagdollFeedbackActive` (decompiler name):
/// `ECX` = the controller, one flag.
const RAGDOLL_SET_FEEDBACK_ACTIVE: u32 = 0x00c7_b6a0;
/// `ECX` = the ragdoll controller: a test (meaning not confirmed).
const RAGDOLL_TEST_00888A50: u32 = 0x0088_8a50;
/// `ECX` = the ragdoll controller, one flag.
const RAGDOLL_SET_00C75580: u32 = 0x00c7_5580;

/// `ECX` = the actor mover's process, the float read by `008bd550`: the
/// game setting `0x01267c6c`.
const RAGDOLL_RANGE_SETTING: u32 = 0x0126_7c6c;

/// `ECX` = a process `+0xc0` object, `(object, value)`: cdecl copy of a
/// four-word value (the call `008bd5e0` makes).
const FOUR_WORD_ASSIGN: u32 = 0x0056_1500;
/// `ECX` = a 3D node: returns the child count of a node (its `+4`).
const CHILD_COUNT: u32 = 0x0043_b480;
/// `ECX` = the child array, the index: returns a child.
const CHILD_AT: u32 = 0x0043_b4a0;
/// Returns `3` (no arguments): the property type `NiAVObject::GetProperty`
/// is asked for.
const PROPERTY_TYPE_SHADER: u32 = 0x0043_8220;
/// `NiAVObject::GetProperty` (decompiler name): `ECX` = the node, the
/// property type.
const NODE_GET_PROPERTY: u32 = 0x00a5_9d30;
/// `BSShaderProperty::SetAlpha` (decompiler name): `ECX` = the property, a
/// `float`.
const SHADER_SET_ALPHA: u32 = 0x00ba_8ab0;
/// `ECX` = the property, a `float`: the second request `FadeSkins` makes
/// with the alpha.
const SHADER_SET_FADE: u32 = 0x0081_9a70;
/// Virtual `+0x18` of a node: its skinned geometry object, if any.
const VSLOT_NODE_SKIN: u32 = 0x18;
/// Virtual `+0x34` of the skin object (zero lets the fade go on).
const VSLOT_SKIN_0X34: u32 = 0x34;
/// Virtual `+0x0c` of a node: its child array (zero when it has none).
const VSLOT_NODE_CHILDREN: u32 = 0x0c;

/// Virtual `+0x24` of `base form + 0x30` (a test; a true answer ends
/// `008bd700` with true).
const VSLOT_FORM_PART_0X24: u32 = 0x24;
/// Actor virtual `+0x21c`.
const VSLOT_ACTOR_0X21C: u32 = 0x21c;
/// `ECX` = the actor: a test, also the answer when the actor virtual
/// `+0x21c` holds.
const ACTOR_TEST_008ACE90: u32 = 0x008a_ce90;
/// Actor virtual `+0x1a0` (called with `0`).
const VSLOT_ACTOR_0X1A0: u32 = 0x1a0;

/// `ECX` = the reference: returns `ECX + 0x24`, the rotation triple.
const ROTATION_POINTER: u32 = 0x0043_0830;
/// Actor virtual `+0x100`, a test `GetHeading` makes.
const VSLOT_ACTOR_0X100: u32 = 0x100;
/// `TESActorBase::IsImmobile` (decompiler name): `ECX` = the base form.
const BASE_FORM_IS_IMMOBILE: u32 = 0x005f_0c80;
/// `ClampAngle` (decompiler name), cdecl, one `float`, `float` in ST0.
const CLAMP_ANGLE: u32 = 0x004b_1480;

/// Virtual `+0x1f4` of the actor: the address of its eye position triple.
const VSLOT_ACTOR_POSITION: u32 = 0x1f4;
/// Virtual `+0x2bc` of the actor (one word, `0`): a heading, `float` in ST0.
const VSLOT_ACTOR_HEADING: u32 = 0x2bc;
/// Virtual `+0x1d0` of a reference: its 3D data.
const VSLOT_REFERENCE_3D: u32 = 0x1d0;
/// The `float` game setting added to the eye height (`ECX` of `00403e20`).
const VATS_HEIGHT_SETTING: u32 = 0x011d_1200;
/// The `float` game settings `GetVATSTargetVisible` steps and bounds the
/// search by (`ECX` of `00403e20`).
const VATS_STEP_COUNT_SETTING: u32 = 0x011d_0850;
const VATS_STEP_LENGTH_SETTING: u32 = 0x011d_09e8;
/// The `float` length of the area ray (10000.0).
const VATS_RAY_LENGTH: u32 = 0x0102_2958;
/// The `float` answer when nothing is hit (`FLT_MAX`).
const VATS_NOTHING_HIT: u32 = 0x0101_6970;
/// The `double` `0.85` the visibility ray fraction is compared with.
const VATS_FRACTION_LIMIT: u32 = 0x0101_9de8;
/// The `double` `256.0` the visibility ray distance is compared with.
const VATS_DISTANCE_LIMIT: u32 = 0x0102_31d8;
/// `ECX` = a matrix: sets the rotation of the heading about the up axis
/// (takes the angle).
const MATRIX_SET_HEADING: u32 = 0x004a_0c90;
/// `ECX` = a matrix, `(column, out)`: copies a column to `out`.
const MATRIX_GET_COLUMN: u32 = 0x0043_9f50;
/// `ECX` = a vector: normalises it in place.
const VECTOR_NORMALIZE: u32 = 0x004a_0c10;
/// `ECX` = the ray input: resets it.
const RAY_RESET: u32 = 0x004a_3c20;
/// `ECX` = the ray input, the start point address.
const RAY_SET_FROM: u32 = 0x004a_3da0;
/// `ECX` = the ray input, the end point address.
const RAY_SET_TO: u32 = 0x004a_3eb0;
/// `ECX` = the collision filter, one word: stores it.
const FILTER_CONSTRUCTOR: u32 = 0x008c_71b0;
/// `ECX` = the collision filter, the layer (low seven bits).
const FILTER_SET_LAYER: u32 = 0x004a_39f0;
/// `ECX` = the actor, an out word: returns the out word holding the actor's
/// collision filter.
const ACTOR_COLLISION_FILTER: u32 = 0x0093_1ed0;
/// `ECX` = that word: its group (the high half).
const FILTER_GROUP_OF: u32 = 0x004a_3a20;
/// `ECX` = the collision filter, the group (stored in the high half).
const FILTER_SET_GROUP: u32 = 0x0059_ce80;
/// `ECX` = the ray input, one word: stores it at `+0x24`.
const RAY_SET_FILTER: u32 = 0x004a_3f70;
/// `hkpAllRayHitCollector::hkpAllRayHitCollector` (Xbox PDB).
const COLLECTOR_CONSTRUCTOR: u32 = 0x004a_3a70;
/// `hkpAllRayHitCollector::~hkpAllRayHitCollector` (Xbox PDB).
const COLLECTOR_DESTRUCTOR: u32 = 0x004a_3bc0;
/// `ECX` = the ray input, the collector.
const RAY_SET_COLLECTOR: u32 = 0x004a_3fb0;
/// `ECX` = the player: its world.
const PLAYER_WORLD: u32 = 0x008d_6f30;
/// `ECX` = the world: its physics world (`bhkWorld`).
const WORLD_PHYSICS: u32 = 0x0045_43c0;
/// Virtual `+0xc8` of the physics world: casts the ray input.
const VSLOT_WORLD_CAST_RAY: u32 = 0xc8;
/// `ECX` = the ray input: returns the collector (`+0xa8`).
const RAY_COLLECTOR: u32 = 0x008c_dd90;
/// `ECX` = the collector: returns the hit array (`+0x10`).
const COLLECTOR_HITS: u32 = 0x0046_0140;
/// `ECX` = the hit array: the number of hits (`+4`).
const HIT_COUNT: u32 = 0x0072_6070;
/// `ECX` = the hit array, the index: the address of the hit (`0x60` bytes
/// each).
const HIT_AT: u32 = 0x004a_46b0;
/// `ECX` = a hit result, one word (the hit): copy-constructs it.
const HIT_RESULT_CONSTRUCTOR: u32 = 0x0069_6c30;
/// `GetAVObjectForCollidable` (decompiler name), cdecl, the collidable.
const AV_OBJECT_FOR_COLLIDABLE: u32 = 0x004b_5820;
/// cdecl, the collidable: a second object of it (zero when there is none).
const COLLIDABLE_OBJECT_004B59F0: u32 = 0x004b_59f0;
/// `NiColorA` constructor: `ECX` = the colour, four floats.
const COLOUR_CONSTRUCTOR: u32 = 0x0041_4430;
/// cdecl `(from, colour, to, colour, 1)`: builds a debug line object.
const DEBUG_LINE_OBJECT: u32 = 0x004b_3890;
/// `TES::AddTempDebugObject` (decompiler name): `ECX` = the `TES` pointer's
/// value, `(object, float seconds)`.
const TES_ADD_TEMP_DEBUG_OBJECT: u32 = 0x0045_8e20;
/// The `TES *` global.
const TES_POINTER: u32 = 0x011d_ea10;
/// The one-byte setting that enables the VATS debug lines.
const VATS_DEBUG_SETTING: u32 = 0x011d_f868;
/// The `float` seconds the debug lines stay (10.0).
const VATS_DEBUG_SECONDS: u32 = 0x0101_7b78;

/// `ECX` = the cached-values block, a mask: true when none of the mask bits
/// is set in its `iFlags` (`00884e90`).
const CACHED_VALUES_NONE_OF: u32 = 0x0088_4e90;
/// `0.0` as a `double`.
const DOUBLE_ZERO: u32 = 0x0101_2060;
/// Virtual `+0x184` of the process: its cached fire node.
const VSLOT_PROCESS_FIRE_NODE: u32 = 0x184;
/// Virtual `+0x188` of the process: stores the fire node.
const VSLOT_PROCESS_SET_FIRE_NODE: u32 = 0x188;
/// `PlayerCharacter::GetCurrent3D` (decompiler name): `ECX` = the player.
const PLAYER_CURRENT_3D: u32 = 0x0095_0be0;
/// `GetCurrentWeapon` (decompiler name): `ECX` = the actor.
const ACTOR_CURRENT_WEAPON: u32 = 0x008a_1710;
/// `TESObjectWEAP::GetFireNode` (decompiler name): `ECX` = the weapon, the
/// 3D data.
const WEAPON_FIRE_NODE: u32 = 0x0052_5700;
/// cdecl `(3D data, name)`: finds a node by name.
const FIND_NODE_BY_NAME: u32 = 0x004a_ae30;
/// The text `ProjectileNode`.
const PROJECTILE_NODE_NAME: u32 = 0x0102_cb10;

/// `GameSetting` `float` read through `00403e20`.
fn setting_float(e: &mut Engine, setting: u32) -> f32 {
    let pointer = e.call(SETTING_FLOAT_POINTER, &args![setting]).u32();
    e.mem.f32(pointer)
}

/// Copies `count` words.
fn copy_words(e: &mut Engine, to: u32, from: u32, count: u32) {
    for i in 0..count {
        let word = e.mem.u32(from + 4 * i);
        e.mem.set_u32(to + 4 * i, word);
    }
}

// Translated from 008bcdd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Returns the `float` at `+0x3ec` of the process (the fade value
/// `008bcda0` returns for the actor).
pub fn fn_008bcdd0(e: &mut Engine, this: Ptr) -> f32 {
    e.mem.f32(this.addr() + 0x3ec)
}

// Translated from 008bcdf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The ragdoll update of an actor that has a `bhkRagdollController`
/// (`Actor + 0xac`) in its initialised state; with `update` zero it only
/// answers false. Otherwise it sets `bFootIKInRange` (Xbox PDB: the 3D
/// bound of the actor's scene is closer to the player's eye than the
/// `0x01267c6c` setting), switches the controller's ragdoll animation and
/// ease-out flags, hands the controller the animation and face state of the
/// actor, and answers whether the ragdoll is wanted. The second stack word is
/// never read.
pub fn fn_008bcdf0(e: &mut Engine, this: Ptr<Actor>, update: u8, _unused_1: u32) -> bool {
    let actor = this.addr();
    let ragdoll = e.get(this, Actor::pRagdollController).addr();
    if ragdoll == 0 || !e.call(RAGDOLL_INIT_TEST, &args![ragdoll]).bool() {
        return false;
    }
    let player = player_pointer(e);
    let hidden_player = actor == player && !e.call(PLAYER_TEST_004EAF60, &args![player]).bool();
    if update == 0 {
        return false;
    }
    let scene = e.vcall(actor, VSLOT_REFERENCE_3D, &args![]).u32();
    let player = player_pointer(e);
    let mut wanted = !(actor == player && !e.call(PLAYER_TEST_00524D10, &args![player]).bool());
    e.set(this, Actor::bFootIKInRange, false);
    if scene != 0 {
        let in_range = e.with_stack(0x60, |e, frame| {
            let position = frame.addr();
            let bound = position + 0x10;
            let scaled = position + 0x20;
            let eye = position + 0x30;
            let difference = position + 0x40;
            e.call(LIST_NODE_DATA_ADDRESS, &args![position]);
            let player = player_pointer(e);
            let source_a = e.global::<u32>(POSITION_SOURCE_A);
            let source_b = e.global::<u32>(POSITION_SOURCE_B);
            if e.call(PLAYER_TEST_004EAF60, &args![player]).bool() && source_a != 0 {
                let from = e.call(OBJECT_POSITION, &args![source_a]).u32();
                copy_words(e, position, from, 3);
            } else if source_b != 0 {
                let from = e.call(OBJECT_POSITION, &args![source_b]).u32();
                copy_words(e, position, from, 3);
            } else {
                let player = player_pointer(e);
                let height = actor_get_eye_level(e, Ptr::new(player));
                let offset = e
                    .call(VECTOR_TIMES_SCALAR, &args![scaled, height, EYE_DIRECTION])
                    .u32();
                let player_position = e.call(REFERENCE_POSITION, &args![player]).u32();
                let sum = e
                    .call(VECTOR_ADD, &args![player_position, eye, offset])
                    .u32();
                copy_words(e, position, sum, 3);
            }
            let from = e.call(WORLD_BOUND, &args![scene]).u32();
            copy_words(e, bound, from, 4);
            let anchor = e.call(LIST_NODE_DATA_ADDRESS, &args![bound]).u32();
            let offset = e
                .call(VECTOR_SUBTRACT, &args![position, difference, anchor])
                .u32();
            let distance = e.call(VECTOR_LENGTH, &args![offset]).f64();
            let radius = e.call(BOUND_RADIUS, &args![bound]).f64();
            let factor = e.global::<f64>(BOUND_RADIUS_FACTOR);
            let margin = (distance - radius * factor) as f32;
            let limit = fn_008bd550(e);
            margin < limit
        });
        e.set(this, Actor::bFootIKInRange, in_range);
    }
    let in_range = e.get(this, Actor::bFootIKInRange);
    if !(wanted && in_range) {
        if !hidden_player
            && e.call(RAGDOLL_TEST_00552490, &args![ragdoll]).bool()
            && fn_008bd5c0(e, Ptr::new(ragdoll)) == 0
        {
            fn_008bd570(e, Ptr::new(ragdoll), 1);
        }
    } else if !e.vcall(actor, VSLOT_ACTOR_0X22C, &args![0u32]).bool()
        && (!e.call(RAGDOLL_TEST_00552490, &args![ragdoll]).bool()
            || e.call(RAGDOLL_TEST_008A3BD0, &args![ragdoll]).bool())
    {
        let active = e.call(RAGDOLL_TEST_00552490, &args![ragdoll]).bool();
        e.call(RAGDOLL_ENABLE_ANIM, &args![ragdoll]);
        if !active {
            e.call(ACTOR_SET_HAVOK_WEAPON, &args![actor]);
        }
    }
    wanted =
        wanted && (e.get(this, Actor::bFootIKInRange) || fn_008bd5c0(e, Ptr::new(ragdoll)) != 0);
    if !wanted {
        return false;
    }

    let flags = e.call(ACTOR_FLAGS_008846E0, &args![actor]).u32();
    let flag_mask = 0x8500u32;
    let mut animating = false;
    let animation = e.vcall(actor, VSLOT_ACTOR_ANIMATION, &args![]).u32();
    if animation != 0 && e.call(ANIMATION_SUB_OBJECT, &args![animation, 1u32]).u32() != 0 {
        let sub = e.call(ANIMATION_SUB_OBJECT, &args![animation, 1u32]).u32();
        let state = e.call(ANIMATION_STATE_OBJECT, &args![sub]).u32();
        if e.call(ANIMATION_STATE_TEST, &args![state]).bool() {
            animating = true;
        } else {
            let sub = e.call(ANIMATION_SUB_OBJECT, &args![animation, 1u32]).u32();
            let state = e.call(ANIMATION_STATE_OBJECT, &args![sub]).u32();
            let code = e.call(ANIMATION_STATE_CODE, &args![state]).u32();
            if matches!(code, 0xe3 | 0xe5 | 0xf1..=0xf4) {
                animating = true;
            }
        }
    }
    let enabled = e.vcall(actor, VSLOT_ACTOR_0X214, &args![]).u32() == 0
        && !animating
        && !e.call(RAGDOLL_TEST_00C78090, &args![ragdoll]).bool()
        && !e.vcall(actor, VSLOT_ACTOR_0X364, &args![]).bool();
    let masked = flags & 0xf != 0;
    let forced = flags & flag_mask != 0 || {
        let player = player_pointer(e);
        actor == player && !e.call(PLAYER_TEST_00524D10, &args![player]).bool()
    };
    let unmasked = forced || !masked;
    e.call(RAGDOLL_SET_005BA130, &args![ragdoll, enabled as u32]);
    let in_range = e.get(this, Actor::bFootIKInRange);
    e.call(
        RAGDOLL_SET_00C747D0,
        &args![ragdoll, (enabled && in_range && unmasked) as u32],
    );
    fn_008bd610(e, Ptr::new(ragdoll), !masked as u8);
    let mode = e.call(ENTRY_OBJECT, &args![RAGDOLL_MODE_OBJECT]).u32();
    if mode == 4 {
        e.call(RAGDOLL_MODE_FOUR, &args![ragdoll]);
    } else {
        e.call(RAGDOLL_MODE_OTHER, &args![ragdoll]);
    }
    let face = e.call(ACTOR_FACE_ANIMATION_DATA, &args![actor]).u32();
    if face != 0 {
        e.with_stack(0x10, |e, value| {
            e.call(LIST_NODE_DATA_ADDRESS, &args![value]);
            e.vcall(face, VSLOT_FACE_DATA_VALUE, &args![value]);
            fn_008bd5e0(e, Ptr::new(ragdoll), value.addr());
        });
    } else {
        fn_008bd5e0(e, Ptr::new(ragdoll), FACE_DATA_DEFAULT);
    }
    let process = e.call(ACTOR_PROCESS, &args![actor]).u32();
    let kind = e.vcall(process, VSLOT_PROCESS_KIND, &args![]).i32();
    if kind == 10 || kind == 5 {
        e.call(RAGDOLL_SET_008A3BF0, &args![ragdoll, 1u32]);
    }
    let process = e.call(ACTOR_PROCESS, &args![actor]).u32();
    if process != 0 && e.call(PROCESS_TEST_0045CD60, &args![process]).u32() == 0 {
        let mover = e.call(PROCESS_ACTOR_MOVER, &args![process]).u32();
        if mover != 0 && e.call(MOVER_MODE_005F36F0, &args![mover]).i32() > 1 {
            if e.call(RAGDOLL_TEST_0089D690, &args![ragdoll]).bool() {
                e.call(RAGDOLL_SET_FEEDBACK_ACTIVE, &args![ragdoll, 0u32]);
            }
            if e.call(RAGDOLL_TEST_00888A50, &args![ragdoll]).bool() {
                e.call(RAGDOLL_SET_00C75580, &args![ragdoll, 0u32]);
            }
        }
    }
    wanted
}

// Translated from 008bd550 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `float` game setting at `0x01267c6c` (read through `00403e20`).
pub fn fn_008bd550(e: &mut Engine) -> f32 {
    setting_float(e, RAGDOLL_RANGE_SETTING)
}

// Translated from 008bd570 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores `value` in the five flag bytes of the ragdoll controller at
/// `+0x221`, `+0x1ed`, `+0x1bd`, `+0xb2` and `+0x43` (PC offsets; `+0x43` is
/// `bEaseOutRagdollAnim` in the Xbox PDB, the others are not confirmed).
pub fn fn_008bd570(e: &mut Engine, this: Ptr, value: u8) {
    for offset in [0x221, 0x1ed, 0x1bd, 0xb2, 0x43] {
        e.mem.set_u8(this.addr() + offset, value);
    }
}

// Translated from 008bd5c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Returns the byte at `+0x43` of the ragdoll controller
/// (`bEaseOutRagdollAnim` in the Xbox PDB).
pub fn fn_008bd5c0(e: &mut Engine, this: Ptr) -> u8 {
    e.mem.u8(this.addr() + 0x43)
}

// Translated from 008bd5e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Hands `value` to the four-word object at `+0xc0` of the ragdoll
/// controller (`00561500`).
pub fn fn_008bd5e0(e: &mut Engine, this: Ptr, value: u32) {
    e.call(FOUR_WORD_ASSIGN, &args![this.addr() + 0xc0, value]);
}

// Translated from 008bd610 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores `value` in the byte at `+0x243` of the ragdoll controller.
pub fn fn_008bd610(e: &mut Engine, this: Ptr, value: u8) {
    e.mem.set_u8(this.addr() + 0x243, value);
}

// Translated from 008bd630 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Actor::FadeSkins` (Xbox PDB): for a node whose skin object (virtual
/// `+0x18`) answers zero to its virtual `+0x34`, sets the alpha of the
/// node's property of the type `00438220` returns (`3`) to `alpha` (twice,
/// through `00ba8ab0` and `00819a70`); then does the same for every child of
/// the node, recursively.
#[allow(clippy::only_used_in_recursion)]
pub fn actor_fade_skins(e: &mut Engine, this: Ptr<Actor>, node: Ptr, alpha: f32) {
    let skin = e.vcall(node.addr(), VSLOT_NODE_SKIN, &args![]).u32();
    if skin != 0 && e.vcall(skin, VSLOT_SKIN_0X34, &args![]).u32() == 0 {
        let kind = e.call(PROPERTY_TYPE_SHADER, &args![]).u32();
        let property = e.call(NODE_GET_PROPERTY, &args![skin, kind]).u32();
        if property != 0 {
            e.call(SHADER_SET_ALPHA, &args![property, alpha]);
            e.call(SHADER_SET_FADE, &args![property, alpha]);
        }
    }
    let children = e.vcall(node.addr(), VSLOT_NODE_CHILDREN, &args![]).u32();
    if children != 0 {
        let mut index = 0u32;
        while index < e.call(CHILD_COUNT, &args![children]).u32() {
            let child = e.call(CHILD_AT, &args![children, index]).u32();
            if child != 0 {
                actor_fade_skins(e, this, Ptr::new(child), alpha);
            }
            index += 1;
        }
    }
}

// Translated from 008bd700 (decompiled, FalloutNV.exe 1.4.0.525)
/// A virtual of the subobject at `Actor + 0x94` (`this - 0x94` is the actor):
/// true when the base form part at `+0x30` answers true to its virtual
/// `+0x24`, or when `008ace90` holds, or when the actor's virtual `+0x1a0`
/// (with `0`) holds; when the actor's virtual `+0x21c` holds, the answer is
/// `008ace90`'s.
pub fn fn_008bd700(e: &mut Engine, this: Ptr) -> u8 {
    let actor = this.addr().wrapping_sub(0x94);
    let base = e.call(GET_BASE_FORM, &args![actor]).u32();
    if e.vcall(base + 0x30, VSLOT_FORM_PART_0X24, &args![]).bool() {
        return 1;
    }
    if e.vcall(actor, VSLOT_ACTOR_0X21C, &args![]).bool() {
        return e.call(ACTOR_TEST_008ACE90, &args![actor]).u8();
    }
    if e.call(ACTOR_TEST_008ACE90, &args![actor]).bool() {
        return 1;
    }
    e.vcall(actor, VSLOT_ACTOR_0X1A0, &args![0u32]).bool() as u8
}

// Translated from 008bd7b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Actor::GetHeading` (Xbox PDB): the `float` at `+8` of the rotation
/// triple (`+0x24`); unless `skip_adjust` is set, for an actor whose virtual
/// `+0x100` holds and whose base form is immobile (`005f0c80`) the angle at
/// `+4` is added and the sum is clamped (`ClampAngle`).
pub fn actor_get_heading(e: &mut Engine, this: Ptr<Actor>, skip_adjust: u8) -> f32 {
    let actor = this.addr();
    let rotation = e.call(ROTATION_POINTER, &args![actor]).u32();
    let mut heading = e.mem.f32(rotation + 8);
    if skip_adjust == 0 && e.vcall(actor, VSLOT_ACTOR_0X100, &args![]).bool() {
        let base = e.call(GET_BASE_FORM, &args![actor]).u32();
        if e.call(BASE_FORM_IS_IMMOBILE, &args![base]).bool() {
            let rotation = e.call(ROTATION_POINTER, &args![actor]).u32();
            let sum = (heading as f64 + e.mem.f32(rotation + 4) as f64) as f32;
            heading = e.call(CLAMP_ANGLE, &args![sum]).f32();
        }
    }
    heading
}

/// The pieces of the ray-cast frame `Actor::GetVATSAreaFree` and
/// `GetVATSTargetVisible` build on their stack.
#[derive(Clone, Copy)]
struct RayFrame {
    ray: u32,
    filter: u32,
    scratch: u32,
    collector: u32,
}

/// The ray set-up both VATS functions share: resets the ray input, sets its
/// start and end, builds the collision filter of the actor (layer `0x23`,
/// the actor's own group), stores it in the ray input and attaches a fresh
/// `hkpAllRayHitCollector`.
fn prepare_ray(e: &mut Engine, actor: u32, frame: RayFrame, from: u32, to: u32) {
    e.call(RAY_RESET, &args![frame.ray]);
    e.call(RAY_SET_FROM, &args![frame.ray, from]);
    e.call(RAY_SET_TO, &args![frame.ray, to]);
    e.call(FILTER_CONSTRUCTOR, &args![frame.filter, 0u32]);
    e.call(FILTER_SET_LAYER, &args![frame.filter, 0x23u32]);
    let word = e
        .call(ACTOR_COLLISION_FILTER, &args![actor, frame.scratch])
        .u32();
    let group = e.call(FILTER_GROUP_OF, &args![word]).u32();
    e.call(FILTER_SET_GROUP, &args![frame.filter, group]);
    let filter = e.mem.u32(frame.filter);
    e.call(RAY_SET_FILTER, &args![frame.ray, filter]);
    e.call(COLLECTOR_CONSTRUCTOR, &args![frame.collector]);
    e.call(RAY_SET_COLLECTOR, &args![frame.ray, frame.collector]);
}

/// Casts the prepared ray through the player's world.
fn cast_ray(e: &mut Engine, ray: u32) {
    let player = player_pointer(e);
    let world = e.call(PLAYER_WORLD, &args![player]).u32();
    let physics = e.call(WORLD_PHYSICS, &args![world]).u32();
    e.vcall(physics, VSLOT_WORLD_CAST_RAY, &args![ray]);
}

/// The number of hits the collector of the ray input holds.
fn ray_hit_count(e: &mut Engine, ray: u32) -> i32 {
    let collector = e.call(RAY_COLLECTOR, &args![ray]).u32();
    let hits = e.call(COLLECTOR_HITS, &args![collector]).u32();
    e.call(HIT_COUNT, &args![hits]).i32()
}

/// What the ray's hit number `index` struck: copies it into `result`, then
/// returns the 3D object of its collidable (`+0x50`), the reference found for
/// that 3D object and the collidable's second object.
fn ray_hit_objects(e: &mut Engine, ray: u32, index: i32, result: u32) -> (u32, u32, u32) {
    let collector = e.call(RAY_COLLECTOR, &args![ray]).u32();
    let hits = e.call(COLLECTOR_HITS, &args![collector]).u32();
    let hit = e.call(HIT_AT, &args![hits, index]).u32();
    e.call(HIT_RESULT_CONSTRUCTOR, &args![result, hit]);
    let collidable = e.mem.u32(result + 0x50);
    let object = e.call(AV_OBJECT_FOR_COLLIDABLE, &args![collidable]).u32();
    let reference = e.call(FIND_REFERENCE_FOR_3D, &args![object]).u32();
    let collidable = e.mem.u32(result + 0x50);
    let second = e.call(COLLIDABLE_OBJECT_004B59F0, &args![collidable]).u32();
    (object, reference, second)
}

// Translated from 008bd830 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Actor::GetVATSAreaFree` (Xbox PDB): casts a ray from the actor's eye
/// position (virtual `+0x1f4`, raised by the `0x011d1200` setting) along its
/// heading (virtual `+0x2bc` plus `heading_offset`) for the length
/// `0x01022958` and returns the distance to the first hit that is not the
/// actor itself, or `FLT_MAX` (`0x01016970`) when nothing is hit. With the
/// debug setting `0x011df868` on, it adds a debug line of that length to
/// `TES` for ten seconds. The first stack word is never read. (The
/// structured-exception frame and the stack cookie are not translated.)
pub fn actor_get_vats_area_free(
    e: &mut Engine,
    this: Ptr<Actor>,
    _unused_1: u32,
    heading_offset: f32,
) -> f32 {
    let actor = this.addr();
    e.with_stack(0x540, |e, block| {
        // The frame keeps the original stack offsets, counted down from here.
        let top = block.addr() + 0x530;
        let start = top - 0x24;
        let direction = top - 0x30;
        let matrix = top - 0x54;
        let scaled_ray = top - 0x68;
        let end = top - 0x74;
        let frame = RayFrame {
            ray: top - 0x130,
            filter: top - 0x134,
            scratch: top - 0x138,
            collector: top - 0x460,
        };
        let result = top - 0x4d0;
        let colour = top - 0x4ec;
        let debug_scaled = top - 0x4f8;
        let debug_end = top - 0x504;

        let position = e.vcall(actor, VSLOT_ACTOR_POSITION, &args![]).u32();
        copy_words(e, start, position, 3);
        let height = setting_float(e, VATS_HEIGHT_SETTING);
        let raised = (e.mem.f32(start + 8) as f64 + height as f64) as f32;
        e.mem.set_f32(start + 8, raised);
        e.call(LIST_NODE_DATA_ADDRESS, &args![direction]);
        e.call(LIST_NODE_DATA_ADDRESS, &args![matrix]);
        let heading = e.vcall(actor, VSLOT_ACTOR_HEADING, &args![0u32]).f64();
        let angle = (heading + heading_offset as f64) as f32;
        e.call(MATRIX_SET_HEADING, &args![matrix, angle]);
        e.call(MATRIX_GET_COLUMN, &args![matrix, 1u32, direction]);
        e.call(VECTOR_NORMALIZE, &args![direction]);
        let length = e.global::<f32>(VATS_RAY_LENGTH);
        let long_ray = e
            .call(VECTOR_TIMES_SCALAR, &args![scaled_ray, length, direction])
            .u32();
        e.call(VECTOR_ADD, &args![start, end, long_ray]);
        let mut found = false;
        let mut distance = e.global::<f32>(VATS_NOTHING_HIT);
        prepare_ray(e, actor, frame, start, end);
        cast_ray(e, frame.ray);
        let mut index = 0;
        while !found && index < ray_hit_count(e, frame.ray) {
            let (object, reference, second) = ray_hit_objects(e, frame.ray, index, result);
            if object != 0 && second != 0 && reference != actor {
                found = true;
                // The hit result's fraction (Xbox PDB hkpRootCdPoint-style
                // distance) at +0x10.
                distance = (e.mem.f32(result + 0x10) as f64 * length as f64) as f32;
            }
            index += 1;
        }
        let setting = e
            .call(SETTING_BYTE_POINTER, &args![VATS_DEBUG_SETTING])
            .u32();
        if e.mem.u8(setting) != 0 {
            e.call(
                COLOUR_CONSTRUCTOR,
                &args![colour, 1.0f32, 1.0f32, 0.0f32, 1.0f32],
            );
            let along = e
                .call(
                    VECTOR_TIMES_SCALAR,
                    &args![debug_scaled, distance, direction],
                )
                .u32();
            let to = e.call(VECTOR_ADD, &args![start, debug_end, along]).u32();
            let line = e
                .call(DEBUG_LINE_OBJECT, &args![start, colour, to, colour, 1u32])
                .u32();
            let seconds = e.global::<f32>(VATS_DEBUG_SECONDS);
            let tes = e.global::<u32>(TES_POINTER);
            e.call(TES_ADD_TEMP_DEBUG_OBJECT, &args![tes, line, seconds]);
        }
        e.call(COLLECTOR_DESTRUCTOR, &args![frame.collector]);
        distance
    })
}

// Translated from 008bdbd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Actor::GetVATSTargetVisible` (Xbox PDB): how far toward `target` the
/// actor can see it. Zero without a target or without its 3D data (virtual
/// `+0x1d0`). Otherwise it casts rays from the actor's eye position along
/// its heading (`heading_offset` added) to the centre of the target's bound,
/// one for each step `n = 1, 2, ...` of length `0x011d09e8` while `n` is
/// below the count `0x011d0850`; a ray that strikes something other than the
/// actor with a fraction up to `0.85` (`0x01019de8`) ends the search, unless
/// the thing is a reference whose virtual `+0x100` holds and the ray is
/// longer than 256 (`0x010231d8`). It returns `step * n` for the last step
/// `n` whose ray was not blocked (zero when the first is blocked). With the
/// debug setting on it adds each ray as
/// a debug line. (The structured-exception frame and the stack cookie are not
/// translated.)
pub fn actor_get_vats_target_visible(
    e: &mut Engine,
    this: Ptr<Actor>,
    target: Ptr,
    heading_offset: f32,
) -> f32 {
    let actor = this.addr();
    if target.is_null() {
        return 0.0;
    }
    if e.vcall(target.addr(), VSLOT_REFERENCE_3D, &args![]).u32() == 0 {
        return 0.0;
    }
    e.with_stack(0x560, |e, block| {
        let top = block.addr() + 0x550;
        let start = top - 0x24;
        let direction = top - 0x30;
        let matrix = top - 0x54;
        let centre = top - 0x64;
        let scaled_ray = top - 0x98;
        let end = top - 0xa4;
        let frame = RayFrame {
            ray: top - 0x160,
            filter: top - 0x164,
            scratch: top - 0x168,
            collector: top - 0x490,
        };
        let colour = top - 0x4a0;
        let result = top - 0x510;
        let difference = top - 0x528;

        let position = e.vcall(actor, VSLOT_ACTOR_POSITION, &args![]).u32();
        copy_words(e, start, position, 3);
        let height = setting_float(e, VATS_HEIGHT_SETTING);
        let raised = (e.mem.f32(start + 8) as f64 + height as f64) as f32;
        e.mem.set_f32(start + 8, raised);
        e.call(LIST_NODE_DATA_ADDRESS, &args![direction]);
        e.call(LIST_NODE_DATA_ADDRESS, &args![matrix]);
        let heading = e.vcall(actor, VSLOT_ACTOR_HEADING, &args![0u32]).f64();
        let angle = (heading + heading_offset as f64) as f32;
        e.call(MATRIX_SET_HEADING, &args![matrix, angle]);
        e.call(MATRIX_GET_COLUMN, &args![matrix, 1u32, direction]);
        e.call(VECTOR_NORMALIZE, &args![direction]);
        let scene = e.vcall(target.addr(), VSLOT_REFERENCE_3D, &args![]).u32();
        let bound = e.call(WORLD_BOUND, &args![scene]).u32();
        let bound = e.call(LIST_NODE_DATA_ADDRESS, &args![bound]).u32();
        copy_words(e, centre, bound, 3);

        let mut blocked = false;
        let mut visible = 0.0f32;
        let mut step = 0i32;
        loop {
            if blocked {
                break;
            }
            let count = setting_float(e, VATS_STEP_COUNT_SETTING);
            if (count as f64).partial_cmp(&(step as f64)) != Some(std::cmp::Ordering::Greater) {
                break;
            }
            let next = step + 1;
            let step_length = setting_float(e, VATS_STEP_LENGTH_SETTING);
            let reach = (step_length as f64 * next as f64) as f32;
            let ray = e
                .call(VECTOR_TIMES_SCALAR, &args![scaled_ray, reach, direction])
                .u32();
            e.call(VECTOR_ADD, &args![start, end, ray]);
            prepare_ray(e, actor, frame, end, centre);
            let setting = e
                .call(SETTING_BYTE_POINTER, &args![VATS_DEBUG_SETTING])
                .u32();
            if e.mem.u8(setting) != 0 {
                e.call(
                    COLOUR_CONSTRUCTOR,
                    &args![colour, 1.0f32, 1.0f32, 0.0f32, 1.0f32],
                );
                let line = e
                    .call(DEBUG_LINE_OBJECT, &args![centre, colour, end, colour, 1u32])
                    .u32();
                let seconds = e.global::<f32>(VATS_DEBUG_SECONDS);
                let tes = e.global::<u32>(TES_POINTER);
                e.call(TES_ADD_TEMP_DEBUG_OBJECT, &args![tes, line, seconds]);
            }
            cast_ray(e, frame.ray);
            let mut index = 0;
            while !blocked && index < ray_hit_count(e, frame.ray) {
                let (object, reference, second) = ray_hit_objects(e, frame.ray, index, result);
                if object != 0 && second != 0 && reference != target.addr() {
                    blocked = true;
                    let fraction = e.mem.f32(result + 0x10);
                    if fraction as f64 > e.global::<f64>(VATS_FRACTION_LIMIT) {
                        blocked = false;
                    }
                    if reference != 0 && e.vcall(reference, VSLOT_ACTOR_0X100, &args![]).bool() {
                        let offset = e
                            .call(VECTOR_SUBTRACT, &args![centre, difference, end])
                            .u32();
                        let length = e.call(VECTOR_LENGTH, &args![offset]).f64();
                        let scaled = (length * fraction as f64) as f32;
                        if scaled as f64 > e.global::<f64>(VATS_DISTANCE_LIMIT) {
                            blocked = false;
                        }
                    }
                }
                index += 1;
            }
            if !blocked {
                let step_length = setting_float(e, VATS_STEP_LENGTH_SETTING);
                visible = (step_length as f64 * (step + 1) as f64) as f32;
            }
            e.call(COLLECTOR_DESTRUCTOR, &args![frame.collector]);
            step += 1;
        }
        visible
    })
}

// Translated from 008be0a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Actor::GetFireNode` (Xbox PDB): zero without a process; otherwise the
/// process's cached fire node (virtual `+0x184`) when it has one. Else it
/// looks the node up in the actor's 3D data (the player's current 3D for the
/// player, actor virtual `+0x1d0` for the others), through the current weapon
/// (`00525700`) or, without a weapon, by the name `ProjectileNode`
/// (`004aae30`), stores it in the process (virtual `+0x188`) and returns it.
pub fn actor_get_fire_node(e: &mut Engine, this: Ptr<Actor>) -> Ptr {
    let actor = this.addr();
    let process = e.get(this, Actor::pCurrentProcess).addr();
    if process == 0 {
        return Ptr::NULL;
    }
    let cached = e.vcall(process, VSLOT_PROCESS_FIRE_NODE, &args![]).u32();
    if cached != 0 {
        return Ptr::new(cached);
    }
    let scene = if actor == player_pointer(e) {
        let player = player_pointer(e);
        e.call(PLAYER_CURRENT_3D, &args![player]).u32()
    } else {
        e.vcall(actor, VSLOT_REFERENCE_3D, &args![]).u32()
    };
    if scene == 0 {
        return Ptr::NULL;
    }
    let weapon = e.call(ACTOR_CURRENT_WEAPON, &args![actor]).u32();
    let node = if weapon != 0 {
        e.call(WEAPON_FIRE_NODE, &args![weapon, scene]).u32()
    } else {
        e.call(FIND_NODE_BY_NAME, &args![scene, PROJECTILE_NODE_NAME])
            .u32()
    };
    let process = e.get(this, Actor::pCurrentProcess).addr();
    e.vcall(process, VSLOT_PROCESS_SET_FIRE_NODE, &args![node]);
    Ptr::new(node)
}

// Translated from 008be180 (decompiled, FalloutNV.exe 1.4.0.525)
/// With a process, hands `node` to its virtual `+0x188` (the setter
/// `Actor::GetFireNode` uses).
pub fn fn_008be180(e: &mut Engine, this: Ptr<Actor>, node: u32) {
    let process = e.get(this, Actor::pCurrentProcess).addr();
    if process != 0 {
        e.vcall(process, VSLOT_PROCESS_SET_FIRE_NODE, &args![node]);
    }
}

/// The `CachedValuesOwner` of an actor (`actor + 0xa8`, null for null).
fn cached_owner_of(actor: Ptr<Actor>) -> Ptr {
    if actor.is_null() {
        Ptr::NULL
    } else {
        Ptr::new(actor.addr() + 0xa8)
    }
}

/// The actor's process when it has a cached-values block.
fn process_with_cached_values(e: &mut Engine, actor: Ptr<Actor>) -> Option<Ptr> {
    let process = e.get(actor, Actor::pCurrentProcess);
    if !process.is_null() && e.call(HAS_CACHED_VALUES, &args![process]).bool() {
        Some(process)
    } else {
        None
    }
}

/// The shared body of the process-level cached-value readers: zero without a
/// cached-values block (`process + 0x2c`); the cached word at `offset` while
/// none of the `mask` flags is set; otherwise the owner's virtual `slot`
/// (the value is recomputed). Returns the `float` in `st0` or the word.
fn cached_value(
    e: &mut Engine,
    process: Ptr,
    owner: Ptr,
    mask: u32,
    offset: u32,
    slot: u32,
) -> Ret {
    // BaseProcess::pCachedValues (Xbox PDB) +0x2c
    let block = e.mem.u32(process.addr() + 0x2c);
    if block == 0 {
        return Ret::default();
    }
    if !e.call(CACHED_VALUES_NONE_OF, &args![block, mask]).bool() {
        Ret {
            eax: e.mem.u32(block + offset),
            st0: e.mem.f32(block + offset) as f64,
            ..Ret::default()
        }
    } else {
        e.vcall(owner.addr(), slot, &args![])
    }
}

/// The shared body of the actor-level getters: the process-level reader
/// `reader` when the actor's process has a cached-values block, otherwise
/// the owner's virtual `slot` directly.
fn cached_getter(
    e: &mut Engine,
    actor: Ptr<Actor>,
    slot: u32,
    reader: fn(&mut Engine, Ptr, Ptr) -> Ret,
) -> Ret {
    if let Some(process) = process_with_cached_values(e, actor) {
        let owner = cached_owner_of(actor);
        return reader(e, process, owner);
    }
    e.vcall(actor.addr() + 0xa8, slot, &args![])
}

// Translated from 008be1b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Actor::GetRadius` (Xbox PDB): `CachedValuesOwner` virtual `+0x00`
/// (`CalculateCachedRadius`) through the process's cached radius
/// (`008be220`) when the process has a cached-values block.
pub fn actor_get_radius(e: &mut Engine, this: Ptr<Actor>) -> f32 {
    cached_getter(e, this, 0x00, process_radius).f32()
}

fn process_radius(e: &mut Engine, process: Ptr, owner: Ptr) -> Ret {
    cached_value(e, process, owner, 0x1, 0x00, 0x00)
}

// Translated from 008be220 (decompiled, FalloutNV.exe 1.4.0.525)
/// The process's radius: `0` without a cached-values block, the cached
/// `fCachedRadius` (`+0x00`) while its flag `0x1` is clear in `iFlags`,
/// otherwise the owner's virtual `+0x00`.
pub fn fn_008be220(e: &mut Engine, this: Ptr, owner: Ptr) -> f32 {
    process_radius(e, this, owner).f32()
}

// Translated from 008be280 (decompiled, FalloutNV.exe 1.4.0.525)
/// Like `Actor::GetRadius` for virtual `+0x04` (`CalculateCachedWidth`)
/// through `008be2f0`.
pub fn fn_008be280(e: &mut Engine, this: Ptr<Actor>) -> f32 {
    cached_getter(e, this, 0x04, process_width).f32()
}

fn process_width(e: &mut Engine, process: Ptr, owner: Ptr) -> Ret {
    cached_value(e, process, owner, 0x2, 0x04, 0x04)
}

// Translated from 008be2f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The process's width: the `008be220` pattern with flag `0x2`, field `+0x04`,
/// owner virtual `+0x04`.
pub fn fn_008be2f0(e: &mut Engine, this: Ptr, owner: Ptr) -> f32 {
    process_width(e, this, owner).f32()
}

// Translated from 008be350 (decompiled, FalloutNV.exe 1.4.0.525)
/// Like `Actor::GetRadius` for virtual `+0x08` (`CalculateCachedLength`)
/// through `008be3c0`.
pub fn fn_008be350(e: &mut Engine, this: Ptr<Actor>) -> f32 {
    cached_getter(e, this, 0x08, process_length).f32()
}

fn process_length(e: &mut Engine, process: Ptr, owner: Ptr) -> Ret {
    cached_value(e, process, owner, 0x4, 0x08, 0x08)
}

// Translated from 008be3c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The process's length: the `008be220` pattern with flag `0x4`, field
/// `+0x08`, owner virtual `+0x08`.
pub fn fn_008be3c0(e: &mut Engine, this: Ptr, owner: Ptr) -> f32 {
    process_length(e, this, owner).f32()
}

// Translated from 008be420 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Actor::GetForwardLength` (Xbox PDB): like `Actor::GetRadius` for virtual
/// `+0x0c` (`CalculateCachedForwardLength`) through `008be490`.
pub fn actor_get_forward_length(e: &mut Engine, this: Ptr<Actor>) -> f32 {
    cached_getter(e, this, 0x0c, process_forward_length).f32()
}

fn process_forward_length(e: &mut Engine, process: Ptr, owner: Ptr) -> Ret {
    cached_value(e, process, owner, 0x8000, 0x0c, 0x0c)
}

// Translated from 008be490 (decompiled, FalloutNV.exe 1.4.0.525)
/// The process's forward length: the `008be220` pattern with flag `0x8000`,
/// field `+0x0c`, owner virtual `+0x0c`.
pub fn fn_008be490(e: &mut Engine, this: Ptr, owner: Ptr) -> f32 {
    process_forward_length(e, this, owner).f32()
}

// Translated from 008be4f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// True when the value of owner virtual `+0x1c` (`CalculateCachedParalysis`,
/// Xbox PDB), read through `008be5a0` when the process has a cached-values
/// block, is above zero (the `double` at `0x01012060`).
pub fn fn_008be4f0(e: &mut Engine, this: Ptr<Actor>) -> bool {
    let value = cached_getter(e, this, 0x1c, process_paralysis).f64();
    value > e.global::<f64>(DOUBLE_ZERO)
}

fn process_paralysis(e: &mut Engine, process: Ptr, owner: Ptr) -> Ret {
    cached_value(e, process, owner, 0x20, 0x1c, 0x1c)
}

// Translated from 008be5a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The process's paralysis: the `008be220` pattern with flag `0x20`, field
/// `+0x1c`, owner virtual `+0x1c`.
pub fn fn_008be5a0(e: &mut Engine, this: Ptr, owner: Ptr) -> f32 {
    process_paralysis(e, this, owner).f32()
}

// Translated from 008be600 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Actor::GetWeaponDamagePerSecond` (Xbox PDB): like `Actor::GetRadius` for
/// virtual `+0x10` (`CalculateCachedWeaponDPS`) through `008be670`.
pub fn actor_get_weapon_damage_per_second(e: &mut Engine, this: Ptr<Actor>) -> f32 {
    cached_getter(e, this, 0x10, process_weapon_dps).f32()
}

fn process_weapon_dps(e: &mut Engine, process: Ptr, owner: Ptr) -> Ret {
    cached_value(e, process, owner, 0x8, 0x10, 0x10)
}

// Translated from 008be670 (decompiled, FalloutNV.exe 1.4.0.525)
/// The process's weapon damage per second: the `008be220` pattern with flag
/// `0x8`, field `+0x10`, owner virtual `+0x10`.
pub fn fn_008be670(e: &mut Engine, this: Ptr, owner: Ptr) -> f32 {
    process_weapon_dps(e, this, owner).f32()
}

// Translated from 008be6d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Like `Actor::GetRadius` for virtual `+0x20` (`CalculateCachedHealingRate`)
/// through `008be740`.
pub fn fn_008be6d0(e: &mut Engine, this: Ptr<Actor>) -> f32 {
    cached_getter(e, this, 0x20, process_healing_rate).f32()
}

fn process_healing_rate(e: &mut Engine, process: Ptr, owner: Ptr) -> Ret {
    cached_value(e, process, owner, 0x40, 0x20, 0x20)
}

// Translated from 008be740 (decompiled, FalloutNV.exe 1.4.0.525)
/// The process's healing rate: the `008be220` pattern with flag `0x40`,
/// field `+0x20`, owner virtual `+0x20`.
pub fn fn_008be740(e: &mut Engine, this: Ptr, owner: Ptr) -> f32 {
    process_healing_rate(e, this, owner).f32()
}

// Translated from 008be7a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Actor::GetEndurance` (Xbox PDB): like `Actor::GetRadius` for virtual
/// `+0x24` (`CalculateCachedEndurance`) through `008be810`.
pub fn actor_get_endurance(e: &mut Engine, this: Ptr<Actor>) -> f32 {
    cached_getter(e, this, 0x24, process_endurance).f32()
}

fn process_endurance(e: &mut Engine, process: Ptr, owner: Ptr) -> Ret {
    cached_value(e, process, owner, 0x80, 0x24, 0x24)
}

// Translated from 008be810 (decompiled, FalloutNV.exe 1.4.0.525)
/// The process's endurance: the `008be220` pattern with flag `0x80`, field
/// `+0x24`, owner virtual `+0x24`.
pub fn fn_008be810(e: &mut Engine, this: Ptr, owner: Ptr) -> f32 {
    process_endurance(e, this, owner).f32()
}

// Translated from 008be870 (decompiled, FalloutNV.exe 1.4.0.525)
/// Like `Actor::GetRadius` for virtual `+0x28`
/// (`CalculateCachedPerceptionCondition`) through `008be8e0`.
pub fn fn_008be870(e: &mut Engine, this: Ptr<Actor>) -> f32 {
    cached_getter(e, this, 0x28, process_perception_condition).f32()
}

fn process_perception_condition(e: &mut Engine, process: Ptr, owner: Ptr) -> Ret {
    cached_value(e, process, owner, 0x100, 0x28, 0x28)
}

// Translated from 008be8e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The process's perception condition: the `008be220` pattern with flag
/// `0x100`, field `+0x28`, owner virtual `+0x28`.
pub fn fn_008be8e0(e: &mut Engine, this: Ptr, owner: Ptr) -> f32 {
    process_perception_condition(e, this, owner).f32()
}

// Translated from 008be940 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Actor::GetEyeLevel` (Xbox PDB): like `Actor::GetRadius` for virtual
/// `+0x2c` (`CalculateCachedEyeLevel`) through `008be9b0`.
pub fn actor_get_eye_level(e: &mut Engine, this: Ptr<Actor>) -> f32 {
    cached_getter(e, this, 0x2c, process_eye_level).f32()
}

fn process_eye_level(e: &mut Engine, process: Ptr, owner: Ptr) -> Ret {
    cached_value(e, process, owner, 0x200, 0x2c, 0x2c)
}

// Translated from 008be9b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The process's eye level: the `008be220` pattern with flag `0x200`, field
/// `+0x2c`, owner virtual `+0x2c`.
pub fn fn_008be9b0(e: &mut Engine, this: Ptr, owner: Ptr) -> f32 {
    process_eye_level(e, this, owner).f32()
}

// Translated from 008bea10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Actor::GetActorAggression` (Xbox PDB): like `Actor::GetRadius` for
/// virtual `+0x30` (`CalculateCachedActorAggression`) through `008bea80`;
/// the value is a word.
pub fn actor_get_actor_aggression(e: &mut Engine, this: Ptr<Actor>) -> u32 {
    cached_getter(e, this, 0x30, process_aggression).u32()
}

fn process_aggression(e: &mut Engine, process: Ptr, owner: Ptr) -> Ret {
    cached_value(e, process, owner, 0x400, 0x30, 0x30)
}

// Translated from 008bea80 (decompiled, FalloutNV.exe 1.4.0.525)
/// The process's aggression word: the `008be220` pattern with flag `0x400`,
/// word `+0x30`, owner virtual `+0x30`.
pub fn fn_008bea80(e: &mut Engine, this: Ptr, owner: Ptr) -> u32 {
    process_aggression(e, this, owner).u32()
}

// Translated from 008beae0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Actor::GetActorAssistance` (Xbox PDB): like `Actor::GetActorAggression`
/// for virtual `+0x34` (`CalculateCachedActorAssistance`) through
/// `008beb50`.
pub fn actor_get_actor_assistance(e: &mut Engine, this: Ptr<Actor>) -> u32 {
    cached_getter(e, this, 0x34, process_assistance).u32()
}

fn process_assistance(e: &mut Engine, process: Ptr, owner: Ptr) -> Ret {
    cached_value(e, process, owner, 0x800, 0x34, 0x34)
}

// Translated from 008beb50 (decompiled, FalloutNV.exe 1.4.0.525)
/// The process's assistance word: the `008be220` pattern with flag `0x800`,
/// word `+0x34`, owner virtual `+0x34`.
pub fn fn_008beb50(e: &mut Engine, this: Ptr, owner: Ptr) -> u32 {
    process_assistance(e, this, owner).u32()
}

// Translated from 008bebb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Like `Actor::GetRadius` for virtual `+0x14`
/// (`CalculateCachedMedicineEffectivenessMult`) through `008bec20`.
pub fn fn_008bebb0(e: &mut Engine, this: Ptr<Actor>) -> f32 {
    cached_getter(e, this, 0x14, process_medicine).f32()
}

fn process_medicine(e: &mut Engine, process: Ptr, owner: Ptr) -> Ret {
    cached_value(e, process, owner, 0x10, 0x14, 0x14)
}

// Translated from 008bec20 (decompiled, FalloutNV.exe 1.4.0.525)
/// The process's medicine effectiveness multiplier: the `008be220` pattern
/// with flag `0x10`, field `+0x14`, owner virtual `+0x14`.
pub fn fn_008bec20(e: &mut Engine, this: Ptr, owner: Ptr) -> f32 {
    process_medicine(e, this, owner).f32()
}

/// This part's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x008b8e90, actor_is_in_faction(Ptr<Actor>, Ptr) -> bool),
        entry!(0x008b8f20, fn_008b8f20(Ptr<Actor>, u32)),
        entry!(0x008b8f90, fn_008b8f90(Ptr<Actor>)),
        entry!(0x008b9220, fn_008b9220(Ptr) -> Ptr),
        entry!(
            0x008b9240,
            actor_condition_modified_callback(Ptr, i32, f32, f32, Ptr)
        ),
        entry!(0x008b9790, fn_008b9790(Ptr) -> f32),
        entry!(0x008b97f0, fn_008b97f0(Ptr, f32)),
        entry!(
            0x008b9830,
            actor_mobility_condition_modified_callback(Ptr, i32, f32, f32, Ptr)
        ),
        entry!(0x008b9960, fn_008b9960(Ptr, i32, f32, f32)),
        entry!(0x008b9ab0, fn_008b9ab0(Ptr)),
        entry!(0x008b9b10, fn_008b9b10(Ptr)),
        entry!(0x008b9b70, fn_008b9b70(Ptr) -> f32),
        entry!(0x008b9be0, fn_008b9be0(Ptr, f32)),
        entry!(0x008b9c10, fn_008b9c10(Ptr) -> f32),
        entry!(0x008b9c80, fn_008b9c80(Ptr, f32)),
        entry!(0x008b9cc0, fn_008b9cc0(Ptr)),
        entry!(0x008b9d20, fn_008b9d20(Ptr) -> f32),
        entry!(0x008b9d80, fn_008b9d80(Ptr, f32)),
        entry!(0x008b9db0, fn_008b9db0(Ptr)),
        entry!(0x008b9e10, fn_008b9e10(Ptr) -> f32),
        entry!(0x008b9e70, fn_008b9e70(Ptr, f32)),
        entry!(
            0x008b9ea0,
            actor_endurance_modified_callback(Ptr, u32, f32, f32)
        ),
        entry!(0x008b9ff0, fn_008b9ff0(Ptr) -> f32),
        entry!(0x008ba050, fn_008ba050(Ptr, f32)),
        entry!(
            0x008ba090,
            actor_queue_replacement_locomotion(Ptr<Actor>, f32, f32, f32)
        ),
        entry!(0x008ba140, fn_008ba140(Ptr)),
        entry!(0x008ba210, fn_008ba210(Ptr)),
        entry!(0x008ba270, fn_008ba270(Ptr)),
        entry!(0x008ba2b0, fn_008ba2b0(Ptr)),
        entry!(0x008ba2f0, fn_008ba2f0(Ptr, u32, f32)),
        entry!(0x008ba3e0, fn_008ba3e0(Ptr<Actor>) -> bool),
        entry!(0x008ba410, fn_008ba410(Ptr<Actor>) -> bool),
        entry!(
            0x008ba440,
            actor_is_heavy_body_armor_worn(Ptr<Actor>) -> bool
        ),
        entry!(0x008ba470, fn_008ba470(Ptr<Actor>) -> bool),
        entry!(0x008ba4a0, actor_is_anim_idle_quequed(Ptr<Actor>) -> bool),
        entry!(
            0x008ba530,
            actor_is_playing_lower_body_special_idle(Ptr<Actor>) -> bool
        ),
        entry!(0x008ba600, fn_008ba600(Ptr<Actor>)),
        entry!(
            0x008bb520,
            fn_008bb520(Ptr<Actor>, f32, f32, f32, bool) -> bool
        ),
        entry!(0x008bb5a0, fn_008bb5a0(Ptr) -> u32),
        entry!(0x008bb5c0, fn_008bb5c0(Ptr<Actor>, f32, bool) -> bool),
        entry!(
            0x008bb630,
            actor_set_pathfinding_flee(Ptr<Actor>, u32) -> bool
        ),
        entry!(0x008bb650, actor_set_iron_sights(Ptr<Actor>, u8, u8, u8)),
        entry!(0x008bbbd0, fn_008bbbd0(Ptr) -> u8),
        entry!(0x008bbbf0, fn_008bbbf0(Ptr, u32)),
        entry!(0x008bbc10, actor_get_iron_sights(Ptr<Actor>) -> u8),
        entry!(
            0x008bbc40,
            actor_apply_constr_template(Ptr<Actor>, u32) -> bool
        ),
        entry!(0x008bbd60, fn_008bbd60(Ptr, u32) -> u32),
        entry!(0x008bbd90, fn_008bbd90(Ptr) -> u32),
        entry!(0x008bbdb0, fn_008bbdb0(Ptr)),
        entry!(0x008bbe00, fn_008bbe00(Ptr)),
        entry!(0x008bbe20, fn_008bbe20(Ptr) -> u32),
        entry!(0x008bbe70, fn_008bbe70(Ptr, u32)),
        entry!(0x008bbeb0, fn_008bbeb0(Ptr) -> u32),
        entry!(0x008bbf00, fn_008bbf00(Ptr, u32)),
        entry!(
            0x008bbf40,
            fn_008bbf40(Ptr<Actor>, u32, u32, u32, u16, u32, u8, u8, u8, u8)
        ),
        entry!(0x008bc240, fn_008bc240(Ptr<Actor>, u32)),
        entry!(0x008bc270, fn_008bc270(Ptr, Ptr)),
        entry!(0x008bc300, fn_008bc300(Ptr<Actor>)),
        entry!(0x008bc3d0, actor_start_greeting_player(Ptr<Actor>, u32)),
        entry!(0x008bc520, fn_008bc520(Ptr) -> bool),
        entry!(0x008bc560, fn_008bc560(Ptr)),
        entry!(0x008bc590, fn_008bc590(u32, u8)),
        entry!(
            0x008bc700,
            actor_is_in_combat_with_actor(Ptr<Actor>, u32) -> bool
        ),
        entry!(0x008bc750, fn_008bc750(Ptr, u32, u8)),
        entry!(0x008bc790, actor_add_follower(Ptr<Actor>, u32)),
        entry!(0x008bc7d0, fn_008bc7d0(Ptr, u32) -> bool),
        entry!(0x008bc7f0, actor_is_following_ov2(Ptr<Actor>, u32) -> bool),
        entry!(
            0x008bc860,
            actor_could_be_following(Ptr<Actor>, u32) -> bool
        ),
        entry!(0x008bc980, fn_008bc980(Ptr<Actor>, u32, u32, u32, u8)),
        entry!(0x008bc9d0, fn_008bc9d0(Ptr<Actor>, Ptr<Actor>, u8) -> bool),
        entry!(0x008bca70, fn_008bca70(Ptr<Actor>) -> bool),
        entry!(0x008bca90, actor_set_player_teammate(Ptr<Actor>, u8)),
        entry!(0x008bcb00, fn_008bcb00(Ptr, u32)),
        entry!(0x008bcb40, fn_008bcb40(Ptr)),
        entry!(0x008bcbd0, fn_008bcbd0(Ptr, Ptr)),
        entry!(0x008bcc60, fn_008bcc60(Ptr, Ptr) -> Ptr),
        entry!(
            0x008bcc80,
            actor_should_actor_avoid_radiation(Ptr<Actor>) -> bool
        ),
        entry!(0x008bcd20, actor_fade_in(Ptr<Actor>)),
        entry!(0x008bcd60, fn_008bcd60(Ptr<Actor>)),
        entry!(0x008bcda0, fn_008bcda0(Ptr<Actor>) -> f32),
        entry!(0x008bcdd0, fn_008bcdd0(Ptr) -> f32),
        entry!(0x008bcdf0, fn_008bcdf0(Ptr<Actor>, u8, u32) -> bool),
        entry!(0x008bd550, fn_008bd550() -> f32),
        entry!(0x008bd570, fn_008bd570(Ptr, u8)),
        entry!(0x008bd5c0, fn_008bd5c0(Ptr) -> u8),
        entry!(0x008bd5e0, fn_008bd5e0(Ptr, u32)),
        entry!(0x008bd610, fn_008bd610(Ptr, u8)),
        entry!(0x008bd630, actor_fade_skins(Ptr<Actor>, Ptr, f32)),
        entry!(0x008bd700, fn_008bd700(Ptr) -> u8),
        entry!(0x008bd7b0, actor_get_heading(Ptr<Actor>, u8) -> f32),
        entry!(
            0x008bd830,
            actor_get_vats_area_free(Ptr<Actor>, u32, f32) -> f32
        ),
        entry!(
            0x008bdbd0,
            actor_get_vats_target_visible(Ptr<Actor>, Ptr, f32) -> f32
        ),
        entry!(0x008be0a0, actor_get_fire_node(Ptr<Actor>) -> Ptr),
        entry!(0x008be180, fn_008be180(Ptr<Actor>, u32)),
        entry!(0x008be1b0, actor_get_radius(Ptr<Actor>) -> f32),
        entry!(0x008be220, fn_008be220(Ptr, Ptr) -> f32),
        entry!(0x008be280, fn_008be280(Ptr<Actor>) -> f32),
        entry!(0x008be2f0, fn_008be2f0(Ptr, Ptr) -> f32),
        entry!(0x008be350, fn_008be350(Ptr<Actor>) -> f32),
        entry!(0x008be3c0, fn_008be3c0(Ptr, Ptr) -> f32),
        entry!(0x008be420, actor_get_forward_length(Ptr<Actor>) -> f32),
        entry!(0x008be490, fn_008be490(Ptr, Ptr) -> f32),
        entry!(0x008be4f0, fn_008be4f0(Ptr<Actor>) -> bool),
        entry!(0x008be5a0, fn_008be5a0(Ptr, Ptr) -> f32),
        entry!(
            0x008be600,
            actor_get_weapon_damage_per_second(Ptr<Actor>) -> f32
        ),
        entry!(0x008be670, fn_008be670(Ptr, Ptr) -> f32),
        entry!(0x008be6d0, fn_008be6d0(Ptr<Actor>) -> f32),
        entry!(0x008be740, fn_008be740(Ptr, Ptr) -> f32),
        entry!(0x008be7a0, actor_get_endurance(Ptr<Actor>) -> f32),
        entry!(0x008be810, fn_008be810(Ptr, Ptr) -> f32),
        entry!(0x008be870, fn_008be870(Ptr<Actor>) -> f32),
        entry!(0x008be8e0, fn_008be8e0(Ptr, Ptr) -> f32),
        entry!(0x008be940, actor_get_eye_level(Ptr<Actor>) -> f32),
        entry!(0x008be9b0, fn_008be9b0(Ptr, Ptr) -> f32),
        entry!(0x008bea10, actor_get_actor_aggression(Ptr<Actor>) -> u32),
        entry!(0x008bea80, fn_008bea80(Ptr, Ptr) -> u32),
        entry!(0x008beae0, actor_get_actor_assistance(Ptr<Actor>) -> u32),
        entry!(0x008beb50, fn_008beb50(Ptr, Ptr) -> u32),
        entry!(0x008bebb0, fn_008bebb0(Ptr<Actor>) -> f32),
        entry!(0x008bec20, fn_008bec20(Ptr, Ptr) -> f32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    const ACTOR_TABLE: u32 = 0x0200_0000;
    const OWNER_TABLE: u32 = 0x0200_2000;
    const PROCESS_TABLE: u32 = 0x0200_4000;
    const REFERENCE_TABLE: u32 = 0x0200_6000;
    const BASE_TABLE: u32 = 0x0200_8000;
    const ANIMATION_TABLE: u32 = 0x0200_a000;
    const PLAYER_TABLE: u32 = 0x0200_c000;

    fn eax(value: u32) -> Ret {
        Ret {
            eax: value,
            ..Ret::default()
        }
    }

    fn st0(value: f32) -> Ret {
        Ret {
            st0: value as f64,
            ..Ret::default()
        }
    }

    /// An engine with the vtable areas and the exe data pages the code reads.
    fn engine() -> Engine {
        let mut e = Engine::new();
        for table in [
            ACTOR_TABLE,
            OWNER_TABLE,
            PROCESS_TABLE,
            REFERENCE_TABLE,
            BASE_TABLE,
            ANIMATION_TABLE,
            PLAYER_TABLE,
        ] {
            e.map(table, 0x800);
        }
        for page in [
            0x0101_1000,
            0x0101_2000,
            0x0101_6000,
            0x0119_7000,
            0x011d_9000,
            0x011d_e000,
        ] {
            e.map(page, 0x1000);
        }
        e.set_global(DOUBLE_TWO, 2.0f64);
        e.set_global(DOUBLE_ONE, 1.0f64);
        e.set_global(FULL_VALUE, 100.0f32);
        e.set_global(MESSAGE_TIME, 2.0f32);
        e
    }

    fn double(e: &mut Engine, addr: u32, ret: Ret) {
        e.register_double(addr, move |_, _| ret);
    }

    /// Doubles that answer zero to every address in `addrs`.
    fn quiet(e: &mut Engine, addrs: &[u32]) {
        for addr in addrs {
            double(e, *addr, Ret::default());
        }
    }

    fn slot_target(table: u32, offset: u32) -> u32 {
        0x0300_0000 + (table - 0x0200_0000) + offset
    }

    /// Puts a function answering `ret` into a vtable slot.
    fn slot(e: &mut Engine, table: u32, offset: u32, ret: Ret) {
        let target = slot_target(table, offset);
        e.mem.set_u32(table + offset, target);
        double(e, target, ret);
    }

    /// An actor with the actor and `ActorValueOwner` vtables.
    fn new_actor(e: &mut Engine) -> Ptr<Actor> {
        let actor = e.new_object::<Actor>();
        e.mem.set_u32(actor.addr(), ACTOR_TABLE);
        e.mem.set_u32(actor.addr() + 0xa4, OWNER_TABLE);
        actor
    }

    fn owner(actor: Ptr<Actor>) -> Ptr {
        Ptr::new(actor.addr() + 0xa4)
    }

    /// Gives the actor a process with a vtable.
    fn with_process(e: &mut Engine, actor: Ptr<Actor>) -> u32 {
        let process = e.mem.alloc(0x800);
        e.mem.set_u32(process, PROCESS_TABLE);
        e.set(actor, Actor::pCurrentProcess, Ptr::new(process));
        process
    }

    /// Process-level doubles of the cached-values helpers: the process has
    /// a block at `+0x2c`, and the forwarded masks are recorded.
    fn cached_helpers(e: &mut Engine) -> Rc<RefCell<Vec<u32>>> {
        let masks = Rc::new(RefCell::new(Vec::new()));
        e.register(HAS_CACHED_VALUES, |e, a| {
            eax((e.mem.u32(a[0] + 0x2c) != 0) as u32)
        });
        let seen = masks.clone();
        e.register_double(PROCESS_FORWARD_FLAG_MASK, move |_, a| {
            seen.borrow_mut().push(a[1]);
            Ret::default()
        });
        e.register(CACHED_VALUES_ADD_FLAGS, |e, a| {
            let flags = e.mem.u32(a[0] + 0x44);
            e.mem.set_u32(a[0] + 0x44, flags | a[1]);
            Ret::default()
        });
        masks
    }

    fn logged(e: &Engine) -> Vec<(u32, Vec<u32>)> {
        e.call_log.clone().unwrap()
    }

    fn logged_addresses(e: &Engine) -> Vec<u32> {
        logged(e).iter().map(|entry| entry.0).collect()
    }

    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        logged(e)
            .into_iter()
            .filter(|entry| entry.0 == addr)
            .map(|entry| entry.1)
            .collect()
    }

    // ---- 008b8e90 / 008b8f20 -------------------------------------------

    fn faction_case(changes: u32, in_base: bool, expelled: bool, in_changes: bool) -> bool {
        let mut e = engine();
        let actor = new_actor(&mut e);
        double(&mut e, EXTRA_LIST_OF_REFERENCE, eax(0x5000));
        double(&mut e, EXTRA_LIST_GET_FACTION_CHANGES, eax(changes));
        double(&mut e, GET_BASE_FORM, eax(0x7000));
        e.register_double(BASE_FACTION_LIST_CONTAINS, move |_, a| {
            assert_eq!(a, [0x7030, 0x77]);
            eax(in_base as u32)
        });
        double(&mut e, FACTION_CHANGES_IS_EXPELLED, eax(expelled as u32));
        double(
            &mut e,
            FACTION_CHANGES_IS_IN_FACTION,
            eax(in_changes as u32),
        );
        actor_is_in_faction(&mut e, actor, Ptr::new(0x77))
    }

    #[test]
    fn faction_membership_uses_the_base_list_without_extra_data() {
        assert!(faction_case(0, true, true, false));
        assert!(!faction_case(0, false, false, true));
    }

    #[test]
    fn faction_membership_with_extra_data() {
        // Base member who is not expelled.
        assert!(faction_case(0x6000, true, false, false));
        // Base member who is expelled falls back to the extra data.
        assert!(!faction_case(0x6000, true, true, false));
        assert!(faction_case(0x6000, true, true, true));
        // Not a base member: the extra data decides.
        assert!(faction_case(0x6000, false, false, true));
        assert!(!faction_case(0x6000, false, false, false));
    }

    #[test]
    fn faction_changes_are_added_when_missing() {
        let mut e = engine();
        let actor = new_actor(&mut e);
        slot(&mut e, ACTOR_TABLE, 0x48, Ret::default());
        double(&mut e, EXTRA_LIST_OF_REFERENCE, eax(0x5000));
        let lookups = Rc::new(RefCell::new(0));
        let seen = lookups.clone();
        e.register_double(EXTRA_LIST_GET_FACTION_CHANGES, move |_, _| {
            *seen.borrow_mut() += 1;
            eax(if *seen.borrow() == 1 { 0 } else { 0x6000 })
        });
        quiet(
            &mut e,
            &[EXTRA_LIST_ADD_FACTION_CHANGES, FACTION_CHANGES_ADD],
        );
        e.call_log = Some(vec![]);
        fn_008b8f20(&mut e, actor, 0x1234);
        assert_eq!(
            calls_to(&e, slot_target(ACTOR_TABLE, 0x48)),
            [vec![actor.addr(), 0x8000_0000]]
        );
        assert_eq!(calls_to(&e, EXTRA_LIST_ADD_FACTION_CHANGES).len(), 1);
        assert_eq!(calls_to(&e, FACTION_CHANGES_ADD), [vec![0x6000, 0x1234]]);
    }

    #[test]
    fn faction_changes_already_present_are_not_added() {
        let mut e = engine();
        let actor = new_actor(&mut e);
        slot(&mut e, ACTOR_TABLE, 0x48, Ret::default());
        double(&mut e, EXTRA_LIST_OF_REFERENCE, eax(0x5000));
        double(&mut e, EXTRA_LIST_GET_FACTION_CHANGES, eax(0x6000));
        quiet(
            &mut e,
            &[EXTRA_LIST_ADD_FACTION_CHANGES, FACTION_CHANGES_ADD],
        );
        e.call_log = Some(vec![]);
        fn_008b8f20(&mut e, actor, 9);
        assert!(calls_to(&e, EXTRA_LIST_ADD_FACTION_CHANGES).is_empty());
        assert_eq!(calls_to(&e, FACTION_CHANGES_ADD), [vec![0x6000, 9]]);
    }

    // ---- 008b8f90 / 008b9220 -------------------------------------------

    /// One contact in the controller's list; `reference_table_slots` decides
    /// what the struck reference answers. Returns the engine and the actor.
    fn contact_setup(damage: f32) -> (Engine, Ptr<Actor>) {
        let mut e = engine();
        let actor = new_actor(&mut e);
        e.mem.set_u32(PLAYER_POINTER, 0x9000);
        let contact = e.mem.alloc(0x40);
        e.mem.set_u32(contact + 4, 0x11);
        e.mem.set_u32(contact + 8, 0x12);
        e.mem.set_u32(contact + 0xc, 0x13);
        e.mem.set_u32(contact + 0x10, 0x21);
        e.mem.set_u32(contact + 0x14, 0x22);
        e.mem.set_u32(contact + 0x18, 0x23);
        let data = e.mem.alloc(8);
        e.mem.set_u32(data, contact);
        double(&mut e, GET_CHAR_CONTROLLER, eax(0x3000));
        e.register(LIST_NODE_IS_EMPTY, |_, _| eax(0));
        e.register_double(LIST_NODE_DATA_ADDRESS, move |_, _| eax(data));
        e.register(LIST_NODE_NEXT, |_, _| eax(0));
        e.register(POINTER_GET, |_, a| eax(a[0] + 0x1000));
        double(&mut e, PROXY_OBJECT, eax(0x100));
        double(&mut e, HAVOK_TO_BODY, eax(0x200));
        double(&mut e, ENTRY_OBJECT, eax(0x300));
        let reference = e.mem.alloc(8);
        e.mem.set_u32(reference, REFERENCE_TABLE);
        double(&mut e, FIND_REFERENCE_FOR_3D, eax(reference));
        double(&mut e, PLAYER_EXCLUDED_REFERENCE, eax(0x4444));
        double(&mut e, CONTACT_VALUE_A, st0(1.0));
        double(&mut e, CONTACT_VALUE_B, st0(2.0));
        double(&mut e, IMPACT_DAMAGE, st0(damage));
        double(&mut e, OPERATOR_NEW, eax(0x8000));
        double(&mut e, HIT_DATA_CONSTRUCTOR, eax(0x8100));
        quiet(
            &mut e,
            &[
                HIT_DATA_POINTER_CONSTRUCTOR,
                INITIALIZE_IMPACT_DATA,
                ACTOR_HIT_ME,
                HIT_DATA_POINTER_DESTRUCTOR,
                REACT_TO_CONTACT,
            ],
        );
        slot(&mut e, REFERENCE_TABLE, 0x224, eax(0));
        slot(&mut e, REFERENCE_TABLE, 0x304, eax(0));
        (e, actor)
    }

    #[test]
    fn contacts_without_a_controller_do_nothing() {
        let (mut e, actor) = contact_setup(5.0);
        double(&mut e, GET_CHAR_CONTROLLER, eax(0));
        e.call_log = Some(vec![]);
        fn_008b8f90(&mut e, actor);
        assert_eq!(logged_addresses(&e), [GET_CHAR_CONTROLLER]);
    }

    #[test]
    fn a_damaging_contact_creates_hit_data_and_hits_the_actor() {
        let (mut e, actor) = contact_setup(5.0);
        e.call_log = Some(vec![]);
        fn_008b8f90(&mut e, actor);
        let impact = calls_to(&e, INITIALIZE_IMPACT_DATA);
        assert_eq!(impact.len(), 1);
        // hit data, struck reference, actor, damage, contact entry
        assert_eq!(impact[0][2], actor.addr());
        assert_eq!(f32::from_bits(impact[0][3]), 5.0);
        // `Actor::HitMe` gets the hit data the `NiPointer` slot holds (the
        // pointer double answers slot + 0x1000) and a zero flag.
        let hit_me = calls_to(&e, ACTOR_HIT_ME);
        assert_eq!(hit_me.len(), 1);
        assert_eq!(hit_me[0][0], actor.addr());
        assert_eq!(hit_me[0][2], 0);
        let constructed = calls_to(&e, HIT_DATA_POINTER_CONSTRUCTOR);
        assert_eq!(constructed[0][1], 0x8100);
        assert_eq!(hit_me[0][1], constructed[0][0] + 0x1000);
        assert_eq!(calls_to(&e, HIT_DATA_CONSTRUCTOR), [vec![0x8000]]);
        assert_eq!(calls_to(&e, HIT_DATA_POINTER_DESTRUCTOR).len(), 1);
        assert!(calls_to(&e, REACT_TO_CONTACT).is_empty());
        // The damage formula got (value B, value A).
        let formula = calls_to(&e, IMPACT_DAMAGE);
        assert_eq!(f32::from_bits(formula[0][0]), 2.0);
        assert_eq!(f32::from_bits(formula[0][1]), 1.0);
    }

    #[test]
    fn a_contact_without_damage_does_nothing() {
        let (mut e, actor) = contact_setup(0.0);
        e.call_log = Some(vec![]);
        fn_008b8f90(&mut e, actor);
        assert!(calls_to(&e, OPERATOR_NEW).is_empty());
        assert!(calls_to(&e, ACTOR_HIT_ME).is_empty());
    }

    #[test]
    fn the_players_own_reference_is_skipped() {
        let (mut e, actor) = contact_setup(5.0);
        let reference = e.call(FIND_REFERENCE_FOR_3D, &args![]).u32();
        double(&mut e, PLAYER_EXCLUDED_REFERENCE, eax(reference));
        e.call_log = Some(vec![]);
        fn_008b8f90(&mut e, actor);
        assert!(calls_to(&e, IMPACT_DAMAGE).is_empty());
        assert!(calls_to(&e, REACT_TO_CONTACT).is_empty());
    }

    #[test]
    fn a_character_reference_of_type_three_reacts_to_the_contact() {
        let (mut e, actor) = contact_setup(5.0);
        slot(&mut e, REFERENCE_TABLE, 0x224, eax(1));
        slot(&mut e, REFERENCE_TABLE, 0x304, eax(3));
        e.call_log = Some(vec![]);
        fn_008b8f90(&mut e, actor);
        let reaction = calls_to(&e, REACT_TO_CONTACT);
        assert_eq!(reaction.len(), 1);
        // reference, controller, vector at +4, vector at +0x10
        assert_eq!(
            reaction[0][1..],
            [0x3000, 0x11, 0x12, 0x13, 0x21, 0x22, 0x23]
        );
        assert!(calls_to(&e, IMPACT_DAMAGE).is_empty());
    }

    #[test]
    fn a_character_reference_of_another_type_is_ignored() {
        let (mut e, actor) = contact_setup(5.0);
        slot(&mut e, REFERENCE_TABLE, 0x224, eax(1));
        slot(&mut e, REFERENCE_TABLE, 0x304, eax(2));
        e.call_log = Some(vec![]);
        fn_008b8f90(&mut e, actor);
        assert!(calls_to(&e, REACT_TO_CONTACT).is_empty());
        assert!(calls_to(&e, IMPACT_DAMAGE).is_empty());
    }

    #[test]
    fn the_contact_list_head_is_inside_the_controller() {
        let mut e = engine();
        assert_eq!(fn_008b9220(&mut e, Ptr::new(0x1000)).addr(), 0x1648);
        assert_eq!(e.call(0x008b9220, &args![0x2000u32]).u32(), 0x2648);
    }

    // ---- 008b9240 ---------------------------------------------------------

    /// Callback setup: the owner's value for any index is `value`, virtual
    /// `+0x360` answers `limb_actor`, and the player is `player`.
    fn condition_setup(value: f32, limb_actor: bool, player: u32) -> (Engine, Ptr<Actor>) {
        let mut e = engine();
        let actor = new_actor(&mut e);
        e.mem.set_u32(PLAYER_POINTER, player);
        slot(&mut e, OWNER_TABLE, 0xc, st0(value));
        slot(&mut e, ACTOR_TABLE, 0x360, eax(limb_actor as u32));
        slot(&mut e, ACTOR_TABLE, 0x394, Ret::default());
        slot(&mut e, ACTOR_TABLE, 0x22c, eax(0));
        quiet(
            &mut e,
            &[
                SURGERY_MENU_RESET_LIMB,
                SHOW_CRIPPLED_GUY,
                TRIGGER_PAIN,
                MISC_STAT_INCREMENT,
                REFRESH_AFTER_LIMB_CHANGE,
                REQUEST_LOCOMOTION_CHANGE,
            ],
        );
        cached_helpers(&mut e);
        (e, actor)
    }

    #[test]
    fn crippling_a_limb_of_the_player_shows_the_hud_effect() {
        let (mut e, actor) = condition_setup(0.0, true, 0);
        e.mem.set_u32(PLAYER_POINTER, actor.addr());
        e.call_log = Some(vec![]);
        actor_condition_modified_callback(&mut e, owner(actor), 0x19, 20.0, -20.0, Ptr::new(0));
        assert_eq!(calls_to(&e, SURGERY_MENU_RESET_LIMB), [vec![0x19]]);
        assert_eq!(calls_to(&e, SHOW_CRIPPLED_GUY).len(), 1);
        assert_eq!(calls_to(&e, TRIGGER_PAIN), [vec![actor.addr(), 1, 0]]);
        assert_eq!(calls_to(&e, MISC_STAT_INCREMENT), [vec![0x1e]]);
        assert!(calls_to(&e, REFRESH_AFTER_LIMB_CHANGE).is_empty());
    }

    #[test]
    fn a_limb_that_keeps_some_condition_is_not_crippled() {
        let (mut e, actor) = condition_setup(5.0, true, 0);
        e.call_log = Some(vec![]);
        actor_condition_modified_callback(&mut e, owner(actor), 0x19, 20.0, -15.0, Ptr::new(0));
        assert!(calls_to(&e, SHOW_CRIPPLED_GUY).is_empty());
        // Index 0x19 with a change flags the cached values (the process has
        // none here, so nothing is forwarded).
        assert!(calls_to(&e, PROCESS_FORWARD_FLAG_MASK).is_empty());
    }

    fn message_setup() -> (Engine, Ptr<Actor>, Ptr<Actor>) {
        let (mut e, actor) = condition_setup(0.0, false, 0);
        let player = new_actor(&mut e);
        e.mem.set_u32(PLAYER_POINTER, player.addr());
        e.mem.set_u32(HUD_MAIN_MENU_POINTER, 0x6000);
        e.mem
            .set_u32(BASE_TABLE + 0x180, slot_target(BASE_TABLE, 0x180));
        double(&mut e, slot_target(BASE_TABLE, 0x180), eax(0x7100));
        double(&mut e, GET_BASE_FORM, eax(BASE_TABLE_OBJECT));
        e.mem.set_u32(BASE_TABLE_OBJECT, BASE_TABLE);
        double(&mut e, GET_BODY_PART, eax(0x7200));
        double(&mut e, BODY_PART_NAME, eax(0x7300));
        quiet(
            &mut e,
            &[COPY_STRING, BUILD_LIMB_TEXT, FORMAT_STRING, SHOW_MESSAGE],
        );
        double(&mut e, ACTOR_NAME, eax(0x7400));
        (e, actor, player)
    }

    /// An object that has the base form vtable.
    const BASE_TABLE_OBJECT: u32 = 0x0200_8400;

    #[test]
    fn the_player_crippling_another_actors_head_shows_a_message() {
        let (mut e, actor, player) = message_setup();
        e.call_log = Some(vec![]);
        // Index 0x1f also asks the actor's virtual +0x394 with 100.0.
        actor_condition_modified_callback(&mut e, owner(actor), 0x1f, 20.0, -20.0, owner(player));
        assert_eq!(
            calls_to(&e, slot_target(ACTOR_TABLE, 0x394)),
            [vec![actor.addr(), 0, 100.0f32.to_bits()]]
        );
        assert_eq!(calls_to(&e, GET_BODY_PART), [vec![0x7100, 0x1f]]);
        let text = calls_to(&e, BUILD_LIMB_TEXT);
        assert_eq!(text[0][0..2], [0x1f, 0x7300]);
        assert_eq!(text[0][3], 200);
        let format = calls_to(&e, FORMAT_STRING);
        assert_eq!(format[0][1..4], [500, FORMAT_TWO_STRINGS, 0x7400]);
        let shown = calls_to(&e, SHOW_MESSAGE);
        assert_eq!(shown.len(), 1);
        assert_eq!(shown[0][0], 0x6000);
        assert_eq!(shown[0][2..5], [1, ICON_VERY_HAPPY, SOUND_POPUP_GENERAL]);
        assert_eq!(f32::from_bits(shown[0][5]), 2.0);
    }

    #[test]
    fn no_message_when_someone_else_caused_it_or_the_hud_is_missing() {
        let (mut e, actor, player) = message_setup();
        e.call_log = Some(vec![]);
        // The source is not the player.
        actor_condition_modified_callback(&mut e, owner(actor), 0x1f, 20.0, -20.0, owner(actor));
        assert!(calls_to(&e, SHOW_MESSAGE).is_empty());
        // No HUD.
        e.mem.set_u32(HUD_MAIN_MENU_POINTER, 0);
        actor_condition_modified_callback(&mut e, owner(actor), 0x1a, 20.0, -20.0, owner(player));
        assert!(calls_to(&e, SHOW_MESSAGE).is_empty());
        // Virtual +0x22c answering true.
        e.mem.set_u32(HUD_MAIN_MENU_POINTER, 0x6000);
        slot(&mut e, ACTOR_TABLE, 0x22c, eax(1));
        actor_condition_modified_callback(&mut e, owner(actor), 0x1a, 20.0, -20.0, owner(player));
        assert!(calls_to(&e, SHOW_MESSAGE).is_empty());
        // A head with a body part of data that has no part still messages.
        slot(&mut e, ACTOR_TABLE, 0x22c, eax(0));
        double(&mut e, GET_BODY_PART, eax(0));
        actor_condition_modified_callback(&mut e, owner(actor), 0x1a, 20.0, -20.0, owner(player));
        assert_eq!(calls_to(&e, SHOW_MESSAGE).len(), 1);
        assert_eq!(calls_to(&e, BUILD_LIMB_TEXT)[0][1], 0);
    }

    #[test]
    fn mobility_value_queues_the_replacement_locomotion() {
        let (mut e, actor) = condition_setup(0.0, false, 0);
        // Both legs crippled; the value went from 3 to 0.
        e.register(GET_BODY_PART_CONDITION, |_, _| st0(0.0));
        e.call_log = Some(vec![]);
        actor_condition_modified_callback(&mut e, owner(actor), 0x48, 3.0, -3.0, Ptr::new(0));
        // delta = 0 - 1 < 0 with sum 2.0 and b == 0: requests (actor, 1, 0),
        // once for each leg.
        assert_eq!(
            calls_to(&e, REQUEST_LOCOMOTION_CHANGE),
            [vec![actor.addr(), 1, 0], vec![actor.addr(), 1, 0]]
        );
        assert_eq!(
            calls_to(&e, REFRESH_AFTER_LIMB_CHANGE),
            [vec![actor.addr()]]
        );
    }

    #[test]
    fn mobility_value_restored_from_zero_queues_the_other_request() {
        let (mut e, actor) = condition_setup(7.0, false, 0);
        e.register(GET_BODY_PART_CONDITION, |_, a| {
            st0(if a[1] == 0x1d { 0.0 } else { 9.0 })
        });
        e.call_log = Some(vec![]);
        actor_condition_modified_callback(&mut e, owner(actor), 0x48, 0.0, 7.0, Ptr::new(0));
        // delta = 1 - 0 > 0, sum 1 < 2, b != 0 and c > 0: (actor, 0, 0),
        // once: only the left leg condition is not above zero.
        assert_eq!(
            calls_to(&e, REQUEST_LOCOMOTION_CHANGE),
            [vec![actor.addr(), 0, 0]]
        );
    }

    #[test]
    fn perception_changes_forward_the_cached_flag() {
        let (mut e, actor) = condition_setup(9.0, false, 0);
        let masks = cached_helpers(&mut e);
        let process = with_process(&mut e, actor);
        let cached = e.mem.alloc(0x48);
        e.mem.set_u32(process + 0x2c, cached);
        actor_condition_modified_callback(&mut e, owner(actor), 0x19, 9.0, 1.0, Ptr::new(0));
        assert_eq!(*masks.borrow(), [0x100]);
        // A zero change forwards nothing.
        actor_condition_modified_callback(&mut e, owner(actor), 0x19, 9.0, 0.0, Ptr::new(0));
        assert_eq!(*masks.borrow(), [0x100]);
    }

    #[test]
    fn a_null_owner_does_nothing() {
        let (mut e, _) = condition_setup(0.0, true, 0);
        e.call_log = Some(vec![]);
        actor_condition_modified_callback(&mut e, Ptr::new(0), 0x19, 1.0, -1.0, Ptr::new(0));
        assert!(logged(&e).is_empty());
    }

    // ---- 008b9790 and the cached-value family ---------------------------

    /// An actor whose process has a cached-values block; returns the
    /// `CachedValuesOwner` pointer and the block.
    fn cached_actor(e: &mut Engine) -> (Ptr<Actor>, Ptr, u32) {
        let actor = new_actor(e);
        let process = with_process(e, actor);
        let cached = e.mem.alloc(0x48);
        e.mem.set_u32(process + 0x2c, cached);
        cached_helpers(e);
        let cache_owner = Ptr::new(actor.addr() + 0xa8);
        (actor, cache_owner, cached)
    }

    #[test]
    fn perception_condition_is_cached() {
        let mut e = engine();
        let (_, cache_owner, cached) = cached_actor(&mut e);
        slot(&mut e, OWNER_TABLE, 0xc, st0(0.75));
        e.call_log = Some(vec![]);
        assert_eq!(fn_008b9790(&mut e, cache_owner), 0.75);
        assert_eq!(calls_to(&e, slot_target(OWNER_TABLE, 0xc)).len(), 1);
        assert_eq!(calls_to(&e, slot_target(OWNER_TABLE, 0xc))[0][1], 0x19);
        assert_eq!(e.mem.f32(cached + 0x28), 0.75);
        assert_eq!(e.mem.u32(cached + 0x44), 0x100);
    }

    #[test]
    fn cached_values_are_not_stored_without_a_block() {
        let mut e = engine();
        let actor = new_actor(&mut e);
        with_process(&mut e, actor);
        cached_helpers(&mut e);
        slot(&mut e, OWNER_TABLE, 0xc, st0(0.75));
        let cache_owner = Ptr::new(actor.addr() + 0xa8);
        assert_eq!(fn_008b9790(&mut e, cache_owner), 0.75);
        // Without a process either.
        let bare = new_actor(&mut e);
        let bare_owner = Ptr::new(bare.addr() + 0xa8);
        assert_eq!(fn_008b9790(&mut e, bare_owner), 0.75);
    }

    #[test]
    fn the_stat_virtuals_store_into_their_fields() {
        // (function, field offset, flag mask, source of the value)
        type Getter = fn(&mut Engine, Ptr) -> f32;
        let cases: [(Getter, u32, u32, Option<u32>); 6] = [
            (fn_008b9790, 0x28, 0x100, Some(0x19)),
            (fn_008b9d20, 0x1c, 0x20, Some(0x2f)),
            (fn_008b9e10, 0x20, 0x40, Some(0xf)),
            (fn_008b9ff0, 0x24, 0x80, Some(0xf)),
            (fn_008b9b70, 0x14, 0x10, None),
            (fn_008b9c10, 0x18, 0x8000_0000, None),
        ];
        for (getter, field, mask, index) in cases {
            let mut e = engine();
            let (_, cache_owner, cached) = cached_actor(&mut e);
            slot(&mut e, OWNER_TABLE, 0xc, st0(0.5));
            double(&mut e, CACHED_VALUE_MEDICINE, st0(1.5));
            double(&mut e, CACHED_VALUE_SURVIVAL, st0(2.5));
            e.call_log = Some(vec![]);
            let value = getter(&mut e, cache_owner);
            let expected = match index {
                Some(_) => 0.5,
                None if field == 0x14 => 1.5,
                None => 2.5,
            };
            assert_eq!(value, expected, "field {field:#x}");
            assert_eq!(e.mem.f32(cached + field), expected, "field {field:#x}");
            assert_eq!(e.mem.u32(cached + 0x44), mask, "field {field:#x}");
            if let Some(index) = index {
                assert_eq!(calls_to(&e, slot_target(OWNER_TABLE, 0xc))[0][1], index);
            }
        }
    }

    #[test]
    fn the_medicine_value_gets_the_owner_pointer() {
        let mut e = engine();
        let (actor, cache_owner, _) = cached_actor(&mut e);
        double(&mut e, CACHED_VALUE_MEDICINE, st0(1.5));
        e.call_log = Some(vec![]);
        fn_008b9b70(&mut e, cache_owner);
        assert_eq!(
            calls_to(&e, CACHED_VALUE_MEDICINE),
            [vec![actor.addr() + 0xa4]]
        );
    }

    #[test]
    fn the_setters_store_the_given_value() {
        type Setter = fn(&mut Engine, Ptr, f32);
        let cases: [(Setter, u32, u32); 6] = [
            (fn_008b97f0, 0x28, 0x100),
            (fn_008b9be0, 0x14, 0x10),
            (fn_008b9c80, 0x18, 0x8000_0000),
            (fn_008b9d80, 0x1c, 0x20),
            (fn_008b9e70, 0x20, 0x40),
            (fn_008ba050, 0x24, 0x80),
        ];
        for (setter, field, mask) in cases {
            let mut e = engine();
            let actor = new_actor(&mut e);
            let process = with_process(&mut e, actor);
            cached_helpers(&mut e);
            // Without a block nothing happens.
            setter(&mut e, Ptr::new(process), 3.5);
            let cached = e.mem.alloc(0x48);
            e.mem.set_u32(process + 0x2c, cached);
            setter(&mut e, Ptr::new(process), 3.5);
            assert_eq!(e.mem.f32(cached + field), 3.5);
            assert_eq!(e.mem.u32(cached + 0x44), mask);
        }
    }

    // ---- the mask callbacks ---------------------------------------------

    #[test]
    fn mask_callbacks_forward_their_masks_to_the_process() {
        type Callback = fn(&mut Engine, Ptr);
        let cases: [(Callback, u32); 6] = [
            (fn_008b9ab0, 0x10),
            (fn_008b9b10, 0x8000_0000),
            (fn_008b9cc0, 0x20),
            (fn_008b9db0, 0x40),
            (fn_008ba140, 0x400),
            (fn_008ba210, 0x800),
        ];
        for (callback, mask) in cases {
            let mut e = engine();
            let (actor, _, _) = cached_actor(&mut e);
            // 008ba140 also touches the combat controller and the process.
            slot(&mut e, ACTOR_TABLE, 0x428, eax(0));
            let process = e.mem.u32(actor.addr() + 0x68);
            double(&mut e, ACTOR_PROCESS, eax(process));
            slot(&mut e, PROCESS_TABLE, 0x68, Ret::default());
            slot(&mut e, PROCESS_TABLE, 0x6c, Ret::default());
            let masks = cached_helpers(&mut e);
            callback(&mut e, Ptr::new(0));
            assert!(masks.borrow().is_empty());
            callback(&mut e, owner(actor));
            assert_eq!(*masks.borrow(), [mask], "mask {mask:#x}");
        }
    }

    #[test]
    fn mask_callbacks_need_a_process_with_a_block() {
        let mut e = engine();
        let actor = new_actor(&mut e);
        let masks = cached_helpers(&mut e);
        fn_008b9ab0(&mut e, owner(actor));
        assert!(masks.borrow().is_empty());
        with_process(&mut e, actor);
        fn_008b9ab0(&mut e, owner(actor));
        assert!(masks.borrow().is_empty());
    }

    #[test]
    fn combat_controller_work_before_the_0x400_mask() {
        let mut e = engine();
        let (actor, _, _) = cached_actor(&mut e);
        let process = e.mem.u32(actor.addr() + 0x68);
        double(&mut e, ACTOR_PROCESS, eax(process));
        slot(&mut e, ACTOR_TABLE, 0x428, eax(0x5500));
        slot(&mut e, PROCESS_TABLE, 0x68, Ret::default());
        slot(&mut e, PROCESS_TABLE, 0x6c, Ret::default());
        double(&mut e, ACTOR_COMBAT_CONTROLLER_STATE, eax(0x5600));
        double(&mut e, COMBAT_CONTROLLER_APPLY, Ret::default());
        e.call_log = Some(vec![]);
        fn_008ba140(&mut e, owner(actor));
        let addresses = logged_addresses(&e);
        assert_eq!(calls_to(&e, COMBAT_CONTROLLER_APPLY), [vec![0x5600]]);
        assert_eq!(
            calls_to(&e, slot_target(PROCESS_TABLE, 0x68)),
            [vec![process, actor.addr()]]
        );
        assert_eq!(
            calls_to(&e, slot_target(PROCESS_TABLE, 0x6c)),
            [vec![process, actor.addr()]]
        );
        assert!(addresses.contains(&PROCESS_FORWARD_FLAG_MASK));
    }

    // ---- 008b9830 / 008b9960 / 008b9ea0 / 008ba090 ----------------------

    #[test]
    fn mobility_callback_for_other_values_only_runs_the_general_one() {
        let (mut e, actor) = condition_setup(5.0, false, 0);
        e.call_log = Some(vec![]);
        actor_mobility_condition_modified_callback(
            &mut e,
            owner(actor),
            0x19,
            1.0,
            1.0,
            Ptr::new(0),
        );
        assert!(calls_to(&e, REFRESH_BEFORE_QUEUE).is_empty());
        actor_mobility_condition_modified_callback(
            &mut e,
            Ptr::new(0),
            0x1d,
            1.0,
            1.0,
            Ptr::new(0),
        );
        assert!(calls_to(&e, REFRESH_BEFORE_QUEUE).is_empty());
    }

    #[test]
    fn mobility_callback_queues_with_the_other_legs_condition() {
        let (mut e, actor) = condition_setup(5.0, false, 0);
        slot(&mut e, OWNER_TABLE, 0x8, eax(0));
        quiet(&mut e, &[REFRESH_BEFORE_QUEUE]);
        // Left leg (0x1d) condition 0, right leg (0x1e) condition 4.
        e.register(GET_BODY_PART_CONDITION, |_, a| {
            st0(if a[1] == 0x1d { 0.0 } else { 4.0 })
        });
        e.call_log = Some(vec![]);
        // Index 0x1d: flag is 1.0 for a positive previous value; current = 0,
        // other = 4: delta = 0 - 1 < 0, sum = 1 + 4 = 5: no request.
        actor_mobility_condition_modified_callback(
            &mut e,
            owner(actor),
            0x1d,
            3.0,
            -3.0,
            Ptr::new(0),
        );
        assert_eq!(calls_to(&e, REFRESH_BEFORE_QUEUE), [vec![actor.addr()]]);
        assert!(calls_to(&e, REQUEST_LOCOMOTION_CHANGE).is_empty());
        let conditions = calls_to(&e, GET_BODY_PART_CONDITION);
        assert_eq!(conditions[0][1], 0x1e);
        // Index 0x1e with the other leg at 0: current = 4 > 0 when the flag
        // is 0.0 (previous 0): delta = 4 > 0, sum = 0 + 0 < 2, b != 0, c = 0
        // is not > 0: no request.
        actor_mobility_condition_modified_callback(
            &mut e,
            owner(actor),
            0x1e,
            0.0,
            1.0,
            Ptr::new(0),
        );
        assert!(calls_to(&e, REQUEST_LOCOMOTION_CHANGE).is_empty());
    }

    #[test]
    fn mobility_callback_stops_when_the_owner_answers_non_zero() {
        let (mut e, actor) = condition_setup(5.0, false, 0);
        slot(&mut e, OWNER_TABLE, 0x8, eax(1));
        quiet(&mut e, &[REFRESH_BEFORE_QUEUE]);
        e.call_log = Some(vec![]);
        actor_mobility_condition_modified_callback(
            &mut e,
            owner(actor),
            0x1d,
            3.0,
            -3.0,
            Ptr::new(0),
        );
        assert_eq!(
            calls_to(&e, slot_target(OWNER_TABLE, 0x8)),
            [vec![owner(actor).addr(), 0x48]]
        );
        assert!(calls_to(&e, REFRESH_BEFORE_QUEUE).is_empty());
    }

    fn limb_flag_setup(current: f32) -> (Engine, Ptr<Actor>) {
        let mut e = engine();
        let actor = new_actor(&mut e);
        with_process(&mut e, actor);
        slot(&mut e, OWNER_TABLE, 0x20, st0(current));
        double(&mut e, PROCESS_SET_STATE_FLAG, Ret::default());
        (e, actor)
    }

    #[test]
    fn dropping_below_the_sum_sets_the_state_flag() {
        for (index, mask) in [(0x10, 0x2000_0000u32), (0x16, 0x4000_0000)] {
            let (mut e, actor) = limb_flag_setup(8.0);
            let process = e.mem.u32(actor.addr() + 0x68);
            e.call_log = Some(vec![]);
            // 5 + -1 = 4 < 8 with a negative change.
            fn_008b9960(&mut e, owner(actor), index, 5.0, -1.0);
            assert_eq!(
                calls_to(&e, PROCESS_SET_STATE_FLAG),
                [vec![process, mask, 1]]
            );
            assert_eq!(
                calls_to(&e, slot_target(OWNER_TABLE, 0x20)),
                [vec![owner(actor).addr(), index as u32]]
            );
        }
    }

    #[test]
    fn rising_to_the_sum_clears_the_state_flag() {
        let (mut e, actor) = limb_flag_setup(4.0);
        let process = e.mem.u32(actor.addr() + 0x68);
        e.call_log = Some(vec![]);
        // 3 + 1 = 4 and the value is 4 (not above): positive change clears.
        fn_008b9960(&mut e, owner(actor), 0x10, 3.0, 1.0);
        assert_eq!(
            calls_to(&e, PROCESS_SET_STATE_FLAG),
            [vec![process, 0x2000_0000, 0]]
        );
    }

    #[test]
    fn other_indices_or_no_process_do_nothing_for_the_state_flag() {
        let (mut e, actor) = limb_flag_setup(8.0);
        e.call_log = Some(vec![]);
        fn_008b9960(&mut e, owner(actor), 0x11, 5.0, -1.0);
        fn_008b9960(&mut e, Ptr::new(0), 0x10, 5.0, -1.0);
        // A zero change is neither.
        fn_008b9960(&mut e, owner(actor), 0x10, 5.0, 0.0);
        e.set(actor, Actor::pCurrentProcess, Ptr::new(0));
        fn_008b9960(&mut e, owner(actor), 0x10, 5.0, -1.0);
        assert!(calls_to(&e, PROCESS_SET_STATE_FLAG).is_empty());
    }

    fn endurance_setup(first: f32, second: f32, third: f32, health: u32) -> (Engine, Ptr<Actor>) {
        let mut e = engine();
        let actor = new_actor(&mut e);
        let process = with_process(&mut e, actor);
        let cached = e.mem.alloc(0x48);
        e.mem.set_u32(process + 0x2c, cached);
        cached_helpers(&mut e);
        slot(&mut e, OWNER_TABLE, 0x14, st0(first));
        slot(&mut e, OWNER_TABLE, 0x18, st0(second));
        slot(&mut e, OWNER_TABLE, 0x10, st0(third));
        double(&mut e, ENDURANCE_ADJUSTMENT, st0(-3.0));
        double(&mut e, GET_BASE_FORM, eax(0x7000));
        double(&mut e, GET_BASE_HEALTH, eax(health));
        double(&mut e, RESTORE_ACTOR_VALUE, Ret::default());
        (e, actor)
    }

    #[test]
    fn endurance_loss_restores_the_missing_health() {
        // value -2, health 10, +0x18 = 4, +0x10 = 1, adjustment -3: the sum
        // is 1 + (4 + 10) - 2 - 3 = 10 > 0: nothing to restore.
        let (mut e, actor) = endurance_setup(-2.0, 4.0, 1.0, 10);
        e.call_log = Some(vec![]);
        actor_endurance_modified_callback(&mut e, owner(actor), 0, 10.0, -5.0);
        assert!(calls_to(&e, RESTORE_ACTOR_VALUE).is_empty());
        assert_eq!(
            calls_to(&e, ENDURANCE_ADJUSTMENT),
            [vec![owner(actor).addr(), 5.0f32.to_bits()]]
        );
        // health -20: the sum is 1 + (4 - 20) - 2 - 3 = -20: restore 21.
        let (mut e, actor) = endurance_setup(-2.0, 4.0, 1.0, (-20i32) as u32);
        e.call_log = Some(vec![]);
        actor_endurance_modified_callback(&mut e, owner(actor), 0, 10.0, -5.0);
        assert_eq!(
            calls_to(&e, RESTORE_ACTOR_VALUE),
            [vec![actor.addr(), 0x10, 21.0f32.to_bits()]]
        );
    }

    #[test]
    fn endurance_callback_flags_the_cache_and_needs_a_loss() {
        let (mut e, actor) = endurance_setup(-2.0, 4.0, 1.0, 10);
        let masks = cached_helpers(&mut e);
        e.call_log = Some(vec![]);
        // A gain: only the flag is forwarded.
        actor_endurance_modified_callback(&mut e, owner(actor), 0, 10.0, 5.0);
        assert_eq!(*masks.borrow(), [0x80]);
        assert!(calls_to(&e, ENDURANCE_ADJUSTMENT).is_empty());
        // A loss while actor value 0x10 is not negative: no adjustment.
        slot(&mut e, OWNER_TABLE, 0x14, st0(0.0));
        actor_endurance_modified_callback(&mut e, owner(actor), 0, 10.0, -5.0);
        assert!(calls_to(&e, ENDURANCE_ADJUSTMENT).is_empty());
        actor_endurance_modified_callback(&mut e, Ptr::new(0), 0, 10.0, -5.0);
        assert_eq!(masks.borrow().len(), 2);
    }

    fn locomotion(a: f32, b: f32, c: f32) -> Vec<Vec<u32>> {
        let mut e = engine();
        let actor = new_actor(&mut e);
        double(&mut e, REQUEST_LOCOMOTION_CHANGE, Ret::default());
        e.call_log = Some(vec![]);
        actor_queue_replacement_locomotion(&mut e, actor, a, b, c);
        calls_to(&e, REQUEST_LOCOMOTION_CHANGE)
            .into_iter()
            .map(|call| call[1..].to_vec())
            .collect()
    }

    #[test]
    fn replacement_locomotion_requests() {
        // delta < 0, sum == 2.0, b == 0.
        assert_eq!(locomotion(1.0, 0.0, 1.0), [vec![1, 0]]);
        // delta < 0 but sum != 2.0.
        assert!(locomotion(1.0, 0.0, 0.5).is_empty());
        // delta < 0 with b != 0.
        assert!(locomotion(1.0, 0.5, 1.0).is_empty());
        // delta > 0, sum < 2.0, b != 0, c > 0.
        assert_eq!(locomotion(0.0, 1.0, 1.0), [vec![0, 0]]);
        // c not positive, sum not below 2.0 or delta zero.
        assert!(locomotion(0.0, 1.0, 0.0).is_empty());
        assert!(locomotion(1.5, 2.0, 1.0).is_empty());
        assert!(locomotion(1.0, 1.0, 1.0).is_empty());
    }

    // ---- 008ba270 / 008ba2b0 / 008ba2f0 ----------------------------------

    #[test]
    fn simple_actor_callbacks() {
        let mut e = engine();
        let actor = new_actor(&mut e);
        slot(&mut e, ACTOR_TABLE, 0x3c4, Ret::default());
        double(&mut e, ACTOR_CALLBACK_008C4400, Ret::default());
        e.call_log = Some(vec![]);
        fn_008ba270(&mut e, owner(actor));
        fn_008ba270(&mut e, Ptr::new(0));
        fn_008ba2b0(&mut e, owner(actor));
        fn_008ba2b0(&mut e, Ptr::new(0));
        assert_eq!(
            logged(&e),
            [
                (slot_target(ACTOR_TABLE, 0x3c4), vec![actor.addr()]),
                (ACTOR_CALLBACK_008C4400, vec![actor.addr()]),
            ]
        );
    }

    fn surprise_setup(
        limb_actor: bool,
        answered: bool,
        god_mode: bool,
        value: f32,
    ) -> (Engine, Ptr<Actor>) {
        let mut e = engine();
        let actor = new_actor(&mut e);
        let player = e.mem.alloc(8);
        e.mem.set_u32(player, PLAYER_TABLE);
        e.mem.set_u32(PLAYER_POINTER, player);
        slot(&mut e, ACTOR_TABLE, 0x360, eax(limb_actor as u32));
        slot(&mut e, PLAYER_TABLE, 0x358, eax(answered as u32));
        double(&mut e, PLAYER_IS_GOD_MODE, eax(god_mode as u32));
        double(&mut e, PLAYER_VALUE_008A0C20, st0(value));
        double(&mut e, MESSAGE_QUEUE_ADD, eax(0x5a5a));
        double(&mut e, MESSAGE_QUEUE_SHOW, Ret::default());
        (e, actor)
    }

    #[test]
    fn the_surprised_message_needs_the_player_answer_and_a_big_enough_value() {
        let (mut e, actor) = surprise_setup(true, true, false, 10.0);
        e.call_log = Some(vec![]);
        fn_008ba2f0(&mut e, owner(actor), 0, 5.0);
        assert_eq!(calls_to(&e, MESSAGE_QUEUE_ADD), [vec![MESSAGE_QUEUE]]);
        assert_eq!(
            calls_to(&e, MESSAGE_QUEUE_SHOW),
            [vec![0x5a5a, 0, ICON_SURPRISED, 0, 2.0f32.to_bits(), 0]]
        );
        // The value is below the threshold: no message.
        let (mut e, actor) = surprise_setup(true, true, false, 3.0);
        e.call_log = Some(vec![]);
        fn_008ba2f0(&mut e, owner(actor), 0, 5.0);
        assert!(calls_to(&e, MESSAGE_QUEUE_SHOW).is_empty());
        // God mode makes the low value irrelevant.
        let (mut e, actor) = surprise_setup(true, true, true, 3.0);
        e.call_log = Some(vec![]);
        fn_008ba2f0(&mut e, owner(actor), 0, 5.0);
        assert_eq!(calls_to(&e, MESSAGE_QUEUE_SHOW).len(), 1);
        // The player did not answer.
        let (mut e, actor) = surprise_setup(true, false, false, 10.0);
        e.call_log = Some(vec![]);
        fn_008ba2f0(&mut e, owner(actor), 0, 5.0);
        assert!(calls_to(&e, MESSAGE_QUEUE_SHOW).is_empty());
        // The actor fails its test: the player is not even asked.
        let (mut e, actor) = surprise_setup(false, true, false, 10.0);
        e.call_log = Some(vec![]);
        fn_008ba2f0(&mut e, owner(actor), 0, 5.0);
        fn_008ba2f0(&mut e, Ptr::new(0), 0, 5.0);
        assert!(calls_to(&e, slot_target(PLAYER_TABLE, 0x358)).is_empty());
    }

    // ---- the process getters and idle tests ------------------------------

    #[test]
    fn process_flag_getters() {
        type Getter = fn(&mut Engine, Ptr<Actor>) -> bool;
        let cases: [(Getter, u32); 4] = [
            (fn_008ba3e0, 0x6f4),
            (fn_008ba410, 0x6ec),
            (actor_is_heavy_body_armor_worn, 0x6e8),
            (fn_008ba470, 0x6f0),
        ];
        for (getter, offset) in cases {
            let mut e = engine();
            let actor = new_actor(&mut e);
            assert!(!getter(&mut e, actor));
            with_process(&mut e, actor);
            slot(&mut e, PROCESS_TABLE, offset, eax(0));
            assert!(!getter(&mut e, actor));
            slot(&mut e, PROCESS_TABLE, offset, eax(1));
            assert!(getter(&mut e, actor));
        }
    }

    fn queued(flags: u32) -> bool {
        let mut e = engine();
        let actor = new_actor(&mut e);
        with_process(&mut e, actor);
        slot(&mut e, PROCESS_TABLE, 0x618, eax(flags));
        actor_is_anim_idle_quequed(&mut e, actor)
    }

    #[test]
    fn anim_idle_queued_tests_ten_bits() {
        assert!(!queued(0));
        for bit in [
            0x4, 0x8, 0x10, 0x40, 0x80, 0x100, 0x400, 0x800, 0x1000, 0x2000,
        ] {
            assert!(queued(bit), "bit {bit:#x}");
            assert!(queued(bit | 0x1), "bit {bit:#x}");
        }
        for bit in [0x1, 0x2, 0x20, 0x200, 0x4000, 0x8000, 0x1_0000] {
            assert!(!queued(bit), "bit {bit:#x}");
        }
    }

    fn lower_body_case(
        queued_flags: u32,
        slot_a: u32,
        special_playing: bool,
        slot_b: u32,
        group_type: u32,
    ) -> bool {
        let mut e = engine();
        let actor = new_actor(&mut e);
        with_process(&mut e, actor);
        let animation = e.mem.alloc(0x200);
        slot(&mut e, ACTOR_TABLE, 0x1e4, eax(animation));
        slot(&mut e, PROCESS_TABLE, 0x618, eax(queued_flags));
        e.register(POINTER_GET, |e, a| eax(e.mem.u32(a[0])));
        e.mem.set_u32(animation + 0x128, slot_a);
        e.mem.set_u32(animation + 0x124, slot_b);
        double(
            &mut e,
            ANIMATION_SPECIAL_IDLE_PLAYING,
            eax(special_playing as u32),
        );
        e.register_double(ANIM_GROUP_TYPE, move |_, a| {
            eax(if a[0] == 0x10 || a[0] == 0x20 {
                group_type
            } else {
                99
            })
        });
        actor_is_playing_lower_body_special_idle(&mut e, actor)
    }

    #[test]
    fn lower_body_special_idle_by_group_type() {
        // Queued idle with a group in slot +0x128.
        for (group_type, expected) in [
            (1, true),
            (7, true),
            (0x14, true),
            (2, false),
            (0xffff_ffff, false),
        ] {
            assert_eq!(lower_body_case(4, 0x10, false, 0, group_type), expected);
        }
        // Not queued: the special idle slot +0x124 counts when playing.
        assert!(lower_body_case(0, 0x10, true, 0x20, 7));
        assert!(!lower_body_case(0, 0x10, false, 0x20, 7));
        // Queued but the +0x128 slot is empty falls through to +0x124.
        assert!(lower_body_case(4, 0, true, 0x20, 1));
        // Nothing found.
        assert!(!lower_body_case(0, 0, true, 0, 1));
    }

    #[test]
    fn lower_body_special_idle_needs_an_animation() {
        let mut e = engine();
        let actor = new_actor(&mut e);
        with_process(&mut e, actor);
        slot(&mut e, ACTOR_TABLE, 0x1e4, eax(0));
        assert!(!actor_is_playing_lower_body_special_idle(&mut e, actor));
    }

    // ---- 008ba600 --------------------------------------------------------

    const ANIMATION_OBJECT: u32 = 0x0200_a400;

    /// An actor with a process (pending flags `flags`) and an animation, and
    /// doubles for every callee of the handler that the blocks reach.
    fn handler_setup(flags: u32) -> (Engine, Ptr<Actor>, u32) {
        let mut e = engine();
        let actor = new_actor(&mut e);
        let process = with_process(&mut e, actor);
        e.mem.set_u32(ANIMATION_OBJECT, ANIMATION_TABLE);
        e.set(actor, Actor::eQueuedattack, 0xff);
        slot(&mut e, ACTOR_TABLE, 0x1e4, eax(ANIMATION_OBJECT));
        slot(&mut e, ACTOR_TABLE, 0x22c, eax(0));
        slot(&mut e, ACTOR_TABLE, 0x4b0, Ret::default());
        slot(&mut e, ACTOR_TABLE, 0x3ec, Ret::default());
        let process_slots: [u32; 17] = [
            0x61c, 0x294, 0x3f4, 0xd8, 0x614, 0x2b0, 0x2b8, 0x708, 0x70c, 0x718, 0x720, 0x728,
            0x148, 0x14c, 0x3e8, 0x404, 0x4d0,
        ];
        for offset in process_slots {
            slot(&mut e, PROCESS_TABLE, offset, Ret::default());
        }
        slot(&mut e, PROCESS_TABLE, 0x618, eax(flags));
        slot(&mut e, PROCESS_TABLE, 0x3f0, eax(0));
        slot(&mut e, PROCESS_TABLE, 0x444, st0(0.0));
        slot(&mut e, PROCESS_TABLE, 0x454, eax(0));
        slot(&mut e, PROCESS_TABLE, 0x724, eax(0));
        slot(&mut e, PROCESS_TABLE, 0x27c, eax(0));
        e.register(POINTER_GET, |e, a| eax(e.mem.u32(a[0])));
        e.register(ENTRY_OBJECT, |e, a| eax(e.mem.u32(a[0] + 8)));
        e.register(ACTOR_PROCESS, |e, a| eax(e.mem.u32(a[0] + 0x68)));
        quiet(
            &mut e,
            &[
                PLAY_GROUP,
                GET_OUT_OF_FURNITURE_QUICK,
                ACTOR_HANDLE_FLAG_1000,
                ACTOR_HANDLE_FLAG_400,
                ACTOR_HANDLE_FLAG_8000,
                SET_HAVOK_WEAPON,
                EXPEL_SHELL_CASING,
                SET_IRON_SIGHTS,
                START_ATTACK,
            ],
        );
        double(&mut e, GET_ANIM_GROUP, eax(0x77));
        (e, actor, process)
    }

    #[test]
    fn handler_without_a_process_or_animation_does_nothing() {
        let (mut e, actor, _) = handler_setup(0x20);
        e.set(actor, Actor::pCurrentProcess, Ptr::new(0));
        e.call_log = Some(vec![]);
        fn_008ba600(&mut e, actor);
        assert!(logged(&e).is_empty());
        let (mut e, actor, _) = handler_setup(0x20);
        slot(&mut e, ACTOR_TABLE, 0x1e4, eax(0));
        e.call_log = Some(vec![]);
        fn_008ba600(&mut e, actor);
        // Only the two process virtuals that were read first.
        assert_eq!(
            logged_addresses(&e),
            [
                slot_target(ACTOR_TABLE, 0x1e4),
                slot_target(PROCESS_TABLE, 0x618),
                slot_target(PROCESS_TABLE, 0x61c),
            ]
        );
    }

    #[test]
    fn handler_plays_group_0xe0_for_flag_0x20_and_stops_on_virtual_0x22c() {
        let (mut e, actor, process) = handler_setup(0x20 | 0x1000);
        slot(&mut e, ACTOR_TABLE, 0x22c, eax(1));
        e.call_log = Some(vec![]);
        fn_008ba600(&mut e, actor);
        assert_eq!(
            calls_to(&e, GET_ANIM_GROUP),
            [vec![actor.addr(), 0xe0, 0, 0, 0]]
        );
        assert_eq!(
            calls_to(&e, PLAY_GROUP),
            [vec![ANIMATION_OBJECT, 0x77, 1, 0xffff_ffff, 0xffff_ffff]]
        );
        assert_eq!(
            calls_to(&e, slot_target(PROCESS_TABLE, 0x294)),
            [vec![process, actor.addr()]]
        );
        // Stopped before the 0x1000 block.
        assert!(calls_to(&e, ACTOR_HANDLE_FLAG_1000).is_empty());
    }

    #[test]
    fn handler_simple_flag_blocks() {
        let (mut e, actor, process) =
            handler_setup(0x1000 | 0x400 | 0x8000 | 0x4000 | 0x200 | 0x10);
        e.call_log = Some(vec![]);
        fn_008ba600(&mut e, actor);
        assert_eq!(calls_to(&e, ACTOR_HANDLE_FLAG_1000), [vec![actor.addr()]]);
        assert_eq!(calls_to(&e, ACTOR_HANDLE_FLAG_400), [vec![actor.addr()]]);
        assert_eq!(calls_to(&e, ACTOR_HANDLE_FLAG_8000), [vec![actor.addr()]]);
        assert_eq!(calls_to(&e, SET_HAVOK_WEAPON), [vec![actor.addr()]]);
        assert_eq!(
            calls_to(&e, GET_OUT_OF_FURNITURE_QUICK),
            [vec![actor.addr()]]
        );
        assert_eq!(
            calls_to(&e, slot_target(PROCESS_TABLE, 0x70c)),
            [vec![process, actor.addr()]]
        );
    }

    #[test]
    fn handler_queued_attack_is_started_and_cleared() {
        let (mut e, actor, process) = handler_setup(0);
        e.set(actor, Actor::eQueuedattack, 3);
        // The process says the attack is over (virtual +0x3f0 false).
        e.call_log = Some(vec![]);
        fn_008ba600(&mut e, actor);
        assert_eq!(calls_to(&e, START_ATTACK), [vec![actor.addr(), 3]]);
        assert_eq!(
            calls_to(&e, slot_target(PROCESS_TABLE, 0x3f4)),
            [vec![process, 0]]
        );
        assert_eq!(e.get(actor, Actor::eQueuedattack), 0xff);
        // Still going: virtual +0x3f0 true and a non-zero value.
        e.set(actor, Actor::eQueuedattack, 3);
        slot(&mut e, PROCESS_TABLE, 0x3f0, eax(1));
        slot(&mut e, PROCESS_TABLE, 0x444, st0(1.5));
        e.call_log = Some(vec![]);
        fn_008ba600(&mut e, actor);
        assert!(calls_to(&e, slot_target(PROCESS_TABLE, 0x3f4)).is_empty());
        assert_eq!(e.get(actor, Actor::eQueuedattack), 3);
        // True with a zero value ends it as well.
        slot(&mut e, PROCESS_TABLE, 0x444, st0(0.0));
        fn_008ba600(&mut e, actor);
        assert_eq!(e.get(actor, Actor::eQueuedattack), 0xff);
    }

    #[test]
    fn handler_flag_0x800_clears_the_pending_bit_or_calls_the_process() {
        // The special idle is working: clear the pending bit.
        let (mut e, actor, process) = handler_setup(0x800);
        double(&mut e, SPECIAL_IDLE_WORKING, eax(1));
        e.call_log = Some(vec![]);
        fn_008ba600(&mut e, actor);
        assert_eq!(
            calls_to(&e, slot_target(PROCESS_TABLE, 0x614)),
            [vec![process, 0x800]]
        );
        // Not working: the process's virtual +0xd8 gets the actor.
        let (mut e, actor, process) = handler_setup(0x800);
        double(&mut e, SPECIAL_IDLE_WORKING, eax(0));
        e.call_log = Some(vec![]);
        fn_008ba600(&mut e, actor);
        assert_eq!(
            calls_to(&e, slot_target(PROCESS_TABLE, 0xd8)),
            [vec![process, actor.addr()]]
        );
        assert!(calls_to(&e, slot_target(PROCESS_TABLE, 0x614)).is_empty());
    }

    #[test]
    fn handler_furniture_blocks_clear_flag_0x10() {
        // 0x4 with a small value and a refusing virtual: the process gets +0x708(0)
        // and the 0x10 block no longer runs.
        let (mut e, actor, process) = handler_setup(0x4 | 0x10);
        slot(&mut e, PROCESS_TABLE, 0x4d0, eax(0xc8));
        slot(&mut e, PROCESS_TABLE, 0x2b0, eax(0));
        e.call_log = Some(vec![]);
        fn_008ba600(&mut e, actor);
        assert_eq!(
            calls_to(&e, slot_target(PROCESS_TABLE, 0x2b0)),
            [vec![process, actor.addr()]]
        );
        assert_eq!(
            calls_to(&e, slot_target(PROCESS_TABLE, 0x708)),
            [vec![process, 0]]
        );
        assert!(calls_to(&e, slot_target(PROCESS_TABLE, 0x70c)).is_empty());
        // A value above 0xc8 leaves the furniture quickly.
        let (mut e, actor, _) = handler_setup(0x4);
        slot(&mut e, PROCESS_TABLE, 0x4d0, eax(0xc9));
        e.call_log = Some(vec![]);
        fn_008ba600(&mut e, actor);
        assert_eq!(
            calls_to(&e, GET_OUT_OF_FURNITURE_QUICK),
            [vec![actor.addr()]]
        );
        // 0x8 asks +0x2b8 and asks +0x708 when it refuses.
        let (mut e, actor, process) = handler_setup(0x8 | 0x10);
        slot(&mut e, PROCESS_TABLE, 0x2b8, eax(0));
        e.call_log = Some(vec![]);
        fn_008ba600(&mut e, actor);
        assert_eq!(
            calls_to(&e, slot_target(PROCESS_TABLE, 0x708)),
            [vec![process, 0]]
        );
        assert!(calls_to(&e, slot_target(PROCESS_TABLE, 0x70c)).is_empty());
        let (mut e, actor, _) = handler_setup(0x8);
        slot(&mut e, PROCESS_TABLE, 0x2b8, eax(1));
        e.call_log = Some(vec![]);
        fn_008ba600(&mut e, actor);
        assert!(calls_to(&e, slot_target(PROCESS_TABLE, 0x708)).is_empty());
    }

    #[test]
    fn handler_flag_0x80_plays_an_idle() {
        let (mut e, actor, _) = handler_setup(0x80);
        slot(&mut e, PROCESS_TABLE, 0x718, eax(0x6100));
        double(&mut e, GET_IDLE_ANIM_GROUP_SECTION, eax(0x6200));
        double(&mut e, PLAY_IDLE_SECTION, Ret::default());
        e.call_log = Some(vec![]);
        fn_008ba600(&mut e, actor);
        assert_eq!(calls_to(&e, GET_IDLE_ANIM_GROUP_SECTION), [vec![0x6100]]);
        assert_eq!(
            calls_to(&e, PLAY_IDLE_SECTION),
            [vec![ANIMATION_OBJECT, 0x6100, actor.addr(), 0x6200, 3]]
        );
    }

    #[test]
    fn handler_flag_0x2000_loads_each_queued_idle() {
        let (mut e, actor, _) = handler_setup(0x2000);
        // Each pass asks twice (the loop test and the item), so four answers
        // make two passes.
        let remaining = Rc::new(RefCell::new(4));
        let counter = remaining.clone();
        let target = slot_target(PROCESS_TABLE, 0x724);
        e.register_double(target, move |_, _| {
            let mut left = counter.borrow_mut();
            if *left == 0 {
                eax(0)
            } else {
                *left -= 1;
                eax(0x6300)
            }
        });
        slot(&mut e, PROCESS_TABLE, 0x728, eax(0x6400));
        double(&mut e, IDLE_POINTER_CONSTRUCTOR, Ret::default());
        double(&mut e, IDLE_POINTER_DESTRUCTOR, Ret::default());
        double(&mut e, ANIM_IDLE_LOADED, Ret::default());
        e.register(POINTER_GET, |_, _| eax(0x6500));
        e.call_log = Some(vec![]);
        fn_008ba600(&mut e, actor);
        assert_eq!(
            calls_to(&e, ANIM_IDLE_LOADED),
            [vec![0x6500, 0x6400], vec![0x6500, 0x6400]]
        );
        assert_eq!(calls_to(&e, IDLE_POINTER_CONSTRUCTOR).len(), 2);
        assert_eq!(calls_to(&e, IDLE_POINTER_DESTRUCTOR).len(), 2);
        assert_eq!(calls_to(&e, slot_target(PROCESS_TABLE, 0x720)).len(), 2);
    }

    #[test]
    fn handler_flag_0x40_and_0x100_play_groups() {
        // 0x40 with the process flag set and no weapon in hand.
        let (mut e, actor, _) = handler_setup(0x40);
        slot(&mut e, PROCESS_TABLE, 0x454, eax(1));
        e.call_log = Some(vec![]);
        fn_008ba600(&mut e, actor);
        assert_eq!(
            calls_to(&e, GET_ANIM_GROUP),
            [vec![actor.addr(), 0, 0, 1, ANIMATION_OBJECT]]
        );
        assert_eq!(
            calls_to(&e, PLAY_GROUP),
            [vec![ANIMATION_OBJECT, 0x77, 1, 0xffff_ffff, 0xffff_ffff]]
        );
        // Without the process flag the entry is -1 and the flag 0.
        let (mut e, actor, _) = handler_setup(0x40);
        e.call_log = Some(vec![]);
        fn_008ba600(&mut e, actor);
        assert_eq!(
            calls_to(&e, GET_ANIM_GROUP),
            [vec![actor.addr(), 0, 0xffff_ffff, 0, ANIMATION_OBJECT]]
        );
        // 0x100 needs a current weapon.
        let (mut e, actor, _) = handler_setup(0x100);
        double(&mut e, GET_CURRENT_WEAPON, eax(0));
        e.call_log = Some(vec![]);
        fn_008ba600(&mut e, actor);
        assert!(calls_to(&e, GET_ANIM_GROUP).is_empty());
        double(&mut e, GET_CURRENT_WEAPON, eax(0x6600));
        double(&mut e, CURRENT_WEAPON_ANIM_TYPE, eax(0x6700));
        let entry = e.mem.alloc(0x20);
        slot(&mut e, PROCESS_TABLE, 0x148, eax(entry));
        fn_008ba600(&mut e, actor);
        assert_eq!(
            calls_to(&e, GET_ANIM_GROUP),
            [vec![actor.addr(), 0x6700, entry, 0, ANIMATION_OBJECT]]
        );
    }

    #[test]
    fn handler_flag_0x20000_expels_a_shell_casing_and_0x10000_sets_iron_sights() {
        let (mut e, actor, _) = handler_setup(0x20000 | 0x10000);
        let entry = e.mem.alloc(0x20);
        e.mem.set_u32(entry + 8, 0x6900);
        slot(&mut e, PROCESS_TABLE, 0x148, eax(entry));
        slot(&mut e, PROCESS_TABLE, 0x404, eax(1));
        e.call_log = Some(vec![]);
        fn_008ba600(&mut e, actor);
        assert_eq!(
            calls_to(&e, EXPEL_SHELL_CASING),
            [vec![0x6900, actor.addr()]]
        );
        assert_eq!(
            calls_to(&e, slot_target(PROCESS_TABLE, 0x404)),
            [vec![e.mem.u32(actor.addr() + 0x68)]]
        );
        assert_eq!(calls_to(&e, SET_IRON_SIGHTS), [vec![actor.addr(), 1, 1, 0]]);
        // No weapon in hand: no shell casing.
        slot(&mut e, PROCESS_TABLE, 0x148, eax(0));
        e.call_log = Some(vec![]);
        fn_008ba600(&mut e, actor);
        assert!(calls_to(&e, EXPEL_SHELL_CASING).is_empty());
    }

    fn equip_setup(slot_locations: [u32; 3]) -> (Engine, Ptr<Actor>, u32) {
        let (mut e, actor, process) = handler_setup(0x2);
        let entry = e.mem.alloc(0x20);
        e.mem.set_u32(entry + 8, 0x6a00);
        slot(&mut e, PROCESS_TABLE, 0x148, eax(entry));
        e.register(ANIMATION_SLOT, |_, a| eax(0x1000 + a[1]));
        e.register_double(GET_GENERIC_LOCATION, move |_, a| {
            let slot_index = a[0] - 0x1000;
            let position = [0u32, 1, 4].iter().position(|s| *s == slot_index).unwrap();
            eax(slot_locations[position])
        });
        double(&mut e, HAS_MOD_EFFECT_ACTIVE, eax(1));
        (e, actor, process)
    }

    #[test]
    fn handler_flag_0x2_equips_the_weapon_when_the_slots_are_generic() {
        let (mut e, actor, _) = equip_setup([1, 1, 1]);
        e.call_log = Some(vec![]);
        fn_008ba600(&mut e, actor);
        assert_eq!(
            calls_to(&e, slot_target(ACTOR_TABLE, 0x3ec)),
            [vec![actor.addr(), 0x6a00, 2, 1, 0]]
        );
        assert!(calls_to(&e, slot_target(PROCESS_TABLE, 0x614)).is_empty());
    }

    #[test]
    fn handler_flag_0x2_clears_the_pending_bit_when_a_slot_is_busy() {
        for busy in 0..3 {
            let mut locations = [1, 1, 1];
            locations[busy] = 5;
            let (mut e, actor, process) = equip_setup(locations);
            e.call_log = Some(vec![]);
            fn_008ba600(&mut e, actor);
            assert_eq!(
                calls_to(&e, slot_target(PROCESS_TABLE, 0x614)),
                [vec![process, 2]],
                "slot {busy}"
            );
            assert!(calls_to(&e, slot_target(ACTOR_TABLE, 0x3ec)).is_empty());
        }
    }

    fn weapon_form_setup_case(test_passes: bool) -> Engine {
        let (mut e, actor, _) = handler_setup(0x1);
        let entry = e.mem.alloc(0x20);
        e.mem.set_u32(entry + 8, 0x6b00);
        slot(&mut e, PROCESS_TABLE, 0x148, eax(entry));
        double(&mut e, WEAPON_FORM_TEST, eax(test_passes as u32));
        double(&mut e, WEAPON_FORM_PREPARE, Ret::default());
        double(&mut e, HAS_MOD_EFFECT_ACTIVE, eax(1));
        double(&mut e, WEAPON_FORM_STATE, eax(0x6c00));
        double(&mut e, ACTOR_APPLY_WEAPON_STATE, Ret::default());
        double(&mut e, ACTOR_STATE_TEST_493BB0, eax(0));
        double(&mut e, ACTOR_UPDATE_00899200, Ret::default());
        double(&mut e, OBJECT_TYPE, eax(8));
        double(&mut e, LIST_NODE_NEXT, eax(0));
        double(&mut e, SET_ACTION_FLAG, Ret::default());
        e.call_log = Some(vec![]);
        fn_008ba600(&mut e, actor);
        e
    }

    #[test]
    fn handler_flag_0x1_sets_up_a_weapon_form() {
        let e = weapon_form_setup_case(true);
        assert_eq!(calls_to(&e, WEAPON_FORM_PREPARE).len(), 1);
        assert_eq!(
            calls_to(&e, HAS_MOD_EFFECT_ACTIVE)
                .iter()
                .map(|c| c[1])
                .collect::<Vec<_>>(),
            [0xb, 0x10]
        );
        let state = calls_to(&e, WEAPON_FORM_STATE);
        assert_eq!(state[0][1..], [1, 1]);
        assert_eq!(calls_to(&e, ACTOR_APPLY_WEAPON_STATE)[0][1], 0x6c00);
        assert!(calls_to(&e, ACTOR_UPDATE_00899200).is_empty());
    }

    #[test]
    fn handler_flag_0x1_updates_the_actor_for_other_forms() {
        // The form test fails; the process object is not of type 8 or 0x10:
        // update with (0, 0) when the object exists, else (0, 0) as well.
        let e = weapon_form_setup_case(false);
        assert!(calls_to(&e, WEAPON_FORM_PREPARE).is_empty());
        // The slot +0x27c answers 0, so the else branch (0, 0).
        assert_eq!(calls_to(&e, ACTOR_UPDATE_00899200)[0][1..], [0, 0]);
    }

    #[test]
    fn handler_flag_0x1_sets_the_action_flag_for_forms_with_a_count() {
        let (mut e, actor, _) = handler_setup(0x1);
        let entry = e.mem.alloc(0x20);
        e.mem.set_u32(entry, 0x1500);
        e.mem.set_u32(entry + 8, 0x6b00);
        let held_data = e.mem.alloc(8);
        e.mem.set_u32(held_data, 0xabcd);
        slot(&mut e, PROCESS_TABLE, 0x148, eax(entry));
        double(&mut e, WEAPON_FORM_TEST, eax(1));
        double(&mut e, WEAPON_FORM_PREPARE, Ret::default());
        double(&mut e, HAS_MOD_EFFECT_ACTIVE, eax(0));
        double(&mut e, WEAPON_FORM_STATE, eax(0));
        double(&mut e, ACTOR_APPLY_WEAPON_STATE, Ret::default());
        double(&mut e, LIST_NODE_NEXT, eax(1));
        double(&mut e, LIST_NODE_DATA_ADDRESS, eax(held_data));
        double(&mut e, SET_ACTION_FLAG, Ret::default());
        e.call_log = Some(vec![]);
        fn_008ba600(&mut e, actor);
        assert_eq!(
            calls_to(&e, SET_ACTION_FLAG),
            [vec![0x6b00, 0xabcd, 0x40_0000]]
        );
    }

    #[test]
    fn handler_flag_0x1_non_weapon_branches() {
        for (is_player, state_test, object, object_type, expected) in [
            (true, 0, 0, 0, [0, 1]),
            (false, 1, 0, 0, [0, 1]),
            (false, 0, 0x5000, 8, [0, 1]),
            (false, 0, 0x5000, 0x10, [0, 1]),
            (false, 0, 0x5000, 3, [0, 0]),
            (false, 0, 0, 0, [0, 0]),
        ] {
            let (mut e, actor, _) = handler_setup(0x1);
            let entry = e.mem.alloc(0x20);
            e.mem.set_u32(entry + 8, 0x6b00);
            slot(&mut e, PROCESS_TABLE, 0x148, eax(entry));
            slot(&mut e, PROCESS_TABLE, 0x27c, eax(object));
            if is_player {
                e.mem.set_u32(PLAYER_POINTER, actor.addr());
            }
            double(&mut e, WEAPON_FORM_TEST, eax(0));
            double(&mut e, ACTOR_STATE_TEST_493BB0, eax(state_test));
            double(&mut e, OBJECT_TYPE, eax(object_type));
            double(&mut e, ACTOR_UPDATE_00899200, Ret::default());
            double(&mut e, LIST_NODE_NEXT, eax(0));
            e.call_log = Some(vec![]);
            fn_008ba600(&mut e, actor);
            assert_eq!(calls_to(&e, ACTOR_UPDATE_00899200)[0][1..], expected);
        }
    }

    /// Equip or reload setup: a weapon and ammunition in hand.
    fn reload_setup(
        state: u32,
        clip: i32,
        ammo_count: i32,
        ammo_index_minus_one: i32,
    ) -> (Engine, Ptr<Actor>, u32) {
        let (mut e, actor, process) = handler_setup(0x40000);
        let weapon_entry = e.mem.alloc(0x20);
        e.mem.set_u32(weapon_entry + 8, 0x6d00);
        let ammo_entry = e.mem.alloc(0x20);
        e.mem.set_u32(ammo_entry + 8, 0x6e00);
        slot(&mut e, PROCESS_TABLE, 0x148, eax(weapon_entry));
        slot(&mut e, PROCESS_TABLE, 0x14c, eax(ammo_entry));
        slot(&mut e, PROCESS_TABLE, 0x3e8, eax(0x6f00));
        double(&mut e, ANIMATION_STEP, eax(0x7000));
        double(&mut e, ANIMATION_KIND, eax(2));
        double(&mut e, GET_FLAG_WORD, eax(state));
        double(&mut e, HAS_MOD_EFFECT_ACTIVE, eax(1));
        double(&mut e, GET_FORM_CLIP_ROUNDS, eax(clip as u32));
        double(&mut e, GET_INVENTORY_CHANGES, eax(0x7100));
        double(&mut e, GET_OBJECT_COUNT, eax(ammo_count as u32));
        double(&mut e, LIST_NODE_NEXT, eax(ammo_index_minus_one as u32));
        double(&mut e, QUEUE_EQUIP_OBJECT, Ret::default());
        double(&mut e, WEAPON_ANIM_TYPE, eax(5));
        double(&mut e, ANIMATION_SLOT, eax(0x7200));
        double(&mut e, SET_ANIM_ACTION, Ret::default());
        double(&mut e, PACK_ANIM_GROUP, eax(0x33));
        double(&mut e, PICK_BEST_ANIMATION, eax(0x44));
        double(&mut e, ANIM_GROUP_GET_TYPE, eax(5));
        double(&mut e, PLAYER_GET_ANIMATION, eax(ANIMATION_OBJECT));
        for offset in [0x6f4u32, 0x6ec] {
            slot(&mut e, PROCESS_TABLE, offset, eax(0));
        }
        // The equipped form: the weapon entry's object.
        (e, actor, process)
    }

    #[test]
    fn reload_when_the_clip_is_short_and_ammunition_is_left() {
        // clip 10, ammunition entry count 3 (+1 = 4 <= 5 in the inventory).
        let (mut e, actor, _) = reload_setup(0, 10, 5, 3);
        e.call_log = Some(vec![]);
        fn_008ba600(&mut e, actor);
        assert_eq!(
            calls_to(&e, QUEUE_EQUIP_OBJECT),
            [vec![actor.addr(), 0x6e00, 4, 0, 1, 0, 0]]
        );
        assert_eq!(calls_to(&e, WEAPON_ANIM_TYPE), [vec![0x6d00, 0]]);
        assert_eq!(
            calls_to(&e, SET_ANIM_ACTION),
            [vec![actor.addr(), 0x11, 0x7200]]
        );
        assert_eq!(
            calls_to(&e, PLAY_GROUP),
            [vec![ANIMATION_OBJECT, 0x77, 1, 1, 4]]
        );
        assert!(calls_to(&e, PACK_ANIM_GROUP).is_empty());
    }

    #[test]
    fn picks_the_best_animation_when_there_is_nothing_to_reload() {
        // Ammunition entry count 3 + 1 = 4 and 4 in the inventory: nothing to
        // reload, and the clip rounds are 4 as well.
        let (mut e, actor, _) = reload_setup(0x800 | 0x200 | 0x2, 4, 4, 3);
        e.call_log = Some(vec![]);
        fn_008ba600(&mut e, actor);
        // Slot kind 2, group index 8 (flag 0x2 with 0x200).
        assert_eq!(calls_to(&e, PACK_ANIM_GROUP), [vec![2, 0, 8, 0]]);
        assert_eq!(
            calls_to(&e, PICK_BEST_ANIMATION),
            [vec![ANIMATION_OBJECT, 0x33, 0]]
        );
        let plays = calls_to(&e, PLAY_GROUP);
        assert_eq!(plays[0], [ANIMATION_OBJECT, 0x44, 1, 1, 4]);
        // Not the player: the weapon type group is played with the process
        // animation too and the action set to 9.
        assert_eq!(plays.len(), 2);
        assert_eq!(
            calls_to(&e, SET_ANIM_ACTION),
            [vec![actor.addr(), 9, 0x7200]]
        );
        assert_eq!(
            calls_to(&e, slot_target(ACTOR_TABLE, 0x4b0)),
            [vec![actor.addr(), 0x77, 1]]
        );
    }

    #[test]
    fn the_player_gets_its_own_animation_and_resets_the_action() {
        let (mut e, actor, _) = reload_setup(0x2000 | 0x4, 4, 4, 3);
        e.mem.set_u32(PLAYER_POINTER, actor.addr());
        e.call_log = Some(vec![]);
        fn_008ba600(&mut e, actor);
        // Slot kind 3, group index 5.
        assert_eq!(calls_to(&e, PACK_ANIM_GROUP), [vec![3, 0, 5, 0]]);
        assert_eq!(calls_to(&e, PLAYER_GET_ANIMATION), [vec![actor.addr(), 0]]);
        assert_eq!(
            calls_to(&e, SET_ANIM_ACTION),
            [vec![actor.addr(), 0xffff_ffff, 0]]
        );
        assert_eq!(
            calls_to(&e, slot_target(ACTOR_TABLE, 0x4b0)),
            [vec![actor.addr(), 0x44, 0]]
        );
    }

    #[test]
    fn group_index_and_slot_kind_follow_the_actor_flag_word() {
        // (flag word, slot kind, group index)
        for (state, kind, group) in [
            (0x400u32, 1u32, 0xffu32),
            (0x800 | 0x1, 2, 3),
            (0x2000 | 0x2, 3, 4),
            (0x4, 0, 5),
            (0x8, 0, 6),
            (0x200 | 0x1, 0, 7),
            (0x200 | 0x2, 0, 8),
            (0x200 | 0x4, 0, 9),
            (0x200 | 0x8, 0, 10),
            (0x200, 0, 0xff),
            (0x800 | 0x2000, 2, 0xff),
        ] {
            let (mut e, actor, _) = reload_setup(state, 4, 4, 3);
            e.call_log = Some(vec![]);
            fn_008ba600(&mut e, actor);
            assert_eq!(
                calls_to(&e, PACK_ANIM_GROUP)[0][0..3],
                [kind, 0, group],
                "state {state:#x}"
            );
        }
    }

    #[test]
    fn heavy_armor_or_the_other_flag_getter_sets_the_pack_flag() {
        for (slot_a, slot_b, expected) in [(0, 0, 0), (1, 0, 1), (0, 1, 1)] {
            let (mut e, actor, _) = reload_setup(0, 4, 4, 3);
            slot(&mut e, PROCESS_TABLE, 0x6f4, eax(slot_a));
            slot(&mut e, PROCESS_TABLE, 0x6ec, eax(slot_b));
            e.call_log = Some(vec![]);
            fn_008ba600(&mut e, actor);
            assert_eq!(calls_to(&e, PACK_ANIM_GROUP)[0][3], expected);
        }
    }

    #[test]
    fn no_animation_is_played_when_nothing_is_picked() {
        let (mut e, actor, _) = reload_setup(0, 4, 4, 3);
        double(&mut e, PICK_BEST_ANIMATION, eax(0xff));
        e.call_log = Some(vec![]);
        fn_008ba600(&mut e, actor);
        assert!(calls_to(&e, PLAY_GROUP).is_empty());
    }

    // ---- 008bb520 .. 008bb5c0 -------------------------------------------

    fn rotate_setup(can_move: bool, flag_0x28: u32, flag_0x2ac: u32) -> (Engine, Ptr<Actor>) {
        let mut e = engine();
        let actor = new_actor(&mut e);
        let process = with_process(&mut e, actor);
        e.mem.set_u32(process + 0x2ac, flag_0x2ac);
        e.set(actor, Actor::pActorMover, Ptr::new(0x7700));
        double(&mut e, ACTOR_CAN_MOVE, eax(can_move as u32));
        e.register(ACTOR_PROCESS, |e, a| eax(e.mem.u32(a[0] + 0x68)));
        double(&mut e, PROCESS_FLAG_0X28, eax(flag_0x28));
        double(&mut e, REQUEST_ROTATE_ACTOR, eax(1));
        double(&mut e, REQUEST_ROTATE_ACTOR_OV2, eax(0));
        (e, actor)
    }

    #[test]
    fn rotate_request_goes_to_the_actor_mover() {
        let (mut e, actor) = rotate_setup(true, 0, 0);
        e.call_log = Some(vec![]);
        assert!(fn_008bb520(&mut e, actor, 1.0, 2.0, 3.0, true));
        assert_eq!(
            calls_to(&e, REQUEST_ROTATE_ACTOR),
            [vec![
                0x7700,
                1.0f32.to_bits(),
                2.0f32.to_bits(),
                3.0f32.to_bits(),
                1
            ]]
        );
        assert!(!fn_008bb5c0(&mut e, actor, 0.5, false));
        assert_eq!(
            calls_to(&e, REQUEST_ROTATE_ACTOR_OV2),
            [vec![0x7700, 0.5f32.to_bits(), 0]]
        );
    }

    #[test]
    fn rotate_request_is_answered_true_when_the_actor_cannot_move_or_the_process_blocks() {
        // Cannot move.
        let (mut e, actor) = rotate_setup(false, 0, 0);
        e.call_log = Some(vec![]);
        assert!(fn_008bb520(&mut e, actor, 1.0, 2.0, 3.0, true));
        assert!(fn_008bb5c0(&mut e, actor, 0.5, false));
        assert!(calls_to(&e, REQUEST_ROTATE_ACTOR).is_empty());
        assert!(calls_to(&e, REQUEST_ROTATE_ACTOR_OV2).is_empty());
        // The process blocks (flag +0x28 clear and word +0x2ac set).
        let (mut e, actor) = rotate_setup(true, 0, 9);
        e.call_log = Some(vec![]);
        assert!(fn_008bb520(&mut e, actor, 1.0, 2.0, 3.0, true));
        assert!(fn_008bb5c0(&mut e, actor, 0.5, false));
        assert!(calls_to(&e, REQUEST_ROTATE_ACTOR).is_empty());
        // Flag +0x28 set: the request goes through.
        let (mut e, actor) = rotate_setup(true, 1, 9);
        e.call_log = Some(vec![]);
        assert!(fn_008bb520(&mut e, actor, 1.0, 2.0, 3.0, true));
        assert_eq!(calls_to(&e, REQUEST_ROTATE_ACTOR).len(), 1);
    }

    #[test]
    fn process_word_0x2ac() {
        let mut e = engine();
        let block = e.mem.alloc(0x300);
        e.mem.set_u32(block + 0x2ac, 0x1234);
        assert_eq!(fn_008bb5a0(&mut e, Ptr::new(block)), 0x1234);
        assert_eq!(e.call(0x008bb5a0, &args![block]).u32(), 0x1234);
    }

    // ---- 008bb630 .. 008bcda0 ------------------------------------------

    /// An actor object of `size` bytes with the actor vtables.
    fn big_actor(e: &mut Engine, size: u32) -> Ptr<Actor> {
        let addr = e.mem.alloc(size);
        e.mem.set_u32(addr, ACTOR_TABLE);
        e.mem.set_u32(addr + 0xa4, OWNER_TABLE);
        Ptr::new(addr)
    }

    /// `ACTOR_PROCESS` answering the actor's `+0x68`.
    fn process_getter(e: &mut Engine) {
        e.register(ACTOR_PROCESS, |e, a| eax(e.mem.u32(a[0] + 0x68)));
    }

    /// Doubles of the list node helpers: a node is `{data, next}`.
    fn list_nodes(e: &mut Engine) {
        e.register(LIST_NODE_DATA_ADDRESS, |_, a| eax(a[0]));
        e.register(LIST_NODE_NEXT, |e, a| eax(e.mem.u32(a[0] + 4)));
        e.register(LIST_NODE_IS_EMPTY, |e, a| {
            eax((e.mem.u32(a[0]) == 0 && e.mem.u32(a[0] + 4) == 0) as u32)
        });
    }

    /// A list node `{data, next}`.
    fn node(e: &mut Engine, data: u32, next: u32) -> u32 {
        let node = e.mem.alloc(8);
        e.mem.set_u32(node, data);
        e.mem.set_u32(node + 4, next);
        node
    }

    /// A `float` setting stored in memory: the address of its value.
    fn float_setting(e: &mut Engine, value: f32) -> u32 {
        let address = e.mem.alloc(4);
        e.mem.set_f32(address, value);
        address
    }

    #[test]
    fn set_pathfinding_flee_forwards_the_word_to_the_mover() {
        let mut e = engine();
        let actor = new_actor(&mut e);
        e.set(actor, Actor::pActorMover, Ptr::new(0x7700));
        double(&mut e, ACTOR_MOVER_SET_FLEE, eax(0));
        e.call_log = Some(vec![]);
        e.call(0x008bb630, &args![actor, 5u32]);
        assert_eq!(calls_to(&e, ACTOR_MOVER_SET_FLEE), [vec![0x7700, 5]]);
    }

    #[test]
    fn player_flag_byte_and_sighting_node_word() {
        let mut e = engine();
        let block = e.mem.alloc(0x1000);
        e.mem.set_u8(block + 0x650, 3);
        assert_eq!(fn_008bbbd0(&mut e, Ptr::new(block)), 3);
        assert_eq!(e.call(0x008bbbd0, &args![block]).u8(), 3);
        e.call(0x008bbbf0, &args![block, 0x4321u32]);
        assert_eq!(e.mem.u32(block + 0xe34), 0x4321);
        fn_008bbbf0(&mut e, Ptr::new(block), 0);
        assert_eq!(e.mem.u32(block + 0xe34), 0);
    }

    #[test]
    fn get_iron_sights_asks_the_process() {
        let mut e = engine();
        let actor = new_actor(&mut e);
        assert_eq!(actor_get_iron_sights(&mut e, actor), 0);
        with_process(&mut e, actor);
        slot(&mut e, PROCESS_TABLE, 0x404, eax(0x1201));
        assert_eq!(actor_get_iron_sights(&mut e, actor), 1);
        assert_eq!(e.call(0x008bbc10, &args![actor]).u8(), 1);
    }

    /// The doubles `008bb650` needs for a non-player actor whose animation
    /// step has the kind number `kind` (index 5 of the kind table).
    fn sights_setup(kind: u32) -> (Engine, Ptr<Actor>) {
        let mut e = engine();
        e.map(0x0118_a000, 0x1000);
        let actor = new_actor(&mut e);
        with_process(&mut e, actor);
        process_getter(&mut e);
        slot(&mut e, ACTOR_TABLE, 0x21c, eax(0));
        slot(&mut e, PROCESS_TABLE, 0x404, eax(0));
        slot(&mut e, PROCESS_TABLE, 0x400, Ret::default());
        slot(&mut e, PROCESS_TABLE, 0x148, eax(0));
        double(&mut e, ACTOR_ANIMATION, eax(0x9000));
        double(&mut e, ANIMATION_SLOT, eax(0x9100));
        double(&mut e, GET_GENERIC_LOCATION, eax(2));
        double(&mut e, ANIMATION_STEP, eax(0x9200));
        double(&mut e, ANIMATION_KIND, eax(5));
        e.mem.set_u32(ANIMATION_KIND_TABLE + 5 * 0x24, kind);
        double(&mut e, ANIM_GROUP_IS_IRON_SIGHTS, eax(0));
        double(&mut e, GET_ANIM_GROUP, eax(7));
        e.register(ANIM_GROUP_GET_TYPE, |_, _| eax(5));
        double(&mut e, ANIMATION_STEP_GROUP, eax(9));
        double(&mut e, ANIMATION_BLEND_OUT, Ret::default());
        double(&mut e, ITEM_SIGHTS_TEST_A, eax(1));
        e.register(ENTRY_OBJECT, |_, a| eax(a[0]));
        (e, actor)
    }

    #[test]
    fn set_iron_sights_does_nothing_without_a_process_or_change() {
        // No process.
        let (mut e, actor) = sights_setup(0xb);
        e.set(actor, Actor::pCurrentProcess, Ptr::new(0));
        e.call_log = Some(vec![]);
        actor_set_iron_sights(&mut e, actor, 1, 0, 0);
        assert!(logged(&e).is_empty());
        // The actor's virtual +0x21c answers true.
        let (mut e, actor) = sights_setup(0xb);
        slot(&mut e, ACTOR_TABLE, 0x21c, eax(1));
        e.call_log = Some(vec![]);
        actor_set_iron_sights(&mut e, actor, 1, 0, 0);
        assert!(calls_to(&e, slot_target(PROCESS_TABLE, 0x400)).is_empty());
        // Already in the requested state, unless forced.
        let (mut e, actor) = sights_setup(0xb);
        slot(&mut e, PROCESS_TABLE, 0x404, eax(1));
        e.call_log = Some(vec![]);
        actor_set_iron_sights(&mut e, actor, 1, 0, 0);
        assert!(calls_to(&e, slot_target(PROCESS_TABLE, 0x400)).is_empty());
        actor_set_iron_sights(&mut e, actor, 1, 1, 0);
        assert_eq!(calls_to(&e, slot_target(PROCESS_TABLE, 0x400)).len(), 1);
    }

    #[test]
    fn set_iron_sights_switches_and_stops_when_the_kind_is_not_one() {
        let (mut e, actor) = sights_setup(3);
        e.call_log = Some(vec![]);
        e.call(0x008bb650, &args![actor, 1u32, 0u32, 0u32]);
        assert_eq!(
            calls_to(&e, slot_target(PROCESS_TABLE, 0x400)),
            [vec![e.mem.u32(actor.addr() + 0x68), 1]]
        );
        assert!(calls_to(&e, GET_ANIM_GROUP).is_empty());
        // Without animation the kind is 0xb.
        double(&mut e, ACTOR_ANIMATION, eax(0));
        e.call_log = Some(vec![]);
        actor_set_iron_sights(&mut e, actor, 0, 1, 0);
        assert_eq!(calls_to(&e, slot_target(PROCESS_TABLE, 0x400)).len(), 1);
        assert!(calls_to(&e, ANIMATION_SLOT).is_empty());
    }

    #[test]
    fn set_iron_sights_kind_one_can_skip_the_switch() {
        let (mut e, actor) = sights_setup(1);
        e.call_log = Some(vec![]);
        actor_set_iron_sights(&mut e, actor, 1, 0, 1);
        assert!(calls_to(&e, slot_target(PROCESS_TABLE, 0x400)).is_empty());
        // The kind is not read when the generic location is 3.
        double(&mut e, GET_GENERIC_LOCATION, eax(3));
        e.call_log = Some(vec![]);
        actor_set_iron_sights(&mut e, actor, 1, 0, 1);
        assert_eq!(calls_to(&e, slot_target(PROCESS_TABLE, 0x400)).len(), 1);
    }

    #[test]
    fn set_iron_sights_kind_one_reselects_the_animation_group() {
        // Group 5 as is: group 7 picked, current group 9: blend out.
        let (mut e, actor) = sights_setup(1);
        e.call_log = Some(vec![]);
        actor_set_iron_sights(&mut e, actor, 1, 0, 0);
        let anim = |e: &Engine| calls_to(e, GET_ANIM_GROUP);
        assert_eq!(anim(&e), [vec![actor.addr(), 5, 0, 0, 0x9000]]);
        assert_eq!(calls_to(&e, ANIMATION_BLEND_OUT), [vec![0x9000, 4, 1]]);
        // The picked group is the current one: no blend out.
        double(&mut e, ANIMATION_STEP_GROUP, eax(7));
        e.call_log = Some(vec![]);
        actor_set_iron_sights(&mut e, actor, 1, 0, 0);
        assert!(calls_to(&e, ANIMATION_BLEND_OUT).is_empty());
        // The group type differs from the requested one: no blend out.
        double(&mut e, ANIMATION_STEP_GROUP, eax(9));
        e.register(ANIM_GROUP_GET_TYPE, |_, _| eax(6));
        e.call_log = Some(vec![]);
        actor_set_iron_sights(&mut e, actor, 1, 0, 0);
        assert!(calls_to(&e, ANIMATION_BLEND_OUT).is_empty());
        // An iron sights action takes 3 off the group.
        double(&mut e, ANIM_GROUP_IS_IRON_SIGHTS, eax(1));
        e.call_log = Some(vec![]);
        actor_set_iron_sights(&mut e, actor, 1, 0, 0);
        assert_eq!(calls_to(&e, GET_ANIM_GROUP)[0][1], 2);
    }

    #[test]
    fn set_iron_sights_adds_three_back_for_the_sights_state() {
        // Sights up, nothing held: +3.
        let (mut e, actor) = sights_setup(1);
        slot(&mut e, PROCESS_TABLE, 0x404, eax(1));
        e.call_log = Some(vec![]);
        actor_set_iron_sights(&mut e, actor, 1, 1, 0);
        assert_eq!(calls_to(&e, GET_ANIM_GROUP)[0][1], 8);
        // Something held and the test passes: +3; fails: +0.
        slot(&mut e, PROCESS_TABLE, 0x148, eax(0x6000));
        e.call_log = Some(vec![]);
        actor_set_iron_sights(&mut e, actor, 1, 1, 0);
        assert_eq!(calls_to(&e, GET_ANIM_GROUP)[0][1], 8);
        assert_eq!(calls_to(&e, ITEM_SIGHTS_TEST_A), [vec![0x6000]]);
        double(&mut e, ITEM_SIGHTS_TEST_A, eax(0));
        e.call_log = Some(vec![]);
        actor_set_iron_sights(&mut e, actor, 1, 1, 0);
        assert_eq!(calls_to(&e, GET_ANIM_GROUP)[0][1], 5);
    }

    /// `sights_setup` for the player (`+0xe34` and `+0x650` need room).
    fn player_sights_setup(kind: u32) -> (Engine, Ptr<Actor>) {
        let (mut e, _) = sights_setup(kind);
        let player = big_actor(&mut e, 0x1000);
        with_process(&mut e, player);
        e.set_global(PLAYER_POINTER, player.addr());
        let setting = e.mem.alloc(4);
        e.mem.set_u8(setting, 1);
        double(&mut e, SETTING_BYTE_POINTER, eax(setting));
        double(&mut e, PLAYER_GET_NODE, eax(0x5000));
        double(&mut e, FIND_CHILD_BY_NAME, eax(0x5100));
        e.register(ENTRY_OBJECT, |_, a| {
            eax(if a[0] == HUD_MODE_OBJECT { 3 } else { a[0] })
        });
        double(&mut e, HUD_SET_MENU_MODE, Ret::default());
        double(&mut e, PLAYER_GET_ANIMATION, eax(0x9300));
        (e, player)
    }

    #[test]
    fn set_iron_sights_for_the_player_sets_the_sighting_node() {
        let (mut e, player) = player_sights_setup(3);
        slot(&mut e, PROCESS_TABLE, 0x454, eax(0));
        e.call_log = Some(vec![]);
        actor_set_iron_sights(&mut e, player, 1, 0, 0);
        assert_eq!(e.mem.u32(player.addr() + 0xe34), 0x5100);
        assert_eq!(calls_to(&e, PLAYER_GET_NODE), [vec![player.addr(), 1]]);
        assert_eq!(
            calls_to(&e, FIND_CHILD_BY_NAME),
            [vec![0x5000, SIGHTING_NODE_NAME]]
        );
        assert_eq!(calls_to(&e, HUD_SET_MENU_MODE), [vec![1]]);
        // Lowering clears the word.
        slot(&mut e, PROCESS_TABLE, 0x404, eax(1));
        actor_set_iron_sights(&mut e, player, 0, 0, 0);
        assert_eq!(e.mem.u32(player.addr() + 0xe34), 0);
        // The setting off leaves the word alone.
        let setting = e.mem.alloc(4);
        double(&mut e, SETTING_BYTE_POINTER, eax(setting));
        e.mem.set_u32(player.addr() + 0xe34, 7);
        actor_set_iron_sights(&mut e, player, 1, 1, 0);
        assert_eq!(e.mem.u32(player.addr() + 0xe34), 7);
    }

    #[test]
    fn set_iron_sights_for_the_player_resets_the_gun_wobble() {
        let (mut e, player) = player_sights_setup(3);
        e.mem.set_u32(WOBBLE_TABLE + 8, 0x77);
        slot(&mut e, PROCESS_TABLE, 0x454, eax(1));
        slot(&mut e, PROCESS_TABLE, 0x148, eax(0x6000));
        double(&mut e, ITEM_STEP_00504E60, eax(0x6100));
        double(&mut e, ITEM_STEP_0048CEE0, eax(1));
        double(&mut e, PLAYER_FLAG_A, eax(1));
        double(&mut e, PLAYER_FLAG_B, eax(1));
        double(&mut e, ITEM_TEST_004AD030, eax(0));
        double(&mut e, ITEM_WOBBLE_INDEX, eax(2));
        double(&mut e, CLEAR_GUN_WOBBLE, Ret::default());
        double(&mut e, FORCE_TEMP_FIRST_PERSON, eax(0));
        double(&mut e, NODE_REQUEST_00450F90, Ret::default());
        // Raising: the wobble word of the table entry, the first person view.
        e.call_log = Some(vec![]);
        actor_set_iron_sights(&mut e, player, 1, 1, 0);
        assert_eq!(calls_to(&e, CLEAR_GUN_WOBBLE), [vec![0x77, 0x5000]]);
        assert_eq!(calls_to(&e, ITEM_WOBBLE_INDEX), [vec![0x6000]]);
        assert_eq!(
            calls_to(&e, FORCE_TEMP_FIRST_PERSON),
            [vec![player.addr(), 1]]
        );
        assert_eq!(calls_to(&e, NODE_REQUEST_00450F90), [vec![0x5000, 1]]);
        // The first person request succeeded: no node request.
        double(&mut e, FORCE_TEMP_FIRST_PERSON, eax(1));
        e.call_log = Some(vec![]);
        actor_set_iron_sights(&mut e, player, 1, 1, 0);
        assert!(calls_to(&e, NODE_REQUEST_00450F90).is_empty());
        // Lowering: zero word; the node request happens when +0x650 is zero.
        e.call_log = Some(vec![]);
        actor_set_iron_sights(&mut e, player, 0, 1, 0);
        assert_eq!(calls_to(&e, CLEAR_GUN_WOBBLE), [vec![0, 0x5000]]);
        assert_eq!(calls_to(&e, NODE_REQUEST_00450F90), [vec![0x5000, 0]]);
        e.mem.set_u8(player.addr() + 0x650, 1);
        e.call_log = Some(vec![]);
        actor_set_iron_sights(&mut e, player, 0, 1, 0);
        assert!(calls_to(&e, NODE_REQUEST_00450F90).is_empty());
    }

    #[test]
    fn set_iron_sights_for_the_player_stops_at_the_failed_tests() {
        let (mut e, player) = player_sights_setup(3);
        slot(&mut e, PROCESS_TABLE, 0x454, eax(1));
        slot(&mut e, PROCESS_TABLE, 0x148, eax(0x6000));
        double(&mut e, ITEM_STEP_00504E60, eax(0x6100));
        double(&mut e, ITEM_STEP_0048CEE0, eax(0));
        double(&mut e, PLAYER_FLAG_A, eax(1));
        double(&mut e, PLAYER_FLAG_B, eax(2));
        double(&mut e, CLEAR_GUN_WOBBLE, Ret::default());
        // The object test fails.
        e.call_log = Some(vec![]);
        actor_set_iron_sights(&mut e, player, 1, 1, 0);
        assert!(calls_to(&e, PLAYER_FLAG_A).is_empty());
        // The flags differ.
        double(&mut e, ITEM_STEP_0048CEE0, eax(1));
        e.call_log = Some(vec![]);
        actor_set_iron_sights(&mut e, player, 1, 1, 0);
        assert_eq!(calls_to(&e, PLAYER_FLAG_A).len(), 1);
        assert!(calls_to(&e, CLEAR_GUN_WOBBLE).is_empty());
        // The weapon has mods and the mod effect is not active.
        double(&mut e, PLAYER_FLAG_B, eax(1));
        double(&mut e, ITEM_TEST_004AD030, eax(1));
        let seen = Rc::new(RefCell::new(Vec::new()));
        let capture = seen.clone();
        e.register_double(HAS_MOD_EFFECT_ACTIVE_VALUE, move |e, a| {
            capture.borrow_mut().push((a[1], a[2], e.mem.f32(a[2])));
            eax(0)
        });
        e.call_log = Some(vec![]);
        actor_set_iron_sights(&mut e, player, 1, 1, 0);
        let seen = seen.borrow();
        assert_eq!(seen.len(), 1);
        assert_eq!((seen[0].0, seen[0].2), (0xe, 1.0));
        assert!(calls_to(&e, CLEAR_GUN_WOBBLE).is_empty());
    }

    #[test]
    fn set_iron_sights_for_the_player_also_reselects_its_own_group() {
        let (mut e, player) = player_sights_setup(1);
        slot(&mut e, PROCESS_TABLE, 0x454, eax(0));
        slot(&mut e, PROCESS_TABLE, 0x404, eax(1));
        slot(&mut e, PROCESS_TABLE, 0x148, eax(0x6000));
        double(&mut e, ITEM_SIGHTS_TEST_B, eax(1));
        double(&mut e, ITEM_TEST_004AD030, eax(0));
        // The first person block ran first (group 5 + 3), then the player's
        // own animation (0x9300): sights up, the test passes: 5 + 3.
        e.call_log = Some(vec![]);
        actor_set_iron_sights(&mut e, player, 1, 1, 0);
        let groups = calls_to(&e, GET_ANIM_GROUP);
        assert_eq!(groups.len(), 2);
        assert_eq!(groups[1], [player.addr(), 8, 0, 0, 0x9300]);
        assert_eq!(calls_to(&e, PLAYER_GET_ANIMATION), [vec![player.addr(), 1]]);
        // The test fails: 5.
        double(&mut e, ITEM_SIGHTS_TEST_B, eax(0));
        e.call_log = Some(vec![]);
        actor_set_iron_sights(&mut e, player, 1, 1, 0);
        assert_eq!(calls_to(&e, GET_ANIM_GROUP)[1][1], 5);
        // It passes, the weapon has mods and the mod effect is active: 5.
        double(&mut e, ITEM_SIGHTS_TEST_B, eax(1));
        double(&mut e, ITEM_TEST_004AD030, eax(1));
        double(&mut e, HAS_MOD_EFFECT_ACTIVE, eax(1));
        e.call_log = Some(vec![]);
        actor_set_iron_sights(&mut e, player, 1, 1, 0);
        assert_eq!(calls_to(&e, GET_ANIM_GROUP)[1][1], 5);
        // No animation step for the player: nothing more.
        double(&mut e, PLAYER_GET_ANIMATION, eax(0));
        e.call_log = Some(vec![]);
        actor_set_iron_sights(&mut e, player, 1, 1, 0);
        assert_eq!(calls_to(&e, GET_ANIM_GROUP).len(), 1);
    }

    #[test]
    fn apply_constr_template_scales_every_element_with_a_target() {
        let mut e = engine();
        let actor = new_actor(&mut e);
        let object = e.mem.alloc(0x10);
        e.mem.set_u32(object, REFERENCE_TABLE);
        slot(&mut e, ACTOR_TABLE, 0x1d0, eax(object));
        let target = slot_target(REFERENCE_TABLE, 0x9c);
        e.mem.set_u32(REFERENCE_TABLE + 0x9c, target);
        e.register(target, |_, a| eax(if a[1] == 0x1102 { 0 } else { 0x9999 }));
        e.register(NAME_HANDLE_BUILD, |_, a| eax(a[0]));
        e.register(GET_EXTRA_DATA_BY_NAME, move |_, a| {
            eax(if a[0] == 0x4400 && a[1] != 0 {
                0x6600
            } else {
                0
            })
        });
        double(&mut e, NAME_HANDLE_RELEASE, Ret::default());
        double(&mut e, GET_SCALE, st0(1.5));
        e.register(ENTRY_OBJECT, |_, _| eax(2));
        e.register(LIST_NODE_AT, |_, a| eax(0x100 + a[1]));
        e.register(POINTER_GET, |_, a| eax(a[0] + 0x1000));
        e.register(ELEMENT_KEY, |_, a| eax(a[0] + 1));
        double(&mut e, ELEMENT_APPLY_SCALE, Ret::default());
        e.call_log = Some(vec![]);
        assert!(actor_apply_constr_template(&mut e, actor, 0x4400));
        assert_eq!(calls_to(&e, NAME_HANDLE_BUILD)[0][1], EXTRA_DATA_NAME);
        // Element 0 has a target, element 1 does not.
        assert_eq!(
            calls_to(&e, ELEMENT_APPLY_SCALE),
            [vec![0x1100, 0x9999, 1.5f32.to_bits()]]
        );
        assert_eq!(
            calls_to(&e, LIST_NODE_AT),
            [vec![0x6600 + 0xc, 0], vec![0x6600 + 0xc, 1]]
        );
        // No extra data: false, nothing applied.
        e.register(GET_EXTRA_DATA_BY_NAME, |_, _| eax(0));
        e.call_log = Some(vec![]);
        assert!(!e.call(0x008bbc40, &args![actor, 0x4400u32]).bool());
        assert!(calls_to(&e, ELEMENT_APPLY_SCALE).is_empty());
        // No object: false.
        slot(&mut e, ACTOR_TABLE, 0x1d0, eax(0));
        e.register(GET_EXTRA_DATA_BY_NAME, |_, _| eax(0x6600));
        assert!(!actor_apply_constr_template(&mut e, actor, 0x4400));
    }

    #[test]
    fn list_element_helpers_use_the_list_at_plus_0xc() {
        let mut e = engine();
        e.register(LIST_NODE_AT, |_, a| eax(a[0] * 2 + a[1]));
        e.register(POINTER_GET, |_, a| eax(a[0] + 1));
        e.register(ENTRY_OBJECT, |_, a| eax(a[0]));
        assert_eq!(fn_008bbd60(&mut e, Ptr::new(0x1000), 3), 0x201c);
        assert_eq!(e.call(0x008bbd60, &args![0x1000u32, 3u32]).u32(), 0x201c);
        assert_eq!(fn_008bbd90(&mut e, Ptr::new(0x1000)), 0x100c);
        assert_eq!(e.call(0x008bbd90, &args![0x1000u32]).u32(), 0x100c);
    }

    #[test]
    fn constants_are_stored_into_the_floats() {
        let mut e = engine();
        e.set_global(0x0101_2054, 0.5f32);
        let block = e.mem.alloc(0x200);
        e.call(0x008bbdb0, &args![block]);
        for offset in [0x178, 0x17c, 0x180, 0x184] {
            assert_eq!(e.mem.f32(block + offset), 0.5);
        }
        assert_eq!(e.mem.f32(block + 0x188), 0.0);
        e.set_global(0x0101_2054, 2.0f32);
        e.call(0x008bbe00, &args![block]);
        assert_eq!(e.mem.f32(block + 0x188), 2.0);
        assert_eq!(e.mem.f32(block + 0x178), 0.5);
    }

    /// An actor whose process has a cached-values block; the owner's
    /// virtual `+0x8` answers `value`.
    fn cached_setup(value: u32, with_block: bool) -> (Engine, Ptr<Actor>, u32) {
        let mut e = engine();
        let actor = new_actor(&mut e);
        let process = with_process(&mut e, actor);
        let block = e.mem.alloc(0x80);
        if with_block {
            e.mem.set_u32(process + 0x2c, block);
        }
        slot(&mut e, OWNER_TABLE, 0x8, eax(value));
        e.register(HAS_CACHED_VALUES, |e, a| {
            eax((e.mem.u32(a[0] + 0x2c) != 0) as u32)
        });
        double(&mut e, CACHED_VALUES_ADD_FLAGS, Ret::default());
        (e, actor, block)
    }

    #[test]
    fn cached_value_writers_store_the_owner_answer() {
        for (address, argument, offset, mask) in [
            (0x008bbe20u32, 0u32, 0x30u32, 0x400u32),
            (0x008bbeb0, 0x39, 0x34, 0x800),
        ] {
            let (mut e, actor, block) = cached_setup(7, true);
            e.call_log = Some(vec![]);
            let this = actor.addr() + 0xa8;
            assert_eq!(e.call(address, &args![this]).u32(), 7);
            assert_eq!(e.mem.u32(block + offset), 7);
            assert_eq!(
                calls_to(&e, slot_target(OWNER_TABLE, 0x8)),
                [vec![actor.addr() + 0xa4, argument]]
            );
            assert_eq!(calls_to(&e, CACHED_VALUES_ADD_FLAGS), [vec![block, mask]]);
            // No cached-values block: only the answer.
            let (mut e, actor, block) = cached_setup(7, false);
            e.call_log = Some(vec![]);
            assert_eq!(e.call(address, &args![actor.addr() + 0xa8]).u32(), 7);
            assert_eq!(e.mem.u32(block + offset), 0);
            assert!(calls_to(&e, CACHED_VALUES_ADD_FLAGS).is_empty());
        }
    }

    #[test]
    fn cached_value_setters_need_the_block() {
        let mut e = engine();
        double(&mut e, CACHED_VALUES_ADD_FLAGS, Ret::default());
        let process = e.mem.alloc(0x80);
        let block = e.mem.alloc(0x80);
        e.call_log = Some(vec![]);
        e.call(0x008bbe70, &args![process, 5u32]);
        e.call(0x008bbf00, &args![process, 6u32]);
        assert!(logged(&e).iter().all(|c| c.0 != CACHED_VALUES_ADD_FLAGS));
        e.mem.set_u32(process + 0x2c, block);
        e.call(0x008bbe70, &args![process, 5u32]);
        e.call(0x008bbf00, &args![process, 6u32]);
        assert_eq!((e.mem.u32(block + 0x30), e.mem.u32(block + 0x34)), (5, 6));
        assert_eq!(
            calls_to(&e, CACHED_VALUES_ADD_FLAGS),
            [vec![block, 0x400], vec![block, 0x800]]
        );
    }

    /// Doubles for every callee of `008bbf40`; allocations come from the
    /// memory heap, constructors answer their block.
    fn package_setup() -> (Engine, Ptr<Actor>, u32) {
        let mut e = engine();
        let actor = new_actor(&mut e);
        with_process(&mut e, actor);
        process_getter(&mut e);
        double(&mut e, PACKAGE_CREATE, eax(0x5000));
        e.register(OPERATOR_NEW, |e, a| eax(e.mem.alloc(a[0])));
        for constructor in [PACKAGE_LOCATION_CONSTRUCTOR, PACKAGE_TARGET_CONSTRUCTOR] {
            e.register(constructor, |_, a| eax(a[0]));
        }
        let data = e.mem.alloc(0x40);
        double(&mut e, PACKAGE_USE_WEAPON_DATA, eax(data));
        quiet(
            &mut e,
            &[
                PACKAGE_SET_TYPE,
                PACKAGE_SET_FLAGS_WORD,
                PACKAGE_SET_FLAG,
                PACKAGE_LOCATION_SET_REFERENCE,
                PACKAGE_SET_LOCATION,
                PACKAGE_LOCATION_DESTRUCTOR,
                PACKAGE_TARGET_SET_TYPE,
                PACKAGE_TARGET_SET_FIRST,
                PACKAGE_TARGET_SET_REFERENCE,
                PACKAGE_SET_TARGET,
                PACKAGE_TARGET_DESTRUCTOR,
                PACKAGE_TARGET_STACK_DESTRUCTOR,
                ACTOR_CURRENT_EDITOR_PACKAGE,
                PACKAGE_COPY_EDITOR,
                PACKAGE_SET_SECOND_LOCATION,
                USE_WEAPON_SET_ATTACK_TARGET,
            ],
        );
        slot(&mut e, PROCESS_TABLE, 0x28, Ret::default());
        slot(&mut e, ACTOR_TABLE, 0x2f4, Ret::default());
        (e, actor, data)
    }

    #[test]
    fn use_weapon_package_is_built_and_started() {
        let (mut e, actor, data) = package_setup();
        e.call_log = Some(vec![]);
        e.call(
            0x008bbf40,
            &args![
                actor, 0x111u32, 0x222u32, 0x333u32, 0x4455u16, 0u32, 0xa1u8, 0xa2u8, 0xa3u8,
                0xa4u8
            ],
        );
        assert_eq!(calls_to(&e, PACKAGE_CREATE), [vec![0x10]]);
        assert_eq!(calls_to(&e, PACKAGE_SET_TYPE), [vec![0x5000, 0x10]]);
        assert_eq!(calls_to(&e, PACKAGE_SET_FLAGS_WORD), [vec![0x5000, 0x2c]]);
        assert_eq!(calls_to(&e, PACKAGE_SET_FLAG), [vec![0x5000, 1]]);
        let news = calls_to(&e, OPERATOR_NEW);
        assert_eq!(news, [vec![0xc], vec![0x10]]);
        // The location, then the first target, then the stack target.
        assert_eq!(calls_to(&e, PACKAGE_LOCATION_SET_REFERENCE)[0][1], 0x222);
        assert_eq!(calls_to(&e, PACKAGE_LOCATION_DESTRUCTOR)[0][1], 1);
        let targets = calls_to(&e, PACKAGE_TARGET_SET_TYPE);
        assert_eq!((targets[0][1], targets[1][1]), (1, 0));
        assert_eq!(calls_to(&e, PACKAGE_TARGET_SET_FIRST)[0][1], 0x111);
        assert_eq!(calls_to(&e, PACKAGE_TARGET_SET_REFERENCE)[0][1], 0x333);
        assert_eq!(calls_to(&e, PACKAGE_TARGET_DESTRUCTOR)[0][1], 1);
        // No editor package and no second location.
        assert!(calls_to(&e, PACKAGE_COPY_EDITOR).is_empty());
        assert!(calls_to(&e, PACKAGE_SET_SECOND_LOCATION).is_empty());
        // The weapon data.
        let bytes: Vec<u8> = (0xc..0x18).map(|o| e.mem.u8(data + o)).collect();
        assert_eq!(
            bytes,
            [0xa3, 0xa4, 0xa1, 0xa2, 1, 0, 1, 0, 0x55, 0x44, 0x55, 0x44]
        );
        assert_eq!(calls_to(&e, USE_WEAPON_SET_ATTACK_TARGET)[0][0], data);
        // The process virtual +0x28 and the actor virtual +0x2f4.
        assert_eq!(
            calls_to(&e, slot_target(ACTOR_TABLE, 0x2f4)),
            [vec![actor.addr(), 0x5000, 1, 1]]
        );
        assert_eq!(calls_to(&e, slot_target(PROCESS_TABLE, 0x28)).len(), 1);
        assert_eq!(calls_to(&e, PACKAGE_TARGET_STACK_DESTRUCTOR).len(), 1);
    }

    #[test]
    fn use_weapon_package_takes_the_editor_package_and_second_location() {
        let (mut e, actor, _) = package_setup();
        double(&mut e, ACTOR_CURRENT_EDITOR_PACKAGE, eax(0x5500));
        e.call_log = Some(vec![]);
        fn_008bbf40(&mut e, actor, 1, 2, 3, 4, 0x888, 0, 0, 0, 0);
        assert_eq!(calls_to(&e, PACKAGE_COPY_EDITOR), [vec![0x5000, 0x5500]]);
        assert_eq!(calls_to(&e, OPERATOR_NEW).len(), 3);
        assert_eq!(calls_to(&e, PACKAGE_SET_SECOND_LOCATION).len(), 1);
        assert_eq!(calls_to(&e, PACKAGE_LOCATION_SET_REFERENCE)[1][1], 0x888);
    }

    #[test]
    fn use_weapon_package_needs_a_process() {
        let (mut e, actor, _) = package_setup();
        e.set(actor, Actor::pCurrentProcess, Ptr::new(0));
        e.call_log = Some(vec![]);
        fn_008bbf40(&mut e, actor, 1, 2, 3, 4, 5, 0, 0, 0, 0);
        assert!(calls_to(&e, PACKAGE_CREATE).is_empty());
    }

    #[test]
    fn action_value_and_time_are_stored() {
        let mut e = engine();
        let actor = new_actor(&mut e);
        let setting = float_setting(&mut e, 2.5);
        double(&mut e, SETTING_FLOAT_POINTER, eax(setting));
        e.call_log = Some(vec![]);
        e.call(0x008bc240, &args![actor, 9u32]);
        assert_eq!(e.get(actor, Actor::iActionValue), 9);
        assert_eq!(e.get(actor, Actor::fTimeronAction), 2.5);
        assert_eq!(
            calls_to(&e, SETTING_FLOAT_POINTER),
            [vec![ACTION_TIME_SETTING]]
        );
    }

    fn follower_setup(kind: u32) -> Engine {
        let mut e = engine();
        double(&mut e, OTHER_VALUE_0044EDB0, eax(0x33));
        double(&mut e, OTHER_KIND_009611E0, eax(kind));
        double(&mut e, OBJECT_SET_00C74820, Ret::default());
        double(&mut e, OBJECT_SET_008978F0, Ret::default());
        double(&mut e, OTHER_KARMA, st0(-3.0));
        double(&mut e, OBJECT_SET_KARMA, Ret::default());
        e
    }

    #[test]
    fn callback_hands_kind_and_karma_to_the_object() {
        for (kind, first, second) in [(1u32, 1usize, 0usize), (2, 0, 1), (3, 0, 0)] {
            let mut e = follower_setup(kind);
            let actor = e.mem.alloc(0x100);
            e.mem.set_u32(actor + 0xac, 0x7a00);
            e.call_log = Some(vec![]);
            e.call(0x008bc270, &args![actor, 0x5500u32]);
            assert_eq!(calls_to(&e, OBJECT_SET_00C74820).len(), first);
            assert_eq!(calls_to(&e, OBJECT_SET_008978F0).len(), second);
            if first == 1 {
                assert_eq!(calls_to(&e, OBJECT_SET_00C74820), [vec![0x7a00, 0x33, 1]]);
            }
            assert_eq!(
                calls_to(&e, OBJECT_SET_KARMA),
                [vec![0x7a00, 0x33, (-3.0f32).to_bits()]]
            );
        }
        // No object: nothing.
        let mut e = follower_setup(1);
        let actor = e.mem.alloc(0x100);
        e.call_log = Some(vec![]);
        fn_008bc270(&mut e, Ptr::new(actor), Ptr::new(0x5500));
        assert!(logged(&e).is_empty());
    }

    fn register_setup(player: bool) -> (Engine, Ptr<Actor>) {
        let mut e = engine();
        let actor = new_actor(&mut e);
        if player {
            e.set_global(PLAYER_POINTER, actor.addr());
        }
        slot(&mut e, ACTOR_TABLE, 0x238, eax(0x6a00));
        slot(&mut e, ACTOR_TABLE, 0x1d0, eax(0x6b00));
        double(&mut e, CALLBACK_REGISTER, Ret::default());
        double(&mut e, PLAYER_LOOKUP_OBJECT, eax(0x6c00));
        e.register(LOOKUP_KEY, |_, a| eax(a[0] + 1));
        e.register(TABLE_FIND, |_, a| {
            eax(if a[1] == 0x6b01 { 0x6d00 } else { 0 })
        });
        double(&mut e, ENTRY_TARGET, eax(0x6e00));
        double(&mut e, TARGET_NOTIFY, Ret::default());
        (e, actor)
    }

    #[test]
    fn actor_registers_its_callback_and_notifies_the_entry_target() {
        let (mut e, actor) = register_setup(false);
        e.call_log = Some(vec![]);
        e.call(0x008bc300, &args![actor]);
        assert_eq!(
            calls_to(&e, CALLBACK_REGISTER),
            [vec![0x6a00, 1, 0x008b_c270]]
        );
        assert!(calls_to(&e, PLAYER_LOOKUP_OBJECT).is_empty());
        assert_eq!(calls_to(&e, LOOKUP_KEY), [vec![0x6b00]]);
        assert_eq!(calls_to(&e, TABLE_FIND), [vec![LOOKUP_TABLE, 0x6b01]]);
        assert_eq!(calls_to(&e, ENTRY_TARGET), [vec![0x6d00]]);
        assert_eq!(calls_to(&e, TARGET_NOTIFY), [vec![0x6e00, 1, 0x6a00]]);
        // The player uses the other object; its key is not in the table.
        let (mut e, actor) = register_setup(true);
        e.call_log = Some(vec![]);
        fn_008bc300(&mut e, actor);
        assert_eq!(calls_to(&e, PLAYER_LOOKUP_OBJECT), [vec![actor.addr()]]);
        assert_eq!(calls_to(&e, LOOKUP_KEY), [vec![0x6c00]]);
        assert!(calls_to(&e, TARGET_NOTIFY).is_empty());
    }

    /// The doubles of `008bc3d0`; the player is a big object whose timer
    /// at `+0xe24` is `timer`.
    fn greeting_setup(timer: f32) -> (Engine, Ptr<Actor>, Ptr<Actor>) {
        let mut e = engine();
        let actor = new_actor(&mut e);
        with_process(&mut e, actor);
        let player = big_actor(&mut e, 0x1000);
        e.set_global(PLAYER_POINTER, player.addr());
        e.mem.set_f32(player.addr() + 0xe24, timer);
        e.set_global(GREETING_TIMER_LIMIT, 5.0f64);
        e.register(REFERENCE_OWNER, |_, a| eax(a[0] + 0x10));
        e.register(OWNER_BLOCKS_GREETING, |_, _| eax(0));
        let setting = e.mem.alloc(4);
        e.mem.set_u8(setting, 1);
        double(&mut e, SETTING_BYTE_POINTER, eax(setting));
        double(&mut e, GREETING_PREPARE, Ret::default());
        double(&mut e, CURRENT_TIME_WORD, eax(1234));
        for offset in [0x628, 0x310, 0x2a4, 0x1dc] {
            slot(&mut e, PROCESS_TABLE, offset, Ret::default());
        }
        slot(&mut e, PROCESS_TABLE, 0x4bc, eax(0));
        slot(&mut e, PROCESS_TABLE, 0x30c, eax(1));
        (e, actor, player)
    }

    #[test]
    fn greeting_starts_when_the_timer_is_below_the_limit() {
        let (mut e, actor, player) = greeting_setup(1.0);
        e.call_log = Some(vec![]);
        e.call(0x008bc3d0, &args![actor, 0x4000u32]);
        let process = e.mem.u32(actor.addr() + 0x68);
        let p = player.addr();
        assert_eq!(
            calls_to(&e, slot_target(PROCESS_TABLE, 0x628)),
            [vec![process, p]]
        );
        assert_eq!(calls_to(&e, GREETING_PREPARE), [vec![actor.addr(), p]]);
        assert_eq!(
            calls_to(&e, slot_target(PROCESS_TABLE, 0x310)),
            [vec![process, 1]]
        );
        assert_eq!(
            calls_to(&e, slot_target(PROCESS_TABLE, 0x2a4)),
            [vec![process, actor.addr(), 0x4000, 0, 0, 1, 0]]
        );
        assert_eq!(e.mem.f32(p + 0xe24), 1234.0);
        // The process neither answers +0x4bc nor fails +0x30c: +0x1dc.
        assert_eq!(
            calls_to(&e, slot_target(PROCESS_TABLE, 0x1dc)),
            [vec![process, actor.addr(), 1]]
        );
    }

    #[test]
    fn greeting_skips_the_start_when_the_timer_is_over() {
        let (mut e, actor, player) = greeting_setup(9.0);
        slot(&mut e, PROCESS_TABLE, 0x4bc, eax(1));
        e.call_log = Some(vec![]);
        actor_start_greeting_player(&mut e, actor, 0x4000);
        assert!(calls_to(&e, slot_target(PROCESS_TABLE, 0x2a4)).is_empty());
        assert!(calls_to(&e, slot_target(PROCESS_TABLE, 0x1dc)).is_empty());
        assert_eq!(calls_to(&e, slot_target(PROCESS_TABLE, 0x628)).len(), 1);
        assert_eq!(e.mem.f32(player.addr() + 0xe24), 9.0);
        // +0x4bc zero but +0x30c false: no +0x1dc.
        slot(&mut e, PROCESS_TABLE, 0x4bc, eax(0));
        slot(&mut e, PROCESS_TABLE, 0x30c, eax(0));
        e.call_log = Some(vec![]);
        actor_start_greeting_player(&mut e, actor, 0x4000);
        assert!(calls_to(&e, slot_target(PROCESS_TABLE, 0x1dc)).is_empty());
    }

    #[test]
    fn greeting_needs_an_owner_that_does_not_block_and_a_process() {
        // No other actor.
        let (mut e, actor, _) = greeting_setup(1.0);
        e.call_log = Some(vec![]);
        actor_start_greeting_player(&mut e, actor, 0);
        assert!(logged(&e).is_empty());
        // No owner.
        e.register(REFERENCE_OWNER, |_, _| eax(0));
        actor_start_greeting_player(&mut e, actor, 0x4000);
        assert!(calls_to(&e, slot_target(PROCESS_TABLE, 0x628)).is_empty());
        // A blocking owner.
        e.register(REFERENCE_OWNER, |_, a| eax(a[0] + 0x10));
        e.register(OWNER_BLOCKS_GREETING, |_, _| eax(1));
        actor_start_greeting_player(&mut e, actor, 0x4000);
        assert!(calls_to(&e, slot_target(PROCESS_TABLE, 0x628)).is_empty());
        // No process.
        e.register(OWNER_BLOCKS_GREETING, |_, _| eax(0));
        e.set(actor, Actor::pCurrentProcess, Ptr::new(0));
        actor_start_greeting_player(&mut e, actor, 0x4000);
        assert!(calls_to(&e, slot_target(PROCESS_TABLE, 0x628)).is_empty());
    }

    #[test]
    fn greeting_timer_compares_and_stores() {
        let mut e = engine();
        e.set_global(GREETING_TIMER_LIMIT, 5.0f64);
        let player = e.mem.alloc(0x1000);
        for (timer, expected) in [
            (4.9f32, true),
            (5.0, false),
            (6.0, false),
            (f32::NAN, false),
        ] {
            e.mem.set_f32(player + 0xe24, timer);
            assert_eq!(fn_008bc520(&mut e, Ptr::new(player)), expected);
            assert_eq!(e.call(0x008bc520, &args![player]).bool(), expected);
        }
        double(&mut e, CURRENT_TIME_WORD, eax(0xffff_ffff));
        e.call(0x008bc560, &args![player]);
        assert_eq!(e.mem.f32(player + 0xe24), 4294967295u32 as f32);
    }

    /// The tracked actor of `008bc590`: a big object with a process whose
    /// `+0x748` answers `ready`, and the actor's `+0x1e4` answers an object.
    fn tracked_setup(ready: u32) -> (Engine, Ptr<Actor>, u32) {
        let mut e = engine();
        e.map(0x011d_f000, 0x1000);
        let actor = big_actor(&mut e, 0x200);
        let process = with_process(&mut e, actor);
        process_getter(&mut e);
        e.set_global(TRACKED_ACTOR_POINTER, actor.addr());
        slot(&mut e, ACTOR_TABLE, 0x1e4, eax(0x7000));
        slot(&mut e, PROCESS_TABLE, 0x748, eax(ready));
        for offset in [0x744, 0x74c, 0x44] {
            slot(&mut e, PROCESS_TABLE, offset, Ret::default());
        }
        double(&mut e, TRACKED_OBJECT_SET_VALUE, Ret::default());
        let setting = float_setting(&mut e, 3.5);
        double(&mut e, SETTING_FLOAT_POINTER, eax(setting));
        e.set_global(TRACKED_ACTOR_TIME, 1.0f32);
        (e, actor, process)
    }

    #[test]
    fn tracked_actor_is_reset() {
        let (mut e, actor, process) = tracked_setup(1);
        e.mem.set_u8(actor.addr() + 0x7d, 9);
        e.call_log = Some(vec![]);
        e.call(0x008bc590, &args![0u32, 0u8]);
        let a = actor.addr();
        assert_eq!(
            (e.mem.u8(a + 0x7d), e.mem.u8(a + 0x7c), e.mem.u8(a + 0x7e)),
            (0, 1, 0)
        );
        assert_eq!(
            calls_to(&e, TRACKED_OBJECT_SET_VALUE),
            [vec![0x7000, 0.0f32.to_bits()]]
        );
        assert_eq!(
            calls_to(&e, slot_target(PROCESS_TABLE, 0x744)),
            [vec![process, 0]]
        );
        assert_eq!(
            calls_to(&e, slot_target(PROCESS_TABLE, 0x74c)),
            [vec![process, 0]]
        );
        assert_eq!(
            calls_to(&e, slot_target(PROCESS_TABLE, 0x44)),
            [vec![process, a, 0, 2, 0, 1, 1]]
        );
        assert_eq!(e.global::<f32>(TRACKED_ACTOR_TIME), 3.5);
        assert_eq!(e.global::<u32>(TRACKED_ACTOR_POINTER), 0);
    }

    #[test]
    fn tracked_actor_flag_keeps_the_time_and_skips_the_process_call() {
        let (mut e, actor, _) = tracked_setup(0);
        e.call_log = Some(vec![]);
        fn_008bc590(&mut e, 1, 1);
        assert!(calls_to(&e, TRACKED_OBJECT_SET_VALUE).is_empty());
        assert!(calls_to(&e, slot_target(PROCESS_TABLE, 0x44)).is_empty());
        assert_eq!(e.mem.u8(actor.addr() + 0x7e), 0);
        assert_eq!(e.global::<f32>(TRACKED_ACTOR_TIME), 1.0);
        assert_eq!(e.global::<u32>(TRACKED_ACTOR_POINTER), 0);
        // Without a process the byte is cleared and nothing is asked.
        let (mut e, actor, _) = tracked_setup(0);
        e.set(actor, Actor::pCurrentProcess, Ptr::new(0));
        e.mem.set_u8(actor.addr() + 0x7e, 1);
        fn_008bc590(&mut e, 0, 0);
        assert_eq!(e.mem.u8(actor.addr() + 0x7e), 0);
        assert_eq!(e.global::<f32>(TRACKED_ACTOR_TIME), 3.5);
        // No tracked actor: only the global is written.
        let (mut e, _, _) = tracked_setup(1);
        e.set_global(TRACKED_ACTOR_POINTER, 0u32);
        e.call_log = Some(vec![]);
        fn_008bc590(&mut e, 1, 0);
        assert!(logged(&e).is_empty());
        assert_eq!(e.global::<f32>(TRACKED_ACTOR_TIME), 1.0);
    }

    #[test]
    fn tracked_actor_without_a_zeroable_object_skips_the_reset() {
        let (mut e, _, _) = tracked_setup(1);
        slot(&mut e, ACTOR_TABLE, 0x1e4, eax(0));
        e.call_log = Some(vec![]);
        fn_008bc590(&mut e, 0, 0);
        assert!(calls_to(&e, TRACKED_OBJECT_SET_VALUE).is_empty());
        assert!(calls_to(&e, slot_target(PROCESS_TABLE, 0x744)).is_empty());
        assert_eq!(calls_to(&e, slot_target(PROCESS_TABLE, 0x44)).len(), 1);
    }

    #[test]
    fn combat_with_actor_searches_the_target_list() {
        let mut e = engine();
        let actor = new_actor(&mut e);
        e.set(actor, Actor::pCurrentCombatTargetArray, Ptr::new(0x7100));
        double(&mut e, ACTOR_STATE_TEST_493BB0, eax(1));
        let seen = Rc::new(RefCell::new(Vec::new()));
        let capture = seen.clone();
        e.register_double(LIST_FIND, move |e, a| {
            capture
                .borrow_mut()
                .push((a[0], e.mem.u32(a[1]), a[2], a[3]));
            eax(2)
        });
        assert!(e.call(0x008bc700, &args![actor, 0x8800u32]).bool());
        assert_eq!(*seen.borrow(), [(0x7100, 0x8800, 0, COMBAT_TARGET_COMPARE)]);
        // Not found.
        e.register(LIST_FIND, |_, _| eax(0xffff_ffff));
        assert!(!actor_is_in_combat_with_actor(&mut e, actor, 0x8800));
        // No list.
        e.set(actor, Actor::pCurrentCombatTargetArray, Ptr::new(0));
        e.register(LIST_FIND, |_, _| eax(0));
        assert!(!actor_is_in_combat_with_actor(&mut e, actor, 0x8800));
        // The first test fails.
        e.set(actor, Actor::pCurrentCombatTargetArray, Ptr::new(0x7100));
        double(&mut e, ACTOR_STATE_TEST_493BB0, eax(0));
        assert!(!actor_is_in_combat_with_actor(&mut e, actor, 0x8800));
    }

    #[test]
    fn extra_list_writers_use_the_reference_list() {
        let mut e = engine();
        let actor = new_actor(&mut e);
        double(&mut e, EXTRA_LIST_OF_REFERENCE, eax(0x5000));
        double(&mut e, EXTRA_LIST_ADD_0041D700, Ret::default());
        double(&mut e, EXTRA_LIST_ADD_00422480, Ret::default());
        double(&mut e, EXTRA_LIST_REMOVE_00422550, Ret::default());
        slot(&mut e, ACTOR_TABLE, 0x48, Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x008bc750, &args![actor, 0x10u32, 0x1ffu32]);
        assert_eq!(
            calls_to(&e, EXTRA_LIST_ADD_0041D700),
            [vec![0x5000, 0x10, 0xff]]
        );
        assert_eq!(
            calls_to(&e, slot_target(ACTOR_TABLE, 0x48)),
            [vec![actor.addr(), 0x8000_0000]]
        );
        e.call_log = Some(vec![]);
        e.call(0x008bc790, &args![actor, 0x20u32]);
        assert_eq!(calls_to(&e, EXTRA_LIST_ADD_00422480), [vec![0x5000, 0x20]]);
        assert_eq!(
            calls_to(&e, slot_target(ACTOR_TABLE, 0x48)),
            [vec![actor.addr(), 0x800]]
        );
        e.call_log = Some(vec![]);
        e.call(0x008bc7d0, &args![actor, 0x30u32]);
        assert_eq!(
            calls_to(&e, EXTRA_LIST_REMOVE_00422550),
            [vec![0x5000, 0x30]]
        );
        assert!(calls_to(&e, slot_target(ACTOR_TABLE, 0x48)).is_empty());
    }

    /// Follower tests: a package is its own type number.
    fn following_setup() -> (Engine, Ptr<Actor>) {
        let mut e = engine();
        let actor = new_actor(&mut e);
        with_process(&mut e, actor);
        e.register(ACTOR_CURRENT_PACKAGE, |_, _| eax(1));
        e.register(PACKAGE_TYPE, |_, a| eax(a[0]));
        slot(&mut e, PROCESS_TABLE, 0x128, eax(0x4000));
        slot(&mut e, PROCESS_TABLE, 0x22c, eax(0));
        slot(&mut e, PROCESS_TABLE, 0x20c, eax(0));
        double(&mut e, EXTRA_LIST_OF_REFERENCE, eax(0x5000));
        double(&mut e, EXTRA_LIST_GET_PACKAGE, eax(0));
        (e, actor)
    }

    #[test]
    fn is_following_needs_a_follow_package_and_the_same_leader() {
        let (mut e, actor) = following_setup();
        assert!(e.call(0x008bc7f0, &args![actor, 0x4000u32]).bool());
        assert!(!actor_is_following_ov2(&mut e, actor, 0x4001));
        // Type 7 counts too, type 5 does not, no package does not.
        e.register(ACTOR_CURRENT_PACKAGE, |_, _| eax(7));
        assert!(actor_is_following_ov2(&mut e, actor, 0x4000));
        e.register(ACTOR_CURRENT_PACKAGE, |_, _| eax(5));
        assert!(!actor_is_following_ov2(&mut e, actor, 0x4000));
        e.register(ACTOR_CURRENT_PACKAGE, |_, _| eax(0));
        assert!(!actor_is_following_ov2(&mut e, actor, 0x4000));
        // No process.
        e.set(actor, Actor::pCurrentProcess, Ptr::new(0));
        e.register(ACTOR_CURRENT_PACKAGE, |_, _| eax(1));
        assert!(!actor_is_following_ov2(&mut e, actor, 0x4000));
    }

    #[test]
    fn could_be_following_also_looks_at_the_other_packages() {
        let (mut e, actor) = following_setup();
        assert!(e.call(0x008bc860, &args![actor, 0x4000u32]).bool());
        // Wrong leader and no other package: false.
        assert!(!actor_could_be_following(&mut e, actor, 0x4001));
        // The process's second package.
        slot(&mut e, PROCESS_TABLE, 0x22c, eax(7));
        assert!(actor_could_be_following(&mut e, actor, 0x4001));
        slot(&mut e, PROCESS_TABLE, 0x22c, eax(5));
        assert!(!actor_could_be_following(&mut e, actor, 0x4001));
        // The third package.
        slot(&mut e, PROCESS_TABLE, 0x20c, eax(1));
        assert!(actor_could_be_following(&mut e, actor, 0x4001));
        slot(&mut e, PROCESS_TABLE, 0x20c, eax(0));
        // The package extra of the extra data list.
        double(&mut e, EXTRA_LIST_GET_PACKAGE, eax(7));
        assert!(actor_could_be_following(&mut e, actor, 0x4001));
        // No process: false.
        e.set(actor, Actor::pCurrentProcess, Ptr::new(0));
        assert!(!actor_could_be_following(&mut e, actor, 0x4001));
    }

    #[test]
    fn speech_start_needs_the_object_of_virtual_0x428() {
        let mut e = engine();
        let actor = new_actor(&mut e);
        double(&mut e, SPEECH_START, Ret::default());
        slot(&mut e, ACTOR_TABLE, 0x428, eax(0));
        e.call_log = Some(vec![]);
        e.call(0x008bc980, &args![actor, 1u32, 2u32, 3u32, 0x1ffu32]);
        assert!(calls_to(&e, SPEECH_START).is_empty());
        slot(&mut e, ACTOR_TABLE, 0x428, eax(0x7700));
        e.call(0x008bc980, &args![actor, 1u32, 2u32, 3u32, 0x1ffu32]);
        assert_eq!(calls_to(&e, SPEECH_START), [vec![0x7700, 1, 2, 3, 0xff]]);
    }

    #[test]
    fn combat_bit_of_the_target_word() {
        let mut e = engine();
        let actor = new_actor(&mut e);
        for (word, expected) in [(0u32, false), (1, true), (0x7100, false), (0x7101, true)] {
            e.set(actor, Actor::pCurrentCombatTargetArray, Ptr::new(word));
            assert_eq!(fn_008bca70(&mut e, actor), expected);
            assert_eq!(e.call(0x008bca70, &args![actor]).bool(), expected);
        }
    }

    #[test]
    fn can_be_commanded_cases() {
        let mut e = engine();
        let actor = new_actor(&mut e);
        let other = new_actor(&mut e);
        double(&mut e, OTHER_TEST_0047BCF0, eax(0));
        double(&mut e, GET_BASE_FORM, eax(0x7000));
        e.register(BASE_COMMANDING_FORM, |_, _| eax(0x7100));
        e.register(COMMANDING_ENTRY, |_, a| eax(a[0] + 1));
        let seen = Rc::new(RefCell::new(Vec::new()));
        let capture = seen.clone();
        e.register_double(COMMANDING_TEST, move |e, a| {
            capture.borrow_mut().push((a[0], e.mem.u32(a[1])));
            eax(1)
        });
        slot(&mut e, ACTOR_TABLE, 0x218, eax(0));
        // Not the "+0x218" kind: the commanding test decides.
        assert!(e.call(0x008bc9d0, &args![actor, other, 0u32]).bool());
        assert_eq!(*seen.borrow(), [(0x7101, other.addr())]);
        e.register(COMMANDING_TEST, |_, _| eax(0));
        assert!(!fn_008bc9d0(&mut e, actor, other, 0));
        e.register(BASE_COMMANDING_FORM, |_, _| eax(0));
        assert!(!fn_008bc9d0(&mut e, actor, other, 0));
        // The +0x218 kind: forced, or the other actor's test.
        slot(&mut e, ACTOR_TABLE, 0x218, eax(1));
        assert!(fn_008bc9d0(&mut e, actor, other, 1));
        assert!(!fn_008bc9d0(&mut e, actor, other, 0));
        double(&mut e, OTHER_TEST_0047BCF0, eax(1));
        assert!(fn_008bc9d0(&mut e, actor, other, 0));
        // The other actor has bit 0 of its target word set: never.
        e.set(other, Actor::pCurrentCombatTargetArray, Ptr::new(1));
        assert!(!fn_008bc9d0(&mut e, actor, other, 1));
    }

    #[test]
    fn player_teammate_flag_registers_and_removes() {
        let mut e = engine();
        let actor = new_actor(&mut e);
        let player = e.mem.alloc(0x1000);
        e.set_global(PLAYER_POINTER, player);
        double(&mut e, PLAYER_ADD_TEAMMATE, Ret::default());
        double(&mut e, TEAMMATE_LIST_REMOVE, Ret::default());
        e.mem.set_u32(player + 0xd68, 3);
        e.call_log = Some(vec![]);
        e.call(0x008bca90, &args![actor, 1u32]);
        assert_eq!(
            calls_to(&e, PLAYER_ADD_TEAMMATE),
            [vec![player, actor.addr()]]
        );
        assert!(actor_is_teammate(&e, actor));
        // Already a teammate: no second registration.
        actor_set_player_teammate(&mut e, actor, 1);
        assert_eq!(calls_to(&e, PLAYER_ADD_TEAMMATE).len(), 1);
        // Removal: count down and list removal.
        actor_set_player_teammate(&mut e, actor, 0);
        assert!(!actor_is_teammate(&e, actor));
        assert_eq!(e.mem.u32(player + 0xd68), 2);
        assert_eq!(calls_to(&e, TEAMMATE_LIST_REMOVE).len(), 1);
        // Not a teammate and not asked to be: nothing more.
        actor_set_player_teammate(&mut e, actor, 0);
        assert_eq!(calls_to(&e, TEAMMATE_LIST_REMOVE).len(), 1);
        // The player itself is left alone.
        let player_actor = Ptr::<Actor>::new(player);
        actor_set_player_teammate(&mut e, player_actor, 1);
        assert_eq!(e.mem.u8(player + 0x18d), 0);
    }

    fn actor_is_teammate(e: &Engine, actor: Ptr<Actor>) -> bool {
        e.mem.u8(actor.addr() + 0x18d) != 0
    }

    #[test]
    fn teammate_removal_passes_the_teammate_by_address() {
        let mut e = engine();
        let player = e.mem.alloc(0x1000);
        e.mem.set_u32(player + 0xd68, 0);
        let seen = Rc::new(RefCell::new(Vec::new()));
        let capture = seen.clone();
        e.register_double(TEAMMATE_LIST_REMOVE, move |e, a| {
            capture.borrow_mut().push((a[0], e.mem.u32(a[1])));
            Ret::default()
        });
        e.call(0x008bcb00, &args![player, 0x4242u32]);
        assert_eq!(*seen.borrow(), [(player + 0x5fc, 0x4242)]);
        assert_eq!(e.mem.u32(player + 0xd68), 0xffff_ffff);
    }

    /// A crime list `{node1 -> node2}` where node1's data is a holder
    /// `{0xf1 -> {0xf2 -> end}}` and node2's data is zero.
    fn crime_list(e: &mut Engine) -> u32 {
        let second_element = node(e, 0xf2, 0);
        let first_element = node(e, 0xf1, second_element);
        let last = node(e, 0, 0);
        node(e, first_element, last)
    }

    #[test]
    fn crime_list_walk_marks_every_element() {
        let mut e = engine();
        list_nodes(&mut e);
        double(&mut e, EXTRA_LIST_OF_REFERENCE, eax(0x5000));
        let list = crime_list(&mut e);
        e.register(CRIME_LIST_ITERATOR, |_, a| eax(a[0]));
        double(&mut e, CRIME_ELEMENT_MARK, Ret::default());
        double(&mut e, EXTRA_LIST_GET_CRIME_LIST, eax(list));
        e.call_log = Some(vec![]);
        e.call(0x008bcb40, &args![0x4400u32]);
        // The first node's holder starts at 0xf1: the first element, then
        // the next ({0xf2, 0}), then the end; the second node's data is zero.
        assert_eq!(
            calls_to(&e, CRIME_ELEMENT_MARK),
            [vec![0xf1, 0], vec![0xf2, 0]]
        );
        // An empty list: nothing.
        e.register(EXTRA_LIST_GET_CRIME_LIST, |_, _| eax(0));
        e.call_log = Some(vec![]);
        fn_008bcb40(&mut e, Ptr::new(0x4400));
        assert!(calls_to(&e, CRIME_ELEMENT_MARK).is_empty());
    }

    #[test]
    fn adding_a_crime_marks_the_elements_up_to_the_first_empty_one() {
        let mut e = engine();
        list_nodes(&mut e);
        double(&mut e, EXTRA_LIST_OF_REFERENCE, eax(0x5000));
        let crime = e.mem.alloc(0x80);
        e.mem.set_u32(crime + 0x34, 0xabcd);
        let seen = Rc::new(RefCell::new(Vec::new()));
        let capture = seen.clone();
        e.register_double(CRIME_STAMP_VALUE, move |e, a| {
            capture.borrow_mut().push(e.mem.u32(a[0]));
            st0(2.5)
        });
        double(&mut e, EXTRA_LIST_ADD_CRIME_LIST, Ret::default());
        // The second element is empty ({0, 0}) and ends the walk.
        let stop = node(&mut e, 0, 0);
        let first_element = node(&mut e, 0xf3, stop);
        double(&mut e, CRIME_LIST_ITERATOR, eax(first_element));
        double(&mut e, CRIME_ELEMENT_MARK, Ret::default());
        double(&mut e, PROCESS_LISTS_NOTIFY, Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x008bcbd0, &args![0x4400u32, crime]);
        assert_eq!(*seen.borrow(), [0xabcd]);
        assert_eq!(
            calls_to(&e, EXTRA_LIST_ADD_CRIME_LIST),
            [vec![0x5000, crime, 2.5f32.to_bits()]]
        );
        assert_eq!(calls_to(&e, CRIME_ELEMENT_MARK), [vec![0xf3, 1]]);
        assert_eq!(
            calls_to(&e, PROCESS_LISTS_NOTIFY),
            [vec![PROCESS_LISTS_OBJECT, crime]]
        );
    }

    #[test]
    fn word_at_0x34_is_copied_out() {
        let mut e = engine();
        let source = e.mem.alloc(0x80);
        e.mem.set_u32(source + 0x34, 0x5151);
        let out = e.mem.alloc(4);
        assert_eq!(e.call(0x008bcc60, &args![source, out]).u32(), out);
        assert_eq!(e.mem.u32(out), 0x5151);
    }

    /// Doubles for `008bcc80`; the actor value `0x14` is `value`, the
    /// setting `limit`.
    fn radiation_setup(value: f64, limit: f32) -> (Engine, Ptr<Actor>) {
        let mut e = engine();
        let actor = new_actor(&mut e);
        double(&mut e, ACTOR_STATE_TEST_493BB0, eax(0));
        slot(
            &mut e,
            OWNER_TABLE,
            0xc,
            Ret {
                st0: value,
                ..Ret::default()
            },
        );
        let setting = float_setting(&mut e, limit);
        double(&mut e, SETTING_FLOAT_POINTER, eax(setting));
        e.register(ACTOR_CURRENT_PACKAGE, |_, _| eax(0x7300));
        double(&mut e, PACKAGE_AVOID_TEST_A, eax(1));
        double(&mut e, PACKAGE_AVOID_TEST_B, eax(0));
        (e, actor)
    }

    #[test]
    fn avoiding_radiation_cases() {
        // The package passes the first test and fails the second: false.
        let (mut e, actor) = radiation_setup(10.0, 20.0);
        e.call_log = Some(vec![]);
        assert!(!e.call(0x008bcc80, &args![actor]).bool());
        assert_eq!(
            calls_to(&e, slot_target(OWNER_TABLE, 0xc)),
            [vec![actor.addr() + 0xa4, 0x14]]
        );
        assert_eq!(
            calls_to(&e, SETTING_FLOAT_POINTER),
            [vec![RADIATION_LIMIT_SETTING]]
        );
        // The package fails the first test: true.
        double(&mut e, PACKAGE_AVOID_TEST_A, eax(0));
        assert!(actor_should_actor_avoid_radiation(&mut e, actor));
        // It passes both: true.
        double(&mut e, PACKAGE_AVOID_TEST_A, eax(1));
        double(&mut e, PACKAGE_AVOID_TEST_B, eax(1));
        assert!(actor_should_actor_avoid_radiation(&mut e, actor));
        // No package: true.
        e.register(ACTOR_CURRENT_PACKAGE, |_, _| eax(0));
        assert!(actor_should_actor_avoid_radiation(&mut e, actor));
        // The setting is below the actor value: false; equal: go on.
        let (mut e, actor) = radiation_setup(30.0, 20.0);
        assert!(!actor_should_actor_avoid_radiation(&mut e, actor));
        let (mut e, actor) = radiation_setup(20.0, 20.0);
        assert!(!actor_should_actor_avoid_radiation(&mut e, actor));
        double(&mut e, PACKAGE_AVOID_TEST_B, eax(1));
        assert!(actor_should_actor_avoid_radiation(&mut e, actor));
        // The first test holds: false.
        double(&mut e, ACTOR_STATE_TEST_493BB0, eax(1));
        assert!(!actor_should_actor_avoid_radiation(&mut e, actor));
    }

    #[test]
    fn fades_act_only_on_process_type_zero() {
        let mut e = engine();
        let actor = new_actor(&mut e);
        e.register(ACTOR_PROCESS, |_, _| eax(0x7400));
        e.register(GET_PROCESS_TYPE, |_, _| eax(0));
        double(&mut e, HIGH_PROCESS_FADE_IN, Ret::default());
        double(&mut e, HIGH_PROCESS_FADE_OUT, Ret::default());
        double(&mut e, FADE_VALUE, st0(0.25));
        e.call_log = Some(vec![]);
        e.call(0x008bcd20, &args![actor]);
        e.call(0x008bcd60, &args![actor]);
        assert_eq!(
            calls_to(&e, HIGH_PROCESS_FADE_IN),
            [vec![0x7400, actor.addr(), 0]]
        );
        assert_eq!(
            calls_to(&e, HIGH_PROCESS_FADE_OUT),
            [vec![0x7400, actor.addr(), 0, 0]]
        );
        assert_eq!(e.call(0x008bcda0, &args![actor]).f32(), 0.25);
        assert_eq!(calls_to(&e, FADE_VALUE), [vec![0x7400]]);
        // Another process type: no fades, the value is 1.
        e.register(GET_PROCESS_TYPE, |_, _| eax(2));
        e.call_log = Some(vec![]);
        actor_fade_in(&mut e, actor);
        fn_008bcd60(&mut e, actor);
        assert_eq!(fn_008bcda0(&mut e, actor), 1.0);
        assert!(calls_to(&e, HIGH_PROCESS_FADE_IN).is_empty());
        assert!(calls_to(&e, HIGH_PROCESS_FADE_OUT).is_empty());
    }

    // ---- 008bcdd0 .. 008bec20 ------------------------------------------

    const CACHE_OWNER_TABLE: u32 = 0x0200_e000;
    const NODE_TABLE: u32 = 0x0200_e800;
    const SKIN_TABLE: u32 = 0x0200_f000;
    const CHILD_TABLE: u32 = 0x0200_f800;

    fn word(value: f32) -> u32 {
        value.to_bits()
    }

    #[test]
    fn the_fade_value_is_the_float_at_0x3ec() {
        let mut e = engine();
        let process = e.mem.alloc(0x400);
        e.mem.set_f32(process + 0x3ec, 0.75);
        assert_eq!(e.call(0x008bcdd0, &args![process]).f32(), 0.75);
    }

    #[test]
    fn the_ragdoll_range_is_a_float_setting() {
        let mut e = engine();
        e.map(0x0126_7000, 0x1000);
        e.set_global(0x0126_7c6c, 33.5f32);
        e.register_double(SETTING_FLOAT_POINTER, |_, a| eax(a[0]));
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x008bd550, &args![]).f32(), 33.5);
        assert_eq!(calls_to(&e, SETTING_FLOAT_POINTER), [vec![0x0126_7c6c]]);
    }

    #[test]
    fn the_ease_out_flags_are_set_together_and_read_back() {
        let mut e = engine();
        let ragdoll = e.mem.alloc(0x300);
        e.call(0x008bd570, &args![ragdoll, 1u32]);
        for offset in [0x221, 0x1ed, 0x1bd, 0xb2, 0x43] {
            assert_eq!(e.mem.u8(ragdoll + offset), 1);
        }
        assert_eq!(e.mem.u8(ragdoll + 0x44), 0);
        assert_eq!(e.call(0x008bd5c0, &args![ragdoll]).u8(), 1);
        e.call(0x008bd570, &args![ragdoll, 0u32]);
        assert_eq!(e.call(0x008bd5c0, &args![ragdoll]).u8(), 0);
        assert_eq!(e.mem.u8(ragdoll + 0x221), 0);
    }

    #[test]
    fn the_four_word_value_goes_to_the_object_at_0xc0() {
        let mut e = engine();
        let ragdoll = e.mem.alloc(0x300);
        double(&mut e, FOUR_WORD_ASSIGN, Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x008bd5e0, &args![ragdoll, 0x1234u32]);
        assert_eq!(
            calls_to(&e, FOUR_WORD_ASSIGN),
            [vec![ragdoll + 0xc0, 0x1234]]
        );
    }

    #[test]
    fn the_byte_at_0x243_is_stored() {
        let mut e = engine();
        let ragdoll = e.mem.alloc(0x300);
        e.call(0x008bd610, &args![ragdoll, 1u32]);
        assert_eq!(e.mem.u8(ragdoll + 0x243), 1);
        e.call(0x008bd610, &args![ragdoll, 0u32]);
        assert_eq!(e.mem.u8(ragdoll + 0x243), 0);
    }

    // The fade: a root node with a skin that allows the fade and one child
    // whose skin does not.
    #[test]
    fn fading_skins_walks_the_children() {
        let mut e = engine();
        for table in [NODE_TABLE, SKIN_TABLE, CHILD_TABLE] {
            e.map(table, 0x800);
        }
        let actor = new_actor(&mut e);
        let root = e.mem.alloc(0x40);
        let child = e.mem.alloc(0x40);
        let root_skin = e.mem.alloc(0x40);
        let child_skin = e.mem.alloc(0x40);
        let children = e.mem.alloc(0x40);
        e.mem.set_u32(root, NODE_TABLE);
        e.mem.set_u32(child, CHILD_TABLE);
        e.mem.set_u32(root_skin, SKIN_TABLE);
        e.mem.set_u32(child_skin, SKIN_TABLE + 0x100);
        e.map(SKIN_TABLE + 0x100, 0x100);
        slot(&mut e, NODE_TABLE, 0x18, eax(root_skin));
        slot(&mut e, NODE_TABLE, 0x0c, eax(children));
        slot(&mut e, CHILD_TABLE, 0x18, eax(child_skin));
        slot(&mut e, CHILD_TABLE, 0x0c, eax(0));
        slot(&mut e, SKIN_TABLE, 0x34, eax(0));
        slot(&mut e, SKIN_TABLE + 0x100, 0x34, eax(1));
        e.register(CHILD_COUNT, |_, _| eax(2));
        e.register_double(CHILD_AT, move |_, a| eax(if a[1] == 0 { child } else { 0 }));
        e.register(PROPERTY_TYPE_SHADER, |_, _| eax(3));
        e.register(NODE_GET_PROPERTY, |_, a| eax(a[0] + 0x1000));
        double(&mut e, SHADER_SET_ALPHA, Ret::default());
        double(&mut e, SHADER_SET_FADE, Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x008bd630, &args![actor, root, 0.5f32]);
        assert_eq!(
            calls_to(&e, NODE_GET_PROPERTY),
            [vec![root_skin, 3]],
            "the child's skin refuses the fade"
        );
        assert_eq!(
            calls_to(&e, SHADER_SET_ALPHA),
            [vec![root_skin + 0x1000, word(0.5)]]
        );
        assert_eq!(
            calls_to(&e, SHADER_SET_FADE),
            [vec![root_skin + 0x1000, word(0.5)]]
        );
        // A node without a skin and without children does nothing.
        e.call_log = Some(vec![]);
        slot(&mut e, NODE_TABLE, 0x18, eax(0));
        slot(&mut e, NODE_TABLE, 0x0c, eax(0));
        e.call(0x008bd630, &args![actor, root, 0.5f32]);
        assert!(calls_to(&e, NODE_GET_PROPERTY).is_empty());
        // A skin without the property sets nothing.
        slot(&mut e, NODE_TABLE, 0x18, eax(root_skin));
        e.register(NODE_GET_PROPERTY, |_, _| eax(0));
        e.call_log = Some(vec![]);
        e.call(0x008bd630, &args![actor, root, 0.5f32]);
        assert!(calls_to(&e, SHADER_SET_ALPHA).is_empty());
    }

    fn visible_case(part: u32, vslot_21c: u32, test: u32, vslot_1a0: u32) -> (u8, Vec<Vec<u32>>) {
        let mut e = engine();
        e.map(NODE_TABLE, 0x800);
        let actor = new_actor(&mut e);
        let base = e.mem.alloc(0x100);
        e.mem.set_u32(base + 0x30, NODE_TABLE);
        slot(&mut e, NODE_TABLE, 0x24, eax(part));
        slot(&mut e, ACTOR_TABLE, 0x21c, eax(vslot_21c));
        slot(&mut e, ACTOR_TABLE, 0x1a0, eax(vslot_1a0));
        e.register_double(GET_BASE_FORM, move |_, _| eax(base));
        e.register_double(ACTOR_TEST_008ACE90, move |_, _| eax(test));
        e.call_log = Some(vec![]);
        let result = e.call(0x008bd700, &args![actor.addr() + 0x94]).u8();
        let arguments = calls_to(&e, slot_target(ACTOR_TABLE, 0x1a0));
        (result, arguments)
    }

    #[test]
    fn the_subobject_test_at_0x94_walks_its_four_answers() {
        // The form part answers true: 1, nothing else asked.
        assert_eq!(visible_case(1, 0, 0, 0), (1, vec![]));
        // The actor virtual holds: the answer of 008ace90, as it is.
        assert_eq!(visible_case(0, 1, 7, 1).0, 7);
        assert_eq!(visible_case(0, 1, 0, 1).0, 0);
        // 008ace90 holds: 1 without asking the last virtual.
        assert_eq!(visible_case(0, 0, 1, 1), (1, vec![]));
        // The last virtual decides, with the argument 0.
        let (result, asked) = visible_case(0, 0, 0, 1);
        assert_eq!(result, 1);
        assert_eq!(asked.len(), 1);
        assert_eq!(asked[0][1], 0);
        assert_eq!(visible_case(0, 0, 0, 0).0, 0);
    }

    fn heading_case(skip: u32, virtual_holds: bool, immobile: bool) -> (f32, Vec<(u32, Vec<u32>)>) {
        let mut e = engine();
        let actor = new_actor(&mut e);
        let rotation = e.mem.alloc(0x40);
        e.mem.set_f32(rotation + 4, 0.5);
        e.mem.set_f32(rotation + 8, 1.25);
        e.register_double(ROTATION_POINTER, move |_, _| eax(rotation));
        slot(&mut e, ACTOR_TABLE, 0x100, eax(virtual_holds as u32));
        e.register(GET_BASE_FORM, |_, _| eax(0x7000));
        e.register_double(BASE_FORM_IS_IMMOBILE, move |_, _| eax(immobile as u32));
        e.register(CLAMP_ANGLE, |_, a| st0(f32::from_bits(a[0]) * 2.0));
        e.call_log = Some(vec![]);
        let heading = e.call(0x008bd7b0, &args![actor, skip]).f32();
        (heading, logged(&e))
    }

    #[test]
    fn the_heading_is_clamped_only_for_immobile_actors() {
        let (heading, log) = heading_case(0, true, true);
        assert_eq!(heading, 3.5);
        assert!(log
            .iter()
            .any(|call| call.0 == CLAMP_ANGLE && call.1 == vec![word(1.75)]));
        // Not immobile, the virtual fails or the adjustment is skipped: the
        // stored angle.
        assert_eq!(heading_case(0, true, false).0, 1.25);
        assert_eq!(heading_case(0, false, true).0, 1.25);
        let (heading, log) = heading_case(1, true, true);
        assert_eq!(heading, 1.25);
        assert!(log.iter().all(|call| call.0 != CLAMP_ANGLE));
    }

    // ---- the VATS ray casts --------------------------------------------

    const RAY_COLLECTOR_BLOCK: u32 = 0x0200_a100;
    const POSITION_BLOCK: u32 = 0x0200_a200;
    const OBJECT_BLOCK: u32 = 0x0200_a300;
    /// References of the ray hits, each with the vtable `VATS_REF_TABLE`.
    const VATS_REF_A: u32 = 0x0200_a400;
    const VATS_REF_B: u32 = 0x0200_a440;
    const VATS_REF_C: u32 = 0x0200_a480;
    const VATS_REF_TABLE: u32 = 0x0200_a600;

    /// One ray hit the doubles report: `(fraction, 3D object, reference,
    /// second object)`.
    type Hit = (f32, u32, u32, u32);

    struct Vats {
        e: Engine,
        actor: Ptr<Actor>,
        /// The hits of each cast, in the order of the casts.
        rays: Rc<RefCell<Vec<Vec<Hit>>>>,
        casts: Rc<RefCell<u32>>,
    }

    fn vats_rig(rays: Vec<Vec<Hit>>) -> Vats {
        let mut e = engine();
        for page in [
            0x011d_0000,
            0x011d_1000,
            0x011d_f000,
            0x0101_7000,
            0x0101_9000,
            0x0102_1000,
            0x0102_2000,
            0x0102_3000,
        ] {
            e.map(page, 0x1000);
        }
        e.map(0x0200_a000, 0x1000);
        for reference in [VATS_REF_A, VATS_REF_B, VATS_REF_C] {
            e.mem.set_u32(reference, VATS_REF_TABLE);
        }
        slot(&mut e, VATS_REF_TABLE, 0x100, eax(0));
        let actor = new_actor(&mut e);
        e.set_global(PLAYER_POINTER, 0x0200_a500u32);
        e.set_global(VATS_HEIGHT_SETTING, 8.0f32);
        e.set_global(VATS_STEP_COUNT_SETTING, 3.0f32);
        e.set_global(VATS_STEP_LENGTH_SETTING, 100.0f32);
        e.set_global(VATS_RAY_LENGTH, 10_000.0f32);
        e.set_global(VATS_NOTHING_HIT, f32::MAX);
        e.set_global(VATS_FRACTION_LIMIT, 0.85f64);
        e.set_global(VATS_DISTANCE_LIMIT, 256.0f64);
        e.set_global(VATS_DEBUG_SECONDS, 10.0f32);
        e.set_global(VATS_DEBUG_SETTING, 0u8);
        e.set_global(TES_POINTER, 0x7777u32);
        e.mem.set_f32(POSITION_BLOCK, 1.0);
        e.mem.set_f32(POSITION_BLOCK + 4, 2.0);
        e.mem.set_f32(POSITION_BLOCK + 8, 100.0);
        slot(&mut e, ACTOR_TABLE, 0x1f4, eax(POSITION_BLOCK));
        slot(&mut e, ACTOR_TABLE, 0x2bc, st0(0.25));
        e.register(SETTING_FLOAT_POINTER, |_, a| eax(a[0]));
        e.register(SETTING_BYTE_POINTER, |_, a| eax(a[0]));
        e.register(LIST_NODE_DATA_ADDRESS, |_, a| eax(a[0]));
        quiet(
            &mut e,
            &[
                MATRIX_SET_HEADING,
                MATRIX_GET_COLUMN,
                VECTOR_NORMALIZE,
                RAY_RESET,
                RAY_SET_FROM,
                RAY_SET_TO,
                FILTER_CONSTRUCTOR,
                FILTER_SET_LAYER,
                FILTER_SET_GROUP,
                RAY_SET_FILTER,
                COLLECTOR_CONSTRUCTOR,
                COLLECTOR_DESTRUCTOR,
                RAY_SET_COLLECTOR,
                COLOUR_CONSTRUCTOR,
                TES_ADD_TEMP_DEBUG_OBJECT,
                HIT_RESULT_CONSTRUCTOR,
            ],
        );
        e.register(VECTOR_TIMES_SCALAR, |_, a| eax(a[0]));
        e.register(VECTOR_ADD, |_, a| eax(a[1]));
        e.register(VECTOR_SUBTRACT, |_, a| eax(a[1]));
        e.register(ACTOR_COLLISION_FILTER, |_, a| eax(a[1]));
        e.register(FILTER_GROUP_OF, |_, _| eax(0x55));
        e.register(DEBUG_LINE_OBJECT, |_, _| eax(0x9900));
        e.register(PLAYER_WORLD, |_, _| eax(0x4100));
        e.register(WORLD_PHYSICS, |_, _| eax(OBJECT_BLOCK));
        e.mem.set_u32(OBJECT_BLOCK, OBJECT_BLOCK + 0x100);
        let casts = Rc::new(RefCell::new(0u32));
        let counted = casts.clone();
        let cast_target = 0x0300_ff00;
        e.mem.set_u32(OBJECT_BLOCK + 0x100 + 0xc8, cast_target);
        e.register_double(cast_target, move |_, _| {
            *counted.borrow_mut() += 1;
            Ret::default()
        });
        let rays = Rc::new(RefCell::new(rays));
        e.register(RAY_COLLECTOR, |_, _| eax(RAY_COLLECTOR_BLOCK));
        e.register(COLLECTOR_HITS, |_, _| eax(RAY_COLLECTOR_BLOCK + 0x10));
        let (list, count) = (rays.clone(), casts.clone());
        e.register_double(HIT_COUNT, move |_, _| {
            let ray = (*count.borrow() as usize).saturating_sub(1);
            eax(list.borrow().get(ray).map_or(0, |hits| hits.len() as u32))
        });
        e.register(HIT_AT, |_, a| eax(a[1]));
        let (list, count) = (rays.clone(), casts.clone());
        e.register_double(HIT_RESULT_CONSTRUCTOR, move |e, a| {
            let ray = (*count.borrow() as usize).saturating_sub(1);
            let hit = list.borrow()[ray][a[1] as usize];
            e.mem.set_f32(a[0] + 0x10, hit.0);
            e.mem.set_u32(a[0] + 0x50, a[1] + 1);
            Ret::default()
        });
        let (list, count) = (rays.clone(), casts.clone());
        e.register_double(AV_OBJECT_FOR_COLLIDABLE, move |_, a| {
            let ray = (*count.borrow() as usize).saturating_sub(1);
            eax(list.borrow()[ray][a[0] as usize - 1].1)
        });
        let (list, count) = (rays.clone(), casts.clone());
        e.register_double(COLLIDABLE_OBJECT_004B59F0, move |_, a| {
            let ray = (*count.borrow() as usize).saturating_sub(1);
            eax(list.borrow()[ray][a[0] as usize - 1].3)
        });
        let (list, count) = (rays.clone(), casts.clone());
        e.register_double(FIND_REFERENCE_FOR_3D, move |_, a| {
            // The reference of the 3D object `a[0]` is the one of the hit
            // that reported it.
            let ray = (*count.borrow() as usize).saturating_sub(1);
            let list = list.borrow();
            eax(list[ray]
                .iter()
                .find(|hit| hit.1 == a[0])
                .map_or(0, |hit| hit.2))
        });
        Vats {
            e,
            actor,
            rays,
            casts,
        }
    }

    #[test]
    fn the_area_ray_is_built_from_the_raised_eye_position() {
        let mut vats = vats_rig(vec![vec![]]);
        let (e, actor) = (&mut vats.e, vats.actor);
        e.call_log = Some(vec![]);
        let distance = e.call(0x008bd830, &args![actor, 0u32, 0.5f32]).f32();
        assert_eq!(distance, f32::MAX, "nothing hit");
        // The start is the position with the setting added to its height.
        let from = calls_to(e, RAY_SET_FROM)[0][1];
        assert_eq!(e.mem.f32(from), 1.0);
        assert_eq!(e.mem.f32(from + 8), 108.0);
        // The angle is the virtual's plus the argument, the ray is 10000 long
        // and starts from the start.
        assert_eq!(calls_to(e, MATRIX_SET_HEADING)[0][1], word(0.75));
        let scaling = &calls_to(e, VECTOR_TIMES_SCALAR)[0];
        assert_eq!(scaling[1], word(10_000.0));
        let sum = &calls_to(e, VECTOR_ADD)[0];
        assert_eq!(sum[0], from);
        assert_eq!(sum[2], scaling[0]);
        assert_eq!(calls_to(e, RAY_SET_TO)[0][1], sum[1]);
        assert_eq!(calls_to(e, FILTER_SET_LAYER)[0][1], 0x23);
        assert_eq!(calls_to(e, FILTER_SET_GROUP)[0][1], 0x55);
        assert_eq!(*vats.casts.borrow(), 1);
        assert_eq!(calls_to(e, COLLECTOR_DESTRUCTOR).len(), 1);
        // No debug line without the setting.
        assert!(calls_to(e, DEBUG_LINE_OBJECT).is_empty());
    }

    #[test]
    fn the_area_distance_is_the_first_hit_that_is_not_the_actor() {
        let actor_addr;
        let mut vats = {
            let vats = vats_rig(vec![]);
            actor_addr = vats.actor.addr();
            let hits = vec![vec![
                (0.1, 0x61, actor_addr, 0x71), // the actor itself
                (0.2, 0, VATS_REF_A, 0x72),    // no 3D object
                (0.3, 0x62, VATS_REF_A, 0),    // no second object
                (0.5, 0x63, VATS_REF_B, 0x73), // the answer
                (0.9, 0x64, VATS_REF_C, 0x74), // never looked at
            ]];
            *vats.rays.borrow_mut() = hits;
            vats
        };
        let (e, actor) = (&mut vats.e, vats.actor);
        e.call_log = Some(vec![]);
        let distance = e.call(0x008bd830, &args![actor, 0u32, 0.0f32]).f32();
        assert_eq!(distance, 5000.0);
        let looked_at: Vec<u32> = calls_to(e, HIT_AT).iter().map(|c| c[1]).collect();
        assert_eq!(looked_at, [0, 1, 2, 3]);
    }

    #[test]
    fn the_area_debug_line_is_added_with_the_setting() {
        let mut vats = vats_rig(vec![vec![(0.25, 0x61, VATS_REF_A, 0x71)]]);
        let (e, actor) = (&mut vats.e, vats.actor);
        e.set_global(VATS_DEBUG_SETTING, 1u8);
        e.call_log = Some(vec![]);
        let distance = e.call(0x008bd830, &args![actor, 0u32, 0.0f32]).f32();
        assert_eq!(distance, 2500.0);
        let colour = &calls_to(e, COLOUR_CONSTRUCTOR)[0];
        assert_eq!(colour[1..], [word(1.0), word(1.0), word(0.0), word(1.0)]);
        // The line goes from the start to start + 2500 along the direction.
        let scaling = calls_to(e, VECTOR_TIMES_SCALAR);
        assert_eq!(scaling[1][1], word(2500.0));
        let line = &calls_to(e, DEBUG_LINE_OBJECT)[0];
        assert_eq!(line[1], line[3], "the same colour twice");
        assert_eq!(line[4], 1);
        assert_eq!(
            calls_to(e, TES_ADD_TEMP_DEBUG_OBJECT),
            [vec![0x7777, 0x9900, word(10.0)]]
        );
    }

    fn target_rig(rays: Vec<Vec<Hit>>) -> (Vats, u32) {
        let mut vats = vats_rig(rays);
        vats.e.map(NODE_TABLE, 0x800);
        vats.e.map(CHILD_TABLE, 0x800);
        let target = vats.e.mem.alloc(0x40);
        vats.e.mem.set_u32(target, NODE_TABLE);
        slot(&mut vats.e, NODE_TABLE, 0x1d0, eax(0x6000));
        vats.e
            .register(WORLD_BOUND, |_, _| eax(OBJECT_BLOCK + 0x200));
        vats.e.mem.set_f32(OBJECT_BLOCK + 0x200, 5.0);
        vats.e.mem.set_f32(OBJECT_BLOCK + 0x204, 6.0);
        vats.e.mem.set_f32(OBJECT_BLOCK + 0x208, 7.0);
        (vats, target)
    }

    #[test]
    fn nothing_to_see_without_a_target_or_its_3d_data() {
        let (mut vats, target) = target_rig(vec![]);
        let (e, actor) = (&mut vats.e, vats.actor);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x008bdbd0, &args![actor, 0u32, 0.0f32]).f32(), 0.0);
        slot(e, NODE_TABLE, 0x1d0, eax(0));
        assert_eq!(e.call(0x008bdbd0, &args![actor, target, 0.0f32]).f32(), 0.0);
        assert_eq!(*vats.casts.borrow(), 0);
    }

    #[test]
    fn a_clear_line_is_visible_for_every_step() {
        let (mut vats, target) = target_rig(vec![vec![], vec![], vec![]]);
        let (e, actor) = (&mut vats.e, vats.actor);
        e.call_log = Some(vec![]);
        let seen = e.call(0x008bdbd0, &args![actor, target, 0.5f32]).f32();
        assert_eq!(seen, 300.0);
        assert_eq!(*vats.casts.borrow(), 3);
        // Each ray goes from start + step * n to the centre of the bound.
        let to: Vec<u32> = calls_to(e, RAY_SET_TO).iter().map(|c| c[1]).collect();
        assert_eq!(e.mem.f32(to[0]), 5.0);
        assert_eq!(e.mem.f32(to[0] + 8), 7.0);
        let scaling: Vec<u32> = calls_to(e, VECTOR_TIMES_SCALAR)
            .iter()
            .map(|c| c[1])
            .collect();
        assert_eq!(scaling, [word(100.0), word(200.0), word(300.0)]);
        assert_eq!(calls_to(e, COLLECTOR_DESTRUCTOR).len(), 3);
        assert_eq!(calls_to(e, MATRIX_SET_HEADING)[0][1], word(0.75));
    }

    #[test]
    fn a_block_ends_the_search_at_the_previous_step() {
        // The second ray is blocked at a short fraction.
        let blocker = (0.5, 0x62, VATS_REF_A, 0x72);
        let (mut vats, target) = target_rig(vec![vec![], vec![blocker], vec![]]);
        let (e, actor) = (&mut vats.e, vats.actor);
        assert_eq!(
            e.call(0x008bdbd0, &args![actor, target, 0.0f32]).f32(),
            100.0
        );
        assert_eq!(*vats.casts.borrow(), 2);
        // Blocked at once: nothing is visible.
        let (mut vats, target) = target_rig(vec![vec![blocker]]);
        let (e, actor) = (&mut vats.e, vats.actor);
        assert_eq!(e.call(0x008bdbd0, &args![actor, target, 0.0f32]).f32(), 0.0);
    }

    #[test]
    fn hits_of_the_target_itself_and_far_fractions_do_not_block() {
        let (mut vats, target) = target_rig(vec![vec![]]);
        let blockers = vec![
            // The target itself.
            vec![(0.5, 0x62, target, 0x72)],
            // Past 0.85 of the ray.
            vec![(0.9, 0x62, VATS_REF_A, 0x72)],
            // No 3D object or no second object.
            vec![(0.5, 0, VATS_REF_A, 0x72), (0.5, 0x63, VATS_REF_A, 0)],
        ];
        for hits in blockers {
            *vats.rays.borrow_mut() = vec![hits, vec![], vec![]];
            *vats.casts.borrow_mut() = 0;
            let (e, actor) = (&mut vats.e, vats.actor);
            assert_eq!(
                e.call(0x008bdbd0, &args![actor, target, 0.0f32]).f32(),
                300.0
            );
        }
    }

    #[test]
    fn a_far_actor_like_blocker_is_ignored_beyond_256() {
        let (mut vats, target) = target_rig(vec![vec![]]);
        let reference = vats.e.mem.alloc(0x40);
        vats.e.mem.set_u32(reference, NODE_TABLE + 0x200);
        vats.e.map(NODE_TABLE + 0x200, 0x200);
        slot(&mut vats.e, NODE_TABLE + 0x200, 0x100, eax(1));
        vats.e.register(VECTOR_LENGTH, |_, _| st0(600.0));
        let blocker = (0.5, 0x62, reference, 0x72);
        // 600 * 0.5 = 300 > 256: not a block.
        *vats.rays.borrow_mut() = vec![vec![blocker], vec![], vec![]];
        let (e, actor) = (&mut vats.e, vats.actor);
        assert_eq!(
            e.call(0x008bdbd0, &args![actor, target, 0.0f32]).f32(),
            300.0
        );
        // 400 * 0.5 = 200: a block.
        e.register(VECTOR_LENGTH, |_, _| st0(400.0));
        *vats.casts.borrow_mut() = 0;
        assert_eq!(e.call(0x008bdbd0, &args![actor, target, 0.0f32]).f32(), 0.0);
        // The virtual does not hold: a block whatever the length.
        slot(e, NODE_TABLE + 0x200, 0x100, eax(0));
        e.register(VECTOR_LENGTH, |_, _| st0(600.0));
        *vats.casts.borrow_mut() = 0;
        assert_eq!(e.call(0x008bdbd0, &args![actor, target, 0.0f32]).f32(), 0.0);
    }

    #[test]
    fn the_visibility_debug_line_goes_from_the_centre_to_the_ray_end() {
        let (mut vats, target) = target_rig(vec![vec![], vec![], vec![]]);
        let (e, actor) = (&mut vats.e, vats.actor);
        e.set_global(VATS_DEBUG_SETTING, 1u8);
        e.call_log = Some(vec![]);
        e.call(0x008bdbd0, &args![actor, target, 0.0f32]);
        let lines = calls_to(e, DEBUG_LINE_OBJECT);
        assert_eq!(lines.len(), 3);
        let to = calls_to(e, RAY_SET_TO)[0][1];
        assert_eq!(lines[0][0], to, "from the centre of the bound");
        assert_eq!(lines[0][2], calls_to(e, RAY_SET_FROM)[0][1]);
        assert_eq!(calls_to(e, TES_ADD_TEMP_DEBUG_OBJECT).len(), 3);
    }

    // ---- the fire node -------------------------------------------------

    struct Fire {
        e: Engine,
        actor: Ptr<Actor>,
        process: u32,
    }

    fn fire_rig() -> Fire {
        let mut e = engine();
        let actor = new_actor(&mut e);
        let process = with_process(&mut e, actor);
        slot(&mut e, PROCESS_TABLE, 0x184, eax(0));
        slot(&mut e, PROCESS_TABLE, 0x188, Ret::default());
        slot(&mut e, ACTOR_TABLE, 0x1d0, eax(0x6000));
        e.set_global(PLAYER_POINTER, 0x0200_a500u32);
        e.register(PLAYER_CURRENT_3D, |_, _| eax(0x6100));
        e.register(ACTOR_CURRENT_WEAPON, |_, _| eax(0));
        e.register(WEAPON_FIRE_NODE, |_, _| eax(0x6200));
        e.register(FIND_NODE_BY_NAME, |_, _| eax(0x6300));
        e.call_log = Some(vec![]);
        Fire { e, actor, process }
    }

    #[test]
    fn the_fire_node_needs_a_process() {
        let mut fire = fire_rig();
        fire.e.set(fire.actor, Actor::pCurrentProcess, Ptr::NULL);
        assert_eq!(fire.e.call(0x008be0a0, &args![fire.actor]).u32(), 0);
        assert!(logged(&fire.e).len() == 1);
    }

    #[test]
    fn a_cached_fire_node_is_returned_as_it_is() {
        let mut fire = fire_rig();
        slot(&mut fire.e, PROCESS_TABLE, 0x184, eax(0x6400));
        assert_eq!(fire.e.call(0x008be0a0, &args![fire.actor]).u32(), 0x6400);
        assert!(calls_to(&fire.e, slot_target(PROCESS_TABLE, 0x188)).is_empty());
    }

    #[test]
    fn the_fire_node_comes_from_the_weapon_or_the_node_name() {
        let mut fire = fire_rig();
        let setter = slot_target(PROCESS_TABLE, 0x188);
        // No weapon: the node named ProjectileNode, stored in the process.
        assert_eq!(fire.e.call(0x008be0a0, &args![fire.actor]).u32(), 0x6300);
        assert_eq!(
            calls_to(&fire.e, FIND_NODE_BY_NAME),
            [vec![0x6000, PROJECTILE_NODE_NAME]]
        );
        assert_eq!(calls_to(&fire.e, setter), [vec![fire.process, 0x6300]]);
        // A weapon: its own fire node.
        fire.e.register(ACTOR_CURRENT_WEAPON, |_, _| eax(0x6500));
        assert_eq!(fire.e.call(0x008be0a0, &args![fire.actor]).u32(), 0x6200);
        assert_eq!(calls_to(&fire.e, WEAPON_FIRE_NODE), [vec![0x6500, 0x6000]]);
        assert_eq!(calls_to(&fire.e, setter).len(), 2);
    }

    #[test]
    fn the_player_uses_its_current_3d_and_no_3d_gives_no_node() {
        let mut fire = fire_rig();
        let player = fire.actor.addr();
        fire.e.set_global(PLAYER_POINTER, player);
        assert_eq!(fire.e.call(0x008be0a0, &args![fire.actor]).u32(), 0x6300);
        assert_eq!(calls_to(&fire.e, FIND_NODE_BY_NAME)[0][0], 0x6100);
        // No 3D data: zero, and nothing stored.
        let mut fire = fire_rig();
        slot(&mut fire.e, ACTOR_TABLE, 0x1d0, eax(0));
        assert_eq!(fire.e.call(0x008be0a0, &args![fire.actor]).u32(), 0);
        assert!(calls_to(&fire.e, slot_target(PROCESS_TABLE, 0x188)).is_empty());
    }

    #[test]
    fn the_fire_node_setter_needs_a_process() {
        let mut fire = fire_rig();
        fire.e.call(0x008be180, &args![fire.actor, 0x6600u32]);
        assert_eq!(
            calls_to(&fire.e, slot_target(PROCESS_TABLE, 0x188)),
            [vec![fire.process, 0x6600]]
        );
        fire.e.set(fire.actor, Actor::pCurrentProcess, Ptr::NULL);
        fire.e.call_log = Some(vec![]);
        fire.e.call(0x008be180, &args![fire.actor, 0x6600u32]);
        assert!(calls_to(&fire.e, slot_target(PROCESS_TABLE, 0x188)).is_empty());
    }

    // ---- the cached-value readers --------------------------------------

    /// Checks one process-level reader and its actor-level wrapper:
    /// `mask` is the flag, `offset` the cached field and the owner virtual.
    fn check_cached(wrapper: u32, reader: u32, mask: u32, offset: u32, word_value: bool) {
        let mut e = engine();
        e.map(CACHE_OWNER_TABLE, 0x100);
        let actor = new_actor(&mut e);
        e.mem.set_u32(actor.addr() + 0xa8, CACHE_OWNER_TABLE);
        slot(
            &mut e,
            CACHE_OWNER_TABLE,
            offset,
            Ret {
                eax: 0x1234,
                st0: 7.5,
                ..Ret::default()
            },
        );
        let owner = actor.addr() + 0xa8;
        let process = e.mem.alloc(0x100);
        let block = e.mem.alloc(0x80);
        let cached = 1.5f32;
        let value = |ret: Ret| -> u32 {
            if word_value {
                ret.eax
            } else {
                (ret.st0 as f32).to_bits()
            }
        };
        let computed = if word_value { 0x1234 } else { word(7.5) };
        let stored = word(cached);
        // Reader: no block, zero.
        let ret = e.call(reader, &args![process, owner]);
        assert_eq!(value(ret), 0);
        // With a block: the flag decides between the stored and the computed.
        e.mem.set_u32(process + 0x2c, block);
        e.mem.set_f32(block + offset, cached);
        e.mem.set_u32(block + 0x44, mask);
        assert_eq!(value(e.call(reader, &args![process, owner])), stored);
        e.mem.set_u32(block + 0x44, !mask);
        e.call_log = Some(vec![]);
        assert_eq!(value(e.call(reader, &args![process, owner])), computed);
        assert_eq!(
            calls_to(&e, slot_target(CACHE_OWNER_TABLE, offset)),
            [vec![owner]],
            "the owner is the virtual's this"
        );
        // Wrapper without a process: the owner's virtual directly.
        e.call_log = Some(vec![]);
        assert_eq!(value(e.call(wrapper, &args![actor])), computed);
        assert_eq!(
            calls_to(&e, slot_target(CACHE_OWNER_TABLE, offset)),
            [vec![owner]]
        );
        // With a process that has the block: the reader.
        e.set(actor, Actor::pCurrentProcess, Ptr::new(process));
        e.mem.set_u32(block + 0x44, mask);
        e.call_log = Some(vec![]);
        assert_eq!(value(e.call(wrapper, &args![actor])), stored);
        assert!(calls_to(&e, slot_target(CACHE_OWNER_TABLE, offset)).is_empty());
        // A process without the block goes to the owner again.
        e.mem.set_u32(process + 0x2c, 0);
        assert_eq!(value(e.call(wrapper, &args![actor])), computed);
    }

    macro_rules! cached_test {
        ($name:ident, $wrapper:literal, $reader:literal, $mask:literal, $offset:literal, $word:literal) => {
            #[test]
            fn $name() {
                check_cached($wrapper, $reader, $mask, $offset, $word);
            }
        };
    }

    cached_test!(radius_is_cached, 0x008be1b0, 0x008be220, 0x1, 0x00, false);
    cached_test!(width_is_cached, 0x008be280, 0x008be2f0, 0x2, 0x04, false);
    cached_test!(length_is_cached, 0x008be350, 0x008be3c0, 0x4, 0x08, false);
    cached_test!(
        forward_length_is_cached,
        0x008be420,
        0x008be490,
        0x8000,
        0x0c,
        false
    );
    cached_test!(
        weapon_dps_is_cached,
        0x008be600,
        0x008be670,
        0x8,
        0x10,
        false
    );
    cached_test!(
        healing_rate_is_cached,
        0x008be6d0,
        0x008be740,
        0x40,
        0x20,
        false
    );
    cached_test!(
        endurance_is_cached,
        0x008be7a0,
        0x008be810,
        0x80,
        0x24,
        false
    );
    cached_test!(
        perception_condition_is_cached_by_the_getter,
        0x008be870,
        0x008be8e0,
        0x100,
        0x28,
        false
    );
    cached_test!(
        eye_level_is_cached,
        0x008be940,
        0x008be9b0,
        0x200,
        0x2c,
        false
    );
    cached_test!(
        aggression_is_cached,
        0x008bea10,
        0x008bea80,
        0x400,
        0x30,
        true
    );
    cached_test!(
        assistance_is_cached,
        0x008beae0,
        0x008beb50,
        0x800,
        0x34,
        true
    );
    cached_test!(
        medicine_multiplier_is_cached,
        0x008bebb0,
        0x008bec20,
        0x10,
        0x14,
        false
    );

    #[test]
    fn paralysis_is_true_above_zero() {
        let mut e = engine();
        e.map(CACHE_OWNER_TABLE, 0x100);
        let actor = new_actor(&mut e);
        e.mem.set_u32(actor.addr() + 0xa8, CACHE_OWNER_TABLE);
        e.set_global(DOUBLE_ZERO, 0.0f64);
        // Through the owner's virtual.
        slot(&mut e, CACHE_OWNER_TABLE, 0x1c, st0(0.5));
        assert!(e.call(0x008be4f0, &args![actor]).bool());
        slot(&mut e, CACHE_OWNER_TABLE, 0x1c, st0(0.0));
        assert!(!e.call(0x008be4f0, &args![actor]).bool());
        slot(&mut e, CACHE_OWNER_TABLE, 0x1c, st0(-1.0));
        assert!(!e.call(0x008be4f0, &args![actor]).bool());
        // Through a cached value.
        let process = e.mem.alloc(0x100);
        let block = e.mem.alloc(0x80);
        e.mem.set_u32(process + 0x2c, block);
        e.mem.set_u32(block + 0x44, 0x20);
        e.set(actor, Actor::pCurrentProcess, Ptr::new(process));
        e.mem.set_f32(block + 0x1c, 2.0);
        assert!(e.call(0x008be4f0, &args![actor]).bool());
        e.mem.set_f32(block + 0x1c, 0.0);
        assert!(!e.call(0x008be4f0, &args![actor]).bool());
    }

    // ---- 008bcdf0, the ragdoll update ----------------------------------

    const RAGDOLL_TABLE: u32 = 0x0200_b000;
    const TAIL_TABLE: u32 = 0x0200_b800;

    struct Ragdoll {
        e: Engine,
        actor: Ptr<Actor>,
        ragdoll: u32,
    }

    /// An actor with a ragdoll controller and doubles for every callee: the
    /// bound is far (the actor is out of range) unless a test changes it.
    fn ragdoll_rig() -> Ragdoll {
        let mut e = engine();
        e.map(RAGDOLL_TABLE, 0x800);
        e.map(TAIL_TABLE, 0x800);
        for page in [0x0126_7000, 0x011e_0000, 0x0102_1000, 0x011f_2000] {
            e.map(page, 0x1000);
        }
        let actor = new_actor(&mut e);
        let ragdoll = e.mem.alloc(0x300);
        e.set(actor, Actor::pRagdollController, Ptr::new(ragdoll));
        e.set_global(PLAYER_POINTER, 0x0200_a500u32);
        e.set_global(POSITION_SOURCE_A, 0u32);
        e.set_global(POSITION_SOURCE_B, OBJECT_BLOCK);
        e.set_global(0x0126_7c6c, 100.0f32);
        e.set_global(BOUND_RADIUS_FACTOR, 1.25f64);
        e.register(SETTING_FLOAT_POINTER, |_, a| eax(a[0]));
        e.register(RAGDOLL_INIT_TEST, |_, _| eax(1));
        e.register(PLAYER_TEST_004EAF60, |_, _| eax(0));
        e.register(PLAYER_TEST_00524D10, |_, _| eax(1));
        e.register(LIST_NODE_DATA_ADDRESS, |_, a| eax(a[0]));
        e.register(OBJECT_POSITION, |_, a| eax(a[0]));
        slot(&mut e, ACTOR_TABLE, 0x1d0, eax(0x6000));
        e.register(WORLD_BOUND, |_, _| eax(OBJECT_BLOCK + 0x200));
        e.mem.set_f32(OBJECT_BLOCK + 0x20c, 10.0);
        e.register(VECTOR_SUBTRACT, |_, a| eax(a[1]));
        e.register(VECTOR_LENGTH, |_, _| st0(500.0));
        e.register(BOUND_RADIUS, |_, _| st0(10.0));
        quiet(
            &mut e,
            &[
                RAGDOLL_TEST_00552490,
                RAGDOLL_TEST_008A3BD0,
                RAGDOLL_ENABLE_ANIM,
                ACTOR_SET_HAVOK_WEAPON,
                RAGDOLL_SET_005BA130,
                RAGDOLL_SET_00C747D0,
                RAGDOLL_MODE_FOUR,
                RAGDOLL_MODE_OTHER,
                RAGDOLL_SET_008A3BF0,
                RAGDOLL_SET_FEEDBACK_ACTIVE,
                RAGDOLL_SET_00C75580,
                RAGDOLL_TEST_00C78090,
                RAGDOLL_TEST_0089D690,
                RAGDOLL_TEST_00888A50,
                FOUR_WORD_ASSIGN,
            ],
        );
        slot(&mut e, ACTOR_TABLE, 0x22c, eax(0));
        slot(&mut e, ACTOR_TABLE, 0x1e4, eax(0));
        slot(&mut e, ACTOR_TABLE, 0x214, eax(0));
        slot(&mut e, ACTOR_TABLE, 0x364, eax(0));
        e.register(ACTOR_FLAGS_008846E0, |_, _| eax(0));
        e.register(ENTRY_OBJECT, |_, _| eax(0));
        e.register(ACTOR_FACE_ANIMATION_DATA, |_, _| eax(0));
        let process = with_process(&mut e, actor);
        slot(&mut e, PROCESS_TABLE, 0x4bc, eax(3));
        e.register_double(ACTOR_PROCESS, move |_, _| eax(process));
        e.register(PROCESS_TEST_0045CD60, |_, _| eax(1));
        e.register(PROCESS_ACTOR_MOVER, |_, _| eax(0));
        e.register(MOVER_MODE_005F36F0, |_, _| eax(1));
        Ragdoll { e, actor, ragdoll }
    }

    fn update(rig: &mut Ragdoll, flag: u8) -> bool {
        rig.e
            .call(0x008bcdf0, &args![rig.actor, flag as u32, 0u32])
            .bool()
    }

    fn in_range(rig: &Ragdoll) -> bool {
        rig.e.get(rig.actor, Actor::bFootIKInRange)
    }

    #[test]
    fn no_controller_or_an_uninitialised_one_answers_false() {
        let mut rig = ragdoll_rig();
        rig.e.call_log = Some(vec![]);
        rig.e.register(RAGDOLL_INIT_TEST, |_, _| eax(0));
        assert!(!update(&mut rig, 1));
        assert_eq!(logged_addresses(&rig.e), [0x008bcdf0, RAGDOLL_INIT_TEST]);
        let actor = rig.actor;
        rig.e.set(actor, Actor::pRagdollController, Ptr::NULL);
        rig.e.call_log = Some(vec![]);
        assert!(!update(&mut rig, 1));
        assert_eq!(logged_addresses(&rig.e), [0x008bcdf0]);
    }

    #[test]
    fn without_the_update_flag_only_the_checks_run() {
        let mut rig = ragdoll_rig();
        rig.e.call_log = Some(vec![]);
        assert!(!update(&mut rig, 0));
        assert!(calls_to(&rig.e, slot_target(ACTOR_TABLE, 0x1d0)).is_empty());
        assert!(calls_to(&rig.e, RAGDOLL_ENABLE_ANIM).is_empty());
    }

    #[test]
    fn foot_ik_range_compares_the_margin_with_the_setting() {
        // Distance 500, radius 10 * 1.25: margin 487.5 against 100: out.
        let mut rig = ragdoll_rig();
        update(&mut rig, 1);
        assert!(!in_range(&rig));
        // The setting above the margin: in range.
        rig.e.set_global(0x0126_7c6c, 487.75f32);
        update(&mut rig, 1);
        assert!(in_range(&rig));
        // Exactly the margin is not in range.
        rig.e.set_global(0x0126_7c6c, 487.5f32);
        update(&mut rig, 1);
        assert!(!in_range(&rig));
        // No scene: the flag is cleared and stays so.
        slot(&mut rig.e, ACTOR_TABLE, 0x1d0, eax(0));
        let actor = rig.actor;
        rig.e.set(actor, Actor::bFootIKInRange, true);
        update(&mut rig, 1);
        assert!(!in_range(&rig));
    }

    #[test]
    fn the_position_comes_from_a_global_object_or_the_players_eye() {
        let mut rig = ragdoll_rig();
        rig.e.call_log = Some(vec![]);
        update(&mut rig, 1);
        assert_eq!(calls_to(&rig.e, OBJECT_POSITION), [vec![OBJECT_BLOCK]]);
        // The first object counts only while the player test holds.
        rig.e.set_global(POSITION_SOURCE_A, 0x0200_a900u32);
        rig.e.call_log = Some(vec![]);
        update(&mut rig, 1);
        assert_eq!(calls_to(&rig.e, OBJECT_POSITION), [vec![OBJECT_BLOCK]]);
        rig.e.register(PLAYER_TEST_004EAF60, |_, _| eax(1));
        rig.e.call_log = Some(vec![]);
        update(&mut rig, 1);
        assert_eq!(calls_to(&rig.e, OBJECT_POSITION), [vec![0x0200_a900]]);
    }

    #[test]
    fn without_global_objects_the_position_is_the_players_eye() {
        let mut rig = ragdoll_rig();
        rig.e.set_global(POSITION_SOURCE_B, 0u32);
        let player = rig.e.mem.alloc(0x200);
        rig.e.mem.set_u32(player, ACTOR_TABLE);
        rig.e.mem.set_u32(player + 0xa8, CACHE_OWNER_TABLE);
        rig.e.map(CACHE_OWNER_TABLE, 0x100);
        slot(&mut rig.e, CACHE_OWNER_TABLE, 0x2c, st0(1.75));
        rig.e.set_global(PLAYER_POINTER, player);
        let positions = player + 0x30;
        rig.e
            .register_double(REFERENCE_POSITION, move |_, _| eax(positions));
        rig.e.register(VECTOR_TIMES_SCALAR, |_, a| eax(a[0]));
        rig.e.register(VECTOR_ADD, |_, a| eax(a[1]));
        rig.e.call_log = Some(vec![]);
        update(&mut rig, 1);
        let scaling = &calls_to(&rig.e, VECTOR_TIMES_SCALAR)[0];
        assert_eq!(scaling[1], word(1.75));
        assert_eq!(scaling[2], EYE_DIRECTION);
        let sum = &calls_to(&rig.e, VECTOR_ADD)[0];
        assert_eq!(sum[0], positions);
        assert_eq!(sum[2], scaling[0]);
        assert!(calls_to(&rig.e, OBJECT_POSITION).is_empty());
    }

    #[test]
    fn out_of_range_the_ease_out_flags_are_raised_for_an_active_ragdoll() {
        let mut rig = ragdoll_rig();
        // The ragdoll test fails: the flags stay and the update answers false.
        assert!(!update(&mut rig, 1));
        assert_eq!(rig.e.mem.u8(rig.ragdoll + 0x43), 0);
        // The test holds: the flags are raised, which keeps the update going.
        rig.e.register(RAGDOLL_TEST_00552490, |_, _| eax(1));
        assert!(update(&mut rig, 1));
        assert_eq!(rig.e.mem.u8(rig.ragdoll + 0x43), 1);
        assert_eq!(rig.e.mem.u8(rig.ragdoll + 0x221), 1);
        // A hidden player does not raise them.
        let mut rig = ragdoll_rig();
        let actor = rig.actor.addr();
        rig.e.set_global(PLAYER_POINTER, actor);
        rig.e.register(PLAYER_TEST_004EAF60, |_, _| eax(0));
        rig.e.register(RAGDOLL_TEST_00552490, |_, _| eax(1));
        assert!(!update(&mut rig, 1));
        assert_eq!(rig.e.mem.u8(rig.ragdoll + 0x43), 0);
        // An already raised flag is left alone and counts.
        let mut rig = ragdoll_rig();
        rig.e.register(RAGDOLL_TEST_00552490, |_, _| eax(1));
        rig.e.mem.set_u8(rig.ragdoll + 0x43, 1);
        rig.e.call_log = Some(vec![]);
        assert!(update(&mut rig, 1));
    }

    fn near(rig: &mut Ragdoll) {
        rig.e.set_global(0x0126_7c6c, 1000.0f32);
    }

    #[test]
    fn in_range_the_ragdoll_animation_is_enabled_unless_the_virtual_holds() {
        let mut rig = ragdoll_rig();
        near(&mut rig);
        rig.e.call_log = Some(vec![]);
        // The test fails: the animation is enabled and the havok weapon set.
        assert!(update(&mut rig, 1));
        assert_eq!(calls_to(&rig.e, RAGDOLL_ENABLE_ANIM).len(), 1);
        assert_eq!(calls_to(&rig.e, ACTOR_SET_HAVOK_WEAPON).len(), 1);
        assert_eq!(
            calls_to(&rig.e, slot_target(ACTOR_TABLE, 0x22c)),
            [vec![rig.actor.addr(), 0]]
        );
        // The test holds and the second does not: nothing.
        rig.e.register(RAGDOLL_TEST_00552490, |_, _| eax(1));
        rig.e.call_log = Some(vec![]);
        assert!(update(&mut rig, 1));
        assert!(calls_to(&rig.e, RAGDOLL_ENABLE_ANIM).is_empty());
        // Both hold: enabled, but the havok weapon is left.
        rig.e.register(RAGDOLL_TEST_008A3BD0, |_, _| eax(1));
        rig.e.call_log = Some(vec![]);
        update(&mut rig, 1);
        assert_eq!(calls_to(&rig.e, RAGDOLL_ENABLE_ANIM).len(), 1);
        assert!(calls_to(&rig.e, ACTOR_SET_HAVOK_WEAPON).is_empty());
        // The actor virtual holds: nothing is enabled.
        slot(&mut rig.e, ACTOR_TABLE, 0x22c, eax(1));
        rig.e.call_log = Some(vec![]);
        update(&mut rig, 1);
        assert!(calls_to(&rig.e, RAGDOLL_ENABLE_ANIM).is_empty());
    }

    fn argument_of(rig: &Ragdoll, addr: u32) -> Vec<Vec<u32>> {
        calls_to(&rig.e, addr)
    }

    #[test]
    fn the_flags_are_handed_to_the_controller() {
        let mut rig = ragdoll_rig();
        near(&mut rig);
        rig.e.call_log = Some(vec![]);
        // Nothing prevents the ragdoll, no flag bits: enabled, in range, not
        // masked.
        assert!(update(&mut rig, 1));
        let ragdoll = rig.ragdoll;
        assert_eq!(argument_of(&rig, RAGDOLL_SET_005BA130), [vec![ragdoll, 1]]);
        assert_eq!(argument_of(&rig, RAGDOLL_SET_00C747D0), [vec![ragdoll, 1]]);
        assert_eq!(rig.e.mem.u8(ragdoll + 0x243), 1, "no 0xf flag bit");
        // The 0xf bits: masked, so the last flag clears and the ragdoll
        // is not "unmasked" unless 0x8500 is also set.
        rig.e.register(ACTOR_FLAGS_008846E0, |_, _| eax(0x3));
        rig.e.call_log = Some(vec![]);
        update(&mut rig, 1);
        assert_eq!(argument_of(&rig, RAGDOLL_SET_00C747D0), [vec![ragdoll, 0]]);
        assert_eq!(rig.e.mem.u8(ragdoll + 0x243), 0);
        rig.e.register(ACTOR_FLAGS_008846E0, |_, _| eax(0x8503));
        rig.e.call_log = Some(vec![]);
        update(&mut rig, 1);
        assert_eq!(argument_of(&rig, RAGDOLL_SET_00C747D0), [vec![ragdoll, 1]]);
        // A player for whom the second test fails is not wanted at all, so the
        // update stops before the flags.
        let actor = rig.actor.addr();
        rig.e.set_global(PLAYER_POINTER, actor);
        rig.e.register(PLAYER_TEST_00524D10, |_, _| eax(0));
        rig.e.call_log = Some(vec![]);
        assert!(!update(&mut rig, 1));
        assert!(argument_of(&rig, RAGDOLL_SET_00C747D0).is_empty());
    }

    #[test]
    fn what_prevents_the_ragdoll_disables_it() {
        // Each condition alone turns the first flag off.
        type Prevention = fn(&mut Ragdoll);
        let cases: [(&str, Prevention); 4] = [
            ("virtual 0x214", |rig| {
                slot(&mut rig.e, ACTOR_TABLE, 0x214, eax(5))
            }),
            ("virtual 0x364 (008bd287)", |rig| {
                slot(&mut rig.e, ACTOR_TABLE, 0x364, eax(1))
            }),
            ("test c78090", |rig| {
                rig.e.register(RAGDOLL_TEST_00C78090, |_, _| eax(1))
            }),
            ("animation 5f4d60", |rig| {
                slot(&mut rig.e, ACTOR_TABLE, 0x1e4, eax(0x8100));
                rig.e.register(ANIMATION_SUB_OBJECT, |_, _| eax(0x8200));
                rig.e.register(ANIMATION_STATE_OBJECT, |_, _| eax(0x8300));
                rig.e.register(ANIMATION_STATE_TEST, |_, _| eax(1));
            }),
        ];
        for (name, prevent) in cases {
            let mut rig = ragdoll_rig();
            near(&mut rig);
            prevent(&mut rig);
            rig.e.call_log = Some(vec![]);
            update(&mut rig, 1);
            assert_eq!(
                argument_of(&rig, RAGDOLL_SET_005BA130),
                [vec![rig.ragdoll, 0]],
                "{name}"
            );
        }
    }

    #[test]
    fn some_animation_codes_count_as_animating() {
        for (code, animating) in [
            (0xe3, true),
            (0xe4, false),
            (0xe5, true),
            (0xf0, false),
            (0xf1, true),
            (0xf4, true),
            (0xf5, false),
            (0x10, false),
        ] {
            let mut rig = ragdoll_rig();
            near(&mut rig);
            slot(&mut rig.e, ACTOR_TABLE, 0x1e4, eax(0x8100));
            rig.e.register(ANIMATION_SUB_OBJECT, |_, _| eax(0x8200));
            rig.e.register(ANIMATION_STATE_OBJECT, |_, _| eax(0x8300));
            rig.e.register(ANIMATION_STATE_TEST, |_, _| eax(0));
            rig.e
                .register_double(ANIMATION_STATE_CODE, move |_, _| eax(code));
            rig.e.call_log = Some(vec![]);
            update(&mut rig, 1);
            assert_eq!(
                argument_of(&rig, RAGDOLL_SET_005BA130),
                [vec![rig.ragdoll, !animating as u32]],
                "code {code:#x}"
            );
            // The sub-object is asked for once to test and again for each use.
            assert_eq!(
                calls_to(&rig.e, ANIMATION_SUB_OBJECT).len(),
                3,
                "code {code:#x}"
            );
        }
        // No sub-object: not animating, asked once.
        let mut rig = ragdoll_rig();
        near(&mut rig);
        slot(&mut rig.e, ACTOR_TABLE, 0x1e4, eax(0x8100));
        rig.e.register(ANIMATION_SUB_OBJECT, |_, _| eax(0));
        rig.e.call_log = Some(vec![]);
        update(&mut rig, 1);
        assert_eq!(calls_to(&rig.e, ANIMATION_SUB_OBJECT).len(), 1);
        assert_eq!(
            argument_of(&rig, RAGDOLL_SET_005BA130),
            [vec![rig.ragdoll, 1]]
        );
    }

    #[test]
    fn the_mode_word_picks_the_controller_setup() {
        let mut rig = ragdoll_rig();
        near(&mut rig);
        rig.e.call_log = Some(vec![]);
        update(&mut rig, 1);
        assert_eq!(calls_to(&rig.e, RAGDOLL_MODE_OTHER).len(), 1);
        assert!(calls_to(&rig.e, RAGDOLL_MODE_FOUR).is_empty());
        rig.e.register(ENTRY_OBJECT, |_, _| eax(4));
        rig.e.call_log = Some(vec![]);
        update(&mut rig, 1);
        assert_eq!(calls_to(&rig.e, RAGDOLL_MODE_FOUR).len(), 1);
        assert_eq!(
            calls_to(&rig.e, ENTRY_OBJECT),
            [vec![RAGDOLL_MODE_OBJECT]],
            "the mode object is the fixed one"
        );
    }

    #[test]
    fn face_data_or_its_default_goes_to_the_controller() {
        let mut rig = ragdoll_rig();
        near(&mut rig);
        rig.e.call_log = Some(vec![]);
        update(&mut rig, 1);
        let ragdoll = rig.ragdoll;
        assert_eq!(
            calls_to(&rig.e, FOUR_WORD_ASSIGN),
            [vec![ragdoll + 0xc0, FACE_DATA_DEFAULT]]
        );
        // With face data: its virtual 0x9c fills a temporary handed on.
        let face = rig.e.mem.alloc(0x40);
        rig.e.mem.set_u32(face, TAIL_TABLE);
        slot(&mut rig.e, TAIL_TABLE, 0x9c, Ret::default());
        rig.e
            .register_double(ACTOR_FACE_ANIMATION_DATA, move |_, _| eax(face));
        rig.e.call_log = Some(vec![]);
        update(&mut rig, 1);
        let filled = &calls_to(&rig.e, slot_target(TAIL_TABLE, 0x9c))[0];
        assert_eq!(filled[0], face);
        let assigned = &calls_to(&rig.e, FOUR_WORD_ASSIGN)[0];
        assert_eq!(assigned, &vec![ragdoll + 0xc0, filled[1]]);
    }

    #[test]
    fn the_process_kind_and_the_mover_mode_adjust_the_controller() {
        let mut rig = ragdoll_rig();
        near(&mut rig);
        let process = rig.e.mem.alloc(0x40);
        rig.e.mem.set_u32(process, PROCESS_TABLE);
        rig.e
            .register_double(ACTOR_PROCESS, move |_, _| eax(process));
        slot(&mut rig.e, PROCESS_TABLE, 0x4bc, eax(3));
        rig.e.register(PROCESS_TEST_0045CD60, |_, _| eax(0));
        rig.e.register(PROCESS_ACTOR_MOVER, |_, _| eax(0x8400));
        rig.e.register(MOVER_MODE_005F36F0, |_, _| eax(1));
        rig.e.call_log = Some(vec![]);
        update(&mut rig, 1);
        let ragdoll = rig.ragdoll;
        // Kind 3 and mover mode 1: nothing.
        assert!(calls_to(&rig.e, RAGDOLL_SET_008A3BF0).is_empty());
        assert!(calls_to(&rig.e, RAGDOLL_SET_FEEDBACK_ACTIVE).is_empty());
        assert!(calls_to(&rig.e, RAGDOLL_SET_00C75580).is_empty());
        // Kinds 5 and 10 raise the flag.
        for kind in [5, 10] {
            slot(&mut rig.e, PROCESS_TABLE, 0x4bc, eax(kind));
            rig.e.call_log = Some(vec![]);
            update(&mut rig, 1);
            assert_eq!(
                calls_to(&rig.e, RAGDOLL_SET_008A3BF0),
                [vec![ragdoll, 1]],
                "kind {kind}"
            );
        }
        // Mover mode 2: the two tests decide the two resets.
        rig.e.register(MOVER_MODE_005F36F0, |_, _| eax(2));
        rig.e.call_log = Some(vec![]);
        update(&mut rig, 1);
        assert!(calls_to(&rig.e, RAGDOLL_SET_FEEDBACK_ACTIVE).is_empty());
        rig.e.register(RAGDOLL_TEST_0089D690, |_, _| eax(1));
        rig.e.call_log = Some(vec![]);
        update(&mut rig, 1);
        assert_eq!(
            calls_to(&rig.e, RAGDOLL_SET_FEEDBACK_ACTIVE),
            [vec![ragdoll, 0]]
        );
        assert!(calls_to(&rig.e, RAGDOLL_SET_00C75580).is_empty());
        rig.e.register(RAGDOLL_TEST_00888A50, |_, _| eax(1));
        rig.e.call_log = Some(vec![]);
        update(&mut rig, 1);
        assert_eq!(calls_to(&rig.e, RAGDOLL_SET_00C75580), [vec![ragdoll, 0]]);
        // A process the test refuses, or no mover: nothing more.
        rig.e.register(PROCESS_TEST_0045CD60, |_, _| eax(1));
        rig.e.call_log = Some(vec![]);
        update(&mut rig, 1);
        assert!(calls_to(&rig.e, RAGDOLL_SET_00C75580).is_empty());
        rig.e.register(PROCESS_TEST_0045CD60, |_, _| eax(0));
        rig.e.register(PROCESS_ACTOR_MOVER, |_, _| eax(0));
        rig.e.call_log = Some(vec![]);
        update(&mut rig, 1);
        assert!(calls_to(&rig.e, RAGDOLL_SET_00C75580).is_empty());
    }

    #[test]
    fn registered_in_the_function_table() {
        let funcs = funcs();
        assert_eq!(funcs.len(), 120);
        let mut addresses: Vec<u32> = funcs.iter().map(|f| f.0).collect();
        addresses.sort_unstable();
        addresses.dedup();
        assert_eq!(addresses.len(), 120);
    }

    #[test]
    fn flee_and_extra_list_wrappers_answer_their_callees() {
        let mut e = engine();
        let actor = new_actor(&mut e);
        e.set(actor, Actor::pActorMover, Ptr::new(0x7700));
        double(&mut e, ACTOR_MOVER_SET_FLEE, eax(1));
        assert!(e.call(0x008bb630, &args![actor, 5u32]).bool());
        double(&mut e, EXTRA_LIST_OF_REFERENCE, eax(0x5000));
        double(&mut e, EXTRA_LIST_REMOVE_00422550, eax(1));
        assert!(e.call(0x008bc7d0, &args![actor, 0x30u32]).bool());
        double(&mut e, EXTRA_LIST_REMOVE_00422550, eax(0));
        assert!(!e.call(0x008bc7d0, &args![actor, 0x30u32]).bool());
    }
}

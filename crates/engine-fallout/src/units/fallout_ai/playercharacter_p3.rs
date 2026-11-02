//! `fallout/ai/playercharacter.cpp` (Xbox PDB source unit), part 3: its functions from `0095d090` up to
//! (not including) `00967aa0` in FalloutNV.exe 1.4.0.525, translated
//! (docs/ENGINE_CRATE.md). The unit's shared layouts and helpers are in
//! [`super::playercharacter`]; anything public there may be used here.
//!
//! The earlier sessions cover `0095d090` to `00964060`; the latest covers
//! `009640b0` to `00967a00`, the end of the range (no function of the range
//! is open after it).
//! The main file declares no
//! layout yet, so the `PlayerCharacter` fields used here are declared in
//! this file. The PC `PlayerCharacter` is the Xbox PDB's with every field
//! after `TESForm` 0x10 lower (`TESForm` is 0x18 on PC); each offset below
//! was checked against the PC code.
//!
//! Notes on the callees shared by many functions here:
//! - `006815c0`, `00726070`, `008256d0` are `BSSimpleList` node accessors the
//!   linker folded under unrelated names: the node itself (its data slot is
//!   the first word), the next node, and "is empty" (no data and no next).
//! - `0044ddc0` reads the dword at +8 of its argument (the form's `iFormID`
//!   for a `TESForm`, here used on process/camera objects too).

#[allow(unused_imports)]
use super::playercharacter::*;
#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::crt::glue::alloca16_esp;

/// `PlayerCharacter::pSingleton`: the global holding the player's pointer.
const PLAYER_CHARACTER: u32 = 0x011d_ea3c;

/// `BSSimpleList` node: the node itself, whose first word is the item.
const LIST_NODE_SLOT: u32 = 0x0068_15c0;
/// `BSSimpleList` node: the next node.
const LIST_NODE_NEXT: u32 = 0x0072_6070;
/// `BSSimpleList` node: true when the node holds no item and has no next node.
const LIST_NODE_IS_EMPTY: u32 = 0x0082_56d0;
/// Reads the dword at +8 of the object (see the module notes).
const READ_FIELD_8: u32 = 0x0044_ddc0;
/// `__RTDynamicCast` (cdecl): `(object, vfdelta, source type, target type, is reference)`.
const RT_DYNAMIC_CAST: u32 = 0x00ec_43fb;
/// The `TESForm` type descriptor.
const TYPE_TES_FORM: u32 = 0x0118_3108;
/// The `MagicItem` type descriptor (`MagicItem` cast target of the queued
/// enchantment and eat/drink code).
const TYPE_MAGIC_ITEM: u32 = 0x0118_39b4;
/// The type descriptor the equip code casts to (`0x1183998`).
const TYPE_EQUIPPABLE: u32 = 0x0118_3998;
/// `MemoryManager`-style scope guard constructor `(this, 0x34, 1, file, line)`
/// and destructor.
const SCOPE_GUARD_NEW: u32 = 0x0040_4eb0;
const SCOPE_GUARD_DELETE: u32 = 0x0040_4ee0;
/// Source file name the scope guards of this file are given.
const SCOPE_GUARD_FILE: u32 = 0x0108_a8e0;
/// `strcpy_s`-like `(destination, size, source)`, cdecl.
const STRING_COPY: u32 = 0x0040_6d30;
/// `strrchr(string, character)`, cdecl.
const STRING_FIND_LAST: u32 = 0x0040_ab30;
/// Value pointer of a setting object (`Setting` at `ECX`): a pointer to its
/// `char*`/`float` value.
const SETTING_STRING: u32 = 0x0040_3df0;
const SETTING_FLOAT: u32 = 0x0040_3e20;
/// Deleting destructor `(object, 1)` of a list/object (`operator delete` included).
const SCALAR_DELETE: u32 = 0x0044_59e0;
const LIST_DELETE: u32 = 0x0047_02f0;
/// `BSSimpleList::Clear`-style `(list)`: frees the items of the list.
const LIST_CLEAR_ITEMS: u32 = 0x0047_0470;
/// `ItemChange::ItemChange` `(this, object, count)` (Xbox PDB).
const ITEM_CHANGE_NEW: u32 = 0x004b_c550;
/// `operator new` `(size)`, cdecl.
const OPERATOR_NEW: u32 = 0x0040_1000;
/// `ItemChange::GetHotKey` (Xbox PDB): the hot key index, -1 if none.
const ITEM_CHANGE_GET_HOT_KEY: u32 = 0x004b_d7a0;
/// `ItemChange::HasModEffectActive_ov2` (Xbox PDB) `(this, mod index)`.
const ITEM_CHANGE_HAS_MOD: u32 = 0x004b_da70;
/// `InventoryChanges::GetInventoryChanges` (Xbox PDB), cdecl `(actor)`.
const GET_INVENTORY_CHANGES: u32 = 0x004b_f220;
/// `InventoryChanges::GetObjectCount` (Xbox PDB) `(changes, object)`.
const INVENTORY_GET_OBJECT_COUNT: u32 = 0x004c_8f30;
/// `InventoryChanges::GetInventoryItem` (Xbox PDB) `(changes, object, flag)`.
const INVENTORY_GET_ITEM: u32 = 0x004d_0650;
/// Looks up the object bound to a hot key: `(table, hot key)`.
const HOT_KEY_TO_OBJECT: u32 = 0x004b_fb30;
/// `TESObjectWEAP::GetCurrentAmmo` (Xbox PDB) `(weapon, player)`.
const WEAPON_GET_CURRENT_AMMO: u32 = 0x0052_5980;
/// `TESObjectWEAP::GetFormClipRounds` (Xbox PDB) `(weapon, flag)`.
const WEAPON_GET_CLIP_ROUNDS: u32 = 0x004f_e160;
/// `(weapon, flag)`: the animation group index of the weapon's attack.
const WEAPON_GET_ATTACK_ANIM_BASE: u32 = 0x0051_e2a0;
/// The player's table of hot keys (`ECX` of [`HOT_KEY_TO_OBJECT`]).
const HOT_KEY_TABLE: u32 = 0x011d_ea3c;
/// `min`-like helper `(a, b)`, cdecl (returns the smaller).
const MIN_COUNT: u32 = 0x004a_8f20;
/// `Actor::QueueEquipObject` (Xbox PDB) `(actor, object, count, 0, 1, 0, 0)`.
const ACTOR_QUEUE_EQUIP_OBJECT: u32 = 0x0088_c650;
/// `Actor::GetAnimGroup` (Xbox PDB) `(actor, index, 0, 0, animation)`.
const ACTOR_GET_ANIM_GROUP: u32 = 0x0089_7910;
/// `TESAnimGroup::GetType` (Xbox PDB), cdecl `(group)`.
const ANIM_GROUP_GET_TYPE: u32 = 0x005f_2440;
/// `Actor::SetAnimAction` (Xbox PDB) `(actor, action, value)`.
const ACTOR_SET_ANIM_ACTION: u32 = 0x008a_73e0;
/// `Actor::PlayAnimGroup`-style `(actor, group)` used after an equip.
const ACTOR_RELEASE_ANIM_GROUP: u32 = 0x008a_8820;
/// `PlayerCharacter::GetAnimation` (Xbox PDB) `(player, first person)`.
const PLAYER_GET_ANIMATION: u32 = 0x0095_0a60;
/// `Animation::PlayGroup` (Xbox PDB) `(animation, group, 1, -1, -1)`.
const ANIMATION_PLAY_GROUP: u32 = 0x0049_4740;
/// `Animation::BlendOut` (Xbox PDB) `(animation, group, flag)`.
const ANIMATION_BLEND_OUT: u32 = 0x0049_94f0;
/// `(animation, 4)`: the animation's current action value.
const ANIMATION_GET_ACTION: u32 = 0x0049_1040;
/// `CombatProcedureAttackMelee::Initialize` (Xbox PDB) name on the folded
/// body `(item change, 0)`: installs the item change as the process's
/// equipped item.
const PROCESS_SET_EQUIPPED_FOLLOWUP: u32 = 0x006e_cd40;
/// `MagicTarget` helpers (`this` is the `MagicTarget` subobject at +0x94).
const MAGIC_TARGET_ADD_EFFECT_ITEM: u32 = 0x0082_4110;
const MAGIC_TARGET_REFRESH: u32 = 0x0082_55a0;
/// `MagicTarget::UpdateTarget` (Xbox PDB) `(target, float)`.
const MAGIC_TARGET_UPDATE_TARGET: u32 = 0x0082_3c40;
/// `Actor::CastAlchemy` (Xbox PDB) `(actor, item, flag)`.
const ACTOR_CAST_ALCHEMY: u32 = 0x008c_2090;
/// `VATS::GetCount` (Xbox PDB) `(item)` on the item's +0x10 member.
const ITEM_GET_STACK_LIMIT: u32 = 0x005a_e380;
/// Appends an item to a list `(list, &item)`.
const LIST_APPEND: u32 = 0x005a_e3d0;
/// `(list, &item)`: true when the list contains the item.
const LIST_CONTAINS: u32 = 0x005f_65d0;
/// The float the eat/drink code passes to `UpdateTarget`.
const UPDATE_TARGET_VALUE: u32 = 0x0101_3ea4;

layout! {
    /// `PlayerCharacter` (Xbox PDB), 0xE50 bytes on PC. Only the fields the
    /// functions of this part use are declared; offsets are the PDB's minus
    /// 0x10 (see the module notes).
    pub struct PlayerCharacter: 0xe50 {
        /// `pCurrentProcess` (Xbox PDB, `MobileObject`): `BaseProcess*`.
        0x68 pCurrentProcess: Ptr,
        /// `pActorMover` (Xbox PDB, `Actor`): `ActorMover*`.
        0x190 pActorMover: Ptr,
        /// `pCameraCaster` (Xbox PDB): `CameraCaster*`.
        0x21c pCameraCaster: Ptr,
        /// `EatDrinkItems` (Xbox PDB): `BSSimpleList<MagicItem*>*`.
        0x238 EatDrinkItems: Ptr,
        /// `QueuedWornEnchantments` (Xbox PDB): `BSSimpleList<TESBoundObject*>*`.
        0x23c QueuedWornEnchantments: Ptr,
        /// First float of `TemporaryActorValueModifiers` (Xbox PDB): `f32[0x4d]`.
        0x244 TemporaryActorValueModifiers: f32,
        /// `spGrabSpring` (Xbox PDB): `NiPointer<bhkMouseSpringAction>`.
        0x634 spGrabSpring: Ptr,
        /// `pGrabbedObject` (Xbox PDB): `TESObjectREFR*`.
        0x638 pGrabbedObject: Ptr,
        /// `eGrabType` (Xbox PDB).
        0x63c eGrabType: u32,
        /// `fGrabObjectWeight` (Xbox PDB).
        0x640 fGrabObjectWeight: f32,
        /// `fGrabDistance` (Xbox PDB).
        0x644 fGrabDistance: f32,
        /// `b3rdPerson` (Xbox PDB).
        0x64a b3rdPerson: u8,
        /// `bWant3rdPerson` (Xbox PDB).
        0x64c bWant3rdPerson: u8,
        /// `fFOV` (Xbox PDB).
        0x65c fFOV: f32,
        /// `fWorldFOV` (Xbox PDB).
        0x670 fWorldFOV: f32,
        /// `f1stPersonFOV` (Xbox PDB).
        0x674 f1stPersonFOV: f32,
        /// `ucControlsDisabled` (Xbox PDB).
        0x680 ucControlsDisabled: u8,
        /// `sp1stPerson3D` (Xbox PDB): `NiPointer<NiAVObject>`.
        0x694 sp1stPerson3D: Ptr,
        /// `pListofActions` (Xbox PDB): `BSSimpleList<PlayerActionObject *> *`.
        0x60c pListofActions: Ptr,
        /// `p1stPersonAnimation` (Xbox PDB): `Animation*`.
        0x690 p1stPersonAnimation: Ptr,
        /// `fEyeHeight` (Xbox PDB).
        0x698 fEyeHeight: f32,
        /// `fSitHeadingDelta` (Xbox PDB).
        0x6e4 fSitHeadingDelta: f32,
        /// `pPendingPoison` (Xbox PDB): `AlchemyItem*`.
        0x758 pPendingPoison: u32,
        /// `bBeingChased` (Xbox PDB).
        0x7c4 bBeingChased: u8,
        /// `fUFOCameraHeading` (Xbox PDB).
        0x7e0 fUFOCameraHeading: f32,
        /// `fUFOCameraPitch` (Xbox PDB).
        0x7e4 fUFOCameraPitch: f32,
        /// `iSelectedSpellCastSoundID` (Xbox PDB).
        0x7f4 iSelectedSpellCastSoundID: u32,
        /// `bInsufficientChargeMessageShown` (Xbox PDB).
        0x86c bInsufficientChargeMessageShown: u8,
        /// `fDropAngleMod` (Xbox PDB).
        0x870 fDropAngleMod: f32,
        /// `fLastDropAngleMod` (Xbox PDB).
        0x874 fLastDropAngleMod: f32,
        /// `pAutoAimActor` (Xbox PDB): `Actor*`.
        0xd2c pAutoAimActor: Ptr,
        /// `pClosestConversation` (Xbox PDB): `DialoguePackage*`.
        0x208 pClosestConversation: Ptr,
        /// `iNumberofStealWarnings` (Xbox PDB).
        0x228 iNumberofStealWarnings: u32,
        /// `fStealWarningTimer` (Xbox PDB).
        0x22c fStealWarningTimer: f32,
        /// `iNumberofPickpocketWarnings` (Xbox PDB).
        0x230 iNumberofPickpocketWarnings: u32,
        /// `fPickPocketWarningTimer` (Xbox PDB).
        0x234 fPickPocketWarningTimer: f32,
        /// `pIronsightsDOFInstance` (Xbox PDB): `ImageSpaceModifierInstanceDOF*`.
        0x5ec pIronsightsDOFInstance: Ptr,
        /// `pVatsDOFInstance` (Xbox PDB): `ImageSpaceModifierInstanceDOF*`.
        0x5f0 pVatsDOFInstance: Ptr,
        /// `pVatsDRBInstance` (Xbox PDB): `ImageSpaceModifierInstanceDRB*`.
        0x5f4 pVatsDRBInstance: Ptr,
        /// `BulletAutoAim` (Xbox PDB), an `NiPoint3`: x.
        0xd30 BulletAutoAimX: f32,
        /// `BulletAutoAim`: y.
        0xd34 BulletAutoAimY: f32,
        /// `BulletAutoAim`: z.
        0xd38 BulletAutoAimZ: f32,
        /// `bTarget3DDistant` (Xbox PDB).
        0xd40 bTarget3DDistant: u8,
        /// `pPlayersTargetActor` (Xbox PDB): `Actor*`.
        0xd44 pPlayersTargetActor: Ptr,
        /// `pListofPercievedActors` (Xbox PDB): `BSSimpleList<ActorsNearPlayerData *>*`.
        0xd48 pListofPercievedActors: Ptr,
        /// `fMenuModeButtonTimer` (Xbox PDB).
        0xd4c fMenuModeButtonTimer: f32,
        /// `bMenuModeButtonClicked` (Xbox PDB).
        0xd54 bMenuModeButtonClicked: u8,
    }
}

/// Number of `TemporaryActorValueModifiers` entries.
const ACTOR_VALUE_MODIFIER_COUNT: u32 = 0x4d;
/// Offset of the `MagicCaster` base (`GetMagicCaster`'s subobject).
const MAGIC_CASTER_OFFSET: u32 = 0x88;
/// Offset of the `MagicTarget` base.
const MAGIC_TARGET_OFFSET: u32 = 0x94;

/// The process object of the player (`this->pCurrentProcess`).
fn process(e: &mut Engine, this: Ptr<PlayerCharacter>) -> u32 {
    e.get(this, PlayerCharacter::pCurrentProcess).addr()
}

/// Virtual call `slot` of the player's process.
fn process_slot(e: &mut Engine, this: Ptr<PlayerCharacter>, slot: u32, args: &[u32]) -> u32 {
    let process = process(e, this);
    e.vcall(process, slot, args).u32()
}

/// `__RTDynamicCast(object, 0, TESForm type, target, 0)`.
fn dynamic_cast_form(e: &mut Engine, object: u32, target: u32) -> u32 {
    e.call(
        RT_DYNAMIC_CAST,
        &args![object, 0u32, TYPE_TES_FORM, target, 0u32],
    )
    .u32()
}

// Translated from 0095d090 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PlayerCharacter::CastEatDrinkItems` (Xbox PDB): clears the eat/drink
/// list (the body is `BSSimpleList` item freeing on `EatDrinkItems`).
pub fn player_character_cast_eat_drink_items(e: &mut Engine, this: Ptr<PlayerCharacter>) {
    let list = e.get(this, PlayerCharacter::EatDrinkItems);
    e.call(LIST_CLEAR_ITEMS, &args![list]);
}

// Translated from 0095d0b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PlayerCharacter::AddEatDrinkItem` (Xbox PDB): casts the item at once, or
/// for a form whose virtual at +0x18 answers 7 first queues it on the
/// eat/drink list when fewer than 100 such items are queued. Then it tells
/// the `MagicTarget` base to take the item and update. Always true.
pub fn player_character_add_eat_drink_item(
    e: &mut Engine,
    this: Ptr<PlayerCharacter>,
    item: Ptr,
) -> bool {
    let kind = e.vcall(item.addr(), 0x18, &args![]).u32();
    if kind == 7 {
        let limit = 100;
        if fn_0095d160(e, this) < limit {
            let list = e.get(this, PlayerCharacter::EatDrinkItems);
            e.with_stack(4, |e, slot| {
                e.mem.set_u32(slot.addr(), item.addr());
                e.call(LIST_APPEND, &args![list, slot]);
            });
            e.call(ACTOR_CAST_ALCHEMY, &args![this, item, 1u32]);
        }
    } else {
        e.call(ACTOR_CAST_ALCHEMY, &args![this, item, 1u32]);
    }
    let target = this.byte_add(MAGIC_TARGET_OFFSET);
    e.call(MAGIC_TARGET_ADD_EFFECT_ITEM, &args![target, item]);
    e.call(MAGIC_TARGET_REFRESH, &args![target]);
    let value: f32 = e.global(UPDATE_TARGET_VALUE);
    e.call(MAGIC_TARGET_UPDATE_TARGET, &args![target, value]);
    true
}

/// `0095d160`'s fixed frame under `EBP` (`SUB ESP,0x44`), the entries of each of
/// its two `_alloca16` tables (the local at `EBP-8`), and the whole block: the
/// fixed locals, both tables and up to 0xf bytes of alignment pad under each.
const EAT_DRINK_FIXED_LOCALS: u32 = 0x44;
const EAT_DRINK_TABLE_SIZE: u32 = 100;
const EAT_DRINK_FRAME_SIZE: u32 = EAT_DRINK_FIXED_LOCALS + 2 * (EAT_DRINK_TABLE_SIZE * 4) + 0x20;

// Translated from 0095d160 (decompiled, FalloutNV.exe 1.4.0.525)
/// Counts the eat/drink items already queued. A first pass counts every
/// listed item whose virtual at +0x18 answers 7; a second pass walks the
/// `MagicTarget` base's list (virtual at +8), keeping up to 100 distinct such
/// items in a table with a per-item count (limited by the item's own stack
/// limit), and the result is the total of the first pass plus the units the
/// second pass added.
///
/// The frame is the exe's: `EBP` is the top of one `with_stack` block of
/// [`EAT_DRINK_FRAME_SIZE`] bytes, the fixed locals sit under it (`SUB ESP,0x44`;
/// the form whose address goes to the list's contains test at `EBP-0x20`) and the
/// two tables come from `_alloca16` (`00ec5ec0`, twice, with the local count
/// `[EBP-8]` = 100 times 4): each is placed where the probe puts it,
/// [`alloca16_esp`] of the `ESP` before the call (first `EBP-0x44`, then the
/// first table), so their addresses and 16-byte alignment are the exe's. The
/// block has room for both tables and both alignment pads; the epilogue's
/// `LEA ESP,[EBP-0x44]` dropping them is the block's end. The `/GS` cookie at
/// `EBP-0x2c` is not written.
pub fn fn_0095d160(e: &mut Engine, this: Ptr<PlayerCharacter>) -> i32 {
    e.with_stack(EAT_DRINK_FRAME_SIZE, |e, block| {
        let ebp = block.addr() + EAT_DRINK_FRAME_SIZE;
        let mut total: i32 = 0;
        let mut node = e.get(this, PlayerCharacter::EatDrinkItems).addr();
        while node != 0 {
            if e.call(LIST_NODE_IS_EMPTY, &args![node]).bool() {
                break;
            }
            let slot = e.call(LIST_NODE_SLOT, &args![node]).u32();
            if e.mem.u32(slot) != 0 {
                let slot = e.call(LIST_NODE_SLOT, &args![node]).u32();
                let item = e.mem.u32(slot);
                if e.vcall(item, 0x18, &args![]).u32() == 7 {
                    total += 1;
                }
            }
            node = e.call(LIST_NODE_NEXT, &args![node]).u32();
        }

        // Two `_alloca16` tables of 100 dwords: the distinct items and their counts.
        let items = alloca16_esp(ebp - EAT_DRINK_FIXED_LOCALS, EAT_DRINK_TABLE_SIZE * 4);
        let counts = alloca16_esp(items, EAT_DRINK_TABLE_SIZE * 4);
        for i in 0..EAT_DRINK_TABLE_SIZE {
            e.mem.set_u32(items + i * 4, 0);
            e.mem.set_u32(counts + i * 4, 0);
        }
        let target = this.addr() + MAGIC_TARGET_OFFSET;
        let mut node = e.vcall(target, 0x8, &args![]).u32();
        while node != 0 {
            if e.call(LIST_NODE_IS_EMPTY, &args![node]).bool() {
                break;
            }
            let slot = e.call(LIST_NODE_SLOT, &args![node]).u32();
            let form = if e.mem.u32(slot) == 0 {
                0
            } else {
                let slot = e.call(LIST_NODE_SLOT, &args![node]).u32();
                let object = e.mem.u32(slot);
                e.call(READ_FIELD_8, &args![object]).u32()
            };
            if form != 0 && e.vcall(form, 0x18, &args![]).u32() == 7 {
                let queued = e.get(this, PlayerCharacter::EatDrinkItems).addr() != 0 && {
                    let list = e.get(this, PlayerCharacter::EatDrinkItems);
                    let cell = ebp - 0x20;
                    e.mem.set_u32(cell, form);
                    e.call(LIST_CONTAINS, &args![list, cell]).bool()
                };
                if !queued {
                    let mut placed = false;
                    let mut index = 0;
                    while index < EAT_DRINK_TABLE_SIZE && !placed {
                        let entry = e.mem.u32(items + index * 4);
                        if entry == 0 {
                            e.mem.set_u32(items + index * 4, form);
                            e.mem.set_u32(counts + index * 4, 1);
                            total += 1;
                            placed = true;
                        } else if entry == form {
                            let limit = e.call(ITEM_GET_STACK_LIMIT, &args![form + 0x10]).u32();
                            let count = e.mem.u32(counts + index * 4);
                            if count < limit {
                                e.mem.set_u32(counts + index * 4, count + 1);
                                placed = true;
                            }
                        }
                        index += 1;
                    }
                }
            }
            node = e.call(LIST_NODE_NEXT, &args![node]).u32();
        }
        total
    })
}

// Translated from 0095d3f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Equips (or uses) `item` through [`fn_0095d450`] when the item answers the
/// check at `004938c0`, else through the player's virtual at +0x3e8 (which
/// takes only the first three arguments).
pub fn fn_0095d3f0(
    e: &mut Engine,
    this: Ptr<PlayerCharacter>,
    item: Ptr,
    mode: u32,
    flag: u8,
    stack_all: u8,
) -> bool {
    if item.addr() != 0 && e.call(GET_BYTE_AT_384, &args![item]).bool() {
        return fn_0095d450(e, this, item, mode, flag, stack_all);
    }
    e.vcall(this.addr(), 0x3e8, &args![item, mode, flag]).bool()
}

/// The part of the equip functions that looks the item's hot key up: if
/// `item` is not the current ammo and has a hot key bound to another object,
/// that object replaces `current`. Returns the (possibly replaced) object.
fn equip_hot_key_replacement(e: &mut Engine, inventory: u32, item: Ptr, current: u32) -> u32 {
    let mut current = current;
    let item_change = e
        .call(INVENTORY_GET_ITEM, &args![inventory, item, 0u32])
        .u32();
    let hot_key = e.call(ITEM_CHANGE_GET_HOT_KEY, &args![item_change]).i32();
    if hot_key >= 0 {
        let hot_key = e.call(ITEM_CHANGE_GET_HOT_KEY, &args![item_change]).u32();
        let table = e.global::<u32>(HOT_KEY_TABLE);
        let replacement = e.call(HOT_KEY_TO_OBJECT, &args![table, hot_key]).u32();
        if replacement != 0 {
            current = replacement;
        }
    }
    if item_change != 0 {
        e.call(SCALAR_DELETE, &args![item_change, 1u32]);
    }
    current
}

/// Builds `ItemChange(object, count)` in a fresh 0xc-byte block, null when
/// the allocation fails.
fn new_item_change(e: &mut Engine, object: u32, count: u32) -> u32 {
    let block = e.call(OPERATOR_NEW, &args![0xcu32]).u32();
    if block == 0 {
        0
    } else {
        e.call(ITEM_CHANGE_NEW, &args![block, object, count]).u32()
    }
}

// Translated from 0095d450 (decompiled, FalloutNV.exe 1.4.0.525)
/// Equips `item` for the player through the hot key logic. `mode` 0 queues
/// the equip, 1 plays the animation group `item`'s attack base minus 0x17
/// when the actor has it, 2 starts that group and sets animation action
/// 0x11. Returns false when nothing is done (unknown mode, or the same
/// item and count are already equipped). The exception frame is not
/// translated.
pub fn fn_0095d450(
    e: &mut Engine,
    this: Ptr<PlayerCharacter>,
    item: Ptr,
    mode: u32,
    flag: u8,
    use_count: u8,
) -> bool {
    let player = e.global::<u32>(PLAYER_CHARACTER);
    let mut current = e.call(WEAPON_GET_CURRENT_AMMO, &args![item, player]).u32();
    let inventory = e.call(GET_INVENTORY_CHANGES, &args![this]).u32();
    let _held = e
        .call(INVENTORY_GET_OBJECT_COUNT, &args![inventory, current])
        .u32();
    let equipped_before = process_slot(e, this, 0x148, &args![]);
    let mut equipped_change = process_slot(e, this, 0x14c, &args![]);
    e.call(PLAYER_GET_ANIMATION, &args![this, 0u32]);

    if item.addr() != 0 && current != 0 && item.addr() != current {
        current = equip_hot_key_replacement(e, inventory, item, current);
    }
    let mut count = e
        .call(INVENTORY_GET_OBJECT_COUNT, &args![inventory, current])
        .u32();

    if equipped_change == 0 || e.call(READ_FIELD_8, &args![equipped_change]).u32() != current {
        let change = new_item_change(e, current, count);
        process_slot(e, this, 0x168, &args![change]);
        let installed = process_slot(e, this, 0x14c, &args![]);
        e.call(PROCESS_SET_EQUIPPED_FOLLOWUP, &args![installed, 0u32]);
        equipped_change = change;
    }

    if equipped_before != 0 && equipped_change != 0 {
        let have = e.call(READ_FIELD_4, &args![equipped_change]).u32();
        let has_mod = e
            .call(ITEM_CHANGE_HAS_MOD, &args![equipped_before, 2u32])
            .u8();
        let clip = e
            .call(WEAPON_GET_CLIP_ROUNDS, &args![item, u32::from(has_mod)])
            .u32();
        if have == clip {
            return false;
        }
        if count == e.call(READ_FIELD_4, &args![equipped_change]).u32() {
            return false;
        }
    }

    if item.addr() != 0 && current != item.addr() {
        let clip = e
            .call(WEAPON_GET_CLIP_ROUNDS, &args![item, u32::from(flag)])
            .u32();
        count = e.call(MIN_COUNT, &args![count, clip]).u32();
    }

    match mode {
        0 => {
            let mut equip_count = 1u32;
            let process_change = process_slot(e, this, 0x14c, &args![]);
            if process_change != 0 {
                let again = process_slot(e, this, 0x14c, &args![]);
                equip_count = e.call(READ_FIELD_4, &args![again]).u32().wrapping_add(1);
            }
            if use_count != 0 {
                equip_count = count;
            }
            e.call(
                ACTOR_QUEUE_EQUIP_OBJECT,
                &args![this, current, equip_count, 0u32, 1u32, 0u32, 0u32],
            );
            true
        }
        1 => {
            let animation = e.call(PLAYER_GET_ANIMATION, &args![this, 0u32]).u32();
            let base = e
                .call(WEAPON_GET_ATTACK_ANIM_BASE, &args![item, u32::from(flag)])
                .u32();
            let index = base.wrapping_sub(0x17);
            let group = e
                .call(
                    ACTOR_GET_ANIM_GROUP,
                    &args![this, index, 0u32, 0u32, animation],
                )
                .u16();
            if e.call(ANIM_GROUP_GET_TYPE, &args![u32::from(group)]).u32() == index {
                e.call(
                    ACTOR_RELEASE_ANIM_GROUP,
                    &args![animation, u32::from(group)],
                );
            }
            true
        }
        2 => {
            let base = e
                .call(WEAPON_GET_ATTACK_ANIM_BASE, &args![item, u32::from(flag)])
                .u32();
            let index = base.wrapping_sub(0x17);
            let group = e
                .call(ACTOR_GET_ANIM_GROUP, &args![this, index, 0u32, 0u32, 0u32])
                .u16();
            if e.call(ANIM_GROUP_GET_TYPE, &args![u32::from(group)]).u32() == index {
                let animation = e.call(PLAYER_GET_ANIMATION, &args![this, 0u32]).u32();
                e.call(
                    ANIMATION_PLAY_GROUP,
                    &args![animation, u32::from(group), 1u32, -1i32, -1i32],
                );
                e.vcall(this.addr(), 0x4b0, &args![u32::from(group), 1u32]);
                let action = e.call(ANIMATION_GET_ACTION, &args![animation, 4u32]).u32();
                e.call(ACTOR_SET_ANIM_ACTION, &args![this, 0x11u32, action]);
            }
            true
        }
        _ => false,
    }
}

/// Makes a new `ItemChange(object, count)` the process's equipped item (the
/// process's virtual at +0x168), then lets the process's current item
/// change (virtual at +0x14c) follow up.
fn install_item_change(e: &mut Engine, this: Ptr<PlayerCharacter>, object: u32, count: u32) -> u32 {
    let change = new_item_change(e, object, count);
    process_slot(e, this, 0x168, &args![change]);
    let installed = process_slot(e, this, 0x14c, &args![]);
    e.call(PROCESS_SET_EQUIPPED_FOLLOWUP, &args![installed, 0u32]);
    change
}

// Translated from 0095d850 (decompiled, FalloutNV.exe 1.4.0.525)
/// Equips `item` like [`fn_0095d450`] but also accepts an item that has no
/// current-ammo object when its kind (`00446390`) is 0xa to 0xd, then makes
/// the stack the player carries current and plays or queues the equip.
/// `mode` 2 first starts the attack animation group when the equipped count
/// differs, `mode` 1 afterwards plays or blends the group. Returns false
/// when there is nothing to equip, or the same item and count are already
/// equipped. The exception frame is not translated.
pub fn fn_0095d850(
    e: &mut Engine,
    this: Ptr<PlayerCharacter>,
    item: Ptr,
    mode: u32,
    flag: u8,
) -> bool {
    let player = e.global::<u32>(PLAYER_CHARACTER);
    let mut current = e.call(WEAPON_GET_CURRENT_AMMO, &args![item, player]).u32();
    let equipped_before = process_slot(e, this, 0x148, &args![]);
    if current == 0 {
        let is_kind =
            |e: &mut Engine, kind: u32| e.call(GET_BYTE_AT_F4, &args![item]).u32() == kind;
        if is_kind(e, 0xa) || is_kind(e, 0xb) || is_kind(e, 0xc) || is_kind(e, 0xd) {
            current = item.addr();
        }
    }
    if current == 0 {
        return false;
    }
    let inventory = e.call(GET_INVENTORY_CHANGES, &args![this]).u32();
    if item.addr() != 0 && current != 0 && current != item.addr() {
        current = equip_hot_key_replacement(e, inventory, item, current);
    }
    let mut count = e
        .call(INVENTORY_GET_OBJECT_COUNT, &args![inventory, current])
        .u32();

    let equipped_change = process_slot(e, this, 0x14c, &args![]);
    if equipped_change == 0 || e.call(READ_FIELD_8, &args![equipped_change]).u32() != current {
        install_item_change(e, this, current, count);
    }
    if item.addr() != 0 && current != item.addr() {
        let clip = e
            .call(WEAPON_GET_CLIP_ROUNDS, &args![item, u32::from(flag)])
            .u32();
        count = e.call(MIN_COUNT, &args![count, clip]).u32();
    }

    if mode == 2 && process_slot(e, this, 0x14c, &args![]) != 0 {
        let has_mod = e
            .call(ITEM_CHANGE_HAS_MOD, &args![equipped_before, 2u32])
            .u8();
        let clip = e
            .call(WEAPON_GET_CLIP_ROUNDS, &args![item, u32::from(has_mod)])
            .u32();
        let change = process_slot(e, this, 0x14c, &args![]);
        let equipped_count = e.call(READ_FIELD_4, &args![change]).u32();
        if clip != equipped_count {
            let change = process_slot(e, this, 0x14c, &args![]);
            if count != e.call(READ_FIELD_4, &args![change]).u32() {
                let index = e
                    .call(WEAPON_GET_ATTACK_ANIM_BASE, &args![item, u32::from(flag)])
                    .u32();
                let group = e
                    .call(ACTOR_GET_ANIM_GROUP, &args![this, index, 0u32, 0u32, 0u32])
                    .u16();
                if e.call(ANIM_GROUP_GET_TYPE, &args![u32::from(group)]).u32() == index {
                    let animation = e.call(PLAYER_GET_ANIMATION, &args![this, 0u32]).u32();
                    e.call(
                        ANIMATION_PLAY_GROUP,
                        &args![animation, u32::from(group), 1u32, -1i32, -1i32],
                    );
                    e.vcall(this.addr(), 0x4b0, &args![u32::from(group), 1u32]);
                    let action = e.call(ANIMATION_GET_ACTION, &args![animation, 4u32]).u32();
                    e.call(ACTOR_SET_ANIM_ACTION, &args![this, 9u32, action]);
                }
            }
        }
    }

    if count == 0 {
        return false;
    }
    let change = process_slot(e, this, 0x14c, &args![]);
    if e.call(READ_FIELD_4, &args![change]).u32() != 0 {
        if equipped_before == 0 {
            return false;
        }
        let has_mod = e
            .call(ITEM_CHANGE_HAS_MOD, &args![equipped_before, 2u32])
            .u8();
        let clip = e
            .call(WEAPON_GET_CLIP_ROUNDS, &args![item, u32::from(has_mod)])
            .u32();
        let change = process_slot(e, this, 0x14c, &args![]);
        if clip == e.call(READ_FIELD_4, &args![change]).u32() {
            return false;
        }
        let change = process_slot(e, this, 0x14c, &args![]);
        if count == e.call(READ_FIELD_4, &args![change]).u32() {
            return false;
        }
    }

    let queue_equip = |e: &mut Engine| {
        e.call(
            ACTOR_QUEUE_EQUIP_OBJECT,
            &args![this, current, count, 0u32, 1u32, 0u32, 0u32],
        );
    };
    if current == item.addr() {
        e.call(SET_BYTE_AT_4, &args![current, 0x29u32]);
        let process = process(e, this);
        let queue = process == 0
            || e.call(GET_DWORD_AT_28, &args![process]).i32() > 1
            || e.call(INTERFACE_IS_IN_MENU_MODE, &args![]).bool()
            || {
                let object = e.global::<u32>(POINTER_011DDF38);
                e.call(TEST_BIT_2_AT_244, &args![object]).bool()
            };
        if queue {
            queue_equip(e);
        }
        e.call(SET_BYTE_AT_4, &args![current, 0x28u32]);
    } else {
        queue_equip(e);
    }

    if mode == 1 {
        let animation = e.call(PLAYER_GET_ANIMATION, &args![this, 0u32]).u32();
        let index = e
            .call(WEAPON_GET_ATTACK_ANIM_BASE, &args![item, u32::from(flag)])
            .u32();
        let group = e
            .call(
                ACTOR_GET_ANIM_GROUP,
                &args![this, index, 0u32, 0u32, animation],
            )
            .u16();
        if e.call(ANIM_GROUP_GET_TYPE, &args![u32::from(group)]).u32() == index {
            if e.call(TEST_BIT_2_BYTE_AT_100, &args![item]).bool() {
                e.call(ANIMATION_BLEND_OUT, &args![animation, 5u32, 0u32]);
                e.call(ANIMATION_BLEND_OUT, &args![animation, 6u32, 0u32]);
                e.call(
                    ANIMATION_PLAY_GROUP,
                    &args![animation, u32::from(group), 1u32, -1i32, -1i32],
                );
                e.vcall(this.addr(), 0x4b0, &args![u32::from(group), 1u32]);
                let action = e.call(ANIMATION_GET_ACTION, &args![animation, 4u32]).u32();
                e.call(ACTOR_SET_ANIM_ACTION, &args![this, 9u32, action]);
            } else {
                e.call(
                    ACTOR_RELEASE_ANIM_GROUP,
                    &args![animation, u32::from(group)],
                );
            }
        }
    }
    if !e.call(TEST_DWORD_AT_8, &args![OBJECT_011F2250]).bool() {
        e.vcall(this.addr(), 0x340, &args![9u32]);
    }
    true
}

// Translated from 0095ddc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Returns 0 without a process or without the process's item change
/// (virtual at +0x14c); 100 when god mode is on and demigod mode is off;
/// else whatever `008a89a0` answers for `argument`.
pub fn fn_0095ddc0(e: &mut Engine, this: Ptr<PlayerCharacter>, argument: u32) -> i32 {
    if process(e, this) == 0 || process_slot(e, this, 0x14c, &args![]) == 0 {
        return 0;
    }
    if e.call(PLAYER_IS_GOD_MODE, &args![]).bool()
        && !e.call(PLAYER_IS_DEMIGOD_MODE, &args![]).bool()
    {
        return 100;
    }
    e.call(ACTOR_FUNCTION_008A89A0, &args![this, argument])
        .i32()
}

/// Value of a float setting object: the game reads the `float` the setting's
/// value pointer (`00403e20`) returns.
fn setting_float(e: &mut Engine, setting: u32) -> f32 {
    let value = e.call(SETTING_FLOAT, &args![setting]).u32();
    e.mem.f32(value)
}

/// Moves `value` toward `target` by `step` (the compare/step/clamp of the
/// field-of-view update), without overshooting. Returns the new value.
fn approach(value: f32, target: f32, step: f32) -> f32 {
    let mut value = value;
    if target < value {
        value = (value as f64 - step as f64) as f32;
        if target > value {
            value = target;
        }
    } else if target > value {
        value = (value as f64 + step as f64) as f32;
        if target < value {
            value = target;
        }
    }
    value
}

/// Like [`approach`], also answering whether the target was reached (equal,
/// unordered, or the step overshot and the value was clamped to it).
fn approach_reporting(value: f32, target: f32, step: f32) -> (f32, bool) {
    let mut value = value;
    if target < value {
        value = (value as f64 - step as f64) as f32;
        if target > value {
            return (target, true);
        }
        (value, false)
    } else if target > value {
        value = (value as f64 + step as f64) as f32;
        if target < value {
            return (target, true);
        }
        (value, false)
    } else {
        (value, true)
    }
}

// Translated from 0095de30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Per-frame update of the player's two fields of view (`fWorldFOV` at
/// +0x670, `f1stPersonFOV` at +0x674). Does nothing unless the setting at
/// `011e097c` is on. With the Pipboy active it resets both to their
/// defaults. In the normal case it picks target values (the defaults, or
/// the zoomed ones when an iron sight is raised and the camera state asks
/// for it) and moves the fields toward them by `delta * 30 / (setting)`
/// per call; in the camera states 2 and 3 with a camera object present it
/// drives two `PlayerCharacter` spring objects (created on first use, 0x1a0
/// bytes each) instead. `011a3b31` is set while the fields are still moving
/// or settled and cleared while a spring runs. The exception frame is not
/// translated.
pub fn fn_0095de30(e: &mut Engine, this: Ptr<PlayerCharacter>, delta: f32) {
    let enabled = e.call(SETTING_BOOL_POINTER, &args![SETTING_011E097C]).u32();
    if e.mem.u8(enabled) == 0 {
        return;
    }
    let held = process_slot(e, this, 0x148, &args![]);
    let held_object = if held != 0 {
        e.call(READ_FIELD_8, &args![held]).u32()
    } else {
        0
    };
    let default_world = setting_float(e, SETTING_0120315C);
    let default_first_person = setting_float(e, SETTING_01203168);
    let default_second = setting_float(e, SETTING_01203174);
    let mut zoom = setting_float(e, SETTING_011E0970);
    let divisor = setting_float(e, SETTING_011CE870);
    // A local the game initializes to 1.0 is never read.
    let camera = e.call(GET_CAMERA_HOLDER, &args![]).u32();
    let target = e.call(READ_DWORD_AT_AC, &args![camera]).u32();
    e.call(COPY_16_BYTES_TO_OFFSET_100, &args![target, SOURCE_011AD840]);

    if held_object != 0 && process_slot(e, this, 0x454, &args![]) as u8 != 0 {
        let base = e.call(READ_FLOAT_AT_110, &args![held_object]).f32();
        zoom = e.call(FLOAT_FUNCTION_00408840, &args![base]).f32();
        let ammo = process_slot(e, this, 0x148, &args![]);
        let bonus = e.with_stack(4, |e, cell| {
            e.mem.set_f32(cell.addr(), 0.0);
            let found = e
                .call(ITEM_CHANGE_HAS_MOD_EFFECT, &args![ammo, 0xeu32, cell])
                .bool();
            found.then(|| e.mem.f32(cell.addr()))
        });
        if let Some(bonus) = bonus {
            zoom = (zoom as f64 - bonus as f64) as f32;
        }
    }

    if e.call(PLAYER_IS_PIPBOY_ACTIVE, &args![this]).bool() {
        if e.call(ACTOR_GET_IRON_SIGHTS, &args![this]).bool() {
            e.call(ACTOR_SET_IRON_SIGHTS, &args![this, 0u32, 0u32, 0u32]);
        }
        e.set(this, PlayerCharacter::f1stPersonFOV, default_second);
        e.set(this, PlayerCharacter::fWorldFOV, default_world);
        return;
    }

    if e.mem.u8(FLAG_011E07B8) != 0 && e.mem.u8(FOV_MOVING_FLAG) != 0 {
        let camera = e.call(GET_CAMERA_HOLDER, &args![]).u32();
        let current = e.call(READ_FLOAT_AT_BC, &args![camera]).f32();
        if default_world != current {
            e.call(FUNCTION_00950610, &args![this, default_world]);
        }
        return;
    }

    let mut world_target = default_world;
    let mut first_person_target = default_first_person;
    let step_scale: f64 = e.global(DOUBLE_30);
    let step = ((delta as f64 * step_scale) / divisor as f64) as f32;

    let zoom_limit: f64 = e.global(DOUBLE_5);
    if e.call(ACTOR_GET_IRON_SIGHTS, &args![this]).bool() && zoom as f64 > zoom_limit {
        let state_is_9 = e.call(ACTOR_GET_ANIM_ACTION, &args![this]).i32() == 9;
        let state_is_x11 = !state_is_9 && e.call(ACTOR_GET_ANIM_ACTION, &args![this]).i32() == 0x11;
        let camera_state_4 = (state_is_9 || state_is_x11)
            && e.call(READ_FIELD_8, &args![OBJECT_011F2250]).i32() == 4;
        if !(state_is_9 || state_is_x11) || camera_state_4 {
            world_target = zoom;
            first_person_target =
                ((zoom as f64 / default_world as f64) * default_first_person as f64) as f32;
        }
    }

    let camera_mode = e.call(READ_FIELD_8, &args![OBJECT_011F2250]).i32();
    let object_21cc = e.global::<u32>(POINTER_011F21CC);
    let spring_case = (camera_mode == 2
        || (camera_mode != 2 && e.call(READ_FIELD_8, &args![OBJECT_011F2250]).i32() == 3))
        && object_21cc != 0
        && e.get(this, PlayerCharacter::bWant3rdPerson) == e.get(this, PlayerCharacter::b3rdPerson);

    if spring_case {
        let node = e.vcall(object_21cc, 0x1d0, &args![]).u32();
        let node_data = e.call(GET_FIELD_20_OR_DEFAULT, &args![node]).u32();
        let slot = e.call(LIST_NODE_SLOT, &args![node_data]).u32();
        let owner = e.global::<u32>(POINTER_011E07D0);
        let anchor = e.call(ADD_8C_TO_ADDRESS, &args![owner]).u32();
        let length = e.with_stack(0x10, |e, offset| {
            e.call(VECTOR_SUBTRACT, &args![anchor, offset, slot]);
            e.call(VECTOR_LENGTH, &args![offset]).f32()
        });
        let node_scale = e.call(READ_FLOAT_AT_C, &args![node_data]).f32();
        let multiplier = e.call(SETTING_FLOAT_VALUE, &args![SETTING_011E0B84]).f32();
        let phase = ((multiplier as f64 * node_scale as f64) / length as f64) as f32;
        let near = setting_float(e, SETTING_011CE5A0);
        let far = setting_float(e, SETTING_011CE344);
        let spread = (near as f64 - far as f64) as f32;
        let range = setting_float(e, SETTING_011CFE2C);
        let mut ratio = (length as f64 / range as f64) as f32;
        if ratio as f64 > e.global::<f64>(DOUBLE_ONE) {
            ratio = 1.0;
        }
        let base = setting_float(e, SETTING_011CE344);
        let blend = (spread as f64 * ratio as f64 + base as f64) as f32;
        let wave = e.call(FLOAT_FUNCTION_005DC330, &args![phase]).f32();
        world_target = (wave as f64 * blend as f64) as f32;
        let floor = setting_float(e, SETTING_011CEA44);
        if floor > world_target {
            world_target = floor;
        }
        first_person_target = world_target;
        let ceiling = setting_float(e, SETTING_01203150);
        if ceiling <= first_person_target {
            first_person_target = ceiling;
        }

        // Function-local statics: the two spring objects, created on first use.
        for (guard_bit, holder) in [
            (1u32, FIRST_STATE_OBJECT_SLOT),
            (2u32, SECOND_STATE_OBJECT_SLOT),
        ] {
            let guard = e.mem.u32(STATE_OBJECT_GUARD);
            if guard & guard_bit == 0 {
                e.mem.set_u32(STATE_OBJECT_GUARD, guard | guard_bit);
                let block = e.call(OPERATOR_NEW, &args![0x1a0u32]).u32();
                let object = if block == 0 {
                    0
                } else {
                    e.call(STATE_OBJECT_NEW, &args![block]).u32()
                };
                e.mem.set_u32(holder, object);
            }
        }
        let first_object = e.mem.u32(FIRST_STATE_OBJECT_SLOT);
        let second_object = e.mem.u32(SECOND_STATE_OBJECT_SLOT);
        if e.mem.u32(LAST_OBJECT_011E0D4C) != object_21cc {
            let mut rate: f32 = e.global(DEFAULT_VALUE_01016264);
            if e.call(READ_FIELD_8, &args![OBJECT_011F2250]).i32() == 1 {
                rate = setting_float(e, SETTING_011D48DC);
            } else if e.call(READ_FIELD_8, &args![OBJECT_011F2250]).i32() == 2 {
                rate = setting_float(e, SETTING_011D4A2C);
            }
            let world = e.get(this, PlayerCharacter::fWorldFOV);
            e.call(
                STATE_OBJECT_SET,
                &args![first_object, world, world_target, rate],
            );
            let first_person = e.get(this, PlayerCharacter::f1stPersonFOV);
            e.call(
                STATE_OBJECT_SET,
                &args![second_object, first_person, first_person_target, rate],
            );
            e.mem.set_u32(LAST_OBJECT_011E0D4C, object_21cc);
        }
        let time = e.call(READ_FLOAT_AT_C, &args![OBJECT_011F6394]).f32();
        e.call(STATE_OBJECT_ADVANCE, &args![first_object, time]);
        let time = e.call(READ_FLOAT_AT_C, &args![OBJECT_011F6394]).f32();
        e.call(STATE_OBJECT_ADVANCE, &args![second_object, time]);
        let world = e.call(STATE_OBJECT_GET, &args![first_object]).f32();
        e.set(this, PlayerCharacter::fWorldFOV, world);
        let first_person = e.call(STATE_OBJECT_GET, &args![second_object]).f32();
        e.set(this, PlayerCharacter::f1stPersonFOV, first_person);
        if e.call(STATE_OBJECT_CHECK, &args![second_object]).bool() {
            e.call(FUNCTION_007F3BD0, &args![1u32]);
            e.mem.set_u8(FOV_MOVING_FLAG, 1);
        } else {
            e.call(FUNCTION_007F3BD0, &args![0u32]);
            e.mem.set_u8(FOV_MOVING_FLAG, 0);
        }
        return;
    }

    if e.call(READ_FIELD_8, &args![OBJECT_011F2250]).i32() == 4 {
        e.mem.set_u32(LAST_OBJECT_011E0D4C, 0);
        e.set(this, PlayerCharacter::fWorldFOV, world_target);
        e.set(this, PlayerCharacter::f1stPersonFOV, first_person_target);
        e.mem.set_u8(FOV_MOVING_FLAG, 1);
        return;
    }

    e.mem.set_u32(LAST_OBJECT_011E0D4C, 0);
    let world = e.get(this, PlayerCharacter::fWorldFOV);
    e.set(
        this,
        PlayerCharacter::fWorldFOV,
        approach(world, world_target, step),
    );
    let first_person = e.get(this, PlayerCharacter::f1stPersonFOV);
    let (first_person, reached) = approach_reporting(first_person, first_person_target, step);
    e.set(this, PlayerCharacter::f1stPersonFOV, first_person);
    e.mem.set_u8(FOV_MOVING_FLAG, reached as u8);
}

// Translated from 0095e5f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Walks the list of the object the player's virtual at +0x37c returns
/// (the list head is at +0x30 of it) and calls the player's virtual at
/// +0x3e4 with each non-empty item, stopping at the first empty one.
pub fn fn_0095e5f0(e: &mut Engine, this: Ptr<PlayerCharacter>) {
    let owner = e.vcall(this.addr(), 0x37c, &args![]).u32();
    if owner == 0 {
        return;
    }
    let owner = e.vcall(this.addr(), 0x37c, &args![]).u32();
    let mut node = e.call(ADD_4_TO_ADDRESS, &args![owner + 0x2c]).u32();
    while node != 0 {
        let slot = e.call(LIST_NODE_SLOT, &args![node]).u32();
        if e.mem.u32(slot) == 0 {
            break;
        }
        let slot = e.call(LIST_NODE_SLOT, &args![node]).u32();
        let item = e.mem.u32(slot);
        e.vcall(this.addr(), 0x3e4, &args![item]);
        node = e.call(LIST_NODE_NEXT, &args![node]).u32();
    }
}

// Translated from 0095e670 (decompiled, FalloutNV.exe 1.4.0.525)
/// Runs `008c1d20(this, a, b)`, then looks the inventory item for `a` up
/// (`00576260`, with the result of `0084e3a0(a)`); when found the temporary
/// is deleted and `0094c950(this, a)` is called, else `0094c950(this, 0)`.
pub fn fn_0095e670(e: &mut Engine, this: Ptr<PlayerCharacter>, a: Ptr, b: u32) {
    e.call(FUNCTION_008C1D20, &args![this, a, b]);
    let extra = e.call(READ_DWORD_AT_C, &args![a]).u32();
    let found = e.call(GET_INVENTORY_ITEM_REF, &args![this, a, extra]).u32();
    if found != 0 {
        e.call(SCALAR_DELETE, &args![found, 1u32]);
        e.call(FUNCTION_0094C950, &args![this, a]);
    } else {
        e.call(FUNCTION_0094C950, &args![this, 0u32]);
    }
}

/// Shows the message box the skill-advance code uses: `text`, the callback
/// [`fn_0095e900`] (`0095e900`), the button count and button texts (a
/// zero-terminated list), as the game pushes them.
fn show_message_box(e: &mut Engine, text: u32, buttons: &[u32]) {
    let mut words = args![
        text,
        0u32,
        0u32,
        0x0095_e900u32,
        buttons.len() as u32,
        0x17u32
    ];
    words.extend(args![0.0f32, 0.0f32]);
    words.extend_from_slice(buttons);
    words.push(0);
    e.call(FUNCTION_00703E80, &words);
}

/// The failure message of [`fn_0095e6f0`]: setting `0011d38b8` is the only
/// button, `message_setting` the text.
fn show_skill_failure_message(e: &mut Engine, message_setting: u32) {
    let button = e.call(SETTING_STRING, &args![SETTING_011D38B8]).u32();
    let text = e.call(SETTING_STRING, &args![message_setting]).u32();
    show_message_box(e, text, &[button]);
}

// Translated from 0095e6f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Starts an advance of the item the player holds: with `item` zero nothing
/// happens. Otherwise the process's held change (virtual at +0x148) must
/// cast to the type at `01183998`, have a kind of 0x2d or 0x26 and no
/// poison; if not, a message box with one of the settings texts is shown.
/// When all holds, `pPendingPoison` (+0x758) is set to `item` and a message
/// box with two buttons asks for confirmation (its callback is
/// [`fn_0095e900`]).
pub fn fn_0095e6f0(e: &mut Engine, this: Ptr<PlayerCharacter>, item: u32) {
    if item == 0 {
        return;
    }
    let process = e.call(PLAYER_GET_PROCESS, &args![this]).u32();
    let held = e.vcall(process, 0x148, &args![]).u32();
    let weapon = if held != 0 {
        let object = e.call(READ_FIELD_8, &args![held]).u32();
        dynamic_cast_form(e, object, TYPE_EQUIPPABLE)
    } else {
        0
    };
    if weapon == 0 {
        show_skill_failure_message(e, SETTING_011D2748);
        return;
    }
    if e.call(READ_DWORD_AT_15C, &args![weapon]).i32() != 0x2d
        && e.call(READ_DWORD_AT_15C, &args![weapon]).i32() != 0x26
    {
        show_skill_failure_message(e, SETTING_011D2748);
        return;
    }
    if e.call(ITEM_CHANGE_GET_POISON, &args![held]).u32() != 0 {
        show_skill_failure_message(e, SETTING_011D2814);
        return;
    }
    e.set(this, PlayerCharacter::pPendingPoison, item);
    let name = e.call(SETTING_STRING, &args![SETTING_011D4E94]).u32();
    let weapon_name = e
        .call(STRING_OR_EMPTY_00408DA0, &args![weapon + 0x30])
        .u32();
    e.with_stack(0x110, |e, buffer| {
        e.call(SPRINTF, &args![buffer, FORMAT_0108B3B4, name, weapon_name]);
        let first = e.call(SETTING_STRING, &args![SETTING_011D3684]).u32();
        let second = e.call(SETTING_STRING, &args![SETTING_011D34F8]).u32();
        show_message_box(e, buffer.addr(), &[second, first]);
    });
}

// Translated from 0095e900 (decompiled, FalloutNV.exe 1.4.0.525)
/// The callback of the message box [`fn_0095e6f0`] shows (no arguments):
/// when the pending `pPendingPoison` of the player is set and the box was
/// answered with button 2, calls `005f5950` for the codes 5 and 9, hands
/// the pending value to the held change (`004bdd20`), calls the player's
/// virtual at +0x17c and then `00704af0`. Always clears `pPendingPoison`
/// when it was set.
pub fn fn_0095e900(e: &mut Engine) {
    let player = Ptr::<PlayerCharacter>::new(e.global::<u32>(PLAYER_CHARACTER));
    if e.get(player, PlayerCharacter::pPendingPoison) == 0 {
        return;
    }
    let answer = e.call(GET_MESSAGE_MENU_RESULT, &args![]).u8() as i8;
    if answer == 2 {
        let process = e.call(PLAYER_GET_PROCESS, &args![player]).u32();
        let held = e.vcall(process, 0x148, &args![]).u32();
        let weapon = if held != 0 {
            let object = e.call(READ_FIELD_8, &args![held]).u32();
            dynamic_cast_form(e, object, TYPE_EQUIPPABLE)
        } else {
            0
        };
        if weapon != 0 {
            for code in [5u32, 9u32] {
                let value = e.get(player, PlayerCharacter::pPendingPoison);
                e.call(
                    FUNCTION_005F5950,
                    &args![code, 1u32, value, 0u32, 0u32, 0u32, 0u32],
                );
            }
            let value = e.get(player, PlayerCharacter::pPendingPoison);
            e.call(ITEM_CHANGE_FUNCTION_004BDD20, &args![held, value]);
            let value = e.get(player, PlayerCharacter::pPendingPoison);
            e.vcall(
                player.addr(),
                0x17c,
                &args![value, 0u32, 1u32, 0u32, 0u32, 0u32, 0u32, 0u32, 1u32, 0u32],
            );
            e.call(FUNCTION_00704AF0, &args![]);
        }
    }
    e.set(player, PlayerCharacter::pPendingPoison, 0);
}

// Translated from 0095ea30 (decompiled, FalloutNV.exe 1.4.0.525)
/// For a form of type 0x18 or 0x1a, casts it to the type at `011839b4` and
/// takes the pointer `00726070` reads from the cast (0 when the cast
/// fails); when that is not 0, calls the virtual at +0x14 of the base at
/// +0x88 with (the result plus 0x18, `form`, 0) and then `00824110` on the
/// base at +0x94 with the result plus 0x18.
pub fn fn_0095ea30(e: &mut Engine, this: Ptr<PlayerCharacter>, form: Ptr) {
    let kind = e.call(FORM_GET_TYPE, &args![form]).u32();
    if kind != 0x18 && e.call(FORM_GET_TYPE, &args![form]).u32() != 0x1a {
        return;
    }
    let cast = dynamic_cast_form(e, form.addr(), TYPE_MAGIC_ITEM);
    let field = if cast != 0 {
        e.call(READ_FIELD_4, &args![cast]).u32()
    } else {
        0
    };
    if field == 0 {
        return;
    }
    let inner = field + 0x18;
    e.vcall(
        this.addr() + MAGIC_CASTER_OFFSET,
        0x14,
        &args![inner, form, 0u32],
    );
    e.call(
        MAGIC_TARGET_ADD_EFFECT_ITEM,
        &args![this.byte_add(MAGIC_TARGET_OFFSET), inner],
    );
}

// Translated from 0095eb10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PlayerCharacter::RemoveQueuedEnchantment` (Xbox PDB): for a form of
/// type 0x18 or 0x1a with a queued enchantment list, casts it like
/// [`fn_0095ea30`]; when the base at +0x94 knows the cast's inner
/// pointer (`00822b90`), asks it to drop `form` (`008248e0`). Then removes
/// `form` from the list (`00905330` with the address of the argument) and
/// frees the list once it is empty.
pub fn player_character_remove_queued_enchantment(
    e: &mut Engine,
    this: Ptr<PlayerCharacter>,
    form: Ptr,
) {
    let kind = e.call(FORM_GET_TYPE, &args![form]).u32();
    if kind != 0x18 && e.call(FORM_GET_TYPE, &args![form]).u32() != 0x1a {
        return;
    }
    if e.get(this, PlayerCharacter::QueuedWornEnchantments)
        .is_null()
    {
        return;
    }
    let cast = dynamic_cast_form(e, form.addr(), TYPE_MAGIC_ITEM);
    let field = if cast != 0 {
        e.call(READ_FIELD_4, &args![cast]).u32()
    } else {
        0
    };
    if field == 0 {
        return;
    }
    let inner = field + 0x18;
    let target = this.byte_add(MAGIC_TARGET_OFFSET);
    if e.call(MAGIC_TARGET_IS_SPELL_TARGET, &args![target, inner, 1u32])
        .bool()
    {
        e.call(FUNCTION_008248E0, &args![target, form, 0u32]);
    }
    let list = e.get(this, PlayerCharacter::QueuedWornEnchantments);
    e.with_stack(4, |e, cell| {
        e.mem.set_u32(cell.addr(), form.addr());
        e.call(LIST_REMOVE, &args![list, cell]);
    });
    let list = e.get(this, PlayerCharacter::QueuedWornEnchantments);
    if e.call(LIST_NODE_IS_EMPTY, &args![list]).bool() {
        let list = e.get(this, PlayerCharacter::QueuedWornEnchantments);
        if !list.is_null() {
            e.call(LIST_DELETE, &args![list, 1u32]);
        }
        e.set(this, PlayerCharacter::QueuedWornEnchantments, Ptr::NULL);
    }
}

// Translated from 0095ec40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PlayerCharacter::CastQueuedEnchantments` (Xbox PDB): for every queued
/// enchantment that casts to the type at `011839b4` and yields a pointer,
/// calls the virtual at +0x14 of the base at +0x88 like [`fn_0095ea30`];
/// then frees the list's items (`00470470`), deletes the list and clears the
/// field.
pub fn player_character_cast_queued_enchantments(e: &mut Engine, this: Ptr<PlayerCharacter>) {
    if e.get(this, PlayerCharacter::QueuedWornEnchantments)
        .is_null()
    {
        return;
    }
    let mut node = e.get(this, PlayerCharacter::QueuedWornEnchantments).addr();
    while node != 0 && !e.call(LIST_NODE_IS_EMPTY, &args![node]).bool() {
        let slot = e.call(LIST_NODE_SLOT, &args![node]).u32();
        if e.mem.u32(slot) != 0 {
            let slot = e.call(LIST_NODE_SLOT, &args![node]).u32();
            let form = e.mem.u32(slot);
            let cast = dynamic_cast_form(e, form, TYPE_MAGIC_ITEM);
            let field = if cast != 0 {
                e.call(READ_FIELD_4, &args![cast]).u32()
            } else {
                0
            };
            if field != 0 {
                let inner = field + 0x18;
                let slot = e.call(LIST_NODE_SLOT, &args![node]).u32();
                let form = e.mem.u32(slot);
                e.vcall(
                    this.addr() + MAGIC_CASTER_OFFSET,
                    0x14,
                    &args![inner, form, 0u32],
                );
            }
        }
        node = e.call(LIST_NODE_NEXT, &args![node]).u32();
    }
    let list = e.get(this, PlayerCharacter::QueuedWornEnchantments);
    e.call(LIST_CLEAR_ITEMS, &args![list]);
    let list = e.get(this, PlayerCharacter::QueuedWornEnchantments);
    if !list.is_null() {
        e.call(LIST_DELETE, &args![list, 1u32]);
    }
    e.set(this, PlayerCharacter::QueuedWornEnchantments, Ptr::NULL);
}

// Translated from 0095ed80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Zeroes every `TemporaryActorValueModifiers` entry (0x4d floats from
/// +0x244) that is not 0.0 and whose index the base at +0x94 does not report
/// as active (`00822f70`).
pub fn fn_0095ed80(e: &mut Engine, this: Ptr<PlayerCharacter>) {
    let zero: f64 = e.global(ZERO_DOUBLE);
    for index in 0..ACTOR_VALUE_MODIFIER_COUNT {
        let address = this.addr() + 0x244 + index * 4;
        let value = e.mem.f32(address);
        if value as f64 != zero
            && !e
                .call(
                    FUNCTION_00822F70,
                    &args![this.byte_add(MAGIC_TARGET_OFFSET), index],
                )
                .bool()
        {
            e.mem.set_f32(address, 0.0);
        }
    }
}

// Translated from 0095edf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PlayerCharacter::Setup3rdPersonCameraCaster` (Xbox PDB): when the player
/// has a camera caster and an object at `008d6f30`, makes the caster
/// follow what `004543c0` answers for that object if it differs from what
/// the caster has (`00621440`).
pub fn player_character_setup_3rd_person_camera_caster(e: &mut Engine, this: Ptr<PlayerCharacter>) {
    let caster = e.get(this, PlayerCharacter::pCameraCaster);
    if caster.is_null() || e.call(READ_DWORD_AT_40, &args![this]).u32() == 0 {
        return;
    }
    let current = e.call(CASTER_GET_WORLD, &args![caster]).u32();
    let object = e.call(READ_DWORD_AT_40, &args![this]).u32();
    let wanted = e.call(OBJECT_FUNCTION_004543C0, &args![object]).u32();
    if current != wanted {
        let object = e.call(READ_DWORD_AT_40, &args![this]).u32();
        let wanted = e.call(OBJECT_FUNCTION_004543C0, &args![object]).u32();
        let caster = e.get(this, PlayerCharacter::pCameraCaster);
        e.call(CASTER_SET_WORLD, &args![caster, wanted]);
    }
}

// Translated from 0095ee60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Forwards to `00705800` and returns its result.
pub fn fn_0095ee60(e: &mut Engine) -> u32 {
    e.call(FUNCTION_00705800, &args![]).u32()
}

// Translated from 0095ee70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Forwards to `007d0b90` on the object the global at `011dea0c` points to
/// and returns its result.
pub fn fn_0095ee70(e: &mut Engine) -> u32 {
    let object = e.global::<u32>(POINTER_011DEA0C);
    e.call(FUNCTION_007D0B90, &args![object]).u32()
}

/// Copies `text` over the path buffer at `destination`, bounded by the
/// space left after `position` (`strcpy_s(destination, 0x104 - (position -
/// buffer) - 1, text)`).
fn append_path(e: &mut Engine, buffer: u32, position: u32, destination: u32, text: u32) {
    let size = 0x104u32
        .wrapping_sub(position.wrapping_sub(buffer))
        .wrapping_sub(1);
    e.call(STRING_COPY, &args![destination, size, text]);
}

/// Adds the file at `buffer` to the idle manager's root idle arrays.
fn add_root_idle_array(e: &mut Engine, buffer: u32) {
    let manager = e.global::<u32>(IDLE_MANAGER_POINTER);
    e.call(ADD_ROOT_IDLE_ARRAY, &args![manager, buffer, 0u32, 0u32]);
}

// Translated from 0095ee80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PlayerCharacter::Preload1stPersonAnimsFileList` (Xbox PDB): builds the
/// path of the first-person animation folder from the setting at
/// `011cdd78` (cut after its last backslash), and registers, with the
/// idle manager, the folder's file name patterns (the suffix strings at
/// `0108b3d8`, `0108b3bc`, `01016f1c`, `01016f38`, `0108b0d8` after the
/// `01016f..` folder name `010170ec`), then the same patterns under the
/// player's model path (`005715d0`), where the last pattern is `01016f58`.
pub fn player_character_preload_1st_person_anims_file_list(
    e: &mut Engine,
    this: Ptr<PlayerCharacter>,
) {
    e.with_stack(8, |e, guard| {
        e.call(
            SCOPE_GUARD_NEW,
            &args![guard, 0x34u32, 1u32, SCOPE_GUARD_FILE, 0x41c5u32],
        );
        e.with_stack(0x104, |e, buffer| {
            let buffer = buffer.addr();
            let folder = e.call(SETTING_STRING, &args![SETTING_011CDD78]).u32();
            e.call(STRING_COPY, &args![buffer, 0x104u32, folder]);
            let mut position = e.call(STRING_FIND_LAST, &args![buffer, 0x5cu32]).u32();
            if position != 0 {
                append_path(e, buffer, position, position + 1, SUFFIX_010170EC);
            }
            add_root_idle_array(e, buffer);
            for suffix in [
                SUFFIX_0108B3D8,
                SUFFIX_0108B3BC,
                SUFFIX_01016F1C,
                SUFFIX_01016F38,
                SUFFIX_0108B0D8,
            ] {
                append_path(e, buffer, position, position, suffix);
                add_root_idle_array(e, buffer);
            }
            let model = e.call(GET_MODEL_PATH, &args![this]).u32();
            e.call(STRING_COPY, &args![buffer, 0x104u32, model]);
            position = e.call(STRING_FIND_LAST, &args![buffer, 0x5cu32]).u32();
            for suffix in [
                SUFFIX_0108B3D8,
                SUFFIX_0108B3BC,
                SUFFIX_01016F1C,
                SUFFIX_01016F38,
                SUFFIX_01016F58,
            ] {
                append_path(e, buffer, position, position, suffix);
                add_root_idle_array(e, buffer);
            }
        });
        e.call(SCOPE_GUARD_DELETE, &args![guard]);
    });
}

// Translated from 0095f210 (decompiled, FalloutNV.exe 1.4.0.525)
/// Queues the animation file lists for the player's base form: sets the
/// base's string by sex (`005f0cc0` is 1: setting `011d2718`, else
/// `011d3e44`), builds the file list of the first-person folder and, when
/// `008397d0`/`0087f4c0` say so, the extra and sex-specific suffixes, hands
/// every list to the model loader, then does the same for the player's own
/// model path. `first` and `second` are passed through to the loader.
/// The exception frame is not translated.
pub fn fn_0095f210(e: &mut Engine, this: Ptr<PlayerCharacter>, first: u32, second: u32) {
    let loader = e.global::<u32>(MODEL_LOADER_POINTER);
    let manager = e.global::<u32>(IDLE_MANAGER_POINTER);
    e.with_stack(8, |e, guard| {
        e.call(
            SCOPE_GUARD_NEW,
            &args![guard, 0x34u32, 1u32, SCOPE_GUARD_FILE, 0x41ffu32],
        );
        let base = e.call(READ_DWORD_AT_20, &args![this]).u32();
        let sex = e.call(ACTOR_BASE_GET_SEX, &args![base]).u32();
        let setting = if sex == 1 {
            SETTING_011D2718
        } else {
            SETTING_011D3E44
        };
        let text = e.call(SETTING_STRING, &args![setting]).u32();
        e.vcall(base + 0xdc, 0x18, &args![text]);

        let folder = e.call(SETTING_STRING, &args![SETTING_011CDD78]).u32();
        let list = e
            .call(BUILD_FILE_LIST, &args![loader, folder, 1u32, 0u32, 0xcu32])
            .u32();
        e.with_stack(0x104, |e, buffer| {
            let buffer = buffer.addr();
            let folder = e.call(SETTING_STRING, &args![SETTING_011CDD78]).u32();
            let path = e.call(FUNCTION_00464F30, &args![folder, 0u32]).u32();
            e.call(STRING_COPY, &args![buffer, 0x104u32, path]);
            let position = e.call(STRING_FIND_LAST, &args![buffer, 0x5cu32]).u32();
            if e.call(FUNCTION_008397D0, &args![this]).bool() {
                append_path(e, buffer, position, position, SUFFIX_0108B0D8);
                let names = e
                    .call(GET_ROOT_FILENAME_LIST, &args![manager, buffer, 0u32])
                    .u32();
                e.call(COPY_FILENAME_LIST, &args![loader, names, list]);
            }
            let suffix = if e.call(FUNCTION_0087F4C0, &args![this]).u32() == 1 {
                SUFFIX_01016F38
            } else {
                SUFFIX_01016F1C
            };
            append_path(e, buffer, position, position, suffix);
            let names = e
                .call(GET_ROOT_FILENAME_LIST, &args![manager, buffer, 0u32])
                .u32();
            e.call(COPY_FILENAME_LIST, &args![loader, names, list]);
        });
        e.call(FUNCTION_00446500, &args![loader, list, first, second, 0u32]);
        if list != 0 {
            e.call(LIST_DELETE, &args![list, 1u32]);
        }
        let folder = e.call(SETTING_STRING, &args![SETTING_011CDD78]).u32();
        let path = e.call(FUNCTION_00464F30, &args![folder, 0u32]).u32();
        e.call(
            QUEUE_MODEL,
            &args![loader, path, first, second, 0u32, 1u32, 0u32, 0u32],
        );
        let model = e.call(GET_MODEL_PATH, &args![this]).u32();
        let list = e
            .call(BUILD_FILE_LIST, &args![loader, model, 1u32, 0u32, 0xcu32])
            .u32();
        e.call(FUNCTION_00446500, &args![loader, list, first, second, 0u32]);
        if list != 0 {
            e.call(LIST_DELETE, &args![list, 1u32]);
        }
        e.call(SCOPE_GUARD_DELETE, &args![guard]);
    });
}

// Translated from 0095f530 (decompiled, FalloutNV.exe 1.4.0.525)
/// With `enable` set, ORs `bits` into `ucControlsDisabled`; else clears
/// them; then applies the result with [`player_character_set_controls_disabled`].
pub fn fn_0095f530(e: &mut Engine, this: Ptr<PlayerCharacter>, enable: u8, bits: u8) {
    let current = e.get(this, PlayerCharacter::ucControlsDisabled);
    let value = if enable != 0 {
        current | bits
    } else {
        current & !bits
    };
    player_character_set_controls_disabled(e, this, value);
}

// Translated from 0095f590 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PlayerCharacter::SetControlsDisabled` (Xbox PDB): stores the disabled
/// controls byte (bit 7 is dropped unless the byte is 0xff) and applies it:
/// bit 0 selects HUD menu mode 0xc (set) or 1 (clear) through `00771700`;
/// bit 3 calls `008a6840(this, 0)`; bit 4 clears `bWant3rdPerson` and
/// updates the camera when the first-person 3D pointer is set (else
/// zeroes the global at `011e0768`); bit 6 has the actor mover (virtual
/// at +0xc) take the actor's `008846e0` value without bit 10.
pub fn player_character_set_controls_disabled(
    e: &mut Engine,
    this: Ptr<PlayerCharacter>,
    disabled: u8,
) {
    e.set(this, PlayerCharacter::ucControlsDisabled, disabled);
    if disabled != 0xff {
        e.set(this, PlayerCharacter::ucControlsDisabled, disabled & 0x7f);
    }
    let bits = e.get(this, PlayerCharacter::ucControlsDisabled);
    if bits & 1 != 0 {
        e.call(HUD_SET_MENU_MODE, &args![0xcu32]);
    } else {
        e.call(HUD_SET_MENU_MODE, &args![1u32]);
    }
    if e.get(this, PlayerCharacter::ucControlsDisabled) & 8 != 0 {
        e.call(FUNCTION_008A6840, &args![this, 0u32]);
    }
    if e.get(this, PlayerCharacter::ucControlsDisabled) & 0x10 != 0 {
        e.set(this, PlayerCharacter::bWant3rdPerson, 0);
        let pointer = this.byte_add(0x694);
        if e.call(READ_DWORD_AT_0, &args![pointer]).u32() != 0 {
            e.call(PLAYER_UPDATE_CAMERA, &args![this, 0u32, 0u32]);
        } else {
            e.set_global(GLOBAL_011E0768, 0.0f32);
        }
    }
    if e.get(this, PlayerCharacter::ucControlsDisabled) & 0x40 != 0 {
        let value = e.call(ACTOR_FUNCTION_008846E0, &args![this]).u16() & 0xfbff;
        let mover = e.get(this, PlayerCharacter::pActorMover);
        e.vcall(mover.addr(), 0xc, &args![u32::from(value)]);
    }
}

// Translated from 0095f6a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Forwards to [`fn_00961280`] and returns its result.
pub fn fn_0095f6a0(e: &mut Engine, this: Ptr<PlayerCharacter>) -> Ptr<PlayerCharacter> {
    fn_00961280(e, this)
}

// Translated from 0095f6c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Per-frame handling of the grab (mouse spring) controls. Reads the
/// control query object (`00877720` on the global at `011dea0c`) for control
/// 0x1b and the mode `00705a90`; with a grab spring present it keeps updating
/// it ([`fn_00960520`], grab type 1) or releases it ([`fn_00961280`]); with
/// none, when the control is down (or the mode is 1) and nothing is grabbed
/// yet, it picks the object in front of the player (virtual +0x1d0 of
/// `00703350`'s object, collision lookups `004b5260`/`006fa820`) and starts
/// a spring on it with [`player_character_create_mouse_spring`], marks
/// `011e0d5c` and hands the object over to `004213c0`/`00410140`.
/// `delta` is passed on to the update.
pub fn fn_0095f6c0(e: &mut Engine, this: Ptr<PlayerCharacter>, delta: f32) {
    let owner = e.global::<u32>(POINTER_011DEA0C);
    let controls = e.call(FUNCTION_00877720, &args![owner]).u32();
    let control_down = e.call(CONTROL_QUERY, &args![controls, 0x1bu32, 1u32]).u32() != 0;
    if control_down {
        e.call(FUNCTION_00705AD0, &args![0u32]);
    }
    let mode = e.call(FUNCTION_00705A90, &args![]).i32();
    let mode_is_1 = mode == 1;
    let mode_is_2 = mode == 2;
    if mode == 1 {
        e.call(FUNCTION_00705AD0, &args![2u32]);
    }
    let spring_slot = this.byte_add(0x634);
    if !control_down
        && e.mem.u8(FLAG_011E0D5C) == 0
        && e.call(READ_DWORD_AT_0, &args![spring_slot]).u32() == 0
        && !mode_is_1
        && !mode_is_2
    {
        return;
    }

    if e.call(READ_DWORD_AT_0, &args![spring_slot]).u32() != 0 {
        if e.get(this, PlayerCharacter::eGrabType) == 1 {
            if (e.mem.u8(FLAG_011E0D5C) != 0 && !control_down) || mode_is_2 {
                fn_00960520(e, this, delta);
            } else {
                fn_00961280(e, this);
                e.mem.set_u8(FLAG_011E0D5C, 0);
            }
        }
        return;
    }

    if !(control_down || mode_is_1) {
        return;
    }
    if e.get(this, PlayerCharacter::eGrabType) != 0 {
        return;
    }
    let target = e.call(FUNCTION_00703350, &args![]).u32();
    if target == 0 {
        return;
    }
    let target_kind = e.call(READ_DWORD_AT_20, &args![target]).u32();
    if target_kind == e.global::<u32>(POINTER_011CA27C) {
        return;
    }
    if e.call(READ_DWORD_AT_20, &args![target]).u32() == e.global::<u32>(POINTER_011CA280) {
        return;
    }
    let node = e.vcall(target, 0x1d0, &args![]).u32();
    let collision = if node != 0 {
        e.call(FIND_FIRST_COLLISION_OBJECT, &args![node]).u32()
    } else {
        0
    };
    let body = if collision != 0 {
        e.call(FUNCTION_006FA820, &args![collision]).u32()
    } else {
        0
    };
    if body == 0 {
        return;
    }
    if e.vcall(body, 0x94, &args![]).u32() == 0 {
        return;
    }
    let weight = e.call(BODY_VALUE_004B5400, &args![body]).f32();
    let limit = setting_float(e, SETTING_011D1230);
    if !matches!(
        weight.partial_cmp(&limit),
        Some(std::cmp::Ordering::Less | std::cmp::Ordering::Equal)
    ) {
        return;
    }
    let distance: f32 = e.global(F32_CONSTANT_01013974);
    player_character_create_mouse_spring(e, this, Ptr::new(target), 1, distance);
    if e.call(READ_DWORD_AT_0, &args![spring_slot]).u32() != 0 {
        e.vcall(target, 0x48, &args![4u32]);
    }
    e.mem.set_u8(FLAG_011E0D5C, 1);
    let extra = e.call(ADD_44_TO_ADDRESS, &args![target]).u32();
    e.call(FUNCTION_004213C0, &args![extra, target, this]);
    let extra = e.call(ADD_44_TO_ADDRESS, &args![target]).u32();
    e.call(EXTRA_LIST_REMOVE_EXTRA, &args![extra, 0x7cu32]);
}

/// The locals of a function whose stack frame a translation emulates in one
/// heap block: [`Frame::at`] is the address of the game's `[EBP + offset]`,
/// offsets written as the code has them (negative ones as 32-bit values).
#[derive(Clone, Copy)]
struct Frame {
    ebp: u32,
}

impl Frame {
    fn at(self, offset: u32) -> u32 {
        self.ebp.wrapping_add(offset)
    }
}

/// `-value` as the 32-bit offset the disassembly shows for `[EBP - value]`.
fn neg(value: u32) -> u32 {
    0u32.wrapping_sub(value)
}

/// Size of the emulated frame of [`player_character_create_mouse_spring`]
/// (its locals reach down to `[EBP - 0x280]`), and where `EBP` is in it.
const CREATE_MOUSE_SPRING_FRAME: u32 = 0x300;
const CREATE_MOUSE_SPRING_EBP: u32 = 0x2b0;

// Translated from 0095f930 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PlayerCharacter::CreateMouseSpring` (Xbox PDB): starts holding `object`
/// with a mouse spring of kind `grab_type` at `distance`.
///
/// Unless the setting at `011e0b1c` is on it only grabs actors that answer
/// the virtual at +0x22c, and it refuses an object whose virtuals at +0x224,
/// +0x304 (3) and +0x318 say so and that `00444d00(.., 0x200)` rejects. With
/// the object's collision body found (`004b52f0` on its 3D) it records the
/// grab (grabbed object, grab type, distance, flag 0x80000 on its extra
/// data), finds where to hold it (a ray cast from the eye position for grab
/// type 1, or the body's own position), collects the spring settings in the
/// parameter block at `[EBP - 0x70]` (stiffness and damping settings scaled
/// by the object's kind and mass), and builds the `bhkMouseSpringAction`
/// (`004b5040`) into the spring slot. Without a collision body only grab
/// type 2 is recorded. The exception-unwinding frame is not translated; the
/// 0x2b0-byte frame is emulated so that the locals the callees receive by
/// address keep the layout the game gives them.
pub fn player_character_create_mouse_spring(
    e: &mut Engine,
    this: Ptr<PlayerCharacter>,
    object: Ptr,
    grab_type: u32,
    distance: f32,
) {
    // `EBP` is on a 16-byte boundary in the exe (`AND ESP,0xfffffff0` before `MOV EBP,ESP`).
    e.with_stack(CREATE_MOUSE_SPRING_FRAME + 0x10, |e, raw| {
        let frame = Frame {
            ebp: (raw.addr() + CREATE_MOUSE_SPRING_EBP + 0xf) & !0xf,
        };
        create_mouse_spring_in_frame(e, frame, this, object, grab_type, distance)
    });
}

fn create_mouse_spring_in_frame(
    e: &mut Engine,
    f: Frame,
    this: Ptr<PlayerCharacter>,
    object: Ptr,
    grab_type: u32,
    distance: f32,
) {
    let obj = object.addr();
    let option = e.call(SETTING_BOOL_POINTER, &args![SETTING_011E0B1C]).u32();
    // 0095f981..0095f9b3: with the setting off, only an object whose virtual 0x100 answers true
    // and whose 0x22c(0) answers false is refused; 0x100 false goes on (`JZ 0x0095f9b8`), so
    // clutter such as a paperweight is grabbed.
    if e.mem.u8(option) == 0
        && e.vcall(obj, 0x100, &args![]).bool()
        && !e.vcall(obj, 0x22c, &args![0u32]).bool()
    {
        return;
    }
    if obj != 0
        && e.vcall(obj, 0x224, &args![]).bool()
        && e.vcall(obj, 0x304, &args![]).u32() == 3
        && e.vcall(obj, 0x318, &args![]).bool()
        && !e.call(FUNCTION_00444D00, &args![obj, 0x200u32]).bool()
    {
        return;
    }
    if obj == 0 {
        return;
    }
    if e.call(READ_DWORD_AT_40, &args![obj]).u32() == 0 {
        return;
    }
    let node = e.vcall(obj, 0x1d0, &args![]).u32();
    let body = e.call(FUNCTION_004B52F0, &args![node]).u32();
    if body == 0 {
        if grab_type == 2 {
            e.set(this, PlayerCharacter::pGrabbedObject, object);
            let extra = e.call(ADD_44_TO_ADDRESS, &args![object]).u32();
            e.call(SCRIPT_SET_ACTION_FLAG, &args![0u32, extra, 0x8_0000u32]);
            e.set(this, PlayerCharacter::eGrabType, 2);
            e.set(this, PlayerCharacter::fGrabDistance, distance);
        }
        return;
    }
    let big = e.call(FUNCTION_00624B70, &args![node, 8u32]).u32() != 0
        || e.vcall(obj, 0x100, &args![]).bool();
    e.set(this, PlayerCharacter::pGrabbedObject, object);
    let extra = e.call(ADD_44_TO_ADDRESS, &args![object]).u32();
    e.call(SCRIPT_SET_ACTION_FLAG, &args![0u32, extra, 0x8_0000u32]);
    e.set(this, PlayerCharacter::eGrabType, grab_type);
    e.set(this, PlayerCharacter::fGrabDistance, distance);

    // [EBP - 0x70]: the spring parameter block (its members at +0x10 and
    // +0x20 are the vectors [EBP - 0x60] and [EBP - 0x50]).
    let params = f.at(neg(0x70));
    let vector_60 = f.at(neg(0x60));
    let vector_50 = f.at(neg(0x50));
    e.call(PARAMS_NEW_004B4F40, &args![params]);
    e.mem.set_f32(f.at(neg(0x74)), 0.0);
    let hold_point = f.at(0xffff_ff70);
    e.call(IDENTITY_006815C0, &args![hold_point]);
    if big && e.vcall(obj, 0x100, &args![]).bool() {
        e.call(FUNCTION_00C8E940, &args![node, hold_point, f.at(neg(0x74))]);
    } else {
        e.vcall(body, 0xf0, &args![hold_point]);
    }
    let eye_position = f.at(0xffff_ff58);
    let eye_direction = f.at(0xffff_ff64);
    e.call(IDENTITY_006815C0, &args![eye_direction]);
    e.call(IDENTITY_006815C0, &args![eye_position]);
    e.call(
        ACTOR_GET_EYE_VECTOR,
        &args![this, eye_position, eye_direction, 1u32],
    );
    let mut hit_found = false;
    let loaded = e.call(READ_DWORD_AT_40, &args![object]).u32();
    let held = e.call(OBJECT_FUNCTION_004543C0, &args![loaded]).u32();
    let mut hit_object = 0u32;
    let flags_object = |e: &Engine| e.global::<u32>(POINTER_011DDF38);

    if e.get(this, PlayerCharacter::eGrabType) == 1 && held != 0 {
        let flags = flags_object(e);
        if e.call(TEST_BIT_2_AT_244, &args![flags]).bool() {
            // Skips the whole search.
        } else if e.call(FUNCTION_00703430, &args![]).bool() && fn_00960510(e) != 0 {
            let vector = f.at(0xffff_fe50);
            e.call(FUNCTION_007033D0, &args![vector]);
            let flags = flags_object(e);
            if !e.call(TEST_BIT_2_AT_244, &args![flags]).bool() {
                let difference = e
                    .call(
                        FUNCTION_00439EF0,
                        &args![vector, f.at(0xffff_fe44), eye_position],
                    )
                    .u32();
                let length = e.call(FUNCTION_00457990, &args![difference]).f32();
                e.set(this, PlayerCharacter::fGrabDistance, length);
            }
            e.call(FUNCTION_004B4E50, &args![vector_50, vector]);
            let owner = fn_00960510(e);
            hit_object = e.call(BODY_LOOKUP_004AE750, &args![owner]).u32();
            hit_found = true;
        } else {
            let ray = f.at(0xffff_fea0);
            e.call(FUNCTION_004A3C20, &args![ray]);
            e.call(FUNCTION_008C71B0, &args![f.at(0xffff_fe9c), 0u32]);
            e.call(FUNCTION_004A39F0, &args![f.at(0xffff_fe9c), 0x24u32]);
            let owner = e
                .call(FUNCTION_00931ED0, &args![this, f.at(0xffff_fe98)])
                .u32();
            let filter = e.call(FUNCTION_004A3A20, &args![owner]).u32();
            e.call(FUNCTION_0059CE80, &args![f.at(0xffff_fe9c), filter]);
            let filter_word = e.mem.u32(f.at(0xffff_fe9c));
            e.call(FUNCTION_004A3F70, &args![ray, filter_word]);
            e.call(FUNCTION_004A3DA0, &args![ray, eye_position]);
            let reach = e.get(this, PlayerCharacter::fGrabDistance);
            let end = e
                .call(
                    FUNCTION_0045BB20,
                    &args![eye_direction, f.at(0xffff_fe8c), reach],
                )
                .u32();
            e.call(FUNCTION_007F68A0, &args![ray, end]);
            let hit = e.vcall(held, 0xc8, &args![ray]).bool();
            if hit {
                let picked = e.call(FUNCTION_00C66FD0, &args![ray]).u32();
                let picked_node = e.call(FUNCTION_004B5A20, &args![picked]).u32();
                let picked_form = if picked_node != 0 {
                    e.call(READ_FIELD_8, &args![picked_node]).u32()
                } else {
                    0
                };
                let reference = e.call(FIND_REFERENCE_FOR_3D, &args![picked_form]).u32();
                if picked != 0 && reference == obj {
                    if !big {
                        let body_object = e.call(FUNCTION_004B5A80, &args![picked]).u32();
                        e.vcall(body_object, 0xf0, &args![hold_point]);
                    }
                    hit_found = true;
                    hit_object = picked;
                    let fraction = e.mem.f32(f.at(0xffff_fee0));
                    let reach = e.get(this, PlayerCharacter::fGrabDistance);
                    e.set(
                        this,
                        PlayerCharacter::fGrabDistance,
                        (reach as f64 * fraction as f64) as f32,
                    );
                    let reach = e.get(this, PlayerCharacter::fGrabDistance);
                    let end = e
                        .call(
                            FUNCTION_0045BB20,
                            &args![eye_direction, f.at(0xffff_fe68), reach],
                        )
                        .u32();
                    e.call(
                        FUNCTION_00439E90,
                        &args![eye_position, f.at(0xffff_fe5c), end],
                    );
                    e.call(FUNCTION_004B4E50, &args![vector_50, f.at(0xffff_fe5c)]);
                }
            }
        }
    }

    if !hit_found {
        e.call(FUNCTION_004B4DB0, &args![vector_50, hold_point]);
        let offset = f.at(0xffff_fe30);
        e.call(IDENTITY_006815C0, &args![offset]);
        if big {
            e.call(IDENTITY_006815C0, &args![f.at(0xffff_fe20)]);
            e.call(IDENTITY_006815C0, &args![f.at(0xffff_fe10)]);
            let transform = e.call(FUNCTION_00461130, &args![node]).u32();
            let result = e
                .call(
                    FUNCTION_004B4500,
                    &args![transform, f.at(0xffff_fe04), UP_VECTOR_011A9484],
                )
                .u32();
            e.call(FUNCTION_00553FC0, &args![f.at(0xffff_fe10), result]);
            e.call(FUNCTION_004B4D10, &args![vector_50, offset]);
            e.call(FUNCTION_00553F70, &args![offset, f.at(0xffff_fe10)]);
            e.call(FUNCTION_004B4DB0, &args![vector_50, offset]);
        }
        let flags = flags_object(e);
        if !e.call(TEST_BIT_2_AT_244, &args![flags]).bool() {
            e.call(IDENTITY_006815C0, &args![f.at(0xffff_fdf8)]);
            let returned = e.call(FUNCTION_004B4D10, &args![vector_50, offset]).u32();
            e.call(FUNCTION_00458620, &args![f.at(0xffff_fdf8), returned]);
            let difference = e
                .call(
                    FUNCTION_00439EF0,
                    &args![f.at(0xffff_fdf8), f.at(0xffff_fdec), eye_position],
                )
                .u32();
            let length = e.call(FUNCTION_00457990, &args![difference]).f32();
            e.set(this, PlayerCharacter::fGrabDistance, length);
        }
        hit_object = e.call(BODY_LOOKUP_004AE750, &args![body]).u32();
    }

    if e.get(this, PlayerCharacter::eGrabType) == 1 {
        let value: f32 = e.global(F32_CONSTANT_01017868);
        e.call(FUNCTION_00963EB0, &args![this, 5u32, value, object]);
    }

    // The found body is the spring's `+4` word ([EBP-0x6c] beside the block at [EBP-0x70]): the
    // exe stores it there where it finds it (0095fd78, 0095fe45, 0095ff5e) and reads it back here
    // (0095ff85); bhkMouseSpringAction::Init (00c87260) hands it to the Havok action as its
    // entity. Without it the action was built on a null entity and the grab stopped the game.
    e.mem.set_u32(f.at(neg(0x6c)), hit_object);
    let hit = hit_object;
    if e.call(READ_FIELD_8, &args![hit]).u32() == 0
        || e.call(FUNCTION_00517670, &args![hit]).u32() == 5
        || e.call(FUNCTION_00517670, &args![hit]).u32() == 4
    {
        fn_00961280(e, this);
        return;
    }
    let controller = e.call(GET_CHAR_CONTROLLER, &args![this]).u32();
    if controller != 0 {
        let radius = e.call(FUNCTION_00C6E280, &args![controller]).f32();
        let scaled = e.call(FUNCTION_004587D0, &args![radius]).f32();
        let margin: f64 = e.global(DOUBLE_5);
        let limit = (scaled as f64 + margin) as f32;
        let reach = e.get(this, PlayerCharacter::fGrabDistance);
        if limit > reach {
            e.set(this, PlayerCharacter::fGrabDistance, limit);
        }
    }

    // The spring parameters: [EBP - 0x40], -0x3c, -0x38, -0x34 of the block.
    let set_param = |e: &mut Engine, offset: u32, value: f32| e.mem.set_f32(f.at(offset), value);
    if e.get(this, PlayerCharacter::eGrabType) == 3 {
        if big {
            let value = setting_float(e, SETTING_011D18A8);
            set_param(e, neg(0x40), value);
            let value = setting_float(e, SETTING_011D183C);
            set_param(e, neg(0x3c), value);
            let value = setting_float(e, SETTING_011D1728);
            set_param(e, neg(0x34), value);
            let value = setting_float(e, SETTING_011D1604);
            set_param(e, neg(0x38), value);
        } else {
            let value = setting_float(e, SETTING_011D1A08);
            set_param(e, neg(0x40), value);
            let value = setting_float(e, SETTING_011D1C7C);
            set_param(e, neg(0x3c), value);
            let value = setting_float(e, SETTING_011D1B44);
            set_param(e, neg(0x34), value);
            let value = setting_float(e, SETTING_011D177C);
            set_param(e, neg(0x38), value);
        }
    } else {
        let mut damping_scale = 1.0f32;
        let mut stiffness_scale = 1.0f32;
        let grabbed = e.get(this, PlayerCharacter::pGrabbedObject).addr();
        if e.vcall(grabbed, 0x100, &args![]).bool() {
            let a = setting_float(e, SETTING_011D1260);
            let b = setting_float(e, SETTING_011D0048);
            let c = setting_float(e, SETTING_011D0220);
            let d = setting_float(e, SETTING_011D0E4C);
            let weight = e.call(FUNCTION_0062C300, &args![node]).f32();
            let clamped = e.call(FUNCTION_00404010, &args![a, weight]).f32();
            let value = e.call(FUNCTION_0040EBD0, &args![b, clamped]).f32();
            let mapped = e.call(FUNCTION_004B3AB0, &args![d, c, b, a, value]).f32();
            stiffness_scale = (mapped as f64 * stiffness_scale as f64) as f32;
            let mut still_matches = true;
            let mut index: i32 = 0;
            loop {
                let table = fn_009604f0(e, Ptr::new(hit)).addr();
                let count = e.call(FUNCTION_0096A350, &args![table]).i32();
                if !(index < count) || !still_matches {
                    break;
                }
                let table = fn_009604f0(e, Ptr::new(hit)).addr();
                let element = e.call(FUNCTION_0096A370, &args![table, index as u32]).u32();
                let entry = e.mem.u32(element);
                let form = e.call(READ_DWORD_AT_C, &args![entry]).u32();
                still_matches = e.vcall(form, 0x20, &args![]).u32() == 0xb;
                index += 1;
            }
            if still_matches {
                let k: f64 = e.global(F64_CONSTANT_0101FFA0);
                damping_scale = (damping_scale as f64 * k) as f32;
                let k: f64 = e.global(F64_CONSTANT_01018A90);
                stiffness_scale = (stiffness_scale as f64 * k) as f32;
            }
        } else {
            let scratch = f.at(0xffff_fdb4);
            let kind_object = e.call(FUNCTION_0043B4F0, &args![body, scratch]).u32();
            if e.call(FUNCTION_0043B4D0, &args![kind_object]).i32() == 0xe {
                let k: f64 = e.global(F64_CONSTANT_0101FFA0);
                damping_scale = (damping_scale as f64 * k) as f32;
                let k: f64 = e.global(DOUBLE_HALF);
                stiffness_scale = (stiffness_scale as f64 * k) as f32;
            }
        }
        let value = setting_float(e, SETTING_011D0358);
        set_param(e, neg(0x40), value);
        let value = setting_float(e, SETTING_011CFEEC);
        set_param(e, neg(0x3c), (value as f64 * damping_scale as f64) as f32);
        let value = setting_float(e, SETTING_011D08C8);
        set_param(e, neg(0x34), value);
        let value = setting_float(e, SETTING_011D0FFC);
        set_param(e, neg(0x38), (value as f64 * stiffness_scale as f64) as f32);
    }

    // Anchors of the spring.
    let anchor_a = f.at(0xffff_fda0);
    let anchor_b = f.at(0xffff_fd90);
    e.call(IDENTITY_006815C0, &args![anchor_a]);
    e.call(IDENTITY_006815C0, &args![anchor_b]);
    e.call(FUNCTION_004B4D10, &args![vector_50, anchor_a]);
    let origin = e.call(FUNCTION_004B4EC0, &args![hit]).u32();
    fn_00961190(e, Ptr::new(anchor_a), Ptr::new(origin));
    let transform = e.call(FUNCTION_004B4F00, &args![hit]).u32();
    let transform = e.call(IDENTITY_006815C0, &args![transform]).u32();
    let produced = e.call(FUNCTION_004B4D10, &args![vector_60, anchor_b]).u32();
    e.call(FUNCTION_004B4AB0, &args![produced, transform, anchor_a]);
    e.call(FUNCTION_004B4DB0, &args![vector_60, anchor_b]);

    let grabbed = e.get(this, PlayerCharacter::pGrabbedObject);
    let loaded = e.call(READ_DWORD_AT_40, &args![grabbed]).u32();
    let cell = e.call(OBJECT_FUNCTION_004543C0, &args![loaded]).u32();
    e.call(OBJECT_FUNCTION_005533C0, &args![cell]);
    let block = e.call(ALLOCATE_00AA13E0, &args![0x10u32]).u32();
    let action = if block != 0 {
        e.call(MOUSE_SPRING_ACTION_NEW, &args![block, params]).u32()
    } else {
        0
    };
    let spring_slot = this.byte_add(0x634);
    e.call(SMART_POINTER_ASSIGN, &args![spring_slot, action]);
    let spring = e.call(READ_DWORD_AT_0, &args![spring_slot]).u32();
    let grabbed = e.get(this, PlayerCharacter::pGrabbedObject);
    let loaded = e.call(READ_DWORD_AT_40, &args![grabbed]).u32();
    let cell = e.call(OBJECT_FUNCTION_004543C0, &args![loaded]).u32();
    e.vcall(spring, 0x9c, &args![cell]);
    let grabbed = e.get(this, PlayerCharacter::pGrabbedObject);
    let loaded = e.call(READ_DWORD_AT_40, &args![grabbed]).u32();
    let cell = e.call(OBJECT_FUNCTION_004543C0, &args![loaded]).u32();
    e.call(OBJECT_FUNCTION_005533C0, &args![cell]);
    let node = e.vcall(obj, 0x1d0, &args![]).u32();
    let weight = e.call(FUNCTION_0062C300, &args![node]).f32();
    e.set(this, PlayerCharacter::fGrabObjectWeight, weight);
}

// Translated from 009604f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The address of the member at +0xac of `this` (an embedded array).
pub fn fn_009604f0(_e: &mut Engine, this: Ptr) -> Ptr {
    this.byte_add(0xac)
}

// Translated from 00960510 (decompiled, FalloutNV.exe 1.4.0.525)
/// The dword the global at `011cc5ec` holds.
pub fn fn_00960510(e: &mut Engine) -> u32 {
    e.global(GLOBAL_011CC5EC)
}

// Translated from 00961190 (decompiled, FalloutNV.exe 1.4.0.525)
/// Subtracts the four floats at `other` from the four at `this`
/// (`SUBPS`, an `hkVector4` in place).
pub fn fn_00961190(e: &mut Engine, this: Ptr, other: Ptr) {
    for lane in 0..4 {
        let offset = lane * 4;
        let value = e.mem.f32(this.addr() + offset) - e.mem.f32(other.addr() + offset);
        e.mem.set_f32(this.addr() + offset, value);
    }
}

// Translated from 009611e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The dword at +0x18 of `this`.
pub fn fn_009611e0(e: &mut Engine, this: Ptr) -> u32 {
    e.mem.u32(this.addr() + 0x18)
}

// Translated from 00961200 (decompiled, FalloutNV.exe 1.4.0.525)
/// When `004ae750` finds something for `this`, calls `004a3f10(target,
/// found + 0x30)`.
pub fn fn_00961200(e: &mut Engine, this: Ptr, target: Ptr) {
    let found = e.call(BODY_LOOKUP_004AE750, &args![this]).u32();
    if found != 0 {
        e.call(FUNCTION_004A3F10, &args![target, found + 0x30]);
    }
}

// Translated from 00961230 (decompiled, FalloutNV.exe 1.4.0.525)
/// Like [`fn_00961200`] with the member at +0x20 of what `004ae750` finds.
pub fn fn_00961230(e: &mut Engine, this: Ptr, target: Ptr) {
    let found = e.call(BODY_LOOKUP_004AE750, &args![this]).u32();
    if found != 0 {
        e.call(FUNCTION_004A3F10, &args![target, found + 0x20]);
    }
}

// Translated from 00961260 (decompiled, FalloutNV.exe 1.4.0.525)
/// The dword at +0x608 of `this`.
pub fn fn_00961260(e: &mut Engine, this: Ptr) -> u32 {
    e.mem.u32(this.addr() + 0x608)
}

// Translated from 00961280 (decompiled, FalloutNV.exe 1.4.0.525)
/// Releases the grab: when a grab spring exists, asks the spring (virtual at
/// +0xa0) to stop after the cell data of the player's loaded object
/// (`008d6f30`, `004543c0`, `005533c0` around it) and calls `00c9c1d0` on
/// the spring's entity; then removes the player action 5 (`00963e00`),
/// drops the spring pointer (`0066b0d0`), clears the grab type and weight,
/// and, when an object was grabbed whose virtual +0xf0 answers true, calls
/// `005ac750` with the flag 0x100000 on its extra data. Clears the grabbed
/// object. Returns `this`.
pub fn fn_00961280(e: &mut Engine, this: Ptr<PlayerCharacter>) -> Ptr<PlayerCharacter> {
    let spring_slot = this.byte_add(0x634);
    if e.call(READ_DWORD_AT_0, &args![spring_slot]).u32() != 0 {
        let loaded = e.call(READ_DWORD_AT_40, &args![this]).u32();
        let cell = if loaded != 0 {
            e.call(OBJECT_FUNCTION_004543C0, &args![loaded]).u32()
        } else {
            0
        };
        let spring = e.call(READ_DWORD_AT_0, &args![spring_slot]).u32();
        e.vcall(spring, 0xa0, &args![]);
        if cell != 0 {
            e.call(OBJECT_FUNCTION_005533C0, &args![cell]);
        }
        let spring = e.call(READ_DWORD_AT_0, &args![spring_slot]).u32();
        let entity = e.call(FUNCTION_0082AEE0, &args![spring]).u32();
        if entity != 0 {
            e.call(HKP_ENTITY_ACTIVATE, &args![entity]);
        }
        if cell != 0 {
            e.call(OBJECT_FUNCTION_005533C0, &args![cell]);
        }
    }
    e.call(PLAYER_REMOVE_PLAYER_ACTION, &args![this, 5u32, 0u32]);
    e.call(SMART_POINTER_ASSIGN, &args![spring_slot, 0u32]);
    e.set(this, PlayerCharacter::eGrabType, 0);
    e.set(this, PlayerCharacter::fGrabObjectWeight, 0.0);
    let grabbed = e.get(this, PlayerCharacter::pGrabbedObject);
    if !grabbed.is_null() {
        let grabbed = e.get(this, PlayerCharacter::pGrabbedObject);
        if e.vcall(grabbed.addr(), 0xf0, &args![]).bool() {
            let grabbed = e.get(this, PlayerCharacter::pGrabbedObject);
            let extra = e.call(ADD_44_TO_ADDRESS, &args![grabbed]).u32();
            e.call(SCRIPT_SET_ACTION_FLAG, &args![0u32, extra, 0x10_0000u32]);
        }
    }
    e.set(this, PlayerCharacter::pGrabbedObject, Ptr::NULL);
    this
}

// Translated from 009613c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PlayerCharacter::UpdateDropSpring` (Xbox PDB): only acts for grab type 2
/// (a dropped object held by a spring). Without a spring it creates one on
/// the grabbed object (`CreateMouseSpring` with the distance
/// `scale * setting 011d0628`); with one it releases it
/// ([`fn_00961280`]) unless control 4 is in state 1 or 0 and not in state
/// 2, in which case it updates it ([`fn_00960520`]). Returns whether the
/// grab type is still 2.
pub fn player_character_update_drop_spring(
    e: &mut Engine,
    this: Ptr<PlayerCharacter>,
    delta: f32,
) -> bool {
    if e.get(this, PlayerCharacter::eGrabType) == 2 {
        let owner = e.global::<u32>(POINTER_011DEA0C);
        let controls = e.call(FUNCTION_00877720, &args![owner]).u32();
        let spring_slot = this.byte_add(0x634);
        if e.call(READ_DWORD_AT_0, &args![spring_slot]).u32() == 0 {
            let scale = e.call(OBJECT_GET_SCALE, &args![this]).f32();
            let factor = setting_float(e, SETTING_011D0628);
            let distance = (factor as f64 * scale as f64) as f32;
            let grabbed = e.get(this, PlayerCharacter::pGrabbedObject);
            player_character_create_mouse_spring(e, this, grabbed.cast(), 2, distance);
        }
        if e.call(READ_DWORD_AT_0, &args![spring_slot]).u32() != 0 {
            let query = |e: &mut Engine, state: u32| {
                e.call(CONTROL_QUERY, &args![controls, 4u32, state]).u32() != 0
            };
            if query(e, 2) || (!query(e, 1) && !query(e, 0)) {
                fn_00961280(e, this);
            } else {
                fn_00960520(e, this, delta);
            }
        }
    }
    e.get(this, PlayerCharacter::eGrabType) == 2
}

// Translated from 00961c40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Applies `intensity` to the player's two 3D roots (`00950bb0` for
/// first person and third person) through `00b68770`, tells the extra data
/// (`00422750`) and the Pipboy (`007fa8d0`). With `enable` set and a
/// non-positive intensity the intensity comes from the process's virtual
/// +0x5b4; a non-positive intensity turns the flag off for the 3D roots.
pub fn fn_00961c40(e: &mut Engine, this: Ptr<PlayerCharacter>, enable: u8, intensity: f32) {
    let zero: f64 = e.global(ZERO_DOUBLE);
    let mut value = intensity;
    if enable != 0 && value as f64 <= zero {
        let process = e.call(PLAYER_GET_PROCESS, &args![this]).u32();
        if process != 0 {
            let process = e.call(PLAYER_GET_PROCESS, &args![this]).u32();
            value = e.vcall(process, 0x5b4, &args![]).f32();
        }
    }
    let mut flag = enable;
    if flag != 0 && value as f64 <= zero {
        flag = 0;
    }
    for first_person in [1u32, 0u32] {
        let root = e
            .call(PLAYER_GET_ROOT_NODE, &args![this, first_person])
            .u32();
        e.call(
            FUNCTION_00B68770,
            &args![root, u32::from(flag), value, 0u32, 0.0f32, 1u32],
        );
    }
    let extra = e.call(ADD_44_TO_ADDRESS, &args![this]).u32();
    if extra != 0 {
        e.call(FUNCTION_00422750, &args![extra, u32::from(flag), intensity]);
    }
    let pipboy = e.call(INTERFACE_GET_PIPBOY, &args![]).u32();
    if pipboy != 0 {
        e.call(FUNCTION_007FA8D0, &args![pipboy, u32::from(enable)]);
    }
}

// Translated from 00961d50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PlayerCharacter::DeathMenu` (Xbox PDB): clears the flag at `011e07c0`,
/// then loads the most recent save game when the message menu result
/// (`00703fa0`) is 1, or chooses the main menu when it is 2.
pub fn player_character_death_menu(e: &mut Engine) {
    e.mem.set_u8(FLAG_011E07C0, 0);
    let result = e.call(GET_MESSAGE_MENU_RESULT, &args![]).u8() as i8;
    if result == 1 {
        let manager = e.global::<u32>(POINTER_011DE134);
        e.call(LOAD_MOST_RECENT_SAVE_GAME, &args![manager]);
    } else if result == 2 {
        e.call(CHOOSE_MAIN_MENU, &args![]);
    }
}

// Translated from 00961d90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stops vanity mode (`009500a0`), then with `save` set stores the
/// player's `fSitHeadingDelta` into the global at `011e0d60`, else restores
/// it from there, and calls `00933890(this, save)`.
pub fn fn_00961d90(e: &mut Engine, this: Ptr<PlayerCharacter>, save: u8) {
    e.call(PLAYER_STOP_VANITY_MODE, &args![this]);
    if save != 0 {
        let value = e.get(this, PlayerCharacter::fSitHeadingDelta);
        e.set_global(F32_CONSTANT_011E0D60, value);
    } else {
        let value: f32 = e.global(F32_CONSTANT_011E0D60);
        e.set(this, PlayerCharacter::fSitHeadingDelta, value);
    }
    e.call(FUNCTION_00933890, &args![this, u32::from(save)]);
}

// Callees of the functions from `00961de0` (named by what the call sites
// show; the engine map has no better name for most of them).
/// `Actor::GetCurrentWeapon` (Xbox PDB).
const ACTOR_GET_CURRENT_WEAPON: u32 = 0x008a_1710;
/// `Actor::GetAnimation` (Xbox PDB).
const ACTOR_GET_ANIMATION: u32 = 0x008b_70d0;
/// `Actor::GetHeight` (Xbox PDB): the height in `ST0`.
const ACTOR_GET_HEIGHT: u32 = 0x0088_53a0;
/// Adds 0x24 to the address (`ECX`) and returns it.
const ADD_24_TO_ADDRESS: u32 = 0x0043_0830;
/// `ExtraDataList::GetContainerChanges` (Xbox PDB).
const EXTRA_LIST_GET_CONTAINER_CHANGES: u32 = 0x0041_8520;
/// `bhkWorld::Activate` (Xbox PDB), cdecl `(3D, 1, 1, 0)`.
const BHK_WORLD_ACTIVATE: u32 = 0x00c6_a270;
/// `bhkWorld::SetMotion` (Xbox PDB), cdecl `(3D, 1, 1, 1, 1)`.
const BHK_WORLD_SET_MOTION: u32 = 0x00c6_a350;
/// `bhkCharacterController::CheckInsideOfObject` (Xbox PDB).
const CHECK_INSIDE_OF_OBJECT: u32 = 0x00c6_fa20;
/// `BSShaderUtil::RecursiveSetPropertyFadeAlpha` (Xbox PDB), cdecl
/// `(root, alpha)`.
const RECURSIVE_SET_PROPERTY_FADE_ALPHA: u32 = 0x00b6_bb30;
/// `NiMatrix3::FromEulerAnglesXYZ` (Xbox PDB) `(matrix, x, y, z)`.
const MATRIX_FROM_EULER: u32 = 0x00a5_9540;
/// `NiMatrix3::ToEulerAnglesXYZ` (Xbox PDB) `(matrix, &x, &y, &z)`.
const MATRIX_TO_EULER: u32 = 0x00a5_92c0;
/// `NiMatrix3` product (map name `NiMatrix3::operatorP`) `(this, out, other)`,
/// returns `out`.
const MATRIX_MULTIPLY: u32 = 0x0043_f8d0;
/// `NiPoint3` sum `(this, out, other)`, returns `out`.
const VECTOR_ADD: u32 = 0x0043_9e90;
/// `NiPoint3::Dot` (Xbox PDB) `(this, other)`, the result in `ST0`.
const VECTOR_DOT: u32 = 0x004b_6190;
/// `NiPoint3` constructor `(this, x, y, z)`, returns `this`.
const NI_POINT3_NEW: u32 = 0x0041_6870;
/// `NiPick::PickObjects` (Xbox PDB) `(picker, origin, direction, 0)`.
const PICK_OBJECTS: u32 = 0x00e9_8e20;
/// Clears the pick results `(picker, 0)` (a jump into `00e98cb0` with
/// `picker + 0x18`).
const PICKER_CLEAR: u32 = 0x00e9_8e10;
/// The results member of the picker: `picker + 0x18`.
const PICKER_RESULTS: u32 = 0x0050_0940;
/// Result `(results, 0)` of the pick results.
const PICK_RESULT_GET: u32 = 0x0096_8670;
/// The normal of a pick result: `result + 0x28`.
const PICK_RESULT_NORMAL: u32 = 0x0046_10d0;
/// `NiRTTI` style dynamic cast (cdecl `(target type, object)`, null for a
/// null object).
const NI_OBJECT_CAST: u32 = 0x0065_3270;
/// Finds a child node by name, cdecl `(root, name)`.
const FIND_NODE_BY_NAME: u32 = 0x004a_ae30;
/// Float at `+0xd0` of the object (`ST0`).
const READ_FLOAT_AT_D0: u32 = 0x0045_3700;
/// `(weapon)`: true for the weapon types 0 to 2 (the byte at `+0xf4`).
const WEAPON_IS_MELEE_TYPE: u32 = 0x0064_50c0;
/// The `gunwobble.cpp` function `(table index, animation float, out
/// matrix)`: writes the wobble matrix and returns the node name for the
/// index.
const GUN_WOBBLE_GET_MATRIX: u32 = 0x008d_6970;
/// `PlayerCharacter::IsPlayerCharacterInCombat` (Xbox PDB) `(player, &out
/// byte)`.
const PLAYER_IS_IN_COMBAT: u32 = 0x0095_3c50;
/// `TESObjectREFR::SetTargeted` (Xbox PDB) `(reference, flag)`.
const TES_OBJECT_REFR_SET_TARGETED: u32 = 0x0056_4db0;
/// `00971c30` on the process lists: `(player, 0x12, 0)` returns a list.
const FUNCTION_00971C30: u32 = 0x0097_1c30;
/// Process lists object (`this` of [`FUNCTION_00971C30`]).
const PROCESS_LISTS: u32 = 0x011e_0e80;

// `BSSoundHandle` (Xbox PDB) functions, 0xc bytes each.
const SOUND_HANDLE_SIZE: u32 = 0xc;
/// `BSSoundHandle::IsValid` (Xbox PDB).
const SOUND_HANDLE_IS_VALID: u32 = 0x00ad_8ce0;
/// `BSSoundHandle::Stop` (Xbox PDB).
const SOUND_HANDLE_STOP: u32 = 0x00ad_88f0;
/// `BSSoundHandle::SetPosition` (Xbox PDB) `(handle, x, y, z)`.
const SOUND_HANDLE_SET_POSITION: u32 = 0x00ad_8b60;
/// `BSSoundHandle::SetObjectToFollow` (Xbox PDB).
const SOUND_HANDLE_SET_OBJECT_TO_FOLLOW: u32 = 0x00ad_8f20;
/// `BSSoundHandle::Play` (Xbox PDB).
const SOUND_HANDLE_PLAY: u32 = 0x00ad_8830;
/// Handle constructor (id -1, byte 0, word 0).
const SOUND_HANDLE_NEW: u32 = 0x0041_a250;
/// Handle copy `(this, source)`.
const SOUND_HANDLE_COPY: u32 = 0x0041_8900;
/// A destructor that does nothing.
const EMPTY_DESTRUCTOR: u32 = 0x0048_3710;

// The picker of `fn_00962950`.
/// Global holding the picker pointer.
const PICKER_POINTER: u32 = 0x011e_0d64;
/// The picker object the global is first set to.
const PICKER_OBJECT: u32 = 0x011e_0aa8;
/// The list at `+0x40` of the picker (`fn_00962cd0`, `fn_00962d00`).
const PICKER_LIST: u32 = 0x011e_0ae8;
/// `(object, key)` argument of `0045bad0` after a hit.
const PICK_FILTER: u32 = 0x011f_4aa0;
/// Floats used as ray components.
const PICK_DISTANCE_0104E0E8: u32 = 0x0104_e0e8;
const PICK_DISTANCE_0108B3F0: u32 = 0x0108_b3f0;
/// Double multiplied with the player's height.
const HEIGHT_SCALE_010290B0: u32 = 0x0102_90b0;

// Gun wobble.
/// Table of the wobble index per weapon type (dwords).
const WOBBLE_INDEX_TABLE: u32 = 0x0118_a838;
/// The identity matrix (9 floats).
const IDENTITY_MATRIX: u32 = 0x011a_9448;
/// The first person wobble rotation (a matrix of 9 floats).
const WOBBLE_RESULT_MATRIX: u32 = 0x011e_09ec;
/// The current wobble scale.
const WOBBLE_SCALE_GLOBAL: u32 = 0x011a_3b2c;
/// Settings read with [`SETTING_FLOAT`].
const WOBBLE_SCALE_SETTING: u32 = 0x011c_ec34;
const WOBBLE_STEP_SETTING: u32 = 0x011c_f588;
const FIRST_PERSON_SCALE_SETTING: u32 = 0x011c_f718;
const AIM_SMOOTHING_SETTING: u32 = 0x011c_e960;
/// The object `0084d030` reads the frame time from.
const FRAME_TIME_OBJECT: u32 = 0x011f_6394;
/// Aim offsets moved towards the values the player computes.
const AIM_GLOBAL_X: u32 = 0x011e_0d6c;
const AIM_GLOBAL_Y: u32 = 0x011e_0d68;
/// Float passed twice to `00965620`.
const AIM_LIMIT_0102EFC4: u32 = 0x0102_efc4;
/// Name strings (addresses).
const AIM_NODE_NAME: u32 = 0x0102_cb10;
const DEFAULT_WOBBLE_NODE_NAME: u32 = 0x0102_0594;
/// Run-time type descriptors the wobble code casts to.
const TYPE_COLLISION_BASE: u32 = 0x0126_7e64;
const TYPE_BLEND_COLLISION: u32 = 0x0126_81cc;

// Other constants.
/// Float stored by `fn_00963b00`.
const RESET_VALUE_01012054: u32 = 0x0101_2054;
/// Float given to the message of `fn_009627a0`.
const MESSAGE_DURATION_010162C0: u32 = 0x0101_62c0;
/// Setting whose string is the message of `fn_009627a0`.
const SETTING_011D3048: u32 = 0x011d_3048;
/// Comparison function address `007a7eb0` receives for perk entries.
const PERK_ENTRY_COMPARE: u32 = 0x005e_b550;

// Callees named by their call sites.
/// `(this, alpha)`: the fade alpha setter run before the shader update.
const FUNCTION_008C4790: u32 = 0x008c_4790;
/// Byte at `+6` of the `Main` object.
const FUNCTION_005BB4D0: u32 = 0x005b_b4d0;
/// Returns the interface manager object (0 when it is not ready).
const FUNCTION_00705950: u32 = 0x0070_5950;
/// Sound form for a magic failure sound index (one of six globals, 0 above
/// 5), cdecl `(index)`.
const FUNCTION_0040DEE0: u32 = 0x0040_dee0;
/// Creates the sound `(mobile object, out handle, key, play flag, flags,
/// 1)` and returns `out`.
const FUNCTION_00933150: u32 = 0x0093_3150;
/// `(extra list)`: the extra data object whose `+0xc` list the walkers read.
const FUNCTION_00422700: u32 = 0x0042_2700;
/// `(actor)`: its current package (`MobileObject::GetCurrentPackage`).
const FUNCTION_009344A0: u32 = 0x0093_44a0;
/// `(package)`: package type.
const FUNCTION_0041CA90: u32 = 0x0041_ca90;
/// `(extra list)`: object the player light code tests.
const FUNCTION_00418250: u32 = 0x0041_8250;
/// `(object, light slot)`: true when the light is set.
const FUNCTION_004B0460: u32 = 0x004b_0460;
/// `(3D root)`: true for a usable root.
const FUNCTION_00456610: u32 = 0x0045_6610;
/// `(object, light slot)`: assigns the light.
const FUNCTION_006E5CC0: u32 = 0x006e_5cc0;
/// `(player)`: refresh after the light choice.
const FUNCTION_0088B4E0: u32 = 0x0088_b4e0;
/// `()`: count tested by `fn_00962590`.
const FUNCTION_00570F60: u32 = 0x0057_0f60;
/// `(form id, 0x7fffffff, player, 1, 1)`, cdecl.
const FUNCTION_008CE180: u32 = 0x008c_e180;
/// `(camera caster, value)`.
const FUNCTION_00620BA0: u32 = 0x0062_0ba0;
/// `(character controller)`: value for the camera caster.
const FUNCTION_00819250: u32 = 0x0081_9250;
/// `(reference, 0)`.
const FUNCTION_00954910: u32 = 0x0095_4910;
/// `(map, key, &out)`: looks a key up in the `NiTMap` of random door spaces.
const FUNCTION_0057C850: u32 = 0x0057_c850;
/// `(map, key, value)`: stores a value in the `NiTMap`.
const FUNCTION_0084D310: u32 = 0x0084_d310;
/// `(float, 3D)` cdecl: image space modifier for a hit distance.
const FUNCTION_005D2860: u32 = 0x005d_2860;
/// `(modifier)` cdecl: triggers an image space modifier.
const FUNCTION_005299A0: u32 = 0x0052_99a0;
/// `(float)` cdecl: the distance returned by `00648a80`.
const FLOAT_FUNCTION_00648A80: u32 = 0x0064_8a80;
/// `(message text, 0, 0, 0, float, 0)`, cdecl: shows a message.
const FUNCTION_007052F0: u32 = 0x0070_52f0;
/// `(picker, byte)`: sets the byte at `+0x10`.
const FUNCTION_00632D20: u32 = 0x0063_2d20;
/// `(picker, byte)`: sets the byte at `+0x11`.
const FUNCTION_00458B30: u32 = 0x0045_8b30;
/// `(picker, root)`: gives the picker its root.
const FUNCTION_00705FC0: u32 = 0x0070_5fc0;
/// `(hit)` then `009611e0`: the object a pick hit belongs to.
const FUNCTION_00458B50: u32 = 0x0045_8b50;
const FUNCTION_009611E0: u32 = 0x0096_11e0;
/// `(filter, key)`, cdecl.
const FUNCTION_0045BAD0: u32 = 0x0045_bad0;
/// `(3D, 0)`.
const FUNCTION_00450F90: u32 = 0x0045_0f90;
/// `(3D, 1, 1, 0)`, cdecl.
const FUNCTION_00C6A0B0: u32 = 0x00c6_a0b0;
/// `(perk, player, old rank, new rank, companion)`.
const FUNCTION_005EB6A0: u32 = 0x005e_b6a0;
/// `(perk, player, companion)`.
const FUNCTION_005EB800: u32 = 0x005e_b800;
/// The stats menu update.
const FUNCTION_007DD710: u32 = 0x007d_d710;
/// `(player)`: refresh after a perk change.
const FUNCTION_008C17C0: u32 = 0x008c_17c0;
/// `(entry form)`: the perk entry type.
const FUNCTION_0062F2F0: u32 = 0x0062_f2f0;
/// `(list, form, comparison)`: inserts a perk entry.
const FUNCTION_007A7EB0: u32 = 0x007a_7eb0;
/// `(list)`: initializes an empty `BSSimpleList`.
const FUNCTION_0096A2D0: u32 = 0x0096_a2d0;
/// `(entry)`: constructs a 0xc byte player action.
const FUNCTION_0078D900: u32 = 0x0078_d900;
/// `(mobile object)`: the 3D root the wobble nodes are found in.
const FUNCTION_0043FCD0: u32 = 0x0043_fcd0;
/// `(player, 2 or 0)`: wobble scale (`ST0`).
const FUNCTION_008B0DD0: u32 = 0x008b_0dd0;
/// `(player)`: true when the value of `008a7570` is 2 to 6.
const FUNCTION_00894900: u32 = 0x0089_4900;
/// `(wobble node)`: its collision object.
const FUNCTION_006838B0: u32 = 0x0068_38b0;
/// `(node, collision object)`.
const FUNCTION_0062BC90: u32 = 0x0062_bc90;
/// `(block)`: constructs a blend collision object.
const FUNCTION_00C8FFD0: u32 = 0x00c8_ffd0;
/// `(new, source)`: copies a blend collision object.
const FUNCTION_00C90090: u32 = 0x00c9_0090;
/// `(collision object, matrix)`: stores the rotation.
const FUNCTION_004F0110: u32 = 0x004f_0110;
/// `(weapon)`: address of the weapon part `0048cee0` tests.
const FUNCTION_00504E60: u32 = 0x0050_4e60;
const FUNCTION_0048CEE0: u32 = 0x0048_cee0;
/// `(weapon)`: tests flag `0x2000` of the weapon.
const FUNCTION_004AD030: u32 = 0x004a_d030;
/// `(player, value)`: adds to the character's angle.
const FUNCTION_00931E50: u32 = 0x0093_1e50;
const FUNCTION_00931D30: u32 = 0x0093_1d30;
/// `PlayerCharacter` aim offsets `(player, 0, node, &x, &y, 0, limit,
/// limit)`.
const FUNCTION_00965620: u32 = 0x0096_5620;

// Translated from 00961de0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the fade alpha of the player: `008c4790(this, alpha)`, then, when the
/// first person 3D root exists (`00950bb0(this, 1)`), applies the alpha to
/// every property under it (`BSShaderUtil::RecursiveSetPropertyFadeAlpha`,
/// Xbox PDB).
pub fn fn_00961de0(e: &mut Engine, this: Ptr<PlayerCharacter>, alpha: f32) {
    e.call(FUNCTION_008C4790, &args![this, alpha]);
    let root = e.call(PLAYER_GET_ROOT_NODE, &args![this, 1u32]).u32();
    if root != 0 {
        e.call(RECURSIVE_SET_PROPERTY_FADE_ALPHA, &args![root, alpha]);
    }
}

// Translated from 00961e30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Toggles the UFO camera: the new state is the negation of the flag byte at
/// `+6` of the object in `011dea0c` (read through `005bb4d0`). It is stored
/// back (`00961f70`), stored at `+7` too when `enable` is also set
/// (`00961f50`) and mirrored on the byte at `+0x54` of the object
/// `00705950` returns (`00961f30`). When turned on, the camera position
/// starts at the player's position (virtual `+0x1f4`) raised by the player's
/// scale times `fEyeHeight`, and the heading and pitch are taken from the
/// two floats of the extra data at `this + 0x24`. Returns the new state.
pub fn fn_00961e30(e: &mut Engine, this: Ptr<PlayerCharacter>, enable: u8) -> u8 {
    let main = Ptr::<()>::new(e.global::<u32>(POINTER_011DEA0C));
    let was_on = e.call(FUNCTION_005BB4D0, &args![main]).u8();
    let on = u8::from(was_on == 0);
    fn_00961f70(e, main, on);
    fn_00961f50(e, main, u8::from(on != 0 && enable != 0));
    let manager = e.call(FUNCTION_00705950, &args![]).u32();
    fn_00961f30(e, Ptr::new(manager), on);
    if on != 0 {
        let position = e.vcall(this.addr(), 0x1f4, &args![]).u32();
        let camera = this.addr() + 0x7e8;
        for word in 0..3 {
            let value = e.mem.u32(position + word * 4);
            e.mem.set_u32(camera + word * 4, value);
        }
        let scale = e.call(OBJECT_GET_SCALE, &args![this]).f64();
        let eye_height = f64::from(e.get(this, PlayerCharacter::fEyeHeight));
        let height = f64::from(e.mem.f32(camera + 8));
        e.mem
            .set_f32(camera + 8, (scale * eye_height + height) as f32);
        let extra = e.call(ADD_24_TO_ADDRESS, &args![this]).u32();
        let heading = e.mem.f32(extra + 8);
        e.set(this, PlayerCharacter::fUFOCameraHeading, heading);
        let pitch = e.mem.f32(extra);
        e.set(this, PlayerCharacter::fUFOCameraPitch, pitch);
    }
    on
}

// Translated from 00961f30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores `value` in the byte at `+0x54` of the object.
pub fn fn_00961f30(e: &mut Engine, this: Ptr, value: u8) {
    e.mem.set_u8(this.addr() + 0x54, value);
}

// Translated from 00961f50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores `value` in the byte at `+7` of the object.
pub fn fn_00961f50(e: &mut Engine, this: Ptr, value: u8) {
    e.mem.set_u8(this.addr() + 7, value);
}

// Translated from 00961f70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores `value` in the byte at `+6` of the object.
pub fn fn_00961f70(e: &mut Engine, this: Ptr, value: u8) {
    e.mem.set_u8(this.addr() + 6, value);
}

// Translated from 00961f90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PlayerCharacter::ResetMagicCastSound` (Xbox PDB): when the cast sound
/// handle at `+0x7f8` is valid, overwrites it with a fresh invalid handle
/// (`0041a250`, copy `00418900`) and clears the sound id at `+0x7f4`. The
/// exception frame is not translated.
pub fn player_character_reset_magic_cast_sound(e: &mut Engine, this: Ptr<PlayerCharacter>) {
    let handle = this.addr() + 0x7f8;
    if e.call(SOUND_HANDLE_IS_VALID, &args![handle]).bool() {
        e.with_stack(SOUND_HANDLE_SIZE, |e, fresh| {
            e.call(SOUND_HANDLE_NEW, &args![fresh]);
            e.call(SOUND_HANDLE_COPY, &args![handle, fresh]);
            e.call(EMPTY_DESTRUCTOR, &args![fresh]);
        });
        e.set(this, PlayerCharacter::iSelectedSpellCastSoundID, 0);
    }
}

// Translated from 00962030 (decompiled, FalloutNV.exe 1.4.0.525)
/// Plays magic failure sound `index` (the handles are an array of 0xc-byte
/// `BSSoundHandle` at `+0x804`). A valid handle is stopped, moved to the
/// player's position (virtual `+0x1f4`), made to follow the player's 3D
/// (virtual `+0x1d0`) and played again. Otherwise the sound form for the
/// index (`0040dee0`, one of six globals, 0 above 5) gives, through its
/// word at `+0xc`, the sound `00933150` creates (play flag 0, flags 2, 1),
/// which is copied into the handle. The exception frame is not translated.
pub fn fn_00962030(e: &mut Engine, this: Ptr<PlayerCharacter>, index: u32) {
    let handle = this
        .addr()
        .wrapping_add(index.wrapping_mul(SOUND_HANDLE_SIZE))
        .wrapping_add(0x804);
    if e.call(SOUND_HANDLE_IS_VALID, &args![handle]).bool() {
        e.call(SOUND_HANDLE_STOP, &args![handle]);
        let position = e.vcall(this.addr(), 0x1f4, &args![]).u32();
        let x = e.mem.u32(position);
        let y = e.mem.u32(position + 4);
        let z = e.mem.u32(position + 8);
        e.call(SOUND_HANDLE_SET_POSITION, &args![handle, x, y, z]);
        let follow = e.vcall(this.addr(), 0x1d0, &args![]).u32();
        e.call(SOUND_HANDLE_SET_OBJECT_TO_FOLLOW, &args![handle, follow]);
        e.call(SOUND_HANDLE_PLAY, &args![handle, 0u32]);
    } else {
        let sound_form = e.call(FUNCTION_0040DEE0, &args![index]).u32();
        if sound_form != 0 {
            let key = e.call(READ_DWORD_AT_C, &args![sound_form]).u32();
            e.with_stack(SOUND_HANDLE_SIZE, |e, out| {
                let created = e
                    .call(FUNCTION_00933150, &args![this, out, key, 0u32, 2u32, 1u32])
                    .u32();
                e.call(SOUND_HANDLE_COPY, &args![handle, created]);
                e.call(EMPTY_DESTRUCTOR, &args![out]);
            });
        }
    }
}

// Translated from 00962190 (decompiled, FalloutNV.exe 1.4.0.525)
/// Recomputes the player's `bBeingChased` flag (`+0x7c4`): clears it, then
/// walks the list `00971c30` returns on the process lists object `011e0e80`
/// (player, 0x12, 0) and sets it when an item answers true to its virtual
/// `+0x100` and its virtual `+0x304` is true. The list is cleared and deleted
/// afterwards.
pub fn fn_00962190(e: &mut Engine, _this: Ptr<PlayerCharacter>) {
    let player = Ptr::<PlayerCharacter>::new(e.global::<u32>(PLAYER_CHARACTER));
    let list = e
        .call(
            FUNCTION_00971C30,
            &args![PROCESS_LISTS, player, 0x12u32, 0u32],
        )
        .u32();
    e.set(player, PlayerCharacter::bBeingChased, 0);
    let mut node = list;
    while node != 0 {
        let mut candidate = 0;
        let slot = e.call(LIST_NODE_SLOT, &args![node]).u32();
        let item = e.mem.u32(slot);
        if item != 0 && e.vcall(item, 0x100, &args![]).bool() {
            let slot = e.call(LIST_NODE_SLOT, &args![node]).u32();
            candidate = e.mem.u32(slot);
        }
        if candidate != 0 && e.vcall(candidate, 0x304, &args![]).bool() {
            e.set(player, PlayerCharacter::bBeingChased, 1);
            break;
        }
        node = e.call(LIST_NODE_NEXT, &args![node]).u32();
    }
    if list != 0 {
        e.call(LIST_CLEAR_ITEMS, &args![list]);
        e.call(LIST_DELETE, &args![list, 1u32]);
    }
}

// Translated from 00962290 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PlayerCharacter::PlayImpactBasedHitShader` (Xbox PDB): for a `reference`
/// with a 3D (virtual `+0x1d0`), takes the distance to it (`00439ef0`
/// subtraction, `00457990` length, `00648a80`) and, when positive and the
/// object `011f2250` has no value at `+8`, triggers the image space modifier
/// `005d2860(distance, 3D)` returns (`005299a0`).
pub fn player_character_play_impact_based_hit_shader(
    e: &mut Engine,
    this: Ptr<PlayerCharacter>,
    reference: Ptr,
) {
    if reference.addr() == 0 || e.vcall(reference.addr(), 0x1d0, &args![]).u32() == 0 {
        return;
    }
    let own_position = e.vcall(this.addr(), 0x1f4, &args![]).u32();
    let other_position = e.vcall(reference.addr(), 0x1f4, &args![]).u32();
    let distance = e.with_stack(12, |e, offset| {
        e.call(
            VECTOR_SUBTRACT,
            &args![other_position, offset, own_position],
        );
        let length = e.call(VECTOR_LENGTH, &args![offset]).f32();
        e.call(FLOAT_FUNCTION_00648A80, &args![length]).f32()
    });
    let zero: f64 = e.global(ZERO_DOUBLE);
    if f64::from(distance).partial_cmp(&zero) != Some(std::cmp::Ordering::Greater) {
        return;
    }
    if e.call(READ_FIELD_8, &args![OBJECT_011F2250]).u32() != 0 {
        return;
    }
    let node = e.vcall(reference.addr(), 0x1d0, &args![]).u32();
    // 00962331 / 00962337: the cdecl `005d2860` reads the two pushed words without popping
    // them; they are the last two words of the cdecl `005299a0` (`(form, strength,
    // target)`, one ADD ESP,0xc after both).
    let modifier = e.call(FUNCTION_005D2860, &args![distance, node]).u32();
    e.call(FUNCTION_005299A0, &args![modifier, distance, node]);
}

// Translated from 00962350 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PlayerCharacter::HavokActivateDroppedReference` (Xbox PDB): for every
/// reference in the `DroppedRefList` (`+0x84c`) that has a 3D (virtual
/// `+0x1d0`), sets its world motion (`00c6a350(3D, 1, 1, 1, 1)`), activates
/// it (`00c6a270(3D, 1, 1, 0)`), removes it from the list (`00905330`) and
/// calls `00954910(reference, 0)`. After a removal the walk goes on from the
/// node after the last node that was kept.
pub fn player_character_havok_activate_dropped_reference(
    e: &mut Engine,
    this: Ptr<PlayerCharacter>,
) {
    let list = this.addr() + 0x84c;
    if e.call(LIST_NODE_IS_EMPTY, &args![list]).bool() {
        return;
    }
    let mut previous = list;
    let mut node = list;
    while node != 0 {
        if e.call(LIST_NODE_IS_EMPTY, &args![node]).bool() {
            break;
        }
        let slot = e.call(LIST_NODE_SLOT, &args![node]).u32();
        let reference = e.mem.u32(slot);
        if reference != 0 && e.vcall(reference, 0x1d0, &args![]).u32() != 0 {
            let root = e.vcall(reference, 0x1d0, &args![]).u32();
            e.call(BHK_WORLD_SET_MOTION, &args![root, 1u32, 1u32, 1u32, 1u32]);
            let root = e.vcall(reference, 0x1d0, &args![]).u32();
            e.call(BHK_WORLD_ACTIVATE, &args![root, 1u32, 1u32, 0u32]);
            e.with_stack(4, |e, cell| {
                e.mem.set_u32(cell.addr(), reference);
                e.call(LIST_REMOVE, &args![list, cell]);
            });
            e.call(FUNCTION_00954910, &args![reference, 0u32]);
            node = e.call(LIST_NODE_NEXT, &args![previous]).u32();
        } else {
            previous = node;
            node = e.call(LIST_NODE_NEXT, &args![node]).u32();
        }
    }
}

// Translated from 00962450 (decompiled, FalloutNV.exe 1.4.0.525)
/// When the player has a character controller (`009306d0`), gives
/// `00819250(controller)` to the camera caster's `00620ba0`.
pub fn fn_00962450(e: &mut Engine, this: Ptr<PlayerCharacter>) {
    let controller = e.call(GET_CHAR_CONTROLLER, &args![this]).u32();
    if controller != 0 {
        let value = e.call(FUNCTION_00819250, &args![controller]).u32();
        let caster = e.get(this, PlayerCharacter::pCameraCaster);
        e.call(FUNCTION_00620BA0, &args![caster, value]);
    }
}

// Translated from 00962490 (decompiled, FalloutNV.exe 1.4.0.525)
/// Assigns `value` (`NiPointer` assignment `0066b0d0`) to the player's
/// first person light (`+0x864`) when `first_person` is set, else to the
/// third person light (`+0x868`).
pub fn fn_00962490(e: &mut Engine, this: Ptr<PlayerCharacter>, value: u32, first_person: u8) {
    let slot = if first_person != 0 { 0x864 } else { 0x868 };
    e.call(SMART_POINTER_ASSIGN, &args![this.addr() + slot, value]);
}

// Translated from 009624d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Chooses the player's light from the extra data (`005d43c0` +0x44,
/// `00418250`): when `004b0460(extra, first light)` holds and the first
/// person 3D root's `00456610` is true, `006e5cc0(extra, third light)`
/// runs; otherwise when `004b0460(extra, third light)` holds and the third
/// person root's `00456610` is true, `006e5cc0(extra, first light)` runs.
/// Then `0088b4e0(this)` is called.
pub fn fn_009624d0(e: &mut Engine, this: Ptr<PlayerCharacter>) {
    let list = e.call(ADD_44_TO_ADDRESS, &args![this]).u32();
    let extra = e.call(FUNCTION_00418250, &args![list]).u32();
    if extra == 0 {
        return;
    }
    let first = this.addr() + 0x864;
    let third = this.addr() + 0x868;
    let mut swapped = false;
    if e.call(FUNCTION_004B0460, &args![extra, first]).bool() {
        let root = e.call(PLAYER_GET_ROOT_NODE, &args![this, 1u32]).u32();
        if e.call(FUNCTION_00456610, &args![root]).bool() {
            e.call(FUNCTION_006E5CC0, &args![extra, third]);
            swapped = true;
        }
    }
    if !swapped && e.call(FUNCTION_004B0460, &args![extra, third]).bool() {
        let root = e.call(PLAYER_GET_ROOT_NODE, &args![this, 0u32]).u32();
        if e.call(FUNCTION_00456610, &args![root]).bool() {
            e.call(FUNCTION_006E5CC0, &args![extra, first]);
        }
    }
    e.call(FUNCTION_0088B4E0, &args![this]);
}

// Translated from 00962590 (decompiled, FalloutNV.exe 1.4.0.525)
/// When `00570f60()` is positive and the process object (`008d8520`) has an
/// object from its virtual `+0x14c`, calls
/// `008ce180(form id, 0x7fffffff, this, 1, 1)` with the id `0044ddc0` reads
/// from that object (when it is not 0).
pub fn fn_00962590(e: &mut Engine, this: Ptr<PlayerCharacter>) {
    if e.call(FUNCTION_00570F60, &args![]).i32() <= 0 {
        return;
    }
    let process = e.call(PLAYER_GET_PROCESS, &args![this]).u32();
    if process == 0 || e.vcall(process, 0x14c, &args![]).u32() == 0 {
        return;
    }
    let object = e.vcall(process, 0x14c, &args![]).u32();
    let form_id = e.call(READ_FIELD_8, &args![object]).u32();
    if form_id != 0 {
        e.call(
            FUNCTION_008CE180,
            &args![form_id, 0x7fff_ffffu32, this, 1u32, 1u32],
        );
    }
}

// Translated from 00962620 (decompiled, FalloutNV.exe 1.4.0.525)
/// Counts the entries of the player's extra data list at `+0xc` of
/// `00422700` that still qualify: each must have a process (`008d8520`), a
/// package (`009344a0`) of type 1 (`0041ca90`) and virtual `+0x2c8` equal to
/// the player. An entry that does not qualify is removed (`00905330`) and
/// the count restarts from the head of the list. The walk ends at a node
/// without an item.
pub fn fn_00962620(e: &mut Engine, _this: Ptr<PlayerCharacter>) -> u32 {
    let player = e.global::<u32>(PLAYER_CHARACTER);
    let mut count = 0;
    let list = e.call(ADD_44_TO_ADDRESS, &args![player]).u32();
    let extra = e.call(FUNCTION_00422700, &args![list]).u32();
    if extra == 0 || e.mem.u32(extra + 0xc) == 0 {
        return count;
    }
    let mut node = e.mem.u32(extra + 0xc);
    while node != 0 {
        let slot = e.call(LIST_NODE_SLOT, &args![node]).u32();
        let item = e.mem.u32(slot);
        if item == 0 {
            break;
        }
        let mut qualifies = false;
        if e.call(PLAYER_GET_PROCESS, &args![item]).u32() != 0
            && e.call(FUNCTION_009344A0, &args![item]).u32() != 0
        {
            let package = e.call(FUNCTION_009344A0, &args![item]).u32();
            qualifies = e.call(FUNCTION_0041CA90, &args![package]).u32() == 1
                && e.vcall(item, 0x2c8, &args![]).u32() == player;
        }
        if qualifies {
            count += 1;
            node = e.call(LIST_NODE_NEXT, &args![node]).u32();
        } else {
            let head = e.mem.u32(extra + 0xc);
            e.with_stack(4, |e, cell| {
                e.mem.set_u32(cell.addr(), item);
                e.call(LIST_REMOVE, &args![head, cell]);
            });
            node = e.mem.u32(extra + 0xc);
            count = 0;
        }
    }
    count
}

// Translated from 00962720 (decompiled, FalloutNV.exe 1.4.0.525)
/// True when `item` is in the list at `+0xc` of the player's extra data
/// (`005d43c0` +0x44, `00422700`); the walk ends at a node without an item.
pub fn fn_00962720(e: &mut Engine, _this: Ptr<PlayerCharacter>, item: u32) -> bool {
    let player = e.global::<u32>(PLAYER_CHARACTER);
    let list = e.call(ADD_44_TO_ADDRESS, &args![player]).u32();
    let extra = e.call(FUNCTION_00422700, &args![list]).u32();
    if extra == 0 || e.mem.u32(extra + 0xc) == 0 {
        return false;
    }
    let mut node = e.mem.u32(extra + 0xc);
    while node != 0 {
        let slot = e.call(LIST_NODE_SLOT, &args![node]).u32();
        if e.mem.u32(slot) == 0 {
            break;
        }
        if e.mem.u32(slot) == item {
            return true;
        }
        node = e.call(LIST_NODE_NEXT, &args![node]).u32();
    }
    false
}

// Translated from 009627a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The first time (byte `+0x86c`, `bInsufficientChargeMessageShown`, clear),
/// sets the byte and shows the message whose text is the value of the setting
/// at `011d3048`: `007052f0(text, 0, 0, 0, float at 010162c0, 0)`.
pub fn fn_009627a0(e: &mut Engine, this: Ptr<PlayerCharacter>) {
    if e.get(this, PlayerCharacter::bInsufficientChargeMessageShown) == 0 {
        e.set(this, PlayerCharacter::bInsufficientChargeMessageShown, 1);
        let text = e.call(SETTING_STRING, &args![SETTING_011D3048]).u32();
        let duration: f32 = e.global(MESSAGE_DURATION_010162C0);
        e.call(
            FUNCTION_007052F0,
            &args![text, 0u32, 0u32, 0u32, duration, 0u32],
        );
    }
}

// Translated from 009627f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Clears `bInsufficientChargeMessageShown` (`+0x86c`).
pub fn fn_009627f0(e: &mut Engine, this: Ptr<PlayerCharacter>) {
    e.set(this, PlayerCharacter::bInsufficientChargeMessageShown, 0);
}

// Translated from 00962810 (decompiled, FalloutNV.exe 1.4.0.525)
/// Looks `form` (when given) up in the `RandomDoorSpaceMap` (`+0x854`) with
/// the key at its `+0xc` (`0057c850`) and returns the value found, 0xff when
/// there is none.
pub fn fn_00962810(e: &mut Engine, this: Ptr<PlayerCharacter>, form: Ptr) -> u8 {
    e.with_stack(4, |e, found| {
        e.mem.set_u8(found.addr(), 0xff);
        if form.addr() != 0 {
            let key = e.call(READ_DWORD_AT_C, &args![form]).u32();
            e.call(FUNCTION_0057C850, &args![this.addr() + 0x854, key, found]);
        }
        e.mem.u8(found.addr())
    })
}

// Translated from 00962850 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores `value` in the `RandomDoorSpaceMap` (`+0x854`) under the key at
/// `+0xc` of `form` (`0084d310`), when `form` is given.
pub fn fn_00962850(e: &mut Engine, this: Ptr<PlayerCharacter>, form: Ptr, value: u8) {
    if form.addr() != 0 {
        let key = e.call(READ_DWORD_AT_C, &args![form]).u32();
        e.call(
            FUNCTION_0084D310,
            &args![this.addr() + 0x854, key, u32::from(value)],
        );
    }
}

// Translated from 00962880 (decompiled, FalloutNV.exe 1.4.0.525)
/// Calls virtual `+0x24` of the process (`008d8520`) of every item of the
/// player's extra data list (`+0xc` of `00422700`) with `(item, 1)`. When the
/// call changed the list after the current node the walk restarts from the
/// head, otherwise it goes on with the node that followed.
pub fn fn_00962880(e: &mut Engine, _this: Ptr<PlayerCharacter>) {
    let player = e.global::<u32>(PLAYER_CHARACTER);
    let list = e.call(ADD_44_TO_ADDRESS, &args![player]).u32();
    let extra = e.call(FUNCTION_00422700, &args![list]).u32();
    if extra == 0 || e.mem.u32(extra + 0xc) == 0 {
        return;
    }
    let mut node = e.mem.u32(extra + 0xc);
    while node != 0 {
        let slot = e.call(LIST_NODE_SLOT, &args![node]).u32();
        if e.mem.u32(slot) == 0 {
            break;
        }
        let next = e.call(LIST_NODE_NEXT, &args![node]).u32();
        let slot = e.call(LIST_NODE_SLOT, &args![node]).u32();
        let item = e.mem.u32(slot);
        if item != 0 && e.call(PLAYER_GET_PROCESS, &args![item]).u32() != 0 {
            let process = e.call(PLAYER_GET_PROCESS, &args![item]).u32();
            e.vcall(process, 0x24, &args![item, 1u32]);
            node = if e.call(LIST_NODE_NEXT, &args![node]).u32() == next {
                next
            } else {
                e.mem.u32(extra + 0xc)
            };
        } else {
            node = next;
        }
    }
}

/// Removes `item` from the `BSSimpleList` starting at `list` (`00905330`,
/// which takes the address of a cell holding the item).
fn list_remove_item(e: &mut Engine, list: u32, item: u32) {
    e.with_stack(4, |e, cell| {
        e.mem.set_u32(cell.addr(), item);
        e.call(LIST_REMOVE, &args![list, cell]);
    });
}

/// Copies the three words at `source` to `destination`.
fn copy_vector(e: &mut Engine, source: u32, destination: u32) {
    for word in 0..3 {
        let value = e.mem.u32(source + word * 4);
        e.mem.set_u32(destination + word * 4, value);
    }
}

/// One probe of [`fn_00962950`]: `origin + direction` is built (`00439e90`),
/// `NiPick::PickObjects` (`00e98e20`) runs on the picker with it and the
/// direction, and a hit counts when the hit's normal (`004610d0` of the
/// result `00968670(picker + 0x18, 0)` returns) has a positive dot product
/// with the direction and `0045bad0(011f4aa0, 009611e0(00458b50(hit)))` is
/// false.
fn pick_blocks(e: &mut Engine, picker: u32, origin: u32, direction: u32, scratch: u32) -> bool {
    let sum = scratch;
    let normal = scratch + 0x10;
    let probe = e.call(VECTOR_ADD, &args![origin, sum, direction]).u32();
    if !e
        .call(PICK_OBJECTS, &args![picker, probe, direction, 0u32])
        .bool()
    {
        return false;
    }
    let results = e.call(PICKER_RESULTS, &args![picker]).u32();
    let hit = e.call(PICK_RESULT_GET, &args![results, 0u32]).u32();
    let hit_normal = e.call(PICK_RESULT_NORMAL, &args![hit]).u32();
    copy_vector(e, hit_normal, normal);
    let dot = e.call(VECTOR_DOT, &args![direction, normal]).f64();
    let zero: f64 = e.global(ZERO_DOUBLE);
    if dot.partial_cmp(&zero) != Some(std::cmp::Ordering::Greater) {
        return false;
    }
    let object = e.call(FUNCTION_00458B50, &args![hit]).u32();
    let key = e.call(FUNCTION_009611E0, &args![object]).u32();
    !e.call(FUNCTION_0045BAD0, &args![PICK_FILTER, key]).bool()
}

// Translated from 00962950 (decompiled, FalloutNV.exe 1.4.0.525)
/// Probes around `target`'s position: returns true when one of three
/// `NiPick` rays from a point above the player's position (the player's
/// virtual `+0x1f4` position raised by `Actor::GetHeight` times the double
/// at `010290b0`) along the directions `(0, a, 0)`, `(a, b, 0)` and
/// `(b, b, 0)` (floats `0104e0e8` and `0108b3f0`) hits (see [`pick_blocks`]).
/// When none does it asks the player's character controller
/// (`009306d0`, `bhkCharacterController::CheckInsideOfObject`, Xbox PDB).
/// The picker (global `011e0d64`, first set to the object at `011e0aa8`) is
/// set up the first time and given `target` as its root (`00705fc0`) while
/// probing. Returns false for a null `target`.
pub fn fn_00962950(e: &mut Engine, this: Ptr<PlayerCharacter>, target: u32) -> bool {
    if target == 0 {
        return false;
    }
    if e.global::<u32>(PICKER_POINTER) == 0 {
        e.set_global(PICKER_POINTER, PICKER_OBJECT);
        e.call(FUNCTION_00632D20, &args![PICKER_OBJECT, 0u32]);
        fn_00962cb0(e, Ptr::new(PICKER_OBJECT), 1);
        e.call(FUNCTION_00458B30, &args![PICKER_OBJECT, 1u32]);
    }
    let picker = e.global::<u32>(PICKER_POINTER);
    let found = e.with_stack(0x60, |e, frame| {
        let frame = frame.addr();
        let origin = frame;
        let direction = frame + 0x10;
        let vector_temp = frame + 0x20;
        let scratch = frame + 0x30;
        let position = e.vcall(this.addr(), 0x1f4, &args![]).u32();
        copy_vector(e, position, origin);
        let offset_a: f32 = e.global(PICK_DISTANCE_0104E0E8);
        let offset_b: f32 = e.global(PICK_DISTANCE_0108B3F0);
        e.call(NI_POINT3_NEW, &args![direction, 0.0f32, offset_a, 0.0f32]);
        let player = e.global::<u32>(PLAYER_CHARACTER);
        let height = e.call(ACTOR_GET_HEIGHT, &args![player]).f64();
        let scale: f64 = e.global(HEIGHT_SCALE_010290B0);
        let raised = height * scale + f64::from(e.mem.f32(origin + 8));
        e.mem.set_f32(origin + 8, raised as f32);
        e.call(FUNCTION_00705FC0, &args![picker, target]);
        let mut found = pick_blocks(e, picker, origin, direction, scratch);
        if !found {
            let made = e
                .call(
                    NI_POINT3_NEW,
                    &args![vector_temp, offset_a, offset_b, 0.0f32],
                )
                .u32();
            copy_vector(e, made, direction);
            found = pick_blocks(e, picker, origin, direction, scratch);
        }
        if !found {
            let made = e
                .call(
                    NI_POINT3_NEW,
                    &args![vector_temp, offset_b, offset_b, 0.0f32],
                )
                .u32();
            copy_vector(e, made, direction);
            found = pick_blocks(e, picker, origin, direction, scratch);
        }
        found
    });
    e.call(FUNCTION_00705FC0, &args![picker, 0u32]);
    e.call(PICKER_CLEAR, &args![picker, 0u32]);
    if found {
        return true;
    }
    let controller = e.call(GET_CHAR_CONTROLLER, &args![this]).u32();
    e.call(CHECK_INSIDE_OF_OBJECT, &args![controller]).bool()
}

// Translated from 00962cb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores `value` in the byte at `+0x31` of the object (the picker).
pub fn fn_00962cb0(e: &mut Engine, this: Ptr, value: u8) {
    e.mem.set_u8(this.addr() + 0x31, value);
}

// Translated from 00962cd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Removes `item` (when not null) from the list at `011e0ae8` (`00905330`).
pub fn fn_00962cd0(e: &mut Engine, _this: Ptr, item: u32) {
    if item != 0 {
        list_remove_item(e, PICKER_LIST, item);
    }
}

// Translated from 00962d00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Walks the list at `011e0ae8`. An entry whose virtual `+0x1d0` (3D) is
/// null, or for which [`fn_00962950`] is false, is dropped from the list:
/// the second case also calls `00450f90(3D, 0)` and
/// `00c6a0b0(3D, 1, 1, 0)`. After a removal the walk restarts from the
/// head, and it ends when the list is empty.
pub fn fn_00962d00(e: &mut Engine, this: Ptr<PlayerCharacter>) {
    if e.call(LIST_NODE_IS_EMPTY, &args![PICKER_LIST]).bool() {
        return;
    }
    let mut node = PICKER_LIST;
    while node != 0 {
        let mut remove = false;
        let slot = e.call(LIST_NODE_SLOT, &args![node]).u32();
        let entry = e.mem.u32(slot);
        let root = e.vcall(entry, 0x1d0, &args![]).u32();
        if root == 0 {
            remove = true;
        } else if !fn_00962950(e, this, root) {
            remove = true;
            e.call(FUNCTION_00450F90, &args![root, 0u32]);
            e.call(FUNCTION_00C6A0B0, &args![root, 1u32, 1u32, 0u32]);
        }
        if remove {
            list_remove_item(e, PICKER_LIST, entry);
            if e.call(LIST_NODE_IS_EMPTY, &args![PICKER_LIST]).bool() {
                return;
            }
            node = PICKER_LIST;
        } else {
            node = e.call(LIST_NODE_NEXT, &args![node]).u32();
        }
    }
}

/// Frame of [`player_character_add_gun_wobble`]: the matrix the wobble is
/// built in, the identity-based matrices, the euler angles and the offsets
/// the camera code returns.
const WOBBLE_FRAME_SIZE: u32 = 0x100;
const WOBBLE_MATRIX: u32 = 0x00;
const WOBBLE_BASE: u32 = 0x40;
const WOBBLE_PRODUCT: u32 = 0x80;
const WOBBLE_ANGLES: u32 = 0xc0;
const WOBBLE_OFFSET_X: u32 = 0xd0;
const WOBBLE_OFFSET_Y: u32 = 0xd4;

/// Copies a 3x3 matrix (9 words).
fn copy_matrix(e: &mut Engine, source: u32, destination: u32) {
    for word in 0..9 {
        let value = e.mem.u32(source + word * 4);
        e.mem.set_u32(destination + word * 4, value);
    }
}

/// `(value as f32) * scale` rounded to `f32` the way the stores of the x87
/// code do.
fn scaled(value: f32, scale: f32) -> f32 {
    (f64::from(value) * f64::from(scale)) as f32
}

/// Gives the node `wobble` a `bhkBlendCollisionObjectAddRotation`
/// (`00c8ffd0` on a 0x50 byte block from `00aa13e0`) through `0062bc90`.
fn attach_blend_collision_object(e: &mut Engine, wobble: u32, copy_from: u32) {
    let block = e.call(ALLOCATE_00AA13E0, &args![0x50u32]).u32();
    let object = if block != 0 {
        e.call(FUNCTION_00C8FFD0, &args![block]).u32()
    } else {
        0
    };
    if copy_from != 0 {
        e.call(FUNCTION_00C90090, &args![object, copy_from]);
    }
    e.call(FUNCTION_0062BC90, &args![wobble, object]);
}

// Translated from 00962de0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PlayerCharacter::AddGunWobble` (Xbox PDB): moves the gun wobble nodes of
/// the player's current weapon and leaves the first person rotation in the
/// global matrix at `011e09ec`.
///
/// While the process says it has a weapon out (virtual `+0x454`), the matrix
/// and the node name come from `008d6970(table[weapon type], animation
/// value, matrix)` for a ranged weapon; otherwise they come from the aim
/// offsets the player's `00965620` computes, smoothed towards the globals at
/// `011e0d68` and `011e0d6c`. The node found (or cached at `+0xd74` + 4 *
/// type) gets its blend rotation collision object: the euler angles of the
/// matrix are scaled by the wobble scale, which moves towards its target at
/// `011a3b2c` by a step per frame. The first person pass then does the same
/// with the nodes cached from `+0xda4`, or, with no first person node
/// index, adds the rotation to the character (`00931e50`, `00931d30`).
/// The exception frame is not translated.
pub fn player_character_add_gun_wobble(e: &mut Engine, this: Ptr<PlayerCharacter>) {
    e.with_stack(WOBBLE_FRAME_SIZE, |e, frame| {
        add_gun_wobble(e, this, frame.addr());
    });
}

/// Body of [`player_character_add_gun_wobble`] with its stack `frame`.
fn add_gun_wobble(e: &mut Engine, this: Ptr<PlayerCharacter>, frame: u32) {
    let matrix = frame + WOBBLE_MATRIX;
    let base = frame + WOBBLE_BASE;
    let product = frame + WOBBLE_PRODUCT;
    let angles = frame + WOBBLE_ANGLES;
    let weapon = e.call(ACTOR_GET_CURRENT_WEAPON, &args![this]).u32();
    let mut node_name = 0u32;
    let mut weapon_type = 0u32;
    let mut scale = 0.0f32;
    let process = e.get(this, PlayerCharacter::pCurrentProcess).addr();
    if process != 0 && e.vcall(process, 0x454, &args![]).bool() {
        let animation = e.call(ACTOR_GET_ANIMATION, &args![this]).u32();
        let ranged = weapon != 0 && !e.call(WEAPON_IS_MELEE_TYPE, &args![weapon]).bool();
        if ranged {
            weapon_type = e.call(GET_BYTE_AT_F4, &args![weapon]).i32() as u32;
            let value = e.call(READ_FLOAT_AT_D0, &args![animation]).f32();
            let index = e.mem.u32(WOBBLE_INDEX_TABLE + weapon_type.wrapping_mul(4));
            node_name = e
                .call(GUN_WOBBLE_GET_MATRIX, &args![index, value, matrix])
                .u32();
        } else {
            node_name = DEFAULT_WOBBLE_NODE_NAME;
            e.mem.set_f32(frame + WOBBLE_OFFSET_X, 0.0);
            e.mem.set_f32(frame + WOBBLE_OFFSET_Y, 0.0);
            let player = e.global::<u32>(PLAYER_CHARACTER);
            let root = e.call(PLAYER_GET_ROOT_NODE, &args![player, 0u32]).u32();
            let aim_node = e.call(FIND_NODE_BY_NAME, &args![root, AIM_NODE_NAME]).u32();
            if fn_00963730(e, Ptr::new(player)).addr() != 0 {
                let limit: f32 = e.global(AIM_LIMIT_0102EFC4);
                e.call(
                    FUNCTION_00965620,
                    &args![
                        player,
                        0u32,
                        aim_node,
                        frame + WOBBLE_OFFSET_X,
                        frame + WOBBLE_OFFSET_Y,
                        0u32,
                        limit,
                        limit
                    ],
                );
            }
            // Both aim globals move towards the offsets by a fraction of the
            // frame time.
            for (global, offset) in [
                (AIM_GLOBAL_X, frame + WOBBLE_OFFSET_X),
                (AIM_GLOBAL_Y, frame + WOBBLE_OFFSET_Y),
            ] {
                let current = e.global::<f32>(global);
                let difference = f64::from(current) - f64::from(e.mem.f32(offset));
                let frame_time = e.call(READ_FLOAT_AT_C, &args![FRAME_TIME_OBJECT]).f64();
                let weighted = frame_time * difference;
                let rate = e.call(SETTING_FLOAT, &args![AIM_SMOOTHING_SETTING]).u32();
                let rate = e.mem.f32(rate);
                e.set_global(
                    global,
                    (f64::from(current) - f64::from(rate) * weighted) as f32,
                );
            }
            let x = e.global::<f32>(AIM_GLOBAL_X);
            let z = e.global::<f32>(AIM_GLOBAL_Y);
            e.call(MATRIX_FROM_EULER, &args![matrix, x, 0.0f32, z]);
        }
    }
    if node_name != 0 {
        let cache = this.addr() + 0xd74 + weapon_type.wrapping_mul(4);
        let mut wobble = e.mem.u32(cache);
        if wobble == 0 {
            let model_root = e.call(FUNCTION_0043FCD0, &args![this]).u32();
            wobble = e
                .call(FIND_NODE_BY_NAME, &args![model_root, node_name])
                .u32();
            e.mem.set_u32(cache, wobble);
        }
        if wobble != 0 {
            let ranged = weapon != 0 && !e.call(WEAPON_IS_MELEE_TYPE, &args![weapon]).bool();
            scale = if ranged {
                e.call(FUNCTION_008B0DD0, &args![this, 2u32]).f32()
            } else {
                1.0
            };
            let state_flag = e.call(FUNCTION_00894900, &args![this]).bool();
            if !state_flag {
                let factor = e.call(SETTING_FLOAT, &args![WOBBLE_SCALE_SETTING]).u32();
                scale = scaled(scale, e.mem.f32(factor));
            }
            let target: f32 = e.global(WOBBLE_SCALE_GLOBAL);
            let delta = (f64::from(scale) - f64::from(target)) as f32;
            let rate = e.call(SETTING_FLOAT, &args![WOBBLE_STEP_SETTING]).u32();
            let frame_time = e.call(READ_FLOAT_AT_C, &args![FRAME_TIME_OBJECT]).f64();
            let step = (frame_time * f64::from(e.mem.f32(rate))) as f32;
            if delta != 0.0 {
                let magnitude = e.call(FLOAT_FUNCTION_00408840, &args![delta]).f64();
                let current: f32 = e.global(WOBBLE_SCALE_GLOBAL);
                if f64::from(step).partial_cmp(&magnitude) != Some(std::cmp::Ordering::Less) {
                    e.set_global(WOBBLE_SCALE_GLOBAL, scale);
                } else if delta > 0.0 {
                    if !state_flag {
                        let moved = (f64::from(current) + f64::from(step)) as f32;
                        e.set_global(WOBBLE_SCALE_GLOBAL, moved);
                        scale = moved;
                    } else {
                        e.set_global(WOBBLE_SCALE_GLOBAL, scale);
                    }
                } else {
                    let moved = (f64::from(current) - f64::from(step)) as f32;
                    e.set_global(WOBBLE_SCALE_GLOBAL, moved);
                    scale = moved;
                }
            }
            let collision = e.call(FUNCTION_006838B0, &args![wobble]).u32();
            if collision == 0 {
                attach_blend_collision_object(e, wobble, 0);
            } else {
                let casted = e
                    .call(NI_OBJECT_CAST, &args![TYPE_COLLISION_BASE, collision])
                    .u32();
                if casted != 0
                    && e.call(NI_OBJECT_CAST, &args![TYPE_BLEND_COLLISION, casted])
                        .u32()
                        == 0
                {
                    attach_blend_collision_object(e, wobble, casted);
                }
            }
            let collision = e.call(FUNCTION_006838B0, &args![wobble]).u32();
            let blend = e
                .call(NI_OBJECT_CAST, &args![TYPE_BLEND_COLLISION, collision])
                .u32();
            if blend != 0 {
                copy_matrix(e, IDENTITY_MATRIX, base);
                e.call(
                    MATRIX_TO_EULER,
                    &args![matrix, angles, angles + 4, angles + 8],
                );
                let x = scaled(e.mem.f32(angles), scale);
                let y = scaled(e.mem.f32(angles + 4), scale);
                let z = scaled(e.mem.f32(angles + 8), scale);
                e.call(MATRIX_FROM_EULER, &args![matrix, x, y, z]);
                let made = e.call(MATRIX_MULTIPLY, &args![base, product, matrix]).u32();
                copy_matrix(e, made, base);
                e.call(FUNCTION_004F0110, &args![blend, base]);
            }
        }
    }
    copy_matrix(e, IDENTITY_MATRIX, WOBBLE_RESULT_MATRIX);
    let process = e.get(this, PlayerCharacter::pCurrentProcess).addr();
    if process == 0 || weapon == 0 || !e.vcall(process, 0x454, &args![]).bool() {
        return;
    }
    let weapon_type = e.call(GET_BYTE_AT_F4, &args![weapon]).i32() as u32;
    let mut index = e.mem.u32(WOBBLE_INDEX_TABLE + weapon_type.wrapping_mul(4));
    if e.call(ACTOR_GET_IRON_SIGHTS, &args![this]).bool() {
        let form_part = e.call(FUNCTION_00504E60, &args![weapon]).u32();
        if e.call(FUNCTION_0048CEE0, &args![form_part]).u32() != 0 {
            let flagged = e.call(FUNCTION_004AD030, &args![weapon]).bool();
            let clear = if flagged {
                let item = e.vcall(process, 0x148, &args![]).u32();
                e.with_stack(4, |e, out| {
                    e.call(ITEM_CHANGE_HAS_MOD_EFFECT, &args![item, 0xeu32, out])
                        .bool()
                })
            } else {
                true
            };
            if clear {
                index = 0;
            }
        }
    }
    let first_person = e.get(this, PlayerCharacter::p1stPersonAnimation).addr();
    let value = e.call(READ_FLOAT_AT_D0, &args![first_person]).f32();
    node_name = e
        .call(GUN_WOBBLE_GET_MATRIX, &args![index, value, matrix])
        .u32();
    if node_name == 0 {
        return;
    }
    if index == 0 {
        scale = e.call(FUNCTION_008B0DD0, &args![this, 0u32]).f32();
        let factor = e
            .call(SETTING_FLOAT, &args![FIRST_PERSON_SCALE_SETTING])
            .u32();
        scale = scaled(scale, e.mem.f32(factor));
        e.call(
            MATRIX_TO_EULER,
            &args![matrix, angles, angles + 4, angles + 8],
        );
        let a = scaled(e.mem.f32(angles), scale);
        e.call(FUNCTION_00931E50, &args![this, a]);
        let c = scaled(e.mem.f32(angles + 8), scale);
        e.call(FUNCTION_00931D30, &args![this, c]);
        let x = scaled(e.mem.f32(angles), scale);
        let y = scaled(e.mem.f32(angles + 4), scale);
        let z = scaled(-e.mem.f32(angles + 8), scale);
        e.call(MATRIX_FROM_EULER, &args![WOBBLE_RESULT_MATRIX, z, y, x]);
    } else {
        let cache = this.addr() + 0xda4 + index.wrapping_mul(4);
        let mut wobble = e.mem.u32(cache);
        if wobble == 0 {
            let root = e.call(PLAYER_GET_ROOT_NODE, &args![this, 1u32]).u32();
            wobble = e.call(FIND_NODE_BY_NAME, &args![root, node_name]).u32();
            e.mem.set_u32(cache, wobble);
        }
        if wobble == 0 {
            return;
        }
        if e.call(FUNCTION_006838B0, &args![wobble]).u32() == 0 {
            attach_blend_collision_object(e, wobble, 0);
        }
        let collision = e.call(FUNCTION_006838B0, &args![wobble]).u32();
        let blend = e
            .call(NI_OBJECT_CAST, &args![TYPE_BLEND_COLLISION, collision])
            .u32();
        if blend == 0 {
            return;
        }
        e.call(
            MATRIX_TO_EULER,
            &args![matrix, angles, angles + 4, angles + 8],
        );
        let x = scaled(e.mem.f32(angles), scale);
        let y = scaled(e.mem.f32(angles + 4), scale);
        let z = scaled(e.mem.f32(angles + 8), scale);
        e.call(MATRIX_FROM_EULER, &args![WOBBLE_RESULT_MATRIX, x, y, z]);
        e.call(FUNCTION_004F0110, &args![blend, WOBBLE_RESULT_MATRIX]);
        let x = scaled(e.mem.f32(angles), scale);
        let y = scaled(e.mem.f32(angles + 4), scale);
        let z = scaled(-e.mem.f32(angles + 8), scale);
        e.call(MATRIX_FROM_EULER, &args![WOBBLE_RESULT_MATRIX, z, y, x]);
    }
}

// Translated from 00963730 (decompiled, FalloutNV.exe 1.4.0.525)
/// The player's auto aim actor (`pAutoAimActor`, `+0xd2c`) when it has a 3D
/// (virtual `+0x1d0`), else null.
pub fn fn_00963730(e: &mut Engine, this: Ptr<PlayerCharacter>) -> Ptr {
    let actor = e.get(this, PlayerCharacter::pAutoAimActor);
    if actor.addr() != 0 && e.vcall(actor.addr(), 0x1d0, &args![]).u32() != 0 {
        actor
    } else {
        Ptr::new(0)
    }
}

/// The player's perk list: `Perks` (`+0x87c`) or, with `companion` set,
/// `CompanionPerks` (`+0xad4`).
fn perk_list(this: Ptr<PlayerCharacter>, companion: u8) -> u32 {
    this.addr() + if companion != 0 { 0xad4 } else { 0x87c }
}

/// The `PerkRankData` (perk word, rank byte at `+4`) of `perk` in the
/// `BSSimpleList` starting at `list`, or 0.
fn find_perk_rank_data(e: &mut Engine, list: u32, perk: u32) -> u32 {
    let mut node = list;
    while node != 0 {
        let slot = e.call(LIST_NODE_SLOT, &args![node]).u32();
        let data = e.mem.u32(slot);
        if data != 0 && e.mem.u32(data) == perk {
            return data;
        }
        node = e.call(LIST_NODE_NEXT, &args![node]).u32();
    }
    0
}

// Translated from 00963790 (decompiled, FalloutNV.exe 1.4.0.525)
/// Gives the player `perk` at `rank` (`companion` selects the companion
/// list). An existing entry with another rank is changed (`005eb6a0(perk,
/// player, old rank, new rank, companion)`); otherwise a new 8 byte
/// `PerkRankData` is allocated and appended when not already in the list
/// (`005f65d0`, `005ae3d0`) and `005eb6a0(perk, player, 0, rank, companion)`
/// runs. Both refresh the stats menu (`007dd710`) for the player's own list;
/// a new entry also calls `008c17c0(player)`.
pub fn fn_00963790(e: &mut Engine, this: Ptr<PlayerCharacter>, perk: u32, rank: u8, companion: u8) {
    if perk == 0 {
        return;
    }
    let list = perk_list(this, companion);
    let mut node = list;
    while node != 0 {
        let slot = e.call(LIST_NODE_SLOT, &args![node]).u32();
        let data = e.mem.u32(slot);
        if data != 0 && e.mem.u32(data) == perk {
            let old_rank = e.mem.u8(data + 4);
            if old_rank != rank {
                e.call(
                    FUNCTION_005EB6A0,
                    &args![
                        perk,
                        this,
                        u32::from(old_rank),
                        u32::from(rank),
                        u32::from(companion)
                    ],
                );
                e.mem.set_u8(data + 4, rank);
                if companion == 0 {
                    e.call(FUNCTION_007DD710, &args![]);
                }
            }
            return;
        }
        node = e.call(LIST_NODE_NEXT, &args![node]).u32();
    }
    let data = e.call(OPERATOR_NEW, &args![8u32]).u32();
    e.mem.set_u32(data, perk);
    e.mem.set_u8(data + 4, rank);
    e.with_stack(4, |e, cell| {
        e.mem.set_u32(cell.addr(), data);
        if !e.call(LIST_CONTAINS, &args![list, cell]).bool() {
            e.call(LIST_APPEND, &args![list, cell]);
        }
    });
    e.call(
        FUNCTION_005EB6A0,
        &args![perk, this, 0u32, u32::from(rank), u32::from(companion)],
    );
    if companion == 0 {
        e.call(FUNCTION_007DD710, &args![]);
    }
    e.call(FUNCTION_008C17C0, &args![this]);
}

// Translated from 00963900 (decompiled, FalloutNV.exe 1.4.0.525)
/// Takes `perk` away from the player (`companion` selects the list): when
/// it is in the list, `005eb800(perk, player, companion)` runs, the entry is
/// removed from the list (`00905330`) and the stats menu (`007dd710`) and
/// `008c17c0(player)` are refreshed. The entry itself is not freed.
pub fn fn_00963900(e: &mut Engine, this: Ptr<PlayerCharacter>, perk: u32, companion: u8) {
    if perk == 0 {
        return;
    }
    let list = perk_list(this, companion);
    let data = find_perk_rank_data(e, list, perk);
    if data == 0 {
        return;
    }
    e.call(FUNCTION_005EB800, &args![perk, this, u32::from(companion)]);
    list_remove_item(e, list, data);
    e.call(FUNCTION_007DD710, &args![]);
    e.call(FUNCTION_008C17C0, &args![this]);
}

// Translated from 009639e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The rank at which the player has `perk` (`companion` selects the list),
/// 0 when it is not in the list.
pub fn fn_009639e0(e: &mut Engine, this: Ptr<PlayerCharacter>, perk: u32, companion: u8) -> u8 {
    let list = perk_list(this, companion);
    let data = find_perk_rank_data(e, list, perk);
    if data != 0 {
        e.mem.u8(data + 4)
    } else {
        0
    }
}

/// Number of perk entry lists (`PerkEntryLists`, `CompanionPerkEntryLists`).
const PERK_ENTRY_LIST_COUNT: u8 = 0x4a;

/// The entry type (`0062f2f0`) of a perk entry `form` (virtual `+0x10`
/// equal to 2), when it is below [`PERK_ENTRY_LIST_COUNT`].
fn perk_entry_type(e: &mut Engine, form: Ptr) -> Option<u8> {
    if form.addr() == 0 || e.vcall(form.addr(), 0x10, &args![]).u32() != 2 {
        return None;
    }
    let kind = e.call(FUNCTION_0062F2F0, &args![form]).u8();
    (kind < PERK_ENTRY_LIST_COUNT).then_some(kind)
}

// Translated from 00963a50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Adds the perk entry `form` (its virtual `+0x10` must be 2, its type
/// `0062f2f0` below 0x4a) to the player's entry list for that type
/// (`PerkEntryLists` at `+0x884`, or `CompanionPerkEntryLists` at `+0xadc`;
/// 8 bytes each) through `007a7eb0(list, form, 005eb550)`, then resets the
/// float at `+8` of the player's extra data container (`00418520`,
/// `00963b00`).
pub fn fn_00963a50(e: &mut Engine, this: Ptr<PlayerCharacter>, form: Ptr, companion: u8) {
    let Some(kind) = perk_entry_type(e, form) else {
        return;
    };
    let list = this.addr() + u32::from(kind) * 8 + if companion != 0 { 0xadc } else { 0x884 };
    e.call(FUNCTION_007A7EB0, &args![list, form, PERK_ENTRY_COMPARE]);
    let player = e.global::<u32>(PLAYER_CHARACTER);
    let extra = e.call(ADD_44_TO_ADDRESS, &args![player]).u32();
    let container = e
        .call(EXTRA_LIST_GET_CONTAINER_CHANGES, &args![extra])
        .u32();
    fn_00963b00(e, Ptr::new(container));
}

// Translated from 00963b00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores the float at `01012054` in the float at `+8` of the object.
pub fn fn_00963b00(e: &mut Engine, this: Ptr) {
    let value: f32 = e.global(RESET_VALUE_01012054);
    e.mem.set_f32(this.addr() + 8, value);
}

// Translated from 00963b20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Removes the perk entry `form` from the player's entry list for its type
/// (see [`fn_00963a50`]) with `00905330`.
pub fn fn_00963b20(e: &mut Engine, this: Ptr<PlayerCharacter>, form: Ptr, companion: u8) {
    let Some(kind) = perk_entry_type(e, form) else {
        return;
    };
    let list = this.addr() + u32::from(kind) * 8 + if companion != 0 { 0xadc } else { 0x884 };
    list_remove_item(e, list, form.addr());
}

// Translated from 00963ba0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The player's perk entry list for entry type `kind` (see
/// [`fn_00963a50`]), or null for a type of 0x4a or more.
pub fn fn_00963ba0(_e: &mut Engine, this: Ptr<PlayerCharacter>, kind: u8, companion: u8) -> Ptr {
    if kind >= PERK_ENTRY_LIST_COUNT {
        return Ptr::new(0);
    }
    Ptr::new(this.addr() + u32::from(kind) * 8 + if companion != 0 { 0xadc } else { 0x884 })
}

// Translated from 00963bf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Ages the player's action list (`pListofActions`, `+0x60c`, entries with
/// the type at `+0`, a timer float at `+4` and an object at `+8`): an entry
/// whose timer is not positive and whose type is not 5 or 8 is removed from
/// the list (`00905330`, with the list starting at the entry's node) and
/// freed, and the walk restarts at the head. Afterwards every remaining
/// timer is reduced by the float `0084d030` reads from `011f6394`.
pub fn fn_00963bf0(e: &mut Engine, this: Ptr<PlayerCharacter>) {
    let list = e.get(this, PlayerCharacter::pListofActions).addr();
    let mut node = list;
    while node != 0 {
        let slot = e.call(LIST_NODE_SLOT, &args![node]).u32();
        let action = e.mem.u32(slot);
        if action == 0 {
            break;
        }
        let timer = e.mem.f32(action + 4);
        let kind = e.mem.u32(action);
        if timer > 0.0 || timer.is_nan() || kind == 5 || kind == 8 {
            node = e.call(LIST_NODE_NEXT, &args![node]).u32();
        } else {
            list_remove_item(e, node, action);
            e.call(OPERATOR_DELETE, &args![action]);
            node = e.get(this, PlayerCharacter::pListofActions).addr();
        }
    }
    let mut node = e.get(this, PlayerCharacter::pListofActions).addr();
    while node != 0 {
        let slot = e.call(LIST_NODE_SLOT, &args![node]).u32();
        let action = e.mem.u32(slot);
        if action == 0 {
            break;
        }
        let elapsed = e.call(READ_FLOAT_AT_C, &args![FRAME_TIME_OBJECT]).f64();
        let timer = f64::from(e.mem.f32(action + 4));
        e.mem.set_f32(action + 4, (timer - elapsed) as f32);
        node = e.call(LIST_NODE_NEXT, &args![node]).u32();
    }
}

// Translated from 00963ce0 (decompiled, FalloutNV.exe 1.4.0.525)
/// True when the player's action list (`+0x60c`) has an entry of type
/// `kind`.
pub fn fn_00963ce0(e: &mut Engine, this: Ptr<PlayerCharacter>, kind: u32) -> bool {
    let mut found = false;
    let mut node = e.get(this, PlayerCharacter::pListofActions).addr();
    while node != 0 && !found {
        let slot = e.call(LIST_NODE_SLOT, &args![node]).u32();
        let action = e.mem.u32(slot);
        if action == 0 {
            break;
        }
        if e.mem.u32(action) == kind {
            found = true;
        }
        node = e.call(LIST_NODE_NEXT, &args![node]).u32();
    }
    found
}

// Translated from 00963d60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Removes and frees every entry of the player's action list (`+0x60c`)
/// whose object (`+8`) is `object`, restarting the walk at the head after
/// each removal.
pub fn fn_00963d60(e: &mut Engine, this: Ptr<PlayerCharacter>, object: u32) {
    if e.get(this, PlayerCharacter::pListofActions).addr() == 0 || object == 0 {
        return;
    }
    let mut node = e.get(this, PlayerCharacter::pListofActions).addr();
    while node != 0 {
        let slot = e.call(LIST_NODE_SLOT, &args![node]).u32();
        let action = e.mem.u32(slot);
        if action == 0 {
            break;
        }
        if e.mem.u32(action + 8) == object {
            list_remove_item(e, node, action);
            e.call(OPERATOR_DELETE, &args![action]);
            node = e.get(this, PlayerCharacter::pListofActions).addr();
        } else {
            node = e.call(LIST_NODE_NEXT, &args![node]).u32();
        }
    }
}

// Translated from 00963e00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PlayerCharacter::RemovePlayerAction` (Xbox PDB): removes and frees the
/// first entry of the action list (`+0x60c`) of type `kind` whose object
/// (`+8`) is `object` (any object when `object` is 0).
pub fn player_character_remove_player_action(
    e: &mut Engine,
    this: Ptr<PlayerCharacter>,
    kind: u32,
    object: u32,
) {
    if e.get(this, PlayerCharacter::pListofActions).addr() == 0 {
        return;
    }
    let mut node = e.get(this, PlayerCharacter::pListofActions).addr();
    while node != 0 {
        let slot = e.call(LIST_NODE_SLOT, &args![node]).u32();
        let action = e.mem.u32(slot);
        if action == 0 {
            break;
        }
        if e.mem.u32(action) == kind && (object == 0 || object == e.mem.u32(action + 8)) {
            list_remove_item(e, node, action);
            e.call(OPERATOR_DELETE, &args![action]);
            return;
        }
        node = e.call(LIST_NODE_NEXT, &args![node]).u32();
    }
}

// Translated from 00963eb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Adds an action of type `kind` with `timer` and `object` to the player's
/// action list (`+0x60c`, created on first use with `0096a2d0`): nothing
/// happens while `IsPlayerCharacterInCombat` (`00953c50`) is true. An
/// existing entry of that type is taken out of the list and updated,
/// otherwise a new 0xc byte entry is made (`0078d900`); the object gets
/// `SetTargeted(1)` (`00564db0`) and the entry is appended (`005ae3d0`). The
/// exception frame is not translated.
pub fn fn_00963eb0(e: &mut Engine, this: Ptr<PlayerCharacter>, kind: u32, timer: f32, object: u32) {
    let combat = e.with_stack(4, |e, out| {
        e.mem.set_u8(out.addr(), 0);
        e.call(PLAYER_IS_IN_COMBAT, &args![this, out]).bool()
    });
    if combat {
        return;
    }
    if e.get(this, PlayerCharacter::pListofActions).addr() == 0 {
        let block = e.call(OPERATOR_NEW, &args![8u32]).u32();
        let list = if block != 0 {
            e.call(FUNCTION_0096A2D0, &args![block]).u32()
        } else {
            0
        };
        e.set(this, PlayerCharacter::pListofActions, Ptr::new(list));
    }
    let mut existing = 0u32;
    let mut node = e.get(this, PlayerCharacter::pListofActions).addr();
    while node != 0 {
        let slot = e.call(LIST_NODE_SLOT, &args![node]).u32();
        if e.mem.u32(slot) == 0 || existing != 0 {
            break;
        }
        let action = e.mem.u32(slot);
        if action != 0 && e.mem.u32(action) == kind {
            existing = action;
        }
        node = e.call(LIST_NODE_NEXT, &args![node]).u32();
    }
    let list = e.get(this, PlayerCharacter::pListofActions).addr();
    if existing == 0 {
        let block = e.call(OPERATOR_NEW, &args![0xcu32]).u32();
        existing = if block != 0 {
            e.call(FUNCTION_0078D900, &args![block]).u32()
        } else {
            0
        };
    } else {
        list_remove_item(e, list, existing);
    }
    e.mem.set_u32(existing, kind);
    e.mem.set_f32(existing + 4, timer);
    if object != 0 {
        e.call(TES_OBJECT_REFR_SET_TARGETED, &args![object, 1u32]);
    }
    e.mem.set_u32(existing + 8, object);
    e.with_stack(4, |e, cell| {
        e.mem.set_u32(cell.addr(), existing);
        e.call(LIST_APPEND, &args![list, cell]);
    });
}

// Translated from 00964060 (decompiled, FalloutNV.exe 1.4.0.525)
/// The type of the first entry of the player's action list (`+0x60c`), 0
/// when the list is missing or empty.
pub fn fn_00964060(e: &mut Engine, this: Ptr<PlayerCharacter>) -> u32 {
    let list = e.get(this, PlayerCharacter::pListofActions).addr();
    if list != 0 {
        let slot = e.call(LIST_NODE_SLOT, &args![list]).u32();
        if e.mem.u32(slot) != 0 {
            let slot = e.call(LIST_NODE_SLOT, &args![list]).u32();
            let action = e.mem.u32(slot);
            return e.mem.u32(action);
        }
    }
    0
}

/// Size of the emulated frame of [`fn_00960520`] (its locals reach down to
/// `[EBP - 0x6cc]`), and where `EBP` is in it.
const UPDATE_SPRING_FRAME: u32 = 0x700;
const UPDATE_SPRING_EBP: u32 = 0x6e0;

/// Releases the grab and destroys the ray-hit collector the update built.
fn release_and_destroy_collector(e: &mut Engine, this: Ptr<PlayerCharacter>, collector: u32) {
    fn_00961280(e, this);
    e.call(HIT_COLLECTOR_DELETE_004A3BC0, &args![collector]);
}

// Translated from 00960520 (decompiled, FalloutNV.exe 1.4.0.525)
/// Per-frame update of the grab spring (called by [`fn_0095f6c0`] for grab
/// type 1 and by `UpdateDropSpring`): releases the grab when the spring or
/// the grabbed object is gone, when the player's character controller stands
/// on the spring's body, when a ray from the eye position is blocked by
/// another object, when the held object is too far (`011d0664`) or too heavy
/// relative to the player's (`011cffc4`), and otherwise nudges the held
/// object (a velocity from the spring offset, scaled by settings and by the
/// held object's mass) and moves the spring's target to the point at
/// `distance` along the view ray. `_unused_1` is the float the callers pass
/// and the function never reads. The exception frame is not translated; the
/// 0x6cc-byte frame is emulated so that the locals the callees receive by
/// address keep the layout the game gives them.
pub fn fn_00960520(e: &mut Engine, this: Ptr<PlayerCharacter>, _unused_1: f32) {
    // `EBP` is on a 16-byte boundary in the exe (`AND ESP,0xfffffff0` before `MOV EBP,ESP`).
    e.with_stack(UPDATE_SPRING_FRAME + 0x10, |e, raw| {
        let frame = Frame {
            ebp: (raw.addr() + UPDATE_SPRING_EBP + 0xf) & !0xf,
        };
        update_spring_in_frame(e, frame, this)
    });
}

fn update_spring_in_frame(e: &mut Engine, f: Frame, this: Ptr<PlayerCharacter>) {
    let spring_slot = this.byte_add(0x634);
    let spring_of = |e: &mut Engine| e.call(READ_DWORD_AT_0, &args![spring_slot]).u32();
    if e.get(this, PlayerCharacter::pGrabbedObject).is_null() {
        fn_00961280(e, this);
        return;
    }
    let spring = spring_of(e);
    if spring == 0 {
        fn_00961280(e, this);
        return;
    }
    let spring = spring_of(e);
    if e.call(FUNCTION_0082AEB0, &args![spring]).u32() == 0 {
        fn_00961280(e, this);
        return;
    }
    let controller = e.call(GET_CHAR_CONTROLLER, &args![this]).u32();
    if controller != 0 {
        let body = e.call(FUNCTION_00C6FD20, &args![controller]).u32();
        let spring = spring_of(e);
        let spring_body = e.call(FUNCTION_0082AEB0, &args![spring]).u32();
        if body == spring_body && fn_00961260(e, Ptr::new(controller)) == 1 {
            fn_00961280(e, this);
            return;
        }
    }

    let player = e.global::<u32>(PLAYER_CHARACTER);
    let start_offset = 0.0f32;
    let reach = e.get(this, PlayerCharacter::fGrabDistance);
    let mut length = (reach as f64 + start_offset as f64) as f32;
    let eye_position = f.at(neg(0x30));
    let eye_direction = f.at(neg(0x3c));
    e.call(IDENTITY_006815C0, &args![eye_position]);
    e.call(IDENTITY_006815C0, &args![eye_direction]);
    e.call(
        ACTOR_GET_EYE_VECTOR,
        &args![this, eye_position, eye_direction, 1u32],
    );
    let ray = f.at(0xffff_ff10);
    e.call(FUNCTION_004A3C20, &args![ray]);
    let spring = spring_of(e);
    let entity = e.call(FUNCTION_0082AEE0, &args![spring]).u32();
    let entity_body = e.call(FUNCTION_004B5A80, &args![entity]).u32();
    e.call(FUNCTION_0043B4F0, &args![entity_body, f.at(0xffff_ff04)]);
    let owner = e
        .call(FUNCTION_00931ED0, &args![player, f.at(0xffff_ff00)])
        .u32();
    let filter = e.call(FUNCTION_004A3A20, &args![owner]).u32();
    e.call(FUNCTION_0059CE80, &args![f.at(0xffff_ff04), filter]);
    let filter_word = e.mem.u32(f.at(0xffff_ff04));
    e.call(FUNCTION_004A3F70, &args![ray, filter_word]);
    e.call(FUNCTION_004A3DA0, &args![ray, eye_position]);
    let end = e
        .call(
            FUNCTION_0045BB20,
            &args![eye_direction, f.at(0xffff_fef4), length],
        )
        .u32();
    e.call(FUNCTION_007F68A0, &args![ray, end]);
    let collector = f.at(0xffff_fbd0);
    e.call(HIT_COLLECTOR_NEW_004A3A70, &args![collector]);
    e.call(FUNCTION_004A3FB0, &args![ray, collector]);
    let player_loaded = e.call(READ_DWORD_AT_40, &args![player]).u32();
    let held = e
        .call(OBJECT_FUNCTION_004543C0, &args![player_loaded])
        .u32();
    let grabbed = e.get(this, PlayerCharacter::pGrabbedObject).addr();

    if e.vcall(held, 0xc8, &args![ray]).bool() {
        let one: f64 = e.global(DOUBLE_ONE);
        let mut fraction = 1.0f32;
        let mut index: i32 = 0;
        while fraction as f64 == one {
            let list = e.call(IDENTITY_00460140, &args![collector]).u32();
            let count = e.call(READ_FIELD_4, &args![list]).i32();
            if index >= count {
                break;
            }
            let list = e.call(IDENTITY_00460140, &args![collector]).u32();
            let hit = e
                .call(HIT_COLLECTOR_GET_004A46B0, &args![list, index as u32])
                .u32();
            let copy = f.at(0xffff_fb60);
            e.call(FUNCTION_00696C30, &args![copy, hit]);
            let collidable = e.mem.u32(f.at(0xffff_fbb0));
            let av_object = e
                .call(GET_AV_OBJECT_FOR_COLLIDABLE, &args![collidable])
                .u32();
            if av_object != 0 {
                let reference = e.call(FIND_REFERENCE_FOR_3D, &args![av_object]).u32();
                if reference != 0 && reference != grabbed && reference != player {
                    fraction = e.mem.f32(f.at(0xffff_fb70));
                }
            }
            index += 1;
        }
        let blocked = (fraction as f64 * length as f64 - start_offset as f64) as f32;
        length = e.call(FUNCTION_00404010, &args![0.0f32, blocked]).f32();
        if e.get(this, PlayerCharacter::eGrabType) == 3 {
            e.set(this, PlayerCharacter::fGrabDistance, length);
        }
    }

    if e.get(this, PlayerCharacter::eGrabType) == 3 {
        let second_ray = f.at(0xffff_faa0);
        e.call(FUNCTION_004A3C20, &args![second_ray]);
        let word = e
            .call(FUNCTION_0043B4F0, &args![entity_body, f.at(0xffff_fa9c)])
            .u32();
        let word = e.mem.u32(word);
        e.call(FUNCTION_004A3F70, &args![second_ray, word]);
        e.call(FUNCTION_004A3DA0, &args![second_ray, eye_position]);
        let node = e.vcall(grabbed, 0x1d0, &args![]).u32();
        let position_owner = e.call(GET_FIELD_20_OR_DEFAULT, &args![node]).u32();
        let position = e.call(IDENTITY_006815C0, &args![position_owner]).u32();
        for word in 0..3 {
            let value = e.mem.u32(position + word * 4);
            e.mem.set_u32(f.at(0xffff_fa90) + word * 4, value);
        }
        e.call(FUNCTION_004A3EB0, &args![second_ray, f.at(0xffff_fa90)]);
        if e.call(READ_DWORD_AT_40, &args![player]).u32() != 0 {
            let player_loaded = e.call(READ_DWORD_AT_40, &args![player]).u32();
            let held = e
                .call(OBJECT_FUNCTION_004543C0, &args![player_loaded])
                .u32();
            if e.vcall(held, 0xc8, &args![second_ray]).bool() {
                let picked = e.call(FUNCTION_00C66FB0, &args![second_ray]).u32();
                let reference = e.call(FIND_REFERENCE_FOR_3D, &args![picked]).u32();
                if reference != 0
                    && grabbed != reference
                    && !e.vcall(reference, 0x100, &args![]).bool()
                {
                    release_and_destroy_collector(e, this, collector);
                    return;
                }
            }
        }
    }

    let transform_a = f.at(0xffff_fa70);
    let anchor_a = f.at(0xffff_fa60);
    let anchor_b = f.at(0xffff_fa50);
    let transform = f.at(0xffff_fa10);
    e.call(IDENTITY_006815C0, &args![transform_a]);
    e.call(IDENTITY_006815C0, &args![anchor_a]);
    e.call(IDENTITY_006815C0, &args![anchor_b]);
    e.call(FUNCTION_0056DED0, &args![transform]);
    let spring = spring_of(e);
    fn_00961200(e, Ptr::new(spring), Ptr::new(anchor_b));
    let spring = spring_of(e);
    fn_00961230(e, Ptr::new(spring), Ptr::new(anchor_a));
    e.vcall(entity_body, 0xf4, &args![transform]);
    e.call(
        HK_SET_TRANSFORMED_POS,
        &args![transform_a, transform, anchor_a],
    );
    fn_00961190(e, Ptr::new(transform_a), Ptr::new(anchor_b));
    let difference = e
        .call(FUNCTION_00630B40, &args![transform_a, f.at(0xffff_fa00)])
        .u32();
    let squared = e.call(FUNCTION_004586D0, &args![difference]).f32();
    let separation = e.call(FUNCTION_004587D0, &args![squared]).f32();
    if e.get(this, PlayerCharacter::eGrabType) != 3 {
        let limit: f64 = e.global(DOUBLE_96);
        if separation as f64 > limit {
            release_and_destroy_collector(e, this, collector);
            return;
        }
    }

    let entity_count = e.call(FUNCTION_00C9C380, &args![entity]).i32();
    let mut heaviest = 0.0f32;
    let mut index: i32 = 0;
    while index < entity_count {
        let member = e
            .call(FUNCTION_00C9C390, &args![entity, index as u32])
            .u32();
        if member != 0 {
            let form = e.call(READ_DWORD_AT_C, &args![member]).u32();
            if e.vcall(form, 0x20, &args![]).u32() == 0xb {
                let owner = e.call(FUNCTION_00825C00, &args![member]).u32();
                let selected = if owner == entity {
                    fn_009611e0(e, Ptr::new(member))
                } else {
                    e.call(FUNCTION_00825C00, &args![member]).u32()
                };
                if selected != 0 {
                    let mass = e.call(FUNCTION_004B4EA0, &args![selected]).f32();
                    if heaviest < mass {
                        heaviest = e.call(FUNCTION_004B4EA0, &args![selected]).f32();
                    }
                }
            }
        }
        index += 1;
    }
    let entity_mass = e.call(FUNCTION_004B4EA0, &args![entity]).f32();
    let ratio = (heaviest as f64 / entity_mass as f64) as f32;
    let distance_limit = setting_float(e, SETTING_011D0664);
    if distance_limit < separation {
        let ratio_limit = setting_float(e, SETTING_011CFFC4);
        if ratio_limit < ratio {
            release_and_destroy_collector(e, this, collector);
            return;
        }
    }

    let mut node = e.vcall(grabbed, 0x1d0, &args![]).u32();
    let has_flag_8 = e.call(FUNCTION_00624B70, &args![node, 8u32]).u32() != 0;
    let has_flag_80 = e.call(FUNCTION_00624B70, &args![node, 0x80u32]).u32() != 0;
    let mut member_of_group = has_flag_8;
    if e.vcall(grabbed, 0x100, &args![]).bool() {
        let kind_object = e
            .call(FUNCTION_0043B4F0, &args![entity_body, f.at(0xffff_f9d0)])
            .u32();
        if e.call(FUNCTION_00624070, &args![kind_object]).bool() {
            member_of_group = false;
            let group = e.vcall(node, 0xc, &args![]).u32();
            if group != 0 && e.call(FUNCTION_0043B4A0, &args![group, 0u32]).u32() != 0 {
                node = e.call(FUNCTION_0043B4A0, &args![group, 0u32]).u32();
            }
            let owner_node = e.call(FUNCTION_004B5A20, &args![entity]).u32();
            let mut current = if owner_node != 0 {
                e.call(READ_FIELD_8, &args![owner_node]).u32()
            } else {
                0
            };
            while current != 0 && current != group {
                if current == node {
                    member_of_group = true;
                }
                current = fn_009611e0(e, Ptr::new(current));
            }
        }
    }
    if node != 0 && member_of_group {
        let near = setting_float(e, SETTING_011D1308);
        if near < separation {
            let scale_a = setting_float(e, SETTING_011D0808);
            let near_again = setting_float(e, SETTING_011D1308);
            let excess = (((separation as f64 - near_again as f64) * scale_a as f64)
                / separation as f64) as f32;
            let low = setting_float(e, SETTING_011D04D8);
            let high = setting_float(e, SETTING_011D037C);
            let weight = e.call(FUNCTION_0062C300, &args![node]).f32();
            let clamped = e.call(FUNCTION_00404010, &args![low, weight]).f32();
            let value = e.call(FUNCTION_0040EBD0, &args![high, clamped]).f32();
            let mapped = e
                .call(FUNCTION_004B3AB0, &args![0.0f32, 1.0f32, high, low, value])
                .f32();
            let factor: f64 = e.global(DOUBLE_MINUS_ONE);
            let strength = ((excess as f64 * factor) * mapped as f64) as f32;
            let velocity = f.at(0xffff_f990);
            e.call(FUNCTION_005DBF20, &args![velocity, strength]);
            e.call(FUNCTION_00627920, &args![transform_a, velocity]);
            if has_flag_80 || e.vcall(grabbed, 0x100, &args![]).bool() {
                let impulse = f.at(0xffff_f984);
                e.call(IDENTITY_006815C0, &args![impulse]);
                e.call(FUNCTION_00458620, &args![impulse, transform_a]);
                e.call(FUNCTION_0062B8D0, &args![node, impulse, 0u32]);
            } else {
                let spring = spring_of(e);
                let spring_entity = e.call(FUNCTION_0082AEE0, &args![spring]).u32();
                if spring_entity != 0 {
                    let body_object = e.call(FUNCTION_004B5A80, &args![spring_entity]).u32();
                    let vector = f.at(0xffff_f960);
                    e.call(IDENTITY_006815C0, &args![vector]);
                    let z = e.call(FUNCTION_00560D30, &args![transform_a, 2u32]).u32();
                    let z = e.mem.f32(z);
                    let y = e.call(FUNCTION_00560D30, &args![transform_a, 1u32]).u32();
                    let y = e.mem.f32(y);
                    let x = e.call(FUNCTION_00560D30, &args![transform_a, 0u32]).u32();
                    let x = e.mem.f32(x);
                    e.call(FUNCTION_004B4D50, &args![vector, x, y, z, 0.0f32]);
                    let position = e.call(FUNCTION_00560D80, &args![body_object]).u32();
                    e.call(FUNCTION_00553F70, &args![vector, position]);
                    e.call(FUNCTION_00561690, &args![body_object, vector]);
                }
            }
        }
    }

    let target = f.at(0xffff_f948);
    let end = e
        .call(
            FUNCTION_0045BB20,
            &args![eye_direction, f.at(0xffff_f954), length],
        )
        .u32();
    e.call(FUNCTION_00439E90, &args![eye_position, target, end]);
    if controller != 0 {
        let pulled = f.at(0xffff_f93c);
        e.call(FUNCTION_0045BB20, &args![eye_direction, pulled, length]);
        let drop_check = e.mem.f32(f.at(neg(0x34)));
        let zero: f64 = e.global(ZERO_DOUBLE);
        if (drop_check as f64) < zero {
            e.mem.set_f32(f.at(0xffff_f944), 0.0);
            let pulled_length = e.call(NI_POINT3_UNITIZE_GET_LENGTH, &args![pulled]).f32();
            let radius = e.call(FUNCTION_00C6E280, &args![controller]).f32();
            let scaled = e.call(FUNCTION_004587D0, &args![radius]).f32();
            let margin: f64 = e.global(DOUBLE_5);
            let limit = (scaled as f64 + margin) as f32;
            if limit > pulled_length {
                let remaining = (limit as f64 - pulled_length as f64) as f32;
                let moved = e
                    .call(
                        FUNCTION_0045BB20,
                        &args![pulled, f.at(0xffff_f924), remaining],
                    )
                    .u32();
                e.call(FUNCTION_0063C8A0, &args![target, moved]);
            }
        }
    }
    let spring = spring_of(e);
    e.call(FUNCTION_008C5D90, &args![spring, target]);
    e.call(HIT_COLLECTOR_DELETE_004A3BC0, &args![collector]);
}

/// Size of the emulated frame of [`fn_009614b0`] (its locals reach down to
/// `[EBP - 0x6c0]`), and where `EBP` is in it.
const DROP_POSITION_FRAME: u32 = 0x700;
const DROP_POSITION_EBP: u32 = 0x6e0;

/// Copies the three floats at `source` to `destination`.
fn copy_vector3(e: &mut Engine, source: u32, destination: u32) {
    for word in 0..3 {
        let value = e.mem.u32(source + word * 4);
        e.mem.set_u32(destination + word * 4, value);
    }
}

// Translated from 009614b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Finds where `object` can be dropped in front of the player, writing the
/// position to the vector at `out`: the object's bound size and the player's
/// character controller radius give the object_size. With `rotate` set the
/// drop angle (`fDropAngleMod`, +0x870) is advanced first and remembered in
/// `fLastDropAngleMod` (+0x874). Then up to `steps` candidate directions
/// (`steps` is 0xa with `search`, else 0), each rotated a further share of a
/// turn, are tried: a ray from the eye position, then a sphere shape
/// swept against the world; the first free spot is stored in `out` and the
/// function answers true. When none is free and `search` is set, the
/// player's position raised by a clamped amount is used and the answer is
/// true; otherwise it answers whether a spot was found. The exception
/// frame is not translated; the 0x6c0-byte frame is emulated so that the
/// locals the callees receive by address keep the layout the game gives
/// them.
pub fn fn_009614b0(
    e: &mut Engine,
    this: Ptr<PlayerCharacter>,
    object: Ptr,
    out: Ptr,
    search: u8,
    rotate: u8,
) -> bool {
    // `EBP` is on a 16-byte boundary in the exe (`AND ESP,0xfffffff0` before `MOV EBP,ESP`).
    e.with_stack(DROP_POSITION_FRAME + 0x10, |e, raw| {
        let frame = Frame {
            ebp: (raw.addr() + DROP_POSITION_EBP + 0xf) & !0xf,
        };
        drop_position_in_frame(e, frame, this, object, out, search, rotate)
    })
}

fn drop_position_in_frame(
    e: &mut Engine,
    f: Frame,
    this: Ptr<PlayerCharacter>,
    object: Ptr,
    out: Ptr,
    search: u8,
    rotate: u8,
) -> bool {
    let player = e.global::<u32>(PLAYER_CHARACTER);
    let two: f64 = e.global(DOUBLE_TWO);
    let half: f64 = e.global(DOUBLE_HALF);
    let full_turn: f64 = e.global(DOUBLE_TWO_PI);
    let zero: f64 = e.global(ZERO_DOUBLE);

    let bound = e.call(GET_BOUND_SIZE_0050EBF0, &args![object]).f32();
    let object_size = (bound as f64 * two) as f32;
    let controller = e.call(GET_CHAR_CONTROLLER, &args![player]).u32();
    let radius = e.call(FUNCTION_00C6E280, &args![controller]).f32();
    let radius_scaled = e.call(FUNCTION_004587D0, &args![radius]).f32();
    let scale = e.call(OBJECT_GET_SCALE, &args![this]).f32();
    let factor = setting_float(e, SETTING_011D0628);
    let reach = (factor as f64 * scale as f64) as f32;

    let eye_position = f.at(neg(0x38));
    let eye_direction = f.at(neg(0x44));
    e.call(IDENTITY_006815C0, &args![eye_position]);
    e.call(IDENTITY_006815C0, &args![eye_direction]);
    e.call(
        ACTOR_GET_EYE_VECTOR,
        &args![this, eye_position, eye_direction, 1u32],
    );
    let rotation = f.at(neg(0x68));
    e.call(IDENTITY_006815C0, &args![rotation]);

    if rotate != 0 {
        let angle = e.get(this, PlayerCharacter::fDropAngleMod);
        if angle as f64 != zero {
            let step = ((object_size as f64 * half) / reach as f64) as f32;
            let limited = e.call(FUNCTION_0040EBD0, &args![1.0f32, step]).f32();
            let change = e.call(FUNCTION_004B5510, &args![limited]).f32();
            let angle = e.get(this, PlayerCharacter::fDropAngleMod);
            e.set(
                this,
                PlayerCharacter::fDropAngleMod,
                (change as f64 + angle as f64) as f32,
            );
        }
        let angle = e.get(this, PlayerCharacter::fDropAngleMod);
        e.call(FUNCTION_004A0C90, &args![rotation, angle]);
        e.set(this, PlayerCharacter::fLastDropAngleMod, angle);
        let step = ((object_size as f64 * half) / reach as f64) as f32;
        let limited = e.call(FUNCTION_0040EBD0, &args![1.0f32, step]).f32();
        let change = e.call(FUNCTION_004B5510, &args![limited]).f32();
        let angle = e.get(this, PlayerCharacter::fDropAngleMod);
        e.set(
            this,
            PlayerCharacter::fDropAngleMod,
            (change as f64 + angle as f64) as f32,
        );
        let angle = e.get(this, PlayerCharacter::fDropAngleMod);
        if angle as f64 > full_turn {
            e.set(
                this,
                PlayerCharacter::fDropAngleMod,
                (angle as f64 - full_turn) as f32,
            );
        }
    } else {
        let last = e.get(this, PlayerCharacter::fLastDropAngleMod);
        e.call(FUNCTION_004A0C90, &args![rotation, last]);
    }

    let mut found = false;
    let steps: i32 = if search != 0 { 0xa } else { 0 };
    let direction = f.at(0xffff_ff78);
    let mut attempt: i32 = 0;
    while !found && attempt <= steps {
        e.call(
            FUNCTION_004B4500,
            &args![rotation, direction, eye_direction],
        );
        if attempt != 0 {
            e.mem.set_f32(f.at(neg(0x80)), 0.0);
            e.call(FUNCTION_004A0C10, &args![direction]);
            let share = ((attempt - 1) as f64 * (full_turn / steps as f64)) as f32;
            let partial = f.at(0xffff_ff4c);
            e.call(IDENTITY_006815C0, &args![partial]);
            e.call(FUNCTION_004A0C90, &args![partial, share]);
            let turned = e
                .call(
                    FUNCTION_004B4500,
                    &args![partial, f.at(0xffff_ff40), direction],
                )
                .u32();
            copy_vector3(e, turned, direction);
        }
        let mut spot_found = false;
        let ray = f.at(0xffff_fe80);
        e.call(FUNCTION_004A3C20, &args![ray]);
        e.call(FUNCTION_008C71B0, &args![f.at(0xffff_fe7c), 0u32]);
        e.call(FUNCTION_004A39F0, &args![f.at(0xffff_fe7c), 0x2au32]);
        let owner = e
            .call(FUNCTION_00931ED0, &args![player, f.at(0xffff_fe78)])
            .u32();
        let filter = e.call(FUNCTION_004A3A20, &args![owner]).u32();
        e.call(FUNCTION_0059CE80, &args![f.at(0xffff_fe7c), filter]);
        let filter_word = e.mem.u32(f.at(0xffff_fe7c));
        e.call(FUNCTION_004A3F70, &args![ray, filter_word]);
        e.call(FUNCTION_004A3DA0, &args![ray, eye_position]);
        let total = (reach as f64 + object_size as f64) as f32;
        let end = e
            .call(
                FUNCTION_0045BB20,
                &args![direction, f.at(0xffff_fe68), total],
            )
            .u32();
        e.call(FUNCTION_007F68A0, &args![ray, end]);
        let collector = f.at(0xffff_fb40);
        e.call(HIT_COLLECTOR_NEW_004A3A70, &args![collector]);
        e.call(FUNCTION_004A3FB0, &args![ray, collector]);
        let player_loaded = e.call(READ_DWORD_AT_40, &args![player]).u32();
        let held = e
            .call(OBJECT_FUNCTION_004543C0, &args![player_loaded])
            .u32();
        if e.vcall(held, 0xc8, &args![ray]).bool() {
            let sum = reach as f64 + object_size as f64;
            let list = e.call(IDENTITY_00460140, &args![collector]).u32();
            let first_hit = e.call(HIT_COLLECTOR_GET_004A46B0, &args![list, 0u32]).u32();
            let fraction = e.mem.f32(first_hit + 0x10);
            let hit_distance = (fraction as f64 * sum) as f32;
            let pulled_back = (hit_distance as f64 - object_size as f64 / two) as f32;
            e.call(FUNCTION_00439180, &args![direction, pulled_back]);
            let position = e
                .call(
                    FUNCTION_00439E90,
                    &args![eye_position, f.at(0xffff_fb1c), direction],
                )
                .u32();
            copy_vector3(e, position, out.addr());
            let x = e.mem.f32(direction);
            let y = e.mem.f32(direction + 4);
            let horizontal = f.at(0xffff_fb10);
            e.call(FUNCTION_00416870, &args![horizontal, x, y, 0.0f32]);
            let length = e.call(FUNCTION_00457990, &args![horizontal]).f32();
            let margin = length as f64 - object_size as f64 / two;
            let margin = (margin) as f32;
            if margin as f64 - radius_scaled as f64 > zero {
                spot_found = true;
            }
        } else {
            let beyond = (object_size as f64 / two + reach as f64) as f32;
            e.call(FUNCTION_00439180, &args![direction, beyond]);
            let position = e
                .call(
                    FUNCTION_00439E90,
                    &args![eye_position, f.at(0xffff_fafc), direction],
                )
                .u32();
            copy_vector3(e, position, out.addr());
            spot_found = true;
        }
        if spot_found {
            let block = e.call(ALLOCATE_00AA13E0, &args![0x14u32]).u32();
            let shape = if block != 0 {
                let sphere_radius = (object_size as f64 / two) as f32;
                e.call(SPHERE_SHAPE_NEW, &args![block, sphere_radius, 1u32])
                    .u32()
            } else {
                0
            };
            let shape_slot = f.at(0xffff_fae8);
            e.call(SMART_POINTER_ASSIGN_00633C90, &args![shape_slot, shape]);
            let owner = e
                .call(FUNCTION_00931ED0, &args![player, f.at(0xffff_fae4)])
                .u32();
            let filter = e.call(FUNCTION_004A3A20, &args![owner]).u32();
            let phantom_slot = f.at(0xffff_fae0);
            e.call(
                FUNCTION_00622290,
                &args![phantom_slot, 0x2au32, filter, 0u32],
            );
            let construction = f.at(0xffff_fa80);
            e.call(FUNCTION_0056E090, &args![construction]);
            let phantom = e.call(READ_DWORD_AT_0, &args![phantom_slot]).u32();
            e.mem.set_u32(construction, phantom);
            let shape_value = e.call(READ_DWORD_AT_0, &args![shape_slot]).u32();
            let shape_body = e.call(BODY_LOOKUP_004AE750, &args![shape_value]).u32();
            e.mem.set_u32(construction + 4, shape_body);
            e.call(FUNCTION_0056E120, &args![f.at(0xffff_faa0)]);
            let position_copy = f.at(0xffff_fa70);
            e.call(IDENTITY_006815C0, &args![position_copy]);
            e.call(FUNCTION_004A3E00, &args![position_copy, out]);
            e.call(FUNCTION_004B4DB0, &args![f.at(0xffff_fad0), position_copy]);
            let phantom_object = f.at(0xffff_fa58);
            e.call(FUNCTION_0056E2D0, &args![phantom_object, construction]);
            let loaded = e.call(READ_DWORD_AT_40, &args![this]).u32();
            let cell = e.call(OBJECT_FUNCTION_004543C0, &args![loaded]).u32();
            e.call(WORLD_OBJECT_ADD, &args![phantom_object, cell]);
            let pair_collector = f.at(0xffff_f940);
            e.call(FUNCTION_0062D350, &args![pair_collector]);
            e.call(FUNCTION_0062D310, &args![pair_collector]);
            e.call(FUNCTION_00623150, &args![phantom_object, pair_collector]);
            e.call(PHANTOM_REMOVE, &args![phantom_object]);
            e.call(SMART_POINTER_ASSIGN, &args![shape_slot, 0u32]);
            let bodies = e.call(FUNCTION_00413F40, &args![pair_collector]).u32();
            if e.call(READ_FIELD_4, &args![bodies]).u32() == 0 {
                found = true;
            }
            e.call(FUNCTION_0062D4B0, &args![pair_collector]);
            e.call(PHANTOM_DELETE_00C9EEC0, &args![phantom_object]);
            e.call(FUNCTION_0056EA90, &args![construction]);
            e.call(FUNCTION_0045CEC0, &args![shape_slot]);
        }
        e.call(HIT_COLLECTOR_DELETE_004A3BC0, &args![collector]);
        attempt += 1;
    }

    if !found && search != 0 {
        let share = (object_size as f64 / two) as f32;
        let ground: f32 = e.global(F32_CONSTANT_0101E6EC);
        let raise = e.call(FUNCTION_0040EBD0, &args![ground, share]).f32();
        let position = e.call(FUNCTION_00436AA0, &args![this]).u32();
        copy_vector3(e, position, out.addr());
        let height = e.mem.f32(out.addr() + 8);
        e.mem
            .set_f32(out.addr() + 8, (height as f64 + raise as f64) as f32);
        return true;
    }
    found
}

/// `TESActorBase::GetSex` (Xbox PDB): 1 for female.
const ACTOR_BASE_GET_SEX: u32 = 0x005f_0cc0;
/// `Actor::GetAnimAction` (Xbox PDB).
const ACTOR_GET_ANIM_ACTION: u32 = 0x008a_7570;
/// `Actor::GetEyeVector` (Xbox PDB) `(actor, position out, direction out, flag)`.
const ACTOR_GET_EYE_VECTOR: u32 = 0x008a_ff80;
/// `Actor::GetIronSights` (Xbox PDB).
const ACTOR_GET_IRON_SIGHTS: u32 = 0x008b_bc10;
/// `Actor::SetIronSights` (Xbox PDB) `(actor, flag, flag, flag)`.
const ACTOR_SET_IRON_SIGHTS: u32 = 0x008b_b650;
/// Returns `this + 0x44` (the embedded extra data list of a reference).
const ADD_44_TO_ADDRESS: u32 = 0x005d_43c0;
/// Returns `this + 4`.
const ADD_4_TO_ADDRESS: u32 = 0x0071_7e50;
/// Returns `this + 0x8c`.
const ADD_8C_TO_ADDRESS: u32 = 0x0045_bb80;
/// `TESIdleManager::AddRootIdleArray` (Xbox PDB) `(manager, path, 0, 0)`.
const ADD_ROOT_IDLE_ARRAY: u32 = 0x0060_0170;
/// `ModelLoader::BuildKFFileList` (Xbox PDB) `(loader, folder, 1, 0, 0xc)`.
const BUILD_FILE_LIST: u32 = 0x0044_7330;
/// `ViewCaster::GetWorld` (Xbox PDB).
const CASTER_GET_WORLD: u32 = 0x0062_1440;
/// The setter paired with [`CASTER_GET_WORLD`] `(caster, world)`.
const CASTER_SET_WORLD: u32 = 0x0062_1370;
/// `StartMenu::ChooseMainMenu` (Xbox PDB).
const CHOOSE_MAIN_MENU: u32 = 0x007d_0a70;
/// Control state query `(controls, control, state)`; answers non-zero when it holds.
const CONTROL_QUERY: u32 = 0x00a2_4660;
/// Copies the 16 bytes at the argument into `this + 0x100`.
const COPY_16_BYTES_TO_OFFSET_100: u32 = 0x0071_2e60;
/// `ModelLoader::CopyFilenameList` (Xbox PDB) `(loader, names, list)`.
const COPY_FILENAME_LIST: u32 = 0x0044_7850;
/// `30.0` (`double`).
const DOUBLE_30: u32 = 0x0101_db88;
/// `5.0` (`double`).
const DOUBLE_5: u32 = 0x0102_0998;
/// `96.0` (`double`).
const DOUBLE_96: u32 = 0x0107_2f98;
/// `0.5` (`double`).
const DOUBLE_HALF: u32 = 0x0101_1588;
/// `-1.0` (`double`).
const DOUBLE_MINUS_ONE: u32 = 0x0101_a6b0;
/// `1.0` (`double`).
const DOUBLE_ONE: u32 = 0x0101_2070;
/// `2.0` (`double`).
const DOUBLE_TWO: u32 = 0x0101_1590;
/// `2 * pi` (`double`).
const DOUBLE_TWO_PI: u32 = 0x0101_ff48;
/// `BaseExtraList::RemoveExtra_ov2` (Xbox PDB) `(list, extra type)`.
const EXTRA_LIST_REMOVE_EXTRA: u32 = 0x0041_0140;
/// `FindFirstCollisionObject` (Xbox PDB), cdecl `(3D)`.
const FIND_FIRST_COLLISION_OBJECT: u32 = 0x004b_5260;
/// `TESObjectREFR::FindReferenceFor3D` (Xbox PDB), cdecl `(3D)`.
const FIND_REFERENCE_FOR_3D: u32 = 0x0056_f930;
/// Function-local static: the first of the two state objects of [`fn_0095de30`].
const FIRST_STATE_OBJECT_SLOT: u32 = 0x011e_0d54;
/// Function-local static: the second of the two state objects of [`fn_0095de30`].
const SECOND_STATE_OBJECT_SLOT: u32 = 0x011e_0d50;
/// Reads the byte at +4 (the form type of a `TESForm`).
const FORM_GET_TYPE: u32 = 0x0040_1170;
/// Byte flag written by the field-of-view update ([`fn_0095de30`]).
const FOV_MOVING_FLAG: u32 = 0x011a_3b31;
/// `GetAVObjectForCollidable` (Xbox PDB), cdecl.
const GET_AV_OBJECT_FOR_COLLIDABLE: u32 = 0x004b_5820;
/// Reads the signed byte at +0xf4.
const GET_BYTE_AT_F4: u32 = 0x0044_6390;
/// Reads the byte at +0x384.
const GET_BYTE_AT_384: u32 = 0x0049_38c0;
/// `MobileObject::GetCharController` (Xbox PDB).
const GET_CHAR_CONTROLLER: u32 = 0x0093_06d0;
/// Reads the dword at +0x28.
const GET_DWORD_AT_28: u32 = 0x0045_cd60;
/// Reads the dword at +0x20, or the address `011f4288` when it is 0.
const GET_FIELD_20_OR_DEFAULT: u32 = 0x0043_d450;
/// `TESObjectREFR::GetInventoryItem` (Xbox PDB) `(reference, form, extra)`.
const GET_INVENTORY_ITEM_REF: u32 = 0x0057_6260;
/// `Interface::GetMessageMenuResult` (Xbox PDB): the answered button.
const GET_MESSAGE_MENU_RESULT: u32 = 0x0070_3fa0;
/// `TESObjectREFR::GetModel` (Xbox PDB): the reference's model path string.
const GET_MODEL_PATH: u32 = 0x0057_15d0;
/// `TESIdleManager::GetRootFilenameList` (Xbox PDB) `(manager, path, 0)`.
const GET_ROOT_FILENAME_LIST: u32 = 0x0060_0700;
/// `hkpEntity::activate` (Xbox PDB).
const HKP_ENTITY_ACTIVATE: u32 = 0x00c9_c1d0;
/// `hkVector4::setTransformedPos` (Xbox PDB) `(out, transform, position)`.
const HK_SET_TRANSFORMED_POS: u32 = 0x00c7_f640;
/// `HUDMainMenu::SetMenuMode` (Xbox PDB), cdecl.
const HUD_SET_MENU_MODE: u32 = 0x0077_1700;
/// Global pointer to the idle manager.
const IDLE_MANAGER_POINTER: u32 = 0x011c_b6a0;
/// `Interface::GetPipboy` (Xbox PDB).
const INTERFACE_GET_PIPBOY: u32 = 0x0070_5990;
/// `Interface::IsInMenuMode` (Xbox PDB).
const INTERFACE_IS_IN_MENU_MODE: u32 = 0x0070_2360;
/// `ItemChange::GetPoison` (Xbox PDB).
const ITEM_CHANGE_GET_POISON: u32 = 0x004b_dcc0;
/// `ItemChange::HasModEffectActive` (Xbox PDB) `(change, mod, float out)`.
const ITEM_CHANGE_HAS_MOD_EFFECT: u32 = 0x004b_d8d0;
/// Removes an item from a `BSSimpleList` `(list, &item)`.
const LIST_REMOVE: u32 = 0x0090_5330;
/// `BGSSaveLoadManager::LoadMostRecentSaveGame` (Xbox PDB).
const LOAD_MOST_RECENT_SAVE_GAME: u32 = 0x0085_12f0;
/// `MagicTarget::IsSpellTarget` (Xbox PDB) `(target, item, flag)`.
const MAGIC_TARGET_IS_SPELL_TARGET: u32 = 0x0082_2b90;
/// Global pointer to the model loader.
const MODEL_LOADER_POINTER: u32 = 0x011c_3b3c;
/// `bhkMouseSpringAction::bhkMouseSpringAction` (Xbox PDB) `(block, parameters)`.
const MOUSE_SPRING_ACTION_NEW: u32 = 0x004b_5040;
/// `NiPoint3::UnitizeGetLength` (Xbox PDB).
const NI_POINT3_UNITIZE_GET_LENGTH: u32 = 0x0045_7910;
/// `TESObjectREFR::GetScale` (Xbox PDB).
const OBJECT_GET_SCALE: u32 = 0x0056_7400;
/// `bhkPhantom::Remove` (Xbox PDB).
const PHANTOM_REMOVE: u32 = 0x00c9_f420;
/// Returns the process (`pCurrentProcess`, +0x68) of the actor.
const PLAYER_GET_PROCESS: u32 = 0x008d_8520;
/// The player's 3D root: the first-person one (argument 1) or the third-person one.
const PLAYER_GET_ROOT_NODE: u32 = 0x0095_0bb0;
/// `PlayerCharacter::IsDemigodMode` (Xbox PDB).
const PLAYER_IS_DEMIGOD_MODE: u32 = 0x0095_26f0;
/// `PlayerCharacter::IsGodMode` (Xbox PDB).
const PLAYER_IS_GOD_MODE: u32 = 0x0095_26b0;
/// `PlayerCharacter::IsPipboyActive` (Xbox PDB).
const PLAYER_IS_PIPBOY_ACTIVE: u32 = 0x0096_7ae0;
/// `PlayerCharacter::RemovePlayerAction` (Xbox PDB) `(player, kind, flag)`.
const PLAYER_REMOVE_PLAYER_ACTION: u32 = 0x0096_3e00;
/// `PlayerCharacter::StopVanityMode` (Xbox PDB).
const PLAYER_STOP_VANITY_MODE: u32 = 0x0095_00a0;
/// `PlayerCharacter::UpdateCamera` (Xbox PDB) `(player, flag, flag)`.
const PLAYER_UPDATE_CAMERA: u32 = 0x0094_ae40;
/// `ModelLoader::QueueModel` (Xbox PDB).
const QUEUE_MODEL: u32 = 0x0044_4040;
/// Reads the dword at the address (the pointer a `NiPointer` holds).
const READ_DWORD_AT_0: u32 = 0x0055_9450;
/// Reads the dword at +0x15c.
const READ_DWORD_AT_15C: u32 = 0x008d_85e0;
/// Reads the dword at +0x20.
const READ_DWORD_AT_20: u32 = 0x007a_f430;
/// Reads the dword at +0x40.
const READ_DWORD_AT_40: u32 = 0x008d_6f30;
/// Reads the dword at +0xac.
const READ_DWORD_AT_AC: u32 = 0x0066_29f0;
/// Reads the dword at +0xc (the form id of a `TESForm`).
const READ_DWORD_AT_C: u32 = 0x0084_e3a0;
/// Reads the dword at +4 (the next node of a `BSSimpleList` node, the count of an `ItemChange`).
const READ_FIELD_4: u32 = 0x0072_6070;
/// Reads the float at +0x110, returned in ST0.
const READ_FLOAT_AT_110: u32 = 0x0050_8070;
/// Reads the float at +0xbc, returned in ST0.
const READ_FLOAT_AT_BC: u32 = 0x0099_e040;
/// Reads the float at +0xc, returned in ST0.
const READ_FLOAT_AT_C: u32 = 0x0084_d030;
/// `Script::SetActionFlag` (Xbox PDB), cdecl `(0, extra data list, flag)`.
const SCRIPT_SET_ACTION_FLAG: u32 = 0x005a_c750;
/// Pointer to the value of a boolean setting object.
const SETTING_BOOL_POINTER: u32 = 0x0040_8d60;
/// The float value of a setting object, in ST0.
const SETTING_FLOAT_VALUE: u32 = 0x0045_0410;
/// Stores its byte argument at +4.
const SET_BYTE_AT_4: u32 = 0x004f_15a0;
/// `NiPointer` assignment `(slot, new)`: releases the old, takes the new.
const SMART_POINTER_ASSIGN: u32 = 0x0066_b0d0;
/// `bhkSphereShape::bhkSphereShape` (Xbox PDB) `(block, radius, flag)`.
const SPHERE_SHAPE_NEW: u32 = 0x0056_e950;
/// `sprintf` (cdecl).
const SPRINTF: u32 = 0x00ec_623a;
/// Function of the state object `(object, float)`.
const STATE_OBJECT_ADVANCE: u32 = 0x0094_6140;
/// Function of the state object, answers a flag.
const STATE_OBJECT_CHECK: u32 = 0x0094_6280;
/// Function of the state object, answers a float in ST0.
const STATE_OBJECT_GET: u32 = 0x0094_6190;
/// Function-local statics' init guard of [`fn_0095de30`] (bit 0 and bit 1).
const STATE_OBJECT_GUARD: u32 = 0x011e_0d58;
/// Constructor of the 0x1a0-byte state object.
const STATE_OBJECT_NEW: u32 = 0x0093_9880;
/// Function of the state object `(object, float, float, float)`.
const STATE_OBJECT_SET: u32 = 0x0094_6020;
/// Tests bit 1 of the dword at +0x244.
const TEST_BIT_2_AT_244: u32 = 0x0042_ce10;
/// Tests bit 1 of the byte at +0x100.
const TEST_BIT_2_BYTE_AT_100: u32 = 0x0052_4b40;
/// Tests whether the dword at +8 is not zero.
const TEST_DWORD_AT_8: u32 = 0x0052_5430;
/// Length of the `NiPoint3` at `this`, in ST0.
const VECTOR_LENGTH: u32 = 0x0045_7990;
/// `NiPoint3` subtraction `(this, out, other)`.
const VECTOR_SUBTRACT: u32 = 0x0043_9ef0;
/// `bhkWorldObject::Add` (Xbox PDB).
const WORLD_OBJECT_ADD: u32 = 0x00c8_5810;
/// `0.0` (`double`).
const ZERO_DOUBLE: u32 = 0x0101_2060;
/// The function at `008846e0` (called as its callers do).
const ACTOR_FUNCTION_008846E0: u32 = 0x0088_46e0;
/// The function at `008a89a0` (called as its callers do).
const ACTOR_FUNCTION_008A89A0: u32 = 0x008a_89a0;
/// The function at `00aa13e0` (called as its callers do).
const ALLOCATE_00AA13E0: u32 = 0x00aa_13e0;
/// The function at `004b5400` (called as its callers do).
const BODY_VALUE_004B5400: u32 = 0x004b_5400;
/// The function at `004ae750` (called as its callers do).
const BODY_LOOKUP_004AE750: u32 = 0x004a_e750;
/// The `float` constant at `01016264`.
const DEFAULT_VALUE_01016264: u32 = 0x0101_6264;
/// The `double` constant at `01018a90`.
const F64_CONSTANT_01018A90: u32 = 0x0101_8a90;
/// The `double` constant at `0101ffa0`.
const F64_CONSTANT_0101FFA0: u32 = 0x0101_ffa0;
/// The byte flag at `011e07b8`.
const FLAG_011E07B8: u32 = 0x011e_07b8;
/// The byte flag at `011e07c0`.
const FLAG_011E07C0: u32 = 0x011e_07c0;
/// The byte flag at `011e0d5c`.
const FLAG_011E0D5C: u32 = 0x011e_0d5c;
/// The `float` constant at `01013974`.
const F32_CONSTANT_01013974: u32 = 0x0101_3974;
/// The `float` constant at `01017868`.
const F32_CONSTANT_01017868: u32 = 0x0101_7868;
/// The `float` constant at `0101e6ec`.
const F32_CONSTANT_0101E6EC: u32 = 0x0101_e6ec;
/// The `float` constant at `011e0d60`.
const F32_CONSTANT_011E0D60: u32 = 0x011e_0d60;
/// The `float` constant at `00408840`.
const FLOAT_FUNCTION_00408840: u32 = 0x0040_8840;
/// The `float` constant at `005dc330`.
const FLOAT_FUNCTION_005DC330: u32 = 0x005d_c330;
/// The string at `0108b3b4`.
const FORMAT_0108B3B4: u32 = 0x0108_b3b4;
/// The function at `00404010` (called as its callers do).
const FUNCTION_00404010: u32 = 0x0040_4010;
/// The function at `0040ebd0` (called as its callers do).
const FUNCTION_0040EBD0: u32 = 0x0040_ebd0;
/// The function at `00413f40` (called as its callers do).
const FUNCTION_00413F40: u32 = 0x0041_3f40;
/// The function at `00416870` (called as its callers do).
const FUNCTION_00416870: u32 = 0x0041_6870;
/// The function at `004213c0` (called as its callers do).
const FUNCTION_004213C0: u32 = 0x0042_13c0;
/// The function at `00422750` (called as its callers do).
const FUNCTION_00422750: u32 = 0x0042_2750;
/// The function at `00436aa0` (called as its callers do).
const FUNCTION_00436AA0: u32 = 0x0043_6aa0;
/// The function at `00439180` (called as its callers do).
const FUNCTION_00439180: u32 = 0x0043_9180;
/// The function at `00439e90` (called as its callers do).
const FUNCTION_00439E90: u32 = 0x0043_9e90;
/// The function at `00439ef0` (called as its callers do).
const FUNCTION_00439EF0: u32 = 0x0043_9ef0;
/// The function at `0043b4a0` (called as its callers do).
const FUNCTION_0043B4A0: u32 = 0x0043_b4a0;
/// The function at `0043b4d0` (called as its callers do).
const FUNCTION_0043B4D0: u32 = 0x0043_b4d0;
/// The function at `0043b4f0` (called as its callers do).
const FUNCTION_0043B4F0: u32 = 0x0043_b4f0;
/// The function at `00444d00` (called as its callers do).
const FUNCTION_00444D00: u32 = 0x0044_4d00;
/// The function at `00446500` (called as its callers do).
const FUNCTION_00446500: u32 = 0x0044_6500;
/// The function at `00457990` (called as its callers do).
const FUNCTION_00457990: u32 = 0x0045_7990;
/// The function at `00458620` (called as its callers do).
const FUNCTION_00458620: u32 = 0x0045_8620;
/// The function at `004586d0` (called as its callers do).
const FUNCTION_004586D0: u32 = 0x0045_86d0;
/// The function at `004587d0` (called as its callers do).
const FUNCTION_004587D0: u32 = 0x0045_87d0;
/// The function at `0045bb20` (called as its callers do).
const FUNCTION_0045BB20: u32 = 0x0045_bb20;
/// The function at `0045cec0` (called as its callers do).
const FUNCTION_0045CEC0: u32 = 0x0045_cec0;
/// The function at `00461130` (called as its callers do).
const FUNCTION_00461130: u32 = 0x0046_1130;
/// The function at `00464f30` (called as its callers do).
const FUNCTION_00464F30: u32 = 0x0046_4f30;
/// The function at `004a0c10` (called as its callers do).
const FUNCTION_004A0C10: u32 = 0x004a_0c10;
/// The function at `004a0c90` (called as its callers do).
const FUNCTION_004A0C90: u32 = 0x004a_0c90;
/// The function at `004a39f0` (called as its callers do).
const FUNCTION_004A39F0: u32 = 0x004a_39f0;
/// The function at `004a3a20` (called as its callers do).
const FUNCTION_004A3A20: u32 = 0x004a_3a20;
/// The function at `004a3c20` (called as its callers do).
const FUNCTION_004A3C20: u32 = 0x004a_3c20;
/// The function at `004a3da0` (called as its callers do).
const FUNCTION_004A3DA0: u32 = 0x004a_3da0;
/// The function at `004a3e00` (called as its callers do).
const FUNCTION_004A3E00: u32 = 0x004a_3e00;
/// The function at `004a3eb0` (called as its callers do).
const FUNCTION_004A3EB0: u32 = 0x004a_3eb0;
/// The function at `004a3f10` (called as its callers do).
const FUNCTION_004A3F10: u32 = 0x004a_3f10;
/// The function at `004a3f70` (called as its callers do).
const FUNCTION_004A3F70: u32 = 0x004a_3f70;
/// The function at `004a3fb0` (called as its callers do).
const FUNCTION_004A3FB0: u32 = 0x004a_3fb0;
/// The function at `004b3ab0` (called as its callers do).
const FUNCTION_004B3AB0: u32 = 0x004b_3ab0;
/// The function at `004b4500` (called as its callers do).
const FUNCTION_004B4500: u32 = 0x004b_4500;
/// The function at `004b4ab0` (called as its callers do).
const FUNCTION_004B4AB0: u32 = 0x004b_4ab0;
/// The function at `004b4d10` (called as its callers do).
const FUNCTION_004B4D10: u32 = 0x004b_4d10;
/// The function at `004b4d50` (called as its callers do).
const FUNCTION_004B4D50: u32 = 0x004b_4d50;
/// The function at `004b4db0` (called as its callers do).
const FUNCTION_004B4DB0: u32 = 0x004b_4db0;
/// The function at `004b4e50` (called as its callers do).
const FUNCTION_004B4E50: u32 = 0x004b_4e50;
/// The function at `004b4ea0` (called as its callers do).
const FUNCTION_004B4EA0: u32 = 0x004b_4ea0;
/// The function at `004b4ec0` (called as its callers do).
const FUNCTION_004B4EC0: u32 = 0x004b_4ec0;
/// The function at `004b4f00` (called as its callers do).
const FUNCTION_004B4F00: u32 = 0x004b_4f00;
/// The function at `004b52f0` (called as its callers do).
const FUNCTION_004B52F0: u32 = 0x004b_52f0;
/// The function at `004b5510` (called as its callers do).
const FUNCTION_004B5510: u32 = 0x004b_5510;
/// The function at `004b5a20` (called as its callers do).
const FUNCTION_004B5A20: u32 = 0x004b_5a20;
/// The function at `004b5a80` (called as its callers do).
const FUNCTION_004B5A80: u32 = 0x004b_5a80;
/// The function at `00517670` (called as its callers do).
const FUNCTION_00517670: u32 = 0x0051_7670;
/// The function at `00553f70` (called as its callers do).
const FUNCTION_00553F70: u32 = 0x0055_3f70;
/// The function at `00553fc0` (called as its callers do).
const FUNCTION_00553FC0: u32 = 0x0055_3fc0;
/// The function at `00560d30` (called as its callers do).
const FUNCTION_00560D30: u32 = 0x0056_0d30;
/// The function at `00560d80` (called as its callers do).
const FUNCTION_00560D80: u32 = 0x0056_0d80;
/// The function at `00561690` (called as its callers do).
const FUNCTION_00561690: u32 = 0x0056_1690;
/// The function at `0056ded0` (called as its callers do).
const FUNCTION_0056DED0: u32 = 0x0056_ded0;
/// The function at `0056e090` (called as its callers do).
const FUNCTION_0056E090: u32 = 0x0056_e090;
/// The function at `0056e120` (called as its callers do).
const FUNCTION_0056E120: u32 = 0x0056_e120;
/// The function at `0056e2d0` (called as its callers do).
const FUNCTION_0056E2D0: u32 = 0x0056_e2d0;
/// The function at `0056ea90` (called as its callers do).
const FUNCTION_0056EA90: u32 = 0x0056_ea90;
/// The function at `0059ce80` (called as its callers do).
const FUNCTION_0059CE80: u32 = 0x0059_ce80;
/// The function at `005dbf20` (called as its callers do).
const FUNCTION_005DBF20: u32 = 0x005d_bf20;
/// The function at `005f5950` (called as its callers do).
const FUNCTION_005F5950: u32 = 0x005f_5950;
/// The function at `00622290` (called as its callers do).
const FUNCTION_00622290: u32 = 0x0062_2290;
/// The function at `00623150` (called as its callers do).
const FUNCTION_00623150: u32 = 0x0062_3150;
/// The function at `00624070` (called as its callers do).
const FUNCTION_00624070: u32 = 0x0062_4070;
/// The function at `00624b70` (called as its callers do).
const FUNCTION_00624B70: u32 = 0x0062_4b70;
/// The function at `00627920` (called as its callers do).
const FUNCTION_00627920: u32 = 0x0062_7920;
/// The function at `0062b8d0` (called as its callers do).
const FUNCTION_0062B8D0: u32 = 0x0062_b8d0;
/// The function at `0062c300` (called as its callers do).
const FUNCTION_0062C300: u32 = 0x0062_c300;
/// The function at `0062d310` (called as its callers do).
const FUNCTION_0062D310: u32 = 0x0062_d310;
/// The function at `0062d350` (called as its callers do).
const FUNCTION_0062D350: u32 = 0x0062_d350;
/// The function at `0062d4b0` (called as its callers do).
const FUNCTION_0062D4B0: u32 = 0x0062_d4b0;
/// The function at `00630b40` (called as its callers do).
const FUNCTION_00630B40: u32 = 0x0063_0b40;
/// The function at `0063c8a0` (called as its callers do).
const FUNCTION_0063C8A0: u32 = 0x0063_c8a0;
/// The function at `00696c30` (called as its callers do).
const FUNCTION_00696C30: u32 = 0x0069_6c30;
/// The function at `006fa820` (called as its callers do).
const FUNCTION_006FA820: u32 = 0x006f_a820;
/// The function at `00703350` (called as its callers do).
const FUNCTION_00703350: u32 = 0x0070_3350;
/// The function at `007033d0` (called as its callers do).
const FUNCTION_007033D0: u32 = 0x0070_33d0;
/// The function at `00703430` (called as its callers do).
const FUNCTION_00703430: u32 = 0x0070_3430;
/// The function at `00703e80` (called as its callers do).
const FUNCTION_00703E80: u32 = 0x0070_3e80;
/// The function at `00704af0` (called as its callers do).
const FUNCTION_00704AF0: u32 = 0x0070_4af0;
/// The function at `00705800` (called as its callers do).
const FUNCTION_00705800: u32 = 0x0070_5800;
/// The function at `00705a90` (called as its callers do).
const FUNCTION_00705A90: u32 = 0x0070_5a90;
/// The function at `00705ad0` (called as its callers do).
const FUNCTION_00705AD0: u32 = 0x0070_5ad0;
/// The function at `007d0b90` (called as its callers do).
const FUNCTION_007D0B90: u32 = 0x007d_0b90;
/// The function at `007f3bd0` (called as its callers do).
const FUNCTION_007F3BD0: u32 = 0x007f_3bd0;
/// The function at `007f68a0` (called as its callers do).
const FUNCTION_007F68A0: u32 = 0x007f_68a0;
/// The function at `007fa8d0` (called as its callers do).
const FUNCTION_007FA8D0: u32 = 0x007f_a8d0;
/// The function at `00822f70` (called as its callers do).
const FUNCTION_00822F70: u32 = 0x0082_2f70;
/// The function at `008248e0` (called as its callers do).
const FUNCTION_008248E0: u32 = 0x0082_48e0;
/// The function at `00825c00` (called as its callers do).
const FUNCTION_00825C00: u32 = 0x0082_5c00;
/// The function at `0082aeb0` (called as its callers do).
const FUNCTION_0082AEB0: u32 = 0x0082_aeb0;
/// The function at `0082aee0` (called as its callers do).
const FUNCTION_0082AEE0: u32 = 0x0082_aee0;
/// The function at `008397d0` (called as its callers do).
const FUNCTION_008397D0: u32 = 0x0083_97d0;
/// The function at `00877720` (called as its callers do).
const FUNCTION_00877720: u32 = 0x0087_7720;
/// The function at `0087f4c0` (called as its callers do).
const FUNCTION_0087F4C0: u32 = 0x0087_f4c0;
/// The function at `008a6840` (called as its callers do).
const FUNCTION_008A6840: u32 = 0x008a_6840;
/// The function at `008c1d20` (called as its callers do).
const FUNCTION_008C1D20: u32 = 0x008c_1d20;
/// The function at `008c5d90` (called as its callers do).
const FUNCTION_008C5D90: u32 = 0x008c_5d90;
/// The function at `008c71b0` (called as its callers do).
const FUNCTION_008C71B0: u32 = 0x008c_71b0;
/// The function at `00931ed0` (called as its callers do).
const FUNCTION_00931ED0: u32 = 0x0093_1ed0;
/// The function at `00933890` (called as its callers do).
const FUNCTION_00933890: u32 = 0x0093_3890;
/// The function at `0094c950` (called as its callers do).
const FUNCTION_0094C950: u32 = 0x0094_c950;
/// The function at `00950610` (called as its callers do).
const FUNCTION_00950610: u32 = 0x0095_0610;
/// The function at `00963eb0` (called as its callers do).
const FUNCTION_00963EB0: u32 = 0x0096_3eb0;
/// The function at `0096a350` (called as its callers do).
const FUNCTION_0096A350: u32 = 0x0096_a350;
/// The function at `0096a370` (called as its callers do).
const FUNCTION_0096A370: u32 = 0x0096_a370;
/// The function at `00b68770` (called as its callers do).
const FUNCTION_00B68770: u32 = 0x00b6_8770;
/// The function at `00c66fb0` (called as its callers do).
const FUNCTION_00C66FB0: u32 = 0x00c6_6fb0;
/// The function at `00c66fd0` (called as its callers do).
const FUNCTION_00C66FD0: u32 = 0x00c6_6fd0;
/// The function at `00c6e280` (called as its callers do).
const FUNCTION_00C6E280: u32 = 0x00c6_e280;
/// The function at `00c6fd20` (called as its callers do).
const FUNCTION_00C6FD20: u32 = 0x00c6_fd20;
/// The function at `00c8e940` (called as its callers do).
const FUNCTION_00C8E940: u32 = 0x00c8_e940;
/// The function at `00c9c380` (called as its callers do).
const FUNCTION_00C9C380: u32 = 0x00c9_c380;
/// The function at `00c9c390` (called as its callers do).
const FUNCTION_00C9C390: u32 = 0x00c9_c390;
/// The function at `0050ebf0` (called as its callers do).
const GET_BOUND_SIZE_0050EBF0: u32 = 0x0050_ebf0;
/// The data at `011cc5ec`.
const GLOBAL_011CC5EC: u32 = 0x011c_c5ec;
/// The data at `011e0768`.
const GLOBAL_011E0768: u32 = 0x011e_0768;
/// The function at `004a3bc0` (called as its callers do).
const HIT_COLLECTOR_DELETE_004A3BC0: u32 = 0x004a_3bc0;
/// The function at `004a46b0` (called as its callers do).
const HIT_COLLECTOR_GET_004A46B0: u32 = 0x004a_46b0;
/// The function at `004a3a70` (called as its callers do).
const HIT_COLLECTOR_NEW_004A3A70: u32 = 0x004a_3a70;
/// The function at `00460140` (called as its callers do).
const IDENTITY_00460140: u32 = 0x0046_0140;
/// The function at `006815c0` (called as its callers do).
const IDENTITY_006815C0: u32 = 0x0068_15c0;
/// The function at `004bdd20` (called as its callers do).
const ITEM_CHANGE_FUNCTION_004BDD20: u32 = 0x004b_dd20;
/// The data at `011e0d4c`.
const LAST_OBJECT_011E0D4C: u32 = 0x011e_0d4c;
/// The data at `011f2250`.
const OBJECT_011F2250: u32 = 0x011f_2250;
/// The data at `011f6394`.
const OBJECT_011F6394: u32 = 0x011f_6394;
/// The data at `004543c0`.
const OBJECT_FUNCTION_004543C0: u32 = 0x0045_43c0;
/// The data at `005533c0`.
const OBJECT_FUNCTION_005533C0: u32 = 0x0055_33c0;
/// The function at `004b4f40` (called as its callers do).
const PARAMS_NEW_004B4F40: u32 = 0x004b_4f40;
/// The function at `00c9eec0` (called as its callers do).
const PHANTOM_DELETE_00C9EEC0: u32 = 0x00c9_eec0;
/// The global at `011ca27c`, holding a pointer.
const POINTER_011CA27C: u32 = 0x011c_a27c;
/// The global at `011ca280`, holding a pointer.
const POINTER_011CA280: u32 = 0x011c_a280;
/// The global at `011ddf38`, holding a pointer.
const POINTER_011DDF38: u32 = 0x011d_df38;
/// The global at `011de134`, holding a pointer.
const POINTER_011DE134: u32 = 0x011d_e134;
/// The global at `011dea0c`, holding a pointer.
const POINTER_011DEA0C: u32 = 0x011d_ea0c;
/// The global at `011e07d0`, holding a pointer.
const POINTER_011E07D0: u32 = 0x011e_07d0;
/// The global at `011f21cc`, holding a pointer.
const POINTER_011F21CC: u32 = 0x011f_21cc;
/// The setting object at `011cdd78`.
const SETTING_011CDD78: u32 = 0x011c_dd78;
/// The setting object at `011ce344`.
const SETTING_011CE344: u32 = 0x011c_e344;
/// The setting object at `011ce5a0`.
const SETTING_011CE5A0: u32 = 0x011c_e5a0;
/// The setting object at `011ce870`.
const SETTING_011CE870: u32 = 0x011c_e870;
/// The setting object at `011cea44`.
const SETTING_011CEA44: u32 = 0x011c_ea44;
/// The setting object at `011cfe2c`.
const SETTING_011CFE2C: u32 = 0x011c_fe2c;
/// The setting object at `011cfeec`.
const SETTING_011CFEEC: u32 = 0x011c_feec;
/// The setting object at `011cffc4`.
const SETTING_011CFFC4: u32 = 0x011c_ffc4;
/// The setting object at `011d0048`.
const SETTING_011D0048: u32 = 0x011d_0048;
/// The setting object at `011d0220`.
const SETTING_011D0220: u32 = 0x011d_0220;
/// The setting object at `011d0358`.
const SETTING_011D0358: u32 = 0x011d_0358;
/// The setting object at `011d037c`.
const SETTING_011D037C: u32 = 0x011d_037c;
/// The setting object at `011d04d8`.
const SETTING_011D04D8: u32 = 0x011d_04d8;
/// The setting object at `011d0628`.
const SETTING_011D0628: u32 = 0x011d_0628;
/// The setting object at `011d0664`.
const SETTING_011D0664: u32 = 0x011d_0664;
/// The setting object at `011d0808`.
const SETTING_011D0808: u32 = 0x011d_0808;
/// The setting object at `011d08c8`.
const SETTING_011D08C8: u32 = 0x011d_08c8;
/// The setting object at `011d0e4c`.
const SETTING_011D0E4C: u32 = 0x011d_0e4c;
/// The setting object at `011d0ffc`.
const SETTING_011D0FFC: u32 = 0x011d_0ffc;
/// The setting object at `011d1230`.
const SETTING_011D1230: u32 = 0x011d_1230;
/// The setting object at `011d1260`.
const SETTING_011D1260: u32 = 0x011d_1260;
/// The setting object at `011d1308`.
const SETTING_011D1308: u32 = 0x011d_1308;
/// The setting object at `011d1604`.
const SETTING_011D1604: u32 = 0x011d_1604;
/// The setting object at `011d1728`.
const SETTING_011D1728: u32 = 0x011d_1728;
/// The setting object at `011d177c`.
const SETTING_011D177C: u32 = 0x011d_177c;
/// The setting object at `011d183c`.
const SETTING_011D183C: u32 = 0x011d_183c;
/// The setting object at `011d18a8`.
const SETTING_011D18A8: u32 = 0x011d_18a8;
/// The setting object at `011d1a08`.
const SETTING_011D1A08: u32 = 0x011d_1a08;
/// The setting object at `011d1b44`.
const SETTING_011D1B44: u32 = 0x011d_1b44;
/// The setting object at `011d1c7c`.
const SETTING_011D1C7C: u32 = 0x011d_1c7c;
/// The setting object at `011d2718`.
const SETTING_011D2718: u32 = 0x011d_2718;
/// The setting object at `011d2748`.
const SETTING_011D2748: u32 = 0x011d_2748;
/// The setting object at `011d2814`.
const SETTING_011D2814: u32 = 0x011d_2814;
/// The setting object at `011d34f8`.
const SETTING_011D34F8: u32 = 0x011d_34f8;
/// The setting object at `011d3684`.
const SETTING_011D3684: u32 = 0x011d_3684;
/// The setting object at `011d38b8`.
const SETTING_011D38B8: u32 = 0x011d_38b8;
/// The setting object at `011d3e44`.
const SETTING_011D3E44: u32 = 0x011d_3e44;
/// The setting object at `011d48dc`.
const SETTING_011D48DC: u32 = 0x011d_48dc;
/// The setting object at `011d4a2c`.
const SETTING_011D4A2C: u32 = 0x011d_4a2c;
/// The setting object at `011d4e94`.
const SETTING_011D4E94: u32 = 0x011d_4e94;
/// The setting object at `011e0970`.
const SETTING_011E0970: u32 = 0x011e_0970;
/// The setting object at `011e097c`.
const SETTING_011E097C: u32 = 0x011e_097c;
/// The setting object at `011e0b1c`.
const SETTING_011E0B1C: u32 = 0x011e_0b1c;
/// The setting object at `011e0b84`.
const SETTING_011E0B84: u32 = 0x011e_0b84;
/// The setting object at `01203150`.
const SETTING_01203150: u32 = 0x0120_3150;
/// The setting object at `0120315c`.
const SETTING_0120315C: u32 = 0x0120_315c;
/// The setting object at `01203168`.
const SETTING_01203168: u32 = 0x0120_3168;
/// The setting object at `01203174`.
const SETTING_01203174: u32 = 0x0120_3174;
/// The function at `00633c90` (called as its callers do).
const SMART_POINTER_ASSIGN_00633C90: u32 = 0x0063_3c90;
/// The data at `011ad840`.
const SOURCE_011AD840: u32 = 0x011a_d840;
/// The function at `00408da0` (called as its callers do).
const STRING_OR_EMPTY_00408DA0: u32 = 0x0040_8da0;
/// The string at `01016f1c`.
const SUFFIX_01016F1C: u32 = 0x0101_6f1c;
/// The string at `01016f38`.
const SUFFIX_01016F38: u32 = 0x0101_6f38;
/// The string at `01016f58`.
const SUFFIX_01016F58: u32 = 0x0101_6f58;
/// The string at `010170ec`.
const SUFFIX_010170EC: u32 = 0x0101_70ec;
/// The string at `0108b0d8`.
const SUFFIX_0108B0D8: u32 = 0x0108_b0d8;
/// The string at `0108b3bc`.
const SUFFIX_0108B3BC: u32 = 0x0108_b3bc;
/// The string at `0108b3d8`.
const SUFFIX_0108B3D8: u32 = 0x0108_b3d8;
/// The data at `011a9484`.
const UP_VECTOR_011A9484: u32 = 0x011a_9484;

/// Offset of the `listNotes` (`BSSimpleList<BGSNote *>`, Xbox PDB) list: its
/// first node is embedded in the player object (PC `+0x5e4`).
const NOTES_LIST_OFFSET: u32 = 0x5e4;
/// `(list)`: the number of items in a `BSSimpleList` (the map names the
/// folded body `VATS::GetCount`).
const LIST_COUNT: u32 = 0x005a_e380;
/// `ImageSpaceModifierInstance::Stop` (Xbox PDB), cdecl `(instance)`.
const IMAGE_SPACE_INSTANCE_STOP: u32 = 0x0052_93f0;
/// `ImageSpaceModifierInstanceDOF::Trigger` (Xbox PDB), cdecl, six words.
const IMAGE_SPACE_DOF_TRIGGER: u32 = 0x0052_a150;
/// Cdecl, ten words: starts the VATS `ImageSpaceModifierInstanceDRB` and
/// returns it.
const IMAGE_SPACE_DRB_TRIGGER: u32 = 0x0052_a780;
/// `(instance, flag)`: sets or clears bit 0 of the instance's flags (+0x18).
const IMAGE_SPACE_INSTANCE_SET_ACTIVE: u32 = 0x005b_bd60;
/// `(float *value, minimum, maximum)`, cdecl: clamps `*value` into the range.
const CLAMP_FLOAT_IN_PLACE: u32 = 0x0053_30e0;
/// `NiPoint2::NiPoint2(x, y)` (`ECX` = the point), returns the point.
const NI_POINT2_NEW: u32 = 0x0045_2dc0;
/// The byte the iron sights update sets: 1 while the player aims through
/// the iron sights in first person.
const IRON_SIGHTS_ACTIVE: u32 = 0x011f_9423;
/// The `ActorMover` constructor `(this, owner)`, on a 0xa0 byte block.
const ACTOR_MOVER_NEW: u32 = 0x009e_9d00;
/// `(actor mover, mask, set)`: sets or clears the bits of `mask` in the word
/// at `+0x94` of the actor mover.
const ACTOR_MOVER_SET_FLAGS: u32 = 0x009e_a360;
/// `(holder, index)`: the 16-bit animation group id the holder keeps for
/// `index` (`+0x4c + index * 2`).
const ANIMATION_HOLDER_GET_GROUP: u32 = 0x0043_01b0;
/// `(object, mask)`: whether the object's flag word has any bit of `mask`.
const FLAGS_TEST: u32 = 0x004f_d420;
/// `(object)`: the dword at `+0xa0` of the object.
const READ_FIELD_A0: u32 = 0x004f_d400;
/// `(reference)`: the reference's base form (the folded `004181e0`).
const GET_BASE_FORM: u32 = 0x0041_81e0;
/// `(form)`: the float at `+0x68`.
const READ_FIELD_68: u32 = 0x0069_ef80;
/// Cdecl `(strength, a, b)` blending function.
const BLEND_FLOATS: u32 = 0x009c_3f50;
/// `(message)`, cdecl: logs a script warning.
const LOG_WARNING: u32 = 0x005b_5e40;
/// `BSStringT` constructor and destructor on a local (`ECX` = the string).
const STRING_NEW: u32 = 0x0040_37b0;
const STRING_DELETE: u32 = 0x0040_37d0;
/// `MapMarkerData::GetLocationName` (Xbox PDB, folded): `ECX` = the note's
/// name member; answers the string.
const GET_LOCATION_NAME: u32 = 0x0040_8da0;
/// `(string, format, text)`: formats into a `BSStringT`.
const STRING_FORMAT: u32 = 0x0040_6f60;
/// `ECX` = a `BSStringT`: its text pointer (the dword at `+0`).
const STRING_TEXT: u32 = 0x0055_9450;
/// `(text)`, cdecl: shows the text as a HUD message.
const SHOW_HUD_MESSAGE: u32 = 0x0070_3c00;
/// `(note)`, cdecl: takes a note out of the note display.
const NOTE_REMOVE_FROM_DISPLAY: u32 = 0x0070_4470;
/// The setting holding the format of the new note message.
const NOTE_FORMAT_SETTING: u32 = 0x011d_3ca0;
/// The text logged when a null note is added.
const NULL_NOTE_WARNING: u32 = 0x0108_b400;
/// The sound played when a note is added.
const NOTE_ADDED_SOUND: u32 = 0x0108_4dd4;
/// `BGSTerminal` type descriptor.
const TYPE_BGS_TERMINAL: u32 = 0x0118_9ebc;
/// `BGSTerminal::IsHacked` (Xbox PDB), cdecl `(reference)`.
const TERMINAL_IS_HACKED: u32 = 0x0050_1870;
/// `BGSTerminal::IsUnlocked` (Xbox PDB), `ECX` = terminal form, `(reference)`.
const TERMINAL_IS_UNLOCKED: u32 = 0x0050_1ae0;
/// `BGSTerminal::IsLockedOut` (Xbox PDB), cdecl `(reference)`.
const TERMINAL_IS_LOCKED_OUT: u32 = 0x0050_1990;
/// `BGSTerminal::GetRawDifficulty` (Xbox PDB), `ECX` = terminal form, `(reference)`.
const TERMINAL_GET_RAW_DIFFICULTY: u32 = 0x0050_1a30;
/// `BGSTerminal::IsHackEligible` (Xbox PDB), `ECX` = terminal form,
/// `(reference, skill)`.
const TERMINAL_IS_HACK_ELIGIBLE: u32 = 0x0050_1760;
/// `ActorValueOwner::GetClampedActorFloatValue` (Xbox PDB) `(owner, actor value)`.
const ACTOR_VALUE_GET_CLAMPED: u32 = 0x0066_ef50;
/// Offset of the `ActorValueOwner` base in an actor.
const ACTOR_VALUE_OWNER_OFFSET: u32 = 0xa4;
/// The global pointer to the form map the iteration below walks.
const FORM_MAP: u32 = 0x011c_54c0;
/// `(map)`: the first occupied bucket entry of the map.
const MAP_FIRST_ENTRY: u32 = 0x004b_9ba0;
/// `(map, &entry, &key, &value)`: reads the entry and steps to the next one.
const MAP_NEXT_ENTRY: u32 = 0x006b_7f20;
/// `(reference)`, cdecl: removes extra data 0x50 from the reference's list.
const REFERENCE_REMOVE_EXTRA_50: u32 = 0x0050_1a10;
/// The setting a steal or pickpocket warning timer takes when it is unset.
const WARNING_TIMER_SETTING: u32 = 0x011c_d5fc;
/// `bhkCharacterProxy` accessors (`ECX` = the controller from `009306d0`): the
/// word at `+0x0c` of the object the controller holds at `+0x630`, and the
/// word at `+0xc0` of the object it holds at `+0x638` (0 when that is null).
const CONTROLLER_BODY_AT_630: u32 = 0x00c6_fd40;
const CONTROLLER_BODY_AT_638: u32 = 0x00c6_fd60;
/// `(body)`: the scene graph object of a Havok body.
const BODY_GET_AV_OBJECT: u32 = 0x0081_0660;
/// `ActorsNearPlayerData` constructor on an 8 byte block.
const NEAR_PLAYER_DATA_NEW: u32 = 0x0047_81b0;
/// `BSSimpleList` constructor on an 8 byte block.
const LIST_NEW: u32 = 0x0096_a2d0;
/// `(node)`: removes the item of a list node: the next node's item and link move into it (an
/// only node is just cleared) and the next node is freed.
const LIST_REMOVE_NODE: u32 = 0x0063_f7b0;
/// `Actor::GetDetectionLevelAgainstActor` (Xbox PDB).
const ACTOR_GET_DETECTION_LEVEL: u32 = 0x008a_0d10;
/// `Actor::LineOfSight` (Xbox PDB).
const ACTOR_LINE_OF_SIGHT: u32 = 0x0088_b880;
/// `MiddleHighProcess::GetForceNextUpdate` (Xbox PDB), `ECX` = the actor.
const ACTOR_GET_FORCE_NEXT_UPDATE: u32 = 0x0056_6950;
/// `TESObjectREFR::GetDistanceFromReference` (Xbox PDB) `(from, other, 0, 0)`.
const REFERENCE_GET_DISTANCE: u32 = 0x0057_23b0;
/// `DialoguePackage::GetActorStartedConversation` (Xbox PDB).
const CONVERSATION_GET_STARTER: u32 = 0x009e_e040;
/// `DialoguePackage::GetTargetOfConversation` (Xbox PDB).
const CONVERSATION_GET_TARGET: u32 = 0x008d_80e0;
/// `TESObjectREFR::IsFurniture` (Xbox PDB).
const REFERENCE_IS_FURNITURE: u32 = 0x0056_8680;
/// The largest float (`FLT_MAX`).
const FLOAT_MAX: u32 = 0x0101_6970;
/// The distance (1200.0, a double) under which a conversation can become
/// the closest.
const CONVERSATION_DISTANCE: u32 = 0x0108_b468;

// Translated from 009640b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The object (`+8`) of the first entry of the player's action list
/// (`+0x60c`), 0 when the list is missing or empty.
pub fn fn_009640b0(e: &mut Engine, this: Ptr<PlayerCharacter>) -> u32 {
    let list = e.get(this, PlayerCharacter::pListofActions).addr();
    if list != 0 {
        let slot = e.call(LIST_NODE_SLOT, &args![list]).u32();
        if e.mem.u32(slot) != 0 {
            let slot = e.call(LIST_NODE_SLOT, &args![list]).u32();
            let action = e.mem.u32(slot);
            return e.mem.u32(action + 8);
        }
    }
    0
}

// Translated from 00964100 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PlayerCharacter::GetPlayerAction` (Xbox PDB): the first entry of the
/// action list (`+0x60c`) of type `kind` (its first word), or null.
pub fn player_character_get_player_action(
    e: &mut Engine,
    this: Ptr<PlayerCharacter>,
    kind: u32,
) -> Ptr {
    let list = e.get(this, PlayerCharacter::pListofActions).addr();
    if list == 0 {
        return Ptr::new(0);
    }
    let mut found = 0;
    let mut node = list;
    while node != 0 {
        let slot = e.call(LIST_NODE_SLOT, &args![node]).u32();
        if e.mem.u32(slot) == 0 || found != 0 {
            break;
        }
        let slot = e.call(LIST_NODE_SLOT, &args![node]).u32();
        let action = e.mem.u32(slot);
        if action != 0 && e.mem.u32(action) == kind {
            found = action;
        }
        node = e.call(LIST_NODE_NEXT, &args![node]).u32();
    }
    Ptr::new(found)
}

// Translated from 00964190 (decompiled, FalloutNV.exe 1.4.0.525)
/// Replaces the player's actor mover (`+0x190`): the old one gets its
/// deleting destructor (virtual at +0), a new 0xa0 byte `ActorMover` is built
/// by `009e9d00(this)`. The exception frame is not translated.
pub fn fn_00964190(e: &mut Engine, this: Ptr<PlayerCharacter>) {
    let old = e.get(this, PlayerCharacter::pActorMover).addr();
    if old != 0 {
        e.vcall(old, 0, &args![1u32]);
    }
    let block = e.call(OPERATOR_NEW, &args![0xa0u32]).u32();
    let mover = if block != 0 {
        e.call(ACTOR_MOVER_NEW, &args![block, this]).u32()
    } else {
        0
    };
    e.set(this, PlayerCharacter::pActorMover, Ptr::new(mover));
}

// Translated from 00965240 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the byte at `011f9423` (1 while the iron sights are in use in first
/// person, see [`player_character_update_image_space_ironsights`]).
pub fn fn_00965240(e: &mut Engine, value: u8) {
    e.set_global(IRON_SIGHTS_ACTIVE, value);
}

// Translated from 00965250 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores the float at `+0x28` of the image space modifier instance.
pub fn fn_00965250(e: &mut Engine, this: Ptr, value: f32) {
    e.mem.set_f32(this.addr() + 0x28, x87_quiet(value));
}

// Translated from 00965580 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores the float at `+0x38` of the image space modifier instance.
pub fn fn_00965580(e: &mut Engine, this: Ptr, value: f32) {
    e.mem.set_f32(this.addr() + 0x38, x87_quiet(value));
}

// Translated from 009655a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PlayerCharacter::StopIronSightsImageSpace` (Xbox PDB): stops the iron
/// sights depth of field instance (`+0x5ec`) and forgets it.
pub fn player_character_stop_iron_sights_image_space(e: &mut Engine, this: Ptr<PlayerCharacter>) {
    let instance = e.get(this, PlayerCharacter::pIronsightsDOFInstance);
    if !instance.is_null() {
        e.call(IMAGE_SPACE_INSTANCE_STOP, &args![instance]);
        e.set(this, PlayerCharacter::pIronsightsDOFInstance, Ptr::new(0));
    }
}

// Translated from 009655e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stops the VATS depth of field instance (`+0x5f0`) and forgets it.
pub fn fn_009655e0(e: &mut Engine, this: Ptr<PlayerCharacter>) {
    let instance = e.get(this, PlayerCharacter::pVatsDOFInstance);
    if !instance.is_null() {
        e.call(IMAGE_SPACE_INSTANCE_STOP, &args![instance]);
        e.set(this, PlayerCharacter::pVatsDOFInstance, Ptr::new(0));
    }
}

// Translated from 00966980 (decompiled, FalloutNV.exe 1.4.0.525)
/// The float at `+0x64`, or 1.0 when the flag word of the object has bit
/// 0x20000 (`004fd420`).
pub fn fn_00966980(e: &mut Engine, this: Ptr) -> f32 {
    if e.call(FLAGS_TEST, &args![this, 0x20000u32]).bool() {
        1.0
    } else {
        e.mem.f32(this.addr() + 0x64)
    }
}

// Translated from 009669c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Calls the blend function `009c3f50` with the float at `+0x68` of the
/// object's base form (`004181e0`, `0069ef80`) and the floats at `+0xcc`
/// and `+0xd0`, and answers its float result (the function leaves it in
/// `ST0`).
pub fn fn_009669c0(e: &mut Engine, this: Ptr) -> f32 {
    let first = e.mem.f32(this.addr() + 0xcc);
    let second = e.mem.f32(this.addr() + 0xd0);
    let form = e.call(GET_BASE_FORM, &args![this]).u32();
    let strength = e.call(READ_FIELD_68, &args![form]).f32();
    e.call(BLEND_FLOATS, &args![strength, first, second]).f32()
}

// Translated from 00966a00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores the float at `+0xd0`.
pub fn fn_00966a00(e: &mut Engine, this: Ptr, value: f32) {
    e.mem.set_f32(this.addr() + 0xd0, x87_quiet(value));
}

// Translated from 00966a20 (decompiled, FalloutNV.exe 1.4.0.525)
/// The float at `+0xec`.
pub fn fn_00966a20(e: &mut Engine, this: Ptr) -> f32 {
    e.mem.f32(this.addr() + 0xec)
}

// Translated from 00966a40 (decompiled, FalloutNV.exe 1.4.0.525)
/// The float global at `011f1c00`.
pub fn fn_00966a40(e: &mut Engine) -> f32 {
    e.global::<f32>(0x011f_1c00)
}

// Translated from 00966a50 (decompiled, FalloutNV.exe 1.4.0.525)
/// The number of entries of the player's note list (`+0x5e4`), as the list
/// counter `005ae380` answers.
pub fn fn_00966a50(e: &mut Engine, this: Ptr<PlayerCharacter>) -> u32 {
    e.call(LIST_COUNT, &args![this.addr() + NOTES_LIST_OFFSET])
        .u32()
}

// Translated from 00966a70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PlayerCharacter::AddNote` (Xbox PDB): adds `note` to the note list
/// (`+0x5e4`) unless it is already there; a null note only logs a warning.
/// A new note formats its location name with the format setting `011d3ca0`
/// into a local string and, when `show` is set, shows that text as a HUD
/// message. In every non-null case the sound `UIItemGenericUp` message is
/// shown by `007052f0` with the formatted text. The exception frame is not
/// translated.
pub fn player_character_add_note(e: &mut Engine, this: Ptr<PlayerCharacter>, note: u32, show: u8) {
    e.with_stack(8, |e, string| {
        e.call(STRING_NEW, &args![string]);
        if note == 0 {
            e.call(LOG_WARNING, &args![NULL_NOTE_WARNING]);
            e.call(STRING_DELETE, &args![string]);
            return;
        }
        let list = this.addr() + NOTES_LIST_OFFSET;
        e.with_stack(4, |e, cell| {
            e.mem.set_u32(cell.addr(), note);
            if !e.call(LIST_CONTAINS, &args![list, cell]).bool() {
                e.call(LIST_APPEND, &args![list, cell]);
                let name = e.call(GET_LOCATION_NAME, &args![note + 0x48]).u32();
                let format = e.call(SETTING_STRING, &args![NOTE_FORMAT_SETTING]).u32();
                e.call(STRING_FORMAT, &args![string, format, name]);
                if show != 0 {
                    let text = e.call(STRING_TEXT, &args![string]).u32();
                    e.call(SHOW_HUD_MESSAGE, &args![text]);
                }
            }
        });
        let text = e.call(STRING_TEXT, &args![string]).u32();
        let duration: f32 = e.global(MESSAGE_DURATION_010162C0);
        e.call(
            FUNCTION_007052F0,
            &args![text, 0u32, 0u32, NOTE_ADDED_SOUND, duration, 0u32],
        );
        e.call(STRING_DELETE, &args![string]);
    });
}

// Translated from 00966b80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Takes `note` out of the note display (`00704470`) and removes it from
/// the player's note list (`+0x5e4`).
pub fn fn_00966b80(e: &mut Engine, this: Ptr<PlayerCharacter>, note: u32) {
    e.call(NOTE_REMOVE_FROM_DISPLAY, &args![note]);
    list_remove_item(e, this.addr() + NOTES_LIST_OFFSET, note);
}

// Translated from 00966bb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The note at position `index` of the player's note list (`+0x5e4`), 0 when
/// the list is shorter.
pub fn fn_00966bb0(e: &mut Engine, this: Ptr<PlayerCharacter>, index: u32) -> u32 {
    let mut node = this.addr() + NOTES_LIST_OFFSET;
    let mut position = 0u32;
    while node != 0 {
        let slot = e.call(LIST_NODE_SLOT, &args![node]).u32();
        if e.mem.u32(slot) == 0 {
            break;
        }
        if position == index {
            let slot = e.call(LIST_NODE_SLOT, &args![node]).u32();
            return e.mem.u32(slot);
        }
        node = e.call(LIST_NODE_NEXT, &args![node]).u32();
        position = position.wrapping_add(1);
    }
    0
}

// Translated from 00966c10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether `note` (non-null) is in the player's note list (`+0x5e4`).
pub fn fn_00966c10(e: &mut Engine, this: Ptr<PlayerCharacter>, note: u32) -> bool {
    if note == 0 {
        return false;
    }
    let list = this.addr() + NOTES_LIST_OFFSET;
    e.with_stack(4, |e, cell| {
        e.mem.set_u32(cell.addr(), note);
        e.call(LIST_CONTAINS, &args![list, cell]).bool()
    })
}

// Translated from 00966c60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PlayerCharacter::GetTerminalAccess` (Xbox PDB): how the player can use
/// the terminal reference `terminal`: 4 when it has no terminal form, 0 when
/// the terminal's `+0xa0` word is a note the player has, 1 when it is hacked
/// or unlocked, 2 when it is locked out or of raw difficulty 5, 3 when the
/// player's actor value 0x28 allows hacking it, else 4.
pub fn player_character_get_terminal_access(
    e: &mut Engine,
    this: Ptr<PlayerCharacter>,
    terminal: u32,
) -> u32 {
    let mut form = 0;
    if terminal != 0 && e.call(READ_DWORD_AT_20, &args![terminal]).u32() != 0 {
        let base = e.call(READ_DWORD_AT_20, &args![terminal]).u32();
        form = dynamic_cast_form(e, base, TYPE_BGS_TERMINAL);
    }
    if form == 0 {
        return 4;
    }
    let note = e.call(READ_FIELD_A0, &args![form]).u32();
    if fn_00966c10(e, this, note) {
        return 0;
    }
    if e.call(TERMINAL_IS_HACKED, &args![terminal]).bool()
        || e.call(TERMINAL_IS_UNLOCKED, &args![form, terminal]).bool()
    {
        return 1;
    }
    if e.call(TERMINAL_IS_LOCKED_OUT, &args![terminal]).bool()
        || e.call(TERMINAL_GET_RAW_DIFFICULTY, &args![form, terminal])
            .u32()
            == 5
    {
        return 2;
    }
    let owner = this.addr() + ACTOR_VALUE_OWNER_OFFSET;
    let skill = e
        .call(ACTOR_VALUE_GET_CLAMPED, &args![owner, 0x28u32])
        .f32();
    if e.call(TERMINAL_IS_HACK_ELIGIBLE, &args![form, terminal, skill])
        .bool()
    {
        3
    } else {
        4
    }
}

// Translated from 00966d70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Walks the form map (`011c54c0`): for each entry whose value is a form of
/// type 0x3a (a reference) whose base form (`+0x20`) is of type 0x17, calls
/// `00501a10(reference)`. `ECX` is not read.
pub fn fn_00966d70(e: &mut Engine, _unused_this: Ptr) {
    let map = e.global::<u32>(FORM_MAP);
    let first = e.call(MAP_FIRST_ENTRY, &args![map]).u32();
    e.with_stack(12, |e, cells| {
        let entry = cells.addr();
        let key = entry + 4;
        let value = entry + 8;
        e.mem.set_u32(entry, first);
        while e.mem.u32(entry) != 0 {
            e.mem.set_u32(value, 0);
            e.mem.set_u32(key, 0);
            e.call(MAP_NEXT_ENTRY, &args![map, entry, key, value]);
            let form = e.mem.u32(value);
            if form != 0 && e.call(FORM_GET_TYPE, &args![form]).u32() == 0x3a {
                let base = e.call(READ_DWORD_AT_20, &args![form]).u32();
                if e.call(FORM_GET_TYPE, &args![base]).u32() == 0x17 {
                    e.call(REFERENCE_REMOVE_EXTRA_50, &args![form]);
                }
            }
        }
    });
}

// Translated from 00966e00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Counts one more steal warning (`+0x228`); when its timer (`+0x22c`) is
/// -1.0 it is set to the setting at `011cd5fc`.
pub fn fn_00966e00(e: &mut Engine, this: Ptr<PlayerCharacter>) {
    let count = e.get(this, PlayerCharacter::iNumberofStealWarnings);
    e.set(
        this,
        PlayerCharacter::iNumberofStealWarnings,
        count.wrapping_add(1),
    );
    let timer = e.get(this, PlayerCharacter::fStealWarningTimer);
    if f64::from(timer) == e.global::<f64>(DOUBLE_MINUS_ONE) {
        let value = setting_float(e, WARNING_TIMER_SETTING);
        e.set(this, PlayerCharacter::fStealWarningTimer, value);
    }
}

// Translated from 00966e50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Counts one more pickpocket warning (`+0x230`); when its timer (`+0x234`)
/// is -1.0 it is set to the setting at `011cd5fc`.
pub fn fn_00966e50(e: &mut Engine, this: Ptr<PlayerCharacter>) {
    let count = e.get(this, PlayerCharacter::iNumberofPickpocketWarnings);
    e.set(
        this,
        PlayerCharacter::iNumberofPickpocketWarnings,
        count.wrapping_add(1),
    );
    let timer = e.get(this, PlayerCharacter::fPickPocketWarningTimer);
    if f64::from(timer) == e.global::<f64>(DOUBLE_MINUS_ONE) {
        let value = setting_float(e, WARNING_TIMER_SETTING);
        e.set(this, PlayerCharacter::fPickPocketWarningTimer, value);
    }
}

// Translated from 00966ea0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets or clears (`set`) the bits `mask` in the flag word of the player's
/// actor mover (`009ea360`, the mover of the global player). When bit 0x400
/// is in `mask` and the player has a third person animation, the animation
/// group the actor keeps at index 4 (virtual `+0x1e4`, `004301b0`) is
/// blended out when its type is 0x11.
pub fn fn_00966ea0(e: &mut Engine, this: Ptr<PlayerCharacter>, mask: u32, set: u8) {
    let player = e.global::<u32>(PLAYER_CHARACTER);
    let mover = e.mem.u32(player + 0x190);
    e.call(ACTOR_MOVER_SET_FLAGS, &args![mover, mask, u32::from(set)]);
    if mask & 0x400 != 0 {
        let animation = e.call(PLAYER_GET_ANIMATION, &args![this, 0u32]).u32();
        if animation != 0 {
            let holder = e.vcall(this.addr(), 0x1e4, &args![]).u32();
            let group = e
                .call(ANIMATION_HOLDER_GET_GROUP, &args![holder, 4u32])
                .u16();
            if e.call(ANIM_GROUP_GET_TYPE, &args![u32::from(group)]).u32() == 0x11 {
                let animation = e.call(PLAYER_GET_ANIMATION, &args![this, 0u32]).u32();
                e.call(ANIMATION_BLEND_OUT, &args![animation, 4u32, 0u32]);
            }
        }
    }
}

// Translated from 00966f20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Notes that `actor` is near the player: nothing happens when the actor's
/// base form answers true to its virtual at `+0x20` (on the subobject at
/// `+0x30`) or the actor's virtual at `+0x22c` answers true. Otherwise an
/// existing `ActorsNearPlayerData` entry of the actor (`IsActorPerceived`)
/// gets `bHostile` (`+4`) and `bHasLineofSightNotDetection` (`+5`) set; a
/// missing entry is made (8 bytes, `004781b0`) and appended to the list
/// (`+0xd48`, itself created on first use). The exception frame is not
/// translated.
pub fn fn_00966f20(
    e: &mut Engine,
    this: Ptr<PlayerCharacter>,
    actor: u32,
    hostile: u8,
    line_of_sight_only: u8,
) {
    let base = e.call(GET_BASE_FORM, &args![actor]).u32();
    if e.vcall(base + 0x30, 0x20, &args![]).bool() {
        return;
    }
    if e.vcall(actor, 0x22c, &args![0u32]).bool() {
        return;
    }
    let entry = player_character_is_actor_perceived(e, this, actor).addr();
    if entry != 0 {
        e.mem.set_u8(entry + 5, line_of_sight_only);
        e.mem.set_u8(entry + 4, hostile);
        return;
    }
    if e.get(this, PlayerCharacter::pListofPercievedActors)
        .is_null()
    {
        let block = e.call(OPERATOR_NEW, &args![8u32]).u32();
        let list = if block != 0 {
            e.call(LIST_NEW, &args![block]).u32()
        } else {
            0
        };
        e.set(
            this,
            PlayerCharacter::pListofPercievedActors,
            Ptr::new(list),
        );
    }
    let block = e.call(OPERATOR_NEW, &args![8u32]).u32();
    let entry = if block != 0 {
        e.call(NEAR_PLAYER_DATA_NEW, &args![block]).u32()
    } else {
        0
    };
    e.mem.set_u32(entry, actor);
    e.mem.set_u8(entry + 4, hostile);
    e.mem.set_u8(entry + 5, line_of_sight_only);
    let list = e.get(this, PlayerCharacter::pListofPercievedActors).addr();
    e.with_stack(4, |e, cell| {
        e.mem.set_u32(cell.addr(), entry);
        e.call(LIST_APPEND, &args![list, cell]);
    });
}

// Translated from 00967090 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PlayerCharacter::IsActorPerceived` (Xbox PDB): the `ActorsNearPlayerData`
/// entry of `actor` in the list at `+0xd48`, or null.
pub fn player_character_is_actor_perceived(
    e: &mut Engine,
    this: Ptr<PlayerCharacter>,
    actor: u32,
) -> Ptr {
    let mut node = e.get(this, PlayerCharacter::pListofPercievedActors).addr();
    while node != 0 {
        let slot = e.call(LIST_NODE_SLOT, &args![node]).u32();
        if e.mem.u32(slot) == 0 {
            break;
        }
        let slot = e.call(LIST_NODE_SLOT, &args![node]).u32();
        let entry = e.mem.u32(slot);
        if e.mem.u32(entry) == actor {
            return Ptr::new(entry);
        }
        node = e.call(LIST_NODE_NEXT, &args![node]).u32();
    }
    Ptr::new(0)
}

// Translated from 009670f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether `actor` has an entry in the perceived list (`+0xd48`) whose
/// `bHasLineofSightNotDetection` (`+5`) is clear; false when the actor has
/// none.
pub fn fn_009670f0(e: &mut Engine, this: Ptr<PlayerCharacter>, actor: u32) -> bool {
    let mut node = e.get(this, PlayerCharacter::pListofPercievedActors).addr();
    while node != 0 {
        let slot = e.call(LIST_NODE_SLOT, &args![node]).u32();
        if e.mem.u32(slot) == 0 {
            break;
        }
        let slot = e.call(LIST_NODE_SLOT, &args![node]).u32();
        let entry = e.mem.u32(slot);
        if e.mem.u32(entry) == actor {
            return e.mem.u8(entry + 5) == 0;
        }
        node = e.call(LIST_NODE_NEXT, &args![node]).u32();
    }
    false
}

// Translated from 00967160 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PlayerCharacter::IsPlayerDetectedByNonTeammates` (Xbox PDB): whether
/// some actor of the perceived list (`+0xd48`) other than `excluded`, that
/// has an actor and no forced next update (`00566950`), has a line of sight
/// to the global player (`Actor::LineOfSight`) and a positive detection
/// level against it (`Actor::GetDetectionLevelAgainstActor`).
pub fn player_character_is_player_detected_by_non_teammates(
    e: &mut Engine,
    this: Ptr<PlayerCharacter>,
    excluded: u32,
) -> bool {
    let mut node = e.get(this, PlayerCharacter::pListofPercievedActors).addr();
    while node != 0 {
        let slot = e.call(LIST_NODE_SLOT, &args![node]).u32();
        if e.mem.u32(slot) == 0 {
            break;
        }
        let slot = e.call(LIST_NODE_SLOT, &args![node]).u32();
        let entry = e.mem.u32(slot);
        let actor = e.mem.u32(entry);
        if actor != 0
            && !e.call(ACTOR_GET_FORCE_NEXT_UPDATE, &args![actor]).bool()
            && actor != excluded
        {
            let player = e.global::<u32>(PLAYER_CHARACTER);
            let line_of_sight = e
                .call(
                    ACTOR_LINE_OF_SIGHT,
                    &args![actor, 0u32, player, 1u32, 0u32, 0u32],
                )
                .u8();
            let level = e.with_stack(4, |e, cell| {
                e.mem.set_u8(cell.addr(), line_of_sight);
                e.call(
                    ACTOR_GET_DETECTION_LEVEL,
                    &args![actor, 0u32, player, cell, 0u32, 0u32, 0u32, 0u32],
                )
                .i32()
            });
            if level > 0 {
                return true;
            }
        }
        node = e.call(LIST_NODE_NEXT, &args![node]).u32();
    }
    false
}

// Translated from 00967220 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PlayerCharacter::ChangePerceivedActorHostileStatus` (Xbox PDB): sets
/// `bHostile` (`+4`) of the perceived-list entry of `actor`.
pub fn player_character_change_perceived_actor_hostile_status(
    e: &mut Engine,
    this: Ptr<PlayerCharacter>,
    actor: u32,
    hostile: u8,
) {
    let mut node = e.get(this, PlayerCharacter::pListofPercievedActors).addr();
    while node != 0 {
        let slot = e.call(LIST_NODE_SLOT, &args![node]).u32();
        if e.mem.u32(slot) == 0 {
            break;
        }
        let slot = e.call(LIST_NODE_SLOT, &args![node]).u32();
        let entry = e.mem.u32(slot);
        if e.mem.u32(entry) == actor {
            e.mem.set_u8(entry + 4, hostile);
            return;
        }
        node = e.call(LIST_NODE_NEXT, &args![node]).u32();
    }
}

// Translated from 00967290 (decompiled, FalloutNV.exe 1.4.0.525)
/// Empties and deletes the perceived-actors list (`+0xd48`): every entry is
/// freed and its head node removed (`0063f7b0`), then the list object gets its
/// deleting destructor and the field is cleared.
pub fn fn_00967290(e: &mut Engine, this: Ptr<PlayerCharacter>) {
    if e.get(this, PlayerCharacter::pListofPercievedActors)
        .is_null()
    {
        return;
    }
    loop {
        let list = e.get(this, PlayerCharacter::pListofPercievedActors).addr();
        if list == 0 {
            break;
        }
        let slot = e.call(LIST_NODE_SLOT, &args![list]).u32();
        if e.mem.u32(slot) == 0 {
            break;
        }
        let slot = e.call(LIST_NODE_SLOT, &args![list]).u32();
        let entry = e.mem.u32(slot);
        e.call(OPERATOR_DELETE, &args![entry]);
        let list = e.get(this, PlayerCharacter::pListofPercievedActors).addr();
        e.call(LIST_REMOVE_NODE, &args![list]);
    }
    let list = e.get(this, PlayerCharacter::pListofPercievedActors).addr();
    if list != 0 {
        e.call(LIST_DELETE, &args![list, 1u32]);
    }
    e.set(this, PlayerCharacter::pListofPercievedActors, Ptr::new(0));
}

// Translated from 00967350 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PlayerCharacter::RemovePerceivedActor` (Xbox PDB): frees the entry of
/// `actor` in the perceived list (`+0xd48`) and removes its node
/// (`0063f7b0`).
pub fn player_character_remove_perceived_actor(
    e: &mut Engine,
    this: Ptr<PlayerCharacter>,
    actor: u32,
) {
    let mut node = e.get(this, PlayerCharacter::pListofPercievedActors).addr();
    while node != 0 {
        let slot = e.call(LIST_NODE_SLOT, &args![node]).u32();
        if e.mem.u32(slot) == 0 {
            break;
        }
        let slot = e.call(LIST_NODE_SLOT, &args![node]).u32();
        let entry = e.mem.u32(slot);
        if e.mem.u32(entry) == actor {
            e.call(OPERATOR_DELETE, &args![entry]);
            e.call(LIST_REMOVE_NODE, &args![node]);
            return;
        }
        node = e.call(LIST_NODE_NEXT, &args![node]).u32();
    }
}

// Translated from 00967700 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether either of the bytes at `+0x160` and `+0x161` is set (the Pipboy
/// manager's two "open" flags).
pub fn fn_00967700(e: &mut Engine, this: Ptr) -> bool {
    e.mem.u8(this.addr() + 0x160) != 0 || e.mem.u8(this.addr() + 0x161) != 0
}

// Translated from 00967730 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PlayerCharacter::CheckForClosesConversation` (Xbox PDB): decides the
/// global player's `pClosestConversation` (`+0x208`). `first` and `second`
/// are the two participants of the conversation `conversation`; the
/// smaller of their distances to the player is compared with the distance
/// of the current conversation's closest participant (infinite when there
/// is none): when either participant is the player the field is cleared;
/// otherwise `conversation` replaces it when there is no current one and the
/// distance is at most 1200.0, or when it is no farther than the current
/// one.
///
/// thiscall, `RET 0xc` (`00967894`): `this` (ECX, stored at `00967736` and
/// never read again; the body reads the global player) and three words.
pub fn player_character_check_for_closes_conversation(
    e: &mut Engine,
    _this: Ptr<PlayerCharacter>,
    first: u32,
    second: u32,
    conversation: u32,
) {
    let player = e.global::<u32>(PLAYER_CHARACTER);
    let mut distance = e
        .call(REFERENCE_GET_DISTANCE, &args![first, player, 0u32, 0u32])
        .f32();
    let second_distance = e
        .call(REFERENCE_GET_DISTANCE, &args![second, player, 0u32, 0u32])
        .f32();
    if distance > second_distance {
        distance = second_distance;
    }
    let mut closest: f32 = e.global(FLOAT_MAX);
    let current = e.mem.u32(player + 0x208);
    if current != 0 {
        if e.call(CONVERSATION_GET_STARTER, &args![current]).u32() != 0 {
            let starter = e.call(CONVERSATION_GET_STARTER, &args![current]).u32();
            closest = e
                .call(REFERENCE_GET_DISTANCE, &args![starter, player, 0u32, 0u32])
                .f32();
        }
        if e.call(CONVERSATION_GET_TARGET, &args![current]).u32() != 0 {
            let target = e.call(CONVERSATION_GET_TARGET, &args![current]).u32();
            let target_distance = e
                .call(REFERENCE_GET_DISTANCE, &args![target, player, 0u32, 0u32])
                .f32();
            if closest > target_distance {
                closest = target_distance;
            }
        }
    }
    if second == player || first == player {
        e.mem.set_u32(player + 0x208, 0);
    } else {
        let current = e.mem.u32(player + 0x208);
        let limit: f64 = e.global(CONVERSATION_DISTANCE);
        if (current == 0 && f64::from(distance) <= limit) || (current != 0 && distance <= closest) {
            e.mem.set_u32(player + 0x208, conversation);
        }
    }
}

/// The reference the Havok body `body` stands for: `0044ddc0` of the node
/// owner `004b5a20(004ae750(body))`, 0 when any step is missing.
fn body_reference_word(e: &mut Engine, body: u32) -> u32 {
    let owner = if body != 0 {
        let proxy = e.call(BODY_LOOKUP_004AE750, &args![body]).u32();
        e.call(FUNCTION_004B5A20, &args![proxy]).u32()
    } else {
        0
    };
    if owner != 0 {
        e.call(READ_FIELD_8, &args![owner]).u32()
    } else {
        0
    }
}

// Translated from 009678a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PlayerCharacter::IsStandingOnFurniture` (Xbox PDB): whether the
/// reference of the body under the player's character controller
/// (`00c6fd20`) is furniture (`TESObjectREFR::IsFurniture`).
pub fn player_character_is_standing_on_furniture(
    e: &mut Engine,
    this: Ptr<PlayerCharacter>,
) -> bool {
    let controller = e.call(GET_CHAR_CONTROLLER, &args![this]).u32();
    if controller == 0 {
        return false;
    }
    let body = e.call(FUNCTION_00C6FD20, &args![controller]).u32();
    let word = body_reference_word(e, body);
    let reference = e.call(FIND_REFERENCE_FOR_3D, &args![word]).u32();
    reference != 0 && e.call(REFERENCE_IS_FURNITURE, &args![reference]).bool()
}

// Translated from 00967950 (decompiled, FalloutNV.exe 1.4.0.525)
/// The reference of the body `00c6fd40` of the player's character
/// controller, when that differs from the body `00c6fd20` returns and has a
/// reference word; null otherwise.
pub fn fn_00967950(e: &mut Engine, this: Ptr<PlayerCharacter>) -> Ptr {
    let controller = e.call(GET_CHAR_CONTROLLER, &args![this]).u32();
    if controller == 0 {
        return Ptr::new(0);
    }
    let body = e.call(FUNCTION_00C6FD20, &args![controller]).u32();
    let standing = e.call(CONTROLLER_BODY_AT_630, &args![controller]).u32();
    if body == standing {
        return Ptr::new(0);
    }
    let standing = e.call(CONTROLLER_BODY_AT_630, &args![controller]).u32();
    let word = body_reference_word(e, standing);
    if word == 0 {
        return Ptr::new(0);
    }
    Ptr::new(e.call(FIND_REFERENCE_FOR_3D, &args![word]).u32())
}

// Translated from 00967a00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PlayerCharacter::GetBumpedRef` (Xbox PDB): the reference of the body
/// the player's character controller bumped (`00c6fd60`, then its scene
/// object `00810660`), when its virtual at `+0x100` answers true; else null.
pub fn player_character_get_bumped_ref(e: &mut Engine, this: Ptr<PlayerCharacter>) -> Ptr {
    let controller = e.call(GET_CHAR_CONTROLLER, &args![this]).u32();
    if controller == 0 {
        return Ptr::new(0);
    }
    let body = e.call(CONTROLLER_BODY_AT_638, &args![controller]).u32();
    let node = if body != 0 {
        e.call(BODY_GET_AV_OBJECT, &args![body]).u32()
    } else {
        0
    };
    let reference = e.call(FIND_REFERENCE_FOR_3D, &args![node]).u32();
    if reference != 0 && e.vcall(reference, 0x100, &args![]).bool() {
        Ptr::new(reference)
    } else {
        Ptr::new(0)
    }
}

/// `BSAudio::QInstance` (Xbox PDB), no arguments: the audio manager.
const AUDIO_INSTANCE: u32 = 0x0045_3a70;
/// `BSAudio::GetSoundHandleByName` (Xbox PDB) `(audio, handle out, name, flags)`.
const AUDIO_GET_SOUND_HANDLE_BY_NAME: u32 = 0x00ad_7550;
/// The setting object of the iron sights' depth of field start.
const SETTING_011CFD70: u32 = 0x011c_fd70;
const SETTING_011CF8C8: u32 = 0x011c_f8c8;
/// The setting read as the iron sights' fade limit and target.
const SETTING_011CE6B4: u32 = 0x011c_e6b4;
const SETTING_011CE4D4: u32 = 0x011c_e4d4;
/// The setting object of the iron sights' blend rate.
const SETTING_011CEDF4: u32 = 0x011c_edf4;
/// The settings of the VATS depth of field and DRB image space modifiers.
const SETTING_011CFE20: u32 = 0x011c_fe20;
const SETTING_011CF38C: u32 = 0x011c_f38c;
const SETTING_011CEC04: u32 = 0x011c_ec04;
const SETTING_011CFCC8: u32 = 0x011c_fcc8;
const SETTING_011CE924: u32 = 0x011c_e924;
const SETTING_011CF4F8: u32 = 0x011c_f4f8;
/// The float (a small scale factor) the VATS DRB point is built from.
const POINT_SCALE_01016248: u32 = 0x0101_6248;
/// The held-menu-button time setting.
const SETTING_011CD098: u32 = 0x011c_d098;
/// The byte that says the Pipboy light toggle already fired for the
/// current press of the menu mode button.
const MENU_MODE_BUTTON_FIRED: u32 = 0x011e_0d70;
/// `(pipboy manager)`: true while the Pipboy is in use (`00507d80`).
const PIPBOY_IS_ACTIVE: u32 = 0x0050_7d80;
/// `FOPipboyManager::ShowPipboyLightEffect` (Xbox PDB) `(manager, a, b, c)`.
const PIPBOY_SHOW_LIGHT_EFFECT: u32 = 0x007f_a310;
/// `MagicTarget::Dispel` (Xbox PDB) `(target, item, flag)`.
const MAGIC_TARGET_DISPEL: u32 = 0x0082_4400;
/// The caster object whose `MagicTarget` base the Pipboy light effects
/// affect (`0093ccd0`, no arguments).
const GET_LIGHT_TARGET_OWNER: u32 = 0x0093_ccd0;
/// `PlayerCharacter::IsInState`-like `(player, mask)`: whether the byte at
/// `+0x680` has any bit of `mask`.
const PLAYER_HAS_CONTROL_BITS: u32 = 0x005a_03f0;
/// The sounds of the Pipboy light switching on and off.
const PIPBOY_LIGHT_ON_SOUND: u32 = 0x0108_b444;
const PIPBOY_LIGHT_OFF_SOUND: u32 = 0x0108_b454;

/// One step of the image space blend: moves `current` towards `target` by
/// `frame time / rate` (the float setting `rate_setting` is the time to
/// cross one unit) and clamps it into `0..=limit`, as the x87 code does
/// (`005330e0` on a stack float).
fn approach_image_space(
    e: &mut Engine,
    current: f32,
    target: f32,
    rate_setting: u32,
    limit: f32,
) -> f32 {
    let rate = setting_float(e, rate_setting);
    let per_second = 1.0 / f64::from(rate);
    let frame = e.call(READ_FLOAT_AT_C, &args![FRAME_TIME_OBJECT]).f32();
    let step = (f64::from(frame) * per_second) as f32;
    let moved = if current < target {
        (f64::from(current) + f64::from(step)) as f32
    } else {
        (f64::from(current) - f64::from(step)) as f32
    };
    e.with_stack(4, |e, cell| {
        e.mem.set_f32(cell.addr(), moved);
        e.call(CLAMP_FLOAT_IN_PLACE, &args![cell, 0.0f32, limit]);
        e.mem.f32(cell.addr())
    })
}

// Translated from 009650a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PlayerCharacter::UpdateImageSpaceIronsights` (Xbox PDB): keeps the iron
/// sights depth of field instance (`+0x5ec`, triggered on first use and
/// flagged active) moving to its target. The target is the setting
/// `011ce6b4` while the player aims through the iron sights outside third
/// person (also setting the iron sights byte `011f9423`), else 0. The
/// strength steps towards it by the frame time over the setting
/// `011cedf4`, clamped to `0..=setting 011ce6b4`, and is stored in the
/// instance (`0063f790`) together with the setting `011ce4d4`
/// (`00965250`). The read of the setting `011ce4d4` before the target is
/// kept; its value is not used.
pub fn player_character_update_image_space_ironsights(e: &mut Engine, this: Ptr<PlayerCharacter>) {
    if e.get(this, PlayerCharacter::pIronsightsDOFInstance)
        .is_null()
    {
        let first = setting_float(e, SETTING_011CFD70);
        let second = setting_float(e, SETTING_011CF8C8);
        let instance = e
            .call(
                IMAGE_SPACE_DOF_TRIGGER,
                &args![second, first, 4u32, 0.0f32, 0.0f32, 0.0f32],
            )
            .u32();
        e.set(
            this,
            PlayerCharacter::pIronsightsDOFInstance,
            Ptr::new(instance),
        );
        if instance != 0 {
            e.call(IMAGE_SPACE_INSTANCE_SET_ACTIVE, &args![instance, 1u32]);
        }
    }
    let limit = setting_float(e, SETTING_011CE6B4);
    let mut target = 0.0f32;
    let instance = e.get(this, PlayerCharacter::pIronsightsDOFInstance);
    let mut current = e.call(READ_FLOAT_AT_C, &args![instance]).f32();
    if e.call(ACTOR_GET_IRON_SIGHTS, &args![this]).bool()
        && e.get(this, PlayerCharacter::b3rdPerson) == 0
    {
        setting_float(e, SETTING_011CE4D4);
        target = setting_float(e, SETTING_011CE6B4);
        fn_00965240(e, 1);
    } else {
        fn_00965240(e, 0);
    }
    if current != target {
        current = approach_image_space(e, current, target, SETTING_011CEDF4, limit);
    }
    let instance = e.get(this, PlayerCharacter::pIronsightsDOFInstance);
    e.call(SET_FLOAT_AT_C, &args![instance, current]);
    let value = e.call(SETTING_FLOAT_VALUE, &args![SETTING_011CE4D4]).f32();
    let instance = e.get(this, PlayerCharacter::pIronsightsDOFInstance);
    fn_00965250(e, instance, value);
}

// Translated from 00965270 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PlayerCharacter::UpdateImageSpaceVATS` (Xbox PDB): keeps the VATS depth
/// of field (`+0x5f0`) and DRB (`+0x5f4`) image space instances, made on
/// first use and flagged active, moving to their targets. While the object
/// at `011f2250` reports mode 2 or 3 and the target object `011f21cc`
/// exists, the targets are the settings `011cfcc8` (DRB) and `011ce924`
/// (depth of field) and the fade is the distance (virtual `+0x1f4`
/// positions) between that object and the player; otherwise the targets are
/// 0. Each strength steps by the frame time over the setting `011cf4f8`
/// and is clamped to `0..=` its setting; both instances get their strength
/// and the fade (`0063f790`, `00693d50`, `00965580`).
pub fn player_character_update_image_space_vats(e: &mut Engine, this: Ptr<PlayerCharacter>) {
    let mut fade = 0.0f32;
    if e.get(this, PlayerCharacter::pVatsDOFInstance).is_null() {
        let strength = setting_float(e, SETTING_011CFE20);
        let instance = e
            .call(
                IMAGE_SPACE_DOF_TRIGGER,
                &args![fade, strength, 0u32, 0.0f32, 0.0f32, 0.0f32],
            )
            .u32();
        e.set(this, PlayerCharacter::pVatsDOFInstance, Ptr::new(instance));
        if instance != 0 {
            e.call(IMAGE_SPACE_INSTANCE_SET_ACTIVE, &args![instance, 1u32]);
        }
    }
    if e.get(this, PlayerCharacter::pVatsDRBInstance).is_null() {
        let scale: f32 = e.global(POINT_SCALE_01016248);

        let instance = e.with_stack(8, |e, point| {
            e.call(NI_POINT2_NEW, &args![point, scale, scale]);
            let at_cf38c = setting_float(e, SETTING_011CF38C);
            let at_cec04 = setting_float(e, SETTING_011CEC04);
            let strength = setting_float(e, SETTING_011CFE20);
            e.call(
                IMAGE_SPACE_DRB_TRIGGER,
                &args![
                    fade, strength, 0u32, 0.0f32, at_cec04, at_cf38c, 0.0f32, 0.0f32, point, 0.0f32
                ],
            )
            .u32()
        });
        e.set(this, PlayerCharacter::pVatsDRBInstance, Ptr::new(instance));
        if instance != 0 {
            e.call(IMAGE_SPACE_INSTANCE_SET_ACTIVE, &args![instance, 1u32]);
        }
    }
    let drb_limit = setting_float(e, SETTING_011CFCC8);
    let dof_limit = setting_float(e, SETTING_011CE924);
    let mut dof_target = 0.0f32;
    let mut drb_target = 0.0f32;
    let dof_instance = e.get(this, PlayerCharacter::pVatsDOFInstance);
    let mut dof_current = e.call(READ_FLOAT_AT_C, &args![dof_instance]).f32();
    let drb_instance = e.get(this, PlayerCharacter::pVatsDRBInstance);
    let mut drb_current = e.call(READ_FLOAT_AT_C, &args![drb_instance]).f32();
    let mode = e.call(READ_FIELD_8, &args![OBJECT_011F2250]).u32();
    let in_mode = mode == 2 || e.call(READ_FIELD_8, &args![OBJECT_011F2250]).u32() == 3;
    let target_object = e.global::<u32>(POINTER_011F21CC);
    if in_mode && target_object != 0 {
        drb_target = drb_limit;
        dof_target = dof_limit;
        let own = e.vcall(this.addr(), 0x1f4, &args![]).u32();
        let other = e.with_stack(12, |e, difference| {
            let position = e.vcall(target_object, 0x1f4, &args![]).u32();
            let result = e
                .call(VECTOR_SUBTRACT, &args![position, difference, own])
                .u32();
            e.call(VECTOR_LENGTH, &args![result]).f32()
        });
        fade = other;
    }
    if dof_current != dof_target {
        dof_current = approach_image_space(e, dof_current, dof_target, SETTING_011CF4F8, dof_limit);
    }
    if drb_current != drb_target {
        drb_current = approach_image_space(e, drb_current, drb_target, SETTING_011CF4F8, drb_limit);
    }
    let dof_instance = e.get(this, PlayerCharacter::pVatsDOFInstance);
    e.call(SET_FLOAT_AT_C, &args![dof_instance, dof_current]);
    let dof_instance = e.get(this, PlayerCharacter::pVatsDOFInstance);
    e.call(SET_FLOAT_AT_20, &args![dof_instance, fade]);
    let drb_instance = e.get(this, PlayerCharacter::pVatsDRBInstance);
    e.call(SET_FLOAT_AT_C, &args![drb_instance, drb_current]);
    let drb_instance = e.get(this, PlayerCharacter::pVatsDRBInstance);
    fn_00965580(e, drb_instance, fade);
}

/// Plays the sound `name` (`GetSoundHandleByName` flags 0x121) on a local
/// sound handle, then destroys the handle.
fn play_ui_sound(e: &mut Engine, name: u32) {
    e.with_stack(SOUND_HANDLE_SIZE, |e, handle| {
        let audio = e.call(AUDIO_INSTANCE, &args![]).u32();
        let found = e
            .call(
                AUDIO_GET_SOUND_HANDLE_BY_NAME,
                &args![audio, handle, name, 0x121u32],
            )
            .u32();
        e.call(SOUND_HANDLE_PLAY, &args![found, 0u32]);
        e.call(EMPTY_DESTRUCTOR, &args![handle]);
    });
}

/// `ShowPipboyLightEffect(a, b, c)` on the current Pipboy manager.
fn show_pipboy_light_effect(e: &mut Engine, a: u32, b: u32, c: u32) {
    let pipboy = e.call(INTERFACE_GET_PIPBOY, &args![]).u32();
    e.call(PIPBOY_SHOW_LIGHT_EFFECT, &args![pipboy, a, b, c]);
}

/// The `MagicTarget` of the object `0093ccd0` answers (`+0x18`), 0 when
/// there is none.
fn light_target(e: &mut Engine) -> u32 {
    let owner = e.call(GET_LIGHT_TARGET_OWNER, &args![]).u32();
    if owner != 0 {
        owner + 0x18
    } else {
        0
    }
}

// Translated from 009673d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PlayerCharacter::UpdateMenuModeButton` (Xbox PDB): the Pipboy light
/// toggle on a held menu button. While control 0xe is down in state 2 (and
/// the toggle has not fired) for longer than the setting `011cd098`, the
/// button counts as clicked (`+0xd54`). Otherwise, while control 0xe is
/// down in state 1 or 0 the timer (`+0xd4c`) grows by `delta`; the first
/// time it passes the setting with the Pipboy in use, the Pipboy light is
/// toggled (effects, sounds `UIPipBoyLightOn`/`Off`, a dispel or a cast
/// through the `MagicCaster` base at `+0x88`) and the fired byte
/// (`011e0d70`) is set; when the button is released the click, the timer
/// and the byte are cleared. Last, while the Pipboy manager reports one of
/// its flags (`00967700`) but is not in use, the light effects are cleared
/// and the target dispelled. The exception frame is not translated.
pub fn player_character_update_menu_mode_button(
    e: &mut Engine,
    this: Ptr<PlayerCharacter>,
    delta: f32,
) {
    let owner = e.global::<u32>(POINTER_011DEA0C);
    let controls = e.call(FUNCTION_00877720, &args![owner]).u32();
    let magic_target = this.addr() + MAGIC_TARGET_OFFSET;
    let mut clicked = false;
    if e.global::<u8>(MENU_MODE_BUTTON_FIRED) == 0
        && e.call(CONTROL_QUERY, &args![controls, 0xeu32, 2u32]).u32() != 0
    {
        let threshold = setting_float(e, SETTING_011CD098);
        let timer = e.get(this, PlayerCharacter::fMenuModeButtonTimer);
        if threshold > timer {
            e.set(this, PlayerCharacter::bMenuModeButtonClicked, 1);
            clicked = true;
        }
    }
    if !clicked {
        let down = e.call(CONTROL_QUERY, &args![controls, 0xeu32, 1u32]).u32() != 0
            || e.call(CONTROL_QUERY, &args![controls, 0xeu32, 0u32]).u32() != 0;
        if !down {
            e.set(this, PlayerCharacter::bMenuModeButtonClicked, 0);
            e.set(this, PlayerCharacter::fMenuModeButtonTimer, 0.0);
            e.set_global(MENU_MODE_BUTTON_FIRED, 0u8);
        } else {
            let timer = e.get(this, PlayerCharacter::fMenuModeButtonTimer);
            let timer = (f64::from(timer) + f64::from(delta)) as f32;
            e.set(this, PlayerCharacter::fMenuModeButtonTimer, timer);
            if e.global::<u8>(MENU_MODE_BUTTON_FIRED) == 0 {
                let pipboy = e.call(INTERFACE_GET_PIPBOY, &args![]).u32();
                if e.call(PIPBOY_IS_ACTIVE, &args![pipboy]).u32() != 0 {
                    let threshold = setting_float(e, SETTING_011CD098);
                    let timer = e.get(this, PlayerCharacter::fMenuModeButtonTimer);
                    if threshold < timer {
                        let target = light_target(e);
                        if e.call(
                            MAGIC_TARGET_IS_SPELL_TARGET,
                            &args![magic_target, target, 1u32],
                        )
                        .bool()
                        {
                            show_pipboy_light_effect(e, 1, 0, 1);
                            show_pipboy_light_effect(e, 0, 0, 1);
                            play_ui_sound(e, PIPBOY_LIGHT_OFF_SOUND);
                            let target = light_target(e);
                            e.call(
                                MAGIC_TARGET_DISPEL,
                                &args![magic_target, target, 0u32, 0u32],
                            );
                        } else if !e
                            .call(
                                PLAYER_HAS_CONTROL_BITS,
                                &args![e.global::<u32>(PLAYER_CHARACTER), 4u32],
                            )
                            .bool()
                        {
                            show_pipboy_light_effect(e, 1, 1, 1);
                            show_pipboy_light_effect(e, 0, 1, 1);
                            play_ui_sound(e, PIPBOY_LIGHT_ON_SOUND);
                            let caster = e.call(GET_LIGHT_TARGET_OWNER, &args![]).u32();
                            e.vcall(this.addr() + MAGIC_CASTER_OFFSET, 0, &args![caster, 0u32]);
                        }
                        e.set_global(MENU_MODE_BUTTON_FIRED, 1u8);
                    }
                }
            }
        }
    }
    let pipboy = e.call(INTERFACE_GET_PIPBOY, &args![]).u32();
    if fn_00967700(e, Ptr::new(pipboy)) {
        let pipboy = e.call(INTERFACE_GET_PIPBOY, &args![]).u32();
        if e.call(PIPBOY_IS_ACTIVE, &args![pipboy]).u32() == 0 {
            show_pipboy_light_effect(e, 1, 0, 1);
            show_pipboy_light_effect(e, 0, 0, 1);
            let target = light_target(e);
            e.call(
                MAGIC_TARGET_DISPEL,
                &args![magic_target, target, 0u32, 0u32],
            );
        }
    }
}

/// `(array, 0)`: the number of entries of the process list `ProcessLists +
/// 4`.
const FUNCTION_005BE5C0: u32 = 0x005b_e5c0;
/// `MobileObject::GetCurrentProcessType` (Xbox PDB).
const GET_CURRENT_PROCESS_TYPE: u32 = 0x0093_1850;
/// `NiAVObject::GetWorldBound` (Xbox PDB), `ECX` = the node: a pointer to the
/// 16 byte bound (centre and radius).
const NODE_GET_WORLD_BOUND: u32 = 0x0043_d450;
/// `NiCamera::ScreenSpaceBoundBound` (Xbox PDB) `(camera, bound, out, out, epsilon)`.
const SCREEN_SPACE_BOUND_BOUND: u32 = 0x00a7_05f0;
/// `(object, value)`: stores a float at `+0x0c`.
const SET_FLOAT_AT_C: u32 = 0x0063_f790;
/// `(object, value)`: stores a float at `+0x20`.
const SET_FLOAT_AT_20: u32 = 0x0069_3d50;
/// `_ftol2_sse` (`00ec62c0`) truncating the value held in `ST0` (an `f64`).
const FTOL2: u32 = 0x00ec_62c0;
/// `(camera, a, b, origin out, direction out, 0)`: the camera's ray for the
/// window point `(a, b)`.
const WINDOW_POINT_TO_RAY: u32 = 0x00a7_1080;
/// The getter of the global object at `011d8a80` (it ignores its arguments).
const GLOBAL_OBJECT_GETTER: u32 = 0x004b_7210;
/// The float at `+0x40` / `+0x38` of the object.
const READ_FLOAT_AT_40: u32 = 0x0079_2760;
const READ_FLOAT_AT_38: u32 = 0x009a_9350;
/// The holder of the player's camera (`ECX` of [`READ_DWORD_AT_AC`]).
const GET_CAMERA_HOLDER: u32 = 0x0045_c670;
/// The function at `008cfbc0`, cdecl, nine words `(node, origin xyz, end xyz,
/// &distance, 0)`: casts the ray and returns what it hits.
const FUNCTION_008CFBC0: u32 = 0x008c_fbc0;
/// The function at `0062a190` `(object, 0x25, form)`.
const FUNCTION_0062A190: u32 = 0x0062_a190;
/// The function at `0059ceb0` `(object, other)`.
const FUNCTION_0059CEB0: u32 = 0x0059_ceb0;
/// The function at `0059cee0` `(object)`: destroys the `0062a190` object.
const FUNCTION_0059CEE0: u32 = 0x0059_cee0;
/// The function at `005df560` `(matrix, index, &a, &b, &c)`.
const FUNCTION_005DF560: u32 = 0x005d_f560;
/// The function at `005df510` `(matrix, index, &a)`.
const FUNCTION_005DF510: u32 = 0x005d_f510;
/// The function at `005df5b0` `(object)`: a pointer to three floats.
const FUNCTION_005DF5B0: u32 = 0x005d_f5b0;
/// `NiMatrix3::MakeXRotation` (Xbox PDB) `(matrix, angle)`.
const MATRIX_MAKE_X_ROTATION: u32 = 0x0052_4ac0;
/// `MobileObject::GetLooking` (Xbox PDB).
const MOBILE_OBJECT_GET_LOOKING: u32 = 0x0093_1d70;
/// `ECX` = the player; adds a float to the position (`0070c3b0`).
const FUNCTION_0070C3B0: u32 = 0x0070_c3b0;
/// The function at `00888960`: reads the pick count override.
const FUNCTION_00888960: u32 = 0x0088_8960;
/// `bhkWorld::SetPickCountOverride` (Xbox PDB), cdecl `(count)`.
const WORLD_SET_PICK_COUNT_OVERRIDE: u32 = 0x00c6_6230;
/// `TES::Pick` (Xbox PDB), `ECX` = the `TES` global, `(pick data)`.
const TES_PICK: u32 = 0x0045_8420;
/// The global `TES` singleton pointer.
const TES_SINGLETON: u32 = 0x011d_ea10;
/// The function at `004b71d0`, cdecl, no arguments.
const FUNCTION_004B71D0: u32 = 0x004b_71d0;
/// The function at `008b3ef0` `(reference, index)`.
const FUNCTION_008B3EF0: u32 = 0x008b_3ef0;
/// The function at `007f3b40`, cdecl `(index)`.
const FUNCTION_007F3B40: u32 = 0x007f_3b40;
/// `BGSBodyPartData::GetBodyPart` (Xbox PDB) `(data, index)`.
const BODY_PART_DATA_GET_BODY_PART: u32 = 0x005e_50f0;
/// The function at `005e5190`: the byte of a body part.
const FUNCTION_005E5190: u32 = 0x005e_5190;
/// The setting for the auto aim range.
const SETTING_011CED64: u32 = 0x011c_ed64;
/// The setting for the half size of the auto aim screen box.
const SETTING_011CEFA4: u32 = 0x011c_efa4;
/// The double the auto aim box setting is multiplied with.
const AIM_BOX_SCALE_01023B88: u32 = 0x0102_3b88;
/// The float epsilon `ScreenSpaceBoundBound` gets.
const SCREEN_EPSILON_010718C0: u32 = 0x0107_18c0;
/// The double added to the distance to the targeted object.
const AIM_DISTANCE_PAD_0102E430: u32 = 0x0102_e430;
/// The global written with the body part byte of the aimed actor.
const AIMED_BODY_PART: u32 = 0x011a_59f0;

/// Size of the emulated frame of [`player_character_update_auto_aim_actor`]
/// (its locals reach down to `[EBP - 0x3cc]`), and where `EBP` is in it.
const AUTO_AIM_FRAME: u32 = 0x440;
const AUTO_AIM_EBP: u32 = 0x400;

/// Copies `words` words from `source` to `destination`.
fn copy_words(e: &mut Engine, source: u32, destination: u32, words: u32) {
    for word in 0..words {
        let value = e.mem.u32(source + word * 4);
        e.mem.set_u32(destination + word * 4, value);
    }
}

/// Truncates a float like the game's `_ftol2` helper.
fn float_to_int(e: &mut Engine, value: f32) -> u32 {
    e.call(FTOL2, &args![f64::from(value)]).u32()
}

/// The camera ray of the window: `origin` and `direction` are filled by
/// `00a71080` from the camera and the screen object's `+0x38` / `+0x40`
/// floats, truncated to integers.
fn camera_window_ray(e: &mut Engine, origin: u32, direction: u32) {
    let screen = e
        .call(GLOBAL_OBJECT_GETTER, &args![origin, direction, 0u32])
        .u32();
    let second = e.call(READ_FLOAT_AT_40, &args![screen]).f32();
    let second = float_to_int(e, second);
    let screen = e.call(GLOBAL_OBJECT_GETTER, &args![second]).u32();
    let first = e.call(READ_FLOAT_AT_38, &args![screen]).f32();
    let first = float_to_int(e, first);
    let holder = e.call(GET_CAMERA_HOLDER, &args![]).u32();
    let camera = e.call(READ_DWORD_AT_AC, &args![holder]).u32();
    e.call(
        WINDOW_POINT_TO_RAY,
        &args![camera, first, second, origin, direction, 0u32],
    );
}

/// `8cfbc0(node, origin (3 words), end (3 words), &distance, 0)`.
fn cast_aim_ray(e: &mut Engine, node: u32, origin: u32, end: u32, distance: u32) -> u32 {
    let origin_words = [
        e.mem.u32(origin),
        e.mem.u32(origin + 4),
        e.mem.u32(origin + 8),
    ];
    let end_words = [e.mem.u32(end), e.mem.u32(end + 4), e.mem.u32(end + 8)];
    e.call(
        FUNCTION_008CFBC0,
        &args![
            node,
            origin_words[0],
            origin_words[1],
            origin_words[2],
            end_words[0],
            end_words[1],
            end_words[2],
            distance,
            0u32
        ],
    )
    .u32()
}

/// One actor of the auto aim scan: if it is a candidate (an actor whose
/// virtual `+0x100` answers true, not `+0x22c`, in process type 0, with a 3D
/// node, and perceived by the player), is closer than `range`, and its
/// screen bounds overlap the auto aim box and it is in the player's line
/// of sight, it becomes the player's target actor (`+0xd44`) and, when
/// it is a hostile entry of the perceived list, the best actor.
fn auto_aim_candidate(
    e: &mut Engine,
    f: Frame,
    this: Ptr<PlayerCharacter>,
    camera: u32,
    actor: u32,
    best: &mut u32,
    range: &mut f32,
) {
    let player = e.global::<u32>(PLAYER_CHARACTER);
    if actor == 0
        || !e.vcall(actor, 0x100, &args![]).bool()
        || e.vcall(actor, 0x22c, &args![0u32]).bool()
        || e.call(GET_CURRENT_PROCESS_TYPE, &args![actor]).u32() != 0
        || e.vcall(actor, 0x1d0, &args![]).u32() == 0
        || player_character_is_actor_perceived(e, Ptr::new(player), actor).is_null()
    {
        return;
    }
    let own_position = e.vcall(this.addr(), 0x1f4, &args![]).u32();
    let actor_position = e.vcall(actor, 0x1f4, &args![]).u32();
    e.call(
        VECTOR_SUBTRACT,
        &args![actor_position, f.at(0xffff_ffb4), own_position],
    );
    let distance = e.call(VECTOR_LENGTH, &args![f.at(0xffff_ffb4)]).f32();
    let entry = player_character_is_actor_perceived(e, Ptr::new(player), actor).addr();
    let hostile = e.mem.u8(entry + 4);
    if (*range).partial_cmp(&distance) != Some(std::cmp::Ordering::Greater) {
        return;
    }
    for offset in [0xffff_ffa0u32, 0xffff_ff94, 0xffff_ff88, 0xffff_ff7c] {
        e.call(IDENTITY_006815C0, &args![f.at(offset)]);
    }
    let node = e.vcall(actor, 0x1d0, &args![]).u32();
    let bound = e.call(NODE_GET_WORLD_BOUND, &args![node]).u32();
    copy_words(e, bound, f.at(0xffff_ff6c), 4);
    let epsilon: f32 = e.global(SCREEN_EPSILON_010718C0);
    e.call(
        SCREEN_SPACE_BOUND_BOUND,
        &args![
            camera,
            f.at(0xffff_ff6c),
            f.at(0xffff_ff88),
            f.at(0xffff_ff7c),
            epsilon
        ],
    );
    let radius_at = e.vcall(actor, 0x1dc, &args![f.at(0xffff_ff60)]).u32();
    let radius = e.mem.f32(radius_at);
    e.call(SET_FLOAT_AT_C, &args![f.at(0xffff_ff6c), radius]);
    let epsilon: f32 = e.global(SCREEN_EPSILON_010718C0);
    e.call(
        SCREEN_SPACE_BOUND_BOUND,
        &args![
            camera,
            f.at(0xffff_ff6c),
            f.at(0xffff_ffa0),
            f.at(0xffff_ff94),
            epsilon
        ],
    );
    // The box: [-0x34] is its minimum, [-0x30] its maximum.
    let low = e.mem.f32(f.at(0xffff_ffcc));
    let high = e.mem.f32(f.at(0xffff_ffd0));
    let minus_one: f64 = e.global(DOUBLE_MINUS_ONE);
    let one: f64 = e.global(DOUBLE_ONE);
    let first = e.mem.f32(f.at(0xffff_ffa0));
    let second = e.mem.f32(f.at(0xffff_ff94));
    let third = e.mem.f32(f.at(0xffff_ff8c));
    let fourth = e.mem.f32(f.at(0xffff_ff80));
    let hidden = (f64::from(first) == minus_one && f64::from(second) == one)
        || (low > first && low > second)
        || (high < first && high < second)
        || (f64::from(third) == minus_one && f64::from(fourth) == one)
        || (low > third && low > fourth)
        || (high < third && high < fourth);
    if hidden {
        return;
    }
    let line_of_sight = e
        .call(
            ACTOR_LINE_OF_SIGHT,
            &args![player, 0u32, actor, 1u32, 0u32, 0u32],
        )
        .bool();
    if !line_of_sight {
        return;
    }
    e.set(this, PlayerCharacter::pPlayersTargetActor, Ptr::new(actor));
    if hostile != 0 {
        *best = actor;
        *range = distance;
    }
}

// Translated from 00964260 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PlayerCharacter::UpdateAutoAimActor` (Xbox PDB): picks the actor the
/// player is auto aiming at and the point the bullets aim for. Nothing
/// happens without a 3D node (virtual `+0x1d0`). The process list is scanned
/// for the closest hostile perceived actor (see [`auto_aim_candidate`]) within
/// the range setting `011ced64` whose screen bounds are in the box of half
/// size `011cefa4`; the last one in sight is the player's target actor
/// (`+0xd44`), the closest hostile one the best actor. A pick from the
/// player's aim ray (`TES::Pick`, or a ray from the camera when the object
/// at `011f2250` reports mode 4 or a flag is set) sets the targeted 3D
/// (`+0xd3c`) and the bullet aim point (`+0xd30`). Depending on the mode
/// the best or the picked actor is re-checked with a camera ray
/// (`008cfbc0`), and may set the target actor and the aimed body part
/// (`011a59f0`). The best actor is dropped when it is also the picked one and
/// the player's weapon is not a melee type (`006450c0`); it is stored in
/// `pAutoAimActor` (`+0xd2c`). `bTarget3DDistant` (`+0xd40`) says whether the
/// picked actor is farther away than the word at `+0x18` of the screen
/// object, and the iron sights and VATS image space updates run. The
/// exception frame and the stack cookie are not translated; the 0x3cc byte
/// frame is emulated so that the locals the callees receive by address keep
/// the layout the game gives them.
pub fn player_character_update_auto_aim_actor(e: &mut Engine, this: Ptr<PlayerCharacter>) {
    // `EBP` is on a 16-byte boundary in the exe (`AND ESP,0xfffffff0` before `MOV EBP,ESP`).
    e.with_stack(AUTO_AIM_FRAME + 0x10, |e, raw| {
        let frame = Frame {
            ebp: (raw.addr() + AUTO_AIM_EBP + 0xf) & !0xf,
        };
        update_auto_aim_actor_in_frame(e, frame, this)
    });
}

fn update_auto_aim_actor_in_frame(e: &mut Engine, f: Frame, this: Ptr<PlayerCharacter>) {
    if e.vcall(this.addr(), 0x1d0, &args![]).u32() == 0 {
        return;
    }
    let mut best = 0u32;
    let mut range = setting_float(e, SETTING_011CED64);
    let holder = e.call(GET_CAMERA_HOLDER, &args![]).u32();
    let camera = e.call(READ_DWORD_AT_AC, &args![holder]).u32();
    let scale: f64 = e.global(AIM_BOX_SCALE_01023B88);
    let half = (f64::from(setting_float(e, SETTING_011CEFA4)) * scale) as f32;
    let scale: f64 = e.global(AIM_BOX_SCALE_01023B88);
    let negative_half = (-f64::from(setting_float(e, SETTING_011CEFA4)) * scale) as f32;
    e.call(
        NI_POINT2_NEW,
        &args![f.at(0xffff_ffcc), negative_half, half],
    );
    e.set(this, PlayerCharacter::pPlayersTargetActor, Ptr::new(0));
    let mut index = 0u32;
    loop {
        let array = e.call(ADD_4_TO_ADDRESS, &args![PROCESS_LISTS]).u32();
        let count = e.call(FUNCTION_005BE5C0, &args![array, 0u32]).u32();
        if index >= count {
            break;
        }
        let array = e.call(ADD_4_TO_ADDRESS, &args![PROCESS_LISTS]).u32();
        let actor = e.call(PICK_RESULT_GET, &args![array, index]).u32();
        auto_aim_candidate(e, f, this, camera, actor, &mut best, &mut range);
        index += 1;
    }
    e.call(FUNCTION_004A3C20, &args![f.at(0xffff_fea0)]);
    e.call(FUNCTION_0062A190, &args![f.at(0xffff_fe20), 0x25u32, best]);
    if best != 0 {
        e.call(
            FUNCTION_0059CEB0,
            &args![f.at(0xffff_fea0), f.at(0xffff_fe20)],
        );
    }
    let position = e.vcall(this.addr(), 0x1f4, &args![]).u32();
    copy_words(e, position, f.at(0xffff_fe14), 3);
    let reach = setting_float(e, SETTING_011CED64);
    e.call(
        FUNCTION_00416870,
        &args![f.at(0xffff_fe08), 0.0f32, reach, 0.0f32],
    );
    let origin = f.at(0xffff_fe14);
    let direction = f.at(0xffff_fe08);
    if e.call(READ_FIELD_8, &args![OBJECT_011F2250]).u32() == 4
        || e.global::<u8>(FLAG_011E07B8) != 0
    {
        // The ray follows the player's view: rotate the (0, reach, 0)
        // direction by the heading and looking angles.
        let lift = e.call(FUNCTION_0070C3B0, &args![this]).f32();
        let z = e.mem.f32(origin + 8);
        e.mem
            .set_f32(origin + 8, (f64::from(lift) + f64::from(z)) as f32);
        e.call(IDENTITY_006815C0, &args![f.at(0xffff_fde4)]);
        e.call(IDENTITY_006815C0, &args![f.at(0xffff_fdc0)]);
        let heading = e.vcall(this.addr(), 0x2bc, &args![0u32]).f32();
        e.call(FUNCTION_004A0C90, &args![f.at(0xffff_fde4), heading]);
        let looking = e.call(MOBILE_OBJECT_GET_LOOKING, &args![this]).f32();
        e.call(MATRIX_MAKE_X_ROTATION, &args![f.at(0xffff_fdc0), looking]);
        let product = e
            .call(
                MATRIX_MULTIPLY,
                &args![f.at(0xffff_fde4), f.at(0xffff_fd9c), f.at(0xffff_fdc0)],
            )
            .u32();
        copy_words(e, product, f.at(0xffff_fde4), 9);
        let rotated = e
            .call(
                FUNCTION_004B4500,
                &args![f.at(0xffff_fde4), f.at(0xffff_fd84), direction],
            )
            .u32();
        let sum = e
            .call(VECTOR_ADD, &args![rotated, f.at(0xffff_fd90), origin])
            .u32();
        copy_words(e, sum, direction, 3);
    } else {
        let matrix = e.call(FUNCTION_00461130, &args![camera]).u32();
        copy_words(e, matrix, f.at(0xffff_fd60), 9);
        e.call(IDENTITY_006815C0, &args![f.at(0xffff_fd3c)]);
        e.call(IDENTITY_006815C0, &args![f.at(0xffff_fd30)]);
        for axis in 0..3u32 {
            e.call(
                FUNCTION_005DF560,
                &args![
                    f.at(0xffff_fd60),
                    axis,
                    f.at(0xffff_fd34),
                    f.at(0xffff_fd38),
                    f.at(0xffff_fd30)
                ],
            );
            e.call(
                FUNCTION_005DF510,
                &args![f.at(0xffff_fd3c), axis, f.at(0xffff_fd30)],
            );
        }
        let position = e.call(FUNCTION_005DF5B0, &args![this]).u32();
        copy_words(e, position, origin, 3);
        let rotated = e
            .call(
                FUNCTION_004B4500,
                &args![f.at(0xffff_fd3c), f.at(0xffff_fd18), direction],
            )
            .u32();
        let sum = e
            .call(VECTOR_ADD, &args![rotated, f.at(0xffff_fd24), origin])
            .u32();
        copy_words(e, sum, direction, 3);
    }
    e.call(FUNCTION_004A3DA0, &args![f.at(0xffff_fea0), origin]);
    e.call(FUNCTION_004A3EB0, &args![f.at(0xffff_fea0), direction]);
    e.call(FUNCTION_00931ED0, &args![this, f.at(0xffff_fd14)]);
    e.call(FUNCTION_008C71B0, &args![f.at(0xffff_fd10), 0u32]);
    e.call(FUNCTION_004A39F0, &args![f.at(0xffff_fd10), 0x25u32]);
    let filter = e.call(FUNCTION_004A3A20, &args![f.at(0xffff_fd14)]).u32();
    e.call(FUNCTION_0059CE80, &args![f.at(0xffff_fd10), filter]);
    let word = e.mem.u32(f.at(0xffff_fd10));
    e.call(FUNCTION_004A3F70, &args![f.at(0xffff_fea0), word]);
    let previous = e.call(FUNCTION_00888960, &args![]).u8();
    e.call(WORLD_SET_PICK_COUNT_OVERRIDE, &args![1u32]);
    let tes = e.global::<u32>(TES_SINGLETON);
    let picked = e.call(TES_PICK, &args![tes, f.at(0xffff_fea0)]).u32();
    e.call(WORLD_SET_PICK_COUNT_OVERRIDE, &args![u32::from(previous)]);
    let picked_reference = if picked != 0 {
        e.call(FIND_REFERENCE_FOR_3D, &args![picked]).u32()
    } else {
        0
    };
    e.call(SMART_POINTER_ASSIGN, &args![this.addr() + 0xd3c, picked]);
    let ray_length = e.mem.f32(f.at(0xffff_fee0));
    let difference = e
        .call(
            VECTOR_SUBTRACT,
            &args![direction, f.at(0xffff_fce8), origin],
        )
        .u32();
    let scaled = e
        .call(
            FUNCTION_0045BB20,
            &args![difference, f.at(0xffff_fcf4), ray_length],
        )
        .u32();
    let aim_point = e
        .call(VECTOR_ADD, &args![origin, f.at(0xffff_fcdc), scaled])
        .u32();
    copy_words(e, aim_point, this.addr() + 0xd30, 3);
    let process = e.call(PLAYER_GET_PROCESS, &args![this]).u32();
    let weapon = e.vcall(process, 0x148, &args![]).u32();
    let weapon_word = if weapon != 0 {
        e.call(READ_FIELD_8, &args![weapon]).u32()
    } else {
        0
    };
    if e.call(READ_FIELD_8, &args![OBJECT_011F2250]).u32() != 2
        && picked_reference != 0
        && picked_reference == best
    {
        let node = e.vcall(best, 0x1d0, &args![]).u32();
        let reach = setting_float(e, SETTING_011CED64);
        e.mem.set_f32(f.at(0xffff_fcc4), reach);
        e.call(IDENTITY_006815C0, &args![f.at(0xffff_fcb8)]);
        e.call(IDENTITY_006815C0, &args![f.at(0xffff_fcac)]);
        camera_window_ray(e, f.at(0xffff_fcb8), f.at(0xffff_fcac));
        let hit = cast_aim_ray(
            e,
            node,
            f.at(0xffff_fcb8),
            f.at(0xffff_fcac),
            f.at(0xffff_fcc4),
        );
        if hit != 0 {
            let reference = e.call(FIND_REFERENCE_FOR_3D, &args![hit]).u32();
            if reference != 0 && e.vcall(reference, 0x100, &args![]).bool() {
                e.set(
                    this,
                    PlayerCharacter::pPlayersTargetActor,
                    Ptr::new(reference),
                );
            }
        }
    } else if e.call(READ_FIELD_8, &args![OBJECT_011F2250]).u32() == 2
        && e.global::<u32>(POINTER_011F21CC) != 0
        && weapon_word != 0
        && !e.call(WEAPON_IS_MELEE_TYPE, &args![weapon_word]).bool()
        && !e.call(FUNCTION_004B71D0, &args![]).bool()
    {
        let target_object = e.global::<u32>(POINTER_011F21CC);
        let node = e.vcall(target_object, 0x1d0, &args![]).u32();
        let player = e.global::<u32>(PLAYER_CHARACTER);
        let own_node = e.vcall(player, 0x1d0, &args![]).u32();
        let own_position = e.call(ADD_8C_TO_ADDRESS, &args![own_node]).u32();
        let target_node = e.vcall(target_object, 0x1d0, &args![]).u32();
        let target_position = e.call(ADD_8C_TO_ADDRESS, &args![target_node]).u32();
        let between = e
            .call(
                VECTOR_SUBTRACT,
                &args![target_position, f.at(0xffff_fc94), own_position],
            )
            .u32();
        let length = e.call(VECTOR_LENGTH, &args![between]).f32();
        let pad: f64 = e.global(AIM_DISTANCE_PAD_0102E430);
        e.mem
            .set_f32(f.at(0xffff_fc90), (f64::from(length) + pad) as f32);
        e.call(IDENTITY_006815C0, &args![f.at(0xffff_fc84)]);
        e.call(IDENTITY_006815C0, &args![f.at(0xffff_fc78)]);
        camera_window_ray(e, f.at(0xffff_fc84), f.at(0xffff_fc78));
        let hit = cast_aim_ray(
            e,
            node,
            f.at(0xffff_fc84),
            f.at(0xffff_fc78),
            f.at(0xffff_fc90),
        );
        if hit != 0 {
            let reference = e.call(FIND_REFERENCE_FOR_3D, &args![hit]).u32();
            e.set_global(AIMED_BODY_PART, 0xffff_ffffu32);
            if e.vcall(reference, 0x100, &args![]).bool() {
                let piece = if e.vcall(hit, 0xc, &args![]).u32() != 0 {
                    let part = e.vcall(hit, 0xc, &args![]).u32();
                    e.call(FUNCTION_008B3EF0, &args![reference, part]).u32()
                } else {
                    let part = fn_009611e0(e, Ptr::new(hit));
                    e.call(FUNCTION_008B3EF0, &args![reference, part]).u32()
                };
                e.call(FUNCTION_007F3B40, &args![piece]);
                let base = e.call(GET_BASE_FORM, &args![reference]).u32();
                let body_parts = e.vcall(base, 0x180, &args![]).u32();
                if body_parts != 0 {
                    let part = e
                        .call(BODY_PART_DATA_GET_BODY_PART, &args![body_parts, piece])
                        .u32();
                    if part != 0 {
                        let byte = e.call(FUNCTION_005E5190, &args![part]).u8();
                        e.set_global(AIMED_BODY_PART, i32::from(byte as i8) as u32);
                    }
                }
            }
        }
    }
    if picked_reference != 0 && picked_reference == best {
        let weapon = e.call(ACTOR_GET_CURRENT_WEAPON, &args![this]).u32();
        if weapon != 0 {
            let weapon = e.call(ACTOR_GET_CURRENT_WEAPON, &args![this]).u32();
            if !e.call(WEAPON_IS_MELEE_TYPE, &args![weapon]).bool() {
                best = 0;
            }
        }
    }
    e.set(this, PlayerCharacter::pAutoAimActor, Ptr::new(best));
    let aimed = f.at(0xffff_fc50);
    e.call(IDENTITY_006815C0, &args![aimed]);
    if picked_reference != 0 {
        let position = e.vcall(picked_reference, 0x1f4, &args![]).u32();
        let difference = e
            .call(VECTOR_SUBTRACT, &args![position, f.at(0xffff_fc44), origin])
            .u32();
        copy_words(e, difference, aimed, 3);
    }
    let distance = e.call(VECTOR_LENGTH, &args![aimed]).f32();
    let distant = if picked_reference != 0 {
        let screen = e.call(GLOBAL_OBJECT_GETTER, &args![]).u32();
        let limit = fn_009611e0(e, Ptr::new(screen)) as i32;
        f64::from(distance) > f64::from(limit)
    } else {
        false
    };
    e.set(this, PlayerCharacter::bTarget3DDistant, u8::from(distant));
    player_character_update_image_space_ironsights(e, this);
    player_character_update_image_space_vats(e, this);
    e.call(FUNCTION_0059CEE0, &args![f.at(0xffff_fe20)]);
}

/// The vector of three zero floats at `011f426c`.
const ZERO_VECTOR: u32 = 0x011f_426c;
/// `VATS::GetCurrentAction`-like getter, `ECX` = `011f2250`: the current VATS
/// action (its `+0x0c` is the target, `+0x10` the body part index, `+4` a flag).
const FUNCTION_009C71C0: u32 = 0x009c_71c0;
/// The function at `004ad010` (`ECX` = weapon): a weapon flag.
const FUNCTION_004AD010: u32 = 0x004a_d010;
/// `BGSBodyPartData::GetBodyPart_ov2` (Xbox PDB), `ECX` = the data, `(index)`.
const FUNCTION_005E5130: u32 = 0x005e_5130;
/// `ECX` = a body part: its node name.
const FUNCTION_0043B230: u32 = 0x0043_b230;
/// The function at `0062bfb0`, cdecl `(out, node)`: returns a pointer to a vector.
const FUNCTION_0062BFB0: u32 = 0x0062_bfb0;
/// `ECX` = the actor: a pointer to a 20 byte VATS aim record or null.
const FUNCTION_008D8B80: u32 = 0x008d_8b80;
/// The function at `00524c40` `(matrix, out, vector)`: the vector rotated by the matrix.
const FUNCTION_00524C40: u32 = 0x0052_4c40;
/// `NiPoint3::operator!=` `(this, other)`: true when the vectors differ.
const VECTOR_NOT_EQUAL: u32 = 0x0043_9090;
/// `(minimum, maximum)`, cdecl: a random float in the range.
const FUNCTION_00476B70: u32 = 0x0047_6b70;
/// The function at `004579e0`, cdecl `(float)`: a square root.
const FUNCTION_004579E0: u32 = 0x0045_79e0;
/// The functions at `004e4470` / `004e44b0`, cdecl `(angle)`: cosine and sine.
const FUNCTION_004E4470: u32 = 0x004e_4470;
const FUNCTION_004E44B0: u32 = 0x004e_44b0;
/// `ECX` = the actor: a float.
const FUNCTION_004E4600: u32 = 0x004e_4600;
/// `NiMatrix3::Inverse` (Xbox PDB) `(this, out)`: returns `out`.
const MATRIX_INVERSE: u32 = 0x004b_45b0;
/// Cdecl `(vector)`: the heading angle of the vector (`GetZAngleFromVector`).
const FUNCTION_004B13C0: u32 = 0x004b_13c0;
/// Cdecl `(float)`: an arc tangent.
const FUNCTION_004B1460: u32 = 0x004b_1460;
/// Cdecl `(float)`: an arc sine.
const FUNCTION_004B5530: u32 = 0x004b_5530;
/// Cdecl `(float)`: a float function.
const FUNCTION_004019B0: u32 = 0x0040_19b0;
/// `CombatUtilities::GetProjectedPointForTarget_ov4` (Xbox PDB), cdecl, 13 words.
const FUNCTION_009A8F00: u32 = 0x009a_8f00;
/// `CombatUtilities::CalculateProjectileLOS` (Xbox PDB), cdecl, 12 words.
const FUNCTION_009A6E90: u32 = 0x009a_6e90;
/// Cdecl `(actor, vector xyz, float, 0)`: a pitch angle.
const FUNCTION_009A7D50: u32 = 0x009a_7d50;
/// `ECX` = a matrix, `(index, out)`: reads a column of the matrix.
const FUNCTION_00439F50: u32 = 0x0043_9f50;
/// `ECX` = a weapon, `(has mod, 0, actor)`: the number of projectiles.
const FUNCTION_00525B20: u32 = 0x0052_5b20;
/// `ECX` = the player.
const FUNCTION_00524D10: u32 = 0x0052_4d10;
/// The settings of the aim offset limits and the aim jitter.
const SETTING_011CF438: u32 = 0x011c_f438;
const SETTING_011CF0BC: u32 = 0x011c_f0bc;
const SETTING_011CE90C: u32 = 0x011c_e90c;
const SETTING_011CFC20: u32 = 0x011c_fc20;
const SETTING_011CE780: u32 = 0x011c_e780;
const SETTING_011CFAF8: u32 = 0x011c_faf8;
const SETTING_011CF9B0: u32 = 0x011c_f9b0;
const SETTING_011CFD10: u32 = 0x011c_fd10;
/// `FLT_MIN` as a double (the "unset" angle default).
const FLOAT_MIN_DOUBLE: u32 = 0x0107_7e90;
/// -pi, 2 pi and pi as doubles of floats.
const NEGATIVE_PI_DOUBLE: u32 = 0x0101_ff58;
const TWO_PI_DOUBLE: u32 = 0x0101_ff48;
const PI_DOUBLE: u32 = 0x0101_ff40;
/// 1.553343 and -1.553343 as doubles and floats (the pitch limits).
const PITCH_LIMIT_DOUBLE: u32 = 0x0108_b3f8;
const PITCH_LIMIT_FLOAT: u32 = 0x0108_a7f8;
const NEGATIVE_PITCH_LIMIT_DOUBLE: u32 = 0x0108_a7f0;
const NEGATIVE_PITCH_LIMIT_FLOAT: u32 = 0x0108_a7ec;
/// Degrees to radians (a double).
const DEGREES_TO_RADIANS: u32 = 0x0102_3128;
/// 1.25, 0.1 and 10.0 as doubles.
const SCALE_ONE_AND_A_QUARTER: u32 = 0x0102_1798;
const STEP_FRACTION: u32 = 0x0107_2758;
const AIM_LIFT: u32 = 0x0102_0758;
/// 16.0 (float): the radius used for a bound with no radius.
const DEFAULT_BOUND_RADIUS: u32 = 0x0101_e57c;
/// 2 pi as a float.
const TWO_PI_FLOAT: u32 = 0x0101_ff50;
/// The node name `Weapon`.
const WEAPON_NODE_NAME: u32 = 0x0101_3be8;

/// Wraps an angle into `-pi ..= pi` the way the aim code does.
fn wrap_angle(e: &mut Engine, angle: f32) -> f32 {
    let low: f64 = e.global(NEGATIVE_PI_DOUBLE);
    let turn: f64 = e.global(TWO_PI_DOUBLE);
    let mut angle = angle;
    while f64::from(angle) < low {
        angle = (f64::from(angle) + turn) as f32;
    }
    let high: f64 = e.global(PI_DOUBLE);
    while f64::from(angle) > high {
        angle = (f64::from(angle) - turn) as f32;
    }
    angle
}

/// Limits a pitch to the 1.553343 range the aim code uses.
fn clamp_pitch(e: &mut Engine, pitch: f32) -> f32 {
    let mut pitch = pitch;
    let high: f64 = e.global(PITCH_LIMIT_DOUBLE);
    if f64::from(pitch) > high {
        pitch = e.global(PITCH_LIMIT_FLOAT);
    }
    let low: f64 = e.global(NEGATIVE_PITCH_LIMIT_DOUBLE);
    if f64::from(pitch) < low {
        pitch = e.global(NEGATIVE_PITCH_LIMIT_FLOAT);
    }
    pitch
}

/// `value` limited to `-limit ..= limit` by the two compares of the aim code.
fn clamp_symmetric(value: f32, limit: f32) -> f32 {
    if limit < value {
        limit
    } else if value < -limit {
        -limit
    } else {
        value
    }
}

/// Size of the emulated frame of [`fn_00965620`] (its locals reach down to
/// `[EBP - 0x20c]`), and where `EBP` is in it.
const AIM_OFFSETS_FRAME: u32 = 0x240;
const AIM_OFFSETS_EBP: u32 = 0x220;

// Translated from 00965620 (decompiled, FalloutNV.exe 1.4.0.525)
/// Computes how far the aim of the player is off the direction to the
/// target and adds it to `*heading` and `*pitch` (the wobble code calls it
/// with the offsets of the frame, see [`player_character_add_gun_wobble`]).
/// `actor` is the actor whose heading and pitch are the reference (or 0, then
/// `default_heading` and `default_pitch` are), `aim_node` the node the
/// shot comes from (0: the player's root node), `origin_override` a vector
/// to use as the origin instead of the node's position (or 0). The target
/// point is the VATS target's node (when the object at `011f2250` reports mode
/// 4 and an action with a target exists), the player's targeted actor
/// (`00963730`) or the stored bullet aim point (`+0xd30`); without one the
/// function leaves the offsets alone. For a VATS target the projectile's
/// flight is searched for a free line (`009a8f00`, `009a6e90`) and the
/// actor's `+0xd0` float is set; the angles are then limited by settings
/// that depend on the weapon (melee or none, VATS, `00524d10`) and the
/// offsets of a multi-projectile weapon and the VATS jitter are added.
/// `*heading` is wrapped into `-pi ..= pi` and `*pitch` limited to 1.553343.
/// An early return happens when the iron sights are up with a weapon that
/// has a flagged mod or the mode word is 0 and the weapon flags forbid it.
#[allow(clippy::too_many_arguments)]
pub fn fn_00965620(
    e: &mut Engine,
    this: Ptr<PlayerCharacter>,
    actor: u32,
    aim_node: u32,
    heading: Ptr,
    pitch: Ptr,
    origin_override: u32,
    default_heading: f32,
    default_pitch: f32,
) {
    e.with_stack(AIM_OFFSETS_FRAME, |e, block| {
        let frame = Frame {
            ebp: block.addr() + AIM_OFFSETS_EBP,
        };
        aim_offsets_in_frame(
            e,
            frame,
            this,
            actor,
            aim_node,
            heading.addr(),
            pitch.addr(),
            origin_override,
            default_heading,
            default_pitch,
        );
    });
}

/// The weapon's current weapon form of the player (`Actor::GetCurrentWeapon`).
fn current_weapon(e: &mut Engine, this: Ptr<PlayerCharacter>) -> u32 {
    e.call(ACTOR_GET_CURRENT_WEAPON, &args![this]).u32()
}

/// Whether the player's current weapon is a melee type (`006450c0`).
fn current_weapon_is_melee(e: &mut Engine, this: Ptr<PlayerCharacter>) -> bool {
    let weapon = current_weapon(e, this);
    e.call(WEAPON_IS_MELEE_TYPE, &args![weapon]).bool()
}

#[allow(clippy::too_many_arguments)]
fn aim_offsets_in_frame(
    e: &mut Engine,
    f: Frame,
    this: Ptr<PlayerCharacter>,
    actor: u32,
    aim_node: u32,
    heading_out: u32,
    pitch_out: u32,
    origin_override: u32,
    default_heading: f32,
    default_pitch: f32,
) {
    let vec_v = f.at(0xffff_ffdc);
    let vec_s = f.at(0xffff_ffe8);
    let vec_w = f.at(0xffff_ffc8);
    let vec_z = f.at(0xffff_ffa4);
    let vec_d = f.at(0xffff_ff44);
    let matrix_m = f.at(0xffff_ff64);
    let player = e.global::<u32>(PLAYER_CHARACTER);
    copy_words(e, ZERO_VECTOR, vec_v, 3);
    let vats_action = e.call(FUNCTION_009C71C0, &args![OBJECT_011F2250]).u32();
    let mut in_vats_target = false;
    let mut node = 0u32;
    let source = if origin_override != 0 {
        origin_override
    } else if aim_node != 0 {
        e.call(ADD_8C_TO_ADDRESS, &args![aim_node]).u32()
    } else {
        ZERO_VECTOR
    };
    copy_words(e, source, vec_s, 3);
    if e.call(ACTOR_GET_IRON_SIGHTS, &args![this]).bool()
        && e.call(READ_FIELD_8, &args![OBJECT_011F2250]).u32() == 0
        && current_weapon(e, this) != 0
    {
        let weapon = current_weapon(e, this);
        if e.call(FUNCTION_004AD030, &args![weapon]).bool() {
            let process = e.get(this, PlayerCharacter::pCurrentProcess).addr();
            if process != 0 && e.vcall(process, 0x148, &args![]).u32() != 0 {
                let item = e.vcall(process, 0x148, &args![]).u32();
                if e.call(ITEM_CHANGE_HAS_MOD, &args![item, 0xeu32]).bool() {
                    return;
                }
            }
        }
        let weapon = current_weapon(e, this);
        if e.call(FUNCTION_004AD010, &args![weapon]).bool() {
            let weapon = current_weapon(e, this);
            if !e.call(FUNCTION_004AD030, &args![weapon]).bool() {
                return;
            }
        }
    }
    if e.call(READ_FIELD_8, &args![OBJECT_011F2250]).u32() == 4
        && vats_action != 0
        && e.mem.u32(vats_action + 0xc) != 0
    {
        in_vats_target = true;
        let target = e.mem.u32(vats_action + 0xc);
        node = e.vcall(target, 0x1d0, &args![]).u32();
    }
    let mut jitter_heading = 0.0f32;
    let mut jitter_pitch = 0.0f32;
    let ranged = aim_node != 0 && current_weapon(e, this) != 0 && !current_weapon_is_melee(e, this);
    if !ranged {
        // No node, no weapon or a melee weapon: the offset to the player's
        // targeted actor or to the stored bullet aim point.
        let targeted = fn_00963730(e, this).addr();
        if targeted != 0 {
            let targeted = fn_00963730(e, this).addr();
            let root = e.call(PLAYER_GET_ROOT_NODE, &args![this, 0u32]).u32();
            let root_bound = e.call(NODE_GET_WORLD_BOUND, &args![root]).u32();
            let root_bound = e.call(IDENTITY_006815C0, &args![root_bound]).u32();
            let target_node = e.vcall(targeted, 0x1d0, &args![]).u32();
            let bound = e.call(NODE_GET_WORLD_BOUND, &args![target_node]).u32();
            let bound = e.call(IDENTITY_006815C0, &args![bound]).u32();
            let difference = e
                .call(
                    VECTOR_SUBTRACT,
                    &args![bound, f.at(0xffff_fee0), root_bound],
                )
                .u32();
            copy_words(e, difference, vec_v, 3);
        } else if e
            .call(VECTOR_NOT_EQUAL, &args![this.addr() + 0xd30, ZERO_VECTOR])
            .bool()
        {
            let root = e.call(PLAYER_GET_ROOT_NODE, &args![this, 0u32]).u32();
            let root_bound = e.call(NODE_GET_WORLD_BOUND, &args![root]).u32();
            let root_bound = e.call(IDENTITY_006815C0, &args![root_bound]).u32();
            let difference = e
                .call(
                    VECTOR_SUBTRACT,
                    &args![this.addr() + 0xd30, f.at(0xffff_fed4), root_bound],
                )
                .u32();
            copy_words(e, difference, vec_v, 3);
        }
    } else if in_vats_target {
        let target = e.mem.u32(vats_action + 0xc);
        if e.vcall(target, 0x100, &args![]).bool()
            && (actor == 0 || e.call(FORM_GET_TYPE, &args![actor]).u32() != 0x3e)
        {
            let reference = e.mem.u32(vats_action + 0xc);
            let base = e.call(GET_BASE_FORM, &args![reference]).u32();
            let parts = e.vcall(base, 0x180, &args![]).u32();
            if parts != 0 {
                let index = e.mem.u32(vats_action + 0x10);
                let part = e.call(FUNCTION_005E5130, &args![parts, index]).u32();
                if part != 0 || index == 0xffff_ffff {
                    let name = if index == 0xffff_ffff {
                        WEAPON_NODE_NAME
                    } else {
                        e.call(FUNCTION_0043B230, &args![part]).u32()
                    };
                    let model = e.vcall(reference, 0x1d0, &args![]).u32();
                    let found = e.call(FIND_NODE_BY_NAME, &args![model, name]).u32();
                    if found != 0 {
                        node = found;
                    }
                }
            }
        }
        if node == 0 {
            return;
        }
        let centre = e.call(ADD_8C_TO_ADDRESS, &args![node]).u32();
        copy_words(e, centre, vec_w, 3);
        if e.mem.u8(vats_action + 4) != 0 {
            e.vcall(node, 0xc, &args![]);
            copy_words(e, ZERO_VECTOR, vec_z, 3);
            if actor == 0 || e.call(FORM_GET_TYPE, &args![actor]).u32() != 0x3e {
                let point = e
                    .call(FUNCTION_0062BFB0, &args![f.at(0xffff_fec8), node])
                    .u32();
                copy_words(e, point, vec_z, 3);
            }
            if actor != 0 && e.call(FUNCTION_008D8B80, &args![actor]).u32() != 0 {
                let record = e.call(FUNCTION_008D8B80, &args![actor]).u32();
                let target = e.mem.u32(vats_action + 0xc);
                e.mem.set_u32(record, target);
                e.call(SMART_POINTER_ASSIGN, &args![record + 4, node]);
                copy_words(e, vec_z, record + 8, 3);
            }
            let matrix = e.call(FUNCTION_00461130, &args![node]).u32();
            let rotated = e
                .call(FUNCTION_00524C40, &args![matrix, f.at(0xffff_febc), vec_z])
                .u32();
            copy_words(e, rotated, vec_w, 3);
        } else {
            let bound = e.call(NODE_GET_WORLD_BOUND, &args![node]).u32();
            let radius = e.call(READ_FLOAT_AT_C, &args![bound]).f64();
            let position = if radius == 0.0 {
                e.call(ADD_8C_TO_ADDRESS, &args![node]).u32()
            } else {
                let bound = e.call(NODE_GET_WORLD_BOUND, &args![node]).u32();
                e.call(IDENTITY_006815C0, &args![bound]).u32()
            };
            copy_words(e, position, vec_w, 3);
        }
        let difference = e
            .call(VECTOR_SUBTRACT, &args![vec_w, f.at(0xffff_feac), vec_s])
            .u32();
        copy_words(e, difference, vec_v, 3);
        if e.mem.u8(vats_action + 4) == 0 {
            let bound = e.call(NODE_GET_WORLD_BOUND, &args![node]).u32();
            let mut radius = e.call(READ_FLOAT_AT_C, &args![bound]).f32();
            if f64::from(radius) == 0.0 {
                radius = e.global(DEFAULT_BOUND_RADIUS);
            }
            let upper = setting_float(e, SETTING_011CF438);
            let lower = setting_float(e, SETTING_011CF0BC);
            let random = e.call(FUNCTION_00476B70, &args![lower, upper]).f64();
            let radius = (random * f64::from(radius)) as f32;
            let length = e.call(VECTOR_LENGTH, &args![vec_v]).f32();
            let squares = (f64::from(radius) * f64::from(radius)
                + f64::from(length) * f64::from(length)) as f32;
            let hypotenuse = e.call(FUNCTION_004579E0, &args![squares]).f32();
            let tau: f32 = e.global(TWO_PI_FLOAT);
            let angle = e.call(FUNCTION_00476B70, &args![0.0f32, tau]).f32();
            let cosine = e.call(FUNCTION_004E4470, &args![angle]).f64();
            jitter_heading = (cosine * (f64::from(radius) / f64::from(hypotenuse))) as f32;
            let sine = e.call(FUNCTION_004E44B0, &args![angle]).f64();
            jitter_pitch = (sine * (f64::from(radius) / f64::from(hypotenuse))) as f32;
        }
    } else {
        let targeted = fn_00963730(e, this).addr();
        if targeted != 0 {
            let targeted = fn_00963730(e, this).addr();
            let target_node = e.vcall(targeted, 0x1d0, &args![]).u32();
            let bound = e.call(NODE_GET_WORLD_BOUND, &args![target_node]).u32();
            let bound = e.call(IDENTITY_006815C0, &args![bound]).u32();
            let difference = e
                .call(VECTOR_SUBTRACT, &args![bound, f.at(0xffff_fea0), vec_s])
                .u32();
            copy_words(e, difference, vec_v, 3);
        } else if aim_node != 0
            && e.call(VECTOR_NOT_EQUAL, &args![this.addr() + 0xd30, ZERO_VECTOR])
                .bool()
        {
            let difference = e
                .call(
                    VECTOR_SUBTRACT,
                    &args![this.addr() + 0xd30, f.at(0xffff_fe94), vec_s],
                )
                .u32();
            copy_words(e, difference, vec_v, 3);
        }
    }
    let minimum: f64 = e.global(FLOAT_MIN_DOUBLE);
    let use_angles = actor != 0
        || (f64::from(default_pitch) != minimum && f64::from(default_heading) != minimum);
    if !e.call(VECTOR_NOT_EQUAL, &args![vec_v, ZERO_VECTOR]).bool() {
        return;
    }
    copy_words(e, vec_v, vec_d, 3);
    e.call(FUNCTION_004A0C10, &args![vec_d]);
    e.call(IDENTITY_006815C0, &args![matrix_m]);
    if !use_angles {
        let matrix = if aim_node != 0 {
            e.call(FUNCTION_00461130, &args![aim_node]).u32()
        } else {
            let root = e.call(PLAYER_GET_ROOT_NODE, &args![this, 0u32]).u32();
            e.call(FUNCTION_00461130, &args![root]).u32()
        };
        copy_words(e, matrix, matrix_m, 9);
        let inverse = e
            .call(MATRIX_INVERSE, &args![matrix_m, f.at(0xffff_fe70)])
            .u32();
        let local = e
            .call(FUNCTION_004B4500, &args![inverse, f.at(0xffff_fe64), vec_d])
            .u32();
        copy_words(e, local, vec_d, 3);
        if !in_vats_target {
            let global = e
                .call(
                    FUNCTION_004B4500,
                    &args![WOBBLE_RESULT_MATRIX, f.at(0xffff_fe58), vec_d],
                )
                .u32();
            copy_words(e, global, vec_d, 3);
        }
    }
    let direction_y = e.mem.f32(vec_d + 4);
    let mut heading_raw: f32;
    let mut pitch_raw: f32;
    if f64::from(direction_y) == 0.0 {
        heading_raw = 0.0;
        pitch_raw = 0.0;
    } else {
        let mut special = false;
        if in_vats_target && actor != 0 {
            let base = e.call(GET_BASE_FORM, &args![actor]).u32();
            special = fn_00966980(e, Ptr::new(base)) > 0.0;
        }
        if special {
            if e.call(FORM_GET_TYPE, &args![actor]).u32() == 0x3e {
                find_clear_shot(e, f, actor, vats_action);
            }
            if use_angles {
                heading_raw = e.call(FUNCTION_004B13C0, &args![vec_d]).f32();
                let reference = aim_pitch(e, actor, vec_v);
                pitch_raw = -reference as f32;
            } else {
                let ratio = (f64::from(e.mem.f32(vec_d)) / f64::from(direction_y)) as f32;
                heading_raw = e.call(FUNCTION_004B1460, &args![ratio]).f32();
                let reference = aim_pitch(e, actor, vec_v);
                let current = e.mem.f32(pitch_out);
                pitch_raw = (-reference - f64::from(current)) as f32;
            }
        } else {
            if use_angles {
                heading_raw = e.call(FUNCTION_004B13C0, &args![vec_d]).f32();
            } else {
                let ratio = (f64::from(e.mem.f32(vec_d)) / f64::from(direction_y)) as f32;
                heading_raw = e.call(FUNCTION_004B1460, &args![ratio]).f32();
            }
            let z = e.mem.f32(vec_d + 8);
            pitch_raw = -e.call(FUNCTION_004B5530, &args![z]).f32();
        }
    }
    let mut heading_delta: f32;
    let mut pitch_delta: f32;
    if use_angles {
        heading_raw = wrap_angle(e, heading_raw);
        pitch_raw = clamp_pitch(e, pitch_raw);
        let reference_heading = if actor != 0 {
            e.vcall(actor, 0x2bc, &args![0u32]).f32()
        } else {
            default_heading
        };
        let reference_pitch = if actor != 0 {
            e.call(MOBILE_OBJECT_GET_LOOKING, &args![actor]).f32()
        } else {
            default_pitch
        };
        let reference_heading = wrap_angle(e, reference_heading);
        let reference_pitch = clamp_pitch(e, reference_pitch);
        heading_delta = (f64::from(heading_raw) - f64::from(reference_heading)) as f32;
        pitch_delta = (f64::from(pitch_raw) - f64::from(reference_pitch)) as f32;
    } else {
        heading_delta = heading_raw;
        pitch_delta = pitch_raw;
    }
    heading_delta = wrap_angle(e, heading_delta);
    if current_weapon(e, this) == 0 || current_weapon_is_melee(e, this) {
        if f64::from(heading_delta) < 0.0 {
            heading_delta = 0.0;
        }
        pitch_delta = 0.0;
    }
    let weapon_missing_or_melee = current_weapon(e, this) == 0 || current_weapon_is_melee(e, this);
    let mut limit = if weapon_missing_or_melee {
        setting_float(e, SETTING_011CE90C)
    } else if in_vats_target {
        setting_float(e, SETTING_011CFC20)
    } else if e.call(FUNCTION_00524D10, &args![this]).bool() {
        setting_float(e, SETTING_011CE780)
    } else {
        setting_float(e, SETTING_011CFAF8)
    };
    let degrees: f64 = e.global(DEGREES_TO_RADIANS);
    limit = (f64::from(limit) * degrees) as f32;
    heading_delta = clamp_symmetric(heading_delta, limit);
    pitch_delta = clamp_symmetric(pitch_delta, limit);
    let weapon = current_weapon(e, this);
    if use_angles {
        let mut has_mod = 0u8;
        let process = e.get(this, PlayerCharacter::pCurrentProcess).addr();
        if e.vcall(process, 0x148, &args![]).u32() != 0 {
            let item = e.vcall(process, 0x148, &args![]).u32();
            has_mod = e.call(ITEM_CHANGE_HAS_MOD, &args![item, 0xcu32]).u8();
        }
        if !in_vats_target {
            let column = f.at(0xffff_fef8);
            e.call(IDENTITY_006815C0, &args![column]);
            e.call(
                FUNCTION_00439F50,
                &args![WOBBLE_RESULT_MATRIX, 1u32, column],
            );
            let angle = e.call(FUNCTION_004B13C0, &args![column]).f64();
            heading_delta = (angle + f64::from(heading_delta)) as f32;
            let z = e.mem.f32(f.at(0xffff_ff00));
            let arc = e.call(FUNCTION_004B5530, &args![z]).f64();
            pitch_delta = (-arc + f64::from(pitch_delta)) as f32;
            let heading_extra = if actor != 0 {
                fn_00966a20(e, Ptr::new(actor))
            } else {
                0.0
            };
            heading_delta = (f64::from(heading_delta) + f64::from(heading_extra)) as f32;
            let pitch_extra = if actor != 0 {
                e.call(FUNCTION_004E4600, &args![actor]).f32()
            } else {
                0.0
            };
            pitch_delta = (f64::from(pitch_delta) + f64::from(pitch_extra)) as f32;
        } else if weapon != 0 {
            let projectiles = e
                .call(
                    FUNCTION_00525B20,
                    &args![weapon, u32::from(has_mod), 0u32, player],
                )
                .u8();
            if projectiles > 1 {
                let heading_extra = if actor != 0 {
                    fn_00966a20(e, Ptr::new(actor))
                } else {
                    0.0
                };
                let spread = setting_float(e, SETTING_011CF9B0);
                heading_delta = (f64::from(spread) * f64::from(heading_extra)
                    + f64::from(heading_delta)) as f32;
                let pitch_extra = if actor != 0 {
                    e.call(FUNCTION_004E4600, &args![actor]).f32()
                } else {
                    0.0
                };
                let spread = setting_float(e, SETTING_011CF9B0);
                pitch_delta =
                    (f64::from(spread) * f64::from(pitch_extra) + f64::from(pitch_delta)) as f32;
            }
        }
    }
    if in_vats_target && e.mem.u8(vats_action + 4) == 0 {
        let degrees: f64 = e.global(DEGREES_TO_RADIANS);
        let jitter_limit = (f64::from(setting_float(e, SETTING_011CFD10)) * degrees) as f32;
        jitter_heading = clamp_symmetric(jitter_heading, jitter_limit);
        jitter_pitch = clamp_symmetric(jitter_pitch, jitter_limit);
        heading_delta = (f64::from(heading_delta) + f64::from(jitter_heading)) as f32;
        pitch_delta = (f64::from(pitch_delta) + f64::from(jitter_pitch)) as f32;
    }
    let sum = (f64::from(e.mem.f32(heading_out)) + f64::from(heading_delta)) as f32;
    let sum = wrap_angle(e, sum);
    e.mem.set_f32(heading_out, sum);
    let sum = (f64::from(e.mem.f32(pitch_out)) + f64::from(pitch_delta)) as f32;
    let sum = clamp_pitch(e, sum);
    e.mem.set_f32(pitch_out, sum);
}

/// `-009a7d50(actor, vector, 0.0, 0)` as the double the game holds in `ST0`
/// (the sign is applied by the caller).
fn aim_pitch(e: &mut Engine, actor: u32, vector: u32) -> f64 {
    let x = e.mem.u32(vector);
    let y = e.mem.u32(vector + 4);
    let z = e.mem.u32(vector + 8);
    e.call(FUNCTION_009A7D50, &args![actor, x, y, z, 0.0f32, 0u32])
        .f64()
}

/// The search for a clear shot to a VATS target actor of type 0x3e (the
/// block of [`fn_00965620`] that scales the actor's `+0xd0` float): steps
/// the projected distance from `009669c0(actor) * 1.25` ... until the
/// projectile line of sight (`009a6e90`) reaches the target, then stores
/// `found * (actor's +0xd0 float)`.
fn find_clear_shot(e: &mut Engine, f: Frame, actor: u32, vats_action: u32) {
    let vec_v = f.at(0xffff_ffdc);
    let vec_s = f.at(0xffff_ffe8);
    let player = e.global::<u32>(PLAYER_CHARACTER);
    let length = e.call(VECTOR_LENGTH, &args![vec_v]).f64();
    let scale = fn_00966a40(e);
    let scaled = (f64::from(scale) * length) as f32;
    let start = e.call(FUNCTION_004019B0, &args![scaled]).f32();
    let reach = fn_009669c0(e, Ptr::new(actor));
    let end = (f64::from(reach) * e.global::<f64>(SCALE_ONE_AND_A_QUARTER)) as f32;
    let mut found = 0.0f32;
    let mut pass = 1i32;
    while f64::from(found) == 0.0 && pass < 2 {
        let mut step = start;
        while f64::from(found) == 0.0 && end >= step {
            let lift: f64 = e.global(AIM_LIFT);
            let z = e.mem.f32(vec_v + 8);
            e.mem.set_f32(vec_v + 8, (f64::from(z) + lift) as f32);
            let multiplier = fn_00966a40(e);
            let zero = [
                e.mem.u32(ZERO_VECTOR),
                e.mem.u32(ZERO_VECTOR + 4),
                e.mem.u32(ZERO_VECTOR + 8),
            ];
            let sum = e
                .call(VECTOR_ADD, &args![vec_v, f.at(0xffff_fe4c), vec_s])
                .u32();
            let sum_words = [e.mem.u32(sum), e.mem.u32(sum + 4), e.mem.u32(sum + 8)];
            let origin = [e.mem.u32(vec_s), e.mem.u32(vec_s + 4), e.mem.u32(vec_s + 8)];
            let projected = f.at(0xffff_ff18);
            e.call(
                FUNCTION_009A8F00,
                &args![
                    projected,
                    origin[0],
                    origin[1],
                    origin[2],
                    sum_words[0],
                    sum_words[1],
                    sum_words[2],
                    zero[0],
                    zero[1],
                    zero[2],
                    step,
                    multiplier,
                    0u32
                ],
            );
            let hit = f.at(0xffff_ff14);
            e.mem.set_u32(hit, 0);
            let point = [
                e.mem.u32(projected),
                e.mem.u32(projected + 4),
                e.mem.u32(projected + 8),
            ];
            let origin = [e.mem.u32(vec_s), e.mem.u32(vec_s + 4), e.mem.u32(vec_s + 8)];
            let base = e.call(GET_BASE_FORM, &args![actor]).u32();
            let clear = e
                .call(
                    FUNCTION_009A6E90,
                    &args![
                        player, base, step, origin[0], origin[1], origin[2], point[0], point[1],
                        point[2], 0u32, hit, 0u32
                    ],
                )
                .bool();
            let target = e.mem.u32(vats_action + 0xc);
            if !clear || e.mem.u32(hit) == target {
                let reach = fn_009669c0(e, Ptr::new(actor));
                found = (f64::from(step) / f64::from(reach)) as f32;
            }
            let fraction: f64 = e.global(STEP_FRACTION);
            step = ((f64::from(end) - f64::from(start)) * fraction + f64::from(step)) as f32;
        }
        pass += 1;
    }
    if f64::from(found) != 0.0 {
        let distance = e.call(READ_FLOAT_AT_D0, &args![actor]).f64();
        let value = (distance * f64::from(found)) as f32;
        fn_00966a00(e, Ptr::new(actor), value);
    }
}

/// This part's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x0095d090,
            player_character_cast_eat_drink_items(Ptr<PlayerCharacter>)
        ),
        entry!(
            0x0095d0b0,
            player_character_add_eat_drink_item(Ptr<PlayerCharacter>, Ptr) -> bool
        ),
        entry!(0x0095d160, fn_0095d160(Ptr<PlayerCharacter>) -> i32),
        entry!(
            0x0095d3f0,
            fn_0095d3f0(Ptr<PlayerCharacter>, Ptr, u32, u8, u8) -> bool
        ),
        entry!(
            0x0095d450,
            fn_0095d450(Ptr<PlayerCharacter>, Ptr, u32, u8, u8) -> bool
        ),
        entry!(
            0x0095d850,
            fn_0095d850(Ptr<PlayerCharacter>, Ptr, u32, u8) -> bool
        ),
        entry!(0x0095ddc0, fn_0095ddc0(Ptr<PlayerCharacter>, u32) -> i32),
        entry!(0x0095de30, fn_0095de30(Ptr<PlayerCharacter>, f32)),
        entry!(0x0095e5f0, fn_0095e5f0(Ptr<PlayerCharacter>)),
        entry!(0x0095e670, fn_0095e670(Ptr<PlayerCharacter>, Ptr, u32)),
        entry!(0x0095e6f0, fn_0095e6f0(Ptr<PlayerCharacter>, u32)),
        entry!(0x0095e900, fn_0095e900()),
        entry!(0x0095ea30, fn_0095ea30(Ptr<PlayerCharacter>, Ptr)),
        entry!(
            0x0095eb10,
            player_character_remove_queued_enchantment(Ptr<PlayerCharacter>, Ptr)
        ),
        entry!(
            0x0095ec40,
            player_character_cast_queued_enchantments(Ptr<PlayerCharacter>)
        ),
        entry!(0x0095ed80, fn_0095ed80(Ptr<PlayerCharacter>)),
        entry!(
            0x0095edf0,
            player_character_setup_3rd_person_camera_caster(Ptr<PlayerCharacter>)
        ),
        entry!(0x0095ee60, fn_0095ee60() -> u32),
        entry!(0x0095ee70, fn_0095ee70() -> u32),
        entry!(
            0x0095ee80,
            player_character_preload_1st_person_anims_file_list(Ptr<PlayerCharacter>)
        ),
        entry!(0x0095f210, fn_0095f210(Ptr<PlayerCharacter>, u32, u32)),
        entry!(0x0095f530, fn_0095f530(Ptr<PlayerCharacter>, u8, u8)),
        entry!(
            0x0095f590,
            player_character_set_controls_disabled(Ptr<PlayerCharacter>, u8)
        ),
        entry!(
            0x0095f6a0,
            fn_0095f6a0(Ptr<PlayerCharacter>) -> Ptr<PlayerCharacter>
        ),
        entry!(0x0095f6c0, fn_0095f6c0(Ptr<PlayerCharacter>, f32)),
        entry!(
            0x0095f930,
            player_character_create_mouse_spring(Ptr<PlayerCharacter>, Ptr, u32, f32)
        ),
        entry!(0x009604f0, fn_009604f0(Ptr) -> Ptr),
        entry!(0x00960510, fn_00960510() -> u32),
        entry!(0x00960520, fn_00960520(Ptr<PlayerCharacter>, f32)),
        entry!(0x00961190, fn_00961190(Ptr, Ptr)),
        entry!(0x009611e0, fn_009611e0(Ptr) -> u32),
        entry!(0x00961200, fn_00961200(Ptr, Ptr)),
        entry!(0x00961230, fn_00961230(Ptr, Ptr)),
        entry!(0x00961260, fn_00961260(Ptr) -> u32),
        entry!(
            0x00961280,
            fn_00961280(Ptr<PlayerCharacter>) -> Ptr<PlayerCharacter>
        ),
        entry!(
            0x009613c0,
            player_character_update_drop_spring(Ptr<PlayerCharacter>, f32) -> bool
        ),
        entry!(
            0x009614b0,
            fn_009614b0(Ptr<PlayerCharacter>, Ptr, Ptr, u8, u8) -> bool
        ),
        entry!(0x00961c40, fn_00961c40(Ptr<PlayerCharacter>, u8, f32)),
        entry!(0x00961d50, player_character_death_menu()),
        entry!(0x00961d90, fn_00961d90(Ptr<PlayerCharacter>, u8)),
        entry!(0x00961de0, fn_00961de0(Ptr<PlayerCharacter>, f32)),
        entry!(0x00961e30, fn_00961e30(Ptr<PlayerCharacter>, u8) -> u8),
        entry!(0x00961f30, fn_00961f30(Ptr, u8)),
        entry!(0x00961f50, fn_00961f50(Ptr, u8)),
        entry!(0x00961f70, fn_00961f70(Ptr, u8)),
        entry!(
            0x00961f90,
            player_character_reset_magic_cast_sound(Ptr<PlayerCharacter>)
        ),
        entry!(0x00962030, fn_00962030(Ptr<PlayerCharacter>, u32)),
        entry!(0x00962190, fn_00962190(Ptr<PlayerCharacter>)),
        entry!(
            0x00962290,
            player_character_play_impact_based_hit_shader(Ptr<PlayerCharacter>, Ptr)
        ),
        entry!(
            0x00962350,
            player_character_havok_activate_dropped_reference(Ptr<PlayerCharacter>)
        ),
        entry!(0x00962450, fn_00962450(Ptr<PlayerCharacter>)),
        entry!(0x00962490, fn_00962490(Ptr<PlayerCharacter>, u32, u8)),
        entry!(0x009624d0, fn_009624d0(Ptr<PlayerCharacter>)),
        entry!(0x00962590, fn_00962590(Ptr<PlayerCharacter>)),
        entry!(0x00962620, fn_00962620(Ptr<PlayerCharacter>) -> u32),
        entry!(0x00962720, fn_00962720(Ptr<PlayerCharacter>, u32) -> bool),
        entry!(0x009627a0, fn_009627a0(Ptr<PlayerCharacter>)),
        entry!(0x009627f0, fn_009627f0(Ptr<PlayerCharacter>)),
        entry!(0x00962810, fn_00962810(Ptr<PlayerCharacter>, Ptr) -> u8),
        entry!(0x00962850, fn_00962850(Ptr<PlayerCharacter>, Ptr, u8)),
        entry!(0x00962880, fn_00962880(Ptr<PlayerCharacter>)),
        entry!(0x00962950, fn_00962950(Ptr<PlayerCharacter>, u32) -> bool),
        entry!(0x00962cb0, fn_00962cb0(Ptr, u8)),
        entry!(0x00962cd0, fn_00962cd0(Ptr, u32)),
        entry!(0x00962d00, fn_00962d00(Ptr<PlayerCharacter>)),
        entry!(
            0x00962de0,
            player_character_add_gun_wobble(Ptr<PlayerCharacter>)
        ),
        entry!(0x00963730, fn_00963730(Ptr<PlayerCharacter>) -> Ptr),
        entry!(0x00963790, fn_00963790(Ptr<PlayerCharacter>, u32, u8, u8)),
        entry!(0x00963900, fn_00963900(Ptr<PlayerCharacter>, u32, u8)),
        entry!(0x009639e0, fn_009639e0(Ptr<PlayerCharacter>, u32, u8) -> u8),
        entry!(0x00963a50, fn_00963a50(Ptr<PlayerCharacter>, Ptr, u8)),
        entry!(0x00963b00, fn_00963b00(Ptr)),
        entry!(0x00963b20, fn_00963b20(Ptr<PlayerCharacter>, Ptr, u8)),
        entry!(0x00963ba0, fn_00963ba0(Ptr<PlayerCharacter>, u8, u8) -> Ptr),
        entry!(0x00963bf0, fn_00963bf0(Ptr<PlayerCharacter>)),
        entry!(0x00963ce0, fn_00963ce0(Ptr<PlayerCharacter>, u32) -> bool),
        entry!(0x00963d60, fn_00963d60(Ptr<PlayerCharacter>, u32)),
        entry!(
            0x00963e00,
            player_character_remove_player_action(Ptr<PlayerCharacter>, u32, u32)
        ),
        entry!(0x00963eb0, fn_00963eb0(Ptr<PlayerCharacter>, u32, f32, u32)),
        entry!(0x00964060, fn_00964060(Ptr<PlayerCharacter>) -> u32),
        entry!(0x009640b0, fn_009640b0(Ptr<PlayerCharacter>) -> u32),
        entry!(
            0x00964100,
            player_character_get_player_action(Ptr<PlayerCharacter>, u32) -> Ptr
        ),
        entry!(0x00964190, fn_00964190(Ptr<PlayerCharacter>)),
        entry!(
            0x00964260,
            player_character_update_auto_aim_actor(Ptr<PlayerCharacter>)
        ),
        entry!(
            0x009650a0,
            player_character_update_image_space_ironsights(Ptr<PlayerCharacter>)
        ),
        entry!(0x00965240, fn_00965240(u8)),
        entry!(0x00965250, fn_00965250(Ptr, f32)),
        entry!(
            0x00965270,
            player_character_update_image_space_vats(Ptr<PlayerCharacter>)
        ),
        entry!(0x00965580, fn_00965580(Ptr, f32)),
        entry!(
            0x009655a0,
            player_character_stop_iron_sights_image_space(Ptr<PlayerCharacter>)
        ),
        entry!(0x009655e0, fn_009655e0(Ptr<PlayerCharacter>)),
        entry!(
            0x00965620,
            fn_00965620(Ptr<PlayerCharacter>, u32, u32, Ptr, Ptr, u32, f32, f32)
        ),
        entry!(0x00966980, fn_00966980(Ptr) -> f32),
        entry!(0x009669c0, fn_009669c0(Ptr) -> f32),
        entry!(0x00966a00, fn_00966a00(Ptr, f32)),
        entry!(0x00966a20, fn_00966a20(Ptr) -> f32),
        entry!(0x00966a40, fn_00966a40() -> f32),
        entry!(0x00966a50, fn_00966a50(Ptr<PlayerCharacter>) -> u32),
        entry!(
            0x00966a70,
            player_character_add_note(Ptr<PlayerCharacter>, u32, u8)
        ),
        entry!(0x00966b80, fn_00966b80(Ptr<PlayerCharacter>, u32)),
        entry!(0x00966bb0, fn_00966bb0(Ptr<PlayerCharacter>, u32) -> u32),
        entry!(0x00966c10, fn_00966c10(Ptr<PlayerCharacter>, u32) -> bool),
        entry!(
            0x00966c60,
            player_character_get_terminal_access(Ptr<PlayerCharacter>, u32) -> u32
        ),
        entry!(0x00966d70, fn_00966d70(Ptr)),
        entry!(0x00966e00, fn_00966e00(Ptr<PlayerCharacter>)),
        entry!(0x00966e50, fn_00966e50(Ptr<PlayerCharacter>)),
        entry!(0x00966ea0, fn_00966ea0(Ptr<PlayerCharacter>, u32, u8)),
        entry!(0x00966f20, fn_00966f20(Ptr<PlayerCharacter>, u32, u8, u8)),
        entry!(
            0x00967090,
            player_character_is_actor_perceived(Ptr<PlayerCharacter>, u32) -> Ptr
        ),
        entry!(0x009670f0, fn_009670f0(Ptr<PlayerCharacter>, u32) -> bool),
        entry!(
            0x00967160,
            player_character_is_player_detected_by_non_teammates(Ptr<PlayerCharacter>, u32) -> bool
        ),
        entry!(
            0x00967220,
            player_character_change_perceived_actor_hostile_status(Ptr<PlayerCharacter>, u32, u8)
        ),
        entry!(0x00967290, fn_00967290(Ptr<PlayerCharacter>)),
        entry!(
            0x00967350,
            player_character_remove_perceived_actor(Ptr<PlayerCharacter>, u32)
        ),
        entry!(
            0x009673d0,
            player_character_update_menu_mode_button(Ptr<PlayerCharacter>, f32)
        ),
        entry!(0x00967700, fn_00967700(Ptr) -> bool),
        entry!(
            0x00967730,
            player_character_check_for_closes_conversation(Ptr<PlayerCharacter>, u32, u32, u32)
        ),
        entry!(
            0x009678a0,
            player_character_is_standing_on_furniture(Ptr<PlayerCharacter>) -> bool
        ),
        entry!(0x00967950, fn_00967950(Ptr<PlayerCharacter>) -> Ptr),
        entry!(
            0x00967a00,
            player_character_get_bumped_ref(Ptr<PlayerCharacter>) -> Ptr
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::{Cell, RefCell};
    use std::collections::HashMap;
    use std::rc::Rc;

    type CallLog = Vec<(u32, Vec<u32>)>;

    /// Vtable targets the tests give their objects.
    const SLOT_A: u32 = 0x7000_0001;
    const SLOT_B: u32 = 0x7000_0002;
    const SLOT_C: u32 = 0x7000_0003;
    const SLOT_D: u32 = 0x7000_0004;
    const SLOT_E: u32 = 0x7000_0005;
    const SLOT_F: u32 = 0x7000_0006;

    fn int(value: u32) -> Ret {
        Ret {
            eax: value,
            ..Ret::default()
        }
    }

    fn float(value: f64) -> Ret {
        Ret {
            st0: value,
            ..Ret::default()
        }
    }

    /// An engine with the exe's data range mapped and a double answering 0
    /// for every address in `callees`.
    fn engine(callees: &[u32]) -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0050_0000);
        for &address in callees {
            e.register_double(address, |_, _| Ret::default());
        }
        e
    }

    /// A double answering `value` for `address`.
    fn answer(e: &mut Engine, address: u32, value: u32) {
        e.register_double(address, move |_, _| int(value));
    }

    /// A double answering a float in ST0.
    fn answer_float(e: &mut Engine, address: u32, value: f64) {
        e.register_double(address, move |_, _| float(value));
    }

    /// The arguments of the calls to `address` in `log`.
    fn calls_to(log: &[(u32, Vec<u32>)], address: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|(target, _)| *target == address)
            .map(|(_, args)| args.clone())
            .collect()
    }

    /// An object whose vtable has the given `(slot offset, target)` entries.
    fn object_with_slots(e: &mut Engine, slots: &[(u32, u32)]) -> u32 {
        let vtable = e.mem.alloc(0x800);
        for (offset, target) in slots {
            e.mem.set_u32(vtable + offset, *target);
        }
        let object = e.mem.alloc(0x400);
        e.mem.set_u32(object, vtable);
        object
    }

    /// A player whose vtable and process vtable have the given slots, set as
    /// the global player.
    fn player_with(
        e: &mut Engine,
        player_slots: &[(u32, u32)],
        process_slots: &[(u32, u32)],
    ) -> Ptr<PlayerCharacter> {
        let vtable = e.mem.alloc(0x800);
        for (offset, target) in player_slots {
            e.mem.set_u32(vtable + offset, *target);
        }
        let player = e.new_object::<PlayerCharacter>();
        e.mem.set_u32(player.addr(), vtable);
        let process = object_with_slots(e, process_slots);
        e.set(player, PlayerCharacter::pCurrentProcess, Ptr::new(process));
        e.set_global(PLAYER_CHARACTER, player.addr());
        player
    }

    /// Real `BSSimpleList` node accessors on `{item, next}` nodes.
    fn list_accessors(e: &mut Engine) {
        e.register_double(LIST_NODE_SLOT, |_, a| int(a[0]));
        e.register_double(LIST_NODE_NEXT, |e, a| int(e.mem.u32(a[0] + 4)));
        e.register_double(LIST_NODE_IS_EMPTY, |e, a| {
            int((e.mem.u32(a[0]) == 0 && e.mem.u32(a[0] + 4) == 0) as u32)
        });
    }

    /// A list of `{item, next}` nodes; returns the first node.
    fn list_of(e: &mut Engine, items: &[u32]) -> u32 {
        let mut next = 0;
        for &item in items.iter().rev() {
            let node = e.mem.alloc(8);
            e.mem.set_u32(node, item);
            e.mem.set_u32(node + 4, next);
            next = node;
        }
        next
    }

    /// Setting objects answering the float values in `values` (0.0 for any
    /// other) through the value pointer accessor.
    fn float_settings(e: &mut Engine, values: &[(u32, f32)]) {
        let table: HashMap<u32, f32> = values.iter().copied().collect();
        e.register_double(SETTING_FLOAT, move |e, a| {
            let cell = e.mem.alloc(4);
            e.mem
                .set_f32(cell, table.get(&a[0]).copied().unwrap_or(0.0));
            int(cell)
        });
    }

    /// A NUL-terminated string in fresh memory.
    fn text(e: &mut Engine, string: &str) -> u32 {
        let address = e.mem.alloc(string.len() as u32 + 1);
        e.mem.set_cstr(address, string.as_bytes());
        address
    }

    fn read_text(e: &Engine, address: u32) -> String {
        String::from_utf8(e.mem.cstr(address)).unwrap()
    }

    /// Runs `body` with the call log on and returns the log.
    fn logged(e: &mut Engine, body: impl FnOnce(&mut Engine)) -> CallLog {
        e.call_log = Some(Vec::new());
        body(e);
        e.call_log.take().unwrap()
    }

    #[test]
    fn cast_eat_drink_items_clears_the_list() {
        let mut e = engine(CALLEES_0095D090);
        let player = e.new_object::<PlayerCharacter>();
        e.set(
            player,
            PlayerCharacter::EatDrinkItems,
            Ptr::new(0x3000_0000),
        );
        let log = logged(&mut e, |e| {
            e.call(0x0095_d090, &args![player]);
        });
        assert_eq!(
            log,
            vec![
                (0x0095_d090, vec![player.addr()]),
                (LIST_CLEAR_ITEMS, vec![0x3000_0000]),
            ]
        );
    }

    #[test]
    fn add_eat_drink_item_queues_a_consumable_below_the_limit() {
        let mut e = engine(CALLEES_0095D0B0);
        list_accessors(&mut e);
        e.set_global(UPDATE_TARGET_VALUE, 0.01f32);
        let player = player_with(&mut e, &[], &[]);
        // The MagicTarget base at +0x94 has an empty list (virtual +8).
        let target = object_with_slots(&mut e, &[(8, SLOT_A)]);
        let vtable = e.mem.u32(target);
        e.mem.set_u32(player.addr() + MAGIC_TARGET_OFFSET, vtable);
        answer(&mut e, SLOT_A, 0);
        answer(&mut e, SLOT_B, 7);
        answer(&mut e, SLOT_C, 3);
        let consumable = object_with_slots(&mut e, &[(0x18, SLOT_B)]);
        let other = object_with_slots(&mut e, &[(0x18, SLOT_C)]);
        let queue = e.mem.alloc(8);
        e.set(player, PlayerCharacter::EatDrinkItems, Ptr::new(queue));

        let log = logged(&mut e, |e| {
            assert!(e.call(0x0095_d0b0, &args![player, consumable]).bool());
        });
        let appended = calls_to(&log, LIST_APPEND);
        assert_eq!(appended.len(), 1);
        assert_eq!(appended[0][0], queue);
        assert_eq!(
            calls_to(&log, ACTOR_CAST_ALCHEMY),
            vec![vec![player.addr(), consumable, 1]]
        );
        let target = player.addr() + MAGIC_TARGET_OFFSET;
        assert_eq!(
            calls_to(&log, MAGIC_TARGET_ADD_EFFECT_ITEM),
            vec![vec![target, consumable]]
        );
        assert_eq!(calls_to(&log, MAGIC_TARGET_REFRESH), vec![vec![target]]);
        assert_eq!(
            calls_to(&log, MAGIC_TARGET_UPDATE_TARGET),
            vec![vec![target, 0.01f32.to_bits()]]
        );

        // Any other kind of item is cast at once, never queued.
        let log = logged(&mut e, |e| {
            assert!(e.call(0x0095_d0b0, &args![player, other]).bool());
        });
        assert!(calls_to(&log, LIST_APPEND).is_empty());
        assert_eq!(
            calls_to(&log, ACTOR_CAST_ALCHEMY),
            vec![vec![player.addr(), other, 1]]
        );
    }

    #[test]
    fn eat_drink_count_adds_the_queued_items_and_the_distinct_ones() {
        let mut e = engine(CALLEES_0095D160);
        list_accessors(&mut e);
        answer(&mut e, SLOT_A, 7);
        let item = object_with_slots(&mut e, &[(0x18, SLOT_A)]);
        let queued = list_of(&mut e, &[item, item]);
        let player = player_with(&mut e, &[], &[]);
        e.set(player, PlayerCharacter::EatDrinkItems, Ptr::new(queued));

        // The MagicTarget base lists the same form three times through
        // wrapper records whose +8 is the form.
        let record = e.mem.alloc(0x20);
        e.mem.set_u32(record + 8, item);
        let active = list_of(&mut e, &[record, record, record]);
        let target = object_with_slots(&mut e, &[(8, SLOT_B)]);
        answer(&mut e, SLOT_B, active);
        let vtable = e.mem.u32(target);
        e.mem.set_u32(player.addr() + MAGIC_TARGET_OFFSET, vtable);
        e.register_double(READ_FIELD_8, |e, a| int(e.mem.u32(a[0] + 8)));
        // Not queued yet; the stack limit of the form is 2.
        answer(&mut e, LIST_CONTAINS, 0);
        answer(&mut e, ITEM_GET_STACK_LIMIT, 2);

        // Two queued; the first record opens a slot (+1), the second fills
        // that slot's count to the limit, the third needs a new slot (+1).
        let count = e.call(0x0095_d160, &args![player]).i32();
        assert_eq!(count, 4);

        // Without a list nothing is queued and the table is empty.
        e.set(player, PlayerCharacter::EatDrinkItems, Ptr::NULL);
        assert_eq!(e.call(0x0095_d160, &args![player]).i32(), 2);
    }

    #[test]
    fn eat_drink_count_places_its_alloca16_tables_as_the_exe_does() {
        let mut e = engine(CALLEES_0095D160);
        list_accessors(&mut e);
        answer(&mut e, SLOT_A, 7);
        let item = object_with_slots(&mut e, &[(0x18, SLOT_A)]);
        let queued = list_of(&mut e, &[item]);
        let player = player_with(&mut e, &[], &[]);
        e.set(player, PlayerCharacter::EatDrinkItems, Ptr::new(queued));
        let record = e.mem.alloc(0x20);
        e.mem.set_u32(record + 8, item);
        let active = list_of(&mut e, &[record, record, record]);
        let target = object_with_slots(&mut e, &[(8, SLOT_B)]);
        answer(&mut e, SLOT_B, active);
        let vtable = e.mem.u32(target);
        e.mem.set_u32(player.addr() + MAGIC_TARGET_OFFSET, vtable);
        e.register_double(READ_FIELD_8, |e, a| int(e.mem.u32(a[0] + 8)));
        answer(&mut e, LIST_CONTAINS, 0);
        answer(&mut e, ITEM_GET_STACK_LIMIT, 2);

        // Frame blocks whose top (EBP) sits at both 8-byte residues mod 16, so the
        // alignment pads differ: the call runs under a local of 8 or of 16 bytes, which moves
        // its frame on the thread's stack by 8.
        let mut residues = Vec::new();
        for under in [8, 16] {
            let (block, log) = e.with_stack(under, |e, _| {
                // The frame is the next local on the thread's stack; its bytes stay there after
                // the call returns.
                let block = e.next_stack_local(EAT_DRINK_FRAME_SIZE);
                let log = logged(e, |e| {
                    // One queued item, then a slot opened (+1) and filled to the limit; the
                    // third record opens a second slot (+1).
                    assert_eq!(e.call(0x0095_d160, &args![player]).i32(), 3);
                });
                (block, log)
            });
            let ebp = block + EAT_DRINK_FRAME_SIZE;
            residues.push(ebp & 0xf);
            // `_alloca16` from ESP = EBP-0x44, then from the first table: (esp - 0x190) & ~0xf.
            let items = (ebp - 0x44 - 0x190) & !0xf;
            let counts = (items - 0x190) & !0xf;
            assert_eq!(items & 0xf, 0);
            assert_eq!(counts & 0xf, 0);
            assert_eq!(e.mem.u32(items), item);
            assert_eq!(e.mem.u32(items + 4), item);
            assert_eq!(e.mem.u32(counts), 2);
            assert_eq!(e.mem.u32(counts + 4), 1);
            assert!(counts >= block);
            // The form asked about sits in the local at EBP-0x20.
            assert!(calls_to(&log, LIST_CONTAINS)
                .iter()
                .all(|c| c[1] == ebp - 0x20));
            assert!(!calls_to(&log, LIST_CONTAINS).is_empty());
        }
        assert_ne!(residues[0], residues[1]);
    }

    #[test]
    fn equip_dispatch_uses_the_item_check_or_the_virtual() {
        let mut e = engine(CALLEES_0095D3F0);
        let (player, _) = equip_world(&mut e, 0x5000_1000, 1, 0, 0);
        add_slot(&mut e, player.addr(), 0x3e8, SLOT_D);
        answer(&mut e, SLOT_D, 1);
        let item = 0x5000_0000;
        let args_of = |mode| args![player, item, mode, 3u32, 4u32];

        answer(&mut e, GET_BYTE_AT_384, 1);
        let log = logged(&mut e, |e| {
            e.call(0x0095_d3f0, &args_of(7));
        });
        assert_eq!(calls_to(&log, SLOT_D), Vec::<Vec<u32>>::new());
        assert_eq!(
            calls_to(&log, WEAPON_GET_CURRENT_AMMO),
            vec![vec![item, player.addr()]]
        );

        answer(&mut e, GET_BYTE_AT_384, 0);
        let mut result = false;
        let log = logged(&mut e, |e| {
            result = e.call(0x0095_d3f0, &args_of(5)).bool();
        });
        assert!(result);
        assert_eq!(
            calls_to(&log, SLOT_D),
            vec![vec![player.addr(), item, 5, 3]]
        );
        assert!(calls_to(&log, WEAPON_GET_CURRENT_AMMO).is_empty());

        // A null item always takes the virtual.
        let log = logged(&mut e, |e| {
            e.call(0x0095_d3f0, &args![player, 0u32, 1u32, 0u32, 0u32]);
        });
        assert_eq!(calls_to(&log, SLOT_D).len(), 1);
        assert!(calls_to(&log, GET_BYTE_AT_384).is_empty());
    }

    const CALLEES_0095D090: &[u32] = &[LIST_CLEAR_ITEMS];
    const CALLEES_0095D0B0: &[u32] = &[
        ACTOR_CAST_ALCHEMY,
        ITEM_GET_STACK_LIMIT,
        LIST_APPEND,
        LIST_CONTAINS,
        LIST_NODE_IS_EMPTY,
        LIST_NODE_NEXT,
        LIST_NODE_SLOT,
        MAGIC_TARGET_ADD_EFFECT_ITEM,
        MAGIC_TARGET_REFRESH,
        MAGIC_TARGET_UPDATE_TARGET,
        READ_FIELD_8,
    ];
    const CALLEES_0095D160: &[u32] = &[
        ITEM_GET_STACK_LIMIT,
        LIST_CONTAINS,
        LIST_NODE_IS_EMPTY,
        LIST_NODE_NEXT,
        LIST_NODE_SLOT,
        READ_FIELD_8,
    ];
    const CALLEES_0095D3F0: &[u32] = &[
        ACTOR_GET_ANIM_GROUP,
        ACTOR_QUEUE_EQUIP_OBJECT,
        ACTOR_RELEASE_ANIM_GROUP,
        ACTOR_SET_ANIM_ACTION,
        ANIMATION_GET_ACTION,
        ANIMATION_PLAY_GROUP,
        ANIM_GROUP_GET_TYPE,
        GET_BYTE_AT_384,
        GET_INVENTORY_CHANGES,
        HOT_KEY_TO_OBJECT,
        INVENTORY_GET_ITEM,
        INVENTORY_GET_OBJECT_COUNT,
        ITEM_CHANGE_GET_HOT_KEY,
        ITEM_CHANGE_HAS_MOD,
        ITEM_CHANGE_NEW,
        MIN_COUNT,
        OPERATOR_NEW,
        PLAYER_GET_ANIMATION,
        PROCESS_SET_EQUIPPED_FOLLOWUP,
        READ_FIELD_4,
        READ_FIELD_8,
        SCALAR_DELETE,
        WEAPON_GET_ATTACK_ANIM_BASE,
        WEAPON_GET_CLIP_ROUNDS,
        WEAPON_GET_CURRENT_AMMO,
    ];
    const CALLEES_0095D450: &[u32] = &[
        ACTOR_GET_ANIM_GROUP,
        ACTOR_QUEUE_EQUIP_OBJECT,
        ACTOR_RELEASE_ANIM_GROUP,
        ACTOR_SET_ANIM_ACTION,
        ANIMATION_GET_ACTION,
        ANIMATION_PLAY_GROUP,
        ANIM_GROUP_GET_TYPE,
        GET_INVENTORY_CHANGES,
        HOT_KEY_TO_OBJECT,
        INVENTORY_GET_ITEM,
        INVENTORY_GET_OBJECT_COUNT,
        ITEM_CHANGE_GET_HOT_KEY,
        ITEM_CHANGE_HAS_MOD,
        ITEM_CHANGE_NEW,
        MIN_COUNT,
        OPERATOR_NEW,
        PLAYER_GET_ANIMATION,
        PROCESS_SET_EQUIPPED_FOLLOWUP,
        READ_FIELD_4,
        READ_FIELD_8,
        SCALAR_DELETE,
        WEAPON_GET_ATTACK_ANIM_BASE,
        WEAPON_GET_CLIP_ROUNDS,
        WEAPON_GET_CURRENT_AMMO,
    ];
    const CALLEES_0095D850: &[u32] = &[
        ACTOR_GET_ANIM_GROUP,
        ACTOR_QUEUE_EQUIP_OBJECT,
        ACTOR_RELEASE_ANIM_GROUP,
        ACTOR_SET_ANIM_ACTION,
        ANIMATION_BLEND_OUT,
        ANIMATION_GET_ACTION,
        ANIMATION_PLAY_GROUP,
        ANIM_GROUP_GET_TYPE,
        GET_BYTE_AT_F4,
        GET_DWORD_AT_28,
        GET_INVENTORY_CHANGES,
        HOT_KEY_TO_OBJECT,
        INTERFACE_IS_IN_MENU_MODE,
        INVENTORY_GET_ITEM,
        INVENTORY_GET_OBJECT_COUNT,
        ITEM_CHANGE_GET_HOT_KEY,
        ITEM_CHANGE_HAS_MOD,
        ITEM_CHANGE_NEW,
        MIN_COUNT,
        OPERATOR_NEW,
        PLAYER_GET_ANIMATION,
        PROCESS_SET_EQUIPPED_FOLLOWUP,
        READ_FIELD_4,
        READ_FIELD_8,
        SCALAR_DELETE,
        SET_BYTE_AT_4,
        TEST_BIT_2_AT_244,
        TEST_BIT_2_BYTE_AT_100,
        TEST_DWORD_AT_8,
        WEAPON_GET_ATTACK_ANIM_BASE,
        WEAPON_GET_CLIP_ROUNDS,
        WEAPON_GET_CURRENT_AMMO,
    ];
    const CALLEES_0095DDC0: &[u32] = &[
        ACTOR_FUNCTION_008A89A0,
        PLAYER_IS_DEMIGOD_MODE,
        PLAYER_IS_GOD_MODE,
    ];
    const CALLEES_0095DE30: &[u32] = &[
        ACTOR_GET_ANIM_ACTION,
        ACTOR_GET_IRON_SIGHTS,
        ACTOR_SET_IRON_SIGHTS,
        ADD_8C_TO_ADDRESS,
        COPY_16_BYTES_TO_OFFSET_100,
        FLOAT_FUNCTION_00408840,
        FLOAT_FUNCTION_005DC330,
        FUNCTION_007F3BD0,
        FUNCTION_00950610,
        GET_FIELD_20_OR_DEFAULT,
        ITEM_CHANGE_HAS_MOD_EFFECT,
        LIST_NODE_SLOT,
        OPERATOR_NEW,
        PLAYER_IS_PIPBOY_ACTIVE,
        READ_DWORD_AT_AC,
        READ_FIELD_8,
        READ_FLOAT_AT_110,
        READ_FLOAT_AT_BC,
        READ_FLOAT_AT_C,
        GET_CAMERA_HOLDER,
        SETTING_BOOL_POINTER,
        SETTING_FLOAT,
        SETTING_FLOAT_VALUE,
        STATE_OBJECT_ADVANCE,
        STATE_OBJECT_CHECK,
        STATE_OBJECT_GET,
        STATE_OBJECT_NEW,
        STATE_OBJECT_SET,
        VECTOR_LENGTH,
        VECTOR_SUBTRACT,
    ];
    const CALLEES_0095E5F0: &[u32] = &[ADD_4_TO_ADDRESS, LIST_NODE_NEXT, LIST_NODE_SLOT];
    const CALLEES_0095E670: &[u32] = &[
        FUNCTION_008C1D20,
        FUNCTION_0094C950,
        GET_INVENTORY_ITEM_REF,
        READ_DWORD_AT_C,
        SCALAR_DELETE,
    ];
    const CALLEES_0095E6F0: &[u32] = &[
        FUNCTION_00703E80,
        ITEM_CHANGE_GET_POISON,
        PLAYER_GET_PROCESS,
        READ_DWORD_AT_15C,
        READ_FIELD_8,
        RT_DYNAMIC_CAST,
        SETTING_STRING,
        SPRINTF,
        STRING_OR_EMPTY_00408DA0,
    ];
    const CALLEES_0095E900: &[u32] = &[
        FUNCTION_005F5950,
        FUNCTION_00704AF0,
        GET_MESSAGE_MENU_RESULT,
        ITEM_CHANGE_FUNCTION_004BDD20,
        PLAYER_GET_PROCESS,
        READ_FIELD_8,
        RT_DYNAMIC_CAST,
    ];
    const CALLEES_0095EA30: &[u32] = &[
        FORM_GET_TYPE,
        MAGIC_TARGET_ADD_EFFECT_ITEM,
        READ_FIELD_4,
        RT_DYNAMIC_CAST,
    ];
    const CALLEES_0095EB10: &[u32] = &[
        FORM_GET_TYPE,
        FUNCTION_008248E0,
        LIST_DELETE,
        LIST_NODE_IS_EMPTY,
        LIST_REMOVE,
        MAGIC_TARGET_IS_SPELL_TARGET,
        READ_FIELD_4,
        RT_DYNAMIC_CAST,
    ];
    const CALLEES_0095EC40: &[u32] = &[
        LIST_CLEAR_ITEMS,
        LIST_DELETE,
        LIST_NODE_IS_EMPTY,
        LIST_NODE_NEXT,
        LIST_NODE_SLOT,
        READ_FIELD_4,
        RT_DYNAMIC_CAST,
    ];
    const CALLEES_0095ED80: &[u32] = &[FUNCTION_00822F70];
    const CALLEES_0095EDF0: &[u32] = &[
        CASTER_GET_WORLD,
        CASTER_SET_WORLD,
        OBJECT_FUNCTION_004543C0,
        READ_DWORD_AT_40,
    ];
    const CALLEES_0095EE80: &[u32] = &[
        ADD_ROOT_IDLE_ARRAY,
        GET_MODEL_PATH,
        SCOPE_GUARD_DELETE,
        SCOPE_GUARD_NEW,
        SETTING_STRING,
        STRING_COPY,
        STRING_FIND_LAST,
    ];
    const CALLEES_0095F210: &[u32] = &[
        ACTOR_BASE_GET_SEX,
        BUILD_FILE_LIST,
        COPY_FILENAME_LIST,
        FUNCTION_00446500,
        FUNCTION_00464F30,
        FUNCTION_008397D0,
        FUNCTION_0087F4C0,
        GET_MODEL_PATH,
        GET_ROOT_FILENAME_LIST,
        LIST_DELETE,
        QUEUE_MODEL,
        READ_DWORD_AT_20,
        SCOPE_GUARD_DELETE,
        SCOPE_GUARD_NEW,
        SETTING_STRING,
        STRING_COPY,
        STRING_FIND_LAST,
    ];
    const CALLEES_0095F530: &[u32] = &[
        ACTOR_FUNCTION_008846E0,
        FUNCTION_008A6840,
        HUD_SET_MENU_MODE,
        PLAYER_UPDATE_CAMERA,
        READ_DWORD_AT_0,
    ];
    const CALLEES_0095F590: &[u32] = &[
        ACTOR_FUNCTION_008846E0,
        FUNCTION_008A6840,
        HUD_SET_MENU_MODE,
        PLAYER_UPDATE_CAMERA,
        READ_DWORD_AT_0,
    ];
    const CALLEES_0095F6A0: &[u32] = &[
        ADD_44_TO_ADDRESS,
        FUNCTION_0082AEE0,
        HKP_ENTITY_ACTIVATE,
        OBJECT_FUNCTION_004543C0,
        OBJECT_FUNCTION_005533C0,
        PLAYER_REMOVE_PLAYER_ACTION,
        READ_DWORD_AT_0,
        READ_DWORD_AT_40,
        SCRIPT_SET_ACTION_FLAG,
        SMART_POINTER_ASSIGN,
    ];
    const CALLEES_0095F6C0: &[u32] = &[
        ACTOR_GET_EYE_VECTOR,
        ADD_44_TO_ADDRESS,
        ALLOCATE_00AA13E0,
        BODY_LOOKUP_004AE750,
        BODY_VALUE_004B5400,
        CONTROL_QUERY,
        EXTRA_LIST_REMOVE_EXTRA,
        FIND_FIRST_COLLISION_OBJECT,
        FIND_REFERENCE_FOR_3D,
        FUNCTION_00404010,
        FUNCTION_0040EBD0,
        FUNCTION_004213C0,
        FUNCTION_00439E90,
        FUNCTION_00439EF0,
        FUNCTION_0043B4A0,
        FUNCTION_0043B4D0,
        FUNCTION_0043B4F0,
        FUNCTION_00444D00,
        FUNCTION_00457990,
        FUNCTION_00458620,
        FUNCTION_004586D0,
        FUNCTION_004587D0,
        FUNCTION_0045BB20,
        FUNCTION_00461130,
        FUNCTION_004A39F0,
        FUNCTION_004A3A20,
        FUNCTION_004A3C20,
        FUNCTION_004A3DA0,
        FUNCTION_004A3EB0,
        FUNCTION_004A3F10,
        FUNCTION_004A3F70,
        FUNCTION_004A3FB0,
        FUNCTION_004B3AB0,
        FUNCTION_004B4500,
        FUNCTION_004B4AB0,
        FUNCTION_004B4D10,
        FUNCTION_004B4D50,
        FUNCTION_004B4DB0,
        FUNCTION_004B4E50,
        FUNCTION_004B4EA0,
        FUNCTION_004B4EC0,
        FUNCTION_004B4F00,
        FUNCTION_004B52F0,
        FUNCTION_004B5A20,
        FUNCTION_004B5A80,
        FUNCTION_00517670,
        FUNCTION_00553F70,
        FUNCTION_00553FC0,
        FUNCTION_00560D30,
        FUNCTION_00560D80,
        FUNCTION_00561690,
        FUNCTION_0056DED0,
        FUNCTION_0059CE80,
        FUNCTION_005DBF20,
        FUNCTION_00624070,
        FUNCTION_00624B70,
        FUNCTION_00627920,
        FUNCTION_0062B8D0,
        FUNCTION_0062C300,
        FUNCTION_00630B40,
        FUNCTION_0063C8A0,
        FUNCTION_00696C30,
        FUNCTION_006FA820,
        FUNCTION_00703350,
        FUNCTION_007033D0,
        FUNCTION_00703430,
        FUNCTION_00705A90,
        FUNCTION_00705AD0,
        FUNCTION_007F68A0,
        FUNCTION_00825C00,
        FUNCTION_0082AEB0,
        FUNCTION_0082AEE0,
        FUNCTION_00877720,
        FUNCTION_008C5D90,
        FUNCTION_008C71B0,
        FUNCTION_00931ED0,
        FUNCTION_00963EB0,
        FUNCTION_0096A350,
        FUNCTION_0096A370,
        FUNCTION_00C66FB0,
        FUNCTION_00C66FD0,
        FUNCTION_00C6E280,
        FUNCTION_00C6FD20,
        FUNCTION_00C8E940,
        FUNCTION_00C9C380,
        FUNCTION_00C9C390,
        GET_AV_OBJECT_FOR_COLLIDABLE,
        GET_CHAR_CONTROLLER,
        GET_FIELD_20_OR_DEFAULT,
        HIT_COLLECTOR_DELETE_004A3BC0,
        HIT_COLLECTOR_GET_004A46B0,
        HIT_COLLECTOR_NEW_004A3A70,
        HKP_ENTITY_ACTIVATE,
        HK_SET_TRANSFORMED_POS,
        IDENTITY_00460140,
        IDENTITY_006815C0,
        MOUSE_SPRING_ACTION_NEW,
        NI_POINT3_UNITIZE_GET_LENGTH,
        OBJECT_FUNCTION_004543C0,
        OBJECT_FUNCTION_005533C0,
        PARAMS_NEW_004B4F40,
        PLAYER_REMOVE_PLAYER_ACTION,
        READ_DWORD_AT_0,
        READ_DWORD_AT_20,
        READ_DWORD_AT_40,
        READ_DWORD_AT_C,
        READ_FIELD_4,
        READ_FIELD_8,
        SCRIPT_SET_ACTION_FLAG,
        SETTING_BOOL_POINTER,
        SETTING_FLOAT,
        SMART_POINTER_ASSIGN,
        TEST_BIT_2_AT_244,
    ];
    const CALLEES_0095F930: &[u32] = &[
        ACTOR_GET_EYE_VECTOR,
        ADD_44_TO_ADDRESS,
        ALLOCATE_00AA13E0,
        BODY_LOOKUP_004AE750,
        FIND_REFERENCE_FOR_3D,
        FUNCTION_00404010,
        FUNCTION_0040EBD0,
        FUNCTION_00439E90,
        FUNCTION_00439EF0,
        FUNCTION_0043B4D0,
        FUNCTION_0043B4F0,
        FUNCTION_00444D00,
        FUNCTION_00457990,
        FUNCTION_00458620,
        FUNCTION_004587D0,
        FUNCTION_0045BB20,
        FUNCTION_00461130,
        FUNCTION_004A39F0,
        FUNCTION_004A3A20,
        FUNCTION_004A3C20,
        FUNCTION_004A3DA0,
        FUNCTION_004A3F70,
        FUNCTION_004B3AB0,
        FUNCTION_004B4500,
        FUNCTION_004B4AB0,
        FUNCTION_004B4D10,
        FUNCTION_004B4DB0,
        FUNCTION_004B4E50,
        FUNCTION_004B4EC0,
        FUNCTION_004B4F00,
        FUNCTION_004B52F0,
        FUNCTION_004B5A20,
        FUNCTION_004B5A80,
        FUNCTION_00517670,
        FUNCTION_00553F70,
        FUNCTION_00553FC0,
        FUNCTION_0059CE80,
        FUNCTION_00624B70,
        FUNCTION_0062C300,
        FUNCTION_007033D0,
        FUNCTION_00703430,
        FUNCTION_007F68A0,
        FUNCTION_0082AEE0,
        FUNCTION_008C71B0,
        FUNCTION_00931ED0,
        FUNCTION_00963EB0,
        FUNCTION_0096A350,
        FUNCTION_0096A370,
        FUNCTION_00C66FD0,
        FUNCTION_00C6E280,
        FUNCTION_00C8E940,
        GET_CHAR_CONTROLLER,
        HKP_ENTITY_ACTIVATE,
        IDENTITY_006815C0,
        MOUSE_SPRING_ACTION_NEW,
        OBJECT_FUNCTION_004543C0,
        OBJECT_FUNCTION_005533C0,
        PARAMS_NEW_004B4F40,
        PLAYER_REMOVE_PLAYER_ACTION,
        READ_DWORD_AT_0,
        READ_DWORD_AT_40,
        READ_DWORD_AT_C,
        READ_FIELD_8,
        SCRIPT_SET_ACTION_FLAG,
        SETTING_BOOL_POINTER,
        SETTING_FLOAT,
        SMART_POINTER_ASSIGN,
        TEST_BIT_2_AT_244,
    ];
    const CALLEES_00961200: &[u32] = &[BODY_LOOKUP_004AE750, FUNCTION_004A3F10];
    const CALLEES_00961280: &[u32] = &[
        ADD_44_TO_ADDRESS,
        FUNCTION_0082AEE0,
        HKP_ENTITY_ACTIVATE,
        OBJECT_FUNCTION_004543C0,
        OBJECT_FUNCTION_005533C0,
        PLAYER_REMOVE_PLAYER_ACTION,
        READ_DWORD_AT_0,
        READ_DWORD_AT_40,
        SCRIPT_SET_ACTION_FLAG,
        SMART_POINTER_ASSIGN,
    ];
    const CALLEES_009613C0: &[u32] = &[
        ACTOR_GET_EYE_VECTOR,
        ADD_44_TO_ADDRESS,
        ALLOCATE_00AA13E0,
        BODY_LOOKUP_004AE750,
        CONTROL_QUERY,
        FIND_REFERENCE_FOR_3D,
        FUNCTION_00404010,
        FUNCTION_0040EBD0,
        FUNCTION_00439E90,
        FUNCTION_00439EF0,
        FUNCTION_0043B4A0,
        FUNCTION_0043B4D0,
        FUNCTION_0043B4F0,
        FUNCTION_00444D00,
        FUNCTION_00457990,
        FUNCTION_00458620,
        FUNCTION_004586D0,
        FUNCTION_004587D0,
        FUNCTION_0045BB20,
        FUNCTION_00461130,
        FUNCTION_004A39F0,
        FUNCTION_004A3A20,
        FUNCTION_004A3C20,
        FUNCTION_004A3DA0,
        FUNCTION_004A3EB0,
        FUNCTION_004A3F10,
        FUNCTION_004A3F70,
        FUNCTION_004A3FB0,
        FUNCTION_004B3AB0,
        FUNCTION_004B4500,
        FUNCTION_004B4AB0,
        FUNCTION_004B4D10,
        FUNCTION_004B4D50,
        FUNCTION_004B4DB0,
        FUNCTION_004B4E50,
        FUNCTION_004B4EA0,
        FUNCTION_004B4EC0,
        FUNCTION_004B4F00,
        FUNCTION_004B52F0,
        FUNCTION_004B5A20,
        FUNCTION_004B5A80,
        FUNCTION_00517670,
        FUNCTION_00553F70,
        FUNCTION_00553FC0,
        FUNCTION_00560D30,
        FUNCTION_00560D80,
        FUNCTION_00561690,
        FUNCTION_0056DED0,
        FUNCTION_0059CE80,
        FUNCTION_005DBF20,
        FUNCTION_00624070,
        FUNCTION_00624B70,
        FUNCTION_00627920,
        FUNCTION_0062B8D0,
        FUNCTION_0062C300,
        FUNCTION_00630B40,
        FUNCTION_0063C8A0,
        FUNCTION_00696C30,
        FUNCTION_007033D0,
        FUNCTION_00703430,
        FUNCTION_007F68A0,
        FUNCTION_00825C00,
        FUNCTION_0082AEB0,
        FUNCTION_0082AEE0,
        FUNCTION_00877720,
        FUNCTION_008C5D90,
        FUNCTION_008C71B0,
        FUNCTION_00931ED0,
        FUNCTION_00963EB0,
        FUNCTION_0096A350,
        FUNCTION_0096A370,
        FUNCTION_00C66FB0,
        FUNCTION_00C66FD0,
        FUNCTION_00C6E280,
        FUNCTION_00C6FD20,
        FUNCTION_00C8E940,
        FUNCTION_00C9C380,
        FUNCTION_00C9C390,
        GET_AV_OBJECT_FOR_COLLIDABLE,
        GET_CHAR_CONTROLLER,
        GET_FIELD_20_OR_DEFAULT,
        HIT_COLLECTOR_DELETE_004A3BC0,
        HIT_COLLECTOR_GET_004A46B0,
        HIT_COLLECTOR_NEW_004A3A70,
        HKP_ENTITY_ACTIVATE,
        HK_SET_TRANSFORMED_POS,
        IDENTITY_00460140,
        IDENTITY_006815C0,
        MOUSE_SPRING_ACTION_NEW,
        NI_POINT3_UNITIZE_GET_LENGTH,
        OBJECT_FUNCTION_004543C0,
        OBJECT_FUNCTION_005533C0,
        OBJECT_GET_SCALE,
        PARAMS_NEW_004B4F40,
        PLAYER_REMOVE_PLAYER_ACTION,
        READ_DWORD_AT_0,
        READ_DWORD_AT_40,
        READ_DWORD_AT_C,
        READ_FIELD_4,
        READ_FIELD_8,
        SCRIPT_SET_ACTION_FLAG,
        SETTING_BOOL_POINTER,
        SETTING_FLOAT,
        SMART_POINTER_ASSIGN,
        TEST_BIT_2_AT_244,
    ];
    const CALLEES_00961C40: &[u32] = &[
        ADD_44_TO_ADDRESS,
        FUNCTION_00422750,
        FUNCTION_007FA8D0,
        FUNCTION_00B68770,
        INTERFACE_GET_PIPBOY,
        PLAYER_GET_PROCESS,
        PLAYER_GET_ROOT_NODE,
    ];
    const CALLEES_00961D50: &[u32] = &[
        CHOOSE_MAIN_MENU,
        GET_MESSAGE_MENU_RESULT,
        LOAD_MOST_RECENT_SAVE_GAME,
    ];
    const CALLEES_00961D90: &[u32] = &[FUNCTION_00933890, PLAYER_STOP_VANITY_MODE];
    const CALLEES_00960520: &[u32] = &[
        ACTOR_GET_EYE_VECTOR,
        ADD_44_TO_ADDRESS,
        BODY_LOOKUP_004AE750,
        FIND_REFERENCE_FOR_3D,
        FUNCTION_00404010,
        FUNCTION_0040EBD0,
        FUNCTION_00439E90,
        FUNCTION_0043B4A0,
        FUNCTION_0043B4F0,
        FUNCTION_00458620,
        FUNCTION_004586D0,
        FUNCTION_004587D0,
        FUNCTION_0045BB20,
        FUNCTION_004A3A20,
        FUNCTION_004A3C20,
        FUNCTION_004A3DA0,
        FUNCTION_004A3EB0,
        FUNCTION_004A3F10,
        FUNCTION_004A3F70,
        FUNCTION_004A3FB0,
        FUNCTION_004B3AB0,
        FUNCTION_004B4D50,
        FUNCTION_004B4EA0,
        FUNCTION_004B5A20,
        FUNCTION_004B5A80,
        FUNCTION_00553F70,
        FUNCTION_00560D30,
        FUNCTION_00560D80,
        FUNCTION_00561690,
        FUNCTION_0056DED0,
        FUNCTION_0059CE80,
        FUNCTION_005DBF20,
        FUNCTION_00624070,
        FUNCTION_00624B70,
        FUNCTION_00627920,
        FUNCTION_0062B8D0,
        FUNCTION_0062C300,
        FUNCTION_00630B40,
        FUNCTION_0063C8A0,
        FUNCTION_00696C30,
        FUNCTION_007F68A0,
        FUNCTION_00825C00,
        FUNCTION_0082AEB0,
        FUNCTION_0082AEE0,
        FUNCTION_008C5D90,
        FUNCTION_00931ED0,
        FUNCTION_00C66FB0,
        FUNCTION_00C6E280,
        FUNCTION_00C6FD20,
        FUNCTION_00C9C380,
        FUNCTION_00C9C390,
        GET_AV_OBJECT_FOR_COLLIDABLE,
        GET_CHAR_CONTROLLER,
        GET_FIELD_20_OR_DEFAULT,
        HIT_COLLECTOR_DELETE_004A3BC0,
        HIT_COLLECTOR_GET_004A46B0,
        HIT_COLLECTOR_NEW_004A3A70,
        HKP_ENTITY_ACTIVATE,
        HK_SET_TRANSFORMED_POS,
        IDENTITY_00460140,
        IDENTITY_006815C0,
        NI_POINT3_UNITIZE_GET_LENGTH,
        OBJECT_FUNCTION_004543C0,
        OBJECT_FUNCTION_005533C0,
        PLAYER_REMOVE_PLAYER_ACTION,
        READ_DWORD_AT_0,
        READ_DWORD_AT_40,
        READ_DWORD_AT_C,
        READ_FIELD_4,
        READ_FIELD_8,
        SCRIPT_SET_ACTION_FLAG,
        SETTING_FLOAT,
        SMART_POINTER_ASSIGN,
    ];
    const CALLEES_009614B0: &[u32] = &[
        ACTOR_GET_EYE_VECTOR,
        ALLOCATE_00AA13E0,
        BODY_LOOKUP_004AE750,
        FUNCTION_0040EBD0,
        FUNCTION_00413F40,
        FUNCTION_00416870,
        FUNCTION_00436AA0,
        FUNCTION_00439180,
        FUNCTION_00439E90,
        FUNCTION_00457990,
        FUNCTION_004587D0,
        FUNCTION_0045BB20,
        FUNCTION_0045CEC0,
        FUNCTION_004A0C10,
        FUNCTION_004A0C90,
        FUNCTION_004A39F0,
        FUNCTION_004A3A20,
        FUNCTION_004A3C20,
        FUNCTION_004A3DA0,
        FUNCTION_004A3E00,
        FUNCTION_004A3F70,
        FUNCTION_004A3FB0,
        FUNCTION_004B4500,
        FUNCTION_004B4DB0,
        FUNCTION_004B5510,
        FUNCTION_0056E090,
        FUNCTION_0056E120,
        FUNCTION_0056E2D0,
        FUNCTION_0056EA90,
        FUNCTION_0059CE80,
        FUNCTION_00622290,
        FUNCTION_00623150,
        FUNCTION_0062D310,
        FUNCTION_0062D350,
        FUNCTION_0062D4B0,
        FUNCTION_007F68A0,
        FUNCTION_008C71B0,
        FUNCTION_00931ED0,
        FUNCTION_00C6E280,
        GET_BOUND_SIZE_0050EBF0,
        GET_CHAR_CONTROLLER,
        HIT_COLLECTOR_DELETE_004A3BC0,
        HIT_COLLECTOR_GET_004A46B0,
        HIT_COLLECTOR_NEW_004A3A70,
        IDENTITY_00460140,
        IDENTITY_006815C0,
        OBJECT_FUNCTION_004543C0,
        OBJECT_GET_SCALE,
        PHANTOM_DELETE_00C9EEC0,
        PHANTOM_REMOVE,
        READ_DWORD_AT_0,
        READ_DWORD_AT_40,
        READ_FIELD_4,
        SETTING_FLOAT,
        SMART_POINTER_ASSIGN,
        SMART_POINTER_ASSIGN_00633C90,
        SPHERE_SHAPE_NEW,
        WORLD_OBJECT_ADD,
    ];

    /// The doubles the equip functions share: an inventory with `count`
    /// items of the current object `held`, the process's equipped change
    /// (virtual +0x14c) holding `change_count` of `held`, and the process's
    /// equipped item (virtual +0x148) `before`. Returns (player, change).
    /// Adds a virtual slot to an object that has a vtable.
    fn add_slot(e: &mut Engine, object: u32, offset: u32, target: u32) {
        let vtable = e.mem.u32(object);
        e.mem.set_u32(vtable + offset, target);
    }

    fn equip_world(
        e: &mut Engine,
        held: u32,
        count: u32,
        change_count: u32,
        before: u32,
    ) -> (Ptr<PlayerCharacter>, u32) {
        let change = e.mem.alloc(0x10);
        e.mem.set_u32(change + 4, change_count);
        e.mem.set_u32(change + 8, held);
        let player = player_with(e, &[], &[(0x148, SLOT_A), (0x14c, SLOT_B), (0x168, SLOT_C)]);
        answer(e, SLOT_A, before);
        answer(e, SLOT_B, change);
        answer(e, SLOT_C, 0);
        e.register_double(READ_FIELD_8, |e, a| int(e.mem.u32(a[0] + 8)));
        e.register_double(READ_FIELD_4, |e, a| int(e.mem.u32(a[0] + 4)));
        answer(e, GET_INVENTORY_CHANGES, 0x4000_0000);
        answer(e, INVENTORY_GET_OBJECT_COUNT, count);
        answer(e, WEAPON_GET_CURRENT_AMMO, held);
        e.register_double(MIN_COUNT, |_, a| int(a[0].min(a[1])));
        answer(e, ITEM_CHANGE_GET_HOT_KEY, u32::MAX);
        answer(e, PLAYER_GET_ANIMATION, 0x9000_0000);
        (player, change)
    }

    #[test]
    fn equip_item_queues_or_plays_by_mode() {
        let mut e = engine(CALLEES_0095D450);
        let held = 0x5000_1000;
        let (player, _) = equip_world(&mut e, held, 3, 5, 0);
        let none = Ptr::<()>::NULL;

        // Mode 0 with no item: queue the equip with the change's count + 1,
        // or the inventory count when the last flag is set.
        let log = logged(&mut e, |e| {
            assert!(e
                .call(0x0095_d450, &args![player, none, 0u32, 0u32, 0u32])
                .bool());
        });
        assert_eq!(
            calls_to(&log, ACTOR_QUEUE_EQUIP_OBJECT),
            vec![vec![player.addr(), held, 6, 0, 1, 0, 0]]
        );
        let log = logged(&mut e, |e| {
            assert!(e
                .call(0x0095_d450, &args![player, none, 0u32, 0u32, 1u32])
                .bool());
        });
        assert_eq!(
            calls_to(&log, ACTOR_QUEUE_EQUIP_OBJECT),
            vec![vec![player.addr(), held, 3, 0, 1, 0, 0]]
        );

        // An unknown mode does nothing and answers false.
        assert!(!e
            .call(0x0095_d450, &args![player, none, 9u32, 0u32, 0u32])
            .bool());

        // With another item: the count is limited by its clip rounds, and
        // mode 1 releases the attack animation group the actor has.
        let item = e.mem.alloc(0x40);
        answer(&mut e, WEAPON_GET_CLIP_ROUNDS, 2);
        answer(&mut e, WEAPON_GET_ATTACK_ANIM_BASE, 0x20);
        answer(&mut e, ACTOR_GET_ANIM_GROUP, 0x1234);
        answer(&mut e, ANIM_GROUP_GET_TYPE, 9);
        let log = logged(&mut e, |e| {
            assert!(e
                .call(
                    0x0095_d450,
                    &args![player, Ptr::<()>::new(item), 1u32, 1u32, 0u32]
                )
                .bool());
        });
        assert_eq!(
            calls_to(&log, ACTOR_GET_ANIM_GROUP),
            vec![vec![player.addr(), 9, 0, 0, 0x9000_0000]]
        );
        assert_eq!(
            calls_to(&log, ACTOR_RELEASE_ANIM_GROUP),
            vec![vec![0x9000_0000, 0x1234]]
        );

        // Mode 2 starts the group and sets animation action 0x11.
        answer(&mut e, ANIMATION_GET_ACTION, 0x77);
        answer(&mut e, SLOT_D, 0);
        add_slot(&mut e, player.addr(), 0x4b0, SLOT_D);
        let log = logged(&mut e, |e| {
            assert!(e
                .call(
                    0x0095_d450,
                    &args![player, Ptr::<()>::new(item), 2u32, 1u32, 0u32]
                )
                .bool());
        });
        assert_eq!(
            calls_to(&log, ANIMATION_PLAY_GROUP),
            vec![vec![0x9000_0000, 0x1234, 1, u32::MAX, u32::MAX]]
        );
        assert_eq!(calls_to(&log, SLOT_D), vec![vec![player.addr(), 0x1234, 1]]);
        assert_eq!(
            calls_to(&log, ACTOR_SET_ANIM_ACTION),
            vec![vec![player.addr(), 0x11, 0x77]]
        );
    }

    #[test]
    fn equip_item_stops_when_the_same_count_is_already_equipped() {
        let mut e = engine(CALLEES_0095D450);
        let held = 0x5000_1000;
        let (player, _) = equip_world(&mut e, held, 3, 5, 0x7000_2000);
        answer(&mut e, ITEM_CHANGE_HAS_MOD, 0);
        answer(&mut e, WEAPON_GET_CLIP_ROUNDS, 5);
        let log = logged(&mut e, |e| {
            assert!(!e
                .call(
                    0x0095_d450,
                    &args![player, Ptr::<()>::NULL, 0u32, 0u32, 0u32]
                )
                .bool());
        });
        assert!(calls_to(&log, ACTOR_QUEUE_EQUIP_OBJECT).is_empty());
        // The mod check asks about index 2 of the equipped item.
        assert_eq!(
            calls_to(&log, ITEM_CHANGE_HAS_MOD),
            vec![vec![0x7000_2000, 2]]
        );
    }

    #[test]
    fn equip_item_replaces_the_object_by_its_hot_key() {
        let mut e = engine(CALLEES_0095D450);
        let held = 0x5000_1000;
        let replacement = 0x5000_2000;
        let (player, _) = equip_world(&mut e, held, 3, 5, 0);
        let item = e.mem.alloc(0x40);
        answer(&mut e, INVENTORY_GET_ITEM, 0x6100_0000);
        answer(&mut e, ITEM_CHANGE_GET_HOT_KEY, 4);
        answer(&mut e, HOT_KEY_TO_OBJECT, replacement);
        answer(&mut e, WEAPON_GET_CLIP_ROUNDS, 9);
        let log = logged(&mut e, |e| {
            assert!(e
                .call(
                    0x0095_d450,
                    &args![player, Ptr::<()>::new(item), 0u32, 0u32, 0u32]
                )
                .bool());
        });
        // The temporary item change is deleted, the replacement is equipped.
        assert_eq!(calls_to(&log, SCALAR_DELETE), vec![vec![0x6100_0000, 1]]);
        assert_eq!(
            calls_to(&log, HOT_KEY_TO_OBJECT),
            vec![vec![player.addr(), 4]]
        );
        assert_eq!(
            calls_to(&log, ACTOR_QUEUE_EQUIP_OBJECT)[0][2],
            6,
            "the equipped change differs from the replacement, so a new one is installed"
        );
        assert_eq!(calls_to(&log, ACTOR_QUEUE_EQUIP_OBJECT)[0][1], replacement);
    }

    #[test]
    fn equip_with_a_slot_for_the_item_only_handles_kinds_ten_to_thirteen() {
        let mut e = engine(CALLEES_0095D850);
        let (player, _) = equip_world(&mut e, 0, 2, 0, 0);
        answer(&mut e, WEAPON_GET_CURRENT_AMMO, 0);
        let item = Ptr::<()>::new(e.mem.alloc(0x40));
        answer(&mut e, GET_BYTE_AT_F4, 7);
        assert!(!e.call(0x0095_d850, &args![player, item, 0u32, 0u32]).bool());

        // Kind 0xb: the item itself becomes the current object.
        answer(&mut e, GET_BYTE_AT_F4, 0xb);
        answer(&mut e, GET_DWORD_AT_28, 1);
        add_slot(&mut e, player.addr(), 0x340, SLOT_E);
        answer(&mut e, SLOT_E, 0);
        let log = logged(&mut e, |e| {
            assert!(e.call(0x0095_d850, &args![player, item, 0u32, 0u32]).bool());
        });
        // No queued equip while nothing says to queue; the state byte goes
        // to 0x29 and 0x28 around it and the player's virtual +0x340 follows.
        assert!(calls_to(&log, ACTOR_QUEUE_EQUIP_OBJECT).is_empty());
        assert_eq!(
            calls_to(&log, SET_BYTE_AT_4),
            vec![vec![item.addr(), 0x29], vec![item.addr(), 0x28]]
        );

        // In menu mode the equip is queued.
        answer(&mut e, INTERFACE_IS_IN_MENU_MODE, 1);
        let log = logged(&mut e, |e| {
            assert!(e.call(0x0095_d850, &args![player, item, 0u32, 0u32]).bool());
        });
        assert_eq!(
            calls_to(&log, ACTOR_QUEUE_EQUIP_OBJECT),
            vec![vec![player.addr(), item.addr(), 2, 0, 1, 0, 0]]
        );
    }

    #[test]
    fn equip_with_a_slot_for_the_item_runs_the_attack_animation_for_mode_one() {
        let mut e = engine(CALLEES_0095D850);
        let held = 0x5000_1000;
        let (player, _) = equip_world(&mut e, held, 2, 0, 0);
        add_slot(&mut e, player.addr(), 0x4b0, SLOT_D);
        add_slot(&mut e, player.addr(), 0x340, SLOT_E);
        answer(&mut e, SLOT_D, 0);
        answer(&mut e, SLOT_E, 0);
        answer(&mut e, WEAPON_GET_CURRENT_AMMO, held);
        answer(&mut e, WEAPON_GET_ATTACK_ANIM_BASE, 0x30);
        answer(&mut e, ACTOR_GET_ANIM_GROUP, 0x1234);
        answer(&mut e, ANIM_GROUP_GET_TYPE, 0x30);
        answer(&mut e, TEST_BIT_2_BYTE_AT_100, 1);
        answer(&mut e, ANIMATION_GET_ACTION, 0x55);
        let log = logged(&mut e, |e| {
            assert!(e
                .call(
                    0x0095_d850,
                    &args![player, Ptr::<()>::new(held), 1u32, 0u32]
                )
                .bool());
        });
        assert_eq!(
            calls_to(&log, ANIMATION_BLEND_OUT),
            vec![vec![0x9000_0000, 5, 0], vec![0x9000_0000, 6, 0]]
        );
        assert_eq!(
            calls_to(&log, ANIMATION_PLAY_GROUP),
            vec![vec![0x9000_0000, 0x1234, 1, u32::MAX, u32::MAX]]
        );
        assert_eq!(
            calls_to(&log, ACTOR_SET_ANIM_ACTION),
            vec![vec![player.addr(), 9, 0x55]]
        );
        // The player's virtual +0x340 is called with 9 when TEST_DWORD_AT_8
        // answers 0.
        assert_eq!(calls_to(&log, SLOT_E), vec![vec![player.addr(), 9]]);
        // A weapon that is not automatic only releases the group.
        answer(&mut e, TEST_BIT_2_BYTE_AT_100, 0);
        let log = logged(&mut e, |e| {
            e.call(
                0x0095_d850,
                &args![player, Ptr::<()>::new(held), 1u32, 0u32],
            );
        });
        assert_eq!(
            calls_to(&log, ACTOR_RELEASE_ANIM_GROUP),
            vec![vec![0x9000_0000, 0x1234]]
        );
    }

    #[test]
    fn rating_is_zero_without_an_item_change_and_one_hundred_in_god_mode() {
        let mut e = engine(CALLEES_0095DDC0);
        let player = player_with(&mut e, &[], &[(0x14c, SLOT_B)]);
        answer(&mut e, SLOT_B, 0);
        answer(&mut e, ACTOR_FUNCTION_008A89A0, 77);
        assert_eq!(e.call(0x0095_ddc0, &args![player, 3u32]).i32(), 0);

        answer(&mut e, SLOT_B, 0x6000_0000);
        assert_eq!(e.call(0x0095_ddc0, &args![player, 3u32]).i32(), 77);
        let log = logged(&mut e, |e| {
            e.call(0x0095_ddc0, &args![player, 3u32]);
        });
        assert_eq!(
            calls_to(&log, ACTOR_FUNCTION_008A89A0),
            vec![vec![player.addr(), 3]]
        );

        answer(&mut e, PLAYER_IS_GOD_MODE, 1);
        assert_eq!(e.call(0x0095_ddc0, &args![player, 3u32]).i32(), 100);
        answer(&mut e, PLAYER_IS_DEMIGOD_MODE, 1);
        assert_eq!(e.call(0x0095_ddc0, &args![player, 3u32]).i32(), 77);
        e.mem.set_u32(player.addr() + 0x68, 0);
        assert_eq!(e.call(0x0095_ddc0, &args![player, 3u32]).i32(), 0);
    }

    /// A `__RTDynamicCast` double: `(object, 0, source, target, 0)` answers the
    /// cast registered for the object, or 0.
    fn dynamic_casts(e: &mut Engine, casts: &[(u32, u32)]) {
        let table: HashMap<u32, u32> = casts.iter().copied().collect();
        e.register_double(RT_DYNAMIC_CAST, move |_, a| {
            int(table.get(&a[0]).copied().unwrap_or(0))
        });
    }

    fn fov_world(e: &mut Engine) -> Ptr<PlayerCharacter> {
        let player = player_with(e, &[], &[(0x148, SLOT_A), (0x454, SLOT_B)]);
        answer(e, SLOT_A, 0);
        answer(e, SLOT_B, 0);
        let option = e.mem.alloc(4);
        e.mem.set_u8(option, 1);
        answer(e, SETTING_BOOL_POINTER, option);
        e.mem.set_f64(DOUBLE_30, 30.0);
        e.mem.set_f64(DOUBLE_5, 5.0);
        e.mem.set_f64(DOUBLE_ONE, 1.0);
        float_settings(
            e,
            &[
                (SETTING_0120315C, 75.0),
                (SETTING_01203168, 80.0),
                (SETTING_01203174, 70.0),
                (SETTING_011CE870, 15.0),
            ],
        );
        player
    }

    #[test]
    fn field_of_view_update_is_off_unless_the_setting_is_on() {
        let mut e = engine(CALLEES_0095DE30);
        let player = fov_world(&mut e);
        let option = e.mem.alloc(4);
        answer(&mut e, SETTING_BOOL_POINTER, option);
        let log = logged(&mut e, |e| {
            e.call(0x0095_de30, &args![player, 1.0f32]);
        });
        assert_eq!(log.len(), 2);
        assert_eq!(log[1], (SETTING_BOOL_POINTER, vec![SETTING_011E097C]));
    }

    #[test]
    fn field_of_view_moves_toward_the_targets_and_flags_arrival() {
        let mut e = engine(CALLEES_0095DE30);
        let player = fov_world(&mut e);
        let mode = Rc::new(std::cell::Cell::new(1u32));
        let seen = mode.clone();
        e.register_double(READ_FIELD_8, move |_, _| int(seen.get()));
        e.set(player, PlayerCharacter::fWorldFOV, 70.0);
        e.set(player, PlayerCharacter::f1stPersonFOV, 60.0);
        // step = delta * 30 / 15 = 2 per call.
        e.call(0x0095_de30, &args![player, 1.0f32]);
        assert_eq!(e.get(player, PlayerCharacter::fWorldFOV), 72.0);
        assert_eq!(e.get(player, PlayerCharacter::f1stPersonFOV), 62.0);
        assert_eq!(e.mem.u8(FOV_MOVING_FLAG), 0);

        // The step that would pass the target stops on it; the first-person
        // field arriving sets the flag.
        e.set(player, PlayerCharacter::fWorldFOV, 74.5);
        e.set(player, PlayerCharacter::f1stPersonFOV, 79.0);
        e.call(0x0095_de30, &args![player, 1.0f32]);
        assert_eq!(e.get(player, PlayerCharacter::fWorldFOV), 75.0);
        assert_eq!(e.get(player, PlayerCharacter::f1stPersonFOV), 80.0);
        assert_eq!(e.mem.u8(FOV_MOVING_FLAG), 1);

        // Above the target the fields come down by the step.
        e.set(player, PlayerCharacter::fWorldFOV, 90.0);
        e.set(player, PlayerCharacter::f1stPersonFOV, 100.0);
        e.call(0x0095_de30, &args![player, 1.0f32]);
        assert_eq!(e.get(player, PlayerCharacter::fWorldFOV), 88.0);
        assert_eq!(e.get(player, PlayerCharacter::f1stPersonFOV), 98.0);
        assert_eq!(e.mem.u8(FOV_MOVING_FLAG), 0);

        // Camera state 4 jumps to the targets at once.
        mode.set(4);
        e.call(0x0095_de30, &args![player, 1.0f32]);
        assert_eq!(e.get(player, PlayerCharacter::fWorldFOV), 75.0);
        assert_eq!(e.get(player, PlayerCharacter::f1stPersonFOV), 80.0);
        assert_eq!(e.mem.u8(FOV_MOVING_FLAG), 1);
    }

    #[test]
    fn field_of_view_uses_the_zoom_when_an_iron_sight_is_up() {
        let mut e = engine(CALLEES_0095DE30);
        let player = fov_world(&mut e);
        float_settings(
            &mut e,
            &[
                (SETTING_0120315C, 75.0),
                (SETTING_01203168, 80.0),
                (SETTING_01203174, 70.0),
                (SETTING_011CE870, 15.0),
                (SETTING_011E0970, 20.0),
            ],
        );
        e.register_double(READ_FIELD_8, |_, _| int(4));
        answer(&mut e, ACTOR_GET_IRON_SIGHTS, 1);
        answer(&mut e, ACTOR_GET_ANIM_ACTION, 3);
        e.call(0x0095_de30, &args![player, 1.0f32]);
        // Camera state 4 takes the targets at once: world 20, first person
        // 20 / 75 * 80.
        assert_eq!(e.get(player, PlayerCharacter::fWorldFOV), 20.0);
        assert_eq!(
            e.get(player, PlayerCharacter::f1stPersonFOV),
            (20.0f64 / 75.0 * 80.0) as f32
        );
    }

    #[test]
    fn field_of_view_resets_while_the_pipboy_is_active() {
        let mut e = engine(CALLEES_0095DE30);
        let player = fov_world(&mut e);
        answer(&mut e, PLAYER_IS_PIPBOY_ACTIVE, 1);
        answer(&mut e, ACTOR_GET_IRON_SIGHTS, 1);
        let log = logged(&mut e, |e| {
            e.call(0x0095_de30, &args![player, 1.0f32]);
        });
        assert_eq!(
            calls_to(&log, ACTOR_SET_IRON_SIGHTS),
            vec![vec![player.addr(), 0, 0, 0]]
        );
        assert_eq!(e.get(player, PlayerCharacter::fWorldFOV), 75.0);
        assert_eq!(e.get(player, PlayerCharacter::f1stPersonFOV), 70.0);
    }

    #[test]
    fn field_of_view_keeps_the_default_while_moving_flagged() {
        let mut e = engine(CALLEES_0095DE30);
        let player = fov_world(&mut e);
        e.mem.set_u8(FLAG_011E07B8, 1);
        e.mem.set_u8(FOV_MOVING_FLAG, 1);
        answer_float(&mut e, READ_FLOAT_AT_BC, 75.0);
        let log = logged(&mut e, |e| {
            e.call(0x0095_de30, &args![player, 1.0f32]);
        });
        assert!(calls_to(&log, FUNCTION_00950610).is_empty());
        answer_float(&mut e, READ_FLOAT_AT_BC, 60.0);
        let log = logged(&mut e, |e| {
            e.call(0x0095_de30, &args![player, 1.0f32]);
        });
        assert_eq!(
            calls_to(&log, FUNCTION_00950610),
            vec![vec![player.addr(), 75.0f32.to_bits()]]
        );
    }

    #[test]
    fn field_of_view_drives_the_two_state_objects_in_the_third_person_states() {
        let mut e = engine(CALLEES_0095DE30);
        let player = fov_world(&mut e);
        let camera_object = object_with_slots(&mut e, &[(0x1d0, SLOT_C)]);
        answer(&mut e, SLOT_C, 0x8000_0000);
        e.set_global(POINTER_011F21CC, camera_object);
        e.register_double(READ_FIELD_8, |_, _| int(2));
        e.register_double(OPERATOR_NEW, |e, a| int(e.mem.alloc(a[0])));
        e.register_double(STATE_OBJECT_NEW, |_, a| int(a[0]));
        e.register_double(GET_FIELD_20_OR_DEFAULT, |e, _| int(e.mem.alloc(0x20)));
        e.register_double(READ_FLOAT_AT_C, |_, a| {
            float(if a[0] == OBJECT_011F6394 { 0.5 } else { 2.0 })
        });
        answer(&mut e, ADD_8C_TO_ADDRESS, 0x8100_0000);
        answer_float(&mut e, VECTOR_LENGTH, 4.0);
        answer_float(&mut e, SETTING_FLOAT_VALUE, 3.0);
        answer_float(&mut e, FLOAT_FUNCTION_005DC330, 0.5);
        float_settings(
            &mut e,
            &[
                (SETTING_0120315C, 75.0),
                (SETTING_01203168, 80.0),
                (SETTING_01203174, 70.0),
                (SETTING_011CE870, 15.0),
                (SETTING_011CE5A0, 10.0),
                (SETTING_011CE344, 2.0),
                (SETTING_011CFE2C, 8.0),
                (SETTING_011CEA44, 1.0),
                (SETTING_01203150, 2.5),
                (SETTING_011D4A2C, 4.0),
            ],
        );
        e.mem.set_f64(DOUBLE_ONE, 1.0);
        e.register_double(STATE_OBJECT_GET, |_, a| {
            float(if a[0] == 0 { 0.0 } else { 71.0 })
        });
        let firsts = Rc::new(RefCell::new(Vec::new()));
        let seen = firsts.clone();
        e.register_double(STATE_OBJECT_GET, move |e, a| {
            seen.borrow_mut().push(a[0]);
            let first = e.mem.u32(FIRST_STATE_OBJECT_SLOT);
            float(if a[0] == first { 71.0 } else { 61.0 })
        });
        answer(&mut e, STATE_OBJECT_CHECK, 1);
        e.set(player, PlayerCharacter::fWorldFOV, 70.0);
        e.set(player, PlayerCharacter::f1stPersonFOV, 60.0);

        let log = logged(&mut e, |e| {
            e.call(0x0095_de30, &args![player, 1.0f32]);
        });
        let first = e.mem.u32(FIRST_STATE_OBJECT_SLOT);
        let second = e.mem.u32(SECOND_STATE_OBJECT_SLOT);
        assert!(first != 0 && second != 0 && first != second);
        assert_eq!(e.mem.u32(STATE_OBJECT_GUARD) & 3, 3);
        // Targets: wave 0.5 * (8 * 0.5 + 2) = 3 (world), limited to 2.5 for
        // the first-person field; the rate for state 2 is the setting.
        assert_eq!(
            calls_to(&log, STATE_OBJECT_SET),
            vec![
                vec![first, 70.0f32.to_bits(), 3.0f32.to_bits(), 4.0f32.to_bits()],
                vec![
                    second,
                    60.0f32.to_bits(),
                    2.5f32.to_bits(),
                    4.0f32.to_bits()
                ],
            ]
        );
        assert_eq!(
            calls_to(&log, STATE_OBJECT_ADVANCE),
            vec![
                vec![first, 0.5f32.to_bits()],
                vec![second, 0.5f32.to_bits()]
            ]
        );
        assert_eq!(e.get(player, PlayerCharacter::fWorldFOV), 71.0);
        assert_eq!(e.get(player, PlayerCharacter::f1stPersonFOV), 61.0);
        assert_eq!(calls_to(&log, FUNCTION_007F3BD0), vec![vec![1]]);
        assert_eq!(e.mem.u8(FOV_MOVING_FLAG), 1);
        assert_eq!(e.mem.u32(LAST_OBJECT_011E0D4C), camera_object);

        // The same camera object again does not restart the springs.
        let log = logged(&mut e, |e| {
            e.call(0x0095_de30, &args![player, 1.0f32]);
        });
        assert!(calls_to(&log, STATE_OBJECT_SET).is_empty());
        assert!(calls_to(&log, OPERATOR_NEW).is_empty());
    }

    #[test]
    fn list_of_pending_items_is_walked_until_an_empty_one() {
        let mut e = engine(CALLEES_0095E5F0);
        list_accessors(&mut e);
        e.register_double(ADD_4_TO_ADDRESS, |_, a| int(a[0] + 4));
        let player = player_with(&mut e, &[(0x37c, SLOT_A), (0x3e4, SLOT_B)], &[]);
        let owner = e.mem.alloc(0x100);
        answer(&mut e, SLOT_A, owner);
        answer(&mut e, SLOT_B, 0);
        let third = e.mem.alloc(8);
        let second = e.mem.alloc(8);
        e.mem.set_u32(second, 0x5000_0002);
        e.mem.set_u32(second + 4, third);
        e.mem.set_u32(owner + 0x30, 0x5000_0001);
        e.mem.set_u32(owner + 0x34, second);
        let log = logged(&mut e, |e| {
            e.call(0x0095_e5f0, &args![player]);
        });
        assert_eq!(
            calls_to(&log, SLOT_B),
            vec![
                vec![player.addr(), 0x5000_0001],
                vec![player.addr(), 0x5000_0002]
            ]
        );
        // Without the owner nothing happens.
        answer(&mut e, SLOT_A, 0);
        let log = logged(&mut e, |e| {
            e.call(0x0095_e5f0, &args![player]);
        });
        assert!(calls_to(&log, SLOT_B).is_empty());
    }

    #[test]
    fn item_lookup_hands_the_found_reference_to_the_helper() {
        let mut e = engine(CALLEES_0095E670);
        let player = e.new_object::<PlayerCharacter>();
        let form = e.mem.alloc(0x20);
        answer(&mut e, READ_DWORD_AT_C, 0x77);
        answer(&mut e, GET_INVENTORY_ITEM_REF, 0x6000_0000);
        let log = logged(&mut e, |e| {
            e.call(0x0095_e670, &args![player, form, 5u32]);
        });
        assert_eq!(
            log[1..].to_vec(),
            vec![
                (FUNCTION_008C1D20, vec![player.addr(), form, 5]),
                (READ_DWORD_AT_C, vec![form]),
                (GET_INVENTORY_ITEM_REF, vec![player.addr(), form, 0x77]),
                (SCALAR_DELETE, vec![0x6000_0000, 1]),
                (FUNCTION_0094C950, vec![player.addr(), form]),
            ]
        );
        answer(&mut e, GET_INVENTORY_ITEM_REF, 0);
        let log = logged(&mut e, |e| {
            e.call(0x0095_e670, &args![player, form, 5u32]);
        });
        assert!(calls_to(&log, SCALAR_DELETE).is_empty());
        assert_eq!(
            calls_to(&log, FUNCTION_0094C950),
            vec![vec![player.addr(), 0]]
        );
    }

    type Messages = Rc<RefCell<Vec<(String, Vec<u32>)>>>;

    /// Every setting string is the text `setting <address>`; the message box
    /// helper records its text and argument words.
    fn message_world(e: &mut Engine) -> Messages {
        e.register_double(SETTING_STRING, |e, a| {
            let string = format!("setting {:08x}", a[0]);
            int(text(e, &string))
        });
        let messages: Messages = Rc::new(RefCell::new(Vec::new()));
        let seen = messages.clone();
        e.register_double(FUNCTION_00703E80, move |e, a| {
            let shown = read_text(e, a[0]);
            seen.borrow_mut().push((shown, a.to_vec()));
            Ret::default()
        });
        messages
    }

    /// A player whose process holds `held` (virtual +0x148), a change whose
    /// object (+8) is `form`.
    fn holder_world(
        e: &mut Engine,
        kind: u32,
        poison: u32,
    ) -> (Ptr<PlayerCharacter>, u32, u32, u32) {
        let player = player_with(e, &[(0x17c, SLOT_E)], &[(0x148, SLOT_A)]);
        let form = e.mem.alloc(0x20);
        let held = e.mem.alloc(0x20);
        e.mem.set_u32(held + 8, form);
        let weapon = e.mem.alloc(0x40);
        answer(e, SLOT_A, held);
        answer(e, SLOT_E, 0);
        e.register_double(PLAYER_GET_PROCESS, |e, a| int(e.mem.u32(a[0] + 0x68)));
        e.register_double(READ_FIELD_8, |e, a| int(e.mem.u32(a[0] + 8)));
        dynamic_casts(e, &[(form, weapon)]);
        answer(e, READ_DWORD_AT_15C, kind);
        answer(e, ITEM_CHANGE_GET_POISON, poison);
        (player, held, form, weapon)
    }

    #[test]
    fn poison_start_checks_the_held_weapon_before_asking() {
        let mut e = engine(CALLEES_0095E6F0);
        let messages = message_world(&mut e);
        let (player, _, _, _) = holder_world(&mut e, 0x2d, 0);
        let poison = 0x5000_0001;
        let callback = 0x0095_e900;

        // No poison: nothing at all.
        let log = logged(&mut e, |e| {
            e.call(0x0095_e6f0, &args![player, 0u32]);
        });
        assert_eq!(log.len(), 1);

        // A weapon kind of 0x26 passes as well; 0x2d and 0x26 are accepted.
        answer(&mut e, READ_DWORD_AT_15C, 0x26);
        e.register_double(SPRINTF, |e, a| {
            assert_eq!(a[1], FORMAT_0108B3B4);
            let shown = format!("{}|{}", read_text(e, a[2]), read_text(e, a[3]));
            e.mem.set_cstr(a[0], shown.as_bytes());
            Ret::default()
        });
        e.register_double(STRING_OR_EMPTY_00408DA0, |e, a| {
            assert!(a[0] != 0);
            int(text(e, "weapon name"))
        });
        e.call(0x0095_e6f0, &args![player, poison]);
        assert_eq!(e.get(player, PlayerCharacter::pPendingPoison), poison);
        let shown = messages.borrow().last().cloned().unwrap();
        assert_eq!(
            shown.0,
            format!("setting {:08x}|weapon name", SETTING_011D4E94)
        );
        // Two buttons: the texts of settings 011d34f8 then 011d3684.
        assert_eq!(shown.1[3], callback);
        assert_eq!(shown.1[4], 2);
        assert_eq!(shown.1[5], 0x17);
        assert_eq!(
            read_text(&e, shown.1[8]),
            format!("setting {:08x}", SETTING_011D34F8)
        );
        assert_eq!(
            read_text(&e, shown.1[9]),
            format!("setting {:08x}", SETTING_011D3684)
        );
        assert_eq!(shown.1[10], 0);
    }

    #[test]
    fn poison_start_reports_each_reason_it_cannot_apply() {
        let mut e = engine(CALLEES_0095E6F0);
        let messages = message_world(&mut e);
        let (player, _, form, weapon) = holder_world(&mut e, 0x2d, 0);
        let failing = |e: &Engine, messages: &Messages, setting: u32| {
            let shown = messages.borrow().last().cloned().unwrap();
            assert_eq!(shown.0, format!("setting {setting:08x}"));
            // One button: the text of setting 011d38b8.
            assert_eq!(shown.1[4], 1);
            assert_eq!(
                read_text(e, shown.1[8]),
                format!("setting {SETTING_011D38B8:08x}")
            );
        };

        // The wrong weapon kind.
        answer(&mut e, READ_DWORD_AT_15C, 1);
        e.call(0x0095_e6f0, &args![player, 0x5000_0001u32]);
        failing(&e, &messages, SETTING_011D2748);
        // No cast.
        dynamic_casts(&mut e, &[]);
        e.call(0x0095_e6f0, &args![player, 0x5000_0001u32]);
        failing(&e, &messages, SETTING_011D2748);
        // Already poisoned.
        dynamic_casts(&mut e, &[(form, weapon)]);
        answer(&mut e, READ_DWORD_AT_15C, 0x2d);
        answer(&mut e, ITEM_CHANGE_GET_POISON, 0x5000_0009);
        e.call(0x0095_e6f0, &args![player, 0x5000_0001u32]);
        failing(&e, &messages, SETTING_011D2814);
        assert_eq!(e.get(player, PlayerCharacter::pPendingPoison), 0);
        assert_eq!(messages.borrow().len(), 3);
    }

    #[test]
    fn poison_confirmation_applies_only_for_the_second_button() {
        let mut e = engine(CALLEES_0095E900);
        let (player, held, form, weapon) = holder_world(&mut e, 0x2d, 0);
        let poison = 0x5000_0001;
        e.set(player, PlayerCharacter::pPendingPoison, poison);
        dynamic_casts(&mut e, &[(form, weapon)]);

        // Button 1: only the pending poison is dropped.
        answer(&mut e, GET_MESSAGE_MENU_RESULT, 1);
        let log = logged(&mut e, |e| {
            e.call(0x0095_e900, &args![]);
        });
        assert_eq!(log.len(), 2);
        assert_eq!(e.get(player, PlayerCharacter::pPendingPoison), 0);

        // Button 2: the poison is applied.
        e.set(player, PlayerCharacter::pPendingPoison, poison);
        answer(&mut e, GET_MESSAGE_MENU_RESULT, 2);
        let log = logged(&mut e, |e| {
            e.call(0x0095_e900, &args![]);
        });
        assert_eq!(
            calls_to(&log, FUNCTION_005F5950),
            vec![
                vec![5, 1, poison, 0, 0, 0, 0],
                vec![9, 1, poison, 0, 0, 0, 0]
            ]
        );
        assert_eq!(
            calls_to(&log, ITEM_CHANGE_FUNCTION_004BDD20),
            vec![vec![held, poison]]
        );
        assert_eq!(
            calls_to(&log, SLOT_E),
            vec![vec![player.addr(), poison, 0, 1, 0, 0, 0, 0, 0, 1, 0]]
        );
        assert_eq!(calls_to(&log, FUNCTION_00704AF0).len(), 1);
        assert_eq!(e.get(player, PlayerCharacter::pPendingPoison), 0);

        // Nothing pending: nothing happens.
        let log = logged(&mut e, |e| {
            e.call(0x0095_e900, &args![]);
        });
        assert_eq!(log.len(), 1);
    }

    /// A reference-like object whose base at +0x88 has a virtual +0x14
    /// recorded as `SLOT_D` and whose `MagicTarget` base is at +0x94.
    fn caster_world(e: &mut Engine) -> Ptr<PlayerCharacter> {
        let player = e.new_object::<PlayerCharacter>();
        let caster = object_with_slots(e, &[(0x14, SLOT_D)]);
        let vtable = e.mem.u32(caster);
        e.mem.set_u32(player.addr() + MAGIC_CASTER_OFFSET, vtable);
        answer(e, SLOT_D, 0);
        e.register_double(FORM_GET_TYPE, |e, a| int(u32::from(e.mem.u8(a[0] + 4))));
        e.register_double(READ_FIELD_4, |e, a| int(e.mem.u32(a[0] + 4)));
        player
    }

    #[test]
    fn effect_item_forwarding_needs_a_form_of_type_0x18_or_0x1a() {
        let mut e = engine(CALLEES_0095EA30);
        let player = caster_world(&mut e);
        let form = e.mem.alloc(0x20);
        let cast = e.mem.alloc(0x20);
        e.mem.set_u32(cast + 4, 0x9000_0000);
        dynamic_casts(&mut e, &[(form, cast)]);
        e.mem.set_u8(form + 4, 0x18);
        let target = player.addr() + MAGIC_TARGET_OFFSET;
        let log = logged(&mut e, |e| {
            e.call(0x0095_ea30, &args![player, Ptr::<()>::new(form)]);
        });
        assert_eq!(
            calls_to(&log, SLOT_D),
            vec![vec![player.addr() + 0x88, 0x9000_0018, form, 0]]
        );
        assert_eq!(
            calls_to(&log, MAGIC_TARGET_ADD_EFFECT_ITEM),
            vec![vec![target, 0x9000_0018]]
        );
        // Type 0x1a also; any other type and a failed cast do nothing.
        e.mem.set_u8(form + 4, 0x1a);
        let log = logged(&mut e, |e| {
            e.call(0x0095_ea30, &args![player, Ptr::<()>::new(form)]);
        });
        assert_eq!(calls_to(&log, SLOT_D).len(), 1);
        e.mem.set_u8(form + 4, 0x19);
        let log = logged(&mut e, |e| {
            e.call(0x0095_ea30, &args![player, Ptr::<()>::new(form)]);
        });
        assert_eq!(log.len(), 3);
        e.mem.set_u8(form + 4, 0x18);
        dynamic_casts(&mut e, &[]);
        let log = logged(&mut e, |e| {
            e.call(0x0095_ea30, &args![player, Ptr::<()>::new(form)]);
        });
        assert!(calls_to(&log, SLOT_D).is_empty());
    }

    #[test]
    fn queued_enchantment_removal_frees_the_list_when_it_empties() {
        let mut e = engine(CALLEES_0095EB10);
        let player = caster_world(&mut e);
        let form = e.mem.alloc(0x20);
        e.mem.set_u8(form + 4, 0x1a);
        let cast = e.mem.alloc(0x20);
        e.mem.set_u32(cast + 4, 0x9000_0000);
        dynamic_casts(&mut e, &[(form, cast)]);
        let list = e.mem.alloc(8);
        e.set(
            player,
            PlayerCharacter::QueuedWornEnchantments,
            Ptr::new(list),
        );
        answer(&mut e, MAGIC_TARGET_IS_SPELL_TARGET, 1);
        let target = player.addr() + MAGIC_TARGET_OFFSET;

        let log = logged(&mut e, |e| {
            e.call(0x0095_eb10, &args![player, Ptr::<()>::new(form)]);
        });
        assert_eq!(
            calls_to(&log, MAGIC_TARGET_IS_SPELL_TARGET),
            vec![vec![target, 0x9000_0018, 1]]
        );
        assert_eq!(
            calls_to(&log, FUNCTION_008248E0),
            vec![vec![target, form, 0]]
        );
        let removed = calls_to(&log, LIST_REMOVE);
        assert_eq!(removed.len(), 1);
        assert_eq!(removed[0][0], list);
        // The list is empty (the double says so): freed and cleared.
        answer(&mut e, LIST_NODE_IS_EMPTY, 1);
        e.call(0x0095_eb10, &args![player, Ptr::<()>::new(form)]);
        assert!(e
            .get(player, PlayerCharacter::QueuedWornEnchantments)
            .is_null());

        // Without a queued list nothing is done.
        let log = logged(&mut e, |e| {
            e.call(0x0095_eb10, &args![player, Ptr::<()>::new(form)]);
        });
        assert_eq!(log.len(), 3);
    }

    #[test]
    fn queued_enchantment_removal_keeps_a_list_that_still_has_items() {
        let mut e = engine(CALLEES_0095EB10);
        let player = caster_world(&mut e);
        let form = e.mem.alloc(0x20);
        e.mem.set_u8(form + 4, 0x18);
        let cast = e.mem.alloc(0x20);
        e.mem.set_u32(cast + 4, 0x9000_0000);
        dynamic_casts(&mut e, &[(form, cast)]);
        let list = e.mem.alloc(8);
        e.set(
            player,
            PlayerCharacter::QueuedWornEnchantments,
            Ptr::new(list),
        );
        let log = logged(&mut e, |e| {
            e.call(0x0095_eb10, &args![player, Ptr::<()>::new(form)]);
        });
        // The target does not know the item: no removal there.
        assert!(calls_to(&log, FUNCTION_008248E0).is_empty());
        assert!(calls_to(&log, LIST_DELETE).is_empty());
        assert_eq!(
            e.get(player, PlayerCharacter::QueuedWornEnchantments),
            Ptr::new(list)
        );
    }

    #[test]
    fn queued_enchantments_are_all_cast_then_the_list_is_freed() {
        let mut e = engine(CALLEES_0095EC40);
        list_accessors(&mut e);
        let player = caster_world(&mut e);
        let first = e.mem.alloc(0x20);
        let second = e.mem.alloc(0x20);
        let cast = e.mem.alloc(0x20);
        e.mem.set_u32(cast + 4, 0x9000_0000);
        dynamic_casts(&mut e, &[(first, cast)]);
        let list = list_of(&mut e, &[first, second]);
        e.set(
            player,
            PlayerCharacter::QueuedWornEnchantments,
            Ptr::new(list),
        );
        let log = logged(&mut e, |e| {
            e.call(0x0095_ec40, &args![player]);
        });
        assert_eq!(
            calls_to(&log, SLOT_D),
            vec![vec![player.addr() + 0x88, 0x9000_0018, first, 0]]
        );
        assert_eq!(calls_to(&log, LIST_CLEAR_ITEMS), vec![vec![list]]);
        assert_eq!(calls_to(&log, LIST_DELETE), vec![vec![list, 1]]);
        assert!(e
            .get(player, PlayerCharacter::QueuedWornEnchantments)
            .is_null());
        let log = logged(&mut e, |e| {
            e.call(0x0095_ec40, &args![player]);
        });
        assert_eq!(log.len(), 1);
    }

    #[test]
    fn actor_value_modifiers_are_cleared_unless_the_target_reports_them() {
        let mut e = engine(CALLEES_0095ED80);
        let player = e.new_object::<PlayerCharacter>();
        for index in 0..ACTOR_VALUE_MODIFIER_COUNT {
            e.mem.set_f32(player.addr() + 0x244 + index * 4, 1.5);
        }
        e.mem.set_f32(player.addr() + 0x244 + 7 * 4, 0.0);
        e.register_double(FUNCTION_00822F70, |_, a| int(u32::from(a[1] == 3)));
        e.call(0x0095_ed80, &args![player]);
        let value = |e: &Engine, index: u32| e.mem.f32(player.addr() + 0x244 + index * 4);
        assert_eq!(value(&e, 3), 1.5);
        assert_eq!(value(&e, 4), 0.0);
        assert_eq!(value(&e, 0), 0.0);
        assert_eq!(value(&e, 7), 0.0);
        assert_eq!(value(&e, ACTOR_VALUE_MODIFIER_COUNT - 1), 0.0);
    }

    #[test]
    fn camera_caster_follows_the_object_it_does_not_yet_have() {
        let mut e = engine(CALLEES_0095EDF0);
        let player = e.new_object::<PlayerCharacter>();
        let caster = e.mem.alloc(0x20);
        // No caster: nothing.
        e.call(0x0095_edf0, &args![player]);
        e.set(player, PlayerCharacter::pCameraCaster, Ptr::new(caster));
        // No object at +0x40: nothing.
        let log = logged(&mut e, |e| {
            e.call(0x0095_edf0, &args![player]);
        });
        assert_eq!(log.len(), 2);

        e.register_double(READ_DWORD_AT_40, |e, a| int(e.mem.u32(a[0] + 0x40)));
        e.mem.set_u32(player.addr() + 0x40, 0x8000_0000);
        answer(&mut e, CASTER_GET_WORLD, 5);
        answer(&mut e, OBJECT_FUNCTION_004543C0, 5);
        let log = logged(&mut e, |e| {
            e.call(0x0095_edf0, &args![player]);
        });
        assert!(calls_to(&log, CASTER_SET_WORLD).is_empty());
        answer(&mut e, OBJECT_FUNCTION_004543C0, 6);
        let log = logged(&mut e, |e| {
            e.call(0x0095_edf0, &args![player]);
        });
        assert_eq!(calls_to(&log, CASTER_SET_WORLD), vec![vec![caster, 6]]);
    }

    #[test]
    fn forwarders_return_what_the_callee_returns() {
        let mut e = engine(&[FUNCTION_00705800, FUNCTION_007D0B90]);
        answer(&mut e, FUNCTION_00705800, 0x1234);
        answer(&mut e, FUNCTION_007D0B90, 0x5678);
        e.set_global(POINTER_011DEA0C, 0x4000_0000);
        assert_eq!(e.call(0x0095_ee60, &args![]).u32(), 0x1234);
        let log = logged(&mut e, |e| {
            assert_eq!(e.call(0x0095_ee70, &args![]).u32(), 0x5678);
        });
        assert_eq!(calls_to(&log, FUNCTION_007D0B90), vec![vec![0x4000_0000]]);
    }

    /// `strcpy_s(destination, size, source)` and `strrchr(string, c)` as
    /// the game's string functions do them.
    fn string_functions(e: &mut Engine) {
        e.register_double(STRING_COPY, |e, a| {
            let source = e.mem.cstr(a[2]);
            let room = (a[1] as usize).saturating_sub(1);
            let copied = &source[..source.len().min(room)];
            e.mem.set_cstr(a[0], copied);
            Ret::default()
        });
        e.register_double(STRING_FIND_LAST, |e, a| {
            let bytes = e.mem.cstr(a[0]);
            int(bytes
                .iter()
                .rposition(|&b| u32::from(b) == a[1])
                .map_or(0, |at| a[0] + at as u32))
        });
    }

    #[test]
    fn first_person_animation_folders_are_registered_with_each_suffix() {
        let mut e = engine(CALLEES_0095EE80);
        string_functions(&mut e);
        let folder = text(&mut e, "meshes\\anim\\first.kf");
        e.register_double(SETTING_STRING, move |_, _| int(folder));
        let model = text(&mut e, "meshes\\actors\\p.nif");
        answer(&mut e, GET_MODEL_PATH, model);
        for (address, suffix) in [
            (SUFFIX_010170EC, "\\folder"),
            (SUFFIX_0108B3D8, "\\a"),
            (SUFFIX_0108B3BC, "\\b"),
            (SUFFIX_01016F1C, "\\c"),
            (SUFFIX_01016F38, "\\d"),
            (SUFFIX_0108B0D8, "\\e"),
            (SUFFIX_01016F58, "\\f"),
        ] {
            e.mem.set_cstr(address, suffix.as_bytes());
        }
        e.set_global(IDLE_MANAGER_POINTER, 0x4000_0000);
        let registered = Rc::new(RefCell::new(Vec::new()));
        let seen = registered.clone();
        e.register_double(ADD_ROOT_IDLE_ARRAY, move |e, a| {
            assert_eq!((a[0], a[2], a[3]), (0x4000_0000, 0, 0));
            seen.borrow_mut().push(read_text(e, a[1]));
            Ret::default()
        });
        let player = e.new_object::<PlayerCharacter>();
        let log = logged(&mut e, |e| {
            e.call(0x0095_ee80, &args![player]);
        });
        assert_eq!(
            *registered.borrow(),
            vec![
                "meshes\\anim\\\\folder",
                "meshes\\anim\\a",
                "meshes\\anim\\b",
                "meshes\\anim\\c",
                "meshes\\anim\\d",
                "meshes\\anim\\e",
                "meshes\\actors\\a",
                "meshes\\actors\\b",
                "meshes\\actors\\c",
                "meshes\\actors\\d",
                "meshes\\actors\\f",
            ]
        );
        // The scope guard is built with the unit's source file and line
        // and destroyed at the end.
        let guard = calls_to(&log, SCOPE_GUARD_NEW);
        assert_eq!(guard.len(), 1);
        assert_eq!(guard[0][1..], [0x34, 1, SCOPE_GUARD_FILE, 0x41c5]);
        assert_eq!(calls_to(&log, SCOPE_GUARD_DELETE), vec![vec![guard[0][0]]]);
    }

    #[test]
    fn animation_lists_are_built_and_queued_for_the_players_base_and_model() {
        let mut e = engine(CALLEES_0095F210);
        string_functions(&mut e);
        let player = e.new_object::<PlayerCharacter>();
        let base = object_with_slots(&mut e, &[]);
        let base_part = object_with_slots(&mut e, &[(0x18, SLOT_D)]);
        e.mem.set_u32(base + 0xdc, e.mem.u32(base_part));
        answer(&mut e, SLOT_D, 0);
        answer(&mut e, READ_DWORD_AT_20, base);
        answer(&mut e, ACTOR_BASE_GET_SEX, 1);
        let folder = text(&mut e, "meshes\\anim\\first.kf");
        e.register_double(SETTING_STRING, move |e, a| {
            if a[0] == SETTING_011CDD78 {
                int(folder)
            } else {
                int(text(e, &format!("setting {:08x}", a[0])))
            }
        });
        e.register_double(FUNCTION_00464F30, |_, a| int(a[0]));
        let model = text(&mut e, "meshes\\actors\\p.nif");
        answer(&mut e, GET_MODEL_PATH, model);
        e.mem.set_cstr(SUFFIX_0108B0D8, b"\\extra");
        e.mem.set_cstr(SUFFIX_01016F38, b"\\female");
        e.mem.set_cstr(SUFFIX_01016F1C, b"\\male");
        e.set_global(IDLE_MANAGER_POINTER, 0x4000_0000);
        e.set_global(MODEL_LOADER_POINTER, 0x4100_0000);
        let next = Rc::new(std::cell::Cell::new(0x6000_0000u32));
        let counter = next.clone();
        e.register_double(BUILD_FILE_LIST, move |_, _| {
            counter.set(counter.get() + 0x100);
            int(counter.get())
        });
        e.register_double(GET_ROOT_FILENAME_LIST, |e, a| {
            int(e.mem.cstr(a[1]).len() as u32 + 0x7000_0000)
        });
        answer(&mut e, FUNCTION_008397D0, 1);
        answer(&mut e, FUNCTION_0087F4C0, 1);

        let log = logged(&mut e, |e| {
            e.call(0x0095_f210, &args![player, 11u32, 22u32]);
        });
        // Female base: the first sex setting, 011d2718, goes to the base.
        assert_eq!(calls_to(&log, SLOT_D).len(), 1);
        assert_eq!(
            read_text(&e, calls_to(&log, SLOT_D)[0][1]),
            format!("setting {SETTING_011D2718:08x}")
        );
        // Both file lists are built by the loader and queued, then deleted.
        assert_eq!(
            calls_to(&log, FUNCTION_00446500),
            vec![
                vec![0x4100_0000, 0x6000_0100, 11, 22, 0],
                vec![0x4100_0000, 0x6000_0200, 11, 22, 0],
            ]
        );
        assert_eq!(
            calls_to(&log, LIST_DELETE),
            vec![vec![0x6000_0100, 1], vec![0x6000_0200, 1]]
        );
        // The root filename lists of the extra and sex suffixes are copied
        // into the first list: "...\extra" and "...\female".
        let copies = calls_to(&log, COPY_FILENAME_LIST);
        assert_eq!(copies.len(), 2);
        assert_eq!(copies[0][0], 0x4100_0000);
        assert_eq!(copies[0][2], 0x6000_0100);
        assert_eq!(
            copies[0][1],
            0x7000_0000 + "meshes\\anim\\extra".len() as u32
        );
        assert_eq!(
            copies[1][1],
            0x7000_0000 + "meshes\\anim\\female".len() as u32
        );
        // The model of the player itself is queued with the same words.
        assert_eq!(
            calls_to(&log, QUEUE_MODEL),
            vec![vec![0x4100_0000, folder, 11, 22, 0, 1, 0, 0]]
        );

        // A male base without the extra suffix.
        answer(&mut e, ACTOR_BASE_GET_SEX, 0);
        answer(&mut e, FUNCTION_0087F4C0, 0);
        answer(&mut e, FUNCTION_008397D0, 0);
        let log = logged(&mut e, |e| {
            e.call(0x0095_f210, &args![player, 11u32, 22u32]);
        });
        assert_eq!(
            read_text(&e, calls_to(&log, SLOT_D)[0][1]),
            format!("setting {SETTING_011D3E44:08x}")
        );
        let copies = calls_to(&log, COPY_FILENAME_LIST);
        assert_eq!(copies.len(), 1);
        assert_eq!(
            copies[0][1],
            0x7000_0000 + "meshes\\anim\\male".len() as u32
        );
    }

    fn controls_world(e: &mut Engine) -> Ptr<PlayerCharacter> {
        let player = e.new_object::<PlayerCharacter>();
        let mover = object_with_slots(e, &[(0xc, SLOT_D)]);
        answer(e, SLOT_D, 0);
        e.set(player, PlayerCharacter::pActorMover, Ptr::new(mover));
        e.register_double(READ_DWORD_AT_0, |e, a| int(e.mem.u32(a[0])));
        player
    }

    #[test]
    fn disabled_controls_are_applied_bit_by_bit() {
        let mut e = engine(CALLEES_0095F590);
        let player = controls_world(&mut e);
        e.set(player, PlayerCharacter::bWant3rdPerson, 1);
        answer(&mut e, ACTOR_FUNCTION_008846E0, 0xffff);

        // Bit 0: HUD menu mode 0xc, else 1; bit 7 is dropped.
        let log = logged(&mut e, |e| {
            e.call(0x0095_f590, &args![player, 0x81u8]);
        });
        assert_eq!(e.get(player, PlayerCharacter::ucControlsDisabled), 0x01);
        assert_eq!(calls_to(&log, HUD_SET_MENU_MODE), vec![vec![0xc]]);
        assert_eq!(log.len(), 2);
        let log = logged(&mut e, |e| {
            e.call(0x0095_f590, &args![player, 0x00u8]);
        });
        assert_eq!(calls_to(&log, HUD_SET_MENU_MODE), vec![vec![1]]);

        // 0xff is stored whole.
        e.call(0x0095_f590, &args![player, 0xffu8]);
        assert_eq!(e.get(player, PlayerCharacter::ucControlsDisabled), 0xff);
    }

    #[test]
    fn disabled_controls_side_effects() {
        let mut e = engine(CALLEES_0095F590);
        let player = controls_world(&mut e);
        e.set(player, PlayerCharacter::bWant3rdPerson, 1);
        e.set_global(GLOBAL_011E0768, 1.0f32);
        answer(&mut e, ACTOR_FUNCTION_008846E0, 0xffff);

        // Bit 3 calls the helper; bit 4 clears the third person wish and
        // zeroes the global when there is no first-person 3D.
        let log = logged(&mut e, |e| {
            e.call(0x0095_f590, &args![player, 0x18u8]);
        });
        assert_eq!(
            calls_to(&log, FUNCTION_008A6840),
            vec![vec![player.addr(), 0]]
        );
        assert_eq!(e.get(player, PlayerCharacter::bWant3rdPerson), 0);
        assert_eq!(e.global::<f32>(GLOBAL_011E0768), 0.0);
        assert!(calls_to(&log, PLAYER_UPDATE_CAMERA).is_empty());

        // With a first-person 3D the camera is updated instead.
        e.set(
            player,
            PlayerCharacter::sp1stPerson3D,
            Ptr::new(0x8000_0000),
        );
        e.set_global(GLOBAL_011E0768, 1.0f32);
        let log = logged(&mut e, |e| {
            e.call(0x0095_f590, &args![player, 0x10u8]);
        });
        assert_eq!(
            calls_to(&log, PLAYER_UPDATE_CAMERA),
            vec![vec![player.addr(), 0, 0]]
        );
        assert_eq!(e.global::<f32>(GLOBAL_011E0768), 1.0);

        // Bit 6: the actor mover gets the actor's word without bit 10.
        let log = logged(&mut e, |e| {
            e.call(0x0095_f590, &args![player, 0x40u8]);
        });
        let mover = e.get(player, PlayerCharacter::pActorMover).addr();
        assert_eq!(calls_to(&log, SLOT_D), vec![vec![mover, 0xfbff]]);
    }

    #[test]
    fn control_bits_are_set_or_cleared_before_applying() {
        let mut e = engine(CALLEES_0095F530);
        let player = controls_world(&mut e);
        e.set(player, PlayerCharacter::ucControlsDisabled, 0b0000_0101);
        e.call(0x0095_f530, &args![player, 1u8, 0b0000_0010u8]);
        assert_eq!(e.get(player, PlayerCharacter::ucControlsDisabled), 0b0111);
        e.call(0x0095_f530, &args![player, 0u8, 0b0000_0101u8]);
        assert_eq!(e.get(player, PlayerCharacter::ucControlsDisabled), 0b0010);
    }

    #[test]
    fn grab_release_forwarder_returns_the_player() {
        let mut e = engine(CALLEES_0095F6A0);
        let player = controls_world(&mut e);
        assert_eq!(
            e.call(0x0095_f6a0, &args![player]).ptr::<PlayerCharacter>(),
            player
        );
    }

    fn grab_input_world(e: &mut Engine) -> Ptr<PlayerCharacter> {
        let player = controls_world(e);
        e.set_global(POINTER_011DEA0C, 0x4200_0000);
        player
    }

    #[test]
    fn grab_update_does_nothing_without_input_state_or_spring() {
        let mut e = engine(CALLEES_0095F6C0);
        let player = grab_input_world(&mut e);
        let log = logged(&mut e, |e| {
            e.call(0x0095_f6c0, &args![player, 0.016f32]);
        });
        assert_eq!(
            log.iter().map(|(a, _)| *a).collect::<Vec<_>>(),
            vec![
                0x0095_f6c0,
                FUNCTION_00877720,
                CONTROL_QUERY,
                FUNCTION_00705A90,
                READ_DWORD_AT_0
            ]
        );
        assert_eq!(calls_to(&log, CONTROL_QUERY), vec![vec![0, 0x1b, 1]]);
    }

    #[test]
    fn grab_update_keeps_updating_a_held_spring_only_while_the_control_is_down() {
        let mut e = engine(CALLEES_0095F6C0);
        let player = grab_input_world(&mut e);
        e.register_double(READ_DWORD_AT_0, |e, a| int(e.mem.u32(a[0])));
        let spring = object_with_slots(&mut e, &[(0xa0, SLOT_E)]);
        answer(&mut e, SLOT_E, 0);
        e.mem.set_u32(player.addr() + 0x634, spring);
        e.set(player, PlayerCharacter::eGrabType, 1);
        e.mem.set_u8(FLAG_011E0D5C, 1);
        // Mode 2 forces the update even with the control down: with no
        // grabbed object the update releases (which calls the action
        // removal), but the flag stays.
        answer(&mut e, FUNCTION_00705A90, 2);
        let log = logged(&mut e, |e| {
            e.call(0x0095_f6c0, &args![player, 0.016f32]);
        });
        assert_eq!(e.mem.u8(FLAG_011E0D5C), 1);
        assert_eq!(calls_to(&log, PLAYER_REMOVE_PLAYER_ACTION).len(), 1);
        // The flag is on and the control up: the update runs again.
        e.mem.set_u32(player.addr() + 0x634, spring);
        e.set(player, PlayerCharacter::eGrabType, 1);
        answer(&mut e, FUNCTION_00705A90, 0);
        e.call(0x0095_f6c0, &args![player, 0.016f32]);
        assert_eq!(e.mem.u8(FLAG_011E0D5C), 1);
        // With the control down and no mode 2, the spring is released and
        // the flag cleared.
        e.mem.set_u32(player.addr() + 0x634, spring);
        e.set(player, PlayerCharacter::eGrabType, 1);
        answer(&mut e, CONTROL_QUERY, 1);
        e.call(0x0095_f6c0, &args![player, 0.016f32]);
        assert_eq!(e.mem.u8(FLAG_011E0D5C), 0);
    }

    #[test]
    fn grab_update_starts_a_spring_on_the_object_in_front() {
        let mut e = engine(CALLEES_0095F6C0);
        let player = grab_input_world(&mut e);
        e.register_double(READ_DWORD_AT_0, |e, a| int(e.mem.u32(a[0])));
        answer(&mut e, CONTROL_QUERY, 1);
        e.set_global(POINTER_011CA27C, 0x1111);
        e.set_global(POINTER_011CA280, 0x2222);
        e.set_global(F32_CONSTANT_01013974, 1000.0f32);
        let option = e.mem.alloc(4);
        answer(&mut e, SETTING_BOOL_POINTER, option);
        answer(&mut e, SLOT_D, 0);
        let target = object_with_slots(
            &mut e,
            &[
                (0x1d0, SLOT_A),
                (0x48, SLOT_B),
                (0x100, SLOT_D),
                (0x224, SLOT_B),
            ],
        );
        e.mem.set_u32(target + 0x40, 0x8200_0000);
        e.register_double(READ_DWORD_AT_40, |e, a| int(e.mem.u32(a[0] + 0x40)));
        answer(&mut e, FUNCTION_004B52F0, 0);
        let body = object_with_slots(&mut e, &[(0x94, SLOT_C)]);
        answer(&mut e, SLOT_A, 0x8000_0000);
        answer(&mut e, SLOT_B, 0);
        answer(&mut e, SLOT_C, 1);
        answer(&mut e, FUNCTION_00703350, target);
        answer(&mut e, READ_DWORD_AT_20, 0x3333);
        answer(&mut e, FIND_FIRST_COLLISION_OBJECT, 0x8100_0000);
        answer(&mut e, FUNCTION_006FA820, body);
        answer_float(&mut e, BODY_VALUE_004B5400, 5.0);
        float_settings(&mut e, &[(SETTING_011D1230, 10.0)]);
        e.register_double(ADD_44_TO_ADDRESS, |_, a| int(a[0] + 0x44));
        let log = logged(&mut e, |e| {
            e.call(0x0095_f6c0, &args![player, 0.016f32]);
        });
        // The creation call is the real one; the object is not an actor, so it goes on
        // (0095f998), and with no rigid body under its 3D (004b52f0 null) and grab type 1 it
        // records nothing.
        assert!(e.get(player, PlayerCharacter::pGrabbedObject).is_null());
        assert_eq!(e.mem.u8(FLAG_011E0D5C), 1);
        assert_eq!(
            calls_to(&log, FUNCTION_004213C0),
            vec![vec![target + 0x44, target, player.addr()]]
        );
        assert_eq!(
            calls_to(&log, EXTRA_LIST_REMOVE_EXTRA),
            vec![vec![target + 0x44, 0x7c]]
        );

        // Too heavy: no spring.
        float_settings(&mut e, &[(SETTING_011D1230, 1.0)]);
        e.mem.set_u8(FLAG_011E0D5C, 0);
        e.call(0x0095_f6c0, &args![player, 0.016f32]);
        assert_eq!(e.mem.u8(FLAG_011E0D5C), 0);
    }

    /// A grabbed object (an object with vtable slots 0x100, 0x224, 0x1d0,
    /// 0xf0), a collision body and the doubles the spring creation reads.
    /// Returns (player, object, body).
    fn spring_world(e: &mut Engine) -> (Ptr<PlayerCharacter>, u32, u32) {
        let player = player_with(e, &[], &[]);
        let object = object_with_slots(
            e,
            &[
                (0x100, SLOT_D),
                (0x224, SLOT_C),
                (0x1d0, SLOT_A),
                (0x22c, SLOT_C),
                (0xf0, SLOT_B),
            ],
        );
        answer(e, SLOT_A, 0x8000_0000);
        answer(e, SLOT_C, 0);
        answer(e, SLOT_D, 0);
        e.mem.set_u32(object + 0x40, 0x8200_0000);
        let body = object_with_slots(e, &[(0xf0, SLOT_B)]);
        answer(e, SLOT_B, 0);
        answer(e, FUNCTION_004B52F0, body);
        let option = e.mem.alloc(4);
        e.mem.set_u8(option, 1);
        answer(e, SETTING_BOOL_POINTER, option);
        e.register_double(READ_DWORD_AT_40, |e, a| int(e.mem.u32(a[0] + 0x40)));
        e.register_double(READ_DWORD_AT_0, |e, a| int(e.mem.u32(a[0])));
        e.register_double(ADD_44_TO_ADDRESS, |_, a| int(a[0] + 0x44));
        (player, object, body)
    }

    /// 0095f981..0095f9b3: with the setting off, an object whose virtual +0x100 answers false
    /// (clutter: the maintainer's paperweight) goes on to the grab; one that answers +0x100 but
    /// not +0x22c(0) is refused; one that answers both goes on.
    #[test]
    fn mouse_spring_without_the_option_refuses_only_what_answers_0x100_and_not_0x22c() {
        let mut e = engine(CALLEES_0095F930);
        let (player, object, _) = spring_world(&mut e);
        let option = e.mem.alloc(4);
        answer(&mut e, SETTING_BOOL_POINTER, option);
        answer(&mut e, FUNCTION_004B52F0, 0);
        let log = logged(&mut e, |e| {
            e.call(
                0x0095_f930,
                &args![player, Ptr::<()>::new(object), 1u32, 5.0f32],
            );
        });
        assert_eq!(
            calls_to(&log, SETTING_BOOL_POINTER),
            vec![vec![SETTING_011E0B1C]]
        );
        // The +0x100 answer is false: no +0x22c question, and the grab goes on (the object
        // here has no body to continue with).
        assert!(calls_to(&log, SLOT_C).iter().all(|c| c.len() == 1));
        assert_eq!(calls_to(&log, READ_DWORD_AT_40), vec![vec![object]]);
        // +0x100 true and +0x22c(0) false: refused before anything else.
        let refusing = object_with_slots(&mut e, &[(0x100, SLOT_F), (0x22c, SLOT_C)]);
        answer(&mut e, SLOT_F, 1);
        let log = logged(&mut e, |e| {
            e.call(
                0x0095_f930,
                &args![player, Ptr::<()>::new(refusing), 1u32, 5.0f32],
            );
        });
        assert_eq!(calls_to(&log, SLOT_C), vec![vec![refusing, 0]]);
        assert!(calls_to(&log, READ_DWORD_AT_40).is_empty());
        // An actor that answers the virtual +0x22c continues (and the
        // object here has no body to continue with).
        let actor = object_with_slots(
            &mut e,
            &[
                (0x100, SLOT_E),
                (0x22c, SLOT_E),
                (0x224, SLOT_D),
                (0x1d0, SLOT_A),
            ],
        );
        e.mem.set_u32(actor + 0x40, 1);
        answer(&mut e, SLOT_E, 1);
        answer(&mut e, FUNCTION_004B52F0, 0);
        let log = logged(&mut e, |e| {
            e.call(
                0x0095_f930,
                &args![player, Ptr::<()>::new(actor), 1u32, 5.0f32],
            );
        });
        assert_eq!(calls_to(&log, SLOT_E), vec![vec![actor], vec![actor, 0]]);
    }

    #[test]
    fn mouse_spring_is_refused_for_an_object_the_check_rejects() {
        let mut e = engine(CALLEES_0095F930);
        let (player, _, _) = spring_world(&mut e);
        let object =
            object_with_slots(&mut e, &[(0x224, SLOT_E), (0x304, SLOT_D), (0x318, SLOT_C)]);
        answer(&mut e, SLOT_E, 1);
        answer(&mut e, SLOT_D, 3);
        answer(&mut e, SLOT_C, 1);
        answer(&mut e, FUNCTION_00444D00, 0);
        let log = logged(&mut e, |e| {
            e.call(
                0x0095_f930,
                &args![player, Ptr::<()>::new(object), 1u32, 5.0f32],
            );
        });
        assert_eq!(calls_to(&log, FUNCTION_00444D00), vec![vec![object, 0x200]]);
        assert!(calls_to(&log, READ_DWORD_AT_40).is_empty());
        assert!(e.get(player, PlayerCharacter::pGrabbedObject).is_null());
    }

    #[test]
    fn a_body_less_object_is_only_recorded_for_grab_type_two() {
        let mut e = engine(CALLEES_0095F930);
        let (player, object, _) = spring_world(&mut e);
        answer(&mut e, FUNCTION_004B52F0, 0);
        let log = logged(&mut e, |e| {
            e.call(
                0x0095_f930,
                &args![player, Ptr::<()>::new(object), 1u32, 5.0f32],
            );
        });
        assert!(e.get(player, PlayerCharacter::pGrabbedObject).is_null());
        assert!(calls_to(&log, SCRIPT_SET_ACTION_FLAG).is_empty());
        let log = logged(&mut e, |e| {
            e.call(
                0x0095_f930,
                &args![player, Ptr::<()>::new(object), 2u32, 5.0f32],
            );
        });
        assert_eq!(
            e.get(player, PlayerCharacter::pGrabbedObject).addr(),
            object
        );
        assert_eq!(e.get(player, PlayerCharacter::eGrabType), 2);
        assert_eq!(e.get(player, PlayerCharacter::fGrabDistance), 5.0);
        assert_eq!(
            calls_to(&log, SCRIPT_SET_ACTION_FLAG),
            vec![vec![0, object + 0x44, 0x8_0000]]
        );
    }

    /// The doubles the spring creation needs to run through to the creation
    /// of the `bhkMouseSpringAction`; the parameter block's four settings
    /// are captured when the action is built.
    fn spring_creation_world(
        e: &mut Engine,
    ) -> (Ptr<PlayerCharacter>, u32, u32, Rc<RefCell<Vec<f32>>>) {
        let (player, object, _) = spring_world(e);
        let hit = e.mem.alloc(0x40);
        e.mem.set_u32(hit + 8, 1);
        answer(e, BODY_LOOKUP_004AE750, hit);
        e.register_double(READ_FIELD_8, |e, a| int(e.mem.u32(a[0] + 8)));
        let zero_vector = e.mem.alloc(0x20);
        answer(e, FUNCTION_004B4EC0, zero_vector);
        let spring = object_with_slots(e, &[(0x9c, SLOT_E)]);
        answer(e, SLOT_E, 0);
        answer(e, ALLOCATE_00AA13E0, spring);
        e.register_double(SMART_POINTER_ASSIGN, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            int(a[0])
        });
        let captured = Rc::new(RefCell::new(Vec::new()));
        let seen = captured.clone();
        e.register_double(MOUSE_SPRING_ACTION_NEW, move |e, a| {
            for offset in [0x30, 0x34, 0x38, 0x3c] {
                seen.borrow_mut().push(e.mem.f32(a[1] + offset));
            }
            // The block's +4 is the found body (0095ff5e), the action's entity.
            seen.borrow_mut().push(f32::from_bits(e.mem.u32(a[1] + 4)));
            int(a[0])
        });
        answer_float(e, FUNCTION_0062C300, 12.5);
        (player, object, spring, captured)
    }

    #[test]
    fn mouse_spring_of_type_three_takes_its_parameters_from_the_settings() {
        let mut e = engine(CALLEES_0095F930);
        let (player, object, spring, captured) = spring_creation_world(&mut e);
        answer_float(&mut e, FUNCTION_00457990, 6.0);
        float_settings(
            &mut e,
            &[
                (SETTING_011D1A08, 1.0),
                (SETTING_011D1C7C, 2.0),
                (SETTING_011D1B44, 3.0),
                (SETTING_011D177C, 4.0),
            ],
        );
        let log = logged(&mut e, |e| {
            e.call(
                0x0095_f930,
                &args![player, Ptr::<()>::new(object), 3u32, 7.5f32],
            );
        });
        // [EBP-0x40], -0x3c, -0x38, -0x34 of the block are +0x30 to +0x3c; [EBP-0x6c] is its +4,
        // the body the lookup found (0095ff5e).
        assert!(!calls_to(&log, BODY_LOOKUP_004AE750).is_empty());
        let captured = captured.borrow();
        assert_eq!(captured[..4], [1.0, 2.0, 4.0, 3.0]);
        assert_ne!(captured[4].to_bits(), 0);
        assert_eq!(
            e.get(player, PlayerCharacter::pGrabbedObject).addr(),
            object
        );
        assert_eq!(e.get(player, PlayerCharacter::eGrabType), 3);
        // The search for the hold point sets the distance to the length it finds.
        assert_eq!(e.get(player, PlayerCharacter::fGrabDistance), 6.0);
        assert_eq!(e.get(player, PlayerCharacter::fGrabObjectWeight), 12.5);
        assert_eq!(e.mem.u32(player.addr() + 0x634), spring);
        assert_eq!(calls_to(&log, SLOT_E).len(), 1);
        assert_eq!(
            calls_to(&log, SCRIPT_SET_ACTION_FLAG),
            vec![vec![0, object + 0x44, 0x8_0000]]
        );
    }

    #[test]
    fn mouse_spring_of_other_types_scales_the_settings_by_the_object_kind() {
        let mut e = engine(CALLEES_0095F930);
        let (player, object, _, captured) = spring_creation_world(&mut e);
        float_settings(
            &mut e,
            &[
                (SETTING_011D0358, 2.0),
                (SETTING_011CFEEC, 10.0),
                (SETTING_011D08C8, 3.0),
                (SETTING_011D0FFC, 8.0),
            ],
        );
        e.mem.set_f64(F64_CONSTANT_0101FFA0, 0.5);
        e.mem.set_f64(DOUBLE_HALF, 0.25);
        // The object is not an actor; the kind check answers 0xe.
        answer(&mut e, FUNCTION_0043B4F0, 0x8300_0000);
        answer(&mut e, FUNCTION_0043B4D0, 0xe);
        e.call(
            0x0095_f930,
            &args![player, Ptr::<()>::new(object), 2u32, 7.5f32],
        );
        assert_eq!(captured.borrow()[..4], [2.0, 5.0, 2.0, 3.0]);

        // Any other kind keeps the scales at 1.
        captured.borrow_mut().clear();
        answer(&mut e, FUNCTION_0043B4D0, 0x3);
        e.call(
            0x0095_f930,
            &args![player, Ptr::<()>::new(object), 2u32, 7.5f32],
        );
        assert_eq!(captured.borrow()[..4], [2.0, 10.0, 8.0, 3.0]);
    }

    #[test]
    fn mouse_spring_gives_up_when_the_hit_object_is_unusable() {
        let mut e = engine(CALLEES_0095F930);
        let (player, object, _, captured) = spring_creation_world(&mut e);
        let log = logged(&mut e, |e| {
            e.register_double(FUNCTION_00517670, |_, _| int(5));
            e.call(
                0x0095_f930,
                &args![player, Ptr::<()>::new(object), 3u32, 7.5f32],
            );
        });
        // The release removes the player action and builds no spring.
        assert!(captured.borrow().is_empty());
        assert_eq!(
            calls_to(&log, PLAYER_REMOVE_PLAYER_ACTION),
            vec![vec![player.addr(), 5, 0]]
        );
        assert!(e.get(player, PlayerCharacter::pGrabbedObject).is_null());
    }

    #[test]
    fn small_vector_and_getter_helpers() {
        let mut e = engine(&[]);
        let block = e.mem.alloc(0x700);
        e.mem.set_u32(block + 0x18, 0x1234);
        e.mem.set_u32(block + 0x608, 1);
        assert_eq!(e.call(0x0096_04f0, &args![block]).u32(), block + 0xac);
        e.set_global(GLOBAL_011CC5EC, 0x9876);
        assert_eq!(e.call(0x0096_0510, &args![]).u32(), 0x9876);
        assert_eq!(e.call(0x0096_11e0, &args![block]).u32(), 0x1234);
        assert_eq!(e.call(0x0096_1260, &args![block]).u32(), 1);
        let first = e.mem.alloc(16);
        let second = e.mem.alloc(16);
        for lane in 0..4 {
            e.mem.set_f32(first + lane * 4, 10.0 + lane as f32);
            e.mem.set_f32(second + lane * 4, lane as f32);
        }
        e.call(0x0096_1190, &args![first, second]);
        for lane in 0..4 {
            assert_eq!(e.mem.f32(first + lane * 4), 10.0);
        }
    }

    #[test]
    fn member_copies_use_the_body_that_the_lookup_finds() {
        let mut e = engine(CALLEES_00961200);
        let object = e.mem.alloc(0x40);
        let target = e.mem.alloc(0x40);
        answer(&mut e, BODY_LOOKUP_004AE750, 0);
        let log = logged(&mut e, |e| {
            e.call(0x0096_1200, &args![object, target]);
            e.call(0x0096_1230, &args![object, target]);
        });
        assert!(calls_to(&log, FUNCTION_004A3F10).is_empty());
        answer(&mut e, BODY_LOOKUP_004AE750, 0x8000_0000);
        let log = logged(&mut e, |e| {
            e.call(0x0096_1200, &args![object, target]);
            e.call(0x0096_1230, &args![object, target]);
        });
        assert_eq!(
            calls_to(&log, FUNCTION_004A3F10),
            vec![vec![target, 0x8000_0030], vec![target, 0x8000_0020]]
        );
    }

    #[test]
    fn grab_release_stops_the_spring_and_clears_the_grab() {
        let mut e = engine(CALLEES_00961280);
        let player = e.new_object::<PlayerCharacter>();
        e.register_double(READ_DWORD_AT_0, |e, a| int(e.mem.u32(a[0])));
        e.register_double(READ_DWORD_AT_40, |e, a| int(e.mem.u32(a[0] + 0x40)));
        let spring = object_with_slots(&mut e, &[(0xa0, SLOT_A)]);
        answer(&mut e, SLOT_A, 0);
        let grabbed = object_with_slots(&mut e, &[(0xf0, SLOT_B)]);
        answer(&mut e, SLOT_B, 1);
        e.mem.set_u32(player.addr() + 0x634, spring);
        e.mem.set_u32(player.addr() + 0x40, 0x8000_0000);
        e.set(player, PlayerCharacter::pGrabbedObject, Ptr::new(grabbed));
        e.set(player, PlayerCharacter::eGrabType, 3);
        e.set(player, PlayerCharacter::fGrabObjectWeight, 4.0);
        answer(&mut e, OBJECT_FUNCTION_004543C0, 0x8400_0000);
        answer(&mut e, FUNCTION_0082AEE0, 0x8500_0000);
        e.register_double(ADD_44_TO_ADDRESS, |_, a| int(a[0] + 0x44));

        let log = logged(&mut e, |e| {
            assert_eq!(
                e.call(0x0096_1280, &args![player]).ptr::<PlayerCharacter>(),
                player
            );
        });
        let order: Vec<u32> = log
            .iter()
            .map(|(address, _)| *address)
            .filter(|address| {
                [
                    SLOT_A,
                    OBJECT_FUNCTION_005533C0,
                    HKP_ENTITY_ACTIVATE,
                    PLAYER_REMOVE_PLAYER_ACTION,
                    SMART_POINTER_ASSIGN,
                    SLOT_B,
                    SCRIPT_SET_ACTION_FLAG,
                ]
                .contains(address)
            })
            .collect();
        assert_eq!(
            order,
            vec![
                SLOT_A,
                OBJECT_FUNCTION_005533C0,
                HKP_ENTITY_ACTIVATE,
                OBJECT_FUNCTION_005533C0,
                PLAYER_REMOVE_PLAYER_ACTION,
                SMART_POINTER_ASSIGN,
                SLOT_B,
                SCRIPT_SET_ACTION_FLAG,
            ]
        );
        assert_eq!(calls_to(&log, HKP_ENTITY_ACTIVATE), vec![vec![0x8500_0000]]);
        assert_eq!(
            calls_to(&log, SCRIPT_SET_ACTION_FLAG),
            vec![vec![0, grabbed + 0x44, 0x10_0000]]
        );
        assert_eq!(e.get(player, PlayerCharacter::eGrabType), 0);
        assert_eq!(e.get(player, PlayerCharacter::fGrabObjectWeight), 0.0);
        assert!(e.get(player, PlayerCharacter::pGrabbedObject).is_null());

        // Nothing to release: only the action removal and the clearing.
        e.mem.set_u32(player.addr() + 0x634, 0);
        let log = logged(&mut e, |e| {
            e.call(0x0096_1280, &args![player]);
        });
        assert_eq!(
            calls_to(&log, PLAYER_REMOVE_PLAYER_ACTION),
            vec![vec![player.addr(), 5, 0]]
        );
        assert!(calls_to(&log, HKP_ENTITY_ACTIVATE).is_empty());
    }

    #[test]
    fn drop_spring_only_acts_for_grab_type_two() {
        let mut e = engine(CALLEES_009613C0);
        let (player, object, _) = spring_world(&mut e);
        e.set_global(POINTER_011DEA0C, 0x4200_0000);
        answer_float(&mut e, OBJECT_GET_SCALE, 2.0);
        float_settings(&mut e, &[(SETTING_011D0628, 3.0)]);
        answer(&mut e, FUNCTION_004B52F0, 0);

        // Another grab type: nothing and false.
        e.set(player, PlayerCharacter::eGrabType, 1);
        e.set(player, PlayerCharacter::pGrabbedObject, Ptr::new(object));
        let log = logged(&mut e, |e| {
            assert!(!e.call(0x0096_13c0, &args![player, 0.016f32]).bool());
        });
        assert_eq!(log.len(), 1);

        // Grab type 2 without a spring creates one at scale * setting.
        e.set(player, PlayerCharacter::eGrabType, 2);
        let log = logged(&mut e, |e| {
            assert!(e.call(0x0096_13c0, &args![player, 0.016f32]).bool());
        });
        assert_eq!(calls_to(&log, FUNCTION_00877720).len(), 1);
        assert_eq!(e.get(player, PlayerCharacter::fGrabDistance), 6.0);
    }

    #[test]
    fn drop_spring_releases_or_updates_by_the_control_state() {
        let mut e = engine(CALLEES_009613C0);
        let player = e.new_object::<PlayerCharacter>();
        e.set_global(POINTER_011DEA0C, 0x4200_0000);
        e.register_double(READ_DWORD_AT_0, |e, a| int(e.mem.u32(a[0])));
        // The update path asks for the character controller; a controller
        // standing on the spring body makes it release at once.
        let controller = e.mem.alloc(0x700);
        e.mem.set_u32(controller + 0x608, 1);
        answer(&mut e, GET_CHAR_CONTROLLER, controller);
        answer(&mut e, FUNCTION_0082AEB0, 1);
        answer(&mut e, FUNCTION_00C6FD20, 1);
        let grabbed = object_with_slots(&mut e, &[(0xf0, SLOT_B)]);
        answer(&mut e, SLOT_B, 0);
        let spring = object_with_slots(&mut e, &[(0xa0, SLOT_E)]);
        answer(&mut e, SLOT_E, 0);
        e.set(player, PlayerCharacter::pGrabbedObject, Ptr::new(grabbed));
        let state = Rc::new(RefCell::new((false, false, false)));
        let held = state.clone();
        e.register_double(CONTROL_QUERY, move |_, a| {
            assert_eq!(a[1], 4);
            let (two, one, zero) = *held.borrow();
            int(u32::from(match a[2] {
                2 => two,
                1 => one,
                _ => zero,
            }))
        });
        let updated = |e: &mut Engine, flags: (bool, bool, bool)| {
            *state.borrow_mut() = flags;
            e.set(player, PlayerCharacter::eGrabType, 2);
            e.mem.set_u32(player.addr() + 0x634, spring);
            e.set(player, PlayerCharacter::pGrabbedObject, Ptr::new(grabbed));
            let log = logged(e, |e| {
                e.call(0x0096_13c0, &args![player, 0.016f32]);
            });
            !calls_to(&log, GET_CHAR_CONTROLLER).is_empty()
        };
        // Control 4 in state 2 down, or in no state down: release. State 1
        // or 0 down (and 2 up): update.
        assert!(!updated(&mut e, (true, true, true)));
        assert!(!updated(&mut e, (false, false, false)));
        assert!(updated(&mut e, (false, true, false)));
        assert!(updated(&mut e, (false, false, true)));
    }

    #[test]
    fn shader_intensity_is_applied_to_both_roots_and_the_pipboy() {
        let mut e = engine(CALLEES_00961C40);
        let player = player_with(&mut e, &[], &[(0x5b4, SLOT_A)]);
        answer_float(&mut e, SLOT_A, 0.75);
        e.register_double(PLAYER_GET_PROCESS, |e, a| int(e.mem.u32(a[0] + 0x68)));
        e.register_double(PLAYER_GET_ROOT_NODE, |_, a| int(0x8000_0000 + a[1]));
        e.register_double(ADD_44_TO_ADDRESS, |_, a| int(a[0] + 0x44));
        answer(&mut e, INTERFACE_GET_PIPBOY, 0x8800_0000);
        let zero = 0.0f32.to_bits();

        // A positive intensity is used as given.
        let log = logged(&mut e, |e| {
            e.call(0x0096_1c40, &args![player, 1u8, 0.5f32]);
        });
        assert_eq!(
            calls_to(&log, FUNCTION_00B68770),
            vec![
                vec![0x8000_0001, 1, 0.5f32.to_bits(), 0, zero, 1],
                vec![0x8000_0000, 1, 0.5f32.to_bits(), 0, zero, 1],
            ]
        );
        assert_eq!(
            calls_to(&log, FUNCTION_00422750),
            vec![vec![player.addr() + 0x44, 1, 0.5f32.to_bits()]]
        );
        assert_eq!(
            calls_to(&log, FUNCTION_007FA8D0),
            vec![vec![0x8800_0000, 1]]
        );

        // A zero intensity with the flag takes the value of the process.
        let log = logged(&mut e, |e| {
            e.call(0x0096_1c40, &args![player, 1u8, 0.0f32]);
        });
        assert_eq!(
            calls_to(&log, FUNCTION_00B68770)[0],
            vec![0x8000_0001, 1, 0.75f32.to_bits(), 0, zero, 1]
        );
        // Without the flag a non-positive intensity switches the flag off.
        let log = logged(&mut e, |e| {
            e.call(0x0096_1c40, &args![player, 0u8, -1.0f32]);
        });
        assert_eq!(
            calls_to(&log, FUNCTION_00B68770)[0],
            vec![0x8000_0001, 0, (-1.0f32).to_bits(), 0, zero, 1]
        );
    }

    #[test]
    fn death_menu_loads_or_returns_to_the_main_menu() {
        let mut e = engine(CALLEES_00961D50);
        e.mem.set_u8(FLAG_011E07C0, 1);
        e.set_global(POINTER_011DE134, 0x4300_0000);
        answer(&mut e, GET_MESSAGE_MENU_RESULT, 1);
        let log = logged(&mut e, |e| {
            e.call(0x0096_1d50, &args![]);
        });
        assert_eq!(e.mem.u8(FLAG_011E07C0), 0);
        assert_eq!(
            calls_to(&log, LOAD_MOST_RECENT_SAVE_GAME),
            vec![vec![0x4300_0000]]
        );
        assert!(calls_to(&log, CHOOSE_MAIN_MENU).is_empty());
        answer(&mut e, GET_MESSAGE_MENU_RESULT, 2);
        let log = logged(&mut e, |e| {
            e.call(0x0096_1d50, &args![]);
        });
        assert_eq!(calls_to(&log, CHOOSE_MAIN_MENU).len(), 1);
        answer(&mut e, GET_MESSAGE_MENU_RESULT, 0);
        let log = logged(&mut e, |e| {
            e.call(0x0096_1d50, &args![]);
        });
        assert_eq!(log.len(), 2);
    }

    #[test]
    fn sit_heading_delta_is_saved_or_restored() {
        let mut e = engine(CALLEES_00961D90);
        let player = e.new_object::<PlayerCharacter>();
        e.set(player, PlayerCharacter::fSitHeadingDelta, 1.25);
        let log = logged(&mut e, |e| {
            e.call(0x0096_1d90, &args![player, 1u8]);
        });
        assert_eq!(e.global::<f32>(F32_CONSTANT_011E0D60), 1.25);
        assert_eq!(
            calls_to(&log, PLAYER_STOP_VANITY_MODE),
            vec![vec![player.addr()]]
        );
        assert_eq!(
            calls_to(&log, FUNCTION_00933890),
            vec![vec![player.addr(), 1]]
        );
        e.set(player, PlayerCharacter::fSitHeadingDelta, 9.0);
        e.call(0x0096_1d90, &args![player, 0u8]);
        assert_eq!(e.get(player, PlayerCharacter::fSitHeadingDelta), 1.25);
    }

    /// The doubles shared by the spring update and the drop position
    /// searches: the float helpers `404010` (the larger of its two
    /// arguments), `0040ebd0` (the smaller) and `004587d0` (half).
    fn float_helpers(e: &mut Engine) {
        e.register_double(FUNCTION_00404010, |_, a| {
            float(f64::from(f32::from_bits(a[0]).max(f32::from_bits(a[1]))))
        });
        e.register_double(FUNCTION_0040EBD0, |_, a| {
            float(f64::from(f32::from_bits(a[0]).min(f32::from_bits(a[1]))))
        });
        e.register_double(FUNCTION_004587D0, |_, a| {
            float(f64::from(f32::from_bits(a[0]) * 0.5))
        });
        e.mem.set_f64(ZERO_DOUBLE, 0.0);
        e.mem.set_f64(DOUBLE_ONE, 1.0);
        e.mem.set_f64(DOUBLE_5, 5.0);
        e.mem.set_f64(DOUBLE_96, 96.0);
        e.mem.set_f64(DOUBLE_TWO, 2.0);
        e.mem.set_f64(DOUBLE_HALF, 0.5);
        e.mem.set_f64(DOUBLE_TWO_PI, std::f64::consts::TAU);
    }

    /// A player holding `grabbed` (virtuals +0x1d0 and +0x100) with a spring
    /// whose body the `0082aeb0` and `0082aee0` doubles answer, and a held
    /// object whose virtual +0xc8 answers the shared `hit` cell.
    /// Returns (player, grabbed, spring, hit).
    fn update_world(e: &mut Engine) -> (Ptr<PlayerCharacter>, u32, u32, Rc<std::cell::Cell<u32>>) {
        float_helpers(e);
        let player = player_with(e, &[], &[]);
        float_settings(e, &[]);
        let loaded = e.mem.alloc(0x40);
        e.mem.set_u32(player.addr() + 0x40, loaded);
        e.register_double(READ_DWORD_AT_40, |e, a| int(e.mem.u32(a[0] + 0x40)));
        e.register_double(READ_DWORD_AT_0, |e, a| int(e.mem.u32(a[0])));
        e.register_double(READ_DWORD_AT_C, |e, a| int(e.mem.u32(a[0] + 0xc)));
        e.register_double(READ_FIELD_4, |e, a| int(e.mem.u32(a[0] + 4)));
        e.register_double(IDENTITY_00460140, |_, a| int(a[0]));
        e.register_double(IDENTITY_006815C0, |_, a| int(a[0]));
        let hit = Rc::new(std::cell::Cell::new(0u32));
        let answer_hit = hit.clone();
        let held = object_with_slots(e, &[(0xc8, SLOT_B)]);
        e.register_double(SLOT_B, move |_, _| int(answer_hit.get()));
        answer(e, OBJECT_FUNCTION_004543C0, held);
        let grabbed = object_with_slots(e, &[(0x1d0, SLOT_A), (0x100, SLOT_D), (0xf0, SLOT_F)]);
        answer(e, SLOT_F, 0);
        answer(e, SLOT_A, 0x8000_0010);
        answer(e, SLOT_D, 0);
        e.set(player, PlayerCharacter::pGrabbedObject, Ptr::new(grabbed));
        e.set(player, PlayerCharacter::eGrabType, 1);
        let spring = object_with_slots(e, &[(0xa0, SLOT_E)]);
        answer(e, SLOT_E, 0);
        e.mem.set_u32(player.addr() + 0x634, spring);
        answer(e, FUNCTION_0082AEB0, 1);
        answer(e, FUNCTION_0082AEE0, 0x8600_0000);
        let entity_body = object_with_slots(e, &[(0xf4, SLOT_C)]);
        answer(e, SLOT_C, 0);
        answer(e, FUNCTION_004B5A80, entity_body);
        let block = e.mem.alloc(0x40);
        answer(e, FUNCTION_0043B4F0, block);
        (player, grabbed, spring, hit)
    }

    #[test]
    fn spring_update_releases_without_a_grabbed_object_or_a_spring_body() {
        let mut e = engine(CALLEES_00960520);
        let (player, _, _, _) = update_world(&mut e);
        e.set(player, PlayerCharacter::pGrabbedObject, Ptr::NULL);
        let log = logged(&mut e, |e| {
            e.call(0x0096_0520, &args![player, 0.016f32]);
        });
        assert_eq!(
            calls_to(&log, PLAYER_REMOVE_PLAYER_ACTION),
            vec![vec![player.addr(), 5, 0]]
        );
        assert!(calls_to(&log, HIT_COLLECTOR_NEW_004A3A70).is_empty());

        // The spring without a body is released too.
        let (player, _, _, _) = update_world(&mut e);
        answer(&mut e, FUNCTION_0082AEB0, 0);
        let log = logged(&mut e, |e| {
            e.call(0x0096_0520, &args![player, 0.016f32]);
        });
        assert_eq!(calls_to(&log, PLAYER_REMOVE_PLAYER_ACTION).len(), 1);
        assert!(calls_to(&log, GET_CHAR_CONTROLLER).is_empty());
    }

    #[test]
    fn spring_update_releases_when_the_character_stands_on_the_spring_body() {
        let mut e = engine(CALLEES_00960520);
        let (player, grabbed, _, _) = update_world(&mut e);
        let controller = e.mem.alloc(0x700);
        e.mem.set_u32(controller + 0x608, 1);
        answer(&mut e, GET_CHAR_CONTROLLER, controller);
        answer(&mut e, FUNCTION_00C6FD20, 1);
        let log = logged(&mut e, |e| {
            e.call(0x0096_0520, &args![player, 0.016f32]);
        });
        assert_eq!(calls_to(&log, PLAYER_REMOVE_PLAYER_ACTION).len(), 1);
        assert!(calls_to(&log, HIT_COLLECTOR_NEW_004A3A70).is_empty());
        // A different body: the update goes on.
        answer(&mut e, FUNCTION_00C6FD20, 2);
        e.set(player, PlayerCharacter::pGrabbedObject, Ptr::new(grabbed));
        let log = logged(&mut e, |e| {
            e.call(0x0096_0520, &args![player, 0.016f32]);
        });
        assert!(calls_to(&log, PLAYER_REMOVE_PLAYER_ACTION).is_empty());
        assert_eq!(calls_to(&log, HIT_COLLECTOR_NEW_004A3A70).len(), 1);
    }

    #[test]
    fn spring_update_moves_the_target_along_the_view_ray() {
        let mut e = engine(CALLEES_00960520);
        let (player, _, spring, _) = update_world(&mut e);
        let log = logged(&mut e, |e| {
            e.call(0x0096_0520, &args![player, 0.016f32]);
        });
        assert!(calls_to(&log, PLAYER_REMOVE_PLAYER_ACTION).is_empty());
        // The ray collector is built before the target is set and destroyed
        // afterwards.
        let order: Vec<u32> = log
            .iter()
            .map(|(address, _)| *address)
            .filter(|address| {
                [
                    HIT_COLLECTOR_NEW_004A3A70,
                    FUNCTION_008C5D90,
                    HIT_COLLECTOR_DELETE_004A3BC0,
                ]
                .contains(address)
            })
            .collect();
        assert_eq!(
            order,
            vec![
                HIT_COLLECTOR_NEW_004A3A70,
                FUNCTION_008C5D90,
                HIT_COLLECTOR_DELETE_004A3BC0
            ]
        );
        let target_calls = calls_to(&log, FUNCTION_008C5D90);
        assert_eq!(target_calls[0][0], spring);
        // The collector the ray filled is the one destroyed.
        assert_eq!(
            calls_to(&log, HIT_COLLECTOR_NEW_004A3A70)[0][0],
            calls_to(&log, HIT_COLLECTOR_DELETE_004A3BC0)[0][0]
        );
    }

    #[test]
    fn spring_update_releases_when_the_spring_is_stretched_too_far() {
        let mut e = engine(CALLEES_00960520);
        let (player, _, _, _) = update_world(&mut e);
        // The separation comes out of 004587d0 (half of 004586d0's float).
        answer_float(&mut e, FUNCTION_004586D0, 400.0);
        let log = logged(&mut e, |e| {
            e.call(0x0096_0520, &args![player, 0.016f32]);
        });
        assert_eq!(calls_to(&log, PLAYER_REMOVE_PLAYER_ACTION).len(), 1);
        assert!(calls_to(&log, FUNCTION_008C5D90).is_empty());
        assert_eq!(calls_to(&log, HIT_COLLECTOR_DELETE_004A3BC0).len(), 1);
        // Grab type 3 never releases for the distance.
        let (player, _, _, _) = update_world(&mut e);
        answer_float(&mut e, FUNCTION_004586D0, 400.0);
        let block = e.mem.alloc(0x20);
        answer(&mut e, GET_FIELD_20_OR_DEFAULT, block);
        e.set(player, PlayerCharacter::eGrabType, 3);
        let log = logged(&mut e, |e| {
            e.call(0x0096_0520, &args![player, 0.016f32]);
        });
        assert!(calls_to(&log, PLAYER_REMOVE_PLAYER_ACTION).is_empty());
    }

    #[test]
    fn spring_update_pushes_a_group_member_that_is_pulled_too_far() {
        let mut e = engine(CALLEES_00960520);
        let (player, _, _, _) = update_world(&mut e);
        // Separation 50 is below the limit 96 and above setting 011d1308
        // (10); the node is in the group (flag 8) with flag 0x80 set too.
        answer_float(&mut e, FUNCTION_004586D0, 100.0);
        answer(&mut e, FUNCTION_00624B70, 1);
        e.mem.set_f64(DOUBLE_MINUS_ONE, -1.0);
        float_settings(
            &mut e,
            &[
                (SETTING_011D1308, 10.0),
                (SETTING_011D0808, 2.0),
                (SETTING_011D04D8, 1.0),
                (SETTING_011D037C, 3.0),
            ],
        );
        answer_float(&mut e, FUNCTION_0062C300, 2.0);
        answer_float(&mut e, FUNCTION_004B3AB0, 0.5);
        let log = logged(&mut e, |e| {
            e.call(0x0096_0520, &args![player, 0.016f32]);
        });
        // excess = (50 - 10) * 2 / 50 = 1.6; strength = 1.6 * -1 * 0.5.
        let strength = calls_to(&log, FUNCTION_005DBF20);
        assert_eq!(strength.len(), 1);
        assert_eq!(f32::from_bits(strength[0][1]), (-1.6f64 * 0.5) as f32);
        // The flagged node is pushed by an impulse.
        let impulse = calls_to(&log, FUNCTION_0062B8D0);
        assert_eq!(impulse.len(), 1);
        assert_eq!(impulse[0][0], 0x8000_0010);
        assert_eq!(impulse[0][2], 0);
    }

    /// The doubles shared by the drop position searches.
    fn drop_world(e: &mut Engine) -> (Ptr<PlayerCharacter>, Rc<std::cell::Cell<u32>>, u32) {
        float_helpers(e);
        let (player, _, _, hit) = update_world(e);
        let out = e.mem.alloc(0x10);
        answer_float(e, GET_BOUND_SIZE_0050EBF0, 3.0);
        answer_float(e, FUNCTION_00C6E280, 3.0);
        answer_float(e, OBJECT_GET_SCALE, 2.0);
        float_settings(e, &[(SETTING_011D0628, 1.5)]);
        e.register_double(FUNCTION_004B4500, |e, a| {
            for (lane, value) in [1.0f32, 2.0, 3.0].into_iter().enumerate() {
                e.mem.set_f32(a[1] + 4 * lane as u32, value);
            }
            int(a[1])
        });
        let position = e.mem.alloc(0x10);
        for (lane, value) in [7.0f32, 8.0, 9.0].into_iter().enumerate() {
            e.mem.set_f32(position + 4 * lane as u32, value);
        }
        answer(e, FUNCTION_00439E90, position);
        let bodies = e.mem.alloc(0x10);
        answer(e, FUNCTION_00413F40, bodies);
        answer(e, ALLOCATE_00AA13E0, 0x8700_0000);
        let hit_record = e.mem.alloc(0x40);
        e.mem.set_f32(hit_record + 0x10, 0.5);
        answer(e, HIT_COLLECTOR_GET_004A46B0, hit_record);
        (player, hit, out)
    }

    fn vector_at(e: &Engine, address: u32) -> [f32; 3] {
        [
            e.mem.f32(address),
            e.mem.f32(address + 4),
            e.mem.f32(address + 8),
        ]
    }

    #[test]
    fn drop_position_without_a_hit_is_the_point_beyond_the_reach() {
        let mut e = engine(CALLEES_009614B0);
        let (player, _, out) = drop_world(&mut e);
        e.set(player, PlayerCharacter::fLastDropAngleMod, 0.75);
        let object = Ptr::<()>::new(0x8800_0000);
        let log = logged(&mut e, |e| {
            assert!(e
                .call(0x0096_14b0, &args![player, object, out, 0u8, 0u8])
                .bool());
        });
        // Size 3 * 2 = 6; the point is at 6 / 2 + reach (1.5 * 2) = 6.
        assert_eq!(calls_to(&log, FUNCTION_00439180)[0][1], 6.0f32.to_bits());
        assert_eq!(vector_at(&e, out), [7.0, 8.0, 9.0]);
        // Without rotate the last drop angle is reused.
        assert_eq!(calls_to(&log, FUNCTION_004A0C90)[0][1], 0.75f32.to_bits());
        // The swept sphere has half of the size as its radius.
        assert_eq!(
            calls_to(&log, SPHERE_SHAPE_NEW),
            vec![vec![0x8700_0000, 3.0f32.to_bits(), 1]]
        );
        assert_eq!(calls_to(&log, HIT_COLLECTOR_NEW_004A3A70).len(), 1);
        assert_eq!(calls_to(&log, HIT_COLLECTOR_DELETE_004A3BC0).len(), 1);
    }

    #[test]
    fn drop_position_with_a_ray_hit_stops_short_of_the_wall() {
        let mut e = engine(CALLEES_009614B0);
        let (player, hit, out) = drop_world(&mut e);
        hit.set(1);
        answer_float(&mut e, FUNCTION_00457990, 5.0);
        let object = Ptr::<()>::new(0x8800_0000);
        let log = logged(&mut e, |e| {
            assert!(e
                .call(0x0096_14b0, &args![player, object, out, 0u8, 0u8])
                .bool());
        });
        // Hit fraction 0.5 of (reach 3 + size 6) = 4.5, less half the size.
        assert_eq!(calls_to(&log, FUNCTION_00439180)[0][1], 1.5f32.to_bits());
        // The horizontal length is measured from the direction's x and y.
        let horizontal = calls_to(&log, FUNCTION_00416870);
        assert_eq!(horizontal.len(), 1);
        assert_eq!(
            horizontal[0][1..],
            [1.0f32.to_bits(), 2.0f32.to_bits(), 0.0f32.to_bits()]
        );
        // 5 - 3 = 2 clears the character radius 1.5: the spot is taken.
        assert_eq!(calls_to(&log, SPHERE_SHAPE_NEW).len(), 1);
        // A hit too close to the player does not.
        answer_float(&mut e, FUNCTION_00457990, 4.0);
        let log = logged(&mut e, |e| {
            e.call(0x0096_14b0, &args![player, object, out, 0u8, 0u8]);
        });
        assert!(calls_to(&log, SPHERE_SHAPE_NEW).is_empty());
    }

    #[test]
    fn drop_position_falls_back_to_the_players_position_when_everything_is_blocked() {
        let mut e = engine(CALLEES_009614B0);
        let (player, _, out) = drop_world(&mut e);
        // Something is always in the way of the swept sphere.
        let blocking = e.mem.alloc(0x10);
        e.mem.set_u32(blocking + 4, 1);
        answer(&mut e, FUNCTION_00413F40, blocking);
        let position = e.mem.alloc(0x10);
        for (lane, value) in [4.0f32, 5.0, 6.0].into_iter().enumerate() {
            e.mem.set_f32(position + 4 * lane as u32, value);
        }
        answer(&mut e, FUNCTION_00436AA0, position);
        e.set_global(F32_CONSTANT_0101E6EC, 64.0f32);
        let object = Ptr::<()>::new(0x8800_0000);

        // Without the search flag only the straight attempt is made.
        let log = logged(&mut e, |e| {
            assert!(!e
                .call(0x0096_14b0, &args![player, object, out, 0u8, 0u8])
                .bool());
        });
        assert_eq!(calls_to(&log, HIT_COLLECTOR_NEW_004A3A70).len(), 1);
        // With it: eleven attempts, then the player's position raised by
        // min(64, half the size = 3).
        let log = logged(&mut e, |e| {
            assert!(e
                .call(0x0096_14b0, &args![player, object, out, 1u8, 0u8])
                .bool());
        });
        assert_eq!(calls_to(&log, HIT_COLLECTOR_NEW_004A3A70).len(), 11);
        assert_eq!(calls_to(&log, HIT_COLLECTOR_DELETE_004A3BC0).len(), 11);
        assert_eq!(vector_at(&e, out), [4.0, 5.0, 9.0]);
    }

    #[test]
    fn drop_position_advances_the_drop_angle_when_rotating() {
        let mut e = engine(CALLEES_009614B0);
        let (player, _, out) = drop_world(&mut e);
        answer_float(&mut e, FUNCTION_004B5510, 0.25);
        let object = Ptr::<()>::new(0x8800_0000);
        e.set(player, PlayerCharacter::fDropAngleMod, 7.0);
        let log = logged(&mut e, |e| {
            e.call(0x0096_14b0, &args![player, object, out, 0u8, 1u8]);
        });
        // The angle in use is the first advance; it is remembered, then
        // advanced again and wrapped past a full turn.
        assert_eq!(calls_to(&log, FUNCTION_004A0C90)[0][1], 7.25f32.to_bits());
        assert_eq!(e.get(player, PlayerCharacter::fLastDropAngleMod), 7.25);
        let wrapped = (7.5f64 - std::f64::consts::TAU) as f32;
        assert_eq!(e.get(player, PlayerCharacter::fDropAngleMod), wrapped);

        // From zero the first advance is skipped.
        e.set(player, PlayerCharacter::fDropAngleMod, 0.0);
        let log = logged(&mut e, |e| {
            e.call(0x0096_14b0, &args![player, object, out, 0u8, 1u8]);
        });
        assert_eq!(calls_to(&log, FUNCTION_004A0C90)[0][1], 0.0f32.to_bits());
        assert_eq!(e.get(player, PlayerCharacter::fDropAngleMod), 0.25);
    }

    // ---- tests of the functions from 00961de0 -------------------------------

    #[test]
    fn fade_alpha_goes_to_the_first_person_root() {
        let mut e = engine(&[FUNCTION_008C4790, RECURSIVE_SET_PROPERTY_FADE_ALPHA]);
        answer(&mut e, PLAYER_GET_ROOT_NODE, 0x3000_0000);
        let player = e.new_object::<PlayerCharacter>();
        let log = logged(&mut e, |e| {
            e.call(0x0096_1de0, &args![player, 0.5f32]);
        });
        assert_eq!(
            calls_to(&log, FUNCTION_008C4790),
            [[player.addr(), 0.5f32.to_bits()]]
        );
        assert_eq!(calls_to(&log, PLAYER_GET_ROOT_NODE), [[player.addr(), 1]]);
        assert_eq!(
            calls_to(&log, RECURSIVE_SET_PROPERTY_FADE_ALPHA),
            [[0x3000_0000, 0.5f32.to_bits()]]
        );

        answer(&mut e, PLAYER_GET_ROOT_NODE, 0);
        let log = logged(&mut e, |e| {
            e.call(0x0096_1de0, &args![player, 0.5f32]);
        });
        assert!(calls_to(&log, RECURSIVE_SET_PROPERTY_FADE_ALPHA).is_empty());
    }

    /// The `Main` object, the interface manager and the extra data block the
    /// UFO camera toggle uses; the player's position is `(1, 2, 3)`.
    fn ufo_world(e: &mut Engine, flag_byte: u32) -> (Ptr<PlayerCharacter>, u32, u32) {
        let main = e.mem.alloc(0x10);
        e.set_global(POINTER_011DEA0C, main);
        answer(e, FUNCTION_005BB4D0, flag_byte);
        let manager = e.mem.alloc(0x60);
        answer(e, FUNCTION_00705950, manager);
        answer_float(e, OBJECT_GET_SCALE, 2.0);
        let extra = e.mem.alloc(0x10);
        e.mem.set_f32(extra, 0.25);
        e.mem.set_f32(extra + 8, 0.75);
        answer(e, ADD_24_TO_ADDRESS, extra);
        let position = e.mem.alloc(0x10);
        for (lane, value) in [1.0f32, 2.0, 3.0].into_iter().enumerate() {
            e.mem.set_f32(position + 4 * lane as u32, value);
        }
        answer(e, SLOT_A, position);
        let player = player_with(e, &[(0x1f4, SLOT_A)], &[]);
        e.set(player, PlayerCharacter::fEyeHeight, 10.0);
        (player, main, manager)
    }

    #[test]
    fn ufo_camera_turns_on_at_the_player() {
        let mut e = engine(&[
            FUNCTION_005BB4D0,
            FUNCTION_00705950,
            OBJECT_GET_SCALE,
            ADD_24_TO_ADDRESS,
        ]);
        let (player, main, manager) = ufo_world(&mut e, 0);
        assert_eq!(e.call(0x0096_1e30, &args![player, 1u8]).u8(), 1);
        assert_eq!(e.mem.u8(main + 6), 1);
        assert_eq!(e.mem.u8(main + 7), 1);
        assert_eq!(e.mem.u8(manager + 0x54), 1);
        assert_eq!(e.mem.f32(player.addr() + 0x7e8), 1.0);
        assert_eq!(e.mem.f32(player.addr() + 0x7ec), 2.0);
        // z = scale * eye height + the copied z
        assert_eq!(e.mem.f32(player.addr() + 0x7f0), 23.0);
        assert_eq!(e.get(player, PlayerCharacter::fUFOCameraHeading), 0.75);
        assert_eq!(e.get(player, PlayerCharacter::fUFOCameraPitch), 0.25);

        // Without `enable` the byte at +7 stays clear.
        e.mem.set_u8(main + 7, 0xaa);
        assert_eq!(e.call(0x0096_1e30, &args![player, 0u8]).u8(), 1);
        assert_eq!(e.mem.u8(main + 7), 0);
    }

    #[test]
    fn ufo_camera_turns_off_without_touching_the_camera() {
        let mut e = engine(&[
            FUNCTION_005BB4D0,
            FUNCTION_00705950,
            OBJECT_GET_SCALE,
            ADD_24_TO_ADDRESS,
        ]);
        let (player, main, manager) = ufo_world(&mut e, 1);
        e.mem.set_u8(main + 6, 1);
        e.mem.set_u8(main + 7, 1);
        assert_eq!(e.call(0x0096_1e30, &args![player, 1u8]).u8(), 0);
        assert_eq!(e.mem.u8(main + 6), 0);
        assert_eq!(e.mem.u8(main + 7), 0);
        assert_eq!(e.mem.u8(manager + 0x54), 0);
        assert_eq!(e.mem.f32(player.addr() + 0x7e8), 0.0);
    }

    #[test]
    fn single_byte_setters() {
        let mut e = engine(&[]);
        let object = e.mem.alloc(0x60);
        e.call(0x0096_1f30, &args![Ptr::<()>::new(object), 7u8]);
        e.call(0x0096_1f50, &args![Ptr::<()>::new(object), 8u8]);
        e.call(0x0096_1f70, &args![Ptr::<()>::new(object), 9u8]);
        assert_eq!(e.mem.u8(object + 0x54), 7);
        assert_eq!(e.mem.u8(object + 7), 8);
        assert_eq!(e.mem.u8(object + 6), 9);
    }

    #[test]
    fn reset_magic_cast_sound_replaces_a_valid_handle() {
        let mut e = engine(&[
            SOUND_HANDLE_IS_VALID,
            SOUND_HANDLE_NEW,
            SOUND_HANDLE_COPY,
            EMPTY_DESTRUCTOR,
        ]);
        let player = e.new_object::<PlayerCharacter>();
        e.set(player, PlayerCharacter::iSelectedSpellCastSoundID, 5);
        let log = logged(&mut e, |e| {
            e.call(0x0096_1f90, &args![player]);
        });
        // Not valid: nothing happens.
        assert_eq!(calls_to(&log, SOUND_HANDLE_COPY).len(), 0);
        assert_eq!(e.get(player, PlayerCharacter::iSelectedSpellCastSoundID), 5);

        answer(&mut e, SOUND_HANDLE_IS_VALID, 1);
        let log = logged(&mut e, |e| {
            e.call(0x0096_1f90, &args![player]);
        });
        let fresh = calls_to(&log, SOUND_HANDLE_NEW)[0][0];
        assert_eq!(
            calls_to(&log, SOUND_HANDLE_COPY),
            [[player.addr() + 0x7f8, fresh]]
        );
        assert_eq!(calls_to(&log, EMPTY_DESTRUCTOR), [[fresh]]);
        assert_eq!(e.get(player, PlayerCharacter::iSelectedSpellCastSoundID), 0);
    }

    const CALLEES_00962030: &[u32] = &[
        SOUND_HANDLE_IS_VALID,
        SOUND_HANDLE_STOP,
        SOUND_HANDLE_SET_POSITION,
        SOUND_HANDLE_SET_OBJECT_TO_FOLLOW,
        SOUND_HANDLE_PLAY,
        SOUND_HANDLE_COPY,
        EMPTY_DESTRUCTOR,
        FUNCTION_0040DEE0,
        READ_DWORD_AT_C,
        FUNCTION_00933150,
    ];

    #[test]
    fn magic_failure_sound_restarts_a_valid_handle() {
        let mut e = engine(CALLEES_00962030);
        answer(&mut e, SOUND_HANDLE_IS_VALID, 1);
        let position = e.mem.alloc(0x10);
        e.mem.set_f32(position, 1.5);
        e.mem.set_f32(position + 4, 2.5);
        e.mem.set_f32(position + 8, 3.5);
        answer(&mut e, SLOT_A, position);
        answer(&mut e, SLOT_B, 0x6000_0000);
        let player = player_with(&mut e, &[(0x1f4, SLOT_A), (0x1d0, SLOT_B)], &[]);
        let handle = player.addr() + 0x804 + 2 * 0xc;
        let log = logged(&mut e, |e| {
            e.call(0x0096_2030, &args![player, 2u32]);
        });
        let order: Vec<u32> = log
            .iter()
            .map(|(target, _)| *target)
            .filter(|target| CALLEES_00962030.contains(target))
            .collect();
        assert_eq!(
            order,
            [
                SOUND_HANDLE_IS_VALID,
                SOUND_HANDLE_STOP,
                SOUND_HANDLE_SET_POSITION,
                SOUND_HANDLE_SET_OBJECT_TO_FOLLOW,
                SOUND_HANDLE_PLAY
            ]
        );
        assert_eq!(
            calls_to(&log, SOUND_HANDLE_SET_POSITION),
            [[handle, 1.5f32.to_bits(), 2.5f32.to_bits(), 3.5f32.to_bits()]]
        );
        assert_eq!(
            calls_to(&log, SOUND_HANDLE_SET_OBJECT_TO_FOLLOW),
            [[handle, 0x6000_0000]]
        );
        assert_eq!(calls_to(&log, SOUND_HANDLE_PLAY), [[handle, 0]]);
    }

    #[test]
    fn magic_failure_sound_creates_the_sound_when_the_handle_is_not_valid() {
        let mut e = engine(CALLEES_00962030);
        let player = e.new_object::<PlayerCharacter>();
        // No sound form for the index: nothing is created.
        let log = logged(&mut e, |e| {
            e.call(0x0096_2030, &args![player, 1u32]);
        });
        assert!(calls_to(&log, FUNCTION_00933150).is_empty());

        answer(&mut e, FUNCTION_0040DEE0, 0x6100_0000);
        answer(&mut e, READ_DWORD_AT_C, 0x1234);
        answer(&mut e, FUNCTION_00933150, 0x6200_0000);
        let log = logged(&mut e, |e| {
            e.call(0x0096_2030, &args![player, 1u32]);
        });
        assert_eq!(calls_to(&log, FUNCTION_0040DEE0), [[1]]);
        let created = calls_to(&log, FUNCTION_00933150);
        assert_eq!(created.len(), 1);
        let out = created[0][1];
        assert_eq!(created[0], [player.addr(), out, 0x1234, 0, 2, 1]);
        assert_eq!(
            calls_to(&log, SOUND_HANDLE_COPY),
            [[player.addr() + 0x804 + 0xc, 0x6200_0000]]
        );
        assert_eq!(calls_to(&log, EMPTY_DESTRUCTOR), [[out]]);
    }

    const CALLEES_00962190: &[u32] = &[FUNCTION_00971C30, LIST_CLEAR_ITEMS, LIST_DELETE];

    #[test]
    fn being_chased_follows_the_process_list() {
        let mut e = engine(CALLEES_00962190);
        list_accessors(&mut e);
        answer(&mut e, SLOT_A, 1);
        answer(&mut e, SLOT_B, 1);
        let first = object_with_slots(&mut e, &[(0x100, SLOT_A), (0x304, SLOT_B)]);
        let list = list_of(&mut e, &[first]);
        answer(&mut e, FUNCTION_00971C30, list);
        let player = player_with(&mut e, &[], &[]);
        let log = logged(&mut e, |e| {
            e.call(0x0096_2190, &args![player]);
        });
        assert_eq!(e.get(player, PlayerCharacter::bBeingChased), 1);
        assert_eq!(
            calls_to(&log, FUNCTION_00971C30),
            [[PROCESS_LISTS, player.addr(), 0x12, 0]]
        );
        assert_eq!(calls_to(&log, LIST_CLEAR_ITEMS), [[list]]);
        assert_eq!(calls_to(&log, LIST_DELETE), [[list, 1]]);

        // The second virtual says no: the flag stays clear.
        answer(&mut e, SLOT_B, 0);
        e.call(0x0096_2190, &args![player]);
        assert_eq!(e.get(player, PlayerCharacter::bBeingChased), 0);

        // No list: nothing to clear.
        answer(&mut e, FUNCTION_00971C30, 0);
        let log = logged(&mut e, |e| {
            e.call(0x0096_2190, &args![player]);
        });
        assert!(calls_to(&log, LIST_CLEAR_ITEMS).is_empty());
    }

    const CALLEES_00962290: &[u32] = &[
        VECTOR_SUBTRACT,
        VECTOR_LENGTH,
        FLOAT_FUNCTION_00648A80,
        READ_FIELD_8,
        FUNCTION_005D2860,
        FUNCTION_005299A0,
    ];

    #[test]
    fn impact_shader_triggers_for_a_positive_distance() {
        let mut e = engine(CALLEES_00962290);
        let position = e.mem.alloc(0x10);
        answer(&mut e, SLOT_A, position);
        answer(&mut e, SLOT_B, 0x6300_0000);
        let reference = object_with_slots(&mut e, &[(0x1d0, SLOT_B), (0x1f4, SLOT_A)]);
        let player = player_with(&mut e, &[(0x1f4, SLOT_A)], &[]);
        answer_float(&mut e, VECTOR_LENGTH, 4.0);
        answer_float(&mut e, FLOAT_FUNCTION_00648A80, 2.0);
        answer(&mut e, FUNCTION_005D2860, 0x6400_0000);
        let log = logged(&mut e, |e| {
            e.call(0x0096_2290, &args![player, Ptr::<()>::new(reference)]);
        });
        assert_eq!(
            calls_to(&log, FLOAT_FUNCTION_00648A80),
            [[4.0f32.to_bits()]]
        );
        assert_eq!(
            calls_to(&log, FUNCTION_005D2860),
            [[2.0f32.to_bits(), 0x6300_0000]]
        );
        assert_eq!(
            calls_to(&log, FUNCTION_005299A0),
            [[0x6400_0000, 2.0f32.to_bits(), 0x6300_0000]]
        );
    }

    #[test]
    fn impact_shader_stops_early() {
        let mut e = engine(CALLEES_00962290);
        let position = e.mem.alloc(0x10);
        answer(&mut e, SLOT_A, position);
        answer(&mut e, SLOT_B, 0x6300_0000);
        let reference = object_with_slots(&mut e, &[(0x1d0, SLOT_B), (0x1f4, SLOT_A)]);
        let player = player_with(&mut e, &[(0x1f4, SLOT_A)], &[]);
        answer_float(&mut e, FLOAT_FUNCTION_00648A80, 2.0);

        // No reference.
        let log = logged(&mut e, |e| {
            e.call(0x0096_2290, &args![player, 0u32]);
        });
        assert_eq!(log.len(), 1);

        // Zero distance.
        answer_float(&mut e, FLOAT_FUNCTION_00648A80, 0.0);
        let log = logged(&mut e, |e| {
            e.call(0x0096_2290, &args![player, Ptr::<()>::new(reference)]);
        });
        assert!(calls_to(&log, FUNCTION_005299A0).is_empty());

        // A value in the object at 011f2250 blocks the shader.
        answer_float(&mut e, FLOAT_FUNCTION_00648A80, 2.0);
        answer(&mut e, READ_FIELD_8, 1);
        let log = logged(&mut e, |e| {
            e.call(0x0096_2290, &args![player, Ptr::<()>::new(reference)]);
        });
        assert!(calls_to(&log, FUNCTION_005299A0).is_empty());

        // A reference without 3D.
        answer(&mut e, READ_FIELD_8, 0);
        answer(&mut e, SLOT_B, 0);
        let log = logged(&mut e, |e| {
            e.call(0x0096_2290, &args![player, Ptr::<()>::new(reference)]);
        });
        assert!(calls_to(&log, VECTOR_SUBTRACT).is_empty());
    }

    #[test]
    fn dropped_references_are_released_to_havok() {
        let mut e = engine(&[BHK_WORLD_SET_MOTION, BHK_WORLD_ACTIVATE, FUNCTION_00954910]);
        list_accessors(&mut e);
        list_remove_unlinks(&mut e);
        answer(&mut e, SLOT_A, 0x6500_0000);
        let with_3d = object_with_slots(&mut e, &[(0x1d0, SLOT_A)]);
        answer(&mut e, SLOT_B, 0);
        let without_3d = object_with_slots(&mut e, &[(0x1d0, SLOT_B)]);
        let player = e.new_object::<PlayerCharacter>();
        // The list head is stored inline at +0x84c.
        let second = list_of(&mut e, &[without_3d]);
        e.mem.set_u32(player.addr() + 0x84c, with_3d);
        e.mem.set_u32(player.addr() + 0x850, second);
        let log = logged(&mut e, |e| {
            e.call(0x0096_2350, &args![player]);
        });
        assert_eq!(
            calls_to(&log, BHK_WORLD_SET_MOTION),
            [[0x6500_0000, 1, 1, 1, 1]]
        );
        assert_eq!(calls_to(&log, BHK_WORLD_ACTIVATE), [[0x6500_0000, 1, 1, 0]]);
        assert_eq!(calls_to(&log, FUNCTION_00954910), [[with_3d, 0]]);
        assert_eq!(calls_to(&log, LIST_REMOVE).len(), 1);

        // An empty list does nothing.
        let empty = e.new_object::<PlayerCharacter>();
        let log = logged(&mut e, |e| {
            e.call(0x0096_2350, &args![empty]);
        });
        assert_eq!(log.len(), 2);
    }

    #[test]
    fn camera_caster_gets_the_controller_value() {
        let mut e = engine(&[GET_CHAR_CONTROLLER, FUNCTION_00819250, FUNCTION_00620BA0]);
        let player = e.new_object::<PlayerCharacter>();
        e.set(
            player,
            PlayerCharacter::pCameraCaster,
            Ptr::new(0x6600_0000),
        );
        let log = logged(&mut e, |e| {
            e.call(0x0096_2450, &args![player]);
        });
        assert!(calls_to(&log, FUNCTION_00620BA0).is_empty());

        answer(&mut e, GET_CHAR_CONTROLLER, 0x6700_0000);
        answer(&mut e, FUNCTION_00819250, 0x6800_0000);
        let log = logged(&mut e, |e| {
            e.call(0x0096_2450, &args![player]);
        });
        assert_eq!(calls_to(&log, FUNCTION_00819250), [[0x6700_0000]]);
        assert_eq!(
            calls_to(&log, FUNCTION_00620BA0),
            [[0x6600_0000, 0x6800_0000]]
        );
    }

    #[test]
    fn light_assignment_picks_the_slot() {
        let mut e = engine(&[SMART_POINTER_ASSIGN]);
        let player = e.new_object::<PlayerCharacter>();
        let log = logged(&mut e, |e| {
            e.call(0x0096_2490, &args![player, 0x6900_0000u32, 1u8]);
            e.call(0x0096_2490, &args![player, 0x6a00_0000u32, 0u8]);
        });
        assert_eq!(
            calls_to(&log, SMART_POINTER_ASSIGN),
            [
                [player.addr() + 0x864, 0x6900_0000],
                [player.addr() + 0x868, 0x6a00_0000]
            ]
        );
    }

    const CALLEES_009624D0: &[u32] = &[
        ADD_44_TO_ADDRESS,
        FUNCTION_00418250,
        FUNCTION_004B0460,
        PLAYER_GET_ROOT_NODE,
        FUNCTION_00456610,
        FUNCTION_006E5CC0,
        FUNCTION_0088B4E0,
    ];

    #[test]
    fn light_choice_follows_the_extra_data() {
        let mut e = engine(CALLEES_009624D0);
        let player = e.new_object::<PlayerCharacter>();
        let first = player.addr() + 0x864;
        let third = player.addr() + 0x868;

        // No extra data object: nothing at all.
        let log = logged(&mut e, |e| {
            e.call(0x0096_24d0, &args![player]);
        });
        assert!(calls_to(&log, FUNCTION_0088B4E0).is_empty());

        answer(&mut e, FUNCTION_00418250, 0x6b00_0000);
        answer(&mut e, FUNCTION_00456610, 1);
        // The first light is set and the first person root is usable.
        e.register_double(FUNCTION_004B0460, move |_, a| int(u32::from(a[1] == first)));
        let log = logged(&mut e, |e| {
            e.call(0x0096_24d0, &args![player]);
        });
        assert_eq!(calls_to(&log, FUNCTION_006E5CC0), [[0x6b00_0000, third]]);
        assert_eq!(calls_to(&log, FUNCTION_0088B4E0), [[player.addr()]]);

        // The third light is set and the third person root is usable.
        e.register_double(FUNCTION_004B0460, move |_, a| int(u32::from(a[1] == third)));
        let log = logged(&mut e, |e| {
            e.call(0x0096_24d0, &args![player]);
        });
        assert_eq!(calls_to(&log, FUNCTION_006E5CC0), [[0x6b00_0000, first]]);
        assert_eq!(calls_to(&log, PLAYER_GET_ROOT_NODE).last().unwrap()[1], 0);

        // The root is not usable: no assignment.
        answer(&mut e, FUNCTION_00456610, 0);
        let log = logged(&mut e, |e| {
            e.call(0x0096_24d0, &args![player]);
        });
        assert!(calls_to(&log, FUNCTION_006E5CC0).is_empty());
        assert_eq!(calls_to(&log, FUNCTION_0088B4E0).len(), 1);
    }

    #[test]
    fn process_object_form_id_is_sent_when_the_count_is_positive() {
        let mut e = engine(&[
            FUNCTION_00570F60,
            PLAYER_GET_PROCESS,
            READ_FIELD_8,
            FUNCTION_008CE180,
        ]);
        let player = e.new_object::<PlayerCharacter>();
        answer(&mut e, SLOT_A, 0x6c00_0000);
        let process = object_with_slots(&mut e, &[(0x14c, SLOT_A)]);
        answer(&mut e, PLAYER_GET_PROCESS, process);
        answer(&mut e, READ_FIELD_8, 0x77);

        // Count not positive.
        let log = logged(&mut e, |e| {
            e.call(0x0096_2590, &args![player]);
        });
        assert!(calls_to(&log, FUNCTION_008CE180).is_empty());

        answer(&mut e, FUNCTION_00570F60, 2);
        let log = logged(&mut e, |e| {
            e.call(0x0096_2590, &args![player]);
        });
        assert_eq!(calls_to(&log, READ_FIELD_8), [[0x6c00_0000]]);
        assert_eq!(
            calls_to(&log, FUNCTION_008CE180),
            [[0x77, 0x7fff_ffff, player.addr(), 1, 1]]
        );

        // No object from the virtual.
        answer(&mut e, SLOT_A, 0);
        let log = logged(&mut e, |e| {
            e.call(0x0096_2590, &args![player]);
        });
        assert!(calls_to(&log, FUNCTION_008CE180).is_empty());

        // No form id.
        answer(&mut e, SLOT_A, 0x6c00_0000);
        answer(&mut e, READ_FIELD_8, 0);
        let log = logged(&mut e, |e| {
            e.call(0x0096_2590, &args![player]);
        });
        assert!(calls_to(&log, FUNCTION_008CE180).is_empty());
    }

    const CALLEES_00962620: &[u32] = &[
        ADD_44_TO_ADDRESS,
        FUNCTION_00422700,
        PLAYER_GET_PROCESS,
        FUNCTION_009344A0,
        FUNCTION_0041CA90,
    ];

    /// An extra data object whose list at +0xc holds `items`; the player is
    /// the owner every item's virtual `+0x2c8` answers.
    fn extra_list_world(e: &mut Engine, items: &[u32]) -> (Ptr<PlayerCharacter>, u32, u32) {
        list_accessors(e);
        list_remove_unlinks(e);
        let player = player_with(e, &[], &[]);
        let list = list_of(e, items);
        let extra = e.mem.alloc(0x20);
        e.mem.set_u32(extra + 0xc, list);
        answer(e, FUNCTION_00422700, extra);
        (player, extra, list)
    }

    #[test]
    fn qualifying_extra_items_are_counted_and_the_rest_removed() {
        let mut e = engine(CALLEES_00962620);
        answer(&mut e, SLOT_A, 0);
        let good = object_with_slots(&mut e, &[(0x2c8, SLOT_A)]);
        let bad = object_with_slots(&mut e, &[(0x2c8, SLOT_B)]);
        let (player, _, list) = extra_list_world(&mut e, &[good, bad, good]);
        e.set_global(PLAYER_CHARACTER, player.addr());
        answer(&mut e, SLOT_A, player.addr());
        answer(&mut e, SLOT_B, 0x1234);
        answer(&mut e, PLAYER_GET_PROCESS, 1);
        answer(&mut e, FUNCTION_009344A0, 0x6d00_0000);
        answer(&mut e, FUNCTION_0041CA90, 1);
        let log = logged(&mut e, |e| {
            assert_eq!(e.call(0x0096_2620, &args![player]).u32(), 2);
        });
        // `bad` was removed once and the count restarted.
        assert_eq!(calls_to(&log, LIST_REMOVE).len(), 1);
        assert_eq!(calls_to(&log, LIST_REMOVE)[0][0], list);
    }

    #[test]
    fn extra_items_need_a_process_and_a_type_one_package() {
        // (package type, process): only a type one package and a process pass.
        for (package_type, process, expected) in [(2, 1, 0), (1, 1, 1), (1, 0, 0)] {
            let mut e = engine(CALLEES_00962620);
            answer(&mut e, SLOT_A, 0);
            let item = object_with_slots(&mut e, &[(0x2c8, SLOT_A)]);
            let (player, _, _) = extra_list_world(&mut e, &[item]);
            answer(&mut e, SLOT_A, player.addr());
            answer(&mut e, PLAYER_GET_PROCESS, process);
            answer(&mut e, FUNCTION_009344A0, 0x6d00_0000);
            answer(&mut e, FUNCTION_0041CA90, package_type);
            assert_eq!(e.call(0x0096_2620, &args![player]).u32(), expected);
        }

        // No extra data object at all.
        let mut e = engine(CALLEES_00962620);
        let (player, _, _) = extra_list_world(&mut e, &[]);
        answer(&mut e, FUNCTION_00422700, 0);
        assert_eq!(e.call(0x0096_2620, &args![player]).u32(), 0);
    }

    #[test]
    fn extra_list_membership() {
        let mut e = engine(&[ADD_44_TO_ADDRESS, FUNCTION_00422700]);
        let (player, _, _) = extra_list_world(&mut e, &[0x6e00_0001, 0x6e00_0002]);
        e.set_global(PLAYER_CHARACTER, player.addr());
        assert!(e.call(0x0096_2720, &args![player, 0x6e00_0002u32]).bool());
        assert!(!e.call(0x0096_2720, &args![player, 0x6e00_0003u32]).bool());
        answer(&mut e, FUNCTION_00422700, 0);
        assert!(!e.call(0x0096_2720, &args![player, 0x6e00_0002u32]).bool());
    }

    #[test]
    fn insufficient_charge_message_is_shown_once() {
        let mut e = engine(&[SETTING_STRING, FUNCTION_007052F0]);
        e.set_global(MESSAGE_DURATION_010162C0, 3.0f32);
        answer(&mut e, SETTING_STRING, 0x6f00_0000);
        let player = e.new_object::<PlayerCharacter>();
        let log = logged(&mut e, |e| {
            e.call(0x0096_27a0, &args![player]);
            e.call(0x0096_27a0, &args![player]);
        });
        assert_eq!(calls_to(&log, SETTING_STRING), [[SETTING_011D3048]]);
        assert_eq!(
            calls_to(&log, FUNCTION_007052F0),
            [[0x6f00_0000, 0, 0, 0, 3.0f32.to_bits(), 0]]
        );
        assert_eq!(
            e.get(player, PlayerCharacter::bInsufficientChargeMessageShown),
            1
        );
        e.call(0x0096_27f0, &args![player]);
        assert_eq!(
            e.get(player, PlayerCharacter::bInsufficientChargeMessageShown),
            0
        );
    }

    #[test]
    fn random_door_space_map_lookup_and_store() {
        let mut e = engine(&[READ_DWORD_AT_C, FUNCTION_0057C850, FUNCTION_0084D310]);
        let player = e.new_object::<PlayerCharacter>();
        answer(&mut e, READ_DWORD_AT_C, 0x42);
        e.register_double(FUNCTION_0057C850, |e, a| {
            e.mem.set_u8(a[2], 9);
            int(1)
        });
        let form = Ptr::<()>::new(0x7100_0000);
        let log = logged(&mut e, |e| {
            assert_eq!(e.call(0x0096_2810, &args![player, form]).u8(), 9);
        });
        assert_eq!(
            calls_to(&log, FUNCTION_0057C850)[0][..2],
            [player.addr() + 0x854, 0x42]
        );
        // No form: 0xff and no lookup.
        let log = logged(&mut e, |e| {
            assert_eq!(e.call(0x0096_2810, &args![player, 0u32]).u8(), 0xff);
        });
        assert!(calls_to(&log, FUNCTION_0057C850).is_empty());

        let log = logged(&mut e, |e| {
            e.call(0x0096_2850, &args![player, form, 5u8]);
            e.call(0x0096_2850, &args![player, 0u32, 5u8]);
        });
        assert_eq!(
            calls_to(&log, FUNCTION_0084D310),
            [[player.addr() + 0x854, 0x42, 5]]
        );
    }

    #[test]
    fn extra_items_process_virtual_runs_for_each_item() {
        let mut e = engine(&[ADD_44_TO_ADDRESS, FUNCTION_00422700, PLAYER_GET_PROCESS]);
        answer(&mut e, SLOT_A, 1);
        let process = object_with_slots(&mut e, &[(0x24, SLOT_A)]);
        let (player, _, _) = extra_list_world(&mut e, &[0x7200_0001, 0x7200_0002]);
        e.set_global(PLAYER_CHARACTER, player.addr());
        answer(&mut e, PLAYER_GET_PROCESS, process);
        let log = logged(&mut e, |e| {
            e.call(0x0096_2880, &args![player]);
        });
        assert_eq!(
            calls_to(&log, SLOT_A),
            [[process, 0x7200_0001, 1], [process, 0x7200_0002, 1]]
        );

        // Items without a process are skipped.
        answer(&mut e, PLAYER_GET_PROCESS, 0);
        let log = logged(&mut e, |e| {
            e.call(0x0096_2880, &args![player]);
        });
        assert!(calls_to(&log, SLOT_A).is_empty());
    }

    /// Double for `LIST_REMOVE` that unlinks the node holding the item in the
    /// cell, the way `BSSimpleList::Remove` does (the head takes over the
    /// next node).
    fn list_remove_unlinks(e: &mut Engine) {
        e.register_double(LIST_REMOVE, |e, a| {
            let wanted = e.mem.u32(a[1]);
            let mut previous = 0;
            let mut node = a[0];
            while node != 0 {
                if e.mem.u32(node) == wanted {
                    let next = e.mem.u32(node + 4);
                    if previous != 0 {
                        e.mem.set_u32(previous + 4, next);
                    } else if next != 0 {
                        let item = e.mem.u32(next);
                        let after = e.mem.u32(next + 4);
                        e.mem.set_u32(node, item);
                        e.mem.set_u32(node + 4, after);
                    } else {
                        e.mem.set_u32(node, 0);
                    }
                    return int(1);
                }
                previous = node;
                node = e.mem.u32(node + 4);
            }
            int(0)
        });
    }

    const CALLEES_00962950: &[u32] = &[
        FUNCTION_00632D20,
        FUNCTION_00458B30,
        NI_POINT3_NEW,
        ACTOR_GET_HEIGHT,
        FUNCTION_00705FC0,
        VECTOR_ADD,
        PICK_OBJECTS,
        PICKER_RESULTS,
        PICK_RESULT_GET,
        PICK_RESULT_NORMAL,
        VECTOR_DOT,
        FUNCTION_00458B50,
        FUNCTION_009611E0,
        FUNCTION_0045BAD0,
        PICKER_CLEAR,
        GET_CHAR_CONTROLLER,
        CHECK_INSIDE_OF_OBJECT,
    ];

    /// A player at `(1, 2, 3)` and the doubles of the probe: the point
    /// constructor stores its components, a hit has the normal `(1, 0, 0)`
    /// and a dot product of `dot`.
    fn picker_world(e: &mut Engine, dot: f64) -> Ptr<PlayerCharacter> {
        e.register_double(NI_POINT3_NEW, |e, a| {
            for lane in 0..3 {
                e.mem.set_u32(a[0] + 4 * lane, a[1 + lane as usize]);
            }
            int(a[0])
        });
        e.register_double(VECTOR_ADD, |_, a| int(a[1]));
        answer_float(e, ACTOR_GET_HEIGHT, 2.0);
        e.set_global(HEIGHT_SCALE_010290B0, 0.25f64);
        e.set_global(PICK_DISTANCE_0104E0E8, 24.0f32);
        e.set_global(PICK_DISTANCE_0108B3F0, -24.0f32);
        let normal = e.mem.alloc(0x10);
        e.mem.set_f32(normal, 1.0);
        answer(e, PICK_RESULT_NORMAL, normal);
        answer_float(e, VECTOR_DOT, dot);
        let position = e.mem.alloc(0x10);
        for (lane, value) in [1.0f32, 2.0, 3.0].into_iter().enumerate() {
            e.mem.set_f32(position + 4 * lane as u32, value);
        }
        answer(e, SLOT_A, position);
        player_with(e, &[(0x1f4, SLOT_A)], &[])
    }

    #[test]
    fn probe_of_a_null_target_does_nothing() {
        let mut e = engine(CALLEES_00962950);
        let player = picker_world(&mut e, 1.0);
        let log = logged(&mut e, |e| {
            assert!(!e.call(0x0096_2950, &args![player, 0u32]).bool());
        });
        assert_eq!(log.len(), 1);
    }

    #[test]
    fn probe_sets_the_picker_up_once_and_hits_with_the_first_ray() {
        let mut e = engine(CALLEES_00962950);
        let player = picker_world(&mut e, 1.0);
        answer(&mut e, PICK_OBJECTS, 1);
        let origin_z = Rc::new(Cell::new(0.0f32));
        let seen = origin_z.clone();
        e.register_double(VECTOR_ADD, move |e, a| {
            seen.set(e.mem.f32(a[0] + 8));
            int(a[1])
        });
        let log = logged(&mut e, |e| {
            assert!(e.call(0x0096_2950, &args![player, 0x7d00_0000u32]).bool());
        });
        assert_eq!(e.global::<u32>(PICKER_POINTER), PICKER_OBJECT);
        assert_eq!(e.mem.u8(PICKER_OBJECT + 0x31), 1);
        assert_eq!(calls_to(&log, FUNCTION_00632D20), [[PICKER_OBJECT, 0]]);
        assert_eq!(calls_to(&log, FUNCTION_00458B30), [[PICKER_OBJECT, 1]]);
        // One ray, from (1, 2, 3 + height * 0.25) along (0, 24, 0).
        let rays = calls_to(&log, PICK_OBJECTS);
        assert_eq!(rays.len(), 1);
        assert_eq!(origin_z.get(), 3.5);
        assert_eq!(calls_to(&log, VECTOR_ADD)[0][2], rays[0][2]);
        assert_eq!(
            calls_to(&log, NI_POINT3_NEW),
            [[rays[0][2], 0, 24.0f32.to_bits(), 0]]
        );
        assert_eq!(
            calls_to(&log, FUNCTION_00705FC0),
            [[PICKER_OBJECT, 0x7d00_0000], [PICKER_OBJECT, 0]]
        );
        assert_eq!(calls_to(&log, PICKER_CLEAR), [[PICKER_OBJECT, 0]]);
        assert!(calls_to(&log, CHECK_INSIDE_OF_OBJECT).is_empty());
        assert_eq!(calls_to(&log, FUNCTION_0045BAD0)[0][0], PICK_FILTER);

        // The second probe does not set the picker up again.
        let log = logged(&mut e, |e| {
            e.call(0x0096_2950, &args![player, 0x7d00_0000u32]);
        });
        assert!(calls_to(&log, FUNCTION_00632D20).is_empty());
    }

    #[test]
    fn probe_tries_three_rays_then_asks_the_controller() {
        let mut e = engine(CALLEES_00962950);
        let player = picker_world(&mut e, 1.0);
        // Every pick hits, but the filter accepts the hit object: no blocking.
        answer(&mut e, PICK_OBJECTS, 1);
        answer(&mut e, FUNCTION_0045BAD0, 1);
        answer(&mut e, CHECK_INSIDE_OF_OBJECT, 1);
        answer(&mut e, GET_CHAR_CONTROLLER, 0x7e00_0000);
        let log = logged(&mut e, |e| {
            assert!(e.call(0x0096_2950, &args![player, 0x7d00_0000u32]).bool());
        });
        assert_eq!(calls_to(&log, PICK_OBJECTS).len(), 3);
        let made = calls_to(&log, NI_POINT3_NEW);
        assert_eq!(made.len(), 3);
        assert_eq!(made[1][1..], [24.0f32.to_bits(), (-24.0f32).to_bits(), 0]);
        assert_eq!(
            made[2][1..],
            [(-24.0f32).to_bits(), (-24.0f32).to_bits(), 0]
        );
        assert_eq!(calls_to(&log, CHECK_INSIDE_OF_OBJECT), [[0x7e00_0000]]);

        // A hit whose normal faces away does not count either.
        answer(&mut e, FUNCTION_0045BAD0, 0);
        answer_float(&mut e, VECTOR_DOT, -1.0);
        answer(&mut e, CHECK_INSIDE_OF_OBJECT, 0);
        assert!(!e.call(0x0096_2950, &args![player, 0x7d00_0000u32]).bool());
    }

    #[test]
    fn probe_stops_at_the_second_ray_that_blocks() {
        let mut e = engine(CALLEES_00962950);
        let player = picker_world(&mut e, 1.0);
        let picks = Rc::new(RefCell::new(0u32));
        let counter = picks.clone();
        e.register_double(PICK_OBJECTS, move |_, _| {
            *counter.borrow_mut() += 1;
            int(u32::from(*counter.borrow() == 2))
        });
        assert!(e.call(0x0096_2950, &args![player, 0x7d00_0000u32]).bool());
        assert_eq!(*picks.borrow(), 2);
    }

    #[test]
    fn picker_byte_and_list_removal() {
        let mut e = engine(&[]);
        list_remove_unlinks(&mut e);
        let object = e.mem.alloc(0x40);
        e.call(0x0096_2cb0, &args![Ptr::<()>::new(object), 1u8]);
        assert_eq!(e.mem.u8(object + 0x31), 1);

        let log = logged(&mut e, |e| {
            e.call(0x0096_2cd0, &args![Ptr::<()>::new(object), 0x7f00_0000u32]);
            e.call(0x0096_2cd0, &args![Ptr::<()>::new(object), 0u32]);
        });
        let removals = calls_to(&log, LIST_REMOVE);
        assert_eq!(removals.len(), 1);
        assert_eq!(removals[0][0], PICKER_LIST);
    }

    #[test]
    fn picker_list_drops_entries_that_fail() {
        let mut e = engine(CALLEES_00962950);
        let player = picker_world(&mut e, 1.0);
        list_accessors(&mut e);
        list_remove_unlinks(&mut e);
        e.register_double(FUNCTION_00450F90, |_, _| Ret::default());
        e.register_double(FUNCTION_00C6A0B0, |_, _| Ret::default());
        // The probe finds the controller inside the object for one root only.
        let last_root = Rc::new(Cell::new(0u32));
        let remembered = last_root.clone();
        e.register_double(FUNCTION_00705FC0, move |_, a| {
            if a[1] != 0 {
                remembered.set(a[1]);
            }
            Ret::default()
        });
        e.register_double(CHECK_INSIDE_OF_OBJECT, move |_, _| {
            int(u32::from(last_root.get() == 0x8200_0000))
        });
        answer(&mut e, SLOT_B, 0);
        answer(&mut e, SLOT_C, 0x8100_0000);
        answer(&mut e, SLOT_D, 0x8200_0000);
        let no_3d = object_with_slots(&mut e, &[(0x1d0, SLOT_B)]);
        let failing = object_with_slots(&mut e, &[(0x1d0, SLOT_C)]);
        let passing = object_with_slots(&mut e, &[(0x1d0, SLOT_D)]);
        let rest = list_of(&mut e, &[failing, passing]);
        e.mem.set_u32(PICKER_LIST, no_3d);
        e.mem.set_u32(PICKER_LIST + 4, rest);
        let log = logged(&mut e, |e| {
            e.call(0x0096_2d00, &args![player]);
        });
        assert_eq!(calls_to(&log, LIST_REMOVE).len(), 2);
        assert_eq!(calls_to(&log, FUNCTION_00450F90), [[0x8100_0000, 0]]);
        assert_eq!(calls_to(&log, FUNCTION_00C6A0B0), [[0x8100_0000, 1, 1, 0]]);
        // Only the entry that passed is left.
        assert_eq!(e.mem.u32(PICKER_LIST), passing);
        assert_eq!(e.mem.u32(PICKER_LIST + 4), 0);

        // An emptied list stops at once.
        e.mem.set_u32(PICKER_LIST, 0);
        let log = logged(&mut e, |e| {
            e.call(0x0096_2d00, &args![player]);
        });
        assert_eq!(log.len(), 2);
    }

    // ---- gun wobble ---------------------------------------------------------

    const CALLEES_00962DE0: &[u32] = &[
        ACTOR_GET_CURRENT_WEAPON,
        ACTOR_GET_ANIMATION,
        WEAPON_IS_MELEE_TYPE,
        GET_BYTE_AT_F4,
        READ_FLOAT_AT_D0,
        GUN_WOBBLE_GET_MATRIX,
        PLAYER_GET_ROOT_NODE,
        FIND_NODE_BY_NAME,
        FUNCTION_00965620,
        READ_FLOAT_AT_C,
        SETTING_FLOAT,
        MATRIX_FROM_EULER,
        MATRIX_TO_EULER,
        MATRIX_MULTIPLY,
        FUNCTION_0043FCD0,
        FUNCTION_008B0DD0,
        FUNCTION_00894900,
        FLOAT_FUNCTION_00408840,
        FUNCTION_006838B0,
        ALLOCATE_00AA13E0,
        FUNCTION_00C8FFD0,
        FUNCTION_00C90090,
        FUNCTION_0062BC90,
        NI_OBJECT_CAST,
        FUNCTION_004F0110,
        ACTOR_GET_IRON_SIGHTS,
        FUNCTION_00504E60,
        FUNCTION_0048CEE0,
        FUNCTION_004AD030,
        ITEM_CHANGE_HAS_MOD_EFFECT,
        FUNCTION_00931E50,
        FUNCTION_00931D30,
    ];

    /// The doubles of the wobble: ranged weapon of type 3 (wobble index 7),
    /// the matrix-to-angles double answers `(0.1, 0.2, 0.3)`.
    fn wobble_world(e: &mut Engine) -> Ptr<PlayerCharacter> {
        for (lane, value) in [1.0f32, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0]
            .into_iter()
            .enumerate()
        {
            e.mem.set_f32(IDENTITY_MATRIX + 4 * lane as u32, value);
        }
        e.mem.set_u32(WOBBLE_INDEX_TABLE + 3 * 4, 7);
        e.set_global(WOBBLE_SCALE_GLOBAL, 0.25f32);
        answer(e, ACTOR_GET_CURRENT_WEAPON, 0x7400_0000);
        answer(e, SLOT_A, 1);
        let player = player_with(e, &[], &[(0x454, SLOT_A)]);
        answer(e, GET_BYTE_AT_F4, 3);
        answer_float(e, READ_FLOAT_AT_D0, 0.5);
        answer(e, GUN_WOBBLE_GET_MATRIX, 0x7500_0000);
        answer(e, FUNCTION_0043FCD0, 0x7600_0000);
        answer(e, FIND_NODE_BY_NAME, 0x7700_0000);
        answer_float(e, FUNCTION_008B0DD0, 0.5);
        answer_float(e, READ_FLOAT_AT_C, 0.1);
        float_settings(
            e,
            &[
                (WOBBLE_SCALE_SETTING, 2.0),
                (WOBBLE_STEP_SETTING, 1.0),
                (FIRST_PERSON_SCALE_SETTING, 4.0),
                (AIM_SMOOTHING_SETTING, 0.5),
            ],
        );
        e.register_double(FLOAT_FUNCTION_00408840, |_, a| {
            float(f64::from(f32::from_bits(a[0]).abs()))
        });
        let collisions = Rc::new(Cell::new(0u32));
        e.register_double(FUNCTION_006838B0, move |_, _| {
            collisions.set(collisions.get() + 1);
            int(if collisions.get() == 1 {
                0
            } else {
                0x7800_0000
            })
        });
        answer(e, ALLOCATE_00AA13E0, 0x7900_0000);
        e.register_double(FUNCTION_00C8FFD0, |_, a| int(a[0]));
        answer(e, NI_OBJECT_CAST, 0x7a00_0000);
        e.register_double(MATRIX_TO_EULER, |e, a| {
            for (lane, value) in [0.1f32, 0.2, 0.3].into_iter().enumerate() {
                e.mem.set_f32(a[1 + lane], value);
            }
            Ret::default()
        });
        e.register_double(MATRIX_MULTIPLY, |_, a| int(a[1]));
        player
    }

    fn scaled_angles(scale: f32) -> [u32; 3] {
        [0.1f32, 0.2, 0.3].map(|angle| scaled(angle, scale).to_bits())
    }

    #[test]
    fn gun_wobble_without_a_process_resets_the_matrix() {
        let mut e = engine(CALLEES_00962DE0);
        let player = wobble_world(&mut e);
        e.set(player, PlayerCharacter::pCurrentProcess, Ptr::new(0));
        e.mem.set_u32(WOBBLE_RESULT_MATRIX, 0x1234);
        let log = logged(&mut e, |e| {
            e.call(0x0096_2de0, &args![player]);
        });
        assert_eq!(log.len(), 2);
        assert_eq!(e.mem.f32(WOBBLE_RESULT_MATRIX), 1.0);
        assert_eq!(e.mem.f32(WOBBLE_RESULT_MATRIX + 16), 1.0);
        assert_eq!(e.mem.f32(WOBBLE_RESULT_MATRIX + 4), 0.0);
    }

    #[test]
    fn gun_wobble_scales_the_rotation_of_a_ranged_weapon() {
        let mut e = engine(CALLEES_00962DE0);
        let player = wobble_world(&mut e);
        e.set(
            player,
            PlayerCharacter::p1stPersonAnimation,
            Ptr::new(0x7b00_0000),
        );
        let log = logged(&mut e, |e| {
            e.call(0x0096_2de0, &args![player]);
        });
        // The wobble node is found and cached for both passes.
        assert_eq!(e.mem.u32(player.addr() + 0xd74 + 3 * 4), 0x7700_0000);
        assert_eq!(e.mem.u32(player.addr() + 0xda4 + 7 * 4), 0x7700_0000);
        let calls = calls_to(&log, GUN_WOBBLE_GET_MATRIX);
        assert_eq!(calls.len(), 2);
        assert_eq!(calls[0][..2], [7, 0.5f32.to_bits()]);
        assert_eq!(calls[1][..2], [7, 0.5f32.to_bits()]);
        // The scale: 0.5 * 2.0 = 1.0 moves 0.1 (frame time * 1.0) from 0.25.
        let moved = (0.25f64 + f64::from(0.1f32)) as f32;
        assert_eq!(e.global::<f32>(WOBBLE_SCALE_GLOBAL), moved);
        // No collision object yet: one is made and attached.
        assert_eq!(
            calls_to(&log, FUNCTION_0062BC90),
            [[0x7700_0000, 0x7900_0000]]
        );
        // The rotation is scaled: first for the node, then twice for the
        // first person matrix.
        let from_euler = calls_to(&log, MATRIX_FROM_EULER);
        assert_eq!(from_euler.len(), 3);
        assert_eq!(from_euler[0][1..], scaled_angles(moved));
        assert_eq!(from_euler[1][0], WOBBLE_RESULT_MATRIX);
        assert_eq!(from_euler[1][1..], scaled_angles(moved));
        assert_eq!(
            from_euler[2][1..],
            [
                scaled(-0.3, moved).to_bits(),
                scaled(0.2, moved).to_bits(),
                scaled(0.1, moved).to_bits()
            ]
        );
        let stored = calls_to(&log, FUNCTION_004F0110);
        assert_eq!(stored.len(), 2);
        assert_eq!(stored[0][0], 0x7a00_0000);
        assert_eq!(stored[1], [0x7a00_0000, WOBBLE_RESULT_MATRIX]);
        // The product replaces the base matrix that is stored.
        assert_eq!(calls_to(&log, MATRIX_MULTIPLY).len(), 1);
    }

    #[test]
    fn gun_wobble_scale_jumps_when_the_step_covers_the_distance() {
        let mut e = engine(CALLEES_00962DE0);
        let player = wobble_world(&mut e);
        // Target 0.95, step 0.1: the scale 1.0 is reached at once.
        e.set_global(WOBBLE_SCALE_GLOBAL, 0.95f32);
        e.call(0x0096_2de0, &args![player]);
        assert_eq!(e.global::<f32>(WOBBLE_SCALE_GLOBAL), 1.0);

        // A scale below the target walks down by the step.
        e.set_global(WOBBLE_SCALE_GLOBAL, 2.0f32);
        e.call(0x0096_2de0, &args![player]);
        assert_eq!(
            e.global::<f32>(WOBBLE_SCALE_GLOBAL),
            (2.0f64 - f64::from(0.1f32)) as f32
        );

        // With the state flag the target is taken at once when rising.
        answer(&mut e, FUNCTION_00894900, 1);
        e.set_global(WOBBLE_SCALE_GLOBAL, 0.25f32);
        e.call(0x0096_2de0, &args![player]);
        assert_eq!(e.global::<f32>(WOBBLE_SCALE_GLOBAL), 0.5);

        // A scale equal to the target changes nothing.
        e.set_global(WOBBLE_SCALE_GLOBAL, 0.5f32);
        e.call(0x0096_2de0, &args![player]);
        assert_eq!(e.global::<f32>(WOBBLE_SCALE_GLOBAL), 0.5);
    }

    #[test]
    fn gun_wobble_of_the_bare_hands_follows_the_aim_offsets() {
        let mut e = engine(CALLEES_00962DE0);
        let player = wobble_world(&mut e);
        answer(&mut e, ACTOR_GET_CURRENT_WEAPON, 0);
        answer(&mut e, PLAYER_GET_ROOT_NODE, 0x8300_0000);
        answer(&mut e, FIND_NODE_BY_NAME, 0x8400_0000);
        answer(&mut e, SLOT_B, 1);
        let actor = object_with_slots(&mut e, &[(0x1d0, SLOT_B)]);
        e.set(player, PlayerCharacter::pAutoAimActor, Ptr::new(actor));
        e.register_double(FUNCTION_00965620, |e, a| {
            e.mem.set_f32(a[3], 1.0);
            e.mem.set_f32(a[4], 2.0);
            Ret::default()
        });
        e.set_global(AIM_GLOBAL_X, 0.0f32);
        e.set_global(AIM_GLOBAL_Y, 0.0f32);
        e.set_global(AIM_LIMIT_0102EFC4, 5.0f32);
        let log = logged(&mut e, |e| {
            e.call(0x0096_2de0, &args![player]);
        });
        let aim = calls_to(&log, FUNCTION_00965620);
        assert_eq!(aim.len(), 1);
        assert_eq!(aim[0][0], player.addr());
        assert_eq!(aim[0][1], 0);
        assert_eq!(aim[0][2], 0x8400_0000);
        assert_eq!(aim[0][5..], [0, 5.0f32.to_bits(), 5.0f32.to_bits()]);
        assert_eq!(
            calls_to(&log, FIND_NODE_BY_NAME)[0],
            [0x8300_0000, AIM_NODE_NAME]
        );
        // Each global moves towards its offset: g - 0.5 * (0.1 * (g - offset)).
        let expect = |offset: f32| {
            let weighted = 0.1f64 * (0.0f64 - f64::from(offset));
            (0.0f64 - 0.5f64 * weighted) as f32
        };
        assert_eq!(e.global::<f32>(AIM_GLOBAL_X), expect(1.0));
        assert_eq!(e.global::<f32>(AIM_GLOBAL_Y), expect(2.0));
        let from_euler = calls_to(&log, MATRIX_FROM_EULER);
        assert_eq!(
            from_euler[0][1..],
            [expect(1.0).to_bits(), 0, expect(2.0).to_bits()]
        );
        // The default node name is looked up in the model of the player and
        // the bare hands scale the wobble by 1.0.
        assert_eq!(
            calls_to(&log, FIND_NODE_BY_NAME)[1],
            [0x7600_0000, DEFAULT_WOBBLE_NODE_NAME]
        );
        // The weapon is gone for the first person pass.
        assert_eq!(calls_to(&log, GUN_WOBBLE_GET_MATRIX).len(), 0);
    }

    #[test]
    fn gun_wobble_first_person_without_an_index_turns_the_player() {
        let mut e = engine(CALLEES_00962DE0);
        let player = wobble_world(&mut e);
        // Type 0 has no wobble index; the first pass finds no node name.
        answer(&mut e, GET_BYTE_AT_F4, 0);
        let names = Rc::new(Cell::new(0u32));
        e.register_double(GUN_WOBBLE_GET_MATRIX, move |_, _| {
            names.set(names.get() + 1);
            int(if names.get() == 1 { 0 } else { 0x7500_0000 })
        });
        let log = logged(&mut e, |e| {
            e.call(0x0096_2de0, &args![player]);
        });
        // 0.5 (second pass scale call) * 4.0 (setting) = 2.0.
        let angle_x = scaled(0.1, 2.0);
        let angle_z = scaled(0.3, 2.0);
        assert_eq!(
            calls_to(&log, FUNCTION_00931E50),
            [[player.addr(), angle_x.to_bits()]]
        );
        assert_eq!(
            calls_to(&log, FUNCTION_00931D30),
            [[player.addr(), angle_z.to_bits()]]
        );
        let from_euler = calls_to(&log, MATRIX_FROM_EULER);
        assert_eq!(from_euler.len(), 1);
        assert_eq!(from_euler[0][0], WOBBLE_RESULT_MATRIX);
        assert_eq!(
            from_euler[0][1..],
            [
                scaled(-0.3, 2.0).to_bits(),
                scaled(0.2, 2.0).to_bits(),
                angle_x.to_bits()
            ]
        );
        assert_eq!(calls_to(&log, FUNCTION_008B0DD0)[0], [player.addr(), 0]);
    }

    #[test]
    fn gun_wobble_iron_sights_can_clear_the_index() {
        for (flagged, mod_active, cleared) in [(0, 0, true), (1, 1, true), (1, 0, false)] {
            let mut e = engine(CALLEES_00962DE0);
            let player = wobble_world(&mut e);
            // The first pass finds no node name, only the second pass does.
            let names = Rc::new(Cell::new(0u32));
            e.register_double(GUN_WOBBLE_GET_MATRIX, move |_, a| {
                names.set(names.get() + 1);
                int(if names.get() == 1 {
                    0
                } else {
                    a[0] + 0x7500_0000
                })
            });
            answer(&mut e, ACTOR_GET_IRON_SIGHTS, 1);
            answer(&mut e, FUNCTION_00504E60, 0x8600_0000);
            answer(&mut e, FUNCTION_0048CEE0, 1);
            answer(&mut e, FUNCTION_004AD030, flagged);
            answer(&mut e, ITEM_CHANGE_HAS_MOD_EFFECT, mod_active);
            answer(&mut e, SLOT_C, 0x8700_0000);
            let process = e.get(player, PlayerCharacter::pCurrentProcess).addr();
            let vtable = e.mem.u32(process);
            e.mem.set_u32(vtable + 0x148, SLOT_C);
            let log = logged(&mut e, |e| {
                e.call(0x0096_2de0, &args![player]);
            });
            let requested = calls_to(&log, GUN_WOBBLE_GET_MATRIX)[1][0];
            assert_eq!(
                requested,
                if cleared { 0 } else { 7 },
                "{flagged} {mod_active}"
            );
            assert_eq!(
                calls_to(&log, FUNCTION_00931E50).len(),
                usize::from(cleared)
            );
            assert_eq!(calls_to(&log, FUNCTION_0048CEE0), [[0x8600_0000]]);
        }
    }

    // ---- auto aim, perks and player actions -----------------------------------

    #[test]
    fn auto_aim_actor_needs_a_3d() {
        let mut e = engine(&[]);
        let player = e.new_object::<PlayerCharacter>();
        assert_eq!(e.call(0x0096_3730, &args![player]).u32(), 0);
        answer(&mut e, SLOT_A, 0x8800_0000);
        let actor = object_with_slots(&mut e, &[(0x1d0, SLOT_A)]);
        e.set(player, PlayerCharacter::pAutoAimActor, Ptr::new(actor));
        assert_eq!(e.call(0x0096_3730, &args![player]).u32(), actor);
        answer(&mut e, SLOT_A, 0);
        assert_eq!(e.call(0x0096_3730, &args![player]).u32(), 0);
    }

    const CALLEES_PERKS: &[u32] = &[
        FUNCTION_005EB6A0,
        FUNCTION_005EB800,
        FUNCTION_007DD710,
        FUNCTION_008C17C0,
        LIST_CONTAINS,
        LIST_APPEND,
        OPERATOR_NEW,
    ];

    /// A player whose own perk list (at +0x87c) holds `perk` at rank 1 and
    /// whose companion list (at +0xad4) is empty.
    fn perk_world(e: &mut Engine, perk: u32) -> (Ptr<PlayerCharacter>, u32) {
        list_accessors(e);
        list_remove_unlinks(e);
        let player = e.new_object::<PlayerCharacter>();
        let data = e.mem.alloc(8);
        e.mem.set_u32(data, perk);
        e.mem.set_u8(data + 4, 1);
        e.mem.set_u32(player.addr() + 0x87c, data);
        (player, data)
    }

    #[test]
    fn perk_rank_change_calls_the_perk_and_refreshes_the_menu() {
        let mut e = engine(CALLEES_PERKS);
        let (player, data) = perk_world(&mut e, 0x9000_0001);
        let log = logged(&mut e, |e| {
            e.call(0x0096_3790, &args![player, 0x9000_0001u32, 3u8, 0u8]);
        });
        assert_eq!(
            calls_to(&log, FUNCTION_005EB6A0),
            [[0x9000_0001, player.addr(), 1, 3, 0]]
        );
        assert_eq!(e.mem.u8(data + 4), 3);
        assert_eq!(calls_to(&log, FUNCTION_007DD710).len(), 1);
        assert!(calls_to(&log, OPERATOR_NEW).is_empty());

        // The same rank again does nothing.
        let log = logged(&mut e, |e| {
            e.call(0x0096_3790, &args![player, 0x9000_0001u32, 3u8, 0u8]);
        });
        assert!(calls_to(&log, FUNCTION_005EB6A0).is_empty());
        assert!(calls_to(&log, FUNCTION_007DD710).is_empty());

        // No perk.
        let log = logged(&mut e, |e| {
            e.call(0x0096_3790, &args![player, 0u32, 3u8, 0u8]);
        });
        assert_eq!(log.len(), 1);
    }

    #[test]
    fn a_new_perk_is_appended_once() {
        let mut e = engine(CALLEES_PERKS);
        let (player, _) = perk_world(&mut e, 0x9000_0001);
        let block = e.mem.alloc(8);
        answer(&mut e, OPERATOR_NEW, block);
        let log = logged(&mut e, |e| {
            e.call(0x0096_3790, &args![player, 0x9000_0002u32, 2u8, 0u8]);
        });
        assert_eq!(e.mem.u32(block), 0x9000_0002);
        assert_eq!(e.mem.u8(block + 4), 2);
        assert_eq!(calls_to(&log, OPERATOR_NEW), [[8]]);
        let list = player.addr() + 0x87c;
        assert_eq!(calls_to(&log, LIST_CONTAINS)[0][0], list);
        assert_eq!(calls_to(&log, LIST_APPEND)[0][0], list);
        assert_eq!(
            calls_to(&log, FUNCTION_005EB6A0),
            [[0x9000_0002, player.addr(), 0, 2, 0]]
        );
        assert_eq!(calls_to(&log, FUNCTION_007DD710).len(), 1);
        assert_eq!(calls_to(&log, FUNCTION_008C17C0), [[player.addr()]]);

        // Already in the list (as far as the list says): not appended again.
        answer(&mut e, LIST_CONTAINS, 1);
        let log = logged(&mut e, |e| {
            e.call(0x0096_3790, &args![player, 0x9000_0002u32, 2u8, 0u8]);
        });
        assert!(calls_to(&log, LIST_APPEND).is_empty());
    }

    #[test]
    fn a_companion_perk_uses_the_companion_list_and_leaves_the_menu() {
        let mut e = engine(CALLEES_PERKS);
        let (player, _) = perk_world(&mut e, 0x9000_0001);
        let block = e.mem.alloc(8);
        answer(&mut e, OPERATOR_NEW, block);
        let log = logged(&mut e, |e| {
            e.call(0x0096_3790, &args![player, 0x9000_0001u32, 2u8, 1u8]);
        });
        assert_eq!(calls_to(&log, LIST_APPEND)[0][0], player.addr() + 0xad4);
        assert_eq!(
            calls_to(&log, FUNCTION_005EB6A0),
            [[0x9000_0001, player.addr(), 0, 2, 1]]
        );
        assert!(calls_to(&log, FUNCTION_007DD710).is_empty());
    }

    #[test]
    fn perk_removal_and_rank_query() {
        let mut e = engine(CALLEES_PERKS);
        let (player, data) = perk_world(&mut e, 0x9000_0001);
        assert_eq!(
            e.call(0x0096_39e0, &args![player, 0x9000_0001u32, 0u8])
                .u8(),
            1
        );
        assert_eq!(
            e.call(0x0096_39e0, &args![player, 0x9000_0002u32, 0u8])
                .u8(),
            0
        );
        assert_eq!(
            e.call(0x0096_39e0, &args![player, 0x9000_0001u32, 1u8])
                .u8(),
            0
        );

        // Unknown perk or no perk: nothing.
        let log = logged(&mut e, |e| {
            e.call(0x0096_3900, &args![player, 0x9000_0002u32, 0u8]);
            e.call(0x0096_3900, &args![player, 0u32, 0u8]);
        });
        assert!(calls_to(&log, FUNCTION_005EB800).is_empty());

        let log = logged(&mut e, |e| {
            e.call(0x0096_3900, &args![player, 0x9000_0001u32, 0u8]);
        });
        assert_eq!(
            calls_to(&log, FUNCTION_005EB800),
            [[0x9000_0001, player.addr(), 0]]
        );
        assert_eq!(calls_to(&log, LIST_REMOVE)[0][0], player.addr() + 0x87c);
        assert_eq!(calls_to(&log, FUNCTION_007DD710).len(), 1);
        assert_eq!(calls_to(&log, FUNCTION_008C17C0), [[player.addr()]]);
        // The entry left the list but was not freed.
        assert_eq!(e.mem.u32(player.addr() + 0x87c), 0);
        assert_eq!(e.mem.u32(data), 0x9000_0001);
    }

    const CALLEES_PERK_ENTRIES: &[u32] = &[
        FUNCTION_0062F2F0,
        FUNCTION_007A7EB0,
        ADD_44_TO_ADDRESS,
        EXTRA_LIST_GET_CONTAINER_CHANGES,
    ];

    #[test]
    fn perk_entries_go_to_the_list_of_their_type() {
        let mut e = engine(CALLEES_PERK_ENTRIES);
        list_remove_unlinks(&mut e);
        e.set_global(RESET_VALUE_01012054, 6.5f32);
        answer(&mut e, SLOT_A, 2);
        answer(&mut e, FUNCTION_0062F2F0, 5);
        let container = e.mem.alloc(0x10);
        answer(&mut e, EXTRA_LIST_GET_CONTAINER_CHANGES, container);
        let form = object_with_slots(&mut e, &[(0x10, SLOT_A)]);
        let player = player_with(&mut e, &[], &[]);
        let log = logged(&mut e, |e| {
            e.call(0x0096_3a50, &args![player, Ptr::<()>::new(form), 0u8]);
            e.call(0x0096_3a50, &args![player, Ptr::<()>::new(form), 1u8]);
        });
        assert_eq!(
            calls_to(&log, FUNCTION_007A7EB0),
            [
                [player.addr() + 0x884 + 5 * 8, form, PERK_ENTRY_COMPARE],
                [player.addr() + 0xadc + 5 * 8, form, PERK_ENTRY_COMPARE]
            ]
        );
        assert_eq!(e.mem.f32(container + 8), 6.5);

        // The removal uses the same list.
        let log = logged(&mut e, |e| {
            e.call(0x0096_3b20, &args![player, Ptr::<()>::new(form), 0u8]);
            e.call(0x0096_3b20, &args![player, Ptr::<()>::new(form), 1u8]);
        });
        let removals = calls_to(&log, LIST_REMOVE);
        assert_eq!(removals[0][0], player.addr() + 0x884 + 5 * 8);
        assert_eq!(removals[1][0], player.addr() + 0xadc + 5 * 8);
    }

    #[test]
    fn perk_entries_of_other_kinds_are_ignored() {
        let mut e = engine(CALLEES_PERK_ENTRIES);
        list_remove_unlinks(&mut e);
        answer(&mut e, SLOT_A, 2);
        answer(&mut e, SLOT_B, 3);
        let ok = object_with_slots(&mut e, &[(0x10, SLOT_A)]);
        let other = object_with_slots(&mut e, &[(0x10, SLOT_B)]);
        let player = player_with(&mut e, &[], &[]);
        let log = logged(&mut e, |e| {
            // Not an entry (the virtual says 3), no form, a type out of range.
            e.call(0x0096_3a50, &args![player, Ptr::<()>::new(other), 0u8]);
            e.call(0x0096_3a50, &args![player, 0u32, 0u8]);
            answer_type(e, 0x4a);
            e.call(0x0096_3a50, &args![player, Ptr::<()>::new(ok), 0u8]);
            e.call(0x0096_3b20, &args![player, Ptr::<()>::new(ok), 0u8]);
        });
        assert!(calls_to(&log, FUNCTION_007A7EB0).is_empty());
        assert!(calls_to(&log, LIST_REMOVE).is_empty());
    }

    /// Makes `0062f2f0` answer `kind`.
    fn answer_type(e: &mut Engine, kind: u32) {
        answer(e, FUNCTION_0062F2F0, kind);
    }

    #[test]
    fn perk_entry_list_address() {
        let mut e = engine(&[]);
        let player = e.new_object::<PlayerCharacter>();
        assert_eq!(
            e.call(0x0096_3ba0, &args![player, 3u8, 0u8]).u32(),
            player.addr() + 0x884 + 24
        );
        assert_eq!(
            e.call(0x0096_3ba0, &args![player, 3u8, 1u8]).u32(),
            player.addr() + 0xadc + 24
        );
        assert_eq!(e.call(0x0096_3ba0, &args![player, 0x4au8, 0u8]).u32(), 0);
        assert_eq!(
            e.call(0x0096_3ba0, &args![player, 0x49u8, 0u8]).u32(),
            player.addr() + 0x884 + 0x49 * 8
        );
    }

    #[test]
    fn float_reset() {
        let mut e = engine(&[]);
        e.set_global(RESET_VALUE_01012054, 4.25f32);
        let object = e.mem.alloc(0x10);
        e.call(0x0096_3b00, &args![Ptr::<()>::new(object)]);
        assert_eq!(e.mem.f32(object + 8), 4.25);
    }

    /// A player action: type, timer, object.
    fn action(e: &mut Engine, kind: u32, timer: f32, object: u32) -> u32 {
        let block = e.mem.alloc(0xc);
        e.mem.set_u32(block, kind);
        e.mem.set_f32(block + 4, timer);
        e.mem.set_u32(block + 8, object);
        block
    }

    /// A player whose action list (`+0x60c`) holds `actions`.
    fn action_world(e: &mut Engine, actions: &[u32]) -> Ptr<PlayerCharacter> {
        list_accessors(e);
        list_remove_unlinks(e);
        let player = e.new_object::<PlayerCharacter>();
        let list = list_of(e, actions);
        e.set(player, PlayerCharacter::pListofActions, Ptr::new(list));
        player
    }

    #[test]
    fn actions_age_and_expired_ones_go() {
        let mut e = engine(&[READ_FLOAT_AT_C, OPERATOR_DELETE]);
        let expired = action(&mut e, 1, 0.0, 0);
        let kept_type = action(&mut e, 5, -1.0, 0);
        let running = action(&mut e, 2, 2.0, 0);
        let player = action_world(&mut e, &[expired, kept_type, running]);
        answer_float(&mut e, READ_FLOAT_AT_C, 0.5);
        let log = logged(&mut e, |e| {
            e.call(0x0096_3bf0, &args![player]);
        });
        assert_eq!(calls_to(&log, OPERATOR_DELETE), [[expired]]);
        assert_eq!(e.mem.f32(kept_type + 4), -1.5);
        assert_eq!(e.mem.f32(running + 4), 1.5);
        // The list now starts with the entry that was kept.
        let head = e.get(player, PlayerCharacter::pListofActions).addr();
        assert_eq!(e.mem.u32(head), kept_type);
    }

    #[test]
    fn action_queries() {
        let mut e = engine(&[]);
        let first = action(&mut e, 3, 1.0, 0x9100_0001);
        let second = action(&mut e, 4, 1.0, 0x9100_0002);
        let player = action_world(&mut e, &[first, second]);
        assert!(e.call(0x0096_3ce0, &args![player, 4u32]).bool());
        assert!(!e.call(0x0096_3ce0, &args![player, 9u32]).bool());
        assert_eq!(e.call(0x0096_4060, &args![player]).u32(), 3);

        let empty = e.new_object::<PlayerCharacter>();
        assert!(!e.call(0x0096_3ce0, &args![empty, 4u32]).bool());
        assert_eq!(e.call(0x0096_4060, &args![empty]).u32(), 0);
    }

    #[test]
    fn actions_are_removed_by_object_or_by_type() {
        let mut e = engine(&[OPERATOR_DELETE]);
        let first = action(&mut e, 3, 1.0, 0x9100_0001);
        let second = action(&mut e, 4, 1.0, 0x9100_0002);
        let third = action(&mut e, 3, 1.0, 0x9100_0002);
        let player = action_world(&mut e, &[first, second, third]);
        let log = logged(&mut e, |e| {
            e.call(0x0096_3d60, &args![player, 0x9100_0002u32]);
            e.call(0x0096_3d60, &args![player, 0u32]);
        });
        assert_eq!(calls_to(&log, OPERATOR_DELETE), [[second], [third]]);
        let head = e.get(player, PlayerCharacter::pListofActions).addr();
        assert_eq!(e.mem.u32(head), first);
        // What is left after the entry is only an emptied node.
        assert_eq!(e.mem.u32(e.mem.u32(head + 4)), 0);

        // By type and object: any object with 0, only the first match goes.
        let a = action(&mut e, 7, 1.0, 0x9100_0003);
        let b = action(&mut e, 7, 1.0, 0x9100_0004);
        let player = action_world(&mut e, &[a, b]);
        let log = logged(&mut e, |e| {
            e.call(0x0096_3e00, &args![player, 7u32, 0x9100_0009u32]);
            e.call(0x0096_3e00, &args![player, 7u32, 0x9100_0004u32]);
            e.call(0x0096_3e00, &args![player, 8u32, 0u32]);
        });
        assert_eq!(calls_to(&log, OPERATOR_DELETE), [[b]]);
        let log = logged(&mut e, |e| {
            e.call(0x0096_3e00, &args![player, 7u32, 0u32]);
        });
        assert_eq!(calls_to(&log, OPERATOR_DELETE), [[a]]);

        // No list: nothing.
        let empty = e.new_object::<PlayerCharacter>();
        let log = logged(&mut e, |e| {
            e.call(0x0096_3e00, &args![empty, 7u32, 0u32]);
            e.call(0x0096_3d60, &args![empty, 5u32]);
        });
        assert_eq!(log.len(), 2);
    }

    const CALLEES_00963EB0: &[u32] = &[
        PLAYER_IS_IN_COMBAT,
        OPERATOR_NEW,
        FUNCTION_0096A2D0,
        FUNCTION_0078D900,
        TES_OBJECT_REFR_SET_TARGETED,
        LIST_APPEND,
    ];

    #[test]
    fn a_new_action_is_made_and_appended() {
        let mut e = engine(CALLEES_00963EB0);
        let player = action_world(&mut e, &[]);
        e.set(player, PlayerCharacter::pListofActions, Ptr::new(0));
        let list_block = e.mem.alloc(8);
        let action_block = e.mem.alloc(0xc);
        e.register_double(OPERATOR_NEW, move |_, a| {
            int(if a[0] == 8 { list_block } else { action_block })
        });
        e.register_double(FUNCTION_0096A2D0, |_, a| int(a[0]));
        e.register_double(FUNCTION_0078D900, |_, a| int(a[0]));
        let log = logged(&mut e, |e| {
            e.call(0x0096_3eb0, &args![player, 6u32, 2.5f32, 0x9200_0000u32]);
        });
        assert_eq!(
            e.get(player, PlayerCharacter::pListofActions).addr(),
            list_block
        );
        assert_eq!(e.mem.u32(action_block), 6);
        assert_eq!(e.mem.f32(action_block + 4), 2.5);
        assert_eq!(e.mem.u32(action_block + 8), 0x9200_0000);
        assert_eq!(
            calls_to(&log, TES_OBJECT_REFR_SET_TARGETED),
            [[0x9200_0000, 1]]
        );
        assert_eq!(calls_to(&log, LIST_APPEND)[0][0], list_block);
        assert_eq!(calls_to(&log, FUNCTION_0078D900), [[action_block]]);
    }

    #[test]
    fn an_action_of_the_same_type_is_updated_in_place() {
        let mut e = engine(CALLEES_00963EB0);
        let old = action(&mut e, 6, 1.0, 0);
        let player = action_world(&mut e, &[old]);
        let log = logged(&mut e, |e| {
            e.call(0x0096_3eb0, &args![player, 6u32, 3.0f32, 0u32]);
        });
        assert!(calls_to(&log, FUNCTION_0078D900).is_empty());
        let list = e.get(player, PlayerCharacter::pListofActions).addr();
        assert_eq!(calls_to(&log, LIST_REMOVE)[0][0], list);
        assert_eq!(e.mem.f32(old + 4), 3.0);
        // No object: no targeting.
        assert!(calls_to(&log, TES_OBJECT_REFR_SET_TARGETED).is_empty());
        assert_eq!(calls_to(&log, LIST_APPEND).len(), 1);
    }

    #[test]
    fn no_action_is_added_in_combat() {
        let mut e = engine(CALLEES_00963EB0);
        answer(&mut e, PLAYER_IS_IN_COMBAT, 1);
        let player = action_world(&mut e, &[]);
        let log = logged(&mut e, |e| {
            e.call(0x0096_3eb0, &args![player, 6u32, 3.0f32, 0u32]);
        });
        assert_eq!(log.len(), 2);
    }

    #[test]
    fn first_action_object_is_read_from_the_head_of_the_list() {
        let mut e = engine(&[]);
        let first = action(&mut e, 3, 1.0, 0x9100_0001);
        let second = action(&mut e, 4, 1.0, 0x9100_0002);
        let player = action_world(&mut e, &[first, second]);
        assert_eq!(e.call(0x0096_40b0, &args![player]).u32(), 0x9100_0001);
        let empty = e.new_object::<PlayerCharacter>();
        assert_eq!(e.call(0x0096_40b0, &args![empty]).u32(), 0);
        // A list whose head holds no action.
        let blank = action_world(&mut e, &[0]);
        assert_eq!(e.call(0x0096_40b0, &args![blank]).u32(), 0);
    }

    #[test]
    fn get_player_action_finds_the_first_entry_of_the_type() {
        let mut e = engine(&[]);
        let first = action(&mut e, 3, 1.0, 0);
        let second = action(&mut e, 4, 1.0, 0);
        let third = action(&mut e, 4, 2.0, 0);
        let player = action_world(&mut e, &[first, second, third]);
        assert_eq!(e.call(0x0096_4100, &args![player, 4u32]).u32(), second);
        assert_eq!(e.call(0x0096_4100, &args![player, 3u32]).u32(), first);
        assert_eq!(e.call(0x0096_4100, &args![player, 9u32]).u32(), 0);
        let empty = e.new_object::<PlayerCharacter>();
        assert_eq!(e.call(0x0096_4100, &args![empty, 4u32]).u32(), 0);
    }

    #[test]
    fn actor_mover_is_replaced_and_the_old_one_destroyed() {
        let mut e = engine(&[OPERATOR_NEW, ACTOR_MOVER_NEW]);
        answer(&mut e, SLOT_A, 0);
        answer(&mut e, OPERATOR_NEW, 0x4000_0000);
        e.register_double(ACTOR_MOVER_NEW, |_, a| int(a[0] + 0x10));
        let old = object_with_slots(&mut e, &[(0, SLOT_A)]);
        let player = e.new_object::<PlayerCharacter>();
        e.set(player, PlayerCharacter::pActorMover, Ptr::new(old));
        let log = logged(&mut e, |e| {
            e.call(0x0096_4190, &args![player]);
        });
        assert_eq!(calls_to(&log, SLOT_A), [[old, 1]]);
        assert_eq!(calls_to(&log, OPERATOR_NEW), [[0xa0]]);
        assert_eq!(
            calls_to(&log, ACTOR_MOVER_NEW),
            [[0x4000_0000, player.addr()]]
        );
        assert_eq!(
            e.get(player, PlayerCharacter::pActorMover).addr(),
            0x4000_0010
        );
        // Without a mover nothing is destroyed.
        let fresh = e.new_object::<PlayerCharacter>();
        let log = logged(&mut e, |e| {
            e.call(0x0096_4190, &args![fresh]);
        });
        assert!(calls_to(&log, SLOT_A).is_empty());
        assert_eq!(
            e.get(fresh, PlayerCharacter::pActorMover).addr(),
            0x4000_0010
        );
    }

    #[test]
    fn image_space_setters_store_their_floats() {
        let mut e = engine(&[]);
        e.call(0x0096_5240, &args![1u8]);
        assert_eq!(e.global::<u8>(IRON_SIGHTS_ACTIVE), 1);
        e.call(0x0096_5240, &args![0u8]);
        assert_eq!(e.global::<u8>(IRON_SIGHTS_ACTIVE), 0);
        let object = e.mem.alloc(0x80);
        e.call(0x0096_5250, &args![Ptr::<()>::new(object), 1.5f32]);
        e.call(0x0096_5580, &args![Ptr::<()>::new(object), -2.0f32]);
        e.call(0x0096_6a00, &args![Ptr::<()>::new(object), 3.25f32]);
        assert_eq!(e.mem.f32(object + 0x28), 1.5);
        assert_eq!(e.mem.f32(object + 0x38), -2.0);
        assert_eq!(e.mem.f32(object + 0xd0), 3.25);
    }

    #[test]
    fn iron_sights_and_vats_depth_of_field_instances_are_stopped() {
        let mut e = engine(&[IMAGE_SPACE_INSTANCE_STOP]);
        let player = e.new_object::<PlayerCharacter>();
        e.set(
            player,
            PlayerCharacter::pIronsightsDOFInstance,
            Ptr::new(0x5000_0001),
        );
        e.set(
            player,
            PlayerCharacter::pVatsDOFInstance,
            Ptr::new(0x5000_0002),
        );
        let log = logged(&mut e, |e| {
            e.call(0x0096_55a0, &args![player]);
            e.call(0x0096_55e0, &args![player]);
        });
        assert_eq!(
            calls_to(&log, IMAGE_SPACE_INSTANCE_STOP),
            [[0x5000_0001], [0x5000_0002]]
        );
        assert!(e
            .get(player, PlayerCharacter::pIronsightsDOFInstance)
            .is_null());
        assert!(e.get(player, PlayerCharacter::pVatsDOFInstance).is_null());
        // Nothing to stop the second time.
        let log = logged(&mut e, |e| {
            e.call(0x0096_55a0, &args![player]);
            e.call(0x0096_55e0, &args![player]);
        });
        assert!(calls_to(&log, IMAGE_SPACE_INSTANCE_STOP).is_empty());
    }

    #[test]
    fn flag_dependent_float_getter() {
        let mut e = engine(&[FLAGS_TEST]);
        let object = e.mem.alloc(0x80);
        e.mem.set_f32(object + 0x64, 0.25);
        answer(&mut e, FLAGS_TEST, 0);
        assert_eq!(
            e.call(0x0096_6980, &args![Ptr::<()>::new(object)]).f32(),
            0.25
        );
        answer(&mut e, FLAGS_TEST, 1);
        let log = logged(&mut e, |e| {
            assert_eq!(
                e.call(0x0096_6980, &args![Ptr::<()>::new(object)]).f32(),
                1.0
            );
        });
        assert_eq!(calls_to(&log, FLAGS_TEST), [[object, 0x20000]]);
    }

    #[test]
    fn blend_function_gets_the_form_float_and_the_two_object_floats() {
        let mut e = engine(&[GET_BASE_FORM, READ_FIELD_68, BLEND_FLOATS]);
        answer(&mut e, GET_BASE_FORM, 0x6000_0000);
        answer_float(&mut e, READ_FIELD_68, 0.5);
        answer_float(&mut e, BLEND_FLOATS, 0.75);
        let object = e.mem.alloc(0x100);
        e.mem.set_f32(object + 0xcc, 2.0);
        e.mem.set_f32(object + 0xd0, 3.0);
        let log = logged(&mut e, |e| {
            assert_eq!(
                e.call(0x0096_69c0, &args![Ptr::<()>::new(object)]).f32(),
                0.75
            );
        });
        assert_eq!(calls_to(&log, GET_BASE_FORM), [[object]]);
        assert_eq!(calls_to(&log, READ_FIELD_68), [[0x6000_0000]]);
        assert_eq!(
            calls_to(&log, BLEND_FLOATS),
            [[0.5f32.to_bits(), 2.0f32.to_bits(), 3.0f32.to_bits()]]
        );
    }

    #[test]
    fn small_getters() {
        let mut e = engine(&[LIST_COUNT]);
        let object = e.mem.alloc(0x100);
        e.mem.set_f32(object + 0xec, 9.5);
        assert_eq!(
            e.call(0x0096_6a20, &args![Ptr::<()>::new(object)]).f32(),
            9.5
        );
        e.set_global(0x011f_1c00, 4.5f32);
        assert_eq!(e.call(0x0096_6a40, &args![]).f32(), 4.5);
        answer(&mut e, LIST_COUNT, 6);
        let player = e.new_object::<PlayerCharacter>();
        let log = logged(&mut e, |e| {
            assert_eq!(e.call(0x0096_6a50, &args![player]).u32(), 6);
        });
        assert_eq!(calls_to(&log, LIST_COUNT), [[player.addr() + 0x5e4]]);
    }

    const CALLEES_00966A70: &[u32] = &[
        STRING_NEW,
        STRING_DELETE,
        LOG_WARNING,
        LIST_CONTAINS,
        LIST_APPEND,
        GET_LOCATION_NAME,
        SETTING_STRING,
        STRING_FORMAT,
        STRING_TEXT,
        SHOW_HUD_MESSAGE,
        FUNCTION_007052F0,
    ];

    #[test]
    fn a_new_note_is_listed_formatted_and_announced() {
        let mut e = engine(CALLEES_00966A70);
        e.set_global(MESSAGE_DURATION_010162C0, 2.0f32);
        answer(&mut e, SETTING_STRING, 0x6000_0001);
        answer(&mut e, GET_LOCATION_NAME, 0x6000_0002);
        answer(&mut e, STRING_TEXT, 0x6000_0003);
        let note = 0x7000_0000u32;
        let seen = Rc::new(Cell::new(0u32));
        let remember = seen.clone();
        e.register_double(LIST_CONTAINS, move |e, a| {
            remember.set(e.mem.u32(a[1]));
            int(0)
        });
        let player = e.new_object::<PlayerCharacter>();
        let list = player.addr() + 0x5e4;
        let log = logged(&mut e, |e| {
            e.call(0x0096_6a70, &args![player, note, 1u8]);
        });
        assert_eq!(seen.get(), note);
        let appended = calls_to(&log, LIST_APPEND);
        assert_eq!(appended.len(), 1);
        assert_eq!(appended[0][0], list);
        let created = calls_to(&log, STRING_NEW);
        assert_eq!(calls_to(&log, STRING_DELETE), created);
        let string = created[0][0];
        assert_eq!(
            calls_to(&log, STRING_FORMAT),
            [[string, 0x6000_0001, 0x6000_0002]]
        );
        assert_eq!(calls_to(&log, GET_LOCATION_NAME), [[note + 0x48]]);
        assert_eq!(calls_to(&log, SETTING_STRING), [[NOTE_FORMAT_SETTING]]);
        assert_eq!(calls_to(&log, SHOW_HUD_MESSAGE), [[0x6000_0003]]);
        assert_eq!(
            calls_to(&log, FUNCTION_007052F0),
            [[0x6000_0003, 0, 0, NOTE_ADDED_SOUND, 2.0f32.to_bits(), 0]]
        );
        // Without the show flag the message is not shown, the sound still is.
        let log = logged(&mut e, |e| {
            e.call(0x0096_6a70, &args![player, note, 0u8]);
        });
        assert!(calls_to(&log, SHOW_HUD_MESSAGE).is_empty());
        assert_eq!(calls_to(&log, FUNCTION_007052F0).len(), 1);
    }

    #[test]
    fn a_known_note_is_only_announced_and_a_null_note_only_logged() {
        let mut e = engine(CALLEES_00966A70);
        answer(&mut e, LIST_CONTAINS, 1);
        let player = e.new_object::<PlayerCharacter>();
        let log = logged(&mut e, |e| {
            e.call(0x0096_6a70, &args![player, 0x7000_0000u32, 1u8]);
        });
        assert!(calls_to(&log, LIST_APPEND).is_empty());
        assert!(calls_to(&log, STRING_FORMAT).is_empty());
        assert!(calls_to(&log, SHOW_HUD_MESSAGE).is_empty());
        assert_eq!(calls_to(&log, FUNCTION_007052F0).len(), 1);

        let log = logged(&mut e, |e| {
            e.call(0x0096_6a70, &args![player, 0u32, 1u8]);
        });
        assert_eq!(calls_to(&log, LOG_WARNING), [[NULL_NOTE_WARNING]]);
        assert!(calls_to(&log, FUNCTION_007052F0).is_empty());
        assert!(calls_to(&log, LIST_CONTAINS).is_empty());
        assert_eq!(
            calls_to(&log, STRING_NEW).len(),
            calls_to(&log, STRING_DELETE).len()
        );
    }

    #[test]
    fn a_note_is_removed_from_the_display_and_the_list() {
        let mut e = engine(&[NOTE_REMOVE_FROM_DISPLAY, LIST_REMOVE]);
        let seen = Rc::new(Cell::new(0u32));
        let remember = seen.clone();
        e.register_double(LIST_REMOVE, move |e, a| {
            remember.set(e.mem.u32(a[1]));
            int(1)
        });
        let player = e.new_object::<PlayerCharacter>();
        let log = logged(&mut e, |e| {
            e.call(0x0096_6b80, &args![player, 0x7000_0010u32]);
        });
        assert_eq!(calls_to(&log, NOTE_REMOVE_FROM_DISPLAY), [[0x7000_0010]]);
        assert_eq!(seen.get(), 0x7000_0010);
        assert_eq!(calls_to(&log, LIST_REMOVE)[0][0], player.addr() + 0x5e4);
    }

    /// A player whose embedded note list holds `notes`.
    fn note_world(e: &mut Engine, notes: &[u32]) -> Ptr<PlayerCharacter> {
        list_accessors(e);
        let player = e.new_object::<PlayerCharacter>();
        if let Some((&first, rest)) = notes.split_first() {
            let next = list_of(e, rest);
            e.mem.set_u32(player.addr() + 0x5e4, first);
            e.mem.set_u32(player.addr() + 0x5e8, next);
        }
        player
    }

    #[test]
    fn note_by_index() {
        let mut e = engine(&[]);
        let player = note_world(&mut e, &[0x7000_0001, 0x7000_0002]);
        assert_eq!(e.call(0x0096_6bb0, &args![player, 0u32]).u32(), 0x7000_0001);
        assert_eq!(e.call(0x0096_6bb0, &args![player, 1u32]).u32(), 0x7000_0002);
        assert_eq!(e.call(0x0096_6bb0, &args![player, 2u32]).u32(), 0);
        let none = note_world(&mut e, &[]);
        assert_eq!(e.call(0x0096_6bb0, &args![none, 0u32]).u32(), 0);
    }

    #[test]
    fn note_membership_asks_the_list_unless_the_note_is_null() {
        let mut e = engine(&[LIST_CONTAINS]);
        let seen = Rc::new(Cell::new(0u32));
        let remember = seen.clone();
        e.register_double(LIST_CONTAINS, move |e, a| {
            remember.set(e.mem.u32(a[1]));
            int(1)
        });
        let player = e.new_object::<PlayerCharacter>();
        assert!(e.call(0x0096_6c10, &args![player, 0x7000_0003u32]).bool());
        assert_eq!(seen.get(), 0x7000_0003);
        let log = logged(&mut e, |e| {
            assert!(!e.call(0x0096_6c10, &args![player, 0u32]).bool());
        });
        assert!(calls_to(&log, LIST_CONTAINS).is_empty());
    }

    const CALLEES_00966C60: &[u32] = &[
        READ_DWORD_AT_20,
        RT_DYNAMIC_CAST,
        READ_FIELD_A0,
        LIST_CONTAINS,
        TERMINAL_IS_HACKED,
        TERMINAL_IS_UNLOCKED,
        TERMINAL_IS_LOCKED_OUT,
        TERMINAL_GET_RAW_DIFFICULTY,
        TERMINAL_IS_HACK_ELIGIBLE,
        ACTOR_VALUE_GET_CLAMPED,
    ];

    /// The terminal reference 0x7000_0000 with the terminal form
    /// 0x7100_0000 and the answers of the terminal queries.
    fn terminal_access(
        owned: u32,
        hacked: u32,
        unlocked: u32,
        locked_out: u32,
        difficulty: u32,
        eligible: u32,
    ) -> (u32, Vec<Vec<u32>>) {
        let mut e = engine(CALLEES_00966C60);
        answer(&mut e, READ_DWORD_AT_20, 0x7200_0000);
        answer(&mut e, RT_DYNAMIC_CAST, 0x7100_0000);
        answer(&mut e, READ_FIELD_A0, 0x7300_0000);
        answer(&mut e, LIST_CONTAINS, owned);
        answer(&mut e, TERMINAL_IS_HACKED, hacked);
        answer(&mut e, TERMINAL_IS_UNLOCKED, unlocked);
        answer(&mut e, TERMINAL_IS_LOCKED_OUT, locked_out);
        answer(&mut e, TERMINAL_GET_RAW_DIFFICULTY, difficulty);
        answer(&mut e, TERMINAL_IS_HACK_ELIGIBLE, eligible);
        answer_float(&mut e, ACTOR_VALUE_GET_CLAMPED, 25.0);
        let player = e.new_object::<PlayerCharacter>();
        let mut result = 0;
        let log = logged(&mut e, |e| {
            result = e.call(0x0096_6c60, &args![player, 0x7000_0000u32]).u32();
        });
        (result, log.into_iter().map(|(_, args)| args).collect())
    }

    #[test]
    fn terminal_access_levels() {
        // (owned note, hacked, unlocked, locked out, difficulty, eligible)
        assert_eq!(terminal_access(1, 0, 0, 0, 0, 0).0, 0);
        assert_eq!(terminal_access(0, 1, 0, 0, 0, 0).0, 1);
        assert_eq!(terminal_access(0, 0, 1, 0, 0, 0).0, 1);
        assert_eq!(terminal_access(0, 0, 0, 1, 0, 0).0, 2);
        assert_eq!(terminal_access(0, 0, 0, 0, 5, 0).0, 2);
        assert_eq!(terminal_access(0, 0, 0, 0, 2, 1).0, 3);
        assert_eq!(terminal_access(0, 0, 0, 0, 2, 0).0, 4);
    }

    #[test]
    fn terminal_access_without_a_terminal_form() {
        let mut e = engine(CALLEES_00966C60);
        let player = e.new_object::<PlayerCharacter>();
        // No reference at all.
        assert_eq!(e.call(0x0096_6c60, &args![player, 0u32]).u32(), 4);
        // A base form that is not a terminal (the cast answers null).
        answer(&mut e, READ_DWORD_AT_20, 0x7200_0000);
        answer(&mut e, RT_DYNAMIC_CAST, 0);
        let log = logged(&mut e, |e| {
            assert_eq!(e.call(0x0096_6c60, &args![player, 0x7000_0000u32]).u32(), 4);
        });
        assert_eq!(
            calls_to(&log, RT_DYNAMIC_CAST),
            [[0x7200_0000, 0, TYPE_TES_FORM, TYPE_BGS_TERMINAL, 0]]
        );
        // A reference with no base form.
        answer(&mut e, READ_DWORD_AT_20, 0);
        assert_eq!(e.call(0x0096_6c60, &args![player, 0x7000_0000u32]).u32(), 4);
    }

    #[test]
    fn terminal_access_passes_the_skill_to_the_hack_check() {
        let (_, calls) = terminal_access(0, 0, 0, 0, 2, 1);
        assert!(calls.contains(&vec![0x7100_0000, 0x7000_0000, 25.0f32.to_bits()]));
        assert!(calls.contains(&vec![0x7100_0000, 0x7000_0000]));
    }

    #[test]
    fn form_map_walk_clears_extra_data_of_matching_references() {
        let mut e = engine(&[
            MAP_FIRST_ENTRY,
            MAP_NEXT_ENTRY,
            FORM_GET_TYPE,
            READ_DWORD_AT_20,
            REFERENCE_REMOVE_EXTRA_50,
        ]);
        e.set_global(FORM_MAP, 0x6000_0000u32);
        // Forms: +4 is the type, +0x20 the base form.
        let make = |e: &mut Engine, kind: u8, base_kind: u8| {
            let base = e.mem.alloc(0x40);
            e.mem.set_u8(base + 4, base_kind);
            let form = e.mem.alloc(0x40);
            e.mem.set_u8(form + 4, kind);
            e.mem.set_u32(form + 0x20, base);
            form
        };
        let wanted = make(&mut e, 0x3a, 0x17);
        let other_base = make(&mut e, 0x3a, 0x18);
        let other_kind = make(&mut e, 0x10, 0x17);
        let forms = vec![wanted, other_base, other_kind];
        e.register_double(FORM_GET_TYPE, |e, a| int(u32::from(e.mem.u8(a[0] + 4))));
        e.register_double(READ_DWORD_AT_20, |e, a| int(e.mem.u32(a[0] + 0x20)));
        answer(&mut e, MAP_FIRST_ENTRY, 1);
        let table = forms.clone();
        e.register_double(MAP_NEXT_ENTRY, move |e, a| {
            let position = e.mem.u32(a[1]) as usize;
            e.mem.set_u32(a[3], table[position - 1]);
            let next = if position < 3 { position as u32 + 1 } else { 0 };
            e.mem.set_u32(a[1], next);
            Ret::default()
        });
        let log = logged(&mut e, |e| {
            e.call(0x0096_6d70, &args![Ptr::<()>::new(0)]);
        });
        assert_eq!(calls_to(&log, REFERENCE_REMOVE_EXTRA_50), [[wanted]]);
        assert_eq!(calls_to(&log, MAP_NEXT_ENTRY).len(), 3);
    }

    #[test]
    fn steal_and_pickpocket_warnings_count_and_arm_their_timers() {
        let mut e = engine(&[]);
        e.set_global(DOUBLE_MINUS_ONE, -1.0f64);
        float_settings(&mut e, &[(WARNING_TIMER_SETTING, 7.5)]);
        let player = e.new_object::<PlayerCharacter>();
        e.set(player, PlayerCharacter::fStealWarningTimer, -1.0);
        e.set(player, PlayerCharacter::fPickPocketWarningTimer, 2.0);
        e.call(0x0096_6e00, &args![player]);
        e.call(0x0096_6e50, &args![player]);
        assert_eq!(e.get(player, PlayerCharacter::iNumberofStealWarnings), 1);
        assert_eq!(e.get(player, PlayerCharacter::fStealWarningTimer), 7.5);
        assert_eq!(
            e.get(player, PlayerCharacter::iNumberofPickpocketWarnings),
            1
        );
        // A running timer is left alone; the count still grows.
        assert_eq!(e.get(player, PlayerCharacter::fPickPocketWarningTimer), 2.0);
        e.call(0x0096_6e00, &args![player]);
        assert_eq!(e.get(player, PlayerCharacter::iNumberofStealWarnings), 2);
        assert_eq!(e.get(player, PlayerCharacter::fStealWarningTimer), 7.5);
        // The pickpocket timer is armed the same way.
        e.set(player, PlayerCharacter::fPickPocketWarningTimer, -1.0);
        e.call(0x0096_6e50, &args![player]);
        assert_eq!(e.get(player, PlayerCharacter::fPickPocketWarningTimer), 7.5);
    }

    const CALLEES_00966EA0: &[u32] = &[
        ACTOR_MOVER_SET_FLAGS,
        PLAYER_GET_ANIMATION,
        ANIMATION_HOLDER_GET_GROUP,
        ANIM_GROUP_GET_TYPE,
        ANIMATION_BLEND_OUT,
    ];

    #[test]
    fn mover_flags_are_set_and_the_blend_out_needs_group_type_0x11() {
        let mut e = engine(CALLEES_00966EA0);
        let player = player_with(&mut e, &[(0x1e4, SLOT_A)], &[]);
        let mover = e.mem.alloc(0x100);
        e.set(player, PlayerCharacter::pActorMover, Ptr::new(mover));
        answer(&mut e, SLOT_A, 0x6000_0001);
        answer(&mut e, PLAYER_GET_ANIMATION, 0x6000_0002);
        answer(&mut e, ANIMATION_HOLDER_GET_GROUP, 0x77);
        answer(&mut e, ANIM_GROUP_GET_TYPE, 0x11);
        // Mask without 0x400: only the flags.
        let log = logged(&mut e, |e| {
            e.call(0x0096_6ea0, &args![player, 0x3u32, 1u8]);
        });
        assert_eq!(calls_to(&log, ACTOR_MOVER_SET_FLAGS), [[mover, 3, 1]]);
        assert!(calls_to(&log, PLAYER_GET_ANIMATION).is_empty());
        // With 0x400 and the right group type the animation is blended out.
        let log = logged(&mut e, |e| {
            e.call(0x0096_6ea0, &args![player, 0x400u32, 0u8]);
        });
        assert_eq!(calls_to(&log, ACTOR_MOVER_SET_FLAGS), [[mover, 0x400, 0]]);
        assert_eq!(
            calls_to(&log, ANIMATION_HOLDER_GET_GROUP),
            [[0x6000_0001, 4]]
        );
        assert_eq!(calls_to(&log, ANIM_GROUP_GET_TYPE), [[0x77]]);
        assert_eq!(calls_to(&log, ANIMATION_BLEND_OUT), [[0x6000_0002, 4, 0]]);
        // Another group type: no blend out.
        answer(&mut e, ANIM_GROUP_GET_TYPE, 0x10);
        let log = logged(&mut e, |e| {
            e.call(0x0096_6ea0, &args![player, 0x400u32, 0u8]);
        });
        assert!(calls_to(&log, ANIMATION_BLEND_OUT).is_empty());
        // No animation at all: the group is not even asked for.
        answer(&mut e, PLAYER_GET_ANIMATION, 0);
        let log = logged(&mut e, |e| {
            e.call(0x0096_6ea0, &args![player, 0x400u32, 0u8]);
        });
        assert!(calls_to(&log, ANIMATION_HOLDER_GET_GROUP).is_empty());
    }

    /// A player whose perceived list (`+0xd48`) holds one `ActorsNearPlayerData`
    /// entry per `(actor, hostile, line of sight only)`; returns the entries.
    fn perceived_world(
        e: &mut Engine,
        entries: &[(u32, u8, u8)],
    ) -> (Ptr<PlayerCharacter>, Vec<u32>) {
        list_accessors(e);
        let player = player_with(e, &[], &[]);
        let mut blocks = Vec::new();
        for &(actor, hostile, sight) in entries {
            let block = e.mem.alloc(8);
            e.mem.set_u32(block, actor);
            e.mem.set_u8(block + 4, hostile);
            e.mem.set_u8(block + 5, sight);
            blocks.push(block);
        }
        let list = list_of(e, &blocks);
        e.set(
            player,
            PlayerCharacter::pListofPercievedActors,
            Ptr::new(list),
        );
        (player, blocks)
    }

    const CALLEES_00966F20: &[u32] = &[
        GET_BASE_FORM,
        OPERATOR_NEW,
        NEAR_PLAYER_DATA_NEW,
        LIST_NEW,
        LIST_APPEND,
    ];

    /// An actor whose virtual `+0x22c` answers `second` and whose base form
    /// answers `first` to its subobject's virtual `+0x20`.
    fn noticed_actor(e: &mut Engine, first: u32, second: u32) -> u32 {
        answer(e, SLOT_A, first);
        answer(e, SLOT_B, second);
        let sub = object_with_slots(e, &[(0x20, SLOT_A)]);
        let base = e.mem.alloc(0x40);
        let vtable = e.mem.u32(sub);
        e.mem.set_u32(base + 0x30, vtable);
        answer(e, GET_BASE_FORM, base);
        object_with_slots(e, &[(0x22c, SLOT_B)])
    }

    #[test]
    fn a_noticed_actor_gets_a_new_entry_and_the_list_is_made() {
        let mut e = engine(CALLEES_00966F20);
        list_accessors(&mut e);
        let actor = noticed_actor(&mut e, 0, 0);
        let list_block = e.mem.alloc(8);
        let entry_block = e.mem.alloc(8);
        let mut blocks = vec![list_block, entry_block].into_iter();
        e.register_double(OPERATOR_NEW, move |_, _| int(blocks.next().unwrap()));
        e.register_double(LIST_NEW, |_, a| int(a[0]));
        e.register_double(NEAR_PLAYER_DATA_NEW, |_, a| int(a[0]));
        let player = e.new_object::<PlayerCharacter>();
        let seen = Rc::new(Cell::new(0u32));
        let remember = seen.clone();
        e.register_double(LIST_APPEND, move |e, a| {
            remember.set(e.mem.u32(a[1]));
            Ret::default()
        });
        let log = logged(&mut e, |e| {
            e.call(0x0096_6f20, &args![player, actor, 1u8, 0u8]);
        });
        assert_eq!(
            e.get(player, PlayerCharacter::pListofPercievedActors)
                .addr(),
            list_block
        );
        assert_eq!(e.mem.u32(entry_block), actor);
        assert_eq!(e.mem.u8(entry_block + 4), 1);
        assert_eq!(e.mem.u8(entry_block + 5), 0);
        assert_eq!(calls_to(&log, LIST_APPEND)[0][0], list_block);
        assert_eq!(seen.get(), entry_block);
        assert_eq!(calls_to(&log, LIST_NEW), [[list_block]]);
    }

    #[test]
    fn a_known_actor_is_updated_in_place_and_blocked_ones_are_ignored() {
        let mut e = engine(CALLEES_00966F20);
        let actor = noticed_actor(&mut e, 0, 0);
        let (player, entries) = perceived_world(&mut e, &[(actor, 0, 0)]);
        let log = logged(&mut e, |e| {
            e.call(0x0096_6f20, &args![player, actor, 1u8, 1u8]);
        });
        assert!(calls_to(&log, OPERATOR_NEW).is_empty());
        assert_eq!(e.mem.u8(entries[0] + 4), 1);
        assert_eq!(e.mem.u8(entries[0] + 5), 1);
        // The base form's virtual says the actor is not to be listed.
        let mut e = engine(CALLEES_00966F20);
        let actor = noticed_actor(&mut e, 1, 0);
        let (player, entries) = perceived_world(&mut e, &[(actor, 0, 0)]);
        e.call(0x0096_6f20, &args![player, actor, 1u8, 1u8]);
        assert_eq!(e.mem.u8(entries[0] + 4), 0);
        // So does the actor's own virtual.
        let mut e = engine(CALLEES_00966F20);
        let actor = noticed_actor(&mut e, 0, 1);
        let (player, entries) = perceived_world(&mut e, &[(actor, 0, 0)]);
        let log = logged(&mut e, |e| {
            e.call(0x0096_6f20, &args![player, actor, 1u8, 1u8]);
        });
        assert_eq!(e.mem.u8(entries[0] + 4), 0);
        assert_eq!(calls_to(&log, SLOT_B), [[actor, 0]]);
    }

    #[test]
    fn perceived_actor_lookups() {
        let mut e = engine(&[]);
        let (player, entries) =
            perceived_world(&mut e, &[(0x6000_0001, 1, 0), (0x6000_0002, 0, 1)]);
        assert_eq!(
            e.call(0x0096_7090, &args![player, 0x6000_0002u32]).u32(),
            entries[1]
        );
        assert_eq!(e.call(0x0096_7090, &args![player, 0x6000_0009u32]).u32(), 0);
        // Entries with a line of sight only flag count as not seen.
        assert!(e.call(0x0096_70f0, &args![player, 0x6000_0001u32]).bool());
        assert!(!e.call(0x0096_70f0, &args![player, 0x6000_0002u32]).bool());
        assert!(!e.call(0x0096_70f0, &args![player, 0x6000_0009u32]).bool());
        let none = e.new_object::<PlayerCharacter>();
        assert_eq!(e.call(0x0096_7090, &args![none, 0x6000_0001u32]).u32(), 0);
    }

    #[test]
    fn hostile_status_of_a_perceived_actor_can_be_changed() {
        let mut e = engine(&[]);
        let (player, entries) =
            perceived_world(&mut e, &[(0x6000_0001, 0, 0), (0x6000_0002, 0, 0)]);
        e.call(0x0096_7220, &args![player, 0x6000_0002u32, 1u8]);
        assert_eq!(e.mem.u8(entries[0] + 4), 0);
        assert_eq!(e.mem.u8(entries[1] + 4), 1);
        e.call(0x0096_7220, &args![player, 0x6000_0009u32, 1u8]);
        assert_eq!(e.mem.u8(entries[0] + 4), 0);
    }

    const CALLEES_00967160: &[u32] = &[
        ACTOR_GET_FORCE_NEXT_UPDATE,
        ACTOR_LINE_OF_SIGHT,
        ACTOR_GET_DETECTION_LEVEL,
    ];

    #[test]
    fn detection_by_non_teammates() {
        let mut e = engine(CALLEES_00967160);
        let (player, _) = perceived_world(
            &mut e,
            &[
                (0x6000_0001, 0, 0),
                (0x6000_0002, 0, 0),
                (0x6000_0003, 0, 0),
            ],
        );
        answer(&mut e, ACTOR_LINE_OF_SIGHT, 1);
        // Actor 1 is the excluded one, actor 2 has a forced update; only
        // actor 3 counts and is detected.
        e.register_double(ACTOR_GET_FORCE_NEXT_UPDATE, |_, a| {
            int(u32::from(a[0] == 0x6000_0002))
        });
        let seen = Rc::new(RefCell::new(Vec::new()));
        let remember = seen.clone();
        e.register_double(ACTOR_GET_DETECTION_LEVEL, move |e, a| {
            remember.borrow_mut().push((a[0], e.mem.u8(a[3])));
            int(2)
        });
        let log = logged(&mut e, |e| {
            assert!(e.call(0x0096_7160, &args![player, 0x6000_0001u32]).bool());
        });
        assert_eq!(*seen.borrow(), [(0x6000_0003, 1)]);
        assert_eq!(
            calls_to(&log, ACTOR_LINE_OF_SIGHT),
            [[0x6000_0003, 0, player.addr(), 1, 0, 0]]
        );
        assert_eq!(
            calls_to(&log, ACTOR_GET_DETECTION_LEVEL)[0][1..3].to_vec(),
            vec![0, player.addr()]
        );
        // Nobody detected: false.
        answer(&mut e, ACTOR_GET_DETECTION_LEVEL, 0);
        assert!(!e.call(0x0096_7160, &args![player, 0x6000_0001u32]).bool());
        // An empty list: false.
        let empty = e.new_object::<PlayerCharacter>();
        assert!(!e.call(0x0096_7160, &args![empty, 0x6000_0001u32]).bool());
    }

    #[test]
    fn the_perceived_list_is_emptied_and_deleted() {
        let mut e = engine(&[OPERATOR_DELETE, LIST_REMOVE_NODE, LIST_DELETE]);
        let (player, entries) =
            perceived_world(&mut e, &[(0x6000_0001, 0, 0), (0x6000_0002, 0, 0)]);
        // Removing a node moves the next node's item into it.
        e.register_double(LIST_REMOVE_NODE, |e, a| {
            let next = e.mem.u32(a[0] + 4);
            if next != 0 {
                let item = e.mem.u32(next);
                let after = e.mem.u32(next + 4);
                e.mem.set_u32(a[0], item);
                e.mem.set_u32(a[0] + 4, after);
            } else {
                e.mem.set_u32(a[0], 0);
            }
            Ret::default()
        });
        let list = e
            .get(player, PlayerCharacter::pListofPercievedActors)
            .addr();
        let log = logged(&mut e, |e| {
            e.call(0x0096_7290, &args![player]);
        });
        assert_eq!(
            calls_to(&log, OPERATOR_DELETE),
            [[entries[0]], [entries[1]]]
        );
        assert_eq!(calls_to(&log, LIST_DELETE), [[list, 1]]);
        assert!(e
            .get(player, PlayerCharacter::pListofPercievedActors)
            .is_null());
        // Nothing to do without a list.
        let log = logged(&mut e, |e| {
            e.call(0x0096_7290, &args![player]);
        });
        assert_eq!(log.len(), 1);
    }

    #[test]
    fn one_perceived_actor_is_removed() {
        let mut e = engine(&[OPERATOR_DELETE, LIST_REMOVE_NODE]);
        let (player, entries) =
            perceived_world(&mut e, &[(0x6000_0001, 0, 0), (0x6000_0002, 0, 0)]);
        let first = e
            .get(player, PlayerCharacter::pListofPercievedActors)
            .addr();
        let second = e.mem.u32(first + 4);
        let log = logged(&mut e, |e| {
            e.call(0x0096_7350, &args![player, 0x6000_0002u32]);
        });
        assert_eq!(calls_to(&log, OPERATOR_DELETE), [[entries[1]]]);
        assert_eq!(calls_to(&log, LIST_REMOVE_NODE), [[second]]);
        let log = logged(&mut e, |e| {
            e.call(0x0096_7350, &args![player, 0x6000_0009u32]);
        });
        assert!(calls_to(&log, OPERATOR_DELETE).is_empty());
    }

    #[test]
    fn pipboy_flag_check() {
        let mut e = engine(&[]);
        let object = e.mem.alloc(0x200);
        assert!(!e.call(0x0096_7700, &args![Ptr::<()>::new(object)]).bool());
        e.mem.set_u8(object + 0x161, 1);
        assert!(e.call(0x0096_7700, &args![Ptr::<()>::new(object)]).bool());
        e.mem.set_u8(object + 0x161, 0);
        e.mem.set_u8(object + 0x160, 1);
        assert!(e.call(0x0096_7700, &args![Ptr::<()>::new(object)]).bool());
    }

    const CALLEES_00967730: &[u32] = &[
        REFERENCE_GET_DISTANCE,
        CONVERSATION_GET_STARTER,
        CONVERSATION_GET_TARGET,
    ];

    /// The global player (conversation `current`) and the distances of the
    /// references 0x6000_0001 .. 3 to it.
    fn conversation_world(current: u32, distances: [f32; 3]) -> (Engine, Ptr<PlayerCharacter>) {
        let mut e = engine(CALLEES_00967730);
        e.set_global(FLOAT_MAX, f32::MAX);
        e.set_global(CONVERSATION_DISTANCE, 1200.0f64);
        let player = player_with(&mut e, &[], &[]);
        e.mem.set_u32(player.addr() + 0x208, current);
        e.register_double(REFERENCE_GET_DISTANCE, move |_, a| {
            float(f64::from(distances[(a[0] - 0x6000_0001) as usize]))
        });
        // The current conversation 0x6100_0000: starter 3, target 2 (none for
        // the other conversation).
        e.register_double(CONVERSATION_GET_STARTER, |_, a| {
            int(if a[0] == 0x6100_0000 { 0x6000_0003 } else { 0 })
        });
        e.register_double(CONVERSATION_GET_TARGET, |_, a| {
            int(if a[0] == 0x6100_0000 { 0x6000_0002 } else { 0 })
        });
        (e, player)
    }

    #[test]
    fn a_conversation_within_range_becomes_the_closest_one() {
        let (mut e, player) = conversation_world(0, [500.0, 900.0, 0.0]);
        e.call(
            0x0096_7730,
            &args![player, 0x6000_0001u32, 0x6000_0002u32, 0x6200_0000u32],
        );
        assert_eq!(e.mem.u32(player.addr() + 0x208), 0x6200_0000);
        // Beyond 1200: not taken.
        let (mut e, player) = conversation_world(0, [1500.0, 1300.0, 0.0]);
        e.call(
            0x0096_7730,
            &args![player, 0x6000_0001u32, 0x6000_0002u32, 0x6200_0000u32],
        );
        assert_eq!(e.mem.u32(player.addr() + 0x208), 0);
        // The smaller of the two distances counts.
        let (mut e, player) = conversation_world(0, [1500.0, 1100.0, 0.0]);
        e.call(
            0x0096_7730,
            &args![player, 0x6000_0001u32, 0x6000_0002u32, 0x6200_0000u32],
        );
        assert_eq!(e.mem.u32(player.addr() + 0x208), 0x6200_0000);
    }

    #[test]
    fn a_conversation_replaces_the_current_one_only_when_closer() {
        // The current conversation's closest member is at 600.
        let (mut e, player) = conversation_world(0x6100_0000, [250.0, 900.0, 600.0]);
        e.call(
            0x0096_7730,
            &args![player, 0x6000_0001u32, 0x6000_0002u32, 0x6200_0000u32],
        );
        assert_eq!(e.mem.u32(player.addr() + 0x208), 0x6200_0000);
        let (mut e, player) = conversation_world(0x6100_0000, [650.0, 900.0, 600.0]);
        e.call(
            0x0096_7730,
            &args![player, 0x6000_0001u32, 0x6000_0002u32, 0x6200_0000u32],
        );
        assert_eq!(e.mem.u32(player.addr() + 0x208), 0x6100_0000);
        // Equal distances count as closer.
        let (mut e, player) = conversation_world(0x6100_0000, [600.0, 900.0, 600.0]);
        e.call(
            0x0096_7730,
            &args![player, 0x6000_0001u32, 0x6000_0002u32, 0x6200_0000u32],
        );
        assert_eq!(e.mem.u32(player.addr() + 0x208), 0x6200_0000);
    }

    #[test]
    fn a_conversation_with_the_player_clears_the_closest_one() {
        let (mut e, player) = conversation_world(0x6100_0000, [250.0, 900.0, 600.0]);
        let me = player.addr();
        e.register_double(REFERENCE_GET_DISTANCE, |_, _| float(10.0));
        e.call(0x0096_7730, &args![me, me, 0x6000_0002u32, 0x6200_0000u32]);
        assert_eq!(e.mem.u32(me + 0x208), 0);
        e.mem.set_u32(me + 0x208, 0x6100_0000);
        e.call(0x0096_7730, &args![me, 0x6000_0002u32, me, 0x6200_0000u32]);
        assert_eq!(e.mem.u32(me + 0x208), 0);
    }

    const CALLEES_FURNITURE: &[u32] = &[
        GET_CHAR_CONTROLLER,
        FUNCTION_00C6FD20,
        CONTROLLER_BODY_AT_630,
        CONTROLLER_BODY_AT_638,
        BODY_LOOKUP_004AE750,
        FUNCTION_004B5A20,
        READ_FIELD_8,
        FIND_REFERENCE_FOR_3D,
        REFERENCE_IS_FURNITURE,
        BODY_GET_AV_OBJECT,
    ];

    /// The body-to-reference chain: each step adds one to its argument and
    /// `FindReferenceFor3D` knows the word 0x6400_0004.
    fn body_chain(e: &mut Engine) {
        answer(e, GET_CHAR_CONTROLLER, 0x6000_0000);
        e.register_double(BODY_LOOKUP_004AE750, |_, a| int(a[0] + 1));
        e.register_double(FUNCTION_004B5A20, |_, a| int(a[0] + 1));
        e.register_double(READ_FIELD_8, |_, a| int(a[0] + 1));
        e.register_double(FIND_REFERENCE_FOR_3D, |_, a| {
            int(if a[0] == 0x6400_0004 { 0x6500_0000 } else { 0 })
        });
    }

    #[test]
    fn standing_on_furniture() {
        let mut e = engine(CALLEES_FURNITURE);
        body_chain(&mut e);
        answer(&mut e, REFERENCE_IS_FURNITURE, 1);
        let player = e.new_object::<PlayerCharacter>();
        // The body 0x6400_0001 goes through three +1 steps to the word the
        // reference lookup knows (0x6400_0004).
        answer(&mut e, FUNCTION_00C6FD20, 0x6400_0001);
        let log = logged(&mut e, |e| {
            assert!(e.call(0x0096_78a0, &args![player]).bool());
        });
        assert_eq!(calls_to(&log, REFERENCE_IS_FURNITURE), [[0x6500_0000]]);
        // Not furniture.
        answer(&mut e, REFERENCE_IS_FURNITURE, 0);
        assert!(!e.call(0x0096_78a0, &args![player]).bool());
        // No body: the lookup is asked with 0 and finds nothing.
        answer(&mut e, FUNCTION_00C6FD20, 0);
        let log = logged(&mut e, |e| {
            assert!(!e.call(0x0096_78a0, &args![player]).bool());
        });
        assert_eq!(calls_to(&log, FIND_REFERENCE_FOR_3D), [[0]]);
        // No controller.
        answer(&mut e, GET_CHAR_CONTROLLER, 0);
        assert!(!e.call(0x0096_78a0, &args![player]).bool());
    }

    #[test]
    fn reference_of_the_second_body() {
        let mut e = engine(CALLEES_FURNITURE);
        body_chain(&mut e);
        let player = e.new_object::<PlayerCharacter>();
        answer(&mut e, FUNCTION_00C6FD20, 0x6400_0001);
        answer(&mut e, CONTROLLER_BODY_AT_630, 0x6400_0001);
        // Same body twice: nothing.
        assert_eq!(e.call(0x0096_7950, &args![player]).u32(), 0);
        // A different second body, three steps from the known word.
        answer(&mut e, CONTROLLER_BODY_AT_630, 0x6400_0001);
        answer(&mut e, FUNCTION_00C6FD20, 0x6400_0009);
        assert_eq!(e.call(0x0096_7950, &args![player]).u32(), 0x6500_0000);
        // Second body missing.
        answer(&mut e, CONTROLLER_BODY_AT_630, 0);
        assert_eq!(e.call(0x0096_7950, &args![player]).u32(), 0);
        // Its chain ends in no word.
        answer(&mut e, CONTROLLER_BODY_AT_630, 0x6400_0100);
        assert_eq!(e.call(0x0096_7950, &args![player]).u32(), 0);
        answer(&mut e, GET_CHAR_CONTROLLER, 0);
        assert_eq!(e.call(0x0096_7950, &args![player]).u32(), 0);
    }

    #[test]
    fn bumped_reference() {
        let mut e = engine(CALLEES_FURNITURE);
        body_chain(&mut e);
        let player = e.new_object::<PlayerCharacter>();
        answer(&mut e, CONTROLLER_BODY_AT_638, 0x6600_0000);
        // The scene object of the bumped body is the word the lookup knows.
        answer(&mut e, BODY_GET_AV_OBJECT, 0x6400_0004);
        // The reference whose virtual +0x100 decides.
        let reference = object_with_slots(&mut e, &[(0x100, SLOT_A)]);
        e.register_double(FIND_REFERENCE_FOR_3D, move |_, a| {
            int(if a[0] == 0x6400_0004 { reference } else { 0 })
        });
        answer(&mut e, SLOT_A, 1);
        assert_eq!(e.call(0x0096_7a00, &args![player]).u32(), reference);
        answer(&mut e, SLOT_A, 0);
        assert_eq!(e.call(0x0096_7a00, &args![player]).u32(), 0);
        // No bumped body: the lookup gets 0 and finds nothing.
        answer(&mut e, CONTROLLER_BODY_AT_638, 0);
        assert_eq!(e.call(0x0096_7a00, &args![player]).u32(), 0);
        answer(&mut e, GET_CHAR_CONTROLLER, 0);
        assert_eq!(e.call(0x0096_7a00, &args![player]).u32(), 0);
    }

    const CALLEES_009650A0: &[u32] = &[
        IMAGE_SPACE_DOF_TRIGGER,
        IMAGE_SPACE_INSTANCE_SET_ACTIVE,
        READ_FLOAT_AT_C,
        ACTOR_GET_IRON_SIGHTS,
        CLAMP_FLOAT_IN_PLACE,
        SET_FLOAT_AT_C,
        SETTING_FLOAT_VALUE,
        SETTING_FLOAT,
    ];

    /// The real clamp: `*value` limited to `minimum ..= maximum`.
    fn clamp_double(e: &mut Engine) {
        e.register_double(CLAMP_FLOAT_IN_PLACE, |e, a| {
            let value = e.mem.f32(a[0]);
            let minimum = f32::from_bits(a[1]);
            let maximum = f32::from_bits(a[2]);
            if value <= maximum {
                if value < minimum {
                    e.mem.set_f32(a[0], minimum);
                }
            } else {
                e.mem.set_f32(a[0], maximum);
            }
            Ret::default()
        });
    }

    /// `READ_FLOAT_AT_C` answering the frame time for the frame time object
    /// and the strength kept per image space instance otherwise.
    fn strengths(e: &mut Engine, frame_time: f64, instances: &[(u32, f64)]) {
        let table: HashMap<u32, f64> = instances.iter().copied().collect();
        e.register_double(READ_FLOAT_AT_C, move |_, a| {
            if a[0] == FRAME_TIME_OBJECT {
                float(frame_time)
            } else {
                float(table.get(&a[0]).copied().unwrap_or(0.0))
            }
        });
    }

    #[test]
    fn iron_sights_instance_is_made_and_the_strength_moves_to_the_limit() {
        let mut e = engine(CALLEES_009650A0);
        float_settings(
            &mut e,
            &[
                (SETTING_011CFD70, 8.0),
                (SETTING_011CF8C8, 9.0),
                (SETTING_011CE6B4, 0.5),
                (SETTING_011CE4D4, 3.0),
                (SETTING_011CEDF4, 0.5),
            ],
        );
        clamp_double(&mut e);
        let instance = e.mem.alloc(0x80);
        answer(&mut e, IMAGE_SPACE_DOF_TRIGGER, instance);
        strengths(&mut e, 0.1, &[(instance, 0.0)]);
        answer(&mut e, ACTOR_GET_IRON_SIGHTS, 1);
        answer_float(&mut e, SETTING_FLOAT_VALUE, 3.0);
        let player = e.new_object::<PlayerCharacter>();
        let log = logged(&mut e, |e| {
            e.call(0x0096_50a0, &args![player]);
        });
        assert_eq!(
            calls_to(&log, IMAGE_SPACE_DOF_TRIGGER),
            [[
                9.0f32.to_bits(),
                8.0f32.to_bits(),
                4,
                0.0f32.to_bits(),
                0.0f32.to_bits(),
                0.0f32.to_bits()
            ]]
        );
        assert_eq!(
            calls_to(&log, IMAGE_SPACE_INSTANCE_SET_ACTIVE),
            [[instance, 1]]
        );
        assert_eq!(
            e.get(player, PlayerCharacter::pIronsightsDOFInstance)
                .addr(),
            instance
        );
        // Target 0.5, strength 0 plus 0.1 / 0.5 clamped into 0 ..= 0.5.
        assert_eq!(
            calls_to(&log, SET_FLOAT_AT_C),
            [[instance, 0.2f32.to_bits()]]
        );
        assert_eq!(
            calls_to(&log, CLAMP_FLOAT_IN_PLACE)[0][1..],
            [0.0f32.to_bits(), 0.5f32.to_bits()]
        );
        assert_eq!(e.global::<u8>(IRON_SIGHTS_ACTIVE), 1);
        assert_eq!(e.mem.f32(instance + 0x28), 3.0);
    }

    #[test]
    fn iron_sights_strength_falls_back_without_the_sights() {
        let mut e = engine(CALLEES_009650A0);
        float_settings(&mut e, &[(SETTING_011CE6B4, 0.5), (SETTING_011CEDF4, 0.5)]);
        clamp_double(&mut e);
        let instance = e.mem.alloc(0x80);
        strengths(&mut e, 0.1, &[(instance, 0.3)]);
        answer(&mut e, ACTOR_GET_IRON_SIGHTS, 1);
        let player = e.new_object::<PlayerCharacter>();
        e.set(
            player,
            PlayerCharacter::pIronsightsDOFInstance,
            Ptr::new(instance),
        );
        e.set_global(IRON_SIGHTS_ACTIVE, 1u8);
        // Third person: the sights do not count.
        e.set(player, PlayerCharacter::b3rdPerson, 1);
        let log = logged(&mut e, |e| {
            e.call(0x0096_50a0, &args![player]);
        });
        assert!(calls_to(&log, IMAGE_SPACE_DOF_TRIGGER).is_empty());
        assert_eq!(e.global::<u8>(IRON_SIGHTS_ACTIVE), 0);
        // 0.3 - 0.2 = 0.1.
        let stored = calls_to(&log, SET_FLOAT_AT_C);
        assert_eq!(stored.len(), 1);
        assert!((f32::from_bits(stored[0][1]) - 0.1).abs() < 1e-6);
    }

    #[test]
    fn iron_sights_strength_at_its_target_is_not_stepped() {
        let mut e = engine(CALLEES_009650A0);
        float_settings(&mut e, &[(SETTING_011CE6B4, 0.5)]);
        let instance = e.mem.alloc(0x80);
        strengths(&mut e, 0.1, &[(instance, 0.0)]);
        answer(&mut e, ACTOR_GET_IRON_SIGHTS, 0);
        let player = e.new_object::<PlayerCharacter>();
        e.set(
            player,
            PlayerCharacter::pIronsightsDOFInstance,
            Ptr::new(instance),
        );
        let log = logged(&mut e, |e| {
            e.call(0x0096_50a0, &args![player]);
        });
        assert!(calls_to(&log, CLAMP_FLOAT_IN_PLACE).is_empty());
        assert_eq!(calls_to(&log, SET_FLOAT_AT_C), [[instance, 0]]);
    }

    const CALLEES_00965270: &[u32] = &[
        IMAGE_SPACE_DOF_TRIGGER,
        IMAGE_SPACE_DRB_TRIGGER,
        NI_POINT2_NEW,
        IMAGE_SPACE_INSTANCE_SET_ACTIVE,
        READ_FLOAT_AT_C,
        READ_FIELD_8,
        VECTOR_SUBTRACT,
        VECTOR_LENGTH,
        CLAMP_FLOAT_IN_PLACE,
        SET_FLOAT_AT_C,
        SET_FLOAT_AT_20,
        SETTING_FLOAT,
    ];

    fn vats_settings(e: &mut Engine) {
        float_settings(
            e,
            &[
                (SETTING_011CFE20, 2.0),
                (SETTING_011CF38C, 3.0),
                (SETTING_011CEC04, 4.0),
                (SETTING_011CFCC8, 0.8),
                (SETTING_011CE924, 0.6),
                (SETTING_011CF4F8, 0.5),
            ],
        );
    }

    #[test]
    fn vats_instances_are_made_when_missing() {
        let mut e = engine(CALLEES_00965270);
        vats_settings(&mut e);
        e.set_global(POINT_SCALE_01016248, 0.25f32);
        let dof = e.mem.alloc(0x80);
        let drb = e.mem.alloc(0x80);
        answer(&mut e, IMAGE_SPACE_DOF_TRIGGER, dof);
        answer(&mut e, IMAGE_SPACE_DRB_TRIGGER, drb);
        strengths(&mut e, 0.1, &[]);
        let player = e.new_object::<PlayerCharacter>();
        let log = logged(&mut e, |e| {
            e.call(0x0096_5270, &args![player]);
        });
        assert_eq!(
            calls_to(&log, IMAGE_SPACE_DOF_TRIGGER),
            [[
                0,
                2.0f32.to_bits(),
                0,
                0.0f32.to_bits(),
                0.0f32.to_bits(),
                0.0f32.to_bits()
            ]]
        );
        let point = calls_to(&log, NI_POINT2_NEW)[0][0];
        assert_eq!(
            calls_to(&log, NI_POINT2_NEW),
            [[point, 0.25f32.to_bits(), 0.25f32.to_bits()]]
        );
        assert_eq!(
            calls_to(&log, IMAGE_SPACE_DRB_TRIGGER),
            [[
                0,
                2.0f32.to_bits(),
                0,
                0.0f32.to_bits(),
                4.0f32.to_bits(),
                3.0f32.to_bits(),
                0.0f32.to_bits(),
                0.0f32.to_bits(),
                point,
                0.0f32.to_bits()
            ]]
        );
        assert_eq!(
            calls_to(&log, IMAGE_SPACE_INSTANCE_SET_ACTIVE),
            [[dof, 1], [drb, 1]]
        );
        assert_eq!(e.get(player, PlayerCharacter::pVatsDOFInstance).addr(), dof);
        assert_eq!(e.get(player, PlayerCharacter::pVatsDRBInstance).addr(), drb);
    }

    #[test]
    fn vats_strengths_follow_the_targets_and_the_fade_is_the_distance() {
        let mut e = engine(CALLEES_00965270);
        vats_settings(&mut e);
        let dof = e.mem.alloc(0x80);
        let drb = e.mem.alloc(0x80);
        strengths(&mut e, 0.1, &[(dof, 0.0), (drb, 0.7)]);
        clamp_double(&mut e);
        // Mode 2 and a target object whose position the virtual +0x1f4 gives.
        e.register_double(READ_FIELD_8, |_, a| {
            int(if a[0] == OBJECT_011F2250 { 2 } else { 0 })
        });
        let target = object_with_slots(&mut e, &[(0x1f4, SLOT_A)]);
        e.set_global(POINTER_011F21CC, target);
        answer(&mut e, SLOT_A, 0x6000_0010);
        let player = player_with(&mut e, &[(0x1f4, SLOT_B)], &[]);
        answer(&mut e, SLOT_B, 0x6000_0020);
        e.register_double(VECTOR_SUBTRACT, |_, a| int(a[1]));
        answer_float(&mut e, VECTOR_LENGTH, 5.0);
        e.set(player, PlayerCharacter::pVatsDOFInstance, Ptr::new(dof));
        e.set(player, PlayerCharacter::pVatsDRBInstance, Ptr::new(drb));
        let log = logged(&mut e, |e| {
            e.call(0x0096_5270, &args![player]);
        });
        let difference = calls_to(&log, VECTOR_SUBTRACT)[0].clone();
        assert_eq!(difference[0], 0x6000_0010);
        assert_eq!(difference[2], 0x6000_0020);
        // Depth of field climbs to 0.6, the DRB falls from 0.7 towards 0.8
        // (0.7 + 0.2 clamped to its limit 0.8 gives 0.8).
        let stored = calls_to(&log, SET_FLOAT_AT_C);
        assert_eq!(stored[0], vec![dof, 0.2f32.to_bits()]);
        assert_eq!(stored[1], vec![drb, 0.8f32.to_bits()]);
        assert_eq!(calls_to(&log, SET_FLOAT_AT_20), [[dof, 5.0f32.to_bits()]]);
        assert_eq!(e.mem.f32(drb + 0x38), 5.0);
    }

    #[test]
    fn vats_targets_are_zero_outside_the_modes() {
        let mut e = engine(CALLEES_00965270);
        vats_settings(&mut e);
        let dof = e.mem.alloc(0x80);
        let drb = e.mem.alloc(0x80);
        strengths(&mut e, 0.1, &[(dof, 0.5), (drb, 0.0)]);
        clamp_double(&mut e);
        let player = e.new_object::<PlayerCharacter>();
        e.set(player, PlayerCharacter::pVatsDOFInstance, Ptr::new(dof));
        e.set(player, PlayerCharacter::pVatsDRBInstance, Ptr::new(drb));
        let log = logged(&mut e, |e| {
            e.call(0x0096_5270, &args![player]);
        });
        // The depth of field falls by 0.2 to 0.3; the DRB is already at 0.
        let stored = calls_to(&log, SET_FLOAT_AT_C);
        assert_eq!(stored.len(), 2);
        assert!((f32::from_bits(stored[0][1]) - 0.3).abs() < 1e-6);
        assert_eq!(stored[1], vec![drb, 0]);
        assert_eq!(calls_to(&log, CLAMP_FLOAT_IN_PLACE).len(), 1);
        assert!(calls_to(&log, VECTOR_SUBTRACT).is_empty());
    }

    const CALLEES_009673D0: &[u32] = &[
        FUNCTION_00877720,
        CONTROL_QUERY,
        SETTING_FLOAT,
        INTERFACE_GET_PIPBOY,
        PIPBOY_IS_ACTIVE,
        GET_LIGHT_TARGET_OWNER,
        MAGIC_TARGET_IS_SPELL_TARGET,
        PIPBOY_SHOW_LIGHT_EFFECT,
        AUDIO_INSTANCE,
        AUDIO_GET_SOUND_HANDLE_BY_NAME,
        SOUND_HANDLE_PLAY,
        EMPTY_DESTRUCTOR,
        MAGIC_TARGET_DISPEL,
        PLAYER_HAS_CONTROL_BITS,
    ];

    /// The menu mode button world: `held` says for the control states 0, 1
    /// and 2 whether the control query answers yes; the Pipboy manager is
    /// `active` and has `flags` in its two flag bytes; the threshold setting
    /// is 1.0.
    fn menu_button_world(
        held: [bool; 3],
        active: bool,
        flags: u8,
        spell_target: bool,
        player_bits: bool,
    ) -> (Engine, Ptr<PlayerCharacter>, u32) {
        let mut e = engine(CALLEES_009673D0);
        float_settings(&mut e, &[(SETTING_011CD098, 1.0)]);
        let pipboy = e.mem.alloc(0x200);
        e.mem.set_u8(pipboy + 0x160, flags);
        answer(&mut e, INTERFACE_GET_PIPBOY, pipboy);
        answer(&mut e, PIPBOY_IS_ACTIVE, u32::from(active));
        e.register_double(CONTROL_QUERY, move |_, a| {
            int(u32::from(held[a[2] as usize]))
        });
        answer(&mut e, FUNCTION_00877720, 0x6000_0001);
        answer(&mut e, GET_LIGHT_TARGET_OWNER, 0x6100_0000);
        answer(
            &mut e,
            MAGIC_TARGET_IS_SPELL_TARGET,
            u32::from(spell_target),
        );
        answer(&mut e, PLAYER_HAS_CONTROL_BITS, u32::from(player_bits));
        answer(&mut e, AUDIO_INSTANCE, 0x6200_0000);
        e.register_double(AUDIO_GET_SOUND_HANDLE_BY_NAME, |_, a| int(a[1]));
        answer(&mut e, SLOT_A, 0);
        let caster = object_with_slots(&mut e, &[(0, SLOT_A)]);
        let player = player_with(&mut e, &[], &[]);
        let caster_vtable = e.mem.u32(caster);
        e.mem
            .set_u32(player.addr() + MAGIC_CASTER_OFFSET, caster_vtable);
        (e, player, pipboy)
    }

    #[test]
    fn holding_the_button_in_state_two_below_the_threshold_clicks_it() {
        let (mut e, player, _) = menu_button_world([false, false, true], true, 0, false, false);
        e.set(player, PlayerCharacter::fMenuModeButtonTimer, 0.5);
        let log = logged(&mut e, |e| {
            e.call(0x0096_73d0, &args![player, 0.25f32]);
        });
        assert_eq!(e.get(player, PlayerCharacter::bMenuModeButtonClicked), 1);
        assert_eq!(e.get(player, PlayerCharacter::fMenuModeButtonTimer), 0.5);
        assert_eq!(calls_to(&log, CONTROL_QUERY), [[0x6000_0001, 0xe, 2]]);
        assert!(calls_to(&log, PIPBOY_SHOW_LIGHT_EFFECT).is_empty());
    }

    #[test]
    fn releasing_the_button_clears_the_click_the_timer_and_the_fired_byte() {
        let (mut e, player, _) = menu_button_world([false; 3], true, 0, false, false);
        e.set(player, PlayerCharacter::fMenuModeButtonTimer, 0.75);
        e.set(player, PlayerCharacter::bMenuModeButtonClicked, 1);
        e.set_global(MENU_MODE_BUTTON_FIRED, 1u8);
        e.call(0x0096_73d0, &args![player, 0.25f32]);
        assert_eq!(e.get(player, PlayerCharacter::bMenuModeButtonClicked), 0);
        assert_eq!(e.get(player, PlayerCharacter::fMenuModeButtonTimer), 0.0);
        assert_eq!(e.global::<u8>(MENU_MODE_BUTTON_FIRED), 0);
    }

    #[test]
    fn a_long_press_switches_the_pipboy_light_off_when_it_is_a_spell_target() {
        let (mut e, player, pipboy) = menu_button_world([false, true, false], true, 0, true, false);
        e.set(player, PlayerCharacter::fMenuModeButtonTimer, 0.75);
        let log = logged(&mut e, |e| {
            e.call(0x0096_73d0, &args![player, 0.5f32]);
        });
        assert_eq!(e.get(player, PlayerCharacter::fMenuModeButtonTimer), 1.25);
        assert_eq!(
            calls_to(&log, MAGIC_TARGET_IS_SPELL_TARGET),
            [[player.addr() + 0x94, 0x6100_0018, 1]]
        );
        assert_eq!(
            calls_to(&log, PIPBOY_SHOW_LIGHT_EFFECT),
            [[pipboy, 1, 0, 1], [pipboy, 0, 0, 1]]
        );
        assert_eq!(
            calls_to(&log, AUDIO_GET_SOUND_HANDLE_BY_NAME)[0][2..],
            [PIPBOY_LIGHT_OFF_SOUND, 0x121]
        );
        assert_eq!(calls_to(&log, SOUND_HANDLE_PLAY).len(), 1);
        assert_eq!(calls_to(&log, EMPTY_DESTRUCTOR).len(), 1);
        assert_eq!(
            calls_to(&log, MAGIC_TARGET_DISPEL),
            [[player.addr() + 0x94, 0x6100_0018, 0, 0]]
        );
        assert_eq!(e.global::<u8>(MENU_MODE_BUTTON_FIRED), 1);
        assert!(calls_to(&log, SLOT_A).is_empty());
    }

    #[test]
    fn a_long_press_switches_the_pipboy_light_on_through_the_caster() {
        let (mut e, player, pipboy) =
            menu_button_world([true, false, false], true, 0, false, false);
        e.set(player, PlayerCharacter::fMenuModeButtonTimer, 0.75);
        let log = logged(&mut e, |e| {
            e.call(0x0096_73d0, &args![player, 0.5f32]);
        });
        assert_eq!(
            calls_to(&log, PIPBOY_SHOW_LIGHT_EFFECT),
            [[pipboy, 1, 1, 1], [pipboy, 0, 1, 1]]
        );
        assert_eq!(
            calls_to(&log, AUDIO_GET_SOUND_HANDLE_BY_NAME)[0][2..],
            [PIPBOY_LIGHT_ON_SOUND, 0x121]
        );
        assert_eq!(
            calls_to(&log, SLOT_A),
            [[player.addr() + MAGIC_CASTER_OFFSET, 0x6100_0000, 0]]
        );
        assert!(calls_to(&log, MAGIC_TARGET_DISPEL).is_empty());
        assert_eq!(e.global::<u8>(MENU_MODE_BUTTON_FIRED), 1);
    }

    #[test]
    fn a_long_press_with_a_player_state_bit_only_marks_the_press_as_handled() {
        let (mut e, player, _) = menu_button_world([true, false, false], true, 0, false, true);
        e.set(player, PlayerCharacter::fMenuModeButtonTimer, 0.75);
        let log = logged(&mut e, |e| {
            e.call(0x0096_73d0, &args![player, 0.5f32]);
        });
        assert_eq!(
            calls_to(&log, PLAYER_HAS_CONTROL_BITS),
            [[player.addr(), 4]]
        );
        assert!(calls_to(&log, PIPBOY_SHOW_LIGHT_EFFECT).is_empty());
        assert_eq!(e.global::<u8>(MENU_MODE_BUTTON_FIRED), 1);
        // The press is not handled twice.
        let log = logged(&mut e, |e| {
            e.call(0x0096_73d0, &args![player, 0.5f32]);
        });
        assert!(calls_to(&log, PLAYER_HAS_CONTROL_BITS).is_empty());
    }

    #[test]
    fn the_pipboy_light_effects_are_cleared_when_its_flags_stay_set_without_the_pipboy() {
        let (mut e, player, pipboy) = menu_button_world([false; 3], false, 1, false, false);
        let log = logged(&mut e, |e| {
            e.call(0x0096_73d0, &args![player, 0.5f32]);
        });
        assert_eq!(
            calls_to(&log, PIPBOY_SHOW_LIGHT_EFFECT),
            [[pipboy, 1, 0, 1], [pipboy, 0, 0, 1]]
        );
        assert_eq!(
            calls_to(&log, MAGIC_TARGET_DISPEL),
            [[player.addr() + 0x94, 0x6100_0018, 0, 0]]
        );
        // With the Pipboy in use nothing happens.
        let (mut e, player, _) = menu_button_world([false; 3], true, 1, false, false);
        let log = logged(&mut e, |e| {
            e.call(0x0096_73d0, &args![player, 0.5f32]);
        });
        assert!(calls_to(&log, PIPBOY_SHOW_LIGHT_EFFECT).is_empty());
    }

    const SLOT_G: u32 = 0x7000_0007;
    const SLOT_H: u32 = 0x7000_0008;
    const SLOT_I: u32 = 0x7000_0009;
    const SLOT_J: u32 = 0x7000_000a;

    /// Every callee of `PlayerCharacter::UpdateAutoAimActor` and of the two
    /// image space updates it runs at its end.
    fn auto_aim_callees() -> Vec<u32> {
        let mut all = vec![
            GET_CAMERA_HOLDER,
            READ_DWORD_AT_AC,
            NI_POINT2_NEW,
            ADD_4_TO_ADDRESS,
            FUNCTION_005BE5C0,
            PICK_RESULT_GET,
            GET_CURRENT_PROCESS_TYPE,
            VECTOR_SUBTRACT,
            VECTOR_LENGTH,
            IDENTITY_006815C0,
            NODE_GET_WORLD_BOUND,
            SCREEN_SPACE_BOUND_BOUND,
            ACTOR_LINE_OF_SIGHT,
            FUNCTION_004A3C20,
            FUNCTION_0062A190,
            FUNCTION_0059CEB0,
            FUNCTION_00416870,
            READ_FIELD_8,
            FUNCTION_0070C3B0,
            FUNCTION_004A0C90,
            MOBILE_OBJECT_GET_LOOKING,
            MATRIX_MAKE_X_ROTATION,
            MATRIX_MULTIPLY,
            FUNCTION_004B4500,
            VECTOR_ADD,
            FUNCTION_00461130,
            FUNCTION_005DF560,
            FUNCTION_005DF510,
            FUNCTION_005DF5B0,
            FUNCTION_004A3DA0,
            FUNCTION_004A3EB0,
            FUNCTION_00931ED0,
            FUNCTION_008C71B0,
            FUNCTION_004A39F0,
            FUNCTION_004A3A20,
            FUNCTION_0059CE80,
            FUNCTION_004A3F70,
            FUNCTION_00888960,
            WORLD_SET_PICK_COUNT_OVERRIDE,
            TES_PICK,
            FIND_REFERENCE_FOR_3D,
            SMART_POINTER_ASSIGN,
            FUNCTION_0045BB20,
            PLAYER_GET_PROCESS,
            WEAPON_IS_MELEE_TYPE,
            FUNCTION_004B71D0,
            ADD_8C_TO_ADDRESS,
            GLOBAL_OBJECT_GETTER,
            READ_FLOAT_AT_40,
            READ_FLOAT_AT_38,
            WINDOW_POINT_TO_RAY,
            FUNCTION_008CFBC0,
            FUNCTION_008B3EF0,
            FUNCTION_007F3B40,
            GET_BASE_FORM,
            BODY_PART_DATA_GET_BODY_PART,
            FUNCTION_005E5190,
            ACTOR_GET_CURRENT_WEAPON,
            FUNCTION_0059CEE0,
        ];
        all.extend_from_slice(CALLEES_009650A0);
        all.extend_from_slice(CALLEES_00965270);
        all
    }

    /// The auto aim fixture: a player (3D node, position, a perceived list
    /// with the one `actor`) scanning a process list of that one actor.
    struct AutoAim {
        e: Engine,
        player: Ptr<PlayerCharacter>,
        actor: u32,
    }

    const AIM_NODE: u32 = 0x6000_0003;
    const AIM_CAMERA: u32 = 0x6000_0006;

    fn auto_aim_world(hostile: u8, distance: f64) -> AutoAim {
        let mut e = engine(&auto_aim_callees());
        list_accessors(&mut e);
        e.set_global(DOUBLE_MINUS_ONE, -1.0f64);
        e.set_global(DOUBLE_ONE, 1.0f64);
        e.set_global(AIM_BOX_SCALE_01023B88, 1.0f64);
        float_settings(
            &mut e,
            &[(SETTING_011CED64, 5000.0), (SETTING_011CEFA4, 0.5)],
        );
        let own_position = e.mem.alloc(12);
        let actor_position = e.mem.alloc(12);
        let radius = e.mem.alloc(4);
        e.mem.set_f32(radius, 2.0);
        answer(&mut e, SLOT_A, AIM_NODE);
        answer(&mut e, SLOT_B, own_position);
        answer(&mut e, SLOT_C, 1);
        answer(&mut e, SLOT_D, 0);
        answer(&mut e, SLOT_F, actor_position);
        answer(&mut e, SLOT_G, radius);
        answer(&mut e, SLOT_H, 0);
        let player = player_with(
            &mut e,
            &[(0x1d0, SLOT_A), (0x1f4, SLOT_B)],
            &[(0x148, SLOT_H)],
        );
        let actor = object_with_slots(
            &mut e,
            &[
                (0x100, SLOT_C),
                (0x22c, SLOT_D),
                (0x1d0, SLOT_A),
                (0x1f4, SLOT_F),
                (0x1dc, SLOT_G),
            ],
        );
        let entry = e.mem.alloc(8);
        e.mem.set_u32(entry, actor);
        e.mem.set_u8(entry + 4, hostile);
        let list = list_of(&mut e, &[entry]);
        e.set(
            player,
            PlayerCharacter::pListofPercievedActors,
            Ptr::new(list),
        );
        let process = object_with_slots(&mut e, &[(0x148, SLOT_H)]);
        answer(&mut e, PLAYER_GET_PROCESS, process);
        answer(&mut e, ADD_4_TO_ADDRESS, 0x6000_0010);
        answer(&mut e, FUNCTION_005BE5C0, 1);
        answer(&mut e, PICK_RESULT_GET, actor);
        for address in [
            VECTOR_SUBTRACT,
            VECTOR_ADD,
            FUNCTION_004B4500,
            MATRIX_MULTIPLY,
            FUNCTION_0045BB20,
        ] {
            e.register_double(address, |_, a| int(a[1]));
        }
        answer_float(&mut e, VECTOR_LENGTH, distance);
        let bound = e.mem.alloc(16);
        answer(&mut e, NODE_GET_WORLD_BOUND, bound);
        answer(&mut e, GET_CAMERA_HOLDER, 0x6000_0005);
        answer(&mut e, READ_DWORD_AT_AC, AIM_CAMERA);
        answer(&mut e, ACTOR_LINE_OF_SIGHT, 1);
        let matrix = e.mem.alloc(36);
        answer(&mut e, FUNCTION_00461130, matrix);
        let position = e.mem.alloc(12);
        answer(&mut e, FUNCTION_005DF5B0, position);
        let dof = e.mem.alloc(0x80);
        let drb = e.mem.alloc(0x80);
        answer(&mut e, IMAGE_SPACE_DOF_TRIGGER, dof);
        answer(&mut e, IMAGE_SPACE_DRB_TRIGGER, drb);
        AutoAim { e, player, actor }
    }

    #[test]
    fn auto_aim_does_nothing_without_a_3d_node() {
        let mut world = auto_aim_world(1, 100.0);
        answer(&mut world.e, SLOT_A, 0);
        let player = world.player;
        let log = logged(&mut world.e, |e| {
            e.call(0x0096_4260, &args![player]);
        });
        // The call itself and the one virtual.
        assert_eq!(log.len(), 2);
    }

    #[test]
    fn auto_aim_picks_the_visible_hostile_actor_in_range() {
        let mut world = auto_aim_world(1, 100.0);
        let (player, actor) = (world.player, world.actor);
        let log = logged(&mut world.e, |e| {
            e.call(0x0096_4260, &args![player]);
        });
        let e = &mut world.e;
        assert_eq!(
            e.get(player, PlayerCharacter::pPlayersTargetActor).addr(),
            actor
        );
        assert_eq!(e.get(player, PlayerCharacter::pAutoAimActor).addr(), actor);
        assert_eq!(e.get(player, PlayerCharacter::bTarget3DDistant), 0);
        // The box is the setting scaled by the double: -0.5 and 0.5.
        let point = calls_to(&log, NI_POINT2_NEW)[0].clone();
        assert_eq!(point[1..], [(-0.5f32).to_bits(), 0.5f32.to_bits()]);
        // Both screen space bounds are asked for with the camera, the second
        // one with the radius the actor's virtual +0x1dc gave.
        let bounds = calls_to(&log, SCREEN_SPACE_BOUND_BOUND);
        assert_eq!(bounds.len(), 2);
        assert_eq!(bounds[0][0], AIM_CAMERA);
        assert_eq!(bounds[1][1], bounds[0][1]);
        assert_eq!(
            calls_to(&log, SET_FLOAT_AT_C)[0],
            vec![bounds[0][1], 2.0f32.to_bits()]
        );
        assert_eq!(
            calls_to(&log, ACTOR_LINE_OF_SIGHT),
            [[player.addr(), 0, actor, 1, 0, 0]]
        );
        let query = calls_to(&log, FUNCTION_0062A190)[0].clone();
        assert_eq!(query[1..], [0x25, actor]);
        assert_eq!(
            calls_to(&log, FUNCTION_0059CEB0),
            [[calls_to(&log, FUNCTION_004A3C20)[0][0], query[0]]]
        );
        assert_eq!(calls_to(&log, FUNCTION_0059CEE0), [[query[0]]]);
    }

    #[test]
    fn auto_aim_keeps_a_friendly_actor_as_target_but_not_as_the_best_one() {
        let mut world = auto_aim_world(0, 100.0);
        let (player, actor) = (world.player, world.actor);
        world.e.call(0x0096_4260, &args![player]);
        let e = &mut world.e;
        assert_eq!(
            e.get(player, PlayerCharacter::pPlayersTargetActor).addr(),
            actor
        );
        assert!(e.get(player, PlayerCharacter::pAutoAimActor).is_null());
    }

    #[test]
    fn auto_aim_ignores_actors_out_of_range_or_off_screen() {
        let mut world = auto_aim_world(1, 6000.0);
        let player = world.player;
        let log = logged(&mut world.e, |e| {
            e.call(0x0096_4260, &args![player]);
        });
        assert!(calls_to(&log, SCREEN_SPACE_BOUND_BOUND).is_empty());
        assert!(world
            .e
            .get(player, PlayerCharacter::pPlayersTargetActor)
            .is_null());

        // On range, but the second bound covers the whole screen.
        let mut world = auto_aim_world(1, 100.0);
        let player = world.player;
        let mut calls = 0;
        world
            .e
            .register_double(SCREEN_SPACE_BOUND_BOUND, move |e, a| {
                calls += 1;
                if calls == 2 {
                    e.mem.set_f32(a[2], -1.0);
                    e.mem.set_f32(a[3], 1.0);
                }
                Ret::default()
            });
        let log = logged(&mut world.e, |e| {
            e.call(0x0096_4260, &args![player]);
        });
        assert!(calls_to(&log, ACTOR_LINE_OF_SIGHT).is_empty());
        assert!(world
            .e
            .get(player, PlayerCharacter::pPlayersTargetActor)
            .is_null());
        assert!(world
            .e
            .get(player, PlayerCharacter::pAutoAimActor)
            .is_null());
    }

    #[test]
    fn auto_aim_rechecks_the_picked_actor_with_a_camera_ray() {
        let mut world = auto_aim_world(1, 100.0);
        let (player, actor) = (world.player, world.actor);
        let e = &mut world.e;
        let picked = 0x6000_0009u32;
        answer(e, TES_PICK, picked);
        let second = object_with_slots(e, &[(0x100, SLOT_I)]);
        answer(e, SLOT_I, 1);
        e.register_double(FIND_REFERENCE_FOR_3D, move |_, a| {
            int(match a[0] {
                0x6000_0009 => actor,
                0x6b00_0000 => second,
                _ => 0,
            })
        });
        let screen = e.mem.alloc(0x40);
        e.mem.set_u32(screen + 0x18, 50);
        answer(e, GLOBAL_OBJECT_GETTER, screen);
        answer_float(e, READ_FLOAT_AT_40, 10.75);
        answer_float(e, READ_FLOAT_AT_38, 20.5);
        answer(e, FUNCTION_008CFBC0, 0x6b00_0000);
        let log = logged(e, |e| {
            e.call(0x0096_4260, &args![player]);
        });
        let ray = calls_to(&log, WINDOW_POINT_TO_RAY)[0].clone();
        assert_eq!(ray[..3], [AIM_CAMERA, 20, 10]);
        assert_eq!(ray[5], 0);
        let cast = calls_to(&log, FUNCTION_008CFBC0)[0].clone();
        assert_eq!(cast[0], AIM_NODE);
        assert_eq!(cast[8], 0);
        assert_eq!(
            e.get(player, PlayerCharacter::pPlayersTargetActor).addr(),
            second
        );
        assert_eq!(e.get(player, PlayerCharacter::pAutoAimActor).addr(), actor);
        assert_eq!(
            calls_to(&log, SMART_POINTER_ASSIGN),
            [[player.addr() + 0xd3c, picked]]
        );
        // The picked actor is 100 away, the screen object's word is 50.
        assert_eq!(e.get(player, PlayerCharacter::bTarget3DDistant), 1);
    }

    #[test]
    fn auto_aim_in_mode_two_records_the_body_part_the_ray_hits() {
        let mut world = auto_aim_world(1, 100.0);
        let player = world.player;
        let e = &mut world.e;
        e.register_double(READ_FIELD_8, |_, a| {
            int(if a[0] == OBJECT_011F2250 {
                2
            } else {
                0x6900_0000
            })
        });
        let target = object_with_slots(e, &[(0x1d0, SLOT_A), (0x1f4, SLOT_F)]);
        e.set_global(POINTER_011F21CC, target);
        answer(e, SLOT_H, 0x6800_0000);
        e.set_global(AIM_DISTANCE_PAD_0102E430, 128.0f64);
        let hit = object_with_slots(e, &[(0xc, SLOT_E)]);
        answer(e, SLOT_E, 0);
        e.mem.set_u32(hit + 0x18, 7);
        let reference = object_with_slots(e, &[(0x100, SLOT_I)]);
        answer(e, SLOT_I, 1);
        e.register_double(FIND_REFERENCE_FOR_3D, move |_, a| {
            int(if a[0] == hit { reference } else { 0 })
        });
        answer(e, FUNCTION_008CFBC0, hit);
        answer(e, FUNCTION_008B3EF0, 3);
        let base = object_with_slots(e, &[(0x180, SLOT_J)]);
        answer(e, SLOT_J, 0x6c00_0000);
        answer(e, GET_BASE_FORM, base);
        answer(e, BODY_PART_DATA_GET_BODY_PART, 0x6d00_0000);
        answer(e, FUNCTION_005E5190, 0xfe);
        let log = logged(e, |e| {
            e.call(0x0096_4260, &args![player]);
        });
        let cast = calls_to(&log, FUNCTION_008CFBC0)[0].clone();
        assert_eq!(cast[0], AIM_NODE);
        assert_eq!(calls_to(&log, FUNCTION_008B3EF0), [[reference, 7]]);
        assert_eq!(calls_to(&log, FUNCTION_007F3B40), [[3]]);
        assert_eq!(
            calls_to(&log, BODY_PART_DATA_GET_BODY_PART),
            [[0x6c00_0000, 3]]
        );
        // The part byte is sign extended.
        assert_eq!(e.global::<u32>(AIMED_BODY_PART), 0xffff_fffe);
    }

    /// Every callee of `fn_00965620` and of the helpers it runs for real.
    fn aim_offsets_callees() -> Vec<u32> {
        vec![
            FUNCTION_009C71C0,
            ACTOR_GET_IRON_SIGHTS,
            READ_FIELD_8,
            ACTOR_GET_CURRENT_WEAPON,
            FUNCTION_004AD030,
            FUNCTION_004AD010,
            ITEM_CHANGE_HAS_MOD,
            WEAPON_IS_MELEE_TYPE,
            ADD_8C_TO_ADDRESS,
            PLAYER_GET_ROOT_NODE,
            NODE_GET_WORLD_BOUND,
            IDENTITY_006815C0,
            VECTOR_SUBTRACT,
            VECTOR_NOT_EQUAL,
            FORM_GET_TYPE,
            GET_BASE_FORM,
            FUNCTION_005E5130,
            FUNCTION_0043B230,
            FIND_NODE_BY_NAME,
            FUNCTION_0062BFB0,
            FUNCTION_008D8B80,
            SMART_POINTER_ASSIGN,
            FUNCTION_00461130,
            FUNCTION_00524C40,
            READ_FLOAT_AT_C,
            FUNCTION_00476B70,
            VECTOR_LENGTH,
            FUNCTION_004579E0,
            FUNCTION_004E4470,
            FUNCTION_004E44B0,
            FUNCTION_004A0C10,
            MATRIX_INVERSE,
            FUNCTION_004B4500,
            FUNCTION_004B13C0,
            FUNCTION_004B1460,
            FUNCTION_004B5530,
            MOBILE_OBJECT_GET_LOOKING,
            FUNCTION_00524D10,
            FUNCTION_00439F50,
            FUNCTION_00525B20,
            FUNCTION_004E4600,
            FUNCTION_009A7D50,
            FUNCTION_004019B0,
            FUNCTION_009A8F00,
            FUNCTION_009A6E90,
            READ_FLOAT_AT_D0,
            SETTING_FLOAT,
            VECTOR_ADD,
            FLAGS_TEST,
            BLEND_FLOATS,
            READ_FIELD_68,
            PLAYER_GET_ROOT_NODE,
        ]
    }

    /// The angle and limit constants of the exe.
    fn aim_constants(e: &mut Engine) {
        let pi = std::f32::consts::PI;
        e.set_global(FLOAT_MIN_DOUBLE, f64::from(f32::MIN_POSITIVE));
        e.set_global(NEGATIVE_PI_DOUBLE, -f64::from(pi));
        e.set_global(TWO_PI_DOUBLE, f64::from(2.0 * pi));
        e.set_global(PI_DOUBLE, f64::from(pi));
        e.set_global(PITCH_LIMIT_DOUBLE, f64::from(1.553343f32));
        e.set_global(PITCH_LIMIT_FLOAT, 1.553343f32);
        e.set_global(NEGATIVE_PITCH_LIMIT_DOUBLE, -f64::from(1.553343f32));
        e.set_global(NEGATIVE_PITCH_LIMIT_FLOAT, -1.553343f32);
        e.set_global(DEGREES_TO_RADIANS, f64::from(0.017453292f32));
        e.set_global(SCALE_ONE_AND_A_QUARTER, 1.25f64);
        e.set_global(STEP_FRACTION, 0.1f64);
        e.set_global(AIM_LIFT, 10.0f64);
        e.set_global(TWO_PI_FLOAT, 2.0 * pi);
        e.set_global(DEFAULT_BOUND_RADIUS, 16.0f32);
    }

    /// The radians of `degrees` the way the aim limits are computed.
    fn degrees_to_radians(degrees: f32) -> f32 {
        (f64::from(degrees) * f64::from(0.017453292f32)) as f32
    }

    /// An engine with the doubles every aim offset test shares: real
    /// vector subtraction and comparison, identity transforms, identity
    /// angle functions, a player (process virtual `+0x148` is `SLOT_H`).
    fn aim_offsets_world() -> (Engine, Ptr<PlayerCharacter>) {
        let mut e = engine(&aim_offsets_callees());
        aim_constants(&mut e);
        e.register_double(VECTOR_SUBTRACT, |e, a| {
            for i in 0..3 {
                let value = e.mem.f32(a[0] + 4 * i) - e.mem.f32(a[2] + 4 * i);
                e.mem.set_f32(a[1] + 4 * i, value);
            }
            int(a[1])
        });
        e.register_double(VECTOR_NOT_EQUAL, |e, a| {
            let same = (0..3).all(|i| e.mem.f32(a[0] + 4 * i) == e.mem.f32(a[1] + 4 * i));
            int(u32::from(!same))
        });
        e.register_double(FUNCTION_004B4500, |e, a| {
            for i in 0..3 {
                let value = e.mem.u32(a[2] + 4 * i);
                e.mem.set_u32(a[1] + 4 * i, value);
            }
            int(a[1])
        });
        e.register_double(MATRIX_INVERSE, |_, a| int(a[1]));
        e.register_double(VECTOR_ADD, |_, a| int(a[1]));
        float_settings(&mut e, &[]);
        e.register_double(IDENTITY_006815C0, |_, a| int(a[0]));
        e.register_double(FUNCTION_004B1460, |_, a| {
            float(f64::from(f32::from_bits(a[0])))
        });
        e.register_double(FUNCTION_004B5530, |_, a| {
            float(f64::from(f32::from_bits(a[0])))
        });
        answer(&mut e, SLOT_H, 0);
        let player = player_with(&mut e, &[], &[(0x148, SLOT_H)]);
        (e, player)
    }

    /// Calls the offsets function for `player`: returns the two outputs.
    #[allow(clippy::too_many_arguments)]
    fn run_aim_offsets(
        e: &mut Engine,
        player: Ptr<PlayerCharacter>,
        actor: u32,
        aim_node: u32,
        origin_override: u32,
        defaults: (f32, f32),
        start: (f32, f32),
    ) -> (f32, f32) {
        let heading = e.mem.alloc(4);
        let pitch = e.mem.alloc(4);
        e.mem.set_f32(heading, start.0);
        e.mem.set_f32(pitch, start.1);
        e.call(
            0x0096_5620,
            &args![
                player,
                actor,
                aim_node,
                heading,
                pitch,
                origin_override,
                defaults.0,
                defaults.1
            ],
        );
        (e.mem.f32(heading), e.mem.f32(pitch))
    }

    fn set_bullet_aim(e: &mut Engine, player: Ptr<PlayerCharacter>, point: (f32, f32, f32)) {
        e.set(player, PlayerCharacter::BulletAutoAimX, point.0);
        e.set(player, PlayerCharacter::BulletAutoAimY, point.1);
        e.set(player, PlayerCharacter::BulletAutoAimZ, point.2);
    }

    /// The player's root node bound, a bound centre of three floats.
    fn root_bound(e: &mut Engine, centre: (f32, f32, f32)) {
        answer(e, PLAYER_GET_ROOT_NODE, 0x6000_0001);
        let bound = e.mem.alloc(16);
        e.mem.set_f32(bound, centre.0);
        e.mem.set_f32(bound + 4, centre.1);
        e.mem.set_f32(bound + 8, centre.2);
        answer(e, NODE_GET_WORLD_BOUND, bound);
    }

    #[test]
    fn aim_offsets_leave_the_outputs_alone_without_a_target() {
        let (mut e, player) = aim_offsets_world();
        root_bound(&mut e, (0.0, 0.0, 0.0));
        let log = logged(&mut e, |e| {
            let (heading, pitch) = run_aim_offsets(e, player, 0, 0, 0, (0.0, 0.0), (0.25, -0.5));
            assert_eq!((heading, pitch), (0.25, -0.5));
        });
        // The bullet aim point is the zero vector: nothing is rotated.
        assert!(calls_to(&log, FUNCTION_004A0C10).is_empty());
    }

    #[test]
    fn aim_offsets_from_the_bullet_aim_point_without_angles() {
        let (mut e, player) = aim_offsets_world();
        float_settings(&mut e, &[(SETTING_011CE90C, 20.0)]);
        root_bound(&mut e, (1.0, 1.0, 0.25));
        set_bullet_aim(&mut e, player, (3.0, 4.0, 0.5));
        let node_matrix = e.mem.alloc(36);
        answer(&mut e, FUNCTION_00461130, node_matrix);
        let log = logged(&mut e, |e| {
            // Both angle defaults at FLT_MIN and no actor: no angle mode.
            let (heading, pitch) = run_aim_offsets(
                e,
                player,
                0,
                0,
                0,
                (f32::MIN_POSITIVE, f32::MIN_POSITIVE),
                (0.125, 0.5),
            );
            // The direction (2, 3, 0.25) gives a heading of 2/3 and a pitch
            // of -0.25, but with no weapon the pitch is dropped and the
            // heading limited to 20 degrees.
            let limit = degrees_to_radians(20.0);
            assert_eq!(heading, (f64::from(0.125f32) + f64::from(limit)) as f32);
            assert_eq!(pitch, 0.5);
        });
        // The matrix is the root node's, inverted then multiplied with the
        // global matrix.
        assert_eq!(calls_to(&log, FUNCTION_00461130), [[0x6000_0001]]);
        assert_eq!(calls_to(&log, MATRIX_INVERSE).len(), 1);
        let transforms = calls_to(&log, FUNCTION_004B4500);
        assert_eq!(transforms.len(), 2);
        assert_eq!(transforms[1][0], WOBBLE_RESULT_MATRIX);
        assert_eq!(
            calls_to(&log, FUNCTION_004B1460),
            [[(2.0f32 / 3.0).to_bits()]]
        );
        assert_eq!(calls_to(&log, FUNCTION_004B5530), [[0.25f32.to_bits()]]);
    }

    #[test]
    fn aim_offsets_with_angles_follow_the_actor() {
        let (mut e, player) = aim_offsets_world();
        float_settings(&mut e, &[(SETTING_011CFAF8, 90.0)]);
        root_bound(&mut e, (0.0, 0.0, 0.0));
        set_bullet_aim(&mut e, player, (1.0, 2.0, 0.5));
        // The actor: heading 0.125 (virtual +0x2bc), looking 0.25, offsets
        // 0.0625 and 0.125.
        answer_float(&mut e, SLOT_A, 0.125);
        let actor = object_with_slots(&mut e, &[(0x2bc, SLOT_A)]);
        e.mem.set_f32(actor + 0xec, 0.0625);
        answer_float(&mut e, MOBILE_OBJECT_GET_LOOKING, 0.25);
        answer_float(&mut e, FUNCTION_004E4600, 0.125);
        // A ranged weapon.
        answer(&mut e, ACTOR_GET_CURRENT_WEAPON, 0x6000_0004);
        let mut angles = vec![0.5, 0.25].into_iter();
        e.register_double(FUNCTION_004B13C0, move |_, _| float(angles.next().unwrap()));
        let log = logged(&mut e, |e| {
            let (heading, pitch) = run_aim_offsets(e, player, actor, 0, 0, (0.0, 0.0), (0.0, 0.0));
            // Heading 0.5 - 0.125 + the column angle 0.25 + 0.0625; pitch
            // -0.5 - 0.25 and the actor's 0.125.
            assert_eq!(heading, 0.6875);
            assert_eq!(pitch, -0.625);
        });
        assert_eq!(calls_to(&log, FUNCTION_004B13C0).len(), 2);
        assert_eq!(
            calls_to(&log, FUNCTION_00439F50)[0][..2],
            [WOBBLE_RESULT_MATRIX, 1]
        );
        assert_eq!(calls_to(&log, SLOT_A), [[actor, 0]]);
    }

    #[test]
    fn aim_offsets_return_early_for_iron_sights_with_a_flagged_mod() {
        let (mut e, player) = aim_offsets_world();
        root_bound(&mut e, (0.0, 0.0, 0.0));
        set_bullet_aim(&mut e, player, (1.0, 2.0, 0.5));
        answer(&mut e, ACTOR_GET_IRON_SIGHTS, 1);
        answer(&mut e, ACTOR_GET_CURRENT_WEAPON, 0x6000_0004);
        answer(&mut e, FUNCTION_004AD030, 1);
        answer(&mut e, SLOT_H, 0x6000_0005);
        answer(&mut e, ITEM_CHANGE_HAS_MOD, 1);
        let log = logged(&mut e, |e| {
            let (heading, pitch) = run_aim_offsets(e, player, 0, 0, 0, (0.0, 0.0), (0.25, 0.5));
            assert_eq!((heading, pitch), (0.25, 0.5));
        });
        assert_eq!(calls_to(&log, ITEM_CHANGE_HAS_MOD), [[0x6000_0005, 0xe]]);
        assert!(calls_to(&log, VECTOR_NOT_EQUAL).is_empty());
        // Without the flagged mod the computation goes on.
        answer(&mut e, ITEM_CHANGE_HAS_MOD, 0);
        answer(&mut e, FUNCTION_004AD010, 0);
        let log = logged(&mut e, |e| {
            run_aim_offsets(e, player, 0, 0, 0, (0.0, 0.0), (0.25, 0.5));
        });
        assert!(!calls_to(&log, VECTOR_NOT_EQUAL).is_empty());
        // A weapon with the second flag but not the first returns too.
        answer(&mut e, FUNCTION_004AD030, 0);
        answer(&mut e, FUNCTION_004AD010, 1);
        let log = logged(&mut e, |e| {
            run_aim_offsets(e, player, 0, 0, 0, (0.0, 0.0), (0.25, 0.5));
        });
        assert!(calls_to(&log, VECTOR_NOT_EQUAL).is_empty());
    }

    /// A VATS action: target reference `+0xc`, body part index `+0x10`
    /// (-1: the weapon node) and the marker flag `+4`.
    fn vats_action(e: &mut Engine, marker: u8) -> (u32, u32) {
        let target = object_with_slots(e, &[(0x100, SLOT_C), (0x1d0, SLOT_D)]);
        answer(e, SLOT_C, 1);
        answer(e, SLOT_D, 0x6000_0040);
        let action = e.mem.alloc(0x20);
        e.mem.set_u32(action + 0xc, target);
        e.mem.set_u32(action + 0x10, 0xffff_ffff);
        e.mem.set_u8(action + 4, marker);
        answer(e, FUNCTION_009C71C0, action);
        e.register_double(READ_FIELD_8, |_, a| {
            int(if a[0] == OBJECT_011F2250 { 4 } else { 0 })
        });
        (action, target)
    }

    /// The vectors the VATS node lookup and rotation answer with.
    fn vats_vectors(e: &mut Engine) -> (u32, u32) {
        let vector = |e: &mut Engine, value: f32| {
            let block = e.mem.alloc(12);
            for i in 0..3 {
                e.mem.set_f32(block + 4 * i, value);
            }
            block
        };
        let centre = vector(e, 1.0);
        let rotated = vector(e, 2.0);
        (centre, rotated)
    }

    #[test]
    fn aim_offsets_at_a_vats_target_use_the_weapon_node_and_the_vats_limit() {
        let (mut e, player) = aim_offsets_world();
        float_settings(&mut e, &[(SETTING_011CFC20, 45.0)]);
        let (action, target) = vats_action(&mut e, 1);
        let (centre, rotated) = vats_vectors(&mut e);
        // The target's base form has body parts; index -1 asks for the
        // weapon node, found under the target's 3D.
        let base = object_with_slots(&mut e, &[(0x180, SLOT_J)]);
        answer(&mut e, SLOT_J, 0x6c00_0000);
        answer(&mut e, GET_BASE_FORM, base);
        let node = object_with_slots(&mut e, &[(0xc, SLOT_I)]);
        answer(&mut e, SLOT_I, 0);
        answer(&mut e, FIND_NODE_BY_NAME, node);
        answer(&mut e, ADD_8C_TO_ADDRESS, centre);
        answer(&mut e, FUNCTION_0062BFB0, centre);
        let matrix = e.mem.alloc(36);
        answer(&mut e, FUNCTION_00461130, matrix);
        answer(&mut e, FUNCTION_00524C40, rotated);
        // A ranged weapon with three projectiles.
        answer(&mut e, ACTOR_GET_CURRENT_WEAPON, 0x6000_0004);
        answer(&mut e, FUNCTION_00525B20, 3);
        answer_float(&mut e, FUNCTION_004B13C0, 0.5);
        let log = logged(&mut e, |e| {
            let (heading, pitch) =
                run_aim_offsets(e, player, 0, 0x6000_0020, 0, (0.125, 0.25), (0.0, 0.0));
            // Heading 0.5 - 0.125; the pitch -1 - 0.25 is limited to
            // the 45 degree VATS limit.
            assert_eq!(heading, 0.375);
            assert_eq!(pitch, -degrees_to_radians(45.0));
        });
        assert_eq!(
            calls_to(&log, FIND_NODE_BY_NAME),
            [[0x6000_0040, WEAPON_NODE_NAME]]
        );
        assert_eq!(calls_to(&log, FUNCTION_0062BFB0)[0][1], node);
        assert_eq!(calls_to(&log, FUNCTION_00461130), [[node]]);
        assert_eq!(calls_to(&log, FUNCTION_00524C40)[0][0], matrix);
        assert_eq!(
            calls_to(&log, FUNCTION_00525B20),
            [[0x6000_0004, 0, 0, player.addr()]]
        );
        let _ = (action, target);
    }

    #[test]
    fn aim_offsets_search_a_clear_shot_for_a_vats_actor_of_type_0x3e() {
        let (mut e, player) = aim_offsets_world();
        float_settings(&mut e, &[(SETTING_011CFC20, 45.0)]);
        e.set_global(0x011f_1c00, 0.5f32);
        let (action, target) = vats_action(&mut e, 1);
        let (centre, rotated) = vats_vectors(&mut e);
        // The actor: type 0x3e, base form with a positive float at +0x64.
        answer_float(&mut e, SLOT_A, 0.0);
        let actor = object_with_slots(&mut e, &[(0x2bc, SLOT_A)]);
        let base = object_with_slots(&mut e, &[(0x180, SLOT_J)]);
        e.mem.set_f32(base + 0x64, 1.0);
        answer(&mut e, SLOT_J, 0x6c00_0000);
        answer(&mut e, GET_BASE_FORM, base);
        answer(&mut e, FORM_GET_TYPE, 0x3e);
        let node = object_with_slots(&mut e, &[(0xc, SLOT_I)]);
        answer(&mut e, SLOT_I, 0);
        answer(&mut e, ADD_8C_TO_ADDRESS, centre);
        let matrix = e.mem.alloc(36);
        answer(&mut e, FUNCTION_00461130, matrix);
        answer(&mut e, FUNCTION_00524C40, rotated);
        answer(&mut e, FUNCTION_008D8B80, 0);
        answer(&mut e, ACTOR_GET_CURRENT_WEAPON, 0x6000_0004);
        answer(&mut e, FUNCTION_00525B20, 1);
        answer_float(&mut e, FUNCTION_004B13C0, 0.5);
        answer_float(&mut e, FUNCTION_009A7D50, 0.25);
        // The vats action's node: the target's 3D is the node here.
        answer(&mut e, SLOT_D, node);
        // The search: a length of 10 scaled by 0.5, start 5, and a reach of
        // 4 * 1.25 = 5: one probe, the projectile line of sight reaches the
        // target.
        answer_float(&mut e, VECTOR_LENGTH, 10.0);
        answer_float(&mut e, FUNCTION_004019B0, 5.0);
        answer_float(&mut e, BLEND_FLOATS, 4.0);
        answer_float(&mut e, READ_FIELD_68, 2.0);
        answer_float(&mut e, READ_FLOAT_AT_D0, 8.0);
        e.register_double(FUNCTION_009A6E90, move |e, a| {
            e.mem.set_u32(a[10], target);
            int(1)
        });
        let log = logged(&mut e, |e| {
            run_aim_offsets(e, player, actor, 0x6000_0020, 0, (0.0, 0.0), (0.0, 0.0));
        });
        let probe = calls_to(&log, FUNCTION_009A8F00);
        assert_eq!(probe.len(), 1);
        assert_eq!(probe[0][10], 5.0f32.to_bits());
        assert_eq!(probe[0][11], 0.5f32.to_bits());
        assert_eq!(probe[0][12], 0);
        let los = calls_to(&log, FUNCTION_009A6E90);
        assert_eq!(los.len(), 1);
        assert_eq!(los[0][0], player.addr());
        assert_eq!(los[0][1], base);
        assert_eq!(los[0][2], 5.0f32.to_bits());
        assert_eq!(los[0][9], 0);
        assert_eq!(los[0][11], 0);
        // found = 5 / 4, scaled by the actor's +0xd0 float (8) = 10.
        assert_eq!(e.mem.f32(actor + 0xd0), 10.0);
        let _ = action;
    }

    // ---- the exe's stack frames ----

    /// The local the next `with_stack(size)` on this thread takes (the frame of a function the
    /// test calls directly, outside any other local).
    fn next_frame(e: &mut Engine, size: u32) -> u32 {
        e.next_stack_local(size)
    }

    /// Where `EBP` lands in a frame block whose exe frame aligns `EBP` to 16 bytes.
    fn aligned_ebp(raw: u32, ebp: u32) -> u32 {
        (raw + ebp + 0xf) & !0xf
    }

    #[test]
    fn the_mouse_spring_parameters_sit_below_an_aligned_ebp() {
        let mut e = engine(CALLEES_0095F930);
        let (player, object, _, _) = spring_creation_world(&mut e);
        answer_float(&mut e, FUNCTION_00457990, 6.0);
        float_settings(
            &mut e,
            &[
                (SETTING_011D1A08, 1.0),
                (SETTING_011D1C7C, 2.0),
                (SETTING_011D1B44, 3.0),
                (SETTING_011D177C, 4.0),
            ],
        );
        let raw = next_frame(&mut e, CREATE_MOUSE_SPRING_FRAME + 0x10);
        let log = logged(&mut e, |e| {
            e.call(
                0x0095_f930,
                &args![player, Ptr::<()>::new(object), 3u32, 7.5f32],
            );
        });
        let ebp = aligned_ebp(raw, CREATE_MOUSE_SPRING_EBP);
        assert_eq!(calls_to(&log, MOUSE_SPRING_ACTION_NEW)[0][1], ebp - 0x70);
    }

    #[test]
    fn the_spring_update_eye_vectors_sit_below_an_aligned_ebp() {
        let mut e = engine(CALLEES_00960520);
        let (player, _, _, _) = update_world(&mut e);
        let raw = next_frame(&mut e, UPDATE_SPRING_FRAME + 0x10);
        let log = logged(&mut e, |e| {
            e.call(0x0096_0520, &args![player, 0.016f32]);
        });
        let ebp = aligned_ebp(raw, UPDATE_SPRING_EBP);
        assert_eq!(
            calls_to(&log, ACTOR_GET_EYE_VECTOR)[0][1..3],
            [ebp - 0x30, ebp - 0x3c]
        );
    }

    #[test]
    fn the_drop_position_eye_vectors_sit_below_an_aligned_ebp() {
        let mut e = engine(CALLEES_009614B0);
        let (player, _, out) = drop_world(&mut e);
        let object = Ptr::<()>::new(0x8800_0000);
        let raw = next_frame(&mut e, DROP_POSITION_FRAME + 0x10);
        let log = logged(&mut e, |e| {
            e.call(0x0096_14b0, &args![player, object, out, 0u8, 0u8]);
        });
        let ebp = aligned_ebp(raw, DROP_POSITION_EBP);
        assert_eq!(
            calls_to(&log, ACTOR_GET_EYE_VECTOR)[0][1..3],
            [ebp - 0x38, ebp - 0x44]
        );
    }

    #[test]
    fn the_auto_aim_box_sits_below_an_aligned_ebp() {
        let mut world = auto_aim_world(1, 100.0);
        let player = world.player;
        let raw = next_frame(&mut world.e, AUTO_AIM_FRAME + 0x10);
        let log = logged(&mut world.e, |e| {
            e.call(0x0096_4260, &args![player]);
        });
        let ebp = aligned_ebp(raw, AUTO_AIM_EBP);
        assert_eq!(calls_to(&log, NI_POINT2_NEW)[0][0], ebp - 0x34);
    }

    #[test]
    fn the_aim_offsets_source_sits_at_its_ebp_offset() {
        let (mut e, player) = aim_offsets_world();
        root_bound(&mut e, (0.0, 0.0, 0.0));
        let source = e.mem.alloc(12);
        e.mem.set_f32(source, 1.5);
        e.mem.set_f32(source + 4, 2.5);
        e.mem.set_f32(source + 8, 3.5);
        let block = next_frame(&mut e, AIM_OFFSETS_FRAME);
        run_aim_offsets(&mut e, player, 0, 0, source, (0.0, 0.0), (0.25, -0.5));
        // `EBP-0x18` holds the copied source.
        let at = block + AIM_OFFSETS_EBP - 0x18;
        assert_eq!(
            [e.mem.f32(at), e.mem.f32(at + 4), e.mem.f32(at + 8)],
            [1.5, 2.5, 3.5]
        );
    }
}

//! `fallout/ai/highprocess.cpp` (Xbox PDB source unit), part 5: its functions from `00903160` up to
//! (not including) `ffffffff` in FalloutNV.exe 1.4.0.525, translated
//! (docs/ENGINE_CRATE.md). The unit's shared layouts and helpers are in
//! [`super::highprocess`]; anything public there may be used here.
//!
//! This part covers `00903160` to `00905ae0` and four accessors elsewhere (`009b6600`, `009b88a0`, `009b88c0`, `009ee020`): the combat detection lists,
//! the radiation and package-start handlers, the detection-event and
//! avoid-area bookkeeping, the idle-conversation checks, the sandman and
//! cannibal handlers, and the first container helpers of the pathing
//! message queue.
//!
//! Virtual calls on `this` (a `HighProcess`) use the process vtable's byte
//! offsets as the code indexes them; where the Xbox PDB lists a method at the
//! same offset of its process vtable the doc comment names it, as a hint (the
//! PC vtable is not confirmed slot by slot). Virtual calls on an actor use
//! the actor's own vtable and are described by what the code does with the
//! result.
//!
//! x87 note: the game computes in extended precision and stores `float`
//! results; the translations compute in `f64` and round to `f32` at each
//! store.

#[allow(unused_imports)]
use super::highprocess::*;
#[allow(unused_imports)]
use crate::prelude::*;

/// `BSSimpleList` node accessor (`006815c0`): returns the node itself, whose
/// first word is the item.
const LIST_NODE_ITEM: u32 = 0x0068_15c0;
/// The word at +4 of its `this` (`00726070`): the next node of a
/// `BSSimpleList` node, the result script of a `PackageEventAction`.
const WORD_AT_4: u32 = 0x0072_6070;
/// The word at +8 of its `this` (`0044ddc0`): the topic of a
/// `PackageEventAction`.
const WORD_AT_8: u32 = 0x0044_ddc0;
/// `BSSimpleList` emptiness test (`008256d0`): no item and no next node.
const LIST_IS_EMPTY: u32 = 0x0082_56d0;
/// `BSSimpleList` clear (`00470470`): frees every node after the head and
/// zeroes the head's item.
const LIST_CLEAR: u32 = 0x0047_0470;
/// `BSSimpleList` scalar deleting destructor (`004702f0`), flag 1 = free.
const LIST_DELETE: u32 = 0x0047_02f0;
/// `BSSimpleList` constructor (`0096a2d0`).
const LIST_CONSTRUCTOR: u32 = 0x0096_a2d0;
/// `operator new` (`00401000`).
const OPERATOR_NEW: u32 = 0x0040_1000;
/// `operator delete` (`00401030`).
const OPERATOR_DELETE: u32 = 0x0040_1030;
/// The allocator `CreateAvoidArray` uses for the array object (`00aa13e0`).
const ARRAY_ALLOCATE: u32 = 0x00aa_13e0;
/// Game-setting float getter (`00403e20`): `this` is the setting, the
/// result a pointer to its float.
const SETTING_FLOAT: u32 = 0x0040_3e20;
/// Seconds since the previous frame (`0084d030`, `this` = [`FRAME_TIME_OBJECT`]),
/// a `float` in `ST0`.
const FRAME_TIME: u32 = 0x0084_d030;
const FRAME_TIME_OBJECT: u32 = 0x011f_6394;
/// The player character pointer (`011dea3c`).
const PLAYER_POINTER: u32 = 0x011d_ea3c;
/// The combat manager singleton pointer (`011f1958`).
const COMBAT_MANAGER_POINTER: u32 = 0x011f_1958;
/// `TESObjectREFR::SetTargeted` (`00564db0`).
const SET_TARGETED: u32 = 0x0056_4db0;
/// `NiPointer` getter (`00559450`): the first word of `this`.
const NI_POINTER_GET: u32 = 0x0055_9450;
/// Script run (`005ac1e0`, RET 0x10): `this` = the script, arguments the
/// reference, its script locals, 0 and 1.
const SCRIPT_RUN: u32 = 0x005a_c1e0;
/// The address of a reference's extra data list (`005d43c0`, `this + 0x44`).
const EXTRA_DATA_LIST_OF: u32 = 0x005d_43c0;
/// Script locals of an extra data list (`00418830`).
const SCRIPT_LOCALS: u32 = 0x0041_8830;
/// Compare-exchange of the message queue lock (`0043b460`) and the no-op
/// debug check around it (`0040fbe0`), as in the main block.
const COMPARE_EXCHANGE_LOCK: u32 = 0x0043_b460;
const DEBUG_CHECK: u32 = 0x0040_fbe0;
/// `_ftol2_sse` (`00ec62c0`): the float in `ST0` as an integer, passed here
/// as a leading `f64` argument.
const FLOAT_TO_INT: u32 = 0x00ec_62c0;
/// `float` minimum of its two arguments (`0040ebd0`, cdecl, `ST0` result).
const MINIMUM: u32 = 0x0040_ebd0;
/// The cdecl float helpers `UpdateRadiation` calls: the scaling of a setting
/// value (`00408820`) and the absolute value (`00408860`).
const RADIATION_SCALE: u32 = 0x0040_8820;
const ABSOLUTE_VALUE: u32 = 0x0040_8860;
/// A process's `MiddleHighProcess` accessor of a reference (`008d8520`).
const REFERENCE_PROCESS: u32 = 0x008d_8520;

// ---- Settings read by the functions below -------------------------------------------------

const RADIATION_CAP_SETTING: u32 = 0x011d_0db0;
const RADIATION_SCALE_SETTING: u32 = 0x011d_1314;
const RADIATION_RATE_SETTING: u32 = 0x011d_0f18;
const DETECTION_EVENT_LIFETIME_SETTING: u32 = 0x011c_e558;
const AVOID_RADIATION_SETTING: u32 = 0x011c_e570;
const IDLE_TALK_DISTANCE_SETTING: u32 = 0x011c_d92c;
const IDLE_TALK_DISTANCE_FOLLOWER_SETTING: u32 = 0x011c_d75c;
const TALKED_TO_CLEAR_SETTING: u32 = 0x011c_dcd8;
const TALK_CHANCE_SETTING: u32 = 0x011c_dc04;
const TALK_CHANCE_FOLLOWER_SETTING: u32 = 0x011c_d02c;
const CHECK_TO_TALK_FIRST_SETTING: u32 = 0x011c_d328;
const CHECK_TO_TALK_SECOND_SETTING: u32 = 0x011c_d038;
/// Floats in the exe's data: the talk distance default (`0102226c`), the two
/// floats `ProcessAvoidArea` gives the flee request through `006d3b00`
/// (`01017868`) and `00507610` (`01013dc0`), the float `ProcessSandman` hands
/// `00675a50` (`0104ef80`) and the one the knock call gets twice
/// (`01012054`).
const DEFAULT_TALK_DISTANCE: u32 = 0x0102_226c;
const FLEE_REQUEST_FIRST_FLOAT: u32 = 0x0101_7868;
const FLEE_REQUEST_SECOND_FLOAT: u32 = 0x0101_3dc0;
const SANDMAN_PACKAGE_FLOAT: u32 = 0x0104_ef80;
const KNOCK_FLOAT: u32 = 0x0101_2054;
/// A `double` (`01035810`): the distance at or below which the sandman
/// handler takes its close-range branch.
const SANDMAN_RANGE: u32 = 0x0103_5810;

// ---- Offsets of fields other classes own (names from the Xbox PDB, PC offsets) -----------

/// `LowProcess::pTarget` (`TESObjectREFR*`).
const PROCESS_TARGET: u32 = 0x40;
/// `HighProcess::bPickPackIdle`.
const PROCESS_PICK_PACK_IDLE: u32 = 0x110;
/// `HighProcess::AggroList`, `GroupsToHelpList`, `TargetToAddList`,
/// `SpectatorList` (`BSSimpleList<StartCombatStates *>` heads embedded in
/// the process) and `AggroRadiusList`, `AvoidActorList`
/// (`BSSimpleList<Actor *>`).
const PROCESS_AGGRO_LIST: u32 = 0x274;
const PROCESS_GROUPS_TO_HELP_LIST: u32 = 0x27c;
const PROCESS_TARGET_TO_ADD_LIST: u32 = 0x284;
const PROCESS_SPECTATOR_LIST: u32 = 0x28c;
const PROCESS_AGGRO_RADIUS_LIST: u32 = 0x38c;
const PROCESS_AVOID_ACTOR_LIST: u32 = 0x394;
/// `BaseProcess::CurrentPackage` and `HighProcess::RunOncePackage` (embedded
/// `ActorPackage` objects).
const PROCESS_CURRENT_PACKAGE: u32 = 0x4;
const PROCESS_RUN_ONCE_PACKAGE: u32 = 0xe4;
/// `PackageEventAction` (Xbox PDB, 0x10 bytes) inside a `TESPackage` on PC:
/// `OnBegin`, `OnEnd`, `OnChange`. Its words: idle (+0), result script
/// (+4), topic (+8).
const PACKAGE_ON_BEGIN: u32 = 0x4c;
const PACKAGE_ON_END: u32 = 0x5c;
const PACKAGE_ON_CHANGE: u32 = 0x6c;
/// The actor's embedded actor-value interface (a second vtable pointer at
/// +0xa4) and the actor value index the radiation code passes.
const ACTOR_VALUE_INTERFACE: u32 = 0xa4;
const RADIATION_ACTOR_VALUE: u32 = 0x36;

layout! {
    /// `StartCombatStates` (Xbox PDB), 0x14 bytes: an entry of the process's
    /// combat start lists. Only the fields the evaluation reads.
    pub struct StartCombatStates: 0x14 {
        /// `pTarget` (Xbox PDB): `Actor*`.
        0x00 pTarget: Ptr,
        /// `pGroup` (Xbox PDB): `CombatGroup*`.
        0x04 pGroup: Ptr,
        /// `bSendAlarm` (Xbox PDB).
        0x09 bSendAlarm: u8,
        /// `bFleeing` (Xbox PDB).
        0x0b bFleeing: u8,
    }

    /// `AvoidAreaStruct` (Xbox PDB), 0x34 bytes: an entry of
    /// `HighProcess::pListOfAvoidAreas`. The first 0x24 bytes are a
    /// `PathingAvoidNode`.
    pub struct AvoidAreaStruct: 0x34 {
        /// `fTimeExpire` (Xbox PDB).
        0x24 fTimeExpire: f32,
        /// `pRadiationMarker` (Xbox PDB): `TESObjectSTAT*`.
        0x28 pRadiationMarker: Ptr,
        /// `afRadiationLevel` (Xbox PDB).
        0x2c afRadiationLevel: f32,
        /// `pRefObj` (Xbox PDB): `TESObjectREFR*`.
        0x30 pRefObj: Ptr,
    }
}

/// The item of a list node: `*node.item`. The game fetches it through the
/// node accessor (`006815c0`).
fn node_item(e: &mut Engine, node: u32) -> u32 {
    let slot = e.call(LIST_NODE_ITEM, &args![node]).u32();
    e.mem.u32(slot)
}

/// The next node of a list node.
fn node_next(e: &mut Engine, node: u32) -> u32 {
    e.call(WORD_AT_4, &args![node]).u32()
}

fn list_is_empty(e: &mut Engine, list: u32) -> bool {
    e.call(LIST_IS_EMPTY, &args![list]).bool()
}

fn list_clear(e: &mut Engine, list: u32) {
    e.call(LIST_CLEAR, &args![list]);
}

fn operator_delete_block(e: &mut Engine, block: u32) {
    e.call(OPERATOR_DELETE, &args![block]);
}

/// The float of a game setting.
fn setting_float(e: &mut Engine, setting: u32) -> f32 {
    let slot = e.call(SETTING_FLOAT, &args![setting]).u32();
    e.mem.f32(slot)
}

fn frame_time(e: &mut Engine) -> f32 {
    e.call(FRAME_TIME, &args![FRAME_TIME_OBJECT]).f32()
}

fn player(e: &Engine) -> u32 {
    e.mem.u32(PLAYER_POINTER)
}

fn set_targeted(e: &mut Engine, reference: u32) {
    e.call(SET_TARGETED, &args![reference, 1u32]);
}

/// `timer = timer - frame time`, stored as a `float`.
fn subtract_frame_time(e: &mut Engine, address: u32) {
    let step = frame_time(e);
    let timer = e.mem.f32(address);
    e.mem.set_f32(address, (timer as f64 - step as f64) as f32);
}

/// Runs `script` for `actor` with the script locals of the actor's extra
/// data list, as the package handlers do.
fn run_package_script(e: &mut Engine, script: u32, actor: Ptr) {
    // 00903aab / 00903c33 / 00903d3b: `005d43c0` takes no stack word (plain RET);
    // the pushed 0, 1 are the last two words of `005ac1e0` (RET 0x10).
    let list = e.call(EXTRA_DATA_LIST_OF, &args![actor]).u32();
    let locals = e.call(SCRIPT_LOCALS, &args![list]).u32();
    e.call(SCRIPT_RUN, &args![script, actor, locals, 0u32, 1u32]);
}

// Translated from 00903160 (decompiled, FalloutNV.exe 1.4.0.525)
/// Adds `value` to the `float` at +0x10 of `this` (an object that is not a
/// `HighProcess`; its caller is `ProcessGuardWaitToAttack`).
pub fn fn_00903160(e: &mut Engine, this: Ptr, value: f32) {
    let sum = e.mem.f32(this.addr() + 0x10) as f64 + value as f64;
    e.mem.set_f32(this.addr() + 0x10, sum as f32);
}

// Translated from 00903180 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HighProcess::AddPostAnimationAction` (Xbox PDB): sets the bits of
/// `actions` in `ePostAnimActon` (+0x424).
pub fn high_process_add_post_animation_action(
    e: &mut Engine,
    this: Ptr<HighProcess>,
    actions: u32,
) {
    let current = e.get(this, HighProcess::ePostAnimActon);
    e.set(this, HighProcess::ePostAnimActon, current | actions);
}

// Translated from 009031b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HighProcess::EvaluateCombatDetectionLists` (Xbox PDB): turns the lists
/// of combat starts the process gathered into combat for `actor`, then
/// clears them.
///
/// 1. For each entry of `GroupsToHelpList` (+0x27c) whose group differs from
///    the actor's own (`actor` vtable +0x3f8): asks the group's aggressor
///    flag (through `fn_00903600` and the member's process, vtable +0x1c4),
///    calls the process's enter-combat (vtable +0x33c, the Xbox PDB
///    `EnterCombat`), frees the entry and stops once the actor can no longer
///    attack (`actor` vtable +0x428 is zero) or its own group changed.
/// 2. If `AggroList` (+0x274) is empty it calls the process's vtable +0xd0
///    with 0. For each entry it works out whether the actor should attack the
///    entry's actor (faction fight reaction `008b87a0`, angry with the
///    player `008bffc0`, `008b06d0`), enters combat and frees the entry.
/// 3. Then, by the actor's answer from `00493bb0`: with a non-empty
///    `AggroRadiusList` (+0x38c) and no run-once package of kind 0xe (vtable
///    +0x20c), `00897bd0`; otherwise, with a non-empty `AvoidActorList`
///    (+0x394), each avoided actor gets the actor's vtable +0x410 call
///    unless `008a6650` says so and is added to a running package of kind
///    0x16 (`009f1310`); otherwise each `SpectatorList` (+0x28c) entry is
///    passed to the combat manager (`00992480`) and freed.
/// 4. The four lists (+0x27c, +0x274, +0x284, +0x28c) are cleared.
pub fn high_process_evaluate_combat_detection_lists(
    e: &mut Engine,
    this: Ptr<HighProcess>,
    actor: Ptr,
) {
    let process = this.addr();
    let group = e.vcall(actor.addr(), 0x3f8, &args![]).u32();

    let mut node = process + PROCESS_GROUPS_TO_HELP_LIST;
    while node != 0 && node_item(e, node) != 0 {
        let entry: Ptr<StartCombatStates> = Ptr::new(node_item(e, node));
        let entry_group = e.get(entry, StartCombatStates::pGroup);
        if group != entry_group.addr() {
            let mut aggressor = 0u8;
            if !entry_group.is_null() {
                let member = fn_00903600(e, entry_group, 0);
                if member != 0 {
                    let member_process = e.call(REFERENCE_PROCESS, &args![member]).u32();
                    aggressor = e.vcall(member_process, 0x1c4, &args![]).u8();
                }
            }
            let send_alarm = e.get(entry, StartCombatStates::bSendAlarm);
            let fleeing = e.get(entry, StartCombatStates::bFleeing);
            e.vcall(
                process,
                0x33c,
                &args![
                    actor,
                    0u32,
                    0u32,
                    send_alarm,
                    entry_group,
                    fleeing,
                    0u32,
                    aggressor,
                    0u32,
                    0u32,
                    0u32,
                    1u32,
                    0u32
                ],
            );
            operator_delete_block(e, entry.addr());
            if e.vcall(actor.addr(), 0x428, &args![]).u32() == 0 {
                break;
            }
            if group != e.vcall(actor.addr(), 0x3f8, &args![]).u32() {
                break;
            }
        }
        node = node_next(e, node);
    }

    let mut node = process + PROCESS_AGGRO_LIST;
    if node_item(e, node) == 0 {
        e.vcall(process, 0xd0, &args![0u32]);
    }
    while node != 0 && node_item(e, node) != 0 {
        let entry: Ptr<StartCombatStates> = Ptr::new(node_item(e, node));
        let target = e.get(entry, StartCombatStates::pTarget);
        let should_attack = e.with_stack(8, |e, block| {
            // +0: the reaction word, +4: the byte `008b87a0` reports.
            let reaction = e
                .call(0x008b_87a0, &args![actor, target, block.byte_add(4)])
                .u32();
            e.mem.set_u32(block.addr(), reaction);
            let angry = e.call(0x008b_ffc0, &args![actor]).bool();
            let reported = e.mem.u8(block.addr() + 4);
            let player_target = target.addr() == player(e);
            if (!angry || !player_target) && reported == 0 {
                e.call(0x008b_06d0, &args![actor, target, 0u32, block, 0u32])
                    .bool()
            } else {
                false
            }
        });
        let send_alarm = e.get(entry, StartCombatStates::bSendAlarm);
        e.vcall(
            process,
            0x33c,
            &args![
                actor,
                target,
                0u32,
                send_alarm,
                0u32,
                0u32,
                0u32,
                should_attack,
                0u32,
                0u32,
                0u32,
                1u32,
                0u32
            ],
        );
        operator_delete_block(e, entry.addr());
        node = node_next(e, node);
    }

    let in_group = e.call(0x0049_3bb0, &args![actor]).bool();
    if !in_group && !list_is_empty(e, process + PROCESS_AGGRO_RADIUS_LIST) {
        let running = e.vcall(process, 0x20c, &args![]).u32();
        let skip = if running != 0 {
            let package = e.vcall(process, 0x20c, &args![]).u32();
            e.call(0x0041_ca90, &args![package]).u32() == 0xe
        } else {
            false
        };
        if !skip {
            e.call(0x0089_7bd0, &args![actor, actor]);
        }
    } else if !e.call(0x0049_3bb0, &args![actor]).bool()
        && !list_is_empty(e, process + PROCESS_AVOID_ACTOR_LIST)
    {
        let mut node = process + PROCESS_AVOID_ACTOR_LIST;
        while node != 0 && node_item(e, node) != 0 {
            let avoided = node_item(e, node);
            if !e.call(0x008a_6650, &args![actor, 0u32]).bool() {
                let knock = e.mem.f32(KNOCK_FLOAT);
                e.vcall(
                    actor.addr(),
                    0x410,
                    &args![avoided, 0u32, 1u32, 1u32, 0u32, 0u32, knock, knock],
                );
            }
            let package = e.vcall(process, 0x27c, &args![]).u32();
            if package != 0 && e.call(0x0041_ca90, &args![package]).u32() == 0x16 {
                e.call(0x009f_1310, &args![package, avoided]);
            }
            node = node_next(e, node);
        }
    } else if !e.call(0x0049_3bb0, &args![actor]).bool() {
        let manager = e.mem.u32(COMBAT_MANAGER_POINTER);
        let mut node = process + PROCESS_SPECTATOR_LIST;
        while node != 0 && node_item(e, node) != 0 {
            let entry = node_item(e, node);
            let spectator = e.mem.u32(entry);
            e.call(0x0099_2480, &args![manager, actor, spectator]);
            operator_delete_block(e, entry);
            node = node_next(e, node);
        }
    }
    list_clear(e, process + PROCESS_GROUPS_TO_HELP_LIST);
    list_clear(e, process + PROCESS_AGGRO_LIST);
    list_clear(e, process + PROCESS_TARGET_TO_ADD_LIST);
    list_clear(e, process + PROCESS_SPECTATOR_LIST);
}

// Translated from 00903600 (decompiled, FalloutNV.exe 1.4.0.525)
/// Looks `argument` up in the table embedded at +0x18 of `this` (`006dad90`)
/// and returns the word the lookup points to. `EvaluateCombatDetectionLists`
/// calls it with a combat group and 0, getting the group's first member.
pub fn fn_00903600(e: &mut Engine, this: Ptr, argument: u32) -> u32 {
    let slot = e
        .call(0x006d_ad90, &args![this.addr() + 0x18, argument])
        .u32();
    e.mem.u32(slot)
}

// Translated from 00903620 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HighProcess::UpdateRadiation` (Xbox PDB): for an actor that is a
/// follower (`actor` vtable +0x360) or has the property at +0x21c, adds
/// `elapsed` to `fRadiationTimer` (+0x43c), caps it at the setting
/// `011d0db0` plus 1, and when `fHighestRadiation` (+0x440) is non-zero
/// changes the actor's radiation actor value (index 0x36) by the radiation
/// accumulated over the timer, never below zero in total; then both fields
/// are reset.
pub fn high_process_update_radiation(
    e: &mut Engine,
    this: Ptr<HighProcess>,
    actor: Ptr,
    elapsed: f32,
) {
    if !e.vcall(actor.addr(), 0x360, &args![]).bool()
        && !e.vcall(actor.addr(), 0x21c, &args![]).bool()
    {
        return;
    }
    let timer = (e.get(this, HighProcess::fRadiationTimer) as f64 + elapsed as f64) as f32;
    e.set(this, HighProcess::fRadiationTimer, timer);
    if actor.addr() == player(e) {
        e.call(0x0096_8730, &args![actor]);
    }
    let cap = (setting_float(e, RADIATION_CAP_SETTING) as f64 + 1.0) as f32;
    let timer = e.get(this, HighProcess::fRadiationTimer);
    let limited = e.call(MINIMUM, &args![timer, cap]).f32();
    e.set(this, HighProcess::fRadiationTimer, limited);
    let highest = e.get(this, HighProcess::fHighestRadiation);
    if highest == 0.0 {
        return;
    }
    let negative = -(limited as f64);
    let scale_setting = setting_float(e, RADIATION_SCALE_SETTING);
    let scale = e.call(RADIATION_SCALE, &args![scale_setting]).f32();
    let mut change = (scale as f64 * negative) as f32;
    let highest = e.get(this, HighProcess::fHighestRadiation);
    if highest != 0.0 {
        let timer = e.get(this, HighProcess::fRadiationTimer);
        let product = highest as f64 * timer as f64;
        let rate = setting_float(e, RADIATION_RATE_SETTING);
        change = (rate as f64 * product + change as f64) as f32;
    }
    let current = e
        .vcall(
            actor.addr() + ACTOR_VALUE_INTERFACE,
            0xc,
            &args![RADIATION_ACTOR_VALUE],
        )
        .f32();
    if current as f64 + (change as f64) < 0.0 {
        change = -current;
    }
    if e.call(ABSOLUTE_VALUE, &args![change]).f32() != 0.0 {
        e.vcall(
            actor.addr(),
            0x3ac,
            &args![RADIATION_ACTOR_VALUE, change, 0u32],
        );
    }
    e.set(this, HighProcess::fHighestRadiation, 0.0);
    e.set(this, HighProcess::fRadiationTimer, 0.0);
}

// Translated from 009037e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HighProcess::FireAtObject` (Xbox PDB): when the process can attack
/// (vtable +0x3f8, the Xbox PDB `CanAttack`), decides whether to fire and
/// which attack to queue for `actor` with `weapon`.
///
/// With a weapon that `00524b40` accepts it asks the automatic-shots count
/// (vtable +0x438) and fires through the burst setter (vtable +0x43c), unless
/// the ammunition group (`004301b0`/`005f25d0`/`0070f490`) refuses; without
/// one it fires when the animation action (vtable +0x3e4) is -1 or 4. It then
/// picks the attack id: 0x72 when the process says the weapon is a grenade or
/// thrown (vtable +0x1a8, +0x1b4), 0x66 for a mine (+0x1ac), a value by the
/// animation group otherwise (`00495e40`: 0x33..0x38 give 0x26, 0x2c, 0x32,
/// 0x38, 0x3e, 0x44; 0x6c gives 0x1a), else the weapon's own (`0051f5f0`)
/// unless it is 0xff, else 0x20. `Actor::QueueAttack` (`008935f0`) queues it
/// and, if it accepted, the process's vtable +0x6e0 gets `00524b40`'s byte.
pub fn high_process_fire_at_object(
    e: &mut Engine,
    this: Ptr<HighProcess>,
    actor: Ptr,
    weapon: Ptr,
) {
    let process = this.addr();
    if !e.vcall(process, 0x3f8, &args![]).bool() {
        return;
    }
    let mut fire = false;
    let owner = e.vcall(actor.addr(), 0x1e4, &args![]).u32();
    if e.call(0x0052_4b40, &args![weapon]).bool() {
        let mut allowed = true;
        let shots = e.vcall(process, 0x438, &args![actor]).u8();
        let group = e.call(0x0043_01b0, &args![owner, 4u32]).u16();
        if e.call(0x005f_25d0, &args![group]).bool() {
            let count = e.call(0x0070_f490, &args![owner, 4u32]).i32();
            if !(1..2).contains(&count) {
                allowed = false;
            }
        }
        if allowed && shots != 0 {
            e.vcall(process, 0x43c, &args![shots]);
            fire = true;
        }
    } else {
        let action = e.vcall(process, 0x3e4, &args![]).i32();
        if action == -1 || action == 4 {
            fire = true;
        }
    }
    if !fire {
        return;
    }
    let attack: u32 =
        if e.vcall(process, 0x1a8, &args![]).bool() || e.vcall(process, 0x1b4, &args![]).bool() {
            0x72
        } else if e.vcall(process, 0x1ac, &args![]).bool() {
            0x66
        } else {
            let kind = e.call(0x0049_5e40, &args![owner, 0u32]).u8() as i8;
            match kind {
                0x33 => 0x26,
                0x34 => 0x2c,
                0x35 => 0x32,
                0x36 => 0x38,
                0x37 => 0x3e,
                0x38 => 0x44,
                0x6c => 0x1a,
                _ => {
                    if !weapon.is_null() && e.call(0x0051_f5f0, &args![weapon]).u32() != 0xff {
                        e.call(0x0051_f5f0, &args![weapon]).u32()
                    } else {
                        0x20
                    }
                }
            }
        };
    if e.call(0x0089_35f0, &args![actor, attack]).bool() {
        let byte = e.call(0x0052_4b40, &args![weapon]).u8();
        e.vcall(process, 0x6e0, &args![byte]);
    }
}

// Translated from 00903a80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HighProcess::StartNewPackage` (Xbox PDB): when `package` is not null,
/// runs its `OnBegin` result script for `actor`, sets up its `OnBegin`
/// idle (process vtable +0x44, the Xbox PDB `SetupSpecialIdle`, then +0x588
/// `SetBeginIdlesPlayed`) or, without one, sets `bPickPackIdle` (+0x110) and,
/// when the actor's vtable +0x1e4 result is not null and `004985f0` on it is
/// false, calls vtable +0x44 with no idle; then starts its `OnBegin` topic
/// (vtable +0x2a4, `ProcessGreet`). Finally calls `00884f80` on the actor
/// when it is not null.
pub fn high_process_start_new_package(
    e: &mut Engine,
    this: Ptr<HighProcess>,
    actor: Ptr,
    package: Ptr,
) {
    let process = this.addr();
    if !package.is_null() {
        let script = fn_00903bd0(e, package);
        if !script.is_null() {
            run_package_script(e, script.addr(), actor);
        }
        let idle = e.call(0x0054_6930, &args![package]).u32();
        if idle != 0 {
            e.vcall(process, 0x44, &args![actor, idle, 2u32, 1u32, 0u32, 1u32]);
            e.vcall(process, 0x588, &args![1u32]);
        } else {
            e.mem.set_u8(process + PROCESS_PICK_PACK_IDLE, 1);
            if e.vcall(actor.addr(), 0x1e4, &args![]).u32() != 0 {
                let special = e.vcall(actor.addr(), 0x1e4, &args![]).u32();
                if !e.call(0x0049_85f0, &args![special]).bool() {
                    e.vcall(process, 0x44, &args![actor, 0u32, 2u32, 0u32, 1u32, 1u32]);
                }
            }
        }
        let topic = fn_00903bb0(e, package);
        if !topic.is_null() {
            e.vcall(process, 0x2a4, &args![actor, topic, 0u32, 0u32, 1u32, 1u32]);
        }
    }
    if !actor.is_null() {
        e.call(0x0088_4f80, &args![actor]);
    }
}

// Translated from 00903bb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The topic of the package's `OnBegin` event action (`PackageEventAction::pTopic`,
/// +8 of the action at +0x4c).
pub fn fn_00903bb0(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(WORD_AT_8, &args![this.addr() + PACKAGE_ON_BEGIN])
        .ptr()
}

// Translated from 00903bd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The result script of the package's `OnBegin` event action
/// (`PackageEventAction::pResultScript`, +4 of the action at +0x4c).
pub fn fn_00903bd0(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(WORD_AT_4, &args![this.addr() + PACKAGE_ON_BEGIN])
        .ptr()
}

// Translated from 00903bf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HighProcess::ChangePackage` (Xbox PDB): for a non-null `package`, runs
/// its `OnChange` result script (only when the script passes the checks of
/// `00671d10` or `0055b980`), sets up its `OnChange` idle (vtable +0x44) and
/// starts its `OnChange` topic (vtable +0x2a4). Returns what the process's
/// vtable +0x22c (the Xbox PDB `GetCurrentPackage`) returns.
pub fn high_process_change_package(
    e: &mut Engine,
    this: Ptr<HighProcess>,
    actor: Ptr,
    package: Ptr,
) -> u32 {
    let process = this.addr();
    if !package.is_null() {
        let script = fn_00903cf0(e, package);
        if !script.is_null() {
            let first = e.call(0x0067_1d10, &args![script]).u32();
            if first != 0 || e.call(0x0055_b980, &args![script]).u32() != 0 {
                run_package_script(e, script.addr(), actor);
            }
        }
        let idle = e.call(0x0066_98d0, &args![package]).u32();
        if idle != 0 {
            e.vcall(process, 0x44, &args![actor, idle, 2u32, 1u32, 0u32, 1u32]);
        }
        let topic = fn_00903cd0(e, package);
        if !topic.is_null() {
            e.vcall(process, 0x2a4, &args![actor, topic, 0u32, 0u32, 1u32, 1u32]);
        }
    }
    e.vcall(process, 0x22c, &args![]).u32()
}

// Translated from 00903cd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The topic of the package's `OnChange` event action (+8 of the action at
/// +0x6c).
pub fn fn_00903cd0(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(WORD_AT_8, &args![this.addr() + PACKAGE_ON_CHANGE])
        .ptr()
}

// Translated from 00903cf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The result script of the package's `OnChange` event action (+4 of the
/// action at +0x6c).
pub fn fn_00903cf0(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(WORD_AT_4, &args![this.addr() + PACKAGE_ON_CHANGE])
        .ptr()
}

// Translated from 00903d10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HighProcess::PackageDone` (Xbox PDB): for a non-null `package`, runs
/// its `OnEnd` result script for `actor`, sets up its `OnEnd` idle (vtable
/// +0x44) and, when `0067efd0` says the package has bit 4 set in +0x1c,
/// sets the end-idles flag (vtable +0x590, the Xbox PDB `SetEndIdlesPlayed`),
/// and starts its `OnEnd` topic (vtable +0x2a4).
pub fn high_process_package_done(e: &mut Engine, this: Ptr<HighProcess>, actor: Ptr, package: Ptr) {
    let process = this.addr();
    if package.is_null() {
        return;
    }
    let script = fn_00903e20(e, package);
    if !script.is_null() {
        run_package_script(e, script.addr(), actor);
    }
    let idle = fn_00903de0(e, package);
    if !idle.is_null() {
        e.vcall(process, 0x44, &args![actor, idle, 2u32, 1u32, 0u32, 1u32]);
        if e.call(0x0067_efd0, &args![package]).bool() {
            e.vcall(process, 0x590, &args![1u32]);
        }
    }
    let topic = fn_00903e00(e, package);
    if !topic.is_null() {
        e.vcall(process, 0x2a4, &args![actor, topic, 0u32, 0u32, 1u32, 1u32]);
    }
}

// Translated from 00903de0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The idle of the package's `OnEnd` event action (`PackageEventAction::pIdle`,
/// the first word of the action at +0x5c, read as a `NiPointer`).
pub fn fn_00903de0(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(NI_POINTER_GET, &args![this.addr() + PACKAGE_ON_END])
        .ptr()
}

// Translated from 00903e00 (decompiled, FalloutNV.exe 1.4.0.525)
/// The topic of the package's `OnEnd` event action (+8 of the action at
/// +0x5c).
pub fn fn_00903e00(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(WORD_AT_8, &args![this.addr() + PACKAGE_ON_END])
        .ptr()
}

// Translated from 00903e20 (decompiled, FalloutNV.exe 1.4.0.525)
/// The result script of the package's `OnEnd` event action (+4 of the action
/// at +0x5c).
pub fn fn_00903e20(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(WORD_AT_4, &args![this.addr() + PACKAGE_ON_END])
        .ptr()
}

// Translated from 00903e40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HighProcess::SetActorsDetectionEvent` (Xbox PDB): replaces the process's
/// generated detection event (`pActorsGeneratedDetectionEvent`, +0x3dc) with
/// a new one: action value, a location (three floats), the current time
/// stamp and `reference` (marked targeted when not null). Words 1 and 6 of
/// the arguments are not read.
#[allow(clippy::too_many_arguments)]
pub fn high_process_set_actors_detection_event(
    e: &mut Engine,
    this: Ptr<HighProcess>,
    _unused_1: u32,
    location_x: f32,
    location_y: f32,
    location_z: f32,
    action_value: u32,
    _unused_6: u32,
    reference: Ptr,
) {
    let previous = e.get(this, HighProcess::pActorsGeneratedDetectionEvent);
    if !previous.is_null() {
        operator_delete_block(e, previous.addr());
    }
    let block = e.call(OPERATOR_NEW, &args![0x1cu32]).u32();
    let event: Ptr<DetectionEvent> = if block != 0 {
        e.call(0x008f_db10, &args![block]).ptr()
    } else {
        Ptr::NULL
    };
    e.set(
        this,
        HighProcess::pActorsGeneratedDetectionEvent,
        event.cast(),
    );
    e.set(event, DetectionEvent::iActionValue, action_value);
    // `Location` (NiPoint3) is three consecutive floats at +4.
    e.mem.set_f32(event.addr() + 4, location_x);
    e.mem.set_f32(event.addr() + 8, location_y);
    e.mem.set_f32(event.addr() + 12, location_z);
    let now = e.call(0x0043_5dd0, &args![]).f32();
    let stamp = e.with_stack(4, |e, stamp| {
        e.call(0x0043_5de0, &args![stamp, now]);
        e.mem.u32(stamp.addr())
    });
    e.set(event, DetectionEvent::fTimeStamp, stamp);
    e.set(event, DetectionEvent::pRef, reference);
    if !reference.is_null() {
        set_targeted(e, reference.addr());
    }
}

// Translated from 00903f50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HighProcess::RemoveDetectionEvent` (Xbox PDB): frees the generated
/// detection event, if any, and clears the pointer.
pub fn high_process_remove_detection_event(e: &mut Engine, this: Ptr<HighProcess>) {
    let event = e.get(this, HighProcess::pActorsGeneratedDetectionEvent);
    if !event.is_null() {
        operator_delete_block(e, event.addr());
    }
    e.set(this, HighProcess::pActorsGeneratedDetectionEvent, Ptr::NULL);
}

// Translated from 00903f90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HighProcess::CheckForExpiredDetectionEvent` (Xbox PDB): frees the
/// generated detection event once the time since its stamp (`00435e00`)
/// exceeds the setting `011ce558`.
pub fn high_process_check_for_expired_detection_event(e: &mut Engine, this: Ptr<HighProcess>) {
    let event = e.get(this, HighProcess::pActorsGeneratedDetectionEvent);
    if event.is_null() {
        return;
    }
    let elapsed = e.call(0x0043_5e00, &args![event.addr() + 0x10]).f32();
    let lifetime = setting_float(e, DETECTION_EVENT_LIFETIME_SETTING);
    if lifetime < elapsed {
        operator_delete_block(e, event.addr());
        e.set(this, HighProcess::pActorsGeneratedDetectionEvent, Ptr::NULL);
    }
}

// Translated from 00904000 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HighProcess::NotifyGuardAboutActivation` (Xbox PDB): when `activated`
/// answers its vtable +0x100 query with true and `guard` is not `owner`,
/// looks at the run-once package and the
/// current package of the process: the first of them that is a package of
/// kind 0x29 whose topic word (+8) is `owner` and for which `008b0670`
/// accepts `guard`/`activated` makes the process enter combat (vtable +0x33c)
/// with `guard` and `activated`.
pub fn high_process_notify_guard_about_activation(
    e: &mut Engine,
    this: Ptr<HighProcess>,
    guard: Ptr,
    activated: Ptr,
    owner: Ptr,
) {
    if !e.vcall(activated.addr(), 0x100, &args![]).bool() || guard == owner {
        return;
    }
    let process = this.addr();
    for package_slot in [
        process + PROCESS_RUN_ONCE_PACKAGE,
        process + PROCESS_CURRENT_PACKAGE,
    ] {
        let package = e.call(NI_POINTER_GET, &args![package_slot]).u32();
        if package != 0
            && e.call(0x0096_11e0, &args![package]).u32() == 0x29
            && e.call(WORD_AT_8, &args![package_slot]).u32() == owner.addr()
            && e.call(0x008b_0670, &args![guard, activated]).bool()
        {
            e.vcall(
                process,
                0x33c,
                &args![
                    guard, activated, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 1u32,
                    0u32
                ],
            );
            return;
        }
    }
}

// Translated from 009040f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HighProcess::IsAvoidAreaInAvoidPathingList` (Xbox PDB): whether an entry
/// of `pListOfAvoidAreas` (+0x44c) has `reference` as its `pRefObj`.
pub fn high_process_is_avoid_area_in_avoid_pathing_list(
    e: &mut Engine,
    this: Ptr<HighProcess>,
    reference: Ptr,
) -> bool {
    let mut node = e.get(this, HighProcess::pListOfAvoidAreas).addr();
    let mut found = false;
    while node != 0 && node_item(e, node) != 0 && !found {
        let area: Ptr<AvoidAreaStruct> = Ptr::new(node_item(e, node));
        node = node_next(e, node);
        if e.get(area, AvoidAreaStruct::pRefObj) == reference {
            found = true;
        }
    }
    found
}

// Translated from 00904160 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HighProcess::ClearAvoidAreas` (Xbox PDB): frees every avoid area of
/// `pListOfAvoidAreas` (+0x44c), clears the list, destroys it and sets the
/// pointer to null.
pub fn high_process_clear_avoid_areas(e: &mut Engine, this: Ptr<HighProcess>) {
    let list = e.get(this, HighProcess::pListOfAvoidAreas).addr();
    if list == 0 {
        return;
    }
    let mut node = list;
    while node != 0 && node_item(e, node) != 0 {
        let area = node_item(e, node);
        node = node_next(e, node);
        operator_delete_block(e, area);
    }
    let list = e.get(this, HighProcess::pListOfAvoidAreas).addr();
    list_clear(e, list);
    let list = e.get(this, HighProcess::pListOfAvoidAreas).addr();
    if list != 0 {
        e.call(LIST_DELETE, &args![list, 1u32]);
    }
    e.set(this, HighProcess::pListOfAvoidAreas, Ptr::NULL);
}

// Translated from 00904220 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HighProcess::RemoveAvoidPathingNode` (Xbox PDB): removes from
/// `pListOfAvoidAreas` (+0x44c) the first entry whose `pRefObj` is
/// `reference`, through the by-value list removal (`fn_00905330`), and frees
/// the entry.
pub fn high_process_remove_avoid_pathing_node(
    e: &mut Engine,
    this: Ptr<HighProcess>,
    reference: Ptr,
) {
    let mut node = e.get(this, HighProcess::pListOfAvoidAreas).addr();
    while node != 0 && node_item(e, node) != 0 {
        let area = node_item(e, node);
        if e.mem.u32(area + 0x30) == reference.addr() {
            let list = e.get(this, HighProcess::pListOfAvoidAreas);
            let freed = e.with_stack(4, |e, slot| {
                e.mem.set_u32(slot.addr(), area);
                fn_00905330(e, list, slot);
                e.mem.u32(slot.addr())
            });
            operator_delete_block(e, freed);
            return;
        }
        node = node_next(e, node);
    }
}

// Translated from 009042a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HighProcess::AddAvoidPathingArea` (Xbox PDB): adds an avoid area for
/// `reference` to `pListOfAvoidAreas` (+0x44c), creating the list when the
/// process has none. The area is a `PathingAvoidNode` built by `006915d0`
/// from the position, `radius` and `radiation_level` (its cost), then
/// `fTimeExpire`, `pRadiationMarker`, `afRadiationLevel` and `pRefObj` are
/// set and `reference` is marked targeted. When the process's
/// `fHighestRadiation` is not positive, asks `actor` to start an avoid
/// package (`008982c0`), with `time_expire` as its time unless that is
/// `f32::MAX` or the actor is pathing (`008b3bd0`), then 0.
#[allow(clippy::too_many_arguments)]
pub fn high_process_add_avoid_pathing_area(
    e: &mut Engine,
    this: Ptr<HighProcess>,
    actor: Ptr,
    position_x: f32,
    position_y: f32,
    position_z: f32,
    radius: f32,
    time_expire: f32,
    radiation_level: f32,
    reference: Ptr,
    radiation_marker: Ptr,
) {
    let block = e.call(OPERATOR_NEW, &args![0x34u32]).u32();
    let area: Ptr<AvoidAreaStruct> = if block != 0 {
        e.call(0x008f_db90, &args![block]).ptr()
    } else {
        Ptr::NULL
    };
    // The node is built into a temporary and copied over the area's first
    // nine words.
    let node_words: Vec<u32> = e.with_stack(0x30, |e, scratch| {
        let point = scratch;
        e.mem.set_f32(point.addr(), position_x);
        e.mem.set_f32(point.addr() + 4, position_y);
        e.mem.set_f32(point.addr() + 8, position_z);
        let temporary = scratch.byte_add(0xc);
        let built = e
            .call(
                0x0069_15d0,
                &args![temporary, point, radius, radiation_level],
            )
            .u32();
        (0..9).map(|i| e.mem.u32(built + 4 * i)).collect()
    });
    for (i, word) in node_words.iter().enumerate() {
        e.mem.set_u32(area.addr() + 4 * i as u32, *word);
    }
    e.set(area, AvoidAreaStruct::afRadiationLevel, radiation_level);
    e.set(area, AvoidAreaStruct::fTimeExpire, time_expire);
    e.set(area, AvoidAreaStruct::pRadiationMarker, radiation_marker);
    e.set(area, AvoidAreaStruct::pRefObj, reference);
    set_targeted(e, reference.addr());
    if e.get(this, HighProcess::pListOfAvoidAreas).is_null() {
        let block = e.call(OPERATOR_NEW, &args![8u32]).u32();
        let list: Ptr = if block != 0 {
            e.call(LIST_CONSTRUCTOR, &args![block]).ptr()
        } else {
            Ptr::NULL
        };
        e.set(this, HighProcess::pListOfAvoidAreas, list);
    }
    let list = e.get(this, HighProcess::pListOfAvoidAreas);
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), area.addr());
        e.call(0x005a_e3d0, &args![list, slot]);
    });
    if fn_00904430(e, this) <= 0.0 {
        let mut delay = 0.0f32;
        if time_expire != f32::MAX && !e.call(0x008b_3bd0, &args![actor]).bool() {
            delay = time_expire;
        }
        e.call(0x0089_82c0, &args![actor, reference, delay]);
    }
}

// Translated from 00904430 (decompiled, FalloutNV.exe 1.4.0.525)
/// Returns `fHighestRadiation` (+0x440, `float`).
pub fn fn_00904430(e: &mut Engine, this: Ptr<HighProcess>) -> f32 {
    e.get(this, HighProcess::fHighestRadiation)
}

// Translated from 00904450 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HighProcess::CreateAvoidArray` (Xbox PDB): when `008bcc80` says `actor`
/// should avoid radiation and the process has a non-empty
/// `pListOfAvoidAreas`, builds a `PathingAvoidNodeArray` (0x18 bytes from
/// `00aa13e0`, constructed by `006e3850`) holding a copy of the first avoid
/// area's node; returns the array, or null. (The game's loop over the list
/// runs its first iteration only: the code never jumps back.)
pub fn high_process_create_avoid_array(e: &mut Engine, this: Ptr<HighProcess>, actor: Ptr) -> Ptr {
    let mut array = Ptr::NULL;
    if !e.call(0x008b_cc80, &args![actor]).bool() {
        return array;
    }
    let list = e.get(this, HighProcess::pListOfAvoidAreas).addr();
    if list == 0 || list_is_empty(e, list) {
        return array;
    }
    if array.is_null() {
        let block = e.call(ARRAY_ALLOCATE, &args![0x18u32]).u32();
        array = if block != 0 {
            e.call(0x006e_3850, &args![block]).ptr()
        } else {
            Ptr::NULL
        };
    }
    let area = node_item(e, list);
    bs_simple_array_pathing_avoid_node_add(e, array, Ptr::new(area));
    node_next(e, list);
    array
}

// Translated from 00904540 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HighProcess::ProcessAvoidArea` (Xbox PDB): the avoid-radiation behavior
/// of `actor`.
///
/// Without a target (`LowProcess::pTarget`, +0x40) it asks the process to
/// pick one (vtable +0x8c, `SetTargetForPackage`). If it now has one, it
/// builds a flee pathing request (`006e5390`, 0xbc bytes on the stack) for the
/// actor, with the actor's travel value (vtable +0x2bc), a flag from its
/// object when the actor answers vtable +0x21c, two floats from the exe's
/// data, the process's avoid area (vtable +0x254, `CreateAvoidArray`), and
/// hands it to the actor (`008bb630`). Without a target, once `fAvoidWaitTimer` (+0x450) is
/// at most zero it calls vtable +0x288 (`AddToProcedureIndexRunning`) with the
/// actor and 1; otherwise the timer runs down by the frame time. With a target
/// the same wait happens when `008b3bb0` says so.
///
/// Reads the target's and the actor's positions (vtable +0x1f4) and the
/// setting `011ce570` as the game does, though it does not use them.
pub fn high_process_process_avoid_area(e: &mut Engine, this: Ptr<HighProcess>, actor: Ptr) {
    let process = this.addr();
    let target = e.mem.u32(process + PROCESS_TARGET);
    if target == 0 {
        e.vcall(process, 0x8c, &args![actor]);
        let target = e.mem.u32(process + PROCESS_TARGET);
        if target != 0 {
            let target_position = e.vcall(target, 0x1f4, &args![]).u32();
            for i in 0..3 {
                let _ = e.mem.u32(target_position + 4 * i);
            }
            let actor_position = e.vcall(actor.addr(), 0x1f4, &args![]).u32();
            for i in 0..3 {
                let _ = e.mem.u32(actor_position + 4 * i);
            }
            let _ = setting_float(e, AVOID_RADIATION_SETTING);
            e.with_stack(0xbc, |e, request| {
                e.call(0x006e_5390, &args![request]);
                e.call(0x006e_29f0, &args![request, actor]);
                let travel = e.vcall(actor.addr(), 0x2bc, &args![0u32]).f32();
                e.call(0x006e_2bb0, &args![request, travel]);
                if e.vcall(actor.addr(), 0x21c, &args![]).bool() {
                    let base = e.call(0x0041_81e0, &args![actor]).u32();
                    let flag = e.vcall(base + 0x30, 0x28, &args![]).u8();
                    e.call(0x006e_2b50, &args![request, flag]);
                }
                let cost = e.mem.f32(FLEE_REQUEST_FIRST_FLOAT);
                e.call(0x006d_3b00, &args![request, cost]);
                let avoid_array = e.vcall(process, 0x254, &args![actor]).u32();
                e.call(0x006d_61e0, &args![request, avoid_array]);
                let radius = e.mem.f32(FLEE_REQUEST_SECOND_FLOAT);
                e.call(0x0050_7610, &args![request, radius]);
                e.call(0x008d_f000, &args![request]);
                e.call(0x008b_b630, &args![actor, request]);
                e.call(0x006d_ad70, &args![request]);
            });
        } else {
            wait_or_process_procedure(e, this, actor);
        }
    } else if e.call(0x008b_3bb0, &args![actor]).bool()
        && e.get(this, HighProcess::fAvoidWaitTimer) <= 0.0
    {
        e.vcall(process, 0x288, &args![actor, 1u32]);
    } else {
        subtract_frame_time(e, process + 0x450);
    }
}

/// The wait of `ProcessAvoidArea` for a process without a target.
fn wait_or_process_procedure(e: &mut Engine, this: Ptr<HighProcess>, actor: Ptr) {
    if e.get(this, HighProcess::fAvoidWaitTimer) <= 0.0 {
        e.vcall(this.addr(), 0x288, &args![actor, 1u32]);
    } else {
        subtract_frame_time(e, this.addr() + 0x450);
    }
}

// Translated from 00904800 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HighProcess::CheckIfThereSomeoneTalkWith` (Xbox PDB): lets `actor`,
/// while it runs a package that allows it, start an idle conversation.
/// Returns whether one was started.
///
/// Needs the running package (process vtable +0x27c, the Xbox PDB
/// `GetPackageThatIsRunning`) not to be an interrupt package (`00678610`)
/// and the actor not to skip fall-out behavior (`008a78f0`). The package's
/// kind (`0041ca90`) decides: kinds 1, 2 and 7 talk to the process's target
/// (`LowProcess::pTarget`, +0x40, with the talk flag set); 4, 8, 9, 0xe, 0xf
/// and 0x10 never; 0xc only if `00901c50` says so. When
/// `fCheckToTalkTimer` (+0x2c8) is at most zero and it is allowed, and the
/// process has no target, it walks `pDetectedActorList` (+0x25c) for an
/// entry of detection level 3 whose actor is alive, neither `actor` nor the
/// player, that may talk to `actor` and be talked to (vtable +0x1f4 of both
/// processes) and is within the setting distance (`011cd92c`, or `011cd75c`
/// for a follower per `008d6f30`/`00425fd0`), then rolls against the setting
/// chance (`011cdc04`, or `011cd02c`) after the last-spoke-to list
/// (`pLastSpokeToList`, +0x264) has been timed out (`fClearTalkToListTimer`,
/// +0x2a0, set from `011cdcd8` while the list is empty, else run down by the
/// frame time, clearing the list at zero). The chosen target (not the
/// player) is then talked to (`actor` vtable +0x280) and
/// `fCheckToTalkTimer` is set to a random value between two settings
/// (`006465f0`).
pub fn high_process_check_if_there_someone_talk_with(
    e: &mut Engine,
    this: Ptr<HighProcess>,
    actor: Ptr,
) -> bool {
    let process = this.addr();
    let package = e.vcall(process, 0x27c, &args![]).u32();
    let mut result = false;
    let mut talk_flag = 0u8;
    let mut target = 0u32;
    if package == 0
        || e.call(0x0067_8610, &args![package]).bool()
        || e.call(0x008a_78f0, &args![actor, 1u32]).bool()
    {
        return result;
    }
    let mut allowed = true;
    match e.call(0x0041_ca90, &args![package]).u32() {
        1 | 2 | 7 => {
            talk_flag = 1;
            target = e.mem.u32(process + PROCESS_TARGET);
        }
        4 | 8 | 9 | 0xe | 0xf | 0x10 => allowed = false,
        0xc => allowed = e.call(0x0090_1c50, &args![package]).bool(),
        _ => {}
    }
    if !(e.get(this, HighProcess::fCheckToTalkTimer) <= 0.0 && allowed) {
        return result;
    }
    if target == 0 {
        e.set(this, HighProcess::bCheckDeadTalk, 0);
        let mut node = e.get(this, HighProcess::pDetectedActorList).addr();
        while node != 0 && node_item(e, node) != 0 {
            let state = node_item(e, node);
            if state == 0 {
                break;
            }
            target = e.mem.u32(state);
            if target != 0
                && !e.call(0x0043_7b90, &args![target]).bool()
                && e.mem.u32(state + 4) == 3
                && target != actor.addr()
                && target != player(e)
            {
                let target_process = e.call(REFERENCE_PROCESS, &args![target]).u32();
                if e.vcall(target_process, 0x1f4, &args![target, actor]).bool()
                    && e.vcall(process, 0x1f4, &args![actor, target]).bool()
                {
                    let target_package = e.call(0x0093_44a0, &args![target]).u32();
                    if target_package != 0
                        && e.call(0x0096_11e0, &args![target_package]).u32() == 0x25
                        && !e.call(0x0090_1c50, &args![target_package]).bool()
                    {
                        target = 0;
                    } else if let Some(chosen) = pick_conversation_target(e, this, actor, target) {
                        target = chosen;
                        e.vcall(process, 0x1f0, &args![target]);
                        let target_process = e.call(REFERENCE_PROCESS, &args![target]).u32();
                        e.vcall(target_process, 0x1f0, &args![actor]);
                        break;
                    } else {
                        target = 0;
                    }
                } else {
                    target = 0;
                }
            } else {
                target = 0;
            }
            node = node_next(e, node);
        }
    }
    if target != 0 && target != player(e) {
        e.vcall(
            actor.addr(),
            0x280,
            &args![target, 0u32, 0u32, 1u32, talk_flag, 0u32, 0u32, talk_flag, 0u32],
        );
        let maximum = setting_float(e, CHECK_TO_TALK_SECOND_SETTING);
        let minimum = setting_float(e, CHECK_TO_TALK_FIRST_SETTING);
        let wait = e.call(0x0064_65f0, &args![minimum, maximum]).f32();
        e.set(this, HighProcess::fCheckToTalkTimer, wait);
        result = true;
    }
    result
}

/// The distance, timer and chance checks `CheckIfThereSomeoneTalkWith` makes
/// for a candidate `target`; `Some(target)` when it is chosen.
fn pick_conversation_target(
    e: &mut Engine,
    this: Ptr<HighProcess>,
    actor: Ptr,
    target: u32,
) -> Option<u32> {
    let mut limit = e.mem.f32(DEFAULT_TALK_DISTANCE);
    let mut other = setting_float(e, IDLE_TALK_DISTANCE_SETTING);
    let follower_check = e.call(0x008d_6f30, &args![actor]).u32();
    if e.call(0x0042_5fd0, &args![follower_check]).bool() {
        other = setting_float(e, IDLE_TALK_DISTANCE_FOLLOWER_SETTING);
    }
    if other < limit {
        limit = setting_float(e, IDLE_TALK_DISTANCE_SETTING);
    }
    let distance = e.call(0x0057_23b0, &args![target, actor, 0u32, 0u32]).f32();
    if limit < distance || limit.is_nan() || distance.is_nan() {
        return None;
    }
    let spoke_to = e.get(this, HighProcess::pLastSpokeToList).addr();
    if list_is_empty(e, spoke_to) {
        let clear_after = setting_float(e, TALKED_TO_CLEAR_SETTING);
        e.set(this, HighProcess::fClearTalkToListTimer, clear_after);
    } else {
        subtract_frame_time(e, this.addr() + 0x2a0);
    }
    if e.get(this, HighProcess::fClearTalkToListTimer) <= 0.0 {
        list_clear(e, spoke_to);
    }
    let roll = e.call(0x0048_7f50, &args![]).u32() % 100;
    let listed = e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), target);
        e.call(0x005f_65d0, &args![spoke_to, slot]).bool()
    });
    if listed {
        return None;
    }
    let mut chance = setting_float(e, TALK_CHANCE_SETTING);
    let follower_check = e.call(0x008d_6f30, &args![actor]).u32();
    if e.call(0x0042_5fd0, &args![follower_check]).bool() {
        chance = setting_float(e, TALK_CHANCE_FOLLOWER_SETTING);
    }
    if (roll as f64) < chance as f64 {
        Some(target)
    } else {
        None
    }
}

// Translated from 00904c80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HighProcess::ComputeAllowSandboxConversation` (Xbox PDB): whether `actor`
/// may start a sandbox conversation with `target`: `target` must be alive
/// (`00437b90`), neither `actor` nor the player, and the process must allow
/// it (vtable +0x1f4); a target whose current package (`009344a0`) is of kind
/// 0x25 and fails `00901c50` is refused. It times out the last-spoke-to list
/// like `CheckIfThereSomeoneTalkWith` and allows the conversation only if
/// `target` is not in it.
pub fn high_process_compute_allow_sandbox_conversation(
    e: &mut Engine,
    this: Ptr<HighProcess>,
    actor: Ptr,
    target: Ptr,
) -> bool {
    if target.is_null()
        || e.call(0x0043_7b90, &args![target]).bool()
        || target == actor
        || target.addr() == player(e)
        || !e.vcall(this.addr(), 0x1f4, &args![actor, target]).bool()
    {
        return false;
    }
    let target_package = e.call(0x0093_44a0, &args![target]).u32();
    if target_package != 0
        && e.call(0x0096_11e0, &args![target_package]).u32() == 0x25
        && !e.call(0x0090_1c50, &args![target_package]).bool()
    {
        return false;
    }
    let spoke_to = e.get(this, HighProcess::pLastSpokeToList).addr();
    if list_is_empty(e, spoke_to) {
        let clear_after = setting_float(e, TALKED_TO_CLEAR_SETTING);
        e.set(this, HighProcess::fClearTalkToListTimer, clear_after);
    } else {
        subtract_frame_time(e, this.addr() + 0x2a0);
    }
    if e.get(this, HighProcess::fClearTalkToListTimer) <= 0.0 {
        list_clear(e, spoke_to);
    }
    let listed = e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), target.addr());
        e.call(0x005f_65d0, &args![spoke_to, slot]).bool()
    });
    !listed
}

// Translated from 00904da0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HighProcess::ProcessCannibal` (Xbox PDB): the cannibal feeding
/// behavior of `actor`.
///
/// Picks a target when there is none (vtable +0x8c), puts the weapon away
/// (vtable +0x454/+0x450, `GetWeaponDrawn`/`SetWantWeaponDrawn`), and takes
/// the target as the victim when it answers its vtable +0x100 query. While
/// the current action is not complete (vtable +0x11c) it sets up the eating
/// idle (vtable +0x44), marks the victim eaten (`008a80c0`) and completes the
/// action (vtable +0x118). Once the idle has finished playing (`004985f0`)
/// it moves on (vtable +0x288), makes the victim's process forget `actor`,
/// raises the attack alarm (`008c0460`) and clears the player's target
/// (`005cc7a0`).
pub fn high_process_process_cannibal(e: &mut Engine, this: Ptr<HighProcess>, actor: Ptr) {
    let process = this.addr();
    e.vcall(process, 0x27c, &args![]);
    if e.mem.u32(process + PROCESS_TARGET) == 0 {
        e.vcall(process, 0x8c, &args![actor]);
    }
    if e.vcall(process, 0x454, &args![]).bool() {
        e.vcall(process, 0x450, &args![0u32]);
    }
    let mut victim = 0u32;
    let target = e.mem.u32(process + PROCESS_TARGET);
    if target != 0 && e.vcall(target, 0x100, &args![]).bool() {
        victim = e.mem.u32(process + PROCESS_TARGET);
    }
    let idle = e.vcall(actor.addr(), 0x1e4, &args![]).u32();
    if !e.vcall(process, 0x11c, &args![]).bool() {
        e.vcall(process, 0x44, &args![actor, 0u32, 2u32, 1u32, 1u32, 1u32]);
        if victim != 0 {
            e.call(0x008a_80c0, &args![victim, 1u32]);
        }
        e.vcall(process, 0x118, &args![1u32]);
    } else if e.call(0x0049_85f0, &args![idle]).bool() {
        e.vcall(process, 0x288, &args![actor, 1u32]);
        if victim != 0 {
            let victim_process = e.call(REFERENCE_PROCESS, &args![victim]).u32();
            if victim_process != 0 {
                e.call(0x008c_0460, &args![victim, actor, 0u32, 1u32]);
                let victim_process = e.call(REFERENCE_PROCESS, &args![victim]).u32();
                e.vcall(victim_process, 0x210, &args![0u32, victim]);
            }
        }
        let actor_process = e.call(REFERENCE_PROCESS, &args![actor]).u32();
        let player_reference = player(e);
        e.vcall(actor_process, 0x210, &args![0u32, player_reference]);
        e.call(0x005c_c7a0, &args![player_reference, 0u32]);
    }
}

// Translated from 00904f50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HighProcess::ProcessSandman` (Xbox PDB): the sleep-and-attend behavior of
/// `actor`.
///
/// Picks a target when there is none (vtable +0x8c) and takes it as the
/// victim when it answers its vtable +0x100 query. The distance from the
/// actor to the position at +0x148 of the process (`00572380`, truncated by
/// `_ftol2`) decides: above the double at `01035810` the actor walks closer
/// (for a non-player actor through the package's `00675a50`/`00675c20` and
/// `008b3690`; for the player through the actor's vtable +0x2a8 and +0x2c4);
/// within it the process puts the weapon away (vtable +0x454/+0x450) and,
/// unless the actor's vtable +0x214 answers 4 or 9, returns if the process
/// has a current furniture (vtable +0x4c8) and its vtable +0x2b0 accepts the
/// actor; otherwise it calls `0089d900` on the victim (with the actor when
/// `fn_00905230` of the player is positive), moves on (vtable +0x288), calls
/// vtable +0x210 on the victim's and the actor's processes and clears the
/// player's target (`005cc7a0`, `0093dd20`). For the answers 4 and 9 it
/// moves on, calls vtable +0x210 on the victim's process, calls vtable
/// +0x4ec, clears the player's target and calls `008c0460` on the victim.
pub fn high_process_process_sandman(e: &mut Engine, this: Ptr<HighProcess>, actor: Ptr) {
    let process = this.addr();
    let package = e.vcall(process, 0x27c, &args![]).u32();
    if e.mem.u32(process + PROCESS_TARGET) == 0 {
        e.vcall(process, 0x8c, &args![actor]);
    }
    let mut victim = 0u32;
    let target = e.mem.u32(process + PROCESS_TARGET);
    if target != 0 && e.vcall(target, 0x100, &args![]).bool() {
        victim = e.mem.u32(process + PROCESS_TARGET);
    }
    let distance = e.call(0x0057_2380, &args![actor, process + 0x148]).f32();
    let distance = e.call(FLOAT_TO_INT, &args![distance as f64]).i32();
    let near = (distance as f64) <= e.mem.f64(SANDMAN_RANGE);
    if !near {
        if actor.addr() != player(e) {
            let weight = e.mem.f32(SANDMAN_PACKAGE_FLOAT);
            // 0090501b / 00905028: `00675a50` and `00675c20` take one word each (RET 4,
            // the actor); the weight, the 0 and the first result are the last three
            // words of `008b3690` (RET 0x14).
            let first = e.call(0x0067_5a50, &args![package, actor]).u32();
            let second = e.call(0x0067_5c20, &args![package, actor]).u32();
            e.call(
                0x008b_3690,
                &args![actor, process + 0x148, second, first, weight, 0u32],
            );
        } else {
            e.vcall(actor.addr(), 0x2a8, &args![process + 0x148]);
            let height = e.call(0x0056_8650, &args![process + 0x148]).f32();
            e.vcall(actor.addr(), 0x2c4, &args![height]);
        }
        return;
    }
    if e.vcall(process, 0x454, &args![]).bool() {
        e.vcall(process, 0x450, &args![0u32]);
    }
    let run_once_kind = e.vcall(actor.addr(), 0x214, &args![]).i32();
    let second_kind = if run_once_kind != 4 {
        e.vcall(actor.addr(), 0x214, &args![]).i32()
    } else {
        4
    };
    if run_once_kind != 4 && second_kind != 9 {
        let furniture = e.vcall(process, 0x4c8, &args![]).u32();
        if furniture != 0 && e.vcall(process, 0x2b0, &args![actor]).bool() {
            return;
        }
        let player_reference = player(e);
        let count = fn_00905230(e, Ptr::new(player_reference));
        if count > 0 {
            e.call(0x0089_d900, &args![victim, actor, 0.0f32]);
        } else {
            e.call(0x0089_d900, &args![victim, 0u32, 0.0f32]);
        }
        e.vcall(process, 0x288, &args![actor, 1u32]);
        let victim_process = e.call(REFERENCE_PROCESS, &args![victim]).u32();
        e.vcall(victim_process, 0x210, &args![0u32, victim]);
        let actor_process = e.call(REFERENCE_PROCESS, &args![actor]).u32();
        e.vcall(actor_process, 0x210, &args![0u32, actor]);
        e.call(0x005c_c7a0, &args![player_reference, 0u32]);
        e.call(0x0093_dd20, &args![player_reference]);
    } else {
        e.vcall(process, 0x288, &args![actor, 1u32]);
        let victim_process = e.call(REFERENCE_PROCESS, &args![victim]).u32();
        e.vcall(victim_process, 0x210, &args![0u32, victim]);
        e.vcall(process, 0x4ec, &args![1u32]);
        let player_reference = player(e);
        e.call(0x005c_c7a0, &args![player_reference, 0u32]);
        e.call(0x008c_0460, &args![victim, actor, 0u32, 1u32]);
    }
}

// Translated from 00905230 (decompiled, FalloutNV.exe 1.4.0.525)
/// Returns the signed word at +0x1fc of `this` (a `PlayerCharacter`;
/// `ProcessSandman` tests it for being positive).
pub fn fn_00905230(e: &mut Engine, this: Ptr) -> i32 {
    e.mem.i32(this.addr() + 0x1fc)
}

// Translated from 00905250 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HighProcess::SetPathLookAtTarget` (Xbox PDB): stores `target` in
/// `pPathLookAtTarget` (+0x2ac) and marks it targeted when it is not null.
pub fn high_process_set_path_look_at_target(e: &mut Engine, this: Ptr<HighProcess>, target: Ptr) {
    e.set(this, HighProcess::pPathLookAtTarget, target);
    if !target.is_null() {
        set_targeted(e, target.addr());
    }
}

// Translated from 00905280 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HighProcess::SetAnimation` (Xbox PDB): when `00602150` returns non-zero
/// for the process and `animation` is null, sets the animation action to -1
/// through the process's vtable +0x3ec (the Xbox PDB `SetAnimAction`, second
/// argument 0); then stores `animation` through `0091fe80`.
pub fn high_process_set_animation(e: &mut Engine, this: Ptr<HighProcess>, animation: Ptr) {
    if e.call(0x0060_2150, &args![this]).u32() != 0 && animation.is_null() {
        e.vcall(this.addr(), 0x3ec, &args![-1i32, 0u32]);
    }
    e.call(0x0091_fe80, &args![this, animation]);
}

// Translated from 009052c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The address of element `index` of the 0x54-byte-element array whose
/// buffer pointer is the word at +4 of `this` (a `BSSimpleArray`).
pub fn fn_009052c0(e: &mut Engine, this: Ptr, index: u32) -> Ptr {
    let buffer = e.mem.u32(this.addr() + 4);
    Ptr::new(buffer.wrapping_add(index.wrapping_mul(0x54)))
}

// Translated from 009052e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Reference-counted pointer assignment: `this` is the address of a pointer
/// cell, `source` the address of another; when they differ, releases the old
/// target (`0044b050`), copies the pointer and takes a reference on the new
/// one (`0044b010`). Returns `this`.
pub fn fn_009052e0(e: &mut Engine, this: Ptr, source: Ptr) -> Ptr {
    let old = e.mem.u32(this.addr());
    let new = e.mem.u32(source.addr());
    if old != new {
        if old != 0 {
            e.call(0x0044_b050, &args![old]);
        }
        e.mem.set_u32(this.addr(), new);
        if new != 0 {
            e.call(0x0044_b010, &args![new]);
        }
    }
    this
}

// Translated from 00905330 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleList` removal by value: `this` is the list head, `item_slot` the
/// address of a word holding the item to remove. Does nothing for a null
/// item or an empty list (`008256d0`). Removing the head node copies the
/// second node into it and frees the second node; removing a later node
/// unlinks and frees it (`004702f0`, flag 1). The item itself is not freed.
pub fn fn_00905330(e: &mut Engine, this: Ptr, item_slot: Ptr) {
    let head = this.addr();
    let item = e.mem.u32(item_slot.addr());
    if item == 0 || list_is_empty(e, head) {
        return;
    }
    let mut node = head;
    let mut previous = head;
    while node != 0 && e.mem.u32(node) != item {
        previous = node;
        node = e.mem.u32(node + 4);
    }
    if node == 0 {
        return;
    }
    if node == head {
        let second = e.mem.u32(head + 4);
        if second != 0 {
            let after = e.mem.u32(second + 4);
            e.mem.set_u32(head + 4, after);
            let moved = e.mem.u32(second);
            e.mem.set_u32(head, moved);
            e.mem.set_u32(second + 4, 0);
            e.call(LIST_DELETE, &args![second, 1u32]);
        } else {
            e.mem.set_u32(head, 0);
        }
    } else {
        let after = e.mem.u32(node + 4);
        e.mem.set_u32(previous + 4, after);
        e.mem.set_u32(node + 4, 0);
        e.call(LIST_DELETE, &args![node, 1u32]);
    }
}

// Translated from 00905450 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<PathingAvoidNode,1024>::Add` (Xbox PDB): makes room for one
/// more element (`00905a10`, which returns its index), constructs it in
/// place (`006e3c90`) and copies the 0x24-byte `PathingAvoidNode` at `item`
/// over it. Returns the index.
pub fn bs_simple_array_pathing_avoid_node_add(e: &mut Engine, this: Ptr, item: Ptr) -> u32 {
    let index = e.call(0x0090_5a10, &args![this]).u32();
    let buffer = e.mem.u32(this.addr() + 4);
    let element = buffer.wrapping_add(index.wrapping_mul(0x24));
    e.call(0x006e_3c90, &args![this, element, 1u32]);
    let buffer = e.mem.u32(this.addr() + 4);
    let element = buffer.wrapping_add(index.wrapping_mul(0x24));
    for i in 0..9 {
        let word = e.mem.u32(item.addr() + 4 * i);
        e.mem.set_u32(element + 4 * i, word);
    }
    index
}

// Translated from 009054a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSTCommonMessageQueue<ActorPathingMessage>::Push` (Xbox PDB): spins until
/// it takes the queue's lock (compare-exchange of the word at +4 from 0 to
/// 1), calls the virtual push (vtable +0x14) with `message` and releases the
/// lock. Returns the virtual call's result.
pub fn bst_common_message_queue_actor_pathing_message_push(
    e: &mut Engine,
    this: Ptr,
    message: Ptr,
) -> bool {
    locked_vcall_spinning(e, this, 0x14, message)
}

// Translated from 00905510 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSTCommonMessageQueue<ActorPathingMessage>::Pop` (Xbox PDB): the same
/// spinning lock around the virtual pop (vtable +0x18).
pub fn bst_common_message_queue_actor_pathing_message_pop(
    e: &mut Engine,
    this: Ptr,
    message: Ptr,
) -> bool {
    locked_vcall_spinning(e, this, 0x18, message)
}

/// The body `Push` and `Pop` share.
fn locked_vcall_spinning(e: &mut Engine, this: Ptr, slot: u32, message: Ptr) -> bool {
    let lock = this.addr() + 4;
    loop {
        let previous = e
            .call(COMPARE_EXCHANGE_LOCK, &args![lock, 0u32, 1u32])
            .u32();
        if previous == 0 {
            break;
        }
    }
    e.call(DEBUG_CHECK, &args![]);
    let result = e.vcall(this.addr(), slot, &args![message]).bool();
    e.call(DEBUG_CHECK, &args![]);
    e.call(COMPARE_EXCHANGE_LOCK, &args![lock, 1u32, 0u32]);
    result
}

/// Vtable of `BSTCommonLLMessageQueue<ActorPathingMessage>` (`010883f8`).
const LL_QUEUE_VTABLE: u32 = 0x0108_83f8;
/// Vtable of `BSTCommonMessageQueue<ActorPathingMessage>` (`01088418`).
const COMMON_QUEUE_VTABLE: u32 = 0x0108_8418;
/// Vtable of the message queue's base class (`01088438`).
const QUEUE_BASE_VTABLE: u32 = 0x0108_8438;
/// Vtable of `BSSimpleArray<TESIdleForm *,1024>` (`01088450`).
const IDLE_FORM_ARRAY_VTABLE: u32 = 0x0108_8450;
/// `ActorPathingMessage` constructor (`006e9bd0`, 0x10 bytes) and destructor
/// (`00800190`), and its assignment (`006e9c60`: `this` = destination, the
/// argument the source).
const MESSAGE_CONSTRUCT: u32 = 0x006e_9bd0;
const MESSAGE_DESTRUCT: u32 = 0x0080_0190;
const MESSAGE_ASSIGN: u32 = 0x006e_9c60;
/// `BSTCommonMessageQueue<ActorPathingMessage>::TryPop` (`006ec390`, Xbox PDB).
const QUEUE_TRY_POP: u32 = 0x006e_c390;
/// Free-list pop (`006ecd60`): takes the lock, stores the first free node in
/// the out parameter (or 0) and answers whether there was one.
const FREE_LIST_TAKE: u32 = 0x006e_cd60;
/// Free-list push (`006ecc70`): links the node the argument points to in
/// front of the free nodes, and zeroes that pointer.
const FREE_LIST_GIVE: u32 = 0x006e_cc70;
/// Construction of a free-list node holding a message (`00905b00`).
const FREE_NODE_CONSTRUCT: u32 = 0x0090_5b00;
/// `BSSimpleList` node constructor (`00470440`): `this` = the node, the
/// argument a pointer to the item.
const LIST_NODE_CONSTRUCT: u32 = 0x0047_0440;
/// Array constructor helper (`006b3eb0`): `this`, a size and a count.
const ARRAY_CONSTRUCT: u32 = 0x006b_3eb0;
/// Array clear (`008454f0`): `this` and a flag (1 = also free the buffer).
const ARRAY_CLEAR: u32 = 0x0084_54f0;
/// Is the array full (`00438b90`): count equals capacity.
const ARRAY_IS_FULL: u32 = 0x0043_8b90;
/// Next capacity (`009a3910`): twice the capacity up to 0x400, then
/// 0x400 more.
const ARRAY_NEXT_CAPACITY: u32 = 0x009a_3910;
/// Array resize (`006e3d30`): `this`, the new capacity and the count.
const ARRAY_RESIZE: u32 = 0x006e_3d30;
/// Reference release (`00401970`) and acquire (`0040f6e0`) of the object
/// `+ 0x14`.
const REFERENCE_RELEASE: u32 = 0x0040_1970;
const REFERENCE_ACQUIRE: u32 = 0x0040_f6e0;
/// Scalar deleting destructor of the message holder at `009bc890`
/// (`this`, flags).
const MESSAGE_HOLDER_DESTRUCT: u32 = 0x009b_c890;

// Translated from 00905580 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of `BSTCommonLLMessageQueue<ActorPathingMessage>` (the class
/// whose destructor is `009055c0`; unnamed in the map): the base constructor
/// (`00905990`), then the vtable, the free list `free_list` at +8, no head
/// node at +0xc and the tail pointer at +0x10 aiming at the head slot.
/// Returns `this`.
pub fn fn_00905580(e: &mut Engine, this: Ptr, free_list: u32) -> Ptr {
    fn_00905990(e, this);
    let base = this.addr();
    e.mem.set_u32(base, LL_QUEUE_VTABLE);
    e.mem.set_u32(base + 8, free_list);
    e.mem.set_u32(base + 0xc, 0);
    e.mem.set_u32(base + 0x10, base + 0xc);
    this
}

// Translated from 009055c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSTCommonLLMessageQueue<ActorPathingMessage>::~BSTCommonLLMessageQueue`
/// (Xbox PDB): installs its vtable, pops every queued message into a
/// temporary `ActorPathingMessage` until `TryPop` (`006ec390`) says the
/// queue is empty, destroys the temporary and runs the base destructor
/// (`00905650`). The C++ unwinding state is not translated.
pub fn bst_common_ll_message_queue_actor_pathing_message_destructor(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), LL_QUEUE_VTABLE);
    e.with_stack(0x10, |e, message| {
        e.call(MESSAGE_CONSTRUCT, &args![message]);
        while e.call(QUEUE_TRY_POP, &args![this, message]).bool() {}
        e.call(MESSAGE_DESTRUCT, &args![message]);
    });
    fn_00905650(e, this);
}

// Translated from 00905650 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor of `BSTCommonMessageQueue<ActorPathingMessage>` (unnamed in the
/// map): installs its vtable and runs the base destructor (`00905670`).
/// Returns `this`.
pub fn fn_00905650(e: &mut Engine, this: Ptr) -> Ptr {
    e.mem.set_u32(this.addr(), COMMON_QUEUE_VTABLE);
    fn_00905670(e, this)
}

// Translated from 00905670 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor of the message queue's base class (unnamed in the map): only
/// installs its vtable. Returns `this`.
pub fn fn_00905670(e: &mut Engine, this: Ptr) -> Ptr {
    e.mem.set_u32(this.addr(), QUEUE_BASE_VTABLE);
    this
}

// Translated from 00905690 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor of `BSTCommonMessageQueue<ActorPathingMessage>`
/// (unnamed in the map): the destructor (`00905650`), then `operator delete`
/// when bit 0 of `flags` is set. Returns `this`.
pub fn fn_00905690(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_00905650(e, this);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 009056c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSTMessageQueue<ActorPathingMessage>::_scalar_deleting_destructor_` (Xbox
/// PDB): the base destructor (`00905670`), then `operator delete` when bit 0
/// of `flags` is set. Returns `this`.
pub fn bst_message_queue_actor_pathing_message_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    fn_00905670(e, this);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 009056f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSTCommonLLMessageQueue<ActorPathingMessage>::DoTryPush` (Xbox PDB): with
/// a free list at +8, takes a free node for `message` (`00905aa0`); on
/// success appends it at the tail (the tail pointer at +0x10, reset to the
/// head slot at +0xc when the queue is empty) and returns true. False when
/// there is no free list or no free node.
pub fn bst_common_ll_message_queue_actor_pathing_message_do_try_push(
    e: &mut Engine,
    this: Ptr,
    message: Ptr,
) -> bool {
    let base = this.addr();
    let free_list = e.mem.u32(base + 8);
    if free_list == 0 {
        return false;
    }
    let taken = e.with_stack(4, |e, out| {
        if fn_00905aa0(e, Ptr::new(free_list), out, message) {
            Some(e.mem.u32(out.addr()))
        } else {
            None
        }
    });
    let Some(node) = taken else {
        return false;
    };
    if e.mem.u32(base + 0xc) == 0 {
        e.mem.set_u32(base + 0x10, base + 0xc);
    }
    let tail = e.mem.u32(base + 0x10);
    e.mem.set_u32(tail, node);
    e.mem.set_u32(node + 8, 0);
    e.mem.set_u32(base + 0x10, node + 8);
    true
}

// Translated from 00905770 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSTCommonLLMessageQueue<ActorPathingMessage>::DoTryPop` (Xbox PDB): with a
/// head node at +0xc, assigns its message (`006e9c60`) to `message`, makes
/// the next node (+8) the head, hands the node back to the free list
/// (`009059c0`) and returns true; false when the queue is empty. The tail
/// pointer is left as it is.
pub fn bst_common_ll_message_queue_actor_pathing_message_do_try_pop(
    e: &mut Engine,
    this: Ptr,
    message: Ptr,
) -> bool {
    let base = this.addr();
    let node = e.mem.u32(base + 0xc);
    if node == 0 {
        return false;
    }
    let item = e.call(LIST_NODE_ITEM, &args![node]).u32();
    e.call(MESSAGE_ASSIGN, &args![message, item]);
    let next = e.mem.u32(node + 8);
    e.mem.set_u32(base + 0xc, next);
    let free_list = e.mem.u32(base + 8);
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), node);
        bst_free_list_actor_pathing_message_deallocate(e, Ptr::new(free_list), slot);
    });
    true
}

// Translated from 009057d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Assignment of a reference-counted pointer (unnamed in the map): `this`
/// points at the pointer slot. When the new object differs from the stored
/// one, releases the old one (`00401970` on its +0x14) if any, stores the
/// new one and acquires it (`0040f6e0` on its +0x14) if not null. Returns
/// `this`.
pub fn fn_009057d0(e: &mut Engine, this: Ptr, object: u32) -> Ptr {
    let old = e.mem.u32(this.addr());
    if old != object {
        if old != 0 {
            e.call(REFERENCE_RELEASE, &args![old + 0x14]);
        }
        e.mem.set_u32(this.addr(), object);
        if object != 0 {
            e.call(REFERENCE_ACQUIRE, &args![object + 0x14]);
        }
    }
    this
}

// Translated from 00905820 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleList` append (unnamed in the map; `this` is the list's head
/// node, `item` points at the item to add). Does nothing when the item is
/// null. Walks to the last node; if that node already holds an item, a new
/// 8-byte node is allocated (`operator new`, node constructor `00470440`)
/// and linked after it, otherwise the item goes into that node. The C++
/// unwinding state around the allocation is not translated.
pub fn fn_00905820(e: &mut Engine, this: Ptr, item: Ptr) {
    if e.mem.u32(item.addr()) == 0 {
        return;
    }
    let mut node = this.addr();
    loop {
        let next = e.mem.u32(node + 4);
        if next == 0 {
            break;
        }
        node = next;
    }
    if e.mem.u32(node) != 0 {
        let memory = e.call(OPERATOR_NEW, &args![8u32]).u32();
        let created = if memory != 0 {
            e.call(LIST_NODE_CONSTRUCT, &args![memory, item]).u32()
        } else {
            0
        };
        e.mem.set_u32(node + 4, created);
    } else {
        let value = e.mem.u32(item.addr());
        e.mem.set_u32(node, value);
    }
}

// Translated from 009058e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of a `BSSimpleArray<TESIdleForm *,1024>` (the class whose
/// destructor is `00905910`; unnamed in the map): installs the vtable and
/// runs the array constructor helper (`006b3eb0`) with `size` as both the
/// size and the count. Returns `this`.
pub fn fn_009058e0(e: &mut Engine, this: Ptr, size: u32) -> Ptr {
    e.mem.set_u32(this.addr(), IDLE_FORM_ARRAY_VTABLE);
    e.call(ARRAY_CONSTRUCT, &args![this, size, size]);
    this
}

// Translated from 00905910 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor of `BSSimpleArray<TESIdleForm *,1024>` (unnamed in the map):
/// installs the vtable and clears the array, freeing its buffer
/// (`008454f0` with 1).
pub fn fn_00905910(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), IDLE_FORM_ARRAY_VTABLE);
    e.call(ARRAY_CLEAR, &args![this, 1u32]);
}

// Translated from 00905930 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSTCommonLLMessageQueue<ActorPathingMessage>::_scalar_deleting_destructor_`
/// (Xbox PDB): the destructor (`009055c0`), then `operator delete` when bit 0
/// of `flags` is set. Returns `this`.
pub fn bst_common_ll_message_queue_actor_pathing_message_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    bst_common_ll_message_queue_actor_pathing_message_destructor(e, this);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 00905960 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<TESIdleForm *,1024>::_scalar_deleting_destructor_` (Xbox
/// PDB, `BSSimpleArray<TESIdleForm_P_1024>`): the destructor (`00905910`),
/// then `operator delete` when bit 0 of `flags` is set. Returns `this`.
pub fn bs_simple_array_tes_idle_form_p_1024_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    fn_00905910(e, this);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 00905990 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of `BSTCommonMessageQueue<ActorPathingMessage>` (unnamed in
/// the map): the base constructor (`009059f0`), then its vtable and a zero
/// at +4 (the lock word). Returns `this`.
pub fn fn_00905990(e: &mut Engine, this: Ptr) -> Ptr {
    fn_009059f0(e, this);
    e.mem.set_u32(this.addr(), COMMON_QUEUE_VTABLE);
    e.mem.set_u32(this.addr() + 4, 0);
    this
}

// Translated from 009059c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSTFreeList<ActorPathingMessage>::Deallocate` (Xbox PDB): releases the
/// message of the node `*node_slot` (`00905ae0`), then gives the node back
/// to the free list (`006ecc70`, which zeroes the slot).
pub fn bst_free_list_actor_pathing_message_deallocate(e: &mut Engine, this: Ptr, node_slot: Ptr) {
    let node = e.mem.u32(node_slot.addr());
    fn_00905ae0(e, Ptr::new(node));
    e.call(FREE_LIST_GIVE, &args![this, node_slot]);
}

// Translated from 009059f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of the message queue's base class (unnamed in the map): only
/// installs the vtable. Returns `this`.
pub fn fn_009059f0(e: &mut Engine, this: Ptr) -> Ptr {
    e.mem.set_u32(this.addr(), QUEUE_BASE_VTABLE);
    this
}

// Translated from 00905a10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Reserves one more element of a `BSSimpleArray` (unnamed in the map; the
/// fields are the buffer at +4, count at +8 and capacity at +0xc). When the
/// array is full (`00438b90`): with no capacity yet, allocates 4 elements
/// through the allocator at vtable +4 and takes capacity 4; otherwise grows
/// to the next capacity (`009a3910`) with the resize helper (`006e3d30`).
/// Then counts the new element and returns its index.
pub fn fn_00905a10(e: &mut Engine, this: Ptr) -> u32 {
    let base = this.addr();
    if e.call(ARRAY_IS_FULL, &args![this]).bool() {
        if e.mem.u32(base + 0xc) == 0 {
            let capacity = 4u32;
            let buffer = e.vcall(base, 4, &args![capacity]).u32();
            e.mem.set_u32(base + 4, buffer);
            e.mem.set_u32(base + 0xc, capacity);
        } else {
            let capacity = e.call(ARRAY_NEXT_CAPACITY, &args![this]).u32();
            let count = e.mem.u32(base + 8);
            e.call(ARRAY_RESIZE, &args![this, capacity, count]);
            e.mem.set_u32(base + 0xc, capacity);
        }
    }
    let count = e.mem.u32(base + 8).wrapping_add(1);
    e.mem.set_u32(base + 8, count);
    count.wrapping_sub(1)
}

// Translated from 00905aa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Takes a free node for a message (unnamed in the map; `this` is the free
/// list): `006ecd60` stores a node in `*out_node`; if it found one, the node
/// is constructed for `message` (`00905b00`) and the result is true.
pub fn fn_00905aa0(e: &mut Engine, this: Ptr, out_node: Ptr, message: Ptr) -> bool {
    if !e.call(FREE_LIST_TAKE, &args![this, out_node]).bool() {
        return false;
    }
    let node = e.mem.u32(out_node.addr());
    e.call(FREE_NODE_CONSTRUCT, &args![node, message]);
    true
}

// Translated from 00905ae0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Releases the message held by a free-list node (unnamed in the map): the
/// holder's scalar deleting destructor (`009bc890`) with flags 0, so the
/// message is destroyed but the memory is kept.
pub fn fn_00905ae0(e: &mut Engine, this: Ptr) {
    e.call(MESSAGE_HOLDER_DESTRUCT, &args![this, 0u32]);
}

// Translated from 009b6600 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MiddleHighProcess::GetWeaponEnchantmentVisuals` (Xbox PDB): the
/// `pCurrentWeaponEffect (TESEffectShader*)` at +0x16c.
pub fn middle_high_process_get_weapon_enchantment_visuals(e: &mut Engine, this: Ptr) -> Ptr {
    Ptr::new(e.mem.u32(this.addr() + 0x16c))
}

// Translated from 009b88a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `LowProcess::GetDeathTime` (Xbox PDB): the float at +0xa8.
pub fn low_process_get_death_time(e: &mut Engine, this: Ptr) -> f32 {
    e.mem.f32(this.addr() + 0xa8)
}

// Translated from 009b88c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `LowProcess::GetTrackedDamage` (Xbox PDB): the float at +0xac.
pub fn low_process_get_tracked_damage(e: &mut Engine, this: Ptr) -> f32 {
    e.mem.f32(this.addr() + 0xac)
}

// Translated from 009ee020 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MiddleHighProcess::GetBSBound` (Xbox PDB): the `pBSBound (BSBound*)` at
/// +0x224.
pub fn middle_high_process_get_bs_bound(e: &mut Engine, this: Ptr) -> Ptr {
    Ptr::new(e.mem.u32(this.addr() + 0x224))
}

/// This part's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00903160, fn_00903160(Ptr, f32)),
        entry!(
            0x00903180,
            high_process_add_post_animation_action(Ptr<HighProcess>, u32)
        ),
        entry!(
            0x009031b0,
            high_process_evaluate_combat_detection_lists(Ptr<HighProcess>, Ptr)
        ),
        entry!(0x00903600, fn_00903600(Ptr, u32) -> u32),
        entry!(
            0x00903620,
            high_process_update_radiation(Ptr<HighProcess>, Ptr, f32)
        ),
        entry!(
            0x009037e0,
            high_process_fire_at_object(Ptr<HighProcess>, Ptr, Ptr)
        ),
        entry!(
            0x00903a80,
            high_process_start_new_package(Ptr<HighProcess>, Ptr, Ptr)
        ),
        entry!(0x00903bb0, fn_00903bb0(Ptr) -> Ptr),
        entry!(0x00903bd0, fn_00903bd0(Ptr) -> Ptr),
        entry!(
            0x00903bf0,
            high_process_change_package(Ptr<HighProcess>, Ptr, Ptr) -> u32
        ),
        entry!(0x00903cd0, fn_00903cd0(Ptr) -> Ptr),
        entry!(0x00903cf0, fn_00903cf0(Ptr) -> Ptr),
        entry!(
            0x00903d10,
            high_process_package_done(Ptr<HighProcess>, Ptr, Ptr)
        ),
        entry!(0x00903de0, fn_00903de0(Ptr) -> Ptr),
        entry!(0x00903e00, fn_00903e00(Ptr) -> Ptr),
        entry!(0x00903e20, fn_00903e20(Ptr) -> Ptr),
        entry!(
            0x00903e40,
            high_process_set_actors_detection_event(
                Ptr<HighProcess>,
                u32,
                f32,
                f32,
                f32,
                u32,
                u32,
                Ptr,
            )
        ),
        entry!(
            0x00903f50,
            high_process_remove_detection_event(Ptr<HighProcess>)
        ),
        entry!(
            0x00903f90,
            high_process_check_for_expired_detection_event(Ptr<HighProcess>)
        ),
        entry!(
            0x00904000,
            high_process_notify_guard_about_activation(Ptr<HighProcess>, Ptr, Ptr, Ptr)
        ),
        entry!(
            0x009040f0,
            high_process_is_avoid_area_in_avoid_pathing_list(Ptr<HighProcess>, Ptr) -> bool
        ),
        entry!(0x00904160, high_process_clear_avoid_areas(Ptr<HighProcess>)),
        entry!(
            0x00904220,
            high_process_remove_avoid_pathing_node(Ptr<HighProcess>, Ptr)
        ),
        entry!(
            0x009042a0,
            high_process_add_avoid_pathing_area(
                Ptr<HighProcess>,
                Ptr,
                f32,
                f32,
                f32,
                f32,
                f32,
                f32,
                Ptr,
                Ptr,
            )
        ),
        entry!(0x00904430, fn_00904430(Ptr<HighProcess>) -> f32),
        entry!(
            0x00904450,
            high_process_create_avoid_array(Ptr<HighProcess>, Ptr) -> Ptr
        ),
        entry!(
            0x00904540,
            high_process_process_avoid_area(Ptr<HighProcess>, Ptr)
        ),
        entry!(
            0x00904800,
            high_process_check_if_there_someone_talk_with(Ptr<HighProcess>, Ptr) -> bool
        ),
        entry!(
            0x00904c80,
            high_process_compute_allow_sandbox_conversation(Ptr<HighProcess>, Ptr, Ptr) -> bool
        ),
        entry!(
            0x00904da0,
            high_process_process_cannibal(Ptr<HighProcess>, Ptr)
        ),
        entry!(
            0x00904f50,
            high_process_process_sandman(Ptr<HighProcess>, Ptr)
        ),
        entry!(0x00905230, fn_00905230(Ptr) -> i32),
        entry!(
            0x00905250,
            high_process_set_path_look_at_target(Ptr<HighProcess>, Ptr)
        ),
        entry!(
            0x00905280,
            high_process_set_animation(Ptr<HighProcess>, Ptr)
        ),
        entry!(0x009052c0, fn_009052c0(Ptr, u32) -> Ptr),
        entry!(0x009052e0, fn_009052e0(Ptr, Ptr) -> Ptr),
        entry!(0x00905330, fn_00905330(Ptr, Ptr)),
        entry!(0x00905450, bs_simple_array_pathing_avoid_node_add(Ptr, Ptr) -> u32),
        entry!(0x009054a0, bst_common_message_queue_actor_pathing_message_push(Ptr, Ptr) -> bool),
        entry!(0x00905510, bst_common_message_queue_actor_pathing_message_pop(Ptr, Ptr) -> bool),
        entry!(0x00905580, fn_00905580(Ptr, u32) -> Ptr),
        entry!(
            0x009055c0,
            bst_common_ll_message_queue_actor_pathing_message_destructor(Ptr)
        ),
        entry!(0x00905650, fn_00905650(Ptr) -> Ptr),
        entry!(0x00905670, fn_00905670(Ptr) -> Ptr),
        entry!(0x00905690, fn_00905690(Ptr, u32) -> Ptr),
        entry!(
            0x009056c0,
            bst_message_queue_actor_pathing_message_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(
            0x009056f0,
            bst_common_ll_message_queue_actor_pathing_message_do_try_push(Ptr, Ptr) -> bool
        ),
        entry!(
            0x00905770,
            bst_common_ll_message_queue_actor_pathing_message_do_try_pop(Ptr, Ptr) -> bool
        ),
        entry!(0x009057d0, fn_009057d0(Ptr, u32) -> Ptr),
        entry!(0x00905820, fn_00905820(Ptr, Ptr)),
        entry!(0x009058e0, fn_009058e0(Ptr, u32) -> Ptr),
        entry!(0x00905910, fn_00905910(Ptr)),
        entry!(
            0x00905930,
            bst_common_ll_message_queue_actor_pathing_message_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(
            0x00905960,
            bs_simple_array_tes_idle_form_p_1024_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(0x00905990, fn_00905990(Ptr) -> Ptr),
        entry!(
            0x009059c0,
            bst_free_list_actor_pathing_message_deallocate(Ptr, Ptr)
        ),
        entry!(0x009059f0, fn_009059f0(Ptr) -> Ptr),
        entry!(0x00905a10, fn_00905a10(Ptr) -> u32),
        entry!(0x00905aa0, fn_00905aa0(Ptr, Ptr, Ptr) -> bool),
        entry!(0x00905ae0, fn_00905ae0(Ptr)),
        entry!(0x009b6600, middle_high_process_get_weapon_enchantment_visuals(Ptr) -> Ptr),
        entry!(0x009b88a0, low_process_get_death_time(Ptr) -> f32),
        entry!(0x009b88c0, low_process_get_tracked_damage(Ptr) -> f32),
        entry!(0x009ee020, middle_high_process_get_bs_bound(Ptr) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::{Cell, RefCell};
    use std::rc::Rc;

    const ACTOR_VT: u32 = 0x7100_0000;
    const PROCESS_VT: u32 = 0x7200_0000;
    const OTHER_VT: u32 = 0x7300_0000;
    const VALUE_VT: u32 = 0x7400_0000;
    const SETTINGS_TABLE: u32 = 0x7ff0_0000;
    const FRAME_CELL: u32 = 0x7ff3_0000;

    /// Every callee outside this file that the functions under test may
    /// reach. They return 0 unless a test registers a double over them.
    const STUBS: &[u32] = &[
        0x0040_8820,
        0x0040_8860,
        0x0040_ebd0,
        0x0041_81e0,
        0x0041_8830,
        0x0041_ca90,
        0x0042_5fd0,
        0x0043_01b0,
        0x0043_5dd0,
        0x0043_5de0,
        0x0043_5e00,
        0x0043_7b90,
        0x0044_b010,
        0x0044_b050,
        0x0048_7f50,
        0x0049_3bb0,
        0x0049_5e40,
        0x0049_85f0,
        0x0050_7610,
        0x0051_f5f0,
        0x0052_4b40,
        0x0054_6930,
        0x0055_b980,
        0x0056_4db0,
        0x0056_8650,
        0x0057_2380,
        0x0057_23b0,
        0x005a_c1e0,
        0x005a_e3d0,
        0x005c_c7a0,
        0x005f_25d0,
        0x005f_65d0,
        0x0060_2150,
        0x0064_65f0,
        0x0066_98d0,
        0x0067_1d10,
        0x0067_5a50,
        0x0067_5c20,
        0x0067_8610,
        0x0067_efd0,
        0x0069_15d0,
        0x006d_3b00,
        0x006d_61e0,
        0x006d_ad70,
        0x006d_ad90,
        0x006e_29f0,
        0x006e_2b50,
        0x006e_2bb0,
        0x006e_3850,
        0x006e_3c90,
        0x006e_5390,
        0x0070_f490,
        0x0088_4f80,
        0x0089_35f0,
        0x0089_7bd0,
        0x0089_82c0,
        0x0089_d900,
        0x008a_6650,
        0x008a_78f0,
        0x008a_80c0,
        0x008b_0670,
        0x008b_06d0,
        0x008b_3690,
        0x008b_3bb0,
        0x008b_3bd0,
        0x008b_87a0,
        0x008b_b630,
        0x008b_cc80,
        0x008b_ffc0,
        0x008c_0460,
        0x008d_6f30,
        0x008d_8520,
        0x008d_f000,
        0x008f_db10,
        0x008f_db90,
        0x0090_1c50,
        0x0090_5a10,
        0x0091_fe80,
        0x0093_44a0,
        0x0093_dd20,
        0x0096_11e0,
        0x0096_8730,
        0x0096_a2d0,
        0x0099_2480,
        0x009f_1310,
        0x00aa_13e0,
        0x00ec_62c0,
    ];

    fn word(value: u32) -> Ret {
        Ret {
            eax: value,
            ..Ret::default()
        }
    }

    fn float(value: f32) -> Ret {
        Ret {
            st0: value as f64,
            ..Ret::default()
        }
    }

    /// An engine with the globals the code reads mapped, every outside callee
    /// stubbed, and the list, setting and frame-time helpers behaving like
    /// the game's (reading the lists and settings the test builds).
    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(PLAYER_POINTER, 4);
        e.map(COMBAT_MANAGER_POINTER, 4);
        for (address, value) in [
            (KNOCK_FLOAT, -1.0f32),
            (DEFAULT_TALK_DISTANCE, 200.0),
            (FLEE_REQUEST_FIRST_FLOAT, 20.0),
            (FLEE_REQUEST_SECOND_FLOAT, 300.0),
            (SANDMAN_PACKAGE_FLOAT, 40.0),
        ] {
            e.map(address, 4);
            e.mem.set_f32(address, value);
        }
        e.map(SANDMAN_RANGE, 8);
        e.mem.set_f64(SANDMAN_RANGE, 40.0);
        e.map(SETTINGS_TABLE, 0x2_0000);
        e.map(FRAME_CELL, 8);
        for address in STUBS {
            e.register(*address, |_, _| Ret::default());
        }
        e.register(LIST_NODE_ITEM, |_, a| word(a[0]));
        e.register(WORD_AT_4, |e, a| word(e.mem.u32(a[0] + 4)));
        e.register(WORD_AT_8, |e, a| word(e.mem.u32(a[0] + 8)));
        e.register(NI_POINTER_GET, |e, a| word(e.mem.u32(a[0])));
        e.register(LIST_IS_EMPTY, |e, a| {
            word((e.mem.u32(a[0]) == 0 && e.mem.u32(a[0] + 4) == 0) as u32)
        });
        e.register(LIST_CLEAR, |e, a| {
            e.mem.set_u32(a[0], 0);
            e.mem.set_u32(a[0] + 4, 0);
            Ret::default()
        });
        e.register(LIST_DELETE, |_, _| Ret::default());
        e.register(OPERATOR_DELETE, |_, _| Ret::default());
        e.register(SETTING_FLOAT, |e, a| {
            let cell = e.mem.u32(SETTINGS_TABLE + (a[0] & 0x1_ffff));
            assert_ne!(cell, 0, "setting {:08x} not set by the test", a[0]);
            word(cell)
        });
        e.register(FRAME_TIME, |e, _| float(e.mem.f32(FRAME_CELL)));
        e.register(EXTRA_DATA_LIST_OF, |_, a| word(a[0] + 0x44));
        e.register(SCRIPT_LOCALS, |_, _| word(0x9999));
        e.register(0x0054_6930, |e, a| word(e.mem.u32(a[0] + PACKAGE_ON_BEGIN)));
        e.register(0x0066_98d0, |e, a| {
            word(e.mem.u32(a[0] + PACKAGE_ON_CHANGE))
        });
        e.register(0x006d_ad90, |_, _| word(FRAME_CELL + 4));
        e.register(COMPARE_EXCHANGE_LOCK, |_, _| Ret::default());
        e.register(DEBUG_CHECK, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e
    }

    fn put_setting(e: &mut Engine, setting: u32, value: f32) {
        let cell = e.mem.alloc(4);
        e.mem.set_f32(cell, value);
        e.mem.set_u32(SETTINGS_TABLE + (setting & 0x1_ffff), cell);
    }

    fn calls(e: &Engine, address: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == address)
            .map(|(_, words)| words.clone())
            .collect()
    }

    fn vt_fn(table: u32, slot: u32) -> u32 {
        table + 0x800 + slot
    }

    fn vtable(e: &mut Engine, object: u32, table: u32) {
        e.map(table, 0x1000);
        e.mem.set_u32(object, table);
    }

    fn on_slot(
        e: &mut Engine,
        table: u32,
        slot: u32,
        f: impl FnMut(&mut Engine, &[u32]) -> Ret + 'static,
    ) {
        e.map(table, 0x1000);
        e.mem.set_u32(table + slot, vt_fn(table, slot));
        e.register_double(vt_fn(table, slot), f);
    }

    fn slot_ret(e: &mut Engine, table: u32, slot: u32, value: u32) {
        on_slot(e, table, slot, move |_, _| word(value));
    }

    /// The argument lists (with `this` first) of the calls made through one
    /// vtable slot.
    fn slot_calls(e: &Engine, table: u32, slot: u32) -> Vec<Vec<u32>> {
        calls(e, vt_fn(table, slot))
    }

    fn fill_list(e: &mut Engine, head: u32, items: &[u32]) {
        let mut node = head;
        for (i, item) in items.iter().enumerate() {
            e.mem.set_u32(node, *item);
            if i + 1 < items.len() {
                let next = e.mem.alloc(8);
                e.mem.set_u32(node + 4, next);
                node = next;
            }
        }
    }

    fn deleted(e: &Engine) -> Vec<u32> {
        calls(e, OPERATOR_DELETE).iter().map(|a| a[0]).collect()
    }

    /// Gives every slot of a vtable a double that returns 0; tests override the
    /// slots they care about.
    fn default_slots(e: &mut Engine, table: u32) {
        for slot in (0..0x700).step_by(4) {
            on_slot(e, table, slot, |_, _| Ret::default());
        }
    }

    struct Scene {
        e: Engine,
        process: u32,
        actor: u32,
        player: u32,
    }

    /// A process and an actor with vtables (every slot unset), and a player.
    fn scene() -> Scene {
        let mut e = engine();
        let process = e.mem.alloc(0x500);
        let actor = e.mem.alloc(0x400);
        let player = e.mem.alloc(0x400);
        vtable(&mut e, process, PROCESS_VT);
        vtable(&mut e, actor, ACTOR_VT);
        default_slots(&mut e, PROCESS_VT);
        default_slots(&mut e, ACTOR_VT);
        e.mem.set_u32(PLAYER_POINTER, player);
        Scene {
            e,
            process,
            actor,
            player,
        }
    }

    fn combat_entry(e: &mut Engine, target: u32, group: u32, alarm: u8, fleeing: u8) -> u32 {
        let entry = e.mem.alloc(0x14);
        e.mem.set_u32(entry, target);
        e.mem.set_u32(entry + 4, group);
        e.mem.set_u8(entry + 9, alarm);
        e.mem.set_u8(entry + 0xb, fleeing);
        entry
    }

    #[test]
    fn test_fn_00903160() {
        let mut e = engine();
        let object = e.mem.alloc(0x20);
        e.mem.set_f32(object + 0x10, 1.5);
        e.call(0x0090_3160, &args![object, 2.25f32]);
        assert_eq!(e.mem.f32(object + 0x10), 3.75);
        assert_eq!(e.mem.f32(object + 0x14), 0.0);
    }

    #[test]
    fn test_high_process_add_post_animation_action() {
        let mut s = scene();
        s.e.mem.set_u32(s.process + 0x424, 0x1);
        s.e.call(0x0090_3180, &args![s.process, 0x4u32]);
        assert_eq!(s.e.mem.u32(s.process + 0x424), 0x5);
        s.e.call(0x0090_3180, &args![s.process, 0x1u32]);
        assert_eq!(s.e.mem.u32(s.process + 0x424), 0x5);
    }

    #[test]
    fn test_high_process_evaluate_combat_detection_lists_groups() {
        let mut s = scene();
        let (process, actor) = (s.process, s.actor);
        slot_ret(&mut s.e, ACTOR_VT, 0x3f8, 7);
        slot_ret(&mut s.e, ACTOR_VT, 0x428, 1);
        let same = combat_entry(&mut s.e, 0x5001, 7, 0, 0);
        let other = combat_entry(&mut s.e, 0x5002, 9, 1, 1);
        fill_list(&mut s.e, process + 0x27c, &[same, other]);
        // The group's lookup points at the word 0x6000, the member; the
        // member's process answers the aggressor query with 1.
        let cell = s.e.mem.alloc(4);
        s.e.mem.set_u32(cell, 0x6000);
        s.e.register(0x006d_ad90, |e, a| {
            let _ = a;
            word(e.mem.u32(FRAME_CELL + 4))
        });
        s.e.mem.set_u32(FRAME_CELL + 4, cell);
        let member_process = s.e.mem.alloc(0x40);
        vtable(&mut s.e, member_process, OTHER_VT);
        slot_ret(&mut s.e, OTHER_VT, 0x1c4, 1);
        s.e.register_double(REFERENCE_PROCESS, move |_, a| {
            assert_eq!(a[0], 0x6000);
            word(member_process)
        });
        s.e.call(0x0090_31b0, &args![process, actor]);
        // Only the entry of the other group enters combat, and is freed.
        assert_eq!(
            slot_calls(&s.e, PROCESS_VT, 0x33c),
            vec![vec![process, actor, 0, 0, 1, 9, 1, 0, 1, 0, 0, 0, 1, 0]]
        );
        assert_eq!(calls(&s.e, 0x006d_ad90), vec![vec![9 + 0x18, 0]]);
        assert_eq!(deleted(&s.e), vec![other]);
        // The empty aggro list triggers the process's vtable +0xd0 call.
        assert_eq!(slot_calls(&s.e, PROCESS_VT, 0xd0), vec![vec![process, 0]]);
        // The four lists are cleared, in order.
        let cleared: Vec<u32> = calls(&s.e, LIST_CLEAR).iter().map(|a| a[0]).collect();
        assert_eq!(
            cleared,
            vec![
                process + 0x27c,
                process + 0x274,
                process + 0x284,
                process + 0x28c
            ]
        );
    }

    #[test]
    fn test_high_process_evaluate_combat_detection_lists_group_stops() {
        // Cannot attack any more after the first entry: the second is kept.
        let mut s = scene();
        let (process, actor) = (s.process, s.actor);
        slot_ret(&mut s.e, ACTOR_VT, 0x3f8, 7);
        slot_ret(&mut s.e, ACTOR_VT, 0x428, 0);
        let first = combat_entry(&mut s.e, 0x5001, 9, 0, 0);
        let second = combat_entry(&mut s.e, 0x5002, 9, 0, 0);
        fill_list(&mut s.e, process + 0x27c, &[first, second]);
        s.e.call(0x0090_31b0, &args![process, actor]);
        assert_eq!(slot_calls(&s.e, PROCESS_VT, 0x33c).len(), 1);
        assert_eq!(deleted(&s.e), vec![first]);

        // The actor's own group changes after the first entry: same stop.
        let mut s = scene();
        let (process, actor) = (s.process, s.actor);
        let asked = Rc::new(Cell::new(0));
        let counter = asked.clone();
        on_slot(&mut s.e, ACTOR_VT, 0x3f8, move |_, _| {
            counter.set(counter.get() + 1);
            word(if counter.get() == 1 { 7 } else { 8 })
        });
        slot_ret(&mut s.e, ACTOR_VT, 0x428, 1);
        let first = combat_entry(&mut s.e, 0x5001, 9, 0, 0);
        let second = combat_entry(&mut s.e, 0x5002, 9, 0, 0);
        fill_list(&mut s.e, process + 0x27c, &[first, second]);
        s.e.call(0x0090_31b0, &args![process, actor]);
        assert_eq!(slot_calls(&s.e, PROCESS_VT, 0x33c).len(), 1);
        assert_eq!(deleted(&s.e), vec![first]);
    }

    #[test]
    fn test_high_process_evaluate_combat_detection_lists_aggro() {
        let mut s = scene();
        let (process, actor, player) = (s.process, s.actor, s.player);
        let (attacked, ignored, reported) = (0x5100u32, 0x5200u32, 0x5300u32);
        let calm = combat_entry(&mut s.e, attacked, 0, 1, 0);
        let on_player = combat_entry(&mut s.e, player, 0, 0, 0);
        let already = combat_entry(&mut s.e, reported, 0, 0, 0);
        let _ = ignored;
        fill_list(&mut s.e, process + 0x274, &[calm, on_player, already]);
        // 008b87a0 returns a reaction and reports through its byte for the
        // third target only; 008bffc0 says the actor is angry.
        s.e.register(0x008b_87a0, |e, a| {
            let reported = a[1] == 0x5300;
            e.mem.set_u8(a[2], reported as u8);
            word(0x55)
        });
        s.e.register(0x008b_ffc0, |_, _| word(1));
        let reaction = Rc::new(Cell::new(0));
        let seen = reaction.clone();
        s.e.register_double(0x008b_06d0, move |e, a| {
            seen.set(e.mem.u32(a[3]));
            word(1)
        });
        s.e.call(0x0090_31b0, &args![process, actor]);
        // Asked once, for the first target; the block held the reaction.
        assert_eq!(calls(&s.e, 0x008b_06d0).len(), 1);
        assert_eq!(calls(&s.e, 0x008b_06d0)[0][..3], [actor, attacked, 0]);
        assert_eq!(reaction.get(), 0x55);
        let combat = slot_calls(&s.e, PROCESS_VT, 0x33c);
        assert_eq!(
            combat,
            vec![
                vec![process, actor, attacked, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0],
                vec![process, actor, player, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0],
                vec![process, actor, reported, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0],
            ]
        );
        assert_eq!(deleted(&s.e), vec![calm, on_player, already]);
        // A non-empty aggro list does not call vtable +0xd0.
        assert!(slot_calls(&s.e, PROCESS_VT, 0xd0).is_empty());
    }

    #[test]
    fn test_high_process_evaluate_combat_detection_lists_after_lists() {
        // A non-empty radius list calls 00897bd0 unless a run-once package of
        // kind 0xe is running.
        let mut s = scene();
        let (process, actor) = (s.process, s.actor);
        fill_list(&mut s.e, process + 0x38c, &[0x7001]);
        s.e.call(0x0090_31b0, &args![process, actor]);
        assert_eq!(calls(&s.e, 0x0089_7bd0), vec![vec![actor, actor]]);

        let mut s = scene();
        let (process, actor) = (s.process, s.actor);
        fill_list(&mut s.e, process + 0x38c, &[0x7001]);
        slot_ret(&mut s.e, PROCESS_VT, 0x20c, 0x7700);
        s.e.register(0x0041_ca90, |_, a| {
            word(if a[0] == 0x7700 { 0xe } else { 0 })
        });
        s.e.call(0x0090_31b0, &args![process, actor]);
        assert!(calls(&s.e, 0x0089_7bd0).is_empty());

        // An actor the answer of 00493bb0 puts in a group skips everything.
        let mut s = scene();
        let (process, actor) = (s.process, s.actor);
        fill_list(&mut s.e, process + 0x394, &[0x7001]);
        s.e.register(0x0049_3bb0, |_, _| word(1));
        s.e.call(0x0090_31b0, &args![process, actor]);
        assert!(slot_calls(&s.e, ACTOR_VT, 0x410).is_empty());
        assert!(calls(&s.e, 0x0099_2480).is_empty());
        assert!(calls(&s.e, 0x0089_7bd0).is_empty());
    }

    #[test]
    fn test_high_process_evaluate_combat_detection_lists_avoid_and_spectators() {
        // Avoided actors: knocked unless 008a6650 says no; added to a running
        // package of kind 0x16.
        let mut s = scene();
        let (process, actor) = (s.process, s.actor);
        fill_list(&mut s.e, process + 0x394, &[0x7001, 0x7002]);
        s.e.mem.set_f32(KNOCK_FLOAT, -1.0);
        s.e.register(0x008a_6650, |_, _| word(0));
        slot_ret(&mut s.e, PROCESS_VT, 0x27c, 0x7700);
        s.e.register(0x0041_ca90, |_, _| word(0x16));
        s.e.call(0x0090_31b0, &args![process, actor]);
        let minus_one = (-1.0f32).to_bits();
        assert_eq!(
            slot_calls(&s.e, ACTOR_VT, 0x410),
            vec![
                vec![actor, 0x7001, 0, 1, 1, 0, 0, minus_one, minus_one],
                vec![actor, 0x7002, 0, 1, 1, 0, 0, minus_one, minus_one],
            ]
        );
        assert_eq!(
            calls(&s.e, 0x009f_1310),
            vec![vec![0x7700, 0x7001], vec![0x7700, 0x7002]]
        );
        assert!(calls(&s.e, 0x0099_2480).is_empty());

        // No knock when 008a6650 says so; no package: nothing added.
        let mut s = scene();
        let (process, actor) = (s.process, s.actor);
        fill_list(&mut s.e, process + 0x394, &[0x7001]);
        s.e.register(0x008a_6650, |_, _| word(1));
        s.e.call(0x0090_31b0, &args![process, actor]);
        assert!(slot_calls(&s.e, ACTOR_VT, 0x410).is_empty());
        assert!(calls(&s.e, 0x009f_1310).is_empty());

        // Spectators go to the combat manager and are freed.
        let mut s = scene();
        let (process, actor) = (s.process, s.actor);
        let manager = s.e.mem.alloc(8);
        s.e.mem.set_u32(COMBAT_MANAGER_POINTER, manager);
        let one = combat_entry(&mut s.e, 0x7101, 0, 0, 0);
        let two = combat_entry(&mut s.e, 0x7102, 0, 0, 0);
        fill_list(&mut s.e, process + 0x28c, &[one, two]);
        s.e.call(0x0090_31b0, &args![process, actor]);
        assert_eq!(
            calls(&s.e, 0x0099_2480),
            vec![vec![manager, actor, 0x7101], vec![manager, actor, 0x7102]]
        );
        assert_eq!(deleted(&s.e), vec![one, two]);
    }

    #[test]
    fn test_fn_00903600() {
        let mut e = engine();
        let cell = e.mem.alloc(4);
        e.mem.set_u32(cell, 0x1234);
        e.register(0x006d_ad90, |e, _| word(e.mem.u32(FRAME_CELL + 4)));
        e.mem.set_u32(FRAME_CELL + 4, cell);
        let group = e.mem.alloc(0x40);
        assert_eq!(e.call(0x0090_3600, &args![group, 5u32]).u32(), 0x1234);
        assert_eq!(calls(&e, 0x006d_ad90), vec![vec![group + 0x18, 5]]);
    }

    #[test]
    fn test_high_process_update_radiation() {
        // Neither a follower nor flagged: nothing happens.
        let mut s = scene();
        let (process, actor) = (s.process, s.actor);
        s.e.mem.set_f32(process + 0x43c, 1.5);
        s.e.call(0x0090_3620, &args![process, actor, 0.5f32]);
        assert_eq!(s.e.mem.f32(process + 0x43c), 1.5);
        assert!(calls(&s.e, MINIMUM).is_empty());

        // A follower with no radiation: the timer grows and is capped by the
        // setting plus one.
        let mut s = scene();
        let (process, actor) = (s.process, s.actor);
        slot_ret(&mut s.e, ACTOR_VT, 0x360, 1);
        put_setting(&mut s.e, RADIATION_CAP_SETTING, 1.5);
        s.e.register(MINIMUM, |_, a| {
            float(f32::from_bits(a[0]).min(f32::from_bits(a[1])))
        });
        s.e.mem.set_f32(process + 0x43c, 2.0);
        s.e.call(0x0090_3620, &args![process, actor, 1.0f32]);
        assert_eq!(s.e.mem.f32(process + 0x43c), 2.5);
        assert_eq!(
            calls(&s.e, MINIMUM),
            vec![vec![3.0f32.to_bits(), 2.5f32.to_bits()]]
        );
        assert!(calls(&s.e, 0x0096_8730).is_empty());
        assert!(slot_calls(&s.e, ACTOR_VT, 0x3ac).is_empty());
    }

    /// The radiation case of `UpdateRadiation`: timer 1.5 + 0.5 = 2.0,
    /// highest radiation 3.0, `current` radiation on the actor.
    fn radiation(current: f32, absolute: f32) -> Scene {
        let mut s = scene();
        let (process, actor) = (s.process, s.actor);
        slot_ret(&mut s.e, ACTOR_VT, 0x21c, 1);
        put_setting(&mut s.e, RADIATION_CAP_SETTING, 5.0);
        put_setting(&mut s.e, RADIATION_SCALE_SETTING, 0.25);
        put_setting(&mut s.e, RADIATION_RATE_SETTING, 0.5);
        s.e.register(MINIMUM, |_, a| {
            float(f32::from_bits(a[0]).min(f32::from_bits(a[1])))
        });
        // The scaling helper gets the setting value 0.25 and answers 4.0.
        s.e.register(RADIATION_SCALE, |_, a| {
            assert_eq!(f32::from_bits(a[0]), 0.25);
            float(4.0)
        });
        s.e.register_double(ABSOLUTE_VALUE, move |_, _| float(absolute));
        s.e.mem.set_u32(actor + 0xa4, VALUE_VT);
        s.e.map(VALUE_VT, 0x1000);
        on_slot(&mut s.e, VALUE_VT, 0xc, move |_, a| {
            assert_eq!(a[1], 0x36);
            float(current)
        });
        s.e.mem.set_f32(process + 0x43c, 1.5);
        s.e.mem.set_f32(process + 0x440, 3.0);
        s.e.call(0x0090_3620, &args![process, actor, 0.5f32]);
        s
    }

    #[test]
    fn test_high_process_update_radiation_change() {
        // change = 4.0 * -2.0 + 0.5 * (3.0 * 2.0) = -5.0; the actor has 10.
        let s = radiation(10.0, 5.0);
        assert_eq!(
            slot_calls(&s.e, ACTOR_VT, 0x3ac),
            vec![vec![s.actor, 0x36, (-5.0f32).to_bits(), 0]]
        );
        assert_eq!(s.e.mem.f32(s.process + 0x43c), 0.0);
        assert_eq!(s.e.mem.f32(s.process + 0x440), 0.0);

        // With only 2.0 on the actor the change is limited to -2.0.
        let s = radiation(2.0, 2.0);
        assert_eq!(
            slot_calls(&s.e, ACTOR_VT, 0x3ac),
            vec![vec![s.actor, 0x36, (-2.0f32).to_bits(), 0]]
        );
        // A change whose absolute value is zero is not applied, but the
        // fields are still reset.
        let s = radiation(10.0, 0.0);
        assert!(slot_calls(&s.e, ACTOR_VT, 0x3ac).is_empty());
        assert_eq!(s.e.mem.f32(s.process + 0x440), 0.0);
    }

    #[test]
    fn test_high_process_update_radiation_player() {
        let mut s = scene();
        let (process, player) = (s.process, s.player);
        vtable(&mut s.e, player, OTHER_VT);
        default_slots(&mut s.e, OTHER_VT);
        slot_ret(&mut s.e, OTHER_VT, 0x360, 1);
        put_setting(&mut s.e, RADIATION_CAP_SETTING, 1.0);
        s.e.register(MINIMUM, |_, a| float(f32::from_bits(a[0])));
        s.e.call(0x0090_3620, &args![process, player, 0.5f32]);
        assert_eq!(calls(&s.e, 0x0096_8730), vec![vec![player]]);
    }

    /// Runs `FireAtObject` with the usual doubles: the process can attack
    /// and the weapon is accepted by `00524b40` (byte 1).
    fn fire(setup: impl FnOnce(&mut Scene)) -> Scene {
        let mut s = scene();
        slot_ret(&mut s.e, PROCESS_VT, 0x3f8, 1);
        s.e.register(0x0052_4b40, |_, a| word((a[0] != 0) as u32));
        s.e.register(0x0089_35f0, |_, _| word(1));
        setup(&mut s);
        let (process, actor) = (s.process, s.actor);
        s.e.call(0x0090_37e0, &args![process, actor, 0x6000u32]);
        s
    }

    #[test]
    fn test_high_process_fire_at_object_cannot_attack() {
        let mut s = scene();
        let (process, actor) = (s.process, s.actor);
        s.e.call(0x0090_37e0, &args![process, actor, 0x6000u32]);
        assert!(calls(&s.e, 0x0089_35f0).is_empty());
        assert!(slot_calls(&s.e, PROCESS_VT, 0x43c).is_empty());
    }

    #[test]
    fn test_high_process_fire_at_object_weapon() {
        // Three automatic shots, no ammunition restriction, animation group
        // 0x33 selects the attack 0x26; the queue accepts it.
        let s = fire(|s| {
            slot_ret(&mut s.e, PROCESS_VT, 0x438, 3);
            s.e.register(0x0049_5e40, |_, _| word(0x33));
        });
        assert_eq!(
            slot_calls(&s.e, PROCESS_VT, 0x43c),
            vec![vec![s.process, 3]]
        );
        assert_eq!(calls(&s.e, 0x0089_35f0), vec![vec![s.actor, 0x26]]);
        assert_eq!(
            slot_calls(&s.e, PROCESS_VT, 0x6e0),
            vec![vec![s.process, 1]]
        );

        // No shots to fire: nothing is queued.
        let s = fire(|_| {});
        assert!(calls(&s.e, 0x0089_35f0).is_empty());

        // The ammunition group asks for a count, and only 1 is allowed.
        let s = fire(|s| {
            slot_ret(&mut s.e, PROCESS_VT, 0x438, 3);
            s.e.register(0x005f_25d0, |_, _| word(1));
            s.e.register(0x0070_f490, |_, _| word(2));
        });
        assert!(calls(&s.e, 0x0089_35f0).is_empty());
        assert_eq!(
            calls(&s.e, 0x0070_f490).len(),
            1,
            "asked once, with the group 4"
        );
        let s = fire(|s| {
            slot_ret(&mut s.e, PROCESS_VT, 0x438, 3);
            s.e.register(0x005f_25d0, |_, _| word(1));
            s.e.register(0x0070_f490, |_, _| word(1));
        });
        assert_eq!(calls(&s.e, 0x0089_35f0).len(), 1);
        let s = fire(|s| {
            slot_ret(&mut s.e, PROCESS_VT, 0x438, 3);
            s.e.register(0x005f_25d0, |_, _| word(1));
            s.e.register(0x0070_f490, |_, _| word(0));
        });
        assert!(calls(&s.e, 0x0089_35f0).is_empty());
    }

    #[test]
    fn test_high_process_fire_at_object_attack_ids() {
        let attack = |setup: fn(&mut Scene)| {
            let s = fire(|s| {
                slot_ret(&mut s.e, PROCESS_VT, 0x438, 1);
                setup(s);
            });
            let log = calls(&s.e, 0x0089_35f0);
            assert_eq!(log.len(), 1);
            log[0][1]
        };
        // Grenade or thrown weapons.
        assert_eq!(attack(|s| slot_ret(&mut s.e, PROCESS_VT, 0x1a8, 1)), 0x72);
        assert_eq!(attack(|s| slot_ret(&mut s.e, PROCESS_VT, 0x1b4, 1)), 0x72);
        // Mines.
        assert_eq!(attack(|s| slot_ret(&mut s.e, PROCESS_VT, 0x1ac, 1)), 0x66);
        // The animation group table.
        for (group, id) in [
            (0x33u32, 0x26),
            (0x34, 0x2c),
            (0x35, 0x32),
            (0x36, 0x38),
            (0x37, 0x3e),
            (0x38, 0x44),
            (0x6c, 0x1a),
        ] {
            let s = fire(|s| {
                slot_ret(&mut s.e, PROCESS_VT, 0x438, 1);
                s.e.register_double(0x0049_5e40, move |_, _| word(group));
            });
            assert_eq!(calls(&s.e, 0x0089_35f0)[0][1], id, "group {group:#x}");
        }
        // Any other group: the weapon's own id, unless it is 0xff.
        assert_eq!(
            attack(|s| s.e.register(0x0051_f5f0, |_, _| word(0x31))),
            0x31
        );
        assert_eq!(
            attack(|s| s.e.register(0x0051_f5f0, |_, _| word(0xff))),
            0x20
        );
    }

    #[test]
    fn test_high_process_fire_at_object_without_weapon() {
        let queued = |action: i32| {
            let mut s = scene();
            slot_ret(&mut s.e, PROCESS_VT, 0x3f8, 1);
            s.e.register(0x0052_4b40, |_, _| word(0));
            s.e.register(0x0089_35f0, |_, _| word(1));
            slot_ret(&mut s.e, PROCESS_VT, 0x3e4, action as u32);
            let (process, actor) = (s.process, s.actor);
            s.e.call(0x0090_37e0, &args![process, actor, 0u32]);
            let log = calls(&s.e, 0x0089_35f0);
            // No weapon: the default attack 0x20.
            log.iter().map(|a| a[1]).collect::<Vec<_>>()
        };
        assert_eq!(queued(-1), vec![0x20]);
        assert_eq!(queued(4), vec![0x20]);
        assert!(queued(2).is_empty());
    }

    /// A package with the three event actions: idle, script and topic words.
    fn test_package(e: &mut Engine) -> u32 {
        let package = e.mem.alloc(0x90);
        for (base, ids) in [
            (PACKAGE_ON_BEGIN, [0x11u32, 0x12, 0x13]),
            (PACKAGE_ON_END, [0x31, 0x32, 0x33]),
            (PACKAGE_ON_CHANGE, [0x21, 0x22, 0x23]),
        ] {
            for (i, id) in ids.iter().enumerate() {
                e.mem.set_u32(package + base + 4 * i as u32, *id);
            }
        }
        package
    }

    #[test]
    fn test_high_process_start_new_package() {
        let mut s = scene();
        let (process, actor) = (s.process, s.actor);
        let package = test_package(&mut s.e);
        s.e.call(0x0090_3a80, &args![process, actor, package]);
        let log: Vec<u32> = s.e.call_log.as_ref().unwrap().iter().map(|c| c.0).collect();
        let position = |address: u32| log.iter().position(|a| *a == address).unwrap();
        // Script, then the idle, then the begin-idles flag, then the topic.
        assert_eq!(
            calls(&s.e, SCRIPT_RUN),
            vec![vec![0x12, actor, 0x9999, 0, 1]]
        );
        assert_eq!(
            slot_calls(&s.e, PROCESS_VT, 0x44),
            vec![vec![process, actor, 0x11, 2, 1, 0, 1]]
        );
        assert_eq!(slot_calls(&s.e, PROCESS_VT, 0x588), vec![vec![process, 1]]);
        assert_eq!(
            slot_calls(&s.e, PROCESS_VT, 0x2a4),
            vec![vec![process, actor, 0x13, 0, 0, 1, 1]]
        );
        assert!(position(SCRIPT_RUN) < position(vt_fn(PROCESS_VT, 0x44)));
        assert!(position(vt_fn(PROCESS_VT, 0x588)) < position(vt_fn(PROCESS_VT, 0x2a4)));
        assert_eq!(calls(&s.e, 0x0088_4f80), vec![vec![actor]]);
        assert_eq!(s.e.mem.u8(process + 0x110), 0);
    }

    #[test]
    fn test_high_process_start_new_package_without_idle() {
        // No idle: the pick-pack flag is set, and a special idle is started
        // only when the actor's idle has not finished.
        let mut s = scene();
        let (process, actor) = (s.process, s.actor);
        let package = test_package(&mut s.e);
        s.e.mem.set_u32(package + PACKAGE_ON_BEGIN, 0);
        s.e.mem.set_u32(package + PACKAGE_ON_BEGIN + 4, 0);
        s.e.mem.set_u32(package + PACKAGE_ON_BEGIN + 8, 0);
        slot_ret(&mut s.e, ACTOR_VT, 0x1e4, 0x8800);
        s.e.call(0x0090_3a80, &args![process, actor, package]);
        assert_eq!(s.e.mem.u8(process + 0x110), 1);
        assert_eq!(
            slot_calls(&s.e, PROCESS_VT, 0x44),
            vec![vec![process, actor, 0, 2, 0, 1, 1]]
        );
        assert_eq!(calls(&s.e, 0x0049_85f0), vec![vec![0x8800]]);
        assert!(calls(&s.e, SCRIPT_RUN).is_empty());
        assert!(slot_calls(&s.e, PROCESS_VT, 0x2a4).is_empty());

        // The idle has finished: no call.
        let mut s = scene();
        let (process, actor) = (s.process, s.actor);
        let package = test_package(&mut s.e);
        s.e.mem.set_u32(package + PACKAGE_ON_BEGIN, 0);
        slot_ret(&mut s.e, ACTOR_VT, 0x1e4, 0x8800);
        s.e.register(0x0049_85f0, |_, _| word(1));
        s.e.call(0x0090_3a80, &args![process, actor, package]);
        assert!(slot_calls(&s.e, PROCESS_VT, 0x44).is_empty());

        // No special idle at all.
        let mut s = scene();
        let (process, actor) = (s.process, s.actor);
        let package = test_package(&mut s.e);
        s.e.mem.set_u32(package + PACKAGE_ON_BEGIN, 0);
        s.e.call(0x0090_3a80, &args![process, actor, package]);
        assert!(calls(&s.e, 0x0049_85f0).is_empty());
    }

    #[test]
    fn test_high_process_start_new_package_null() {
        let mut s = scene();
        let (process, actor) = (s.process, s.actor);
        s.e.call(0x0090_3a80, &args![process, actor, 0u32]);
        assert_eq!(calls(&s.e, 0x0088_4f80), vec![vec![actor]]);
        assert!(calls(&s.e, SCRIPT_RUN).is_empty());
        s.e.call(0x0090_3a80, &args![process, 0u32, 0u32]);
        assert_eq!(calls(&s.e, 0x0088_4f80).len(), 1, "a null actor is skipped");
    }

    fn event_action_getter(address: u32, expected: u32) {
        let mut e = engine();
        let package = test_package(&mut e);
        assert_eq!(e.call(address, &args![package]).u32(), expected);
    }

    #[test]
    fn test_fn_00903bb0() {
        event_action_getter(0x0090_3bb0, 0x13);
    }

    #[test]
    fn test_fn_00903bd0() {
        event_action_getter(0x0090_3bd0, 0x12);
    }

    #[test]
    fn test_fn_00903cd0() {
        event_action_getter(0x0090_3cd0, 0x23);
    }

    #[test]
    fn test_fn_00903cf0() {
        event_action_getter(0x0090_3cf0, 0x22);
    }

    #[test]
    fn test_fn_00903de0() {
        event_action_getter(0x0090_3de0, 0x31);
    }

    #[test]
    fn test_fn_00903e00() {
        event_action_getter(0x0090_3e00, 0x33);
    }

    #[test]
    fn test_fn_00903e20() {
        event_action_getter(0x0090_3e20, 0x32);
    }

    #[test]
    fn test_high_process_change_package() {
        let mut s = scene();
        let (process, actor) = (s.process, s.actor);
        let package = test_package(&mut s.e);
        slot_ret(&mut s.e, PROCESS_VT, 0x22c, 0x4242);
        // The script is run only when one of the two checks passes.
        let result = s.e.call(0x0090_3bf0, &args![process, actor, package]).u32();
        assert_eq!(result, 0x4242);
        assert!(calls(&s.e, SCRIPT_RUN).is_empty());
        assert_eq!(calls(&s.e, 0x0067_1d10), vec![vec![0x22]]);
        assert_eq!(calls(&s.e, 0x0055_b980), vec![vec![0x22]]);
        assert_eq!(
            slot_calls(&s.e, PROCESS_VT, 0x44),
            vec![vec![process, actor, 0x21, 2, 1, 0, 1]]
        );
        assert_eq!(
            slot_calls(&s.e, PROCESS_VT, 0x2a4),
            vec![vec![process, actor, 0x23, 0, 0, 1, 1]]
        );

        for address in [0x0067_1d10u32, 0x0055_b980] {
            let mut s = scene();
            let (process, actor) = (s.process, s.actor);
            let package = test_package(&mut s.e);
            s.e.register(address, |_, _| word(1));
            s.e.call(0x0090_3bf0, &args![process, actor, package]);
            assert_eq!(
                calls(&s.e, SCRIPT_RUN),
                vec![vec![0x22, actor, 0x9999, 0, 1]]
            );
        }
    }

    #[test]
    fn test_high_process_change_package_null() {
        let mut s = scene();
        let (process, actor) = (s.process, s.actor);
        slot_ret(&mut s.e, PROCESS_VT, 0x22c, 9);
        assert_eq!(s.e.call(0x0090_3bf0, &args![process, actor, 0u32]).u32(), 9);
        assert!(slot_calls(&s.e, PROCESS_VT, 0x44).is_empty());
        assert!(slot_calls(&s.e, PROCESS_VT, 0x2a4).is_empty());
    }

    #[test]
    fn test_high_process_package_done() {
        let mut s = scene();
        let (process, actor) = (s.process, s.actor);
        let package = test_package(&mut s.e);
        s.e.register(0x0067_efd0, |_, _| word(1));
        s.e.call(0x0090_3d10, &args![process, actor, package]);
        assert_eq!(
            calls(&s.e, SCRIPT_RUN),
            vec![vec![0x32, actor, 0x9999, 0, 1]]
        );
        assert_eq!(
            slot_calls(&s.e, PROCESS_VT, 0x44),
            vec![vec![process, actor, 0x31, 2, 1, 0, 1]]
        );
        assert_eq!(slot_calls(&s.e, PROCESS_VT, 0x590), vec![vec![process, 1]]);
        assert_eq!(
            slot_calls(&s.e, PROCESS_VT, 0x2a4),
            vec![vec![process, actor, 0x33, 0, 0, 1, 1]]
        );

        // The end-idles flag is only set when 0067efd0 says so.
        let mut s = scene();
        let (process, actor) = (s.process, s.actor);
        let package = test_package(&mut s.e);
        s.e.call(0x0090_3d10, &args![process, actor, package]);
        assert!(slot_calls(&s.e, PROCESS_VT, 0x590).is_empty());

        // Nothing for a null package.
        let mut s = scene();
        let (process, actor) = (s.process, s.actor);
        s.e.call(0x0090_3d10, &args![process, actor, 0u32]);
        assert!(slot_calls(&s.e, PROCESS_VT, 0x44).is_empty());
        assert!(slot_calls(&s.e, PROCESS_VT, 0x2a4).is_empty());
    }

    #[test]
    fn test_high_process_set_actors_detection_event() {
        let mut s = scene();
        let process = s.process;
        let reference = s.e.mem.alloc(0x40);
        s.e.register(0x008f_db10, |_, a| word(a[0]));
        s.e.register(0x0043_5dd0, |_, _| float(12.5));
        s.e.register(0x0043_5de0, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            Ret::default()
        });
        // A previous event is freed first.
        let previous = s.e.mem.alloc(0x1c);
        s.e.mem.set_u32(process + 0x3dc, previous);
        s.e.call(
            0x0090_3e40,
            &args![process, 0xdeadu32, 1.0f32, 2.0f32, 3.0f32, 77u32, 0xbeefu32, reference],
        );
        assert_eq!(deleted(&s.e), vec![previous]);
        let event = s.e.mem.u32(process + 0x3dc);
        assert_ne!(event, 0);
        assert_eq!(s.e.mem.u32(event), 77);
        assert_eq!(s.e.mem.f32(event + 4), 1.0);
        assert_eq!(s.e.mem.f32(event + 8), 2.0);
        assert_eq!(s.e.mem.f32(event + 12), 3.0);
        assert_eq!(s.e.mem.f32(event + 0x10), 12.5);
        assert_eq!(s.e.mem.u32(event + 0x18), reference);
        assert_eq!(calls(&s.e, SET_TARGETED), vec![vec![reference, 1]]);
        assert_eq!(calls(&s.e, 0x008f_db10).len(), 1);

        // No previous event to free, and a null reference is not targeted.
        let mut s = scene();
        let process = s.process;
        s.e.register(0x008f_db10, |_, a| word(a[0]));
        s.e.register(0x0043_5dd0, |_, _| float(1.0));
        s.e.call(
            0x0090_3e40,
            &args![process, 0u32, 0.0f32, 0.0f32, 0.0f32, 5u32, 0u32, 0u32],
        );
        assert!(deleted(&s.e).is_empty());
        assert!(calls(&s.e, SET_TARGETED).is_empty());
        assert_eq!(s.e.mem.u32(s.e.mem.u32(process + 0x3dc)), 5);
    }

    #[test]
    fn test_high_process_remove_detection_event() {
        let mut s = scene();
        let process = s.process;
        s.e.call(0x0090_3f50, &args![process]);
        assert!(deleted(&s.e).is_empty());
        let event = s.e.mem.alloc(0x1c);
        s.e.mem.set_u32(process + 0x3dc, event);
        s.e.call(0x0090_3f50, &args![process]);
        assert_eq!(deleted(&s.e), vec![event]);
        assert_eq!(s.e.mem.u32(process + 0x3dc), 0);
    }

    #[test]
    fn test_high_process_check_for_expired_detection_event() {
        let run = |lifetime: f32, elapsed: f32, present: bool| {
            let mut s = scene();
            let process = s.process;
            let event = s.e.mem.alloc(0x1c);
            if present {
                s.e.mem.set_u32(process + 0x3dc, event);
            }
            put_setting(&mut s.e, DETECTION_EVENT_LIFETIME_SETTING, lifetime);
            s.e.register_double(0x0043_5e00, move |_, a| {
                assert_eq!(a[0], event + 0x10);
                float(elapsed)
            });
            s.e.call(0x0090_3f90, &args![process]);
            (
                deleted(&s.e) == vec![event],
                s.e.mem.u32(process + 0x3dc) == 0,
            )
        };
        assert_eq!(run(5.0, 6.0, true), (true, true));
        assert_eq!(run(5.0, 5.0, true), (false, false));
        assert_eq!(run(5.0, 4.0, true), (false, false));
        assert_eq!(run(5.0, 9.0, false), (false, true));
    }

    #[test]
    fn test_high_process_notify_guard_about_activation() {
        // The run-once package (+0xe4) is of kind 0x29 with owner 0x7000.
        let setup = |slot: u32, kind: u32, topic: u32| {
            let mut s = scene();
            let process = s.process;
            let activated = s.e.mem.alloc(0x40);
            vtable(&mut s.e, activated, OTHER_VT);
            default_slots(&mut s.e, OTHER_VT);
            slot_ret(&mut s.e, OTHER_VT, 0x100, 1);
            let package = s.e.mem.alloc(0x20);
            s.e.mem.set_u32(process + slot, package);
            s.e.mem.set_u32(process + slot + 8, topic);
            s.e.register_double(0x0096_11e0, move |_, a| {
                word(if a[0] == package { kind } else { 0 })
            });
            s.e.register(0x008b_0670, |_, _| word(1));
            (s, activated)
        };
        let (mut s, activated) = setup(0xe4, 0x29, 0x7000);
        let process = s.process;
        s.e.call(
            0x0090_4000,
            &args![process, 0x6001u32, activated, 0x7000u32],
        );
        assert_eq!(
            slot_calls(&s.e, PROCESS_VT, 0x33c),
            vec![vec![
                process, 0x6001, activated, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0
            ]]
        );
        assert_eq!(calls(&s.e, 0x008b_0670), vec![vec![0x6001, activated]]);

        // The current package (+4) counts too, after the run-once one.
        let (mut s, activated) = setup(4, 0x29, 0x7000);
        let process = s.process;
        s.e.call(
            0x0090_4000,
            &args![process, 0x6001u32, activated, 0x7000u32],
        );
        assert_eq!(slot_calls(&s.e, PROCESS_VT, 0x33c).len(), 1);

        // Wrong kind, wrong owner, the guard being the owner: nothing.
        let (mut s, activated) = setup(0xe4, 0x28, 0x7000);
        let process = s.process;
        s.e.call(
            0x0090_4000,
            &args![process, 0x6001u32, activated, 0x7000u32],
        );
        assert!(slot_calls(&s.e, PROCESS_VT, 0x33c).is_empty());
        let (mut s, activated) = setup(0xe4, 0x29, 0x7001);
        let process = s.process;
        s.e.call(
            0x0090_4000,
            &args![process, 0x6001u32, activated, 0x7000u32],
        );
        assert!(slot_calls(&s.e, PROCESS_VT, 0x33c).is_empty());
        let (mut s, activated) = setup(0xe4, 0x29, 0x7000);
        let process = s.process;
        s.e.call(
            0x0090_4000,
            &args![process, 0x7000u32, activated, 0x7000u32],
        );
        assert!(slot_calls(&s.e, PROCESS_VT, 0x33c).is_empty());
        // The activated reference refusing the query ends it at once.
        let (mut s, activated) = setup(0xe4, 0x29, 0x7000);
        let process = s.process;
        slot_ret(&mut s.e, OTHER_VT, 0x100, 0);
        s.e.call(
            0x0090_4000,
            &args![process, 0x6001u32, activated, 0x7000u32],
        );
        assert!(slot_calls(&s.e, PROCESS_VT, 0x33c).is_empty());
        assert!(calls(&s.e, 0x008b_0670).is_empty());
        // The combat call is refused by 008b0670.
        let (mut s, activated) = setup(0xe4, 0x29, 0x7000);
        let process = s.process;
        s.e.register(0x008b_0670, |_, _| word(0));
        s.e.call(
            0x0090_4000,
            &args![process, 0x6001u32, activated, 0x7000u32],
        );
        assert!(slot_calls(&s.e, PROCESS_VT, 0x33c).is_empty());
    }

    /// Avoid areas as the game keeps them: 0x34-byte records whose `pRefObj`
    /// (+0x30) is the given reference.
    fn avoid_area(e: &mut Engine, reference: u32) -> u32 {
        let area = e.mem.alloc(0x34);
        e.mem.set_u32(area + 0x30, reference);
        area
    }

    #[test]
    fn test_high_process_is_avoid_area_in_avoid_pathing_list() {
        let mut s = scene();
        let process = s.process;
        // No list at all.
        assert!(!s.e.call(0x0090_40f0, &args![process, 0x8001u32]).bool());
        let one = avoid_area(&mut s.e, 0x8001);
        let two = avoid_area(&mut s.e, 0x8002);
        let list = s.e.mem.alloc(8);
        fill_list(&mut s.e, list, &[one, two]);
        s.e.mem.set_u32(process + 0x44c, list);
        assert!(s.e.call(0x0090_40f0, &args![process, 0x8001u32]).bool());
        assert!(s.e.call(0x0090_40f0, &args![process, 0x8002u32]).bool());
        assert!(!s.e.call(0x0090_40f0, &args![process, 0x8003u32]).bool());
    }

    #[test]
    fn test_high_process_clear_avoid_areas() {
        let mut s = scene();
        let process = s.process;
        s.e.call(0x0090_4160, &args![process]);
        assert!(calls(&s.e, LIST_CLEAR).is_empty());
        let one = avoid_area(&mut s.e, 0x8001);
        let two = avoid_area(&mut s.e, 0x8002);
        let list = s.e.mem.alloc(8);
        fill_list(&mut s.e, list, &[one, two]);
        s.e.mem.set_u32(process + 0x44c, list);
        s.e.call(0x0090_4160, &args![process]);
        assert_eq!(deleted(&s.e), vec![one, two]);
        assert_eq!(calls(&s.e, LIST_CLEAR), vec![vec![list]]);
        assert_eq!(calls(&s.e, LIST_DELETE), vec![vec![list, 1]]);
        assert_eq!(s.e.mem.u32(process + 0x44c), 0);
    }

    #[test]
    fn test_high_process_remove_avoid_pathing_node() {
        // Removing the second entry unlinks its node and frees the entry.
        let mut s = scene();
        let process = s.process;
        let one = avoid_area(&mut s.e, 0x8001);
        let two = avoid_area(&mut s.e, 0x8002);
        let list = s.e.mem.alloc(8);
        fill_list(&mut s.e, list, &[one, two]);
        let second_node = s.e.mem.u32(list + 4);
        s.e.mem.set_u32(process + 0x44c, list);
        s.e.call(0x0090_4220, &args![process, 0x8002u32]);
        assert_eq!(deleted(&s.e), vec![two]);
        assert_eq!(s.e.mem.u32(list + 4), 0);
        assert_eq!(s.e.mem.u32(list), one);
        assert_eq!(calls(&s.e, LIST_DELETE), vec![vec![second_node, 1]]);

        // Removing the first entry moves the second into the head.
        let mut s = scene();
        let process = s.process;
        let one = avoid_area(&mut s.e, 0x8001);
        let two = avoid_area(&mut s.e, 0x8002);
        let list = s.e.mem.alloc(8);
        fill_list(&mut s.e, list, &[one, two]);
        s.e.mem.set_u32(process + 0x44c, list);
        s.e.call(0x0090_4220, &args![process, 0x8001u32]);
        assert_eq!(deleted(&s.e), vec![one]);
        assert_eq!(s.e.mem.u32(list), two);
        assert_eq!(s.e.mem.u32(list + 4), 0);

        // Nothing matches, or there is no list.
        let mut s = scene();
        let process = s.process;
        s.e.call(0x0090_4220, &args![process, 0x8001u32]);
        let one = avoid_area(&mut s.e, 0x8001);
        let list = s.e.mem.alloc(8);
        fill_list(&mut s.e, list, &[one]);
        s.e.mem.set_u32(process + 0x44c, list);
        s.e.call(0x0090_4220, &args![process, 0x8009u32]);
        assert!(deleted(&s.e).is_empty());
    }

    /// Runs `AddAvoidPathingArea` with the doubles for the node builder
    /// (which answers a block of nine words 1..=9) and the area constructor.
    fn add_avoid_area(s: &mut Scene, highest: f32, time_expire: f32, pathing: bool) -> (u32, u32) {
        let (process, actor) = (s.process, s.actor);
        let reference = s.e.mem.alloc(0x40);
        let marker = s.e.mem.alloc(0x40);
        s.e.register(0x008f_db90, |_, a| word(a[0]));
        s.e.register(LIST_CONSTRUCTOR, |_, a| word(a[0]));
        let built = s.e.mem.alloc(0x24);
        for i in 0..9 {
            s.e.mem.set_u32(built + 4 * i, i + 1);
        }
        s.e.register_double(0x0069_15d0, move |_, _| word(built));
        s.e.register_double(0x008b_3bd0, move |_, _| word(pathing as u32));
        s.e.mem.set_f32(process + 0x440, highest);
        s.e.call(
            0x0090_42a0,
            &args![
                process,
                actor,
                10.0f32,
                20.0f32,
                30.0f32,
                4.5f32,
                time_expire,
                0.75f32,
                reference,
                marker
            ],
        );
        (reference, marker)
    }

    #[test]
    fn test_high_process_add_avoid_pathing_area() {
        let mut s = scene();
        let (process, actor) = (s.process, s.actor);
        // The list is created, the area is added through 005ae3d0, and the
        // actor is asked to avoid because the process's highest radiation is
        // not positive.
        let added = Rc::new(RefCell::new(vec![]));
        let log = added.clone();
        s.e.register_double(0x005a_e3d0, move |e, a| {
            log.borrow_mut().push((a[0], e.mem.u32(a[1])));
            Ret::default()
        });
        let (reference, marker) = add_avoid_area(&mut s, 0.0, 12.0, false);
        let list = s.e.mem.u32(process + 0x44c);
        assert_ne!(list, 0);
        assert_eq!(calls(&s.e, LIST_CONSTRUCTOR).len(), 1);
        let added = added.borrow();
        assert_eq!(added.len(), 1);
        assert_eq!(added[0].0, list);
        let area = added[0].1;
        // The node words come from the builder; the tail fields are set.
        for i in 0..9 {
            assert_eq!(s.e.mem.u32(area + 4 * i), i + 1);
        }
        assert_eq!(s.e.mem.f32(area + 0x24), 12.0);
        assert_eq!(s.e.mem.u32(area + 0x28), marker);
        assert_eq!(s.e.mem.f32(area + 0x2c), 0.75);
        assert_eq!(s.e.mem.u32(area + 0x30), reference);
        // The builder got the point (as a block), radius and level.
        let build = &calls(&s.e, 0x0069_15d0)[0];
        assert_eq!(build[2..], [4.5f32.to_bits(), 0.75f32.to_bits()]);
        assert_eq!(calls(&s.e, SET_TARGETED), vec![vec![reference, 1]]);
        assert_eq!(
            calls(&s.e, 0x0089_82c0),
            vec![vec![actor, reference, 12.0f32.to_bits()]]
        );
    }

    #[test]
    fn test_high_process_add_avoid_pathing_area_avoid_package_delay() {
        // The delay is the expiry time, except for f32::MAX or a pathing actor.
        for (expire, pathing, delay) in [
            (12.0f32, false, 12.0f32),
            (f32::MAX, false, 0.0),
            (12.0, true, 0.0),
        ] {
            let mut s = scene();
            let (reference, _) = add_avoid_area(&mut s, -1.0, expire, pathing);
            assert_eq!(
                calls(&s.e, 0x0089_82c0),
                vec![vec![s.actor, reference, delay.to_bits()]]
            );
        }
        // A positive highest radiation: no avoid package; and a list that
        // already exists is reused.
        let mut s = scene();
        let list = s.e.mem.alloc(8);
        s.e.mem.set_u32(s.process + 0x44c, list);
        add_avoid_area(&mut s, 1.0, 12.0, false);
        assert!(calls(&s.e, 0x0089_82c0).is_empty());
        assert!(calls(&s.e, LIST_CONSTRUCTOR).is_empty());
        assert_eq!(calls(&s.e, 0x005a_e3d0)[0][0], list);
    }

    #[test]
    fn test_fn_00904430() {
        let mut s = scene();
        s.e.mem.set_f32(s.process + 0x440, -2.5);
        assert_eq!(s.e.call(0x0090_4430, &args![s.process]).f32(), -2.5);
    }

    #[test]
    fn test_high_process_create_avoid_array() {
        let mut s = scene();
        let (process, actor) = (s.process, s.actor);
        s.e.register(ARRAY_ALLOCATE, |e, a| {
            assert_eq!(a[0], 0x18);
            word(e.mem.alloc(0x18))
        });
        s.e.register_double(0x006e_3850, move |_, a| word(a[0]));
        // The array's buffer: room for one element, as 00905a10 would make.
        s.e.register(0x0090_5a10, |e, a| {
            let buffer = e.mem.alloc(0x48);
            e.mem.set_u32(a[0] + 4, buffer);
            word(0)
        });
        // Not avoiding radiation, or no list: null.
        assert_eq!(s.e.call(0x0090_4450, &args![process, actor]).u32(), 0);
        s.e.register(0x008b_cc80, |_, _| word(1));
        assert_eq!(s.e.call(0x0090_4450, &args![process, actor]).u32(), 0);

        // With a list holding an area, the area is copied into the array.
        let area = s.e.mem.alloc(0x34);
        for i in 0..9 {
            s.e.mem.set_u32(area + 4 * i, 0x100 + i);
        }
        let list = s.e.mem.alloc(8);
        fill_list(&mut s.e, list, &[area]);
        s.e.mem.set_u32(process + 0x44c, list);
        let result = s.e.call(0x0090_4450, &args![process, actor]).u32();
        assert_ne!(result, 0);
        let buffer = s.e.mem.u32(result + 4);
        for i in 0..9 {
            assert_eq!(s.e.mem.u32(buffer + 4 * i), 0x100 + i);
        }

        // An empty list gives null.
        s.e.mem.set_u32(list, 0);
        assert_eq!(s.e.call(0x0090_4450, &args![process, actor]).u32(), 0);
    }

    /// `ProcessAvoidArea` with the target and positions wired up.
    fn avoid_scene() -> Scene {
        let mut s = scene();
        let positions = s.e.mem.alloc(0x20);
        slot_ret(&mut s.e, ACTOR_VT, 0x1f4, positions);
        let target = s.e.mem.alloc(0x40);
        vtable(&mut s.e, target, OTHER_VT);
        default_slots(&mut s.e, OTHER_VT);
        slot_ret(&mut s.e, OTHER_VT, 0x1f4, positions);
        s.e.mem.set_u32(s.process + 0x40, target);
        s
    }

    #[test]
    fn test_high_process_process_avoid_area_wait() {
        // A target is set and the actor is pathing again: the timer decides.
        let mut s = avoid_scene();
        let (process, actor) = (s.process, s.actor);
        s.e.register(0x008b_3bb0, |_, _| word(1));
        s.e.mem.set_f32(process + 0x450, 0.0);
        s.e.call(0x0090_4540, &args![process, actor]);
        assert_eq!(
            slot_calls(&s.e, PROCESS_VT, 0x288),
            vec![vec![process, actor, 1]]
        );

        // Not yet: the timer runs down by the frame time.
        let mut s = avoid_scene();
        let (process, actor) = (s.process, s.actor);
        s.e.register(0x008b_3bb0, |_, _| word(1));
        s.e.mem.set_f32(process + 0x450, 2.0);
        s.e.mem.set_f32(FRAME_CELL, 0.5);
        s.e.call(0x0090_4540, &args![process, actor]);
        assert_eq!(s.e.mem.f32(process + 0x450), 1.5);
        assert!(slot_calls(&s.e, PROCESS_VT, 0x288).is_empty());

        // The actor is not at its destination: the timer runs down even at zero.
        let mut s = avoid_scene();
        let (process, actor) = (s.process, s.actor);
        s.e.mem.set_f32(process + 0x450, 0.0);
        s.e.mem.set_f32(FRAME_CELL, 0.25);
        s.e.call(0x0090_4540, &args![process, actor]);
        assert_eq!(s.e.mem.f32(process + 0x450), -0.25);

        // Without a target the process picks one (vtable +0x8c); if it still
        // has none, the same wait applies.
        let mut s = scene();
        let (process, actor) = (s.process, s.actor);
        s.e.mem.set_f32(process + 0x450, 0.0);
        s.e.call(0x0090_4540, &args![process, actor]);
        assert_eq!(
            slot_calls(&s.e, PROCESS_VT, 0x8c),
            vec![vec![process, actor]]
        );
        assert_eq!(
            slot_calls(&s.e, PROCESS_VT, 0x288),
            vec![vec![process, actor, 1]]
        );
        s.e.mem.set_f32(process + 0x450, 3.0);
        s.e.mem.set_f32(FRAME_CELL, 1.0);
        s.e.call(0x0090_4540, &args![process, actor]);
        assert_eq!(s.e.mem.f32(process + 0x450), 2.0);
        assert_eq!(slot_calls(&s.e, PROCESS_VT, 0x288).len(), 1);
    }

    #[test]
    fn test_high_process_process_avoid_area_request() {
        // The process finds a target when asked, and builds the flee request.
        let mut s = scene();
        let (process, actor) = (s.process, s.actor);
        let positions = s.e.mem.alloc(0x20);
        slot_ret(&mut s.e, ACTOR_VT, 0x1f4, positions);
        let target = s.e.mem.alloc(0x40);
        vtable(&mut s.e, target, OTHER_VT);
        default_slots(&mut s.e, OTHER_VT);
        slot_ret(&mut s.e, OTHER_VT, 0x1f4, positions);
        on_slot(&mut s.e, PROCESS_VT, 0x8c, move |e, a| {
            e.mem.set_u32(a[0] + 0x40, target);
            Ret::default()
        });
        put_setting(&mut s.e, AVOID_RADIATION_SETTING, 7.0);
        on_slot(&mut s.e, ACTOR_VT, 0x2bc, |_, _| float(2.5));
        slot_ret(&mut s.e, ACTOR_VT, 0x21c, 1);
        slot_ret(&mut s.e, PROCESS_VT, 0x254, 0x5a5a);
        // 004181e0 answers an object whose +0x30 sub-object has a vtable.
        let base = s.e.mem.alloc(0x40);
        vtable(&mut s.e, base + 0x30, VALUE_VT);
        on_slot(&mut s.e, VALUE_VT, 0x28, |_, _| word(0x01));
        s.e.register_double(0x0041_81e0, move |_, _| word(base));
        s.e.call(0x0090_4540, &args![process, actor]);
        let request = calls(&s.e, 0x006e_5390)[0][0];
        assert_eq!(calls(&s.e, 0x006e_29f0), vec![vec![request, actor]]);
        assert_eq!(
            calls(&s.e, 0x006e_2bb0),
            vec![vec![request, 2.5f32.to_bits()]]
        );
        assert_eq!(calls(&s.e, 0x006e_2b50), vec![vec![request, 1]]);
        assert_eq!(
            calls(&s.e, 0x006d_3b00),
            vec![vec![request, 20.0f32.to_bits()]]
        );
        assert_eq!(calls(&s.e, 0x006d_61e0), vec![vec![request, 0x5a5a]]);
        assert_eq!(
            calls(&s.e, 0x0050_7610),
            vec![vec![request, 300.0f32.to_bits()]]
        );
        assert_eq!(calls(&s.e, 0x008d_f000), vec![vec![request]]);
        assert_eq!(calls(&s.e, 0x008b_b630), vec![vec![actor, request]]);
        assert_eq!(calls(&s.e, 0x006d_ad70), vec![vec![request]]);
        assert_eq!(
            slot_calls(&s.e, PROCESS_VT, 0x254),
            vec![vec![process, actor]]
        );
        assert!(slot_calls(&s.e, PROCESS_VT, 0x288).is_empty());

        // Without the actor's run flag the flag call is skipped.
        let mut s = avoid_scene_with_request();
        let (process, actor) = (s.process, s.actor);
        s.e.call(0x0090_4540, &args![process, actor]);
        assert!(calls(&s.e, 0x006e_2b50).is_empty());
    }

    /// Like `avoid_scene` but the process builds the request (no target at
    /// first).
    fn avoid_scene_with_request() -> Scene {
        let mut s = scene();
        let positions = s.e.mem.alloc(0x20);
        slot_ret(&mut s.e, ACTOR_VT, 0x1f4, positions);
        let target = s.e.mem.alloc(0x40);
        vtable(&mut s.e, target, OTHER_VT);
        default_slots(&mut s.e, OTHER_VT);
        slot_ret(&mut s.e, OTHER_VT, 0x1f4, positions);
        on_slot(&mut s.e, PROCESS_VT, 0x8c, move |e, a| {
            e.mem.set_u32(a[0] + 0x40, target);
            Ret::default()
        });
        put_setting(&mut s.e, AVOID_RADIATION_SETTING, 7.0);
        s
    }

    /// The idle-conversation setup: a running package of kind 3 (the
    /// default, which allows talking), settings that make a close
    /// neighbour a certain pick, and one detected actor.
    struct Talk {
        s: Scene,
        candidate: u32,
        candidate_process: u32,
    }

    fn talk() -> Talk {
        let mut s = scene();
        let process = s.process;
        slot_ret(&mut s.e, PROCESS_VT, 0x27c, 0x7700);
        s.e.register(0x0041_ca90, |_, _| word(3));
        // The detected actor: a state of detection level 3.
        let candidate = s.e.mem.alloc(0x40);
        let state = s.e.mem.alloc(0x24);
        s.e.mem.set_u32(state, candidate);
        s.e.mem.set_u32(state + 4, 3);
        let list = s.e.mem.alloc(8);
        fill_list(&mut s.e, list, &[state]);
        s.e.mem.set_u32(process + 0x25c, list);
        let spoke_to = s.e.mem.alloc(8);
        s.e.mem.set_u32(process + 0x264, spoke_to);
        let candidate_process = s.e.mem.alloc(0x40);
        vtable(&mut s.e, candidate_process, OTHER_VT);
        default_slots(&mut s.e, OTHER_VT);
        slot_ret(&mut s.e, OTHER_VT, 0x1f4, 1);
        slot_ret(&mut s.e, PROCESS_VT, 0x1f4, 1);
        s.e.register_double(REFERENCE_PROCESS, move |_, a| {
            word(if a[0] == candidate {
                candidate_process
            } else {
                0
            })
        });
        put_setting(&mut s.e, IDLE_TALK_DISTANCE_SETTING, 150.0);
        put_setting(&mut s.e, IDLE_TALK_DISTANCE_FOLLOWER_SETTING, 400.0);
        put_setting(&mut s.e, TALKED_TO_CLEAR_SETTING, 30.0);
        put_setting(&mut s.e, TALK_CHANCE_SETTING, 50.0);
        put_setting(&mut s.e, TALK_CHANCE_FOLLOWER_SETTING, 90.0);
        put_setting(&mut s.e, CHECK_TO_TALK_FIRST_SETTING, 10.0);
        put_setting(&mut s.e, CHECK_TO_TALK_SECOND_SETTING, 20.0);
        // Distance 100, random roll 7 (7 % 100).
        s.e.register(0x0057_23b0, |_, _| float(100.0));
        s.e.register(0x0048_7f50, |_, _| word(207));
        s.e.register(0x0064_65f0, |_, a| {
            let (first, second) = (f32::from_bits(a[0]), f32::from_bits(a[1]));
            float(first + second)
        });
        Talk {
            s,
            candidate,
            candidate_process,
        }
    }

    #[test]
    fn test_high_process_check_if_there_someone_talk_with_detected() {
        let mut t = talk();
        let (process, actor) = (t.s.process, t.s.actor);
        assert!(t.s.e.call(0x0090_4800, &args![process, actor]).bool());
        // Both processes are told, the check timer is a random value.
        assert_eq!(
            slot_calls(&t.s.e, PROCESS_VT, 0x1f0),
            vec![vec![process, t.candidate]]
        );
        assert_eq!(
            slot_calls(&t.s.e, OTHER_VT, 0x1f0),
            vec![vec![t.candidate_process, actor]]
        );
        assert_eq!(
            slot_calls(&t.s.e, ACTOR_VT, 0x280),
            vec![vec![actor, t.candidate, 0, 0, 1, 0, 0, 0, 0, 0]]
        );
        // 006465f0 gets the settings 011cd328 and 011cd038 in that order.
        assert_eq!(
            calls(&t.s.e, 0x0064_65f0),
            vec![vec![10.0f32.to_bits(), 20.0f32.to_bits()]]
        );
        assert_eq!(t.s.e.mem.f32(process + 0x2c8), 30.0);
        assert_eq!(t.s.e.mem.u8(process + 0x2c6), 0);
        // The last-spoke-to list was empty, so its timer was set.
        assert_eq!(t.s.e.mem.f32(process + 0x2a0), 30.0);
    }

    #[test]
    fn test_high_process_check_if_there_someone_talk_with_rejections() {
        // Each change makes the pick fail.
        type Change = fn(&mut Talk);
        let changes: Vec<(&str, Change)> = vec![
            ("level 2", |t| {
                let state = t.s.e.mem.u32(t.s.e.mem.u32(t.s.process + 0x25c));
                t.s.e.mem.set_u32(state + 4, 2);
            }),
            ("dead", |t| t.s.e.register(0x0043_7b90, |_, _| word(1))),
            ("too far", |t| {
                t.s.e.register(0x0057_23b0, |_, _| float(151.0))
            }),
            ("chance", |t| t.s.e.register(0x0048_7f50, |_, _| word(50))),
            ("not allowed by the actor", |t| {
                slot_ret(&mut t.s.e, OTHER_VT, 0x1f4, 0)
            }),
            ("not allowed by the process", |t| {
                slot_ret(&mut t.s.e, PROCESS_VT, 0x1f4, 0)
            }),
            ("already spoken to", |t| {
                t.s.e.register(0x005f_65d0, |_, _| word(1))
            }),
            ("busy package", |t| {
                t.s.e.register(0x0093_44a0, |_, _| word(0x9000));
                t.s.e.register(0x0096_11e0, |_, _| word(0x25));
            }),
            ("timer running", |t| {
                let process = t.s.process;
                t.s.e.mem.set_f32(process + 0x2c8, 1.0);
            }),
            ("no package", |t| slot_ret(&mut t.s.e, PROCESS_VT, 0x27c, 0)),
            ("interrupt package", |t| {
                t.s.e.register(0x0067_8610, |_, _| word(1))
            }),
            ("skip behavior", |t| {
                t.s.e.register(0x008a_78f0, |_, _| word(1))
            }),
            ("kind 4", |t| t.s.e.register(0x0041_ca90, |_, _| word(4))),
            ("kind 0xc refused", |t| {
                t.s.e.register(0x0041_ca90, |_, _| word(0xc))
            }),
        ];
        for (name, change) in changes {
            let mut t = talk();
            change(&mut t);
            let (process, actor) = (t.s.process, t.s.actor);
            assert!(
                !t.s.e.call(0x0090_4800, &args![process, actor]).bool(),
                "{name}"
            );
            assert!(slot_calls(&t.s.e, PROCESS_VT, 0x1f0).is_empty(), "{name}");
            assert!(slot_calls(&t.s.e, ACTOR_VT, 0x280).is_empty(), "{name}");
        }
    }

    #[test]
    fn test_high_process_check_if_there_someone_talk_with_variants() {
        // A follower talks at the follower distance and chance.
        let mut t = talk();
        let (process, actor) = (t.s.process, t.s.actor);
        t.s.e.register(0x0042_5fd0, |_, _| word(1));
        t.s.e.register(0x0057_23b0, |_, _| float(150.0));
        t.s.e.register(0x0048_7f50, |_, _| word(89));
        assert!(t.s.e.call(0x0090_4800, &args![process, actor]).bool());

        // Kind 0xc allowed by 00901c50.
        let mut t = talk();
        let (process, actor) = (t.s.process, t.s.actor);
        t.s.e.register(0x0041_ca90, |_, _| word(0xc));
        t.s.e.register(0x0090_1c50, |_, _| word(1));
        assert!(t.s.e.call(0x0090_4800, &args![process, actor]).bool());

        // Kind 1 talks to the process's target, flagged.
        let mut t = talk();
        let (process, actor) = (t.s.process, t.s.actor);
        t.s.e.register(0x0041_ca90, |_, _| word(1));
        t.s.e.mem.set_u32(process + 0x40, 0x5555);
        assert!(t.s.e.call(0x0090_4800, &args![process, actor]).bool());
        assert_eq!(
            slot_calls(&t.s.e, ACTOR_VT, 0x280),
            vec![vec![actor, 0x5555, 0, 0, 1, 1, 0, 0, 1, 0]]
        );
        // No search happened.
        assert!(calls(&t.s.e, 0x0057_23b0).is_empty());

        // The player as the target is not talked to.
        let mut t = talk();
        let (process, actor, player) = (t.s.process, t.s.actor, t.s.player);
        t.s.e.register(0x0041_ca90, |_, _| word(1));
        t.s.e.mem.set_u32(process + 0x40, player);
        assert!(!t.s.e.call(0x0090_4800, &args![process, actor]).bool());

        // A busy sandbox partner (kind 0x25 refused by 00901c50) is skipped.
        let mut t = talk();
        let (process, actor) = (t.s.process, t.s.actor);
        t.s.e.register(0x0093_44a0, |_, _| word(0x9000));
        t.s.e.register(0x0096_11e0, |_, _| word(0x25));
        t.s.e.register(0x0090_1c50, |_, _| word(1));
        assert!(t.s.e.call(0x0090_4800, &args![process, actor]).bool());
    }

    #[test]
    fn test_high_process_check_if_there_someone_talk_with_timers() {
        // A non-empty last-spoke-to list runs its timer down; reaching zero
        // clears it. The list holds an actor other than the candidate.
        let mut t = talk();
        let (process, actor) = (t.s.process, t.s.actor);
        let spoke = t.s.e.mem.alloc(8);
        t.s.e.mem.set_u32(spoke, 0x4444);
        t.s.e.mem.set_u32(process + 0x264, spoke);
        t.s.e.mem.set_f32(process + 0x2a0, 3.0);
        t.s.e.mem.set_f32(FRAME_CELL, 1.0);
        assert!(t.s.e.call(0x0090_4800, &args![process, actor]).bool());
        assert_eq!(t.s.e.mem.f32(process + 0x2a0), 2.0);
        assert!(calls(&t.s.e, LIST_CLEAR).is_empty());
        // The membership question was asked about a block holding the candidate.
        assert_eq!(calls(&t.s.e, 0x005f_65d0)[0][0], spoke);

        let mut t = talk();
        let (process, actor) = (t.s.process, t.s.actor);
        let spoke = t.s.e.mem.alloc(8);
        t.s.e.mem.set_u32(spoke, 0x4444);
        t.s.e.mem.set_u32(process + 0x264, spoke);
        t.s.e.mem.set_f32(process + 0x2a0, 0.5);
        t.s.e.mem.set_f32(FRAME_CELL, 1.0);
        t.s.e.call(0x0090_4800, &args![process, actor]);
        assert_eq!(calls(&t.s.e, LIST_CLEAR), vec![vec![spoke]]);
    }

    #[test]
    fn test_high_process_compute_allow_sandbox_conversation() {
        let run = |setup: fn(&mut Talk)| {
            let mut t = talk();
            setup(&mut t);
            let (process, actor, candidate) = (t.s.process, t.s.actor, t.candidate);
            (
                t.s.e
                    .call(0x0090_4c80, &args![process, actor, candidate])
                    .bool(),
                t,
            )
        };
        let (allowed, t) = run(|_| {});
        assert!(allowed);
        assert_eq!(
            slot_calls(&t.s.e, PROCESS_VT, 0x1f4),
            vec![vec![t.s.process, t.s.actor, t.candidate]]
        );
        assert_eq!(t.s.e.mem.f32(t.s.process + 0x2a0), 30.0);
        // Refusals.
        assert!(!run(|t| t.s.e.register(0x0043_7b90, |_, _| word(1))).0);
        assert!(!run(|t| slot_ret(&mut t.s.e, PROCESS_VT, 0x1f4, 0)).0);
        assert!(!run(|t| t.s.e.register(0x005f_65d0, |_, _| word(1))).0);
        assert!(
            !run(|t| {
                t.s.e.register(0x0093_44a0, |_, _| word(0x9000));
                t.s.e.register(0x0096_11e0, |_, _| word(0x25));
            })
            .0
        );
        // A partner whose package is of kind 0x25 but passes 00901c50 is fine.
        assert!(
            run(|t| {
                t.s.e.register(0x0093_44a0, |_, _| word(0x9000));
                t.s.e.register(0x0096_11e0, |_, _| word(0x25));
                t.s.e.register(0x0090_1c50, |_, _| word(1));
            })
            .0
        );
        // Null target, the actor itself, the player.
        let mut t = talk();
        let (process, actor, player) = (t.s.process, t.s.actor, t.s.player);
        assert!(!t.s.e.call(0x0090_4c80, &args![process, actor, 0u32]).bool());
        assert!(!t
            .s
            .e
            .call(0x0090_4c80, &args![process, actor, actor])
            .bool());
        assert!(!t
            .s
            .e
            .call(0x0090_4c80, &args![process, actor, player])
            .bool());
        assert!(slot_calls(&t.s.e, PROCESS_VT, 0x1f4).is_empty());
        // A running last-spoke-to timer runs down and clears at zero.
        let mut t = talk();
        let (process, actor, candidate) = (t.s.process, t.s.actor, t.candidate);
        let spoke = t.s.e.mem.alloc(8);
        t.s.e.mem.set_u32(spoke, 0x4444);
        t.s.e.mem.set_u32(process + 0x264, spoke);
        t.s.e.mem.set_f32(process + 0x2a0, 0.5);
        t.s.e.mem.set_f32(FRAME_CELL, 1.0);
        assert!(t
            .s
            .e
            .call(0x0090_4c80, &args![process, actor, candidate])
            .bool());
        assert_eq!(t.s.e.mem.f32(process + 0x2a0), -0.5);
        assert_eq!(calls(&t.s.e, LIST_CLEAR), vec![vec![spoke]]);
    }

    /// A victim target that answers the vtable +0x100 query with true, set
    /// as the process's target.
    fn victim_scene() -> (Scene, u32, u32, u32) {
        let mut s = scene();
        let victim = s.e.mem.alloc(0x40);
        vtable(&mut s.e, victim, OTHER_VT);
        default_slots(&mut s.e, OTHER_VT);
        slot_ret(&mut s.e, OTHER_VT, 0x100, 1);
        s.e.mem.set_u32(s.process + 0x40, victim);
        let victim_process = s.e.mem.alloc(0x40);
        vtable(&mut s.e, victim_process, VALUE_VT);
        default_slots(&mut s.e, VALUE_VT);
        let actor_process = s.e.mem.alloc(0x40);
        vtable(&mut s.e, actor_process, 0x7500_0000);
        default_slots(&mut s.e, 0x7500_0000);
        let (actor, player) = (s.actor, s.player);
        s.e.register_double(REFERENCE_PROCESS, move |_, a| {
            word(if a[0] == victim {
                victim_process
            } else if a[0] == actor || a[0] == player {
                actor_process
            } else {
                0
            })
        });
        (s, victim, victim_process, actor_process)
    }

    #[test]
    fn test_high_process_process_cannibal_start() {
        // The action is not complete: the eating idle is set up.
        let (mut s, victim, _, _) = victim_scene();
        let (process, actor) = (s.process, s.actor);
        slot_ret(&mut s.e, PROCESS_VT, 0x454, 1);
        s.e.call(0x0090_4da0, &args![process, actor]);
        assert_eq!(slot_calls(&s.e, PROCESS_VT, 0x450), vec![vec![process, 0]]);
        assert_eq!(
            slot_calls(&s.e, PROCESS_VT, 0x44),
            vec![vec![process, actor, 0, 2, 1, 1, 1]]
        );
        assert_eq!(calls(&s.e, 0x008a_80c0), vec![vec![victim, 1]]);
        assert_eq!(slot_calls(&s.e, PROCESS_VT, 0x118), vec![vec![process, 1]]);
        assert!(slot_calls(&s.e, PROCESS_VT, 0x288).is_empty());
    }

    #[test]
    fn test_high_process_process_cannibal_finish() {
        // The action is complete and the idle has finished.
        let (mut s, victim, victim_process, actor_process) = victim_scene();
        let (process, actor, player) = (s.process, s.actor, s.player);
        slot_ret(&mut s.e, PROCESS_VT, 0x11c, 1);
        slot_ret(&mut s.e, ACTOR_VT, 0x1e4, 0x8800);
        s.e.register(0x0049_85f0, |_, a| {
            assert_eq!(a[0], 0x8800);
            word(1)
        });
        s.e.call(0x0090_4da0, &args![process, actor]);
        assert_eq!(
            slot_calls(&s.e, PROCESS_VT, 0x288),
            vec![vec![process, actor, 1]]
        );
        assert_eq!(calls(&s.e, 0x008c_0460), vec![vec![victim, actor, 0, 1]]);
        assert_eq!(
            slot_calls(&s.e, VALUE_VT, 0x210),
            vec![vec![victim_process, 0, victim]]
        );
        assert_eq!(
            slot_calls(&s.e, 0x7500_0000, 0x210),
            vec![vec![actor_process, 0, player]]
        );
        assert_eq!(calls(&s.e, 0x005c_c7a0), vec![vec![player, 0]]);
        assert!(slot_calls(&s.e, PROCESS_VT, 0x44).is_empty());

        // The idle has not finished: nothing else happens.
        let (mut s, _, _, _) = victim_scene();
        let (process, actor) = (s.process, s.actor);
        slot_ret(&mut s.e, PROCESS_VT, 0x11c, 1);
        s.e.call(0x0090_4da0, &args![process, actor]);
        assert!(slot_calls(&s.e, PROCESS_VT, 0x288).is_empty());
        assert!(calls(&s.e, 0x005c_c7a0).is_empty());
    }

    #[test]
    fn test_high_process_process_cannibal_target() {
        // Without a target the process finds one (vtable +0x8c); a target
        // that refuses the vtable +0x100 query is no victim.
        let mut s = scene();
        let (process, actor) = (s.process, s.actor);
        let target = s.e.mem.alloc(0x40);
        vtable(&mut s.e, target, OTHER_VT);
        default_slots(&mut s.e, OTHER_VT);
        on_slot(&mut s.e, PROCESS_VT, 0x8c, move |e, a| {
            e.mem.set_u32(a[0] + 0x40, target);
            Ret::default()
        });
        s.e.call(0x0090_4da0, &args![process, actor]);
        assert_eq!(
            slot_calls(&s.e, PROCESS_VT, 0x8c),
            vec![vec![process, actor]]
        );
        assert!(calls(&s.e, 0x008a_80c0).is_empty());
        assert_eq!(slot_calls(&s.e, PROCESS_VT, 0x118), vec![vec![process, 1]]);
    }

    #[test]
    fn test_high_process_process_sandman_walk_closer() {
        // Far away: a non-player actor walks to the position through the package.
        let mut s = scene();
        let (process, actor) = (s.process, s.actor);
        slot_ret(&mut s.e, PROCESS_VT, 0x27c, 0x7700);
        s.e.register(0x0057_2380, |_, _| float(41.0));
        s.e.register(FLOAT_TO_INT, |_, a| {
            word(f64::from_bits(a[0] as u64 | (a[1] as u64) << 32) as i32 as u32)
        });
        s.e.register(0x0067_5a50, |_, _| word(0xa1));
        s.e.register(0x0067_5c20, |_, _| word(0xb2));
        s.e.call(0x0090_4f50, &args![process, actor]);
        assert_eq!(calls(&s.e, 0x0067_5a50), vec![vec![0x7700, actor]]);
        assert_eq!(calls(&s.e, 0x0067_5c20), vec![vec![0x7700, actor]]);
        assert_eq!(
            calls(&s.e, 0x008b_3690),
            vec![vec![
                actor,
                process + 0x148,
                0xb2,
                0xa1,
                40.0f32.to_bits(),
                0
            ]]
        );
        assert!(slot_calls(&s.e, PROCESS_VT, 0x288).is_empty());

        // The player is moved through its own vtable instead.
        let mut s = scene();
        let (process, player) = (s.process, s.player);
        vtable(&mut s.e, player, OTHER_VT);
        default_slots(&mut s.e, OTHER_VT);
        s.e.register(0x0057_2380, |_, _| float(90.0));
        s.e.register(FLOAT_TO_INT, |_, _| word(90));
        s.e.register(0x0056_8650, |_, _| float(1.75));
        s.e.call(0x0090_4f50, &args![process, player]);
        assert_eq!(
            slot_calls(&s.e, OTHER_VT, 0x2a8),
            vec![vec![player, process + 0x148]]
        );
        assert_eq!(
            slot_calls(&s.e, OTHER_VT, 0x2c4),
            vec![vec![player, 1.75f32.to_bits()]]
        );
        assert!(calls(&s.e, 0x0067_5a50).is_empty());
    }

    /// `ProcessSandman` close to its target, with the victim wired up.
    fn sandman_close() -> (Scene, u32, u32, u32) {
        let (mut s, victim, victim_process, actor_process) = victim_scene();
        s.e.register(0x0057_2380, |_, _| float(10.0));
        s.e.register(FLOAT_TO_INT, |_, _| word(10));
        (s, victim, victim_process, actor_process)
    }

    #[test]
    fn test_high_process_process_sandman_attack() {
        let (mut s, victim, victim_process, actor_process) = sandman_close();
        let (process, actor, player) = (s.process, s.actor, s.player);
        slot_ret(&mut s.e, PROCESS_VT, 0x454, 1);
        // The player counts 2 (positive): the attack names the actor.
        s.e.mem.set_u32(player + 0x1fc, 2);
        s.e.call(0x0090_4f50, &args![process, actor]);
        assert_eq!(slot_calls(&s.e, PROCESS_VT, 0x450), vec![vec![process, 0]]);
        assert_eq!(
            calls(&s.e, 0x0089_d900),
            vec![vec![victim, actor, 0.0f32.to_bits()]]
        );
        assert_eq!(
            slot_calls(&s.e, PROCESS_VT, 0x288),
            vec![vec![process, actor, 1]]
        );
        assert_eq!(
            slot_calls(&s.e, VALUE_VT, 0x210),
            vec![vec![victim_process, 0, victim]]
        );
        let actor_calls = slot_calls(&s.e, 0x7500_0000, 0x210);
        assert_eq!(actor_calls, vec![vec![actor_process, 0, actor]]);
        assert_eq!(calls(&s.e, 0x005c_c7a0), vec![vec![player, 0]]);
        assert_eq!(calls(&s.e, 0x0093_dd20), vec![vec![player]]);

        // With a non-positive count the attack has no actor.
        let (mut s, victim, _, _) = sandman_close();
        let (process, actor) = (s.process, s.actor);
        s.e.call(0x0090_4f50, &args![process, actor]);
        assert_eq!(
            calls(&s.e, 0x0089_d900),
            vec![vec![victim, 0, 0.0f32.to_bits()]]
        );
    }

    #[test]
    fn test_high_process_process_sandman_busy_and_special() {
        // A current furniture that the process accepts ends it at once.
        let (mut s, _, _, _) = sandman_close();
        let (process, actor) = (s.process, s.actor);
        slot_ret(&mut s.e, PROCESS_VT, 0x4c8, 0x9100);
        slot_ret(&mut s.e, PROCESS_VT, 0x2b0, 1);
        s.e.call(0x0090_4f50, &args![process, actor]);
        assert_eq!(
            slot_calls(&s.e, PROCESS_VT, 0x2b0),
            vec![vec![process, actor]]
        );
        assert!(calls(&s.e, 0x0089_d900).is_empty());
        assert!(slot_calls(&s.e, PROCESS_VT, 0x288).is_empty());

        // A furniture that is refused goes on to the attack.
        let (mut s, _, _, _) = sandman_close();
        let (process, actor) = (s.process, s.actor);
        slot_ret(&mut s.e, PROCESS_VT, 0x4c8, 0x9100);
        s.e.call(0x0090_4f50, &args![process, actor]);
        assert_eq!(calls(&s.e, 0x0089_d900).len(), 1);

        // The actor answering 4, or 9 the second time, takes the other branch.
        for answers in [[4u32, 4], [0, 9]] {
            let (mut s, victim, victim_process, _) = sandman_close();
            let (process, actor, player) = (s.process, s.actor, s.player);
            let sequence = Rc::new(RefCell::new(answers.to_vec()));
            let queue = sequence.clone();
            on_slot(&mut s.e, ACTOR_VT, 0x214, move |_, _| {
                let mut queue = queue.borrow_mut();
                word(if queue.len() > 1 {
                    queue.remove(0)
                } else {
                    queue[0]
                })
            });
            slot_ret(&mut s.e, PROCESS_VT, 0x454, 1);
            s.e.call(0x0090_4f50, &args![process, actor]);
            assert_eq!(
                slot_calls(&s.e, PROCESS_VT, 0x288),
                vec![vec![process, actor, 1]]
            );
            assert_eq!(
                slot_calls(&s.e, VALUE_VT, 0x210),
                vec![vec![victim_process, 0, victim]]
            );
            assert_eq!(slot_calls(&s.e, PROCESS_VT, 0x4ec), vec![vec![process, 1]]);
            assert_eq!(calls(&s.e, 0x005c_c7a0), vec![vec![player, 0]]);
            assert_eq!(calls(&s.e, 0x008c_0460), vec![vec![victim, actor, 0, 1]]);
            assert!(calls(&s.e, 0x0089_d900).is_empty());
        }
    }

    #[test]
    fn test_fn_00905230() {
        let mut e = engine();
        let player = e.mem.alloc(0x400);
        e.mem.set_u32(player + 0x1fc, 5);
        assert_eq!(e.call(0x0090_5230, &args![player]).i32(), 5);
        e.mem.set_i32(player + 0x1fc, -3);
        assert_eq!(e.call(0x0090_5230, &args![player]).i32(), -3);
    }

    #[test]
    fn test_high_process_set_path_look_at_target() {
        let mut s = scene();
        let process = s.process;
        s.e.call(0x0090_5250, &args![process, 0x4321u32]);
        assert_eq!(s.e.mem.u32(process + 0x2ac), 0x4321);
        assert_eq!(calls(&s.e, SET_TARGETED), vec![vec![0x4321, 1]]);
        s.e.call(0x0090_5250, &args![process, 0u32]);
        assert_eq!(s.e.mem.u32(process + 0x2ac), 0);
        assert_eq!(calls(&s.e, SET_TARGETED).len(), 1);
    }

    #[test]
    fn test_high_process_set_animation() {
        // The check passes and the animation is null: the action is reset.
        let mut s = scene();
        let process = s.process;
        s.e.register(0x0060_2150, |_, _| word(1));
        s.e.call(0x0090_5280, &args![process, 0u32]);
        assert_eq!(
            slot_calls(&s.e, PROCESS_VT, 0x3ec),
            vec![vec![process, 0xffff_ffff, 0]]
        );
        assert_eq!(calls(&s.e, 0x0091_fe80), vec![vec![process, 0]]);

        // A non-null animation, or a failing check: only the store.
        let mut s = scene();
        let process = s.process;
        s.e.register(0x0060_2150, |_, _| word(1));
        s.e.call(0x0090_5280, &args![process, 0x6000u32]);
        assert!(slot_calls(&s.e, PROCESS_VT, 0x3ec).is_empty());
        assert_eq!(calls(&s.e, 0x0091_fe80), vec![vec![process, 0x6000]]);
        let mut s = scene();
        let process = s.process;
        s.e.call(0x0090_5280, &args![process, 0u32]);
        assert!(slot_calls(&s.e, PROCESS_VT, 0x3ec).is_empty());
        assert_eq!(calls(&s.e, 0x0091_fe80).len(), 1);
    }

    #[test]
    fn test_fn_009052c0() {
        let mut e = engine();
        let array = e.mem.alloc(0x10);
        e.mem.set_u32(array + 4, 0x1000);
        assert_eq!(e.call(0x0090_52c0, &args![array, 0u32]).u32(), 0x1000);
        assert_eq!(
            e.call(0x0090_52c0, &args![array, 3u32]).u32(),
            0x1000 + 3 * 0x54
        );
    }

    #[test]
    fn test_fn_009052e0() {
        let mut e = engine();
        let (cell, source) = (e.mem.alloc(8), e.mem.alloc(8));
        e.mem.set_u32(cell, 0x100);
        e.mem.set_u32(source, 0x200);
        // Different pointers: release the old, copy, take the new.
        assert_eq!(e.call(0x0090_52e0, &args![cell, source]).u32(), cell);
        assert_eq!(e.mem.u32(cell), 0x200);
        assert_eq!(calls(&e, 0x0044_b050), vec![vec![0x100]]);
        assert_eq!(calls(&e, 0x0044_b010), vec![vec![0x200]]);
        // The same pointer: nothing.
        e.call(0x0090_52e0, &args![cell, source]);
        assert_eq!(calls(&e, 0x0044_b050).len(), 1);
        assert_eq!(calls(&e, 0x0044_b010).len(), 1);
        // Old null, new null.
        let mut e = engine();
        let (cell, source) = (e.mem.alloc(8), e.mem.alloc(8));
        e.mem.set_u32(source, 0x200);
        e.call(0x0090_52e0, &args![cell, source]);
        assert!(calls(&e, 0x0044_b050).is_empty());
        assert_eq!(calls(&e, 0x0044_b010), vec![vec![0x200]]);
        e.mem.set_u32(source, 0);
        e.call(0x0090_52e0, &args![cell, source]);
        assert_eq!(calls(&e, 0x0044_b050), vec![vec![0x200]]);
        assert_eq!(e.mem.u32(cell), 0);
        assert_eq!(calls(&e, 0x0044_b010).len(), 1);
    }

    /// Frees go through the list destructor double, which here also records
    /// which node it got.
    fn list_of(e: &mut Engine, items: &[u32]) -> (u32, Vec<u32>) {
        let head = e.mem.alloc(8);
        fill_list(e, head, items);
        let mut nodes = vec![head];
        let mut node = head;
        while e.mem.u32(node + 4) != 0 {
            node = e.mem.u32(node + 4);
            nodes.push(node);
        }
        (head, nodes)
    }

    fn remove_item(e: &mut Engine, head: u32, item: u32) {
        let slot = e.mem.alloc(4);
        e.mem.set_u32(slot, item);
        e.call(0x0090_5330, &args![head, slot]);
    }

    #[test]
    fn test_fn_00905330_middle_and_tail() {
        let mut e = engine();
        let (head, nodes) = list_of(&mut e, &[10, 20, 30]);
        remove_item(&mut e, head, 20);
        assert_eq!(e.mem.u32(head), 10);
        assert_eq!(e.mem.u32(head + 4), nodes[2]);
        assert_eq!(e.mem.u32(nodes[1] + 4), 0, "the removed node is unlinked");
        assert_eq!(calls(&e, LIST_DELETE), vec![vec![nodes[1], 1]]);
        remove_item(&mut e, head, 30);
        assert_eq!(e.mem.u32(head + 4), 0);
        assert_eq!(calls(&e, LIST_DELETE).len(), 2);
    }

    #[test]
    fn test_fn_00905330_head() {
        // The head takes the second node's item and next, and the second
        // node is freed.
        let mut e = engine();
        let (head, nodes) = list_of(&mut e, &[10, 20, 30]);
        remove_item(&mut e, head, 10);
        assert_eq!(e.mem.u32(head), 20);
        assert_eq!(e.mem.u32(head + 4), nodes[2]);
        assert_eq!(calls(&e, LIST_DELETE), vec![vec![nodes[1], 1]]);
        // A one-node list: the item is cleared and nothing is freed.
        let mut e = engine();
        let (head, _) = list_of(&mut e, &[10]);
        remove_item(&mut e, head, 10);
        assert_eq!(e.mem.u32(head), 0);
        assert!(calls(&e, LIST_DELETE).is_empty());
    }

    #[test]
    fn test_fn_00905330_nothing_to_remove() {
        let mut e = engine();
        let (head, _) = list_of(&mut e, &[10, 20]);
        remove_item(&mut e, head, 99);
        remove_item(&mut e, head, 0);
        assert_eq!(e.mem.u32(head), 10);
        assert!(calls(&e, LIST_DELETE).is_empty());
        // An empty list is not even searched.
        let mut e = engine();
        let head = e.mem.alloc(8);
        remove_item(&mut e, head, 10);
        assert!(calls(&e, LIST_DELETE).is_empty());
    }

    #[test]
    fn test_bs_simple_array_pathing_avoid_node_add() {
        let mut e = engine();
        let array = e.mem.alloc(0x18);
        let buffer = e.mem.alloc(0x24 * 3);
        e.mem.set_u32(array + 4, buffer);
        // 00905a10 makes room and answers the new index.
        e.register(0x0090_5a10, |_, _| word(2));
        let node = e.mem.alloc(0x24);
        for i in 0..9 {
            e.mem.set_u32(node + 4 * i, 0xa0 + i);
        }
        assert_eq!(e.call(0x0090_5450, &args![array, node]).u32(), 2);
        for i in 0..9 {
            assert_eq!(e.mem.u32(buffer + 2 * 0x24 + 4 * i), 0xa0 + i);
            assert_eq!(e.mem.u32(buffer + 4 * i), 0, "other elements untouched");
        }
        // The element is constructed in place first (with 1).
        assert_eq!(
            calls(&e, 0x006e_3c90),
            vec![vec![array, buffer + 2 * 0x24, 1]]
        );
    }

    fn queue(e: &mut Engine) -> u32 {
        let queue = e.mem.alloc(0x20);
        vtable(e, queue, OTHER_VT);
        default_slots(e, OTHER_VT);
        queue
    }

    #[test]
    fn test_bst_common_message_queue_actor_pathing_message_push() {
        let mut e = engine();
        let queue = queue(&mut e);
        slot_ret(&mut e, OTHER_VT, 0x14, 1);
        assert!(e.call(0x0090_54a0, &args![queue, 0x77u32]).bool());
        assert_eq!(slot_calls(&e, OTHER_VT, 0x14), vec![vec![queue, 0x77]]);
        // The lock is taken (0 -> 1) and released (1 -> 0).
        assert_eq!(
            calls(&e, COMPARE_EXCHANGE_LOCK),
            vec![vec![queue + 4, 0, 1], vec![queue + 4, 1, 0]]
        );
        // A false result is passed on.
        slot_ret(&mut e, OTHER_VT, 0x14, 0);
        assert!(!e.call(0x0090_54a0, &args![queue, 0x77u32]).bool());
    }

    #[test]
    fn test_bst_common_message_queue_actor_pathing_message_push_spins() {
        // The lock is taken by someone else twice before it is free.
        let mut e = engine();
        let queue = queue(&mut e);
        slot_ret(&mut e, OTHER_VT, 0x14, 1);
        let tries = Rc::new(Cell::new(0));
        let counter = tries.clone();
        e.register_double(COMPARE_EXCHANGE_LOCK, move |_, a| {
            if a[1] == 0 {
                counter.set(counter.get() + 1);
                word(if counter.get() <= 2 { 1 } else { 0 })
            } else {
                Ret::default()
            }
        });
        assert!(e.call(0x0090_54a0, &args![queue, 0x77u32]).bool());
        assert_eq!(tries.get(), 3);
        assert_eq!(slot_calls(&e, OTHER_VT, 0x14).len(), 1);
    }

    #[test]
    fn test_bst_common_message_queue_actor_pathing_message_pop() {
        let mut e = engine();
        let queue = queue(&mut e);
        slot_ret(&mut e, OTHER_VT, 0x18, 1);
        assert!(e.call(0x0090_5510, &args![queue, 0x88u32]).bool());
        assert_eq!(slot_calls(&e, OTHER_VT, 0x18), vec![vec![queue, 0x88]]);
        assert_eq!(
            calls(&e, COMPARE_EXCHANGE_LOCK),
            vec![vec![queue + 4, 0, 1], vec![queue + 4, 1, 0]]
        );
        slot_ret(&mut e, OTHER_VT, 0x18, 0);
        assert!(!e.call(0x0090_5510, &args![queue, 0x88u32]).bool());
        let tries = Rc::new(Cell::new(0));
        let counter = tries.clone();
        e.register_double(COMPARE_EXCHANGE_LOCK, move |_, a| {
            if a[1] == 0 {
                counter.set(counter.get() + 1);
                word((counter.get() == 1) as u32)
            } else {
                Ret::default()
            }
        });
        e.call(0x0090_5510, &args![queue, 0x88u32]);
        assert_eq!(tries.get(), 2);
    }

    // ---- 00905580 .. 009ee020 ----------------------------------------------

    /// An engine with call logging, for the functions of this block; each test
    /// registers the callees it reaches.
    fn bare() -> Engine {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e
    }

    fn quiet(e: &mut Engine, addresses: &[u32]) {
        for address in addresses {
            e.register(*address, |_, _| Ret::default());
        }
    }

    #[test]
    fn test_fn_00905580() {
        let mut e = bare();
        let queue = e.mem.alloc(0x14);
        e.mem.set_u32(queue + 4, 0xdead);
        e.mem.set_u32(queue + 0xc, 0xbeef);
        assert_eq!(e.call(0x0090_5580, &args![queue, 0x5555u32]).u32(), queue);
        assert_eq!(e.mem.u32(queue), 0x0108_83f8);
        assert_eq!(
            e.mem.u32(queue + 4),
            0,
            "the base constructor clears the lock"
        );
        assert_eq!(e.mem.u32(queue + 8), 0x5555);
        assert_eq!(e.mem.u32(queue + 0xc), 0);
        assert_eq!(e.mem.u32(queue + 0x10), queue + 0xc);
    }

    #[test]
    fn test_bst_common_ll_message_queue_actor_pathing_message_destructor() {
        let mut e = bare();
        quiet(&mut e, &[0x006e_9bd0, 0x0080_0190]);
        let pops = Rc::new(Cell::new(0));
        let counter = pops.clone();
        // Two messages are queued, the third pop finds the queue empty.
        e.register_double(0x006e_c390, move |_, _| {
            counter.set(counter.get() + 1);
            word((counter.get() <= 2) as u32)
        });
        let queue = e.mem.alloc(0x14);
        e.call(0x0090_55c0, &args![queue]);
        assert_eq!(pops.get(), 3);
        // The same temporary is constructed first, popped into and destroyed.
        let construct = calls(&e, 0x006e_9bd0);
        assert_eq!(construct.len(), 1);
        let message = construct[0][0];
        assert_eq!(calls(&e, 0x0080_0190), vec![vec![message]]);
        assert!(calls(&e, 0x006e_c390)
            .iter()
            .all(|a| a == &vec![queue, message]));
        // The base destructors leave the base vtable.
        assert_eq!(e.mem.u32(queue), 0x0108_8438);
    }

    #[test]
    fn test_fn_00905650_and_00905670() {
        let mut e = bare();
        let object = e.mem.alloc(8);
        assert_eq!(e.call(0x0090_5670, &args![object]).u32(), object);
        assert_eq!(e.mem.u32(object), 0x0108_8438);
        e.mem.set_u32(object, 0);
        assert_eq!(e.call(0x0090_5650, &args![object]).u32(), object);
        assert_eq!(e.mem.u32(object), 0x0108_8438, "ends in the base vtable");
    }

    #[test]
    fn test_scalar_deleting_destructors_of_the_queues() {
        for (address, vtable) in [(0x0090_5690u32, 0x0108_8438u32), (0x0090_56c0, 0x0108_8438)] {
            let mut e = bare();
            quiet(&mut e, &[OPERATOR_DELETE]);
            let object = e.mem.alloc(8);
            assert_eq!(e.call(address, &args![object, 0u32]).u32(), object);
            assert_eq!(e.mem.u32(object), vtable);
            assert!(calls(&e, OPERATOR_DELETE).is_empty());
            assert_eq!(e.call(address, &args![object, 3u32]).u32(), object);
            assert_eq!(calls(&e, OPERATOR_DELETE), vec![vec![object]]);
        }
    }

    /// A free list double: the free node, and the nodes constructed for
    /// messages.
    fn push_scene(e: &mut Engine, free_node: u32) {
        e.register_double(0x006e_cd60, move |e, a| {
            if free_node == 0 {
                return word(0);
            }
            e.mem.set_u32(a[1], free_node);
            word(1)
        });
        quiet(e, &[0x0090_5b00]);
    }

    #[test]
    fn test_bst_common_ll_message_queue_actor_pathing_message_do_try_push() {
        let mut e = bare();
        let queue = e.mem.alloc(0x14);
        // No free list: nothing happens.
        assert!(!e.call(0x0090_56f0, &args![queue, 0x77u32]).bool());
        let free_list = e.mem.alloc(0x10);
        e.mem.set_u32(queue + 8, free_list);
        // A free list without a free node: false.
        push_scene(&mut e, 0);
        assert!(!e.call(0x0090_56f0, &args![queue, 0x77u32]).bool());
        assert_eq!(e.mem.u32(queue + 0xc), 0);
        // The first message becomes the head and the tail.
        let first = e.mem.alloc(0x10);
        push_scene(&mut e, first);
        assert!(e.call(0x0090_56f0, &args![queue, 0x77u32]).bool());
        assert_eq!(calls(&e, 0x0090_5b00), vec![vec![first, 0x77]]);
        assert_eq!(e.mem.u32(queue + 0xc), first);
        assert_eq!(e.mem.u32(first + 8), 0);
        assert_eq!(e.mem.u32(queue + 0x10), first + 8);
        // The second is linked after it.
        let second = e.mem.alloc(0x10);
        push_scene(&mut e, second);
        assert!(e.call(0x0090_56f0, &args![queue, 0x78u32]).bool());
        assert_eq!(e.mem.u32(queue + 0xc), first);
        assert_eq!(e.mem.u32(first + 8), second);
        assert_eq!(e.mem.u32(queue + 0x10), second + 8);
    }

    #[test]
    fn test_bst_common_ll_message_queue_actor_pathing_message_do_try_pop() {
        let mut e = bare();
        quiet(&mut e, &[MESSAGE_ASSIGN_ADDRESS, 0x009b_c890]);
        e.register(LIST_NODE_ITEM, |_, a| word(a[0] + 0x100));
        e.register_double(0x006e_cc70, |e, a| {
            e.mem.set_u32(a[1], 0);
            Ret::default()
        });
        let free_list = e.mem.alloc(0x10);
        let queue = e.mem.alloc(0x14);
        e.mem.set_u32(queue + 8, free_list);
        let message = e.mem.alloc(0x10);
        // Empty: false and nothing assigned.
        assert!(!e.call(0x0090_5770, &args![queue, message]).bool());
        assert!(calls(&e, MESSAGE_ASSIGN_ADDRESS).is_empty());
        // Two nodes: the head is consumed and handed back.
        let first = e.mem.alloc(0x10);
        let second = e.mem.alloc(0x10);
        e.mem.set_u32(first + 8, second);
        e.mem.set_u32(queue + 0xc, first);
        e.mem.set_u32(queue + 0x10, second + 8);
        assert!(e.call(0x0090_5770, &args![queue, message]).bool());
        assert_eq!(
            calls(&e, MESSAGE_ASSIGN_ADDRESS),
            vec![vec![message, first + 0x100]]
        );
        assert_eq!(e.mem.u32(queue + 0xc), second);
        assert_eq!(
            e.mem.u32(queue + 0x10),
            second + 8,
            "the tail is not touched"
        );
        // The node's message is released and the node given back.
        assert_eq!(calls(&e, 0x009b_c890), vec![vec![first, 0]]);
        let give = calls(&e, 0x006e_cc70);
        assert_eq!(give.len(), 1);
        assert_eq!(give[0][0], free_list);
        // The last node empties the queue.
        assert!(e.call(0x0090_5770, &args![queue, message]).bool());
        assert_eq!(e.mem.u32(queue + 0xc), 0);
        assert!(!e.call(0x0090_5770, &args![queue, message]).bool());
    }

    /// `ActorPathingMessage` assignment (`006e9c60`).
    const MESSAGE_ASSIGN_ADDRESS: u32 = 0x006e_9c60;

    #[test]
    fn test_fn_009057d0() {
        let mut e = bare();
        quiet(&mut e, &[0x0040_1970, 0x0040_f6e0]);
        let slot = e.mem.alloc(4);
        // Empty slot: only the acquire.
        assert_eq!(e.call(0x0090_57d0, &args![slot, 0x1000u32]).u32(), slot);
        assert_eq!(e.mem.u32(slot), 0x1000);
        assert!(calls(&e, 0x0040_1970).is_empty());
        assert_eq!(calls(&e, 0x0040_f6e0), vec![vec![0x1014]]);
        // The same object again: nothing.
        e.call(0x0090_57d0, &args![slot, 0x1000u32]);
        assert_eq!(calls(&e, 0x0040_f6e0).len(), 1);
        // Another object: release the old, acquire the new.
        e.call(0x0090_57d0, &args![slot, 0x2000u32]);
        assert_eq!(e.mem.u32(slot), 0x2000);
        assert_eq!(calls(&e, 0x0040_1970), vec![vec![0x1014]]);
        assert_eq!(calls(&e, 0x0040_f6e0).last().unwrap(), &vec![0x2014]);
        // Null: release only.
        e.call(0x0090_57d0, &args![slot, 0u32]);
        assert_eq!(e.mem.u32(slot), 0);
        assert_eq!(calls(&e, 0x0040_1970).last().unwrap(), &vec![0x2014]);
        assert_eq!(calls(&e, 0x0040_f6e0).len(), 2);
    }

    #[test]
    fn test_fn_00905820() {
        let mut e = bare();
        let created = e.mem.alloc(8);
        e.register_double(OPERATOR_NEW, move |_, _| word(created));
        e.register(0x0047_0440, |e, a| {
            let item = e.mem.u32(a[1]);
            e.mem.set_u32(a[0], item);
            e.mem.set_u32(a[0] + 4, 0);
            word(a[0])
        });
        let item = e.mem.alloc(4);
        let head = e.mem.alloc(8);
        // A null item adds nothing.
        e.call(0x0090_5820, &args![head, item]);
        assert_eq!(e.mem.u32(head), 0);
        assert!(calls(&e, OPERATOR_NEW).is_empty());
        // An empty head takes the item itself.
        e.mem.set_u32(item, 0x42);
        e.call(0x0090_5820, &args![head, item]);
        assert_eq!(e.mem.u32(head), 0x42);
        assert!(calls(&e, OPERATOR_NEW).is_empty());
        // A used head gets a new node after the last one.
        let tail = e.mem.alloc(8);
        e.mem.set_u32(tail, 0x41);
        e.mem.set_u32(head + 4, tail);
        e.call(0x0090_5820, &args![head, item]);
        assert_eq!(calls(&e, OPERATOR_NEW), vec![vec![8]]);
        assert_eq!(e.mem.u32(tail + 4), created);
        assert_eq!(e.mem.u32(created), 0x42);
        assert_eq!(e.mem.u32(head + 4), tail, "the head's link is unchanged");
    }

    #[test]
    fn test_fn_00905820_allocation_fails() {
        let mut e = bare();
        e.register(OPERATOR_NEW, |_, _| word(0));
        quiet(&mut e, &[0x0047_0440]);
        let item = e.mem.alloc(4);
        e.mem.set_u32(item, 0x42);
        let head = e.mem.alloc(8);
        e.mem.set_u32(head, 0x41);
        e.call(0x0090_5820, &args![head, item]);
        assert_eq!(e.mem.u32(head + 4), 0);
        assert!(calls(&e, 0x0047_0440).is_empty());
    }

    #[test]
    fn test_fn_009058e0_and_00905910() {
        let mut e = bare();
        quiet(&mut e, &[0x006b_3eb0, 0x0084_54f0]);
        let array = e.mem.alloc(0x10);
        assert_eq!(e.call(0x0090_58e0, &args![array, 12u32]).u32(), array);
        assert_eq!(e.mem.u32(array), 0x0108_8450);
        assert_eq!(calls(&e, 0x006b_3eb0), vec![vec![array, 12, 12]]);
        e.mem.set_u32(array, 0);
        e.call(0x0090_5910, &args![array]);
        assert_eq!(e.mem.u32(array), 0x0108_8450);
        assert_eq!(calls(&e, 0x0084_54f0), vec![vec![array, 1]]);
    }

    #[test]
    fn test_bst_common_ll_message_queue_actor_pathing_message_scalar_deleting_destructor() {
        let mut e = bare();
        quiet(
            &mut e,
            &[0x006e_9bd0, 0x0080_0190, 0x006e_c390, OPERATOR_DELETE],
        );
        let queue = e.mem.alloc(0x14);
        assert_eq!(e.call(0x0090_5930, &args![queue, 0u32]).u32(), queue);
        assert_eq!(calls(&e, 0x006e_9bd0).len(), 1, "the destructor ran");
        assert_eq!(e.mem.u32(queue), 0x0108_8438);
        assert!(calls(&e, OPERATOR_DELETE).is_empty());
        e.call(0x0090_5930, &args![queue, 1u32]);
        assert_eq!(calls(&e, OPERATOR_DELETE), vec![vec![queue]]);
    }

    #[test]
    fn test_bs_simple_array_tes_idle_form_p_1024_scalar_deleting_destructor() {
        let mut e = bare();
        quiet(&mut e, &[0x0084_54f0, OPERATOR_DELETE]);
        let array = e.mem.alloc(0x10);
        assert_eq!(e.call(0x0090_5960, &args![array, 0u32]).u32(), array);
        assert_eq!(calls(&e, 0x0084_54f0), vec![vec![array, 1]]);
        assert!(calls(&e, OPERATOR_DELETE).is_empty());
        e.call(0x0090_5960, &args![array, 1u32]);
        assert_eq!(calls(&e, OPERATOR_DELETE), vec![vec![array]]);
    }

    #[test]
    fn test_fn_00905990_and_009059f0() {
        let mut e = bare();
        let object = e.mem.alloc(8);
        assert_eq!(e.call(0x0090_59f0, &args![object]).u32(), object);
        assert_eq!(e.mem.u32(object), 0x0108_8438);
        e.mem.set_u32(object + 4, 9);
        assert_eq!(e.call(0x0090_5990, &args![object]).u32(), object);
        assert_eq!(e.mem.u32(object), 0x0108_8418);
        assert_eq!(e.mem.u32(object + 4), 0);
    }

    #[test]
    fn test_bst_free_list_actor_pathing_message_deallocate() {
        let mut e = bare();
        quiet(&mut e, &[0x009b_c890, 0x006e_cc70]);
        let free_list = e.mem.alloc(0x10);
        let slot = e.mem.alloc(4);
        e.mem.set_u32(slot, 0x3000);
        e.call(0x0090_59c0, &args![free_list, slot]);
        // The node's message is released first, then the slot is given back.
        assert_eq!(calls(&e, 0x009b_c890), vec![vec![0x3000, 0]]);
        assert_eq!(calls(&e, 0x006e_cc70), vec![vec![free_list, slot]]);
        let log = e.call_log.as_ref().unwrap();
        let release = log.iter().position(|(a, _)| *a == 0x009b_c890).unwrap();
        let give = log.iter().position(|(a, _)| *a == 0x006e_cc70).unwrap();
        assert!(release < give);
    }

    fn array_scene(e: &mut Engine, count: u32, capacity: u32) -> u32 {
        let array = e.mem.alloc(0x10);
        vtable(e, array, VALUE_VT);
        e.mem.set_u32(array + 4, 0x6000);
        e.mem.set_u32(array + 8, count);
        e.mem.set_u32(array + 0xc, capacity);
        e.register(0x0043_8b90, |e, a| {
            word((e.mem.u32(a[0] + 8) == e.mem.u32(a[0] + 0xc)) as u32)
        });
        quiet(e, &[0x006e_3d30]);
        e.register(0x009a_3910, |_, _| word(8));
        array
    }

    #[test]
    fn test_fn_00905a10_first_allocation() {
        let mut e = bare();
        let array = array_scene(&mut e, 0, 0);
        slot_ret(&mut e, VALUE_VT, 4, 0x7000);
        assert_eq!(e.call(0x0090_5a10, &args![array]).u32(), 0);
        assert_eq!(slot_calls(&e, VALUE_VT, 4), vec![vec![array, 4]]);
        assert_eq!(e.mem.u32(array + 4), 0x7000);
        assert_eq!(e.mem.u32(array + 8), 1);
        assert_eq!(e.mem.u32(array + 0xc), 4);
    }

    #[test]
    fn test_fn_00905a10_grow_and_room() {
        let mut e = bare();
        let array = array_scene(&mut e, 4, 4);
        assert_eq!(e.call(0x0090_5a10, &args![array]).u32(), 4);
        assert_eq!(calls(&e, 0x006e_3d30), vec![vec![array, 8, 4]]);
        assert_eq!(e.mem.u32(array + 8), 5);
        assert_eq!(e.mem.u32(array + 0xc), 8);
        // Room left: only the count changes.
        assert_eq!(e.call(0x0090_5a10, &args![array]).u32(), 5);
        assert_eq!(calls(&e, 0x006e_3d30).len(), 1);
        assert_eq!(e.mem.u32(array + 8), 6);
        assert_eq!(e.mem.u32(array + 4), 0x6000);
    }

    #[test]
    fn test_fn_00905aa0() {
        let mut e = bare();
        quiet(&mut e, &[0x0090_5b00]);
        let out = e.mem.alloc(4);
        push_scene(&mut e, 0);
        assert!(!e.call(0x0090_5aa0, &args![0x5000u32, out, 0x77u32]).bool());
        assert!(calls(&e, 0x0090_5b00).is_empty());
        push_scene(&mut e, 0x8000);
        assert!(e.call(0x0090_5aa0, &args![0x5000u32, out, 0x77u32]).bool());
        assert_eq!(
            calls(&e, 0x006e_cd60),
            vec![vec![0x5000, out], vec![0x5000, out]]
        );
        assert_eq!(calls(&e, 0x0090_5b00), vec![vec![0x8000, 0x77]]);
    }

    #[test]
    fn test_fn_00905ae0() {
        let mut e = bare();
        quiet(&mut e, &[0x009b_c890]);
        e.call(0x0090_5ae0, &args![0x4000u32]);
        assert_eq!(calls(&e, 0x009b_c890), vec![vec![0x4000, 0]]);
    }

    #[test]
    fn test_process_getters() {
        let mut e = bare();
        let process = e.mem.alloc(0x240);
        e.mem.set_u32(process + 0x16c, 0x1234);
        e.mem.set_u32(process + 0x224, 0x5678);
        e.mem.set_f32(process + 0xa8, 12.5);
        e.mem.set_f32(process + 0xac, -3.0);
        assert_eq!(e.call(0x009b_6600, &args![process]).u32(), 0x1234);
        assert_eq!(e.call(0x009e_e020, &args![process]).u32(), 0x5678);
        assert_eq!(e.call(0x009b_88a0, &args![process]).f32(), 12.5);
        assert_eq!(e.call(0x009b_88c0, &args![process]).f32(), -3.0);
    }
}

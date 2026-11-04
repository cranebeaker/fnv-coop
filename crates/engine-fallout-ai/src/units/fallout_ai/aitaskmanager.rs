//! `fallout/ai/aitaskmanager.cpp` (Xbox PDB source unit), subsystem `fallout/ai`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The AI task manager runs the per-actor work of a frame as "tasklets"
//! (`MobileObjectTaskletData` and its five subclasses, each embedded in the
//! manager: Detection at +0x00, Animation +0x40, PackageUpdate +0x78,
//! ActorUpdate +0xBC, ActorsScript +0xF8, Movement +0x138). A tasklet owns a
//! message queue (at +0x1C) that the producer fills with count, mob and end
//! messages; `Process` / `RunToCompletion` drain it and dispatch each message
//! to the subclass's `HandleCountMessage` / `HandleMobMessage` /
//! `HandleEndMessage` (vtable slots +0x10 / +0x14 / +0x18).
//! The movement tasklet's `iUpdated` is Xbox PDB +0x58 and PC +0x38.
//!
//! The thread manager, its ten-entry PC task ring, worker semaphore helpers,
//! the scheduling bodies (`008c94e0`, `008ca070`), the worker startup and
//! shutdown (`008c9d70`, `008c9e40`) and the message queue with its node
//! pool (`BSTFreeList<MobileObjectMessage>`: lock word +4, free-list head
//! +8, 0x1000 nodes of 0x10 bytes at +0xC, a node's next link at +0xC) are
//! also translated in this unit. The scheduling bodies read the process
//! lists' actor list through `00717e50` / `005be5c0` / `00968670` and the
//! tasklet settings through `00408d60` / `0043d4d0`; they queue count, mob
//! and end messages to the tasklets and either put the tasklets on the worker
//! ring (settings threaded) or run them to completion.
//! Every function of the unit is translated. The compiler's exception
//! frames (`FS:[0]` chains) are not translated anywhere in the unit.
//!
//! Layouts: the PC class is 0x38 bytes where the Xbox one is 0x58 (the Xbox
//! timing fields `StartTime`, `EndTime`, ... and `iThread` are not in the PC
//! build), so the subclass fields start at +0x38 here.
//!
//! The critical sections the count and end handlers enter and leave are
//! globals of other units; their roles are not named here.

#[allow(unused_imports)]
use crate::prelude::*;

/// The global holding the `PlayerCharacter` pointer.
const PLAYER_CHARACTER: u32 = 0x011d_ea3c;
/// `operator new(size)` (`00401000`) and `operator delete(ptr)` (`00401030`).
const OPERATOR_NEW: u32 = 0x0040_1000;
const OPERATOR_DELETE: u32 = 0x0040_1030;
/// Enters a critical section (`004538a0`, `this` = the section, one word
/// argument passed as 0) and leaves it (`004538c0`).
const CRITICAL_SECTION_ENTER: u32 = 0x0045_38a0;
const CRITICAL_SECTION_LEAVE: u32 = 0x0045_38c0;
/// The critical sections used by the tasklets' count/end handlers.
const CRITICAL_SECTION_011DFBE8: u32 = 0x011d_fbe8;
const CRITICAL_SECTION_011DFC04: u32 = 0x011d_fc04;
const CRITICAL_SECTION_011DFC38: u32 = 0x011d_fc38;

/// Vtables of the tasklet classes (RTTI locator in the word before).
const VTABLE_MOBILE_OBJECT_TASKLET_DATA: u32 = 0x0108_5748;
const VTABLE_DETECTION_TASK_DATA: u32 = 0x0108_5728;
const VTABLE_ANIMATION_TASK_DATA: u32 = 0x0108_5768;
const VTABLE_ACTOR_UPDATE_TASK_DATA: u32 = 0x0108_5788;
const VTABLE_PACKAGE_UPDATE_TASK_DATA: u32 = 0x0108_57a8;
const VTABLE_ACTORS_SCRIPT_TASK_DATA: u32 = 0x0108_57c8;
const VTABLE_MOVEMENT_TASK_DATA: u32 = 0x0108_57e8;

/// `BSXenonTaskletData` constructor (`006c7850`): the base of the tasklet.
const TASKLET_BASE_CONSTRUCTOR: u32 = 0x006c_7850;
/// `BSTCommonLLMessageQueue<MobileObjectMessage>` constructor (`008ca440`,
/// argument: the owner, here the manager's shared word) and destructor body
/// (`008ca480`).
const MESSAGE_QUEUE_CONSTRUCTOR: u32 = 0x008c_a440;
const MESSAGE_QUEUE_DESTRUCTOR: u32 = 0x008c_a480;
/// `BSXenonTaskletData` destructor body (`00b00ec0`).
const TASKLET_BASE_DESTRUCTOR: u32 = 0x00b0_0ec0;
/// Sets the "yielding" byte (+4) of a tasklet (`006ea330`).
const TASKLET_SET_YIELDING: u32 = 0x006e_a330;

/// `MobileObject` (here an `Actor`) members the handlers read.
/// `Actor::bProcessMe` (Xbox PDB `+0xCC`; PC `+0xBC`).
const ACTOR_PROCESS_ME: u32 = 0xbc;
/// `MobileObject::IsActor`-style test: vtable slot +0x100 answers true for
/// actors (the handlers use it to cast the mob).
const VSLOT_IS_ACTOR: u32 = 0x100;
/// `MobileObject::GetCurrentProcessType` (`00931850`).
const GET_CURRENT_PROCESS_TYPE: u32 = 0x0093_1850;
/// `MiddleHighProcess::GetSavedAcquireObject`'s body (`008d8520`): the map
/// names it so by identical-code folding; it returns the word at +0x68 of
/// the object (the actor's process).
const GET_PROCESS: u32 = 0x008d_8520;

layout! {
    /// `MobileObjectMessage` (Xbox PDB), 0xC bytes.
    pub struct MobileObjectMessage: 0x0C {
        /// `eMsgType` (Xbox PDB): 0 = count, 1 = mob, 2 = end, as the
        /// dispatch `fn_008c81c0` uses them.
        0x00 eMsgType: u32,
        /// `uiCount` (Xbox PDB).
        0x04 uiCount: u32,
        /// `pMob` (Xbox PDB): `MobileObject*`.
        0x08 pMob: Ptr,
    }

    /// `MobileObjectTaskletData` (Xbox PDB), 0x38 bytes on the PC (0x58 on
    /// the Xbox, which has timing fields after `eType`).
    pub struct MobileObjectTaskletData: 0x38 {
        /// `BSTaskletData::bYielding` (Xbox PDB).
        0x04 bYielding: bool,
        /// `pmoblist` (Xbox PDB): `BSSimpleList<MobileObject *>*`.
        0x18 pmoblist: Ptr,
        /// `uiHighActorCount` (Xbox PDB): set by a count message.
        0x30 uiHighActorCount: u32,
        /// `eType` (Xbox PDB, `AITaskType`): passed to `HandleEndMessage`.
        0x34 eType: u32,
    }

    /// `DetectionTaskData` (Xbox PDB).
    pub struct DetectionTaskData: 0x40 {
        0x18 pmoblist: Ptr,
        0x30 uiHighActorCount: u32,
        0x34 eType: u32,
        /// `fNumber` (Xbox PDB).
        0x38 fNumber: f32,
        /// `iUpdated` (Xbox PDB).
        0x3C iUpdated: i32,
    }

    /// `AnimationTaskData` (Xbox PDB): no fields of its own.
    pub struct AnimationTaskData: 0x38 {
        0x18 pmoblist: Ptr,
        0x30 uiHighActorCount: u32,
        0x34 eType: u32,
    }

    /// `ActorUpdateTaskData` (Xbox PDB).
    pub struct ActorUpdateTaskData: 0x3C {
        0x18 pmoblist: Ptr,
        0x30 uiHighActorCount: u32,
        0x34 eType: u32,
        /// `iUpdated` (Xbox PDB).
        0x38 iUpdated: i32,
    }

    /// `PackageUpdateTaskData` (Xbox PDB): two counters of its own (the
    /// Xbox PDB lists none for this build; the names follow the uses).
    pub struct PackageUpdateTaskData: 0x44 {
        0x18 pmoblist: Ptr,
        0x30 uiHighActorCount: u32,
        0x34 eType: u32,
        /// Number of mobs sent to `vtable +0x25C` (the plain update).
        0x3C iPackageUpdated: i32,
        /// Number of actors handed to the process (`vtable +0x24`).
        0x40 iActorsProcessed: i32,
    }

    /// `ActorsScriptTaskData` (Xbox PDB).
    pub struct ActorsScriptTaskData: 0x40 {
        0x18 pmoblist: Ptr,
        0x30 uiHighActorCount: u32,
        0x34 eType: u32,
        /// Number of mobs whose script ran.
        0x3C iScriptsRun: i32,
    }

    /// `MovementTaskData` (Xbox PDB): `iUpdated` is Xbox +0x58; its PC
    /// offset is +0x38 because the PC tasklet base is 0x20 bytes smaller.
    pub struct MovementTaskData: 0x3C {
        0x18 pmoblist: Ptr,
        0x30 uiHighActorCount: u32,
        0x34 eType: u32,
        /// `iUpdated` (Xbox PDB +0x58; PC +0x38).
        0x38 iUpdated: i32,
    }
}

/// Reads a word as the pointer it holds.
fn player_character(e: &Engine) -> Ptr {
    Ptr::new(e.global::<u32>(PLAYER_CHARACTER))
}

fn is_actor(e: &mut Engine, mob: Ptr) -> bool {
    e.vcall(mob.addr(), VSLOT_IS_ACTOR, &[]).bool()
}

/// The object (`011f6394`) whose float at +0xC (`0084d030`) scales the
/// per-actor delay and is passed to the process lists at the end of the
/// package update.
const TASK_SCALE_OBJECT_011F6394: u32 = 0x011f_6394;
/// A `float` the detection handler caps its random delay with.
const DELAY_CAP: u32 = 0x0101_712c;
/// `0.0` as a `double`.
const ZERO: u32 = 0x0101_2060;
/// A `float`: the amount restored to an actor of a resting player.
const RESTORE_AMOUNT: u32 = 0x0101_62c0;
/// A global whose value is `this` for `0047c850`.
const OBJECT_011DE45C: u32 = 0x011d_e45c;
/// The global holding the fader manager pointer (`007014e0` is
/// `FaderManager::GetFaderAlpha`).
const FADER_MANAGER: u32 = 0x011d_8804;
/// Game settings read with `00403e20` (a pointer to the setting's float):
/// the distance below which an actor is inserted close to the player, and
/// the detection level the actor is compared with.
const SETTING_011CCF94: u32 = 0x011c_cf94;
const SETTING_011D13C8: u32 = 0x011d_13c8;
/// The `ProcessLists` object (`0096e3c0` is
/// `ProcessLists::InsertActorCloseToPlayer`).
const PROCESS_LISTS: u32 = 0x011e_0e80;
/// A global byte cleared by `fn_008c8bd0`.
const RESET_FLAG_011E01DD: u32 = 0x011e_01dd;

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x008c8130,
            mobile_object_tasklet_data_process(Ptr<MobileObjectTaskletData>)
        ),
        entry!(
            0x008c8190,
            fn_008c8190(Ptr<MobileObjectMessage>) -> Ptr<MobileObjectMessage>
        ),
        entry!(
            0x008c81c0,
            fn_008c81c0(Ptr<MobileObjectTaskletData>, Ptr<MobileObjectMessage>, u32) -> bool
        ),
        entry!(
            0x008c8240,
            mobile_object_tasklet_data_run_to_completion(Ptr<MobileObjectTaskletData>, u32)
        ),
        entry!(
            0x008c82a0,
            fn_008c82a0(Ptr<DetectionTaskData>, u32) -> Ptr<DetectionTaskData>
        ),
        entry!(
            0x008c82e0,
            fn_008c82e0(Ptr<MobileObjectTaskletData>, u32) -> Ptr<MobileObjectTaskletData>
        ),
        entry!(
            0x008c8370,
            fn_008c8370(Ptr<MobileObjectTaskletData>, u32) -> Ptr<MobileObjectTaskletData>
        ),
        entry!(0x008c83a0, fn_008c83a0(Ptr<MobileObjectTaskletData>)),
        entry!(
            0x008c8400,
            detection_task_data_scalar_deleting_destructor(
                Ptr<DetectionTaskData>,
                u32,
            ) -> Ptr<DetectionTaskData>
        ),
        entry!(0x008c8430, fn_008c8430(Ptr<DetectionTaskData>)),
        entry!(
            0x008c8450,
            detection_task_data_handle_count_message(Ptr<DetectionTaskData>, u32)
        ),
        entry!(
            0x008c8490,
            detection_task_data_handle_end_message(Ptr<DetectionTaskData>, u32)
        ),
        entry!(
            0x008c84c0,
            detection_task_data_handle_mob_message(Ptr<DetectionTaskData>, Ptr)
        ),
        entry!(
            0x008c8830,
            fn_008c8830(Ptr<AnimationTaskData>, u32) -> Ptr<AnimationTaskData>
        ),
        entry!(
            0x008c8860,
            animation_task_data_scalar_deleting_destructor(
                Ptr<AnimationTaskData>,
                u32,
            ) -> Ptr<AnimationTaskData>
        ),
        entry!(0x008c8890, fn_008c8890(Ptr<AnimationTaskData>)),
        entry!(
            0x008c88b0,
            animation_task_data_handle_count_message(Ptr<AnimationTaskData>, u32)
        ),
        entry!(
            0x008c88e0,
            animation_task_data_handle_end_message(Ptr<AnimationTaskData>, u32)
        ),
        entry!(
            0x008c8910,
            animation_task_data_handle_mob_message(Ptr<AnimationTaskData>, Ptr)
        ),
        entry!(
            0x008c89a0,
            fn_008c89a0(Ptr<ActorUpdateTaskData>, u32) -> Ptr<ActorUpdateTaskData>
        ),
        entry!(
            0x008c89d0,
            actor_update_task_data_scalar_deleting_destructor(
                Ptr<ActorUpdateTaskData>,
                u32,
            )
                -> Ptr<ActorUpdateTaskData>
        ),
        entry!(0x008c8a00, fn_008c8a00(Ptr<ActorUpdateTaskData>)),
        entry!(
            0x008c8a20,
            actor_update_task_data_handle_count_message(Ptr<ActorUpdateTaskData>, u32)
        ),
        entry!(
            0x008c8a50,
            actor_update_task_data_handle_end_message(Ptr<ActorUpdateTaskData>, u32)
        ),
        entry!(
            0x008c8a70,
            actor_update_task_data_handle_mob_message(Ptr<ActorUpdateTaskData>, Ptr)
        ),
        entry!(
            0x008c8ae0,
            fn_008c8ae0(Ptr<PackageUpdateTaskData>, u32) -> Ptr<PackageUpdateTaskData>
        ),
        entry!(
            0x008c8b20,
            package_update_task_data_scalar_deleting_destructor(
                Ptr<PackageUpdateTaskData>,
                u32,
            )
                -> Ptr<PackageUpdateTaskData>
        ),
        entry!(0x008c8b50, fn_008c8b50(Ptr<PackageUpdateTaskData>)),
        entry!(
            0x008c8b70,
            package_update_task_data_handle_count_message(Ptr<PackageUpdateTaskData>, u32)
        ),
        entry!(0x008c8bb0, fn_008c8bb0(Ptr, u8)),
        entry!(0x008c8bd0, fn_008c8bd0()),
        entry!(
            0x008c8be0,
            package_update_task_data_handle_mob_message(Ptr<PackageUpdateTaskData>, Ptr)
        ),
        entry!(0x008c8fd0, fn_008c8fd0(Ptr, u8)),
        entry!(
            0x008c8ff0,
            package_update_task_data_handle_end_message(Ptr<PackageUpdateTaskData>, u32)
        ),
        entry!(
            0x008c9020,
            fn_008c9020(Ptr<ActorsScriptTaskData>, u32) -> Ptr<ActorsScriptTaskData>
        ),
        entry!(
            0x008c9050,
            actors_script_task_data_scalar_deleting_destructor(
                Ptr<ActorsScriptTaskData>,
                u32,
            )
                -> Ptr<ActorsScriptTaskData>
        ),
        entry!(0x008c9080, fn_008c9080(Ptr<ActorsScriptTaskData>)),
        entry!(
            0x008c90a0,
            actors_script_task_data_handle_count_message(Ptr<ActorsScriptTaskData>, u32)
        ),
        entry!(
            0x008c90e0,
            actors_script_task_data_handle_end_message(Ptr<ActorsScriptTaskData>, u32)
        ),
        entry!(
            0x008c9110,
            actors_script_task_data_handle_mob_message(Ptr<ActorsScriptTaskData>, Ptr)
        ),
        entry!(
            0x008c9170,
            movement_task_data_constructor(Ptr<MovementTaskData>, u32) -> Ptr<MovementTaskData>
        ),
        entry!(
            0x008c91a0,
            movement_task_data_scalar_deleting_destructor(
                Ptr<MovementTaskData>,
                u32,
            ) -> Ptr<MovementTaskData>
        ),
        entry!(
            0x008c91d0,
            movement_task_data_destructor(Ptr<MovementTaskData>)
        ),
        entry!(
            0x008c91f0,
            movement_task_data_handle_count_message(Ptr<MovementTaskData>, u32)
        ),
        entry!(
            0x008c9220,
            movement_task_data_handle_mob_message(Ptr<MovementTaskData>, Ptr)
        ),
        entry!(
            0x008c9300,
            movement_task_data_handle_end_message(Ptr<MovementTaskData>, u32)
        ),
        entry!(0x008c9330, ai_task_manager_constructor(Ptr) -> Ptr),
        entry!(0x008c9430, ai_task_manager_destructor(Ptr)),
        entry!(0x008c94e0, ai_task_manager_process(Ptr)),
        entry!(0x008c9a20, task_queue_item_new(Ptr, u32) -> Ptr),
        entry!(0x008c9a50, task_queue_item_existing(Ptr, u32) -> Ptr),
        entry!(0x008c9a80, ai_task_queue_enqueue(Ptr, u32)),
        entry!(0x008c9ae0, ai_task_thread_manager_add_thread(Ptr, u32)),
        entry!(0x008c9bd0, ai_task_thread_constructor(Ptr, u32) -> Ptr),
        entry!(0x008c9c70, ai_task_thread_thread_proc(Ptr) -> u32),
        entry!(0x008c9cf0, ai_task_queue_dequeue(Ptr) -> u32),
        entry!(0x008c9d70, ai_task_thread_on_startup(Ptr)),
        entry!(0x008c9e40, ai_task_thread_on_shutdown(Ptr)),
        entry!(0x008c9ec0, thread_context_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x008c9ef0, thread_context_destructor(Ptr)),
        entry!(0x008c9f70, ai_task_thread_manager_release_semaphores(Ptr)),
        entry!(0x008c9fb0, ai_task_thread_release_semaphore(Ptr)),
        entry!(0x008c9fd0, ai_task_thread_manager_wake_threads(Ptr)),
        entry!(0x008ca010, ai_task_queue_push(Ptr, Ptr)),
        entry!(0x008ca040, ai_task_queue_push_end(Ptr)),
        entry!(0x008ca070, ai_task_manager_process_parallel(Ptr)),
        entry!(0x008ca2e0, ai_task_manager_release_semaphore(Ptr, u32)),
        entry!(0x008ca300, ai_task_manager_clear_parallel_flag(Ptr)),
        entry!(0x008ca330, ai_task_manager_signal_semaphore(Ptr, u32)),
        entry!(0x008ca350, ai_task_message_pool_destructor(Ptr)),
        entry!(0x008ca3c0, ai_task_message_pool_base_destructor(Ptr)),
        entry!(0x008ca3e0, ai_task_message_pool_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x008ca410, ai_task_message_pool_array_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x008ca440, common_message_queue_constructor(Ptr, Ptr) -> Ptr),
        entry!(0x008ca480, common_message_queue_destructor(Ptr)),
        entry!(0x008ca500, message_queue_destructor(Ptr)),
        entry!(0x008ca520, message_queue_base_destructor(Ptr)),
        entry!(0x008ca540, message_queue_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x008ca570, mobile_message_queue_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x008ca5a0, common_ll_message_queue_try_push(Ptr, Ptr) -> bool),
        entry!(0x008ca620, mobile_message_queue_do_try_pop(Ptr, Ptr) -> bool),
        entry!(0x008ca680, ai_task_message_pool_constructor(Ptr) -> Ptr),
        entry!(
            0x008ca710,
            common_message_queue_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(0x008ca740, message_queue_constructor(Ptr) -> Ptr),
        entry!(0x008ca770, message_pool_release_node(Ptr, Ptr)),
        entry!(0x008ca7a0, message_queue_base_constructor(Ptr) -> Ptr),
        entry!(0x008ca7c0, message_pool_allocate(Ptr, Ptr, Ptr) -> bool),
        entry!(0x008ca800, message_pool_base_constructor(Ptr) -> Ptr),
        entry!(0x008ca830, message_pool_link_nodes(Ptr, u32, Ptr)),
        entry!(0x008ca8d0, message_pool_push_free_node(Ptr, Ptr)),
        entry!(0x008ca940, message_pool_pop_free_node(Ptr, Ptr) -> bool),
        entry!(0x008ca9e0, message_construct_copy(Ptr, Ptr) -> Ptr),
    ]
}

// Translated from 008c8130 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MobileObjectTaskletData::Process` (Xbox PDB): drains the message queue
/// (queue vtable slot +0x10 with a message buffer), dispatching each message
/// with the tasklet's `eType`. When the queue runs dry it marks the tasklet
/// as yielding (`006ea330`); when a message ends the run (an end message) it
/// returns without doing so.
pub fn mobile_object_tasklet_data_process(e: &mut Engine, this: Ptr<MobileObjectTaskletData>) {
    e.with_stack(12, |e, buffer| {
        let msg: Ptr<MobileObjectMessage> = buffer.cast();
        fn_008c8190(e, msg);
        loop {
            if !e.vcall(this.addr() + 0x1c, 0x10, &args![msg]).bool() {
                e.call(TASKLET_SET_YIELDING, &args![this]);
                return;
            }
            let end_arg = e.get(this, MobileObjectTaskletData::eType);
            if !fn_008c81c0(e, this, msg, end_arg) {
                return;
            }
        }
    })
}

// Translated from 008c8190 (decompiled, FalloutNV.exe 1.4.0.525)
/// Initializes a `MobileObjectMessage`: type 2 (end), count 0, no mob.
/// Returns `msg`.
pub fn fn_008c8190(e: &mut Engine, msg: Ptr<MobileObjectMessage>) -> Ptr<MobileObjectMessage> {
    e.set(msg, MobileObjectMessage::eMsgType, 2);
    e.set(msg, MobileObjectMessage::uiCount, 0);
    e.set(msg, MobileObjectMessage::pMob, Ptr::NULL);
    msg
}

// Translated from 008c81c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Dispatches one `MobileObjectMessage`: a count message stores the count in
/// `uiHighActorCount` and calls `HandleCountMessage` (slot +0x10) with it; a
/// mob message calls `HandleMobMessage` (+0x14) with the mob; an end message
/// calls `HandleEndMessage` (+0x18) with `end_arg` and returns false. Any
/// other type does nothing. Returns true to keep draining the queue.
pub fn fn_008c81c0(
    e: &mut Engine,
    this: Ptr<MobileObjectTaskletData>,
    msg: Ptr<MobileObjectMessage>,
    end_arg: u32,
) -> bool {
    match e.get(msg, MobileObjectMessage::eMsgType) {
        0 => {
            let count = e.get(msg, MobileObjectMessage::uiCount);
            e.set(this, MobileObjectTaskletData::uiHighActorCount, count);
            e.vcall(this.addr(), 0x10, &args![count]);
            true
        }
        1 => {
            let mob = e.get(msg, MobileObjectMessage::pMob);
            e.vcall(this.addr(), 0x14, &args![mob]);
            true
        }
        2 => {
            e.vcall(this.addr(), 0x18, &args![end_arg]);
            false
        }
        _ => true,
    }
}

// Translated from 008c8240 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MobileObjectTaskletData::RunToCompletion` (Xbox PDB): stores `end_arg`
/// as `eType`, then drains the queue (queue vtable slot +0x0C, `Process`
/// uses +0x10), dispatching until the queue is empty or an end message
/// arrives.
pub fn mobile_object_tasklet_data_run_to_completion(
    e: &mut Engine,
    this: Ptr<MobileObjectTaskletData>,
    end_arg: u32,
) {
    e.set(this, MobileObjectTaskletData::eType, end_arg);
    e.with_stack(12, |e, buffer| {
        let msg: Ptr<MobileObjectMessage> = buffer.cast();
        fn_008c8190(e, msg);
        loop {
            if !e.vcall(this.addr() + 0x1c, 0xc, &args![msg]).bool() {
                return;
            }
            if !fn_008c81c0(e, this, msg, end_arg) {
                return;
            }
        }
    })
}

// Translated from 008c82a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `DetectionTaskData` constructor: the base constructor, the Detection
/// vtable, then `fNumber` and `iUpdated` cleared. `shared` is the manager's
/// object (+0x174). Returns `this`.
pub fn fn_008c82a0(
    e: &mut Engine,
    this: Ptr<DetectionTaskData>,
    shared: u32,
) -> Ptr<DetectionTaskData> {
    fn_008c82e0(e, this.cast(), shared);
    e.mem.set_u32(this.addr(), VTABLE_DETECTION_TASK_DATA);
    e.set(this, DetectionTaskData::fNumber, 0.0);
    e.set(this, DetectionTaskData::iUpdated, 0);
    this
}

// Translated from 008c82e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MobileObjectTaskletData` constructor: the `BSXenonTaskletData` base
/// constructor, this class's vtable, an empty mob list, the message queue
/// constructed at +0x1C, and `uiHighActorCount` / `eType` cleared. The
/// compiler's exception frame is not translated. Returns `this`.
pub fn fn_008c82e0(
    e: &mut Engine,
    this: Ptr<MobileObjectTaskletData>,
    shared: u32,
) -> Ptr<MobileObjectTaskletData> {
    e.call(TASKLET_BASE_CONSTRUCTOR, &args![this]);
    e.mem
        .set_u32(this.addr(), VTABLE_MOBILE_OBJECT_TASKLET_DATA);
    e.set(this, MobileObjectTaskletData::pmoblist, Ptr::NULL);
    e.call(
        MESSAGE_QUEUE_CONSTRUCTOR,
        &args![this.addr() + 0x1c, shared],
    );
    e.set(this, MobileObjectTaskletData::uiHighActorCount, 0);
    e.set(this, MobileObjectTaskletData::eType, 0);
    this
}

// Translated from 008c8370 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MobileObjectTaskletData`'s scalar deleting destructor (vtable slot 0 of
/// `01085748`; the map has no name): runs the destructor body `83a0` and,
/// when bit 0 of `flags` is set, `operator delete`. Returns `this`.
pub fn fn_008c8370(
    e: &mut Engine,
    this: Ptr<MobileObjectTaskletData>,
    flags: u32,
) -> Ptr<MobileObjectTaskletData> {
    fn_008c83a0(e, this);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 008c83a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor body shared by the tasklet classes: destroys the message queue
/// at +0x1C, then the `BSXenonTaskletData` base. The compiler's exception
/// frame is not translated.
pub fn fn_008c83a0(e: &mut Engine, this: Ptr<MobileObjectTaskletData>) {
    e.call(MESSAGE_QUEUE_DESTRUCTOR, &args![this.addr() + 0x1c]);
    e.call(TASKLET_BASE_DESTRUCTOR, &args![this]);
}

// Translated from 008c8400 (decompiled, FalloutNV.exe 1.4.0.525)
/// `DetectionTaskData::_scalar_deleting_destructor_` (Xbox PDB): the
/// destructor body `fn_008c8430`, then `operator delete` when bit 0 of
/// `flags` is set.
pub fn detection_task_data_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<DetectionTaskData>,
    flags: u32,
) -> Ptr<DetectionTaskData> {
    fn_008c8430(e, this);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 008c8430 (decompiled, FalloutNV.exe 1.4.0.525)
/// `DetectionTaskData` destructor body: resets the vtable to the Detection
/// one, then runs `83a0`.
pub fn fn_008c8430(e: &mut Engine, this: Ptr<DetectionTaskData>) {
    e.mem.set_u32(this.addr(), VTABLE_DETECTION_TASK_DATA);
    fn_008c83a0(e, this.cast());
}

// Translated from 008c8450 (decompiled, FalloutNV.exe 1.4.0.525)
/// `DetectionTaskData::HandleCountMessage` (Xbox PDB): clears `fNumber` and
/// `iUpdated` and enters the critical sections `011dfc04` and `011dfbe8`
/// (the count it is given is not used).
pub fn detection_task_data_handle_count_message(
    e: &mut Engine,
    this: Ptr<DetectionTaskData>,
    _unused_1: u32,
) {
    e.set(this, DetectionTaskData::fNumber, 0.0);
    e.set(this, DetectionTaskData::iUpdated, 0);
    e.call(
        CRITICAL_SECTION_ENTER,
        &args![CRITICAL_SECTION_011DFC04, 0u32],
    );
    e.call(
        CRITICAL_SECTION_ENTER,
        &args![CRITICAL_SECTION_011DFBE8, 0u32],
    );
}

// Translated from 008c8490 (decompiled, FalloutNV.exe 1.4.0.525)
/// `DetectionTaskData::HandleEndMessage` (Xbox PDB): lets the player update
/// its auto-aim actor (`PlayerCharacter::UpdateAutoAimActor`, `this` = the
/// player), then leaves the critical sections `011dfc04` and `011dfbe8`
/// (the end argument is not used).
pub fn detection_task_data_handle_end_message(
    e: &mut Engine,
    _this: Ptr<DetectionTaskData>,
    _unused_1: u32,
) {
    let player = player_character(e);
    e.call(0x0096_4260, &args![player]);
    e.call(CRITICAL_SECTION_LEAVE, &args![CRITICAL_SECTION_011DFC04]);
    e.call(CRITICAL_SECTION_LEAVE, &args![CRITICAL_SECTION_011DFBE8]);
}

// Translated from 008c84c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `DetectionTaskData::HandleMobMessage` (Xbox PDB): the per-actor detection
/// step.
///
/// First it draws a random delay: the float at +0xC of `011f6394` (read with
/// `0084d030`) times `uiHighActorCount`, capped at `0101712c`, then a
/// uniform random in `[0, that]`. If the mob is an actor that is flagged
/// `bProcessMe`, is not hidden by its extra data (`004181e0` +0x30, slot
/// +0x20), is not the player and passes `0047c850`:
/// - when its slot +0x2B4 value is not negative, or `00437bd0` or slot
///   +0x22C(0) say so, the "idle" path runs: a negative slot +0x2B4 value
///   removes it from the player's perceived actors (`00967350`), and slot
///   +0x2B8 is called;
/// - otherwise, if the fader alpha is not positive, slot +0x2B0 gets the
///   random delay and, when its process (slot +0x70) accepts the mob, the
///   detection level against the player (`008a0d10`) and the attack/combat
///   tests (`008b06d0`, `008bc700`, the extra-data checks `00705cf0` /
///   `00825c00`) decide whether `00966f20` is told the mob is hostile, and
///   the mob is inserted into the close-to-player list when nearer than the
///   `011ccf94` setting, or dropped from the perceived actors.
///
/// Any actor, even one that is skipped, adds one to `iUpdated`.
pub fn detection_task_data_handle_mob_message(
    e: &mut Engine,
    this: Ptr<DetectionTaskData>,
    mob: Ptr,
) {
    let count = e.get(this, DetectionTaskData::uiHighActorCount);
    let scale = e
        .call(0x0084_d030, &args![TASK_SCALE_OBJECT_011F6394])
        .f32();
    let mut delay = (scale as f64 * count as f64) as f32;
    let delay_cap: f32 = e.global(DELAY_CAP);
    delay = e.call(0x0040_ebd0, &args![delay, delay_cap]).f32();
    delay = e.call(0x0047_6b70, &args![0.0f32, delay]).f32();

    if mob.is_null() || !is_actor(e, mob) {
        return;
    }
    let actor = mob;
    let player = player_character(e);
    if e.mem.u8(actor.addr() + ACTOR_PROCESS_ME) != 0 {
        // The extra-data list's embedded object at +0x30 (slot +0x20).
        let extra_data = e.call(0x0041_81e0, &args![actor]).u32() + 0x30;
        if !e.vcall(extra_data, 0x20, &[]).bool() {
            let distance = e.call(0x0057_23b0, &args![actor, player, 0u32, 0u32]).f32();
            let object_011de45c = e.global::<u32>(OBJECT_011DE45C);
            if actor != player && !e.call(0x0047_c850, &args![object_011de45c]).bool() {
                let zero: f64 = e.global(ZERO);
                // x87 compare: "not below zero" is also true for NaN.
                let value = e.vcall(actor.addr(), 0x2b4, &[]).f64();
                let mut idle = value.partial_cmp(&zero) != Some(std::cmp::Ordering::Less);
                idle = idle || e.call(0x0043_7bd0, &args![actor]).bool();
                idle = idle || e.vcall(actor.addr(), 0x22c, &args![0u32]).bool();
                if idle {
                    if e.vcall(actor.addr(), 0x2b4, &[]).f64() < zero {
                        e.call(0x0096_7350, &args![player, actor]);
                    }
                    e.vcall(actor.addr(), 0x2b8, &[]);
                } else {
                    let fader = e.global::<u32>(FADER_MANAGER);
                    let alpha = e.call(0x0070_14e0, &args![fader, 1u32]).f64();
                    if alpha <= zero {
                        detect_against_player(e, actor, player, distance, delay);
                    }
                }
            }
        }
    }
    let updated = e.get(this, DetectionTaskData::iUpdated);
    e.set(this, DetectionTaskData::iUpdated, updated.wrapping_add(1));
}

/// The part of `DetectionTaskData::HandleMobMessage` that runs when the
/// fader alpha is not positive: tells the actor the delay (slot +0x2B0),
/// asks its process whether it accepts the mob (slot +0x70), computes the
/// detection level against the player and reports it to the player.
fn detect_against_player(e: &mut Engine, actor: Ptr, player: Ptr, distance: f32, delay: f32) {
    e.vcall(actor.addr(), 0x2b0, &args![delay]);
    let process = e.call(GET_PROCESS, &args![actor]).u32();
    if !e.vcall(process, 0x70, &args![actor]).bool() {
        return;
    }
    // Locals the game keeps on its stack: +0 = detected by sound/proximity
    // flag byte, +1 = second flag byte, +4 = the should-attack result.
    e.with_stack(8, |e, locals| {
        let flag_a = locals.addr();
        let flag_b = locals.addr() + 1;
        let attack_state = locals.addr() + 4;
        e.mem.set_u8(flag_a, 0);
        let sneaking = e.call(0x0049_3bb0, &args![actor]).u8();
        e.mem.set_u8(flag_b, 0);
        let level = e
            .call(
                0x008a_0d10,
                &args![player, 1u32, actor, flag_a, sneaking, 0u32, 0u32, flag_b],
            )
            .i32();
        let mut hostile = false;
        e.mem.set_u32(attack_state, 0);
        let attack_args = args![actor, player, 0u32, attack_state, 0u32];
        if e.call(0x008b_06d0, &attack_args).bool()
            || e.call(0x008b_c700, &args![actor, player]).bool()
        {
            hostile = true;
        } else if e.mem.u32(attack_state) <= 1 {
            let extra = e.call(0x0041_81e0, &args![actor]).u32() + 0x90;
            if e.call(0x0070_5cf0, &args![extra]).bool() {
                let extra = e.call(0x0041_81e0, &args![actor]).u32() + 0x90;
                if e.call(0x0082_5c00, &args![extra]).u32() != 0 {
                    hostile = true;
                }
            }
        }
        let near_setting = e.call(0x0040_3e20, &args![SETTING_011CCF94]).u32();
        if e.mem.f32(near_setting) > distance {
            e.call(0x0096_e3c0, &args![PROCESS_LISTS, actor]);
        }
        let level_setting = e.call(0x0040_3e20, &args![SETTING_011D13C8]).u32();
        if e.mem.f32(level_setting) as f64 <= level as f64 {
            e.call(0x0096_6f20, &args![player, actor, hostile, 0u32]);
        } else if e.mem.u8(flag_b) != 0 || e.mem.u8(flag_a) != 0 {
            e.call(0x0096_6f20, &args![player, actor, hostile, 1u32]);
        } else {
            e.call(0x0096_7350, &args![player, actor]);
        }
    });
}

// Translated from 008c8830 (decompiled, FalloutNV.exe 1.4.0.525)
/// `AnimationTaskData` constructor: the base constructor and the Animation
/// vtable. `shared` is the manager's object (+0x174). Returns `this`.
pub fn fn_008c8830(
    e: &mut Engine,
    this: Ptr<AnimationTaskData>,
    shared: u32,
) -> Ptr<AnimationTaskData> {
    fn_008c82e0(e, this.cast(), shared);
    e.mem.set_u32(this.addr(), VTABLE_ANIMATION_TASK_DATA);
    this
}

// Translated from 008c8860 (decompiled, FalloutNV.exe 1.4.0.525)
/// `AnimationTaskData::_scalar_deleting_destructor_` (Xbox PDB): the
/// destructor body `fn_008c8890`, then `operator delete` when bit 0 of
/// `flags` is set.
pub fn animation_task_data_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<AnimationTaskData>,
    flags: u32,
) -> Ptr<AnimationTaskData> {
    fn_008c8890(e, this);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 008c8890 (decompiled, FalloutNV.exe 1.4.0.525)
/// `AnimationTaskData` destructor body: resets the vtable to the Animation
/// one, then runs `83a0`.
pub fn fn_008c8890(e: &mut Engine, this: Ptr<AnimationTaskData>) {
    e.mem.set_u32(this.addr(), VTABLE_ANIMATION_TASK_DATA);
    fn_008c83a0(e, this.cast());
}

// Translated from 008c88b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `AnimationTaskData::HandleCountMessage` (Xbox PDB): enters the critical
/// sections `011dfc38` and `011dfc04` (the count is not used).
pub fn animation_task_data_handle_count_message(
    e: &mut Engine,
    _this: Ptr<AnimationTaskData>,
    _unused_1: u32,
) {
    e.call(
        CRITICAL_SECTION_ENTER,
        &args![CRITICAL_SECTION_011DFC38, 0u32],
    );
    e.call(
        CRITICAL_SECTION_ENTER,
        &args![CRITICAL_SECTION_011DFC04, 0u32],
    );
}

// Translated from 008c88e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `AnimationTaskData::HandleEndMessage` (Xbox PDB): leaves the critical
/// sections `011dfc04` and `011dfc38` (the end argument is not used).
pub fn animation_task_data_handle_end_message(
    e: &mut Engine,
    _this: Ptr<AnimationTaskData>,
    _unused_1: u32,
) {
    e.call(CRITICAL_SECTION_LEAVE, &args![CRITICAL_SECTION_011DFC04]);
    e.call(CRITICAL_SECTION_LEAVE, &args![CRITICAL_SECTION_011DFC38]);
}

// Translated from 008c8910 (decompiled, FalloutNV.exe 1.4.0.525)
/// `AnimationTaskData::HandleMobMessage` (Xbox PDB): for an actor that is
/// flagged `bProcessMe`, calls slot +0x268 with 0.0; for a non-null mob
/// that is not an actor it calls slot +0x268(0.0) when `00574900` answers
/// true. A null mob does nothing.
pub fn animation_task_data_handle_mob_message(
    e: &mut Engine,
    _this: Ptr<AnimationTaskData>,
    mob: Ptr,
) {
    if !mob.is_null() && is_actor(e, mob) {
        if e.mem.u8(mob.addr() + ACTOR_PROCESS_ME) != 0 {
            e.vcall(mob.addr(), 0x268, &args![0.0f32]);
        }
    } else if !mob.is_null() && e.call(0x0057_4900, &args![mob]).bool() {
        e.vcall(mob.addr(), 0x268, &args![0.0f32]);
    }
}

// Translated from 008c89a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ActorUpdateTaskData` constructor: the base constructor, the ActorUpdate
/// vtable and `iUpdated` cleared. `shared` is the manager's object (+0x174).
/// Returns `this`.
pub fn fn_008c89a0(
    e: &mut Engine,
    this: Ptr<ActorUpdateTaskData>,
    shared: u32,
) -> Ptr<ActorUpdateTaskData> {
    fn_008c82e0(e, this.cast(), shared);
    e.mem.set_u32(this.addr(), VTABLE_ACTOR_UPDATE_TASK_DATA);
    e.set(this, ActorUpdateTaskData::iUpdated, 0);
    this
}

// Translated from 008c89d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ActorUpdateTaskData::_scalar_deleting_destructor_` (Xbox PDB): the
/// destructor body `fn_008c8a00`, then `operator delete` when bit 0 of
/// `flags` is set.
pub fn actor_update_task_data_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<ActorUpdateTaskData>,
    flags: u32,
) -> Ptr<ActorUpdateTaskData> {
    fn_008c8a00(e, this);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 008c8a00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ActorUpdateTaskData` destructor body: resets the vtable to the
/// ActorUpdate one, then runs `83a0`.
pub fn fn_008c8a00(e: &mut Engine, this: Ptr<ActorUpdateTaskData>) {
    e.mem.set_u32(this.addr(), VTABLE_ACTOR_UPDATE_TASK_DATA);
    fn_008c83a0(e, this.cast());
}

// Translated from 008c8a20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ActorUpdateTaskData::HandleCountMessage` (Xbox PDB): clears `iUpdated`
/// and enters the critical section `011dfc38` (the count is not used).
pub fn actor_update_task_data_handle_count_message(
    e: &mut Engine,
    this: Ptr<ActorUpdateTaskData>,
    _unused_1: u32,
) {
    e.set(this, ActorUpdateTaskData::iUpdated, 0);
    e.call(
        CRITICAL_SECTION_ENTER,
        &args![CRITICAL_SECTION_011DFC38, 0u32],
    );
}

// Translated from 008c8a50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ActorUpdateTaskData::HandleEndMessage` (Xbox PDB): leaves the critical
/// section `011dfc38` (the end argument is not used).
pub fn actor_update_task_data_handle_end_message(
    e: &mut Engine,
    _this: Ptr<ActorUpdateTaskData>,
    _unused_1: u32,
) {
    e.call(CRITICAL_SECTION_LEAVE, &args![CRITICAL_SECTION_011DFC38]);
}

// Translated from 008c8a70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ActorUpdateTaskData::HandleMobMessage` (Xbox PDB): for an actor, calls
/// slot +0x2F8(0.0) when it is flagged `bProcessMe`, and counts it in
/// `iUpdated` either way. A null or non-actor mob does nothing.
pub fn actor_update_task_data_handle_mob_message(
    e: &mut Engine,
    this: Ptr<ActorUpdateTaskData>,
    mob: Ptr,
) {
    if mob.is_null() || !is_actor(e, mob) {
        return;
    }
    if e.mem.u8(mob.addr() + ACTOR_PROCESS_ME) != 0 {
        e.vcall(mob.addr(), 0x2f8, &args![0.0f32]);
    }
    let updated = e.get(this, ActorUpdateTaskData::iUpdated);
    e.set(this, ActorUpdateTaskData::iUpdated, updated.wrapping_add(1));
}

// Translated from 008c8ae0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PackageUpdateTaskData` constructor: the base constructor, the
/// PackageUpdate vtable and its two counters cleared. `shared` is the
/// manager's object (+0x174). Returns `this`.
pub fn fn_008c8ae0(
    e: &mut Engine,
    this: Ptr<PackageUpdateTaskData>,
    shared: u32,
) -> Ptr<PackageUpdateTaskData> {
    fn_008c82e0(e, this.cast(), shared);
    e.mem.set_u32(this.addr(), VTABLE_PACKAGE_UPDATE_TASK_DATA);
    e.set(this, PackageUpdateTaskData::iPackageUpdated, 0);
    e.set(this, PackageUpdateTaskData::iActorsProcessed, 0);
    this
}

// Translated from 008c8b20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PackageUpdateTaskData::_scalar_deleting_destructor_` (Xbox PDB): the
/// destructor body `fn_008c8b50`, then `operator delete` when bit 0 of
/// `flags` is set.
pub fn package_update_task_data_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<PackageUpdateTaskData>,
    flags: u32,
) -> Ptr<PackageUpdateTaskData> {
    fn_008c8b50(e, this);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 008c8b50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PackageUpdateTaskData` destructor body: resets the vtable to the
/// PackageUpdate one, then runs `83a0`.
pub fn fn_008c8b50(e: &mut Engine, this: Ptr<PackageUpdateTaskData>) {
    e.mem.set_u32(this.addr(), VTABLE_PACKAGE_UPDATE_TASK_DATA);
    fn_008c83a0(e, this.cast());
}

// Translated from 008c8b70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PackageUpdateTaskData::HandleCountMessage` (Xbox PDB): clears the two
/// counters, clears the byte at `011e01dd` (`fn_008c8bd0`), clears the byte
/// at +0x5F8 of the player (`fn_008c8bb0`) and enters the critical section
/// `011dfbe8` (the count is not used).
pub fn package_update_task_data_handle_count_message(
    e: &mut Engine,
    this: Ptr<PackageUpdateTaskData>,
    _unused_1: u32,
) {
    e.set(this, PackageUpdateTaskData::iPackageUpdated, 0);
    e.set(this, PackageUpdateTaskData::iActorsProcessed, 0);
    fn_008c8bd0(e);
    let player = player_character(e);
    fn_008c8bb0(e, player, 0);
    e.call(
        CRITICAL_SECTION_ENTER,
        &args![CRITICAL_SECTION_011DFBE8, 0u32],
    );
}

// Translated from 008c8bb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores `value` in the byte at +0x5F8 of the object (the player character
/// at its callers).
pub fn fn_008c8bb0(e: &mut Engine, this: Ptr, value: u8) {
    e.mem.set_u8(this.addr() + 0x5f8, value);
}

// Translated from 008c8bd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Clears the global byte at `011e01dd`.
pub fn fn_008c8bd0(e: &mut Engine) {
    e.set_global(RESET_FLAG_011E01DD, 0u8);
}

// Translated from 008c8be0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PackageUpdateTaskData::HandleMobMessage` (Xbox PDB): runs the package
/// update for one mob.
///
/// Skipped unless the mob passes `00576d30`, `00440d80` and `00440da0` with
/// false, has a process (`008d8520`) and its current process type
/// (`00931850`) is 0. When the mob is an actor that is not flagged
/// `bProcessMe` nothing more happens (and nothing is counted). If it is an
/// actor for which `008b0520` answers true, its process (slot +0x24) is
/// handed the actor, `iActorsProcessed` is incremented and the process's
/// byte at +0x2C7 is set to 1; otherwise the mob's slot +0x25C is called
/// with 0.0 and `iPackageUpdated` is incremented. A sleeping or resting
/// player restores the actor's health and fatigue by the `010162c0` amount.
/// A mob whose process type is not 0 afterwards clears `pmoblist`. Finally,
/// when the actor was not handed to its process, the entries of a list
/// (at +0xC of the object `00422700` returns for the mob's `005d43c0`
/// object) that are neither the player nor accepted by `008bc860` for the
/// actor are passed to `00422690` (named `ExtraDataList::RemoveFollower` in
/// the Xbox PDB), through two temporary lists; and when `008256d0` answers
/// true for that list, `00422720` (`ExtraDataList::RemoveFollowerExtra`) runs.
///
/// The compiler's exception frame is not translated.
pub fn package_update_task_data_handle_mob_message(
    e: &mut Engine,
    this: Ptr<PackageUpdateTaskData>,
    mob: Ptr,
) {
    if mob.is_null()
        || e.call(0x0057_6d30, &args![mob]).bool()
        || e.call(0x0044_0d80, &args![mob]).bool()
        || e.call(0x0044_0da0, &args![mob]).bool()
        || e.call(GET_PROCESS, &args![mob]).u32() == 0
        || e.call(GET_CURRENT_PROCESS_TYPE, &args![mob]).u32() != 0
    {
        return;
    }
    let mut actor = Ptr::NULL;
    let mut not_handed_over = true;
    if is_actor(e, mob) {
        actor = mob;
    }
    if !actor.is_null() && e.mem.u8(actor.addr() + ACTOR_PROCESS_ME) == 0 {
        return;
    }
    if !actor.is_null() && e.call(0x008b_0520, &args![actor]).bool() {
        let process = e.call(GET_PROCESS, &args![actor]).u32();
        e.vcall(process, 0x24, &args![actor, 0u32]);
        not_handed_over = false;
        let n = e.get(this, PackageUpdateTaskData::iActorsProcessed);
        e.set(
            this,
            PackageUpdateTaskData::iActorsProcessed,
            n.wrapping_add(1),
        );
        let process = e.call(GET_PROCESS, &args![actor]).u32();
        fn_008c8fd0(e, Ptr::new(process), 1);
    } else {
        e.vcall(mob.addr(), 0x25c, &args![0.0f32]);
        let n = e.get(this, PackageUpdateTaskData::iPackageUpdated);
        e.set(
            this,
            PackageUpdateTaskData::iPackageUpdated,
            n.wrapping_add(1),
        );
    }

    let player = player_character(e);
    if !actor.is_null() && e.call(0x0094_df60, &args![player]).bool() {
        let amount: f32 = e.global(RESTORE_AMOUNT);
        e.call(0x0088_b510, &args![actor, amount]);
        e.call(0x0088_b5a0, &args![actor, amount]);
    }

    if e.call(GET_CURRENT_PROCESS_TYPE, &args![mob]).u32() != 0 {
        e.set(this, PackageUpdateTaskData::pmoblist, Ptr::NULL);
    }

    if not_handed_over {
        reconcile_followers(e, mob, actor, player);
    }
}

/// The follower clean-up at the end of
/// `PackageUpdateTaskData::HandleMobMessage`: `005d43c0` on the mob gives an
/// object, `00422700` on that an owner whose list (at +0xC) is copied into
/// one temporary list; the entries that are neither null, the player nor
/// accepted by `008bc860` for `actor` are collected in a second, and each of
/// those is passed to `00422690` on the mob's `005d43c0` object. Both
/// temporaries are then cleared and freed.
fn reconcile_followers(e: &mut Engine, mob: Ptr, actor: Ptr, player: Ptr) {
    let extra = e.call(0x005d_43c0, &args![mob]).u32();
    let owner = if extra != 0 {
        let extra = e.call(0x005d_43c0, &args![mob]).u32();
        e.call(0x0042_2700, &args![extra]).u32()
    } else {
        0
    };
    if owner == 0 {
        return;
    }
    let first_list = new_list(e);
    let second_list = new_list(e);

    // Copy the owner's list into `second_list`.
    let mut node = e.mem.u32(owner + 0xc);
    while node != 0 {
        let item_slot = e.call(0x0068_15c0, &args![node]).u32();
        if e.mem.u32(item_slot) == 0 {
            break;
        }
        let item_slot = e.call(0x0068_15c0, &args![node]).u32();
        e.call(0x0090_5820, &args![second_list, item_slot]);
        node = e.call(0x0072_6070, &args![node]).u32();
    }

    // Collect into `first_list` the entries that are not the player and
    // cannot follow `actor`.
    let mut node = second_list;
    while node != 0 {
        let item_slot = e.call(0x0068_15c0, &args![node]).u32();
        if e.mem.u32(item_slot) == 0 {
            break;
        }
        let item_slot = e.call(0x0068_15c0, &args![node]).u32();
        let item = e.mem.u32(item_slot);
        if item != 0 && item != player.addr() && !e.call(0x008b_c860, &args![item, actor]).bool() {
            e.with_stack(4, |e, slot| {
                e.mem.set_u32(slot.addr(), item);
                e.call(0x005a_e3d0, &args![first_list, slot]);
            });
        }
        node = e.call(0x0072_6070, &args![node]).u32();
    }

    // Remove each collected entry from the owner.
    let mut node = first_list;
    while node != 0 {
        let item_slot = e.call(0x0068_15c0, &args![node]).u32();
        if e.mem.u32(item_slot) == 0 {
            break;
        }
        let item_slot = e.call(0x0068_15c0, &args![node]).u32();
        let item = e.mem.u32(item_slot);
        let extra = e.call(0x005d_43c0, &args![mob]).u32();
        e.call(0x0042_2690, &args![extra, item]);
        node = e.call(0x0072_6070, &args![node]).u32();
    }

    for list in [first_list, second_list] {
        e.call(0x0047_0470, &args![list]);
        if list != 0 {
            e.call(0x0047_02f0, &args![list, 1u32]);
        }
    }
    let owner_list = e.mem.u32(owner + 0xc);
    if e.call(0x0082_56d0, &args![owner_list]).bool() {
        let extra = e.call(0x005d_43c0, &args![mob]).u32();
        e.call(0x0042_2720, &args![extra]);
    }
}

/// `new BSSimpleList` as the handler writes it: 8 bytes from `operator new`
/// constructed with `0096a2d0` (null stays null).
fn new_list(e: &mut Engine) -> u32 {
    let list = e.call(OPERATOR_NEW, &args![8u32]).u32();
    if list == 0 {
        0
    } else {
        e.call(0x0096_a2d0, &args![list]).u32()
    }
}

// Translated from 008c8fd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores `value` in the byte at +0x2C7 of the object (a process, at the
/// callers).
pub fn fn_008c8fd0(e: &mut Engine, this: Ptr, value: u8) {
    e.mem.set_u8(this.addr() + 0x2c7, value);
}

// Translated from 008c8ff0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PackageUpdateTaskData::HandleEndMessage` (Xbox PDB): passes the float at
/// +0xC of `011f6394` (`0084d030`) to the process lists (`00977130`), then
/// leaves the critical section `011dfbe8` (the end argument is not used).
pub fn package_update_task_data_handle_end_message(
    e: &mut Engine,
    _this: Ptr<PackageUpdateTaskData>,
    _unused_1: u32,
) {
    let scale = e
        .call(0x0084_d030, &args![TASK_SCALE_OBJECT_011F6394])
        .f32();
    e.call(0x0097_7130, &args![PROCESS_LISTS, scale]);
    e.call(CRITICAL_SECTION_LEAVE, &args![CRITICAL_SECTION_011DFBE8]);
}

// Translated from 008c9020 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ActorsScriptTaskData` constructor: the base constructor, the
/// ActorsScript vtable and its counter cleared. `shared` is the manager's
/// object (+0x174). Returns `this`.
pub fn fn_008c9020(
    e: &mut Engine,
    this: Ptr<ActorsScriptTaskData>,
    shared: u32,
) -> Ptr<ActorsScriptTaskData> {
    fn_008c82e0(e, this.cast(), shared);
    e.mem.set_u32(this.addr(), VTABLE_ACTORS_SCRIPT_TASK_DATA);
    e.set(this, ActorsScriptTaskData::iScriptsRun, 0);
    this
}

// Translated from 008c9050 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ActorsScriptTaskData::_scalar_deleting_destructor_` (Xbox PDB): the
/// destructor body `fn_008c9080`, then `operator delete` when bit 0 of
/// `flags` is set.
pub fn actors_script_task_data_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<ActorsScriptTaskData>,
    flags: u32,
) -> Ptr<ActorsScriptTaskData> {
    fn_008c9080(e, this);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 008c9080 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ActorsScriptTaskData` destructor body: resets the vtable to the
/// ActorsScript one, then runs `83a0`.
pub fn fn_008c9080(e: &mut Engine, this: Ptr<ActorsScriptTaskData>) {
    e.mem.set_u32(this.addr(), VTABLE_ACTORS_SCRIPT_TASK_DATA);
    fn_008c83a0(e, this.cast());
}

// Translated from 008c90a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ActorsScriptTaskData::HandleCountMessage` (Xbox PDB): clears the script
/// counter and enters the critical sections `011dfc38`, `011dfc04` and
/// `011dfbe8` (the count is not used).
pub fn actors_script_task_data_handle_count_message(
    e: &mut Engine,
    this: Ptr<ActorsScriptTaskData>,
    _unused_1: u32,
) {
    e.set(this, ActorsScriptTaskData::iScriptsRun, 0);
    e.call(
        CRITICAL_SECTION_ENTER,
        &args![CRITICAL_SECTION_011DFC38, 0u32],
    );
    e.call(
        CRITICAL_SECTION_ENTER,
        &args![CRITICAL_SECTION_011DFC04, 0u32],
    );
    e.call(
        CRITICAL_SECTION_ENTER,
        &args![CRITICAL_SECTION_011DFBE8, 0u32],
    );
}

// Translated from 008c90e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ActorsScriptTaskData::HandleEndMessage` (Xbox PDB): leaves the critical
/// sections `011dfbe8`, `011dfc04` and `011dfc38` (the end argument is not
/// used).
pub fn actors_script_task_data_handle_end_message(
    e: &mut Engine,
    _this: Ptr<ActorsScriptTaskData>,
    _unused_1: u32,
) {
    e.call(CRITICAL_SECTION_LEAVE, &args![CRITICAL_SECTION_011DFBE8]);
    e.call(CRITICAL_SECTION_LEAVE, &args![CRITICAL_SECTION_011DFC04]);
    e.call(CRITICAL_SECTION_LEAVE, &args![CRITICAL_SECTION_011DFC38]);
}

// Translated from 008c9110 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ActorsScriptTaskData::HandleMobMessage` (Xbox PDB): for an actor, when
/// scripts are being processed (`Script::GetProcessScripts`, `005ac740`),
/// runs the reference's script (`TESObjectREFR::RunScript`, `00565870`) and
/// counts it in `iScriptsRun`.
pub fn actors_script_task_data_handle_mob_message(
    e: &mut Engine,
    this: Ptr<ActorsScriptTaskData>,
    mob: Ptr,
) {
    if mob.is_null() || !is_actor(e, mob) {
        return;
    }
    if e.call(0x005a_c740, &[]).bool() {
        e.call(0x0056_5870, &args![mob]);
        let n = e.get(this, ActorsScriptTaskData::iScriptsRun);
        e.set(this, ActorsScriptTaskData::iScriptsRun, n.wrapping_add(1));
    }
}

// Translated from 008c9170 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MovementTaskData` (Xbox PDB) constructor: constructs the tasklet base
/// with its task type, installs the movement vtable, and clears `iUpdated`.
pub fn movement_task_data_constructor(
    e: &mut Engine,
    this: Ptr<MovementTaskData>,
    task_type: u32,
) -> Ptr<MovementTaskData> {
    fn_008c82e0(e, this.cast(), task_type);
    e.mem.set_u32(this.addr(), VTABLE_MOVEMENT_TASK_DATA);
    e.set(this, MovementTaskData::iUpdated, 0);
    this
}

// Translated from 008c91a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MovementTaskData` (Xbox PDB) scalar deleting destructor. Bit zero of
/// `flags` requests deallocation after destruction.
pub fn movement_task_data_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<MovementTaskData>,
    flags: u32,
) -> Ptr<MovementTaskData> {
    movement_task_data_destructor(e, this);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 008c91d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MovementTaskData` (Xbox PDB) destructor: restores its vtable, then
/// destroys the `MobileObjectTaskletData` base.
pub fn movement_task_data_destructor(e: &mut Engine, this: Ptr<MovementTaskData>) {
    e.mem.set_u32(this.addr(), VTABLE_MOVEMENT_TASK_DATA);
    fn_008c83a0(e, this.cast());
}

// Translated from 008c91f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MovementTaskData::HandleCountMessage` (Xbox PDB): resets the update
/// count and enters the two critical sections in address order.
pub fn movement_task_data_handle_count_message(
    e: &mut Engine,
    this: Ptr<MovementTaskData>,
    _count: u32,
) {
    e.set(this, MovementTaskData::iUpdated, 0);
    e.call(
        CRITICAL_SECTION_ENTER,
        &args![CRITICAL_SECTION_011DFC04, 0u32],
    );
    e.call(
        CRITICAL_SECTION_ENTER,
        &args![CRITICAL_SECTION_011DFBE8, 0u32],
    );
}

// Translated from 008c9220 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MovementTaskData::HandleMobMessage` (Xbox PDB): actor mobs call the
/// actor update slot only when `bProcessMe` is set; other mobs follow the
/// two executable vtable checks. Every message increments `iUpdated`.
pub fn movement_task_data_handle_mob_message(
    e: &mut Engine,
    this: Ptr<MovementTaskData>,
    mob: Ptr,
) {
    if mob.is_null() || !is_actor(e, mob) {
        if e.vcall(mob.addr(), 0x224, &[]).bool() {
            e.call(0x009b_ec10, &args![mob, 0.0f32]);
        } else if e.vcall(mob.addr(), 0x220, &[]).bool() {
            e.call(0x009a_e580, &args![mob, 0.0f32]);
        }
    } else if e.mem.u8(mob.addr() + ACTOR_PROCESS_ME) != 0 {
        let player = player_character(e);
        let sleeping = e.call(0x0094_df60, &args![player]).bool();
        e.vcall(mob.addr(), 0x348, &args![0.0f32, sleeping]);
    }
    let updated = e.get(this, MovementTaskData::iUpdated);
    e.set(this, MovementTaskData::iUpdated, updated.wrapping_add(1));
}

// Translated from 008c9300 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MovementTaskData::HandleEndMessage` (Xbox PDB): leaves the two
/// critical sections, first `011dfbe8`, then `011dfc04`.
pub fn movement_task_data_handle_end_message(
    e: &mut Engine,
    _this: Ptr<MovementTaskData>,
    _end_type: u32,
) {
    e.call(CRITICAL_SECTION_LEAVE, &args![CRITICAL_SECTION_011DFBE8]);
    e.call(CRITICAL_SECTION_LEAVE, &args![CRITICAL_SECTION_011DFC04]);
}

// Translated from 008c9330 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 008c9330 - compiler exception cleanup path
/// Constructs the six embedded tasklet data objects and the thread manager.
// open: compiler SEH cleanup path.
pub fn ai_task_manager_constructor(e: &mut Engine, this: Ptr) -> Ptr {
    fn_008c82a0(e, this.cast(), this.addr() + 0x174);
    fn_008c8830(e, Ptr::new(this.addr() + 0x40), this.addr() + 0x174);
    fn_008c8ae0(e, Ptr::new(this.addr() + 0x78), this.addr() + 0x174);
    fn_008c89a0(e, Ptr::new(this.addr() + 0xbc), this.addr() + 0x174);
    fn_008c9020(e, Ptr::new(this.addr() + 0xf8), this.addr() + 0x174);
    movement_task_data_constructor(e, Ptr::new(this.addr() + 0x138), this.addr() + 0x174);
    e.call(0x008c_a680, &args![this.addr() + 0x174]);
    e.mem.set_u8(this.addr() + 0x10180, 0);
    this
}

// Translated from 008c9430 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 008c9430 - compiler exception cleanup path
/// Destroys the embedded message pool and tasklets in reverse construction
/// order.
// open: compiler SEH cleanup path.
pub fn ai_task_manager_destructor(e: &mut Engine, this: Ptr) {
    ai_task_message_pool_destructor(e, Ptr::new(this.addr() + 0x174));
    movement_task_data_destructor(e, Ptr::new(this.addr() + 0x138));
    fn_008c9080(e, Ptr::new(this.addr() + 0xf8));
    fn_008c8a00(e, Ptr::new(this.addr() + 0xbc));
    fn_008c8b50(e, Ptr::new(this.addr() + 0x78));
    fn_008c8890(e, Ptr::new(this.addr() + 0x40));
    fn_008c8430(e, Ptr::new(this.addr()));
}

// Translated from 008c94e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The manager's per-frame scheduling (`AITaskManager::Process`): creates the
/// worker threads once (processors 1, 3 and 5, when the threading setting
/// `011dfb68` is on), and, when the actor-package global `011f1220` is set
/// and the process list has entries, queues one count message, one mob
/// message per actor and one end message to each tasklet whose setting
/// allows it (Detection `011dfc70` is skipped when set; Animation +
/// ActorUpdate only when `011c3ea4` is 1; Movement `011dfb88` is skipped when
/// set), then either hands the tasklets to the worker ring and wakes the
/// workers or runs them to completion here. The tail passes setting
/// `011d10d4` to `007034c0` and calls `00703490` when `011c3ea4` is 1.
/// The actor list is `ProcessLists + 4` (`00717e50`), its count the word at
/// +0x20 (`005be5c0`) and its entries read with `00968670`. The compiler's
/// exception frames are not translated.
pub fn ai_task_manager_process(e: &mut Engine, this: Ptr) {
    if e.mem.u8(0x011a_34e7) != 0 {
        if setting_byte(e, SETTING_THREADED) != 0 {
            for processor in [1, 3, 5] {
                ai_task_thread_manager_add_thread(e, Ptr::new(THREAD_MANAGER), processor);
            }
        }
        e.mem.set_u8(0x011a_34e7, 0);
    }
    if e.mem.u8(ACTOR_PACKAGES_ACTIVE) != 0 && actor_count(e) > 0 {
        let count = actor_count(e);
        let detection = this;
        let animation = Ptr::new(this.addr() + 0x40);
        let package = Ptr::new(this.addr() + 0x78);
        let actor_update = Ptr::new(this.addr() + 0xbc);
        let script = Ptr::new(this.addr() + 0xf8);
        let movement = Ptr::new(this.addr() + 0x138);

        // One count message to every tasklet that takes part.
        if setting_byte(e, SETTING_NO_DETECTION) == 0 {
            push_count_message(e, detection, count);
        }
        push_count_message(e, package, count);
        if setting_value(e, SETTING_SPLIT_UPDATE) == 1 {
            push_count_message(e, actor_update, count);
            push_count_message(e, animation, count);
        }
        push_count_message(e, script, count);
        if setting_byte(e, SETTING_NO_MOVEMENT) == 0 {
            push_count_message(e, movement, count);
        }

        // One mob message per actor.
        let mut index = 0u32;
        loop {
            if setting_byte(e, SETTING_NO_DETECTION) == 0 {
                push_mob_message(e, detection, index);
            }
            push_mob_message(e, package, index);
            if setting_value(e, SETTING_SPLIT_UPDATE) == 1 {
                push_mob_message(e, actor_update, index);
                push_mob_message(e, animation, index);
            }
            push_mob_message(e, script, index);
            if setting_byte(e, SETTING_NO_MOVEMENT) == 0 {
                push_mob_message(e, movement, index);
            }
            index = index.wrapping_add(1);
            if index >= actor_count(e) {
                break;
            }
        }

        // The end messages.
        if setting_byte(e, SETTING_NO_DETECTION) == 0 {
            ai_task_queue_push_end(e, detection);
        }
        ai_task_queue_push_end(e, package);
        if setting_value(e, SETTING_SPLIT_UPDATE) == 1 {
            ai_task_queue_push_end(e, animation);
            ai_task_queue_push_end(e, actor_update);
        }
        ai_task_queue_push_end(e, script);
        if setting_byte(e, SETTING_NO_MOVEMENT) == 0 {
            ai_task_queue_push_end(e, movement);
        }

        if setting_byte(e, SETTING_THREADED) != 0 {
            // The workers take the tasklets from the ten-slot ring.
            let ring = Ptr::new(TASK_RING);
            if setting_value(e, SETTING_SPLIT_UPDATE) == 1 {
                ai_task_queue_enqueue(e, ring, actor_update.addr());
            }
            ai_task_queue_enqueue(e, ring, package.addr());
            ai_task_queue_enqueue(e, ring, script.addr());
            if setting_byte(e, SETTING_NO_DETECTION) == 0 {
                ai_task_queue_enqueue(e, ring, detection.addr());
            }
            if setting_byte(e, SETTING_NO_MOVEMENT) == 0 {
                ai_task_queue_enqueue(e, ring, movement.addr());
                ai_task_queue_enqueue(e, ring, animation.addr());
            }
            ai_task_thread_manager_release_semaphores(e, Ptr::new(THREAD_MANAGER));
            ai_task_thread_manager_wake_threads(e, Ptr::new(THREAD_MANAGER));
        } else {
            if setting_byte(e, SETTING_NO_DETECTION) == 0 {
                mobile_object_tasklet_data_run_to_completion(e, detection.cast(), 0);
            }
            mobile_object_tasklet_data_run_to_completion(e, package.cast(), 0);
            mobile_object_tasklet_data_run_to_completion(e, script.cast(), 0);
            if setting_value(e, SETTING_SPLIT_UPDATE) == 1 {
                mobile_object_tasklet_data_run_to_completion(e, animation.cast(), 0);
                mobile_object_tasklet_data_run_to_completion(e, actor_update.cast(), 0);
            }
            if setting_byte(e, SETTING_NO_MOVEMENT) == 0 {
                mobile_object_tasklet_data_run_to_completion(e, movement.cast(), 0);
            }
        }
    }
    if setting_value(e, SETTING_SPLIT_UPDATE) == 1 {
        let value = setting_value(e, 0x011d_10d4) as u32;
        e.call(0x0070_34c0, &args![value]);
        e.call(0x0070_3490, &[]);
    }
}

/// The global task ring (`011dfb98`, `AITaskQueue`) and the thread manager
/// (`011dfc24`) used by the scheduling functions.
const TASK_RING: u32 = 0x011d_fb98;
const THREAD_MANAGER: u32 = 0x011d_fc24;
/// Byte global: actor packages are being scheduled this frame.
const ACTOR_PACKAGES_ACTIVE: u32 = 0x011f_1220;
/// Setting bytes read with `00408d60` (a pointer to the setting's value):
/// worker threads on, Detection tasklet off, Movement tasklet off. Setting
/// `011c3ea4` (an int read with `0043d4d0`) is 1 when the Animation and
/// ActorUpdate tasklets run. Their names are not in the exe's symbols.
const SETTING_THREADED: u32 = 0x011d_fb68;
const SETTING_NO_DETECTION: u32 = 0x011d_fc70;
const SETTING_NO_MOVEMENT: u32 = 0x011d_fb88;
const SETTING_SPLIT_UPDATE: u32 = 0x011c_3ea4;

/// The actor list of the process lists: `ProcessLists + 4` (`00717e50`,
/// `this` only).
fn actor_list(e: &mut Engine) -> u32 {
    e.call(0x0071_7e50, &args![PROCESS_LISTS]).u32()
}

/// The list's entry count (`005be5c0` with index 0).
fn actor_count(e: &mut Engine) -> u32 {
    let list = actor_list(e);
    e.call(0x005b_e5c0, &args![list, 0u32]).u32()
}

/// Entry `index` of the actor list (`00968670`).
fn actor_at(e: &mut Engine, index: u32) -> u32 {
    let list = actor_list(e);
    e.call(0x0096_8670, &args![list, index]).u32()
}

/// Builds a count message (`008c9a20`) on the stack and pushes it
/// (`008ca010`) to the tasklet.
fn push_count_message(e: &mut Engine, tasklet: Ptr, count: u32) {
    e.with_stack(12, |e, item| {
        task_queue_item_new(e, item, count);
        ai_task_queue_push(e, tasklet, item);
    });
}

/// Builds a mob message (`008c9a50`) for the actor at `index` and pushes it
/// to the tasklet.
fn push_mob_message(e: &mut Engine, tasklet: Ptr, index: u32) {
    let actor = actor_at(e, index);
    e.with_stack(12, |e, item| {
        task_queue_item_existing(e, item, actor);
        ai_task_queue_push(e, tasklet, item);
    });
}

fn setting_byte(e: &mut Engine, setting: u32) -> u8 {
    let data = e.call(0x0040_8d60, &args![setting]).u32();
    e.mem.u8(data)
}

fn setting_value(e: &mut Engine, setting: u32) -> i32 {
    let data = e.call(0x0043_d4d0, &args![setting]).u32();
    e.mem.i32(data)
}

// Translated from 008c9a20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Initializes a 12-byte task queue item as a new task carrying `value`.
pub fn task_queue_item_new(e: &mut Engine, this: Ptr, value: u32) -> Ptr {
    e.mem.set_u32(this.addr(), 0);
    e.mem.set_u32(this.addr() + 4, value);
    e.mem.set_u32(this.addr() + 8, 0);
    this
}

// Translated from 008c9a50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Initializes a 12-byte task queue item as an existing task carrying
/// `value`; the field at +4 is deliberately left untouched.
pub fn task_queue_item_existing(e: &mut Engine, this: Ptr, value: u32) -> Ptr {
    e.mem.set_u32(this.addr(), 1);
    e.mem.set_u32(this.addr() + 8, value);
    this
}

// Translated from 008c9a80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `AITaskQueue::Enqueue` body (Xbox PDB): appends `value` to the ten-entry
/// PC circular buffer until the recorded count reaches ten.
pub fn ai_task_queue_enqueue(e: &mut Engine, this: Ptr, value: u32) {
    let count = e.mem.i32(this.addr() + 0x40);
    if count < 10 {
        let mut index = e.mem.i32(this.addr() + 0x44).wrapping_add(count);
        if index >= 10 {
            index = index.wrapping_sub(10);
        }
        e.mem.set_u32(this.addr() + 0x18 + index as u32 * 4, value);
        e.mem.set_i32(this.addr() + 0x40, count.wrapping_add(1));
    }
}

// Translated from 008c9c70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Runs the worker loop: wait on its start semaphore, drain the global task
/// ring through `RunToCompletion`, release the completion semaphore, and
/// stop when the kill byte is set.
pub fn ai_task_thread_thread_proc(e: &mut Engine, this: Ptr) -> u32 {
    ai_task_thread_on_startup(e, this);
    loop {
        e.call(0x0044_24e0, &args![this.addr() + 0x30]);
        if e.mem.u8(this.addr() + 0x48) != 0 {
            break;
        }
        loop {
            let tasklet = ai_task_queue_dequeue(e, Ptr::new(0x011d_fb98));
            if tasklet == 0 {
                break;
            }
            let end_arg = e.mem.u32(this.addr() + 0x4c);
            mobile_object_tasklet_data_run_to_completion(e, Ptr::new(tasklet), end_arg);
        }
        e.call(0x0044_2550, &args![this.addr() + 0x3c]);
    }
    ai_task_thread_on_shutdown(e, this);
    0
}

/// The worker thread's Havok memory router (embedded at +0x50) and the
/// 0x16800-byte scratch block (pointer at +0x74) the startup allocates.
const THREAD_ROUTER: u32 = 0x50;
const THREAD_SCRATCH: u32 = 0x74;
const THREAD_SCRATCH_SIZE: u32 = 0x16800;
/// The name string (`01085820`) `threadInit` is given.
const THREAD_NAME: u32 = 0x0108_5820;
/// `hkMemorySystem::getInstance` (`00cbbfc0`), `hkBaseSystem::initThread`
/// (`00cbb930`, cdecl) and `hkBaseSystem::quitThread` (`00cbb990`).
const MEMORY_SYSTEM_GET_INSTANCE: u32 = 0x00cb_bfc0;
const HK_INIT_THREAD: u32 = 0x00cb_b930;
const HK_QUIT_THREAD: u32 = 0x00cb_b990;
/// Accessors of the router: its word +0x10 (`0044edb0`), setters of its
/// words +0xC (`0041fd00`) and +0x18 (`00984f60`), and the call that gives it
/// the scratch block and size (`008c70c0`).
const ROUTER_WORD: u32 = 0x0044_edb0;
const ROUTER_SET_WORD_C: u32 = 0x0041_fd00;
const ROUTER_SET_WORD_18: u32 = 0x0098_4f60;
const ROUTER_SET_BLOCK: u32 = 0x008c_70c0;
/// `TESDataHandler::CreateThreadSafeFiles` (Xbox PDB, `00462ee0`) and the
/// global holding its `this`.
const CREATE_THREAD_SAFE_FILES: u32 = 0x0046_2ee0;
const THREAD_SAFE_FILES_OWNER: u32 = 0x011c_3f2c;

// Translated from 008c9d70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `AITaskThread::OnStartup` (Xbox PDB): sets up the worker's Havok memory
/// router and scratch memory. Calls `hkMemorySystem::threadInit` (slot +0xC
/// of `hkMemorySystem::getInstance`, `00cbbfc0`) with the thread's router at
/// +0x50, a name string (`01085820`) and flags 1, `hkBaseSystem::initThread`
/// (`00cbb930`), copies the router's word +0x10 (`0044edb0`) to its +0xC
/// (`0041fd00`) and +0x18 (`00984f60`), allocates 0x16800 bytes of scratch
/// memory from the object at that word (vtable slot +4) into this+0x74,
/// hands the block to the router (`008c70c0`) and creates the thread-safe
/// files (`00462ee0`, `this` = the global at `011c3f2c`).
pub fn ai_task_thread_on_startup(e: &mut Engine, this: Ptr) {
    let router = this.addr() + THREAD_ROUTER;
    let memory_system = e.call(MEMORY_SYSTEM_GET_INSTANCE, &[]).u32();
    e.vcall(memory_system, 0x0c, &args![router, THREAD_NAME, 1u32]);
    e.call(HK_INIT_THREAD, &args![router]);
    let word = e.call(ROUTER_WORD, &args![router]).u32();
    e.call(ROUTER_SET_WORD_C, &args![router, word]);
    let word = e.call(ROUTER_WORD, &args![router]).u32();
    e.call(ROUTER_SET_WORD_18, &args![router, word]);
    let allocator = e.call(ROUTER_WORD, &args![router]).u32();
    let block = e.vcall(allocator, 0x04, &args![THREAD_SCRATCH_SIZE]).u32();
    e.mem.set_u32(this.addr() + THREAD_SCRATCH, block);
    let router_again = e.call(RETURN_THIS, &args![router]).u32();
    e.call(
        ROUTER_SET_BLOCK,
        &args![router_again, block, THREAD_SCRATCH_SIZE],
    );
    let files = e.global::<u32>(THREAD_SAFE_FILES_OWNER);
    e.call(CREATE_THREAD_SAFE_FILES, &args![files]);
}

// Translated from 008c9e40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `AITaskThread::OnShutdown` (Xbox PDB): frees the scratch memory with the
/// allocator at the router's word +0x10 (vtable slot +8: block, size),
/// `hkBaseSystem::quitThread` (`00cbb990`), `hkMemorySystem::threadQuit`
/// (slot +0x10 with the router and flags 1) and clears the router's block
/// (`008c70c0` with 0 and 0).
pub fn ai_task_thread_on_shutdown(e: &mut Engine, this: Ptr) {
    let router = this.addr() + THREAD_ROUTER;
    let allocator = e.call(ROUTER_WORD, &args![router]).u32();
    let block = e.mem.u32(this.addr() + THREAD_SCRATCH);
    e.vcall(allocator, 0x08, &args![block, THREAD_SCRATCH_SIZE]);
    e.call(HK_QUIT_THREAD, &[]);
    let memory_system = e.call(MEMORY_SYSTEM_GET_INSTANCE, &[]).u32();
    e.vcall(memory_system, 0x10, &args![router, 1u32]);
    let router_again = e.call(RETURN_THIS, &args![router]).u32();
    e.call(ROUTER_SET_BLOCK, &args![router_again, 0u32, 0u32]);
}

// Translated from 008ca070 (decompiled, FalloutNV.exe 1.4.0.525)
/// The second scheduling pass, for the Detection (`011dfc70`) and Movement
/// (`011dfb88`) tasklets when those settings are on. Only when `011c3ea4` is
/// 1, at least one of the two is on, `011f1220` is set and the actor list
/// is not empty: queues a count message, one mob message per actor (the
/// count is read once) and an end message to each enabled tasklet, then
/// either puts them on the worker ring, releases the worker with index 2 and
/// sets the manager's flag +0x10180 (threads on, `011dfb68`), or runs them
/// to completion here. The compiler's exception frames are not translated.
pub fn ai_task_manager_process_parallel(e: &mut Engine, this: Ptr) {
    if setting_value(e, SETTING_SPLIT_UPDATE) != 1 {
        return;
    }
    if setting_byte(e, SETTING_NO_DETECTION) == 0 && setting_byte(e, SETTING_NO_MOVEMENT) == 0 {
        return;
    }
    if e.mem.u8(ACTOR_PACKAGES_ACTIVE) == 0 || actor_count(e) == 0 {
        return;
    }
    let count = actor_count(e);
    let detection = this;
    let movement = Ptr::new(this.addr() + 0x138);
    if setting_byte(e, SETTING_NO_DETECTION) != 0 {
        push_count_message(e, detection, count);
    }
    if setting_byte(e, SETTING_NO_MOVEMENT) != 0 {
        push_count_message(e, movement, count);
    }
    let mut index = 0u32;
    while index < count {
        if setting_byte(e, SETTING_NO_DETECTION) != 0 {
            push_mob_message(e, detection, index);
        }
        if setting_byte(e, SETTING_NO_MOVEMENT) != 0 {
            push_mob_message(e, movement, index);
        }
        index += 1;
    }
    if setting_byte(e, SETTING_NO_DETECTION) != 0 {
        ai_task_queue_push_end(e, detection);
    }
    if setting_byte(e, SETTING_NO_MOVEMENT) != 0 {
        ai_task_queue_push_end(e, movement);
    }
    if setting_byte(e, SETTING_THREADED) != 0 {
        let ring = Ptr::new(TASK_RING);
        if setting_byte(e, SETTING_NO_DETECTION) != 0 {
            ai_task_queue_enqueue(e, ring, detection.addr());
        }
        if setting_byte(e, SETTING_NO_MOVEMENT) != 0 {
            ai_task_queue_enqueue(e, ring, movement.addr());
        }
        ai_task_manager_release_semaphore(e, Ptr::new(THREAD_MANAGER), 2);
        e.mem.set_u8(this.addr() + 0x10180, 1);
    } else {
        if setting_byte(e, SETTING_NO_DETECTION) != 0 {
            mobile_object_tasklet_data_run_to_completion(e, detection.cast(), 0);
        }
        if setting_byte(e, SETTING_NO_MOVEMENT) != 0 {
            mobile_object_tasklet_data_run_to_completion(e, movement.cast(), 0);
        }
    }
}

// Translated from 008ca350 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 008ca350 - compiler exception cleanup path
/// Destroys the message array and restores its own base vtable.
// open: compiler exception cleanup around the generated array destructor.
pub fn ai_task_message_pool_destructor(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), 0x0108_5834);
    e.call(
        0x00ec_5fce,
        &args![this.addr() + 0x0c, 0x10u32, 0x1000u32, 0x0048_3710u32],
    );
    ai_task_message_pool_base_destructor(e, this);
}

// Translated from 008ca3c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Restores the array base class vtable.
pub fn ai_task_message_pool_base_destructor(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), 0x0108_583c);
}

// Translated from 008c9ae0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 008c9ae0 - compiler exception cleanup path
/// Adds a thread while the manager has fewer than three workers. A thread
/// is constructed only when `operator new` returns a non-null address.
// open: compiler SEH cleanup path.
pub fn ai_task_thread_manager_add_thread(e: &mut Engine, this: Ptr, processor: u32) {
    let count = e.mem.i32(this.addr() + 0x0c);
    if count >= 3 {
        return;
    }
    let thread_addr = e.call(OPERATOR_NEW, &args![0x78u32]).u32();
    let thread = Ptr::new(thread_addr);
    let constructed = if thread.is_null() {
        Ptr::NULL
    } else {
        ai_task_thread_constructor(e, thread, processor)
    };
    e.mem
        .set_u32(this.addr() + count as u32 * 4, constructed.addr());
    e.call(0x00aa_6430, &args![constructed, 0x10000u32, 0u32, 0u32]);
    e.call(0x00aa_6550, &args![constructed, processor]);
    e.call(0x00aa_6540, &args![constructed, 0x0108_5804u32]);
    e.mem.set_i32(this.addr() + 0x0c, count.wrapping_add(1));
}

// Translated from 008c9bd0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 008c9bd0 - compiler exception cleanup path
/// Constructs a worker's `BSThread` base, two semaphores, memory router,
/// worker index and clear kill flag.
// open: compiler SEH cleanup path.
pub fn ai_task_thread_constructor(e: &mut Engine, this: Ptr, thread_index: u32) -> Ptr {
    e.call(0x00aa_6360, &args![this]);
    e.mem.set_u32(this.addr(), 0x0108_5818);
    e.call(0x0086_c6a0, &args![this.addr() + 0x30, 0u32, 10u32]);
    e.call(0x0086_c6a0, &args![this.addr() + 0x3c, 0u32, 10u32]);
    e.call(0x00c8_5770, &args![this.addr() + 0x50]);
    e.mem.set_u32(this.addr() + 0x4c, thread_index);
    e.mem.set_u8(this.addr() + 0x48, 0);
    this
}

// Translated from 008c9cf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Removes and returns the oldest entry in the ten-slot task queue while
/// holding its critical section. An empty queue returns zero.
pub fn ai_task_queue_dequeue(e: &mut Engine, this: Ptr) -> u32 {
    e.call(CRITICAL_SECTION_ENTER, &args![this, 0u32]);
    let mut value = 0;
    let count = e.mem.i32(this.addr() + 0x40);
    if count > 0 {
        let head = e.mem.i32(this.addr() + 0x44);
        value = e.mem.u32(this.addr() + 0x18 + head as u32 * 4);
        e.mem.set_i32(this.addr() + 0x40, count.wrapping_sub(1));
        let mut next = head.wrapping_add(1);
        if next >= 10 {
            next = 0;
        }
        e.mem.set_i32(this.addr() + 0x44, next);
    }
    e.call(CRITICAL_SECTION_LEAVE, &args![this]);
    value
}

// Translated from 008c9ec0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Worker context scalar deleting destructor; bit zero of `flags` requests
/// deallocation after member destruction.
pub fn thread_context_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    thread_context_destructor(e, this);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 008c9ef0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 008c9ef0 - compiler exception cleanup path
/// Destroys the memory router and both semaphores, then the `BSThread`
/// base.
// open: compiler SEH unwind chain.
pub fn thread_context_destructor(e: &mut Engine, this: Ptr) {
    e.call(0x0052_d240, &args![this.addr() + 0x50]);
    e.call(0x0055_a2d0, &args![this.addr() + 0x3c]);
    e.call(0x0055_a2d0, &args![this.addr() + 0x30]);
    e.call(0x00aa_63c0, &args![this]);
}

// Translated from 008c9f70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Releases the start semaphore of each registered worker.
pub fn ai_task_thread_manager_release_semaphores(e: &mut Engine, this: Ptr) {
    let count = e.mem.i32(this.addr() + 0x0c);
    for index in 0..count.max(0) as u32 {
        let thread = Ptr::new(e.mem.u32(this.addr() + index * 4));
        ai_task_thread_release_semaphore(e, thread);
    }
}

// Translated from 008c9fb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Releases a worker's semaphore at `+0x30`.
pub fn ai_task_thread_release_semaphore(e: &mut Engine, this: Ptr) {
    e.call(0x0044_2550, &args![this.addr() + 0x30]);
}

// Translated from 008c9fd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Signals the secondary semaphore of each registered worker.
pub fn ai_task_thread_manager_wake_threads(e: &mut Engine, this: Ptr) {
    let count = e.mem.i32(this.addr() + 0x0c);
    for index in 0..count.max(0) as u32 {
        let thread = e.mem.u32(this.addr() + index * 4);
        e.call(0x008c_7490, &args![thread]);
    }
}

// Translated from 008ca010 (decompiled, FalloutNV.exe 1.4.0.525)
/// Pushes an item through the queue object's virtual `Push` slot at +4.
pub fn ai_task_queue_push(e: &mut Engine, this: Ptr, item: Ptr) {
    e.vcall(this.addr() + 0x1c, 4, &args![item]);
}

// Translated from 008ca040 (decompiled, FalloutNV.exe 1.4.0.525)
/// Builds an end message on the stack and sends it through the queue's
/// virtual `Push` slot at +4.
pub fn ai_task_queue_push_end(e: &mut Engine, this: Ptr) {
    e.with_stack(12, |e, msg| {
        fn_008c8190(e, msg.cast());
        e.vcall(this.addr() + 0x1c, 4, &args![msg]);
    });
}

// Translated from 008ca2e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Releases the semaphore belonging to the thread at `index` in a three
/// pointer array.
pub fn ai_task_manager_release_semaphore(e: &mut Engine, threads: Ptr, index: u32) {
    let thread = Ptr::new(e.mem.u32(threads.addr() + index * 4));
    ai_task_thread_release_semaphore(e, thread);
}

// Translated from 008ca300 (decompiled, FalloutNV.exe 1.4.0.525)
/// If the parallel task flag is set, signals semaphore index 2 and clears
/// the flag.
pub fn ai_task_manager_clear_parallel_flag(e: &mut Engine, this: Ptr) {
    if e.mem.u8(this.addr() + 0x10180) != 0 {
        ai_task_manager_signal_semaphore(e, Ptr::new(0x011d_fc24), 2);
        e.mem.set_u8(this.addr() + 0x10180, 0);
    }
}

// Translated from 008ca330 (decompiled, FalloutNV.exe 1.4.0.525)
/// Signals the secondary semaphore for the thread at `index`.
pub fn ai_task_manager_signal_semaphore(e: &mut Engine, threads: Ptr, index: u32) {
    let thread = e.mem.u32(threads.addr() + index * 4);
    e.call(0x008c_7490, &args![thread]);
}

// Translated from 008ca3e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Message pool base scalar deleting destructor.
pub fn ai_task_message_pool_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    ai_task_message_pool_base_destructor(e, this);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 008ca410 (decompiled, FalloutNV.exe 1.4.0.525)
/// Message pool scalar deleting destructor, including its array destructor.
pub fn ai_task_message_pool_array_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    ai_task_message_pool_destructor(e, this);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 008ca440 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructs the common message queue base and initializes its allocator,
/// empty head and tail pointer.
pub fn common_message_queue_constructor(e: &mut Engine, this: Ptr, allocator: Ptr) -> Ptr {
    e.call(0x008c_a740, &args![this]);
    e.mem.set_u32(this.addr(), 0x0108_5844);
    e.mem.set_u32(this.addr() + 8, allocator.addr());
    e.mem.set_u32(this.addr() + 0x0c, 0);
    e.mem.set_u32(this.addr() + 0x10, this.addr() + 0x0c);
    this
}

// Translated from 008ca480 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 008ca480 - compiler exception cleanup path
/// Drains messages with the queue's `TryPop` helper, then destroys the
/// message queue base.
// open: compiler SEH cleanup path.
pub fn common_message_queue_destructor(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), 0x0108_5844);
    e.with_stack(12, |e, msg| {
        fn_008c8190(e, msg.cast());
        while e.call(0x006e_c390, &args![this, msg]).bool() {}
    });
    message_queue_destructor(e, this);
}

// Translated from 008ca500 (decompiled, FalloutNV.exe 1.4.0.525)
/// Restores the derived message queue vtable, then destroys its base.
pub fn message_queue_destructor(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), 0x0108_5864);
    message_queue_base_destructor(e, this);
}

// Translated from 008ca520 (decompiled, FalloutNV.exe 1.4.0.525)
/// Restores the message queue base vtable.
pub fn message_queue_base_destructor(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), 0x0108_5884);
}

// Translated from 008ca540 (decompiled, FalloutNV.exe 1.4.0.525)
/// Message queue scalar deleting destructor with the executable's bit-zero
/// deallocation condition.
pub fn message_queue_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    message_queue_destructor(e, this);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 008ca570 (decompiled, FalloutNV.exe 1.4.0.525)
/// Mobile message queue scalar deleting destructor with the executable's
/// bit-zero deallocation condition.
pub fn mobile_message_queue_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    message_queue_base_destructor(e, this);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 008ca5a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Allocates a linked-list node from the queue's freelist and appends it at
/// the tail. Returns false when no freelist is configured or allocation
/// fails.
pub fn common_ll_message_queue_try_push(e: &mut Engine, this: Ptr, value: Ptr) -> bool {
    let pool = e.mem.u32(this.addr() + 8);
    if pool == 0 {
        return false;
    }
    e.with_stack(8, |e, out| {
        if !e.call(0x008c_a7c0, &args![pool, out, value]).bool() {
            return false;
        }
        let node = e.mem.u32(out.addr());
        if e.mem.u32(this.addr() + 0x0c) == 0 {
            e.mem.set_u32(this.addr() + 0x10, this.addr() + 0x0c);
        }
        let tail_link = e.mem.u32(this.addr() + 0x10);
        e.mem.set_u32(tail_link, node);
        e.mem.set_u32(node + 0x0c, 0);
        e.mem.set_u32(this.addr() + 0x10, node + 0x0c);
        true
    })
}

/// `0043b460`: the interlocked compare-exchange the message pool's lock word
/// (+4) is taken and released with. `0040fbe0` is the empty debug check the
/// game calls around the locked region. `00483710` is an empty destructor
/// and `006815c0` returns its `this`.
const POOL_COMPARE_EXCHANGE: u32 = 0x0043_b460;
const POOL_DEBUG_CHECK: u32 = 0x0040_fbe0;
const EMPTY_DESTRUCTOR: u32 = 0x0048_3710;
const RETURN_THIS: u32 = 0x0068_15c0;
/// `_eh_vector_constructor_iterator_` (stdcall: array, element size, count,
/// constructor, destructor) and the element constructor `008caae0`, which
/// clears the node's next link (+0xC).
const VECTOR_CONSTRUCTOR_ITERATOR: u32 = 0x00ec_782f;
const NODE_CONSTRUCTOR: u32 = 0x008c_aae0;
/// `operator new(size, address)` placement form (`006e6da0`, cdecl): returns
/// the address.
const PLACEMENT_NEW: u32 = 0x006e_6da0;
/// `MobileObjectMessage` copy constructor (`008caa70`, one argument).
const MESSAGE_COPY: u32 = 0x008c_aa70;
/// Vtable of the message pool with its array (`01085834`) and of its base
/// (`0108583c`); of the derived message queue (`01085864`) and of its base
/// (`01085884`).
const VTABLE_MESSAGE_POOL: u32 = 0x0108_5834;
const VTABLE_MESSAGE_POOL_BASE: u32 = 0x0108_583c;
const VTABLE_MESSAGE_QUEUE: u32 = 0x0108_5864;
const VTABLE_MESSAGE_QUEUE_BASE: u32 = 0x0108_5884;
/// Number of nodes of the message pool and their size.
const POOL_NODE_COUNT: u32 = 0x1000;
const POOL_NODE_SIZE: u32 = 0x10;

/// Takes the pool's lock word (+4): spins on the compare-exchange 0 -> 1.
fn pool_lock(e: &mut Engine, pool: Ptr) {
    let lock = pool.addr() + 4;
    while e
        .call(POOL_COMPARE_EXCHANGE, &args![lock, 0u32, 1u32])
        .u32()
        != 0
    {}
    e.call(POOL_DEBUG_CHECK, &args![]);
}

/// Releases the pool's lock word (compare-exchange 1 -> 0).
fn pool_unlock(e: &mut Engine, pool: Ptr) {
    e.call(POOL_DEBUG_CHECK, &args![]);
    e.call(POOL_COMPARE_EXCHANGE, &args![pool.addr() + 4, 1u32, 0u32]);
}

// Translated from 008ca620 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSTCommonLLMessageQueue<MobileObjectMessage>::DoTryPop` (Xbox PDB): takes
/// the first node of the queue's list (+0xC), copies its message to `out`,
/// unlinks it and gives the node back to the pool (`*(this + 8)`). False when
/// the list is empty.
pub fn mobile_message_queue_do_try_pop(e: &mut Engine, this: Ptr, out: Ptr) -> bool {
    let node = e.mem.u32(this.addr() + 0x0c);
    if node == 0 {
        return false;
    }
    let source = e.call(RETURN_THIS, &args![node]).u32();
    e.call(MESSAGE_COPY, &args![out, source]);
    let next = e.mem.u32(node + 0x0c);
    e.mem.set_u32(this.addr() + 0x0c, next);
    let pool = Ptr::new(e.mem.u32(this.addr() + 8));
    e.with_stack(4, |e, holder| {
        e.mem.set_u32(holder.addr(), node);
        message_pool_release_node(e, pool, holder.cast());
    });
    true
}

// Translated from 008ca680 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 008ca680 - compiler exception cleanup path
/// Message pool constructor: the base (`008ca800`), the pool vtable, the
/// 0x1000 nodes of 0x10 bytes at +0xC constructed
/// (`_eh_vector_constructor_iterator_` with the node constructor `008caae0`)
/// and linked into the free list (`008ca830`). The compiler's exception frame
/// is not translated.
// open: compiler exception frame (FS:[0] chain) around the node array construction.
pub fn ai_task_message_pool_constructor(e: &mut Engine, this: Ptr) -> Ptr {
    message_pool_base_constructor(e, this);
    e.mem.set_u32(this.addr(), VTABLE_MESSAGE_POOL);
    e.call(
        VECTOR_CONSTRUCTOR_ITERATOR,
        &args![
            this.addr() + 0x0c,
            POOL_NODE_SIZE,
            POOL_NODE_COUNT,
            NODE_CONSTRUCTOR,
            EMPTY_DESTRUCTOR
        ],
    );
    message_pool_link_nodes(e, this, POOL_NODE_COUNT, Ptr::new(this.addr() + 0x0c));
    this
}

// Translated from 008ca710 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSTCommonLLMessageQueue<MobileObjectMessage>::scalar deleting destructor`
/// (Xbox PDB): the queue destructor (`008ca480`), then `operator delete`
/// when bit zero of `flags` is set. Returns `this`.
pub fn common_message_queue_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    common_message_queue_destructor(e, this);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 008ca740 (decompiled, FalloutNV.exe 1.4.0.525)
/// Message queue constructor: the base constructor, the derived queue vtable
/// and the word at +4 cleared. Returns `this`.
pub fn message_queue_constructor(e: &mut Engine, this: Ptr) -> Ptr {
    message_queue_base_constructor(e, this);
    e.mem.set_u32(this.addr(), VTABLE_MESSAGE_QUEUE);
    e.mem.set_u32(this.addr() + 4, 0);
    this
}

// Translated from 008ca770 (decompiled, FalloutNV.exe 1.4.0.525)
/// Gives a node back to the pool: runs the node's (empty) destructor
/// `00483710` on `*node_slot`, then pushes it on the free list
/// (`008ca8d0`, which clears `*node_slot`).
pub fn message_pool_release_node(e: &mut Engine, this: Ptr, node_slot: Ptr) {
    let node = e.mem.u32(node_slot.addr());
    e.call(EMPTY_DESTRUCTOR, &args![node]);
    message_pool_push_free_node(e, this, node_slot);
}

// Translated from 008ca7a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Message queue base constructor: only the base vtable. Returns `this`.
pub fn message_queue_base_constructor(e: &mut Engine, this: Ptr) -> Ptr {
    e.mem.set_u32(this.addr(), VTABLE_MESSAGE_QUEUE_BASE);
    this
}

// Translated from 008ca7c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSTFreeList<MobileObjectMessage>::Allocate<MobileObjectMessage>` (Xbox
/// PDB): takes a free node into `*out` (`008ca940`) and, if there was one,
/// constructs a copy of the message `value` in it (`008ca9e0`). False when
/// the pool is exhausted.
pub fn message_pool_allocate(e: &mut Engine, this: Ptr, out: Ptr, value: Ptr) -> bool {
    if !message_pool_pop_free_node(e, this, out) {
        return false;
    }
    let node = Ptr::new(e.mem.u32(out.addr()));
    message_construct_copy(e, node, value);
    true
}

// Translated from 008ca800 (decompiled, FalloutNV.exe 1.4.0.525)
/// Message pool base constructor: base vtable, lock word (+4) and free-list
/// head (+8) cleared. Returns `this`.
pub fn message_pool_base_constructor(e: &mut Engine, this: Ptr) -> Ptr {
    e.mem.set_u32(this.addr(), VTABLE_MESSAGE_POOL_BASE);
    e.mem.set_u32(this.addr() + 4, 0);
    e.mem.set_u32(this.addr() + 8, 0);
    this
}

// Translated from 008ca830 (decompiled, FalloutNV.exe 1.4.0.525)
/// Makes `array` (`count` nodes of 0x10 bytes) the free list: stores it as
/// the head (+8), then, holding the lock, links every node's next word
/// (+0xC) to the following node and ends the last with zero.
pub fn message_pool_link_nodes(e: &mut Engine, this: Ptr, count: u32, array: Ptr) {
    e.mem.set_u32(this.addr() + 8, array.addr());
    pool_lock(e, this);
    let mut node = array.addr();
    let mut index = 0u32;
    while index < count.wrapping_sub(1) {
        e.mem
            .set_u32(node + 0x0c, node.wrapping_add(POOL_NODE_SIZE));
        index += 1;
        node = node.wrapping_add(POOL_NODE_SIZE);
    }
    e.mem.set_u32(node + 0x0c, 0);
    pool_unlock(e, this);
}

// Translated from 008ca8d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Pushes the node `*node_slot` on the free list under the lock and clears
/// `*node_slot`.
pub fn message_pool_push_free_node(e: &mut Engine, this: Ptr, node_slot: Ptr) {
    pool_lock(e, this);
    let node = e.mem.u32(node_slot.addr());
    let head = e.mem.u32(this.addr() + 8);
    e.mem.set_u32(node + 0x0c, head);
    e.mem.set_u32(this.addr() + 8, node);
    e.mem.set_u32(node_slot.addr(), 0);
    pool_unlock(e, this);
}

// Translated from 008ca940 (decompiled, FalloutNV.exe 1.4.0.525)
/// Pops the first free node under the lock into `*out` (its next link is
/// cleared); when the free list is empty `*out` is zero and the result false.
pub fn message_pool_pop_free_node(e: &mut Engine, this: Ptr, out: Ptr) -> bool {
    pool_lock(e, this);
    let head = e.mem.u32(this.addr() + 8);
    let found = head != 0;
    if found {
        e.mem.set_u32(out.addr(), head);
        let next = e.mem.u32(head + 0x0c);
        e.mem.set_u32(this.addr() + 8, next);
        e.mem.set_u32(head + 0x0c, 0);
    } else {
        e.mem.set_u32(out.addr(), 0);
    }
    pool_unlock(e, this);
    found
}

// Translated from 008ca9e0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 008ca9e0 - compiler exception cleanup path
/// Constructs a copy of the `MobileObjectMessage` `value` in the node `this`:
/// the empty destructor-style call `00483710`, placement `operator new`
/// (`006e6da0`, size 0xC) and, if that gave an address, the copy
/// constructor (`008caa70`). Returns the constructed message or zero. The
/// compiler's exception frame is not translated.
// open: compiler exception frame (FS:[0] chain) around the placement-new copy construction.
pub fn message_construct_copy(e: &mut Engine, this: Ptr, value: Ptr) -> Ptr {
    e.call(EMPTY_DESTRUCTOR, &args![this]);
    let place = e.call(PLACEMENT_NEW, &args![0xcu32, this]).u32();
    if place == 0 {
        return Ptr::NULL;
    }
    e.call(MESSAGE_COPY, &args![place, value]).ptr()
}
#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    const HANDLE_COUNT: u32 = 0x7000_0010;
    const HANDLE_MOB: u32 = 0x7000_0014;
    const HANDLE_END: u32 = 0x7000_0018;
    const QUEUE_RUN: u32 = 0x7000_0020;
    const QUEUE_DRAIN: u32 = 0x7000_0030;

    fn ok(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    /// Engine with the pages of the globals the handlers read and write.
    fn engine() -> Engine {
        let mut e = Engine::new();
        for page in [
            0x011d_e000u32,
            0x011d_f000,
            0x011d_8000,
            0x011e_0000,
            0x0101_2000,
            0x0101_7000,
            0x0101_6000,
        ] {
            e.map(page, 0x1000);
        }
        e.set_global(ZERO, 0.0f64);
        e
    }

    /// An object whose first word points at a fresh vtable holding the
    /// given (byte offset, target) slots.
    fn object_with_slots(e: &mut Engine, size: u32, slots: &[(u32, u32)]) -> Ptr {
        let table = e.mem.alloc(0x400);
        for (offset, target) in slots {
            e.mem.set_u32(table + offset, *target);
        }
        let object = e.mem.alloc(size);
        e.mem.set_u32(object, table);
        Ptr::new(object)
    }

    /// A tasklet (any subclass, `size` bytes) with the handler slots and a
    /// queue object at +0x1C whose vtable has the two drain slots.
    fn tasklet(e: &mut Engine, size: u32, handlers: &[(u32, u32)]) -> Ptr {
        let this: Ptr = object_with_slots(e, size, handlers);
        let queue_table = e.mem.alloc(0x40);
        e.mem.set_u32(queue_table + 0xc, QUEUE_RUN);
        e.mem.set_u32(queue_table + 0x10, QUEUE_DRAIN);
        e.mem.set_u32(this.addr() + 0x1c, queue_table);
        this
    }

    fn standard_handlers() -> [(u32, u32); 3] {
        [(0x10, HANDLE_COUNT), (0x14, HANDLE_MOB), (0x18, HANDLE_END)]
    }

    fn log_calls(e: &mut Engine) {
        e.call_log = Some(vec![]);
    }

    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, args)| args.clone())
            .collect()
    }

    fn address_log(e: &Engine) -> Vec<u32> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .map(|(a, _)| *a)
            .collect()
    }

    /// A queue double that hands out the given (type, count, mob) messages
    /// one by one and then reports an empty queue.
    fn queue_script(e: &mut Engine, slot: u32, messages: Vec<(u32, u32, u32)>) {
        let mut messages = messages.into_iter();
        e.register_double(slot, move |e, a| match messages.next() {
            Some((kind, count, mob)) => {
                e.mem.set_u32(a[1], kind);
                e.mem.set_u32(a[1] + 4, count);
                e.mem.set_u32(a[1] + 8, mob);
                ok(1)
            }
            None => ok(0),
        });
    }

    fn quiet(e: &mut Engine, addrs: &[u32]) {
        for addr in addrs {
            e.register(*addr, |_, _| Ret::default());
        }
    }

    #[test]
    fn process_dispatches_messages_until_the_end_message() {
        let mut e = engine();
        quiet(
            &mut e,
            &[HANDLE_COUNT, HANDLE_MOB, HANDLE_END, TASKLET_SET_YIELDING],
        );
        let this: Ptr = tasklet(&mut e, 0x38, &standard_handlers());
        e.mem.set_u32(this.addr() + 0x34, 0xe7e7);
        queue_script(
            &mut e,
            QUEUE_DRAIN,
            vec![(0, 5, 0), (1, 0, 0x1234), (2, 0, 0)],
        );
        log_calls(&mut e);
        e.call(0x008c_8130, &args![this]);
        assert_eq!(calls_to(&e, HANDLE_COUNT), vec![vec![this.addr(), 5]]);
        assert_eq!(calls_to(&e, HANDLE_MOB), vec![vec![this.addr(), 0x1234]]);
        assert_eq!(calls_to(&e, HANDLE_END), vec![vec![this.addr(), 0xe7e7]]);
        assert!(calls_to(&e, TASKLET_SET_YIELDING).is_empty());
        assert_eq!(e.mem.u32(this.addr() + 0x30), 5);
    }

    #[test]
    fn process_yields_when_the_queue_runs_dry() {
        let mut e = engine();
        quiet(
            &mut e,
            &[HANDLE_COUNT, HANDLE_MOB, HANDLE_END, TASKLET_SET_YIELDING],
        );
        let this: Ptr = tasklet(&mut e, 0x38, &standard_handlers());
        queue_script(&mut e, QUEUE_DRAIN, vec![(1, 0, 0x55)]);
        log_calls(&mut e);
        e.call(0x008c_8130, &args![this]);
        assert_eq!(calls_to(&e, HANDLE_MOB), vec![vec![this.addr(), 0x55]]);
        assert!(calls_to(&e, HANDLE_END).is_empty());
        assert_eq!(calls_to(&e, TASKLET_SET_YIELDING), vec![vec![this.addr()]]);
    }

    #[test]
    fn message_initializer_makes_an_end_message() {
        let mut e = engine();
        let msg: Ptr<MobileObjectMessage> = e.new_object();
        e.mem.set_u32(msg.addr(), 9);
        e.mem.set_u32(msg.addr() + 4, 9);
        e.mem.set_u32(msg.addr() + 8, 9);
        assert_eq!(e.call(0x008c_8190, &args![msg]).ptr::<()>(), msg.cast());
        assert_eq!(e.mem.u32(msg.addr()), 2);
        assert_eq!(e.mem.u32(msg.addr() + 4), 0);
        assert_eq!(e.mem.u32(msg.addr() + 8), 0);
    }

    #[test]
    fn dispatch_handles_each_message_type() {
        let mut e = engine();
        quiet(&mut e, &[HANDLE_COUNT, HANDLE_MOB, HANDLE_END]);
        let this: Ptr = tasklet(&mut e, 0x38, &standard_handlers());
        let msg: Ptr<MobileObjectMessage> = e.new_object();
        log_calls(&mut e);
        for (kind, expected_continue) in [(0u32, true), (1, true), (2, false), (3, true)] {
            e.mem.set_u32(msg.addr(), kind);
            e.mem.set_u32(msg.addr() + 4, 7);
            e.mem.set_u32(msg.addr() + 8, 0x99);
            let r = e.call(0x008c_81c0, &args![this, msg, 0xabu32]).bool();
            assert_eq!(r, expected_continue, "type {kind}");
        }
        assert_eq!(calls_to(&e, HANDLE_COUNT), vec![vec![this.addr(), 7]]);
        assert_eq!(calls_to(&e, HANDLE_MOB), vec![vec![this.addr(), 0x99]]);
        assert_eq!(calls_to(&e, HANDLE_END), vec![vec![this.addr(), 0xab]]);
        assert_eq!(e.mem.u32(this.addr() + 0x30), 7);
    }

    #[test]
    fn run_to_completion_stores_the_type_and_drains_the_queue() {
        let mut e = engine();
        quiet(
            &mut e,
            &[HANDLE_COUNT, HANDLE_MOB, HANDLE_END, TASKLET_SET_YIELDING],
        );
        let this: Ptr = tasklet(&mut e, 0x38, &standard_handlers());
        queue_script(&mut e, QUEUE_RUN, vec![(1, 0, 0x21), (1, 0, 0x22)]);
        log_calls(&mut e);
        e.call(0x008c_8240, &args![this, 3u32]);
        assert_eq!(e.mem.u32(this.addr() + 0x34), 3);
        assert_eq!(
            calls_to(&e, HANDLE_MOB),
            vec![vec![this.addr(), 0x21], vec![this.addr(), 0x22]]
        );
        // The queue ran dry: no end message was dispatched, nothing yields.
        assert!(calls_to(&e, HANDLE_END).is_empty());
        assert!(calls_to(&e, TASKLET_SET_YIELDING).is_empty());
    }

    #[test]
    fn run_to_completion_stops_at_the_end_message() {
        let mut e = engine();
        quiet(&mut e, &[HANDLE_MOB, HANDLE_END]);
        let this: Ptr = tasklet(&mut e, 0x38, &standard_handlers());
        queue_script(&mut e, QUEUE_RUN, vec![(2, 0, 0), (1, 0, 0x21)]);
        log_calls(&mut e);
        e.call(0x008c_8240, &args![this, 4u32]);
        assert_eq!(calls_to(&e, HANDLE_END), vec![vec![this.addr(), 4]]);
        assert!(calls_to(&e, HANDLE_MOB).is_empty());
    }

    #[test]
    fn base_constructor_builds_the_queue_and_clears_the_fields() {
        let mut e = engine();
        quiet(
            &mut e,
            &[TASKLET_BASE_CONSTRUCTOR, MESSAGE_QUEUE_CONSTRUCTOR],
        );
        let this: Ptr<MobileObjectTaskletData> = e.new_object();
        for off in [0x18, 0x30, 0x34] {
            e.mem.set_u32(this.addr() + off, 0xdead);
        }
        log_calls(&mut e);
        let r = e.call(0x008c_82e0, &args![this, 0x5150u32]).ptr::<()>();
        assert_eq!(r, this.cast());
        assert_eq!(
            address_log(&e),
            vec![
                0x008c_82e0,
                TASKLET_BASE_CONSTRUCTOR,
                MESSAGE_QUEUE_CONSTRUCTOR
            ]
        );
        assert_eq!(
            calls_to(&e, MESSAGE_QUEUE_CONSTRUCTOR),
            vec![vec![this.addr() + 0x1c, 0x5150]]
        );
        assert_eq!(e.mem.u32(this.addr()), VTABLE_MOBILE_OBJECT_TASKLET_DATA);
        for off in [0x18, 0x30, 0x34] {
            assert_eq!(e.mem.u32(this.addr() + off), 0);
        }
    }

    #[test]
    fn destructor_body_destroys_the_queue_then_the_base() {
        let mut e = engine();
        quiet(&mut e, &[MESSAGE_QUEUE_DESTRUCTOR, TASKLET_BASE_DESTRUCTOR]);
        let this: Ptr<MobileObjectTaskletData> = e.new_object();
        log_calls(&mut e);
        e.call(0x008c_83a0, &args![this]);
        assert_eq!(
            address_log(&e),
            vec![
                0x008c_83a0,
                MESSAGE_QUEUE_DESTRUCTOR,
                TASKLET_BASE_DESTRUCTOR
            ]
        );
        assert_eq!(
            calls_to(&e, MESSAGE_QUEUE_DESTRUCTOR),
            vec![vec![this.addr() + 0x1c]]
        );
        assert_eq!(
            calls_to(&e, TASKLET_BASE_DESTRUCTOR),
            vec![vec![this.addr()]]
        );
    }

    /// Checks a scalar deleting destructor: it runs the destructor body,
    /// deletes only when bit 0 of the flags is set, and returns `this`.
    fn check_deleting_destructor(entry: u32, vtable: u32, size: u32) {
        let mut e = engine();
        quiet(
            &mut e,
            &[
                MESSAGE_QUEUE_DESTRUCTOR,
                TASKLET_BASE_DESTRUCTOR,
                OPERATOR_DELETE,
            ],
        );
        let this: Ptr = Ptr::new(e.mem.alloc(size));
        for (flags, deleted) in [(0u32, false), (1, true)] {
            log_calls(&mut e);
            let r = e.call(entry, &args![this, flags]).ptr::<()>();
            assert_eq!(r, this);
            assert_eq!(e.mem.u32(this.addr()), vtable);
            assert_eq!(calls_to(&e, MESSAGE_QUEUE_DESTRUCTOR).len(), 1);
            assert_eq!(calls_to(&e, OPERATOR_DELETE).len(), usize::from(deleted));
        }
    }

    #[test]
    fn base_scalar_deleting_destructor_deletes_on_bit_zero() {
        let mut e = engine();
        quiet(
            &mut e,
            &[
                MESSAGE_QUEUE_DESTRUCTOR,
                TASKLET_BASE_DESTRUCTOR,
                OPERATOR_DELETE,
            ],
        );
        let this: Ptr = Ptr::new(e.mem.alloc(0x38));
        for (flags, deleted) in [(0u32, false), (1, true)] {
            log_calls(&mut e);
            assert_eq!(e.call(0x008c_8370, &args![this, flags]).ptr::<()>(), this);
            assert_eq!(calls_to(&e, MESSAGE_QUEUE_DESTRUCTOR).len(), 1);
            assert_eq!(calls_to(&e, OPERATOR_DELETE).len(), usize::from(deleted));
        }
    }

    #[test]
    fn detection_destructor_resets_the_vtable_and_deletes_on_bit_zero() {
        check_deleting_destructor(0x008c_8400, VTABLE_DETECTION_TASK_DATA, 0x40);
    }

    #[test]
    fn detection_destructor_body_resets_the_vtable() {
        let mut e = engine();
        quiet(&mut e, &[MESSAGE_QUEUE_DESTRUCTOR, TASKLET_BASE_DESTRUCTOR]);
        let this: Ptr = Ptr::new(e.mem.alloc(0x40));
        e.call(0x008c_8430, &args![this]);
        assert_eq!(e.mem.u32(this.addr()), VTABLE_DETECTION_TASK_DATA);
    }

    #[test]
    fn detection_constructor_sets_vtable_and_clears_the_fields() {
        let mut e = engine();
        quiet(
            &mut e,
            &[TASKLET_BASE_CONSTRUCTOR, MESSAGE_QUEUE_CONSTRUCTOR],
        );
        let this: Ptr<DetectionTaskData> = e.new_object();
        e.mem.set_f32(this.addr() + 0x38, 9.0);
        e.mem.set_u32(this.addr() + 0x3c, 9);
        let r = e.call(0x008c_82a0, &args![this, 1u32]).ptr::<()>();
        assert_eq!(r, this.cast());
        assert_eq!(e.mem.u32(this.addr()), VTABLE_DETECTION_TASK_DATA);
        assert_eq!(e.mem.f32(this.addr() + 0x38), 0.0);
        assert_eq!(e.mem.u32(this.addr() + 0x3c), 0);
    }

    #[test]
    fn detection_count_message_clears_and_enters_the_sections() {
        let mut e = engine();
        quiet(&mut e, &[CRITICAL_SECTION_ENTER]);
        let this: Ptr<DetectionTaskData> = e.new_object();
        e.mem.set_f32(this.addr() + 0x38, 9.0);
        e.mem.set_u32(this.addr() + 0x3c, 9);
        log_calls(&mut e);
        e.call(0x008c_8450, &args![this, 77u32]);
        assert_eq!(e.mem.f32(this.addr() + 0x38), 0.0);
        assert_eq!(e.mem.u32(this.addr() + 0x3c), 0);
        assert_eq!(
            calls_to(&e, CRITICAL_SECTION_ENTER),
            vec![
                vec![CRITICAL_SECTION_011DFC04, 0],
                vec![CRITICAL_SECTION_011DFBE8, 0]
            ]
        );
    }

    #[test]
    fn detection_end_message_updates_the_auto_aim_actor_and_leaves_the_sections() {
        let mut e = engine();
        quiet(&mut e, &[0x0096_4260, CRITICAL_SECTION_LEAVE]);
        e.set_global(PLAYER_CHARACTER, 0x4242_0000u32);
        let this: Ptr<DetectionTaskData> = e.new_object();
        log_calls(&mut e);
        e.call(0x008c_8490, &args![this, 0u32]);
        assert_eq!(
            e.call_log.clone().unwrap(),
            vec![
                (0x008c_8490, vec![this.addr(), 0]),
                (0x0096_4260, vec![0x4242_0000]),
                (CRITICAL_SECTION_LEAVE, vec![CRITICAL_SECTION_011DFC04]),
                (CRITICAL_SECTION_LEAVE, vec![CRITICAL_SECTION_011DFBE8]),
            ]
        );
    }

    // ---- DetectionTaskData::HandleMobMessage -------------------------

    const FAKE_IS_ACTOR: u32 = 0x7000_0100;
    const FAKE_HIDDEN: u32 = 0x7000_0104;
    const FAKE_SLOT_2B4: u32 = 0x7000_0108;
    const FAKE_SLOT_22C: u32 = 0x7000_010c;
    const FAKE_SLOT_2B0: u32 = 0x7000_0110;
    const FAKE_SLOT_2B8: u32 = 0x7000_0114;
    const FAKE_PROCESS_ACCEPTS: u32 = 0x7000_0118;
    const SETTING_CELL_NEAR: u32 = 0x7100_0000;
    const SETTING_CELL_LEVEL: u32 = 0x7100_0010;

    /// What the detection scenario varies.
    struct Scene {
        is_actor: bool,
        process_me: bool,
        hidden: bool,
        blocked: bool,
        is_player: bool,
        slot_2b4: f32,
        alpha: f64,
        process_accepts: bool,
        attacked: bool,
        distance: f32,
        near_setting: f32,
        level_setting: f32,
        level: u32,
        flags: (u8, u8),
    }

    impl Scene {
        fn new() -> Scene {
            Scene {
                is_actor: true,
                process_me: true,
                hidden: false,
                blocked: false,
                is_player: false,
                slot_2b4: -1.0,
                alpha: 0.0,
                process_accepts: true,
                attacked: false,
                distance: 100.0,
                near_setting: 200.0,
                level_setting: 3.0,
                level: 5,
                flags: (0, 0),
            }
        }
    }

    /// Runs the handler on the scene; returns the engine (with its call
    /// log), the tasklet and the mob.
    fn run_detection(scene: &Scene) -> (Engine, Ptr<DetectionTaskData>, Ptr) {
        let mut e = engine();
        e.map(0x7100_0000, 0x1000);
        e.set_global(ZERO, 0.0f64);
        e.set_global(DELAY_CAP, 4.0f32);
        e.mem.set_f32(SETTING_CELL_NEAR, scene.near_setting);
        e.mem.set_f32(SETTING_CELL_LEVEL, scene.level_setting);

        let process = object_with_slots(&mut e, 0x10, &[(0x70, FAKE_PROCESS_ACCEPTS)]);
        let extra_data = object_with_slots(&mut e, 0x100, &[]);
        let extra_table = e.mem.alloc(0x40);
        e.mem.set_u32(extra_table + 0x20, FAKE_HIDDEN);
        e.mem.set_u32(extra_data.addr() + 0x30, extra_table);

        let actor = object_with_slots(
            &mut e,
            0x1c4,
            &[
                (0x100, FAKE_IS_ACTOR),
                (0x22c, FAKE_SLOT_22C),
                (0x2b0, FAKE_SLOT_2B0),
                (0x2b4, FAKE_SLOT_2B4),
                (0x2b8, FAKE_SLOT_2B8),
            ],
        );
        e.mem
            .set_u8(actor.addr() + 0xbc, u8::from(scene.process_me));
        let player: Ptr = if scene.is_player {
            actor
        } else {
            Ptr::new(e.mem.alloc(0x40))
        };
        e.set_global(PLAYER_CHARACTER, player.addr());

        // Doubles: virtual slots first, then the helper functions.
        let is_actor = scene.is_actor;
        e.register_double(FAKE_IS_ACTOR, move |_, _| ok(u32::from(is_actor)));
        let hidden = scene.hidden;
        e.register_double(FAKE_HIDDEN, move |_, _| ok(u32::from(hidden)));
        let slot_2b4 = scene.slot_2b4;
        e.register_double(FAKE_SLOT_2B4, move |_, _| slot_2b4.into_ret());
        quiet(&mut e, &[FAKE_SLOT_22C, FAKE_SLOT_2B0, FAKE_SLOT_2B8]);
        let accepts = scene.process_accepts;
        e.register_double(FAKE_PROCESS_ACCEPTS, move |_, _| ok(u32::from(accepts)));

        e.register(0x0084_d030, |_, _| 2.0f32.into_ret());
        e.register(0x0040_ebd0, |_, a| {
            f32::from_bits(a[0]).min(f32::from_bits(a[1])).into_ret()
        });
        e.register(0x0047_6b70, |_, a| (f32::from_bits(a[1]) / 2.0).into_ret());
        let extra_addr = extra_data.addr();
        e.register_double(0x0041_81e0, move |_, _| ok(extra_addr));
        let distance = scene.distance;
        e.register_double(0x0057_23b0, move |_, _| distance.into_ret());
        e.register(0x0047_c850, |_, _| ok(0));
        let blocked = scene.blocked;
        e.register_double(0x0043_7bd0, move |_, _| ok(u32::from(blocked)));
        let alpha = scene.alpha;
        e.register_double(0x0070_14e0, move |_, _| alpha.into_ret());
        let process_addr = process.addr();
        e.register_double(GET_PROCESS, move |_, _| ok(process_addr));
        e.register(0x0049_3bb0, |_, _| ok(0));
        let (flag_a, flag_b, level) = (scene.flags.0, scene.flags.1, scene.level);
        e.register_double(0x008a_0d10, move |e, a| {
            e.mem.set_u8(a[3], flag_a);
            e.mem.set_u8(a[7], flag_b);
            ok(level)
        });
        let attacked = scene.attacked;
        e.register_double(0x008b_06d0, move |e, a| {
            e.mem.set_u32(a[3], 0);
            ok(u32::from(attacked))
        });
        e.register(0x008b_c700, |_, _| ok(0));
        e.register(0x0070_5cf0, |_, _| ok(0));
        e.register(0x0082_5c00, |_, _| ok(0));
        e.register(0x0040_3e20, |_, a| {
            ok(if a[0] == SETTING_011CCF94 {
                SETTING_CELL_NEAR
            } else {
                SETTING_CELL_LEVEL
            })
        });
        quiet(&mut e, &[0x0096_e3c0, 0x0096_6f20, 0x0096_7350]);

        let this: Ptr<DetectionTaskData> = e.new_object();
        e.mem.set_u32(this.addr() + 0x30, 3);
        log_calls(&mut e);
        e.call(0x008c_84c0, &args![this, actor]);
        (e, this, actor)
    }

    fn iupdated(e: &Engine, this: Ptr<DetectionTaskData>) -> u32 {
        e.mem.u32(this.addr() + 0x3c)
    }

    #[test]
    fn detection_mob_message_reports_a_detected_actor() {
        let scene = Scene::new();
        let (e, this, actor) = run_detection(&scene);
        // Delay: 2.0 * 3 = 6.0, capped at 4.0, random in [0, 4.0] = 2.0.
        assert_eq!(
            calls_to(&e, 0x0040_ebd0),
            vec![vec![6.0f32.to_bits(), 4.0f32.to_bits()]]
        );
        assert_eq!(
            calls_to(&e, 0x0047_6b70),
            vec![vec![0.0f32.to_bits(), 4.0f32.to_bits()]]
        );
        assert_eq!(
            calls_to(&e, FAKE_SLOT_2B0),
            vec![vec![actor.addr(), 2.0f32.to_bits()]]
        );
        let player = e.global::<u32>(PLAYER_CHARACTER);
        // Level 5 against a setting of 3.0 reports without the second flag.
        assert_eq!(
            calls_to(&e, 0x0096_6f20),
            vec![vec![player, actor.addr(), 0, 0]]
        );
        // The actor (100.0) is nearer than the setting (200.0).
        assert_eq!(
            calls_to(&e, 0x0096_e3c0),
            vec![vec![PROCESS_LISTS, actor.addr()]]
        );
        assert!(calls_to(&e, 0x0096_7350).is_empty());
        assert_eq!(iupdated(&e, this), 1);
    }

    #[test]
    fn detection_mob_message_marks_attackers_hostile() {
        let mut scene = Scene::new();
        scene.attacked = true;
        scene.near_setting = 50.0;
        let (e, _, actor) = run_detection(&scene);
        let player = e.global::<u32>(PLAYER_CHARACTER);
        assert_eq!(
            calls_to(&e, 0x0096_6f20),
            vec![vec![player, actor.addr(), 1, 0]]
        );
        // 100.0 is not nearer than 50.0.
        assert!(calls_to(&e, 0x0096_e3c0).is_empty());
    }

    #[test]
    fn detection_mob_message_below_the_level_setting_keeps_or_drops_the_perception() {
        let mut scene = Scene::new();
        scene.level_setting = 10.0;
        scene.flags = (0, 1);
        let (e, _, actor) = run_detection(&scene);
        let player = e.global::<u32>(PLAYER_CHARACTER);
        assert_eq!(
            calls_to(&e, 0x0096_6f20),
            vec![vec![player, actor.addr(), 0, 1]]
        );

        scene.flags = (0, 0);
        let (e, _, actor) = run_detection(&scene);
        assert!(calls_to(&e, 0x0096_6f20).is_empty());
        assert_eq!(calls_to(&e, 0x0096_7350), vec![vec![player, actor.addr()]]);
    }

    #[test]
    fn detection_mob_message_idle_actor_is_removed_when_negative() {
        let mut scene = Scene::new();
        scene.slot_2b4 = 1.0;
        let (e, this, _) = run_detection(&scene);
        // A non-negative slot value takes the idle path: slot +0x2B8 only.
        assert_eq!(calls_to(&e, FAKE_SLOT_2B8).len(), 1);
        assert!(calls_to(&e, 0x0096_7350).is_empty());
        assert!(calls_to(&e, FAKE_SLOT_2B0).is_empty());
        assert_eq!(iupdated(&e, this), 1);

        // `00437bd0` also sends a negative-valued actor down the idle path,
        // where it is removed from the player's perceived actors first.
        let mut scene = Scene::new();
        scene.blocked = true;
        let (e, _, actor) = run_detection(&scene);
        let player = e.global::<u32>(PLAYER_CHARACTER);
        assert_eq!(calls_to(&e, 0x0096_7350), vec![vec![player, actor.addr()]]);
        assert_eq!(calls_to(&e, FAKE_SLOT_2B8).len(), 1);
        assert!(calls_to(&e, FAKE_SLOT_2B0).is_empty());
    }

    #[test]
    fn detection_mob_message_does_nothing_while_the_fader_is_visible() {
        let mut scene = Scene::new();
        scene.alpha = 1.0;
        let (e, this, _) = run_detection(&scene);
        assert!(calls_to(&e, FAKE_SLOT_2B0).is_empty());
        assert!(calls_to(&e, 0x0096_6f20).is_empty());
        assert!(calls_to(&e, FAKE_SLOT_2B8).is_empty());
        assert_eq!(iupdated(&e, this), 1);
    }

    #[test]
    fn detection_mob_message_stops_when_the_process_refuses() {
        let mut scene = Scene::new();
        scene.process_accepts = false;
        let (e, this, actor) = run_detection(&scene);
        assert_eq!(calls_to(&e, FAKE_SLOT_2B0).len(), 1);
        assert!(calls_to(&e, 0x008a_0d10).is_empty());
        assert!(calls_to(&e, 0x0096_6f20).is_empty());
        assert_eq!(iupdated(&e, this), 1);
        let _ = actor;
    }

    #[test]
    fn detection_mob_message_skips_ineligible_mobs_but_counts_actors() {
        let mut scene = Scene::new();
        scene.is_actor = false;
        let (e, this, _) = run_detection(&scene);
        assert_eq!(iupdated(&e, this), 0);
        assert!(calls_to(&e, FAKE_SLOT_2B0).is_empty());

        let mut scene = Scene::new();
        scene.process_me = false;
        let (e, this, _) = run_detection(&scene);
        assert_eq!(iupdated(&e, this), 1);
        assert!(calls_to(&e, 0x0057_23b0).is_empty());

        let mut scene = Scene::new();
        scene.hidden = true;
        let (e, this, _) = run_detection(&scene);
        assert_eq!(iupdated(&e, this), 1);
        assert!(calls_to(&e, 0x0057_23b0).is_empty());

        let mut scene = Scene::new();
        scene.is_player = true;
        let (e, this, _) = run_detection(&scene);
        assert_eq!(iupdated(&e, this), 1);
        assert!(calls_to(&e, FAKE_SLOT_2B0).is_empty());
        assert!(calls_to(&e, FAKE_SLOT_2B8).is_empty());
    }

    #[test]
    fn detection_mob_message_with_no_mob_only_draws_the_delay() {
        let mut e = engine();
        e.set_global(DELAY_CAP, 4.0f32);
        e.register(0x0084_d030, |_, _| 2.0f32.into_ret());
        e.register(0x0040_ebd0, |_, a| {
            f32::from_bits(a[0]).min(f32::from_bits(a[1])).into_ret()
        });
        e.register(0x0047_6b70, |_, a| (f32::from_bits(a[1]) / 2.0).into_ret());
        let this: Ptr<DetectionTaskData> = e.new_object();
        e.call(0x008c_84c0, &args![this, 0u32]);
        assert_eq!(e.mem.u32(this.addr() + 0x3c), 0);
    }

    // ---- AnimationTaskData -------------------------------------------

    #[test]
    fn animation_constructor_sets_the_vtable() {
        let mut e = engine();
        quiet(
            &mut e,
            &[TASKLET_BASE_CONSTRUCTOR, MESSAGE_QUEUE_CONSTRUCTOR],
        );
        let this: Ptr<AnimationTaskData> = e.new_object();
        assert_eq!(
            e.call(0x008c_8830, &args![this, 2u32]).ptr::<()>(),
            this.cast()
        );
        assert_eq!(e.mem.u32(this.addr()), VTABLE_ANIMATION_TASK_DATA);
    }

    #[test]
    fn animation_destructor_resets_the_vtable_and_deletes_on_bit_zero() {
        check_deleting_destructor(0x008c_8860, VTABLE_ANIMATION_TASK_DATA, 0x38);
    }

    #[test]
    fn animation_destructor_body_resets_the_vtable() {
        let mut e = engine();
        quiet(&mut e, &[MESSAGE_QUEUE_DESTRUCTOR, TASKLET_BASE_DESTRUCTOR]);
        let this: Ptr = Ptr::new(e.mem.alloc(0x38));
        e.call(0x008c_8890, &args![this]);
        assert_eq!(e.mem.u32(this.addr()), VTABLE_ANIMATION_TASK_DATA);
    }

    #[test]
    fn animation_count_message_enters_the_sections() {
        let mut e = engine();
        quiet(&mut e, &[CRITICAL_SECTION_ENTER]);
        let this: Ptr<AnimationTaskData> = e.new_object();
        log_calls(&mut e);
        e.call(0x008c_88b0, &args![this, 1u32]);
        assert_eq!(
            calls_to(&e, CRITICAL_SECTION_ENTER),
            vec![
                vec![CRITICAL_SECTION_011DFC38, 0],
                vec![CRITICAL_SECTION_011DFC04, 0]
            ]
        );
    }

    #[test]
    fn animation_end_message_leaves_the_sections() {
        let mut e = engine();
        quiet(&mut e, &[CRITICAL_SECTION_LEAVE]);
        let this: Ptr<AnimationTaskData> = e.new_object();
        log_calls(&mut e);
        e.call(0x008c_88e0, &args![this, 1u32]);
        assert_eq!(
            calls_to(&e, CRITICAL_SECTION_LEAVE),
            vec![
                vec![CRITICAL_SECTION_011DFC04],
                vec![CRITICAL_SECTION_011DFC38]
            ]
        );
    }

    #[test]
    fn animation_mob_message_updates_flagged_actors_and_asks_about_others() {
        let mut e = engine();
        quiet(&mut e, &[0x7000_0200]);
        e.register_double(FAKE_IS_ACTOR, |_, _| ok(1));
        let this: Ptr<AnimationTaskData> = e.new_object();
        let actor = object_with_slots(
            &mut e,
            0x1c4,
            &[(0x100, FAKE_IS_ACTOR), (0x268, 0x7000_0200)],
        );
        log_calls(&mut e);
        // Flagged actor: slot +0x268(0.0).
        e.mem.set_u8(actor.addr() + 0xbc, 1);
        e.call(0x008c_8910, &args![this, actor]);
        assert_eq!(calls_to(&e, 0x7000_0200), vec![vec![actor.addr(), 0]]);
        // Unflagged actor: nothing.
        e.mem.set_u8(actor.addr() + 0xbc, 0);
        e.call(0x008c_8910, &args![this, actor]);
        assert_eq!(calls_to(&e, 0x7000_0200).len(), 1);
        // Null mob: nothing.
        e.call(0x008c_8910, &args![this, 0u32]);
        assert_eq!(calls_to(&e, 0x7000_0200).len(), 1);
    }

    #[test]
    fn animation_mob_message_for_a_non_actor_follows_the_other_test() {
        let mut e = engine();
        quiet(&mut e, &[0x7000_0200]);
        e.register_double(FAKE_IS_ACTOR, |_, _| ok(0));
        let this: Ptr<AnimationTaskData> = e.new_object();
        let mob = object_with_slots(
            &mut e,
            0x1c4,
            &[(0x100, FAKE_IS_ACTOR), (0x268, 0x7000_0200)],
        );
        // `00574900` answers false: nothing happens.
        e.register_double(0x0057_4900, |_, _| ok(0));
        log_calls(&mut e);
        e.call(0x008c_8910, &args![this, mob]);
        assert!(calls_to(&e, 0x7000_0200).is_empty());
        // `00574900` answers true for this mob: slot +0x268(0.0).
        let addr = mob.addr();
        e.register_double(0x0057_4900, move |_, a| ok(u32::from(a[0] == addr)));
        e.call(0x008c_8910, &args![this, mob]);
        assert_eq!(calls_to(&e, 0x7000_0200), vec![vec![addr, 0]]);
    }

    // ---- ActorUpdateTaskData -----------------------------------------

    #[test]
    fn actor_update_constructor_sets_the_vtable_and_clears_the_counter() {
        let mut e = engine();
        quiet(
            &mut e,
            &[TASKLET_BASE_CONSTRUCTOR, MESSAGE_QUEUE_CONSTRUCTOR],
        );
        let this: Ptr<ActorUpdateTaskData> = e.new_object();
        e.mem.set_u32(this.addr() + 0x38, 6);
        assert_eq!(
            e.call(0x008c_89a0, &args![this, 2u32]).ptr::<()>(),
            this.cast()
        );
        assert_eq!(e.mem.u32(this.addr()), VTABLE_ACTOR_UPDATE_TASK_DATA);
        assert_eq!(e.mem.u32(this.addr() + 0x38), 0);
    }

    #[test]
    fn actor_update_destructor_resets_the_vtable_and_deletes_on_bit_zero() {
        check_deleting_destructor(0x008c_89d0, VTABLE_ACTOR_UPDATE_TASK_DATA, 0x3c);
    }

    #[test]
    fn actor_update_destructor_body_resets_the_vtable() {
        let mut e = engine();
        quiet(&mut e, &[MESSAGE_QUEUE_DESTRUCTOR, TASKLET_BASE_DESTRUCTOR]);
        let this: Ptr = Ptr::new(e.mem.alloc(0x3c));
        e.call(0x008c_8a00, &args![this]);
        assert_eq!(e.mem.u32(this.addr()), VTABLE_ACTOR_UPDATE_TASK_DATA);
    }

    #[test]
    fn actor_update_count_message_clears_the_counter_and_enters_the_section() {
        let mut e = engine();
        quiet(&mut e, &[CRITICAL_SECTION_ENTER]);
        let this: Ptr<ActorUpdateTaskData> = e.new_object();
        e.mem.set_u32(this.addr() + 0x38, 6);
        log_calls(&mut e);
        e.call(0x008c_8a20, &args![this, 1u32]);
        assert_eq!(e.mem.u32(this.addr() + 0x38), 0);
        assert_eq!(
            calls_to(&e, CRITICAL_SECTION_ENTER),
            vec![vec![CRITICAL_SECTION_011DFC38, 0]]
        );
    }

    #[test]
    fn actor_update_end_message_leaves_the_section() {
        let mut e = engine();
        quiet(&mut e, &[CRITICAL_SECTION_LEAVE]);
        let this: Ptr<ActorUpdateTaskData> = e.new_object();
        log_calls(&mut e);
        e.call(0x008c_8a50, &args![this, 1u32]);
        assert_eq!(
            calls_to(&e, CRITICAL_SECTION_LEAVE),
            vec![vec![CRITICAL_SECTION_011DFC38]]
        );
    }

    #[test]
    fn actor_update_mob_message_updates_flagged_actors_and_counts_all_actors() {
        let mut e = engine();
        quiet(&mut e, &[0x7000_0200]);
        e.register_double(FAKE_IS_ACTOR, |_, _| ok(1));
        let this: Ptr<ActorUpdateTaskData> = e.new_object();
        let actor = object_with_slots(
            &mut e,
            0x1c4,
            &[(0x100, FAKE_IS_ACTOR), (0x2f8, 0x7000_0200)],
        );
        log_calls(&mut e);
        e.mem.set_u8(actor.addr() + 0xbc, 1);
        e.call(0x008c_8a70, &args![this, actor]);
        assert_eq!(calls_to(&e, 0x7000_0200), vec![vec![actor.addr(), 0]]);
        assert_eq!(e.mem.u32(this.addr() + 0x38), 1);
        // Unflagged: counted, not updated.
        e.mem.set_u8(actor.addr() + 0xbc, 0);
        e.call(0x008c_8a70, &args![this, actor]);
        assert_eq!(calls_to(&e, 0x7000_0200).len(), 1);
        assert_eq!(e.mem.u32(this.addr() + 0x38), 2);
        // Null mob: neither.
        e.call(0x008c_8a70, &args![this, 0u32]);
        assert_eq!(e.mem.u32(this.addr() + 0x38), 2);
    }

    #[test]
    fn actor_update_mob_message_ignores_non_actors() {
        let mut e = engine();
        e.register_double(FAKE_IS_ACTOR, |_, _| ok(0));
        let this: Ptr<ActorUpdateTaskData> = e.new_object();
        let mob = object_with_slots(&mut e, 0x1c4, &[(0x100, FAKE_IS_ACTOR)]);
        e.call(0x008c_8a70, &args![this, mob]);
        assert_eq!(e.mem.u32(this.addr() + 0x38), 0);
    }

    // ---- PackageUpdateTaskData ---------------------------------------

    #[test]
    fn package_update_constructor_sets_the_vtable_and_clears_the_counters() {
        let mut e = engine();
        quiet(
            &mut e,
            &[TASKLET_BASE_CONSTRUCTOR, MESSAGE_QUEUE_CONSTRUCTOR],
        );
        let this: Ptr<PackageUpdateTaskData> = e.new_object();
        e.mem.set_u32(this.addr() + 0x3c, 6);
        e.mem.set_u32(this.addr() + 0x40, 6);
        assert_eq!(
            e.call(0x008c_8ae0, &args![this, 2u32]).ptr::<()>(),
            this.cast()
        );
        assert_eq!(e.mem.u32(this.addr()), VTABLE_PACKAGE_UPDATE_TASK_DATA);
        assert_eq!(e.mem.u32(this.addr() + 0x3c), 0);
        assert_eq!(e.mem.u32(this.addr() + 0x40), 0);
    }

    #[test]
    fn package_update_destructor_resets_the_vtable_and_deletes_on_bit_zero() {
        check_deleting_destructor(0x008c_8b20, VTABLE_PACKAGE_UPDATE_TASK_DATA, 0x44);
    }

    #[test]
    fn package_update_destructor_body_resets_the_vtable() {
        let mut e = engine();
        quiet(&mut e, &[MESSAGE_QUEUE_DESTRUCTOR, TASKLET_BASE_DESTRUCTOR]);
        let this: Ptr = Ptr::new(e.mem.alloc(0x44));
        e.call(0x008c_8b50, &args![this]);
        assert_eq!(e.mem.u32(this.addr()), VTABLE_PACKAGE_UPDATE_TASK_DATA);
    }

    #[test]
    fn package_update_count_message_resets_the_counters_and_flags() {
        let mut e = engine();
        quiet(&mut e, &[CRITICAL_SECTION_ENTER]);
        let player: Ptr = Ptr::new(e.mem.alloc(0x700));
        e.mem.set_u8(player.addr() + 0x5f8, 1);
        e.set_global(PLAYER_CHARACTER, player.addr());
        e.set_global(RESET_FLAG_011E01DD, 1u8);
        let this: Ptr<PackageUpdateTaskData> = e.new_object();
        e.mem.set_u32(this.addr() + 0x3c, 6);
        e.mem.set_u32(this.addr() + 0x40, 7);
        log_calls(&mut e);
        e.call(0x008c_8b70, &args![this, 1u32]);
        assert_eq!(e.mem.u32(this.addr() + 0x3c), 0);
        assert_eq!(e.mem.u32(this.addr() + 0x40), 0);
        assert_eq!(e.global::<u8>(RESET_FLAG_011E01DD), 0);
        assert_eq!(e.mem.u8(player.addr() + 0x5f8), 0);
        assert_eq!(
            calls_to(&e, CRITICAL_SECTION_ENTER),
            vec![vec![CRITICAL_SECTION_011DFBE8, 0]]
        );
    }

    #[test]
    fn byte_setters_store_their_byte() {
        let mut e = engine();
        let object: Ptr = Ptr::new(e.mem.alloc(0x700));
        e.call(0x008c_8bb0, &args![object, 0x7fu8]);
        assert_eq!(e.mem.u8(object.addr() + 0x5f8), 0x7f);
        e.call(0x008c_8fd0, &args![object, 1u8]);
        assert_eq!(e.mem.u8(object.addr() + 0x2c7), 1);
        e.set_global(RESET_FLAG_011E01DD, 1u8);
        e.call(0x008c_8bd0, &[]);
        assert_eq!(e.global::<u8>(RESET_FLAG_011E01DD), 0);
    }

    #[test]
    fn package_update_end_message_passes_the_scale_and_leaves_the_section() {
        let mut e = engine();
        e.register(0x0084_d030, |_, _| 0.25f32.into_ret());
        quiet(&mut e, &[0x0097_7130, CRITICAL_SECTION_LEAVE]);
        let this: Ptr<PackageUpdateTaskData> = e.new_object();
        log_calls(&mut e);
        e.call(0x008c_8ff0, &args![this, 0u32]);
        assert_eq!(
            calls_to(&e, 0x0084_d030),
            vec![vec![TASK_SCALE_OBJECT_011F6394]]
        );
        assert_eq!(
            calls_to(&e, 0x0097_7130),
            vec![vec![PROCESS_LISTS, 0.25f32.to_bits()]]
        );
        assert_eq!(
            calls_to(&e, CRITICAL_SECTION_LEAVE),
            vec![vec![CRITICAL_SECTION_011DFBE8]]
        );
    }

    // ---- PackageUpdateTaskData::HandleMobMessage ---------------------

    const FAKE_SLOT_24: u32 = 0x7000_0300;
    const FAKE_SLOT_25C: u32 = 0x7000_0304;

    /// What the package scenario varies.
    struct Package {
        guard_576d30: bool,
        has_process: bool,
        process_type: u32,
        later_process_type: u32,
        is_actor: bool,
        process_me: bool,
        hands_over: bool,
        resting: bool,
    }

    impl Package {
        fn new() -> Package {
            Package {
                guard_576d30: false,
                has_process: true,
                process_type: 0,
                later_process_type: 0,
                is_actor: true,
                process_me: true,
                hands_over: false,
                resting: false,
            }
        }
    }

    /// A node-based list model for the follower clean-up: the list object
    /// is its own first node (`item` at +0, `next` at +4).
    fn list_append(e: &mut Engine, list: u32, item: u32) {
        if e.mem.u32(list) == 0 {
            e.mem.set_u32(list, item);
            return;
        }
        let mut node = list;
        while e.mem.u32(node + 4) != 0 {
            node = e.mem.u32(node + 4);
        }
        let fresh = e.mem.alloc(8);
        e.mem.set_u32(fresh, item);
        e.mem.set_u32(node + 4, fresh);
    }

    const OWNER_EXTRA: u32 = 0x7300_0077;
    /// The player's address in the package scenario.
    const PACKAGE_PLAYER: u32 = 0x7500_0000;
    const NODE_DATA: u32 = 0x0068_15c0;
    const NODE_NEXT: u32 = 0x0072_6070;

    fn run_package(
        scene: &Package,
        owner_items: &[u32],
    ) -> (Engine, Ptr<PackageUpdateTaskData>, Ptr, Ptr, u32) {
        let mut e = engine();
        e.set_global(RESTORE_AMOUNT, 2.0f32);
        e.map(PACKAGE_PLAYER, 0x1000);
        let player: Ptr = Ptr::new(PACKAGE_PLAYER);
        e.set_global(PLAYER_CHARACTER, player.addr());

        let process = object_with_slots(&mut e, 0x300, &[(0x24, FAKE_SLOT_24)]);
        let mob = object_with_slots(
            &mut e,
            0x1c4,
            &[(0x100, FAKE_IS_ACTOR), (0x25c, FAKE_SLOT_25C)],
        );
        e.mem.set_u8(mob.addr() + 0xbc, u8::from(scene.process_me));
        quiet(&mut e, &[FAKE_SLOT_24, FAKE_SLOT_25C]);
        let is_actor = scene.is_actor;
        e.register_double(FAKE_IS_ACTOR, move |_, _| ok(u32::from(is_actor)));
        let guard = scene.guard_576d30;
        e.register_double(0x0057_6d30, move |_, _| ok(u32::from(guard)));
        e.register(0x0044_0d80, |_, _| ok(0));
        e.register(0x0044_0da0, |_, _| ok(0));
        let (has_process, process_addr) = (scene.has_process, process.addr());
        e.register_double(GET_PROCESS, move |_, _| {
            ok(if has_process { process_addr } else { 0 })
        });
        // The first answer is the guard's, later ones are the re-check
        // after the update.
        let (process_type, later_type) = (scene.process_type, scene.later_process_type);
        let mut asked = 0;
        e.register_double(GET_CURRENT_PROCESS_TYPE, move |_, _| {
            asked += 1;
            ok(if asked == 1 { process_type } else { later_type })
        });
        let hands_over = scene.hands_over;
        e.register_double(0x008b_0520, move |_, _| ok(u32::from(hands_over)));
        let resting = scene.resting;
        e.register_double(0x0094_df60, move |_, _| ok(u32::from(resting)));
        quiet(&mut e, &[0x0088_b510, 0x0088_b5a0]);

        // Follower clean-up doubles: a real node model.
        let owner = e.mem.alloc(0x20);
        let first = e.mem.alloc(8);
        for item in owner_items {
            list_append(&mut e, first, *item);
        }
        e.mem.set_u32(owner + 0xc, first);
        e.register_double(0x005d_43c0, |_, _| ok(OWNER_EXTRA));
        e.register_double(0x0042_2700, move |_, _| ok(owner));
        e.register(NODE_DATA, |_, a| ok(a[0]));
        e.register(NODE_NEXT, |e, a| ok(e.mem.u32(a[0] + 4)));
        e.register(0x0096_a2d0, |_, a| ok(a[0]));
        e.register(0x0090_5820, |e, a| {
            let item = e.mem.u32(a[1]);
            list_append(e, a[0], item);
            Ret::default()
        });
        e.register(0x005a_e3d0, |e, a| {
            let item = e.mem.u32(a[1]);
            list_append(e, a[0], item);
            Ret::default()
        });
        // `could follow`: true for the item 0xb000.
        e.register(0x008b_c860, |_, a| ok(u32::from(a[0] == 0xb000)));
        quiet(
            &mut e,
            &[0x0042_2690, 0x0047_0470, 0x0047_02f0, 0x0042_2720],
        );
        e.register(0x0082_56d0, |_, a| ok(u32::from(a[0] != 0)));

        let this: Ptr<PackageUpdateTaskData> = e.new_object();
        e.mem.set_u32(this.addr() + 0x18, 0x5555);
        log_calls(&mut e);
        e.call(0x008c_8be0, &args![this, mob]);
        (e, this, mob, player, owner)
    }

    fn counters(e: &Engine, this: Ptr<PackageUpdateTaskData>) -> (u32, u32) {
        (e.mem.u32(this.addr() + 0x3c), e.mem.u32(this.addr() + 0x40))
    }

    #[test]
    fn package_mob_message_skips_mobs_the_guards_reject() {
        for scene in [
            Package {
                guard_576d30: true,
                ..Package::new()
            },
            Package {
                has_process: false,
                ..Package::new()
            },
            Package {
                process_type: 2,
                ..Package::new()
            },
        ] {
            let (e, this, _, _, _) = run_package(&scene, &[]);
            assert_eq!(counters(&e, this), (0, 0));
            assert!(calls_to(&e, FAKE_SLOT_25C).is_empty());
            assert!(calls_to(&e, FAKE_SLOT_24).is_empty());
            assert_eq!(e.mem.u32(this.addr() + 0x18), 0x5555);
        }
    }

    #[test]
    fn package_mob_message_ignores_actors_not_flagged_for_processing() {
        let scene = Package {
            process_me: false,
            ..Package::new()
        };
        let (e, this, _, _, _) = run_package(&scene, &[]);
        assert_eq!(counters(&e, this), (0, 0));
        assert!(calls_to(&e, FAKE_SLOT_25C).is_empty());
    }

    #[test]
    fn package_mob_message_hands_the_actor_to_its_process() {
        let scene = Package {
            hands_over: true,
            resting: true,
            ..Package::new()
        };
        let (e, this, mob, _, _) = run_package(&scene, &[0xa000]);
        assert_eq!(counters(&e, this), (0, 1));
        assert_eq!(calls_to(&e, FAKE_SLOT_24).len(), 1);
        assert_eq!(calls_to(&e, FAKE_SLOT_24)[0][1..], [mob.addr(), 0]);
        assert!(calls_to(&e, FAKE_SLOT_25C).is_empty());
        let process = calls_to(&e, FAKE_SLOT_24)[0][0];
        assert_eq!(e.mem.u8(process + 0x2c7), 1);
        // The rest restores health and fatigue by the same amount.
        assert_eq!(
            calls_to(&e, 0x0088_b510),
            vec![vec![mob.addr(), 2.0f32.to_bits()]]
        );
        assert_eq!(
            calls_to(&e, 0x0088_b5a0),
            vec![vec![mob.addr(), 2.0f32.to_bits()]]
        );
        // Handed over: no follower clean-up.
        assert!(calls_to(&e, 0x0042_2690).is_empty());
    }

    #[test]
    fn package_mob_message_plain_update_cleans_up_followers() {
        let scene = Package::new();
        let (e, this, mob, _, _) = run_package(&scene, &[0xa000, 0, 0xb000, 0xc000]);
        // The list ends at the first null entry, as the game's loops do.
        assert_eq!(counters(&e, this), (1, 0));
        assert_eq!(calls_to(&e, FAKE_SLOT_25C), vec![vec![mob.addr(), 0]]);
        assert!(calls_to(&e, 0x0088_b510).is_empty());
        assert_eq!(calls_to(&e, 0x0042_2690), vec![vec![OWNER_EXTRA, 0xa000]]);
        // Both temporary lists are cleared and freed.
        assert_eq!(calls_to(&e, 0x0047_0470).len(), 2);
        assert_eq!(calls_to(&e, 0x0047_02f0).len(), 2);
        // The follower extra is removed once the owner's list is checked.
        assert_eq!(calls_to(&e, 0x0042_2720), vec![vec![OWNER_EXTRA]]);
    }

    #[test]
    fn package_mob_message_follower_cleanup_filters_the_player_and_followers() {
        let scene = Package::new();
        let (e, _, _, _, _) = run_package(&scene, &[PACKAGE_PLAYER, 0xb000, 0xc000, 0xd000]);
        // The player and `0xb000` (which could follow) are skipped.
        assert_eq!(
            calls_to(&e, 0x0042_2690),
            vec![vec![OWNER_EXTRA, 0xc000], vec![OWNER_EXTRA, 0xd000]]
        );
    }

    #[test]
    fn package_mob_message_without_a_follower_owner_does_no_cleanup() {
        // `005d43c0` answers 0 for the guard-free mob: no lists are built.
        let scene = Package::new();
        let (mut e, _, _, _, _) = run_package(&scene, &[0xa000]);
        e.register_double(0x005d_43c0, |_, _| ok(0));
        let this: Ptr<PackageUpdateTaskData> = e.new_object();
        let mob = object_with_slots(
            &mut e,
            0x1c4,
            &[(0x100, FAKE_IS_ACTOR), (0x25c, FAKE_SLOT_25C)],
        );
        e.mem.set_u8(mob.addr() + 0xbc, 1);
        log_calls(&mut e);
        e.call(0x008c_8be0, &args![this, mob]);
        assert!(calls_to(&e, 0x0042_2690).is_empty());
        assert!(calls_to(&e, 0x0047_0470).is_empty());
        assert_eq!(e.mem.u32(this.addr() + 0x3c), 1);
    }

    #[test]
    fn package_mob_message_clears_the_mob_list_when_the_process_type_changes() {
        let scene = Package {
            later_process_type: 1,
            ..Package::new()
        };
        let (e, this, _, _, _) = run_package(&scene, &[]);
        assert_eq!(e.mem.u32(this.addr() + 0x18), 0);

        let scene = Package::new();
        let (e, this, _, _, _) = run_package(&scene, &[]);
        assert_eq!(e.mem.u32(this.addr() + 0x18), 0x5555);
    }

    // ---- ActorsScriptTaskData ----------------------------------------

    #[test]
    fn actors_script_constructor_sets_the_vtable_and_clears_the_counter() {
        let mut e = engine();
        quiet(
            &mut e,
            &[TASKLET_BASE_CONSTRUCTOR, MESSAGE_QUEUE_CONSTRUCTOR],
        );
        let this: Ptr<ActorsScriptTaskData> = e.new_object();
        e.mem.set_u32(this.addr() + 0x3c, 6);
        assert_eq!(
            e.call(0x008c_9020, &args![this, 2u32]).ptr::<()>(),
            this.cast()
        );
        assert_eq!(e.mem.u32(this.addr()), VTABLE_ACTORS_SCRIPT_TASK_DATA);
        assert_eq!(e.mem.u32(this.addr() + 0x3c), 0);
    }

    #[test]
    fn actors_script_destructor_resets_the_vtable_and_deletes_on_bit_zero() {
        check_deleting_destructor(0x008c_9050, VTABLE_ACTORS_SCRIPT_TASK_DATA, 0x40);
    }

    #[test]
    fn actors_script_destructor_body_resets_the_vtable() {
        let mut e = engine();
        quiet(&mut e, &[MESSAGE_QUEUE_DESTRUCTOR, TASKLET_BASE_DESTRUCTOR]);
        let this: Ptr = Ptr::new(e.mem.alloc(0x40));
        e.call(0x008c_9080, &args![this]);
        assert_eq!(e.mem.u32(this.addr()), VTABLE_ACTORS_SCRIPT_TASK_DATA);
    }

    #[test]
    fn actors_script_count_message_clears_the_counter_and_enters_three_sections() {
        let mut e = engine();
        quiet(&mut e, &[CRITICAL_SECTION_ENTER]);
        let this: Ptr<ActorsScriptTaskData> = e.new_object();
        e.mem.set_u32(this.addr() + 0x3c, 6);
        log_calls(&mut e);
        e.call(0x008c_90a0, &args![this, 1u32]);
        assert_eq!(e.mem.u32(this.addr() + 0x3c), 0);
        assert_eq!(
            calls_to(&e, CRITICAL_SECTION_ENTER),
            vec![
                vec![CRITICAL_SECTION_011DFC38, 0],
                vec![CRITICAL_SECTION_011DFC04, 0],
                vec![CRITICAL_SECTION_011DFBE8, 0]
            ]
        );
    }

    #[test]
    fn actors_script_end_message_leaves_three_sections() {
        let mut e = engine();
        quiet(&mut e, &[CRITICAL_SECTION_LEAVE]);
        let this: Ptr<ActorsScriptTaskData> = e.new_object();
        log_calls(&mut e);
        e.call(0x008c_90e0, &args![this, 1u32]);
        assert_eq!(
            calls_to(&e, CRITICAL_SECTION_LEAVE),
            vec![
                vec![CRITICAL_SECTION_011DFBE8],
                vec![CRITICAL_SECTION_011DFC04],
                vec![CRITICAL_SECTION_011DFC38]
            ]
        );
    }

    #[test]
    fn actors_script_mob_message_runs_scripts_only_when_enabled() {
        let mut e = engine();
        e.register_double(FAKE_IS_ACTOR, |_, _| ok(1));
        quiet(&mut e, &[0x0056_5870]);
        let enabled = Rc::new(RefCell::new(true));
        let flag = enabled.clone();
        e.register_double(0x005a_c740, move |_, _| ok(u32::from(*flag.borrow())));
        let this: Ptr<ActorsScriptTaskData> = e.new_object();
        let mob = object_with_slots(&mut e, 0x1c4, &[(0x100, FAKE_IS_ACTOR)]);
        log_calls(&mut e);
        e.call(0x008c_9110, &args![this, mob]);
        assert_eq!(calls_to(&e, 0x0056_5870), vec![vec![mob.addr()]]);
        assert_eq!(e.mem.u32(this.addr() + 0x3c), 1);
        *enabled.borrow_mut() = false;
        e.call(0x008c_9110, &args![this, mob]);
        assert_eq!(calls_to(&e, 0x0056_5870).len(), 1);
        assert_eq!(e.mem.u32(this.addr() + 0x3c), 1);
        // A null mob does nothing.
        e.call(0x008c_9110, &args![this, 0u32]);
        assert_eq!(e.mem.u32(this.addr() + 0x3c), 1);
    }

    #[test]
    fn movement_constructor_passes_the_shared_word_and_clears_its_counter() {
        let mut e = engine();
        quiet(
            &mut e,
            &[TASKLET_BASE_CONSTRUCTOR, MESSAGE_QUEUE_CONSTRUCTOR],
        );
        let this: Ptr<MovementTaskData> = Ptr::new(e.mem.alloc(0x3c));
        log_calls(&mut e);
        e.call(0x008c_9170, &args![this, 0x4321u32]);
        assert_eq!(e.mem.u32(this.addr()), VTABLE_MOVEMENT_TASK_DATA);
        assert_eq!(e.mem.u32(this.addr() + 0x38), 0);
        assert_eq!(
            calls_to(&e, MESSAGE_QUEUE_CONSTRUCTOR),
            vec![vec![this.addr() + 0x1c, 0x4321]]
        );
    }

    #[test]
    fn movement_scalar_destructor_deletes_only_when_bit_zero_is_set() {
        let mut e = engine();
        quiet(
            &mut e,
            &[
                MESSAGE_QUEUE_DESTRUCTOR,
                TASKLET_BASE_DESTRUCTOR,
                OPERATOR_DELETE,
            ],
        );
        let this: Ptr<MovementTaskData> = Ptr::new(e.mem.alloc(0x3c));
        log_calls(&mut e);
        assert_eq!(
            e.call(0x008c_91a0, &args![this, 2u32]).ptr::<()>(),
            this.cast()
        );
        assert!(calls_to(&e, OPERATOR_DELETE).is_empty());
        assert_eq!(e.mem.u32(this.addr()), VTABLE_MOVEMENT_TASK_DATA);
        e.call(0x008c_91a0, &args![this, 3u32]);
        assert_eq!(calls_to(&e, OPERATOR_DELETE), vec![vec![this.addr()]]);
    }

    #[test]
    fn movement_destructor_restores_vtable_before_destroying_the_base() {
        let mut e = engine();
        quiet(&mut e, &[MESSAGE_QUEUE_DESTRUCTOR, TASKLET_BASE_DESTRUCTOR]);
        let this: Ptr<MovementTaskData> = Ptr::new(e.mem.alloc(0x3c));
        log_calls(&mut e);
        e.call(0x008c_91d0, &args![this]);
        assert_eq!(e.mem.u32(this.addr()), VTABLE_MOVEMENT_TASK_DATA);
        assert_eq!(
            calls_to(&e, MESSAGE_QUEUE_DESTRUCTOR),
            vec![vec![this.addr() + 0x1c]]
        );
        assert_eq!(
            calls_to(&e, TASKLET_BASE_DESTRUCTOR),
            vec![vec![this.addr()]]
        );
    }

    #[test]
    fn movement_count_resets_before_entering_both_sections() {
        let mut e = engine();
        quiet(&mut e, &[CRITICAL_SECTION_ENTER]);
        let this: Ptr<MovementTaskData> = e.new_object();
        e.set(this, MovementTaskData::iUpdated, 19);
        log_calls(&mut e);
        e.call(0x008c_91f0, &args![this, 4u32]);
        assert_eq!(e.get(this, MovementTaskData::iUpdated), 0);
        assert_eq!(
            calls_to(&e, CRITICAL_SECTION_ENTER),
            vec![
                vec![CRITICAL_SECTION_011DFC04, 0],
                vec![CRITICAL_SECTION_011DFBE8, 0],
            ]
        );
    }

    #[test]
    fn movement_mob_routes_by_actor_state_and_increments_for_each_message() {
        const ACTOR_UPDATE: u32 = 0x7000_0220;
        const NONACTOR_A: u32 = 0x7000_0224;
        const NONACTOR_B: u32 = 0x7000_0228;
        const ACTOR_CHECK: u32 = 0x7000_022c;
        let mut e = engine();
        e.register_double(ACTOR_CHECK, |_, _| ok(1));
        let sleeping_calls = Rc::new(RefCell::new(Vec::new()));
        let seen_sleeping = sleeping_calls.clone();
        e.register_double(0x0094_df60, move |_, a| {
            seen_sleeping.borrow_mut().push(a.to_vec());
            ok(1)
        });
        quiet(&mut e, &[ACTOR_UPDATE, 0x009b_ec10, 0x009a_e580]);
        let actor = object_with_slots(
            &mut e,
            0x1c4,
            &[(0x100, ACTOR_CHECK), (0x348, ACTOR_UPDATE)],
        );
        e.mem.set_u8(actor.addr() + ACTOR_PROCESS_ME, 1);
        let player: Ptr = Ptr::new(e.mem.alloc(0x40));
        e.set_global(PLAYER_CHARACTER, player.addr());
        let not_actor = object_with_slots(
            &mut e,
            0x40,
            &[
                (0x100, ACTOR_CHECK),
                (0x224, NONACTOR_A),
                (0x220, NONACTOR_B),
            ],
        );
        let check_calls = Rc::new(RefCell::new(0));
        let checks = check_calls.clone();
        e.register_double(ACTOR_CHECK, move |_, _| {
            let call = *checks.borrow();
            *checks.borrow_mut() += 1;
            ok(u32::from(call == 0))
        });
        e.register_double(NONACTOR_A, |_, _| ok(0));
        e.register_double(NONACTOR_B, |_, _| ok(1));
        let this: Ptr<MovementTaskData> = e.new_object();
        log_calls(&mut e);
        e.call(0x008c_9220, &args![this, actor]);
        e.call(0x008c_9220, &args![this, not_actor]);
        assert_eq!(e.mem.u32(this.addr() + 0x38), 2);
        assert_eq!(*sleeping_calls.borrow(), vec![vec![player.addr()]]);
        assert_eq!(
            calls_to(&e, ACTOR_UPDATE),
            vec![vec![actor.addr(), 0.0f32.to_bits(), 1]]
        );
        assert_eq!(
            calls_to(&e, 0x009a_e580),
            vec![vec![not_actor.addr(), 0.0f32.to_bits()]]
        );
    }

    #[test]
    fn movement_end_leaves_sections_in_the_executable_order() {
        let mut e = engine();
        quiet(&mut e, &[CRITICAL_SECTION_LEAVE]);
        let this: Ptr<MovementTaskData> = e.new_object();
        log_calls(&mut e);
        e.call(0x008c_9300, &args![this, 0u32]);
        assert_eq!(
            calls_to(&e, CRITICAL_SECTION_LEAVE),
            vec![
                vec![CRITICAL_SECTION_011DFBE8],
                vec![CRITICAL_SECTION_011DFC04],
            ]
        );
    }

    #[test]
    fn manager_constructor_initializes_each_tasklet_and_clears_its_worker_flag() {
        let mut e = engine();
        quiet(
            &mut e,
            &[
                TASKLET_BASE_CONSTRUCTOR,
                MESSAGE_QUEUE_CONSTRUCTOR,
                0x008c_a680,
            ],
        );
        let this: Ptr = Ptr::new(e.mem.alloc(0x10184));
        log_calls(&mut e);
        assert_eq!(e.call(0x008c_9330, &args![this]).ptr::<()>(), this.cast());
        assert_eq!(e.mem.u8(this.addr() + 0x10180), 0);
        assert_eq!(calls_to(&e, TASKLET_BASE_CONSTRUCTOR).len(), 6);
        assert_eq!(
            calls_to(&e, MESSAGE_QUEUE_CONSTRUCTOR),
            [0u32, 0x40, 0x78, 0xbc, 0xf8, 0x138]
                .into_iter()
                .map(|offset| vec![this.addr() + offset + 0x1c, this.addr() + 0x174])
                .collect::<Vec<_>>()
        );
        assert_eq!(calls_to(&e, 0x008c_a680), vec![vec![this.addr() + 0x174]]);
    }

    #[test]
    fn manager_destructor_walks_embedded_tasklets_in_reverse_order() {
        let mut e = engine();
        quiet(
            &mut e,
            &[
                MESSAGE_QUEUE_DESTRUCTOR,
                TASKLET_BASE_DESTRUCTOR,
                0x00ec_5fce,
            ],
        );
        let this: Ptr = Ptr::new(e.mem.alloc(0x10184));
        log_calls(&mut e);
        e.call(0x008c_9430, &args![this]);
        assert_eq!(calls_to(&e, MESSAGE_QUEUE_DESTRUCTOR).len(), 6);
        assert_eq!(calls_to(&e, TASKLET_BASE_DESTRUCTOR).len(), 6);
        assert_eq!(
            calls_to(&e, 0x00ec_5fce),
            vec![vec![this.addr() + 0x174 + 0x0c, 0x10, 0x1000, 0x0048_3710]]
        );
        let cleanup_order: Vec<(u32, Vec<u32>)> = e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| {
                matches!(
                    *address,
                    0x00ec_5fce | MESSAGE_QUEUE_DESTRUCTOR | TASKLET_BASE_DESTRUCTOR
                )
            })
            .cloned()
            .collect();
        let expected = [0x138u32, 0xf8, 0xbc, 0x78, 0x40, 0];
        let mut expected_order = vec![(
            0x00ec_5fce,
            vec![this.addr() + 0x174 + 0x0c, 0x10, 0x1000, 0x0048_3710],
        )];
        for offset in expected {
            expected_order.push((MESSAGE_QUEUE_DESTRUCTOR, vec![this.addr() + offset + 0x1c]));
            expected_order.push((TASKLET_BASE_DESTRUCTOR, vec![this.addr() + offset]));
        }
        assert_eq!(cleanup_order, expected_order);
    }

    #[test]
    fn new_queue_item_initializes_all_three_words() {
        let mut e = engine();
        let item: Ptr = Ptr::new(e.mem.alloc(12));
        for offset in [0, 4, 8] {
            e.mem.set_u32(item.addr() + offset, 0xeeee);
        }
        assert_eq!(
            e.call(0x008c_9a20, &args![item, 0x1234u32]).ptr::<()>(),
            item.cast()
        );
        assert_eq!(
            [0, 4, 8].map(|offset| e.mem.u32(item.addr() + offset)),
            [0, 0x1234, 0]
        );
    }

    #[test]
    fn existing_queue_item_leaves_the_new_task_word_untouched() {
        let mut e = engine();
        let item: Ptr = Ptr::new(e.mem.alloc(12));
        e.mem.set_u32(item.addr() + 4, 0xeeee);
        assert_eq!(
            e.call(0x008c_9a50, &args![item, 0x5678u32]).ptr::<()>(),
            item.cast()
        );
        assert_eq!(e.mem.u32(item.addr()), 1);
        assert_eq!(e.mem.u32(item.addr() + 4), 0xeeee);
        assert_eq!(e.mem.u32(item.addr() + 8), 0x5678);
    }

    #[test]
    fn task_queue_appends_with_one_wrap_and_stops_at_capacity() {
        let mut e = engine();
        let queue: Ptr = Ptr::new(e.mem.alloc(0x48));
        e.mem.set_i32(queue.addr() + 0x40, 9);
        e.mem.set_i32(queue.addr() + 0x44, 4);
        e.call(0x008c_9a80, &args![queue, 0xaaaa_u32]);
        assert_eq!(e.mem.u32(queue.addr() + 0x18 + 3 * 4), 0xaaaa);
        assert_eq!(e.mem.i32(queue.addr() + 0x40), 10);
        e.call(0x008c_9a80, &args![queue, 0xbbbb_u32]);
        assert_eq!(e.mem.i32(queue.addr() + 0x40), 10);
        assert_eq!(e.mem.u32(queue.addr() + 0x18 + 4 * 4), 0);
    }

    #[test]
    fn manager_process_clears_start_flag_and_runs_final_setting_branch() {
        let mut e = engine();
        e.map(0x011a_3000, 0x1000);
        e.map(0x011f_1000, 0x1000);
        e.map(0x011c_3000, 0x1000);
        e.map(0x011d_1000, 0x1000);
        let setting_byte_data = e.mem.alloc(1);
        e.mem.set_u8(setting_byte_data, 0);
        let setting_a = e.mem.alloc(4);
        let setting_b = e.mem.alloc(4);
        e.mem.set_u8(0x011a_34e7, 1);
        e.mem.set_u8(0x011f_1220, 0);
        e.mem.set_i32(setting_a, 1);
        e.mem.set_i32(setting_b, 42);
        e.register_double(0x0040_8d60, move |_, a| {
            assert_eq!(a, &[0x011d_fb68]);
            ok(setting_byte_data)
        });
        e.register_double(0x0043_d4d0, move |_, a| {
            ok(match a[0] {
                0x011c_3ea4 => setting_a,
                0x011d_10d4 => setting_b,
                _ => 0,
            })
        });
        quiet(&mut e, &[0x0070_34c0, 0x0070_3490]);
        let this: Ptr = Ptr::new(e.mem.alloc(0x10184));
        log_calls(&mut e);
        e.call(0x008c_94e0, &args![this]);
        assert_eq!(e.mem.u8(0x011a_34e7), 0);
        assert_eq!(calls_to(&e, 0x0070_34c0), vec![vec![42]]);
        assert_eq!(calls_to(&e, 0x0070_3490), vec![vec![]]);
    }

    #[test]
    fn thread_manager_adds_at_most_three_workers_and_initializes_the_new_one() {
        let mut e = engine();
        quiet(
            &mut e,
            &[
                0x00aa_6360,
                0x0086_c6a0,
                0x00c8_5770,
                0x00aa_6430,
                0x00aa_6550,
                0x00aa_6540,
            ],
        );
        let new_address = Rc::new(RefCell::new(0));
        let returned = new_address.clone();
        e.register_double(OPERATOR_NEW, move |_, _| ok(*returned.borrow()));
        let manager: Ptr = Ptr::new(e.mem.alloc(0x10));
        let thread_addr = e.mem.alloc(0x78);
        *new_address.borrow_mut() = thread_addr;
        log_calls(&mut e);
        e.call(0x008c_9ae0, &args![manager, 2u32]);
        assert_eq!(e.mem.u32(manager.addr()), thread_addr);
        assert_eq!(e.mem.i32(manager.addr() + 0x0c), 1);
        assert_eq!(e.mem.u32(thread_addr + 0x4c), 2);
        assert_eq!(e.mem.u8(thread_addr + 0x48), 0);
        assert_eq!(
            calls_to(&e, 0x00aa_6430),
            vec![vec![thread_addr, 0x10000, 0, 0]]
        );
        e.mem.set_i32(manager.addr() + 0x0c, 3);
        let prior_calls = e.call_log.as_ref().unwrap().len();
        e.call(0x008c_9ae0, &args![manager, 1u32]);
        assert_eq!(e.call_log.as_ref().unwrap().len(), prior_calls + 1);
        assert_eq!(calls_to(&e, OPERATOR_NEW).len(), 1);
    }

    #[test]
    fn worker_constructor_passes_semaphore_addresses_and_sets_index() {
        let mut e = engine();
        quiet(&mut e, &[0x00aa_6360, 0x0086_c6a0, 0x00c8_5770]);
        let this: Ptr = Ptr::new(e.mem.alloc(0x78));
        log_calls(&mut e);
        e.call(0x008c_9bd0, &args![this, 5u32]);
        assert_eq!(e.mem.u32(this.addr()), 0x0108_5818);
        assert_eq!(e.mem.u32(this.addr() + 0x4c), 5);
        assert_eq!(e.mem.u8(this.addr() + 0x48), 0);
        assert_eq!(
            calls_to(&e, 0x0086_c6a0),
            vec![
                vec![this.addr() + 0x30, 0, 10],
                vec![this.addr() + 0x3c, 0, 10]
            ]
        );
    }

    #[test]
    fn task_queue_dequeue_wraps_the_head_and_leaves_empty_count_unchanged() {
        let mut e = engine();
        quiet(&mut e, &[CRITICAL_SECTION_ENTER, CRITICAL_SECTION_LEAVE]);
        let queue: Ptr = Ptr::new(0x011d_fb98);
        e.mem.set_i32(queue.addr() + 0x40, 2);
        e.mem.set_i32(queue.addr() + 0x44, 9);
        e.mem.set_u32(queue.addr() + 0x18 + 9 * 4, 0xaaa0);
        e.mem.set_u32(queue.addr() + 0x18, 0xbbb0);
        log_calls(&mut e);
        assert_eq!(e.call(0x008c_9cf0, &args![queue]).u32(), 0xaaa0);
        assert_eq!(e.mem.i32(queue.addr() + 0x44), 0);
        assert_eq!(e.call(0x008c_9cf0, &args![queue]).u32(), 0xbbb0);
        assert_eq!(e.mem.i32(queue.addr() + 0x40), 0);
        assert_eq!(e.call(0x008c_9cf0, &args![queue]).u32(), 0);
        assert_eq!(e.mem.i32(queue.addr() + 0x44), 1);
        assert_eq!(calls_to(&e, CRITICAL_SECTION_ENTER).len(), 3);
        assert_eq!(calls_to(&e, CRITICAL_SECTION_LEAVE).len(), 3);
    }

    #[test]
    fn context_scalar_destructor_deletes_only_when_requested() {
        let mut e = engine();
        quiet(
            &mut e,
            &[0x0052_d240, 0x0055_a2d0, 0x00aa_63c0, OPERATOR_DELETE],
        );
        let this: Ptr = Ptr::new(e.mem.alloc(0x78));
        log_calls(&mut e);
        e.call(0x008c_9ec0, &args![this, 0u32]);
        assert!(calls_to(&e, OPERATOR_DELETE).is_empty());
        e.call(0x008c_9ec0, &args![this, 1u32]);
        assert_eq!(calls_to(&e, OPERATOR_DELETE), vec![vec![this.addr()]]);
    }

    #[test]
    fn context_destructor_destroys_router_semaphores_then_base() {
        let mut e = engine();
        quiet(&mut e, &[0x0052_d240, 0x0055_a2d0, 0x00aa_63c0]);
        let this: Ptr = Ptr::new(e.mem.alloc(0x78));
        log_calls(&mut e);
        e.call(0x008c_9ef0, &args![this]);
        assert_eq!(
            e.call_log.as_ref().unwrap(),
            &vec![
                (0x008c_9ef0, vec![this.addr()]),
                (0x0052_d240, vec![this.addr() + 0x50]),
                (0x0055_a2d0, vec![this.addr() + 0x3c]),
                (0x0055_a2d0, vec![this.addr() + 0x30]),
                (0x00aa_63c0, vec![this.addr()]),
            ]
        );
    }

    #[test]
    fn thread_manager_release_walks_each_registered_pointer() {
        let mut e = engine();
        quiet(&mut e, &[0x0044_2550]);
        let manager: Ptr = Ptr::new(e.mem.alloc(0x10));
        e.mem.set_i32(manager.addr() + 0x0c, 2);
        e.mem.set_u32(manager.addr(), 0x7100_0000);
        e.mem.set_u32(manager.addr() + 4, 0x7100_0080);
        log_calls(&mut e);
        e.call(0x008c_9f70, &args![manager]);
        assert_eq!(
            calls_to(&e, 0x0044_2550),
            vec![vec![0x7100_0030], vec![0x7100_00b0]]
        );
    }

    #[test]
    fn thread_manager_wake_signals_each_registered_pointer() {
        let mut e = engine();
        quiet(&mut e, &[0x008c_7490]);
        let manager: Ptr = Ptr::new(e.mem.alloc(0x10));
        e.mem.set_i32(manager.addr() + 0x0c, 2);
        e.mem.set_u32(manager.addr(), 0x7100_0000);
        e.mem.set_u32(manager.addr() + 4, 0x7100_0080);
        log_calls(&mut e);
        e.call(0x008c_9fd0, &args![manager]);
        assert_eq!(
            calls_to(&e, 0x008c_7490),
            vec![vec![0x7100_0000], vec![0x7100_0080]]
        );
    }

    #[test]
    fn worker_release_uses_its_start_semaphore() {
        let mut e = engine();
        quiet(&mut e, &[0x0044_2550]);
        let this: Ptr = Ptr::new(e.mem.alloc(0x78));
        log_calls(&mut e);
        e.call(0x008c_9fb0, &args![this]);
        assert_eq!(calls_to(&e, 0x0044_2550), vec![vec![this.addr() + 0x30]]);
    }

    #[test]
    fn queue_push_uses_the_queue_vtable_slot_four() {
        let mut e = engine();
        quiet(&mut e, &[QUEUE_RUN]);
        let pushed: Ptr = Ptr::new(0x7100_0040);
        let queue: Ptr = Ptr::new(e.mem.alloc(0x30));
        let table = e.mem.alloc(0x40);
        e.mem.set_u32(table + 4, QUEUE_RUN);
        e.mem.set_u32(queue.addr() + 0x1c, table);
        log_calls(&mut e);
        e.call(0x008c_a010, &args![queue, pushed]);
        assert_eq!(
            calls_to(&e, QUEUE_RUN),
            vec![vec![queue.addr() + 0x1c, pushed.addr()]]
        );
    }

    #[test]
    fn queue_push_end_sends_a_zeroed_end_message() {
        let mut e = engine();
        let seen = Rc::new(RefCell::new(Vec::new()));
        let captured = seen.clone();
        e.register_double(QUEUE_RUN, move |e, a| {
            captured.borrow_mut().push(vec![
                a[0],
                a[1],
                e.mem.u32(a[1]),
                e.mem.u32(a[1] + 4),
                e.mem.u32(a[1] + 8),
            ]);
            Ret::default()
        });
        let queue: Ptr = Ptr::new(e.mem.alloc(0x30));
        let table = e.mem.alloc(0x40);
        e.mem.set_u32(table + 4, QUEUE_RUN);
        e.mem.set_u32(queue.addr() + 0x1c, table);
        e.call(0x008c_a040, &args![queue]);
        let captured = seen.borrow();
        assert_eq!(captured.len(), 1);
        assert_eq!(captured[0][0], queue.addr() + 0x1c);
        assert_eq!(&captured[0][2..], &[2, 0, 0]);
    }

    #[test]
    fn release_semaphore_helper_uses_the_indexed_thread() {
        let mut e = engine();
        quiet(&mut e, &[0x0044_2550]);
        let threads: Ptr = Ptr::new(0x011d_fc24);
        e.mem.map(0x7100_0000, 0x80);
        e.mem.set_u32(threads.addr() + 8, 0x7100_0000);
        e.mem.set_u32(0x7100_0030, 0xaaaa);
        log_calls(&mut e);
        e.call(0x008c_a2e0, &args![threads, 2u32]);
        assert_eq!(calls_to(&e, 0x0044_2550), vec![vec![0x7100_0030]]);
    }

    #[test]
    fn clear_parallel_flag_signals_index_two_then_clears_the_byte() {
        let mut e = engine();
        quiet(&mut e, &[0x008c_7490]);
        e.mem.set_u32(0x011d_fc24 + 8, 0xaaaa);
        let manager: Ptr = Ptr::new(e.mem.alloc(0x10184));
        e.mem.set_u8(manager.addr() + 0x10180, 1);
        log_calls(&mut e);
        e.call(0x008c_a300, &args![manager]);
        assert_eq!(e.mem.u8(manager.addr() + 0x10180), 0);
        assert_eq!(calls_to(&e, 0x008c_7490), vec![vec![0xaaaa]]);
        e.call(0x008c_a300, &args![manager]);
        assert_eq!(calls_to(&e, 0x008c_7490).len(), 1);
    }

    #[test]
    fn manager_signal_helper_signals_the_indexed_thread() {
        let mut e = engine();
        quiet(&mut e, &[0x008c_7490]);
        let threads: Ptr = Ptr::new(0x011d_fc24);
        e.mem.set_u32(threads.addr() + 4, 0x1234);
        log_calls(&mut e);
        e.call(0x008c_a330, &args![threads, 1u32]);
        assert_eq!(calls_to(&e, 0x008c_7490), vec![vec![0x1234]]);
    }

    #[test]
    fn message_pool_destructor_runs_the_array_destructor_then_resets_base() {
        let mut e = engine();
        quiet(&mut e, &[0x00ec_5fce]);
        let this: Ptr = Ptr::new(e.mem.alloc(0x1000c));
        log_calls(&mut e);
        e.call(0x008c_a350, &args![this]);
        assert_eq!(e.mem.u32(this.addr()), 0x0108_583c);
        assert_eq!(
            calls_to(&e, 0x00ec_5fce),
            vec![vec![this.addr() + 0x0c, 0x10, 0x1000, 0x0048_3710]]
        );
    }

    #[test]
    fn message_pool_base_destructor_sets_its_vtable() {
        let mut e = engine();
        let this: Ptr = Ptr::new(e.mem.alloc(0x20));
        e.call(0x008c_a3c0, &args![this]);
        assert_eq!(e.mem.u32(this.addr()), 0x0108_583c);
    }

    #[test]
    fn message_pool_base_deleting_destructor_obeys_the_allocation_flag() {
        let mut e = engine();
        quiet(&mut e, &[OPERATOR_DELETE]);
        let base: Ptr = Ptr::new(e.mem.alloc(0x20));
        log_calls(&mut e);
        e.call(0x008c_a3e0, &args![base, 0u32]);
        assert!(calls_to(&e, OPERATOR_DELETE).is_empty());
        assert_eq!(e.mem.u32(base.addr()), 0x0108_583c);
    }

    #[test]
    fn message_pool_array_deleting_destructor_frees_on_bit_zero() {
        let mut e = engine();
        quiet(&mut e, &[0x00ec_5fce, OPERATOR_DELETE]);
        let array: Ptr = Ptr::new(e.mem.alloc(0x1000c));
        log_calls(&mut e);
        e.call(0x008c_a410, &args![array, 1u32]);
        assert_eq!(calls_to(&e, OPERATOR_DELETE), vec![vec![array.addr()]]);
    }

    #[test]
    fn common_message_queue_constructor_sets_allocator_and_empty_tail() {
        let mut e = engine();
        quiet(&mut e, &[0x008c_a740]);
        let queue: Ptr = Ptr::new(e.mem.alloc(0x20));
        let allocator: Ptr = Ptr::new(0x7100_0050);
        log_calls(&mut e);
        e.call(0x008c_a440, &args![queue, allocator]);
        assert_eq!(e.mem.u32(queue.addr()), 0x0108_5844);
        assert_eq!(e.mem.u32(queue.addr() + 8), allocator.addr());
        assert_eq!(e.mem.u32(queue.addr() + 0xc), 0);
        assert_eq!(e.mem.u32(queue.addr() + 0x10), queue.addr() + 0xc);
        assert_eq!(calls_to(&e, 0x008c_a740), vec![vec![queue.addr()]]);
    }

    #[test]
    fn common_message_queue_drain_stops_on_the_first_empty_pop() {
        let mut e = engine();
        let pops = Rc::new(RefCell::new(0u32));
        let count = pops.clone();
        e.register_double(0x006e_c390, move |_, _| {
            *count.borrow_mut() += 1;
            ok(u32::from(*count.borrow() == 1))
        });
        let queue: Ptr = Ptr::new(e.mem.alloc(0x20));
        log_calls(&mut e);
        e.call(0x008c_a480, &args![queue]);
        assert_eq!(*pops.borrow(), 2);
        assert_eq!(e.mem.u32(queue.addr()), 0x0108_5884);
        assert_eq!(calls_to(&e, 0x006e_c390).len(), 2);
    }

    #[test]
    fn message_queue_destructor_restores_derived_vtable() {
        let mut e = engine();
        let queue: Ptr = Ptr::new(e.mem.alloc(0x20));
        e.call(0x008c_a500, &args![queue]);
        assert_eq!(e.mem.u32(queue.addr()), 0x0108_5884);
    }

    #[test]
    fn message_queue_base_destructor_sets_base_vtable() {
        let mut e = engine();
        let queue: Ptr = Ptr::new(e.mem.alloc(0x20));
        e.call(0x008c_a520, &args![queue]);
        assert_eq!(e.mem.u32(queue.addr()), 0x0108_5884);
    }

    #[test]
    fn message_queue_scalar_deleting_destructor_deletes_on_bit_zero() {
        let mut e = engine();
        quiet(&mut e, &[OPERATOR_DELETE]);
        let first: Ptr = Ptr::new(e.mem.alloc(0x20));
        log_calls(&mut e);
        e.call(0x008c_a540, &args![first, 0u32]);
        assert!(calls_to(&e, OPERATOR_DELETE).is_empty());
        assert_eq!(e.mem.u32(first.addr()), 0x0108_5884);
    }

    #[test]
    fn mobile_message_queue_scalar_deleting_destructor_deletes_on_bit_zero() {
        let mut e = engine();
        quiet(&mut e, &[OPERATOR_DELETE]);
        let second: Ptr = Ptr::new(e.mem.alloc(0x20));
        log_calls(&mut e);
        e.call(0x008c_a570, &args![second, 1u32]);
        assert_eq!(calls_to(&e, OPERATOR_DELETE), vec![vec![second.addr()]]);
        assert_eq!(e.mem.u32(second.addr()), 0x0108_5884);
    }

    #[test]
    fn linked_message_queue_push_appends_and_reports_allocator_failure() {
        let mut e = engine();
        let queue: Ptr = Ptr::new(e.mem.alloc(0x20));
        let node = e.mem.alloc(0x20);
        let pool: Ptr = Ptr::new(0x7100_0000);
        e.mem.set_u32(queue.addr() + 8, pool.addr());
        e.mem.set_u32(queue.addr() + 0xc, 0);
        e.mem.set_u32(queue.addr() + 0x10, queue.addr() + 0x0c);
        e.register_double(0x008c_a7c0, move |e, a| {
            e.mem.set_u32(a[1], node);
            ok(1)
        });
        log_calls(&mut e);
        assert!(e.call(0x008c_a5a0, &args![queue, 0x1234u32]).bool());
        assert_eq!(e.mem.u32(queue.addr() + 0x0c), node);
        assert_eq!(e.mem.u32(node + 0x0c), 0);
        assert_eq!(e.mem.u32(queue.addr() + 0x10), node + 0x0c);
        e.mem.set_u32(queue.addr() + 8, 0);
        assert!(!e.call(0x008c_a5a0, &args![queue, 0x5678u32]).bool());
        assert_eq!(calls_to(&e, 0x008c_a7c0).len(), 1);
    }
    // ---- scheduling (008c94e0, 008ca070) ----

    const PUSH_SLOT: u32 = 0x7000_0040;
    const SETTING_BYTE: u32 = 0x0040_8d60;
    const SETTING_INT: u32 = 0x0043_d4d0;
    const ACTOR_LIST: u32 = 0x7200_0000;
    /// The offsets of the six embedded tasklets in the manager.
    const TASKLET_OFFSETS: [u32; 6] = [0x00, 0x40, 0x78, 0xbc, 0xf8, 0x138];

    /// One message a tasklet's queue received: the tasklet's offset in the
    /// manager, message type, count and mob.
    type Received = (u32, u32, u32, u32);

    struct Schedule {
        manager: Ptr,
        received: Rc<RefCell<Vec<Received>>>,
    }

    /// A manager whose six tasklets have a queue with a push slot (logging the
    /// messages), a run slot and a drain slot; settings (bytes by setting
    /// address, ints by setting address), and an actor list of `count`
    /// entries whose actor `i` is `0x5000 + i`.
    fn schedule(e: &mut Engine, bytes: &[(u32, u8)], ints: &[(u32, i32)], count: u32) -> Schedule {
        e.map(0x011a_3000, 0x1000);
        e.map(0x011f_1000, 0x1000);
        e.map(0x011c_3000, 0x1000);
        e.map(0x011d_1000, 0x1000);
        e.mem.set_u8(0x011a_34e7, 0);
        e.mem.set_u8(0x011f_1220, 1);
        let manager: Ptr = Ptr::new(e.mem.alloc(0x10184));
        let table = e.mem.alloc(0x40);
        e.mem.set_u32(table + 4, PUSH_SLOT);
        e.mem.set_u32(table + 0xc, QUEUE_RUN);
        e.mem.set_u32(table + 0x10, QUEUE_DRAIN);
        for offset in TASKLET_OFFSETS {
            e.mem.set_u32(manager.addr() + offset + 0x1c, table);
        }
        let received = Rc::new(RefCell::new(Vec::new()));
        let log = received.clone();
        let base = manager.addr();
        e.register_double(PUSH_SLOT, move |e, a| {
            let message = a[1];
            log.borrow_mut().push((
                a[0] - 0x1c - base,
                e.mem.u32(message),
                e.mem.u32(message + 4),
                e.mem.u32(message + 8),
            ));
            ok(1)
        });
        e.register(QUEUE_RUN, |_, _| ok(0));
        let mut cells = Vec::new();
        for (setting, value) in bytes {
            let cell = e.mem.alloc(1);
            e.mem.set_u8(cell, *value);
            cells.push((*setting, cell));
        }
        let byte_cells = cells.clone();
        e.register_double(SETTING_BYTE, move |_, a| {
            ok(byte_cells.iter().find(|(s, _)| *s == a[0]).unwrap().1)
        });
        let mut int_cells = Vec::new();
        for (setting, value) in ints {
            let cell = e.mem.alloc(4);
            e.mem.set_i32(cell, *value);
            int_cells.push((*setting, cell));
        }
        e.register_double(SETTING_INT, move |_, a| {
            ok(int_cells.iter().find(|(s, _)| *s == a[0]).unwrap().1)
        });
        e.register(0x0071_7e50, |_, a| {
            assert_eq!(a, &[PROCESS_LISTS]);
            ok(ACTOR_LIST)
        });
        e.register_double(0x005b_e5c0, move |_, a| {
            assert_eq!(a, &[ACTOR_LIST, 0]);
            ok(count)
        });
        e.register(0x0096_8670, |_, a| {
            assert_eq!(a[0], ACTOR_LIST);
            ok(0x5000 + a[1])
        });
        quiet(e, &[0x0070_34c0, 0x0070_3490]);
        Schedule { manager, received }
    }

    const BYTE_SETTINGS_ON: [(u32, u8); 3] = [
        (SETTING_THREADED, 0),
        (SETTING_NO_DETECTION, 0),
        (SETTING_NO_MOVEMENT, 0),
    ];

    #[test]
    fn manager_process_queues_every_tasklet_and_runs_them_here_without_threads() {
        let mut e = engine();
        let s = schedule(
            &mut e,
            &BYTE_SETTINGS_ON,
            &[(SETTING_SPLIT_UPDATE, 1), (0x011d_10d4, 7)],
            2,
        );
        log_calls(&mut e);
        e.call(0x008c_94e0, &args![s.manager]);
        let mut expected: Vec<Received> = Vec::new();
        // Counts: detection, package, actor update, animation, script, movement.
        for offset in [0x00, 0x78, 0xbc, 0x40, 0xf8, 0x138] {
            expected.push((offset, 0, 2, 0));
        }
        for actor in [0x5000, 0x5001] {
            for offset in [0x00, 0x78, 0xbc, 0x40, 0xf8, 0x138] {
                expected.push((offset, 1, 0, actor));
            }
        }
        // Ends: detection, package, animation, actor update, script, movement.
        for offset in [0x00, 0x78, 0x40, 0xbc, 0xf8, 0x138] {
            expected.push((offset, 2, 0, 0));
        }
        assert_eq!(*s.received.borrow(), expected);
        // The tasklets run to completion: detection, package, script,
        // animation, actor update, movement.
        let runs: Vec<u32> = calls_to(&e, QUEUE_RUN)
            .iter()
            .map(|args| args[0] - 0x1c - s.manager.addr())
            .collect();
        assert_eq!(runs, vec![0x00, 0x78, 0xf8, 0x40, 0xbc, 0x138]);
        assert_eq!(calls_to(&e, 0x0070_34c0), vec![vec![7]]);
        assert_eq!(calls_to(&e, 0x0070_3490), vec![vec![]]);
        // Nothing went to the worker ring.
        assert_eq!(e.mem.i32(TASK_RING + 0x40), 0);
    }

    #[test]
    fn manager_process_skips_disabled_tasklets() {
        let mut e = engine();
        let s = schedule(
            &mut e,
            &[
                (SETTING_THREADED, 0),
                (SETTING_NO_DETECTION, 1),
                (SETTING_NO_MOVEMENT, 1),
            ],
            &[(SETTING_SPLIT_UPDATE, 0)],
            1,
        );
        log_calls(&mut e);
        e.call(0x008c_94e0, &args![s.manager]);
        // Only the package and script tasklets take part.
        assert_eq!(
            *s.received.borrow(),
            vec![
                (0x78, 0, 1, 0),
                (0xf8, 0, 1, 0),
                (0x78, 1, 0, 0x5000),
                (0xf8, 1, 0, 0x5000),
                (0x78, 2, 0, 0),
                (0xf8, 2, 0, 0),
            ]
        );
        assert_eq!(calls_to(&e, QUEUE_RUN).len(), 2);
        // The final branch needs the setting to be 1.
        assert!(calls_to(&e, 0x0070_34c0).is_empty());
    }

    #[test]
    fn manager_process_hands_the_tasklets_to_the_workers_with_threads() {
        let mut e = engine();
        let s = schedule(
            &mut e,
            &[
                (SETTING_THREADED, 1),
                (SETTING_NO_DETECTION, 0),
                (SETTING_NO_MOVEMENT, 0),
            ],
            &[(SETTING_SPLIT_UPDATE, 1), (0x011d_10d4, 3)],
            1,
        );
        // One registered worker.
        let thread = e.mem.alloc(0x78);
        e.mem.set_u32(THREAD_MANAGER, thread);
        e.mem.set_i32(THREAD_MANAGER + 0x0c, 1);
        quiet(&mut e, &[0x0044_2550, 0x008c_7490]);
        e.mem.set_i32(TASK_RING + 0x40, 0);
        e.mem.set_i32(TASK_RING + 0x44, 0);
        log_calls(&mut e);
        e.call(0x008c_94e0, &args![s.manager]);
        let base = s.manager.addr();
        // The ring holds: actor update, package, script, detection, movement,
        // animation.
        assert_eq!(e.mem.i32(TASK_RING + 0x40), 6);
        let ring: Vec<u32> = (0..6)
            .map(|i| e.mem.u32(TASK_RING + 0x18 + i * 4))
            .collect();
        assert_eq!(
            ring,
            vec![
                base + 0xbc,
                base + 0x78,
                base + 0xf8,
                base,
                base + 0x138,
                base + 0x40
            ]
        );
        // No tasklet runs here; the worker is released and woken.
        assert!(calls_to(&e, QUEUE_RUN).is_empty());
        assert_eq!(calls_to(&e, 0x0044_2550), vec![vec![thread + 0x30]]);
        assert_eq!(calls_to(&e, 0x008c_7490), vec![vec![thread]]);
        assert_eq!(calls_to(&e, 0x0070_34c0), vec![vec![3]]);
    }

    #[test]
    fn manager_process_does_nothing_for_an_empty_actor_list_or_without_the_flag() {
        let mut e = engine();
        let s = schedule(&mut e, &BYTE_SETTINGS_ON, &[(SETTING_SPLIT_UPDATE, 0)], 0);
        e.call(0x008c_94e0, &args![s.manager]);
        assert!(s.received.borrow().is_empty());
        let mut e = engine();
        let s = schedule(&mut e, &BYTE_SETTINGS_ON, &[(SETTING_SPLIT_UPDATE, 0)], 3);
        e.mem.set_u8(0x011f_1220, 0);
        e.call(0x008c_94e0, &args![s.manager]);
        assert!(s.received.borrow().is_empty());
        assert_eq!(e.mem.i32(TASK_RING + 0x40), 0);
    }

    #[test]
    fn manager_process_creates_three_workers_once() {
        let mut e = engine();
        let s = schedule(
            &mut e,
            &[
                (SETTING_THREADED, 1),
                (SETTING_NO_DETECTION, 0),
                (SETTING_NO_MOVEMENT, 0),
            ],
            &[(SETTING_SPLIT_UPDATE, 0)],
            0,
        );
        e.mem.set_u8(0x011a_34e7, 1);
        e.mem.set_i32(THREAD_MANAGER + 0x0c, 0);
        quiet(
            &mut e,
            &[
                0x00aa_6360,
                0x0086_c6a0,
                0x00c8_5770,
                0x00aa_6430,
                0x00aa_6540,
                0x00aa_6550,
            ],
        );
        e.register_double(OPERATOR_NEW, |e, _| ok(e.mem.alloc(0x78)));
        log_calls(&mut e);
        e.call(0x008c_94e0, &args![s.manager]);
        let processors: Vec<u32> = calls_to(&e, 0x00aa_6550)
            .iter()
            .map(|args| args[1])
            .collect();
        assert_eq!(processors, vec![1, 3, 5]);
        assert_eq!(e.mem.i32(THREAD_MANAGER + 0x0c), 3);
        assert_eq!(e.mem.u8(0x011a_34e7), 0);
    }

    #[test]
    fn parallel_process_handles_only_the_tasklets_the_first_pass_skipped() {
        let mut e = engine();
        let s = schedule(
            &mut e,
            &[
                (SETTING_THREADED, 0),
                (SETTING_NO_DETECTION, 1),
                (SETTING_NO_MOVEMENT, 0),
            ],
            &[(SETTING_SPLIT_UPDATE, 1)],
            2,
        );
        log_calls(&mut e);
        e.call(0x008c_a070, &args![s.manager]);
        assert_eq!(
            *s.received.borrow(),
            vec![
                (0x00, 0, 2, 0),
                (0x00, 1, 0, 0x5000),
                (0x00, 1, 0, 0x5001),
                (0x00, 2, 0, 0),
            ]
        );
        let runs: Vec<u32> = calls_to(&e, QUEUE_RUN)
            .iter()
            .map(|args| args[0] - 0x1c - s.manager.addr())
            .collect();
        assert_eq!(runs, vec![0x00]);
        assert_eq!(e.mem.u8(s.manager.addr() + 0x10180), 0);
    }

    #[test]
    fn parallel_process_with_threads_queues_both_and_releases_worker_two() {
        let mut e = engine();
        let s = schedule(
            &mut e,
            &[
                (SETTING_THREADED, 1),
                (SETTING_NO_DETECTION, 1),
                (SETTING_NO_MOVEMENT, 1),
            ],
            &[(SETTING_SPLIT_UPDATE, 1)],
            1,
        );
        let worker = e.mem.alloc(0x78);
        e.mem.set_u32(THREAD_MANAGER + 8, worker);
        quiet(&mut e, &[0x0044_2550]);
        e.mem.set_i32(TASK_RING + 0x40, 0);
        e.mem.set_i32(TASK_RING + 0x44, 0);
        log_calls(&mut e);
        e.call(0x008c_a070, &args![s.manager]);
        let base = s.manager.addr();
        assert_eq!(
            *s.received.borrow(),
            vec![
                (0x00, 0, 1, 0),
                (0x138, 0, 1, 0),
                (0x00, 1, 0, 0x5000),
                (0x138, 1, 0, 0x5000),
                (0x00, 2, 0, 0),
                (0x138, 2, 0, 0),
            ]
        );
        assert_eq!(e.mem.i32(TASK_RING + 0x40), 2);
        assert_eq!(e.mem.u32(TASK_RING + 0x18), base);
        assert_eq!(e.mem.u32(TASK_RING + 0x1c), base + 0x138);
        assert_eq!(calls_to(&e, 0x0044_2550), vec![vec![worker + 0x30]]);
        assert_eq!(e.mem.u8(base + 0x10180), 1);
        assert!(calls_to(&e, QUEUE_RUN).is_empty());
    }

    #[test]
    fn parallel_process_needs_the_setting_a_tasklet_and_actors() {
        // Setting not 1.
        let mut e = engine();
        let s = schedule(
            &mut e,
            &[
                (SETTING_THREADED, 0),
                (SETTING_NO_DETECTION, 1),
                (SETTING_NO_MOVEMENT, 1),
            ],
            &[(SETTING_SPLIT_UPDATE, 0)],
            2,
        );
        e.call(0x008c_a070, &args![s.manager]);
        assert!(s.received.borrow().is_empty());
        // Neither tasklet is switched on.
        let mut e = engine();
        let s = schedule(&mut e, &BYTE_SETTINGS_ON, &[(SETTING_SPLIT_UPDATE, 1)], 2);
        e.call(0x008c_a070, &args![s.manager]);
        assert!(s.received.borrow().is_empty());
        // No actors.
        let mut e = engine();
        let s = schedule(
            &mut e,
            &[
                (SETTING_THREADED, 0),
                (SETTING_NO_DETECTION, 1),
                (SETTING_NO_MOVEMENT, 1),
            ],
            &[(SETTING_SPLIT_UPDATE, 1)],
            0,
        );
        e.call(0x008c_a070, &args![s.manager]);
        assert!(s.received.borrow().is_empty());
        // Flag cleared.
        let mut e = engine();
        let s = schedule(
            &mut e,
            &[
                (SETTING_THREADED, 0),
                (SETTING_NO_DETECTION, 1),
                (SETTING_NO_MOVEMENT, 1),
            ],
            &[(SETTING_SPLIT_UPDATE, 1)],
            2,
        );
        e.mem.set_u8(0x011f_1220, 0);
        e.call(0x008c_a070, &args![s.manager]);
        assert!(s.received.borrow().is_empty());
    }

    // ---- worker startup and shutdown (008c9d70, 008c9e40, 008c9c70) ----

    const MEMORY_SYSTEM_INIT: u32 = 0x7000_0100;
    const MEMORY_SYSTEM_QUIT: u32 = 0x7000_0110;
    const ALLOCATE: u32 = 0x7000_0120;
    const FREE: u32 = 0x7000_0130;

    /// What the Havok doubles hand out.
    struct ThreadEnvironment {
        memory_system: u32,
        allocator: u32,
        block: u32,
    }

    /// Doubles for the Havok and file calls of the worker startup and
    /// shutdown.
    fn thread_environment(e: &mut Engine) -> ThreadEnvironment {
        e.map(0x011c_3000, 0x1000);
        e.set_global(THREAD_SAFE_FILES_OWNER, 0x7300_0000u32);
        let memory_system = object_with_slots(
            e,
            0x10,
            &[(0x0c, MEMORY_SYSTEM_INIT), (0x10, MEMORY_SYSTEM_QUIT)],
        )
        .addr();
        let allocator = object_with_slots(e, 0x10, &[(0x04, ALLOCATE), (0x08, FREE)]).addr();
        let block = e.mem.alloc(0x20);
        e.register_double(MEMORY_SYSTEM_GET_INSTANCE, move |_, _| ok(memory_system));
        e.register_double(ROUTER_WORD, move |_, _| ok(allocator));
        e.register(RETURN_THIS, |_, a| ok(a[0]));
        e.register_double(ALLOCATE, move |_, _| ok(block));
        quiet(
            e,
            &[
                MEMORY_SYSTEM_INIT,
                MEMORY_SYSTEM_QUIT,
                FREE,
                HK_INIT_THREAD,
                HK_QUIT_THREAD,
                ROUTER_SET_WORD_C,
                ROUTER_SET_WORD_18,
                ROUTER_SET_BLOCK,
                CREATE_THREAD_SAFE_FILES,
            ],
        );
        ThreadEnvironment {
            memory_system,
            allocator,
            block,
        }
    }

    #[test]
    fn worker_startup_sets_up_the_router_and_scratch_memory() {
        let mut e = engine();
        let env = thread_environment(&mut e);
        let this: Ptr = Ptr::new(e.mem.alloc(0x78));
        let router = this.addr() + 0x50;
        log_calls(&mut e);
        e.call(0x008c_9d70, &args![this]);
        assert_eq!(e.mem.u32(this.addr() + 0x74), env.block);
        let wanted = [
            MEMORY_SYSTEM_INIT,
            HK_INIT_THREAD,
            ROUTER_SET_WORD_C,
            ROUTER_SET_WORD_18,
            ALLOCATE,
            ROUTER_SET_BLOCK,
            CREATE_THREAD_SAFE_FILES,
        ];
        let log: Vec<(u32, Vec<u32>)> = e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| wanted.contains(a))
            .cloned()
            .collect();
        assert_eq!(
            log,
            vec![
                (
                    MEMORY_SYSTEM_INIT,
                    vec![env.memory_system, router, 0x0108_5820, 1]
                ),
                (HK_INIT_THREAD, vec![router]),
                (ROUTER_SET_WORD_C, vec![router, env.allocator]),
                (ROUTER_SET_WORD_18, vec![router, env.allocator]),
                (ALLOCATE, vec![env.allocator, 0x16800]),
                (ROUTER_SET_BLOCK, vec![router, env.block, 0x16800]),
                (CREATE_THREAD_SAFE_FILES, vec![0x7300_0000]),
            ]
        );
    }

    #[test]
    fn worker_shutdown_frees_the_scratch_memory_and_quits_the_thread() {
        let mut e = engine();
        let env = thread_environment(&mut e);
        let this: Ptr = Ptr::new(e.mem.alloc(0x78));
        let router = this.addr() + 0x50;
        e.mem.set_u32(this.addr() + 0x74, env.block);
        log_calls(&mut e);
        e.call(0x008c_9e40, &args![this]);
        let wanted = [FREE, HK_QUIT_THREAD, MEMORY_SYSTEM_QUIT, ROUTER_SET_BLOCK];
        let log: Vec<(u32, Vec<u32>)> = e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| wanted.contains(a))
            .cloned()
            .collect();
        assert_eq!(
            log,
            vec![
                (FREE, vec![env.allocator, env.block, 0x16800]),
                (HK_QUIT_THREAD, vec![]),
                (MEMORY_SYSTEM_QUIT, vec![env.memory_system, router, 1]),
                (ROUTER_SET_BLOCK, vec![router, 0, 0]),
            ]
        );
    }

    #[test]
    fn worker_thread_proc_runs_queued_tasklets_until_the_kill_flag() {
        let mut e = engine();
        thread_environment(&mut e);
        let this: Ptr = Ptr::new(e.mem.alloc(0x78));
        e.mem.set_u32(this.addr() + 0x4c, 5);
        quiet(
            &mut e,
            &[CRITICAL_SECTION_ENTER, CRITICAL_SECTION_LEAVE, 0x0044_2550],
        );
        // One tasklet waits in the ring.
        let tasklet: Ptr = tasklet(&mut e, 0x38, &standard_handlers());
        quiet(&mut e, &[QUEUE_RUN]);
        e.mem.set_i32(TASK_RING + 0x40, 1);
        e.mem.set_i32(TASK_RING + 0x44, 0);
        e.mem.set_u32(TASK_RING + 0x18, tasklet.addr());
        // The first wait returns with the flag clear, the second with it set.
        let waits = Rc::new(RefCell::new(0u32));
        let counter = waits.clone();
        let kill = this.addr() + 0x48;
        let wait_arg = this.addr() + 0x30;
        e.register_double(0x0044_24e0, move |e, a| {
            assert_eq!(a, &[wait_arg]);
            *counter.borrow_mut() += 1;
            if *counter.borrow() == 2 {
                e.mem.set_u8(kill, 1);
            }
            ok(0)
        });
        log_calls(&mut e);
        assert_eq!(e.call(0x008c_9c70, &args![this]).u32(), 0);
        assert_eq!(*waits.borrow(), 2);
        // The tasklet ran to completion with the worker's index.
        assert_eq!(calls_to(&e, QUEUE_RUN).len(), 1);
        assert_eq!(e.mem.u32(tasklet.addr() + 0x34), 5);
        assert_eq!(e.mem.i32(TASK_RING + 0x40), 0);
    }

    // ---- message pool and queue (008ca620 .. 008ca9e0) ----

    /// A pool object (0x20 bytes) whose free list holds `nodes` fresh nodes,
    /// chained through their next word (+0xC); returns the pool and the nodes.
    fn pool_with_nodes(e: &mut Engine, nodes: usize) -> (Ptr, Vec<u32>) {
        let pool: Ptr = Ptr::new(e.mem.alloc(0x20));
        let nodes: Vec<u32> = (0..nodes).map(|_| e.mem.alloc(0x10)).collect();
        for pair in nodes.windows(2) {
            e.mem.set_u32(pair[0] + 0x0c, pair[1]);
        }
        e.mem
            .set_u32(pool.addr() + 8, nodes.first().copied().unwrap_or(0));
        (pool, nodes)
    }

    fn lock_calls(e: &Engine) -> Vec<(u32, Vec<u32>)> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == POOL_COMPARE_EXCHANGE || *a == POOL_DEBUG_CHECK)
            .cloned()
            .collect()
    }

    fn lock_sequence(pool: Ptr) -> Vec<(u32, Vec<u32>)> {
        vec![
            (POOL_COMPARE_EXCHANGE, vec![pool.addr() + 4, 0, 1]),
            (POOL_DEBUG_CHECK, vec![]),
            (POOL_DEBUG_CHECK, vec![]),
            (POOL_COMPARE_EXCHANGE, vec![pool.addr() + 4, 1, 0]),
        ]
    }

    /// Doubles for the lock calls and the message copy helpers.
    fn pool_environment(e: &mut Engine) {
        quiet(
            e,
            &[POOL_COMPARE_EXCHANGE, POOL_DEBUG_CHECK, EMPTY_DESTRUCTOR],
        );
        e.register(RETURN_THIS, |_, a| ok(a[0]));
        e.register(PLACEMENT_NEW, |_, a| {
            assert_eq!(a[0], 0xc);
            ok(a[1])
        });
        e.register(MESSAGE_COPY, |e, a| {
            for offset in [0, 4, 8] {
                let word = e.mem.u32(a[1] + offset);
                e.mem.set_u32(a[0] + offset, word);
            }
            ok(a[0])
        });
    }

    #[test]
    fn pool_pop_takes_the_first_free_node_under_the_lock() {
        let mut e = engine();
        pool_environment(&mut e);
        let (pool, nodes) = pool_with_nodes(&mut e, 2);
        let out = e.mem.alloc(4);
        log_calls(&mut e);
        assert!(e.call(0x008c_a940, &args![pool, out]).bool());
        assert_eq!(e.mem.u32(out), nodes[0]);
        assert_eq!(e.mem.u32(pool.addr() + 8), nodes[1]);
        assert_eq!(e.mem.u32(nodes[0] + 0x0c), 0);
        assert_eq!(lock_calls(&e), lock_sequence(pool));
        assert!(e.call(0x008c_a940, &args![pool, out]).bool());
        assert_eq!(e.mem.u32(out), nodes[1]);
        // The list is empty now: no node, and the output is cleared.
        assert!(!e.call(0x008c_a940, &args![pool, out]).bool());
        assert_eq!(e.mem.u32(out), 0);
        assert_eq!(e.mem.u32(pool.addr() + 8), 0);
    }

    #[test]
    fn pool_lock_spins_until_the_compare_exchange_succeeds() {
        let mut e = engine();
        pool_environment(&mut e);
        let (pool, _) = pool_with_nodes(&mut e, 1);
        let out = e.mem.alloc(4);
        let busy = Rc::new(RefCell::new(2u32));
        let remaining = busy.clone();
        e.register_double(POOL_COMPARE_EXCHANGE, move |_, a| {
            if a[1] == 0 && *remaining.borrow() > 0 {
                *remaining.borrow_mut() -= 1;
                return ok(1);
            }
            ok(0)
        });
        log_calls(&mut e);
        assert!(e.call(0x008c_a940, &args![pool, out]).bool());
        let acquires = calls_to(&e, POOL_COMPARE_EXCHANGE)
            .iter()
            .filter(|args| args[1] == 0)
            .count();
        assert_eq!(acquires, 3);
        assert_eq!(*busy.borrow(), 0);
    }

    #[test]
    fn pool_push_puts_the_node_on_top_of_the_free_list() {
        let mut e = engine();
        pool_environment(&mut e);
        let (pool, nodes) = pool_with_nodes(&mut e, 1);
        let node = e.mem.alloc(0x10);
        let slot = e.mem.alloc(4);
        e.mem.set_u32(slot, node);
        log_calls(&mut e);
        e.call(0x008c_a8d0, &args![pool, slot]);
        assert_eq!(e.mem.u32(pool.addr() + 8), node);
        assert_eq!(e.mem.u32(node + 0x0c), nodes[0]);
        assert_eq!(e.mem.u32(slot), 0);
        assert_eq!(lock_calls(&e), lock_sequence(pool));
    }

    #[test]
    fn pool_release_runs_the_node_destructor_then_pushes() {
        let mut e = engine();
        pool_environment(&mut e);
        let (pool, _) = pool_with_nodes(&mut e, 0);
        let node = e.mem.alloc(0x10);
        let slot = e.mem.alloc(4);
        e.mem.set_u32(slot, node);
        log_calls(&mut e);
        e.call(0x008c_a770, &args![pool, slot]);
        assert_eq!(calls_to(&e, EMPTY_DESTRUCTOR), vec![vec![node]]);
        assert_eq!(e.mem.u32(pool.addr() + 8), node);
        assert_eq!(e.mem.u32(slot), 0);
    }

    #[test]
    fn pool_link_chains_the_array_and_ends_it_with_zero() {
        let mut e = engine();
        pool_environment(&mut e);
        let pool: Ptr = Ptr::new(e.mem.alloc(0x20));
        let array = e.mem.alloc(0x30);
        log_calls(&mut e);
        e.call(0x008c_a830, &args![pool, 3u32, array]);
        assert_eq!(e.mem.u32(pool.addr() + 8), array);
        assert_eq!(e.mem.u32(array + 0x0c), array + 0x10);
        assert_eq!(e.mem.u32(array + 0x1c), array + 0x20);
        assert_eq!(e.mem.u32(array + 0x2c), 0);
        assert_eq!(lock_calls(&e), lock_sequence(pool));
        // A single node is just terminated.
        let single = e.mem.alloc(0x10);
        e.mem.set_u32(single + 0x0c, 0xdead);
        e.call(0x008c_a830, &args![pool, 1u32, single]);
        assert_eq!(e.mem.u32(single + 0x0c), 0);
    }

    #[test]
    fn pool_allocate_copies_the_message_into_a_free_node() {
        let mut e = engine();
        pool_environment(&mut e);
        let (pool, nodes) = pool_with_nodes(&mut e, 1);
        let out = e.mem.alloc(4);
        let message = e.mem.alloc(0xc);
        e.mem.set_u32(message, 1);
        e.mem.set_u32(message + 4, 2);
        e.mem.set_u32(message + 8, 0x3333);
        log_calls(&mut e);
        assert!(e.call(0x008c_a7c0, &args![pool, out, message]).bool());
        assert_eq!(e.mem.u32(out), nodes[0]);
        assert_eq!(e.mem.u32(nodes[0]), 1);
        assert_eq!(e.mem.u32(nodes[0] + 4), 2);
        assert_eq!(e.mem.u32(nodes[0] + 8), 0x3333);
        assert_eq!(calls_to(&e, MESSAGE_COPY), vec![vec![nodes[0], message]]);
        // The pool is empty now: nothing is constructed.
        let copies = calls_to(&e, MESSAGE_COPY).len();
        assert!(!e.call(0x008c_a7c0, &args![pool, out, message]).bool());
        assert_eq!(calls_to(&e, MESSAGE_COPY).len(), copies);
    }

    #[test]
    fn construct_copy_returns_zero_when_placement_new_gives_no_address() {
        let mut e = engine();
        pool_environment(&mut e);
        let node = e.mem.alloc(0x10);
        let message = e.mem.alloc(0xc);
        log_calls(&mut e);
        assert_eq!(e.call(0x008c_a9e0, &args![node, message]).u32(), node);
        assert_eq!(calls_to(&e, EMPTY_DESTRUCTOR), vec![vec![node]]);
        e.register(PLACEMENT_NEW, |_, _| ok(0));
        let copies = calls_to(&e, MESSAGE_COPY).len();
        assert_eq!(e.call(0x008c_a9e0, &args![node, message]).u32(), 0);
        assert_eq!(calls_to(&e, MESSAGE_COPY).len(), copies);
    }

    #[test]
    fn queue_try_pop_copies_the_first_message_and_frees_its_node() {
        let mut e = engine();
        pool_environment(&mut e);
        let (pool, _) = pool_with_nodes(&mut e, 0);
        let queue: Ptr = Ptr::new(e.mem.alloc(0x20));
        let first = e.mem.alloc(0x10);
        let second = e.mem.alloc(0x10);
        e.mem.set_u32(first, 1);
        e.mem.set_u32(first + 4, 2);
        e.mem.set_u32(first + 8, 0x4444);
        e.mem.set_u32(first + 0x0c, second);
        e.mem.set_u32(queue.addr() + 8, pool.addr());
        e.mem.set_u32(queue.addr() + 0x0c, first);
        let out = e.mem.alloc(0xc);
        log_calls(&mut e);
        assert!(e.call(0x008c_a620, &args![queue, out]).bool());
        assert_eq!(
            [e.mem.u32(out), e.mem.u32(out + 4), e.mem.u32(out + 8)],
            [1, 2, 0x4444]
        );
        assert_eq!(e.mem.u32(queue.addr() + 0x0c), second);
        // The node went back to the pool.
        assert_eq!(e.mem.u32(pool.addr() + 8), first);
        assert_eq!(e.mem.u32(first + 0x0c), 0);
        assert_eq!(calls_to(&e, EMPTY_DESTRUCTOR), vec![vec![first]]);
        // An empty queue pops nothing.
        e.mem.set_u32(queue.addr() + 0x0c, 0);
        assert!(!e.call(0x008c_a620, &args![queue, out]).bool());
        assert_eq!(e.mem.u32(pool.addr() + 8), first);
    }

    #[test]
    fn pool_constructor_builds_the_node_array_and_links_it() {
        let mut e = engine();
        pool_environment(&mut e);
        e.register(VECTOR_CONSTRUCTOR_ITERATOR, |_, _| Ret::default());
        let pool: Ptr = Ptr::new(e.mem.alloc(0x1000c));
        e.mem.set_u32(pool.addr() + 4, 0x99);
        log_calls(&mut e);
        assert_eq!(e.call(0x008c_a680, &args![pool]).u32(), pool.addr());
        assert_eq!(e.mem.u32(pool.addr()), 0x0108_5834);
        assert_eq!(e.mem.u32(pool.addr() + 4), 0);
        assert_eq!(
            calls_to(&e, VECTOR_CONSTRUCTOR_ITERATOR),
            vec![vec![
                pool.addr() + 0x0c,
                0x10,
                0x1000,
                NODE_CONSTRUCTOR,
                EMPTY_DESTRUCTOR
            ]]
        );
        assert_eq!(e.mem.u32(pool.addr() + 8), pool.addr() + 0x0c);
        assert_eq!(e.mem.u32(pool.addr() + 0x0c + 0x0c), pool.addr() + 0x1c);
        let last = pool.addr() + 0x0c + 0xfff * 0x10;
        assert_eq!(e.mem.u32(last + 0x0c), 0);
        assert_eq!(e.mem.u32(last - 4), last);
    }

    #[test]
    fn small_constructors_set_their_vtables_and_clear_their_words() {
        let mut e = engine();
        let pool: Ptr = Ptr::new(e.mem.alloc(0x20));
        e.mem.set_u32(pool.addr() + 4, 7);
        e.mem.set_u32(pool.addr() + 8, 7);
        assert_eq!(e.call(0x008c_a800, &args![pool]).u32(), pool.addr());
        assert_eq!(e.mem.u32(pool.addr()), 0x0108_583c);
        assert_eq!(e.mem.u32(pool.addr() + 4), 0);
        assert_eq!(e.mem.u32(pool.addr() + 8), 0);
        let queue: Ptr = Ptr::new(e.mem.alloc(0x20));
        e.mem.set_u32(queue.addr() + 4, 7);
        assert_eq!(e.call(0x008c_a740, &args![queue]).u32(), queue.addr());
        assert_eq!(e.mem.u32(queue.addr()), 0x0108_5864);
        assert_eq!(e.mem.u32(queue.addr() + 4), 0);
        let base: Ptr = Ptr::new(e.mem.alloc(0x20));
        assert_eq!(e.call(0x008c_a7a0, &args![base]).u32(), base.addr());
        assert_eq!(e.mem.u32(base.addr()), 0x0108_5884);
    }

    #[test]
    fn queue_scalar_deleting_destructor_drains_and_deletes_on_bit_zero() {
        let mut e = engine();
        e.register(0x006e_c390, |_, _| ok(0));
        quiet(&mut e, &[OPERATOR_DELETE]);
        let queue: Ptr = Ptr::new(e.mem.alloc(0x20));
        log_calls(&mut e);
        assert_eq!(e.call(0x008c_a710, &args![queue, 0u32]).u32(), queue.addr());
        assert_eq!(e.mem.u32(queue.addr()), 0x0108_5884);
        assert!(calls_to(&e, OPERATOR_DELETE).is_empty());
        assert_eq!(calls_to(&e, 0x006e_c390).len(), 1);
        e.call(0x008c_a710, &args![queue, 1u32]);
        assert_eq!(calls_to(&e, OPERATOR_DELETE), vec![vec![queue.addr()]]);
    }
}

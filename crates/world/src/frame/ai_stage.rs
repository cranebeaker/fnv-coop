//! The frame's AI task stage: the work `Main::OnIdle` (`0086e650`) starts
//! in stage 6 (`AILinearTaskThreadManager::StartThreads`, `008c78c0`) and
//! joins in stage 8 (`WaitForThreads`, `008c7990`), and the main thread's
//! animations and effects next to it (`Main::OnIdle_UpdateAnimationsAndEffects`,
//! `0086fc60`), in the exe's order, with the tests that decide which of
//! their calls run (docs/FRAME_SKELETON.md, "PR 6 result").
//!
//! **The threads.** `AILinearTaskThreadManager::CreateThreads` (`008c7290`,
//! called from `main` at `0086b12d` when the thread count is more than 1)
//! makes one thread on [`COMBINED`] (`008c7bd0`, "AI Linear Task Thread")
//! when the thread count is at most 2 (`008c7307`: `cmp [eax],2`, `jle`),
//! else two, on [`THREAD_1`] (`008c7da0`, "AI Linear Task Thread 1") and
//! [`THREAD_2`] (`008c7f50`, "AI Linear Task Thread 2"). The count is
//! `iNumHWThreads:General`, which `main` sets to the processor count and
//! raises from 1 to 2 (`0086a950`-`0086a990`): a two-processor PC runs the
//! combined function, anything with more processors the pair. With a count
//! of 1 there are no AI threads and stage 6 starts the AI task queue
//! instead (`AITaskManager::StartTasksDuringRendering`, `008ca070`), which
//! is not modelled here.
//!
//! Each thread function is a list of [`ThreadStep`]s: the calls doing the
//! work and the threads' handshakes. `008c79e0(thread, stage)` sets the
//! event of `stage` in the thread slot's table (`[manager+0x8c+thread*4]`)
//! and records the stage (only when the slot has a thread, `008c79e0`'s
//! test of `[manager+thread*4]`); `008c7a70(thread, stage)` waits for that
//! event (`WaitForSingleObject`, no time-out); `008c7d80(thread, stage)`
//! records the stage without an event (the combined function, which has no
//! partner to wake). `008c80d0` (through `008c80b0`) waits for the render
//! semaphore that `AILinearTaskThreadManager::SetMainRendering(0)` signals
//! (`008c80e0` → `008c80c0`, the last call of `Main::Swap`): the thread's
//! calls after it run once the main thread has drawn the frame. Between
//! some calls the threads also take and drop the process lists' lock
//! (`0040fbf0`/`0040fba0` on `011f11a0`); left out here, as are the getters
//! the calls take their arguments from (`00713d80` the thread manager,
//! `004537b0` the task queue, `005c42d0` the fader's flag `[011dea2d]` for
//! `CombatManager::Update`, `00453850` the frame time `[011dea30]`,
//! `00408d60` the obstacle setting).
//!
//! [`schedule`] puts the threads' calls in one order, as the viewer's one
//! thread runs them: the combined function's own order with two threads;
//! with more, thread 1 runs until it waits for something thread 2 hasn't
//! signalled, then thread 2 likewise, and so on, which is one order the
//! exe can run them in (any order that keeps each thread's order and its
//! waits is one; which one a frame takes is the machine's timing, the
//! thread split itself is platform). Both orders split at the render wait.
//!
//! **The calls.** [`FUNCTIONS`] holds, the way `world::frame::world_time`
//! does for stage 4, the calls doing the work inside the functions the
//! threads and the main thread call here: `Main::OnIdle_UpdateAnimationsAndEffects`
//! (`0086fc60`), `TES::UpdateCellAnimations` (`00453550`, the Havok step),
//! `TES::UpdateCellMainThread` (`004537c0`, the sky), the managed nodes of
//! the exterior grid (`004ba9a0`) and of a cell (`00551890`), the exterior
//! world's step (`00554780`), `ProcessLists::RunActorUpdates` (`0096c7c0`)
//! and `Actor::Update` (`00888b50`), each with its [`SubGate`] (a test of the
//! function's own branches on an [`AiState`]) and, where the call also
//! depends on its block's own tests, those branches as
//! [`SubStep::own_tests`], not modelled.
//!
//! Names are Xbox PDB names (ADR-0002): the engine map's, or, for the
//! thread functions' callees the map doesn't name, the ones Phase 1 PR 1
//! paired by aligning the call lists with the Xbox
//! `AILinearTaskThreadManager::AIThreadCombined`, `AIThread1TaskFunc` and
//! `AIThread2TaskFunc` ([`ThreadOp::Call`]'s `lead`).
//!
//! The disassembly is the source for every call site and branch named
//! here. The thread functions, `0096c7c0` and `004ba9a0` have no
//! translation in the engine crate; for the others its tests drive the
//! translation and check its calls against this model ([`follows`]).
//!
//! The viewer orders the AI stage's systems by these sets
//! (`viewer/src/frame_order.rs`, Phase 1 PR 6).

// AILinearTaskThreadManager's thread functions 008c7bd0, 008c7da0, 008c7f50:
// their calls read from the disassembly (FalloutNV.exe 1.4.0.525); no
// translation in the engine crate.
// Translated from 0086fc60 (decompiled, FalloutNV.exe 1.4.0.525)
// Translated from 00453550 (decompiled, FalloutNV.exe 1.4.0.525)
// Translated from 004537c0 (decompiled, FalloutNV.exe 1.4.0.525)
// GridCellArray::UpdateManagedNodes (004ba9a0) and ProcessLists::RunActorUpdates
// (0096c7c0): read from the disassembly; no translation in the engine crate.
// Translated from 00554780 (decompiled, FalloutNV.exe 1.4.0.525)
// Translated from 00551890 (decompiled, FalloutNV.exe 1.4.0.525)
// Translated from 00888b50 (decompiled, FalloutNV.exe 1.4.0.525)

pub use super::world_time::Callee;
use super::Wiring;

/// `Main::OnIdle` (Xbox PDB).
pub const ON_IDLE: u32 = 0x0086_e650;
/// `AILinearTaskThreadManager::StartThreads` (stage 6).
pub const START_THREADS: u32 = 0x008c_78c0;
/// `AILinearTaskThreadManager::WaitForThreads` (stage 8).
pub const WAIT_FOR_THREADS: u32 = 0x008c_7990;
/// `AILinearTaskThreadManager::CreateThreads`.
pub const CREATE_THREADS: u32 = 0x008c_7290;
/// `Main::OnIdle_UpdateAnimationsAndEffects`.
pub const UPDATE_ANIMATIONS_AND_EFFECTS: u32 = 0x0086_fc60;
/// `TES::UpdateCellAnimations`.
pub const UPDATE_CELL_ANIMATIONS: u32 = 0x0045_3550;
/// `TES::UpdateCellMainThread`.
pub const UPDATE_CELL_MAIN_THREAD: u32 = 0x0045_37c0;
/// `ProcessLists::RunActorUpdates` (a lead).
pub const RUN_ACTOR_UPDATES: u32 = 0x0096_c7c0;
/// `Actor::Update`.
pub const ACTOR_UPDATE: u32 = 0x0088_8b50;
/// `GridCellArray::UpdateManagedNodes`.
pub const GRID_MANAGED_NODES: u32 = 0x004b_a9a0;
/// `TESObjectCELL::UpdateManagedNodes`.
pub const CELL_MANAGED_NODES: u32 = 0x0055_1890;
/// The exterior world's step (`tesobjectcell.cpp`).
pub const EXTERIOR_WORLD_STEP: u32 = 0x0055_4780;
/// `bhkWorld::Update`'s slot in the `bhkWorld` and `bhkWorldM` vtables
/// (`010c4178`, `010c6ab8`; docs/FRAME_SKELETON.md, "The Havok step").
pub const HAVOK_STEP_SLOT: u32 = 0xc4;
/// `bhkWorld::Update`.
pub const HAVOK_WORLD_UPDATE: u32 = 0x00c6_ae70;

/// What a thread function's step does.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThreadOp {
    /// A call doing work.
    Call {
        callee: u32,
        /// Its Xbox PDB name.
        name: Option<&'static str>,
        /// The name is a pairing by position in the Xbox call list (Phase 1
        /// PR 1), not the engine map's.
        lead: bool,
        gate: ThreadGate,
        wiring: Wiring,
    },
    /// `008c79e0(thread, stage)`: the event set, the stage recorded.
    Signal { thread: u8, stage: u8 },
    /// `008c7a70(thread, stage)`: waits for that event.
    Wait { thread: u8, stage: u8 },
    /// `008c7d80(thread, stage)`: the stage recorded, no event.
    Mark { thread: u8, stage: u8 },
    /// `008c80d0`: waits for the main thread's render (`Main::Swap`'s
    /// `SetMainRendering(0)`).
    WaitRender,
}

/// A thread function's test before a call.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ThreadGate {
    Always,
    /// `bObstacleAvoidance:Pathfinding` (`011d73e4`, through `00408d60`) on:
    /// the obstacle manager's update (`008c7d2c` in the combined function,
    /// `008c7f23` in thread 1).
    ObstacleAvoidance,
}

impl ThreadGate {
    pub fn holds(self, s: &AiState) -> bool {
        match self {
            ThreadGate::Always => true,
            ThreadGate::ObstacleAvoidance => s.obstacle_avoidance,
        }
    }
}

/// One step of a thread function.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ThreadStep {
    /// The call instruction.
    pub site: u32,
    pub op: ThreadOp,
}

/// One AI linear task thread function.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Thread {
    pub address: u32,
    /// Its size in bytes (the engine map's).
    pub size: u32,
    /// The thread's name (`CreateThreads`' `AddThread`, `008c74d0`).
    pub name: &'static str,
    /// Where the name string is.
    pub name_string: u32,
    /// Its branch on the thread count in `CreateThreads`.
    pub created_at: u32,
    pub steps: &'static [ThreadStep],
}

impl Thread {
    /// Whether `address` is inside the function.
    pub fn contains(&self, address: u32) -> bool {
        (self.address..self.address + self.size).contains(&address)
    }

    /// The index of the work call of `callee`, if it makes one.
    pub fn call_of(&self, callee: u32) -> Option<usize> {
        self.steps
            .iter()
            .position(|s| matches!(s.op, ThreadOp::Call { callee: c, .. } if c == callee))
    }
}

const fn call(site: u32, callee: u32, name: &'static str, lead: bool) -> ThreadStep {
    ThreadStep {
        site,
        op: ThreadOp::Call {
            callee,
            name: Some(name),
            lead,
            gate: ThreadGate::Always,
            wiring: Wiring::Open,
        },
    }
}

const fn wired(
    site: u32,
    callee: u32,
    name: &'static str,
    lead: bool,
    wiring: Wiring,
) -> ThreadStep {
    ThreadStep {
        site,
        op: ThreadOp::Call {
            callee,
            name: Some(name),
            lead,
            gate: ThreadGate::Always,
            wiring,
        },
    }
}

const fn obstacle(site: u32, callee: u32, name: &'static str) -> ThreadStep {
    ThreadStep {
        site,
        op: ThreadOp::Call {
            callee,
            name: Some(name),
            lead: false,
            gate: ThreadGate::ObstacleAvoidance,
            wiring: Wiring::Open,
        },
    }
}

const fn signal(site: u32, thread: u8, stage: u8) -> ThreadStep {
    ThreadStep {
        site,
        op: ThreadOp::Signal { thread, stage },
    }
}

const fn wait(site: u32, thread: u8, stage: u8) -> ThreadStep {
    ThreadStep {
        site,
        op: ThreadOp::Wait { thread, stage },
    }
}

const fn mark(site: u32, thread: u8, stage: u8) -> ThreadStep {
    ThreadStep {
        site,
        op: ThreadOp::Mark { thread, stage },
    }
}

const fn wait_render(site: u32) -> ThreadStep {
    ThreadStep {
        site,
        op: ThreadOp::WaitRender,
    }
}

/// `Main::OnIdle_DoInterfaceIdle`.
pub const INTERFACE_IDLE: u32 = 0x0086_fd70;
/// `ProcessLists::RunActorAnimationUpdates` (a lead).
pub const ACTOR_ANIMATION_UPDATES: u32 = 0x0096_cca0;
/// `TES::RunAnimations`.
pub const RUN_ANIMATIONS: u32 = 0x0045_5640;
/// `ProcessLists::UpdateActorsMovement` (a lead).
pub const ACTORS_MOVEMENT: u32 = 0x0096_db30;
/// `ProcessLists::RunActorRagdollAnimationUpdates` (a lead).
pub const RAGDOLL_UPDATES: u32 = 0x0096_cb50;

const IDLE_WIRING: Wiring = Wiring::Partial(
    "viewer: hud::update_hud, local_map::update_local_map, pipboy::update_pipboy and \
     pipboy_light, at the call but outside its gate (the interface idle runs on the main \
     thread when there is no AI work, step 105)",
);
const ANIMATION_WIRING: Wiring = Wiring::Partial(
    "viewer: actors::animate_actors (with the script and line idles before it, the head \
     tracking, and the clothes and faces after it), at the call but outside its gate (it \
     handles menu mode and the dialogue menu's speaker itself)",
);
const RUN_ANIMATIONS_WIRING: Wiring =
    Wiring::Partial("viewer: move_pieces (models' own animations, door leaves)");
const MOVEMENT_WIRING: Wiring = Wiring::Partial(
    "viewer: ai::move_actors at the call, outside its gate; the projectiles under it \
     (fighting::resolve_shots, bolts::fly_bolts, explosives::fly_thrown: the level's \
     projectiles `009bec10` and explosions `009ae580`)",
);
const RAGDOLL_WIRING: Wiring =
    Wiring::Partial("viewer: actors::animate_actors' ragdolls (at the animation updates)");

/// The combined thread function (`008c7bd0`), for a thread count of 2.
pub const COMBINED: Thread = Thread {
    address: 0x008c_7bd0,
    size: 418,
    name: "AI Linear Task Thread",
    name_string: 0x0108_561c,
    created_at: 0x008c_730a,
    steps: &[
        signal(0x008c_7bde, 0, 0),
        wired(
            0x008c_7be9,
            INTERFACE_IDLE,
            "Main::OnIdle_DoInterfaceIdle",
            false,
            IDLE_WIRING,
        ),
        signal(0x008c_7bf9, 0, 1),
        call(
            0x008c_7c05,
            0x0087_a6b0,
            "TaskQueueInterface::ThreadBeginInput",
            true,
        ),
        call(0x008c_7c19, 0x0099_1500, "CombatManager::Update", false),
        wired(
            0x008c_7c2f,
            ACTOR_ANIMATION_UPDATES,
            "ProcessLists::RunActorAnimationUpdates",
            true,
            ANIMATION_WIRING,
        ),
        call(
            0x008c_7c39,
            0x0096_cda0,
            "ProcessLists::ParallelActorAnimationMovementUpdates",
            true,
        ),
        wired(
            0x008c_7c4e,
            RUN_ANIMATIONS,
            "TES::RunAnimations",
            false,
            RUN_ANIMATIONS_WIRING,
        ),
        call(
            0x008c_7c64,
            0x0097_84c0,
            "ProcessLists::RunActorMagicUpdates",
            true,
        ),
        call(
            0x008c_7c79,
            0x0099_1dc0,
            "CombatManager::UpdateCombatants",
            true,
        ),
        call(
            0x008c_7c97,
            0x0096_bcd0,
            "ProcessLists::UpdateHighListPackages",
            true,
        ),
        call(
            0x008c_7c9c,
            0x0096_d520,
            "ProcessLists::UpdatePlayerFollowers",
            true,
        ),
        call(
            0x008c_7ca6,
            0x0096_c330,
            "ProcessLists::RunDetectionForAllActors",
            true,
        ),
        wired(
            0x008c_7cba,
            ACTORS_MOVEMENT,
            "ProcessLists::UpdateActorsMovement",
            true,
            MOVEMENT_WIRING,
        ),
        call(0x008c_7ccc, 0x008d_0600, "ProcessLists::PrintLists", false),
        call(
            0x008c_7cd6,
            RUN_ACTOR_UPDATES,
            "ProcessLists::RunActorUpdates",
            true,
        ),
        wired(
            0x008c_7ce0,
            RAGDOLL_UPDATES,
            "ProcessLists::RunActorRagdollAnimationUpdates",
            true,
            RAGDOLL_WIRING,
        ),
        call(
            0x008c_7ce5,
            0x0047_72f0,
            "BGSDestructibleObjectForm::UpdateDestructibleObjects",
            true,
        ),
        call(
            0x008c_7d03,
            UPDATE_CELL_ANIMATIONS,
            "TES::UpdateCellAnimations",
            false,
        ),
        mark(0x008c_7d13, 1, 6),
        wait_render(0x008c_7d18),
        obstacle(
            0x008c_7d2e,
            0x006c_0720,
            "NavMeshObstacleManager::GetInstance",
        ),
        obstacle(0x008c_7d35, 0x006c_3640, "NavMeshObstacleManager::Update"),
        call(
            0x008c_7d41,
            0x0087_a6d0,
            "TaskQueueInterface::ThreadEndInput",
            true,
        ),
        call(
            0x008c_7d46,
            0x005a_e270,
            "Script::ClearOptimizations",
            false,
        ),
        call(
            0x008c_7d4b,
            0x005a_9d60,
            "ScriptLocals::ClearOptimizations",
            false,
        ),
        mark(0x008c_7d5b, 0, 11),
        mark(0x008c_7d6b, 1, 7),
    ],
};

/// AI linear task thread 1 (`008c7da0`, slot 0), with a thread count of 3
/// or more: the interface idle, combat, the actors' animations, the cells'
/// animations, magic, packages, followers and the actor updates.
pub const THREAD_1: Thread = Thread {
    address: 0x008c_7da0,
    size: 429,
    name: "AI Linear Task Thread 1",
    name_string: 0x0108_564c,
    created_at: 0x008c_730a,
    steps: &[
        signal(0x008c_7dae, 0, 0),
        wired(
            0x008c_7db9,
            INTERFACE_IDLE,
            "Main::OnIdle_DoInterfaceIdle",
            false,
            IDLE_WIRING,
        ),
        signal(0x008c_7dc9, 0, 1),
        call(
            0x008c_7dd5,
            0x0087_a6b0,
            "TaskQueueInterface::ThreadBeginInput",
            true,
        ),
        call(0x008c_7de9, 0x0099_1500, "CombatManager::Update", false),
        wired(
            0x008c_7df3,
            ACTOR_ANIMATION_UPDATES,
            "ProcessLists::RunActorAnimationUpdates",
            true,
            ANIMATION_WIRING,
        ),
        call(
            0x008c_7dfd,
            0x0096_cda0,
            "ProcessLists::ParallelActorAnimationMovementUpdates",
            true,
        ),
        signal(0x008c_7e0d, 0, 2),
        wired(
            0x008c_7e18,
            RUN_ANIMATIONS,
            "TES::RunAnimations",
            false,
            RUN_ANIMATIONS_WIRING,
        ),
        signal(0x008c_7e28, 0, 3),
        call(
            0x008c_7e33,
            0x0099_1dc0,
            "CombatManager::UpdateCombatants",
            true,
        ),
        signal(0x008c_7e43, 0, 4),
        call(
            0x008c_7e4d,
            0x0097_84c0,
            "ProcessLists::RunActorMagicUpdates",
            true,
        ),
        signal(0x008c_7e5d, 0, 5),
        call(
            0x008c_7e6f,
            0x0096_bcd0,
            "ProcessLists::UpdateHighListPackages",
            true,
        ),
        signal(0x008c_7e7f, 0, 6),
        call(
            0x008c_7e84,
            0x0096_d520,
            "ProcessLists::UpdatePlayerFollowers",
            true,
        ),
        signal(0x008c_7e94, 0, 10),
        wait(0x008c_7ea4, 1, 1),
        wait(0x008c_7eb4, 1, 3),
        call(
            0x008c_7ebe,
            RUN_ACTOR_UPDATES,
            "ProcessLists::RunActorUpdates",
            true,
        ),
        signal(0x008c_7ece, 0, 7),
        call(
            0x008c_7eda,
            0x0087_a6d0,
            "TaskQueueInterface::ThreadEndInput",
            true,
        ),
        signal(0x008c_7eea, 0, 8),
        wait(0x008c_7efa, 1, 7),
        wait_render(0x008c_7eff),
        signal(0x008c_7f0f, 0, 9),
        obstacle(
            0x008c_7f25,
            0x006c_0720,
            "NavMeshObstacleManager::GetInstance",
        ),
        obstacle(0x008c_7f2c, 0x006c_3640, "NavMeshObstacleManager::Update"),
        call(
            0x008c_7f31,
            0x005a_e270,
            "Script::ClearOptimizations",
            false,
        ),
        call(
            0x008c_7f36,
            0x005a_9d60,
            "ScriptLocals::ClearOptimizations",
            false,
        ),
        signal(0x008c_7f46, 0, 11),
    ],
};

/// AI linear task thread 2 (`008c7f50`, slot 1): detection, movement, the
/// ragdolls, the destructibles and the cells' animations (the Havok step).
pub const THREAD_2: Thread = Thread {
    address: 0x008c_7f50,
    size: 346,
    name: "AI Linear Task Thread 2",
    name_string: 0x0108_5634,
    created_at: 0x008c_730a,
    steps: &[
        signal(0x008c_7f5e, 1, 0),
        wait(0x008c_7f6e, 0, 1),
        call(
            0x008c_7f7a,
            0x0087_a6b0,
            "TaskQueueInterface::ThreadBeginInput",
            true,
        ),
        call(
            0x008c_7f84,
            0x0096_c330,
            "ProcessLists::RunDetectionForAllActors",
            true,
        ),
        signal(0x008c_7f94, 1, 1),
        signal(0x008c_7fa4, 1, 2),
        wait(0x008c_7fb4, 0, 2),
        wired(
            0x008c_7fc8,
            ACTORS_MOVEMENT,
            "ProcessLists::UpdateActorsMovement",
            true,
            MOVEMENT_WIRING,
        ),
        signal(0x008c_7fd8, 1, 3),
        call(0x008c_7fea, 0x008d_0600, "ProcessLists::PrintLists", false),
        wired(
            0x008c_7ff4,
            RAGDOLL_UPDATES,
            "ProcessLists::RunActorRagdollAnimationUpdates",
            true,
            RAGDOLL_WIRING,
        ),
        signal(0x008c_8004, 1, 4),
        call(
            0x008c_8009,
            0x0047_72f0,
            "BGSDestructibleObjectForm::UpdateDestructibleObjects",
            true,
        ),
        signal(0x008c_8019, 1, 5),
        wait(0x008c_8029, 0, 4),
        call(
            0x008c_803d,
            UPDATE_CELL_ANIMATIONS,
            "TES::UpdateCellAnimations",
            false,
        ),
        signal(0x008c_804d, 1, 6),
        call(
            0x008c_8059,
            0x0087_a6d0,
            "TaskQueueInterface::ThreadEndInput",
            true,
        ),
        wait(0x008c_8069, 0, 8),
        signal(0x008c_8079, 1, 7),
        call(
            0x008c_807e,
            0x005a_e270,
            "Script::ClearOptimizations",
            false,
        ),
        call(
            0x008c_8083,
            0x005a_9d60,
            "ScriptLocals::ClearOptimizations",
            false,
        ),
        wait(0x008c_8093, 0, 9),
        wait(0x008c_80a3, 0, 11),
    ],
};

/// The thread functions `CreateThreads` makes for a thread count: none
/// with 1 (`main` makes no threads, `0086b117`), the combined one with 2,
/// the pair with more (`008c7307`).
pub fn threads_for(threads: i32) -> &'static [Thread] {
    const ONE: [Thread; 1] = [COMBINED];
    const TWO: [Thread; 2] = [THREAD_1, THREAD_2];
    match threads {
        i32::MIN..=1 => &[],
        2 => &ONE,
        _ => &TWO,
    }
}

/// One place in [`schedule`]'s order.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Slot {
    /// Step `.1` of the thread function at `.0`.
    Step(u32, usize),
    /// The main thread has drawn the frame (`Main::Swap`): the render wait
    /// is passed.
    Render,
}

/// The threads' steps for a thread count in one order the exe can run them
/// in: each thread's own order, every wait after its signal, the render
/// wait after the main thread's render ([`Slot::Render`], once). Thread 1
/// runs first and each thread runs until it waits for what hasn't happened
/// yet.
pub fn schedule(threads: i32) -> Vec<Slot> {
    let list = threads_for(threads);
    let mut at = vec![0usize; list.len()];
    let mut signalled: Vec<(u8, u8)> = Vec::new();
    let mut rendered = false;
    let mut out = Vec::new();
    loop {
        let mut moved = false;
        for (t, thread) in list.iter().enumerate() {
            while at[t] < thread.steps.len() {
                let step = &thread.steps[at[t]];
                match step.op {
                    ThreadOp::Wait { thread: w, stage } if !signalled.contains(&(w, stage)) => {
                        break
                    }
                    ThreadOp::WaitRender if !rendered => break,
                    ThreadOp::Signal { thread: w, stage } => signalled.push((w, stage)),
                    _ => {}
                }
                out.push(Slot::Step(thread.address, at[t]));
                at[t] += 1;
                moved = true;
            }
        }
        if at.iter().zip(list).all(|(&i, t)| i == t.steps.len()) {
            if !rendered && !list.is_empty() {
                out.push(Slot::Render);
            }
            return out;
        }
        if !moved {
            assert!(!rendered, "the AI threads wait for each other");
            rendered = true;
            out.push(Slot::Render);
        }
    }
}

/// The thread function at `address`.
pub fn thread(address: u32) -> Option<&'static Thread> {
    [&COMBINED, &THREAD_1, &THREAD_2]
        .into_iter()
        .find(|t| t.address == address)
}

/// The work calls of [`schedule`], in order, each as (thread function,
/// step index), and whether it comes after the render.
pub fn scheduled_calls(threads: i32) -> Vec<(u32, usize, bool)> {
    let mut after = false;
    let mut out = Vec::new();
    for slot in schedule(threads) {
        match slot {
            Slot::Render => after = true,
            Slot::Step(t, i) => {
                let th = thread(t).expect("a thread function");
                if matches!(th.steps[i].op, ThreadOp::Call { .. }) {
                    out.push((t, i, after));
                }
            }
        }
    }
    out
}

/// What a function tests before a sub-step. [`SubGate::holds`] evaluates
/// it on an [`AiState`], [`SubGate::branches`] names the branches.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SubGate {
    Always,
    // `Main::OnIdle_UpdateAnimationsAndEffects` (`0086fc60`): nothing runs
    // unless menu mode is clear, or the dialogue flag `[011dea2c]`
    // (`Interface::InDialog`, set at `0086e6ec`/`0086e7d3`) or the fader
    // flag `[011dea2d]` is set (`0086fc72`, `0086fc7d`, `0086fc88`), and the
    // world isn't frozen (`Main` +7, `0086fc97`).
    /// The temporary effects in parallel: threads > 1 (`0086fcaa`).
    TempEffectsThreaded,
    /// One by one: one thread (`0086fcaa`).
    TempEffectsSingleThread,
    /// `TES::UpdateCellAnimations` here: one thread and menu mode clear
    /// (`0086fceb`, `0086fcf6`).
    CellAnimationsHere,
    /// Else `TES::UpdateCellMainThread` (`0086fceb`, `0086fcf6`).
    CellMainThreadHere,
    /// The particles, with threads > 1 (`0086fd3d`).
    ParticlesThreaded,
    /// With one thread (`0086fd3d`).
    ParticlesSingleThread,
    // `TES::UpdateCellAnimations` (`00453550`).
    /// An interior loaded (`TES` +0x34, `005f36f0`; `00453568`): no wind.
    WindInterior,
    /// None: the sky's wind speed (+0xcc) and angle (+0xd0).
    WindExterior,
    /// The interior's managed nodes (`004535f7`).
    CellInterior,
    /// The exterior grid's (`004535f7`).
    CellExterior,
    /// The task queue: threads > 1 (`0045363b`).
    TaskQueueThreaded,
    /// The sky: one thread (`00453666`).
    SingleThread,
    /// The main thread's part: one thread (`004536c0`).
    SingleThreadMain,
    // `Actor::Update` (`00888b50`).
    /// The actor has its 3D, a parent cell and the cell passes `00450ff0`
    /// (`00888c5e`, `00888c64`, `00888c73`), and the time is at least
    /// `[010848d0]` (`00888c88`): the process is only woken.
    ActorLongStep,
    /// The same, with the time under it: the update.
    ActorUpdates,
    /// And a character controller (`MobileObject::GetCharController`,
    /// `009306d0`; `00888de3`).
    ActorController,
    /// And the actor isn't the player (`00888df5`): the head's target.
    ActorControllerTarget,
    /// The player in first person (the player, `[011dea3c]`, and its
    /// +0x64a clear, `004eaf60`; `00888d9e`, `00888db0`, `00889810`): its 3D
    /// placed at its position.
    FirstPersonPlayer,
    /// Anyone else (`00889810`).
    NotFirstPersonPlayer,
    /// Anyone else with a controller, outside menu mode
    /// (`Interface::IsInMenuMode`, asked at `008898bd`; `008898ad`,
    /// `008898b7`, `008898c7`): swimming and wading.
    ActorWater,
}

/// The stage's inputs to the gates. `Default` is a frame in game mode with
/// two threads (the least `main` leaves), outdoors, obstacle avoidance on
/// (its default), and for `Actor::Update` an actor that is ready, has a
/// controller and isn't the player.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AiState {
    /// `iNumHWThreads:General`.
    pub threads: i32,
    /// Menu mode, `[011dea2b]`.
    pub menu_flag: bool,
    /// `Interface::IsInMenuMode` as asked again.
    pub in_menu_mode: bool,
    /// `[011dea2c]`: `Interface::InDialog`.
    pub in_dialog: bool,
    /// `[011dea2d]`: fader 1 visible.
    pub fader_visible: bool,
    /// `Main` +7.
    pub world_frozen: bool,
    /// `bObstacleAvoidance:Pathfinding`.
    pub obstacle_avoidance: bool,
    /// `TES` +0x34.
    pub interior_loaded: bool,
    /// `Actor::Update`: the actor's 3D, cell and cell state.
    pub actor_ready: bool,
    /// The time passed is at least `[010848d0]`.
    pub actor_long_step: bool,
    /// The actor has a character controller.
    pub actor_controller: bool,
    /// The actor is the player.
    pub actor_is_player: bool,
    /// The player's +0x64a (third person).
    pub third_person: bool,
}

impl Default for AiState {
    fn default() -> AiState {
        AiState {
            threads: 2,
            menu_flag: false,
            in_menu_mode: false,
            in_dialog: false,
            fader_visible: false,
            world_frozen: false,
            obstacle_avoidance: true,
            interior_loaded: false,
            actor_ready: true,
            actor_long_step: false,
            actor_controller: true,
            actor_is_player: false,
            third_person: false,
        }
    }
}

impl AiState {
    /// `0086fc60` works: menu mode clear, or in dialogue, or the fader
    /// visible; the world not frozen.
    pub fn animations_run(&self) -> bool {
        (!self.menu_flag || self.in_dialog || self.fader_visible) && !self.world_frozen
    }

    /// `Actor::Update`'s first-person player.
    pub fn first_person_player(&self) -> bool {
        self.actor_is_player && !self.third_person
    }
}

impl SubGate {
    /// Whether the sub-step is reached, given that its function is called.
    pub fn holds(self, s: &AiState) -> bool {
        let runs = s.animations_run();
        let one = s.threads == 1;
        let updates = s.actor_ready && !s.actor_long_step;
        let fpp = s.first_person_player();
        match self {
            SubGate::Always => true,
            SubGate::TempEffectsThreaded | SubGate::ParticlesThreaded => runs && s.threads > 1,
            SubGate::TempEffectsSingleThread | SubGate::ParticlesSingleThread => {
                runs && s.threads <= 1
            }
            SubGate::CellAnimationsHere => runs && one && !s.menu_flag,
            SubGate::CellMainThreadHere => runs && !(one && !s.menu_flag),
            SubGate::WindInterior | SubGate::CellInterior => s.interior_loaded,
            SubGate::WindExterior | SubGate::CellExterior => !s.interior_loaded,
            SubGate::TaskQueueThreaded => s.threads > 1,
            SubGate::SingleThread | SubGate::SingleThreadMain => one,
            SubGate::ActorLongStep => s.actor_ready && s.actor_long_step,
            SubGate::ActorUpdates => updates,
            SubGate::ActorController => updates && s.actor_controller,
            SubGate::ActorControllerTarget => updates && s.actor_controller && !s.actor_is_player,
            SubGate::FirstPersonPlayer => updates && fpp,
            SubGate::NotFirstPersonPlayer => updates && !fpp,
            SubGate::ActorWater => updates && !fpp && s.actor_controller && !s.in_menu_mode,
        }
    }

    /// The branch instructions that make up the test.
    pub fn branches(self) -> &'static [u32] {
        const READY: [u32; 4] = [0x0088_8c5e, 0x0088_8c64, 0x0088_8c73, 0x0088_8c88];
        match self {
            SubGate::Always => &[],
            SubGate::TempEffectsThreaded | SubGate::TempEffectsSingleThread => &[
                0x0086_fc72,
                0x0086_fc7d,
                0x0086_fc88,
                0x0086_fc97,
                0x0086_fcaa,
            ],
            SubGate::CellAnimationsHere | SubGate::CellMainThreadHere => &[
                0x0086_fc72,
                0x0086_fc7d,
                0x0086_fc88,
                0x0086_fc97,
                0x0086_fceb,
                0x0086_fcf6,
            ],
            SubGate::ParticlesThreaded | SubGate::ParticlesSingleThread => &[
                0x0086_fc72,
                0x0086_fc7d,
                0x0086_fc88,
                0x0086_fc97,
                0x0086_fd3d,
            ],
            SubGate::WindInterior | SubGate::WindExterior => &[0x0045_3568],
            SubGate::CellInterior | SubGate::CellExterior => &[0x0045_35f7],
            SubGate::TaskQueueThreaded => &[0x0045_363b],
            SubGate::SingleThread => &[0x0045_3666],
            SubGate::SingleThreadMain => &[0x0045_36c0],
            SubGate::ActorLongStep | SubGate::ActorUpdates => &READY,
            SubGate::ActorController => &[
                0x0088_8c5e,
                0x0088_8c64,
                0x0088_8c73,
                0x0088_8c88,
                0x0088_8de3,
            ],
            SubGate::ActorControllerTarget => &[
                0x0088_8c5e,
                0x0088_8c64,
                0x0088_8c73,
                0x0088_8c88,
                0x0088_8de3,
                0x0088_8df5,
            ],
            SubGate::FirstPersonPlayer | SubGate::NotFirstPersonPlayer => &[
                0x0088_8c5e,
                0x0088_8c64,
                0x0088_8c73,
                0x0088_8c88,
                0x0088_8d9e,
                0x0088_8db0,
                0x0088_9810,
            ],
            SubGate::ActorWater => &[
                0x0088_8c5e,
                0x0088_8c64,
                0x0088_8c73,
                0x0088_8c88,
                0x0088_98ad,
                0x0088_98b7,
                0x0088_98c7,
            ],
        }
    }
}

/// One sub-step of a [`Function`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SubStep {
    pub callee: Callee,
    /// Its call instructions.
    pub sites: &'static [u32],
    /// The callee's Xbox PDB name (the engine map's, or `frame.tsv`'s).
    pub name: Option<&'static str>,
    pub gate: SubGate,
    /// The branches of the step's own block that decide whether the call
    /// is made once the gate holds; not modelled.
    pub own_tests: &'static [u32],
    pub wiring: Wiring,
}

/// A call of a modelled function: who calls it, from where.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Caller {
    /// `Main::OnIdle` ([`ON_IDLE`]), a thread function or a function of
    /// [`FUNCTIONS`].
    pub function: u32,
    /// The call instruction.
    pub site: u32,
}

/// One modelled function of the stage.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Function {
    pub address: u32,
    /// Its size in bytes (the engine map's).
    pub size: u32,
    pub name: Option<&'static str>,
    /// Its calls in the frame.
    pub callers: &'static [Caller],
    pub steps: &'static [SubStep],
}

impl Function {
    /// Whether `address` is inside the function.
    pub fn contains(&self, address: u32) -> bool {
        (self.address..self.address + self.size).contains(&address)
    }
}

const fn sub(
    callee: Callee,
    sites: &'static [u32],
    name: Option<&'static str>,
    gate: SubGate,
    own_tests: &'static [u32],
    wiring: Wiring,
) -> SubStep {
    SubStep {
        callee,
        sites,
        name,
        gate,
        own_tests,
        wiring,
    }
}

use Callee::{Direct, Virtual};
use SubGate as G;
use Wiring::{Open, Partial};

/// `Main::OnIdle_UpdateAnimationsAndEffects` (`0086fc60`, frame step 103 by
/// `frame.tsv`'s count): the temporary effects, the cell's animations (one
/// thread outside menu mode) or the main thread's part of them (the sky),
/// the particles.
pub const UPDATE_ANIMATIONS_AND_EFFECTS_STEPS: [SubStep; 6] = [
    sub(
        Direct(0x0097_46c0),
        &[0x0086_fcbf],
        Some("ProcessLists::UpdateTempEffectsParallel"),
        G::TempEffectsThreaded,
        &[],
        Open,
    ),
    sub(
        Direct(0x0097_4420),
        &[0x0086_fcd9],
        Some("ProcessLists::UpdateTempEffects"),
        G::TempEffectsSingleThread,
        &[],
        Open,
    ),
    sub(
        Direct(UPDATE_CELL_ANIMATIONS),
        &[0x0086_fd08],
        Some("TES::UpdateCellAnimations"),
        G::CellAnimationsHere,
        &[],
        Open,
    ),
    sub(
        Direct(UPDATE_CELL_MAIN_THREAD),
        &[0x0086_fd15],
        Some("TES::UpdateCellMainThread"),
        G::CellMainThreadHere,
        &[],
        Open,
    ),
    sub(
        Direct(0x00c5_0610),
        &[0x0086_fd4a],
        Some("BSParticleSystemManager::UpdateParallel"),
        G::ParticlesThreaded,
        &[],
        Open,
    ),
    sub(
        Direct(0x00c5_0610),
        &[0x0086_fd5c],
        Some("BSParticleSystemManager::UpdateParallel"),
        G::ParticlesSingleThread,
        &[],
        Open,
    ),
];

/// `TES::UpdateCellAnimations(seconds)` (`00453550`): the wind for the
/// trees' modifier (`00c468c0`, `bswindmodifier.obj`) and Havok's wind
/// listener (`00c74550`), the time `011c3c08` advanced, between
/// `TES::LockHavokUpdateMT(1)` and `(0)` (a critical section only with
/// threads other than 1) the managed nodes (the Havok step) of the interior
/// or the exterior grid, the task queue (threads > 1), the collision
/// listener (`00623640`, `focollisionlistener.cpp`), with one thread the sky
/// and the main thread's part, and `ProcessLists::PostProcessProjectiles`.
pub const UPDATE_CELL_ANIMATIONS_STEPS: [SubStep; 13] = [
    sub(
        Direct(0x00c4_68c0),
        &[0x0045_3576],
        None,
        G::WindInterior,
        &[],
        Open,
    ),
    sub(
        Direct(0x00c7_4550),
        &[0x0045_3589],
        None,
        G::WindInterior,
        &[],
        Open,
    ),
    sub(
        Direct(0x00c4_68c0),
        &[0x0045_35bd],
        None,
        G::WindExterior,
        &[],
        Open,
    ),
    sub(
        Direct(0x00c7_4550),
        &[0x0045_35d2],
        None,
        G::WindExterior,
        &[],
        Partial("viewer: clutter::simulate (the sky's wind for the wind listener)"),
    ),
    sub(
        Direct(0x0045_3860),
        &[0x0045_35ee],
        Some("TES::LockHavokUpdateMT"),
        G::Always,
        &[],
        Open,
    ),
    sub(
        Direct(CELL_MANAGED_NODES),
        &[0x0045_3608],
        Some("TESObjectCELL::UpdateManagedNodes"),
        G::CellInterior,
        &[],
        Open,
    ),
    sub(
        Direct(GRID_MANAGED_NODES),
        &[0x0045_361f],
        Some("GridCellArray::UpdateManagedNodes"),
        G::CellExterior,
        &[],
        Open,
    ),
    sub(
        Direct(0x0045_3860),
        &[0x0045_3629],
        Some("TES::LockHavokUpdateMT"),
        G::Always,
        &[],
        Open,
    ),
    sub(
        Direct(0x0087_aa90),
        &[0x0045_3648],
        None,
        G::TaskQueueThreaded,
        &[],
        Open,
    ),
    sub(
        Direct(0x0062_3640),
        &[0x0045_3654],
        None,
        G::Always,
        &[],
        Open,
    ),
    sub(
        Direct(0x0063_ac70),
        &[0x0045_36ae],
        Some("Sky::Update"),
        G::SingleThread,
        &[],
        Open,
    ),
    sub(
        Direct(UPDATE_CELL_MAIN_THREAD),
        &[0x0045_36c5],
        Some("TES::UpdateCellMainThread"),
        G::SingleThreadMain,
        &[],
        Open,
    ),
    sub(
        Direct(0x0097_5080),
        &[0x0045_36cf],
        Some("ProcessLists::PostProcessProjectiles"),
        G::Always,
        &[],
        Open,
    ),
];

/// `TES::UpdateCellMainThread` (`004537c0`): `Sky::Update` with the frame
/// time (in dialogue `fAnimationMult` times `0084d030`'s, `004537d3`), then
/// the temporary node manager's update (`009611e0`'s object, `00a59c60`)
/// with the time `011c3c08`.
pub const UPDATE_CELL_MAIN_THREAD_STEPS: [SubStep; 2] = [
    sub(
        Direct(0x0063_ac70),
        &[0x0045_3811],
        Some("Sky::Update"),
        G::Always,
        &[],
        Partial(
            "viewer: weather::run_weather (the weather step); follow_sky, \
             daylight::follow_the_clock and the emittance pair after it, outside its gate",
        ),
    ),
    sub(
        Direct(0x00a5_9c60),
        &[0x0045_383a],
        None,
        G::Always,
        &[],
        Open,
    ),
];

/// `GridCellArray::UpdateManagedNodes(seconds)` (`004ba9a0`): the exterior
/// world's step first, then each loaded grid cell's managed nodes
/// (`GridCellArray::Get`, `004ba490`; a slot with a cell, `004baa7d`), the
/// flag the checkerboard test at `004baa20`-`004baa61` works out passed on.
pub const GRID_MANAGED_NODES_STEPS: [SubStep; 2] = [
    sub(
        Direct(EXTERIOR_WORLD_STEP),
        &[0x004b_a9b0],
        None,
        G::Always,
        &[],
        Open,
    ),
    sub(
        Direct(CELL_MANAGED_NODES),
        &[0x004b_aa98],
        Some("TESObjectCELL::UpdateManagedNodes"),
        G::Always,
        &[0x004b_a9f8, 0x004b_aa16, 0x004b_aa7d],
        Open,
    ),
];

/// The exterior world's step (`00554780`): `bhkWorld::Update` (slot +0xc4)
/// on the world in `011ca0d8` (`00559450`) when there is one (`00554790`).
pub const EXTERIOR_WORLD_STEP_STEPS: [SubStep; 1] = [sub(
    Virtual(HAVOK_STEP_SLOT),
    &[0x0055_47ad],
    Some("bhkWorld::Update"),
    G::Always,
    &[0x0055_4790],
    Partial("viewer: clutter::simulate (the bodies' world, outdoors)"),
)];

/// `TESObjectCELL::UpdateManagedNodes(seconds, flag)` (`00551890`): only its
/// first call is modelled, the Havok step of the cell's own world
/// (`004543c0`) for an interior (`00425fd0`, `005518af`) that is attached
/// (`00450ff0`, `005518c1`) and has a world (`005518d5`); the rest of its
/// 2,793 bytes (the cell's animated and Havok-driven nodes) is not split.
pub const CELL_MANAGED_NODES_STEPS: [SubStep; 1] = [sub(
    Virtual(HAVOK_STEP_SLOT),
    &[0x0055_18e5],
    Some("bhkWorld::Update"),
    G::Always,
    &[0x0055_18af, 0x0055_18c1, 0x0055_18d5],
    Partial("viewer: clutter::simulate (the bodies' world, indoors)"),
)];

/// `ProcessLists::RunActorUpdates` (`0096c7c0`): for each object of the
/// high process list (`ProcessLists` +4; `005de0f0`, `005be5c0`,
/// `00968670`) that is an actor (slot +0x100, `0096c823`) with `bProcessMe`
/// (+0xbc, `0096c837`), its slot +0x2f8 with 0.0: `Actor::Update`
/// (`00888b50`, the `Actor` and `Creature` vtables' slot, `0108454c`,
/// `010873a4`), or for a `Character` `008d3550` (`01086d64`), which calls
/// `Actor::Update` first (`008d3582`).
pub const RUN_ACTOR_UPDATES_STEPS: [SubStep; 1] = [sub(
    Virtual(0x2f8),
    &[0x0096_c84d],
    Some("Actor::Update"),
    G::Always,
    &[0x0096_c7f4, 0x0096_c80c, 0x0096_c823, 0x0096_c837],
    Open,
)];

/// `Actor::Update(seconds)` (`00888b50`): the calls doing its work, in
/// order. Not split further: the animation block flags at `0088aaad` and the
/// swim section's own calls after the water depth.
pub const ACTOR_UPDATE_STEPS: [SubStep; 19] = [
    sub(
        Direct(0x008b_bdb0),
        &[0x0088_8b9a],
        None,
        G::Always,
        &[],
        Open,
    ),
    sub(
        Virtual(0x5f8),
        &[0x0088_8ca0],
        None,
        G::ActorLongStep,
        &[0x0088_8c8e],
        Open,
    ),
    sub(
        Virtual(0x44c),
        &[0x0088_8cbb],
        None,
        G::ActorUpdates,
        &[],
        Open,
    ),
    sub(
        Direct(0x0088_b030),
        &[0x0088_8d40, 0x0088_8d8d],
        None,
        G::ActorUpdates,
        &[
            0x0088_8cc9,
            0x0088_8cec,
            0x0088_8d06,
            0x0088_8d53,
            0x0088_8d6d,
        ],
        Open,
    ),
    sub(
        Direct(0x00c6_d6b0),
        &[0x0088_8ebc],
        None,
        G::ActorControllerTarget,
        &[],
        Open,
    ),
    sub(
        Direct(0x0088_b070),
        &[0x0088_8f87],
        None,
        G::ActorController,
        &[],
        Open,
    ),
    sub(
        Virtual(0x704),
        &[0x0088_8fbf],
        None,
        G::ActorUpdates,
        &[0x0088_8f96],
        Open,
    ),
    sub(
        Direct(0x008a_1800),
        &[0x0088_9141, 0x0088_91b1],
        Some("Actor::SetLifeState"),
        G::ActorUpdates,
        &[
            0x0088_8f96,
            0x0088_9073,
            0x0088_90b9,
            0x0088_90cf,
            0x0088_9137,
            0x0088_91a7,
        ],
        Open,
    ),
    sub(
        Direct(0x0041_a540),
        &[0x0088_937d, 0x0088_95be],
        None,
        G::ActorUpdates,
        &[
            0x0088_91de,
            0x0088_91e8,
            0x0088_91f2,
            0x0088_9202,
            0x0088_9219,
            0x0088_9237,
            0x0088_927c,
            0x0088_928f,
        ],
        Open,
    ),
    sub(
        Direct(0x008b_39f0),
        &[0x0088_977a, 0x0088_97ff],
        None,
        G::ActorUpdates,
        &[
            0x0088_95cf,
            0x0088_9618,
            0x0088_966d,
            0x0088_9694,
            0x0088_9794,
        ],
        Open,
    ),
    sub(
        Direct(0x0044_0460),
        &[0x0088_9821],
        None,
        G::FirstPersonPlayer,
        &[],
        Open,
    ),
    sub(
        Direct(0x0095_2290),
        &[0x0088_982c],
        None,
        G::FirstPersonPlayer,
        &[],
        Open,
    ),
    sub(
        Direct(0x0088_b150),
        &[0x0088_9839],
        None,
        G::NotFirstPersonPlayer,
        &[],
        Partial("viewer: ai::move_actors (each person's 3D put where they moved)"),
    ),
    sub(
        Virtual(0x428),
        &[0x0088_9861],
        None,
        G::NotFirstPersonPlayer,
        &[],
        Open,
    ),
    sub(
        Direct(0x00b5_f080),
        &[0x0088_9898],
        None,
        G::ActorUpdates,
        &[0x0088_9875, 0x0088_9881],
        Open,
    ),
    sub(
        Direct(0x0088_b020),
        &[0x0088_989f],
        None,
        G::ActorUpdates,
        &[],
        Open,
    ),
    sub(
        Direct(0x0088_5560),
        &[0x0088_990c],
        None,
        G::ActorWater,
        &[0x0088_98ea],
        Open,
    ),
    sub(
        Direct(0x008b_3230),
        &[0x0088_afd9],
        None,
        G::ActorUpdates,
        &[],
        Open,
    ),
    sub(
        Direct(0x008c_1470),
        &[0x0088_afe4],
        None,
        G::ActorUpdates,
        &[],
        Open,
    ),
];

/// The modelled functions, callers first.
pub const FUNCTIONS: [Function; 8] = [
    Function {
        address: UPDATE_ANIMATIONS_AND_EFFECTS,
        size: 261,
        name: Some("Main::OnIdle_UpdateAnimationsAndEffects"),
        callers: &[Caller {
            function: ON_IDLE,
            site: 0x0086_ec9b,
        }],
        steps: &UPDATE_ANIMATIONS_AND_EFFECTS_STEPS,
    },
    Function {
        address: UPDATE_CELL_ANIMATIONS,
        size: 394,
        name: Some("TES::UpdateCellAnimations"),
        callers: &[
            Caller {
                function: UPDATE_ANIMATIONS_AND_EFFECTS,
                site: 0x0086_fd08,
            },
            Caller {
                function: 0x008c_7bd0,
                site: 0x008c_7d03,
            },
            Caller {
                function: 0x008c_7f50,
                site: 0x008c_803d,
            },
        ],
        steps: &UPDATE_CELL_ANIMATIONS_STEPS,
    },
    Function {
        address: UPDATE_CELL_MAIN_THREAD,
        size: 131,
        name: Some("TES::UpdateCellMainThread"),
        callers: &[
            Caller {
                function: UPDATE_ANIMATIONS_AND_EFFECTS,
                site: 0x0086_fd15,
            },
            Caller {
                function: UPDATE_CELL_ANIMATIONS,
                site: 0x0045_36c5,
            },
        ],
        steps: &UPDATE_CELL_MAIN_THREAD_STEPS,
    },
    Function {
        address: GRID_MANAGED_NODES,
        size: 269,
        name: Some("GridCellArray::UpdateManagedNodes"),
        callers: &[Caller {
            function: UPDATE_CELL_ANIMATIONS,
            site: 0x0045_361f,
        }],
        steps: &GRID_MANAGED_NODES_STEPS,
    },
    Function {
        address: EXTERIOR_WORLD_STEP,
        size: 51,
        name: None,
        callers: &[Caller {
            function: GRID_MANAGED_NODES,
            site: 0x004b_a9b0,
        }],
        steps: &EXTERIOR_WORLD_STEP_STEPS,
    },
    Function {
        address: CELL_MANAGED_NODES,
        size: 2793,
        name: Some("TESObjectCELL::UpdateManagedNodes"),
        callers: &[
            Caller {
                function: UPDATE_CELL_ANIMATIONS,
                site: 0x0045_3608,
            },
            Caller {
                function: GRID_MANAGED_NODES,
                site: 0x004b_aa98,
            },
        ],
        steps: &CELL_MANAGED_NODES_STEPS,
    },
    Function {
        address: RUN_ACTOR_UPDATES,
        size: 149,
        name: Some("ProcessLists::RunActorUpdates"),
        callers: &[
            Caller {
                function: 0x008c_7bd0,
                site: 0x008c_7cd6,
            },
            Caller {
                function: 0x008c_7da0,
                site: 0x008c_7ebe,
            },
        ],
        steps: &RUN_ACTOR_UPDATES_STEPS,
    },
    Function {
        address: ACTOR_UPDATE,
        size: 9413,
        name: Some("Actor::Update"),
        callers: &[Caller {
            function: RUN_ACTOR_UPDATES,
            site: 0x0096_c84d,
        }],
        steps: &ACTOR_UPDATE_STEPS,
    },
];

/// The function at `address` in [`FUNCTIONS`].
pub fn function(address: u32) -> Option<&'static Function> {
    FUNCTIONS.iter().find(|f| f.address == address)
}

/// Whether sub-step `i` of `f` is reached, given that `f` is called.
pub fn step_runs(s: &AiState, f: &Function, i: usize) -> bool {
    f.steps[i].gate.holds(s)
}

/// The sub-steps of `f` reached with these inputs.
pub fn steps_run(s: &AiState, f: &Function) -> Vec<usize> {
    (0..f.steps.len()).filter(|&i| step_runs(s, f, i)).collect()
}

/// The index of `f`'s sub-step whose call is at `site`.
pub fn step_at(f: &Function, site: u32) -> Option<usize> {
    f.steps.iter().position(|s| s.sites.contains(&site))
}

/// Whether thread step `i` of `t` makes its call with these inputs (the
/// thread runs: the frame's `AiThreadsStart` gate).
pub fn thread_call_runs(s: &AiState, t: &Thread, i: usize) -> bool {
    match t.steps[i].op {
        ThreadOp::Call { gate, .. } => gate.holds(s),
        _ => false,
    }
}

/// The caller through which a frame with `threads` reaches `f` first: the
/// frame, a thread function `CreateThreads` makes for that count, or a
/// modelled function's sub-step whose gate can hold in game mode with that
/// count (`f`'s own first such caller).
pub fn first_caller(f: &Function, threads: i32) -> Option<Caller> {
    let s = AiState {
        threads,
        ..AiState::default()
    };
    f.callers.iter().copied().find(|c| {
        if c.function == ON_IDLE {
            true
        } else if let Some(t) = thread(c.function) {
            threads_for(threads).iter().any(|x| x.address == t.address)
        } else {
            let caller = function(c.function).expect("a modelled caller");
            let k = step_at(caller, c.site).expect("the caller's call");
            caller.steps[k].gate.holds(&s) && first_caller(caller, threads).is_some()
        }
    })
}

/// The AI stage's inputs a frame's [`super::FrameState`] gives, with the
/// dialogue flag and whether an interior is loaded; the rest at
/// [`AiState::default`].
pub fn from_frame(frame: &super::FrameState, in_dialog: bool, interior_loaded: bool) -> AiState {
    AiState {
        threads: frame.threads,
        menu_flag: frame.menu_flag(),
        in_menu_mode: frame.in_menu_mode,
        in_dialog,
        fader_visible: frame.fader_visible,
        world_frozen: frame.world_frozen,
        obstacle_avoidance: frame.obstacle_avoidance,
        interior_loaded,
        ..AiState::default()
    }
}

/// Checks a translation's calls (`log`, in order) against `f`'s model with
/// inputs `s`, as `world::frame::world_time::follows` does: every reached
/// sub-step without own tests is called, in order, and no call goes to a
/// callee only unreached sub-steps have. `ignore`: callees the translation
/// calls as Rust functions (not in its log).
pub fn follows(f: &Function, s: &AiState, log: &[Callee], ignore: &[Callee]) -> Result<(), String> {
    let reached = steps_run(s, f);
    let must: Vec<Callee> = reached
        .iter()
        .map(|&i| &f.steps[i])
        .filter(|st| st.own_tests.is_empty() && !ignore.contains(&st.callee))
        .map(|st| st.callee)
        .collect();
    let known: Vec<Callee> = f.steps.iter().map(|st| st.callee).collect();
    let seen: Vec<Callee> = log.iter().copied().filter(|c| known.contains(c)).collect();
    let mut at = 0;
    for want in &must {
        match seen[at..].iter().position(|c| c == want) {
            Some(p) => at += p + 1,
            None => {
                return Err(format!(
                    "{want:?} not called in order; calls {seen:?}, want {must:?}"
                ))
            }
        }
    }
    let allowed: Vec<Callee> = reached.iter().map(|&i| f.steps[i].callee).collect();
    if let Some(c) = seen
        .iter()
        .find(|c| !allowed.contains(c) && !ignore.contains(c))
    {
        return Err(format!(
            "{c:?} called but only unreached sub-steps call it; calls {seen:?}"
        ));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::frame::{step_of, FrameState, Gate, Stage, STEPS};

    /// `research/engine-map/frame.tsv`, the committed call tree.
    const FRAME_TSV: &str = include_str!("../../../../research/engine-map/frame.tsv");

    /// The rows of `frame.tsv` as (depth, address).
    fn rows() -> Vec<(u32, u32)> {
        FRAME_TSV
            .lines()
            .filter(|l| !l.starts_with('#') && !l.starts_with("seq\t"))
            .map(|l| {
                let f: Vec<&str> = l.split('\t').collect();
                (
                    f[1].parse().unwrap(),
                    u32::from_str_radix(f[3], 16).unwrap(),
                )
            })
            .collect()
    }

    /// The direct calls `frame.tsv` lists under `0086fc60` (depth 1) or
    /// under one of its depth-2 rows.
    fn tsv_callees(address: u32) -> Vec<u32> {
        let rows = rows();
        let top = rows
            .iter()
            .position(|&(d, a)| d == 1 && a == UPDATE_ANIMATIONS_AND_EFFECTS)
            .expect("the step's row");
        let (depth, start) = if address == UPDATE_ANIMATIONS_AND_EFFECTS {
            (1, top)
        } else {
            let i = top
                + rows[top..]
                    .iter()
                    .position(|&(d, a)| d == 2 && a == address)
                    .expect("a depth-2 row");
            (2, i)
        };
        rows[start + 1..]
            .iter()
            .take_while(|&&(d, _)| d > depth)
            .filter(|&&(d, _)| d == depth + 1)
            .map(|&(_, a)| a)
            .collect()
    }

    /// `0086fc60`'s, `00453550`'s and `004537c0`'s direct calls appear in
    /// `frame.tsv` under them, in the same order.
    #[test]
    fn direct_calls_are_in_frame_tsv_in_order() {
        for address in [
            UPDATE_ANIMATIONS_AND_EFFECTS,
            UPDATE_CELL_ANIMATIONS,
            UPDATE_CELL_MAIN_THREAD,
        ] {
            let f = function(address).unwrap();
            let listed = tsv_callees(address);
            let mut at = 0;
            for st in f.steps {
                if let Callee::Direct(a) = st.callee {
                    for _ in st.sites {
                        let p = listed[at..]
                            .iter()
                            .position(|&x| x == a)
                            .unwrap_or_else(|| panic!("{address:08x}: {a:08x} not in order"));
                        at += p + 1;
                    }
                }
            }
        }
    }

    #[test]
    fn thread_steps_are_inside_their_functions_in_order() {
        for t in [&COMBINED, &THREAD_1, &THREAD_2] {
            assert!(t.steps.windows(2).all(|w| w[0].site < w[1].site));
            assert!(
                t.steps.iter().all(|s| t.contains(s.site)),
                "{:08x}",
                t.address
            );
            // Every work call is named; the obstacle manager is the only gated
            // pair.
            for s in t.steps {
                if let ThreadOp::Call {
                    name, gate, callee, ..
                } = s.op
                {
                    assert!(name.is_some());
                    assert_eq!(
                        gate == ThreadGate::ObstacleAvoidance,
                        matches!(callee, 0x006c_0720 | 0x006c_3640)
                    );
                }
            }
        }
        // The work of the pair is the combined function's, apart from the
        // per-thread input and the scripts' optimizations, which each thread
        // does for itself.
        let per_thread = [0x0087_a6b0, 0x0087_a6d0, 0x005a_e270, 0x005a_9d60];
        let mut combined: Vec<u32> = calls(&COMBINED);
        let mut pair: Vec<u32> = [calls(&THREAD_1), calls(&THREAD_2)].concat();
        combined.retain(|c| !per_thread.contains(c));
        pair.retain(|c| !per_thread.contains(c));
        combined.sort_unstable();
        pair.sort_unstable();
        assert_eq!(combined, pair);
        let own = calls(&THREAD_2);
        assert_eq!(own.iter().filter(|c| per_thread.contains(c)).count(), 4);
    }

    fn calls(t: &Thread) -> Vec<u32> {
        t.steps
            .iter()
            .filter_map(|s| match s.op {
                ThreadOp::Call { callee, .. } => Some(callee),
                _ => None,
            })
            .collect()
    }

    // `0086b117` (no threads with 1), `008c7307` (one combined thread up to 2).
    #[test]
    fn create_threads_by_count() {
        assert!(threads_for(1).is_empty());
        assert_eq!(threads_for(2), [COMBINED]);
        assert_eq!(threads_for(3), [THREAD_1, THREAD_2]);
        assert_eq!(threads_for(16), [THREAD_1, THREAD_2]);
        assert!(schedule(1).is_empty());
    }

    /// With two threads the order is the combined function's, with the
    /// render before its render wait (`008c7d18`).
    #[test]
    fn two_threads_run_the_combined_order() {
        let s = schedule(2);
        let mut want: Vec<Slot> = (0..COMBINED.steps.len())
            .map(|i| Slot::Step(COMBINED.address, i))
            .collect();
        let render = COMBINED
            .steps
            .iter()
            .position(|s| s.op == ThreadOp::WaitRender)
            .unwrap();
        assert_eq!(COMBINED.steps[render].site, 0x008c_7d18);
        want.insert(render, Slot::Render);
        assert_eq!(s, want);
    }

    fn sites(calls: &[(u32, usize, bool)]) -> Vec<u32> {
        calls
            .iter()
            .map(|&(t, i, _)| thread(t).unwrap().steps[i].site)
            .collect()
    }

    /// With more, every wait comes after its signal, each thread keeps its
    /// order, the render comes once before both render waits are passed,
    /// and every step is there once.
    #[test]
    fn the_pair_keeps_its_handshakes() {
        let s = schedule(4);
        let mut signalled = Vec::new();
        let mut rendered = false;
        let mut next = [0usize; 2];
        for slot in &s {
            match *slot {
                Slot::Render => {
                    assert!(!rendered);
                    rendered = true;
                }
                Slot::Step(t, i) => {
                    let k = usize::from(t == THREAD_2.address);
                    assert_eq!(i, next[k]);
                    next[k] += 1;
                    let th = thread(t).unwrap();
                    match th.steps[i].op {
                        ThreadOp::Signal { thread, stage } => signalled.push((thread, stage)),
                        ThreadOp::Wait { thread, stage } => assert!(
                            signalled.contains(&(thread, stage)),
                            "{:08x}",
                            th.steps[i].site
                        ),
                        ThreadOp::WaitRender => assert!(rendered),
                        _ => {}
                    }
                }
            }
        }
        assert_eq!(next, [THREAD_1.steps.len(), THREAD_2.steps.len()]);
        // In this order thread 2's Havok step (`008c803d`) comes before
        // thread 1's actor updates (`008c7ebe`), which wait for its movement
        // (`008c7fc8`, signalled at stage 3).
        let order = sites(&scheduled_calls(4));
        let at = |site: u32| order.iter().position(|&s| s == site).unwrap();
        assert!(at(0x008c_7fc8) < at(0x008c_7ebe));
        assert!(at(0x008c_803d) < at(0x008c_7ebe));
        // The movement waits for thread 1's animations (stage 2); the Havok
        // step for its combat update (stage 4).
        assert!(at(0x008c_7dfd) < at(0x008c_7fc8));
        assert!(at(0x008c_7e33) < at(0x008c_803d));
        // After the render: thread 1's obstacle manager and optimizations.
        let after: Vec<(u32, usize, bool)> =
            scheduled_calls(4).into_iter().filter(|c| c.2).collect();
        assert_eq!(
            sites(&after),
            [0x008c_7f25, 0x008c_7f2c, 0x008c_7f31, 0x008c_7f36]
        );
        let after: Vec<(u32, usize, bool)> =
            scheduled_calls(2).into_iter().filter(|c| c.2).collect();
        assert_eq!(
            sites(&after),
            [
                0x008c_7d2e,
                0x008c_7d35,
                0x008c_7d41,
                0x008c_7d46,
                0x008c_7d4b
            ]
        );
    }

    /// The frame starts the threads in stage 6 and joins them in stage 8
    /// under the same test, with `Main::Swap` (whose last call lets the
    /// render wait pass) between.
    #[test]
    fn the_frame_starts_and_joins_the_threads() {
        let start = step_of(START_THREADS).unwrap();
        let join = step_of(WAIT_FOR_THREADS).unwrap();
        let swap = step_of(0x0086_ff70).unwrap();
        assert!(start < swap && swap < join);
        assert_eq!(STEPS[start].gate, Gate::AiThreadsStart);
        assert_eq!(STEPS[join].gate, Gate::AiThreadsJoin);
        assert_eq!(STEPS[start].stage, Stage::AiStart);
        assert_eq!(STEPS[join].stage, Stage::AiJoin);
        assert_eq!(step_of(UPDATE_ANIMATIONS_AND_EFFECTS), Some(start + 2));
        for threads in [2, 4] {
            let menu = FrameState {
                in_menu_mode: true,
                threads,
                ..FrameState::default()
            };
            assert!(!Gate::AiThreadsStart.holds(&menu));
            let game = FrameState {
                threads,
                ..FrameState::default()
            };
            assert!(Gate::AiThreadsStart.holds(&game) && Gate::AiThreadsJoin.holds(&game));
        }
    }

    #[test]
    fn sites_and_tests_are_inside_their_functions_in_order() {
        for f in &FUNCTIONS {
            let sites: Vec<u32> = f
                .steps
                .iter()
                .flat_map(|s| s.sites.iter().copied())
                .collect();
            assert!(sites.windows(2).all(|w| w[0] < w[1]), "{:08x}", f.address);
            assert!(sites.iter().all(|&a| f.contains(a)), "{:08x}", f.address);
            for st in f.steps {
                let b = st.gate.branches();
                assert_eq!(
                    b.is_empty(),
                    st.gate == SubGate::Always,
                    "{:08x}",
                    st.sites[0]
                );
                let last = *st.sites.last().unwrap();
                assert!(
                    b.iter()
                        .chain(st.own_tests)
                        .all(|&a| f.contains(a) && a < last),
                    "{:08x}",
                    st.sites[0]
                );
            }
            // Each caller calls it there.
            for c in f.callers {
                if c.function == ON_IDLE {
                    let i = step_of(f.address).expect("a frame step");
                    assert_eq!(STEPS[i].stage, Stage::AiStart);
                } else if let Some(t) = thread(c.function) {
                    let i = t.steps.iter().position(|s| s.site == c.site).unwrap();
                    assert!(matches!(
                        t.steps[i].op,
                        ThreadOp::Call { callee, .. } if callee == f.address
                    ));
                } else {
                    let caller = function(c.function).expect("a modelled caller");
                    let k = step_at(caller, c.site).expect("a sub-step");
                    // Directly, or through a vtable (`Actor::Update`, below).
                    let callee = caller.steps[k].callee;
                    assert!(
                        callee == Callee::Direct(f.address) || matches!(callee, Callee::Virtual(_)),
                        "{:08x}",
                        c.site
                    );
                }
            }
        }
        // `RunActorUpdates`' slot +0x2f8 is `Actor::Update`'s call.
        let r = function(RUN_ACTOR_UPDATES).unwrap();
        assert_eq!(r.steps[0].callee, Callee::Virtual(0x2f8));
        assert_eq!(
            function(ACTOR_UPDATE).unwrap().callers[0].site,
            r.steps[0].sites[0]
        );
    }

    fn reached(f: &Function, s: &AiState) -> Vec<u32> {
        steps_run(s, f)
            .iter()
            .map(|&i| f.steps[i].sites[0])
            .collect()
    }

    // `0086fc72`-`0086fd3d`.
    #[test]
    fn animations_and_effects() {
        let f = function(UPDATE_ANIMATIONS_AND_EFFECTS).unwrap();
        let game = AiState::default();
        assert_eq!(reached(f, &game), [0x0086_fcbf, 0x0086_fd15, 0x0086_fd4a]);
        let one = AiState { threads: 1, ..game };
        assert_eq!(reached(f, &one), [0x0086_fcd9, 0x0086_fd08, 0x0086_fd5c]);
        let menu = AiState {
            menu_flag: true,
            in_menu_mode: true,
            ..game
        };
        assert!(reached(f, &menu).is_empty());
        // Dialogue (and the fader) let it run in menu mode; with one thread
        // the main thread then takes the sky's part.
        for open in [
            AiState {
                in_dialog: true,
                ..menu
            },
            AiState {
                fader_visible: true,
                ..menu
            },
        ] {
            assert_eq!(reached(f, &open), [0x0086_fcbf, 0x0086_fd15, 0x0086_fd4a]);
            let one = AiState { threads: 1, ..open };
            assert_eq!(reached(f, &one), [0x0086_fcd9, 0x0086_fd15, 0x0086_fd5c]);
        }
        let frozen = AiState {
            world_frozen: true,
            ..game
        };
        assert!(reached(f, &frozen).is_empty());
    }

    // `00453568`-`004536c0`.
    #[test]
    fn cell_animations() {
        let f = function(UPDATE_CELL_ANIMATIONS).unwrap();
        let game = AiState::default();
        assert_eq!(
            reached(f, &game),
            [
                0x0045_35bd,
                0x0045_35d2,
                0x0045_35ee,
                0x0045_361f,
                0x0045_3629,
                0x0045_3648,
                0x0045_3654,
                0x0045_36cf
            ]
        );
        let inside = AiState {
            interior_loaded: true,
            threads: 1,
            ..game
        };
        assert_eq!(
            reached(f, &inside),
            [
                0x0045_3576,
                0x0045_3589,
                0x0045_35ee,
                0x0045_3608,
                0x0045_3629,
                0x0045_3654,
                0x0045_36ae,
                0x0045_36c5,
                0x0045_36cf
            ]
        );
        let m = function(UPDATE_CELL_MAIN_THREAD).unwrap();
        assert_eq!(steps_run(&game, m).len(), 2);
    }

    // `00888c5e`-`008898c7`.
    #[test]
    fn actor_update() {
        let f = function(ACTOR_UPDATE).unwrap();
        let npc = AiState::default();
        let r = reached(f, &npc);
        assert!(r.contains(&0x0088_8ebc) && r.contains(&0x0088_9839));
        assert!(r.contains(&0x0088_990c));
        assert!(!r.contains(&0x0088_8ca0) && !r.contains(&0x0088_9821));
        assert_eq!(*r.last().unwrap(), 0x0088_afe4);
        let not_ready = AiState {
            actor_ready: false,
            ..npc
        };
        assert_eq!(reached(f, &not_ready), [0x0088_8b9a]);
        let long = AiState {
            actor_long_step: true,
            ..npc
        };
        assert_eq!(reached(f, &long), [0x0088_8b9a, 0x0088_8ca0]);
        let player = AiState {
            actor_is_player: true,
            ..npc
        };
        let r = reached(f, &player);
        assert!(r.contains(&0x0088_9821) && r.contains(&0x0088_982c));
        assert!(!r.contains(&0x0088_8ebc) && !r.contains(&0x0088_9839));
        assert!(!r.contains(&0x0088_990c));
        // In third person the player is placed as anyone else.
        let third = AiState {
            third_person: true,
            ..player
        };
        let r = reached(f, &third);
        assert!(r.contains(&0x0088_9839) && r.contains(&0x0088_990c));
        assert!(!r.contains(&0x0088_9821));
        let menu = AiState {
            in_menu_mode: true,
            ..npc
        };
        assert!(!reached(f, &menu).contains(&0x0088_990c));
        let no_controller = AiState {
            actor_controller: false,
            ..npc
        };
        let r = reached(f, &no_controller);
        assert!(!r.contains(&0x0088_8ebc) && !r.contains(&0x0088_8f87));
        assert!(!r.contains(&0x0088_990c));
    }

    // The callers a thread count reaches first.
    #[test]
    fn first_callers() {
        let cells = function(UPDATE_CELL_ANIMATIONS).unwrap();
        assert_eq!(first_caller(cells, 1).unwrap().site, 0x0086_fd08);
        assert_eq!(first_caller(cells, 2).unwrap().site, 0x008c_7d03);
        assert_eq!(first_caller(cells, 8).unwrap().site, 0x008c_803d);
        let main = function(UPDATE_CELL_MAIN_THREAD).unwrap();
        assert_eq!(first_caller(main, 2).unwrap().site, 0x0086_fd15);
        assert_eq!(first_caller(main, 1).unwrap().site, 0x0045_36c5);
        let updates = function(RUN_ACTOR_UPDATES).unwrap();
        assert_eq!(first_caller(updates, 2).unwrap().site, 0x008c_7cd6);
        assert_eq!(first_caller(updates, 3).unwrap().site, 0x008c_7ebe);
        assert!(first_caller(updates, 1).is_none());
        let grid = function(GRID_MANAGED_NODES).unwrap();
        assert_eq!(first_caller(grid, 4).unwrap().site, 0x0045_361f);
    }

    #[test]
    fn from_the_frame() {
        let menu = FrameState {
            in_menu_mode: true,
            threads: 6,
            obstacle_avoidance: false,
            ..FrameState::default()
        };
        let s = from_frame(&menu, true, true);
        assert!(s.menu_flag && s.in_menu_mode && s.in_dialog && s.interior_loaded);
        assert_eq!(s.threads, 6);
        assert!(!s.obstacle_avoidance);
        assert!(s.animations_run());
        let obstacle = COMBINED.call_of(0x006c_3640).unwrap();
        assert!(!thread_call_runs(&s, &COMBINED, obstacle));
        let updates = COMBINED.call_of(RUN_ACTOR_UPDATES).unwrap();
        assert!(thread_call_runs(&s, &COMBINED, updates));
    }

    #[test]
    fn follows_checks_order_and_unreached_calls() {
        let f = function(UPDATE_CELL_MAIN_THREAD).unwrap();
        let s = AiState::default();
        let ok = [Callee::Direct(0x0063_ac70), Callee::Direct(0x00a5_9c60)];
        assert!(follows(f, &s, &ok, &[]).is_ok());
        let wrong = [Callee::Direct(0x00a5_9c60), Callee::Direct(0x0063_ac70)];
        assert!(follows(f, &s, &wrong, &[]).is_err());
        let g = function(UPDATE_ANIMATIONS_AND_EFFECTS).unwrap();
        let unreached = [Callee::Direct(0x0097_4420)];
        assert!(follows(g, &s, &unreached, &[]).is_err());
    }

    #[test]
    fn wiring() {
        let partial: Vec<u32> = FUNCTIONS
            .iter()
            .flat_map(|f| f.steps.iter())
            .filter(|s| matches!(s.wiring, Wiring::Partial(_)))
            .map(|s| s.sites[0])
            .collect();
        assert_eq!(
            partial,
            [
                0x0045_35d2,
                0x0045_3811,
                0x0055_47ad,
                0x0055_18e5,
                0x0088_9839
            ]
        );
        let threads: Vec<u32> = COMBINED
            .steps
            .iter()
            .filter(|s| {
                matches!(
                    s.op,
                    ThreadOp::Call {
                        wiring: Wiring::Partial(_),
                        ..
                    }
                )
            })
            .map(|s| s.site)
            .collect();
        assert_eq!(
            threads,
            [
                0x008c_7be9,
                0x008c_7c2f,
                0x008c_7c4e,
                0x008c_7cba,
                0x008c_7ce0
            ]
        );
        assert!(FUNCTIONS
            .iter()
            .flat_map(|f| f.steps.iter())
            .all(|s| s.wiring.is_open()));
    }
}

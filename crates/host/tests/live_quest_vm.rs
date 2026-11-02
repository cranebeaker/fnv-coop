//! Opt-in: the world's quest runs through the game's interpreter
//! ([`host::quest_vm`], the hook [`world::scripting::QuestVm`]) on the
//! player's `FalloutNV.esm`.
//!
//! ```text
//! NV_DATA="C:\...\Fallout New Vegas\Data" cargo test -p host --release \
//!     --test live_quest_vm -- --ignored --nocapture
//! ```

use esm::{ActivePlugins, FormId, FourCC, LoadOrder};
use std::path::PathBuf;
use world::scripting::{script_of, GameState, Runner, ScriptCache};

fn data() -> PathBuf {
    PathBuf::from(std::env::var("NV_DATA").expect("NV_DATA: the game's Data folder"))
}

fn variable(state: &GameState, quest: FormId, name: &str) -> f64 {
    state
        .variables
        .get(&quest)
        .and_then(|l| l.get(name))
        .unwrap_or(0.0)
}

/// Which quests the viewer runs through the game's interpreter, and that
/// running them through the hook keeps their variables.
#[test]
#[ignore = "needs the player's FalloutNV.exe and FalloutNV.esm (set NV_DATA)"]
fn the_viewers_quests_run_through_the_games_interpreter() {
    let data = data();
    let mut host = host::Host::start(&data).unwrap_or_else(|e| panic!("{e}"));
    let order = LoadOrder::from_data_dir(&data, &ActivePlugins::OfficialOnly).expect("load order");
    let scripts = ScriptCache::default();
    let (taken, scripted) = host::quest_vm::eligible(&order, &scripts);
    println!("the game's interpreter takes {taken} of {scripted} scripted quests:");
    let mut state = GameState::new(&order);
    let mut running_taken = Vec::new();
    for rr in order.records_of_type(FourCC(*b"QUST")) {
        let Some(id) = script_of(&order, rr.form_id) else {
            continue;
        };
        let Some(parsed) = scripts.script(&order, id) else {
            continue;
        };
        if host::quest_vm::takes(&parsed) {
            let name = rr.editor_id().ok().flatten().unwrap_or_default();
            println!("  {name} {:?}", rr.form_id);
            if state.running.contains(&rr.form_id) {
                running_taken.push(rr.form_id);
            }
        }
    }
    assert!(taken > 0 && taken < scripted);
    // A new game's first frames: the running quests it takes run through
    // it (their scripts are variable holders: nothing changes).
    for q in &running_taken {
        state
            .variables
            .entry(*q)
            .or_default()
            .insert("probe", script::VarKind::Integer, 0.0);
    }
    let before = state.variables.clone();
    for _ in 0..3 {
        Runner::new(&order, &scripts, &mut state)
            .with_quest_vm(&mut host)
            .update(2.0);
    }
    println!("{}", host.quests.summary());
    // Quests of plugins with masters (the DLCs) can't be loaded yet (their
    // form ids need the master table TESFile::OpenTES builds): they go back
    // to crates/script, and run there. Every other one runs through the
    // game's interpreter.
    for (q, why) in &host.quests.stopped {
        println!("  back to crates/script: {q:?}: {why}");
        assert!(why.contains("has masters"), "{q:?}: {why}");
    }
    assert_eq!(
        host.quests.quests_run() + host.quests.stopped.len(),
        running_taken.len()
    );
    assert!(host.quests.quests_run() > 0);
    for q in &running_taken {
        let a = &before[q];
        let b = &state.variables[q];
        for (name, _, value) in a.iter() {
            assert_eq!(b.get(name), Some(value), "{q:?}.{name}");
        }
    }
}

/// `VMS49Timer` counts its timer down with the world's time through the
/// game's interpreter exactly as through `crates/script`, run by run. Its
/// script calls `EVP` and `StopQuest` once the timer is out, so the viewer
/// doesn't give it to the game's interpreter ([`host::quest_vm::takes`]);
/// here the test takes it, and stops while the timer is still above 0.
#[test]
#[ignore = "needs the player's FalloutNV.exe and FalloutNV.esm (set NV_DATA)"]
fn vms49_timer_advances_with_the_worlds_time_through_the_hook() {
    let data = data();
    let mut host = host::Host::start(&data).unwrap_or_else(|e| panic!("{e}"));
    let order = LoadOrder::from_data_dir(&data, &ActivePlugins::OfficialOnly).expect("load order");
    let scripts = ScriptCache::default();
    let quest = order
        .records_of_type(FourCC(*b"QUST"))
        .find(|rr| rr.editor_id().ok().flatten().as_deref() == Some("VMS49Timer"))
        .expect("VMS49Timer")
        .form_id;
    let parsed = scripts
        .script(&order, script_of(&order, quest).unwrap())
        .unwrap();
    assert!(
        !host::quest_vm::takes(&parsed),
        "it calls EVP and StopQuest"
    );
    host.quests.accept = |_| true;

    // Two games, the quest alone running with its own delay of 0.1 s (as
    // `SetQuestDelay` sets it): one through the game's interpreter, one
    // through crates/script.
    let start = || {
        let mut state = GameState::new(&order);
        state.running.clear();
        state.running.insert(quest);
        state.quest_delays.insert(quest, 0.1);
        state
    };
    let (mut game, mut world_only) = (start(), start());
    let mut runs = 0;
    for frame in 0..16 {
        Runner::new(&order, &scripts, &mut game)
            .with_quest_vm(&mut host)
            .update(0.05);
        Runner::new(&order, &scripts, &mut world_only).update(0.05);
        let (a, b) = (
            variable(&game, quest, "fTimer"),
            variable(&world_only, quest, "fTimer"),
        );
        println!("frame {frame}: game's interpreter {a:.4}, crates/script {b:.4}");
        assert!((a - b).abs() < 1e-6, "frame {frame}: {a} vs {b}");
        assert_eq!(
            variable(&game, quest, "bInitialized"),
            variable(&world_only, quest, "bInitialized")
        );
        assert!(a > 0.0, "the test stops before the timer runs out");
        runs = host.quests.runs;
    }
    // Ran at once (the frame's 0.05), then every other frame (0.1 each).
    assert_eq!(runs, 8);
    assert!((variable(&game, quest, "fTimer") - 0.25).abs() < 1e-6);
    assert_eq!(variable(&game, quest, "bInitialized"), 1.0);
}

//! Opt-in acceptance: a real quest script of the player's `FalloutNV.esm`
//! run by the game's own interpreter in the host.
//!
//! `VMS49Timer` (quest) runs `VMS49TimerSCRIPT`:
//!
//! ```text
//! float fTimer;
//! short bInitialized;
//! BEGIN GameMode
//!     if (bInitialized == 0)
//!         set bInitialized to 1;
//!         set fTimer to 1.0;
//!     endif
//!     if (fTimer > 0)
//!         set fTimer to fTimer - GetSecondsPassed;
//!     else
//!         ...
//! ```
//!
//! The quest is made with the game's constructor and its script compiled
//! code (`SCDA`) is run through `0060d4d0` → `Script::Run` →
//! `ScriptRunManager::Run` → `ScriptRunner::Run` → `EvaluateLine` /
//! `CExpression::Eval`, all translated; its variables live in the game's
//! `ScriptLocals`.
//!
//! ```text
//! NV_DATA="C:\...\Fallout New Vegas\Data" cargo test -p host --release \
//!     --test live_quest_script -- --ignored --nocapture
//! ```

use esm::{ActivePlugins, FourCC, LoadOrder};
use std::path::PathBuf;

fn variable(state: &host::quest_form::QuestState, name: &str) -> f64 {
    state
        .variables
        .iter()
        .find(|(n, _)| n.eq_ignore_ascii_case(name))
        .unwrap_or_else(|| panic!("no variable {name}"))
        .1
}

#[test]
#[ignore = "needs the player's FalloutNV.exe and FalloutNV.esm (set NV_DATA)"]
fn vms49_timer_initializes_through_the_games_interpreter() {
    let data = PathBuf::from(std::env::var("NV_DATA").expect("NV_DATA: the game's Data folder"));
    let mut host = host::Host::start(&data).unwrap_or_else(|e| panic!("{e}"));
    let order = LoadOrder::from_data_dir(&data, &ActivePlugins::OfficialOnly).expect("load order");
    let id = order
        .records_of_type(FourCC(*b"QUST"))
        .find(|rr| rr.editor_id().ok().flatten().as_deref() == Some("VMS49Timer"))
        .expect("quest VMS49Timer")
        .form_id;
    let e = &mut host.engine;
    let (script, quest) = host.quests.make(e, &order, id).expect("VMS49Timer");
    // TESQuest::Load read its stages; VMS49Timer has none.
    println!("stages: {:?}", quest.stages(e));

    let before = quest.snapshot(e, &script);
    assert_eq!(variable(&before, "bInitialized"), 0.0);
    assert_eq!(variable(&before, "fTimer"), 0.0);

    // First GameMode run: the first block initializes; the second takes
    // fTimer > 0 and subtracts the seconds passed (none yet).
    quest.run(e);
    let first = quest.snapshot(e, &script);
    println!("after the first run: {first:?}");
    assert_eq!(variable(&first, "bInitialized"), 1.0);
    assert_eq!(variable(&first, "fTimer"), 1.0);

    // Already initialized: the first block is skipped, so a changed timer
    // isn't reset.
    let locals = quest.locals(e);
    let timer = script
        .variables
        .iter()
        .find(|v| v.name.eq_ignore_ascii_case("fTimer"))
        .unwrap();
    e.call(0x005a_9290, &[locals, timer.id, 0, 0x3fe0_0000]); // SetVariable 0.5
    quest.run(e);
    let second = quest.snapshot(e, &script);
    println!("after the second run: {second:?}");
    assert_eq!(variable(&second, "bInitialized"), 1.0);
    assert_eq!(variable(&second, "fTimer"), 0.5);
}

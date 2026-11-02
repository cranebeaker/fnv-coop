//! Opt-in: quests loaded by the game's own loaders (`TESFile::SetOffset`,
//! `TESQuest::Load`, `InitItem`) from the player's `FalloutNV.esm` get their
//! stages, and `SetStage`'s path (`TESQuest::SetStageDone` `0060d510` → the
//! stage's mark-done `0060f4a0` → `TESQuestStage::SetDone` `0060fb60` for each
//! stage item) and `GetStage`'s (`TESQuest::GetCurrentStage` `0060d700`) run
//! on them.
//!
//! ```text
//! NV_DATA="C:\...\Fallout New Vegas\Data" cargo test -p host --release \
//!     --test live_quest_stages -- --ignored --nocapture
//! ```

use esm::{ActivePlugins, FormId, FourCC, LoadOrder};
use std::path::PathBuf;

const SET_STAGE_DONE: u32 = 0x0060_d510;
const STAGE_SET_DONE: u32 = 0x0060_fb60;
const GET_CURRENT_STAGE: u32 = 0x0060_d700;

fn quest(order: &LoadOrder, name: &str) -> FormId {
    order
        .records_of_type(FourCC(*b"QUST"))
        .find(|rr| rr.editor_id().ok().flatten().as_deref() == Some(name))
        .unwrap_or_else(|| panic!("quest {name}"))
        .form_id
}

/// The stage numbers the record names (`INDX`), in order.
fn record_stages(order: &LoadOrder, id: FormId) -> Vec<u8> {
    order
        .get(id)
        .unwrap()
        .record()
        .unwrap()
        .subrecords
        .iter()
        .filter(|s| &s.kind.0 == b"INDX")
        .filter_map(|s| s.data.first().copied())
        .collect()
}

#[test]
#[ignore = "needs the player's FalloutNV.exe and FalloutNV.esm (set NV_DATA)"]
fn quests_load_their_stages_and_set_stage_runs_through_set_done() {
    let data = PathBuf::from(std::env::var("NV_DATA").expect("NV_DATA: the game's Data folder"));
    let mut host = host::Host::start(&data).unwrap_or_else(|e| panic!("{e}"));
    let order = LoadOrder::from_data_dir(&data, &ActivePlugins::OfficialOnly).expect("load order");

    // Stages, as TESQuest::Load read them from the record.
    for name in ["VMS49Timer", "VMS03", "VMQHouseLockdown"] {
        let id = quest(&order, name);
        let (_, q) = host.quests.make(&mut host.engine, &order, id).expect(name);
        let stages = q.stages(&mut host.engine);
        println!("{name}: stages {stages:?}");
        assert_eq!(stages, record_stages(&order, id), "{name}");
    }

    // A stage without items: SetStage sets it done, GetStage reads it.
    let (_, vms03) = host
        .quests
        .make(&mut host.engine, &order, quest(&order, "VMS03"))
        .unwrap();
    let e = &mut host.engine;
    assert_eq!(e.call(GET_CURRENT_STAGE, &[vms03.form]).u32() & 0xff, 0);
    assert_eq!(e.call(SET_STAGE_DONE, &[vms03.form, 5]).u32() & 0xff, 1);
    assert_eq!(e.call(GET_CURRENT_STAGE, &[vms03.form]).u32() & 0xff, 5);
    // A stage it doesn't have: not found, nothing changes.
    assert_eq!(e.call(SET_STAGE_DONE, &[vms03.form, 250]).u32() & 0xff, 0);
    assert_eq!(e.call(GET_CURRENT_STAGE, &[vms03.form]).u32() & 0xff, 5);

    // A stage with items: SetStage reaches TESQuestStage::SetDone for its
    // first item. SetDone then needs functions not translated yet (the
    // first is 004b0fc0), so the run stops there; the test records how far
    // it got.
    let (_, lockdown) = host
        .quests
        .make(&mut host.engine, &order, quest(&order, "VMQHouseLockdown"))
        .unwrap();
    let e = &mut host.engine;
    let first = lockdown.stages(e)[0];
    e.call_log = Some(vec![]);
    let hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(|_| {}));
    let stop = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        e.call(SET_STAGE_DONE, &[lockdown.form, u32::from(first)]);
    }));
    std::panic::set_hook(hook);
    let log = e.call_log.take().unwrap();
    assert!(
        log.iter().any(|(a, _)| *a == STAGE_SET_DONE),
        "SetStage reaches TESQuestStage::SetDone"
    );
    let stop = stop
        .err()
        .and_then(|p| p.downcast_ref::<String>().cloned())
        .unwrap_or_else(|| "ran to the end".into());
    println!("VMQHouseLockdown stage {first}: SetDone reached; {stop}");
    assert_eq!(
        e.call(GET_CURRENT_STAGE, &[lockdown.form]).u32() & 0xff,
        u32::from(first)
    );
}

//! Opt-in survey: every quest of the player's `FalloutNV.esm` that has a
//! script, loaded by the game's own loaders (`TESFile::SetOffset`,
//! `TESQuest::Load`, `InitItem`: [`host::quest_vm::QuestVm::make`]) and run
//! once through the game's interpreter (`0060d4d0` → `Script::Run`),
//! recording its stages, what each run changed and where loading or running
//! stopped.
//!
//! ```text
//! NV_DATA="C:\...\Fallout New Vegas\Data" cargo test -p host --release \
//!     --test live_quest_survey -- --ignored --nocapture
//! ```

use esm::{ActivePlugins, FourCC, LoadOrder};
use std::collections::BTreeMap;
use std::path::PathBuf;
use world::scripting::script_of;

fn stop_of(payload: Box<dyn std::any::Any + Send>) -> String {
    let text = payload
        .downcast_ref::<String>()
        .cloned()
        .or_else(|| payload.downcast_ref::<&str>().map(|s| s.to_string()))
        .unwrap_or_default();
    text.lines().next().unwrap_or("").to_string()
}

#[test]
#[ignore = "needs the player's FalloutNV.exe and FalloutNV.esm (set NV_DATA)"]
fn every_scripted_quest_loads_and_runs_once() {
    let data = PathBuf::from(std::env::var("NV_DATA").expect("NV_DATA: the game's Data folder"));
    let mut host = host::Host::start(&data).unwrap_or_else(|e| panic!("{e}"));
    let order = LoadOrder::from_data_dir(&data, &ActivePlugins::OfficialOnly).expect("load order");
    let hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(|_| {}));
    let mut stops: BTreeMap<String, Vec<String>> = BTreeMap::new();
    let (mut quests, mut loaded, mut with_stages, mut stages, mut ran) = (0, 0, 0, 0, 0);
    let mut changed = Vec::new();
    let mut set_ok = Vec::new();
    let ids: Vec<_> = order
        .records_of_type(FourCC(*b"QUST"))
        .filter(|rr| rr.plugin.name.eq_ignore_ascii_case("FalloutNV.esm"))
        .filter(|rr| script_of(&order, rr.form_id).is_some())
        .map(|rr| {
            (
                rr.form_id,
                rr.editor_id().ok().flatten().unwrap_or_default(),
            )
        })
        .collect();
    for (id, name) in ids {
        quests += 1;
        let host = &mut host;
        let made = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            host.quests.make(&mut host.engine, &order, id)
        }));
        let (script, quest) = match made {
            Ok(Ok(m)) => m,
            Ok(Err(err)) => {
                stops.entry(format!("load: {err}")).or_default().push(name);
                continue;
            }
            Err(p) => {
                stops
                    .entry(format!("load: {}", stop_of(p)))
                    .or_default()
                    .push(name);
                continue;
            }
        };
        loaded += 1;
        let e = &mut host.engine;
        let n = quest.stages(e).len();
        // The stages the record names (INDX), against the ones loaded.
        let record = order.get(id).unwrap().record().unwrap();
        let indx: Vec<u8> = record
            .subrecords
            .iter()
            .filter(|s| &s.kind.0 == b"INDX")
            .filter_map(|s| s.data.first().copied())
            .collect();
        let loaded_stages = quest.stages(e);
        if loaded_stages != indx {
            stops
                .entry("stages differ from the record's INDX".to_string())
                .or_default()
                .push(format!("{name} {loaded_stages:?} vs {indx:?}"));
        }
        if let Some(&first) = loaded_stages.first() {
            let tried = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                e.call_log = Some(vec![]);
                let found = e.call(0x0060_d510, &[quest.form, u32::from(first)]).u32() & 0xff;
                let log = e.call_log.take().unwrap_or_default();
                let set_done = log.iter().filter(|(a, _)| *a == 0x0060_fb60).count();
                let current = e.call(0x0060_d700, &[quest.form]).u32() & 0xff;
                (found, set_done, current)
            }));
            e.call_log = None;
            match tried {
                Ok((1, d, c)) if c == u32::from(first) => {
                    set_ok.push(format!("{name} stage {first}: {d} item(s) set done"))
                }
                Ok(other) => stops
                    .entry(format!("SetStageDone answered {other:?}"))
                    .or_default()
                    .push(name.clone()),
                Err(p) => stops
                    .entry(format!("set stage: {}", stop_of(p)))
                    .or_default()
                    .push(name.clone()),
            }
        }
        if n > 0 {
            with_stages += 1;
            stages += n;
        }
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let before = quest.snapshot(e, &script);
            quest.run(e);
            (before, quest.snapshot(e, &script))
        }));
        match result {
            Ok((before, after)) => {
                ran += 1;
                if before != after {
                    changed.push(format!("{name}: {before:?} -> {after:?}"));
                }
            }
            Err(p) => stops
                .entry(format!("run: {}", stop_of(p)))
                .or_default()
                .push(name),
        }
    }
    std::panic::set_hook(hook);
    println!(
        "quests with a script: {quests}; loaded: {loaded} ({with_stages} with stages, {stages} \
         stages); ran to the end: {ran}"
    );
    let mut by_count: Vec<_> = stops.iter().collect();
    by_count.sort_by_key(|(_, v)| std::cmp::Reverse(v.len()));
    for (stop, names) in by_count {
        println!(
            "  stopped {}: {stop}  [{}]",
            names.len(),
            names.iter().take(4).cloned().collect::<Vec<_>>().join(", ")
        );
    }
    println!(
        "SetStageDone on the first stage ran through: {}",
        set_ok.len()
    );
    for s in set_ok.iter().take(15) {
        println!("  {s}");
    }
    println!("changed by one run: {}", changed.len());
    for c in changed.iter().take(20) {
        println!("  {}", &c[..c.len().min(200)]);
    }
}

//! Checks on the real ESMs. Skipped (passing) when `FNV_ESM_DIR` is not set.
//! One test, so the plugins are loaded once (they are large).

use nv_coverage::{data, dialogue, funcs, graph, quests, scan};

#[test]
fn measures_the_real_game_data() {
    let Some(d) = data::locate(None)
        .ok()
        .and_then(|dir| data::open(&dir).ok())
    else {
        return;
    };

    // Script functions.
    let sources = scan::sources(&d.order);
    assert!(sources.len() > 10_000);
    let rep = funcs::analyse(&d.order, &sources);
    assert!(rep.used_count() > 300);
    assert!(rep.used_handled() > 250);
    let set_stage = rep.stats.iter().find(|s| s.name == "SetStage").unwrap();
    assert!(set_stage.handled && set_stage.calls > 1000);
    let handled = funcs::handled_set();
    for s in rep.missing() {
        assert!(!handled.contains(&s.name.to_ascii_lowercase()));
    }

    // Quest walk.
    let lines = quests::walk(&d.order);
    let main = lines.get(&data::Line::Main).expect("main line");
    assert!(main.stages.iter().any(|s| s.editor == "VCG02"));
    assert!(
        main.clean() * 10 > main.stages.len() * 5,
        "most main stages should run"
    );
    assert!(main.targets > 100);

    // Dialogue and the stage graph.
    let dlg = dialogue::analyse(&d.order, &sources, &handled);
    assert!(dlg.infos > 20_000);
    assert!(dlg.condition_percent() > 90.0);
    let g = graph::build(&d.order, &sources, &rep);
    assert!(g.stages.contains_key("VCG02"));
    assert!(g.setters.keys().any(|(q, _)| q == "vcg02"));
}

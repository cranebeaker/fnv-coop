//! The engine interface (`coop_game`) on the `world` adapter (`world::coop::game::WorldCoop`).
//!
//! The conformance rows run on new games of the script test world (`testdata::quests`): each
//! row must pass, and each twin must fail for its own cause. The other tests check what
//! only this adapter has: the effect queue behind the export, the policy gate behind an
//! intent. Each has a mutation twin inside it.

use std::sync::Arc;

use coop_core::ids::{FormId as CForm, PlayerId};
use coop_core::schema::{flag, Field};
use coop_game::conformance::{check, Sample, ROWS};
use coop_game::{opening, Intent, Names, Players, Refused, Role, Setup, Shared, Sheet, Story};
use coop_policy::PolicyTable;
use esm::{ActivePlugins, FormId, LoadOrder};
use testdata::quest_ids::*;
use world::coop::apply::Ctx;
use world::coop::game::WorldCoop;
use world::coop::Authority;
use world::scripting::{GameState, ScriptCache};

fn order(tag: &str) -> (testdata::TempData, LoadOrder) {
    let data = testdata::quests(tag);
    let order = LoadOrder::from_data_dir(data.path(), &ActivePlugins::OfficialOnly).unwrap();
    (data, order)
}

/// A policy table with one row: the shared quest `TestQuest`, which the host advances.
fn table(row_extra: &str) -> Arc<PolicyTable> {
    let json = format!(
        r#"{{"schema":2,
"policy":{{"death":{{"mode":"downed","revive_s":60,"all_down":"reload-host-save"}},"late_join":{{"during_scene":"wait","after_scene":"warp-to-zone"}},"barrier_timeout_s":120,"tether_timeout_s":120,"follower_cap":{{"humanoid":1,"critter":1}},"party_cap":4}},
"shadow_classes":{{"skills":"per_player","perks":"per_player","followers":"per_player","quest_var":"per_player","karma":"shared","reputation":"shared"}},
"quests":[{{"edid":"TestQuest","id":"0x{QUEST:08X}","local":"0x{:08X}","plugin":"FalloutNV.esm","tier":"T1","line":"t","status":"confirmed","advance":"host"{row_extra}}}]}}"#,
        QUEST & 0x00FF_FFFF
    );
    Arc::new(PolicyTable::from_json(&json).expect("table"))
}

/// The script test world for the rows: `TestQuest`, whose stage 10 shows objective 10.
fn sample(policy: Option<Arc<PolicyTable>>) -> Sample {
    Sample {
        editor_id: "TestQuest".into(),
        no_editor_id: "NoSuchThing".into(),
        plugin: "FalloutNV.esm".into(),
        quest: CForm(QUEST),
        stage: 10,
        objective: 10,
        line: "set TestGlobal to 9".into(),
        policy,
    }
}

fn setup(role: Role, policy: Option<Arc<PolicyTable>>) -> Setup {
    Setup {
        role,
        me: match role {
            Role::Host => 0,
            Role::Guest => 1,
        },
        policy,
        keep_own_rows: false,
    }
}

fn stage(stage: u16) -> Intent {
    Intent::SetStage {
        quest: CForm(QUEST),
        stage,
    }
}

fn rows_and_twins(tag: &str, policy: Option<Arc<PolicyTable>>) {
    let (_d, order) = order(tag);
    let scripts = ScriptCache::default();
    let ctx = Ctx {
        order: &order,
        scripts: &scripts,
    };
    let sample = sample(policy);
    for row in ROWS {
        let mut game = GameState::new(&order);
        let mut twin = GameState::new(&order);
        let result = check(
            row,
            &mut WorldCoop::new(&mut game, ctx),
            &mut WorldCoop::new(&mut twin, ctx),
            &sample,
        );
        assert_eq!(result, Ok(()));
    }
}

#[test]
fn each_row_passes_and_each_twin_fails() {
    rows_and_twins("coop-game-rows", None);
}

#[test]
fn each_row_passes_and_each_twin_fails_with_a_policy() {
    rows_and_twins("coop-game-rows-policy", Some(table("")));
}

#[test]
fn configure_sets_the_authority_and_installs_the_policy() {
    let (_d, order) = order("coop-game-configure");
    let scripts = ScriptCache::default();
    let ctx = Ctx {
        order: &order,
        scripts: &scripts,
    };
    let mut host = GameState::new(&order);
    WorldCoop::new(&mut host, ctx)
        .configure(&setup(Role::Host, Some(table(""))))
        .unwrap();
    assert_eq!(host.authority(), Authority::Host);
    assert!(host.coop.gate.is_some() && host.coop.bank.enabled());
    assert_eq!(host.active_slot(), 0);

    let mut guest = GameState::new(&order);
    let mut keep = setup(Role::Guest, Some(table("")));
    keep.keep_own_rows = true;
    WorldCoop::new(&mut guest, ctx).configure(&keep).unwrap();
    assert_eq!(guest.authority(), Authority::Guest);
    assert!(guest.coop.gate.is_none() && guest.coop.bank.enabled());
    assert_eq!(guest.active_slot(), 1);
    assert!(guest.coop.bank.hold_own);
    // a later setup with no policy keeps the plan, and it changes the flag
    WorldCoop::new(&mut guest, ctx)
        .configure(&setup(Role::Guest, None))
        .unwrap();
    assert!(guest.coop.bank.enabled() && !guest.coop.bank.hold_own);

    // the twin: no policy in the setup, no gate and no plan in the game
    let mut plain = GameState::new(&order);
    WorldCoop::new(&mut plain, ctx)
        .configure(&setup(Role::Host, None))
        .unwrap();
    assert_eq!(plain.authority(), Authority::Host);
    assert!(plain.coop.gate.is_none() && !plain.coop.bank.enabled());
}

#[test]
fn an_idle_host_gives_no_list_until_a_write_comes() {
    let (_d, order) = order("coop-game-idle");
    let scripts = ScriptCache::default();
    let ctx = Ctx {
        order: &order,
        scripts: &scripts,
    };
    let mut host = GameState::new(&order);
    let mut game = WorldCoop::new(&mut host, ctx);
    game.configure(&setup(Role::Host, None)).unwrap();
    let first = game.export_shared(true).expect("a forced export");
    assert_eq!(game.export_shared(false), None, "nothing changed");
    // the game's own story code writes: the list comes, once
    let before = game.activity();
    assert_eq!(game.run_intent(0, &stage(10)), Ok(true));
    assert!(game.activity() > before);
    let second = game.export_shared(false).expect("the stage changed");
    assert_ne!(second, first);
    assert!(second.contains(&(CForm(QUEST), Field::QuestStage { stage: 10 })));
    assert_eq!(game.export_shared(false), None);
    // a plain write through the adapter shows too
    let topic = Field::Topic { known: true };
    let change = (topic.key(CForm(TOPIC_SECRET)), Some(topic.clone()));
    game.apply_shared(&[change], false);
    let third = game.export_shared(false).expect("the topic changed");
    assert!(third.contains(&(CForm(TOPIC_SECRET), topic)));
    assert_eq!(game.export_shared(false), None);
    assert_eq!(game.export_shared(true), Some(third));

    // the twin: a guest records no effect, so it is never sure
    let mut guest = GameState::new(&order);
    let mut game = WorldCoop::new(&mut guest, ctx);
    game.configure(&setup(Role::Guest, None)).unwrap();
    let list = game.export_shared(true);
    assert!(list.is_some());
    assert_eq!(game.export_shared(false), list);
    assert_eq!(game.export_shared(false), list);
}

#[test]
fn a_host_answers_each_kind_of_intent() {
    let (_d, order) = order("coop-game-intents");
    let scripts = ScriptCache::default();
    let ctx = Ctx {
        order: &order,
        scripts: &scripts,
    };
    let mut host = GameState::new(&order);
    let mut game = WorldCoop::new(&mut host, ctx);
    game.configure(&setup(Role::Host, None)).unwrap();
    // a global is not a quest
    let not_a_quest = Intent::SetStage {
        quest: CForm(GLOBAL),
        stage: 10,
    };
    assert_eq!(game.run_intent(0, &not_a_quest), Err(Refused::NoSuchForm));
    assert_eq!(game.quest(CForm(QUEST)), Some((0, flag::RUNNING)));
    assert_eq!(game.run_intent(0, &stage(10)), Ok(true));
    // the stage's script shows an objective, and that announces the quest
    let started = flag::RUNNING | flag::ANNOUNCED;
    assert_eq!(game.quest(CForm(QUEST)), Some((10, started)));
    // the game's rule: a done stage does not run again
    assert_eq!(game.run_intent(0, &stage(10)), Ok(false));
    let line = Intent::ScriptLine("set TestGlobal to 9".into());
    assert_eq!(game.run_intent(0, &line), Ok(true));
    // the script parser refuses a bracket with no end
    let not_a_line = Intent::ScriptLine("set TestGlobal to (1".into());
    assert_eq!(game.run_intent(0, &not_a_line), Ok(false));
    // the stage's script ran (the objective shows), and the line ran
    assert_eq!(host.objectives.get(&(FormId(QUEST), 10)), Some(&false));
    assert_eq!(host.globals.get(&FormId(GLOBAL)), Some(&9.0));

    // the twin: a game for one player is no host, so nothing runs
    let mut single = GameState::new(&order);
    let mut game = WorldCoop::new(&mut single, ctx);
    assert_eq!(game.run_intent(0, &stage(10)), Err(Refused::NotHost));
    assert_eq!(game.run_intent(0, &line), Err(Refused::NotHost));
    assert_eq!(game.quest(CForm(QUEST)), Some((0, flag::RUNNING)));
    assert!(single.objectives.is_empty());
    assert_eq!(single.globals.get(&FormId(GLOBAL)), Some(&5.0));
    assert_eq!(single.coop.stats.suppressed_runs, 0);
}

#[test]
fn a_stage_that_the_policy_holds_gives_held() {
    let (_d, order) = order("coop-game-held");
    let scripts = ScriptCache::default();
    let ctx = Ctx {
        order: &order,
        scripts: &scripts,
    };
    // `barrier` true: stage 10 waits until each present player passed the barrier
    let answer = |barrier: bool| {
        let extra = if barrier {
            r#","barrier":{"stages":[10]}"#
        } else {
            ""
        };
        let mut host = GameState::new(&order);
        let mut game = WorldCoop::new(&mut host, ctx);
        game.configure(&setup(Role::Host, Some(table(extra))))
            .unwrap();
        // The interface has no call for the present players yet: the gate gets them here.
        let gate = host.coop.gate.as_mut().unwrap();
        gate.set_present((0..=2).map(PlayerId));
        let answer = WorldCoop::new(&mut host, ctx).run_intent(0, &stage(10));
        (answer, host.stages.get(&FormId(QUEST)).copied())
    };
    assert_eq!(answer(true), (Err(Refused::Held), None));
    // the twin: the same steps with no barrier
    assert_eq!(answer(false), (Ok(true), Some(10)));
}

#[test]
fn names_come_from_the_load_order() {
    let (_d, order) = order("coop-game-names");
    let scripts = ScriptCache::default();
    let ctx = Ctx {
        order: &order,
        scripts: &scripts,
    };
    let mut gs = GameState::new(&order);
    let game = WorldCoop::new(&mut gs, ctx);
    assert_eq!(game.form_of("testglobal"), Some(CForm(GLOBAL)));
    assert_eq!(game.name_of(CForm(GLOBAL)).as_deref(), Some("TestGlobal"));
    assert_eq!(game.load_index("falloutnv.ESM"), Some(0));
    // the twin: a form and a plugin that the load order does not have
    assert_eq!(game.name_of(CForm(0x00FF_FFF0)), None);
    assert_eq!(game.load_index("DeadMoney.esm"), None);
}

#[test]
fn a_view_with_no_game_data_carries_the_state_and_knows_no_name() {
    let (_d, order) = order("coop-game-no-data");
    let mut host = GameState::new(&order);
    let mut game = WorldCoop::over(&mut host, None);
    game.configure(&setup(Role::Host, None)).unwrap();
    assert_eq!(game.form_of("TestQuest"), None);
    assert_eq!(game.name_of(CForm(QUEST)), None);
    assert_eq!(game.load_index("FalloutNV.esm"), None);
    // the role and the shared state work: a plain write shows in the export
    let field = Field::QuestStage { stage: 10 };
    let change = (field.key(CForm(QUEST)), Some(field.clone()));
    let applied = game.apply_shared(&[change], true);
    assert_eq!((applied.set, applied.cleared), (1, 0));
    let list = game.export_shared(false).expect("a write came");
    assert!(list.contains(&(CForm(QUEST), field)));
    assert_eq!(game.stage(CForm(QUEST)), Some(10));
    // an intent needs a script of the game data
    let line = Intent::ScriptLine("set TestGlobal to 9".into());
    assert_eq!(game.run_intent(0, &stage(20)), Err(Refused::NoSuchForm));
    assert_eq!(game.run_intent(0, &line), Err(Refused::NoSuchForm));
    assert_eq!(host.authority(), Authority::Host);
    assert_eq!(host.stages.get(&FormId(QUEST)), Some(&10));
    assert_eq!(host.globals.get(&FormId(GLOBAL)), Some(&5.0));

    // the twin: a view of a new game with its data knows the name and runs the intents
    let scripts = ScriptCache::default();
    let ctx = Ctx {
        order: &order,
        scripts: &scripts,
    };
    let mut host = GameState::new(&order);
    let mut game = WorldCoop::over(&mut host, Some(ctx));
    game.configure(&setup(Role::Host, None)).unwrap();
    assert_eq!(game.form_of("TestQuest"), Some(CForm(QUEST)));
    assert_eq!(game.run_intent(0, &stage(10)), Ok(true));
    assert_eq!(game.run_intent(0, &line), Ok(true));
    assert_eq!(host.stages.get(&FormId(QUEST)), Some(&10));
    assert_eq!(host.globals.get(&FormId(GLOBAL)), Some(&9.0));
}

fn sheet(name: &str) -> Sheet {
    Sheet {
        name: name.into(),
        female: Some(true),
        special: [3, 4, 5, 6, 7, 8, 9],
        tags: vec![32, 40, 41],
        perks: vec![CForm(0x1234)],
    }
}

#[test]
fn the_local_player_is_the_live_player_rows() {
    let (_d, order) = order("coop-game-local");
    let mut gs = GameState::new(&order);
    // a new game: no place, and the attributes that no menu changed
    let new = WorldCoop::over(&mut gs, None).local_player();
    assert_eq!(new.place, coop_game::Place::default());
    assert_eq!(new.sheet.special, [5; 7]);
    assert_eq!((new.sheet.name.as_str(), new.sheet.female), ("", None));
    // outdoors the game can know the worldspace and not the cell
    gs.player_world = Some(FormId(0x000D_A726));
    gs.player_position = Some([1.0, 2.0, 3.0]);
    gs.player_heading = 0.5;
    gs.player_name = Some("Al".into());
    gs.actor_values
        .insert((world::dialogue::PLAYER_REF, 5), 11.6);
    gs.actor_values
        .insert((world::dialogue::PLAYER_REF, 11), 0.2);
    gs.tag_skills = [41u16, 32].into_iter().collect();
    gs.perks.insert(FormId(0x0200));
    gs.perks.insert(FormId(0x0100));
    let local = WorldCoop::over(&mut gs, None).local_player();
    assert_eq!(local.place.cell, None);
    assert_eq!(local.place.world, Some(CForm(0x000D_A726)));
    assert_eq!(local.place.pos, Some([1.0, 2.0, 3.0]));
    assert_eq!(local.place.heading, 0.5);
    assert_eq!(local.sheet.name, "Al");
    // an attribute stays in 1 to 10
    assert_eq!(local.sheet.special, [10, 5, 5, 5, 5, 5, 1]);
    assert_eq!(local.sheet.tags, [32, 41]);
    assert_eq!(local.sheet.perks, [CForm(0x0100), CForm(0x0200)]);
    // the read function for a caller with `&GameState` gives the same answer
    assert_eq!(world::coop::game::local_player_of(&gs), local);
    // the twin: indoors there is a cell and no worldspace
    gs.player_world = None;
    gs.player_cell = Some(FormId(0x0010_4C0F));
    let indoors = WorldCoop::over(&mut gs, None).local_player().place;
    assert_eq!(
        (indoors.cell, indoors.world),
        (Some(CForm(0x0010_4C0F)), None)
    );
}

#[test]
fn a_character_goes_to_the_rows_of_its_player() {
    let (_d, order) = order("coop-game-character");
    let mut host = GameState::new(&order);
    // a view with no game data takes a character: it needs no script
    let mut game = WorldCoop::over(&mut host, None);
    game.configure(&setup(Role::Host, Some(table("")))).unwrap();
    assert!(game.add_player(1));
    assert!(game.add_player(1), "a second add gives the same answer");
    let before = game.local_player();
    assert_eq!(
        game.run_intent(1, &Intent::Character(sheet("Bo"))),
        Ok(true)
    );
    // the host's own player did not change
    assert_eq!(game.local_player(), before);
    let rows = host.shadow_of(1).expect("rows of player 1");
    assert_eq!(rows.name.as_deref(), Some("Bo"));
    assert_eq!(rows.female, Some(true));
    assert_eq!(rows.actor_values.get(&11), Some(&9.0));
    assert!(rows.perks.contains(&FormId(0x1234)));
    assert_eq!(rows.tag_skills, [32u16, 40, 41].into_iter().collect());

    // the twin: the same character for the host's own player changes the local player
    let mut game = WorldCoop::over(&mut host, None);
    assert_eq!(
        game.run_intent(0, &Intent::Character(sheet("Al"))),
        Ok(true)
    );
    assert_eq!(game.local_player().sheet, sheet("Al"));
    // the opening is done for that player, and a later stage does not go down
    assert_eq!(game.stage(opening::VCG00), Some(opening::VCG00_DONE));
    assert_eq!(
        game.stage(opening::VCG01),
        Some(opening::OPENING_OVER_STAGE)
    );
    host.stages.insert(FormId(opening::VCG01.0), 150);
    let mut game = WorldCoop::over(&mut host, None);
    assert_eq!(
        game.run_intent(0, &Intent::Character(sheet("Al"))),
        Ok(true)
    );
    assert_eq!(game.stage(opening::VCG01), Some(150));
}

#[test]
fn a_game_with_no_policy_keeps_one_shared_player() {
    let (_d, order) = order("coop-game-one-player");
    let mut host = GameState::new(&order);
    let mut game = WorldCoop::over(&mut host, None);
    game.configure(&setup(Role::Host, None)).unwrap();
    assert!(!game.add_player(1), "no rows for a player");
    assert!(host.shadow_of(1).is_none());
    // the twin: with a policy the game keeps rows for each player
    let mut host = GameState::new(&order);
    let mut game = WorldCoop::over(&mut host, None);
    game.configure(&setup(Role::Host, Some(table("")))).unwrap();
    assert!(game.add_player(1));
    assert!(host.shadow_of(1).is_some());
    // a guest's game and a game for one player take no character
    let mut guest = GameState::new(&order);
    let mut game = WorldCoop::over(&mut guest, None);
    let bo = Intent::Character(sheet("Bo"));
    assert_eq!(game.run_intent(0, &bo), Err(Refused::NotHost));
    game.configure(&setup(Role::Guest, None)).unwrap();
    assert_eq!(game.run_intent(1, &bo), Err(Refused::NotHost));
    assert_eq!(guest.player_name, None);
}

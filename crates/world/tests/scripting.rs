//! Running the game's scripts: a greeting's result scripts set a quest
//! stage, the stage's log entry shows an objective and a message, the
//! quest's own script advances it on a timer, and the next stage completes
//! the quest, sets a global, enables a door and gives the player caps.

use esm::{ActivePlugins, FormId, LoadOrder};
use testdata::quest_ids::*;
use world::dialogue::{self, Speaker, PLAYER_REF};
use world::scripting::{Event, GameState, Runner, ScriptCache};

fn order(tag: &str) -> (testdata::TempData, LoadOrder) {
    let data = testdata::quests(tag);
    let order = LoadOrder::from_data_dir(data.path(), &ActivePlugins::OfficialOnly).unwrap();
    (data, order)
}

#[test]
fn a_new_game_runs_start_game_quests_with_globals_at_their_values() {
    let (_data, order) = order("scripting-new");
    let state = GameState::new(&order);
    assert!(state.running.contains(&FormId(QUEST)));
    assert_eq!(state.globals.get(&FormId(GLOBAL)), Some(&5.0));
    assert!(state.stages.is_empty());
    // The door starts disabled.
    assert!(!world::enabled_now(
        &order,
        FormId(DOOR_REF),
        &state.disabled
    ));
}

#[test]
fn a_greeting_and_the_quest_script_carry_the_quest_through() {
    let (_data, order) = order("scripting-quest");
    let scripts = ScriptCache::default();
    let mut state = GameState::new(&order);

    // The doctor's greeting, and its result scripts.
    let doc = Speaker::load(&order, FormId(DOC_REF), FormId(DOC)).unwrap();
    let line = dialogue::pick(&order, FormId(GREETING), &doc, &state).unwrap();
    assert_eq!(line.form_id, FormId(GREETING_LINE));
    // Only while its quest runs.
    state.running.remove(&FormId(QUEST));
    assert!(dialogue::pick(&order, FormId(GREETING), &doc, &state).is_none());
    state.running.insert(FormId(QUEST));
    // And only for whoever the quest's own conditions allow (the doctor).
    let player = Speaker::load(&order, PLAYER_REF, FormId(PLAYER)).unwrap();
    assert!(dialogue::pick(&order, FormId(GREETING), &player, &state).is_none());
    // It's said once only.
    state.said.insert(line.form_id);
    assert!(dialogue::pick(&order, FormId(GREETING), &doc, &state).is_none());
    let mut runner = Runner::new(&order, &scripts, &mut state);
    runner.run_source(
        line.begin_script.as_deref().unwrap(),
        Some(FormId(DOC_REF)),
        Some(FormId(DOC_REF)),
    );
    runner.run_source(
        line.end_script.as_deref().unwrap(),
        Some(FormId(DOC_REF)),
        Some(FormId(DOC_REF)),
    );
    assert_eq!(state.stages.get(&FormId(QUEST)), Some(&10));
    assert_eq!(state.objectives.get(&(FormId(QUEST), 10)), Some(&false));
    assert_eq!(state.globals.get(&FormId(GLOBAL)), Some(&6.0));
    let events: Vec<Event> = std::mem::take(&mut state.events);
    assert_eq!(
        events,
        vec![
            Event::Journal {
                quest: FormId(QUEST),
                text: "Talked to the doctor.".into()
            },
            Event::Objective {
                quest: FormId(QUEST),
                text: "Talk to the doctor.".into(),
                completed: false
            },
            // Its first objective shown: "Quest added" (`005ec5d0`).
            Event::QuestText(world::quest_text::QuestText::Quest {
                quest: FormId(QUEST),
                update: world::quest_text::Update::Added
            }),
            Event::Message {
                title: Some("Note".into()),
                text: "Hello there.".into(),
                buttons: vec![],
                icon: None,
            },
        ]
    );

    // The quest's script (every second) starts a 3-second timer, then
    // sets stage 20.
    let mut runner = Runner::new(&order, &scripts, &mut state);
    for _ in 0..3 {
        runner.update(1.0);
    }
    assert_eq!(runner.state.stages.get(&FormId(QUEST)), Some(&10));
    runner.update(1.0);
    assert_eq!(state.stages.get(&FormId(QUEST)), Some(&20));
    // The second log entry was used (the first's condition fails).
    assert!(state.completed.contains(&FormId(QUEST)));
    assert_eq!(state.objectives.get(&(FormId(QUEST), 10)), Some(&true));
    assert_eq!(state.globals.get(&FormId(GLOBAL)), Some(&7.0));
    // 3 to start with, and 25.
    assert_eq!(state.item_count(&order, PLAYER_REF, FormId(CAPS)), 28);
    assert!(world::enabled_now(
        &order,
        FormId(DOOR_REF),
        &state.disabled
    ));
    assert!(state
        .events
        .contains(&Event::Enable(FormId(DOOR_REF), true)));
    assert!(state.events.contains(&Event::Journal {
        quest: FormId(QUEST),
        text: "Done.".into()
    }));
    // Setting a stage that's done again does nothing.
    let mut runner = Runner::new(&order, &scripts, &mut state);
    assert!(!runner.set_stage(FormId(QUEST), 20));
    assert!(!runner.set_stage(FormId(QUEST), 15));
}

#[test]
fn the_menu_lists_top_level_and_learned_topics_the_speaker_answers() {
    let (_data, order) = order("scripting-topics");
    let mut state = GameState::new(&order);
    let top = dialogue::top_level_topics(&order);
    // Highest priority first; the secret isn't top-level.
    let ids: Vec<u32> = top.iter().map(|t| t.form_id.0).collect();
    assert_eq!(ids, [TOPIC_NOBODY, TOPIC_TOWN, TOPIC_ABOUT]);
    let doc = Speaker::load(&order, FormId(DOC_REF), FormId(DOC)).unwrap();
    let ids = |choices: Vec<dialogue::Choice>| -> Vec<u32> {
        choices.into_iter().map(|c| c.topic.form_id.0).collect()
    };
    let menu = |state: &GameState| -> Vec<(u32, String)> {
        dialogue::menu_topics(&order, &top, &doc, state)
            .into_iter()
            .map(|c| (c.topic.form_id.0, c.label))
            .collect()
    };
    // Nobody here answers the first; the town's line has its own prompt.
    assert_eq!(
        menu(&state),
        [
            (TOPIC_TOWN, "Where am I?".to_string()),
            (TOPIC_ABOUT, "Tell me about yourself.".to_string()),
        ]
    );
    // Asking about him teaches the secret, which is offered from then on.
    let about = dialogue::pick(&order, FormId(TOPIC_ABOUT), &doc, &state).unwrap();
    dialogue::line_begins(&mut state, &about, FormId(DOC_REF));
    assert!(state.said.contains(&FormId(ABOUT_LINE)));
    assert!(state.talked_to.contains(&FormId(DOC_REF)));
    let learned: Vec<u32> = menu(&state).into_iter().map(|(t, _)| t).collect();
    assert_eq!(learned, [TOPIC_TOWN, TOPIC_ABOUT, TOPIC_SECRET]);
    // A line with follow-ups offers only those (that he answers).
    let next = dialogue::next_choices(&order, &about, &top, &doc, &state);
    assert_eq!(ids(next), [TOPIC_TOWN]);
    // One without goes back to the main list.
    let town = dialogue::pick(&order, FormId(TOPIC_TOWN), &doc, &state).unwrap();
    let back = dialogue::next_choices(&order, &town, &top, &doc, &state);
    assert_eq!(ids(back), [TOPIC_TOWN, TOPIC_ABOUT, TOPIC_SECRET]);
    // Nothing while the quest the lines belong to isn't running.
    state.running.remove(&FormId(QUEST));
    assert!(menu(&state).is_empty());
}

#[test]
fn a_topic_with_no_lines_says_the_lines_connected_to_it() {
    let (_data, order) = order("scripting-linked-topic");
    let state = GameState::new(&order);
    let doc = Speaker::load(&order, FormId(DOC_REF), FormId(DOC)).unwrap();
    // The topic has no lines of its own: its connection (`INFC`) names the
    // secret's, which stands under the secret topic.
    let own = dialogue::topic_lines(&order, FormId(TOPIC_LINKED));
    assert_eq!(own.len(), 1);
    assert_eq!(own[0].form_id, FormId(SECRET_LINE));
    assert_eq!(own[0].topic, Some(FormId(TOPIC_SECRET)));
    let said = dialogue::pick(&order, FormId(TOPIC_LINKED), &doc, &state).unwrap();
    assert_eq!(said.form_id, FormId(SECRET_LINE));
    // The secret topic still has its line, once.
    assert_eq!(dialogue::topic_lines(&order, FormId(TOPIC_SECRET)).len(), 1);
}

#[test]
fn waiting_moves_the_clock_unless_enemies_are_near() {
    let (_data, order) = order("scripting-wait");
    let mut state = GameState::new(&order);
    assert_eq!(state.global(&order, "GameHour"), Some(10.0));
    state.wait(&order, 3.0).unwrap();
    assert!((state.global(&order, "GameHour").unwrap() - 13.0).abs() < 1e-4);
    // Past midnight the day rolls over.
    state.wait(&order, 12.0).unwrap();
    assert!((state.global(&order, "GameHour").unwrap() - 1.0).abs() < 1e-4);
    // Waiting heals Heal Rate × hours ÷ TimeScale: nothing at Endurance 5,
    // 10 ÷ 30 an hour at 9 (`fAVDHealRateEndurance9Bonus`).
    state.damage.insert(PLAYER_REF, 10.0);
    state.wait(&order, 3.0).unwrap();
    assert_eq!(state.damage[&PLAYER_REF], 10.0);
    state.actor_values.insert((PLAYER_REF, 7), 9.0);
    state.wait(&order, 3.0).unwrap();
    assert!((state.damage[&PLAYER_REF] - 9.0).abs() < 1e-6);
    // Someone fighting the player: no waiting.
    state.combat.insert(FormId(GECKO_REF), PLAYER_REF);
    let hour = state.global(&order, "GameHour").unwrap();
    assert!(state.wait(&order, 1.0).is_err());
    assert!((state.global(&order, "GameHour").unwrap() - hour).abs() < 1e-4);
}

#[test]
fn the_killing_blow_is_kept_for_how_the_body_falls() {
    let (_data, order) = order("scripting-blow");
    let scripts = ScriptCache::default();
    let mut state = GameState::new(&order);
    let before = state.globals[&FormId(GLOBAL)];
    Runner::new(&order, &scripts, &mut state).run_source("GeckoRef.KillActor player", None, None);
    assert!(state.dead.contains(&FormId(GECKO_REF)));
    // Its `OnDeath` ran (VCG02's geckos count their deaths there).
    assert_eq!(state.globals[&FormId(GLOBAL)], before + 100.0);
    // Killing the dead again does nothing more.
    Runner::new(&order, &scripts, &mut state).run_source("GeckoRef.KillActor player", None, None);
    assert_eq!(state.globals[&FormId(GLOBAL)], before + 100.0);
    let (by, damage) = state.last_blow[&FormId(GECKO_REF)];
    assert_eq!(by, PLAYER_REF);
    // `KillActor` deals no hit: the body goes limp with the death
    // routine's nudge only, the attacker named.
    assert_eq!(
        state.deaths[&FormId(GECKO_REF)],
        world::combat::DeathStart {
            killer: Some(PLAYER_REF),
            hit: false
        }
    );
    // Without a weapon, `fDeathForce…`: 20 to 60 Havok units a second as
    // the damage goes from 0.1 to 40, at most past it (shots: 20 to 40 over
    // 0.1 to 10).
    use world::combat::death_push;
    let h = nif::collision::HAVOK_SCALE;
    let close = |a: f32, b: f32| (a - b).abs() < 1e-3;
    assert!(close(
        death_push(&order, None, damage, false, 0.0),
        60.0 * h
    ));
    assert!(close(death_push(&order, None, 0.1, false, 0.0), 20.0 * h));
    assert!(close(death_push(&order, None, 20.05, false, 0.0), 40.0 * h));
    assert!(close(death_push(&order, None, 5.05, true, 0.0), 30.0 * h));
    // With one: its kill impulse × 2.5, a tenth past its impulse distance.
    let mut pistol = world::combat::Weapon::load(&order, FormId(PISTOL)).unwrap();
    pistol.kill_impulse = 4.0;
    pistol.impulse_distance = 1000.0;
    assert!(close(
        death_push(&order, Some(&pistol), 1.0, true, 500.0),
        10.0 * h
    ));
    assert!(close(
        death_push(&order, Some(&pistol), 1.0, true, 1500.0),
        h
    ));
    // The bodies' shares by part: the body all, a hand half, a foot a
    // quarter.
    assert_eq!(world::combat::death_push_share(2), 1.0);
    assert_eq!(world::combat::death_push_share(7), 0.5);
    assert_eq!(world::combat::death_push_share(16), 0.25);
}

#[test]
fn a_death_nudges_the_body_along_its_way() {
    let (_data, order) = order("scripting-nudge");
    let scripts = ScriptCache::default();
    let mut state = GameState::new(&order);
    // `KillActor` with nobody named: no attacker (`005be2a0`), no hit.
    Runner::new(&order, &scripts, &mut state).run_source("GeckoRef.KillActor", None, None);
    assert_eq!(
        state.deaths[&FormId(GECKO_REF)],
        world::combat::DeathStart {
            killer: None,
            hit: false
        }
    );
    // `0089d900`: a unit direction × `fDeathForceForceMin` (the exe's 35
    // here; the fixture doesn't set it), game units a second.
    use world::combat::death_nudge;
    let close = |a: [f32; 3], b: [f32; 3]| (0..3).all(|i| (a[i] - b[i]).abs() < 1e-3);
    // Standing still: the way it faces (heading clockwise from north).
    let east = std::f32::consts::FRAC_PI_2;
    assert!(close(
        death_nudge(&order, [0.0; 3], east, None),
        [35.0, 0.0, 0.0]
    ));
    assert!(close(
        death_nudge(&order, [0.0; 3], 0.0, None),
        [0.0, 35.0, 0.0]
    ));
    // Moving: along the controller's velocity, whatever its speed.
    assert!(close(
        death_nudge(&order, [3.0, -4.0, 0.0], east, None),
        [21.0, -28.0, 0.0]
    ));
    // Killed by the player: away from the player, moving or not.
    assert!(close(
        death_nudge(
            &order,
            [3.0, -4.0, 0.0],
            east,
            Some(([0.0, 100.0, 0.0], [0.0, 0.0, 0.0]))
        ),
        [0.0, 35.0, 0.0]
    ));
}

#[test]
fn falls_hurt_past_six_hundred_units() {
    let (_data, order) = order("scripting-fall");
    // `fJumpFallHeightMult` 0.025 × (fall − 600) ^ 1.65 (the exe's
    // defaults here).
    assert_eq!(world::combat::fall_damage(&order, 600.0), 0.0);
    let d = world::combat::fall_damage(&order, 700.0);
    assert!((d - 49.9).abs() < 0.1, "{d}");
    let mut state = GameState::new(&order);
    let full = world::combat::health(&order, &state, PLAYER_REF).unwrap();
    world::combat::land(&order, &mut state, PLAYER_REF, 1000.0);
    let now = world::combat::health(&order, &state, PLAYER_REF).unwrap();
    assert!((full - now - 491.3).abs() < 0.5, "{full} → {now}");
}

#[test]
fn skill_checks_show_what_they_need_and_pick_the_outcome() {
    let (_data, order) = order("scripting-check");
    let scripts = ScriptCache::default();
    let mut state = GameState::new(&order);
    let doc = Speaker::load(&order, FormId(DOC_REF), FormId(DOC)).unwrap();
    let top = dialogue::top_level_topics(&order);
    // A line offering the check as its follow-up.
    let mut about = dialogue::pick(&order, FormId(TOPIC_ABOUT), &doc, &state).unwrap();
    about.choices = vec![FormId(TOPIC_CHECK)];
    let offered = |state: &mut GameState, speech: u32| {
        Runner::new(&order, &scripts, state).run_source(
            &format!("player.setav speech {speech}"),
            None,
            None,
        );
        let choices = dialogue::next_choices(&order, &about, &top, &doc, state);
        assert_eq!(choices.len(), 1);
        (choices[0].info.form_id.0, choices[0].label.clone())
    };
    // Short of it: the failing line, with the player's value and the need
    // (from the first `GetActorValue` condition, named by `KNAM`; two
    // spaces before the prompt).
    assert_eq!(
        offered(&mut state, 12),
        (CHECK_FAILED, "[Speech 12/25]  Please?".to_string())
    );
    assert_eq!(
        offered(&mut state, 25),
        (CHECK_PASSED, "[Speech 25]  Trust me.".to_string())
    );
}

#[test]
fn a_reference_keeps_its_own_script_variables() {
    let (_data, order) = order("scripting-activate");
    let scripts = ScriptCache::default();
    let mut state = GameState::new(&order);
    let mut runner = Runner::new(&order, &scripts, &mut state);
    // Activated by the player twice: only the block for the player runs.
    for _ in 0..2 {
        runner.run_event(FormId(DOC_REF), "onactivate", PLAYER_REF);
    }
    assert_eq!(
        runner
            .state
            .variables
            .get(&FormId(DOC_REF))
            .unwrap()
            .get("iByDoor"),
        Some(0.0)
    );
    assert_eq!(runner.state.action_ref, None);
    assert_eq!(
        state
            .variables
            .get(&FormId(DOC_REF))
            .unwrap()
            .get("iTalked"),
        Some(2.0)
    );
    // Other scripts read it by the reference's editor ID; the doctor's
    // values come from his record.
    let mut runner = Runner::new(&order, &scripts, &mut state);
    runner.run_source(
        "set TestGlobal to DocRef.iTalked * 10 + DocRef.GetActorValue Luck",
        None,
        None,
    );
    assert_eq!(state.globals.get(&FormId(GLOBAL)), Some(&23.0));
    assert!(state.unhandled.is_empty());
}

#[test]
fn cells_load_with_what_scripts_enabled_and_disabled() {
    let (_data, order) = order("scripting-load");
    let has = |list: &[world::Placement], id: u32| list.iter().any(|p| p.form_id == FormId(id));
    let start = world::load_cell(&order, FormId(CELL)).unwrap();
    assert!(!has(&start.objects, DOOR_REF));
    assert!(has(&start.actors, DOC_REF));
    let mut disabled = world::Disabled::new();
    disabled.insert(FormId(DOOR_REF), false);
    disabled.insert(FormId(DOC_REF), true);
    let now = world::load_cell_now(&order, FormId(CELL), &disabled).unwrap();
    assert!(has(&now.objects, DOOR_REF));
    assert!(!has(&now.actors, DOC_REF));
}

#[test]
fn finds_a_cells_scripted_objects_and_aims_at_them() {
    let (_data, order) = order("scripting-cell");
    let refs = world::scripting::interactive_references(&order, FormId(CELL));
    // The doctor, the gecko and the bottle (with scripts), the chest, the
    // strongbox, the caps, the chair and the terminal; not the doors.
    assert_eq!(refs.len(), 8);
    assert!(refs
        .iter()
        .any(|r| r.reference == FormId(CHEST_REF) && r.is_container()));
    assert!(refs
        .iter()
        .any(|r| r.reference == FormId(CAPS_REF) && r.is_item() && r.count == 5));
    let mut doc = refs
        .iter()
        .find(|r| r.reference == FormId(DOC_REF))
        .unwrap()
        .clone();
    assert_eq!(doc.script, Some(FormId(DOC_SCRIPT)));
    assert_eq!(doc.name.as_deref(), Some("Doc"));
    assert!(doc.trigger.is_none());

    // A box 20 x 10 x 5 around it, turned a quarter (clockwise, seen from
    // above): its long side now runs north-south.
    doc.position = [100.0, 0.0, 0.0];
    doc.rotation = [0.0, 0.0, std::f32::consts::FRAC_PI_2];
    doc.bounds = Some(([-20.0, -10.0, -5.0], [20.0, 10.0, 5.0]));
    // From the south, looking north: meets the box's near face 20 units
    // from its middle.
    let hit = doc.ray_hit([100.0, -100.0, 0.0], [0.0, 1.0, 0.0]).unwrap();
    assert!((hit - 80.0).abs() < 1e-3, "{hit}");
    // From the west, looking east: 10 units from the middle.
    let hit = doc.ray_hit([0.0, 0.0, 0.0], [1.0, 0.0, 0.0]).unwrap();
    assert!((hit - 90.0).abs() < 1e-3, "{hit}");
    assert!(doc.ray_hit([0.0, 50.0, 0.0], [1.0, 0.0, 0.0]).is_none());

    // As a trigger with those half sizes.
    doc.trigger = Some(world::scripting::TriggerBox {
        half: [20.0, 10.0, 5.0],
        shape: 1,
    });
    assert!(doc.contains([100.0, 15.0, 0.0]));
    assert!(!doc.contains([115.0, 0.0, 0.0]));
}

#[test]
fn inventories_start_from_records_and_follow_what_happens() {
    let (_data, order) = order("scripting-items");
    let scripts = ScriptCache::default();
    let mut state = GameState::new(&order);
    let caps = FormId(CAPS);
    // From the records: the player's 3, the chest's 10.
    assert_eq!(state.item_count(&order, PLAYER_REF, caps), 3);
    assert_eq!(state.item_count(&order, FormId(CHEST_REF), caps), 10);
    // A script reads and changes them.
    Runner::new(&order, &scripts, &mut state).run_source(
        "if ChestRef.GetItemCount Caps001 == 10\n\tChestRef.RemoveItem Caps001 4\nendif",
        None,
        None,
    );
    assert_eq!(state.item_count(&order, FormId(CHEST_REF), caps), 6);
    // Taking all (with the 2 cups the leveled list gives at level 1, not
    // the level-5 caps), and picking up the 5 on the floor.
    let moved = state.take_all(&order, FormId(CHEST_REF), PLAYER_REF);
    assert_eq!(moved, vec![(caps, 6), (FormId(CUP), 2)]);
    assert!(state.inventory(&order, FormId(CHEST_REF)).is_empty());
    state.pick_up(&order, FormId(CAPS_REF), caps, 5);
    assert_eq!(
        state.inventory(&order, PLAYER_REF),
        vec![(caps, 14), (FormId(CUP), 2)]
    );
    assert!(!world::enabled_now(
        &order,
        FormId(CAPS_REF),
        &state.disabled
    ));
    // Items' value and weight from their records; what the player carries
    // weighs (two cups at 0.5; caps weigh nothing).
    let cup = world::items::item_info(&order, FormId(CUP)).unwrap();
    assert_eq!((cup.name.as_str(), cup.value, cup.weight), ("Cup", 2, 0.5));
    assert_eq!(state.inventory_weight(&order, PLAYER_REF), 1.0);
    assert_eq!(
        ask(&order, &scripts, &mut state, "player.GetAV InventoryWeight"),
        1.0
    );
    // The container screen moves a stack, or part of one, back.
    assert_eq!(
        state.move_item(&order, PLAYER_REF, FormId(CHEST_REF), caps, 4),
        4
    );
    assert_eq!(
        state.move_item(&order, PLAYER_REF, FormId(CHEST_REF), FormId(CUP), 9),
        2
    );
    assert_eq!(
        state.inventory(&order, FormId(CHEST_REF)),
        vec![(caps, 4), (FormId(CUP), 2)]
    );
    assert_eq!(state.inventory(&order, PLAYER_REF), vec![(caps, 10)]);
}

#[test]
fn merchants_sell_from_their_container_at_the_games_prices() {
    use world::barter;
    let (_data, order) = order("scripting-barter");
    let scripts = ScriptCache::default();
    let mut state = GameState::new(&order);
    // A dialogue line's `ShowBarterMenu` on the speaker.
    Runner::new(&order, &scripts, &mut state).run_source(
        "ShowBarterMenu",
        Some(FormId(DOC_REF)),
        None,
    );
    assert_eq!(state.events.pop(), Some(Event::Barter(FormId(DOC_REF))));
    let goods = barter::merchant_container(&order, FormId(DOC_REF)).unwrap();
    assert_eq!(goods, FormId(CHEST_REF));
    // Barter 15 (2 + 2 × Charisma 5 + 3): a cup worth 2 costs 2 × (1.55 −
    // 0.45 × 0.15) = 2.97, rounded to 3, and sells for 2 × (0.45 + 0.45 ×
    // 0.15) = 1.04, rounded to 1.
    assert_eq!(
        ask(&order, &scripts, &mut state, "player.GetAV Barter"),
        15.0
    );
    let cup = FormId(CUP);
    let m = barter::multipliers(&order, &state);
    let worth = barter::item_value(&order, &state, goods, cup);
    assert_eq!(worth, 2.0);
    assert_eq!(barter::price(&order, &state, cup, worth, false, m, 0), 3.0);
    assert_eq!(barter::price(&order, &state, cup, worth, true, m, 0), 1.0);
    // Accepting the trade: the player's 3 caps for one cup.
    assert_eq!(
        barter::accept(&order, &mut state, FormId(DOC_REF), &[(cup, 1)], &[], -3.0),
        Some("ITMBottlecapsDown")
    );
    assert_eq!(state.item_count(&order, PLAYER_REF, FormId(CAPS)), 0);
    assert_eq!(state.item_count(&order, goods, FormId(CAPS)), 13);
    // Selling it back for 1: sold things go into the merchant container.
    assert_eq!(
        barter::accept(&order, &mut state, FormId(DOC_REF), &[], &[(cup, 1)], 1.0),
        Some("ITMBottlecapsUp")
    );
    assert_eq!(state.item_count(&order, PLAYER_REF, FormId(CAPS)), 1);
    assert_eq!(state.item_count(&order, goods, cup), 2);
}
#[test]
fn hits_hurt_kill_and_run_the_targets_scripts() {
    use world::combat::{self, Weapon};
    let (_data, order) = order("scripting-combat");
    let scripts = ScriptCache::default();
    let mut state = GameState::new(&order);
    let gecko = FormId(GECKO_REF);
    // The player picks up and equips the pistol.
    state.items.insert((PLAYER_REF, FormId(PISTOL)), 1);
    Runner::new(&order, &scripts, &mut state).run_source("player.EquipItem TestPistol", None, None);
    let pistol = combat::weapon_in_hand(&order, &state, PLAYER_REF).unwrap();
    assert_eq!(pistol, Weapon::load(&order, FormId(PISTOL)).unwrap());
    assert_eq!((pistol.damage, pistol.clip, pistol.skill), (16.0, 13, 41));
    assert!(!pistol.is_melee() && (pistol.shot_interval() - 0.32).abs() < 1e-6);
    // Guns 15: 16 × (0.5 + 0.5 × 0.15) × (0.66 + 0.34) = 9.2 a shot.
    let dealt = Runner::new(&order, &scripts, &mut state)
        .hit(PLAYER_REF, gecko, Some(&pistol))
        .unwrap();
    assert!((dealt - 9.2).abs() < 1e-4, "{dealt}");
    assert!((combat::health(&order, &state, gecko).unwrap() - 20.8).abs() < 1e-4);
    assert!(
        (ask(&order, &scripts, &mut state, "GeckoRef.GetHealthPercentage") - 20.8 / 30.0).abs()
            < 1e-5
    );
    // Hurt, it fights the player back.
    assert_eq!(state.combat.get(&gecko), Some(&PLAYER_REF));
    assert_eq!(ask(&order, &scripts, &mut state, "player.IsInCombat"), 1.0);
    // A damage threshold of 5 leaves 4.2; one of 8 would leave 1.2, but
    // 20% (1.84) always gets through.
    assert!((combat::after_threshold(&order, 9.2, 5.0) - 4.2).abs() < 1e-5);
    assert!((combat::after_threshold(&order, 9.2, 8.0) - 1.84).abs() < 1e-5);
    // Damage resistance comes off first (here 50%: 9.2 → 4.6, at least
    // 1.84): the gecko with DR 50 and DT 2 takes 2.6.
    let mut armoured = state.clone();
    armoured.actor_values.insert((gecko, 18), 50.0);
    armoured.actor_values.insert((gecko, 76), 2.0);
    assert!((combat::through_armour(&order, &armoured, 9.2, gecko, None) - 2.6).abs() < 1e-4);
    // Hollow points: the DT tripled (2 → 6) is taken off, then what's
    // left × 1.75 (the 20% floor last): (9.2 − 6) × 1.75 = 5.6.
    armoured.actor_values.insert((gecko, 18), 0.0);
    let hp = Some(FormId(HOLLOW_POINT));
    assert_eq!(combat::ammo_effects(&order, FormId(HOLLOW_POINT)).len(), 2);
    let dealt = combat::through_armour(&order, &armoured, 9.2, gecko, hp);
    assert!((dealt - (9.2 - 6.0) * 1.75).abs() < 1e-4, "{dealt}");
    // At most 85% resisted.
    armoured.actor_values.insert((gecko, 18), 100.0);
    armoured.actor_values.insert((gecko, 76), 0.0);
    assert!((combat::through_armour(&order, &armoured, 10.0, gecko, None) - 2.0).abs() < 1e-4);
    // Criticals: Luck 5 × the weapon's multiplier 1 = 5%: rolls below 50
    // (per mille) are critical; a sneak attack always is.
    armoured.actor_values.insert((PLAYER_REF, 11), 5.0);
    assert!(combat::critical(
        &order,
        &armoured,
        PLAYER_REF,
        Some(&pistol),
        gecko,
        false,
        49
    ));
    assert!(!combat::critical(
        &order,
        &armoured,
        PLAYER_REF,
        Some(&pistol),
        gecko,
        false,
        50
    ));
    assert!(combat::critical(
        &order,
        &armoured,
        PLAYER_REF,
        Some(&pistol),
        gecko,
        true,
        999
    ));
    assert_eq!(combat::sneak_multiplier(&order, false), 2.0);
    assert_eq!(combat::sneak_multiplier(&order, true), 5.0);
    // A shot: one projectile, no cone (the test pistol has no min spread);
    // fists reach 64.
    assert_eq!(pistol.shot(&order, None), (1, 0.0));
    assert_eq!(Weapon::melee_reach(None), 64.0);
    // Three more shots kill it: its OnDeath adds 100.
    state.globals.insert(FormId(GLOBAL), 0.0);
    let mut runner = Runner::new(&order, &scripts, &mut state);
    runner.hit(PLAYER_REF, gecko, Some(&pistol));
    runner.hit(PLAYER_REF, gecko, Some(&pistol));
    runner.hit(PLAYER_REF, gecko, Some(&pistol));
    assert!(state.dead.contains(&gecko));
    assert!(state.combat.is_empty());
    assert_eq!(state.globals[&FormId(GLOBAL)], 100.0);
    assert!(state.events.contains(&Event::Died {
        who: gecko,
        by: PLAYER_REF
    }));
    assert_eq!(ask(&order, &scripts, &mut state, "GeckoRef.GetDead"), 1.0);
    assert_eq!(
        ask(&order, &scripts, &mut state, "GetDeadCount TestGecko"),
        1.0
    );
    // The bottle counts the pistol's hits; it takes no damage.
    state.globals.insert(FormId(GLOBAL), 0.0);
    let bottle = Runner::new(&order, &scripts, &mut state).hit(
        PLAYER_REF,
        FormId(BOTTLE_REF),
        Some(&pistol),
    );
    assert_eq!(bottle, None);
    assert_eq!(state.globals[&FormId(GLOBAL)], 1.0);
    // Fists don't count as the pistol.
    Runner::new(&order, &scripts, &mut state).hit(PLAYER_REF, FormId(BOTTLE_REF), None);
    assert_eq!(state.globals[&FormId(GLOBAL)], 1.0);
    // A living gecko bites the player for its record's 8.
    Runner::new(&order, &scripts, &mut state).run_source("GeckoRef.ResurrectActor", None, None);
    let full = combat::health(&order, &state, PLAYER_REF).unwrap();
    let bite = Runner::new(&order, &scripts, &mut state)
        .hit(gecko, PLAYER_REF, None)
        .unwrap();
    assert_eq!(bite, 8.0);
    assert_eq!(combat::health(&order, &state, PLAYER_REF), Some(full - 8.0));
    assert!(state.unhandled.is_empty(), "{:?}", state.unhandled_first);
}

#[test]
fn explosions_hurt_through_the_hit_path() {
    use world::combat::{self, Weapon};
    let (_data, order) = order("scripting-explosion");
    let scripts = ScriptCache::default();
    let mut state = GameState::new(&order);
    let gecko = FormId(GECKO_REF);
    let pistol = Weapon::load(&order, FormId(PISTOL)).unwrap();
    // No criticals (Luck 0), no armour: the damage given is taken.
    state.actor_values.insert((PLAYER_REF, 11), 0.0);
    let hit = Runner::new(&order, &scripts, &mut state)
        .explosion_hit(Some(PLAYER_REF), gecko, Some(&pistol), 20.0)
        .unwrap();
    assert!((hit.dealt - 20.0).abs() < 1e-4 && hit.part.is_none() && !hit.critical);
    assert!((combat::health(&order, &state, gecko).unwrap() - 10.0).abs() < 1e-4);
    // Hurt, it fights its maker.
    assert_eq!(state.combat.get(&gecko), Some(&PLAYER_REF));
    // Armour comes off as for any hit: DR 50 halves it.
    let mut armoured = state.clone();
    armoured.actor_values.insert((gecko, 18), 50.0);
    let dealt = Runner::new(&order, &scripts, &mut armoured)
        .explosion_hit(Some(PLAYER_REF), gecko, Some(&pistol), 8.0)
        .unwrap()
        .dealt;
    assert!((dealt - 4.0).abs() < 1e-4, "{dealt}");
    // Another kills it: its OnDeath runs, the player the killer.
    state.globals.insert(FormId(GLOBAL), 0.0);
    Runner::new(&order, &scripts, &mut state).explosion_hit(
        Some(PLAYER_REF),
        gecko,
        Some(&pistol),
        20.0,
    );
    assert!(state.dead.contains(&gecko));
    assert_eq!(state.globals[&FormId(GLOBAL)], 100.0);
    assert!(state.events.contains(&Event::Died {
        who: gecko,
        by: PLAYER_REF
    }));
    // An object's OnHitWith blocks run (the bottle counts the weapon); it
    // takes no damage.
    state.globals.insert(FormId(GLOBAL), 0.0);
    let bottle = Runner::new(&order, &scripts, &mut state).explosion_hit(
        Some(PLAYER_REF),
        FormId(BOTTLE_REF),
        Some(&pistol),
        20.0,
    );
    assert_eq!(bottle, None);
    assert_eq!(state.globals[&FormId(GLOBAL)], 1.0);
}

#[test]
fn clothes_go_on_by_slot_and_aid_heals() {
    let (_data, order) = order("scripting-pipboy");
    let mut state = GameState::new(&order);
    let p = PLAYER_REF;
    // Clothes: the coat takes the shirt's slot; the hat has its own.
    state.equip(&order, p, FormId(SHIRT));
    state.equip(&order, p, FormId(HAT));
    state.equip(&order, p, FormId(COAT));
    assert!(state.is_equipped(p, FormId(COAT)) && state.is_equipped(p, FormId(HAT)));
    assert!(!state.is_equipped(p, FormId(SHIRT)));
    // One weapon in hand.
    state.equip(&order, p, FormId(PISTOL));
    assert!(state.is_equipped(p, FormId(PISTOL)));
    state.unequip(p, FormId(PISTOL));
    assert!(!state.is_equipped(p, FormId(PISTOL)));
    // Medicine: its effect read from the record, then +30 health.
    let effects = world::items::effects(&order, FormId(MEDICINE));
    assert_eq!(effects.len(), 1);
    assert_eq!((effects[0].magnitude, effects[0].actor_value), (30, 16));
    state.damage.insert(p, 50.0);
    assert!(world::items::use_item(&order, &mut state, p, FormId(MEDICINE)).is_none());
    state.items.insert((p, FormId(MEDICINE)), 2);
    let said = world::items::use_item(&order, &mut state, p, FormId(MEDICINE)).unwrap();
    assert_eq!(said, "Used Medicine: +30 Health.");
    assert_eq!(state.damage[&p], 20.0);
    assert_eq!(state.item_count(&order, p, FormId(MEDICINE)), 1);
    // Healing stops at full health.
    world::items::use_item(&order, &mut state, p, FormId(MEDICINE));
    assert_eq!(state.damage[&p], 0.0);
}

#[test]
fn effects_last_their_time_heal_over_time_and_scripts_cast_spells() {
    let (_data, order) = order("scripting-magic");
    let scripts = ScriptCache::default();
    let mut state = GameState::new(&order);
    let p = PLAYER_REF;
    let update = |state: &mut GameState, seconds: f32, times: usize| {
        for _ in 0..times {
            Runner::new(&order, &scripts, state).update(seconds);
        }
    };
    let strength = |state: &mut GameState| ask(&order, &scripts, state, "player.GetAV Strength");
    let rads = |state: &mut GameState| ask(&order, &scripts, state, "player.GetAV RadiationRads");
    // The player's record has Strength 5; half of any radiation is resisted.
    assert_eq!(strength(&mut state), 5.0);
    Runner::new(&order, &scripts, &mut state).run_source("player.SetAV RadResist 50", None, None);
    state.damage.insert(p, 20.0);
    state.items.insert((p, FormId(TONIC)), 1);
    let caps = state.item_count(&order, p, FormId(CAPS));
    let said = world::items::use_item(&order, &mut state, p, FormId(TONIC)).unwrap();
    assert_eq!(
        said,
        "Used Tonic: +2 Strength for 10 s, +3 Health a second for 4 s, +5 RadiationRads."
    );
    // At once: +2 Strength while it lasts, and 5 rads.
    assert_eq!(strength(&mut state), 7.0);
    assert_eq!(
        ask(&order, &scripts, &mut state, "player.GetBaseAV Strength"),
        5.0
    );
    assert_eq!(rads(&mut state), 5.0);
    // Over 4 s, 12 health comes back; the script effect gives a cap.
    update(&mut state, 0.5, 8);

    assert_eq!(state.damage[&p], 8.0);
    assert_eq!(state.item_count(&order, p, FormId(CAPS)), caps + 1);
    assert_eq!(strength(&mut state), 7.0);
    // After 10 s the Strength is back.
    update(&mut state, 0.5, 12);
    assert_eq!(strength(&mut state), 5.0);
    // A doctor's cure, as the game's dialogue does it.
    Runner::new(&order, &scripts, &mut state).run_source(
        "player.RestoreAV RadiationRads 1000",
        None,
        None,
    );
    assert_eq!(rads(&mut state), 0.0);

    // A spell cast lasts its time; an ability lasts while held.
    Runner::new(&order, &scripts, &mut state).run_source(
        "player.CastImmediateOnSelf TestSpell\nplayer.AddSpell TestAbility",
        None,
        None,
    );
    assert_eq!(strength(&mut state), 9.0);
    assert_eq!(
        ask(&order, &scripts, &mut state, "player.GetPermAV Strength"),
        8.0
    );
    assert_eq!(
        ask(
            &order,
            &scripts,
            &mut state,
            "player.IsSpellTarget TestSpell"
        ),
        1.0
    );
    assert_eq!(
        ask(
            &order,
            &scripts,
            &mut state,
            "player.HasMagicEffect TestFortifyStrength"
        ),
        1.0
    );
    // Saved and loaded, they carry on.
    let text = world::save::save(&state, None);
    let (mut state, _) = world::save::load(&text).unwrap();
    assert_eq!(world::save::save(&state, None), text);
    update(&mut state, 50.0, 2);
    assert_eq!(strength(&mut state), 8.0);
    Runner::new(&order, &scripts, &mut state).run_source(
        "player.RemoveSpell TestAbility",
        None,
        None,
    );
    assert_eq!(strength(&mut state), 5.0);
    assert_eq!(
        ask(
            &order,
            &scripts,
            &mut state,
            "player.HasMagicEffect TestFortifyStrength"
        ),
        0.0
    );

    // A worn weapon does less damage.
    state.items.insert((p, FormId(PISTOL)), 1);
    state.equip(&order, p, FormId(PISTOL));
    let pistol = world::combat::Weapon::load(&order, FormId(PISTOL)).unwrap();
    let whole = world::combat::hit_damage(&order, &state, p, &pistol);
    assert_eq!(
        ask(&order, &scripts, &mut state, "player.GetWeaponHealthPerc"),
        100.0
    );
    Runner::new(&order, &scripts, &mut state).run_source(
        "player.SetWeaponHealthPerc 50",
        None,
        None,
    );
    assert_eq!(
        ask(&order, &scripts, &mut state, "player.GetWeaponHealthPerc"),
        50.0
    );
    // The game's curve: full above 75%, then 1 − 0.67 × (0.75 − 0.5).
    let worn = world::combat::hit_damage(&order, &state, p, &pistol);
    assert!((worn / whole - 0.8325).abs() < 1e-4, "{worn} / {whole}");
    Runner::new(&order, &scripts, &mut state).run_source(
        "player.SetWeaponHealthPerc 80",
        None,
        None,
    );
    let fine = world::combat::hit_damage(&order, &state, p, &pistol);
    assert!((fine - whole).abs() < 1e-5, "{fine} / {whole}");
    assert!(state.unhandled.is_empty(), "{:?}", state.unhandled_first);
}

#[test]
fn people_go_through_load_doors_toward_their_package() {
    use world::ai;
    let (_data, order) = order("scripting-doors");
    let scripts = ScriptCache::default();
    let mut state = GameState::new(&order);
    let doc = FormId(DOC_REF);
    // A package to the marker in the other cell.
    Runner::new(&order, &scripts, &mut state).run_source(
        "DocRef.AddScriptPackage TestFarTravelPackage",
        None,
        None,
    );
    let package = ai::current_package(&order, &state, doc).unwrap();
    assert_eq!(package.form_id, FormId(FAR_TRAVEL));
    // Not reachable on foot from here...
    assert!(ai::destination(&order, &state, doc, &package).is_none());
    let (space, _) = ai::target_place(&order, &state, doc, &package).unwrap();
    assert_eq!(space, FormId(CELL2));
    // ...but through the door at 150,150, coming out at 400,0 over there.
    let way = ai::door_toward(&order, &state, doc, space).unwrap();
    assert_eq!(way.door, FormId(LINK_DOOR));
    assert_eq!(
        (way.at, way.to, way.to_space),
        ([150.0, 150.0, 0.0], [400.0, 0.0, 0.0], FormId(CELL2))
    );
    // Out of sight they walk there at their pace (`009ea8a0`): 100 units
    // toward the door (212 away), then on through it, then to the marker
    // (141 from the far side; arriving within 20 of it, a chest's bounds-less
    // radius).
    let mut navs = ai::NavCache::default();
    assert_eq!(
        ai::move_offstage(&order, &mut state, doc, 100.0, &mut navs),
        ai::Offstage::Moved
    );
    let (space, _, at, _) = state.place(&order, doc).unwrap();
    assert_eq!(space, FormId(CELL));
    assert!(
        (at[0] - 70.71).abs() < 0.1 && (at[1] - 70.71).abs() < 0.1,
        "{at:?}"
    );
    assert_eq!(
        ai::move_offstage(&order, &mut state, doc, 150.0, &mut navs),
        ai::Offstage::Moved
    );
    assert_eq!(state.place(&order, doc).unwrap().0, FormId(CELL2));
    assert_eq!(ai::moved_into(&order, &state, FormId(CELL2)), vec![doc]);
    assert!(ai::moved_into(&order, &state, FormId(CELL)).is_empty());
    assert_eq!(
        ai::move_offstage(&order, &mut state, doc, 1000.0, &mut navs),
        ai::Offstage::Arrived
    );
    let at = state.place(&order, doc).unwrap().2;
    let d = ((at[0] - 500.0).powi(2) + (at[1] - 100.0).powi(2)).sqrt();
    assert!(d < 20.0 + 1e-3, "{at:?}");
    assert_eq!(
        ai::move_offstage(&order, &mut state, doc, 1000.0, &mut navs),
        ai::Offstage::Arrived
    );
    // Following the player: near wherever the player is, within the
    // package's 200; through the door when the player is elsewhere.
    Runner::new(&order, &scripts, &mut state).run_source(
        "DocRef.AddScriptPackage TestFollowPlayerPackage",
        None,
        None,
    );
    let follow = ai::current_package(&order, &state, doc).unwrap();
    state.player_cell = Some(FormId(CELL2));
    state.player_position = Some([300.0, 50.0, 0.0]);
    assert_eq!(
        ai::destination(&order, &state, doc, &follow),
        Some(([300.0, 50.0, 0.0], 200.0))
    );
    state.player_cell = Some(FormId(CELL));
    assert!(ai::destination(&order, &state, doc, &follow).is_none());
    assert_eq!(
        ai::target_place(&order, &state, doc, &follow).map(|t| t.0),
        Some(FormId(CELL))
    );
    // And back, from the far side's door.
    let home = ai::door_toward(&order, &state, doc, FormId(CELL)).unwrap();
    assert_eq!(
        (home.door, home.to),
        (FormId(LINK_DOOR2), [150.0, 120.0, 0.0])
    );
}

#[test]
fn locks_keys_and_terminals() {
    use world::locks::{self, Opening};
    let (_data, order) = order("scripting-locks");
    let scripts = ScriptCache::default();
    let mut state = GameState::new(&order);
    let strongbox = FormId(STRONGBOX_REF);
    // Placed locked (level 50, its key), as `XLOC` says.
    let lock = locks::lock_now(&order, &state, strongbox).unwrap();
    assert_eq!((lock.level, lock.key), (50, Some(FormId(KEY))));
    assert_eq!(
        ask(&order, &scripts, &mut state, "StrongboxRef.GetLockLevel"),
        50.0
    );
    // Not enough Lockpick: it stays shut.
    Runner::new(&order, &scripts, &mut state).run_source("player.SetAV Lockpick 20", None, None);
    assert_eq!(
        locks::try_open(&order, &mut state, strongbox),
        Opening::NeedsSkill(50)
    );
    // With the key it opens, and stays unlocked: "Unlocked with <key>."
    // with the key picture (`005180b0`, `00516dc0`).
    state.events.clear();
    state.items.insert((PLAYER_REF, FormId(KEY)), 1);
    assert_eq!(
        locks::try_open(&order, &mut state, strongbox),
        Opening::WithKey
    );
    let unlocked: Vec<_> = state
        .events
        .iter()
        .filter_map(|e| match e {
            Event::Message { text, icon, .. } => Some((text.clone(), icon.clone())),
            _ => None,
        })
        .collect();
    assert_eq!(unlocked.len(), 1, "{unlocked:?}");
    assert!(unlocked[0].0.starts_with("Unlocked with "), "{unlocked:?}");
    assert_eq!(unlocked[0].1.as_deref(), Some(world::message_icon::KEY));
    assert_eq!(
        ask(&order, &scripts, &mut state, "StrongboxRef.GetLocked"),
        0.0
    );
    // Scripts lock it again, and a skilled player picks it.
    state.items.clear();
    state.stocked.clear();
    Runner::new(&order, &scripts, &mut state).run_source(
        "StrongboxRef.Lock 25\nplayer.SetAV Lockpick 30",
        None,
        None,
    );
    assert_eq!(
        ask(&order, &scripts, &mut state, "StrongboxRef.GetLocked"),
        1.0
    );
    // With the skill the lockpicking game opens; won, the lock opens.
    assert_eq!(
        locks::try_open(&order, &mut state, strongbox),
        Opening::Pick(1)
    );
    assert_eq!(
        ask(&order, &scripts, &mut state, "StrongboxRef.GetLocked"),
        1.0
    );
    world::lockpick::picked(&order, &mut state, strongbox, 1);
    assert_eq!(
        ask(&order, &scripts, &mut state, "StrongboxRef.GetLocked"),
        0.0
    );
    // Picking an easy lock (25) is worth `iXPRewardPickLockEasy`; the key
    // gave nothing. "Locks Picked" counts it.
    assert_eq!(world::experience::xp(&state), 30.0);
    assert_eq!(
        state.misc_stats.get(&world::stats::LOCKS_PICKED).copied(),
        Some(1)
    );
    // Picked again: counted, but no more experience.
    state.locks.insert(strongbox, Some(25));
    world::lockpick::picked(&order, &mut state, strongbox, 1);
    assert_eq!(world::experience::xp(&state), 30.0);
    // Forcing that fails breaks it: then only its key opens it.
    state.locks.insert(strongbox, Some(25));
    assert_eq!(
        ask(&order, &scripts, &mut state, "StrongboxRef.GetIsLockBroken"),
        0.0
    );
    world::lockpick::break_lock(&mut state, strongbox);
    assert_eq!(
        locks::try_open(&order, &mut state, strongbox),
        Opening::NeedsKey
    );
    assert_eq!(
        ask(&order, &scripts, &mut state, "StrongboxRef.GetIsLockBroken"),
        1.0
    );
    state.broken_locks.clear();
    state.locks.insert(strongbox, None);

    // The terminal: header, difficulty, items; its first unlocks its link.
    let t = world::terminal::Terminal::load(&order, FormId(TERMINAL)).unwrap();
    assert_eq!(
        (t.name.as_str(), t.header.as_str()),
        ("Office Terminal", "Welcome, USER")
    );
    assert!(!t.unlocked());
    assert_eq!(world::hacking::min_skill(t.difficulty), 25.0);
    assert_eq!(t.items.len(), 2);
    assert_eq!(t.items[0].result.as_deref(), Some("Unlocking..."));
    assert_eq!(t.items[1].note, Some(FormId(NOTE)));
    assert_eq!(
        world::terminal::note_text(&order, FormId(NOTE)),
        Some(("Memo".into(), "The code is 1234.".into()))
    );
    Runner::new(&order, &scripts, &mut state).run_source("StrongboxRef.Lock 75", None, None);
    let terminal = Some(FormId(TERMINAL_REF));
    Runner::new(&order, &scripts, &mut state).run_source(
        t.items[0].script.as_deref().unwrap(),
        terminal,
        terminal,
    );
    assert!(locks::lock_now(&order, &state, strongbox).is_none());
    assert!(state.unhandled.is_empty(), "{:?}", state.unhandled_first);

    // Getting in: the hacking game once Science reaches the easy
    // minimum (25); hacked, it opens for good, worth
    // `iXPRewardHackComputerEasy` once, and `GetLocked` still says 1.
    use world::terminal::{access, Access};
    let r = FormId(TERMINAL_REF);
    Runner::new(&order, &scripts, &mut state).run_source("player.SetAV Science 10", None, None);
    assert_eq!(access(&order, &state, &t, r), Access::NeedsScience(25));
    Runner::new(&order, &scripts, &mut state).run_source("player.SetAV Science 25", None, None);
    assert_eq!(access(&order, &state, &t, r), Access::Hack);
    world::terminal::hacked(&order, &mut state, &t, r);
    assert_eq!(world::experience::xp(&state), 60.0);
    Runner::new(&order, &scripts, &mut state).run_source("player.SetAV Science 0", None, None);
    assert_eq!(access(&order, &state, &t, r), Access::Open);
    assert_eq!(
        ask(&order, &scripts, &mut state, "TerminalRef.GetLocked"),
        1.0
    );
    assert_eq!(
        ask(&order, &scripts, &mut state, "TerminalRef.GetLockLevel"),
        1.0
    );
    // A script's `Lock` takes the hack away and sets the level; `Unlock`
    // opens it.
    Runner::new(&order, &scripts, &mut state).run_source("TerminalRef.Lock 3", None, None);
    Runner::new(&order, &scripts, &mut state).run_source("player.SetAV Science 60", None, None);
    assert_eq!(access(&order, &state, &t, r), Access::NeedsScience(75));
    assert_eq!(
        ask(&order, &scripts, &mut state, "TerminalRef.GetLockLevel"),
        3.0
    );
    Runner::new(&order, &scripts, &mut state).run_source("TerminalRef.Unlock", None, None);
    assert_eq!(access(&order, &state, &t, r), Access::Open);
    assert_eq!(
        ask(&order, &scripts, &mut state, "TerminalRef.GetLocked"),
        0.0
    );
    assert_eq!(
        ask(&order, &scripts, &mut state, "TerminalRef.GetLockLevel"),
        -1.0
    );
    // Locked out: `GetLocked` 2; level 5 ("requires key") too.
    Runner::new(&order, &scripts, &mut state).run_source("TerminalRef.Lock", None, None);
    world::terminal::lock_out(&mut state, r);
    assert_eq!(access(&order, &state, &t, r), Access::LockedOut);
    assert_eq!(
        ask(&order, &scripts, &mut state, "TerminalRef.GetLocked"),
        2.0
    );
    Runner::new(&order, &scripts, &mut state).run_source("TerminalRef.Unlock", None, None);
    Runner::new(&order, &scripts, &mut state).run_source("TerminalRef.Lock 5", None, None);
    assert_eq!(access(&order, &state, &t, r), Access::LockedOut);
    Runner::new(&order, &scripts, &mut state).run_source("TerminalRef.Unlock", None, None);
    Runner::new(&order, &scripts, &mut state).run_source("player.SetAV Science 0", None, None);
    // `ForceTerminalBack` (a terminal item's script, the terminal menu
    // open) asks the menu back.
    state.events.clear();
    state.more.menu_open = Some(world::terminal::TERMINAL_MENU);
    Runner::new(&order, &scripts, &mut state).run_source("ForceTerminalBack", Some(r), Some(r));
    state.more.menu_open = None;
    assert!(state
        .events
        .iter()
        .any(|e| matches!(e, world::scripting::Event::TerminalBack)));
    // Quest scripts reward experience too.
    Runner::new(&order, &scripts, &mut state).run_source("RewardXP 25", None, None);
    assert_eq!(world::experience::xp(&state), 85.0);
    assert_eq!(ask(&order, &scripts, &mut state, "player.GetAV XP"), 85.0);
}

#[test]
fn levels_come_as_the_game_works_them_out() {
    use world::experience::{self as xp, LevelUp};
    let (_data, order) = order("scripting-levels");
    let scripts = ScriptCache::default();
    let mut state = GameState::new(&order);
    // 25 (L − 1)(3L + 2): `iXPBase` 200 (the exe's default) growing by
    // `iXPBumpBase` 150.
    assert_eq!(xp::xp_for_level(&order, 1), 0.0);
    assert_eq!(xp::xp_for_level(&order, 2), 200.0);
    assert_eq!(xp::xp_for_level(&order, 3), 550.0);
    assert_eq!(xp::xp_for_level(&order, 30), 66_700.0);
    // No XP yet: the base form gives 0 for it (`005f0fb0`), so a script
    // testing it goes on (VCG04's revise prompt); the resistances only
    // effects raise are 0 the same way.
    assert_eq!(ask(&order, &scripts, &mut state, "player.GetAV XP"), 0.0);
    assert_eq!(
        ask(&order, &scripts, &mut state, "player.GetAV XP == 0"),
        1.0
    );
    assert_eq!(
        ask(&order, &scripts, &mut state, "player.GetAV RadResist"),
        0.0
    );
    assert_eq!(
        ask(&order, &scripts, &mut state, "player.GetAV FireResist"),
        0.0
    );
    let health_at_1 = world::combat::max_health(&order, &state, PLAYER_REF).unwrap();
    Runner::new(&order, &scripts, &mut state).run_source("player.SetAV Intelligence 5", None, None);

    // Short of level 2 nothing waits; reaching it, a level-up waits until
    // the player is out of combat.
    xp::reward(&order, &mut state, 199.0);
    assert!(!state.level_up_pending);
    xp::reward(&order, &mut state, 1.0);
    assert!(state.level_up_pending);
    state.combat.insert(FormId(GECKO_REF), PLAYER_REF);
    assert!(!xp::level_up_ready(&state));
    state.combat.clear();
    assert!(xp::level_up_ready(&state));
    // Level 2: ⌊10 + 0.5 × 5⌋ = 12, + 1 on an even level with odd
    // Intelligence; a perk on even levels.
    assert_eq!(
        xp::level_up(&order, &mut state),
        LevelUp {
            level: 2,
            skill_points: 13,
            perk: true
        }
    );
    assert!(!state.level_up_pending);
    // +5 health a level.
    let health_at_2 = world::combat::max_health(&order, &state, PLAYER_REF).unwrap();
    assert_eq!(health_at_2 - health_at_1, 5.0);

    // The perk list: not traits, not the women's perk for a man; the
    // level-4 perk listed but not available yet.
    let choices = world::perks::level_up_choices(&order, &state);
    let names: Vec<&str> = choices.iter().map(|c| c.name.as_str()).collect();
    assert_eq!(names, ["Quick Study", "Schooled", "Test Perk"]);
    assert!(choices[0].available && !choices[1].available);
    Runner::new(&order, &scripts, &mut state).run_source("player.SexChange 1", None, None);
    let names: Vec<String> = world::perks::level_up_choices(&order, &state)
        .into_iter()
        .map(|c| c.name)
        .collect();
    assert!(names.contains(&"Ladies Only".to_string()));

    // Perks' entry points: experience × 1.1, rounded up (10 → 11, 7 → 8);
    // skill points + 2.
    Runner::new(&order, &scripts, &mut state).run_source(
        "player.AddPerk QuickStudy\nplayer.AddPerk Schooled",
        None,
        None,
    );
    assert_eq!(xp::reward(&order, &mut state, 10.0), 11.0);
    assert_eq!(xp::reward(&order, &mut state, 7.0), 8.0);
    assert_eq!(xp::xp(&state), 219.0);
    // Its second rank uses the second rank's entry (× 1.2), not both.
    let choices = world::perks::level_up_choices(&order, &state);
    let quick = choices.iter().find(|c| c.name == "Quick Study").unwrap();
    assert_eq!((quick.rank, quick.ranks), (2, 2));
    Runner::new(&order, &scripts, &mut state).run_source("player.AddPerk QuickStudy", None, None);
    assert_eq!(xp::reward(&order, &mut state, 10.0), 12.0);
    state.actor_values.insert((PLAYER_REF, xp::XP), 219.0);
    xp::reward(&order, &mut state, 400.0);
    assert!(state.level_up_pending);
    let up = xp::level_up(&order, &mut state);
    assert_eq!((up.level, up.skill_points, up.perk), (3, 14, false));

    // Nothing past the last level's experience, and nothing at it.
    state.player_level = 29;
    let cap = xp::xp_for_level(&order, 30);
    state.actor_values.insert((PLAYER_REF, xp::XP), cap - 5.0);
    assert_eq!(xp::reward(&order, &mut state, 100.0), 5.0);
    state.player_level = 30;
    assert_eq!(xp::reward(&order, &mut state, 100.0), 0.0);
}

#[test]
fn perks_give_abilities_and_conditional_entries() {
    let (_data, order) = order("scripting-perk-abilities");
    let scripts = ScriptCache::default();
    let mut state = GameState::new(&order);
    Runner::new(&order, &scripts, &mut state).run_source("player.SetAV Strength 5", None, None);
    let dt = |state: &GameState| world::combat::damage_threshold(&order, state, PLAYER_REF);
    let before = dt(&state);
    Runner::new(&order, &scripts, &mut state).run_source("player.AddPerk ToughSkin", None, None);
    // The ability: + 3 Strength while the perk is held.
    assert_eq!(
        ask(&order, &scripts, &mut state, "player.GetAV Strength"),
        8.0
    );
    // The entry's condition (a woman) fails for a man: no change.
    assert_eq!(dt(&state), before);
    Runner::new(&order, &scripts, &mut state).run_source("player.SexChange 1", None, None);
    assert_eq!(dt(&state), before + 3.0);
    Runner::new(&order, &scripts, &mut state).run_source("player.RemovePerk ToughSkin", None, None);
    assert_eq!(
        ask(&order, &scripts, &mut state, "player.GetAV Strength"),
        5.0
    );
    assert_eq!(dt(&state), before);
    assert_eq!(
        world::perks::ENTRY_POINT_NAMES[56],
        "Modify Damage Threshold (defender)"
    );
}

#[test]
fn reputation_and_karma_move_as_the_game_moves_them() {
    let (_data, order) = order("scripting-reputation");
    let scripts = ScriptCache::default();
    let mut state = GameState::new(&order);
    let run = |state: &mut GameState, line: &str| {
        Runner::new(&order, &scripts, state).run_source(line, None, None);
    };
    let rep = FormId(REPUTATION);
    // A new reputation: nothing known, threshold 1 on every axis.
    assert_eq!(
        ask(
            &order,
            &scripts,
            &mut state,
            "GetReputationThreshold RepTestville 1"
        ),
        1.0
    );
    // Fame (type 1) by an average bump: 4 of 20 = 0.2, level 1, "Accepted".
    state.events.clear();
    run(&mut state, "AddReputation RepTestville 1 3");
    assert_eq!(world::reputation::get(&state, rep, 1), 4.0);
    assert!(
        (ask(
            &order,
            &scripts,
            &mut state,
            "GetReputationPct RepTestville 1"
        ) - 0.2)
            .abs()
            < 1e-6
    );
    assert_eq!(
        ask(
            &order,
            &scripts,
            &mut state,
            "GetReputationThreshold RepTestville 1"
        ),
        4.0
    );
    let texts: Vec<String> = state
        .events
        .iter()
        .filter_map(|e| match e {
            Event::Message { text, .. } => Some(text.clone()),
            _ => None,
        })
        .collect();
    // The notice with the exe's words (this world's data has none); the
    // new title in the game's box: titled with the reputation's name,
    // "<title>\n<description>".
    assert_eq!(texts, ["Testville\nReputation Gain"]);
    assert!(state.events.iter().any(|e| matches!(e,
        Event::Popup { title: Some(t), text, .. }
            if t == "Testville"
                && text == "Accepted\nFolks have come to accept you for your helpful nature.")));
    // Fame gained has the very happy Vault Boy (`sRepPositiveGainIcon`).
    let icons: Vec<Option<String>> = state
        .events
        .iter()
        .filter_map(|e| match e {
            Event::Message { icon, .. } => Some(icon.clone()),
            _ => None,
        })
        .collect();
    assert_eq!(icons, [Some(world::message_icon::VERY_HAPPY.to_string())]);
    // Infamy to the top (12 + 12 = 24, clamped at 20): level 3 against fame
    // 1, "Merciful Thug", on the bad axis 3.
    run(
        &mut state,
        "AddReputation RepTestville 0 5\nAddReputation RepTestville 0 5",
    );
    assert_eq!(world::reputation::get(&state, rep, 0), 20.0);
    assert_eq!(
        ask(
            &order,
            &scripts,
            &mut state,
            "GetReputationThreshold RepTestville 2"
        ),
        3.0
    );
    assert_eq!(world::reputation::title(&order, 1, 3), "Merciful Thug");
    // Exact and set; set doesn't clamp.
    run(
        &mut state,
        "RemoveReputationExact RepTestville 0 2.5\nSetReputation RepTestville 1 50",
    );
    assert_eq!(world::reputation::get(&state, rep, 0), 17.5);
    assert_eq!(world::reputation::get(&state, rep, 1), 50.0);
    // Karma: clamped at 1000.
    run(&mut state, "RewardKarma 300");
    assert_eq!(world::reputation::karma(&order, &state), 300.0);
    run(&mut state, "RewardKarma 900");
    assert_eq!(world::reputation::karma(&order, &state), 1000.0);
    assert_eq!(world::reputation::alignment(&order, 1000.0), 3);
    assert_eq!(world::reputation::alignment(&order, -300.0), 2);
    // Kept in a save.
    let (loaded, _) = world::save::load(&world::save::save(&state, None)).unwrap();
    assert_eq!(world::reputation::get(&loaded, rep, 0), 17.5);
}

#[test]
fn stealing_costs_karma_and_a_second_seen_theft_starts_a_fight() {
    use world::crime;
    let (_data, order) = order("scripting-steal");
    let scripts = ScriptCache::default();
    let mut state = GameState::new(&order);
    // The player stands in front of the doctor (facing north at 0,0).
    state.player_cell = Some(FormId(CELL));
    state.player_world = None;
    state.player_position = Some([0.0, 50.0, 0.0]);
    let (chest, doc) = (FormId(CHEST_REF), FormId(DOC));
    assert!(crime::may_take(&order, &state, None));
    assert!(!crime::may_take(&order, &state, Some(doc)));
    // Taking the doctor's things: −5 karma (he isn't evil), and he saw.
    assert!(crime::steal(&order, &mut state, chest, doc));
    assert_eq!(world::reputation::karma(&order, &state), -5.0);
    assert!(!state.combat.contains_key(&FormId(DOC_REF)));
    // Again the same day: he attacks.
    assert!(crime::steal(&order, &mut state, chest, doc));
    assert_eq!(state.combat.get(&FormId(DOC_REF)), Some(&PLAYER_REF));
    // Thefts count for his town, not for the player's own minor crimes
    // (`Actor::StealAlarm` counts them on the victim).
    assert_eq!(state.player_crimes.0, 0);
    // Out of his sight (behind him), nobody sees, but the karma goes.
    let mut unseen = GameState::new(&order);
    unseen.player_cell = Some(FormId(CELL));
    unseen.player_position = Some([0.0, -1400.0, 0.0]);
    assert!(!crime::steal(&order, &mut unseen, chest, doc));
    assert_eq!(world::reputation::karma(&order, &unseen), -5.0);
    // Hitting the doctor (who wasn't fighting): an assault, so he fights
    // back; killing him a murder.
    let mut state = GameState::new(&order);
    state.player_cell = Some(FormId(CELL));
    state.player_position = Some([0.0, 50.0, 0.0]);
    let doc_ref = FormId(DOC_REF);
    Runner::new(&order, &scripts, &mut state).hit(PLAYER_REF, doc_ref, None);
    assert_eq!(state.player_crimes.1, 1);
    assert_eq!(state.combat.get(&doc_ref), Some(&PLAYER_REF));
    state.combat.clear();
    let full = world::combat::max_health(&order, &state, doc_ref).unwrap();
    state.damage.insert(doc_ref, full - 0.01);
    Runner::new(&order, &scripts, &mut state).hit(PLAYER_REF, doc_ref, None);
    assert!(state.dead.contains(&doc_ref));
    assert!(state.player_murderer);
    assert_eq!(ask(&order, &scripts, &mut state, "IsPCAMurderer"), 1.0);
    // A faction holding the player as an enemy for their crimes: its
    // members are enemies until it's cleared.
    let mut state = GameState::new(&order);
    Runner::new(&order, &scripts, &mut state).run_source(
        "SetPCEnemyofFaction TestRaiders",
        None,
        None,
    );
    assert_eq!(
        ask(
            &order,
            &scripts,
            &mut state,
            "GetPCEnemyofFaction TestRaiders"
        ),
        1.0
    );
    assert_eq!(
        world::factions::reaction(&order, &state, FormId(DOC_REF), PLAYER_REF),
        world::factions::Reaction::Enemy
    );
    Runner::new(&order, &scripts, &mut state).run_source(
        "ClearFactionPlayerEnemyFlag TestRaiders",
        None,
        None,
    );
    assert_eq!(
        world::factions::reaction(&order, &state, FormId(DOC_REF), PLAYER_REF),
        world::factions::Reaction::Neutral
    );
}

#[test]
fn skill_books_raise_their_skill_and_are_used_up() {
    let (_data, order) = order("scripting-books");
    let scripts = ScriptCache::default();
    let mut state = GameState::new(&order);
    let book = FormId(SCIENCE_BOOK);
    assert_eq!(world::items::book_skill(&order, book), Some(40));
    state.items.insert((PLAYER_REF, book), 2);
    state.stocked.insert(PLAYER_REF);
    // `fBookPerkBonus` 3, kept with the points put into Science.
    // The notice is `sSkillIncreasedNum` "%s increased by %d" (`00515040`).
    assert_eq!(
        world::items::read_book(&order, &mut state, book).as_deref(),
        Some("Science increased by 3")
    );
    // Reading in combat, outside the menus, is refused (`sCanNotReadBook`).
    assert_eq!(world::items::cannot_read_in_combat(&order, &state), None);
    state.combat.insert(PLAYER_REF, FormId(GECKO_REF));
    assert_eq!(
        world::items::cannot_read_in_combat(&order, &state).as_deref(),
        Some("You cannot read a book during combat!")
    );
    state.combat.clear();
    assert_eq!(state.skill_points.get(&40), Some(&3));
    assert_eq!(state.item_count(&order, PLAYER_REF, book), 1);
    // With a perk adding 1 to book points (entry point 11), 4.
    Runner::new(&order, &scripts, &mut state).run_source("player.AddPerk Bookworm", None, None);
    world::items::read_book(&order, &mut state, book);
    assert_eq!(state.skill_points.get(&40), Some(&7));
    assert_eq!(state.item_count(&order, PLAYER_REF, book), 0);
    // A value scripts set takes the points itself.
    state.items.insert((PLAYER_REF, book), 1);
    Runner::new(&order, &scripts, &mut state).run_source("player.SetAV Science 50", None, None);
    world::items::read_book(&order, &mut state, book);
    assert_eq!(
        ask(&order, &scripts, &mut state, "player.GetAV Science"),
        54.0
    );
    // Nothing left to read.
    assert_eq!(world::items::read_book(&order, &mut state, book), None);
    // At 200 nothing happens and nothing is said; the book stays.
    state.items.insert((PLAYER_REF, book), 1);
    Runner::new(&order, &scripts, &mut state).run_source("player.SetAV Science 200", None, None);
    assert_eq!(world::items::read_book(&order, &mut state, book), None);
    assert_eq!(state.item_count(&order, PLAYER_REF, book), 1);
}

#[test]
fn kills_give_experience_for_the_players_share() {
    use world::combat::hurt;
    let (_data, order) = order("scripting-kills");
    let scripts = ScriptCache::default();
    let gecko = FormId(GECKO_REF);
    let doc = FormId(DOC_REF);
    // The gecko has 30 health. The player does 12 (40%, not more than
    // `iXPDeathRewardHealthThreshold`), the doctor the rest: nothing.
    let mut state = GameState::new(&order);
    hurt(&order, &mut state, gecko, 12.0, PLAYER_REF);
    assert!(hurt(&order, &mut state, gecko, 20.0, doc));
    assert_eq!(world::experience::xp(&state), 0.0);
    // 13 (43%): the creature table's reward for its level (0: the very
    // easy row, 1 XP by the exe's default).
    let mut state = GameState::new(&order);
    hurt(&order, &mut state, gecko, 13.0, PLAYER_REF);
    hurt(&order, &mut state, gecko, 20.0, doc);
    assert_eq!(world::experience::xp(&state), 1.0);
    // The Pip-Boy counts the player's kills only (the doctor killed this
    // one): none yet; scripts can add to a statistic.
    assert_eq!(
        ask(
            &order,
            &scripts,
            &mut state,
            "GetPCMiscStat \"Creatures Killed\""
        ),
        0.0
    );
    Runner::new(&order, &scripts, &mut state).run_source(
        "ModPCMiscStat \"Books Read\" 2",
        None,
        None,
    );
    assert_eq!(world::stats::get(&state, world::stats::BOOKS_READ), 2);
    // A teammate's damage counts as the player's.
    let mut state = GameState::new(&order);
    Runner::new(&order, &scripts, &mut state).run_source("DocRef.SetPlayerTeammate 1", None, None);
    assert_eq!(
        ask(&order, &scripts, &mut state, "GetPlayerTeammateCount"),
        1.0
    );
    hurt(&order, &mut state, gecko, 30.0, doc);
    assert_eq!(world::experience::xp(&state), 1.0);
    // Killed by the player: a creature, counted.
    let mut state = GameState::new(&order);
    hurt(&order, &mut state, gecko, 30.0, PLAYER_REF);
    assert_eq!(
        ask(
            &order,
            &scripts,
            &mut state,
            "GetPCMiscStat \"Creatures Killed\""
        ),
        1.0
    );
    assert_eq!(
        world::stats::get(&state, world::stats::TOTAL_THINGS_KILLED),
        1
    );
    // People's table: by their level against `iXPLevelKillNPC…`.
    assert_eq!(world::experience::kill_xp(&order, false, 0), 0.0);
    assert_eq!(world::experience::kill_xp(&order, false, 5), 20.0);
    assert_eq!(world::experience::kill_xp(&order, false, 40), 50.0);
    assert_eq!(world::experience::kill_xp(&order, true, 3), 5.0);
}

#[test]
fn someone_activating_a_chair_sits_in_it() {
    let (_data, order) = order("scripting-sit");
    let scripts = ScriptCache::default();
    let mut state = GameState::new(&order);
    Runner::new(&order, &scripts, &mut state).run_source("ChairRef.Activate DocRef", None, None);
    assert_eq!(
        state.furniture.get(&FormId(DOC_REF)),
        Some(&FormId(CHAIR_REF))
    );
    assert_eq!(ask(&order, &scripts, &mut state, "DocRef.GetSitting"), 3.0);
    // Not something that isn't furniture.
    Runner::new(&order, &scripts, &mut state).run_source("ChestRef.Activate DocRef", None, None);
    assert_eq!(
        state.furniture.get(&FormId(DOC_REF)),
        Some(&FormId(CHAIR_REF))
    );
}

#[test]
fn scripts_make_factions_enemies_and_the_aggressive_attack() {
    use world::factions::{self, Reaction};
    let (_data, order) = order("scripting-factions");
    let scripts = ScriptCache::default();
    let mut state = GameState::new(&order);
    let doc = FormId(DOC_REF);
    // In a faction with no relation to the player: aggressive, but
    // neutral, so he doesn't attack.
    assert!(factions::factions_of(&order, &state, doc).contains(&FormId(RAIDERS)));
    assert_eq!(factions::aggression(&order, doc), 1);
    assert_eq!(
        factions::reaction(&order, &state, doc, PLAYER_REF),
        Reaction::Neutral
    );
    assert!(!factions::attacks_on_sight(&order, &state, doc, PLAYER_REF));
    // A script makes the faction the player's enemy (as the game's do for
    // the Powder Gangers).
    Runner::new(&order, &scripts, &mut state).run_source(
        "SetEnemy TestRaiders PlayerFaction",
        None,
        None,
    );
    assert!(factions::attacks_on_sight(&order, &state, doc, PLAYER_REF));
    assert_eq!(
        ask(
            &order,
            &scripts,
            &mut state,
            "DocRef.GetFactionRelation player"
        ),
        1.0
    );
    // Allies again, and out of the faction.
    Runner::new(&order, &scripts, &mut state).run_source(
        "SetAlly TestRaiders PlayerFaction 1 0\nDocRef.RemoveFromFaction TestRaiders",
        None,
        None,
    );
    assert_eq!(
        factions::faction_reaction(&order, &state, FormId(RAIDERS), factions::PLAYER_FACTION),
        Reaction::Friend
    );
    assert_eq!(
        ask(
            &order,
            &scripts,
            &mut state,
            "DocRef.GetInFaction TestRaiders"
        ),
        0.0
    );
    Runner::new(&order, &scripts, &mut state).run_source(
        "player.AddToFaction TestRaiders 0",
        None,
        None,
    );
    assert_eq!(
        ask(
            &order,
            &scripts,
            &mut state,
            "player.GetInFaction TestRaiders"
        ),
        1.0
    );
    assert!(state.unhandled.is_empty(), "{:?}", state.unhandled_first);
}

#[test]
fn distances_are_known_within_one_place() {
    let (_data, order) = order("scripting-distance");
    let scripts = ScriptCache::default();
    let mut state = GameState::new(&order);
    state.player_cell = Some(FormId(CELL));
    state.player_position = Some([30.0, 40.0, 0.0]);
    Runner::new(&order, &scripts, &mut state).run_source(
        "set TestGlobal to Player.GetDistance DocRef",
        None,
        None,
    );
    assert_eq!(state.globals.get(&FormId(GLOBAL)), Some(&50.0));
    // Somewhere else: not known, so the script stops.
    state.player_cell = Some(FormId(0xFFFF));
    Runner::new(&order, &scripts, &mut state).run_source(
        "set TestGlobal to DocRef.GetDistance Player",
        None,
        None,
    );
    assert_eq!(state.globals.get(&FormId(GLOBAL)), Some(&50.0));
    assert_eq!(state.unhandled.get("GetDistance"), Some(&1));
}

#[test]
fn people_follow_the_package_their_conditions_pick_over_the_navmesh() {
    use world::ai::{current_package, destination, NavMesh};
    let (_data, order) = order("scripting-ai");
    let scripts = ScriptCache::default();
    let mut state = GameState::new(&order);
    let doc = FormId(DOC_REF);
    // Before stage 10 the travel package's condition fails.
    assert!(current_package(&order, &state, doc).is_none());
    Runner::new(&order, &scripts, &mut state).set_stage(FormId(QUEST), 10);
    let package = current_package(&order, &state, doc).unwrap();
    assert_eq!(package.form_id, FormId(TRAVEL));
    assert_eq!(package.kind, world::ai::kinds::TRAVEL);
    // To the door, at 100,0: within 20 (`00678670`: a reference without
    // a radius of its own, round(half its bounds' diagonal, none here) +
    // 20).
    let (to, radius) = destination(&order, &state, doc, &package).unwrap();
    assert_eq!((to, radius), ([100.0, 0.0, 0.0], 20.0));
    let mesh = NavMesh::load(&order, FormId(CELL));
    assert_eq!((mesh.vertices.len(), mesh.triangles.len()), (4, 2));
    let path = mesh.path([-150.0, 100.0, 0.0], to).unwrap();
    assert_eq!(path, vec![[-150.0, 100.0, 0.0], [100.0, 0.0, 0.0]]);
    // On its own, the east edge leads nowhere (its link counts into the
    // external connections, not this navmesh's triangles).
    assert_eq!(mesh.triangles[0].neighbors, [None, None, Some(1)]);
    // Joined with the next cell's, it leads into it, and back.
    let both = NavMesh::load_cells(&order, &[FormId(CELL), FormId(CELL2)]);
    assert_eq!(both.triangles.len(), 4);
    assert_eq!(both.triangles[0].neighbors[1], Some(3));
    assert_eq!(both.triangles[3].neighbors[2], Some(0));
    let across = both
        .path([-150.0, -150.0, 0.0], [500.0, 100.0, 0.0])
        .unwrap();
    assert_eq!(across.last(), Some(&[500.0, 100.0, 0.0]));
    // Somewhere off the navmesh: no path (not a straight line to it).
    assert!(mesh
        .path([-150.0, 100.0, 0.0], [5000.0, 0.0, 0.0])
        .is_none());
    // A package a script gives comes first.
    Runner::new(&order, &scripts, &mut state).run_source(
        "DocRef.RemoveScriptPackage\nDocRef.AddScriptPackage TestTravelPackage",
        None,
        None,
    );
    assert_eq!(state.script_packages.get(&doc), Some(&FormId(TRAVEL)));
}

#[test]
fn a_saved_game_loads_back_the_same() {
    use world::save::{load, save, PlayerPlace};
    let (_data, order) = order("scripting-save");
    let scripts = ScriptCache::default();
    let mut state = GameState::new(&order);
    let mut runner = Runner::new(&order, &scripts, &mut state);
    runner.set_stage(FormId(QUEST), 10);
    runner.run_event(FormId(DOC_REF), "onactivate", PLAYER_REF);
    runner.update(1.0);
    state.take_all(&order, FormId(CHEST_REF), PLAYER_REF);
    state.sit(FormId(DOC_REF), FormId(CHEST_REF));
    let place = PlayerPlace {
        cell: FormId(CELL),
        world: None,
        position: [1.5, -2.0, 3.25],
        heading: 0.5,
    };
    let text = save(&state, Some(place));
    let (back, where_) = load(&text).unwrap();
    assert_eq!(where_, Some(place));
    assert_eq!(back.stages, state.stages);
    assert_eq!(back.items, state.items);
    assert_eq!(back.variables[&FormId(DOC_REF)].get("iTalked"), Some(1.0));
    assert_eq!(back.furniture, state.furniture);
    // Saving what was loaded gives the same text.
    assert_eq!(save(&back, where_), text);
    assert!(load("something else").is_err());
}

#[test]
fn scripts_move_the_player() {
    let (_data, order) = order("scripting-moveto");
    let scripts = ScriptCache::default();
    let mut state = GameState::new(&order);
    Runner::new(&order, &scripts, &mut state).run_source(
        "Player.RemoveItem Caps001 1 1\nplayer.moveto DocRef\nset TestGlobal to 9",
        None,
        None,
    );
    assert!(
        state.events.contains(&Event::MoveTo {
            what: PLAYER_REF,
            to: FormId(DOC_REF)
        }),
        "{:?}",
        state.events
    );
    assert_eq!(state.globals.get(&FormId(GLOBAL)), Some(&9.0));
}

/// A script's answer: `expr` set into `TestGlobal`; -1 when the script
/// stopped.
fn ask(order: &LoadOrder, scripts: &ScriptCache, state: &mut GameState, expr: &str) -> f32 {
    state.globals.insert(FormId(GLOBAL), -1.0);
    Runner::new(order, scripts, state).run_source(&format!("set TestGlobal to {expr}"), None, None);
    state.globals[&FormId(GLOBAL)]
}

#[test]
fn scripts_know_where_things_are_markers_regions_and_packages() {
    let (_data, order) = order("scripting-places");
    let scripts = ScriptCache::default();
    let mut state = GameState::new(&order);
    // MoveTo puts the player where the doctor stands (plus an offset), in
    // his cell, and the door where the chest is.
    Runner::new(&order, &scripts, &mut state).run_source(
        "player.moveto DocRef 0 0 10\nDoorRef.moveto ChestRef",
        None,
        None,
    );
    assert_eq!(state.player_cell, Some(FormId(CELL)));
    assert_eq!(state.player_world, None);
    assert_eq!(state.player_position, Some([0.0, 0.0, 10.0]));
    assert_eq!(
        state.place(&order, FormId(DOOR_REF)).map(|p| p.2),
        Some([0.0, 100.0, 0.0])
    );
    let mut q = |expr: &str| ask(&order, &scripts, &mut state, expr);
    assert_eq!(q("player.GetInCell TestCell"), 1.0);
    assert_eq!(q("player.GetInSameCell DocRef"), 1.0);
    assert_eq!(q("player.GetDistance DocRef"), 10.0);
    assert_eq!(q("DoorRef.GetDistance ChestRef"), 0.0);
    // The cell lists the region.
    assert_eq!(q("IsPlayerInRegion TestRegion"), 1.0);
    // Map markers: shown from the start by their flag, or by ShowMap.
    assert_eq!(q("MarkerRef.GetMapMarkerVisible"), 1.0);
    assert_eq!(q("HiddenMarkerRef.GetMapMarkerVisible"), 0.0);
    q("ShowMap HiddenMarkerRef");
    assert_eq!(q("HiddenMarkerRef.GetMapMarkerVisible"), 1.0);
    // The doctor's travel package applies from stage 10.
    assert_eq!(q("DocRef.GetIsCurrentPackage TestTravelPackage"), 0.0);
    q("SetStage TestQuest 10");
    assert_eq!(q("DocRef.GetIsCurrentPackage TestTravelPackage"), 1.0);
    // Nobody's hurt: body parts are whole.
    assert_eq!(q("player.GetAV PerceptionCondition"), 100.0);
    q("SetQuestDelay TestQuest 0.5");
    assert_eq!(state.quest_delays.get(&FormId(QUEST)), Some(&0.5));
    assert!(state.unhandled.is_empty(), "{:?}", state.unhandled_first);
}

#[test]
fn message_boxes_ask_and_the_character_is_made() {
    use world::chargen::{self, CharacterMenu};
    let (_data, order) = order("scripting-chargen");
    let scripts = ScriptCache::default();
    let mut state = GameState::new(&order);
    // A message box: its values filled in, the buttons the player may
    // see, numbered as stored.
    Runner::new(&order, &scripts, &mut state).run_source(
        "ShowMessage TestChoice TestGlobal 50",
        None,
        None,
    );
    assert_eq!(
        state.events.pop(),
        Some(Event::Message {
            title: Some("Choose".into()),
            text: "You have 5 caps (50%).".into(),
            buttons: vec![(0, "First".into()), (2, "Third".into())],
            icon: None,
        })
    );
    // Nothing pressed yet: -1. Pressed: given once.
    assert_eq!(ask(&order, &scripts, &mut state, "GetButtonPressed"), -1.0);
    state.button = Some(2);
    assert_eq!(ask(&order, &scripts, &mut state, "GetButtonPressed"), 2.0);
    assert_eq!(ask(&order, &scripts, &mut state, "GetButtonPressed"), -1.0);

    // The player's sex, as `VCG01SCRIPT` sets it.
    assert_eq!(ask(&order, &scripts, &mut state, "GetPCIsSex Female"), 0.0);
    Runner::new(&order, &scripts, &mut state).run_source("player.SexChange female 1", None, None);
    assert_eq!(state.player_female, Some(true));
    assert_eq!(ask(&order, &scripts, &mut state, "GetPCIsSex Female"), 1.0);

    // The menus the opening opens.
    Runner::new(&order, &scripts, &mut state).run_source(
        "ShowLoveTesterMenuParams 40\nSetTagSkills 3 1\nShowTraitMenu\nGetPlayerName\nSetTagSkills 2 0\nSetTagSkills 1",
        None,
        None,
    );
    let menus: Vec<Event> = state.events.split_off(state.events.len() - 6);
    assert_eq!(
        menus,
        [
            CharacterMenu::Special { points: 40 },
            CharacterMenu::TagSkills {
                count: 3,
                preselect: true
            },
            CharacterMenu::Traits { max: 2 },
            CharacterMenu::Name,
            CharacterMenu::TagSkills {
                count: 2,
                preselect: false
            },
            CharacterMenu::TagSkills {
                count: 1,
                preselect: true
            },
        ]
        .map(Event::CharacterMenu)
    );
    // The psych exam tags by slot; a slot set again replaces its skill.
    Runner::new(&order, &scripts, &mut state).run_source(
        "SetPlayerTagSkill Guns 0\nSetPlayerTagSkill Speech 1\nSetPlayerTagSkill Unarmed 0",
        None,
        None,
    );
    assert_eq!(state.tag_skills, [43, 45].into());
    state.tag_skills.clear();
    state.tag_slots.clear();
    // Guns from SPECIAL: 2 + 2 × Agility + Luck / 2 rounded up, + 15
    // tagged.
    assert_eq!(ask(&order, &scripts, &mut state, "player.GetAV Guns"), 15.0);
    assert!(chargen::special_ok([6, 5, 5, 5, 5, 8, 6], 40));
    assert!(!chargen::special_ok([6, 5, 5, 5, 5, 8, 7], 40));
    assert!(!chargen::special_ok([11, 5, 5, 5, 5, 5, 4], 40));
    chargen::set_special(&mut state, [6, 5, 5, 5, 5, 8, 3]);
    assert_eq!(ask(&order, &scripts, &mut state, "player.GetAV Guns"), 20.0);
    state.tag_skills.insert(41);
    assert_eq!(ask(&order, &scripts, &mut state, "player.GetAV Guns"), 35.0);
    // Controls, as Doc Mitchell's intro locks them: everything but
    // looking, then movement and looking back.
    use world::scripting::controls;
    Runner::new(&order, &scripts, &mut state).run_source(
        "DisablePlayerControls 1 1 1 1 0 0 1",
        None,
        None,
    );
    assert_eq!(
        state.controls_off,
        [true, true, true, true, false, false, true]
    );
    assert_eq!(
        ask(&order, &scripts, &mut state, "GetPlayerControlsDisabled"),
        1.0
    );
    Runner::new(&order, &scripts, &mut state).run_source(
        "EnablePlayerControls 1 0 0 1 1 1 0",
        None,
        None,
    );
    assert!(!state.controls_off[controls::MOVEMENT] && state.controls_off[controls::PIPBOY]);
    Runner::new(&order, &scripts, &mut state).run_source("EnablePlayerControls", None, None);
    assert_eq!(state.controls_off, [false; 7]);
    // Traits: playable perks flagged as traits.
    let traits = chargen::traits(&order);
    assert_eq!(traits.len(), 1);
    assert_eq!(traits[0].form_id, FormId(TRAIT));
    assert_eq!(chargen::player_name(&order, &state), "Courier");
}

#[test]
fn image_space_modifiers_play_their_keys_over_their_duration() {
    use world::modifier::{track, Modifier};
    let (_data, order) = order("scripting-imad");
    let scripts = ScriptCache::default();
    let mut state = GameState::new(&order);
    Runner::new(&order, &scripts, &mut state).run_source(
        "ApplyImageSpaceModifier TestFlash",
        None,
        None,
    );
    assert_eq!(state.modifiers, [FormId(FLASH)]);
    let m = Modifier::load(&order, FormId(FLASH)).unwrap();
    assert!(m.animatable && m.duration == 2.0);
    // Keys run over the duration: half a second in is a quarter through.
    let start = m.at(0.0);
    assert_eq!(start.fade, [1.0, 1.0, 1.0, 1.0]);
    let quarter = m.at(0.5);
    assert!((quarter.fade[3] - 0.5).abs() < 1e-6);
    assert!((quarter.multiply[track::BRIGHT_SCALE] - 1.75).abs() < 1e-6);
    assert!((quarter.add[track::BRIGHT_SCALE] - 0.125).abs() < 1e-6);
    // Untouched values stay as they are.
    assert_eq!(quarter.multiply[track::BRIGHT_CLAMP], 1.0);
    assert_eq!(m.at(1.5).fade[3], 0.0);
    assert!(!m.finished(1.9) && m.finished(2.0));
    Runner::new(&order, &scripts, &mut state).run_source(
        "RemoveImageSpaceModifier TestFlash",
        None,
        None,
    );
    assert!(state.modifiers.is_empty());
}

#[test]
fn a_function_that_isnt_carried_out_stops_the_script_instead_of_guessing() {
    let (_data, order) = order("scripting-unhandled");
    let scripts = ScriptCache::default();
    let mut state = GameState::new(&order);
    let mut runner = Runner::new(&order, &scripts, &mut state);
    // The player is nowhere yet, so the distance isn't known.
    runner.run_source(
        "if Player.GetDistance DocRef < 100\n\tSetStage TestQuest 10\nendif",
        None,
        None,
    );
    assert!(state.stages.is_empty());
    assert_eq!(state.unhandled.get("GetDistance"), Some(&1));
}

#[test]
fn play_group_has_an_object_play_its_models_sequence() {
    // A house window's script turning its glow off by day (as
    // `WastelandWindowScript` does): the object plays "Right".
    let (_data, order) = order("scripting-playgroup");
    let scripts = ScriptCache::default();
    let mut state = GameState::new(&order);
    Runner::new(&order, &scripts, &mut state).run_source(
        "PlayGroup Right 0",
        Some(FormId(DOOR_REF)),
        Some(FormId(DOOR_REF)),
    );
    assert_eq!(
        std::mem::take(&mut state.events),
        vec![Event::PlayGroup {
            who: FormId(DOOR_REF),
            group: "Right".into(),
            flags: 0,
        }]
    );
}

#[test]
fn play_bink_gives_the_movie_and_its_flags_with_the_games_defaults() {
    use world::scripting::Video;
    let (_data, order) = order("scripting-playbink");
    let scripts = ScriptCache::default();
    let mut state = GameState::new(&order);
    let mut play = |line: &str| {
        Runner::new(&order, &scripts, &mut state).run_source(line, None, None);
        state.events.pop()
    };
    // The opening quest's own call.
    assert_eq!(
        play("PlayBink \"FNVIntro.bik\" 1 1 0 1"),
        Some(Event::Video(Video {
            file: "FNVIntro.bik".into(),
            interruptable: true,
            mute_audio: true,
            pause_music: false,
            letterbox: true,
        }))
    );
    // Left out: the handler's defaults 0, 1, 1, 1.
    assert_eq!(
        play("PlayBink \"Other.bik\""),
        Some(Event::Video(Video {
            file: "Other.bik".into(),
            interruptable: false,
            mute_audio: true,
            pause_music: true,
            letterbox: true,
        }))
    );
    assert_eq!(
        play("PlayBink \"Other.bik\" 0 0"),
        Some(Event::Video(Video {
            file: "Other.bik".into(),
            interruptable: false,
            mute_audio: false,
            pause_music: true,
            letterbox: true,
        }))
    );
}

/// Scripted items see their `OnAdd` on the holder's next script run
/// (`00574fa0` flags it, `004d2480` runs it), each item its own copy of
/// the script: `TestCaseBundle`'s `OnAdd Player` gives 25 cases and
/// `RemoveMe` takes the bundle away (as the game's `Case10mmAddScript`).
#[test]
fn scripted_items_see_their_onadd() {
    let (_data, order) = order("scripting-onadd");
    let scripts = ScriptCache::default();
    let mut state = GameState::new(&order);
    let (bundle, case) = (FormId(CASE_BUNDLE), FormId(CASE));
    Runner::new(&order, &scripts, &mut state).run_source(
        "player.AddItem TestCaseBundle 2",
        None,
        None,
    );
    assert_eq!(state.item_count(&order, PLAYER_REF, bundle), 2);
    Runner::new(&order, &scripts, &mut state).update(0.1);
    assert_eq!(state.item_count(&order, PLAYER_REF, bundle), 0);
    assert_eq!(state.item_count(&order, PLAYER_REF, case), 50);
    // In a chest the block (`OnAdd Player`) doesn't run.
    let chest = FormId(CHEST_REF);
    Runner::new(&order, &scripts, &mut state).run_source(
        "ChestRef.AddItem TestCaseBundle 1",
        None,
        None,
    );
    Runner::new(&order, &scripts, &mut state).update(0.1);
    assert_eq!(state.item_count(&order, chest, bundle), 1);
    // Taken from it, it runs for the player.
    state.move_item(&order, chest, PLAYER_REF, bundle, 1);
    Runner::new(&order, &scripts, &mut state).update(0.1);
    assert_eq!(state.item_count(&order, PLAYER_REF, case), 75);
    assert_eq!(state.item_count(&order, PLAYER_REF, bundle), 0);
    // `RemoveMe` outside an item's own run does nothing.
    Runner::new(&order, &scripts, &mut state).run_source("RemoveMe", None, None);
    assert!(state.unhandled.is_empty(), "{:?}", state.unhandled_first);
    // Each item keeps its own variables: `OnAdd` marks it, the same run's
    // `GameMode` (later in the script) gives 5 cases and removes it.
    Runner::new(&order, &scripts, &mut state).run_source(
        "player.AddItem TestLateBundle 1",
        None,
        None,
    );
    Runner::new(&order, &scripts, &mut state).update(0.1);
    assert_eq!(state.item_count(&order, PLAYER_REF, FormId(LATE_BUNDLE)), 0);
    assert_eq!(state.item_count(&order, PLAYER_REF, case), 80);
    assert!(state.item_scripts.is_empty());
    // `OnEquip` and `OnUnequip` (the faction outfits' warnings).
    let global = |state: &mut GameState| state.globals.get(&FormId(GLOBAL)).copied();
    Runner::new(&order, &scripts, &mut state).run_source(
        "player.AddItem TestScriptedHat 1",
        None,
        None,
    );
    Runner::new(&order, &scripts, &mut state).update(0.1);
    Runner::new(&order, &scripts, &mut state).run_source(
        "player.EquipItem TestScriptedHat",
        None,
        None,
    );
    Runner::new(&order, &scripts, &mut state).update(0.1);
    assert_eq!(global(&mut state), Some(99.0));
    Runner::new(&order, &scripts, &mut state).run_source(
        "player.UnequipItem TestScriptedHat",
        None,
        None,
    );
    Runner::new(&order, &scripts, &mut state).update(0.1);
    assert_eq!(global(&mut state), Some(7.0));
}

/// A quest with its own delay runs at once, and on that first run
/// `GetSecondsPassed` is the frame's time (`005ab400`, `005ac1e0`,
/// `0059c430`); afterwards the time gathered since the last run. Doc
/// Mitchell's farewell timer (`VGenericTimer`, delay 0.1, `fTimer` 0.1)
/// counts down over its second and third runs this way.
#[test]
fn an_own_delay_quests_first_run_counts_the_frame() {
    use testdata::{group, record, sub, zstr};
    let data = testdata::TempData::empty("quest-first-run");
    let mut script = sub(b"EDID", &zstr("TestTimerScript"));
    script.extend(sub(b"SCHR", &[0; 20]));
    script.extend(sub(
        b"SCTX",
        b"scn TestTimerScript\nfloat fSeen\nshort nRuns\nBegin GameMode\nset fSeen to GetSecondsPassed\nset nRuns to nRuns + 1\nEnd",
    ));
    let mut quest = sub(b"EDID", &zstr("TestTimer"));
    quest.extend(sub(b"SCRI", &0x900u32.to_le_bytes()));
    let mut qdata = vec![0x01, 50, 0, 0];
    qdata.extend(0.1f32.to_le_bytes());
    quest.extend(sub(b"DATA", &qdata));
    let mut hedr = 1.34f32.to_le_bytes().to_vec();
    hedr.extend([0; 8]);
    let mut plugin = record(b"TES4", 0, &sub(b"HEDR", &hedr));
    plugin.extend(group(*b"SCPT", 0, &record(b"SCPT", 0x900, &script)));
    plugin.extend(group(*b"QUST", 0, &record(b"QUST", 0x901, &quest)));
    data.write("FalloutNV.esm", &plugin);
    let order = LoadOrder::from_data_dir(data.path(), &ActivePlugins::OfficialOnly).unwrap();
    let scripts = ScriptCache::default();
    let mut state = GameState::new(&order);
    let var = |state: &GameState, name: &str| {
        state
            .variables
            .get(&FormId(0x901))
            .and_then(|l| l.iter().find(|v| v.0 == name).map(|v| v.2))
            .unwrap_or(0.0)
    };
    Runner::new(&order, &scripts, &mut state).update(0.0625);
    assert_eq!(var(&state, "nruns"), 1.0);
    assert_eq!(var(&state, "fseen"), 0.0625);
    // Not again until 0.1 s have gathered; then all of it.
    Runner::new(&order, &scripts, &mut state).update(0.0625);
    assert_eq!(var(&state, "nruns"), 1.0);
    Runner::new(&order, &scripts, &mut state).update(0.0625);
    assert_eq!(var(&state, "nruns"), 2.0);
    assert_eq!(var(&state, "fseen"), 0.125);
}

/// `RemoveMe` on an actor wearing the item (`005b53d0` → `00575400`,
/// `004bfda0`): the worn one goes, taken off first, and the one left isn't
/// worn. `TestVanishingHat`'s `OnEquip Player` removes it; with two, the
/// player puts one on and is left with the other in their bag.
#[test]
fn removeme_takes_the_worn_one() {
    let (_data, order) = order("scripting-removeme-worn");
    let scripts = ScriptCache::default();
    let mut state = GameState::new(&order);
    let hat = FormId(VANISHING_HAT);
    Runner::new(&order, &scripts, &mut state).run_source(
        "player.AddItem TestVanishingHat 2",
        None,
        None,
    );
    Runner::new(&order, &scripts, &mut state).update(0.1);
    Runner::new(&order, &scripts, &mut state).run_source(
        "player.EquipItem TestVanishingHat",
        None,
        None,
    );
    assert!(state.is_equipped(PLAYER_REF, hat));
    Runner::new(&order, &scripts, &mut state).update(0.1);
    assert_eq!(state.item_count(&order, PLAYER_REF, hat), 1);
    assert!(!state.is_equipped(PLAYER_REF, hat));
    // The other one's script is still there; the worn one's went with it.
    assert_eq!(
        state
            .item_scripts
            .iter()
            .filter(|s| s.holder == PLAYER_REF && s.item == hat)
            .count(),
        1
    );
    assert!(state.unhandled.is_empty(), "{:?}", state.unhandled_first);
}

/// A quest script that sets a stage from its `GameMode` block runs that
/// stage's result script with the quest's own variables, and what it sets
/// stays (the game keeps one copy of a script's variables). `VCG01SCRIPT`
/// does this at the start of the opening: `setstage VCG01 1` runs stage 1's
/// `set VCG01.fTimer to 2.8`, the 2.8 s before Doc speaks. Keeping the
/// block's old copy of `fTimer` lost it (Doc spoke 0.2 s in, not 3 s).
#[test]
fn a_stage_set_from_the_quests_script_keeps_the_variables_it_sets() {
    use testdata::{group, record, sub, zstr};
    let data = testdata::TempData::empty("quest-stage-vars");
    let mut script = sub(b"EDID", &zstr("TestStageVarsScript"));
    script.extend(sub(b"SCHR", &[0; 20]));
    script.extend(sub(
        b"SCTX",
        b"scn TestStageVarsScript\nfloat fTimer\nshort nStep\nBegin GameMode\nif nStep == 0\nset fTimer to 0\nset nStep to 1\nelseif fTimer > 0\nset fTimer to fTimer - GetSecondsPassed\nelse\nsetstage TestStageVars 1\nendif\nEnd",
    ));
    let mut quest = sub(b"EDID", &zstr("TestStageVars"));
    quest.extend(sub(b"SCRI", &0x900u32.to_le_bytes()));
    let mut qdata = vec![0x01, 50, 0, 0];
    qdata.extend(0.1f32.to_le_bytes());
    quest.extend(sub(b"DATA", &qdata));
    quest.extend(sub(b"INDX", &1u16.to_le_bytes()));
    quest.extend(sub(b"QSDT", &[0]));
    quest.extend(sub(b"SCHR", &[0; 20]));
    quest.extend(sub(b"SCTX", b"set TestStageVars.fTimer to 2.8"));
    let mut hedr = 1.34f32.to_le_bytes().to_vec();
    hedr.extend([0; 8]);
    let mut plugin = record(b"TES4", 0, &sub(b"HEDR", &hedr));
    plugin.extend(group(*b"SCPT", 0, &record(b"SCPT", 0x900, &script)));
    plugin.extend(group(*b"QUST", 0, &record(b"QUST", 0x901, &quest)));
    data.write("FalloutNV.esm", &plugin);
    let order = LoadOrder::from_data_dir(data.path(), &ActivePlugins::OfficialOnly).unwrap();
    let scripts = ScriptCache::default();
    let mut state = GameState::new(&order);
    state.running.insert(FormId(0x901));
    let timer = |state: &GameState| {
        state
            .variables
            .get(&FormId(0x901))
            .and_then(|l| l.iter().find(|v| v.0 == "ftimer").map(|v| v.2))
            .unwrap_or(0.0)
    };
    // The first run sets the step; the next sees no timer and sets stage 1,
    // whose script sets the timer, which the block mustn't overwrite.
    for _ in 0..4 {
        Runner::new(&order, &scripts, &mut state).update(0.1);
    }
    assert_eq!(state.stages.get(&FormId(0x901)), Some(&1));
    assert!(
        timer(&state) > 2.0 && timer(&state) <= 2.8,
        "fTimer is {}",
        timer(&state)
    );
}

/// Another runner of quest scripts ([`world::scripting::QuestVm`], the
/// game's own interpreter in the viewer): a quest it takes runs through it,
/// with the quest's variables and the frame's `GetSecondsPassed`, and the
/// world's interpreter doesn't run it; a quest it declines runs as before.
#[test]
fn a_quest_vm_runs_the_quests_it_takes_and_declines_the_rest() {
    use script::interp::Locals;
    use testdata::{group, record, sub, zstr};
    use world::scripting::QuestVm;

    struct Fake {
        take: bool,
        seen: Vec<(FormId, FormId, f32)>,
    }
    impl QuestVm for Fake {
        fn run_quest(
            &mut self,
            _order: &LoadOrder,
            quest: FormId,
            script: FormId,
            parsed: &script::Script,
            locals: &mut Locals,
            seconds_passed: f32,
        ) -> bool {
            assert!(parsed.blocks.iter().any(|b| b.kind == "gamemode"));
            self.seen.push((quest, script, seconds_passed));
            if !self.take {
                return false;
            }
            let runs = locals.get("nRuns").unwrap_or(0.0);
            locals.set("nRuns", runs + 10.0);
            locals.set("fSeen", f64::from(seconds_passed) * 100.0);
            true
        }
    }

    let data = testdata::TempData::empty("quest-vm");
    let mut script = sub(b"EDID", &zstr("TestVmScript"));
    script.extend(sub(b"SCHR", &[0; 20]));
    script.extend(sub(
        b"SCTX",
        b"scn TestVmScript\nfloat fSeen\nshort nRuns\nBegin GameMode\nset fSeen to GetSecondsPassed\nset nRuns to nRuns + 1\nEnd",
    ));
    let mut quest = sub(b"EDID", &zstr("TestVm"));
    quest.extend(sub(b"SCRI", &0x900u32.to_le_bytes()));
    let mut qdata = vec![0x01, 50, 0, 0];
    qdata.extend(0.1f32.to_le_bytes());
    quest.extend(sub(b"DATA", &qdata));
    let mut hedr = 1.34f32.to_le_bytes().to_vec();
    hedr.extend([0; 8]);
    let mut plugin = record(b"TES4", 0, &sub(b"HEDR", &hedr));
    plugin.extend(group(*b"SCPT", 0, &record(b"SCPT", 0x900, &script)));
    plugin.extend(group(*b"QUST", 0, &record(b"QUST", 0x901, &quest)));
    data.write("FalloutNV.esm", &plugin);
    let order = LoadOrder::from_data_dir(data.path(), &ActivePlugins::OfficialOnly).unwrap();
    let scripts = ScriptCache::default();
    let var = |state: &GameState, name: &str| {
        state
            .variables
            .get(&FormId(0x901))
            .and_then(|l| l.iter().find(|v| v.0 == name).map(|v| v.2))
            .unwrap_or(0.0)
    };

    // Taken: the hook's writes are the quest's variables; the world's
    // interpreter (which would add 1) doesn't run.
    let mut state = GameState::new(&order);
    let mut vm = Fake {
        take: true,
        seen: vec![],
    };
    Runner::new(&order, &scripts, &mut state)
        .with_quest_vm(&mut vm)
        .update(0.0625);
    assert_eq!(vm.seen, [(FormId(0x901), FormId(0x900), 0.0625)]);
    assert_eq!(var(&state, "nruns"), 10.0);
    assert_eq!(var(&state, "fseen"), 6.25);
    // The world's own delay still decides when it runs; the second run
    // gets the time gathered since the first, and starts from its values.
    Runner::new(&order, &scripts, &mut state)
        .with_quest_vm(&mut vm)
        .update(0.0625);
    assert_eq!(vm.seen.len(), 1);
    Runner::new(&order, &scripts, &mut state)
        .with_quest_vm(&mut vm)
        .update(0.0625);
    assert_eq!(vm.seen[1], (FormId(0x901), FormId(0x900), 0.125));
    assert_eq!(var(&state, "nruns"), 20.0);

    // Declined: the world's interpreter runs it, as without a hook.
    let mut state = GameState::new(&order);
    let mut vm = Fake {
        take: false,
        seen: vec![],
    };
    Runner::new(&order, &scripts, &mut state)
        .with_quest_vm(&mut vm)
        .update(0.0625);
    assert_eq!(vm.seen.len(), 1);
    assert_eq!(var(&state, "nruns"), 1.0);
    assert_eq!(var(&state, "fseen"), 0.0625);
}

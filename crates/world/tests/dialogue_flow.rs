//! The dialogue menu's flow after a line (`00762ff0`, `00762860`,
//! `0061af30`, `0061a7d0`, `0061e600`), on generated data shaped like
//! Doc Mitchell's farewell at his front door (`testdata::dialogue`).

use esm::{ActivePlugins, FormId, LoadOrder};
use testdata::dialogue::ids::*;
use world::dialogue::{self, AfterLine, Ending, Speaker, PLAYER_REF};
use world::scripting::{GameState, Runner, ScriptCache};

fn order(tag: &str) -> (testdata::TempData, LoadOrder) {
    let data = testdata::dialogue::world(tag);
    let order = LoadOrder::from_data_dir(data.path(), &ActivePlugins::OfficialOnly).unwrap();
    (data, order)
}

fn doc(order: &LoadOrder) -> Speaker {
    // The doctor's own reference isn't placed here; conditions ask the base.
    Speaker::load(order, FormId(0x1299), FormId(DOC)).unwrap()
}

fn info(order: &LoadOrder, id: u32) -> dialogue::Info {
    dialogue::Info::load(order, FormId(id)).unwrap()
}

/// The farewell runs through its follow-ups to the Goodbye line, whose
/// result scripts run and which closes the menu: the player can leave.
#[test]
fn the_farewell_follows_up_to_a_goodbye_that_closes_the_menu() {
    let (_data, order) = order("dialogue-flow-farewell");
    let mut state = GameState::new(&order);
    let doc = doc(&order);
    let top = dialogue::top_level_topics(&order);

    let welcome = dialogue::pick(&order, FormId(GREETING), &doc, &state).unwrap();
    assert_eq!(welcome.form_id, FormId(WELCOME));
    assert_eq!(welcome.follow_ups, [FormId(GIFT)]);
    dialogue::line_begins(&mut state, &welcome, doc.reference);

    // The greeting has no topics of its own: the speaker goes straight on.
    let AfterLine::FollowUp(gift) = dialogue::after_line(&order, &welcome, &top, &doc, &state)
    else {
        panic!("the greeting should go on to the gift");
    };
    assert_eq!(gift.form_id, FormId(GIFT));
    dialogue::line_begins(&mut state, &gift, doc.reference);

    // Of the gift's two follow-ups, the one whose condition passes.
    let AfterLine::FollowUp(middle) = dialogue::after_line(&order, &gift, &top, &doc, &state)
    else {
        panic!("the gift should go on");
    };
    assert_eq!(middle.form_id, FormId(MIDDLE_B));
    state.globals.insert(FormId(SWITCH), 1.0);
    assert_eq!(
        dialogue::follow_up(&order, &gift, &doc, &state).map(|i| i.form_id),
        Some(FormId(MIDDLE_A))
    );
    state.globals.insert(FormId(SWITCH), 0.0);

    // The middle offers its topic.
    let AfterLine::Topics(list) = dialogue::after_line(&order, &middle, &top, &doc, &state) else {
        panic!("the middle line should offer topics");
    };
    let topics: Vec<u32> = list.iter().map(|c| c.topic.form_id.0).collect();
    assert_eq!(topics, [THANKS]);
    let thanks = list[0].info.clone();
    assert_eq!(thanks.form_id, FormId(THANKS_LINE));

    // The answer goes on to the Goodbye line.
    let AfterLine::FollowUp(bye) = dialogue::after_line(&order, &thanks, &top, &doc, &state) else {
        panic!("the answer should go on to the goodbye");
    };
    assert_eq!(bye.form_id, FormId(BYE));
    assert_eq!(dialogue::ending(&bye), Ending::GoodbyeLine);
    // Its scripts run (the door opens: here a global), then the menu
    // closes.
    let scripts = ScriptCache::default();
    let mut runner = Runner::new(&order, &scripts, &mut state);
    runner.run_source(
        bye.begin_script.as_deref().unwrap(),
        Some(doc.reference),
        None,
    );
    assert_eq!(state.globals.get(&FormId(DOOR_OPEN)), Some(&1.0));
    assert!(dialogue::menu_runs_end_script(&bye));
    let mut runner = Runner::new(&order, &scripts, &mut state);
    runner.run_source(
        bye.end_script.as_deref().unwrap(),
        Some(doc.reference),
        None,
    );
    assert_eq!(state.globals.get(&FormId(DOOR_OPEN)), Some(&2.0));
    assert_eq!(
        dialogue::after_line(&order, &bye, &top, &doc, &state),
        AfterLine::Close
    );

    // The greeting was said once: it isn't picked again.
    assert!(dialogue::pick(&order, FormId(GREETING), &doc, &state).is_none());
}

/// Activating someone (`005fa330`): a greeting flagged Goodbye with a
/// single response is only said; anything else opens the dialogue menu.
#[test]
fn a_one_response_goodbye_greeting_is_said_without_the_menu() {
    let (_data, order) = order("dialogue-flow-activation");
    let bye = info(&order, BYE);
    assert_eq!(bye.responses.len(), 1);
    assert!(dialogue::activation_says_a_line(&bye));
    // The doctor's ordinary greeting opens the menu.
    assert!(!dialogue::activation_says_a_line(&info(&order, WELCOME)));
    // A Goodbye line of two responses opens it too (`0083c7e0` finds a
    // next response).
    let mut two = bye.clone();
    two.responses.push(two.responses[0].clone());
    assert!(!dialogue::activation_says_a_line(&two));
}

/// A Goodbye line takes no follow-up (`00762ff0` skips it for `+0x2c` 2);
/// a line answering `GOODBYE` without the flag closes the menu after its
/// follow-ups are looked for; a "run immediately" line takes none.
#[test]
fn goodbye_states_and_lines_without_follow_ups() {
    let (_data, order) = order("dialogue-flow-goodbye");
    let state = GameState::new(&order);
    let doc = doc(&order);
    let top = dialogue::top_level_topics(&order);

    let mut bye = info(&order, BYE);
    bye.follow_ups = vec![FormId(GIFT)];
    assert_eq!(
        dialogue::after_line(&order, &bye, &top, &doc, &state),
        AfterLine::Close
    );

    let see_you = dialogue::pick(&order, FormId(GOODBYE), &doc, &state).unwrap();
    assert_eq!(dialogue::ending(&see_you), Ending::GoodbyeTopic);
    assert_eq!(
        dialogue::after_line(&order, &see_you, &top, &doc, &state),
        AfterLine::Close
    );
    let mut see_you_more = see_you.clone();
    see_you_more.follow_ups = vec![FormId(GIFT)];
    assert!(matches!(
        dialogue::after_line(&order, &see_you_more, &top, &doc, &state),
        AfterLine::FollowUp(i) if i.form_id == FormId(GIFT)
    ));

    let immediate = info(&order, IMMEDIATE_LINE);
    assert!(!dialogue::menu_runs_end_script(&immediate));
    assert!(dialogue::menu_runs_begin_script(&immediate));
    // No follow-up, no topics: the menu closes ("invalid choice list").
    assert_eq!(
        dialogue::after_line(&order, &immediate, &top, &doc, &state),
        AfterLine::Close
    );
    assert_eq!(dialogue::ending(&immediate), Ending::Continue);
}

/// A run of random lines is chosen among (`rand() % count`), up to the
/// random-end line; a plain line after it isn't in the draw.
#[test]
fn random_runs_are_chosen_among() {
    let (_data, order) = order("dialogue-flow-random");
    let mut state = GameState::new(&order);
    let doc = doc(&order);
    let mut seen = std::collections::BTreeSet::new();
    for dice in 0..8 {
        state.dice = dice;
        let line = dialogue::pick(&order, FormId(RANDOM_TOPIC), &doc, &state).unwrap();
        seen.insert(line.form_id.0);
    }
    assert_eq!(seen.into_iter().collect::<Vec<_>>(), [RANDOM_1, RANDOM_2]);
    // Lines all said once leave the plain one.
    let mut lines = dialogue::topic_lines(&order, FormId(RANDOM_TOPIC));
    for l in &mut lines[..2] {
        l.flags |= dialogue::SAY_ONCE;
    }
    state.said.insert(FormId(RANDOM_1));
    state.said.insert(FormId(RANDOM_2));
    let left: Vec<_> = lines
        .into_iter()
        .filter(|i| dialogue::line_available(&order, i, &doc, PLAYER_REF, &state))
        .collect();
    assert_eq!(
        dialogue::choose(left, 5).map(|i| i.form_id),
        Some(FormId(PLAIN))
    );
}

/// The main list is the player's topics (`0083ec30`, `0083ed50`): every
/// top-level topic whatever its kind, without the hard-coded
/// `SpeechChallengeFailure` and `InfoRefusal`, and only the topics of the
/// player's Intelligence class, highest priority first.
#[test]
fn the_main_list_is_the_players_topics() {
    let (_data, order) = order("dialogue-flow-main-list");
    let mut state = GameState::new(&order);
    let doc = doc(&order);
    let top = dialogue::top_level_topics(&order);
    state.globals.insert(FormId(LIST_ON), 1.0);
    let list = |state: &GameState| -> Vec<u32> {
        dialogue::menu_topics(&order, &top, &doc, state)
            .into_iter()
            .map(|c| c.topic.form_id.0)
            .collect()
    };
    state.actor_values.insert((PLAYER_REF, 9), 3.0);
    assert_eq!(list(&state), [DUMB_ASK, ASK, CHAT]);
    state.actor_values.insert((PLAYER_REF, 9), 6.0);
    assert_eq!(list(&state), [SMART_ASK, ASK, CHAT]);
    // A line without choices of its own leads there.
    let middle = info(&order, IMMEDIATE_LINE);
    let AfterLine::Topics(after) = dialogue::after_line(&order, &middle, &top, &doc, &state) else {
        panic!("the main list should follow");
    };
    assert_eq!(after.len(), 3);
}

/// Low-Intelligence lines only for a player at or below
/// `iDialogueDummySpeakThisIntOrBelow` (3 here), high ones only above.
#[test]
fn lines_suit_the_players_intelligence() {
    let (_data, order) = order("dialogue-flow-smart");
    let mut state = GameState::new(&order);
    let doc = doc(&order);
    state.actor_values.insert((PLAYER_REF, 9), 3.0);
    let line = dialogue::pick(&order, FormId(SMART_TOPIC), &doc, &state).unwrap();
    assert_eq!(line.form_id, FormId(DUMB_LINE));
    state.actor_values.insert((PLAYER_REF, 9), 4.0);
    let line = dialogue::pick(&order, FormId(SMART_TOPIC), &doc, &state).unwrap();
    assert_eq!(line.form_id, FormId(SMART_LINE));
}

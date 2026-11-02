//! Reputation, karma, crime and faction armour disguises
//! (`world::reputation`, `world::crime`, `world::factions`) on
//! `testdata::factions`'s world, with `FalloutNV.esm`'s values.

use esm::{ActivePlugins, FormId, LoadOrder};
use testdata::factions::ids::*;
use world::dialogue::PLAYER_REF;
use world::factions::{reaction, Reaction};
use world::reputation::{self, FAME, INFAMY};
use world::scripting::{Event, GameState, Runner, ScriptCache};

fn order(tag: &str) -> (testdata::TempData, LoadOrder) {
    let data = testdata::factions::factions(tag);
    let order = LoadOrder::from_data_dir(data.path(), &ActivePlugins::OfficialOnly).unwrap();
    (data, order)
}

/// A new game with the player in `TestOutpost`, north of its people (who
/// face north).
fn at_outpost(order: &LoadOrder) -> GameState {
    let mut state = GameState::new(order);
    state.player_cell = Some(FormId(OUTPOST));
    state.player_world = None;
    state.player_position = Some([0.0, 100.0, 0.0]);
    state
}

fn run(order: &LoadOrder, scripts: &ScriptCache, state: &mut GameState, source: &str) {
    Runner::new(order, scripts, state).run_source(source, None, None);
}

/// An expression's value, as a script sets a global to it.
fn ask(order: &LoadOrder, scripts: &ScriptCache, state: &mut GameState, expr: &str) -> f32 {
    state.globals.insert(FormId(VALUE), -12345.0);
    run(order, scripts, state, &format!("set TestValue to {expr}"));
    state.globals[&FormId(VALUE)]
}

/// The notices queued since last asked (messages without buttons).
fn notices(state: &mut GameState) -> Vec<String> {
    let mut out = Vec::new();
    state.events.retain(|e| match e {
        Event::Message { text, buttons, .. } if buttons.is_empty() => {
            out.push(text.clone());
            false
        }
        _ => true,
    });
    out
}

/// The game's own boxes queued since last asked: (title, text, icon, sound).
fn popups(state: &mut GameState) -> Vec<(Option<String>, String, String, String)> {
    let mut out = Vec::new();
    state.events.retain(|e| match e {
        Event::Popup {
            title,
            text,
            icon,
            sound,
        } => {
            out.push((
                title.clone(),
                text.clone(),
                icon.clone().unwrap_or_default(),
                sound.clone().unwrap_or_default(),
            ));
            false
        }
        _ => true,
    });
    out
}

#[test]
fn reputation_moves_and_titles_show_as_the_game_shows_them() {
    let (_data, order) = order("factions-reputation");
    let scripts = ScriptCache::default();
    let mut state = at_outpost(&order);
    let ncr = FormId(REP_NCR);
    // An average bump (4) of the NCR's 80: 0.05, still no level. The
    // notice is the reputation's name and the data's words.
    run(&order, &scripts, &mut state, "AddReputation RepNVNCR 1 3");
    assert_eq!(reputation::get(&state, ncr, FAME), 4.0);
    assert_eq!(notices(&mut state), ["NCR\nFame Gained!"]);
    assert!(popups(&mut state).is_empty());
    // 12 of 80 = 0.15: level 1, the title box ("Accepted"), titled with
    // the reputation, the exe's description and icon, the good sound.
    run(
        &order,
        &scripts,
        &mut state,
        "AddReputationExact RepNVNCR 1 8",
    );
    assert_eq!(
        popups(&mut state),
        [(
            Some("NCR".to_string()),
            "Accepted\nFolks have come to accept you for your helpful nature.".to_string(),
            "Interface\\Icons\\Message Icons\\glow_message_vaultboy_neutral.dds".to_string(),
            "UIRepGood".to_string()
        )]
    );
    assert_eq!(
        ask(
            &order,
            &scripts,
            &mut state,
            "GetReputationThreshold RepNVNCR 1"
        ),
        4.0
    );
    // Adding stops at the most (`00615730`), with level 3.
    notices(&mut state);
    run(
        &order,
        &scripts,
        &mut state,
        "AddReputationExact RepNVNCR 1 200",
    );
    assert_eq!(reputation::get(&state, ncr, FAME), 80.0);
    assert_eq!(popups(&mut state)[0].1.lines().next(), Some("Idolized"));
    notices(&mut state);
    // A very minor removal (1): 79 = 0.9875, level 2 ("Liked"), the
    // notice's words; a bad change, so the bad sound.
    run(
        &order,
        &scripts,
        &mut state,
        "RemoveReputation RepNVNCR 1 1",
    );
    assert_eq!(reputation::get(&state, ncr, FAME), 79.0);
    assert_eq!(notices(&mut state), ["NCR\nFame Reduced"]);
    let p = popups(&mut state);
    assert_eq!(p[0].1.lines().next(), Some("Liked"));
    assert_eq!(p[0].3, "UIRepBad");
    // Removing stops at 0 (`00615a00`).
    run(
        &order,
        &scripts,
        &mut state,
        "RemoveReputationExact RepNVNCR 1 500",
    );
    assert_eq!(reputation::get(&state, ncr, FAME), 0.0);
    // Adding has no floor and removing no ceiling: a negative exact
    // addition still says "gained".
    notices(&mut state);
    run(
        &order,
        &scripts,
        &mut state,
        "AddReputationExact RepNVNCR 1 -5",
    );
    assert_eq!(reputation::get(&state, ncr, FAME), -5.0);
    assert_eq!(notices(&mut state), ["NCR\nFame Gained!"]);
    run(
        &order,
        &scripts,
        &mut state,
        "SetReputation RepNVNCR 1 120\nRemoveReputation RepNVNCR 1 1",
    );
    assert_eq!(reputation::get(&state, ncr, FAME), 119.0);
    notices(&mut state);
    // A bump size outside 1–5, or a type outside 0–1, does nothing.
    run(
        &order,
        &scripts,
        &mut state,
        "AddReputation RepNVNCR 1 6\nAddReputation RepNVNCR 2 1",
    );
    assert_eq!(reputation::get(&state, ncr, FAME), 119.0);
    assert!(notices(&mut state).is_empty());
    // Infamy, its words.
    run(&order, &scripts, &mut state, "AddReputation RepNVNCR 0 1");
    assert_eq!(notices(&mut state), ["NCR\nInfamy Gained!"]);
    assert_eq!(reputation::get(&state, ncr, INFAMY), 1.0);
    // The data's description where it has one.
    assert_eq!(
        reputation::title_description(&order, 2, 3),
        "Most people say you're the devil himself, but they also admit you've done a bit of good."
    );
    assert_eq!(reputation::title(&order, 2, 3), "Soft-Hearted Devil");
}

#[test]
fn karma_for_kills_follows_the_victims_own_karma() {
    let (_data, order) = order("factions-karma");
    let mut state = at_outpost(&order);
    let karma =
        |state: &GameState, i: u32| reputation::kill_karma(&order, state, FormId(GOOD_REF + i));
    // In a faction that tracks crime (the NCR): by the victim's band, the
    // data's `fKarmaModKillingEvilActor` 100, the exe's others.
    assert_eq!(karma(&state, 0), -50);
    assert_eq!(karma(&state, 1), -100);
    assert_eq!(karma(&state, 2), 100);
    assert_eq!(karma(&state, 3), 2);
    assert_eq!(karma(&state, 4), 0);
    // In none: nothing, whatever their karma (500).
    assert_eq!(karma(&state, 5), 0);
    // The change and its notice.
    reputation::reward_karma(&order, &mut state, -50);
    assert_eq!(reputation::karma(&order, &state), -50.0);
    assert_eq!(notices(&mut state), ["You've lost Karma!"]);
}

#[test]
fn an_owned_terminal_costs_karma_and_hacking_it_raises_the_alarm() {
    let (_data, order) = order("factions-terminal");
    let mut state = at_outpost(&order);
    // Opening the NCR's terminal: −5 karma (the NCR isn't evil).
    world::terminal::opened(&order, &mut state, FormId(TERMINAL), FormId(TERMINAL_REF));
    assert_eq!(reputation::karma(&order, &state), -5.0);
    // One flagged unlocked costs nothing.
    world::terminal::opened(
        &order,
        &mut state,
        FormId(OPEN_TERMINAL),
        FormId(OPEN_TERMINAL_REF),
    );
    assert_eq!(reputation::karma(&order, &state), -5.0);
    // Hacking it in front of a trooper: he turns on the player, a minor
    // crime for the NCR with 2 infamy, and one for the player.
    let t = world::terminal::Terminal::load(&order, FormId(TERMINAL)).unwrap();
    world::terminal::hacked(&order, &mut state, &t, FormId(TERMINAL_REF));
    assert_eq!(state.combat.get(&FormId(TROOPER_REF)), Some(&PLAYER_REF));
    assert!(!state.combat.contains_key(&FormId(LEGIONARY_REF)));
    assert_eq!(state.faction_crimes.get(&FormId(NCR)), Some(&(1, 0)));
    assert_eq!(reputation::get(&state, FormId(REP_NCR), INFAMY), 2.0);
    assert_eq!(state.player_crimes, (1, 0));
}

#[test]
fn murder_and_assault_make_the_victims_factions_enemies() {
    let (_data, order) = order("factions-murder");
    let scripts = ScriptCache::default();
    let trooper = FormId(TROOPER_REF);
    let sniffer = FormId(SNIFFER_REF);
    // Unseen (the player far behind everyone): the NCR holds the player as
    // an enemy all the same, but no crime is counted and no infamy given;
    // the player is a murderer (the trooper's factions aren't evil).
    let mut state = at_outpost(&order);
    state.player_position = Some([0.0, -5000.0, 0.0]);
    assert_eq!(
        reaction(&order, &state, sniffer, PLAYER_REF),
        Reaction::Neutral
    );
    world::crime::murder(&order, &mut state, trooper, PLAYER_REF);
    assert!(state.crime_enemies.contains(&FormId(NCR)));
    assert_eq!(state.player_crimes, (0, 0));
    assert_eq!(reputation::get(&state, FormId(REP_NCR), INFAMY), 0.0);
    assert!(state.player_murderer);
    assert_eq!(
        reaction(&order, &state, sniffer, PLAYER_REF),
        Reaction::Enemy
    );
    assert_eq!(
        ask(
            &order,
            &scripts,
            &mut state,
            "GetPCEnemyofFaction NCRFactionNV"
        ),
        1.0
    );
    // Every crime-tracking faction the trooper was in: the armour faction
    // too (its `DATA` 0x101).
    assert!(state.crime_enemies.contains(&FormId(ARMOR_NCR)));
    // A friend relation through another faction still wins over a flagged
    // one (`008b87a0` goes on past it): the NCR alone flagged, the
    // disguise's armour faction a friend.
    let mut state = at_outpost(&order);
    run(
        &order,
        &scripts,
        &mut state,
        "SetPCEnemyofFaction NCRFactionNV\nSetAlly PlayerFaction ArmorNCRFactionNV 1 1",
    );
    assert_eq!(
        reaction(&order, &state, sniffer, PLAYER_REF),
        Reaction::Friend
    );
    run(
        &order,
        &scripts,
        &mut state,
        "SetEnemy PlayerFaction ArmorNCRFactionNV 1 1",
    );
    assert_eq!(
        reaction(&order, &state, sniffer, PLAYER_REF),
        Reaction::Enemy
    );
    // Seen: a major crime for the player and the NCR, with 30 infamy (of
    // 80: "Shunned").
    let mut state = at_outpost(&order);
    world::crime::murder(&order, &mut state, trooper, PLAYER_REF);
    assert_eq!(state.player_crimes, (0, 1));
    assert_eq!(state.faction_crimes.get(&FormId(NCR)), Some(&(0, 1)));
    assert_eq!(reputation::get(&state, FormId(REP_NCR), INFAMY), 30.0);
    assert_eq!(popups(&mut state)[0].1.lines().next(), Some("Shunned"));
    // An assault the trooper sees: the NCR an enemy, a major crime, no
    // infamy.
    let mut state = at_outpost(&order);
    world::crime::assault_crime(&order, &mut state, trooper);
    assert!(state.crime_enemies.contains(&FormId(NCR)));
    assert_eq!(state.player_crimes, (0, 1));
    assert_eq!(state.faction_crimes.get(&FormId(NCR)), Some(&(0, 1)));
    assert_eq!(reputation::get(&state, FormId(REP_NCR), INFAMY), 0.0);
    // Nobody detecting the player: no crime at all.
    let mut state = at_outpost(&order);
    state.player_position = Some([0.0, -5000.0, 0.0]);
    world::crime::assault_crime(&order, &mut state, trooper);
    assert!(state.crime_enemies.is_empty());
    assert_eq!(state.player_crimes, (0, 0));
}

#[test]
fn ncr_armour_disguises_the_player_until_a_sniffer_comes_near() {
    let (_data, order) = order("factions-disguise");
    let scripts = ScriptCache::default();
    let mut state = at_outpost(&order);
    let (trooper, sniffer, legionary) = (
        FormId(TROOPER_REF),
        FormId(SNIFFER_REF),
        FormId(LEGIONARY_REF),
    );
    // The NCR hates the player (as `VEFR02NCRBad2QuestSCRIPT` does it), with
    // infamy to match.
    run(
        &order,
        &scripts,
        &mut state,
        "SetEnemy NCRFactionNV PlayerFaction\nSetReputation RepNVNCR 0 50",
    );
    assert_eq!(
        reaction(&order, &state, trooper, PLAYER_REF),
        Reaction::Enemy
    );
    assert_eq!(
        reaction(&order, &state, legionary, PLAYER_REF),
        Reaction::Neutral
    );
    // Putting on the trooper armour runs its script's `OnEquip Player`.
    run(
        &order,
        &scripts,
        &mut state,
        "player.AddItem ArmorNVNCRTrooper 1\nplayer.EquipItem ArmorNVNCRTrooper",
    );
    Runner::new(&order, &scripts, &mut state).run_item_scripts();
    assert_eq!(notices(&mut state), ["Faction armor", "Wearing NCR armor"]);
    // The armour faction is the player's friend: friend beats enemy.
    assert_eq!(
        reaction(&order, &state, trooper, PLAYER_REF),
        Reaction::Friend
    );
    // In `ArmorNCRFactionNVEnemy`, the Legion's enemy.
    assert_eq!(
        reaction(&order, &state, legionary, PLAYER_REF),
        Reaction::Enemy
    );
    // The NCR's infamy is kept aside and cleared while it's worn.
    assert_eq!(reputation::get(&state, FormId(REP_NCR), INFAMY), 0.0);
    assert!(state.running.contains(&FormId(PULSE_QUEST)));
    // The pulse (every second, area 25 × 22 = 550 units) doesn't reach the
    // sniffer 1000 units east: still fooled.
    for _ in 0..3 {
        Runner::new(&order, &scripts, &mut state).update(1.0);
    }
    assert_eq!(
        reaction(&order, &state, sniffer, PLAYER_REF),
        Reaction::Friend
    );
    assert_eq!(
        reaction(&order, &state, trooper, PLAYER_REF),
        Reaction::Friend
    );
    // Walking up to him: the pulse takes him out of the armour faction.
    state.player_position = Some([1000.0, 100.0, 0.0]);
    for _ in 0..3 {
        Runner::new(&order, &scripts, &mut state).update(1.0);
    }
    assert!(world::magic::is_target_of(
        &state,
        sniffer,
        FormId(PULSE_SPELL)
    ));
    assert_eq!(
        reaction(&order, &state, sniffer, PLAYER_REF),
        Reaction::Enemy
    );
    assert_eq!(
        ask(
            &order,
            &scripts,
            &mut state,
            "SnifferRef.GetInFaction ArmorNCRFactionNV"
        ),
        0.0
    );
    // Away again: once his last pulse (5 s) runs out he's fooled again.
    state.player_position = Some([0.0, 100.0, 0.0]);
    for _ in 0..7 {
        Runner::new(&order, &scripts, &mut state).update(1.0);
    }
    assert_eq!(
        reaction(&order, &state, sniffer, PLAYER_REF),
        Reaction::Friend
    );
    // Taking it off: the pulse stops, the alliance ends, and the infamy
    // comes back.
    run(
        &order,
        &scripts,
        &mut state,
        "player.UnequipItem ArmorNVNCRTrooper",
    );
    Runner::new(&order, &scripts, &mut state).run_item_scripts();
    assert_eq!(notices(&mut state), ["Removed NCR armor"]);
    assert!(!state.running.contains(&FormId(PULSE_QUEST)));
    assert_eq!(
        reaction(&order, &state, trooper, PLAYER_REF),
        Reaction::Enemy
    );
    assert_eq!(
        reaction(&order, &state, legionary, PLAYER_REF),
        Reaction::Neutral
    );
    assert_eq!(reputation::get(&state, FormId(REP_NCR), INFAMY), 50.0);
}

/// A viewer stand-in: every ray blocked, or none.
struct Walls(bool);

impl world::sight::Sight for Walls {
    fn bound(&self, _: FormId) -> Option<([f32; 3], [f32; 3])> {
        None
    }
    fn camera(&self) -> Option<[f32; 3]> {
        None
    }
    fn in_view(&self, _: [f32; 3], _: [f32; 3]) -> bool {
        false
    }
    fn ray(&self, _: [f32; 3], _: [f32; 3]) -> Option<f32> {
        self.0.then_some(1.0)
    }
}

#[test]
fn an_area_cast_reaches_the_player_and_needs_a_line_of_sight() {
    // 00818ce0: the player is asked after the loaded actors; each needs
    // a line of sight (008190d0) unless the spell ignores it.
    let (_data, order) = order("factions-area");
    let mut state = at_outpost(&order);
    let sniffer = FormId(SNIFFER_REF);
    let legionary = FormId(LEGIONARY_REF);
    let (_, _, at, _) = state.place(&order, sniffer).unwrap();
    state.player_position = Some([at[0], at[1] + 100.0, at[2]]);
    let spell = FormId(PULSE_SPELL);
    let reached = |state: &GameState, sight: Option<&dyn world::sight::Sight>| {
        world::magic::area_targets(&order, state, spell, legionary, sniffer, sight)
    };
    assert!(reached(&state, None).contains(&PLAYER_REF));
    assert!(reached(&state, Some(&Walls(false))).contains(&PLAYER_REF));
    assert!(!reached(&state, Some(&Walls(true))).contains(&PLAYER_REF));
    // The caster is never reached; a ghost isn't either (008ace90).
    let by_player = world::magic::area_targets(&order, &state, spell, PLAYER_REF, sniffer, None);
    assert!(!by_player.contains(&PLAYER_REF));
    state.more.ghosts.insert(PLAYER_REF);
    assert!(!reached(&state, None).contains(&PLAYER_REF));
}

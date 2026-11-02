//! The script functions in `world::script_functions`, each as the game's own
//! handler carries it out (`testdata::functions` builds the world).

use esm::{ActivePlugins, FormId, LoadOrder};
use testdata::functions::ids::*;
use world::dialogue::{PLAYER_BASE, PLAYER_REF};
use world::scripting::{Event, GameState, Runner, ScriptCache};

fn order(tag: &str) -> (testdata::TempData, LoadOrder) {
    let data = testdata::functions::functions(tag);
    let order = LoadOrder::from_data_dir(data.path(), &ActivePlugins::OfficialOnly).unwrap();
    (data, order)
}

/// Not a value any function here gives: what `ask` returns when the
/// script stopped (the function wasn't carried out).
const STOPPED: f32 = -12345.0;

/// An expression's value, as a script sets a global to it.
fn ask(order: &LoadOrder, scripts: &ScriptCache, state: &mut GameState, expr: &str) -> f32 {
    state.globals.insert(FormId(VALUE), STOPPED);
    Runner::new(order, scripts, state).run_source(&format!("set TestValue to {expr}"), None, None);
    state.globals[&FormId(VALUE)]
}

fn run(order: &LoadOrder, scripts: &ScriptCache, state: &mut GameState, source: &str) {
    Runner::new(order, scripts, state).run_source(source, None, None);
}

/// A new game with the player in the house, in front of the adult (who
/// faces north from 100,0,0).
fn new_game(order: &LoadOrder) -> GameState {
    let mut state = GameState::new(order);
    state.player_cell = Some(FormId(HOUSE));
    state.player_world = None;
    state.player_position = Some([100.0, 50.0, 0.0]);
    state
}

#[test]
fn game_settings_come_from_the_data_else_the_exe() {
    let (_data, order) = order("fn-settings");
    let scripts = ScriptCache::default();
    let mut state = new_game(&order);
    let mut q = |e: &str| ask(&order, &scripts, &mut state, e);
    assert_eq!(q("GetGameSetting fTestSetting"), 2.5);
    assert_eq!(q("GetGS iTestCount"), 7.0);
    // A text setting gives 0; one only the exe defines, its value there.
    assert_eq!(q("GetGameSetting sTestText"), 0.0);
    assert_eq!(q("GetGameSetting iLockLevelMaxEasy"), 25.0);
    // Unknown here: not made up.
    assert_eq!(q("GetGameSetting fNoSuchSetting"), STOPPED);
}

#[test]
fn children_by_race_flag_or_body_model() {
    let (_data, order) = order("fn-child");
    let scripts = ScriptCache::default();
    let mut state = new_game(&order);
    let mut q = |e: &str| ask(&order, &scripts, &mut state, e);
    assert_eq!(q("KidRef.IsChild"), 1.0);
    assert_eq!(q("SmallRef.IsChild"), 1.0);
    assert_eq!(q("AdultRef.IsChild"), 0.0);
    assert_eq!(q("DogRef.IsChild"), 0.0);
    assert_eq!(q("player.IsChild"), 0.0);
}

#[test]
fn restraints_doors_and_owners() {
    let (_data, order) = order("fn-doors");
    let scripts = ScriptCache::default();
    let mut state = new_game(&order);
    assert_eq!(
        ask(&order, &scripts, &mut state, "AdultRef.GetRestrained"),
        0.0
    );
    run(&order, &scripts, &mut state, "AdultRef.SetRestrained 1");
    assert_eq!(
        ask(&order, &scripts, &mut state, "AdultRef.GetRestrained"),
        1.0
    );
    run(&order, &scripts, &mut state, "AdultRef.SetRestrained 0");
    assert_eq!(
        ask(&order, &scripts, &mut state, "AdultRef.GetRestrained"),
        0.0
    );
    // Doors: shut, open by default, locked; anything else 0.
    let q = |state: &mut GameState, e: &str| ask(&order, &scripts, state, e);
    assert_eq!(q(&mut state, "DoorRef.GetOpenState"), 3.0);
    assert_eq!(q(&mut state, "OpenDoorRef.GetOpenState"), 1.0);
    assert_eq!(q(&mut state, "CupRef.GetOpenState"), 0.0);
    // `SetOpenState` activates the door with nobody: its model's sequences
    // aren't known here, so it flips at once; a lock isn't in the way (the
    // lock is only put to an actor) and is cleared.
    run(
        &order,
        &scripts,
        &mut state,
        "DoorRef.SetOpenState 1\nOpenDoorRef.SetOpenState 0\nLockedDoorRef.SetOpenState 1",
    );
    assert_eq!(q(&mut state, "DoorRef.GetOpenState"), 1.0);
    assert_eq!(q(&mut state, "OpenDoorRef.GetOpenState"), 3.0);
    assert_eq!(q(&mut state, "LockedDoorRef.GetOpenState"), 1.0);
    assert_eq!(q(&mut state, "LockedDoorRef.GetLocked"), 0.0);
    // Locking an open door shuts it at once.
    run(&order, &scripts, &mut state, "LockedDoorRef.Lock 50");
    assert_eq!(q(&mut state, "LockedDoorRef.GetOpenState"), 3.0);
    assert_eq!(q(&mut state, "LockedDoorRef.GetLocked"), 1.0);
    // With the model's sequence lengths known, a door swings: opening (2)
    // for its "Open"'s length, then open; closing (4) for "Close"'s; and
    // an activation while it swings does nothing (`0047a560`).
    state.door_lengths.insert(
        FormId(DOOR),
        world::doors::Lengths {
            open: 1.0,
            close: 0.5,
        },
    );
    let door = FormId(DOOR_REF);
    run(&order, &scripts, &mut state, "DoorRef.SetOpenState 0");
    assert_eq!(q(&mut state, "DoorRef.GetOpenState"), 4.0);
    assert_eq!(
        world::doors::activate(&order, &mut state, door, Some(PLAYER_REF)),
        world::doors::Activated::Busy
    );
    Runner::new(&order, &scripts, &mut state).update(0.25);
    assert_eq!(q(&mut state, "DoorRef.GetOpenState"), 4.0);
    Runner::new(&order, &scripts, &mut state).update(0.3);
    assert_eq!(q(&mut state, "DoorRef.GetOpenState"), 3.0);
    assert_eq!(
        world::doors::activate(&order, &mut state, door, Some(PLAYER_REF)),
        world::doors::Activated::Opening
    );
    assert_eq!(q(&mut state, "DoorRef.GetOpenState"), 2.0);
    Runner::new(&order, &scripts, &mut state).update(0.9);
    assert_eq!(q(&mut state, "DoorRef.GetOpenState"), 2.0);
    // Asking for what it already is (opening counts as open) does nothing.
    run(&order, &scripts, &mut state, "DoorRef.SetOpenState 1");
    assert_eq!(q(&mut state, "DoorRef.GetOpenState"), 2.0);
    // Where the leaf is along its swing: 0.9 s into "Open".
    let (opening, at) = world::doors::pose(&order, &state, door, state.seconds);
    assert!(opening && (at - 0.9).abs() < 1e-4);
    Runner::new(&order, &scripts, &mut state).update(0.2);
    // The doors whose sequence has ended (the one flipped at once earlier
    // counts too: nobody has asked since; the locked one's was dropped by
    // the lock), given back once.
    let done = world::doors::finished(&mut state);
    assert_eq!(done.len(), 2, "{done:?}");
    let (_, motion) = done.iter().find(|(d, _)| *d == door).unwrap();
    assert!(motion.opening && motion.by == Some(PLAYER_REF));
    assert_eq!(q(&mut state, "DoorRef.GetOpenState"), 1.0);
    assert!(world::doors::finished(&mut state).is_empty());
    // At rest, the leaf is at the end of its state's sequence.
    assert_eq!(
        world::doors::pose(&order, &state, door, state.seconds),
        (true, 1.0)
    );
    // Owners: the chest is the adult's; set to the player (no argument).
    assert_eq!(q(&mut state, "ChestRef.IsOwner TestAdult"), 1.0);
    assert_eq!(q(&mut state, "ChestRef.IsOwner"), 0.0);
    let chest = FormId(CHEST_REF);
    assert!(!world::crime::may_take(
        &order,
        &state,
        world::crime::owner_of(&order, &state, chest)
    ));
    run(&order, &scripts, &mut state, "ChestRef.SetOwnership");
    assert_eq!(q(&mut state, "ChestRef.IsOwner"), 1.0);
    assert_eq!(
        world::crime::owner_of(&order, &state, chest),
        Some(PLAYER_BASE)
    );
    // The house is the town's: trespassing, until it's the player's or
    // public.
    let house = FormId(HOUSE);
    assert!(world::crime::trespassing(&order, &state, house));
    run(
        &order,
        &scripts,
        &mut state,
        "SetCellPublicFlag TestHouse 1",
    );
    assert!(!world::crime::trespassing(&order, &state, house));
    run(
        &order,
        &scripts,
        &mut state,
        "SetCellPublicFlag TestHouse 0",
    );
    assert!(world::crime::trespassing(&order, &state, house));
    run(&order, &scripts, &mut state, "SetCellOwnership TestHouse");
    assert!(!world::crime::trespassing(&order, &state, house));
    // All kept in a save.
    let (loaded, _) = world::save::load(&world::save::save(&state, None)).unwrap();
    assert_eq!(loaded.set_by_scripts.open, state.set_by_scripts.open);
    assert_eq!(loaded.set_by_scripts.owners, state.set_by_scripts.owners);
    assert_eq!(
        loaded.set_by_scripts.cell_owners,
        state.set_by_scripts.cell_owners
    );
    assert_eq!(
        loaded.set_by_scripts.public_cells,
        state.set_by_scripts.public_cells
    );
}

#[test]
fn a_fighting_friend_forgives_three_hits_within_ten_seconds() {
    let (_data, order) = order("fn-friendhits");
    let scripts = ScriptCache::default();
    let adult_ref = FormId(ADULT_REF);
    let mut state = new_game(&order);
    // In a fight: `iFriendHitCombatAllowed` (the exe's 3).
    state.combat.insert(adult_ref, FormId(0xABC));
    for second in [0.0, 1.0, 2.0] {
        state.seconds = second;
        assert!(!world::crime::assault(&order, &mut state, adult_ref));
    }
    // A hit within `fFriendMinimumLastHitTime` (0.5 s) of the last isn't
    // counted.
    state.seconds = 2.2;
    assert!(!world::crime::assault(&order, &mut state, adult_ref));
    assert_eq!(
        ask(&order, &scripts, &mut state, "AdultRef.GetFriendHit"),
        3.0
    );
    // The fourth: an assault. He saw it, so his town (which tracks crime)
    // holds the player as an enemy from now on (`Actor::AttackAlarm` →
    // `SetFactionsThatCareAboutCrime`).
    state.seconds = 3.0;
    assert!(world::crime::assault(&order, &mut state, adult_ref));
    assert!(state.crime_enemies.contains(&FormId(TOWN)));
    // Past `fFriendHitTimer` (10 s) the old hits are gone.
    state.seconds = 20.0;
    assert_eq!(
        ask(&order, &scripts, &mut state, "AdultRef.GetFriendHit"),
        0.0
    );
    // No longer a friend: no allowance.
    assert!(world::crime::assault(&order, &mut state, adult_ref));
    // Forgiven again once the town's flag is cleared.
    state.crime_enemies.clear();
    assert!(!world::crime::assault(&order, &mut state, adult_ref));
}

#[test]
fn people_who_ignore_crime_or_friendly_hits() {
    let (_data, order) = order("fn-crime");
    let scripts = ScriptCache::default();
    let chest = FormId(CHEST_REF);
    let adult = FormId(ADULT);
    // Stealing the adult's things in front of him: seen.
    let mut state = new_game(&order);
    assert!(world::crime::steal(&order, &mut state, chest, adult));
    // Unless he ignores crime.
    let mut state = new_game(&order);
    run(&order, &scripts, &mut state, "AdultRef.IgnoreCrime 1");
    assert!(!world::crime::steal(&order, &mut state, chest, adult));
    // The town is the player's friend: the first hit is forgiven (none
    // allowed out of combat: `iFriendHitNonCombatAllowed` 0, so it's
    // already an assault, and with no allowance the hit isn't counted:
    // `008987f0` adds friend hits only for an allowance above 0).
    let adult_ref = FormId(ADULT_REF);
    let mut state = new_game(&order);
    assert!(world::crime::assault(&order, &mut state, adult_ref));
    assert_eq!(
        ask(&order, &scripts, &mut state, "AdultRef.GetFriendHit"),
        0.0
    );
    // Ignoring friendly hits: no notice at all, not counted.
    let mut state = new_game(&order);
    run(
        &order,
        &scripts,
        &mut state,
        "AdultRef.SetIgnoreFriendlyHits 1",
    );
    assert!(!world::crime::assault(&order, &mut state, adult_ref));
    assert_eq!(
        ask(&order, &scripts, &mut state, "AdultRef.GetFriendHit"),
        0.0
    );
    // Ignoring crime: no crime, but he still fights back.
    let mut state = new_game(&order);
    run(&order, &scripts, &mut state, "AdultRef.IgnoreCrime 1");
    assert!(world::crime::assault(&order, &mut state, adult_ref));
    assert_eq!(state.player_crimes, (0, 0));
    // A faction holding the player as an enemy for their crimes.
    assert_eq!(
        ask(
            &order,
            &scripts,
            &mut state,
            "AdultRef.GetActorFactionPlayerEnemy"
        ),
        0.0
    );
    run(&order, &scripts, &mut state, "SetPCEnemyofFaction TestTown");
    assert_eq!(
        ask(
            &order,
            &scripts,
            &mut state,
            "AdultRef.GetActorFactionPlayerEnemy"
        ),
        1.0
    );
}

#[test]
fn assault_alarms_and_regions() {
    let (_data, order) = order("fn-alarms");
    let scripts = ScriptCache::default();
    let adult = FormId(ADULT_REF);
    // On the adult himself: he reports it (a witnessed major crime for his
    // town) and turns on the player.
    let mut state = new_game(&order);
    run(&order, &scripts, &mut state, "AdultRef.SendAssaultAlarm");
    assert_eq!(state.combat.get(&adult), Some(&PLAYER_REF));
    assert_eq!(state.player_crimes.1, 1);
    assert_eq!(state.faction_crimes.get(&FormId(TOWN)), Some(&(0, 1)));
    // "Player <faction>": the faction's member near the player who notices
    // them.
    let mut state = new_game(&order);
    run(
        &order,
        &scripts,
        &mut state,
        "KidRef.SendAssaultAlarm Player TestTown",
    );
    assert_eq!(state.combat.get(&adult), Some(&PLAYER_REF));
    assert!(!state.combat.contains_key(&FormId(KID_REF)));
    // "Player" alone does nothing; nor does anything but a person.
    let mut state = new_game(&order);
    run(
        &order,
        &scripts,
        &mut state,
        "AdultRef.SendAssaultAlarm Player",
    );
    assert!(state.combat.is_empty());
    assert_eq!(state.player_crimes, (0, 0));
    // Talking: whoever the viewer has saying a line.
    assert_eq!(ask(&order, &scripts, &mut state, "AdultRef.IsTalking"), 0.0);
    state.speaking.insert(adult);
    assert_eq!(ask(&order, &scripts, &mut state, "AdultRef.IsTalking"), 1.0);
    // The house lists the region.
    assert_eq!(
        ask(&order, &scripts, &mut state, "PlayerInRegion TestRegion"),
        1.0
    );
    state.player_cell = Some(FormId(ELSEWHERE));
    assert_eq!(
        ask(&order, &scripts, &mut state, "PlayerInRegion TestRegion"),
        0.0
    );
}

#[test]
fn current_packages_by_the_games_numbers() {
    let (_data, order) = order("fn-packages");
    let scripts = ScriptCache::default();
    let mut state = new_game(&order);
    let q = |state: &mut GameState, e: &str| ask(&order, &scripts, state, e);
    // No package: 0; a travel package 14, a sandbox 36 (the game's own
    // numbering, not the record's type).
    assert_eq!(q(&mut state, "AdultRef.GetCurrentAIPackage"), 0.0);
    run(
        &order,
        &scripts,
        &mut state,
        "AdultRef.AddScriptPackage TestTravel",
    );
    assert_eq!(q(&mut state, "AdultRef.GetCurrentAIPackage"), 14.0);
    run(
        &order,
        &scripts,
        &mut state,
        "AdultRef.AddScriptPackage TestSandbox",
    );
    assert_eq!(q(&mut state, "AdultRef.GetCurrentAIPackage"), 36.0);
    assert_eq!(q(&mut state, "CupRef.GetCurrentAIPackage"), 0.0);
    // Fighting: the game's combat package, not known here.
    state.combat.insert(FormId(ADULT_REF), PLAYER_REF);
    assert_eq!(q(&mut state, "AdultRef.GetCurrentAIPackage"), STOPPED);
}

#[test]
fn entering_a_trigger_runs_its_events_for_whoever_entered() {
    let (_data, order) = order("fn-trigger");
    let scripts = ScriptCache::default();
    let mut state = new_game(&order);
    run(
        &order,
        &scripts,
        &mut state,
        "TriggerRef.EnterTrigger AdultRef",
    );
    assert_eq!(
        ask(&order, &scripts, &mut state, "TriggerRef.iEntered"),
        1.0
    );
    assert_eq!(ask(&order, &scripts, &mut state, "TriggerRef.iInside"), 0.0);
    run(
        &order,
        &scripts,
        &mut state,
        "TriggerRef.EnterTrigger player",
    );
    assert_eq!(ask(&order, &scripts, &mut state, "TriggerRef.iInside"), 1.0);
    assert!(state.unhandled.is_empty(), "{:?}", state.unhandled);
}

#[test]
fn fast_travel_and_waiting_as_scripts_allow() {
    let (_data, order) = order("fn-travel");
    let scripts = ScriptCache::default();
    let mut state = new_game(&order);
    // In the town, so not trespassing in its house (the game refuses
    // waiting while trespassing, `00969fa0`).
    run(
        &order,
        &scripts,
        &mut state,
        "player.AddToFaction TestTown 0",
    );
    let block = "Fast travel is currently unavailable from this location.";
    // The house doesn't allow fast travel anyway; the script's block is
    // asked first.
    run(&order, &scripts, &mut state, "EnableFastTravel 0");
    assert_eq!(
        world::map::travel_refused(&order, &state).as_deref(),
        Some(block)
    );
    assert!(state.wait(&order, 1.0).is_ok());
    // Moving the player allows it again.
    run(&order, &scripts, &mut state, "player.MoveTo CupRef");
    assert_ne!(
        world::map::travel_refused(&order, &state).as_deref(),
        Some(block)
    );
    // Unless the script keeps it; and the second flag 0 forbids waiting.
    run(&order, &scripts, &mut state, "EnableFastTravel 0 0 1");
    assert!(state.wait(&order, 1.0).is_err());
    run(&order, &scripts, &mut state, "player.MoveTo CupRef");
    assert_eq!(
        world::map::travel_refused(&order, &state).as_deref(),
        Some(block)
    );
    let (loaded, _) = world::save::load(&world::save::save(&state, None)).unwrap();
    assert_eq!(
        loaded.set_by_scripts.fast_travel,
        state.set_by_scripts.fast_travel
    );
    run(&order, &scripts, &mut state, "EnableFastTravel 1");
    assert!(state.wait(&order, 1.0).is_ok());
    assert_ne!(
        world::map::travel_refused(&order, &state).as_deref(),
        Some(block)
    );
}

#[test]
fn factions_notes_and_achievements() {
    let (_data, order) = order("fn-factions");
    let scripts = ScriptCache::default();
    let mut state = new_game(&order);
    let q = |state: &mut GameState, e: &str| ask(&order, &scripts, state, e);
    // New Vegas gives 1 for a member, -1 otherwise (not the rank); a
    // rank of -1 in the record isn't membership.
    assert_eq!(q(&mut state, "AdultRef.GetFactionRank TestTown"), 1.0);
    assert_eq!(q(&mut state, "AdultRef.GetFactionRank OtherFaction"), -1.0);
    assert_eq!(q(&mut state, "CupRef.GetFactionRank TestTown"), -1.0);
    // Nor in anything else: the faction rules leave it out too.
    let factions = world::factions::factions_of(&order, &state, FormId(ADULT_REF));
    assert!(factions.contains(&FormId(TOWN)));
    assert!(!factions.contains(&FormId(OTHER_FACTION)));
    run(
        &order,
        &scripts,
        &mut state,
        "AdultRef.RemoveFromAllFactions",
    );
    assert_eq!(q(&mut state, "AdultRef.GetFactionRank TestTown"), -1.0);
    assert_eq!(q(&mut state, "AdultRef.GetInFaction TestTown"), 0.0);
    // Notes.
    run(&order, &scripts, &mut state, "AddNote TestNote");
    assert_eq!(q(&mut state, "GetHasNote TestNote"), 1.0);
    run(&order, &scripts, &mut state, "RemoveNote TestNote");
    assert_eq!(q(&mut state, "GetHasNote TestNote"), 0.0);
    // Achievements and challenges are kept; nothing completes a challenge.
    run(
        &order,
        &scripts,
        &mut state,
        "AddAchievement 5\nUnlockChallenge TestChallenge",
    );
    assert!(state.set_by_scripts.achievements.contains(&5));
    assert!(state.set_by_scripts.challenges.contains(&FormId(CHALLENGE)));
    assert_eq!(q(&mut state, "GetChallengeCompleted TestChallenge"), 0.0);
    assert_eq!(q(&mut state, "GetCasinoWinningStage TestChallenge"), 0.0);
    assert!(state.unhandled.is_empty(), "{:?}", state.unhandled);
}

#[test]
fn positions_angles_and_scale() {
    let (_data, order) = order("fn-positions");
    let scripts = ScriptCache::default();
    let mut state = new_game(&order);
    let q = |state: &mut GameState, e: &str| ask(&order, &scripts, state, e);
    let close = |a: f32, b: f32| (a - b).abs() < 1e-3;
    assert_eq!(q(&mut state, "CupRef.GetPos X"), 100.0);
    assert!(close(q(&mut state, "CupRef.GetAngle Z"), 30.0));
    assert!(close(q(&mut state, "CupRef.GetAngle X"), 10.0));
    run(
        &order,
        &scripts,
        &mut state,
        "CupRef.SetPos X 500\nCupRef.SetAngle Z 45\nCupRef.SetAngle X 5",
    );
    assert_eq!(q(&mut state, "CupRef.GetPos X"), 500.0);
    assert_eq!(q(&mut state, "CupRef.GetPos Y"), 100.0);
    assert!(close(q(&mut state, "CupRef.GetAngle Z"), 45.0));
    assert!(close(q(&mut state, "CupRef.GetAngle X"), 5.0));
    assert!(close(q(&mut state, "CupRef.GetAngle Y"), 20.0));
    // Where it started stays.
    assert_eq!(q(&mut state, "CupRef.GetStartingPos X"), 100.0);
    assert!(close(q(&mut state, "CupRef.GetStartingAngle Z"), 30.0));
    assert_eq!(q(&mut state, "player.GetPos Y"), 50.0);
    // Scale: placed 2, then as set.
    assert_eq!(q(&mut state, "CupRef.GetScale"), 2.0);
    run(&order, &scripts, &mut state, "CupRef.SetScale 3");
    assert_eq!(q(&mut state, "CupRef.GetScale"), 3.0);
    // The cup is ahead of the adult (north): 0Â°; the rifle to his east
    // (clockwise): 90Â°.
    assert!(close(
        q(&mut state, "AdultRef.GetHeadingAngle RifleRef"),
        90.0
    ));
    assert!(close(
        q(&mut state, "AdultRef.GetHeadingAngle KidRef"),
        -135.0
    ));
    let (loaded, _) = world::save::load(&world::save::save(&state, None)).unwrap();
    assert_eq!(loaded.set_by_scripts.tilts, state.set_by_scripts.tilts);
}

#[test]
fn creatures_classes_weapons_and_lists() {
    let (_data, order) = order("fn-kinds");
    let scripts = ScriptCache::default();
    let mut state = new_game(&order);
    let q = |state: &mut GameState, e: &str| ask(&order, &scripts, state, e);
    assert_eq!(q(&mut state, "DogRef.GetIsCreature"), 1.0);
    assert_eq!(q(&mut state, "AdultRef.GetIsCreature"), 0.0);
    assert_eq!(q(&mut state, "DogRef.GetIsCreatureType 2"), 1.0);
    assert_eq!(q(&mut state, "DogRef.GetIsCreatureType 1"), 0.0);
    assert_eq!(q(&mut state, "AdultRef.GetIsClass TestClass"), 1.0);
    assert_eq!(q(&mut state, "KidRef.GetIsClass TestClass"), 0.0);
    // No weapon: hand-to-hand (1), the fists aren't in the list.
    assert_eq!(q(&mut state, "AdultRef.GetWeaponAnimType"), 1.0);
    assert_eq!(q(&mut state, "AdultRef.IsWeaponInList TestWeapons"), 0.0);
    run(
        &order,
        &scripts,
        &mut state,
        "AdultRef.AddItem TestRifle 1\nAdultRef.EquipItem TestRifle",
    );
    assert_eq!(q(&mut state, "AdultRef.GetWeaponAnimType"), 5.0);
    assert_eq!(q(&mut state, "AdultRef.IsWeaponInList TestWeapons"), 1.0);
    assert_eq!(q(&mut state, "CupRef.GetWeaponAnimType"), 0.0);
    // The player: only what's equipped counts.
    run(&order, &scripts, &mut state, "player.AddItem TestRifle 1");
    assert_eq!(q(&mut state, "player.GetWeaponAnimType"), 1.0);
    run(&order, &scripts, &mut state, "player.EquipItem TestRifle");
    assert_eq!(q(&mut state, "player.GetWeaponAnimType"), 5.0);
    // Asked about the rifle: Guns; anything else counts as unarmed.
    assert_eq!(q(&mut state, "RifleRef.IsWeaponSkillType Guns"), 1.0);
    assert_eq!(q(&mut state, "RifleRef.IsWeaponSkillType Unarmed"), 0.0);
    assert_eq!(q(&mut state, "CupRef.IsWeaponSkillType Unarmed"), 1.0);
    // Form lists, with what scripts add.
    assert_eq!(q(&mut state, "CupRef.IsInList TestList"), 1.0);
    assert_eq!(q(&mut state, "AdultRef.IsInList TestList"), 1.0);
    assert_eq!(q(&mut state, "KidRef.IsInList TestList"), 0.0);
    run(
        &order,
        &scripts,
        &mut state,
        "AddFormToFormList TestList TestKid",
    );
    assert_eq!(q(&mut state, "KidRef.IsInList TestList"), 1.0);
    let (loaded, _) = world::save::load(&world::save::save(&state, None)).unwrap();
    assert_eq!(
        loaded.set_by_scripts.list_additions,
        state.set_by_scripts.list_additions
    );
}

#[test]
fn objectives_shown_and_completed_as_the_game_keeps_them() {
    let (_data, order) = order("fn-objectives");
    let scripts = ScriptCache::default();
    let mut state = new_game(&order);
    let q = |state: &mut GameState, e: &str| ask(&order, &scripts, state, e);
    run(
        &order,
        &scripts,
        &mut state,
        "SetObjectiveDisplayed TestQuest 10 1",
    );
    // The first objective shown says "Quest added" on the HUD (once:
    // `005ec5d0`, the quest's flag 0x20), and is kept in a save.
    let added = |state: &GameState| {
        state
            .events
            .iter()
            .filter(|e| {
                matches!(
                    e,
                    Event::QuestText(world::quest_text::QuestText::Quest {
                        update: world::quest_text::Update::Added,
                        ..
                    })
                )
            })
            .count()
    };
    assert_eq!(added(&state), 1);
    let (loaded, _) = world::save::load(&world::save::save(&state, None)).unwrap();
    assert!(loaded.quests_announced.contains(&FormId(QUEST)));
    run(
        &order,
        &scripts,
        &mut state,
        "SetObjectiveDisplayed TestQuest 20 1\nSetObjectiveDisplayed TestQuest 20 0",
    );
    assert_eq!(added(&state), 1);
    state.events.clear();
    run(
        &order,
        &scripts,
        &mut state,
        "CompleteAllObjectives TestQuest",
    );
    // The shown one is done and still shown (with its notice); the others
    // are done without being shown.
    assert_eq!(q(&mut state, "GetObjectiveCompleted TestQuest 10"), 1.0);
    assert_eq!(q(&mut state, "GetObjectiveDisplayed TestQuest 10"), 1.0);
    assert_eq!(q(&mut state, "GetObjectiveCompleted TestQuest 20"), 1.0);
    assert_eq!(q(&mut state, "GetObjectiveDisplayed TestQuest 20"), 0.0);
    assert!(state.events.contains(&Event::Objective {
        quest: FormId(QUEST),
        text: "Objective 10".into(),
        completed: true
    }));
    assert_eq!(
        state
            .events
            .iter()
            .filter(|e| matches!(e, Event::Objective { .. }))
            .count(),
        1
    );
    let (loaded, _) = world::save::load(&world::save::save(&state, None)).unwrap();
    assert_eq!(
        loaded.set_by_scripts.hidden_completed,
        state.set_by_scripts.hidden_completed
    );
    // Turning completion off clears the objective altogether (state 0).
    run(
        &order,
        &scripts,
        &mut state,
        "SetObjectiveCompleted TestQuest 10 0",
    );
    assert_eq!(q(&mut state, "GetObjectiveDisplayed TestQuest 10"), 0.0);
    assert_eq!(q(&mut state, "GetObjectiveCompleted TestQuest 10"), 0.0);
    // An objective the quest doesn't have: nothing.
    run(
        &order,
        &scripts,
        &mut state,
        "SetObjectiveDisplayed TestQuest 99 1",
    );
    assert_eq!(q(&mut state, "GetObjectiveDisplayed TestQuest 99"), 0.0);
    // Resetting the quest: back to the start (running, as it starts with
    // the game), nothing shown or done.
    run(
        &order,
        &scripts,
        &mut state,
        "SetStage TestQuest 10\nCompleteQuest TestQuest",
    );
    assert_eq!(q(&mut state, "GetStage TestQuest"), 10.0);
    run(&order, &scripts, &mut state, "ResetQuest TestQuest");
    assert_eq!(q(&mut state, "GetStage TestQuest"), 0.0);
    assert_eq!(q(&mut state, "GetStageDone TestQuest 10"), 0.0);
    assert_eq!(q(&mut state, "GetQuestCompleted TestQuest"), 0.0);
    assert_eq!(q(&mut state, "GetQuestRunning TestQuest"), 1.0);
    assert_eq!(q(&mut state, "GetObjectiveCompleted TestQuest 20"), 0.0);
    // One that doesn't start with the game stops.
    run(&order, &scripts, &mut state, "SetStage OtherQuest 5");
    assert_eq!(q(&mut state, "GetQuestRunning OtherQuest"), 1.0);
    run(&order, &scripts, &mut state, "ResetQuest OtherQuest");
    assert_eq!(q(&mut state, "GetQuestRunning OtherQuest"), 0.0);
}

#[test]
fn cells_by_editor_id_interiors_and_image_spaces() {
    let (_data, order) = order("fn-cells");
    let scripts = ScriptCache::default();
    let mut state = new_game(&order);
    let q = |state: &mut GameState, e: &str| ask(&order, &scripts, state, e);
    // The cellar's editor ID begins with the house's.
    assert_eq!(q(&mut state, "CellarCupRef.GetInCell TestHouse"), 1.0);
    assert_eq!(q(&mut state, "CupRef.GetInCell TestHouseCellar"), 0.0);
    assert_eq!(q(&mut state, "CupRef.GetInCell Elsewhere"), 0.0);
    assert_eq!(q(&mut state, "GetInCellParam TestHouse CellarCupRef"), 1.0);
    assert_eq!(q(&mut state, "GetInCellParam Elsewhere CellarCupRef"), 0.0);
    assert_eq!(q(&mut state, "CupRef.IsInInterior"), 1.0);
    assert_eq!(q(&mut state, "player.IsInInterior"), 1.0);
    assert_eq!(q(&mut state, "IsImageSpaceActive TestFlash"), 0.0);
    run(
        &order,
        &scripts,
        &mut state,
        "ApplyImageSpaceModifier TestFlash",
    );
    assert_eq!(q(&mut state, "IsImageSpaceActive TestFlash"), 1.0);
    assert_eq!(q(&mut state, "IsPS3"), 0.0);
    assert_eq!(q(&mut state, "IsPC1stPerson"), 1.0);
    assert_eq!(q(&mut state, "AdultRef.Exists KidRef"), 1.0);
    assert_eq!(q(&mut state, "AdultRef.Exists AdultRef"), 0.0);
}

#[test]
fn health_killers_tags_names_and_inventories() {
    let (_data, order) = order("fn-people");
    let scripts = ScriptCache::default();
    let mut state = new_game(&order);
    let q = |state: &mut GameState, e: &str| ask(&order, &scripts, state, e);
    let adult = FormId(ADULT_REF);
    state.damage.insert(adult, 10.0);
    state.value_damage.insert((adult, 29), 40.0);
    run(&order, &scripts, &mut state, "AdultRef.ResetHealth");
    assert_eq!(state.damage.get(&adult).copied().unwrap_or(0.0), 0.0);
    assert_eq!(
        state.value_damage.get(&(adult, 29)).copied().unwrap_or(0.0),
        0.0
    );
    // Who killed whom, kept in a save.
    run(&order, &scripts, &mut state, "DogRef.KillActor player");
    assert_eq!(q(&mut state, "DogRef.IsKiller player"), 1.0);
    assert_eq!(q(&mut state, "DogRef.IsKiller AdultRef"), 0.0);
    assert_eq!(q(&mut state, "AdultRef.IsKiller player"), 0.0);
    let (mut loaded, _) = world::save::load(&world::save::save(&state, None)).unwrap();
    assert_eq!(q(&mut loaded, "DogRef.IsKiller player"), 1.0);
    // Tag skills.
    state.tag_skills.insert(41);
    assert_eq!(q(&mut state, "IsPlayerTagSkill Guns"), 1.0);
    assert_eq!(q(&mut state, "IsPlayerTagSkill Sneak"), 0.0);
    // A new name: the message's name (`FULL`), not its text.
    run(
        &order,
        &scripts,
        &mut state,
        "AdultRef.SetActorFullName TestNameMessage",
    );
    assert_eq!(
        world::script_functions::full_name(&order, &state, adult).as_deref(),
        Some("A Title")
    );
    let (loaded, _) = world::save::load(&world::save::save(&state, None)).unwrap();
    assert_eq!(loaded.set_by_scripts.names, state.set_by_scripts.names);
    // An inventory back to the record's.
    run(
        &order,
        &scripts,
        &mut state,
        "AdultRef.AddItem TestCup 3\nAdultRef.ResetInventory",
    );
    assert_eq!(state.item_count(&order, adult, FormId(CUP)), 0);
    // Items at a health: the rifle half worn.
    run(
        &order,
        &scripts,
        &mut state,
        "AdultRef.AddItemHealthPercent TestRifle 1 0.5",
    );
    assert_eq!(state.item_count(&order, adult, FormId(RIFLE)), 1);
    assert_eq!(state.weapon_health.get(&(adult, FormId(RIFLE))), Some(&0.5));
}

#[test]
fn removing_items_by_type_keeps_holdouts_and_quest_items() {
    let (_data, order) = order("fn-remove");
    let scripts = ScriptCache::default();
    let mut state = new_game(&order);
    run(
        &order,
        &scripts,
        &mut state,
        "player.AddItem TestCup 1\nplayer.AddItem TestShirt 1\nplayer.AddItem TestDrink 2\n\
         player.AddItem TestQuestKey 1\nplayer.EquipItem TestShirt",
    );
    let (player, chest) = (PLAYER_REF, FormId(CHEST_REF));
    let has =
        |state: &GameState, who: FormId, item: u32| state.item_count(&order, who, FormId(item));
    // Armour (24) only, kept out by the holdout list.
    run(
        &order,
        &scripts,
        &mut state,
        "player.RemoveAllTypedItems ChestRef 0 0 24 TestHoldout",
    );
    assert_eq!(has(&state, player, SHIRT), 1);
    // Armour only: the shirt goes, taken off.
    run(
        &order,
        &scripts,
        &mut state,
        "player.RemoveAllTypedItems ChestRef 0 0 24",
    );
    assert_eq!(has(&state, player, SHIRT), 0);
    assert_eq!(has(&state, chest, SHIRT), 1);
    assert!(!state.is_equipped(player, FormId(SHIRT)));
    assert_eq!(has(&state, player, CUP), 1);
    // A script makes the cup a quest item: everything else goes, quest
    // items stay with the player.
    run(
        &order,
        &scripts,
        &mut state,
        "SetQuestObject TestCup 1\nplayer.RemoveAllTypedItems ChestRef",
    );
    assert_eq!(has(&state, player, CUP), 1);
    assert_eq!(has(&state, player, QUEST_KEY), 1);
    assert_eq!(has(&state, player, DRINK), 0);
    assert_eq!(has(&state, chest, DRINK), 2);
    // `RemoveAllItems` keeps them too.
    run(
        &order,
        &scripts,
        &mut state,
        "SetQuestObject TestCup 0\nplayer.RemoveAllItems",
    );
    assert_eq!(has(&state, player, CUP), 0);
    assert_eq!(has(&state, player, QUEST_KEY), 1);
}

#[test]
fn what_scripts_show_lines_animations_idles_and_textures() {
    let (_data, order) = order("fn-events");
    let scripts = ScriptCache::default();
    let mut state = new_game(&order);
    run(
        &order,
        &scripts,
        &mut state,
        "AdultRef.Say TestTopic\nDoorRef.PlayGroup Forward 1\nAdultRef.PlayIdle TestWave\n\
         SwapTexture CupRef \"Rim\" \"cups\\red\"\nAdultRef.Look player\nShowWarning \"careful\"\n\
         PurgeCellBuffers",
    );
    assert!(state.unhandled.is_empty(), "{:?}", state.unhandled);
    let adult = FormId(ADULT_REF);
    assert_eq!(
        state.events,
        vec![
            Event::Talk {
                speaker: adult,
                to: FormId(0),
                topic: Some(FormId(TOPIC)),
                conversation: false,
            },
            Event::PlayGroup {
                who: FormId(DOOR_REF),
                group: "Forward".into(),
                flags: 1,
            },
            Event::PlayIdle {
                who: adult,
                idle: FormId(WAVE),
            },
            Event::SwapTexture {
                what: FormId(CUP_REF),
                node: "Rim".into(),
                texture: "Textures\\cups\\red.dds".into(),
            },
        ]
    );
    assert_eq!(state.set_by_scripts.looking.get(&adult), Some(&PLAYER_REF));
    run(&order, &scripts, &mut state, "AdultRef.StopLook");
    assert!(state.set_by_scripts.looking.is_empty());
}

/// `ShowTutorialMenu` (`005da630`) opens the tutorial menu with the
/// message it names, straight away (not through the tutorial manager);
/// without one, with none.
#[test]
fn show_tutorial_menu_asks_for_the_menu_with_its_message() {
    let (_data, order) = order("fn-tutorial");
    let scripts = ScriptCache::default();
    let mut state = new_game(&order);
    run(
        &order,
        &scripts,
        &mut state,
        "ShowTutorialMenu TestNameMessage",
    );
    assert!(state.unhandled.is_empty(), "{:?}", state.unhandled);
    assert_eq!(
        state.events,
        vec![Event::TutorialMenu(FormId(NAME_MESSAGE))]
    );
    assert!(state.tutorials == Default::default());
}

/// The menu backgrounds' modifiers, generated as `FalloutNV.esm` lays them
/// out: not animatable, so their first keys hold; the Vigor Tester's
/// popup blur of 3 draws the radius-3 pass with the table's row 3, and the
/// pause menu's also takes the colour out and tints.
#[test]
fn menu_backgrounds_take_their_modifiers_first_keys() {
    use world::menu_background::{blur_weights, modifier, weight_row, MenuState};
    use world::modifier::{track, Modifier};
    let (_data, order) = order("menu-background");
    let tester = MenuState {
        setting: true,
        menu_mode: true,
        top: Some(world::menu_background::menu::LOVE_TESTER),
        ..MenuState::default()
    };
    let id = modifier(&tester).unwrap();
    assert_eq!(id, FormId(MENU_POPUP_FX));
    let popup = Modifier::load(&order, id).unwrap();
    assert!(!popup.animatable);
    // Held at the first keys however long the menu is up.
    for age in [0.0, 0.5, 30.0] {
        let v = popup.at(age);
        assert_eq!(v.blur, 3.0);
        assert_eq!(v.multiply[track::SATURATION], 1.0);
        assert_eq!(v.tint[3], 0.0);
    }
    assert_eq!(blur_weights(popup.at(0.0).blur), Some((3, weight_row(3))));
    let pause = MenuState {
        pause_menu: true,
        ..tester
    };
    let pause = Modifier::load(&order, modifier(&pause).unwrap()).unwrap();
    assert_eq!(pause.editor_id.as_deref(), Some("PauseBackgroundFX"));
    let v = pause.at(10.0);
    assert_eq!((v.blur, v.multiply[track::SATURATION]), (3.0, 0.0));
    assert_eq!(v.tint, [0.671, 0.659, 0.239, 0.588]);
}

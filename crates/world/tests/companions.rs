//! Companions (`OpenTeammateContainer`, trading things with them).

use esm::{ActivePlugins, FormId, LoadOrder};
use testdata::quest_ids::*;
use world::scripting::{Event, GameState, Runner, ScriptCache};

fn order(tag: &str) -> (testdata::TempData, LoadOrder) {
    let data = testdata::quests(tag);
    let order = LoadOrder::from_data_dir(data.path(), &ActivePlugins::OfficialOnly).unwrap();
    (data, order)
}

/// `OpenTeammateContainer` (`005d9430`): on a teammate, or anyone with a
/// number other than 0; a companion's room (`0075dc80`, `008a0c20`,
/// `00577250`).
#[test]
fn trading_with_a_companion() {
    let (_data, order) = order("companions-trade");
    let scripts = ScriptCache::default();
    let mut state = GameState::new(&order);
    let doc = FormId(DOC_REF);
    let run = |state: &mut GameState, line: &str| {
        state.events.clear();
        Runner::new(&order, &scripts, state).run_source(line, None, None);
        assert!(state.unhandled.is_empty(), "{:?}", state.unhandled_first);
        state.events.clone()
    };
    assert!(run(&mut state, "DocRef.OpenTeammateContainer").is_empty());
    assert_eq!(
        run(&mut state, "DocRef.OpenTeammateContainer 1"),
        vec![Event::TeammateContainer(doc)]
    );
    state.teammates.insert(doc);
    assert_eq!(
        run(&mut state, "DocRef.OpenTeammateContainer"),
        vec![Event::TeammateContainer(doc)]
    );
    // Not on something that isn't a person.
    assert!(run(&mut state, "ChestRef.OpenTeammateContainer 1").is_empty());

    // Carry Weight 200 (the test world has no `fAVDCarryWeights…`).
    state.actor_values.insert((doc, 13), 200.0);
    let (carried, most) = world::items::carry(&order, &state, doc);
    assert!(most > 0.0 && carried <= most);
    // The rifle weighs 6.
    let room = ((most - carried) / 6.0).floor() as i32;
    assert!(world::items::has_room(
        &order,
        &state,
        doc,
        FormId(RIFLE),
        room
    ));
    assert!(!world::items::has_room(
        &order,
        &state,
        doc,
        FormId(RIFLE),
        room + 1
    ));
}

/// The companion wheel (`CompanionWheelMenu`): who it comes up for
/// (`005fa330`, `00754d90`), its switches (`00754de0`), a button's topic
/// said with both result scripts run at once (`007573d0`, `007575b0`), the
/// Stimpak (`00756490`).
#[test]
fn the_companion_wheel() {
    use world::companions;
    use world::living::pickpocket::{use_person, Use};
    let (_data, order) = order("companions-wheel");
    let scripts = ScriptCache::default();
    let mut state = GameState::new(&order);
    let doc = FormId(DOC_REF);
    assert_eq!(use_person(&order, &state, doc, false), Use::Talk);
    state.teammates.insert(doc);
    // Unaggressive: the wheel; frenzied (he'd attack the player): nothing.
    state.actor_values.insert((doc, 0), 0.0);
    assert_eq!(use_person(&order, &state, doc, false), Use::Wheel);
    state.actor_values.insert((doc, 0), 3.0);
    assert_eq!(use_person(&order, &state, doc, false), Use::Nothing);
    state.actor_values.insert((doc, 0), 0.0);
    // Sneaking doesn't pick a teammate's pockets.
    state.player_sneaking = true;
    assert_eq!(use_person(&order, &state, doc, false), Use::Wheel);
    state.player_sneaking = false;

    // His script's variables, at 0 before it has run.
    let switches = companions::switches(&order, &scripts, &state, doc);
    assert_eq!(switches, companions::Switches::default());
    assert_eq!(
        companions::variable(&order, &scripts, &state, doc, "NoSuchThing"),
        -1.0
    );
    assert!(companions::set_variable(
        &order,
        &scripts,
        &mut state,
        doc,
        "FollowerSwitchAggressive",
        1.0
    ));
    assert!(companions::switches(&order, &scripts, &state, doc).aggressive);

    // "Wait Here": his line, then its begin and end scripts.
    let info = companions::say_topic(&order, &scripts, &mut state, doc, "FollowersWait", true)
        .expect("his line");
    assert_eq!(info.responses[0].text, "I'll sit tight.");
    assert!(companions::switches(&order, &scripts, &state, doc).waiting);
    assert_eq!(state.globals.get(&FormId(GLOBAL)), Some(&7.0));
    // Not through the dialogue's own result-script call (`0061f170`): the
    // line isn't marked said, nor he talked to.
    assert!(!state.said.contains(&FormId(WAIT_LINE)));
    assert!(!state.talked_to.contains(&doc));

    // Back Up (`008a7760`): the hook for the AI's default package.
    state.events.clear();
    companions::back_up(&mut state, doc);
    assert_eq!(state.events, vec![Event::BackUp(doc)]);

    // The Stimpak: none, then one; not hurt, nothing.
    assert!(!companions::heal_with_stimpak(&order, &mut state, doc));
    state
        .items
        .insert((world::dialogue::PLAYER_REF, FormId(STIMPAK)), 1);
    assert!(!companions::heal_with_stimpak(&order, &mut state, doc));
    // Hurt: healed, the Stimpak used.
    state.damage.insert(doc, 50.0);
    assert!(companions::heal_with_stimpak(&order, &mut state, doc));
    assert_eq!(
        state.item_count(&order, world::dialogue::PLAYER_REF, FormId(STIMPAK)),
        0
    );
    assert!(state.damage.get(&doc).copied().unwrap_or(0.0) < 50.0);
    // A crippled leg, at full health: restored, and the Stimpak used.
    state.damage.remove(&doc);
    state
        .items
        .insert((world::dialogue::PLAYER_REF, FormId(STIMPAK)), 1);
    state.value_damage.insert((doc, 29), 100.0);
    assert!(companions::heal_with_stimpak(&order, &mut state, doc));
    assert!(!state.value_damage.contains_key(&(doc, 29)));
}

/// Nerve (`00644ce0`, `009b5170`, `009b5a30`): a teammate's damage, and
/// their resistance and threshold when hit, × 1 + 0.05 × the player's
/// Charisma (kept within 1 to 10).
#[test]
fn nerve() {
    use world::combat;
    use world::companions;
    let (_data, order) = order("companions-nerve");
    let scripts = ScriptCache::default();
    let mut state = GameState::new(&order);
    let doc = FormId(DOC_REF);
    let player = world::dialogue::PLAYER_REF;
    state.actor_values.insert((player, 8), 6.0);
    assert_eq!(companions::nerve(&order, &state, doc), 1.0);
    let rifle = combat::Weapon::load(&order, FormId(RIFLE)).unwrap();
    let plain = combat::weapon_damage(&order, &state, doc, Some(&rifle), false);
    let plain_armour = combat::through_armour(&order, &state, 100.0, doc, None);
    let mut fists = state.clone();
    let plain_punch = Runner::new(&order, &scripts, &mut fists)
        .hit(doc, FormId(GECKO_REF), None)
        .unwrap();

    state.teammates.insert(doc);
    assert!((companions::nerve(&order, &state, doc) - 1.3).abs() < 1e-6);
    // The player isn't their own teammate.
    assert_eq!(companions::nerve(&order, &state, player), 1.0);
    let nervy = combat::weapon_damage(&order, &state, doc, Some(&rifle), false);
    assert!((nervy - plain * 1.3).abs() < 1e-3, "{plain} {nervy}");
    // Fists: `00644ce0`'s Nerve and `009b5170`'s (the same dice).
    let mut fists = state.clone();
    let punch = Runner::new(&order, &scripts, &mut fists)
        .hit(doc, FormId(GECKO_REF), None)
        .unwrap();
    assert!(
        (punch - plain_punch * 1.3 * 1.3).abs() < 1e-3,
        "{plain_punch} {punch}"
    );

    // Hit: resistance 20 and threshold 4 become 26 and 5.2.
    state.actor_values.insert((doc, 18), 20.0);
    state.actor_values.insert((doc, 76), 4.0);
    let hit = combat::through_armour(&order, &state, 100.0, doc, None);
    assert!((hit - (100.0 * (1.0 - 0.26) - 5.2)).abs() < 1e-3, "{hit}");
    state.teammates.remove(&doc);
    let hit = combat::through_armour(&order, &state, 100.0, doc, None);
    assert!((hit - 76.0).abs() < 1e-3, "{hit} {plain_armour}");

    // Charisma within 1 to 10.
    state.teammates.insert(doc);
    state.actor_values.insert((player, 8), 0.0);
    assert!((companions::nerve(&order, &state, doc) - 1.05).abs() < 1e-6);
    state.actor_values.insert((player, 8), 14.0);
    assert!((companions::nerve(&order, &state, doc) - 1.5).abs() < 1e-6);
}

/// After trading (`0075b750` → `00606540` → `006047c0`): per body slot
/// the best armour (`004c8220`: resistance + threshold, the first on a
/// tie), not one covering the upper body outside its own slot, and what's
/// locked on with `EquipItem`'s flag kept.
#[test]
fn a_companion_picks_their_armour() {
    use world::companions;
    let (_data, order) = order("companions-armour");
    let scripts = ScriptCache::default();
    let mut state = GameState::new(&order);
    let doc = FormId(DOC_REF);
    state.stock(&order, doc);
    for item in [SHIRT, COAT, HAT, ARMOR, HELMET] {
        state.items.insert((doc, FormId(item)), 1);
    }
    assert_eq!(
        companions::best_armour(&order, &state, doc, 2),
        Some(FormId(ARMOR))
    );
    assert_eq!(
        companions::best_armour(&order, &state, doc, 0),
        Some(FormId(HELMET))
    );
    // The coat covers the upper body and a hand: only the upper body's
    // pick puts such a thing on.
    assert_eq!(
        companions::best_armour(&order, &state, doc, 3),
        Some(FormId(COAT))
    );
    // Slot by slot: the helmet (0), the armour (2), the hat (10).
    assert_eq!(
        companions::wear_best_armour(&order, &mut state, doc),
        vec![FormId(HELMET), FormId(ARMOR), FormId(HAT)]
    );
    assert!(!state.is_equipped(doc, FormId(COAT)));
    // Again: nothing new.
    assert!(companions::wear_best_armour(&order, &mut state, doc).is_empty());

    // The shirt locked on (`EquipItem TestShirt 1`) stays.
    Runner::new(&order, &scripts, &mut state).run_source(
        "DocRef.EquipItem TestShirt 1",
        None,
        None,
    );
    assert!(state.is_equipped(doc, FormId(SHIRT)));
    assert_eq!(
        companions::best_armour(&order, &state, doc, 2),
        Some(FormId(SHIRT))
    );
    companions::wear_best_armour(&order, &mut state, doc);
    assert!(state.is_equipped(doc, FormId(SHIRT)));
    assert!(!state.is_equipped(doc, FormId(ARMOR)));
    // Without the flag it's picked again.
    Runner::new(&order, &scripts, &mut state).run_source("DocRef.EquipItem TestShirt", None, None);
    companions::wear_best_armour(&order, &mut state, doc);
    assert!(state.is_equipped(doc, FormId(ARMOR)));
}

/// Coming along when the player is put somewhere (`0093c200` →
/// `00973de0` → `008ad1c0`): a teammate near the player comes (not one
/// waiting); someone else only when following the player.
#[test]
fn teammates_come_along() {
    use world::companions;
    let (_data, order) = order("companions-along");
    let scripts = ScriptCache::default();
    let mut state = GameState::new(&order);
    let doc = FormId(DOC_REF);
    // The player somewhere else entirely.
    let elsewhere = FormId(0xFFF0);
    state.player_cell = Some(elsewhere);
    state.player_world = None;
    state.player_position = Some([10.0, 20.0, 30.0]);
    state.player_heading = 1.5;
    // Not a teammate, not following: stays.
    assert!(companions::come_along(&order, &scripts, &mut state, &[doc]).is_empty());
    // A teammate waiting (`Waiting` 1): stays.
    state.teammates.insert(doc);
    companions::set_variable(&order, &scripts, &mut state, doc, "Waiting", 1.0);
    assert!(companions::come_along(&order, &scripts, &mut state, &[doc]).is_empty());
    // Following again: comes, to where the player stands.
    companions::set_variable(&order, &scripts, &mut state, doc, "Waiting", 0.0);
    assert_eq!(
        companions::come_along(&order, &scripts, &mut state, &[doc]),
        vec![doc]
    );
    assert_eq!(
        state.place(&order, doc),
        Some((elsewhere, elsewhere, [10.0, 20.0, 30.0], 1.5))
    );
    assert!(state.evaluate.contains(&doc));
    // Dead: no.
    state.dead.insert(doc);
    state.player_position = Some([5000.0, 0.0, 0.0]);
    assert!(companions::come_along(&order, &scripts, &mut state, &[doc]).is_empty());
}

/// Ammunition (`Actor::ShouldUseAmmo`, `Actor::UseAmmo`, carried out by
/// `world::npc_combat`): a teammate's shots use theirs up; someone else's
/// don't.
#[test]
fn a_teammates_shots_use_ammunition() {
    use world::npc_combat::{fired, should_use_ammo};
    let (_data, order) = order("companions-ammo");
    let mut state = GameState::new(&order);
    let doc = FormId(DOC_REF);
    // The test pistol firing the cased rounds.
    let mut pistol = world::combat::Weapon::load(&order, FormId(PISTOL)).unwrap();
    let ammo = FormId(CASED_AMMO);
    pistol.ammo = vec![ammo];
    state.stock(&order, doc);
    state.items.insert((doc, ammo), 10);
    assert!(!should_use_ammo(&state, doc, &pistol));
    fired(&order, &mut state, doc, &pistol);
    assert_eq!(state.item_count(&order, doc, ammo), 10);
    state.teammates.insert(doc);
    assert!(should_use_ammo(&state, doc, &pistol));
    let per = i32::from(pistol.ammo_use.max(1));
    fired(&order, &mut state, doc, &pistol);
    assert_eq!(state.item_count(&order, doc, ammo), 10 - per);
    // A weapon not playable: not even a teammate's.
    let mut npc_gun = pistol.clone();
    npc_gun.flags1 |= 0x80;
    assert!(!should_use_ammo(&state, doc, &npc_gun));
    // "NPCs use ammo": anyone's.
    npc_gun.flags2 |= 0x02;
    state.teammates.remove(&doc);
    assert!(should_use_ammo(&state, doc, &npc_gun));
    assert!(should_use_ammo(
        &state,
        world::dialogue::PLAYER_REF,
        &pistol
    ));
}

/// A fall (`008a62b0`): the player's teammate takes no damage from it.
#[test]
fn teammates_take_no_fall_damage() {
    let (_data, order) = order("companions-fall");
    let mut state = GameState::new(&order);
    let doc = FormId(DOC_REF);
    assert!(world::combat::land(&order, &mut state, doc, 1000.0) > 0.0);
    state.damage.remove(&doc);
    state.dead.remove(&doc);
    state.teammates.insert(doc);
    assert_eq!(world::combat::land(&order, &mut state, doc, 1000.0), 0.0);
    assert!(!state.damage.contains_key(&doc));
}

/// Knocked out (`00888b50`): a teammate's time down stops while the
/// player fights; in Hardcore a teammate isn't essential (`0087f3d0`).
#[test]
fn a_knocked_out_teammate_waits_for_the_fight_to_end() {
    use world::companions;
    let (_data, order) = order("companions-down");
    let mut state = GameState::new(&order);
    let doc = FormId(DOC_REF);
    let gecko = FormId(GECKO_REF);
    assert!(companions::down_time_runs(&state, doc));
    state.combat.insert(gecko, world::dialogue::PLAYER_REF);
    assert!(companions::down_time_runs(&state, doc));
    state.teammates.insert(doc);
    assert!(!companions::down_time_runs(&state, doc));
    // Down in the fight: the time stands still (`combat::advance_down`).
    state.more.down.insert(doc, 10.0);
    world::combat::advance_down(&order, &mut state, 60.0);
    assert!(state.more.down.contains_key(&doc));
    state.combat.clear();
    assert!(companions::down_time_runs(&state, doc));
    world::combat::advance_down(&order, &mut state, 9.9);
    assert!(state.more.down.contains_key(&doc));
    world::combat::advance_down(&order, &mut state, 0.2);
    assert!(!state.more.down.contains_key(&doc));
    assert!(world::more_functions::is_essential(&order, &state, doc));
    state.living.hardcore = true;
    assert!(!world::more_functions::is_essential(&order, &state, doc));
}

/// Sneaking with the player (`008a0d10`): a teammate out of a fight
/// isn't detected while the player sneaks out of one, and sneaks with the
/// player's skill when it's better.
#[test]
fn a_teammate_sneaks_with_the_player() {
    use world::companions;
    let (_data, order) = order("companions-sneak");
    let mut state = GameState::new(&order);
    let doc = FormId(DOC_REF);
    let gecko = FormId(GECKO_REF);
    let player = world::dialogue::PLAYER_REF;
    state.teammates.insert(doc);
    assert!(!companions::hidden_with_player(&state, doc));
    state.player_sneaking = true;
    assert!(companions::hidden_with_player(&state, doc));
    let facts = |state: &GameState| {
        world::scripting::Facts {
            order: &order,
            state,
            speaker: None,
        }
        .detection(gecko, doc)
    };
    assert_eq!(facts(&state), Some(-100));
    // Fighting: seen as usual.
    state.combat.insert(gecko, player);
    assert!(!companions::hidden_with_player(&state, doc));
    state.combat.clear();
    // The player's Sneak, when better.
    state.player_sneaking = false;
    state.actor_values.insert((doc, 42), 10.0);
    state.actor_values.insert((player, 42), 80.0);
    let f = world::scripting::Facts {
        order: &order,
        state: &state,
        speaker: None,
    };
    let i = f
        .detection_inputs(gecko, doc, true, None)
        .expect("in one place");
    assert_eq!(i.sneak, 80.0);
}

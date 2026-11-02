//! What companions pick to fight with and the numbers on the item cards
//! (`world::dps`, `world::item_card`, `world::companions::sort_out_gear`)
//! on the generated world `testdata::companion_gear`, whose records carry
//! the master's values (the 9mm pistol, Cass's shotgun and knife, the 10mm
//! submachine gun, ED-E's zap guns, the Stimpak, Nuka-Cola).

use std::sync::Arc;

use esm::{ActivePlugins, FormId, LoadOrder};
use nif::anim::Sequence;
use testdata::companion_gear::ids::*;
use world::animation::groups::AnimSet;
use world::animation::pick::Library;
use world::combat::Weapon;
use world::companions;
use world::dialogue::PLAYER_REF;
use world::dps::{self, DpsCall};
use world::item_card;
use world::scripting::{game_setting, GameState};

fn order(tag: &str) -> (testdata::TempData, LoadOrder) {
    let data = testdata::companion_gear::companion_gear(tag);
    let order = LoadOrder::from_data_dir(data.path(), &ActivePlugins::OfficialOnly).unwrap();
    (data, order)
}

/// The player's third-person attack animations as the game's files have
/// them (`_male\1hpattackright.kf`: `a:R` at 0.4 s; `_male\
/// 2hrattackleft.kf`: `a:L` at 0.4667 s).
struct PlayerAnims {
    set: AnimSet,
}

fn sequence(name: &str, a: f32) -> Arc<Sequence> {
    Arc::new(Sequence {
        name: name.into(),
        start: 0.0,
        stop: 0.5,
        looping: false,
        tracks: Vec::new(),
        accum_root: None,
        materials: Vec::new(),
        text_keys: vec![
            (0.0, "start".into()),
            (0.0333, "Hit".into()),
            (a, if name == "AttackRight" { "a:R" } else { "a:L" }.into()),
            (0.4667, "BlendIn:1".into()),
            (0.5, "end".into()),
        ],
    })
}

impl PlayerAnims {
    fn new() -> PlayerAnims {
        let mut set = AnimSet::default();
        set.add(r"meshes\characters\_male\1hpattackright.kf", "AttackRight");
        set.add(r"meshes\characters\_male\2hrattackleft.kf", "AttackLeft");
        PlayerAnims { set }
    }
}

impl Library for PlayerAnims {
    fn set(&self) -> &AnimSet {
        &self.set
    }
    fn sequence(&mut self, id: u16) -> Option<Arc<Sequence>> {
        let path = self.set.file(id, 0)?;
        Some(if path.contains("1hp") {
            sequence("AttackRight", 0.4)
        } else {
            sequence("AttackLeft", 0.4667)
        })
    }
}

fn close(a: f32, b: f32) -> bool {
    (a - b).abs() < 1e-3 * b.abs().max(1.0)
}

/// The action times of an attack's text keys (`005f3a20`): `Start`,
/// `Hit`, `a:` skipping `Eject`, `End`; a reload's `Start` and `End`.
#[test]
fn an_animation_groups_action_times() {
    let keys: Vec<(f32, String)> = [
        (0.0, "start"),
        (0.0333, "Hit"),
        (0.4, "a:R"),
        (0.4667, "BlendIn:1"),
        (0.5, "end"),
    ]
    .iter()
    .map(|&(t, s)| (t, s.to_string()))
    .collect();
    let times = world::animation::action_times(world::animation::group::ATTACK_RIGHT, &keys);
    assert_eq!(times, [0.0, 0.0333, 0.0, 0.4, 0.5]);
    // With an `Eject` key it's the third action.
    let mut eject = keys.clone();
    eject.insert(2, (0.2, "Eject".into()));
    let times = world::animation::action_times(world::animation::group::ATTACK_RIGHT, &eject);
    assert_eq!(times, [0.0, 0.0333, 0.2, 0.4, 0.5]);
    // A reload: start and end only; an attack's names don't count.
    let reload = world::animation::action_times(
        world::animation::group::RELOAD_A,
        &[
            (0.0, "Start".into()),
            (0.5, "Hit".into()),
            (1.6, "End".into()),
        ],
    );
    assert_eq!(reload, [0.0, 1.6, 0.0, 0.0, 0.0]);
}

/// The shots a second with an inventory entry (`00645dc0`): a
/// semi-automatic's off the player's attack animation (1 ÷ 0.4 s × speed
/// 1 × the 9mm's attack multiplier 1.25 = the record's own 3.125); without
/// animations, and for automatics, the fire rate × the attack multiplier
/// twice.
#[test]
fn shots_a_second_from_the_players_animations() {
    let (_data, order) = order("cw-shots");
    let pistol = Weapon::load(&order, FormId(PISTOL)).unwrap();
    let mut anims = PlayerAnims::new();
    assert!(close(
        dps::shots_per_second(&pistol, Some(&mut anims)),
        3.125
    ));
    assert!(close(dps::shots_per_second(&pistol, None), 1.5625));
    let shotgun = Weapon::load(&order, FormId(CASS_SHOTGUN)).unwrap();
    assert!(close(
        dps::shots_per_second(&shotgun, Some(&mut anims)),
        1.0 / 0.4667 * 1.5
    ));
    let smg = Weapon::load(&order, FormId(SMG)).unwrap();
    assert!(close(dps::shots_per_second(&smg, Some(&mut anims)), 9.0));
    // A group the animations lack: no rate.
    let mut none = PlayerAnims {
        set: AnimSet::default(),
    };
    assert_eq!(dps::shots_per_second(&pistol, Some(&mut none)), 0.0);
}

/// The ITEMS card's damage a second for the player's 9mm pistol
/// (`00707e30` → `00645380`): 16 × (0.5 + 0.5 × Guns 50 ÷ 100) = 12 a
/// shot, plus the critical share 5% × 16 = 0.8, × 3.125 shots a second =
/// 40; worn to half its condition the shot loses (0.75 − 0.5) × 0.67 of
/// itself; the combat controller's figure for someone else is the same
/// here (no semi-automatic delay beyond its attack, no burst).
#[test]
fn the_cards_damage_a_second() {
    let (_data, order) = order("cw-card");
    let s = |n: &str, d: f32| game_setting(&order, n).unwrap_or(d);
    let mut state = GameState::new(&order);
    state.actor_values.insert((PLAYER_REF, 41), 50.0);
    state.actor_values.insert((PLAYER_REF, 14), 5.0);
    state.items.insert((PLAYER_REF, FormId(PISTOL)), 1);
    state.items.insert((PLAYER_REF, FormId(AMMO_9MM)), 30);
    let pistol = Weapon::load(&order, FormId(PISTOL)).unwrap();
    let mut anims = PlayerAnims::new();
    let d = item_card::card_dps(&order, &state, &pistol, Some(&mut anims), &s);
    assert!(close(d, 40.0), "{d}");
    // Without the player's animations the rate is the fire rate's.
    let d = item_card::card_dps(&order, &state, &pistol, None, &s);
    assert!(close(d, 20.0), "{d}");
    state
        .weapon_health
        .insert((PLAYER_REF, FormId(PISTOL)), 0.5);
    let d = item_card::card_dps(&order, &state, &pistol, Some(&mut anims), &s);
    assert!(close(d, (0.8 + 12.0 * (1.0 - 0.25 * 0.67)) * 3.125), "{d}");
    // Broken: nothing.
    state
        .weapon_health
        .insert((PLAYER_REF, FormId(PISTOL)), 0.0);
    assert_eq!(
        item_card::card_dps(&order, &state, &pistol, Some(&mut anims), &s),
        0.0
    );
    let cass = FormId(CASS_REF);
    state.actor_values.insert((cass, 41), 50.0);
    state.actor_values.insert((cass, 14), 5.0);
    let d = dps::weapon_dps(
        &order,
        &state,
        &DpsCall::controller(cass, Some(&pistol), 1.0),
        None,
        &s,
    );
    assert!(close(d, 40.0), "{d}");
}

fn cass_state(order: &LoadOrder) -> GameState {
    let mut state = GameState::new(order);
    let cass = FormId(CASS_REF);
    state.stock(order, cass);
    state.actor_values.insert((cass, 41), 60.0);
    state.actor_values.insert((cass, 38), 40.0);
    state.actor_values.insert((cass, 14), 5.0);
    state.equip(order, cass, FormId(CASS_SHOTGUN));
    state
}

/// Cass after trading (`0075b750` → `006047c0`'s weapon part →
/// `004c7400`): with her ranged style she keeps her shotgun (44.8 a
/// second at Guns 60: 54 × 0.8 + 5% × 8, two rounds, the reload) over a
/// 9mm pistol rated at the fire rate (21.25); with the player's
/// animations the pistol's rate is its 3.125 and it rates 42.5 against
/// the shotgun's 41.1, so she takes it; the 10mm submachine gun (68.9:
/// 9 a second, bursts) beats both; her knife only once the melee style
/// puts guns down to a ten-thousandth.
#[test]
fn cass_picks_her_weapon_after_trading() {
    let (_data, order) = order("cw-cass");
    let s = |n: &str, d: f32| game_setting(&order, n).unwrap_or(d);
    let cass = FormId(CASS_REF);
    let mut state = cass_state(&order);
    let shotgun = Weapon::load(&order, FormId(CASS_SHOTGUN)).unwrap();
    let style = world::more_functions::combat_style(&order, &state, cass);
    assert_eq!(style.weapon_restrictions, 2);
    let call = DpsCall {
        who: cass,
        weapon: Some(&shotgun),
        condition: 1.0,
        perks: false,
        entry: true,
        ammo: Some(FormId(AMMO_COMPANION)),
    };
    let d = dps::combat_weapon_dps(&order, &state, &call, &style, None, &s);
    assert!(close(d, (0.4 + 43.2) * 2.0 / (2.0 / 4.5 + 1.5)), "{d}");

    // The player gives her a 9mm pistol and rounds.
    state.items.insert((cass, FormId(PISTOL)), 1);
    state.items.insert((cass, FormId(AMMO_9MM)), 20);
    assert_eq!(
        companions::sort_out_gear(&order, &mut state, cass, None, &s),
        vec![]
    );
    assert!(state.is_equipped(cass, FormId(CASS_SHOTGUN)));
    let mut anims = PlayerAnims::new();
    assert_eq!(
        companions::sort_out_gear(&order, &mut state, cass, Some(&mut anims), &s),
        vec![FormId(PISTOL)]
    );
    assert!(!state.is_equipped(cass, FormId(CASS_SHOTGUN)));
    // Without rounds for it the pistol can't be picked.
    state.items.insert((cass, FormId(AMMO_9MM)), 0);
    assert_eq!(
        companions::hold_best_weapon(&order, &mut state, cass, Some(&mut anims), &s),
        Some(FormId(CASS_SHOTGUN))
    );
    // The submachine gun and its rounds.
    state.items.insert((cass, FormId(SMG)), 1);
    state.items.insert((cass, FormId(AMMO_10MM)), 60);
    assert_eq!(
        companions::hold_best_weapon(&order, &mut state, cass, Some(&mut anims), &s),
        Some(FormId(SMG))
    );
    // The wheel's Melee switch: `SetCombatStyle FollowersCombatStyleMelee`.
    state.more.combat_styles.insert(cass, FormId(STYLE_MELEE));
    assert_eq!(
        companions::hold_best_weapon(&order, &mut state, cass, Some(&mut anims), &s),
        Some(FormId(CASS_KNIFE))
    );
    assert!(!state.is_equipped(cass, FormId(SMG)));
}

/// What `GetBestWeapon` passes over (`004c7400`, `008bc9d0`): a "player
/// only" weapon, one worn to nothing; a weapon a script locked in their
/// hands stays.
#[test]
fn weapons_never_picked_and_locked_ones() {
    let (_data, order) = order("cw-rules");
    let s = |n: &str, d: f32| game_setting(&order, n).unwrap_or(d);
    let cass = FormId(CASS_REF);
    let mut state = cass_state(&order);
    let player_only = Weapon::load(&order, FormId(PLAYER_ONLY)).unwrap();
    assert!(!dps::can_use_weapon(&order, cass, &player_only));
    state.items.insert((cass, FormId(PLAYER_ONLY)), 1);
    state.items.insert((cass, FormId(AMMO_9MM)), 20);
    state.items.insert((cass, FormId(SMG)), 1);
    state.items.insert((cass, FormId(AMMO_10MM)), 60);
    state.weapon_health.insert((cass, FormId(SMG)), 0.0);
    assert_eq!(
        dps::best_weapon(&order, &state, cass, None, true, None, &s),
        Some(FormId(CASS_SHOTGUN))
    );
    // Repaired, it would be picked; locked on, the shotgun stays.
    state.weapon_health.insert((cass, FormId(SMG)), 1.0);
    state.equip_locked.insert((cass, FormId(CASS_SHOTGUN)));
    assert_eq!(
        companions::hold_best_weapon(&order, &mut state, cass, None, &s),
        None
    );
    assert!(state.is_equipped(cass, FormId(CASS_SHOTGUN)));
    assert_eq!(
        dps::best_weapon(&order, &state, cass, None, false, None, &s),
        Some(FormId(SMG))
    );
    // One kind only: the best melee weapon.
    assert_eq!(
        dps::best_weapon(
            &order,
            &state,
            cass,
            Some(world::npc_combat::kind::MELEE),
            false,
            None,
            &s
        ),
        Some(FormId(CASS_KNIFE))
    );
}

/// ED-E (`TESCreature::InitDefaultWorn`, `005f9e00`): a creature holds
/// only weapons in its record's weapon list (`LNAM` → `EmbeddedWeapons`,
/// `008bc9d0`), the best of them: the upgraded zap gun once given it.
#[test]
fn ede_holds_the_best_of_its_own_weapons() {
    let (_data, order) = order("cw-ede");
    let s = |n: &str, d: f32| game_setting(&order, n).unwrap_or(d);
    let ede = FormId(EDE_REF);
    let mut state = GameState::new(&order);
    state.stock(&order, ede);
    state.actor_values.insert((ede, 34), 50.0);
    state.actor_values.insert((ede, 41), 50.0);
    state.actor_values.insert((ede, 14), 5.0);
    assert_eq!(
        dps::creature_weapon_list(&order, ede),
        vec![FormId(ZAP), FormId(ZAP_UPGRADE)]
    );
    let smg = Weapon::load(&order, FormId(SMG)).unwrap();
    assert!(!dps::can_use_weapon(&order, ede, &smg));
    assert_eq!(
        companions::sort_out_gear(&order, &mut state, ede, None, &s),
        vec![FormId(ZAP)]
    );
    state.items.insert((ede, FormId(SMG)), 1);
    state.items.insert((ede, FormId(AMMO_10MM)), 60);
    assert_eq!(
        companions::sort_out_gear(&order, &mut state, ede, None, &s),
        vec![]
    );
    state.items.insert((ede, FormId(ZAP_UPGRADE)), 1);
    assert_eq!(
        companions::sort_out_gear(&order, &mut state, ede, None, &s),
        vec![FormId(ZAP_UPGRADE)]
    );
    assert!(!state.is_equipped(ede, FormId(ZAP)));
}

/// A person whose package has "Weapons Unequipped" (`00606540`,
/// `00441b00`) sorts out their clothes but not their weapon.
#[test]
fn weapons_unequipped_packages_pick_no_weapon() {
    let (_data, order) = order("cw-package");
    let s = |n: &str, d: f32| game_setting(&order, n).unwrap_or(d);
    let guard = FormId(GUARD_REF);
    let mut state = GameState::new(&order);
    state.stock(&order, guard);
    assert!(world::ai::current_package(&order, &state, guard)
        .is_some_and(|p| p.flags & companions::WEAPONS_UNEQUIPPED != 0));
    assert!(companions::sort_out_gear(&order, &mut state, guard, None, &s).is_empty());
    assert!(!state.is_equipped(guard, FormId(PISTOL)));
    assert_eq!(
        companions::hold_best_weapon(&order, &mut state, guard, None, &s),
        Some(FormId(PISTOL))
    );
}

/// The cards' effects text (`00406620`, `00503a70`, `00707e30`).
#[test]
fn the_cards_effects_text() {
    let (_data, order) = order("cw-effects");
    let mut state = GameState::new(&order);
    let text = |state: &GameState, item: u32| item_card::card_effects(&order, state, FormId(item));
    state.actor_values.insert((PLAYER_REF, 20), 0.0);
    // A medicine: 30 × (1 + 2 × Medicine 15 ÷ 100) = 39.
    state.actor_values.insert((PLAYER_REF, 37), 15.0);
    assert_eq!(text(&state, STIMPAK).as_deref(), Some("HP +39"));
    state.actor_values.insert((PLAYER_REF, 37), 50.0);
    assert_eq!(text(&state, STIMPAK).as_deref(), Some("HP +60"));
    // Latest added first; rads written positive; a duration in seconds;
    // the script effect has no actor value and isn't written.
    assert_eq!(
        text(&state, NUKA_COLA).as_deref(),
        Some("Rads +3, HP +2(25s)")
    );
    // Rad Resistance 50: 3 × 0.5 rounded down.
    state.actor_values.insert((PLAYER_REF, 20), 50.0);
    assert_eq!(
        text(&state, NUKA_COLA).as_deref(),
        Some("Rads +1, HP +2(25s)")
    );
    state.actor_values.insert((PLAYER_REF, 20), 0.0);
    // A food: Survival 50 doubles its health but not its rads.
    state.actor_values.insert((PLAYER_REF, 44), 50.0);
    assert_eq!(text(&state, FOOD).as_deref(), Some("Rads +2, HP +20"));
    // A name alone first; minutes; a recovering effect has no duration.
    assert_eq!(
        text(&state, CHEM).as_deref(),
        Some("Radiation Immunity, HP +1(1.5m), PER +2")
    );
    // Enchanted apparel, unless the enchantment hides its effect.
    assert_eq!(text(&state, HAT).as_deref(), Some("PER +1"));
    assert_eq!(text(&state, HIDDEN_HAT), None);
    // A weapon mod's description; ammunition's effects.
    assert_eq!(
        text(&state, WEAPON_MOD).as_deref(),
        Some("Increases the 9mm pistol's magazine.")
    );
    assert_eq!(text(&state, AMMO_20GA).as_deref(), Some("DAM x 1.20"));
    assert_eq!(text(&state, AMMO_9MM), None);
    // A weapon without an enchantment has no effects card.
    assert_eq!(text(&state, PISTOL), None);
}

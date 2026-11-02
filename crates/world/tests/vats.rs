//! V.A.T.S. read from a plugin built from scratch (`testdata::vats`):
//! weapons' costs and bursts, action points coming back, the chance to hit
//! Doc Mitchell with a 9mm, perks only raising it, the parts and targets
//! offered, and what V.A.T.S. changes about hits.

use esm::{ActivePlugins, FormId, LoadOrder};
use testdata::vats::ids::*;
use world::body_parts::{part, BodyPartData};
use world::combat::{self, Weapon};
use world::dialogue::PLAYER_REF;
use world::scripting::{Facts, GameState, Runner, ScriptCache, Value};
use world::vats::{self, mode, AttackFacts, Candidate, ChanceQuery, Settings, Stance};

fn order(tag: &str) -> (testdata::TempData, LoadOrder) {
    let data = testdata::vats::vats(tag);
    let order = LoadOrder::from_data_dir(data.path(), &ActivePlugins::OfficialOnly).unwrap();
    (data, order)
}

fn weapon(order: &LoadOrder, id: u32) -> Weapon {
    Weapon::load(order, FormId(id)).unwrap()
}

/// The player holding a weapon, with Guns 50.
fn player_with(order: &LoadOrder, id: u32) -> GameState {
    let mut state = GameState::new(order);
    state.equipped.insert(PLAYER_REF, vec![FormId(id)]);
    state.actor_values.insert((PLAYER_REF, 41), 50.0);
    state
}

#[test]
fn costs_come_from_the_weapon_else_the_table_then_perks() {
    let (_data, order) = order("vats-costs");
    let s = Settings::load(&order);
    let pistol = weapon(&order, PISTOL);
    let extra = vats::WeaponVats::load(&order, FormId(PISTOL)).unwrap();
    assert_eq!((extra.to_hit, extra.ap), (15, 17.0));
    assert_eq!((extra.strength_req, extra.skill_req), (4, 25));
    let mut state = GameState::new(&order);
    let cost = |state: &GameState, w: Option<&Weapon>| vats::attack_cost(&order, state, &s, w);
    // Its own cost (second flags 0x08); the table for the SMG (a rifle's
    // 25), the machete (one-handed melee 15) and fists (22).
    assert_eq!(cost(&state, Some(&pistol)), 17.0);
    assert_eq!(cost(&state, Some(&weapon(&order, SMG))), 25.0);
    assert_eq!(cost(&state, Some(&weapon(&order, CARBINE))), 20.0);
    assert_eq!(cost(&state, Some(&weapon(&order, MACHETE))), 15.0);
    assert_eq!(cost(&state, None), 22.0);
    // Fast Shot: × 0.8 for Guns (its weapon tab), not for melee or fists.
    state.perks.insert(FormId(FAST_SHOT));
    assert!((cost(&state, Some(&pistol)) - 13.6).abs() < 1e-5);
    assert_eq!(cost(&state, Some(&weapon(&order, MACHETE))), 15.0);
    assert_eq!(cost(&state, None), 22.0);
}

#[test]
fn automatic_weapons_fire_bursts() {
    let (_data, order) = order("vats-bursts");
    let s = Settings::load(&order);
    let shots = |id: u32| vats::shots(&s, Some(&weapon(&order, id)));
    assert_eq!(shots(PISTOL), (1, 0));
    // 9 × 0.43 = 3.87 → 4; 12 × 0.43 → 5; 20 × 0.43 → 9, and 20 × 1.75
    // = 35 for a long burst: 26 more.
    assert_eq!(shots(SMG), (4, 0));
    assert_eq!(shots(CARBINE), (5, 0));
    assert_eq!(shots(MINIGUN), (9, 26));
}

#[test]
fn action_points_come_back_only_with_vats_off() {
    let (_data, order) = order("vats-ap");
    let mut state = GameState::new(&order);
    // 65 + 3 × Agility 5.
    assert_eq!(vats::max_action_points(&order, &state), 80.0);
    vats::spend(&mut state, 50.0);
    assert_eq!(vats::action_points(&order, &state), 30.0);
    // 6% of 80 a second.
    vats::regenerate(&order, &mut state, 5.0);
    assert!((vats::action_points(&order, &state) - 54.0).abs() < 1e-4);
    // Not while V.A.T.S. is on.
    state.vats.mode = mode::MENU;
    vats::regenerate(&order, &mut state, 5.0);
    assert!((vats::action_points(&order, &state) - 54.0).abs() < 1e-4);
    state.vats.mode = mode::OFF;
    // Time passing does it (the scripts' update), never past the most.
    let cache = ScriptCache::default();
    Runner::new(&order, &cache, &mut state).update(1.0);
    assert!((vats::action_points(&order, &state) - 58.8).abs() < 1e-3);
    vats::regenerate(&order, &mut state, 100.0);
    assert_eq!(vats::action_points(&order, &state), 80.0);
    // A perk's "Action Point Regen" (× 2 here) speeds it up.
    state.perks.insert(FormId(QUICK_RECOVERY));
    vats::spend(&mut state, 50.0);
    vats::regenerate(&order, &mut state, 1.0);
    assert!((vats::action_points(&order, &state) - 39.6).abs() < 1e-3);
    // Waiting an hour: all back.
    state.wait(&order, 1.0).unwrap();
    assert_eq!(vats::action_points(&order, &state), 80.0);
}

/// The chance of each of Doc's parts at `distance` with the player's
/// weapon, fully visible.
fn chances(order: &LoadOrder, state: &mut GameState, distance: f32) -> Vec<(String, u32)> {
    let s = Settings::load(order);
    let data = BodyPartData::of(order, FormId(DOC_REF)).unwrap();
    let held = combat::weapon_in_hand(order, state, PLAYER_REF);
    let entries = vats::parts_offered(Some(&data), None);
    entries
        .iter()
        .map(|e| {
            let chance = vats::part_chance(
                order,
                state,
                &s,
                &ChanceQuery {
                    target: FormId(DOC_REF),
                    part: data.part(e.slot as u8),
                    part_av: e.actor_value,
                    weapon: held.as_ref(),
                    distance,
                    visible: 1.0,
                    stance: Stance::SCANNING,
                },
            );
            (e.name.clone(), vats::shown_percent(&s, chance, 0, false))
        })
        .collect()
}

fn percent(list: &[(String, u32)], name: &str) -> u32 {
    list.iter().find(|(n, _)| n == name).unwrap().1
}

#[test]
fn the_findings_9mm_against_doc_mitchell() {
    let (_data, order) = order("vats-doc");
    let s = Settings::load(&order);
    assert!((vats::bound_of(&order, FormId(DOC_REF)) - 143.86).abs() < 0.01);
    let mut state = player_with(&order, PISTOL);
    // Guns 50, standing, aiming: wobble 0.1 × 0.75.
    let pistol = weapon(&order, PISTOL);
    let w = vats::wobble(
        &order,
        &state,
        &s,
        PLAYER_REF,
        Some(&pistol),
        Stance::SCANNING,
    );
    assert!((w - 0.075).abs() < 1e-6, "{w}");
    // 500 units: torso 95 (capped), head 52, arms 78, legs 87.
    let at_500 = chances(&order, &mut state, 500.0);
    assert_eq!(percent(&at_500, "Torso"), 95);
    assert_eq!(percent(&at_500, "Head"), 52);
    assert_eq!(percent(&at_500, "Left Arm"), 78);
    assert_eq!(percent(&at_500, "Right Leg"), 87);
    // 1500: torso 34, head 17.
    let at_1500 = chances(&order, &mut state, 1500.0);
    assert_eq!(percent(&at_1500, "Torso"), 34);
    assert_eq!(percent(&at_1500, "Head"), 17);
    // A new character's Guns (15, short of the pistol's 25): wobble
    // (0.1 + 1 × 0.025) × (1 − 0.075).
    state.actor_values.insert((PLAYER_REF, 41), 15.0);
    let w = vats::wobble(
        &order,
        &state,
        &s,
        PLAYER_REF,
        Some(&pistol),
        Stance::SCANNING,
    );
    assert!((w - 0.115_625).abs() < 1e-6, "{w}");
    // Not aiming adds 0.2 (outside V.A.T.S.).
    let loose = Stance {
        aiming: false,
        ..Stance::SCANNING
    };
    let w = vats::wobble(&order, &state, &s, PLAYER_REF, Some(&pistol), loose);
    assert!((w - 0.300_625).abs() < 1e-6, "{w}");
}

#[test]
fn perks_only_raise_the_chance() {
    let (_data, order) = order("vats-perks");
    let mut state = player_with(&order, PISTOL);
    let plain = chances(&order, &mut state, 1000.0);
    // A perk halving it changes nothing.
    state.perks.insert(FormId(CLUMSY));
    assert_eq!(chances(&order, &mut state, 1000.0), plain);
    state.perks.remove(&FormId(CLUMSY));
    // Sniper: the head (GetVATSValue 5 is 25) × 1.25; the torso as it was.
    state.perks.insert(FormId(SNIPER));
    let sniper = chances(&order, &mut state, 1000.0);
    let head = percent(&plain, "Head");
    assert_eq!(percent(&sniper, "Head"), (head as f32 * 1.25) as u32);
    assert_eq!(percent(&sniper, "Torso"), percent(&plain, "Torso"));
    // Gunslinger: a pistol is GetWeaponAnimType 4 → every part × 1.25;
    // with the SMG (an automatic rifle, 6) nothing.
    state.perks.remove(&FormId(SNIPER));
    state.perks.insert(FormId(GUNSLINGER));
    let slinger = chances(&order, &mut state, 1000.0);
    assert!(percent(&slinger, "Torso") > percent(&plain, "Torso"));
    state.equipped.insert(PLAYER_REF, vec![FormId(SMG)]);
    let smg_with = chances(&order, &mut state, 1000.0);
    state.perks.remove(&FormId(GUNSLINGER));
    assert_eq!(chances(&order, &mut state, 1000.0), smg_with);
    // The template is put back after asking.
    assert!(state.vats.attack.is_none());
}

#[test]
fn melee_mines_and_the_weapon_held() {
    let (_data, order) = order("vats-kinds");
    let s = Settings::load(&order);
    let mut state = player_with(&order, MACHETE);
    // Melee: 95 within 300 units of the edges, 0 beyond, whatever the part.
    let near = chances(&order, &mut state, 300.0);
    assert!(near.iter().all(|(_, p)| *p == 95));
    let far = chances(&order, &mut state, 301.0);
    assert!(far.iter().all(|(_, p)| *p == 0));
    // A mine's projectile goes off by proximity: no chance.
    state.equipped.insert(PLAYER_REF, vec![FormId(MINE)]);
    assert!(chances(&order, &mut state, 200.0)
        .iter()
        .all(|(_, p)| *p == 0));
    // The gunman's pistol is a part of its own, at the pistol's to-hit 15
    // (his body has no part for it).
    state.equipped.insert(PLAYER_REF, vec![FormId(PISTOL)]);
    let data = BodyPartData::of(&order, FormId(GUNMAN_REF)).unwrap();
    let held = combat::weapon_in_hand(&order, &state, FormId(GUNMAN_REF));
    let parts = vats::parts_offered(Some(&data), held.as_ref());
    assert_eq!(parts.len(), 7);
    let gun = parts.last().unwrap();
    assert_eq!((gun.slot, gun.actor_value), (part::WEAPON as i8, -1));
    let pistol = weapon(&order, PISTOL);
    fn query<'a>(
        part: Option<&'a world::body_parts::BodyPart>,
        av: i32,
        pistol: &'a Weapon,
    ) -> ChanceQuery<'a> {
        ChanceQuery {
            target: FormId(GUNMAN_REF),
            part,
            part_av: av,
            weapon: Some(pistol),
            distance: 500.0,
            visible: 1.0,
            stance: Stance::SCANNING,
        }
    }
    let on_gun = vats::part_chance(&order, &mut state, &s, &query(None, -1, &pistol));
    let head = query(data.part(part::HEAD), 25, &pistol);
    let on_head = vats::part_chance(&order, &mut state, &s, &head);
    // Both scale the same circle: 15 against the head's 30.
    assert!((on_gun / on_head - 0.5).abs() < 1e-4, "{on_gun} {on_head}");
    // Doc holds nothing: no weapon part.
    let doc = BodyPartData::of(&order, FormId(DOC_REF)).unwrap();
    assert_eq!(vats::parts_offered(Some(&doc), None).len(), 6);
}

#[test]
fn who_can_be_targeted_and_when_vats_opens() {
    let (_data, order) = order("vats-targets");
    let s = Settings::load(&order);
    let c = |reference: u32, distance: f32, on_screen: bool| Candidate {
        reference: FormId(reference),
        alive: true,
        distance,
        line_of_sight: true,
        on_screen,
    };
    assert!(vats::eligible(&order, &s, &c(DOC_REF, 500.0, true), true));
    // Banned bases never.
    assert!(!vats::eligible(
        &order,
        &s,
        &c(BANNED_REF, 500.0, true),
        true
    ));
    // Off screen: only within the sneak distance (2500 indoors, 5000 out).
    assert!(vats::eligible(&order, &s, &c(DOC_REF, 2400.0, false), true));
    assert!(!vats::eligible(
        &order,
        &s,
        &c(DOC_REF, 2600.0, false),
        true
    ));
    assert!(vats::eligible(
        &order,
        &s,
        &c(DOC_REF, 2600.0, false),
        false
    ));
    // Too near, too far, hidden, dead.
    assert!(!vats::eligible(&order, &s, &c(DOC_REF, 1.0, true), true));
    assert!(!vats::eligible(
        &order,
        &s,
        &c(DOC_REF, 5001.0, true),
        false
    ));
    let hidden = Candidate {
        line_of_sight: false,
        ..c(DOC_REF, 500.0, true)
    };
    assert!(!vats::eligible(&order, &s, &hidden, true));
    // Opening: a banned weapon, the fighting controls off.
    let mut state = GameState::new(&order);
    let banned = weapon(&order, BANNED_GUN);
    assert_eq!(
        vats::can_enter(&order, &state, Some(&banned), false, false),
        Err(vats::EnterRefusal::BannedWeapon)
    );
    assert!(vats::can_enter(&order, &state, Some(&weapon(&order, PISTOL)), false, false).is_ok());
    state.controls_off[world::scripting::controls::FIGHTING] = true;
    assert_eq!(
        vats::can_enter(&order, &state, None, false, false),
        Err(vats::EnterRefusal::Busy)
    );
}

#[test]
fn hits_while_vats_plays() {
    let (_data, order) = order("vats-hits");
    let mut state = player_with(&order, PISTOL);
    let pistol = weapon(&order, PISTOL);
    state.actor_values.insert((PLAYER_REF, 11), 5.0);
    // Luck 5: 5% (50 per mille); a queued V.A.T.S. hit playing: + 5.
    assert!(!combat::critical(
        &order,
        &state,
        PLAYER_REF,
        Some(&pistol),
        FormId(DOC_REF),
        false,
        70
    ));
    let playing = AttackFacts {
        weapon: Some(FormId(PISTOL)),
        target: FormId(DOC_REF),
        part_av: 25,
        kind: vats::kind::PISTOL,
        hit: true,
        distance: 500.0,
        paralyzing_palm: false,
    };
    vats::begin_attack(&mut state, playing.clone());
    assert!(combat::critical(
        &order,
        &state,
        PLAYER_REF,
        Some(&pistol),
        FormId(DOC_REF),
        false,
        70
    ));
    // A queued miss doesn't carry it.
    vats::begin_attack(
        &mut state,
        AttackFacts {
            hit: false,
            ..playing.clone()
        },
    );
    assert!(!combat::critical(
        &order,
        &state,
        PLAYER_REF,
        Some(&pistol),
        FormId(DOC_REF),
        false,
        70
    ));
    // The player takes three quarters while V.A.T.S. is on.
    let cache = ScriptCache::default();
    let mut calm = GameState::new(&order);
    let full = Runner::new(&order, &cache, &mut calm)
        .hit_at(FormId(GUNMAN_REF), PLAYER_REF, Some(&pistol), None)
        .unwrap()
        .dealt;
    let mut on = GameState::new(&order);
    vats::set_mode(&mut on, mode::MENU);
    let less = Runner::new(&order, &cache, &mut on)
        .hit_at(FormId(GUNMAN_REF), PLAYER_REF, Some(&pistol), None)
        .unwrap()
        .dealt;
    assert!((less / full - 0.75).abs() < 1e-4, "{full} {less}");
    // The machete's special: × its own damage multiplier (2).
    let machete = weapon(&order, MACHETE);
    vats::begin_attack(
        &mut state,
        AttackFacts {
            kind: vats::kind::SPECIAL,
            ..playing
        },
    );
    assert_eq!(
        vats::attack_damage_mult(&order, &state, PLAYER_REF, Some(&machete)),
        2.0
    );
    assert_eq!(
        vats::attack_damage_mult(&order, &state, PLAYER_REF, Some(&pistol)),
        1.0
    );
}

#[test]
fn conditions_ask_about_the_attack() {
    let (_data, order) = order("vats-conditions");
    let mut state = player_with(&order, PISTOL);
    let value = |state: &GameState, name: &str, on: Option<FormId>, args: &[Value]| {
        let facts = Facts {
            order: &order,
            state,
            speaker: None,
        };
        let n = world::functions::FUNCTION_NAMES
            .iter()
            .position(|f| *f == name)
            .unwrap() as u16;
        facts.value(n, on, args)
    };
    assert_eq!(value(&state, "GetVATSMode", None, &[]), Some(0.0));
    assert_eq!(
        value(&state, "GetWeaponAnimType", Some(PLAYER_REF), &[]),
        Some(4.0)
    );
    let n = |x: f64| Value::Number(x);
    // No attack: 0.
    assert_eq!(
        value(&state, "GetVATSValue", None, &[n(5.0), n(25.0)]),
        Some(0.0)
    );
    vats::begin_attack(
        &mut state,
        AttackFacts {
            weapon: Some(FormId(PISTOL)),
            target: FormId(DOC_REF),
            part_av: 25,
            kind: vats::kind::PISTOL,
            hit: true,
            distance: 480.0,
            paralyzing_palm: false,
        },
    );
    assert_eq!(value(&state, "GetVATSMode", None, &[]), Some(4.0));
    let ask = |state: &GameState, k: f64, p: f64| value(state, "GetVATSValue", None, &[n(k), n(p)]);
    assert_eq!(ask(&state, 5.0, 25.0), Some(1.0));
    assert_eq!(ask(&state, 5.0, 26.0), Some(0.0));
    assert_eq!(ask(&state, 0.0, f64::from(PISTOL)), Some(1.0));
    assert_eq!(ask(&state, 2.0, f64::from(DOC)), Some(1.0));
    assert_eq!(ask(&state, 4.0, 0.0), Some(480.0));
    assert_eq!(ask(&state, 7.0, 0.0), Some(1.0));
    assert_eq!(ask(&state, 15.0, 3.0), Some(1.0));
    // No helpers here.
    assert_eq!(ask(&state, 16.0, 0.0), Some(0.0));
    // The smart camera checks: the player's, once the viewer has worked
    // them out for the attack; 0 before, and for anyone else.
    let side = |state: &GameState, name: &str, on: FormId| value(state, name, Some(on), &[n(0.0)]);
    assert_eq!(
        side(&state, "GetVATSRightTargetVisible", PLAYER_REF),
        Some(0.0)
    );
    state.vats.smart_camera = Some(vats::SmartCamera {
        area_free: [f32::MAX, 300.0, 1200.0, 60.0],
        target_visible: [640.0, 256.0, 0.0, 128.0],
    });
    assert_eq!(
        side(&state, "GetVATSRightTargetVisible", PLAYER_REF),
        Some(256.0)
    );
    assert_eq!(
        side(&state, "GetVATSLeftAreaFree", PLAYER_REF),
        Some(1200.0)
    );
    assert_eq!(side(&state, "GetVATSBackAreaFree", PLAYER_REF), Some(60.0));
    assert_eq!(
        side(&state, "GetVATSFrontTargetVisible", PLAYER_REF),
        Some(640.0)
    );
    assert_eq!(
        side(&state, "GetVATSRightTargetVisible", FormId(DOC_REF)),
        Some(0.0)
    );
}

#[test]
fn a_kill_in_playback_gives_grim_reapers_points() {
    let (_data, order) = order("vats-reaper");
    let mut state = GameState::new(&order);
    state.perks.insert(FormId(GRIM_REAPER));
    vats::set_mode(&mut state, mode::PLAYBACK);
    vats::spend(&mut state, 60.0);
    vats::end(&order, &mut state, 0);
    assert_eq!(vats::action_points(&order, &state), 20.0);
    vats::set_mode(&mut state, mode::PLAYBACK);
    vats::end(&order, &mut state, 1);
    assert_eq!(vats::action_points(&order, &state), 40.0);
    assert_eq!(state.vats.mode, mode::OFF);
}

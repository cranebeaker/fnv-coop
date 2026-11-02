//! Launchers, explosive projectiles, thrown weapons and mines
//! (`docs/EXPLOSIVES.md`): the ammunition's own projectile, missiles
//! falling under their gravity, an explosion's object effect, knockdowns,
//! the AI's danger test, and placed mines going off, being disarmed and
//! taken, on generated records carrying `FalloutNV.esm`'s values.

use esm::{ActivePlugins, FormId, LoadOrder};
use testdata::launchers::ids::*;
use world::combat::{self, Weapon};
use world::dialogue::PLAYER_REF;
use world::explosions::{self, ExplosionRecord, ProjectileRecord};
use world::mines::{self, MineSettings, MineUse};
use world::scripting::{Facts, GameState};

fn order(tag: &str) -> (testdata::TempData, LoadOrder) {
    let data = testdata::launchers::launchers(tag);
    let order = LoadOrder::from_data_dir(data.path(), &ActivePlugins::OfficialOnly).unwrap();
    (data, order)
}

fn explosives(order: &LoadOrder, state: &GameState, who: FormId) -> f32 {
    Facts {
        order,
        state,
        speaker: None,
    }
    .current_actor_value(who, 35)
    .unwrap_or(0.0) as f32
}

#[test]
fn launchers_fire_their_ammunitions_projectile_and_grenades_fall() {
    let (_data, order) = order("launchers-records");
    let mut state = GameState::new(&order);
    let launcher = Weapon::load(&order, FormId(MISSILE_LAUNCHER)).unwrap();
    assert_eq!(launcher.projectile, Some(FormId(MISSILE)));
    // Plain missiles fly the launcher's projectile; high-velocity ones
    // their own (`AMMO` `DAT2` form at 4).
    state.stocked.insert(PLAYER_REF);
    state.items.insert((PLAYER_REF, FormId(MISSILE_AMMO)), 2);
    let fired = |s: &GameState| combat::fired_projectile(&order, s, PLAYER_REF, &launcher);
    assert_eq!(fired(&state), Some(FormId(MISSILE)));
    state.items.remove(&(PLAYER_REF, FormId(MISSILE_AMMO)));
    state.items.insert((PLAYER_REF, FormId(MISSILE_AMMO_HV)), 2);
    assert_eq!(fired(&state), Some(FormId(MISSILE_HV)));
    let hv = ProjectileRecord::load(&order, FormId(MISSILE_HV)).unwrap();
    assert_eq!(hv.speed, 5000.0);
    // The rocket doesn't fall; the 40 mm grenade falls at 1.5 × the
    // world's gravity (the projectile's character controller, `009be0a0`).
    let rocket = ProjectileRecord::load(&order, FormId(MISSILE)).unwrap();
    let grenade = ProjectileRecord::load(&order, FormId(GRENADE_40MM)).unwrap();
    assert_eq!(world::projectiles::missile_gravity(&rocket), 0.0);
    assert_eq!(
        world::projectiles::missile_gravity(&grenade),
        1.5 * world::combat_ai::WORLD_GRAVITY
    );
    assert_eq!(
        world::projectiles::delivery(&grenade, false),
        world::projectiles::Delivery::Flies
    );
}

#[test]
fn a_pulse_grenade_casts_its_emp_and_blasts_knock_down_by_formula() {
    let (_data, order) = order("launchers-blast");
    let mut state = GameState::new(&order);
    let who = FormId(WANDERER_REF);
    let pulse = ExplosionRecord::load(&order, FormId(PULSE_EXPLOSION)).unwrap();
    assert_eq!(pulse.enchantment, Some(FormId(EMP)));
    let full = combat::health(&order, &state, who).unwrap();
    let said = explosions::cast_enchantment(&order, &mut state, &pulse, Some(PLAYER_REF), who);
    assert_eq!(said.len(), 1, "{said:?}");
    assert!(combat::health(&order, &state, who).unwrap() <= full - 200.0 + 1e-3);
    // No knockdown flag on the missile's explosion; the 40 mm grenade's
    // knocks down by formula.
    let mut state = GameState::new(&order);
    let missile = ExplosionRecord::load(&order, FormId(MISSILE_EXPLOSION)).unwrap();
    let forty = ExplosionRecord::load(&order, FormId(EXPLOSION_40MM)).unwrap();
    assert!(!explosions::knocks_down(
        &order, &state, &missile, who, 90.0, 0
    ));
    // 90 of 100 health is over 75 %: always.
    assert!(explosions::knocks_down(
        &order, &state, &forty, who, 90.0, 999
    ));
    // 30 is over 25 % of the 100 left: (30 × 0.3) ÷ (Agility 5 × 10) =
    // 0.18, so a roll under 180 of 1000.
    assert!(explosions::knocks_down(
        &order, &state, &forty, who, 30.0, 179
    ));
    assert!(!explosions::knocks_down(
        &order, &state, &forty, who, 30.0, 180
    ));
    // 20 is not over 25 %: never; nor for the dead.
    assert!(!explosions::knocks_down(
        &order, &state, &forty, who, 20.0, 0
    ));
    state.dead.insert(who);
    assert!(!explosions::knocks_down(
        &order, &state, &forty, who, 90.0, 0
    ));
    // The arithmetic alone (`00646400`): a hit that kills doesn't knock.
    let s = explosions::KnockdownSettings::read(&order);
    assert!(!explosions::knockdown_by_formula(
        (100.0, 100.0),
        5.0,
        100,
        &s,
        0
    ));
    assert!((s.chance - 0.25).abs() < 1e-6);
}

#[test]
fn people_hold_explosives_that_would_catch_their_own_side() {
    let (_data, order) = order("launchers-danger");
    let s = explosions::DangerSettings::read(&order);
    let missile = ExplosionRecord::load(&order, FormId(MISSILE_EXPLOSION)).unwrap();
    // Radius 1000 units (flag 0x01), damage 200: 5 damage still reaches
    // 1000 × √(1 − 5/200) = 987 units out.
    let blast = Some((missile.radius_units(22.0), missile.damage));
    let reach = explosions::reach_of_damage(1000.0, 200.0, 5.0);
    assert!((reach - 987.4).abs() < 0.1);
    let mut asked = 0.0;
    let nobody = |r: f32| {
        asked = r;
        explosions::Nearby::default()
    };
    assert!(explosions::explosion_attack_allowed(
        blast, &s, 0, 50.0, nobody
    ));
    assert!((asked - reach).abs() < 1e-3);
    let near = |own, group, spectators| {
        move |_: f32| explosions::Nearby {
            own,
            group,
            spectators,
        }
    };
    use explosions::style_flags::*;
    // Themselves in it: not unless the style ignores damaging themselves.
    assert!(!explosions::explosion_attack_allowed(
        blast,
        &s,
        0,
        0.0,
        near(1, 0, 0)
    ));
    assert!(explosions::explosion_attack_allowed(
        blast,
        &s,
        IGNORE_DAMAGING_SELF,
        0.0,
        near(1, 0, 0)
    ));
    assert!(!explosions::explosion_attack_allowed(
        blast,
        &s,
        0,
        0.0,
        near(0, 2, 0)
    ));
    // Bystanders matter to the responsible (50 and up).
    assert!(explosions::explosion_attack_allowed(
        blast,
        &s,
        0,
        49.0,
        near(0, 0, 1)
    ));
    assert!(!explosions::explosion_attack_allowed(
        blast,
        &s,
        0,
        50.0,
        near(0, 0, 1)
    ));
    assert!(explosions::explosion_attack_allowed(
        blast,
        &s,
        IGNORE_DAMAGING_SPECTATORS,
        50.0,
        near(0, 0, 1)
    ));
    // Small blasts and no blast are never refused.
    assert!(explosions::explosion_attack_allowed(
        Some((20.0, 200.0)),
        &s,
        0,
        100.0,
        near(1, 1, 1)
    ));
    assert!(explosions::explosion_attack_allowed(
        Some((500.0, 4.0)),
        &s,
        0,
        100.0,
        near(1, 1, 1)
    ));
    assert!(explosions::explosion_attack_allowed(
        None,
        &s,
        0,
        100.0,
        near(1, 1, 1)
    ));
}

#[test]
fn a_placed_mine_goes_off_for_strangers_not_its_owners() {
    let (_data, order) = order("launchers-mine");
    let state = GameState::new(&order);
    let s = MineSettings::read(&order);
    let placed = mines::placed_in(&order, &state, FormId(ROOM));
    assert_eq!(placed.len(), 1);
    let mut mine = placed[0].mine.clone();
    assert_eq!(placed[0].reference, FormId(MINE_REF));
    assert_eq!(mine.owner, Some(FormId(GUARDS)));
    assert!(!mine.exterior && mines::proximity_radius(&s, &mine) == 100.0);
    let guard = FormId(GUARD_REF);
    let wanderer = FormId(WANDERER_REF);
    // The guard (in the owning faction) walks right over it; the wanderer
    // 150 away is out of reach; at 60 the wanderer sets it off, with
    // Explosives × 3 s ÷ 100 + `fMinesDelayMin`.
    let fuse = |m: &mut mines::Mine, people: &[(FormId, [f32; 3])], player, roll| {
        mines::check_proximity(&order, &state, &s, m, people, player, roll)
    };
    assert_eq!(fuse(&mut mine, &[(guard, [0.0, 10.0, 0.0])], None, 0), None);
    assert_eq!(
        fuse(&mut mine, &[(wanderer, [150.0, 0.0, 0.0])], None, 0),
        None
    );
    let set = fuse(&mut mine, &[(wanderer, [60.0, 0.0, 0.0])], None, 0).unwrap();
    let expected = explosives(&order, &state, wanderer) * 3.0 / 100.0 + s.delay_min;
    assert!((set - expected).abs() < 1e-4, "{set} {expected}");
    // The player: the chance is 100 without Light Step.
    let player_fuse = fuse(&mut mine, &[], Some([0.0, 50.0, 0.0]), 99).unwrap();
    let expected = explosives(&order, &state, PLAYER_REF) * 3.0 / 100.0 + s.delay_min;
    assert!((player_fuse - expected).abs() < 1e-4);
    // Outdoors the proximity is × `fMineExteriorRadiusMult`.
    let outdoors = mines::Mine {
        exterior: true,
        ..mine.clone()
    };
    assert_eq!(
        mines::proximity_radius(&s, &outdoors),
        100.0 * s.exterior_radius_mult
    );
    // Its fuse blinks faster as it runs out (`009c3190`).
    assert!((mines::blink_interval(&s, s.blink_max * 2.0) - s.blink_slow).abs() < 1e-6);
    assert!((mines::blink_interval(&s, 0.0) - s.blink_fast).abs() < 1e-6);
    let half = mines::blink_interval(&s, s.blink_max / 2.0);
    assert!((half - (s.blink_slow + s.blink_fast) / 2.0).abs() < 1e-6);
}

#[test]
fn light_step_spares_the_player_and_layers_spare_themselves() {
    let (_data, order) = order("launchers-lightstep");
    let mut state = GameState::new(&order);
    state.perks.insert(FormId(LIGHT_STEP));
    let s = MineSettings::read(&order);
    let mut mine = mines::placed_in(&order, &state, FormId(ROOM))[0]
        .mine
        .clone();
    let near = Some([0.0, 10.0, 0.0]);
    assert_eq!(
        mines::check_proximity(&order, &state, &s, &mut mine, &[], near, 0),
        None
    );
    // The roll failed: it spares them from then on.
    assert!(mine.spares_player);
    state.perks.clear();
    assert_eq!(
        mines::check_proximity(&order, &state, &s, &mut mine, &[], near, 0),
        None
    );
    // A mine the player laid: without `iProjectileMineShooterCanTrigger`
    // (the exe's 0; the game's data sets 1) it ignores them.
    let mut laid = mines::Mine {
        shooter: Some(PLAYER_REF),
        owner: None,
        spares_player: false,
        ..mine.clone()
    };
    assert!(!s.shooter_can_trigger);
    assert!(!mines::reacts_to(&order, &state, &s, &laid, PLAYER_REF));
    let can = MineSettings {
        shooter_can_trigger: true,
        ..s
    };
    assert!(mines::check_proximity(&order, &state, &can, &mut laid, &[], near, 0).is_some());
}

#[test]
fn the_player_disarms_a_mine_then_takes_it() {
    let (_data, order) = order("launchers-disarm");
    let mut state = GameState::new(&order);
    let s = MineSettings::read(&order);
    let r = FormId(MINE_REF);
    let info = world::activation::info(&order, &state, r).unwrap();
    assert_eq!(info.action.as_deref(), Some("Disarm Mine"));
    assert_eq!(info.target, "MineFragProjectile");
    let mut placed = mines::placed_in(&order, &state, FormId(ROOM)).remove(0);
    assert_eq!(mines::activation(&placed.mine), MineUse::Disarm);
    assert!(mines::disarm(
        &order,
        &mut state,
        &s,
        &mut placed.mine,
        PLAYER_REF,
        false,
        false
    ));
    state.more.mines.disarmed.insert(r);
    assert_eq!(world::stats::get(&state, world::stats::MINES_DISARMED), 1);
    // Disarmed, it waits for no one, and E takes it as a frag mine.
    let mut m = placed.mine.clone();
    let wanderer = (FormId(WANDERER_REF), [0.0, 10.0, 0.0]);
    assert_eq!(
        mines::check_proximity(&order, &state, &s, &mut m, &[wanderer], None, 0),
        None
    );
    let info = world::activation::info(&order, &state, r).unwrap();
    assert_eq!(info.action.as_deref(), Some("Take"));
    let placed = mines::placed_in(&order, &state, FormId(ROOM)).remove(0);
    assert!(placed.mine.disarmed);
    assert_eq!(
        mines::activation(&placed.mine),
        MineUse::PickUp(Some(FormId(FRAG_MINE)))
    );
    // Saved and loaded, it's still disarmed.
    let text = world::save::save(&state, None);
    let (back, _) = world::save::load(&text).unwrap();
    assert_eq!(back.more.mines, state.more.mines);
    assert_eq!(
        mines::take(&order, &mut state, &placed),
        Some(FormId(FRAG_MINE))
    );
    assert_eq!(state.item_count(&order, PLAYER_REF, FormId(FRAG_MINE)), 1);
    assert!(mines::placed_in(&order, &state, FormId(ROOM)).is_empty());
    let text = world::save::save(&state, None);
    let (back, _) = world::save::load(&text).unwrap();
    assert!(back.more.mines.gone.contains(&r));
}

#[test]
fn disarming_credits_a_running_fuse_even_when_the_mine_spares_the_player() {
    // 009c43e0: credited when the fuse (+0xe4) is below FLT_MAX, else only
    // when the mine reacts to the one disarming (009c3930).
    let (_data, order) = order("launchers-disarm-fuse");
    let mut state = GameState::new(&order);
    let s = MineSettings::read(&order);
    let r = FormId(MINE_REF);
    // The player in the owning faction: the guards' mine spares them.
    state
        .faction_changes
        .insert((PLAYER_REF, FormId(GUARDS)), 0);
    let placed = mines::placed_in(&order, &state, FormId(ROOM)).remove(0);
    assert!(!mines::reacts_to(
        &order,
        &state,
        &s,
        &placed.mine,
        PLAYER_REF
    ));
    // Quiet: disarmed, no credit.
    assert!(mines::use_placed(&order, &mut state, r).is_some());
    assert_eq!(world::stats::get(&state, world::stats::MINES_DISARMED), 0);
    // Again with its fuse running (set off by someone else): credited.
    let mut state = GameState::new(&order);
    state
        .faction_changes
        .insert((PLAYER_REF, FormId(GUARDS)), 0);
    state.more.mines.fuse_running.insert(r);
    assert!(mines::use_placed(&order, &mut state, r).is_some());
    assert_eq!(world::stats::get(&state, world::stats::MINES_DISARMED), 1);
    assert!(state.more.mines.fuse_running.is_empty());
}

#[test]
fn a_laid_mine_waits_then_its_fuse_runs() {
    let (_data, order) = order("launchers-laid");
    let p = ProjectileRecord::load(&order, FormId(MINE_PROJECTILE)).unwrap();
    assert!(p.is_mine());
    assert_eq!(p.weapon_source, Some(FormId(FRAG_MINE)));
    // Mines live for ever (`fMineAgeMax` 0).
    let settings = explosions::FlightSettings::read(&order, &p);
    assert_eq!(settings.age_max, 0.0);
    let mut flight = explosions::Flight::launch(p, [0.0, 0.0, 10.0], [0.0, 1.0, 0.0], 0.0);
    let mut floor = |from: [f32; 3], dir: [f32; 3], len: f32| {
        (dir[2] < 0.0 && from[2] + dir[2] * len <= 0.0)
            .then(|| (from[2] / -dir[2], [0.0, 0.0, 1.0]))
    };
    for _ in 0..600 {
        assert_eq!(
            flight.step(0.1, &settings, &mut floor),
            explosions::FlightEvent::Flying
        );
    }
    flight.set_off(0.25);
    assert_eq!(
        flight.step(0.1, &settings, &mut floor),
        explosions::FlightEvent::Flying
    );
    assert_eq!(
        flight.step(0.1, &settings, &mut floor),
        explosions::FlightEvent::Flying
    );
    assert!(matches!(
        flight.step(0.1, &settings, &mut floor),
        explosions::FlightEvent::Explode { .. }
    ));
}

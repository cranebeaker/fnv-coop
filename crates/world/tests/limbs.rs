//! Body parts: which body part data people and creatures use, shots to the
//! head, limb damage and crippling, a dropped weapon, slower legs, and the
//! Stimpak's limb healing (`world::body_parts`).

use esm::{ActivePlugins, FormId, LoadOrder};
use testdata::quest_ids::*;
use world::body_parts::{self, part, BodyPartData};
use world::combat::{self, Weapon};
use world::dialogue::PLAYER_REF;
use world::scripting::{GameState, Runner, ScriptCache};

fn order(tag: &str) -> (testdata::TempData, LoadOrder) {
    let data = testdata::quests(tag);
    let order = LoadOrder::from_data_dir(data.path(), &ActivePlugins::OfficialOnly).unwrap();
    (data, order)
}

/// A script expression's value (`set TestGlobal to …`).
fn ask(order: &LoadOrder, scripts: &ScriptCache, state: &mut GameState, what: &str) -> f32 {
    Runner::new(order, scripts, state).run_source(&format!("set TestGlobal to {what}"), None, None);
    state.globals[&FormId(GLOBAL)]
}

fn pistol(order: &LoadOrder) -> Weapon {
    Weapon::load(order, FormId(PISTOL)).unwrap()
}

#[test]
fn people_creatures_and_the_player_have_their_own_body_part_data() {
    let (_data, order) = order("limbs-data");
    assert_eq!(
        body_parts::data_form(&order, FormId(DOC_REF)),
        Some(FormId(DEFAULT_BODY_PARTS))
    );
    assert_eq!(
        body_parts::data_form(&order, PLAYER_REF),
        Some(FormId(PLAYER_BODY_PARTS))
    );
    assert_eq!(
        body_parts::data_form(&order, FormId(GECKO_REF)),
        Some(FormId(GECKO_BODY_PARTS))
    );
    let people = BodyPartData::of(&order, FormId(DOC_REF)).unwrap();
    assert_eq!(people.editor_id.as_deref(), Some("DefaultBodyPartData"));
    let head = people.part(part::HEAD).unwrap();
    assert_eq!(
        (head.damage_mult, head.health_percent, head.actor_value),
        (2.0, 20, 25)
    );
    let player = BodyPartData::of(&order, PLAYER_REF).unwrap();
    assert_eq!(player.part(part::LEFT_LEG).unwrap().health_percent, 150);
    assert_eq!(player.part(part::HEAD).unwrap().damage_mult, 1.0);
    // The weapon's limb damage multiplier (`DNAM` 116).
    assert_eq!(pistol(&order).limb_damage_mult, 1.0);
    assert!(!pistol(&order).two_handed());
    assert!(Weapon::load(&order, FormId(RIFLE)).unwrap().two_handed());
}

#[test]
fn a_shot_to_the_head_does_double_and_hurts_the_head() {
    let (_data, order) = order("limbs-headshot");
    let scripts = ScriptCache::default();
    let mut state = GameState::new(&order);
    let doc = FormId(DOC_REF);
    let pistol = pistol(&order);
    // Guns 15: 9.2 a shot after Doc's (no) armour. On the head: limb
    // damage 9.2 ÷ (20% × 100 health) × 100 = 46; the health × 2.
    let hit = Runner::new(&order, &scripts, &mut state)
        .hit_at(PLAYER_REF, doc, Some(&pistol), Some(part::HEAD))
        .unwrap();
    assert!((hit.dealt - 18.4).abs() < 1e-4, "{hit:?}");
    assert_eq!(hit.multiplier, 2.0);
    let hurt = hit.hurt.unwrap();
    assert_eq!(hurt.name, "Head");
    assert!((hurt.lost - 46.0).abs() < 1e-4);
    assert!(!hurt.crippled);
    assert!((combat::health(&order, &state, doc).unwrap() - 81.6).abs() < 1e-4);
    // Scripts see the part's condition and where the hit landed.
    assert!(
        (ask(
            &order,
            &scripts,
            &mut state,
            "DocRef.GetAV PerceptionCondition"
        ) - 54.0)
            .abs()
            < 1e-4
    );
    assert_eq!(
        ask(&order, &scripts, &mut state, "DocRef.GetHitLocation"),
        1.0
    );
    // A second leaves it at 8, above 0; a third cripples it.
    let second = Runner::new(&order, &scripts, &mut state)
        .hit_at(PLAYER_REF, doc, Some(&pistol), Some(part::HEAD))
        .unwrap();
    assert!(!second.hurt.unwrap().crippled);
    let third = Runner::new(&order, &scripts, &mut state)
        .hit_at(PLAYER_REF, doc, Some(&pistol), Some(part::HEAD))
        .unwrap();
    assert!(third.hurt.unwrap().crippled);
    assert!(body_parts::is_crippled(&order, &state, doc, 25));
    assert_eq!(
        body_parts::crippled_parts(&order, &state, doc),
        vec!["Head".to_string()]
    );
    // Three shots of 18.4 leave 44.8; two more kill: the killing blow's
    // part is kept.
    let mut runner = Runner::new(&order, &scripts, &mut state);
    runner.hit_at(PLAYER_REF, doc, Some(&pistol), Some(part::TORSO));
    runner.hit_at(PLAYER_REF, doc, Some(&pistol), Some(part::HEAD));
    runner.hit_at(PLAYER_REF, doc, Some(&pistol), Some(part::HEAD));
    assert!(state.dead.contains(&doc));
    assert_eq!(
        ask(&order, &scripts, &mut state, "DocRef.GetKillingBlowLimb"),
        1.0
    );
    assert!(state.unhandled.is_empty(), "{:?}", state.unhandled_first);
}

#[test]
fn melee_and_unknown_parts_dont_multiply_and_no_part_means_no_sneak_bonus() {
    let (_data, order) = order("limbs-melee");
    let scripts = ScriptCache::default();
    let mut state = GameState::new(&order);
    let doc = FormId(DOC_REF);
    // Fists to the head: × 1, but the head still loses its share.
    let fists = Runner::new(&order, &scripts, &mut state)
        .hit_at(PLAYER_REF, doc, None, Some(part::HEAD))
        .unwrap();
    assert_eq!(fists.multiplier, 1.0);
    let lost = fists.hurt.unwrap().lost;
    assert!((lost - fists.dealt / 20.0 * 100.0).abs() < 1e-4, "{lost}");
    // No part found: nothing multiplied, no limb damage.
    let pistol = pistol(&order);
    let plain = Runner::new(&order, &scripts, &mut state)
        .hit_at(PLAYER_REF, doc, Some(&pistol), None)
        .unwrap();
    assert_eq!((plain.multiplier, plain.hurt.is_none()), (0.0, true));
    assert_eq!(
        ask(&order, &scripts, &mut state, "DocRef.GetHitLocation"),
        -1.0
    );
    // A sneak attack (unseen: nobody's place is known here) with a gun:
    // × 2 on the torso, but without a part no bonus at all. (Criticals
    // add the test pistol's 0.)
    state.player_sneaking = true;
    let sneak = Runner::new(&order, &scripts, &mut state)
        .hit_at(PLAYER_REF, doc, Some(&pistol), Some(part::TORSO))
        .unwrap();
    assert!(sneak.critical);
    assert!((sneak.dealt - 18.4).abs() < 1e-4, "{sneak:?}");
    let unplaced = Runner::new(&order, &scripts, &mut state)
        .hit_at(PLAYER_REF, doc, Some(&pistol), None)
        .unwrap();
    assert!((unplaced.dealt - 9.2).abs() < 1e-4, "{unplaced:?}");
}

#[test]
fn a_crippled_right_arm_drops_the_weapon_and_a_two_handed_one_the_left_too() {
    let (_data, order) = order("limbs-drop");
    let scripts = ScriptCache::default();
    let mut state = GameState::new(&order);
    let doc = FormId(DOC_REF);
    state.stock(&order, doc);
    state.items.insert((doc, FormId(PISTOL)), 1);
    state.items.insert((doc, FormId(RIFLE)), 1);
    state.equip(&order, doc, FormId(PISTOL));
    let pistol = pistol(&order);
    // The right arm (25%): 36.8 a shot; the third cripples it.
    let mut last = None;
    for _ in 0..3 {
        last = Runner::new(&order, &scripts, &mut state).hit_at(
            PLAYER_REF,
            doc,
            Some(&pistol),
            Some(part::RIGHT_ARM),
        );
    }
    let hurt = last.take().unwrap().hurt.unwrap();
    assert!(hurt.crippled);
    assert_eq!(hurt.dropped, Some(FormId(PISTOL)));
    assert!(state.dropped.contains(&(doc, FormId(PISTOL))));
    // They fight with what's left: the rifle.
    assert_eq!(
        combat::weapon_in_hand(&order, &state, doc).map(|w| w.form_id),
        Some(FormId(RIFLE))
    );
    // A crippled left arm drops a two-handed weapon (not a one-handed one).
    for _ in 0..3 {
        last = Runner::new(&order, &scripts, &mut state).hit_at(
            PLAYER_REF,
            doc,
            Some(&pistol),
            Some(part::LEFT_ARM),
        );
    }
    let hurt = last.unwrap().hurt.unwrap();
    assert_eq!((hurt.crippled, hurt.dropped), (true, Some(FormId(RIFLE))));
    assert!(combat::weapon_in_hand(&order, &state, doc).is_none());
    // Kept in a saved game.
    let (loaded, _) = world::save::load(&world::save::save(&state, None)).unwrap();
    assert_eq!(loaded.dropped, state.dropped);
}

#[test]
fn ignoring_crippled_limbs_keeps_the_weapon_and_the_pace() {
    let (_data, order) = order("limbs-ignore");
    let scripts = ScriptCache::default();
    let mut state = GameState::new(&order);
    let doc = FormId(DOC_REF);
    state.stock(&order, doc);
    state.items.insert((doc, FormId(PISTOL)), 1);
    state.actor_values.insert((doc, 72), 1.0);
    let pistol = pistol(&order);
    let mut last = None;
    for _ in 0..3 {
        last = Runner::new(&order, &scripts, &mut state).hit_at(
            PLAYER_REF,
            doc,
            Some(&pistol),
            Some(part::RIGHT_ARM),
        );
    }
    let hurt = last.unwrap().hurt.unwrap();
    assert_eq!((hurt.crippled, hurt.dropped), (true, None));
    state.value_damage.insert((doc, 29), 100.0);
    assert_eq!(body_parts::leg_speed_mult(&order, &state, doc), 1.0);
}

#[test]
fn crippled_legs_slow_and_the_player_takes_half_limb_damage() {
    let (_data, order) = order("limbs-legs");
    let scripts = ScriptCache::default();
    let mut state = GameState::new(&order);
    let p = PLAYER_REF;
    assert_eq!(body_parts::leg_speed_mult(&order, &state, p), 1.0);
    // The gecko bites the player's left leg (150% of 100 health): 8 ÷ 150
    // × 100 = 5.33, halved for the player.
    let gecko = FormId(GECKO_REF);
    let bite = Runner::new(&order, &scripts, &mut state)
        .hit_at(gecko, p, None, Some(part::LEFT_LEG))
        .unwrap();
    assert_eq!(bite.dealt, 8.0);
    assert!((bite.hurt.unwrap().lost - 8.0 / 1.5 * 0.5).abs() < 1e-4);
    // One leg crippled: × 0.85; both: × 0.75.
    state.value_damage.insert((p, 29), 100.0);
    assert_eq!(body_parts::leg_speed_mult(&order, &state, p), 0.85);
    state.value_damage.insert((p, 30), 120.0);
    assert_eq!(body_parts::leg_speed_mult(&order, &state, p), 0.75);
    // A Stimpak heals each part a tenth of its 30.
    state.items.insert((p, FormId(STIMPAK)), 1);
    world::items::use_item(&order, &mut state, p, FormId(STIMPAK)).unwrap();
    assert!((state.value_damage[&(p, 29)] - 97.0).abs() < 1e-6);
    assert!((state.value_damage[&(p, 30)] - 117.0).abs() < 1e-6);
    // Whole parts stay whole.
    assert!(!state.value_damage.contains_key(&(p, 25)));
    // Aimed at the right leg from the Pip-Boy's healing mode (`00823210`,
    // `0082b970`): that leg gets all 30, health a tenth, the left leg
    // nothing; and the aim is kept with the effect in a save.
    state.damage.insert(p, 10.0);
    state.healing_part = Some(30);
    state.items.insert((p, FormId(STIMPAK)), 1);
    world::items::use_item(&order, &mut state, p, FormId(STIMPAK)).unwrap();
    assert!((state.value_damage[&(p, 30)] - 87.0).abs() < 1e-6);
    assert!((state.value_damage[&(p, 29)] - 97.0).abs() < 1e-6);
    assert!((state.damage[&p] - 7.0).abs() < 1e-6);
}

/// An effect aimed at a part keeps its aim in a save (`effect` line's
/// fourteenth field; older saves without it aim at none).
#[test]
fn an_aimed_effect_keeps_its_part_in_a_save() {
    let mut state = GameState::default();
    state.active_effects.push(world::magic::ActiveEffect {
        target: PLAYER_REF,
        source: FormId(0x10),
        effect: FormId(0x11),
        actor_value: 16,
        magnitude: 3.0,
        detrimental: false,
        recover: false,
        archetype: world::magic::archetype::VALUE_AND_PARTS,
        resist: -1,
        script: None,
        remaining: 5.0,
        started: false,
        locals: Default::default(),
        part: 27,
        caster: None,
        item: None,
    });
    let saved = world::save::save(&state, None);
    let (back, _) = world::save::load(&saved).unwrap();
    assert_eq!(back.active_effects[0].part, 27);
}

#[test]
fn a_hit_on_the_weapon_damages_it_and_a_critical_one_knocks_it_away() {
    let (_data, order) = order("limbs-weapon");
    let scripts = ScriptCache::default();
    let mut state = GameState::new(&order);
    let doc = FormId(DOC_REF);
    state.stock(&order, doc);
    state.items.insert((doc, FormId(PISTOL)), 1);
    state.equip(&order, doc, FormId(PISTOL));
    let pistol = pistol(&order);
    // Not critical (the first roll here isn't): the 9.2 comes off the
    // weapon's 150 condition, none off Doc.
    let hit = Runner::new(&order, &scripts, &mut state)
        .hit_at(PLAYER_REF, doc, Some(&pistol), Some(part::WEAPON))
        .unwrap();
    assert!(!hit.critical);
    assert_eq!(hit.dealt, 0.0);
    assert_eq!(combat::health(&order, &state, doc), Some(100.0));
    let condition = combat::weapon_condition(&state, doc, FormId(PISTOL));
    assert!(
        (condition - (1.0 - 9.2 / 150.0)).abs() < 1e-5,
        "{condition}"
    );
    assert!(state.dropped.is_empty());
    // A critical one (a sneak attack's): dropped.
    state.player_sneaking = true;
    let hit = Runner::new(&order, &scripts, &mut state)
        .hit_at(PLAYER_REF, doc, Some(&pistol), Some(part::WEAPON))
        .unwrap();
    assert!(hit.critical);
    assert_eq!(hit.hurt.unwrap().dropped, Some(FormId(PISTOL)));
}

#[test]
fn a_gecko_has_no_part_below_its_torso_node() {
    let (_data, order) = order("limbs-gecko");
    let gecko = BodyPartData::of(&order, FormId(GECKO_REF)).unwrap();
    let bones: Vec<nif::Bone> = [
        ("Scene Root", None),
        ("Bip01", Some(0)),
        ("Bip01 NonAccum", Some(1)),
        ("Bip01 Pelvis", Some(2)),
        ("Bip01 Spine1", Some(3)),
        ("Bip01 Spine2", Some(4)),
        ("Bip01 Neck1", Some(5)),
        ("Bip01 Head", Some(6)),
    ]
    .iter()
    .map(|&(name, parent)| nif::Bone {
        name: name.into(),
        parent,
        local: nif::Transform::IDENTITY,
    })
    .collect();
    assert_eq!(gecko.part_of_bone(&bones, 7), Some(part::HEAD));
    assert_eq!(gecko.part_of_bone(&bones, 5), Some(part::TORSO));
    // The pelvis is above the torso's node: no part.
    assert_eq!(gecko.part_of_bone(&bones, 3), None);
}

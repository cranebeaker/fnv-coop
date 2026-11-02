//! Hit effects read from records (`world::impacts`), on the test world
//! `testdata::impacts`.

use esm::{FormId, LoadOrder};
use testdata::impacts::ids::*;
use world::impacts::{self, DeathCry, Impact, ImpactSet, Material, Orientation, TextureSet};
use world::scripting::GameState;

fn world(tag: &str) -> (testdata::TempData, LoadOrder) {
    let data = testdata::impacts::impacts(tag);
    let order = LoadOrder::from_data_dir(data.path(), &esm::ActivePlugins::OfficialOnly).unwrap();
    (data, order)
}

fn f(id: u32) -> FormId {
    FormId(id)
}

#[test]
fn impacts_sets_and_texture_sets_are_read_as_the_game_lays_them_out() {
    let (_data, order) = world("impacts-records");
    let stone = Impact::load(&order, f(STONE_IMPACT)).unwrap();
    assert_eq!(stone.model.as_deref(), Some("Effects\\TestStone.NIF"));
    assert_eq!(stone.duration, 0.25);
    assert_eq!(stone.orientation, Orientation::ProjectileReflection);
    assert_eq!(
        (stone.angle_threshold, stone.placement_radius),
        (15.0, 16.0)
    );
    assert_eq!(stone.sound_level, 1);
    assert_eq!(stone.texture_set, Some(f(HOLE_SET)));
    assert!(stone.has_decal());
    let decal = stone.decal.unwrap();
    assert_eq!((decal.min_width, decal.max_width), (10.0, 30.0));
    let flesh = Impact::load(&order, f(FLESH_IMPACT)).unwrap();
    let sounds: Vec<FormId> = flesh.sounds().collect();
    assert_eq!(sounds, [f(FLESH_SOUND), f(FLESH_SOUND2)]);
    let set = ImpactSet::load(&order, f(GUN_SET)).unwrap();
    assert_eq!(set.get(Material::Stone), Some(f(STONE_IMPACT)));
    assert_eq!(set.get(Material::Organic), Some(f(FLESH_IMPACT)));
    assert_eq!(set.get(Material::Wood), None);
    let blood = TextureSet::load(&order, f(BLOOD_SET)).unwrap();
    assert_eq!(blood.diffuse.as_deref(), Some("Test\\Blood.dds"));
    assert_eq!(impacts::weapon_impact_set(&order, f(GUN)), Some(f(GUN_SET)));
}

#[test]
fn a_shot_on_the_world_takes_the_weapons_impact_for_the_surfaces_material() {
    let (_data, order) = world("impacts-surface");
    let at = |havok: u32| impacts::surface_impact(&order, f(GUN), Material::from_havok(havok));
    // Stone, heavy stone, broken concrete: the stone impact.
    for h in [0, 10, 19] {
        assert_eq!(at(h), Some(f(STONE_IMPACT)));
    }
    // Skin is organic.
    assert_eq!(at(7), Some(f(FLESH_IMPACT)));
    // Chains are metal; wood has nothing in this set.
    assert_eq!(at(13), Some(f(METAL_IMPACT)));
    assert_eq!(at(9), None);
}

#[test]
fn people_are_organic_unless_their_record_or_power_armour_says_otherwise() {
    let (_data, order) = world("impacts-material");
    let mut state = GameState::new(&order);
    assert_eq!(
        impacts::actor_material(&order, f(PERSON_REF)),
        Material::Organic
    );
    assert_eq!(
        impacts::actor_material(&order, f(ROBOT_REF)),
        Material::Metal
    );
    // The knight carries power armour on the body: the body is metal, the
    // head isn't (no helmet).
    let armour = impacts::power_armour(&order, &state, f(KNIGHT_REF));
    assert_eq!((armour.body, armour.helmet), (true, false));
    assert_eq!(
        impacts::hit_material(&order, &state, f(KNIGHT_REF), Some(0)),
        Material::Metal
    );
    assert_eq!(
        impacts::hit_material(&order, &state, f(KNIGHT_REF), Some(1)),
        Material::Organic
    );
    // A hat isn't power armour.
    assert_eq!(
        impacts::power_armour(&order, &state, f(PERSON_REF)),
        impacts::PowerArmour::default()
    );
    // What the state says is worn wins: the helmet alone.
    state.equipped.insert(f(KNIGHT_REF), vec![f(POWER_HELMET)]);
    let armour = impacts::power_armour(&order, &state, f(KNIGHT_REF));
    assert_eq!((armour.body, armour.helmet), (false, true));
    assert_eq!(
        impacts::hit_material(&order, &state, f(KNIGHT_REF), Some(2)),
        Material::Metal
    );
}

#[test]
fn blood_needs_gore_and_one_of_spray_or_decals() {
    let (_data, order) = world("impacts-gore");
    assert!(impacts::shows_blood(&order, f(PERSON_REF), false));
    assert!(!impacts::shows_blood(&order, f(PERSON_REF), true));
    // No blood spray and no blood decal: none.
    assert!(!impacts::shows_blood(&order, f(ROBOT_REF), false));
}

#[test]
fn body_parts_have_their_own_impact_sets() {
    let (_data, order) = world("impacts-parts");
    // People: the default body part data's torso; creatures their own.
    assert_eq!(
        impacts::body_part_impact_set(&order, f(PERSON_REF), 0),
        Some(f(SPATTER_SET))
    );
    assert_eq!(
        impacts::body_part_impact_set(&order, f(PERSON_REF), 1),
        None
    );
    assert_eq!(
        impacts::body_part_impact_set(&order, f(GECKO_REF), 0),
        Some(f(SPATTER_SET))
    );
}

#[test]
fn creatures_take_their_sounds_from_their_template_and_roll_down_the_list() {
    let (_data, order) = world("impacts-sounds");
    // The gecko has no sounds of its own (ACBS flag 0x100 unset): its CSCR's.
    let deaths = impacts::creature_sounds(&order, f(GECKO_REF), impacts::creature_sound::DEATH);
    assert_eq!(deaths, [(f(STONE_SOUND), 0), (f(DEATH_SOUND), 100)]);
    // The first passing its chance: 0% never does.
    let mut roll = || 0u64;
    assert_eq!(
        impacts::creature_sound(&order, f(GECKO_REF), 8, &mut roll),
        Some(f(DEATH_SOUND))
    );
    assert_eq!(
        impacts::death_cry(&order, f(GECKO_REF), &mut roll),
        DeathCry::Sound(f(DEATH_SOUND))
    );
    // People say their Death line.
    assert_eq!(
        impacts::death_cry(&order, f(PERSON_REF), &mut roll),
        DeathCry::Line
    );
    // Nothing for a kind it doesn't list.
    assert!(impacts::creature_sounds(&order, f(GECKO_REF), 4).is_empty());
}

#[test]
fn a_hit_on_someone_plays_the_blows_sounds_and_shows_the_held_weapons_blood() {
    let (_data, order) = world("impacts-actor-hit");
    let state = GameState::new(&order);
    let mut roll = || 0u64;
    let mut hit = |attacker: u32, target: u32, weapon: Option<u32>, part: Option<u8>| {
        impacts::actor_hit(
            &order,
            &state,
            f(attacker),
            f(target),
            weapon.map(f),
            weapon.map(f),
            part,
            false,
            &mut roll,
        )
    };
    // A shot in a person's torso: the gun's flesh impact, both its sounds;
    // the torso's own set spatters the wall.
    let e = hit(KNIGHT_REF, PERSON_REF, Some(GUN), Some(0));
    assert_eq!(e.material, Material::Organic);
    assert_eq!(e.sounds, [f(FLESH_SOUND), f(FLESH_SOUND2)]);
    assert_eq!(e.blood, Some(f(FLESH_IMPACT)));
    assert_eq!(e.spatter, Some(f(SPATTER_IMPACT)));
    // The head has no set of its own.
    assert_eq!(
        hit(KNIGHT_REF, PERSON_REF, Some(GUN), Some(1)).spatter,
        None
    );
    // On power armour: metal blood (sparks), the flesh sounds plus the
    // metal impact's, no spatter.
    let e = hit(PERSON_REF, KNIGHT_REF, Some(GUN), Some(0));
    assert_eq!(e.material, Material::Metal);
    assert_eq!(e.sounds, [f(FLESH_SOUND), f(FLESH_SOUND2), f(METAL_SOUND)]);
    assert_eq!(e.blood, Some(f(METAL_IMPACT)));
    assert_eq!(e.spatter, None);
    // Its head isn't covered: flesh, but the body armour still adds its
    // sound.
    let e = hit(PERSON_REF, KNIGHT_REF, Some(GUN), Some(1));
    assert_eq!(e.material, Material::Organic);
    assert_eq!(e.blood, Some(f(FLESH_IMPACT)));
    assert_eq!(e.sounds.last(), Some(&f(METAL_SOUND)));
    // A gecko's bite: its own set's sounds and its "weapon" sound; it holds
    // no weapon, so the blood is the default set's.
    let e = hit(GECKO_REF, PERSON_REF, None, Some(0));
    assert_eq!(e.sounds, [f(FLESH_SOUND), f(BITE_SOUND)]);
    assert_eq!(e.blood, Some(f(DEFAULT_FLESH_IMPACT)));
    // A person's bare hands: the fists' set.
    let e = hit(KNIGHT_REF, PERSON_REF, None, Some(0));
    assert_eq!(e.sounds, [f(FLESH_SOUND)]);
    // The robot shows no blood, and is metal.
    let e = hit(PERSON_REF, ROBOT_REF, Some(GUN), Some(0));
    assert_eq!(e.sounds, [f(METAL_SOUND)]);
    assert_eq!((e.blood, e.spatter), (None, None));
}

#[test]
fn voice_files_keep_whole_names_up_to_25_letters() {
    use world::dialogue::voice_name_parts;
    // Sunny Smiles' hurt line: 20 + 3 letters, kept whole.
    assert_eq!(
        voice_name_parts("VFreeformGoodsprings", "Hit"),
        ("vfreeformgoodsprings".into(), "hit".into())
    );
    // Doc Mitchell's greeting: 20 + 8, cut to 10 and 15.
    assert_eq!(
        voice_name_parts("VFreeformGoodsprings", "GREETING"),
        ("vfreeformg".into(), "greeting".into())
    );
    assert_eq!(
        voice_name_parts("VFreeformGoodsprings", "PlayerFireWeapon"),
        ("vfreeformg".into(), "playerfireweapo".into())
    );
    // Exactly 25: whole.
    assert_eq!(
        voice_name_parts("VFreeformGoodsprings", "Death").0.len(),
        20
    );
}

/// B11: Doc Mitchell's psych test answers and his Pip-Boy line played no
/// voice (the line text showed, silent). Their quest `VCG01` is short, so
/// the game (`006172c0`) keeps it whole and cuts the topic to the rest of
/// the 25 letters: `vcg01_vcg01docmitchelltopi_0010556b_1.ogg` in
/// `Fallout - Voices1.bsa`, not the 10 + 15 cut (`vcg01docmitchel`).
#[test]
fn a_short_quest_leaves_the_topic_the_rest_of_25_letters() {
    use world::dialogue::voice_name_parts;
    assert_eq!(
        voice_name_parts("VCG01", "VCG01DocMitchellTopic075"),
        ("vcg01".into(), "vcg01docmitchelltopi".into())
    );
    // A 10-letter quest still keeps all of it (0xb is the bound), the
    // topic 15.
    assert_eq!(
        voice_name_parts("ABCDEFGHIJ", "VCG01DocMitchellTopic075"),
        ("abcdefghij".into(), "vcg01docmitchel".into())
    );
    // 11 letters: cut to 10, the topic to 15.
    assert_eq!(
        voice_name_parts("ABCDEFGHIJK", "VCG01DocMitchellTopic075"),
        ("abcdefghij".into(), "vcg01docmitchel".into())
    );
}

#[test]
fn hurt_lines_and_screen_blood_follow_the_games_settings() {
    let (_data, order) = world("impacts-voice");
    // FalloutNV.esm's 0.01 threshold and chance: 2 of 100 is above it.
    assert!(impacts::says_hurt_line(&order, 2.0, 100.0, false, 0.99));
    // Half a point of 100.9 (taken as 100): below; then the 1% chance.
    assert!(!impacts::says_hurt_line(&order, 0.5, 100.9, false, 0.5));
    assert!(impacts::says_hurt_line(&order, 0.5, 100.9, false, 0.005));
    assert!(impacts::says_hurt_line(&order, 0.0, 100.0, true, 0.99));
    // Screen blood: 2 + 0.1 × damage + U(−1, 1), cut to a whole number.
    assert_eq!(impacts::screen_blood_count(&order, 30.0, 0.5), 5);
    assert_eq!(impacts::screen_blood_count(&order, 30.0, 0.0), 4);
    assert_eq!(impacts::screen_blood_count(&order, 0.0, 0.0), 1);
    assert_eq!(impacts::screen_blood_count(&order, -50.0, 1.0), 0);
    // The hit modifier and its strength.
    assert_eq!(impacts::get_hit_modifier(&order), (FormId(0x162), 1.5));
}

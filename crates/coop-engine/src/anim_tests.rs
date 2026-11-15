//! Tests of [`super`] and of the head rebuild and look restore of [`crate::look`], on the
//! testkit's anim and look kits.

use super::*;
use crate::look::{self, Face};
use crate::testkit::{anim_kit, look_kit, Story, StorySpec};

const HOUSE: u32 = 0x0010_5a00;
const CHAIR: u32 = 0x0010_7200;
const WAVE: u32 = 0x0010_7201;
const RACE: u32 = 0x0010_7100;
const OTHER_RACE: u32 = 0x0010_7106;
const HAIR: u32 = 0x0010_7101;
const EYES: u32 = 0x0010_7102;
const NOSE: u32 = 0x0010_7103;
/// Group types of the kit's own choosing: a hit reaction, an attack (the kit's `0x20..0x30`) and
/// an aim (`0x30..0x40`).
const HIT: u16 = 0x0045;
const ATTACK: u16 = 0x0025;
const AIM: u16 = 0x0035;

fn story() -> Story {
    let mut s = Story::new(&StorySpec {
        cells: vec![HOUSE],
        ..StorySpec::default()
    });
    look_kit::install(&mut s);
    anim_kit::install(&mut s);
    for id in [CHAIR, WAVE, RACE, OTHER_RACE, HAIR, EYES, NOSE] {
        s.add_look_form(id, 0x0c);
    }
    s.move_player(HOUSE, [10.0, 20.0, 0.0], 0.5);
    s
}

fn player() -> u32 {
    crate::actors::PLAYER_REF_ID
}

fn avatar(s: &mut Story) -> u32 {
    live::avatar_create(&mut s.e, 7).expect("an avatar").id
}

#[test]
fn the_players_body_is_its_third_person_animation_and_its_process() {
    let mut s = story();
    let b = player_body(&mut s.e).unwrap();
    assert_eq!(
        b.groups,
        [addr::ANIM_GROUP_NONE; 8],
        "not the first person's"
    );
    assert_eq!((b.walk, b.run), (anim_kit::WALK, anim_kit::RUN));
    s.start_group(player(), 2, HIT);
    s.start_idle(player(), WAVE);
    s.set_controller(player(), 2);
    s.set_furniture(player(), CHAIR);
    s.set_body(player(), true, 90.0, 360.0, 3);
    let b = player_body(&mut s.e).unwrap();
    assert_eq!(b.groups[2], HIT);
    assert_eq!(b.idle, WAVE);
    assert_eq!(b.controller, 2);
    assert_eq!((b.furniture, b.sit), (CHAIR, anim_kit::SITTING));
    assert!(b.sights);
    assert_eq!((b.walk, b.run, b.knock), (90.0, 360.0, 3));
}

#[test]
fn only_what_the_avatar_would_not_start_itself_is_a_group_to_give_it() {
    let mut s = story();
    assert!(replayable(&mut s.e, 2, HIT));
    assert!(!replayable(&mut s.e, addr::ANIMATION_MOVEMENT_SECTION, HIT));
    assert!(!replayable(&mut s.e, 2, addr::ANIM_GROUP_NONE));
    assert!(!replayable(
        &mut s.e,
        4,
        u16::from(addr::ANIM_GROUP_TYPE_DRAW)
    ));
    assert!(!replayable(
        &mut s.e,
        4,
        u16::from(addr::ANIM_GROUP_TYPE_HOLSTER) | 0x0300
    ));
    assert!(!replayable(&mut s.e, 4, ATTACK));
    assert!(!replayable(&mut s.e, 4, AIM));
}

#[test]
fn an_avatar_plays_a_group_and_an_idle_as_the_commands_do() {
    let mut s = story();
    let a = avatar(&mut s);
    s.group_section(HIT as u8, 2);
    assert!(play_group(&mut s.e, a, HIT as u8));
    let shape = s.shape(a);
    // Its own group for the type: the kit's weapon bits added.
    assert_eq!(shape.plays, [HIT | anim_kit::WEAPON_BITS]);
    assert_eq!(shape.actions, [addr::ANIM_ACTION_SCRIPT]);
    assert_eq!(s.group_in(a, 2), HIT | anim_kit::WEAPON_BITS);
    assert!(play_idle(&mut s.e, a, WAVE));
    assert_eq!(s.shape(a).idles, [WAVE]);
    assert_eq!(body_of_id(&mut s.e, a).unwrap().idle, WAVE);
    // The idle it plays already: nothing asked.
    assert!(!play_idle(&mut s.e, a, WAVE));
    assert_eq!(s.shape(a).idles, [WAVE]);
    // Not here, or an idle this game lacks: nothing.
    assert!(!play_group(&mut s.e, 0x0010_9999, HIT as u8));
    assert!(!play_idle(&mut s.e, a, 0x0010_9998));
}

#[test]
fn an_avatar_jumps_aims_keeps_its_players_speed_and_falls_down() {
    let mut s = story();
    let a = avatar(&mut s);
    assert!(jump(&mut s.e, a));
    let shape = s.shape(a);
    assert_eq!(shape.jumps, 1);
    assert_eq!(
        body_of_id(&mut s.e, a).unwrap().controller,
        addr::CONTROLLER_JUMPING
    );
    assert!(set_sights(&mut s.e, a, true));
    assert!(body_of_id(&mut s.e, a).unwrap().sights);
    assert!(set_speeds(&mut s.e, a, 95.0, 410.0));
    let b = body_of_id(&mut s.e, a).unwrap();
    assert_eq!((b.walk, b.run), (95.0, 410.0));
    // Knocked down by its fatigue, and up again with it given back.
    assert!(knock(&mut s.e, a, true));
    assert_eq!(body_of_id(&mut s.e, a).unwrap().knock, anim_kit::KNOCKED);
    assert!(knock(&mut s.e, a, false));
    assert_eq!(body_of_id(&mut s.e, a).unwrap().knock, 0);
    assert_eq!(s.shape(a).fatigue, anim_kit::FATIGUE);
    assert!(!jump(&mut s.e, 0x0010_9999));
}

#[test]
fn an_avatar_sits_on_its_players_furniture_and_gets_up() {
    let mut s = story();
    let a = avatar(&mut s);
    assert!(sit(&mut s.e, a, CHAIR));
    let b = body_of_id(&mut s.e, a).unwrap();
    assert_eq!((b.furniture, b.sit), (CHAIR, anim_kit::SITTING));
    assert_eq!(s.shape(a).sits, [CHAIR]);
    assert!(get_up(&mut s.e, a));
    let b = body_of_id(&mut s.e, a).unwrap();
    assert_eq!((b.furniture, b.sit), (0, 0));
    assert!(!sit(&mut s.e, a, 0x0010_9999));
}

#[test]
fn a_heading_that_changes_while_standing_turns_on_the_spot() {
    let left = addr::MOVE_TURN_LEFT;
    let right = addr::MOVE_TURN_RIGHT;
    assert_eq!(turn(0, 1.0, 1.1), left);
    assert_eq!(turn(0, 1.0, 0.9), right);
    // Across the full circle, the short way.
    assert_eq!(turn(0, 6.25, 0.05), left);
    assert_eq!(turn(0, 0.05, 6.25), right);
    // Too small a change, or moving, or already turning: as the player's.
    assert_eq!(turn(0, 1.0, 1.005), 0);
    assert_eq!(turn(0x201, 1.0, 2.0), 0x201);
    assert_eq!(turn(right, 1.0, 2.0), right);
}

fn a_face() -> Face {
    Face {
        female: true,
        race: OTHER_RACE,
        hair: HAIR,
        hair_length: 0.25,
        eyes: EYES,
        hair_color: 0x0040_2010,
        height: 1.02,
        weight: 0.5,
        head_parts: vec![NOSE],
        coord: [
            (3, 1, vec![0.5, -1.25, 2.0]),
            (2, 1, vec![0.125, -0.75]),
            (0, 0, Vec::new()),
            (0, 0, Vec::new()),
        ],
        name: "Courier Two".into(),
    }
}

#[test]
fn a_saved_look_is_written_onto_the_player_through_the_menus_calls_and_its_head_rebuilt() {
    let mut s = story();
    let mut mine = a_face();
    mine.female = false;
    mine.race = RACE;
    mine.hair = 0;
    mine.name = "Courier One".into();
    assert!(look::set_face(&mut s.e, 7, &mine));
    let r = look::restore(&mut s.e, &a_face()).unwrap();
    assert!(r.race && r.sex && r.face && r.head);
    assert_eq!(look::player_face(&mut s.e), Some(a_face()));
    let calls = s.menu_calls();
    // The race as the menu's race list sets it, in its order; the sex, hair and eyes; the name.
    let race_calls = [
        addr::PLAYER_RACE_CHANGED,
        addr::PLAYER_RECALCULATE,
        addr::PLAYER_UPDATE,
        addr::NPC_SET_RACE,
    ];
    let at = calls
        .iter()
        .position(|c| *c == race_calls[0])
        .expect("the race changed");
    assert_eq!(calls[at..at + 4], race_calls);
    for c in [
        addr::RACE_KILL_EGT_DATA,
        addr::ACTOR_BASE_SET_FLAG_BIT,
        addr::NPC_SET_HAIR,
        addr::UPDATE_PLAYER_NAME,
        addr::PLAYER_REFRESH_3D,
        addr::NPC_UPDATE_3D,
    ] {
        assert!(calls.contains(&c), "{c:08x} called");
    }
    assert!(
        !calls.contains(&addr::NPC_SET_EYES),
        "the eyes were the same"
    );
    // The head built as a loaded save's: cleared, built, the face applied to both nodes.
    let shape = s.shape(player());
    assert_eq!(
        (shape.cleared, shape.heads_built, shape.faces_applied),
        (1, 1, 2)
    );
    // Built again: the old head nodes taken off first.
    assert!(look::rebuild_head(&mut s.e, player()));
    let shape = s.shape(player());
    assert_eq!((shape.heads_taken_off, shape.heads_built), (2, 2));
    // The same look again: nothing asked of the game.
    let before = s.menu_calls().len();
    assert_eq!(
        look::restore(&mut s.e, &a_face()),
        Some(look::Restored::default())
    );
    assert_eq!(s.menu_calls().len(), before);
}

#[test]
fn a_look_with_the_same_race_and_sex_changes_neither() {
    let mut s = story();
    let mut mine = a_face();
    mine.name = "Courier One".into();
    mine.hair_color = 7;
    assert!(look::set_face(&mut s.e, 7, &mine));
    let r = look::restore(&mut s.e, &a_face()).unwrap();
    assert!(!r.race && !r.sex && r.face);
    let calls = s.menu_calls();
    assert!(!calls.contains(&addr::NPC_SET_RACE));
    assert!(!calls.contains(&addr::ACTOR_BASE_SET_FLAG_BIT));
    assert!(!calls.contains(&addr::NPC_UPDATE_3D));
    assert_eq!(look::player_face(&mut s.e), Some(a_face()));
}

#[test]
fn an_avatars_head_is_built_from_its_base_forms_face() {
    let mut s = story();
    let copy = look::base_copy(&mut s.e, 7).unwrap();
    assert!(look::set_face(&mut s.e, copy, &a_face()));
    let a = live::avatar_create(&mut s.e, copy).unwrap().id;
    assert!(look::rebuild_head(&mut s.e, a));
    let shape = s.shape(a);
    assert_eq!(
        (shape.cleared, shape.heads_built, shape.faces_applied),
        (1, 1, 2)
    );
    // Without its 3D: nothing.
    s.set_has_3d(a, false);
    assert!(!look::rebuild_head(&mut s.e, a));
}

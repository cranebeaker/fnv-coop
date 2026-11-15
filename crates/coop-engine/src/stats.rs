//! The player's own character (docs/coop/SPECS.md S30): the SPECIAL attributes and the skills,
//! the level, experience and karma, the perks with their ranks, and what is worn, read and set
//! through the game's own functions. Co-op wraps nothing here: every function is called, and
//! only by a session (a guest's part of a party save, a conversation's reward).
//!
//! - **Attributes and skills** are the player's own values, read and stored as the level-up
//!   menu reads and stores a skill (the player's virtual `+0x48c` and `+0x394`): the value the
//!   character has by itself, without what worn items and effects add.
//! - **The level** is set as the set-level command sets the player's
//!   (`CharacterProgression::ForceLevel`), which also moves the experience; **experience** and
//!   **karma** are then changed by the difference, through the actor's integer change of a
//!   value (what `RewardExperience` and `RewardKarma` end in).
//! - **Perks** are the player's list of perk and rank, given and taken through the player's own
//!   functions.
//! - **Worn items** are the stacks of the inventory marked worn; one is put on as the
//!   `EquipItem` command does (`Actor::QueueEquipObject`).

use engine_core::prelude::*;

use crate::{addr, forms, items, live};

/// The first form id of a form made while playing: another game does not know it.
const FIRST_DYNAMIC_ID: u32 = 0xff00_0000;
/// The most nodes of one of the game's lists a walk goes through.
const MAX_NODES: usize = 1 << 12;

/// How many SPECIAL attributes and skills a sheet holds.
pub const SPECIAL: usize = addr::ACTOR_VALUE_SPECIAL_COUNT as usize;
/// See [`SPECIAL`].
pub const SKILLS: usize = addr::ACTOR_VALUE_SKILL_COUNT as usize;

/// A character as a party save keeps it.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Sheet {
    /// The player's own value of each SPECIAL attribute (actor values 5 to 0xb, in order).
    pub special: [f32; SPECIAL],
    /// The player's own value of each skill (actor values 0x20 to 0x2d, in order).
    pub skills: [f32; SKILLS],
    /// The level.
    pub level: u16,
    /// The experience.
    pub xp: i32,
    /// The karma.
    pub karma: i32,
    /// The perks: (the perk's form id, its rank).
    pub perks: Vec<(u32, u8)>,
    /// The form ids of the items worn.
    pub worn: Vec<u32>,
}

/// The actor value codes of the attributes and skills, in a sheet's order.
pub fn codes() -> impl Iterator<Item = u32> {
    let special = addr::ACTOR_VALUE_SPECIAL_FIRST
        ..addr::ACTOR_VALUE_SPECIAL_FIRST + addr::ACTOR_VALUE_SPECIAL_COUNT;
    let skills =
        addr::ACTOR_VALUE_SKILL_FIRST..addr::ACTOR_VALUE_SKILL_FIRST + addr::ACTOR_VALUE_SKILL_COUNT;
    special.chain(skills)
}

/// Whether this game has the functions a character is read and set through (a test engine that
/// holds only what its test needs has not).
pub fn available(e: &Engine) -> bool {
    e.is_registered(addr::ACTOR_GET_LEVEL)
}

/// The player's own value of actor value `code`, as the level-up menu reads it.
pub fn value(e: &mut Engine, player: u32, code: u32) -> f32 {
    e.with_stack(4, |e, kept| {
        e.mem.set_u32(kept.addr(), 0);
        e.vcall(
            player,
            addr::PLAYER_OWN_VALUE_SLOT,
            &args![code, kept.addr()],
        )
        .f32()
    })
}

/// Stores `to` as the player's own value of actor value `code`, as the level-up menu stores a
/// skill. False when it is that already.
pub fn set_value(e: &mut Engine, player: u32, code: u32, to: f32) -> bool {
    if value(e, player, code) == to {
        return false;
    }
    e.vcall(player, addr::PLAYER_SET_OWN_VALUE_SLOT, &args![code, to]);
    true
}

/// Actor value `code` of the player as the integer the game reads it as (experience, karma).
pub fn int_value(e: &mut Engine, player: u32, code: u32) -> i32 {
    e.vcall(
        player + addr::ACTOR_VALUE_OWNER_OFFSET,
        addr::OWNER_INT_VALUE_SLOT,
        &args![code],
    )
    .i32()
}

/// Changes actor value `code` of the player by `by`, as `RewardExperience` and `RewardKarma`
/// change theirs. False for no change.
pub fn change_int(e: &mut Engine, player: u32, code: u32, by: i32) -> bool {
    if by == 0 {
        return false;
    }
    e.vcall(
        player,
        addr::ACTOR_MODIFY_INT_VALUE_SLOT,
        &args![code, by as u32, 0u32],
    );
    true
}

/// The player's level.
pub fn level(e: &mut Engine, player: u32) -> u16 {
    e.call(addr::ACTOR_GET_LEVEL, &args![player]).u16()
}

/// The player's level-up state, or 0.
fn level_state(e: &mut Engine, player: u32) -> u32 {
    match e.call(addr::PLAYER_PROGRESSION, &args![player]).u32() {
        0 => 0,
        holder => e.mem.u32(holder),
    }
}

/// Sets or clears the byte that keeps a change of the player's experience from being shown and
/// judged for a level-up (the game's own `ForceLevel` sets it around its change). Answers what
/// it was.
pub fn set_quiet(e: &mut Engine, player: u32, on: bool) -> bool {
    let state = level_state(e, player);
    if state == 0 {
        return false;
    }
    let at = state + addr::LEVEL_STATE_QUIET_OFFSET;
    let was = e.mem.u8(at) != 0;
    e.mem.set_u8(at, u8::from(on));
    was
}

/// Gives the player karma (or takes it) through the game's own `RewardKarma`: within the
/// game's limits, with its message and sound.
pub fn reward_karma(e: &mut Engine, player: u32, by: i32) {
    if by != 0 {
        e.call(addr::PLAYER_REWARD_KARMA, &args![player, by as u32]);
    }
}

/// The data of each node of one of the game's lists (data, next), empty nodes left out.
fn nodes(e: &mut Engine, list: u32) -> Vec<u32> {
    let mut out = Vec::new();
    let (mut node, mut walked) = (list, 0);
    while node != 0 && walked < MAX_NODES {
        let data = e.mem.u32(node);
        if data != 0 {
            out.push(data);
        }
        node = e.mem.u32(node + 4);
        walked += 1;
    }
    out
}

/// The player's perks: (the perk's form id, its rank). A perk made while playing is left out.
pub fn perks(e: &mut Engine, player: u32) -> Vec<(u32, u8)> {
    let mut out = Vec::new();
    for data in nodes(e, player + addr::PLAYER_PERK_LIST_OFFSET) {
        let perk = e.mem.u32(data);
        if perk == 0 {
            continue;
        }
        let id = e.mem.u32(perk + addr::FORM_ID_OFFSET);
        if id != 0 && id < FIRST_DYNAMIC_ID {
            out.push((id, e.mem.u8(data + addr::PERK_RANK_OFFSET)));
        }
    }
    out
}

/// The perk with form id `id`, or `None` when the data lack it or the form is not a perk.
fn perk_form(e: &mut Engine, id: u32) -> Option<u32> {
    let f = forms::form(e, id)?;
    (e.mem.u8(f + addr::FORM_TYPE_OFFSET) == addr::FORM_TYPE_PERK).then_some(f)
}

/// Gives the player perk `id` at `rank` (or changes its rank). False when this game has no such
/// perk.
pub fn set_perk(e: &mut Engine, player: u32, id: u32, rank: u8) -> bool {
    let Some(perk) = perk_form(e, id) else {
        return false;
    };
    e.call(
        addr::PLAYER_SET_PERK_RANK,
        &args![player, perk, u32::from(rank), 0u32],
    );
    true
}

/// Takes perk `id` away from the player. False when this game has no such perk.
pub fn remove_perk(e: &mut Engine, player: u32, id: u32) -> bool {
    let Some(perk) = perk_form(e, id) else {
        return false;
    };
    e.call(addr::PLAYER_REMOVE_PERK, &args![player, perk, 0u32]);
    true
}

/// The form ids of the items reference `r` wears: the items of its inventory with a stack marked
/// worn. An item made while playing is left out.
pub fn worn(e: &mut Engine, r: u32) -> Vec<u32> {
    if !e.is_registered(addr::REF_INVENTORY_CHANGES) {
        return Vec::new();
    }
    let inv = e.call(addr::REF_INVENTORY_CHANGES, &args![r]).u32();
    if inv == 0 {
        return Vec::new();
    }
    let mut out = Vec::new();
    let entries = e.mem.u32(inv);
    for entry in nodes(e, entries) {
        let form = e.mem.u32(entry + addr::ENTRY_FORM_OFFSET);
        if form == 0 {
            continue;
        }
        let extend = e.mem.u32(entry + addr::ENTRY_EXTEND_OFFSET);
        let is_worn = nodes(e, extend).into_iter().any(|xdata| {
            e.call(
                addr::EXTRA_GET_BY_TYPE,
                &args![xdata, addr::EXTRA_TYPE_WORN],
            )
            .u32()
                != 0
        });
        let id = e.mem.u32(form + addr::FORM_ID_OFFSET);
        if is_worn && id != 0 && id < FIRST_DYNAMIC_ID && !out.contains(&id) {
            out.push(id);
        }
    }
    out
}

/// Has reference `r` put item `item` on, as the `EquipItem` command does (one of them, free to
/// be taken off again). False when it is worn already, `r` holds none, or the form is no item.
pub fn equip(e: &mut Engine, r: u32, item: u32) -> bool {
    if !e.is_registered(addr::REF_INVENTORY_CHANGES) {
        return false;
    }
    let Some(form) = items::item_form(e, item) else {
        return false;
    };
    if items::count(e, r, item) <= 0 || worn(e, r).contains(&item) {
        return false;
    }
    e.call(
        addr::ACTOR_QUEUE_EQUIP,
        &args![r, form, 1u32, 0u32, 1u32, 0u32, 1u32],
    );
    true
}

/// The character of this game's player; `None` with no player, or in a game without the
/// functions ([`available`]).
pub fn read(e: &mut Engine) -> Option<Sheet> {
    if !available(e) {
        return None;
    }
    let player = live::player(e)?;
    let mut sheet = Sheet {
        level: level(e, player),
        xp: int_value(e, player, addr::ACTOR_VALUE_EXPERIENCE),
        karma: int_value(e, player, addr::ACTOR_VALUE_KARMA),
        perks: perks(e, player),
        worn: worn(e, player),
        ..Sheet::default()
    };
    for (i, code) in codes().enumerate() {
        let v = value(e, player, code);
        match i.checked_sub(SPECIAL) {
            None => sheet.special[i] = v,
            Some(skill) => sheet.skills[skill] = v,
        }
    }
    Some(sheet)
}

/// Makes this game's player the character of `sheet`: the attributes and skills stored, the
/// level forced, experience and karma moved to theirs, perks given, changed and (those the
/// sheet lacks) taken away, and what was worn put on where the player carries it. Answers how
/// many things it changed; 0 with no player or without the functions.
pub fn apply(e: &mut Engine, sheet: &Sheet) -> u32 {
    if !available(e) {
        return 0;
    }
    let Some(player) = live::player(e) else {
        return 0;
    };
    let mut changed = 0;
    let wanted = sheet.special.iter().chain(&sheet.skills);
    for (code, to) in codes().zip(wanted) {
        changed += u32::from(set_value(e, player, code, *to));
    }
    if sheet.level != 0 && level(e, player) != sheet.level {
        let progression = e.call(addr::PLAYER_PROGRESSION, &args![player]).u32();
        if progression != 0 {
            e.call(
                addr::PROGRESSION_FORCE_LEVEL,
                &args![progression, u32::from(sheet.level)],
            );
            changed += 1;
        }
    }
    for (code, to) in [
        (addr::ACTOR_VALUE_EXPERIENCE, sheet.xp),
        (addr::ACTOR_VALUE_KARMA, sheet.karma),
    ] {
        let by = to.wrapping_sub(int_value(e, player, code));
        changed += u32::from(change_int(e, player, code, by));
    }
    let have = perks(e, player);
    for (id, _) in &have {
        if !sheet.perks.iter().any(|p| p.0 == *id) && remove_perk(e, player, *id) {
            changed += 1;
        }
    }
    for (id, rank) in &sheet.perks {
        if !have.contains(&(*id, *rank)) && set_perk(e, player, *id, *rank) {
            changed += 1;
        }
    }
    for item in &sheet.worn {
        changed += u32::from(equip(e, player, *item));
    }
    changed
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::itemkit::{self, ItemKitSpec, ItemSpec};
    use crate::statkit as kit;
    use crate::testkit::{Story, StorySpec};

    const CELL: u32 = 0x0010_5a00;
    const TOUGHNESS: u32 = 0x0003_1dd8;
    const EDUCATED: u32 = 0x0003_1dd9;
    const ARMOR: u32 = 0x0002_0420;
    const RIFLE: u32 = 0x0000_4333;
    const SPEECH: u32 = addr::ACTOR_VALUE_SKILL_FIRST + 0xb;
    const STRENGTH: u32 = addr::ACTOR_VALUE_SPECIAL_FIRST;

    fn story() -> Story {
        let mut s = Story::new(&StorySpec {
            cells: vec![CELL],
            ..StorySpec::default()
        });
        itemkit::install(
            &mut s,
            &ItemKitSpec {
                items: vec![
                    ItemSpec {
                        id: ARMOR,
                        health: 100,
                    },
                    ItemSpec {
                        id: RIFLE,
                        health: 200,
                    },
                ],
                ..ItemKitSpec::default()
            },
        );
        kit::install(&mut s, &[TOUGHNESS, EDUCATED]);
        s.move_player(CELL, [0.0; 3], 0.0);
        s
    }

    #[test]
    fn the_codes_are_the_seven_attributes_and_the_fourteen_skills() {
        let all: Vec<u32> = codes().collect();
        assert_eq!(all.len(), SPECIAL + SKILLS);
        assert_eq!(all[..SPECIAL], [5, 6, 7, 8, 9, 0xa, 0xb]);
        assert_eq!(all[SPECIAL], 0x20);
        assert_eq!(all[SPECIAL + SKILLS - 1], 0x2d);
    }

    #[test]
    fn a_game_without_the_functions_has_no_sheet_and_takes_none() {
        let mut s = Story::new(&StorySpec {
            cells: vec![CELL],
            ..StorySpec::default()
        });
        assert!(!available(&s.e));
        assert_eq!(read(&mut s.e), None);
        assert_eq!(apply(&mut s.e, &Sheet::default()), 0);
    }

    #[test]
    fn a_character_is_read_through_the_games_functions() {
        let mut s = story();
        let player = live::player(&mut s.e).unwrap();
        kit::set(&mut s, STRENGTH, 7.0);
        kit::set(&mut s, SPEECH, 45.0);
        kit::set_level(&mut s, 4, 3200);
        assert!(set_perk(&mut s.e, player, TOUGHNESS, 2));
        // Not a perk, not in the data: nothing is given.
        assert!(!set_perk(&mut s.e, player, ARMOR, 1));
        assert!(!set_perk(&mut s.e, player, 0x0099_9999, 1));
        reward_karma(&mut s.e, player, -30);
        items::add(&mut s.e, player, ARMOR, 1, None);
        items::add(&mut s.e, player, RIFLE, 1, Some(0.5));
        assert!(equip(&mut s.e, player, ARMOR));
        // Worn already, not carried, not an item.
        assert!(!equip(&mut s.e, player, ARMOR));
        assert!(!equip(&mut s.e, player, 0x0001_5169));
        assert!(!equip(&mut s.e, player, TOUGHNESS));
        let sheet = read(&mut s.e).unwrap();
        assert_eq!(sheet.special[0], 7.0);
        assert_eq!(sheet.special[1], 5.0);
        assert_eq!(sheet.skills[0xb], 45.0);
        assert_eq!(
            (sheet.level, sheet.xp, sheet.karma),
            (4, 3200, -30),
            "level, experience, karma"
        );
        assert_eq!(sheet.perks, [(TOUGHNESS, 2)]);
        assert_eq!(sheet.worn, [ARMOR]);
        // Wearing an item does not change what the player is listed to carry.
        assert_eq!(items::count(&mut s.e, player, ARMOR), 1);
        assert_eq!(items::list(&mut s.e, player).len(), 2);
    }

    #[test]
    fn a_sheet_makes_another_games_player_that_character() {
        let mut a = story();
        let pa = live::player(&mut a.e).unwrap();
        kit::set(&mut a, STRENGTH, 8.0);
        kit::set(&mut a, SPEECH, 60.0);
        kit::set_level(&mut a, 6, 5400);
        set_perk(&mut a.e, pa, TOUGHNESS, 1);
        reward_karma(&mut a.e, pa, 120);
        items::add(&mut a.e, pa, ARMOR, 1, None);
        equip(&mut a.e, pa, ARMOR);
        let sheet = read(&mut a.e).unwrap();

        // Another game: a new character with a perk of its own, carrying the armor unworn.
        let mut b = story();
        let pb = live::player(&mut b.e).unwrap();
        set_perk(&mut b.e, pb, EDUCATED, 1);
        items::add(&mut b.e, pb, ARMOR, 1, None);
        let shown = kit::counts(&mut b);
        // Two values, the level, experience, karma, a perk off, a perk on, the armor.
        assert_eq!(apply(&mut b.e, &sheet), 8);
        assert_eq!(read(&mut b.e), Some(sheet.clone()));
        // The level went through the game's own ForceLevel, karma through the value's change.
        let after = kit::counts(&mut b);
        assert_eq!(after.forced, shown.forced + 1);
        assert_eq!(after.karma_messages, shown.karma_messages);
        // Applied again it changes nothing.
        assert_eq!(apply(&mut b.e, &sheet), 0);
        // A perk at another rank is changed, not added twice.
        let mut higher = sheet.clone();
        higher.perks = vec![(TOUGHNESS, 2)];
        assert_eq!(apply(&mut b.e, &higher), 1);
        assert_eq!(perks(&mut b.e, pb), [(TOUGHNESS, 2)]);
        // An item the player does not carry is not put on; a perk this game lacks is left out.
        let mut other = higher.clone();
        other.worn = vec![ARMOR, RIFLE];
        other.perks.push((0x0099_9999, 1));
        assert_eq!(apply(&mut b.e, &other), 0);
        assert_eq!(worn(&mut b.e, pb), [ARMOR]);
    }

    #[test]
    fn experience_changes_quietly_only_while_the_games_byte_is_set() {
        let mut s = story();
        let player = live::player(&mut s.e).unwrap();
        assert!(change_int(
            &mut s.e,
            player,
            addr::ACTOR_VALUE_EXPERIENCE,
            50
        ));
        assert_eq!(kit::counts(&mut s).gains_shown, 1);
        assert!(!set_quiet(&mut s.e, player, true));
        change_int(&mut s.e, player, addr::ACTOR_VALUE_EXPERIENCE, 50);
        assert!(set_quiet(&mut s.e, player, false));
        assert_eq!(kit::counts(&mut s).gains_shown, 1);
        assert_eq!(int_value(&mut s.e, player, addr::ACTOR_VALUE_EXPERIENCE), 100);
        assert!(!change_int(&mut s.e, player, addr::ACTOR_VALUE_KARMA, 0));
    }
}

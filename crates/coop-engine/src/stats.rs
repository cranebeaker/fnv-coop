//! The player's own character (docs/coop/SPECS.md S32), read and set through the game's own
//! functions. Co-op wraps nothing here: every function is called, and only by a session (a
//! guest's part of a party save being taken back).
//!
//! - **The character** ([`Character`]): the SPECIAL attributes and the skills as base values (the
//!   value owner's `+0x04`, changed by `00881360`, which adds to a base value as the game does),
//!   the tag skills (the player's class, as the `SetPlayerTagSkill` command sets them), the level
//!   (`CharacterProgression::ForceLevel`, as the set-level command), the experience (the actor's
//!   integer change of a value, with the game's quiet byte set as `ForceLevel` sets it), and the
//!   perks with their ranks (the player's own list and functions; traits are perks of that list
//!   when the game keeps them there).
//! - **The condition** ([`Condition`]): health, action points, radiation, the seven body
//!   conditions as current values (the owner's `+0x0c`, changed through the actor's `+0x3ac`, the
//!   way damage, restoring and radiation change them), and karma (the integer change).
//! - **What is worn**: the stacks of the inventory marked worn; one is put on as the `EquipItem`
//!   command does (`Actor::QueueEquipObject`) and taken off as `UnequipItem` does.

use engine_core::prelude::*;

use crate::{addr, forms, items, live};

/// The first form id of a form made while playing: another game does not know it.
const FIRST_DYNAMIC_ID: u32 = 0xff00_0000;
/// The most nodes of one of the game's lists a walk goes through.
const MAX_NODES: usize = 1 << 12;

/// How many SPECIAL attributes, skills, tag skills and body conditions a sheet holds.
pub const SPECIAL: usize = addr::ACTOR_VALUE_SPECIAL_COUNT as usize;
/// See [`SPECIAL`].
pub const SKILLS: usize = addr::ACTOR_VALUE_SKILL_COUNT as usize;
/// See [`SPECIAL`].
pub const TAGS: usize = addr::TAG_SKILL_COUNT as usize;
/// See [`SPECIAL`].
pub const CONDITIONS: usize = addr::ACTOR_VALUE_CONDITION_COUNT as usize;

/// What a character is made of.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Character {
    /// The base value of each SPECIAL attribute (actor values 5 to 0xb, in order).
    pub special: [f32; SPECIAL],
    /// The base value of each skill (actor values 0x20 to 0x2d, in order).
    pub skills: [f32; SKILLS],
    /// The tag skills: a skill's actor value, or -1 for none.
    pub tags: [i32; TAGS],
    /// The level.
    pub level: u16,
    /// The experience.
    pub xp: i32,
    /// The perks: (the perk's form id, its rank).
    pub perks: Vec<(u32, u8)>,
}

/// How a character is.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Condition {
    /// Health, action points and radiation, current values.
    pub health: f32,
    /// See [`Condition::health`].
    pub action_points: f32,
    /// See [`Condition::health`].
    pub radiation: f32,
    /// The current value of each body condition (actor values 0x19 to 0x1f, in order).
    pub body: [f32; CONDITIONS],
    /// The karma.
    pub karma: i32,
}

/// A character as a party save keeps it. A part that this game could not read is `None`.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Sheet {
    /// See [`Character`].
    pub character: Option<Character>,
    /// See [`Condition`].
    pub condition: Option<Condition>,
    /// The form ids of the items worn.
    pub worn: Option<Vec<u32>>,
}

/// What [`apply`] changed, by part.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Changed {
    /// Values, the level, the experience, tag skills and perks set.
    pub character: u32,
    /// Condition values and karma set.
    pub condition: u32,
    /// Items put on or taken off.
    pub worn: u32,
}

impl Changed {
    /// All the changes.
    pub fn total(&self) -> u32 {
        self.character + self.condition + self.worn
    }
}

/// The actor value codes of the attributes and skills, in a sheet's order.
pub fn codes() -> impl Iterator<Item = u32> {
    let special = addr::ACTOR_VALUE_SPECIAL_FIRST
        ..addr::ACTOR_VALUE_SPECIAL_FIRST + addr::ACTOR_VALUE_SPECIAL_COUNT;
    let skills = addr::ACTOR_VALUE_SKILL_FIRST
        ..addr::ACTOR_VALUE_SKILL_FIRST + addr::ACTOR_VALUE_SKILL_COUNT;
    special.chain(skills)
}

/// The actor value codes of the body conditions, in a sheet's order.
pub fn condition_codes() -> impl Iterator<Item = u32> {
    addr::ACTOR_VALUE_CONDITION_FIRST
        ..addr::ACTOR_VALUE_CONDITION_FIRST + addr::ACTOR_VALUE_CONDITION_COUNT
}

/// Whether this game has the functions a character is read and set through (a test engine that
/// holds only what its test needs has not).
pub fn available(e: &Engine) -> bool {
    e.is_registered(addr::ACTOR_GET_LEVEL) && e.is_registered(addr::ACTOR_MOD_BASE_VALUE)
}

fn owner(actor: u32) -> u32 {
    actor.wrapping_add(addr::ACTOR_VALUE_OWNER_OFFSET)
}

/// The base value of actor value `code` of `actor`.
pub fn base_value(e: &mut Engine, actor: u32, code: u32) -> f32 {
    e.vcall(owner(actor), addr::OWNER_BASE_VALUE_SLOT, &args![code])
        .f32()
}

/// Makes `to` the base value of actor value `code` of `actor`, through the game's own addition
/// to a base value. False when it is that already.
pub fn set_base_value(e: &mut Engine, actor: u32, code: u32, to: f32) -> bool {
    let have = base_value(e, actor, code);
    if have == to {
        return false;
    }
    e.call(
        addr::ACTOR_MOD_BASE_VALUE,
        &args![actor, code, (to - have).to_bits()],
    );
    true
}

/// The current value of actor value `code` of `actor`.
pub fn current(e: &mut Engine, actor: u32, code: u32) -> f32 {
    e.vcall(owner(actor), addr::OWNER_CURRENT_VALUE_SLOT, &args![code])
        .f32()
}

/// Makes `to` the current value of actor value `code` of `actor`, changed by the difference
/// through the actor's change of a value (what damage, restoring and radiation call). False when
/// it is that already.
pub fn set_current(e: &mut Engine, actor: u32, code: u32, to: f32) -> bool {
    let by = to - current(e, actor, code);
    if by == 0.0 || !by.is_finite() {
        return false;
    }
    e.vcall(
        actor,
        addr::ACTOR_MODIFY_VALUE_SLOT,
        &args![code, by.to_bits(), 0u32],
    );
    true
}

/// Actor value `code` of `actor` as the integer the game reads it as (experience, karma).
pub fn int_value(e: &mut Engine, actor: u32, code: u32) -> i32 {
    e.vcall(owner(actor), addr::OWNER_INT_VALUE_SLOT, &args![code])
        .i32()
}

/// Changes actor value `code` of `actor` by `by`, as `RewardExperience` and `RewardKarma` change
/// theirs. False for no change.
pub fn change_int(e: &mut Engine, actor: u32, code: u32, by: i32) -> bool {
    if by == 0 {
        return false;
    }
    e.vcall(
        actor,
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

/// Makes the player level `to` as the set-level command does. False when it is that already or
/// the player has no progression.
pub fn set_level(e: &mut Engine, player: u32, to: u16) -> bool {
    if to == 0 || level(e, player) == to {
        return false;
    }
    let progression = e.call(addr::PLAYER_PROGRESSION, &args![player]).u32();
    if progression == 0 {
        return false;
    }
    e.call(
        addr::PROGRESSION_FORCE_LEVEL,
        &args![progression, u32::from(to)],
    );
    true
}

/// Makes `to` the player's experience, quietly (no message, no level-up judged). False when it
/// is that already.
pub fn set_xp(e: &mut Engine, player: u32, to: i32) -> bool {
    let by = to.wrapping_sub(int_value(e, player, addr::ACTOR_VALUE_EXPERIENCE));
    if by == 0 {
        return false;
    }
    let was = set_quiet(e, player, true);
    change_int(e, player, addr::ACTOR_VALUE_EXPERIENCE, by);
    set_quiet(e, player, was);
    true
}

/// The player's class, or 0.
fn class(e: &mut Engine, player: u32) -> u32 {
    if !e.is_registered(addr::ACTOR_GET_CLASS) {
        return 0;
    }
    e.call(addr::ACTOR_GET_CLASS, &args![player]).u32()
}

/// The player's tag skills (-1 for none); all -1 without a class.
pub fn tags(e: &mut Engine, player: u32) -> [i32; TAGS] {
    let c = class(e, player);
    let mut out = [-1; TAGS];
    if c != 0 {
        for (i, t) in out.iter_mut().enumerate() {
            *t = e.mem.i32(c + addr::CLASS_TAG_SKILLS_OFFSET + 4 * i as u32);
        }
    }
    out
}

/// Makes `to` the player's tag skills, each through `TESClass::SetTagSkill`. Answers how many
/// it changed.
pub fn set_tags(e: &mut Engine, player: u32, to: &[i32; TAGS]) -> u32 {
    let c = class(e, player);
    if c == 0 {
        return 0;
    }
    let have = tags(e, player);
    let mut changed = 0;
    for (i, (h, w)) in have.iter().zip(to).enumerate() {
        if h != w {
            e.call(addr::CLASS_SET_TAG_SKILL, &args![c, i as u32, *w as u32]);
            changed += 1;
        }
    }
    changed
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

/// The form ids of the items reference `r` wears, in order: the items of its inventory with a
/// stack marked worn. An item made while playing is left out.
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
    out.sort_unstable();
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

/// Has reference `r` take item `item` off, as the `UnequipItem` command does. False when it does
/// not wear it.
pub fn unequip(e: &mut Engine, r: u32, item: u32) -> bool {
    if !worn(e, r).contains(&item) {
        return false;
    }
    let Some(form) = items::item_form(e, item) else {
        return false;
    };
    e.call(
        addr::ACTOR_QUEUE_UNEQUIP,
        &args![r, form, 1u32, 0u32, 1u32, 0u32, 1u32],
    );
    true
}

/// The character of `player`.
pub fn read_character(e: &mut Engine, player: u32) -> Character {
    let mut c = Character {
        tags: tags(e, player),
        level: level(e, player),
        xp: int_value(e, player, addr::ACTOR_VALUE_EXPERIENCE),
        perks: perks(e, player),
        ..Character::default()
    };
    for (i, code) in codes().enumerate() {
        let v = base_value(e, player, code);
        match i.checked_sub(SPECIAL) {
            None => c.special[i] = v,
            Some(skill) => c.skills[skill] = v,
        }
    }
    c
}

/// Makes `player` the character `c`. The level first (it moves the experience), then the
/// experience, the perks (those `c` lacks taken away), the tag skills, and the attributes and
/// skills last, so that what a perk's effects did to a base value is set back. Answers how many
/// things it changed.
pub fn apply_character(e: &mut Engine, player: u32, c: &Character) -> u32 {
    let mut changed = u32::from(set_level(e, player, c.level));
    changed += u32::from(set_xp(e, player, c.xp));
    let have = perks(e, player);
    for (id, _) in &have {
        if !c.perks.iter().any(|p| p.0 == *id) && remove_perk(e, player, *id) {
            changed += 1;
        }
    }
    for (id, rank) in &c.perks {
        if !have.contains(&(*id, *rank)) && set_perk(e, player, *id, *rank) {
            changed += 1;
        }
    }
    changed += set_tags(e, player, &c.tags);
    let wanted = c.special.iter().chain(&c.skills);
    for (code, to) in codes().zip(wanted) {
        changed += u32::from(set_base_value(e, player, code, *to));
    }
    changed
}

/// The condition of `actor`.
pub fn read_condition(e: &mut Engine, actor: u32) -> Condition {
    let mut c = Condition {
        health: current(e, actor, addr::ACTOR_VALUE_HEALTH),
        action_points: current(e, actor, addr::ACTOR_VALUE_ACTION_POINTS),
        radiation: current(e, actor, addr::ACTOR_VALUE_RADIATION),
        karma: int_value(e, actor, addr::ACTOR_VALUE_KARMA),
        ..Condition::default()
    };
    for (v, code) in c.body.iter_mut().zip(condition_codes()) {
        *v = current(e, actor, code);
    }
    c
}

/// Makes `actor`'s condition `c`. Answers how many values it changed.
pub fn apply_condition(e: &mut Engine, actor: u32, c: &Condition) -> u32 {
    let mut changed = 0;
    let values = [
        (addr::ACTOR_VALUE_HEALTH, c.health),
        (addr::ACTOR_VALUE_ACTION_POINTS, c.action_points),
        (addr::ACTOR_VALUE_RADIATION, c.radiation),
    ];
    for (code, to) in values
        .into_iter()
        .chain(condition_codes().zip(c.body.iter().copied()))
    {
        changed += u32::from(set_current(e, actor, code, to));
    }
    let by = c
        .karma
        .wrapping_sub(int_value(e, actor, addr::ACTOR_VALUE_KARMA));
    changed += u32::from(change_int(e, actor, addr::ACTOR_VALUE_KARMA, by));
    changed
}

/// Makes `r` wear the items `want`: those it wears and `want` lacks taken off, those it carries
/// and does not wear put on. An item it does not carry is left out. Answers how many it changed.
pub fn apply_worn(e: &mut Engine, r: u32, want: &[u32]) -> u32 {
    let mut changed = 0;
    for item in worn(e, r) {
        if !want.contains(&item) && unequip(e, r, item) {
            changed += 1;
        }
    }
    for item in want {
        changed += u32::from(equip(e, r, *item));
    }
    changed
}

/// The character of this game's player; `None` with no player, or in a game without the
/// functions ([`available`]). What is worn is read only in a game with the inventory functions.
pub fn read(e: &mut Engine) -> Option<Sheet> {
    if !available(e) {
        return None;
    }
    let player = live::player(e)?;
    let worn = e
        .is_registered(addr::REF_INVENTORY_CHANGES)
        .then(|| worn(e, player));
    Some(Sheet {
        character: Some(read_character(e, player)),
        condition: Some(read_condition(e, player)),
        worn,
    })
}

/// Makes this game's player the character of `sheet`: the character, then its condition (its
/// limits follow from the character), then what it wears. Nothing with no player or without the
/// functions.
pub fn apply(e: &mut Engine, sheet: &Sheet) -> Changed {
    let mut out = Changed::default();
    if !available(e) {
        return out;
    }
    let Some(player) = live::player(e) else {
        return out;
    };
    if let Some(c) = &sheet.character {
        out.character = apply_character(e, player, c);
    }
    if let Some(c) = &sheet.condition {
        out.condition = apply_condition(e, player, c);
    }
    if let Some(w) = &sheet.worn {
        out.worn = apply_worn(e, player, w);
    }
    out
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
    const HAT: u32 = 0x0002_0421;
    const SPEECH: u32 = addr::ACTOR_VALUE_SKILL_FIRST + 0xb;
    const STRENGTH: u32 = addr::ACTOR_VALUE_SPECIAL_FIRST;
    const LEFT_LEG: u32 = addr::ACTOR_VALUE_CONDITION_FIRST + 4;

    fn story() -> Story {
        let mut s = Story::new(&StorySpec {
            cells: vec![CELL],
            ..StorySpec::default()
        });
        let item = |id, health| ItemSpec { id, health };
        itemkit::install(
            &mut s,
            &ItemKitSpec {
                items: vec![item(ARMOR, 100), item(RIFLE, 200), item(HAT, 0)],
                ..ItemKitSpec::default()
            },
        );
        kit::install(&mut s, &[TOUGHNESS, EDUCATED]);
        s.move_player(CELL, [0.0; 3], 0.0);
        s
    }

    #[test]
    fn the_codes_are_the_attributes_the_skills_and_the_body_conditions() {
        let all: Vec<u32> = codes().collect();
        assert_eq!(all.len(), SPECIAL + SKILLS);
        assert_eq!(all[..SPECIAL], [5, 6, 7, 8, 9, 0xa, 0xb]);
        assert_eq!(all[SPECIAL], 0x20);
        assert_eq!(all[SPECIAL + SKILLS - 1], 0x2d);
        let body: Vec<u32> = condition_codes().collect();
        assert_eq!(body, [0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f]);
    }

    #[test]
    fn a_game_without_the_functions_has_no_sheet_and_takes_none() {
        let mut s = Story::new(&StorySpec {
            cells: vec![CELL],
            ..StorySpec::default()
        });
        assert!(!available(&s.e));
        assert_eq!(read(&mut s.e), None);
        let sheet = Sheet {
            character: Some(Character::default()),
            condition: Some(Condition::default()),
            worn: Some(vec![ARMOR]),
        };
        assert_eq!(apply(&mut s.e, &sheet), Changed::default());
    }

    #[test]
    fn a_character_is_read_through_the_games_functions() {
        let mut s = story();
        let player = live::player(&mut s.e).unwrap();
        kit::set_base(&mut s, STRENGTH, 7.0);
        kit::set_base(&mut s, SPEECH, 45.0);
        kit::set_level(&mut s, 4, 3200);
        kit::set_tags(&mut s, [SPEECH as i32, 0x29, -1, -1]);
        assert!(set_perk(&mut s.e, player, TOUGHNESS, 2));
        // Not a perk, not in the data: nothing is given.
        assert!(!set_perk(&mut s.e, player, ARMOR, 1));
        assert!(!set_perk(&mut s.e, player, 0x0099_9999, 1));
        let c = read_character(&mut s.e, player);
        assert_eq!(c.special[0], 7.0);
        assert_eq!(c.special[1], 5.0);
        assert_eq!(c.skills[0xb], 45.0);
        assert_eq!(c.tags, [SPEECH as i32, 0x29, -1, -1]);
        assert_eq!((c.level, c.xp), (4, 3200), "level, experience");
        assert_eq!(c.perks, [(TOUGHNESS, 2)]);
    }

    #[test]
    fn a_character_makes_another_games_player_that_character() {
        let mut a = story();
        let pa = live::player(&mut a.e).unwrap();
        kit::set_base(&mut a, STRENGTH, 8.0);
        kit::set_base(&mut a, SPEECH, 60.0);
        kit::set_level(&mut a, 6, 5400);
        kit::set_tags(&mut a, [SPEECH as i32, -1, -1, -1]);
        set_perk(&mut a.e, pa, TOUGHNESS, 1);
        let c = read_character(&mut a.e, pa);

        // Another game: a new character with a perk of its own.
        let mut b = story();
        let pb = live::player(&mut b.e).unwrap();
        set_perk(&mut b.e, pb, EDUCATED, 1);
        let before = kit::counts(&mut b);
        // The level, experience, a perk off, a perk on, a tag skill, two values.
        assert_eq!(apply_character(&mut b.e, pb, &c), 7);
        assert_eq!(read_character(&mut b.e, pb), c);
        // The level through the game's own ForceLevel; the experience quietly.
        let after = kit::counts(&mut b);
        assert_eq!(after.forced, before.forced + 1);
        assert_eq!(after.gains_shown, before.gains_shown);
        // Applied again it changes nothing.
        assert_eq!(apply_character(&mut b.e, pb, &c), 0);
        // A perk at another rank is changed, not added twice; one this game lacks is left out.
        let mut higher = c.clone();
        higher.perks = vec![(TOUGHNESS, 2), (0x0099_9999, 1)];
        assert_eq!(apply_character(&mut b.e, pb, &higher), 1);
        assert_eq!(perks(&mut b.e, pb), [(TOUGHNESS, 2)]);
    }

    #[test]
    fn the_base_values_are_set_after_the_perks() {
        // In game a the perk raised Strength by one when it was given.
        let mut a = story();
        let pa = live::player(&mut a.e).unwrap();
        kit::perk_raises(&mut a, EDUCATED, STRENGTH, 1.0);
        set_perk(&mut a.e, pa, EDUCATED, 1);
        let c = read_character(&mut a.e, pa);
        assert_eq!(c.special[0], 6.0);
        // Game b's perk does the same when it is given back: Strength ends as saved, not raised
        // twice.
        let mut b = story();
        let pb = live::player(&mut b.e).unwrap();
        kit::perk_raises(&mut b, EDUCATED, STRENGTH, 1.0);
        apply_character(&mut b.e, pb, &c);
        assert_eq!(base_value(&mut b.e, pb, STRENGTH), 6.0);
        assert_eq!(read_character(&mut b.e, pb), c);
    }

    #[test]
    fn the_condition_is_read_and_set_through_the_change_of_a_value() {
        let mut a = story();
        let pa = live::player(&mut a.e).unwrap();
        kit::set_current(&mut a, addr::ACTOR_VALUE_HEALTH, 62.5);
        kit::set_current(&mut a, addr::ACTOR_VALUE_RADIATION, 140.0);
        kit::set_current(&mut a, LEFT_LEG, 0.0);
        kit::set_karma(&mut a, -250);
        let c = read_condition(&mut a.e, pa);
        assert_eq!((c.health, c.radiation, c.karma), (62.5, 140.0, -250));
        assert_eq!(c.body[4], 0.0);
        assert_eq!(c.body[0], 100.0);

        let mut b = story();
        let pb = live::player(&mut b.e).unwrap();
        // Health, radiation, the leg, karma.
        assert_eq!(apply_condition(&mut b.e, pb, &c), 4);
        assert_eq!(read_condition(&mut b.e, pb), c);
        assert_eq!(apply_condition(&mut b.e, pb, &c), 0);
        // Each through the actor's change of a value, by the difference.
        assert_eq!(
            kit::changes(&mut b),
            [
                (addr::ACTOR_VALUE_HEALTH, -37.5),
                (addr::ACTOR_VALUE_RADIATION, 140.0),
                (LEFT_LEG, -100.0),
            ]
        );
    }

    #[test]
    fn what_is_worn_is_put_on_and_what_is_not_taken_off() {
        let mut s = story();
        let player = live::player(&mut s.e).unwrap();
        items::add(&mut s.e, player, ARMOR, 1, None);
        items::add(&mut s.e, player, RIFLE, 1, Some(0.5));
        items::add(&mut s.e, player, HAT, 1, None);
        assert!(equip(&mut s.e, player, ARMOR));
        // Worn already, not carried, not an item.
        assert!(!equip(&mut s.e, player, ARMOR));
        assert!(!equip(&mut s.e, player, 0x0001_5169));
        assert!(!equip(&mut s.e, player, TOUGHNESS));
        assert!(equip(&mut s.e, player, HAT));
        assert_eq!(worn(&mut s.e, player), [ARMOR, HAT]);
        // Wearing an item does not change what the player is listed to carry.
        assert_eq!(items::count(&mut s.e, player, ARMOR), 1);
        assert_eq!(items::list(&mut s.e, player).len(), 3);
        // The rifle on, the hat off, the armor kept; a stimpak not carried is left out.
        assert_eq!(
            apply_worn(&mut s.e, player, &[ARMOR, RIFLE, 0x0001_5169]),
            2
        );
        assert_eq!(worn(&mut s.e, player), [RIFLE, ARMOR]);
        assert!(!unequip(&mut s.e, player, HAT));
        assert_eq!(apply_worn(&mut s.e, player, &[ARMOR, RIFLE]), 0);
    }

    #[test]
    fn a_sheet_is_read_and_applied_whole() {
        let mut a = story();
        let pa = live::player(&mut a.e).unwrap();
        kit::set_base(&mut a, STRENGTH, 9.0);
        kit::set_level(&mut a, 3, 2100);
        kit::set_current(&mut a, addr::ACTOR_VALUE_ACTION_POINTS, 40.0);
        items::add(&mut a.e, pa, ARMOR, 1, None);
        equip(&mut a.e, pa, ARMOR);
        let sheet = read(&mut a.e).unwrap();
        assert_eq!(sheet.worn, Some(vec![ARMOR]));

        let mut b = story();
        let pb = live::player(&mut b.e).unwrap();
        items::add(&mut b.e, pb, ARMOR, 1, None);
        let changed = apply(&mut b.e, &sheet);
        assert_eq!(
            (changed.character, changed.condition, changed.worn),
            (3, 1, 1)
        );
        assert_eq!(read(&mut b.e), Some(sheet.clone()));
        assert_eq!(apply(&mut b.e, &sheet).total(), 0);
        // A sheet of which a part is missing leaves that part alone.
        let only_worn = Sheet {
            worn: Some(vec![]),
            ..Sheet::default()
        };
        assert_eq!(apply(&mut b.e, &only_worn).total(), 1);
        assert_eq!(base_value(&mut b.e, pb, STRENGTH), 9.0);
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
        assert!(set_xp(&mut s.e, player, 100));
        assert_eq!(kit::counts(&mut s).gains_shown, 1);
        assert_eq!(
            int_value(&mut s.e, player, addr::ACTOR_VALUE_EXPERIENCE),
            100
        );
        assert!(
            !set_quiet(&mut s.e, player, false),
            "the byte is cleared again"
        );
        assert!(!change_int(&mut s.e, player, addr::ACTOR_VALUE_KARMA, 0));
    }
}

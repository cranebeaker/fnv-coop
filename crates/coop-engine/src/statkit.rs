//! Stand-ins for the game's character functions, for tests of [`crate::stats`] and of the
//! session's party parts (docs/coop/SPECS.md S32): the story's player gets base and current
//! values, a level, experience, karma, a class with tag skills and a perk list laid out in engine
//! memory where co-op reads them, and the functions co-op calls on them. They do only what co-op
//! relies on; nothing here says how the real functions behave beyond that.
//!
//! The kit owns the player's value owner (`+0xa4`) and its slots, so it is not for a story that
//! also has the fight kit's values on its player. What is worn needs the item kit
//! ([`crate::itemkit`]) in the same story.

use std::collections::BTreeMap;

use engine_core::prelude::*;

use crate::testkit::{self, Story};
use crate::{addr, live};

/// The kit's memory: perk forms, the class, list nodes and the level-up state, handed out
/// upwards.
const SK: u32 = 0x0600_0000;
const SK_LEN: u32 = 0x0010_0000;
/// The level-up state, the value owner's vtable, the class.
const STATE: u32 = SK;
const OWNER_VT: u32 = SK + 0x100;
const CLASS: u32 = SK + 0x200;
/// The vtable of the kit's perk forms: a perk is no item.
const PERK_VT: u32 = SK + 0x400;
const FIRST: u32 = SK + 0x1000;
/// The kit's functions behind virtual slots.
const FNS: u32 = 0x0133_0000;
const BASE_VALUE_FN: u32 = FNS;
const CURRENT_VALUE_FN: u32 = FNS + 0x10;
const MODIFY_FN: u32 = FNS + 0x20;
const MODIFY_INT_FN: u32 = FNS + 0x30;
const OWNER_INT_FN: u32 = FNS + 0x40;
const FALSE_FN: u32 = FNS + 0x50;
/// The player's `CharacterProgression` (`this + 0x878`, as the game's `00464e10` answers).
const PROGRESSION_OFFSET: u32 = 0x878;
/// The experience the kit says a level needs.
const PER_LEVEL: i32 = 1000;

/// What the stand-ins did, for a test to count.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Counts {
    /// Rises of experience the game showed and judged for a level-up (not quiet).
    pub gains_shown: u32,
    /// Calls of `ForceLevel` that changed the level.
    pub forced: u32,
}

#[derive(Debug, Default)]
struct Kit {
    next: u32,
    base: BTreeMap<u32, f32>,
    current: BTreeMap<u32, f32>,
    level: u16,
    xp: i32,
    karma: i32,
    /// (perk form, rank).
    perks: Vec<(u32, u8)>,
    /// What giving a perk (form) does to a base value: (value, amount).
    raises: BTreeMap<u32, (u32, f32)>,
    /// Each change through the actor's `+0x3ac`: (value, amount).
    changes: Vec<(u32, f32)>,
    counts: Counts,
}

fn eax(v: u32) -> Ret {
    Ret {
        eax: v,
        ..Ret::default()
    }
}

fn st0(v: f32) -> Ret {
    Ret {
        st0: f64::from(v),
        ..Ret::default()
    }
}

fn new(e: &mut Engine, size: u32) -> u32 {
    let k = e.ext::<Kit>();
    let at = k.next;
    k.next += (size + 7) & !7;
    assert!(k.next <= SK + SK_LEN, "the stat kit's memory is used up");
    at
}

fn is_special(code: u32) -> bool {
    (addr::ACTOR_VALUE_SPECIAL_FIRST
        ..addr::ACTOR_VALUE_SPECIAL_FIRST + addr::ACTOR_VALUE_SPECIAL_COUNT)
        .contains(&code)
}

fn base_of(e: &mut Engine, code: u32) -> f32 {
    let d = if is_special(code) { 5.0 } else { 15.0 };
    e.ext::<Kit>().base.get(&code).copied().unwrap_or(d)
}

fn current_of(e: &mut Engine, code: u32) -> f32 {
    let d = if code == addr::ACTOR_VALUE_RADIATION {
        0.0
    } else {
        100.0
    };
    e.ext::<Kit>().current.get(&code).copied().unwrap_or(d)
}

/// Lays the player's perk list out anew: the first node is in the player, as the game's lists.
fn rebuild_perks(e: &mut Engine, player: u32) {
    let perks = e.ext::<Kit>().perks.clone();
    let head = player + addr::PLAYER_PERK_LIST_OFFSET;
    e.mem.set_u32(head, 0);
    e.mem.set_u32(head + 4, 0);
    let mut node = head;
    for (i, (perk, rank)) in perks.iter().enumerate() {
        let data = new(e, 8);
        e.mem.set_u32(data, *perk);
        e.mem.set_u8(data + addr::PERK_RANK_OFFSET, *rank);
        if i > 0 {
            let next = new(e, 8);
            e.mem.set_u32(node + 4, next);
            node = next;
        }
        e.mem.set_u32(node, data);
        e.mem.set_u32(node + 4, 0);
    }
}

/// The player's `+0x3a8`: `(player, value, amount, source)`.
fn modify_int(e: &mut Engine, a: &[u32]) -> Ret {
    let (code, by) = (a[1], a[2] as i32);
    match code {
        addr::ACTOR_VALUE_EXPERIENCE => {
            let quiet = e.mem.u8(STATE + addr::LEVEL_STATE_QUIET_OFFSET) != 0;
            let k = e.ext::<Kit>();
            k.xp += by;
            if by > 0 && !quiet {
                k.counts.gains_shown += 1;
            }
        }
        addr::ACTOR_VALUE_KARMA => e.ext::<Kit>().karma += by,
        _ => {
            let v = current_of(e, code) + by as f32;
            e.ext::<Kit>().current.insert(code, v);
        }
    }
    Ret::default()
}

/// `CharacterProgression::ForceLevel(progression, level)`: nothing for the player's own level;
/// otherwise the level is set and the experience moved to what the level before it needs.
fn force_level(e: &mut Engine, a: &[u32]) -> Ret {
    let level = a[1] as u16;
    let k = e.ext::<Kit>();
    if level != k.level {
        k.level = level;
        k.xp = PER_LEVEL * (i32::from(level) - 1);
        k.counts.forced += 1;
    }
    Ret::default()
}

/// Gives `story`'s player a character and the stand-ins for the game's character functions,
/// and adds a perk form for each id of `perks`. The story must have a player (cells).
pub fn install(story: &mut Story, perks: &[u32]) {
    let e = &mut story.e;
    let player = live::player(e).expect("a story with a player");
    e.map(SK, SK_LEN);
    {
        let k = e.ext::<Kit>();
        k.next = FIRST;
        k.level = 1;
    }
    // The player's own slots, in its own vtable, and its value owner's.
    let vt = e.mem.u32(player);
    e.mem.set_u32(vt + addr::ACTOR_MODIFY_VALUE_SLOT, MODIFY_FN);
    e.mem
        .set_u32(vt + addr::ACTOR_MODIFY_INT_VALUE_SLOT, MODIFY_INT_FN);
    e.mem
        .set_u32(player + addr::ACTOR_VALUE_OWNER_OFFSET, OWNER_VT);
    e.mem
        .set_u32(OWNER_VT + addr::OWNER_BASE_VALUE_SLOT, BASE_VALUE_FN);
    e.mem
        .set_u32(OWNER_VT + addr::OWNER_INT_VALUE_SLOT, OWNER_INT_FN);
    e.mem
        .set_u32(OWNER_VT + addr::OWNER_CURRENT_VALUE_SLOT, CURRENT_VALUE_FN);
    e.mem.set_u32(player + PROGRESSION_OFFSET, STATE);
    e.mem.set_u32(STATE + 4, PER_LEVEL as u32);
    for i in 0..addr::TAG_SKILL_COUNT {
        e.mem
            .set_i32(CLASS + addr::CLASS_TAG_SKILLS_OFFSET + 4 * i, -1);
    }

    e.register(BASE_VALUE_FN, |e, a| st0(base_of(e, a[1])));
    e.register(CURRENT_VALUE_FN, |e, a| st0(current_of(e, a[1])));
    e.register(OWNER_INT_FN, |e, a| {
        eax(match a[1] {
            addr::ACTOR_VALUE_EXPERIENCE => e.ext::<Kit>().xp as u32,
            addr::ACTOR_VALUE_KARMA => e.ext::<Kit>().karma as u32,
            code => current_of(e, code) as i32 as u32,
        })
    });
    e.register(addr::ACTOR_MOD_BASE_VALUE, |e, a| {
        let v = base_of(e, a[1]) + f32::from_bits(a[2]);
        e.ext::<Kit>().base.insert(a[1], v);
        Ret::default()
    });
    e.register(MODIFY_FN, |e, a| {
        let (code, by) = (a[1], f32::from_bits(a[2]));
        let v = current_of(e, code) + by;
        let k = e.ext::<Kit>();
        k.current.insert(code, v);
        k.changes.push((code, by));
        Ret::default()
    });
    e.register(MODIFY_INT_FN, modify_int);
    e.register(addr::ACTOR_GET_LEVEL, |e, _| {
        eax(u32::from(e.ext::<Kit>().level))
    });
    e.register(addr::PLAYER_PROGRESSION, |_, a| {
        eax(a[0] + PROGRESSION_OFFSET)
    });
    e.register(addr::PROGRESSION_FORCE_LEVEL, force_level);
    e.register(addr::ACTOR_GET_CLASS, |_, _| eax(CLASS));
    e.register(addr::CLASS_SET_TAG_SKILL, |e, a| {
        let at = a[0] + addr::CLASS_TAG_SKILLS_OFFSET + 4 * a[1];
        e.mem.set_u32(at, a[2]);
        Ret::default()
    });
    e.register(addr::PLAYER_SET_PERK_RANK, |e, a| {
        let (player, perk, rank) = (a[0], a[1], a[2] as u8);
        let k = e.ext::<Kit>();
        let raise = match k.perks.iter_mut().find(|p| p.0 == perk) {
            Some(p) => {
                p.1 = rank;
                None
            }
            None => {
                k.perks.push((perk, rank));
                k.raises.get(&perk).copied()
            }
        };
        if let Some((code, by)) = raise {
            let v = base_of(e, code) + by;
            e.ext::<Kit>().base.insert(code, v);
        }
        rebuild_perks(e, player);
        Ret::default()
    });
    e.register(addr::PLAYER_REMOVE_PERK, |e, a| {
        let (player, perk) = (a[0], a[1]);
        e.ext::<Kit>().perks.retain(|p| p.0 != perk);
        rebuild_perks(e, player);
        Ret::default()
    });
    // With the item kit in the story, a worn item is one of its holder's.
    e.register(addr::ACTOR_QUEUE_EQUIP, |e, a| {
        crate::itemkit::wear(e, a[0], a[1]);
        Ret::default()
    });
    e.register(addr::ACTOR_QUEUE_UNEQUIP, |e, a| {
        crate::itemkit::unwear(e, a[0], a[1]);
        Ret::default()
    });

    e.register(FALSE_FN, |_, _| eax(0));
    e.mem.set_u32(PERK_VT + addr::FORM_IS_ITEM_SLOT, FALSE_FN);
    for id in perks {
        let f = new(e, 0x40);
        e.mem.set_u32(f, PERK_VT);
        e.mem
            .set_u8(f + addr::FORM_TYPE_OFFSET, addr::FORM_TYPE_PERK);
        e.mem.set_u32(f + addr::FORM_ID_OFFSET, *id);
        testkit::add_form(e, *id, f);
    }
}

/// Makes `v` the player's base value of actor value `code`, as play does.
pub fn set_base(s: &mut Story, code: u32, v: f32) {
    s.e.ext::<Kit>().base.insert(code, v);
}

/// Makes `v` the player's current value of actor value `code`, as play does.
pub fn set_current(s: &mut Story, code: u32, v: f32) {
    s.e.ext::<Kit>().current.insert(code, v);
}

/// Makes `v` the player's karma, as play does.
pub fn set_karma(s: &mut Story, v: i32) {
    s.e.ext::<Kit>().karma = v;
}

/// Makes the player level `level` with `xp` experience, as play does.
pub fn set_level(s: &mut Story, level: u16, xp: i32) {
    let k = s.e.ext::<Kit>();
    k.level = level;
    k.xp = xp;
}

/// Makes `tags` the player's tag skills, as character creation does.
pub fn set_tags(s: &mut Story, tags: [i32; 4]) {
    for (i, t) in tags.iter().enumerate() {
        s.e.mem
            .set_i32(CLASS + addr::CLASS_TAG_SKILLS_OFFSET + 4 * i as u32, *t);
    }
}

/// From now on, giving perk `perk` (a form id of [`install`]'s) adds `by` to the player's base
/// value of actor value `code`.
pub fn perk_raises(s: &mut Story, perk: u32, code: u32, by: f32) {
    let form = crate::forms::form(&mut s.e, perk).expect("a perk of the kit");
    s.e.ext::<Kit>().raises.insert(form, (code, by));
}

/// Each change through the actor's change of a value so far: (value, amount).
pub fn changes(s: &mut Story) -> Vec<(u32, f32)> {
    s.e.ext::<Kit>().changes.clone()
}

/// What the stand-ins did so far.
pub fn counts(s: &mut Story) -> Counts {
    s.e.ext::<Kit>().counts
}

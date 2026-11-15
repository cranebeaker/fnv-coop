//! Stand-ins for the game's character functions, for tests of [`crate::stats`] and of the
//! session's party parts and conversation rewards (docs/coop/SPECS.md S30, S31): the story's
//! player gets its own values, a level, experience, karma and a perk list laid out in engine
//! memory where co-op reads them, and the functions co-op calls on them. They do only what
//! co-op relies on; nothing here says how the real functions behave beyond that.
//!
//! A story with this kit can also give a line of dialogue a gift ([`set_gift`]): what its
//! result script gives the player of the game that runs it.

use std::collections::BTreeMap;

use engine_core::prelude::*;

use crate::testkit::{self, Story};
use crate::{addr, forms, items, live, social_kit};

/// The kit's memory: perk forms, list nodes and the level-up state, handed out upwards.
const SK: u32 = 0x0600_0000;
const SK_LEN: u32 = 0x0010_0000;
/// The level-up state and the value owner's vtable.
const STATE: u32 = SK;
const OWNER_VT: u32 = SK + 0x100;
/// The vtable of the kit's perk forms: a perk is no item.
const PERK_VT: u32 = SK + 0x400;
const FIRST: u32 = SK + 0x1000;
/// The kit's functions behind virtual slots.
const FNS: u32 = 0x0133_0000;
const OWN_VALUE_FN: u32 = FNS;
const SET_OWN_VALUE_FN: u32 = FNS + 0x10;
const MODIFY_INT_FN: u32 = FNS + 0x20;
const OWNER_INT_FN: u32 = FNS + 0x30;
const FALSE_FN: u32 = FNS + 0x40;
/// The player's `CharacterProgression` (`this + 0x878`, as the game's `00464e10` answers).
const PROGRESSION_OFFSET: u32 = 0x878;
/// The experience the kit says a level needs.
const PER_LEVEL: i32 = 1000;

/// What a line's result script gives the player of the game that runs it.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Gift {
    /// Items: (form id, count); a negative count is taken from the player (a payment). Caps
    /// are the item 0xf.
    pub items: Vec<(u32, i32)>,
    /// Experience, given as the game's reward does.
    pub xp: i32,
    /// Karma, through the game's `RewardKarma`.
    pub karma: i32,
    /// Changes of the player's own values: (actor value, the amount added).
    pub values: Vec<(u32, f32)>,
}

/// What the stand-ins did, for a test to count.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Counts {
    /// Rises of experience the game showed and judged for a level-up (not quiet).
    pub gains_shown: u32,
    /// Calls of `RewardKarma` (each shows the game's karma message).
    pub karma_messages: u32,
    /// Calls of `ForceLevel` that changed the level.
    pub forced: u32,
    /// Gifts given by a result script that ran here.
    pub gifts: u32,
}

#[derive(Debug, Default)]
struct Kit {
    next: u32,
    values: BTreeMap<u32, f32>,
    level: u16,
    xp: i32,
    karma: i32,
    /// (perk form, rank).
    perks: Vec<(u32, u8)>,
    gifts: BTreeMap<(u32, u32), Gift>,
    counts: Counts,
}

fn eax(v: u32) -> Ret {
    Ret {
        eax: v,
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

fn default_value(code: u32) -> f32 {
    let special = addr::ACTOR_VALUE_SPECIAL_FIRST
        ..addr::ACTOR_VALUE_SPECIAL_FIRST + addr::ACTOR_VALUE_SPECIAL_COUNT;
    if special.contains(&code) {
        5.0
    } else {
        15.0
    }
}

fn kit_value(e: &mut Engine, code: u32) -> f32 {
    e.ext::<Kit>()
        .values
        .get(&code)
        .copied()
        .unwrap_or_else(|| default_value(code))
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
            let v = kit_value(e, code) + by as f32;
            e.ext::<Kit>().values.insert(code, v);
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

/// The hook of the social kit's `RunResult`: `(info, index)`, when the result script ran.
fn result_ran(e: &mut Engine, a: &[u32]) -> Ret {
    let id = e.mem.u32(a[0] + addr::FORM_ID_OFFSET);
    let Some(gift) = e.ext::<Kit>().gifts.get(&(id, a[1])).cloned() else {
        return Ret::default();
    };
    let Some(player) = live::player(e) else {
        return Ret::default();
    };
    for (item, count) in &gift.items {
        if *count > 0 {
            items::add(e, player, *item, *count, None);
        } else {
            items::remove(e, player, *item, -*count);
        }
    }
    if gift.xp != 0 {
        e.vcall(
            player,
            addr::ACTOR_MODIFY_INT_VALUE_SLOT,
            &args![addr::ACTOR_VALUE_EXPERIENCE, gift.xp as u32, 0u32],
        );
    }
    if gift.karma != 0 {
        e.call(
            addr::PLAYER_REWARD_KARMA,
            &args![player, gift.karma as u32],
        );
    }
    for (code, by) in &gift.values {
        let v = kit_value(e, *code) + by;
        e.ext::<Kit>().values.insert(*code, v);
    }
    e.ext::<Kit>().counts.gifts += 1;
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
    e.mem
        .set_u32(vt + addr::PLAYER_OWN_VALUE_SLOT, OWN_VALUE_FN);
    e.mem
        .set_u32(vt + addr::PLAYER_SET_OWN_VALUE_SLOT, SET_OWN_VALUE_FN);
    e.mem
        .set_u32(vt + addr::ACTOR_MODIFY_INT_VALUE_SLOT, MODIFY_INT_FN);
    let owner = player + addr::ACTOR_VALUE_OWNER_OFFSET;
    if e.mem.u32(owner) == 0 {
        e.mem.set_u32(owner, OWNER_VT);
    }
    let owner_vt = e.mem.u32(owner);
    e.mem
        .set_u32(owner_vt + addr::OWNER_INT_VALUE_SLOT, OWNER_INT_FN);
    e.mem.set_u32(player + PROGRESSION_OFFSET, STATE);
    e.mem.set_u32(STATE + 4, PER_LEVEL as u32);

    e.register(OWN_VALUE_FN, |e, a| {
        e.mem.set_u8(a[2], 1);
        Ret {
            st0: f64::from(kit_value(e, a[1])),
            ..Ret::default()
        }
    });
    e.register(SET_OWN_VALUE_FN, |e, a| {
        e.ext::<Kit>().values.insert(a[1], f32::from_bits(a[2]));
        Ret::default()
    });
    e.register(MODIFY_INT_FN, modify_int);
    e.register(OWNER_INT_FN, |e, a| {
        eax(match a[1] {
            addr::ACTOR_VALUE_EXPERIENCE => e.ext::<Kit>().xp as u32,
            addr::ACTOR_VALUE_KARMA => e.ext::<Kit>().karma as u32,
            code => kit_value(e, code) as i32 as u32,
        })
    });
    e.register(addr::ACTOR_GET_LEVEL, |e, _| {
        eax(u32::from(e.ext::<Kit>().level))
    });
    e.register(addr::PLAYER_PROGRESSION, |_, a| eax(a[0] + PROGRESSION_OFFSET));
    e.register(addr::PROGRESSION_FORCE_LEVEL, force_level);
    e.register(addr::PLAYER_REWARD_KARMA, |e, a| {
        let k = e.ext::<Kit>();
        k.karma += a[1] as i32;
        k.counts.karma_messages += 1;
        Ret::default()
    });
    e.register(addr::PLAYER_SET_PERK_RANK, |e, a| {
        let (player, perk, rank) = (a[0], a[1], a[2] as u8);
        let k = e.ext::<Kit>();
        match k.perks.iter_mut().find(|p| p.0 == perk) {
            Some(p) => p.1 = rank,
            None => k.perks.push((perk, rank)),
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
    e.register(social_kit::RESULT_RAN_HOOK, result_ran);

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

/// Stores `v` as the player's own value of actor value `code`, as play does.
pub fn set(s: &mut Story, code: u32, v: f32) {
    s.e.ext::<Kit>().values.insert(code, v);
}

/// Makes the player level `level` with `xp` experience, as play does.
pub fn set_level(s: &mut Story, level: u16, xp: i32) {
    let k = s.e.ext::<Kit>();
    k.level = level;
    k.xp = xp;
}

/// What the stand-ins did so far.
pub fn counts(s: &mut Story) -> Counts {
    s.e.ext::<Kit>().counts
}

/// Gives info `info`'s result script (said: `end` false; ended: `end` true) a gift for the
/// player of the game that runs it. The info needs a result ([`Story::set_info_result`]).
pub fn set_gift(s: &mut Story, info: u32, end: bool, gift: Gift) {
    assert!(forms::form(&mut s.e, info).is_some(), "an info of the story");
    s.e.ext::<Kit>().gifts.insert((info, u32::from(end)), gift);
}

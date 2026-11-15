//! The testkit's avatars as players (docs/coop/SPECS.md S35): inventories that know what is
//! worn, weapons drawn and put away, animation actions, NPC base forms with a face, and the
//! stand-ins co-op's look code calls. Installed on a story with [`install`]; until then every
//! hook here does nothing, so the other tests' stories are as they were. Each stand-in does what
//! co-op relies on the real one to do, and no more:
//!
//! - an inventory's changes list each item the reference holds (an entry: its count and form),
//!   the count is the sum of what it holds, and `WearingObject` answers whether it wears it;
//! - `QueueEquipObject` puts on an item the actor holds (a weapon takes the place of the one it
//!   held), `QueueUnEquipObject` takes it off; the weapon held is the one worn;
//! - the draw call sets the drawn byte `IsWeaponDrawn` reads, `QueueReload` is counted;
//! - `CreateFormOfType` of an NPC makes a form with a new id; `TESNPC::Copy` copies the base's
//!   bytes and makes the source its face NPC (as the game does for any NPC but the player's);
//! - a face-gen matrix's resize gives it zeroed values; a string's set copies the text.

use std::collections::BTreeMap;

use engine_core::prelude::*;

use super::{eax, Story, PLAYER};
use crate::{addr, forms};

/// The kit's memory.
const LK: u32 = 0x0700_0000;
const LK_LEN: u32 = 0x0020_0000;
/// The vtables of the kit's NPC bases and items, and an empty string.
const NPC_VT: u32 = LK;
const ITEM_VT: u32 = LK + 0x400;
const EMPTY: u32 = LK + 0xff0;
const FIRST: u32 = LK + 0x1000;
/// The stand-ins behind virtual slots.
const FNS: u32 = 0x0134_0000;
const COPY_FN: u32 = FNS;
const TRUE_FN: u32 = FNS + 0x10;
/// The first id of an NPC base the kit's `CreateFormOfType` makes.
pub const FIRST_NPC_ID: u32 = 0xff80_0001;
/// The bytes of a kit NPC base (the game's `TESNPC` is `0x20c`).
const NPC_LEN: u32 = 0x400;
/// The face-gen coordinate inside an NPC (`RaceFaceOffsetCoord`).
const COORD_OFFSET: u32 = 0x134;
/// Where a placed actor's mover is said to be (only its being there is read).
const MOVER_AT: u32 = 0x300;

#[derive(Debug, Default, Clone)]
struct Body {
    drawn: bool,
    draws: u32,
    action: i32,
    reloads: u32,
    /// (item form, count, worn).
    items: Vec<(u32, i32, bool)>,
    inv: u32,
    equips: u32,
    unequips: u32,
}

#[derive(Debug, Default)]
struct Kit {
    on: bool,
    next: u32,
    bodies: BTreeMap<u32, Body>,
    /// A weapon form's ammunition form.
    ammo: BTreeMap<u32, u32>,
    npcs: u32,
    /// The player's attacks started, and whether the game refuses them.
    starts: u32,
    refuse_starts: bool,
}

fn new(e: &mut Engine, size: u32) -> u32 {
    let k = e.ext::<Kit>();
    let at = k.next;
    k.next += (size + 15) & !15;
    assert!(k.next <= LK + LK_LEN, "the look kit's memory is used up");
    at
}

fn on(e: &mut Engine) -> bool {
    e.ext::<Kit>().on
}

fn body(e: &mut Engine, r: u32) -> &mut Body {
    e.ext::<Kit>().bodies.entry(r).or_default()
}

fn is_weapon(e: &mut Engine, form: u32) -> bool {
    form != 0 && e.mem.u8(form + addr::FORM_TYPE_OFFSET) == addr::FORM_TYPE_WEAPON
}

/// Lays reference `r`'s inventory changes out anew: the first word of its changes object points
/// at the list of entries (`{extend, count, form}`).
fn rebuild(e: &mut Engine, r: u32) {
    if body(e, r).inv == 0 {
        let inv = new(e, 0x10);
        body(e, r).inv = inv;
    }
    let (inv, items) = {
        let b = body(e, r);
        (b.inv, b.items.clone())
    };
    let mut first = 0;
    let mut last = 0;
    for (form, count, _) in items.iter().filter(|i| i.1 > 0) {
        let entry = new(e, 0x10);
        e.mem.set_u32(entry + addr::ENTRY_EXTEND_OFFSET, 0);
        e.mem
            .set_u32(entry + addr::ENTRY_COUNT_OFFSET, *count as u32);
        e.mem.set_u32(entry + addr::ENTRY_FORM_OFFSET, *form);
        let node = new(e, 8);
        e.mem.set_u32(node, entry);
        e.mem.set_u32(node + 4, 0);
        if last == 0 {
            first = node;
        } else {
            e.mem.set_u32(last + 4, node);
        }
        last = node;
    }
    e.mem.set_u32(inv, first);
}

fn body_of_inv(e: &mut Engine, inv: u32) -> Option<u32> {
    e.ext::<Kit>()
        .bodies
        .iter()
        .find(|(_, b)| b.inv == inv && inv != 0)
        .map(|(r, _)| *r)
}

fn hold(e: &mut Engine, r: u32, form: u32, count: i32) {
    let b = body(e, r);
    match b.items.iter_mut().find(|i| i.0 == form) {
        Some(i) => i.1 += count,
        None => b.items.push((form, count, false)),
    }
    rebuild(e, r);
}

/// Called by `Story::new`, before co-op is installed: `Actor::StartAttack`, which the player's
/// attack button calls, starts the attack (unless a test refuses it) and is counted.
pub(super) fn register(e: &mut Engine) {
    e.register(addr::ACTOR_START_ATTACK, |e, _| {
        let k = e.ext::<Kit>();
        if k.refuse_starts {
            return eax(0);
        }
        k.starts += 1;
        eax(1)
    });
}

/// Called by the fights' `PlaceAtMe` for each reference it placed: with the kit installed, an
/// actor of base `form` with a mover and no attack queued.
pub(super) fn placed(e: &mut Engine, r: u32, form: u32) {
    if !on(e) {
        return;
    }
    e.mem.set_u32(r + addr::REF_BASE_FORM_OFFSET, form);
    e.mem
        .set_u8(r + addr::FORM_TYPE_OFFSET, addr::FORM_TYPE_ACHR);
    e.mem
        .set_u32(r + addr::ACTOR_QUEUED_ATTACK_OFFSET, addr::NO_QUEUED_ATTACK);
    e.mem.set_u32(r + addr::ACTOR_MOVER_OFFSET, r + MOVER_AT);
}

/// Called by the testkit's hand-over of a container: with the kit installed, what it holds goes
/// into `to`'s inventory.
pub(super) fn give(e: &mut Engine, container: u32, to: u32) {
    if !on(e) || to == 0 {
        return;
    }
    let mut node = container + 4;
    while node != 0 {
        let item = e.mem.u32(node);
        if item != 0 {
            let (count, form) = (e.mem.u32(item) as i32, e.mem.u32(item + 4));
            hold(e, to, form, count);
        }
        node = e.mem.u32(node + 4);
    }
}

fn copy_npc(e: &mut Engine, a: &[u32]) -> Ret {
    let (this, source) = (a[0], a[1]);
    for o in (0x10..0x20c).step_by(4) {
        let w = e.mem.u32(source + o);
        e.mem.set_u32(this + o, w);
    }
    e.mem.set_u32(this + addr::NPC_FACE_NPC_OFFSET, source);
    Ret::default()
}

/// Puts the look kit on `s`: its inventories, weapons, NPC bases and faces. The player's base
/// form is form 7, an NPC.
pub fn install(s: &mut Story) {
    let e = &mut s.e;
    e.map(LK, LK_LEN);
    {
        let k = e.ext::<Kit>();
        k.on = true;
        k.next = FIRST;
    }
    e.mem.set_u32(NPC_VT + addr::FORM_COPY_SLOT, COPY_FN);
    e.mem.set_u32(ITEM_VT + addr::FORM_IS_ITEM_SLOT, TRUE_FN);
    let base = forms::form(e, 7).expect("the story's base form 7");
    e.mem.set_u32(base, NPC_VT);
    e.mem
        .set_u8(base + addr::FORM_TYPE_OFFSET, addr::FORM_TYPE_NPC);
    e.mem.set_u32(PLAYER + addr::REF_BASE_FORM_OFFSET, base);
    e.register(COPY_FN, copy_npc);
    e.register(TRUE_FN, |_, _| eax(1));
    e.register(addr::REF_INVENTORY_CHANGES, |e, a| {
        if body(e, a[0]).inv == 0 {
            rebuild(e, a[0]);
        }
        eax(body(e, a[0]).inv)
    });
    e.register(addr::CHANGES_ITEM_COUNT, |e, a| {
        let n: i32 = match body_of_inv(e, a[0]) {
            Some(r) => body(e, r)
                .items
                .iter()
                .filter(|i| i.0 == a[1])
                .map(|i| i.1)
                .sum(),
            None => 0,
        };
        eax(n as u32)
    });
    e.register(addr::CHANGES_WEARING_OBJECT, |e, a| {
        let worn = body_of_inv(e, a[0]).is_some_and(|r| {
            body(e, r)
                .items
                .iter()
                .any(|i| i.0 == a[1] && i.1 > 0 && i.2)
        });
        eax(if worn { a[0] } else { 0 })
    });
    e.register(addr::FORM_AS_HEALTH_FORM, |_, _| eax(0));
    // (no caps in the kit's inventories)
    e.register(addr::CHANGES_GOLD_AMOUNT, |_, _| eax(0));
    e.register(addr::ACTOR_QUEUE_EQUIP_OBJECT, |e, a| {
        let (actor, form) = (a[0], a[1]);
        let weapon = is_weapon(e, form);
        let held: Vec<u32> = body(e, actor).items.iter().map(|i| i.0).collect();
        let types: Vec<(u32, bool)> = held.into_iter().map(|f| (f, is_weapon(e, f))).collect();
        let b = body(e, actor);
        b.equips += 1;
        if b.items.iter().any(|i| i.0 == form && i.1 > 0) {
            for i in b.items.iter_mut() {
                if i.0 == form {
                    i.2 = true;
                } else if weapon && types.iter().any(|t| t.0 == i.0 && t.1) {
                    i.2 = false;
                }
            }
        }
        Ret::default()
    });
    e.register(addr::ACTOR_QUEUE_UNEQUIP_OBJECT, |e, a| {
        let b = body(e, a[0]);
        b.unequips += 1;
        for i in b.items.iter_mut().filter(|i| i.0 == a[1]) {
            i.2 = false;
        }
        Ret::default()
    });
    e.register(addr::ACTOR_IS_WEAPON_DRAWN, |e, a| {
        eax(u32::from(body(e, a[0]).drawn))
    });
    e.register(addr::ACTOR_DRAW_WEAPON, |e, a| {
        let b = body(e, a[0]);
        b.drawn = a[1] & 0xff != 0;
        b.draws += 1;
        Ret::default()
    });
    e.register(addr::ACTOR_GET_CURRENT_WEAPON, |e, a| {
        let held: Vec<u32> = body(e, a[0])
            .items
            .iter()
            .filter(|i| i.1 > 0 && i.2)
            .map(|i| i.0)
            .collect();
        eax(held.into_iter().find(|f| is_weapon(e, *f)).unwrap_or(0))
    });
    e.register(addr::ACTOR_GET_ANIM_ACTION, |e, a| {
        eax(body(e, a[0]).action as u32)
    });
    e.register(addr::ACTOR_QUEUE_RELOAD, |e, a| {
        body(e, a[0]).reloads += 1;
        Ret::default()
    });
    e.register(addr::WEAPON_GET_CURRENT_AMMO, |e, a| {
        eax(e.ext::<Kit>().ammo.get(&a[0]).copied().unwrap_or(0))
    });
    e.register(addr::CREATE_FORM_OF_TYPE, |e, a| {
        if a[0] as u8 != addr::FORM_TYPE_NPC {
            return eax(0);
        }
        let npc = new(e, NPC_LEN);
        let n = {
            let k = e.ext::<Kit>();
            k.npcs += 1;
            k.npcs - 1
        };
        e.mem.set_u32(npc, NPC_VT);
        e.mem
            .set_u8(npc + addr::FORM_TYPE_OFFSET, addr::FORM_TYPE_NPC);
        e.mem.set_u32(npc + addr::FORM_ID_OFFSET, FIRST_NPC_ID + n);
        super::add_form(e, FIRST_NPC_ID + n, npc);
        eax(npc)
    });
    e.register(addr::LIST_CLEAR, |e, a| {
        e.mem.set_u32(a[0], 0);
        e.mem.set_u32(a[0] + 4, 0);
        Ret::default()
    });
    e.register(addr::LIST_APPEND, |e, a| {
        let item = e.mem.u32(a[1]);
        if item == 0 {
            return Ret::default();
        }
        let mut node = a[0];
        while e.mem.u32(node + 4) != 0 {
            node = e.mem.u32(node + 4);
        }
        if e.mem.u32(node) != 0 {
            let fresh = new(e, 8);
            e.mem.set_u32(fresh + 4, 0);
            e.mem.set_u32(node + 4, fresh);
            node = fresh;
        }
        e.mem.set_u32(node, item);
        Ret::default()
    });
    e.register(addr::NPC_FACE_COORD, |_, a| eax(a[0] + COORD_OFFSET));
    e.register(addr::MATRIX_RESIZE, |e, a| {
        let (m, rows, cols) = (a[0], a[1], a[2]);
        let data = new(e, 4 * (rows * cols).max(1));
        for i in 0..rows * cols {
            e.mem.set_f32(data + 4 * i, 0.0);
        }
        e.mem.set_u32(m + addr::MATRIX_DATA_OFFSET, data);
        e.mem.set_u32(m + addr::MATRIX_ROWS_OFFSET, rows);
        e.mem.set_u32(m + addr::MATRIX_COLUMNS_OFFSET, cols);
        Ret::default()
    });
    e.register(addr::FULL_NAME_TEXT, |e, a| {
        match e.mem.u32(a[0] + addr::NPC_NAME_STRING_OFFSET) {
            0 => eax(EMPTY),
            text => eax(text),
        }
    });
    e.register(addr::STRING_SET, |e, a| {
        let (string, text) = (a[0], a[1]);
        let mut bytes = Vec::new();
        while text != 0 && e.mem.u8(text + bytes.len() as u32) != 0 {
            bytes.push(e.mem.u8(text + bytes.len() as u32));
        }
        let copy = new(e, bytes.len() as u32 + 1);
        for (i, b) in bytes.iter().enumerate() {
            e.mem.set_u8(copy + i as u32, *b);
        }
        e.mem.set_u8(copy + bytes.len() as u32, 0);
        e.mem.set_u32(string, copy);
        e.mem.set_u16(string + 4, bytes.len() as u16);
        eax(u32::from(!bytes.is_empty()))
    });
}

/// What an avatar (or the player) has as the look kit sees it.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Gear {
    /// Its weapon is drawn.
    pub drawn: bool,
    /// Times the game was asked to draw or put away its weapon.
    pub draws: u32,
    /// Times it was asked to reload.
    pub reloads: u32,
    /// Times it was asked to put something on, and to take something off.
    pub equips: u32,
    /// See [`Gear::equips`].
    pub unequips: u32,
}

impl Story {
    fn look_ptr(&mut self, id: u32) -> u32 {
        forms::form(&mut self.e, id).expect("a reference of the story")
    }

    /// Makes an item form of `kind` (a form type: `0x18` armour, `0x28` weapon, `0x29` ammunition)
    /// with id `id`.
    pub fn add_look_item(&mut self, id: u32, kind: u8) {
        let f = new(&mut self.e, 0x40);
        self.e.mem.set_u32(f, ITEM_VT);
        self.e.mem.set_u8(f + addr::FORM_TYPE_OFFSET, kind);
        self.e.mem.set_u32(f + addr::FORM_ID_OFFSET, id);
        super::add_form(&mut self.e, id, f);
    }

    /// Makes a plain form with id `id` (a race, hair, eyes or head part).
    pub fn add_look_form(&mut self, id: u32, kind: u8) {
        let f = new(&mut self.e, 0x40);
        self.e.mem.set_u8(f + addr::FORM_TYPE_OFFSET, kind);
        self.e.mem.set_u32(f + addr::FORM_ID_OFFSET, id);
        super::add_form(&mut self.e, id, f);
    }

    /// Weapon `weapon` fires ammunition `ammo` (form ids).
    pub fn set_look_ammo(&mut self, weapon: u32, ammo: u32) {
        let (w, a) = (self.look_ptr(weapon), self.look_ptr(ammo));
        self.e.ext::<Kit>().ammo.insert(w, a);
    }

    /// Gives reference `id` `count` of item `item`, worn or not.
    pub fn look_give(&mut self, id: u32, item: u32, count: i32, worn: bool) {
        let (r, f) = (self.look_ptr(id), self.look_ptr(item));
        hold(&mut self.e, r, f, count);
        if worn {
            for i in body(&mut self.e, r).items.iter_mut().filter(|i| i.0 == f) {
                i.2 = true;
            }
        }
    }

    /// Has reference `id` stop wearing item `item`.
    pub fn look_take_off(&mut self, id: u32, item: u32) {
        let (r, f) = (self.look_ptr(id), self.look_ptr(item));
        for i in body(&mut self.e, r).items.iter_mut().filter(|i| i.0 == f) {
            i.2 = false;
        }
    }

    /// The item form ids reference `id` wears, in the order it got them.
    pub fn look_wearing(&mut self, id: u32) -> Vec<u32> {
        let r = self.look_ptr(id);
        let forms: Vec<u32> = body(&mut self.e, r)
            .items
            .iter()
            .filter(|i| i.1 > 0 && i.2)
            .map(|i| i.0)
            .collect();
        forms
            .into_iter()
            .map(|f| self.e.mem.u32(f + addr::FORM_ID_OFFSET))
            .collect()
    }

    /// How many of item `item` reference `id` holds.
    pub fn look_count(&mut self, id: u32, item: u32) -> i32 {
        let (r, f) = (self.look_ptr(id), self.look_ptr(item));
        body(&mut self.e, r)
            .items
            .iter()
            .filter(|i| i.0 == f)
            .map(|i| i.1)
            .sum()
    }

    /// Sets reference `id`'s weapon drawn or put away, as its own game does.
    pub fn set_look_drawn(&mut self, id: u32, drawn: bool) {
        let r = self.look_ptr(id);
        body(&mut self.e, r).drawn = drawn;
    }

    /// Sets reference `id`'s animation action.
    pub fn set_look_action(&mut self, id: u32, action: i32) {
        let r = self.look_ptr(id);
        body(&mut self.e, r).action = action;
    }

    /// Reference `id`'s weapon, draws, reloads and equips as the kit has them.
    pub fn look_gear(&mut self, id: u32) -> Gear {
        let r = self.look_ptr(id);
        let b = body(&mut self.e, r).clone();
        Gear {
            drawn: b.drawn,
            draws: b.draws,
            reloads: b.reloads,
            equips: b.equips,
            unequips: b.unequips,
        }
    }

    /// The player presses its attack button and the game starts an attack of animation group
    /// `group` (`Actor::StartAttack`, as the player's attack code calls it). Whether it started.
    pub fn player_attack(&mut self, group: u32) -> bool {
        self.e
            .call(addr::ACTOR_START_ATTACK, &args![PLAYER, group])
            .bool()
    }

    /// Has the game start or refuse the player's attacks from now on.
    pub fn refuse_player_attacks(&mut self, refuse: bool) {
        self.e.ext::<Kit>().refuse_starts = refuse;
    }

    /// The attacks the game started (any actor's, through `Actor::StartAttack`).
    pub fn attacks_started(&mut self) -> u32 {
        self.e.ext::<Kit>().starts
    }

    /// The NPC bases the game made while playing (`CreateFormOfType`).
    pub fn npcs_made(&mut self) -> u32 {
        self.e.ext::<Kit>().npcs
    }

    /// The face NPC of NPC base `id` (its form id, 0 for none).
    pub fn face_npc(&mut self, id: u32) -> u32 {
        let npc = self.look_ptr(id);
        match self.e.mem.u32(npc + addr::NPC_FACE_NPC_OFFSET) {
            0 => 0,
            f => self.e.mem.u32(f + addr::FORM_ID_OFFSET),
        }
    }
}

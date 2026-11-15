//! The testkit's bodies for [`crate::anim`] and the head rebuild of [`crate::look`]
//! (docs/coop/SPECS.md S39): an animation with eight sections, an idle, a character controller,
//! a sit and sleep state, a knock state, furniture, iron sights, walking and running speeds and
//! two head nodes for the player and every placed actor, and the stand-ins co-op calls on them.
//! Installed on a story with [`install`]; until then every hook here does nothing. Each stand-in
//! does what co-op relies on the real one to do, and no more; the kit's own choices (named as
//! such) say nothing about the game:
//!
//! - `PlayGroup` puts the group in the section of its type (the kit's table, section 3 unless a
//!   test says otherwise) and counts it; `GetAnimGroup` adds the kit's weapon bits `0x0300`;
//! - `IsAttackAction` and `IsAimAction` answer for the kit's types `0x20..0x30` and `0x30..0x40`;
//! - the process plays the idle it is given when told to with `0x80`;
//! - the jump puts the controller in state 1; the iron sights and speeds are stored;
//! - fatigue below zero knocks an actor down (knock state 4), as the game's knock state does;
//! - the sit package puts the actor on the furniture (sit state 4), the get-up package off it;
//! - building a head gives the actor two head nodes, each with a parent that takes it off again.

use std::collections::BTreeMap;

use engine_core::prelude::*;

use super::{eax, Story, PLAYER};
use crate::{addr, forms};

const AK: u32 = 0x0800_0000;
const AK_LEN: u32 = 0x0010_0000;
/// The vtables of placed actors' processes, of nodes and of head parents.
const PROC_VT: u32 = AK;
const NODE_VT: u32 = AK + 0x800;
const PARENT_VT: u32 = AK + 0x900;
/// The player's first-person animation, which co-op is not to read (its groups say `0x77`).
const FIRST_PERSON: u32 = AK + 0xa00;
const FIRST: u32 = AK + 0x1000;
/// The stand-ins behind virtual slots.
const FNS: u32 = 0x0135_0000;
const ANIM_FN: u32 = FNS;
const SET_IDLE_FN: u32 = FNS + 0x10;
const FLAGS_FN: u32 = FNS + 0x20;
const SIT_FN: u32 = FNS + 0x30;
const KNOCK_FN: u32 = FNS + 0x40;
const FURNITURE_FN: u32 = FNS + 0x50;
const PUT_PACKAGE_FN: u32 = FNS + 0x60;
const HEAD_FN: u32 = FNS + 0x70;
const BIPED_HEAD_FN: u32 = FNS + 0x80;
const ROOT_FN: u32 = FNS + 0x90;
const DETACH_FN: u32 = FNS + 0xa0;
/// Where a kit actor's controller is said to be (only its being there is read).
const CONTROLLER_AT: u32 = 0x3a0;
/// Bytes of a kit animation, idle, package and node.
const ANIM_LEN: u32 = 0x140;
/// A section's group in a kit animation (`Animation::group`, `+0x4c`, as the game).
const GROUP_AT: u32 = 0x4c;
/// The kit's walking and running speeds of an actor no test set (kit's own numbers).
pub const WALK: f32 = 80.0;
/// See [`WALK`].
pub const RUN: f32 = 320.0;
/// The kit's full fatigue (kit's own number).
pub const FATIGUE: f32 = 100.0;
/// The kit's weapon bits of an actor's group (kit's own number).
pub const WEAPON_BITS: u16 = 0x0300;
/// The knock state the kit gives an actor whose fatigue is below zero.
pub const KNOCKED: i32 = 4;
/// The sit state the kit gives an actor on furniture.
pub const SITTING: i32 = 4;

/// What a kit actor's body has, for a test to read.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Shape {
    /// The groups started through `PlayGroup`, in order.
    pub plays: Vec<u16>,
    /// The animation actions set.
    pub actions: Vec<u32>,
    /// The idles played (form ids), in order.
    pub idles: Vec<u32>,
    /// Jumps started.
    pub jumps: u32,
    /// The iron sights, and how often they were set.
    pub sights: bool,
    /// See [`Shape::sights`].
    pub sights_set: u32,
    /// The walking and running speeds, and how often the process was given them.
    pub walk: f32,
    /// See [`Shape::walk`].
    pub run: f32,
    /// See [`Shape::walk`].
    pub speeds_set: u32,
    /// The fatigue and the knock state.
    pub fatigue: f32,
    /// See [`Shape::fatigue`].
    pub knock: i32,
    /// The sit and sleep packages given (furniture form ids), and the get-up packages.
    pub sits: Vec<u32>,
    /// See [`Shape::sits`].
    pub get_ups: u32,
    /// The furniture form id it uses, 0 for none.
    pub furniture: u32,
    /// Heads built, taken off, cleared, and the face parameters applied to a head.
    pub heads_built: u32,
    /// See [`Shape::heads_built`].
    pub heads_taken_off: u32,
    /// See [`Shape::heads_built`].
    pub cleared: u32,
    /// See [`Shape::heads_built`].
    pub faces_applied: u32,
}

#[derive(Debug, Default)]
struct Body {
    anim: u32,
    controller: u32,
    sit: i32,
    knock: i32,
    furniture: u32,
    idle_asked: u32,
    heads: [u32; 2],
    shape: Shape,
}

#[derive(Debug, Default)]
struct Kit {
    on: bool,
    next: u32,
    bodies: BTreeMap<u32, Body>,
    /// The section of a group type, where a test set one.
    sections: BTreeMap<u8, u32>,
    /// Calls the race and sex menu's functions got, by address.
    calls: Vec<u32>,
}

fn new(e: &mut Engine, size: u32) -> u32 {
    let k = e.ext::<Kit>();
    let at = k.next;
    k.next += (size + 15) & !15;
    assert!(k.next <= AK + AK_LEN, "the anim kit's memory is used up");
    at
}

fn on(e: &mut Engine) -> bool {
    e.ext::<Kit>().on
}

fn body(e: &mut Engine, actor: u32) -> &mut Body {
    let fresh = !e.ext::<Kit>().bodies.contains_key(&actor);
    if fresh {
        let k = e.ext::<Kit>();
        let b = k.bodies.entry(actor).or_default();
        b.shape.walk = WALK;
        b.shape.run = RUN;
        b.shape.fatigue = FATIGUE;
    }
    e.ext::<Kit>().bodies.entry(actor).or_default()
}

fn anim_of(e: &mut Engine, actor: u32) -> u32 {
    if body(e, actor).anim == 0 {
        let a = new(e, ANIM_LEN);
        for s in 0..addr::ANIMATION_SECTIONS {
            e.mem.set_u16(a + GROUP_AT + 2 * s, addr::ANIM_GROUP_NONE);
        }
        body(e, actor).anim = a;
    }
    body(e, actor).anim
}

fn actor_of(e: &mut Engine, f: impl Fn(u32, &Body) -> bool) -> Option<u32> {
    e.ext::<Kit>()
        .bodies
        .iter()
        .find(|(a, b)| f(**a, b))
        .map(|(a, _)| *a)
}

fn actor_of_process(e: &mut Engine, p: u32) -> Option<u32> {
    let actors: Vec<u32> = e.ext::<Kit>().bodies.keys().copied().collect();
    actors
        .into_iter()
        .find(|a| e.mem.u32(a + addr::ACTOR_PROCESS_OFFSET) == p)
}

fn st0(v: f32) -> Ret {
    Ret {
        st0: f64::from(v),
        ..Ret::default()
    }
}

fn id_of(e: &mut Engine, form: u32) -> u32 {
    match form {
        0 => 0,
        f => e.mem.u32(f + addr::FORM_ID_OFFSET),
    }
}

/// Gives the process of `actor` the kit's process slots: its own vtable when it has none, else
/// the kit's slots in the vtable it has.
fn dress(e: &mut Engine, actor: u32) {
    let p = e.mem.u32(actor + addr::ACTOR_PROCESS_OFFSET);
    if p == 0 {
        return;
    }
    let mut vt = e.mem.u32(p);
    if vt == 0 {
        vt = PROC_VT;
        e.mem.set_u32(p, vt);
    }
    for (slot, f) in [
        (addr::PROCESS_SET_IDLE_SLOT, SET_IDLE_FN),
        (addr::PROCESS_QUEUE_FLAGS_SLOT, FLAGS_FN),
        (addr::PROCESS_SIT_SLEEP_STATE_SLOT, SIT_FN),
        (addr::PROCESS_KNOCK_STATE_SLOT, KNOCK_FN),
        (addr::PROCESS_CURRENT_FURNITURE_SLOT, FURNITURE_FN),
    ] {
        e.mem.set_u32(vt + slot, f);
    }
    body(e, actor);
}

/// Called by the fights' `PlaceAtMe` for each reference it placed: with the kit installed, an
/// actor with a body.
pub(super) fn placed(e: &mut Engine, r: u32) {
    if on(e) {
        dress(e, r);
    }
}

/// Puts the anim kit on `s`: bodies for the player and every placed actor.
pub fn install(s: &mut Story) {
    let e = &mut s.e;
    e.map(AK, AK_LEN);
    {
        let k = e.ext::<Kit>();
        k.on = true;
        k.next = FIRST;
    }
    let actor_vt = e.mem.u32(PLAYER);
    for (slot, f) in [
        (addr::ACTOR_GET_ANIMATION_SLOT, ANIM_FN),
        (addr::ACTOR_PUT_PACKAGE_SLOT, PUT_PACKAGE_FN),
        (addr::ACTOR_HEAD_SLOTS[0], HEAD_FN),
        (addr::ACTOR_HEAD_SLOTS[1], BIPED_HEAD_FN),
    ] {
        e.mem.set_u32(actor_vt + slot, f);
    }
    for s in 0..addr::ANIMATION_SECTIONS {
        e.mem.set_u16(FIRST_PERSON + GROUP_AT + 2 * s, 0x77);
    }
    e.mem.set_u32(NODE_VT + 0xc, ROOT_FN);
    e.mem.set_u32(PARENT_VT + addr::NODE_DETACH_SLOT, DETACH_FN);
    dress(e, PLAYER);
    e.register(ANIM_FN, |e, a| eax(anim_of(e, a[0])));
    e.register(addr::PLAYER_GET_ANIMATION, |e, a| {
        // The first-person animation is another one, which co-op is not to read.
        if a[1] & 0xff != 0 {
            return eax(FIRST_PERSON);
        }
        eax(anim_of(e, a[0]))
    });
    e.register(addr::ANIMATION_GROUP_OF_SLOT, |e, a| {
        eax(u32::from(e.mem.u16(a[0] + GROUP_AT + 2 * a[1])))
    });
    e.register(addr::ACTOR_GET_ANIM_GROUP, |_, a| {
        eax((a[1] & 0xff) | u32::from(WEAPON_BITS))
    });
    e.register(addr::ANIMATION_PLAY_GROUP, |e, a| {
        let (anim, group) = (a[0], a[1] as u16);
        let kind = group as u8;
        let section = e.ext::<Kit>().sections.get(&kind).copied().unwrap_or(3);
        e.mem.set_u16(anim + GROUP_AT + 2 * section, group);
        if let Some(actor) = actor_of(e, |_, b| b.anim == anim) {
            body(e, actor).shape.plays.push(group);
        }
        Ret::default()
    });
    e.register(addr::ACTOR_SET_ANIM_ACTION, |e, a| {
        body(e, a[0]).shape.actions.push(a[1]);
        Ret::default()
    });
    e.register(addr::ANIMATION_SECTION_SEQUENCE, |_, _| eax(0));
    e.register(addr::ANIM_GROUP_IS_ATTACK, |_, a| {
        eax(u32::from((0x20..0x30).contains(&(a[0] & 0xff))))
    });
    e.register(addr::ANIM_GROUP_IS_AIM, |_, a| {
        eax(u32::from((0x30..0x40).contains(&(a[0] & 0xff))))
    });
    e.register(SET_IDLE_FN, |e, a| {
        if let Some(actor) = actor_of_process(e, a[0]) {
            body(e, actor).idle_asked = a[1];
        }
        Ret::default()
    });
    e.register(FLAGS_FN, |e, a| {
        let Some(actor) = actor_of_process(e, a[0]) else {
            return Ret::default();
        };
        let idle = body(e, actor).idle_asked;
        if a[1] & addr::PROCESS_FLAG_PLAY_IDLE != 0 && idle != 0 {
            let held = new(e, 0x40);
            e.mem.set_u32(held + addr::ANIM_IDLE_FORM_OFFSET, idle);
            let anim = anim_of(e, actor);
            e.mem.set_u32(anim + addr::ANIMATION_IDLE_OFFSET, held);
            let id = id_of(e, idle);
            let b = body(e, actor);
            b.idle_asked = 0;
            b.shape.idles.push(id);
        }
        Ret::default()
    });
    e.register(SIT_FN, |e, a| {
        let actor = actor_of_process(e, a[0]).unwrap_or(0);
        eax(body(e, actor).sit as u32)
    });
    e.register(KNOCK_FN, |e, a| {
        let actor = actor_of_process(e, a[0]).unwrap_or(0);
        let b = body(e, actor);
        let k = if b.shape.fatigue < 0.0 {
            KNOCKED
        } else {
            b.knock
        };
        eax(k as u32)
    });
    e.register(FURNITURE_FN, |e, a| {
        let actor = actor_of_process(e, a[0]).unwrap_or(0);
        eax(body(e, actor).furniture)
    });
    e.register(addr::MOBILE_GET_CHAR_CONTROLLER, |e, a| {
        body(e, a[0]);
        eax(a[0] + CONTROLLER_AT)
    });
    e.register(addr::CONTROLLER_STATE, |e, a| {
        eax(body(e, a[0] - CONTROLLER_AT).controller)
    });
    e.register(addr::MOBILE_JUMP, |e, a| {
        let b = body(e, a[0]);
        b.controller = addr::CONTROLLER_JUMPING;
        b.shape.jumps += 1;
        Ret::default()
    });
    e.register(addr::ACTOR_GET_IRON_SIGHTS, |e, a| {
        eax(u32::from(body(e, a[0]).shape.sights))
    });
    e.register(addr::ACTOR_SET_IRON_SIGHTS, |e, a| {
        let b = body(e, a[0]);
        b.shape.sights = a[1] & 0xff != 0;
        b.shape.sights_set += 1;
        Ret::default()
    });
    e.register(addr::ACTOR_GET_WALK_SPEED, |e, a| {
        st0(body(e, a[0]).shape.walk)
    });
    e.register(addr::ACTOR_GET_RUN_SPEED, |e, a| {
        st0(body(e, a[0]).shape.run)
    });
    e.register(addr::PROCESS_SET_WALK_SPEED, |e, a| {
        if let Some(actor) = actor_of_process(e, a[0]) {
            let b = body(e, actor);
            b.shape.walk = f32::from_bits(a[1]);
            b.shape.speeds_set += 1;
        }
        Ret::default()
    });
    e.register(addr::PROCESS_SET_RUN_SPEED, |e, a| {
        if let Some(actor) = actor_of_process(e, a[0]) {
            body(e, actor).shape.run = f32::from_bits(a[1]);
        }
        Ret::default()
    });
    // Fatigue through the actor's change of a value; any other value as the fight kit has it.
    e.register(addr::ACTOR_MODIFY_VALUE, |e, a| {
        if a[1] != addr::ACTOR_VALUE_FATIGUE {
            return super::fight_kit::modify_value(e, a);
        }
        let b = body(e, a[0]);
        b.shape.fatigue = (b.shape.fatigue + f32::from_bits(a[2])).min(FATIGUE);
        b.shape.knock = if b.shape.fatigue < 0.0 { KNOCKED } else { 0 };
        Ret::default()
    });
    // The sit and sleep package: a block with its type at +0 and its place at +4.
    e.register(addr::PACKAGE_CREATE, |e, a| {
        let p = new(e, 0x20);
        e.mem.set_u32(p, a[0]);
        eax(p)
    });
    for f in [
        addr::PACKAGE_SET_TYPE,
        addr::PACKAGE_SET_826B40,
        addr::PACKAGE_SET_826B90,
        addr::LOCATION_DELETE,
        addr::PACKAGE_SET_984F60,
    ] {
        e.register(f, |_, _| Ret::default());
    }
    e.register(addr::PACKAGE_LOCATION_CONSTRUCT, |e, a| {
        e.mem.set_u32(a[0], 0);
        eax(a[0])
    });
    e.register(addr::PACKAGE_LOCATION_SET_REFERENCE, |e, a| {
        e.mem.set_u32(a[0], a[1]);
        Ret::default()
    });
    e.register(addr::PACKAGE_SET_LOCATION, |e, a| {
        let at = e.mem.u32(a[1]);
        e.mem.set_u32(a[0] + 4, at);
        Ret::default()
    });
    e.register(PUT_PACKAGE_FN, |e, a| {
        let (actor, package) = (a[0], a[1]);
        if e.mem.u32(package) == addr::PACKAGE_TYPE_SIT_SLEEP {
            let furniture = e.mem.u32(package + 4);
            let id = id_of(e, furniture);
            let b = body(e, actor);
            b.furniture = furniture;
            b.sit = SITTING;
            b.shape.sits.push(id);
            b.shape.furniture = id;
        }
        Ret::default()
    });
    e.register(addr::ACTOR_GET_UP_PACKAGE, |e, a| {
        let b = body(e, a[0]);
        b.furniture = 0;
        b.sit = 0;
        b.shape.furniture = 0;
        b.shape.get_ups += 1;
        Ret::default()
    });
    // Heads: a node with a root, two head nodes with a parent.
    e.register(addr::REF_NODE, |e, a| {
        let n = new(e, 0x10);
        e.mem.set_u32(n, NODE_VT);
        body(e, a[0]);
        eax(n)
    });
    e.register(ROOT_FN, |_, a| eax(a[0]));
    e.register(addr::ACTOR_ANIM_DATA, |_, _| eax(0));
    e.register(HEAD_FN, |e, a| eax(body(e, a[0]).heads[0]));
    e.register(BIPED_HEAD_FN, |e, a| eax(body(e, a[0]).heads[1]));
    // `009611e0` is the testkit's package procedure type too: a kit head answers its parent, any
    // other object as the testkit's stand-in does.
    e.register(addr::NODE_PARENT, |e, a| {
        if (AK..AK + AK_LEN).contains(&a[0]) {
            return eax(e.mem.u32(a[0] + 4));
        }
        eax(if e.mem.u32(a[0] + 0x100) == 0 {
            u32::MAX
        } else {
            0
        })
    });
    e.register(addr::PALETTE_REMOVE, |_, _| Ret::default());
    e.register(DETACH_FN, |e, a| {
        let head = a[1];
        if let Some(actor) = actor_of(e, |_, b| b.heads.contains(&head)) {
            let b = body(e, actor);
            for h in b.heads.iter_mut().filter(|h| **h == head) {
                *h = 0;
            }
            b.shape.heads_taken_off += 1;
        }
        Ret::default()
    });
    e.register(addr::NPC_CLEAR_HEAD, |e, a| {
        let actors: Vec<u32> = e.ext::<Kit>().bodies.keys().copied().collect();
        for actor in actors {
            if e.mem.u32(actor + addr::REF_BASE_FORM_OFFSET) == a[0] {
                body(e, actor).shape.cleared += 1;
            }
        }
        Ret::default()
    });
    e.register(addr::ACTOR_BIPED, |_, a| eax(a[0] + 0x10));
    e.register(addr::NPC_HEAD_LOAD, |e, a| {
        let actor = a[1];
        let mut heads = [0; 2];
        for h in heads.iter_mut() {
            let node = new(e, 0x10);
            let parent = new(e, 0x10);
            e.mem.set_u32(parent, PARENT_VT);
            e.mem.set_u32(node + 4, parent);
            *h = node;
        }
        let b = body(e, actor);
        b.heads = heads;
        b.shape.heads_built += 1;
        Ret::default()
    });
    for f in [
        addr::FACE_PARAMS_CONSTRUCT,
        addr::FACE_PARAMS_DESTROY,
        addr::RACE_FACE_GEN_DATA,
    ] {
        e.register(f, |_, _| Ret::default());
    }
    e.register(addr::APPLY_FACE_PARAMS, |e, a| {
        let head = a[0];
        if let Some(actor) = actor_of(e, |_, b| b.heads.contains(&head)) {
            body(e, actor).shape.faces_applied += 1;
        }
        Ret::default()
    });
    // The race and sex menu's calls on the player, counted.
    macro_rules! counted {
        ($($f:expr),*) => {$(
            e.register($f, |e, _| {
                e.ext::<Kit>().calls.push($f);
                Ret::default()
            });
        )*};
    }
    counted!(
        addr::RACE_KILL_EGT_DATA,
        addr::PLAYER_RACE_CHANGED,
        addr::PLAYER_RECALCULATE,
        addr::PLAYER_UPDATE,
        addr::PLAYER_REFRESH_3D,
        addr::UPDATE_PLAYER_NAME,
        addr::NPC_UPDATE_3D
    );
    e.register(addr::NPC_SET_RACE, |e, a| {
        e.ext::<Kit>().calls.push(addr::NPC_SET_RACE);
        e.mem.set_u32(a[0] + addr::NPC_RACE_OFFSET, a[1]);
        Ret::default()
    });
    e.register(addr::ACTOR_BASE_SET_FLAG_BIT, |e, a| {
        e.ext::<Kit>().calls.push(addr::ACTOR_BASE_SET_FLAG_BIT);
        let flags = a[0] + 4;
        let v = e.mem.u32(flags);
        let v = if a[2] & 0xff != 0 {
            v | a[1]
        } else {
            v & !a[1]
        };
        e.mem.set_u32(flags, v);
        Ret::default()
    });
    e.register(addr::NPC_SET_HAIR, |e, a| {
        e.ext::<Kit>().calls.push(addr::NPC_SET_HAIR);
        e.mem.set_u32(a[0] + addr::NPC_HAIR_OFFSET, a[1]);
        Ret::default()
    });
    e.register(addr::NPC_SET_EYES, |e, a| {
        e.ext::<Kit>().calls.push(addr::NPC_SET_EYES);
        e.mem.set_u32(a[0] + addr::NPC_EYES_OFFSET, a[1]);
        Ret::default()
    });
}

impl Story {
    fn anim_ptr(&mut self, id: u32) -> u32 {
        forms::form(&mut self.e, id).expect("a reference of the story")
    }

    /// Reference `id`'s body as the kit has it.
    pub fn shape(&mut self, id: u32) -> Shape {
        let a = self.anim_ptr(id);
        body(&mut self.e, a).shape.clone()
    }

    /// Reference `id`'s game starts group `group` in section `section`, as its own animation
    /// code does.
    pub fn start_group(&mut self, id: u32, section: u32, group: u16) {
        let a = self.anim_ptr(id);
        let anim = anim_of(&mut self.e, a);
        self.e.mem.set_u16(anim + GROUP_AT + 2 * section, group);
    }

    /// The group playing in section `section` of reference `id`.
    pub fn group_in(&mut self, id: u32, section: u32) -> u16 {
        let a = self.anim_ptr(id);
        let anim = anim_of(&mut self.e, a);
        self.e.mem.u16(anim + GROUP_AT + 2 * section)
    }

    /// Group type `kind` plays in section `section` in this game.
    pub fn group_section(&mut self, kind: u8, section: u32) {
        self.e.ext::<Kit>().sections.insert(kind, section);
    }

    /// Reference `id`'s game plays idle `idle` (a form id) on it, as its own idle code does.
    pub fn start_idle(&mut self, id: u32, idle: u32) {
        let (a, f) = (self.anim_ptr(id), self.anim_ptr(idle));
        let held = new(&mut self.e, 0x40);
        self.e.mem.set_u32(held + addr::ANIM_IDLE_FORM_OFFSET, f);
        let anim = anim_of(&mut self.e, a);
        self.e.mem.set_u32(anim + addr::ANIMATION_IDLE_OFFSET, held);
    }

    /// Reference `id`'s controller state (0 ground, 1 jumping, 2 in the air).
    pub fn set_controller(&mut self, id: u32, state: u32) {
        let a = self.anim_ptr(id);
        body(&mut self.e, a).controller = state;
    }

    /// Reference `id`'s iron sights, walking and running speeds, knock state, as its own game
    /// has them.
    pub fn set_body(&mut self, id: u32, sights: bool, walk: f32, run: f32, knock: i32) {
        let a = self.anim_ptr(id);
        let b = body(&mut self.e, a);
        b.shape.sights = sights;
        b.shape.walk = walk;
        b.shape.run = run;
        b.knock = knock;
    }

    /// Reference `id` sits on furniture `furniture` (a reference id; 0 to get up) in its own
    /// game.
    pub fn set_furniture(&mut self, id: u32, furniture: u32) {
        let a = self.anim_ptr(id);
        let f = match furniture {
            0 => 0,
            f => self.anim_ptr(f),
        };
        let b = body(&mut self.e, a);
        b.furniture = f;
        b.sit = if f == 0 { 0 } else { SITTING };
        b.shape.furniture = furniture;
    }

    /// The race and sex menu's functions called, by address, in order.
    pub fn menu_calls(&mut self) -> Vec<u32> {
        self.e.ext::<Kit>().calls.clone()
    }
}

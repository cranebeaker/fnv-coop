//! The testkit's fights (docs/coop/SPECS.md S12): actors with health, a combat flag, a combat
//! target and a process, the game's list of actors near its player, and stand-ins for the game
//! functions co-op's fight code calls. Each does what co-op relies on the real one to do, and no
//! more:
//!
//! - the `ActorValueOwner`'s current value answers the actor's health; the actor's value change
//!   (virtual `0x3ac`) adds to it;
//! - the damage applier (virtual `0x338`) lowers health by the loss, notes the attacker, and
//!   does nothing for an essential actor;
//! - `Actor::HitMe` applies the hit's health damage through the applier and kills at 0;
//! - the process's start of combat (virtual `0x33c`) sets the combat flag and the target.

use engine_core::prelude::*;

use super::{bump, eax, Story, PLAYER, PLAYER_VTABLE, RAN, TABLE};
use crate::{addr, forms};

/// Test-only places inside an actor: its health, combat target, essential byte, last attacker,
/// and its process (whose first word is the process's vtable).
const HEALTH: u32 = 0x200;
const TARGET: u32 = 0x204;
const ESSENTIAL: u32 = 0x208;
const ATTACKER: u32 = 0x20c;
const PROCESS: u32 = 0x300;
const IN_COMBAT: u32 = 0x104;
/// The ghost byte of a placed reference (0x100 bytes: its process is one word at +0xf0).
const GHOST: u32 = 0xf4;
const SMALL_PROCESS: u32 = 0xf0;
/// The vtables of the `ActorValueOwner` part and of a process (in the player's vtable page).
const OWNER_VTABLE: u32 = PLAYER_VTABLE + 0x800;
const PROCESS_VTABLE: u32 = PLAYER_VTABLE + 0x900;
/// The addresses of the stand-ins.
const OWNER_VALUE_FN: u32 = 0x0130_0010;
const MODIFY_VALUE_FN: u32 = 0x0130_0020;
const APPLY_DAMAGE_FN: u32 = 0x0130_0030;
const COMBAT_TARGET_FN: u32 = 0x0130_0040;
const ALARMED_FN: u32 = 0x0130_0050;
const START_COMBAT_FN: u32 = 0x0130_0060;
/// Counters: real `HitMe` runs, starts of combat.
const HITS: u32 = RAN + 32;
const COMBATS: u32 = RAN + 36;
/// A scratch `HitData`.
const HIT_DATA: u32 = RAN + 0x80;

fn st0(v: f32) -> Ret {
    Ret {
        st0: f64::from(v),
        ..Ret::default()
    }
}

fn owner_value(e: &mut Engine, a: &[u32]) -> Ret {
    if a[1] != addr::ACTOR_VALUE_HEALTH {
        return st0(0.0);
    }
    st0(e.mem.f32(a[0] - addr::ACTOR_VALUE_OWNER_OFFSET + HEALTH))
}

fn modify_value(e: &mut Engine, a: &[u32]) -> Ret {
    if a[1] == addr::ACTOR_VALUE_HEALTH {
        let h = e.mem.f32(a[0] + HEALTH) + f32::from_bits(a[2]);
        e.mem.set_f32(a[0] + HEALTH, h);
    }
    Ret::default()
}

fn apply_damage(e: &mut Engine, a: &[u32]) -> Ret {
    let (actor, loss, attacker) = (a[0], f32::from_bits(a[1]), a[3]);
    if e.mem.u8(actor + ESSENTIAL) != 0 {
        return Ret::default();
    }
    if loss > 0.0 {
        let h = e.mem.f32(actor + HEALTH) - loss;
        e.mem.set_f32(actor + HEALTH, h);
        e.mem.set_u32(actor + ATTACKER, attacker);
    }
    Ret::default()
}

fn hit_me(e: &mut Engine, a: &[u32]) -> Ret {
    let (target, hit) = (a[0], a[1]);
    bump(e, HITS);
    let loss = e.mem.u32(hit + addr::HIT_HEALTH_DAMAGE_OFFSET);
    let aggressor = e.mem.u32(hit + addr::HIT_AGGRESSOR_OFFSET);
    e.vcall(
        target,
        addr::ACTOR_APPLY_DAMAGE_SLOT,
        &args![loss, 0u32, aggressor],
    );
    if e.mem.f32(target + HEALTH) <= 0.0 {
        e.call(addr::ACTOR_KILL, &args![target, aggressor, loss]);
    }
    Ret::default()
}

fn start_combat(e: &mut Engine, a: &[u32]) -> Ret {
    let (actor, target) = (a[1], a[2]);
    e.mem.set_u8(actor + IN_COMBAT, 1);
    e.mem.set_u32(actor + TARGET, target);
    bump(e, COMBATS);
    Ret::default()
}

fn place_at_me(e: &mut Engine, a: &[u32]) -> Ret {
    let r = super::place_at_me(e, a);
    if r.eax != 0 {
        e.mem.set_u32(r.eax, PLAYER_VTABLE);
        e.mem
            .set_u32(r.eax + addr::ACTOR_PROCESS_OFFSET, r.eax + SMALL_PROCESS);
    }
    r
}

/// Makes `actor` (in a slot of the heap of the story, or the player) an actor fights can read.
fn dress(e: &mut Engine, actor: u32) {
    e.mem.set_u32(actor, PLAYER_VTABLE);
    e.mem
        .set_u32(actor + addr::ACTOR_VALUE_OWNER_OFFSET, OWNER_VTABLE);
    e.mem.set_f32(actor + HEALTH, 100.0);
    e.mem
        .set_u32(actor + addr::ACTOR_PROCESS_OFFSET, actor + PROCESS);
    e.mem.set_u32(actor + PROCESS, PROCESS_VTABLE);
}

/// Called by `Story::new` once its forms are in the table and before co-op is installed.
pub(super) fn register(e: &mut Engine) {
    e.map(addr::PROCESS_LISTS & !0xfff, 0x1000);
    for (slot, f) in [
        (addr::ACTOR_MODIFY_VALUE_SLOT, MODIFY_VALUE_FN),
        (addr::ACTOR_APPLY_DAMAGE_SLOT, APPLY_DAMAGE_FN),
        (addr::ACTOR_COMBAT_TARGET_SLOT, COMBAT_TARGET_FN),
        (addr::ACTOR_ALARMED_SLOT, ALARMED_FN),
    ] {
        e.mem.set_u32(PLAYER_VTABLE + slot, f);
    }
    e.mem.set_u32(
        OWNER_VTABLE + addr::OWNER_CURRENT_VALUE_SLOT,
        OWNER_VALUE_FN,
    );
    e.mem.set_u32(
        PROCESS_VTABLE + addr::PROCESS_START_COMBAT_SLOT,
        START_COMBAT_FN,
    );
    for i in 0..e.mem.u32(TABLE) {
        let f = e.mem.u32(TABLE + 8 + 8 * i);
        let kind = e.mem.u8(f + addr::FORM_TYPE_OFFSET);
        if f == PLAYER || crate::actors::is_actor_type(kind) {
            dress(e, f);
            if f != PLAYER {
                e.mem.set_u8(f + addr::ACTOR_PROCESS_ME_OFFSET, 1);
            }
        }
    }
    e.register(OWNER_VALUE_FN, owner_value);
    e.register(MODIFY_VALUE_FN, modify_value);
    e.register(APPLY_DAMAGE_FN, apply_damage);
    e.register(COMBAT_TARGET_FN, |e, a| eax(e.mem.u32(a[0] + TARGET)));
    e.register(ALARMED_FN, |_, _| eax(0));
    e.register(START_COMBAT_FN, start_combat);
    e.register(addr::ACTOR_IN_COMBAT, |e, a| {
        eax(u32::from(e.mem.u8(a[0] + IN_COMBAT)))
    });
    e.register(addr::ACTOR_GET_ESSENTIAL, |e, a| {
        eax(u32::from(e.mem.u8(a[0] + ESSENTIAL)))
    });
    e.register(addr::ACTOR_HIT_ME, hit_me);
    e.register(addr::PLACE_AT_ME, place_at_me);
    e.register(addr::ACTOR_SET_GHOST, |e, a| {
        e.mem.set_u8(a[0] + GHOST, a[1] as u8);
        bump(e, super::GHOSTS);
        Ret::default()
    });
}

/// A placed actor of a story as a test reads it.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ActorView {
    /// Position.
    pub pos: [f32; 3],
    /// Heading, radians.
    pub heading: f32,
    /// Health.
    pub health: f32,
    /// Whether its AI runs (`bProcessMe`).
    pub ai: bool,
    /// Dying or dead.
    pub dead: bool,
    /// In combat.
    pub in_combat: bool,
    /// The form id of its combat target (0: none).
    pub target: u32,
    /// The form id of whoever last damaged it through the game (0: nobody).
    pub attacker: u32,
}

impl Story {
    fn actor_ptr(&mut self, id: u32) -> u32 {
        forms::form(&mut self.e, id).expect("an actor of the story")
    }

    fn id_at(&mut self, r: u32) -> u32 {
        match r {
            0 => 0,
            r => self.e.mem.u32(r + addr::FORM_ID_OFFSET),
        }
    }

    /// Puts actor `id` in `cell` at `pos`, facing `heading`, with its 3D, among the actors the
    /// game runs near its player.
    pub fn place_actor(&mut self, id: u32, cell: u32, pos: [f32; 3], heading: f32) {
        let a = self.actor_ptr(id);
        let c = forms::form(&mut self.e, cell).expect("a cell of the story");
        self.e.mem.set_u32(a + addr::REF_CELL_OFFSET, c);
        self.e.mem.set_u32(a + addr::REF_LOADED_OFFSET, 1);
        self.move_actor(id, pos, heading);
        self.set_near(id, true);
    }

    /// Adds actor `id` to the list of actors the game runs near its player, or takes it off.
    pub fn set_near(&mut self, id: u32, near: bool) {
        let a = self.actor_ptr(id);
        let lists = addr::PROCESS_LISTS;
        let mut list = crate::fight::near(&mut self.e);
        list.retain(|x| *x != a);
        if near {
            list.push(a);
        }
        for i in 0..addr::CLOSE_ACTORS_MAX {
            let v = list.get(i as usize).copied().unwrap_or(0);
            self.e
                .mem
                .set_u32(lists + addr::CLOSE_ACTORS_OFFSET + 4 * i, v);
        }
        self.e
            .mem
            .set_u32(lists + addr::CLOSE_ACTORS_COUNT_OFFSET, list.len() as u32);
    }

    /// Moves an actor as its own AI does.
    pub fn move_actor(&mut self, id: u32, pos: [f32; 3], heading: f32) {
        let a = self.actor_ptr(id);
        for (i, v) in pos.into_iter().enumerate() {
            self.e
                .mem
                .set_f32(a + addr::REF_POS_OFFSET + 4 * i as u32, v);
        }
        self.e.mem.set_f32(a + addr::REF_HEADING_OFFSET, heading);
    }

    /// An actor (or the player, id `0x14`) as this game has it.
    pub fn actor_view(&mut self, id: u32) -> ActorView {
        let a = self.actor_ptr(id);
        let (target, attacker) = (self.e.mem.u32(a + TARGET), self.e.mem.u32(a + ATTACKER));
        ActorView {
            pos: [0, 4, 8].map(|o| self.e.mem.f32(a + addr::REF_POS_OFFSET + o)),
            heading: self.e.mem.f32(a + addr::REF_HEADING_OFFSET),
            health: self.e.mem.f32(a + HEALTH),
            ai: self.e.mem.u8(a + addr::ACTOR_PROCESS_ME_OFFSET) != 0,
            dead: matches!(self.e.mem.u32(a + addr::ACTOR_LIFE_STATE_OFFSET), 1 | 2),
            in_combat: self.e.mem.u8(a + IN_COMBAT) != 0,
            target: self.id_at(target),
            attacker: self.id_at(attacker),
        }
    }

    /// Sets the health of an actor (or of the player).
    pub fn set_health(&mut self, id: u32, health: f32) {
        let a = self.actor_ptr(id);
        self.e.mem.set_f32(a + HEALTH, health);
    }

    /// Marks an actor essential (the game does not let damage through to it).
    pub fn set_essential(&mut self, id: u32, essential: bool) {
        let a = self.actor_ptr(id);
        self.e.mem.set_u8(a + ESSENTIAL, u8::from(essential));
    }

    /// Puts an actor in combat with `target` (a form id; 0 ends its combat), as its own AI does.
    pub fn set_combat(&mut self, id: u32, target: u32) {
        let a = self.actor_ptr(id);
        let t = match target {
            0 => 0,
            t => self.actor_ptr(t),
        };
        self.e.mem.set_u8(a + IN_COMBAT, u8::from(target != 0));
        self.e.mem.set_u32(a + TARGET, t);
    }

    /// A weapon hit as the game makes it: `Actor::HitMe` on `target` with a `HitData` naming
    /// `aggressor` (a form id; 0 for none) and `damage` points of health damage.
    pub fn hit(&mut self, target: u32, aggressor: u32, damage: f32) {
        let t = self.actor_ptr(target);
        let a = match aggressor {
            0 => 0,
            a => self.actor_ptr(a),
        };
        self.e.mem.set_u32(HIT_DATA + addr::HIT_AGGRESSOR_OFFSET, a);
        self.e
            .mem
            .set_u32(HIT_DATA + addr::HIT_LIMB_OFFSET, u32::MAX);
        self.e
            .mem
            .set_f32(HIT_DATA + addr::HIT_HEALTH_DAMAGE_OFFSET, damage);
        self.e.call(addr::ACTOR_HIT_ME, &args![t, HIT_DATA, 0u32]);
    }

    /// How often the game ran its own `HitMe` (a hit co-op held back does not count).
    pub fn hits_run(&self) -> u32 {
        self.e.mem.u32(HITS)
    }

    /// How often the game was asked to start a combat.
    pub fn combats_started(&self) -> u32 {
        self.e.mem.u32(COMBATS)
    }

    /// Whether a placed reference is a ghost.
    pub fn ghost(&mut self, id: u32) -> bool {
        let a = self.actor_ptr(id);
        self.e.mem.u8(a + GHOST) != 0
    }
}

impl Story {
    /// Whether the game killed its player.
    pub fn player_dead(&mut self) -> bool {
        matches!(
            self.e.mem.u32(PLAYER + addr::ACTOR_LIFE_STATE_OFFSET),
            1 | 2
        )
    }

    /// The byte of the player's disabled-controls flags.
    pub fn controls(&mut self) -> u8 {
        self.e
            .mem
            .u8(PLAYER + addr::PLAYER_DISABLED_CONTROLS_OFFSET)
    }
}

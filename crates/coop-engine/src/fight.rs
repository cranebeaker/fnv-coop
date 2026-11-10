//! Shared fights in the game (docs/coop/SPECS.md S12): the host's game is the authority for
//! actors that are not players.
//!
//! - Reading: the actors the game runs around its player ([`near`]) and one actor's place, life,
//!   health, combat flag and combat target ([`snap`]).
//! - Holding, on a guest: an actor the host runs is kept as an avatar is ([`hold`]): its AI off
//!   (`bProcessMe`), moved with the bodies of `SetPos` and `SetAngle`, its health set to the
//!   host's. [`release`] gives it back to the guest's game.
//!
//! Every function is called between two of the game's frames, as the live ladder's are.

use engine_core::prelude::*;

use crate::{actors, addr, live};

/// One actor as the game has it now.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Snap {
    /// The reference's form id.
    pub id: u32,
    /// The space it is in, as [`live::Local::cell`]: an interior cell's form id or a world
    /// space's; 0 when it is in no cell.
    pub space: u32,
    /// Position.
    pub pos: [f32; 3],
    /// Heading, radians.
    pub heading: f32,
    /// `Actor::eLifeState`: 0 alive, 1 dying, 2 dead, 6 an essential actor down.
    pub life: u32,
    /// Health (actor value `0x10`), as the game answers it.
    pub health: f32,
    /// The game's "in combat" byte.
    pub in_combat: bool,
    /// The form id of its combat target, 0 for none (read only while in combat).
    pub target: u32,
    /// Whether it has its 3D.
    pub loaded: bool,
}

impl Snap {
    /// Dying or dead.
    pub fn dead(&self) -> bool {
        matches!(self.life, 1 | 2)
    }
}

/// The space a reference is in (see [`Snap::space`]).
pub fn space_of(e: &mut Engine, r: u32) -> u32 {
    let cell = e.mem.u32(r.wrapping_add(addr::REF_CELL_OFFSET));
    if cell == 0 {
        return 0;
    }
    let space = if e.call(addr::CELL_IS_INTERIOR, &args![cell]).bool() {
        cell
    } else {
        match e.call(addr::CELL_GET_WORLD_SPACE, &args![cell]).u32() {
            0 => cell,
            world => world,
        }
    };
    e.mem.u32(space.wrapping_add(addr::FORM_ID_OFFSET))
}

/// The actors the game runs at full detail around its player, nearest first (at most 50): the
/// process lists' close-to-player array. Read without the array's lock, which is safe only
/// between frames, where every caller here is.
pub fn near(e: &mut Engine) -> Vec<u32> {
    let lists = addr::PROCESS_LISTS;
    let count = e.mem.u32(lists + addr::CLOSE_ACTORS_COUNT_OFFSET) as i32;
    let count = count.clamp(0, addr::CLOSE_ACTORS_MAX as i32) as u32;
    (0..count)
        .map(|i| e.mem.u32(lists + addr::CLOSE_ACTORS_OFFSET + 4 * i))
        .filter(|a| *a != 0)
        .collect()
}

/// An actor's health, through its `ActorValueOwner`.
pub fn health(e: &mut Engine, actor: u32) -> f32 {
    e.vcall(
        actor.wrapping_add(addr::ACTOR_VALUE_OWNER_OFFSET),
        addr::OWNER_CURRENT_VALUE_SLOT,
        &args![addr::ACTOR_VALUE_HEALTH],
    )
    .f32()
}

/// Reads the actor at `actor` (a pointer from [`near`], or a form looked up this frame).
pub fn snap(e: &mut Engine, actor: u32) -> Snap {
    let in_combat = e.call(addr::ACTOR_IN_COMBAT, &args![actor]).bool();
    let target = if in_combat {
        match e
            .vcall(actor, addr::ACTOR_COMBAT_TARGET_SLOT, &args![])
            .u32()
        {
            0 => 0,
            t => e.mem.u32(t.wrapping_add(addr::FORM_ID_OFFSET)),
        }
    } else {
        0
    };
    Snap {
        id: e.mem.u32(actor.wrapping_add(addr::FORM_ID_OFFSET)),
        space: space_of(e, actor),
        pos: [0, 4, 8].map(|o| e.mem.f32(actor.wrapping_add(addr::REF_POS_OFFSET + o))),
        heading: e.mem.f32(actor.wrapping_add(addr::REF_HEADING_OFFSET)),
        life: e.mem.u32(actor.wrapping_add(addr::ACTOR_LIFE_STATE_OFFSET)),
        health: health(e, actor),
        in_combat,
        target,
        loaded: e.mem.u32(actor.wrapping_add(addr::REF_LOADED_OFFSET)) != 0,
    }
}

/// The placed actor `id` as the game has it now; `None` when it is not in this game now.
pub fn snap_of(e: &mut Engine, id: u32) -> Option<Snap> {
    let a = actors::actor(e, id)?;
    Some(snap(e, a))
}

/// This game's player's health; `None` before the game has a player.
pub fn player_health(e: &mut Engine) -> Option<f32> {
    let p = live::player(e)?;
    Some(health(e, p))
}

/// Raises or lowers an actor's health to `want` through the game's own value change (virtual
/// `0x3ac`), when it is off by half a point or more.
fn set_health(e: &mut Engine, actor: u32, want: f32) {
    let change = want - health(e, actor);
    if change.abs() >= 0.5 {
        e.vcall(
            actor,
            addr::ACTOR_MODIFY_VALUE_SLOT,
            &args![addr::ACTOR_VALUE_HEALTH, change.to_bits(), 0u32],
        );
    }
}

/// What one frame's hold of an actor found.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Held {
    /// The game had its AI on (the first hold, or a load switched it back on); it is off now.
    pub woke: bool,
    /// Where the actor is after the move.
    pub pos: [f32; 3],
}

/// One frame of an actor the host runs, on a guest: its AI kept off, moved to `pos` and
/// `heading` (radians), and its health set to `health` (never below 1 here: a death comes as a
/// death, through the game's kill). `None`, and nothing done, when the actor is not in this
/// game now, has no 3D here, or is dead here.
pub fn hold(e: &mut Engine, id: u32, pos: [f32; 3], heading: f32, health: f32) -> Option<Held> {
    let a = actors::actor(e, id)?;
    if e.mem.u32(a.wrapping_add(addr::REF_LOADED_OFFSET)) == 0 || actors::dead(e, id)? {
        return None;
    }
    let flag = a.wrapping_add(addr::ACTOR_PROCESS_ME_OFFSET);
    let woke = e.mem.u8(flag) != 0;
    if woke {
        e.mem.set_u8(flag, 0);
        e.call(addr::ACTOR_END_MOVEMENT, &args![a]);
    }
    for (axis, v) in (*b"XYZ").into_iter().zip(pos) {
        e.call(addr::SET_POS_BASE, &args![a, u32::from(axis), v.to_bits()]);
    }
    e.call(
        addr::SET_ANGLE_BASE,
        &args![a, u32::from(b'Z'), heading.to_degrees().to_bits()],
    );
    set_health(e, a, health.max(1.0));
    Some(Held {
        woke,
        pos: [0, 4, 8].map(|o| e.mem.f32(a.wrapping_add(addr::REF_POS_OFFSET + o))),
    })
}

/// Gives a held actor back to this game: its AI on again. False when it is not in this game now.
pub fn release(e: &mut Engine, id: u32) -> bool {
    match actors::actor(e, id) {
        Some(a) => {
            e.mem
                .set_u8(a.wrapping_add(addr::ACTOR_PROCESS_ME_OFFSET), 1);
            true
        }
        None => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testkit::{Story, StorySpec};

    const HOUSE: u32 = 0x0010_5a00;
    const RAIDER: u32 = 0x0010_4f25;
    const GECKO: u32 = 0x0010_4f26;

    fn story() -> Story {
        Story::new(&StorySpec {
            cells: vec![HOUSE],
            exteriors: vec![(0x0010_5b00, 0x0000_da7e)],
            actors: vec![RAIDER, GECKO],
            ..StorySpec::default()
        })
    }

    #[test]
    fn the_actors_near_the_player_are_read_as_the_game_has_them() {
        let mut s = story();
        assert!(near(&mut s.e).is_empty());
        s.place_actor(RAIDER, HOUSE, [1.0, 2.0, 3.0], 0.5);
        s.place_actor(GECKO, 0x0010_5b00, [9.0, 9.0, 9.0], 0.0);
        s.set_health(GECKO, 40.0);
        s.set_combat(GECKO, RAIDER);
        let list = near(&mut s.e);
        assert_eq!(list.len(), 2);
        let (r, g) = (snap(&mut s.e, list[0]), snap(&mut s.e, list[1]));
        assert_eq!(
            (r.id, r.space, r.pos, r.heading, r.health, r.in_combat),
            (RAIDER, HOUSE, [1.0, 2.0, 3.0], 0.5, 100.0, false)
        );
        // An exterior's space is its world space; a combat target is named by its form id.
        assert_eq!(
            (g.id, g.space, g.health, g.in_combat, g.target, g.dead()),
            (GECKO, 0x0000_da7e, 40.0, true, RAIDER, false)
        );
        s.kill(GECKO);
        assert!(snap_of(&mut s.e, GECKO).unwrap().dead());
        s.set_near(RAIDER, false);
        assert_eq!(near(&mut s.e).len(), 1);
        assert_eq!(snap_of(&mut s.e, 0x9999), None);
        assert_eq!(player_health(&mut s.e), Some(100.0));
    }

    #[test]
    fn a_held_actor_has_its_ai_off_and_is_where_and_as_healthy_as_it_is_told() {
        let mut s = story();
        s.place_actor(RAIDER, HOUSE, [1.0, 2.0, 3.0], 0.5);
        let h = hold(&mut s.e, RAIDER, [10.0, 20.0, 30.0], 1.0, 62.0).unwrap();
        assert_eq!((h.woke, h.pos), (true, [10.0, 20.0, 30.0]));
        let v = s.actor_view(RAIDER);
        assert_eq!((v.pos, v.health, v.ai), ([10.0, 20.0, 30.0], 62.0, false));
        assert!((v.heading - 1.0).abs() < 1e-5);
        // Held again: nothing woke; health goes up as well as down, and never to a death.
        let h = hold(&mut s.e, RAIDER, [10.0, 20.0, 30.0], 1.0, 0.0).unwrap();
        assert!(!h.woke);
        assert_eq!(s.actor_view(RAIDER).health, 1.0);
        hold(&mut s.e, RAIDER, [10.0, 20.0, 30.0], 1.0, 80.0).unwrap();
        assert_eq!(s.actor_view(RAIDER).health, 80.0);
        assert!(release(&mut s.e, RAIDER));
        assert!(s.actor_view(RAIDER).ai);
        // Not here, without 3D, or dead: left alone.
        assert_eq!(hold(&mut s.e, 0x9999, [0.0; 3], 0.0, 1.0), None);
        assert_eq!(hold(&mut s.e, GECKO, [0.0; 3], 0.0, 1.0), None);
        s.kill(RAIDER);
        assert_eq!(hold(&mut s.e, RAIDER, [0.0; 3], 0.0, 50.0), None);
        assert_eq!(s.actor_view(RAIDER).pos, [10.0, 20.0, 30.0]);
        assert!(!release(&mut s.e, 0x9999));
    }
}

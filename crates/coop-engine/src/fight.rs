//! Shared fights in the game (docs/coop/SPECS.md S13): the host's game is the authority for
//! actors that are not players.
//!
//! - Reading: the actors the game runs around its player ([`near`]) and one actor's place, life,
//!   health, combat flag and combat target ([`snap`]).
//! - Holding, on a guest: an actor the host runs is kept as an avatar is ([`hold`]): its AI off
//!   (`bProcessMe`), moved with the bodies of `SetPos` and `SetAngle`, its health set to the
//!   host's. [`release`] gives it back to the guest's game.
//!
//! - Hits: a wrap of `Actor::HitMe` (`0089a760`), as [`Owner::Coop`], decides whose game applies
//!   a weapon hit ([`hit_me`]). The host's game applies every hit it makes, except one on a
//!   player's avatar, which belongs to that player's own game. A guest's game applies none on an
//!   actor the host runs: its player's hit is kept for the host as a claim, and what the held
//!   actors do to each other or to the guest's player is the host's to decide. With no session
//!   ([`Role::Off`]) every hit goes to the game as before.
//! - The player's fall: a wrap of `Actor::Kill` (`0089d900`) holds this game's player's death
//!   while a session shares revives ([`set_spare`]); the session reads the fall ([`take_falls`]),
//!   takes the player's controls away ([`set_controls`]) and gives them back on a revive.
//! - Damage the session decides: [`hurt_actor`] and [`kill_by`] on the host, [`hurt_player`] on
//!   the machine of the player who was hurt.
//!
//! Except for the wrap, every function is called between two of the game's frames, as the live
//! ladder's are.

use std::collections::BTreeSet;
use std::sync::atomic::{AtomicU32, AtomicU64, AtomicU8, Ordering};
use std::sync::{Mutex, PoisonError, RwLock};

use engine_core::extend::{Owner, WrapSlot};
use engine_core::prelude::*;

use crate::{actors, addr, forms, live};

/// Co-op's place in front of `Actor::Kill`.
pub(crate) const KILL: WrapSlot = WrapSlot::new(Owner::Coop, addr::ACTOR_KILL);

/// Co-op's place in front of `Actor::HitMe`.
pub(crate) const HIT_ME: WrapSlot = WrapSlot::new(Owner::Coop, addr::ACTOR_HIT_ME);

/// What this machine is in a session, for the hit wrap.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Role {
    /// No session, or fights not shared: every hit goes to the game.
    Off,
    /// The host: its game applies its hits, except those on a player's avatar.
    Host,
    /// A guest: hits on actors the host runs are the host's.
    Guest,
}

/// A weapon hit the wrap kept from the game.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Hit {
    /// The form id of the actor that was hit.
    pub target: u32,
    /// The form id of the aggressor (`HitData::pAggressor`), 0 for none.
    pub aggressor: u32,
    /// `HitData::fHealthDamage`.
    pub amount: f32,
    /// `HitData::eDamageLimb` (-1 for none).
    pub limb: i32,
}

/// Most hits kept between two drains.
const HITS_MAX: usize = 256;

/// The fights' game-wide state (a lock-free shared slot, as the journal's): the wrap runs on
/// whatever thread the game hits on, the session reads between frames.
#[derive(Debug, Default)]
pub struct Fights {
    /// 0 off, 1 host, 2 guest.
    role: AtomicU8,
    /// `Actor::HitMe` is wrapped on this game.
    wrapped: AtomicU8,
    /// The form ids of the other players' avatars in this game.
    avatars: RwLock<BTreeSet<u32>>,
    /// Guest: the form ids of the actors held at the host's state.
    held: RwLock<BTreeSet<u32>>,
    /// Hits kept from the game since the last drain.
    hits: Mutex<Vec<Hit>>,
    /// Hits kept from the game, ever.
    kept: AtomicU64,
    /// This game's player is not to die: its kill is held (1) or not (0).
    spare: AtomicU8,
    /// Kills of this game's player held since the last [`take_falls`].
    falls: AtomicU32,
}

/// Installs the hit wrap when the game has `Actor::HitMe` (the translation, or a test's
/// stand-in); called by [`crate::install`], before the game's threads start.
pub(crate) fn install(e: &mut Engine) {
    let wrapped = e.is_registered(addr::ACTOR_HIT_ME);
    if wrapped {
        e.wrap(HIT_ME, hit_me);
    }
    if e.is_registered(addr::ACTOR_KILL) {
        e.wrap(KILL, kill);
    }
    e.ext_free::<Fights>()
        .wrapped
        .store(u8::from(wrapped), Ordering::SeqCst);
}

/// Holds this game's player's death from now on (or stops holding it): a kill of the player is
/// kept from the game and counted for [`take_falls`]. Every other actor dies as before.
pub fn set_spare(e: &mut Engine, on: bool) {
    e.ext_free::<Fights>()
        .spare
        .store(u8::from(on), Ordering::SeqCst);
}

/// How often the player's death was held since the last call.
pub fn take_falls(e: &mut Engine) -> u32 {
    e.ext_free::<Fights>().falls.swap(0, Ordering::SeqCst)
}

/// The wrap at `0089d900`. Its words are the call's: the actor, the attacker and the damage.
pub(crate) fn kill(e: &mut Engine, a: &[u32]) -> Ret {
    let actor = a.first().copied().unwrap_or(0);
    let spare = e.ext_free::<Fights>().spare.load(Ordering::SeqCst) != 0;
    if spare && actor != 0 && actor == e.mem.u32(addr::PLAYER_SINGLETON) {
        e.ext_free::<Fights>().falls.fetch_add(1, Ordering::SeqCst);
        return Ret::default();
    }
    e.call_next(addr::ACTOR_KILL, a)
}

/// Whether `Actor::HitMe` is wrapped on this game (hits can be shared).
pub fn hits_wrapped(e: &mut Engine) -> bool {
    e.ext_free::<Fights>().wrapped.load(Ordering::SeqCst) != 0
}

/// Sets what this machine is for the hit wrap.
pub fn set_role(e: &mut Engine, role: Role) {
    e.ext_free::<Fights>().role.store(
        match role {
            Role::Off => 0,
            Role::Host => 1,
            Role::Guest => 2,
        },
        Ordering::SeqCst,
    );
}

fn put(set: &RwLock<BTreeSet<u32>>, ids: impl IntoIterator<Item = u32>) {
    *set.write().unwrap_or_else(PoisonError::into_inner) = ids.into_iter().collect();
}

fn has(set: &RwLock<BTreeSet<u32>>, id: u32) -> bool {
    set.read()
        .unwrap_or_else(PoisonError::into_inner)
        .contains(&id)
}

/// Names the other players' avatars in this game (their form ids).
pub fn set_avatars(e: &mut Engine, ids: impl IntoIterator<Item = u32>) {
    put(&e.ext_free::<Fights>().avatars, ids);
}

/// Names the actors a guest holds at the host's state (their form ids).
pub fn set_held(e: &mut Engine, ids: impl IntoIterator<Item = u32>) {
    put(&e.ext_free::<Fights>().held, ids);
}

/// The hits kept from the game since the last call, oldest first.
pub fn take_hits(e: &mut Engine) -> Vec<Hit> {
    std::mem::take(
        &mut *e
            .ext_free::<Fights>()
            .hits
            .lock()
            .unwrap_or_else(PoisonError::into_inner),
    )
}

/// How many hits the wrap kept from the game, ever.
pub fn hits_kept(e: &mut Engine) -> u64 {
    e.ext_free::<Fights>().kept.load(Ordering::SeqCst)
}

/// The wrap at `0089a760`. Its words are the call's: the actor that is hit, the `HitData` and
/// the kind byte. A hit it keeps from the game answers as the game's own call does for an actor
/// without a process: nothing happened.
pub(crate) fn hit_me(e: &mut Engine, a: &[u32]) -> Ret {
    let word = |i: usize| a.get(i).copied().unwrap_or(0);
    let (target, hit) = (word(0), word(1));
    let role = e.ext_free::<Fights>().role.load(Ordering::SeqCst);
    if role == 0 || target == 0 || hit == 0 {
        return e.call_next(addr::ACTOR_HIT_ME, a);
    }
    let id_of = |e: &mut Engine, r: u32| match r {
        0 => 0,
        r => e.mem.u32(r.wrapping_add(addr::FORM_ID_OFFSET)),
    };
    let tid = id_of(e, target);
    let aggressor = e.mem.u32(hit.wrapping_add(addr::HIT_AGGRESSOR_OFFSET));
    let aid = id_of(e, aggressor);
    let f = e.ext_free::<Fights>();
    // (keep the hit from the game, hand it to the session)
    let (keep, tell) = if has(&f.avatars, tid) {
        // A player's avatar: that player's own game takes the damage, if the host says so.
        (true, true)
    } else if role == 2 && tid == actors::PLAYER_REF_ID {
        // This guest's player: what the host's actors and the other players do to it comes
        // from the host.
        (has(&f.held, aid) || has(&f.avatars, aid), false)
    } else if role == 2 && has(&f.held, tid) {
        // An actor the host runs: this player's hit is a claim for the host; anything else
        // that hits it here is for the host's game to decide.
        (true, aid == actors::PLAYER_REF_ID)
    } else {
        (false, false)
    };
    if !keep {
        return e.call_next(addr::ACTOR_HIT_ME, a);
    }
    let kept = Hit {
        target: tid,
        aggressor: aid,
        amount: e.mem.f32(hit.wrapping_add(addr::HIT_HEALTH_DAMAGE_OFFSET)),
        limb: e.mem.u32(hit.wrapping_add(addr::HIT_LIMB_OFFSET)) as i32,
    };
    let f = e.ext_free::<Fights>();
    f.kept.fetch_add(1, Ordering::SeqCst);
    if tell {
        let mut hits = f.hits.lock().unwrap_or_else(PoisonError::into_inner);
        if hits.len() < HITS_MAX {
            hits.push(kept);
        }
    }
    Ret::default()
}

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
    /// `Actor::GetEssential`: the game does not let it die.
    pub essential: bool,
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
        essential: e.call(addr::ACTOR_GET_ESSENTIAL, &args![actor]).bool(),
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

/// The host's game takes a guest's hit on an actor: `amount` points of health through the
/// game's own damage applier (virtual `0x338`), from `attacker` (a form id, the guest's avatar
/// here, or 0), so the difficulty's adjustment and the combat controllers' notes happen as for
/// any hit. Answers the actor's health afterwards; `None` when it is not in this game now. The
/// applier does not kill: see [`kill_by`].
pub fn hurt_actor(e: &mut Engine, id: u32, amount: f32, attacker: u32) -> Option<f32> {
    let a = actors::actor(e, id)?;
    let by = forms::form(e, attacker).unwrap_or(0);
    e.vcall(
        a,
        addr::ACTOR_APPLY_DAMAGE_SLOT,
        &args![amount.to_bits(), 0.0f32.to_bits(), by],
    );
    Some(health(e, a))
}

/// Kills actor `id` through the game's `Actor::Kill`, credited to `attacker` (a form id, or 0).
/// False when the actor is not in this game now. The game keeps an essential actor alive.
pub fn kill_by(e: &mut Engine, id: u32, attacker: u32) -> bool {
    let Some(a) = actors::actor(e, id) else {
        return false;
    };
    let by = forms::form(e, attacker).unwrap_or(0);
    e.call(addr::ACTOR_KILL, &args![a, by, 0.0f32.to_bits()]);
    true
}

/// This game's player takes `amount` points of health damage the host decided, from `attacker`
/// (a form id, or 0), through the game's own damage applier. Answers the player's health
/// afterwards; `None` before the game has a player.
pub fn hurt_player(e: &mut Engine, amount: f32, attacker: u32) -> Option<f32> {
    let p = live::player(e)?;
    let by = forms::form(e, attacker).unwrap_or(0);
    e.vcall(
        p,
        addr::ACTOR_APPLY_DAMAGE_SLOT,
        &args![amount.to_bits(), 0.0f32.to_bits(), by],
    );
    Some(health(e, p))
}

/// Kills this game's player through the game's `Actor::Kill`, as a lethal hit does (the game's
/// own death follows). False before the game has a player.
pub fn kill_player(e: &mut Engine) -> bool {
    match live::player(e) {
        Some(p) => {
            e.call(addr::ACTOR_KILL, &args![p, 0u32, 0.0f32.to_bits()]);
            true
        }
        None => false,
    }
}

/// The byte of this game's player's disabled-controls flags; `None` before the game has a
/// player.
pub fn controls(e: &mut Engine) -> Option<u8> {
    let p = live::player(e)?;
    Some(
        e.mem
            .u8(p.wrapping_add(addr::PLAYER_DISABLED_CONTROLS_OFFSET)),
    )
}

/// Writes the byte of this game's player's disabled-controls flags (all seven:
/// [`addr::PLAYER_ALL_CONTROLS`]). Only the byte the game reads is written; what the
/// `DisablePlayerControls` command does besides (the weapon put away) is not done.
pub fn set_controls(e: &mut Engine, flags: u8) {
    if let Some(p) = live::player(e) {
        e.mem
            .set_u8(p.wrapping_add(addr::PLAYER_DISABLED_CONTROLS_OFFSET), flags);
    }
}

/// Sets this game's player's health to `want` through the game's own value change.
pub fn set_player_health(e: &mut Engine, want: f32) {
    if let Some(p) = live::player(e) {
        set_health(e, p, want);
    }
}

/// Makes actor `id` fight `target` (form ids; the target is a player's avatar, or this game's
/// player, `0x14`) the way the `StartCombat` command does: the actor's process starts the
/// combat (virtual `0x33c`, with the command's own words). False, and nothing done, when either
/// is not in this game now or has no process, as the command checks.
pub fn start_combat(e: &mut Engine, id: u32, target: u32) -> bool {
    let (Some(a), Some(t)) = (actors::actor(e, id), forms::form(e, target)) else {
        return false;
    };
    let process = e.mem.u32(a.wrapping_add(addr::ACTOR_PROCESS_OFFSET));
    if process == 0 || e.mem.u32(t.wrapping_add(addr::ACTOR_PROCESS_OFFSET)) == 0 {
        return false;
    }
    let not_alarmed = !e.vcall(a, addr::ACTOR_ALARMED_SLOT, &args![]).bool();
    e.vcall(
        process,
        addr::PROCESS_START_COMBAT_SLOT,
        &args![
            a,
            t,
            1u32,
            1u32,
            0u32,
            0u32,
            0u32,
            u32::from(not_alarmed),
            0u32,
            1u32,
            0u32,
            1u32,
            0u32
        ],
    );
    true
}

/// Sets or clears an avatar's ghost flag (`Actor::SetGhost`). An avatar is placed as a ghost;
/// one the host's actors are to fight is not one. False when it is not in this game now.
pub fn set_ghost(e: &mut Engine, id: u32, on: bool) -> bool {
    match forms::form(e, id) {
        Some(r) => {
            e.call(addr::ACTOR_SET_GHOST, &args![r, u32::from(on)]);
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

#[cfg(test)]
mod hit_tests {
    use super::*;
    use crate::testkit::{Story, StorySpec};

    const HOUSE: u32 = 0x0010_5a00;
    const RAIDER: u32 = 0x0010_4f25;
    const GECKO: u32 = 0x0010_4f26;
    const PLAYER: u32 = actors::PLAYER_REF_ID;

    fn story() -> Story {
        let mut s = Story::new(&StorySpec {
            cells: vec![HOUSE],
            actors: vec![RAIDER, GECKO],
            ..StorySpec::default()
        });
        s.move_player(HOUSE, [0.0; 3], 0.0);
        s.place_actor(RAIDER, HOUSE, [1.0, 2.0, 3.0], 0.5);
        s.place_actor(GECKO, HOUSE, [4.0, 5.0, 6.0], 0.5);
        s
    }

    #[test]
    fn with_no_session_every_hit_is_the_games() {
        let mut s = story();
        assert!(hits_wrapped(&mut s.e));
        set_held(&mut s.e, [RAIDER]);
        s.hit(RAIDER, PLAYER, 30.0);
        s.hit(PLAYER, RAIDER, 10.0);
        assert_eq!(s.hits_run(), 2);
        assert_eq!(s.actor_view(RAIDER).health, 70.0);
        assert_eq!(s.actor_view(PLAYER).health, 90.0);
        assert_eq!((hits_kept(&mut s.e), take_hits(&mut s.e)), (0, vec![]));
    }

    #[test]
    fn a_guest_keeps_hits_on_held_actors_and_from_them() {
        let mut s = story();
        let avatar = live::avatar_create(&mut s.e, 7).unwrap().id;
        set_role(&mut s.e, Role::Guest);
        set_held(&mut s.e, [RAIDER]);
        set_avatars(&mut s.e, [avatar]);
        // The player's hit on a held actor: kept, and handed over as a claim.
        s.hit(RAIDER, PLAYER, 30.0);
        // Someone else's hit on it, its hit on the player, an avatar's hit on the player: kept,
        // and nobody's claim.
        s.hit(RAIDER, GECKO, 20.0);
        s.hit(PLAYER, RAIDER, 10.0);
        s.hit(PLAYER, avatar, 10.0);
        // The player's hit on an avatar: kept, and handed over (the friendly-fire rule).
        s.hit(avatar, PLAYER, 15.0);
        assert_eq!(s.hits_run(), 0);
        assert_eq!(hits_kept(&mut s.e), 5);
        assert_eq!(
            take_hits(&mut s.e),
            vec![
                Hit {
                    target: RAIDER,
                    aggressor: PLAYER,
                    amount: 30.0,
                    limb: -1
                },
                Hit {
                    target: avatar,
                    aggressor: PLAYER,
                    amount: 15.0,
                    limb: -1
                }
            ]
        );
        assert_eq!(take_hits(&mut s.e), vec![]);
        // An actor this game runs itself, and the world's own harm to the player: the game's.
        s.hit(GECKO, PLAYER, 40.0);
        s.hit(PLAYER, GECKO, 5.0);
        s.hit(PLAYER, 0, 5.0);
        assert_eq!(s.hits_run(), 3);
        assert_eq!(s.actor_view(GECKO).health, 60.0);
        assert_eq!(s.actor_view(PLAYER).health, 90.0);
    }

    #[test]
    fn a_host_keeps_only_hits_on_avatars() {
        let mut s = story();
        let avatar = live::avatar_create(&mut s.e, 7).unwrap().id;
        set_role(&mut s.e, Role::Host);
        set_avatars(&mut s.e, [avatar]);
        s.hit(avatar, RAIDER, 25.0);
        assert_eq!(s.hits_run(), 0);
        assert_eq!(
            take_hits(&mut s.e),
            vec![Hit {
                target: avatar,
                aggressor: RAIDER,
                amount: 25.0,
                limb: -1
            }]
        );
        s.hit(RAIDER, PLAYER, 30.0);
        s.hit(PLAYER, RAIDER, 10.0);
        s.hit(GECKO, RAIDER, 10.0);
        assert_eq!(s.hits_run(), 3);
        // Back to no session: the avatar's hits are the game's again.
        set_role(&mut s.e, Role::Off);
        s.hit(RAIDER, PLAYER, 30.0);
        assert_eq!(s.hits_run(), 4);
    }

    #[test]
    fn the_session_s_damage_goes_through_the_game() {
        let mut s = story();
        let avatar = live::avatar_create(&mut s.e, 7).unwrap().id;
        assert_eq!(hurt_actor(&mut s.e, RAIDER, 30.0, avatar), Some(70.0));
        assert_eq!(s.actor_view(RAIDER).attacker, avatar);
        assert_eq!(hurt_actor(&mut s.e, 0x9999, 30.0, 0), None);
        // The applier does not kill; the kill is the game's own.
        assert_eq!(hurt_actor(&mut s.e, RAIDER, 100.0, 0), Some(-30.0));
        assert_eq!(s.dead(RAIDER), Some(false));
        assert!(kill_by(&mut s.e, RAIDER, avatar));
        assert_eq!(s.dead(RAIDER), Some(true));
        assert!(!kill_by(&mut s.e, 0x9999, 0));
        assert_eq!(hurt_player(&mut s.e, 12.0, GECKO), Some(88.0));
        assert_eq!(s.actor_view(PLAYER).attacker, GECKO);
        // An enemy is set on a player's avatar the way StartCombat does it; an avatar can stop
        // being a ghost.
        assert!(s.ghost(avatar));
        assert!(set_ghost(&mut s.e, avatar, false));
        assert!(!s.ghost(avatar));
        assert!(start_combat(&mut s.e, GECKO, avatar));
        let g = snap_of(&mut s.e, GECKO).unwrap();
        assert_eq!((g.in_combat, g.target), (true, avatar));
        assert!(!start_combat(&mut s.e, GECKO, 0x9999));
        assert!(!start_combat(&mut s.e, 0x9999, avatar));
        assert_eq!(s.combats_started(), 1);
    }
}

#[cfg(test)]
mod fall_tests {
    use super::*;
    use crate::testkit::{Story, StorySpec};

    const HOUSE: u32 = 0x0010_5a00;
    const RAIDER: u32 = 0x0010_4f25;
    const PLAYER: u32 = actors::PLAYER_REF_ID;

    #[test]
    fn a_spared_player_s_death_is_held_and_counted_and_nobody_else_s() {
        let mut s = Story::new(&StorySpec {
            cells: vec![HOUSE],
            actors: vec![RAIDER],
            ..StorySpec::default()
        });
        s.move_player(HOUSE, [0.0; 3], 0.0);
        s.place_actor(RAIDER, HOUSE, [1.0, 2.0, 3.0], 0.5);
        set_spare(&mut s.e, true);
        // A lethal hit of the game's own: the damage lands, the kill is held.
        s.hit(PLAYER, RAIDER, 500.0);
        assert!(!s.player_dead());
        assert_eq!(player_health(&mut s.e), Some(-400.0));
        assert!(kill_player(&mut s.e));
        assert_eq!(take_falls(&mut s.e), 2);
        assert_eq!(take_falls(&mut s.e), 0);
        // The fallen player: 1 health, no controls; then up again.
        set_player_health(&mut s.e, 1.0);
        assert_eq!(controls(&mut s.e), Some(0));
        set_controls(&mut s.e, addr::PLAYER_ALL_CONTROLS);
        assert_eq!(s.controls(), 0x7f);
        set_controls(&mut s.e, 0);
        set_player_health(&mut s.e, 30.0);
        assert_eq!(player_health(&mut s.e), Some(30.0));
        // Other actors die as before.
        s.kill(RAIDER);
        assert_eq!(s.dead(RAIDER), Some(true));
        assert_eq!(take_falls(&mut s.e), 0);
        // Not spared any more: the player dies as the game has it.
        set_spare(&mut s.e, false);
        assert!(kill_player(&mut s.e));
        assert!(s.player_dead());
        assert_eq!(take_falls(&mut s.e), 0);
    }
}

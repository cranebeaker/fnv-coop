//! The live side of co-op in the game (docs/coop/DECISIONS.md D3): where the local player is, moving
//! a guest's player to the host's cell, and a remote player's avatar: an actor placed with the
//! game's own `PlaceAtMe`, its AI switched off, moved each frame with the bodies of `SetPos` and
//! `SetAngle`. Every function here is one step of the ladder `coop-engine-session` logs.

use engine_core::prelude::*;

use crate::{addr, forms};

/// Where the local player is.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Local {
    /// The space the player is in: the interior cell's form id, or for an exterior the world
    /// space's (its cells are one seamless place, and a world space is always a known form while an
    /// exterior cell is only one where it is loaded).
    pub cell: u32,
    /// Position.
    pub pos: [f32; 3],
    /// Heading, radians.
    pub heading: f32,
    /// Whether the player has its 3D.
    pub loaded: bool,
}

/// The player, or `None` before the game has one.
pub fn player(e: &mut Engine) -> Option<u32> {
    match e.mem.u32(addr::PLAYER_SINGLETON) {
        0 => None,
        p => Some(p),
    }
}

fn pos_of(e: &mut Engine, r: u32) -> [f32; 3] {
    [0, 4, 8].map(|o| e.mem.f32(r.wrapping_add(addr::REF_POS_OFFSET + o)))
}

/// Where the local player is; `None` at the main menu (no player, or a player in no cell).
pub fn local(e: &mut Engine) -> Option<Local> {
    let p = player(e)?;
    let cell = e.mem.u32(p.wrapping_add(addr::REF_CELL_OFFSET));
    if cell == 0 {
        return None;
    }
    let space = if e.call(addr::CELL_IS_INTERIOR, &args![cell]).bool() {
        cell
    } else {
        match e.call(addr::CELL_GET_WORLD_SPACE, &args![cell]).u32() {
            0 => cell,
            world => world,
        }
    };
    Some(Local {
        cell: e.mem.u32(space.wrapping_add(addr::FORM_ID_OFFSET)),
        pos: pos_of(e, p),
        heading: e.mem.f32(p.wrapping_add(addr::REF_HEADING_OFFSET)),
        loaded: e.mem.u32(p.wrapping_add(addr::REF_LOADED_OFFSET)) != 0,
    })
}

/// Why a step could not be done.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Refusal {
    /// The game has no player yet.
    NoPlayer,
    /// No form with that id is loaded (an exterior cell that is not loaded, or other data).
    FormNotFound(u32),
    /// `PlaceAtMe` answered no reference.
    NotPlaced,
    /// The player cannot be moved right now (the game says so); try again later.
    Busy,
    /// The game could not allocate the request.
    NoMemory,
}

impl std::fmt::Display for Refusal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Refusal::NoPlayer => write!(f, "the game has no player"),
            Refusal::FormNotFound(id) => write!(f, "form {id:08x} not found"),
            Refusal::NotPlaced => write!(f, "PlaceAtMe returned no reference"),
            Refusal::Busy => write!(f, "the game says the player cannot be moved now"),
            Refusal::NoMemory => write!(f, "the game could not allocate the move request"),
        }
    }
}

/// Writes a reference's position and heading through the game's `SetPos` and `SetAngle` bodies.
fn place(e: &mut Engine, r: u32, pos: [f32; 3], heading: f32) {
    for (axis, v) in (*b"XYZ").into_iter().zip(pos) {
        e.call(addr::SET_POS_BASE, &args![r, u32::from(axis), v.to_bits()]);
    }
    e.call(
        addr::SET_ANGLE_BASE,
        &args![r, u32::from(b'Z'), heading.to_degrees().to_bits()],
    );
}

/// Asks the game to move the local player to `pos`, facing `heading` (radians), in the space
/// `space` (an interior cell's form id or a world space's, as [`Local::cell`]). The game carries the
/// move out in its own frame (`PlayerCharacter::RequestPositionPlayer`, the way a door moves the
/// player); [`local`] shows the new place once it has.
pub fn warp(e: &mut Engine, space: u32, pos: [f32; 3], heading: f32) -> Result<(), Refusal> {
    let p = player(e).ok_or(Refusal::NoPlayer)?;
    let form = forms::form(e, space).ok_or(Refusal::FormNotFound(space))?;
    if e.vcall(p, addr::PLAYER_CANNOT_MOVE_SLOT, &args![0u32])
        .bool()
    {
        return Err(Refusal::Busy);
    }
    let request = e
        .call(addr::OPERATOR_NEW, &args![addr::POSITION_REQUEST_SIZE])
        .u32();
    if request == 0 {
        return Err(Refusal::NoMemory);
    }
    let request = e
        .call(addr::POSITION_REQUEST_CONSTRUCT, &args![request])
        .u32();
    for (i, v) in pos.into_iter().enumerate() {
        e.mem.set_f32(request + 0x08 + 4 * i as u32, v);
    }
    e.mem.set_f32(request + 0x1c, heading);
    e.mem.set_u8(request + 0x20, 1);
    let is_cell = e.mem.u8(form.wrapping_add(addr::FORM_TYPE_OFFSET)) == addr::FORM_TYPE_CELL;
    e.mem
        .set_u32(request + if is_cell { 0x04 } else { 0x00 }, form);
    e.call(addr::REQUEST_POSITION_PLAYER, &args![p, request]);
    Ok(())
}

/// A remote player's avatar: the reference's form id (a pointer is never kept across frames).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Avatar {
    /// The placed reference's form id.
    pub id: u32,
}

/// Places an avatar of base form `base` at the local player and switches its AI off.
pub fn avatar_create(e: &mut Engine, base: u32) -> Result<Avatar, Refusal> {
    let p = player(e).ok_or(Refusal::NoPlayer)?;
    let form = forms::form(e, base).ok_or(Refusal::FormNotFound(base))?;
    let r = e
        .call(
            addr::PLACE_AT_ME,
            &args![p, form, 1u32, 0u32, 0u32, 1.0f32.to_bits()],
        )
        .u32();
    if r == 0 {
        return Err(Refusal::NotPlaced);
    }
    e.mem
        .set_u8(r.wrapping_add(addr::ACTOR_PROCESS_ME_OFFSET), 0);
    e.call(addr::ACTOR_END_MOVEMENT, &args![r]);
    e.call(addr::ACTOR_SET_GHOST, &args![r, 1u32]);
    Ok(Avatar {
        id: e.mem.u32(r.wrapping_add(addr::FORM_ID_OFFSET)),
    })
}

/// What one frame's hold of an avatar found.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Held {
    /// The game had switched its AI back on (a load does); it was switched off again.
    pub woke: bool,
    /// It was in a cell this game does not have loaded (or in none): it was brought into the
    /// player's cell first (see [`avatar_hold`]).
    pub rehomed: bool,
    /// Where the avatar is after the move.
    pub pos: [f32; 3],
    /// It is within [`LOADED_REACH`] of this player and has no 3D: it should be seen and is not
    /// drawn.
    pub unseen: bool,
}

/// One frame of an avatar: its AI kept off, then moved to `pos` and `heading` (radians). `None`
/// when the reference is gone (the game deleted it, or another game was loaded).
///
/// Moving it (`SetPos`) leaves it in the cell it was placed in. When this game lets that cell go
/// (the player fast travelled away, or walked far), the avatar keeps its place but is drawn no more
/// (PC run 30: after a fast travel each player saw the other only once they went into the saloon
/// together). An avatar in a cell this game does not have loaded, or in none, is first brought
/// with the game's own `MoveTo` to its place, which puts it in the cell there, when that place is
/// within [`LOADED_REACH`] of this player (the cells this game keeps loaded). A player farther away
/// cannot be seen from here: the avatar is only moved (the harness, 2026-11-13: between one
/// player's fast travel and the other's, a `MoveTo` every frame put it in another unloaded cell).
pub fn avatar_hold(e: &mut Engine, a: Avatar, pos: [f32; 3], heading: f32) -> Option<Held> {
    let r = forms::form(e, a.id)?;
    let flag = r.wrapping_add(addr::ACTOR_PROCESS_ME_OFFSET);
    let woke = e.mem.u8(flag) != 0;
    if woke {
        e.mem.set_u8(flag, 0);
        e.call(addr::ACTOR_END_MOVEMENT, &args![r]);
    }
    let loaded = in_loaded_cell(e, r);
    let rehomed = !loaded && rehome(e, r, pos);
    place(e, r, pos, heading);
    let unseen = near_player(e, pos) && e.vcall(r, addr::REF_GET_3D_SLOT, &args![]).u32() == 0;
    Some(Held {
        woke,
        rehomed,
        pos: pos_of(e, r),
        unseen,
    })
}

/// Whether actor `id` is here and in a cell this game has loaded (one in a cell this game let go
/// is neither drawn nor moved by `SetPos`). True where the program has no such function.
pub fn is_loaded(e: &mut Engine, id: u32) -> bool {
    if !e.is_registered(addr::CELL_IS_LOADED) {
        return true;
    }
    forms::form(e, id).is_some_and(|r| in_loaded_cell(e, r))
}

/// Brings actor `id`, in a cell this game let go, to `pos` the way an avatar is (its `MoveTo` to
/// the local player, when `pos` is within [`LOADED_REACH`] of it). Whether it was brought.
pub fn bring_into_view(e: &mut Engine, id: u32, pos: [f32; 3]) -> bool {
    match forms::form(e, id) {
        Some(r) if e.is_registered(addr::CELL_IS_LOADED) && !in_loaded_cell(e, r) => {
            rehome(e, r, pos)
        }
        _ => false,
    }
}

/// Whether reference `r` is in a cell this game has loaded.
fn in_loaded_cell(e: &mut Engine, r: u32) -> bool {
    let cell = e.mem.u32(r.wrapping_add(addr::REF_CELL_OFFSET));
    let tes = e.mem.u32(addr::TES_SINGLETON);
    cell != 0 && e.call(addr::CELL_IS_LOADED, &args![tes, cell, 0u32]).bool()
}

/// How far from the local player an avatar's place can be and still be in the cells this game
/// keeps loaded: two exterior cells of 4096 units (the game's `uGridsToLoad` of 5 is the player's
/// cell and two around it).
pub const LOADED_REACH: f32 = 8192.0;

/// Whether `pos` is within [`LOADED_REACH`] of this game's player, in the world.
fn near_player(e: &mut Engine, pos: [f32; 3]) -> bool {
    match player(e) {
        Some(p) if e.mem.u32(p.wrapping_add(addr::REF_LOADED_OFFSET)) != 0 => {
            let at = [0, 4, 8].map(|o| e.mem.f32(p.wrapping_add(addr::REF_POS_OFFSET + o)));
            (0..2).map(|i| (pos[i] - at[i]).powi(2)).sum::<f32>().sqrt() <= LOADED_REACH
        }
        _ => false,
    }
}

/// Brings reference `r` to `pos` with the game's `MoveTo` to the local player (the difference as
/// offsets), which puts it in the cell there. False when the player is not in the world, `pos` is
/// farther than [`LOADED_REACH`] from it, or the program has no such function.
fn rehome(e: &mut Engine, r: u32, pos: [f32; 3]) -> bool {
    let p = match player(e) {
        Some(p) if e.mem.u32(p.wrapping_add(addr::REF_LOADED_OFFSET)) != 0 => p,
        _ => return false,
    };
    if !e.is_registered(addr::MOVE_TO_FUNCTION_BASE) {
        return false;
    }
    let at = [0, 4, 8].map(|o| e.mem.f32(p.wrapping_add(addr::REF_POS_OFFSET + o)));
    let far = (0..2).map(|i| (pos[i] - at[i]).powi(2)).sum::<f32>().sqrt();
    if far > LOADED_REACH {
        return false;
    }
    let off = [0, 1, 2].map(|i| (pos[i] - at[i]).to_bits());
    e.call(
        addr::MOVE_TO_FUNCTION_BASE,
        &args![r, p, off[0], off[1], off[2]],
    );
    true
}

/// Removes an avatar from the world (`TESObjectREFR::Disable`); false when it was already gone.
pub fn avatar_remove(e: &mut Engine, a: Avatar) -> bool {
    match forms::form(e, a.id) {
        Some(r) => {
            e.call(addr::REF_DISABLE, &args![r]);
            true
        }
        None => false,
    }
}

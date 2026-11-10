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
    /// Where the avatar is after the move.
    pub pos: [f32; 3],
}

/// One frame of an avatar: its AI kept off, then moved to `pos` and `heading` (radians). `None`
/// when the reference is gone (the game deleted it, or another game was loaded).
pub fn avatar_hold(e: &mut Engine, a: Avatar, pos: [f32; 3], heading: f32) -> Option<Held> {
    let r = forms::form(e, a.id)?;
    let flag = r.wrapping_add(addr::ACTOR_PROCESS_ME_OFFSET);
    let woke = e.mem.u8(flag) != 0;
    if woke {
        e.mem.set_u8(flag, 0);
        e.call(addr::ACTOR_END_MOVEMENT, &args![r]);
    }
    place(e, r, pos, heading);
    Some(Held {
        woke,
        pos: pos_of(e, r),
    })
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

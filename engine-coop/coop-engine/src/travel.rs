//! Where the local player is, as a place the other machine can name, and the move of a guest's
//! player to the host.
//!
//! A space is an interior cell, or a worldspace for an exterior: the cells of a worldspace are
//! one seamless place, a worldspace is a form that each game has loaded, and an exterior cell is
//! a form only where it is loaded. A guest is moved by the game's own request, the way a door
//! moves the player: the game carries it out in its own frame.

use crate::addr;
use crate::program::Program;

/// The space of a cell (its address): the cell's form id for an interior, the worldspace's
/// for an exterior (the cell's own id when the game knows no worldspace for it).
pub fn space_of(p: &mut dyn Program, cell: u32) -> u32 {
    let at = if p.call(addr::CELL_IS_INTERIOR, &[cell]).bool() {
        cell
    } else {
        match p.call(addr::CELL_WORLD_SPACE, &[cell]).eax {
            0 => cell,
            world => world,
        }
    };
    p.u32(at + addr::FORM_ID)
}

/// Where the local player is.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Local {
    /// The space ([`space_of`]).
    pub space: u32,
    /// The form id of the player's cell.
    pub cell: u32,
    /// The place of the feet.
    pub place: [f32; 3],
    /// The heading, in radians.
    pub heading: f32,
    /// True when the player has its 3D: it is in the world, not at a menu or in a load.
    pub loaded: bool,
}

/// Where the local player is. `None` at the main menu: no player, or a player in no cell.
pub fn local(p: &mut dyn Program) -> Option<Local> {
    let player = p.u32(addr::PLAYER);
    if player == 0 {
        return None;
    }
    let cell = p.u32(player + addr::REF_CELL);
    if cell == 0 {
        return None;
    }
    Some(Local {
        space: space_of(p, cell),
        cell: p.u32(cell + addr::FORM_ID),
        place: [0, 4, 8].map(|o| p.f32(player + addr::REF_POSITION + o)),
        heading: p.f32(player + addr::REF_ANGLE + 8),
        loaded: p.u32(player + addr::REF_NODE) != 0,
    })
}

/// Why the game was not asked to move the player.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Refusal {
    /// The game has no player yet.
    NoPlayer,
    /// No form with that id is loaded.
    NoSuchSpace(u32),
    /// The game says the player cannot be moved now. Ask again later.
    Busy,
    /// The game gave no memory for the request.
    NoMemory,
}

impl std::fmt::Display for Refusal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Refusal::NoPlayer => write!(f, "the game has no player"),
            Refusal::NoSuchSpace(id) => write!(f, "form {id:08x} is not loaded"),
            Refusal::Busy => write!(f, "the game says the player cannot be moved now"),
            Refusal::NoMemory => write!(f, "the game gave no memory for the move request"),
        }
    }
}

/// Asks the game to move the local player to `place`, facing `heading` (radians), in the space
/// `space` (an interior cell's form id or a worldspace's). The game moves the player in its own
/// frame; [`local`] shows the new place once it has.
pub fn request_move(
    p: &mut dyn Program,
    space: u32,
    place: [f32; 3],
    heading: f32,
) -> Result<(), Refusal> {
    let player = p.u32(addr::PLAYER);
    if player == 0 {
        return Err(Refusal::NoPlayer);
    }
    let form = p.call(addr::FORM_BY_ID, &[space]).eax;
    if form == 0 {
        return Err(Refusal::NoSuchSpace(space));
    }
    let busy = p.u32(p.u32(player) + addr::PLAYER_CANNOT_MOVE_SLOT);
    if busy != 0 && p.call(busy, &[player, 0]).bool() {
        return Err(Refusal::Busy);
    }
    let block = p.call(addr::HEAP_ALLOC, &[addr::MOVE_REQUEST_BYTES]).eax;
    if block == 0 {
        return Err(Refusal::NoMemory);
    }
    let request = p.call(addr::MOVE_REQUEST_NEW, &[block]).eax;
    for (i, v) in place.iter().enumerate() {
        p.set_f32(request + addr::MOVE_REQUEST_PLACE + 4 * i as u32, *v);
    }
    p.set_f32(request + addr::MOVE_REQUEST_HEADING, heading);
    p.set_u8(request + addr::MOVE_REQUEST_WEATHER, 1);
    let is_cell = p.call(addr::FORM_TYPE, &[form]).eax as u8 == addr::FORM_TYPE_CELL;
    let slot = if is_cell {
        addr::MOVE_REQUEST_CELL
    } else {
        addr::MOVE_REQUEST_WORLD
    };
    p.set_u32(request + slot, form);
    p.call(addr::PLAYER_REQUEST_MOVE, &[player, request]);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fake::FakeProgram;

    const HOUSE: u32 = 0x0010_3df9;
    const WASTELAND: u32 = 0x000d_a726;
    const OUTSIDE: u32 = 0x000d_e0f1;

    /// An interior is its own space; an exterior cell's space is its worldspace.
    #[test]
    fn the_space_of_an_interior_and_of_an_exterior() {
        let mut p = FakeProgram::new();
        assert_eq!(local(&mut p), None, "no player");
        let player = p.add_player([1.0, 2.0, 3.0]);
        assert_eq!(local(&mut p), None, "a player in no cell");
        let house = p.add_cell(HOUSE, addr::cell_state::ATTACHED);
        let world = p.add_world(WASTELAND);
        let outside = p.add_exterior(OUTSIDE, world, addr::cell_state::ATTACHED);
        p.set_u32(player + addr::REF_CELL, house);
        let me = local(&mut p).unwrap();
        assert_eq!(
            (me.space, me.cell, me.place),
            (HOUSE, HOUSE, [1.0, 2.0, 3.0])
        );
        assert!(!me.loaded);
        p.set_u32(player + addr::REF_NODE, 0x3000_0000);
        p.set_u32(player + addr::REF_CELL, outside);
        let me = local(&mut p).unwrap();
        assert_eq!((me.space, me.cell), (WASTELAND, OUTSIDE));
        assert!(me.loaded);
    }

    /// The guest asks the game to move its player into the host's space: an interior by its
    /// cell, an exterior by its worldspace and place. The game moves the player in its frame.
    /// A busy player and an unknown space are not asked for. Mutation twin: a move that the
    /// game did not carry out leaves the player where it was.
    #[test]
    fn a_guest_is_moved_by_the_games_own_request() {
        let mut p = FakeProgram::new();
        let player = p.add_player([0.0; 3]);
        let house = p.add_cell(HOUSE, addr::cell_state::ATTACHED);
        let world = p.add_world(WASTELAND);
        p.add_exterior(OUTSIDE, world, addr::cell_state::ATTACHED);
        p.set_u32(player + addr::REF_CELL, house);

        assert_eq!(
            request_move(&mut p, 0xbad, [0.0; 3], 0.0),
            Err(Refusal::NoSuchSpace(0xbad))
        );
        p.player_busy = true;
        assert_eq!(
            request_move(&mut p, WASTELAND, [0.0; 3], 0.0),
            Err(Refusal::Busy)
        );
        p.player_busy = false;
        request_move(&mut p, WASTELAND, [-80.0, 10.0, 7000.0], 1.5).unwrap();
        assert_eq!(
            local(&mut p).unwrap().space,
            HOUSE,
            "the twin: not carried out"
        );
        assert!(p.carry_out_move());
        let me = local(&mut p).unwrap();
        assert_eq!(
            (me.space, me.place, me.heading),
            (WASTELAND, [-80.0, 10.0, 7000.0], 1.5)
        );

        request_move(&mut p, HOUSE, [5.0, 6.0, 7.0], 0.0).unwrap();
        assert!(p.carry_out_move());
        assert_eq!(local(&mut p).unwrap().space, HOUSE);
        assert!(!p.carry_out_move(), "one request, one move");
        let mut none = FakeProgram::new();
        assert_eq!(
            request_move(&mut none, HOUSE, [0.0; 3], 0.0),
            Err(Refusal::NoPlayer)
        );
    }
}

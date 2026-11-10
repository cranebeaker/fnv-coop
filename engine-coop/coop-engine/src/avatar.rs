//! The remote player's avatar (SPEC-06): another machine's player as an actor of this game.
//!
//! The avatar is a plain reference made from a temporary copy of the player's base form. It
//! is restrained, with its AI on: the game's own mover walks it from the remote player's
//! movement flags, so the game selects the animation. The adapter commands the heading and
//! the place each frame.
//!
//! The adapter keeps the avatar's address and its form id. The game can destroy the object
//! (a load, a cell change): the address is used only while the id still answers it.

use crate::addr;
use crate::program::Program;

/// How a remote player looks, as far as the base form says it.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Look {
    /// A female body.
    pub female: bool,
    /// The race's form id (0: the base form's own race).
    pub race: u32,
}

/// The base form of one remote player: a temporary copy of the player's base form.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Base {
    /// The copy's address.
    pub at: u32,
}

/// What the adapter commands for one frame.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Drive {
    /// The place: X, Y and Z.
    pub place: [f32; 3],
    /// The heading, in degrees.
    pub heading: f32,
    /// The movement flags ([`addr::move_flag`]). 0: the remote player stands.
    pub flags: u32,
    /// Command Z too. Without it the game keeps the avatar on the ground.
    pub with_z: bool,
}

/// What an avatar is now, read from the program.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Seen {
    /// The place.
    pub place: [f32; 3],
    /// The heading, in degrees.
    pub heading: f32,
    /// The cell's address (0: no cell).
    pub cell: u32,
    /// The life state ([`addr::life_state`]).
    pub life: u32,
    /// The movement animation group that plays (the high half of the animation data's group
    /// word: 3 forward, 7 fast forward, 0x1003 sneak forward). 0xFFFF: none.
    pub group: u32,
    /// The mover's flag override.
    pub flags: u32,
}

/// Why an avatar cannot be used.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gone {
    /// The program has no player yet: there is no anchor to make a reference at.
    NoPlayer,
    /// The program did not make the base copy or the reference.
    NotMade,
    /// The avatar's form id does not answer its address any more: the game destroyed it.
    Destroyed,
    /// The avatar is dead by the game's own code (life state 1 or 2).
    Dead,
}

/// One avatar in this game.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Avatar {
    /// The reference's address.
    pub at: u32,
    /// The reference's form id.
    pub id: u32,
}

impl Base {
    /// Makes the base form for a remote player. `None` when the program has no player base
    /// or makes no copy.
    pub fn make(p: &mut dyn Program, look: Look) -> Option<Base> {
        let source = p.call(addr::FORM_BY_ID, &[addr::PLAYER_BASE_ID]).eax;
        if source == 0 {
            return None;
        }
        let copy = p.call(addr::NPC_COPY, &[source, 0, 0]).eax;
        if copy == 0 {
            return None;
        }
        p.call(
            addr::NPC_SET_SEX,
            &[copy + addr::NPC_SEX_DATA, 1, u32::from(look.female), 1],
        );
        if look.race != 0 {
            let race = p.call(addr::FORM_BY_ID, &[look.race]).eax;
            if race != 0 {
                p.call(addr::NPC_SET_RACE, &[copy, race, 0]);
            }
        }
        // The copy is not a form of the save: the temporary mark is the game's own way.
        p.call(addr::FORM_SET_TEMPORARY, &[copy]);
        Some(Base { at: copy })
    }
}

impl Avatar {
    /// Makes the avatar at the local player, restrains it, and commands its first place.
    pub fn make(p: &mut dyn Program, base: Base, first: &Drive) -> Result<Avatar, Gone> {
        let anchor = p.u32(addr::PLAYER);
        if anchor == 0 {
            return Err(Gone::NoPlayer);
        }
        let one = 1.0f32.to_bits();
        let at = p
            .call(addr::PLACE_AT_ME, &[anchor, base.at, 1, 0, 0, one])
            .eax;
        if at == 0 {
            return Err(Gone::NotMade);
        }
        let avatar = Avatar {
            at,
            id: p.u32(at + addr::FORM_ID),
        };
        p.call(addr::ACTOR_RESTRAIN, &[at, 1]);
        // The first place is exact: Z too.
        let first = Drive {
            with_z: true,
            ..*first
        };
        avatar.drive(p, &first)?;
        Ok(avatar)
    }

    /// The avatar's address, while its form id still answers it and it lives.
    fn checked(&self, p: &mut dyn Program) -> Result<u32, Gone> {
        if p.call(addr::FORM_BY_ID, &[self.id]).eax != self.at {
            return Err(Gone::Destroyed);
        }
        match p.u32(self.at + addr::ACTOR_LIFE_STATE) {
            addr::life_state::DYING | addr::life_state::DEAD => Err(Gone::Dead),
            _ => Ok(self.at),
        }
    }

    /// Commands one frame: the hold, the heading, the movement flags and the place.
    pub fn drive(&self, p: &mut dyn Program, d: &Drive) -> Result<(), Gone> {
        let at = self.checked(p)?;
        // The game can take the restraint away (a menu, a reset of the actor's process).
        if p.u32(at + addr::ACTOR_LIFE_STATE) != addr::life_state::RESTRAINED {
            p.call(addr::ACTOR_RESTRAIN, &[at, 1]);
        }
        p.call(
            addr::REF_SET_ANGLE,
            &[at, u32::from(b'Z'), d.heading.to_bits()],
        );
        let mover = p.u32(at + addr::ACTOR_MOVER);
        if mover != 0 {
            if d.flags == 0 {
                p.call(addr::MOVER_CLEAR_FLAGS, &[mover]);
            } else {
                p.call(addr::MOVER_SET_FLAGS, &[mover, d.flags]);
            }
        }
        let axes: &[(u8, f32)] = &[(b'X', d.place[0]), (b'Y', d.place[1]), (b'Z', d.place[2])];
        let count = if d.with_z { 3 } else { 2 };
        for (axis, value) in &axes[..count] {
            p.call(addr::REF_SET_POS, &[at, u32::from(*axis), value.to_bits()]);
        }
        Ok(())
    }

    /// What the avatar is now.
    pub fn seen(&self, p: &mut dyn Program) -> Result<Seen, Gone> {
        if p.call(addr::FORM_BY_ID, &[self.id]).eax != self.at {
            return Err(Gone::Destroyed);
        }
        let at = self.at;
        let mover = p.u32(at + addr::ACTOR_MOVER);
        let animation = p.call(addr::ACTOR_ANIM_DATA, &[at]).eax;
        let group = match animation {
            0 => 0xffff,
            data => p.u32(data + addr::ANIM_GROUP) >> 16,
        };
        Ok(Seen {
            place: [0, 4, 8].map(|o| p.f32(at + addr::REF_POSITION + o)),
            heading: p.f32(at + addr::REF_ANGLE + 8).to_degrees(),
            cell: p.u32(at + addr::REF_CELL),
            life: p.u32(at + addr::ACTOR_LIFE_STATE),
            group,
            flags: match mover {
                0 => 0,
                mover => p.u32(mover + addr::MOVER_FLAGS),
            },
        })
    }

    /// How the avatar's base looks to the game's own tests: a female body, and the race
    /// `race` (a form id).
    pub fn look(&self, p: &mut dyn Program, race: u32) -> Result<(bool, bool), Gone> {
        if p.call(addr::FORM_BY_ID, &[self.id]).eax != self.at {
            return Err(Gone::Destroyed);
        }
        let female = p.call(addr::ACTOR_IS_FEMALE, &[self.at]).bool();
        let race = p.call(addr::FORM_BY_ID, &[race]).eax;
        let mut is_race = false;
        if race != 0 {
            let answer = p.alloc(8);
            p.write(answer, &[0; 8]);
            p.call(addr::REF_IS_RACE, &[self.at, race, 0, answer]);
            let mut bytes = [0u8; 8];
            p.read(answer, &mut bytes);
            p.free(answer);
            is_race = f64::from_le_bytes(bytes) == 1.0;
        }
        Ok((female, is_race))
    }

    /// Removes the avatar from the game. The game destroys the object in its next frame. An
    /// avatar that the game destroyed already needs no call.
    pub fn remove(self, p: &mut dyn Program) {
        if p.call(addr::FORM_BY_ID, &[self.id]).eax != self.at {
            return;
        }
        p.call(addr::REF_DISABLE, &[self.at, 0]);
        p.call(addr::REF_MARK_FOR_DELETE, &[0, 0, self.at, 0, 0, 0, 0, 0]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fake::FakeProgram;

    fn game() -> FakeProgram {
        let mut p = FakeProgram::new();
        p.add_player([100.0, 200.0, 300.0]);
        p
    }

    fn walk(x: f32) -> Drive {
        Drive {
            place: [x, 1611.5, 7361.0],
            heading: 270.0,
            flags: addr::move_flag::FORWARD | addr::move_flag::WALK,
            with_z: false,
        }
    }

    /// The avatar is made from a marked copy of the player's base, restrained, and placed.
    /// Each frame's command sets the heading, the flags and X and Y: Z is the game's.
    #[test]
    fn an_avatar_is_made_restrained_and_follows_its_commands() {
        let mut p = game();
        let look = Look {
            female: true,
            race: 0,
        };
        let base = Base::make(&mut p, look).expect("a base copy");
        assert!(p.is_temporary(base.at), "the copy is no form of the save");
        assert!(p.is_female(base.at));

        let avatar = Avatar::make(&mut p, base, &walk(2300.0)).expect("an avatar");
        assert!(!p.is_temporary(avatar.at), "the reference is plain");
        let seen = avatar.seen(&mut p).unwrap();
        assert_eq!(seen.life, addr::life_state::RESTRAINED);
        assert_eq!(
            seen.place,
            [2300.0, 1611.5, 7361.0],
            "the first place has Z"
        );
        assert_eq!(seen.flags, 0x101);
        assert_eq!(seen.group, 3, "the game selects the forward group");
        assert!((seen.heading - 270.0).abs() < 0.01, "{}", seen.heading);

        // The game moved Z (the ground). A command without Z leaves it.
        p.set_f32(avatar.at + addr::REF_POSITION + 8, 7300.0);
        avatar.drive(&mut p, &walk(2298.6)).unwrap();
        let seen = avatar.seen(&mut p).unwrap();
        assert_eq!(seen.place, [2298.6, 1611.5, 7300.0]);

        // The remote player stands: the override goes.
        let stand = Drive {
            flags: 0,
            ..walk(2298.6)
        };
        avatar.drive(&mut p, &stand).unwrap();
        let seen = avatar.seen(&mut p).unwrap();
        assert_eq!((seen.flags, seen.group), (0, 0xffff));
    }

    /// The game can take the restraint away. The next command gives it back. Mutation twin:
    /// an avatar that the adapter does not drive stays loose.
    #[test]
    fn a_lost_restraint_comes_back_with_the_next_command() {
        let mut p = game();
        let base = Base::make(&mut p, Look::default()).unwrap();
        let avatar = Avatar::make(&mut p, base, &walk(2300.0)).unwrap();
        p.set_u32(avatar.at + addr::ACTOR_LIFE_STATE, addr::life_state::ALIVE);
        assert_eq!(
            avatar.seen(&mut p).unwrap().life,
            addr::life_state::ALIVE,
            "the twin: no command, no hold"
        );
        avatar.drive(&mut p, &walk(2300.0)).unwrap();
        assert_eq!(
            avatar.seen(&mut p).unwrap().life,
            addr::life_state::RESTRAINED
        );
    }

    /// An address is used only while the form id answers it. After the game destroyed the
    /// avatar, a command writes nothing, and a dead avatar is reported.
    #[test]
    fn a_destroyed_or_dead_avatar_is_not_used() {
        let mut p = game();
        let base = Base::make(&mut p, Look::default()).unwrap();
        let avatar = Avatar::make(&mut p, base, &walk(2300.0)).unwrap();

        p.set_u32(avatar.at + addr::ACTOR_LIFE_STATE, addr::life_state::DEAD);
        assert_eq!(avatar.drive(&mut p, &walk(1.0)), Err(Gone::Dead));
        assert_eq!(avatar.seen(&mut p).unwrap().place[0], 2300.0);

        p.set_u32(
            avatar.at + addr::ACTOR_LIFE_STATE,
            addr::life_state::RESTRAINED,
        );
        avatar.remove(&mut p);
        let calls = p.calls.len();
        assert_eq!(avatar.drive(&mut p, &walk(1.0)), Err(Gone::Destroyed));
        assert_eq!(avatar.seen(&mut p), Err(Gone::Destroyed));
        // Only the two lookups ran: no write to an address that is not the avatar's.
        assert_eq!(p.calls.len(), calls + 2);
        avatar.remove(&mut p);
        assert_eq!(p.calls.len(), calls + 3, "a second removal only looks");
    }

    /// The base copy takes the remote player's sex and race, and the game's own tests on the
    /// avatar say so. Mutation twin: a base that was not written is not female and has
    /// another race.
    #[test]
    fn an_avatar_has_the_remote_players_look() {
        const RACE: u32 = 0x424a;
        let mut p = game();
        p.add_form(RACE);
        let look = Look {
            female: true,
            race: RACE,
        };
        let base = Base::make(&mut p, look).unwrap();
        let avatar = Avatar::make(&mut p, base, &walk(2300.0)).unwrap();
        assert_eq!(avatar.look(&mut p, RACE), Ok((true, true)));
        assert!(p.live.is_empty(), "the answer's memory is given back");

        let plain = Base::make(&mut p, Look::default()).unwrap();
        let twin = Avatar::make(&mut p, plain, &walk(2200.0)).unwrap();
        assert_eq!(twin.look(&mut p, RACE), Ok((false, false)));
        assert_eq!(twin.look(&mut p, 0xbad), Ok((false, false)));
    }

    #[test]
    fn no_player_means_no_avatar() {
        let mut p = FakeProgram::new();
        assert_eq!(Base::make(&mut p, Look::default()), None);
        let base = Base { at: 0x2000_0100 };
        assert_eq!(Avatar::make(&mut p, base, &walk(0.0)), Err(Gone::NoPlayer));
    }
}

/// A local hit on an avatar, for the host's hit ledger (SPEC-07). The local game applied
/// nothing.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Hit {
    /// The remote player whose avatar was hit.
    pub slot: u8,
    /// The form id of the reference that made the hit (0: none).
    pub source: u32,
    /// The weapon's form id (0: none).
    pub weapon: u32,
    /// The health damage that the local game computed.
    pub damage: f32,
    /// The body part (14: none).
    pub location: u32,
    /// The hit's flags (4: a critical hit).
    pub flags: u32,
}

/// The avatars' damage rules: a local hit on an avatar is recorded and not applied, and local
/// code does not kill an avatar. The remote player's own game owns its health.
#[derive(Clone, Debug, Default)]
pub struct Guard {
    /// The avatars in this game: address to slot.
    bodies: std::collections::BTreeMap<u32, u8>,
    /// The same avatars, for a removal: the reference and its slot.
    avatars: Vec<(Avatar, u8)>,
    hits: Vec<Hit>,
    /// The avatars that left the game for a save or a load, each one time.
    left: Vec<(u8, Left)>,
    /// How many kills of an avatar the guard stopped.
    pub kills_stopped: u64,
}

/// Why an avatar left the game by the guard's rule.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Left {
    /// The game saved: the avatar's base form is still good, and the avatar can be made
    /// again in the next gap.
    Save,
    /// The game loaded: each form of the game's own making is gone, the base copy too.
    Load,
}

impl Guard {
    /// The avatars that are in the game now, each with its slot.
    pub fn set_bodies(&mut self, bodies: impl IntoIterator<Item = (Avatar, u8)>) {
        self.avatars = bodies.into_iter().collect();
        self.bodies = self.avatars.iter().map(|(a, slot)| (a.at, *slot)).collect();
    }

    /// The game saves or loads: no save must hold an avatar, and no load must meet one. The
    /// guard forgets each avatar and gives them to the caller, which removes them from the
    /// game before the save or the load goes on.
    pub fn leave(&mut self, why: Left) -> Vec<Avatar> {
        self.bodies.clear();
        let avatars = std::mem::take(&mut self.avatars);
        self.left
            .extend(avatars.iter().map(|(_, slot)| (*slot, why)));
        avatars.into_iter().map(|(a, _)| a).collect()
    }

    /// The avatars that left for a save or a load. The list is empty after.
    pub fn take_left(&mut self) -> Vec<(u8, Left)> {
        std::mem::take(&mut self.left)
    }

    /// The slot of the avatar at this address.
    pub fn slot_of(&self, actor: u32) -> Option<u8> {
        self.bodies.get(&actor).copied()
    }

    /// A hit reached the game's funnel. True: the victim is an avatar, the hit is recorded,
    /// and the game must not apply it.
    pub fn hit(&mut self, p: &dyn Program, victim: u32, data: u32) -> bool {
        let Some(slot) = self.slot_of(victim) else {
            return false;
        };
        let form_id = |at: u32| match at {
            0 => 0,
            at => p.u32(at + addr::FORM_ID),
        };
        self.hits.push(Hit {
            slot,
            source: form_id(p.u32(data + addr::HIT_SOURCE)),
            weapon: form_id(p.u32(data + addr::HIT_WEAPON)),
            damage: p.f32(data + addr::HIT_HEALTH_DAMAGE),
            location: p.u32(data + addr::HIT_LOCATION),
            flags: p.u32(data + addr::HIT_FLAGS),
        });
        true
    }

    /// The game wants to kill an actor. True: it is an avatar, and the game must not.
    pub fn kill(&mut self, actor: u32) -> bool {
        let stop = self.bodies.contains_key(&actor);
        if stop {
            self.kills_stopped += 1;
        }
        stop
    }

    /// The hits that wait for the host. The list is empty after.
    pub fn take_hits(&mut self) -> Vec<Hit> {
        std::mem::take(&mut self.hits)
    }
}

impl Avatar {
    /// Gives the avatar its full health again, with the game's own command. Local damage
    /// that is no weapon hit (a fall, a script) lowers it, and the remote player's own game
    /// owns the true value. False when the game has no console manager yet.
    pub fn heal(&self, p: &mut dyn Program) -> Result<bool, Gone> {
        if p.call(addr::FORM_BY_ID, &[self.id]).eax != self.at {
            return Err(Gone::Destroyed);
        }
        Ok(crate::console::run_line(p, "ResetHealth", self.at))
    }

    /// The avatar's health as a part of 1, by the game's own test.
    pub fn health(&self, p: &mut dyn Program) -> Result<f64, Gone> {
        if p.call(addr::FORM_BY_ID, &[self.id]).eax != self.at {
            return Err(Gone::Destroyed);
        }
        let answer = p.alloc(8);
        p.write(answer, &[0; 8]);
        p.call(addr::REF_HEALTH_PART, &[self.at, 0, 0, answer]);
        let mut bytes = [0u8; 8];
        p.read(answer, &mut bytes);
        p.free(answer);
        Ok(f64::from_le_bytes(bytes))
    }
}

#[cfg(test)]
mod guard_tests {
    use super::*;
    use crate::fake::FakeProgram;

    /// A hit on an avatar is recorded with its source, weapon, damage and place, and the
    /// game must not apply it. A hit on another actor is the game's own. Mutation twin: with
    /// the avatar not in the guard's list, its hit is the game's too.
    #[test]
    fn a_hit_on_an_avatar_is_recorded_and_not_applied() {
        let mut p = FakeProgram::new();
        let player = p.add_player([0.0; 3]);
        let pistol = p.add_form(0x000e_3778);
        let data = 0x3000_0000;
        p.set_u32(data + addr::HIT_SOURCE, player);
        p.set_u32(data + addr::HIT_WEAPON, pistol);
        p.set_f32(data + addr::HIT_HEALTH_DAMAGE, 12.5);
        p.set_u32(data + addr::HIT_LOCATION, 1);
        p.set_u32(data + addr::HIT_FLAGS, 4);

        let mut guard = Guard::default();
        assert!(
            !guard.hit(&p, 0x2222_0000, data),
            "the twin: not in the list"
        );
        let body = Avatar {
            at: 0x2222_0000,
            id: 0xff00_130a,
        };
        guard.set_bodies([(body, 3)]);
        assert!(!guard.hit(&p, 0x2222_0100, data), "another actor");
        assert!(guard.hit(&p, 0x2222_0000, data));
        assert_eq!(
            guard.take_hits(),
            [Hit {
                slot: 3,
                source: 0x14,
                weapon: 0x000e_3778,
                damage: 12.5,
                location: 1,
                flags: 4,
            }]
        );
        assert!(guard.take_hits().is_empty());
    }

    #[test]
    fn local_code_does_not_kill_an_avatar() {
        let body = Avatar {
            at: 0x2222_0000,
            id: 0xff00_130a,
        };
        let mut guard = Guard::default();
        assert!(!guard.kill(0x2222_0000));
        guard.set_bodies([(body, 1)]);
        assert!(guard.kill(0x2222_0000));
        assert!(!guard.kill(0x2222_0100));
        assert_eq!(guard.kills_stopped, 1);
        guard.set_bodies([]);
        assert!(!guard.kill(0x2222_0000), "a removed avatar is no avatar");
    }

    /// At a save or a load each avatar leaves: the guard gives them for the removal, knows
    /// none after, and reports each slot one time with the reason. Mutation twin: with no
    /// avatar in the list, a save removes nothing.
    #[test]
    fn each_avatar_leaves_at_a_save_and_at_a_load() {
        let mut p = FakeProgram::new();
        p.add_player([0.0; 3]);
        let base = Base::make(&mut p, Look::default()).unwrap();
        let first = Drive::default();
        let one = Avatar::make(&mut p, base, &first).unwrap();
        let two = Avatar::make(&mut p, base, &first).unwrap();

        let mut guard = Guard::default();
        assert!(guard.leave(Left::Save).is_empty(), "the twin: no avatar");
        guard.set_bodies([(one, 1), (two, 2)]);
        let leaving = guard.leave(Left::Save);
        assert_eq!(leaving, [one, two]);
        for avatar in leaving {
            avatar.remove(&mut p);
        }
        assert_eq!(one.seen(&mut p), Err(Gone::Destroyed));
        assert_eq!(two.seen(&mut p), Err(Gone::Destroyed));
        assert_eq!(guard.slot_of(one.at), None);
        assert!(!guard.kill(one.at));
        assert_eq!(guard.take_left(), [(1, Left::Save), (2, Left::Save)]);
        assert!(guard.take_left().is_empty());

        guard.set_bodies([(one, 1)]);
        guard.leave(Left::Load);
        assert_eq!(guard.take_left(), [(1, Left::Load)]);
    }
}

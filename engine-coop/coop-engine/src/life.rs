//! Placed actors' life and death, shared between the games.
//!
//! Only actors that both games know by the same id are shared: placed ones from the plugins. An
//! actor that the game makes while playing (ids from `ff000000`) has another id, or none, in the
//! other game. The game's own life state setter marks the actor's change ([`addr::CHANGE_LIFE`]),
//! so the game's map of changed forms names each actor whose life changed.

use std::collections::BTreeMap;

use crate::program::Program;
use crate::{addr, changes};

/// Ids from here up are made while playing, and they differ between two games.
pub const FIRST_DYNAMIC_ID: u32 = 0xff00_0000;
/// The player's reference.
pub const PLAYER_REF_ID: u32 = 0x14;

/// True for a placed actor's form type (an NPC's or a creature's reference).
pub fn is_actor_type(kind: u8) -> bool {
    kind == addr::FORM_TYPE_NPC_REF || kind == addr::FORM_TYPE_CREATURE_REF
}

/// True when the two games can mean the same actor by `id`: a placed actor, not the player.
pub fn shareable(id: u32) -> bool {
    id != 0 && id != PLAYER_REF_ID && id < FIRST_DYNAMIC_ID
}

/// The address of the placed actor with id `id` in this game now. `None`: no such form here, or
/// not an actor.
pub fn actor(p: &mut dyn Program, id: u32) -> Option<u32> {
    let at = p.call(addr::FORM_BY_ID, &[id]).eax;
    if at == 0 {
        return None;
    }
    is_actor_type(p.call(addr::FORM_TYPE, &[at]).eax as u8).then_some(at)
}

/// True when actor `id` is dying or dead. `None`: it is not in this game now.
pub fn dead(p: &mut dyn Program, id: u32) -> Option<bool> {
    let at = actor(p, id)?;
    Some(matches!(
        p.u32(at + addr::ACTOR_LIFE_STATE),
        addr::life_state::DYING | addr::life_state::DEAD
    ))
}

/// Kills actor `id` with the game's own kill, with no killer (nobody here is credited). False
/// when the actor is not in this game now. The game may keep it alive (an essential actor), or
/// do the kill later in its frame: read [`dead`] after.
pub fn kill(p: &mut dyn Program, id: u32) -> bool {
    let Some(at) = actor(p, id) else {
        return false;
    };
    p.call(addr::ACTOR_KILL, &[at, 0, 0f32.to_bits()]);
    true
}

/// The shareable actors whose life changed, from the game's map of changed forms. A form keeps
/// its flag after its first change, so the watch keeps each actor's last state and reports a
/// change of it.
#[derive(Clone, Debug, Default)]
pub struct LifeWatch {
    last: BTreeMap<u32, bool>,
}

impl LifeWatch {
    /// Each shareable actor of the change map whose life flag is set and whose state is new
    /// since the last look: (id, dead). An actor seen the first time is reported only dead.
    pub fn look(&mut self, p: &mut dyn Program) -> Vec<(u32, bool)> {
        let candidates: Vec<u32> = changes::walk(p)
            .into_iter()
            .filter(|e| e.flags & addr::CHANGE_LIFE != 0 && shareable(e.form))
            .map(|e| e.form)
            .collect();
        let mut out = Vec::new();
        for id in candidates {
            let Some(now) = dead(p, id) else {
                continue;
            };
            let before = self.last.insert(id, now);
            if before.map_or(now, |was| was != now) {
                out.push((id, now));
            }
        }
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fake::FakeProgram;

    const SUNNY: u32 = 0x0010_4e84;
    const GECKO: u32 = 0x0010_9f3a;

    fn game() -> FakeProgram {
        let mut p = FakeProgram::new();
        p.add_change_map(16);
        p.add_actor(SUNNY, addr::FORM_TYPE_NPC_REF);
        p.add_actor(GECKO, addr::FORM_TYPE_CREATURE_REF);
        p
    }

    #[test]
    fn only_placed_actors_are_shared() {
        assert!(shareable(SUNNY));
        assert!(!shareable(PLAYER_REF_ID));
        assert!(!shareable(0));
        assert!(!shareable(0xff00_0812));
        let mut p = game();
        p.add_form(0x0001_0000);
        assert_eq!(dead(&mut p, 0x0001_0000), None, "not an actor");
        assert_eq!(dead(&mut p, 0xbad), None, "not here");
        assert_eq!(dead(&mut p, SUNNY), Some(false));
        assert!(!kill(&mut p, 0xbad));
    }

    /// The game's kill makes an actor dead and marks its life change, and the watch reports
    /// it once. Mutation twin: an actor whose life did not change is not reported.
    #[test]
    fn a_death_is_found_in_the_change_map_once() {
        let mut p = game();
        let mut watch = LifeWatch::default();
        assert!(watch.look(&mut p).is_empty(), "the twin: nobody died");
        assert!(kill(&mut p, GECKO));
        assert_eq!(dead(&mut p, GECKO), Some(true));
        assert_eq!(watch.look(&mut p), [(GECKO, true)]);
        assert!(watch.look(&mut p).is_empty(), "once");
        // An actor alive again is a change too.
        let at = p.call(addr::FORM_BY_ID, &[GECKO]).eax;
        p.set_u32(at + addr::ACTOR_LIFE_STATE, addr::life_state::ALIVE);
        assert_eq!(watch.look(&mut p), [(GECKO, false)]);
        // A form that the change map names with another flag is not looked at.
        p.mark_changed(SUNNY, 0x2);
        assert!(watch.look(&mut p).is_empty());
    }
}

//! Doors and gates that swing where they stand, as the game's own code
//! does them (`world::doors`, `%USERPROFILE%\nv-re\findings\doors.md`).
//!
//! The rules live in `world::doors`; this file plays them on screen: a
//! door activated (E, a script's `Activate` or `SetOpenState`, a person on
//! a path) starts its model's `Open` or `Close` sequence from its start
//! (`0047a560`), its leaf pieces follow the sequence (`move_pieces`), the
//! sequence's `sound:` text keys play as its clock passes them
//! (`004eef00`), and when it ends (`00567050`) the door's `OnOpen` /
//! `OnClose` block runs and the leaf's collision is put where the leaf
//! now is. With the INI's `bAnimateDoorPhysics` on, the collision follows
//! the leaf every frame instead; it's off in the shipped `Fallout_default.ini`
//! (`0047ab40`: the Havok bodies are only re-placed when the sequence
//! finishes), so a swinging leaf doesn't push anyone.

use std::collections::HashMap;

use bevy::prelude::*;
use cellview::SwingDoor;
use esm::FormId;
use world::doors::{Activated, OpenState};
use world::scripting::{GameState, Runner};

use crate::dialogue::DialogueState;
use crate::sounds::SoundRequests;
use crate::walk::CellCollision;
use crate::GameFiles;

/// The doors here that open where they stand, by placed reference.
#[derive(Resource, Default)]
pub struct SwingDoors {
    pub doors: HashMap<u32, SwingDoor>,
    /// The collider was rebuilt: every leaf is put back where its door
    /// has it.
    pub dirty: bool,
    /// Per door: the motion whose text keys are being played (its start)
    /// and how many have fired.
    fired: HashMap<u32, (f64, usize)>,
    /// The pose each door's collision was last moved to.
    applied: HashMap<u32, (bool, f32)>,
    /// `[General] bAnimateDoorPhysics` (0 in `Fallout_default.ini`): the
    /// leaf's collision follows the swing frame by frame.
    pub animate_physics: bool,
}

/// Where each door's leaf is this frame: whether its `Open` (true) or
/// `Close` sequence shows, and the moment of it (seconds from its start),
/// for the door's pieces (`move_pieces`).
#[derive(Resource, Default)]
pub struct DoorPoses(pub HashMap<u32, (bool, f32)>);

impl SwingDoors {
    /// Takes the doors of a place (an interior, or the loaded outdoor
    /// squares together), telling the state their models' sequence lengths
    /// (`GameState::door_lengths`).
    pub fn replace<'a>(
        &mut self,
        doors: impl Iterator<Item = &'a SwingDoor>,
        state: &mut GameState,
    ) {
        self.doors.clear();
        for door in doors {
            if let Some(lengths) = door.lengths() {
                state.door_lengths.insert(door.base, lengths);
            }
            self.doors.insert(door.reference.0, door.clone());
        }
        self.dirty = true;
    }

    /// Reads the INI setting the physics follow.
    pub fn read_settings(&mut self, settings: &assets::IniSettings) {
        self.animate_physics = settings
            .get("General", "bAnimateDoorPhysics")
            .is_some_and(|v| v.trim() != "0");
    }
}

/// Activates a door for someone (E, a script, a person on a path): the
/// swing starts (the state keeps it) and its sound plays
/// (`world::doors::activate`, `0047a560`).
pub fn activate(
    order: &esm::LoadOrder,
    state: &mut GameState,
    sounds: &mut SoundRequests,
    door: FormId,
    by: Option<FormId>,
) -> Activated {
    let activated = world::doors::activate(order, state, door, by);
    if let Some(sound) = world::doors::sound(order, door, activated) {
        sounds.0.push(sound);
    }
    activated
}

/// What E on a door says.
pub fn prompt(order: &esm::LoadOrder, state: &GameState, door: FormId) -> &'static str {
    if world::doors::open_state(order, state, door).is_open() {
        "E) Close door"
    } else {
        "E) Open door"
    }
}

/// Every frame, after the scripts: finished swings run their `OnOpen` /
/// `OnClose` blocks (`0047ac70`, event flags 0x10000 / 0x20000), each
/// door's leaf pose is worked out for its pieces, its collision moved as
/// the setting says, and the playing sequence's `sound:` text keys are
/// played as its clock passes them (`004eef00`: a key fires on the first
/// frame whose time is past it).
pub fn update_doors(
    game: Res<GameFiles>,
    scripts: Res<crate::scripts::Scripts>,
    mut state: ResMut<DialogueState>,
    mut doors: ResMut<SwingDoors>,
    mut poses: ResMut<DoorPoses>,
    mut collision: ResMut<CellCollision>,
    mut sounds: ResMut<SoundRequests>,
) {
    let order = &game.0.order;
    let state = &mut state.0;
    // Swings that have just ended.
    let finished = world::doors::finished(state);
    for (door, motion) in &finished {
        if motion.length <= 0.0 {
            // A model without sequences just flips (`0047a560`'s other
            // branch): no finished sequence, no block.
            continue;
        }
        let kind = if motion.opening { "onopen" } else { "onclose" };
        let by = motion.by.unwrap_or(FormId(0));
        Runner::new(order, &scripts.0, state).run_event(*door, kind, by);
    }
    let dirty = std::mem::take(&mut doors.dirty);
    let now = state.seconds;
    let animate = doors.animate_physics;
    let doors = &mut *doors;
    let mut new_poses = HashMap::with_capacity(doors.doors.len());
    for (&reference, door) in &doors.doors {
        let id = FormId(reference);
        let (opening, at) = world::doors::pose(order, state, id, now);
        new_poses.insert(reference, (opening, at));
        let moving = world::doors::open_state(order, state, id).is_moving();
        // Text keys of the playing sequence, as its clock passes them.
        if let Some(motion) = state.door_motion.get(&id).filter(|_| moving) {
            let entry = doors.fired.entry(reference).or_insert((motion.started, 0));
            if entry.0 != motion.started {
                *entry = (motion.started, 0);
            }
            if let Some((sequence, _)) = door.sequence_at(opening, at) {
                let elapsed = (now - motion.started) as f32;
                while let Some((time, text)) = sequence.text_keys.get(entry.1) {
                    if *time >= elapsed {
                        break;
                    }
                    entry.1 += 1;
                    if let Some(sound) = text_key_sound(order, text) {
                        sounds.0.push(sound);
                    }
                }
            }
        }
        // The collision: with the swing when the setting says so, else
        // once it has ended (and whenever the collider was rebuilt).
        let follow = animate || !moving;
        let applied = doors.applied.get(&reference).copied();
        if (dirty || applied != Some((opening, at))) && follow {
            let layer = door.sequence_at(opening, at);
            if let Some((rotation, translation)) = door.leaf_move(layer) {
                collision.0.move_owner(reference, &rotation, translation);
            }
            doors.applied.insert(reference, (opening, at));
        }
    }
    poses.0 = new_poses;
}

/// The sound a `sound:` text key names (`004eef00`: the word after
/// `Sound: `, looked up by editor ID), if it's one.
pub(crate) fn text_key_sound(order: &esm::LoadOrder, text: &str) -> Option<FormId> {
    let rest = text
        .get(..6)
        .filter(|head| head.eq_ignore_ascii_case("sound:"))
        .map(|_| text[6..].trim_start())?;
    let name = rest.split_whitespace().next()?;
    order.form_by_editor_id(name)
}

/// The door a walker's path is about to go through, as the game's path
/// following opens doors (`009e20c0`): a closed one is activated by the
/// walker when they reach its portal, and they wait while it opens;
/// anything else (open, opening by someone else, closing) is walked on
/// through. Whether they should wait this frame.
pub fn walker_at_door(
    order: &esm::LoadOrder,
    state: &mut GameState,
    sounds: &mut SoundRequests,
    walker: FormId,
    position: [f32; 3],
    doors_ahead: &mut Vec<(FormId, [f32; 3])>,
) -> bool {
    let Some(&(door, portal)) = doors_ahead.first() else {
        return false;
    };
    let dx = portal[0] - position[0];
    let dy = portal[1] - position[1];
    if dx * dx + dy * dy > PORTAL_REACH * PORTAL_REACH {
        return false;
    }
    match world::doors::open_state(order, state, door) {
        // Locked: the walker's activation fails (`009e20c0` → `00573170`;
        // whether they carry its key isn't asked): no opening, and the
        // closed door stops them (their walk gets stuck and fails).
        OpenState::Closed if world::locks::lock_now(order, state, door).is_some() => {
            doors_ahead.remove(0);
            false
        }
        OpenState::Closed => {
            activate(order, state, sounds, door, Some(walker));
            true
        }
        OpenState::Opening => true,
        _ => {
            doors_ahead.remove(0);
            false
        }
    }
}

/// How near a door's portal triangle someone must be to use the door: the
/// portal is the navmesh triangle across the doorway, so about a
/// triangle's reach (a guess; the game acts when its path segment is the
/// portal's).
const PORTAL_REACH: f32 = 128.0;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn text_keys_name_sounds_after_sound_colon() {
        // Only the lookup needs the load order; the parsing is checked
        // through what's asked for.
        fn word(t: &str) -> Option<String> {
            t.get(..6)
                .filter(|h| h.eq_ignore_ascii_case("sound:"))
                .map(|_| t[6..].split_whitespace().next().unwrap_or("").to_string())
        }
        assert_eq!(
            word("sound: DRSFencePicketOpen").as_deref(),
            Some("DRSFencePicketOpen")
        );
        assert_eq!(word("Sound: DRSWoodOpen 1").as_deref(), Some("DRSWoodOpen"));
        assert_eq!(word("start"), None);
        assert_eq!(word("end"), None);
    }
}

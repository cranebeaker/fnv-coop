//! What an actor's body plays each frame, as `Actor::PickAnimations`
//! (Xbox PDB, `00895110`) picks it: the movement group for the mover's
//! flags with the weapon kind and movement kind (sneaking) the actor's
//! state gives, looked up with the game's fallbacks
//! ([`super::groups::AnimSet::lookup`]); the walk's rate; drawing and
//! putting away the weapon (the `Equip` and `Unequip` groups, the weapon
//! going to the hand or back at their `Attach`/`Detach` key); the aim while
//! the weapon is drawn (`00888070` → `008b28c0`), the aim cross-fading in
//! over an attack or a draw that ends (`004994f0`); the base idle of the
//! kinds (`sneakmtidle.kf`, `h2hidle.kf`); an idle the idle tree plays in
//! the movement or base section holding it (the anim action 0xd that
//! `00496fe0` sets).
//!
//! And the rules that feed it: whether a non-player sneaks (`00888b50`),
//! wants the weapon out (`008eeec0`, the combat's equip action
//! `009da7c0`), and which direction group a walk facing a point plays
//! (`DetailedActorPathHandler`, Xbox PDB, `009e2aa0`).
//!
//! Read from FalloutNV.exe 1.4.0.525. Not here: jumping (`JumpStart` …),
//! swimming, the weapon up/down sections (their weights come from the aim
//! pitch, `009295c0`, not traced), the attack-speed perk entry (`005e58f0`)
//! and the first-person player's own paths.

use std::sync::Arc;

use nif::anim::{Bone, Sequence};

use super::groups::{self, group_of, id, movement_kind_of, AnimSet};
use super::{group, section, section_of, MoveFlags, Player, State, GROUPS};

/// The animations an actor's 3D has, and their sequences.
pub trait Library {
    /// The files by group id.
    fn set(&self) -> &AnimSet;
    /// The sequence of an exact id (one of its files: several are drawn
    /// from at random, `0048f450`).
    fn sequence(&mut self, id: u16) -> Option<Arc<Sequence>>;
}

/// The process's animation action (`GetAnimAction`, Xbox PDB, process
/// vtable +0x3e4) as far as these groups go: drawing the weapon (0),
/// putting it away (1), an attack (2), a reload (0x11), and an idle that
/// holds the base or movement section (0xd, `00496fe0`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action {
    Equip,
    Unequip,
    Attack,
    Reload,
}

/// What the actor is doing this frame, as `PickAnimations` reads it.
#[derive(Debug, Clone, Copy, Default)]
pub struct Frame<'a> {
    /// The mover's direction, turn and run flags (`008846e0`).
    pub flags: MoveFlags,
    /// Sneaking (movement flag 0x400 without 0x800, `004997b0`).
    pub sneaking: bool,
    /// The mover's speed (`00884dc0` walking, `00884eb0` running), units
    /// a second before the actor's scale.
    pub speed: f32,
    /// The turn animations' rate (the caller's turn scale, `00888070`:
    /// `fAITurnSpeedScale` 1.5, in combat `fAICombatTurnSpeedScale` 2.5).
    pub turn_rate: f32,
    /// The process wants the weapon out (`GetWantWeaponDrawn`, Xbox PDB,
    /// `008a6970`).
    pub want_drawn: bool,
    /// In combat (actor +0x104, `bInCombat`, Xbox PDB).
    pub in_combat: bool,
    /// A character, not a creature (actor vfunc +0x390).
    pub character: bool,
    /// The weapon kind of the weapon in hand (`0118a838` of its animation
    /// type); none without a weapon (fists play the unarmed kind, 1).
    pub weapon_kind: Option<u8>,
    /// Power armour worn (`008ba3e0`/`008ba410`: the process's
    /// `HasBackPackWorn`/`IsPowerBodyArmorWorn`, Xbox PDB).
    pub power_armor: bool,
    /// Free to draw or put away (`008843a0`: not knocked out, dead,
    /// paralysed …).
    pub can_act: bool,
    /// In furniture (sit state other than 0): no drawing.
    pub seated: bool,
    /// The furniture's loop, played as `DynamicIdle` in the base section
    /// while seated (sit states 3–5 and 8–10 with a marker under 21).
    pub dynamic_idle: Option<&'a Arc<Sequence>>,
    pub dead: bool,
}

/// One actor's picking state: the group ids the sections were last given,
/// the action under way and how far its keys are, whether the weapon is
/// drawn, and attack or reload requests.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Picker {
    /// The id each section plays (`Animation` +0x4c, Xbox PDB `group`).
    ids: [u16; 8],
    /// The action under way, and the group it plays in the weapon section.
    pub action: Option<Action>,
    action_group: u8,
    /// The equip's or unequip's attach key passed (`Animation` +0x5c,
    /// `action`, at 1).
    attached: bool,
    /// The weapon is drawn (`GetWeaponDrawn`, Xbox PDB).
    pub drawn: bool,
    /// A section an idle holds (anim action 0xd).
    pub idle_section: Option<u8>,
    /// An attack or reload group asked for, to play next frame.
    pending: Option<(Action, u8)>,
}

impl Picker {
    /// A picker for someone whose weapon is drawn or not.
    pub fn new(drawn: bool) -> Picker {
        Picker {
            ids: [groups::NONE; 8],
            drawn,
            ..Default::default()
        }
    }

    /// The id last played in a section.
    pub fn id(&self, section: u8) -> u16 {
        self.ids[usize::from(super::slot(section)).min(7)]
    }

    /// An attack (its group: `AttackRight` …) to play over the aim.
    pub fn attack(&mut self, group: u8) {
        self.pending = Some((Action::Attack, group));
    }

    /// A reload (`ReloadA` … `ReloadZ`) to play.
    pub fn reload(&mut self, group: u8) {
        self.pending = Some((Action::Reload, group));
    }

    /// An idle the tree gave was played in a section: in the base or
    /// movement section it holds it from the walk (`00497f20` makes such
    /// an idle type 3, `00496fe0` sets the anim action 0xd).
    pub fn idle_played(&mut self, section: u8) {
        let s = super::slot(section);
        self.idle_section = (s <= section::MOVEMENT).then_some(s);
    }

    /// The weapon kind and movement kind for the group ids this frame
    /// (as `00895110` works them out).
    fn kinds(&self, f: &Frame, starting: bool) -> (u8, u8) {
        let weapon = if (!f.character && (f.want_drawn || f.in_combat)) || self.drawn || starting {
            f.weapon_kind.unwrap_or(1)
        } else {
            0
        };
        let movement = if f.sneaking {
            groups::movement_kind::SNEAK
        } else {
            groups::movement_kind::NORMAL
        };
        (weapon, movement)
    }

    /// One frame's picking (`00895110`, then `00888070`'s aim for a drawn
    /// weapon with nothing in the weapon section).
    // Translated from 00895110 and 00888070 (decompiled, FalloutNV.exe
    // 1.4.0.525)
    pub fn pick(&mut self, player: &mut Player, lib: &mut impl Library, f: &Frame, bones: &[Bone]) {
        // The action's sequence over: no action (`008a73e0(-1)`).
        if self.action.is_some() && player.playing(section::WEAPON) != Some(self.action_group) {
            self.action = None;
        }
        if let Some(s) = self.idle_section {
            if player.playing(s) != Some(group::SPECIAL_IDLE) {
                self.idle_section = None;
            }
        }
        // Sneaking began or ended: the weapon section's group, if another
        // one plays for it now, is stopped (the top of `00895110`).
        if self.action.is_none() && player.state(section::WEAPON) == Some(State::Animating) {
            let current = self.ids[4];
            let sneaking = movement_kind_of(current) == groups::movement_kind::SNEAK;
            if current != groups::NONE && sneaking != f.sneaking {
                let (weapon, movement) = self.kinds(f, false);
                let again =
                    lib.set()
                        .lookup(id(movement, weapon, group_of(current), f.power_armor));
                if again != current {
                    player.stop_section(section::WEAPON);
                }
            }
        }
        // Drawing or putting away: the weapon to the hand or back once the
        // attach key is passed (cases 0 and 1).
        if matches!(self.action, Some(Action::Equip | Action::Unequip)) {
            if !self.attached {
                let past = match (player.data(section::WEAPON), player.time(section::WEAPON)) {
                    (Some(d), Some(t)) => d.start + t > d.attach,
                    _ => false,
                };
                self.attached = past;
            }
            let equipping = self.action == Some(Action::Equip);
            if self.attached && self.drawn != equipping {
                self.drawn = equipping;
            }
        }
        let mut started = None;
        let mut chosen = group::IDLE;
        if self.action.is_none() && f.can_act && !f.dead {
            if f.want_drawn && !self.drawn && !f.seated {
                chosen = group::EQUIP;
                started = Some(Action::Equip);
            } else if !f.want_drawn && self.drawn {
                chosen = group::UNEQUIP;
                started = Some(Action::Unequip);
            }
        }
        let (weapon, movement) = self.kinds(f, started.is_some());
        let mut speed = 0.0;
        if chosen == group::IDLE {
            if f.dynamic_idle.is_some() {
                chosen = group::DYNAMIC_IDLE;
            }
            let m = f.flags;
            let dirs = [m.forward, m.backward, m.left, m.right];
            match dirs.iter().position(|&d| d) {
                None if m.turn_left => chosen = group::TURN_LEFT,
                None if m.turn_right => chosen = group::TURN_RIGHT,
                None => {}
                Some(d) => {
                    let base = if m.running {
                        group::FAST_FORWARD
                    } else {
                        group::FORWARD
                    };
                    chosen = base + d as u8;
                    speed = f.speed;
                }
            }
        }
        // Slower than 1 a second: the idle.
        if speed < 1.0 && (group::FORWARD..=group::FAST_RIGHT).contains(&chosen) {
            chosen = group::IDLE;
        }
        // The seat's loop: not looked up (the furniture's own idle).
        let looked = if chosen == group::DYNAMIC_IDLE {
            u16::from(group::DYNAMIC_IDLE)
        } else {
            lib.set()
                .lookup(id(movement, weapon, chosen, f.power_armor))
        };
        let g = group_of(looked);
        // The lookup fell back from a draw or putting away: none starts
        // (putting away with no file for it puts the weapon away at once).
        if let Some(a) = started {
            if chosen != g {
                if a == Action::Unequip {
                    self.drawn = false;
                }
                started = None;
            }
        }
        let sec = section_of(g);
        // An idle holding this section: it goes on.
        let held = self.idle_section == Some(sec);
        if !held {
            self.rate(player, lib, f, looked, speed);
            self.play(player, lib, f, (looked, sec), (movement, weapon), bones);
            if let Some(a) = started {
                if self.ids[usize::from(sec)] == looked && !matches!(g, 0 | 1 | 0xaa) {
                    self.action = Some(a);
                    self.action_group = g;
                    self.attached = false;
                }
            }
        }
        // A non-movement group chosen: the walk eases out, unless an idle
        // holds the movement section or the walk is still blending in.
        let table_section = GROUPS.get(usize::from(g)).map_or(0, |e| e.1);
        if player.playing(section::MOVEMENT).is_some()
            && table_section != section::MOVEMENT
            && self.idle_section != Some(section::MOVEMENT)
            && player.state(section::MOVEMENT) == Some(State::Animating)
        {
            player.stop_section(section::MOVEMENT);
            self.ids[1] = groups::NONE;
        }
        // An attack or reload asked for: over the aim, the weapon drawn.
        if let Some((action, group)) = self.pending.take() {
            if self.drawn
                && self.action.is_none()
                && !f.dead
                && self.play_weapon(player, lib, f, group, 0, bones)
            {
                self.action = Some(action);
                self.action_group = group;
            }
        }
        // The weapon drawn and nothing in its section: the aim
        // (`00888070` → `008b28c0(0x11)`).
        player.weapon_drawn = self.drawn;
        if !f.dead && self.drawn && player.playing(section::WEAPON).is_none() {
            self.play_weapon(player, lib, f, group::AIM, -1, bones);
        }
    }

    /// Another weapon was put in the hand's 3D (`004ab750`, the biped's
    /// weapon attach: the weapon's model under the `Weapon` node, then,
    /// for an actor with a process and animation data): the weapon section
    /// stops (`004994f0(4, 0)`) and the process puts the weapon where its
    /// drawn state (`GetWeaponDrawn`, process vfunc +0x454) has it
    /// (`ForceWeaponDrawnSheathed`, +0x1cc, `009231d0`), for the player at
    /// once (`MiddleHighProcess::ReparentWeapon`, Xbox PDB, `00923960`):
    /// drawn, the kinds' `Equip` plays with the action `Equip` (vfunc
    /// +0x3ec) and the weapon already in hand; put away, the `Holster`
    /// group (`ActorRig` hangs it as its holster pose). The sequence played,
    /// whose `prn:` key the weapon bone moves under.
    // Translated from 004ab750, 009231d0 and 00923960 (decompiled,
    // FalloutNV.exe 1.4.0.525)
    pub fn weapon_attached(
        &mut self,
        player: &mut Player,
        lib: &mut impl Library,
        f: &Frame,
        bones: &[Bone],
    ) -> Option<Arc<Sequence>> {
        player.stop_section(section::WEAPON);
        self.ids[4] = groups::NONE;
        self.action = None;
        self.pending = None;
        if !self.drawn || f.dead {
            return None;
        }
        let (looked, seq) = self.weapon_group(lib, f, group::EQUIP)?;
        player.play(group::EQUIP, &seq, 0, bones);
        self.ids[4] = looked;
        self.action = Some(Action::Equip);
        self.action_group = group::EQUIP;
        self.attached = true;
        Some(seq)
    }

    /// A weapon-section group's id and sequence for this frame's kinds
    /// (`00897910`'s lookup), when the lookup gives that group.
    pub fn weapon_group(
        &self,
        lib: &mut impl Library,
        f: &Frame,
        group: u8,
    ) -> Option<(u16, Arc<Sequence>)> {
        let (weapon, movement) = self.kinds(f, false);
        let looked = lib.set().lookup(id(movement, weapon, group, f.power_armor));
        if group_of(looked) != group {
            return None;
        }
        Some((looked, lib.sequence(looked)?))
    }

    /// After the update: a group that ended in the weapon section with the
    /// weapon drawn gives way to the aim, cross-fading (`004994f0` →
    /// `008b28c0(0x11)`).
    // Translated from 004994f0 (decompiled, FalloutNV.exe 1.4.0.525)
    pub fn ended(
        &mut self,
        player: &mut Player,
        lib: &mut impl Library,
        f: &Frame,
        finished: &[super::Finished],
        bones: &[Bone],
    ) {
        if !finished.iter().any(|e| e.section == section::WEAPON) {
            return;
        }
        if self.drawn {
            self.action = None;
            if !self.play_weapon(player, lib, f, group::AIM, -1, bones) {
                player.weapon_drawn = false;
                player.stop_section(section::WEAPON);
                player.weapon_drawn = self.drawn;
            }
        }
    }

    /// Plays a weapon-section group with this frame's kinds (`008b28c0`
    /// through `00897910`): nothing (and the section stopped) when the
    /// lookup gives another group. Whether it plays.
    fn play_weapon(
        &mut self,
        player: &mut Player,
        lib: &mut impl Library,
        f: &Frame,
        group: u8,
        loops: i32,
        bones: &[Bone],
    ) -> bool {
        let (weapon, movement) = self.kinds(f, false);
        let looked = lib.set().lookup(id(movement, weapon, group, f.power_armor));
        if group_of(looked) != group {
            player.stop_section(section::WEAPON);
            self.ids[4] = groups::NONE;
            return false;
        }
        let Some(seq) = lib.sequence(looked) else {
            return false;
        };
        player.play(group, &seq, loops, bones);
        self.ids[4] = looked;
        true
    }

    /// The movement rate (`008950f0`): the turn's scale for a turn; for a
    /// walk or run the speed over the whole units a second the kinds'
    /// `Forward` (`FastForward` when running) travels (`00494300`: its own
    /// file only, and only when one file plays it; else the rate is 1).
    fn rate(
        &self,
        player: &mut Player,
        lib: &mut impl Library,
        f: &Frame,
        looked: u16,
        speed: f32,
    ) {
        let g = group_of(looked);
        if g == group::TURN_LEFT || g == group::TURN_RIGHT {
            player.movement_rate = f.turn_rate;
            return;
        }
        if !(group::FORWARD..=group::TURN_RIGHT).contains(&g) {
            return;
        }
        let base = match g {
            _ if !f.character => group::FORWARD,
            4..=6 | 0xb..=0xe => group::FORWARD,
            8..=10 => group::FAST_FORWARD,
            other => other,
        };
        let forward = looked & 0xff00 | u16::from(base);
        let whole = if lib.set().single(forward) {
            lib.sequence(forward)
                .map_or(0.0, |s| super::GroupData::read(&s).speed().trunc())
        } else {
            0.0
        };
        player.movement_rate = if whole != 0.0 { speed / whole } else { 1.0 };
    }

    /// Plays the chosen group in its section unless that section already
    /// plays it; a movement kind changing under a walk re-picks the base
    /// idle too (`00895110`, the part after the lookup).
    fn play(
        &mut self,
        player: &mut Player,
        lib: &mut impl Library,
        f: &Frame,
        (looked, sec): (u16, u8),
        (movement, weapon): (u8, u8),
        bones: &[Bone],
    ) {
        let g = group_of(looked);
        let slot = usize::from(sec);
        // (The seat's loop is compared by its sequence: another seat's is
        // another group 1.)
        if self.ids[slot] == looked && player.playing(sec) == Some(g) && g != group::DYNAMIC_IDLE {
            return;
        }
        let seq = if g == group::DYNAMIC_IDLE {
            f.dynamic_idle.cloned()
        } else if lib.set().has(looked) {
            lib.sequence(looked)
        } else {
            None
        };
        let Some(seq) = seq else {
            return;
        };
        if sec == section::MOVEMENT
            && player.playing(section::MOVEMENT).is_some()
            && movement_kind_of(self.ids[1]) != movement
        {
            let idle = lib
                .set()
                .lookup(id(movement, weapon, group::IDLE, f.power_armor));
            if idle != self.ids[0] {
                if let Some(s) = lib.sequence(idle) {
                    player.play(group::IDLE, &s, -1, bones);
                    self.ids[0] = idle;
                }
            }
        }
        player.play(g, &seq, -1, bones);
        self.ids[slot] = looked;
    }
}

/// Whether someone other than the player sneaks (`00888b50`): told to
/// (`SetForceSneak`, actor +0x125, `bForceSneak` Xbox PDB) or running a
/// package with the always-sneak flag (0x20000, `0067a4f0`). (The third
/// way, staying crouched in combat while unseen, `00566950`/`009549a0`,
/// isn't traced.)
// Translated from 00888b50 (decompiled, FalloutNV.exe 1.4.0.525)
pub fn npc_sneaks(force_sneak: bool, package_flags: Option<u32>) -> bool {
    force_sneak || package_flags.is_some_and(|f| f & 0x2_0000 != 0)
}

/// Whether a package keeps the weapon out: its "weapon drawn" flag
/// (0x800000, `0067a460`).
pub fn package_draws(package_flags: Option<u32>) -> bool {
    package_flags.is_some_and(|f| f & 0x80_0000 != 0)
}

/// Whether the process wants the weapon out after this frame. In combat
/// the combat's equip action draws it once its weapon is in hand
/// (`009da7c0`). Out of combat (`008eeec0`): a running package that draws
/// it, or being alerted (`GetAlert`, process vtable +0x31c), draws it when
/// standing (sit state 0) with no action under way; otherwise someone other
/// than the player whose weapon is drawn and wanted puts it away.
/// (`008eeec0`'s dialogue-package exception and its other two reasons to
/// draw, process +0x349 and an extra package's flag, aren't modelled.)
// Translated from 008eeec0 and 009da7c0 (decompiled, FalloutNV.exe
// 1.4.0.525)
pub fn want_weapon_out(
    want: bool,
    drawn: bool,
    in_combat: bool,
    keep_out: bool,
    (seated, acting, player): (bool, bool, bool),
) -> bool {
    if in_combat {
        return true;
    }
    if keep_out {
        if !seated && !acting && !drawn && !want {
            return true;
        }
        return want;
    }
    if !player && drawn && want {
        return false;
    }
    want
}

/// The direction group a path handler gives someone walking along a path
/// while facing a point (`009e2aa0`: `travel` the path's heading, `facing`
/// the heading to the point, radians clockwise from north). Within a degree
/// of the facing, or before the handler has run a quarter second (`waited`,
/// its timer +0x8c), the mover's own (forward); past 135° back when the 3D
/// has a `Backward` (people do; swimming only when the angle is 15° or more
/// from square, by the game's test); within 45° forward; else left when the
/// travel is anticlockwise of the facing, right when clockwise (a creature
/// without that side's group walks forward).
// Translated from 009e2aa0 (decompiled, FalloutNV.exe 1.4.0.525)
pub fn facing_direction(
    travel: f32,
    facing: f32,
    waited: f32,
    has: impl Fn(u8) -> bool,
    creature: bool,
) -> MoveFlags {
    let forward = MoveFlags {
        forward: true,
        ..Default::default()
    };
    if waited <= 0.25 {
        return forward;
    }
    // `004b15e0`: the facing less the travel, into (−π, π]; the sign
    // positive when the facing is clockwise of the travel.
    let tau = std::f32::consts::TAU;
    let mut d = (facing - travel).rem_euclid(tau);
    if d > std::f32::consts::PI {
        d -= tau;
    }
    let angle = d.abs();
    if angle <= 0.017_453_3 {
        return forward;
    }
    let degrees = angle * 57.295_78;
    if degrees > 135.0 && has(group::BACKWARD) {
        return MoveFlags {
            backward: true,
            ..Default::default()
        };
    }
    if degrees <= 45.0 {
        return forward;
    }
    if d > 0.0 {
        if creature && !has(group::LEFT) {
            return forward;
        }
        MoveFlags {
            left: true,
            ..Default::default()
        }
    } else {
        if creature && !has(group::RIGHT) {
            return forward;
        }
        MoveFlags {
            right: true,
            ..Default::default()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::animation::{GroupData, Settings};
    use nif::anim::{Motion, Track};
    use std::collections::HashMap;

    /// A library of generated sequences by file name.
    struct Lib {
        set: AnimSet,
        seqs: HashMap<String, Arc<Sequence>>,
    }

    impl Library for Lib {
        fn set(&self) -> &AnimSet {
            &self.set
        }
        fn sequence(&mut self, id: u16) -> Option<Arc<Sequence>> {
            let file = self.set.file(id, 0)?.to_string();
            self.seqs.get(&file).cloned()
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn holds(
        name: &str,
        group: &str,
        at: f32,
        priority: u8,
        length: f32,
        keys: &[(f32, &str)],
        travel: f32,
        looping: bool,
    ) -> Sequence {
        let mut tracks = vec![Track {
            node: "Arm".into(),
            priority,
            motion: Motion::Keys {
                translation: vec![(0.0, [0.0, 0.0, at])],
                rotation: Vec::new(),
                scale: Vec::new(),
                default: (None, None, None),
                euler: None,
            },
        }];
        if travel > 0.0 {
            tracks.push(Track {
                node: "Bip01".into(),
                priority,
                motion: Motion::Keys {
                    translation: vec![(0.0, [0.0; 3]), (length, [0.0, travel * length, 0.0])],
                    rotation: Vec::new(),
                    scale: Vec::new(),
                    default: (None, None, None),
                    euler: None,
                },
            });
        }
        let _ = name;
        Sequence {
            name: group.into(),
            start: 0.0,
            stop: length,
            looping,
            accum_root: Some("Bip01".into()),
            materials: Vec::new(),
            text_keys: keys.iter().map(|(t, k)| (*t, k.to_string())).collect(),
            tracks,
        }
    }

    fn lib() -> Lib {
        let files: Vec<(&str, Sequence)> = vec![
            (
                "c\\locomotion\\mtidle.kf",
                holds(
                    "i",
                    "Idle",
                    10.0,
                    10,
                    4.0,
                    &[(0.0, "start"), (4.0, "end")],
                    0.0,
                    true,
                ),
            ),
            (
                "c\\locomotion\\male\\mtforward.kf",
                holds(
                    "w",
                    "Forward",
                    20.0,
                    30,
                    1.2,
                    &[(0.0, "start"), (0.033, "Blend:6")],
                    85.3,
                    true,
                ),
            ),
            (
                "c\\locomotion\\male\\mtbackward.kf",
                holds(
                    "b",
                    "Backward",
                    21.0,
                    30,
                    1.2,
                    &[(0.0, "start")],
                    70.0,
                    true,
                ),
            ),
            (
                "c\\locomotion\\1hpforward.kf",
                holds(
                    "pw",
                    "Forward",
                    22.0,
                    31,
                    1.27,
                    &[(0.0, "start")],
                    90.6,
                    true,
                ),
            ),
            (
                "c\\sneakmtforward.kf",
                holds(
                    "sw",
                    "Forward",
                    23.0,
                    30,
                    1.53,
                    &[(0.0, "start")],
                    40.2,
                    true,
                ),
            ),
            (
                "c\\sneakmtidle.kf",
                holds(
                    "si",
                    "Idle",
                    11.0,
                    20,
                    3.3,
                    &[(0.0, "start"), (0.067, "Blend: 12")],
                    0.0,
                    true,
                ),
            ),
            (
                "c\\locomotion\\mtturnleft.kf",
                holds(
                    "tl",
                    "TurnLeft",
                    24.0,
                    30,
                    1.0,
                    &[(0.0, "start")],
                    0.0,
                    true,
                ),
            ),
            (
                "c\\1hpequip.kf",
                holds(
                    "e",
                    "Equip",
                    40.0,
                    55,
                    0.37,
                    &[(0.0, "start"), (0.167, "Attach"), (0.367, "end")],
                    0.0,
                    false,
                ),
            ),
            (
                "c\\1hpunequip.kf",
                holds(
                    "u",
                    "Unequip",
                    41.0,
                    55,
                    0.47,
                    &[(0.0, "start"), (0.233, "Detach"), (0.467, "end")],
                    0.0,
                    false,
                ),
            ),
            (
                "c\\1hpaim.kf",
                holds(
                    "a",
                    "Aim",
                    50.0,
                    45,
                    2.47,
                    &[(0.0, "start"), (2.467, "end")],
                    0.0,
                    true,
                ),
            ),
            (
                "c\\h2haim.kf",
                holds(
                    "ha",
                    "Aim",
                    51.0,
                    45,
                    4.67,
                    &[(0.0, "start"), (4.667, "end")],
                    0.0,
                    true,
                ),
            ),
            (
                "c\\1hpattackright.kf",
                holds(
                    "ar",
                    "AttackRight",
                    60.0,
                    55,
                    0.5,
                    &[(0.0, "start"), (0.5, "end")],
                    0.0,
                    false,
                ),
            ),
        ];
        let mut set = AnimSet::default();
        let mut seqs = HashMap::new();
        for (path, seq) in files {
            set.add(path, &seq.name);
            seqs.insert(path.to_string(), Arc::new(seq));
        }
        Lib { set, seqs }
    }

    fn bones() -> Vec<Bone> {
        let bone = |name: &str, parent, z| Bone {
            name: String::from(name),
            parent,
            local: nif::Transform {
                translation: [0.0, 0.0, z],
                ..nif::Transform::IDENTITY
            },
        };
        vec![bone("Bip01", None, 68.0), bone("Arm", Some(0), 10.0)]
    }

    fn frame<'a>() -> Frame<'a> {
        Frame {
            character: true,
            can_act: true,
            turn_rate: 1.5,
            weapon_kind: Some(4),
            ..Default::default()
        }
    }

    fn run(p: &mut Picker, player: &mut Player, lib: &mut Lib, f: &Frame, dt: f32) {
        let b = bones();
        p.pick(player, lib, f, &b);
        let done = player.update(dt);
        p.ended(player, lib, f, &done, &b);
    }

    #[test]
    fn walking_sneaking_and_backing_up_pick_their_files_and_rates() {
        let mut lib = lib();
        let mut p = Picker::new(false);
        let mut player = Player::new(Settings::default());
        let mut f = frame();
        run(&mut p, &mut player, &mut lib, &f, 0.1);
        assert_eq!(p.id(section::IDLE), 0x0000);
        // Walking at 77: the plain walk at 77 / 85.
        f.flags.forward = true;
        f.speed = 77.0;
        run(&mut p, &mut player, &mut lib, &f, 0.1);
        assert_eq!(p.id(section::MOVEMENT), 0x0003);
        assert!((player.movement_rate - 77.0 / 85.0).abs() < 1e-5);
        // Sneaking: the sneak walk at its own 40 a second, and the sneak
        // idle under it (a new movement kind under a walk).
        f.sneaking = true;
        f.speed = 77.0 * 0.57;
        run(&mut p, &mut player, &mut lib, &f, 0.1);
        assert_eq!(p.id(section::MOVEMENT), 0x1003);
        assert_eq!(p.id(section::IDLE), 0x1000);
        assert!((player.movement_rate - 77.0 * 0.57 / 40.0).abs() < 1e-4);
        // Backing up, sneaking: no sneak file: the plain back, at the sneak
        // walk's rate? No: the rate's divisor is the kinds' Forward of what
        // played (plain): 77·0.57 / 85.
        f.flags = MoveFlags {
            backward: true,
            ..Default::default()
        };
        run(&mut p, &mut player, &mut lib, &f, 0.1);
        assert_eq!(p.id(section::MOVEMENT), 0x0004);
        assert!((player.movement_rate - 77.0 * 0.57 / 85.0).abs() < 1e-4);
        // Standing: the walk eases out once it's in.
        f.flags = MoveFlags::default();
        f.speed = 0.0;
        for _ in 0..5 {
            run(&mut p, &mut player, &mut lib, &f, 0.1);
        }
        assert_eq!(player.playing(section::MOVEMENT), None);
        // Turning in place: the turn at the turn's rate.
        f.flags.turn_left = true;
        run(&mut p, &mut player, &mut lib, &f, 0.1);
        assert_eq!(player.playing(section::MOVEMENT), Some(group::TURN_LEFT));
        assert_eq!(player.movement_rate, 1.5);
    }

    #[test]
    fn another_weapon_in_hand_starts_the_weapon_section_over() {
        let mut lib = lib();
        let mut p = Picker::new(true);
        let mut player = Player::new(Settings::default());
        let b = bones();
        // Fists drawn: the unarmed guard.
        let mut f = Frame {
            want_drawn: true,
            weapon_kind: Some(1),
            ..frame()
        };
        run(&mut p, &mut player, &mut lib, &f, 0.1);
        assert_eq!(p.id(section::WEAPON), 0x0111);
        // A pistol in hand: its kind alone doesn't change the guard that
        // plays (what the player's third-person body showed).
        f.weapon_kind = Some(4);
        run(&mut p, &mut player, &mut lib, &f, 0.1);
        assert_eq!(p.id(section::WEAPON), 0x0111);
        // Attached (`004ab750`): drawn, the pistol's equip plays at once
        // with the weapon in hand, then the pistol's aim.
        let played = p.weapon_attached(&mut player, &mut lib, &f, &b);
        assert_eq!(played.map(|s| s.name.clone()), Some("Equip".to_string()));
        assert_eq!(p.action, Some(Action::Equip));
        assert_eq!(p.id(section::WEAPON), 0x0418);
        assert!(p.drawn);
        for _ in 0..6 {
            run(&mut p, &mut player, &mut lib, &f, 0.1);
            assert!(p.drawn);
        }
        assert_eq!(p.action, None);
        assert_eq!(player.playing(section::WEAPON), Some(group::AIM));
        assert_eq!(p.id(section::WEAPON), 0x0411);
        // Put away: the section stops and nothing plays in it.
        p.drawn = false;
        f.want_drawn = false;
        assert!(p.weapon_attached(&mut player, &mut lib, &f, &b).is_none());
        for _ in 0..4 {
            run(&mut p, &mut player, &mut lib, &f, 0.1);
        }
        assert_eq!(player.playing(section::WEAPON), None);
    }

    #[test]
    fn drawing_and_putting_away_follow_the_attach_keys() {
        let mut lib = lib();
        let mut p = Picker::new(false);
        let mut player = Player::new(Settings::default());
        let mut f = frame();
        run(&mut p, &mut player, &mut lib, &f, 0.1);
        // Wanting it out: the pistol's equip plays; the weapon is in hand
        // once its Attach key (0.167 s) is passed.
        f.want_drawn = true;
        run(&mut p, &mut player, &mut lib, &f, 0.1);
        assert_eq!(p.action, Some(Action::Equip));
        assert_eq!(p.id(section::WEAPON), 0x0418);
        assert!(!p.drawn);
        run(&mut p, &mut player, &mut lib, &f, 0.05);
        assert!(!p.drawn);
        run(&mut p, &mut player, &mut lib, &f, 0.05);
        run(&mut p, &mut player, &mut lib, &f, 0.05);
        assert!(p.drawn, "past Attach");
        // At its end the aim cross-fades in.
        for _ in 0..4 {
            run(&mut p, &mut player, &mut lib, &f, 0.05);
        }
        assert_eq!(player.playing(section::WEAPON), Some(group::AIM));
        assert_eq!(p.action, None);
        // Walking drawn: the pistol's walk.
        f.flags.forward = true;
        f.speed = 77.0;
        run(&mut p, &mut player, &mut lib, &f, 0.1);
        assert_eq!(p.id(section::MOVEMENT), 0x0403);
        // An attack plays over the aim, then the aim again.
        p.attack(group::ATTACK_RIGHT);
        run(&mut p, &mut player, &mut lib, &f, 0.1);
        assert_eq!(player.playing(section::WEAPON), Some(group::ATTACK_RIGHT));
        for _ in 0..6 {
            run(&mut p, &mut player, &mut lib, &f, 0.1);
        }
        assert_eq!(player.playing(section::WEAPON), Some(group::AIM));
        // Putting it away: the unequip; back at Detach; then nothing.
        f.want_drawn = false;
        run(&mut p, &mut player, &mut lib, &f, 0.1);
        assert_eq!(p.action, Some(Action::Unequip));
        assert!(p.drawn);
        run(&mut p, &mut player, &mut lib, &f, 0.1);
        run(&mut p, &mut player, &mut lib, &f, 0.1);
        assert!(p.drawn, "0.2 s: before Detach (0.233 s)");
        run(&mut p, &mut player, &mut lib, &f, 0.1);
        assert!(!p.drawn, "past Detach");
        for _ in 0..8 {
            run(&mut p, &mut player, &mut lib, &f, 0.1);
        }
        assert_eq!(player.playing(section::WEAPON), None);
        assert_eq!(p.id(section::MOVEMENT), 0x0003);
    }

    #[test]
    fn an_idle_holding_the_movement_section_keeps_the_walk_out() {
        let mut lib = lib();
        let mut p = Picker::new(false);
        let mut player = Player::new(Settings::default());
        let f = frame();
        let b = bones();
        run(&mut p, &mut player, &mut lib, &f, 0.1);
        let hit = Arc::new(holds(
            "h",
            "SpecialIdle",
            70.0,
            70,
            1.4,
            &[(0.0, "start"), (1.4, "end")],
            0.0,
            false,
        ));
        player.play_idle_in(section::MOVEMENT, &hit, 0, &b);
        p.idle_played(section::MOVEMENT);
        let mut walking = f;
        walking.flags.forward = true;
        walking.speed = 77.0;
        run(&mut p, &mut player, &mut lib, &walking, 0.1);
        assert_eq!(player.playing(section::MOVEMENT), Some(group::SPECIAL_IDLE));
        // Over: the walk takes the section again.
        for _ in 0..16 {
            run(&mut p, &mut player, &mut lib, &walking, 0.1);
        }
        assert_eq!(player.playing(section::MOVEMENT), Some(group::FORWARD));
        let _ = GroupData::read(&hit);
    }

    #[test]
    fn the_rules_that_feed_the_pick() {
        // Sneaking: told to, or the package's always-sneak flag.
        assert!(npc_sneaks(true, None));
        assert!(npc_sneaks(false, Some(0x2_0000)));
        assert!(!npc_sneaks(false, Some(0x2000)));
        // Combat draws; out of it a drawn, wanted weapon is put away.
        assert!(want_weapon_out(
            false,
            false,
            true,
            false,
            (false, false, false)
        ));
        assert!(!want_weapon_out(
            true,
            true,
            false,
            false,
            (false, false, false)
        ));
        // Still drawing: kept wanted until drawn.
        assert!(want_weapon_out(
            true,
            false,
            false,
            false,
            (false, false, false)
        ));
        // The player's isn't put away by it.
        assert!(want_weapon_out(
            true,
            true,
            false,
            false,
            (false, false, true)
        ));
        // A package that draws: out when standing and free.
        assert!(want_weapon_out(
            false,
            false,
            false,
            true,
            (false, false, false)
        ));
        assert!(!want_weapon_out(
            false,
            false,
            false,
            true,
            (true, false, false)
        ));
        assert!(package_draws(Some(0x80_0000)));
        // Facing a point while walking.
        let has = |_| true;
        let deg = |d: f32| d.to_radians();
        assert!(facing_direction(0.0, deg(30.0), 1.0, has, false).forward);
        assert!(facing_direction(0.0, deg(150.0), 1.0, has, false).backward);
        // Facing clockwise of the travel: the travel is to the left.
        assert!(facing_direction(0.0, deg(90.0), 1.0, has, false).left);
        assert!(facing_direction(0.0, deg(-90.0), 1.0, has, false).right);
        // Not before a quarter second.
        assert!(facing_direction(0.0, deg(90.0), 0.2, has, false).forward);
        // A creature without the side's group walks forward.
        assert!(facing_direction(0.0, deg(90.0), 1.0, |_| false, true).forward);
    }
}

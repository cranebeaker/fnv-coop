//! People and creatures moving: each placed actor gets a root entity where
//! it stands and one joint entity per bone of its skeleton; its pieces are
//! skinned meshes on those joints (the lit shader skins them, `SKINNED` in
//! `game_lit.wgsl`), and every frame the joints take the pose its
//! animations give.
//!
//! The animations are played as the game plays them
//! (`world::animation`), picked each frame as `Actor::PickAnimations`
//! picks them (`world::animation::pick`): the group for what the actor
//! does (standing, walking, running or sneaking in a direction, turning in
//! place, drawing or putting away the weapon, aiming, attacking) with its
//! weapon kind and movement kind, looked up in the files its 3D loads
//! (`anim_library`) with the game's fallbacks; the seat's loop in place of
//! the idle; sitting down, getting up and the tree's idles in the
//! special-idle section. A change of group cross-fades over the files'
//! blend times, the sequences blended per bone by their priorities. The
//! frame's flags (`walking`, `running`, `fighting`, …, set by `ai`,
//! `fighting`, `sitting` and `player_body`) stand for the game's mover and
//! process state.

use std::sync::Arc;

use bevy::asset::RenderAssetUsages;
use bevy::prelude::*;
use bevy::render::mesh::skinning::{SkinnedMesh, SkinnedMeshInverseBindposes};
use bevy::render::mesh::{Indices, PrimitiveTopology, VertexAttributeValues};
use cellview::{space, ActorData, MeshData};
use preview::cell::ActorSkeleton;
use world::animation::groups::AnimSet;
use world::animation::pick::{Frame, Library, Picker};
use world::animation::{self, group, section, MoveFlags, Player};

/// The player's reference.
const PLAYER: esm::FormId = esm::FormId(0x14);
use world::movement::TurnSide;

/// The animation settings from the INI (`[General]`
/// `fAnimationDefaultBlend` 0.2, `fAnimationMult` 1).
#[derive(Resource, Clone, Copy)]
pub struct AnimSettings(pub animation::Settings);

impl AnimSettings {
    pub fn read(ini: &assets::IniSettings) -> AnimSettings {
        let base = animation::Settings::default();
        AnimSettings(animation::Settings {
            default_blend: ini
                .float("General", "fAnimationDefaultBlend")
                .unwrap_or(base.default_blend),
            mult: ini
                .float("General", "fAnimationMult")
                .filter(|m| *m > 0.0)
                .unwrap_or(base.mult),
        })
    }
}

/// A placed actor's skeleton, its joint entities (one per bone, in the
/// skeleton's order) and the animations playing on it.
#[derive(Component)]
pub struct ActorRig {
    pub skeleton: Arc<ActorSkeleton>,
    pub joints: Vec<Entity>,
    /// The sequences playing and blending (the game's `AnimData`).
    pub player: Player,
    /// What `PickAnimations` keeps: the ids played, the action under way,
    /// whether the weapon is drawn.
    pub picker: Picker,
    /// The files the actor's 3D has (`anim_library`), once read.
    pub anims: Option<Arc<AnimSet>>,
    /// The actor's scale: the rates are worked out before it.
    pub scale: f32,
    /// How fast it moves over the ground this frame, units a second with
    /// its scale (`ai`, `fighting`); 0 standing.
    pub speed: f32,
    /// Walking (`ai`), which plays the walk instead of the idle; running
    /// (in a fight or fleeing), which plays the run.
    pub walking: bool,
    pub running: bool,
    /// Sneaking (movement flag 0x400): the sneak groups.
    pub sneaking: bool,
    /// Turning in place (`ai`: the game's turn, movement flags 0x10 left,
    /// 0x20 right), which plays the turn animation when not walking, at the
    /// turn's own scale (`rate`: `fAITurnSpeedScale` 1.5, `00888070`).
    pub turning: Option<(TurnSide, f32)>,
    /// Holding its pose (the dead).
    pub still: bool,
    /// In a fight (`ai`; actor +0x104, `bInCombat`, Xbox PDB).
    pub fighting: bool,
    /// The process wants the weapon out (`GetWantWeaponDrawn`, Xbox PDB):
    /// drawing and putting it away follow it.
    pub want_drawn: bool,
    /// The weapon in hand's weapon kind (none: unarmed).
    pub weapon_kind: Option<u8>,
    /// A character, not a creature.
    pub character: bool,
    /// In furniture (no drawing the weapon).
    pub seated: bool,
    /// When (elapsed seconds) the last attack began, and the group it
    /// plays (the weapon's attack animation, `AttackRight` by default).
    pub attack_at: Option<f32>,
    pub attack_group: u8,
    /// A reload begun: when, and its group (`ReloadA` … `ReloadZ`).
    pub reload_at: Option<(f32, u8)>,
    /// The base loop the idle tree gave instead of the standing idle (the
    /// seated loop in a chair, `ai::Seats`), looping.
    pub dynamic_idle: Option<Arc<nif::Sequence>>,
    /// A whole-body animation playing over everything (sitting down,
    /// getting up, a seated or standing idle from the tree), and how many
    /// seconds into it its owner has it.
    pub overlay: Option<(Arc<nif::Sequence>, f32)>,
    /// A script-requested special idle owns its actual KF clock until it
    /// finishes; the furniture/free-idle clock must not overwrite it.
    pub scripted_idle: Option<esm::FormId>,
    /// The section a requested or scripted idle plays in (its `IDLE`
    /// record's: 7 the special idle, 0x15 the upper body, 1 or 0x14 the
    /// movement section, `00498290`), and the one the overlay (a free idle
    /// from the tree, sitting down, getting up) plays in.
    pub idle_section: u8,
    pub overlay_section: u8,
    /// The section the overlay was last played in.
    overlay_in: Option<u8>,
    /// Dead: the ragdoll its bones follow.
    pub ragdoll: Option<Box<DeadBody>>,
    /// Has dropped its weapon (`world::body_parts::hurt_part`): the weapon
    /// model, on the `Weapon` bone, is hidden by shrinking that bone.
    pub disarmed: bool,
    /// The attack and reload the weapon section was last told to play.
    started_attack: Option<f32>,
    started_reload: Option<f32>,
    /// The mover's direction flags when they aren't just "walking forward"
    /// (the player's keys, `player_body`; a fighter stepping while facing
    /// its target, `world::animation::pick::facing_direction`).
    pub direction: Option<MoveFlags>,
    /// How long the path handler has walked facing a point (its timer
    /// +0x8c, `009e2aa0`).
    pub facing_time: f32,
    /// The draw for groups with several files (`0048f450`'s random pick).
    pub draw: u32,
    /// The running package's flags, by package (read once each).
    package_flags: Option<(esm::FormId, u32)>,
    /// Reloading last frame (`world::npc_combat::reloading`).
    reloading: bool,
    /// Whether the spine node faces up in the last pose (`IsFacingUp`;
    /// `None` without the node): see [`spine_up`].
    pub spine_up: Option<bool>,
    /// The node the `Weapon` bone hangs under: the `prn:` key of the group
    /// that last put it there (`00923960`), once known.
    pub weapon_parent: Option<String>,
    /// Another weapon was put in the hand (`004ab750`): handled at the next
    /// drive (`Picker::weapon_attached`).
    pub weapon_attached: bool,
    /// Someone else than the player: the weapon in hand
    /// (`world::combat::weapon_in_hand`) as [`npc_frame`] last saw it,
    /// for the redraw (`dress`); `None` until it has looked.
    pub held_weapon: Option<Option<esm::FormId>>,
}

/// A dead actor's ragdoll (`preview::ragdoll`): the bodies in motion, the
/// pose it died in, and where its skeleton stands in the world (game axes).
pub struct DeadBody {
    pub sim: physics::ragdoll::Ragdoll,
    pub died: Vec<nif::Transform>,
    pub placement: nif::Transform,
}

impl ActorRig {
    /// A rig standing idle: its idle loop already faded in (the game
    /// blends it in from the skeleton's own pose when the actor's 3D
    /// loads; a second of it has passed here so nobody appears in that
    /// pose), `phase` seconds into it.
    pub fn new(skeleton: Arc<ActorSkeleton>, scale: f32, phase: f32) -> ActorRig {
        let mut rig = ActorRig {
            joints: Vec::new(),
            player: Player::default(),
            picker: Picker::new(false),
            anims: None,
            scale: scale.max(1e-3),
            speed: 0.0,
            walking: false,
            running: false,
            sneaking: false,
            turning: None,
            still: false,
            fighting: false,
            want_drawn: false,
            weapon_kind: None,
            character: true,
            seated: false,
            attack_at: None,
            attack_group: group::ATTACK_RIGHT,
            reload_at: None,
            dynamic_idle: None,
            overlay: None,
            scripted_idle: None,
            idle_section: section::SPECIAL_IDLE,
            overlay_section: section::SPECIAL_IDLE,
            overlay_in: None,
            ragdoll: None,
            disarmed: false,
            started_attack: None,
            started_reload: None,
            direction: None,
            facing_time: 0.0,
            draw: 0,
            package_flags: None,
            reloading: false,
            spine_up: None,
            weapon_parent: None,
            weapon_attached: false,
            held_weapon: None,
            skeleton,
        };
        if let Some(idle) = rig.skeleton.idle.clone() {
            rig.player.play(group::IDLE, &idle, -1, &rig.skeleton.bones);
            rig.player.update(1.0);
            rig.player.sync_time(section::IDLE, phase);
        }
        rig
    }

    /// The bones' transforms now (as `animate_actors` poses them): put
    /// away with nothing drawing or putting it away, the weapon hangs in
    /// its holster pose (the `Holster` group the game plays, under its
    /// `prn:` node); otherwise the `Weapon` bone, as the playing groups move
    /// it, hangs under the node the last reparent named
    /// ([`ActorRig::weapon_parent`]): the equip's `prn:` (the right hand,
    /// fists the forearm's twist bone) from its `Attach` key, the
    /// unequip's from its `Detach`.
    pub fn pose_now(&self, _now: f32) -> Vec<nif::Transform> {
        let bones = &self.skeleton.bones;
        let mut pose = self.player.pose(bones);
        let readying = matches!(
            self.picker.action,
            Some(world::animation::pick::Action::Equip | world::animation::pick::Action::Unequip)
        );
        if !self.picker.drawn && !readying {
            if let Some(h) = &self.skeleton.holster {
                nif::hang_weapon(bones, &mut pose, h);
            }
        } else if let Some(parent) = &self.weapon_parent {
            nif::reparent_weapon(bones, &mut pose, parent);
        }
        if self.disarmed {
            hide_weapon(bones, &mut pose);
        }
        pose
    }

    /// What `PickAnimations` reads this frame, from the flags.
    fn frame<'a>(&self, dynamic_idle: Option<&'a Arc<nif::Sequence>>) -> Frame<'a> {
        let turn = self.turning.filter(|_| !self.walking);
        let mut flags = self.direction.unwrap_or(MoveFlags {
            forward: self.walking,
            ..Default::default()
        });
        flags.running = self.running;
        flags.turn_left = matches!(turn, Some((TurnSide::Left, _)));
        flags.turn_right = matches!(turn, Some((TurnSide::Right, _)));
        Frame {
            flags,
            sneaking: self.sneaking,
            speed: self.speed / self.scale,
            turn_rate: turn.map_or(1.0, |(_, rate)| rate),
            want_drawn: self.want_drawn,
            in_combat: self.fighting,
            character: self.character,
            weapon_kind: self.weapon_kind,
            power_armor: false,
            can_act: !self.still,
            seated: self.seated,
            dynamic_idle,
            dead: self.ragdoll.is_some(),
        }
    }

    /// One frame of the game's animation picking (`world::animation::
    /// pick`, with the flags this viewer keeps instead of the mover's and
    /// the process's), the special-idle section, and the sequences' update.
    fn drive(&mut self, dt: f32, lib: &mut impl Library) {
        let sk = self.skeleton.clone();
        let bones = &sk.bones;
        if self.attack_at.is_some() && self.attack_at != self.started_attack {
            self.started_attack = self.attack_at;
            self.picker.attack(self.attack_group);
        }
        if let Some((at, g)) = self.reload_at {
            if self.started_reload != Some(at) {
                self.started_reload = Some(at);
                self.picker.reload(g);
            }
        }
        let drawn_before = self.picker.drawn;
        {
            let seat = self.dynamic_idle.clone();
            let frame = self.frame(seat.as_ref());
            let mut picker = std::mem::take(&mut self.picker);
            // Another weapon in hand: the weapon section starts over and
            // the weapon goes where the drawn state has it.
            if std::mem::take(&mut self.weapon_attached) {
                let played = picker.weapon_attached(&mut self.player, lib, &frame, bones);
                self.weapon_parent = match played {
                    Some(seq) => nif::weapon_parent(&seq).map(str::to_string),
                    None => self.holster_parent(),
                };
            }
            picker.pick(&mut self.player, lib, &frame, bones);
            if self.weapon_parent.is_none() || picker.drawn != drawn_before {
                self.weapon_parent = self.reparented(&picker, lib, &frame);
            }
            self.picker = picker;
        }
        // The overlay: whatever plays over the rest in its section (the
        // special idle's, or the tree idle's own: `00498290`), its clock its
        // owner's (`sitting`).
        let slot = animation::slot(self.overlay_section);
        match (&self.scripted_idle, &self.overlay) {
            (Some(_), _) => {}
            (None, Some((seq, _))) => {
                if let Some(old) = self.overlay_in.filter(|s| *s != slot) {
                    if self.player.playing(old) == Some(group::SPECIAL_IDLE) {
                        self.player.stop_section(old);
                    }
                }
                self.player
                    .play_in(slot, group::SPECIAL_IDLE, seq, -1, bones);
                self.picker.idle_played(slot);
                self.overlay_in = Some(slot);
            }
            (None, None) => {
                if let Some(old) = self.overlay_in.take() {
                    if self.player.playing(old) == Some(group::SPECIAL_IDLE) {
                        self.player.stop_section(old);
                    }
                }
            }
        }
        let done = self.player.update(dt);
        {
            let seat = self.dynamic_idle.clone();
            let frame = self.frame(seat.as_ref());
            let mut picker = std::mem::take(&mut self.picker);
            picker.ended(&mut self.player, lib, &frame, &done, bones);
            self.picker = picker;
        }
        if self.scripted_idle.is_some() {
            let s = animation::slot(self.idle_section);
            if self.player.playing(s) != Some(group::SPECIAL_IDLE) {
                self.scripted_idle = None;
                self.idle_section = section::SPECIAL_IDLE;
            }
        } else if let (Some((_, elapsed)), Some(s)) = (&self.overlay, self.overlay_in) {
            self.player.sync_time(s, *elapsed);
        }
    }

    /// The holster pose's `prn:` node.
    fn holster_parent(&self) -> Option<String> {
        self.skeleton
            .holster
            .as_deref()
            .and_then(nif::weapon_parent)
            .map(str::to_string)
    }

    /// Where the weapon bone goes as the weapon is drawn or put away
    /// (`MiddleHighProcess::UpdateReparentWeapon`, Xbox PDB, `00923020`,
    /// once the process's reparent flag is set: `ReparentWeapon`,
    /// `00923960`): under the `prn:` node of the equip or unequip playing
    /// in the weapon section; with neither playing, of the kinds' `Equip`
    /// when drawn, else of the `Holster` group.
    // Translated from 00923960 (decompiled, FalloutNV.exe 1.4.0.525)
    fn reparented(&self, picker: &Picker, lib: &mut impl Library, f: &Frame) -> Option<String> {
        let readying = self
            .player
            .playing(section::WEAPON)
            .filter(|g| *g == group::EQUIP || *g == group::UNEQUIP)
            .and_then(|_| self.player.sequence(section::WEAPON));
        if let Some(seq) = readying {
            return nif::weapon_parent(seq).map(str::to_string);
        }
        if picker.drawn {
            let (_, seq) = picker.weapon_group(lib, f, group::EQUIP)?;
            return nif::weapon_parent(&seq).map(str::to_string);
        }
        self.holster_parent()
    }

    /// Goes limp as the game's dead do, if its skeleton has a ragdoll:
    /// from the pose it's in now (no death animation: the death routine
    /// `0089d900` makes a ragdoll's bodies dynamic at once; the `Death`
    /// group is played only by `UpdateAnimation` `00888070` for a death
    /// put off until the 3D loads, and then only for a creature without a
    /// ragdoll, `HasRagDoll` vtable +0x38c false), placed by `placement`;
    /// every body gains `nudge` (`world::combat::death_nudge`), then is
    /// thrown as the game throws the dead by a killing hit (`throw`: from
    /// where, and the speed every body gains, × its part's share:
    /// `world::combat::death_push_share`). False if it can't.
    pub fn go_limp(
        &mut self,
        now: f32,
        placement: nif::Transform,
        nudge: Option<[f32; 3]>,
        throw: Option<([f32; 3], f32)>,
    ) -> bool {
        let Some(rig) = &self.skeleton.ragdoll else {
            return false;
        };
        let died = self.pose_now(now);
        let mut sim = rig.start(&died, &placement);
        if let Some(v) = nudge {
            sim.add_velocity(v);
        }
        if let Some((origin, speed)) = throw {
            let shares: Vec<f32> = rig
                .ragdoll
                .bodies
                .iter()
                .map(|b| world::combat::death_push_share(b.part))
                .collect();
            sim.throw_from(origin, speed, &shares);
        }
        self.ragdoll = Some(Box::new(DeadBody {
            sim,
            died,
            placement,
        }));
        self.walking = false;
        self.still = true;
        true
    }
}

/// Shrinks the `Weapon` bone to nothing, so the weapon skinned to it isn't
/// seen.
fn hide_weapon(bones: &[nif::Bone], pose: &mut [nif::Transform]) {
    let weapon = bones
        .iter()
        .position(|b| b.name.eq_ignore_ascii_case("Weapon"));
    if let Some(t) = weapon.and_then(|i| pose.get_mut(i)) {
        t.scale = 0.0;
    }
}

/// A game-space transform as a Bevy one (still in game units and axes:
/// joints sit under the actor's root, which converts).
pub(crate) fn bevy_transform(t: &nif::Transform) -> Transform {
    let r = t.rotation;
    let rotation = Mat3::from_cols(
        Vec3::new(r[0][0], r[1][0], r[2][0]),
        Vec3::new(r[0][1], r[1][1], r[2][1]),
        Vec3::new(r[0][2], r[1][2], r[2][2]),
    );
    Transform {
        translation: Vec3::from(t.translation),
        rotation: Quat::from_mat3(&rotation).normalize(),
        scale: Vec3::splat(t.scale),
    }
}

/// Spawns an actor's root and joints; returns the root and the joints.
pub fn spawn_actor(
    commands: &mut Commands,
    actor: &ActorData,
    index: usize,
    root_tag: impl Bundle,
) -> (Entity, Vec<Entity>) {
    let root = commands
        .spawn((
            Transform::from_matrix(Mat4::from_cols_array(&space::matrix(&actor.transform))),
            Visibility::default(),
            root_tag,
        ))
        .id();
    // Spread the actors over their idle (a fixed spread, so pictures
    // repeat).
    let phase = (index as f32 * 0.618_034).fract() * 4.0;
    let m = &actor.transform;
    let scale = (m[0] * m[0] + m[1] * m[1] + m[2] * m[2]).sqrt();
    let mut rig = ActorRig::new(actor.skeleton.clone(), scale, phase);
    let pose = rig.pose_now(0.0);
    let joints: Vec<Entity> = pose
        .iter()
        .map(|t| commands.spawn((bevy_transform(t), ChildOf(root))).id())
        .collect();
    rig.joints = joints.clone();
    commands.entity(root).insert((
        rig,
        crate::ai::Walker::new(actor),
        crate::sitting::Life::default(),
    ));
    // People are redrawn as what they wear changes (`dress`).
    if let Some(look) = actor.look.as_ref().filter(|l| !l.creature) {
        commands
            .entity(root)
            .insert(crate::dress::Dressed::new(look.clone()));
    }
    (root, joints)
}

/// A skinned piece as a Bevy mesh (in the piece's own space, game units),
/// with its inverse bind poses. `None` for pieces without a rig.
pub fn skinned_mesh(data: &MeshData) -> Option<(Mesh, SkinnedMeshInverseBindposes)> {
    let rig = data.rig.as_ref()?;
    let mut mesh = Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::default(),
    );
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, data.positions.clone());
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, data.normals.clone());
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, data.uvs.clone());
    if let Some(tangents) = &data.tangents {
        mesh.insert_attribute(Mesh::ATTRIBUTE_TANGENT, tangents.clone());
    }
    if let Some(colors) = &data.colors {
        mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, colors.clone());
    }
    mesh.insert_attribute(
        Mesh::ATTRIBUTE_JOINT_INDEX,
        VertexAttributeValues::Uint16x4(rig.joint_indices.clone()),
    );
    mesh.insert_attribute(Mesh::ATTRIBUTE_JOINT_WEIGHT, rig.joint_weights.clone());
    mesh.insert_indices(Indices::U16(data.indices.clone()));
    let binds: Vec<Mat4> = rig
        .joints
        .iter()
        .map(|(_, m)| Mat4::from_cols_array(m))
        .collect();
    Some((mesh, SkinnedMeshInverseBindposes::from(binds)))
}

/// The joints a piece's skin uses, from its actor's joints.
pub fn skin_joints(data: &MeshData, joints: &[Entity]) -> Vec<Entity> {
    data.rig
        .as_ref()
        .map(|rig| {
            rig.joints
                .iter()
                .map(|(bone, _)| joints.get(*bone).copied().unwrap_or(joints[0]))
                .collect()
        })
        .unwrap_or_default()
}

/// Deferred NPC PlayIdle requests (008ba600, process flag 0x80).
/// Only loaded, unconditional leaf special idles are represented here.
/// Trees/other groups need their own original dispatch, not a fallback.
#[allow(clippy::too_many_arguments)]
pub fn script_idles(
    mut requests: ResMut<crate::player_idle::PlayerIdle>,
    game: Res<crate::GameFiles>,
    mut state: ResMut<crate::dialogue::DialogueState>,
    mut seats: ResMut<crate::sitting::Seats>,
    settings: Res<AnimSettings>,
    menus: Res<crate::menus::Menus>,
    pending: Res<crate::PendingScene>,
    pending_exterior: Res<crate::exterior::PendingExterior>,
    mut actors: Query<(
        &crate::ai::Walker,
        &mut crate::sitting::Life,
        &mut ActorRig,
        &Visibility,
    )>,
) {
    // F9 installs its destination next Update. Keep restored requests until
    // that scene exists, rather than applying them to the source cell.
    if menus.is_open() || pending.0.is_some() || pending_exterior.0.is_some() {
        return;
    }
    for (who, requested) in std::mem::take(&mut requests.npc_requests) {
        let Some((_, mut life, mut rig, visibility)) =
            actors.iter_mut().find(|(w, _, _, _)| w.reference == who)
        else {
            eprintln!("Script idle {requested} for {who}: no loaded actor animation data");
            continue;
        };
        if *visibility == Visibility::Hidden || state.0.dead.contains(&who) {
            continue;
        }
        let Some(idle) = seats.tree.get(requested).cloned() else {
            continue;
        };
        // Any section an idle's record may name (`00498290`): the base
        // loop, the movement section, the special idle, the whole or the
        // upper body.
        if !matches!(idle.group(), 0 | 1 | 7 | 0x14 | 0x15)
            || !idle.is_animation()
            || !idle.conditions.is_empty()
        {
            eprintln!("Script idle {requested}: group/tree/condition dispatch pending");
            continue;
        }
        let Some(seq) = seats.sequence(&game.0, &idle.model) else {
            eprintln!("Script idle {requested}: couldn't load {}", idle.model);
            continue;
        };
        let count =
            idle.extra_loops(|lo, hi| lo + (state.0.roll() % (u64::from(hi - lo) + 1)) as u8);
        let loops = if count == 255 { -1 } else { i32::from(count) };
        let bones = rig.skeleton.clone();
        rig.player.settings = settings.0;
        // The loaded idle freed at once wherever it played, the new one in
        // its section (`00498290` → `00498910(0,1)`).
        let old = animation::slot(rig.idle_section);
        let section = animation::slot(idle.group());
        if old != section && rig.player.playing(old) == Some(group::SPECIAL_IDLE) {
            rig.player.cut_section(old);
        }
        rig.player.play_idle_in(section, &seq, loops, &bones.bones);
        rig.picker.idle_played(section);
        rig.idle_section = section;
        rig.scripted_idle = Some(requested);
        rig.overlay = None;
        life.idles.stop();
        life.idles.last = Some(requested);
        println!("{who}: script idle {} started", idle.editor_id);
    }
}

/// Every frame: each actor's animations are picked and run on, and its
/// joints take the pose; the dead's follow their ragdolls until they come
/// to rest. While the dialogue menu or a viewer menu is open the game is
/// in menu mode and stands still, animations included (someone talked to
/// mid-stride kept walking on the spot before), except, in the dialogue
/// menu, the speaker: they stop walking and turn to face the player.
#[allow(clippy::too_many_arguments)]
pub fn animate_actors(
    time: Res<Time>,
    settings: Option<Res<AnimSettings>>,
    conversation: Option<Res<crate::dialogue::Conversation>>,
    menus: Option<Res<crate::menus::Menus>>,
    drawn: Option<Res<crate::game_menus::MenuDraw>>,
    collision: Option<Res<crate::walk::CellCollision>>,
    havok: Option<Res<crate::clutter::HavokFrame>>,
    look: Option<Res<crate::look::LookSettings>>,
    game: Option<Res<crate::GameFiles>>,
    state: Option<Res<crate::dialogue::DialogueState>>,
    mut library: Option<ResMut<crate::anim_library::AnimLibrary>>,
    mut anchors: Option<ResMut<crate::look::LookAnchors>>,
    mut rigs: Query<(
        &mut ActorRig,
        Option<&mut crate::look::HeadTrack>,
        Option<&crate::ai::Walker>,
    )>,
    mut joints: Query<&mut Transform>,
) {
    let now = time.elapsed_secs();
    let dt = time.delta_secs();
    let in_dialogue = conversation
        .as_ref()
        .is_some_and(|c| c.0.as_ref().is_some_and(|t| !t.is_line_only()));
    if menu_stops_animation(
        menus.as_ref().is_some_and(|m| m.is_open()) && crate::coop_live::menus_freeze_time(),
        menus.as_ref().is_some_and(|m| m.only_game_menus())
            && crate::coop_live::menus_freeze_time(),
        in_dialogue,
        drawn.as_ref().map_or(&[][..], |d| &d.1[..]),
    ) {
        return;
    }
    for (mut rig, mut head_track, walker) in &mut rigs {
        let rig = &mut *rig;
        // In the dialogue menu only the speaker moves: `ai` holds everyone
        // else (`still`) and has the speaker stop walking and turn in place
        // to face the player (`008a5580`, see `ai::move_actors`), which
        // plays here; the dead's ragdolls wait.
        if in_dialogue && (rig.still || rig.ragdoll.is_some()) {
            continue;
        }
        let pose = if let Some(dead) = rig.ragdoll.as_deref_mut() {
            if dead.sim.asleep {
                continue;
            }
            // By the game frame's time (`clutter::HavokFrame`), as the
            // bodies' world.
            let havok_dt = havok.as_ref().map_or(0.0, |h| h.dt);
            if let (Some(c), true) = (&collision, havok_dt > 0.0) {
                dead.sim.update(&c.0, havok_dt);
            }
            if dead.sim.asleep {
                // The game takes settled bodies out of the world here
                // (`FinishDying` `008f7350` → `DetachHavok`).
                let (_, at) = dead.sim.frame(0);
                println!(
                    "{:.1} s: {} comes to rest, its first body at ({:.1}, {:.1}, {:.1})",
                    now,
                    walker.map_or(PLAYER, |w| w.reference),
                    at[0],
                    at[1],
                    at[2]
                );
            }
            let Some(r) = &rig.skeleton.ragdoll else {
                continue;
            };
            r.bones(&dead.sim, &rig.skeleton.bones, &dead.died, &dead.placement)
        } else if rig.still {
            continue;
        } else {
            if let Some(s) = &settings {
                rig.player.settings = s.0;
            }
            match (&game, library.as_deref_mut()) {
                (Some(game), Some(library)) => {
                    let set = match &rig.anims {
                        Some(set) => set.clone(),
                        None => {
                            let set = library.set_for(&game.0, &rig.skeleton);
                            rig.anims = Some(set.clone());
                            set
                        }
                    };
                    if let (Some(w), Some(state)) = (walker, &state) {
                        npc_frame(rig, w, &game.0, &state.0, &set, (now, dt));
                    }
                    rig.draw = rig.draw.wrapping_mul(1_103_515_245).wrapping_add(12_345);
                    let mut lib = crate::anim_library::ActorLibrary {
                        set: &set,
                        library,
                        game: &game.0,
                        draw: rig.draw >> 16,
                    };
                    let before = (rig.picker.action, rig.picker.drawn);
                    rig.drive(dt, &mut lib);
                    let after = (rig.picker.action, rig.picker.drawn);
                    if before != after {
                        let who = walker.map_or(PLAYER, |w| w.reference);
                        let what = match after {
                            (Some(a), _) if before.0 != Some(a) => {
                                format!("{a:?} ({:04x})", rig.picker.id(section::WEAPON))
                            }
                            (_, true) if !before.1 => "weapon in hand".to_string(),
                            (_, false) if before.1 => "weapon put away".to_string(),
                            _ => String::new(),
                        };
                        if !what.is_empty() {
                            println!("{now:.1} s: {who}: {what}");
                        }
                    }
                }
                _ => {
                    let set = AnimSet::default();
                    rig.drive(dt, &mut Unloaded(&set));
                }
            }
            let mut pose = rig.pose_now(now);
            if let (Some(ht), Some(w), Some(s)) = (head_track.as_deref_mut(), walker, look.as_ref())
            {
                // Whom they look at: `ai`'s head-track target
                // (`world::head_track`), at their look anchor
                // (`look::LookAnchors`).
                let target = w
                    .looking_at()
                    .and_then(|who| anchors.as_ref()?.0.get(&who).copied());
                let bones = &rig.skeleton.bones;
                let placement = w.placement();
                crate::look::track(ht, &s.0, bones, &mut pose, &placement, w.position, target);
            }
            // Where the others look at this one (008a2fa0).
            if let (Some(w), Some(a)) = (walker, anchors.as_deref_mut()) {
                match crate::look::anchor_of(
                    head_track.as_deref(),
                    &rig.skeleton.bones,
                    &pose,
                    &w.placement(),
                    w.position,
                ) {
                    Some(t) => a.0.insert(w.reference, t),
                    None => a.0.remove(&w.reference),
                };
            }
            pose
        };
        rig.spine_up = spine_up(&rig.skeleton.bones, &pose);
        for (joint, t) in rig.joints.iter().zip(&pose) {
            if let Ok(mut transform) = joints.get_mut(*joint) {
                *transform = bevy_transform(t);
            }
        }
    }
}

/// A library with no files read (no game data): nothing but what already
/// plays.
struct Unloaded<'a>(&'a AnimSet);

impl Library for Unloaded<'_> {
    fn set(&self) -> &AnimSet {
        self.0
    }
    fn sequence(&mut self, _id: u16) -> Option<Arc<nif::Sequence>> {
        None
    }
}

/// Someone else than the player, this frame, as the game's process and
/// mover would have it for `PickAnimations`: sneaking (`00888b50`), the
/// weapon wanted out (in a fight, by the running package or alerted:
/// `world::animation::pick::want_weapon_out`), the weapon in hand's kind,
/// attack and reload groups, furniture, and a fighter's steps while it
/// keeps facing its target (`009e2aa0`: `facing_direction`).
fn npc_frame(
    rig: &mut ActorRig,
    walker: &crate::ai::Walker,
    game: &cellview::Game,
    state: &world::scripting::GameState,
    set: &AnimSet,
    (now, dt): (f32, f32),
) {
    use world::animation::pick;
    let order = &game.order;
    let me = walker.reference;
    let flags = match (walker.package, rig.package_flags) {
        (Some(p), Some((q, f))) if p == q => Some(f),
        (Some(p), _) => {
            let f = world::ai::Package::load(order, p).map_or(0, |pk| pk.flags);
            rig.package_flags = Some((p, f));
            Some(f)
        }
        (None, _) => None,
    };
    rig.character = !world::combat::is_creature(order, me);
    rig.sneaking = pick::npc_sneaks(state.more.forced_sneak.contains(&me), flags);
    rig.seated = state.sitters.contains_key(&me) || state.furniture.contains_key(&me);
    let weapon = world::combat::weapon_in_hand(order, state, me);
    rig.held_weapon = Some(weapon.as_ref().map(|w| w.form_id));
    rig.weapon_kind = weapon
        .as_ref()
        .map(|w| world::animation::groups::weapon_kind(w.animation));
    if let Some(w) = &weapon {
        rig.attack_group = match w.attack_animation {
            g @ 26..=0xa8 => g,
            _ => group::ATTACK_RIGHT,
        };
    }
    let keep_out = pick::package_draws(flags) || state.more.alerted.contains(&me);
    rig.want_drawn = pick::want_weapon_out(
        rig.want_drawn,
        rig.picker.drawn,
        rig.fighting,
        keep_out,
        (rig.seated, rig.picker.action.is_some(), false),
    );
    let reloading = world::npc_combat::reloading(state, me);
    if reloading && !rig.reloading {
        if let Some(w) = &weapon {
            rig.reload_at = Some((now, group::RELOAD_A + w.reload_animation.min(22)));
        }
    }
    rig.reloading = reloading;
    // A fighter stepping while it faces its target: the side or back
    // group by the angle (people's path handler, `009e2aa0`).
    let v = walker.velocity;
    if rig.fighting && rig.walking && v[0].hypot(v[1]) > 1.0 {
        rig.facing_time += dt;
        let travel = v[0].atan2(v[1]);
        rig.direction = Some(pick::facing_direction(
            travel,
            walker.heading,
            rig.facing_time,
            |g| set.has(u16::from(g)),
            !rig.character,
        ));
    } else {
        rig.facing_time = 0.0;
        rig.direction = None;
    }
}

/// Whether menu mode stops every animation this frame: any menu open
/// (`menu_open`), except that the dialogue menu updates its speaker each
/// frame (`00762950` → `Actor::UpdateInDialogue`, Xbox PDB, `008a5580`):
/// in a conversation with nothing but the game's own menus up
/// (`only_game_menus`) and all of those the dialogue menu (class 1009,
/// `open_classes`), the speaker animates (the rest are held `still` by
/// `ai`). Before, the dialogue menu's own screen counted as a menu here
/// and froze the speaker too.
fn menu_stops_animation(
    menu_open: bool,
    only_game_menus: bool,
    in_dialogue: bool,
    open_classes: &[i32],
) -> bool {
    let dialogue_only = in_dialogue
        && only_game_menus
        && open_classes.iter().all(|c| *c == ui::menus::dialog::CLASS);
    menu_open && !dialogue_only
}

/// Whether a pose's spine node (`Bip01 Spine`, else `Bip01 Spine01`) faces
/// up, as `IsFacingUp` asks (`005a0710` → `00c6b7b0`): its world rotation's
/// [2][1] above 0. An actor's placement only turns it about Z, which
/// leaves that row alone, so the pose's own (model-space) rotation answers.
/// `None` when the skeleton has neither node.
pub fn spine_up(bones: &[nif::Bone], pose: &[nif::Transform]) -> Option<bool> {
    let find = |name: &str| bones.iter().position(|b| b.name.eq_ignore_ascii_case(name));
    let i = find("Bip01 Spine").or_else(|| find("Bip01 Spine01"))?;
    Some(pose.get(i)?.rotation[2][1] > 0.0)
}

/// Tells the world which way each person's spine faces (`IsFacingUp`).
pub fn report_facing_up(
    mut state: ResMut<crate::dialogue::DialogueState>,
    rigs: Query<(&crate::ai::Walker, &ActorRig)>,
) {
    let facing = rigs
        .iter()
        .filter_map(|(w, rig)| Some((w.reference, rig.spine_up?)))
        .collect();
    world::more_functions::report_facing_up(&mut state.0, facing);
}

/// The skinned piece entity's components.
pub fn skinned(
    inverse_bindposes: Handle<SkinnedMeshInverseBindposes>,
    joints: Vec<Entity>,
) -> SkinnedMesh {
    SkinnedMesh {
        inverse_bindposes,
        joints,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use nif::anim::{Motion, Sequence, Track};

    #[test]
    fn only_the_dialogue_menu_lets_the_speaker_animate() {
        let dialog = ui::menus::dialog::CLASS;
        // Nothing open: everyone animates.
        assert!(!menu_stops_animation(false, true, false, &[]));
        // The game's dialogue menu alone, in a conversation: animating.
        assert!(!menu_stops_animation(true, true, true, &[dialog]));
        // A game menu on top of it (barter, 1053), or another game menu
        // without a conversation: stopped.
        assert!(menu_stops_animation(true, true, true, &[dialog, 1053]));
        assert!(menu_stops_animation(true, true, false, &[dialog]));
        // The Pip-Boy or a viewer menu: stopped.
        assert!(menu_stops_animation(true, false, true, &[dialog]));
    }

    #[test]
    fn restored_requests_wait_for_the_destination_scene() {
        let data = testdata::functions::functions("idle-load-order");
        let game = cellview::Game::open(
            data.path(),
            &cellview::Options {
                official: true,
                ..default()
            },
        )
        .unwrap();
        let state = world::scripting::GameState::new(&game.order);
        let scene = game
            .load_cell_now(
                esm::FormId(testdata::functions::ids::HOUSE),
                &state.disabled,
            )
            .unwrap();
        let mut idle = crate::player_idle::PlayerIdle::default();
        idle.npc_requests.insert(esm::FormId(42), esm::FormId(43));
        let mut app = App::new();
        app.insert_resource(crate::sitting::Seats::new(&game.order));
        app.insert_resource(crate::GameFiles(Arc::new(game)));
        app.insert_resource(crate::dialogue::DialogueState(state));
        app.insert_resource(idle);
        app.insert_resource(AnimSettings(animation::Settings::default()));
        app.insert_resource(crate::menus::Menus::default());
        app.insert_resource(crate::PendingScene(Some(scene)));
        app.insert_resource(crate::exterior::PendingExterior::default());
        app.add_systems(Update, script_idles);
        app.update();
        assert_eq!(
            app.world()
                .resource::<crate::player_idle::PlayerIdle>()
                .npc_requests
                .len(),
            1
        );
        // Only after destination installation may dispatch consume the request.
        app.world_mut().resource_mut::<crate::PendingScene>().0 = None;
        app.update();
        assert!(app
            .world()
            .resource::<crate::player_idle::PlayerIdle>()
            .npc_requests
            .is_empty());
    }
    #[test]
    fn the_spine_faces_up_by_its_rotation() {
        let bone = |name: &str| nif::Bone {
            name: name.into(),
            parent: None,
            local: nif::Transform::IDENTITY,
        };
        let bones = [bone("Bip01"), bone("Bip01 Spine")];
        // Standing: the spine's Y axis level, [2][1] = 0: not up.
        let mut pose = vec![nif::Transform::IDENTITY; 2];
        assert_eq!(spine_up(&bones, &pose), Some(false));
        // Turned 90° about X (Y → Z): up.
        pose[1].rotation = [[1.0, 0.0, 0.0], [0.0, 0.0, -1.0], [0.0, 1.0, 0.0]];
        assert_eq!(spine_up(&bones, &pose), Some(true));
        // The other name, and none at all.
        assert_eq!(
            spine_up(&[bone("Bip01"), bone("Bip01 Spine01")], &pose),
            Some(true)
        );
        assert_eq!(spine_up(&bones[..1], &pose), None);
    }

    #[test]
    fn a_dropped_weapon_shrinks_its_bone_away() {
        let bone = |name: &str, parent| nif::Bone {
            name: name.into(),
            parent,
            local: nif::Transform::IDENTITY,
        };
        let bones = [
            bone("Bip01", None),
            bone("Bip01 R Hand", Some(0)),
            bone("Weapon", Some(1)),
        ];
        let mut pose = vec![nif::Transform::IDENTITY; 3];
        hide_weapon(&bones, &mut pose);
        assert_eq!(pose[2].scale, 0.0);
        assert_eq!((pose[0].scale, pose[1].scale), (1.0, 1.0));
    }

    #[test]
    fn game_transforms_keep_their_meaning() {
        let s = std::f32::consts::FRAC_1_SQRT_2;
        // 90° about z (x → y), moved and scaled.
        let t = nif::Transform {
            rotation: [[0.0, -1.0, 0.0], [1.0, 0.0, 0.0], [0.0, 0.0, 1.0]],
            translation: [1.0, 2.0, 3.0],
            scale: 2.0,
        };
        let b = bevy_transform(&t);
        let p = b.transform_point(Vec3::new(1.0, 0.0, 0.0));
        assert!((p - Vec3::new(1.0, 4.0, 3.0)).length() < 1e-5, "{p}");
        assert!((b.rotation.w - s).abs() < 1e-5);
    }

    /// A skeleton of a root and an arm, with an idle holding the arm at
    /// `idle_arm` and a walk whose root travels 102.3 in 1.2 s holding it
    /// at `walk_arm`.
    fn skeleton(idle_arm: f32, walk_arm: f32) -> Arc<ActorSkeleton> {
        let bone = |name: &str, parent, z| nif::Bone {
            name: name.into(),
            parent,
            local: nif::Transform {
                translation: [0.0, 0.0, z],
                ..nif::Transform::IDENTITY
            },
        };
        let holds = |node: &str, at: [f32; 3], priority: u8| Track {
            node: node.into(),
            priority,
            motion: Motion::Keys {
                translation: vec![(0.0, at)],
                rotation: Vec::new(),
                scale: Vec::new(),
                default: (None, None, None),
                euler: None,
            },
        };
        let idle = Sequence {
            name: "Idle".into(),
            start: 0.0,
            stop: 4.0,
            looping: true,
            accum_root: Some("Bip01".into()),
            materials: Vec::new(),
            text_keys: vec![(0.0, "start".into()), (4.0, "end".into())],
            tracks: vec![holds("Arm", [0.0, 0.0, idle_arm], 10)],
        };
        let walk = Sequence {
            name: "Forward".into(),
            start: 0.0,
            stop: 1.2,
            looping: true,
            accum_root: Some("Bip01".into()),
            materials: Vec::new(),
            text_keys: vec![(0.0, "start".into()), (0.033, "Blend:6".into())],
            tracks: vec![
                Track {
                    node: "Bip01".into(),
                    priority: 30,
                    motion: Motion::Keys {
                        translation: vec![(0.0, [0.0; 3]), (1.2, [0.0, 102.3, 0.0])],
                        rotation: Vec::new(),
                        scale: Vec::new(),
                        default: (None, None, None),
                        euler: None,
                    },
                },
                holds("Arm", [0.0, 0.0, walk_arm], 30),
            ],
        };
        Arc::new(ActorSkeleton {
            bones: vec![bone("Bip01", None, 68.0), bone("Arm", Some(0), 10.0)],
            idle: Some(Arc::new(idle)),
            walk: Some(Arc::new(walk)),
            ..Default::default()
        })
    }

    /// The files a test actor's 3D has: its idle and walk (the same
    /// sequences its skeleton carries) and any more.
    struct TestLib {
        set: AnimSet,
        seqs: std::collections::HashMap<String, Arc<Sequence>>,
    }

    impl TestLib {
        fn new(sk: &ActorSkeleton, more: Vec<(&str, Arc<Sequence>)>) -> TestLib {
            let mut all = vec![
                ("c\\locomotion\\mtidle.kf", sk.idle.clone().unwrap()),
                (
                    "c\\locomotion\\male\\mtforward.kf",
                    sk.walk.clone().unwrap(),
                ),
            ];
            all.extend(more);
            let mut set = AnimSet::default();
            let mut seqs = std::collections::HashMap::new();
            for (path, seq) in all {
                set.add(path, &seq.name);
                seqs.insert(path.to_string(), seq);
            }
            TestLib { set, seqs }
        }
    }

    impl Library for TestLib {
        fn set(&self) -> &AnimSet {
            &self.set
        }
        fn sequence(&mut self, id: u16) -> Option<Arc<Sequence>> {
            self.seqs.get(self.set.file(id, 0)?).cloned()
        }
    }

    fn drive(rig: &mut ActorRig, lib: &mut TestLib, dt: f32) {
        rig.drive(dt, lib);
    }
    #[test]
    fn scripted_idle_keeps_its_clock_and_finishes_without_restarting() {
        let mut rig = ActorRig::new(skeleton(10.0, 20.0), 1.0, 0.0);
        let mut lib = TestLib::new(&rig.skeleton, Vec::new());
        let seq = Arc::new(Sequence {
            name: "SpecialIdle".into(),
            start: 0.0,
            stop: 1.0,
            looping: false,
            accum_root: None,
            materials: Vec::new(),
            text_keys: vec![(0.0, "start".into()), (1.0, "end".into())],
            tracks: vec![Track {
                node: "Arm".into(),
                priority: 95,
                motion: Motion::Keys {
                    translation: vec![(0.0, [0.0, 0.0, 40.0])],
                    rotation: Vec::new(),
                    scale: Vec::new(),
                    default: (None, None, None),
                    euler: None,
                },
            }],
        });
        rig.player.play_script_idle(&seq, 0, &rig.skeleton.bones);
        rig.scripted_idle = Some(esm::FormId(123));
        // A stale furniture/free-idle overlay must neither replace the
        // script sequence nor overwrite its clock.
        rig.overlay = Some((rig.skeleton.idle.clone().unwrap(), 10.0));
        drive(&mut rig, &mut lib, 0.2);
        assert!(Arc::ptr_eq(
            rig.player.sequence(section::SPECIAL_IDLE).unwrap(),
            &seq
        ));
        assert!(rig.player.time(section::SPECIAL_IDLE).unwrap() < 1.0);
        rig.overlay = None;
        for _ in 0..15 {
            drive(&mut rig, &mut lib, 0.1);
        }
        assert!(rig.scripted_idle.is_none());
        assert!(rig.player.sequence(section::SPECIAL_IDLE).is_none());
        drive(&mut rig, &mut lib, 0.2);
        assert!(rig.player.sequence(section::SPECIAL_IDLE).is_none());
        assert!((rig.pose_now(0.0)[1].translation[2] - 10.0).abs() < 1e-3);
    }

    /// A seated tree idle holds getting up only while it's starting
    /// (`00498f80`): Doc Mitchell's 16 s `SitChairRelaxA.kf` held his
    /// help-up in the opening until it had played out (B14).
    #[test]
    fn a_tree_idle_counts_as_starting_only_until_it_has_blended_in() {
        let mut rig = ActorRig::new(skeleton(10.0, 20.0), 1.0, 0.0);
        let mut lib = TestLib::new(&rig.skeleton, Vec::new());
        let relax = Arc::new(Sequence {
            name: "SitChairRelaxA".into(),
            start: 0.0,
            stop: 16.1,
            looping: false,
            accum_root: None,
            materials: Vec::new(),
            text_keys: vec![(0.0, "start".into()), (16.1, "end".into())],
            tracks: vec![Track {
                node: "Arm".into(),
                priority: 95,
                motion: Motion::Keys {
                    translation: vec![(0.0, [0.0, 0.0, 40.0])],
                    rotation: Vec::new(),
                    scale: Vec::new(),
                    default: (None, None, None),
                    euler: None,
                },
            }],
        });
        let mut life = crate::sitting::Life::default();
        let starting = |life: &crate::sitting::Life, rig: &ActorRig| {
            crate::sitting::special_idle_starting(life, rig, Some(&relax))
        };
        // No idle: nothing holds getting up.
        assert!(!starting(&life, &rig));
        // Taken by the clock, not yet played by the rig: starting.
        life.idles.playing = Some(world::idles::PlayingIdle {
            idle: esm::FormId(7),
            model: "SitChairRelaxA.kf".into(),
            length: 16.1,
            elapsed: 0.0,
            loops_left: 0,
        });
        assert!(starting(&life, &rig));
        // Playing, blending in from the pose: starting.
        rig.overlay = Some((relax.clone(), 0.0));
        drive(&mut rig, &mut lib, 0.0);
        assert!(starting(&life, &rig));
        // In at full weight with 15 s still to play: not starting.
        for frame in 1..=30 {
            rig.overlay = Some((relax.clone(), frame as f32 / 60.0));
            drive(&mut rig, &mut lib, 1.0 / 60.0);
        }
        assert!(!starting(&life, &rig));
        assert!(life.idles.playing.is_some());
    }

    #[test]
    fn starting_to_walk_blends_in_and_plays_at_the_actors_speed() {
        let mut rig = ActorRig::new(skeleton(10.0, 20.0), 1.0, 0.0);
        let mut lib = TestLib::new(&rig.skeleton, Vec::new());
        let arm = |rig: &ActorRig| rig.pose_now(0.0)[1].translation[2];
        // Standing: the idle, already in.
        assert!((arm(&rig) - 10.0).abs() < 1e-4, "{}", arm(&rig));
        // Walking at 77: the walk blends in over Blend:6 = 0.2 s, no jump.
        rig.walking = true;
        rig.speed = 77.0;
        drive(&mut rig, &mut lib, 0.0);
        assert!((arm(&rig) - 10.0).abs() < 1e-4, "{}", arm(&rig));
        drive(&mut rig, &mut lib, 0.1);
        let half = arm(&rig);
        assert!(half > 12.0 && half < 18.0, "{half}");
        drive(&mut rig, &mut lib, 0.1);
        assert!((arm(&rig) - 20.0).abs() < 1e-3, "{}", arm(&rig));
        assert_eq!(rig.player.playing(section::MOVEMENT), Some(group::FORWARD));
        // At 77 / 85 of the file's rate (85.3 units a second → 85).
        assert!((rig.player.movement_rate - 77.0 / 85.0).abs() < 1e-6);
        // Stopping: back to the idle over 0.2 s, no jump either.
        rig.walking = false;
        rig.speed = 0.0;
        drive(&mut rig, &mut lib, 0.0);
        assert!((arm(&rig) - 20.0).abs() < 1e-3, "{}", arm(&rig));
        drive(&mut rig, &mut lib, 0.1);
        let half = arm(&rig);
        assert!(half > 10.5 && half < 19.5, "{half}");
        drive(&mut rig, &mut lib, 0.2);
        assert!((arm(&rig) - 10.0).abs() < 1e-3, "{}", arm(&rig));
        assert_eq!(rig.player.playing(section::MOVEMENT), None);
    }

    #[test]
    fn turning_in_place_plays_the_turn_group_at_the_turns_scale() {
        // `mtturnleft.kf`: 1 s, looping, `Blend: 3`.
        let turn = Sequence {
            name: "TurnLeft".into(),
            start: 0.0,
            stop: 1.0,
            looping: true,
            accum_root: Some("Bip01".into()),
            materials: Vec::new(),
            text_keys: vec![
                (0.0, "start".into()),
                (0.03, "Blend: 3".into()),
                (1.0, "end".into()),
            ],
            tracks: vec![Track {
                node: "Arm".into(),
                priority: 30,
                motion: Motion::Keys {
                    translation: vec![(0.0, [0.0, 0.0, 30.0])],
                    rotation: Vec::new(),
                    scale: Vec::new(),
                    default: (None, None, None),
                    euler: None,
                },
            }],
        };
        let sk = skeleton(10.0, 20.0);
        let mut rig = ActorRig::new(sk, 1.0, 0.0);
        let mut lib = TestLib::new(
            &rig.skeleton,
            vec![("c\\locomotion\\mtturnleft.kf", Arc::new(turn))],
        );
        // Turning left on the spot at people's in-place scale 1.5: the
        // turn group, played 1.5 times as fast.
        rig.turning = Some((TurnSide::Left, 1.5));
        drive(&mut rig, &mut lib, 0.1);
        assert_eq!(
            rig.player.playing(section::MOVEMENT),
            Some(group::TURN_LEFT)
        );
        assert!((rig.player.movement_rate - 1.5).abs() < 1e-6);
        drive(&mut rig, &mut lib, 0.1);
        assert!((rig.pose_now(0.0)[1].translation[2] - 30.0).abs() < 1e-3);
        // No file for the group: the idle (`00495740`'s last fallback).
        rig.turning = Some((TurnSide::Right, 1.5));
        drive(&mut rig, &mut lib, 0.1);
        assert_eq!(rig.player.playing(section::MOVEMENT), None);
        // Walking takes the walk, whatever the turn.
        rig.turning = Some((TurnSide::Left, 1.5));
        rig.walking = true;
        rig.speed = 77.0;
        drive(&mut rig, &mut lib, 0.1);
        assert_eq!(rig.player.playing(section::MOVEMENT), Some(group::FORWARD));
        assert!((rig.player.movement_rate - 77.0 / 85.0).abs() < 1e-6);
    }

    #[test]
    fn backing_up_plays_its_own_group_at_the_forward_groups_rate() {
        let back = Sequence {
            name: "Backward".into(),
            start: 0.0,
            stop: 1.0,
            looping: true,
            accum_root: Some("Bip01".into()),
            materials: Vec::new(),
            text_keys: vec![(0.0, "start".into()), (1.0, "end".into())],
            tracks: vec![Track {
                node: "Arm".into(),
                priority: 30,
                motion: Motion::Keys {
                    translation: vec![(0.0, [0.0, 0.0, 40.0])],
                    rotation: Vec::new(),
                    scale: Vec::new(),
                    default: (None, None, None),
                    euler: None,
                },
            }],
        };
        let mut rig = ActorRig::new(skeleton(10.0, 20.0), 1.0, 0.0);
        let mut lib = TestLib::new(&rig.skeleton, Vec::new());
        rig.walking = true;
        rig.speed = 77.0;
        rig.direction = Some(MoveFlags {
            backward: true,
            ..Default::default()
        });
        // No file for the group: no movement animation, the idle shows.
        drive(&mut rig, &mut lib, 0.1);
        assert_eq!(rig.player.playing(section::MOVEMENT), None);
        // With one: it plays, at 77 over the Forward group's 85.
        let mut lib = TestLib::new(
            &rig.skeleton,
            vec![("c\\locomotion\\male\\mtbackward.kf", Arc::new(back))],
        );
        drive(&mut rig, &mut lib, 0.1);
        assert_eq!(rig.player.playing(section::MOVEMENT), Some(group::BACKWARD));
        assert!((rig.player.movement_rate - 77.0 / 85.0).abs() < 1e-6);
    }

    #[test]
    fn a_scaled_actor_plays_at_its_unscaled_rate() {
        let mut rig = ActorRig::new(skeleton(10.0, 20.0), 1.1, 0.0);
        let mut lib = TestLib::new(&rig.skeleton, Vec::new());
        rig.walking = true;
        rig.speed = 77.0 * 1.1;
        drive(&mut rig, &mut lib, 0.1);
        assert!((rig.player.movement_rate - 77.0 / 85.0).abs() < 1e-5);
    }
}

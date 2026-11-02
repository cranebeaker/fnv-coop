//! First-person camera tracks in package-requested special idles.
//!
//! The package lifecycle and request gate are in `world`; this layer loads
//! the actual skeleton/KF and presents Camera1st. It does not move the body.

use bevy::prelude::*;
use world::animation::{self, section};
use world::dialogue::PLAYER_REF;

use crate::dialogue::DialogueState;
use crate::{FlyCamera, GameFiles};

#[derive(Resource)]
pub struct PlayerIdle {
    pub requests: Vec<esm::FormId>,
    /// Deferred NPC script requests: the process holds one IDLE pointer,
    /// so another request before dispatch replaces it (005cb2d0).
    pub npc_requests: std::collections::HashMap<esm::FormId, esm::FormId>,
    bones: Vec<nif::Bone>,
    camera: Option<usize>,
    looking: Option<usize>,
    animation: animation::Player,
    package: Option<esm::FormId>,
    hand_follow: f32,
}

impl Default for PlayerIdle {
    fn default() -> Self {
        Self {
            requests: Vec::new(),
            npc_requests: Default::default(),
            bones: Vec::new(),
            camera: None,
            looking: None,
            animation: animation::Player::default(),
            package: None,
            hand_follow: 1.0,
        }
    }
}

impl PlayerIdle {
    pub fn snapshot(
        &self,
        seats: &crate::sitting::Seats,
        pitch: f32,
    ) -> Result<world::save::camera::Camera, String> {
        if !pitch.is_finite() || !self.hand_follow.is_finite() {
            return Err("non-finite player camera state".into());
        }
        let animation = if self.animation.is_empty() {
            None
        } else {
            Some(
                self.animation
                    .snapshot(&self.bones, |sequence| seats.sequence_path(sequence))
                    .map_err(|e| format!("player camera snapshot: {e:?}"))?,
            )
        };
        Ok(world::save::camera::Camera {
            animation,
            requests: self.requests.clone(),
            npc_requests: self.npc_requests.iter().map(|(&a, &b)| (a, b)).collect(),
            package: self.package,
            hand_follow: self.hand_follow,
            pitch,
        })
    }

    /// Prepare all assets before F9 commits any running world resources.
    pub fn restore(
        game: &cellview::Game,
        seats: &mut crate::sitting::Seats,
        saved: Option<&world::save::camera::Camera>,
    ) -> Result<Self, String> {
        let Some(saved) = saved else {
            return Ok(Self::default());
        };
        let mut idle = Self::default();
        if let Some(animation) = &saved.animation {
            if !idle.load(game) {
                return Err("saved player camera skeleton is unavailable".into());
            }
            idle.animation = animation
                .restore(&idle.bones, |path| seats.sequence(game, path))
                .map_err(|e| format!("saved player camera: {e:?}"))?;
        }
        idle.requests.clone_from(&saved.requests);
        idle.npc_requests = saved.npc_requests.iter().map(|(&a, &b)| (a, b)).collect();
        idle.package = saved.package;
        idle.hand_follow = saved.hand_follow;
        Ok(idle)
    }

    /// The first-person seated loop (the furniture's dynamic idle for the
    /// first-person view, `1stP_ChairDynamicIdle.kf`): played in the idle
    /// section, looping, so its `Camera1st` track places the seated eye;
    /// `None` eases it out (`004994f0`). The sit procedure
    /// (`sitting::player_furniture`) decides when.
    pub fn set_seated_loop(
        &mut self,
        game: &cellview::Game,
        sequence: Option<std::sync::Arc<nif::Sequence>>,
    ) {
        match sequence {
            Some(seq) if self.load(game) => {
                let PlayerIdle {
                    animation, bones, ..
                } = self;
                animation.play(animation::group::IDLE, &seq, -1, bones);
            }
            Some(_) => {}
            None => self.animation.stop_section(section::IDLE),
        }
    }

    fn load(&mut self, game: &cellview::Game) -> bool {
        if self.camera.is_some() && self.looking.is_some() {
            return true;
        }
        let path = format!("meshes\\{}", world::actor::FIRST_PERSON_SKELETON);
        let loaded = game
            .assets
            .read(&path)
            .ok()
            .flatten()
            .and_then(|bytes| nif::Nif::parse(bytes).ok())
            .and_then(|nif| nif.skeleton().ok());
        let Some(bones) = loaded else {
            eprintln!("Player idle: couldn't read {path}");
            return false;
        };
        self.camera = bones
            .iter()
            .position(|b| b.name.eq_ignore_ascii_case("Camera1st"));
        self.looking = bones
            .iter()
            .position(|b| b.name.eq_ignore_ascii_case("Bip01 Looking"));
        self.bones = bones;
        self.camera.is_some() && self.looking.is_some()
    }
}

/// Read the camera from the posed skeleton, leaving the physical actor fixed.
/// First-person skeleton root follows actor position (`00888b50`, 00889812),
/// Camera1st supplies its world rotation/position (`0094ae40`, 0094b1a9).
/// `Bip01 Looking` takes the input pitch (`00952290`), root takes heading.
fn camera_transform(pose: &nif::Transform) -> Transform {
    let forward = Vec3::from(cellview::space::direction(
        pose.apply_direction([0.0, 1.0, 0.0]),
    ));
    let up = Vec3::from(cellview::space::direction(
        pose.apply_direction([0.0, 0.0, 1.0]),
    ));
    Transform::from_translation(Vec3::from(cellview::space::point(pose.translation)))
        .looking_to(forward, up)
}

#[allow(clippy::too_many_arguments)]
pub fn animate(
    mut idle: ResMut<PlayerIdle>,
    time: Res<Time>,
    game: Res<GameFiles>,
    mut state: ResMut<DialogueState>,
    player: Res<crate::walk::Player>,
    settings: Res<crate::actors::AnimSettings>,
    menus: Res<crate::menus::Menus>,
    attack: Res<crate::combat::PlayerAttack>,
    conversation: Res<crate::dialogue::Conversation>,
    vats: Res<crate::vats::Vats>,
    mut seats: ResMut<crate::sitting::Seats>,
    mut cameras: Query<(&mut Transform, &FlyCamera)>,
) {
    // Removing the scripted package releases the view. F9 restores the
    // cached package together with the clocks, including a pending release.
    let package = state.0.script_packages.get(&PLAYER_REF).copied();
    if idle.package.is_some() && package.is_none() {
        idle.animation.free_special_idle();
    }
    idle.package = package;
    idle.animation.settings = settings.0;
    if !idle.requests.is_empty() && !idle.load(&game.0) {
        idle.requests.clear();
    }
    if !idle.requests.is_empty() {
        for request in std::mem::take(&mut idle.requests) {
            let Some(record) = seats.tree.get(request).cloned() else {
                continue;
            };
            // This path presents leaf special-idle camera tracks. Other
            // groups/tree traversal need their own dispatch, not a substitute.
            if record.group() != section::SPECIAL_IDLE || !record.conditions.is_empty() {
                eprintln!("Player idle {request}: group/condition dispatch pending");
                continue;
            }
            let Some(sequence) = seats.sequence(&game.0, &record.model) else {
                continue;
            };
            if !sequence
                .tracks
                .iter()
                .any(|t| t.node.eq_ignore_ascii_case("Camera1st"))
            {
                eprintln!("Player idle {request}: no Camera1st track");
                continue;
            }
            let loops =
                record.extra_loops(|lo, hi| lo + (state.0.roll() % (u64::from(hi - lo) + 1)) as u8);
            let loops = if loops == 255 { -1 } else { i32::from(loops) };
            let PlayerIdle {
                animation, bones, ..
            } = &mut *idle;
            if animation.request_special_idle(&sequence, loops, bones) {
                println!("Player camera idle: {}", record.editor_id);
            }
        }
    }
    if idle.camera.is_none() || !player.ready || !player.walking {
        return;
    }
    if !menus.is_open() {
        let dt = time.delta_secs();
        idle.animation.update(dt);
        let setting =
            |name, default| world::scripting::game_setting(&game.0.order, name).unwrap_or(default);
        let target = if attack.out {
            1.0
        } else {
            setting("fFirstPersonHandFollowMult", 0.85)
        };
        let chase = if target == 1.0 {
            setting("fFirstPersonHandChaseSecondsAttack", 0.05)
        } else {
            setting("fFirstPersonHandChaseSeconds", 2.0)
        };
        idle.hand_follow = animation::camera::chase_follow(idle.hand_follow, target, dt, chase);
    }
    if idle.animation.all().is_empty() {
        return;
    }
    // The holstered package-camera path is the opening's supported case.
    // Armed first-person groups and Pip-Boy hand-follow targets need their
    // own group/state dispatch before this can present those combinations.
    // The interactive-dialogue override also requires a process flag
    // (+0x364, 00933840) we do not yet model. Leave that combination out;
    // line-only Say/SayTo speech used during the opening has no menu.
    if attack.out
        || vats.shot_view()
        || conversation
            .0
            .as_ref()
            .is_some_and(|talk| !talk.is_line_only())
    {
        return;
    }
    let pose = idle.animation.pose(&idle.bones);
    let (Some(camera), Some(looking)) = (idle.camera, idle.looking) else {
        return;
    };
    if let Ok((mut transform, input)) = cameras.single_mut() {
        let transform_in_game = animation::camera::first_person_camera(
            &pose[camera],
            pose[looking].translation,
            player.character.feet,
            -input.yaw,
            input.pitch,
            idle.hand_follow,
            true,
        );
        *transform = camera_transform(&transform_in_game);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn camera_tracks_move_the_view_relative_to_fixed_feet() {
        let bone = nif::Transform {
            translation: [10.0, 20.0, 40.0],
            ..nif::Transform::IDENTITY
        };
        let feet = [100.0, 200.0, 300.0];
        let posed = animation::camera::first_person_camera(
            &bone,
            [0.0, 0.0, 118.0],
            feet,
            std::f32::consts::FRAC_PI_2,
            0.0,
            0.85,
            true,
        );
        let view = camera_transform(&posed);
        let eye = crate::walk::game_point(view.translation);
        for (got, want) in eye.into_iter().zip([120.0, 190.0, 340.0]) {
            assert!((got - want).abs() < 0.001);
        }
        assert!(view.forward().x > 0.999);
        assert_eq!(feet, [100.0, 200.0, 300.0]);
    }
}

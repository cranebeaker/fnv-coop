//! V.A.T.S. in the viewer, run as the game runs it. The rules are
//! `world::vats` (chances, costs, the queue), the menu's view and the camera
//! shots `world::vats_camera`, the menu itself `ui::vats` (drawn by `hud`
//! over the HUD); findings in `%USERPROFILE%\nv-re\findings\vats.md` and
//! `vats_camera_menu.md`.
//!
//! - V opens it (`00942800`, `007e9200`) on the target V.A.T.S. picks
//!   first; the others are listed by their bearing from the player
//!   (`007f0500`). The game's world stops while its menu is up (MenuMode:
//!   the main loop `0086e650` skips the clock, actors and physics): Bevy's
//!   virtual clock is paused, and the menu, its animations and its camera
//!   run on real time.
//! - Mode 1 while V is held after opening, 2 once it's let go, 3 while the
//!   target is scanned; V pressed again goes back to 1 (`007ec810`). The
//!   player is turned toward the target and the view zoomed on it, eased
//!   (`world::vats_camera::MenuView`); the part labels wait for both.
//! - Keys as the PC game binds them: A / D the previous / next target
//!   (`UIVATSSelectTarget`, or `UIVATSEnterFail` with only one), W / S the
//!   next / previous part whose label shows (`UIVATSSelectTargetPart`), the
//!   left button queues an attack on the part (`UIVATSMove`), B or the right
//!   button takes the last one back (`UIMenuCancel`; with nothing queued it
//!   leaves), R / F the special attacks, E plays the queue (`UIVATSExit`,
//!   `UIVATSReady`; with nothing queued it leaves).
//! - Playback (`009c7240`, `world::vats_camera::Playback`): each attack
//!   picks its camera path (`CPTH`, its conditions asked about the player
//!   and the target) and plays its shots (`CAMS`): the world at the shot's
//!   multiplier, the player and the target at theirs (every vanilla player
//!   attack gets `SimpleFrontHit01`: the world at ¼, the player × 4, the
//!   target × 3, from the moment the shot lands); the camera at the shot's
//!   animated model (`meshes\vatscameras\*.nif`) on its node, turned by the
//!   heading, looking at the target with the model's own angles laid on,
//!   its field of view keyed; the shot's image space modifier on. The
//!   camera is kept out of walls as the game's chase camera is
//!   (`0094a0c0`, `world::vats_camera::ChaseDistance`): a sphere
//!   `fCameraCasterSize` wide cast through the cell's collision from the
//!   shot's node toward where the model wants the camera. Before a shot
//!   starts the player's own view, turned at the part (`009445b0` mode
//!   4). Then `fVATSPlaybackDelay` of the world's time, and off.
//! - Each attack's camera path conditions can ask the smart camera checks
//!   (`GetVATS<Side>AreaFree`, `…TargetVisible`): lines cast here as
//!   `008bd830` / `008bdbd0` cast them, the rules in `world::vats`.
//! - A gun reloads itself in playback when its clip runs out, as it does
//!   with the attack control held; the menu charges the reload on the
//!   attack that empties the clip and lists "Reload" after it.
//! - The menu is open to the scripts as `MenuMode 1056`.
//! - Action points are taken as each attack is done; they come back only
//!   once V.A.T.S. is off.
//!
//! Not the game's (each also in the project notes): the scan's part
//! visibility (the game counts pixels with occlusion queries; here the
//! share of points along a part's capsules a line from the eye reaches),
//! read two frames after the target comes up; shots are hitscan (the
//! game's fly as missiles in V.A.T.S.), so a shot's projectile is there for
//! the frame it's fired and gone the next; the attack's shots spaced by the
//! weapon's fire rate in the player's time (the game's attack animation
//! fires them); a melee blow lands as it's swung; the target's
//! multiplier speeds its animations only (its movement and AI timers run
//! at the world's); bounds from `OBND` (`vats_camera::bound_sphere`); the
//! third-person view the game keeps between shots isn't drawn (no player
//! body): the player's view shows there. Not done: the scan highlight, the
//! per-target light, mouse picking and edge panning, the damage preview,
//! quaternion-keyed cameras' turns, the attacker's bone
//! for shots that follow it (such shots aren't placed), Mysterious
//! Stranger and Miss Fortune, the kill camera's own paths, image space
//! cross-fades, grenades and thrown weapons, objects as targets.

use std::collections::HashMap;
use std::sync::Arc;

use bevy::prelude::*;
use bevy::time::{Real, Virtual};
use cellview::space;
use esm::FormId;
use nif::camera::CameraModel;
use world::body_parts::BodyPartData;
use world::combat::{self, Weapon};
use world::dialogue::PLAYER_REF;
use world::scripting::{Facts, GameState, Runner};
use world::vats::{
    self, kind, mode, AttackFacts, Attempt, Candidate, ChanceQuery, LineHit, PartEntry, Plan,
    QueuedAttack, Settings, Side, SmartCamera, SmartCameraSettings, Stance,
};
use world::vats_camera::{
    self as cams, AttackNow, CameraPaths, CameraSettings, CameraShot, ChaseDistance, ChaseSettings,
    MenuView, MenuViewInput, NodesThere, Playback, PlaybackEvent, PlaybackInput, ShotNode,
};

use crate::actors::ActorRig;
use crate::ai::Walker;
use crate::combat::{first_met, tell_hit, Met, PlayerAttack};
use crate::dialogue::{Conversation, DialogueState, Talkers};
use crate::menus::Menus;
use crate::scripts::{CellScripts, Scripts};
use crate::sounds::SoundRequests;
use crate::walk::{game_point, CellCollision, Player};
use crate::{FlyCamera, GameFiles};

/// How long a scan waits before parts never seen get the average
/// (`007f3e00`: 2000 ms).
const SCAN_SECONDS: f64 = 2.0;

/// Frames from a target coming up to its parts' visibility being read
/// (the game's occlusion queries answer a frame or more later; a guess).
const QUERY_FRAMES: u32 = 2;

/// `--vats`: seconds after loading before V.A.T.S. opens by itself.
const AUTO_AFTER: f64 = 3.0;

#[derive(Debug, Clone, Copy, Default, PartialEq)]
enum Phase {
    #[default]
    Off,
    Menu,
    Playback,
}

/// A target in the menu.
struct Target {
    reference: FormId,
    /// Their feet, the distance from the player's, and between the edges.
    position: [f32; 3],
    distance: f32,
    gap: f32,
    radius: f32,
    parts: Vec<PartEntry>,
    /// When (real seconds) its scan began; whether it's done.
    scan_started: Option<f64>,
    scanned: bool,
}

/// The attack playing.
struct Playing {
    /// Which of the queue's attacks (counted from the first).
    id: u64,
    attack: QueuedAttack,
    hit: bool,
    special: Option<String>,
    /// Shots (or swings) still to come, the next one's time and when the
    /// attack's own animation is over (the player's clock).
    shots_left: u32,
    next_shot: f32,
    ends: Option<f32>,
    /// Melee: the blow has landed.
    struck: bool,
}

/// The camera shot on screen (`0058c5d0`, `0058cf60`).
struct ShotView {
    shot: CameraShot,
    model: Option<Arc<CameraModel>>,
    /// The camera clock when it started (the time-zero shift).
    start_clock: f32,
    location: Option<ShotNode>,
    look: ShotNode,
    /// The located-at actor's heading and the location's place when the
    /// shot began (`0058cbc0`).
    start_heading: f32,
    start_position: Option<[f32; 3]>,
    /// Where the camera was before it (the dolly's start).
    from: [f32; 3],
}

/// V.A.T.S. on screen.
#[derive(Resource, Default)]
pub struct Vats {
    phase: Phase,
    /// The menu's mode (1–3).
    mode: u8,
    settings: Option<Settings>,
    cam: Option<CameraSettings>,
    paths: Option<CameraPaths>,
    models: HashMap<String, Option<Arc<CameraModel>>>,
    weapon: Option<Weapon>,
    targets: Vec<Target>,
    selected: usize,
    part: usize,
    plan: Plan,
    /// Each queued attack's special's name, if it's one.
    specials_queued: Vec<Option<String>>,
    concentrated_fire: bool,
    paralyzing_palm: bool,
    /// The part's actor value last aimed at (`[011a59f0]`: the head to
    /// begin with).
    last_aimed: i32,
    view: Option<MenuView>,
    /// Where the menu aims at the target (`[011e0bd0]`), set when it comes
    /// up.
    aim: Option<(FormId, [f32; 3])>,
    /// Frames since the target came up in modes 2–3.
    frames_on_target: u32,
    // Playback.
    playback: Playback,
    playing: Option<Playing>,
    /// The attack last started, which the shot keeps looking at through
    /// the playback delay.
    last_attack: Option<QueuedAttack>,
    /// Attacks done so far (the next one's id).
    done: u64,
    kills: u32,
    /// The player's own clock (the world's time × the player's
    /// multiplier) and when the weapon can fire again on it.
    player_clock: f32,
    weapon_ready: f32,
    player_mult: f32,
    shot_view: Option<ShotView>,
    third_person: bool,
    /// Where the camera was last frame (`[011e0808]`).
    camera_at: Option<[f32; 3]>,
    /// The shot's image space modifier on now.
    modifier: Option<FormId>,
    /// A projectile was fired this frame (hitscan: gone the next).
    fired_now: bool,
    /// The camera for `apply_shot_camera`: translation, rotation, vertical
    /// field of view.
    shot_camera: Option<(Vec3, Quat, f32)>,
    /// The target's time multiplier for `scale_target_time`.
    target_time: Option<(FormId, f32)>,
    /// The view's field of view to put back when V.A.T.S. ends.
    restore_fov: Option<f32>,
    /// The chase camera's distance from its pivot (`[011e0768]`, kept out
    /// of walls) and the settings it reads.
    chase: ChaseDistance,
    chase_settings: Option<ChaseSettings>,
    /// The smart camera checks' settings.
    smart: Option<SmartCameraSettings>,
    /// A reload under way in playback: when it's done, on the player's
    /// clock.
    reloading: Option<f32>,
    /// The free camera's eye when V.A.T.S. opened (pictures), put back.
    fly_eye: Option<[f32; 3]>,
    /// `--vats [N]`: open by itself, then queue and play N attacks.
    auto: Option<u32>,
    /// What the menu shows (drawn by `hud`), and the HUD's mask
    /// (`ui::hud::mask`).
    pub menu: Option<ui::vats::VatsInput>,
    pub hud_mask: Option<u32>,
    /// The first-person view's field of view (a 4:3 width, degrees) while
    /// the menu zooms.
    pub first_person_fov: Option<f32>,
}

impl Vats {
    pub fn new(auto: Option<u32>) -> Vats {
        Vats {
            auto,
            last_aimed: 25,
            playback: Playback::new(),
            player_mult: 1.0,
            ..Vats::default()
        }
    }

    /// Whether V.A.T.S. has the controls (choosing or playing).
    pub fn is_on(&self) -> bool {
        self.phase != Phase::Off
    }

    /// Whether the menu is up (choosing targets), for the tutorial
    /// manager (`VATSMenu`, 1056).
    pub fn in_menu(&self) -> bool {
        self.phase == Phase::Menu
    }

    /// The V.A.T.S. manager's mode (`[011f2250]`+8, `world::vats::mode`):
    /// off, the menu's mode (1 to 3) while it's open, playback while the
    /// queue plays.
    pub fn manager_mode(&self) -> u8 {
        match self.phase {
            Phase::Off => mode::OFF,
            Phase::Menu => self.mode.max(mode::MENU),
            Phase::Playback => mode::PLAYBACK,
        }
    }

    /// Whether a camera shot has the view (the first-person model hides).
    pub fn shot_view(&self) -> bool {
        self.shot_camera.is_some()
    }
}

/// A number 0 up to (not including) 1 from the game's dice.
fn unit(state: &mut GameState) -> f32 {
    (state.roll() % 1_000_000) as f32 / 1_000_000.0
}

/// Someone's walker and rig.
fn rig_of<'a>(
    rigs: &'a Query<(&Walker, &ActorRig)>,
    reference: FormId,
) -> Option<(&'a Walker, &'a ActorRig)> {
    rigs.iter().find(|(w, _)| w.reference == reference)
}

/// Where a node of someone's skeleton is now (game space).
fn node_point(
    walker: &Walker,
    rig: &ActorRig,
    pose: &[nif::Transform],
    name: &str,
) -> Option<[f32; 3]> {
    let i = rig
        .skeleton
        .bones
        .iter()
        .position(|b| b.name.eq_ignore_ascii_case(name))?;
    Some(walker.placement().apply_point(pose.get(i)?.translation))
}

/// Where a game-space point is on screen (logical pixels), if it's inside
/// it (`007f52c0`: strictly within 0..1 both ways).
fn on_screen(camera: &Camera, global: &GlobalTransform, p: [f32; 3]) -> Option<Vec2> {
    let v = camera
        .world_to_viewport(global, Vec3::from(space::point(p)))
        .ok()?;
    let size = camera.logical_viewport_size()?;
    (v.x > 0.0 && v.y > 0.0 && v.x < size.x && v.y < size.y).then_some(v)
}

/// Where a point lands on the screen, 0–1 across and 0–1 up (`0045c670`),
/// on screen or not; `None` behind the camera.
fn screen_fraction(camera: &Camera, global: &GlobalTransform, p: [f32; 3]) -> Option<[f32; 2]> {
    let v = camera
        .world_to_viewport(global, Vec3::from(space::point(p)))
        .ok()?;
    let size = camera.logical_viewport_size()?;
    Some([v.x / size.x, 1.0 - v.y / size.y])
}

fn distance(a: [f32; 3], b: [f32; 3]) -> f32 {
    (0..3).map(|i| (a[i] - b[i]).powi(2)).sum::<f32>().sqrt()
}

fn unit_toward(from: [f32; 3], to: [f32; 3]) -> [f32; 3] {
    let d = distance(from, to).max(1e-4);
    [0, 1, 2].map(|i| (to[i] - from[i]) / d)
}

/// The view's heading (clockwise from north) and pitch (up) for a
/// direction.
fn angles(dir: [f32; 3]) -> (f32, f32) {
    (dir[0].atan2(dir[1]), dir[2].clamp(-1.0, 1.0).asin())
}

/// The direction for a heading and pitch.
fn direction(heading: f32, pitch: f32) -> [f32; 3] {
    [
        heading.sin() * pitch.cos(),
        heading.cos() * pitch.cos(),
        pitch.sin(),
    ]
}

/// Turns the view to a heading and pitch.
fn turn_to(fly: &mut FlyCamera, heading: f32, pitch: f32) {
    fly.yaw = space::heading_to_yaw(heading);
    fly.pitch = pitch.clamp(-1.54, 1.54);
}

/// The view's heading and pitch now.
fn view_angles(fly: &FlyCamera) -> (f32, f32) {
    (-fly.yaw, fly.pitch)
}

/// Points along the capsules of someone's part (their ragdoll's bodies
/// whose bones give that part), with each capsule's radius; the part's node
/// alone when none do (the weapon, skeletons without bodies).
fn part_samples(
    walker: &Walker,
    rig: &ActorRig,
    pose: &[nif::Transform],
    data: Option<&BodyPartData>,
    slot: i8,
    node: Option<[f32; 3]>,
) -> Vec<([f32; 3], f32)> {
    let mut out = Vec::new();
    if let (Some(ragdoll), Some(data), Ok(slot)) =
        (rig.skeleton.ragdoll.as_ref(), data, u8::try_from(slot))
    {
        let placement = walker.placement();
        for (b, offset) in ragdoll.ragdoll.bodies.iter().zip(&ragdoll.offsets) {
            let Some((a, c, r)) = b.capsule else {
                continue;
            };
            if data.part_of_bone(&rig.skeleton.bones, b.bone) != Some(slot) {
                continue;
            }
            let Some(bone) = pose.get(b.bone) else {
                continue;
            };
            let frame = placement.then_child(bone).then_child(offset);
            let (a, c) = (frame.apply_point(a), frame.apply_point(c));
            for t in [0.0, 0.25, 0.5, 0.75, 1.0] {
                out.push(([0, 1, 2].map(|i| a[i] + (c[i] - a[i]) * t), r * frame.scale));
            }
        }
    }
    if out.is_empty() {
        out.extend(node.map(|p| (p, 0.0)));
    }
    out
}

/// Everyone's pose now, for what hides what.
type Poses<'a> = Vec<(&'a Walker, &'a ActorRig, Vec<nif::Transform>)>;

/// The share of a part's points a line from the eye reaches: on screen,
/// with no wall nearer and no other body (anyone else's, or another part
/// of the target's own) in front.
#[allow(clippy::too_many_arguments)]
fn visible_share(
    samples: &[([f32; 3], f32)],
    eye: [f32; 3],
    camera: (&Camera, &GlobalTransform),
    collision: &CellCollision,
    poses: &Poses<'_>,
    target: FormId,
    data: Option<&BodyPartData>,
    slot: i8,
) -> f32 {
    if samples.is_empty() {
        return 0.0;
    }
    let seen = samples
        .iter()
        .filter(|&&(p, r)| {
            if on_screen(camera.0, camera.1, p).is_none() {
                return false;
            }
            let d = distance(eye, p);
            let dir = unit_toward(eye, p);
            if collision
                .0
                .raycast(eye, dir, d)
                .is_some_and(|(t, _)| t < d - 2.0)
            {
                return false;
            }
            !poses.iter().any(|(walker, rig, pose)| {
                let Some(ragdoll) = rig.skeleton.ragdoll.as_ref() else {
                    return false;
                };
                let Some((t, bone)) = ragdoll.ray_hit(pose, &walker.placement(), eye, dir) else {
                    return false;
                };
                if t >= d - r - 1.0 {
                    return false;
                }
                let own_part = walker.reference == target
                    && data.and_then(|d| d.part_of_bone(&rig.skeleton.bones, bone))
                        == u8::try_from(slot).ok();
                !own_part
            })
        })
        .count();
    seen as f32 / samples.len() as f32
}

/// The part a new target opens on (`007eecb0` cases 2 and 3): its torso's
/// entry (actor value 26), else the first.
fn torso_or_first(parts: &[PartEntry]) -> usize {
    parts.iter().position(|p| p.actor_value == 26).unwrap_or(0)
}

/// Rounds of the weapon's ammunition the player carries (0 for weapons
/// that use none).
fn carried_rounds(order: &esm::LoadOrder, state: &GameState, w: &Weapon) -> u32 {
    w.ammo_in_use(order, state, PLAYER_REF)
        .map_or(0, |a| state.item_count(order, PLAYER_REF, a).max(0) as u32)
}

/// A reference's name as the menu shows it (`0055d520`).
fn display_name(order: &esm::LoadOrder, state: &GameState, reference: FormId) -> String {
    world::script_functions::full_name(order, state, reference).unwrap_or_default()
}

/// The game's "hostile" for the bracket's colour (as the HUD's compass
/// takes it): fighting the player, or attacking them on sight.
fn hostile(order: &esm::LoadOrder, state: &GameState, who: FormId) -> bool {
    state.combat.get(&who) == Some(&PLAYER_REF)
        || world::factions::attacks_on_sight(order, state, who, PLAYER_REF)
}

/// Plays a sound by its editor ID.
fn sound(order: &esm::LoadOrder, sounds: &mut SoundRequests, name: &str) {
    if let Some(s) = order.form_by_editor_id(name) {
        sounds.0.push(s);
    }
}

/// The view's default fields of view (4:3 widths, degrees): the world's
/// (the viewer's, `fDefaultFOV` 75 confirmed) and the first-person view's.
fn default_fovs() -> (f32, f32) {
    (
        cellview::GAME_FOV_DEGREES,
        crate::viewmodel::FIRST_PERSON_FOV_DEGREES,
    )
}

/// The system: V opens V.A.T.S.; the menu takes the keys while it's open;
/// the queue plays.
#[allow(clippy::too_many_arguments)]
pub fn run_vats(
    (real, mut virt): (Res<Time<Real>>, ResMut<Time<Virtual>>),
    (game, scripts): (Res<GameFiles>, Res<Scripts>),
    (mut keys, mut mouse): (
        ResMut<ButtonInput<KeyCode>>,
        ResMut<ButtonInput<MouseButton>>,
    ),
    mut vats: ResMut<Vats>,
    mut state: ResMut<DialogueState>,
    (mut player, menus, conversation): (ResMut<Player>, Res<Menus>, Res<Conversation>),
    (talkers, cell_scripts, collision): (Res<Talkers>, Res<CellScripts>, Res<CellCollision>),
    mut attack: ResMut<PlayerAttack>,
    mut sounds: ResMut<SoundRequests>,
    mut cameras: Query<(
        &mut FlyCamera,
        &mut Transform,
        &Camera,
        &GlobalTransform,
        &mut Projection,
    )>,
    rigs: Query<(&Walker, &ActorRig)>,
    mut prompts: Query<&mut Visibility, With<crate::walk::Prompt>>,
) {
    let order = &game.0.order;
    let vats = &mut *vats;
    let state = &mut state.0;
    let now = real.elapsed_secs_f64();
    let now_virtual = virt.elapsed_secs();
    let Ok((mut fly, mut transform, camera, global, mut projection)) = cameras.single_mut() else {
        return;
    };
    // The player's eye: walking, at their feet (the camera may still be on
    // last frame's shot); flying (pictures), where the camera was when
    // V.A.T.S. opened, which it goes back to.
    if let Some(e) = vats.fly_eye {
        transform.translation = Vec3::from(space::point(e));
        if vats.phase == Phase::Off {
            vats.fly_eye = None;
        }
    }
    let eye = if player.walking {
        let f = player.character.feet;
        [f[0], f[1], f[2] + cellview::EYE_HEIGHT]
    } else {
        game_point(transform.translation)
    };
    let s = vats
        .settings
        .get_or_insert_with(|| Settings::load(order))
        .clone();
    let cam = vats
        .cam
        .get_or_insert_with(|| CameraSettings::load(order))
        .clone();
    // The game's timer: the frame's real milliseconds, 10 to 166
    // (`00aa4ee0`).
    let real_dt = real.delta_secs().clamp(0.010, 0.166);
    vats.shot_camera = None;
    vats.target_time = None;

    match vats.phase {
        Phase::Off => {
            vats.menu = None;
            vats.hud_mask = None;
            vats.first_person_fov = None;
            if let Some(fov) = vats.restore_fov.take() {
                if let Projection::Perspective(p) = &mut *projection {
                    p.fov = fov;
                }
            }
            let auto_due = vats.auto.is_some() && now >= AUTO_AFTER && !rigs.is_empty();
            let key = keys.just_pressed(KeyCode::KeyV);
            let auto_due = auto_due && !key;
            // Walking (not the free camera); `--vats` also in pictures,
            // which fly unless `--walk` is given.
            if !(key || auto_due)
                || !(player.walking || auto_due)
                || !player.ready
                || state.dead.contains(&PLAYER_REF)
            {
                return;
            }
            let busy = menus.is_open() || conversation.0.is_some();
            let opened = open(
                order,
                &s,
                vats,
                state,
                &player,
                (&fly, &transform, camera, global),
                (&talkers, &collision, &rigs, now_virtual),
                &mut attack,
                &mut sounds,
                (busy, auto_due),
            );
            if opened {
                vats.fly_eye = (!player.walking).then_some(eye);
                vats.restore_fov = match &*projection {
                    Projection::Perspective(p) => Some(p.fov),
                    _ => None,
                };
                virt.pause();
                // The menu is open to the scripts (`MenuMode 1056`,
                // `0071e420`'s id for `VATSMenu`), whose `MenuMode` blocks
                // run as it opens (how often the game runs them while it
                // stays open isn't traced; once here).
                state.more.menu_open = Some(vats::VATS_MENU);
                Runner::new(order, &scripts.0, state).menu_mode(vats::VATS_MENU);
                // `VATSMenu::Create` asks for its help over itself
                // (`007e9200`: `ShowMessage(0x15, 1056, 512)`; 0x1B with
                // a pad, which V.A.T.S. here doesn't take).
                world::tutorial::ask(
                    order,
                    &mut state.tutorials,
                    world::tutorial::id::VATS_PC,
                    world::tutorial::menu::VATS,
                    world::tutorial::menu::DELAY,
                );
            }
        }
        Phase::Menu => {
            // One of the game's menus over it (its help) has the keys and
            // buttons.
            let keys_now = if menus.game_open {
                MenuKeys::default()
            } else {
                MenuKeys {
                    v_pressed: keys.just_pressed(KeyCode::KeyV),
                    v_held: keys.pressed(KeyCode::KeyV),
                    previous_target: keys.just_pressed(KeyCode::KeyA),
                    next_target: keys.just_pressed(KeyCode::KeyD),
                    next_part: keys.just_pressed(KeyCode::KeyW),
                    previous_part: keys.just_pressed(KeyCode::KeyS),
                    queue: mouse.just_pressed(MouseButton::Left),
                    undo: mouse.just_pressed(MouseButton::Right)
                        || keys.just_pressed(KeyCode::KeyB),
                    execute: keys.just_pressed(KeyCode::KeyE),
                    special: [
                        keys.just_pressed(KeyCode::KeyR),
                        keys.just_pressed(KeyCode::KeyF),
                    ],
                }
            };
            menu(
                order,
                (&s, &cam),
                vats,
                state,
                &keys_now,
                &mut sounds,
                (&mut fly, eye, camera, global, &mut projection),
                (&rigs, &collision, &player, now, now_virtual, real_dt),
            );
            if vats.phase != Phase::Menu && state.more.menu_open == Some(vats::VATS_MENU) {
                state.more.menu_open = None;
            }
            if vats.phase == Phase::Playback {
                // Out of the menu: the world runs again (at the shots'
                // pace, from the next frame).
                virt.unpause();
                vats.playback = Playback::new();
                vats.player_clock = 0.0;
                vats.weapon_ready = 0.0;
                vats.reloading = None;
                vats.player_mult = cam.player_time_mult;
                attack.sped_up = Some(cam.player_time_mult);
            } else if vats.phase == Phase::Off {
                virt.unpause();
                virt.set_relative_speed(1.0);
            }
        }
        Phase::Playback => {
            let world_dt = virt.delta_secs();
            let world = (&*talkers, &*cell_scripts, &*collision, &rigs);
            play(
                order,
                &scripts.0,
                (&s, &cam),
                vats,
                state,
                (&mut fly, eye, camera),
                &mut player,
                world,
                &mut attack,
                &mut sounds,
                (now_virtual, real_dt, world_dt),
                &game.0,
            );
            if vats.phase == Phase::Off {
                virt.set_relative_speed(1.0);
                attack.sped_up = None;
            } else {
                // The world's pace for the next frame (`00aa4db0`).
                virt.set_relative_speed(vats.playback.world_mult.max(1e-4));
            }
        }
    }
    // V.A.T.S. has the keyboard and mouse while it's on, and the viewer's
    // roll-over line goes (the game picks nothing under the crosshair in
    // its menus).
    if vats.is_on() {
        keys.reset_all();
        mouse.reset_all();
    }
    let want = if vats.is_on() {
        Visibility::Hidden
    } else {
        Visibility::Inherited
    };
    for mut v in &mut prompts {
        if *v != want {
            *v = want;
        }
    }
}

/// The keys the menu reads this frame.
#[derive(Default)]
struct MenuKeys {
    v_pressed: bool,
    v_held: bool,
    previous_target: bool,
    next_target: bool,
    next_part: bool,
    previous_part: bool,
    queue: bool,
    undo: bool,
    execute: bool,
    special: [bool; 2],
}

/// Opens V.A.T.S. (`00942800`, `007e9200`): who can be targeted
/// (`007f52c0`), the first target, the plan; mode 1, `UIVATSEnter`.
#[allow(clippy::too_many_arguments)]
fn open(
    order: &esm::LoadOrder,
    s: &Settings,
    vats: &mut Vats,
    state: &mut GameState,
    player: &Player,
    (fly, transform, camera, global): (&FlyCamera, &Transform, &Camera, &GlobalTransform),
    (talkers, collision, rigs, now_virtual): (
        &Talkers,
        &CellCollision,
        &Query<(&Walker, &ActorRig)>,
        f32,
    ),
    attack: &mut PlayerAttack,
    sounds: &mut SoundRequests,
    (busy, auto_due): (bool, bool),
) -> bool {
    let feet = player.character.feet;
    let weapon = combat::weapon_in_hand(order, state, PLAYER_REF);
    match vats::can_enter(order, state, weapon.as_ref(), false, busy) {
        Ok(()) => {}
        Err(vats::EnterRefusal::BannedWeapon) => {
            println!("V.A.T.S. can't be used with this weapon.");
            vats.auto = None;
            return false;
        }
        Err(_) => return false,
    }
    let interior = state.player_world.is_none();
    let (heading, _) = {
        let f = transform.forward().as_vec3();
        angles([f.x, -f.z, f.y])
    };
    let mut found: Vec<(Target, f32)> = Vec::new();
    for t in &talkers.0 {
        let Some((walker, rig)) = rig_of(rigs, t.reference) else {
            continue;
        };
        let position = walker.position;
        let pose = rig.pose_now(now_virtual);
        let data = BodyPartData::of(order, t.reference);
        let held = combat::weapon_in_hand(order, state, t.reference).filter(|_| !rig.disarmed);
        let parts = vats::parts_offered(data.as_ref(), held.as_ref());
        let seen = on_screen(camera, global, position).is_some()
            || parts.iter().any(|p| {
                node_point(walker, rig, &pose, &p.node)
                    .is_some_and(|n| on_screen(camera, global, n).is_some())
            });
        let d = distance(feet, position);
        let candidate = Candidate {
            reference: t.reference,
            alive: !state.dead.contains(&t.reference) && rig.ragdoll.is_none(),
            distance: d,
            line_of_sight: crate::fighting::clear_between(&collision.0, feet, position),
            on_screen: seen,
        };
        if !vats::eligible(order, s, &candidate, interior) {
            continue;
        }
        let radius = crate::fighting::Kit::read(order, state, walker, &rig.skeleton).radius;
        let toward = (position[0] - feet[0]).atan2(position[1] - feet[1]);
        let mut off = (toward - heading).to_degrees().rem_euclid(360.0);
        if off > 180.0 {
            off = 360.0 - off;
        }
        found.push((
            Target {
                reference: t.reference,
                position,
                distance: d,
                gap: d - radius - physics::CharacterShape::PLAYER.radius,
                radius,
                parts,
                scan_started: None,
                scanned: false,
            },
            off,
        ));
    }
    let picks: Vec<(f32, f32)> = found.iter().map(|(t, a)| (*a, t.distance)).collect();
    let Some(first) = vats::first_target(s, &picks) else {
        // `--vats` tries again until someone's there to target.
        if !auto_due {
            println!("V.A.T.S.: nothing to target.");
            sound(order, sounds, "UIVATSEnterFail");
        }
        return false;
    };
    let first_ref = found[first].0.reference;
    let mut targets: Vec<Target> = found.into_iter().map(|(t, _)| t).collect();
    // Listed by bearing (`007f0500`).
    targets.sort_by(|a, b| {
        cams::bearing(feet, a.position).total_cmp(&cams::bearing(feet, b.position))
    });
    vats.selected = targets
        .iter()
        .position(|t| t.reference == first_ref)
        .unwrap_or(0);
    vats.part = torso_or_first(&targets[vats.selected].parts);
    vats.targets = targets;
    vats.weapon = weapon;
    vats.kills = 0;
    vats.done = 0;
    vats.playing = None;
    vats.specials_queued.clear();
    vats.aim = None;
    vats.frames_on_target = 0;
    vats.third_person = false;
    vats.shot_view = None;
    vats.camera_at = None;
    vats.reloading = None;
    vats.chase = ChaseDistance::default();
    let carried = vats
        .weapon
        .as_ref()
        .map_or(0, |w| carried_rounds(order, state, w));
    let clip = vats.weapon.as_ref().map_or(0, |w| attack.clip(w, carried));
    vats.plan = Plan::new(
        vats::action_points(order, state),
        clip,
        carried.saturating_sub(clip),
    );
    vats.concentrated_fire = vats::concentrated_fire(order, state);
    vats.paralyzing_palm =
        world::perks::apply(order, state, world::perks::entry::HAS_PARALYZING_PALM, 0.0) > 0.0;
    let (h, p) = view_angles(fly);
    vats.view = Some(MenuView::new(h, p, default_fovs()));
    vats.phase = Phase::Menu;
    vats.mode = mode::MENU;
    if vats.auto == Some(0) {
        vats.auto = None;
    }
    vats::set_mode(state, mode::MENU);
    sound(order, sounds, "UIVATSEnter");
    println!(
        "V.A.T.S.: {} target{}, {:.0} action points.",
        vats.targets.len(),
        if vats.targets.len() == 1 { "" } else { "s" },
        vats.plan.ap_left
    );
    true
}

/// One frame of the menu (`007ec810`): the mode, the keys, the view, the
/// scan, and what it shows.
#[allow(clippy::too_many_arguments)]
fn menu(
    order: &esm::LoadOrder,
    (s, cam): (&Settings, &CameraSettings),
    vats: &mut Vats,
    state: &mut GameState,
    keys: &MenuKeys,
    sounds: &mut SoundRequests,
    (fly, eye, camera, global, projection): (
        &mut FlyCamera,
        [f32; 3],
        &Camera,
        &GlobalTransform,
        &mut Projection,
    ),
    (rigs, collision, player, now, now_virtual, real_dt): (
        &Query<(&Walker, &ActorRig)>,
        &CellCollision,
        &Player,
        f64,
        f32,
        f32,
    ),
) {
    // Leaving when the target is gone or dead, or the player dies
    // (`007ec810` → `007ebd50`).
    let target_ref = vats.targets[vats.selected].reference;
    if state.dead.contains(&PLAYER_REF)
        || state.dead.contains(&target_ref)
        || rig_of(rigs, target_ref).is_none()
    {
        leave(order, vats, state, sounds);
        return;
    }
    // The V key (`007ec810`): `--vats` lets go at once.
    let held = keys.v_held && vats.auto.is_none();
    let m = cams::menu_mode(vats.mode, keys.v_pressed, held);
    if m != vats.mode {
        vats.mode = m;
        state.vats.mode = m;
    }
    let settled = vats.view.as_ref().is_some_and(MenuView::settled);
    // Targets (`007eecb0` cases 2 and 3).
    if keys.previous_target || keys.next_target {
        let count = vats.targets.len();
        if count < 2 {
            sound(order, sounds, "UIVATSEnterFail");
        } else {
            sound(order, sounds, "UIVATSSelectTarget");
            vats.selected = if keys.next_target {
                (vats.selected + 1) % count
            } else {
                (vats.selected + count - 1) % count
            };
            let t = &mut vats.targets[vats.selected];
            if !t.scanned {
                t.scan_started = None;
                for p in &mut t.parts {
                    p.scanned = false;
                }
            }
            vats.part = torso_or_first(&t.parts);
            vats.aim = None;
            vats.frames_on_target = 0;
        }
    }
    // Parts (`007f3070`): ranged weapons on people, and while scanning.
    let melee = vats.weapon.as_ref().is_none_or(|w| w.is_melee());
    let part_selection = !melee;
    if (part_selection || vats.mode == mode::SCANNING) && (keys.next_part || keys.previous_part) {
        let shown = labels_shown(order, state, vats);
        let next = cams::step_part(&shown, vats.part, keys.next_part);
        if next != vats.part {
            vats.part = next;
            vats.last_aimed = vats.targets[vats.selected].parts[next].actor_value;
            sound(order, sounds, "UIVATSSelectTargetPart");
        }
    }
    let target = &vats.targets[vats.selected];
    let reference = target.reference;
    let Some((walker, rig)) = rig_of(rigs, reference) else {
        return;
    };
    let pose = rig.pose_now(now_virtual);
    // Where the menu aims (`009445b0`), set when the target comes up.
    let feet = player.character.feet;
    let base_aim = match vats.aim {
        Some((r, p)) if r == reference => p,
        _ => {
            let nodes: Vec<(i8, [f32; 3])> = target
                .parts
                .iter()
                .filter(|p| (0..15).contains(&p.slot))
                .filter_map(|p| Some((p.slot, node_point(walker, rig, &pose, &p.node)?)))
                .collect();
            let head = nodes.iter().find(|(slot, _)| *slot == 1).map(|(_, p)| *p);
            let points: Vec<[f32; 3]> = nodes.iter().map(|(_, p)| *p).collect();
            let p = cams::aim_point(head, &points, feet, walker.position);
            vats.aim = Some((reference, p));
            p
        }
    };
    let part_node = target
        .parts
        .get(vats.part)
        .and_then(|p| node_point(walker, rig, &pose, &p.node));
    let part = part_node.and_then(|n| screen_fraction(camera, global, n).map(|[_, v]| (v, n[2])));
    // The zoom on the target (`0095de30`).
    let (middle, radius) = cams::bound_sphere(order, reference, walker.position, walker.heading);
    let dlg_focus = 3.2;
    let fov = cams::target_fov(cam, radius, distance(eye, middle), dlg_focus);
    let zoom = cams::clamp_fovs(cam, fov, cellview::GAME_FOV_DEGREES);
    let input = MenuViewInput {
        mode: vats.mode,
        dt: real_dt,
        target: Some((u64::from(reference.0), base_aim)),
        part,
        eye,
        zoom: Some(zoom),
        default_fovs: default_fovs(),
    };
    if let Some(view) = vats.view.as_mut() {
        let (h, p) = view.frame(cam, &input);
        turn_to(fly, h, p);
        if let Projection::Perspective(pp) = projection {
            pp.fov = cellview::vertical_fov(view.world_fov);
        }
        vats.first_person_fov = Some(view.first_fov);
    }
    // The scan (`007f3e00`): in modes 2 and 3, once the parts' visibility
    // can be read, every part on screen; after 2 seconds the rest get the
    // average.
    if vats.mode == mode::READY || vats.mode == mode::SCANNING {
        vats.frames_on_target = vats.frames_on_target.saturating_add(1);
        if !vats.targets[vats.selected].scanned {
            if vats.mode != mode::SCANNING {
                vats.mode = mode::SCANNING;
                state.vats.mode = mode::SCANNING;
            }
            if vats.frames_on_target > QUERY_FRAMES {
                scan(
                    order,
                    s,
                    vats,
                    state,
                    (eye, camera, global),
                    (rigs, collision, now, now_virtual),
                );
            }
        }
    }
    let settled_now = vats.view.as_ref().is_some_and(MenuView::settled);
    let ready = vats.mode == mode::READY && settled_now;
    // `--vats N`: N attacks on the part it opened on, then play.
    let mut queue_now = u32::from(ready && keys.queue);
    let mut go = keys.execute;
    if let Some(n) = vats
        .auto
        .filter(|_| ready && vats.targets[vats.selected].scanned)
    {
        queue_now = n;
        go = true;
        vats.auto = None;
    }
    for _ in 0..queue_now {
        queue(order, s, vats, state, sounds, None);
    }
    for (i, pressed) in keys.special.iter().enumerate() {
        if *pressed && ready {
            let special = special_buttons(order, state, s, vats.weapon.as_ref())[i].clone();
            if let Some(sp) = special {
                queue(order, s, vats, state, sounds, Some(sp));
            }
        }
    }
    if keys.undo && vats.mode == mode::READY && settled {
        if vats.plan.undo().is_some() {
            vats.specials_queued.pop();
            sound(order, sounds, "UIMenuCancel");
        } else {
            // Nothing to take back: the menu closes (`007efa10` → `00705780`).
            leave(order, vats, state, sounds);
            return;
        }
    }
    if go {
        // `007ebd50`: the menu closes; with nothing queued V.A.T.S. ends.
        if vats.plan.attacks.is_empty() {
            leave(order, vats, state, sounds);
            return;
        }
        sound(order, sounds, "UIVATSExit");
        sound(order, sounds, "UIVATSReady");
        vats.phase = Phase::Playback;
        vats::set_mode(state, mode::PLAYBACK);
        vats.menu = None;
        vats.hud_mask = Some(ui::hud::mask::VATS_PLAYBACK);
        vats.first_person_fov = None;
        if let Some(fov) = vats.restore_fov {
            if let Projection::Perspective(p) = projection {
                p.fov = fov;
            }
        }
        return;
    }
    vats.hud_mask = Some(ui::hud::mask::VATS_MENU);
    vats.menu = Some(menu_input(
        order,
        s,
        vats,
        state,
        (camera, global),
        rigs,
        (now, now_virtual),
        part_selection,
    ));
}

/// Leaves with nothing queued (`007ebd50` → mode 0): `UIVATSExit`.
fn leave(
    order: &esm::LoadOrder,
    vats: &mut Vats,
    state: &mut GameState,
    sounds: &mut SoundRequests,
) {
    vats.phase = Phase::Off;
    vats.mode = mode::OFF;
    vats::set_mode(state, mode::OFF);
    vats.menu = None;
    vats.hud_mask = None;
    vats.first_person_fov = None;
    sound(order, sounds, "UIVATSExit");
}

/// Which of the selected target's labels show (`ui::vats::shared_labels`).
fn labels_shown(order: &esm::LoadOrder, state: &GameState, vats: &Vats) -> Vec<bool> {
    let views = part_views(order, state, vats, None);
    ui::vats::shared_labels(&views).0
}

/// The special attack buttons (`007eb920`): R the weapon's or the first
/// unarmed one, F Cross.
fn special_buttons(
    order: &esm::LoadOrder,
    state: &GameState,
    s: &Settings,
    weapon: Option<&Weapon>,
) -> [Option<vats::Special>; 2] {
    let mut out: [Option<vats::Special>; 2] = [None, None];
    for sp in vats::specials(order, state, s, weapon, false) {
        let slot = usize::from(sp.kind == kind::CROSS);
        if out[slot].is_none() {
            out[slot] = Some(sp);
        }
    }
    out
}

/// The scan of the selected target (`007f3e00`).
fn scan(
    order: &esm::LoadOrder,
    s: &Settings,
    vats: &mut Vats,
    state: &mut GameState,
    (eye, camera, global): ([f32; 3], &Camera, &GlobalTransform),
    (rigs, collision, now, now_virtual): (&Query<(&Walker, &ActorRig)>, &CellCollision, f64, f32),
) {
    let reference = vats.targets[vats.selected].reference;
    let Some((walker, rig)) = rig_of(rigs, reference) else {
        return;
    };
    let pose = rig.pose_now(now_virtual);
    let poses: Poses = rigs
        .iter()
        .filter(|(w, r)| r.ragdoll.is_none() && !state.dead.contains(&w.reference))
        .map(|(w, r)| (w, r, r.pose_now(now_virtual)))
        .collect();
    let data = BodyPartData::of(order, reference);
    let weapon = vats.weapon.clone();
    let target = &mut vats.targets[vats.selected];
    let started = *target.scan_started.get_or_insert(now);
    let gap = target.gap;
    for i in 0..target.parts.len() {
        if target.parts[i].scanned {
            continue;
        }
        let entry = target.parts[i].clone();
        let point = node_point(walker, rig, &pose, &entry.node);
        if point.is_none_or(|p| on_screen(camera, global, p).is_none()) {
            continue;
        }
        let samples = part_samples(walker, rig, &pose, data.as_ref(), entry.slot, point);
        let visible = visible_share(
            &samples,
            eye,
            (camera, global),
            collision,
            &poses,
            reference,
            data.as_ref(),
            entry.slot,
        );
        let bp = data
            .as_ref()
            .and_then(|d| u8::try_from(entry.slot).ok().and_then(|slot| d.part(slot)));
        let chance = vats::part_chance(
            order,
            state,
            s,
            &ChanceQuery {
                target: reference,
                part: bp,
                part_av: entry.actor_value,
                weapon: weapon.as_ref(),
                distance: gap,
                visible,
                stance: Stance::SCANNING,
            },
        );
        let p = &mut target.parts[i];
        if chance > 0.0 {
            p.chance = chance;
        }
        p.scanned = true;
    }
    let all = target.parts.iter().all(|p| p.scanned);
    if all || now - started >= SCAN_SECONDS {
        vats::fill_unscanned(&mut target.parts);
        vats::share_highest(&mut target.parts);
        target.scanned = true;
        vats.part = vats::default_part(&target.parts, gap, vats.last_aimed);
        vats.mode = mode::READY;
        state.vats.mode = mode::READY;
    }
}

/// The selected target's parts as the menu shows them.
fn part_views(
    order: &esm::LoadOrder,
    state: &GameState,
    vats: &Vats,
    screen: Option<(
        &Camera,
        &GlobalTransform,
        &Walker,
        &ActorRig,
        &[nif::Transform],
    )>,
) -> Vec<ui::vats::PartView> {
    let s = vats.settings.as_ref();
    let target = &vats.targets[vats.selected];
    let facts = Facts {
        order,
        state,
        speaker: None,
    };
    let held = combat::weapon_in_hand(order, state, target.reference);
    target
        .parts
        .iter()
        .map(|p| {
            let n = vats.plan.in_a_row(target.reference, p.actor_value);
            let percent = s.map_or(0, |s| {
                vats::shown_percent(s, p.chance, n, vats.concentrated_fire)
            });
            // `_MeterPercent`: the part's condition, a weapon its health.
            let meter = match u16::try_from(p.actor_value) {
                Ok(av) if (25..=31).contains(&av) => facts
                    .current_actor_value(target.reference, av)
                    .map_or(1.0, |c| (c / 100.0) as f32),
                _ if p.slot == 14 => held.as_ref().map_or(1.0, |w| {
                    combat::weapon_condition(state, target.reference, w.form_id)
                }),
                _ => 1.0,
            };
            let at = screen.and_then(|(camera, global, walker, rig, pose)| {
                node_point(walker, rig, pose, &p.node)
                    .and_then(|n| screen_fraction(camera, global, n))
            });
            ui::vats::PartView {
                part_type: i32::from(p.slot),
                group: p.actor_value,
                name: p.name.clone(),
                percent: format!("{percent}%"),
                meter: meter.clamp(0.0, 1.0),
                screen: at,
            }
        })
        .collect()
}

/// What the menu shows this frame (`007ec810`).
#[allow(clippy::too_many_arguments)]
fn menu_input(
    order: &esm::LoadOrder,
    s: &Settings,
    vats: &Vats,
    state: &GameState,
    (camera, global): (&Camera, &GlobalTransform),
    rigs: &Query<(&Walker, &ActorRig)>,
    (now, now_virtual): (f64, f32),
    part_selection: bool,
) -> ui::vats::VatsInput {
    let target = &vats.targets[vats.selected];
    let rig = rig_of(rigs, target.reference);
    let pose = rig.map(|(_, r)| r.pose_now(now_virtual));
    let screen = rig
        .zip(pose.as_deref())
        .map(|((w, r), p)| (camera, global, w, r, p));
    let parts = part_views(order, state, vats, screen);
    let health = |who| {
        let now = combat::health(order, state, who).unwrap_or(0.0) as f32;
        let full = combat::max_health(order, state, who).unwrap_or(0.0) as f32;
        (now, full)
    };
    let (th, tf) = health(target.reference);
    let share = if tf > 0.0 { th / tf } else { 0.0 };
    let (ph, pf) = health(PLAYER_REF);
    let weapon = vats.weapon.as_ref().map(|w| ui::vats::WeaponView {
        ammo: (!w.is_melee() && !w.ammo.is_empty())
            .then_some((vats.plan.clip as i32, vats.plan.reserve as i32)),
        condition: combat::weapon_condition(state, PLAYER_REF, w.form_id),
    });
    let cost = vats::attack_cost(order, state, s, vats.weapon.as_ref());
    let specials = special_buttons(order, state, s, vats.weapon.as_ref());
    ui::vats::VatsInput {
        time: now,
        mode: vats.mode,
        settled: vats.view.as_ref().is_some_and(MenuView::settled),
        part_selection,
        health: ph,
        health_max: pf,
        dead: false,
        action_points: vats.plan.ap_left,
        action_points_max: vats::max_action_points(order, state),
        cost,
        hovering: false,
        weapon,
        compass: Default::default(),
        target: Some(ui::vats::TargetView {
            key: u64::from(target.reference.0),
            name: display_name(order, state, target.reference),
            is_actor: true,
            alive: !state.dead.contains(&target.reference),
            hostile: hostile(order, state, target.reference),
            health: share,
            // The damage preview isn't worked out (`007f48e0`).
            health_after: share,
            parts,
            selected: vats.part,
        }),
        queue: queue_lines(order, state, vats),
        specials: specials.map(|sp| sp.map(|sp| sp.name)),
    }
}

/// The queue's lines (`007efa10`): "<target>: <part>" ("---" for a part
/// not found), ": <special>" after a special's, and a "Reload" line after
/// an attack with a reload.
fn queue_lines(order: &esm::LoadOrder, state: &GameState, vats: &Vats) -> Vec<String> {
    let mut out = Vec::new();
    for (a, special) in vats.plan.attacks.iter().zip(&vats.specials_queued) {
        let name = display_name(order, state, a.target);
        let part = vats
            .targets
            .iter()
            .find(|t| t.reference == a.target)
            .and_then(|t| t.parts.iter().find(|p| p.slot == a.slot))
            .map_or_else(|| "---".to_string(), |p| p.name.clone());
        let mut line = format!("{name}: {part}");
        if let Some(sp) = special {
            line.push_str(": ");
            line.push_str(sp);
        }
        out.push(line);
        if a.reload {
            out.push(vats::kind_label(kind::RELOAD).to_string());
        }
    }
    out
}

/// Queues an attack (or a special) on the selected part (`007ec810`,
/// `007efa10`): `UIVATSMove`, or the refusal's message and sound.
fn queue(
    order: &esm::LoadOrder,
    s: &Settings,
    vats: &mut Vats,
    state: &mut GameState,
    sounds: &mut SoundRequests,
    special: Option<vats::Special>,
) {
    let target = &vats.targets[vats.selected];
    let Some(entry) = target.parts.get(vats.part) else {
        return;
    };
    let weapon = vats.weapon.as_ref();
    let k = vats::attack_kind(weapon.map(|w| w.animation));
    let (kind, cost) = match &special {
        Some(sp) => (sp.kind, sp.cost),
        None => (k, vats::attack_cost(order, state, s, weapon)),
    };
    let attempt = Attempt {
        target: target.reference,
        is_actor: true,
        slot: entry.slot,
        part_av: entry.actor_value,
        chance: entry.chance,
        kind,
        cost,
        shots: vats::shots(s, weapon),
        melee: weapon.is_none_or(|w| w.is_melee()),
        unarmed: weapon.is_none_or(|w| w.animation == 0),
        clip_size: weapon
            .filter(|w| !w.is_melee() && !w.ammo.is_empty())
            .map(|w| w.clip),
        ammo_use: weapon.map_or(1, |w| w.ammo_use.max(1)),
        thrown_left: None,
        parts: target
            .parts
            .iter()
            .map(|p| (p.slot, p.actor_value))
            .collect(),
        concentrated_fire: vats.concentrated_fire,
        paralyzing_palm: vats.paralyzing_palm,
        always_hit: false,
    };
    let selected = entry.actor_value;
    let result = vats.plan.queue(s, &attempt, &mut || unit(state)).cloned();
    match result {
        Ok(a) => {
            vats.last_aimed = selected;
            vats.specials_queued.push(special.map(|sp| sp.name));
            sound(order, sounds, "UIVATSMove");
            // The roll stays hidden in the menu, as in the game; the
            // console says it.
            println!(
                "V.A.T.S.: queued: {} ({:.0} action points{}): {}",
                vats::kind_label(a.kind),
                a.ap,
                if a.reload { ", a reload after it" } else { "" },
                if a.hit { "a hit" } else { "a miss" }
            );
        }
        Err(why) => {
            if let Some(m) = why.message() {
                println!("V.A.T.S.: {m}");
            }
            if let Some(snd) = why.sound() {
                sound(order, sounds, snd);
            }
        }
    }
}

/// One frame of playback (`009445b0` mode 4, `009c7240`).
#[allow(clippy::too_many_arguments)]
fn play(
    order: &esm::LoadOrder,
    scripts: &world::scripting::ScriptCache,
    (s, cam): (&Settings, &CameraSettings),
    vats: &mut Vats,
    state: &mut GameState,
    (fly, eye, camera): (&mut FlyCamera, [f32; 3], &Camera),
    player: &mut Player,
    (talkers, cell_scripts, collision, rigs): (
        &Talkers,
        &CellScripts,
        &CellCollision,
        &Query<(&Walker, &ActorRig)>,
    ),
    attack: &mut PlayerAttack,
    sounds: &mut SoundRequests,
    (now_virtual, real_dt, world_dt): (f32, f32, f32),
    game: &cellview::Game,
) {
    // The player's own time.
    vats.player_clock += world_dt * vats.player_mult;
    if state.dead.contains(&PLAYER_REF) && !vats.plan.attacks.is_empty() {
        vats.plan.attacks.clear();
        vats.specials_queued.clear();
        vats.playing = None;
    }
    // Attacks on someone who has died are dropped, their action points
    // kept (`009c86b0`).
    if vats.playing.is_none() {
        while vats
            .plan
            .attacks
            .first()
            .is_some_and(|a| state.dead.contains(&a.target))
        {
            vats.plan.attacks.remove(0);
            vats.specials_queued.remove(0);
            vats.done += 1;
        }
    }
    // The next attack.
    if vats.playing.is_none() && !vats.plan.attacks.is_empty() {
        let heading = view_angles(fly).0;
        start_attack(
            order,
            s,
            vats,
            state,
            (eye, heading, collision, rigs),
            player,
            now_virtual,
        );
    }
    // Turned straight at the attack's part (`009445b0` mode 4).
    if let Some(p) = vats.playing.as_ref() {
        if let Some((walker, rig)) = rig_of(rigs, p.attack.target) {
            let pose = rig.pose_now(now_virtual);
            let node = aim_point(&vats.targets, walker, rig, &pose, &p.attack);
            let f = player.character.feet;
            let eye_now = [f[0], f[1], f[2] + cellview::EYE_HEIGHT];
            let (h, pitch) = angles(unit_toward(eye_now, node));
            turn_to(fly, h, pitch);
        }
    }
    // Shots and blows due now.
    vats.fired_now = false;
    let clock = vats.player_clock;
    let weapon = vats.weapon.clone();
    let interval = attack_interval(weapon.as_ref());
    // The gun reloads itself when its clip runs out, as it does with the
    // attack control held outside V.A.T.S. (playback holds it, `009c7240`
    // → `00a24280(6)`): the next shot waits for the reload. Done: a full
    // clip.
    let gun = weapon
        .as_ref()
        .filter(|w| !w.is_melee() && !w.ammo.is_empty());
    if let Some(done) = vats.reloading {
        if clock >= done {
            if let Some(w) = gun {
                let carried = carried_rounds(order, state, w);
                attack.set_clip(w, w.clip.min(carried));
            }
            vats.reloading = None;
        }
    }
    let needs_reload = |attack: &PlayerAttack, state: &GameState| {
        gun.is_some_and(|w| {
            let carried = carried_rounds(order, state, w);
            let use_ = u32::from(w.ammo_use.max(1));
            attack.clip(w, carried) < use_ && carried >= use_
        })
    };
    let due = vats.reloading.is_none()
        && vats
            .playing
            .as_ref()
            .is_some_and(|p| p.shots_left > 0 && clock >= p.next_shot);
    if due && needs_reload(attack, state) {
        let time = gun.map_or(0.0, |w| w.reload_time);
        vats.reloading = Some(clock + time);
        vats.weapon_ready = clock + time;
        attack.reload_started = Some(now_virtual);
        if let Some(p) = vats.playing.as_mut() {
            p.next_shot = clock + time;
        }
    } else if due {
        let (a, hit) = {
            let p = vats.playing.as_mut().expect("due");
            p.shots_left -= 1;
            p.next_shot = clock + interval;
            if p.shots_left == 0 {
                p.ends = Some(clock + interval);
            }
            (p.attack.clone(), p.hit)
        };
        vats.weapon_ready = clock + interval;
        let fired = fire(
            order,
            scripts,
            s,
            vats,
            state,
            player,
            (talkers, cell_scripts, collision, rigs),
            attack,
            sounds,
            (&a, hit),
            now_virtual,
        );
        if let Some(p) = vats.playing.as_mut() {
            if !fired {
                p.shots_left = 0;
                p.ends = Some(clock);
            }
            if vats::is_melee_kind(a.kind) {
                p.struck = true;
            }
        }
        vats.fired_now = fired && !vats::is_melee_kind(a.kind);
        // The last round fired: the reload starts at once.
        if fired && needs_reload(attack, state) {
            let time = gun.map_or(0.0, |w| w.reload_time);
            vats.reloading = Some(clock + time);
            vats.weapon_ready = clock + time;
            attack.reload_started = Some(now_virtual);
            if let Some(p) = vats.playing.as_mut() {
                p.next_shot = p.next_shot.max(clock + time);
            }
        }
    }
    // The camera's playback.
    let input = PlaybackInput {
        real_dt,
        attack: vats.playing.as_ref().map(|p| AttackNow {
            id: p.id,
            kind: p.attack.kind,
            melee: vats::is_melee_kind(p.attack.kind),
            has_target: true,
            shots_left: p.shots_left > 0,
            ap: p.attack.ap,
        }),
        projectile_a: vats.fired_now,
        projectile_b: vats.fired_now,
        projectile_a_near: false,
        projectile_b_near: false,
        melee_struck: vats.playing.as_ref().is_some_and(|p| p.struck),
        melee_over: vats
            .playing
            .as_ref()
            .is_some_and(|p| p.ends.is_some_and(|e| clock >= e)),
    };
    let target = vats.playing.as_ref().map(|p| p.attack.target);
    let events = {
        let Vats {
            playback,
            paths,
            models,
            ..
        } = &mut *vats;
        let paths = paths.get_or_insert_with(|| CameraPaths::load(order));
        let facts_state: &GameState = state;
        let mut pick = || {
            let target = target?;
            let facts = Facts {
                order,
                state: facts_state,
                speaker: None,
            };
            let id =
                paths.pick(&mut |p| facts.conditions_pass(&p.conditions, PLAYER_REF, target))?;
            let path = &paths.paths[&id];
            let shots = path
                .shots
                .iter()
                .filter_map(|&s| CameraShot::load(order, s))
                .collect();
            Some((id, path.zoom_mode(), shots))
        };
        let mut load = |shot: &CameraShot| camera_model(game, models, &shot.model).is_some();
        playback.frame(cam, &input, &mut pick, &mut load)
    };
    for e in events {
        match e {
            PlaybackEvent::BeginAttack(_) => {}
            PlaybackEvent::ShotStarted { index, clock, .. } => {
                start_shot(
                    vats,
                    player,
                    fly,
                    rigs,
                    index,
                    clock,
                    eye,
                    now_virtual,
                    game,
                );
            }
            PlaybackEvent::ShotEnded(_) => vats.shot_view = None,
            PlaybackEvent::FirstPerson => vats.third_person = false,
            PlaybackEvent::ThirdPerson => vats.third_person = true,
            PlaybackEvent::ImageSpace { modifier, .. } => {
                if let Some(old) = vats.modifier.take() {
                    state.modifiers.retain(|m| *m != old);
                }
                if let Some(m) = modifier {
                    state.modifiers.push(m);
                    vats.modifier = Some(m);
                }
            }
            PlaybackEvent::AttackDone { .. } => {
                // Its action points are taken now (`009c7240`).
                if let Some(p) = vats.playing.take() {
                    vats::finish_attack(state, &p.attack);
                    if !vats.plan.attacks.is_empty() {
                        vats.plan.attacks.remove(0);
                        vats.specials_queued.remove(0);
                    }
                    vats.done += 1;
                    state.vats.mode = mode::PLAYBACK;
                    println!(
                        "V.A.T.S.: done: {}{}.",
                        vats::kind_label(p.attack.kind),
                        p.special.map_or(String::new(), |s| format!(" ({s})"))
                    );
                }
            }
            PlaybackEvent::QueueCleared => {
                vats.plan.attacks.clear();
                vats.specials_queued.clear();
                vats.playing = None;
            }
            PlaybackEvent::Over => {
                // `009c6c30` with 0: the shot ends, the modifiers go, the
                // view is the player's; the kill reward (`009c8950`).
                if let Some(old) = vats.modifier.take() {
                    state.modifiers.retain(|m| *m != old);
                }
                vats::end(order, state, vats.kills);
                vats.phase = Phase::Off;
                vats.mode = mode::OFF;
                vats.shot_view = None;
                vats.third_person = false;
                vats.hud_mask = None;
                println!(
                    "V.A.T.S. over: {:.0} action points left.",
                    vats::action_points(order, state)
                );
                return;
            }
        }
    }
    // The time multipliers for the next frame (`009c8cc0`, `009c8d60`).
    let kind = vats.playing.as_ref().map(|p| p.attack.kind);
    let ts = vats.playback.time_scale(cam, kind);
    if ts.player != vats.player_mult {
        // The first-person animations keep their place as the pace
        // changes.
        let rebase = |at: &mut Option<f32>| {
            if let Some(start) = at {
                let done = (now_virtual - *start) * vats.player_mult;
                *start = now_virtual - done / ts.player.max(1e-4);
            }
        };
        rebase(&mut attack.fired_at);
        rebase(&mut attack.reload_started);
        vats.player_mult = ts.player;
    }
    attack.sped_up = Some(vats.player_mult);
    vats.target_time = target.map(|t| (t, ts.target));
    // The shot's camera.
    vats.shot_camera = None;
    if vats.third_person && vats.playback.active() && !vats.playback.cut_away {
        if let Some(c) = shot_camera(
            order,
            cam,
            vats,
            player,
            fly,
            (rigs, collision, game),
            camera,
            (now_virtual, world_dt),
        ) {
            vats.shot_camera = Some(c);
        }
    }
    vats.camera_at = Some(match vats.shot_camera {
        Some((t, _, _)) => game_point(t),
        None => eye,
    });
}

/// A camera model, read once (`0058c5d0` needs its first child to be a
/// camera).
fn camera_model(
    game: &cellview::Game,
    models: &mut HashMap<String, Option<Arc<CameraModel>>>,
    path: &str,
) -> Option<Arc<CameraModel>> {
    if path.is_empty() {
        return None;
    }
    models
        .entry(path.to_ascii_lowercase())
        .or_insert_with(|| {
            let full = format!("meshes\\{path}");
            let bytes = game.assets.read(&full).ok().flatten()?;
            let nif = nif::Nif::parse(bytes).ok()?;
            nif.camera_model().ok().flatten().map(Arc::new)
        })
        .clone()
}

/// The next attack starts (`009c8e00`, `009c9280`): what `GetVATSValue`
/// sees, a hit out of sight now a miss, the smart camera checks, a melee
/// attack's warp.
#[allow(clippy::too_many_arguments)]
fn start_attack(
    order: &esm::LoadOrder,
    s: &Settings,
    vats: &mut Vats,
    state: &mut GameState,
    (eye, heading, collision, rigs): ([f32; 3], f32, &CellCollision, &Query<(&Walker, &ActorRig)>),
    player: &mut Player,
    now_virtual: f32,
) {
    let next = vats.plan.attacks[0].clone();
    let special = vats.specials_queued.first().cloned().flatten();
    let weapon = vats.weapon.clone();
    let Some((walker, rig)) = rig_of(rigs, next.target) else {
        // Gone: dropped like the dead's.
        vats.plan.attacks.remove(0);
        vats.specials_queued.remove(0);
        vats.done += 1;
        return;
    };
    let pose = rig.pose_now(now_virtual);
    let node = aim_point(&vats.targets, walker, rig, &pose, &next);
    let d = distance(eye, node);
    let blocked = collision
        .0
        .raycast(eye, unit_toward(eye, node), d)
        .is_some_and(|(t, _)| t < d - 10.0);
    let hit = next.hit && !blocked;
    let melee = vats::is_melee_kind(next.kind);
    let radius = vats
        .targets
        .iter()
        .find(|t| t.reference == next.target)
        .map_or(physics::CharacterShape::PLAYER.radius, |t| t.radius);
    let gap = distance(player.character.feet, walker.position)
        - radius
        - physics::CharacterShape::PLAYER.radius;
    vats::begin_attack(
        state,
        AttackFacts {
            weapon: weapon.as_ref().map(|w| w.form_id),
            target: next.target,
            part_av: next.part_av,
            kind: next.kind,
            hit,
            distance: gap,
            paralyzing_palm: next.paralyzing_palm,
        },
    );
    if melee {
        // The warp (`009c9280`): reach × 0.32 (0.27 unarmed) between the
        // bodies, on the line from them to the player.
        let warp = vats::warp_distance(s, weapon.as_ref(), 1.0);
        let away = {
            let f = player.character.feet;
            let (dx, dy) = (f[0] - walker.position[0], f[1] - walker.position[1]);
            let l = (dx * dx + dy * dy).sqrt().max(1e-3);
            [dx / l, dy / l]
        };
        let reach = radius + physics::CharacterShape::PLAYER.radius + warp;
        let spot = [
            walker.position[0] + away[0] * reach,
            walker.position[1] + away[1] * reach,
            walker.position[2],
        ];
        player.character = physics::Character::new(spot);
    }
    // The smart camera checks the camera paths' conditions ask about the
    // attacker (`008bd830`, `008bdbd0`): lines from the player's position,
    // raised, out to each side and from there to the target's part; the
    // cell's collision and the people's bodies in the way.
    let smart = vats
        .smart
        .get_or_insert_with(|| SmartCameraSettings::load(order))
        .clone();
    let feet = player.character.feet;
    let mut checks = SmartCamera::default();
    for (i, side) in Side::ALL.iter().enumerate() {
        let (from, to) = vats::area_free_line(&smart, feet, heading, *side);
        let first = line_hits(collision, rigs, &vats.targets, (from, to), PLAYER_REF, None)
            .into_iter()
            .map(|h| h.fraction)
            .reduce(f32::min);
        checks.area_free[i] = vats::area_free(first);
        let samples = vats::visible_samples(&smart, feet, heading, *side);
        let me = Some((feet, physics::CharacterShape::PLAYER.radius));
        checks.target_visible[i] = vats::target_visible(&smart, &samples, Some(node), |a, b| {
            line_hits(collision, rigs, &vats.targets, (a, b), next.target, me)
        });
    }
    state.vats.smart_camera = Some(checks);
    let show = |v: f32| {
        if v == f32::MAX {
            "none".to_string()
        } else {
            format!("{v:.0}")
        }
    };
    println!(
        "V.A.T.S.: smart camera (front, right, left, back): area free {}, {}, {}, {}; target visible {}, {}, {}, {}.",
        show(checks.area_free[0]),
        show(checks.area_free[1]),
        show(checks.area_free[2]),
        show(checks.area_free[3]),
        checks.target_visible[0],
        checks.target_visible[1],
        checks.target_visible[2],
        checks.target_visible[3],
    );
    let first = vats.player_clock.max(vats.weapon_ready);
    println!(
        "V.A.T.S.: {}{}.",
        vats::kind_label(next.kind),
        if hit != next.hit {
            " (out of sight now: a miss)"
        } else {
            ""
        }
    );
    vats.last_attack = Some(next.clone());
    vats.playing = Some(Playing {
        id: vats.done,
        shots_left: u32::from(next.shots) + u32::from(next.extra_shots),
        attack: next,
        hit,
        special,
        next_shot: first,
        ends: None,
        struck: false,
    });
}

/// What a line from one point to another meets, as the smart camera
/// checks read it (`008bd830`, `008bdbd0`, which pick on the `CAMERAPICK`
/// layer): the cell's collision (the nearest surface), and the living
/// people as upright cylinders of their radius and height, `skip` left
/// out (the actor itself for the area checks, the target for the
/// visibility ones); the player's own body counts when `me` gives their
/// feet and radius (the visibility lines, which start beside them).
fn line_hits(
    collision: &CellCollision,
    rigs: &Query<(&Walker, &ActorRig)>,
    targets: &[Target],
    (from, to): ([f32; 3], [f32; 3]),
    skip: FormId,
    me: Option<([f32; 3], f32)>,
) -> Vec<LineHit> {
    let d = [to[0] - from[0], to[1] - from[1], to[2] - from[2]];
    let len = (d[0] * d[0] + d[1] * d[1] + d[2] * d[2]).sqrt();
    if len < 1e-6 {
        return Vec::new();
    }
    let mut out = Vec::new();
    if let Some((t, _)) = collision.0.raycast(from, d.map(|x| x / len), len) {
        out.push(LineHit {
            fraction: t / len,
            actor: false,
        });
    }
    let height = physics::CharacterShape::PLAYER.height;
    let mut body = |feet: [f32; 3], radius: f32| {
        if let Some(f) = cylinder_hit(from, to, feet, radius, height) {
            out.push(LineHit {
                fraction: f,
                actor: true,
            });
        }
    };
    for (walker, rig) in rigs.iter() {
        if walker.reference == skip || rig.ragdoll.is_some() {
            continue;
        }
        let radius = targets
            .iter()
            .find(|t| t.reference == walker.reference)
            .map_or(physics::CharacterShape::PLAYER.radius, |t| t.radius);
        body(walker.position, radius);
    }
    if let Some((feet, radius)) = me {
        body(feet, radius);
    }
    out
}

/// Where the line from `from` to `to` enters an upright cylinder standing
/// on `feet` (its side; a line coming in over the top isn't caught), as a
/// share of the line; 0 when it starts inside.
fn cylinder_hit(
    from: [f32; 3],
    to: [f32; 3],
    feet: [f32; 3],
    radius: f32,
    height: f32,
) -> Option<f32> {
    let (dx, dy, dz) = (to[0] - from[0], to[1] - from[1], to[2] - from[2]);
    let (mx, my) = (from[0] - feet[0], from[1] - feet[1]);
    let a = dx * dx + dy * dy;
    let b = 2.0 * (mx * dx + my * dy);
    let c = mx * mx + my * my - radius * radius;
    let inside = c <= 0.0;
    let t = if a < 1e-9 {
        if inside {
            0.0
        } else {
            return None;
        }
    } else {
        let disc = b * b - 4.0 * a * c;
        if disc < 0.0 {
            return None;
        }
        let t = (-b - disc.sqrt()) / (2.0 * a);
        if t < 0.0 {
            if inside {
                0.0
            } else {
                return None;
            }
        } else {
            t
        }
    };
    if t > 1.0 {
        return None;
    }
    let z = from[2] + dz * t;
    (z >= feet[2] && z <= feet[2] + height).then_some(t)
}

/// A shot starts (`009c7240` → `0058c5d0`): its nodes, the located-at
/// actor's heading and the location's place now, where the camera was.
#[allow(clippy::too_many_arguments)]
fn start_shot(
    vats: &mut Vats,
    player: &Player,
    fly: &FlyCamera,
    rigs: &Query<(&Walker, &ActorRig)>,
    index: usize,
    clock: f32,
    eye: [f32; 3],
    now_virtual: f32,
    game: &cellview::Game,
) {
    let Some(shot) = vats.playback.shots.get(index).cloned() else {
        return;
    };
    let Some(p) = vats.playing.as_ref() else {
        return;
    };
    let target_ref = p.attack.target;
    let rig = rig_of(rigs, target_ref);
    let part_node = rig.and_then(|(walker, rig)| {
        let pose = rig.pose_now(now_virtual);
        part_node(&vats.targets, walker, rig, &pose, &p.attack)
    });
    let there = NodesThere {
        projectile_a: vats.fired_now,
        projectile_b: vats.fired_now,
        target_root: rig.is_some(),
        target_part: part_node.is_some(),
    };
    let (location, look) = cams::shot_nodes(&shot, there);
    let at_target = shot.location == cams::place::TARGET;
    let start_heading = match rig {
        Some((walker, _)) if at_target => walker.heading,
        _ => view_angles(fly).0,
    };
    let start_position =
        location.and_then(|l| node_position(l, player, fly, rig.map(|(w, _)| w), part_node));
    let model = camera_model(game, &mut vats.models, &shot.model);
    println!(
        "V.A.T.S.: camera shot {} ({}).",
        shot.editor_id,
        if model.is_some() {
            shot.model.as_str()
        } else {
            "no camera model"
        }
    );
    vats.shot_view = Some(ShotView {
        shot,
        model,
        start_clock: clock,
        location,
        look,
        start_heading,
        start_position,
        from: vats.camera_at.unwrap_or(eye),
    });
    // The first placing takes the cast's distance at once (`0058c5d0`
    // sets `[011e07c3]`).
    vats.chase.shot_begins();
}

/// Where a shot's node is now. The attacker's bone (`00acbb70`) isn't
/// traced and projectiles don't linger (hitscan): those give nothing.
fn node_position(
    node: ShotNode,
    player: &Player,
    _fly: &FlyCamera,
    target: Option<&Walker>,
    part: Option<[f32; 3]>,
) -> Option<[f32; 3]> {
    match node {
        ShotNode::AttackerRoot => Some(player.character.feet),
        ShotNode::TargetRoot => target.map(|w| w.position),
        ShotNode::TargetPart => part.or_else(|| target.map(|w| w.position)),
        ShotNode::AttackerBone | ShotNode::ProjectileA | ShotNode::ProjectileB => None,
    }
}

/// The shot's camera now (`0058cf60`, `0094ae40`): the model's keyed offset
/// turned by the heading and set at the location, kept out of walls
/// (`0094a0c0`: a sphere `fCameraCasterSize` wide cast from the location
/// toward it, `ChaseDistance`), or slid there by the dolly, looking at the
/// look point with the model's angles laid on; the frustum keyed and
/// widened for the screen. Close enough to cut away: first person for the
/// rest of the shot.
#[allow(clippy::too_many_arguments)]
fn shot_camera(
    order: &esm::LoadOrder,
    cam: &CameraSettings,
    vats: &mut Vats,
    player: &Player,
    fly: &FlyCamera,
    (rigs, collision, game): (
        &Query<(&Walker, &ActorRig)>,
        &CellCollision,
        &cellview::Game,
    ),
    camera: &Camera,
    (now_virtual, world_dt): (f32, f32),
) -> Option<(Vec3, Quat, f32)> {
    let view = vats.shot_view.as_ref()?;
    let p = vats
        .playing
        .as_ref()
        .map(|p| &p.attack)
        .or(vats.last_attack.as_ref());
    let shot = view.shot.clone();
    let model = view.model.clone()?;
    let target_ref = p.map(|a| a.target)?;
    let rig = rig_of(rigs, target_ref);
    let part = match (rig, p) {
        (Some((walker, rig)), Some(a)) => {
            let pose = rig.pose_now(now_virtual);
            part_node(&vats.targets, walker, rig, &pose, a)
        }
        _ => None,
    };
    let target_walker = rig.map(|(w, _)| w);
    let clock = vats.playback.clock;
    let shift = shot
        .has(cams::shot_flags::START_AT_TIME_ZERO)
        .then_some(-view.start_clock);
    // Where.
    let location = if shot.has(cams::shot_flags::POSITION_FOLLOWS_LOCATION) {
        view.location
            .and_then(|l| node_position(l, player, fly, target_walker, part))
    } else {
        view.start_position
    }?;
    let attacker_heading = view_angles(fly).0;
    let located_heading = match target_walker {
        Some(w) if shot.location == cams::place::TARGET => w.heading,
        _ => attacker_heading,
    };
    let heading = cams::path_heading(&shot, attacker_heading, located_heading, view.start_heading);
    let offset = model.translation_at(clock, shift);
    let wanted = cams::shot_position(offset, heading, location);
    // Kept out of walls: the cast from the location node toward the place
    // wanted, and the chase camera's distance rules.
    let chase = vats
        .chase_settings
        .get_or_insert_with(|| {
            let mut c = ChaseSettings::load(order);
            c.read_ini(|section, key| game.settings.float(section, key));
            c
        })
        .clone();
    let touched = {
        let d = [0, 1, 2].map(|i| wanted[i] - location[i]);
        let len = (d[0] * d[0] + d[1] * d[1] + d[2] * d[2]).sqrt();
        if len > 1e-6 {
            collision
                .0
                .spherecast(location, d.map(|x| x / len), len, chase.caster_size)
                .map(|h| h.point)
        } else {
            None
        }
    };
    let to = vats
        .chase
        .place(&chase, location, wanted, touched, world_dt, true);
    let from = view.from;
    let eye = cams::dolly_position(cam, &mut vats.playback.dolly, shot.action, from, to, clock);
    // What it looks at.
    let target_node = node_position(view.look, player, fly, target_walker, part)?;
    let attacker_middle =
        cams::bound_sphere(order, PLAYER_REF, player.character.feet, attacker_heading).0;
    let target_middle = target_walker.map_or(target_node, |w| {
        cams::bound_sphere(order, target_ref, w.position, w.heading).0
    });
    let point = cams::look_point(&shot, target_node, attacker_middle, target_middle);
    if distance(eye, point) < cam.cut_away_distance {
        vats.playback.cut_away = true;
        vats.third_person = false;
        return None;
    }
    let angles = model.angles_at(clock, shift);
    let (forward, up) = cams::shot_orientation(eye, point, angles);
    let size = camera
        .logical_viewport_size()
        .unwrap_or(Vec2::new(16.0, 9.0));
    let frustum = cams::shot_frustum(model.frustum_at(clock, shift), size.x / size.y.max(1.0));
    let top = frustum.top / frustum.near.max(1e-6);
    let fov = 2.0 * top.atan();
    let translation = Vec3::from(space::point(eye));
    let rotation = Transform::IDENTITY
        .looking_to(
            Vec3::from(space::direction(forward)),
            Vec3::from(space::direction(up)),
        )
        .rotation;
    Some((translation, rotation, fov))
}

/// Puts the shot's camera on the view, after walking and the first-person
/// view have placed it (`0058cf60`).
pub fn apply_shot_camera(
    vats: Res<Vats>,
    mut cameras: Query<(&mut Transform, &mut Projection), With<FlyCamera>>,
) {
    let Some((translation, rotation, fov)) = vats.shot_camera else {
        // Out of the shot: the player's own field of view.
        if vats.phase == Phase::Playback {
            if let (Ok((_, mut projection)), Some(restore)) =
                (cameras.single_mut(), vats.restore_fov)
            {
                if let Projection::Perspective(p) = &mut *projection {
                    if p.fov != restore {
                        p.fov = restore;
                    }
                }
            }
        }
        return;
    };
    let Ok((mut transform, mut projection)) = cameras.single_mut() else {
        return;
    };
    transform.translation = translation;
    transform.rotation = rotation;
    if let Projection::Perspective(p) = &mut *projection {
        p.fov = fov;
    }
}

/// The target's own pace in a shot (`009c8d60`): its animations run at the
/// world's time × its multiplier.
pub fn scale_target_time(
    time: Res<Time>,
    vats: Res<Vats>,
    mut rigs: Query<(&Walker, &mut ActorRig)>,
) {
    let Some((target, mult)) = vats.target_time else {
        return;
    };
    let extra = time.delta_secs() * (mult - 1.0);
    if extra == 0.0 {
        return;
    }
    for (walker, mut rig) in &mut rigs {
        if walker.reference != target || rig.ragdoll.is_some() {
            continue;
        }
        // The target's animations run at its multiplier: the rest of the
        // frame's time on top of the ordinary update.
        rig.player.update(extra);
        if let Some(at) = rig.attack_at.as_mut() {
            *at -= extra;
        }
        if let Some((_, t)) = rig.overlay.as_mut() {
            *t += extra;
        }
    }
}

/// Seconds between shots (or swings) in the player's time: an automatic's
/// fire rate, else the weapon's attacks a second (fists: half a second).
fn attack_interval(weapon: Option<&Weapon>) -> f32 {
    match weapon {
        Some(w) if w.flags1 & vats::flags::AUTOMATIC != 0 && w.fire_rate > 0.0 => {
            1.0 / (w.fire_rate * w.attack_mult.max(0.01)).max(0.01)
        }
        Some(w) => w.shot_interval(),
        None => 0.5,
    }
}

/// The node of an attack's part (`BPNT`), if it has one.
fn part_node(
    targets: &[Target],
    walker: &Walker,
    rig: &ActorRig,
    pose: &[nif::Transform],
    a: &QueuedAttack,
) -> Option<[f32; 3]> {
    targets
        .iter()
        .find(|t| t.reference == a.target)
        .and_then(|t| t.parts.iter().find(|p| p.slot == a.slot))
        .and_then(|p| node_point(walker, rig, pose, &p.node))
}

/// The point an attack aims at: its part's node, else the middle of the
/// body.
fn aim_point(
    targets: &[Target],
    walker: &Walker,
    rig: &ActorRig,
    pose: &[nif::Transform],
    a: &QueuedAttack,
) -> [f32; 3] {
    part_node(targets, walker, rig, pose, a).unwrap_or([
        walker.position[0],
        walker.position[1],
        walker.position[2] + 64.0,
    ])
}

/// One shot (or swing) of the attack playing; false when nothing could be
/// fired (out of rounds).
#[allow(clippy::too_many_arguments)]
fn fire(
    order: &esm::LoadOrder,
    scripts: &world::scripting::ScriptCache,
    s: &Settings,
    vats: &mut Vats,
    state: &mut GameState,
    player: &Player,
    (talkers, cell_scripts, collision, rigs): (
        &Talkers,
        &CellScripts,
        &CellCollision,
        &Query<(&Walker, &ActorRig)>,
    ),
    attack: &mut PlayerAttack,
    sounds: &mut SoundRequests,
    (a, hit): (&QueuedAttack, bool),
    now_virtual: f32,
) -> bool {
    let weapon = vats.weapon.clone();
    let Some((walker, rig)) = rig_of(rigs, a.target) else {
        return false;
    };
    let f = player.character.feet;
    let eye = [f[0], f[1], f[2] + cellview::EYE_HEIGHT];
    let pose = rig.pose_now(now_virtual);
    let node = aim_point(&vats.targets, walker, rig, &pose, a);
    let melee = vats::is_melee_kind(a.kind);
    let ammo = weapon
        .as_ref()
        .and_then(|w| w.ammo_in_use(order, state, PLAYER_REF));
    // Rounds: each shot uses the weapon's ammo use from the clip.
    if let Some(w) = weapon.as_ref().filter(|w| !melee && !w.ammo.is_empty()) {
        let carried = carried_rounds(order, state, w);
        let clip = attack.clip(w, carried);
        let use_ = u32::from(w.ammo_use.max(1));
        if clip < use_ {
            println!("V.A.T.S.: out of rounds.");
            return false;
        }
        attack.set_clip(w, clip - use_);
        if let Some(n) = ammo.and_then(|am| state.items.get_mut(&(PLAYER_REF, am))) {
            *n -= use_ as i32;
        }
    }
    attack.fired_at = Some(now_virtual);
    if let Some(snd) = weapon.as_ref().and_then(|w| w.sound) {
        sounds.0.push(snd);
    }
    let was_dead = |state: &GameState, r: FormId| state.dead.contains(&r);
    let mut struck: Vec<(FormId, bool)> = Vec::new();
    // The move's group (`world::vats::attack_group`: Uppercut `Attack6`,
    // Cross `Attack7`) and what it does to the one hit
    // (`world::melee::special_of`).
    let blow = world::melee::Blow {
        power: false,
        special: vats::attack_group(a.kind, state.player_sneaking)
            .map_or(world::melee::Special::None, |g| {
                world::melee::special_of(weapon.as_ref(), g)
            }),
    };
    if melee {
        // A queued hit lands on its part; a miss swings as an ordinary blow,
        // reaching × `fVATSMeleeReachMult`.
        let toward = unit_toward(eye, node);
        if hit {
            struck.push((a.target, was_dead(state, a.target)));
            let part = u8::try_from(a.slot).ok();
            let result = Runner::new(order, scripts, state).blow_at(
                PLAYER_REF,
                a.target,
                weapon.as_ref(),
                part,
                blow,
            );
            if let Some(h) = result {
                let d = distance(eye, node);
                println!("  {}.", tell_hit(order, state, attack, a.target, d, &h));
            }
        } else {
            let reach = Weapon::melee_reach(weapon.as_ref()) * s.melee_reach_mult
                + physics::CharacterShape::PLAYER.radius;
            let met = first_met(
                order,
                state,
                attack,
                talkers,
                cell_scripts,
                collision,
                rigs,
                (eye, toward),
                reach,
                true,
                now_virtual,
            );
            match met {
                Met::Thing {
                    distance: d,
                    reference,
                    part,
                } => {
                    struck.push((reference, was_dead(state, reference)));
                    if let Some(h) = Runner::new(order, scripts, state).blow_at(
                        PLAYER_REF,
                        reference,
                        weapon.as_ref(),
                        part,
                        blow,
                    ) {
                        println!(
                            "  (missed) {}.",
                            tell_hit(order, state, attack, reference, d, &h)
                        );
                    }
                }
                Met::Nothing(_) => println!("  The swing missed."),
            }
        }
    } else {
        let w = weapon.clone();
        let (count, cone) = match w.as_ref() {
            Some(w) => w.shot(order, ammo),
            None => (1, 0.0),
        };
        let reach = w.as_ref().and_then(|w| w.range(order)).unwrap_or(10_000.0);
        let pellet = w.map(|mut w| {
            w.damage /= count as f32;
            w
        });
        // Aimed at the node for a hit; off it for a miss.
        let (mut heading, mut pitch) = angles(unit_toward(eye, node));
        if !hit {
            let bound = vats::bound_of(order, a.target);
            let (u1, u2) = (unit(state), unit(state));
            let (dh, dp) = vats::miss_offset(s, bound * 0.5, distance(eye, node), u1, u2);
            heading += dh;
            pitch += dp;
        }
        for _ in 0..count {
            // A shotgun's pellets keep their own spread
            // (× `fVatsShotgunSpreadRatio`).
            let (h, p) = if count > 1 {
                let r = cone * s.shotgun_spread_ratio * unit(state);
                let theta = std::f32::consts::TAU * unit(state);
                (heading + r * theta.cos(), pitch + r * theta.sin())
            } else {
                (heading, pitch)
            };
            let dir = direction(h, p);
            let met = first_met(
                order,
                state,
                attack,
                talkers,
                cell_scripts,
                collision,
                rigs,
                (eye, dir),
                reach,
                false,
                now_virtual,
            );
            // A hit meeting its target (or slipping between its capsules)
            // lands on the part chosen (`009b6620`).
            let (who, part, d) = match met {
                Met::Thing {
                    distance,
                    reference,
                    ..
                } if hit && reference == a.target => {
                    (Some(reference), u8::try_from(a.slot).ok(), distance)
                }
                Met::Nothing(false) if hit => (
                    Some(a.target),
                    u8::try_from(a.slot).ok(),
                    distance(eye, node),
                ),
                Met::Thing {
                    distance,
                    reference,
                    part,
                } => (Some(reference), part, distance),
                Met::Nothing(wall) => {
                    println!(
                        "  The shot hit nothing{}.",
                        if wall { " but a wall" } else { "" }
                    );
                    (None, None, 0.0)
                }
            };
            let Some(who) = who else {
                continue;
            };
            struck.push((who, was_dead(state, who)));
            match Runner::new(order, scripts, state).hit_at(PLAYER_REF, who, pellet.as_ref(), part)
            {
                Some(h) => println!("  {}.", tell_hit(order, state, attack, who, d, &h)),
                None => println!("  Hit {who} at {d:.0} units."),
            }
        }
    }
    // Kills while it plays (Grim Reaper's Sprint, `009c8950`).
    for (who, before) in struck {
        if !before && state.dead.contains(&who) {
            vats.kills += 1;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    fn weapon(animation: u32, flags1: u8, fire_rate: f32, attack_mult: f32) -> Weapon {
        Weapon {
            form_id: FormId(1),
            name: "Test".into(),
            damage: 10.0,
            clip: 30,
            health: 100,
            animation,
            ammo: Vec::new(),
            ammo_use: 1,
            min_spread: 1.0,
            spread: 0.0,
            projectile: None,
            projectiles: 1,
            min_range: 0.0,
            max_range: 1000.0,
            shots_per_second: 3.125,
            reload_time: 2.0,
            skill: 41,
            crit_damage: 0.0,
            crit_mult: 1.0,
            sound: None,
            attack_animation: 255,
            reload_animation: 0,
            kill_impulse: 0.0,
            impulse_distance: 0.0,
            reach: 0.0,
            limb_damage_mult: 1.0,
            flags1,
            flags2: 0,
            fire_rate,
            attack_mult,
            aim_arc: 0.0,
            semi_auto_delay: (0.0, 0.0),
            speed: 1.0,
            cone_mult: 1.0,
            crit_effect: None,
            crit_on_death: false,
            resist: None,
        }
    }

    #[test]
    fn views_turn_to_the_direction_asked() {
        for dir in [[0.0, 1.0, 0.0], [1.0, 0.0, 0.0], [0.6, -0.48, 0.64]] {
            let (h, p) = angles(dir);
            let back = direction(h, p);
            assert!(
                (0..3).all(|i| (back[i] - dir[i]).abs() < 1e-5),
                "{dir:?} {back:?}"
            );
        }
        // North is heading 0, east a quarter turn clockwise.
        assert!(angles([0.0, 1.0, 0.0]).0.abs() < 1e-6);
        assert!((angles([1.0, 0.0, 0.0]).0 - std::f32::consts::FRAC_PI_2).abs() < 1e-6);
        // The view's yaw and back.
        let mut fly = FlyCamera {
            yaw: 0.0,
            pitch: 0.0,
            speed: 1.0,
            start: (Vec3::ZERO, 0.0),
        };
        turn_to(&mut fly, 1.0, 0.2);
        assert_eq!(view_angles(&fly), (1.0, 0.2));
    }

    #[test]
    fn bursts_follow_the_fire_rate_and_shots_the_attacks_a_second() {
        // An automatic: 12 a second × its attack multiplier.
        let auto = weapon(6, vats::flags::AUTOMATIC, 12.0, 1.0);
        assert!((attack_interval(Some(&auto)) - 1.0 / 12.0).abs() < 1e-6);
        // Semi-automatic: its attacks a second (3.125); fists half a second.
        let pistol = weapon(3, 0, 1.0, 1.0);
        assert!((attack_interval(Some(&pistol)) - 0.32).abs() < 1e-6);
        assert_eq!(attack_interval(None), 0.5);
    }

    #[test]
    fn a_new_target_opens_on_the_torso() {
        let entry = |slot: i8, av: i32| PartEntry {
            slot,
            name: String::new(),
            node: String::new(),
            actor_value: av,
            chance: 0.0,
            scanned: false,
        };
        assert_eq!(
            torso_or_first(&[entry(1, 25), entry(0, 26), entry(3, 27)]),
            1
        );
        assert_eq!(torso_or_first(&[entry(1, 25), entry(3, 27)]), 0);
    }
}

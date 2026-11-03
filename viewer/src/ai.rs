//! People following their AI packages (`world::ai`) by the game's rules
//! (`world::movement`, `world::social`, read from its code). Packages are
//! looked at again every 20 s, on each game-hour change and when forced
//! (`world::movement::PackageClock`); one that sends someone somewhere
//! gets a path over the navmesh (the interior's, or outdoors the 3 × 3
//! squares around the player, joined): first an in-place turn toward it,
//! then the walk at their walk animation's own speed, turning by the
//! walking rule, the body moving toward the steering point while the
//! facing catches up, through the cell's collision with their character
//! controller ([`move_body`]); others in the way are waited for or gone
//! round (`world::movement::Avoidance`), and someone held back 1.5 s is
//! stuck and asks for the way again ([`unstick`]). Dialogue packages walk up to their
//! target and talk; people greet the player, chatter and talk with each
//! other (`chatter` says the lines). Furniture, idles and sandbox packages
//! are `sitting`'s; noticing others, fighting and running away are
//! `fighting`'s. People out of sight walk their paths in game-time steps
//! ([`move_offstage`]).

use std::collections::{HashMap, HashSet, VecDeque};

use bevy::prelude::*;
use cellview::{space, ActorData};
use esm::FormId;
use world::ai::{current_package, destination, DialogueStep, NavMesh};
use world::dialogue::{Speaker, PLAYER_REF};
use world::furniture::SitState;
use world::head_track::{Candidate, HeadTrack, Slot};
use world::movement::{
    self as mv, AvoidStep, Avoidance, MoveSettings, Obstacle, PackageClock, ProcessLevel, Turn,
    TurnSide,
};
use world::social::{Social, SocialSettings};

use crate::actors::ActorRig;
use crate::chatter::Lines;
use crate::dialogue::{Conversation, DialogueState, Talkers};
use crate::sitting::{Ctx, Life, Seats};
use crate::GameFiles;

/// The settings people's moving and talking read, once.
#[derive(Resource)]
pub struct Moves {
    pub settings: MoveSettings,
    pub social: SocialSettings,
    /// `fAIMoveDistanceToRecalcFollowPath` (300): a target that moved this
    /// far since the path was made gets a new one.
    pub recalc_follow: f32,
    /// Head tracking's settings (`world::head_track`): game settings and
    /// `[HeadTracking]` in the INI.
    pub head_track: world::head_track::Settings,
}

impl Moves {
    pub fn new(game: &cellview::Game) -> Moves {
        let order = &game.order;
        let g = |name: &str, d: f32| world::scripting::game_setting(order, name).unwrap_or(d);
        Moves {
            settings: MoveSettings::read(order, &|section, key| game.settings.float(section, key)),
            social: SocialSettings::read(order),
            recalc_follow: g("fAIMoveDistanceToRecalcFollowPath", 300.0),
            head_track: world::head_track::Settings::read(
                |name| world::scripting::game_setting(order, name),
                |section, key| game.settings.float(section, key),
            ),
        }
    }
}

/// A conversation between two people (`StartConversation`, the type-0x1c
/// package `008b2170`): the one who started walks up to within `reach` of
/// the other (90; 200 when seated), who waits facing them; then the lines,
/// worked out as it began (`world::social::conversation`), each said once
/// the last is over and at least 2 s after it began (the playback's pause
/// timer, `009ee0a0`; which of its 2/10/50 s pauses applies when isn't
/// traced, so the line's own length and 2 s are taken).
#[derive(Debug, Clone)]
pub struct Chat {
    pub with: FormId,
    pub starter: bool,
    pub reach: f32,
    pub lines: Vec<world::social::Line>,
    pub next: usize,
    /// When the last line began; whether its speaker is still saying it.
    pub last_line: f32,
    pub talking: bool,
}

/// Conversations between people, by each of the two.
#[derive(Resource, Default)]
pub struct Chats(pub HashMap<FormId, Chat>);

/// `StartConversation` asked of someone by a script (`005c8740` →
/// `008b2170`): who, with whom, about what. The one asked comes up first
/// (the conversation package, type 0x1c): to within 90 of the other (200
/// when seated), then the dialogue menu with the player, or a conversation
/// with anyone else (`start_chat`).
#[derive(Resource, Default)]
pub struct Starts(pub Vec<(FormId, FormId, Option<FormId>)>);

/// A person's place and what they're doing.
#[derive(Component)]
pub struct Walker {
    pub reference: FormId,
    pub position: [f32; 3],
    /// Radians clockwise from north.
    pub heading: f32,
    pub scale: f32,
    pub(crate) package: Option<FormId>,
    pub(crate) package_kind: Option<u8>,
    pub(crate) target: Option<[f32; 3]>,
    /// The reference the package sends them to, if any, and where it stood
    /// when the path was made.
    target_ref: Option<FormId>,
    path_target: Option<[f32; 3]>,
    pub(crate) path: Vec<[f32; 3]>,
    /// The point walked toward (the steering point's segment end).
    pub(crate) next: usize,
    /// Path progress: point index and fraction (`009e3d50`).
    pub(crate) progress: f32,
    /// A point to keep facing while walking (strafing in a fight).
    pub(crate) face_point: Option<[f32; 3]>,
    /// The turn in place under way, their turning speed (degrees a second)
    /// and in-place rates (radians a second, out of and in combat), and the
    /// side turned this frame (for the turn animation).
    pub(crate) turn: Turn,
    pub(crate) turn_speed: f32,
    pub(crate) rates: [f32; 2],
    pub(crate) turning: Option<TurnSide>,
    /// When the package is looked at again; forced when set.
    clock: PackageClock,
    pub(crate) evaluate: bool,
    /// Placed (or moved by a script) since the last frame: start from the
    /// state's position.
    fresh: bool,
    /// Loaded since their last package choice: the cell (or attached
    /// square) they're in has just loaded, so the game's pass over the
    /// loaded actors (`00972d30`) seats them in their package's furniture
    /// at once ([`crate::sitting::seat_on_load`]). Taken by the next
    /// [`rethink`].
    pub(crate) settle: bool,
    avoidance: Avoidance,
    /// Units a second, as last moved.
    pub(crate) velocity: [f32; 3],
    /// The player's detection value at their last detection run.
    pub(crate) detected_player: i32,
    /// A greeting tried with no line begun (the player's cooldown not over,
    /// or no `HELLO` line): `008bc3d0` put the player in their ACTION
    /// head-track slot all the same, and only the GREET procedure's end
    /// (never reached) clears it, so it holds till the head-track update
    /// empties the slots (`008a3100` step 4: the player no longer
    /// detected).
    pub(crate) greet_look: bool,
    pub(crate) social: Option<Social>,
    /// A dialogue package's steps (`world::ai::talk`): its travel over
    /// (walking up to the target may take them off the place; the game
    /// doesn't go back to the travel, only a new start of the package
    /// does), the conversation package made, the talk under way.
    talk_run: world::ai::talk::DialogueRun,
    /// Whom they look at (and so turn the body to, `008a3100`): the head-
    /// track target slots (`world::head_track`).
    pub(crate) head_track: HeadTrack,
    /// `bDisableHeadTracking:HeadTracking`: the head doesn't follow.
    head_tracking_off: bool,
    /// How they fight, once read (`fighting::Kit`), and the fight under way.
    pub(crate) kit: Option<crate::fighting::Kit>,
    pub(crate) fight: Option<crate::fighting::Fight>,
    /// When their next detection run is due, and whom they've noticed
    /// (above −20) since that last dropped (`fighting::detect`).
    pub(crate) detect_at: f32,
    pub(crate) noticed: crate::fighting::Noticed,
    /// Their combat targets and what their detection runs found of each
    /// (`fighting::Targets`).
    pub(crate) targets: crate::fighting::Targets,
    /// Running from someone (`fighting::flee`).
    pub(crate) fleeing: Option<FormId>,
    /// Fallen (dead).
    fallen: bool,
    /// Held still while the dialogue menu stops the world.
    paused: bool,
    /// A woman (for the idle tree's women's idles).
    pub(crate) female: bool,
    /// The load door they're walking to, toward a package's place
    /// elsewhere.
    door: Option<world::ai::DoorWay>,
    /// The doors their path goes through that they haven't passed yet,
    /// with each one's portal (`doors::walker_at_door`).
    pub(crate) doors_ahead: Vec<(FormId, [f32; 3])>,
    /// How near the target counts as there: the path request's radius.
    pub(crate) radius: f32,
    /// The heading to turn to when the walk ends (the end of the travel
    /// procedure: an `XMarkerHeading`'s, `world::ai::arrival_heading`), and
    /// the one being turned to now, standing.
    pub(crate) arrival: Option<f32>,
    pub(crate) facing: Option<f32>,
    /// Where they stood when their package began (the middle of a wander
    /// "near the current location").
    pub(crate) home: [f32; 3],
    /// A script's `StartConversation` to carry out ([`Starts`]), and the
    /// walk up to the player it became: the topic and the reach.
    start: Option<(FormId, Option<FormId>)>,
    talk_to: Option<(Option<FormId>, f32)>,
    /// The package's end action has been asked for since it began (the
    /// process flag +0x5a8, `0091ecf0`).
    ended: bool,
    /// Running, not walking, along the path (a guard far from its post,
    /// `world::ai::guard::runs`; fleeing).
    pub(crate) run: bool,
    /// The path walked ends short of the place: the attached cells' part
    /// of the long way ([`long_walk`]); its end isn't the travel's end.
    partial: bool,
    /// The long way last planned: the attached squares and the goal it
    /// was planned for.
    long: Option<LongPlan>,
    /// Hidden here because they stand outdoors beyond the attached cells
    /// (the game's lower processes; [`move_offstage`] moves them).
    parked: bool,
    /// Their character controller: the game builds one for every actor
    /// whose 3D is set up (`00930c70` → `00c741e0`), the player's kind
    /// (`physics::Character`), and moves them through the cell's collision
    /// with it ([`move_body`]). `None` until there is collision under them.
    pub(crate) body: Option<physics::Character>,
    /// An immobile creature (a turret): `MobileObject::Move` never moves
    /// them without their controller (`world::ground::immobile`).
    pub(crate) immobile: bool,
    /// This frame's move along the path, in world units (x, y): the path
    /// handler's move vector turned toward the steering point (`009e3560`,
    /// handler +0x1c, its world form at +0xa0 from `009e0a00`), which the
    /// mover hands on (`009ddc00`, mover +0x10) for the controller.
    pub(crate) wanted: Option<[f32; 2]>,
    /// The stuck test's state (`world::movement::Stuck`, `009e4cf0`), the
    /// length of the last frame's move (handler +0x9c; 0 while turning in
    /// place), whether their controller touches someone, and where they
    /// got stuck (their path failed there: [`unstick`]).
    pub(crate) stuck: mv::Stuck,
    pub(crate) last_move: f32,
    pub(crate) against_someone: bool,
    pub(crate) stuck_at: Option<[f32; 3]>,
    /// The radius their path requests carry (`world::ai::request_radius`;
    /// the request's default 35 until their kit is read).
    pub(crate) request_radius: f32,
    /// A path asked of the path manager and not back yet ([`ask_path`]):
    /// they stand waiting for it (the mover's state 1, `009db090`).
    pub(crate) pending: Option<Pending>,
    /// The last failed search reported (why, and where from and to,
    /// roughly), so a search failing the same way again isn't reported
    /// again.
    last_failure: Option<(String, [[i32; 3]; 2])>,
}

/// The long way last planned: the attached squares and the goal.
type LongPlan = (Vec<(i32, i32)>, [f32; 3]);

/// People turn 135° a second in place until their kit is read.
const PEOPLE_RATES: [f32; 2] = [90.0 * 1.5 * mv::ONE_DEGREE, 90.0 * 2.5 * mv::ONE_DEGREE];

impl Walker {
    /// Discard the running procedure for ResetAI (008a6ce0 / 00923c60),
    /// keeping physical placement. A new package is chosen this frame.
    fn reset_procedure(&mut self) {
        self.clear_path();
        self.package = None;
        self.package_kind = None;
        self.target = None;
        self.target_ref = None;
        self.path_target = None;
        self.talk_run = world::ai::talk::DialogueRun::begin();
        self.door = None;
        self.long = None;
        self.pending = None;
        self.evaluate = true;
    }

    pub fn new(actor: &ActorData) -> Walker {
        let m = &actor.transform;
        // Column 1 is the actor's forward (+y) times its scale.
        let (fx, fy) = (m[4], m[5]);
        let scale = (m[0] * m[0] + m[1] * m[1] + m[2] * m[2]).sqrt().max(1e-3);
        Walker::at(
            FormId(actor.reference),
            actor.position,
            fx.atan2(fy),
            scale,
            actor.female,
        )
    }

    pub(crate) fn at(
        reference: FormId,
        position: [f32; 3],
        heading: f32,
        scale: f32,
        female: bool,
    ) -> Walker {
        Walker {
            reference,
            position,
            heading,
            scale,
            package: None,
            package_kind: None,
            target: None,
            target_ref: None,
            path_target: None,
            path: Vec::new(),
            next: 0,
            progress: 0.0,
            face_point: None,
            turn: Turn::default(),
            turn_speed: 90.0,
            rates: PEOPLE_RATES,
            turning: None,
            clock: PackageClock::default(),
            evaluate: true,
            fresh: true,
            settle: true,
            avoidance: Avoidance::default(),
            velocity: [0.0; 3],
            detected_player: i32::MIN,
            greet_look: false,
            social: None,
            talk_run: world::ai::talk::DialogueRun::begin(),
            head_track: HeadTrack::default(),
            head_tracking_off: false,
            kit: None,
            fight: None,
            detect_at: 0.0,
            noticed: Default::default(),
            targets: Default::default(),
            fleeing: None,
            fallen: false,
            paused: false,
            female,
            door: None,
            doors_ahead: Vec::new(),
            radius: 0.0,
            arrival: None,
            facing: None,
            home: position,
            start: None,
            talk_to: None,
            ended: false,
            run: false,
            partial: false,
            long: None,
            parked: false,
            body: None,
            immobile: false,
            wanted: None,
            stuck: mv::Stuck::default(),
            last_move: 0.0,
            against_someone: false,
            stuck_at: None,
            request_radius: mv::REQUEST_RADIUS,
            pending: None,
            last_failure: None,
        }
    }

    /// Their package is looked at afresh at once (after a fight).
    pub(crate) fn forget_package(&mut self, _now: f32) {
        self.package = None;
        self.target = None;
        self.evaluate = true;
    }

    /// Whether they're on a path (walking, or turning to start it).
    pub(crate) fn on_path(&self) -> bool {
        self.next < self.path.len()
    }

    /// Whether they're walking this frame (on a path, not turning in place
    /// first).
    pub(crate) fn walking_now(&self) -> bool {
        self.on_path() && !self.turn.active
    }

    /// A new path (the path handler made for a request, `009dbdc0`): from
    /// its first point, the walk over within `radius` of its last. With
    /// `turn_first` they first turn in place toward it when more than a
    /// degree off (`009e0470` called with no time: a turn request). The
    /// doors of the old path are forgotten (a path made with its doors sets
    /// them after, `go_to`).
    pub(crate) fn set_path(
        &mut self,
        path: Vec<[f32; 3]>,
        radius: f32,
        turn_first: bool,
        settings: &MoveSettings,
    ) {
        self.target = path.last().copied();
        self.path = path;
        self.next = 1;
        self.progress = 0.0;
        self.radius = radius;
        self.partial = false;
        self.pending = None;
        self.avoidance = Avoidance::default();
        // A new path handler: a new stuck test (`009dbdc0`).
        self.stuck = mv::Stuck::default();
        self.last_move = 0.0;
        self.arrival = None;
        self.facing = None;
        self.doors_ahead.clear();
        if turn_first {
            if let Some(&first) = self.path.get(1) {
                let flat = (first[0] - self.position[0]).hypot(first[1] - self.position[1]);
                if flat > 1e-3 {
                    self.turn
                        .request(self.heading, mv::heading_to(self.position, first), settings);
                }
            }
        }
    }

    /// Off the path.
    pub(crate) fn clear_path(&mut self) {
        self.path.clear();
        self.next = 0;
        self.progress = 0.0;
        self.doors_ahead.clear();
    }

    /// Whom the head follows now: the current head-track target, while
    /// head tracking is on (`008a3100`: with `bDisableHeadTracking` the
    /// look eases out).
    pub fn looking_at(&self) -> Option<FormId> {
        self.head_track
            .current()
            .filter(|_| !self.head_tracking_off)
    }

    /// Where its skeleton stands in the world (game axes): turned by its
    /// heading (clockwise from north), at its scale.
    pub fn placement(&self) -> nif::Transform {
        let (s, c) = self.heading.sin_cos();
        nif::Transform {
            rotation: [[c, s, 0.0], [-s, c, 0.0], [0.0, 0.0, 1.0]],
            translation: self.position,
            scale: self.scale,
        }
    }

    /// Where it stands, as a transform in the game's axes (column-major):
    /// turned by its heading, at its scale.
    pub(crate) fn game_matrix(&self) -> [f32; 16] {
        let (s, c) = self.heading.sin_cos();
        let k = self.scale;
        let [x, y, z] = self.position;
        [
            c * k,
            -s * k,
            0.0,
            0.0,
            s * k,
            c * k,
            0.0,
            0.0,
            0.0,
            0.0,
            k,
            0.0,
            x,
            y,
            z,
            1.0,
        ]
    }
}

/// People scripts moved (`MoveTo`) since the last frame: they go where the
/// state now has them.
#[derive(Resource, Default)]
pub struct Moved(pub Vec<FormId>);

/// `--freeze-ai`: nobody's AI runs (the game's console command `tai`,
/// toggle AI): people and creatures stay where they stand, play their idle
/// and don't start conversations, fights or packages.
#[derive(Resource, Default)]
pub struct FrozenAi(pub bool);

/// The navmesh where the player is: the interior's, or outdoors the
/// attached cells' (the `uGridsToLoad` grid around the game's grid centre,
/// `world::ref_scripts::grid_center`, those of its squares loaded here),
/// joined: a cell's navmesh is there while the cell is attached (the
/// path code asks whether a node's cell is attached, `006c9fc0` →
/// `00450ff0`, cell state 6).
#[derive(Resource, Default)]
pub struct CellNav {
    /// The interior, or the worldspace and attached squares, it was loaded
    /// for.
    key: Option<NavKey>,
    mesh: std::sync::Arc<NavMesh>,
    /// Outdoors, the worldspace and the grid's centre.
    center: Option<(FormId, (i32, i32))>,
    /// The navmesh info map, for the long way (`world::ai::navinfo`).
    infos: world::ai::navinfo::NavInfos,
    /// The collision the navmesh's ray casts were last given (its triangle
    /// count): a new snapshot when it changes ([`ColliderPick`]).
    pick_key: Option<usize>,
}

impl CellNav {
    /// The land's height under a point of the worldspace `world` (the
    /// attached squares' `LAND`, as the game's `004572e0` asks a loaded
    /// cell's land); none indoors, in another worldspace, or off the
    /// squares loaded for it.
    pub fn land_height(&self, world: Option<FormId>, p: [f32; 3]) -> Option<f32> {
        match (&self.key, world) {
            (Some((w, Some(_))), Some(now)) if *w == now => self.mesh.land_height(p),
            _ => None,
        }
    }
}

/// The cell's collision as the path builder's ray casts see it
/// (`world::ai::PathPick`; the game's `PATHPICK` picks, `006e6f90`): a copy
/// taken when the loaded collision changes (the game's picks read the live
/// Havok world; moving clutter here is where it was when the copy was
/// taken).
struct ColliderPick(physics::Collider);

impl world::ai::PathPick for ColliderPick {
    fn pick(&self, from: [f32; 3], to: [f32; 3]) -> Option<f32> {
        let d = [to[0] - from[0], to[1] - from[1], to[2] - from[2]];
        let l = (d[0] * d[0] + d[1] * d[1] + d[2] * d[2]).sqrt();
        if l < 1e-4 {
            return None;
        }
        self.0
            .raycast(from, d.map(|v| v / l), l)
            .map(|(hit, _)| hit / l)
    }
}

/// Keeps the navmesh's knowledge of the world up to date: its ray-cast
/// collision, `fJumpFallHeightMin`, the closed doors marked on it
/// (`world::ai::doors`: every swing door that isn't sliding and stands
/// closed; `bCutDoors` 1) and what walkers may do with each door (locked:
/// a hundred times the cost, `006a6fa0`; whether a walker carries the key
/// isn't asked).
fn sync_nav(
    nav: &mut CellNav,
    order: &esm::LoadOrder,
    state: &world::scripting::GameState,
    collision: &physics::Collider,
    doors: &crate::doors::SwingDoors,
) {
    // (The navmesh is shared with the path manager's searches: it's only
    // copied when something here changes.)
    let key = collision.triangle_count();
    if nav.pick_key != Some(key) || nav.mesh.pick.0.is_none() {
        nav.pick_key = Some(key);
        let mesh = std::sync::Arc::make_mut(&mut nav.mesh);
        mesh.pick = world::ai::Picker(Some(std::sync::Arc::new(ColliderPick(collision.clone()))));
        mesh.fall_height =
            world::scripting::game_setting(order, "fJumpFallHeightMin").unwrap_or(256.0);
    }
    let mut closed: Vec<FormId> = Vec::new();
    let mut rules = Vec::new();
    for door in doors.doors.values() {
        let sliding = world::doors::flags(order, door.base) & 0x10 != 0;
        let shut = world::doors::open_state(order, state, door.reference)
            == world::doors::OpenState::Closed;
        if shut && !sliding && !door.boxes.is_empty() {
            closed.push(door.reference);
        }
        let locked = world::locks::lock_now(order, state, door.reference).is_some();
        let rule = if locked {
            world::ai::navsearch::DoorWay::Locked
        } else {
            world::ai::navsearch::DoorWay::Open
        };
        if nav.mesh.door_rules.get(&door.reference) != Some(&rule) {
            rules.push((door.reference, rule));
        }
    }
    closed.sort();
    let marked = nav.mesh.closed_doors();
    let opened: Vec<FormId> = marked
        .iter()
        .filter(|d| closed.binary_search(d).is_err())
        .copied()
        .collect();
    let shut: Vec<FormId> = closed
        .iter()
        .filter(|d| !marked.contains(d))
        .copied()
        .collect();
    if rules.is_empty() && opened.is_empty() && shut.is_empty() {
        return;
    }
    let mesh = std::sync::Arc::make_mut(&mut nav.mesh);
    mesh.door_rules.extend(rules);
    for d in opened {
        mesh.remove_closed_door(d);
    }
    for d in shut {
        if let Some(door) = doors.doors.get(&d.0) {
            let before = mesh.door_triangles.len();
            mesh.add_closed_door(d, &door.boxes);
            println!(
                "Door {d} ({}) is closed: {} navmesh triangles marked under it.",
                door.name,
                mesh.door_triangles.len() - before
            );
        }
    }
}

/// An interior, or a worldspace and its attached squares (sorted).
type NavKey = (FormId, Option<Vec<(i32, i32)>>);

/// What people's moving uses besides the state: scripts, collision, sounds
/// to play, seats (`sitting`), the settings, lines said and conversations.
#[derive(bevy::ecs::system::SystemParam)]
pub struct Around<'w> {
    scripts: Res<'w, crate::scripts::Scripts>,
    collision: ResMut<'w, crate::walk::CellCollision>,
    sounds: ResMut<'w, crate::sounds::SoundRequests>,
    seats: ResMut<'w, Seats>,
    settings: Res<'w, CombatSettings>,
    frozen: Res<'w, FrozenAi>,
    attack: Res<'w, crate::combat::PlayerAttack>,
    hits: ResMut<'w, crate::hiteffects::HitReports>,
    moves: Res<'w, Moves>,
    lines: ResMut<'w, Lines>,
    chats: ResMut<'w, Chats>,
    starts: ResMut<'w, Starts>,
    talk: ResMut<'w, crate::scripts::ScriptedTalk>,
    shots: ResMut<'w, crate::fighting::NpcShots>,
    swing_doors: Res<'w, crate::doors::SwingDoors>,
    paths: ResMut<'w, PathQueue>,
    menus: Option<Res<'w, crate::menus::Menus>>,
    hello: ResMut<'w, PlayerHello>,
    real: Res<'w, Time<Real>>,
}

/// The game settings fights ask for, looked up once
/// (`world::combat_ai::SettingCache`).
#[derive(Resource, Default)]
pub struct CombatSettings(pub world::combat_ai::SettingCache);

/// Everyone the AI runs: not the puppets and copies the co-op live layer
/// drives (`coop_live::CoopPuppet`).
type NpcOnly = Without<crate::coop_live::CoopPuppet>;

/// A person on screen: where and what they're doing.
type Person<'a> = (
    &'a mut Walker,
    &'a mut Life,
    &'a mut ActorRig,
    &'a mut Transform,
    &'a mut Visibility,
);

/// What the last frame knew: the dialogue menu's speaker and the player's
/// feet (for the player's speed).
#[derive(Default)]
pub struct LastFrame {
    speaker: Option<FormId>,
    player: Option<[f32; 3]>,
    /// Who came to warn the trespassing player.
    warner: Option<FormId>,
}

/// Every frame: people look at their package again when due and walk their
/// paths, use furniture, play idles and sandbox (`sitting`), greet, chatter
/// and talk. Time stands still in the dialogue menu, except that the
/// speaker turns to face the player.
#[allow(clippy::too_many_arguments)]
pub fn move_actors(
    time: Res<Time>,
    game: Res<GameFiles>,
    mut state: ResMut<DialogueState>,
    conversation: Res<Conversation>,
    mut nav: ResMut<CellNav>,
    mut talkers: ResMut<Talkers>,
    exterior: Option<Res<crate::exterior::Exterior>>,
    mut moved: ResMut<Moved>,
    mut last: Local<LastFrame>,
    mut commands: Commands,
    around: Around,
    mut actors: Query<Person, NpcOnly>,
    cameras: Query<&Transform, (With<crate::FlyCamera>, Without<Walker>)>,
) {
    let Around {
        scripts,
        mut collision,
        mut sounds,
        mut seats,
        settings,
        attack,
        mut hits,
        frozen: frozen_ai,
        moves,
        mut lines,
        mut chats,
        mut starts,
        mut talk,
        mut shots,
        swing_doors,
        mut paths,
        menus,
        mut hello,
        real,
    } = around;
    let mut came = paths.take_done();
    let order = &game.0.order;
    let state = &mut state.0;
    let settings = &settings.0;
    let moves = &*moves;
    let moved = std::mem::take(&mut moved.0);
    // Someone sent to warn the trespassing player off (or no longer): their
    // package changed, so it's looked at again at once.
    let warner = state.living.trespass.as_ref().map(|w| w.warner);
    if warner != last.warner {
        state.evaluate.extend(warner.into_iter().chain(last.warner));
        last.warner = warner;
    }
    // In the dialogue menu time stands still, except that the speaker
    // turns to face the player. With the AI held still (`--freeze-ai`)
    // people only take the places scripts put them in.
    let in_menu = conversation.0.as_ref().is_some_and(|t| !t.is_line_only());
    // Any other menu up (a script's message box, the Pip-Boy, a container,
    // the game's own menus) is menu mode too: the main loop doesn't update
    // the process lists then (`0086e650`, `00702360`), as the scripts'
    // time stands still in them (`scripts`). The hardcore question after
    // Doc Mitchell's farewell is one.
    let menu_mode = menus.as_deref().is_some_and(crate::menus::Menus::is_open)
        && crate::coop_live::menus_freeze_time();
    let frozen = frozen_ai.0 || in_menu || menu_mode;
    let speaker = conversation.0.as_ref().map(|t| t.speaker());
    let menu_speaker = speaker.filter(|_| in_menu);
    // The dialogue menu closed: the speaker's package is looked at again at
    // once, and they don't greet for `fAIGreetingTimer` (`00762160`).
    let ended = last.speaker.filter(|s| Some(*s) != menu_speaker);
    last.speaker = menu_speaker;
    let key: NavKey = match (state.player_world, state.player_cell, &exterior) {
        (None, Some(c), _) => (c, None),
        (Some(w), _, Some(e)) => {
            use world::ref_scripts::{grid_center, grid_squares, GRIDS_TO_LOAD};
            let feet = state.player_position.unwrap_or_default();
            let before = nav.center.filter(|(world, _)| *world == w).map(|c| c.1);
            let center = grid_center(before, feet, GRIDS_TO_LOAD);
            nav.center = Some((w, center));
            let loaded = e.loaded_squares();
            let mut squares: Vec<(i32, i32)> = grid_squares(center, GRIDS_TO_LOAD)
                .into_iter()
                .filter(|s| loaded.contains(s))
                .collect();
            squares.sort();
            (w, Some(squares))
        }
        _ => return,
    };
    if nav.key.as_ref() != Some(&key) {
        nav.mesh = std::sync::Arc::new(match (&key.1, &exterior) {
            (None, _) => NavMesh::load(order, key.0),
            (Some(squares), Some(e)) => {
                let cells: Vec<FormId> = squares
                    .iter()
                    .filter_map(|s| e.grid.cells.get(s).copied())
                    .collect();
                let mut mesh = NavMesh::load_cells(order, &cells);
                // The land under it (the straight-line test asks how high
                // the ends stand above it).
                for &s in squares {
                    if let Ok(Some(land)) = e.grid.land(order, s) {
                        if let Some(heights) = land.heights {
                            mesh.set_land(s, heights);
                        }
                    }
                }
                mesh
            }
            _ => NavMesh::default(),
        });
        nav.key = Some(key.clone());
        nav.pick_key = None;
    }
    let nav = &mut *nav;
    sync_nav(nav, order, state, &collision.0, &swing_doors);
    let attached: HashSet<(i32, i32)> = key.1.iter().flatten().copied().collect();
    let now = time.elapsed_secs();
    let dt = time.delta_secs();
    // Essential people who went down count their time to get up; fatigue
    // comes back and the knocked out get up (`world::fatigue`); those who
    // stopped fighting leave their combat groups, and `ForceFlee`'s flees
    // end with a fight or death.
    if !frozen {
        world::combat::advance_down(order, state, dt);
        world::fatigue::advance(order, state, dt);
        world::combat_groups::tidy(state);
        world::ai::flee::tidy(state);
    }
    // Attacks' noise wears off (`world::noise::update`, the actors'
    // update `00886360`).
    world::noise::update(state, dt);
    let game_hour = state.global(order, "GameHour").unwrap_or(12.0);
    let player_velocity = match (state.player_position, last.player) {
        (Some(p), Some(q)) if dt > 0.0 => [0, 1, 2].map(|k| (p[k] - q[k]) / dt),
        _ => [0.0; 3],
    };
    last.player = state.player_position;
    let (others, obstacles) = seen(order, state, &attack, now, &actors, player_velocity);
    let bodies = bodies(state, &actors);
    // The world camera, for `MobileObject::Move`'s far rule (`world::ground`).
    let camera = cameras
        .single()
        .ok()
        .map(|c| crate::walk::game_point(c.translation));
    let mut movers = Vec::new();
    let interior = state.player_world.is_none();
    let mut starts = std::mem::take(&mut starts.0);
    // Greetings this frame (`world::social`): the player's update lets the
    // greeting cooldown go once `fHelloCooldownTime` has passed on the
    // real clock (`GetTickCount`, which wraps at 2^32 ms).
    let now_ms = (real.elapsed().as_millis() % (1u128 << 32)) as u32;
    hello.0.update(now_ms, &moves.social);
    let mut greeter = Greeter {
        interior,
        cooldown: &mut hello.0,
        now_ms,
        line_speaker: speaker.filter(|_| !in_menu),
        player_trespassing: state
            .player_cell
            .is_some_and(|c| world::crime::trespassing(order, state, c)),
        player_in_combat: state
            .combat
            .iter()
            .any(|(who, target)| *target == PLAYER_REF && !state.dead.contains(who)),
    };
    // How many chose whom to look at this frame (`011df674`).
    let mut head_track_choices = 0;
    // Everyone's packages looked at once (`GameState::evaluate_everyone`).
    let everyone = std::mem::take(&mut state.evaluate_everyone);
    for (mut walker, mut life, mut rig, mut transform, mut visibility) in &mut actors {
        let walker = &mut *walker;
        let me = walker.reference;
        if everyone {
            state.evaluate.insert(me);
        }
        // The turn in place under way (as the last frame left it) plays the
        // turn animation, while the turn state lasts (at least
        // `fActorTurnAnimMinTime`).
        rig.turning = turning_of(walker, rig.fighting);
        if Some(me) == ended {
            crate::sitting::dialogue_over(state, &mut life, me);
            // `EndDialogue` puts their dialogue package back at its saved
            // step (`008b1070` → `00913250`), DONE once it has talked
            // (`005fa330`); the close then asks for their package
            // (`00762160` → `008da670`).
            walker.talk_run.conversation_over();
            walker.evaluate = true;
            if let Some(s) = walker.social.as_mut() {
                s.greeted(&moves.social);
            }
        }
        // Moved by a script: start again from there, or leave if it's
        // another place.
        if moved.contains(&me) {
            walker.fresh = true;
            walker.clear_path();
            walker.target = None;
            walker.package = None;
            walker.long = None;
        }
        if walker.fresh {
            let away = state
                .spaces
                .get(&me)
                .is_some_and(|(space, _)| *space != key.0);
            if away {
                *visibility = Visibility::Hidden;
                talkers.0.retain(|t| t.reference != me);
            } else if moved.contains(&me) {
                *visibility = Visibility::Inherited;
            }
        }
        // Outdoors, someone standing beyond the attached cells is in the
        // game's lower processes (`009334b0`: their cell isn't attached), who
        // walk out of sight ([`move_offstage`]): hidden here, and back when
        // that walk brings them into an attached square.
        if let Some(place) = state.place(order, me).filter(|_| key.1.is_some()) {
            let here = place.0 == key.0;
            let inside = here && attached.contains(&world::square_of(place.2));
            if !walker.parked
                && here
                && !inside
                && *visibility != Visibility::Hidden
                && !walker.fallen
                && !state.dead.contains(&me)
            {
                *visibility = Visibility::Hidden;
                walker.parked = true;
                walker.clear_path();
                walker.package = None;
                walker.long = None;
                rig.walking = false;
                rig.speed = 0.0;
                talkers.0.retain(|t| t.reference != me);
                println!("{now:.1} s: {me} is beyond the attached cells: out of sight.");
            } else if walker.parked && inside {
                *visibility = Visibility::Inherited;
                walker.parked = false;
                walker.fresh = true;
                // Their square loaded: the pass after an exterior load
                // (`00454450` → `00972d30`).
                walker.settle = true;
                walker.package = None;
                if !talkers.0.iter().any(|t| t.reference == me) {
                    talkers.0.push(crate::dialogue::Talker {
                        reference: me,
                        base: world::scripting::base_of(order, me).unwrap_or(me),
                        position: place.2,
                    });
                }
                println!("{now:.1} s: {me} walks into the attached cells: in sight again.");
            }
        }
        if *visibility == Visibility::Hidden {
            continue;
        }
        // ResetAI is consumed before furniture's early return. It releases
        // furniture directly (0088d640), unlike EvaluatePackage or the
        // ordinary animated stand-up procedure. Never snap to a marker.
        if state.take_ai_reset(me) {
            walker.reset_procedure();
            life.idles.stop();
            life.base_idle = None;
            life.sandbox = None;
            life.activity = None;
            life.getting_up = false;
            rig.dynamic_idle = None;
            rig.overlay = None;
            rig.player.free_special_idle();
            rig.scripted_idle = None;
            println!("{now:.1} s: {me}: ResetAI released furniture and restarted its procedure.");
        }
        // A script's `StartConversation` for them.
        if let Some(i) = starts.iter().position(|s| s.0 == me) {
            let (_, to, topic) = starts.remove(i);
            walker.start = Some((to, topic));
        }
        // Someone who had moved before this place loaded (or in a loaded
        // game) starts from where they got to.
        if walker.fresh {
            walker.fresh = false;
            walker.evaluate = true;
            if let Some(&(mut p, h)) = state.positions.get(&me) {
                // Back from a walk out of sight (not a script's `MoveTo`):
                // that walk goes straight between the route's rough
                // positions (`009ea8a0`), which can lie under the ground
                // or in a hole of the navmesh; nothing holding them there,
                // they're taken onto the navmesh's nearest point (not
                // traced: the game's move into the high process,
                // `PathBuilder::UpdatePathMoveToHigh` and
                // `DetailedActorPathHandler::FindPathStartingLocation`
                // (Xbox PDB), isn't followed).
                if !moved.contains(&me) && nav.mesh.find_triangle(p).is_none() {
                    if let Some(q) = nav
                        .mesh
                        .closest_point(p)
                        .filter(|q| distance(*q, p) < 512.0)
                    {
                        println!(
                            "{now:.1} s: {me} stands off the navmesh after walking out of sight: put on it at ({:.0}, {:.0}, {:.0}).",
                            q[0], q[1], q[2]
                        );
                        p = q;
                        state.positions.insert(me, (p, h));
                    }
                }
                walker.position = p;
                walker.heading = h;
                *transform = Transform::from_matrix(Mat4::from_cols_array(&space::matrix(
                    &walker.game_matrix(),
                )));
                for t in talkers.0.iter_mut() {
                    if t.reference == me {
                        t.position = p;
                    }
                }
            }
        }
        // Stuck last frame: an obstacle where they stand, and the way asked
        // for again.
        // Paths that came back from the path manager.
        if let Some(list) = came.remove(&me) {
            for (id, found, failure) in list {
                if walker.pending.as_ref().is_some_and(|p| p.id == id) {
                    path_came(walker, found, failure, &moves.settings);
                }
            }
        }
        unstick(walker, &mut nav.mesh, &mut paths, &moves.settings, now);
        let mut ask = Asking {
            queue: &mut paths,
            mesh: nav.mesh.clone(),
        };
        // What asked them to look at someone holds while it lasts and then
        // lets go (`world::head_track`).
        head_track_asks(walker, speaker, in_menu, &lines, &chats, &moves.head_track);
        let before = walker.position;
        walker.turning = None;
        // Talking to the player in the dialogue menu: the world's update
        // stands still (the main loop `0086e650` skips the process lists in
        // menu mode); the menu itself updates the speaker every frame
        // (`00762950` → `Actor::UpdateInDialogue`, Xbox PDB, `008a5580`):
        // they stop where they are (the walk held, not dropped) and, unless
        // seated or in combat, turn in place to face the player whenever
        // more than a degree off and not turning already
        // (`world::dialogue_view::speaker_turns`).
        let seated = state.furniture.contains_key(&me) || state.sitters.contains_key(&me);
        if menu_speaker == Some(me) {
            // A line of their own (chatter, a conversation) gives way.
            if lines.is_saying(me) {
                crate::chatter::hush(&mut commands, &mut lines, me);
            }
            end_chat(&mut chats, me);
            if walker.paused {
                walker.paused = false;
                rig.still = false;
            }
            rig.walking = false;
            rig.speed = 0.0;
            let in_combat = state.combat.contains_key(&me);
            if let Some(p) = state.player_position {
                let toward = mv::heading_to(before, p);
                if world::dialogue_view::speaker_turns(
                    walker.heading,
                    toward,
                    walker.turn.active,
                    seated,
                    in_combat,
                ) {
                    walker.turn.request(walker.heading, toward, &moves.settings);
                }
            }
            // The turn under way plays out (`ActorMover::UpdateMovement`,
            // Xbox PDB, with the mover's dialogue flag set, `009c9900`).
            if walker.turn.active && !seated {
                let rate = walker.rates[usize::from(in_combat)];
                if let Some(side) = walker.turn.update(&mut walker.heading, dt, rate) {
                    walker.turning = Some(side);
                }
            }
            // Their talking idles (`sitting::dialogue_frame`).
            let said = conversation.0.as_ref().and_then(|t| t.said());
            crate::sitting::dialogue_frame(
                &game.0, state, &mut seats, walker, &mut life, &mut rig, said, now,
            );
            place(walker, &mut transform, state, &mut talkers);
            continue;
        }
        // The death routine (`Actor::Kill`, Xbox PDB, `0089d900`) makes the
        // bodies dynamic as the actor dies, not the AI's update: held still
        // by `tai` (`--freeze-ai`) or in the dialogue menu, the dead go limp
        // all the same (the ragdoll then waits for the menu to close:
        // `actors::animate_actors`).
        if frozen && dies_while_frozen(state.dead.contains(&me), walker.fallen) {
            fall(
                walker,
                &mut life,
                &mut rig,
                &mut transform,
                state,
                order,
                &collision,
                now,
            );
            crate::chatter::hush(&mut commands, &mut lines, me);
            end_chat(&mut chats, me);
            continue;
        }
        if frozen {
            // The game stops the world in the dialogue menu: everyone else
            // holds still where they are, mid-stride included.
            if (in_menu || menu_mode) && !walker.paused && !state.dead.contains(&me) {
                walker.paused = true;
                rig.still = true;
            }
            continue;
        }
        if walker.paused {
            walker.paused = false;
            if !state.dead.contains(&me) {
                rig.still = false;
            }
        }
        // The dead go limp (their skeleton's ragdoll, thrown by the killing
        // blow) and do nothing more; without a ragdoll they tip over.
        // Essential people brought to 0 health lie limp for a time (the dead
        // do, for good), then get up where they lay; so do those knocked
        // out by fatigue or paralysis (`world::fatigue`, until their knock
        // state moves on).
        if world::fatigue::lies_down(state, me) {
            if !walker.fallen {
                fall(
                    walker,
                    &mut life,
                    &mut rig,
                    &mut transform,
                    state,
                    order,
                    &collision,
                    now,
                );
                end_chat(&mut chats, me);
            }
            continue;
        }
        if walker.fallen && !state.dead.contains(&me) {
            walker.fallen = false;
            rig.ragdoll = None;
            rig.still = false;
            walker.clear_path();
            place(walker, &mut transform, state, &mut talkers);
        }
        if state.dead.contains(&me) {
            if !walker.fallen {
                fall(
                    walker,
                    &mut life,
                    &mut rig,
                    &mut transform,
                    state,
                    order,
                    &collision,
                    now,
                );
                crate::chatter::hush(&mut commands, &mut lines, me);
                end_chat(&mut chats, me);
            }
            continue;
        }
        // Knocked out by a script (`SetUnconscious 1`, life state 3): they
        // notice nobody, fight nobody and follow no package until woken
        // (inferred from the process update skipping the unconscious;
        // how they lie isn't shown).
        if state.unconscious.contains(&me) {
            rig.walking = false;
            rig.running = false;
            rig.speed = 0.0;
            rig.fighting = false;
            continue;
        }
        if walker.kit.is_none() {
            let kit = crate::fighting::Kit::read(order, state, walker, &rig.skeleton);
            // Their turning speed and in-place rates (`world::movement`).
            let speed = mv::turning_speed(order, me, &moves.settings);
            let creature = kit.creature.is_some();
            walker.turn_speed = speed;
            walker.rates = [
                moves.settings.in_place_rate(speed, creature, false),
                moves.settings.in_place_rate(speed, creature, true),
            ];
            walker.kit = Some(kit);
            walker.immobile = world::scripting::base_of(order, me)
                .is_some_and(|b| world::ground::immobile(order, b));
            walker.request_radius = world::ai::request_radius(order, me);
        }
        // A combat style a script gave them (`SetCombatStyle`) is theirs at
        // once, in a fight under way too (`008a8010`).
        if let Some(&style) = state.more.combat_styles.get(&me) {
            if let Some(kit) = walker
                .kit
                .as_mut()
                .filter(|k| k.style.form_id != Some(style))
            {
                kit.style = world::more_functions::combat_style(order, state, me);
            }
        }
        if walker.social.is_none() {
            let mut dice = crate::fighting::Dice::new(state);
            walker.social = Some(Social::new(&moves.social, &mut || dice.unit()));
        }
        // Noticing (`fighting::detect`): each actor's detection run, every
        // 0.3 s in combat, staggered out of it; none for actors 8192 or
        // more from the player.
        let near_player = state
            .player_position
            .is_some_and(|p| distance(p, walker.position) < world::combat_ai::DETECTION_RANGE);
        if now >= walker.detect_at && near_player {
            let in_combat = state.combat.contains_key(&me);
            let unit = crate::fighting::Dice::new(state).unit();
            let s = |n: &str, d: f32| settings.get(order, n, d);
            walker.detect_at =
                now + world::combat_ai::detection_interval(in_combat, dt, others.len(), unit, &s);
            let noticing = crate::fighting::Noticing {
                order,
                settings,
                collision: &collision.0,
                now,
            };
            crate::fighting::detect(&noticing, state, walker, &others);
        }
        // Whom they look at of their own accord (`008a3100`), seated too.
        choose_head_track(
            walker,
            state,
            &others,
            &collision.0,
            &moves.head_track,
            &mut head_track_choices,
            dt,
        );
        // A fight over (its target dead, or a script's `StopCombat`): back
        // to their package; the next fight starts afresh
        // (`world::npc_combat`).
        // A target killed: the next of their targets, if any is left
        // (`fighting::next_target`).
        if !state.combat.contains_key(&me) {
            crate::fighting::target_killed(order, state, walker, &others, now);
        }
        if !state.combat.contains_key(&me) {
            world::npc_combat::combat_over(state, me);
            if walker.fight.is_some() {
                crate::fighting::end_fight(walker, now);
            }
        }
        rig.fighting = state.combat.contains_key(&me);
        if rig.fighting {
            end_chat(&mut chats, me);
        }
        if let Some(s) = walker.social.as_mut() {
            s.tick(dt);
        }
        // Furniture first: sitting down, seated, getting up (a fight gets
        // them up with the fast exit); nothing else moves them meanwhile.
        let talking = lines.is_saying(me);
        let mut ctx = Ctx {
            game: &game.0,
            state: &mut *state,
            seats: &mut seats,
            mesh: &nav.mesh,
            moves: &moves.settings,
            now,
            dt,
            fighting: rig.fighting,
            talking,
        };
        // The game's package check runs while sit state is 0, 4 or 9
        // (`008da670`): forced, on the 20 s timer or a new hour, also while
        // a settled actor is still in furniture; otherwise this early-return
        // path prevents the package change from requesting the stand-up.
        let package_checked_before_furniture = rethink_queued_package_before_furniture(
            &mut ctx, walker, &mut life, game_hour, &mut ask,
        );
        let in_furniture = crate::sitting::furniture_frame(&mut ctx, walker, &mut life, &mut rig);
        if in_furniture {
            rig.walking = false;
            rig.speed = 0.0;
            begin_conversation(&mut ctx, walker, &mut chats);
            let chatting = chats.0.contains_key(&me);
            if chatting {
                chat_frame(&mut ctx, walker, &mut chats, &mut lines, moves);
            } else if walker.talk_to.is_some() {
                talk_frame(&mut ctx, walker, &mut talk, moves);
            } else if life.sandbox.is_some() && !rig.fighting {
                crate::sitting::sandbox_frame(
                    &mut ctx,
                    walker,
                    &mut life,
                    &mut chats,
                    &moves.social,
                );
            }
            crate::sitting::idles_frame(&mut ctx, walker, &mut life, &mut rig);
            if !rig.fighting {
                social_frame(
                    &mut ctx,
                    walker,
                    &mut chats,
                    &mut lines,
                    moves,
                    &others,
                    &mut greeter,
                );
            }
            place(walker, &mut transform, state, &mut talkers);
            walker.velocity = [0.0; 3];
            continue;
        }
        if let Some(&target) = state.combat.get(&me) {
            life.activity = None;
            life.idles.stop();
            rig.overlay = None;
            let kit = walker.kit.clone().expect("read above");
            let mut ctx = crate::fighting::FightCtx {
                order,
                scripts: &scripts.0,
                settings,
                mesh: &nav.mesh,
                sounds: &mut sounds,
                hits: &mut hits,
                shots: &mut shots,
                others: &others,
                moves: &moves.settings,
                now,
                dt,
            };
            let frame = crate::fighting::fight(&mut ctx, state, walker, &kit, target);
            // Their script's `OnStartCombat`, once the target is detected
            // (seen with a detection value above 0; `world::npc_combat`).
            let detected = walker
                .fight
                .as_ref()
                .is_some_and(|f| f.memory.times_seen > 0);
            world::npc_combat::start_combat_event(
                &mut world::scripting::Runner::new(order, &scripts.0, state),
                me,
                detected,
            );
            rig.walking = frame.gait.is_some();
            rig.running = frame.gait == Some(world::combat_ai::Gait::Run);
            rig.speed = frame.gait.map_or(0.0, |g| {
                g.speed(kit.walk, kit.run) * world::body_parts::leg_speed_mult(order, state, me)
            });
            if frame.attacked {
                rig.attack_at = Some(now);
            }
            move_body(
                walker,
                &mut collision.0,
                &bodies,
                &mut movers,
                &Ground {
                    mesh: &nav.mesh,
                    camera,
                },
                dt,
            );
            place(walker, &mut transform, state, &mut talkers);
            walker.velocity = velocity(before, walker.position, dt);
            continue;
        }
        // Running from someone they won't fight.
        if walker.fleeing.is_some() {
            life.activity = None;
            life.idles.stop();
            rig.overlay = None;
            let kit = walker.kit.clone().expect("read above");
            let moving = crate::fighting::flee(order, settings, &nav.mesh, state, walker, &kit, dt);
            rig.walking = moving && walker.walking_now();
            rig.running = moving;
            rig.speed = if moving { kit.run } else { 0.0 };
            if walker.fleeing.is_none() {
                walker.forget_package(now);
            }
            move_body(
                walker,
                &mut collision.0,
                &bodies,
                &mut movers,
                &Ground {
                    mesh: &nav.mesh,
                    camera,
                },
                dt,
            );
            place(walker, &mut transform, state, &mut talkers);
            walker.velocity = velocity(before, walker.position, dt);
            continue;
        }
        rig.running = false;
        let mut ctx = Ctx {
            game: &game.0,
            state: &mut *state,
            seats: &mut seats,
            mesh: &nav.mesh,
            moves: &moves.settings,
            now,
            dt,
            fighting: false,
            talking,
        };
        // A conversation with someone (or coming up to the player for one):
        // it runs instead of the package.
        begin_conversation(&mut ctx, walker, &mut chats);
        let chatting = chats.0.contains_key(&me);
        if chatting {
            chat_frame(&mut ctx, walker, &mut chats, &mut lines, moves);
        } else if walker.talk_to.is_some() {
            talk_frame(&mut ctx, walker, &mut talk, moves);
        } else if let Some(flee) = ctx.state.forced_flee.get(&me).copied() {
            // `ForceFlee`'s engine flee package in place of theirs.
            life.activity = None;
            life.idles.stop();
            forced_flee_frame(&mut ctx, walker, flee);
        } else {
            // The package, looked at again when due (`008da670`): forced,
            // none, every 20 s, a new game hour; only in sit states 0, 4, 9.
            // A settled actor may have been checked immediately before the
            // furniture procedure. Leave any new request raised by a
            // procedure that ended this frame for the next frame; don't tick
            // the clock or evaluate twice.
            let forced = if package_checked_before_furniture {
                false
            } else {
                take_forced_package_evaluation(walker, ctx.state, me)
            };
            let due = !package_checked_before_furniture
                && walker
                    .clock
                    .due(dt, game_hour, forced, walker.package.is_some());
            if due && !life.getting_up {
                rethink(&mut ctx, walker, &mut life, &mut ask);
            }
            // A dialogue package: walk up and talk (`008e8600`).
            if walker.package_kind == Some(world::ai::kinds::DIALOGUE) && !walker.talk_run.waiting {
                dialogue_frame(&mut ctx, walker, (&mut chats, &mut lines, &mut talk), moves);
            }
            // A sandbox: the game's choices (`sitting`).
            if life.sandbox.is_some() {
                crate::sitting::sandbox_frame(
                    &mut ctx,
                    walker,
                    &mut life,
                    &mut chats,
                    &moves.social,
                );
            }
            // Following someone, or walking to a reference that moves: a new
            // path when it moved `fAIMoveDistanceToRecalcFollowPath` since the
            // last, or they stand farther than the radius from it.
            follow_target(&mut ctx, walker, moves, &mut ask);
            // A place beyond the attached cells' navmesh: the long way.
            if let Some(squares) = &key.1 {
                long_walk(
                    &mut ctx,
                    walker,
                    &mut nav.infos,
                    (key.0, squares),
                    &attached,
                    moves.recalc_follow,
                    &mut ask,
                );
            }
            // A wander package at its place: the wander procedure.
            if walker.package_kind == Some(world::ai::kinds::WANDER) {
                crate::sitting::wander_package_frame(&mut ctx, walker, &mut life);
            }
            // A guard package: to its post, then hold it (`00902290`).
            if walker.package_kind == Some(world::ai::kinds::GUARD) && !life.getting_up {
                guard_frame(&mut ctx, walker, &mut life);
            }
            // A flee package (`008ddac0`).
            if walker.package_kind == Some(world::ai::kinds::FLEE) && !life.getting_up {
                flee_frame(&mut ctx, walker);
            }
        }
        // The game's walking speed (`world::animation::walk_speed`:
        // `fMoveBaseSpeed` 77 × SpeedMult ÷ 100 × the legs' condition),
        // times their scale; the walk animation is played at the rate that
        // makes its root travel this (`actors`). Running: the run speed.
        let speed = if walker.run {
            world::animation::run_speed(order, ctx.state, me)
        } else {
            world::animation::walk_speed(order, ctx.state, me)
        } * walker.scale;
        // The turn as this update began (for the mover's own step below).
        let turn_before = (walker.turn, walker.heading);
        let was_on_path = walker.on_path();
        // Others in the way: wait, or a way round (`009e5ae0`).
        let mut blocked = false;
        if walker.walking_now() {
            let ahead = [walker.heading.sin(), walker.heading.cos()];
            let running = rig.running;
            let target_ref = walker.target_ref;
            let mine: Vec<Obstacle> = obstacles.iter().filter(|o| o.who != me).copied().collect();
            let step = walker.avoidance.update(
                walker.position,
                mv::REQUEST_RADIUS,
                ahead,
                walker.velocity,
                running,
                target_ref,
                &mine,
                &moves.settings.avoidance,
                dt,
            );
            match step {
                AvoidStep::Clear => {}
                AvoidStep::Wait => blocked = true,
                AvoidStep::Repath(nodes) => {
                    if let Some(goal) = walker.path.last().copied() {
                        if let Some(path) = path_avoiding_for(&nav.mesh, walker, goal, &nodes) {
                            // The same walk, round them: its radius, end
                            // heading and the doors still ahead kept.
                            let keep = std::mem::take(&mut walker.avoidance);
                            let doors = std::mem::take(&mut walker.doors_ahead);
                            let (radius, arrival) = (walker.radius, walker.arrival);
                            walker.set_path(path, radius, false, &moves.settings);
                            walker.avoidance = keep;
                            walker.arrival = arrival;
                            walker.doors_ahead = doors;
                            println!("{:.1} s: {me} goes round {} in the way.", now, nodes.len());
                        }
                    }
                }
            }
        }
        // A closed door across the path: they open it and wait while it
        // swings (`doors::walker_at_door`, the game's `009e20c0`), standing.
        let at = walker.position;
        if walker.next >= walker.path.len() {
            walker.doors_ahead.clear();
        }
        let waiting = !walker.doors_ahead.is_empty()
            && crate::doors::walker_at_door(
                order,
                ctx.state,
                &mut sounds,
                me,
                at,
                &mut walker.doors_ahead,
            );
        // Waiting for a path from the path manager: standing.
        let asking = walker.pending.is_some();
        let on_path = if blocked || waiting || asking {
            walker.on_path()
        } else {
            step(walker, speed, dt)
        };
        // The attached cells' part of the long way walked: they stand there
        // short of the place (the travel procedure asks for the path again,
        // `008e5e90`, and gets the same while the attached cells stay).
        if walker.stuck_at.is_some() {
            // Stuck: the walk failed, it isn't over ([`unstick`]).
        } else if was_on_path && !on_path && walker.partial {
            walker.partial = false;
            walker.arrival = None;
            println!(
                "{now:.1} s: {me} stops at the edge of the attached cells, short of their place."
            );
        } else if was_on_path && !on_path && walker.door.is_some() {
            // At a load door toward somewhere else: through it.
            go_through(order, ctx.state, walker);
            life.activity = None;
            continue;
        } else if was_on_path && !on_path {
            // The walk over: they turn in place to the travel's end heading
            // (`008e5e90` → `008bb5c0`), not while using furniture.
            walker.facing = walker.arrival.take();
            // A travel package's walk over: its procedures reach `DONE`
            // (`0091ecf0`: the end action). Not a travel to furniture
            // (`sitting` carries that on).
            if walker.package_kind == Some(world::ai::kinds::TRAVEL)
                && !ctx.state.furniture.contains_key(&me)
            {
                package_done(ctx.state, walker);
            }
        }
        if let Some(h) = walker.facing.filter(|_| !on_path) {
            let using =
                ctx.state.furniture.contains_key(&me) || ctx.state.sitters.contains_key(&me);
            if using || !face(walker, h, dt, false, &moves.settings) {
                walker.facing = None;
            }
        }
        // Walking: the walk plays at the rate that makes its root travel
        // their speed (`actors`); waiting for others or a door, turning in
        // place first or after: not walking.
        let walking = on_path && walker.walking_now() && !blocked && !waiting && !asking;
        rig.walking = walking;
        rig.running = walking && walker.run;
        rig.speed = if walking { speed } else { 0.0 };
        // Greeting the player, idle chatter, starting to talk with others.
        social_frame(
            &mut ctx,
            walker,
            &mut chats,
            &mut lines,
            moves,
            &others,
            &mut greeter,
        );
        // Looking at whom they spoke to: the body turns past 80° off
        // (`008a3100`).
        look_frame(&mut ctx, walker, rig.fighting, moves);
        // Idles (`sitting`): once a second of free time, the idle tree.
        crate::sitting::idles_frame(&mut ctx, walker, &mut life, &mut rig);
        // A turn in place nothing above carried on this update (a walk held
        // up by someone in the way, a door, a path being made; a path over
        // before its first turn): the mover turns them all the same each
        // update, whatever the walk is doing (`ActorMover::UpdateMovement`,
        // Xbox PDB, `009c9900` → `009e7d70`). Left alone the turn stayed on,
        // its animation playing in place.
        if walker.turn.active
            && (walker.turn, walker.heading) == turn_before
            && !ctx.state.furniture.contains_key(&me)
            && !ctx.state.sitters.contains_key(&me)
        {
            let side = walker.turn.update(&mut walker.heading, dt, walker.rates[0]);
            if side.is_some() {
                walker.turning = side;
            }
        }
        // Turned in place or walked: the controller moves them.
        move_body(
            walker,
            &mut collision.0,
            &bodies,
            &mut movers,
            &Ground {
                mesh: &nav.mesh,
                camera,
            },
            dt,
        );
        place(walker, &mut transform, state, &mut talkers);
        walker.velocity = velocity(before, walker.position, dt);
    }
    crate::clutter::walkers(movers);
    for (speaker, ..) in starts {
        println!("A script has {speaker} start a conversation, but they aren't loaded here.");
    }
}

/// The side someone turns in place, and the turn animation's rate (the
/// in-place scale: 1.5, in combat 2.5; creatures 1.25), while they're
/// turning.
fn turning_of(walker: &Walker, combat: bool) -> Option<(TurnSide, f32)> {
    if !walker.turn.active {
        return None;
    }
    let side = if walker.turn.right {
        TurnSide::Right
    } else {
        TurnSide::Left
    };
    let base = (walker.turn_speed * mv::ONE_DEGREE).max(1e-6);
    Some((side, walker.rates[usize::from(combat)] / base))
}

fn velocity(before: [f32; 3], after: [f32; 3], dt: f32) -> [f32; 3] {
    if dt <= 0.0 {
        return [0.0; 3];
    }
    [0, 1, 2].map(|k| (after[k] - before[k]) / dt)
}

/// Whether someone goes limp now though the AI is held still (`tai`, the
/// dialogue menu): dead and not yet fallen. The death routine `0089d900`
/// runs from `Kill` and the damage paths, not from the AI's update.
fn dies_while_frozen(dead: bool, fallen: bool) -> bool {
    dead && !fallen
}

/// Someone has just died: limp (their skeleton's ragdoll, thrown as the
/// game throws the dead), else tipped over.
#[allow(clippy::too_many_arguments)]
fn fall(
    walker: &mut Walker,
    life: &mut Life,
    rig: &mut ActorRig,
    transform: &mut Transform,
    state: &mut world::scripting::GameState,
    order: &esm::LoadOrder,
    collision: &crate::walk::CellCollision,
    now: f32,
) {
    walker.fallen = true;
    walker.clear_path();
    rig.walking = false;
    // How the death began (`world::combat::DeathStart`); none for the dead
    // at load, who already lie where they fell. Essential people going
    // down aren't dead (no death routine's nudge) but fall as their blow
    // throws them.
    let start = state.deaths.remove(&walker.reference);
    let down = world::fatigue::lies_down(state, walker.reference);
    // Every body gains the death routine's nudge (`0089d900`): along the
    // way they were moving, else the way they face, or away from the
    // player who killed them.
    // The nudge is added to what the bodies already move at. Alive, the
    // skeleton's bodies are keyframed (`00930c70`: motion type 4) and the
    // blend object (`bhkBlendCollisionObject` `00c81e30` → `00c65b00`)
    // either places them (vtable +0xe8) or hard-keyframes them, giving
    // them the velocity that reaches the animation's next pose
    // (`00c8e160`); which branch the living biped takes isn't traced.
    // [G] With `NV_GUESSES=1` the actor's own velocity stands for each
    // body's (the limbs' swing within the animation isn't added).
    let nudge = start.map(|s| {
        let player_line = state
            .player_position
            .filter(|_| s.killer == Some(PLAYER_REF))
            .map(|p| (walker.position, p));
        let n = world::combat::death_nudge(order, walker.velocity, walker.heading, player_line);
        let carried = if world::guesses::enabled() {
            walker.velocity
        } else {
            [0.0; 3]
        };
        [0, 1, 2].map(|i| carried[i] + n[i])
    });
    // Then, for a killing hit, thrown as the game throws the dead: by the
    // killer's weapon (or the damage, without one), away from a point
    // 2048 units back along the blow from where it struck (the chest).
    let hit = start.is_some_and(|s| s.hit) || down;
    let push = state
        .last_blow
        .get(&walker.reference)
        .filter(|_| hit)
        .and_then(|&(by, damage)| {
            let from = if by == PLAYER_REF {
                state.player_position
            } else {
                state.place(order, by).map(|p| p.2)
            }?;
            let struck = [
                walker.position[0],
                walker.position[1],
                walker.position[2] + 90.0,
            ];
            let direction = [
                struck[0] - from[0],
                struck[1] - from[1],
                struck[2] - (from[2] + 90.0),
            ];
            let weapon = world::combat::weapon_in_hand(order, state, by);
            let ranged = weapon.as_ref().is_some_and(|w| !w.is_melee());
            let across = (direction[0]).hypot(direction[1]);
            let speed = world::combat::death_push(order, weapon.as_ref(), damage, ranged, across);
            Some((world::combat::death_push_origin(struck, direction), speed))
        });
    let fresh = start.is_some() || down;
    if let Some(v) = nudge {
        println!(
            "{} goes limp at ({:.0}, {:.0}, {:.0}) moving ({:.1}, {:.1}, {:.1}) units/s{}",
            walker.reference,
            walker.position[0],
            walker.position[1],
            walker.position[2],
            v[0],
            v[1],
            v[2],
            if push.is_some() {
                ", then the hit's push"
            } else {
                ""
            }
        );
    }
    // From the pose they're in (seated, if sitting).
    let limp = rig.go_limp(now, walker.placement(), nudge, push);
    rig.dynamic_idle = None;
    rig.overlay = None;
    state.stand(walker.reference);
    life.idles.stop();
    if limp {
        // Dead since before this place loaded: already lying where they
        // fell.
        if !fresh {
            if let Some(dead) = rig.ragdoll.as_mut() {
                for _ in 0..(4.0 / physics::ragdoll::STEP) as usize {
                    dead.sim.step(&collision.0);
                    if dead.sim.asleep {
                        break;
                    }
                }
            }
        }
    } else {
        rig.still = true;
        *transform = fallen_transform(walker);
    }
}

/// Puts someone's root where they are, and tells the state and the
/// talkers.
fn place(
    walker: &Walker,
    transform: &mut Transform,
    state: &mut world::scripting::GameState,
    talkers: &mut Talkers,
) {
    let new = Transform::from_matrix(Mat4::from_cols_array(&space::matrix(&walker.game_matrix())));
    if *transform == new {
        return;
    }
    *transform = new;
    state
        .positions
        .insert(walker.reference, (walker.position, walker.heading));
    for t in talkers.0.iter_mut() {
        if t.reference == walker.reference {
            t.position = walker.position;
        }
    }
}

/// Looks at the person's package again: a new place to go gets a new path
/// (getting up from any seat first: the game's `StandUp`, and this again
/// once they're up). A travel to furniture uses it; a sandbox package gets
/// its area (`sitting`). The same package goes on as it was, even when the
/// look is forced (`0090a1a0`).
fn rethink(ctx: &mut Ctx, walker: &mut Walker, life: &mut Life, ask: &mut Asking) {
    let game = ctx.game;
    let order = &game.order;
    let me = walker.reference;
    let settle = std::mem::take(&mut walker.settle);
    let state = &mut *ctx.state;
    let package = current_package(order, state, me);
    let near = package
        .as_ref()
        .and_then(|p| destination(order, state, me, p));
    // Somewhere else: the load door that leads there.
    let here = state.place(order, me).map(|p| p.0);
    let way = match (&package, near) {
        (Some(p), None) => world::ai::target_place(order, state, me, p)
            .filter(|(space, _)| Some(*space) != here)
            .and_then(|(space, _)| world::ai::door_toward(order, state, me, space)),
        _ => None,
    };
    // A load door: walked to as the path request resolves it, the navmesh
    // point nearest the door (the door itself stands in its frame, where
    // nobody's controller gets); the game then activates the door from the
    // path once its triangles are just ahead (`009e20c0` → `009e22d0`, the
    // triangles under the door flagged 0x1000 by `006997e0`, not
    // registered here), so here at that point's arrival, within the radius
    // a travel to the door reference would have (`00678670`: half its
    // bounds' diagonal + 20; taking the door as the travel's location is
    // an inference).
    let goal = near.or(way.map(|w| {
        let at = ctx.mesh.closest_point(w.at).unwrap_or(w.at);
        let half = world::ai::half_bounds_diagonal(order, w.door);
        let radius = mv::location_radius(
            0,
            mv::Spot::Object {
                half_diagonal: half,
            },
            0.0,
        );
        (at, radius)
    }));
    let package_id = package.as_ref().map(|p| p.form_id);
    let target_ref = package
        .as_ref()
        .and_then(|p| p.location)
        .filter(|l| l.kind == 0)
        .map(|l| l.form)
        .or_else(|| package.as_ref().and_then(world::ai::followed).map(|f| f.0));
    // The same package: carry on, forced or not (`0090a1a0`: the package
    // chosen is the current one, so no new start and its step is kept; a
    // new walk only if its place moved off while they were idle, which
    // `follow_target` sees to). A dialogue package that has talked stays
    // done (`world::ai::talk`).
    if package_id == walker.package {
        return;
    }
    walker.talk_run = world::ai::talk::DialogueRun::begin();
    // Something new: up first if seated (or sitting down).
    let using = state.furniture.get(&me).copied();
    if let Some(sitter) = state.sitters.get_mut(&me) {
        if sitter.state != SitState::Normal {
            if using != target_ref || target_ref.is_none() {
                sitter.stand_up();
                life.getting_up = true;
            }
            return;
        }
    }
    if using.is_some() && using != target_ref {
        state.stand(me);
    }
    walker.door = way;
    walker.long = None;
    walker.package = package_id;
    walker.package_kind = package.as_ref().map(|p| p.kind);
    walker.ended = false;
    walker.run = false;
    // The package begins: its begin action (`0090a1a0` → +0x598), unless
    // `AddScriptPackage` already began it.
    if let Some(id) = package_id.filter(|id| id.0 != 0) {
        world::ai::actions::begin(state, me, id, false);
    }
    walker.target = goal.map(|g| g.0);
    walker.target_ref = target_ref;
    walker.path_target = None;
    walker.clear_path();
    walker.radius = goal.map_or(0.0, |(_, r)| r);
    walker.home = walker.position;
    walker.arrival = None;
    walker.facing = None;
    life.activity = None;
    // A sandbox: its area around where the package puts it (near a
    // reference, the editor location), else where they are now ("near
    // the current location", and "in a cell": the latter's centre isn't
    // traced); its radius the package's, else the search radius.
    life.sandbox = package
        .as_ref()
        .filter(|p| p.kind == world::ai::kinds::SANDBOX)
        .map(|p| {
            let center = match p.location.map(|l| l.kind) {
                Some(0) | Some(3) | Some(6) => near.map_or(walker.position, |(c, _)| c),
                _ => walker.position,
            };
            let radius = p.location.map_or(0, |l| l.radius);
            let s = world::sandbox::Sandbox::new(
                p.form_id,
                center,
                radius,
                world::sandbox::package_flags(order, p.form_id),
                world::sandbox::energy(order, me),
                &ctx.seats.sandbox,
            );
            println!(
                "{me} sandboxes ({}) within {:.0} of {:.0},{:.0},{:.0}",
                p.editor_id.clone().unwrap_or_default(),
                s.radius,
                center[0],
                center[1],
                center[2]
            );
            s
        });
    if life.sandbox.is_some() {
        return;
    }
    // Guard and flee packages run their own procedures each frame
    // (`guard_frame`, `flee_frame`).
    if matches!(
        walker.package_kind,
        Some(world::ai::kinds::GUARD) | Some(world::ai::kinds::FLEE)
    ) {
        return;
    }
    // A travel to a piece of furniture: to use it (the travel procedure's
    // furniture case, `00915f10`).
    if let Some(f) = target_ref.filter(|f| world::scripting::GameState::is_furniture(order, *f)) {
        if state.furniture.get(&me) == Some(&f) {
            return;
        }
        // Just loaded: the pass after the load seats them there at once
        // (`00972d30` → `0088d2f0`); the package's target first, then its
        // location's reference.
        if settle && !ctx.fighting {
            let candidates: Vec<FormId> = package
                .as_ref()
                .and_then(world::ai::followed)
                .map(|f| f.0)
                .into_iter()
                .chain(
                    package
                        .as_ref()
                        .and_then(|p| p.location)
                        .filter(|l| l.kind == 0)
                        .map(|l| l.form),
                )
                .collect();
            if crate::sitting::seat_on_load(ctx, walker, life, &candidates) {
                return;
            }
        }
        if crate::sitting::begin_use(ctx, walker, f) {
            return;
        }
    }
    let state = &mut *ctx.state;
    // A dialogue package walks its own way (`dialogue_frame`).
    if walker.package_kind == Some(world::ai::kinds::DIALOGUE) {
        return;
    }
    let Some((to, radius)) = goal else {
        return;
    };
    // At the travel's end they face an `XMarkerHeading`'s heading (or
    // their editor heading near the editor location; `008e5e90`).
    let arrival = package
        .as_ref()
        .filter(|_| way.is_none())
        .and_then(|p| world::ai::arrival_heading(order, state, me, p));
    if way.is_none() && mv::arrived(walker.position, to, radius) {
        walker.facing = arrival;
        // Already there: the travel procedure is over at once.
        if walker.package_kind == Some(world::ai::kinds::TRAVEL) {
            package_done(state, walker);
        }
        return;
    }
    if way.is_some() && distance(walker.position, to) <= radius.max(1.0) {
        go_through(order, state, walker);
        return;
    }
    // A place beyond the attached navmesh: the long way ([`long_walk`]).
    if beyond_attached(ctx.mesh, to) {
        return;
    }
    // The travel's path request, to the path manager ([`ask_path`]).
    let path_target = target_ref.and_then(|t| state.place(order, t)).map(|p| p.2);
    walker.path_target = path_target;
    let note = package
        .as_ref()
        .and_then(|p| p.editor_id.clone())
        .unwrap_or_default();
    let then = Pending {
        id: 0,
        radius,
        turn_first: true,
        arrival,
        path_target,
        partial: false,
        note,
    };
    let shared = ask.mesh.clone();
    ask_path(ask.queue, &shared, walker, to, then);
}

/// Consume both queued force sources without short-circuiting. `EvaluatePackage`
/// sets state.evaluate; other viewer transitions can set walker.evaluate.
fn take_forced_package_evaluation(
    walker: &mut Walker,
    state: &mut world::scripting::GameState,
    me: FormId,
) -> bool {
    let walker_forced = std::mem::take(&mut walker.evaluate);
    let state_forced = state.evaluate.remove(&me);
    walker_forced || state_forced
}

/// The package check (`008da670`) for a settled sitter, before the
/// furniture path returns early: forced, none, the 20 s timer or a new game
/// hour, as for anyone (sit states 0, 4 and 9 allow it; 4 is sitting, 9
/// sleeping). So someone seated gets up once their package's conditions
/// change without a script's `EvaluatePackage` (Doc at the new game's
/// help-up: his standing package needs `GetStage VCG01 >= 40`). Entry and
/// exit states are left to finish first. Whether the clock was looked at
/// (the caller doesn't count it again this frame).
fn rethink_queued_package_before_furniture(
    ctx: &mut Ctx,
    walker: &mut Walker,
    life: &mut Life,
    game_hour: f32,
    ask: &mut Asking,
) -> bool {
    let me = walker.reference;
    let settled = ctx
        .state
        .sitters
        .get(&me)
        .is_some_and(|sitter| sitter.state.is_settled());
    // Getting up asked for (the exit's state 5 is about to start): no more
    // checks, as on the path for anyone else.
    if !settled || life.getting_up {
        return false;
    }
    let forced = take_forced_package_evaluation(walker, ctx.state, me);
    let due = walker
        .clock
        .due(ctx.dt, game_hour, forced, walker.package.is_some());
    if due {
        rethink(ctx, walker, life, ask);
    }
    true
}

/// A walk to a reference that moves (whom they follow, a package's
/// reference): a new path when it has moved more than
/// `fAIMoveDistanceToRecalcFollowPath` (300) since the path was made, or
/// when they stand idle farther than the radius from it (`009e0a00` and
/// the follow procedure, as `findings\ai_rules.md` §3 reads them).
fn follow_target(ctx: &mut Ctx, walker: &mut Walker, moves: &Moves, ask: &mut Asking) {
    let order = &ctx.game.order;
    let me = walker.reference;
    // Dialogue, sandbox and wander packages keep to their place their own
    // way.
    if walker.door.is_some()
        || walker.package_kind == Some(world::ai::kinds::DIALOGUE)
        || walker.package_kind == Some(world::ai::kinds::SANDBOX)
        || walker.package_kind == Some(world::ai::kinds::WANDER)
        || walker.package_kind == Some(world::ai::kinds::GUARD)
        || walker.package_kind == Some(world::ai::kinds::FLEE)
        || ctx.state.furniture.contains_key(&me)
    {
        return;
    }
    let Some(t) = walker.target_ref else {
        return;
    };
    // Waiting for a path already.
    if walker.pending.is_some() {
        return;
    }
    let Some((there, _, at, _)) = ctx.state.place(order, t) else {
        return;
    };
    if ctx.state.place(order, me).map(|p| p.0) != Some(there) {
        return;
    }
    let moved_far = walker
        .path_target
        .is_some_and(|p| distance(p, at) > moves.recalc_follow);
    let idle_far = !walker.on_path() && !mv::arrived(walker.position, at, walker.radius);
    if !(moved_far || idle_far) {
        return;
    }
    // Look again at where the package sends them (its radius).
    let Some(package) = walker
        .package
        .and_then(|p| world::ai::Package::load(order, p))
        .or_else(|| current_package(order, ctx.state, me))
    else {
        return;
    };
    let Some((to, radius)) = destination(order, ctx.state, me, &package) else {
        return;
    };
    if mv::arrived(walker.position, to, radius) {
        return;
    }
    walker.path_target = Some(at);
    // Beyond the attached navmesh: the long way ([`long_walk`]).
    if beyond_attached(ctx.mesh, to) {
        return;
    }
    let then = Pending {
        id: 0,
        radius,
        turn_first: !walker.on_path(),
        arrival: world::ai::arrival_heading(order, ctx.state, me, &package),
        path_target: Some(at),
        partial: false,
        note: "after whom or what they go to".into(),
    };
    let shared = ask.mesh.clone();
    ask_path(ask.queue, &shared, walker, to, then);
}

/// Someone idle whose package's place (or the load door toward it) isn't
/// on the attached cells' navmesh: the game's path request plans the long
/// way over the navmesh info map (`world::ai::navinfo`, `006c94c0`) and
/// builds the detailed path only for the run of its nodes whose cells are
/// attached (`006c9fc0`, `006ca0e0`), to the last such node (resolved onto
/// the navmesh). They walk that and stand there (`partial`); planned again
/// when the attached squares or the place change, as the travel procedure
/// asks again whenever it's idle short of its place (`008e5e90`).
fn long_walk(
    ctx: &mut Ctx,
    walker: &mut Walker,
    infos: &mut world::ai::navinfo::NavInfos,
    (space, squares): (FormId, &[(i32, i32)]),
    attached: &HashSet<(i32, i32)>,
    recalc_follow: f32,
    ask: &mut Asking,
) {
    use world::ai::kinds;
    let order = &ctx.game.order;
    let me = walker.reference;
    if walker.on_path()
        || walker.pending.is_some()
        || matches!(
            walker.package_kind,
            Some(kinds::DIALOGUE | kinds::SANDBOX | kinds::WANDER | kinds::GUARD | kinds::FLEE)
        )
        || ctx.state.furniture.contains_key(&me)
        || ctx.state.sitters.contains_key(&me)
    {
        return;
    }
    if ctx.state.place(order, me).map(|p| p.0) != Some(space) {
        return;
    }
    let package = walker
        .package
        .and_then(|p| world::ai::Package::load(order, p));
    let (to, radius) = match walker.door {
        Some(d) => (d.at, 0.0),
        None => match package
            .as_ref()
            .and_then(|p| destination(order, ctx.state, me, p))
        {
            Some(d) => d,
            None => return,
        },
    };
    if mv::arrived(walker.position, to, radius) {
        return;
    }
    // Planned already for these squares and (within
    // `fAIMoveDistanceToRecalcFollowPath`, for a target that moves) this
    // place.
    if walker
        .long
        .as_ref()
        .is_some_and(|(s, g)| s == squares && distance(*g, to) <= recalc_follow)
    {
        return;
    }
    walker.long = Some((squares.to_vec(), to));
    let Some(nodes) = infos.virtual_path(order, space, walker.position, to) else {
        println!("{me} has no way to {:.0},{:.0},{:.0}", to[0], to[1], to[2]);
        return;
    };
    let in_attached =
        |n: &world::ai::navinfo::VirtualNode| n.square.is_some_and(|s| attached.contains(&s));
    let Some(last) = world::ai::navinfo::attached_run(&nodes, 0, in_attached) else {
        return;
    };
    let whole = last + 1 == nodes.len();
    let end = if whole {
        // A goal off the navmesh is joined to it by the request's ray-cast
        // way (`world::ai::offmesh`).
        to
    } else {
        // The node resolved onto its own navmesh.
        let node = nodes[last];
        match ctx.mesh.closest_point_on(node.position, Some(node.navmesh)) {
            Some(p) => p,
            None => return,
        }
    };
    // Standing off the navmesh (out of sight they walk straight from node
    // to node, `009ea8a0`, and a navmesh's rough position can lie in a
    // hole of it): the request's ray-cast way onto the navmesh
    // (`world::ai::offmesh`) takes them there.
    let arrival = if whole && walker.door.is_none() {
        package
            .as_ref()
            .and_then(|p| world::ai::arrival_heading(order, ctx.state, me, p))
    } else {
        None
    };
    let then = Pending {
        id: 0,
        radius,
        turn_first: true,
        arrival,
        path_target: None,
        partial: !whole,
        note: format!(
            "of the long way: {} nodes, {} attached{}",
            nodes.len(),
            last + 1,
            if whole { ", to the end" } else { "" }
        ),
    };
    let shared = ask.mesh.clone();
    ask_path(ask.queue, &shared, walker, end, then);
}

/// The package's procedures reached `DONE`: its end action, once per start
/// (`0091ecf0`, process flag +0x5a8).
fn package_done(state: &mut world::scripting::GameState, walker: &mut Walker) {
    if walker.ended {
        return;
    }
    walker.ended = true;
    if let Some(p) = walker.package.filter(|p| p.0 != 0) {
        world::ai::actions::end(state, walker.reference, p);
    }
}

/// The path's length still ahead of someone on it.
fn path_left(walker: &Walker) -> f32 {
    if !walker.on_path() {
        return 0.0;
    }
    let mut left = distance(walker.position, walker.path[walker.next]);
    for w in walker.path[walker.next..].windows(2) {
        left += distance(w[0], w[1]);
    }
    left
}

/// One frame of a guard package (`world::ai::guard`, `00902290`): a
/// seated guard gets up; one away from its post with its mover idle goes
/// there (through a door when its editor location is elsewhere), running
/// or walking by the path left (`008daa20`); at its post it turns to an
/// `XMarkerHeading` location's heading. Its wandering at a post with a
/// radius and its intruder watch aren't carried out (no Goodsprings guard
/// package has either).
fn guard_frame(ctx: &mut Ctx, walker: &mut Walker, life: &mut Life) {
    use world::ai::guard::{self, Post};
    let order = &ctx.game.order;
    let me = walker.reference;
    let Some(package) = walker
        .package
        .and_then(|p| world::ai::Package::load(order, p))
    else {
        return;
    };
    // Seated or asleep: up first (`00902290` → actor vfunc +0x418,
    // `InitiateGetUpPackage` (Xbox PDB)).
    if let Some(sitter) = ctx.state.sitters.get_mut(&me) {
        if sitter.state != SitState::Normal {
            if sitter.state.is_settled() {
                sitter.stand_up();
                life.getting_up = true;
            }
            return;
        }
    }
    let Some(plan) = guard::plan(order, me, &package) else {
        return;
    };
    let here = ctx.state.place(order, me).map(|p| p.0);
    // The post, in the place they're in; else the door toward it.
    let (post, elsewhere) = match plan.post {
        Post::Editor { position } => {
            let editor =
                world::scripting::whereabouts(order, me).map(|w| w.world.unwrap_or(w.cell));
            (position, editor.filter(|s| Some(*s) != here))
        }
        Post::Reference(r) => match ctx.state.place(order, r) {
            Some((space, _, at, _)) => (at, Some(space).filter(|s| Some(*s) != here)),
            None => return,
        },
    };
    let at = match elsewhere {
        Some(_) => false,
        None if plan.has_location => destination(order, ctx.state, me, &package)
            .is_none_or(|(to, r)| mv::arrived(walker.position, to, r)),
        None => {
            let d = (0..3)
                .map(|i| (walker.position[i] - post[i]).powi(2))
                .sum::<f32>()
                .sqrt();
            d < plan.radius
        }
    };
    if at {
        if walker.on_path() && walker.door.is_none() {
            walker.clear_path();
        }
        walker.run = false;
        if !walker.on_path() && walker.facing.is_none() {
            if let Some(h) = guard::facing(order, ctx.state, &package) {
                let off = (h - walker.heading + std::f32::consts::PI)
                    .rem_euclid(std::f32::consts::TAU)
                    - std::f32::consts::PI;
                if off.abs() > mv::ONE_DEGREE {
                    walker.facing = Some(h);
                }
            }
        }
        return;
    }
    if !walker.on_path() {
        if let Some(space) = elsewhere {
            let Some(way) = world::ai::door_toward(order, ctx.state, me, space) else {
                return;
            };
            if let Some(path) = path_for(ctx.mesh, walker, way.at) {
                walker.set_path(path, mv::REQUEST_RADIUS, true, ctx.moves);
                walker.door = Some(way);
            }
        } else if let Some(path) = path_for(ctx.mesh, walker, post) {
            walker.set_path(path, plan.path_radius, true, ctx.moves);
            walker.door = None;
        }
    }
    walker.run = guard::runs(
        path_left(walker),
        plan.radius,
        walker.run,
        package.flags,
        false,
    );
}

/// One frame of a flee package (`world::ai::flee`, `008ddac0`). With no
/// one to flee from and nowhere to flee to the procedure is over at once
/// and they stand; with a place, they run there until within its radius.
/// The engine's flee package from a target without a place (`00897de0`'s
/// path search, `009f1140`) isn't traced: they stand.
fn flee_frame(ctx: &mut Ctx, walker: &mut Walker) {
    use world::ai::flee::{self, FleeStep};
    let order = &ctx.game.order;
    let me = walker.reference;
    let Some(package) = walker
        .package
        .and_then(|p| world::ai::Package::load(order, p))
    else {
        return;
    };
    let here = ctx.state.place(order, me).map(|p| p.0);
    let near = |r: FormId| {
        ctx.state
            .place(order, r)
            .filter(|p| Some(p.0) == here)
            .map(|p| (r, p.2, distance(walker.position, p.2)))
    };
    let from = flee::flee_from(order, ctx.state, me, &package).and_then(near);
    let to = flee::flee_to(order, me, &package).and_then(near);
    let radius = package
        .location
        .map_or(0.0, |l| world::ai::location_radius_of(order, &l) as f32);
    let step = flee::step(
        from.map(|f| (f.0, f.2)),
        to.map(|t| (t.0, t.2)),
        package.target.map(|t| t.2),
        radius,
        package.flags,
        walker.on_path(),
    );
    match step {
        FleeStep::Done => {
            walker.clear_path();
            walker.run = false;
            package_done(ctx.state, walker);
        }
        FleeStep::Safe { stop, finish } => {
            if stop {
                walker.clear_path();
                walker.run = false;
            }
            if finish {
                package_done(ctx.state, walker);
            }
        }
        FleeStep::Run { to: Some(_), .. } => {
            if !walker.on_path() {
                if let Some((_, at, _)) = to {
                    if let Some(path) = path_for(ctx.mesh, walker, at) {
                        walker.set_path(path, radius.max(mv::REQUEST_RADIUS), true, ctx.moves);
                        walker.run = true;
                    }
                }
            }
        }
        FleeStep::Run { to: None, .. } => {}
    }
}

/// One frame of `ForceFlee`'s engine flee package (`world::ai::flee::
/// force`, `00897de0`): to the reference given when it's in their space,
/// running until within the request radius; with none they stand (the
/// flee from nobody, `009f1140` with no one to avoid, moves them nowhere
/// that was traced). The package's own avoiding and door search
/// (`FleePackage`) are the AI's, not done here.
fn forced_flee_frame(ctx: &mut Ctx, walker: &mut Walker, flee: world::ai::flee::ForcedFlee) {
    let order = &ctx.game.order;
    let here = ctx.state.place(order, walker.reference).map(|p| p.0);
    let to = flee
        .to
        .and_then(|r| ctx.state.place(order, r))
        .filter(|p| Some(p.0) == here)
        .map(|p| p.2);
    match to {
        Some(at) if !mv::arrived(walker.position, at, mv::REQUEST_RADIUS) => {
            if !walker.on_path() {
                if let Some(path) = path_for(ctx.mesh, walker, at) {
                    walker.set_path(path, mv::REQUEST_RADIUS, true, ctx.moves);
                }
            }
            walker.run = true;
        }
        _ => {
            if walker.on_path() {
                walker.clear_path();
            }
            walker.run = false;
        }
    }
}

/// One frame of a dialogue package (`world::ai::dialogue_step`): travel
/// first, then wait for the target at its second location, walk up to it,
/// and say the line ("Say To") or talk: the dialogue menu with the player,
/// a conversation with anyone else.
fn dialogue_frame(
    ctx: &mut Ctx,
    walker: &mut Walker,
    (chats, lines, talk): (&mut Chats, &mut Lines, &mut crate::scripts::ScriptedTalk),
    moves: &Moves,
) {
    let order = &ctx.game.order;
    let me = walker.reference;
    let Some(package) = walker
        .package
        .and_then(|p| world::ai::Package::load(order, p))
    else {
        return;
    };
    // The travel step, until it's over (then not again: the procedures
    // only go on, `008e8600`'s list).
    let at_place = walker.talk_run.travelled
        || match destination(order, ctx.state, me, &package) {
            Some((to, radius)) => mv::arrived(walker.position, to, radius),
            None => true,
        };
    walker.talk_run.travelled = at_place;
    if !at_place {
        // The travel: walking there (set up by `rethink`).
        if !walker.on_path() {
            if let Some((to, radius)) = destination(order, ctx.state, me, &package) {
                if let Some(path) = path_for(ctx.mesh, walker, to) {
                    walker.set_path(path, radius, true, ctx.moves);
                    walker.arrival = world::ai::arrival_heading(order, ctx.state, me, &package);
                }
            }
        }
        return;
    }
    let my_radius = walker
        .kit
        .as_ref()
        .map_or(world::combat_ai::PERSON_RADIUS, |k| k.radius);
    let Some(step) = world::ai::dialogue_step(order, ctx.state, me, &package, true, my_radius)
    else {
        return;
    };
    let Some((_, target, _)) = package.target else {
        return;
    };
    let data = world::ai::dialogue_data(order, package.form_id);
    let topic = data.and_then(|d| d.topic);
    match step {
        DialogueStep::Travel | DialogueStep::Wait => {
            if walker.on_path() && walker.door.is_none() && at_place {
                walker.clear_path();
            }
        }
        DialogueStep::Approach { reach } => {
            let Some((_, _, at, _)) = ctx.state.place(order, target) else {
                return;
            };
            let repath = !walker.on_path()
                || walker
                    .path_target
                    .is_some_and(|p| distance(p, at) > moves.recalc_follow);
            if repath {
                if let Some(path) = path_for(ctx.mesh, walker, at) {
                    walker.path_target = Some(at);
                    let turn_first = !walker.on_path();
                    walker.set_path(path, reach, turn_first, ctx.moves);
                }
            }
        }
        DialogueStep::Say => {
            walker.clear_path();
            if lines.is_saying(me) {
                return;
            }
            // The topic's line (else `HELLO`) through the GREET procedure: a
            // line said to the target, no dialogue menu; done after it
            // (`008dbe30` ends a "Say To" dialogue package, step 3).
            let topic = topic.unwrap_or(world::social::topics::HELLO);
            if let Some(info) = pick_line(order, ctx.state, me, target, topic) {
                lines.say_to(me, target, info);
                walker.head_track.set(Slot::Action, Some(target));
            }
            if let Some(s) = walker.social.as_mut() {
                s.greeted(&moves.social);
            }
            walker.talk_run.said();
        }
        DialogueStep::Talk => {
            walker.clear_path();
            // `InitiateDialogue` makes the conversation package on one
            // update; its ACTIVATE activates on the next (`world::ai::talk`).
            if walker.talk_run.talk() == world::ai::talk::TalkUpdate::Initiate {
                return;
            }
            if target == PLAYER_REF {
                // The player is activated: the dialogue menu, about the
                // package's topic.
                if talk.0.is_some() {
                    // Another's talk opens this frame: try again.
                    walker.talk_run.retry();
                    return;
                }
                println!("{:.1} s: {me} starts talking to the player.", ctx.now);
                talk.0 = Some((me, topic, true, false));
            } else {
                start_chat(ctx, walker, chats, target, topic, false);
            }
            // The one activated finishes the dialogue package (`005fa330`:
            // saved at DONE, `PackageDone`): its end action, and no more
            // talk after the conversation.
            package_done(ctx.state, walker);
        }
    }
}

/// The first line `who` can say on a topic to `listener`.
fn pick_line(
    order: &esm::LoadOrder,
    state: &world::scripting::GameState,
    who: FormId,
    listener: FormId,
    topic: FormId,
) -> Option<world::dialogue::Info> {
    let speaker = Speaker::load(order, who, world::scripting::base_of(order, who)?)?;
    world::social::pick_for(order, topic, &speaker, listener, state, &[])
}

/// Starts a conversation between `walker` and `other` (`008b2170`): its
/// lines worked out now (none: no conversation); the starter walks up
/// within 90 (200 when seated), the other waits.
pub(crate) fn start_chat(
    ctx: &mut Ctx,
    walker: &mut Walker,
    chats: &mut Chats,
    other: FormId,
    topic: Option<FormId>,
    seated: bool,
) -> bool {
    let order = &ctx.game.order;
    let me = walker.reference;
    if chats.0.contains_key(&other) || chats.0.contains_key(&me) {
        return false;
    }
    let (Some(a), Some(b)) = (
        world::scripting::base_of(order, me).and_then(|base| Speaker::load(order, me, base)),
        world::scripting::base_of(order, other).and_then(|base| Speaker::load(order, other, base)),
    ) else {
        return false;
    };
    let mut dice = crate::fighting::Dice::new(ctx.state);
    let conversation = world::social::conversation(order, ctx.state, &a, &b, topic, &mut || {
        u64::from(dice.roll())
    });
    if conversation.is_empty() {
        return false;
    }
    println!(
        "{:.1} s: {me} starts a conversation with {other} ({} lines).",
        ctx.now,
        conversation.len()
    );
    let reach = mv::conversation_reach(seated);
    // Scripts start some with the speaker themselves (a performance, the
    // Tops' `TopsPerformerActivatorSCRIPT`): one side only.
    if other != me {
        chats.0.insert(
            other,
            Chat {
                with: me,
                starter: false,
                reach,
                lines: Vec::new(),
                next: 0,
                last_line: f32::NEG_INFINITY,
                talking: false,
            },
        );
    }
    chats.0.insert(
        me,
        Chat {
            with: other,
            starter: true,
            reach,
            lines: conversation,
            next: 0,
            last_line: f32::NEG_INFINITY,
            talking: false,
        },
    );
    true
}

/// A script's `StartConversation` begins (`008b2170`): with the player, the
/// walk up ([`talk_frame`]); with anyone else, a conversation
/// ([`start_chat`], its lines from the topic given).
fn begin_conversation(ctx: &mut Ctx, walker: &mut Walker, chats: &mut Chats) {
    let Some((to, topic)) = walker.start.take() else {
        return;
    };
    let seated = ctx.state.sitters.contains_key(&walker.reference);
    if to == PLAYER_REF {
        walker.talk_to = Some((topic, mv::conversation_reach(seated)));
    } else if !start_chat(ctx, walker, chats, to, topic, seated) {
        println!(
            "{} can't start a conversation with {to} (nothing to say).",
            walker.reference
        );
    }
}

/// Coming up to the player for a script's `StartConversation` (the
/// conversation package's activate step, `008e9640`): within the reach
/// (90, 200 seated), measured as `IsWithinDistance` adding their radius (at
/// least 32), the player is activated: the dialogue menu. Farther, they walk
/// up; seated, they don't (they wait for the player to come).
fn talk_frame(
    ctx: &mut Ctx,
    walker: &mut Walker,
    talk: &mut crate::scripts::ScriptedTalk,
    moves: &Moves,
) {
    let me = walker.reference;
    let Some((topic, reach)) = walker.talk_to else {
        return;
    };
    let Some(at) = ctx.state.player_position else {
        return;
    };
    let my_radius = walker
        .kit
        .as_ref()
        .map_or(world::combat_ai::PERSON_RADIUS, |k| k.radius);
    if mv::within_distance(walker.position, 128.0, Some(my_radius), at, reach, true) {
        if talk.0.is_none() {
            walker.clear_path();
            walker.talk_to = None;
            println!("{:.1} s: {me} starts talking to the player.", ctx.now);
            talk.0 = Some((me, topic, true, false));
        }
        return;
    }
    let seated = ctx.state.sitters.contains_key(&me) || ctx.state.furniture.contains_key(&me);
    if seated {
        return;
    }
    let repath = !walker.on_path()
        || walker
            .path_target
            .is_some_and(|p| distance(p, at) > moves.recalc_follow);
    if repath {
        if let Some(path) = path_for(ctx.mesh, walker, at) {
            walker.path_target = Some(at);
            let turn_first = !walker.on_path();
            walker.set_path(path, reach, turn_first, ctx.moves);
        }
    }
}

/// A conversation is over for both.
fn end_chat(chats: &mut Chats, who: FormId) {
    if let Some(c) = chats.0.remove(&who) {
        chats.0.remove(&c.with);
    }
}

/// The least time between two lines of a conversation (the playback's
/// pause timer, `009ee0a0` sets 2 s).
const LINE_PAUSE: f32 = 2.0;

/// One frame of a conversation between two people. The starter walks up
/// within reach (as `IsWithinDistance`, adding their radius), then says
/// the lines in turn (each by its speaker, through `chatter`); the other
/// turns in place to face the starter every frame (unless seated); the
/// starter turns to face the other only when the other faces more than
/// `iActorTurnDegree` (100°) away from them and it isn't turning already
/// (`008ec460`, as written there). Over: both look at their packages again
/// (`008ec460` zeroes the timer).
fn chat_frame(
    ctx: &mut Ctx,
    walker: &mut Walker,
    chats: &mut Chats,
    lines: &mut Lines,
    moves: &Moves,
) {
    let order = &ctx.game.order;
    let me = walker.reference;
    let Some(chat) = chats.0.get(&me).cloned() else {
        return;
    };
    let other = chat.with;
    let alone = other == me;
    let gone = ctx.state.dead.contains(&other)
        || (!alone && ctx.state.combat.contains_key(&other))
        || ctx.state.place(order, other).map(|p| p.0) != ctx.state.place(order, me).map(|p| p.0);
    let Some((_, _, at, their_heading)) = ctx.state.place(order, other).filter(|_| !gone) else {
        end_chat(chats, me);
        // The DIALOGUE procedure's `EndDialogue` (`008ec460` → `008b1070` →
        // `00913250`): a dialogue package is back at its saved step.
        walker.talk_run.conversation_over();
        walker.evaluate = true;
        return;
    };
    let seated = ctx.state.sitters.contains_key(&me);
    let toward = mv::heading_to(walker.position, at);
    if !chat.starter {
        // Waiting for them, facing them.
        walker.clear_path();
        if !seated && (at[0] - walker.position[0]).hypot(at[1] - walker.position[1]) > 1.0 {
            face(walker, toward, ctx.dt, false, ctx.moves);
        }
        walker.head_track.set(Slot::Dialog, Some(other));
        return;
    }
    let my_radius = walker
        .kit
        .as_ref()
        .map_or(world::combat_ai::PERSON_RADIUS, |k| k.radius);
    let in_reach = alone
        || mv::within_distance(
            walker.position,
            128.0,
            Some(my_radius),
            at,
            chat.reach,
            true,
        );
    if chat.next == 0 && !chat.talking && !in_reach {
        if seated {
            // Seated, they don't walk: too far, no conversation.
            end_chat(chats, me);
            return;
        }
        let repath = !walker.on_path()
            || walker
                .path_target
                .is_some_and(|p| distance(p, at) > moves.recalc_follow);
        if repath {
            match path_for(ctx.mesh, walker, at) {
                Some(path) => {
                    walker.path_target = Some(at);
                    let turn_first = !walker.on_path();
                    walker.set_path(path, chat.reach, turn_first, ctx.moves);
                }
                None => {
                    end_chat(chats, me);
                }
            }
        }
        return;
    }
    walker.clear_path();
    if !alone {
        walker.head_track.set(Slot::Dialog, Some(other));
    }
    // The starter's turn: only when the other faces well away.
    let their_off = mv::wrap_pi(mv::heading_to(at, walker.position) - their_heading).abs();
    if !seated
        && !alone
        && ((walker.turn.active
            && mv::wrap_pi(walker.turn.target - toward).abs() <= mv::ONE_DEGREE)
            || (their_off > moves.settings.turn_degree * mv::ONE_DEGREE && !walker.turn.active))
    {
        face(walker, toward, ctx.dt, false, ctx.moves);
    }
    // The lines.
    let saying = chat.lines.iter().any(|l| lines.is_saying(l.speaker));
    let mut chat = chat;
    if saying || ctx.now - chat.last_line < LINE_PAUSE {
        chats.0.insert(me, chat);
        return;
    }
    if chat.next >= chat.lines.len() {
        println!(
            "{:.1} s: {me}'s conversation with {other} is over.",
            ctx.now
        );
        end_chat(chats, me);
        // The DIALOGUE procedure's `EndDialogue` (`008ec460` → `008b1070` →
        // `00913250`): a dialogue package is back at its saved step.
        walker.talk_run.conversation_over();
        walker.evaluate = true;
        return;
    }
    let line = chat.lines[chat.next].clone();
    // Its says force a tree request unless the starter's package has idles
    // of its own (`009edd80`'s last argument, `world::talk_idles`).
    let force = world::talk_idles::conversation_forces_tree(
        walker
            .package
            .map(|p| world::talk_idles::package_has_idles(order, p)),
    );
    lines.say_in_conversation(line.speaker, line.listener, line.info, force);
    chat.next += 1;
    chat.last_line = ctx.now;
    chat.talking = true;
    chats.0.insert(me, chat);
}

/// What greetings this frame share (`world::social`): the place, the
/// player's state as the player's update leaves it, the player's greeting
/// cooldown (`PlayerCharacter` +0xe24) with the real clock it counts on
/// (`GetTickCount`), and who says a line to the player without the menu
/// (an activated person's greeting or a script's `SayTo`, both the GREET
/// procedure).
pub(crate) struct Greeter<'a> {
    pub interior: bool,
    pub cooldown: &'a mut world::social::HelloCooldown,
    pub now_ms: u32,
    pub line_speaker: Option<FormId>,
    pub player_trespassing: bool,
    pub player_in_combat: bool,
}

/// The player's greeting cooldown (`world::social::HelloCooldown`).
#[derive(Resource, Default)]
pub struct PlayerHello(pub world::social::HelloCooldown);

/// Greeting the player, idle chatter, and starting conversations with
/// others (`008eeec0`, `00904800`; `world::social`), for someone not in a
/// conversation with another.
fn social_frame(
    ctx: &mut Ctx,
    walker: &mut Walker,
    chats: &mut Chats,
    lines: &mut Lines,
    moves: &Moves,
    others: &[crate::fighting::Seen],
    greeter: &mut Greeter,
) {
    let order = &ctx.game.order;
    let me = walker.reference;
    let interior = greeter.interior;
    // Saying a GREET line to someone holds their greeting timer at
    // `fAIGreetingTimer` (`008dbe30` sets process +0x330 on each update
    // while the line lasts): it runs out that long after the line.
    let line_to_player = greeter.line_speaker == Some(me);
    if lines.greeting(me) || line_to_player {
        if let Some(s) = walker.social.as_mut() {
            s.greeted(&moves.social);
        }
    }
    if chats.0.contains_key(&me) || world::combat::is_creature(order, me) {
        return;
    }
    let Some(mut social) = walker.social.take() else {
        return;
    };
    let package_kind = walker.package_kind;
    let player_distance = ctx
        .state
        .player_position
        .map(|p| distance(p, walker.position));
    let detected = walker.detected_player;
    // The greeting that held the player in their ACTION slot is over once
    // the head-track update emptied it.
    if walker.head_track.in_slot(Slot::Action) != Some(PLAYER_REF) {
        walker.greet_look = false;
    }
    // Their target (`008815a0` → process +0x40): their package's own
    // reference target as it started (`0090a1a0`, `006780e0`), or the
    // player a script's `StartConversation` sent them to.
    let target_is_player = walker.talk_to.is_some()
        || walker
            .package
            .and_then(|p| world::ai::Package::load(order, p))
            .and_then(|p| p.target)
            .is_some_and(|(kind, who, _)| kind == 0 && who == PLAYER_REF);
    let check = world::social::GreetingCheck {
        // The player's AI conversation (+0x224): the viewer has none; a
        // conversation with the player opens the dialogue menu, and the
        // world stands still while it is open.
        important_conversation: false,
        in_combat: ctx.state.combat.contains_key(&me),
        fleeing: walker.fleeing.is_some() || package_kind == Some(world::ai::kinds::FLEE),
        unconscious: ctx.state.unconscious.contains(&me),
        knocked: walker.fallen || ctx.state.more.down.contains_key(&me),
        // A line of their own being said (its voice), and the GREET flag.
        speaking: lines.is_saying(me) || line_to_player,
        greeting_line: lines.greeting(me) || line_to_player,
        player_spoken_to: greeter.line_speaker.is_some() || lines.spoken_to(PLAYER_REF),
        detection: detected,
        asleep: crate::sitting::sit_state(ctx.state, me) == 9,
        player_sneaking: ctx.state.player_sneaking,
        player_trespassing: greeter.player_trespassing,
        target_is_player,
        // Run-once packages (process +0xe4) aren't carried out here.
        running_run_once: false,
        head_track_asked: !walker.head_track.action_free(),
        // Packages going on for the player (general flag 0x200, `0090a1a0`)
        // aren't carried out here.
        continuing_for_player: false,
        distance: player_distance.unwrap_or(f32::MAX),
        player_in_combat: greeter.player_in_combat,
        hellos_forbidden: walker
            .package
            .is_some_and(|p| world::social::package_forbids_hellos(order, p)),
        alarm_package: package_kind == Some(world::ai::kinds::ALARM),
        still_in_made_dialogue: walker.talk_run.conversation_package_made()
            && !walker.walking_now(),
    };
    // Busy: neither a greeting nor chatter (`008ef6cc`), but the
    // conversation check below still runs.
    let greeting = social.greeting(&check, &moves.social);
    // A greeting (`008bc3d0`): the player goes in their ACTION head-track
    // slot (`+0x628`) whatever comes of it; only when nobody has greeted
    // the player for `fHelloCooldownTime`, `HELLO` said to the player
    // through the GREET procedure, no dialogue menu (none found for them:
    // nothing said, but their timer and the cooldown are set all the same).
    // A greeting not begun leaves the slot set ([`Walker::greet_look`]), so
    // they aren't near again (`CanSetActionHeadTrackTarget`) till the
    // player is lost from their sight.
    if greeting == world::social::Greeting::Greet {
        walker.head_track.set(Slot::Action, Some(PLAYER_REF));
        walker.greet_look = true;
    }
    if greeting == world::social::Greeting::Greet && greeter.cooldown.free() {
        social.greeted(&moves.social);
        greeter.cooldown.greeted(greeter.now_ms);
        let line = pick_line(
            order,
            ctx.state,
            me,
            PLAYER_REF,
            world::social::topics::HELLO,
        );
        println!(
            "{:.1} s: {me} greets the player{}.",
            ctx.now,
            if line.is_some() {
                ""
            } else {
                " (no HELLO line for them)"
            }
        );
        if let Some(info) = line {
            lines.say_to(me, PLAYER_REF, info);
            // Their line's end lets the slot go (`008dbe30`, `+0x644(1)`).
            walker.greet_look = false;
            // They turn to the player standing, unless their package is
            // one that keeps them busy (GREET, `008dbe30`).
            let busy = matches!(
                package_kind,
                Some(k) if [
                    world::ai::kinds::TRAVEL,
                    world::ai::kinds::ESCORT,
                    world::ai::kinds::FOLLOW,
                    world::ai::kinds::ACCOMPANY,
                    world::ai::kinds::PATROL,
                    world::ai::kinds::SANDBOX,
                    8,
                    16,
                    0,
                ]
                .contains(&k)
            );
            if !busy && !walker.on_path() && !ctx.state.sitters.contains_key(&me) {
                if let Some(p) = ctx.state.player_position {
                    walker.turn.request(
                        walker.heading,
                        mv::heading_to(walker.position, p),
                        ctx.moves,
                    );
                }
            }
        }
    } else if greeting == world::social::Greeting::Away {
        // Idle chatter (`008ef8ee`): their package must allow it.
        let package = walker.package.map(|p| {
            (
                package_kind.unwrap_or(u8::MAX),
                world::social::package_allows_chatter(order, p),
            )
        });
        let made = walker.talk_run.conversation_package_made();
        let mut dice = crate::fighting::Dice::new(ctx.state);
        if social.chatter_due(ctx.dt, package, made, &moves.social, &mut || dice.unit()) {
            if let Some(info) = pick_line(
                order,
                ctx.state,
                me,
                PLAYER_REF,
                world::social::topics::IDLE_CHATTER,
            ) {
                lines.say(me, PLAYER_REF, info);
            }
        }
    }
    // Conversations with others (`00904800`): only with a package, not one
    // the game makes (`00678610`) nor one that forbids them (`008a78f0(1)`,
    // `world::social::package_forbids_conversations`); not while asleep or
    // in sleep, use item at, ambush, guard, dialogue or use weapon
    // packages; a sandbox only if it allows them; follow, escort and
    // accompany only with their target. Not looked for while someone talks
    // in the player's AI conversation, their GREET flag is up or someone
    // says a line to the player (`008eeec0` at `008efa4e`), nor (as before,
    // inferred) while they say a line.
    if check.important_conversation
        || check.greeting_line
        || check.player_spoken_to
        || lines.is_saying(me)
    {
        walker.social = Some(social);
        return;
    }
    let allowed = walker.package.is_some_and(|p| {
        !package_kind.is_some_and(world::social::made_package_kind)
            && !world::social::package_forbids_conversations(order, p)
    }) && match package_kind {
        Some(4) | Some(8) | Some(9) | Some(14) | Some(15) | Some(16) => false,
        Some(12) => walker.package.is_some_and(|p| {
            world::sandbox::package_flags(order, p) & world::sandbox::flags::NO_CONVERSATION == 0
        }),
        _ => true,
    };
    let only = match package_kind {
        Some(1) | Some(2) | Some(7) => walker
            .package
            .and_then(|p| world::ai::Package::load(order, p))
            .and_then(|p| world::ai::followed(&p).map(|f| f.0)),
        _ => None,
    };
    let candidates: Vec<(FormId, f32)> = others
        .iter()
        .filter(|o| o.reference != me && o.reference != PLAYER_REF)
        .filter(|o| {
            !chats.0.contains_key(&o.reference) && !ctx.state.combat.contains_key(&o.reference)
        })
        .filter(|o| !world::combat::is_creature(order, o.reference))
        .filter(|o| {
            // Someone sandboxing talks only if their sandbox allows it.
            current_package(order, ctx.state, o.reference).is_none_or(|p| {
                p.kind != world::ai::kinds::SANDBOX
                    || world::sandbox::package_flags(order, p.form_id)
                        & world::sandbox::flags::NO_CONVERSATION
                        == 0
            })
        })
        .map(|o| (o.reference, distance(o.position, walker.position)))
        .collect();
    let mut dice = crate::fighting::Dice::new(ctx.state);
    let mut dice2 = crate::fighting::Dice::new(ctx.state);
    let started = social.start_conversation(
        me,
        allowed,
        only,
        &candidates,
        interior,
        ctx.dt,
        &moves.social,
        &mut || u64::from(dice.roll()),
        &mut || dice2.unit(),
    );
    walker.social = Some(social);
    if let Some(other) = started {
        let seated = ctx.state.sitters.contains_key(&me);
        start_chat(ctx, walker, chats, other, None, seated);
    }
}

/// Holds or lets go of the head-track slots others asked for, where the
/// viewer stands in for the game's setters (`docs/HEAD_TRACK_TARGET.md`;
/// ported from the contributor branch `playcon/claude/head-track-target`,
/// 9defa47):
///
/// * ACTION (a line said to someone: the "Say To" package, a greeting, and
///   a script's `SayTo` to the player, `005c9100`) holds while they say
///   it, then is cleared with demote, as the "Say To" package does when it
///   ends (`008dbe30`). That it ends with the line is inferred.
/// * DIALOG (a conversation: with someone, `00935480`; the dialogue menu
///   with the player) holds while it lasts, then is cleared with demote
///   (`00933d20`).
///
/// The speaker in a conversation with the player has the player put in
/// the slot here; the other sites put their target in when they start.
fn head_track_asks(
    walker: &mut Walker,
    speaker: Option<FormId>,
    in_menu: bool,
    lines: &Lines,
    chats: &Chats,
    settings: &world::head_track::Settings,
) {
    let me = walker.reference;
    walker.head_tracking_off = settings.disabled;
    let with_player = speaker == Some(me);
    if with_player {
        let slot = if in_menu { Slot::Dialog } else { Slot::Action };
        walker.head_track.set(slot, Some(PLAYER_REF));
    }
    let saying = lines.is_saying(me) || (with_player && !in_menu);
    if walker.head_track.in_slot(Slot::Action).is_some() && !saying && !walker.greet_look {
        walker.head_track.clear(Slot::Action, true, settings);
    }
    let talking = chats.0.contains_key(&me) || (with_player && in_menu);
    if walker.head_track.in_slot(Slot::Dialog).is_some() && !talking {
        walker.head_track.clear(Slot::Dialog, true, settings);
    }
}

/// One head-track update of an actor (`008a3100`, `world::head_track`):
/// none farther than `fAIMaxHeadTrackDistanceFromPC` from the player; the
/// player forgotten as a target once no longer noticed; a target no longer
/// among the people here forgotten; then the timers, and on them a choice
/// among the people here.
///
/// The candidates' facts come from the viewer: distance and angle from
/// positions, detection from its detection run ("noticed", standing for the
/// game's level >= 1, an inference), line of sight from its ray cast
/// between the two (`fighting::clear_between`). The dead aren't among the
/// people here, so the game's halving for someone down never applies.
/// Not modelled: 008a3100's package branch (process +0x27c with flag
/// 0x100000, which eases the look out) and its FaceGen-distance return.
fn choose_head_track(
    walker: &mut Walker,
    state: &mut world::scripting::GameState,
    others: &[crate::fighting::Seen],
    collision: &physics::Collider,
    settings: &world::head_track::Settings,
    chosen_this_frame: &mut u32,
    dt: f32,
) {
    let me = walker.reference;
    let Some(player) = state.player_position else {
        return;
    };
    if distance(player, walker.position) > settings.max_distance_from_player {
        return;
    }
    if walker.head_track.current() == Some(PLAYER_REF) && !walker.noticed.contains(&PLAYER_REF) {
        walker.head_track.clear_all();
    }
    if let Some(t) = walker.head_track.current() {
        if !others.iter().any(|o| o.reference == t) {
            walker.head_track.clear_all();
        }
    }
    let mut dice = crate::fighting::Dice::new(state);
    let (position, heading) = (walker.position, walker.heading);
    let noticed = &walker.noticed;
    let candidates = || -> Vec<Candidate> {
        others
            .iter()
            .filter(|o| o.reference != me)
            .map(|o| Candidate {
                reference: o.reference,
                is_player: o.reference == PLAYER_REF,
                distance: distance(position, o.position),
                off_heading: mv::wrap_pi(mv::heading_to(position, o.position) - heading),
                detected: noticed.contains(&o.reference),
                in_sight: crate::fighting::clear_between(collision, position, o.position),
                down: false,
            })
            .collect()
    };
    walker.head_track.update(
        dt,
        settings,
        chosen_this_frame,
        &mut || dice.unit(),
        |current, timer| world::head_track::choose(&candidates(), current, timer, settings),
    );
}

/// The body turns to whom they look at (`008a3100`): standing, not
/// walking, not fighting, not seated, out of dialogue and "use" packages,
/// when that one is more than 80° off (8° while turning), in place.
fn look_frame(ctx: &mut Ctx, walker: &mut Walker, fighting: bool, moves: &Moves) {
    let order = &ctx.game.order;
    let Some(who) = walker.head_track.current() else {
        return;
    };
    let blocked = fighting
        || walker.on_path()
        || ctx.state.sitters.contains_key(&walker.reference)
        || matches!(walker.package_kind, Some(8) | Some(15) | Some(16));
    if blocked {
        return;
    }
    let Some((_, _, at, _)) = ctx.state.place(order, who) else {
        return;
    };
    if (at[0] - walker.position[0]).hypot(at[1] - walker.position[1]) < 1.0 {
        return;
    }
    let toward = mv::heading_to(walker.position, at);
    // Its own turn carries on; another turn under way (a travel's end
    // heading) isn't taken over: taking it over every frame turned it back
    // and forth between the two headings and it never ended, the turn
    // animation playing in place (Doc Mitchell after getting up).
    let own_turn =
        walker.turn.active && mv::wrap_pi(walker.turn.target - toward).abs() <= mv::ONE_DEGREE;
    if own_turn
        || (!walker.turn.active
            && walker
                .turn
                .should_face(walker.heading, toward, &moves.settings))
    {
        face(walker, toward, ctx.dt, false, ctx.moves);
    }
}

/// The cells the game keeps in memory after the player leaves them (its
/// cell buffers, `[General] uInterior Cell Buffer` 3 and `uExterior Cell
/// Buffer` 36 in this install's INI): the most recent interiors and outdoor
/// squares the player has been in. Which ones the game keeps (most recently
/// used here) isn't traced.
#[derive(Resource)]
pub struct CellBuffer {
    interiors: VecDeque<FormId>,
    squares: VecDeque<FormId>,
    interior_size: usize,
    exterior_size: usize,
}

impl CellBuffer {
    pub fn new(game: &cellview::Game) -> CellBuffer {
        let u =
            |key: &str, d: f32| game.settings.float("General", key).unwrap_or(d).max(0.0) as usize;
        CellBuffer {
            interiors: VecDeque::new(),
            squares: VecDeque::new(),
            interior_size: u("uInterior Cell Buffer", 3.0),
            exterior_size: u("uExterior Cell Buffer", 36.0),
        }
    }

    fn visit(&mut self, cell: FormId, interior: bool) {
        let (list, size) = if interior {
            (&mut self.interiors, self.interior_size)
        } else {
            (&mut self.squares, self.exterior_size)
        };
        if list.front() == Some(&cell) {
            return;
        }
        list.retain(|c| *c != cell);
        list.push_front(cell);
        list.truncate(size);
    }

    fn holds(&self, cell: FormId) -> bool {
        self.interiors.contains(&cell) || self.squares.contains(&cell)
    }
}

/// Who moves out of sight, and when each last moved.
#[derive(Default)]
pub struct Offstage {
    /// Persistent people (always in memory), found once.
    persistent: Vec<FormId>,
    found: bool,
    /// When each last moved, game hours since the game began.
    last: HashMap<FormId, f64>,
    /// Where the low list's round stopped.
    cursor: usize,
    navs: world::ai::NavCache,
}

/// People out of sight walk on (`world::ai::move_offstage`), as the game's
/// lower process levels move them (`009334b0`, `0096b810`/`0096b470`/
/// `0096b050`, `009ea8a0`): the persistent people (who stay in memory) and
/// anyone in a cell the game still holds (its cell buffers), not on
/// screen. Someone in a held cell is moved every 0.3 game hours (middle-low;
/// the middle-high 0.15 is for cells loading or detaching, which the viewer
/// doesn't have), anyone else every game hour (low), the low ones within
/// `iLowProcessingMilliseconds` (2 ms) a frame, resuming where the last
/// frame stopped. Each update covers the game time since their last at
/// their walking speed (`fMoveBaseSpeed` × SpeedMult; running in a fight).
#[allow(clippy::too_many_arguments)]
pub fn move_offstage(
    game: Res<GameFiles>,
    mut state: ResMut<DialogueState>,
    shown: Query<(&Walker, &Visibility)>,
    mut buffer: ResMut<CellBuffer>,
    moves: Res<Moves>,
    frozen: Res<FrozenAi>,
    conversation: Res<Conversation>,
    mut off: Local<Offstage>,
) {
    let order = &game.0.order;
    let state = &mut state.0;
    // The player's cell goes into the buffer as they enter it.
    if let Some(cell) = state.player_cell {
        buffer.visit(cell, state.player_world.is_none());
    }
    if frozen.0 || conversation.0.as_ref().is_some_and(|t| !t.is_line_only()) {
        return;
    }
    let Some(days) = state.global(order, "GameDaysPassed") else {
        return;
    };
    let now = f64::from(days) * 24.0;
    let time_scale = state.global(order, "TimeScale").unwrap_or(30.0);
    if !off.found {
        off.found = true;
        let mut people: Vec<FormId> = [esm::sig::ACHR, esm::sig::ACRE]
            .into_iter()
            .flat_map(|k| order.records_of_type(k))
            .filter(|rr| rr.entry.header.flags & 0x400 != 0 && !rr.entry.header.is_deleted())
            .map(|rr| rr.form_id)
            .collect();
        people.sort();
        off.persistent = people;
    }
    let on_screen: HashSet<FormId> = shown
        .iter()
        .filter(|(_, v)| **v != Visibility::Hidden)
        .map(|(w, _)| w.reference)
        .collect();
    let here = state.player_world.or(state.player_cell);
    // Those in held cells: the people the state has moved there, and the
    // persistent ones.
    let mut middle: Vec<FormId> = state
        .positions
        .keys()
        .chain(state.spaces.keys())
        .copied()
        .filter(|r| *r != PLAYER_REF)
        .collect();
    middle.sort();
    middle.dedup();
    let level_of = |state: &world::scripting::GameState, who: FormId| {
        let (_, cell, ..) = state.place(order, who)?;
        Some(if buffer.holds(cell) {
            ProcessLevel::MiddleLow
        } else {
            ProcessLevel::Low
        })
    };
    let eligible = |state: &world::scripting::GameState, who: FormId| {
        who != PLAYER_REF
            && !on_screen.contains(&who)
            && !state.dead.contains(&who)
            && world::enabled_now(order, who, &state.disabled)
            && !(state.place(order, who).map(|p| p.0) == here && here.is_some() && {
                // In the loaded place but not drawn: outdoors beyond the
                // loaded squares only.
                state.player_world.is_none()
            })
    };
    let step = |state: &mut world::scripting::GameState,
                off: &mut Offstage,
                who: FormId,
                level: ProcessLevel| {
        let last = off.last.get(&who).copied();
        if !level.due(last, now) {
            return;
        }
        off.last.insert(who, now);
        let hours = last.map(|t| (now - t) as f32);
        let in_combat = state.combat.contains_key(&who);
        let seconds = mv::offstage_seconds(hours, time_scale, in_combat, 0.016);
        let facts = world::scripting::Facts {
            order,
            state,
            speaker: None,
        };
        let speed_mult = facts.current_actor_value(who, 21).unwrap_or(100.0) as f32;
        let legs = [29u16, 30]
            .iter()
            .filter(|&&av| facts.current_actor_value(who, av).is_some_and(|v| v <= 0.0))
            .count() as u8;
        let speed = mv::offstage_speed(&moves.settings, speed_mult, legs, in_combat);
        let before = state.place(order, who).map(|p| p.0);
        let result = world::ai::move_offstage(order, state, who, speed * seconds, &mut off.navs);
        if result != world::ai::Offstage::Stayed {
            let after = state.place(order, who).map(|p| p.0);
            if before != after {
                println!(
                    "{who} (out of sight) walks on into {}",
                    after.unwrap_or_default()
                );
            }
        }
    };
    // Whether someone could be due at all: at the shorter interval (a
    // level's `due` implies the shorter one's), before the checks that read
    // their records.
    let maybe_due =
        |off: &Offstage, who: FormId| ProcessLevel::MiddleLow.due(off.last.get(&who).copied(), now);
    for who in middle {
        if !maybe_due(&off, who) {
            continue;
        }
        if !eligible(state, who) || off.persistent.binary_search(&who).is_ok() {
            continue;
        }
        if let Some(level) = level_of(state, who) {
            step(state, &mut off, who, level);
        }
    }
    // The persistent people, a share each frame within the time allowed.
    let budget = std::time::Duration::from_secs_f32(
        world::scripting::game_setting(order, "iLowProcessingMilliseconds").unwrap_or(2.0) / 1000.0,
    );
    let started = std::time::Instant::now();
    let count = off.persistent.len();
    let mut done = 0;
    while done < count {
        let who = off.persistent[off.cursor % count];
        off.cursor = (off.cursor + 1) % count;
        done += 1;
        if maybe_due(&off, who) && eligible(state, who) {
            if let Some(level) = level_of(state, who) {
                step(state, &mut off, who, level);
            }
        }
        // The game looks at the clock every 5 people.
        if done % 5 == 0 && started.elapsed() >= budget {
            break;
        }
    }
}

/// At the load door they were walking to: through it, to its far side
/// (where the game puts the player too), in the other place. They're then
/// looked at again: gone from here, or (`bring_in_people`) on screen there.
fn go_through(
    order: &esm::LoadOrder,
    state: &mut world::scripting::GameState,
    walker: &mut Walker,
) {
    let Some(d) = walker.door.take() else {
        return;
    };
    println!(
        "{} goes through {} to {}",
        walker.reference,
        d.door,
        order
            .get(d.to_space)
            .and_then(|r| r.record().ok())
            .and_then(|r| r.editor_id())
            .unwrap_or_default()
    );
    state.stand(walker.reference);
    state
        .spaces
        .insert(walker.reference, (d.to_space, d.to_cell));
    state.positions.insert(walker.reference, (d.to, d.heading));
    walker.clear_path();
    walker.target = None;
    walker.package = None;
    walker.fresh = true;
}

/// Everyone who can be noticed as this frame begins (`fighting::Seen`):
/// the player (moving and attacking as the state and the player's attack
/// say), then the people on screen and alive, moving as their rigs show
/// and attacking while their attack lasts; and the same as obstacles for
/// walkers (`world::movement::Obstacle`).
fn seen(
    order: &esm::LoadOrder,
    state: &world::scripting::GameState,
    attack: &crate::combat::PlayerAttack,
    now: f32,
    actors: &Query<Person, NpcOnly>,
    player_velocity: [f32; 3],
) -> (Vec<crate::fighting::Seen>, Vec<Obstacle>) {
    let mut out = Vec::new();
    let mut obstacles = Vec::new();
    if let Some(p) = state
        .player_position
        .filter(|_| !state.dead.contains(&PLAYER_REF))
    {
        let weapon = world::combat::weapon_in_hand(order, state, PLAYER_REF);
        let attack_time = weapon.as_ref().map_or(0.5, |w| w.shot_interval());
        out.push(crate::fighting::Seen {
            reference: PLAYER_REF,
            position: p,
            moving: state.player_moving,
            running: state.player_running,
            attacking: attack.fired_at.is_some_and(|t| now - t < attack_time),
            radius: world::combat_ai::PERSON_RADIUS,
        });
        obstacles.push(Obstacle {
            who: PLAYER_REF,
            position: p,
            velocity: player_velocity,
            radius: world::combat_ai::PERSON_RADIUS,
            is_player: true,
            seated: false,
        });
    }
    for (walker, _, rig, _, visibility) in actors.iter() {
        if *visibility == Visibility::Hidden || state.dead.contains(&walker.reference) {
            continue;
        }
        let kit = walker.kit.as_ref();
        let attack_time = kit.map_or(1.0, |k| k.attack_animation);
        let radius = kit.map_or(world::combat_ai::PERSON_RADIUS, |k| k.radius);
        out.push(crate::fighting::Seen {
            reference: walker.reference,
            position: walker.position,
            moving: rig.walking,
            running: rig.running,
            attacking: rig.attack_at.is_some_and(|t| now - t < attack_time),
            radius,
        });
        obstacles.push(Obstacle {
            who: walker.reference,
            position: walker.position,
            velocity: walker.velocity,
            radius,
            is_player: false,
            seated: state.sitters.contains_key(&walker.reference),
        });
    }
    // The other players (co-op): real targets at their real places, moving as
    // they move, whether or not their puppet is on screen.
    for (reference, position, moving, running) in world::coop::combat_live::remote_seen(state) {
        if !out.iter().any(|o| o.reference == reference) {
            out.push(crate::fighting::Seen {
                reference,
                position,
                moving,
                running,
                attacking: false,
                radius: world::combat_ai::PERSON_RADIUS,
            });
        }
    }
    (out, obstacles)
}

/// Lying where they fell: turned a quarter onto their side.
fn fallen_transform(walker: &Walker) -> Transform {
    let side = Mat4::from_cols(
        Vec4::new(0.0, 0.0, -1.0, 0.0),
        Vec4::new(0.0, 1.0, 0.0, 0.0),
        Vec4::new(1.0, 0.0, 0.0, 0.0),
        Vec4::new(0.0, 0.0, 0.0, 1.0),
    );
    let game = Mat4::from_cols_array(&walker.game_matrix()) * side;
    Transform::from_matrix(Mat4::from_cols_array(&space::matrix(&game.to_cols_array())))
}

/// Turns someone in place toward a heading, the game's way (a turn
/// request when more than a degree off, then the in-place rate: 135°/s for
/// people, 225°/s in combat; `009e7610`, `009e7d70`). Whether they're
/// turning.
pub(crate) fn face(
    walker: &mut Walker,
    want: f32,
    dt: f32,
    combat: bool,
    settings: &MoveSettings,
) -> bool {
    let target = want.rem_euclid(std::f32::consts::TAU);
    if !walker.turn.active || mv::wrap_pi(walker.turn.target - target).abs() > mv::ONE_DEGREE {
        walker.turn.request(walker.heading, target, settings);
    }
    let rate = walker.rates[usize::from(combat)];
    let side = walker.turn.update(&mut walker.heading, dt, rate);
    if side.is_some() {
        walker.turning = side;
    }
    walker.turn.active
}

/// Walks along the path for `dt` seconds (`009e0a00`); whether they're
/// still on it. A turn in place under way comes first (the walk waits for
/// it). The walk ends within the path's radius of its end
/// (`world::movement::arrived`). The steering point moves on in tenths
/// (`advance_progress`); the facing turns toward the point a tenth further
/// by the walking rule (`walking_turn`, or toward `face_point` when set),
/// the forward speed cut on sharp turns; the body moves toward the steering
/// point whatever the facing (`009e3560`).
pub(crate) fn step(walker: &mut Walker, speed: f32, dt: f32) -> bool {
    let n = walker.path.len();
    if walker.next >= n || n == 0 {
        return false;
    }
    if walker.turn.active {
        let side = walker.turn.update(&mut walker.heading, dt, walker.rates[0]);
        if side.is_some() {
            walker.turning = side;
        }
        walker.last_move = 0.0;
        return true;
    }
    let pos = walker.position;
    let goal = walker.path[n - 1];
    if mv::arrived(pos, goal, walker.radius) {
        walker.clear_path();
        return false;
    }
    // The stuck test (`009e4cf0`, before the move): stuck, the walk fails
    // where they stand ([`unstick`] marks the triangle and asks again).
    // Only with a controller: without one nothing can stop them.
    if walker.body.is_some()
        && walker
            .stuck
            .update(pos, walker.last_move, walker.against_someone, dt)
    {
        walker.stuck_at = Some(pos);
        walker.clear_path();
        walker.last_move = 0.0;
        return false;
    }
    let frame_move = speed * dt;
    walker.progress = mv::advance_progress(&walker.path, walker.progress, pos, frame_move);
    let steer = mv::path_point(&walker.path, walker.progress);
    let look = match walker.face_point {
        Some(p) => p,
        None => mv::path_point(&walker.path, walker.progress + mv::PATH_STEP),
    };
    let flat_look = (look[0] - pos[0]).hypot(look[1] - pos[1]);
    let desired = if flat_look > 1e-3 {
        mv::heading_to(pos, look)
    } else {
        walker.heading
    };
    let ahead = mv::look_ahead_point(&walker.path, walker.progress, pos, speed);
    let factor = mv::walking_turn_factor(
        walker.turn_speed,
        walker.face_point.is_none(),
        mv::heading_to(pos, ahead) - walker.heading,
    );
    let w = mv::walking_turn(walker.heading, desired, walker.turn_speed, dt, factor);
    walker.heading = w.heading;
    let (dx, dy, dz) = (steer[0] - pos[0], steer[1] - pos[1], steer[2] - pos[2]);
    let flat = dx.hypot(dy);
    walker.last_move = 0.0;
    if flat > 1e-4 {
        // The move: this frame's forward motion, turned toward the steering
        // point (`009e3560`), no longer than the way left to it (`009e0a00`
        // shortens the move vector to the distance to the path's end).
        let go = (frame_move * w.forward).min(flat);
        walker.last_move = go;
        let along = [dx / flat * go, dy / flat * go];
        if walker.body.is_some() {
            // The character controller moves them ([`move_body`]).
            walker.wanted = Some(along);
        } else {
            // No collision under them yet: along the path, at its height.
            walker.position = [
                pos[0] + along[0],
                pos[1] + along[1],
                pos[2] + dz * (go / flat),
            ];
        }
    } else if (walker.progress - (n - 1) as f32).abs() < 1e-4 && walker.body.is_none() {
        walker.position = goal;
    }
    walker.next = ((walker.progress.floor() as usize) + 1).min(n - 1);
    true
}

/// The path request someone's walks are asked with: their radius
/// (`world::ai::request_radius`) and the defaults (`006e2420`).
fn request_of(walker: &Walker) -> world::ai::navsearch::PathRequest<'static> {
    world::ai::navsearch::PathRequest {
        radius: walker.request_radius,
        ..Default::default()
    }
}

/// The path manager (`PathManager::BuildPath` (Xbox PDB), `006eb9d0`):
/// with `bBackgroundPathing` 1 (`00f8ac20`) an actor's path request
/// (`ActorMover`'s, `009db090`, which sets the mover waiting for its path,
/// +0x6c = 1) becomes a task (`PathingTaskData` (Xbox PDB), processed by
/// `006e9fc0` on the task threads) and its solution reaches the actor as a
/// message on a later frame (`PathManagerImpl::Update` `006eae40`). Here
/// one worker thread searches the requests in order, each on the navmesh
/// as it was when asked (a shared copy); the walker stands meanwhile.
/// (The game builds at once when `0094df60` says so, a process-list count
/// at +0x654 above 0, not identified.)
#[derive(Resource)]
pub struct PathQueue {
    jobs: std::sync::mpsc::Sender<PathJob>,
    done: std::sync::Mutex<std::sync::mpsc::Receiver<PathDone>>,
    next: u64,
}

struct PathJob {
    id: u64,
    who: FormId,
    mesh: std::sync::Arc<NavMesh>,
    from: [f32; 3],
    to: [f32; 3],
    radius: f32,
    target_radius: f32,
}

/// A searched path: its points and doors, if one was found.
type PathFound = Option<(Vec<[f32; 3]>, Vec<(FormId, [f32; 3])>)>;

/// Why a search found nothing, and the goal it was for.
type Failure = (String, [f32; 3]);

struct PathDone {
    id: u64,
    who: FormId,
    found: PathFound,
    /// Why none was found: no way onto the navmesh, none off it to the
    /// goal, or no route between.
    why: String,
    to: [f32; 3],
}

impl Default for PathQueue {
    fn default() -> Self {
        let (jobs, inbox) = std::sync::mpsc::channel::<PathJob>();
        let (outbox, done) = std::sync::mpsc::channel::<PathDone>();
        std::thread::Builder::new()
            .name("pathing".into())
            .spawn(move || {
                while let Ok(job) = inbox.recv() {
                    let request = world::ai::navsearch::PathRequest {
                        radius: job.radius,
                        target_radius: job.target_radius,
                        ..Default::default()
                    };
                    let found = job.mesh.plan(job.from, job.to, &request);
                    let why = if found.is_some() {
                        String::new()
                    } else if job.mesh.start_end(job.from, job.radius).is_none() {
                        format!(
                            "no way onto the navmesh: {}",
                            job.mesh.explain_off(job.from)
                        )
                    } else if job
                        .mesh
                        .goal_end(job.to, job.radius, job.target_radius)
                        .is_none()
                    {
                        format!(
                            "no way from the navmesh to the goal: {}",
                            job.mesh.explain_off(job.to)
                        )
                    } else {
                        "no route over the navmesh".to_string()
                    };
                    if outbox
                        .send(PathDone {
                            id: job.id,
                            who: job.who,
                            found,
                            why,
                            to: job.to,
                        })
                        .is_err()
                    {
                        break;
                    }
                }
            })
            .ok();
        PathQueue {
            jobs,
            done: std::sync::Mutex::new(done),
            next: 1,
        }
    }
}

impl PathQueue {
    /// The solutions that came back since the last frame, by walker.
    fn take_done(&self) -> HashMap<FormId, Vec<(u64, PathFound, Failure)>> {
        let mut out: HashMap<FormId, Vec<(u64, PathFound, Failure)>> = HashMap::new();
        if let Ok(rx) = self.done.lock() {
            while let Ok(d) = rx.try_recv() {
                out.entry(d.who)
                    .or_default()
                    .push((d.id, d.found, (d.why, d.to)));
            }
        }
        out
    }
}

/// Whether a place lies beyond the attached navmesh: no triangle holds it
/// and no open edge is near enough for a ray-cast way to it
/// (`world::ai::offmesh`). Such a goal is the navmesh info search's
/// (`long_walk`, `006c94c0`), not a detailed path's.
fn beyond_attached(mesh: &NavMesh, to: [f32; 3]) -> bool {
    mesh.find_triangle(to).is_none()
        && mesh
            .edge_spots(to, world::ai::offmesh::FIND_CLOSEST_EDGES_RADIUS)
            .is_empty()
}

/// What asking the path manager needs this frame: the queue and the
/// navmesh as it now is (shared with the searches).
pub(crate) struct Asking<'a> {
    queue: &'a mut PathQueue,
    mesh: std::sync::Arc<NavMesh>,
}

/// A path asked for and not back yet, and what to do with it when it comes
/// (the walk it's for).
#[derive(Debug, Clone)]
pub(crate) struct Pending {
    id: u64,
    radius: f32,
    turn_first: bool,
    arrival: Option<f32>,
    path_target: Option<[f32; 3]>,
    partial: bool,
    /// What the walk is, for the log.
    note: String,
}

/// Asks the path manager for someone's path to `to` ([`PathQueue`]); they
/// wait for it ([`Walker::pending`]) and walk it as `then` says when it
/// comes ([`path_came`]).
fn ask_path(
    queue: &mut PathQueue,
    mesh: &std::sync::Arc<NavMesh>,
    walker: &mut Walker,
    to: [f32; 3],
    mut then: Pending,
) {
    let id = queue.next;
    queue.next += 1;
    then.id = id;
    let sent = queue.jobs.send(PathJob {
        id,
        who: walker.reference,
        mesh: mesh.clone(),
        from: walker.position,
        to,
        radius: walker.request_radius,
        target_radius: then.radius,
    });
    if sent.is_ok() {
        walker.pending = Some(then);
    }
}

/// A walk's path came back ([`ask_path`]): walked as asked, or reported
/// missing.
fn path_came(walker: &mut Walker, found: PathFound, failure: Failure, settings: &MoveSettings) {
    let Some(p) = walker.pending.take() else {
        return;
    };
    let me = walker.reference;
    match found {
        Some((path, doors)) => {
            let length: f32 = path.windows(2).map(|w| distance(w[0], w[1])).sum();
            println!("{me} walks {length:.0} units ({})", p.note);
            walker.set_path(path, p.radius, p.turn_first, settings);
            walker.arrival = p.arrival;
            walker.partial = p.partial;
            walker.doors_ahead = doors;
            if p.path_target.is_some() {
                walker.path_target = p.path_target;
            }
        }
        None => {
            // Said once while it stays the same.
            let at = walker.position;
            let (why, to) = failure;
            let key = (
                why.clone(),
                [at, to].map(|p| p.map(|v| (v / 64.0).round() as i32)),
            );
            if walker.last_failure.as_ref() != Some(&key) {
                walker.last_failure = Some(key);
                println!(
                    "{me} finds no way from ({:.0}, {:.0}, {:.0}) to ({:.0}, {:.0}, {:.0}): {why} ({}).",
                    at[0], at[1], at[2], to[0], to[1], to[2], p.note
                );
            }
        }
    }
}

/// A path for someone from where they stand ([`request_of`]).
pub(crate) fn path_for(mesh: &NavMesh, walker: &Walker, to: [f32; 3]) -> Option<Vec<[f32; 3]>> {
    path_with_doors_for(mesh, walker, to).map(|(p, _)| p)
}

/// [`path_for`], and the doors on it.
#[allow(clippy::type_complexity)]
pub(crate) fn path_with_doors_for(
    mesh: &NavMesh,
    walker: &Walker,
    to: [f32; 3],
) -> Option<(Vec<[f32; 3]>, Vec<(FormId, [f32; 3])>)> {
    path_from_for(mesh, walker, walker.position, to)
}

/// [`path_with_doors_for`] from another point.
#[allow(clippy::type_complexity)]
fn path_from_for(
    mesh: &NavMesh,
    walker: &Walker,
    from: [f32; 3],
    to: [f32; 3],
) -> Option<(Vec<[f32; 3]>, Vec<(FormId, [f32; 3])>)> {
    mesh.plan(from, to, &request_of(walker))
}

/// A path round others in the way (avoid nodes, `009e5ae0`).
fn path_avoiding_for(
    mesh: &NavMesh,
    walker: &Walker,
    to: [f32; 3],
    avoid: &[mv::AvoidNode],
) -> Option<Vec<[f32; 3]>> {
    let request = world::ai::navsearch::PathRequest {
        avoid,
        ..request_of(walker)
    };
    mesh.plan(walker.position, to, &request).map(|(p, _)| p)
}

/// The size of someone's character controller: people all have the
/// player's (`physics::CharacterShape::PLAYER`, `00c72410` uses one shared
/// shape for anything that isn't a creature); a creature's radius is its
/// skeleton's (`fighting::Kit`) and its height 128 × its scale (a guess, as
/// for the player's collision with them in `walk::people`: the game sizes it
/// from the skeleton's `BSBound`, `00c55170`, not read here).
pub(crate) fn body_shape(walker: &Walker) -> physics::CharacterShape {
    let shape = physics::CharacterShape::PLAYER;
    match walker.kit.as_ref() {
        Some(k) if k.creature.is_some() => physics::CharacterShape {
            radius: k.radius,
            height: shape.height * walker.scale,
            ..shape
        },
        _ => shape,
    }
}

/// Whether someone's controller is passed through by everyone else's: in
/// furniture from sitting down until getting up begins
/// (`Sitter::others_pass_through`, `00920d00` and `00c711d0`).
pub(crate) fn passed_through(state: &world::scripting::GameState, who: FormId) -> bool {
    state
        .sitters
        .get(&who)
        .is_some_and(|s| s.others_pass_through())
}

/// The others someone's controller runs into (the game's controllers
/// collide with each other, layer 30 with itself; `physics::Person`): the
/// player and everyone alive on screen, where they stood as this frame
/// began, but not those in furniture whose controllers others pass
/// through ([`passed_through`]).
fn bodies(
    state: &world::scripting::GameState,
    actors: &Query<Person, NpcOnly>,
) -> Vec<(FormId, physics::Person)> {
    let mut out = Vec::new();
    if let Some(p) = state
        .player_position
        .filter(|_| !state.dead.contains(&PLAYER_REF) && !passed_through(state, PLAYER_REF))
    {
        let shape = physics::CharacterShape::PLAYER;
        out.push((
            PLAYER_REF,
            physics::Person {
                feet: p,
                radius: shape.radius,
                height: shape.height,
            },
        ));
    }
    for (walker, _, _, _, visibility) in actors.iter() {
        if *visibility == Visibility::Hidden
            || state.dead.contains(&walker.reference)
            || passed_through(state, walker.reference)
        {
            continue;
        }
        let shape = body_shape(walker);
        out.push((
            walker.reference,
            physics::Person {
                feet: walker.position,
                radius: shape.radius,
                height: shape.height,
            },
        ));
    }
    out
}

/// `NV_GROUND_LOG=1`: once a second, everyone on screen's feet against the
/// collision under them (a ray from 600 above) and the land's own height
/// (`LAND`), with their controller's state. A diagnostic only.
pub fn ground_log(
    time: Res<Time>,
    nav: Res<CellNav>,
    collision: Res<crate::walk::CellCollision>,
    state: Res<DialogueState>,
    mut next: Local<(f32, f32)>,
    actors: Query<(&Walker, &Visibility)>,
    player: Option<Res<crate::walk::Player>>,
) {
    static ON: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    if !*ON.get_or_init(|| std::env::var("NV_GROUND_LOG").is_ok_and(|v| v == "1")) {
        return;
    }
    let now = time.elapsed_secs();
    let describe = |b: &physics::Character| {
        let state = match b.controller.state {
            physics::controller::State::OnGround => "ground",
            physics::controller::State::Jumping => "jumping",
            physics::controller::State::InAir => "air",
        };
        format!("{state} {:.1} vz {:.0}", b.ground, b.vertical_speed)
    };
    // The player, ten times a second.
    if let Some(p) = player.filter(|p| p.walking && now >= next.1) {
        next.1 = now + 0.1;
        let f = p.character.feet;
        let cast = collision
            .0
            .raycast([f[0], f[1], f[2] + 600.0], [0.0, 0.0, -1.0], 2000.0);
        let hit = cast.map(|(t, _)| f[2] + 600.0 - t);
        let land = nav.mesh.land_height(f);
        println!(
            "GROUND {now:.2} player at ({:.0}, {:.0}, {:.1}) collision {} land {} body {} walking {}",
            f[0],
            f[1],
            f[2],
            hit.map_or("-".into(), |h| format!("{h:.1} ({:+.1})", f[2] - h)),
            land.map_or("-".into(), |h| format!("{h:.1} ({:+.1})", f[2] - h)),
            describe(&p.character),
            p.speed > 0.0,
        );
    }
    if now < next.0 {
        return;
    }
    next.0 = now + 1.0;
    let state = &state.0;
    for (w, vis) in actors.iter() {
        if *vis == Visibility::Hidden || state.dead.contains(&w.reference) || w.fallen {
            continue;
        }
        let p = w.position;
        let cast = collision
            .0
            .raycast([p[0], p[1], p[2] + 600.0], [0.0, 0.0, -1.0], 2000.0);
        let hit = cast.map(|(t, _)| p[2] + 600.0 - t);
        let what = cast.map_or(String::new(), |(_, i)| {
            format!(
                " [{:08X} layer {}]",
                collision.0.reference(i),
                collision.0.layer(i)
            )
        });
        let land = nav.mesh.land_height(p);
        let body = match &w.body {
            None => "none".to_string(),
            Some(b) => describe(b),
        };
        println!(
            "GROUND {now:.1} {} at ({:.0}, {:.0}, {:.1}) collision {}{what} land {} body {} walking {}",
            w.reference,
            p[0],
            p[1],
            p[2],
            hit.map_or("-".into(), |h| format!("{h:.1} ({:+.1})", p[2] - h)),
            land.map_or("-".into(), |h| format!("{h:.1} ({:+.1})", p[2] - h)),
            body,
            w.walking_now(),
        );
    }
}

/// How far below their feet collision must be for someone to get a
/// character controller ([`move_body`]).
const GROUND_PROBE: f32 = 256.0;

/// What `MobileObject::Move`'s rules around the controller look at
/// ([`world::ground`]): the attached cells' navmesh (with their land) and
/// the world camera, in game units.
pub(crate) struct Ground<'a> {
    pub mesh: &'a NavMesh,
    pub camera: Option<[f32; 3]>,
}

/// Further than `fCharControllerWarpDistSqr` from the camera, someone's
/// move is added to where they stand and their feet put on the navmesh
/// under the new spot, without their controller (`0092f260`'s far branch,
/// [`world::ground::moves_without_controller`]); the controller is put
/// there too (`SetPosition`, `00931620`). Standing still, the same with no
/// move. Whether it did: no navmesh there, and the controller moves them.
fn far_move(walker: &mut Walker, wanted: Option<[f32; 2]>, rules: &Ground) -> bool {
    let Some(camera) = rules.camera else {
        return false;
    };
    let p = walker.position;
    let distance_sq: f32 = (0..3).map(|k| (p[k] - camera[k]).powi(2)).sum();
    let mover = world::ground::Mover {
        immobile: walker.immobile,
        ..Default::default()
    };
    if !world::ground::moves_without_controller(
        mover,
        distance_sq,
        world::ground::CHAR_CONTROLLER_WARP_DIST_SQR,
    ) {
        return false;
    }
    let m = wanted.unwrap_or([0.0; 2]);
    let to = [p[0] + m[0], p[1] + m[1], p[2]];
    let Some(z) = world::ground::navmesh_height(rules.mesh, to) else {
        return false;
    };
    walker.position = [to[0], to[1], z];
    if let Some(body) = walker.body.as_mut() {
        body.feet = walker.position;
        body.ground = z;
    }
    true
}

/// Moves someone through the cell's collision with their character
/// controller, wanting this frame's move ([`Walker::wanted`]; none: standing
/// still), as the player's controller moves the player: on the ground at
/// the wanted velocity, falling, stepping up to 31, sliding along walls and
/// round other people (`physics::Character::update_controlled`; the game
/// hands the mover's move vector to the controller each frame, `009ddc00`).
/// Someone moved by anything else since (a script, furniture, a door)
/// starts again where they now are.
///
/// Around the controller, the two rules of `MobileObject::Move`
/// (`0092f260`, [`world::ground`]): further than
/// `fCharControllerWarpDistSqr` from the camera they walk on the navmesh's
/// height without the controller; after the controller, outdoors, feet more
/// than 30 under the land are put on it.
///
/// The viewer's collision loads behind the squares people stand in; the
/// game never has a high-process actor without its cell's collision. So
/// until there is collision within [`GROUND_PROBE`] under them, someone has
/// no controller and walks the path at its own height (not game
/// behaviour: it only bridges the viewer's loading).
pub(crate) fn move_body(
    walker: &mut Walker,
    collider: &mut physics::Collider,
    others: &[(FormId, physics::Person)],
    movers: &mut Vec<physics::proxy::SurfaceContact>,
    rules: &Ground,
    dt: f32,
) {
    let wanted = walker.wanted.take();
    if far_move(walker, wanted, rules) {
        walker.against_someone = false;
        return;
    }
    let p = walker.position;
    if let Some(body) = &walker.body {
        let moved = (body.feet[0] - p[0]).hypot(body.feet[1] - p[1]) > 0.5
            || (body.feet[2] - p[2]).abs() > 1.0;
        if moved {
            walker.body = None;
        }
    }
    if walker.body.is_none() {
        let ground = collider.raycast(
            [p[0], p[1], p[2] + 64.0],
            [0.0, 0.0, -1.0],
            64.0 + GROUND_PROBE,
        );
        if ground.is_none() {
            if let Some(m) = wanted {
                walker.position = [p[0] + m[0], p[1] + m[1], p[2]];
            }
            // Under the land (back from a walk out of sight, `move_offstage`,
            // whose rough positions can be under it): on the land, as
            // `MobileObject::Move` puts anyone the controller left there.
            let q = walker.position;
            walker.position[2] = world::ground::kept_above_land(q[2], rules.mesh.land_height(q));
            return;
        }
        walker.body = Some(physics::Character::new(p));
    }
    if dt <= 0.0 {
        return;
    }
    let me = walker.reference;
    collider.set_people(
        others
            .iter()
            .filter(|(who, _)| *who != me)
            .map(|(_, person)| *person)
            .collect(),
    );
    let shape = body_shape(walker);
    let desired = wanted.map_or([0.0; 2], |m| [m[0] / dt, m[1] / dt]);
    let body = walker.body.as_mut().expect("made above");
    body.update_controlled(
        collider,
        &shape,
        desired,
        None,
        world::locomotion::air_gain(world::locomotion::AIR_CONTROL),
        dt,
    );
    body.fell = None;
    // What the proxy touched, for the clutter to push (`applySurfaceInteractions`).
    movers.extend(body.surface.iter().copied());
    // Outdoors, feet left more than 30 under the land are put on it
    // (`0092f260` at `0093012a`: the land's height under them, `004572e0`,
    // and `SetPosition`, which moves the controller too, `00931620`).
    let lifted = world::ground::kept_above_land(body.feet[2], rules.mesh.land_height(body.feet));
    if lifted != body.feet[2] {
        body.feet[2] = lifted;
        body.ground = lifted;
    }
    walker.position = body.feet;
    // Walked off the collision loaded so far (into a square still loading):
    // no controller till there is ground under them again (the same bridge
    // as above; the game's high-process actors always have their cell's
    // collision).
    if !body.on_ground {
        let f = body.feet;
        let ground = collider.raycast(
            [f[0], f[1], f[2] + 64.0],
            [0.0, 0.0, -1.0],
            64.0 + GROUND_PROBE,
        );
        if ground.is_none() {
            walker.position = [f[0], f[1], body.ground];
            walker.body = None;
        }
    }
    // Up against someone (the controller's contact is an actor, as the stuck
    // test asks, `009e4cf0`): touching their cylinder.
    let feet = walker.position;
    walker.against_someone = others.iter().any(|(who, p)| {
        *who != me
            && (p.feet[0] - feet[0]).hypot(p.feet[1] - feet[1]) < shape.radius + p.radius + 0.5
            && feet[2] < p.feet[2] + p.height
            && p.feet[2] < feet[2] + shape.height
    });
}

/// Someone whose walk got stuck (`009e4cf0`): the triangle they stand on
/// gets an obstacle (`00691510`: the request's radius, cost 1; the path
/// smoother then goes round it, `world::ai::smoother`) and they ask for
/// their way again (the walk failed short of its place, and the travel
/// procedure asks again, `008e5e90`). The game first looks for someone
/// standing in the way and has them make room instead of marking the
/// triangle (`009e4cf0`'s loop over the high actors; what it asks of them,
/// `00804cb0`/`00819a50`, wasn't traced): not done.
fn unstick(
    walker: &mut Walker,
    mesh: &mut std::sync::Arc<NavMesh>,
    queue: &mut PathQueue,
    _settings: &MoveSettings,
    now: f32,
) {
    let Some(at) = walker.stuck_at.take() else {
        return;
    };
    if let Some(t) = mesh.triangle_at(at) {
        std::sync::Arc::make_mut(mesh).mark_obstacle(t, at, walker.request_radius, 1.0);
    }
    println!(
        "{now:.1} s: {} is stuck at ({:.0}, {:.0}, {:.0}): an obstacle there, the way asked for again.",
        walker.reference, at[0], at[1], at[2]
    );
    let Some(goal) = walker.target else {
        return;
    };
    let then = Pending {
        id: 0,
        radius: walker.radius,
        turn_first: false,
        arrival: walker.arrival,
        path_target: None,
        partial: walker.partial,
        note: "again, after being stuck".into(),
    };
    let shared = mesh.clone();
    ask_path(queue, &shared, walker, goal, then);
}

pub(crate) fn distance(a: [f32; 3], b: [f32; 3]) -> f32 {
    (0..3).map(|i| (a[i] - b[i]).powi(2)).sum::<f32>().sqrt()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn walker() -> Walker {
        let mut w = Walker::at(FormId(1), [0.0, 0.0, 0.0], 0.0, 1.0, false);
        w.set_path(
            vec![[0.0, 0.0, 0.0], [0.0, 100.0, 0.0], [100.0, 100.0, 0.0]],
            0.0,
            false,
            &MoveSettings::defaults(),
        );
        w
    }

    #[test]
    fn the_dead_go_limp_with_the_ai_held_still() {
        // Killed under `tai` or in the dialogue menu: limp at once, once.
        assert!(dies_while_frozen(true, false));
        assert!(!dies_while_frozen(true, true));
        // The living held still stay as they are.
        assert!(!dies_while_frozen(false, false));
    }

    #[test]
    fn a_scripted_say_to_has_the_speaker_look_at_the_player_then_hold_it() {
        let s = world::head_track::Settings::default();
        let (lines, chats) = (Lines::default(), Chats::default());
        let mut doc = walker();
        let me = doc.reference;
        // An earlier choice of their own (no one) set the own slot.
        doc.head_track.set(Slot::Default, None);
        // `SayTo Player`: a line said to the player, no menu.
        head_track_asks(&mut doc, Some(me), false, &lines, &chats, &s);
        assert_eq!(doc.looking_at(), Some(PLAYER_REF));
        assert_eq!(doc.head_track.current_slot(), Some(Slot::Action));
        assert!(!doc.head_track.may_choose());
        // The line over: kept as their own choice for 10 s.
        head_track_asks(&mut doc, None, false, &lines, &chats, &s);
        assert_eq!(doc.looking_at(), Some(PLAYER_REF));
        assert_eq!(doc.head_track.current_slot(), Some(Slot::Default));
        assert!(!doc.head_track.may_choose());
        // The dialogue menu: the DIALOG slot, let go when it closes.
        head_track_asks(&mut doc, Some(me), true, &lines, &chats, &s);
        assert_eq!(doc.head_track.current_slot(), Some(Slot::Dialog));
        head_track_asks(&mut doc, None, false, &lines, &chats, &s);
        assert_eq!(doc.head_track.current_slot(), Some(Slot::Default));
        // `bDisableHeadTracking`: the head doesn't follow.
        let off = world::head_track::Settings {
            disabled: true,
            ..s
        };
        head_track_asks(&mut doc, None, false, &lines, &chats, &off);
        assert_eq!(doc.looking_at(), None);
        assert_eq!(doc.head_track.current(), Some(PLAYER_REF));
    }

    #[test]
    fn walkers_follow_their_path_at_their_speed_and_turn_by_the_walking_rule() {
        let mut w = walker();
        // North at 50 a second for one second (in small frames).
        for _ in 0..10 {
            assert!(step(&mut w, 50.0, 0.1));
        }
        assert!((w.position[1] - 50.0).abs() < 0.5, "{:?}", w.position);
        assert!(w.heading.abs() < 1e-3);
        // On round the corner: the body heads east while the facing
        // catches up at up to 270°/s.
        for _ in 0..40 {
            step(&mut w, 50.0, 0.05);
        }
        assert!(w.position[0] > 20.0, "{:?}", w.position);
        assert!(
            (w.heading - std::f32::consts::FRAC_PI_2).abs() < 0.2,
            "{}",
            w.heading
        );
        for _ in 0..200 {
            if !step(&mut w, 50.0, 0.05) {
                break;
            }
        }
        assert!(!w.on_path());
        assert!((w.position[0] - 100.0).abs() < 1.0, "{:?}", w.position);
    }

    /// A floor 1000 across at z = 0, and a wall across it at y = 100.
    fn floor_and_wall() -> physics::Collider {
        let mut c = physics::Collider::new();
        c.add(
            &[
                [-500.0, -500.0, 0.0],
                [500.0, -500.0, 0.0],
                [500.0, 500.0, 0.0],
                [-500.0, 500.0, 0.0],
            ],
            &[[0, 1, 2], [0, 2, 3]],
        );
        c.add(
            &[
                [-500.0, 100.0, 0.0],
                [500.0, 100.0, 0.0],
                [500.0, 100.0, 300.0],
                [-500.0, 100.0, 300.0],
            ],
            &[[0, 1, 2], [0, 2, 3]],
        );
        c
    }

    /// A navmesh square (0,0)–(1000,1000) at height `z`, over an exterior
    /// square whose land (`LAND`, 33 × 33) stands at `land`.
    fn navmesh_over_land(z: f32, land: f32) -> NavMesh {
        use world::ai::NavTriangle;
        let mut mesh = NavMesh {
            vertices: vec![
                [0.0, -1000.0, z],
                [1000.0, -1000.0, z],
                [1000.0, 1000.0, z],
                [0.0, 1000.0, z],
            ],
            triangles: vec![
                NavTriangle {
                    vertices: [0, 1, 2],
                    ..Default::default()
                },
                NavTriangle {
                    vertices: [0, 2, 3],
                    ..Default::default()
                },
            ],
            ..Default::default()
        };
        mesh.set_land((0, 0), vec![land; 33 * 33]);
        mesh.set_land((0, -1), vec![land; 33 * 33]);
        mesh
    }

    /// Back from a walk out of sight 64 under the land (Ringo coming into
    /// the gunfight, run 2026-10-07): no collision within reach below, so
    /// no controller, and before this they walked on under the ground for
    /// good. `MobileObject::Move` puts anyone more than 30 under the land
    /// on it (`0092f260`); then the controller takes them.
    #[test]
    fn someone_under_the_land_is_put_on_it_and_given_their_controller() {
        let mut collider = floor_and_wall();
        let mesh = navmesh_over_land(0.0, 0.0);
        let rules = Ground {
            mesh: &mesh,
            camera: Some([200.0, -300.0, 64.0]),
        };
        let mut w = Walker::at(FormId(1), [200.0, -200.0, -64.5], 0.0, 1.0, false);
        w.set_path(
            vec![[200.0, -200.0, -64.5], [200.0, -100.0, 0.0]],
            0.0,
            false,
            &MoveSettings::defaults(),
        );
        let dt = 1.0 / 60.0;
        step(&mut w, 85.0, dt);
        move_body(&mut w, &mut collider, &[], &mut Vec::new(), &rules, dt);
        assert_eq!(w.position[2], 0.0, "{:?}", w.position);
        for _ in 0..30 {
            step(&mut w, 85.0, dt);
            move_body(&mut w, &mut collider, &[], &mut Vec::new(), &rules, dt);
        }
        assert!(w.body.as_ref().is_some_and(|b| b.on_ground), "{:?}", w.body);
        assert!(w.position[2].abs() < 1.0, "{:?}", w.position);
        assert!(w.position[1] > -200.0 + 30.0, "walked on: {:?}", w.position);
    }

    /// A controller left more than 30 under the land (a floor below it
    /// where the land is higher) is put on the land (`0093012a`).
    #[test]
    fn a_controller_more_than_thirty_under_the_land_is_put_on_it() {
        let mut collider = floor_and_wall();
        for (land, expect) in [(29.0, 0.0), (31.0, 31.0)] {
            let mesh = navmesh_over_land(0.0, land);
            let rules = Ground {
                mesh: &mesh,
                camera: Some([200.0, -300.0, 64.0]),
            };
            let mut w = Walker::at(FormId(1), [200.0, -200.0, 0.0], 0.0, 1.0, false);
            w.body = Some(physics::Character {
                on_ground: true,
                ..physics::Character::new([200.0, -200.0, 0.0])
            });
            move_body(
                &mut w,
                &mut collider,
                &[],
                &mut Vec::new(),
                &rules,
                1.0 / 60.0,
            );
            assert!(
                (w.position[2] - expect).abs() < 1.0,
                "land {land}: {:?}",
                w.position
            );
        }
    }

    /// Further than `fCharControllerWarpDistSqr` (2449.5) from the camera
    /// the move isn't the controller's: added where they stand, the feet
    /// on the navmesh's height (`0092f260`'s far branch), the wall the
    /// controller stops at nearby passed; within it, the controller again.
    #[test]
    fn far_from_the_camera_people_walk_on_the_navmesh_without_their_controller() {
        let mut collider = floor_and_wall();
        let mesh = navmesh_over_land(12.0, 0.0);
        let path = vec![[200.0, 0.0, 0.0], [200.0, 300.0, 0.0]];
        let dt = 1.0 / 60.0;
        let run = |camera: [f32; 3], collider: &mut physics::Collider| {
            let rules = Ground {
                mesh: &mesh,
                camera: Some(camera),
            };
            let mut w = Walker::at(FormId(1), [200.0, 0.0, 0.0], 0.0, 1.0, false);
            w.set_path(path.clone(), 0.0, false, &MoveSettings::defaults());
            for _ in 0..240 {
                step(&mut w, 85.0, dt);
                move_body(&mut w, collider, &[], &mut Vec::new(), &rules, dt);
            }
            w
        };
        let far = run([200.0, -2600.0, 64.0], &mut collider);
        assert!(
            far.position[1] > 150.0,
            "through the wall: {:?}",
            far.position
        );
        assert_eq!(far.position[2], 12.0, "on the navmesh: {:?}", far.position);
        let near = run([200.0, -2300.0, 64.0], &mut collider);
        let radius = physics::CharacterShape::PLAYER.radius;
        assert!(
            near.position[1] <= 100.0 - radius + 0.5,
            "{:?}",
            near.position
        );
        assert!(
            near.position[2].abs() < 1.0,
            "on the floor: {:?}",
            near.position
        );
    }

    #[test]
    fn walkers_are_moved_by_their_controller_and_a_wall_stops_them_till_they_are_stuck() {
        let mut collider = floor_and_wall();
        let mut w = Walker::at(FormId(1), [0.0, 0.0, 0.0], 0.0, 1.0, false);
        w.set_path(
            vec![[0.0, 0.0, 0.0], [0.0, 300.0, 0.0]],
            0.0,
            false,
            &MoveSettings::defaults(),
        );
        let dt = 1.0 / 60.0;
        let mut frames = 0;
        while frames < 600 && w.stuck_at.is_none() {
            step(&mut w, 85.0, dt);
            move_body(
                &mut w,
                &mut collider,
                &[],
                &mut Vec::new(),
                &Ground {
                    mesh: &NavMesh::default(),
                    camera: None,
                },
                dt,
            );
            frames += 1;
        }
        assert!(w.body.is_some(), "collision under them: a controller");
        // Never through the wall: the capsule's radius short of it.
        let radius = physics::CharacterShape::PLAYER.radius;
        assert!(w.position[1] <= 100.0 - radius + 0.5, "{:?}", w.position);
        assert!(w.position[1] > 100.0 - radius - 5.0, "{:?}", w.position);
        // Stuck 1.5 s after reaching it (`009e4cf0`): the walk failed there.
        assert!(w.stuck_at.is_some());
        let reached = ((100.0 - radius) / 85.0 / dt) as i32;
        assert!(
            (frames - reached - 90).abs() <= 10,
            "{frames} frames, wall reached after {reached}"
        );
        assert!(!w.on_path());
    }

    #[test]
    fn walkers_go_round_each_other_not_through() {
        let mut collider = floor_and_wall();
        let mut w = Walker::at(FormId(1), [0.0, -300.0, 0.0], 0.0, 1.0, false);
        w.set_path(
            vec![[0.0, -300.0, 0.0], [0.0, 0.0, 0.0]],
            0.0,
            false,
            &MoveSettings::defaults(),
        );
        // Someone standing on the line.
        let other = (
            FormId(2),
            physics::Person {
                feet: [5.0, -150.0, 0.0],
                radius: 20.25,
                height: 128.0,
            },
        );
        let dt = 1.0 / 60.0;
        let mut closest = f32::MAX;
        for _ in 0..300 {
            step(&mut w, 85.0, dt);
            move_body(
                &mut w,
                &mut collider,
                &[other],
                &mut Vec::new(),
                &Ground {
                    mesh: &NavMesh::default(),
                    camera: None,
                },
                dt,
            );
            closest = closest.min((w.position[0] - 5.0).hypot(w.position[1] + 150.0));
        }
        // Two octagonal hulls: flat against flat their centres are 2 ×
        // (20.25 × cos 22.5° + the 0.7 convex radius) apart.
        assert!(closest >= 2.0 * 19.4 - 0.5, "{closest}");
    }

    /// A square of navmesh, (0,0)–(200,200), two triangles.
    fn square_mesh() -> NavMesh {
        use world::ai::NavTriangle;
        NavMesh {
            vertices: vec![
                [0.0, 0.0, 0.0],
                [200.0, 0.0, 0.0],
                [200.0, 200.0, 0.0],
                [0.0, 200.0, 0.0],
            ],
            triangles: vec![
                NavTriangle {
                    vertices: [0, 1, 2],
                    neighbors: [None, None, Some(1)],
                    linked: 0b100,
                    ..Default::default()
                },
                NavTriangle {
                    vertices: [0, 2, 3],
                    neighbors: [Some(0), None, None],
                    linked: 0b001,
                    ..Default::default()
                },
            ],
            ..Default::default()
        }
    }

    #[test]
    fn a_path_asked_of_the_path_manager_comes_back_later_and_is_walked() {
        let mesh = std::sync::Arc::new(square_mesh());
        let mut queue = PathQueue::default();
        let mut w = Walker::at(FormId(7), [20.0, 20.0, 0.0], 0.0, 1.0, false);
        let then = Pending {
            id: 0,
            radius: 10.0,
            turn_first: false,
            arrival: Some(1.0),
            path_target: None,
            partial: false,
            note: "test".into(),
        };
        ask_path(&mut queue, &mesh, &mut w, [180.0, 170.0, 0.0], then);
        // Waiting: no path yet.
        assert!(w.pending.is_some() && !w.on_path());
        let mut came = HashMap::new();
        for _ in 0..400 {
            came = queue.take_done();
            if !came.is_empty() {
                break;
            }
            std::thread::sleep(std::time::Duration::from_millis(5));
        }
        let (id, found, failure) = came.remove(&FormId(7)).unwrap().pop().unwrap();
        assert_eq!(Some(id), w.pending.as_ref().map(|p| p.id));
        path_came(&mut w, found, failure, &MoveSettings::defaults());
        assert!(w.pending.is_none() && w.on_path());
        assert_eq!(w.path.last(), Some(&[180.0, 170.0, 0.0]));
        assert_eq!((w.radius, w.arrival), (10.0, Some(1.0)));
    }

    #[test]
    fn walking_people_hand_the_bodies_they_touch_to_the_clutter() {
        let mut collider = floor_and_wall();
        let mut w = Walker::at(FormId(1), [0.0, -300.0, 0.0], 0.0, 1.0, false);
        // A clutter box, its face 25 in front of the walker.
        let (h, y) = (15.0, -260.0);
        let corners: Vec<[f32; 3]> = (0..8)
            .map(|k| {
                [
                    if k & 1 == 0 { -h } else { h },
                    y + if k & 2 == 0 { -h } else { h },
                    if k & 4 == 0 { 0.0 } else { 2.0 * h },
                ]
            })
            .collect();
        let faces: [[u32; 3]; 12] = [
            [0, 2, 1],
            [1, 2, 3],
            [4, 5, 6],
            [5, 7, 6],
            [0, 1, 4],
            [1, 5, 4],
            [2, 6, 3],
            [3, 6, 7],
            [0, 4, 2],
            [2, 4, 6],
            [1, 3, 5],
            [3, 7, 5],
        ];
        collider.add_layered(&corners, &faces, (0.0, 77, physics::NO_MATERIAL), None, 4);
        w.set_path(
            vec![[0.0, -300.0, 0.0], [0.0, 0.0, 0.0]],
            0.0,
            false,
            &MoveSettings::defaults(),
        );
        let dt = 1.0 / 60.0;
        let mut movers = Vec::new();
        // The first frame makes the controller; then they walk and push.
        for _ in 0..12 {
            step(&mut w, 85.0, dt);
            move_body(
                &mut w,
                &mut collider,
                &[],
                &mut movers,
                &Ground {
                    mesh: &NavMesh::default(),
                    camera: None,
                },
                dt,
            );
        }
        // The clutter box ahead (reference 77, layer 4) is touched; its
        // face is toward the walker (-y), who closes on it at the walk's
        // speed.
        assert!(!movers.is_empty());
        let t = movers.last().unwrap();
        assert_eq!(t.reference, 77);
        assert!(t.normal[1] < -0.5, "{:?}", t.normal);
        assert!(t.velocity[1] > 0.0, "{:?}", t.velocity);
    }

    #[test]
    fn a_new_path_starts_with_a_turn_in_place() {
        let s = MoveSettings::defaults();
        let mut w = Walker::at(FormId(1), [0.0, 0.0, 0.0], 0.0, 1.0, false);
        // Due east: a quarter turn first, in place, at 135°/s.
        w.set_path(vec![[0.0, 0.0, 0.0], [100.0, 0.0, 0.0]], 0.0, true, &s);
        assert!(w.turn.active);
        let mut t = 0.0f32;
        while w.turn.active {
            assert!(step(&mut w, 85.0, 0.01));
            assert_eq!(w.position, [0.0, 0.0, 0.0]);
            t += 0.01;
        }
        assert!((t - 0.67).abs() < 0.03, "{t}");
        assert!(step(&mut w, 85.0, 0.1));
        assert!(w.position[0] > 8.0);
    }

    #[test]
    fn facing_someone_turns_in_place_at_the_game_rate() {
        let s = MoveSettings::defaults();
        let mut w = Walker::at(FormId(1), [0.0, 0.0, 0.0], 0.0, 1.0, false);
        // Half a second at 135°/s: 67.5° of the 90°.
        for _ in 0..50 {
            face(&mut w, std::f32::consts::FRAC_PI_2, 0.01, false, &s);
        }
        assert!(
            (w.heading.to_degrees() - 67.5).abs() < 1.5,
            "{}",
            w.heading.to_degrees()
        );
        assert_eq!(w.turning, Some(TurnSide::Right));
    }

    #[test]
    fn the_game_matrix_faces_the_heading() {
        let mut w = walker();
        w.heading = std::f32::consts::FRAC_PI_2;
        let m = w.game_matrix();
        // Forward (+y) points east.
        assert!((m[4] - 1.0).abs() < 1e-6 && m[5].abs() < 1e-6);
    }

    #[test]
    fn reset_ai_discards_the_old_path_without_moving_the_actor() {
        let mut w = walker();
        let position = w.position;
        let heading = w.heading;
        w.package = Some(FormId(42));
        w.package_kind = Some(world::ai::kinds::TRAVEL);
        w.set_path(
            vec![position, [1000.0, 0.0, 0.0]],
            0.0,
            true,
            &MoveSettings::defaults(),
        );
        w.reset_procedure();
        assert!(!w.on_path());
        assert!(w.package.is_none());
        assert!(w.target.is_none());
        assert!(w.evaluate);
        assert_eq!(w.position, position);
        assert_eq!(w.heading, heading);
    }

    #[test]
    fn queued_package_change_rethinks_a_seated_actor_before_furniture_returns() {
        use testdata::ai::ids as fixture;

        let data = testdata::ai::world("seated-package-rethink");
        let game = cellview::Game::open(
            data.path(),
            &cellview::Options {
                official: true,
                ..default()
            },
        )
        .unwrap();
        let actor = FormId(fixture::TALKER_REF);
        let chair = FormId(fixture::CHAIR_REF);
        let mut state = world::scripting::GameState::default();
        state
            .script_packages
            .insert(actor, FormId(fixture::TO_MARKER));
        state.evaluate.insert(actor);
        state.furniture.insert(actor, chair);
        let mut pick = |_, _, _| None;
        let marker = world::furniture::PlacedMarker {
            index: 2,
            number: 14,
            position: [0.0, 0.0, 0.0],
            heading: 0.0,
        };
        state.sitters.insert(
            actor,
            world::furniture::Sitter::seated(
                chair,
                marker,
                world::furniture::MarkerSettings::default(),
                1.0,
                &mut pick,
            ),
        );
        let mut walker = Walker::at(actor, [0.0; 3], 0.0, 1.0, false);
        walker.package = Some(FormId(fixture::TO_CHAIR));
        walker.package_kind = Some(world::ai::kinds::TRAVEL);
        let mut life = Life::default();
        let mut seats = Seats::new(&game.order);
        let mesh = world::ai::NavMesh::load(&game.order, FormId(fixture::CELL));
        let moves = MoveSettings::defaults();
        let mut ctx = Ctx {
            game: &game,
            state: &mut state,
            seats: &mut seats,
            mesh: &mesh,
            moves: &moves,
            now: 1.0,
            dt: 0.1,
            fighting: false,
            talking: true,
        };

        assert!(rethink_queued_package_before_furniture(
            &mut ctx,
            &mut walker,
            &mut life,
            12.0,
            &mut Asking {
                queue: &mut PathQueue::default(),
                mesh: Default::default(),
            },
        ));
        let sitter = ctx.state.sitters.get(&actor).unwrap();
        assert!(sitter.stand_requested);
        assert!(life.getting_up);
        assert_eq!(walker.package, Some(FormId(fixture::TO_CHAIR)));
        assert!(!walker.evaluate);
        assert!(!ctx.state.evaluate.contains(&actor));
        assert!((walker.clock.timer - 19.9).abs() < 1e-5);

        // The still-pending chair exit must not cause another package check
        // or tick this frame after the furniture procedure releases the actor.
        assert!(!rethink_queued_package_before_furniture(
            &mut ctx,
            &mut walker,
            &mut life,
            12.0,
            &mut Asking {
                queue: &mut PathQueue::default(),
                mesh: Default::default(),
            },
        ));
        assert!((walker.clock.timer - 19.9).abs() < 1e-5);
    }

    #[test]
    fn queued_package_change_waits_for_furniture_entry_or_exit_to_finish() {
        use testdata::ai::ids as fixture;

        let data = testdata::ai::world("seated-package-rethink-gate");
        let game = cellview::Game::open(
            data.path(),
            &cellview::Options {
                official: true,
                ..default()
            },
        )
        .unwrap();
        let actor = FormId(fixture::TALKER_REF);
        let chair = FormId(fixture::CHAIR_REF);
        let mut state = world::scripting::GameState::default();
        state
            .script_packages
            .insert(actor, FormId(fixture::TO_MARKER));
        state.evaluate.insert(actor);
        state.furniture.insert(actor, chair);
        let mut pick = |_, _, _| None;
        let marker = world::furniture::PlacedMarker {
            index: 2,
            number: 14,
            position: [0.0, 0.0, 0.0],
            heading: 0.0,
        };
        let mut sitter = world::furniture::Sitter::seated(
            chair,
            marker,
            world::furniture::MarkerSettings::default(),
            1.0,
            &mut pick,
        );
        sitter.state = SitState::WantToStand;
        state.sitters.insert(actor, sitter);
        let mut walker = Walker::at(actor, [0.0; 3], 0.0, 1.0, false);
        walker.package = Some(FormId(fixture::TO_CHAIR));
        walker.package_kind = Some(world::ai::kinds::TRAVEL);
        let mut life = Life::default();
        let mut seats = Seats::new(&game.order);
        let mesh = world::ai::NavMesh::load(&game.order, FormId(fixture::CELL));
        let moves = MoveSettings::defaults();
        let mut ctx = Ctx {
            game: &game,
            state: &mut state,
            seats: &mut seats,
            mesh: &mesh,
            moves: &moves,
            now: 1.0,
            dt: 0.1,
            fighting: false,
            talking: false,
        };

        assert!(!rethink_queued_package_before_furniture(
            &mut ctx,
            &mut walker,
            &mut life,
            12.0,
            &mut Asking {
                queue: &mut PathQueue::default(),
                mesh: Default::default(),
            },
        ));
        assert!(ctx.state.evaluate.contains(&actor));
        assert!(!ctx.state.sitters.get(&actor).unwrap().stand_requested);
        assert_eq!(walker.clock, PackageClock::default());
    }
    /// The package fixture's world, opened.
    fn package_game(tag: &str) -> (testdata::TempData, cellview::Game) {
        let data = testdata::packages::world(tag);
        let game = cellview::Game::open(
            data.path(),
            &cellview::Options {
                official: true,
                ..default()
            },
        )
        .unwrap();
        (data, game)
    }

    fn with_package(walker: &mut Walker, package: u32, kind: u8) {
        walker.package = Some(FormId(package));
        walker.package_kind = Some(kind);
    }

    fn package_events(state: &world::scripting::GameState) -> Vec<(u32, PackageActionKind)> {
        state
            .events
            .iter()
            .filter_map(|e| match e {
                world::scripting::Event::PackageAction { package, kind, .. } => {
                    Some((package.0, *kind))
                }
                _ => None,
            })
            .collect()
    }

    use world::scripting::PackageActionKind;

    #[test]
    fn a_guard_runs_to_its_heading_marker_and_turns_to_it() {
        use testdata::packages::ids::*;
        let (_data, game) = package_game("guard-frame");
        let me = FormId(GUARD_REF);
        let mut state = world::scripting::GameState::default();
        let mut seats = Seats::new(&game.order);
        let mesh = world::ai::NavMesh::load(&game.order, FormId(CELL));
        let moves = MoveSettings::defaults();
        let mut life = Life::default();
        let mut walker = Walker::at(me, [100.0, 100.0, 0.0], 0.0, 1.0, false);
        with_package(&mut walker, GUARD_POST, world::ai::kinds::GUARD);
        let mut ctx = Ctx {
            game: &game,
            state: &mut state,
            seats: &mut seats,
            mesh: &mesh,
            moves: &moves,
            now: 1.0,
            dt: 0.1,
            fighting: false,
            talking: false,
        };
        guard_frame(&mut ctx, &mut walker, &mut life);
        // To the marker (800, 800), path radius max(0 × 0.5, 15), running
        // (radius 0: any distance left).
        assert!(walker.on_path());
        assert_eq!(walker.radius, 15.0);
        assert!(walker.run);
        let end = *walker.path.last().unwrap();
        assert!((end[0] - 800.0).abs() < 1e-3 && (end[1] - 800.0).abs() < 1e-3);
        // There: off the path, walking again, turning to the marker's 90°.
        walker.position = [795.0, 800.0, 0.0];
        walker.clear_path();
        guard_frame(&mut ctx, &mut walker, &mut life);
        assert!(!walker.on_path());
        assert!(!walker.run);
        let h = walker.facing.expect("a turn to the marker's heading");
        assert!((h - std::f32::consts::FRAC_PI_2).abs() < 1e-4, "{h}");
    }

    #[test]
    fn a_guard_near_its_editor_location_walks_back_from_where_it_was_moved() {
        use testdata::packages::ids::*;
        let (_data, game) = package_game("guard-editor-frame");
        let me = FormId(GUARD_REF);
        let mut state = world::scripting::GameState::default();
        // A script moved them to (700, 700).
        state.positions.insert(me, ([700.0, 700.0, 0.0], 0.0));
        let mut seats = Seats::new(&game.order);
        let mesh = world::ai::NavMesh::load(&game.order, FormId(CELL));
        let moves = MoveSettings::defaults();
        let mut life = Life::default();
        let mut walker = Walker::at(me, [700.0, 700.0, 0.0], 0.0, 1.0, false);
        with_package(&mut walker, GUARD_EDITOR, world::ai::kinds::GUARD);
        let mut ctx = Ctx {
            game: &game,
            state: &mut state,
            seats: &mut seats,
            mesh: &mesh,
            moves: &moves,
            now: 1.0,
            dt: 0.1,
            fighting: false,
            talking: false,
        };
        guard_frame(&mut ctx, &mut walker, &mut life);
        let end = *walker.path.last().expect("a path back");
        assert!((end[0] - 100.0).abs() < 1e-3 && (end[1] - 100.0).abs() < 1e-3);
        assert!(walker.run);
        // Back at the editor location: no heading to turn to.
        walker.position = [100.0, 100.0, 0.0];
        walker.clear_path();
        guard_frame(&mut ctx, &mut walker, &mut life);
        assert!(!walker.on_path());
        assert_eq!(walker.facing, None);
    }

    #[test]
    fn a_flee_package_with_nowhere_to_go_ends_once_and_they_stand() {
        use testdata::packages::ids::*;
        let (_data, game) = package_game("flee-frame");
        let me = FormId(GUARD_REF);
        let mut state = world::scripting::GameState::default();
        let mut seats = Seats::new(&game.order);
        let mesh = world::ai::NavMesh::load(&game.order, FormId(CELL));
        let moves = MoveSettings::defaults();
        let mut walker = Walker::at(me, [100.0, 100.0, 0.0], 0.0, 1.0, false);
        with_package(&mut walker, FLEE_NOWHERE, world::ai::kinds::FLEE);
        walker.set_path(
            vec![[100.0, 100.0, 0.0], [500.0, 100.0, 0.0]],
            0.0,
            false,
            &moves,
        );
        let mut ctx = Ctx {
            game: &game,
            state: &mut state,
            seats: &mut seats,
            mesh: &mesh,
            moves: &moves,
            now: 1.0,
            dt: 0.1,
            fighting: false,
            talking: false,
        };
        flee_frame(&mut ctx, &mut walker);
        flee_frame(&mut ctx, &mut walker);
        assert!(!walker.on_path());
        assert_eq!(
            package_events(ctx.state),
            [(FLEE_NOWHERE, PackageActionKind::End)]
        );
        // With a place: run there.
        let mut walker = Walker::at(me, [100.0, 100.0, 0.0], 0.0, 1.0, false);
        with_package(&mut walker, FLEE_TO_MARKER, world::ai::kinds::FLEE);
        flee_frame(&mut ctx, &mut walker);
        assert!(walker.on_path() && walker.run);
        let end = *walker.path.last().unwrap();
        assert!((end[0] + 800.0).abs() < 1e-3 && (end[1] + 800.0).abs() < 1e-3);
        assert_eq!(walker.radius, 256.0);
    }

    #[test]
    fn a_new_package_begins_once_and_a_travel_already_there_ends() {
        use testdata::packages::ids::*;
        let (_data, game) = package_game("package-begin-end");
        let me = FormId(GUARD_REF);
        let mut state = world::scripting::GameState::default();
        state.script_packages.insert(me, FormId(WITH_ACTIONS));
        let mut seats = Seats::new(&game.order);
        let mesh = world::ai::NavMesh::load(&game.order, FormId(CELL));
        let moves = MoveSettings::defaults();
        let mut life = Life::default();
        // Standing at the travel's place already (800, 800).
        let mut walker = Walker::at(me, [800.0, 800.0, 0.0], 0.0, 1.0, false);
        let mut ctx = Ctx {
            game: &game,
            state: &mut state,
            seats: &mut seats,
            mesh: &mesh,
            moves: &moves,
            now: 1.0,
            dt: 0.1,
            fighting: false,
            talking: false,
        };
        rethink(
            &mut ctx,
            &mut walker,
            &mut life,
            &mut Asking {
                queue: &mut PathQueue::default(),
                mesh: Default::default(),
            },
        );
        assert_eq!(
            package_events(ctx.state),
            [
                (WITH_ACTIONS, PackageActionKind::Begin),
                (WITH_ACTIONS, PackageActionKind::End)
            ]
        );
        // Looked at again: the same package goes on, nothing more.
        rethink(
            &mut ctx,
            &mut walker,
            &mut life,
            &mut Asking {
                queue: &mut PathQueue::default(),
                mesh: Default::default(),
            },
        );
        assert_eq!(package_events(ctx.state).len(), 2);
        // A package `AddScriptPackage` already began isn't begun again.
        ctx.state.events.clear();
        ctx.state.script_packages.insert(me, FormId(GUARD_POST));
        ctx.state
            .package_begun
            .insert(me, (FormId(GUARD_POST), false));
        rethink(
            &mut ctx,
            &mut walker,
            &mut life,
            &mut Asking {
                queue: &mut PathQueue::default(),
                mesh: Default::default(),
            },
        );
        assert_eq!(walker.package, Some(FormId(GUARD_POST)));
        assert!(package_events(ctx.state).is_empty());
    }

    #[test]
    fn a_far_place_is_walked_toward_as_far_as_the_attached_cells_go() {
        use testdata::long_paths::ids::*;
        let data = testdata::long_paths::long_paths("viewer-long-walk");
        let game = cellview::Game::open(
            data.path(),
            &cellview::Options {
                official: true,
                ..default()
            },
        )
        .unwrap();
        let order = &game.order;
        let me = FormId(WALKER_REF);
        let world = FormId(WORLD);
        let mut state = world::scripting::GameState::new(order);
        let mut seats = Seats::new(order);
        let moves = MoveSettings::defaults();
        let mut infos = world::ai::navinfo::NavInfos::default();
        let mut walker = Walker::at(me, [1000.0, 1000.0, 0.0], 0.0, 1.0, false);
        walker.package = Some(FormId(TRAVEL));
        walker.package_kind = Some(world::ai::kinds::TRAVEL);
        let cells = |n: i32| -> Vec<FormId> { (0..n).map(|i| FormId(CELL + i as u32)).collect() };
        // Squares 0 and 1 attached: to square 1's navmesh (its rough
        // position, the route's last attached node), short of the marker.
        let near: Vec<(i32, i32)> = vec![(0, 0), (1, 0)];
        let mesh = std::sync::Arc::new(NavMesh::load_cells(order, &cells(2)));
        let mut queue = PathQueue::default();
        // The path manager's answer, walked.
        let answer = |queue: &PathQueue, walker: &mut Walker| {
            for _ in 0..400 {
                let mut came = queue.take_done();
                if let Some(list) = came.remove(&walker.reference) {
                    for (_, found, failure) in list {
                        path_came(walker, found, failure, &MoveSettings::defaults());
                    }
                    return;
                }
                std::thread::sleep(std::time::Duration::from_millis(5));
            }
        };
        let mut ctx = Ctx {
            game: &game,
            state: &mut state,
            seats: &mut seats,
            mesh: &mesh,
            moves: &moves,
            now: 1.0,
            dt: 0.1,
            fighting: false,
            talking: false,
        };
        let attached: HashSet<(i32, i32)> = near.iter().copied().collect();
        long_walk(
            &mut ctx,
            &mut walker,
            &mut infos,
            (world, &near),
            &attached,
            300.0,
            &mut Asking {
                queue: &mut queue,
                mesh: mesh.clone(),
            },
        );
        answer(&queue, &mut walker);
        assert!(walker.on_path() && walker.partial);
        let end = *walker.path.last().unwrap();
        assert!(distance(end, [6144.0, 2048.0, 0.0]) < 1.0, "{end:?}");
        // Standing there: the same squares give the same way, not planned
        // again.
        walker.clear_path();
        long_walk(
            &mut ctx,
            &mut walker,
            &mut infos,
            (world, &near),
            &attached,
            300.0,
            &mut Asking {
                queue: &mut queue,
                mesh: mesh.clone(),
            },
        );
        assert!(!walker.on_path() && walker.pending.is_none());
        // Every square attached: on to the marker, the travel's end.
        let all: Vec<(i32, i32)> = (0..SQUARES).map(|x| (x, 0)).collect();
        let all_mesh = std::sync::Arc::new(NavMesh::load_cells(order, &cells(SQUARES)));
        ctx.mesh = &*all_mesh;
        let attached: HashSet<(i32, i32)> = all.iter().copied().collect();
        long_walk(
            &mut ctx,
            &mut walker,
            &mut infos,
            (world, &all),
            &attached,
            300.0,
            &mut Asking {
                queue: &mut queue,
                mesh: all_mesh.clone(),
            },
        );
        answer(&queue, &mut walker);
        assert!(walker.on_path() && !walker.partial);
        let end = *walker.path.last().unwrap();
        assert!(distance(end, [19000.0, 3000.0, 0.0]) < 1.0, "{end:?}");
    }

    /// B10: people greeted too often. While one greets the player nobody
    /// else greets (the player is being spoken to, +0x6cc, `008eeec0`), and
    /// after a greeting nobody greets the player for `fHelloCooldownTime`
    /// (`008bc520`/`008bc560`, the player's update at `00944179`); the one
    /// who greeted waits `fAIGreetingTimer` after their line.
    #[test]
    fn one_greeting_at_a_time_and_none_till_the_cooldown_is_over() {
        use testdata::ai::ids as fixture;
        let data = testdata::ai::world("greeting-cooldown");
        let game = cellview::Game::open(
            data.path(),
            &cellview::Options {
                official: true,
                ..default()
            },
        )
        .unwrap();
        // The player between the greeter (0, 0) and Chatty (0, 200); the
        // talk quest running (start-game enabled).
        let mut state = world::scripting::GameState {
            player_cell: Some(FormId(fixture::CELL)),
            player_position: Some([0.0, 100.0, 0.0]),
            ..world::scripting::GameState::new(&game.order)
        };
        assert!(state.running.contains(&FormId(fixture::QUEST)));
        let mut seats = Seats::new(&game.order);
        let mesh = world::ai::NavMesh::load(&game.order, FormId(fixture::CELL));
        let moves = Moves::new(&game);
        let settings = MoveSettings::defaults();
        let person = |r: u32, at: [f32; 3]| {
            let mut w = Walker::at(FormId(r), at, 0.0, 1.0, false);
            w.social = Some(world::social::Social::new(&moves.social, &mut || 0.5));
            w.detected_player = 50;
            w
        };
        let mut greeter_walker = person(fixture::GREETER_REF, [0.0, 0.0, 0.0]);
        let mut chatty = person(fixture::CHATTY_REF, [0.0, 200.0, 0.0]);
        let (mut chats, mut lines) = (Chats::default(), Lines::default());
        let mut cooldown = world::social::HelloCooldown::default();
        let mut frame = |state: &mut world::scripting::GameState,
                         walker: &mut Walker,
                         lines: &mut Lines,
                         cooldown: &mut world::social::HelloCooldown,
                         now_ms: u32| {
            cooldown.update(now_ms, &moves.social);
            let mut ctx = Ctx {
                game: &game,
                state,
                seats: &mut seats,
                mesh: &mesh,
                moves: &settings,
                now: now_ms as f32 / 1000.0,
                dt: 0.1,
                fighting: false,
                talking: false,
            };
            let mut greeter = Greeter {
                interior: true,
                cooldown,
                now_ms,
                line_speaker: None,
                player_trespassing: false,
                player_in_combat: false,
            };
            social_frame(
                &mut ctx,
                walker,
                &mut chats,
                lines,
                &moves,
                &[],
                &mut greeter,
            );
        };
        // The greeter says "Hi there." to the player.
        frame(
            &mut state,
            &mut greeter_walker,
            &mut lines,
            &mut cooldown,
            1_000,
        );
        assert!(lines.greeting(FormId(fixture::GREETER_REF)));
        assert!(lines.spoken_to(PLAYER_REF));
        assert!(!cooldown.free());
        // Chatty, on the same frame: the player is being spoken to.
        frame(&mut state, &mut chatty, &mut lines, &mut cooldown, 1_000);
        assert!(!lines.is_saying(FormId(fixture::CHATTY_REF)));
        assert_eq!(chatty.social.as_ref().unwrap().greeting, 0.0);
        // The line over, still within the cooldown: nobody greets, but
        // Chatty, ready to, looks at the player (its ACTION slot,
        // `008bc3d0`) and holds the look.
        let mut lines = Lines::default();
        frame(&mut state, &mut chatty, &mut lines, &mut cooldown, 20_000);
        assert_eq!(chatty.social.as_ref().unwrap().greeting, 0.0);
        assert!(lines.queue.is_empty());
        assert_eq!(chatty.head_track.in_slot(Slot::Action), Some(PLAYER_REF));
        // B33: 30 s after the greeting the cooldown is over, but someone
        // asked to look at someone isn't near (`CanSetActionHeadTrackTarget`,
        // `00901460`): no greeting while the look holds.
        frame(&mut state, &mut chatty, &mut lines, &mut cooldown, 31_001);
        assert_eq!(chatty.social.as_ref().unwrap().greeting, 0.0);
        assert!(cooldown.free());
        // The head-track update lets the player go (`008a3100` step 4, the
        // player lost from sight); seen again, Chatty greets (it has no
        // line for the player, but its timer and the cooldown are set all
        // the same).
        chatty.head_track.clear_all();
        frame(&mut state, &mut chatty, &mut lines, &mut cooldown, 32_000);
        assert_eq!(chatty.social.as_ref().unwrap().greeting, 20.0);
        assert!(!cooldown.free());
        // The greeter's own timer was set by its greeting.
        assert_eq!(greeter_walker.social.as_ref().unwrap().greeting, 20.0);
    }

    /// B12: leaving Doc Mitchell's door conversation started another at
    /// once, forever: the forced look at the package when the menu closed
    /// restarted the dialogue package, which talked again in that update.
    /// In the game the same package goes on (`0090a1a0`), restored at the
    /// step it was saved at (`00913250`), which talking to the player made
    /// DONE (`005fa330`); the menu opens an update after the conversation
    /// package is made (`008b19c0`, `008e9640`).
    #[test]
    fn a_dialogue_package_talks_once() {
        use testdata::ai::ids as fixture;
        let data = testdata::ai::world("dialogue-package-close");
        let game = cellview::Game::open(
            data.path(),
            &cellview::Options {
                official: true,
                ..default()
            },
        )
        .unwrap();
        let me = FormId(fixture::TALKER_REF);
        // The player stands next to the talker (-500, 0).
        let mut state = world::scripting::GameState {
            player_cell: Some(FormId(fixture::CELL)),
            player_position: Some([-450.0, 0.0, 0.0]),
            ..Default::default()
        };
        let mut seats = Seats::new(&game.order);
        let mesh = world::ai::NavMesh::load(&game.order, FormId(fixture::CELL));
        let moves = Moves::new(&game);
        let settings = MoveSettings::defaults();
        let mut life = Life::default();
        let mut walker = Walker::at(me, [-500.0, 0.0, 0.0], 0.0, 1.0, false);
        let (mut chats, mut lines, mut talk) = (
            Chats::default(),
            Lines::default(),
            crate::scripts::ScriptedTalk::default(),
        );
        let mut ctx = Ctx {
            game: &game,
            state: &mut state,
            seats: &mut seats,
            mesh: &mesh,
            moves: &settings,
            now: 1.0,
            dt: 0.1,
            fighting: false,
            talking: false,
        };
        let mut queue = PathQueue::default();
        let mut look = |ctx: &mut Ctx, walker: &mut Walker, life: &mut Life| {
            rethink(
                ctx,
                walker,
                life,
                &mut Asking {
                    queue: &mut queue,
                    mesh: Default::default(),
                },
            )
        };
        look(&mut ctx, &mut walker, &mut life);
        assert_eq!(walker.package, Some(FormId(fixture::TALK_PACKAGE)));
        // The conversation package is made on one update; the menu opens on
        // the next.
        let mut update =
            |ctx: &mut Ctx, walker: &mut Walker, talk: &mut crate::scripts::ScriptedTalk| {
                dialogue_frame(ctx, walker, (&mut chats, &mut lines, talk), &moves)
            };
        update(&mut ctx, &mut walker, &mut talk);
        assert!(talk.0.is_none());
        update(&mut ctx, &mut walker, &mut talk);
        assert_eq!(talk.0.map(|t| t.0), Some(me));
        // The menu is open: the package waits for it (`move_actors` runs no
        // more of it).
        assert!(walker.talk_run.waiting);
        // The player leaves: `EndDialogue`, then the forced look at the
        // package; on that same update no new conversation starts.
        talk.0 = None;
        walker.talk_run.conversation_over();
        look(&mut ctx, &mut walker, &mut life);
        assert_eq!(walker.package, Some(FormId(fixture::TALK_PACKAGE)));
        // The package was finished when it activated the player
        // (`005fa330`: saved at DONE, its end action): it doesn't talk
        // again, on this update or later.
        assert!(walker.talk_run.waiting && walker.talk_run.done);
        assert!(walker.ended);
        for _ in 0..3 {
            if !walker.talk_run.waiting {
                update(&mut ctx, &mut walker, &mut talk);
            }
            assert!(talk.0.is_none(), "a new conversation");
        }
        // A different package (Doc's sandbox, once VCG01 is completed)
        // starts anew.
        ctx.state
            .script_packages
            .insert(me, FormId(fixture::TO_MARKER));
        look(&mut ctx, &mut walker, &mut life);
        assert_eq!(walker.package_kind, Some(world::ai::kinds::TRAVEL));
        assert!(!walker.talk_run.travelled && !walker.talk_run.waiting);
    }
}

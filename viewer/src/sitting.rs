//! People using furniture, picking idles and sandboxing, the game's way
//! (`world::furniture`, `world::idles`, `world::sandbox`, read from the
//! game's code). Each frame `ai::move_actors` hands every person here:
//!
//! - Furniture ([`furniture_frame`]): someone heading for a chair walks to
//!   its marker, turns to the marker's heading, and the sit procedure runs
//!   (the entry animation, moving them by its root, then the seated loop);
//!   getting up plays the exit the same way. Whoever the state has in
//!   furniture without that (a script's `Activate`, a loaded game) is
//!   seated at once.
//! - Idles ([`idles_frame`]): once a second of free time near the player,
//!   the idle tree is asked (`IdleClock`) and what it gives plays over
//!   everything.
//! - Sandbox packages ([`sandbox_frame`]): what to do next is chosen by
//!   the game's weights (`Sandbox::choose`), for the game's durations, and
//!   carried out (see [`Activity`]).

use std::collections::HashMap;
use std::sync::Arc;

use bevy::prelude::{Component, Resource};
use esm::{FormId, LoadOrder};
use world::ai::NavMesh;
use world::furniture::{self, Animations, MarkerSettings, SitState, Sitter, Step};
use world::idles::{procedures, Idle, IdleAsker, IdleClock, IdleMarker, IdleQuestion, IdleTree};
use world::sandbox::{self, activities, Candidate, Choice, Kind, Nearby, Sandbox};
use world::scripting::{Facts, GameState};

use crate::actors::ActorRig;
use crate::ai::{face, Chats, Walker};

/// The idle tree, and what's been read for sitting: animations, furniture
/// markers, marker settings, the sandbox settings.
#[derive(Resource)]
pub struct Seats {
    pub tree: IdleTree,
    roots: HashMap<String, Arc<Vec<FormId>>>,
    sequences: HashMap<String, Option<Arc<nif::Sequence>>>,
    markers: HashMap<FormId, Arc<Vec<nif::FurnitureMarker>>>,
    /// Whether each piece of furniture's model (by base) has collision.
    collides: HashMap<FormId, bool>,
    marker_settings: HashMap<u8, MarkerSettings>,
    pub sandbox: sandbox::Settings,
    /// Each cell's references a sandbox could use ([`Seats::candidates`]).
    candidates: HashMap<FormId, Arc<Vec<Usable>>>,
}

/// A reference placed in a cell that a sandbox could use, as its records
/// have it (none of which changes): its kind, and whether a child may use
/// it.
#[derive(Debug, Clone, Copy)]
struct Usable {
    reference: FormId,
    kind: Kind,
    child_can_use: bool,
}

impl Seats {
    pub fn new(order: &LoadOrder) -> Seats {
        Seats {
            tree: IdleTree::load(order),
            roots: HashMap::new(),
            sequences: HashMap::new(),
            markers: HashMap::new(),
            collides: HashMap::new(),
            marker_settings: HashMap::new(),
            sandbox: sandbox::Settings::read(order),
            candidates: HashMap::new(),
        }
    }

    /// A cell's references a sandbox could use (a kind, not marked
    /// ignored), read from their records once: the scan otherwise reads
    /// thousands of records each time someone in a sandbox looks around.
    fn candidates(&mut self, order: &LoadOrder, cell: FormId) -> Arc<Vec<Usable>> {
        self.candidates
            .entry(cell)
            .or_insert_with(|| {
                Arc::new(
                    order
                        .references_in_cell(cell)
                        .into_iter()
                        .filter(|rr| !rr.entry.header.is_deleted())
                        .map(|rr| rr.form_id)
                        .filter(|&r| !sandbox::ignored(order, r))
                        .filter_map(|r| {
                            Some(Usable {
                                reference: r,
                                kind: sandbox::kind_of(order, r)?,
                                child_can_use: sandbox::child_can_use(order, r),
                            })
                        })
                        .collect(),
                )
            })
            .clone()
    }

    /// An animation (a `.kf` under `meshes\`), read once.
    pub fn sequence(&mut self, game: &cellview::Game, model: &str) -> Option<Arc<nif::Sequence>> {
        load(game, &mut self.sequences, model)
    }

    /// Keep the same cached Arc on reload: special-idle request rejection
    /// compares sequence identity, not just its embedded display name.
    pub fn sequence_path(&self, sequence: &Arc<nif::Sequence>) -> Option<String> {
        self.sequences.iter().find_map(|(path, cached)| {
            cached
                .as_ref()
                .filter(|other| Arc::ptr_eq(sequence, other))
                .map(|_| path.clone())
        })
    }

    /// A piece of furniture's markers (by its base), read once.
    fn markers(&mut self, game: &cellview::Game, base: FormId) -> Arc<Vec<nif::FurnitureMarker>> {
        self.markers
            .entry(base)
            .or_insert_with(|| {
                Arc::new(
                    preview::furniture::markers(&game.assets, &game.order, base)
                        .unwrap_or_default(),
                )
            })
            .clone()
    }

    /// Whether a placed piece of furniture's model has collision (by its
    /// reference; read once per base): `Sitter::furniture_collides`.
    fn collides(&mut self, game: &cellview::Game, furniture_ref: FormId) -> bool {
        let Some(base) = world::scripting::base_of(&game.order, furniture_ref) else {
            return false;
        };
        *self
            .collides
            .entry(base)
            .or_insert_with(|| preview::furniture::has_collision(&game.assets, &game.order, base))
    }

    fn marker_settings(&mut self, order: &LoadOrder, number: u8) -> MarkerSettings {
        *self
            .marker_settings
            .entry(number)
            .or_insert_with(|| MarkerSettings::read(order, number))
    }

    /// The idle tree's roots for a skeleton.
    fn roots(&mut self, skeleton: &str) -> Arc<Vec<FormId>> {
        let tree = &self.tree;
        self.roots
            .entry(skeleton.to_ascii_lowercase())
            .or_insert_with(|| Arc::new(tree.roots_for(skeleton)))
            .clone()
    }
}

fn load(
    game: &cellview::Game,
    sequences: &mut HashMap<String, Option<Arc<nif::Sequence>>>,
    model: &str,
) -> Option<Arc<nif::Sequence>> {
    sequences
        .entry(model.to_ascii_lowercase())
        .or_insert_with(|| crate::viewmodel::sequence(game, model).map(Arc::new))
        .clone()
}

/// A placed piece of furniture's markers where it stands, and its `MNAM`,
/// from markers already read (`markers`: by base).
fn placed_markers(
    order: &LoadOrder,
    state: &GameState,
    markers: &HashMap<FormId, Arc<Vec<nif::FurnitureMarker>>>,
    furniture_ref: FormId,
) -> Option<(Vec<furniture::PlacedMarker>, u32)> {
    let base = world::scripting::base_of(order, furniture_ref)?;
    let m = markers.get(&base).filter(|m| !m.is_empty())?;
    let (_, _, at, heading) = state.place(order, furniture_ref)?;
    Some((
        furniture::place_markers(m, at, heading, reference_scale(order, furniture_ref)),
        furniture::marker_flags(order, base),
    ))
}

/// The animations' lengths and root travel, for the sit procedure.
struct Anims<'a> {
    game: &'a cellview::Game,
    sequences: &'a mut HashMap<String, Option<Arc<nif::Sequence>>>,
}

impl Animations for Anims<'_> {
    fn length(&mut self, model: &str) -> Option<f32> {
        let s = load(self.game, self.sequences, model)?;
        Some(s.stop - s.start)
    }

    fn root_offset(&mut self, model: &str, time: f32) -> [f32; 3] {
        load(self.game, self.sequences, model)
            .and_then(|s| s.root_offset(time))
            .unwrap_or([0.0; 3])
    }
}

/// A placed reference's scale (`XSCL`, else 1).
fn reference_scale(order: &LoadOrder, reference: FormId) -> f32 {
    order
        .get(reference)
        .and_then(|rr| rr.record().ok())
        .and_then(|r| {
            r.get(esm::FourCC::new(b"XSCL"))
                .filter(|s| s.data.len() >= 4)
                .map(|s| f32::from_le_bytes([s.data[0], s.data[1], s.data[2], s.data[3]]))
        })
        .unwrap_or(1.0)
}

/// Random numbers from one draw of the game state's dice.
struct Dice(u64);

impl Dice {
    fn next(&mut self) -> u64 {
        // xorshift64, as the state's.
        let mut x = self.0.max(1);
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.0 = x;
        x
    }
}

/// What a sandboxing person is doing for their current choice. Sitting
/// and sleeping are the game's (the furniture's sit procedure). Wandering
/// is the wander procedure (`008ed420`): a spot 32 to 0.75 × the area's
/// radius from its middle (how the game's navmesh search picks within that
/// ring isn't traced: here a random point of a random navmesh triangle
/// reaching into it), walked to within 50, then a pause by the wander timer
/// (`sandbox::WanderTimer`). An idle marker is walked to, faced and its
/// idles played one after another, in order or at random (`00901c70`).
/// Eating is the eat procedure (`008e3140`): food lying in the area is
/// walked to and taken, carried food used otherwise; a free chair nearby is
/// found and sat in, and seated they eat one, then ask for an eating idle
/// every 1–6 s; without a chair they eat where they stand. Talking starts a
/// conversation with the one chosen (`00901a20`, `ai::start_chat`).
#[derive(Debug, Clone, PartialEq)]
pub enum Activity {
    Furniture,
    Wander {
        timer: sandbox::WanderTimer,
    },
    IdleMarker {
        heading: f32,
        idles: IdleMarker,
        next: usize,
    },
    Eat(Eating),
    Talk {
        with: FormId,
    },
}

/// The eat procedure's progress.
#[derive(Debug, Clone, PartialEq)]
pub struct Eating {
    /// The food lying in the world to take first.
    pub food: Option<FormId>,
    /// The item they eat, once they hold it.
    pub item: Option<FormId>,
    /// The chair to eat in, once looked for.
    pub chair: Option<FormId>,
    pub searched: bool,
    pub eaten: bool,
    /// Seconds to the next eating idle.
    pub idle_in: f32,
}

/// What a person keeps for sitting, idles and sandboxing.
#[derive(Component, Default)]
pub struct Life {
    /// Their skeleton (for the idle tree's roots) and bound radius (for
    /// how near the player their idles play), once read.
    pub skeleton: Option<(String, f32)>,
    pub idles: IdleClock,
    /// A base loop the idle tree gave while standing.
    pub base_idle: Option<Arc<nif::Sequence>>,
    pub sandbox: Option<Sandbox>,
    pub activity: Option<Activity>,
    /// Getting up to go elsewhere: the package is looked at again once
    /// they're up.
    pub getting_up: bool,
    /// Talking to the player: the speaking emotion and the idle requests
    /// the dialogue menu makes for them ([`dialogue_frame`]).
    pub talk: world::talk_idles::Talking,
}

/// What every person's frame needs.
pub struct Ctx<'a> {
    pub game: &'a cellview::Game,
    pub state: &'a mut GameState,
    pub seats: &'a mut Seats,
    pub mesh: &'a NavMesh,
    /// Turning and walking settings (`world::movement`).
    pub moves: &'a world::movement::MoveSettings,
    pub now: f32,
    pub dt: f32,
    /// In a fight (the fast exits, no idles).
    pub fighting: bool,
    /// Saying a line now (`IsTalking`).
    pub talking: bool,
}

/// The idle tree's view of someone, with `GetSitting`, `GetSleeping` and
/// `GetFurnitureMarkerID` as given.
fn question(
    walker: &Walker,
    life: &Life,
    order: &LoadOrder,
    (talking, fighting): (bool, bool),
    (sitting, sleeping, marker): (u8, u8, u8),
) -> IdleQuestion {
    let walking = walker.next < walker.path.len();
    let procedure = match &life.activity {
        _ if fighting => procedures::COMBAT,
        // A sandbox: by where it is and what it's doing, as the game's
        // `GetCurrentAIProcedure` reads it (`world::more_functions::
        // procedures::sandbox_number`).
        _ if life.sandbox.is_some() => {
            let sandbox = life.sandbox.as_ref().expect("checked");
            let phase = match sandbox.phase {
                world::sandbox::Phase::Init => 0,
                world::sandbox::Phase::GettingUp => 1,
                world::sandbox::Phase::Moving => 2,
                world::sandbox::Phase::Doing => 3,
            };
            world::more_functions::procedures::sandbox_number(
                phase,
                sandbox.choice.as_ref().map(|c| c.activity),
            )
        }
        Some(Activity::Wander { .. }) => procedures::WANDER,
        Some(Activity::Eat(_)) => procedures::EAT,
        Some(Activity::Talk { .. }) => procedures::DIALOGUE,
        Some(Activity::Furniture) if sleeping > 0 => procedures::SLEEP,
        _ if walking => procedures::TRAVEL,
        _ => procedures::NONE,
    };
    let used_item = match &life.activity {
        Some(Activity::Eat(e)) => e
            .item
            .or_else(|| e.food.and_then(|f| world::scripting::base_of(order, f))),
        _ => None,
    };
    IdleQuestion {
        sitting,
        sleeping,
        marker,
        procedure,
        moving: walking,
        talking,
        alerted: fighting,
        last_idle: life.idles.last,
        used_item,
        female: walker.female,
        player: false,
        child: world::idles::is_child(order, walker.reference),
        first_person: false,
        menu: None,
        emotion: life.talk.dialogue_emotion(),
        // Set by the caller for a hit, and from the body's flags.
        hit_location: None,
        sneaking: false,
        running: false,
        greeting_player: false,
    }
}

/// Someone's skeleton and bound radius: the radius is half their bounds'
/// (`OBND`) diagonal (the game uses its 3D's bound radius; taking the
/// record's bounds for it is a guess), 64 without.
fn skeleton_of(order: &LoadOrder, reference: FormId) -> (String, f32) {
    let base = world::scripting::base_of(order, reference);
    let skeleton = base
        .and_then(|b| world::actor_look(order, b))
        .map(|l| l.skeleton)
        .unwrap_or_default();
    let radius = base
        .and_then(|b| order.get(b))
        .and_then(|rr| rr.record().ok())
        .and_then(|r| {
            r.get(esm::FourCC::new(b"OBND"))
                .filter(|s| s.data.len() >= 12)
                .map(|s| {
                    let v = |i: usize| {
                        f32::from(i16::from_le_bytes([s.data[i * 2], s.data[i * 2 + 1]]))
                    };
                    let d = [v(3) - v(0), v(4) - v(1), v(5) - v(2)];
                    0.5 * (d[0] * d[0] + d[1] * d[1] + d[2] * d[2]).sqrt()
                })
        })
        .filter(|r| *r > 1.0)
        .unwrap_or(64.0);
    (skeleton, radius)
}

fn skeleton(life: &mut Life, order: &LoadOrder, reference: FormId) -> (String, f32) {
    life.skeleton
        .get_or_insert_with(|| skeleton_of(order, reference))
        .clone()
}

/// The idle the tree gives someone now, as (idle, `.kf`).
#[allow(clippy::too_many_arguments)]
fn pick_idle(
    order: &LoadOrder,
    state: &GameState,
    tree: &IdleTree,
    roots: &[FormId],
    walker: &Walker,
    life: &Life,
    flags: (bool, bool),
    values: (u8, u8, u8),
    seed: u64,
) -> Option<Idle> {
    let about = question(walker, life, order, flags, values);
    pick_for(
        order,
        state,
        tree,
        roots,
        walker.reference,
        life,
        about,
        seed,
    )
}

/// The idle the tree gives someone asked as `about`.
#[allow(clippy::too_many_arguments)]
fn pick_for(
    order: &LoadOrder,
    state: &GameState,
    tree: &IdleTree,
    roots: &[FormId],
    who: FormId,
    life: &Life,
    about: IdleQuestion,
    seed: u64,
) -> Option<Idle> {
    let facts = Facts {
        order,
        state,
        speaker: None,
    };
    let asker = IdleAsker::new(who, about, Some(&facts), seed);
    tree.evaluate(roots, &|i| asker.passes(i), &|id| life.idles.is_delayed(id))
        .cloned()
}

/// Starts using a placed piece of furniture: the nearest free marker its
/// `MNAM` allows is theirs (the state's `IsCurrentFurnitureRef` from now
/// on), and they walk to it. False when there's no marker free or no way
/// there.
pub fn begin_use(ctx: &mut Ctx, walker: &mut Walker, furniture_ref: FormId) -> bool {
    let me = walker.reference;
    let game = ctx.game;
    let order = &game.order;
    let Some(base) = world::scripting::base_of(order, furniture_ref) else {
        return false;
    };
    ctx.seats.markers(ctx.game, base);
    let Some((placed, flags)) = placed_markers(order, ctx.state, &ctx.seats.markers, furniture_ref)
    else {
        return false;
    };
    let taken = furniture::taken_markers(ctx.state, furniture_ref, me);
    let Some(marker) =
        furniture::nearest_free(&placed, flags, |i| taken.contains(&i), walker.position)
    else {
        return false;
    };
    let settings = ctx.seats.marker_settings(order, marker.number);
    let mut sitter = Sitter::new(
        furniture_ref,
        marker,
        settings,
        walker.position,
        walker.heading,
    );
    sitter.furniture_collides = ctx.seats.collides(game, furniture_ref);
    if sitter.in_reach(walker.position) {
        walker.clear_path();
    } else {
        match crate::ai::path_for(ctx.mesh, walker, marker.position) {
            Some(path) => walker.set_path(path, 0.0, true, ctx.moves),
            None => return false,
        }
    }
    println!(
        "{me} heads for {furniture_ref} (marker {}, number {})",
        marker.index, marker.number
    );
    walker.target = Some(marker.position);
    ctx.state.furniture.insert(me, furniture_ref);
    ctx.state.sitters.insert(me, sitter);
    true
}

/// Seated at once where the state has them (the game's "already seated"
/// path): the nearest free usable marker, else the first. False if the
/// furniture has none (they're let go).
fn seat_at_once(
    ctx: &mut Ctx,
    walker: &mut Walker,
    life: &mut Life,
    furniture_ref: FormId,
) -> bool {
    let me = walker.reference;
    let game = ctx.game;
    let order = &game.order;
    if let Some(base) = world::scripting::base_of(order, furniture_ref) {
        ctx.seats.markers(ctx.game, base);
    }
    let taken = furniture::taken_markers(ctx.state, furniture_ref, me);
    let marker = placed_markers(order, ctx.state, &ctx.seats.markers, furniture_ref).and_then(
        |(p, flags)| {
            furniture::nearest_free(&p, flags, |i| taken.contains(&i), walker.position)
                .or_else(|| p.first().copied())
        },
    );
    let Some(marker) = marker else {
        ctx.state.stand(me);
        return false;
    };
    let sitter = seated_sitter(ctx, walker, life, furniture_ref, marker);
    println!(
        "{me} is seated at {:.0},{:.0},{:.0} (marker {}){}",
        sitter.position[0],
        sitter.position[1],
        sitter.position[2],
        marker.number,
        if sitter.dynamic_idle.is_some() {
            ""
        } else {
            " (no seated idle)"
        }
    );
    walker.position = sitter.position;
    walker.heading = sitter.heading;
    walker.clear_path();
    ctx.state.sitters.insert(me, sitter);
    true
}

/// The pass the game makes over everyone loaded once a cell has loaded
/// (`00972d30`; the rule is `world::furniture::seat_on_load`): someone
/// whose package sends them to a piece of furniture in their cell, on
/// none yet, is seated there at once by the instant sit (`0088d2f0`) —
/// at the seat, facing the marker's heading plus its heading delta, the
/// seated loop loaded (sit state 1, then 4), the marker occupied. Without
/// a seated loop the game gives up ("%s went to sit at %s and had no
/// animation", state back to 0) and so does this. `candidates`: the
/// package's target, then its location's reference. True if seated.
pub fn seat_on_load(
    ctx: &mut Ctx,
    walker: &mut Walker,
    life: &mut Life,
    candidates: &[FormId],
) -> bool {
    let me = walker.reference;
    let game = ctx.game;
    let order = &game.order;
    for &f in candidates {
        if let Some(base) = world::scripting::base_of(order, f) {
            ctx.seats.markers(game, base);
        }
    }
    let chosen = {
        let (state, markers) = (&*ctx.state, &ctx.seats.markers);
        furniture::seat_on_load(order, state, me, candidates, |f| {
            placed_markers(order, state, markers, f).map(|(p, _)| p)
        })
    };
    let Some((furniture_ref, marker)) = chosen else {
        return false;
    };
    let sitter = seated_sitter(ctx, walker, life, furniture_ref, marker);
    if sitter.dynamic_idle.is_none() {
        println!("{me} went to sit at {furniture_ref} and had no animation");
        return false;
    }
    println!(
        "{me} is seated in {furniture_ref} as the cell loads, at {:.0},{:.0},{:.0} (marker {})",
        sitter.position[0], sitter.position[1], sitter.position[2], marker.number
    );
    walker.position = sitter.position;
    walker.heading = sitter.heading;
    walker.clear_path();
    walker.target = None;
    ctx.state.furniture.insert(me, furniture_ref);
    ctx.state.sitters.insert(me, sitter);
    true
}

/// Someone settled in `marker` of a piece of furniture ([`Sitter::seated`]),
/// with the seated loop the idle tree gives them.
fn seated_sitter(
    ctx: &mut Ctx,
    walker: &Walker,
    life: &mut Life,
    furniture_ref: FormId,
    marker: furniture::PlacedMarker,
) -> Sitter {
    let me = walker.reference;
    let game = ctx.game;
    let order = &game.order;
    let (skeleton, _) = skeleton(life, order, me);
    let roots = ctx.seats.roots(&skeleton);
    let settings = ctx.seats.marker_settings(order, marker.number);
    let collides = ctx.seats.collides(game, furniture_ref);
    let seed = ctx.state.roll();
    let flags = (ctx.talking, ctx.fighting);
    let mut sitter = {
        let (state, tree) = (&*ctx.state, &ctx.seats.tree);
        let (w, l) = (walker, &*life);
        let mut pick = |sitting: u8, sleeping: u8, number: u8| {
            pick_idle(
                order,
                state,
                tree,
                &roots,
                w,
                l,
                flags,
                (sitting, sleeping, number),
                seed,
            )
            .map(|i| (i.form_id, i.model))
        };
        Sitter::seated(furniture_ref, marker, settings, walker.scale, &mut pick)
    };
    sitter.furniture_collides = collides;
    sitter
}

/// The player using furniture (`world::furniture`, the game's own code
/// for the player): what E activated, and the temporary third-person view
/// the game gives while sitting down and getting up.
#[derive(Resource, Default)]
pub struct PlayerSeat {
    /// Furniture the player activated this frame (`scripts::use_object`).
    /// (The temporary third-person view is the camera's,
    /// `player_camera::PlayerView`.)
    pub activated: Option<FormId>,
    /// The heading last given to the camera, so looking around while
    /// seated isn't undone.
    applied_heading: Option<f32>,
    /// The first-person seated loop given to the view (its `.kf`).
    first_person_loop: Option<String>,
    /// The tree was asked for that loop since the view came back.
    loop_asked: bool,
    /// The procedure turned the player by the marker's delta or half a
    /// turn this frame (`Sitter::skip_next_blend`): the third-person
    /// body's animations switch without a blend (`player_body`).
    pub skip_next_blend: bool,
}

/// What the idle tree's furniture branch asks about the player: the sit
/// values, and whether the view is first person (`IsPC1stPerson`).
fn player_question((sitting, sleeping, marker): (u8, u8, u8), first_person: bool) -> IdleQuestion {
    IdleQuestion {
        sitting,
        sleeping,
        marker,
        procedure: procedures::NONE,
        player: true,
        first_person,
        ..IdleQuestion::default()
    }
}

/// The idle the tree gives the player now, as (idle, `.kf`).
fn pick_player_idle(
    order: &LoadOrder,
    state: &GameState,
    tree: &IdleTree,
    roots: &[FormId],
    values: (u8, u8, u8),
    first_person: bool,
    seed: u64,
) -> Option<(FormId, String)> {
    let facts = Facts {
        order,
        state,
        speaker: None,
    };
    let asker = IdleAsker::new(
        world::dialogue::PLAYER_REF,
        player_question(values, first_person),
        Some(&facts),
        seed,
    );
    tree.evaluate(roots, &|i| asker.passes(i), &|_| false)
        .map(|i| (i.form_id, i.model.clone()))
}

/// The player's skeleton (for the idle tree's roots).
fn player_skeleton(order: &LoadOrder) -> String {
    world::actor_look(order, world::dialogue::PLAYER_BASE)
        .map(|l| l.skeleton)
        .unwrap_or_default()
}

/// E on furniture (`TESFurniture::Activate`, `world::furniture::activate`):
/// up if in furniture, else the nearest free usable marker is theirs, the
/// temporary third-person view begins and the sit procedure starts.
fn player_activates(
    game: &cellview::Game,
    state: &mut GameState,
    seats: &mut Seats,
    (seat, view): (&mut PlayerSeat, &mut crate::player_camera::PlayerView),
    furniture_ref: FormId,
    (feet, heading): ([f32; 3], f32),
) {
    use world::dialogue::PLAYER_REF;
    let order = &game.order;
    let current = state
        .sitters
        .get(&PLAYER_REF)
        .map_or(SitState::Normal, |s| s.state);
    let in_furniture = state.furniture.contains_key(&PLAYER_REF);
    let marker = world::scripting::base_of(order, furniture_ref).and_then(|base| {
        seats.markers(game, base);
        let (placed, flags) = placed_markers(order, state, &seats.markers, furniture_ref)?;
        let taken = furniture::taken_markers(state, furniture_ref, PLAYER_REF);
        furniture::nearest_free(&placed, flags, |i| taken.contains(&i), feet)
    });
    let current = if in_furniture && current == SitState::Normal {
        // Seated at once (no procedure yet): in furniture all the same.
        SitState::Sitting
    } else {
        current
    };
    match furniture::activate(current, marker) {
        furniture::Activation::StandUp => {
            view.force_temp_third();
            seat.loop_asked = false;
            if let Some(s) = state.sitters.get_mut(&PLAYER_REF) {
                s.stand_up();
            }
            println!("The player gets up from {furniture_ref}.");
        }
        furniture::Activation::NoMarker => {
            println!("{furniture_ref} has no free marker to use.");
        }
        // Beds take the sleep path before this (`scripts::use_object`).
        furniture::Activation::Sleep(_) => {}
        furniture::Activation::Sit(marker) => {
            view.force_temp_third();
            seat.loop_asked = false;
            let settings = seats.marker_settings(order, marker.number);
            let mut sitter = Sitter::new(furniture_ref, marker, settings, feet, heading);
            sitter.furniture_collides = seats.collides(game, furniture_ref);
            println!(
                "The player uses {furniture_ref} (marker {}, number {}).",
                marker.index, marker.number
            );
            state.furniture.insert(PLAYER_REF, furniture_ref);
            state.sitters.insert(PLAYER_REF, sitter);
            seat.applied_heading = None;
        }
    }
}

/// The player's furniture each frame, after walking and before the camera
/// tracks: E on furniture or, seated, E on nothing (the player update's
/// fallback, `0094076c`..`009407c8`: when nothing under the crosshair was
/// activated, the current furniture is, which gets them up); put on the
/// marker (`player_approach`) and the sit or stand procedure run as for
/// anyone (`Sitter::update`: the entry and exit from the idle tree, moving
/// the player by their root); the view's heading and pitch limit
/// (`clamp_pitch`) follow; once the temporary view ends seated, the
/// first-person seated loop the tree gives (`1stP_ChairDynamicIdle.kf`,
/// its `Camera1st` track placing the seated eye) plays in the view.
/// Someone the state has in furniture with no procedure (a loaded game) is
/// seated at once (`0088d2f0`). Not modelled: movement keys while seated
/// (their handling isn't traced; the player is held on the seat), the
/// player's seated idles (third person only), the HUD mode change
/// (`00771700`) and holstering.
#[allow(clippy::too_many_arguments)]
pub fn player_furniture(
    time: bevy::prelude::Res<bevy::prelude::Time>,
    keys: bevy::prelude::Res<bevy::prelude::ButtonInput<bevy::prelude::KeyCode>>,
    game: bevy::prelude::Res<crate::GameFiles>,
    mut state: bevy::prelude::ResMut<crate::dialogue::DialogueState>,
    (mut seats, mut seat, mut idle): (
        bevy::prelude::ResMut<Seats>,
        bevy::prelude::ResMut<PlayerSeat>,
        bevy::prelude::ResMut<crate::player_idle::PlayerIdle>,
    ),
    mut player: bevy::prelude::ResMut<crate::walk::Player>,
    (activatable, talk_target, conversation, menus): (
        bevy::prelude::Res<crate::scripts::Activatable>,
        bevy::prelude::Res<crate::dialogue::TalkTarget>,
        bevy::prelude::Res<crate::dialogue::Conversation>,
        bevy::prelude::Res<crate::menus::Menus>,
    ),
    (doors, crosshair, mut view): (
        bevy::prelude::Res<crate::walk::Doors>,
        bevy::prelude::Res<crate::crosshair::Crosshair>,
        bevy::prelude::ResMut<crate::player_camera::PlayerView>,
    ),
    mut cameras: bevy::prelude::Query<(&mut bevy::prelude::Transform, &mut crate::FlyCamera)>,
) {
    use bevy::prelude::*;
    use world::dialogue::PLAYER_REF;
    if !player.walking || !player.ready {
        return;
    }
    let Ok((mut transform, mut camera)) = cameras.single_mut() else {
        return;
    };
    let game = &game.0;
    let order = &game.order;
    let state = &mut state.0;
    let feet = player.character.feet;
    let heading = (-camera.yaw).rem_euclid(std::f32::consts::TAU);
    // E on furniture this frame.
    let view = &mut *view;
    if let Some(f) = seat.activated.take() {
        player_activates(
            game,
            state,
            &mut seats,
            (&mut seat, view),
            f,
            (feet, heading),
        );
    }
    // Seated, E on nothing usable (no door, person or object): the
    // current furniture is activated, so they get up. Only with the
    // crosshair's use allowed, as everything E does here.
    let seated = state
        .sitters
        .get(&PLAYER_REF)
        .is_some_and(|s| s.state == SitState::Sitting && s.playing.is_none());
    if seated
        && keys.just_pressed(KeyCode::KeyE)
        && activatable.0.is_none()
        && talk_target.0.is_none()
        && conversation.0.is_none()
        && !menus.is_open()
        && !state.controls_off[world::scripting::controls::ROLLOVER]
    {
        let door = crate::walk::door_in_view(&doors.0, &crosshair);
        if door.is_none() {
            if let Some(&f) = state.furniture.get(&PLAYER_REF) {
                player_activates(
                    game,
                    state,
                    &mut seats,
                    (&mut seat, view),
                    f,
                    (feet, heading),
                );
            }
        }
    }
    // In furniture with no procedure (a loaded game, a script): seated at
    // once, in first person.
    if let Some(&f) = state.furniture.get(&PLAYER_REF) {
        if !state.sitters.contains_key(&PLAYER_REF) {
            let marker = world::scripting::base_of(order, f).and_then(|base| {
                seats.markers(game, base);
                let (placed, flags) = placed_markers(order, state, &seats.markers, f)?;
                let taken = furniture::taken_markers(state, f, PLAYER_REF);
                furniture::nearest_free(&placed, flags, |i| taken.contains(&i), feet)
                    .or_else(|| placed.first().copied())
            });
            match marker {
                Some(marker) => {
                    let settings = seats.marker_settings(order, marker.number);
                    let roots = seats.roots(&player_skeleton(order));
                    let seed = state.roll();
                    let collides = seats.collides(game, f);
                    let mut sitter = {
                        let (snapshot, tree) = (&*state, &seats.tree);
                        let mut pick = |sitting: u8, sleeping: u8, number: u8| {
                            pick_player_idle(
                                order,
                                snapshot,
                                tree,
                                &roots,
                                (sitting, sleeping, number),
                                true,
                                seed,
                            )
                        };
                        Sitter::seated(f, marker, settings, 1.0, &mut pick)
                    };
                    sitter.furniture_collides = collides;
                    state.sitters.insert(PLAYER_REF, sitter);
                    seat.applied_heading = None;
                    view.camera.temp_third = furniture::TempThirdPerson::default();
                    seat.loop_asked = false;
                }
                None => state.stand(PLAYER_REF),
            }
        }
    }
    let dt = if menus.is_open() {
        0.0
    } else {
        time.delta_secs()
    };
    let Some(mut sitter) = state.sitters.remove(&PLAYER_REF) else {
        // Up: the view comes back to first person once nothing plays.
        view.update_temp_third(false);
        if seat.first_person_loop.take().is_some() {
            idle.set_seated_loop(game, None);
        }
        seat.applied_heading = None;
        return;
    };
    // On the way: put on the marker (the player doesn't walk there).
    if sitter.state == SitState::Normal {
        sitter.position = feet;
        sitter.heading = heading;
        furniture::player_approach(&mut sitter);
    }
    let before = sitter.state;
    // `IsPC1stPerson` as the first-person view being the one wanted (which
    // of the camera's flags the condition reads isn't traced).
    let first_person = view.first_person_wanted();
    let roots = seats.roots(&player_skeleton(order));
    let asks =
        sitter.state == SitState::Normal || (sitter.state.is_settled() && sitter.stand_requested);
    let seed = if asks { state.roll() } else { 0 };
    let step = {
        let snapshot = &*state;
        let Seats {
            tree, sequences, ..
        } = &mut *seats;
        let tree = &*tree;
        let mut pick = |sitting: u8, sleeping: u8, number: u8| {
            pick_player_idle(
                order,
                snapshot,
                tree,
                &roots,
                (sitting, sleeping, number),
                first_person,
                seed,
            )
        };
        let mut anims = Anims { game, sequences };
        sitter.update(dt, false, &mut pick, &mut anims)
    };
    if sitter.state != before {
        println!(
            "Player: {} -> {}{}",
            before.name(),
            sitter.state.name(),
            sitter
                .playing
                .as_ref()
                .map(|p| format!(", playing {}", p.model))
                .unwrap_or_default()
        );
    }
    // Getting up: the first-person loop stops as the third-person view
    // takes over.
    if view.camera.temp_third.active && seat.first_person_loop.take().is_some() {
        idle.set_seated_loop(game, None);
    }
    // The body where the procedure has it; the camera turned with it when
    // the procedure turns them.
    player.character = physics::Character::new(sitter.position);
    if seat.applied_heading != Some(sitter.heading) {
        camera.yaw = -sitter.heading;
        seat.applied_heading = Some(sitter.heading);
    }
    let max_down = world::scripting::game_setting(order, "fSittingMaxLookingDown").unwrap_or(40.0);
    camera.pitch = -furniture::clamp_pitch(-camera.pitch, sitter.state, max_down);
    transform.rotation = Quat::from_euler(EulerRot::YXZ, camera.yaw, camera.pitch, 0.0);
    let [x, y, z] = sitter.position;
    transform.translation = Vec3::from(cellview::space::point([x, y, z + cellview::EYE_HEIGHT]));
    let switch_back = view.update_temp_third(sitter.playing.is_some());
    if std::mem::take(&mut sitter.skip_next_blend) {
        seat.skip_next_blend = true;
    }
    match step {
        Step::Released | Step::Failed => {
            if step == Step::Failed {
                println!("The player has no way to sit in {}.", sitter.furniture);
            }
            state.stand(PLAYER_REF);
            seat.applied_heading = None;
            if seat.first_person_loop.take().is_some() {
                idle.set_seated_loop(game, None);
            }
        }
        Step::Busy | Step::Settled => {
            // Back in first person seated: the tree's first-person seated
            // loop (asked at `GetSitting` 1 with `IsPC1stPerson`; which of
            // the player's two animation sets the game reloads it into on
            // the switch, `00950110` → `00951a10`, isn't traced).
            if switch_back {
                seat.loop_asked = false;
            }
            if !seat.loop_asked && view.first_person_wanted() && sitter.state.is_settled() {
                seat.loop_asked = true;
                let seed = state.roll();
                let found = pick_player_idle(
                    order,
                    state,
                    &seats.tree,
                    &roots,
                    (1, 0, sitter.marker.number),
                    true,
                    seed,
                );
                let model = found.map(|(_, m)| m);
                if model != seat.first_person_loop {
                    let seq = model.as_deref().and_then(|m| seats.sequence(game, m));
                    idle.set_seated_loop(game, seq);
                    seat.first_person_loop = model;
                }
            }
            state.sitters.insert(PLAYER_REF, sitter);
        }
    }
}

/// Lets go of a sandbox choice that couldn't be carried out: its target
/// weighs nothing till the next scan, and something else is chosen.
/// Whether their special idle is still starting (`00498f80`, see
/// `world::animation::Player::idle_starting`): a requested one blending
/// in, or the tree's idle not yet playing or blending in. Only this holds
/// a seated actor who is to get up (`00921e80` cases 4 and 9); an idle
/// playing at full weight doesn't, however long it has left. `seq` is the
/// tree's idle's sequence, if one plays.
pub(crate) fn special_idle_starting(
    life: &Life,
    rig: &ActorRig,
    seq: Option<&Arc<nif::Sequence>>,
) -> bool {
    use world::animation::State;
    if rig.scripted_idle.is_some() {
        let slot = world::animation::slot(rig.idle_section);
        return matches!(
            rig.player.state(slot),
            Some(State::EaseIn | State::TransDest)
        );
    }
    match seq {
        Some(seq) if life.idles.playing.is_some() => {
            rig.player.idle_starting(rig.overlay_section, seq)
        }
        _ => false,
    }
}

/// The exit replaces whatever special idle they had (`00497ca0`: the
/// current one stopped with its own blend-out, `004994f0`, then the exit
/// queued): the tree's idle ends, and a requested one is freed.
fn replace_seated_idle(life: &mut Life, rig: &mut ActorRig) {
    life.idles.stop();
    if rig.scripted_idle.take().is_some() {
        let slot = world::animation::slot(rig.idle_section);
        if rig.player.playing(slot) == Some(world::animation::group::SPECIAL_IDLE) {
            rig.player.stop_section(slot);
        }
        rig.idle_section = world::animation::section::SPECIAL_IDLE;
    }
}

fn give_up(life: &mut Life) {
    if let Some(sb) = life.sandbox.as_mut() {
        sb.failed();
        sb.choice = None;
    }
    life.activity = None;
}

/// One frame of someone's furniture use: seated at once if the state has
/// them in furniture without the procedure; on the way, the turn to the
/// marker's heading; then the sit or stand procedure. Whether they're in
/// furniture now (sitting down, seated or getting up: not to walk or
/// fight). `rig` gets the seated loop and the entry or exit to play.
pub fn furniture_frame(
    ctx: &mut Ctx,
    walker: &mut Walker,
    life: &mut Life,
    rig: &mut ActorRig,
) -> bool {
    let me = walker.reference;
    let game = ctx.game;
    let order = &game.order;
    if let Some(&f) = ctx.state.furniture.get(&me) {
        if !ctx.state.sitters.contains_key(&me) && !seat_at_once(ctx, walker, life, f) {
            return false;
        }
    }
    let Some(mut sitter) = ctx.state.sitters.remove(&me) else {
        rig.dynamic_idle = life.base_idle.clone();
        return false;
    };
    if ctx.fighting {
        sitter.stand_up();
    }
    // On the way: walk the path, then turn to the marker's heading
    // (`00921350`), then sit.
    if sitter.state == SitState::Normal {
        rig.dynamic_idle = life.base_idle.clone();
        if walker.next < walker.path.len() {
            ctx.state.sitters.insert(me, sitter);
            return false;
        }
        if sitter.stand_requested {
            ctx.state.stand(me);
            return false;
        }
        if !sitter.in_reach(walker.position) {
            // No path (cleared when a script moved them): a new one.
            if walker.path.is_empty() {
                if let Some(path) = crate::ai::path_for(ctx.mesh, walker, sitter.marker.position) {
                    walker.set_path(path, 0.0, true, ctx.moves);
                    ctx.state.sitters.insert(me, sitter);
                    return false;
                }
            }
            println!(
                "{me} can't reach {} (at {:.0},{:.0},{:.0}, its marker at {:.0},{:.0},{:.0}; path {}/{})",
                sitter.furniture,
                walker.position[0],
                walker.position[1],
                walker.position[2],
                sitter.marker.position[0],
                sitter.marker.position[1],
                sitter.marker.position[2],
                walker.next,
                walker.path.len()
            );
            ctx.state.stand(me);
            give_up(life);
            return false;
        }
        // The turn to the marker's heading: in place, the game's turn
        // (`00921350` → `008bb5c0`).
        if face(walker, sitter.marker.heading, ctx.dt, false, ctx.moves) {
            ctx.state.sitters.insert(me, sitter);
            return true;
        }
        sitter.position = walker.position;
        sitter.heading = walker.heading;
    }
    // The procedure's step, the tree asked with its sit state.
    let (skeleton, _) = skeleton(life, order, me);
    let roots = ctx.seats.roots(&skeleton);
    let seated_seq = life
        .idles
        .playing
        .as_ref()
        .and_then(|p| ctx.seats.sequence(ctx.game, &p.model));
    let seated_idle = special_idle_starting(life, rig, seated_seq.as_ref());
    // The dice only when the tree will be asked (sitting down or getting
    // up), so a run repeats however fast frames come.
    let asks = sitter.state == SitState::Normal
        || (sitter.state.is_settled() && sitter.stand_requested && !seated_idle);
    let seed = if asks { ctx.state.roll() } else { 0 };
    let flags = (ctx.talking, ctx.fighting);
    let before = sitter.state;
    let step = {
        let state = &*ctx.state;
        let Seats {
            tree, sequences, ..
        } = &mut *ctx.seats;
        let tree = &*tree;
        let (w, l) = (&*walker, &*life);
        let mut pick = |sitting: u8, sleeping: u8, number: u8| {
            pick_idle(
                order,
                state,
                tree,
                &roots,
                w,
                l,
                flags,
                (sitting, sleeping, number),
                seed,
            )
            .map(|i| (i.form_id, i.model))
        };
        let mut anims = Anims {
            game: ctx.game,
            sequences,
        };
        sitter.update(ctx.dt, seated_idle, &mut pick, &mut anims)
    };
    if sitter.state != before {
        println!(
            "{:.1} s: {me}: {} -> {}{}",
            ctx.now,
            before.name(),
            sitter.state.name(),
            sitter
                .playing
                .as_ref()
                .map(|p| format!(", playing {}", p.model))
                .unwrap_or_default()
        );
    }
    // Getting up began: the exit takes the special idle's place, the seated
    // idle playing is stopped with its blend-out (`00921e80` case 4/9 →
    // `00497ca0` → `004994f0`).
    if before.is_settled() && !sitter.state.is_settled() {
        replace_seated_idle(life, rig);
    }
    walker.position = sitter.position;
    walker.heading = sitter.heading;
    // The heading jumped by the marker's delta or half a turn: the
    // animations switch in this frame without a blend (`cSkipNextBlend`,
    // `world::animation::Player::skip_next_blend`), as the body's turn in
    // the entry, exit and seated loop is meant to line up with it.
    if std::mem::take(&mut sitter.skip_next_blend) {
        rig.player.skip_next_blend();
    }
    match step {
        Step::Released | Step::Failed => {
            if step == Step::Failed {
                println!("{me} has no way to sit in {}", sitter.furniture);
                give_up(life);
            }
            ctx.state.stand(me);
            rig.dynamic_idle = life.base_idle.clone();
            rig.overlay = None;
            if life.getting_up {
                life.getting_up = false;
                walker.evaluate = true;
            }
            false
        }
        Step::Busy | Step::Settled => {
            rig.dynamic_idle = sitter
                .dynamic_idle
                .as_ref()
                .and_then(|(_, model)| ctx.seats.sequence(ctx.game, model));
            rig.overlay = sitter
                .playing
                .as_ref()
                .and_then(|p| Some((ctx.seats.sequence(ctx.game, &p.model)?, p.elapsed)));
            if rig.overlay.is_some() {
                rig.overlay_section = world::animation::section::SPECIAL_IDLE;
            }
            ctx.state.sitters.insert(me, sitter);
            true
        }
    }
}

/// One frame of someone's idles: the playing one goes on; free near the
/// player (not walking, fighting, sitting down or getting up), the tree is
/// asked once a second ([`IdleClock`]). What it gives plays over
/// everything (`rig.overlay`), unless sitting down or getting up plays.
/// Walking or a fight stops an idle (a guess: how the game cuts them
/// short isn't traced). At an idle marker, its idles play instead of the
/// tree's ([`sandbox_frame`]).
pub fn idles_frame(ctx: &mut Ctx, walker: &Walker, life: &mut Life, rig: &mut ActorRig) {
    // Leaving the dialogue menu frees the talking idle.
    free_talk_idle(life, rig);
    // The process does not ask for a free idle while a requested one
    // owns its animation (008dafd0). Its KF, not IdleClock, advances it.
    if rig.scripted_idle.is_some() {
        life.idles.stop();
        rig.overlay = None;
    }
    let me = walker.reference;
    let game = ctx.game;
    let order = &game.order;
    let walking = walker.next < walker.path.len();
    if ctx.fighting || walking {
        life.idles.stop();
    }
    life.idles.advance(ctx.dt);
    let sitter = ctx.state.sitters.get(&me);
    let values = sitter.map_or((0, 0, 0), |s| s.question());
    let entering = sitter.is_some_and(|s| s.playing.is_some() || !s.state.is_settled());
    let (skeleton, radius) = skeleton(life, order, me);
    let near = ctx.state.player_position.is_some_and(|p| {
        let d = (0..3)
            .map(|k| (p[k] - walker.position[k]).powi(2))
            .sum::<f32>();
        d.sqrt() <= world::idles::IDLE_ANIMATION_DISTANCE * radius / 64.0
    });
    let at_marker = matches!(life.activity, Some(Activity::IdleMarker { .. }));
    // A request the tree answers is played by the process at a later
    // update (`008dab40` queues it, `008dae00` plays it), after the
    // animation update that clears `cSkipNextBlend`: not in the frame the
    // furniture procedure swapped the animations without a blend (else
    // the idle would cut in unblended too).
    let free = near
        && !walking
        && !ctx.fighting
        && !entering
        && !life.getting_up
        && !at_marker
        && rig.scripted_idle.is_none()
        && !rig.player.skips_next_blend();
    if life.idles.due(ctx.dt, free) {
        let roots = ctx.seats.roots(&skeleton);
        let seed = ctx.state.roll();
        let flags = (ctx.talking, ctx.fighting);
        let found = pick_idle(
            order,
            ctx.state,
            &ctx.seats.tree,
            &roots,
            walker,
            life,
            flags,
            values,
            seed,
        );
        if let Some(idle) = found {
            let standing = sitter_is_none(ctx.state, me);
            play(ctx, life, &idle, standing);
        }
    }
    if !ctx
        .state
        .sitters
        .get(&me)
        .is_some_and(|s| s.playing.is_some())
    {
        rig.overlay = life
            .idles
            .playing
            .as_ref()
            .and_then(|p| Some((ctx.seats.sequence(ctx.game, &p.model)?, p.elapsed)));
        // In its record's section (`00498290`): the upper body's idles over
        // the legs' idle or walk, the movement section's holding the walk.
        rig.overlay_section = life
            .idles
            .playing
            .as_ref()
            .and_then(|p| ctx.seats.tree.get(p.idle))
            .map_or(world::animation::section::SPECIAL_IDLE, |i| i.group());
    }
    // What scripts ask about them (`world::more_functions`): walking
    // forward along their path (whether they run isn't told), the last idle
    // played, the procedure.
    let about = question(walker, life, order, (ctx.talking, ctx.fighting), values);
    world::more_functions::report(
        ctx.state,
        me,
        world::more_functions::Seen {
            movement: if walking {
                world::more_functions::movement::FORWARD
            } else {
                0
            },
            last_idle: life.idles.last,
            procedure: (about.procedure != procedures::NONE).then_some(about.procedure),
            swimming: false,
            idle_playing: rig.scripted_idle.is_some() || life.idles.playing.is_some(),
        },
    );
}

fn sitter_is_none(state: &GameState, me: FormId) -> bool {
    !state.sitters.contains_key(&me)
}

/// Someone's sit state number (`Actor` vfunc +0x214): their sit
/// procedure's, else seated (4) when the state has them in furniture.
pub(crate) fn sit_state(state: &GameState, me: FormId) -> u8 {
    match state.sitters.get(&me) {
        Some(s) => s.state.number(),
        None if state.furniture.contains_key(&me) => SitState::Sitting.number(),
        None => 0,
    }
}

/// The speaker in the dialogue menu, each frame (`world::talk_idles`,
/// `Actor::UpdateInDialogue`, Xbox PDB, `008a5580`): a newly said
/// response takes its emotion and asks for its speaker idle or the idle
/// tree's (the say, `008a20d0`); between responses the tree is asked
/// again whenever their special idle is done. The tree is asked as in the
/// menu: `MenuMode 1009`, `GetCurrentAIProcedure` 4 (their dialogue
/// package), `IsTalking` while the response is said, `GetDialogueEmotion`.
/// What it gives plays in the special-idle section on its own clock (the
/// script-idle path, `rig.scripted_idle`). An answer for the base loop
/// (section 0) isn't played here (how the request path places one isn't
/// traced). The process plays a taken request at its next update
/// (`008dae00`); here in the same frame, or later while one is starting.
#[allow(clippy::too_many_arguments)]
pub fn dialogue_frame(
    game: &cellview::Game,
    state: &mut GameState,
    seats: &mut Seats,
    walker: &Walker,
    life: &mut Life,
    rig: &mut ActorRig,
    said: Option<(world::talk_idles::SaidKey, &world::dialogue::Response)>,
    now: f32,
) {
    let me = walker.reference;
    // A taken idle waiting to play.
    if let Some(id) = life.talk.queued {
        if play_requested(game, state, seats, life, rig, (me, id), now) {
            life.talk.queued = None;
        }
    }
    let special_done = idle_done(rig) && life.talk.queued.is_none();
    let ask = said
        .and_then(|(key, r)| life.talk.say(key, r))
        .or_else(|| life.talk.between_says(special_done));
    let Some(ask) = ask else { return };
    // `IsTalking` (`005a1150` → `008a67f0`): actor +0x7d, which the menu's
    // update sets before the say (`008a5580`), so a response being said
    // counts from its first frame.
    let talking = said.is_some() || state.speaking.contains(&me);
    let menu = Asked {
        menu: true,
        talking,
        hit_location: None,
        greeting: false,
    };
    request_idle(game, state, seats, (walker, life, rig), ask, menu, now);
}

/// How the tree is asked for a request: in the dialogue menu (`MenuMode
/// 1009`, `GetCurrentAIProcedure` 4), saying a line (`IsTalking`), for a
/// hit (`GetHitLocation`).
#[derive(Debug, Clone, Copy, Default)]
pub struct Asked {
    pub menu: bool,
    pub talking: bool,
    pub hit_location: Option<i32>,
    /// Saying a GREET line to the player (`IsGreetingPlayer`: the
    /// process's greeting flag is taken as set while the GREET
    /// procedure's line to the player is said; when `008dbe30` sets and
    /// clears it isn't traced).
    pub greeting: bool,
}

/// Every frame: the idle requests of lines said outside the dialogue menu
/// and of hits taken.
///
/// Each response begun (`chatter`) is a say (`008a20d0`, through the GREET
/// procedure `008dbe30` or a conversation `009ee0a0`): the speaker asks for
/// the response's speaker idle, or the tree when the caller forces it (the
/// GREET procedure unless the speaker's package has idles; a conversation
/// as it was made); a listener who is a person (not the player) asks for
/// the listener idle, or the tree unless their package has idles
/// (`world::talk_idles::say_requests`).
///
/// A hit that doesn't kill asks the tree at once with the hit's body part
/// (`GetHitLocation`), when `bPlayHitLocationIdles` is on, the part known,
/// and `IgnoreCrippledLimbs` not set (`0089a760`); the tree's
/// `HitReactionIdles` answer plays in its section (the movement section's
/// `MT_HitTorso.kf` …). (`0089a760`'s other two conditions, its locals
/// 0x2bd/0x289 and 0x34d, aren't traced.)
#[allow(clippy::too_many_arguments)]
pub fn idle_requests(
    time: bevy::prelude::Res<bevy::prelude::Time>,
    game: bevy::prelude::Res<crate::GameFiles>,
    mut state: bevy::prelude::ResMut<crate::dialogue::DialogueState>,
    mut seats: bevy::prelude::ResMut<Seats>,
    lines: bevy::prelude::Res<crate::chatter::Lines>,
    conversation: bevy::prelude::Res<crate::dialogue::Conversation>,
    mut said_to: bevy::prelude::Local<Option<world::talk_idles::SaidKey>>,
    mut actors: bevy::prelude::Query<(&Walker, &mut Life, &mut ActorRig)>,
) {
    use world::talk_idles::{self, Ask, Request};
    let now = time.elapsed_secs();
    let game = &game.0;
    let order = &game.order;
    let state = &mut state.0;
    let hits = std::mem::take(&mut state.hits_taken);
    let has_idles = |w: &Walker| {
        w.package
            .is_some_and(|p| talk_idles::package_has_idles(order, p))
    };
    // A line said to the player without the menu (`SayTo`) is said
    // through the GREET procedure (`005c9100` → `008dbe30` → `008a20d0`):
    // each response begun asks for its speaker idle as a greeting's does
    // (Doc Mitchell's "Whoa, easy there" plays `VCG01DocWhoaThere`).
    let mut say_to = Vec::new();
    if let Some(talk) = conversation.0.as_ref().filter(|t| t.is_line_only()) {
        if let Some((key, response)) = talk.said() {
            if *said_to != Some(key) {
                *said_to = Some(key);
                say_to.push(crate::chatter::ResponseStarted {
                    speaker: talk.speaker(),
                    listener: world::dialogue::PLAYER_REF,
                    response: response.clone(),
                    conversation: None,
                });
            }
        }
    } else {
        *said_to = None;
    }
    for s in lines.started.iter().chain(&say_to) {
        let listener = (s.listener != s.speaker && s.listener != world::dialogue::PLAYER_REF)
            .then(|| {
                actors
                    .iter()
                    .find(|(w, _, _)| w.reference == s.listener)
                    .map(|(w, _, _)| has_idles(w))
            })
            .flatten();
        let Some(force) = actors
            .iter()
            .find(|(w, _, _)| w.reference == s.speaker)
            .map(|(w, _, _)| {
                s.conversation
                    .unwrap_or_else(|| talk_idles::greet_forces_tree(has_idles(w), None))
            })
        else {
            continue;
        };
        let in_combat = state.combat.contains_key(&s.speaker);
        let asks = talk_idles::say_requests(&s.response, force, in_combat, listener);
        for (who, ask, talking) in [
            (s.speaker, asks.speaker, true),
            (s.listener, asks.listener, false),
        ] {
            let Some(ask) = ask else { continue };
            if let Some((w, mut life, mut rig)) =
                actors.iter_mut().find(|(w, _, _)| w.reference == who)
            {
                let asked = Asked {
                    menu: false,
                    talking,
                    hit_location: None,
                    greeting: talking
                        && s.conversation.is_none()
                        && s.listener == world::dialogue::PLAYER_REF,
                };
                request_idle(
                    game,
                    state,
                    &mut seats,
                    (w, &mut life, &mut rig),
                    ask,
                    asked,
                    now,
                );
            }
        }
    }
    let on = game
        .settings
        .get("Combat", "bPlayHitLocationIdles")
        .is_none_or(|v| v.trim() != "0");
    for (who, part, killed) in hits {
        if killed || part < 0 || !on {
            continue;
        }
        let ignores = Facts {
            order,
            state,
            speaker: None,
        }
        .current_actor_value(who, 72)
        .is_some_and(|v| v > 0.0);
        if ignores {
            continue;
        }
        if let Some((w, mut life, mut rig)) = actors.iter_mut().find(|(w, _, _)| w.reference == who)
        {
            let ask = Ask {
                request: Request::Tree,
                forced: true,
            };
            let asked = Asked {
                menu: false,
                talking: false,
                hit_location: Some(part),
                greeting: false,
            };
            request_idle(
                game,
                state,
                &mut seats,
                (w, &mut life, &mut rig),
                ask,
                asked,
                now,
            );
        }
    }
}

/// Whether the requested idle is done: nothing of it plays in its section
/// (`004985f0`).
fn idle_done(rig: &ActorRig) -> bool {
    let slot = world::animation::slot(rig.idle_section);
    rig.player.playing(slot) != Some(world::animation::group::SPECIAL_IDLE)
}

/// An idle request (`008dab40`, process vtable +0x44): refused out of sit
/// states 0, 4 and 9, while the requested idle still plays unless an idle
/// is named or the request forced, while one is starting for the tree, and
/// for a running package with the "no idle anims" flag (`008dade0`); the
/// tree asked as `asked` says; taken, it plays at once (or as soon as no
/// idle is starting: `008dae00`).
pub fn request_idle(
    game: &cellview::Game,
    state: &mut GameState,
    seats: &mut Seats,
    (walker, life, rig): (&Walker, &mut Life, &mut ActorRig),
    ask: world::talk_idles::Ask,
    asked: Asked,
    now: f32,
) {
    use world::animation::State;
    use world::talk_idles::{takes_request, Request};
    let me = walker.reference;
    let order = &game.order;
    let sit = sit_state(state, me);
    let slot = world::animation::slot(rig.idle_section);
    let starting = matches!(
        rig.player.state(slot),
        Some(State::EaseIn | State::TransDest)
    ) && !idle_done(rig);
    let special_done = idle_done(rig) && life.talk.queued.is_none();
    let flags = walker
        .package
        .and_then(|p| world::ai::Package::load(order, p))
        .map(|p| p.flags);
    if world::talk_idles::package_refuses_idles(flags) {
        return;
    }
    if !takes_request(sit, special_done, starting, ask) {
        return;
    }
    let idle = match ask.request {
        Request::Idle(id) => seats.tree.get(id).cloned(),
        Request::Tree => {
            let (skeleton, _) = skeleton(life, order, me);
            let roots = seats.roots(&skeleton);
            let values = state.sitters.get(&me).map_or((0, 0, 0), |s| s.question());
            let mut about = question(walker, life, order, (false, false), values);
            if asked.menu {
                about.procedure = procedures::DIALOGUE;
                about.menu = Some(world::idles::DIALOG_MENU);
            }
            about.talking = asked.talking;
            about.hit_location = asked.hit_location;
            about.sneaking = rig.sneaking;
            about.running = rig.running;
            about.greeting_player = asked.greeting;
            let seed = state.roll();
            pick_for(order, state, &seats.tree, &roots, me, life, about, seed)
        }
    };
    let Some(idle) = idle.filter(|i| i.is_animation() && i.group() != 0) else {
        return;
    };
    life.talk.queued = Some(idle.form_id);
    if play_requested(game, state, seats, life, rig, (me, idle.form_id), now) {
        life.talk.queued = None;
    }
}

/// Plays a taken idle request in its record's section (`008dae00` →
/// `00497f20` → `00498290`: the old one freed at once, the new one blended
/// in from the pose, its loops rolled, `005ff770`; one in the base or
/// movement section holds it from the walk, anim action 0xd). False while
/// an idle is starting (`00498f80`), to try again. A request for the idle
/// already playing is dropped (`00498d30`).
#[allow(clippy::too_many_arguments)]
fn play_requested(
    game: &cellview::Game,
    state: &mut GameState,
    seats: &mut Seats,
    life: &mut Life,
    rig: &mut ActorRig,
    (me, id): (FormId, FormId),
    now: f32,
) -> bool {
    let Some(idle) = seats.tree.get(id).cloned() else {
        return true;
    };
    let Some(seq) = seats.sequence(game, &idle.model) else {
        return true;
    };
    let slot = world::animation::slot(rig.idle_section);
    if !idle_done(rig)
        && rig
            .player
            .sequence(slot)
            .is_some_and(|s| Arc::ptr_eq(s, &seq))
    {
        return true;
    }
    let r = state.roll();
    let count = idle.extra_loops(|lo, hi| lo + (r % (u64::from(hi - lo) + 1)) as u8);
    let loops = if count == 255 { -1 } else { i32::from(count) };
    let bones = rig.skeleton.clone();
    let section = idle.group();
    // A free idle from the tree playing as the overlay gives way.
    if let Some(old) = rig.overlay.take().map(|_| rig.overlay_section) {
        let old = world::animation::slot(old);
        if old != world::animation::slot(section)
            && rig.player.playing(old) == Some(world::animation::group::SPECIAL_IDLE)
        {
            rig.player.cut_section(old);
        }
    }
    if !rig
        .player
        .request_idle_in(rig.idle_section, section, &seq, loops, &bones.bones)
    {
        return false;
    }
    rig.idle_section = world::animation::slot(section);
    rig.picker.idle_played(section);
    rig.scripted_idle = Some(id);
    life.idles.played(&idle);
    println!(
        "{now:.1} s: {me}: requested idle {} ({}, section {section})",
        idle.editor_id,
        idle.model.rsplit(['\\', '/']).next().unwrap_or_default()
    );
    true
}

/// The dialogue menu closed on someone (`Actor::EndDialogue`, Xbox PDB,
/// `008b1070`): in sit state 0, 4 or 9 their special idle is to be freed
/// (process flag 0x800, [`free_talk_idle`]).
pub fn dialogue_over(state: &GameState, life: &mut Life, me: FormId) {
    let sit = sit_state(state, me);
    life.talk.menu_closed(sit);
}

/// Carries out a pending free of the special idle (`008ba600` flag 0x800:
/// kept while a special idle is starting, `00498f80`; then `008daf20`,
/// `00498910(1,0)`: its normal blend out).
fn free_talk_idle(life: &mut Life, rig: &mut ActorRig) {
    if life.talk.free_pending && rig.player.free_idle_in(rig.idle_section) {
        life.talk.free_pending = false;
        life.idles.stop();
    }
}

/// Plays an idle: a base-loop one (section 0) becomes the standing loop
/// (seated people's comes from their seat); any other plays over
/// everything its rolled number of times.
fn play(ctx: &mut Ctx, life: &mut Life, idle: &Idle, standing: bool) {
    if idle.group() == 0 {
        if standing {
            life.base_idle = ctx.seats.sequence(ctx.game, &idle.model);
        }
        return;
    }
    let Some(seq) = ctx.seats.sequence(ctx.game, &idle.model) else {
        return;
    };
    let r = ctx.state.roll();
    life.idles.start(idle, seq.stop - seq.start, |lo, hi| {
        lo + (r % u64::from(hi.saturating_sub(lo) + 1)) as u8
    });
    if let Some(p) = &life.idles.playing {
        println!(
            "{:.1} s: plays the idle {} ({}, {:.1} s, {} more times)",
            ctx.now,
            idle.editor_id,
            idle.model.rsplit(['\\', '/']).next().unwrap_or_default(),
            p.length,
            p.loops_left
        );
    }
}

/// The game's clock as the sandbox counts it: days passed, and the hour.
fn clock(order: &LoadOrder, state: &GameState) -> (i32, f32) {
    (
        state.global(order, "GameDaysPassed").unwrap_or(0.0) as i32,
        state.global(order, "GameHour").unwrap_or(12.0),
    )
}

/// Whether someone's idle marker has an idle that passes for them, with
/// its parents (`00479fb0`).
fn marker_usable(ctx: &mut Ctx, walker: &Walker, life: &Life, marker: &IdleMarker) -> bool {
    let game = ctx.game;
    let order = &game.order;
    let seed = ctx.state.roll();
    let facts = Facts {
        order,
        state: ctx.state,
        speaker: None,
    };
    let about = question(walker, life, order, (ctx.talking, ctx.fighting), (0, 0, 0));
    let asker = IdleAsker::new(walker.reference, about, Some(&facts), seed);
    marker
        .idles
        .iter()
        .any(|&id| ctx.seats.tree.passes_with_parents(id, &|i| asker.passes(i)))
}

/// What's around a sandboxing person for [`Sandbox::scan`]: the
/// references in the cell they're in (outdoors, their square) and people
/// brought there, with what each is; and whether they carry food.
fn nearby_list(ctx: &mut Ctx, walker: &Walker, life: &Life) -> (Vec<Nearby>, bool) {
    let me = walker.reference;
    let game = ctx.game;
    let order = &game.order;
    let Some((space, cell, _, _)) = ctx.state.place(order, me) else {
        return (Vec::new(), false);
    };
    let child = world::idles::is_child(order, me);
    // The cell's own (what their records say read once), then those
    // brought there.
    let mut refs: Vec<(FormId, Kind)> = ctx
        .seats
        .candidates(order, cell)
        .iter()
        .filter(|c| !child || c.child_can_use)
        .map(|c| (c.reference, c.kind))
        .collect();
    for r in world::ai::moved_into(order, ctx.state, space) {
        if sandbox::ignored(order, r) || (child && !sandbox::child_can_use(order, r)) {
            continue;
        }
        if let Some(kind) = sandbox::kind_of(order, r) {
            refs.push((r, kind));
        }
    }
    let mut out = Vec::new();
    for (r, kind) in refs {
        if r == me
            || ctx.state.dead.contains(&r)
            || !world::enabled_now(order, r, &ctx.state.disabled)
        {
            continue;
        }
        let Some((there, _, position, _)) = ctx.state.place(order, r) else {
            continue;
        };
        if there != space {
            continue;
        }
        let usable = match kind {
            Kind::Furniture(_) => world::scripting::base_of(order, r)
                .is_some_and(|b| !ctx.seats.markers(ctx.game, b).is_empty()),
            Kind::IdleMarker => {
                IdleMarker::load(order, r).is_some_and(|m| marker_usable(ctx, walker, life, &m))
            }
            _ => true,
        };
        out.push(Nearby {
            reference: r,
            kind,
            position,
            allowed: sandbox::may_use(order, ctx.state, me, r),
            usable,
        });
    }
    let food = ctx.state.inventory(order, me).iter().any(|(item, n)| {
        *n > 0
            && order
                .get(*item)
                .is_some_and(|r| matches!(r.entry.header.kind.as_bytes(), b"INGR" | b"ALCH"))
    });
    (out, food)
}

/// Where a candidate's target is now, if it can be used (`009f4700`'s
/// checks): in the same place and enabled; furniture with a usable marker
/// free (`00568260`); people alive and not fighting or in a conversation
/// (the game asks whether they can talk, `008b06d0`).
fn current_position(
    order: &LoadOrder,
    state: &GameState,
    markers: &HashMap<FormId, Arc<Vec<nif::FurnitureMarker>>>,
    me: FormId,
    here: Option<FormId>,
    c: &Candidate,
) -> Option<[f32; 3]> {
    let t = c.target?;
    let (space, _, position, _) = state.place(order, t)?;
    if Some(space) != here || !world::enabled_now(order, t, &state.disabled) {
        return None;
    }
    match c.activity {
        activities::SIT | activities::SLEEP => {
            let (placed, flags) = placed_markers(order, state, markers, t)?;
            let taken = furniture::taken_markers(state, t, me);
            furniture::first_free(&placed, flags, |i| taken.contains(&i))?;
        }
        activities::DIALOGUE if state.dead.contains(&t) || state.combat.contains_key(&t) => {
            return None;
        }
        _ => {}
    }
    Some(position)
}

/// The nearest sitting furniture with a free marker within
/// `fAIFindBedChairsDistance` (8000, `00922670`) of someone, for eating.
fn nearest_free_chair(ctx: &mut Ctx, walker: &Walker) -> Option<FormId> {
    let me = walker.reference;
    let game = ctx.game;
    let order = &game.order;
    let (space, cell, _, _) = ctx.state.place(order, me)?;
    let reach = world::scripting::game_setting(order, "fAIFindBedChairsDistance").unwrap_or(8000.0);
    let mut best: Option<(f32, FormId)> = None;
    for rr in order.references_in_cell(cell) {
        let r = rr.form_id;
        let Some(Kind::Furniture(mnam)) = sandbox::kind_of(order, r) else {
            continue;
        };
        if mnam & furniture::SIT_FURNITURE == 0
            || !world::enabled_now(order, r, &ctx.state.disabled)
        {
            continue;
        }
        if let Some(b) = world::scripting::base_of(order, r) {
            ctx.seats.markers(ctx.game, b);
        }
        let c = Candidate {
            activity: activities::SIT,
            target: Some(r),
            weight: 1,
        };
        let Some(p) = current_position(order, ctx.state, &ctx.seats.markers, me, Some(space), &c)
        else {
            continue;
        };
        let d = (0..3)
            .map(|k| (p[k] - walker.position[k]).powi(2))
            .sum::<f32>()
            .sqrt();
        if d <= reach && best.is_none_or(|(bd, _)| d < bd) {
            best = Some((d, r));
        }
    }
    best.map(|b| b.1)
}

/// A walk to a wander spot (`008ed420`): a point of the navmesh between
/// 32 and 0.75 × the radius from the middle (`world::ai::wander_ring`),
/// at random (how the game's search picks within the ring isn't traced:
/// here a random point of a random triangle reaching into it, tried a few
/// times), walked to within 50. How far away it is, if there's a way.
fn wander_to(ctx: &mut Ctx, walker: &mut Walker, center: [f32; 3], radius: f32) -> Option<f32> {
    let mesh = ctx.mesh;
    let (near, far) = world::ai::wander_ring(radius);
    if near > far {
        return None;
    }
    let flat = |p: [f32; 3]| (p[0] - center[0]).hypot(p[1] - center[1]);
    let corners = |t: usize| mesh.triangles[t].vertices.map(|v| mesh.vertices[v]);
    // Those near the middle (the grid's, in the mesh's order), then the
    // ones reaching the ring.
    let reaching: Vec<[[f32; 3]; 3]> = mesh
        .triangles_near(center, far)
        .into_iter()
        .map(corners)
        .filter(|c| c.iter().any(|p| flat(*p) <= far) || holds(c, center))
        .collect();
    if reaching.is_empty() {
        return None;
    }
    let mut dice = Dice(ctx.state.roll());
    let mut unit = || (dice.next() % 1_000_000) as f32 / 1_000_000.0;
    for _ in 0..16 {
        let [a, b, c] = reaching[(unit() * reaching.len() as f32) as usize % reaching.len()];
        let (mut u, mut v) = (unit(), unit());
        if u + v > 1.0 {
            (u, v) = (1.0 - u, 1.0 - v);
        }
        let goal = [0, 1, 2].map(|k| a[k] + u * (b[k] - a[k]) + v * (c[k] - a[k]));
        let d = flat(goal);
        if d < near || d > far {
            continue;
        }
        if let Some(path) = crate::ai::path_for(mesh, walker, goal) {
            let d = crate::ai::distance(walker.position, goal);
            walker.set_path(path, 50.0, true, ctx.moves);
            return Some(d);
        }
    }
    None
}

/// The wander procedure's spot-to-spot loop (`008ed420`), standing: the
/// timer (`sandbox::WanderTimer`) let go within 5 of the middle; at 10 or
/// less a new spot ([`wander_to`]) and the timer set to `pause` when it's
/// 50 or more away. When no spot can be reached the timer stays, so it's
/// tried again next frame, as the code does (they stand meanwhile).
fn wander_about(
    ctx: &mut Ctx,
    walker: &mut Walker,
    timer: &mut sandbox::WanderTimer,
    middle: [f32; 3],
    radius: f32,
    pause: f32,
) {
    let to_middle = crate::ai::distance(walker.position, middle);
    if timer.idle(ctx.dt, to_middle) {
        if let Some(d) = wander_to(ctx, walker, middle, radius) {
            timer.chose(d, pause);
        }
    }
}

/// A wander package (type 5) once the walk to its place is over, each
/// frame (`008ed420` on the package's list 1, `world::ai::wander_step`):
/// with a radius under 60 they stand, turned to an `XMarkerHeading`'s
/// heading if the place is one; farther than the radius + 250 from the
/// middle they walk back to the place (the travel again); otherwise from
/// spot to spot ([`wander_about`]) with the wander pause by their energy.
/// The middle: the place's reference (or editor location), their own
/// position for "in a cell", where they stood when the package began for
/// "near the current location" (as for sandboxes; inferred).
pub fn wander_package_frame(ctx: &mut Ctx, walker: &mut Walker, life: &mut Life) {
    if walker.on_path() {
        return;
    }
    let game = ctx.game;
    let order = &game.order;
    let me = walker.reference;
    if ctx.state.furniture.contains_key(&me) || ctx.state.sitters.contains_key(&me) {
        return;
    }
    let Some(package) = walker
        .package
        .and_then(|p| world::ai::Package::load(order, p))
    else {
        return;
    };
    let interior = ctx.state.player_world.is_none();
    let (radius, own) = world::ai::wander_radius(order, &package, interior);
    let place = world::ai::destination(order, ctx.state, me, &package);
    let middle = if own {
        walker.position
    } else {
        place.map_or(walker.home, |(to, _)| to)
    };
    let at_place = place.is_none_or(|(to, r)| world::movement::arrived(walker.position, to, r));
    let to_middle = crate::ai::distance(walker.position, middle);
    match world::ai::wander_step(radius, own, at_place, to_middle) {
        world::ai::WanderStep::Back => {
            life.activity = None;
            let (to, r) = place.unwrap_or((middle, radius));
            if let Some(path) = crate::ai::path_for(ctx.mesh, walker, to) {
                println!("{me} wanders back to their package's place");
                walker.set_path(path, r, true, ctx.moves);
            }
        }
        world::ai::WanderStep::Stand => {
            life.activity = None;
            if let Some(h) = world::ai::marker_heading(order, ctx.state, &package) {
                let off = world::movement::wrap_pi(walker.heading - h).abs();
                if walker.facing.is_none() && off > world::movement::ONE_DEGREE {
                    walker.facing = Some(h);
                }
            }
        }
        world::ai::WanderStep::Wander => {
            let mut timer = match &life.activity {
                Some(Activity::Wander { timer }) => *timer,
                _ => sandbox::WanderTimer::default(),
            };
            let energy = sandbox::energy(order, me);
            let pause = sandbox::wander_pause(&ctx.seats.sandbox, energy);
            wander_about(ctx, walker, &mut timer, middle, radius, pause);
            life.activity = Some(Activity::Wander { timer });
        }
    }
}

/// Whether a triangle holds a point, seen from above.
fn holds(c: &[[f32; 3]; 3], p: [f32; 3]) -> bool {
    let side =
        |a: [f32; 3], b: [f32; 3]| (b[0] - a[0]) * (p[1] - a[1]) - (b[1] - a[1]) * (p[0] - a[0]);
    let (s0, s1, s2) = (side(c[0], c[1]), side(c[1], c[2]), side(c[2], c[0]));
    (s0 >= 0.0 && s1 >= 0.0 && s2 >= 0.0) || (s0 <= 0.0 && s1 <= 0.0 && s2 <= 0.0)
}

/// Sets out on a sandbox choice.
fn start_activity(
    ctx: &mut Ctx,
    walker: &mut Walker,
    life: &mut Life,
    c: Choice,
    chats: &mut Chats,
    social: &world::social::SocialSettings,
) {
    let me = walker.reference;
    let game = ctx.game;
    let order = &game.order;
    println!(
        "{:.1} s: {me} sandboxes: {}{} for {:.1} game minutes",
        ctx.now,
        activities::name(c.activity),
        c.target.map(|t| format!(" {t}")).unwrap_or_default(),
        c.duration
    );
    let started = match c.activity {
        activities::SIT | activities::SLEEP => {
            let ok = c.target.is_some_and(|t| begin_use(ctx, walker, t));
            if ok {
                life.activity = Some(Activity::Furniture);
            }
            ok
        }
        activities::EAT => {
            life.activity = Some(Activity::Eat(Eating {
                food: c.target,
                item: None,
                chair: None,
                searched: false,
                eaten: false,
                idle_in: 0.0,
            }));
            true
        }
        activities::WANDER => {
            life.activity = Some(Activity::Wander {
                timer: sandbox::WanderTimer::default(),
            });
            true
        }
        activities::IDLE_MARKER => {
            let marker = c
                .target
                .and_then(|t| Some((IdleMarker::load(order, t)?, ctx.state.place(order, t)?)));
            match marker {
                Some((idles, (_, _, at, heading))) => {
                    life.activity = Some(Activity::IdleMarker {
                        heading,
                        idles,
                        next: usize::MAX,
                    });
                    // Within 10 of the marker (`00901c70`).
                    if (0..2).all(|k| (at[k] - walker.position[k]).abs() < 10.0) {
                        true
                    } else {
                        match crate::ai::path_for(ctx.mesh, walker, at) {
                            Some(path) => {
                                walker.set_path(path, 10.0, true, ctx.moves);
                                true
                            }
                            None => false,
                        }
                    }
                }
                None => false,
            }
        }
        // A conversation with the one chosen (`00901a20`), its timer started
        // again.
        activities::DIALOGUE => match c.target {
            Some(with) => {
                life.activity = Some(Activity::Talk { with });
                let seated = ctx.state.sitters.contains_key(&me);
                let ok = crate::ai::start_chat(ctx, walker, chats, with, None, seated);
                if ok {
                    let mut dice = Dice(ctx.state.roll());
                    let unit = (dice.next() % 1_000_000) as f32 / 1_000_000.0;
                    if let Some(s) = walker.social.as_mut() {
                        let (lo, hi) = social.conversation_timer;
                        s.conversation = lo + (hi - lo) * unit;
                    }
                }
                ok
            }
            None => false,
        },
        _ => false,
    };
    if !started {
        println!("{me} can't {}", activities::name(c.activity));
        give_up(life);
    }
}

/// One frame of a sandbox package (`00929fc0`): back to the area if
/// they've strayed; when it's time for something else (and no sitting or
/// idle-marker idle is playing, `009f4300`), up from any seat first, then
/// a new choice (`Sandbox::choose`, the area scanned when due; talking
/// weighs nothing while their conversation timer runs) and off to it; the
/// current activity carried on.
pub fn sandbox_frame(
    ctx: &mut Ctx,
    walker: &mut Walker,
    life: &mut Life,
    chats: &mut Chats,
    social: &world::social::SocialSettings,
) {
    let me = walker.reference;
    let Some(mut sb) = life.sandbox.take() else {
        return;
    };
    let game = ctx.game;
    let order = &game.order;
    let now_clock = clock(order, ctx.state);
    let sit = ctx.state.sitters.get(&me).map(|s| s.state);
    let seated = sit.is_some_and(|s| s != SitState::Normal);
    let walking = walker.on_path();
    if !seated && !walking && sb.strayed(walker.position) {
        if let Some(path) = crate::ai::path_for(ctx.mesh, walker, sb.center) {
            println!("{me} goes back to their sandbox area");
            walker.set_path(path, 0.0, true, ctx.moves);
        }
        sb.choice = None;
        life.activity = None;
        life.sandbox = Some(sb);
        return;
    }
    let idle_busy = life.idles.playing.is_some()
        && matches!(
            life.activity,
            Some(Activity::Furniture) | Some(Activity::IdleMarker { .. })
        );
    if !idle_busy && sb.time_for_something_else(now_clock) {
        if seated {
            // Up first (`Actor::StandUp`); the choice waits until they are.
            if let Some(s) = ctx.state.sitters.get_mut(&me) {
                s.stand_up();
            }
            life.sandbox = Some(sb);
            return;
        }
        if sit == Some(SitState::Normal) {
            ctx.state.stand(me);
        }
        life.activity = None;
        walker.clear_path();
        let nearby = if sb.scan_due(ctx.now) {
            Some(nearby_list(ctx, walker, life))
        } else {
            None
        };
        sb.conversation_wait = walker.social.as_ref().is_some_and(|s| s.conversation > 0.0);
        let mut dice = Dice(ctx.state.roll());
        let choice = {
            let state = &*ctx.state;
            let markers = &ctx.seats.markers;
            let here = state.place(order, me).map(|p| p.0);
            let current = |c: &Candidate| current_position(order, state, markers, me, here, c);
            sb.choose(
                me,
                &ctx.seats.sandbox,
                ctx.now,
                now_clock,
                &mut || dice.next(),
                nearby.as_ref().map(|(l, f)| (l.as_slice(), *f)),
                &current,
            )
        };
        if let Some(c) = choice {
            start_activity(ctx, walker, life, c, chats, social);
            // A choice that couldn't start is chosen again next frame.
            if life.activity.is_none() {
                sb.failed();
                sb.choice = None;
            }
        }
    }
    carry_on(ctx, walker, life, &mut sb);
    life.sandbox = Some(sb);
}

/// The current activity, each frame: wanderers pause and move on, at an
/// idle marker they face its heading and play its idles, eaters go on
/// eating.
fn carry_on(ctx: &mut Ctx, walker: &mut Walker, life: &mut Life, sb: &mut Sandbox) {
    let game = ctx.game;
    let order = &game.order;
    match life.activity.clone() {
        // The wander procedure for a sandbox (`008ed420` with package type
        // 12: no "under 60 stand" and no radius + 250 tests, those are wander
        // packages').
        Some(Activity::Wander { mut timer }) => {
            if walker.on_path() {
                return;
            }
            // No spot (a radius under 43 leaves an empty ring): they stand,
            // tried again next frame; the procedure doesn't fail the
            // activity, which lasts its time.
            let pause = sandbox::wander_pause(&ctx.seats.sandbox, sb.energy);
            wander_about(ctx, walker, &mut timer, sb.center, sb.radius, pause);
            life.activity = Some(Activity::Wander { timer });
        }
        Some(Activity::IdleMarker {
            heading,
            idles,
            next,
        }) => {
            if walker.on_path() {
                return;
            }
            if face(walker, heading, ctx.dt, false, ctx.moves) {
                return;
            }
            if life.idles.playing.is_some() {
                return;
            }
            // Its usable idles, the next in order or one at random.
            let seed = ctx.state.roll();
            let usable: Vec<Idle> = {
                let facts = Facts {
                    order,
                    state: ctx.state,
                    speaker: None,
                };
                let about = question(walker, life, order, (ctx.talking, ctx.fighting), (0, 0, 0));
                let asker = IdleAsker::new(walker.reference, about, Some(&facts), seed);
                idles
                    .idles
                    .iter()
                    .filter(|&&id| ctx.seats.tree.passes_with_parents(id, &|i| asker.passes(i)))
                    .filter_map(|&id| ctx.seats.tree.get(id).cloned())
                    .collect()
            };
            if usable.is_empty() {
                return;
            }
            let i = if idles.in_sequence() {
                if next == usize::MAX || next + 1 >= usable.len() {
                    0
                } else {
                    next + 1
                }
            } else {
                (seed % usable.len() as u64) as usize
            };
            let standing = sitter_is_none(ctx.state, walker.reference);
            play(ctx, life, &usable[i], standing);
            life.activity = Some(Activity::IdleMarker {
                heading,
                idles,
                next: i,
            });
        }
        Some(Activity::Eat(e)) => eat_frame(ctx, walker, life, sb, e),
        // The conversation runs in `ai::chat_frame`.
        _ => {}
    }
}

/// One frame of eating (the eat procedure `008e3140`, as findings §5a read
/// it): food lying in the area is walked to (within its reach, as the
/// activate procedure measures: `world::movement::target_reach`) and taken
/// (`world::sandbox::take_food`), else food carried is used; then a free
/// chair is looked for once and sat in; seated, one is eaten
/// (`world::sandbox::eat`) and an eating idle asked for every random 1–6 s
/// with the food as the idle's item; without a chair they eat where they
/// stand. No food: the activity fails.
fn eat_frame(ctx: &mut Ctx, walker: &mut Walker, life: &mut Life, sb: &mut Sandbox, mut e: Eating) {
    let game = ctx.game;
    let order = &game.order;
    let me = walker.reference;
    if walker.on_path() {
        return;
    }
    let fail = |life: &mut Life, sb: &mut Sandbox| {
        println!("{me} has nothing to eat.");
        sb.failed();
        sb.choice = None;
        life.activity = None;
    };
    if e.item.is_none() {
        match e.food {
            Some(food) => {
                let Some((_, _, at, _)) = ctx.state.place(order, food) else {
                    fail(life, sb);
                    return;
                };
                let reach = world::movement::target_reach(
                    0,
                    world::ai::spot_of(order, ctx.state, food),
                    world::ai::min_location_radius(order),
                );
                let radius = walker
                    .kit
                    .as_ref()
                    .map_or(world::combat_ai::PERSON_RADIUS, |k| k.radius);
                if world::movement::within_distance(
                    walker.position,
                    128.0,
                    Some(radius),
                    at,
                    reach,
                    true,
                ) {
                    e.item = sandbox::take_food(order, ctx.state, me, food);
                    if e.item.is_none() {
                        fail(life, sb);
                        return;
                    }
                    println!("{:.1} s: {me} takes {food} to eat.", ctx.now);
                } else {
                    match crate::ai::path_for(ctx.mesh, walker, at) {
                        Some(path) => {
                            walker.set_path(path, reach, true, ctx.moves);
                            life.activity = Some(Activity::Eat(e));
                        }
                        None => fail(life, sb),
                    }
                    return;
                }
            }
            None => {
                e.item = sandbox::carried_food(order, ctx.state, me);
                if e.item.is_none() {
                    fail(life, sb);
                    return;
                }
            }
        }
    }
    if !e.searched {
        e.searched = true;
        e.chair = nearest_free_chair(ctx, walker);
        if let Some(chair) = e.chair {
            if !begin_use(ctx, walker, chair) {
                e.chair = None;
            }
        }
        life.activity = Some(Activity::Eat(e));
        return;
    }
    let seated = ctx
        .state
        .sitters
        .get(&me)
        .is_some_and(|s| s.state.is_settled());
    let item = e.item.expect("set above");
    if !e.eaten && (e.chair.is_none() || seated) {
        if sandbox::eat(order, ctx.state, me, item) {
            println!("{:.1} s: {me} eats {item}.", ctx.now);
        }
        e.eaten = true;
        e.idle_in = 0.0;
    }
    // Seated and eaten: an eating idle every 1–6 s.
    if e.eaten && seated {
        e.idle_in -= ctx.dt;
        if e.idle_in <= 0.0 && life.idles.playing.is_none() {
            let (lo, hi) = sandbox::EATING_IDLE_EVERY;
            let mut dice = Dice(ctx.state.roll());
            e.idle_in = lo + (hi - lo) * ((dice.next() % 1_000_000) as f32 / 1_000_000.0);
            life.activity = Some(Activity::Eat(e.clone()));
            let (skeleton, _) = skeleton(life, order, me);
            let roots = ctx.seats.roots(&skeleton);
            let values = ctx
                .state
                .sitters
                .get(&me)
                .map_or((0, 0, 0), |s| s.question());
            let seed = ctx.state.roll();
            let found = pick_idle(
                order,
                ctx.state,
                &ctx.seats.tree,
                &roots,
                walker,
                life,
                (ctx.talking, ctx.fighting),
                values,
                seed,
            );
            if let Some(idle) = found {
                play(ctx, life, &idle, false);
            }
        }
    }
    life.activity = Some(Activity::Eat(e));
}

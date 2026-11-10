//! The start of a co-op game: how each player gets a character.
//!
//! The opening (`VCG00`, `VCG01`: Doc Mitchell's house, his speech, the name box, the
//! Vit-o-Matic and its questions, the tag skills and traits) is per player in the campaign
//! policy (`scope: per_player`, VCG01 and VCG00 both). Every player has their own character.
//!
//! * The host runs the normal opening for the host player (the viewer's `--new-game`), or loads
//!   a save (`--coop-load`, `--load-fos`) and plays on.
//! * A guest joins at any time. The join snapshot carries the shared story. The guest then makes
//!   its own character with the same menus, local to its machine, and sends the result (a
//!   [`Sheet`]) to the host, which keeps it in the guest's shadow. The host's world is never
//!   blocked: menus are local only in a session.
//!
//! Where the guest makes it depends on the host ([`Opening`]):
//!
//! | host | venue | the guest |
//! |---|---|---|
//! | still in the opening | [`Venue::HostPlace`] | loads the host's place (Doc's house) and runs the menus there |
//! | opening over (or never run) | [`Venue::LocalRoom`] | loads [`LOCAL_ROOM`], runs the menus, then follows the host |
//!
//! [`StartFlow`] is the state machine: [`Phase::Joining`] -> [`Phase::CharacterCreation`] ->
//! [`Phase::Arriving`] -> [`Phase::Playing`]. It is pure; the [`Session`] feeds it and the viewer
//! (or the headless simulator) opens the menus it asks for. A guest started with `--new-game` makes
//! a new character (the default anyway); with `--load-fos FILE` or `--coop-load FILE` its character
//! is read from that save ([`OwnCharacter::Provided`]) and the menus are skipped. A guest never
//! overrides the host's shared state: only the [`Sheet`] (name, sex, SPECIAL, tags, traits) and its
//! own `VCG00`/`VCG01` rows are taken from it.

use std::collections::BTreeMap;

use coop_core::ids::PlayerId;
use coop_core::scene::SceneKind;
use coop_core::schema::flag;
use coop_game::{opening, Game, Intent, Shared};
use esm::FormId;
// The tests of this file write the player's rows of a `GameState`.
#[cfg(test)]
use world::dialogue::PLAYER_REF;
use world::scripting::GameState;

use crate::ext::Ext;
use crate::json::V;
use crate::session::{send_ext, Session};
use crate::Role;

/// `VCG00`, the prologue quest ("Welcome To Fabulous New Vegas").
pub const VCG00: FormId = FormId(opening::VCG00.0);
/// `VCG01`, Doc Mitchell's intro quest.
pub const VCG01: FormId = FormId(opening::VCG01.0);
/// `VCG00` stage that ends the prologue.
pub const VCG00_DONE: u16 = opening::VCG00_DONE;
/// `VCG01` stage from which the character is made (Doc's farewell transition, after the traits).
pub const OPENING_OVER_STAGE: u16 = opening::OPENING_OVER_STAGE;
/// `VCG01` stage at which the host's own name box opens (`GetPlayerName`, docs/OPENING.md): a
/// guest watching the host's opening starts its own menus from here.
pub const NAME_STAGE: u16 = 10;
/// Where a guest makes its character when the host's opening is over.
pub const LOCAL_ROOM: &str = "GSDocMitchellHouse";
/// SPECIAL points the Vit-o-Matic hands out (`ShowLoveTesterMenuParams 40`).
pub const SPECIAL_POINTS: u32 = 40;
/// Skills to tag (`SetTagSkills 3 1`).
pub const TAG_COUNT: u32 = 3;
/// Traits to pick at most (`ShowTraitMenu`).
pub const TRAIT_MAX: u32 = 2;
/// The SPECIAL attributes' actor values (Strength .. Luck).
pub const SPECIAL: [u16; 7] = coop_game::SPECIAL;
/// Milliseconds between sends of an unacknowledged sheet.
const SHEET_RESEND_MS: u64 = 1000;

/// Where a peer stands in its start.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Phase {
    /// Guest: connected, waiting for the snapshot and the host's opening state.
    Joining,
    /// Making the character (a guest: its own menus; the host: Doc's opening).
    CharacterCreation,
    /// Guest: character sent, going to (and waiting to be at) the host's place.
    Arriving,
    /// In the game.
    Playing,
}

impl Phase {
    /// The name the dump and the log use.
    pub fn name(self) -> &'static str {
        match self {
            Phase::Joining => "joining",
            Phase::CharacterCreation => "character_creation",
            Phase::Arriving => "arriving",
            Phase::Playing => "playing",
        }
    }
}

/// How far the host's own opening is.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Opening {
    /// `VCG00`/`VCG01` running for the host (new game, before the character is made).
    InProgress,
    /// Over, or never run (a loaded save, a cell start).
    Over,
}

impl Opening {
    /// The name the dump and the log use.
    pub fn name(self) -> &'static str {
        match self {
            Opening::InProgress => "in_progress",
            Opening::Over => "over",
        }
    }

    pub(crate) fn byte(self) -> u8 {
        match self {
            Opening::InProgress => 0,
            Opening::Over => 1,
        }
    }

    pub(crate) fn from_byte(b: u8) -> Option<Opening> {
        match b {
            0 => Some(Opening::InProgress),
            1 => Some(Opening::Over),
            _ => None,
        }
    }
}

/// Where a guest makes its character.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Venue {
    /// In the host's cell (the host is still in the opening).
    HostPlace,
    /// In [`LOCAL_ROOM`], away from the host, until done.
    LocalRoom,
}

impl Venue {
    /// The name the dump and the log use.
    pub fn name(self) -> &'static str {
        match self {
            Venue::HostPlace => "host_place",
            Venue::LocalRoom => "local_room",
        }
    }
}

/// Where a guest's character comes from (`--new-game`, `--load-fos`, `--coop-load` on a guest).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum OwnCharacter {
    /// Made with the opening's menus.
    #[default]
    Create,
    /// Read from a save the guest brought; the menus are skipped.
    Provided,
    /// Test harness only (`--coop-no-chargen`): no character making; the guest plays at once.
    Skip,
}

/// What the start needs from the process (set before the first frame).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct StartConfig {
    /// Host: `--new-game`, the opening runs for the host.
    pub new_game: bool,
    /// Guest: where its character comes from.
    pub own: OwnCharacter,
}

/// One menu of the character making, in order.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Step {
    /// The name box (`GetPlayerName`).
    Name,
    /// The Vit-o-Matic: SPECIAL ([`SPECIAL_POINTS`]).
    Special,
    /// The tag skills ([`TAG_COUNT`]).
    TagSkills,
    /// The traits (at most [`TRAIT_MAX`]).
    Traits,
}

/// The menus a guest runs, in order. (The race, sex and face menu, `ShowRaceMenu`, is not built
/// in nv-rs yet; neither the single-player opening nor this flow can show it.)
pub const STEPS: [Step; 4] = [Step::Name, Step::Special, Step::TagSkills, Step::Traits];

impl Step {
    /// The name the log uses.
    pub fn name(self) -> &'static str {
        match self {
            Step::Name => "name",
            Step::Special => "special",
            Step::TagSkills => "tag_skills",
            Step::Traits => "traits",
        }
    }
}

/// What the flow asks the glue to do.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Action {
    /// Open this menu (local only), and report when it is closed (`menu_busy` false again).
    OpenMenu(Step),
    /// The menus are done: read the sheet and send it.
    Finished,
}

/// The start of one peer, as a state machine.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StartFlow {
    role: Role,
    own: OwnCharacter,
    phase: Phase,
    venue: Venue,
    opening: Option<Opening>,
    step: usize,
    opened: bool,
    finished: bool,
    acked: bool,
}

impl StartFlow {
    /// The host's start: the opening when `new_game`, else straight to playing.
    pub fn host(new_game: bool) -> StartFlow {
        StartFlow {
            role: Role::Host,
            own: OwnCharacter::Create,
            phase: if new_game {
                Phase::CharacterCreation
            } else {
                Phase::Playing
            },
            venue: Venue::HostPlace,
            opening: Some(if new_game {
                Opening::InProgress
            } else {
                Opening::Over
            }),
            step: 0,
            opened: false,
            finished: false,
            acked: true,
        }
    }

    /// A guest's start.
    pub fn guest(own: OwnCharacter) -> StartFlow {
        StartFlow {
            role: Role::Guest,
            own,
            phase: Phase::Joining,
            venue: Venue::HostPlace,
            opening: None,
            step: 0,
            opened: false,
            finished: false,
            acked: false,
        }
    }

    /// Where it stands.
    pub fn phase(&self) -> Phase {
        self.phase
    }

    /// Where the character is made (known once the host's opening is).
    pub fn venue(&self) -> Venue {
        self.venue
    }

    /// The host's opening, once known.
    pub fn opening(&self) -> Option<Opening> {
        self.opening
    }

    /// The menu being made (guest), if any.
    pub fn step(&self) -> Option<Step> {
        (self.phase == Phase::CharacterCreation && self.role == Role::Guest)
            .then(|| STEPS.get(self.step).copied())
            .flatten()
    }

    /// Host: the opening's progress from the host's own quest rows. `CharacterCreation` ends when
    /// the opening is over.
    pub fn host_opening(&mut self, o: Opening) {
        if self.role != Role::Host {
            return;
        }
        self.opening = Some(o);
        if o == Opening::Over && self.phase == Phase::CharacterCreation {
            self.phase = Phase::Playing;
        }
    }

    /// Guest: the host told its opening (and the session is ready). From `Joining` this picks the
    /// venue and starts the character making, or arriving for a character that came from a save.
    pub fn guest_joined(&mut self, o: Opening) {
        if self.role != Role::Guest {
            return;
        }
        self.opening = Some(o);
        if self.phase != Phase::Joining {
            return;
        }
        match self.own {
            OwnCharacter::Skip => {
                self.venue = Venue::HostPlace;
                self.phase = Phase::Playing;
            }
            OwnCharacter::Provided => {
                self.venue = Venue::HostPlace;
                self.finished = false;
                self.phase = Phase::Arriving;
            }
            OwnCharacter::Create => {
                self.venue = if o == Opening::InProgress {
                    Venue::HostPlace
                } else {
                    Venue::LocalRoom
                };
                self.phase = Phase::CharacterCreation;
            }
        }
    }

    /// Guest: the host stored the sheet.
    pub fn sheet_acked(&mut self) {
        self.acked = true;
    }

    /// True while a guest may follow the host to a new place: not while it makes its character
    /// away from the host.
    pub fn follow_ok(&self) -> bool {
        match self.role {
            Role::Host => true,
            Role::Guest => {
                !(self.phase == Phase::Joining
                    || (self.phase == Phase::CharacterCreation && self.venue == Venue::LocalRoom))
            }
        }
    }

    /// True while a guest's own player rows are being made here, so the host's copy of its
    /// shadow must not replace them: from the first menu until the host stored the sheet (a
    /// character that came from a save has nothing to protect).
    pub fn holds_own_rows(&self) -> bool {
        self.role == Role::Guest
            && self.own == OwnCharacter::Create
            && (self.phase == Phase::CharacterCreation
                || (self.phase == Phase::Arriving && !self.acked))
    }

    /// One frame. `menu_busy`: some menu is up or queued here. `at_host`: the guest is in the
    /// host's place. Returns what to do now.
    pub fn poll(&mut self, menu_busy: bool, at_host: bool) -> Option<Action> {
        self.poll_gated(menu_busy, at_host, true)
    }

    /// [`Self::poll`] with `host_ready`: false while the guest should still be watching the host's
    /// opening (its intro movie, the scene it is held in, Doc's speech up to the host's own name
    /// box), so the first menu does not open yet.
    pub fn poll_gated(
        &mut self,
        menu_busy: bool,
        at_host: bool,
        host_ready: bool,
    ) -> Option<Action> {
        if self.role != Role::Guest {
            return None;
        }
        match self.phase {
            Phase::Joining | Phase::Playing => None,
            Phase::CharacterCreation => {
                if self.opened && !menu_busy {
                    self.opened = false;
                    self.step += 1;
                }
                if self.step >= STEPS.len() {
                    self.phase = Phase::Arriving;
                    self.finished = true;
                    return Some(Action::Finished);
                }
                if !self.opened && !menu_busy && host_ready {
                    self.opened = true;
                    return Some(Action::OpenMenu(STEPS[self.step]));
                }
                None
            }
            Phase::Arriving => {
                if self.acked && at_host {
                    self.phase = Phase::Playing;
                }
                None
            }
        }
    }

    /// True once the menus are done (the sheet is to be sent).
    pub fn finished(&self) -> bool {
        self.finished
    }
}

/// A player's character: what the opening's menus decide. Sent by a guest to the host.
#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct Sheet {
    /// The name from the name box (empty: not set).
    pub name: String,
    /// Sex when known (the race and sex menu is not built).
    pub female: Option<bool>,
    /// Strength, Perception, Endurance, Charisma, Intelligence, Agility, Luck.
    pub special: [u8; 7],
    /// Tagged skills (actor values).
    pub tags: Vec<u16>,
    /// Perks the player has (the traits among them), form ids.
    pub perks: Vec<u32>,
}

impl Sheet {
    /// The character as `gs` holds it for its own player (the live `PLAYER_REF` rows).
    ///
    /// For a caller that holds a `GameState`. The session itself reads the character
    /// through the engine interface (`coop_game::Players::local_player`). The two give the
    /// same sheet: the `world` adapter has one function for it.
    pub fn read(gs: &GameState) -> Sheet {
        world::coop::game::local_player_of(gs).sheet.into()
    }

    /// Puts the character into `gs`'s player rows and marks the opening done for that player
    /// (`VCG00` and `VCG01` rows, without running any stage script).
    ///
    /// For a caller that holds a `GameState`. The session itself gives a character through
    /// the engine interface (`coop_game::Intent::Character`), which is this function for
    /// the rows of one player.
    pub fn apply(&self, gs: &mut GameState) {
        world::coop::game::apply_sheet(gs, &self.clone().into());
    }
}

impl From<coop_game::Sheet> for Sheet {
    fn from(s: coop_game::Sheet) -> Sheet {
        Sheet {
            name: s.name,
            female: s.female,
            special: s.special,
            tags: s.tags,
            perks: s.perks.iter().map(|perk| perk.0).collect(),
        }
    }
}

impl From<Sheet> for coop_game::Sheet {
    fn from(s: Sheet) -> coop_game::Sheet {
        coop_game::Sheet {
            name: s.name,
            female: s.female,
            special: s.special,
            tags: s.tags,
            perks: s
                .perks
                .iter()
                .map(|perk| coop_game::FormId(*perk))
                .collect(),
        }
    }
}

/// What the dump shows (`start.phase` and friends).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StartInfo {
    /// The phase.
    pub phase: Phase,
    /// The venue (guest).
    pub venue: Venue,
    /// The host's opening when known.
    pub opening: Option<Opening>,
    /// The sheet was stored by the host (guest) or the guest made one (host).
    pub sheet_acked: bool,
    /// The menu of the moment (guest).
    pub step: Option<Step>,
}

impl StartInfo {
    /// The dump's `start` object.
    pub fn json(&self) -> String {
        format!(
            "{{\"phase\":\"{}\",\"venue\":\"{}\",\"opening\":\"{}\",\"sheet_acked\":{},\"step\":\"{}\"}}",
            self.phase.name(),
            self.venue.name(),
            self.opening.map_or("unknown", Opening::name),
            self.sheet_acked,
            self.step.map_or("", Step::name)
        )
    }
}

/// The session's start bookkeeping (both roles).
pub(crate) struct StartSide {
    pub flow: StartFlow,
    pub cfg: StartConfig,
    /// Guest: a sheet that came from a save.
    pub provided: Option<Sheet>,
    /// Guest: the sheet being delivered and when it was last sent.
    pub sheet_out: Option<(Sheet, Option<u64>)>,
    /// Host: what each peer was last told.
    pub told: BTreeMap<PlayerId, Opening>,
    /// Host: the sheets received and not yet applied.
    pub pending: Vec<(PlayerId, Sheet)>,
    /// Host: the sheet last applied per peer (a repeat is only acknowledged).
    pub applied: BTreeMap<PlayerId, Sheet>,
    /// Host: peers whose sheet was applied last frame; their acknowledgement goes out once the
    /// applied rows are in the store (so the guest takes its own rows back from a current copy).
    pub ack_after: Vec<PlayerId>,
    /// The last phase logged.
    pub logged: Option<Phase>,
    /// Both roles: the character names chosen in the name box, per player (host: gathered; guest: told).
    pub char_names: BTreeMap<PlayerId, String>,
    /// Host: the names each peer was last told.
    pub names_told: BTreeMap<PlayerId, BTreeMap<PlayerId, String>>,
}

impl StartSide {
    pub fn new(role: Role) -> StartSide {
        StartSide {
            flow: match role {
                Role::Host => StartFlow::host(false),
                Role::Guest => StartFlow::guest(OwnCharacter::Create),
            },
            cfg: StartConfig::default(),
            provided: None,
            sheet_out: None,
            told: BTreeMap::new(),
            pending: Vec::new(),
            applied: BTreeMap::new(),
            ack_after: Vec::new(),
            logged: None,
            char_names: BTreeMap::new(),
            names_told: BTreeMap::new(),
        }
    }
}

/// The rule of the host's opening. `vcg00` and `vcg01` are the stages of the two opening
/// quests in the host player's rows (`None`: the quest has no stage).
fn opening_from(
    vcg00: Option<u16>,
    vcg01: Option<u16>,
    vcg01_completed: bool,
    new_game: bool,
) -> Opening {
    let finished = vcg01.is_some_and(|s| s >= OPENING_OVER_STAGE) || vcg01_completed;
    if finished {
        return Opening::Over;
    }
    let started = vcg00.is_some() || vcg01.is_some();
    if started || new_game {
        Opening::InProgress
    } else {
        Opening::Over
    }
}

/// The host's opening from its own quest rows.
///
/// For a caller that holds a `GameState`. The session itself reads the rows through the
/// engine interface ([`opening_in`]).
pub fn opening_of(gs: &GameState, new_game: bool) -> Opening {
    let st = |q: FormId| gs.stages.get(&q).copied();
    opening_from(
        st(VCG00),
        st(VCG01),
        gs.completed.contains(&VCG01),
        new_game,
    )
}

/// The host's opening from the rows of the game's live player, through the engine
/// interface. It needs `Shared::stage`: `Shared::quest` does not tell a quest with no stage
/// from a quest at stage 0, and a quest that starts with the game has flags and no stage.
pub(crate) fn opening_in<G: Shared + ?Sized>(game: &G, new_game: bool) -> Opening {
    let completed = game
        .quest(opening::VCG01)
        .is_some_and(|(_, flags)| flags & flag::COMPLETED != 0);
    opening_from(
        game.stage(opening::VCG00),
        game.stage(opening::VCG01),
        completed,
        new_game,
    )
}

impl Session {
    /// Sets how this process starts (`--new-game` on a host, the guest's own character). Call
    /// before the first frame.
    pub fn set_start_config(&mut self, cfg: StartConfig) {
        self.start.cfg = cfg;
        self.start.flow = match self.role {
            Role::Host => StartFlow::host(cfg.new_game),
            Role::Guest => StartFlow::guest(cfg.own),
        };
    }

    /// Guest with [`OwnCharacter::Provided`]: the character read from a save.
    pub fn set_own_sheet(&mut self, sheet: Sheet) {
        self.start.provided = Some(sheet);
    }

    /// The host's opening: computed on the host, told to a guest (guest side: once told).
    pub fn host_opening(&self) -> Option<Opening> {
        self.start.flow.opening()
    }

    /// Where this peer stands in its start.
    pub fn start_phase(&self) -> Phase {
        self.start.flow.phase()
    }

    /// The dump's `start` view.
    pub fn start_info(&self) -> StartInfo {
        let f = &self.start.flow;
        StartInfo {
            phase: f.phase(),
            venue: f.venue(),
            opening: f.opening(),
            sheet_acked: match self.role {
                Role::Guest => f.acked,
                Role::Host => !self.start.applied.is_empty(),
            },
            step: f.step(),
        }
    }

    /// Guest: false while it makes its character away from the host. The viewer's
    /// `follow_place` waits for this.
    pub fn start_follow_ok(&self) -> bool {
        self.start.flow.follow_ok()
    }

    /// True when the local player is in the host's place. Outdoors the worldspace decides:
    /// the game can know the worldspace and not the cell.
    fn at_host_place(&self, at: &coop_game::Place) -> bool {
        let Some(p) = self.host_place_for_start() else {
            return false;
        };
        if p.world != 0 {
            at.world.map(|w| w.0) == Some(p.world)
        } else {
            at.world.is_none() && at.cell.map(|c| c.0) == Some(p.cell)
        }
    }

    fn host_place_for_start(&self) -> Option<crate::ext::Place> {
        self.guest.as_ref().and_then(|g| g.place)
    }

    /// The host's opening quest stages as this machine sees them (its row of the shadow when the
    /// quests are per player, else the shared rows): (`VCG00`, `VCG01`).
    fn host_opening_stages(gs: &GameState) -> (Option<u16>, Option<u16>) {
        match gs.shadow_of(0) {
            Some(sh) if gs.coop.bank.active != 0 => (
                sh.quest_stage.get(&VCG00).copied(),
                sh.quest_stage.get(&VCG01).copied(),
            ),
            _ => (
                gs.stages.get(&VCG00).copied(),
                gs.stages.get(&VCG01).copied(),
            ),
        }
    }

    /// Guest: false while it should still watch the host's opening instead of making its
    /// character: a scene with something to watch or a movie is running here (the intro,
    /// `FNVIntro`), or the host's opening has not reached its own name box ([`NAME_STAGE`]). An
    /// opening whose stages this machine cannot see (no quest rows at all) never holds the guest
    /// back. A control lock alone (no camera) does not hold the menus back: the host's opening
    /// locks the controls of the whole party (the `VCG00` and `VCG01` policy rows) for up to
    /// 120 s, and the character menus are local.
    fn host_opening_ready(&self, gs: &GameState) -> bool {
        let sc = &gs.coop.live.scene;
        let watching = sc
            .view
            .as_ref()
            .is_some_and(|v| v.kind != SceneKind::Control);
        if watching || sc.movie_start.is_some() || sc.movie_busy {
            return false;
        }
        let f = &self.start.flow;
        if f.venue() != Venue::HostPlace || f.opening() != Some(Opening::InProgress) {
            return true;
        }
        if f.step != 0 {
            return true;
        }
        let (a, b) = Self::host_opening_stages(gs);
        if a.is_none() && b.is_none() {
            return true;
        }
        b.unwrap_or(0) >= NAME_STAGE
    }

    /// Guest: once per frame, after the world is loaded. `menu_busy`: a menu is up or queued.
    /// Returns the menu to open now (local only), if any. The host always gets `None`.
    pub fn start_poll(&mut self, gs: &GameState, menu_busy: bool) -> Option<Step> {
        if self.role != Role::Guest {
            return None;
        }
        self.start_join();
        // This call gets a `&GameState`, and the adapter of the engine interface needs
        // `&mut`: the local player comes from the adapter's read function. It is the answer
        // of `coop_game::Players::local_player`.
        let local = world::coop::game::local_player_of(gs);
        let ready = self.host_opening_ready(gs);
        self.start_step(local, ready, menu_busy)
    }

    /// [`Session::start_poll`] on any game of the engine interface. Call it one time for
    /// each frame of a guest, between [`Session::pre_frame_game`] and
    /// [`Session::post_frame_game`]. A guest that never calls it stays in the joining phase.
    ///
    /// The guest does not wait for the host's opening here: the scene and the stages of
    /// the host's player are not behind the interface yet. The first menu opens when the
    /// session is ready.
    pub fn start_poll_game(&mut self, game: &dyn Game, menu_busy: bool) -> Option<Step> {
        if self.role != Role::Guest {
            return None;
        }
        self.start_join();
        self.start_step(game.local_player(), true, menu_busy)
    }

    /// Guest: leaves the joining phase when the session is ready and the host told its
    /// opening.
    fn start_join(&mut self) {
        if self.start.flow.phase() == Phase::Joining && self.ready {
            if let Some(o) = self.start.flow.opening() {
                self.start.flow.guest_joined(o);
                if self.start.cfg.own == OwnCharacter::Provided {
                    if let Some(s) = self.start.provided.clone() {
                        self.start.sheet_out = Some((s, None));
                    }
                }
            }
        }
    }

    /// Guest: one frame of the start flow. `local` is the game's own player. `host_ready`
    /// is false while the guest must still watch the host's opening.
    fn start_step(
        &mut self,
        local: coop_game::LocalPlayer,
        host_ready: bool,
        menu_busy: bool,
    ) -> Option<Step> {
        let at = self.at_host_place(&local.place);
        let act = self.start.flow.poll_gated(menu_busy, at, host_ready);
        let phase = self.start.flow.phase();
        if self.start.logged != Some(phase) {
            self.start.logged = Some(phase);
            let venue = self.start.flow.venue();
            self.log_ev(
                "start_phase",
                &[("phase", V::S(phase.name())), ("venue", V::S(venue.name()))],
            );
        }
        match act {
            Some(Action::OpenMenu(s)) => Some(s),
            Some(Action::Finished) => {
                if self.start.sheet_out.is_none() {
                    self.start.sheet_out = Some((local.sheet.into(), None));
                }
                None
            }
            None => None,
        }
    }

    /// Guest: the opening told by the host.
    pub(crate) fn on_opening(&mut self, o: Opening) {
        let first = self.start.flow.opening() != Some(o);
        if self.role == Role::Guest {
            // the flow keeps the latest; it starts the character making when ready
            let phase = self.start.flow.phase();
            if phase == Phase::Joining {
                // remember without leaving Joining until the session is ready
                self.start.flow.opening = Some(o);
            } else {
                self.start.flow.guest_joined(o);
            }
        }
        if first {
            self.log_ev("opening", &[("state", V::S(o.name()))]);
        }
    }

    /// Host: a guest's sheet arrived.
    pub(crate) fn on_sheet(&mut self, from: PlayerId, sheet: Sheet) {
        if self.host.is_none() {
            return;
        }
        if self.start.applied.get(&from) == Some(&sheet) {
            // a repeat of what is stored already (the ack was lost)
            send_ext(&mut self.net, from, &Ext::SheetAck);
            return;
        }
        // a new sheet is acknowledged once it is applied and in the store (`host_start_frame`)
        if !self
            .start
            .pending
            .iter()
            .any(|(p, s)| *p == from && *s == sheet)
        {
            self.start.pending.push((from, sheet));
        }
    }

    /// Guest: the host stored the sheet.
    pub(crate) fn on_sheet_ack(&mut self) {
        if self.role == Role::Guest && self.start.sheet_out.is_some() {
            self.start.flow.sheet_acked();
            self.log_ev("character_stored", &[]);
        }
    }

    /// Host, once per frame: tells guests the opening and applies the sheets that came.
    /// `own_name` is the name of the host's own character (empty: no name yet).
    pub(crate) fn host_start_frame(&mut self, game: &mut dyn Game, own_name: &str) {
        if self.host.is_none() {
            return;
        }
        let o = opening_in(&*game, self.start.cfg.new_game);
        self.start.flow.host_opening(o);
        let peers: Vec<PlayerId> = self
            .host
            .as_ref()
            .map(|h| h.peers.keys().copied().collect())
            .unwrap_or_default();
        self.start.told.retain(|p, _| peers.contains(p));
        self.start.names_told.retain(|p, _| peers.contains(p));
        let peers_now = peers.clone();
        for p in peers {
            if self.start.told.get(&p) != Some(&o) {
                self.start.told.insert(p, o);
                send_ext(&mut self.net, p, &Ext::Opening(o));
            }
        }
        for p in std::mem::take(&mut self.start.ack_after) {
            send_ext(&mut self.net, p, &Ext::SheetAck);
        }
        match Some(own_name).filter(|n| !n.is_empty()) {
            Some(n) => {
                self.start.char_names.insert(PlayerId::HOST, n.to_string());
            }
            None => {
                self.start.char_names.remove(&PlayerId::HOST);
            }
        }
        self.start
            .char_names
            .retain(|p, _| p.is_host() || peers_now.contains(p));
        for (from, sheet) in std::mem::take(&mut self.start.pending) {
            if from.is_host() {
                continue;
            }
            let slot = from.0;
            let name = sheet.name.clone();
            if !name.is_empty() {
                self.start.char_names.insert(from, name.clone());
            }
            if !game.add_player(slot) {
                // one shared player: there is no row of its own to keep a character in
                send_ext(&mut self.net, from, &Ext::SheetAck);
                continue;
            }
            let character = Intent::Character(sheet.clone().into());
            if game.run_intent(slot, &character) != Ok(true) {
                // The game did not take the character. The sheet waits for the next frame:
                // the guest gets no acknowledgement for a character that is not stored.
                self.start.pending.push((from, sheet));
                continue;
            }
            self.log_ev(
                "character",
                &[("player", V::U(slot as u64)), ("name", V::S(&name))],
            );
            self.start.applied.insert(from, sheet);
            self.start.ack_after.push(from);
        }
        let names = self.start.char_names.clone();
        for p in peers_now {
            if self.start.names_told.get(&p) != Some(&names) {
                self.start.names_told.insert(p, names.clone());
                let list: Vec<(PlayerId, String)> =
                    names.iter().map(|(k, v)| (*k, v.clone())).collect();
                send_ext(&mut self.net, p, &Ext::CharNames(list));
            }
        }
    }

    /// Guest: the host told the character names.
    pub(crate) fn on_char_names(&mut self, list: Vec<(PlayerId, String)>) {
        if self.role == Role::Guest {
            self.start.char_names = list.into_iter().filter(|(_, n)| !n.is_empty()).collect();
        }
    }

    /// The in-game character name a player chose, when known.
    pub fn character_name(&self, p: PlayerId) -> Option<&str> {
        self.start.char_names.get(&p).map(String::as_str)
    }

    /// Guest, once per tick: resends an unacknowledged sheet.
    pub(crate) fn guest_start_tick(&mut self) {
        let now = self.now_ms;
        let active = self
            .guest
            .as_ref()
            .is_some_and(|g| g.sync.phase() == coop_core::sync::GuestPhase::Active);
        if !active || self.start.flow.acked {
            return;
        }
        if let Some((sheet, sent)) = &mut self.start.sheet_out {
            if sent.map_or(true, |t| now.saturating_sub(t) >= SHEET_RESEND_MS) {
                *sent = Some(now);
                let e = Ext::Character(sheet.clone());
                send_ext(&mut self.net, PlayerId::HOST, &e);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn order() -> (testdata::TempData, esm::LoadOrder) {
        let d = testdata::quests("start-unit");
        let o = esm::LoadOrder::from_data_dir(d.path(), &esm::ActivePlugins::OfficialOnly).unwrap();
        (d, o)
    }

    /// Plays the guest's flow with `busy_for` frames per menu; returns the menus opened in order.
    fn run_guest(mut f: StartFlow, busy_for: u32, at_host_after: u32) -> (StartFlow, Vec<Step>) {
        let mut opened = Vec::new();
        let mut busy = 0u32;
        let mut frames = 0u32;
        for _ in 0..200 {
            frames += 1;
            let at = frames > at_host_after;
            if let Some(Action::OpenMenu(s)) = f.poll(busy > 0, at) {
                opened.push(s);
                busy = busy_for;
            }
            busy = busy.saturating_sub(1);
            if f.finished() {
                f.sheet_acked();
            }
        }
        (f, opened)
    }

    #[test]
    fn host_without_a_new_game_plays_at_once() {
        let f = StartFlow::host(false);
        assert_eq!(
            (f.phase(), f.opening()),
            (Phase::Playing, Some(Opening::Over))
        );
        assert!(f.follow_ok());
    }

    #[test]
    fn host_new_game_makes_the_character_until_the_opening_is_over() {
        let mut f = StartFlow::host(true);
        assert_eq!(f.phase(), Phase::CharacterCreation);
        f.host_opening(Opening::InProgress);
        assert_eq!(f.phase(), Phase::CharacterCreation);
        f.host_opening(Opening::Over);
        assert_eq!(f.phase(), Phase::Playing);
        // the host never gets menus from the flow: the game's scripts open them
        assert_eq!(f.poll(false, true), None);
    }

    #[test]
    fn guest_joins_the_host_in_the_opening_and_makes_its_character_there() {
        let mut f = StartFlow::guest(OwnCharacter::Create);
        assert_eq!(f.poll(false, false), None, "nothing while joining");
        assert!(!f.follow_ok());
        f.guest_joined(Opening::InProgress);
        assert_eq!(
            (f.phase(), f.venue()),
            (Phase::CharacterCreation, Venue::HostPlace)
        );
        assert!(f.follow_ok(), "already at the host's place");
        let (f, opened) = run_guest(f, 3, 0);
        assert_eq!(opened, STEPS.to_vec());
        assert_eq!(f.phase(), Phase::Playing);
    }

    #[test]
    fn guest_joining_after_the_opening_uses_a_local_room_and_then_arrives() {
        let mut f = StartFlow::guest(OwnCharacter::Create);
        f.guest_joined(Opening::Over);
        assert_eq!(
            (f.phase(), f.venue()),
            (Phase::CharacterCreation, Venue::LocalRoom)
        );
        assert!(
            !f.follow_ok(),
            "the guest stays in the room while it makes its character"
        );
        // arrives only after the menus; "at host" turns true at frame 50
        let mut g = f.clone();
        let mut seen = Vec::new();
        for frame in 0..200u32 {
            if let Some(Action::OpenMenu(s)) = g.poll(false, frame >= 50) {
                seen.push(s);
            }
            if g.finished() {
                g.sheet_acked();
            }
            seen.dedup();
        }
        assert_eq!(seen.len(), STEPS.len());
        assert_eq!(g.phase(), Phase::Playing);
        assert!(g.follow_ok());
    }

    #[test]
    fn arriving_waits_for_the_ack_and_for_the_place() {
        let mut f = StartFlow::guest(OwnCharacter::Create);
        f.guest_joined(Opening::Over);
        // run the menus without ever acking
        for _ in 0..50 {
            f.poll(false, true);
        }
        assert_eq!(f.phase(), Phase::Arriving);
        f.poll(false, true);
        assert_eq!(f.phase(), Phase::Arriving, "not stored yet");
        f.sheet_acked();
        f.poll(false, false);
        assert_eq!(f.phase(), Phase::Arriving, "not there yet");
        assert!(f.follow_ok(), "arriving follows the host");
        f.poll(false, true);
        assert_eq!(f.phase(), Phase::Playing);
    }

    #[test]
    fn a_busy_menu_is_waited_for_and_never_opened_twice() {
        let mut f = StartFlow::guest(OwnCharacter::Create);
        f.guest_joined(Opening::InProgress);
        assert_eq!(f.poll(true, true), None, "some other box is up first");
        assert_eq!(f.poll(false, true), Some(Action::OpenMenu(Step::Name)));
        assert_eq!(f.poll(true, true), None);
        assert_eq!(f.poll(true, true), None);
        assert_eq!(f.poll(false, true), Some(Action::OpenMenu(Step::Special)));
    }

    #[test]
    fn a_character_from_a_save_skips_the_menus() {
        let mut f = StartFlow::guest(OwnCharacter::Provided);
        f.guest_joined(Opening::InProgress);
        assert_eq!((f.phase(), f.venue()), (Phase::Arriving, Venue::HostPlace));
        assert_eq!(f.poll(false, true), None);
        f.sheet_acked();
        f.poll(false, true);
        assert_eq!(f.phase(), Phase::Playing);
    }

    #[test]
    fn the_harness_can_skip_the_character_making() {
        let mut f = StartFlow::guest(OwnCharacter::Skip);
        f.guest_joined(Opening::Over);
        assert_eq!(f.phase(), Phase::Playing);
        assert_eq!(f.poll(false, true), None);
    }

    #[test]
    fn the_venue_follows_the_hosts_opening_at_the_time_of_joining() {
        for (o, v) in [
            (Opening::InProgress, Venue::HostPlace),
            (Opening::Over, Venue::LocalRoom),
        ] {
            let mut f = StartFlow::guest(OwnCharacter::Create);
            f.guest_joined(o);
            assert_eq!(f.venue(), v);
            // a later change of the host's opening does not move a guest that already began
            f.guest_joined(if o == Opening::Over {
                Opening::InProgress
            } else {
                Opening::Over
            });
            assert_eq!(f.venue(), v);
        }
    }

    #[test]
    fn opening_of_reads_the_host_rows() {
        let (_d, order) = order();
        let mut gs = GameState::new(&order);
        assert_eq!(opening_of(&gs, false), Opening::Over, "no opening ran");
        assert_eq!(
            opening_of(&gs, true),
            Opening::InProgress,
            "a new game, before the first stage"
        );
        gs.stages.insert(VCG00, 0);
        assert_eq!(opening_of(&gs, false), Opening::InProgress);
        gs.stages.insert(VCG01, 90);
        assert_eq!(opening_of(&gs, false), Opening::InProgress);
        gs.stages.insert(VCG01, OPENING_OVER_STAGE);
        assert_eq!(opening_of(&gs, true), Opening::Over);
    }

    #[test]
    fn a_sheet_is_read_and_applied_to_the_players_rows_only() {
        let (_d, order) = order();
        let mut a = GameState::new(&order);
        a.player_name = Some("Bo".into());
        a.player_female = Some(true);
        for (i, av) in SPECIAL.iter().enumerate() {
            a.actor_values.insert((PLAYER_REF, *av), (i + 3) as f64);
        }
        a.tag_skills = [32u16, 40, 41].into_iter().collect();
        a.perks.insert(FormId(0x1234));
        let s = Sheet::read(&a);
        assert_eq!(s.special, [3, 4, 5, 6, 7, 8, 9]);
        assert_eq!(s.tags, vec![32, 40, 41]);
        let mut b = GameState::new(&order);
        s.apply(&mut b);
        assert_eq!(
            Sheet {
                perks: vec![0x1234],
                ..s.clone()
            }
            .perks,
            vec![0x1234]
        );
        assert_eq!(b.player_name.as_deref(), Some("Bo"));
        assert_eq!(b.actor_values.get(&(PLAYER_REF, 11)), Some(&9.0));
        assert!(b.perks.contains(&FormId(0x1234)));
        assert_eq!(b.stages.get(&VCG00), Some(&VCG00_DONE));
        assert_eq!(b.stages.get(&VCG01), Some(&OPENING_OVER_STAGE));
        // applying never lowers a stage
        b.stages.insert(VCG01, 150);
        s.apply(&mut b);
        assert_eq!(b.stages.get(&VCG01), Some(&150));
    }

    #[test]
    fn the_dump_object_names_the_phase() {
        let i = StartInfo {
            phase: Phase::CharacterCreation,
            venue: Venue::LocalRoom,
            opening: Some(Opening::Over),
            sheet_acked: false,
            step: Some(Step::Special),
        };
        assert_eq!(
            i.json(),
            "{\"phase\":\"character_creation\",\"venue\":\"local_room\",\"opening\":\"over\",\"sheet_acked\":false,\"step\":\"special\"}"
        );
    }
}

#[cfg(test)]
mod game_tests {
    use coop_core::schema::Field;
    use coop_core::transport::LoopbackNet;
    use coop_game::fake::{Call, FakeGame};
    use coop_game::{Players, Role as GameRole, Setup};

    use super::*;
    use crate::session::SessionConfig;

    fn host_session() -> Session {
        let net = LoopbackNet::new();
        let cfg = SessionConfig {
            transport: Some(Box::new(net.endpoint(PlayerId::HOST))),
            session_id: 7,
            ..SessionConfig::default()
        };
        Session::host(cfg).expect("a session")
    }

    fn host_game() -> FakeGame {
        let mut game = FakeGame::new();
        let setup = Setup {
            role: GameRole::Host,
            me: 0,
            policy: None,
            keep_own_rows: false,
        };
        game.configure(&setup).unwrap();
        game
    }

    fn sheet(name: &str) -> Sheet {
        Sheet {
            name: name.into(),
            female: Some(true),
            special: [3, 4, 5, 6, 7, 8, 9],
            tags: vec![32, 40, 41],
            perks: vec![0x1234],
        }
    }

    fn stage_of(quest: coop_game::FormId, stage: u16) -> (coop_game::FormId, Field) {
        (quest, Field::QuestStage { stage })
    }

    #[test]
    fn the_opening_comes_from_the_stages_of_the_live_player() {
        let mut game = host_game();
        assert_eq!(opening_in(&game, false), Opening::Over, "no opening ran");
        assert_eq!(opening_in(&game, true), Opening::InProgress);
        // flags and no stage: the quest did not start a stage (`quest` gives stage 0 here)
        let running = Field::QuestFlags {
            flags: flag::RUNNING,
        };
        game.set(opening::VCG00, running);
        assert_eq!(opening_in(&game, false), Opening::Over);
        // the twin: stage 0 is set, so the opening runs
        let (quest, field) = stage_of(opening::VCG00, 0);
        game.set(quest, field);
        assert_eq!(opening_in(&game, false), Opening::InProgress);
        let (quest, field) = stage_of(opening::VCG01, OPENING_OVER_STAGE - 1);
        game.set(quest, field);
        assert_eq!(opening_in(&game, true), Opening::InProgress);
        let (quest, field) = stage_of(opening::VCG01, OPENING_OVER_STAGE);
        game.set(quest, field);
        assert_eq!(opening_in(&game, true), Opening::Over);
        // a completed quest ends the opening at each stage
        let mut game = host_game();
        let (quest, field) = stage_of(opening::VCG01, 10);
        game.set(quest, field);
        assert_eq!(opening_in(&game, false), Opening::InProgress);
        let completed = Field::QuestFlags {
            flags: flag::COMPLETED,
        };
        game.set(opening::VCG01, completed);
        assert_eq!(opening_in(&game, false), Opening::Over);
    }

    #[test]
    fn the_host_gives_a_guests_sheet_to_that_players_rows() {
        let mut host = host_session();
        let mut game = host_game();
        host.start.pending.push((PlayerId(1), sheet("Bo")));
        host.host_start_frame(&mut game, "Host");
        // the character is in the rows of player 1, and the local player did not change
        let stored: Sheet = game.sheet_of(1).expect("rows of player 1").clone().into();
        assert_eq!(stored, sheet("Bo"));
        assert_eq!(game.local_player().sheet, coop_game::Sheet::default());
        assert_eq!(host.start.applied.get(&PlayerId(1)), Some(&sheet("Bo")));
        assert_eq!(host.start.ack_after, [PlayerId(1)]);
        assert!(host.start.pending.is_empty());
        assert_eq!(host.character_name(PlayerId(1)), Some("Bo"));
        assert_eq!(host.character_name(PlayerId::HOST), Some("Host"));
        // the player is added before its character comes
        let order: Vec<&Call> = game
            .calls()
            .iter()
            .filter(|c| matches!(c, Call::AddPlayer { .. } | Call::Intent { .. }))
            .collect();
        assert!(matches!(
            order[0],
            Call::AddPlayer {
                slot: 1,
                kept: true
            }
        ));
        assert!(matches!(
            order[1],
            Call::Intent {
                slot: 1,
                answer: Ok(true),
                ..
            }
        ));
        assert_eq!(order.len(), 2);
    }

    #[test]
    fn a_game_with_one_shared_player_gets_no_character_of_a_guest() {
        let mut host = host_session();
        let mut game = host_game();
        game.one_shared_player();
        host.start.pending.push((PlayerId(1), sheet("Bo")));
        host.host_start_frame(&mut game, "");
        // no intent went to the game: the shared character is the host's
        assert_eq!(game.intents_run(), 0);
        assert_eq!(game.local_player().sheet, coop_game::Sheet::default());
        assert!(host.start.applied.is_empty() && host.start.ack_after.is_empty());
        assert!(host.start.pending.is_empty(), "the sheet got its answer");
        // an empty name of the host's own character is no name
        assert_eq!(host.character_name(PlayerId::HOST), None);
    }

    #[test]
    fn a_sheet_that_the_game_refuses_waits_for_the_next_frame() {
        let mut host = host_session();
        // a game with no role refuses each intent
        let mut game = FakeGame::new();
        host.start.pending.push((PlayerId(1), sheet("Bo")));
        host.host_start_frame(&mut game, "Host");
        assert!(host.start.applied.is_empty() && host.start.ack_after.is_empty());
        assert_eq!(host.start.pending, [(PlayerId(1), sheet("Bo"))]);
        // the twin: the game takes its role, and the same sheet is stored
        let mut game = host_game();
        host.host_start_frame(&mut game, "Host");
        assert_eq!(host.start.applied.get(&PlayerId(1)), Some(&sheet("Bo")));
        assert!(host.start.pending.is_empty());
    }

    #[test]
    fn a_sheet_goes_to_the_interface_and_comes_back_equal() {
        let mine = sheet("Bo");
        let theirs: coop_game::Sheet = mine.clone().into();
        assert_eq!(theirs.perks, [coop_game::FormId(0x1234)]);
        assert_eq!(Sheet::from(theirs), mine);
    }
}

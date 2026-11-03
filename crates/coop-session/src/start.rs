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

use std::collections::{BTreeMap, BTreeSet};

use coop_core::ids::PlayerId;
use esm::FormId;
use world::dialogue::PLAYER_REF;
use world::scripting::GameState;

use crate::ext::Ext;
use crate::json::V;
use crate::session::{send_ext, Session};
use crate::Role;

/// `VCG00`, the prologue quest ("Welcome To Fabulous New Vegas").
pub const VCG00: FormId = FormId(0x0010_2037);
/// `VCG01`, Doc Mitchell's intro quest.
pub const VCG01: FormId = FormId(0x0010_4C1C);
/// `VCG00` stage that ends the prologue.
pub const VCG00_DONE: u16 = 100;
/// `VCG01` stage from which the character is made (Doc's farewell transition, after the traits).
pub const OPENING_OVER_STAGE: u16 = 105;
/// Where a guest makes its character when the host's opening is over.
pub const LOCAL_ROOM: &str = "GSDocMitchellHouse";
/// SPECIAL points the Vit-o-Matic hands out (`ShowLoveTesterMenuParams 40`).
pub const SPECIAL_POINTS: u32 = 40;
/// Skills to tag (`SetTagSkills 3 1`).
pub const TAG_COUNT: u32 = 3;
/// Traits to pick at most (`ShowTraitMenu`).
pub const TRAIT_MAX: u32 = 2;
/// The SPECIAL attributes' actor values (Strength .. Luck).
pub const SPECIAL: [u16; 7] = [5, 6, 7, 8, 9, 10, 11];
/// A SPECIAL value nobody changed.
const SPECIAL_DEFAULT: u8 = 5;
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

    /// One frame. `menu_busy`: some menu is up or queued here. `at_host`: the guest is in the
    /// host's place. Returns what to do now.
    pub fn poll(&mut self, menu_busy: bool, at_host: bool) -> Option<Action> {
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
                if !self.opened && !menu_busy {
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
    pub fn read(gs: &GameState) -> Sheet {
        let mut special = [SPECIAL_DEFAULT; 7];
        for (i, av) in SPECIAL.iter().enumerate() {
            if let Some(v) = gs.actor_values.get(&(PLAYER_REF, *av)) {
                special[i] = v.round().clamp(1.0, 10.0) as u8;
            }
        }
        let mut perks: Vec<u32> = gs.perks.iter().map(|f| f.0).collect();
        perks.sort_unstable();
        Sheet {
            name: gs.player_name.clone().unwrap_or_default(),
            female: gs.player_female,
            special,
            tags: gs.tag_skills.iter().copied().collect(),
            perks,
        }
    }

    /// Puts the character into `gs`'s player rows and marks the opening done for that player
    /// (`VCG00` and `VCG01` rows, without running any stage script).
    pub fn apply(&self, gs: &mut GameState) {
        if !self.name.is_empty() {
            gs.player_name = Some(self.name.clone());
        }
        if self.female.is_some() {
            gs.player_female = self.female;
        }
        for (i, av) in SPECIAL.iter().enumerate() {
            gs.actor_values
                .insert((PLAYER_REF, *av), f64::from(self.special[i]));
        }
        gs.tag_skills = self.tags.iter().copied().collect::<BTreeSet<u16>>();
        for p in &self.perks {
            gs.perks.insert(FormId(*p));
        }
        let at_least = |gs: &mut GameState, q: FormId, st: u16| {
            let cur = gs.stages.get(&q).copied().unwrap_or(0);
            if cur < st {
                gs.stages.insert(q, st);
            }
        };
        at_least(gs, VCG00, VCG00_DONE);
        at_least(gs, VCG01, OPENING_OVER_STAGE);
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
    /// The last phase logged.
    pub logged: Option<Phase>,
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
            logged: None,
        }
    }
}

/// The host's opening from its own quest rows.
pub fn opening_of(gs: &GameState, new_game: bool) -> Opening {
    let st = |q: FormId| gs.stages.get(&q).copied();
    let finished =
        st(VCG01).is_some_and(|s| s >= OPENING_OVER_STAGE) || gs.completed.contains(&VCG01);
    if finished {
        return Opening::Over;
    }
    let started = st(VCG00).is_some() || st(VCG01).is_some();
    if started || new_game {
        Opening::InProgress
    } else {
        Opening::Over
    }
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

    fn at_host_place(&self, gs: &GameState) -> bool {
        let Some(p) = self.host_place_for_start() else {
            return false;
        };
        if p.world != 0 {
            gs.player_world.map(|w| w.0) == Some(p.world)
        } else {
            gs.player_world.is_none() && gs.player_cell.map(|c| c.0) == Some(p.cell)
        }
    }

    fn host_place_for_start(&self) -> Option<crate::ext::Place> {
        self.guest.as_ref().and_then(|g| g.place)
    }

    /// Guest: once per frame, after the world is loaded. `menu_busy`: a menu is up or queued.
    /// Returns the menu to open now (local only), if any. The host always gets `None`.
    pub fn start_poll(&mut self, gs: &GameState, menu_busy: bool) -> Option<Step> {
        if self.role != Role::Guest {
            return None;
        }
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
        let at = self.at_host_place(gs);
        let act = self.start.flow.poll(menu_busy, at);
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
                    self.start.sheet_out = Some((Sheet::read(gs), None));
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
        send_ext(&mut self.net, from, &Ext::SheetAck);
        if self.start.applied.get(&from) != Some(&sheet)
            && !self
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
    pub(crate) fn host_start_frame(&mut self, gs: &mut GameState) {
        if self.host.is_none() {
            return;
        }
        let o = opening_of(gs, self.start.cfg.new_game);
        self.start.flow.host_opening(o);
        let peers: Vec<PlayerId> = self
            .host
            .as_ref()
            .map(|h| h.peers.keys().copied().collect())
            .unwrap_or_default();
        self.start.told.retain(|p, _| peers.contains(p));
        for p in peers {
            if self.start.told.get(&p) != Some(&o) {
                self.start.told.insert(p, o);
                send_ext(&mut self.net, p, &Ext::Opening(o));
            }
        }
        for (from, sheet) in std::mem::take(&mut self.start.pending) {
            if from.is_host() {
                continue;
            }
            let slot = from.0;
            let name = sheet.name.clone();
            if gs.coop.bank.enabled() {
                gs.register_player(slot);
                gs.with_player(slot, |g| sheet.apply(g));
            } else {
                // one shared player: there is no row of its own to keep a character in
                continue;
            }
            self.log_ev(
                "character",
                &[("player", V::U(slot as u64)), ("name", V::S(&name))],
            );
            self.start.applied.insert(from, sheet);
        }
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

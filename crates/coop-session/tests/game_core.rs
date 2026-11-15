//! The session core on games that are not the `world` layer: a host and a guest, each on
//! a `coop_game::fake::FakeGame`, over the in-process transport.
//!
//! The peers use only `Session::pre_frame_game`, `Session::post_frame_game` and
//! `Session::start_poll_game`. This file names no type of the `world` crate: it is the
//! contract of a game program that has no `GameState`.
//!
//! Each row is a function. With `mutate` set it breaks its own scenario, and then it must
//! fail for its own cause: a twin that passes shows that the row checks nothing.

use coop_core::ids::PlayerId;
use coop_core::schema::flag;
use coop_core::transport::LoopbackNet;
use coop_game::fake::{Call, FakeGame};
use coop_game::{Field, FormId, Intent, Key, Place, Story};
use coop_session::{Command, OwnCharacter, Phase, Session, SessionConfig, Sheet, StartConfig};

const DT: f32 = 0.025;
/// In the second plugin, so the load index is not 0.
const QUEST: FormId = FormId(0x0100_0A03);
const GLOBAL: FormId = FormId(0x0000_0A00);
const CELL: FormId = FormId(0x0010_4C0F);
const STAGE: u16 = 10;
const LINE: &str = "set FakeGlobal to 7";

fn stage_key() -> Key {
    Field::QuestStage { stage: 0 }.key(QUEST)
}

fn global_key() -> Key {
    Field::global(0.0).key(GLOBAL)
}

fn objective() -> Field {
    Field::Objective {
        index: STAGE,
        state: 1,
    }
}

fn set_stage() -> Intent {
    Intent::SetStage {
        quest: QUEST,
        stage: STAGE,
    }
}

/// A game with one quest and one global. Stage 10 of the quest has a script: it shows an
/// objective and sets the running flag. The script line sets the global.
fn game() -> FakeGame {
    let mut g = FakeGame::new();
    g.plugin("Base.esm");
    g.plugin("Story.esm");
    g.name("FakeQuest", QUEST);
    g.name("FakeGlobal", GLOBAL);
    g.set(GLOBAL, Field::global(5.0));
    let running = Field::QuestFlags {
        flags: flag::RUNNING,
    };
    let writes = [Field::QuestStage { stage: STAGE }, objective(), running];
    g.script(
        set_stage(),
        writes
            .iter()
            .map(|f| (f.key(QUEST), Some(f.clone())))
            .collect(),
    );
    let seven = Field::global(7.0);
    g.script(
        Intent::ScriptLine(LINE.into()),
        vec![(seven.key(GLOBAL), Some(seven))],
    );
    g
}

struct Peer {
    s: Session,
    game: FakeGame,
}

struct Rig {
    // The network lives as long as its endpoints.
    _net: LoopbackNet,
    host: Peer,
    guest: Peer,
    /// The guest's session runs with no game: it calls `Session::poll` only.
    guest_has_no_game: bool,
    /// The host takes each command and drops it before the core sees it.
    host_drops_commands: bool,
}

fn config(name: &str, net: &LoopbackNet, id: PlayerId) -> SessionConfig {
    SessionConfig {
        name: name.into(),
        session_id: 0xC0DE,
        transport: Some(Box::new(net.endpoint(id))),
        ..SessionConfig::default()
    }
}

impl Rig {
    fn new() -> Rig {
        let net = LoopbackNet::new();
        let host = Session::host(config("Host", &net, PlayerId::HOST)).expect("a host");
        let mut guest = Session::join(config("Guest", &net, PlayerId(1))).expect("a guest");
        // No menus: the guest plays when it is ready.
        guest.set_start_config(StartConfig {
            new_game: false,
            own: OwnCharacter::Skip,
        });
        Rig {
            _net: net,
            host: Peer {
                s: host,
                game: game(),
            },
            guest: Peer {
                s: guest,
                game: game(),
            },
            guest_has_no_game: false,
            host_drops_commands: false,
        }
    }

    /// One frame of each peer, as a game program runs it.
    fn frame(&mut self) {
        self.host.s.pre_frame_game(&mut self.host.game, true);
        if self.guest_has_no_game {
            self.guest.s.poll(DT);
        } else {
            self.guest.s.pre_frame_game(&mut self.guest.game, true);
            self.guest.s.start_poll_game(&self.guest.game, false);
        }
        if self.host_drops_commands {
            self.host.s.take_commands();
        }
        self.host.s.post_frame_game(&mut self.host.game, DT);
        if !self.guest_has_no_game {
            self.guest.s.post_frame_game(&mut self.guest.game, DT);
        }
    }

    fn frames(&mut self, n: u32) {
        for _ in 0..n {
            self.frame();
        }
    }

    /// Runs frames until the guest is in the session, then some more.
    fn join(&mut self) -> Result<(), String> {
        for _ in 0..800 {
            self.frame();
            if self.host.s.is_ready() && self.guest.s.is_connected() {
                self.frames(40);
                return Ok(());
            }
        }
        Err("the guest did not join in 800 frames".into())
    }

    /// The calls of `Story::run_intent` that a game got.
    fn intents(game: &FakeGame) -> Vec<&Call> {
        game.calls()
            .iter()
            .filter(|c| matches!(c, Call::Intent { .. }))
            .collect()
    }
}

/// A row passes, and its twin fails for the cause that the twin is for.
fn check(row: fn(bool) -> Result<(), String>, twin_cause: &str) {
    assert_eq!(row(false), Ok(()));
    let twin = row(true).expect_err("the twin passed");
    assert!(
        twin.contains(twin_cause),
        "the twin failed for another cause: {twin}"
    );
}

/// The guest joins, and its game gets the host's state: the two exports are equal.
///
/// Twin: the guest's session runs with no game.
fn a_guest_joins_and_gets_the_hosts_snapshot(mutate: bool) -> Result<(), String> {
    let mut rig = Rig::new();
    rig.guest_has_no_game = mutate;
    // State that only the host has before the join.
    rig.host.game.set(GLOBAL, Field::global(12.0));
    rig.host.game.set(QUEST, Field::QuestStage { stage: 5 });
    rig.join()?;
    if rig.guest.game.fields() != rig.host.game.fields() {
        return Err(format!(
            "the guest's export differs from the host's: {:?}, want {:?}",
            rig.guest.game.fields(),
            rig.host.game.fields()
        ));
    }
    if !rig.guest.s.is_ready() {
        return Err("the guest is not ready after the snapshot".into());
    }
    if rig.guest.s.start_phase() != Phase::Playing {
        return Err(format!(
            "the guest's start phase is {:?}",
            rig.guest.s.start_phase()
        ));
    }
    let status = rig.guest.s.status();
    if (status.player, status.snapshots) != (PlayerId(1), 1) {
        return Err(format!(
            "the guest is player {:?} with {} snapshots",
            status.player, status.snapshots
        ));
    }
    Ok(())
}

/// The host's game runs a stage with its own story code, and the guest's game gets each
/// slot that the script wrote, as a delta and with no script run.
///
/// Twin: the host's game does not run the stage.
fn the_host_runs_a_stage_and_the_guest_gets_the_delta(mutate: bool) -> Result<(), String> {
    let mut rig = Rig::new();
    rig.join()?;
    if rig.guest.game.field(stage_key()).is_some() {
        return Err("the guest holds a stage before the host ran it".into());
    }
    if !mutate {
        let answer = rig.host.game.run_intent(0, &set_stage());
        if answer != Ok(true) {
            return Err(format!("the host's game answered {answer:?}"));
        }
    }
    rig.frames(40);
    let want = Field::QuestStage { stage: STAGE };
    if rig.guest.game.field(stage_key()) != Some(&want) {
        return Err(format!(
            "the guest's stage is {:?}, want {want:?}",
            rig.guest.game.field(stage_key())
        ));
    }
    if rig.guest.game.field(objective().key(QUEST)) != Some(&objective()) {
        return Err("the guest did not get the objective that the stage's script shows".into());
    }
    if rig.guest.game.fields() != rig.host.game.fields() {
        return Err("the guest's export differs from the host's after the delta".into());
    }
    if rig.guest.game.intents_run() != 0 {
        return Err("the guest's game ran story code".into());
    }
    let snapshots = rig.guest.s.status().snapshots;
    if snapshots != 1 {
        return Err(format!(
            "the change came with {snapshots} snapshots, want the one of the join"
        ));
    }
    Ok(())
}

/// A guest sends a stage and a script line. The host's game runs each one time, with the
/// rows of the guest's player, and the guest's game gets the result.
///
/// Twin: the host's program takes the commands and drops them.
fn a_guests_command_runs_once_on_the_host(mutate: bool) -> Result<(), String> {
    let mut rig = Rig::new();
    rig.host_drops_commands = mutate;
    rig.join()?;
    rig.guest
        .s
        .send_command(Command::Stage("FakeQuest".into(), STAGE));
    rig.guest.s.send_command(Command::Line(LINE.into()));
    // Longer than the time after which a guest sends a command again.
    rig.frames(200);
    let ran: Vec<(u8, Intent)> = Rig::intents(&rig.host.game)
        .into_iter()
        .filter_map(|c| match c {
            Call::Intent {
                slot,
                intent,
                answer: Ok(true),
            } => Some((*slot, intent.clone())),
            _ => None,
        })
        .collect();
    let want = vec![(1, set_stage()), (1, Intent::ScriptLine(LINE.into()))];
    if ran != want {
        return Err(format!(
            "the host's game ran {ran:?}, want each command one time: {want:?}"
        ));
    }
    if rig.guest.s.status().pending_commands != 0 {
        return Err("the guest still waits for an acknowledgement".into());
    }
    let stage = Field::QuestStage { stage: STAGE };
    let seven = Field::global(7.0);
    if rig.guest.game.field(stage_key()) != Some(&stage)
        || rig.guest.game.field(global_key()) != Some(&seven)
    {
        return Err(format!(
            "the guest does not see the result: stage {:?}, global {:?}",
            rig.guest.game.field(stage_key()),
            rig.guest.game.field(global_key())
        ));
    }
    Ok(())
}

/// A command for a quest that the host's game does not know runs nothing, and the host's
/// own command runs with the host's rows.
///
/// Twin: the command names a quest that the game knows.
fn a_command_for_an_unknown_quest_runs_nothing(mutate: bool) -> Result<(), String> {
    let mut rig = Rig::new();
    rig.join()?;
    let quest = if mutate { "FakeQuest" } else { "NoSuchQuest" };
    rig.guest
        .s
        .send_command(Command::Stage(quest.into(), STAGE));
    rig.frames(120);
    let stages = Rig::intents(&rig.host.game).len();
    if stages != 0 {
        return Err(format!(
            "the host's game got {stages} intents for a quest with no form"
        ));
    }
    // The host's own command goes the same way, as player 0.
    rig.host.s.send_command(Command::Line(LINE.into()));
    rig.frames(4);
    match Rig::intents(&rig.host.game).as_slice() {
        [Call::Intent {
            slot: 0,
            answer: Ok(true),
            ..
        }] => Ok(()),
        other => Err(format!("the host's own command gave {other:?}")),
    }
}

/// A guest's game gets no intent: not at the join, not for the host's stage, and not for
/// the guest's own command.
///
/// Twin: the row counts the intents of the host's game.
fn a_guests_game_runs_no_intent(mutate: bool) -> Result<(), String> {
    let mut rig = Rig::new();
    rig.join()?;
    let answer = rig.host.game.run_intent(0, &set_stage());
    if answer != Ok(true) {
        return Err(format!("the host's game answered {answer:?}"));
    }
    rig.guest.s.send_command(Command::Line(LINE.into()));
    rig.frames(120);
    if rig.guest.game.field(global_key()) != Some(&Field::global(7.0)) {
        return Err("the guest's command did not come back as state".into());
    }
    let watched = if mutate {
        &rig.host.game
    } else {
        &rig.guest.game
    };
    let got = Rig::intents(watched).len();
    if got != 0 {
        return Err(format!("the game got {got} intents, want none"));
    }
    if !rig.guest.s.take_commands().is_empty() {
        return Err("a guest's session queued a command for its own game".into());
    }
    Ok(())
}

/// The host tells the guest its place. A guest that brings a character sends it, the host's
/// game stores it in the rows of that player, and the guest plays when it is at the place.
///
/// Twin: the guest's game stands in another cell.
fn a_guests_character_reaches_the_hosts_game(mutate: bool) -> Result<(), String> {
    let mut rig = Rig::new();
    let sheet = Sheet {
        name: "Bo".into(),
        female: Some(true),
        special: [3, 4, 5, 6, 7, 8, 9],
        tags: vec![32, 40, 41],
        perks: vec![0x1234],
    };
    rig.guest.s.set_start_config(StartConfig {
        new_game: false,
        own: OwnCharacter::Provided,
    });
    rig.guest.s.set_own_sheet(sheet.clone());
    let here = Place {
        cell: Some(CELL),
        world: None,
        pos: Some([1.0, 2.0, 3.0]),
        heading: 0.0,
    };
    rig.host.game.set_place(here);
    rig.guest.game.set_place(Place {
        cell: Some(if mutate { FormId(0x0010_4C10) } else { CELL }),
        ..here
    });
    rig.join()?;
    rig.frames(120);
    let told = rig.guest.s.host_place().map(|p| (p.cell, p.world));
    if told != Some((CELL.0, 0)) {
        return Err(format!("the guest was told the place {told:?}"));
    }
    let stored: Option<Sheet> = rig.host.game.sheet_of(1).cloned().map(Sheet::from);
    if stored.as_ref() != Some(&sheet) {
        return Err(format!(
            "the host's game holds {stored:?} for player 1, want {sheet:?}"
        ));
    }
    if rig.host.s.character_name(PlayerId(1)) != Some("Bo")
        || rig.guest.s.character_name(PlayerId(1)) != Some("Bo")
    {
        return Err("the character's name did not reach each peer".into());
    }
    if rig.guest.s.start_phase() != Phase::Playing {
        return Err(format!(
            "the guest's start phase is {:?}, want Playing",
            rig.guest.s.start_phase()
        ));
    }
    Ok(())
}

#[test]
fn guest_joins_and_gets_the_snapshot() {
    check(
        a_guest_joins_and_gets_the_hosts_snapshot,
        "the guest's export differs from the host's",
    );
}

#[test]
fn host_stage_reaches_the_guest_as_a_delta() {
    check(
        the_host_runs_a_stage_and_the_guest_gets_the_delta,
        "the guest's stage is None",
    );
}

#[test]
fn guest_command_runs_once_on_the_host() {
    check(
        a_guests_command_runs_once_on_the_host,
        "the host's game ran []",
    );
}

#[test]
fn command_for_an_unknown_quest_runs_nothing() {
    check(
        a_command_for_an_unknown_quest_runs_nothing,
        "intents for a quest with no form",
    );
}

#[test]
fn guest_game_runs_no_intent() {
    check(a_guests_game_runs_no_intent, "the game got 2 intents");
}

#[test]
fn guest_character_reaches_the_host_game() {
    check(
        a_guests_character_reaches_the_hosts_game,
        "the guest's start phase is Arriving",
    );
}

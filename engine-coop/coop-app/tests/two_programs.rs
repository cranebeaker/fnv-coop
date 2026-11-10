//! Two programs in one session: a host and a guest, each a [`coop_app::App`] on a fake game
//! program, over the in-process transport.
//!
//! The session, the engine adapter and the program's loop are the real code. Only the game
//! program is a fake. Each row is a function. With `mutate` set it breaks its own scenario,
//! and then it must fail for its own cause: a twin that passes shows that the row checks
//! nothing.

use std::io::{BufRead, BufReader, Write};
use std::net::TcpStream;
use std::time::Duration;

use coop_app::fake::{self, FakeMachine, CELL, HOUR, OWN_QUEST, QUEST};
use coop_app::{args, App, Flow};
use coop_core::ids::{FormId, PlayerId};
use coop_core::schema::Field;
use coop_core::transport::LoopbackNet;
use coop_engine::addr;
use coop_engine::game::NameTable;
use coop_engine::gate::Intent;
use coop_engine::Program;

/// The time of one step, in seconds.
const DT: f32 = 0.025;
/// The frames that a change needs to reach the other program, with a margin. The guest
/// checks its whole state against the host's each 2 seconds (80 frames).
const SETTLE: u32 = 200;

const HOST: &[&str] = &[
    "D:/Data",
    "GSDocMitchellHouse",
    "--coop-host",
    "1",
    "--coop-test-token",
];
const GUEST: &[&str] = &[
    "D:/Data",
    "--coop-join",
    "127.0.0.1:1",
    "--coop-test-token",
    "--coop-no-chargen",
];

fn flags(base: &[&str], more: &[&str]) -> args::Args {
    let all: Vec<String> = base.iter().chain(more).map(|s| s.to_string()).collect();
    args::parse(&all).expect("the command line")
}

struct Rig {
    // The network lives as long as its endpoints.
    _net: LoopbackNet,
    host: App<FakeMachine>,
    guest: App<FakeMachine>,
}

impl Rig {
    fn new(host_more: &[&str], guest_more: &[&str]) -> Rig {
        Rig::with(host_more, guest_more, fake::names())
    }

    fn with(host_more: &[&str], guest_more: &[&str], guest_names: NameTable) -> Rig {
        let net = LoopbackNet::new();
        let host = App::new(
            flags(HOST, host_more),
            FakeMachine::game(),
            fake::names(),
            &fake::files(),
            Some(Box::new(net.endpoint(PlayerId::HOST))),
        )
        .expect("a host");
        let guest = App::new(
            flags(GUEST, guest_more),
            FakeMachine::game(),
            guest_names,
            &fake::files(),
            Some(Box::new(net.endpoint(PlayerId(1)))),
        )
        .expect("a guest");
        Rig {
            _net: net,
            host,
            guest,
        }
    }

    /// One frame of each program. The flows of the two steps.
    fn frame(&mut self) -> (Flow, Flow) {
        (self.host.step(DT), self.guest.step(DT))
    }

    fn frames(&mut self, n: u32) {
        for _ in 0..n {
            self.frame();
        }
    }

    /// Runs frames until each program is ready, then some more.
    fn join(&mut self) -> Result<(), String> {
        for _ in 0..1500 {
            self.frame();
            if self.host.ready() && self.guest.ready() {
                self.frames(SETTLE);
                return Ok(());
            }
        }
        Err(format!(
            "the session was not ready in 1500 frames: host {}, guest {}, the guest's first \
             frame of play: {:?}",
            self.host.ready(),
            self.guest.ready(),
            self.guest.machine.first_play
        ))
    }
}

fn stage_of(app: &mut App<FakeMachine>, quest: u32) -> Option<u16> {
    app.export()
        .into_iter()
        .find_map(|(form, field)| match field {
            Field::QuestStage { stage } if form == FormId(quest) => Some(stage),
            _ => None,
        })
}

fn global_of(app: &mut App<FakeMachine>, global: u32) -> Option<f32> {
    app.export()
        .into_iter()
        .find_map(|(form, field)| match field {
            Field::Global { bits } if form == FormId(global) => Some(f32::from_bits(bits)),
            _ => None,
        })
}

/// The value of `"shared":"..."` in a dump.
fn shared_hash(dump: &str) -> String {
    let at = dump.find("\"shared\":\"").expect("the dump has a hash") + 10;
    dump[at..at + 16].to_string()
}

/// The text of one section of a dump: from its key to the next section.
fn section<'a>(dump: &'a str, key: &str, next: &str) -> &'a str {
    let from = dump.find(&format!("\"{key}\":")).expect("the section");
    let to = dump
        .find(&format!("\"{next}\":"))
        .expect("the next section");
    &dump[from..to]
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

/// A guest's game gets its gate at the main menu, before a frame of play. The gate has the
/// policy's rules. Then the guest plays in the host's cell.
///
/// Twin: the guest's name table does not know the host's cell.
fn a_guest_takes_its_gate_at_the_menu_and_plays_in_the_hosts_cell(
    mutate: bool,
) -> Result<(), String> {
    let names = if mutate {
        NameTable::parse("P 00 FalloutNV.esm\nF QUST 0015D912 VCG03\nF GLOB 00000038 GameHour\n")
            .unwrap()
    } else {
        fake::names()
    };
    let mut rig = Rig::with(&[], &[], names);
    let joined = rig.join();
    let guest = &rig.guest.machine;
    let (Some(gate_at), Some(first_play)) = (guest.gate_at, guest.first_play) else {
        return Err(format!(
            "the guest did not reach play with a gate: gate at {:?}, play at {:?}",
            guest.gate_at, guest.first_play
        ));
    };
    joined?;
    if gate_at >= first_play {
        return Err(format!(
            "the gate came at frame {gate_at}, and play started at frame {first_play}"
        ));
    }
    if guest.p.gates.len() != 1 {
        return Err(format!(
            "the guest's game took {} gates",
            guest.p.gates.len()
        ));
    }
    if !guest.p.gates[0].local_quests.contains(&OWN_QUEST) {
        return Err("the gate does not have the policy's per-player quest".into());
    }
    if !rig.host.machine.p.gates.is_empty() {
        return Err("the host's game took a gate".into());
    }
    let player = guest.p.u32(addr::PLAYER);
    let cell = guest
        .p
        .u32(guest.p.u32(player + addr::REF_CELL) + addr::FORM_ID);
    if cell != CELL {
        return Err(format!("the guest plays in the cell {cell:08X}"));
    }
    if guest.gate_frames == 0 {
        return Err("the gate got no frame of play".into());
    }
    Ok(())
}

/// The host's stage and the host's script line reach the guest as plain state: the guest's
/// game has the values, and it ran no stage.
///
/// Twin: the host's command line has no stage and no line.
fn the_hosts_story_reaches_the_guest_as_plain_state(mutate: bool) -> Result<(), String> {
    let more: &[&str] = if mutate {
        &[]
    } else {
        &["--stage", "VCG03", "10", "--run", "set GameHour to 7"]
    };
    let mut rig = Rig::new(more, &[]);
    rig.join()?;
    let stage = stage_of(&mut rig.guest, QUEST);
    if stage != Some(10) {
        return Err(format!(
            "the guest's stage of VCG03 is {stage:?}, want Some(10)"
        ));
    }
    let hour = global_of(&mut rig.guest, HOUR);
    if hour != Some(7.0) {
        return Err(format!("the guest's GameHour is {hour:?}, want Some(7.0)"));
    }
    if rig.host.machine.p.stages_set.len() != 1 {
        return Err(format!(
            "the host's game ran {} stages, want 1",
            rig.host.machine.p.stages_set.len()
        ));
    }
    if !rig.guest.machine.p.stages_set.is_empty() {
        return Err("the guest's game ran a stage: the state did not come as plain values".into());
    }
    Ok(())
}

/// A change that a guest's game wanted goes to the host, and the host's game makes it with
/// its own function. A guest's script line runs on the host's game too. Each game then has
/// the result.
///
/// Twin: the guest's gate noted no intent.
fn a_guests_intent_and_line_run_on_the_hosts_game(mutate: bool) -> Result<(), String> {
    let mut rig = Rig::new(&[], &["--run", "set GameHour to 9"]);
    rig.join()?;
    if !mutate {
        rig.guest.machine.wants(Intent::Stage {
            quest: QUEST,
            stage: 20,
        });
    }
    rig.frames(SETTLE);
    let host_ran: Vec<u8> = rig
        .host
        .machine
        .p
        .stages_set
        .iter()
        .map(|(_, stage)| *stage)
        .collect();
    if host_ran != [20] {
        return Err(format!(
            "the host's game did not get the stage 20: it ran {host_ran:?}"
        ));
    }
    if !rig.guest.machine.p.stages_set.is_empty() {
        return Err("the guest's game ran the stage itself".into());
    }
    for (who, app) in [("host", &mut rig.host), ("guest", &mut rig.guest)] {
        let stage = stage_of(app, QUEST);
        if stage != Some(20) {
            return Err(format!("the {who}'s stage is {stage:?}, want Some(20)"));
        }
        let hour = global_of(app, HOUR);
        if hour != Some(9.0) {
            return Err(format!("the {who}'s GameHour is {hour:?}, want Some(9.0)"));
        }
    }
    if !rig
        .host
        .machine
        .lines
        .iter()
        .any(|l| l == "set GameHour to 9")
    {
        return Err(format!(
            "the host's game did not run the guest's line: {:?}",
            rig.host.machine.lines
        ));
    }
    if !rig
        .guest
        .dump_text()
        .contains("\"intents\":{\"sent\":1,\"open\":0}")
    {
        return Err("the guest's dump does not count the intent".into());
    }
    Ok(())
}

/// The two dumps agree on the shared state. A change that a guest's game makes by itself
/// does not stay: the guest's next check against the host's state takes it away.
///
/// Twin: no frame runs after the guest's own change.
fn the_dumps_agree_and_a_guests_own_change_does_not_stay(mutate: bool) -> Result<(), String> {
    let mut rig = Rig::new(&["--stage", "VCG03", "10"], &[]);
    rig.join()?;
    let (host, guest) = (rig.host.dump_text(), rig.guest.dump_text());
    for (key, next) in [("quests", "player_quests"), ("globals", "positions")] {
        if section(&host, key, next) != section(&guest, key, next) {
            return Err(format!(
                "the dumps differ in {key} before the change: {} and {}",
                section(&host, key, next),
                section(&guest, key, next)
            ));
        }
    }
    if shared_hash(&host) != shared_hash(&guest) {
        return Err("the hashes differ before the change".into());
    }
    // The guest's own code moves a shared global.
    let hour = rig.guest.machine.call(addr::FORM_BY_ID, &[HOUR]).eax;
    rig.guest.machine.p.set_f32(hour + addr::GLOBAL_VALUE, 3.0);
    if shared_hash(&rig.guest.dump_text()) == shared_hash(&host) {
        return Err("the hash did not follow the guest's change".into());
    }
    if !mutate {
        rig.frames(SETTLE);
    }
    let (host, guest) = (rig.host.dump_text(), rig.guest.dump_text());
    if shared_hash(&host) != shared_hash(&guest) {
        return Err(format!(
            "the hashes differ after the guest's own change: the guest's GameHour is {:?}",
            global_of(&mut rig.guest, HOUR)
        ));
    }
    Ok(())
}

/// A guest's game loads its cell in one long frame: the guest's program does not run for
/// seconds while the host's program does. A change that the host makes right after the
/// load still reaches the guest.
///
/// Twin: the host makes no change after the load.
fn a_change_after_a_guests_long_load_reaches_the_guest(mutate: bool) -> Result<(), String> {
    for (silent_frames, after_frames) in [(280u32, 12u32), (280, 1), (120, 12), (600, 12)] {
        let mut rig = Rig::new(&[], &[]);
        // The guest joins. Its first frames run, and then its game loads.
        let mut joined = false;
        for _ in 0..1500 {
            rig.frame();
            if rig.guest.machine.first_play.is_some() {
                joined = true;
                break;
            }
        }
        if !joined {
            return Err("the guest did not reach play".into());
        }
        // Only the host's program runs: the guest is in one long frame.
        for _ in 0..silent_frames {
            rig.host.step(DT);
        }
        rig.frames(after_frames);
        if !mutate {
            let quest = rig.host.machine.call(addr::FORM_BY_ID, &[QUEST]).eax;
            rig.host.machine.call(addr::QUEST_SET_STAGE, &[quest, 10]);
        }
        rig.frames(SETTLE);
        let stage = stage_of(&mut rig.guest, QUEST);
        if stage != Some(10) {
            let dump = rig.guest.dump_text();
            return Err(format!(
                "after {silent_frames} silent frames and {after_frames} frames, the guest's stage is {stage:?}, want Some(10): {dump}"
            ));
        }
    }
    Ok(())
}

/// One request to a control port, and its answer. The program's frames run while the
/// request waits.
fn ask(rig: &mut Rig, port: &mut TcpStream, request: &str) -> Result<(String, Flow), String> {
    port.write_all(format!("{request}\n").as_bytes())
        .map_err(|e| format!("{e}"))?;
    let mut reader = BufReader::new(port.try_clone().map_err(|e| format!("{e}"))?);
    let mut flow = Flow::Go;
    for _ in 0..400 {
        let (host, _) = rig.frame();
        if host != Flow::Go {
            flow = host;
        }
        let mut line = String::new();
        match reader.read_line(&mut line) {
            Ok(n) if n > 0 => return Ok((line.trim().to_string(), flow)),
            _ => {}
        }
    }
    Err(format!("no answer to {request}"))
}

/// The control port answers the requests of the contract: the state with the dump, a
/// script line, a request that is not built, and the end of the program.
///
/// Twin: the command line asks for no control port.
fn the_control_port_answers_the_contracts_requests(mutate: bool) -> Result<(), String> {
    let more: &[&str] = if mutate {
        &[]
    } else {
        &["--coop-control", "0"]
    };
    let mut rig = Rig::new(more, &[]);
    rig.join()?;
    let at = rig
        .host
        .control_addr()
        .ok_or("the host has no control port")?;
    let mut port = TcpStream::connect(at).map_err(|e| format!("{e}"))?;
    port.set_read_timeout(Some(Duration::from_millis(5)))
        .map_err(|e| format!("{e}"))?;
    let (state, _) = ask(&mut rig, &mut port, "{\"cmd\":\"state\",\"id\":7}")?;
    for part in [
        "\"id\":7",
        "\"ok\":true",
        "\"role\":\"host\"",
        "\"ready\":true",
        "\"cell\":\"GSDocMitchellHouse\"",
        "\"state\":{\"schema\":1",
    ] {
        if !state.contains(part) {
            return Err(format!("{part} is not in the state answer: {state}"));
        }
    }
    let (ran, _) = ask(
        &mut rig,
        &mut port,
        "{\"cmd\":\"run\",\"line\":\"SetStage VCG03 20\"}",
    )?;
    if !ran.contains("\"how\":\"host\"") {
        return Err(format!("the run answer: {ran}"));
    }
    rig.frames(SETTLE);
    let stage = stage_of(&mut rig.guest, QUEST);
    if stage != Some(20) {
        return Err(format!("the guest's stage after the run is {stage:?}"));
    }
    let (keys, _) = ask(&mut rig, &mut port, "{\"cmd\":\"keys\",\"key\":\"w\"}")?;
    if !keys.contains("\"ok\":false") || !keys.contains("not built") {
        return Err(format!("the keys answer: {keys}"));
    }
    let (quit, flow) = ask(&mut rig, &mut port, "{\"cmd\":\"quit\"}")?;
    if !quit.contains("\"ok\":true") || flow != Flow::Quit("control") {
        return Err(format!("the quit answer: {quit}, the flow: {flow:?}"));
    }
    Ok(())
}

/// The program ends at the time that the command line gives, counted from the ready
/// session, and it leaves a state dump that is whole.
///
/// Twin: the command line gives no time.
fn the_program_ends_after_its_time_and_leaves_a_dump(mutate: bool) -> Result<(), String> {
    let dump = std::env::temp_dir().join(format!(
        "coop-app-dump-{}-{}.json",
        std::process::id(),
        u8::from(mutate)
    ));
    let _ = std::fs::remove_file(&dump);
    let path = dump.display().to_string();
    let mut more = vec!["--coop-state-dump", path.as_str()];
    if !mutate {
        more.extend(["--coop-quit-after", "1.5"]);
    }
    let mut rig = Rig::new(&more, &[]);
    let mut ended = None;
    for frame in 0..3000u32 {
        let (host, _) = rig.frame();
        if host != Flow::Go {
            ended = Some((frame, host));
            break;
        }
    }
    let text = std::fs::read_to_string(&dump).unwrap_or_default();
    let _ = std::fs::remove_file(&dump);
    let Some((_, flow)) = ended else {
        return Err("the program did not end in 3000 frames".into());
    };
    if flow != Flow::Quit("quit_after") {
        return Err(format!("the program ended with {flow:?}"));
    }
    if !rig.host.ready() {
        return Err("the program ended before the session was ready".into());
    }
    if !text.starts_with("{\"schema\":1")
        || !text.ends_with("]}")
        || !text.contains("\"ready\":true")
    {
        return Err(format!("the dump file is not whole: {text}"));
    }
    Ok(())
}

#[test]
fn a_guest_gets_its_gate_before_play() {
    check(
        a_guest_takes_its_gate_at_the_menu_and_plays_in_the_hosts_cell,
        "the guest did not reach play with a gate",
    );
}

#[test]
fn the_hosts_story_is_plain_state_for_the_guest() {
    check(
        the_hosts_story_reaches_the_guest_as_plain_state,
        "the guest's stage of VCG03 is None",
    );
}

#[test]
fn a_guests_intent_runs_on_the_host() {
    check(
        a_guests_intent_and_line_run_on_the_hosts_game,
        "the host's game did not get the stage 20",
    );
}

#[test]
fn a_change_after_a_long_load_reaches_the_guest() {
    check(
        a_change_after_a_guests_long_load_reaches_the_guest,
        "the guest's stage is None",
    );
}

#[test]
fn the_dumps_agree() {
    check(
        the_dumps_agree_and_a_guests_own_change_does_not_stay,
        "the hashes differ after the guest's own change",
    );
}

#[test]
fn the_control_port_answers() {
    check(
        the_control_port_answers_the_contracts_requests,
        "the host has no control port",
    );
}

#[test]
fn the_program_ends_and_leaves_a_dump() {
    check(
        the_program_ends_after_its_time_and_leaves_a_dump,
        "the program did not end",
    );
}

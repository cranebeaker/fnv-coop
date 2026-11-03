//! Checks over the harness itself and the save layer: verdicts, the simulator's determinism, the
//! live control port, the engine backend stub, the text save format and Continue.
//!
//! Every check takes `mutate`: with it set the check sabotages the thing under test and must
//! then fail (the mutation twin of the row).

use std::collections::HashMap;
use std::io::{BufRead, BufReader, Write};
use std::net::TcpStream;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::{Duration, SystemTime};

use coop_session::control::{self, Cmd};
use esm::FormId;
use nv_launcher::party::{self as lparty, Role as LRole};
use world::scripting::GameState;

use super::builtin::{Check, Env};
use crate::backend::{self, Verdict as BVerdict};
use crate::data::Data;
use crate::json::J;
use crate::scenario::Scenario;
use crate::sim::{self, RunOpts};

/// The checks of this module by name.
pub fn registry() -> Vec<(&'static str, Check)> {
    vec![
        ("save_text_roundtrip", save_text_roundtrip),
        ("save_continue_newest", save_continue_newest),
        ("harness_verdicts", harness_verdicts),
        ("harness_sim_deterministic", harness_sim_deterministic),
        ("harness_live_control", harness_live_control),
        ("backend_engine_stub", backend_engine_stub),
        ("couch_split_screen", couch_split_screen),
    ]
}

/// A small world state with one fact of each kind the save writes.
fn sample_state(tag: u32) -> GameState {
    let mut gs = GameState {
        player_level: 7 + tag as u16,
        ..Default::default()
    };
    gs.stages.insert(FormId(0x0010_4C1C), 10 + tag as u16);
    gs.globals.insert(FormId(0x0000_0039), 2.5 + tag as f32);
    gs.items
        .insert((FormId(0x14), FormId(0x0000_000F)), 120 + tag as i32);
    gs.disabled.insert(FormId(0x0010_4E00), true);
    gs
}

/// The facts of `sample_state` that must come back from a load.
fn same_facts(a: &GameState, b: &GameState) -> Result<(), String> {
    if a.player_level != b.player_level {
        return Err(format!(
            "level {} came back as {}",
            a.player_level, b.player_level
        ));
    }
    if a.stages != b.stages {
        return Err(format!("stages {:?} came back as {:?}", a.stages, b.stages));
    }
    if a.globals != b.globals {
        return Err(format!(
            "globals {:?} came back as {:?}",
            a.globals, b.globals
        ));
    }
    if a.items != b.items {
        return Err(format!("items {:?} came back as {:?}", a.items, b.items));
    }
    if a.disabled != b.disabled {
        return Err(format!(
            "disabled {:?} came back as {:?}",
            a.disabled, b.disabled
        ));
    }
    Ok(())
}

/// `save.format`: the save is plain text, it loads back to the same state, and a second save of
/// the loaded state is byte-identical. The twin drops the item line before the load.
fn save_text_roundtrip(_env: &Env, mutate: bool) -> Result<String, String> {
    let gs = sample_state(0);
    let text = world::save::save(&gs, None);
    if text.lines().count() < 5 || text.contains('\0') {
        return Err("the save is not line-based text".into());
    }
    let fed = if mutate {
        text.lines()
            .filter(|l| !l.starts_with("item "))
            .map(|l| format!("{l}\n"))
            .collect()
    } else {
        text.clone()
    };
    let (back, _) = world::save::load(&fed)?;
    same_facts(&gs, &back)?;
    let again = world::save::save(&back, None);
    if again != text {
        return Err("a second save of the loaded state differs from the first".into());
    }
    Ok(format!(
        "text save of {} lines loads back to the same state and saves byte-identical",
        text.lines().count()
    ))
}

/// A fresh folder for one check run under the temp folder (`TEMP` is on D: in this project).
fn scratch(name: &str, mutate: bool) -> Result<PathBuf, String> {
    let d = std::env::temp_dir().join(format!(
        "coop-check-{name}-{}-{}",
        std::process::id(),
        u8::from(mutate)
    ));
    if d.exists() {
        std::fs::remove_dir_all(&d).map_err(|e| format!("{}: {e}", d.display()))?;
    }
    std::fs::create_dir_all(&d).map_err(|e| format!("{}: {e}", d.display()))?;
    Ok(d)
}

fn write_save(path: &Path, gs: &GameState, age_s: u64) -> Result<(), String> {
    std::fs::write(path, world::save::save(gs, None)).map_err(|e| e.to_string())?;
    let t = SystemTime::now() - Duration::from_secs(age_s);
    std::fs::File::options()
        .write(true)
        .open(path)
        .and_then(|f| f.set_modified(t))
        .map_err(|e| e.to_string())
}

/// `save.continue`: the launcher's Continue hands the viewer the newest host save of the
/// campaign, and that save loads to the last state. The twin never writes the newer save.
fn save_continue_newest(_env: &Env, mutate: bool) -> Result<String, String> {
    let root = scratch("continue", mutate)?.join("saves");
    let id = "00c0ffee12345678";
    let host = lparty::ensure_player_dir(&root, id, LRole::Host, "Host", 1_700_000_000)
        .map_err(|e| e.to_string())?;
    let guest = lparty::ensure_player_dir(&root, id, LRole::Guest, "Ann", 1_700_000_000)
        .map_err(|e| e.to_string())?;
    let (old, new) = (sample_state(0), sample_state(1));
    write_save(&host.join("nv-rs-autosave.txt"), &old, 600)?;
    if !mutate {
        write_save(&host.join("nv-rs-quicksave.txt"), &new, 60)?;
    }
    // a guest's save is newer still: Continue for the host must not take it
    write_save(&guest.join("nv-rs-quicksave.txt"), &sample_state(2), 1)?;
    let other = lparty::ensure_player_dir(&root, "00c0ffee87654321", LRole::Host, "Host", 1)
        .map_err(|e| e.to_string())?;
    write_save(&other.join("nv-rs-quicksave.txt"), &sample_state(3), 0)?;
    let picked = lparty::find_save(&root, id, LRole::Host, "Host")
        .ok_or("Continue found no host save of the campaign")?;
    let text = std::fs::read_to_string(&picked).map_err(|e| e.to_string())?;
    let (back, _) = world::save::load(&text)?;
    let res = same_facts(&new, &back).map_err(|e| {
        format!(
            "Continue picked {} and it is not the last save: {e}",
            picked.display()
        )
    });
    let _ = std::fs::remove_dir_all(root.parent().unwrap_or(&root));
    res?;
    Ok("Continue picks the newest host save of the campaign (not a guest's, not another campaign's) and it loads the last state".into())
}

/// A small scenario in Doc Mitchell's house: the host gives itself 3 stimpaks.
fn tiny_scenario(name: &str, want: u32) -> Result<Scenario, String> {
    let text = format!(
        r#"{{"schema":1,"cell":"GSDocMitchellHouse","network":"perfect","seed":5,"duration_s":6,"settle_s":2,"timeout_s":60,
"description":"harness check","expect":[{{"id":"stims","path":"items.host.Stimpak","op":"eq","value":{want},"on":"both"}}],
"host":{{"script":[{{"at":1,"run":"player.AddItem Stimpak 3"}}]}},"guest":{{"script":[]}}}}"#
    );
    Scenario::parse(&text, name)
}

fn open_data(env: &Env) -> Result<Arc<Data>, String> {
    let d = env.data.ok_or("needs a data folder")?;
    Data::open(&d.dir).map(Arc::new)
}

/// `harness.verdicts`: a passing run and a failing run each end in one verdict file that parses
/// back with its result, class and checks. The twin truncates the file before the parse.
fn harness_verdicts(env: &Env, mutate: bool) -> Result<String, String> {
    let d = open_data(env)?;
    let out = scratch("verdicts", mutate)?;
    let mut seen = Vec::new();
    for (name, want, result) in [("chk_pass", 3, "pass"), ("chk_fail", 4, "fail")] {
        let scn = tiny_scenario(name, want)?;
        let o = RunOpts {
            out: Some(out.clone()),
            ..Default::default()
        };
        let r = sim::run(&scn, &d, &o);
        let dir = out.join(name);
        let files: Vec<PathBuf> = walk(&dir)
            .into_iter()
            .filter(|p| p.file_name().is_some_and(|n| n == "verdict.json"))
            .collect();
        if files.len() != 1 {
            return Err(format!(
                "{name}: {} verdict files, want exactly one",
                files.len()
            ));
        }
        let mut text = std::fs::read_to_string(&files[0]).map_err(|e| e.to_string())?;
        if mutate {
            text.truncate(text.len() / 2);
        }
        let v = J::parse(&text).map_err(|e| format!("{name}: verdict does not parse: {e}"))?;
        let got = v.get("result").and_then(J::as_str).unwrap_or_default();
        if got != result || r.verdict.result != result {
            return Err(format!(
                "{name}: verdict result {got:?}, run said {:?}, want {result}",
                r.verdict.result
            ));
        }
        for k in ["scenario", "class", "seed", "rerun"] {
            if v.get(k).is_none() {
                return Err(format!("{name}: verdict has no {k}"));
            }
        }
        seen.push(format!("{name}={got}"));
    }
    let _ = std::fs::remove_dir_all(&out);
    Ok(format!(
        "one parseable verdict.json per run ({})",
        seen.join(", ")
    ))
}

fn walk(dir: &Path) -> Vec<PathBuf> {
    let mut out = Vec::new();
    let Ok(rd) = std::fs::read_dir(dir) else {
        return out;
    };
    for e in rd.flatten() {
        let p = e.path();
        if p.is_dir() {
            out.extend(walk(&p));
        } else {
            out.push(p);
        }
    }
    out
}

/// `harness.fake_engine`: the simulator gives the same final dumps for the same scenario, seed
/// and network profile, over a small soak matrix. The twin changes the seed of the second run.
fn harness_sim_deterministic(env: &Env, mutate: bool) -> Result<String, String> {
    let d = open_data(env)?;
    // a lossy profile, so the seed drives the packet drops and the timing
    let scn = tiny_scenario("chk_det", 3)?;
    let mut cells = 0;
    for net in ["perfect", "bad_wifi"] {
        for seed in [1u64, 2] {
            let run = |s: u64| {
                sim::run(
                    &scn,
                    &d,
                    &RunOpts {
                        seed: Some(s),
                        net: Some(net.into()),
                        ..Default::default()
                    },
                )
            };
            let a = run(seed);
            let b = run(if mutate { seed + 100 } else { seed });
            if a.hash != b.hash || a.dumps != b.dumps {
                return Err(format!("{net} seed {seed}: two runs gave different dumps"));
            }
            if a.verdict.result != "pass" {
                return Err(format!(
                    "{net} seed {seed}: the run itself failed: {}",
                    a.verdict.result
                ));
            }
            cells += 1;
        }
    }
    Ok(format!(
        "{cells} soak cells (2 profiles x 2 seeds) gave identical dumps twice"
    ))
}

/// Answers one request the way `viewer/src/coop_control.rs` does on its frame.
fn answer(cmd: &Cmd, world: &mut HashMap<String, String>, mutate: bool) -> String {
    let name = if mutate { "state" } else { cmd.name() };
    match cmd {
        Cmd::State => control::ok(name, "\"cell\":\"GSDocMitchellHouse\""),
        Cmd::Run { line, local } => {
            world.insert(line.clone(), local.to_string());
            control::ok(
                name,
                if *local {
                    "\"how\":\"local\""
                } else {
                    "\"how\":\"host\""
                },
            )
        }
        Cmd::Bot { .. } => control::ok(name, "\"how\":\"bot\""),
        Cmd::Keys { .. } | Cmd::Entities | Cmd::Screenshot { .. } => control::ok(name, ""),
        Cmd::Quit => control::ok(name, ""),
    }
}

/// `harness.live_control`: the control port accepts JSON lines on 127.0.0.1, hands each request
/// to the viewer's frame in order, and returns one reply per line with the id echoed; a bad line
/// gets `ok:false`. The viewer's frame is modelled here. The twin answers every request as
/// `state`.
fn harness_live_control(_env: &Env, mutate: bool) -> Result<String, String> {
    let (rx, addr) = control::serve(0)?;
    let frame = std::thread::spawn(move || {
        let mut world = HashMap::new();
        while let Ok(req) = rx.recv_timeout(Duration::from_secs(5)) {
            let quit = matches!(req.cmd, Cmd::Quit);
            let _ = req.reply.send(answer(&req.cmd, &mut world, mutate));
            if quit {
                break;
            }
        }
        world
    });
    let conn = TcpStream::connect(addr).map_err(|e| e.to_string())?;
    conn.set_read_timeout(Some(Duration::from_secs(10)))
        .map_err(|e| e.to_string())?;
    let mut w = conn.try_clone().map_err(|e| e.to_string())?;
    let mut r = BufReader::new(conn);
    let reqs: [(&str, &str, bool); 6] = [
        (r#"{"cmd":"state","id":1}"#, "state", true),
        (
            r#"{"cmd":"run","line":"SetStage VMS01 10","id":2}"#,
            "run",
            true,
        ),
        (
            r#"{"cmd":"run","line":"player.AddItem Caps001 5","local":true,"id":"x3"}"#,
            "run",
            true,
        ),
        (
            r#"{"cmd":"bot","line":"coop teleport 1 2 3","id":4}"#,
            "bot",
            true,
        ),
        (r#"{"cmd":"nonsense","id":5}"#, "", false),
        (r#"{"cmd":"quit","id":6}"#, "quit", true),
    ];
    for (i, (line, cmd, good)) in reqs.iter().enumerate() {
        writeln!(w, "{line}").map_err(|e| e.to_string())?;
        let mut reply = String::new();
        r.read_line(&mut reply)
            .map_err(|e| format!("no reply to {line}: {e}"))?;
        let v = J::parse(reply.trim()).map_err(|e| format!("reply {reply:?}: {e}"))?;
        let ok = v.get("ok").and_then(J::as_bool);
        if ok != Some(*good) {
            return Err(format!("{line}: reply {reply:?}, want ok={good}"));
        }
        if *good && v.get("cmd").and_then(J::as_str) != Some(*cmd) {
            return Err(format!("{line}: reply {reply:?} is for another command"));
        }
        let want_id = control::id_of(line).unwrap_or_default();
        if !reply.contains(&format!("\"id\":{want_id}")) {
            return Err(format!(
                "request {} lost its id {want_id}: {reply:?}",
                i + 1
            ));
        }
    }
    let world = frame.join().map_err(|_| "the frame thread panicked")?;
    if world.get("SetStage VMS01 10").map(String::as_str) != Some("false")
        || world.get("player.AddItem Caps001 5").map(String::as_str) != Some("true")
    {
        return Err(format!("the frame got the wrong run requests: {world:?}"));
    }
    Ok("control port on 127.0.0.1: 6 JSON lines, one reply each in order with ids, a bad line refused, run and local run reach the frame (the viewer's frame is modelled)".into())
}

/// `backend.engine_stub`: on the engine backend every conformance test is PENDING (nothing of
/// the translated engine runs yet), and the checklist turns every world-bound row PENDING with
/// `--backend engine`. The twin runs the tests on the world backend instead.
fn backend_engine_stub(env: &Env, mutate: bool) -> Result<String, String> {
    let d = env.data.ok_or("needs a data folder")?;
    let b = backend::by_name(if mutate { "world" } else { "engine" })?;
    for (id, _) in backend::TESTS {
        match backend::run_test(b.as_ref(), d, id, false) {
            BVerdict::Pending(_) => {}
            other => {
                return Err(format!(
                    "conformance {id} on the {} backend is {other:?}, want PENDING",
                    b.name()
                ))
            }
        }
    }
    let gate = super::run::engine_gate;
    if gate(true, "world").is_none()
        || gate(true, "agnostic").is_some()
        || gate(false, "world").is_some()
    {
        return Err("the checklist's engine gate does not hold world rows only".into());
    }
    Ok(format!(
        "all {} conformance tests PENDING on the engine backend; --backend engine holds world rows only",
        backend::TESTS.len()
    ))
}

/// `save.split_screen`: couch mode on one PC (docs/COOP-COUCH.md). For 2 to 4 local players the
/// launcher tiles the screen into one window per seat (inside the screen, no overlap, each at
/// least the viewer's smallest window), every seat gets its couch seat, window and controller on
/// its viewer command line, and only seat 0 opens a microphone. The twin gives every seat the
/// whole screen.
fn couch_split_screen(_env: &Env, mutate: bool) -> Result<String, String> {
    use nv_launcher::extras::{layout, Couch, Extras, Rect};
    let (sw, sh) = (1920u32, 1080u32);
    for players in 2u8..=4 {
        let rects = layout(if mutate { 1 } else { players }, sw, sh);
        let rects: Vec<Rect> = (0..players as usize)
            .map(|i| rects.get(i).or(rects.first()).copied().ok_or("no layout"))
            .collect::<Result<_, _>>()?;
        let mut area = 0u64;
        for (i, r) in rects.iter().enumerate() {
            if r.x < 0 || r.y < 0 || r.x as u32 + r.w > sw || r.y as u32 + r.h > sh {
                return Err(format!(
                    "{players} players: seat {i} window {r:?} leaves the screen"
                ));
            }
            Rect::parse(&r.arg()).map_err(|e| format!("{players} players: seat {i}: {e}"))?;
            area += r.w as u64 * r.h as u64;
            for (j, o) in rects.iter().enumerate().skip(i + 1) {
                let apart = r.x + r.w as i32 <= o.x
                    || o.x + o.w as i32 <= r.x
                    || r.y + r.h as i32 <= o.y
                    || o.y + o.h as i32 <= r.y;
                if !apart {
                    return Err(format!("{players} players: seats {i} and {j} overlap"));
                }
            }
            let couch = Couch::parse(&format!("{i}/{players}"))?;
            let a = Extras {
                allow_mismatch: false,
                couch: Some(couch),
                window: Some(*r),
                gamepad: Some(i as u8),
                voice: None,
                ptt_key: None,
                ping_key: None,
                difficulty: None,
                friendly_fire: None,
            }
            .args();
            let has = |k: &str, v: &str| a.windows(2).any(|w| w[0] == k && w[1] == v);
            if !has("--coop-couch", &format!("{i}/{players}"))
                || !has("--coop-window", &r.arg())
                || !has("--coop-gamepad", &i.to_string())
            {
                return Err(format!("{players} players: seat {i} command line {a:?}"));
            }
            if has("--coop-voice", "off") == (i == 0) {
                return Err(format!(
                    "{players} players: seat {i} voice {a:?} (only seat 0 opens a microphone)"
                ));
            }
        }
        if players != 3 && area != sw as u64 * sh as u64 {
            return Err(format!(
                "{players} players: the windows do not fill the screen"
            ));
        }
    }
    Ok("couch on one PC: 2 to 4 seats tiled into separate windows without overlap, each with its seat, window and controller; only seat 0 opens a microphone".into())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn env() -> Env<'static> {
        Env {
            data: None,
            peers: 2,
            net: "perfect",
            seed: 1,
        }
    }

    #[test]
    fn checks_without_data_pass_and_their_twins_fail() {
        for (name, f) in [
            ("save_text_roundtrip", save_text_roundtrip as Check),
            ("save_continue_newest", save_continue_newest),
            ("harness_live_control", harness_live_control),
            ("couch_split_screen", couch_split_screen),
        ] {
            assert!(f(&env(), false).is_ok(), "{name}: {:?}", f(&env(), false));
            assert!(f(&env(), true).is_err(), "{name} twin passed");
        }
    }
}

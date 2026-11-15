//! The backend conformance tests on the real game program: the upstream engine, one child
//! process for each peer (`coop_headless::engine_peer`).
//!
//! The tests run only when `COOP_ENGINE_PEER` names the peer server's program (built from
//! `engine-coop/`) and the game's data is found. Set the variable for this test binary only:
//! the other tests of this crate check that the engine backend is PENDING without it.
//! `NV_D3D9_ADAPTER` must name a scratch instance's machine table (`engine-coop/README.md`).
//!
//! Each test starts real games. Run this binary with `--test-threads=1` on a machine that
//! cannot run four games at one time.

use coop_headless::backend::{self, Verdict};
use coop_headless::data::Data;
use coop_headless::engine_peer;

/// The tests that need per-player rows, which the engine backend does not keep yet.
const PLAYER_ROWS: [&str; 2] = ["per_player_stays_private", "shadow_rows_reach_guest"];

fn data() -> Option<Data> {
    let d = Data::locate(None).ok().and_then(|p| Data::open(&p).ok());
    if d.is_none() {
        eprintln!("skipped: the game's data was not found");
    }
    d
}

/// The data, when a peer server is named too.
fn engine_data() -> Option<Data> {
    if engine_peer::program().is_none() {
        eprintln!(
            "skipped: {} does not name a peer server",
            engine_peer::PEER_PROGRAM
        );
        return None;
    }
    data()
}

/// Each conformance test passes on the engine, and its mutation twin fails. The tests of
/// per-player rows are PENDING, with the reason.
#[test]
fn the_conformance_tests_pass_on_the_engine_and_their_twins_fail() {
    let Some(d) = engine_data() else { return };
    let engine = backend::by_name("engine").unwrap();
    let mut passed = 0;
    for (id, _) in backend::TESTS {
        let verdict = backend::run_test(&*engine, &d, id, false);
        if PLAYER_ROWS.contains(id) {
            assert!(
                matches!(&verdict, Verdict::Pending(m) if m.contains("per-player rows")),
                "{id}: {verdict:?}"
            );
            continue;
        }
        assert_eq!(verdict, Verdict::Pass, "{id}");
        let twin = backend::run_test(&*engine, &d, id, true);
        assert!(matches!(twin, Verdict::Fail(_)), "{id} twin: {twin:?}");
        passed += 1;
    }
    assert_eq!(passed, backend::TESTS.len() - PLAYER_ROWS.len());
}

/// Each test of the guest's script gate passes on the engine, and its mutation twin fails
/// for its own reason (not because a game did not start).
#[test]
fn the_gate_tests_pass_on_the_engine_and_their_twins_fail() {
    let Some(d) = engine_data() else { return };
    let engine = backend::by_name("engine").unwrap();
    for (id, _) in backend::GATE_TESTS {
        let verdict = backend::run_test(&*engine, &d, id, false);
        assert_eq!(verdict, Verdict::Pass, "{id}");
        let twin = backend::run_test(&*engine, &d, id, true);
        assert!(
            matches!(&twin, Verdict::Fail(why) if !why.contains("peer server")
                && !why.contains("did not do it")
                && !why.contains("proves nothing")),
            "{id} twin: {twin:?}"
        );
        eprintln!("{id}: pass, twin: {twin:?}");
    }
}

/// Each test of a remote player's avatar passes on the engine, and its mutation twin fails
/// for its own reason.
#[test]
fn the_avatar_tests_pass_on_the_engine_and_their_twins_fail() {
    let Some(d) = engine_data() else { return };
    let engine = backend::by_name("engine").unwrap();
    for (id, _) in backend::AVATAR_TESTS {
        let verdict = backend::run_test(&*engine, &d, id, false);
        assert_eq!(verdict, Verdict::Pass, "{id}");
        let twin = backend::run_test(&*engine, &d, id, true);
        assert!(
            matches!(&twin, Verdict::Fail(why) if !why.contains("peer server")
                && !why.contains("ran no frame")
                && !why.contains("twin's line")),
            "{id} twin: {twin:?}"
        );
        eprintln!("{id}: pass, twin: {twin:?}");
    }
}

/// Each test of the world's state passes on the engine, and its mutation twin fails for its
/// own reason.
#[test]
fn the_world_tests_pass_on_the_engine_and_their_twins_fail() {
    let Some(d) = engine_data() else { return };
    let engine = backend::by_name("engine").unwrap();
    for (id, _) in backend::WORLD_TESTS {
        let verdict = backend::run_test(&*engine, &d, id, false);
        assert_eq!(verdict, Verdict::Pass, "{id}");
        let twin = backend::run_test(&*engine, &d, id, true);
        assert!(
            matches!(&twin, Verdict::Fail(why) if !why.contains("peer server")
                && !why.contains("did not run")
                && !why.contains("did not get")
                && !why.contains("proves nothing")),
            "{id} twin: {twin:?}"
        );
        eprintln!("{id}: pass, twin: {twin:?}");
    }
}

/// The world layer has no script gate of this kind: each gate test is PENDING there, with
/// the reason. A row that only one backend runs is reported as open, not as passed.
#[test]
fn the_gate_tests_are_pending_on_the_world_layer() {
    let Some(d) = data() else { return };
    let world = backend::by_name("world").unwrap();
    for (id, _) in backend::GATE_TESTS.iter().chain(backend::AVATAR_TESTS) {
        let verdict = backend::run_test(&*world, &d, id, false);
        assert!(
            matches!(&verdict, Verdict::Pending(m) if m.contains("no script gate")),
            "{id}: {verdict:?}"
        );
    }
}

/// What is open for a row of the session's interface on the engine adapter.
enum Open {
    /// The row does not pass yet. Its answer must name this cause.
    Row(&'static str),
    /// The row passes. Its twin cannot fail for the row's own cause yet: it must name this
    /// cause.
    Twin(&'static str),
}

/// The rows of the session's interface that are open on the engine adapter. A row leaves this
/// list when its slice is built.
///
/// The rows of each player are not on the engine yet: the game keeps one player, and it
/// answers `Ok(false)` to a character. So the row that gives the local player a character
/// does not pass. The row "a character stays with its player" passes, because the game says
/// that it keeps one player. Its twin gives the character to the game's own player, and
/// stops at the same answer.
const OPEN_INTERFACE_ROWS: &[(&str, Open)] = &[
    (
        "character_reaches_the_local_player",
        Open::Row("Ok(false) to a character"),
    ),
    (
        "character_stays_with_its_player",
        Open::Twin("Ok(false) to the character of player"),
    ),
];

/// The names of the game's data for a peer: each plugin with its load index, and each quest,
/// global and cell with its editor id.
fn names_file(d: &Data) -> std::path::PathBuf {
    let mut text = String::new();
    for plugin in d.order.plugins() {
        text.push_str(&format!("P {:02X} {}\n", plugin.load_index, plugin.name));
    }
    for kind in [b"QUST", b"GLOB", b"CELL"] {
        for record in d.order.records_of_type(esm::FourCC::new(kind)) {
            let name = record.editor_id().ok().flatten().unwrap_or_default();
            text.push_str(&format!(
                "F {} {:08X} {}\n",
                std::str::from_utf8(kind).unwrap_or("????"),
                record.form_id.0,
                name
            ));
        }
    }
    let path = std::env::temp_dir().join(format!("coop-names-{}.tsv", std::process::id()));
    std::fs::write(&path, text).expect("the names file");
    path
}

/// One conformance row of `coop-game` on a new game process: `Ok` or the row's reason.
fn interface_row(d: &Data, names: &std::path::Path, row: &str, twin: bool) -> Result<(), String> {
    let program = engine_peer::program().ok_or("no peer server")?;
    let mut last = String::new();
    // The engine's stepped threads can stop a run in its first frames: a new start works.
    for _ in 0..4 {
        let mut command = std::process::Command::new(&program);
        command
            .arg(&d.dir)
            .args(["--conformance", row, "--names"])
            .arg(names)
            .env("NV_REPEATABLE", "1")
            .stderr(std::process::Stdio::null());
        if twin {
            command.arg("--twin");
        }
        let output = command.output().map_err(|e| format!("{e}"))?;
        let text = String::from_utf8_lossy(&output.stdout);
        let answer = text
            .lines()
            .filter_map(|l| l.strip_prefix("@ "))
            .next_back()
            .unwrap_or("")
            .to_string();
        if answer == "ok" {
            return Ok(());
        }
        if let Some(why) = answer.strip_prefix("fail ") {
            return Err(why.to_string());
        }
        last = answer;
    }
    Err(format!("the peer server gave no verdict: {last}"))
}

/// Each conformance row of the session's interface passes on the engine adapter, on a real
/// game, and its mutation twin fails for the row's own cause. A row that the adapter does not
/// pass yet is in the open list, and it must fail for its named cause.
#[test]
fn the_interface_rows_pass_on_the_engine_and_their_twins_fail() {
    use coop_game::conformance::ROWS;
    let Some(d) = engine_data() else { return };
    let names = names_file(&d);
    for row in ROWS {
        let verdict = interface_row(&d, &names, row.name, false);
        let open = OPEN_INTERFACE_ROWS
            .iter()
            .find(|(n, _)| *n == row.name)
            .map(|(_, open)| open);
        if let Some(Open::Row(cause)) = open {
            assert!(
                matches!(&verdict, Err(why) if why.contains(cause)),
                "{} is an open row: {verdict:?}",
                row.name
            );
            eprintln!("{}: OPEN ({verdict:?})", row.name);
            continue;
        }
        assert_eq!(verdict, Ok(()), "{}", row.name);
        let twin = interface_row(&d, &names, row.name, true);
        let (cause, note) = match open {
            Some(Open::Twin(cause)) => (*cause, "pass, OPEN twin"),
            _ => (row.twin_cause, "pass, twin"),
        };
        assert!(
            matches!(&twin, Err(why) if why.contains(cause)),
            "{} twin: {twin:?}",
            row.name
        );
        eprintln!("{}: {note}: {twin:?}", row.name);
    }
    let _ = std::fs::remove_file(names);
}

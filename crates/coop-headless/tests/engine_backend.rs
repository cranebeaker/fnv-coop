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

/// The world layer has no script gate of this kind: each gate test is PENDING there, with
/// the reason. A row that only one backend runs is reported as open, not as passed.
#[test]
fn the_gate_tests_are_pending_on_the_world_layer() {
    let Some(d) = data() else { return };
    let world = backend::by_name("world").unwrap();
    for (id, _) in backend::GATE_TESTS {
        let verdict = backend::run_test(&*world, &d, id, false);
        assert!(
            matches!(&verdict, Verdict::Pending(m) if m.contains("no script gate")),
            "{id}: {verdict:?}"
        );
    }
}

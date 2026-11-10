//! The backend conformance tests on the real game program: the upstream engine, one child
//! process for each peer (`coop_headless::engine_peer`).
//!
//! The test runs only when `COOP_ENGINE_PEER` names the peer server's program (built from
//! `engine-coop/`) and the game's data is found. Set the variable for this test binary only:
//! the other tests of this crate check that the engine backend is PENDING without it.
//! `NV_D3D9_ADAPTER` must name a scratch instance's machine table (`engine-coop/README.md`).

use coop_headless::backend::{self, Verdict};
use coop_headless::data::Data;
use coop_headless::engine_peer;

/// The tests that need per-player rows, which the engine backend does not keep yet.
const PLAYER_ROWS: [&str; 2] = ["per_player_stays_private", "shadow_rows_reach_guest"];

/// Each conformance test passes on the engine, and its mutation twin fails. The tests of
/// per-player rows are PENDING, with the reason.
#[test]
fn the_conformance_tests_pass_on_the_engine_and_their_twins_fail() {
    if engine_peer::program().is_none() {
        eprintln!(
            "skipped: {} does not name a peer server",
            engine_peer::PEER_PROGRAM
        );
        return;
    }
    let Some(d) = Data::locate(None).ok().and_then(|p| Data::open(&p).ok()) else {
        eprintln!("skipped: the game's data was not found");
        return;
    };
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

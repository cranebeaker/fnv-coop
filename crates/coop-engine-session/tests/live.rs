//! The live ladder on two games in one process (docs/coop/DECISIONS.md D3): a guest is moved to the
//! host's cell, each sees the other as an avatar that follows, and the story channel is untouched.
//! Each rung's failure is shown to be reported as that rung's FAIL and nothing worse.

use coop_core::ids::PlayerId;
use coop_core::proto::{Hello, PROTOCOL_VERSION};
use coop_core::sync::HostConfig;
use coop_core::transport::{LoopbackEndpoint, LoopbackNet};
use coop_engine::live;
use coop_engine::testkit::{Story, StorySpec};
use coop_engine_session::ladder::{Cap, Ladder, DEFAULT_AVATAR_BASE};
use coop_engine_session::{echo, gate_guest, GuestDriver, HostDriver};

engine_core::assemble!();

const MQ: u32 = 0x0010_4c0c;
const HOUSE: u32 = 0x0010_5a00;
const TOWN: u32 = 0x0010_5a01;

fn spec() -> StorySpec {
    StorySpec {
        quests: vec![(MQ, vec![10, 20])],
        globals: vec![(0x39, 0.0)],
        cells: vec![HOUSE, TOWN],
        ..StorySpec::default()
    }
}

fn hello() -> Hello {
    let plugins = vec!["FalloutNV.esm".to_string()];
    Hello {
        proto: PROTOCOL_VERSION,
        build_hash: 1,
        rules_hash: 2,
        mods_hash: coop_core::compat::plugin_list_hash(&plugins),
        exe: "1.4.0.525".into(),
        volatile_hash: 3,
        plugins,
    }
}

struct Pair {
    host: Story,
    hd: HostDriver,
    hnet: LoopbackEndpoint,
    guest: Story,
    gd: GuestDriver,
    gnet: LoopbackEndpoint,
    t: u64,
}

impl Pair {
    fn new(cap: Cap, guest_spec: &StorySpec) -> Pair {
        let net = LoopbackNet::new();
        let mut host = Story::new(&spec());
        let mut hd = HostDriver::new(HostConfig::new(hello(), 1), [MQ], [0x39]);
        hd.ladder = Some(Ladder::new(false, cap, DEFAULT_AVATAR_BASE).quiet());
        hd.start(&mut host.e);
        let mut guest = Story::new(guest_spec);
        gate_guest(&mut guest.e);
        let mut gd = GuestDriver::new(PlayerId(1), hello());
        gd.ladder = Some(Ladder::new(true, cap, DEFAULT_AVATAR_BASE).quiet());
        Pair {
            hnet: net.endpoint(PlayerId::HOST),
            gnet: net.endpoint(PlayerId(1)),
            host,
            hd,
            guest,
            gd,
            t: 0,
        }
    }

    fn frames(&mut self, n: u32) {
        for _ in 0..n {
            self.t += 50;
            self.hd.step(&mut self.host.e, &mut self.hnet, self.t);
            self.guest.run_quest_script();
            self.gd.step(&mut self.guest.e, &mut self.gnet, self.t);
        }
    }

    fn host_lines(&self) -> &[String] {
        &self.hd.ladder.as_ref().unwrap().lines
    }

    fn guest_lines(&self) -> &[String] {
        &self.gd.ladder.as_ref().unwrap().lines
    }
}

/// The rung results of `lines`, as `(rung, passed)`.
fn rungs(lines: &[String]) -> Vec<(u32, bool)> {
    lines
        .iter()
        .filter_map(|l| {
            let rest = l.strip_prefix("COOP-RUNG ")?;
            let (n, rest) = rest.split_once(' ')?;
            match rest.split(' ').next()? {
                "PASS" => Some((n.parse().ok()?, true)),
                "FAIL" => Some((n.parse().ok()?, false)),
                _ => None,
            }
        })
        .collect()
}

fn close(a: [f32; 3], b: [f32; 3]) -> bool {
    (0..3).all(|i| (a[i] - b[i]).abs() <= 0.1)
}

#[test]
fn a_guest_comes_to_the_host_and_each_sees_the_other_follow() {
    let mut p = Pair::new(Cap::Avatar, &spec());
    p.host.move_player(HOUSE, [100.0, 200.0, 0.0], 1.0);
    // The guest pressed New Game: a player somewhere else.
    p.guest.move_player(TOWN, [5.0, 5.0, 5.0], 0.0);
    assert!(p.host.set_stage(MQ, 10));
    let story_rev = |p: &Pair| p.hd.sync.store.rev();
    p.frames(40);
    let rev = story_rev(&p);
    // The guest was moved once, to the host's cell.
    assert_eq!(p.guest.warps(), 1);
    assert_eq!(live::local(&mut p.guest.e).unwrap().cell, HOUSE);
    // Both walk; the avatars follow.
    p.host.move_player(HOUSE, [150.0, 220.0, 0.0], 2.0);
    p.guest.move_player(HOUSE, [10.0, 20.0, 0.0], 0.5);
    p.frames(120);
    assert_eq!(
        rungs(p.guest_lines()),
        [(1, true), (2, true), (3, true), (4, true), (5, true)]
    );
    assert_eq!(
        rungs(p.host_lines()),
        [(1, true), (2, true), (4, true), (5, true)]
    );
    let g = p.guest.placed();
    let h = p.host.placed();
    assert_eq!((g.len(), h.len()), (1, 1));
    // (id, cell, position, AI on, disabled): on the guest the host's avatar, and the reverse.
    assert!(close(g[0].2, [150.0, 220.0, 0.0]), "{:?}", g[0]);
    assert!(close(h[0].2, [10.0, 20.0, 0.0]), "{:?}", h[0]);
    assert!(!g[0].3 && !h[0].3, "avatars' AI stays off");
    // Every entering line comes before its rung's result.
    for lines in [p.guest_lines(), p.host_lines()] {
        for n in [3, 4, 5] {
            let enter = lines
                .iter()
                .position(|l| l.starts_with(&format!("COOP-RUNG {n} entering")));
            let done = lines
                .iter()
                .position(|l| l.starts_with(&format!("COOP-RUNG {n} PASS")));
            if let Some(done) = done {
                assert!(enter.is_some_and(|e| e < done), "rung {n}: {lines:?}");
            }
        }
    }
    // Each rung's entering line is written once (PC run 2026-11-10: rung 5's was written every
    // frame until it passed).
    for lines in [p.guest_lines(), p.host_lines()] {
        for n in 1..=5 {
            let entered = lines
                .iter()
                .filter(|l| l.starts_with(&format!("COOP-RUNG {n} entering")))
                .count();
            assert!(entered <= 1, "rung {n} entered {entered} times");
        }
    }
    // The story channel never saw any of it.
    assert_eq!(story_rev(&p), rev);
    assert!(echo(&mut p.guest.e, &p.gd.sync.store).is_empty());
    assert_eq!((p.gd.sync.desyncs_detected, p.gd.refused), (0, 0));
    assert_eq!(p.guest.scripts_run(), 0);
}

#[test]
fn a_guest_still_at_its_menu_is_left_alone_and_told_why() {
    let mut p = Pair::new(Cap::Avatar, &spec());
    p.host.move_player(HOUSE, [1.0, 2.0, 3.0], 0.0);
    // The guest's player is in no cell (its main menu): nothing in its game is touched.
    p.frames(400);
    assert_eq!(p.guest.warps(), 0);
    assert_eq!(live::local(&mut p.guest.e), None);
    assert_eq!(rungs(p.guest_lines()), [(2, true)]);
    let hints = p
        .guest_lines()
        .iter()
        .filter(|l| l.contains("press New Game"))
        .count();
    assert_eq!(hints, 1);
    // Once it is in the world (it pressed New Game), it is moved to the host, where the host is.
    p.guest.move_player(TOWN, [0.0; 3], 0.0);
    p.frames(20);
    assert_eq!(p.guest.warps(), 1);
    let l = live::local(&mut p.guest.e).unwrap();
    assert_eq!((l.cell, l.pos), (HOUSE, [1.0, 2.0, 3.0]));
}

#[test]
fn a_guest_the_game_will_not_move_now_is_asked_again_later() {
    let mut p = Pair::new(Cap::Avatar, &spec());
    p.host.move_player(HOUSE, [1.0, 2.0, 3.0], 0.0);
    p.guest.move_player(TOWN, [0.0; 3], 0.0);
    p.guest.set_busy(true);
    p.frames(60);
    assert_eq!(p.guest.warps(), 0);
    assert!(!p.guest_lines().iter().any(|l| l.contains("FAIL")));
    p.guest.set_busy(false);
    p.frames(120);
    assert_eq!(p.guest.warps(), 1);
    assert!(rungs(p.guest_lines()).contains(&(3, true)));
}

#[test]
fn players_outdoors_share_the_world_space_whatever_their_cells() {
    let mut s = spec();
    s.exteriors = vec![(0x0010_5b00, 0xda7e), (0x0010_5b01, 0xda7e)];
    let net_spec = s.clone();
    let mut p = Pair::new(Cap::Avatar, &net_spec);
    p.host = Story::new(&s);
    p.hd.start(&mut p.host.e);
    p.host.move_player(0x0010_5b00, [100.0, 0.0, 0.0], 0.0);
    // The guest indoors: it is moved out to the host's world space.
    p.guest.move_player(HOUSE, [0.0; 3], 0.0);
    p.frames(40);
    assert_eq!(live::local(&mut p.guest.e).unwrap().cell, 0xda7e);
    // The host walks into the next exterior cell: no new move of the guest, the avatar stays.
    p.host.move_player(0x0010_5b01, [4200.0, 0.0, 0.0], 0.0);
    p.frames(150);
    assert_eq!(p.guest.warps(), 1);
    let g = p.guest.placed();
    assert_eq!(g.len(), 1);
    assert!(close(g[0].2, [4200.0, 0.0, 0.0]), "{:?}", g[0]);
}

/// PC run 30: a player fast travelled; in the other's game the avatar kept up (moved with
/// `SetPos`) but stayed in the cell it was placed in, which that game let go when its player went
/// there too, and so was drawn no more until both went into the saloon. An avatar in a cell this
/// game does not have loaded is brought into this player's cell, at its place, once.
#[test]
fn an_avatar_left_in_a_cell_this_game_let_go_is_brought_into_the_players_cell() {
    let mut s = spec();
    s.exteriors = vec![(0x0010_5b00, 0xda7e), (0x0010_5b01, 0xda7e)];
    let mut p = Pair::new(Cap::Avatar, &s);
    p.host = Story::new(&s);
    p.hd.start(&mut p.host.e);
    p.host.move_player(0x0010_5b00, [100.0, 0.0, 0.0], 0.0);
    p.guest.move_player(0x0010_5b00, [300.0, 0.0, 0.0], 0.0);
    p.frames(40);
    assert_eq!(p.host.placed()[0].1, 0x0010_5b00);
    // The guest fast travels far: the host's game has its avatar's cell loaded still, then lets it
    // go; the guest is too far to be seen, and its avatar is only moved.
    p.guest.move_player(0x0010_5b01, [40_000.0, 0.0, 0.0], 0.0);
    p.frames(10);
    p.host.set_cell_loaded(0x0010_5b00, false);
    p.frames(10);
    assert_eq!(p.host.placed()[0].1, 0x0010_5b00);
    assert!(close(p.host.placed()[0].2, [40_000.0, 0.0, 0.0]));
    assert!(!p
        .host_lines()
        .iter()
        .any(|l| l.contains("this game let go")));
    // The host goes there too.
    p.host.move_player(0x0010_5b01, [40_200.0, 0.0, 0.0], 0.0);
    p.host.set_cell_loaded(0x0010_5b00, false);
    p.frames(20);
    let h = p.host.placed();
    assert_eq!(h.len(), 1, "{h:?}");
    assert_eq!(h[0].1, 0x0010_5b01, "{h:?}");
    assert!(close(h[0].2, [40_000.0, 0.0, 0.0]), "{h:?}");
    // Brought over once: in a loaded cell it is only moved.
    let let_go = |p: &Pair| {
        p.host_lines()
            .iter()
            .filter(|l| l.contains("this game let go"))
            .count()
    };
    assert_eq!(let_go(&p), 1, "{:?}", p.host_lines());
    p.frames(40);
    assert_eq!(let_go(&p), 1);
    // The game took it out of its cell altogether (as seen after a fast travel in the harness):
    // brought into this player's cell again.
    let a = coop_engine::forms::form(&mut p.host.e, h[0].0).unwrap();
    p.host
        .e
        .mem
        .set_u32(a + coop_engine::addr::REF_CELL_OFFSET, 0);
    p.frames(5);
    assert_eq!(p.host.placed()[0].1, 0x0010_5b01);
    assert_eq!(let_go(&p), 2);
}

#[test]
fn the_cap_stops_the_ladder_at_its_rung() {
    for (cap, warps, placed) in [(Cap::Log, 0, 0), (Cap::Warp, 1, 0), (Cap::Avatar, 1, 1)] {
        let mut p = Pair::new(cap, &spec());
        p.host.move_player(HOUSE, [1.0, 2.0, 3.0], 0.0);
        p.guest.move_player(TOWN, [0.0; 3], 0.0);
        p.frames(80);
        assert_eq!(p.guest.warps(), warps, "{cap:?}");
        assert_eq!(p.guest.placed().len(), placed, "{cap:?}");
        // Rungs 1 and 2 are reported under every cap.
        assert_eq!(
            rungs(p.guest_lines())[..2],
            [(1, true), (2, true)],
            "{cap:?}"
        );
        if cap == Cap::Log {
            assert_eq!(p.host.placed().len(), 0);
        }
    }
    assert_eq!(Cap::parse("log"), Cap::Log);
    assert_eq!(Cap::parse(" warp "), Cap::Warp);
    assert_eq!(Cap::parse(""), Cap::Avatar);
}

#[test]
fn a_game_that_refuses_the_avatar_fails_rung_4_once_and_goes_on() {
    let mut p = Pair::new(Cap::Avatar, &spec());
    p.guest.refuse_placing();
    p.host.move_player(HOUSE, [1.0, 2.0, 3.0], 0.0);
    p.guest.move_player(HOUSE, [9.0, 9.0, 9.0], 0.0);
    p.frames(200);
    assert_eq!(
        rungs(p.guest_lines()),
        [(1, true), (2, true), (3, true), (4, false)]
    );
    let fails = p
        .guest_lines()
        .iter()
        .filter(|l| l.contains("FAIL"))
        .count();
    assert_eq!(fails, 1);
    // The host's side is unaffected.
    assert!(rungs(p.host_lines()).contains(&(5, true)));
}

#[test]
fn a_host_cell_the_guest_lacks_fails_rung_3_by_name() {
    let mut other = spec();
    other.cells = vec![TOWN];
    let mut p = Pair::new(Cap::Avatar, &other);
    p.host.move_player(HOUSE, [1.0, 2.0, 3.0], 0.0);
    p.guest.move_player(TOWN, [0.0; 3], 0.0);
    p.frames(60);
    let line = p
        .guest_lines()
        .iter()
        .find(|l| l.starts_with("COOP-RUNG 3 FAIL"))
        .unwrap();
    assert!(line.contains("00105a00"), "{line}");
    assert_eq!(p.guest.warps(), 0);
    assert_eq!(p.guest.placed().len(), 0);
}

#[test]
fn an_avatar_the_game_wakes_is_put_back_to_sleep_and_counted() {
    let mut p = Pair::new(Cap::Avatar, &spec());
    p.host.move_player(HOUSE, [1.0, 2.0, 3.0], 0.0);
    p.guest.move_player(HOUSE, [9.0, 9.0, 9.0], 0.0);
    p.frames(40);
    p.guest.wake_placed();
    p.frames(2);
    assert!(!p.guest.placed()[0].3);
    assert!(p.gd.ladder.as_ref().unwrap().status().contains("woke 1"));
}

#[test]
fn the_guest_follows_the_host_to_another_cell_and_avatars_are_placed_anew() {
    let mut p = Pair::new(Cap::Avatar, &spec());
    p.host.move_player(HOUSE, [1.0, 2.0, 3.0], 0.0);
    p.guest.move_player(TOWN, [0.0; 3], 0.0);
    p.frames(40);
    p.host.move_player(TOWN, [50.0, 60.0, 0.0], 0.0);
    // Not before five seconds after the last move of the guest.
    p.frames(140);
    assert_eq!(p.guest.warps(), 2);
    assert_eq!(live::local(&mut p.guest.e).unwrap().cell, TOWN);
    let g = p.guest.placed();
    // The first avatar was removed when the host left the cell; a second stands in the new one.
    assert_eq!(g.len(), 2);
    assert!(g[0].4 && !g[1].4);
    assert_eq!(g[1].1, TOWN);
    assert!(close(g[1].2, [50.0, 60.0, 0.0]));
}

/// PC run 2026-11-10: the guest had been with the host indoors for minutes when the host went
/// outdoors, so the guest was moved in the very frame it learned of it, and no frame saw the two
/// in different cells. The avatar placed indoors was kept and moved, still in the house: the guest
/// did not see the host outdoors.
#[test]
fn an_avatar_placed_in_one_cell_is_placed_anew_when_its_player_is_followed_at_once() {
    let mut p = Pair::new(Cap::Avatar, &spec());
    p.host.move_player(HOUSE, [1.0, 2.0, 3.0], 0.0);
    p.guest.move_player(TOWN, [0.0; 3], 0.0);
    // Long enough that the next move of the guest is not held back.
    p.frames(160);
    assert_eq!(p.guest.placed().len(), 1);
    p.host.move_player(TOWN, [50.0, 60.0, 0.0], 0.0);
    p.frames(40);
    assert_eq!(p.guest.warps(), 2);
    let g = p.guest.placed();
    assert_eq!(g.len(), 2, "{g:?}");
    assert!(g[0].4, "the avatar left indoors is removed: {:?}", g[0]);
    assert_eq!((g[1].1, g[1].4), (TOWN, false));
    assert!(close(g[1].2, [50.0, 60.0, 0.0]));
    assert!(
        p.guest_lines()
            .iter()
            .any(|l| l.contains("placed 2 times") && l.contains(&format!("{TOWN:08x}"))),
        "{:?}",
        p.guest_lines()
    );
}

#[test]
fn a_guest_that_leaves_has_its_avatar_removed_on_the_host() {
    let mut p = Pair::new(Cap::Avatar, &spec());
    p.host.move_player(HOUSE, [1.0, 2.0, 3.0], 0.0);
    p.guest.move_player(HOUSE, [9.0, 9.0, 9.0], 0.0);
    p.frames(40);
    assert!(!p.host.placed()[0].4);
    p.hd.sync.disconnect(PlayerId(1));
    p.t += 50;
    p.hd.step(&mut p.host.e, &mut p.hnet, p.t);
    assert!(p.host.placed()[0].4, "disabled");
    assert!(p
        .host_lines()
        .iter()
        .any(|l| l.contains("removed (player 1 left)")));
}

/// PC run 31: after a fast travel the guest was invisible in the host's game (and the mantises
/// there never went for it), its avatar in a loaded cell near the host with no 3D. An avatar there
/// and not drawn for a moment is taken away and placed again.
#[test]
fn an_avatar_there_and_not_drawn_is_placed_again() {
    let mut p = Pair::new(Cap::Avatar, &spec());
    p.host.move_player(HOUSE, [100.0, 0.0, 0.0], 0.0);
    p.guest.move_player(HOUSE, [300.0, 0.0, 0.0], 0.0);
    p.frames(40);
    let live = |p: &mut Pair| -> Vec<(u32, u32, [f32; 3], bool, bool)> {
        p.host.placed().into_iter().filter(|a| !a.4).collect()
    };
    let first = live(&mut p)[0].0;
    p.host.set_has_3d(first, false);
    p.frames((coop_engine_session::ladder::UNSEEN_MS / 50) as u32 - 5);
    assert_eq!(live(&mut p)[0].0, first);
    p.frames(10);
    let again = live(&mut p);
    assert_eq!(again.len(), 1, "{again:?}");
    assert_ne!(again[0].0, first);
    assert!(close(again[0].2, [300.0, 0.0, 0.0]), "{again:?}");
    assert!(p.host_lines().iter().any(|l| l.contains("had no 3D")));
    // Drawn: left as it is.
    p.frames(60);
    assert_eq!(live(&mut p)[0].0, again[0].0);
    // Far from this player (out of the cells its game keeps), not drawn as it should be: left.
    p.guest.move_player(HOUSE, [40_000.0, 0.0, 0.0], 0.0);
    p.frames(10);
    let far = live(&mut p)[0].0;
    p.host.set_has_3d(far, false);
    p.frames(3 * (coop_engine_session::ladder::UNSEEN_MS / 50) as u32);
    assert_eq!(live(&mut p)[0].0, far);
}

/// The owner, 2026-11-16 (the opening is each player's own): in Doc Mitchell's house the other
/// player's avatar took the couch's seat as its player sat, and this game's player could not sit
/// for its own questions. While a game's own opening runs no other player is shown in it; once it
/// is done they are.
#[test]
fn no_other_player_is_shown_while_this_games_own_opening_runs() {
    let mut p = Pair::new(Cap::Avatar, &spec());
    p.hd.ladder.as_mut().unwrap().follow_after = Some((MQ, 20));
    p.host.move_player(HOUSE, [1.0, 2.0, 3.0], 0.0);
    p.guest.move_player(HOUSE, [9.0, 9.0, 9.0], 0.0);
    p.frames(40);
    // The host's opening runs: nobody placed in its game. The guest's game (no opening of its
    // own running) shows the host.
    assert!(p.host.placed().iter().all(|a| a.4), "{:?}", p.host.placed());
    assert!(p.hd.ladder.as_ref().unwrap().avatar_ids().is_empty());
    assert_eq!(p.gd.ladder.as_ref().unwrap().avatar_ids().len(), 1);
    // The opening's last stage is done: the guest is shown.
    assert!(p.host.set_stage(MQ, 20));
    p.frames(40);
    assert_eq!(p.hd.ladder.as_ref().unwrap().avatar_ids().len(), 1);
    assert!(p.host.placed().iter().any(|a| !a.4));
}

//! Party saves through `Session`: a host and two guests over the loopback transport, driven only
//! through the frame calls the viewer makes. Saves go to temp folders.

use std::path::{Path, PathBuf};

use coop_core::ids::PlayerId;
use coop_core::transport::{ChaosProfile, LoopbackNet};
use coop_session::{PartyEvent, Session, SessionConfig};
use esm::{ActivePlugins, FormId, LoadOrder};
use testdata::quest_ids::*;
use world::coop::apply::{Ctx, Opts};
use world::coop::party_char::{carry_coop, CharacterFile};
use world::coop::party_save::{load_party, party_dir_for};
use world::coop::shadow::PlayerShadow;
use world::dialogue::PLAYER_REF;
use world::scripting::{GameState, ScriptCache};

const DT: f32 = 0.025;
const KEY_A: u64 = 0xA11CE;
const KEY_B: u64 = 0xB0B;
const KEY_C: u64 = 0xC0DE;

struct Peer {
    s: Session,
    gs: GameState,
    events: Vec<PartyEvent>,
    alive: bool,
}

struct Rig<'a> {
    order: &'a LoadOrder,
    scripts: &'a ScriptCache,
    host: Peer,
    guests: Vec<Peer>,
}

fn cfg(
    name: &str,
    t: Box<dyn coop_core::transport::Transport>,
    chaos: Option<(&str, u64)>,
) -> SessionConfig {
    SessionConfig {
        name: name.into(),
        session_id: 0xABCD,
        transport: Some(t),
        chaos: chaos.map(|(p, seed)| (ChaosProfile::by_name(p).unwrap(), seed)),
        live: true,
        ..SessionConfig::default()
    }
}

fn tmp(tag: &str) -> PathBuf {
    let d = std::env::temp_dir().join(format!("nvrs-sess-party-{tag}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&d);
    std::fs::create_dir_all(&d).unwrap();
    d
}

impl<'a> Rig<'a> {
    /// `guests`: (name, key, endpoint id): the endpoint id decides the slot.
    fn new(
        order: &'a LoadOrder,
        scripts: &'a ScriptCache,
        chaos: Option<&str>,
        guests: &[(&str, u64, u8)],
        dir: &Path,
    ) -> Rig<'a> {
        let net = LoopbackNet::new();
        let mut host = Session::host(cfg(
            "Host",
            Box::new(net.endpoint(PlayerId::HOST)),
            chaos.map(|p| (p, 1)),
        ))
        .unwrap();
        host.party_host_setup(0, "Host");
        let mk = |s| Peer {
            s,
            gs: GameState::new(order),
            events: Vec::new(),
            alive: true,
        };
        let guests = guests
            .iter()
            .enumerate()
            .map(|(i, (name, key, ep))| {
                let mut g = Session::join(cfg(
                    name,
                    Box::new(net.endpoint(PlayerId(*ep))),
                    chaos.map(|p| (p, 10 + i as u64)),
                ))
                .unwrap();
                g.party_guest_setup(*key, Some(dir.join(format!("{name}.char"))));
                mk(g)
            })
            .collect();
        Rig {
            order,
            scripts,
            host: mk(host),
            guests,
        }
    }

    fn frame(&mut self) {
        let cx = Ctx {
            order: self.order,
            scripts: self.scripts,
        };
        let all = std::iter::once(&mut self.host).chain(self.guests.iter_mut());
        for p in all.filter(|p| p.alive) {
            p.s.pre_frame_with(
                &mut p.gs,
                Opts {
                    present: true,
                    ctx: Some(cx),
                },
            );
            p.gs.player_cell = Some(FormId(CELL));
            p.gs.player_position = Some([900.0, 1000.0, 0.0]);
            p.s.live_frame(&mut p.gs, cx, DT);
            p.s.post_frame(&mut p.gs, DT);
            p.events.extend(p.s.take_party_events());
        }
    }

    fn until(&mut self, max: u32, mut cond: impl FnMut(&mut Rig<'a>) -> bool) -> bool {
        for _ in 0..max {
            if cond(self) {
                return true;
            }
            self.frame();
        }
        cond(self)
    }

    fn settled(&mut self) {
        let ok = self.until(12000, |r| {
            r.guests
                .iter()
                .all(|g| g.s.is_connected() && g.s.is_ready())
                && r.host.s.is_ready()
        });
        assert!(ok, "never connected");
        let ok = self.until(6000, |r| {
            r.guests.iter().all(|g| {
                g.events
                    .iter()
                    .any(|e| matches!(e, PartyEvent::Campaign { .. }))
            })
        });
        assert!(ok, "the guests never heard the campaign");
        self.frames(40);
    }

    fn frames(&mut self, n: u32) {
        for _ in 0..n {
            self.frame();
        }
    }
}

const CELL: u32 = 0xA00;

fn order(tag: &str) -> (testdata::TempData, LoadOrder) {
    let data = testdata::quests(tag);
    let order = LoadOrder::from_data_dir(data.path(), &ActivePlugins::OfficialOnly).unwrap();
    (data, order)
}

/// Gives the guest in `slot` a recognisable character (through the host, as the rules would).
fn make_character(host: &mut GameState, slot: u8, caps: i32, level: u16) {
    host.with_player(slot, |g| {
        *g.items.entry((PLAYER_REF, FormId(CAPS))).or_insert(0) += caps;
        g.player_level = level;
        g.skill_points.insert(32, u32::from(level));
    });
}

/// A character without its place (the live rows carry the cell the player stands in).
fn core(mut s: PlayerShadow) -> PlayerShadow {
    s.cell = None;
    s.world = None;
    s.position = None;
    s.heading = 0;
    s
}

fn caps_of(gs: &GameState, slot: u8) -> i32 {
    gs.shadow_of(slot)
        .and_then(|s| s.items.get(&FormId(CAPS)).copied())
        .unwrap_or(0)
}

#[test]
fn a_host_save_orders_every_guest_to_save_and_the_party_comes_back_by_key() {
    for chaos in [None, Some("good_wifi")] {
        let (_d, order) = order("sess-party");
        let scripts = ScriptCache::default();
        let dir = tmp("round");
        let mut rig = Rig::new(
            &order,
            &scripts,
            chaos,
            &[("Ann", KEY_A, 1), ("Bo", KEY_B, 2)],
            &dir,
        );
        rig.settled();
        // slots follow the endpoint ids
        make_character(&mut rig.host.gs, 1, 70, 4);
        make_character(&mut rig.host.gs, 2, 5, 9);
        rig.host.gs.player_level = 2;
        rig.host.gs.topics.insert(FormId(TOPIC_SECRET));
        rig.frames(400);
        let ann = core(rig.host.gs.shadow_of(1).unwrap());
        let bo = core(rig.host.gs.shadow_of(2).unwrap());
        assert_eq!(
            core(rig.guests[0].gs.shadow_of(1).unwrap()),
            ann,
            "the guest holds what the host holds"
        );
        assert_ne!(ann, bo);

        let save_dir = party_dir_for(&dir.join("Save 1.txt"));
        let text = world::save::save(&rig.host.gs, None);
        let seq = rig
            .host
            .s
            .party_begin_save(&rig.host.gs, save_dir.clone(), text.clone(), 1_700_000_000)
            .unwrap();
        assert!(rig.host.s.party_save_pending());
        let ok = rig.until(4000, |r| {
            r.host
                .events
                .iter()
                .any(|e| matches!(e, PartyEvent::HostSaved { .. }))
        });
        assert!(ok, "no HostSaved ({chaos:?})");
        let saved = rig.host.events.iter().find_map(|e| {
            if let PartyEvent::HostSaved {
                guests,
                from_guests,
                seq: s,
                ..
            } = e
            {
                Some((*guests, *from_guests, *s))
            } else {
                None
            }
        });
        assert_eq!(saved, Some((2, 2, seq)), "both guests answered themselves");
        rig.frames(80);
        for g in &rig.guests {
            assert!(
                g.events.iter().any(
                    |e| matches!(e, PartyEvent::GuestSaved { seq: s, path: Some(_) } if *s == seq)
                ),
                "{:?}",
                g.events
            );
            assert!(g
                .events
                .iter()
                .any(|e| matches!(e, PartyEvent::Stored { ok: true, .. })));
        }
        // the guests kept a local copy of their own character
        let mine = world::coop::party_char::read_character(&dir.join("Ann.char")).unwrap();
        assert_eq!(mine.key, KEY_A);
        assert_eq!(core(mine.shadow().unwrap()), ann);
        let campaign = rig.host.s.party_campaign();
        assert_ne!(campaign, 0);
        assert_eq!(mine.campaign, campaign);
        // a second save while none is pending works and replaces the first
        let seq2 = rig
            .host
            .s
            .party_begin_save(&rig.host.gs, save_dir.clone(), text.clone(), 0)
            .unwrap();
        assert!(seq2 > seq);
        assert!(
            rig.host
                .s
                .party_begin_save(&rig.host.gs, save_dir.clone(), text.clone(), 0)
                .is_err(),
            "one at a time"
        );
        assert!(rig.until(4000, |r| r
            .host
            .events
            .iter()
            .filter(|e| matches!(e, PartyEvent::HostSaved { .. }))
            .count()
            == 2));
        drop(rig);

        // everything is dropped: a new host loads, the guests rejoin with the endpoint ids swapped
        let loaded = load_party(&save_dir).unwrap();
        assert!(loaded.problems.is_empty());
        assert_eq!(loaded.save.campaign, campaign);
        assert_eq!(loaded.save.seq, seq2);
        let mut rig = Rig::new(
            &order,
            &scripts,
            chaos,
            &[("Bo", KEY_B, 1), ("Ann", KEY_A, 2)],
            &dir,
        );
        let (mut state, _) = world::save::load(&loaded.save.world_text).unwrap();
        carry_coop(&mut rig.host.gs, &mut state, Some(&order));
        rig.host.gs = state;
        rig.host.s.party_host_load(&loaded.save);
        rig.settled();
        rig.frames(400);
        assert_eq!(rig.host.s.party_campaign(), campaign);
        assert!(rig.guests[0]
            .events
            .iter()
            .any(|e| matches!(e, PartyEvent::Campaign { matched: true, id } if *id == campaign)));
        assert_eq!(caps_of(&rig.host.gs, 1), caps_of(&rig.guests[0].gs, 1));
        assert_eq!(
            core(rig.host.gs.shadow_of(1).unwrap()),
            bo,
            "Bo is in slot 1 now and has Bo's character"
        );
        assert_eq!(
            core(rig.host.gs.shadow_of(2).unwrap()),
            ann,
            "Ann is in slot 2 now and has Ann's"
        );
        assert_eq!(core(rig.guests[1].gs.shadow_of(2).unwrap()), ann);
        assert!(rig.host.gs.topics.contains(&FormId(TOPIC_SECRET)));
        assert_eq!(rig.host.gs.player_level, 2);
    }
}

/// The replicated shadow leaves a player's health damage out (each machine keeps its own
/// player's health). A saved character's health must still come back on a rejoin by key and
/// on a guest's F9, and ordinary host writes must not heal the guest.
#[test]
fn a_saved_characters_health_comes_back_on_rejoin_and_on_f9() {
    let (_d, order) = order("sess-health");
    let scripts = ScriptCache::default();
    let dir = tmp("health");
    let mut rig = Rig::new(&order, &scripts, None, &[("Ann", KEY_A, 1)], &dir);
    rig.settled();
    // Ann was hurt on her own machine
    rig.guests[0].gs.damage.insert(PLAYER_REF, 12.5);
    rig.frames(100);
    let save_dir = party_dir_for(&dir.join("Save 1.txt"));
    let text = world::save::save(&rig.host.gs, None);
    rig.host
        .s
        .party_begin_save(&rig.host.gs, save_dir.clone(), text, 1_700_000_000)
        .unwrap();
    assert!(rig.until(4000, |r| r
        .host
        .events
        .iter()
        .any(|e| matches!(e, PartyEvent::HostSaved { .. }))));
    rig.frames(80);
    // Ann heals before the F9: the file still holds the saved health
    rig.guests[0].gs.damage.remove(&PLAYER_REF);
    rig.guests[0].s.party_guest_load_local().unwrap();
    assert!(rig.until(4000, |r| r.guests[0]
        .events
        .iter()
        .any(|e| matches!(e, PartyEvent::OfferAnswered { ok: true, .. }))));
    rig.frames(100);
    assert_eq!(
        rig.guests[0].gs.damage.get(&PLAYER_REF),
        Some(&12.5),
        "F9 brings the saved health back"
    );
    // an ordinary host write to Ann's character keeps her health
    make_character(&mut rig.host.gs, 1, 3, 2);
    rig.frames(200);
    assert_eq!(caps_of(&rig.guests[0].gs, 1), caps_of(&rig.host.gs, 1));
    assert_eq!(rig.guests[0].gs.damage.get(&PLAYER_REF), Some(&12.5));
    drop(rig);

    // a new host loads the party save, Ann rejoins by key with full health
    let loaded = load_party(&save_dir).unwrap();
    let mut rig = Rig::new(&order, &scripts, None, &[("Ann", KEY_A, 1)], &dir);
    let (mut state, _) = world::save::load(&loaded.save.world_text).unwrap();
    carry_coop(&mut rig.host.gs, &mut state, Some(&order));
    rig.host.gs = state;
    rig.host.s.party_host_load(&loaded.save);
    rig.settled();
    rig.frames(400);
    assert!(rig.guests[0]
        .events
        .iter()
        .any(|e| matches!(e, PartyEvent::Campaign { matched: true, .. })));
    assert_eq!(
        rig.guests[0].gs.damage.get(&PLAYER_REF),
        Some(&12.5),
        "the rejoin brings the saved health back"
    );
}

/// A player whose key already has a slot (a frozen game) joins again under another slot. Its
/// character moves to the new slot, and the health that the guest last reported comes too.
#[test]
fn a_returning_key_on_another_slot_keeps_its_health() {
    let (_d, order) = order("sess-moved");
    let scripts = ScriptCache::default();
    let dir = tmp("moved");
    let mut rig = Rig::new(
        &order,
        &scripts,
        None,
        &[("Ann", KEY_A, 1), ("Ann again", KEY_A, 2)],
        &dir,
    );
    // Ann's second game does not run yet
    rig.guests[1].alive = false;
    assert!(
        rig.until(12000, |r| r.guests[0].s.is_connected()
            && r.guests[0].s.is_ready()
            && r.host.s.is_ready()),
        "never connected"
    );
    assert!(rig.until(6000, |r| r.guests[0]
        .events
        .iter()
        .any(|e| matches!(e, PartyEvent::Campaign { .. }))));
    // Ann is hurt on her machine: the host's copy of her character learns it
    rig.guests[0].gs.damage.insert(PLAYER_REF, 7.5);
    assert!(
        rig.until(400, |r| r.host.gs.shadow_of(1).and_then(|s| s.damage)
            == Some(7.5)),
        "the host never heard Ann's health"
    );
    // Ann's game freezes; she starts again and joins under slot 2 with full health
    rig.guests[0].alive = false;
    rig.guests[1].alive = true;
    assert!(
        rig.until(12000, |r| r.guests[1]
            .events
            .iter()
            .any(|e| matches!(e, PartyEvent::Campaign { matched: true, .. }))),
        "Ann's key was not matched"
    );
    rig.frames(100);
    assert_eq!(
        rig.guests[1].gs.damage.get(&PLAYER_REF),
        Some(&7.5),
        "the moved character keeps its health"
    );
}

#[test]
fn a_guest_that_does_not_answer_gets_the_hosts_copy_of_its_character() {
    let (_d, order) = order("sess-silent");
    let scripts = ScriptCache::default();
    let dir = tmp("silent");
    let mut rig = Rig::new(
        &order,
        &scripts,
        None,
        &[("Ann", KEY_A, 1), ("Bo", KEY_B, 2)],
        &dir,
    );
    rig.settled();
    make_character(&mut rig.host.gs, 2, 33, 6);
    // Bo was hurt on his own machine: the host's copy of his character learns it
    rig.guests[1].gs.damage.insert(PLAYER_REF, 9.0);
    rig.frames(300);
    let bo = core(rig.host.gs.shadow_of(2).unwrap());
    assert_eq!(bo.damage, Some(9.0), "the host's copy has Bo's health");
    // Bo's game freezes; the link stays up long enough for the order to be sent
    rig.guests[1].alive = false;
    let save_dir = dir.join("s.txt.partysave");
    rig.host
        .s
        .party_begin_save(
            &rig.host.gs,
            save_dir.clone(),
            world::save::save(&rig.host.gs, None),
            1,
        )
        .unwrap();
    assert!(rig.until(4000, |r| r
        .host
        .events
        .iter()
        .any(|e| matches!(e, PartyEvent::HostSaved { .. }))));
    let saved = rig.host.events.iter().find_map(|e| {
        if let PartyEvent::HostSaved {
            guests,
            from_guests,
            ..
        } = e
        {
            Some((*guests, *from_guests))
        } else {
            None
        }
    });
    assert_eq!(
        saved,
        Some((2, 1)),
        "Ann answered, Bo's character is the host's copy"
    );
    let l = load_party(&save_dir).unwrap();
    let sc = l.guest(KEY_B).unwrap();
    assert!(!sc.from_guest);
    assert_eq!(core(sc.shadow().unwrap()), bo);
    assert_eq!(
        sc.shadow().unwrap().damage,
        Some(9.0),
        "the save keeps Bo's health from before he froze"
    );
    assert!(l.guest(KEY_A).unwrap().from_guest);
}

#[test]
fn offline_guests_keep_their_characters_through_later_saves() {
    let (_d, order) = order("sess-offline");
    let scripts = ScriptCache::default();
    let dir = tmp("offline");
    let mut rig = Rig::new(
        &order,
        &scripts,
        None,
        &[("Ann", KEY_A, 1), ("Bo", KEY_B, 2)],
        &dir,
    );
    rig.settled();
    make_character(&mut rig.host.gs, 1, 11, 3);
    make_character(&mut rig.host.gs, 2, 22, 5);
    rig.frames(300);
    let save_dir = dir.join("o.txt.partysave");
    rig.host
        .s
        .party_begin_save(
            &rig.host.gs,
            save_dir.clone(),
            world::save::save(&rig.host.gs, None),
            1,
        )
        .unwrap();
    assert!(rig.until(4000, |r| r
        .host
        .events
        .iter()
        .any(|e| matches!(e, PartyEvent::HostSaved { .. }))));
    let first = load_party(&save_dir).unwrap();
    drop(rig);
    // a new session with only Ann: Bo's sidecar must survive the next save
    let mut rig = Rig::new(&order, &scripts, None, &[("Ann", KEY_A, 1)], &dir);
    let (mut state, _) = world::save::load(&first.save.world_text).unwrap();
    carry_coop(&mut rig.host.gs, &mut state, Some(&order));
    rig.host.gs = state;
    rig.host.s.party_host_load(&first.save);
    rig.settled();
    rig.frames(300);
    rig.host
        .s
        .party_begin_save(
            &rig.host.gs,
            save_dir.clone(),
            world::save::save(&rig.host.gs, None),
            2,
        )
        .unwrap();
    assert!(rig.until(4000, |r| r
        .host
        .events
        .iter()
        .any(|e| matches!(e, PartyEvent::HostSaved { .. }))));
    let second = load_party(&save_dir).unwrap();
    assert_eq!(second.save.guests.len(), 2);
    assert_eq!(
        second.guest(KEY_B),
        first.guest(KEY_B),
        "Bo's sidecar is carried forward unchanged"
    );
    assert_eq!(second.guest(KEY_A).unwrap().shadow().unwrap().level, 3);
}

#[test]
fn a_late_joiner_with_its_own_character_is_accepted_once_under_the_hosts_rules() {
    let (_d, order) = order("sess-late");
    let scripts = ScriptCache::default();
    let dir = tmp("late");
    let mut rig = Rig::new(&order, &scripts, None, &[("Cy", KEY_C, 1)], &dir);
    let mut mine = PlayerShadow::fresh();
    mine.level = 12;
    mine.items.insert(FormId(CAPS), 999);
    mine.items.insert(FormId(0xFE00_0002), 1);
    mine.cell = Some(FormId(0x1234));
    let file = CharacterFile::new(KEY_C, "Cy", 0, 0, 1, &mine, vec![(0xDEAD, 20)]);
    rig.guests[0].s.party_guest_offer(file);
    rig.host.gs.stages.insert(FormId(0xDEAD), 50);
    rig.settled();
    assert!(rig.until(3000, |r| r.guests[0]
        .events
        .iter()
        .any(|e| matches!(e, PartyEvent::OfferAnswered { .. }))));
    let ans = rig.guests[0].events.iter().find_map(|e| {
        if let PartyEvent::OfferAnswered { ok, conflicts, .. } = e {
            Some((*ok, *conflicts))
        } else {
            None
        }
    });
    assert_eq!(
        ans,
        Some((true, 1)),
        "accepted, one shared stage differs and the host's wins"
    );
    rig.frames(300);
    let got = rig.guests[0].gs.shadow_of(1).unwrap();
    assert_eq!(got.level, 12);
    assert_eq!(got.items.get(&FormId(CAPS)), Some(&999));
    assert!(!got
        .items
        .keys()
        .any(|f| world::coop::party_save::is_synthetic(*f)));
    assert_eq!(rig.guests[0].gs.stages.get(&FormId(0xDEAD)), Some(&50));

    // F5 on the guest writes its character to its file; F9 reads it back
    let g0 = &mut rig.guests[0];
    let saved = g0.s.party_guest_save(&g0.gs).unwrap();
    assert_eq!(
        world::coop::party_char::read_character(&saved)
            .unwrap()
            .shadow()
            .unwrap()
            .level,
        12
    );

    // F9 on that guest with a different file: only its own character changes
    let mut older = PlayerShadow::fresh();
    older.level = 3;
    older.items.insert(FormId(CAPS), 7);
    let f9 = CharacterFile::new(KEY_C, "Cy", 0, 0, 1, &older, vec![]);
    let hash_before = Session::state_root(&rig.host.gs);
    rig.guests[0].s.party_guest_load(&f9);
    assert!(rig.until(3000, |r| r.guests[0]
        .events
        .iter()
        .filter(|e| matches!(e, PartyEvent::OfferAnswered { .. }))
        .count()
        == 2));
    rig.frames(300);
    let back = rig.guests[0].gs.shadow_of(1).unwrap();
    assert_eq!(
        (back.level, back.items.get(&FormId(CAPS)).copied()),
        (3, Some(7))
    );
    assert_eq!(
        rig.host.gs.shadow_of(1).unwrap().level,
        3,
        "the host holds the guest's loaded character"
    );
    assert_eq!(rig.host.gs.stages.get(&FormId(0xDEAD)), Some(&50));
    let _ = hash_before;
}

#[test]
fn an_offer_for_a_known_player_or_another_campaign_is_refused() {
    let (_d, order) = order("sess-refuse");
    let scripts = ScriptCache::default();
    let dir = tmp("refuse");
    let mut rig = Rig::new(&order, &scripts, None, &[("Ann", KEY_A, 1)], &dir);
    rig.settled();
    rig.frames(100);
    assert_ne!(
        rig.host.s.party_campaign(),
        0,
        "a new session is a new campaign"
    );
    let mut s = PlayerShadow::fresh();
    s.level = 40;
    let other = CharacterFile::new(KEY_A, "Ann", 0x1111, 0, 1, &s, vec![]);
    rig.guests[0].s.party_guest_load(&other);
    assert!(rig.until(3000, |r| r.guests[0]
        .events
        .iter()
        .any(|e| matches!(e, PartyEvent::OfferAnswered { .. }))));
    let ans = rig.guests[0]
        .events
        .iter()
        .find_map(|e| {
            if let PartyEvent::OfferAnswered { ok, why, .. } = e {
                Some((*ok, why.clone()))
            } else {
                None
            }
        })
        .unwrap();
    assert!(!ans.0 && ans.1.contains("campaign"), "{ans:?}");
    assert_ne!(rig.host.gs.shadow_of(1).unwrap().level, 40);
}

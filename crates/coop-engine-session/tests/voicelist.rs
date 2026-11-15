//! voiceui: every player's name in every game, and who is speaking, through the session on three
//! games in one process (docs/coop/SPECS.md, voice list).

use coop_core::ids::PlayerId;
use coop_core::proto::{Hello, PROTOCOL_VERSION};
use coop_core::sync::HostConfig;
use coop_core::transport::{LoopbackEndpoint, LoopbackNet};
use coop_engine::testkit::{Story, StorySpec};
use coop_engine_session::ladder::{Cap, Ladder, DEFAULT_AVATAR_BASE};
use coop_engine_session::roster;
use coop_engine_session::voice::VoiceLink;
use coop_engine_session::voicelist::HOLD_MS;
use coop_engine_session::{gate_guest, GuestDriver, HostDriver};

engine_core::assemble!();

const MQ: u32 = 0x0010_4c0c;
const HOUSE: u32 = 0x0010_5a00;

fn spec() -> StorySpec {
    StorySpec {
        quests: vec![(MQ, vec![10, 20])],
        globals: vec![(0x39, 0.0)],
        cells: vec![HOUSE],
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

struct Guest {
    story: Story,
    gd: GuestDriver,
    net: LoopbackEndpoint,
}

struct Party {
    host: Story,
    hd: HostDriver,
    hnet: LoopbackEndpoint,
    guests: Vec<Guest>,
    /// The guests stepped (the others have not started yet).
    active: usize,
    t: u64,
}

impl Party {
    fn new(guests: u8, ladder: bool) -> Party {
        let net = LoopbackNet::new();
        let mut host = Story::new(&spec());
        let mut hd = HostDriver::new(HostConfig::new(hello(), 1), [MQ], [0x39]);
        if ladder {
            hd.ladder = Some(Ladder::new(false, Cap::Avatar, DEFAULT_AVATAR_BASE).quiet());
        }
        hd.voice = Some(VoiceLink::new());
        hd.start(&mut host.e);
        let guests: Vec<Guest> = (1..=guests)
            .map(|n| {
                let mut story = Story::new(&spec());
                gate_guest(&mut story.e);
                let mut gd = GuestDriver::new(PlayerId(n), hello());
                if ladder {
                    gd.ladder = Some(Ladder::new(true, Cap::Avatar, DEFAULT_AVATAR_BASE).quiet());
                }
                gd.voice = Some(VoiceLink::new());
                Guest {
                    story,
                    gd,
                    net: net.endpoint(PlayerId(n)),
                }
            })
            .collect();
        Party {
            hnet: net.endpoint(PlayerId::HOST),
            host,
            hd,
            active: guests.len(),
            guests,
            t: 0,
        }
    }

    fn frames(&mut self, n: u32) {
        for _ in 0..n {
            self.t += 50;
            self.hd.step(&mut self.host.e, &mut self.hnet, self.t);
            for g in self.guests.iter_mut().take(self.active) {
                g.story.run_quest_script();
                g.gd.step(&mut g.story.e, &mut g.net, self.t);
            }
        }
    }

    fn names_of_guest(&self, i: usize) -> Vec<(u8, String)> {
        roster::read(&self.guests[i].gd.sync.store)
            .into_iter()
            .map(|(p, n)| (p.0, n))
            .collect()
    }
}

fn named(list: &[(u8, &str)]) -> Vec<(u8, String)> {
    list.iter().map(|(p, n)| (*p, n.to_string())).collect()
}

#[test]
fn every_guest_knows_every_player_by_name() {
    let mut p = Party::new(2, false);
    p.hd.roster.set(PlayerId::HOST, "Ann");
    p.hd.roster.set(PlayerId(1), "Bo");
    p.hd.roster.set(PlayerId(2), "Cy Dee");
    p.frames(20);
    let all = named(&[(0, "Ann"), (1, "Bo"), (2, "Cy Dee")]);
    assert_eq!(p.names_of_guest(0), all);
    assert_eq!(p.names_of_guest(1), all, "a guest knows the other guest");
    let writes = p.hd.roster.writes;
    p.frames(10);
    assert_eq!(p.hd.roster.writes, writes, "written again only on a change");
    // A guest leaves: the others no longer list it.
    p.hd.sync.disconnect(PlayerId(2));
    p.guests.pop();
    p.frames(5);
    assert_eq!(p.names_of_guest(0), named(&[(0, "Ann"), (1, "Bo")]));
    // A name said again (a rejoin with a new name) reaches everyone.
    p.hd.roster.set(PlayerId(1), "Bo2");
    p.frames(5);
    assert_eq!(p.names_of_guest(0), named(&[(0, "Ann"), (1, "Bo2")]));
}

#[test]
fn a_late_guest_gets_the_names_with_its_first_snapshot() {
    let mut p = Party::new(2, false);
    p.active = 1;
    p.hd.roster.set(PlayerId::HOST, "Ann");
    p.hd.roster.set(PlayerId(1), "Bo");
    p.hd.roster.set(PlayerId(2), "Cy");
    p.frames(20);
    assert_eq!(p.names_of_guest(0), named(&[(0, "Ann"), (1, "Bo")]));
    // The second guest starts now: it gets everyone, and the first guest gets it.
    p.active = 2;
    p.frames(20);
    let all = named(&[(0, "Ann"), (1, "Bo"), (2, "Cy")]);
    assert_eq!(p.names_of_guest(1), all);
    assert_eq!(p.names_of_guest(0), all);
    // The host's own record, read from its own store, is the same.
    let mine: Vec<(u8, String)> = roster::read(&p.hd.sync.store)
        .into_iter()
        .map(|(p, n)| (p.0, n))
        .collect();
    assert_eq!(mine, all);
}

#[test]
fn a_player_speaks_while_frames_arrive_and_stops_300_ms_after() {
    let mut p = Party::new(2, false);
    p.frames(20);
    p.hd.voice.as_mut().unwrap().queue(vec![1, 2]);
    p.guests[0].gd.voice.as_mut().unwrap().queue(vec![3, 4]);
    p.frames(2);
    let t = p.t;
    let g2 = p.guests[1].gd.voice.as_ref().unwrap();
    assert_eq!(g2.speaking.now(t), vec![PlayerId::HOST, PlayerId(1)]);
    let host = p.hd.voice.as_ref().unwrap();
    assert_eq!(host.speaking.now(t), vec![PlayerId(1)], "never itself");
    // No more frames: silent once the hold has passed.
    p.frames((HOLD_MS / 50 + 2) as u32);
    let t = p.t;
    assert!(p.guests[1]
        .gd
        .voice
        .as_ref()
        .unwrap()
        .speaking
        .now(t)
        .is_empty());
    assert!(p.hd.voice.as_ref().unwrap().speaking.now(t).is_empty());
}

#[test]
fn the_speaking_player_s_avatar_is_marked_for_the_compass() {
    let mut p = Party::new(1, true);
    p.host.move_player(HOUSE, [100.0, 200.0, 0.0], 1.0);
    p.guests[0]
        .story
        .move_player(HOUSE, [10.0, 20.0, 30.0], 0.5);
    p.frames(80);
    let avatars = p.guests[0].gd.ladder.as_ref().unwrap().avatar_ids();
    let (_, host_avatar) = *avatars
        .iter()
        .find(|(q, _)| *q == PlayerId::HOST)
        .expect("the host's avatar in the guest's game");
    // The voice link has the avatars too (the head labels' places).
    assert!(p.guests[0]
        .gd
        .voice
        .as_ref()
        .unwrap()
        .avatars
        .contains(&(PlayerId::HOST, host_avatar)));
    let e = &mut p.guests[0].story.e;
    assert!(!coop_engine::voicemark::speaking(e, host_avatar));
    p.hd.voice.as_mut().unwrap().queue(vec![1]);
    p.frames(2);
    assert!(coop_engine::voicemark::speaking(
        &mut p.guests[0].story.e,
        host_avatar
    ));
    p.frames((HOLD_MS / 50 + 2) as u32);
    assert!(!coop_engine::voicemark::speaking(
        &mut p.guests[0].story.e,
        host_avatar
    ));
}

//! voice: voice chat's frames through the session on three games in one process (docs/coop/SPECS.md
//! S36): a host and two guests, each frame heard by every other player and never by its speaker,
//! the places from the live ladder, nothing sent before a guest is in.

use coop_core::ids::PlayerId;
use coop_core::proto::{Hello, PROTOCOL_VERSION};
use coop_core::sync::HostConfig;
use coop_core::transport::{LoopbackEndpoint, LoopbackNet};
use coop_engine::testkit::{Story, StorySpec};
use coop_engine_session::ladder::{Cap, Ladder, DEFAULT_AVATAR_BASE};
use coop_engine_session::voice::VoiceLink;
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
        let guests = (1..=guests)
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
            guests,
            t: 0,
        }
    }

    fn frames(&mut self, n: u32) {
        for _ in 0..n {
            self.t += 50;
            self.hd.step(&mut self.host.e, &mut self.hnet, self.t);
            for g in &mut self.guests {
                g.story.run_quest_script();
                g.gd.step(&mut g.story.e, &mut g.net, self.t);
            }
        }
    }

    fn host_voice(&mut self) -> &mut VoiceLink {
        self.hd.voice.as_mut().unwrap()
    }

    fn guest_voice(&mut self, i: usize) -> &mut VoiceLink {
        self.guests[i].gd.voice.as_mut().unwrap()
    }
}

#[test]
fn every_frame_reaches_every_other_player_in_order_and_never_its_speaker() {
    let mut p = Party::new(2, false);
    p.frames(20);
    for _ in 0..3 {
        p.host_voice().queue(vec![0xa0, 1]);
        p.guest_voice(0).queue(vec![0xb0, 1]);
    }
    p.guest_voice(1).queue(vec![0xc0, 1]);
    p.frames(4);
    let host = p.host_voice().take_heard();
    let g1 = p.guest_voice(0).take_heard();
    let g2 = p.guest_voice(1).take_heard();
    let count = |h: &[(PlayerId, Vec<u8>)], who: u8| h.iter().filter(|(s, _)| s.0 == who).count();
    // The host hears both guests; each guest hears the host and the other guest.
    assert_eq!(
        (count(&host, 0), count(&host, 1), count(&host, 2)),
        (0, 3, 1)
    );
    assert_eq!((count(&g1, 0), count(&g1, 1), count(&g1, 2)), (3, 0, 1));
    assert_eq!((count(&g2, 0), count(&g2, 1), count(&g2, 2)), (3, 3, 0));
    // The payload is the speaker's, untouched.
    assert!(g2
        .iter()
        .all(|(s, b)| (s.0 == 0 && b == &[0xa0, 1]) || (s.0 == 1 && b == &[0xb0, 1])));
    assert_eq!(p.host_voice().sent, 3);
    assert_eq!(p.guest_voice(0).sent, 3);
    // The host's live layer checked and relayed the guests' frames.
    assert_eq!(p.hd.sync.live.voice.forged, 0);
    assert!(p.hd.sync.live.voice.relayed >= 4);
    // Nothing more arrives later: each frame was heard once.
    p.frames(4);
    assert!(p.host_voice().take_heard().is_empty());
    assert!(p.guest_voice(1).take_heard().is_empty());
}

#[test]
fn a_guest_sends_nothing_before_the_host_lets_it_in() {
    let mut p = Party::new(1, false);
    // The first step: the guest has only said hello.
    p.guest_voice(0).queue(vec![1, 2, 3]);
    let g = &mut p.guests[0];
    g.gd.step(&mut g.story.e, &mut g.net, 50);
    assert_eq!(p.guest_voice(0).not_joined, 1);
    assert_eq!(p.guest_voice(0).sent, 0);
    p.frames(20);
    assert!(p.host_voice().take_heard().is_empty());
    // In: the next frame goes.
    p.guest_voice(0).queue(vec![4, 5, 6]);
    p.frames(3);
    assert_eq!(
        p.host_voice().take_heard(),
        vec![(PlayerId(1), vec![4, 5, 6])]
    );
}

#[test]
fn places_come_from_this_game_and_the_ladder_s_avatars() {
    let mut p = Party::new(1, true);
    p.host.move_player(HOUSE, [100.0, 200.0, 0.0], 1.0);
    p.guests[0]
        .story
        .move_player(HOUSE, [10.0, 20.0, 30.0], 0.5);
    p.frames(80);
    let host = p.host_voice();
    let me = host.listener.expect("the host's player has a place");
    assert_eq!(
        (me.pos, me.heading, me.space),
        ([100.0, 200.0, 0.0], 1.0, HOUSE)
    );
    let guest_place = *host.speakers.get(&PlayerId(1)).expect("the guest's avatar");
    assert_eq!(guest_place.space, HOUSE);
    let g = p.guest_voice(0);
    let hp = *g.speakers.get(&PlayerId::HOST).expect("the host's avatar");
    assert_eq!(hp.space, HOUSE);
    // The guest may have been moved to the host by the ladder; its own place is its game's.
    assert!(g.listener.is_some());
    assert!(
        (hp.pos[0] - 100.0).abs() < 1.0 && (hp.pos[1] - 200.0).abs() < 1.0,
        "{hp:?}"
    );
}

#[test]
fn without_the_ladder_nobody_has_a_place_and_voice_still_flows() {
    let mut p = Party::new(1, false);
    p.frames(20);
    assert!(p.host_voice().speakers.is_empty());
    p.host_voice().queue(vec![9]);
    p.frames(2);
    assert_eq!(
        p.guest_voice(0).take_heard(),
        vec![(PlayerId::HOST, vec![9])]
    );
}

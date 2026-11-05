//! The start flow end to end over the loopback transport: the host's opening state reaches the
//! guest, the guest makes its own character with local menus, the host stores it in the guest's
//! shadow (and only there), and the guest arrives.

use coop_core::ids::PlayerId;
use coop_core::scene::SceneKind;
use coop_core::transport::LoopbackNet;
use coop_session::start::{OPENING_OVER_STAGE, VCG00, VCG00_DONE, VCG01};
use coop_session::{Opening, Phase, Session, SessionConfig, Step, Venue};
use esm::{ActivePlugins, FormId, LoadOrder};
use world::coop::apply::{Ctx, Opts};
use world::dialogue::PLAYER_REF;
use world::scripting::{GameState, ScriptCache};

const DT: f32 = 0.025;
const HOST_CELL: u32 = 0xB10;
const ROOM_CELL: u32 = 0xB11;

struct Peer {
    s: Session,
    gs: GameState,
}

struct Rig<'a> {
    order: &'a LoadOrder,
    scripts: &'a ScriptCache,
    host: Peer,
    guest: Peer,
    /// frames the guest's current menu stays up
    menu_left: u32,
    opened: Vec<Step>,
}

fn cfg(name: &str, t: Box<dyn coop_core::transport::Transport>) -> SessionConfig {
    SessionConfig {
        name: name.into(),
        session_id: 0xABCD,
        transport: Some(t),
        live: true,
        ..SessionConfig::default()
    }
}

impl<'a> Rig<'a> {
    fn new(order: &'a LoadOrder, scripts: &'a ScriptCache, new_game: bool) -> Rig<'a> {
        let net = LoopbackNet::new();
        let mut hs = Session::host(cfg("Host", Box::new(net.endpoint(PlayerId::HOST)))).unwrap();
        let mut gsn = Session::join(cfg("Ann", Box::new(net.endpoint(PlayerId(1))))).unwrap();
        hs.set_start_config(coop_session::StartConfig {
            new_game,
            ..Default::default()
        });
        gsn.set_start_config(coop_session::StartConfig::default());
        let mut host = Peer {
            s: hs,
            gs: GameState::new(order),
        };
        let mut guest = Peer {
            s: gsn,
            gs: GameState::new(order),
        };
        host.gs.player_cell = Some(FormId(HOST_CELL));
        guest.gs.player_cell = Some(FormId(HOST_CELL));
        Rig {
            order,
            scripts,
            host,
            guest,
            menu_left: 0,
            opened: Vec::new(),
        }
    }

    fn frame(&mut self) {
        let cx = Ctx {
            order: self.order,
            scripts: self.scripts,
        };
        for p in [&mut self.host, &mut self.guest] {
            p.s.pre_frame_with(
                &mut p.gs,
                Opts {
                    present: true,
                    ctx: Some(cx),
                },
            );
            p.gs.player_position = Some([0.0, 0.0, 0.0]);
            p.s.live_frame(&mut p.gs, cx, DT);
        }
        // the viewer glue of the guest: the local menus
        if self.menu_left > 0 {
            self.menu_left -= 1;
        }
        if let Some(step) = self.guest.s.start_poll(&self.guest.gs, self.menu_left > 0) {
            self.opened.push(step);
            self.menu_left = 8;
            // what the menu leaves in the player's rows when it closes
            let gs = &mut self.guest.gs;
            match step {
                Step::Name => gs.player_name = Some("Ann the Courier".into()),
                Step::Special => {
                    for (i, av) in coop_session::start::SPECIAL.iter().enumerate() {
                        gs.actor_values.insert((PLAYER_REF, *av), (3 + i) as f64);
                    }
                }
                Step::TagSkills => gs.tag_skills = [32u16, 40, 41].into_iter().collect(),
                Step::Traits => {
                    gs.perks.insert(FormId(0x4242));
                }
            }
        }
        // follow_place: move to the host once allowed
        if self.guest.s.start_follow_ok() && self.guest.s.start_phase() != Phase::Joining {
            self.guest.gs.player_cell = Some(FormId(HOST_CELL));
        }
        for p in [&mut self.host, &mut self.guest] {
            p.s.post_frame(&mut p.gs, DT);
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
}

fn order(tag: &str) -> (testdata::TempData, LoadOrder) {
    let data = testdata::dialogue::world(tag);
    let order = LoadOrder::from_data_dir(data.path(), &ActivePlugins::OfficialOnly).unwrap();
    (data, order)
}

#[test]
fn a_guest_joining_the_hosts_opening_makes_its_own_character_in_the_hosts_place() {
    let (_d, order) = order("st-open");
    let scripts = ScriptCache::default();
    let mut r = Rig::new(&order, &scripts, true);
    // the host is in the opening, at VCG01 stage 10
    r.host.gs.stages.insert(VCG00, 0);
    r.host.gs.stages.insert(VCG01, 10);
    assert_eq!(r.host.s.start_phase(), Phase::CharacterCreation);
    let mut left_place = false;
    let ok = r.until(8000, |r| {
        left_place |= r.guest.gs.player_cell != Some(FormId(HOST_CELL));
        r.guest.s.start_phase() == Phase::Playing
    });
    assert!(ok, "guest at {:?}", r.guest.s.start_phase());
    assert_eq!(r.guest.s.start_info().venue, Venue::HostPlace);
    assert_eq!(r.guest.s.host_opening(), Some(Opening::InProgress));
    assert_eq!(
        r.opened,
        coop_session::start::STEPS.to_vec(),
        "the menus ran once each, in order"
    );
    assert!(!left_place, "never left the host's place");
    for _ in 0..200 {
        r.frame();
    }
    // the host holds the guest's character in the guest's shadow
    let sh = r.host.gs.shadow_of(1).expect("guest shadow");
    assert_eq!(sh.name.as_deref(), Some("Ann the Courier"));
    assert_eq!(sh.quest_stage.get(&VCG00), Some(&VCG00_DONE));
    assert_eq!(sh.quest_stage.get(&VCG01), Some(&OPENING_OVER_STAGE));
    assert!(sh.perks.contains(&FormId(0x4242)));
    // and the host's own opening is untouched, still per player
    assert_eq!(r.host.gs.stages.get(&VCG01), Some(&10));
    assert_eq!(r.host.s.start_phase(), Phase::CharacterCreation);
    assert!(r.host.gs.player_name.is_none());
}

#[test]
fn a_guest_joining_after_the_opening_makes_its_character_in_a_local_room_first() {
    let (_d, order) = order("st-late");
    let scripts = ScriptCache::default();
    let mut r = Rig::new(&order, &scripts, false);
    // a loaded game: the opening was long done
    r.host.gs.stages.insert(VCG01, 200);
    // the guest loaded the local room, not the host's place
    r.guest.gs.player_cell = Some(FormId(ROOM_CELL));
    let mut stayed = true;
    let ok = r.until(8000, |r| {
        if r.guest.s.start_phase() == Phase::CharacterCreation
            && r.guest.gs.player_cell != Some(FormId(ROOM_CELL))
        {
            stayed = false;
        }
        r.guest.s.start_phase() == Phase::Playing
    });
    assert!(ok, "guest at {:?}", r.guest.s.start_phase());
    assert!(
        stayed,
        "the guest was kept in the room while making its character"
    );
    assert_eq!(r.guest.s.start_info().venue, Venue::LocalRoom);
    assert_eq!(r.guest.s.host_opening(), Some(Opening::Over));
    assert_eq!(r.host.s.start_phase(), Phase::Playing);
    assert_eq!(r.opened.len(), 4);
    assert_eq!(
        r.guest.gs.player_cell,
        Some(FormId(HOST_CELL)),
        "and then arrived at the host's place"
    );
}

#[test]
fn the_hosts_copy_of_the_guests_rows_does_not_undo_the_menus_while_they_run() {
    // Raising a stat in the Vit-o-Matic was undone (and the points seemed to drain) because each
    // store change made the guest take the host's stale copy of its own shadow back.
    let (_d, order) = order("st-hold");
    let scripts = ScriptCache::default();
    let mut r = Rig::new(&order, &scripts, true);
    r.host.gs.stages.insert(VCG00, 0);
    r.host.gs.stages.insert(VCG01, 10);
    let special = coop_session::start::SPECIAL;
    let mut seen_special = false;
    let mut frames_after = 0;
    for i in 0..600u16 {
        // the host's world keeps changing, so the store keeps changing
        r.host.gs.stages.insert(FormId(0x7777), i % 50 + 1);
        r.frame();
        if r.opened.contains(&Step::Special) {
            seen_special = true;
            let gs = &r.guest.gs;
            for (j, av) in special.iter().enumerate() {
                assert_eq!(
                    gs.actor_values.get(&(PLAYER_REF, *av)),
                    Some(&((3 + j) as f64)),
                    "frame {i}, attribute {av} after the menu set it"
                );
            }
            frames_after += 1;
        }
        if r.guest.s.start_phase() == Phase::Playing {
            break;
        }
    }
    assert!(seen_special && frames_after > 10);
    assert_eq!(r.guest.s.start_phase(), Phase::Playing);
    // after the host stored the sheet the guest's rows are the host's copy, which is the same
    let sh = r.host.gs.shadow_of(1).expect("guest shadow");
    assert_eq!(sh.actor_values.get(&special[0]), Some(&3.0));
    assert_eq!(r.guest.gs.player_name.as_deref(), Some("Ann the Courier"));
    assert!(r.guest.gs.perks.contains(&FormId(0x4242)));
}

#[test]
fn the_guest_watches_the_hosts_intro_before_its_first_menu() {
    let (_d, order) = order("st-watch");
    let scripts = ScriptCache::default();
    let mut r = Rig::new(&order, &scripts, true);
    // the host is still in the intro: VCG00 running, VCG01 not begun
    r.host.gs.stages.insert(VCG00, 0);
    for _ in 0..400 {
        r.frame();
    }
    assert!(r.opened.is_empty(), "no menu before the host's name box");
    assert_eq!(r.guest.s.start_phase(), Phase::CharacterCreation);
    // Doc's first lines
    r.host.gs.stages.insert(VCG01, 3);
    for _ in 0..100 {
        r.frame();
    }
    assert!(r.opened.is_empty());
    // the host's name box
    r.host.gs.stages.insert(VCG01, 10);
    assert!(r.until(400, |r| !r.opened.is_empty()));
    assert_eq!(r.opened[0], Step::Name);
}

#[test]
fn a_control_lock_of_the_hosts_opening_leaves_the_guests_menus_free() {
    let (_d, order) = order("st-lock");
    let scripts = ScriptCache::default();
    let mut r = Rig::new(&order, &scripts, true);
    // Doc's first lines: the menus wait for the host's name box
    r.host.gs.stages.insert(VCG01, 3);
    assert!(r.until(400, |r| r.guest.s.start_phase() == Phase::CharacterCreation));
    // the host's opening disables the controls: a control lock for the whole party
    r.host.gs.controls_off = [true; 7];
    r.host.gs.coop_controls_end(Some(VCG01));
    assert!(
        r.until(400, |r| r
            .guest
            .gs
            .coop
            .live
            .scene
            .view
            .as_ref()
            .is_some_and(|v| v.kind == SceneKind::Control)),
        "the guest is in the host's lock"
    );
    assert!(r.opened.is_empty(), "no menu before the host's name box");
    // the host's name box: the guest's menus open, in the lock
    r.host.gs.stages.insert(VCG01, 10);
    assert!(r.until(400, |r| !r.opened.is_empty()));
    assert_eq!(r.opened[0], Step::Name);
    assert!(
        r.guest.gs.coop.live.scene.view.is_some(),
        "still in the lock"
    );
}

#[test]
fn a_scene_or_movie_playing_on_the_guest_holds_its_menus_back() {
    let (_d, order) = order("st-movie");
    let scripts = ScriptCache::default();
    let mut r = Rig::new(&order, &scripts, true);
    r.host.gs.stages.insert(VCG01, 10);
    // the host's intro movie is on the guest's screen
    r.guest.gs.coop.live.scene.movie_busy = true;
    for _ in 0..300 {
        r.frame();
    }
    assert!(r.opened.is_empty(), "the guest sat through the movie");
    r.guest.gs.coop.live.scene.movie_busy = false;
    assert!(r.until(400, |r| !r.opened.is_empty()));
}

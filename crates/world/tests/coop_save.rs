//! Party saves with a host and two guests over coop-core's simulated transport (testdata
//! quests, no game data): save, drop everything, load, rejoin. Saves go to temp folders only.

use std::path::PathBuf;
use std::sync::Arc;

use coop_core::ids::PlayerId;
use coop_core::sim_test::Sim;
use coop_core::sync::GuestPhase;
use coop_core::transport::ChaosProfile;
use coop_policy::PolicyTable;
use esm::{ActivePlugins, FormId, LoadOrder};
use testdata::quest_ids::*;
use world::coop::apply::{Ctx, Opts};
use world::coop::export::{domain_of, export};
use world::coop::facts::puppet_of;
use world::coop::party_char::{
    baseline_diff, baseline_of, carry_coop, install_for_slot, sanitize_offer, CharacterFile,
};
use world::coop::party_save::{
    is_synthetic, load_party, party_dir_for, write_party, write_party_with, GuestSidecar,
    PartySave, Problem, Step,
};
use world::coop::shadow::{shadow_form, PlayerShadow};
use world::coop::{Authority, Domain, GuestBridge, HostBridge, ShadowPlan, WorldBackend};
use world::dialogue::PLAYER_REF;
use world::scripting::{GameState, Runner, ScriptCache};

const KEY_A: u64 = 0xA11CE;
const KEY_B: u64 = 0xB0B;
const CAMPAIGN: u64 = 0x00ab_12cd_34ef_5678;

fn table() -> Arc<PolicyTable> {
    let json = format!(
        r#"{{"schema":2,
"policy":{{"death":{{"mode":"downed","revive_s":60,"all_down":"reload-host-save"}},"late_join":{{"during_scene":"wait","after_scene":"warp-to-zone"}},"barrier_timeout_s":120,"tether_timeout_s":120,"follower_cap":{{"humanoid":1,"critter":1}},"party_cap":4}},
"shadow_classes":{{"skills":"per_player","perks":"per_player","followers":"per_player","karma":"shared","reputation":"shared"}},
"quests":[{{"edid":"TQ","id":"0x{QUEST:08X}","local":"0x{:08X}","plugin":"t.esm","tier":"T1","line":"t","status":"confirmed","advance":"host","scope":"per_player","save_suppress":true}}]}}"#,
        QUEST & 0x00FF_FFFF
    );
    Arc::new(PolicyTable::from_json(&json).expect("table"))
}

fn tmp(tag: &str) -> PathBuf {
    let d = std::env::temp_dir().join(format!("nvrs-coop-save-{tag}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&d);
    std::fs::create_dir_all(&d).unwrap();
    d
}

fn order(tag: &str) -> (testdata::TempData, LoadOrder) {
    let data = testdata::quests(tag);
    let order = LoadOrder::from_data_dir(data.path(), &ActivePlugins::OfficialOnly).unwrap();
    (data, order)
}

struct Rig<'a> {
    order: &'a LoadOrder,
    scripts: &'a ScriptCache,
    sim: Sim,
    host: GameState,
    guests: Vec<GameState>,
    hb: HostBridge,
    gbs: Vec<GuestBridge>,
}

impl<'a> Rig<'a> {
    /// A fresh session: host plus two guests in slots 1 and 2.
    fn new(order: &'a LoadOrder, scripts: &'a ScriptCache, chaos: &str) -> Rig<'a> {
        let t = table();
        let mut host = GameState::new(order);
        host.set_authority(Authority::Host);
        host.enable_coop_policy(t.clone(), 0);
        for s in 1..=2u8 {
            host.register_player(s);
            host.with_player(s, |g| g.stock(order, PLAYER_REF));
        }
        host.coop
            .gate
            .as_mut()
            .unwrap()
            .set_present((0..=2).map(PlayerId));
        let guests = (1..=2u8)
            .map(|s| {
                let mut g = GameState::new(order);
                g.set_authority(Authority::Guest);
                g.enable_shadows(ShadowPlan::from_policy(&t), s);
                g
            })
            .collect();
        let mut hb = HostBridge::default();
        hb.backstop_every = 1;
        let mut rig = Rig {
            order,
            scripts,
            sim: Sim::new(2, ChaosProfile::by_name(chaos).unwrap(), 11),
            host,
            guests,
            hb,
            gbs: (0..2).map(|_| GuestBridge::default()).collect(),
        };
        for i in 0..2 {
            rig.sim.start_guest(i);
        }
        rig.ticks(20);
        rig
    }

    fn tick(&mut self) {
        self.hb.collect(&mut self.host, &mut self.sim.host);
        self.sim.step();
        for i in 0..self.guests.len() {
            if self.sim.guests[i].sync.phase() == GuestPhase::Active {
                let opts = Opts {
                    present: true,
                    ctx: Some(Ctx {
                        order: self.order,
                        scripts: self.scripts,
                    }),
                };
                let store = &self.sim.guests[i].sync.store;
                self.gbs[i].sync(&mut self.guests[i], store, false, opts);
            }
        }
    }

    fn ticks(&mut self, n: u32) {
        for _ in 0..n {
            self.tick();
        }
    }

    fn settle(&mut self) -> bool {
        for _ in 0..6000 {
            self.tick();
            let h = self.host.shard_hashes();
            let ok = (0..self.guests.len()).all(|i| {
                self.sim.guests[i].sync.phase() == GuestPhase::Active
                    && self.guests[i].shard_hashes() == h
                    && self.sim.guests[i].sync.store.rev() == self.sim.host.store.rev()
            });
            if ok {
                return true;
            }
        }
        false
    }
}

/// Hash of the shared part of a state: every exported record except the per-player shadows.
fn shared_hash(gs: &GameState) -> u64 {
    let mut h = 0u64;
    for (k, f) in export(gs) {
        if domain_of(&k) != Domain::Shadow {
            h ^= coop_core::hash::record_hash(k.form, &f).rotate_left(k.form.0 % 61);
        }
    }
    h
}

/// The shared records (shadows left out), for readable diffs.
fn shared_fields(gs: &GameState) -> Vec<String> {
    export(gs)
        .into_iter()
        .filter(|(k, _)| domain_of(k) != Domain::Shadow)
        .map(|(k, f)| format!("{k:?} = {f:?}"))
        .collect()
}

/// Makes the three characters different and the world interesting.
fn play(rig: &mut Rig, order: &LoadOrder, scripts: &ScriptCache) {
    rig.host.with_player(1, |g| {
        *g.items.entry((PLAYER_REF, FormId(CAPS))).or_insert(0) += 70;
        g.skill_points.insert(32, 5);
        g.player_level = 4;
        g.stages.insert(FormId(QUEST), 30);
    });
    rig.host.with_player(2, |g| {
        *g.items.entry((PLAYER_REF, FormId(CAPS))).or_insert(0) += 5;
        g.skill_points.insert(33, 9);
        g.player_level = 8;
        g.stages.insert(FormId(QUEST), 10);
    });
    *rig.host
        .items
        .entry((PLAYER_REF, FormId(CAPS)))
        .or_insert(0) += 1000;
    rig.host.player_level = 2;
    rig.host.topics.insert(FormId(TOPIC_SECRET));
    let mut r = Runner::new(order, scripts, &mut rig.host);
    r.update(1.0);
}

fn capture(rig: &Rig, keys: [u64; 2], seq: u32) -> PartySave {
    let guests = [(1u8, keys[0], "Ann"), (2u8, keys[1], "Bo")]
        .iter()
        .map(|(slot, key, name)| {
            GuestSidecar::from_shadow(*key, name, *slot, true, &rig.host.shadow_of(*slot).unwrap())
        })
        .collect();
    PartySave {
        campaign: CAMPAIGN,
        host_name: "Host".into(),
        seq,
        saved_unix: 1_700_000_000,
        world_text: world::save::save(&rig.host, None),
        guests,
        rules: Default::default(),
    }
}

/// What a joining guest with `key` gets from a loaded party save, installed on the host.
fn rejoin(
    host: &mut GameState,
    loaded: &world::coop::party_save::LoadedParty,
    slot: u8,
    key: u64,
) -> bool {
    let Some(sc) = loaded.guest(key) else {
        return false;
    };
    let plan = host.coop.bank.plan.clone().unwrap();
    let (sh, _) = sanitize_offer(sc.shadow().unwrap(), &plan, true);
    install_for_slot(host, slot, sh)
}

#[test]
fn save_drop_everything_load_rejoin_in_swapped_slots() {
    for chaos in ["perfect", "bad_wifi"] {
        let (_d, order) = order("save-round");
        let scripts = ScriptCache::default();
        let mut rig = Rig::new(&order, &scripts, chaos);
        play(&mut rig, &order, &scripts);
        assert!(rig.settle(), "first session did not converge ({chaos})");
        let hash = shared_hash(&rig.host);
        let before = shared_fields(&rig.host);
        let (ann, bo) = (
            rig.host.shadow_of(1).unwrap(),
            rig.host.shadow_of(2).unwrap(),
        );
        let host_me = PlayerShadow::read(&rig.host, rig.host.coop.bank.plan.as_ref().unwrap());
        assert_ne!(ann, bo);
        let dir = tmp("round").join("Save 1.txt.partysave");
        write_party(&dir, &capture(&rig, [KEY_A, KEY_B], 1)).unwrap();
        drop(rig);

        // everything is gone; a new host loads the party save and the guests rejoin, B first
        let loaded = load_party(&dir).unwrap();
        assert!(loaded.problems.is_empty(), "{:?}", loaded.problems);
        let mut rig = Rig::new(&order, &scripts, chaos);
        let (mut state, _) = world::save::load(&loaded.save.world_text).unwrap();
        carry_coop(&mut rig.host, &mut state, Some(&order));
        rig.host = state;
        assert!(rejoin(&mut rig.host, &loaded, 1, KEY_B));
        assert!(rejoin(&mut rig.host, &loaded, 2, KEY_A));
        assert!(rig.settle(), "second session did not converge ({chaos})");

        let after = shared_fields(&rig.host);
        let only_before: Vec<_> = before.iter().filter(|x| !after.contains(x)).collect();
        let only_after: Vec<_> = after.iter().filter(|x| !before.contains(x)).collect();
        assert!(
            only_before.is_empty() && only_after.is_empty(),
            "lost {only_before:?} gained {only_after:?}"
        );
        assert_eq!(
            shared_hash(&rig.host),
            hash,
            "shared state identical after the load ({chaos})"
        );
        assert_eq!(rig.host.shadow_of(1).unwrap(), bo, "slot 1 is B now");
        assert_eq!(rig.host.shadow_of(2).unwrap(), ann, "slot 2 is A now");
        assert_eq!(
            PlayerShadow::read(&rig.host, rig.host.coop.bank.plan.as_ref().unwrap()),
            host_me
        );
        // the guests hold their own rows live
        assert_eq!(rig.guests[0].shadow_of(1).unwrap(), bo);
        assert_eq!(rig.guests[1].shadow_of(2).unwrap(), ann);
        assert_eq!(
            rig.guests[1]
                .items
                .get(&(PLAYER_REF, FormId(CAPS)))
                .copied(),
            ann.items.get(&FormId(CAPS)).copied(),
            "per-player caps"
        );
        assert!(ann.items.get(&FormId(CAPS)).copied().unwrap_or(0) >= 70);
        assert!(
            !rig.host.stages.contains_key(&FormId(QUEST)),
            "the per-player quest is not shared"
        );
        assert_eq!(ann.quest_stage.get(&FormId(QUEST)), Some(&30));
        assert_eq!(bo.quest_stage.get(&FormId(QUEST)), Some(&10));
        assert!(rig.host.topics.contains(&FormId(TOPIC_SECRET)));
        assert_eq!(rig.guests[0].shard_hashes(), rig.host.shard_hashes());
    }
}

#[test]
fn an_unmatched_joiner_gets_a_fresh_character_and_nobody_else_changes() {
    let (_d, order) = order("save-fresh");
    let scripts = ScriptCache::default();
    let mut rig = Rig::new(&order, &scripts, "perfect");
    play(&mut rig, &order, &scripts);
    assert!(rig.settle());
    let ann = rig.host.shadow_of(1).unwrap();
    let dir = tmp("fresh").join("a.txt.partysave");
    let mut save = capture(&rig, [KEY_A, KEY_B], 1);
    save.guests.retain(|g| g.key == KEY_A);
    write_party(&dir, &save).unwrap();
    drop(rig);
    let loaded = load_party(&dir).unwrap();
    let mut rig = Rig::new(&order, &scripts, "perfect");
    let (mut state, _) = world::save::load(&loaded.save.world_text).unwrap();
    carry_coop(&mut rig.host, &mut state, Some(&order));
    rig.host = state;
    assert!(rejoin(&mut rig.host, &loaded, 2, KEY_A));
    assert!(
        !rejoin(&mut rig.host, &loaded, 1, KEY_B),
        "no sidecar for B"
    );
    assert!(rig.settle());
    assert_eq!(rig.host.shadow_of(2).unwrap(), ann);
    let fresh = rig.host.shadow_of(1).unwrap();
    assert_eq!(fresh.level, 1);
    assert!(fresh.quest_stage.is_empty());
}

#[test]
fn a_late_joiner_brings_its_own_character_under_the_hosts_shared_state() {
    let (_d, order) = order("save-late");
    let scripts = ScriptCache::default();
    let mut rig = Rig::new(&order, &scripts, "perfect");
    play(&mut rig, &order, &scripts);
    rig.host.stages.insert(FormId(0xDEAD), 50);
    assert!(rig.settle());
    // a character from another game: shared quest 0xDEAD at 20, a personal quest at 30, a puppet item
    let mut mine = PlayerShadow::fresh();
    mine.level = 12;
    mine.items.insert(FormId(CAPS), 999);
    mine.items.insert(puppet_of(PlayerId(3)), 1);
    mine.perks.insert(FormId(0x77));
    mine.quest_stage.insert(FormId(QUEST), 30);
    mine.quest_stage.insert(FormId(0xDEAD), 20);
    mine.cell = Some(FormId(0x1234));
    let file = CharacterFile::new(KEY_B, "Bo", 0, 0, 1, &mine, vec![(0xDEAD, 20), (0xBEEF, 7)]);
    let file = CharacterFile::from_bytes(&file.to_bytes()).unwrap();
    let plan = rig.host.coop.bank.plan.clone().unwrap();
    let (sh, rep) = sanitize_offer(file.shadow().unwrap(), &plan, false);
    assert_eq!(rep.synthetic, 1);
    assert_eq!(rep.shared_quest_rows, 1);
    let conflicts = baseline_diff(&file.baseline, &rig.host);
    assert_eq!(conflicts.len(), 2, "{conflicts:?}");
    assert_eq!((conflicts[0].theirs, conflicts[0].host), (7, 0));
    assert_eq!((conflicts[1].theirs, conflicts[1].host), (20, 50));
    assert!(install_for_slot(&mut rig.host, 2, sh));
    assert!(rig.settle());
    let got = rig.guests[1].shadow_of(2).unwrap();
    assert_eq!(got.level, 12);
    assert_eq!(got.items.get(&FormId(CAPS)), Some(&999));
    assert!(got.perks.contains(&FormId(0x77)));
    assert_eq!(
        got.quest_stage.get(&FormId(QUEST)),
        Some(&30),
        "personal quest progress is theirs"
    );
    assert!(
        !got.quest_stage.contains_key(&FormId(0xDEAD)),
        "the shared quest is not theirs"
    );
    assert_eq!(
        rig.guests[1].stages.get(&FormId(0xDEAD)),
        Some(&50),
        "the host's shared stage wins"
    );
    assert!(got.cell.is_none());
    assert!(!got.items.keys().any(|f| is_synthetic(*f)));
    assert_eq!(
        baseline_of(&rig.host).iter().find(|(q, _)| *q == 0xDEAD),
        Some(&(0xDEAD, 50))
    );
}

#[test]
fn puppets_and_shadow_forms_never_reach_a_save() {
    let (_d, order) = order("save-leak");
    let scripts = ScriptCache::default();
    let mut rig = Rig::new(&order, &scripts, "perfect");
    play(&mut rig, &order, &scripts);
    assert!(rig.settle());
    // the live layer's synthetic refs end up in state rows: put them everywhere a save looks
    let p = puppet_of(PlayerId(1));
    rig.host.items.insert((p, FormId(CAPS)), 3);
    rig.host.items.insert((PLAYER_REF, p), 1);
    rig.host.disabled.insert(p, true);
    rig.host.destroyed.insert(shadow_form(1));
    rig.host.positions.insert(p, ([1.0, 2.0, 3.0], 0.0));
    rig.host.topics.insert(shadow_form(2));
    let text = world::save::save(&rig.host, None);
    assert!(
        text.lines().any(|l| l
            .split_whitespace()
            .any(|w| w.len() == 8 && w.starts_with("FE0000"))),
        "precondition: the raw text has them"
    );
    let mut save = capture(&rig, [KEY_A, KEY_B], 1);
    save.world_text = text;
    let mut sh = rig.host.shadow_of(1).unwrap();
    sh.items.insert(p, 2);
    sh.teammates.insert(p);
    save.guests[0] = GuestSidecar::from_shadow(KEY_A, "Ann", 1, true, &sh);
    let dir = tmp("leak").join("l.txt.partysave");
    write_party(&dir, &save).unwrap();
    // check the bytes on disk, not only the loaded value
    let raw = std::fs::read_to_string(dir.join("world.txt")).unwrap();
    for l in raw.lines() {
        for w in l.split_whitespace() {
            if w.len() == 8 && w.bytes().all(|b| b.is_ascii_hexdigit()) {
                assert!(
                    !is_synthetic(FormId(u32::from_str_radix(w, 16).unwrap())),
                    "synthetic id on disk: {l}"
                );
            }
        }
    }
    let loaded = load_party(&dir).unwrap();
    let ann = loaded.guest(KEY_A).unwrap().shadow().unwrap();
    assert!(!ann.items.contains_key(&p) && ann.teammates.is_empty());
    // loading the text back gives a state with none of them
    let (state, _) = world::save::load(&loaded.save.world_text).unwrap();
    assert!(!state
        .items
        .keys()
        .any(|(a, b)| is_synthetic(*a) || is_synthetic(*b)));
    assert!(!state.disabled.contains_key(&p));
}

#[test]
fn damaged_files_cost_only_their_guest_and_failed_writes_keep_the_old_save() {
    let (_d, order) = order("save-damage");
    let scripts = ScriptCache::default();
    let mut rig = Rig::new(&order, &scripts, "perfect");
    play(&mut rig, &order, &scripts);
    assert!(rig.settle());
    let dir = tmp("damage").join("d.txt.partysave");
    write_party(&dir, &capture(&rig, [KEY_A, KEY_B], 1)).unwrap();
    let old = load_party(&dir).unwrap();
    // a write that fails half way
    let mut next = capture(&rig, [KEY_A, KEY_B], 2);
    next.world_text.push_str("level 99\n");
    let r = write_party_with(&dir, &next, &mut |s| {
        if s == Step::File(2) {
            Err(std::io::Error::other("disk full"))
        } else {
            Ok(())
        }
    });
    assert!(r.is_err());
    assert_eq!(load_party(&dir).unwrap(), old);
    // a damaged sidecar: B gets a fresh character, A is intact
    let f = dir.join(world::coop::party_save::sidecar_name(KEY_B));
    let mut b = std::fs::read(&f).unwrap();
    let n = b.len() / 2;
    b[n] ^= 0xFF;
    std::fs::write(&f, b).unwrap();
    let l = load_party(&dir).unwrap();
    assert_eq!(l.problems, vec![Problem::SidecarCorrupt(KEY_B)]);
    assert!(l.guest(KEY_A).is_some() && l.guest(KEY_B).is_none());
}

#[test]
fn a_scene_holds_the_host_save() {
    let (_d, order) = order("save-lock");
    let scripts = ScriptCache::default();
    let mut rig = Rig::new(&order, &scripts, "perfect");
    let dir = tmp("lock");
    let path = dir.join("q.txt");
    rig.host
        .coop
        .gate
        .as_mut()
        .unwrap()
        .scene(FormId(QUEST), true);
    assert!(rig.host.coop_save_blocked());
    let err = world::save::save_to_path(&path, &rig.host, None).unwrap_err();
    assert_eq!(err.kind(), std::io::ErrorKind::PermissionDenied);
    assert!(!path.exists());
    rig.host
        .coop
        .gate
        .as_mut()
        .unwrap()
        .scene(FormId(QUEST), false);
    world::save::save_to_path(&path, &rig.host, None).unwrap();
    assert!(path.exists());
    assert_eq!(party_dir_for(&path), dir.join("q.txt.partysave"));
}

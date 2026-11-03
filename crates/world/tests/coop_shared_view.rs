//! Shared export and hash cover exactly the same rows on every peer, with
//! shadows on and the bundled policy table: per-player quests (e.g. VCG01)
//! never enter the shared export, each player's own rows round-trip through
//! their shadow blob.

use std::sync::Arc;

use coop_core::ids::PlayerId;
use coop_core::sim_test::Sim;
use coop_core::sync::GuestPhase;
use coop_core::transport::ChaosProfile;
use coop_policy::PolicyTable;
use esm::{ActivePlugins, FormId, LoadOrder};
use world::coop::apply::{Ctx, Opts};
use world::coop::{Authority, GuestBridge, HostBridge, ShadowPlan, WorldBackend};
use world::dialogue::PLAYER_REF;
use world::scripting::{GameState, ScriptCache};

const VCG01: u32 = 0x0010_4C1C; // per player in the bundled table
const VMS30: u32 = 0x0012_14AA; // per player
const VCG03: u32 = 0x0015_D912; // shared
const VMQ01: u32 = 0x0008_42DD; // shared

#[test]
fn shared_export_and_hash_exclude_per_player_quests_on_every_peer() {
    let t = Arc::new(PolicyTable::bundled().expect("bundled table"));
    let plan = ShadowPlan::from_policy(&t);
    assert!(plan.quests.contains(&FormId(VCG01)) && plan.quests.contains(&FormId(VMS30)));
    assert!(!plan.quests.contains(&FormId(VCG03)) && !plan.quests.contains(&FormId(VMQ01)));

    let data = testdata::quests("coop-shared-view");
    let order = LoadOrder::from_data_dir(data.path(), &ActivePlugins::OfficialOnly).unwrap();
    let scripts = ScriptCache::default();

    let mut host = GameState::new(&order);
    host.set_authority(Authority::Host);
    host.enable_coop_policy(t.clone(), 0);
    host.register_player(1);
    host.with_player(1, |g| g.stock(&order, PLAYER_REF));
    host.coop
        .gate
        .as_mut()
        .unwrap()
        .set_present((0..=1).map(PlayerId));
    let mut guest = GameState::new(&order);
    guest.set_authority(Authority::Guest);
    guest.enable_shadows(plan.clone(), 1);

    // The host player's live rows: running quests at stage 0 as in the real
    // game, both per-player and shared ones.
    for q in [VCG01, VMS30, VCG03, VMQ01] {
        host.stages.insert(FormId(q), 0);
        host.running.insert(FormId(q));
    }
    host.stages.insert(FormId(VCG03), 10);
    // The guest's own per-player quest.
    host.with_player(1, |g| {
        g.stages.insert(FormId(VMS30), 5);
        g.running.insert(FormId(VMS30));
    });

    // Each avatar stands somewhere else and moves on its own: positions are
    // the live layer's business and must not enter the shared hash.
    host.player_position = Some([10.0, 20.0, 30.0]);
    host.player_heading = 1.5;
    guest.player_position = Some([-4.0, 8.5, 0.25]);
    guest.player_heading = -0.5;

    let mut sim = Sim::new(1, ChaosProfile::by_name("perfect").unwrap(), 3);
    let mut hb = HostBridge::default();
    hb.backstop_every = 1;
    let mut gb = GuestBridge::default();
    sim.start_guest(0);
    let mut ok = false;
    for _ in 0..4000 {
        hb.collect(&mut host, &mut sim.host);
        sim.step();
        if sim.guests[0].sync.phase() == GuestPhase::Active {
            let opts = Opts {
                present: true,
                ctx: Some(Ctx {
                    order: &order,
                    scripts: &scripts,
                }),
            };
            gb.sync(&mut guest, &sim.guests[0].sync.store, false, opts);
        }
        if sim.guests[0].sync.phase() == GuestPhase::Active
            && guest.shard_hashes() == host.shard_hashes()
            && sim.guests[0].sync.store.rev() == sim.host.store.rev()
        {
            ok = true;
            break;
        }
    }
    assert!(ok, "host and guest hashes never agreed");
    assert_eq!(host.shard_hashes().root(), guest.shard_hashes().root());
    host.player_position = Some([11.0, 21.0, 31.0]);
    assert_eq!(
        host.shard_hashes(),
        guest.shard_hashes(),
        "moving does not change the hash"
    );
    assert_eq!(
        guest.player_position,
        Some([-4.0, 8.5, 0.25]),
        "guest keeps its own position"
    );

    // No per-player quest row in either shared export.
    for gs in [&host, &guest] {
        for (form, _) in gs.export_fields() {
            let w = FormId(form.0);
            assert!(
                !plan.quests.contains(&w),
                "per-player quest {w:?} in the shared export"
            );
        }
    }
    // Shared quests reached the guest.
    assert_eq!(guest.stages.get(&FormId(VCG03)), Some(&10));
    assert!(guest.running.contains(&FormId(VMQ01)));
    // Each player's own rows live where they belong.
    assert_eq!(guest.stages.get(&FormId(VMS30)), Some(&5));
    assert!(
        !guest.stages.contains_key(&FormId(VCG01)),
        "host player's quest stays out of the guest's live rows"
    );
    let hs = guest.shadow_of(0).expect("host shadow mirrored");
    assert!(
        hs.quest_flags.contains_key(&FormId(VCG01)),
        "host rows round-trip through the shadow blob"
    );
    assert_eq!(hs.quest_stage.get(&FormId(VCG01)), Some(&0));
    let gs1 = host.shadow_of(1).expect("guest shadow on host");
    assert_eq!(gs1.quest_stage.get(&FormId(VMS30)), Some(&5));
}

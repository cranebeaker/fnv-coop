//! The rows of a player in the model of `coop_core::rows`, against the encoder of the
//! `world` layer (`world::coop::shadow`).
//!
//! The two codecs must give the same bytes for the same rows, or a game on another engine
//! and a game of this layer get different hashes for one state. Each test has a mutation
//! twin inside it.

use std::collections::BTreeMap;

use coop_core::rows::{self, PlayerRows};
use coop_core::schema::{Field, Key};
use esm::FormId;
use world::coop::shadow::{self, PlayerShadow};
use world::coop::{Authority, ShadowPlan, WorldBackend};
use world::scripting::GameState;

const VCG01: FormId = FormId(0x0010_4C1C);

/// A shadow with each field set. Its blob needs more than one chunk.
fn full() -> PlayerShadow {
    let mut s = PlayerShadow::fresh();
    s.actor_values = [(5u16, 7.0), (32, 45.5)].into_iter().collect();
    s.value_damage = [(16u16, 12.25)].into_iter().collect();
    s.damage = Some(3.5);
    s.dead = true;
    s.items = (0..90u32)
        .map(|i| (FormId(0x1000 + i), i as i32 - 3))
        .collect();
    s.stocked = true;
    s.equipped = Some(vec![FormId(0x2002), FormId(0x2001)]);
    s.weapon_health = [(FormId(0x2001), 0.75f32.to_bits())].into_iter().collect();
    s.weapon_mods = [(FormId(0x2001), 5u8)].into_iter().collect();
    s.dropped = [FormId(0x3001)].into_iter().collect();
    s.equip_locked = [FormId(0x3002)].into_iter().collect();
    s.ammo_loaded = Some(FormId(0x4001));
    s.hotkeys[2] = Some(FormId(0x2001));
    s.misc_stats = [(3u8, 9u32)].into_iter().collect();
    s.skill_points = [(32u16, 14u32)].into_iter().collect();
    s.tag_skills = [32u16, 40, 41].into_iter().collect();
    s.tag_slots = [(0u8, 32u16)].into_iter().collect();
    s.level = 7;
    s.level_up_pending = true;
    s.perks = [FormId(0x5001), FormId(0x5002)].into_iter().collect();
    s.perk_ranks = [(FormId(0x5002), 2u8)].into_iter().collect();
    s.teammates = [FormId(0x6001)].into_iter().collect();
    s.cell = Some(FormId(0x0010_4C0F));
    s.world = Some(FormId(0x000D_A726));
    s.position = Some([1.0f32.to_bits(), 2.0f32.to_bits(), 3.0f32.to_bits()]);
    s.heading = 0.5f32.to_bits();
    s.name = Some("Bo".into());
    s.female = Some(true);
    s.quest_stage = [(VCG01, 105u16)].into_iter().collect();
    s.stages_done = [(VCG01, 10u16), (VCG01, 105)].into_iter().collect();
    s.quest_flags = [(VCG01, 3u8)].into_iter().collect();
    s.objectives = [((VCG01, 10), 1u8), ((VCG01, -1), 2)].into_iter().collect();
    s.quest_vars = [((VCG01, "bruntimer".to_string()), 1.0f64.to_bits())]
        .into_iter()
        .collect();
    s
}

/// Shadows that differ in one field each, from the new shadow and from the full one.
fn samples() -> Vec<PlayerShadow> {
    let mut list = vec![PlayerShadow::fresh(), full()];
    let edits: [fn(&mut PlayerShadow); 8] = [
        |s| s.female = Some(false),
        |s| s.equipped = Some(Vec::new()),
        |s| s.name = Some(String::new()),
        |s| s.damage = Some(0.0),
        |s| s.position = Some([0; 3]),
        |s| s.hotkeys = [Some(FormId(1)); 8],
        |s| s.level = u16::MAX,
        |s| {
            s.items.insert(FormId(0xFFFF_FFFF), i32::MIN);
        },
    ];
    for edit in edits {
        for mut base in [PlayerShadow::fresh(), full()] {
            edit(&mut base);
            list.push(base);
        }
    }
    list
}

#[test]
fn the_two_codecs_give_the_same_bytes() {
    for s in samples() {
        let world_bytes = shadow::encode(&s);
        let core_rows = PlayerRows::from(&s);
        assert_eq!(rows::encode(&core_rows), world_bytes);
        // each codec reads the bytes of the other one, and gives the same rows
        let from_world = shadow::decode(&world_bytes).expect("the world codec reads its bytes");
        let from_core = rows::decode(&world_bytes).expect("the core codec reads the world bytes");
        assert_eq!(from_core, core_rows);
        assert_eq!(PlayerShadow::from_rows(&from_core), from_world);
        assert_eq!(PlayerRows::from(&from_world), from_core);
    }
    // the twin: rows that differ in one value give other bytes than the shadow's
    let s = full();
    let mut other = PlayerRows::from(&s);
    other
        .quest_stage
        .insert(coop_core::ids::FormId(VCG01.0), 106);
    assert_ne!(rows::encode(&other), shadow::encode(&s));
    // a blob that one codec refuses, the other one refuses too
    let mut cut = shadow::encode(&s);
    cut.pop();
    assert_eq!(shadow::decode(&cut), None);
    assert_eq!(rows::decode(&cut), None);
}

#[test]
fn the_two_layers_cut_a_blob_into_the_same_chunks() {
    assert_eq!(rows::TAG_BASE, shadow::TAG_BASE);
    assert_eq!(rows::TAG_END, shadow::TAG_END);
    assert_eq!(rows::CHUNK, shadow::CHUNK);
    assert_eq!(rows::ROWS_FORM_BASE, shadow::SHADOW_FORM_BASE);
    for slot in 0..8u8 {
        assert_eq!(rows::rows_form(slot).0, shadow::shadow_form(slot).0);
        let form = shadow::shadow_form(slot);
        assert_eq!(shadow::slot_of_shadow_form(form), Some(slot));
        assert_eq!(rows::slot_of_rows_form(rows::rows_form(slot)), Some(slot));
    }
    // the first form that is no rows form, for each layer
    assert_eq!(shadow::slot_of_shadow_form(shadow::shadow_form(8)), None);
    assert_eq!(rows::slot_of_rows_form(rows::rows_form(8)), None);
}

/// A game's export of the rows of a player, as fields by key.
fn exported(gs: &GameState) -> BTreeMap<Key, Field> {
    gs.export_fields()
        .into_iter()
        .map(|(form, field)| (field.key(form), field))
        .collect()
}

#[test]
fn the_core_codec_reads_the_rows_that_a_game_exports() {
    let mut gs = GameState::default();
    gs.set_authority(Authority::Host);
    gs.enable_shadows(ShadowPlan::all(), 0);
    // the live player (slot 0) and a parked player (slot 1)
    gs.player_name = Some("Al".into());
    gs.tag_skills = [32u16, 41].into_iter().collect();
    gs.register_player(1);
    gs.with_player(1, |g| {
        g.player_name = Some("Bo".into());
        g.player_level = 4;
        g.perks.insert(FormId(0x5001));
    });
    let fields = exported(&gs);
    for (slot, name) in [(0u8, "Al"), (1, "Bo")] {
        let got = rows::rows_of(slot, |k| fields.get(k)).expect("rows in the export");
        assert_eq!(got.name.as_deref(), Some(name));
        // the export is the shadow with no place and no health lost
        let mut want = gs.shadow_of(slot).expect("a shadow");
        want.cell = None;
        want.world = None;
        want.position = None;
        want.heading = 0;
        want.damage = None;
        assert_eq!(got, PlayerRows::from(&want));
        // the chunks of the core layer are the fields of the export
        let chunks = rows::chunks_of(slot, &rows::encode(&got));
        assert!(!chunks.is_empty());
        for (key, field) in &chunks {
            assert_eq!(fields.get(key), Some(field));
        }
    }
    let bo = rows::rows_of(1, |k| fields.get(k)).expect("rows of player 1");
    assert_eq!(bo.level, 4);
    assert!(bo.perks.contains(&coop_core::ids::FormId(0x5001)));
    // the twin: a player with no rows in the game has no rows in the export
    assert_eq!(rows::rows_of(2, |k| fields.get(k)), None);
    assert!(rows::blob_of(2, |k| fields.get(k)).is_empty());
}

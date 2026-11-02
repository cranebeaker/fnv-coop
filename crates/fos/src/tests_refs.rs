//! Tests of the reference, actor, process and base form decoders on
//! synthetic change forms (never a real save).

use crate::decode::{self, extra_kind as k, Coverage, Extra, InitialData, Value};
use crate::write::PipeWriter;
use crate::{save_type as t, ChangeForm, GlobalData, RefId, MINOR_VERSION};

fn cf(save_type: u8, ref_id: u32, flags: u32, data: &[u8]) -> ChangeForm<'_> {
    ChangeForm {
        ref_id: RefId(ref_id),
        flags,
        save_type,
        version: MINOR_VERSION,
        offset: 0,
        data_offset: 0,
        data,
    }
}

/// A moved reference's initial data (27 bytes) as one value.
fn place(w: &mut PipeWriter, space: u32, at: [f32; 3]) {
    let mut b = vec![(space >> 16) as u8, (space >> 8) as u8, space as u8];
    for v in at.iter().chain(&[0.0, 0.0, 1.5]) {
        b.extend_from_slice(&v.to_le_bytes());
    }
    w.bytes(&b);
}

#[test]
fn a_reference_decodes_its_extra_data_and_inventory() {
    // Flags: moved, scale, inventory, ownership (extra), lock (extra),
    // game-only extra.
    let flags = 0x2 | 0x10 | 0x20 | 0x40 | 0x1000 | 0x8000_0000;
    let mut w = PipeWriter::new();
    place(&mut w, 7, [1.0, 2.0, 3.0]);
    w.f32(1.25);
    w.vsval(4);
    w.u8(k::OWNERSHIP).ref_id(5);
    w.u8(k::LOCK).u8(50).u8(1).ref_id(6).u32(2).u32(0);
    w.u8(k::MAP_MARKER).u8(3);
    // A script with a number and a reference variable.
    w.u8(k::SCRIPT).ref_id(8).vsval(2);
    w.u32(1).f64(4.0);
    w.u32(0x8000_0002).ref_id(9);
    w.u8(0).u8(0);
    // Inventory: one item, two of it fewer than the record, one stack
    // with a count, health and worn.
    w.vsval(1).ref_id(10).i32(-2).vsval(1);
    w.vsval(3)
        .u8(k::COUNT)
        .u16(2)
        .u8(k::HEALTH)
        .f32(80.0)
        .u8(k::WORN);
    let data = w.finish();
    let form = cf(t::REFR, 3, flags, &data);
    let r = decode::reference(&form).unwrap();
    assert!(matches!(
        r.initial,
        InitialData::Location {
            space: RefId(7),
            ..
        }
    ));
    assert_eq!(r.data.scale, Some(1.25));
    let extra = r.data.extra.as_ref().unwrap();
    assert_eq!(extra[0], Extra::Ownership(RefId(5)));
    match &extra[1] {
        Extra::Lock(l) => {
            assert_eq!((l.level, l.flags, l.key, l.tries), (50, 1, RefId(6), 2));
        }
        other => panic!("{other:?}"),
    }
    assert_eq!(extra[2], Extra::MapMarker(3));
    match &extra[3] {
        Extra::Script { script, locals } => {
            assert_eq!(*script, RefId(8));
            assert_eq!(locals.variables[0].value, Value::Number(4.0));
            assert_eq!(locals.variables[1].id, 2);
            assert_eq!(locals.variables[1].value, Value::Ref(RefId(9)));
        }
        other => panic!("{other:?}"),
    }
    let inv = r.data.inventory.as_ref().unwrap();
    assert_eq!((inv[0].item, inv[0].count), (RefId(10), -2));
    assert_eq!(
        inv[0].stacks[0],
        [Extra::Count(2), Extra::Health(80.0), Extra::Worn]
    );
    assert_eq!(decode::coverage(&form), Coverage::Exact);
}

#[test]
fn a_reference_with_a_byte_left_or_an_unknown_extra_fails() {
    let mut w = PipeWriter::new();
    w.vsval(1).u8(k::GHOST).u8(0);
    let data = w.finish();
    assert!(matches!(
        decode::coverage(&cf(t::REFR, 1, 0x8000_0000, &data)),
        Coverage::Failed(_)
    ));
    let mut w = PipeWriter::new();
    w.vsval(1).u8(0x01);
    let data = w.finish();
    let e = decode::reference(&cf(t::REFR, 1, 0x8000_0000, &data)).unwrap_err();
    assert!(e.message.contains("0x01"), "{e}");
}

#[test]
fn animation_blocks_are_taken_whole() {
    let mut w = PipeWriter::new();
    let block = PipeWriter::new().u32(1).u8(2).finish();
    w.vsval(block.len() as u32);
    let mut data = w.finish();
    data.extend_from_slice(&block);
    let r = decode::reference(&cf(t::REFR, 1, 0x1000_0000, &data)).unwrap();
    assert_eq!(r.data.animation, Some(&block[..]));
}

#[test]
fn leveled_actors_carry_their_base_data() {
    // An actor with no process whose leveled base saved its name.
    let mut w = PipeWriter::new();
    w.u8(0xFF)
        .vsval(1)
        .u8(k::LEVELED_CREATURE)
        .ref_id(2)
        .ref_id(0x80_0001);
    w.u32(0x20).wstr("Gecko");
    w.mobile_tail().actor_fixed().still_mover(false);
    let data = w.finish();
    let a = decode::actor_form(&cf(t::ACRE, 1, 0x4_0000, &data), false).unwrap();
    assert_eq!(
        a.actor.mobile.data.extra.unwrap(),
        [Extra::LeveledCreature {
            original_base: RefId(2),
            template: RefId(0x80_0001),
            flags: 0x20
        }]
    );
    assert!(a.actor.mobile.process.is_none());
}

fn actor_with(level: u8, flags: u32, process: impl Fn(&mut PipeWriter)) -> Vec<u8> {
    let mut w = PipeWriter::new();
    w.u8(level);
    w.mobile_tail();
    process(&mut w);
    w.actor_fixed();
    if flags & 0x400 != 0 {
        w.u8(2);
    }
    if flags & 0x8_0000 != 0 {
        w.vsval(1).ref_id(4).u32(25);
    }
    if flags & 0x80_0000 != 0 {
        w.vsval(1).u8(16).f32(-10.0);
    }
    w.still_mover(false).u8(0).u8(0);
    w.finish()
}

fn low_process(w: &mut PipeWriter, package: Option<u32>) {
    w.zeros(&[4, 4, 4]);
    match package {
        None => {
            w.ref_id(0);
        }
        Some(p) => {
            // A sandbox package's per-actor data (`009f5a60`).
            w.ref_id(p).u8(12);
            w.zeros(&[4, 4, 4, 4, 4, 4, 4, 4, 1, 1, 1, 1, 4, 1, 4, 4, 4, 4, 4, 6]);
            w.ref_id(0)
                .ref_id(0)
                .ref_id(0)
                .vsval(1)
                .zeros(&[4, 4, 4])
                .ref_id(0);
            w.zeros(&[4, 4, 4]).ref_id(0);
        }
    }
    w.zeros(&[1, 4]).ref_id(0).zeros(&[4, 4, 4, 1, 2, 4, 4]);
    for _ in 0..5 {
        w.ref_id(0);
    }
    w.vsval(0);
}

#[test]
fn a_dead_actor_with_a_low_process_decodes() {
    let flags = 0x400 | 0x8_0000 | 0x80_0000 | 0x20_0000;
    let data = actor_with(3, flags, |w| {
        low_process(w, Some(0x33));
        w.vsval(1).u8(25).f32(-5.0);
    });
    let form = cf(t::ACHR, 1, flags, &data);
    let a = decode::actor_form(&form, false).unwrap().actor;
    assert_eq!(a.life_state, Some(2));
    assert_eq!(a.dispositions, Some(vec![(RefId(4), 25)]));
    assert_eq!(a.permanent_modifiers, Some(vec![(16, -10.0)]));
    let p = a.mobile.process.unwrap();
    assert_eq!((p.level, p.package), (3, Some(RefId(0x33))));
    assert_eq!(p.damage_modifiers, Some(vec![(25, -5.0)]));
    assert_eq!(decode::coverage(&form), Coverage::Exact);
    // A creature has no Character bytes at the end.
    assert!(matches!(
        decode::coverage(&cf(t::ACRE, 1, flags, &data)),
        Coverage::Failed(_)
    ));
}

fn middle_high(w: &mut PipeWriter) {
    low_process(w, None);
    w.u32(0);
    w.zeros(&[
        1, 1, 1, 4, 4, 4, 1, 12, 4, 1, 1, 1, 2, 12, 1, 1, 1, 1, 4, 1, 4, 4, 1, 1, 1, 2, 4, 1, 4, 4,
        1, 1, 4, 4, 4, 4, 1,
    ]);
    for _ in 0..4 {
        w.ref_id(0);
    }
    w.u32(0).vsval(0).ref_id(0);
    // One active effect with a three-byte body.
    let body = PipeWriter::new().u8(9).finish();
    w.vsval(1).ref_id(5).u8(0).vsval(3).vsval(body.len() as u32);
    w.buf.extend_from_slice(&body);
    w.ref_id(0).ref_id(0).ref_id(0).vsval(0);
}

#[test]
fn every_process_level_decodes() {
    let flags = 0;
    let low = actor_with(3, flags, |w| low_process(w, None));
    let middle_low = actor_with(2, flags, |w| {
        low_process(w, None);
        w.u32(0);
    });
    let mid_high = actor_with(1, flags, middle_high);
    let high = actor_with(0, flags, |w| {
        middle_high(w);
        w.zeros(&[
            1, 1, 1, 1, 2, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 2, 2, 2, 1, 12, 4, 4, 4, 4, 4, 4, 1, 4,
            1, 4, 1, 4, 4, 1, 4, 4, 4, 1, 1, 4, 4, 1, 1, 4, 4, 1, 4, 4, 4, 4, 4, 1, 1, 4, 1, 4, 1,
            1, 4, 1, 1, 1, 1,
        ]);
        for _ in 0..7 {
            w.ref_id(0);
        }
        for _ in 0..6 {
            w.ref_id(0).u8(0);
        }
        w.vsval(0).vsval(0).vsval(0).u8(0);
        w.vsval(0).vsval(0).vsval(0).u8(0);
        w.vsval(0);
    });
    for (level, data) in [(3, low), (2, middle_low), (1, mid_high), (0, high)] {
        let form = cf(t::ACHR, 1, flags, &data);
        let a = decode::actor_form(&form, false).unwrap();
        assert_eq!(a.actor.mobile.process.unwrap().level, level);
    }
}

#[test]
fn movers_with_a_path_decode() {
    let mut w = PipeWriter::new();
    w.u8(0xFF).mobile_tail().actor_fixed();
    w.zeros(&[2, 2, 1, 4, 1, 4, 1, 1, 12, 12, 4, 1, 1, 1, 1, 1, 4, 4, 4])
        .pathing_location()
        .ref_id(0);
    // A request (type 0), a solution and a detailed path handler.
    w.u8(1 | 2 | 8).u8(0);
    w.pathing_location().pathing_location();
    w.zeros(&[
        4, 4, 4, 4, 4, 4, 4, 1, 1, 1, 1, 1, 1, 1, 4, 1, 1, 12, 1, 1, 1, 4, 1, 1,
    ]);
    w.vsval(1).u8(1).zeros(&[4, 4, 12, 12]);
    w.zeros(&[1, 4, 4])
        .vsval(1)
        .u32(0)
        .ref_id(0)
        .pathing_location();
    w.vsval(0).vsval(0).vsval(1).ref_id(3);
    w.zeros(&[12; 5])
        .zeros(&[4; 26])
        .zeros(&[1; 9])
        .zeros(&[4, 12, 4]);
    w.ref_id(0).vsval(0);
    w.u8(0).u8(0);
    let data = w.finish();
    let form = cf(t::ACHR, 1, 0, &data);
    assert_eq!(decode::coverage(&form), Coverage::Exact);
}

#[test]
fn the_player_decodes_perks_and_values() {
    let mut w = PipeWriter::new();
    place(&mut w, 0x3C, [10.0, 20.0, 30.0]);
    w.player_head(&[(5, 1.0)]);
    w.u8(0xFF)
        .mobile_tail()
        .actor_fixed()
        .still_mover(true)
        .u8(0)
        .u8(0);
    let mut keys = [0; 8];
    keys[2] = 0x0001_2345;
    w.player_tail(&[(0x40, 1), (0x41, 2)], 0x42, keys);
    let data = w.finish();
    let form = cf(t::ACHR, 2, 0x2, &data);
    let a = decode::actor_form(&form, true).unwrap();
    let p = a.player.unwrap();
    assert_eq!(p.script_values[5], 1.0);
    assert_eq!(p.perks, [(RefId(0x40), 1), (RefId(0x41), 2)]);
    assert_eq!(p.active_quest, RefId(0x42));
    assert_eq!(p.hotkeys[2], 0x0001_2345);
    assert_eq!(decode::coverage_of(&form, Some(0x14)), Coverage::Exact);
    // Read as an ordinary actor it doesn't fit.
    assert!(matches!(
        decode::coverage_of(&form, Some(0x15)),
        Coverage::Failed(_)
    ));
}

#[test]
fn projectiles_decode() {
    let mut w = PipeWriter::new();
    w.u8(0xFF).mobile_tail();
    w.zeros(&[4; 9]).ref_id(0).ref_id(0).ref_id(0);
    w.zeros(&[12, 4, 1, 4, 16, 12, 4, 4, 4, 4, 4]).u8(0);
    w.vsval(1).zeros(&[12, 12, 4, 4, 1, 2, 2]).ref_id(0).u8(0);
    let grenade = w.finish();
    let mut missile = grenade.clone();
    missile.extend_from_slice(&PipeWriter::new().u32(0).finish());
    assert_eq!(decode::coverage(&cf(4, 1, 0, &grenade)), Coverage::Exact);
    assert_eq!(decode::coverage(&cf(3, 1, 0, &missile)), Coverage::Exact);
    assert!(matches!(
        decode::coverage(&cf(3, 1, 0, &grenade)),
        Coverage::Failed(_)
    ));
}

#[test]
fn a_combat_controller_package_decodes() {
    // A low process running a combat package made in game (type 18).
    let data = actor_with(3, 0, |w| {
        w.zeros(&[4, 4, 4]).ref_id(0x80_0010).u8(18);
        // TESPackage: 12 bytes, no parts, a value.
        w.zeros(&[12]).u8(0).u32(0);
        w.ref_id(1).ref_id(2).u32(1);
        // CombatState.
        w.zeros(&[1, 4]);
        for _ in 0..6 {
            w.ref_id(0);
        }
        w.vsval(0)
            .ref_id(0)
            .zeros(&[4; 13])
            .zeros(&[1; 8])
            .ref_id(0);
        w.zeros(&[4, 4, 12, 4, 1, 1, 4, 4, 4, 4, 1]).ref_id(0);
        w.zeros(&[4; 13]);
        w.u8(0).vsval(0).vsval(0).zeros(&[4, 1]).u8(0);
        for _ in 0..2 {
            w.ref_id(0).zeros(&[4, 4]);
        }
        w.u32(0).ref_id(0).zeros(&[1, 4]).zeros(&[4; 22]);
        // Procedures: ranged attack, none, then a list with a move.
        w.u8(0).u32(0).zeros(&[4, 12]).zeros(&[4; 10]);
        w.u8(0xFF);
        w.vsval(1).u8(6).u32(0).u32(0);
        w.zeros(&[12]).ref_id(0).zeros(&[12]).ref_id(0);
        w.u32(0)
            .ref_id(0)
            .zeros(&[4, 4])
            .zeros(&[4; 8])
            .zeros(&[12, 1, 1]);
        // A planner with one plan.
        w.u8(1).u32(1).zeros(&[1, 4]).zeros(&[4, 1]);
        w.zeros(&[1, 4, 4, 4, 1, 4, 4, 4, 1, 1, 1, 4, 4, 12, 1, 12, 1, 1, 1, 1]);
        // No per-actor data, then the package's three values and target.
        w.u8(0xFF).zeros(&[4, 4, 4]).ref_id(0);
        w.zeros(&[1, 4]).ref_id(0).zeros(&[4, 4, 4, 1, 2, 4, 4]);
        for _ in 0..5 {
            w.ref_id(0);
        }
        w.vsval(0);
    });
    let form = cf(t::ACHR, 1, 0, &data);
    let p = decode::actor_form(&form, false)
        .unwrap()
        .actor
        .mobile
        .process
        .unwrap();
    assert_eq!(p.package, Some(RefId(0x80_0010)));
}

#[test]
fn base_forms_decode() {
    let book = PipeWriter::new().u32(25).u8(33).finish();
    let b = decode::base_form(&cf(t::BOOK, 1, 0x2 | 0x20, &book)).unwrap();
    assert_eq!((b.value, b.teaches), (Some(25), Some(33)));
    let list = PipeWriter::new().u32(2).ref_id(4).ref_id(5).finish();
    let l = decode::base_form(&cf(t::FLST, 1, 0x8000_0000, &list)).unwrap();
    assert_eq!(l.added, Some(vec![RefId(4), RefId(5)]));
    let lev = PipeWriter::new()
        .vsval(1)
        .ref_id(6)
        .u16(1)
        .u16(2)
        .f32(-1.0)
        .finish();
    let l = decode::base_form(&cf(t::LVLI, 1, 0x8000_0000, &lev)).unwrap();
    assert_eq!(l.added, Some(vec![RefId(6)]));
    let zone = PipeWriter::new().u8(1).bytes(&[0; 16]).finish();
    assert_eq!(
        decode::coverage(&cf(t::ECZN, 1, 0x8000_0002, &zone)),
        Coverage::Exact
    );
    assert_eq!(decode::coverage(&cf(19, 1, 0, &[])), Coverage::Exact);
    assert!(matches!(
        decode::coverage(&cf(24, 1, 0, &[])),
        Coverage::Skipped(_)
    ));
}

#[test]
fn an_npc_face_decodes_with_the_record_sizes() {
    let mut w = PipeWriter::new();
    w.u8(1);
    for _ in 0..130 {
        w.f32(0.5);
    }
    w.ref_id(1)
        .ref_id(2)
        .f32(1.0)
        .u32(0)
        .vsval(1)
        .ref_id(3)
        .u8(1);
    let data = w.finish();
    assert_eq!(
        decode::coverage(&cf(t::NPC_, 7, 0x800 | 0x100_0000, &data)),
        Coverage::Exact
    );
}

#[test]
fn weather_and_radio_decode() {
    let mut w = PipeWriter::new();
    w.ref_id(1).ref_id(2).ref_id(3).ref_id(0);
    w.f32(13.5).f32(12.0).f32(0.5).u32(0).f32(0.0);
    w.f32(0.1).f32(0.2).f32(0.3).f32(0.0).f32(1.0).u32(0);
    let data = w.finish();
    let sky = decode::sky(&GlobalData {
        kind: 8,
        offset: 0,
        data: &data,
    })
    .unwrap();
    assert_eq!(
        (sky.current, sky.last, sky.default),
        (RefId(1), RefId(2), RefId(3))
    );
    assert_eq!((sky.hour, sky.weather_pct), (13.5, 0.5));

    let mut w = PipeWriter::new();
    w.u32(0).u8(1).wstr("");
    w.vsval(1).ref_id(4).u8(1).u8(0).u8(0).u8(0);
    w.vsval(1).ref_id(4).zeros(&[4, 4, 1, 1, 4]).vsval(0).u8(0);
    w.ref_id(4).ref_id(0).vsval(2).ref_id(4).ref_id(5);
    let data = w.finish();
    let radio = decode::radio(&GlobalData {
        kind: 10,
        offset: 0,
        data: &data,
    })
    .unwrap();
    assert!(radio.on);
    assert_eq!(radio.active, RefId(4));
    assert_eq!(radio.discovered, [RefId(4), RefId(5)]);
    assert_eq!(radio.playing, [RefId(4)]);
}

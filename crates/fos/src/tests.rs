//! Tests on synthetic saves built by `write` (never a real save).

use crate::decode::{self, Coverage, InitialData, Value};
use crate::write::{Form, PipeWriter, SaveWriter};
use crate::{change_flag_name, save_type as t, Pipe, RefId, Save, MINOR_VERSION};

fn sample() -> SaveWriter {
    let mut globals = PipeWriter::new();
    globals.vsval(2).ref_id(1).f32(20.5).ref_id(2).f32(2281.0);
    let mut stats = PipeWriter::new();
    stats.u32(3).u32(1).u32(0).u32(7);
    SaveWriter {
        plugins: vec!["FalloutNV.esm".into(), "DeadMoney.esm".into()],
        global_data_1: vec![(0, stats.finish()), (3, globals.finish())],
        forms: vec![
            Form::new(1, 0, t::INFO, Vec::new()),
            Form::new(3, 1, t::REFR, PipeWriter::new().u32(0x800).finish()),
            // Long enough for a two-byte length.
            Form::new(2, 0, t::REFR, vec![0xAB; 300]),
        ],
        form_ids: vec![0x0000_0038, 0x0000_0039, 0x0100_1234],
        worldspaces: vec![0x000D_A726],
        history: vec!["one".into(), String::new()],
        ..SaveWriter::default()
    }
}

#[test]
fn every_part_is_read_where_the_table_says() {
    let w = sample();
    let (bytes, at) = w.build();
    let save = Save::parse(&bytes).unwrap();
    let h = &save.header;
    assert_eq!(h.version, 0x30);
    assert_eq!(h.language, "ENGLISH");
    assert_eq!((h.screenshot_width, h.screenshot_height), (4, 2));
    assert_eq!(h.save_number, 3);
    assert_eq!(h.player_name, "Tester");
    assert_eq!(h.karma_title, "Drifter");
    assert_eq!(h.level, 2);
    assert_eq!(h.location, "Goodsprings");
    assert_eq!(h.play_time, "001.02.03");
    assert_eq!(save.screenshot_offset, 11 + 4 + w.header().len());
    assert_eq!(save.minor_version, MINOR_VERSION);
    assert_eq!(save.plugins, ["FalloutNV.esm", "DeadMoney.esm"]);
    assert_eq!(save.table.global_data_1 as usize, at.global_data_1);
    assert_eq!(save.table.change_forms as usize, at.change_forms);
    assert_eq!(save.table.change_form_count, 3);
    assert_eq!(save.global_data_1.len(), 2);
    assert_eq!(save.global_data_2[0].kind, 1000);
    assert!(save.global_data_2[0].data.is_empty());
    let forms = &save.change_forms;
    assert_eq!(forms.len(), 3);
    assert_eq!(forms[0].type_name(), Some("INFO"));
    assert!(forms[0].data.is_empty());
    assert_eq!((forms[1].ref_id, forms[1].flags), (RefId(3), 1));
    assert_eq!(forms[2].data.len(), 300);
    assert_eq!(forms[2].data_offset, forms[2].offset + 3 + 4 + 2 + 2);
    assert_eq!(save.form_ids, [0x38, 0x39, 0x0100_1234]);
    assert_eq!(save.worldspaces, [0xDA726]);
    assert_eq!(save.history, ["one", ""]);
    assert_eq!(save.form_id(forms[1].ref_id), Some(0x0100_1234));
    assert_eq!(save.change_form_of(0x39).map(|c| c.data.len()), Some(300));
}

#[test]
fn a_four_byte_length_is_read() {
    let mut w = SaveWriter::default();
    w.forms.push(Form::new(1, 0, t::REFR, vec![7; 0x1_0001]));
    w.form_ids = vec![0x14];
    let (bytes, _) = w.build();
    let save = Save::parse(&bytes).unwrap();
    assert_eq!(save.change_forms[0].data.len(), 0x1_0001);
}

#[test]
fn ref_ids_resolve_through_the_form_id_array() {
    let ids = [0x10, 0x20];
    assert_eq!(RefId(0).form_id(&ids), None);
    assert_eq!(RefId(1).form_id(&ids), Some(0x10));
    assert_eq!(RefId(2).form_id(&ids), Some(0x20));
    assert_eq!(RefId(3).form_id(&ids), None);
    // Created forms carry their own id.
    assert_eq!(RefId(0x80_1234).form_id(&ids), Some(0xFF00_1234));
    assert!(RefId(0x80_0000).is_created());
}

#[test]
fn variable_sized_values_take_one_two_or_four_bytes() {
    let mut w = PipeWriter::new();
    w.vsval(5).vsval(300).vsval(70_000).vsval(0x3F).vsval(0x40);
    let bytes = w.finish();
    assert_eq!(bytes.len(), 2 + 3 + 5 + 2 + 3);
    let mut p = Pipe::new(&bytes, 0);
    for v in [5, 300, 70_000, 0x3F, 0x40] {
        assert_eq!(p.vsval().unwrap(), v);
    }
    p.finish("values").unwrap();
}

#[test]
fn a_missing_bar_is_an_error() {
    let bytes = [1, 0, 0, 0, b'#'];
    let mut p = Pipe::new(&bytes, 0x100);
    let e = p.u32().unwrap_err();
    assert_eq!(e.offset, 0x104);
}

#[test]
fn bad_files_are_refused_with_where() {
    let (good, at) = sample().build();
    // Not a save.
    assert!(Save::parse(b"TES4 nothing").is_err());
    // Cut short anywhere.
    for cut in [5, 20, good.len() / 2, good.len() - 1] {
        assert!(Save::parse(&good[..cut]).is_err(), "cut at {cut}");
    }
    // A byte too many.
    let mut longer = good.clone();
    longer.push(0);
    let e = Save::parse(&longer).unwrap_err();
    assert!(e.message.contains("after the history"), "{e}");
    // The table pointing somewhere else.
    let mut moved = good.clone();
    let o = at.location_table + 0x0C;
    moved[o] = moved[o].wrapping_add(1);
    let e = Save::parse(&moved).unwrap_err();
    assert!(e.message.contains("change forms"), "{e}");
    // The table's unused part isn't zero.
    let mut dirty = good.clone();
    dirty[at.location_table + 0x30] = 1;
    assert!(Save::parse(&dirty).is_err());
    // A change form's data running into the next part.
    let mut long_form = good;
    long_form[at.change_forms + 9] = 0xFF;
    assert!(Save::parse(&long_form).is_err());
}

fn quest_data(version_flag_byte: bool) -> Vec<u8> {
    let mut p = PipeWriter::new();
    p.u32(0x4000).u8(0x03).f32(5.0);
    // Stages: 10 done with a dated log entry, 20 not done with an undated
    // item, 30 done without items.
    p.vsval(3);
    p.u8(10)
        .u8(1)
        .vsval(1)
        .u8(0)
        .u8(1)
        .bytes(&[0x23, 0x01, 0xE9, 0x08]);
    p.u8(20).u8(0).vsval(1).u8(1).u8(0);
    p.u8(30).u8(1).vsval(0);
    // Script: a number and a reference variable, effect data, flag byte.
    p.vsval(2);
    p.u32(1).f64(2.5);
    p.u32(0x8000_0002).ref_id(0x80_0005);
    p.u8(1).bytes(&[1, 2, 3, 4, 5, 6, 7, 8]);
    if version_flag_byte {
        p.u8(0);
    }
    // Objectives.
    p.vsval(2).u32(10).u32(3).u32(20).u32(1);
    p.finish()
}

fn form(save_type: u8, flags: u32, data: Vec<u8>) -> SaveWriter {
    SaveWriter {
        forms: vec![Form::new(1, flags, save_type, data)],
        form_ids: vec![0x0010_4C1C],
        ..SaveWriter::default()
    }
}

const QUEST_ALL: u32 = 0x1 | 0x2 | 0x4 | 0x8000_0000 | 0x4000_0000 | 0x2000_0000;

#[test]
fn a_quest_decodes_stages_script_and_objectives() {
    let (bytes, _) = form(t::QUST, QUEST_ALL, quest_data(true)).build();
    let save = Save::parse(&bytes).unwrap();
    let cf = &save.change_forms[0];
    let q = decode::quest(cf).unwrap();
    assert_eq!(q.form_flags, Some(0x4000));
    assert_eq!(q.flags, Some(0x03));
    assert_eq!(q.script_delay, Some(5.0));
    let stages = q.stages.as_ref().unwrap();
    assert_eq!(stages.len(), 3);
    assert_eq!(
        stages[0].items[0].log_date.map(|d| (d.day, d.year)),
        Some((291, 2281))
    );
    assert_eq!(stages[1].items[0].log_date, None);
    assert!(stages[2].items.is_empty());
    // The highest stage done, not the highest listed.
    assert_eq!(q.current_stage(), Some(30));
    let script = q.script.as_ref().unwrap();
    assert_eq!(script.variables[0].value, Value::Number(2.5));
    assert_eq!(script.variables[1].id, 2);
    assert_eq!(script.variables[1].value, Value::Ref(RefId(0x80_0005)));
    assert_eq!(script.effect_data, Some([1, 2, 3, 4, 5, 6, 7, 8]));
    assert_eq!(script.flag, Some(0));
    let objectives = q.objectives.as_ref().unwrap();
    assert!(objectives[0].completed() && objectives[0].displayed());
    assert!(!objectives[1].completed() && objectives[1].displayed());
    assert_eq!(decode::coverage(cf), Coverage::Exact);
}

#[test]
fn old_script_locals_have_no_flag_byte() {
    let (bytes, _) = form(t::QUST, QUEST_ALL, quest_data(false)).build();
    let save = Save::parse(&bytes).unwrap();
    let mut cf = save.change_forms[0].clone();
    // Version 27 expects the byte, so the objectives' count is misread.
    assert!(decode::quest(&cf).is_err());
    cf.version = 0x14;
    assert_eq!(decode::quest(&cf).unwrap().script.unwrap().flag, None);
}

#[test]
fn a_quest_with_a_byte_left_over_fails() {
    let mut data = quest_data(true);
    data.extend_from_slice(&[0, b'|']);
    let (bytes, _) = form(t::QUST, QUEST_ALL, data).build();
    let save = Save::parse(&bytes).unwrap();
    assert!(matches!(
        decode::coverage(&save.change_forms[0]),
        Coverage::Failed(_)
    ));
}

#[test]
fn globals_misc_stats_and_location_decode() {
    let mut location = PipeWriter::new();
    location
        .u32(0xFF00_10A7)
        .ref_id(1)
        .i32(-17)
        .i32(0)
        .ref_id(1)
        .bytes(&[0; 12])
        .ref_id(2)
        .u32(3)
        .u32(0)
        .u32(1)
        .u8(0);
    let mut w = sample();
    w.global_data_1.push((1, location.finish()));
    let (bytes, _) = w.build();
    let save = Save::parse(&bytes).unwrap();
    let globals = decode::globals(save.global_data(3).unwrap()).unwrap();
    assert_eq!(globals, [(RefId(1), 20.5), (RefId(2), 2281.0)]);
    assert_eq!(
        decode::misc_stats(save.global_data(0).unwrap()).unwrap(),
        [1, 0, 7]
    );
    let l = decode::location(save.global_data(1).unwrap()).unwrap();
    assert_eq!(l.next_created_id, 0xFF00_10A7);
    assert_eq!(l.grid, [-17, 0]);
    assert_eq!(l.loading_menu.1, [3, 0, 1]);
}

fn place(extra: &[u8]) -> Vec<u8> {
    let mut b = vec![0, 0, 2];
    for v in [1.0f32, 2.0, 3.0, 0.0, 0.0, 1.5] {
        b.extend_from_slice(&v.to_le_bytes());
    }
    b.extend_from_slice(extra);
    b
}

#[test]
fn references_start_with_their_initial_data() {
    // Moved: 27 bytes, then form flags and scale.
    let mut p = PipeWriter::new();
    p.bytes(&place(&[])).u32(0).f32(2.0);
    let (bytes, _) = form(t::REFR, 0x2 | 0x1 | 0x10, p.finish()).build();
    let save = Save::parse(&bytes).unwrap();
    let cf = &save.change_forms[0];
    let start = decode::reference_start(cf).unwrap();
    assert_eq!(
        start.initial.place(),
        Some((RefId(2), [1.0, 2.0, 3.0], [0.0, 0.0, 1.5]))
    );
    assert_eq!(decode::coverage(cf), Coverage::Exact);

    // Changed cell: 34 bytes with the editor cell and grid.
    let mut p = PipeWriter::new();
    p.bytes(&place(&[0, 0, 1, 0xFF, 0xFF, 2, 0]));
    let (bytes, _) = form(t::REFR, 0x8 | 0x2, p.finish()).build();
    let save = Save::parse(&bytes).unwrap();
    match decode::reference_start(&save.change_forms[0])
        .unwrap()
        .initial
    {
        InitialData::Moved {
            editor_cell,
            editor_grid,
            ..
        } => assert_eq!((editor_cell, editor_grid), (RefId(1), [-1, 2])),
        other => panic!("{other:?}"),
    }

    // Created: 31 bytes with flags and base, by the refID's created bit;
    // then the Havok block.
    let mut p = PipeWriter::new();
    p.bytes(&place(&[1, 0, 0, 9])).vsval(4).bytes(&[1, 2, 3]);
    let mut w = form(t::REFR, 0x4, p.finish());
    w.forms[0].ref_id = 0x80_0001;
    let (bytes, _) = w.build();
    let save = Save::parse(&bytes).unwrap();
    let cf = &save.change_forms[0];
    let start = decode::reference_start(cf).unwrap();
    match start.initial {
        InitialData::Created { flags, base, .. } => assert_eq!((flags, base), (1, RefId(9))),
        ref other => panic!("{other:?}"),
    }
    assert_eq!(start.havok, Some(&[1, 2, 3, b'|'][..]));
    assert_eq!(decode::coverage(cf), Coverage::Exact);

    // An empty inventory decodes; one cut short fails.
    let (bytes, _) = form(t::REFR, 0x20, vec![0, b'|']).build();
    let save = Save::parse(&bytes).unwrap();
    assert_eq!(decode::coverage(&save.change_forms[0]), Coverage::Exact);
    let (bytes, _) = form(t::REFR, 0x20, vec![4, b'|']).build();
    let save = Save::parse(&bytes).unwrap();
    assert!(matches!(
        decode::coverage(&save.change_forms[0]),
        Coverage::Failed(_)
    ));
}

#[test]
fn cells_decode_inside_and_out() {
    // Exterior, char grid, detach time, flags, seen data (32 bytes),
    // name and owner.
    let mut p = PipeWriter::new();
    p.bytes(&[0, 0, 0xFE, 3, 7, 0, 0, 0])
        .u8(0x40)
        .bytes(&[0xFF; 32])
        .wstr("Somewhere")
        .ref_id(1);
    let flags = 0x4000_0000 | 0x2000_0000 | 0x2 | 0x8000_0000 | 0x4 | 0x8;
    let (bytes, _) = form(t::CELL, flags, p.finish()).build();
    let save = Save::parse(&bytes).unwrap();
    let cf = &save.change_forms[0];
    assert_eq!(
        decode::cell(cf, None).unwrap(),
        InitialData::ExteriorCell {
            worldspace: 0,
            x: -2,
            y: 3,
            detach_time: 7,
            short: false
        }
    );
    assert_eq!(decode::coverage(cf), Coverage::Exact);

    // Interior seen data: counted parts.
    let mut p = PipeWriter::new();
    p.vsval(2);
    for i in 0..2 {
        p.u8(i).u8(i).bytes(&[0; 32]);
    }
    let (bytes, _) = form(t::CELL, 0x8000_0000, p.finish()).build();
    let save = Save::parse(&bytes).unwrap();
    let cf = &save.change_forms[0];
    assert!(decode::cell(cf, Some(false)).is_err());
    assert_eq!(decode::cell(cf, Some(true)), Ok(InitialData::None));
    assert_eq!(decode::coverage(cf), Coverage::Exact);
}

#[test]
fn small_types_decode() {
    // NPC_: base data, spells, attributes, name, gender.
    let mut p = PipeWriter::new();
    p.bytes(&[0; 24])
        .vsval(1)
        .ref_id(1)
        .vsval(0)
        .bytes(&[5, 5, 5, 5, 5, 5, 10])
        .wstr("Name")
        .u8(1);
    let flags = 0x2 | 0x10 | 0x4 | 0x20 | 0x100_0000;
    let (bytes, _) = form(t::NPC_, flags, p.finish()).build();
    let save = Save::parse(&bytes).unwrap();
    assert_eq!(
        decode::actor_base(&save.change_forms[0])
            .unwrap()
            .as_deref(),
        Some("Name")
    );
    // A face cut short fails (`tests_refs` has a whole one).
    let (bytes, _) = form(t::NPC_, 0x800, vec![0, b'|']).build();
    let save = Save::parse(&bytes).unwrap();
    assert!(matches!(
        decode::coverage(&save.change_forms[0]),
        Coverage::Failed(_)
    ));

    // FACT: reactions, flags, crimes (major, minor).
    let mut p = PipeWriter::new();
    p.vsval(1)
        .ref_id(1)
        .i32(-10)
        .i32(1)
        .u32(0x101)
        .i32(2)
        .i32(5);
    let (bytes, _) = form(t::FACT, 0x4 | 0x2 | 0x8000_0000, p.finish()).build();
    let save = Save::parse(&bytes).unwrap();
    let f = decode::faction(&save.change_forms[0]).unwrap();
    assert_eq!(f.reactions, Some(vec![(RefId(1), -10, 1)]));
    assert_eq!((f.flags, f.crimes), (Some(0x101), Some((2, 5))));

    // CLAS tag skills, CHAL progress, INFO said once.
    let mut p = PipeWriter::new();
    p.i32(41).i32(40).i32(45).i32(-1);
    let (bytes, _) = form(t::CLAS, 0x2, p.finish()).build();
    let save = Save::parse(&bytes).unwrap();
    assert_eq!(
        decode::class(&save.change_forms[0]).unwrap(),
        Some([41, 40, 45, -1])
    );
    let mut p = PipeWriter::new();
    p.u32(1).u32(0);
    let (bytes, _) = form(t::CHAL, 0x2, p.finish()).build();
    let save = Save::parse(&bytes).unwrap();
    assert_eq!(decode::pair(&save.change_forms[0]).unwrap(), (1, 0));
    let (bytes, _) = form(t::INFO, 0x8000_0000, Vec::new()).build();
    let save = Save::parse(&bytes).unwrap();
    assert_eq!(decode::coverage(&save.change_forms[0]), Coverage::Exact);
}

#[test]
fn change_flags_are_named_per_type() {
    assert_eq!(change_flag_name(t::QUST, 0x8000_0000), Some("QUEST_STAGES"));
    assert_eq!(
        change_flag_name(t::INFO, 0x8000_0000),
        Some("TOPIC_SAIDONCE")
    );
    assert_eq!(change_flag_name(t::ACHR, 0x400), Some("ACTOR_LIFESTATE"));
    assert_eq!(
        change_flag_name(t::REFR, 0x400),
        Some("OBJECT_EXTRA_ITEM_DATA")
    );
    assert_eq!(change_flag_name(t::CELL, 0x1), Some("FORM_FLAGS"));
    assert_eq!(change_flag_name(t::QUST, 0x8), None);
}

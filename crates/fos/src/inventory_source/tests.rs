use super::*;
use crate::{
    decode::extra_kind as k,
    write::{Form, PipeWriter, SaveWriter},
};

fn count(value: i16) -> Vec<u8> {
    PipeWriter::new().u8(k::COUNT).u16(value as u16).finish()
}
fn health(bits: u32) -> Vec<u8> {
    PipeWriter::new().u8(k::HEALTH).u32(bits).finish()
}
fn worn() -> Vec<u8> {
    PipeWriter::new().u8(k::WORN).finish()
}

fn list(extras: &[Vec<u8>]) -> Vec<u8> {
    let mut bytes = PipeWriter::new().vsval(extras.len() as u32).finish();
    for extra in extras {
        bytes.extend_from_slice(extra);
    }
    bytes
}

fn item(id: u32, delta: i32, stacks: &[Vec<u8>]) -> Vec<u8> {
    let mut bytes = PipeWriter::new()
        .ref_id(id)
        .i32(delta)
        .vsval(stacks.len() as u32)
        .finish();
    for stack in stacks {
        bytes.extend_from_slice(stack);
    }
    bytes
}

fn inventory(items: &[Vec<u8>]) -> Vec<u8> {
    let mut bytes = PipeWriter::new().vsval(items.len() as u32).finish();
    for item in items {
        bytes.extend_from_slice(item);
    }
    bytes
}

fn source(data: Vec<u8>, flags: u32) -> InventorySource {
    let writer = SaveWriter {
        form_ids: vec![0x33, 0x44],
        forms: vec![Form::new(1, flags, save_type::REFR, data)],
        ..SaveWriter::default()
    };
    InventorySource::parse(writer.build().0.into()).unwrap()
}

fn items(record: &InventoryRecord) -> &[ItemEnvelope] {
    match record.state() {
        RecordState::Decoded(items) => items,
        other => panic!("expected complete inventory, got {other:?}"),
    }
}

#[test]
fn original_item_stack_and_extra_bytes_keep_presence_order_and_signed_counts() {
    let extras = [
        vec![worn()],
        vec![count(0), health((-1.0f32).to_bits()), worn()],
        vec![count(1), health(42.5f32.to_bits()), worn()],
    ];
    let stacks: Vec<_> = extras.iter().map(|v| list(v)).collect();
    let first = item(2, -2, &stacks);
    let second = item(0x800005, 99, &[]);
    let data = inventory(&[first.clone(), second.clone()]);
    let captured = source(data.clone(), 0x20);
    let record = &captured.records()[0];
    assert_eq!(record.source().original_data(), data);
    let decoded = items(record);
    assert_eq!(decoded.len(), 2);
    assert_eq!(decoded[0].identity().saved(), RefId(2));
    assert_eq!(
        decoded[0].identity().resolution(),
        Resolution::ResolvedInSave(0x44)
    );
    assert_eq!(decoded[0].count_delta(), -2);
    assert_eq!(decoded[0].original_payload(), first);
    assert_eq!(decoded[1].original_payload(), second);
    assert_eq!(
        decoded[1].identity().resolution(),
        Resolution::ResolvedInSave(0xff000005)
    );
    assert_eq!(decoded[1].count_delta(), 99);
    assert_eq!(decoded[1].ordinal(), 1);
    // Expected boundaries come from independently assembled fixture segments.
    assert_eq!(decoded[0].record_data_span(), 2..2 + first.len());
    assert_eq!(decoded[1].record_data_span(), 2 + first.len()..data.len());
    let mut start = 2 + 4 + 5 + 2; // inventory count, refID, i32, stack count
    for (ordinal, ((stack, bytes), expected_extras)) in decoded[0]
        .stacks()
        .iter()
        .zip(&stacks)
        .zip(&extras)
        .enumerate()
    {
        assert_eq!(stack.ordinal(), ordinal);
        assert_eq!(stack.original_payload(), bytes);
        assert_eq!(&captured.original_bytes()[stack.file_span()], bytes);
        assert!(stack.source().save().same_capture(captured.context()));
        assert_eq!(stack.record_data_span(), start..start + bytes.len());
        let mut extra_start = start + 2;
        for (ordinal, (extra, bytes)) in stack.extras().iter().zip(expected_extras).enumerate() {
            assert_eq!(extra.ordinal(), ordinal);
            assert_eq!(extra.original_payload(), bytes);
            assert_eq!(&captured.original_bytes()[extra.file_span()], bytes);
            assert!(extra.source().save().same_capture(captured.context()));
            assert_eq!(
                extra.record_data_span(),
                extra_start..extra_start + bytes.len()
            );
            extra_start += bytes.len();
        }
        start += bytes.len();
    }
    assert!(matches!(
        decoded[0].stacks()[0].extras()[0].decoded(),
        decode::Extra::Worn
    ));
    assert!(matches!(
        decoded[0].stacks()[1].extras()[0].decoded(),
        decode::Extra::Count(0)
    ));
    assert!(matches!(
        decoded[0].stacks()[2].extras()[0].decoded(),
        decode::Extra::Count(1)
    ));
    let span = decoded[0].file_span();
    assert_eq!(&captured.original_bytes()[span], first);
    // Existing public decoder still returns its original structure/results.
    let parsed = Save::parse(captured.original_bytes()).unwrap();
    let old = decode::reference(&parsed.change_forms[0])
        .unwrap()
        .data
        .inventory
        .unwrap();
    assert_eq!(old[0].count, -2);
    assert_eq!(old[0].stacks[1][0], decode::Extra::Count(0));
}

#[test]
fn absent_inventory_empty_inventory_empty_stack_and_duplicates_remain_distinct() {
    let absent = source(vec![], 0);
    assert!(matches!(
        absent.records()[0].state(),
        RecordState::NoInventory
    ));
    let empty = source(inventory(&[]), 0x20);
    assert!(items(&empty.records()[0]).is_empty());
    let bytes = item(
        2,
        0,
        &[list(&[]), list(&[count(-4), count(1), worn(), worn()])],
    );
    let captured = source(inventory(std::slice::from_ref(&bytes)), 0x20);
    let one = &items(&captured.records()[0])[0];
    assert_eq!(one.original_payload(), bytes);
    assert_eq!(one.stacks().len(), 2);
    assert!(one.stacks()[0].extras().is_empty());
    assert_eq!(one.stacks()[0].original_payload(), [0, b'|']);
    let values: Vec<_> = one.stacks()[1]
        .extras()
        .iter()
        .map(|e| e.decoded())
        .collect();
    assert_eq!(
        values,
        [
            &decode::Extra::Count(-4),
            &decode::Extra::Count(1),
            &decode::Extra::Worn,
            &decode::Extra::Worn
        ]
    );
}

#[test]
fn health_payload_preserves_raw_nan_infinity_and_signed_zero_bits() {
    let bits = [
        0u32, 0x80000000, 0x7fc12345, 0xffc54321, 0x7f812345, 0xff812345, 0x7f800000, 0xff800000, 1,
    ];
    let stacks: Vec<_> = bits.iter().map(|bits| list(&[health(*bits)])).collect();
    let raw = item(2, -1, &stacks);
    let captured = source(inventory(std::slice::from_ref(&raw)), 0x20);
    let entry = &items(&captured.records()[0])[0];
    assert_eq!(entry.original_payload(), raw);
    for (stack, expected) in entry.stacks().iter().zip(bits) {
        let extra = &stack.extras()[0];
        assert_eq!(extra.original_payload(), health(expected));
        match extra.decoded() {
            decode::Extra::Health(value) => assert_eq!(value.to_bits(), expected),
            other => panic!("wrong extra {other:?}"),
        }
    }
}

#[test]
fn known_layout_opaque_extra_retains_its_whole_payload_and_following_boundaries() {
    let opaque = PipeWriter::new()
        .u8(k::PACKAGE_START_LOCATION)
        .ref_id(2)
        .bytes(&[0xa5; 12])
        .u32(0x11223344)
        .finish();
    let raw = item(
        2,
        1,
        &[list(&[
            count(3),
            opaque.clone(),
            health(9.0f32.to_bits()),
            worn(),
        ])],
    );
    let next = item(2, -7, &[]);
    let captured = source(inventory(&[raw.clone(), next.clone()]), 0x20);
    let entries = items(&captured.records()[0]);
    let extra = &entries[0].stacks()[0].extras()[1];
    assert!(extra.is_opaque());
    assert_eq!(extra.kind(), k::PACKAGE_START_LOCATION);
    assert!(matches!(
        extra.decoded(),
        decode::Extra::Other(k::PACKAGE_START_LOCATION)
    ));
    assert_eq!(extra.original_payload(), opaque);
    assert_eq!(entries[0].original_payload(), raw);
    assert_eq!(entries[1].original_payload(), next);
}

#[test]
fn failed_complete_records_preserve_raw_data_and_never_publish_partial_items() {
    let good = inventory(&[item(2, 5, &[list(&[count(3)])])]);
    let unknown = inventory(&[item(2, 5, &[list(&[PipeWriter::new().u8(0x01).finish()])])]);
    let truncated = inventory(&[item(
        2,
        5,
        &[list(&[PipeWriter::new().u8(k::HEALTH).finish()])],
    )]);
    let mut separator = good.clone();
    *separator.last_mut().unwrap() = 0;
    let mut trailing = good.clone();
    trailing.push(0);
    for raw in [unknown, truncated, separator, trailing] {
        let writer = SaveWriter {
            form_ids: vec![0x33, 0x44],
            forms: vec![
                Form::new(1, 0x20, save_type::REFR, raw.clone()),
                Form::new(1, 0x20, save_type::REFR, good.clone()),
            ],
            ..SaveWriter::default()
        };
        let bytes = writer.build().0;
        let parsed = Save::parse(&bytes).unwrap();
        assert!(decode::reference(&parsed.change_forms[0]).is_err());
        let captured = InventorySource::parse(bytes.into()).unwrap();
        assert_eq!(captured.records()[0].source().original_data(), raw);
        assert!(matches!(
            captured.records()[0].state(),
            RecordState::PreservedUndecoded(RecordFailure::Decode(_))
        ));
        assert_eq!(items(&captured.records()[1]).len(), 1);
    }
}

fn actor_inventory(kind: u8, player: bool, inventory: &[u8]) -> (Vec<u8>, usize) {
    let mut head = PipeWriter::new();
    if player {
        head.player_head(&[]);
    }
    head.u8(0xff);
    let mut bytes = head.finish();
    let inventory_start = bytes.len();
    bytes.extend_from_slice(inventory);
    let mut tail = PipeWriter::new();
    tail.mobile_tail().actor_fixed().still_mover(player);
    if kind == save_type::ACHR {
        tail.u8(0).u8(0);
    }
    if player {
        tail.player_tail(&[], 0, [0; 8]);
    }
    bytes.extend_from_slice(&tail.finish());
    (bytes, inventory_start)
}

#[test]
fn reference_actor_creature_and_player_share_the_existing_complete_decode_path() {
    let raw = item(2, -8, &[list(&[count(4), worn()])]);
    let inv = inventory(std::slice::from_ref(&raw));
    for (kind, player) in [
        (save_type::REFR, false),
        (save_type::ACHR, false),
        (save_type::ACRE, false),
        (save_type::ACHR, true),
    ] {
        let (data, start) = if kind == save_type::REFR {
            (inv.clone(), 0)
        } else {
            actor_inventory(kind, player, &inv)
        };
        let writer = SaveWriter {
            form_ids: vec![if player { 0x14 } else { 0x33 }, 0x44],
            forms: vec![Form::new(1, 0x20, kind, data)],
            ..SaveWriter::default()
        };
        let bytes = writer.build().0;
        let parsed = Save::parse(&bytes).unwrap();
        let old = if kind == save_type::REFR {
            decode::reference(&parsed.change_forms[0])
                .unwrap()
                .data
                .inventory
        } else {
            decode::actor_form(&parsed.change_forms[0], player)
                .unwrap()
                .actor
                .mobile
                .data
                .inventory
        }
        .unwrap();
        assert_eq!(old[0].count, -8);
        let captured = InventorySource::parse(bytes.into()).unwrap();
        let entry = &items(&captured.records()[0])[0];
        assert_eq!(entry.count_delta(), old[0].count);
        assert_eq!(entry.original_payload(), raw);
        assert_eq!(entry.record_data_span(), start + 2..start + 2 + raw.len());
    }
}

#[test]
fn created_holder_havok_reader_rebasing_keeps_record_and_file_spans() {
    let raw = item(2, 2, &[list(&[health(2.0f32.to_bits())])]);
    let havok = PipeWriter::new().u32(0x11223344).finish();
    let mut prefix = PipeWriter::new();
    prefix.bytes(&[0; 31]).vsval(havok.len() as u32);
    let mut data = prefix.finish();
    data.extend_from_slice(&havok);
    let start = data.len() + 2;
    data.extend_from_slice(&inventory(std::slice::from_ref(&raw)));
    let writer = SaveWriter {
        form_ids: vec![0x33, 0x44],
        forms: vec![Form::new(0x800007, 0x24, save_type::REFR, data)],
        ..SaveWriter::default()
    };
    let bytes = writer.build().0;
    let parsed = Save::parse(&bytes).unwrap();
    let data_start = parsed.change_forms[0].data_offset;
    assert!(data_start > 0);
    assert_eq!(
        decode::reference(&parsed.change_forms[0]).unwrap().havok,
        Some(havok.as_slice())
    );
    let captured = InventorySource::parse(bytes.into()).unwrap();
    let entry = &items(&captured.records()[0])[0];
    assert_eq!(
        entry.source().holder().resolution(),
        Resolution::ResolvedInSave(0xff000007)
    );
    assert_eq!(entry.record_data_span(), start..start + raw.len());
    assert_eq!(
        entry.file_span(),
        data_start + start..data_start + start + raw.len()
    );
    assert_eq!(entry.original_payload(), raw);
}

#[test]
fn two_save_contexts_keep_identical_saved_ids_isolated_and_items_outlive_owners() {
    fn make(holder: u32, item_id: u32, plugin: &str) -> InventorySource {
        let writer = SaveWriter {
            form_ids: vec![holder, item_id],
            plugins: vec![plugin.into()],
            forms: vec![Form::new(
                1,
                0x20,
                save_type::REFR,
                inventory(&[item(2, -9, &[list(&[health(1.5f32.to_bits())])])]),
            )],
            ..SaveWriter::default()
        };
        let mut caller_bytes = writer.build().0;
        let captured = InventorySource::parse(Arc::from(caller_bytes.clone())).unwrap();
        caller_bytes.fill(0);
        assert_ne!(captured.original_bytes(), caller_bytes);
        captured
    }
    fn owned<T: Send + Sync>() {}
    owned::<InventorySource>();
    owned::<ItemEnvelope>();
    let first = make(0x33, 0x01004455, "First.esm");
    let second = make(0x55, 0x02004455, "Second.esm");
    let a = items(&first.records()[0])[0].clone();
    let b = items(&second.records()[0])[0].clone();
    assert_eq!(a.identity().saved(), b.identity().saved());
    assert!(!a.source().save().same_capture(b.source().save()));
    assert!(a.source().save().same_capture(first.context()));
    let bytes = a.original_payload().to_vec();
    drop(first);
    drop(second);
    assert_eq!(
        a.identity().resolution(),
        Resolution::ResolvedInSave(0x01004455)
    );
    assert_eq!(
        b.identity().resolution(),
        Resolution::ResolvedInSave(0x02004455)
    );
    assert_eq!(
        a.source().holder().resolution(),
        Resolution::ResolvedInSave(0x33)
    );
    assert_eq!(
        b.source().holder().resolution(),
        Resolution::ResolvedInSave(0x55)
    );
    assert_eq!(a.source().save().plugins(), ["First.esm"]);
    assert_eq!(b.source().save().plugins(), ["Second.esm"]);
    assert_eq!(a.original_payload(), bytes);
    assert_eq!(a.clone().original_payload(), bytes);
}

#[test]
fn unresolved_identities_are_explicit_and_never_substituted_with_zero() {
    let raw = inventory(&[item(0, 1, &[]), item(3, 2, &[]), item(99, 3, &[])]);
    let writer = SaveWriter {
        form_ids: vec![0x33, 0x44, 0],
        forms: vec![
            Form::new(1, 0x20, save_type::REFR, raw.clone()),
            Form::new(0, 0x20, save_type::ACHR, vec![]),
            Form::new(99, 0x20, save_type::ACHR, vec![]),
        ],
        ..SaveWriter::default()
    };
    let captured = InventorySource::parse(writer.build().0.into()).unwrap();
    let entries = items(&captured.records()[0]);
    assert_eq!(entries[0].identity().resolution(), Resolution::Null);
    assert_eq!(
        entries[1].identity().resolution(),
        Resolution::ResolvedInSave(0)
    );
    assert_eq!(
        entries[2].identity().resolution(),
        Resolution::MissingIndex { index: 99 }
    );
    for record in &captured.records()[1..] {
        assert!(matches!(
            record.state(),
            RecordState::PreservedUndecoded(RecordFailure::UnresolvedHolder(_))
        ));
        assert!(record.source().original_data().is_empty());
    }
}

#[test]
fn metadata_is_captured_from_the_actual_record_and_structural_failures_remain_errors() {
    let raw = inventory(&[item(2, 1, &[])]);
    let mut form = Form::new(1, 0x08000000, save_type::REFR, raw.clone());
    form.version = 26;
    let writer = SaveWriter {
        form_ids: vec![0x33, 0x44],
        forms: vec![Form::new(1, 0, save_type::CELL, vec![]), form],
        ..SaveWriter::default()
    };
    let bytes = writer.build().0;
    let parsed = Save::parse(&bytes).unwrap();
    let cf = &parsed.change_forms[1];
    let header = bytes[cf.offset..cf.data_offset].to_vec();
    let span = cf.data_offset..cf.data_offset + cf.data.len();
    let captured = InventorySource::parse(bytes.into()).unwrap();
    assert_eq!(captured.records().len(), 1);
    let record = captured.records()[0].source();
    assert_eq!(record.ordinal(), 1);
    assert_eq!(record.version(), 26);
    assert_eq!(record.flags(), 0x08000000);
    assert_eq!(record.save_type(), save_type::REFR);
    assert_eq!(record.original_header(), header);
    assert_eq!(record.original_data(), raw);
    assert_eq!(record.data_file_span(), span);
    assert_eq!(captured.context().version(), crate::VERSION);
    assert_eq!(captured.context().minor_version(), crate::MINOR_VERSION);
    assert!(InventorySource::parse(Arc::from([0u8; 12])).is_err());
}

#[test]
fn capture_admission_rejects_invalid_ranges_and_shape_without_exposing_constructors() {
    let captured = source(inventory(&[item(2, 1, &[list(&[count(2)])])]), 0x20);
    let source = &captured.records()[0].source;
    let base = source.data_file_span.start;
    let end = source.data_file_span.end;
    for bad in [base - 1..end, base..end + 1, end..base, base..base] {
        assert!(relative_span(source, bad).is_err());
    }
    for (previous, bad) in [
        (0, 9..12),
        (0, 12..12),
        (0, Range { start: 14, end: 13 }),
        (0, 11..21),
        (15, 14..16),
        (18, 12..14),
    ] {
        assert!(nested_span(&(10..20), previous, &bad).is_err());
    }
    assert!(envelopes(
        source,
        vec![],
        InventoryCapture {
            items: vec![ItemSpans {
                range: base..end,
                stacks: vec![]
            }]
        }
    )
    .is_err());
    let decoded = decode::ItemChange {
        item: RefId(2),
        count: 1,
        stacks: vec![vec![decode::Extra::Count(2)]],
    };
    assert!(envelopes(
        source,
        vec![decoded.clone()],
        InventoryCapture {
            items: vec![ItemSpans {
                range: base + 2..end,
                stacks: vec![]
            }]
        }
    )
    .is_err());
    assert!(envelopes(
        source,
        vec![decoded],
        InventoryCapture {
            items: vec![ItemSpans {
                range: base + 2..end,
                stacks: vec![StackSpans {
                    range: base + 13..end,
                    extras: vec![]
                }]
            }]
        }
    )
    .is_err());
}

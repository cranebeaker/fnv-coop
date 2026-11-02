//! Choices exposed by the face and body menu (`RaceSexMenu`).
//!
//! The filters below follow the traced setup and choice predicates in
//! `docs/FACE_CREATION.md`: playable races use the runtime race flag, while
//! hair and eye records use their playable and sex flags plus the race's
//! corresponding form list. Results follow `LoadOrder::records_of_type`
//! iteration order; native menu ordering has not been established.

use std::collections::HashSet;

use esm::{FormId, FourCC, LoadOrder, Record, RecordRef};

const RACE: FourCC = FourCC::new(b"RACE");
const HAIR: FourCC = FourCC::new(b"HAIR");
const EYES: FourCC = FourCC::new(b"EYES");
const DATA: FourCC = FourCC::new(b"DATA");
const HNAM: FourCC = FourCC::new(b"HNAM");
const ENAM: FourCC = FourCC::new(b"ENAM");
const FULL: FourCC = FourCC::new(b"FULL");
const DNAM: FourCC = FourCC::new(b"DNAM");

/// A selectable race, hairstyle, or eye set. `name` comes only from `FULL`;
/// it remains `None` when the record has no such subrecord and `Some("")`
/// when the game record explicitly has an empty name.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Choice {
    pub form: FormId,
    pub name: Option<String>,
}

/// Hair/eyes retained or replaced when RaceSexMenu changes race or sex.
/// This does not apply morphs or rebuild an actor's preview.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct PartSelection {
    pub hair: Option<FormId>,
    pub eyes: Option<FormId>,
}

/// Reconcile parts through `007b1ca0`, separately from the visible choices.
/// Existing members need sex compatibility, not the playable bit. Invalid
/// hair uses the race's sex-specific DNAM, then (only if absent) its ordered
/// HNAM list. Invalid eyes use the first ENAM entry without flag filtering.
/// See `docs/FACE_CREATION.md` for addresses and unresolved menu integration.
/// Invalid list references are omitted as in the loader. An unresolved DNAM
/// default produces no hair; its native fixup behavior is not claimed here.
/// Invalid races and malformed DNAM are unsupported and return `None`.
pub fn reconcile_parts(
    order: &LoadOrder,
    race: FormId,
    female: bool,
    current: PartSelection,
) -> Option<PartSelection> {
    let rr = order
        .get(race)
        .filter(|rr| rr.entry.header.kind == RACE && !rr.entry.header.is_deleted())?;
    let record = rr.record().ok()?;
    if record.get_all(DNAM).any(|sub| sub.data.len() != 8) {
        // 00610cd0 requests eight bytes; truncated/oversized subrecords need
        // the lower-level reader traced before assigning fallback behavior.
        return None;
    }
    let hair_members = ordered_forms(order, rr, &record, HNAM, HAIR);
    let eye_members = ordered_forms(order, rr, &record, ENAM, EYES);
    let retained = |part: Option<FormId>, kind, members: &[FormId]| {
        part.filter(|id| {
            members.contains(id)
                && part_flags(order, *id, kind).is_some_and(|flags| sex_allowed(flags, female))
        })
    };
    let hair = retained(current.hair, HAIR, &hair_members).or_else(|| {
        let offset = usize::from(female) * 4;
        let default = record
            .get_all(DNAM)
            .last()
            .map(|sub| u32::from_le_bytes(sub.data[offset..offset + 4].try_into().unwrap()))
            .unwrap_or(0);
        if default != 0 {
            // The native DNAM branch does not revalidate membership or flags.
            return existing_part(order, rr.plugin.to_global(FormId(default)), HAIR);
        }
        hair_members.into_iter().find(|id| {
            part_flags(order, *id, HAIR)
                .is_some_and(|flags| flags & 1 != 0 && sex_allowed(flags, female))
        })
    });
    let eyes = retained(current.eyes, EYES, &eye_members).or_else(|| {
        eye_members
            .first()
            .and_then(|id| existing_part(order, *id, EYES))
    });
    Some(PartSelection { hair, eyes })
}

fn existing_part(order: &LoadOrder, id: FormId, kind: FourCC) -> Option<FormId> {
    (id.0 != 0
        && order
            .get(id)
            .is_some_and(|rr| rr.entry.header.kind == kind && !rr.entry.header.is_deleted()))
    .then_some(id)
}

fn part_flags(order: &LoadOrder, id: FormId, kind: FourCC) -> Option<u8> {
    existing_part(order, id, kind)?;
    let record = order.get(id)?.record().ok()?;
    record.get(DATA)?.data.first().copied()
}

fn sex_allowed(flags: u8, female: bool) -> bool {
    flags & if female { 4 } else { 2 } == 0
}

fn ordered_forms(
    order: &LoadOrder,
    rr: RecordRef<'_>,
    record: &Record,
    kind: FourCC,
    part_type: FourCC,
) -> Vec<FormId> {
    let mut seen = HashSet::new();
    record
        .get_all(kind)
        .filter(|sub| !sub.data.is_empty() && sub.data.len() % 4 == 0)
        .flat_map(|sub| {
            sub.data.chunks_exact(4).map(|bytes| {
                let raw = FormId(u32::from_le_bytes(bytes.try_into().unwrap()));
                if raw.0 == 0 {
                    raw
                } else {
                    rr.plugin.to_global(raw)
                }
            })
        })
        // 00610cd0 omits unresolved/wrong-type entries; 00613810/00613910
        // append only the first instance of each resolved form.
        .filter(|id| existing_part(order, *id, part_type).is_some() && seen.insert(*id))
        .collect()
}

/// Races whose `DATA` runtime flags at byte 32 have bit 0 set:
/// setup007acb60 calls0059f610; loader00610cd0 copies DATA to race+0x50.
pub fn races(order: &LoadOrder) -> Vec<Choice> {
    order
        .records_of_type(RACE)
        .filter(|rr| !rr.entry.header.is_deleted())
        .filter_map(|rr| {
            let record = rr.record().ok()?;
            let data = record.get(DATA)?.data.get(32..36)?;
            (u32::from_le_bytes(data.try_into().ok()?) & 1 != 0).then(|| choice(rr, &record))
        })
        .collect()
}

/// Playable hair permitted by the selected race and sex (`false` male,
/// `true` female). The race's `HNAM` list is read in the race record's
/// owning plugin, so references from overrides and reordered masters map
/// to load-order form IDs correctly.
/// Menu007af300 uses005fdf40/005fdfa0; loader005fdcb0 reads DATA into+0x48.
pub fn hair(order: &LoadOrder, race: FormId, female: bool) -> Vec<Choice> {
    choices_for_parts(order, race, female, HAIR, HNAM)
}

/// Playable eye sets permitted by the selected race and sex (`false` male,
/// `true` female). The race's `ENAM` list is read in the race record's
/// owning plugin, so references from overrides and reordered masters map
/// to load-order form IDs correctly.
/// Menu007af450 uses005fc4d0/005fc5f0; loader005fc220 reads DATA into+0x30.
pub fn eyes(order: &LoadOrder, race: FormId, female: bool) -> Vec<Choice> {
    choices_for_parts(order, race, female, EYES, ENAM)
}

fn choices_for_parts(
    order: &LoadOrder,
    race: FormId,
    female: bool,
    part_type: FourCC,
    list_type: FourCC,
) -> Vec<Choice> {
    let Some(race_ref) = order
        .get(race)
        .filter(|rr| rr.entry.header.kind == RACE && !rr.entry.header.is_deleted())
    else {
        return Vec::new();
    };
    let Ok(race_record) = race_ref.record() else {
        return Vec::new();
    };
    let members = listed_forms(race_ref, &race_record, list_type);
    if members.is_empty() {
        return Vec::new();
    }

    order
        .records_of_type(part_type)
        .filter(|rr| !rr.entry.header.is_deleted())
        .filter_map(|rr| {
            let record = rr.record().ok()?;
            let flags = *record.get(DATA)?.data.first()?;
            let playable = flags & 1 != 0;
            let male_allowed = flags & 2 == 0;
            let female_allowed = flags & 4 == 0;
            let sex_allowed = if female { female_allowed } else { male_allowed };
            (playable && sex_allowed && members.contains(&rr.form_id)).then(|| choice(rr, &record))
        })
        .collect()
}

fn listed_forms(rr: RecordRef<'_>, record: &Record, kind: FourCC) -> HashSet<FormId> {
    record
        .get_all(kind)
        .filter(|sub| !sub.data.is_empty() && sub.data.len() % 4 == 0)
        .flat_map(|sub| {
            sub.data.chunks_exact(4).filter_map(|bytes| {
                let raw = u32::from_le_bytes(bytes.try_into().ok()?);
                (raw != 0).then(|| rr.plugin.to_global(FormId(raw)))
            })
        })
        .collect()
}

fn choice(rr: RecordRef<'_>, record: &Record) -> Choice {
    Choice {
        form: rr.form_id,
        name: record.get(FULL).map(|sub| sub.zstring()),
    }
}

#[cfg(test)]
mod tests {
    use esm::{FormId, LoadOrder, Plugin};
    use testdata::{group, record, sub, zstr};

    use super::{eyes, hair, races, reconcile_parts, PartSelection};

    fn plugin(masters: &[&str], groups: &[(&[u8; 4], Vec<u8>)]) -> Plugin {
        let mut header = 1.34f32.to_le_bytes().to_vec();
        header.extend([0; 8]);
        let mut header_data = sub(b"HEDR", &header);
        for master in masters {
            header_data.extend(sub(b"MAST", &zstr(master)));
            header_data.extend(sub(b"DATA", &[0; 8]));
        }
        let mut bytes = record(b"TES4", 0, &header_data);
        for (kind, contents) in groups {
            bytes.extend(group(**kind, 0, contents));
        }
        Plugin::from_bytes(bytes).unwrap()
    }

    fn deleted(mut bytes: Vec<u8>) -> Vec<u8> {
        let flags = u32::from_le_bytes(bytes[8..12].try_into().unwrap()) | 0x20;
        bytes[8..12].copy_from_slice(&flags.to_le_bytes());
        bytes
    }

    fn race(id: u32, runtime_flags: u32, hair_ids: &[u32], eye_ids: &[u32]) -> Vec<u8> {
        race_defaults(id, runtime_flags, hair_ids, eye_ids, None)
    }

    fn race_defaults(
        id: u32,
        runtime_flags: u32,
        hair_ids: &[u32],
        eye_ids: &[u32],
        defaults: Option<[u32; 2]>,
    ) -> Vec<u8> {
        let mut data = vec![0; 36];
        data[32..36].copy_from_slice(&runtime_flags.to_le_bytes());
        let mut fields = sub(b"FULL", &zstr("Race"));
        fields.extend(sub(b"DATA", &data));
        if !hair_ids.is_empty() {
            let ids: Vec<u8> = hair_ids.iter().flat_map(|id| id.to_le_bytes()).collect();
            fields.extend(sub(b"HNAM", &ids));
        }
        if !eye_ids.is_empty() {
            let ids: Vec<u8> = eye_ids.iter().flat_map(|id| id.to_le_bytes()).collect();
            fields.extend(sub(b"ENAM", &ids));
        }
        if let Some(defaults) = defaults {
            fields.extend(sub(
                b"DNAM",
                &defaults
                    .into_iter()
                    .flat_map(u32::to_le_bytes)
                    .collect::<Vec<_>>(),
            ));
        }
        record(b"RACE", id, &fields)
    }

    fn part(kind: &[u8; 4], id: u32, flags: Option<u8>, name: Option<&str>) -> Vec<u8> {
        let mut fields = Vec::new();
        if let Some(name) = name {
            fields.extend(sub(b"FULL", &zstr(name)));
        }
        if let Some(flags) = flags {
            fields.extend(sub(b"DATA", &[flags]));
        }
        record(kind, id, &fields)
    }

    fn one_plugin(records: &[(&[u8; 4], Vec<u8>)]) -> LoadOrder {
        let groups: Vec<_> = [b"RACE", b"HAIR", b"EYES", b"ARMO"]
            .iter()
            .map(|kind| {
                let contents = records
                    .iter()
                    .filter(|(record_type, _)| *record_type == *kind)
                    .flat_map(|(_, bytes)| bytes.iter().copied())
                    .collect();
                (*kind, contents)
            })
            .collect();
        LoadOrder::single("Test.esm", None, plugin(&[], &groups)).unwrap()
    }

    #[test]
    fn reconciliation_retains_nonplayable_members_and_uses_sex_specific_defaults() {
        let order = one_plugin(&[
            (
                b"RACE",
                race_defaults(0x800, 1, &[0x810], &[0x820, 0x821], Some([0x811, 0x812])),
            ),
            (b"HAIR", part(b"HAIR", 0x810, Some(4), None)), // male, not playable
            (b"HAIR", part(b"HAIR", 0x811, Some(0), None)),
            (b"HAIR", part(b"HAIR", 0x812, Some(7), None)), // default deliberately not selectable
            (b"EYES", part(b"EYES", 0x820, Some(7), None)), // first fallback ignores flags
            (b"EYES", part(b"EYES", 0x821, Some(4), None)), // male, not playable
        ]);
        let current = PartSelection {
            hair: Some(FormId(0x810)),
            eyes: Some(FormId(0x821)),
        };
        assert_eq!(
            reconcile_parts(&order, FormId(0x800), false, current),
            Some(current)
        );
        assert!(hair(&order, FormId(0x800), false).is_empty());
        assert!(eyes(&order, FormId(0x800), false).is_empty());
        assert_eq!(
            reconcile_parts(&order, FormId(0x800), true, current),
            Some(PartSelection {
                hair: Some(FormId(0x812)),
                eyes: Some(FormId(0x820)),
            })
        );
        assert_eq!(
            reconcile_parts(&order, FormId(0x800), false, PartSelection::default())
                .unwrap()
                .hair,
            Some(FormId(0x811))
        );
    }

    #[test]
    fn absent_default_scans_race_hair_order_instead_of_global_choice_order() {
        let order = one_plugin(&[
            (
                b"RACE",
                race(0x800, 1, &[0x812, 0x813, 0x811, 0x810], &[0x821, 0x820]),
            ),
            (b"HAIR", part(b"HAIR", 0x810, Some(1), None)),
            (b"HAIR", part(b"HAIR", 0x811, Some(1), None)),
            (b"HAIR", part(b"HAIR", 0x812, Some(0), None)),
            (b"HAIR", part(b"HAIR", 0x813, Some(3), None)),
            (b"EYES", part(b"EYES", 0x820, Some(1), None)),
            (b"EYES", part(b"EYES", 0x821, Some(0), None)),
        ]);
        assert_eq!(hair(&order, FormId(0x800), false)[0].form, FormId(0x810));
        assert_eq!(
            reconcile_parts(&order, FormId(0x800), false, PartSelection::default()),
            Some(PartSelection {
                hair: Some(FormId(0x811)),
                eyes: Some(FormId(0x821)),
            })
        );
        assert_eq!(
            reconcile_parts(&order, FormId(0x800), true, PartSelection::default())
                .unwrap()
                .hair,
            Some(FormId(0x813))
        );
    }

    #[test]
    fn unresolved_list_entries_are_omitted_but_bad_default_does_not_invent_hair() {
        let order = one_plugin(&[
            (
                b"RACE",
                race_defaults(0x800, 1, &[0x810], &[0x999, 0x820], Some([0x999, 0x999])),
            ),
            (b"RACE", race(0x801, 1, &[0, 0x810], &[0, 0x820])),
            (b"RACE", deleted(race(0x802, 1, &[0x810], &[0x820]))),
            (b"HAIR", part(b"HAIR", 0x810, Some(1), None)),
            (b"EYES", part(b"EYES", 0x820, Some(1), None)),
        ]);
        assert_eq!(
            reconcile_parts(&order, FormId(0x800), false, PartSelection::default()),
            Some(PartSelection {
                hair: None,
                eyes: Some(FormId(0x820))
            })
        );
        assert_eq!(
            reconcile_parts(&order, FormId(0x801), false, PartSelection::default()),
            Some(PartSelection {
                hair: Some(FormId(0x810)),
                eyes: Some(FormId(0x820))
            })
        );
        for race in [0x802, 0x999, 0x810] {
            assert_eq!(
                reconcile_parts(&order, FormId(race), false, PartSelection::default()),
                None
            );
        }
    }

    #[test]
    fn repeated_valid_default_fields_use_the_last_pair() {
        let mut fields = sub(
            b"DNAM",
            &[0x810u32.to_le_bytes(), 0x810u32.to_le_bytes()].concat(),
        );
        fields.extend(sub(
            b"DNAM",
            &[0x811u32.to_le_bytes(), 0x812u32.to_le_bytes()].concat(),
        ));
        let order = one_plugin(&[
            (b"RACE", record(b"RACE", 0x800, &fields)),
            (b"HAIR", part(b"HAIR", 0x810, Some(1), None)),
            (b"HAIR", part(b"HAIR", 0x811, Some(1), None)),
            (b"HAIR", part(b"HAIR", 0x812, Some(1), None)),
        ]);
        for (female, expected) in [(false, 0x811), (true, 0x812)] {
            assert_eq!(
                reconcile_parts(&order, FormId(0x800), female, PartSelection::default())
                    .unwrap()
                    .hair,
                Some(FormId(expected))
            );
        }
    }

    #[test]
    fn malformed_defaults_are_explicitly_unsupported() {
        for bytes in [vec![0; 4], vec![0; 9]] {
            let order = one_plugin(&[(b"RACE", record(b"RACE", 0x800, &sub(b"DNAM", &bytes)))]);
            assert_eq!(
                reconcile_parts(&order, FormId(0x800), false, PartSelection::default()),
                None
            );
        }
    }

    #[test]
    fn reconciled_defaults_and_ordered_lists_use_the_race_owner_mapping() {
        let a = plugin(&[], &[(b"RACE", race(0x800, 1, &[], &[]))]);
        let b = plugin(
            &[],
            &[
                (
                    b"HAIR",
                    [
                        part(b"HAIR", 0x810, Some(1), None),
                        part(b"HAIR", 0x811, Some(1), None),
                    ]
                    .concat(),
                ),
                (b"EYES", part(b"EYES", 0x820, Some(0), None)),
            ],
        );
        let patch = plugin(
            &["B.esm", "A.esm"],
            &[(
                b"RACE",
                race_defaults(0x0100_0800, 1, &[0x811, 0x810], &[0x820], Some([0x810, 0])),
            )],
        );
        let order = LoadOrder::from_plugins(vec![
            ("A.esm".into(), None, a),
            ("B.esm".into(), None, b),
            ("Patch.esp".into(), None, patch),
        ])
        .unwrap();
        for (female, hair) in [(false, 0x0100_0810), (true, 0x0100_0811)] {
            assert_eq!(
                reconcile_parts(&order, FormId(0x800), female, PartSelection::default()),
                Some(PartSelection {
                    hair: Some(FormId(hair)),
                    eyes: Some(FormId(0x0100_0820))
                })
            );
        }
    }

    #[test]
    fn playable_races_need_the_runtime_flag_and_complete_data() {
        let mut short = sub(b"FULL", &zstr("Short"));
        short.extend(sub(b"DATA", &[0; 35]));
        let records = [
            (b"RACE", race(0x800, 1, &[], &[])),
            (b"RACE", race(0x801, 0, &[], &[])),
            (b"RACE", record(b"RACE", 0x802, &short)),
            (b"RACE", record(b"RACE", 0x803, &sub(b"DATA", &[0; 36]))),
        ];
        let order = one_plugin(&records);
        assert_eq!(
            races(&order).iter().map(|c| c.form).collect::<Vec<_>>(),
            [FormId(0x800)]
        );
    }

    #[test]
    fn hair_and_eyes_obey_playable_sex_and_membership_flags() {
        let race_record = race(
            0x800,
            1,
            &[0x810, 0x811, 0x812, 0x813, 0x814],
            &[0x820, 0x821, 0x823, 0x824],
        );
        let order = one_plugin(&[
            (b"RACE", race_record),
            (b"HAIR", part(b"HAIR", 0x810, Some(1), Some("Both"))),
            (b"HAIR", part(b"HAIR", 0x811, Some(3), Some("Female only"))),
            (b"HAIR", part(b"HAIR", 0x812, Some(5), Some("Male only"))),
            (b"HAIR", part(b"HAIR", 0x813, Some(0), Some("Not playable"))),
            (b"HAIR", part(b"HAIR", 0x814, Some(7), Some("Neither"))),
            (b"HAIR", part(b"HAIR", 0x815, Some(1), Some("Not listed"))),
            (b"EYES", part(b"EYES", 0x820, Some(1), Some("Both"))),
            (b"EYES", part(b"EYES", 0x821, Some(5), Some("Male only"))),
            (b"EYES", part(b"EYES", 0x823, Some(3), Some("Female only"))),
            (b"EYES", part(b"EYES", 0x824, Some(7), Some("Neither"))),
            (b"EYES", part(b"EYES", 0x822, Some(1), Some("Not listed"))),
            (b"ARMO", part(b"ARMO", 0x816, Some(1), Some("Wrong type"))),
        ]);
        let hair_male = hair(&order, FormId(0x800), false);
        let hair_female = hair(&order, FormId(0x800), true);
        assert_eq!(
            hair_male
                .iter()
                .map(|c| c.name.as_deref())
                .collect::<Vec<_>>(),
            [Some("Both"), Some("Male only")]
        );
        assert_eq!(
            hair_female
                .iter()
                .map(|c| c.name.as_deref())
                .collect::<Vec<_>>(),
            [Some("Both"), Some("Female only")]
        );
        assert_eq!(
            eyes(&order, FormId(0x800), false)
                .iter()
                .map(|c| c.name.as_deref())
                .collect::<Vec<_>>(),
            [Some("Both"), Some("Male only")]
        );
        assert_eq!(
            eyes(&order, FormId(0x800), true)
                .iter()
                .map(|c| c.name.as_deref())
                .collect::<Vec<_>>(),
            [Some("Both"), Some("Female only")]
        );
        assert!(hair(&order, FormId(0x999), false).is_empty());
        assert!(hair(&order, FormId(0x816), false).is_empty());
    }

    #[test]
    fn malformed_race_lists_and_missing_or_wrong_type_forms_are_ignored() {
        let mut fields = sub(b"DATA", &{
            let mut d = vec![0; 36];
            d[32..36].copy_from_slice(&1u32.to_le_bytes());
            d
        });
        fields.extend(sub(b"HNAM", &[0x10, 0x08, 0x00])); // malformed list
        fields.extend(sub(b"HNAM", &[0x11, 0x08, 0x00, 0x00, 0xff])); // valid ID plus bad tail
        fields.extend(sub(b"HNAM", &[0x10, 0x08, 0x00, 0x00])); // valid-size list
        fields.extend(sub(b"HNAM", &[0x12, 0x08, 0x00, 0x00])); // missing form
        let malformed_race = record(b"RACE", 0x800, &fields);
        let armo = part(b"ARMO", 0x810, Some(1), Some("Wrong type"));
        let order = one_plugin(&[
            (b"RACE", malformed_race),
            (
                b"HAIR",
                part(b"HAIR", 0x811, Some(1), Some("Missing from list")),
            ),
            (b"ARMO", armo),
        ]);
        assert!(hair(&order, FormId(0x800), false).is_empty());
        assert!(hair(&order, FormId(0x810), false).is_empty());
    }

    #[test]
    fn overridden_race_lists_map_references_through_the_owner_plugin() {
        let master_a = plugin(
            &[],
            &[
                (b"RACE", race(0x800, 1, &[0x801], &[])),
                (b"HAIR", part(b"HAIR", 0x801, Some(1), Some("Old"))),
            ],
        );
        let master_b = plugin(
            &[],
            &[(b"HAIR", part(b"HAIR", 0x820, Some(1), Some("Remapped")))],
        );
        // Patch masters are deliberately reversed: local index 0 maps to B,
        // while local index 1 maps to A. The race override lives in A.
        let patch = plugin(
            &["B.esm", "A.esm"],
            &[(
                b"RACE",
                race(
                    0x0100_0800,
                    1,
                    &[0x0000_0820, 0x0000_0999, 0x0000_0830],
                    &[],
                ),
            )],
        );
        let order = LoadOrder::from_plugins(vec![
            ("A.esm".into(), None, master_a),
            ("B.esm".into(), None, master_b),
            ("Patch.esp".into(), None, patch),
        ])
        .unwrap();
        let choices = hair(&order, FormId(0x800), false);
        assert_eq!(choices.len(), 1);
        assert_eq!(choices[0].form, FormId(0x0100_0820));
        assert_eq!(choices[0].name.as_deref(), Some("Remapped"));
    }

    #[test]
    fn deleted_winning_race_hair_and_eye_overrides_are_not_choices() {
        let master = plugin(
            &[],
            &[
                (
                    b"RACE",
                    [
                        race(0x800, 1, &[0x810], &[0x820]),
                        race(0x801, 1, &[0x810], &[0x820]),
                    ]
                    .concat(),
                ),
                (b"HAIR", part(b"HAIR", 0x810, Some(1), Some("Master hair"))),
                (b"EYES", part(b"EYES", 0x820, Some(1), Some("Master eyes"))),
            ],
        );
        let patch = plugin(
            &["Master.esm"],
            &[
                (b"RACE", deleted(race(0x0000_0800, 1, &[0x810], &[0x820]))),
                (
                    b"HAIR",
                    deleted(part(b"HAIR", 0x0000_0810, Some(1), Some("Deleted hair"))),
                ),
                (
                    b"EYES",
                    deleted(part(b"EYES", 0x0000_0820, Some(1), Some("Deleted eyes"))),
                ),
            ],
        );
        let order = LoadOrder::from_plugins(vec![
            ("Master.esm".into(), None, master),
            ("Patch.esp".into(), None, patch),
        ])
        .unwrap();
        assert_eq!(
            races(&order).iter().map(|c| c.form).collect::<Vec<_>>(),
            [FormId(0x801)]
        );
        assert!(hair(&order, FormId(0x800), false).is_empty());
        assert!(eyes(&order, FormId(0x800), false).is_empty());
        // The surviving race must not resurrect deleted parts from its lists.
        assert!(hair(&order, FormId(0x801), false).is_empty());
        assert!(eyes(&order, FormId(0x801), false).is_empty());
    }

    #[test]
    fn missing_and_empty_full_names_are_not_invented() {
        // The empty FULL is represented by the subrecord's terminating NUL.
        let mut race_fields = sub(b"DATA", &{
            let mut d = vec![0; 36];
            d[32..36].copy_from_slice(&1u32.to_le_bytes());
            d
        });
        race_fields.extend(sub(b"HNAM", &0x810u32.to_le_bytes()));
        race_fields.extend(sub(b"HNAM", &0x811u32.to_le_bytes()));
        let order = one_plugin(&[
            (b"RACE", record(b"RACE", 0x800, &race_fields)),
            (b"HAIR", part(b"HAIR", 0x810, Some(1), None)),
            (b"HAIR", part(b"HAIR", 0x811, Some(1), Some(""))),
        ]);
        let choices = hair(&order, FormId(0x800), false);
        assert_eq!(choices[0].name, None);
        assert_eq!(choices[1].name.as_deref(), Some(""));
    }
}

//! A quest's whole story state as one replicated field (docs/coop/DECISIONS.md D2): the current
//! stage, the quest data flags and the done stages travel together, so no delta can deliver one
//! without the others. `Field::Custom { tag: QUEST_RECORD_TAG, data }`: the stage, the flags, a
//! 256-bit set of done stages (stage numbers are bytes in the game, `TESQuestStage::ucIndex`; bit
//! `n % 8` of byte `n / 8`), then the objectives: a count and, for each in list order, its number
//! (4 bytes, little-endian) and its state (`QUEST_OBJECTIVE_STATE`, 0 to 3, one byte). A quest
//! script's variables travel as `Field::ScriptVar` beside it.

use coop_core::schema::Field;
use coop_engine::quest::QuestRecord;

/// The tag of a quest record (also its sub-index: one per quest).
pub const QUEST_RECORD_TAG: u16 = 0x5152;

/// The length of the fixed part (stage, flags, done set, objective count).
pub const LEN: usize = 2 + 32 + 1;

/// Bytes per objective.
const OBJECTIVE_LEN: usize = 5;

/// Most objectives a record carries within coop-core's custom field limit.
pub const MAX_OBJECTIVES: usize = (coop_core::schema::MAX_CUSTOM_BYTES - LEN) / OBJECTIVE_LEN;

/// The record as its field, or `None` when it cannot be carried exactly (more than
/// [`MAX_OBJECTIVES`] objectives, an objective state above 255): the host then sends nothing
/// rather than something wrong.
pub fn encode(r: &QuestRecord) -> Option<Field> {
    if r.objectives.len() > MAX_OBJECTIVES {
        return None;
    }
    let mut data = vec![0u8; LEN];
    data[0] = r.stage;
    data[1] = r.flags;
    for n in &r.done {
        data[2 + usize::from(*n / 8)] |= 1 << (n % 8);
    }
    data[LEN - 1] = r.objectives.len() as u8;
    for &(n, state) in &r.objectives {
        data.extend_from_slice(&n.to_le_bytes());
        data.push(u8::try_from(state).ok()?);
    }
    Some(Field::Custom {
        tag: QUEST_RECORD_TAG,
        data,
    })
}

/// The quest `form`'s record from its field, or `None` when the field is not a well-formed quest
/// record. Done stages come back in ascending order.
pub fn decode(form: u32, f: &Field) -> Option<QuestRecord> {
    let Field::Custom { tag, data } = f else {
        return None;
    };
    if *tag != QUEST_RECORD_TAG || data.len() < LEN {
        return None;
    }
    let count = usize::from(data[LEN - 1]);
    if data.len() != LEN + count * OBJECTIVE_LEN {
        return None;
    }
    let done = (0..=255u8)
        .filter(|n| data[2 + usize::from(*n / 8)] & (1 << (n % 8)) != 0)
        .collect();
    let objectives = data[LEN..]
        .chunks_exact(OBJECTIVE_LEN)
        .map(|c| {
            (
                u32::from_le_bytes([c[0], c[1], c[2], c[3]]),
                u32::from(c[4]),
            )
        })
        .collect();
    Some(QuestRecord {
        form,
        stage: data[0],
        flags: data[1],
        done,
        objectives,
    })
}

/// The same record with its done stages in ascending order (the order [`decode`] gives).
pub fn normalized(r: &QuestRecord) -> QuestRecord {
    let mut done = r.done.clone();
    done.sort_unstable();
    done.dedup();
    QuestRecord { done, ..r.clone() }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn rec(stage: u8, flags: u8, done: &[u8]) -> QuestRecord {
        QuestRecord {
            form: 0x0010_4c0c,
            stage,
            flags,
            done: done.to_vec(),
            objectives: vec![],
        }
    }

    #[test]
    fn a_record_round_trips_with_its_done_stages_sorted() {
        for r in [
            rec(0, 0, &[]),
            rec(255, 0xff, &[255, 0, 7, 8, 128]),
            rec(20, 1, &[10, 5, 20]),
            QuestRecord {
                objectives: vec![(10, 1), (20, 3), (0xffff_ffff, 0)],
                ..rec(20, 1, &[10])
            },
        ] {
            let f = encode(&r).unwrap();
            assert!(f.is_sane());
            assert_eq!(decode(r.form, &f), Some(normalized(&r)));
        }
    }

    #[test]
    fn every_stage_number_has_its_own_bit() {
        let all: Vec<u8> = (0..=255).collect();
        let r = rec(0, 0, &all);
        assert_eq!(decode(r.form, &encode(&r).unwrap()).unwrap().done, all);
    }

    #[test]
    fn a_record_that_cannot_be_carried_exactly_is_not_encoded() {
        let many = QuestRecord {
            objectives: (0..=MAX_OBJECTIVES as u32).map(|n| (n, 1)).collect(),
            ..rec(0, 0, &[])
        };
        assert_eq!(encode(&many), None);
        let fits = QuestRecord {
            objectives: (0..MAX_OBJECTIVES as u32).map(|n| (n, 1)).collect(),
            ..rec(0, 0, &[])
        };
        assert!(encode(&fits).unwrap().is_sane());
        let odd_state = QuestRecord {
            objectives: vec![(1, 256)],
            ..rec(0, 0, &[])
        };
        assert_eq!(encode(&odd_state), None);
    }

    #[test]
    fn malformed_fields_are_refused() {
        let good = encode(&QuestRecord {
            objectives: vec![(1, 1)],
            ..rec(1, 1, &[1])
        })
        .unwrap();
        let Field::Custom { data, .. } = good else {
            unreachable!()
        };
        let mut wrong_count = data.clone();
        wrong_count[LEN - 1] = 2;
        for f in [
            Field::Custom {
                tag: QUEST_RECORD_TAG,
                data: data[..33].to_vec(),
            },
            Field::Custom {
                tag: QUEST_RECORD_TAG,
                data: data[..data.len() - 1].to_vec(),
            },
            Field::Custom {
                tag: QUEST_RECORD_TAG,
                data: wrong_count,
            },
            Field::Custom {
                tag: QUEST_RECORD_TAG + 1,
                data: data.clone(),
            },
            Field::Global { bits: 0 },
        ] {
            assert_eq!(decode(1, &f), None);
        }
    }
}

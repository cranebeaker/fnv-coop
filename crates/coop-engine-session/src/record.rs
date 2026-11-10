//! A quest's whole story state as one replicated field (docs/coop/DECISIONS.md D2): the current
//! stage, the quest data flags and the done stages travel together, so no delta can deliver one
//! without the others. `Field::Custom { tag: QUEST_RECORD_TAG, data }`: the stage, the flags, a
//! 256-bit set of done stages (stage numbers are bytes in the game, `TESQuestStage::ucIndex`; bit
//! `n % 8` of byte `n / 8`), then the objectives: a count and, for each in list order, its number
//! (variable length, one byte below 128) and its state (`QUEST_OBJECTIVE_STATE`, 0 to 3, one byte). A quest
//! script's variables travel as `Field::ScriptVar` beside it.

use coop_core::schema::Field;
use coop_engine::quest::QuestRecord;

/// The tag of a quest record (also its sub-index: one per quest).
pub const QUEST_RECORD_TAG: u16 = 0x5152;

/// The encoding's version, part of what two games must agree on at the handshake (2: objective
/// numbers of variable length).
pub const VERSION: u32 = 2;

/// The length of the fixed part (stage, flags, done set, objective count).
pub const LEN: usize = 2 + 32 + 1;

/// Most bytes a record may take (coop-core's custom field limit).
pub const MAX_LEN: usize = coop_core::schema::MAX_CUSTOM_BYTES;

/// Appends `n` as a variable-length number: seven bits a byte, low bits first, the top bit set on
/// every byte but the last. Objective numbers are small in the game's data (one byte each below
/// 128), so a quest with a hundred objectives fits.
fn put_number(out: &mut Vec<u8>, mut n: u32) {
    while n >= 0x80 {
        out.push(n as u8 | 0x80);
        n >>= 7;
    }
    out.push(n as u8);
}

/// Reads a number [`put_number`] wrote from the front of `data`; `None` when it is cut short, longer
/// than a `u32` needs, or not in its shortest form.
fn take_number(data: &mut &[u8]) -> Option<u32> {
    let mut n = 0u32;
    for i in 0..5 {
        let (&b, rest) = data.split_first()?;
        *data = rest;
        let bits = u32::from(b & 0x7f);
        if i == 4 && bits > 0x0f {
            return None;
        }
        n |= bits << (7 * i);
        if b & 0x80 == 0 {
            // The shortest form only: no zero byte after the first.
            return (i == 0 || bits != 0).then_some(n);
        }
    }
    None
}

/// The record as its field, or `None` when it cannot be carried exactly (more than 255
/// objectives, an objective state above 255, or more than [`MAX_LEN`] bytes): the host then sends
/// nothing rather than something wrong, and says so.
pub fn encode(r: &QuestRecord) -> Option<Field> {
    let mut data = vec![0u8; LEN];
    data[0] = r.stage;
    data[1] = r.flags;
    for n in &r.done {
        data[2 + usize::from(*n / 8)] |= 1 << (n % 8);
    }
    data[LEN - 1] = u8::try_from(r.objectives.len()).ok()?;
    for &(n, state) in &r.objectives {
        put_number(&mut data, n);
        data.push(u8::try_from(state).ok()?);
    }
    (data.len() <= MAX_LEN).then_some(Field::Custom {
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
    let done = (0..=255u8)
        .filter(|n| data[2 + usize::from(*n / 8)] & (1 << (n % 8)) != 0)
        .collect();
    let mut rest = &data[LEN..];
    let mut objectives = Vec::with_capacity(count);
    for _ in 0..count {
        let n = take_number(&mut rest)?;
        let (&state, after) = rest.split_first()?;
        rest = after;
        objectives.push((n, u32::from(state)));
    }
    if !rest.is_empty() {
        return None;
    }
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
    fn a_quest_with_a_hundred_objectives_fits() {
        // Dead Money's 0100f3f7 has 108 objectives (seen on a PC, 2026-11-10).
        let r = QuestRecord {
            objectives: (0..108).map(|n| (n * 5, n % 4)).collect(),
            ..rec(30, 1, &[10, 30])
        };
        let f = encode(&r).unwrap();
        assert!(f.is_sane());
        assert_eq!(decode(r.form, &f), Some(r));
    }

    #[test]
    fn numbers_round_trip_in_their_shortest_form_only() {
        for n in [0, 1, 127, 128, 300, 0x3fff, 0x4000, 0xffff_ffff] {
            let mut b = Vec::new();
            put_number(&mut b, n);
            let mut s = &b[..];
            assert_eq!(take_number(&mut s), Some(n));
            assert!(s.is_empty());
        }
        for bad in [
            &[0x80u8][..],
            &[0x80, 0x00],
            &[0xff, 0xff, 0xff, 0xff, 0x1f],
            &[],
        ] {
            assert_eq!(take_number(&mut &bad[..]), None, "{bad:?}");
        }
    }

    #[test]
    fn a_record_that_cannot_be_carried_exactly_is_not_encoded() {
        let many = QuestRecord {
            objectives: (0..256).map(|n| (n, 1)).collect(),
            ..rec(0, 0, &[])
        };
        assert_eq!(encode(&many), None);
        let too_long = QuestRecord {
            objectives: (0..200).map(|n| (0x1000_0000 + n, 1)).collect(),
            ..rec(0, 0, &[])
        };
        assert_eq!(encode(&too_long), None);
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

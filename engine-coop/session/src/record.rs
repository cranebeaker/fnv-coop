//! A quest's whole story state as one replicated field.
//!
//! The stage, the flags, the done stages and the objectives travel together, so no delta can
//! deliver one without the others: coop-core cuts a delta after a number of keys, and with each
//! part as its own key a guest could hold a stage without its done stages for a step, a state
//! the host never had.
//!
//! `Field::Custom { tag: QUEST_RECORD_TAG, data }`: a presence byte (bit 0: a stage is set),
//! the stage, the shared flag bits (`coop_core::schema::flag`), a 256-bit set of done stages
//! (stage numbers are bytes in the game; bit `n % 8` of byte `n / 8`), then the objectives: a
//! count and, for each in rising order of its number, the number (variable length, one byte
//! below 128) and its status (bit 0 displayed, bit 1 completed). A quest script's variables
//! travel as `Field::ScriptVar` beside it.
//!
//! The record is made from the adapter's own shared fields of a quest
//! ([`coop_engine::game::EngineCoop::fields_of`]), and gives them back.

use std::collections::BTreeMap;

use coop_core::schema::{tag, Field};

/// The tag of a quest record (also its sub-index: one record per quest).
pub const QUEST_RECORD_TAG: u16 = 0x5152;

/// The encoding's version, a part of what two games must agree on at the handshake.
pub const VERSION: u32 = 3;

/// The length of the fixed part: presence, stage, flags, the done set, the objective count.
pub const LEN: usize = 3 + 32 + 1;

/// The most bytes a record may take (coop-core's limit of a custom field).
pub const MAX_LEN: usize = coop_core::schema::MAX_CUSTOM_BYTES;

/// One quest's story state.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct QuestRecord {
    /// The current stage. `None`: no stage was set.
    pub stage: Option<u8>,
    /// The shared flag bits.
    pub flags: u8,
    /// The done stages, rising.
    pub done: Vec<u8>,
    /// The shown objectives: (number, status 1 to 3), rising by number.
    pub objectives: Vec<(u32, u8)>,
}

/// True for a `Custom` tag of a chunk of done stages.
fn stages_chunk(t: u16) -> bool {
    (tag::STAGES_BASE..tag::STAGES_BASE + 0x100).contains(&t)
}

/// True for a field that a quest record carries (the stage, the flags, a chunk of done stages,
/// an objective).
pub fn carries(field: &Field) -> bool {
    match field {
        Field::QuestStage { .. } | Field::QuestFlags { .. } | Field::Objective { .. } => true,
        Field::Custom { tag, .. } => stages_chunk(*tag),
        _ => false,
    }
}

impl QuestRecord {
    /// The record of a quest's shared fields. Fields that a record does not carry are left out.
    pub fn from_fields(fields: &[Field]) -> QuestRecord {
        let mut r = QuestRecord::default();
        for f in fields {
            match f {
                Field::QuestStage { stage } => r.stage = Some(u8::try_from(*stage).unwrap_or(255)),
                Field::QuestFlags { flags } => r.flags = (*flags & 0xff) as u8,
                Field::Objective { index, state } if *state & 3 != 0 => {
                    r.objectives.push((u32::from(*index), *state & 3));
                }
                Field::Custom { tag, data } if stages_chunk(*tag) => {
                    for b in data.chunks_exact(2) {
                        if let Ok(n) = u8::try_from(u16::from_le_bytes([b[0], b[1]])) {
                            r.done.push(n);
                        }
                    }
                }
                _ => {}
            }
        }
        r.done.sort_unstable();
        r.done.dedup();
        r.objectives.sort_unstable();
        r.objectives.dedup_by_key(|o| o.0);
        r
    }

    /// The record's shared fields, as the adapter exports them: no field with its default
    /// value. An objective whose number does not fit the adapter's field is left out.
    pub fn to_fields(&self) -> Vec<Field> {
        let mut out = Vec::new();
        if let Some(stage) = self.stage {
            out.push(Field::QuestStage {
                stage: u16::from(stage),
            });
        }
        let mut chunks: BTreeMap<u16, Vec<u8>> = BTreeMap::new();
        for n in &self.done {
            let n = u16::from(*n);
            chunks
                .entry(n / tag::STAGES_CHUNK)
                .or_default()
                .extend(n.to_le_bytes());
        }
        for (chunk, data) in chunks {
            out.push(Field::Custom {
                tag: tag::STAGES_BASE + chunk,
                data,
            });
        }
        out.push(Field::QuestFlags {
            flags: u32::from(self.flags),
        });
        for (n, state) in &self.objectives {
            if let Ok(index) = u16::try_from(*n) {
                out.push(Field::Objective {
                    index,
                    state: *state,
                });
            }
        }
        out.retain(|f| !f.is_tombstone());
        out
    }

    /// The record as its field. `None` when it cannot be carried exactly (more than 255
    /// objectives, or more than [`MAX_LEN`] bytes): the host then sends nothing rather than
    /// something wrong, and says so.
    pub fn encode(&self) -> Option<Field> {
        let mut data = vec![0u8; LEN];
        data[0] = u8::from(self.stage.is_some());
        data[1] = self.stage.unwrap_or(0);
        data[2] = self.flags;
        for n in &self.done {
            data[3 + usize::from(*n / 8)] |= 1 << (n % 8);
        }
        data[LEN - 1] = u8::try_from(self.objectives.len()).ok()?;
        for &(n, state) in &self.objectives {
            put_number(&mut data, n);
            data.push(state);
        }
        (data.len() <= MAX_LEN).then_some(Field::Custom {
            tag: QUEST_RECORD_TAG,
            data,
        })
    }

    /// The record of a field. `None` when the field is not a well-formed quest record.
    pub fn decode(field: &Field) -> Option<QuestRecord> {
        let Field::Custom { tag, data } = field else {
            return None;
        };
        if *tag != QUEST_RECORD_TAG || data.len() < LEN || data[0] > 1 {
            return None;
        }
        let done = (0..=255u8)
            .filter(|n| data[3 + usize::from(*n / 8)] & (1 << (n % 8)) != 0)
            .collect();
        let count = usize::from(data[LEN - 1]);
        let mut rest = &data[LEN..];
        let mut objectives = Vec::with_capacity(count);
        for _ in 0..count {
            let n = take_number(&mut rest)?;
            let (&state, after) = rest.split_first()?;
            rest = after;
            // Rising numbers, each once, and a shown status.
            if state & 3 == 0 || state > 3 || objectives.last().is_some_and(|(m, _)| *m >= n) {
                return None;
            }
            objectives.push((n, state));
        }
        if !rest.is_empty() {
            return None;
        }
        Some(QuestRecord {
            stage: (data[0] == 1).then_some(data[1]),
            flags: data[2],
            done,
            objectives,
        })
    }
}

/// Appends `n` as a variable-length number: seven bits a byte, low bits first, the top bit set
/// on each byte but the last. Objective numbers are small in the game's data (one byte each
/// below 128), so a quest with a hundred objectives fits.
fn put_number(out: &mut Vec<u8>, mut n: u32) {
    while n >= 0x80 {
        out.push(n as u8 | 0x80);
        n >>= 7;
    }
    out.push(n as u8);
}

/// Reads a number that [`put_number`] wrote from the front of `data`. `None` when it is cut
/// short, longer than a `u32` needs, or not in its shortest form.
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

#[cfg(test)]
mod tests {
    use super::*;
    use coop_core::schema::flag;

    fn rec(stage: Option<u8>, flags: u8, done: &[u8]) -> QuestRecord {
        QuestRecord {
            stage,
            flags,
            done: done.to_vec(),
            objectives: vec![],
        }
    }

    #[test]
    fn a_record_round_trips() {
        for r in [
            rec(None, 0, &[]),
            rec(Some(0), 0, &[]),
            rec(Some(255), 7, &[0, 7, 8, 128, 199, 200, 255]),
            QuestRecord {
                objectives: vec![(10, 1), (20, 3), (0xffff_ffff, 2)],
                ..rec(Some(20), 1, &[10])
            },
        ] {
            let f = r.encode().unwrap();
            assert!(f.is_sane());
            assert_eq!(QuestRecord::decode(&f), Some(r));
        }
    }

    /// The record of the adapter's fields gives the same fields back, in the adapter's order,
    /// and stages above 199 go into the second chunk. Fields that a record does not carry
    /// stay out.
    #[test]
    fn the_adapters_fields_round_trip() {
        let fields = vec![
            Field::QuestStage { stage: 210 },
            Field::Custom {
                tag: tag::STAGES_BASE,
                data: vec![10, 0, 20, 0],
            },
            Field::Custom {
                tag: tag::STAGES_BASE + 1,
                data: vec![210, 0],
            },
            Field::QuestFlags {
                flags: flag::RUNNING | flag::COMPLETED,
            },
            Field::Objective {
                index: 10,
                state: 3,
            },
            Field::Objective {
                index: 20,
                state: 1,
            },
        ];
        let mut with_var = fields.clone();
        with_var.push(Field::script_var(2, 1.0));
        let r = QuestRecord::from_fields(&with_var);
        assert_eq!(r.done, [10, 20, 210]);
        assert_eq!(r.to_fields(), fields);
        assert!(with_var.iter().filter(|f| !carries(f)).count() == 1);
        assert_eq!(QuestRecord::default().to_fields(), Vec::<Field>::new());
    }

    #[test]
    fn every_stage_number_has_its_own_bit() {
        let all: Vec<u8> = (0..=255).collect();
        let r = rec(Some(0), 0, &all);
        assert_eq!(QuestRecord::decode(&r.encode().unwrap()).unwrap().done, all);
    }

    /// A quest of an add-on has 108 objectives (seen on a PC of the other branch). With
    /// numbers of variable length its record fits a custom field.
    #[test]
    fn a_quest_with_a_hundred_objectives_fits() {
        let r = QuestRecord {
            objectives: (0..108).map(|n| (n * 5, (n % 3) as u8 + 1)).collect(),
            ..rec(Some(30), 1, &[10, 30])
        };
        let f = r.encode().unwrap();
        assert!(f.is_sane());
        assert_eq!(QuestRecord::decode(&f), Some(r));
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
            ..rec(None, 0, &[])
        };
        assert_eq!(many.encode(), None);
        let too_long = QuestRecord {
            objectives: (0..200).map(|n| (0x1000_0000 + n, 1)).collect(),
            ..rec(None, 0, &[])
        };
        assert_eq!(too_long.encode(), None);
    }

    #[test]
    fn malformed_fields_are_refused() {
        let good = QuestRecord {
            objectives: vec![(1, 1), (2, 1)],
            ..rec(Some(1), 1, &[1])
        }
        .encode()
        .unwrap();
        let Field::Custom { data, .. } = good else {
            unreachable!()
        };
        let custom = |data: Vec<u8>| Field::Custom {
            tag: QUEST_RECORD_TAG,
            data,
        };
        let mut wrong_count = data.clone();
        wrong_count[LEN - 1] = 3;
        let mut bad_presence = data.clone();
        bad_presence[0] = 2;
        let mut not_rising = data.clone();
        let n = not_rising.len();
        not_rising[n - 2] = 1;
        let mut hidden = data.clone();
        hidden[n - 1] = 0;
        for f in [
            custom(data[..LEN - 1].to_vec()),
            custom(data[..data.len() - 1].to_vec()),
            custom(wrong_count),
            custom(bad_presence),
            custom(not_rising),
            custom(hidden),
            Field::Custom {
                tag: QUEST_RECORD_TAG + 1,
                data: data.clone(),
            },
            Field::Global { bits: 0 },
        ] {
            assert_eq!(QuestRecord::decode(&f), None, "{f:?}");
        }
    }
}

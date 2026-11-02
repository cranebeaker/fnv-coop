//! Quests (`QUST`): their stages, each with log entries (conditions, the
//! journal text and a result script), and their objectives.
//!
//! The record: `EDID`, `SCRI` (the quest's script), `FULL`, `DATA` (flags
//! u8, priority u8, two unused bytes, script delay f32); then each stage
//! `INDX` (its number, i16) followed by its log entries, each `QSDT`
//! (flags: 0x01 completes the quest, 0x02 fails it), `CTDA` conditions,
//! `CNAM` text, the result script (`SCHR` `SCDA` `SCTX` `SCRO`…) and
//! `NAM0` (next quest); then each objective `QOBJ` (its number, i32),
//! `NNAM` (text) and `QSTA` targets with their conditions.

use esm::{FormId, FourCC, LoadOrder};

use crate::cell::{le_f32, le_u32};
use crate::dialogue::{read_condition, Condition};

const QUST: FourCC = FourCC::new(b"QUST");
const SCRI: FourCC = FourCC::new(b"SCRI");
const FULL: FourCC = FourCC::new(b"FULL");
const INDX: FourCC = FourCC::new(b"INDX");
const QSDT: FourCC = FourCC::new(b"QSDT");
const CTDA: FourCC = FourCC::new(b"CTDA");
const CNAM: FourCC = FourCC::new(b"CNAM");
const SCTX: FourCC = FourCC::new(b"SCTX");
const NAM0: FourCC = FourCC::new(b"NAM0");
const QOBJ: FourCC = FourCC::new(b"QOBJ");
const NNAM: FourCC = FourCC::new(b"NNAM");
const QSTA: FourCC = FourCC::new(b"QSTA");

/// `DATA` flag: running from the start of a new game.
pub const START_GAME_ENABLED: u8 = 0x01;
/// `DATA` flag: a stage can be set again, running its script again.
pub const ALLOW_REPEATED_STAGES: u8 = 0x08;
/// `QSDT` flags.
pub const COMPLETES_QUEST: u8 = 0x01;
pub const FAILS_QUEST: u8 = 0x02;

#[derive(Debug, Clone, PartialEq)]
pub struct Quest {
    pub form_id: FormId,
    pub editor_id: Option<String>,
    pub name: Option<String>,
    pub flags: u8,
    pub priority: u8,
    /// How often its script runs, in seconds; 0 means the default.
    pub delay: f32,
    pub script: Option<FormId>,
    /// The quest's own conditions (`CTDA` before the first stage): every
    /// line of its dialogue needs them to pass for the speaker
    /// (`vDialogueEDE`'s: `GetIsID` one of the three ED-E records).
    pub conditions: Vec<Condition>,
    pub stages: Vec<Stage>,
    pub objectives: Vec<Objective>,
}

/// A quest's own conditions, read without the rest of it.
pub fn quest_conditions(order: &LoadOrder, id: FormId) -> Vec<Condition> {
    let Some(rr) = order.get(id).filter(|r| r.entry.header.kind == QUST) else {
        return Vec::new();
    };
    let Ok(record) = rr.record() else {
        return Vec::new();
    };
    record
        .subrecords
        .iter()
        .take_while(|s| s.kind != INDX && s.kind != QOBJ)
        .filter(|s| s.kind == CTDA)
        .filter_map(|s| read_condition(&rr, &s.data))
        .collect()
}

#[derive(Debug, Clone, PartialEq)]
pub struct Stage {
    pub index: i16,
    pub entries: Vec<LogEntry>,
}

/// One of a stage's log entries: the first whose conditions pass is the
/// one used when the stage is set.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct LogEntry {
    pub flags: u8,
    pub conditions: Vec<Condition>,
    /// The journal text.
    pub text: Option<String>,
    /// The result script's source.
    pub script: Option<String>,
    pub next_quest: Option<FormId>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Objective {
    pub index: i32,
    pub text: String,
    /// Where the compass and maps point while the objective is shown
    /// (`QSTA` each, with the `CTDA` conditions after it).
    pub targets: Vec<QuestTarget>,
}

/// One of an objective's targets (`TESQuestTarget` (Xbox PDB): `cFlags`
/// +0x00, `objConditions` +0x04, `m_pTargetRef` +0x0c, `m_TargetPath`
/// +0x10). `QSTA` is 8 bytes (`00610120` reads them: the reference into
/// +0x0c, byte 4 into the flags; bytes 5..8 are left unread).
#[derive(Debug, Clone, PartialEq)]
pub struct QuestTarget {
    pub reference: FormId,
    pub flags: u8,
    /// Asked about the target reference (`005ec500`: `00680c30` with the
    /// reference as the subject and no target).
    pub conditions: Vec<Condition>,
}

/// `QSTA` flag: the compass marker ignores locks
/// (`TESQuestTarget::GetIgnoreLocks` (Xbox PDB), `00610190`: flags & 1).
pub const TARGET_IGNORES_LOCKS: u8 = 0x01;

impl Quest {
    pub fn load(order: &LoadOrder, id: FormId) -> Option<Quest> {
        let rr = order.get(id).filter(|r| r.entry.header.kind == QUST)?;
        let record = rr.record().ok()?;
        let form = |data: &[u8]| {
            (data.len() >= 4)
                .then(|| rr.plugin.to_global(FormId(le_u32(data, 0))))
                .filter(|f| f.0 != 0)
        };
        let data = record.get(esm::sig::DATA).map(|s| &s.data[..]);
        let mut quest = Quest {
            form_id: id,
            editor_id: record.editor_id(),
            name: record.get(FULL).map(|s| s.zstring()),
            flags: data.and_then(|d| d.first().copied()).unwrap_or(0),
            priority: data.and_then(|d| d.get(1).copied()).unwrap_or(0),
            delay: data.filter(|d| d.len() >= 8).map_or(0.0, |d| le_f32(d, 4)),
            script: record.get(SCRI).and_then(|s| form(&s.data)),
            conditions: Vec::new(),
            stages: Vec::new(),
            objectives: Vec::new(),
        };
        // Whether subrecords now belong to objectives (after the first
        // `QOBJ`), where `CTDA` are target conditions.
        let mut in_objectives = false;
        fn entry(q: &mut Quest) -> Option<&mut LogEntry> {
            q.stages.last_mut().and_then(|s| s.entries.last_mut())
        }
        for sub in &record.subrecords {
            match sub.kind {
                k if k == INDX && sub.data.len() >= 2 => {
                    in_objectives = false;
                    quest.stages.push(Stage {
                        index: i16::from_le_bytes([sub.data[0], sub.data[1]]),
                        entries: Vec::new(),
                    });
                }
                k if k == QSDT => {
                    if let Some(stage) = quest.stages.last_mut() {
                        stage.entries.push(LogEntry {
                            flags: sub.data.first().copied().unwrap_or(0),
                            ..LogEntry::default()
                        });
                    }
                }
                k if k == CTDA && !in_objectives => {
                    if let Some(c) = read_condition(&rr, &sub.data) {
                        if quest.stages.is_empty() {
                            quest.conditions.push(c);
                        } else if let Some(e) = entry(&mut quest) {
                            e.conditions.push(c);
                        }
                    }
                }
                k if k == CNAM => {
                    if let Some(e) = entry(&mut quest) {
                        e.text = Some(sub.zstring()).filter(|t| !t.is_empty());
                    }
                }
                k if k == SCTX => {
                    if let Some(e) = entry(&mut quest) {
                        e.script = Some(esm::text::decode_cp1252(&sub.data));
                    }
                }
                k if k == NAM0 => {
                    if let Some(e) = entry(&mut quest) {
                        e.next_quest = form(&sub.data);
                    }
                }
                k if k == QOBJ && sub.data.len() >= 4 => {
                    in_objectives = true;
                    quest.objectives.push(Objective {
                        index: le_u32(&sub.data, 0) as i32,
                        text: String::new(),
                        targets: Vec::new(),
                    });
                }
                k if k == NNAM && in_objectives => {
                    if let Some(o) = quest.objectives.last_mut() {
                        o.text = sub.zstring();
                    }
                }
                k if k == QSTA && in_objectives && sub.data.len() >= 8 => {
                    if let Some(o) = quest.objectives.last_mut() {
                        o.targets.push(QuestTarget {
                            reference: rr.plugin.to_global(FormId(le_u32(&sub.data, 0))),
                            flags: sub.data[4],
                            conditions: Vec::new(),
                        });
                    }
                }
                k if k == CTDA && in_objectives => {
                    let target = quest
                        .objectives
                        .last_mut()
                        .and_then(|o| o.targets.last_mut());
                    if let (Some(t), Some(c)) = (target, read_condition(&rr, &sub.data)) {
                        t.conditions.push(c);
                    }
                }
                _ => {}
            }
        }
        Some(quest)
    }

    pub fn stage(&self, index: u16) -> Option<&Stage> {
        self.stages.iter().find(|s| s.index as u16 == index)
    }

    pub fn objective(&self, index: i32) -> Option<&Objective> {
        self.objectives.iter().find(|o| o.index == index)
    }
}

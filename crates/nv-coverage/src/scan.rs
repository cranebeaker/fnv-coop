//! Collects every script source in the load order, with where it lives
//! (object script, quest script, stage result, dialogue result, ...), and
//! the functions each one calls.

use std::collections::{BTreeMap, HashMap};

use esm::{FormId, FourCC, LoadOrder};

const SCTX: FourCC = FourCC::new(b"SCTX");
const SCHR: FourCC = FourCC::new(b"SCHR");
const INDX: FourCC = FourCC::new(b"INDX");
const QSTI: FourCC = FourCC::new(b"QSTI");
const CTDA: FourCC = FourCC::new(b"CTDA");

/// What kind of script a source is.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Kind {
    ObjectScript,
    QuestScript,
    EffectScript,
    StageResult,
    DialogueResult,
    TerminalResult,
    Other,
}

impl Kind {
    pub const ALL: [Kind; 7] = [
        Kind::ObjectScript,
        Kind::QuestScript,
        Kind::EffectScript,
        Kind::StageResult,
        Kind::DialogueResult,
        Kind::TerminalResult,
        Kind::Other,
    ];
    pub fn label(self) -> &'static str {
        match self {
            Kind::ObjectScript => "object script",
            Kind::QuestScript => "quest script",
            Kind::EffectScript => "effect script",
            Kind::StageResult => "quest stage result",
            Kind::DialogueResult => "dialogue result",
            Kind::TerminalResult => "terminal result",
            Kind::Other => "other",
        }
    }
}

/// One script source.
pub struct Source {
    pub kind: Kind,
    pub record: FourCC,
    pub form: FormId,
    pub editor: String,
    /// The quest it belongs to (the quest itself, or a dialogue topic's quest).
    pub quest: Option<FormId>,
    pub stage: Option<i16>,
    pub plugin: String,
    pub text: String,
    pub parsed: Result<script::Script, String>,
}

/// A script's call: function number and the reference it's called on.
pub struct Used {
    pub function: u16,
}

/// All sources from every plugin in the load order.
pub fn sources(order: &LoadOrder) -> Vec<Source> {
    let mut out = Vec::new();
    let mut topic_quest: HashMap<FormId, Option<FormId>> = HashMap::new();
    for (kind, _) in order.type_counts() {
        for rr in order.records_of_type(kind) {
            let Ok(record) = rr.record() else { continue };
            if !record.subrecords.iter().any(|s| s.kind == SCTX) {
                continue;
            }
            let editor = record.editor_id().unwrap_or_default();
            let quest_of_topic = if kind.as_bytes() == b"INFO" {
                order.topic_of(&rr).and_then(|t| {
                    *topic_quest.entry(t).or_insert_with(|| {
                        let d = order.get(t)?.record().ok()?;
                        let q = d.get(QSTI)?;
                        let raw = q.data.get(..4)?;
                        Some(rr.plugin.to_global(FormId(u32::from_le_bytes([
                            raw[0], raw[1], raw[2], raw[3],
                        ]))))
                    })
                })
            } else {
                None
            };
            let mut stage = None;
            let mut schr_type = 0u16;
            for sub in &record.subrecords {
                if sub.kind == INDX && sub.data.len() >= 2 {
                    stage = Some(i16::from_le_bytes([sub.data[0], sub.data[1]]));
                } else if sub.kind == SCHR {
                    schr_type = sub
                        .data
                        .get(12..14)
                        .map_or(0, |b| u16::from_le_bytes([b[0], b[1]]));
                } else if sub.kind == SCTX {
                    let text = esm::text::decode_cp1252(&sub.data);
                    let (k, quest) = match kind.as_bytes() {
                        b"SCPT" => match schr_type {
                            1 => (Kind::QuestScript, None),
                            0x100 => (Kind::EffectScript, None),
                            _ => (Kind::ObjectScript, None),
                        },
                        b"QUST" => (Kind::StageResult, Some(rr.form_id)),
                        b"INFO" => (Kind::DialogueResult, quest_of_topic),
                        b"TERM" => (Kind::TerminalResult, None),
                        _ => (Kind::Other, None),
                    };
                    out.push(Source {
                        kind: k,
                        record: kind,
                        form: rr.form_id,
                        editor: editor.clone(),
                        quest,
                        stage: if kind.as_bytes() == b"QUST" {
                            stage
                        } else {
                            None
                        },
                        plugin: rr.plugin.name.clone(),
                        parsed: script::parse(&text).map_err(|e| e.to_string()),
                        text,
                    });
                }
            }
        }
    }
    out
}

/// Every call a parsed script makes, in order.
pub fn all_calls(s: &script::Script) -> Vec<&script::Call> {
    fn expr<'a>(e: &'a script::Expr, out: &mut Vec<&'a script::Call>) {
        for item in &e.0 {
            if let script::Item::Call(c) = item {
                out.push(c);
            }
        }
    }
    fn walk<'a>(stmts: &'a [script::Stmt], out: &mut Vec<&'a script::Call>) {
        for st in stmts {
            match st {
                script::Stmt::Set { value, .. } => expr(value, out),
                script::Stmt::Call(c) => out.push(c),
                script::Stmt::If {
                    branches,
                    otherwise,
                } => {
                    for (c, b) in branches {
                        expr(c, out);
                        walk(b, out);
                    }
                    if let Some(b) = otherwise {
                        walk(b, out);
                    }
                }
                script::Stmt::Return => {}
            }
        }
    }
    let mut out = Vec::new();
    for b in &s.blocks {
        walk(&b.body, &mut out);
    }
    walk(&s.body, &mut out);
    out
}

/// The function numbers of every call.
pub fn calls_of(s: &script::Script) -> Vec<u16> {
    all_calls(s).into_iter().map(|c| c.function).collect()
}

/// Condition functions used by records of one type: record type → function → uses.
pub fn condition_uses(order: &LoadOrder, record: &[u8; 4]) -> BTreeMap<u16, usize> {
    let mut map = BTreeMap::new();
    for rr in order.records_of_type(FourCC::new(record)) {
        let Ok(r) = rr.record() else { continue };
        for c in r.get_all(CTDA) {
            if let Some(f) = c.data.get(8..10) {
                *map.entry(u16::from_le_bytes([f[0], f[1]])).or_default() += 1;
            }
        }
    }
    map
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn collects_calls_from_nested_statements() {
        let s = script::parse(
            "ScriptName T\nBegin GameMode\nif GetStage VMQ01 > 5\nSetStage VMQ01 10\nendif\nEnd\n",
        )
        .unwrap();
        let calls = calls_of(&s);
        assert_eq!(calls.len(), 2);
        assert_eq!(script::function_name(calls[0]), "GetStage");
    }
}

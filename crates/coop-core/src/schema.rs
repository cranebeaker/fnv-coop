//! The semantic record schema: what a FormID-keyed write can say.
//!
//! The FormID is the subject of the record (quest, global, topic, holder
//! actor, owner ref, faction ...). The [`Field`] says which slot of the subject
//! changes and carries the new value. Floats travel as raw bits so `Eq` and
//! hashing are exact.

use serde::{Deserialize, Serialize};

use crate::ids::FormId;

/// Largest accepted `Field::Custom` payload.
pub const MAX_CUSTOM_BYTES: usize = 512;

/// Identity of one replicated slot: subject form, field kind, sub-index.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Key {
    /// Subject form.
    pub form: FormId,
    /// Field discriminant (see [`Field::kind`]).
    pub kind: u8,
    /// Sub-index inside the kind (objective index, item id, script var index ...).
    pub sub: u32,
}

/// Who may write a field.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WriteClass {
    /// Story state: only the host writes it.
    Story,
    /// Entity state: the host, or whoever owns the subject form.
    Entity,
}

/// One value-carrying slot update.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Field {
    /// Quest current stage (subject: quest).
    QuestStage {
        /// New stage.
        stage: u16,
    },
    /// Quest flag bits (subject: quest).
    QuestFlags {
        /// Flag bits.
        flags: u32,
    },
    /// Objective state (subject: quest).
    Objective {
        /// Objective index.
        index: u16,
        /// Displayed/completed bits.
        state: u8,
    },
    /// Global variable (subject: global), f32 bits.
    Global {
        /// `f32::to_bits`.
        bits: u32,
    },
    /// Dialogue topic known flag (subject: topic).
    Topic {
        /// Whether the topic is known.
        known: bool,
    },
    /// Inventory count (subject: holder).
    ItemCount {
        /// Item form.
        item: FormId,
        /// Count.
        count: i32,
    },
    /// Reference enabled/disabled (subject: ref).
    RefDisabled {
        /// True when disabled.
        disabled: bool,
    },
    /// Script variable (subject: owner of the script).
    ScriptVar {
        /// Variable index.
        index: u16,
        /// `f64::to_bits`.
        bits: u64,
    },
    /// Shared karma (subject: party singleton).
    Karma {
        /// Karma value.
        value: i32,
    },
    /// Faction reputation (subject: faction).
    Reputation {
        /// Reputation value.
        value: i32,
    },
    /// Faction rank (subject: faction).
    FactionRank {
        /// Rank, -1 for none.
        rank: i8,
    },
    /// Actor or avatar position in fixed point (subject: actor).
    Position {
        /// X.
        x: i32,
        /// Y.
        y: i32,
        /// Z.
        z: i32,
    },
    /// Actor value (subject: actor).
    ActorValue {
        /// Actor value id.
        av: u16,
        /// Value.
        value: i32,
    },
    /// Escape hatch for records the schema does not model yet (host only).
    Custom {
        /// Caller-defined tag, also the sub-index.
        tag: u16,
        /// Opaque bytes, at most [`MAX_CUSTOM_BYTES`].
        data: Vec<u8>,
    },
}

impl Field {
    /// Builds a `Global` from an f32.
    pub fn global(v: f32) -> Field {
        Field::Global { bits: v.to_bits() }
    }

    /// Builds a `ScriptVar` from an f64.
    pub fn script_var(index: u16, v: f64) -> Field {
        Field::ScriptVar {
            index,
            bits: v.to_bits(),
        }
    }

    /// Stable discriminant used in [`Key`].
    pub fn kind(&self) -> u8 {
        match self {
            Field::QuestStage { .. } => 0,
            Field::QuestFlags { .. } => 1,
            Field::Objective { .. } => 2,
            Field::Global { .. } => 3,
            Field::Topic { .. } => 4,
            Field::ItemCount { .. } => 5,
            Field::RefDisabled { .. } => 6,
            Field::ScriptVar { .. } => 7,
            Field::Karma { .. } => 8,
            Field::Reputation { .. } => 9,
            Field::FactionRank { .. } => 10,
            Field::Position { .. } => 11,
            Field::ActorValue { .. } => 12,
            Field::Custom { .. } => 255,
        }
    }

    /// Sub-index that distinguishes slots of the same kind on one form.
    pub fn sub(&self) -> u32 {
        match self {
            Field::Objective { index, .. } => *index as u32,
            Field::ItemCount { item, .. } => item.0,
            Field::ScriptVar { index, .. } => *index as u32,
            Field::ActorValue { av, .. } => *av as u32,
            Field::Custom { tag, .. } => *tag as u32,
            _ => 0,
        }
    }

    /// The slot key of this field on `form`.
    pub fn key(&self, form: FormId) -> Key {
        Key {
            form,
            kind: self.kind(),
            sub: self.sub(),
        }
    }

    /// Write permission class.
    pub fn class(&self) -> WriteClass {
        match self {
            Field::ItemCount { .. }
            | Field::ScriptVar { .. }
            | Field::Position { .. }
            | Field::ActorValue { .. } => WriteClass::Entity,
            _ => WriteClass::Story,
        }
    }

    /// Size sanity check for values arriving from the network.
    pub fn is_sane(&self) -> bool {
        match self {
            Field::Custom { data, .. } => data.len() <= MAX_CUSTOM_BYTES,
            _ => true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn keys_distinguish_sub_slots() {
        let f = FormId(1);
        let a = Field::Objective { index: 1, state: 1 }.key(f);
        let b = Field::Objective { index: 2, state: 1 }.key(f);
        assert_ne!(a, b);
        let c = Field::Objective { index: 1, state: 9 }.key(f);
        assert_eq!(a, c);
        assert_ne!(
            Field::QuestStage { stage: 1 }.key(f),
            Field::QuestFlags { flags: 1 }.key(f)
        );
    }

    #[test]
    fn classes() {
        assert_eq!(Field::QuestStage { stage: 1 }.class(), WriteClass::Story);
        assert_eq!(Field::Karma { value: 1 }.class(), WriteClass::Story);
        assert_eq!(
            Field::ItemCount {
                item: FormId(2),
                count: 1
            }
            .class(),
            WriteClass::Entity
        );
        assert_eq!(
            Field::Custom {
                tag: 1,
                data: vec![]
            }
            .class(),
            WriteClass::Story
        );
    }

    #[test]
    fn custom_size_limit() {
        assert!(Field::Custom {
            tag: 0,
            data: vec![0; MAX_CUSTOM_BYTES]
        }
        .is_sane());
        assert!(!Field::Custom {
            tag: 0,
            data: vec![0; MAX_CUSTOM_BYTES + 1]
        }
        .is_sane());
    }

    #[test]
    fn float_bits_exact() {
        let f = Field::global(f32::NAN);
        assert_eq!(f, f.clone());
        let g = Field::script_var(3, -0.0);
        assert_ne!(g, Field::script_var(3, 0.0));
    }
}

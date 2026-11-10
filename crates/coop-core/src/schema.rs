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

/// Custom tags (sub-index of `Field::Custom`).
pub mod tag {
    /// `said` line.
    pub const SAID: u16 = 1;
    /// `talked_to` person.
    pub const TALKED: u16 = 2;
    /// `notes`.
    pub const NOTE: u16 = 3;
    /// `destroyed`.
    pub const DESTROYED: u16 = 4;
    /// `stocked` holder.
    pub const STOCKED: u16 = 5;
    /// `map_markers`.
    pub const MARKER: u16 = 6;
    /// `perks` (data: rank).
    pub const PERK: u16 = 7;
    /// Player level (data: u16 le), on `PLAYER_REF`.
    pub const LEVEL: u16 = 8;
    /// Reputation (data: fame f32 bits, infamy f32 bits).
    pub const REPUTATION: u16 = 9;
    /// A dead actor (not a player): in the shared hash, so a lost kill
    /// message is noticed and repaired.
    pub const DEAD: u16 = 10;
    /// A faction's relations (`SetEnemy`, `SetAlly`), on the faction (data:
    /// other faction u32 le and reaction u8, per entry, sorted).
    pub const RELATION: u16 = 11;
    /// A faction's crime, on the faction (data: minor u32 le, major u32 le,
    /// holds the player as an enemy u8).
    pub const CRIME: u16 = 12;
    /// The player's crime, on `PLAYER_REF` (data: minor u32 le, major u32
    /// le, steal warnings u32 le, murderer u8).
    pub const PLAYER_CRIME: u16 = 13;
    /// Done stages of a quest: `STAGES_BASE + stage / STAGES_CHUNK`.
    pub const STAGES_BASE: u16 = 0x1000;
    /// Stages per chunk (2 bytes each, well under the 512 byte limit).
    pub const STAGES_CHUNK: u16 = 200;
    /// Faction members: `FACTION_BASE + (person & 0xFF)`.
    pub const FACTION_BASE: u16 = 0x2000;
    /// Entries kept per faction chunk (5 bytes each).
    pub const FACTION_MAX: usize = 100;
}

/// Quest flag bits.
pub mod flag {
    /// In `running`.
    pub const RUNNING: u32 = 1;
    /// In `completed`.
    pub const COMPLETED: u32 = 2;
    /// In `failed`.
    pub const FAILED: u32 = 4;
    /// In `quests_announced`.
    pub const ANNOUNCED: u32 = 8;
}

// The tag blocks do not meet: the single tags end below the chunks of done stages, and the 256
// stage numbers of a quest end below the faction block. The flag bits are distinct.
const _: () = {
    assert!(tag::PLAYER_CRIME < tag::STAGES_BASE);
    assert!(tag::STAGES_BASE + 255 / tag::STAGES_CHUNK < tag::FACTION_BASE);
    assert!(flag::RUNNING | flag::COMPLETED | flag::FAILED | flag::ANNOUNCED == 15);
};

/// A list of fields as one word of text (hex of the wire encoding), for a line protocol
/// between two processes.
pub fn fields_to_hex(fields: &[(FormId, Field)]) -> String {
    // Encoding plain data into a vector cannot fail.
    let bytes = postcard::to_allocvec(fields).unwrap_or_default();
    let mut text = String::with_capacity(bytes.len() * 2 + 1);
    for b in bytes {
        text.push_str(&format!("{b:02x}"));
    }
    text
}

/// The fields of [`fields_to_hex`]'s text. `Err` says what is wrong with the text.
pub fn fields_from_hex(text: &str) -> Result<Vec<(FormId, Field)>, String> {
    if text.is_empty() || text.len() % 2 != 0 {
        return Err("not an even number of hex digits".into());
    }
    let bytes: Result<Vec<u8>, String> = (0..text.len() / 2)
        .map(|i| {
            text.get(2 * i..2 * i + 2)
                .and_then(|pair| u8::from_str_radix(pair, 16).ok())
                .ok_or_else(|| format!("bad hex at {}", 2 * i))
        })
        .collect();
    postcard::from_bytes(&bytes?).map_err(|e| format!("not a field list: {e}"))
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

    /// True when the field is the default ("absent") value of its slot. A slot with its
    /// default value is not exported: each game adapter must agree on this, or the hashes of
    /// two machines differ. A stage, a global, a disabled flag, a reputation, a rank and a
    /// position have no default: each value of theirs is kept.
    pub fn is_tombstone(&self) -> bool {
        match self {
            Field::QuestFlags { flags } => *flags == 0,
            Field::Objective { state, .. } => *state == 0,
            Field::Topic { known } => !known,
            Field::ItemCount { count, .. } => *count == 0,
            Field::ScriptVar { bits, .. } => *bits == 0,
            Field::Karma { value } | Field::ActorValue { value, .. } => *value == 0,
            Field::Custom { data, .. } => data.is_empty(),
            _ => false,
        }
    }

    /// The default-valued field that clears this field's slot (the same key, see
    /// [`Field::is_tombstone`]). A kind with no default gives a copy of the field.
    pub fn tombstone(&self) -> Field {
        match self {
            Field::QuestFlags { .. } => Field::QuestFlags { flags: 0 },
            Field::Objective { index, .. } => Field::Objective {
                index: *index,
                state: 0,
            },
            Field::Topic { .. } => Field::Topic { known: false },
            Field::ItemCount { item, .. } => Field::ItemCount {
                item: *item,
                count: 0,
            },
            Field::ScriptVar { index, .. } => Field::ScriptVar {
                index: *index,
                bits: 0,
            },
            Field::Karma { .. } => Field::Karma { value: 0 },
            Field::ActorValue { av, .. } => Field::ActorValue { av: *av, value: 0 },
            Field::Custom { tag, .. } => Field::Custom {
                tag: *tag,
                data: Vec::new(),
            },
            other => other.clone(),
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

    /// One field of each kind, with a value that is not the default.
    fn one_of_each() -> Vec<Field> {
        vec![
            Field::QuestStage { stage: 10 },
            Field::QuestFlags { flags: 1 },
            Field::Objective { index: 3, state: 1 },
            Field::global(2.5),
            Field::Topic { known: true },
            Field::ItemCount {
                item: FormId(0xf),
                count: 4,
            },
            Field::RefDisabled { disabled: true },
            Field::script_var(2, 1.0),
            Field::Karma { value: -5 },
            Field::Reputation { value: 3 },
            Field::FactionRank { rank: 1 },
            Field::Position { x: 1, y: 2, z: 3 },
            Field::ActorValue { av: 16, value: 9 },
            Field::Custom {
                tag: 7,
                data: vec![1],
            },
        ]
    }

    /// A tombstone keeps the slot's key. The kinds with a default clear to it, and the kinds
    /// with no default stay as they are.
    #[test]
    fn a_tombstone_clears_the_same_slot() {
        let with_default = [1u8, 2, 4, 5, 7, 8, 12, 255];
        let form = FormId(0x10);
        for f in one_of_each() {
            assert!(!f.is_tombstone(), "{f:?}");
            let t = f.tombstone();
            assert_eq!(t.key(form), f.key(form), "{f:?}");
            if with_default.contains(&f.kind()) {
                assert!(t.is_tombstone(), "{f:?}");
                assert_ne!(t, f);
            } else {
                assert!(!t.is_tombstone(), "{f:?}");
                assert_eq!(t, f);
            }
        }
    }

    /// A field list survives the text form, the empty list too. Bad text is refused.
    #[test]
    fn a_field_list_survives_the_text_form() {
        let fields: Vec<(FormId, Field)> = one_of_each()
            .into_iter()
            .enumerate()
            .map(|(i, f)| (FormId(0x100 + i as u32), f))
            .collect();
        let text = fields_to_hex(&fields);
        assert!(text.bytes().all(|b| b.is_ascii_hexdigit()));
        assert_eq!(fields_from_hex(&text).unwrap(), fields);
        assert_eq!(fields_from_hex(&fields_to_hex(&[])).unwrap(), []);
        for bad in ["", "abc", "zz", "ff"] {
            assert!(fields_from_hex(bad).is_err(), "{bad}");
        }
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

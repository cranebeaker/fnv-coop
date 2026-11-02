//! Typed view of weapon (`WEAP`) records.

use crate::cursor::Cursor;
use crate::record::Record;
use crate::types::{sig, FormId};

/// Core stats from a weapon's `DATA` subrecord.
///
/// The 15-byte layout follows community documentation of the Fallout: New
/// Vegas format. If a record's `DATA` has any other length it is not
/// decoded, so a layout mismatch shows up as missing stats rather than
/// wrong numbers.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct WeaponStats {
    /// Base value in caps.
    pub value: i32,
    /// Maximum condition.
    pub health: i32,
    pub weight: f32,
    pub base_damage: i16,
    pub clip_size: u8,
}

impl WeaponStats {
    pub const DATA_LEN: usize = 15;

    pub fn from_data(data: &[u8]) -> Option<Self> {
        if data.len() != Self::DATA_LEN {
            return None;
        }
        let mut c = Cursor::new(data, 0);
        Some(Self {
            value: c.i32("weapon value").ok()?,
            health: c.i32("weapon health").ok()?,
            weight: c.f32("weapon weight").ok()?,
            base_damage: c.i16("weapon damage").ok()?,
            clip_size: c.u8("weapon clip size").ok()?,
        })
    }
}

/// A weapon record's identity and stats.
#[derive(Debug, Clone, PartialEq)]
pub struct Weapon {
    pub form_id: FormId,
    pub editor_id: Option<String>,
    pub name: Option<String>,
    pub stats: Option<WeaponStats>,
}

impl Weapon {
    /// Returns `None` if the record is not a `WEAP`.
    pub fn from_record(record: &Record) -> Option<Self> {
        if record.header.kind != sig::WEAP {
            return None;
        }
        Some(Self {
            form_id: record.header.form_id,
            editor_id: record.editor_id(),
            name: record.full_name(),
            stats: record
                .get(sig::DATA)
                .and_then(|d| WeaponStats::from_data(&d.data)),
        })
    }
}

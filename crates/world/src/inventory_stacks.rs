//! The original game's inventory stacks, as a `.fos` save holds them
//! (Phase 2 step B, docs/ENGINE_STATE.md "World inventory stacks").
//!
//! A holder's inventory is saved as `InventoryChanges::SaveGame` (Xbox
//! PDB), `004d4090`: one `ItemChange` per entry (`004bed60`), each the
//! item's saved id, a signed count delta against the holder's base record
//! (`ItemChange` +4) and one extra data list per stack (`00426a30`, the
//! lists written with change flags 0x400). Loading goes through
//! `TESObjectREFR::LoadGame_ov2` (`005623d0`) to `004d4160` per item when
//! change flag 0x08000020 is set; saving through `00562230` to `004d4090`
//! (FalloutNV.exe 1.4.0.525).
//!
//! This is plain owned data taken from `fos::inventory_source` on import:
//! the per-stack count, raw health bits and worn state that nv-rs reads,
//! and every other extra kept as its original bytes, uninterpreted. nv-rs's
//! aggregate views (`GameState::items`, `equipped`, `weapon_health`) are
//! derived from it on import; gameplay changes those views, not these
//! stacks (making the stacks the one live inventory is open).

use std::collections::BTreeMap;

use esm::FormId;

/// How a saved item id resolved through the save's own form id array
/// (`fos::inventory_source::Resolution`), not the active load order.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SavedResolution {
    /// The saved id was 0.
    Null,
    /// The form id the save's table gives (plugin index as the save had it).
    InSave(u32),
    /// An index past the save's form id array.
    Missing(u32),
}

/// One extra data entry of a stack.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SavedExtra {
    /// `ExtraCount` (0x24): the stack's signed 16-bit count.
    Count(i16),
    /// `ExtraHealth` (0x25): the stack's health, as its raw `f32` bits.
    Health(u32),
    /// `ExtraWorn` (0x16): no payload.
    Worn,
    /// Any other kind: its complete original bytes (type byte, separators
    /// and payload, as `00426a30` wrote them), not interpreted.
    Opaque { kind: u8, bytes: Vec<u8> },
}

/// One stack: its extra data list in saved order.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct SavedStack {
    pub extras: Vec<SavedExtra>,
}

impl SavedStack {
    /// The stack's `ExtraCount`, if it has one (the first, as saved).
    pub fn count(&self) -> Option<i16> {
        self.extras.iter().find_map(|e| match e {
            SavedExtra::Count(n) => Some(*n),
            _ => None,
        })
    }

    /// The stack's `ExtraHealth` raw bits, if it has one (the first).
    pub fn health_bits(&self) -> Option<u32> {
        self.extras.iter().find_map(|e| match e {
            SavedExtra::Health(h) => Some(*h),
            _ => None,
        })
    }

    pub fn worn(&self) -> bool {
        self.extras.iter().any(|e| matches!(e, SavedExtra::Worn))
    }
}

/// One `ItemChange` (`004bed60`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SavedItem {
    /// The item's id as saved (a refID into the save's form id array, or
    /// a created form's).
    pub saved: u32,
    pub resolution: SavedResolution,
    /// The form in nv-rs's load order; `None` when the id didn't resolve
    /// or its plugin isn't loaded.
    pub form: Option<FormId>,
    /// More (or fewer) than the holder's base record holds.
    pub count_delta: i32,
    pub stacks: Vec<SavedStack>,
}

/// A holder's saved inventory changes, in saved order.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct SavedInventory {
    /// The change form's version (script locals in opaque extras depend
    /// on it).
    pub version: u8,
    pub items: Vec<SavedItem>,
}

/// Every imported holder's saved inventory.
pub type SavedInventories = BTreeMap<FormId, SavedInventory>;

fn id(f: FormId) -> String {
    format!("{:08X}", f.0)
}

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02X}")).collect()
}

fn unhex(s: &str) -> Option<Vec<u8>> {
    if s.len() % 2 != 0 {
        return None;
    }
    (0..s.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(s.get(i..i + 2)?, 16).ok())
        .collect()
}

/// nv-rs's own save lines: `invholder`, then per item `invitem`, per
/// stack `invstack` and per extra `invextra`, in saved order.
pub fn save_lines(stacks: &SavedInventories, line: &mut impl FnMut(String)) {
    for (holder, inventory) in stacks {
        let h = id(*holder);
        line(format!("invholder {h} {}", inventory.version));
        for item in &inventory.items {
            let resolution = match item.resolution {
                SavedResolution::Null => "null 0".to_string(),
                SavedResolution::InSave(f) => format!("save {f:08X}"),
                SavedResolution::Missing(i) => format!("missing {i:08X}"),
            };
            line(format!(
                "invitem {h} {:08X} {resolution} {} {}",
                item.saved,
                item.form.map_or("-".to_string(), id),
                item.count_delta
            ));
            for stack in &item.stacks {
                line(format!("invstack {h}"));
                for extra in &stack.extras {
                    line(match extra {
                        SavedExtra::Count(n) => format!("invextra {h} count {n}"),
                        SavedExtra::Health(bits) => format!("invextra {h} health {bits:08X}"),
                        SavedExtra::Worn => format!("invextra {h} worn"),
                        SavedExtra::Opaque { kind, bytes } => {
                            format!("invextra {h} opaque {kind:02X} {}", hex(bytes))
                        }
                    });
                }
            }
        }
    }
}

/// Reads one of [`save_lines`]' lines; `None` if it isn't one.
pub fn load_line(stacks: &mut SavedInventories, raw: &str) -> Option<Result<(), String>> {
    let parts: Vec<&str> = raw.split_whitespace().collect();
    let word = *parts.first()?;
    if !matches!(word, "invholder" | "invitem" | "invstack" | "invextra") {
        return None;
    }
    let bad = || format!("can't read '{raw}'");
    let hex32 = |i: usize| -> Result<u32, String> {
        parts
            .get(i)
            .and_then(|s| u32::from_str_radix(s, 16).ok())
            .ok_or_else(bad)
    };
    let result = (|| -> Result<(), String> {
        let holder = FormId(hex32(1)?);
        if word == "invholder" {
            let version = parts.get(2).and_then(|s| s.parse().ok()).ok_or_else(bad)?;
            if parts.len() != 3 || stacks.contains_key(&holder) {
                return Err(bad());
            }
            stacks.insert(
                holder,
                SavedInventory {
                    version,
                    items: Vec::new(),
                },
            );
            return Ok(());
        }
        let inventory = stacks.get_mut(&holder).ok_or_else(bad)?;
        match word {
            "invitem" => {
                let resolution = match parts.get(3) {
                    Some(&"null") if hex32(4)? == 0 => SavedResolution::Null,
                    Some(&"save") => SavedResolution::InSave(hex32(4)?),
                    Some(&"missing") => SavedResolution::Missing(hex32(4)?),
                    _ => return Err(bad()),
                };
                let form = match parts.get(5) {
                    Some(&"-") => None,
                    _ => Some(FormId(hex32(5)?)),
                };
                let count_delta = parts.get(6).and_then(|s| s.parse().ok()).ok_or_else(bad)?;
                if parts.len() != 7 {
                    return Err(bad());
                }
                inventory.items.push(SavedItem {
                    saved: hex32(2)?,
                    resolution,
                    form,
                    count_delta,
                    stacks: Vec::new(),
                });
            }
            "invstack" => {
                if parts.len() != 2 {
                    return Err(bad());
                }
                let item = inventory.items.last_mut().ok_or_else(bad)?;
                item.stacks.push(SavedStack::default());
            }
            _ => {
                let extra = match (parts.get(2), parts.len()) {
                    (Some(&"count"), 4) => SavedExtra::Count(parts[3].parse().map_err(|_| bad())?),
                    (Some(&"health"), 4) => SavedExtra::Health(hex32(3)?),
                    (Some(&"worn"), 3) => SavedExtra::Worn,
                    (Some(&"opaque"), 5) => {
                        let kind = u8::from_str_radix(parts[3], 16).map_err(|_| bad())?;
                        let bytes = unhex(parts[4]).ok_or_else(bad)?;
                        // The bytes start with the type byte they were saved under.
                        if bytes.first() != Some(&kind) {
                            return Err(bad());
                        }
                        SavedExtra::Opaque { kind, bytes }
                    }
                    _ => return Err(bad()),
                };
                let stack = inventory
                    .items
                    .last_mut()
                    .and_then(|i| i.stacks.last_mut())
                    .ok_or_else(bad)?;
                stack.extras.push(extra);
            }
        }
        Ok(())
    })();
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> SavedInventories {
        let mut stacks = SavedInventories::new();
        stacks.insert(
            FormId(0x14),
            SavedInventory {
                version: 15,
                items: vec![
                    SavedItem {
                        saved: 4,
                        resolution: SavedResolution::InSave(0x410),
                        form: Some(FormId(0x410)),
                        count_delta: -2,
                        stacks: vec![
                            SavedStack {
                                extras: vec![
                                    SavedExtra::Count(2),
                                    SavedExtra::Health(42.5f32.to_bits()),
                                    SavedExtra::Worn,
                                ],
                            },
                            SavedStack::default(),
                            SavedStack {
                                extras: vec![
                                    SavedExtra::Count(0),
                                    // A signaling NaN's bits stay as they are.
                                    SavedExtra::Health(0x7F80_0001),
                                    SavedExtra::Opaque {
                                        kind: 0x3E,
                                        bytes: vec![0x3E, b'|'],
                                    },
                                ],
                            },
                        ],
                    },
                    SavedItem {
                        saved: 0,
                        resolution: SavedResolution::Null,
                        form: None,
                        count_delta: 1,
                        stacks: Vec::new(),
                    },
                    SavedItem {
                        saved: 0x99,
                        resolution: SavedResolution::Missing(0x99),
                        form: None,
                        count_delta: 0,
                        stacks: vec![SavedStack::default()],
                    },
                ],
            },
        );
        stacks.insert(FormId(0x0500), SavedInventory::default());
        stacks
    }

    #[test]
    fn stacks_go_through_their_save_lines_unchanged() {
        let stacks = sample();
        let mut lines = Vec::new();
        save_lines(&stacks, &mut |l| lines.push(l));
        let mut back = SavedInventories::new();
        for l in &lines {
            load_line(&mut back, l).unwrap().unwrap();
        }
        assert_eq!(back, stacks);
        let item = &back[&FormId(0x14)].items[0];
        assert_eq!(item.stacks[0].count(), Some(2));
        assert_eq!(item.stacks[0].health_bits(), Some(42.5f32.to_bits()));
        assert!(item.stacks[0].worn());
        assert_eq!(item.stacks[1].count(), None);
        assert_eq!(item.stacks[2].count(), Some(0));
        assert_eq!(item.stacks[2].health_bits(), Some(0x7F80_0001));
    }

    #[test]
    fn misplaced_or_malformed_lines_are_refused() {
        let mut stacks = SavedInventories::new();
        assert!(load_line(&mut stacks, "item 00000014 00000410 1").is_none());
        assert!(load_line(&mut stacks, "invstack 00000014")
            .unwrap()
            .is_err());
        load_line(&mut stacks, "invholder 00000014 15")
            .unwrap()
            .unwrap();
        assert!(load_line(&mut stacks, "invholder 00000014 15")
            .unwrap()
            .is_err());
        assert!(load_line(&mut stacks, "invstack 00000014")
            .unwrap()
            .is_err());
        assert!(load_line(&mut stacks, "invitem 00000014 4 save 410 - x")
            .unwrap()
            .is_err());
        load_line(&mut stacks, "invitem 00000014 4 save 410 00000410 1")
            .unwrap()
            .unwrap();
        assert!(load_line(&mut stacks, "invextra 00000014 worn")
            .unwrap()
            .is_err());
        load_line(&mut stacks, "invstack 00000014")
            .unwrap()
            .unwrap();
        assert!(load_line(&mut stacks, "invextra 00000014 opaque 3E 3E7")
            .unwrap()
            .is_err());
        assert!(load_line(&mut stacks, "invextra 00000014 opaque 3E 167C")
            .unwrap()
            .is_err());
        load_line(&mut stacks, "invextra 00000014 opaque 3E 3E7C")
            .unwrap()
            .unwrap();
    }
}

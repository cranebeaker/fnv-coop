//! Sounds: sound records (`SOUN`), the sounds doors make, and the loops a
//! place plays (acoustic spaces, `ASPC`).
//!
//! A sound record: `FNAM`, the file under `sound\` (or a folder, one of
//! whose files is played at random), `SNDD` its settings: smallest and
//! largest attenuation distances (bytes, × 5 and × 100 units), frequency
//! shift, an unused byte, flags (u32: 0x10 loops, 0x40 plays in 2D…), then
//! the rest. A door: `SNAM` its opening sound, `ANAM` its closing one. An
//! acoustic space: four `SNAM` loops, for dawn, day, dusk and night, then
//! a crowd one; a cell names its own in `XCAS`.

use esm::{FormId, FourCC, LoadOrder};

use crate::cell::le_u32;

const SOUN: FourCC = FourCC::new(b"SOUN");
const FNAM: FourCC = FourCC::new(b"FNAM");
const SNDD: FourCC = FourCC::new(b"SNDD");
const SNAM: FourCC = FourCC::new(b"SNAM");
const ANAM: FourCC = FourCC::new(b"ANAM");
const XCAS: FourCC = FourCC::new(b"XCAS");
const YNAM: FourCC = FourCC::new(b"YNAM");
const ZNAM: FourCC = FourCC::new(b"ZNAM");
const QNAM: FourCC = FourCC::new(b"QNAM");
const ETYP: FourCC = FourCC::new(b"ETYP");

/// `SNDD` flag: plays over and over.
pub const LOOPS: u32 = 0x10;

/// A sound record.
#[derive(Debug, Clone, PartialEq)]
pub struct Sound {
    pub form_id: FormId,
    /// Relative to `Data`: `sound\fx\...wav`, or a folder to pick from.
    pub file: String,
    pub flags: u32,
    /// Heard fully within this many units, not at all past the largest.
    pub min_distance: f32,
    pub max_distance: f32,
}

impl Sound {
    pub fn load(order: &LoadOrder, id: FormId) -> Option<Sound> {
        let rr = order.get(id).filter(|r| r.entry.header.kind == SOUN)?;
        let record = rr.record().ok()?;
        let file = record.get(FNAM)?.zstring();
        if file.is_empty() {
            return None;
        }
        let sndd = record.get(SNDD).map(|s| s.data.clone()).unwrap_or_default();
        let byte = |i: usize| sndd.get(i).copied().unwrap_or(0);
        Some(Sound {
            form_id: id,
            file: format!("sound\\{}", file.to_ascii_lowercase()),
            flags: if sndd.len() >= 8 { le_u32(&sndd, 4) } else { 0 },
            min_distance: f32::from(byte(0)) * 5.0,
            max_distance: f32::from(byte(1)) * 100.0,
        })
    }

    pub fn loops(&self) -> bool {
        self.flags & LOOPS != 0
    }

    /// Whether `file` names a folder (one of its files plays).
    pub fn is_folder(&self) -> bool {
        !(self.file.ends_with(".wav") || self.file.ends_with(".ogg") || self.file.ends_with(".mp3"))
    }
}

fn form_at(order: &LoadOrder, id: FormId, kind: FourCC) -> Vec<FormId> {
    let Some(rr) = order.get(id) else {
        return Vec::new();
    };
    let Ok(record) = rr.record() else {
        return Vec::new();
    };
    record
        .get_all(kind)
        .filter(|s| s.data.len() >= 4)
        .map(|s| rr.plugin.to_global(FormId(le_u32(&s.data, 0))))
        .collect()
}

/// A door's opening and closing sounds (`SNAM`, `ANAM` on its base).
pub fn door_sounds(order: &LoadOrder, door_base: FormId) -> (Option<FormId>, Option<FormId>) {
    let first = |kind| {
        form_at(order, door_base, kind)
            .into_iter()
            .find(|f| f.0 != 0)
    };
    (first(SNAM), first(ANAM))
}

/// The sound an item makes when picked up (`YNAM` on its record).
pub fn pickup_sound(order: &LoadOrder, item: FormId) -> Option<FormId> {
    form_at(order, item, YNAM).into_iter().find(|f| f.0 != 0)
}

/// The sound an item makes going into or out of an inventory (`008adcf0`):
/// its own pick-up (`YNAM`) or put-down (`ZNAM`) sound; else a weapon's by
/// its equip type (`ETYP`: 0 big guns `UIItemGunsBigUp`/`Down`, 2 small
/// guns `UIItemGunsSmallUp`/`Down`, 3 melee `UIItemMeleeUp`/`Down`); else
/// `UIItemGenericUp`/`Down`.
pub fn item_sound(order: &LoadOrder, item: FormId, up: bool) -> Option<FormId> {
    let own = if up {
        pickup_sound(order, item)
    } else {
        form_at(order, item, ZNAM).into_iter().find(|f| f.0 != 0)
    };
    if own.is_some() {
        return own;
    }
    let rr = order.get(item)?;
    let record = rr.record().ok()?;
    let equip_type = record
        .get(ETYP)
        .filter(|s| s.data.len() >= 4)
        .map(|s| le_u32(&s.data, 0) as i32);
    let kind = if rr.entry.header.kind.as_bytes() == b"WEAP" {
        match equip_type {
            Some(0) => "GunsBig",
            Some(2) => "GunsSmall",
            Some(3) => "Melee",
            _ => "Generic",
        }
    } else {
        "Generic"
    };
    let name = format!("UIItem{kind}{}", if up { "Up" } else { "Down" });
    order.form_by_editor_id(&name)
}

/// The sound a container or a body makes opening or closing (`0075baf0`):
/// a container's (`CONT`) `SNAM` / `QNAM`; people and creatures
/// `DRSBodyGenericOpen` / `DRSBodyGenericClose`.
pub fn container_sound(order: &LoadOrder, base: FormId, opening: bool) -> Option<FormId> {
    let rr = order.get(base)?;
    match rr.entry.header.kind.as_bytes() {
        b"CONT" => form_at(order, base, if opening { SNAM } else { QNAM })
            .into_iter()
            .find(|f| f.0 != 0),
        b"NPC_" | b"CREA" => order.form_by_editor_id(if opening {
            "DRSBodyGenericOpen"
        } else {
            "DRSBodyGenericClose"
        }),
        _ => None,
    }
}

/// The loop an interior plays at an hour: its acoustic space's (`XCAS`)
/// for dawn, day, dusk or night. The hours are New Vegas's climate's
/// sunrise and sunset (6–8 and 18–20, `TNAM`); which loop the game picks
/// between them isn't traced.
pub fn ambient_loop(order: &LoadOrder, cell: FormId, hour: f32) -> Option<FormId> {
    let space = form_at(order, cell, XCAS).into_iter().next()?;
    let loops = form_at(order, space, SNAM);
    let slot = match hour {
        h if (6.0..8.0).contains(&h) => 0,
        h if (8.0..18.0).contains(&h) => 1,
        h if (18.0..20.0).contains(&h) => 2,
        _ => 3,
    };
    loops
        .get(slot)
        .copied()
        .filter(|f| f.0 != 0)
        .or_else(|| loops.first().copied().filter(|f| f.0 != 0))
}

#[cfg(test)]
mod tests {
    use super::*;
    use testdata::{group, record, sub, zstr};

    /// `008adcf0`, `0075baf0`: an item's own sound first, a weapon's by its
    /// equip type, else the generic ones; containers' and bodies' sounds.
    #[test]
    fn item_and_container_sounds() {
        let dir = std::env::temp_dir().join(format!("nv-rs-sound-items-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        let named = |kind: &[u8; 4], id: u32, edid: &str, rest: &[u8]| {
            let mut d = sub(b"EDID", &zstr(edid));
            d.extend(rest);
            record(kind, id, &d)
        };
        let mut plugin = record(
            b"TES4",
            0,
            &sub(b"HEDR", &{
                let mut h = 1.34f32.to_le_bytes().to_vec();
                h.extend([0; 8]);
                h
            }),
        );
        let mut sounds = Vec::new();
        for (id, edid) in [
            (0x900, "UIItemGunsSmallUp"),
            (0x901, "UIItemGenericUp"),
            (0x902, "UIItemGenericDown"),
            (0x903, "OwnPickUp"),
            (0x904, "ChestOpen"),
            (0x905, "ChestClose"),
            (0x906, "DRSBodyGenericOpen"),
        ] {
            sounds.extend(named(b"SOUN", id, edid, &sub(b"FNAM", &zstr("fx\\a.wav"))));
        }
        plugin.extend(group(*b"SOUN", 0, &sounds));
        let mut weapons = named(b"WEAP", 0x800, "Pistol", &sub(b"ETYP", &2u32.to_le_bytes()));
        weapons.extend(named(
            b"WEAP",
            0x801,
            "Laser",
            &sub(b"ETYP", &1u32.to_le_bytes()),
        ));
        plugin.extend(group(*b"WEAP", 0, &weapons));
        plugin.extend(group(
            *b"MISC",
            0,
            &named(
                b"MISC",
                0x810,
                "Can",
                &sub(b"YNAM", &0x903u32.to_le_bytes()),
            ),
        ));
        let mut chest = sub(b"SNAM", &0x904u32.to_le_bytes());
        chest.extend(sub(b"QNAM", &0x905u32.to_le_bytes()));
        plugin.extend(group(*b"CONT", 0, &named(b"CONT", 0x820, "Chest", &chest)));
        plugin.extend(group(*b"NPC_", 0, &named(b"NPC_", 0x830, "Body", &[])));
        std::fs::write(dir.join("FalloutNV.esm"), &plugin).unwrap();
        let order = LoadOrder::from_data_dir(&dir, &esm::ActivePlugins::OfficialOnly).unwrap();
        assert_eq!(item_sound(&order, FormId(0x800), true), Some(FormId(0x900)));
        // Energy weapons (equip type 1) take the generic sound.
        assert_eq!(item_sound(&order, FormId(0x801), true), Some(FormId(0x901)));
        assert_eq!(
            item_sound(&order, FormId(0x801), false),
            Some(FormId(0x902))
        );
        assert_eq!(item_sound(&order, FormId(0x810), true), Some(FormId(0x903)));
        assert_eq!(
            container_sound(&order, FormId(0x820), true),
            Some(FormId(0x904))
        );
        assert_eq!(
            container_sound(&order, FormId(0x820), false),
            Some(FormId(0x905))
        );
        assert_eq!(
            container_sound(&order, FormId(0x830), true),
            Some(FormId(0x906))
        );
        assert_eq!(container_sound(&order, FormId(0x830), false), None);
        let _ = std::fs::remove_dir_all(dir);
    }
}

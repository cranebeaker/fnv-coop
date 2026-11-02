//! Which animation file plays a group for an actor: the 16-bit group ids
//! (the group with a weapon kind and a movement kind), the kinds a file's
//! name gives, the set of files an actor's 3D has loaded, and the game's
//! lookup with its fallbacks (a sneaking pistol walk falls back to the
//! sneak walk, a rifle's sidestep to the pistol's, then the plain one, …).
//!
//! Read from FalloutNV.exe 1.4.0.525: the id (`005f2370`, its parts
//! `005f2400`/`005f23c0`), the file name's kinds (`005f38d0`, the names at
//! `01197794`/`011977a4`), the group from the sequence's name (the text key
//! parser `005f3a20`), the lookup (`00495740`), and the weapon kinds of the
//! weapons' animation types (`0118a838`). Names marked (Xbox PDB) come from
//! the prototype's symbols.

use std::collections::HashMap;

use super::{group, group_named, GROUPS};

/// No group (`0xff`).
pub const NONE: u16 = 0xff;

/// The movement kinds (bits 12–14 of an id): normal, sneaking (movement
/// flag 0x400), swimming (0x800), the third (0x2000). Their file name
/// prefixes are at `01197794`.
pub mod movement_kind {
    pub const NORMAL: u8 = 0;
    pub const SNEAK: u8 = 1;
    pub const SWIM: u8 = 2;
    pub const FLY: u8 = 3;
}

/// The movement kinds' file name prefixes (`01197794`, index 0 empty).
pub const MOVEMENT_KIND_NAMES: [&str; 4] = ["", "Sneak", "Swim", "Fly"];

/// The weapon kinds' file name prefixes (`011977a4`, index 0 empty: the
/// plain `mt` files have no kind).
pub const WEAPON_KIND_NAMES: [&str; 12] = [
    "", "H2H", "1HM", "2HM", "1HP", "2HR", "2HA", "2HH", "2HL", "1GT", "1MD", "1LM",
];

/// The weapon kind of each weapon animation type (`WEAP` `DNAM` byte 0,
/// `HandToHandMelee` … `OneHandThrown`), the table at `0118a838`: energy
/// pistols play the pistol's, energy rifles the rifle's, thrown weapons the
/// grenade's; `OneHandMine` is `1MD`, `OneHandLunchboxMine` `1LM`.
pub const WEAPON_KIND_BY_TYPE: [u8; 14] = [1, 2, 3, 4, 4, 5, 6, 5, 7, 8, 9, 10, 11, 9];

/// A weapon animation type's weapon kind (`0118a838`); 0 past the table.
pub fn weapon_kind(animation_type: u32) -> u8 {
    WEAPON_KIND_BY_TYPE
        .get(animation_type as usize)
        .copied()
        .unwrap_or(0)
}

/// A group id (`005f2370`): the group in the low byte, the weapon kind in
/// bits 8–11, the movement kind in bits 12–14 and the power-armour flag in
/// bit 15.
// Translated from 005f2370 (decompiled, FalloutNV.exe 1.4.0.525)
pub fn id(movement: u8, weapon: u8, group: u8, power_armor: bool) -> u16 {
    (u16::from(power_armor) << 15)
        | (u16::from(movement & 7) << 12)
        | (u16::from(weapon & 0xf) << 8)
        | u16::from(group)
}

/// An id's group (`005f2440`).
pub fn group_of(id: u16) -> u8 {
    (id & 0xff) as u8
}

/// An id's weapon kind (`005f2400`).
pub fn weapon_kind_of(id: u16) -> u8 {
    ((id & 0xf00) >> 8) as u8
}

/// An id's movement kind (`005f23c0`).
pub fn movement_kind_of(id: u16) -> u8 {
    ((id & 0x7000) >> 12) as u8
}

/// The iron-sights groups (`005f2750`): each is three past its plain one
/// (`AimIS` 20 → `Aim` 17).
// Translated from 005f2750 (decompiled, FalloutNV.exe 1.4.0.525)
pub fn is_iron_sights(group: u8) -> bool {
    matches!(
        group,
        0x14..=0x16
            | 0x1d..=0x1f
            | 0x23..=0x25
            | 0x29..=0x2b
            | 0x2f..=0x31
            | 0x35..=0x37
            | 0x3b..=0x3d
            | 0x41..=0x43
            | 0x47..=0x49
            | 0x4d..=0x4f
            | 0x53..=0x55
            | 0x59..=0x5b
            | 0x69..=0x6b
            | 0x6f..=0x71
            | 0x75..=0x77
            | 0x7b..=0x7d
            | 0x81..=0x83
            | 0x87..=0x89
            | 0x8d..=0x8f
            | 0x93..=0x95
    )
}

/// What a file's name says about the group it plays (`005f38d0`): a `PA`
/// prefix (power armour), then a movement kind's name, then a weapon
/// kind's (`sneak1hpforward.kf`: sneaking, pistol).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct FileKinds {
    pub power_armor: bool,
    pub movement: u8,
    pub weapon: u8,
}

/// The kinds of the file at `path` (`005f38d0`: only a path with a folder
/// is read; the name after the last `\`).
// Translated from 005f38d0 (decompiled, FalloutNV.exe 1.4.0.525)
pub fn file_kinds(path: &str) -> FileKinds {
    let mut k = FileKinds::default();
    let Some(slash) = path.rfind('\\') else {
        return k;
    };
    let mut name = &path[slash + 1..];
    let starts = |s: &str, prefix: &str| {
        s.len() >= prefix.len()
            && s.as_bytes()[..prefix.len()].eq_ignore_ascii_case(prefix.as_bytes())
    };
    if starts(name, "PA") {
        k.power_armor = true;
        name = &name[2..];
    }
    for (i, prefix) in MOVEMENT_KIND_NAMES.iter().enumerate().skip(1) {
        if starts(name, prefix) {
            k.movement = i as u8;
            name = &name[prefix.len()..];
            break;
        }
    }
    for (i, prefix) in WEAPON_KIND_NAMES.iter().enumerate().skip(1) {
        if starts(name, prefix) {
            k.weapon = i as u8;
            break;
        }
    }
    k
}

/// The animations an actor's 3D has: per group id, the files that play it
/// (more than one makes the game's `AnimSequenceMultiple`, Xbox PDB, which
/// plays one of them at random: `0048f450`).
#[derive(Debug, Clone, Default, PartialEq)]
pub struct AnimSet {
    by_id: HashMap<u16, Vec<String>>,
}

impl AnimSet {
    /// Adds a file whose sequence is named `sequence` (the group, by its
    /// name: `005f3a20`) with the kinds its path gives (`005f38d0`). The id
    /// it plays, or none when no group has that name.
    pub fn add(&mut self, path: &str, sequence: &str) -> Option<u16> {
        let g = group_named(sequence)?;
        let k = file_kinds(path);
        let id = id(k.movement, k.weapon, g, k.power_armor);
        let files = self.by_id.entry(id).or_default();
        if !files.iter().any(|f| f.eq_ignore_ascii_case(path)) {
            files.push(path.to_string());
        }
        Some(id)
    }

    /// Whether the 3D has the exact id (`0049c390` with a sequence).
    pub fn has(&self, id: u16) -> bool {
        self.by_id.get(&id).is_some_and(|f| !f.is_empty())
    }

    /// The files that play an exact id.
    pub fn files(&self, id: u16) -> &[String] {
        self.by_id.get(&id).map_or(&[], Vec::as_slice)
    }

    /// The file to play for an exact id: the only one, or one of several
    /// by `pick` (the game's random draw, `0048f450`: the draw modulo the
    /// count).
    pub fn file(&self, id: u16, pick: u32) -> Option<&str> {
        let files = self.files(id);
        if files.is_empty() {
            return None;
        }
        Some(files[pick as usize % files.len()].as_str())
    }

    /// Whether an exact id has one file only (`AnimSequenceSingle`, Xbox
    /// PDB): the movement rate is only read off such a group (`00494300`).
    pub fn single(&self, id: u16) -> bool {
        self.files(id).len() == 1
    }

    /// How many ids the set has.
    pub fn len(&self) -> usize {
        self.by_id.len()
    }

    pub fn is_empty(&self) -> bool {
        self.by_id.is_empty()
    }

    /// The id that plays for `id` (`00495740`, for anyone but the
    /// first-person player): the id itself if the 3D has it; else, for the
    /// power-armour flag, the id without it (or its plain version for an
    /// iron-sights group); an iron-sights group falls back to its plain
    /// group; a weapon kind other than the one-handed melee and pistol falls
    /// back, for groups outside the weapon up/down sections, to the
    /// one-handed melee kind (two-handed melee) or the pistol's (the rest),
    /// then to no weapon kind; a run to the walk of the same kinds; a
    /// movement kind (sneaking …) to the same group without it; and last of
    /// all the kinds' idle, else the plain idle (0). The weapon up/down
    /// groups have no fallback (0).
    // Translated from 00495740 (decompiled, FalloutNV.exe 1.4.0.525)
    pub fn lookup(&self, id: u16) -> u16 {
        self.find(id, false)
    }

    fn find(&self, id: u16, nested: bool) -> u16 {
        if self.has(id) {
            return id;
        }
        let g = group_of(id);
        if id & 0x8000 != 0 {
            let c = self.find(id & 0x7fff, true);
            if group_of(c) == g || (is_iron_sights(g) && group_of(c) == g.wrapping_sub(3)) {
                return c;
            }
        }
        if is_iron_sights(g) {
            return self.find(id - 3, true);
        }
        let section = GROUPS.get(usize::from(g)).map_or(0, |e| e.1);
        if (5..=6).contains(&section) {
            return 0;
        }
        if id & 0xf00 != 0 {
            let kind = weapon_kind_of(id);
            if kind != 2 {
                let other = match kind {
                    3 => Some(0x200),
                    4 => None,
                    _ => Some(0x400),
                };
                if let Some(other) = other {
                    let c = id & 0xf0ff | other;
                    if self.has(c) {
                        return c;
                    }
                }
            }
            let c = id & 0xf0ff;
            if self.has(c) {
                return c;
            }
        }
        if id & 0x8000 != 0 {
            let c = self.find(id & 0x7fff, true);
            if group_of(c) == g {
                return c;
            }
        }
        if id != 0 {
            let walk = match g {
                group::FAST_FORWARD..=group::FAST_RIGHT => Some(id & 0x7f00 | u16::from(g - 4)),
                _ => None,
            };
            if let Some(walk) = walk {
                let c = self.find(walk, true);
                if group_of(c) == group_of(walk) {
                    return c;
                }
            }
        }
        if nested {
            return 0;
        }
        if id & 0x7000 != 0 {
            let c = self.find(id & 0xfff, true);
            if group_of(c) == g {
                return c;
            }
        }
        if g == 0 {
            0
        } else {
            self.find(id & 0x7f00, false)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn set(files: &[(&str, &str)]) -> AnimSet {
        let mut s = AnimSet::default();
        for (path, seq) in files {
            s.add(path, seq);
        }
        s
    }

    #[test]
    fn ids_pack_the_kinds_as_the_exe() {
        assert_eq!(id(0, 4, group::FORWARD, false), 0x0403);
        assert_eq!(id(1, 0, group::IDLE, false), 0x1000);
        assert_eq!(id(2, 5, group::AIM, true), 0xa511);
        let i = id(1, 4, group::FAST_LEFT, false);
        assert_eq!(
            (group_of(i), weapon_kind_of(i), movement_kind_of(i)),
            (group::FAST_LEFT, 4, 1)
        );
        assert!(is_iron_sights(20) && !is_iron_sights(17) && is_iron_sights(0x95));
    }

    #[test]
    fn file_names_give_the_kinds() {
        let k = |p: &str| file_kinds(p);
        assert_eq!(
            k("characters\\_male\\sneak1hpforward.kf"),
            FileKinds {
                power_armor: false,
                movement: 1,
                weapon: 4
            }
        );
        assert_eq!(
            k("characters\\_male\\locomotion\\mtidle.kf"),
            FileKinds::default()
        );
        assert_eq!(
            k("characters\\_male\\pasneak2hrequip.kf"),
            FileKinds {
                power_armor: true,
                movement: 1,
                weapon: 5
            }
        );
        assert_eq!(k("characters\\_male\\h2haim.kf").weapon, 1);
        assert_eq!(k("characters\\_male\\1mdequip.kf").weapon, 10);
        // No folder: nothing read.
        assert_eq!(k("1hpaim.kf"), FileKinds::default());
        // The weapon types' kinds: energy pistols as pistols, energy rifles
        // as rifles, the two mines apart, thrown as grenades.
        assert_eq!(weapon_kind(3), 4);
        assert_eq!(weapon_kind(4), 4);
        assert_eq!(weapon_kind(7), 5);
        assert_eq!((weapon_kind(11), weapon_kind(12)), (10, 11));
        assert_eq!(weapon_kind(13), 9);
        assert_eq!(weapon_kind(99), 0);
        // The files' prefixes named by the actor code follow the table.
        for t in 0..14u32 {
            assert_eq!(
                crate::actor::first_person_kind(Some(t)).to_ascii_uppercase(),
                WEAPON_KIND_NAMES[usize::from(weapon_kind(t))],
                "type {t}"
            );
        }
    }

    #[test]
    fn the_lookup_falls_back_as_the_game_does() {
        let s = set(&[
            ("c\\locomotion\\mtidle.kf", "Idle"),
            ("c\\locomotion\\male\\mtforward.kf", "Forward"),
            ("c\\locomotion\\male\\mtbackward.kf", "Backward"),
            ("c\\locomotion\\male\\mtfastforward.kf", "FastForward"),
            ("c\\locomotion\\1hpforward.kf", "Forward"),
            ("c\\locomotion\\1hpfastforward.kf", "FastForward"),
            ("c\\locomotion\\1hpleft.kf", "Left"),
            ("c\\locomotion\\2hrforward.kf", "Forward"),
            ("c\\sneakmtforward.kf", "Forward"),
            ("c\\sneakmtidle.kf", "Idle"),
            ("c\\h2hidle.kf", "Idle"),
            ("c\\1hpaim.kf", "Aim"),
            ("c\\1hpaimis.kf", "AimIS"),
            ("c\\1hmequip.kf", "Equip"),
            ("c\\2hraim.kf", "Aim"),
        ]);
        // Exact.
        assert_eq!(s.lookup(0x0403), 0x0403);
        // A rifle's left step: the pistol's.
        assert_eq!(s.lookup(id(0, 5, group::LEFT, false)), 0x0405);
        // A rifle's backward step: no rifle's, no pistol's: the plain one.
        assert_eq!(s.lookup(id(0, 5, group::BACKWARD, false)), 0x0004);
        // Two-handed melee equip: the one-handed melee one.
        assert_eq!(s.lookup(id(0, 3, group::EQUIP, false)), 0x0218);
        // Sneaking with a pistol: the sneak walk.
        assert_eq!(s.lookup(id(1, 4, group::FORWARD, false)), 0x1003);
        // Sneaking, running: no sneak run: the sneak walk.
        assert_eq!(s.lookup(id(1, 0, group::FAST_FORWARD, false)), 0x1003);
        // Sneaking backward: no sneak file: the plain backward.
        assert_eq!(s.lookup(id(1, 0, group::BACKWARD, false)), 0x0004);
        // A rifle's run: the pistol's.
        assert_eq!(s.lookup(id(0, 5, group::FAST_FORWARD, false)), 0x0407);
        // A thrown weapon's walk: no pistol walk either way round, the
        // plain one.
        assert_eq!(s.lookup(id(0, 9, group::BACKWARD, false)), 0x0004);
        // Unarmed's idle exists; the pistol's idle falls to the plain one.
        assert_eq!(s.lookup(id(0, 1, group::IDLE, false)), 0x0100);
        assert_eq!(s.lookup(id(0, 4, group::IDLE, false)), 0x0000);
        assert_eq!(s.lookup(id(1, 0, group::IDLE, false)), 0x1000);
        // Iron sights fall back to the plain aim of the kind.
        assert_eq!(s.lookup(id(0, 5, 20, false)), 0x0511);
        assert_eq!(s.lookup(id(0, 4, 20, false)), 0x0414);
        // Power armour: the same without it.
        assert_eq!(s.lookup(id(0, 4, group::FORWARD, true)), 0x0403);
        // Nothing for a turn: the kind's idle.
        assert_eq!(s.lookup(id(0, 1, group::TURN_LEFT, false)), 0x0100);
        assert_eq!(s.lookup(id(0, 0, group::TURN_LEFT, false)), 0x0000);
        // Aim up has no fallback.
        assert_eq!(s.lookup(id(0, 4, 18, false)), 0);
    }

    #[test]
    fn several_files_for_one_group_are_drawn_from() {
        let s = set(&[
            ("c\\1hmattackright_a.kf", "AttackRight"),
            ("c\\1hmattackright_b.kf", "AttackRight"),
            ("c\\1hmaim.kf", "Aim"),
            ("c\\not_a_group.kf", "SpecialIdle_Fidget"),
        ]);
        let i = id(0, 2, group::ATTACK_RIGHT, false);
        assert_eq!(s.files(i).len(), 2);
        assert!(!s.single(i));
        assert_eq!(s.file(i, 0), Some("c\\1hmattackright_a.kf"));
        assert_eq!(s.file(i, 3), Some("c\\1hmattackright_b.kf"));
        assert!(s.single(id(0, 2, group::AIM, false)));
        assert_eq!(s.len(), 2);
    }
}

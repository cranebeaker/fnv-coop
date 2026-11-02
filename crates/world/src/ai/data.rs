//! A package's type-specific data (`TESPackageData` subclasses, names from
//! the Xbox PDB), read as the package loader `00673a40` reads it:
//! `PKW3` use-weapon data, `PTD2` the use-weapon attack target, `PKPT`
//! patrol data, `PKE2` escort radius, `PKFD` follow radius, and the second
//! location (`PLD2`), which each type files in its own data (`00671e10`).
//! `PKAM` and `PKED` are markers the loader has no case for.

use esm::{FormId, FourCC, Record};

use super::{read_location, Location};
use crate::cell::{le_f32, le_u32};

/// `TESUseWeaponPackageData` (Xbox PDB), from `PKW3` (24 bytes; `0067cb70`)
/// with `PLD2` as its target location and `PTD2` its attack target.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct UseWeapon {
    pub always_hit: bool,
    pub do_no_damage: bool,
    pub crouch: bool,
    pub hold_fire: bool,
    pub volley_fire: bool,
    pub repeat_fire: bool,
    pub burst_count: u16,
    pub volley_shots_min: u16,
    pub volley_shots_max: u16,
    pub volley_wait_min: f32,
    pub volley_wait_max: f32,
    pub weapon: Option<FormId>,
    /// `PTD2` (kind, form, value), as `PTDT`.
    pub attack_target: Option<(i32, FormId, i32)>,
    pub target_location: Option<Location>,
}

/// `TESPatrolPackageData` (Xbox PDB), from `PKPT` (2 bytes; `0067c450`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Patrol {
    pub repeatable: bool,
    pub start_at_linked_ref: bool,
}

/// What a package type keeps beside its common fields.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum TypeData {
    #[default]
    None,
    UseWeapon(UseWeapon),
    Patrol(Patrol),
    /// `TESEscortPackageData`: the search location (`PLD2`) and the
    /// follow radius (`PKE2`, i32; `0067bc60`).
    Escort {
        search: Option<Location>,
        follow_radius: i32,
    },
    /// `TESFollowPackageData`: the start location (`PLD2`) and trigger
    /// radius (`PKFD`, f32; `0067c040`).
    Follow {
        start: Option<Location>,
        trigger_radius: f32,
    },
    /// `TESAmbushPackageData`: the ambush location (`PLD2`).
    Ambush {
        location: Option<Location>,
    },
    /// `TESEatPackageData`: the food search location (`PLD2`).
    Eat {
        search: Option<Location>,
    },
    /// Use item at: its second location (`PLD2`, data +4).
    UseItemAt {
        location: Option<Location>,
    },
}

const PKDT: FourCC = FourCC::new(b"PKDT");

/// A package's type data, the subrecords in file order as the loader
/// takes them. `kind` is the record type (`PKDT` byte 4).
// Translated from 00673a40 / 00671e10 (decompiled, FalloutNV.exe
// 1.4.0.525): PKW3 replaces the data with use-weapon data whatever the
// type; PKE2 and PKFD replace it with escort and follow data; PKPT with
// patrol data; PLD2 goes to the dialogue data (types 15, 0x1c; read by
// `super::second_location`), the use-weapon target location (16), and
// otherwise to the type's own data (eat 3, escort 2, ambush 9, follow 1,
// use item at 8), created when missing; PTD2 only for use weapon.
pub fn read(record: &Record, global: impl Fn(FormId) -> FormId) -> TypeData {
    let Some(kind) = record.get(PKDT).and_then(|s| s.data.get(4).copied()) else {
        return TypeData::None;
    };
    let mut data = TypeData::None;
    let form = |raw: u32| (raw != 0).then(|| global(FormId(raw)));
    for sub in &record.subrecords {
        let d = &sub.data;
        match sub.kind.as_bytes() {
            b"PKW3" if d.len() >= 24 => {
                data = TypeData::UseWeapon(UseWeapon {
                    always_hit: d[0] != 0,
                    do_no_damage: d[1] != 0,
                    crouch: d[2] != 0,
                    hold_fire: d[3] != 0,
                    volley_fire: d[4] != 0,
                    repeat_fire: d[5] != 0,
                    burst_count: u16::from_le_bytes([d[6], d[7]]),
                    volley_shots_min: u16::from_le_bytes([d[8], d[9]]),
                    volley_shots_max: u16::from_le_bytes([d[10], d[11]]),
                    volley_wait_min: le_f32(d, 12),
                    volley_wait_max: le_f32(d, 16),
                    weapon: form(le_u32(d, 20)),
                    // A new data object: the loader's constructor clears
                    // both (`0067cb70`); later PLD2/PTD2 fill them.
                    attack_target: None,
                    target_location: None,
                });
            }
            b"PKPT" if d.len() >= 2 => {
                data = TypeData::Patrol(Patrol {
                    repeatable: d[0] != 0,
                    start_at_linked_ref: d[1] != 0,
                });
            }
            b"PKE2" if d.len() >= 4 => {
                data = TypeData::Escort {
                    search: None,
                    follow_radius: le_u32(d, 0) as i32,
                };
            }
            b"PKFD" if d.len() >= 4 => {
                data = TypeData::Follow {
                    start: None,
                    trigger_radius: le_f32(d, 0),
                };
            }
            b"PTD2" if d.len() >= 12 && kind == super::kinds::USE_WEAPON => {
                let k = le_u32(d, 0) as i32;
                let raw = le_u32(d, 4);
                let f = if k == 0 {
                    form(raw).unwrap_or(FormId(0))
                } else {
                    FormId(raw)
                };
                let target = Some((k, f, le_u32(d, 8) as i32));
                match &mut data {
                    TypeData::UseWeapon(w) => w.attack_target = target,
                    other => {
                        *other = TypeData::UseWeapon(UseWeapon {
                            attack_target: target,
                            ..UseWeapon::default()
                        })
                    }
                }
            }
            b"PLD2" if d.len() >= 12 => {
                let loc = Some(read_location(d, &global));
                match kind {
                    // The dialogue data's (`super::second_location`).
                    15 | 0x1c => {}
                    16 => match &mut data {
                        TypeData::UseWeapon(w) => w.target_location = loc,
                        other => {
                            *other = TypeData::UseWeapon(UseWeapon {
                                target_location: loc,
                                ..UseWeapon::default()
                            })
                        }
                    },
                    3 => match &mut data {
                        TypeData::Eat { search } => *search = loc,
                        other => *other = TypeData::Eat { search: loc },
                    },
                    2 => match &mut data {
                        TypeData::Escort { search, .. } => *search = loc,
                        other => {
                            *other = TypeData::Escort {
                                search: loc,
                                follow_radius: 0,
                            }
                        }
                    },
                    9 => match &mut data {
                        TypeData::Ambush { location } => *location = loc,
                        other => *other = TypeData::Ambush { location: loc },
                    },
                    1 => match &mut data {
                        TypeData::Follow { start, .. } => *start = loc,
                        other => {
                            *other = TypeData::Follow {
                                start: loc,
                                trigger_radius: 0.0,
                            }
                        }
                    },
                    8 => match &mut data {
                        TypeData::UseItemAt { location } => *location = loc,
                        other => *other = TypeData::UseItemAt { location: loc },
                    },
                    _ => {}
                }
            }
            _ => {}
        }
    }
    data
}

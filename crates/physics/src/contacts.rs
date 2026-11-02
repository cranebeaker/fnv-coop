//! What the game does when Havok bodies come into contact
//! ([`crate::rigid::ContactEvent`]): its collision listener
//! (`FOCollisionListener::contactPointAddedCallback` (Xbox PDB), `00623cb0`)
//! plays impact sounds (`ImpactMixer::PlayCollisionSound` (Xbox PDB),
//! `00837550`, choosing them by Havok material, `00839e00`) and stores
//! physics damage (`FOCollisionListener::StoreObjectDamage` (Xbox PDB),
//! `006238b0`, by `0062be90`) that it deals later (`DealObjectDamage`,
//! `00623640`).
//!
//! Speeds: the contact's projected velocity (`+0x1c` of Havok's event,
//! Havok units a second). The sounds use it × 6.9991 (game units,
//! `004587d0`); the damage uses it as it is.

/// The settings the contacts use, with the executable's defaults
/// (`settings_all.txt`; `[Audio]` ones from the INI section).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ContactSettings {
    /// `fMinSoundVel:Audio` (10): slower contacts are silent.
    pub min_sound_speed: f32,
    /// `fCollisionSoundHeavyThreshold:Audio` (100): faster ones pick the
    /// heavy ("H") sounds.
    pub heavy: f32,
    /// `f{Wood,Stone,Earth,Skin,Metal}{Medium,Large}MassMin:Audio`.
    pub wood: (f32, f32),
    pub stone: (f32, f32),
    pub earth: (f32, f32),
    pub skin: (f32, f32),
    pub metal: (f32, f32),
    /// `iCollisionSoundTimeDelta:Audio` (333 ms): one sound per pair of
    /// materials this often.
    pub time_delta_ms: u32,
    /// `fPhysicsDamage{1,2,3}Mass` (10, 50, 100), `…Damage` (1, 5, 10),
    /// `…SpeedMin` (500, 350, 150).
    pub damage_mass: [f32; 3],
    pub damage: [f32; 3],
    pub damage_speed_min: [f32; 3],
    /// `fPhysicsDamageSpeedBase` (1), `…SpeedMult` (0.0001),
    /// `fPhysicsDamageSpeedMin` (150).
    pub damage_speed_base: f32,
    pub damage_speed_mult: f32,
    pub damage_speed_floor: f32,
}

impl Default for ContactSettings {
    fn default() -> Self {
        ContactSettings {
            min_sound_speed: 10.0,
            heavy: 100.0,
            wood: (7.0, 15.0),
            stone: (5.0, 30.0),
            earth: (5.0, 30.0),
            skin: (5.0, 30.0),
            metal: (8.0, 25.0),
            time_delta_ms: 333,
            damage_mass: [10.0, 50.0, 100.0],
            damage: [1.0, 5.0, 10.0],
            damage_speed_min: [500.0, 350.0, 150.0],
            damage_speed_base: 1.0,
            damage_speed_mult: 0.0001,
            damage_speed_floor: 150.0,
        }
    }
}

impl ContactSettings {
    /// The settings by name (`get`: game settings and INI values), each
    /// from `get` or its default.
    pub fn read(get: impl Fn(&str) -> Option<f32>) -> Self {
        let d = Self::default();
        let f = |name: &str, default: f32| get(name).unwrap_or(default);
        let pair = |a: &str, b: &str, (x, y): (f32, f32)| (f(a, x), f(b, y));
        ContactSettings {
            min_sound_speed: f("fMinSoundVel", d.min_sound_speed),
            heavy: f("fCollisionSoundHeavyThreshold", d.heavy),
            wood: pair("fWoodMediumMassMin", "fWoodLargeMassMin", d.wood),
            stone: pair("fStoneMediumMassMin", "fStoneLargeMassMin", d.stone),
            earth: pair("fEarthMediumMassMin", "fEarthLargeMassMin", d.earth),
            skin: pair("fSkinMediumMassMin", "fSkinLargeMassMin", d.skin),
            metal: pair("fMetalMediumMassMin", "fMetalLargeMassMin", d.metal),
            time_delta_ms: get("iCollisionSoundTimeDelta").map_or(d.time_delta_ms, |v| v as u32),
            damage_mass: [
                f("fPhysicsDamage1Mass", d.damage_mass[0]),
                f("fPhysicsDamage2Mass", d.damage_mass[1]),
                f("fPhysicsDamage3Mass", d.damage_mass[2]),
            ],
            damage: [
                f("fPhysicsDamage1Damage", d.damage[0]),
                f("fPhysicsDamage2Damage", d.damage[1]),
                f("fPhysicsDamage3Damage", d.damage[2]),
            ],
            damage_speed_min: [
                f("fPhysicsDamage1SpeedMin", d.damage_speed_min[0]),
                f("fPhysicsDamage2SpeedMin", d.damage_speed_min[1]),
                f("fPhysicsDamage3SpeedMin", d.damage_speed_min[2]),
            ],
            damage_speed_base: f("fPhysicsDamageSpeedBase", d.damage_speed_base),
            damage_speed_mult: f("fPhysicsDamageSpeedMult", d.damage_speed_mult),
            damage_speed_floor: f("fPhysicsDamageSpeedMin", d.damage_speed_floor),
        }
    }
}

/// Game units a Havok unit (`011c3d18`, the multiplier `004587d0` uses).
const GAME_PER_HAVOK: f32 = crate::HAVOK_UNIT;
/// The mass a side without a moving body counts as (5.0, `0101712c`).
pub const STATIC_SIDE_MASS: f32 = 5.0;
/// Below this mass a body sounds "static" (1e-4, `01032980`).
const STATIC_MASS: f32 = 1e-4;

/// Whether a contact of `speed` (game units a second) makes a sound: at
/// least `fMinSoundVel` and at least 1.
// Translated from 00623cb0 (decompiled, FalloutNV.exe 1.4.0.525)
pub fn audible(speed: f32, s: &ContactSettings) -> bool {
    speed >= s.min_sound_speed && speed >= 1.0
}

/// The game-unit speed of a Havok projected velocity.
pub fn sound_speed(havok_speed: f32) -> f32 {
    (havok_speed * GAME_PER_HAVOK).abs()
}

/// The sound (an editor ID of a `SOUN` record) one side of a contact
/// makes: by its Havok material, its mass and the contact's speed (game
/// units a second).
// Translated from 00839e00 (decompiled, FalloutNV.exe 1.4.0.525)
pub fn material_sound(
    material: i32,
    mass: f32,
    speed: f32,
    s: &ContactSettings,
) -> Option<&'static str> {
    let sized =
        |(medium, large): (f32, f32), small: &'static str, mid: &'static str, big: &'static str| {
            if mass < medium {
                small
            } else if mass < large {
                mid
            } else {
                big
            }
        };
    let with_static = |still: &'static str, sizes: (f32, f32), small, mid, big| {
        if mass < STATIC_MASS {
            still
        } else {
            sized(sizes, small, mid, big)
        }
    };
    let heavy = speed >= s.heavy;
    let hl = |h: &'static str, l: &'static str| if heavy { h } else { l };
    Some(match material {
        0 | 0xf => with_static(
            "CStoneStatic",
            s.stone,
            "CStoneSmall",
            "CStoneMedium",
            "CStoneLarge",
        ),
        1 => hl("PHYGenericClothH", "PHYGenericClothL"),
        2 => sized(s.earth, "CEarthSmall", "CEarthMedium", "CEarthLarge"),
        3 => sized((6.0, 15.0), "CGlassSmall", "CGlassMedium", "CGlassLarge"),
        4 => sized((6.0, 70.0), "CGrassSmall", "CGrassMedium", "CGrassLarge"),
        5 if heavy => with_static(
            "PHYGenericMetalStaticH",
            s.metal,
            "PHYGenericMetalSmallH",
            "PHYGenericMetalMediumH",
            "PHYGenericMetalLargeH",
        ),
        5 => with_static(
            "PHYGenericMetalStaticL",
            s.metal,
            "PHYGenericMetalSmallL",
            "PHYGenericMetalMediumL",
            "PHYGenericMetalLargeL",
        ),
        6 => "COrganicSmall",
        7 if heavy => sized(s.skin, "PHYSkinSmallH", "PHYSkinMediumH", "PHYSkinLargeH"),
        7 => sized(s.skin, "PHYSkinSmallL", "PHYSkinMediumL", "PHYSkinLargeL"),
        8 => sized((6.0, 15.0), "CWoodSmall", "CWoodMedium", "CWoodLarge"),
        9 => with_static(
            "CWoodStatic",
            s.wood,
            "CWoodSmall",
            "CWoodMedium",
            "CWoodLarge",
        ),
        10 => "CSpecialHeavyStone",
        11 => "CSpecialHeavyMetal",
        12 => "CSpecialHeavyWood",
        13 => "PHYChain",
        14 => "PHYBottlecaps",
        0x10 => with_static(
            "PHYGenericMetalHollowStatic",
            s.metal,
            "PHYGenericMetalHollowSmall",
            "PHYGenericMetalHollowMedium",
            "PHYGenericMetalHollowLarge",
        ),
        0x11 => with_static(
            "PHYGenericMetalSheetStatic",
            s.metal,
            "PHYGenericMetalSheetSmall",
            "PHYGenericMetalSheetMedium",
            "PHYGenericMetalSheetLarge",
        ),
        0x14 => hl("PHYVehicleMetalBodyH", "PHYVehicleMetalBodyL"),
        0x15 => hl("PHYVehicleMetalSolidH", "PHYVehicleMetalSolidL"),
        0x16 => hl("PHYVehicleMetalHollowH", "PHYVehicleMetalHollowL"),
        0x17 => hl("PHYBarrelH", "PHYBarrelL"),
        0x18 => hl("PHYBottleH", "PHYBottleL"),
        0x19 => hl("PHYCanSodaH", "PHYCanSodaL"),
        0x1a => hl("PHYWeaponPistolH", "PHYWeaponPistolL"),
        0x1b => hl("PHYWeaponRifleH", "PHYWeaponRifleL"),
        0x1c => hl("PHYShoppingCartH", "PHYShoppingCartL"),
        0x1d => hl("PHYLunchboxH", "PHYLunchboxL"),
        0x1e => "PHYBabyRattle",
        0x1f => "PHYRubberBall",
        0x20 => "PHYChainlink",
        0x21 => "PHYTile",
        0x22 => "PHYCarpet",
        0x23 => "PHYTumbleweed",
        _ => return None,
    })
}

/// The sounds of one contact.
#[derive(Debug, Clone, PartialEq)]
pub struct CollisionSound {
    /// Each side's sound (none for a side whose material has none, nor
    /// for skin against skin).
    pub sounds: [Option<&'static str>; 2],
    /// Static attenuation in hundredths of a decibel: (1 − min(speed ÷
    /// 400, 1)) × 3000, rounded.
    pub attenuation: u16,
    /// Frequency multiplier: 1, or (4500 − attenuation) ÷ 3000 from 1500
    /// on. Set through the sound's command 0x42 (`00ad8a90`), read as
    /// `BSSoundHandle::SetFrequency` from the Xbox PDB's method order
    /// (inferred).
    pub frequency: f32,
    /// The pair of materials, `(a + 1) × (b + 1)`: one sound per key per
    /// `iCollisionSoundTimeDelta`.
    pub key: i32,
}

/// The sounds a contact of `speed` (game units a second) between sides
/// `(material, mass)` makes (`mass`: [`STATIC_SIDE_MASS`] for a side
/// without a moving body). Shell casings (layer 25) have their own sounds
/// in the game (`PHYCasing…`) and aren't handled here.
// Translated from 00837550 (decompiled, FalloutNV.exe 1.4.0.525)
pub fn collision_sound(
    (mat_a, mass_a): (i32, f32),
    (mat_b, mass_b): (i32, f32),
    speed: f32,
    s: &ContactSettings,
) -> Option<CollisionSound> {
    let skin_on_skin = mat_a == 7 && mat_b == 7;
    let a = material_sound(mat_a, mass_a, speed, s).filter(|_| !skin_on_skin);
    let b = material_sound(mat_b, mass_b, speed, s).filter(|_| !skin_on_skin);
    if a.is_none() && b.is_none() {
        return None;
    }
    let share = (speed / 400.0).min(1.0);
    let attenuation = ((1.0 - f64::from(share)) * 3000.0).round() as u16;
    let frequency = if attenuation < 1500 {
        1.0
    } else {
        ((f64::from(attenuation) - 4500.0) / -3000.0) as f32
    };
    Some(CollisionSound {
        sounds: [a, b],
        attenuation,
        frequency,
        key: (mat_a + 1) * (mat_b + 1),
    })
}

/// The damage a body of `mass` closing at `speed` (Havok units a second)
/// deals: none under `fPhysicsDamage1Mass`; then the tier's damage ×
/// (`fPhysicsDamageSpeedBase` + speed × `fPhysicsDamageSpeedMult`) once
/// the speed passes the tier's minimum. A fixed body counts as 10000.
// Translated from 0062be90 (decompiled, FalloutNV.exe 1.4.0.525)
pub fn physics_damage(mass: f32, speed: f32, s: &ContactSettings) -> f32 {
    let (damage, min_speed) = if mass < s.damage_mass[0] {
        (0.0, 1000.0)
    } else if mass < s.damage_mass[1] {
        (s.damage[0], s.damage_speed_min[0])
    } else if mass < s.damage_mass[2] {
        (s.damage[1], s.damage_speed_min[1])
    } else {
        (s.damage[2], s.damage_speed_min[2])
    };
    let scale = if min_speed < speed {
        s.damage_speed_base + speed * s.damage_speed_mult
    } else {
        0.0
    };
    damage * scale
}

/// The mass a fixed side (mass 0) counts as for damage (10000,
/// `01022958`).
pub const FIXED_MASS: f32 = 10000.0;

/// Whether a contact closing at `speed` (Havok units a second) is fast
/// enough for physics damage at all (`fPhysicsDamageSpeedMin`).
// Translated from 006238b0 (decompiled, FalloutNV.exe 1.4.0.525)
pub fn damaging(speed: f32, s: &ContactSettings) -> bool {
    speed.abs() >= s.damage_speed_floor
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sounds_are_chosen_by_material_mass_and_speed() {
        let s = ContactSettings::default();
        // The VCG02 bottle (material 24, bottle) on the ground.
        assert_eq!(material_sound(24, 1.0, 50.0, &s), Some("PHYBottleL"));
        assert_eq!(material_sound(24, 1.0, 150.0, &s), Some("PHYBottleH"));
        // Stone ground: a static side counts as mass 5, medium stone.
        assert_eq!(
            material_sound(0, STATIC_SIDE_MASS, 50.0, &s),
            Some("CStoneMedium")
        );
        assert_eq!(material_sound(0, 0.0, 50.0, &s), Some("CStoneStatic"));
        assert_eq!(material_sound(9, 20.0, 50.0, &s), Some("CWoodLarge"));
        assert_eq!(
            material_sound(5, 10.0, 120.0, &s),
            Some("PHYGenericMetalMediumH")
        );
        assert_eq!(material_sound(0x13, 1.0, 50.0, &s), None);
        assert!(audible(10.0, &s) && !audible(9.9, &s));
    }

    #[test]
    fn soft_contacts_are_quieter_and_lower() {
        let s = ContactSettings::default();
        let c = collision_sound((24, 1.0), (2, STATIC_SIDE_MASS), 80.0, &s).unwrap();
        assert_eq!(c.sounds, [Some("PHYBottleL"), Some("CEarthMedium")]);
        assert_eq!(c.attenuation, 2400);
        assert!((c.frequency - 0.7).abs() < 1e-6);
        assert_eq!(c.key, 25 * 3);
        let hard = collision_sound((24, 1.0), (2, 5.0), 500.0, &s).unwrap();
        assert_eq!((hard.attenuation, hard.frequency), (0, 1.0));
        assert!(collision_sound((7, 1.0), (7, 1.0), 100.0, &s).is_none());
    }

    #[test]
    fn heavy_fast_bodies_do_physics_damage() {
        let s = ContactSettings::default();
        assert_eq!(physics_damage(1.0, 2000.0, &s), 0.0);
        assert_eq!(physics_damage(20.0, 400.0, &s), 0.0);
        assert!((physics_damage(20.0, 600.0, &s) - 1.06).abs() < 1e-5);
        assert!((physics_damage(60.0, 400.0, &s) - 5.2).abs() < 1e-5);
        assert!((physics_damage(FIXED_MASS, 200.0, &s) - 10.2).abs() < 1e-4);
        assert!(damaging(150.0, &s) && !damaging(149.0, &s));
    }
}

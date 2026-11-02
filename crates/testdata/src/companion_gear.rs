//! A world for what companions pick to fight with and for the numbers on
//! the item cards (`world::dps`, `world::item_card`): weapons, ammunition,
//! combat styles, aid items and magic effects with the values of the
//! game's own records (`FalloutNV.esm`: the 9mm pistol, Cass's caravan
//! shotgun and knife, the 10mm submachine gun, ED-E's zap guns, the
//! Stimpak, Nuka-Cola, …), a Cass, an ED-E and a guard, all in one
//! interior.

use crate::{f32s, group, placed, record, sub, zstr, TempData};

/// Form IDs in the [`companion_gear`] world.
pub mod ids {
    pub const SETTINGS: u32 = 0xC00;
    /// The followers' combat styles: ranged only (`CSSD` weapon
    /// restrictions 2, `FollowersCombatStyleRanged`'s) and melee only (1,
    /// `FollowersCombatStyleMelee`'s).
    pub const STYLE_RANGED: u32 = 0xC10;
    pub const STYLE_MELEE: u32 = 0xC11;
    pub const BULLET: u32 = 0xC12;
    /// 9mm rounds, Cass's own rounds (`AmmoCompanion`), ED-E's energy
    /// cells, 10mm rounds, 20 gauge shells with the buckshot damage bonus
    /// (× 1.2, `AmmoEffectBuckshotDamageBonus`).
    pub const AMMO_9MM: u32 = 0xC20;
    pub const AMMO_COMPANION: u32 = 0xC21;
    pub const AMMO_CELL: u32 = 0xC22;
    pub const AMMO_10MM: u32 = 0xC23;
    pub const AMMO_20GA: u32 = 0xC24;
    pub const BUCKSHOT_BONUS: u32 = 0xC25;
    /// `WeapNV9mmPistol` (16 damage, clip 13, 3.125 attacks a second,
    /// attack multiplier 1.25, fire rate 1), `WeapNVCaravanShotgunCass`
    /// (54, clip 2, 7 projectiles, 3.214 a second, reload 1.5 s),
    /// `WeapKnifeCombatCass` (15, melee), `Weap10mmSubmachineGun` (19,
    /// automatic, 9 a second, clip 30), `EDEZapGun` (25) and
    /// `EDEZapGunUpgrade` (30, attack multiplier 3.5, 8.75 a second), and a
    /// "player only" pistol (flags2 0x1).
    pub const PISTOL: u32 = 0xC30;
    pub const CASS_SHOTGUN: u32 = 0xC31;
    pub const CASS_KNIFE: u32 = 0xC32;
    pub const SMG: u32 = 0xC33;
    pub const ZAP: u32 = 0xC34;
    pub const ZAP_UPGRADE: u32 = 0xC35;
    pub const PLAYER_ONLY: u32 = 0xC36;
    /// ED-E's weapon list (`EmbeddedWeapons`): the two zap guns.
    pub const EMBEDDED: u32 = 0xC40;
    /// Cass (the ranged style; her shotgun, 15 of her rounds, her knife), ED-E
    /// (a creature with the weapon list; the zap gun and 20 cells), a guard
    /// whose only package has "Weapons Unequipped" (PKDT flags 0x200000;
    /// the 9mm pistol and 20 rounds).
    pub const CASS: u32 = 0xC50;
    pub const EDE: u32 = 0xC51;
    pub const GUARD: u32 = 0xC52;
    pub const UNARMED_PACKAGE: u32 = 0xC53;
    pub const CELL: u32 = 0xC60;
    pub const CASS_REF: u32 = 0xC61;
    pub const EDE_REF: u32 = 0xC62;
    pub const GUARD_REF: u32 = 0xC63;
    /// Magic effects (`MGEF` `DATA` flags, archetype, actor value):
    /// `RestoreHealth` (0x70, 0, 16), `RestoreHealthStimpak` (0x70, 34, 16),
    /// `DamageRadiationLevel` (0x1000075, 0, 54), a Fortify Perception
    /// that recovers (0x72, 0, 6), one shown by its name alone (0x2000,
    /// "Radiation Immunity"), a script effect (archetype 1).
    pub const RESTORE_HEALTH: u32 = 0xC70;
    pub const RESTORE_HEALTH_STIMPAK: u32 = 0xC71;
    pub const DAMAGE_RADS: u32 = 0xC72;
    pub const FORTIFY_PERCEPTION: u32 = 0xC73;
    pub const NAME_ONLY: u32 = 0xC74;
    pub const SCRIPT_EFFECT: u32 = 0xC75;
    /// Aid: a Stimpak (medicine: `RestoreHealthStimpak` 30), a Nuka-Cola
    /// (neither medicine nor food: Restore Health 2 for 25 s, the script
    /// effect, Damage Rads 3), a food (Restore Health 10, Damage Rads 2)
    /// and a chem (the name-only effect, Fortify Perception 2 for 240 s,
    /// Restore Health 1 for 90 s).
    pub const STIMPAK: u32 = 0xC80;
    pub const NUKA_COLA: u32 = 0xC81;
    pub const FOOD: u32 = 0xC82;
    pub const CHEM: u32 = 0xC83;
    /// The actor values' records with their abbreviations: "HP", "Rads",
    /// "PER".
    pub const AV_HEALTH: u32 = 0xC90;
    pub const AV_RADS: u32 = 0xC91;
    pub const AV_PERCEPTION: u32 = 0xC92;
    /// Enchantments (Fortify Perception 1), one flagged "hide effect", and
    /// hats with them.
    pub const ENCHANTMENT: u32 = 0xCA0;
    pub const HIDDEN_ENCHANTMENT: u32 = 0xCA1;
    pub const HAT: u32 = 0xCB0;
    pub const HIDDEN_HAT: u32 = 0xCB1;
    /// A weapon mod with its description.
    pub const WEAPON_MOD: u32 = 0xCC0;
}

/// Writes the world (see [`ids`]); the settings are the master's
/// (`fDamageSkillBase` and `…Mult` 0.5, the first four
/// `fWeaponConditionJam…` 0, `fMagicMedicineSkillMult` and
/// `fMagicSurvivalSkillMult` 2).
pub fn companion_gear(tag: &str) -> TempData {
    use ids::*;
    let data = TempData::new(tag);
    let edid = |s: &str| sub(b"EDID", &zstr(s));
    let named = |kind: &[u8; 4], id: u32, name: &str, rest: &[u8]| {
        let mut d = edid(name);
        d.extend(rest);
        record(kind, id, &d)
    };

    let mut settings = Vec::new();
    for (i, (name, value)) in [
        ("fDamageSkillBase", 0.5f32),
        ("fDamageSkillMult", 0.5),
        ("fWeaponConditionJam1", 0.0),
        ("fWeaponConditionJam2", 0.0),
        ("fWeaponConditionJam3", 0.0),
        ("fWeaponConditionJam4", 0.0),
        ("fMagicMedicineSkillMult", 2.0),
        ("fMagicSurvivalSkillMult", 2.0),
    ]
    .into_iter()
    .enumerate()
    {
        settings.extend(named(
            b"GMST",
            SETTINGS + i as u32,
            name,
            &sub(b"DATA", &value.to_le_bytes()),
        ));
    }

    // Combat styles: CSTD 92 bytes, CSAD 21 floats, CSSD 64 bytes with the
    // weapon restrictions u32 at 40.
    let style = |id: u32, name: &str, restriction: u32| {
        let mut cstd = vec![0u8; 92];
        cstd[0] = 50;
        cstd[1] = 40;
        cstd[36] = 30;
        cstd[37] = 40;
        let mut cssd = f32s(&[2048.0, 100.0, 2.0, 2.0, 10.0, 10.0, 2.0, 2.0, 1.0, 0.0]);
        cssd.extend(restriction.to_le_bytes());
        cssd.extend(f32s(&[1.0, 0.0, 10000.0, 1.0, 1.0]));
        let mut d = sub(b"CSTD", &cstd);
        d.extend(sub(b"CSAD", &f32s(&[0.0; 21])));
        d.extend(sub(b"CSSD", &cssd));
        named(b"CSTY", id, name, &d)
    };
    let mut styles = style(STYLE_RANGED, "TestFollowersCombatStyleRanged", 2);
    styles.extend(style(STYLE_MELEE, "TestFollowersCombatStyleMelee", 1));

    let mut projectile = vec![0u8; 84];
    projectile[0..2].copy_from_slice(&1u16.to_le_bytes());
    projectile[2..4].copy_from_slice(&1u16.to_le_bytes());
    projectile[8..12].copy_from_slice(&23680.0f32.to_le_bytes());
    projectile[12..16].copy_from_slice(&10000.0f32.to_le_bytes());
    let projectiles = named(b"PROJ", BULLET, "TestBullet", &sub(b"DATA", &projectile));

    // Ammunition: DATA speed, flags, 3 unused, value, clip rounds.
    let ammo = |id: u32, name: &str, effect: Option<u32>| {
        let mut d = sub(b"FULL", &zstr(name));
        let mut data = 1.0f32.to_le_bytes().to_vec();
        data.extend([0; 4]);
        data.extend(1i32.to_le_bytes());
        data.push(12);
        d.extend(sub(b"DATA", &data));
        if let Some(e) = effect {
            d.extend(sub(b"RCIL", &e.to_le_bytes()));
        }
        named(b"AMMO", id, &name.replace(' ', ""), &d)
    };
    let mut ammunition = ammo(AMMO_9MM, "9mm Round", None);
    ammunition.extend(ammo(AMMO_COMPANION, "Companion Round", None));
    ammunition.extend(ammo(AMMO_CELL, "Energy Cell", None));
    ammunition.extend(ammo(AMMO_10MM, "10mm Round", None));
    ammunition.extend(ammo(AMMO_20GA, "20 Gauge Round", Some(BUCKSHOT_BONUS)));
    let mut amef = sub(b"FULL", &zstr("Buckshot Damage Bonus"));
    let mut d = 0u32.to_le_bytes().to_vec();
    d.extend(1u32.to_le_bytes());
    d.extend(1.2f32.to_le_bytes());
    amef.extend(sub(b"DATA", &d));
    let ammo_effects = named(b"AMEF", BUCKSHOT_BONUS, "TestBuckshotDamageBonus", &amef);

    // Weapons: DATA value, health, weight, damage, clip; DNAM (204 bytes);
    // CRDT critical damage u16, 2 unused, multiplier.
    #[derive(Clone, Copy)]
    struct Gun {
        damage: i16,
        clip: u8,
        health: i32,
        animation: u32,
        speed: f32,
        flags1: u8,
        attack_animation: u8,
        projectiles: u8,
        flags2: u32,
        attack_mult: f32,
        fire_rate: f32,
        shots: f32,
        reload: f32,
        jam: f32,
        skill: u32,
        semi: (f32, f32),
        crit: (u16, f32),
        ammo: u32,
    }
    let weapon = |id: u32, name: &str, g: Gun| {
        let mut d = sub(b"FULL", &zstr(name));
        if g.ammo != 0 {
            d.extend(sub(b"NAM0", &g.ammo.to_le_bytes()));
        }
        let mut data = 100i32.to_le_bytes().to_vec();
        data.extend(g.health.to_le_bytes());
        data.extend(1.5f32.to_le_bytes());
        data.extend(g.damage.to_le_bytes());
        data.push(g.clip);
        d.extend(sub(b"DATA", &data));
        let mut dnam = vec![0u8; 204];
        dnam[0..4].copy_from_slice(&g.animation.to_le_bytes());
        dnam[4..8].copy_from_slice(&g.speed.to_le_bytes());
        dnam[12] = g.flags1;
        dnam[13] = 0xff;
        dnam[14] = 1;
        if g.animation >= 3 {
            dnam[36..40].copy_from_slice(&BULLET.to_le_bytes());
        }
        dnam[41] = g.attack_animation;
        dnam[42] = g.projectiles;
        dnam[44..48].copy_from_slice(&256.0f32.to_le_bytes());
        dnam[48..52].copy_from_slice(&2048.0f32.to_le_bytes());
        dnam[56..60].copy_from_slice(&g.flags2.to_le_bytes());
        dnam[60..64].copy_from_slice(&g.attack_mult.to_le_bytes());
        dnam[64..68].copy_from_slice(&g.fire_rate.to_le_bytes());
        dnam[88..92].copy_from_slice(&g.shots.to_le_bytes());
        dnam[92..96].copy_from_slice(&g.reload.to_le_bytes());
        dnam[96..100].copy_from_slice(&g.jam.to_le_bytes());
        dnam[104..108].copy_from_slice(&g.skill.to_le_bytes());
        dnam[128..132].copy_from_slice(&g.semi.0.to_le_bytes());
        dnam[132..136].copy_from_slice(&g.semi.1.to_le_bytes());
        d.extend(sub(b"DNAM", &dnam));
        let mut crdt = g.crit.0.to_le_bytes().to_vec();
        crdt.extend([0; 2]);
        crdt.extend(g.crit.1.to_le_bytes());
        crdt.extend([1, 0, 0, 0]);
        crdt.extend([0; 4]);
        d.extend(sub(b"CRDT", &crdt));
        named(b"WEAP", id, &name.replace([' ', '\''], ""), &d)
    };
    let pistol = Gun {
        damage: 16,
        clip: 13,
        health: 150,
        animation: 3,
        speed: 1.0,
        flags1: 0x04,
        attack_animation: 32,
        projectiles: 1,
        flags2: 0x2008,
        attack_mult: 1.25,
        fire_rate: 1.0,
        shots: 3.125,
        reload: 1.6667,
        jam: 1.4,
        skill: 41,
        semi: (0.0, 0.3),
        crit: (16, 1.0),
        ammo: AMMO_9MM,
    };
    let mut weapons = weapon(PISTOL, "9mm Pistol", pistol);
    weapons.extend(weapon(
        CASS_SHOTGUN,
        "Cass's Caravan Shotgun",
        Gun {
            damage: 54,
            clip: 2,
            health: 140,
            animation: 5,
            flags1: 0x88,
            attack_animation: 26,
            projectiles: 7,
            flags2: 0x8,
            attack_mult: 1.5,
            fire_rate: 2.0,
            shots: 3.2142856,
            reload: 1.5,
            jam: 2.0,
            crit: (8, 1.0),
            ammo: AMMO_COMPANION,
            ..pistol
        },
    ));
    weapons.extend(weapon(
        CASS_KNIFE,
        "Cass's Combat Knife",
        Gun {
            damage: 15,
            clip: 12,
            health: 90,
            animation: 1,
            flags1: 0x88,
            attack_animation: 255,
            flags2: 0x8,
            attack_mult: 1.4,
            shots: 3.230_769,
            reload: 0.0,
            jam: 0.0,
            skill: 38,
            semi: (0.0, 0.0),
            crit: (15, 2.0),
            ammo: 0,
            ..pistol
        },
    ));
    weapons.extend(weapon(
        SMG,
        "10mm Submachine Gun",
        Gun {
            damage: 19,
            clip: 30,
            health: 500,
            flags1: 0x02,
            attack_animation: 74,
            flags2: 0x408,
            attack_mult: 1.0,
            fire_rate: 9.0,
            shots: 9.0,
            reload: 2.6667,
            jam: 2.3333,
            semi: (0.0, 0.0),
            crit: (19, 1.0),
            ammo: AMMO_10MM,
            ..pistol
        },
    ));
    let zap = Gun {
        damage: 25,
        clip: 12,
        health: 100,
        flags1: 0xa8,
        flags2: 0,
        attack_mult: 1.5,
        fire_rate: 1.5,
        shots: 3.7499993,
        reload: 1.3,
        jam: 1.4,
        skill: 34,
        crit: (25, 2.0),
        ammo: AMMO_CELL,
        ..pistol
    };
    weapons.extend(weapon(ZAP, "Electrical Zap", zap));
    weapons.extend(weapon(
        ZAP_UPGRADE,
        "Energy Zap",
        Gun {
            damage: 30,
            clip: 30,
            attack_mult: 3.5,
            fire_rate: 7.0,
            shots: 8.749998,
            semi: (0.0, 0.1),
            crit: (30, 2.5),
            ..zap
        },
    ));
    weapons.extend(weapon(
        PLAYER_ONLY,
        "Player Pistol",
        Gun {
            damage: 100,
            flags2: 0x2009,
            ..pistol
        },
    ));

    let mut list = Vec::new();
    for w in [ZAP, ZAP_UPGRADE] {
        list.extend(sub(b"LNAM", &w.to_le_bytes()));
    }
    let lists = named(b"FLST", EMBEDDED, "TestEmbeddedWeapons", &list);

    // People and creatures: what they carry (CNTO), their style (ZNAM).
    let carried = |items: &[(u32, i32)]| {
        let mut d = Vec::new();
        for &(item, n) in items {
            let mut c = item.to_le_bytes().to_vec();
            c.extend(n.to_le_bytes());
            d.extend(sub(b"CNTO", &c));
        }
        d
    };
    let mut cass = sub(b"FULL", &zstr("Cass"));
    cass.extend(sub(b"ACBS", &[0; 24]));
    cass.extend(sub(b"AIDT", &[0; 20]));
    let mut npc_data = 100i32.to_le_bytes().to_vec();
    npc_data.extend([5; 7]);
    cass.extend(sub(b"DATA", &npc_data));
    cass.extend(carried(&[
        (CASS_SHOTGUN, 1),
        (AMMO_COMPANION, 15),
        (CASS_KNIFE, 1),
    ]));
    cass.extend(sub(b"ZNAM", &STYLE_RANGED.to_le_bytes()));
    let mut npcs = named(b"NPC_", CASS, "TestCass", &cass);
    let mut guard = sub(b"ACBS", &[0; 24]);
    guard.extend(sub(b"AIDT", &[0; 20]));
    guard.extend(sub(b"DATA", &npc_data));
    guard.extend(carried(&[(PISTOL, 1), (AMMO_9MM, 20)]));
    guard.extend(sub(b"PKID", &UNARMED_PACKAGE.to_le_bytes()));
    npcs.extend(named(b"NPC_", GUARD, "TestUnarmedGuard", &guard));
    let mut pkdt = 0x0020_0000u32.to_le_bytes().to_vec();
    pkdt.extend([4, 0, 0, 0]);
    pkdt.extend([0; 4]);
    let mut psdt = vec![0xff, 0xff, 0, 0xff];
    psdt.extend(0i32.to_le_bytes());
    let mut package = sub(b"PKDT", &pkdt);
    package.extend(sub(b"PSDT", &psdt));
    let packages = named(b"PACK", UNARMED_PACKAGE, "TestWeaponsUnequipped", &package);

    let mut ede = sub(b"FULL", &zstr("ED-E"));
    ede.extend(sub(b"ACBS", &[0; 24]));
    ede.extend(sub(b"AIDT", &[0; 20]));
    let mut crea_data = vec![6, 60, 50, 50];
    crea_data.extend(120i16.to_le_bytes());
    crea_data.extend([0, 0]);
    crea_data.extend(10i16.to_le_bytes());
    crea_data.extend([5, 5, 5, 5, 5, 5, 5]);
    ede.extend(sub(b"DATA", &crea_data));
    ede.extend(carried(&[(ZAP, 1), (AMMO_CELL, 20)]));
    ede.extend(sub(b"LNAM", &EMBEDDED.to_le_bytes()));
    let creatures = named(b"CREA", EDE, "TestEDE", &ede);

    let actor = |id: u32, base: u32, pos: [f32; 3], name: &str| {
        let mut r = placed(id, base, pos, [0.0; 3], &sub(b"EDID", &zstr(name)));
        r[..4].copy_from_slice(b"ACHR");
        r
    };
    let mut refs = actor(CASS_REF, CASS, [0.0, 0.0, 0.0], "TestCassRef");
    refs.extend(actor(EDE_REF, EDE, [200.0, 0.0, 0.0], "TestEDERef"));
    refs.extend(actor(GUARD_REF, GUARD, [400.0, 0.0, 0.0], "TestGuardRef"));
    let mut cell = edid("TestGearCell");
    cell.extend(sub(b"DATA", &[1]));
    let mut contents = record(b"CELL", CELL, &cell);
    contents.extend(group(
        CELL.to_le_bytes(),
        6,
        &group(CELL.to_le_bytes(), 9, &refs),
    ));
    let cells = group(*b"CELL", 0, &group([0; 4], 2, &group([0; 4], 3, &contents)));

    // Magic effects: DATA (72 bytes) flags u32 at 0, archetype at 64,
    // actor value at 68.
    let effect = |id: u32, name: &str, full: &str, flags: u32, archetype: u32, av: i32| {
        let mut d = sub(b"FULL", &zstr(full));
        let mut data = vec![0u8; 72];
        data[0..4].copy_from_slice(&flags.to_le_bytes());
        data[16..20].copy_from_slice(&(-1i32).to_le_bytes());
        data[64..68].copy_from_slice(&archetype.to_le_bytes());
        data[68..72].copy_from_slice(&av.to_le_bytes());
        d.extend(sub(b"DATA", &data));
        named(b"MGEF", id, name, &d)
    };
    let mut effects = effect(
        RESTORE_HEALTH,
        "TestRestoreHealth",
        "Restore Health",
        0x70,
        0,
        16,
    );
    effects.extend(effect(
        RESTORE_HEALTH_STIMPAK,
        "TestRestoreHealthStimpak",
        "Restore Health & Conditions",
        0x70,
        34,
        16,
    ));
    effects.extend(effect(
        DAMAGE_RADS,
        "TestDamageRadiationLevel",
        "Damage Rads",
        0x0100_0075,
        0,
        54,
    ));
    effects.extend(effect(
        FORTIFY_PERCEPTION,
        "TestFortifyPerception",
        "Fortify Perception",
        0x72,
        0,
        6,
    ));
    effects.extend(effect(
        NAME_ONLY,
        "TestRadiationImmunity",
        "Radiation Immunity",
        0x2000,
        0,
        20,
    ));
    effects.extend(effect(
        SCRIPT_EFFECT,
        "TestScriptEffect",
        "Script Effect",
        0x0900_0390,
        1,
        -1,
    ));
    // An effect: EFID, EFIT magnitude, area, duration, range, actor value.
    let item_effect = |effect: u32, magnitude: u32, duration: u32, av: i32| {
        let mut d = sub(b"EFID", &effect.to_le_bytes());
        let mut efit = magnitude.to_le_bytes().to_vec();
        efit.extend(0u32.to_le_bytes());
        efit.extend(duration.to_le_bytes());
        efit.extend(0u32.to_le_bytes());
        efit.extend(av.to_le_bytes());
        d.extend(sub(b"EFIT", &efit));
        d
    };
    // Aid: ENIT value, flags, 3 unused, withdrawal, addiction, sound.
    let aid = |id: u32, name: &str, flags: u8, list: &[Vec<u8>]| {
        let mut d = sub(b"FULL", &zstr(name));
        let mut enit = 20i32.to_le_bytes().to_vec();
        enit.extend([flags, 0, 0, 0]);
        enit.extend([0; 12]);
        d.extend(sub(b"ENIT", &enit));
        for e in list {
            d.extend(e);
        }
        named(b"ALCH", id, &name.replace([' ', '-'], ""), &d)
    };
    let mut aids = aid(
        STIMPAK,
        "Stimpak",
        0x05,
        &[item_effect(RESTORE_HEALTH_STIMPAK, 30, 0, 16)],
    );
    aids.extend(aid(
        NUKA_COLA,
        "Nuka-Cola",
        0x01,
        &[
            item_effect(RESTORE_HEALTH, 2, 25, 16),
            item_effect(SCRIPT_EFFECT, 0, 0, -1),
            item_effect(DAMAGE_RADS, 3, 0, 54),
        ],
    ));
    aids.extend(aid(
        FOOD,
        "Test Food",
        0x02,
        &[
            item_effect(RESTORE_HEALTH, 10, 0, 16),
            item_effect(DAMAGE_RADS, 2, 0, 54),
        ],
    ));
    aids.extend(aid(
        CHEM,
        "Test Chem",
        0x01,
        &[
            item_effect(NAME_ONLY, 0, 0, 20),
            item_effect(FORTIFY_PERCEPTION, 2, 240, 6),
            item_effect(RESTORE_HEALTH, 1, 90, 16),
        ],
    ));

    let value = |id: u32, name: &str, full: &str, abbreviation: &str| {
        let mut d = sub(b"FULL", &zstr(full));
        d.extend(sub(b"ANAM", &zstr(abbreviation)));
        named(b"AVIF", id, name, &d)
    };
    let mut values = value(AV_HEALTH, "AVHealth", "Health", "HP");
    values.extend(value(AV_RADS, "AVRadiationRads", "Rads", "Rads"));
    values.extend(value(AV_PERCEPTION, "AVPerception", "Perception", "PER"));

    // Enchantments: ENIT type, charge, cost, flags (0x04 hide effect).
    let enchantment = |id: u32, name: &str, flags: u8| {
        let mut enit = 2u32.to_le_bytes().to_vec();
        enit.extend([0; 8]);
        enit.extend([flags, 0, 0, 0]);
        let mut d = sub(b"ENIT", &enit);
        d.extend(item_effect(FORTIFY_PERCEPTION, 1, 0, 6));
        named(b"ENCH", id, name, &d)
    };
    let mut enchantments = enchantment(ENCHANTMENT, "TestEnchHat", 0);
    enchantments.extend(enchantment(HIDDEN_ENCHANTMENT, "TestEnchHidden", 0x04));
    let hat = |id: u32, name: &str, ench: u32| {
        let mut d = sub(b"FULL", &zstr(name));
        d.extend(sub(b"EITM", &ench.to_le_bytes()));
        let mut data = 10i32.to_le_bytes().to_vec();
        data.extend(50i32.to_le_bytes());
        data.extend(1.0f32.to_le_bytes());
        d.extend(sub(b"DATA", &data));
        named(b"ARMO", id, &name.replace(' ', ""), &d)
    };
    let mut armour = hat(HAT, "Lucky Hat", ENCHANTMENT);
    armour.extend(hat(HIDDEN_HAT, "Plain Hat", HIDDEN_ENCHANTMENT));
    let mut imod = sub(b"FULL", &zstr("9mm Extended Mags"));
    imod.extend(sub(b"DESC", &zstr("Increases the 9mm pistol's magazine.")));
    let mods = named(b"IMOD", WEAPON_MOD, "TestWeaponMod", &imod);

    let mut hedr = 1.34f32.to_le_bytes().to_vec();
    hedr.extend([0; 8]);
    let mut plugin = record(b"TES4", 0, &sub(b"HEDR", &hedr));
    plugin.extend(group(*b"GMST", 0, &settings));
    plugin.extend(group(*b"MGEF", 0, &effects));
    plugin.extend(group(*b"ENCH", 0, &enchantments));
    plugin.extend(group(*b"ARMO", 0, &armour));
    plugin.extend(group(*b"AMMO", 0, &ammunition));
    plugin.extend(group(*b"AMEF", 0, &ammo_effects));
    plugin.extend(group(*b"CSTY", 0, &styles));
    plugin.extend(group(*b"PROJ", 0, &projectiles));
    plugin.extend(group(*b"WEAP", 0, &weapons));
    plugin.extend(group(*b"FLST", 0, &lists));
    plugin.extend(group(*b"PACK", 0, &packages));
    plugin.extend(group(*b"CREA", 0, &creatures));
    plugin.extend(group(*b"NPC_", 0, &npcs));
    plugin.extend(group(*b"ALCH", 0, &aids));
    plugin.extend(group(*b"AVIF", 0, &values));
    plugin.extend(group(*b"IMOD", 0, &mods));
    plugin.extend(cells);
    data.write("FalloutNV.esm", &plugin);
    data
}

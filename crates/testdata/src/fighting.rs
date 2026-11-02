//! A world for the combat AI (`world::combat_ai`): combat styles,
//! weapons, projectiles, a gecko, a giant, people in three factions, all
//! in one interior.

use crate::{f32s, group, placed, record, sub, zstr, TempData};

/// Form IDs in the [`fighting`] world.
pub mod ids {
    /// The default combat style at the engine's own form (attack 40, block
    /// 30, hold 0.5â€“1.5 s: `DefaultCombatstyle`'s values).
    pub const DEFAULT_STYLE: u32 = 0x3D;
    /// The gecko's style: attack 60, block 40, hold 0.1â€“0.35 s, range
    /// multipliers 0.5 / 2, semi-auto delay multipliers 1 / 2.
    pub const GECKO_STYLE: u32 = 0xB00;
    /// The same range multipliers in a record of form version 11 (read as
    /// 1 / 1).
    pub const OLD_STYLE: u32 = 0xB01;
    /// A hitscan bullet (range 10000) and a lobbed grenade (speed 1000, a
    /// lobber, so gravity 1 whatever the record says).
    pub const BULLET: u32 = 0xB02;
    pub const GRENADE: u32 = 0xB03;
    /// A pistol (256â€“768, 3.125 attacks a second, semi-auto delay 0â€“0.3 s,
    /// the bullet), a submachine gun (automatic, 11 a second), a machete
    /// (reach 0.5, Melee Weapons) and a launcher (range fixed, 0â€“2000, the
    /// grenade).
    pub const PISTOL: u32 = 0xB04;
    pub const SMG: u32 = 0xB05;
    pub const MACHETE: u32 = 0xB06;
    pub const LAUNCHER: u32 = 0xB0F;
    /// A gecko (type 1, combat skill 40, health 20, bite 5, Perception 3;
    /// very aggressive, foolhardy, helps friends and allies; reach 25; its
    /// own style) and a giant (type 7, reach 30, no style).
    pub const GECKO: u32 = 0xB07;
    pub const GIANT: u32 = 0xB08;
    /// Guards (allies of the town), the town, raiders (enemies of the
    /// player and of the town).
    pub const GUARDS: u32 = 0xB09;
    pub const TOWN: u32 = 0xB0A;
    pub const RAIDERS: u32 = 0xB0B;
    /// A guard (aggressive, brave, helps allies; the pistol), a townsperson
    /// (unaggressive, cautious; no weapon), a raider (aggressive; the
    /// machete). Health 100 each.
    pub const GUARD: u32 = 0xB0C;
    pub const TOWNSPERSON: u32 = 0xB0D;
    pub const RAIDER: u32 = 0xB0E;
    /// `TestFightCell` and who's in it: the guard at the origin facing
    /// north, the townsperson 300 east, the raider 600 north, the gecko 1000
    /// north, the giant 5000 east.
    pub const CELL: u32 = 0xB10;
    pub const GECKO_REF: u32 = 0xB11;
    pub const GUARD_REF: u32 = 0xB12;
    pub const TOWN_REF: u32 = 0xB13;
    pub const RAIDER_REF: u32 = 0xB14;
    pub const GIANT_REF: u32 = 0xB15;
    /// A creature with nothing of its own (template flags 0x3FF) made from
    /// the gecko through a leveled list, as the game's spawned creatures
    /// are, placed 2000 north.
    pub const SPAWNED: u32 = 0xB16;
    pub const SPAWNED_REF: u32 = 0xB17;
    pub const SPAWN_LIST: u32 = 0xB18;
    pub const SETTINGS: u32 = 0xB20;
    /// A round (`AMMO`), a rifle that takes it (clip 5, 16 damage, 1.25
    /// attacks a second, reload 2 s, 0â€“3000 units) and a stick of dynamite
    /// (a grenade: animation 10, "NPCs use ammo", the grenade projectile).
    pub const ROUND: u32 = 0xB30;
    pub const RIFLE: u32 = 0xB31;
    pub const DYNAMITE: u32 = 0xB32;
    /// `TestFightGlobal` (0), which the raider's script
    /// (`TestRaiderScript`: `Begin OnStartCombat player`) adds 1 to.
    pub const FIGHT_GLOBAL: u32 = 0xB33;
    pub const RAIDER_SCRIPT: u32 = 0xB34;
    /// A leveled item list giving the rifle and 10 rounds together ("use
    /// all", as the game's `WithAmmoNVâ€¦Loot` lists), and a gunman
    /// (`GSPGAAM`-like: raiders' faction) carrying only that list, placed
    /// at 0,-600.
    pub const RIFLE_WITH_AMMO: u32 = 0xB35;
    pub const GUNMAN: u32 = 0xB36;
    pub const GUNMAN_REF: u32 = 0xB37;
    /// The player's faction, as in the game.
    pub const PLAYER_FACTION: u32 = 0x1B2A4;
}

/// The [`ids`] world in a temporary Data folder, with `FalloutNV.esm`'s
/// values for `fCombatAbsoluteMaxRangeMult` (4) and `fConfidenceCautious`
/// (0.375).
pub fn fighting(tag: &str) -> TempData {
    use ids::*;
    let data = TempData::new(tag);
    let edid = |s: &str| sub(b"EDID", &zstr(s));

    let mut settings = Vec::new();
    for (i, (name, value)) in [
        ("fCombatAbsoluteMaxRangeMult", 4.0f32),
        ("fConfidenceCautious", 0.375),
    ]
    .into_iter()
    .enumerate()
    {
        let mut d = edid(name);
        d.extend(sub(b"DATA", &value.to_le_bytes()));
        settings.extend(record(b"GMST", SETTINGS + i as u32, &d));
    }

    // Combat styles: CSTD (92 bytes), CSAD (21 floats), CSSD (64 bytes).
    let style = |attack: u8, block: u8, hold: (f32, f32), ranges: (f32, f32), semi: (f32, f32)| {
        let mut cstd = vec![0u8; 92];
        cstd[0] = 75;
        cstd[1] = 50;
        cstd[36] = block;
        cstd[37] = attack;
        for at in [40, 44, 48, 56, 60] {
            cstd[at..at + 4].copy_from_slice(&5.0f32.to_le_bytes());
        }
        cstd[52] = 25;
        cstd[64..69].copy_from_slice(&[20; 5]);
        cstd[72..76].copy_from_slice(&hold.0.to_le_bytes());
        cstd[76..80].copy_from_slice(&hold.1.to_le_bytes());
        cstd[80..82].copy_from_slice(&0x1C1u16.to_le_bytes());
        let csad = f32s(&[
            -20.0, 0.0, -110.0, 1.0, 1.0, 0.75, 1.0, 0.7, 1.0, 0.5, 20.0, 0.0, 2.0, 1.0, 20.0, 0.0,
            0.75, 1.0, 0.5, 5.0, -10.0,
        ]);
        let mut cssd = f32s(&[2048.0, 100.0, 2.0, 2.0, 10.0, 10.0, 2.0, 2.0, ranges.0, 0.0]);
        cssd.extend(0u32.to_le_bytes());
        cssd.extend(f32s(&[ranges.1, 0.0, 10000.0, semi.0, semi.1]));
        let mut d = sub(b"CSTD", &cstd);
        d.extend(sub(b"CSAD", &csad));
        d.extend(sub(b"CSSD", &cssd));
        d
    };
    let mut styles = Vec::new();
    let mut s = edid("TestDefaultCombatStyle");
    s.extend(style(40, 30, (0.5, 1.5), (1.0, 1.0), (1.0, 1.0)));
    styles.extend(record(b"CSTY", DEFAULT_STYLE, &s));
    let mut s = edid("TestGeckoStyle");
    s.extend(style(60, 40, (0.1, 0.35), (0.5, 2.0), (1.0, 2.0)));
    styles.extend(record(b"CSTY", GECKO_STYLE, &s));
    let mut s = edid("TestOldStyle");
    s.extend(style(60, 40, (0.1, 0.35), (0.5, 2.0), (1.0, 2.0)));
    let mut old = record(b"CSTY", OLD_STYLE, &s);
    // The record header's form version (bytes 20â€“21).
    old[20..22].copy_from_slice(&11u16.to_le_bytes());
    styles.extend(old);

    // Projectiles: DATA flags u16, type u16, gravity, speed, range, â€¦
    let projectile = |id: u32, name: &str, flags: u16, kind: u16, gravity: f32, speed: f32| {
        let mut d = edid(name);
        let mut data = flags.to_le_bytes().to_vec();
        data.extend(kind.to_le_bytes());
        data.extend(f32s(&[gravity, speed, 10000.0]));
        data.resize(84, 0);
        d.extend(sub(b"DATA", &data));
        record(b"PROJ", id, &d)
    };
    let mut projectiles = projectile(BULLET, "TestBullet", 0x01, 1, 3.0, 23680.0);
    projectiles.extend(projectile(GRENADE, "TestGrenade", 0, 2, 0.5, 1000.0));

    // Weapons: DATA value, health, weight, damage, clip; DNAM (204 bytes).
    #[derive(Clone, Copy)]
    struct Gun {
        animation: u32,
        reach: f32,
        flags1: u8,
        projectile: u32,
        range: (f32, f32),
        flags2: u32,
        fire_rate: f32,
        shots: f32,
        semi: (f32, f32),
        skill: u32,
        clip: u8,
        ammo: u32,
        reload: f32,
    }
    let weapon = |id: u32, name: &str, damage: i16, g: Gun| {
        let mut d = edid(name);
        d.extend(sub(b"FULL", &zstr(name)));
        if g.ammo != 0 {
            d.extend(sub(b"NAM0", &g.ammo.to_le_bytes()));
        }
        let mut data = 100i32.to_le_bytes().to_vec();
        data.extend(150i32.to_le_bytes());
        data.extend(1.5f32.to_le_bytes());
        data.extend(damage.to_le_bytes());
        data.push(g.clip);
        d.extend(sub(b"DATA", &data));
        let mut dnam = vec![0u8; 204];
        // No resist type (-1 at 120), as the game's guns.
        dnam[120..124].copy_from_slice(&(-1i32).to_le_bytes());
        dnam[0..4].copy_from_slice(&g.animation.to_le_bytes());
        dnam[4..8].copy_from_slice(&1.0f32.to_le_bytes());
        dnam[8..12].copy_from_slice(&g.reach.to_le_bytes());
        dnam[12] = g.flags1;
        dnam[14] = 1;
        dnam[36..40].copy_from_slice(&g.projectile.to_le_bytes());
        dnam[42] = 1;
        dnam[44..48].copy_from_slice(&g.range.0.to_le_bytes());
        dnam[48..52].copy_from_slice(&g.range.1.to_le_bytes());
        dnam[56..60].copy_from_slice(&g.flags2.to_le_bytes());
        dnam[64..68].copy_from_slice(&g.fire_rate.to_le_bytes());
        dnam[88..92].copy_from_slice(&g.shots.to_le_bytes());
        dnam[92..96].copy_from_slice(&g.reload.to_le_bytes());
        dnam[104..108].copy_from_slice(&g.skill.to_le_bytes());
        dnam[128..132].copy_from_slice(&g.semi.0.to_le_bytes());
        dnam[132..136].copy_from_slice(&g.semi.1.to_le_bytes());
        d.extend(sub(b"DNAM", &dnam));
        record(b"WEAP", id, &d)
    };
    let gun = Gun {
        animation: 3,
        reach: 0.0,
        flags1: 0,
        projectile: BULLET,
        range: (256.0, 768.0),
        flags2: 0x8,
        fire_rate: 1.0,
        shots: 3.125,
        semi: (0.0, 0.3),
        skill: 41,
        clip: 13,
        ammo: 0,
        reload: 0.0,
    };
    let mut weapons = weapon(PISTOL, "TestCombatPistol", 16, gun);
    weapons.extend(weapon(
        SMG,
        "TestSMG",
        8,
        Gun {
            flags1: 0x02,
            range: (128.0, 1024.0),
            fire_rate: 11.0,
            shots: 11.0,
            semi: (0.0, 0.0),
            ..gun
        },
    ));
    weapons.extend(weapon(
        MACHETE,
        "TestMachete",
        20,
        Gun {
            animation: 1,
            reach: 0.5,
            projectile: 0,
            range: (500.0, 2000.0),
            shots: 1.0,
            semi: (0.0, 0.0),
            skill: 38,
            ..gun
        },
    ));
    weapons.extend(weapon(
        LAUNCHER,
        "TestLauncher",
        50,
        Gun {
            animation: 9,
            projectile: GRENADE,
            range: (0.0, 2000.0),
            flags2: 0x20,
            shots: 0.5,
            ..gun
        },
    ));
    weapons.extend(weapon(
        RIFLE,
        "TestRifle",
        16,
        Gun {
            animation: 5,
            range: (500.0, 750.0),
            shots: 1.25,
            clip: 5,
            ammo: ROUND,
            reload: 2.0,
            ..gun
        },
    ));
    weapons.extend(weapon(
        DYNAMITE,
        "TestDynamite",
        1,
        Gun {
            animation: 10,
            projectile: GRENADE,
            range: (500.0, 1024.0),
            flags2: 0x10A,
            shots: 0.405,
            semi: (0.0, 0.0),
            skill: 35,
            clip: 12,
            ..gun
        },
    ));
    let mut round = edid("TestRound");
    round.extend(sub(b"FULL", &zstr("Test Round")));
    let mut ammo_data = 1.0f32.to_le_bytes().to_vec();
    ammo_data.extend([0, 0, 0, 0]);
    ammo_data.extend(1i32.to_le_bytes());
    ammo_data.push(0);
    round.extend(sub(b"DATA", &ammo_data));
    let ammo = record(b"AMMO", ROUND, &round);

    let mut glob = edid("TestFightGlobal");
    glob.extend(sub(b"FNAM", b"s"));
    glob.extend(sub(b"FLTV", &0.0f32.to_le_bytes()));
    let globals = record(b"GLOB", FIGHT_GLOBAL, &glob);
    let mut raider_script = edid("TestRaiderScript");
    raider_script.extend(sub(b"SCHR", &[0; 20]));
    raider_script.extend(sub(
        b"SCTX",
        b"scn TestRaiderScript\nBegin OnStartCombat player\n\tset TestFightGlobal to TestFightGlobal + 1\nEnd",
    ));
    let scripts = record(b"SCPT", RAIDER_SCRIPT, &raider_script);

    // Creatures: DATA type, combat/magic/stealth skills, health i16, 2
    // unused, damage i16, SPECIAL; AIDT; RNAM reach.
    let creature = |id: u32, name: &str, kind: u8, reach: u8, style: Option<u32>| {
        let mut d = edid(name);
        d.extend(sub(b"ACBS", &[0; 24]));
        let mut aidt = vec![0u8; 20];
        aidt[0] = 2; // very aggressive
        aidt[1] = 4; // foolhardy
        aidt[14] = 2; // helps friends and allies
        aidt[15] = 1;
        aidt[16..20].copy_from_slice(&1500i32.to_le_bytes());
        d.extend(sub(b"AIDT", &aidt));
        let mut data = vec![kind, 40, 50, 50];
        data.extend(20i16.to_le_bytes());
        data.extend([0, 0]);
        data.extend(5i16.to_le_bytes());
        data.extend([5, 3, 5, 5, 5, 5, 5]);
        d.extend(sub(b"DATA", &data));
        d.extend(sub(b"RNAM", &[reach]));
        if let Some(s) = style {
            d.extend(sub(b"ZNAM", &s.to_le_bytes()));
        }
        record(b"CREA", id, &d)
    };
    let mut creatures = creature(GECKO, "TestFightGecko", 1, 25, Some(GECKO_STYLE));
    creatures.extend(creature(GIANT, "TestGiant", 7, 30, None));
    // Everything from the template (ACBS template flags at 22): its own
    // values are zeros, its reach 32, its style the default.
    let mut spawned = edid("TestSpawnedGecko");
    let mut acbs = vec![0u8; 24];
    acbs[22..24].copy_from_slice(&0x03FFu16.to_le_bytes());
    spawned.extend(sub(b"ACBS", &acbs));
    spawned.extend(sub(b"TPLT", &SPAWN_LIST.to_le_bytes()));
    spawned.extend(sub(b"AIDT", &[0; 20]));
    spawned.extend(sub(b"DATA", &[0; 17]));
    spawned.extend(sub(b"RNAM", &[32]));
    spawned.extend(sub(b"ZNAM", &DEFAULT_STYLE.to_le_bytes()));
    creatures.extend(record(b"CREA", SPAWNED, &spawned));
    let mut list = edid("TestSpawnList");
    let mut lvlo = 1i16.to_le_bytes().to_vec();
    lvlo.extend([0, 0]);
    lvlo.extend(GECKO.to_le_bytes());
    lvlo.extend(1i16.to_le_bytes());
    lvlo.extend([0, 0]);
    list.extend(sub(b"LVLO", &lvlo));
    let leveled = record(b"LVLC", SPAWN_LIST, &list);
    let with_ammo = {
        let entry = |form: u32, count: u16| {
            let mut d = 1u16.to_le_bytes().to_vec();
            d.extend([0; 2]);
            d.extend(form.to_le_bytes());
            d.extend(count.to_le_bytes());
            d.extend([0; 2]);
            sub(b"LVLO", &d)
        };
        let mut d = edid("TestRifleWithAmmo");
        d.extend(sub(b"LVLD", &[0]));
        d.extend(sub(b"LVLF", &[0x04]));
        d.extend(entry(RIFLE, 1));
        d.extend(entry(ROUND, 10));
        record(b"LVLI", RIFLE_WITH_AMMO, &d)
    };

    // Factions: XNAM other faction, modifier, reaction (1 enemy, 2 ally).
    let faction = |id: u32, name: &str, relations: &[(u32, u32)]| {
        let mut d = edid(name);
        for &(other, reaction) in relations {
            let mut x = other.to_le_bytes().to_vec();
            x.extend(0i32.to_le_bytes());
            x.extend(reaction.to_le_bytes());
            d.extend(sub(b"XNAM", &x));
        }
        d.extend(sub(b"DATA", &[0; 4]));
        record(b"FACT", id, &d)
    };
    let mut factions = faction(GUARDS, "TestGuards", &[(TOWN, 2)]);
    factions.extend(faction(TOWN, "TestTown", &[]));
    factions.extend(faction(
        RAIDERS,
        "TestRaiders2",
        &[(PLAYER_FACTION, 1), (TOWN, 1)],
    ));
    factions.extend(faction(PLAYER_FACTION, "PlayerFaction", &[]));

    // People: health 100, SPECIAL 5; AIDT aggression, confidence,
    // assistance; one faction; what they carry.
    let npc = |id: u32,
               name: &str,
               (aggression, confidence, assistance): (u8, u8, u8),
               faction: u32,
               weapon: Option<u32>,
               script: Option<u32>| {
        let mut d = edid(name);
        if let Some(s) = script {
            d.extend(sub(b"SCRI", &s.to_le_bytes()));
        }
        d.extend(sub(b"ACBS", &[0; 24]));
        let mut membership = faction.to_le_bytes().to_vec();
        membership.extend([0; 4]);
        d.extend(sub(b"SNAM", &membership));
        let mut aidt = vec![0u8; 20];
        aidt[0] = aggression;
        aidt[1] = confidence;
        aidt[14] = assistance;
        d.extend(sub(b"AIDT", &aidt));
        let mut data = 100i32.to_le_bytes().to_vec();
        data.extend([5; 7]);
        d.extend(sub(b"DATA", &data));
        if let Some(w) = weapon {
            let mut c = w.to_le_bytes().to_vec();
            c.extend(1i32.to_le_bytes());
            d.extend(sub(b"CNTO", &c));
        }
        record(b"NPC_", id, &d)
    };
    let mut npcs = npc(GUARD, "TestGuard", (1, 3, 1), GUARDS, Some(PISTOL), None);
    npcs.extend(npc(
        TOWNSPERSON,
        "TestTownsperson",
        (0, 1, 0),
        TOWN,
        None,
        None,
    ));
    npcs.extend(npc(
        RAIDER,
        "TestRaider",
        (1, 2, 1),
        RAIDERS,
        Some(MACHETE),
        Some(RAIDER_SCRIPT),
    ));
    npcs.extend(npc(
        GUNMAN,
        "TestGunman",
        (1, 2, 1),
        RAIDERS,
        Some(RIFLE_WITH_AMMO),
        None,
    ));

    let actor = |id: u32, base: u32, pos: [f32; 3], name: &str| {
        let mut r = placed(id, base, pos, [0.0; 3], &sub(b"EDID", &zstr(name)));
        r[..4].copy_from_slice(b"ACHR");
        r
    };
    let mut refs = actor(GUARD_REF, GUARD, [0.0, 0.0, 0.0], "TestGuardRef");
    refs.extend(actor(
        TOWN_REF,
        TOWNSPERSON,
        [300.0, 0.0, 0.0],
        "TestTownRef",
    ));
    refs.extend(actor(
        RAIDER_REF,
        RAIDER,
        [0.0, 600.0, 0.0],
        "TestRaiderRef",
    ));
    refs.extend(actor(GECKO_REF, GECKO, [0.0, 1000.0, 0.0], "TestGeckoRef"));
    refs.extend(actor(
        GUNMAN_REF,
        GUNMAN,
        [0.0, -600.0, 0.0],
        "TestGunmanRef",
    ));
    refs.extend(actor(GIANT_REF, GIANT, [5000.0, 0.0, 0.0], "TestGiantRef"));
    refs.extend(actor(
        SPAWNED_REF,
        SPAWNED,
        [0.0, 2000.0, 0.0],
        "TestSpawnedRef",
    ));
    let mut cell = edid("TestFightCell");
    cell.extend(sub(b"DATA", &[1]));
    let mut contents = record(b"CELL", CELL, &cell);
    contents.extend(group(
        CELL.to_le_bytes(),
        6,
        &group(CELL.to_le_bytes(), 9, &refs),
    ));
    let cells = group(*b"CELL", 0, &group([0; 4], 2, &group([0; 4], 3, &contents)));

    let mut hedr = 1.34f32.to_le_bytes().to_vec();
    hedr.extend([0; 8]);
    let mut plugin = record(b"TES4", 0, &sub(b"HEDR", &hedr));
    plugin.extend(group(*b"GMST", 0, &settings));
    plugin.extend(group(*b"GLOB", 0, &globals));
    plugin.extend(group(*b"SCPT", 0, &scripts));
    plugin.extend(group(*b"AMMO", 0, &ammo));
    plugin.extend(group(*b"FACT", 0, &factions));
    plugin.extend(group(*b"CSTY", 0, &styles));
    plugin.extend(group(*b"PROJ", 0, &projectiles));
    plugin.extend(group(*b"WEAP", 0, &weapons));
    plugin.extend(group(*b"CREA", 0, &creatures));
    plugin.extend(group(*b"LVLC", 0, &leveled));
    plugin.extend(group(*b"LVLI", 0, &with_ammo));
    plugin.extend(group(*b"NPC_", 0, &npcs));
    plugin.extend(cells);
    data.write("FalloutNV.esm", &plugin);
    data
}

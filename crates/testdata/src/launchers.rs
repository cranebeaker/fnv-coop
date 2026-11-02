//! Launchers, grenades and mines (`world::projectiles`, `world::explosions`,
//! `world::mines`, `docs/EXPLOSIVES.md`): the missile launcher with its
//! high-velocity missiles, the grenade rifle, the pulse grenade and the
//! frag mine, with their projectiles, explosions and the pulse's EMP
//! enchantment carried over from `FalloutNV.esm` (same form IDs and
//! values), a placed frag mine owned by a faction, people and the Light
//! Step perk.

use crate::{group, placed, record, sub, zstr, TempData};

/// Form IDs in the [`launchers`] world (the game's own where the record is
/// carried over).
pub mod ids {
    /// `WeapMissileLauncher` (animation 9, 125 damage, Explosives (35),
    /// ammunition `AmmoListMissile`) firing `MissileProjectile`.
    pub const MISSILE_LAUNCHER: u32 = 0x4340;
    /// `AmmoListMissile`: `AmmoMissile` (no projectile of its own) and
    /// `AmmoMissileHighVelocity` (`DAT2` projectile `MissileProjectileHV`).
    pub const MISSILE_LIST: u32 = 0x1582F0;
    pub const MISSILE_AMMO: u32 = 0x29383;
    pub const MISSILE_AMMO_HV: u32 = 0x13E44C;
    /// `MissileProjectile` (flags 0x0A, missile, gravity 0, speed 1550)
    /// and `MissileProjectileHV` (speed 5000), both exploding as
    /// `MissileExplosion` (force 450, damage 200, radius 1000, flags 0x43).
    pub const MISSILE: u32 = 0x1BB6;
    pub const MISSILE_HV: u32 = 0x1582EB;
    pub const MISSILE_EXPLOSION: u32 = 0x5F9CD;
    /// `WeapNVGrenadeRifle` (animation 5, Explosives) firing
    /// `40mmGrenadeProjectile` (flags 0x0A, missile, gravity 1.5, speed
    /// 1750) into `40mmGrenadeExplosion` (damage 100, radius 750, flags
    /// 0x49: knocks down by formula).
    pub const GRENADE_RIFLE: u32 = 0xFF576;
    pub const GRENADE_40MM: u32 = 0x7EA30;
    pub const EXPLOSION_40MM: u32 = 0x7EA31;
    /// `WeapGrenadePulse` (animation 10) throwing `GrenadePulseProjectile`
    /// (flags 0x06, lobber, speed 1200, timer 2.5) into
    /// `GrenadePulseExplosion` (damage 10, radius 750, flags 0x43) with the
    /// `EMP` enchantment (here one effect: `DamageHealth` 200 at once).
    pub const PULSE_GRENADE: u32 = 0x4331;
    pub const PULSE_PROJECTILE: u32 = 0x3A51F;
    pub const PULSE_EXPLOSION: u32 = 0x298A9;
    pub const EMP: u32 = 0x3A36B;
    pub const DAMAGE_HEALTH: u32 = 0x33595;
    /// `WeapMineFrag` (animation 11) laying `MineFragProjectile` (flags
    /// 0x66: explosion, alt. trigger, can be disabled, can be picked up;
    /// lobber, speed 400, proximity 100, timer 3, default weapon source
    /// `WeapMineFrag`) into `MineFragExplosion` (damage 100, radius 192,
    /// flags 0x49).
    pub const FRAG_MINE: u32 = 0x433C;
    pub const MINE_PROJECTILE: u32 = 0x43FA;
    pub const MINE_EXPLOSION: u32 = 0x7078B;
    /// `LightStep`: entry point 4 (Calculate Mine Explode Chance) set to 0.
    pub const LIGHT_STEP: u32 = 0x31DAB;
    /// A faction owning the placed mine, a person in it and one not (both
    /// health 100, SPECIAL 5).
    pub const GUARDS: u32 = 0xF00;
    pub const GUARD: u32 = 0xF01;
    pub const GUARD_REF: u32 = 0xF02;
    pub const WANDERER: u32 = 0xF03;
    pub const WANDERER_REF: u32 = 0xF04;
    /// The room (an interior) and the placed mine in it, at the origin.
    pub const ROOM: u32 = 0xF05;
    pub const MINE_REF: u32 = 0xF06;
}

/// The [`ids`] world in a temporary Data folder.
pub fn launchers(tag: &str) -> TempData {
    use ids::*;
    let data = TempData::new(tag);
    let edid = |s: &str| sub(b"EDID", &zstr(s));

    // Projectiles: DATA (84 bytes): flags u16, type u16, gravity, speed,
    // range, …, proximity at 28, timer at 32, explosion at 36, impact
    // force at 52, default weapon source at 64, bounciness at 80.
    #[derive(Clone, Copy)]
    struct Proj {
        flags: u16,
        kind: u16,
        gravity: f32,
        speed: f32,
        proximity: f32,
        timer: f32,
        explosion: u32,
        source: u32,
    }
    let projectile = |id: u32, name: &str, p: Proj| {
        let mut d = edid(name);
        d.extend(sub(b"FULL", &zstr(name)));
        let mut data = vec![0u8; 84];
        data[0..2].copy_from_slice(&p.flags.to_le_bytes());
        data[2..4].copy_from_slice(&p.kind.to_le_bytes());
        let mut put = |at: usize, bytes: [u8; 4]| data[at..at + 4].copy_from_slice(&bytes);
        put(4, p.gravity.to_le_bytes());
        put(8, p.speed.to_le_bytes());
        put(12, 10000.0f32.to_le_bytes());
        put(28, p.proximity.to_le_bytes());
        put(32, p.timer.to_le_bytes());
        put(36, p.explosion.to_le_bytes());
        put(52, 2.0f32.to_le_bytes());
        put(64, p.source.to_le_bytes());
        put(80, 1.0f32.to_le_bytes());
        d.extend(sub(b"DATA", &data));
        record(b"PROJ", id, &d)
    };
    let missile = Proj {
        flags: 0x0A,
        kind: 1,
        gravity: 0.0,
        speed: 1550.0,
        proximity: 0.0,
        timer: 0.0,
        explosion: MISSILE_EXPLOSION,
        source: 0,
    };
    let mut projectiles = projectile(MISSILE, "MissileProjectile", missile);
    projectiles.extend(projectile(
        MISSILE_HV,
        "MissileProjectileHV",
        Proj {
            speed: 5000.0,
            ..missile
        },
    ));
    projectiles.extend(projectile(
        GRENADE_40MM,
        "40mmGrenadeProjectile",
        Proj {
            gravity: 1.5,
            speed: 1750.0,
            explosion: EXPLOSION_40MM,
            ..missile
        },
    ));
    projectiles.extend(projectile(
        PULSE_PROJECTILE,
        "GrenadePulseProjectile",
        Proj {
            flags: 0x06,
            kind: 2,
            gravity: 1.0,
            speed: 1200.0,
            timer: 2.5,
            explosion: PULSE_EXPLOSION,
            ..missile
        },
    ));
    projectiles.extend(projectile(
        MINE_PROJECTILE,
        "MineFragProjectile",
        Proj {
            flags: 0x66,
            kind: 2,
            gravity: 1.0,
            speed: 400.0,
            proximity: 100.0,
            timer: 3.0,
            explosion: MINE_EXPLOSION,
            source: FRAG_MINE,
        },
    ));

    // Explosions: DATA force, damage, radius, light, sound, flags at 20,
    // image space radius, …; EITM.
    let explosion = |id: u32, name: &str, damage: f32, radius: f32, flags: u32, ench: u32| {
        let mut d = edid(name);
        let mut data = vec![0u8; 52];
        data[0..4].copy_from_slice(&100.0f32.to_le_bytes());
        data[4..8].copy_from_slice(&damage.to_le_bytes());
        data[8..12].copy_from_slice(&radius.to_le_bytes());
        data[20..24].copy_from_slice(&flags.to_le_bytes());
        d.extend(sub(b"DATA", &data));
        if ench != 0 {
            d.extend(sub(b"EITM", &ench.to_le_bytes()));
        }
        record(b"EXPL", id, &d)
    };
    let mut explosions = explosion(
        MISSILE_EXPLOSION,
        "MissileExplosion",
        200.0,
        1000.0,
        0x43,
        0,
    );
    explosions.extend(explosion(
        EXPLOSION_40MM,
        "40mmGrenadeExplosion",
        100.0,
        750.0,
        0x49,
        0,
    ));
    explosions.extend(explosion(
        PULSE_EXPLOSION,
        "GrenadePulseExplosion",
        10.0,
        750.0,
        0x43,
        EMP,
    ));
    explosions.extend(explosion(
        MINE_EXPLOSION,
        "MineFragExplosion",
        100.0,
        192.0,
        0x49,
        0,
    ));

    // DamageHealth: MGEF DATA flags 0x75 (hostile, detrimental), archetype
    // 0 (value modifier) at 64, actor value 16 (health) at 68.
    let mut mgef = edid("DamageHealth");
    let mut md = vec![0u8; 72];
    md[0..4].copy_from_slice(&0x75u32.to_le_bytes());
    md[12..16].copy_from_slice(&(-1i32).to_le_bytes());
    md[16..20].copy_from_slice(&(-1i32).to_le_bytes());
    md[68..72].copy_from_slice(&16i32.to_le_bytes());
    mgef.extend(sub(b"DATA", &md));
    let magic = record(b"MGEF", DAMAGE_HEALTH, &mgef);
    // EMP: ENIT type 2 (weapon); EFID + EFIT (magnitude 200, area 0,
    // duration 0, type 1 touch, actor value 16).
    let mut ench = edid("EMP");
    ench.extend(sub(
        b"ENIT",
        &[2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    ));
    ench.extend(sub(b"EFID", &DAMAGE_HEALTH.to_le_bytes()));
    let mut efit = 200u32.to_le_bytes().to_vec();
    efit.extend(0u32.to_le_bytes());
    efit.extend(0u32.to_le_bytes());
    efit.extend(1u32.to_le_bytes());
    efit.extend(16i32.to_le_bytes());
    ench.extend(sub(b"EFIT", &efit));
    let enchantments = record(b"ENCH", EMP, &ench);

    // Ammunition: DATA speed, flags, value, clip; DAT2 projectiles per
    // shot, projectile, weight, …
    let ammo = |id: u32, name: &str, projectile: u32| {
        let mut d = edid(name);
        d.extend(sub(b"FULL", &zstr(name)));
        let mut data = 10.0f32.to_le_bytes().to_vec();
        data.extend([0, 0, 0, 0]);
        data.extend(25i32.to_le_bytes());
        data.push(1);
        d.extend(sub(b"DATA", &data));
        let mut dat2 = 0u32.to_le_bytes().to_vec();
        dat2.extend(projectile.to_le_bytes());
        dat2.extend([0; 12]);
        d.extend(sub(b"DAT2", &dat2));
        record(b"AMMO", id, &d)
    };
    let mut ammunition = ammo(MISSILE_AMMO, "AmmoMissile", 0);
    ammunition.extend(ammo(MISSILE_AMMO_HV, "AmmoMissileHighVelocity", MISSILE_HV));
    let mut list = edid("AmmoListMissile");
    for a in [MISSILE_AMMO, MISSILE_AMMO_HV] {
        list.extend(sub(b"LNAM", &a.to_le_bytes()));
    }
    let lists = record(b"FLST", MISSILE_LIST, &list);

    // Weapons: DATA value, health, weight, damage, clip; DNAM (204 bytes):
    // animation at 0, projectile at 36, skill at 104.
    let weapon = |id: u32, name: &str, animation: u32, damage: i16, projectile: u32, ammo: u32| {
        let mut d = edid(name);
        d.extend(sub(b"FULL", &zstr(name)));
        if ammo != 0 {
            d.extend(sub(b"NAM0", &ammo.to_le_bytes()));
        }
        let mut data = 100i32.to_le_bytes().to_vec();
        data.extend(100i32.to_le_bytes());
        data.extend(2.0f32.to_le_bytes());
        data.extend(damage.to_le_bytes());
        data.push(1);
        d.extend(sub(b"DATA", &data));
        let mut dnam = vec![0u8; 204];
        let mut put = |at: usize, bytes: [u8; 4]| dnam[at..at + 4].copy_from_slice(&bytes);
        put(0, animation.to_le_bytes());
        put(4, 1.0f32.to_le_bytes());
        put(36, projectile.to_le_bytes());
        put(48, 1536.0f32.to_le_bytes());
        put(88, 1.0f32.to_le_bytes());
        put(92, 2.0f32.to_le_bytes());
        put(104, 35u32.to_le_bytes());
        put(116, 1.0f32.to_le_bytes());
        put(120, (-1i32).to_le_bytes());
        dnam[14] = 1;
        dnam[42] = 1;
        d.extend(sub(b"DNAM", &dnam));
        record(b"WEAP", id, &d)
    };
    let mut weapons = weapon(
        MISSILE_LAUNCHER,
        "WeapMissileLauncher",
        9,
        125,
        MISSILE,
        MISSILE_LIST,
    );
    weapons.extend(weapon(
        GRENADE_RIFLE,
        "WeapNVGrenadeRifle",
        5,
        2,
        GRENADE_40MM,
        0,
    ));
    weapons.extend(weapon(
        PULSE_GRENADE,
        "WeapGrenadePulse",
        10,
        1,
        PULSE_PROJECTILE,
        0,
    ));
    weapons.extend(weapon(FRAG_MINE, "WeapMineFrag", 11, 1, MINE_PROJECTILE, 0));

    // Light Step: PRKE kind 2, DATA entry 4, function 1 (set), 2 tabs;
    // EPFT 1, EPFD 0.
    let mut perk = edid("LightStep");
    perk.extend(sub(b"FULL", &zstr("Light Step")));
    perk.extend(sub(b"DATA", &[0, 1, 1, 1, 0]));
    perk.extend(sub(b"PRKE", &[2, 0, 0]));
    perk.extend(sub(b"DATA", &[4, 1, 2]));
    perk.extend(sub(b"EPFT", &[1]));
    perk.extend(sub(b"EPFD", &0.0f32.to_le_bytes()));
    perk.extend(sub(b"PRKF", &[]));
    let perks = record(b"PERK", LIGHT_STEP, &perk);

    let mut fact = edid("TestMineGuards");
    fact.extend(sub(b"DATA", &[0, 0, 0, 0]));
    let factions = record(b"FACT", GUARDS, &fact);

    let npc = |id: u32, name: &str, faction: Option<u32>| {
        let mut n = edid(name);
        n.extend(sub(b"FULL", &zstr(name)));
        n.extend(sub(b"ACBS", &[0; 24]));
        if let Some(f) = faction {
            let mut snam = f.to_le_bytes().to_vec();
            snam.extend([0, 0, 0, 0]);
            n.extend(sub(b"SNAM", &snam));
        }
        n.extend(sub(b"AIDT", &[0; 20]));
        let mut d = 100i32.to_le_bytes().to_vec();
        d.extend([5; 7]);
        n.extend(sub(b"DATA", &d));
        record(b"NPC_", id, &n)
    };
    let mut npcs = npc(GUARD, "TestMineGuard", Some(GUARDS));
    npcs.extend(npc(WANDERER, "TestMineWanderer", None));

    let actor = |id: u32, base: u32, at: [f32; 3], name: &str| {
        let mut r = placed(id, base, at, [0.0; 3], &edid(name));
        r[..4].copy_from_slice(b"ACHR");
        r
    };
    let mut refs = actor(GUARD_REF, GUARD, [0.0, 2000.0, 0.0], "TestMineGuardRef");
    refs.extend(actor(
        WANDERER_REF,
        WANDERER,
        [0.0, 3000.0, 0.0],
        "TestMineWandererRef",
    ));
    let mut owned = edid("TestMineRef");
    owned.extend(sub(b"XOWN", &GUARDS.to_le_bytes()));
    let mut mine = placed(MINE_REF, MINE_PROJECTILE, [0.0; 3], [0.0; 3], &owned);
    mine[..4].copy_from_slice(b"PGRE");
    refs.extend(mine);

    let mut cell = edid("TestMineRoom");
    cell.extend(sub(b"DATA", &[1]));
    let mut contents = record(b"CELL", ROOM, &cell);
    contents.extend(group(
        ROOM.to_le_bytes(),
        6,
        &group(ROOM.to_le_bytes(), 9, &refs),
    ));
    let cells = group(*b"CELL", 0, &group([0; 4], 2, &group([0; 4], 3, &contents)));

    let mut hedr = 1.34f32.to_le_bytes().to_vec();
    hedr.extend([0; 8]);
    let mut plugin = record(b"TES4", 0, &sub(b"HEDR", &hedr));
    plugin.extend(group(*b"FACT", 0, &factions));
    plugin.extend(group(*b"MGEF", 0, &magic));
    plugin.extend(group(*b"ENCH", 0, &enchantments));
    plugin.extend(group(*b"PERK", 0, &perks));
    plugin.extend(group(*b"AMMO", 0, &ammunition));
    plugin.extend(group(*b"FLST", 0, &lists));
    plugin.extend(group(*b"EXPL", 0, &explosions));
    plugin.extend(group(*b"PROJ", 0, &projectiles));
    plugin.extend(group(*b"WEAP", 0, &weapons));
    plugin.extend(group(*b"NPC_", 0, &npcs));
    plugin.extend(cells);
    data.write("FalloutNV.esm", &plugin);
    data
}

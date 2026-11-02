//! Energy weapons (`world::combat`, `world::magic`, `docs/ENERGY_WEAPONS.md`):
//! the laser and plasma pistols with their projectiles, energy cells and
//! critical effects, carried over from `FalloutNV.esm`'s records (same
//! form IDs and values; the effect scripts are written for the tests and
//! only call the same functions), the ash and goo piles, the list of
//! effects sitters are spared, and a target in one interior.

use crate::{group, placed, record, sub, zstr, TempData};

/// Form IDs in the [`energy`] world (the game's own where the record is
/// carried over).
pub mod ids {
    /// `WeapLaserPistol`: animation 4, 12 damage, clip 30, ammo use 1,
    /// min spread 0.1, flags 0x40 / 0x8, the beam, Energy Weapons (34),
    /// resisted by Energy Resistance (60), 3.75 attacks a second; `CRDT`
    /// 12 damage, × 1.5, "on death", `LaserDisintegrationFXSpell`.
    pub const LASER_PISTOL: u32 = 0x4335;
    /// `WeapPlasmaPistol`: 33 damage, clip 32, ammo use 2, min spread 0.5,
    /// spread 2.2, the plasma bolt, 1.75 attacks a second; `CRDT` 33,
    /// × 1.5, "on death", `PlasmaEffect`.
    pub const PLASMA_PISTOL: u32 = 0x4343;
    /// `WeapNVLaserRCW`: automatic (flags 0x02), fire rate 9, 15 damage;
    /// `CRDT` 15, × 0.5.
    pub const LASER_RCW: u32 = 0x9073B;
    /// `BeamLaserProjectile` (flags 0x8C, type 4 beam, speed 10000, range
    /// 10000) and `PlasmaProjectile02` (flags 0x20C, type 1 missile, speed
    /// 7500, range 10000, impact force 18).
    pub const BEAM: u32 = 0x14B0F;
    pub const PLASMA_BOLT: u32 = 0xA73D6;
    /// `AmmoListSmallEnergyCell` holding `AmmoSmallEnergyCell` (its
    /// `AmmoEffectEnergyDTBypass`: threshold − 2; 40% of shots leave a
    /// drained cell) and `AmmoSmallEnergyCellOverCharge` (damage × 1.25,
    /// threshold − 5, wear × 1.5).
    pub const CELL_LIST: u32 = 0x158319;
    pub const CELL: u32 = 0x20772;
    pub const CELL_OVERCHARGE: u32 = 0x1582DF;
    pub const DRAINED_CELL: u32 = 0x12115E;
    pub const DT_BYPASS: u32 = 0x17BA40;
    pub const OC_DAMAGE: u32 = 0x1582DC;
    pub const OC_DT_BYPASS: u32 = 0x17BA42;
    pub const OC_WEAR: u32 = 0x1582DE;
    /// The critical effects: `LaserDisintegrationFXSpell` (its effect
    /// `LaserDisintegrationEffect`, flags 0x10000475: survives death) and
    /// `PlasmaEffect` (`GooificationEffect`, 0x10000075), each 4 s, with
    /// test scripts: the laser's sets `DisintegrateStart`, leaves an ash
    /// pile with 0.5 s to go and sets `DisintegrateEnd` at 0; the
    /// plasma's sets `GooStart`, and after 1.4 s leaves a goo pile and
    /// sets `GooEnd`.
    pub const LASER_SPELL: u32 = 0xBDA30;
    pub const LASER_EFFECT: u32 = 0xBDA2F;
    pub const LASER_SCRIPT: u32 = 0xBDA2E;
    pub const PLASMA_SPELL: u32 = 0x92C48;
    pub const PLASMA_EFFECT: u32 = 0x4DFDA;
    pub const PLASMA_SCRIPT: u32 = 0xC7691;
    /// A spell whose effect is dispelled by death (flags 0x05), whose
    /// script adds 1 to `TestEnergyGlobal` when it finishes.
    pub const BURN_SPELL: u32 = 0xE00;
    pub const BURN_EFFECT: u32 = 0xE01;
    pub const BURN_SCRIPT: u32 = 0xE02;
    pub const GLOBAL: u32 = 0xE03;
    /// `BannedEffectsOnSitters` (the goo and the laser disintegration).
    pub const BANNED_ON_SITTERS: u32 = 0x1768D7;
    /// `DefaultAshPile1` ("Ash Pile") and `DefaultAshPile2` ("Goo Pile").
    pub const ASH_PILE: u32 = 0x1B;
    pub const GOO_PILE: u32 = 0x22;
    /// A target (health 100, SPECIAL 5) standing 500 units north of the
    /// origin in `TestEnergyCell`.
    pub const TARGET: u32 = 0xE10;
    pub const TARGET_REF: u32 = 0xE11;
    pub const ROOM: u32 = 0xE12;
}

/// The [`ids`] world in a temporary Data folder.
pub fn energy(tag: &str) -> TempData {
    use ids::*;
    let data = TempData::new(tag);
    let edid = |s: &str| sub(b"EDID", &zstr(s));

    // Projectiles: DATA (84 bytes): flags u16, type u16, gravity, speed,
    // range, …, impact force at 52.
    let projectile = |id: u32, name: &str, flags: u16, kind: u16, speed: f32, force: f32| {
        let mut d = edid(name);
        let mut data = flags.to_le_bytes().to_vec();
        data.extend(kind.to_le_bytes());
        for v in [0.0f32, speed, 10000.0] {
            data.extend(v.to_le_bytes());
        }
        data.resize(84, 0);
        data[52..56].copy_from_slice(&force.to_le_bytes());
        d.extend(sub(b"DATA", &data));
        record(b"PROJ", id, &d)
    };
    let mut projectiles = projectile(BEAM, "BeamLaserProjectile", 0x8C, 4, 10000.0, 0.0);
    projectiles.extend(projectile(
        PLASMA_BOLT,
        "PlasmaProjectile02",
        0x20C,
        1,
        7500.0,
        18.0,
    ));

    // Ammunition effects: DATA kind, operation, value.
    let amef = |id: u32, name: &str, kind: u32, op: u32, value: f32| {
        let mut d = edid(name);
        let mut data = kind.to_le_bytes().to_vec();
        data.extend(op.to_le_bytes());
        data.extend(value.to_le_bytes());
        d.extend(sub(b"DATA", &data));
        record(b"AMEF", id, &d)
    };
    let mut effects = amef(DT_BYPASS, "AmmoEffectEnergyDTBypass", 2, 2, 2.0);
    effects.extend(amef(OC_DAMAGE, "AmmoEffectOCDamageBonus", 0, 1, 1.25));
    effects.extend(amef(OC_DT_BYPASS, "AmmoEffectOCDTBypass", 2, 2, 5.0));
    effects.extend(amef(OC_WEAR, "AmmoEffectOCConditionPenalty", 4, 1, 1.5));

    // Ammunition: DATA speed, flags, value, clip; DAT2 projectiles, …,
    // the item left (form at 12) and its chance (f32 at 16); RCIL effects.
    let ammo = |id: u32, name: &str, rcil: &[u32], chance: f32| {
        let mut d = edid(name);
        d.extend(sub(b"FULL", &zstr(name)));
        let mut data = 10.0f32.to_le_bytes().to_vec();
        data.extend([0, 0, 0, 0]);
        data.extend(2i32.to_le_bytes());
        data.push(20);
        d.extend(sub(b"DATA", &data));
        let mut dat2 = vec![0u8; 12];
        dat2.extend(DRAINED_CELL.to_le_bytes());
        dat2.extend(chance.to_le_bytes());
        d.extend(sub(b"DAT2", &dat2));
        for e in rcil {
            d.extend(sub(b"RCIL", &e.to_le_bytes()));
        }
        record(b"AMMO", id, &d)
    };
    let mut ammunition = ammo(CELL, "AmmoSmallEnergyCell", &[DT_BYPASS], 40.0);
    ammunition.extend(ammo(
        CELL_OVERCHARGE,
        "AmmoSmallEnergyCellOverCharge",
        &[OC_WEAR, OC_DAMAGE, OC_DT_BYPASS],
        15.0,
    ));
    let mut drained = edid("DrainedSmallEnergyCell");
    drained.extend(sub(b"FULL", &zstr("Drained Small Energy Cell")));
    drained.extend(sub(b"DATA", &[0; 8]));
    let miscs = record(b"MISC", DRAINED_CELL, &drained);
    let mut list = edid("AmmoListSmallEnergyCell");
    for a in [CELL, CELL_OVERCHARGE] {
        list.extend(sub(b"LNAM", &a.to_le_bytes()));
    }
    let mut lists = record(b"FLST", CELL_LIST, &list);
    let mut banned = edid("BannedEffectsOnSitters");
    for s in [PLASMA_SPELL, LASER_SPELL] {
        banned.extend(sub(b"LNAM", &s.to_le_bytes()));
    }
    lists.extend(record(b"FLST", BANNED_ON_SITTERS, &banned));

    // Weapons: DATA value, health, weight, damage, clip; DNAM (204
    // bytes); CRDT damage u16, mult f32 at 4, flags u8 at 8, effect at 12.
    #[derive(Clone, Copy)]
    struct Gun {
        animation: u32,
        damage: i16,
        clip: u8,
        ammo_use: u8,
        min_spread: f32,
        spread: f32,
        flags1: u8,
        projectile: u32,
        fire_rate: f32,
        shots: f32,
        reload: f32,
        crit: (u16, f32, u8, u32),
    }
    let weapon = |id: u32, name: &str, g: Gun| {
        let mut d = edid(name);
        d.extend(sub(b"FULL", &zstr(name)));
        d.extend(sub(b"NAM0", &CELL_LIST.to_le_bytes()));
        let mut data = 100i32.to_le_bytes().to_vec();
        data.extend(100i32.to_le_bytes());
        data.extend(2.0f32.to_le_bytes());
        data.extend(g.damage.to_le_bytes());
        data.push(g.clip);
        d.extend(sub(b"DATA", &data));
        let mut dnam = vec![0u8; 204];
        let mut put = |at: usize, bytes: [u8; 4]| dnam[at..at + 4].copy_from_slice(&bytes);
        put(0, g.animation.to_le_bytes());
        put(4, 1.0f32.to_le_bytes());
        put(16, g.min_spread.to_le_bytes());
        put(20, g.spread.to_le_bytes());
        put(36, g.projectile.to_le_bytes());
        put(44, 0.0f32.to_le_bytes());
        put(48, 2048.0f32.to_le_bytes());
        put(56, 0x8u32.to_le_bytes());
        put(64, g.fire_rate.to_le_bytes());
        put(88, g.shots.to_le_bytes());
        put(92, g.reload.to_le_bytes());
        put(104, 34u32.to_le_bytes());
        put(116, 1.0f32.to_le_bytes());
        put(120, 60u32.to_le_bytes());
        dnam[12] = g.flags1;
        dnam[14] = g.ammo_use;
        dnam[42] = 1;
        d.extend(sub(b"DNAM", &dnam));
        let (crit_damage, mult, flags, effect) = g.crit;
        let mut crdt = crit_damage.to_le_bytes().to_vec();
        crdt.extend([0, 0]);
        crdt.extend(mult.to_le_bytes());
        crdt.extend([flags, 0, 0, 0]);
        crdt.extend(effect.to_le_bytes());
        d.extend(sub(b"CRDT", &crdt));
        record(b"WEAP", id, &d)
    };
    let laser = Gun {
        animation: 4,
        damage: 12,
        clip: 30,
        ammo_use: 1,
        min_spread: 0.1,
        spread: 0.0,
        flags1: 0x40,
        projectile: BEAM,
        fire_rate: 1.0,
        shots: 3.75,
        reload: 3.0,
        crit: (12, 1.5, 1, LASER_SPELL),
    };
    let mut weapons = weapon(LASER_PISTOL, "WeapLaserPistol", laser);
    weapons.extend(weapon(
        PLASMA_PISTOL,
        "WeapPlasmaPistol",
        Gun {
            damage: 33,
            clip: 32,
            ammo_use: 2,
            min_spread: 0.5,
            spread: 2.2,
            flags1: 0,
            projectile: PLASMA_BOLT,
            shots: 1.75,
            reload: 2.0,
            crit: (33, 1.5, 1, PLASMA_SPELL),
            ..laser
        },
    ));
    weapons.extend(weapon(
        LASER_RCW,
        "WeapNVLaserRCW",
        Gun {
            animation: 6,
            damage: 15,
            clip: 60,
            min_spread: 0.08,
            spread: 0.5,
            flags1: 0x02,
            fire_rate: 9.0,
            shots: 9.0,
            crit: (15, 0.5, 1, LASER_SPELL),
            ..laser
        },
    ));

    // The critical effects: MGEF DATA (72 bytes): flags, base cost,
    // associated script at 8, school, resist -1 at 16, archetype 1
    // (script) at 64, actor value -1 at 68.
    let mgef = |id: u32, name: &str, flags: u32, script: u32| {
        let mut d = edid(name);
        d.extend(sub(b"FULL", &zstr(name)));
        let mut data = vec![0u8; 72];
        data[0..4].copy_from_slice(&flags.to_le_bytes());
        data[8..12].copy_from_slice(&script.to_le_bytes());
        data[12..16].copy_from_slice(&(-1i32).to_le_bytes());
        data[16..20].copy_from_slice(&(-1i32).to_le_bytes());
        data[64..68].copy_from_slice(&1u32.to_le_bytes());
        data[68..72].copy_from_slice(&(-1i32).to_le_bytes());
        d.extend(sub(b"DATA", &data));
        record(b"MGEF", id, &d)
    };
    let mut magic = mgef(
        LASER_EFFECT,
        "LaserDisintegrationEffect",
        0x1000_0475,
        LASER_SCRIPT,
    );
    magic.extend(mgef(
        PLASMA_EFFECT,
        "GooificationEffect",
        0x1000_0075,
        PLASMA_SCRIPT,
    ));
    magic.extend(mgef(BURN_EFFECT, "TestBurnEffect", 0x05, BURN_SCRIPT));
    // Spells: SPIT type 0, cost, level, flags 0x6A; EFID + EFIT (magnitude
    // 0, area 0, duration 4 s, type 0, actor value -1).
    let spell = |id: u32, name: &str, effect: u32| {
        let mut d = edid(name);
        d.extend(sub(b"FULL", &zstr(name)));
        let mut spit = vec![0u8; 16];
        spit[12] = 0x6A;
        d.extend(sub(b"SPIT", &spit));
        d.extend(sub(b"EFID", &effect.to_le_bytes()));
        let mut efit = 0u32.to_le_bytes().to_vec();
        efit.extend(0u32.to_le_bytes());
        efit.extend(4u32.to_le_bytes());
        efit.extend(0u32.to_le_bytes());
        efit.extend((-1i32).to_le_bytes());
        d.extend(sub(b"EFIT", &efit));
        record(b"SPEL", id, &d)
    };
    let mut spells = spell(LASER_SPELL, "LaserDisintegrationFXSpell", LASER_EFFECT);
    spells.extend(spell(PLASMA_SPELL, "PlasmaEffect", PLASMA_EFFECT));
    spells.extend(spell(BURN_SPELL, "TestBurnSpell", BURN_EFFECT));
    let script = |id: u32, name: &str, text: &str| {
        let mut d = edid(name);
        d.extend(sub(b"SCHR", &[0; 20]));
        d.extend(sub(b"SCTX", text.as_bytes()));
        record(b"SCPT", id, &d)
    };
    let mut scripts = script(
        LASER_SCRIPT,
        "TestLaserDisintegrationScript",
        "scn TestLaserDisintegrationScript\nfloat timer\nshort done\nshort pile\n\
         begin ScriptEffectStart\n\tSetCriticalStage DisintegrateStart\n\tset timer to 1.8\nend\n\
         begin ScriptEffectUpdate\n\tif done == 0\n\t\tif pile == 0 && timer <= 0.5\n\
         \t\t\tAttachAshPile\n\t\t\tset pile to 1\n\t\tendif\n\t\tif timer <= 0\n\
         \t\t\tSetCriticalStage DisintegrateEnd\n\t\t\tset done to 1\n\t\tendif\n\
         \t\tset timer to timer - GetSecondsPassed\n\tendif\nend",
    );
    scripts.extend(script(
        PLASMA_SCRIPT,
        "TestPlasmaGooScript",
        "scn TestPlasmaGooScript\nfloat timer\nshort done\n\
         begin ScriptEffectStart\n\tSetCriticalStage GooStart\n\tset timer to 1.4\nend\n\
         begin ScriptEffectUpdate\n\tif done == 0\n\t\tif timer <= 0\n\t\t\tAttachAshPile 2\n\
         \t\t\tSetCriticalStage GooEnd\n\t\t\tset done to 1\n\t\tendif\n\
         \t\tset timer to timer - ScriptEffectElapsedSeconds\n\tendif\nend",
    ));
    scripts.extend(script(
        BURN_SCRIPT,
        "TestBurnScript",
        "scn TestBurnScript\nbegin ScriptEffectFinish\n\tset TestEnergyGlobal to TestEnergyGlobal + 1\nend",
    ));
    let mut glob = edid("TestEnergyGlobal");
    glob.extend(sub(b"FNAM", b"s"));
    glob.extend(sub(b"FLTV", &0.0f32.to_le_bytes()));
    let globals = record(b"GLOB", GLOBAL, &glob);

    let pile = |id: u32, name: &str, full: &str| {
        let mut d = edid(name);
        d.extend(sub(b"FULL", &zstr(full)));
        record(b"ACTI", id, &d)
    };
    let mut activators = pile(ASH_PILE, "DefaultAshPile1", "Ash Pile");
    activators.extend(pile(GOO_PILE, "DefaultAshPile2", "Goo Pile"));

    let mut npc = edid("TestEnergyTarget");
    npc.extend(sub(b"FULL", &zstr("Target")));
    npc.extend(sub(b"ACBS", &[0; 24]));
    npc.extend(sub(b"AIDT", &[0; 20]));
    let mut d = 100i32.to_le_bytes().to_vec();
    d.extend([5; 7]);
    npc.extend(sub(b"DATA", &d));
    let npcs = record(b"NPC_", TARGET, &npc);

    let mut target = placed(
        TARGET_REF,
        TARGET,
        [0.0, 500.0, 0.0],
        [0.0; 3],
        &edid("TestEnergyTargetRef"),
    );
    target[..4].copy_from_slice(b"ACHR");
    let mut cell = edid("TestEnergyCell");
    cell.extend(sub(b"DATA", &[1]));
    let mut contents = record(b"CELL", ROOM, &cell);
    contents.extend(group(
        ROOM.to_le_bytes(),
        6,
        &group(ROOM.to_le_bytes(), 9, &target),
    ));
    let cells = group(*b"CELL", 0, &group([0; 4], 2, &group([0; 4], 3, &contents)));

    let mut hedr = 1.34f32.to_le_bytes().to_vec();
    hedr.extend([0; 8]);
    let mut plugin = record(b"TES4", 0, &sub(b"HEDR", &hedr));
    plugin.extend(group(*b"GLOB", 0, &globals));
    plugin.extend(group(*b"SCPT", 0, &scripts));
    plugin.extend(group(*b"MGEF", 0, &magic));
    plugin.extend(group(*b"SPEL", 0, &spells));
    plugin.extend(group(*b"ACTI", 0, &activators));
    plugin.extend(group(*b"MISC", 0, &miscs));
    plugin.extend(group(*b"AMEF", 0, &effects));
    plugin.extend(group(*b"AMMO", 0, &ammunition));
    plugin.extend(group(*b"FLST", 0, &lists));
    plugin.extend(group(*b"PROJ", 0, &projectiles));
    plugin.extend(group(*b"WEAP", 0, &weapons));
    plugin.extend(group(*b"NPC_", 0, &npcs));
    plugin.extend(cells);
    data.write("FalloutNV.esm", &plugin);
    data
}

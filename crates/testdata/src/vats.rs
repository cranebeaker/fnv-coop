//! A world for V.A.T.S. (`world::vats`): weapons with and without their own
//! action point costs, automatic ones, a melee weapon with a special
//! attack, a mine; body part data with V.A.T.S. to-hit chances; people;
//! perks that touch the to-hit chance, the costs and the action points; the
//! two form lists V.A.T.S. bans by; `FalloutNV.esm`'s values for the
//! settings it reads.

use crate::{f32s, group, placed, record, sub, zstr, TempData};

/// Form IDs in the [`vats`] world.
pub mod ids {
    /// The player's base (Agility 5 and everything else 5; health 100).
    pub const PLAYER: u32 = 0x7;
    /// People's body part data, at the game's own form: head (slot 1,
    /// actor value 25, to-hit 30, aimed at `Bip01 Head`), torso (0, 26, 60,
    /// `Bip01 Spine2`), left and right arms (3 and 5, 27 and 28, 45),
    /// left and right legs (7 and 10, 29 and 30, 50).
    pub const BODY: u32 = 0x1D;
    /// A hitscan bullet, and a mine's projectile that goes off by
    /// proximity.
    pub const BULLET: u32 = 0xC00;
    pub const MINE_BLAST: u32 = 0xC01;
    /// Rounds for the pistol (clip 13).
    pub const ROUNDS: u32 = 0xC02;
    /// A pistol as the 9mm's record has it: animation 3, min spread 0.7,
    /// Guns, its own cost 17 (second flags 0x08), V.A.T.S. to-hit 15,
    /// Strength 4 and Guns 25 asked for.
    pub const PISTOL: u32 = 0xC03;
    /// An automatic like the 10mm SMG (9 a second, cost from the table:
    /// rifles 25), one like the assault carbine (12 a second), and one
    /// like the minigun (20 a second, long bursts).
    pub const SMG: u32 = 0xC04;
    pub const CARBINE: u32 = 0xC05;
    pub const MINIGUN: u32 = 0xC06;
    /// A one-handed melee weapon costing the table's 15, with a special
    /// attack (Melee Weapons 25 asked for, damage × 2, 25 action points).
    pub const MACHETE: u32 = 0xC07;
    /// A mine (animation 11, the proximity projectile).
    pub const MINE: u32 = 0xC08;
    /// A gun in `VATSBannedWeaponsList`.
    pub const BANNED_GUN: u32 = 0xC09;
    /// Doc Mitchell's bounds ((−23, −17, 0)–(23, 17, 132): 143.9 across),
    /// unarmed; a gunman (the same, holding the pistol); a base in
    /// `BannedVATSTargets`.
    pub const DOC: u32 = 0xC0A;
    pub const GUNMAN: u32 = 0xC0B;
    pub const BANNED: u32 = 0xC0C;
    /// The two form lists, at the game's own forms.
    pub const BANNED_WEAPONS: u32 = 0x0017_4256;
    pub const BANNED_TARGETS: u32 = 0x0017_70BC;
    /// Perks: Fast Shot's action point cost (× 0.8, weapon tab: Guns or
    /// Energy Weapons); Gunslinger (to-hit × 1.25 when `GetWeaponAnimType`
    /// is 4, pistols); Sniper (× 1.25 when `GetVATSValue 5` is 25, the
    /// head); a perk that would halve the chance; Concentrated Fire; Grim
    /// Reaper's Sprint (20 points back for a kill); one doubling the
    /// regeneration.
    pub const FAST_SHOT: u32 = 0xC10;
    pub const GUNSLINGER: u32 = 0xC11;
    pub const SNIPER: u32 = 0xC12;
    pub const CLUMSY: u32 = 0xC13;
    pub const CONCENTRATED_FIRE: u32 = 0xC14;
    pub const GRIM_REAPER: u32 = 0xC15;
    pub const QUICK_RECOVERY: u32 = 0xC16;
    /// `TestVatsCell` (an interior): Doc 500 north of the origin, the
    /// gunman 1000 north, the banned one 300 east.
    pub const CELL: u32 = 0xC20;
    pub const DOC_REF: u32 = 0xC21;
    pub const GUNMAN_REF: u32 = 0xC22;
    pub const BANNED_REF: u32 = 0xC23;
    pub const SETTINGS: u32 = 0xC30;
}

/// One condition (`CTDA`, 28 bytes): `function(params) <comparison>
/// value`, asked about the subject, joined to the next by OR with `or`.
fn ctda(comparison: u8, or: bool, value: f32, function: u16, params: [u32; 2]) -> Vec<u8> {
    let mut d = vec![comparison | u8::from(or), 0, 0, 0];
    d.extend(value.to_le_bytes());
    d.extend(function.to_le_bytes());
    d.extend([0; 2]);
    d.extend(params[0].to_le_bytes());
    d.extend(params[1].to_le_bytes());
    d.extend([0; 8]);
    sub(b"CTDA", &d)
}

/// The [`ids`] world in a temporary Data folder.
pub fn vats(tag: &str) -> TempData {
    use ids::*;
    let data = TempData::new(tag);
    let edid = |s: &str| sub(b"EDID", &zstr(s));

    // `FalloutNV.esm`'s values (where it sets one; the rest stay the
    // exe's).
    let mut settings = Vec::new();
    for (i, (name, value)) in [
        ("fAVDActionPointsBase", 65.0f32),
        ("fAVDActionPointsMult", 3.0),
        ("fActionPointsRestoreRate", 0.06),
        ("fActionPointsAttackUnarmed", 22.0),
        ("fActionPointsAttackOneHandMelee", 15.0),
        ("fActionPointsAttackPistol", 20.0),
        ("fActionPointsAttackRifle", 25.0),
        ("fActionPointsAttackMine", 10.0),
        ("fActionPointsReload", 10.0),
        ("fVATSSpreadMult", 1.1),
        ("fVATSRangeSpreadMax", 100.0),
        ("fVATSShotLongBurstTime", 1.75),
        ("fVATSCriticalChanceBonus", 5.0),
        ("fVATSPlayerDamageMult", 0.75),
        ("fNPCMaxGunWobbleAngle", 15.0),
        ("fWeapStrengthReqPenalty", 0.025),
        ("fSneakMaxDistance", 2500.0),
        ("fSneakExteriorDistanceMult", 2.0),
        ("fVATSParalyzePalmChance", 0.3),
        ("fAVDCritLuckMult", 1.0),
        ("fMinDamMultiplier", 0.2),
    ]
    .into_iter()
    .enumerate()
    {
        let mut d = edid(name);
        d.extend(sub(b"DATA", &value.to_le_bytes()));
        settings.extend(record(b"GMST", SETTINGS + i as u32, &d));
    }

    // Projectiles: DATA flags u16, type u16, gravity, speed, range, then
    // the rest up to the proximity at 28.
    let projectile = |id: u32, name: &str, flags: u16, proximity: f32| {
        let mut d = edid(name);
        let mut data = flags.to_le_bytes().to_vec();
        data.extend(1u16.to_le_bytes());
        data.extend(f32s(&[0.0, 10000.0, 10000.0, 0.0, 0.0, 0.0, proximity]));
        data.resize(84, 0);
        d.extend(sub(b"DATA", &data));
        record(b"PROJ", id, &d)
    };
    let mut projectiles = projectile(BULLET, "TestVatsBullet", 0x01, 0.0);
    projectiles.extend(projectile(MINE_BLAST, "TestVatsMineBlast", 0, 100.0));

    let mut ammo = edid("TestVatsRounds");
    ammo.extend(sub(b"DATA", &[0; 13]));
    let ammo = record(b"AMMO", ROUNDS, &ammo);

    // Weapons: DATA value, health, weight, damage, clip; DNAM (204 bytes).
    #[derive(Clone, Copy)]
    struct Gun {
        animation: u32,
        flags1: u8,
        min_spread: f32,
        projectile: u32,
        to_hit: u8,
        flags2: u32,
        attack_mult: f32,
        fire_rate: f32,
        ap: f32,
        skill: u32,
        strength_req: u32,
        skill_req: u32,
        reach: f32,
    }
    let weapon = |id: u32, name: &str, g: Gun, special: Option<[f32; 3]>| {
        let mut d = edid(name);
        d.extend(sub(b"FULL", &zstr(name)));
        if g.animation >= 3 && g.projectile == BULLET {
            d.extend(sub(b"NAM0", &ROUNDS.to_le_bytes()));
        }
        let mut data = 100i32.to_le_bytes().to_vec();
        data.extend(150i32.to_le_bytes());
        data.extend(1.5f32.to_le_bytes());
        data.extend(16i16.to_le_bytes());
        data.push(13);
        d.extend(sub(b"DATA", &data));
        let mut dnam = vec![0u8; 204];
        // No resist type (-1 at 120), as the game's guns.
        dnam[120..124].copy_from_slice(&(-1i32).to_le_bytes());
        dnam[0..4].copy_from_slice(&g.animation.to_le_bytes());
        dnam[4..8].copy_from_slice(&1.0f32.to_le_bytes());
        dnam[8..12].copy_from_slice(&g.reach.to_le_bytes());
        dnam[12] = g.flags1;
        dnam[14] = 1;
        dnam[16..20].copy_from_slice(&g.min_spread.to_le_bytes());
        dnam[36..40].copy_from_slice(&g.projectile.to_le_bytes());
        dnam[40] = g.to_hit;
        dnam[42] = 1;
        dnam[48..52].copy_from_slice(&2048.0f32.to_le_bytes());
        dnam[56..60].copy_from_slice(&g.flags2.to_le_bytes());
        dnam[60..64].copy_from_slice(&g.attack_mult.to_le_bytes());
        dnam[64..68].copy_from_slice(&g.fire_rate.to_le_bytes());
        dnam[68..72].copy_from_slice(&g.ap.to_le_bytes());
        dnam[88..92].copy_from_slice(&3.125f32.to_le_bytes());
        dnam[104..108].copy_from_slice(&g.skill.to_le_bytes());
        dnam[116..120].copy_from_slice(&1.0f32.to_le_bytes());
        dnam[168..172].copy_from_slice(&g.strength_req.to_le_bytes());
        dnam[200..204].copy_from_slice(&g.skill_req.to_le_bytes());
        d.extend(sub(b"DNAM", &dnam));
        let mut crdt = 16u16.to_le_bytes().to_vec();
        crdt.extend([0, 0]);
        crdt.extend(1.0f32.to_le_bytes());
        crdt.extend([0; 8]);
        d.extend(sub(b"CRDT", &crdt));
        if let Some([skill, mult, ap]) = special {
            let mut v = 0u32.to_le_bytes().to_vec();
            v.extend(f32s(&[skill, mult, ap]));
            v.extend([0, 0, 0, 0]);
            d.extend(sub(b"VATS", &v));
        }
        record(b"WEAP", id, &d)
    };
    let pistol = Gun {
        animation: 3,
        flags1: 0,
        min_spread: 0.7,
        projectile: BULLET,
        to_hit: 15,
        flags2: 0x08,
        attack_mult: 1.0,
        fire_rate: 1.0,
        ap: 17.0,
        skill: 41,
        strength_req: 4,
        skill_req: 25,
        reach: 0.0,
    };
    let automatic = |rate: f32, flags2: u32| Gun {
        animation: 6,
        flags1: 0x02,
        flags2,
        fire_rate: rate,
        ap: 0.0,
        ..pistol
    };
    let mut weapons = weapon(PISTOL, "TestVatsPistol", pistol, None);
    weapons.extend(weapon(SMG, "TestVatsSMG", automatic(9.0, 0), None));
    weapons.extend(weapon(
        CARBINE,
        "TestVatsCarbine",
        Gun {
            ap: 20.0,
            ..automatic(12.0, 0x08)
        },
        None,
    ));
    weapons.extend(weapon(
        MINIGUN,
        "TestVatsMinigun",
        automatic(20.0, 0x800),
        None,
    ));
    weapons.extend(weapon(
        MACHETE,
        "TestVatsMachete",
        Gun {
            animation: 1,
            flags2: 0,
            projectile: 0,
            skill: 38,
            reach: 0.5,
            ..pistol
        },
        Some([25.0, 2.0, 25.0]),
    ));
    weapons.extend(weapon(
        MINE,
        "TestVatsMine",
        Gun {
            animation: 11,
            projectile: MINE_BLAST,
            flags2: 0,
            ..pistol
        },
        None,
    ));
    weapons.extend(weapon(BANNED_GUN, "TestVatsBannedGun", pistol, None));

    // Body part data: per part BPTN, BPNN, BPNT, BPNI, BPND (84 bytes:
    // damage mult, flags, type, health %, actor value, to-hit).
    let mut body = edid("DefaultBodyPartData");
    body.extend(sub(b"MODL", &zstr("Characters\\_Male\\skeleton.NIF")));
    for (name, node, target, kind, mult, health, value, to_hit) in [
        (
            "Head",
            "Bip01 Neck1",
            "Bip01 Head",
            1u8,
            2.0f32,
            20u8,
            25u8,
            30u8,
        ),
        ("Torso", "Bip01", "Bip01 Spine2", 0, 1.0, 60, 26, 60),
        (
            "Left Arm",
            "Bip01 L UpperArm",
            "Bip01 L Forearm",
            3,
            1.0,
            25,
            27,
            45,
        ),
        (
            "Left Leg",
            "Bip01 L Thigh",
            "Bip01 L Calf",
            7,
            1.0,
            25,
            29,
            50,
        ),
        (
            "Right Leg",
            "Bip01 R Thigh",
            "Bip01 R Calf",
            10,
            1.0,
            25,
            30,
            50,
        ),
        (
            "Right Arm",
            "Bip01 R UpperArm",
            "Bip01 R Forearm",
            5,
            1.0,
            25,
            28,
            45,
        ),
    ] {
        body.extend(sub(b"BPTN", &zstr(name)));
        body.extend(sub(b"BPNN", &zstr(node)));
        body.extend(sub(b"BPNT", &zstr(target)));
        body.extend(sub(b"BPNI", &zstr(node)));
        let mut bpnd = vec![0u8; 84];
        bpnd[0..4].copy_from_slice(&mult.to_le_bytes());
        bpnd[4] = 0x09;
        bpnd[5] = kind;
        bpnd[6] = health;
        bpnd[7] = value;
        bpnd[8] = to_hit;
        body.extend(sub(b"BPND", &bpnd));
        body.extend(sub(b"NAM1", &[0]));
        body.extend(sub(b"NAM4", &zstr(node)));
        body.extend(sub(b"NAM5", &[]));
    }
    let body = record(b"BPTD", BODY, &body);

    // People: OBND, ACBS, health 100 and SPECIAL 5, what they carry.
    let npc = |id: u32, name: &str, weapon: Option<u32>| {
        let mut d = edid(name);
        let mut obnd = Vec::new();
        for v in [-23i16, -17, 0, 23, 17, 132] {
            obnd.extend(v.to_le_bytes());
        }
        d.extend(sub(b"OBND", &obnd));
        d.extend(sub(b"ACBS", &[0; 24]));
        d.extend(sub(b"AIDT", &[0; 20]));
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
    let mut npcs = npc(PLAYER, "Player", None);
    npcs.extend(npc(DOC, "TestVatsDoc", None));
    npcs.extend(npc(GUNMAN, "TestVatsGunman", Some(PISTOL)));
    npcs.extend(npc(BANNED, "TestVatsBanned", None));

    let list = |id: u32, name: &str, entries: &[u32]| {
        let mut d = edid(name);
        for e in entries {
            d.extend(sub(b"LNAM", &e.to_le_bytes()));
        }
        record(b"FLST", id, &d)
    };
    let mut lists = list(BANNED_WEAPONS, "VATSBannedWeaponsList", &[BANNED_GUN]);
    lists.extend(list(BANNED_TARGETS, "BannedVATSTargets", &[BANNED]));

    // Perks: DATA (trait, level, ranks, playable, hidden), then one entry
    // point: PRKE (kind 2, rank 0, priority), DATA (entry, function, tab
    // count: the game's for the entry point — the holder, plus the weapon
    // and the target for 8 and 35, the weapon for 34, 39 and 40),
    // conditions by tab (PRKC, CTDA), EPFT 1, EPFD value, PRKF.
    let perk =
        |id: u32, name: &str, entry: u8, function: u8, value: f32, tabs: &[(u8, Vec<u8>)]| {
            let tab_count = match entry {
                8 | 35 => 3,
                34 | 39 | 40 => 2,
                _ => 1,
            };
            let mut d = edid(name);
            d.extend(sub(b"FULL", &zstr(name)));
            d.extend(sub(b"DATA", &[0, 2, 1, 1, 0]));
            d.extend(sub(b"PRKE", &[2, 0, 0]));
            d.extend(sub(b"DATA", &[entry, function, tab_count]));
            for (tab, conditions) in tabs {
                d.extend(sub(b"PRKC", &[*tab]));
                d.extend(conditions);
            }
            d.extend(sub(b"EPFT", &[1]));
            d.extend(sub(b"EPFD", &value.to_le_bytes()));
            d.extend(sub(b"PRKF", &[]));
            record(b"PERK", id, &d)
        };
    // IsWeaponSkillType 109, GetWeaponAnimType 108, GetVATSValue 408.
    let mut guns_or_energy = ctda(0, true, 1.0, 109, [41, 0]);
    guns_or_energy.extend(ctda(0, false, 1.0, 109, [34, 0]));
    let mut perks = perk(
        FAST_SHOT,
        "TestFastShot",
        40,
        3,
        0.8,
        &[(1, guns_or_energy)],
    );
    perks.extend(perk(
        GUNSLINGER,
        "TestGunslinger",
        8,
        3,
        1.25,
        &[(0, ctda(0, false, 4.0, 108, [0, 0]))],
    ));
    perks.extend(perk(
        SNIPER,
        "TestSniper",
        8,
        3,
        1.25,
        &[(0, ctda(0, false, 1.0, 408, [5, 25]))],
    ));
    perks.extend(perk(CLUMSY, "TestClumsy", 8, 3, 0.5, &[]));
    perks.extend(perk(
        CONCENTRATED_FIRE,
        "TestConcentratedFire",
        33,
        1,
        1.0,
        &[],
    ));
    perks.extend(perk(GRIM_REAPER, "TestGrimReaper", 35, 2, 20.0, &[]));
    perks.extend(perk(QUICK_RECOVERY, "TestQuickRecovery", 39, 3, 2.0, &[]));

    let actor = |id: u32, base: u32, pos: [f32; 3], name: &str| {
        let mut r = placed(id, base, pos, [0.0; 3], &sub(b"EDID", &zstr(name)));
        r[..4].copy_from_slice(b"ACHR");
        r
    };
    let mut refs = actor(DOC_REF, DOC, [0.0, 500.0, 0.0], "TestVatsDocRef");
    refs.extend(actor(
        GUNMAN_REF,
        GUNMAN,
        [0.0, 1000.0, 0.0],
        "TestVatsGunmanRef",
    ));
    refs.extend(actor(
        BANNED_REF,
        BANNED,
        [300.0, 0.0, 0.0],
        "TestVatsBannedRef",
    ));
    let mut cell = edid("TestVatsCell");
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
    plugin.extend(group(*b"FLST", 0, &lists));
    plugin.extend(group(*b"PERK", 0, &perks));
    plugin.extend(group(*b"BPTD", 0, &body));
    plugin.extend(group(*b"PROJ", 0, &projectiles));
    plugin.extend(group(*b"AMMO", 0, &ammo));
    plugin.extend(group(*b"WEAP", 0, &weapons));
    plugin.extend(group(*b"NPC_", 0, &npcs));
    plugin.extend(cells);
    data.write("FalloutNV.esm", &plugin);
    data
}

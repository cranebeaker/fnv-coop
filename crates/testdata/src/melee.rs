//! Melee and unarmed fighting (`world::melee`, `docs/MELEE_UNARMED.md`):
//! a two-handed and a one-handed melee weapon with their V.A.T.S.
//! specials, two hand-to-hand weapons and the default fists, the Mauler
//! special's knockdown spell, Super Slam, people with the game's body part
//! data, carried over from `FalloutNV.esm`'s records (same form IDs and
//! values; the spell's script is written for the tests).

use crate::{group, placed, record, sub, zstr, TempData};

/// Form IDs in the [`melee`] world (the game's own where the record is
/// carried over).
pub mod ids {
    /// The player's base (SPECIAL 5; health 100).
    pub const PLAYER: u32 = 0x7;
    /// `DefaultBodyPartData`: head (type 1, actor value 25, × 2), torso
    /// (0, 26), left arm (3, 27), right arm (5, 28), left and right legs
    /// (7 and 10, 29 and 30), each limb taking 25% of the health.
    pub const BODY: u32 = 0x1D;
    /// `WeapSuperSledge`: animation 2 (two-handed melee), reach 1, 70
    /// damage, Melee Weapons (38), its own cost 38 (second flags 0x08);
    /// `CRDT` 35, × 1; `VATS` `MaulerKnockdownSpell`, Melee Weapons 50,
    /// damage × 0.5, 48 action points; `VANM` "Mauler".
    pub const SUPER_SLEDGE: u32 = 0x4352;
    /// `WeapNVMachete`: animation 1, reach 0.5, 11 damage, attack × 1.3,
    /// its own cost 20; `CRDT` 11, × 1.5; `VATS` no spell, skill 0,
    /// damage × 0.7, 16 action points; `VANM` "Back Slash".
    pub const MACHETE: u32 = 0xCE569;
    /// `WeapBrassKnuckles`: animation 0 (hand to hand), reach 1, 18
    /// damage, Unarmed (45), its own cost 18; `CRDT` 18, × 1.
    pub const BRASS_KNUCKLES: u32 = 0x4324;
    /// `WeapNVMantisGauntlet`: hand to hand, reach 1.2, 30 damage.
    pub const MANTIS_GAUNTLET: u32 = 0x1524B3;
    /// `Fists`: hand to hand, reach 0, 0 damage, Unarmed.
    pub const FISTS: u32 = 0x1F4;
    /// `MaulerKnockdownSpell` and its `MaulerKnockdownEffect` (script
    /// effect, 2 s); the test script adds 1 to `TestMeleeKnockdowns`.
    pub const MAULER_SPELL: u32 = 0x10F09F;
    pub const MAULER_EFFECT: u32 = 0x10F0A0;
    pub const MAULER_SCRIPT: u32 = 0x10F0A1;
    pub const KNOCKDOWNS: u32 = 0xD00;
    /// `SuperSlam`: "Knockdown Chance" (entry point 52) 0.15 while the
    /// holder's `GetWeaponAnimType` is below 3 (fists, hand to hand,
    /// one-handed melee), 0.30 while it's 3 (two-handed melee). (The
    /// game's also leave out the automatic melee weapons' list and one
    /// more, not carried here.)
    pub const SUPER_SLAM: u32 = 0x14609E;
    /// People with the default body part data (health 100, SPECIAL 5):
    /// one unarmed, one holding the super sledge.
    pub const PERSON: u32 = 0xD01;
    pub const SLEDGER: u32 = 0xD02;
    /// `TestMeleeCell` (an interior): the person 100 north of the origin,
    /// the sledger 100 east.
    pub const CELL: u32 = 0xD10;
    pub const PERSON_REF: u32 = 0xD11;
    pub const SLEDGER_REF: u32 = 0xD12;
    pub const SETTINGS: u32 = 0xD20;
}

/// One condition (`CTDA`, 28 bytes): `function(params) <comparison>
/// value` (comparison in the type byte's top three bits).
fn ctda(comparison: u8, value: f32, function: u16, params: [u32; 2]) -> Vec<u8> {
    let mut d = vec![comparison << 5, 0, 0, 0];
    d.extend(value.to_le_bytes());
    d.extend(function.to_le_bytes());
    d.extend([0; 2]);
    d.extend(params[0].to_le_bytes());
    d.extend(params[1].to_le_bytes());
    d.extend([0; 8]);
    sub(b"CTDA", &d)
}

/// The [`ids`] world in a temporary Data folder.
pub fn melee(tag: &str) -> TempData {
    use ids::*;
    let data = TempData::new(tag);
    let edid = |s: &str| sub(b"EDID", &zstr(s));

    // `FalloutNV.esm`'s values (the rest stay the exe's).
    let mut settings = Vec::new();
    for (i, (name, value)) in [
        ("fDamagePowerAttackBonus", 2.0f32),
        ("fBlockSkillBase", 5.0),
        ("fBlockSkillMult", 0.3),
        ("fAVDUnarmedDamageBase", 0.5),
        ("fAVDUnarmedDamageMult", 0.05),
        ("fCombatUnarmedCritDamageMult", 1.0),
        ("fMinDamMultiplier", 0.2),
        ("fDamageSkillBase", 0.5),
        ("fDamageSkillMult", 0.5),
        ("fVATSMeleeWarpDistanceMult", 0.32),
        ("fVATSH2HWarpDistanceMult", 0.27),
    ]
    .into_iter()
    .enumerate()
    {
        let mut d = edid(name);
        d.extend(sub(b"DATA", &value.to_le_bytes()));
        settings.extend(record(b"GMST", SETTINGS + i as u32, &d));
    }

    // Weapons: DATA value, health, weight, damage, clip; DNAM (204
    // bytes); CRDT; VATS (effect, skill, damage mult, cost, silent, mod
    // required); VANM.
    #[derive(Clone, Copy)]
    struct Melee {
        animation: u32,
        reach: f32,
        damage: i16,
        attack_mult: f32,
        skill: u32,
        ap: f32,
        crit: (u16, f32),
        special: Option<(u32, [f32; 3], &'static str)>,
    }
    let weapon = |id: u32, name: &str, full: &str, m: Melee| {
        let mut d = edid(name);
        d.extend(sub(b"FULL", &zstr(full)));
        let mut data = 100i32.to_le_bytes().to_vec();
        data.extend(250i32.to_le_bytes());
        data.extend(5.0f32.to_le_bytes());
        data.extend(m.damage.to_le_bytes());
        data.push(0);
        d.extend(sub(b"DATA", &data));
        let mut dnam = vec![0u8; 204];
        dnam[120..124].copy_from_slice(&(-1i32).to_le_bytes());
        dnam[0..4].copy_from_slice(&m.animation.to_le_bytes());
        dnam[4..8].copy_from_slice(&1.0f32.to_le_bytes());
        dnam[8..12].copy_from_slice(&m.reach.to_le_bytes());
        dnam[14] = 1;
        dnam[36..40].copy_from_slice(&0u32.to_le_bytes());
        dnam[56..60].copy_from_slice(&0x08u32.to_le_bytes());
        dnam[60..64].copy_from_slice(&m.attack_mult.to_le_bytes());
        dnam[68..72].copy_from_slice(&m.ap.to_le_bytes());
        dnam[104..108].copy_from_slice(&m.skill.to_le_bytes());
        dnam[116..120].copy_from_slice(&1.0f32.to_le_bytes());
        d.extend(sub(b"DNAM", &dnam));
        let mut crdt = m.crit.0.to_le_bytes().to_vec();
        crdt.extend([0, 0]);
        crdt.extend(m.crit.1.to_le_bytes());
        crdt.extend([1, 0, 0, 0]);
        crdt.extend([0; 4]);
        d.extend(sub(b"CRDT", &crdt));
        let (effect, [skill, mult, cost], label) = m.special.unwrap_or((0, [0.0; 3], ""));
        let mut v = effect.to_le_bytes().to_vec();
        for f in [skill, mult, cost] {
            v.extend(f.to_le_bytes());
        }
        v.extend([1, 0, 0, 0]);
        d.extend(sub(b"VATS", &v));
        if !label.is_empty() {
            d.extend(sub(b"VANM", &zstr(label)));
        }
        record(b"WEAP", id, &d)
    };
    let sledge = Melee {
        animation: 2,
        reach: 1.0,
        damage: 70,
        attack_mult: 1.0,
        skill: 38,
        ap: 38.0,
        crit: (35, 1.0),
        special: Some((MAULER_SPELL, [50.0, 0.5, 48.0], "Mauler")),
    };
    let mut weapons = weapon(SUPER_SLEDGE, "WeapSuperSledge", "Super Sledge", sledge);
    weapons.extend(weapon(
        MACHETE,
        "WeapNVMachete",
        "Machete",
        Melee {
            animation: 1,
            reach: 0.5,
            damage: 11,
            attack_mult: 1.3,
            ap: 20.0,
            crit: (11, 1.5),
            special: Some((0, [0.0, 0.7, 16.0], "Back Slash")),
            ..sledge
        },
    ));
    let knuckles = Melee {
        animation: 0,
        reach: 1.0,
        damage: 18,
        attack_mult: 1.3,
        skill: 45,
        ap: 18.0,
        crit: (18, 1.0),
        special: None,
    };
    weapons.extend(weapon(
        BRASS_KNUCKLES,
        "WeapBrassKnuckles",
        "Brass Knuckles",
        knuckles,
    ));
    weapons.extend(weapon(
        MANTIS_GAUNTLET,
        "WeapNVMantisGauntlet",
        "Mantis Gauntlet",
        Melee {
            reach: 1.2,
            damage: 30,
            attack_mult: 1.55,
            ap: 22.0,
            crit: (30, 3.0),
            ..knuckles
        },
    ));
    weapons.extend(weapon(
        FISTS,
        "Fists",
        "",
        Melee {
            reach: 0.0,
            damage: 0,
            attack_mult: 1.0,
            crit: (0, 1.0),
            ..knuckles
        },
    ));

    // The Mauler's spell: MGEF DATA (72 bytes): flags 0x170, script at 8,
    // archetype 1 (script) at 64; SPEL type 0, EFIT duration 2 s.
    let mut effect = edid("MaulerKnockdownEffect");
    effect.extend(sub(b"FULL", &zstr("MaulerKnockdownEffect")));
    let mut mgef = vec![0u8; 72];
    mgef[0..4].copy_from_slice(&0x170u32.to_le_bytes());
    mgef[8..12].copy_from_slice(&MAULER_SCRIPT.to_le_bytes());
    mgef[12..16].copy_from_slice(&(-1i32).to_le_bytes());
    mgef[16..20].copy_from_slice(&(-1i32).to_le_bytes());
    mgef[64..68].copy_from_slice(&1u32.to_le_bytes());
    mgef[68..72].copy_from_slice(&(-1i32).to_le_bytes());
    effect.extend(sub(b"DATA", &mgef));
    let magic = record(b"MGEF", MAULER_EFFECT, &effect);
    let mut spell = edid("MaulerKnockdownSpell");
    let mut spit = vec![0u8; 16];
    spit[12] = 0x7A;
    spell.extend(sub(b"SPIT", &spit));
    spell.extend(sub(b"EFID", &MAULER_EFFECT.to_le_bytes()));
    let mut efit = 0u32.to_le_bytes().to_vec();
    efit.extend(0u32.to_le_bytes());
    efit.extend(2u32.to_le_bytes());
    efit.extend(0u32.to_le_bytes());
    efit.extend((-1i32).to_le_bytes());
    spell.extend(sub(b"EFIT", &efit));
    let spells = record(b"SPEL", MAULER_SPELL, &spell);
    let mut script = edid("TestMaulerKnockdownScript");
    script.extend(sub(b"SCHR", &[0; 20]));
    script.extend(sub(
        b"SCTX",
        b"scn TestMaulerKnockdownScript\nbegin ScriptEffectStart\n\
          \tset TestMeleeKnockdowns to TestMeleeKnockdowns + 1\nend",
    ));
    let scripts = record(b"SCPT", MAULER_SCRIPT, &script);
    let mut glob = edid("TestMeleeKnockdowns");
    glob.extend(sub(b"FNAM", b"s"));
    glob.extend(sub(b"FLTV", &0.0f32.to_le_bytes()));
    let globals = record(b"GLOB", KNOCKDOWNS, &glob);

    // Super Slam: DATA (trait, level, ranks, playable, hidden), two entry
    // points 52 (PRKE kind 2; DATA entry, function 1 "set", 2 tabs: the
    // holder and the weapon; conditions on the holder, tab 0, about the
    // weapon in their hands; EPFT 1, EPFD).
    let mut perk = edid("SuperSlam");
    perk.extend(sub(b"FULL", &zstr("Super Slam!")));
    perk.extend(sub(b"DATA", &[0, 8, 1, 1, 0]));
    // GetWeaponAnimType (108): < 3 (comparison 4) and == 3 (0).
    for (comparison, value) in [(4u8, 0.15f32), (0, 0.30)] {
        perk.extend(sub(b"PRKE", &[2, 0, 0]));
        perk.extend(sub(b"DATA", &[52, 1, 2]));
        perk.extend(sub(b"PRKC", &[0]));
        perk.extend(ctda(comparison, 3.0, 108, [0, 0]));
        perk.extend(sub(b"EPFT", &[1]));
        perk.extend(sub(b"EPFD", &value.to_le_bytes()));
        perk.extend(sub(b"PRKF", &[]));
    }
    let perks = record(b"PERK", SUPER_SLAM, &perk);

    // Body part data: per part BPTN, BPNN, BPNT, BPNI, BPND (84 bytes:
    // damage mult, flags, type, health %, actor value, to-hit).
    let mut body = edid("DefaultBodyPartData");
    body.extend(sub(b"MODL", &zstr("Characters\\_Male\\skeleton.NIF")));
    for (name, node, kind, mult, value) in [
        ("Head", "Bip01 Head", 1u8, 2.0f32, 25u8),
        ("Torso", "Bip01 Spine2", 0, 1.0, 26),
        ("Left Arm", "Bip01 L Forearm", 3, 1.0, 27),
        ("Right Arm", "Bip01 R Forearm", 5, 1.0, 28),
        ("Left Leg", "Bip01 L Calf", 7, 1.0, 29),
        ("Right Leg", "Bip01 R Calf", 10, 1.0, 30),
    ] {
        body.extend(sub(b"BPTN", &zstr(name)));
        body.extend(sub(b"BPNN", &zstr(node)));
        body.extend(sub(b"BPNT", &zstr(node)));
        body.extend(sub(b"BPNI", &zstr(node)));
        let mut bpnd = vec![0u8; 84];
        bpnd[0..4].copy_from_slice(&mult.to_le_bytes());
        bpnd[4] = 0x09;
        bpnd[5] = kind;
        bpnd[6] = 25;
        bpnd[7] = value;
        bpnd[8] = 50;
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
    npcs.extend(npc(PERSON, "TestMeleePerson", None));
    npcs.extend(npc(SLEDGER, "TestMeleeSledger", Some(SUPER_SLEDGE)));

    let actor = |id: u32, base: u32, at: [f32; 3], name: &str| {
        let mut r = placed(id, base, at, [0.0; 3], &edid(name));
        r[..4].copy_from_slice(b"ACHR");
        r
    };
    let mut refs = actor(PERSON_REF, PERSON, [0.0, 100.0, 0.0], "TestMeleePersonRef");
    refs.extend(actor(
        SLEDGER_REF,
        SLEDGER,
        [100.0, 0.0, 0.0],
        "TestMeleeSledgerRef",
    ));
    let mut cell = edid("TestMeleeCell");
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
    plugin.extend(group(*b"MGEF", 0, &magic));
    plugin.extend(group(*b"SPEL", 0, &spells));
    plugin.extend(group(*b"BPTD", 0, &body));
    plugin.extend(group(*b"WEAP", 0, &weapons));
    plugin.extend(group(*b"PERK", 0, &perks));
    plugin.extend(group(*b"NPC_", 0, &npcs));
    plugin.extend(cells);
    data.write("FalloutNV.esm", &plugin);
    data
}

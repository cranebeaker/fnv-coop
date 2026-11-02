//! A world for hit effects (`world::impacts`): impact data and sets, a gun,
//! fists, sounds, decal texture sets, power armour, people and creatures
//! (one inheriting its sounds), body part data with its own impact set, in
//! one interior. Also an effect model with animation controllers.

use crate::{f32s, group, placed, record, sub, zstr, NifBuilder, TempData};

/// Form IDs in the [`impacts`] world.
pub mod ids {
    /// Sounds: a stone hit (heard within 2000 units), flesh hits (two, the
    /// second within 500), a metal hit, a gecko's bite (its "weapon"
    /// sound) and death cry.
    pub const STONE_SOUND: u32 = 0xC00;
    pub const FLESH_SOUND: u32 = 0xC01;
    pub const METAL_SOUND: u32 = 0xC02;
    pub const FLESH_SOUND2: u32 = 0xC03;
    pub const BITE_SOUND: u32 = 0xC04;
    pub const DEATH_SOUND: u32 = 0xC05;
    /// Decal texture sets: blood, a bullet hole.
    pub const BLOOD_SET: u32 = 0xC10;
    pub const HOLE_SET: u32 = 0xC11;
    /// Impacts: on stone (the bullet hole, the stone sound, the
    /// reflection), on flesh (the blood spray model, blood decal, two flesh
    /// sounds), on metal (the metal sound), a bite on flesh (the flesh
    /// sound), the body part's spatter (blood decal, no sound), the default
    /// set's flesh impact (a different model).
    pub const STONE_IMPACT: u32 = 0xC20;
    pub const FLESH_IMPACT: u32 = 0xC21;
    pub const METAL_IMPACT: u32 = 0xC22;
    pub const BITE_IMPACT: u32 = 0xC23;
    pub const SPATTER_IMPACT: u32 = 0xC24;
    pub const DEFAULT_FLESH_IMPACT: u32 = 0xC25;
    /// Impact sets: the gun's (stone, metal, organic), the gecko's bite
    /// (organic), the torso's own (organic), the fists' (organic: the
    /// bite's impact), and `DefaultImpactDataSet` at its engine form.
    pub const GUN_SET: u32 = 0xC30;
    pub const BITE_SET: u32 = 0xC31;
    pub const SPATTER_SET: u32 = 0xC32;
    pub const FIST_SET: u32 = 0xC33;
    pub const DEFAULT_SET: u32 = 0x276;
    /// A gun with the gun set; the engine's fists (`000001F4`) with theirs.
    pub const GUN: u32 = 0xC40;
    pub const FISTS: u32 = 0x1F4;
    /// Power armour (upper body), its helmet (head and hair), a hat.
    pub const POWER_ARMOUR: u32 = 0xC50;
    pub const POWER_HELMET: u32 = 0xC51;
    pub const HAT: u32 = 0xC52;
    /// People: one plain (organic by default), one in power armour without
    /// the helmet, a robot (`NAM4` metal) with neither blood spray nor
    /// blood decals.
    pub const PERSON: u32 = 0xC60;
    pub const KNIGHT: u32 = 0xC61;
    pub const ROBOT: u32 = 0xC62;
    /// Creatures: a sound template (its own sounds: the bite always, the
    /// death cry at 0% then 100%), and a gecko taking its sounds from it,
    /// biting with the bite set, with the test body part data.
    pub const SOUND_TEMPLATE: u32 = 0xC70;
    pub const GECKO: u32 = 0xC71;
    /// Body part data: the gecko's, and people's default (`0000001D`), each
    /// with a torso whose own impact set is the spatter set and a head
    /// with none.
    pub const GECKO_BODY: u32 = 0xC80;
    pub const DEFAULT_BODY: u32 = 0x1D;
    /// The cell and who's in it.
    pub const CELL: u32 = 0xC90;
    pub const PERSON_REF: u32 = 0xC91;
    pub const KNIGHT_REF: u32 = 0xC92;
    pub const ROBOT_REF: u32 = 0xC93;
    pub const GECKO_REF: u32 = 0xC94;
}

/// A NIF whose root node carries controllers playing `times` (start,
/// stop) one after the other, except the last, which is on its one shape
/// (so both chains and children are looked through).
pub fn animated_effect_nif(times: &[(f32, f32)]) -> Vec<u8> {
    let mut b = NifBuilder::default();
    let first_controller = 2i32;
    let (on_root, on_shape) = match times.split_last() {
        Some((last, rest)) => (rest.to_vec(), Some(*last)),
        None => (Vec::new(), None),
    };
    let mut root = b.av("Root", &[]);
    if !on_root.is_empty() {
        root[8..12].copy_from_slice(&first_controller.to_le_bytes());
    }
    root.extend(1u32.to_le_bytes());
    root.extend(1i32.to_le_bytes());
    root.extend(0u32.to_le_bytes());
    let shape_controller = first_controller + on_root.len() as i32;
    let mut shape = b.av("Mesh", &[]);
    if on_shape.is_some() {
        shape[8..12].copy_from_slice(&shape_controller.to_le_bytes());
    }
    shape.extend((-1i32).to_le_bytes());
    shape.extend((-1i32).to_le_bytes());
    shape.extend(0u32.to_le_bytes());
    shape.extend((-1i32).to_le_bytes());
    shape.push(0);
    let controller = |next: i32, (start, stop): (f32, f32), target: i32| {
        let mut c = next.to_le_bytes().to_vec();
        c.extend(0u16.to_le_bytes());
        c.extend(f32s(&[1.0, 0.0, start, stop]));
        c.extend(target.to_le_bytes());
        c.extend((-1i32).to_le_bytes());
        c
    };
    let mut blocks = vec![
        ("BSFadeNode".to_string(), root),
        ("NiTriShape".to_string(), shape),
    ];
    for (i, t) in on_root.iter().enumerate() {
        let next = if i + 1 < on_root.len() {
            first_controller + i as i32 + 1
        } else {
            -1
        };
        blocks.push(("NiTransformController".into(), controller(next, *t, 0)));
    }
    if let Some(t) = on_shape {
        blocks.push(("NiAlphaController".into(), controller(-1, t, 1)));
    }
    b.blocks = blocks;
    b.build()
}

/// An effect model animated by its own controllers, as the game's impact
/// models are: the root node grows from scale 1 at 0 s to 3 at 0.3 s
/// (`NiTransformController` → `NiTransformInterpolator` →
/// `NiTransformData`, clamped), and its shape `Quad`'s material fades from
/// alpha 1 at 0.05 s to 0 at 0.25 s (`NiAlphaController` →
/// `NiFloatInterpolator` → `NiFloatData`).
pub fn controlled_effect_nif() -> Vec<u8> {
    let mut b = NifBuilder::default();
    let mut root = b.av("Root", &[]);
    root[8..12].copy_from_slice(&2i32.to_le_bytes());
    root.extend(1u32.to_le_bytes());
    root.extend(1i32.to_le_bytes());
    root.extend(0u32.to_le_bytes());
    let mut shape = b.av("Quad", &[5]);
    shape.extend((-1i32).to_le_bytes());
    shape.extend((-1i32).to_le_bytes());
    shape.extend(0u32.to_le_bytes());
    shape.extend((-1i32).to_le_bytes());
    shape.push(0);
    // Clamped (cycle type 2) and active.
    let controller = |(start, stop): (f32, f32), target: i32, interpolator: i32| {
        let mut c = (-1i32).to_le_bytes().to_vec();
        c.extend(0x000Cu16.to_le_bytes());
        c.extend(f32s(&[1.0, 0.0, start, stop]));
        c.extend(target.to_le_bytes());
        c.extend(interpolator.to_le_bytes());
        c
    };
    let mut interpolator = f32s(&[0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0]);
    interpolator.extend(4i32.to_le_bytes());
    // No rotation or translation keys; two linear scale keys.
    let mut transform = 0u32.to_le_bytes().to_vec();
    transform.extend(0u32.to_le_bytes());
    transform.extend(2u32.to_le_bytes());
    transform.extend(1u32.to_le_bytes());
    transform.extend(f32s(&[0.0, 1.0, 0.3, 3.0]));
    let mut material = b.net("");
    material[8..12].copy_from_slice(&6i32.to_le_bytes());
    material.extend(f32s(&[1.0, 1.0, 1.0, 0.0, 0.0, 0.0, 10.0, 1.0, 1.0]));
    let mut float = f32s(&[1.0]);
    float.extend(8i32.to_le_bytes());
    let mut keys = 2u32.to_le_bytes().to_vec();
    keys.extend(1u32.to_le_bytes());
    keys.extend(f32s(&[0.05, 1.0, 0.25, 0.0]));
    b.blocks = vec![
        ("BSFadeNode".into(), root),
        ("NiTriShape".into(), shape),
        ("NiTransformController".into(), controller((0.0, 0.3), 0, 3)),
        ("NiTransformInterpolator".into(), interpolator),
        ("NiTransformData".into(), transform),
        ("NiMaterialProperty".into(), material),
        ("NiAlphaController".into(), controller((0.05, 0.25), 5, 7)),
        ("NiFloatInterpolator".into(), float),
        ("NiFloatData".into(), keys),
    ];
    b.build()
}

/// The [`ids`] world in a temporary Data folder, with `FalloutNV.esm`'s
/// values for `fCombatSpeakHitChance` and `fCombatSpeakHitThreshold`
/// (0.01 each).
pub fn impacts(tag: &str) -> TempData {
    use ids::*;
    let data = TempData::new(tag);
    let edid = |s: &str| sub(b"EDID", &zstr(s));
    let form = |kind: &[u8; 4], id: u32| sub(kind, &id.to_le_bytes());

    let mut settings = Vec::new();
    for (i, (name, value)) in [
        ("fCombatSpeakHitChance", 0.01f32),
        ("fCombatSpeakHitThreshold", 0.01),
    ]
    .into_iter()
    .enumerate()
    {
        let mut d = edid(name);
        d.extend(sub(b"DATA", &value.to_le_bytes()));
        settings.extend(record(b"GMST", 0xCF0 + i as u32, &d));
    }

    // Sounds: FNAM, SNDD min ×5, max ×100.
    let sound = |id: u32, name: &str, file: &str, max: u8| {
        let mut d = edid(name);
        d.extend(sub(b"FNAM", &zstr(file)));
        let mut sndd = vec![10, max, 0, 0];
        sndd.extend(0u32.to_le_bytes());
        d.extend(sub(b"SNDD", &sndd));
        record(b"SOUN", id, &d)
    };
    let mut sounds = sound(STONE_SOUND, "TestStoneHit", "fx\\stone.wav", 20);
    sounds.extend(sound(FLESH_SOUND, "TestFleshHit", "fx\\flesh.wav", 20));
    sounds.extend(sound(METAL_SOUND, "TestMetalHit", "fx\\metal.wav", 20));
    sounds.extend(sound(FLESH_SOUND2, "TestFleshHit2", "fx\\flesh2.wav", 5));
    sounds.extend(sound(BITE_SOUND, "TestBite", "fx\\bite.wav", 20));
    sounds.extend(sound(DEATH_SOUND, "TestGeckoDeath", "fx\\death.wav", 20));

    // Decal data: widths 10–30, heights 8–32, alpha blending.
    let dodt = {
        let mut d = f32s(&[10.0, 30.0, 8.0, 32.0, 16.0, 4.0, 0.04]);
        d.extend([1, 0x02, 0, 0, 255, 255, 255, 255]);
        d
    };
    let txst = |id: u32, name: &str, diffuse: &str| {
        let mut d = edid(name);
        d.extend(sub(b"TX00", &zstr(diffuse)));
        d.extend(sub(b"DODT", &dodt));
        record(b"TXST", id, &d)
    };
    let mut sets = txst(BLOOD_SET, "TestBloodDecal", "Test\\Blood.dds");
    sets.extend(txst(HOLE_SET, "TestBulletHole", "Test\\Hole.dds"));

    // Impacts: MODL, DATA (duration, orientation, angle, radius, sound
    // level, flags), DODT, DNAM, SNAM, NAM1.
    let impact = |id: u32,
                  name: &str,
                  model: Option<&str>,
                  orientation: u32,
                  decal: Option<u32>,
                  sounds: (Option<u32>, Option<u32>)| {
        let mut d = edid(name);
        if let Some(m) = model {
            d.extend(sub(b"MODL", &zstr(m)));
        }
        let mut data = f32s(&[0.25]);
        data.extend(orientation.to_le_bytes());
        data.extend(f32s(&[15.0, 16.0]));
        data.extend(1u32.to_le_bytes());
        data.extend(0u32.to_le_bytes());
        d.extend(sub(b"DATA", &data));
        if let Some(t) = decal {
            d.extend(sub(b"DODT", &dodt));
            d.extend(form(b"DNAM", t));
        }
        if let Some(s) = sounds.0 {
            d.extend(form(b"SNAM", s));
        }
        if let Some(s) = sounds.1 {
            d.extend(form(b"NAM1", s));
        }
        record(b"IPCT", id, &d)
    };
    let mut impacts = impact(
        STONE_IMPACT,
        "TestStoneImpact",
        Some("Effects\\TestStone.NIF"),
        2,
        Some(HOLE_SET),
        (Some(STONE_SOUND), None),
    );
    impacts.extend(impact(
        FLESH_IMPACT,
        "TestFleshImpact",
        Some("Effects\\TestBlood.NIF"),
        0,
        Some(BLOOD_SET),
        (Some(FLESH_SOUND), Some(FLESH_SOUND2)),
    ));
    impacts.extend(impact(
        METAL_IMPACT,
        "TestMetalImpact",
        Some("Effects\\TestSparks.NIF"),
        0,
        None,
        (Some(METAL_SOUND), None),
    ));
    impacts.extend(impact(
        BITE_IMPACT,
        "TestBiteImpact",
        None,
        0,
        None,
        (Some(FLESH_SOUND), None),
    ));
    impacts.extend(impact(
        SPATTER_IMPACT,
        "TestSpatterImpact",
        None,
        0,
        Some(BLOOD_SET),
        (None, None),
    ));
    impacts.extend(impact(
        DEFAULT_FLESH_IMPACT,
        "TestDefaultFlesh",
        Some("Effects\\TestDefaultBlood.NIF"),
        0,
        Some(BLOOD_SET),
        (Some(FLESH_SOUND), None),
    ));

    // Impact sets: 12 forms by material (stone 0, metal 4, organic 6).
    let set = |id: u32, name: &str, by: &[(usize, u32)]| {
        let mut forms = [0u32; 12];
        for &(m, f) in by {
            forms[m] = f;
        }
        let mut d = edid(name);
        d.extend(sub(
            b"DATA",
            &forms
                .iter()
                .flat_map(|f| f.to_le_bytes())
                .collect::<Vec<u8>>(),
        ));
        record(b"IPDS", id, &d)
    };
    let mut impact_sets = set(
        GUN_SET,
        "TestGunImpacts",
        &[(0, STONE_IMPACT), (4, METAL_IMPACT), (6, FLESH_IMPACT)],
    );
    impact_sets.extend(set(BITE_SET, "TestBiteImpacts", &[(6, BITE_IMPACT)]));
    impact_sets.extend(set(
        SPATTER_SET,
        "TestSpatterImpacts",
        &[(6, SPATTER_IMPACT)],
    ));
    impact_sets.extend(set(FIST_SET, "TestFistImpacts", &[(6, BITE_IMPACT)]));
    impact_sets.extend(set(
        DEFAULT_SET,
        "DefaultImpactDataSet",
        &[(6, DEFAULT_FLESH_IMPACT), (4, METAL_IMPACT)],
    ));

    let weapon = |id: u32, name: &str, set: u32| {
        let mut d = edid(name);
        d.extend(form(b"INAM", set));
        let mut data = 10i32.to_le_bytes().to_vec();
        data.extend(100i32.to_le_bytes());
        data.extend(f32s(&[1.0]));
        data.extend(10i16.to_le_bytes());
        data.push(10);
        d.extend(sub(b"DATA", &data));
        record(b"WEAP", id, &d)
    };
    let mut weapons = weapon(GUN, "TestImpactGun", GUN_SET);
    weapons.extend(weapon(FISTS, "Fists", FIST_SET));

    // Armour: BMDT slots, general flags (0x20 power armour).
    let armour = |id: u32, name: &str, slots: u32, flags: u8| {
        let mut d = edid(name);
        let mut bmdt = slots.to_le_bytes().to_vec();
        bmdt.extend([flags, 0, 0, 0]);
        d.extend(sub(b"BMDT", &bmdt));
        d.extend(sub(b"MODL", &zstr("Armor\\Test.NIF")));
        record(b"ARMO", id, &d)
    };
    let mut armours = armour(POWER_ARMOUR, "TestPowerArmor", 0x04, 0x20);
    armours.extend(armour(POWER_HELMET, "TestPowerHelmet", 0x03, 0x20));
    armours.extend(armour(HAT, "TestHat", 0x400, 0));

    // People: ACBS (flags at 0), NAM4 material, what they carry.
    let npc = |id: u32, name: &str, flags: u32, material: Option<u32>, carried: &[u32]| {
        let mut d = edid(name);
        let mut acbs = flags.to_le_bytes().to_vec();
        acbs.extend([0; 20]);
        d.extend(sub(b"ACBS", &acbs));
        if let Some(m) = material {
            d.extend(sub(b"NAM4", &m.to_le_bytes()));
        }
        for c in carried {
            let mut cnto = c.to_le_bytes().to_vec();
            cnto.extend(1i32.to_le_bytes());
            d.extend(sub(b"CNTO", &cnto));
        }
        record(b"NPC_", id, &d)
    };
    let mut npcs = npc(PERSON, "TestPerson", 0, None, &[HAT]);
    npcs.extend(npc(KNIGHT, "TestKnight", 0, None, &[POWER_ARMOUR, GUN]));
    npcs.extend(npc(ROBOT, "TestRobot", 0x800 | 0x1000, Some(4), &[]));

    // Creatures: ACBS flags (0x100: its own sounds), CSDT/CSDI/CSDC,
    // CSCR, CNAM, NAM4, PNAM.
    let mut template = edid("TestGeckoSounds");
    let mut acbs = 0x100u32.to_le_bytes().to_vec();
    acbs.extend([0; 20]);
    template.extend(sub(b"ACBS", &acbs));
    for (kind, sound, chance) in [
        (9u32, BITE_SOUND, 100u8),
        (8, STONE_SOUND, 0),
        (8, DEATH_SOUND, 100),
    ] {
        template.extend(sub(b"CSDT", &kind.to_le_bytes()));
        template.extend(form(b"CSDI", sound));
        template.extend(sub(b"CSDC", &[chance]));
    }
    let mut creatures = record(b"CREA", SOUND_TEMPLATE, &template);
    let mut gecko = edid("TestImpactGecko");
    gecko.extend(sub(b"ACBS", &[0; 24]));
    let mut stats = vec![1, 50, 50, 50];
    stats.extend(20i16.to_le_bytes());
    stats.extend([0, 0]);
    stats.extend(5i16.to_le_bytes());
    stats.extend([5; 7]);
    gecko.extend(sub(b"DATA", &stats));
    gecko.extend(form(b"PNAM", GECKO_BODY));
    gecko.extend(sub(b"NAM4", &6u32.to_le_bytes()));
    gecko.extend(form(b"CSCR", SOUND_TEMPLATE));
    gecko.extend(form(b"CNAM", BITE_SET));
    creatures.extend(record(b"CREA", GECKO, &gecko));

    // Body part data: a torso (type 0) whose BPND has its own impact set at
    // 68, and a head (type 1) with none.
    let body = |id: u32, name: &str| {
        let mut d = edid(name);
        for (part, node, kind, set) in [
            ("Torso", "Bip01 Spine", 0u8, SPATTER_SET),
            ("Head", "Bip01 Head", 1, 0),
        ] {
            d.extend(sub(b"BPTN", &zstr(part)));
            d.extend(sub(b"BPNN", &zstr(node)));
            d.extend(sub(b"BPNT", &zstr(node)));
            let mut bpnd = vec![0u8; 84];
            bpnd[0..4].copy_from_slice(&1.0f32.to_le_bytes());
            bpnd[5] = kind;
            bpnd[6] = 50;
            bpnd[7] = 26;
            bpnd[68..72].copy_from_slice(&set.to_le_bytes());
            d.extend(sub(b"BPND", &bpnd));
        }
        record(b"BPTD", id, &d)
    };
    let mut bodies = body(GECKO_BODY, "TestGeckoBody");
    bodies.extend(body(DEFAULT_BODY, "DefaultBodyPartData"));

    let actor = |id: u32, base: u32, kind: &[u8; 4], name: &str| {
        let mut r = placed(id, base, [0.0; 3], [0.0; 3], &sub(b"EDID", &zstr(name)));
        r[..4].copy_from_slice(kind);
        r
    };
    let mut refs = actor(PERSON_REF, PERSON, b"ACHR", "TestPersonRef");
    refs.extend(actor(KNIGHT_REF, KNIGHT, b"ACHR", "TestKnightRef"));
    refs.extend(actor(ROBOT_REF, ROBOT, b"ACHR", "TestRobotRef"));
    refs.extend(actor(GECKO_REF, GECKO, b"ACRE", "TestGeckoRef"));
    let mut cell = edid("TestImpactCell");
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
    plugin.extend(group(*b"TXST", 0, &sets));
    plugin.extend(group(*b"SOUN", 0, &sounds));
    plugin.extend(group(*b"IPCT", 0, &impacts));
    plugin.extend(group(*b"IPDS", 0, &impact_sets));
    plugin.extend(group(*b"ARMO", 0, &armours));
    plugin.extend(group(*b"WEAP", 0, &weapons));
    plugin.extend(group(*b"BPTD", 0, &bodies));
    plugin.extend(group(*b"NPC_", 0, &npcs));
    plugin.extend(group(*b"CREA", 0, &creatures));
    plugin.extend(cells);
    data.write("FalloutNV.esm", &plugin);
    data
}

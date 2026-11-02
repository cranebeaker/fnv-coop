//! `impacts`: what a weapon's hits play on each material (`world::impacts`),
//! or what hitting a person or creature plays and shows.

use std::io::Write;

use esm::{FormId, LoadOrder};
use world::impacts::{self, Impact, ImpactSet, Material, TextureSet};
use world::scripting::GameState;

use crate::records::{describe_id, find_record};
use crate::CliError;

/// `impacts <WEAPON|PERSON|CREATURE> [MATERIAL]`.
pub fn impacts(
    out: &mut impl Write,
    order: &LoadOrder,
    assets: &assets::Assets,
    target: &str,
    material: Option<&str>,
) -> Result<(), CliError> {
    let material = match material {
        Some(m) => Some(Material::parse(m).ok_or_else(|| {
            let names: Vec<&str> = Material::ALL.iter().map(|m| m.name()).collect();
            CliError::Usage(format!(
                "'{m}' isn't an impact material; they are: {}",
                names.join(", ")
            ))
        })?),
        None => None,
    };
    let found = find_record(order, target)?;
    let kind = found.entry.header.kind;
    match kind.as_bytes() {
        b"WEAP" => weapon(out, order, assets, found.form_id, material),
        b"IPDS" => set(out, order, assets, found.form_id, material),
        b"IPCT" => impact(out, order, assets, found.form_id, ""),
        b"NPC_" | b"CREA" | b"ACHR" | b"ACRE" => actor(out, order, assets, found.form_id),
        _ => Err(CliError::NotFound(format!(
            "{target} is a {kind} record; give a weapon, an impact data set, an impact, a person or a creature"
        ))),
    }
}

fn weapon(
    out: &mut impl Write,
    order: &LoadOrder,
    assets: &assets::Assets,
    id: FormId,
    material: Option<Material>,
) -> Result<(), CliError> {
    writeln!(out, "{}", describe_id(order, id))?;
    match impacts::weapon_impact_set(order, id) {
        Some(s) => {
            writeln!(out, "  impact data set (INAM): {}", describe_id(order, s))?;
            set(out, order, assets, s, material)
        }
        None => {
            writeln!(
                out,
                "  no impact data set (INAM): its hits show and play nothing"
            )?;
            Ok(())
        }
    }
}

fn set(
    out: &mut impl Write,
    order: &LoadOrder,
    assets: &assets::Assets,
    id: FormId,
    material: Option<Material>,
) -> Result<(), CliError> {
    let set = ImpactSet::load(order, id)
        .ok_or_else(|| CliError::NotFound(format!("{} isn't readable", describe_id(order, id))))?;
    let wanted: Vec<Material> = match material {
        Some(m) => vec![m],
        None => Material::ALL.to_vec(),
    };
    for m in wanted {
        match set.get(m) {
            Some(i) => impact(out, order, assets, i, &format!("{:<13}", m.name()))?,
            None => writeln!(out, "  {:<13} nothing", m.name())?,
        }
    }
    writeln!(
        out,
        "  (The game shows the models only within {} units of the camera, fGunParticleCameraDistance.)",
        impacts::effect_distance(order)
    )?;
    Ok(())
}

/// One impact: its model (and how long it plays), decal and sounds.
fn impact(
    out: &mut impl Write,
    order: &LoadOrder,
    assets: &assets::Assets,
    id: FormId,
    label: &str,
) -> Result<(), CliError> {
    let Some(i) = Impact::load(order, id) else {
        writeln!(out, "  {label} {} (not an impact)", describe_id(order, id))?;
        return Ok(());
    };
    writeln!(out, "  {label} {}", describe_id(order, id))?;
    let pad = " ".repeat(label.len() + 3);
    match &i.model {
        Some(m) => {
            let nif = assets
                .read(&assets::mesh_path(m))
                .ok()
                .flatten()
                .and_then(|b| nif::Nif::parse(b).ok());
            let time = nif.as_ref().and_then(cellview::impacts::animation_time);
            let lasts = impacts::effect_lifetime(time, i.duration);
            writeln!(
                out,
                "{pad}model {m}{}; plays {lasts:.2} s ({})",
                if nif.is_none() { " (not found)" } else { "" },
                match time {
                    Some(t) => format!("its animation, {t:.2} s"),
                    None => format!("no animation: the impact's duration {:.2} s", i.duration),
                }
            )?;
        }
        None => writeln!(out, "{pad}no model")?,
    }
    let orientation = match i.orientation {
        impacts::Orientation::SurfaceNormal => "along the surface's normal",
        impacts::Orientation::ProjectileVector => "back along the shot",
        impacts::Orientation::ProjectileReflection => "along the shot's reflection",
    };
    let level = match i.sound_level {
        0 => "loud",
        1 => "normal",
        2 => "silent",
        _ => "?",
    };
    writeln!(
        out,
        "{pad}points {orientation}; angle threshold {}, placement radius {}, noise {level}",
        i.angle_threshold, i.placement_radius
    )?;
    match (i.texture_set, i.has_decal()) {
        (Some(t), true) => {
            let set = TextureSet::load(order, t);
            let d = i.decal.or_else(|| set.as_ref().and_then(|s| s.decal));
            let size = d.map_or(String::new(), |d| {
                format!(", {:.0}–{:.0} across", d.min_width, d.max_width)
            });
            writeln!(
                out,
                "{pad}decal {}{size}: {}",
                describe_id(order, t),
                set.and_then(|s| s.diffuse)
                    .unwrap_or_else(|| "(no texture)".into())
            )?;
        }
        (Some(_), false) => writeln!(out, "{pad}no decal (flagged)")?,
        (None, _) => writeln!(out, "{pad}no decal")?,
    }
    for (name, s) in [("sound", i.sound), ("second sound", i.sound2)] {
        if let Some(s) = s {
            writeln!(out, "{pad}{name} {}", sound(order, s))?;
        }
    }
    Ok(())
}

fn sound(order: &LoadOrder, id: FormId) -> String {
    match world::sound::Sound::load(order, id) {
        Some(s) => format!(
            "{}: {} (heard within {:.0})",
            describe_id(order, id),
            s.file,
            s.max_distance
        ),
        None => describe_id(order, id),
    }
}

fn actor(
    out: &mut impl Write,
    order: &LoadOrder,
    assets: &assets::Assets,
    who: FormId,
) -> Result<(), CliError> {
    let state = GameState::new(order);
    writeln!(out, "{}", describe_id(order, who))?;
    let own = impacts::actor_material(order, who);
    let armour = impacts::power_armour(order, &state, who);
    writeln!(
        out,
        "  material (NAM4): {}{}{}",
        own.name(),
        if armour.body {
            "; power armour on the body: hits there are metal"
        } else {
            ""
        },
        if armour.helmet {
            "; a power armour helmet: head hits are metal"
        } else {
            ""
        }
    )?;
    writeln!(
        out,
        "  blood: {}",
        if impacts::shows_blood(order, who, false) {
            "yes"
        } else {
            "none (no blood spray and no blood decal)"
        }
    )?;
    // An attacker holding no weapon (fists, a bite): the blood comes from
    // the default set at their material (`0088e8d0`).
    let shown = |m: Material| impacts::impact_in(order, impacts::DEFAULT_IMPACT_SET, m);
    if let Some(i) = shown(own) {
        writeln!(
            out,
            "  hit with nothing held (DefaultImpactDataSet, {}):",
            own.name()
        )?;
        impact(out, order, assets, i, "  ")?;
    }
    // Body parts' own sets: spatter on walls behind them.
    if let Some(data) = world::body_parts::BodyPartData::of(order, who) {
        for part in data.parts.iter().flatten() {
            let spatter = impacts::body_part_impact_set(order, who, part.part_type)
                .and_then(|s| impacts::impact_in(order, s, own));
            if let Some(s) = spatter {
                writeln!(
                    out,
                    "  {} ({}): wall spatter {} (chance fCombatEnvironmentBloodChance)",
                    part.name,
                    part.part_type,
                    describe_id(order, s)
                )?;
            }
        }
    }
    if world::combat::is_creature(order, who)
        || order
            .get(who)
            .is_some_and(|r| r.entry.header.kind.as_bytes() == b"CREA")
    {
        if let Some(s) = impacts::creature_impact_set(order, who) {
            writeln!(out, "  bites with (CNAM): {}", describe_id(order, s))?;
        }
        for (kind, name) in [
            (impacts::creature_sound::WEAPON, "its blows (weapon)"),
            (impacts::creature_sound::DEATH, "dying"),
            (
                impacts::creature_sound::HIT,
                "hit (never played by the engine)",
            ),
        ] {
            let list = impacts::creature_sounds(order, who, kind);
            for (s, chance) in list {
                writeln!(out, "  sound when {name}: {} at {chance}%", sound(order, s))?;
            }
        }
    }
    // Their hurt and death lines.
    let base = world::scripting::base_of(order, who).unwrap_or(who);
    let reference = if base == who { FormId(0) } else { who };
    if let Some(speaker) = world::dialogue::Speaker::load(order, reference, base) {
        for (topic, name) in [
            (impacts::HIT_TOPIC, "hurt"),
            (impacts::DEATH_TOPIC, "dying"),
        ] {
            match world::dialogue::pick(order, topic, &speaker, &state) {
                Some(info) => {
                    let text = info
                        .responses
                        .first()
                        .map_or(String::new(), |r| r.text.clone());
                    let file = info
                        .responses
                        .first()
                        .zip(speaker.voice)
                        .and_then(|(r, v)| world::dialogue::voice_path(order, &info, r, v))
                        .unwrap_or_else(|| "(no voice type)".into());
                    let found = assets.read(&file).ok().flatten().is_some();
                    writeln!(
                        out,
                        "  says when {name}: \"{text}\" {file}{}",
                        if found { "" } else { " (not found)" }
                    )?;
                }
                None => writeln!(out, "  says when {name}: nothing")?,
            }
        }
    }
    Ok(())
}

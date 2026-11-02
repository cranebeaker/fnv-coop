//! `vats <TARGET> [WEAPON] [DISTANCE]`: what V.A.T.S. offers a new
//! character against someone (`world::vats`): their action points, what an
//! attack with the weapon costs and how many shots it fires, the gun's
//! wobble, and every part the target offers with the chance shown for it,
//! fully in view.

use std::io::Write;

use esm::LoadOrder;
use world::body_parts::BodyPartData;
use world::combat::{self, Weapon};
use world::dialogue::PLAYER_REF;
use world::scripting::{Facts, GameState};
use world::vats::{self, ChanceQuery, Settings, Stance};

use crate::records::{describe_id, find_record};
use crate::CliError;

/// The distances the table runs through besides the one asked for.
const DISTANCES: [f32; 6] = [100.0, 250.0, 500.0, 1000.0, 1500.0, 2500.0];

pub fn vats(
    out: &mut impl Write,
    order: &LoadOrder,
    target: &str,
    weapon: Option<&str>,
    distance: Option<f32>,
) -> Result<(), CliError> {
    let who = find_record(order, target)?.form_id;
    let kind = order.get(who).map(|r| r.entry.header.kind);
    let is_base = kind.is_some_and(|k| matches!(k.as_bytes(), b"NPC_" | b"CREA"));
    let weapon = match weapon {
        Some(w) => {
            let id = find_record(order, w)?.form_id;
            Some(
                Weapon::load(order, id)
                    .ok_or_else(|| CliError::NotFound(format!("{w} isn't a weapon (WEAP)")))?,
            )
        }
        None => None,
    };
    let s = Settings::load(order);
    let mut state = GameState::new(order);
    if let Some(w) = &weapon {
        state.equipped.insert(PLAYER_REF, vec![w.form_id]);
    }
    let facts = Facts {
        order,
        state: &state,
        speaker: None,
    };
    let av = |a: u16| facts.current_actor_value(PLAYER_REF, a).unwrap_or(0.0);
    let name = weapon
        .as_ref()
        .map_or("fists".to_string(), |w| w.name.clone());
    writeln!(
        out,
        "A new character (Agility {:.0}): {:.0} action points, back at {:.0}% of them a second outside V.A.T.S.",
        av(10),
        vats::max_action_points(order, &state),
        s.restore_rate * 100.0
    )?;
    let cost = vats::attack_cost(order, &state, &s, weapon.as_ref());
    let k = vats::attack_kind(weapon.as_ref().map(|w| w.animation));
    let (shots, extra) = vats::shots(&s, weapon.as_ref());
    let own = weapon
        .as_ref()
        .filter(|w| w.flags2 & vats::flags::OVERRIDE_AP != 0)
        .map_or("the setting for its kind", |_| "the weapon's own");
    let label = vats::kind_label(k).to_lowercase();
    let article = if label.starts_with(['a', 'e', 'i', 'o', 'u']) {
        "an"
    } else {
        "a"
    };
    write!(
        out,
        "With the {name}: {article} {label} costs {cost:.0} ({own}), {shots} shot{}",
        if shots == 1 { "" } else { "s" }
    )?;
    if extra > 0 {
        write!(out, " (+{extra} in a long burst)")?;
    }
    writeln!(out)?;
    if let Some(w) = &weapon {
        let extra_fields = vats::WeaponVats::load(order, w.form_id).unwrap_or_default();
        let skill = av(w.skill);
        let wobble = vats::wobble(order, &state, &s, PLAYER_REF, Some(w), Stance::SCANNING);
        writeln!(
            out,
            "  {} {skill:.0} (asks {}), Strength {:.0} (asks {}); wobble {wobble:.4} × {:.0}° = {:.3}°, + min spread {}° = {:.3}°; to-hit as a target {}",
            world::chargen::actor_value_name(order, w.skill),
            extra_fields.skill_req,
            av(5),
            extra_fields.strength_req,
            s.npc_max_gun_wobble,
            wobble * s.npc_max_gun_wobble,
            w.min_spread,
            w.min_spread + wobble * s.npc_max_gun_wobble,
            extra_fields.to_hit
        )?;
        if w.clip > 0 && !w.ammo.is_empty() {
            writeln!(
                out,
                "  clip {}: a queue reloads when it runs out (+{:.0} action points)",
                w.clip,
                s.cost(vats::kind::RELOAD)
            )?;
        }
    }
    for sp in vats::specials(order, &state, &s, weapon.as_ref(), false) {
        writeln!(out, "  special: {} for {:.0}", sp.name, sp.cost)?;
    }
    // The target.
    let held = if is_base {
        world::actor::carried_weapon(order, who).and_then(|w| Weapon::load(order, w))
    } else {
        combat::weapon_in_hand(order, &state, who)
    };
    let data = BodyPartData::of(order, who);
    writeln!(
        out,
        "\nTarget {}: size {:.1} (its OBND diagonal), body part data {}{}",
        describe_id(order, who),
        vats::bound_of(order, who),
        data.as_ref()
            .map_or("none".to_string(), |d| describe_id(order, d.form_id)),
        held.as_ref()
            .map_or(String::new(), |w| format!(", holding the {}", w.name))
    )?;
    if vats::banned_target(order, who) {
        writeln!(out, "  (in BannedVATSTargets: V.A.T.S. never offers them)")?;
    }
    if weapon
        .as_ref()
        .is_some_and(|w| vats::banned_weapon(order, w.form_id))
    {
        writeln!(
            out,
            "  (the {name} is in VATSBannedWeaponsList: V.A.T.S. won't open)"
        )?;
    }
    let parts = vats::parts_offered(data.as_ref(), held.as_ref());
    let mut distances: Vec<f32> = DISTANCES.to_vec();
    if let Some(d) = distance {
        if !distances.contains(&d) {
            distances.push(d);
            distances.sort_by(|a, b| a.total_cmp(b));
        }
    }
    writeln!(
        out,
        "Chances shown, fully in view, by the distance between the bodies' edges (people's centres are 40.5 farther apart):"
    )?;
    write!(out, "  {:<16}{:>7}", "part", "to-hit")?;
    for d in &distances {
        write!(out, "{:>8} ", format!("{d:.0}"))?;
    }
    writeln!(out)?;
    for p in &parts {
        let bp = data
            .as_ref()
            .and_then(|d| u8::try_from(p.slot).ok().and_then(|slot| d.part(slot)));
        let to_hit = match bp {
            Some(b) => b.to_hit_chance,
            None => held
                .as_ref()
                .and_then(|w| vats::WeaponVats::load(order, w.form_id))
                .map_or(1, |v| v.to_hit),
        };
        write!(out, "  {:<16}{to_hit:>7}", p.name)?;
        for &d in &distances {
            let chance = vats::part_chance(
                order,
                &mut state,
                &s,
                &ChanceQuery {
                    target: who,
                    part: bp,
                    part_av: p.actor_value,
                    weapon: weapon.as_ref(),
                    distance: d,
                    visible: 1.0,
                    stance: Stance::SCANNING,
                },
            );
            let shown = vats::shown_percent(&s, chance, 0, false);
            let mark = if Some(d) == distance { "*" } else { " " };
            write!(out, "{:>7}%{mark}", shown)?;
        }
        writeln!(out)?;
    }
    if let Some(d) = distance {
        writeln!(out, "  (* the distance asked for, {d:.0})")?;
    }
    Ok(())
}

/// One condition as the editor writes it.
fn condition_text(order: &LoadOrder, c: &world::dialogue::Condition) -> String {
    use world::dialogue::Comparison;
    let op = match c.comparison {
        Comparison::Equal => "==",
        Comparison::NotEqual => "!=",
        Comparison::Greater => ">",
        Comparison::GreaterOrEqual => ">=",
        Comparison::Less => "<",
        Comparison::LessOrEqual => "<=",
    };
    let on = match c.run_on {
        0 => "subject (the attacker)".to_string(),
        1 => "target".to_string(),
        2 => c
            .reference
            .map_or("a reference".to_string(), |r| describe_id(order, r)),
        n => format!("run on {n}"),
    };
    let params = script::functions::FUNCTIONS
        .get(usize::from(c.function))
        .map_or(&[][..], |s| s.params);
    let args: Vec<String> = (0..params.len().min(2))
        .map(|i| {
            if script::param_is_form(params[i].kind) {
                describe_id(order, c.param_forms[i])
            } else {
                format!("{}", c.params[i] as i32)
            }
        })
        .collect();
    let value = match c.global {
        Some(g) => describe_id(order, g),
        None => format!("{}", c.value),
    };
    format!(
        "{} {}({}) {op} {value}{}",
        on,
        c.function_name(),
        args.join(", "),
        if c.or { "  OR" } else { "" }
    )
}

/// `vats-cameras [ID]`: the camera paths (`CPTH`) in the game's tree, each
/// with its conditions, zoom and shots (`CAMS`), or one path's or shot's.
pub fn cameras(
    out: &mut impl Write,
    order: &LoadOrder,
    which: Option<&str>,
) -> Result<(), CliError> {
    use world::vats_camera::{CameraPaths, CameraShot};
    let paths = CameraPaths::load(order);
    let shot_line = |id: esm::FormId| -> String {
        match CameraShot::load(order, id) {
            Some(s) => format!(
                "{} {}: action {} at {} looking at {}, flags {:#x}, × player {} target {} world {}, {}–{} s, {}, image space {}",
                id,
                s.editor_id,
                s.action,
                s.location,
                s.target,
                s.flags,
                s.player_mult,
                s.target_mult,
                s.global_mult,
                s.min_time,
                s.max_time,
                s.model,
                s.image_space.map_or("none".into(), |i| describe_id(order, i))
            ),
            None => format!("{id} (not a camera shot)"),
        }
    };
    if let Some(w) = which {
        let id = find_record(order, w)?.form_id;
        if let Some(p) = paths.paths.get(&id) {
            writeln!(out, "{} {}", p.form_id, p.editor_id)?;
            for c in &p.conditions {
                writeln!(out, "  if {}", condition_text(order, c))?;
            }
            writeln!(
                out,
                "  zoom {:#04x}, parent {}, after {}",
                p.zoom,
                p.parent.map_or("none".into(), |x| describe_id(order, x)),
                p.previous.map_or("none".into(), |x| describe_id(order, x))
            )?;
            for s in &p.shots {
                writeln!(out, "  shot {}", shot_line(*s))?;
            }
            for c in &p.children {
                writeln!(out, "  child {}", describe_id(order, *c))?;
            }
            return Ok(());
        }
        writeln!(out, "{}", shot_line(id))?;
        return Ok(());
    }
    fn walk(
        out: &mut impl Write,
        order: &LoadOrder,
        paths: &CameraPaths,
        id: esm::FormId,
        depth: usize,
        shot_line: &dyn Fn(esm::FormId) -> String,
    ) -> Result<(), CliError> {
        let Some(p) = paths.paths.get(&id) else {
            return Ok(());
        };
        let pad = "  ".repeat(depth);
        writeln!(
            out,
            "{pad}{} {}{}{}",
            p.form_id,
            p.editor_id,
            if p.zoom != 0 {
                format!("  [zoom {:#04x}]", p.zoom)
            } else {
                String::new()
            },
            if p.deleted { "  (deleted)" } else { "" }
        )?;
        for c in &p.conditions {
            writeln!(out, "{pad}    if {}", condition_text(order, c))?;
        }
        for s in &p.shots {
            writeln!(out, "{pad}    shot {}", shot_line(*s))?;
        }
        for &c in &p.children {
            walk(out, order, paths, c, depth + 1, shot_line)?;
        }
        Ok(())
    }
    writeln!(
        out,
        "{} camera paths; the top level, in the order an attack tries them:",
        paths.paths.len()
    )?;
    for id in paths.top.iter().flatten() {
        walk(out, order, &paths, *id, 0, &shot_line)?;
    }
    Ok(())
}

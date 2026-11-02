//! `lockpick <REF|LEVEL> [SKILL]`: the lockpicking game for a lock (a
//! placed reference with `XLOC`, or a lock level) and a Lockpick skill (a
//! new character's when not given), worked out as the game does
//! (`world::lockpick`): whether the menu opens, the sweet spot and its
//! rings at this install's screen, how far the lock turns where, how long
//! a pin lasts, forcing's chance; and the menu's models, lights and camera.

use std::io::Write;
use std::path::Path;

use assets::{Assets, IniSettings};
use esm::LoadOrder;
use world::lockpick::{Picking, Refusal, Settings};

use crate::CliError;

/// The screen the game draws at (`[Display] iSize W` / `iSize H`).
fn screen_size(ini: &IniSettings) -> (u32, u32) {
    let get = |k: &str| {
        ini.get("Display", k)
            .and_then(|v| v.trim().parse::<u32>().ok())
    };
    (
        get("iSize W").unwrap_or(1920),
        get("iSize H").unwrap_or(1080),
    )
}

pub fn lockpick(
    out: &mut impl Write,
    order: &LoadOrder,
    assets: &Assets,
    data_dir: &Path,
    target: &str,
    skill: Option<i32>,
) -> Result<(), CliError> {
    let state = world::scripting::GameState::new(order);
    let lock = match target.parse::<u8>() {
        Ok(level) => {
            writeln!(out, "A lock of level {level}")?;
            world::locks::Lock { level, key: None }
        }
        Err(_) => {
            let rr = crate::records::find_record(order, target)?;
            let lock = world::locks::placed_lock(order, rr.form_id)
                .ok_or_else(|| CliError::NotFound(format!("{} has no lock (XLOC)", rr.form_id)))?;
            let name = rr
                .record()
                .ok()
                .and_then(|r| r.editor_id())
                .unwrap_or_default();
            writeln!(out, "{name} ({}): lock level {}", rr.form_id, lock.level)?;
            if let Some(key) = lock.key {
                writeln!(out, "  its key: {key}")?;
            }
            if let Some(owner) = world::crime::owner_of(order, &state, rr.form_id) {
                let evil = world::crime::owner_is_evil(order, owner);
                writeln!(
                    out,
                    "  owned by {owner}: picking it is stealing (−5 karma{}, a crime if a member sees it)",
                    if evil { " not given: the owner is evil" } else { "" }
                )?;
            }
            lock
        }
    };
    let (difficulty, needs) = lock.difficulty(order);
    if lock.key_only() || difficulty > 4 {
        writeln!(
            out,
            "{}: only its key opens it ({})",
            world::lockpick::level_name(order, 5),
            world::lockpick::message_text(
                order,
                "sImpossibleLock",
                "This lock cannot be picked. It requires a key to open."
            )
        )?;
        return Ok(());
    }
    let given = skill.is_some();
    let skill = skill.unwrap_or_else(|| world::lockpick::player_skill(order, &state));
    writeln!(
        out,
        "{} lock: needs Lockpick {needs}; Lockpick {skill}{}",
        world::lockpick::level_name(order, difficulty),
        if given { "" } else { " (a new character's)" }
    )?;
    let settings = Settings::load(order);

    // The menu at this install's screen: the meter the pick runs along.
    let ini = IniSettings::load(&assets::default_settings_files(data_dir));
    let (w, h) = screen_size(&ini);
    let mut read = |p: &str| assets.read(p).ok().flatten();
    let ini_get = |s: &str, k: &str| ini.get(s, k).map(str::to_string);
    let mut ui = ui::game::new_ui(
        &mut read,
        &ini_get,
        crate::ui_cmd::text_settings(order),
        w,
        h,
    );
    let menu =
        ui::lockpick::LockpickMenu::load(&mut ui, &mut read, false).map_err(CliError::NotFound)?;
    let meter = menu.meter_width(&mut ui);
    writeln!(
        out,
        "Screen {w} x {h}: the pick runs along {meter:.2} menu units for its 180° (+90° at the left end, −90° at the right), {:.4}° a unit",
        180.0 / meter
    )?;

    let models = cellview::lockpick::lockpick_scene(assets);
    let spans = models.as_ref().map_or_else(Default::default, |m| {
        cellview::lockpick::spans(&m.lock_sequences, &m.pin_sequences)
    });
    let picking = match Picking::open(
        esm::FormId(0),
        difficulty,
        skill,
        world::lockpick::pins(order, &state),
        settings,
        meter,
        0.5,
        spans,
    ) {
        Ok(p) => p,
        Err(Refusal::SkillTooLow(n)) => {
            writeln!(
                out,
                "The menu doesn't open: {}",
                world::lockpick::message_text(
                    order,
                    "sLockpickSkillTooLow",
                    "You need a lockpick skill of %d to pick this lock."
                )
                .replace("%d", &n.to_string())
            )?;
            return Ok(());
        }
    };
    let z = &picking.zone;
    writeln!(
        out,
        "\nSweet spot (iSweetSpotLength {:?}, iConcentricLength {:?}, mixed {:.3}): {} units across, rings every {} units; outside the rings past {} units from its centre",
        settings.sweet_spot[usize::from(difficulty)],
        settings.concentric[usize::from(difficulty)],
        settings.mix(difficulty, skill),
        z.sweet,
        z.concentric / 5,
        z.half
    )?;
    writeln!(
        out,
        "  (its centre is random, kept {} units from the ends; here put in the middle, {:.2})",
        z.half, z.center
    )?;
    writeln!(
        out,
        "  ring  x      half width   pick within    the lock turns"
    )?;
    for ring in &z.rings {
        let half = ring.width / 2.0;
        writeln!(
            out,
            "  {:4} {:5}  {:10.2}   ±{:6.2}°       {:5.1}°",
            ring.id,
            ring.x,
            half,
            half * 180.0 / meter,
            z.max_turn(z.center + half - 0.01).unwrap_or(0.0)
        )?;
    }
    writeln!(
        out,
        "  outside: 15° at the edge, falling to 0 at the meter's far end ({:.1}° at the left end here)",
        z.max_turn(0.0).unwrap_or(0.0)
    )?;
    let health = settings.pin_health(skill);
    writeln!(
        out,
        "\nA pin: health {health:.2} (1 + {skill} × fLockpickBonusHealth {}); straining takes 1 / fPickBreakSecs ({}) a second: it breaks after {:.2} s, bending from {:.2} s",
        settings.bonus_health,
        settings.break_secs,
        health * settings.break_secs,
        ((health - 1.0) * settings.break_secs).max(0.0)
    )?;
    writeln!(
        out,
        "The cylinder turns and springs back 90° a second; the lock opens at 90°."
    )?;
    writeln!(
        out,
        "{}: forcing succeeds {}% of the time (Lockpick − {needs} + fLockSkillBase {}); failing breaks the lock",
        picking.force_label(&world::lockpick::message_text(order, "sForceLock", "Force Lock")),
        settings.force_chance(difficulty, skill),
        settings.skill_base as i32
    )?;
    writeln!(
        out,
        "Bobby pins (a new character): {}",
        world::lockpick::pins(order, &state)
    )?;

    match models {
        Some(m) => {
            writeln!(
                out,
                "\nModels: {} and {}",
                world::lockpick::LOCK_MODEL,
                world::lockpick::PIN_MODEL
            )?;
            for (label, list) in [("lock", &m.lock_sequences), ("pin", &m.pin_sequences)] {
                for s in list.iter() {
                    writeln!(
                        out,
                        "  {label} {:9} {:.3} .. {:.3} s",
                        s.name, s.start, s.stop
                    )?;
                }
            }
            writeln!(
                out,
                "  {} pieces; the scene's bounding radius {:.2}, so the lights reach {:.2}",
                m.scene.draws.len(),
                m.bound_radius,
                m.bound_radius * cellview::lockpick::LIGHT_RADIUS_MULT
            )?;
            for l in &m.lights {
                writeln!(
                    out,
                    "  light at ({:.2}, {:.2}, {:.2}), colour ({:.3}, {:.3}, {:.3}) × {:.2}",
                    l.position[0],
                    l.position[1],
                    l.position[2],
                    l.color[0],
                    l.color[1],
                    l.color[2],
                    l.fade
                )?;
            }
        }
        None => writeln!(out, "\nThe menu's models can't be read.")?,
    }
    let camera = cellview::lockpick::MenuCamera::new(&ini, w, h);
    writeln!(
        out,
        "Camera: at the origin looking along +x; tan of half the width {:.5}, of half the height {:.5} ({:.2}° across, {:.2}° up and down); near {}",
        camera.tan_half_width,
        camera.tan_half_height,
        2.0 * camera.tan_half_width.atan().to_degrees(),
        camera.vertical_fov().to_degrees(),
        camera.near
    )?;
    Ok(())
}

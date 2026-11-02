//! `living`: sleeping, hardcore needs, pickpocketing and trespassing
//! (`world::living`) on the real data. Without a cell: the needs' stages
//! and the settings. With one: what a new character standing at its
//! arrival point (or at X,Y,Z) would meet: trespassing, waiting, each
//! bed's answer, who sees them, each person's pockets and the chances.

use std::io::Write;

use esm::LoadOrder;
use world::dialogue::PLAYER_REF;
use world::living::{needs, pickpocket, sleep};
use world::scripting::GameState;

use crate::records::describe_id;
use crate::CliError;

pub fn living(
    out: &mut impl Write,
    order: &LoadOrder,
    cell: Option<&str>,
    at: Option<&str>,
) -> Result<(), CliError> {
    let Some(cell) = cell else {
        return stages(out, order);
    };
    let at = match at {
        Some(text) => {
            let v: Vec<f32> = text
                .split(',')
                .map(|n| n.trim().parse::<f32>())
                .collect::<Result<_, _>>()
                .map_err(|_| CliError::Usage(format!("not X,Y,Z: {text}")))?;
            if v.len() != 3 {
                return Err(CliError::Usage(format!("not X,Y,Z: {text}")));
            }
            Some([v[0], v[1], v[2]])
        }
        None => None,
    };
    let id = world::find_cells(order, cell)
        .map_err(crate::render_cmd::world_error)?
        .into_iter()
        .next()
        .ok_or_else(|| CliError::NotFound(format!("no cell '{cell}'")))?;
    let loaded = world::load_cell(order, id).map_err(crate::render_cmd::world_error)?;
    let mut state = GameState::new(order);
    state.player_cell = Some(id);
    state.player_world = None;
    state.player_position =
        Some(at.unwrap_or_else(|| loaded.arrivals.first().map_or([0.0; 3], |a| a.position)));
    let owner = world::script_functions::cell_owner_now(order, &state, id);
    writeln!(
        out,
        "{}: owner {}, {}public; a new character is {}trespassing here",
        describe_id(order, id),
        owner.map_or("none".to_string(), |o| describe_id(order, o)),
        if world::script_functions::cell_is_public(order, &state, id) {
            ""
        } else {
            "not "
        },
        if world::crime::trespassing(order, &state, id) {
            ""
        } else {
            "not "
        }
    )?;
    writeln!(
        out,
        "  waiting: {}",
        match sleep::may_wait(order, &state) {
            Ok(()) => "allowed".to_string(),
            Err(why) => why,
        }
    )?;
    let refs = order.references_in_cell(id);
    for rr in &refs {
        let r = rr.form_id;
        if GameState::is_furniture(order, r) && !sleep::is_bed(order, r) {
            writeln!(
                out,
                "  seat {}: owner {} (sitting is never refused)",
                describe_id(order, r),
                world::crime::owner_of(order, &state, r)
                    .map_or("none".to_string(), |o| { describe_id(order, o) }),
            )?;
        }
        if GameState::is_furniture(order, r) && sleep::is_bed(order, r) {
            writeln!(
                out,
                "  bed {}: owner {}; {}{}",
                describe_id(order, r),
                world::crime::owner_of(order, &state, r)
                    .map_or("none".to_string(), |o| { describe_id(order, o) }),
                match sleep::may_sleep_in(order, &state, r) {
                    Ok(()) => "sleep allowed".to_string(),
                    Err(why) => why,
                },
                if world::living::crosshair_red(order, &state, r) == Some(true) {
                    " (the crosshair's text is red)"
                } else {
                    ""
                }
            )?;
        }
    }
    // Who would see the player standing at the arrival point (not
    // sneaking), and so come to warn a trespasser.
    for rr in &refs {
        if rr.entry.header.kind.as_bytes() != b"ACHR" {
            continue;
        }
        let facts = world::scripting::Facts {
            order,
            state: &state,
            speaker: None,
        };
        if let Some(v) = facts.detection(rr.form_id, PLAYER_REF) {
            let at = state.place(order, rr.form_id).map_or([0.0; 3], |p| p.2);
            let feet = state.player_position.unwrap_or_default();
            let d = (0..3)
                .map(|i| (at[i] - feet[i]).powi(2))
                .sum::<f32>()
                .sqrt();
            writeln!(
                out,
                "  {} ({d:.0} units away) detects a player standing there: {v}",
                describe_id(order, rr.form_id)
            )?;
        }
    }
    state.player_sneaking = true;
    let sneak = world::scripting::Facts {
        order,
        state: &state,
        speaker: None,
    }
    .current_actor_value(PLAYER_REF, 42)
    .unwrap_or(0.0);
    writeln!(out, "  sneaking with Sneak {sneak:.0}:")?;
    for rr in &refs {
        if rr.entry.header.kind.as_bytes() != b"ACHR" {
            continue;
        }
        let who = rr.form_id;
        let used = pickpocket::use_person(order, &state, who, false);
        writeln!(out, "    {}: {used:?}", describe_id(order, who))?;
        if used != pickpocket::Use::Pickpocket {
            continue;
        }
        state.stock(order, who);
        for (item, n) in state.inventory(order, who) {
            let v = pickpocket::stack_value(order, &state, item, n);
            writeln!(
                out,
                "      {} x{n}: value {v:.0}, chance {}%",
                describe_id(order, item),
                if v <= 0.0 {
                    100
                } else {
                    pickpocket::chance(order, &state, who, v)
                }
            )?;
        }
        writeln!(
            out,
            "      placing anything: {}%",
            pickpocket::chance(order, &state, who, 1.0)
        )?;
    }
    Ok(())
}

/// The needs' stages, highest first, and the settings that drive them.
fn stages(out: &mut impl Write, order: &LoadOrder) -> Result<(), CliError> {
    for (av, name) in [
        (needs::DEHYDRATION, "Dehydration"),
        (needs::HUNGER, "Hunger"),
        (needs::SLEEP_DEPRIVATION, "Sleep deprivation"),
        (needs::RADS, "Radiation"),
    ] {
        writeln!(out, "{name} (actor value {av}):")?;
        for s in needs::stages(order, av) {
            writeln!(
                out,
                "  {:>5}  {}  {}",
                s.threshold,
                describe_id(order, s.record),
                describe_id(order, s.spell)
            )?;
        }
    }
    let setting = |n: &str, exe: f32| world::scripting::game_setting(order, n).unwrap_or(exe);
    let scale = GameState::new(order)
        .global(order, "TimeScale")
        .unwrap_or(30.0);
    writeln!(out, "TimeScale {scale}: a point per")?;
    for (n, exe) in [
        ("fHCDehydrationRate", 10.0),
        ("fHCStarvationRate", 0.39),
        ("fHCSleepDeprivationRate", 0.26),
    ] {
        let rate = setting(n, exe);
        writeln!(
            out,
            "  {n} {rate}: {rate} real seconds of game time = {:.1} game minutes",
            rate * scale / 60.0
        )?;
    }
    writeln!(
        out,
        "Sleeping lowers sleep deprivation by min(it, fHCSleepRestorationMod {}) an hour",
        setting("fHCSleepRestorationMod", 60.0)
    )?;
    Ok(())
}

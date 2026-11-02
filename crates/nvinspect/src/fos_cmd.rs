//! `nvinspect fos <SAVE> [PLUGIN]`: what's in one of the original game's
//! saves (`.fos`), read with the `fos` crate (docs/FOS_SAVES.md). Read-only.
//!
//! Prints the header, plugins, the location table, the global data, the
//! change forms counted by type and change flag, the quests and globals
//! decoded, and the check that every part (and every decodable change
//! form) ends exactly where it should. With a plugin (e.g. FalloutNV.esm)
//! forms from it are named by editor ID.

use std::collections::BTreeMap;
use std::io::Write;
use std::path::Path;

use fos::decode::{self, quest_flag, Coverage, InitialData};
use fos::{change_flag_name, global_data_name, RefId, Save, SAVE_TYPES};

use crate::fmt::thousands;
use crate::{file_name_of, CliError};

/// Names forms from one plugin by editor ID.
struct Names {
    plugin: esm::Plugin,
    /// The plugin's index in the save's plugin list.
    save_index: Option<u8>,
    /// The plugin's own index for its records (its master count).
    own_index: u32,
}

impl Names {
    fn open(path: &Path, save: &Save<'_>) -> Result<Self, CliError> {
        let plugin = esm::Plugin::open(path).map_err(|e| CliError::InFile {
            path: path.display().to_string(),
            message: e.to_string(),
        })?;
        let name = file_name_of(path);
        let save_index = save
            .plugins
            .iter()
            .position(|p| p.eq_ignore_ascii_case(&name))
            .and_then(|i| u8::try_from(i).ok());
        let own_index = plugin.header().masters.len() as u32;
        Ok(Names {
            plugin,
            save_index,
            own_index,
        })
    }

    fn editor_id(&self, form_id: u32) -> Option<String> {
        if Some((form_id >> 24) as u8) != self.save_index {
            return None;
        }
        let id = esm::FormId(self.own_index << 24 | (form_id & 0xFF_FFFF));
        let entry = self.plugin.get(id)?;
        self.plugin.editor_id_of(entry).ok().flatten()
    }
}

fn describe(save: &Save<'_>, names: Option<&Names>, r: RefId) -> String {
    match save.form_id(r) {
        None => format!("refID {:06X} (none)", r.0),
        Some(id) => match names.and_then(|n| n.editor_id(id)) {
            Some(ed) => format!("{id:08X} {ed}"),
            None => format!("{id:08X}"),
        },
    }
}

pub fn run(out: &mut impl Write, rest: &[String]) -> Result<(), CliError> {
    crate::expect_args("fos", rest, 1, 1)?;
    let path = Path::new(&rest[0]);
    let bytes = std::fs::read(path).map_err(|e| CliError::Open {
        path: rest[0].clone(),
        message: e.to_string(),
    })?;
    let save = Save::parse(&bytes).map_err(|e| CliError::InFile {
        path: rest[0].clone(),
        message: e.to_string(),
    })?;
    let names = match rest.get(1) {
        Some(p) => Some(Names::open(Path::new(p), &save)?),
        None => None,
    };
    let failures = report(out, &file_name_of(path), bytes.len(), &save, names.as_ref())?;
    if failures > 0 {
        return Err(CliError::InFile {
            path: rest[0].clone(),
            message: format!("{failures} change forms didn't decode to their length"),
        });
    }
    Ok(())
}

fn report(
    out: &mut impl Write,
    file: &str,
    size: usize,
    save: &Save<'_>,
    names: Option<&Names>,
) -> Result<usize, CliError> {
    let h = &save.header;
    writeln!(out, "{file}: {} bytes", thousands(size))?;
    writeln!(
        out,
        "  version {:#x}, minor version {}, language {}",
        h.version, save.minor_version, h.language
    )?;
    writeln!(
        out,
        "  {}, level {}, \"{}\", at {}, played {}, save number {}",
        h.player_name, h.level, h.karma_title, h.location, h.play_time, h.save_number
    )?;
    writeln!(
        out,
        "  screenshot {} x {} at {:#x}",
        h.screenshot_width, h.screenshot_height, save.screenshot_offset
    )?;

    writeln!(out, "\nPlugins ({}):", save.plugins.len())?;
    for (i, p) in save.plugins.iter().enumerate() {
        writeln!(out, "  {i:02X} {p}")?;
    }

    let t = &save.table;
    writeln!(out, "\nLocation table:")?;
    writeln!(
        out,
        "  global data 1   {:#9x}  {}",
        t.global_data_1,
        plural(t.global_data_1_count as usize, "entry", "entries")
    )?;
    writeln!(
        out,
        "  change forms    {:#9x}  {}",
        t.change_forms, t.change_form_count
    )?;
    writeln!(
        out,
        "  global data 2   {:#9x}  {}",
        t.global_data_2,
        plural(t.global_data_2_count as usize, "entry", "entries")
    )?;
    writeln!(
        out,
        "  form id array   {:#9x}  {}, then {}",
        t.form_id_array,
        plural(save.form_ids.len(), "id", "ids"),
        plural(save.worldspaces.len(), "worldspace", "worldspaces")
    )?;
    writeln!(
        out,
        "  history         {:#9x}  {}",
        t.history,
        plural(save.history.len(), "entry", "entries")
    )?;

    writeln!(out, "\nGlobal data:")?;
    for g in save.global_data_1.iter().chain(&save.global_data_2) {
        writeln!(
            out,
            "  {:>4} {:<18} {:#9x} {:>7} bytes",
            g.kind,
            global_data_name(g.kind).unwrap_or("?"),
            g.offset,
            thousands(g.data.len())
        )?;
    }

    let failures = change_forms(out, save)?;
    quests(out, save, names)?;
    globals(out, save, names)?;
    misc_stats(out, save)?;
    player(out, save, names)?;

    bytes(out, size, save)?;
    Ok(failures)
}

fn plural(n: usize, one: &str, many: &str) -> String {
    format!("{n} {}", if n == 1 { one } else { many })
}

/// The file's bytes part by part. The parser has checked that each part
/// ends where the next begins, so these add up to the file's size.
fn bytes(out: &mut impl Write, size: usize, save: &Save<'_>) -> Result<(), CliError> {
    let t = &save.table;
    let h = &save.header;
    let shot = (h.screenshot_width * h.screenshot_height * 3) as usize;
    let table = t.global_data_1 as usize - fos::LOCATION_TABLE_SIZE;
    let headers: usize = save
        .change_forms
        .iter()
        .map(|c| c.data_offset - c.offset)
        .sum();
    let data: usize = save.change_forms.iter().map(|c| c.data.len()).sum();
    let gap = |from: u32, to: u32| to as usize - from as usize;
    let parts = [
        ("magic and header", save.screenshot_offset),
        ("screenshot", shot),
        (
            "minor version and plugins",
            table - save.screenshot_offset - shot,
        ),
        ("location table", fos::LOCATION_TABLE_SIZE),
        ("global data 1", gap(t.global_data_1, t.change_forms)),
        ("change form headers", headers),
        ("change form data", data),
        ("global data 2", gap(t.global_data_2, t.form_id_array)),
        ("form id arrays", gap(t.form_id_array, t.history)),
        ("history", size - t.history as usize),
    ];
    writeln!(out, "\nBytes:")?;
    for (name, n) in parts {
        writeln!(out, "  {name:<26} {:>10}", thousands(n))?;
    }
    let sum: usize = parts.iter().map(|p| p.1).sum();
    writeln!(
        out,
        "  {:<26} {:>10} of {} (every part ends where the location table says)",
        "total",
        thousands(sum),
        thousands(size)
    )?;
    Ok(())
}

#[derive(Default)]
struct TypeCount {
    count: usize,
    bytes: usize,
    exact: usize,
    skipped: usize,
    failed: usize,
    flags: BTreeMap<u32, usize>,
}

fn change_forms(out: &mut impl Write, save: &Save<'_>) -> Result<usize, CliError> {
    let mut by_type: BTreeMap<u8, TypeCount> = BTreeMap::new();
    let mut failures = Vec::new();
    for cf in &save.change_forms {
        let c = by_type.entry(cf.save_type).or_default();
        c.count += 1;
        c.bytes += cf.data.len();
        for bit in (0..32).map(|b| 1u32 << b).filter(|b| cf.flags & b != 0) {
            *c.flags.entry(bit).or_default() += 1;
        }
        match decode::coverage_of(cf, save.form_id(cf.ref_id)) {
            Coverage::Exact => c.exact += 1,
            Coverage::Skipped(_) => c.skipped += 1,
            Coverage::Failed(e) => {
                c.failed += 1;
                failures.push((cf, e));
            }
        }
    }
    let total: usize = save.change_forms.iter().map(|c| c.data.len()).sum();
    writeln!(
        out,
        "\nChange forms: {} ({} bytes of data)",
        save.change_forms.len(),
        thousands(total)
    )?;
    writeln!(
        out,
        "  type  count      bytes  decoded exactly  not decoded  failed"
    )?;
    let type_name = |t: u8| SAVE_TYPES.get(t as usize).map_or("?", |t| t.0);
    for (&t, c) in &by_type {
        writeln!(
            out,
            "  {:<4} {:>6} {:>10} {:>16} {:>12} {:>7}",
            type_name(t),
            c.count,
            thousands(c.bytes),
            c.exact,
            c.skipped,
            c.failed
        )?;
    }
    writeln!(out, "\n  By change flag:")?;
    for (&t, c) in &by_type {
        for (&bit, &n) in &c.flags {
            writeln!(
                out,
                "  {:<4} {bit:#010x} {:<34} {n:>6}",
                type_name(t),
                change_flag_name(t, bit).unwrap_or("?")
            )?;
        }
    }
    for (cf, e) in &failures {
        writeln!(
            out,
            "  FAILED: {} refID {:06X} flags {:#010x}: {e}",
            type_name(cf.save_type),
            cf.ref_id.0,
            cf.flags
        )?;
    }
    Ok(failures.len())
}

fn quests(out: &mut impl Write, save: &Save<'_>, names: Option<&Names>) -> Result<(), CliError> {
    let mut decoded = Vec::new();
    for cf in save.change_forms.iter().filter(|c| c.save_type == 9) {
        if let Ok(q) = decode::quest(cf) {
            decoded.push((cf.ref_id, q));
        }
    }
    let flag = |q: &decode::Quest, f: u8| q.flags.is_some_and(|x| x & f != 0);
    let count = |f: u8| decoded.iter().filter(|(_, q)| flag(q, f)).count();
    let variables: usize = decoded
        .iter()
        .filter_map(|(_, q)| q.script.as_ref())
        .map(|s| s.variables.len())
        .sum();
    writeln!(
        out,
        "\nQuests: {} ({} running, {} completed, {} failed by their saved flags; {} script variables)",
        decoded.len(),
        count(quest_flag::ENABLED),
        count(quest_flag::COMPLETED),
        count(quest_flag::FAILED),
        variables
    )?;
    for (r, q) in &decoded {
        if q.stages.is_none() && q.objectives.is_none() {
            continue;
        }
        let state = [
            (quest_flag::ENABLED, "running"),
            (quest_flag::COMPLETED, "completed"),
            (quest_flag::FAILED, "failed"),
        ]
        .iter()
        .filter(|(f, _)| flag(q, *f))
        .map(|(_, s)| *s)
        .collect::<Vec<_>>()
        .join(", ");
        writeln!(
            out,
            "  {} flags {:#04x} {state}",
            describe(save, names, *r),
            q.flags.unwrap_or(0)
        )?;
        if let Some(stages) = &q.stages {
            let done: Vec<String> = stages
                .iter()
                .filter(|s| s.done)
                .map(|s| s.index.to_string())
                .collect();
            writeln!(
                out,
                "    current stage {}, stages done: {}",
                q.current_stage()
                    .map_or("none".to_string(), |s| s.to_string()),
                done.join(" ")
            )?;
        }
        if let Some(objectives) = &q.objectives {
            let list: Vec<String> = objectives
                .iter()
                .map(|o| format!("{} ({})", o.index, objective_state(o.state)))
                .collect();
            writeln!(out, "    objectives: {}", list.join(", "))?;
        }
    }
    Ok(())
}

fn objective_state(state: u32) -> &'static str {
    match state {
        0 => "dormant",
        1 => "displayed",
        2 => "completed",
        3 => "completed, displayed",
        _ => "?",
    }
}

fn globals(out: &mut impl Write, save: &Save<'_>, names: Option<&Names>) -> Result<(), CliError> {
    let Some(g) = save.global_data(3) else {
        return Ok(());
    };
    match decode::globals(g) {
        Ok(globals) => {
            writeln!(out, "\nGlobals ({}):", globals.len())?;
            for (r, v) in globals {
                writeln!(out, "  {} = {v}", describe(save, names, r))?;
            }
        }
        Err(e) => writeln!(out, "\nGlobals: FAILED: {e}")?,
    }
    Ok(())
}

fn misc_stats(out: &mut impl Write, save: &Save<'_>) -> Result<(), CliError> {
    let Some(g) = save.global_data(0) else {
        return Ok(());
    };
    match decode::misc_stats(g) {
        Ok(stats) => {
            writeln!(out, "\nMisc statistics ({}):", stats.len())?;
            for (i, v) in stats.iter().enumerate() {
                let name = world::stats::NAMES.get(i).copied().unwrap_or("?");
                writeln!(out, "  {i:>2} {name:<28} {v}")?;
            }
        }
        Err(e) => writeln!(out, "\nMisc statistics: FAILED: {e}")?,
    }
    Ok(())
}

/// The player's reference (`PlayerRef`).
const PLAYER_REF: u32 = 0x14;

fn player(out: &mut impl Write, save: &Save<'_>, names: Option<&Names>) -> Result<(), CliError> {
    writeln!(out, "\nPlayer:")?;
    if let Some(g) = save.global_data(1) {
        match decode::location(g) {
            Ok(l) => writeln!(
                out,
                "  location data: worldspace {}, grid {} {}, in {} at {:.1} {:.1} {:.1}; next created id {:08X}",
                describe(save, names, l.worldspace),
                l.grid[0],
                l.grid[1],
                describe(save, names, l.player_space),
                l.player_position[0],
                l.player_position[1],
                l.player_position[2],
                l.next_created_id
            )?,
            Err(e) => writeln!(out, "  location data: FAILED: {e}")?,
        }
    }
    let Some(cf) = save.change_form_of(PLAYER_REF) else {
        return Ok(());
    };
    match decode::reference_start(cf).map(|s| s.initial) {
        Ok(initial) => match initial.place() {
            Some((space, p, r)) => {
                let kind = match initial {
                    InitialData::Moved { .. } => "changed cell",
                    _ => "moved",
                };
                writeln!(
                    out,
                    "  PlayerRef ({kind}): in {} at {:.1} {:.1} {:.1}, rotation {:.3} {:.3} {:.3}",
                    describe(save, names, space),
                    p[0],
                    p[1],
                    p[2],
                    r[0],
                    r[1],
                    r[2]
                )?;
            }
            None => writeln!(out, "  PlayerRef: not moved")?,
        },
        Err(e) => writeln!(out, "  PlayerRef: FAILED: {e}")?,
    }
    if let Ok(form) = decode::actor_form(cf, true) {
        if let Some(p) = &form.player {
            writeln!(
                out,
                "  active quest {}",
                describe(save, names, p.active_quest)
            )?;
            let perks: Vec<String> = p
                .perks
                .iter()
                .map(|(perk, rank)| format!("{} rank {rank}", describe(save, names, *perk)))
                .collect();
            writeln!(out, "  perks ({}): {}", perks.len(), perks.join(", "))?;
            writeln!(out, "  notes {}, topics {}", p.notes.len(), p.topics.len())?;
        }
        let items = form
            .actor
            .mobile
            .data
            .inventory
            .as_ref()
            .map_or(0, Vec::len);
        writeln!(out, "  inventory changes: {items}")?;
    }
    if let Some(Ok(sky)) = save.global_data(8).map(decode::sky) {
        writeln!(
            out,
            "
Weather: {} (fading out {}), default {}, override {}, {:.0}% in",
            describe(save, names, sky.current),
            describe(save, names, sky.last),
            describe(save, names, sky.default),
            describe(save, names, sky.override_weather),
            sky.weather_pct * 100.0
        )?;
    }
    if let Some(Ok(radio)) = save.global_data(10).map(decode::radio) {
        writeln!(
            out,
            "Radio: {}, tuned to {}, {} stations discovered",
            if radio.on { "on" } else { "off" },
            describe(save, names, radio.active),
            radio.discovered.len()
        )?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use fos::save_type as t;
    use fos::write::{Form, PipeWriter, SaveWriter};

    fn sample(quest_extra: &[u8]) -> Vec<u8> {
        let mut quest = PipeWriter::new();
        quest.u8(0x01).vsval(2);
        quest.u8(10).u8(1).vsval(0);
        quest.u8(20).u8(0).vsval(0);
        quest.vsval(0).u8(0).u8(0);
        let mut quest = quest.finish();
        quest.extend_from_slice(quest_extra);
        let mut player = PipeWriter::new();
        let mut place = vec![0, 0, 3];
        for v in [10.0f32, 20.0, 30.0, 0.0, 0.0, 1.0] {
            place.extend_from_slice(&v.to_le_bytes());
        }
        player
            .bytes(&place)
            .player_head(&[])
            .u8(0xFF)
            .mobile_tail()
            .actor_fixed()
            .still_mover(true)
            .u8(0)
            .u8(0)
            .player_tail(&[(3, 1)], 1, [0; 8]);
        let mut globals = PipeWriter::new();
        globals.vsval(1).ref_id(4).f32(7.5);
        let mut stats = PipeWriter::new();
        stats.u32(2).u32(1).u32(4);
        let w = SaveWriter {
            global_data_1: vec![(0, stats.finish()), (3, globals.finish())],
            forms: vec![
                Form::new(1, 0x2 | 0x4000_0000 | 0x8000_0000, t::QUST, quest),
                // The player: moved, with one perk and an active quest.
                Form::new(2, 0x2, t::ACHR, player.finish()),
            ],
            form_ids: vec![0x0010_4C1C, 0x14, 0x3C, 0x38],
            ..SaveWriter::default()
        };
        w.build().0
    }

    fn report_of(bytes: &[u8]) -> (String, usize) {
        let save = Save::parse(bytes).unwrap();
        let mut out = Vec::new();
        let failures = report(&mut out, "test.fos", bytes.len(), &save, None)
            .unwrap_or_else(|_| panic!("report failed"));
        (String::from_utf8(out).unwrap(), failures)
    }

    #[test]
    fn a_save_is_reported_part_by_part() {
        let bytes = sample(&[]);
        let (text, failures) = report_of(&bytes);
        assert_eq!(failures, 0, "{text}");
        assert!(text.contains("Plugins (1):\n  00 FalloutNV.esm"), "{text}");
        assert!(text.contains("  QUST      1"), "{text}");
        assert!(text.contains("QUEST_STAGES"), "{text}");
        assert!(text.contains("00104C1C flags 0x01 running"), "{text}");
        assert!(text.contains("current stage 10, stages done: 10"), "{text}");
        assert!(text.contains("00000038 = 7.5"), "{text}");
        assert!(text.contains(" 1 Locations Discovered         4"), "{text}");
        assert!(
            text.contains("PlayerRef (moved): in 0000003C at 10.0 20.0 30.0"),
            "{text}"
        );
        assert!(text.contains("  ACHR      1"), "{text}");
        assert!(text.contains("perks (1): 0000003C rank 1"), "{text}");
        assert!(text.contains("active quest 00104C1C"), "{text}");
        let total = format!("{} of {}", thousands(bytes.len()), thousands(bytes.len()));
        assert!(text.contains(&total), "{text}");
    }

    #[test]
    fn a_change_form_that_doesnt_decode_to_its_length_is_counted() {
        let (text, failures) = report_of(&sample(&[0, b'|']));
        assert_eq!(failures, 1);
        assert!(text.contains("FAILED: QUST refID 000001"), "{text}");
    }

    #[test]
    fn counts_read_in_the_singular_and_plural() {
        assert_eq!(plural(1, "entry", "entries"), "1 entry");
        assert_eq!(plural(0, "entry", "entries"), "0 entries");
    }
}

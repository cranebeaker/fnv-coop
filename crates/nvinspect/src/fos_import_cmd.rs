//! `nvinspect fos-import <SAVE> <PLUGIN|DATA>`: what nv-rs takes from one
//! of the original game's saves (`world::fos_import`): the report, the
//! player's place, the quests' stages and the game time. Read-only.

use std::io::Write;
use std::path::Path;

use esm::{ActivePlugins, FormId, LoadOrder, Plugin};

use crate::{file_name_of, CliError};

pub fn run(out: &mut impl Write, rest: &[String]) -> Result<(), CliError> {
    crate::expect_args("fos-import", rest, 2, 2)?;
    let bytes = std::fs::read(&rest[0]).map_err(|e| CliError::Open {
        path: rest[0].clone(),
        message: e.to_string(),
    })?;
    let target = Path::new(&rest[1]);
    let order = if target.is_dir() {
        LoadOrder::from_data_dir(target, &ActivePlugins::OfficialOnly)?
    } else {
        let plugin = Plugin::open(target).map_err(|e| CliError::InFile {
            path: rest[1].clone(),
            message: e.to_string(),
        })?;
        LoadOrder::single(file_name_of(target), Some(target.to_path_buf()), plugin)?
    };
    let import = world::fos_import::import(&order, &bytes).map_err(|message| CliError::InFile {
        path: rest[0].clone(),
        message,
    })?;
    report(out, &order, &import)?;
    if !import.report.failures.is_empty() {
        return Err(CliError::InFile {
            path: rest[0].clone(),
            message: format!(
                "{} change forms didn't import",
                import.report.failures.len()
            ),
        });
    }
    Ok(())
}

fn name(order: &LoadOrder, id: FormId) -> String {
    match order.get(id).and_then(|r| r.editor_id().ok().flatten()) {
        Some(e) => format!("{id} {e}"),
        None => id.to_string(),
    }
}

pub(crate) fn report(
    out: &mut impl Write,
    order: &LoadOrder,
    import: &world::fos_import::Import,
) -> Result<(), CliError> {
    let r = &import.report;
    let s = &import.state;
    writeln!(out, "Imported:")?;
    for (what, n) in &r.counts {
        writeln!(out, "  {what:<32} {n}")?;
    }
    if !r.missing_plugins.is_empty() {
        writeln!(
            out,
            "  plugins not loaded: {}",
            r.missing_plugins.join(", ")
        )?;
    }
    writeln!(out, "  forms dropped (plugin not loaded): {}", r.dropped)?;
    for f in &r.failures {
        writeln!(out, "  FAILED: {f}")?;
    }
    match &import.place {
        Some(p) => writeln!(
            out,
            "\nPlayer: in {}{} at {:.1} {:.1} {:.1}, heading {:.3}",
            name(order, p.cell),
            p.world
                .map(|w| format!(" ({})", name(order, w)))
                .unwrap_or_default(),
            p.position[0],
            p.position[1],
            p.position[2],
            p.heading
        )?,
        None => writeln!(out, "\nPlayer: no place")?,
    }
    writeln!(
        out,
        "  name {}, level {}, active quest {}",
        s.player_name.as_deref().unwrap_or("-"),
        s.player_level,
        s.active_quest.map_or("-".to_string(), |q| name(order, q))
    )?;
    for g in [
        "GameYear",
        "GameMonth",
        "GameDay",
        "GameHour",
        "GameDaysPassed",
    ] {
        if let Some(v) = s.global(order, g) {
            writeln!(out, "  {g} = {v}")?;
        }
    }
    let mut stages: Vec<_> = s.stages.iter().collect();
    stages.sort();
    writeln!(out, "\nQuest stages ({}):", stages.len())?;
    for (q, stage) in stages {
        let state = if s.completed.contains(q) {
            "completed"
        } else if s.running.contains(q) {
            "running"
        } else {
            "stopped"
        };
        writeln!(out, "  {} stage {stage} ({state})", name(order, *q))?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use fos::save_type as t;
    use fos::write::{Form, PipeWriter, SaveWriter};

    /// A FalloutNV.esm holding one quest, `TestQuest` (0x1234).
    fn order() -> LoadOrder {
        use testdata::{group, record, sub, zstr};
        let mut header = 1.34f32.to_le_bytes().to_vec();
        header.extend([0; 8]);
        let mut bytes = record(b"TES4", 0, &sub(b"HEDR", &header));
        let quest = record(b"QUST", 0x1234, &sub(b"EDID", &zstr("TestQuest")));
        bytes.extend(group(*b"QUST", 0, &quest));
        let plugin = Plugin::from_bytes(bytes).unwrap();
        LoadOrder::from_plugins(vec![("FalloutNV.esm".into(), None, plugin)]).unwrap()
    }

    #[test]
    fn an_import_is_reported() {
        let order = order();
        let mut quest = PipeWriter::new();
        quest.u8(0x01).vsval(1).u8(10).u8(1).vsval(0);
        let w = SaveWriter {
            forms: vec![Form::new(1, 0x2 | 0x8000_0000, t::QUST, quest.finish())],
            form_ids: vec![0x0000_1234],
            ..SaveWriter::default()
        };
        let (bytes, _) = w.build();
        let import = world::fos_import::import(&order, &bytes).unwrap();
        let mut out = Vec::new();
        report(&mut out, &order, &import).unwrap_or_else(|_| panic!("report failed"));
        let text = String::from_utf8(out).unwrap();
        assert!(text.contains("quests"), "{text}");
        assert!(
            text.contains("00001234 TestQuest stage 10 (running)"),
            "{text}"
        );
    }
}

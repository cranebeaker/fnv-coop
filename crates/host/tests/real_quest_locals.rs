//! Opt-in: every quest script of the player's `FalloutNV.esm` made in the
//! game's memory ([`host::script_form::ScriptForm::load`]) and its
//! variables taken through the game's `ScriptLocals` and back.
//!
//! For each `SCPT` whose header marks a quest script, the world's
//! variables of it (parsed from its source, `SCTX`, as the world keeps a
//! quest's variables) are given distinct values by kind (a whole number, a
//! fraction, a form id), written into a new `ScriptLocals` (`005abf60`,
//! `ScriptLocals::SetVariable`), read back (`ScriptLocals::GetVariable`)
//! into fresh world variables and compared. Also checks that every compiled
//! variable (`SLSD`/`SCVR`) has a world variable of the same name, and that
//! the game keeps the references as their form ids in the low word.
//!
//! ```text
//! NV_DATA="C:\...\Fallout New Vegas\Data" cargo test -p host --release \
//!     --test real_quest_locals -- --ignored --nocapture
//! ```

use esm::{FourCC, Plugin};
use host::script_form::{read_locals, write_locals, ScriptForm};
use script::interp::Locals;
use script::VarKind;
use std::path::PathBuf;

#[test]
#[ignore = "needs the player's FalloutNV.exe and FalloutNV.esm (set NV_DATA)"]
fn quest_script_variables_go_through_the_games_locals_and_back() {
    let data = PathBuf::from(std::env::var("NV_DATA").expect("NV_DATA: the game's Data folder"));
    let mut host = host::Host::start(&data).unwrap_or_else(|e| panic!("{e}"));
    let plugin = Plugin::open(data.join("FalloutNV.esm")).expect("FalloutNV.esm");
    let e = &mut host.engine;
    let (mut scripts, mut variables, mut references) = (0, 0, 0);
    let mut unnamed = Vec::new();
    for entry in plugin.records_of_type(FourCC(*b"SCPT")) {
        let record = plugin.record(entry).expect("SCPT");
        // SCRIPT_HEADER::bIsQuestScript (Xbox PDB), SCHR +0x10.
        let Some(header) = record.get(FourCC(*b"SCHR")) else {
            continue;
        };
        if header.data.get(0x10) != Some(&1) {
            continue;
        }
        let name = record.editor_id().unwrap_or_default();
        let Some(source) = record.get(FourCC(*b"SCTX")) else {
            continue;
        };
        let source = esm::text::decode_cp1252(&source.data);
        let parsed = script::parse(&source).unwrap_or_else(|err| panic!("{name}: {err}"));
        let form = ScriptForm::load(e, record.header.form_id, &record, |id| id)
            .unwrap_or_else(|err| panic!("{name}: {err}"));
        let mut world = Locals::new(&parsed);
        let kinds: Vec<(String, VarKind)> =
            world.iter().map(|(n, k, _)| (n.to_string(), k)).collect();
        for (i, (n, kind)) in kinds.iter().enumerate() {
            let value = match kind {
                VarKind::Integer => i as f64 - 3.0,
                VarKind::Float => i as f64 + 0.25,
                VarKind::Ref => f64::from(0x0010_0000 + i as u32),
            };
            world.set(n, value);
        }
        for v in &form.variables {
            if world.get(&v.name).is_none() {
                unnamed.push(format!("{name}.{}", v.name));
            }
        }
        let locals = form.new_locals(e);
        write_locals(e, &form, locals, &world);
        for v in &form.variables {
            if let Some((_, VarKind::Ref, value)) = world
                .iter()
                .find(|(n, _, _)| n.eq_ignore_ascii_case(&v.name))
            {
                let raw = e.call(0x005a_9140, &[locals, v.id, form.form]).f64();
                assert_eq!(raw.to_bits(), value as u64, "{name}.{}", v.name);
                references += 1;
            }
        }
        let mut back = Locals::new(&parsed);
        read_locals(e, &form, locals, &mut back);
        // Only the compiled variables go through the game's locals.
        for v in &form.variables {
            assert_eq!(
                back.get(&v.name),
                world.get(&v.name),
                "{name}.{} (id {})",
                v.name,
                v.id
            );
        }
        scripts += 1;
        variables += form.variables.len();
    }
    println!(
        "quest scripts: {scripts}, variables through the game's locals: {variables} \
         ({references} references)"
    );
    assert!(scripts > 300, "{scripts} quest scripts");
    assert!(
        unnamed.is_empty(),
        "compiled variables the world's script doesn't name: {unnamed:?}"
    );
}

/// Every global of `FalloutNV.esm` made ([`host::global_form::GlobalForm::load`])
/// and its value read as the interpreter reads it (`00526ac0`), then
/// written and read again.
#[test]
#[ignore = "needs the player's FalloutNV.exe and FalloutNV.esm (set NV_DATA)"]
fn globals_hold_the_worlds_values_where_the_interpreter_reads_them() {
    use host::global_form::{read_global, write_global, GlobalForm};
    let data = PathBuf::from(std::env::var("NV_DATA").expect("NV_DATA: the game's Data folder"));
    let mut host = host::Host::start(&data).unwrap_or_else(|e| panic!("{e}"));
    let plugin = Plugin::open(data.join("FalloutNV.esm")).expect("FalloutNV.esm");
    let e = &mut host.engine;
    let mut globals = 0;
    for entry in plugin.records_of_type(FourCC(*b"GLOB")) {
        let record = plugin.record(entry).expect("GLOB");
        let name = record.editor_id().unwrap_or_default();
        let global = GlobalForm::load(e, record.header.form_id, &record).unwrap();
        let stored = record
            .get(FourCC(*b"FLTV"))
            .map(|s| f32::from_le_bytes(s.data[..4].try_into().unwrap()))
            .unwrap_or(0.0);
        assert_eq!(e.mem.u8(global.form + 4), 0x06, "{name}");
        assert_eq!(
            read_global(e, &global).to_bits(),
            stored.to_bits(),
            "{name}"
        );
        write_global(e, &global, stored + 1.5);
        assert_eq!(read_global(e, &global), stored + 1.5, "{name}");
        globals += 1;
    }
    println!("globals: {globals}");
    assert!(globals > 100, "{globals} globals");
}

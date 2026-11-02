//! Opt-in: the game's own interpreter on the player's real compiled quest
//! scripts. Every `SCPT` record of `FalloutNV.esm` whose header marks a
//! quest script runs once the way the quest entry `0060d4d0` runs it:
//! `Script::Run` with no reference and the quest's locals
//! (→ `ScriptRunManager::Run` → `ScriptRunner::Run` → `EvaluateLine` and
//! `CExpression::Eval`), so its `GameMode` block (and any
//! other block whose BEGIN handler accepts the frame) executes.
//!
//! The forms are generated, not loaded: the `Script` (type 0x11) comes
//! from the game's `CreateFormOfType` (`00465110`), with no owner quest
//! (the `TESQuest` constructor is not translated yet), and so does each
//! referenced form, with the type the exe's form-type table gives its record's signature; the player (0x14) is
//! built as `InitTES` builds it (`00938180`). Only the script's own record
//! data is copied in (SCHR, SCDA, SCRO/SCRV, SLSD/SCVR); referenced forms
//! keep their constructors' defaults with their form id written, and their
//! records are not loaded. This is a probe of the interpreter's coverage,
//! not of game behaviour.
//!
//! Needs the player's installation (nothing from it is stored or
//! committed; the report goes to the terminal):
//!
//! ```text
//! NV_DATA="C:\...\Fallout New Vegas\Data" cargo test -p engine --release \
//!     --test real_scda -- --ignored --nocapture
//! ```
//!
//! `NV_SCRIPT=<editor id>` runs one script; with `NV_SCRIPT_TRACE=<n>` it
//! also prints the last n calls of the run.

use engine::Engine;
use esm::{FormId, FourCC, Plugin, Record};
use std::cell::{Cell, RefCell};
use std::collections::{BTreeMap, HashMap};
use std::path::{Path, PathBuf};
use std::rc::Rc;

/// The exe's form-type table: 0x79 entries of 12 bytes, +4 the record
/// signature's text.
const FORM_TYPE_TABLE: u32 = 0x0118_7000;
const FORM_TYPE_COUNT: u32 = 0x79;
const CREATE_FORM_OF_TYPE: u32 = 0x0046_5110;
const SCRIPT_RUN: u32 = 0x005a_c1e0;
const EVALUATE_LINE: u32 = 0x005e_1550;
const LOG: u32 = 0x005b_5e40;
const PLAYER: u32 = 0x011d_ea3c;

/// How one script's run ended.
enum Outcome {
    /// Ran to the end without an error report.
    Completed,
    /// The interpreter reported an error (log format address).
    Logged(u32),
    /// Stopped in an untranslated function.
    Open(u32),
    /// Any other stop (unmapped memory, a divide error, ...).
    Other(String),
}

struct Run {
    name: String,
    outcome: Outcome,
    lines: u32,
    /// The opcode of the line being evaluated when it stopped.
    last_command: i32,
}

fn panic_text(payload: &(dyn std::any::Any + Send)) -> String {
    if let Some(s) = payload.downcast_ref::<String>() {
        s.clone()
    } else if let Some(s) = payload.downcast_ref::<&str>() {
        (*s).to_string()
    } else {
        "panic".to_string()
    }
}

fn classify(text: &str) -> Outcome {
    if let Some(rest) = text.strip_prefix("open function ") {
        if let Ok(address) = u32::from_str_radix(&rest[..8], 16) {
            return Outcome::Open(address);
        }
    }
    Outcome::Other(text.lines().next().unwrap_or("").to_string())
}

/// The allocator startup the Phase 2 probes run before any form exists.
fn start_allocator(e: &mut Engine) {
    let holder = e.call(0x0040_1000, &[8]).u32();
    let allocator = e.call(0x0040_1000, &[4]).u32();
    let allocator = e.call(0x00aa_2230, &[allocator]).u32();
    let holder = e.call(0x00aa_21d0, &[holder, allocator]).u32();
    e.mem.set_u32(0x011f_6118, holder);
    e.call(0x00aa_2020, &[]);
}

/// The form type of a record signature, from the exe's own table.
fn form_types(e: &Engine) -> HashMap<[u8; 4], u32> {
    let mut types = HashMap::new();
    for index in 0..FORM_TYPE_COUNT {
        let text = e.mem.u32(FORM_TYPE_TABLE + index * 12 + 4);
        let name = e.mem.cstr(text);
        if let Ok(signature) = <[u8; 4]>::try_from(name.as_slice()) {
            types.entry(signature).or_insert(index);
        }
    }
    types
}

/// A `BSSimpleList` of `items` starting with the node embedded at `head`.
fn fill_list(e: &mut Engine, head: u32, items: &[u32]) {
    let mut node = head;
    for (i, item) in items.iter().enumerate() {
        e.mem.set_u32(node, *item);
        if i + 1 < items.len() {
            let next = e.mem.alloc(8);
            e.mem.set_u32(node + 4, next);
            node = next;
        }
    }
}

struct Fixture<'a> {
    plugin: &'a Plugin,
    types: HashMap<[u8; 4], u32>,
    forms: HashMap<u32, u32>,
    gaps: &'a RefCell<BTreeMap<String, usize>>,
}

impl Fixture<'_> {
    /// The generated form for a referenced form id (0 when its record is
    /// not in the plugin or the exe has no type for its signature).
    fn form(&mut self, e: &mut Engine, id: u32) -> u32 {
        if let Some(form) = self.forms.get(&id) {
            return *form;
        }
        let form = if id == 0x14 {
            let made = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                let block = e.call(0x0040_1000, &[0xe50]).u32();
                e.call(0x0093_8180, &[block]).u32()
            }));
            match made {
                Ok(player) => {
                    e.mem.set_u32(PLAYER, player);
                    player
                }
                Err(payload) => {
                    let text = panic_text(payload.as_ref());
                    let key = format!("player: {}", text.lines().next().unwrap_or(""));
                    *self.gaps.borrow_mut().entry(key).or_default() += 1;
                    0
                }
            }
        } else {
            let signature = match self.plugin.get(FormId(id)) {
                Some(entry) => entry.header.kind.0,
                None => *b"----",
            };
            match self.types.get(&signature).copied() {
                Some(kind) => {
                    let made = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                        e.call(CREATE_FORM_OF_TYPE, &[kind]).u32()
                    }));
                    match made {
                        Ok(form) => form,
                        Err(payload) => {
                            let text = panic_text(payload.as_ref());
                            let key = format!(
                                "{}: {}",
                                String::from_utf8_lossy(&signature),
                                text.lines().next().unwrap_or("")
                            );
                            *self.gaps.borrow_mut().entry(key).or_default() += 1;
                            0
                        }
                    }
                }
                None => {
                    let key = format!("{}: not generated", String::from_utf8_lossy(&signature));
                    *self.gaps.borrow_mut().entry(key).or_default() += 1;
                    0
                }
            }
        };
        if form != 0 {
            // TESForm::iFormID (Xbox PDB) +0xC.
            e.mem.set_u32(form + 0xc, id);
        }
        self.forms.insert(id, form);
        form
    }
}

fn subrecord<'r>(record: &'r Record, kind: &[u8; 4]) -> Option<&'r [u8]> {
    record.get(FourCC(*kind)).map(|s| s.data.as_slice())
}

/// Builds the script, its locals and the referenced forms, then runs the
/// script once.
fn run_one(
    exe: &Path,
    plugin: &Plugin,
    record: &Record,
    gaps: &RefCell<BTreeMap<String, usize>>,
    lines: &Rc<Cell<u32>>,
    last_command: &Rc<Cell<i32>>,
    logged: &Rc<RefCell<Vec<u32>>>,
) -> Outcome {
    let mut e = Engine::with_exe(exe).expect("checked installation");
    start_allocator(&mut e);
    let types = form_types(&e);
    let mut fixture = Fixture {
        plugin,
        types,
        forms: HashMap::new(),
        gaps,
    };
    let (Some(header), Some(code)) = (subrecord(record, b"SCHR"), subrecord(record, b"SCDA"))
    else {
        return Outcome::Other("record without SCHR/SCDA".into());
    };
    let script = e.call(CREATE_FORM_OF_TYPE, &[0x11]).u32();
    e.mem.set_u32(script + 0xc, record.header.form_id.0);
    // Script (Xbox PDB): m_header +0x18, m_data +0x30, pOwnerQuest +0x40,
    // listRefObjects +0x44, listVariables +0x4c.
    e.mem
        .write(script + 0x18, &header[..header.len().min(0x14)]);
    let data = e.mem.alloc(code.len() as u32);
    e.mem.write(data, code);
    e.mem.set_u32(script + 0x30, data);
    let mut objects = Vec::new();
    let mut variables = Vec::new();
    let mut pending_local: Option<u32> = None;
    for sub in &record.subrecords {
        match &sub.kind.0 {
            b"SCRO" | b"SCRV" => {
                let word = u32::from_le_bytes(sub.data[..4].try_into().unwrap());
                // SCRIPT_REFERENCED_OBJECT (Xbox PDB): form +8, variable +0xc.
                let object = e.mem.alloc(0x10);
                if &sub.kind.0 == b"SCRO" {
                    let form = fixture.form(&mut e, word);
                    e.mem.set_u32(object + 8, form);
                } else {
                    e.mem.set_u32(object + 0xc, word);
                }
                objects.push(object);
            }
            b"SLSD" => {
                // ScriptVariable: SCRIPT_LOCAL (0x18) then the name string.
                let variable = e.mem.alloc(0x20);
                e.mem.write(variable, &sub.data[..sub.data.len().min(0x18)]);
                variables.push(variable);
                pending_local = Some(variable);
            }
            b"SCVR" => {
                if let Some(variable) = pending_local.take() {
                    let text = e.mem.alloc(sub.data.len() as u32 + 1);
                    e.mem.write(text, &sub.data);
                    let length = sub
                        .data
                        .iter()
                        .position(|b| *b == 0)
                        .unwrap_or(sub.data.len());
                    e.mem.set_u32(variable + 0x18, text);
                    e.mem.set_u16(variable + 0x1c, length as u16);
                    e.mem.set_u16(variable + 0x1e, length as u16 + 1);
                }
            }
            _ => {}
        }
    }
    fill_list(&mut e, script + 0x44, &objects);
    fill_list(&mut e, script + 0x4c, &variables);
    // The quest's locals, as the quest keeps them (TESQuest +0x5c).
    let locals = e.call(0x005a_bf60, &[script]).u32();

    let (lines, last_command, logged) = (lines.clone(), last_command.clone(), logged.clone());
    e.register_double(EVALUATE_LINE, move |e, a| {
        if a[2] != 0x10 {
            lines.set(lines.get() + 1);
        }
        last_command.set(a[2] as i32);
        let ok = engine::units::fallout_shared::tesscriptrunmanager::runner_evaluate_line(
            e,
            engine::Ptr::new(a[0]),
            engine::Ptr::new(a[1]),
            a[2] as i32,
            engine::Ptr::new(a[3]),
            engine::Ptr::new(a[4]),
            a[5],
            engine::Ptr::new(a[6]),
            a[7],
            a[8] as u8 != 0,
            a[9] as u8 != 0,
        );
        engine::Ret {
            eax: u32::from(ok),
            ..engine::Ret::default()
        }
    });
    e.register_double(LOG, move |_, a| {
        logged.borrow_mut().push(a.first().copied().unwrap_or(0));
        engine::Ret::default()
    });
    // What 0060d4d0 does with the quest's script and locals.
    // SCRIPT_HEADER::dataSize (Xbox PDB), cleared by a failed parse.
    let size = e.mem.u32(script + 0x20);
    let trace_lines: Option<usize> = std::env::var("NV_SCRIPT_TRACE")
        .ok()
        .map(|v| v.parse().unwrap_or(40));
    let trace = trace_lines.is_some();
    if trace {
        e.call_log = Some(Vec::new());
    }
    e.call(SCRIPT_RUN, &[script, 0, locals, 0, 0]);
    if trace {
        for (address, words) in e
            .call_log
            .take()
            .unwrap_or_default()
            .iter()
            .rev()
            .take(trace_lines.unwrap_or(40))
            .rev()
        {
            eprintln!("  {address:08x} {words:08x?}");
        }
    }
    if e.mem.u32(script + 0x20) != size {
        return Outcome::Logged(0);
    }
    Outcome::Completed
}

#[test]
#[ignore = "needs the player's FalloutNV.exe and FalloutNV.esm (set NV_DATA)"]
fn quest_scripts_run_through_the_game_interpreter() {
    let data = PathBuf::from(std::env::var("NV_DATA").expect("NV_DATA: the game's Data folder"));
    let exe = engine::exe::installed_exe(&data);
    engine::exe::verify(&exe).unwrap_or_else(|e| panic!("{e}"));
    let plugin = Plugin::open(data.join("FalloutNV.esm")).expect("FalloutNV.esm");

    // Referenced forms whose generation stopped, by signature and stop.
    let gaps = RefCell::new(BTreeMap::new());

    std::panic::set_hook(Box::new(|_| {}));
    let mut runs = Vec::new();
    for entry in plugin.records_of_type(FourCC(*b"SCPT")) {
        let record = plugin.record(entry).expect("SCPT");
        // SCRIPT_HEADER::bIsQuestScript (Xbox PDB), SCHR +0x10.
        let Some(header) = subrecord(&record, b"SCHR") else {
            continue;
        };
        if header.get(0x10) != Some(&1) {
            continue;
        }
        let name = record.editor_id().unwrap_or_default();
        if let Ok(only) = std::env::var("NV_SCRIPT") {
            if name != only {
                continue;
            }
        }
        let id = record.header.form_id.0;
        let lines = Rc::new(Cell::new(0));
        let last_command = Rc::new(Cell::new(0));
        let logged = Rc::new(RefCell::new(Vec::new()));
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            run_one(
                &exe,
                &plugin,
                &record,
                &gaps,
                &lines,
                &last_command,
                &logged,
            )
        }));
        let mut outcome = match result {
            Ok(outcome) => outcome,
            Err(payload) => classify(&panic_text(payload.as_ref())),
        };
        if let (Outcome::Completed | Outcome::Logged(0), Some(format)) =
            (&outcome, logged.borrow().first())
        {
            outcome = Outcome::Logged(*format);
        }
        runs.push(Run {
            name: format!("{name} ({id:08X})"),
            outcome,
            lines: lines.get(),
            last_command: last_command.get(),
        });
    }
    let _ = std::panic::take_hook();

    let completed = runs
        .iter()
        .filter(|r| matches!(r.outcome, Outcome::Completed))
        .count();
    let mut open: BTreeMap<u32, Vec<&Run>> = BTreeMap::new();
    let mut logged: BTreeMap<u32, Vec<&Run>> = BTreeMap::new();
    let mut other: BTreeMap<String, Vec<&Run>> = BTreeMap::new();
    for run in &runs {
        match &run.outcome {
            Outcome::Completed => {}
            Outcome::Open(address) => open.entry(*address).or_default().push(run),
            Outcome::Logged(format) => logged.entry(*format).or_default().push(run),
            Outcome::Other(text) => other.entry(text.clone()).or_default().push(run),
        }
    }
    let executed = runs
        .iter()
        .filter(|r| matches!(r.outcome, Outcome::Completed) && r.lines > 0)
        .count();
    println!(
        "quest scripts: {}, ran to completion: {completed} ({executed} executed block lines), block lines evaluated: {}",
        runs.len(),
        runs.iter().map(|r| r.lines).sum::<u32>()
    );
    let mut by_count: Vec<_> = open.iter().collect();
    by_count.sort_by_key(|(_, list)| std::cmp::Reverse(list.len()));
    println!(
        "stopped in an untranslated function ({} addresses):",
        open.len()
    );
    for (address, list) in by_count {
        let mut commands: Vec<String> = list
            .iter()
            .map(|r| format!("{:04x}", r.last_command))
            .collect();
        commands.sort();
        commands.dedup();
        println!(
            "  {address:08x}: {} scripts (line opcodes {}), e.g. {}",
            list.len(),
            commands.join(" "),
            list[0].name
        );
    }
    println!(
        "reported an error (log format address; 00000000: data size cleared, nothing logged):"
    );
    for (format, list) in &logged {
        println!(
            "  {format:08x}: {} scripts, e.g. {}",
            list.len(),
            list[0].name
        );
    }
    println!("other stops:");
    for (text, list) in &other {
        println!("  {text}: {} scripts, e.g. {}", list.len(), list[0].name);
    }
    println!("referenced forms not generated (signature: stop, references):");
    for (key, count) in gaps.borrow().iter() {
        println!("  {key}: {count}");
    }
    assert!(!runs.is_empty(), "no quest scripts found");
}

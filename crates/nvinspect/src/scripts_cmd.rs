//! `scripts`: parses every script source in the load order (script
//! records, dialogue lines' result scripts, quest stages' result scripts)
//! and reports how many parse and why the rest don't.

use std::collections::BTreeMap;
use std::io::Write;

use esm::{FourCC, LoadOrder};

use crate::CliError;

const SCTX: FourCC = FourCC::new(b"SCTX");
const SCDA: FourCC = FourCC::new(b"SCDA");

pub fn scripts(out: &mut impl Write, order: &LoadOrder, show: usize) -> Result<(), CliError> {
    let mut parsed = 0usize;
    let mut failed: Vec<(String, String)> = Vec::new();
    let mut reasons: BTreeMap<String, usize> = BTreeMap::new();
    let mut total = 0usize;
    // Expressions compared with the compiled form: how many have the
    // same shape, and the first that don't.
    let mut same = 0usize;
    let mut different: Vec<String> = Vec::new();
    // Whole scripts compared statement by statement, calls with their
    // argument counts.
    let mut outlines_same = 0usize;
    let mut outlines_different: Vec<String> = Vec::new();
    for kind in [
        FourCC::new(b"SCPT"),
        FourCC::new(b"INFO"),
        FourCC::new(b"QUST"),
    ] {
        for rr in order.records_of_type(kind) {
            let Ok(record) = rr.record() else { continue };
            for (source, compiled) in source_pairs(&record) {
                let source = esm::text::decode_cp1252(source);
                if let Ok(parsed) = script::parse(&source) {
                    let mine = parsed_outline(&parsed);
                    let theirs = compiled_outline(compiled);
                    if mine == theirs {
                        outlines_same += 1;
                    } else {
                        // The first statement that differs.
                        let at = mine
                            .iter()
                            .zip(&theirs)
                            .position(|(a, b)| a != b)
                            .unwrap_or(mine.len().min(theirs.len()));
                        outlines_different.push(format!(
                            "{} {} statement {at}: parsed {:?}, compiled {:?}",
                            rr.form_id,
                            record.editor_id().unwrap_or_default(),
                            mine.get(at),
                            theirs.get(at)
                        ));
                    }
                }
                let Some(pairs) = written_and_stored(&source, compiled) else {
                    continue;
                };
                for (line, text, s) in pairs {
                    let mine = script::parse_expression(&text).map(|e| e.shape());
                    let theirs =
                        stored_expression(&s).map(|t| script::compiled::expression_shape(&t));
                    match (mine, theirs) {
                        (Ok(a), Some(b)) if a == b => same += 1,
                        (a, b) => different.push(format!(
                            "{line}\n      parsed:   {}\n      compiled: {}",
                            a.unwrap_or_else(|e| e),
                            b.unwrap_or_default()
                        )),
                    }
                }
            }
            for (k, sub) in record.get_all(SCTX).enumerate() {
                total += 1;
                let source = esm::text::decode_cp1252(&sub.data);
                match script::parse(&source) {
                    Ok(_) => parsed += 1,
                    Err(e) => {
                        let label = format!(
                            "{kind} {} {}#{k}",
                            rr.form_id,
                            record.editor_id().unwrap_or_default()
                        );
                        // Group reasons by their wording without the
                        // particulars.
                        let reason = e.message.split(':').next().unwrap_or("").to_string();
                        *reasons.entry(reason).or_default() += 1;
                        failed.push((label, e.to_string()));
                    }
                }
            }
        }
    }
    writeln!(
        out,
        "{parsed} of {total} script sources parse ({} don't)",
        failed.len()
    )?;
    let mut by_count: Vec<(String, usize)> = reasons.into_iter().collect();
    by_count.sort_by_key(|r| std::cmp::Reverse(r.1));
    for (reason, n) in by_count.iter().take(15) {
        writeln!(out, "  {n:>5}  {reason}")?;
    }
    for (label, e) in failed.iter().take(show) {
        writeln!(out, "  {label}: {e}")?;
    }
    writeln!(
        out,
        "{same} of {} expressions are put in the same order as the game's compiled form",
        same + different.len()
    )?;
    for d in different.iter().take(show) {
        writeln!(out, "  {d}")?;
    }
    writeln!(
        out,
        "{outlines_same} of {} scripts have the same statements and argument counts as the compiled form",
        outlines_same + outlines_different.len()
    )?;
    for d in outlines_different.iter().take(show) {
        writeln!(out, "  {d}")?;
    }
    Ok(())
}

/// Each script source with its compiled form: the `SCDA` before it in the
/// same script (a script's header `SCHR` starts each one; a quest stage
/// whose script is only a comment has a source and no compiled form).
fn source_pairs(record: &esm::Record) -> Vec<(&[u8], &[u8])> {
    const SCHR: FourCC = FourCC::new(b"SCHR");
    let mut out = Vec::new();
    let mut compiled: Option<&[u8]> = None;
    for sub in &record.subrecords {
        if sub.kind == SCHR {
            compiled = None;
        } else if sub.kind == SCDA {
            compiled = Some(&sub.data);
        } else if sub.kind == SCTX {
            if let Some(c) = compiled.take() {
                out.push((&sub.data[..], c));
            }
        }
    }
    out
}

/// A parsed script as a list of statements like the compiled form's:
/// `If GetStage/1`, `SetStage/2`, `EndIf`, … (calls with their argument
/// counts).
fn parsed_outline(script: &script::Script) -> Vec<String> {
    fn calls(e: &script::Expr) -> String {
        e.0.iter()
            .filter_map(|item| match item {
                script::Item::Call(c) => Some(format!(
                    " {}/{}",
                    script::function_name(c.function),
                    c.args.len()
                )),
                _ => None,
            })
            .collect()
    }
    fn walk(stmts: &[script::Stmt], out: &mut Vec<String>) {
        for s in stmts {
            match s {
                script::Stmt::Set { value, .. } => out.push(format!("Set{}", calls(value))),
                script::Stmt::Return => out.push("Return".into()),
                script::Stmt::Call(c) => out.push(format!(
                    "{}/{}",
                    script::function_name(c.function),
                    c.args.len()
                )),
                script::Stmt::If {
                    branches,
                    otherwise,
                } => {
                    for (k, (cond, body)) in branches.iter().enumerate() {
                        let word = if k == 0 { "If" } else { "ElseIf" };
                        out.push(format!("{word}{}", calls(cond)));
                        walk(body, out);
                    }
                    if let Some(body) = otherwise {
                        out.push("Else".into());
                        walk(body, out);
                    }
                    out.push("EndIf".into());
                }
            }
        }
    }
    let mut out = Vec::new();
    if script.name.is_some() {
        out.push("ScriptName".into());
    }
    for block in &script.blocks {
        out.push("Begin".into());
        walk(&block.body, &mut out);
        out.push("End".into());
    }
    walk(&script.body, &mut out);
    out
}

/// The compiled form as the same kind of list.
fn compiled_outline(compiled: &[u8]) -> Vec<String> {
    use script::compiled::{code_name, Token};
    // A call's argument count: the first two parameter bytes.
    let count = |params: &[u8]| u16_in(params, 0).unwrap_or(0);
    script::compiled::statements(compiled)
        .iter()
        .map(|s| {
            let name = code_name(s.code);
            if s.code >= 0x1000 {
                return format!("{name}/{}", count(s.data));
            }
            let calls: String = stored_expression(s)
                .unwrap_or_default()
                .iter()
                .filter_map(|t| match t {
                    Token::Call(code, params) => {
                        Some(format!(" {}/{}", code_name(*code), count(params)))
                    }
                    _ => None,
                })
                .collect();
            format!("{name}{calls}")
        })
        .collect()
}

fn u16_in(data: &[u8], i: usize) -> Option<usize> {
    data.get(i..i + 2)
        .map(|b| usize::from(u16::from_le_bytes([b[0], b[1]])))
}

/// Where a `set`'s value starts: after its target, a local (3 bytes) or a
/// reference's variable (6).
fn set_target_len(data: &[u8]) -> usize {
    if data.first() == Some(&b'r') {
        6
    } else {
        3
    }
}

/// The expression of an `if`, `elseif` or `set` (its value).
fn stored_expression<'a>(
    s: &script::compiled::Statement<'a>,
) -> Option<Vec<script::compiled::Token<'a>>> {
    let start = match s.code {
        0x16 | 0x18 => 2,
        0x15 => set_target_len(s.data),
        _ => return None,
    };
    let len = u16_in(s.data, start)?;
    script::compiled::expression(s.data.get(start + 2..start + 2 + len)?)
}

/// An `if`'s, `elseif`'s or `set`'s expression as text, in the order the
/// game evaluates it (`set` with its target first, then `=`).
fn compiled_expression(s: &script::compiled::Statement) -> Option<String> {
    use script::compiled::{expression, expression_text};
    match s.code {
        0x16 | 0x18 => Some(format!(
            "(skip {}) {}",
            u16_in(s.data, 0)?,
            expression_text(&stored_expression(s)?)
        )),
        0x15 => {
            let target_bytes = [b" ", s.data.get(..set_target_len(s.data))?].concat();
            Some(format!(
                "{} = {}",
                expression_text(&expression(&target_bytes)?),
                expression_text(&stored_expression(s)?)
            ))
        }
        0x17 => Some(format!("(skip {})", u16_in(s.data, 0)?)),
        _ => None,
    }
}

/// The `if` / `elseif` / `set` lines of a source, each with its expression
/// text, paired with the compiled statements, when the counts agree.
fn written_and_stored<'a>(
    source: &str,
    compiled: &'a [u8],
) -> Option<Vec<(String, String, script::compiled::Statement<'a>)>> {
    let mut written = Vec::new();
    for line in source.lines() {
        let line = line.split(';').next().unwrap_or("").trim();
        // The first word, which may run straight into a bracket (`if(`).
        let first: String = line
            .chars()
            .take_while(|c| c.is_ascii_alphanumeric())
            .collect::<String>()
            .to_ascii_lowercase();
        let expression = match first.as_str() {
            "if" | "elseif" => line[first.len()..].to_string(),
            "set" => {
                // After the first ` to `.
                let lower = line.to_ascii_lowercase();
                let Some(at) = lower.find(" to ") else {
                    continue;
                };
                line[at + 4..].to_string()
            }
            _ => continue,
        };
        written.push((line.to_string(), expression));
    }
    let stored: Vec<_> = script::compiled::statements(compiled)
        .into_iter()
        .filter(|s| matches!(s.code, 0x15 | 0x16 | 0x18))
        .collect();
    if written.len() != stored.len() {
        return None;
    }
    Some(
        written
            .into_iter()
            .zip(stored)
            .map(|((line, e), s)| (line, e, s))
            .collect(),
    )
}

/// `conditions`: every `if` / `elseif` / `set` whose source contains `text`, as
/// written and as the game's compiler stored it (operands before their
/// operators), to see how it groups them.
pub fn conditions(
    out: &mut impl Write,
    order: &LoadOrder,
    text: &str,
    show: usize,
) -> Result<(), CliError> {
    let needle = text.to_ascii_lowercase();
    let mut shown = 0;
    for kind in [
        FourCC::new(b"SCPT"),
        FourCC::new(b"INFO"),
        FourCC::new(b"QUST"),
    ] {
        for rr in order.records_of_type(kind) {
            let Ok(record) = rr.record() else { continue };
            for sub in record.get_all(SCTX) {
                let source = esm::text::decode_cp1252(&sub.data);
                if !source.to_ascii_lowercase().contains(&needle) {
                    continue;
                }
                // The compiled form of this source's if / elseif / set
                // lines, when it pairs up.
                let compiled = source_pairs(&record)
                    .into_iter()
                    .find(|(s, _)| *s == &sub.data[..])
                    .and_then(|(_, c)| written_and_stored(&source, c))
                    .unwrap_or_default();
                for line in source.lines() {
                    let line = line.split(';').next().unwrap_or("").trim();
                    if !line.to_ascii_lowercase().contains(&needle) {
                        continue;
                    }
                    writeln!(
                        out,
                        "{} {}: {line}",
                        rr.form_id,
                        record.editor_id().unwrap_or_default()
                    )?;
                    if let Some((_, _, s)) = compiled.iter().find(|(w, _, _)| w == line) {
                        writeln!(out, "    => {}", compiled_expression(s).unwrap_or_default())?;
                    }
                    shown += 1;
                    if shown >= show {
                        return Ok(());
                    }
                }
            }
        }
    }
    Ok(())
}

/// `source`: a record's script sources with line numbers, each followed by
/// whether it parses.
pub fn source(out: &mut impl Write, record: &esm::Record) -> Result<(), CliError> {
    let mut any = false;
    for (k, sub) in record.get_all(SCTX).enumerate() {
        any = true;
        let text = esm::text::decode_cp1252(&sub.data);
        writeln!(out, "--- source {k}")?;
        for (n, line) in text.lines().enumerate() {
            writeln!(out, "{:>4}  {line}", n + 1)?;
        }
        match script::parse(&text) {
            Ok(s) => writeln!(
                out,
                "--- parses: {} variables, {} blocks, {} loose statements",
                s.variables.len(),
                s.blocks.len(),
                s.body.len()
            )?,
            Err(e) => writeln!(out, "--- doesn't parse: {e}")?,
        }
    }
    if !any {
        writeln!(out, "(no script source in this record)")?;
    }
    // The compiled form the game runs, statement by statement.
    for (k, sub) in record.get_all(SCDA).enumerate() {
        writeln!(out, "--- compiled {k} ({} bytes)", sub.data.len())?;
        let mut depth = 0usize;
        for s in script::compiled::statements(&sub.data) {
            if matches!(s.code, 0x11 | 0x17 | 0x18 | 0x19) {
                depth = depth.saturating_sub(1);
            }
            let on = s.on.map(|r| format!("ref {r}.")).unwrap_or_default();
            let detail = match compiled_expression(&s) {
                Some(text) => text,
                None => {
                    let hex: Vec<String> =
                        s.data.iter().take(24).map(|b| format!("{b:02X}")).collect();
                    format!(
                        "{}{}",
                        hex.join(" "),
                        if s.data.len() > 24 { " …" } else { "" }
                    )
                }
            };
            writeln!(
                out,
                "{:>6}  {}{on}{}  {detail}",
                s.offset,
                "  ".repeat(depth),
                script::compiled::code_name(s.code),
            )?;
            if matches!(s.code, 0x10 | 0x16 | 0x17 | 0x18) {
                depth += 1;
            }
        }
    }
    Ok(())
}

/// `functions`: every script function the game's scripts call and its
/// conditions (`CTDA`) ask, counted, and which of them the engine doesn't
/// carry out yet (`world::scripting::HANDLED`), most used first.
pub fn functions(out: &mut impl Write, order: &LoadOrder, show: usize) -> Result<(), CliError> {
    // Per function number: calls in scripts, scripts calling it, conditions,
    // and the first record using it.
    #[derive(Default)]
    struct Use {
        calls: usize,
        scripts: usize,
        conditions: usize,
        first: Option<String>,
    }
    fn walk(stmts: &[script::Stmt], seen: &mut BTreeMap<u16, usize>) {
        let expr = |e: &script::Expr, seen: &mut BTreeMap<u16, usize>| {
            for item in &e.0 {
                if let script::Item::Call(c) = item {
                    *seen.entry(c.function).or_default() += 1;
                }
            }
        };
        for s in stmts {
            match s {
                script::Stmt::Set { value, .. } => expr(value, seen),
                script::Stmt::Call(c) => *seen.entry(c.function).or_default() += 1,
                script::Stmt::If {
                    branches,
                    otherwise,
                } => {
                    for (cond, body) in branches {
                        expr(cond, seen);
                        walk(body, seen);
                    }
                    if let Some(body) = otherwise {
                        walk(body, seen);
                    }
                }
                script::Stmt::Return => {}
            }
        }
    }
    const CTDA: FourCC = FourCC::new(b"CTDA");
    let mut uses: BTreeMap<u16, Use> = BTreeMap::new();
    let mut sources = 0usize;
    let mut conditions = 0usize;
    for kind in [
        b"SCPT", b"INFO", b"QUST", b"PACK", b"IDLE", b"PERK", b"MESG", b"TERM", b"CHAL", b"RCPE",
        b"SPEL", b"ENCH", b"ALCH", b"INGR", b"LSCR", b"CPTH", b"AMEF", b"IMOD", b"NOTE",
    ] {
        for rr in order.records_of_type(FourCC::new(kind)) {
            let Ok(record) = rr.record() else { continue };
            let label = || {
                format!(
                    "{} {} {}",
                    rr.entry.header.kind,
                    rr.form_id,
                    record.editor_id().unwrap_or_default()
                )
            };
            for sub in record.get_all(SCTX) {
                let source = esm::text::decode_cp1252(&sub.data);
                let Ok(parsed) = script::parse(&source) else {
                    continue;
                };
                sources += 1;
                let mut seen = BTreeMap::new();
                for block in &parsed.blocks {
                    walk(&block.body, &mut seen);
                }
                walk(&parsed.body, &mut seen);
                for (f, n) in seen {
                    let u = uses.entry(f).or_default();
                    u.calls += n;
                    u.scripts += 1;
                    u.first.get_or_insert_with(label);
                }
            }
            for sub in record.get_all(CTDA) {
                let Some(f) = sub.data.get(8..10) else {
                    continue;
                };
                conditions += 1;
                let u = uses.entry(u16::from_le_bytes([f[0], f[1]])).or_default();
                u.conditions += 1;
                u.first.get_or_insert_with(label);
            }
        }
    }
    let handled = |f: u16| {
        let name = script::function_name(f);
        world::scripting::HANDLED
            .iter()
            .copied()
            .chain(world::script_functions::handled())
            .any(|h| h.eq_ignore_ascii_case(&name))
    };
    let mut missing: Vec<(&u16, &Use)> = uses.iter().filter(|(f, _)| !handled(**f)).collect();
    missing.sort_by_key(|(_, u)| std::cmp::Reverse(u.calls + u.conditions));
    let used_handled = uses.keys().filter(|f| handled(**f)).count();
    writeln!(
        out,
        "{} functions used by {sources} script sources and {conditions} conditions; {used_handled} carried out, {} not:",
        uses.len(),
        missing.len()
    )?;
    writeln!(
        out,
        "  {:<34} {:>6} {:>8} {:>10}  first used in",
        "function", "calls", "scripts", "conditions"
    )?;
    for (f, u) in missing.iter().take(show) {
        writeln!(
            out,
            "  {:<34} {:>6} {:>8} {:>10}  {}",
            script::function_name(**f),
            u.calls,
            u.scripts,
            u.conditions,
            u.first.as_deref().unwrap_or("")
        )?;
    }
    Ok(())
}

//! The executable's `atexit` destructors: the small compiler-generated
//! functions (`00fc7a70` onwards, the engine map's "compiler: atexit
//! destructors") that the static initializers hand to `_atexit` (`00ec658f`)
//! and that the exit sequence ([`crate::units::crt::doexit`]) runs.
//!
//! Almost all are one of two exact shapes, each a row of the generated table
//! `data/atexit_destructors.tsv` (research/initgen `--atexit`, from the
//! function's code, every instruction checked; anything else is refused there
//! and left for hand translation). A row is `entry`, `shape`, and the
//! parameters `global destructor` (addresses):
//!
//! - `call`, 15 bytes: `PUSH EBP; MOV EBP,ESP; MOV ECX,global; CALL
//!   destructor; POP EBP; RET` ([`run_call`]).
//! - `tail`, 10 bytes: `MOV ECX,global; JMP destructor` ([`run_tail`]).
//!
//! The two shapes have the same effect, so their translations are the same
//! three steps; they stay separate rows because the code differs and each
//! translation is checked against its own code. Why each is faithful:
//!
//! - `ECX` is the global: `MOV ECX,global` is the only write to a register
//!   the destructor reads, and the destructor is `__thiscall`, so the
//!   uniform form passes the global as `this`.
//! - The destructor takes nothing from the stack and pops nothing: initgen
//!   checks that every `RET` of each destructor (following direct tail
//!   jumps) is a plain `RET`, so nothing the shape leaves on the stack is
//!   read and nothing the callee pops is missing.
//! - The frame of `call` (`PUSH EBP; MOV EBP,ESP` .. `POP EBP`) restores
//!   `EBP` and leaves `ESP` as it was; the `CALL` returns to a plain `POP
//!   EBP; RET`, which touch neither `EAX` nor `EDX` nor `ST0`; `tail` jumps
//!   so that the destructor's `RET` is the shape's `RET`. In both, what the
//!   caller sees in `EAX` (and `EDX`, `ST0`) is exactly what the destructor
//!   left, which is what [`run_call`] and [`run_tail`] return.
//!
//! Each row is callable at its own address like a translated function:
//! `e.call(addr, ..)` and [`crate::Engine::is_translated`] consult the table
//! when no function is registered at `addr` (a hand translation, or a test
//! double, registered there wins). A row whose destructor is not translated
//! is not translated: [`runs`] says so, and the exit sequence skips and
//! counts it; calling it directly stops in the destructor with the engine's
//! `open function` message.

use crate::{Engine, Ret};
use std::collections::HashMap;
use std::sync::OnceLock;

/// The generated table (research/initgen `--atexit`).
const TABLE: &str = include_str!("../data/atexit_destructors.tsv");

/// One row of the table.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Row {
    /// `PUSH EBP; MOV EBP,ESP; MOV ECX,global; CALL destructor; POP EBP; RET`.
    Call { global: u32, destructor: u32 },
    /// `MOV ECX,global; JMP destructor`.
    Tail { global: u32, destructor: u32 },
}

fn hex(s: &str) -> Result<u32, String> {
    if s.len() != 8 {
        return Err(format!("not an 8-digit hex address: {s}"));
    }
    u32::from_str_radix(s, 16).map_err(|_| format!("not a hex address: {s}"))
}

/// The table's rows by function address.
pub fn parse(text: &str) -> Result<HashMap<u32, Row>, String> {
    let mut rows = HashMap::new();
    for (n, line) in text.lines().enumerate() {
        if line.starts_with('#') || line.trim().is_empty() {
            continue;
        }
        let at = |e: String| format!("atexit_destructors.tsv:{}: {e}", n + 1);
        let mut cols = line.split('\t');
        let (Some(entry), Some(shape), Some(params), None) =
            (cols.next(), cols.next(), cols.next(), cols.next())
        else {
            return Err(at("three columns expected".into()));
        };
        let entry = hex(entry).map_err(at)?;
        let Some((global, destructor)) = params.split_once(' ') else {
            return Err(at(format!("parameters `global destructor`, got {params}")));
        };
        let (global, destructor) = (hex(global).map_err(at)?, hex(destructor).map_err(at)?);
        let row = match shape {
            "call" => Row::Call { global, destructor },
            "tail" => Row::Tail { global, destructor },
            _ => return Err(at(format!("shape {shape}"))),
        };
        if rows.insert(entry, row).is_some() {
            return Err(at(format!("{entry:08x} twice")));
        }
    }
    Ok(rows)
}

/// The rows of the committed table, parsed once.
pub fn table() -> &'static HashMap<u32, Row> {
    static ROWS: OnceLock<HashMap<u32, Row>> = OnceLock::new();
    ROWS.get_or_init(|| parse(TABLE).expect("data/atexit_destructors.tsv"))
}

/// The row of the function at `addr`, if the table has one.
pub fn row(addr: u32) -> Option<Row> {
    table().get(&addr).copied()
}

/// Shape `call`: `MOV ECX,global; CALL destructor` inside a plain frame. The
/// destructor's answer is the function's.
pub fn run_call(e: &mut Engine, global: u32, destructor: u32) -> Ret {
    e.call(destructor, &[global])
}

/// Shape `tail`: `MOV ECX,global; JMP destructor`. The destructor's answer
/// is the function's.
pub fn run_tail(e: &mut Engine, global: u32, destructor: u32) -> Ret {
    e.call(destructor, &[global])
}

/// Runs a row.
pub fn run_row(e: &mut Engine, row: Row) -> Ret {
    match row {
        Row::Call { global, destructor } => run_call(e, global, destructor),
        Row::Tail { global, destructor } => run_tail(e, global, destructor),
    }
}

/// Whether the row can run: its destructor is translated.
pub fn can_run(e: &Engine, row: Row) -> bool {
    match row {
        Row::Call { destructor, .. } | Row::Tail { destructor, .. } => e.is_translated(destructor),
    }
}

/// Whether `addr` is a row whose destructor can run ([`can_run`]).
pub fn runs(e: &Engine, addr: u32) -> bool {
    row(addr).is_some_and(|r| can_run(e, r))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::abi::RetVal;
    use std::cell::RefCell;
    use std::rc::Rc;

    /// The functions of the engine map's atexit unit (address, size): the
    /// list the table was generated from.
    fn atexit_list() -> HashMap<u32, u32> {
        let map = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../research/engine-map/engine_map.tsv");
        let text = std::fs::read_to_string(&map).expect("research/engine-map/engine_map.tsv");
        let mut lines = text.lines().filter(|l| !l.starts_with('#'));
        let header: Vec<&str> = lines.next().expect("header").split('\t').collect();
        let col = |k: &str| header.iter().position(|c| *c == k).expect("column");
        let (address, size, unit) = (col("address"), col("size"), col("unit"));
        lines
            .map(|l| l.split('\t').collect::<Vec<_>>())
            .filter(|f| f[unit] == "compiler: atexit destructors")
            .map(|f| (hex(f[address]).unwrap(), f[size].parse().unwrap()))
            .collect()
    }

    #[test]
    fn the_committed_table_parses() {
        let rows = table();
        assert!(rows.len() > 4_000, "{} rows", rows.len());
        let calls = rows
            .values()
            .filter(|r| matches!(r, Row::Call { .. }))
            .count();
        assert!(calls > 4_000 && calls < rows.len());
    }

    #[test]
    fn rows_parse_by_shape() {
        let rows = parse(
            "# c\n\
             00fc7a70\tcall\t011c3264 00404a00\n\
             00fde581\ttail\t0126fc95 00ec3b35\n",
        )
        .unwrap();
        assert_eq!(
            rows[&0x00fc_7a70],
            Row::Call {
                global: 0x011c_3264,
                destructor: 0x0040_4a00
            }
        );
        assert_eq!(
            rows[&0x00fd_e581],
            Row::Tail {
                global: 0x0126_fc95,
                destructor: 0x00ec_3b35
            }
        );
        assert!(parse("00fc7a70\tcall\t011c3264\n").is_err());
        assert!(parse("00fc7a70\tframe\t011c3264 00404a00\n").is_err());
        assert!(parse("00fc7a70\tcall\t11c3264 00404a00\n").is_err());
        assert!(parse("00fc7a70\tcall\t011c3264 00404a00 00404a00\n").is_err());
        assert!(parse("00fc7a70\tcall\t011c3264 00404a00\textra\n").is_err());
        assert!(
            parse("00fc7a70\tcall\t011c3264 00404a00\n00fc7a70\ttail\t011c3264 00404a00\n")
                .is_err()
        );
    }

    /// Every row is one of the engine map's atexit destructor functions, and
    /// each has the size its shape's code has.
    #[test]
    fn every_row_is_an_atexit_destructor_of_the_engine_map() {
        let list = atexit_list();
        assert_eq!(list.len(), 5_020);
        for (addr, row) in table() {
            let size = list
                .get(addr)
                .unwrap_or_else(|| panic!("{addr:08x} is not an atexit destructor"));
            let expect = match row {
                Row::Call { .. } => 15,
                Row::Tail { .. } => 10,
            };
            assert_eq!(*size, expect, "{addr:08x}");
        }
    }

    type Log = Rc<RefCell<Vec<(u32, Vec<u32>)>>>;

    /// A double for `destructor` that records its calls and answers `ret`.
    fn double(e: &mut Engine, destructor: u32, ret: Ret) -> Log {
        let log: Log = Rc::default();
        let l = log.clone();
        e.register_double(destructor, move |_, a| {
            l.borrow_mut().push((destructor, a.to_vec()));
            ret
        });
        log
    }

    #[test]
    fn the_call_shape_runs_the_destructor_on_the_global_and_answers_its_result() {
        let mut e = Engine::new();
        let answer = Ret {
            eax: 7,
            edx: 8,
            st0: 1.5,
        };
        let log = double(&mut e, 0x0040_4a00, answer);
        let r = run_call(&mut e, 0x011c_3264, 0x0040_4a00);
        assert_eq!(*log.borrow(), [(0x0040_4a00, vec![0x011c_3264])]);
        assert_eq!(r, answer);
    }

    #[test]
    fn the_tail_shape_runs_the_destructor_on_the_global_and_answers_its_result() {
        let mut e = Engine::new();
        let answer = Ret {
            eax: 9,
            edx: 0,
            st0: 0.0,
        };
        let log = double(&mut e, 0x00ec_3b35, answer);
        let r = run_tail(&mut e, 0x0126_fc95, 0x00ec_3b35);
        assert_eq!(*log.borrow(), [(0x00ec_3b35, vec![0x0126_fc95])]);
        assert_eq!(r, answer);
    }

    #[test]
    fn a_row_is_callable_at_its_own_address() {
        let mut e = Engine::new();
        // 00fc7a70: call 00404a00 on 011c3264; 00fde581: tail 00ec3b35 on 0126fc95.
        assert_eq!(
            row(0x00fc_7a70),
            Some(Row::Call {
                global: 0x011c_3264,
                destructor: 0x0040_4a00
            })
        );
        e.register(0x0040_4a00, |_, a| a[0].into_ret());
        e.register(0x00ec_3b35, |_, a| (a[0] + 1).into_ret());
        assert!(e.is_translated(0x00fc_7a70));
        assert_eq!(e.call(0x00fc_7a70, &[]).u32(), 0x011c_3264);
        assert!(e.is_translated(0x00fd_e581));
        assert_eq!(e.call(0x00fd_e581, &[]).u32(), 0x0126_fc96);
        // Not a row, not translated.
        assert!(!e.is_translated(0x00fc_7a71));
    }

    #[test]
    fn a_row_whose_destructor_is_not_translated_is_not_translated() {
        let mut e = Engine::new();
        let row = Row::Call {
            global: 0x011c_3264,
            destructor: 0x00f0_0f00,
        };
        assert!(!e.is_translated(0x00f0_0f00));
        assert!(!can_run(&e, row));
        e.register(0x00f0_0f00, |_, _| Ret::default());
        assert!(can_run(&e, row));
        assert!(!runs(&e, 0x00f0_0f00), "not a row address");
    }

    #[test]
    fn a_registered_function_wins_over_the_row() {
        let mut e = Engine::new();
        e.register(0x0040_4a00, |_, _| 1u32.into_ret());
        e.register(0x00fc_7a70, |_, _| 2u32.into_ret());
        assert_eq!(e.call(0x00fc_7a70, &[]).u32(), 2);
    }
}

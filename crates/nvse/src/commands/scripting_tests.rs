//! Tests of the evaluator (I-6) and the loops (I-7), through the commands' records.

use super::super::test_support::*;
use super::*;

/// Tokens of xNVSE's bytecode.
fn var(var_type: u8, idx: u16) -> Vec<u8> {
    let mut b = vec![b'V', var_type, 0, 0];
    b.extend_from_slice(&idx.to_le_bytes());
    b
}
fn byte(v: u8) -> Vec<u8> {
    vec![b'B', v]
}
fn num(v: f64) -> Vec<u8> {
    let mut b = vec![b'Z'];
    b.extend_from_slice(&v.to_le_bytes());
    b
}
fn text(t: &str) -> Vec<u8> {
    let mut b = vec![b'S'];
    b.extend_from_slice(&(t.len() as u16).to_le_bytes());
    b.extend_from_slice(t.as_bytes());
    b
}
fn op(o: u8) -> Vec<u8> {
    vec![o]
}
/// A command token: `X`, no reference, the opcode, then its argument block (count and
/// arguments) with its length.
fn command(opcode: u16, args: &[Vec<u8>]) -> Vec<u8> {
    let mut block = (args.len() as u16).to_le_bytes().to_vec();
    block.extend(args.concat());
    let mut b = vec![b'X', 0, 0];
    b.extend_from_slice(&opcode.to_le_bytes());
    b.extend_from_slice(&(block.len() as u16 + 2).to_le_bytes());
    b.extend(block);
    b
}
/// One expression: its length word (counting itself) and its tokens.
fn expression(tokens: &[Vec<u8>]) -> Vec<u8> {
    let body = tokens.concat();
    let mut b = (body.len() as u16 + 2).to_le_bytes().to_vec();
    b.extend(body);
    b
}
/// The arguments of `Let`/`eval`: a count byte and one expression.
fn line(tokens: &[Vec<u8>]) -> Vec<u8> {
    let mut b = vec![1u8];
    b.extend(expression(tokens));
    b
}

/// Runs `opcode` with `bytes` as its argument data (the evaluator's layout).
fn run_raw(e: &mut Engine, opcode: u32, bytes: &[u8]) -> (u32, f64) {
    e.mem.set_u16(DATA, opcode as u16);
    e.mem.set_u16(DATA + 2, bytes.len() as u16);
    for (i, b) in bytes.iter().enumerate() {
        e.mem.set_u8(DATA + 4 + i as u32, *b);
    }
    e.mem.set_u32(OFFSET, 4);
    e.mem.set_f64(RESULT, 0.0);
    let record = crate::table::record_of(e, opcode);
    let params = e.mem.u32(record + 0x14);
    let execute = e.mem.u32(record + 0x18);
    let ok = e
        .call(execute, &[params, DATA, 0, 0, SCRIPT, LIST, RESULT, OFFSET])
        .eax;
    (ok & 0xff, e.mem.f64(RESULT))
}

fn upper(_: &mut Engine, a: &[u32]) -> Ret {
    Ret {
        eax: u32::from((a[0] as u8).to_ascii_uppercase()),
        ..Ret::default()
    }
}

fn no_form(_: &mut Engine, _: &[u32]) -> Ret {
    Ret::default()
}

fn setup() -> Engine {
    let mut e = engine_with_table();
    e.register(0x00ec_a7f4, upper);
    e.register(0x0048_39c0, no_form);
    // The script's form id: mod 05.
    e.mem.set_u32(SCRIPT + 0xc, 0x0500_0d00);
    e
}

/// Numbers: the compiler's postfix order, integer variables floored, `^` is `pow`, `%` and the
/// bit operators through 64-bit integers, `+=`.
#[test]
fn let_assigns_numbers_as_xnvse() {
    let mut e = setup();
    let x = add_local(&mut e, 1, "x", 0.0);
    let i = add_local(&mut e, 2, "i", 0.0);
    run_raw(
        &mut e,
        0x1539,
        &line(&[var(0, 1), byte(2), byte(3), byte(4), op(16), op(14), op(0)]),
    );
    assert_eq!(local_value(&e, x), 14.0);
    run_raw(
        &mut e,
        0x1539,
        &line(&[var(1, 2), byte(7), byte(2), op(17), op(0)]),
    );
    assert_eq!(local_value(&e, i), 3.0);
    run_raw(
        &mut e,
        0x1539,
        &line(&[var(0, 1), num(-7.0), byte(3), op(18), op(0)]),
    );
    assert_eq!(local_value(&e, x), -1.0);
    run_raw(
        &mut e,
        0x1539,
        &line(&[var(0, 1), byte(2), byte(10), op(19), op(0)]),
    );
    assert_eq!(local_value(&e, x), 1024.0);
    run_raw(
        &mut e,
        0x1539,
        &line(&[var(0, 1), byte(6), byte(9), op(10), op(0)]),
    );
    assert_eq!(local_value(&e, x), 15.0);
    run_raw(&mut e, 0x1539, &line(&[var(0, 1), byte(5), op(28)]));
    assert_eq!(local_value(&e, x), 20.0);
}

/// Strings: concatenation into a string variable owned by the script's mod, comparison without
/// case, `$` of a number as `%g`.
#[test]
fn strings_concatenate_compare_and_convert() {
    let mut e = setup();
    let s = add_local(&mut e, 1, "s", 0.0);
    run_raw(
        &mut e,
        0x1539,
        &line(&[var(2, 1), text("ab"), text("cd"), op(14), op(0)]),
    );
    let id = local_value(&e, s) as u32;
    assert_eq!(crate::vars::string(&e, id), Some(b"abcd".to_vec()));
    assert_eq!(e.ext::<Nvse>().vars.strings[&id].owning_mod, 5);
    assert_eq!(
        run_raw(&mut e, 0x153a, &line(&[var(2, 1), text("ABCD"), op(4)])).1,
        1.0
    );
    run_raw(&mut e, 0x1539, &line(&[var(2, 1), num(2.5), op(27), op(0)]));
    assert_eq!(crate::vars::string(&e, id), Some(b"2.5".to_vec()));
    // The string variable survives the tick; it is not temporary.
    e.site(0x0086_b386, &[]);
    assert!(crate::vars::string(&e, id).is_some());
}

/// Arrays: `&` boxes a value, `[]` reads and writes elements, the variable keeps a reference.
#[test]
fn arrays_box_subscript_and_assign() {
    let mut e = setup();
    let a = add_local(&mut e, 1, "a", 0.0);
    run_raw(&mut e, 0x1539, &line(&[var(3, 1), byte(5), op(37), op(0)]));
    let id = local_value(&e, a) as u32;
    assert!(crate::vars::array_exists(&e, id));
    assert_eq!(crate::vars::array(&e, id).unwrap().refs, vec![5]);
    run_raw(
        &mut e,
        0x1539,
        &line(&[var(3, 1), byte(0), op(24), byte(7), op(0)]),
    );
    assert_eq!(
        run_raw(
            &mut e,
            0x153a,
            &line(&[var(3, 1), byte(0), op(24), byte(7), op(4)])
        )
        .1,
        1.0
    );
    e.site(0x0086_b386, &[]);
    assert!(crate::vars::array_exists(&e, id));
}

/// `&&` and `||` short-circuit: the division by zero on the right is never evaluated;
/// `testexpr` answers 0 for a failing expression.
#[test]
fn logical_operators_short_circuit() {
    let mut e = setup();
    let (ok, v) = run_raw(
        &mut e,
        0x153a,
        &line(&[byte(0), byte(1), byte(0), op(17), op(2)]),
    );
    assert_eq!((ok, v), (1, 0.0));
    assert_eq!(
        run_raw(
            &mut e,
            0x153a,
            &line(&[byte(1), byte(1), byte(0), op(17), op(1)])
        )
        .1,
        1.0
    );
    assert_eq!(
        run_raw(&mut e, 0x1542, &line(&[byte(1), byte(0), op(17)])).1,
        0.0
    );
    assert_eq!(
        run_raw(&mut e, 0x1542, &line(&[byte(1), byte(2), op(17)])).1,
        1.0
    );
}

/// A command inside an expression runs through its record: `Floor 2.7 + 1` is 3.
#[test]
fn commands_run_inside_expressions() {
    let mut e = setup();
    let x = add_local(&mut e, 1, "x", 0.0);
    run_raw(
        &mut e,
        0x1539,
        &line(&[
            var(0, 1),
            command(0x14cb, &[z(2.7)]),
            byte(1),
            op(14),
            op(0),
        ]),
    );
    assert_eq!(local_value(&e, x), 3.0);
}

/// A parenthesised argument (`0xFFFF` and an expression) of a plain command:
/// `Floor (1 + 1.5)` is 2.
#[test]
fn inline_expressions_are_arguments() {
    let mut e = setup();
    let mut arg = vec![0xff, 0xff];
    arg.extend(expression(&[byte(1), num(1.5), op(14)]));
    assert_eq!(call(&mut e, 0x14cb, 0, &[arg]), (1, 2.0));
}

/// `EvaluateLine`'s stand-in for the loop tests: calls the command whose words are stored at
/// `0x0300_e000` (the execute address at `+0x20`).
fn base_evaluate(e: &mut Engine, _: &[u32]) -> Ret {
    let words: Vec<u32> = (0..8).map(|i| e.mem.u32(0x0300_e000 + 4 * i)).collect();
    let execute = e.mem.u32(0x0300_e020);
    e.call(execute, &words)
}

fn nothing(_: &mut Engine, _: &[u32]) -> Ret {
    Ret::default()
}

const RUNNER: u32 = 0x0300_d000;
const LENGTH: u32 = 0x0300_d100;

/// An engine whose `EvaluateLine` runs one stored command line, with the context's wraps.
fn line_engine() -> Engine {
    let mut e = setup();
    for addr in [0x005e_0d20, 0x005e_1160, 0x005e_13a0] {
        e.register(addr, nothing);
    }
    e.register(crate::context::EVALUATE_LINE.addr(), base_evaluate);
    crate::context::install(&mut e);
    e
}

/// Runs `opcode` with `bytes` as one script line through `EvaluateLine`.
fn run_line(e: &mut Engine, opcode: u32, bytes: &[u8]) {
    e.mem.set_u16(DATA, opcode as u16);
    for (i, b) in bytes.iter().enumerate() {
        e.mem.set_u8(DATA + 4 + i as u32, *b);
    }
    e.mem.set_u32(OFFSET, 4);
    let record = crate::table::record_of(e, opcode);
    let words = [
        e.mem.u32(record + 0x14),
        DATA,
        0,
        0,
        SCRIPT,
        LIST,
        RESULT,
        OFFSET,
    ];
    for (i, w) in words.iter().enumerate() {
        e.mem.set_u32(0x0300_e000 + 4 * i as u32, *w);
    }
    e.mem.set_u32(0x0300_e020, e.mem.u32(record + 0x18));
    e.call(
        crate::context::EVALUATE_LINE.addr(),
        &[RUNNER, SCRIPT, opcode, 0, LENGTH, 4, 0, 1, 0, 0],
    );
}

/// `while` with a false condition breaks at once: the running line's length moves the runner
/// to the loop's end and the if stack is put back.
#[test]
fn a_false_while_jumps_to_its_end() {
    let mut e = line_engine();
    e.mem.set_u32(RUNNER + 0x20, 1);
    e.mem.set_u32(RUNNER + 0x24, 0xaa);
    e.mem.set_u32(RUNNER + 0x28, 0xbb);
    e.mem.set_u32(LENGTH, 100);
    // End offset 0x40, count 1, expression `B 0` (4 bytes with its length word).
    let mut bytes = 0x40u32.to_le_bytes().to_vec();
    bytes.push(1);
    bytes.extend(expression(&[byte(0)]));
    run_line(&mut e, 0x153b, &bytes);
    let start = 4 + 5 + 4;
    assert_eq!(e.mem.u32(LENGTH), 100 + 0x40 - start);
    assert_eq!(e.mem.u32(RUNNER + 0x24), 0xaa);
}

/// A true `while` stays in the loop (no jump); `loop` re-tests the condition and, now false,
/// jumps to the end; `break` outside a loop fails the line.
#[test]
fn while_loop_and_break() {
    let mut e = line_engine();
    let x = add_local(&mut e, 1, "x", 1.0);
    e.mem.set_u32(LENGTH, 100);
    let mut bytes = 0x40u32.to_le_bytes().to_vec();
    bytes.push(1);
    bytes.extend(expression(&[var(0, 1)]));
    run_line(&mut e, 0x153b, &bytes);
    assert_eq!(e.mem.u32(LENGTH), 100, "the condition holds: no jump");
    e.mem.set_f64(x + 8, 0.0);
    e.mem.set_u32(LENGTH, 7);
    run_line(&mut e, 0x153c, &[]);
    // `loop` at offset 4: the condition fails, so the runner jumps to the end (0x40).
    assert_eq!(e.mem.u32(LENGTH), 7 + 0x40 - 4);
    assert!(loops(&mut e).is_empty());
    let record = crate::table::record_of(&e, 0x153f);
    let execute = e.mem.u32(record + 0x18);
    let ok = e
        .call(execute, &[0, DATA, 0, 0, SCRIPT, LIST, RESULT, OFFSET])
        .eax;
    assert_eq!(ok & 0xff, 0);
}

/// `ForEachAlt arr fVal`: the value variable takes each element; when the loop ends it is
/// zeroed (the real game: `alt_sum 18 last 0` for 5, 6, 7, `real-results/udf3.txt`).
#[test]
fn foreach_alt_passes_values_and_zeroes_the_variable_at_the_end() {
    let mut e = line_engine();
    let value = add_local(&mut e, 1, "fVal", 0.0);
    let arr_local = add_local(&mut e, 2, "aArr", 0.0);
    let arr = vars::create_array(&mut e, vars::KEY_NUMBER, true, 5);
    for (i, v) in [5.0, 6.0, 7.0].into_iter().enumerate() {
        vars::set_element(&mut e, arr, &Key::Number(i as f64), vars::Value::Number(v));
    }
    e.mem.set_f64(arr_local + 8, f64::from(arr));
    e.mem.set_u32(LENGTH, 100);
    let mut bytes = 0x40u32.to_le_bytes().to_vec();
    bytes.push(2);
    bytes.extend(expression(&[var(3, 2)]));
    bytes.extend(expression(&[var(0, 1)]));
    run_line(&mut e, 0x1670, &bytes);
    assert_eq!(e.mem.u32(LENGTH), 100, "not empty: no jump");
    let mut seen = vec![e.mem.f64(value + 8)];
    for _ in 0..2 {
        run_line(&mut e, 0x153c, &[]);
        seen.push(e.mem.f64(value + 8));
    }
    assert_eq!(seen, [5.0, 6.0, 7.0]);
    run_line(&mut e, 0x153c, &[]);
    assert!(loops(&mut e).is_empty());
    assert_eq!(e.mem.f64(value + 8), 0.0);
}

/// Review item 7: `string * n` repeats an `int` count (`cvttsd2si`); a count past `int` is
/// `INT_MIN`, nothing; a result past 64 MiB is a script error, nothing allocated.
#[test]
fn string_times_number_counts_in_int() {
    let mut e = setup();
    let s = add_local(&mut e, 1, "s", 0.0);
    let repeat = |e: &mut Engine, n: f64| {
        run_raw(
            e,
            0x1539,
            &line(&[var(2, 1), text("ab"), num(n), op(16), op(0)]),
        );
        let id = local_value(e, s) as u32;
        crate::vars::string(e, id)
    };
    assert_eq!(repeat(&mut e, 3.9), Some(b"ababab".to_vec()));
    assert_eq!(repeat(&mut e, -2.0), Some(Vec::new()));
    assert_eq!(repeat(&mut e, 1e12), Some(Vec::new()));
    run_raw(&mut e, 0x1539, &line(&[var(2, 1), text("x"), op(0)]));
    assert_eq!(
        repeat(&mut e, 2e9),
        Some(b"x".to_vec()),
        "too long: not assigned"
    );
}

/// `Run`'s stand-in: a script that begins a loop and returns inside it; the outer one (`a[2]`
/// 0) also calls an inner one first and notes how many loops are left after it at `RESULT`.
fn run_leaving_a_loop(e: &mut Engine, a: &[u32]) -> Ret {
    add_loop(e, Loop::While { expr_offset: 0 }, a[0], 4, 0x40);
    if a[2] == 0 {
        e.call(0x005e_0d20, &[RUNNER + 0x100, SCRIPT, 1]);
        let left = loops(e).len();
        e.mem.set_u32(RESULT, left as u32);
    }
    Ret::default()
}

/// Review item 4: a loop a called script leaves (`return` inside `ForEach`/`while`) ends with
/// that call, so the caller's `loop` finds its own loop; the caller's goes when it ends.
#[test]
fn a_loop_left_inside_a_call_ends_with_the_call() {
    let mut e = setup();
    for addr in [0x005e_1160, 0x005e_13a0, 0x005e_1550] {
        e.register(addr, nothing);
    }
    e.register(0x005e_0d20, run_leaving_a_loop);
    crate::context::install(&mut e);
    e.call(0x005e_0d20, &[RUNNER, SCRIPT, 0]);
    assert_eq!(
        e.mem.u32(RESULT),
        1,
        "only the caller's loop after the inner call"
    );
    assert!(loops(&mut e).is_empty());
}

/// Review item 6, checked against xNVSE 6.4.9: `UpdateForEachAltIterator` stores an inner
/// array's id in the value variable without a reference (`Loops.cpp`, `kDataType_Array`:
/// `GetScriptLocal()->data = elem->m_data.arrID`), and the loop's end zeroes it without one.
/// The port does the same: the inner array's references are the outer element's alone.
#[test]
fn foreach_alt_takes_no_reference_to_an_inner_array() {
    let mut e = line_engine();
    let value = add_local(&mut e, 1, "aVal", 0.0);
    let outer_local = add_local(&mut e, 2, "aOuter", 0.0);
    let outer = vars::create_array(&mut e, vars::KEY_NUMBER, true, 5);
    let inner = vars::create_array(&mut e, vars::KEY_NUMBER, true, 5);
    vars::set_element(&mut e, outer, &Key::Number(0.0), vars::Value::Array(inner));
    e.mem.set_f64(outer_local + 8, f64::from(outer));
    let refs_before = vars::array(&e, inner).unwrap().refs.clone();
    e.mem.set_u32(LENGTH, 100);
    let mut bytes = 0x40u32.to_le_bytes().to_vec();
    bytes.push(2);
    bytes.extend(expression(&[var(3, 2)]));
    bytes.extend(expression(&[var(3, 1)]));
    run_line(&mut e, 0x1670, &bytes);
    assert_eq!(e.mem.f64(value + 8), f64::from(inner));
    assert_eq!(vars::array(&e, inner).unwrap().refs, refs_before);
    run_line(&mut e, 0x153c, &[]);
    assert_eq!(e.mem.f64(value + 8), 0.0);
    assert_eq!(vars::array(&e, inner).unwrap().refs, refs_before);
}

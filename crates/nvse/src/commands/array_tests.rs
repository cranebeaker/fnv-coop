//! Tests of the array commands through their records, on the evaluator's arguments.

use super::super::test_support::*;
use super::*;

fn ret(v: u32) -> Ret {
    Ret {
        eax: v,
        ..Ret::default()
    }
}

/// The table, ASCII `toupper`, no forms, no console, an empty data handler.
fn setup() -> Engine {
    let mut e = engine_with_table();
    e.register(0x00ec_a7f4, |_, a| {
        ret(u32::from((a[0] as u8).to_ascii_uppercase()))
    });
    e.register(LOOKUP_FORM, |_, _| ret(0));
    e.register(0x0071_b160, |_, _| ret(0));
    e.map(0x011c_3000, 0x1000);
    e.mem.set_u32(SCRIPT + 0xc, 0x0500_0d00);
    e
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
fn var(var_type: u8, idx: u16) -> Vec<u8> {
    let mut b = vec![b'V', var_type, 0, 0];
    b.extend_from_slice(&idx.to_le_bytes());
    b
}
fn arr(idx: u16) -> Vec<Vec<u8>> {
    vec![var(3, idx)]
}
fn n1(v: f64) -> Vec<Vec<u8>> {
    vec![num(v)]
}
fn t1(t: &str) -> Vec<Vec<u8>> {
    vec![text(t)]
}
/// `left::right`.
fn pair(l: Vec<u8>, r: Vec<u8>) -> Vec<Vec<u8>> {
    vec![l, r, vec![expr::op::MAKE_PAIR]]
}
/// `lo:hi`.
fn slice(lo: f64, hi: f64) -> Vec<Vec<u8>> {
    vec![num(lo), num(hi), vec![expr::op::SLICE]]
}

/// Runs `opcode` with these expressions as its arguments; answers the result.
fn run(e: &mut Engine, opcode: u32, exprs: &[Vec<Vec<u8>>]) -> f64 {
    let mut bytes = vec![exprs.len() as u8];
    for x in exprs {
        let body = x.concat();
        bytes.extend_from_slice(&(body.len() as u16 + 2).to_le_bytes());
        bytes.extend(body);
    }
    e.mem.set_u16(DATA, opcode as u16);
    e.mem.set_u16(DATA + 2, bytes.len() as u16);
    e.mem.write(DATA + 4, &bytes);
    e.mem.set_u32(OFFSET, 4);
    e.mem.set_f64(RESULT, 0.0);
    let record = crate::table::record_of(e, opcode);
    let (params, execute) = (e.mem.u32(record + 0x14), e.mem.u32(record + 0x18));
    e.call(execute, &[params, DATA, 0, 0, SCRIPT, LIST, RESULT, OFFSET]);
    e.mem.f64(RESULT)
}

/// An array of these values in local `idx`.
fn make(e: &mut Engine, idx: u16, key: u8, packed: bool, items: &[(Key, Value)]) -> u32 {
    let id = vars::create_array(e, key, packed, 5);
    for (k, v) in items {
        vars::set_element(e, id, k, v.clone());
    }
    add_local(e, idx, &format!("a{idx}"), f64::from(id));
    id
}

fn nums(vs: &[f64]) -> Vec<(Key, Value)> {
    vs.iter()
        .enumerate()
        .map(|(i, v)| (Key::Number(i as f64), Value::Number(*v)))
        .collect()
}

fn values(e: &Engine, id: f64) -> Vec<String> {
    entries(e, id as u32)
        .into_iter()
        .map(|(k, x)| {
            let v = match x.data {
                Data::Number(n) => format!("{n}"),
                Data::Str(s) => String::from_utf8_lossy(&s).into_owned(),
                Data::Form(f) => format!("form {f:x}"),
                Data::Array(a) => format!("array {a}"),
                Data::Invalid => "invalid".into(),
            };
            match k {
                Key::Number(n) if !is_packed(e, id as u32) => format!("{n}={v}"),
                Key::Str(s) => format!("{}={v}", String::from_utf8_lossy(&s)),
                Key::Number(_) => v,
            }
        })
        .collect()
}

fn text_of(e: &Engine, id: f64) -> String {
    String::from_utf8_lossy(&vars::string(e, id as u32).expect("a string")).into_owned()
}

/// `ar_List`, `ar_Map`, `ar_Range`, `ar_Init`, `ar_Size`, `ar_Packed`, `ar_Construct`.
#[test]
fn arrays_are_made_as_xnvse_makes_them() {
    let mut e = setup();
    let l = run(&mut e, 0x1567, &[n1(1.0), t1("a"), n1(2.5)]);
    assert_eq!(values(&e, l), ["1", "a", "2.5"]);
    let m = run(
        &mut e,
        0x1568,
        &[pair(num(3.0), text("x")), pair(num(-1.0), num(5.0))],
    );
    assert_eq!(values(&e, m), ["-1=5", "3=x"]);
    let sm = run(
        &mut e,
        0x1568,
        &[
            pair(text("b"), num(1.0)),
            pair(text("A"), num(2.0)),
            pair(num(7.0), num(3.0)),
        ],
    );
    assert_eq!(
        values(&e, sm),
        ["=3", "A=2", "b=1"],
        "a number key in a string map is \"\""
    );
    assert_eq!(run(&mut e, 0x1568, &[n1(1.0)]), 0.0, "not a pair: no array");
    let r = run(&mut e, 0x1569, &[n1(0.0), n1(10.0), n1(3.0)]);
    assert_eq!(values(&e, r), ["0", "3", "6", "9"]);
    let r = run(&mut e, 0x1569, &[n1(5.0), n1(1.0), n1(-2.0)]);
    assert_eq!(values(&e, r), ["5", "3", "1"]);
    let r = run(&mut e, 0x1569, &[n1(2.9), n1(4.2)]);
    assert_eq!(values(&e, r), ["2", "3", "4"]);
    let i = run(&mut e, 0x162e, &[n1(3.0), t1("x")]);
    assert_eq!(values(&e, i), ["x", "x", "x"]);
    assert_eq!(run(&mut e, 0x162e, &[n1(0.0), t1("x")]), 0.0);
    make(&mut e, 1, KEY_NUMBER, true, &nums(&[1.0, 2.0]));
    assert_eq!(run(&mut e, 0x1550, &[arr(1)]), 2.0);
    assert_eq!(run(&mut e, 0x1550, &[n1(1.0)]), -1.0);
    assert_eq!(run(&mut e, 0x1604, &[arr(1)]), 1.0);
    add_local(&mut e, 9, "m", m);
    assert_eq!(run(&mut e, 0x1604, &[arr(9)]), 0.0);
    // `ar_Construct` makes an array only inside an expression.
    let a = call(&mut e, 0x154f, 0, &[s("StringMap")]).1;
    assert_eq!(a, 0.0);
    let made = |e: &mut Engine, kind: &str| {
        expr::with_evaluator(e, |e| call(e, 0x154f, 0, &[s(kind)]).1) as u32
    };
    let (sm, nm, pk) = (
        made(&mut e, "stringmap"),
        made(&mut e, "Map"),
        made(&mut e, "Array"),
    );
    assert_eq!(key_type(&e, sm), KEY_STRING);
    assert_eq!((key_type(&e, nm), is_packed(&e, nm)), (KEY_NUMBER, false));
    assert!(is_packed(&e, pk));
}

/// `ar_Find`, `ar_First`/`Last`/`Next`/`Prev`, `ar_HasKey`, `ar_Keys`, the bad indexes.
#[test]
fn finding_and_walking_keys() {
    let mut e = setup();
    make(
        &mut e,
        1,
        KEY_NUMBER,
        true,
        &nums(&[10.0, 20.0, 30.0, 20.0]),
    );
    let s = |k: &str| Key::Str(k.as_bytes().to_vec());
    make(
        &mut e,
        2,
        KEY_STRING,
        false,
        &[(s("a"), Value::Number(1.0)), (s("B"), Value::Number(2.0))],
    );
    assert_eq!(run(&mut e, 0x1557, &[n1(20.0), arr(1)]), 1.0);
    assert_eq!(run(&mut e, 0x1557, &[n1(99.0), arr(1)]), -99999.0);
    assert_eq!(
        run(&mut e, 0x1557, &[n1(20.0), arr(1), slice(2.0, 9.0)]),
        3.0
    );
    assert_eq!(
        run(&mut e, 0x1557, &[n1(20.0), arr(1), slice(3.0, 2.0)]),
        -99999.0
    );
    let found = run(&mut e, 0x1557, &[n1(2.0), arr(2)]);
    assert_eq!(text_of(&e, found), "B");
    let missing = run(&mut e, 0x1557, &[n1(9.0), arr(2)]);
    assert_eq!(text_of(&e, missing), "");
    assert_eq!(run(&mut e, 0x1558, &[arr(1)]), 0.0);
    assert_eq!(run(&mut e, 0x1559, &[arr(1)]), 3.0);
    assert_eq!(run(&mut e, 0x155a, &[arr(1), n1(1.0)]), 2.0);
    assert_eq!(run(&mut e, 0x155a, &[arr(1), n1(3.0)]), -99999.0);
    assert_eq!(run(&mut e, 0x155b, &[arr(1), n1(0.0)]), -99999.0);
    assert_eq!(
        run(&mut e, 0x155b, &[arr(1), n1(-1.0)]),
        -99999.0,
        "no index from the end"
    );
    let first = run(&mut e, 0x1558, &[arr(2)]);
    assert_eq!(text_of(&e, first), "a");
    let next = run(&mut e, 0x155a, &[arr(2), t1("A")]);
    assert_eq!(text_of(&e, next), "B");
    let none = run(&mut e, 0x155a, &[arr(2), t1("b")]);
    assert_eq!(text_of(&e, none), "");
    assert_eq!(run(&mut e, 0x155d, &[arr(1), n1(-1.0)]), 1.0);
    assert_eq!(run(&mut e, 0x155d, &[arr(1), n1(4.0)]), 0.0);
    assert_eq!(run(&mut e, 0x155d, &[arr(2), t1("b")]), 1.0);
    assert_eq!(run(&mut e, 0x155d, &[arr(2), n1(0.0)]), 0.0);
    assert_eq!(run(&mut e, 0x155f, &[]), -99999.0);
    let bad = run(&mut e, 0x155e, &[]);
    assert_eq!(text_of(&e, bad), "");
    assert_eq!(run(&mut e, 0x155c, &[arr(2)]), 0.0, "outside an expression");
    let keys = expr::with_evaluator(&mut e, |e| run(e, 0x155c, &[arr(2)]));
    assert_eq!(values(&e, keys), ["a", "B"]);
}

/// `ar_Erase`, `ar_Insert`, `ar_InsertRange`, `ar_Append`, `ar_Resize`, `ar_Cat`.
#[test]
fn editing_arrays() {
    let mut e = setup();
    let a = make(
        &mut e,
        1,
        KEY_NUMBER,
        true,
        &nums(&[0.0, 1.0, 2.0, 3.0, 4.0, 5.0]),
    );
    assert_eq!(run(&mut e, 0x1553, &[arr(1), n1(0.0)]), 1.0);
    assert_eq!(run(&mut e, 0x1553, &[arr(1), slice(1.0, 2.0)]), 2.0);
    assert_eq!(values(&e, f64::from(a)), ["1", "4", "5"]);
    assert_eq!(run(&mut e, 0x1553, &[arr(1), t1("x")]), 4294967295.0);
    assert_eq!(run(&mut e, 0x1553, &[arr(1), n1(7.0)]), 0.0);
    assert_eq!(run(&mut e, 0x1564, &[arr(1), n1(1.0), t1("in")]), 1.0);
    assert_eq!(run(&mut e, 0x1564, &[arr(1), n1(9.0), t1("no")]), 0.0);
    assert_eq!(run(&mut e, 0x1566, &[arr(1), n1(6.0)]), 1.0);
    assert_eq!(values(&e, f64::from(a)), ["1", "in", "4", "5", "6"]);
    make(&mut e, 2, KEY_NUMBER, true, &nums(&[8.0, 9.0]));
    assert_eq!(run(&mut e, 0x1565, &[arr(1), n1(0.0), arr(2)]), 1.0);
    assert_eq!(
        values(&e, f64::from(a)),
        ["8", "9", "1", "in", "4", "5", "6"]
    );
    assert_eq!(run(&mut e, 0x1563, &[arr(1), n1(2.0)]), 1.0);
    assert_eq!(run(&mut e, 0x1563, &[arr(1), n1(4.0), t1("p")]), 1.0);
    assert_eq!(values(&e, f64::from(a)), ["8", "9", "p", "p"]);
    assert_eq!(run(&mut e, 0x1553, &[arr(1)]), 4.0);
    assert_eq!(run(&mut e, 0x1553, &[arr(1)]), 0.0);
    assert_eq!(
        run(&mut e, 0x1553, &[arr(1), n1(0.0)]),
        4294967295.0,
        "empty"
    );
    // A number map erases whole keys from the low end up to, not including, the high one.
    let m = make(
        &mut e,
        3,
        KEY_NUMBER,
        false,
        &[
            (Key::Number(1.0), Value::Number(1.0)),
            (Key::Number(1.5), Value::Number(2.0)),
            (Key::Number(3.0), Value::Number(3.0)),
        ],
    );
    assert_eq!(run(&mut e, 0x1553, &[arr(3), slice(0.0, 3.0)]), 1.0);
    assert_eq!(values(&e, f64::from(m)), ["1.5=2", "3=3"]);
    assert_eq!(
        run(&mut e, 0x1563, &[arr(3), n1(1.0)]),
        0.0,
        "a map has no size to set"
    );
    // `ar_Cat`: an array appends; a map adds the keys it lacks (all with the flag).
    let x = make(
        &mut e,
        4,
        KEY_NUMBER,
        false,
        &[(Key::Number(1.0), Value::Number(1.0))],
    );
    make(
        &mut e,
        5,
        KEY_NUMBER,
        false,
        &[
            (Key::Number(1.0), Value::Number(9.0)),
            (Key::Number(2.0), Value::Number(2.0)),
        ],
    );
    run(&mut e, 0x167e, &[arr(4), arr(5)]);
    assert_eq!(values(&e, f64::from(x)), ["1=1", "2=2"]);
    run(&mut e, 0x167e, &[arr(4), arr(5), n1(1.0)]);
    assert_eq!(values(&e, f64::from(x)), ["1=9", "2=2"]);
    let y = make(&mut e, 6, KEY_NUMBER, true, &nums(&[1.0]));
    run(&mut e, 0x167e, &[arr(6), arr(2)]);
    assert_eq!(values(&e, f64::from(y)), ["1", "8", "9"]);
    run(&mut e, 0x167e, &[arr(6), arr(4)]);
    assert_eq!(
        values(&e, f64::from(y)),
        ["1", "8", "9"],
        "different containers"
    );
}

/// Copies, sorts, comparisons, counts, `ar_Exists`, `ar_GetNth`, `ar_Unique`.
#[test]
fn copying_sorting_and_comparing() {
    let mut e = setup();
    let inner = make(&mut e, 1, KEY_NUMBER, true, &nums(&[7.0]));
    let outer = make(
        &mut e,
        2,
        KEY_NUMBER,
        true,
        &[
            (Key::Number(0.0), Value::Array(inner)),
            (Key::Number(1.0), Value::Str(b"s".to_vec())),
        ],
    );
    let shallow = run(&mut e, 0x1560, &[arr(2)]);
    assert_eq!(values(&e, shallow), [format!("array {inner}"), "s".into()]);
    let deep = run(&mut e, 0x1561, &[arr(2)]);
    let copied = match &entries(&e, deep as u32)[0].1.data {
        Data::Array(a) => *a,
        other => panic!("{other:?}"),
    };
    assert_ne!(copied, inner);
    assert_eq!(values(&e, f64::from(copied)), ["7"]);
    add_local(&mut e, 3, "d", deep);
    assert_eq!(run(&mut e, 0x162f, &[arr(2), arr(3)]), 1.0);
    vars::set_element(&mut e, copied, &Key::Number(0.0), Value::Number(8.0));
    assert_eq!(run(&mut e, 0x162f, &[arr(2), arr(3)]), 0.0);
    assert_eq!(
        run(&mut e, 0x162f, &[n1(1.0), n1(2.0)]),
        1.0,
        "neither an array"
    );
    let _ = outer;
    let list = make(
        &mut e,
        4,
        KEY_NUMBER,
        true,
        &[
            (Key::Number(0.0), Value::Str(b"b".to_vec())),
            (Key::Number(1.0), Value::Str(b"A".to_vec())),
            (Key::Number(2.0), Value::Number(1.0)),
            (Key::Number(3.0), Value::Str(b"c".to_vec())),
            (Key::Number(4.0), Value::Str(b"a".to_vec())),
        ],
    );
    let up = run(&mut e, 0x1554, &[arr(4)]);
    assert_eq!(
        values(&e, up),
        ["A", "a", "b", "c"],
        "strings only, case-blind, stable"
    );
    let down = run(&mut e, 0x1554, &[arr(4), n1(1.0)]);
    assert_eq!(values(&e, down), ["c", "b", "a", "A"]);
    let _ = list;
    make(&mut e, 5, KEY_NUMBER, true, &nums(&[3.0, 1.0, 2.0, 1.0]));
    let sorted = run(&mut e, 0x1556, &[arr(5)]);
    assert_eq!(
        values(&e, sorted),
        ["1", "1", "2", "3"],
        "alpha of numbers sorts by value"
    );
    let unique = run(&mut e, 0x1635, &[arr(5)]);
    assert_eq!(values(&e, unique), ["3", "1", "2"]);
    assert_eq!(run(&mut e, 0x1672, &[arr(5), n1(1.0)]), 2.0);
    assert_eq!(run(&mut e, 0x1671, &[arr(5), n1(2.0)]), 1.0);
    assert_eq!(
        run(&mut e, 0x1671, &[arr(5), n1(2.0), slice(0.0, 1.0)]),
        0.0
    );
    assert_eq!(
        run(&mut e, 0x1671, &[arr(5), pair(num(-1.0), num(1.0))]),
        1.0
    );
    assert_eq!(
        run(&mut e, 0x1671, &[arr(5), pair(num(0.0), num(1.0))]),
        0.0
    );
    let s = |k: &str| Key::Str(k.as_bytes().to_vec());
    make(
        &mut e,
        6,
        KEY_STRING,
        false,
        &[(s("k"), Value::Str(b"v".to_vec()))],
    );
    assert_eq!(run(&mut e, 0x1671, &[arr(6), t1("K")]), 1.0);
    assert_eq!(
        run(&mut e, 0x1671, &[arr(6), pair(text("k"), text("V"))]),
        1.0
    );
    assert_eq!(run(&mut e, 0x1675, &[arr(5), n1(2.0)]), 2.0);
    assert_eq!(run(&mut e, 0x1675, &[arr(5), n1(-1.0)]), 0.0);
    let v = run(&mut e, 0x1675, &[arr(6), n1(0.0)]);
    assert_eq!(text_of(&e, v), "v");
    // No array: xNVSE reads through the null pointer and the game stops (vanilla); the
    // missing array guard row answers 0.
    add_local(&mut e, 7, "stale", 999.0);
    for (op, args) in [(0x1675, [n1(3.0), n1(0.0)]), (0x1672, [arr(7), n1(0.0)])] {
        let mut vanilla = setup();
        add_local(&mut vanilla, 7, "stale", 999.0);
        let stopped = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            run(&mut vanilla, op, &args);
        }));
        assert!(stopped.is_err(), "{op:#x} stops the game in vanilla");
    }
    set_fix(&mut e, NULL_GUARD, true);
    assert_eq!(run(&mut e, 0x1675, &[n1(3.0), n1(0.0)]), 0.0, "no array");
    assert_eq!(run(&mut e, 0x1672, &[arr(7), n1(0.0)]), 0.0, "no array");
    assert_eq!(
        run(&mut e, 0x1672, &[n1(3.0), n1(0.0)]),
        0.0,
        "not an array"
    );
}

thread_local! {
    /// The stand-in file system of `ar_DumpF`'s test: files by path, open handles (path, at end).
    static FILES: std::cell::RefCell<std::collections::HashMap<Vec<u8>, Vec<u8>>> =
        std::cell::RefCell::new(std::collections::HashMap::new());
    static HANDLES: std::cell::RefCell<std::collections::HashMap<u32, Vec<u8>>> =
        std::cell::RefCell::new(std::collections::HashMap::new());
}

fn create_file(e: &mut Engine, a: &[u32]) -> Ret {
    let path = crate::table::guest_text(e, a[0]);
    FILES.with(|f| {
        let mut f = f.borrow_mut();
        if a[4] == CREATE_ALWAYS || !f.contains_key(&path) {
            f.insert(path.clone(), Vec::new());
        }
    });
    let h = HANDLES.with(|h| {
        let mut h = h.borrow_mut();
        let id = 0x100 + h.len() as u32;
        h.insert(id, path);
        id
    });
    ret(h)
}

fn write_file(e: &mut Engine, a: &[u32]) -> Ret {
    let path = HANDLES.with(|h| h.borrow()[&a[0]].clone());
    let bytes = e.mem.bytes(a[1], a[2]);
    FILES.with(|f| f.borrow_mut().get_mut(&path).unwrap().extend(bytes));
    e.mem.set_u32(a[3], a[2]);
    ret(1)
}

/// The dump's lines, and `ar_DumpF` writing them through the game's file calls (here a
/// stand-in in memory) with the C runtime's text-mode new lines.
#[test]
fn dumps_list_every_element() {
    let mut e = setup();
    e.register(CREATE_FILE_A, create_file);
    e.register(WRITE_FILE, write_file);
    e.register(SET_FILE_POINTER, |_, _| ret(0));
    e.register(CLOSE_HANDLE, |_, _| ret(1));
    let id = make(
        &mut e,
        1,
        KEY_NUMBER,
        true,
        &[
            (Key::Number(0.0), Value::Number(1.5)),
            (Key::Number(1.0), Value::Str(b"s".to_vec())),
        ],
    );
    let s = |k: &str| Key::Str(k.as_bytes().to_vec());
    let m = make(
        &mut e,
        2,
        KEY_STRING,
        false,
        &[(s("x"), Value::Number(2.0)), (s("y"), Value::Array(id))],
    );
    let lines: Vec<String> = dump_lines(&mut e, m)
        .into_iter()
        .map(|l| String::from_utf8_lossy(&l).into_owned())
        .collect();
    assert_eq!(
        lines,
        [
            format!("** Dumping Array #{m} **\nRefs: 0 Owner 05: "),
            "[ x ] : 2.000000".to_string(),
            "[ y ] : [1.5, \"s\"]".to_string(),
        ]
    );
    let path = r"..\dump.txt";
    run(&mut e, 0x1625, &[arr(1), t1(path), n1(0.0)]);
    run(&mut e, 0x1625, &[arr(1), t1(path)]);
    let written = FILES.with(|f| f.borrow()[path.as_bytes()].clone());
    let one = format!("** Dumping Array #{id} **\r\nRefs: 1 Owner 05: \r\n[ 0.000000 ] : 1.500000\r\n[ 1.000000 ] : s\r\n");
    assert_eq!(String::from_utf8_lossy(&written), one.repeat(2));
    run(&mut e, 0x1625, &[arr(1), t1(path), n1(0.0)]);
    let written = FILES.with(|f| f.borrow()[path.as_bytes()].clone());
    assert_eq!(
        String::from_utf8_lossy(&written),
        one,
        "\"w+\" makes the file anew"
    );
}

/// The function commands: no function script, no run (each answers 0).
#[test]
fn function_commands_need_a_function() {
    let mut e = setup();
    make(&mut e, 1, KEY_NUMBER, true, &nums(&[1.0]));
    for op in [0x1627, 0x1628, 0x1629, 0x1630, 0x1631, 0x1632, 0x1673] {
        assert_eq!(run(&mut e, op, &[arr(1), n1(0.0)]), 0.0, "{op:#x}");
    }
    assert_eq!(run(&mut e, 0x162d, &[n1(2.0), n1(0.0)]), 0.0);
    let sorted = run(&mut e, 0x1555, &[arr(1), n1(0.0)]);
    assert!(
        values(&e, sorted).is_empty(),
        "no comparator: an empty array"
    );
}

/// The walk the function commands make sees the array as it is at each step: an element the
/// function appends is walked, one it erases ahead is not.
#[test]
fn the_walk_follows_the_live_array() {
    let mut e = setup();
    let a = make(&mut e, 1, KEY_NUMBER, true, &nums(&[1.0, 2.0, 3.0]));
    let mut seen = Vec::new();
    walk(&mut e, a, |e, _, x| {
        let Data::Number(n) = x.data else {
            return false;
        };
        seen.push(n);
        if n == 1.0 {
            vars::erase_element(e, a, &Key::Number(1.0));
            insert_at(e, a, 2, &Data::Number(9.0));
        }
        true
    });
    assert_eq!(seen, [1.0, 3.0, 9.0]);
}

/// An array that holds itself: with the cycle guard row the deep copy holds its own copy
/// there, the deep compare counts the pair as equal and the dump writes `[...]` (vanilla walks
/// on until the stack runs out, as xNVSE).
#[test]
fn the_cycle_guard_ends_self_holding_walks() {
    let mut e = setup();
    set_fix(&mut e, CYCLE_GUARD, true);
    let a = make(&mut e, 1, KEY_NUMBER, true, &nums(&[1.0]));
    vars::set_element(&mut e, a, &Key::Number(1.0), Value::Array(a));
    let copy = run(&mut e, 0x1561, &[arr(1)]) as u32;
    assert_ne!(copy, a);
    assert_eq!(entries(&e, copy)[1].1.data, Data::Array(copy));
    add_local(&mut e, 2, "c", f64::from(copy));
    assert_eq!(run(&mut e, 0x162f, &[arr(1), arr(2)]), 1.0);
    assert_eq!(array_repr(&mut e, a), b"[1, [...]]");
}

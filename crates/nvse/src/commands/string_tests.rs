//! Tests of the string commands: the helpers table by table, then each command through its
//! record.

use super::super::test_support::*;
use super::*;

const HEAP: u32 = 0x0280_0000;

/// Forms' classes for the fake cast, a bit each at `+0x7F0`, and where the class lies.
fn class(to: u32) -> (u32, u32) {
    match to {
        rtti::TES_OBJECT_REFR => (1, 0),
        RTTI_TES_MODEL => (2, 0x100),
        RTTI_TES_ICON | RTTI_TES_TEXTURE => (4, 0x200),
        rtti::TES_BIPED_MODEL_FORM => (8, 0x300),
        RTTI_TES_FACTION => (16, 0),
        rtti::TES_OBJECT_WEAP => (32, 0),
        RTTI_TES_FULL_NAME => (64, 0x18),
        RTTI_TES_GLOBAL => (128, 0),
        _ => (0, 0),
    }
}

fn cast(e: &mut Engine, a: &[u32]) -> Ret {
    let (bit, at) = class(a[3]);
    let is = a[0] != 0 && e.mem.u32(a[0] + 0x7f0) & bit != 0;
    Ret {
        eax: if is { a[0] + at } else { 0 },
        ..Ret::default()
    }
}

fn ret(v: u32) -> Ret {
    Ret {
        eax: v,
        ..Ret::default()
    }
}

/// The table, ASCII `toupper`/`tolower`, forms whose id is their address, a bump heap.
fn setup() -> Engine {
    let mut e = engine_with_table();
    e.map(HEAP, 0x10000);
    e.mem.set_u32(HEAP, HEAP + 0x10);
    e.register(0x00ec_43fb, cast);
    e.register(GAME_TOUPPER, |_, a| {
        ret(u32::from((a[0] as u8).to_ascii_uppercase()))
    });
    e.register(GAME_TOLOWER, |_, a| {
        ret(u32::from((a[0] as u8).to_ascii_lowercase()))
    });
    e.register(LOOKUP_FORM, |_, a| {
        ret(if (0x0200_0000..0x0201_0000).contains(&a[0]) {
            a[0]
        } else {
            0
        })
    });
    e.register(FORM_HEAP_ALLOCATE, |e, a| {
        let at = e.mem.u32(HEAP);
        e.mem.set_u32(HEAP, at + a[0].div_ceil(4) * 4);
        ret(at)
    });
    e.register(FORM_HEAP_FREE, |_, _| ret(0));
    e.mem.set_u32(SCRIPT + 0xc, 0x0500_0d00);
    e
}

/// A form at `at` (its id its address) of the classes `bits`.
fn form(e: &mut Engine, at: u32, kind: u8, bits: u32) -> u32 {
    e.mem.set_u8(at + 4, kind);
    e.mem.set_u32(at + 0xc, at);
    e.mem.set_u32(at + 0x7f0, bits);
    at
}

/// A guest `String` at `at` holding `text` (its buffer just after it).
fn bs(e: &mut Engine, at: u32, text: &str) {
    let data = at + 0x40;
    e.mem.write(data, text.as_bytes());
    e.mem.set_u8(data + text.len() as u32, 0);
    e.mem.set_u32(at, data);
    e.mem.set_u16(at + 4, text.len() as u16);
    e.mem.set_u16(at + 6, text.len() as u16);
}

fn text_of(e: &Engine, id: f64) -> Vec<u8> {
    vars::string(e, id as u32).expect("a string")
}

fn s_id(e: &mut Engine, text: &str) -> u32 {
    vars::add_string(e, 5, text.as_bytes(), false)
}

fn local_of(e: &mut Engine, id: u16, value: f64) -> Vec<u8> {
    add_local(e, id, &format!("v{id}"), value);
    local(id)
}

/// The UCRT's `printf` on numbers as `ExtractFormattedString` hands it: each case's format and
/// its doubles.
#[test]
fn printf_formats_as_the_ucrt() {
    let e = Engine::new();
    let list = |vs: &[f64]| vs.iter().flat_map(|v| v.to_le_bytes()).collect::<Vec<u8>>();
    let cases: &[(&str, &[f64], &str)] = &[
        ("%g", &[3.0], "3"),
        ("%g items", &[0.1], "0.1 items"),
        ("%.2f", &[1.005], "1.00"),
        ("%.0f", &[0.5], "0"),
        ("%.0f", &[1.5], "2"),
        ("%5.1f|", &[-2.25], " -2.2|"),
        ("%f", &[1.0 / 3.0], "0.333333"),
        ("%e", &[12345.678], "1.234568e+04"),
        ("%E", &[0.0], "0.000000E+00"),
        ("%g", &[1e-5], "1e-05"),
        ("%g", &[123456789.0], "1.23457e+08"),
        ("%g", &[100000.0], "100000"),
        ("%g", &[1e6], "1e+06"),
        ("%#g", &[1.0], "1.00000"),
        ("%G", &[1e-10], "1E-10"),
        ("%08.3f", &[1.23456], "0001.235"),
        ("%-6g|", &[1.5], "1.5   |"),
        ("%+g % g", &[2.0, 2.0], "+2  2"),
        ("%g %g", &[f64::INFINITY, f64::NEG_INFINITY], "inf -inf"),
        ("%g", &[f64::from_bits(0xfff8_0000_0000_0000)], "-nan(ind)"),
        ("%f", &[f64::from_bits(0x7ff8_0000_0000_0000)], "nan"),
        ("%a", &[1.0], "0x1.0000000000000p+0"),
        ("%.1a", &[1.5], "0x1.8p+0"),
        ("%A", &[-0.5], "-0X1.0000000000000P-1"),
        ("%d|%d", &[1.0], "0|1072693248"),
        (
            "%5.3d|%x|%#X",
            &[f64::from_bits(0x0000_00ff_0000_0007)],
            "  007|ff|0",
        ),
        (
            "%llX",
            &[f64::from_bits(0xdead_beef_0000_0001)],
            "DEADBEEF00000001",
        ),
        ("%08llX", &[f64::from_bits(255)], "000000FF"),
        ("%c%c", &[f64::from_bits(0x42_0000_0041)], "AB"),
        ("%s|%5.2s|", &[0.0], "(null)|   (n|"),
        ("%%%n tail", &[], ""),
        (
            "%.*f",
            &[f64::from_bits(2 | (0x4004_0000u64 << 32)), 0.0],
            "0.00",
        ),
    ];
    for (format, args, want) in cases {
        let got = ucrt_format(&e, format.as_bytes(), &list(args));
        assert_eq!(String::from_utf8_lossy(&got), *want, "{format}");
    }
}

/// xNVSE's own specifiers, through `sv_Construct` (each case: the format, its arguments, the
/// text).
#[test]
fn format_strings_expand_xnvses_specifiers() {
    let mut e = setup();
    let abc = s_id(&mut e, "abc%d");
    let cases: Vec<(&str, Vec<Vec<u8>>, &str)> = vec![
        ("%g items", vec![z(3.0)], "3 items"),
        ("100%%", vec![], "100%"),
        ("plain %r%q%e", vec![], "plain \n\""),
        ("%x|%x4", vec![z(255.0), z(255.0)], "FF|00FF"),
        ("%X", vec![z(-1.0)], "FFFFFFFFFFFFFFFF"),
        ("%a%a", vec![z(72.0), z(105.0)], "Hi"),
        ("%z!", vec![z(f64::from(abc))], "abc%d!"),
        ("x%{hidden %g%}y", vec![z(0.0), z(5.0)], "xy"),
        ("x%{shown %g%}y", vec![z(1.0), z(5.0)], "xshown 5y"),
        ("%.1f%%", vec![n(2)], "2.0%"),
        ("%g", vec![], ""),
        ("%i", vec![local_of(&mut e, 9, 0.0)], "00000000"),
    ];
    for (format, args, want) in cases {
        let mut all = vec![s(format)];
        all.extend(args);
        let (ok, id) = call(&mut e, 0x156b, 0, &all);
        assert_eq!(ok, 1);
        assert_eq!(String::from_utf8_lossy(&text_of(&e, id)), want, "{format}");
    }
}

/// A replacement case: the text, what to find, what to put, the start, case-sensitive, how many,
/// the text after and the count.
type Replace<'a> = (&'a str, &'a str, &'a str, u32, bool, u32, &'a str, u32);

/// `StringVar`'s operations, case by case: (text, sub, start, count, case) and the answers.
#[test]
fn string_operations_follow_stringvar() {
    let mut e = setup();
    let finds: &[(&str, &str, u32, u32, bool, u32)] = &[
        ("Hello World", "o", 0, u32::MAX, true, 4),
        ("Hello World", "o", 5, u32::MAX, true, 7),
        ("Hello World", "WORLD", 0, u32::MAX, false, 6),
        ("Hello World", "WORLD", 0, u32::MAX, true, u32::MAX),
        ("Hello World", "", 3, u32::MAX, true, 3),
        ("Hello", "l", 9, u32::MAX, true, u32::MAX),
        ("Hello", "lo", 0, 4, true, u32::MAX),
    ];
    for &(text, sub, start, num, case, want) in finds {
        let got = sv_find_in(&mut e, text.as_bytes(), sub.as_bytes(), start, num, case);
        assert_eq!(got, want, "find {sub:?} in {text:?}");
    }
    let counts: &[(&str, &str, u32, bool, u32)] = &[
        ("banana", "a", 0, true, 3),
        ("banana", "ana", 0, true, 1),
        ("BaNaNa", "a", 0, false, 3),
        ("banana", "", 0, true, 0),
        ("banana", "a", 6, true, 0),
    ];
    for &(text, sub, start, case, want) in counts {
        let got = sv_count_in(
            &mut e,
            text.as_bytes(),
            sub.as_bytes(),
            start,
            u32::MAX,
            case,
        );
        assert_eq!(got, want, "count {sub:?} in {text:?}");
    }
    let replaces: &[Replace] = &[
        (r"a.b.c", ".", "-", 0, true, u32::MAX, "a-b-c", 2),
        (r"a.b.c", ".", "-", 2, true, u32::MAX, "a.b-c", 1),
        (r"a.b.c", ".", "", 0, true, 1, "ab.c", 1),
        (r"AbAB", "ab", "x", 0, false, u32::MAX, "xx", 2),
        (r"end.", ".", "!!", 0, true, u32::MAX, "end!!", 1),
        (r"ab", "", "X", 0, true, 3, "XXXab", 3),
        (r"", "", "X", 0, true, u32::MAX, "", 0),
    ];
    for &(text, find, with, start, case, many, want, count) in replaces {
        let mut d = text.as_bytes().to_vec();
        let n = sv_replace_in(
            &mut e,
            &mut d,
            find.as_bytes(),
            with.as_bytes(),
            start,
            u32::MAX,
            case,
            many,
        );
        assert_eq!(
            (String::from_utf8_lossy(&d).into_owned(), n),
            (want.to_string(), count),
            "{text:?}"
        );
    }
    let mut d = b"abcdef".to_vec();
    sv_erase_in(&mut d, 1, 2);
    assert_eq!(d, b"adef");
    sv_erase_in(&mut d, 2, u32::MAX);
    assert_eq!(d, b"ad");
    assert_eq!(sv_substring_of(b"abcdef", 2, 3), b"cde");
    assert_eq!(sv_substring_of(b"abcdef", 4, 9), b"ef");
    assert_eq!(sv_substring_of(b"abc", 3, 1), b"");
    let mut d = b"ad".to_vec();
    sv_insert_at(&mut d, b"bc", 1);
    sv_insert_at(&mut d, b"!", 4);
    sv_insert_at(&mut d, b"?", 9);
    assert_eq!(d, b"abcd!");
    assert_eq!(trimmed(b" \t x y \r\n"), b"x y");
    assert_eq!(trimmed(b"   "), b"");
    assert_eq!(
        tokens(b",,a,b;;c,", b",;"),
        vec![b"a".to_vec(), b"b".to_vec(), b"c".to_vec()]
    );
    assert_eq!(tokens(b"abc", b""), vec![b"abc".to_vec()]);
    assert!(tokens(b"", b",").is_empty());
    let pieces = |v: Vec<Vec<u8>>| {
        v.into_iter()
            .map(|p| String::from_utf8(p).unwrap())
            .collect::<Vec<_>>()
    };
    assert_eq!(
        pieces(split_on(b"a&+b&+&+c&+", b"&+")),
        ["a", "b", "", "c", ""]
    );
    assert_eq!(pieces(split_on(b"ab", b"")), ["a", "b"]);
    assert!(split_on(b"", b",").is_empty());
    let types: &[(u32, u32)] = &[
        (b'a' as u32, ALPHABETIC | PRINTABLE),
        (b'Q' as u32, ALPHABETIC | PRINTABLE | UPPERCASE),
        (b'7' as u32, DIGIT | PRINTABLE),
        (b'!' as u32, PUNCTUATION | PRINTABLE),
        (b' ' as u32, PRINTABLE),
        (9, 0),
        (0xe9, 0),
        (0x141, ALPHABETIC | PRINTABLE | UPPERCASE),
    ];
    for &(c, want) in types {
        assert_eq!(char_type(c), want, "{c:#x}");
    }
}

/// The `sv_*` commands on a string variable read from a local, through their records.
#[test]
fn sv_commands_edit_string_variables() {
    let mut e = setup();
    let id = s_id(&mut e, "Hello World");
    let v = local_of(&mut e, 1, f64::from(id));
    assert_eq!(call(&mut e, 0x156e, 0, std::slice::from_ref(&v)).1, 11.0);
    assert_eq!(call(&mut e, 0x156e, 0, &[n(999)]).1, -1.0);
    assert_eq!(
        call(&mut e, 0x156d, 0, &[s("hello world"), v.clone()]).1,
        0.0
    );
    assert_eq!(
        call(&mut e, 0x156d, 0, &[s("hello world"), v.clone(), n(1)]).1,
        1.0
    );
    assert_eq!(call(&mut e, 0x156d, 0, &[s("A"), v.clone()]).1, -1.0);
    assert_eq!(call(&mut e, 0x156d, 0, &[s("x"), n(999)]).1, -2.0);
    assert_eq!(call(&mut e, 0x1574, 0, &[s("world"), v.clone()]).1, 6.0);
    assert_eq!(
        call(
            &mut e,
            0x1574,
            0,
            &[s("world"), v.clone(), n(0), n(-1), n(1)]
        )
        .1,
        -1.0
    );
    assert_eq!(call(&mut e, 0x1573, 0, &[s("l"), v.clone()]).1, 3.0);
    assert_eq!(
        call(&mut e, 0x1576, 0, &[v.clone(), n(4)]).1,
        f64::from(b'o')
    );
    assert_eq!(call(&mut e, 0x1576, 0, &[v.clone(), n(40)]).1, -1.0);
    // A replacement needs the separator.
    assert_eq!(call(&mut e, 0x1575, 0, &[s("World"), v.clone()]).1, -1.0);
    assert_eq!(call(&mut e, 0x1575, 0, &[s("o|0"), v.clone()]).1, 2.0);
    assert_eq!(text_of(&e, f64::from(id)), b"Hell0 W0rld");
    call(&mut e, 0x156f, 0, &[v.clone(), n(4), n(2)]);
    assert_eq!(text_of(&e, f64::from(id)), b"HellW0rld");
    call(&mut e, 0x1572, 0, &[s("o, "), v.clone(), n(4)]);
    assert_eq!(text_of(&e, f64::from(id)), b"Hello, W0rld");
    let (_, sub) = call(&mut e, 0x1570, 0, &[v.clone(), n(7)]);
    assert_eq!(text_of(&e, sub), b"W0rld");
    let (_, none) = call(&mut e, 0x1570, 0, &[n(999)]);
    assert_eq!(none, 0.0, "no string: nothing assigned");
    call(
        &mut e,
        0x156c,
        0,
        &[s("%g and %g"), z(1.0), z(2.5), v.clone()],
    );
    assert_eq!(text_of(&e, f64::from(id)), b"1 and 2.5");
    let num = s_id(&mut e, "  12.5e1xyz");
    let nv = local_of(&mut e, 2, f64::from(num));
    assert_eq!(call(&mut e, 0x1571, 0, std::slice::from_ref(&nv)).1, 125.0);
    assert_eq!(call(&mut e, 0x1571, 0, &[nv, n(5)]).1, 50.0);
}

/// The character commands and the number-to-text commands.
#[test]
fn character_and_number_commands() {
    let mut e = setup();
    let cases: &[(u32, i32, f64)] = &[
        (0x157b, b'a' as i32, 1.0),
        (0x157b, b'1' as i32, 0.0),
        (0x157c, b'5' as i32, 1.0),
        (0x157d, b'~' as i32, 1.0),
        (0x157d, 0x7f, 0.0),
        (0x157e, b'.' as i32, 1.0),
        (0x157f, b'a' as i32, 0.0),
        (0x157f, b'A' as i32, 1.0),
        (0x1581, b'q' as i32, b'Q' as f64),
        (0x1582, b'Q' as i32, b'q' as f64),
    ];
    for &(op, arg, want) in cases {
        assert_eq!(call(&mut e, op, 0, &[n(arg)]).1, want, "{op:#x} {arg}");
    }
    assert_eq!(call(&mut e, 0x1580, 0, &[s("A")]).1, 65.0);
    assert_eq!(call(&mut e, 0x1580, 0, &[s("AB")]).1, -1.0);
    let strings: &[(u32, Vec<Vec<u8>>, &str)] = &[
        (0x1583, vec![n(66)], "B"),
        (0x1583, vec![n(300)], ""),
        (0x1584, vec![n(255)], "000000FF"),
        (0x1584, vec![n(255), n(2)], "FF"),
        (0x1656, vec![n(255), n(4), n(1)], "0x00FF"),
        (0x1656, vec![n(-1), n(20)], "FFFFFFFF"),
        (0x1655, vec![n(5), n(4), n(1)], "0b0101"),
        (0x1655, vec![n(5)], "00000000000000000000000000000101"),
    ];
    for (op, args, want) in strings {
        let (_, id) = call(&mut e, *op, 0, args);
        assert_eq!(String::from_utf8_lossy(&text_of(&e, id)), *want, "{op:#x}");
    }
}

/// A `set ... to` line: the variable's string takes the text and stops being temporary; a
/// variable with no string gets a new, kept string.
#[test]
fn a_set_statement_keeps_its_variables_string() {
    let mut e = setup();
    let id = vars::add_string(&mut e, 5, b"old", true);
    let local = add_local(&mut e, 3, "s", f64::from(id));
    // `set s to sv_Construct "new"`: the statement, the variable, the expression's length, a
    // space, the command token.
    let mut line = vec![0x15, 0, 0, 0, b's', 3, 0, 0, 0, b' ', b'X'];
    line.extend_from_slice(&0x156bu16.to_le_bytes());
    let mut args = 1u16.to_le_bytes().to_vec();
    args.extend(s("new"));
    line.extend_from_slice(&(args.len() as u16).to_le_bytes());
    line.extend(args);
    let at = DATA + 0x100;
    e.mem.write(at, &line);
    // The variable's info in the script's list: id 3 at `+0`.
    let record = crate::table::record_of(&e, 0x156b);
    let (params, execute) = (e.mem.u32(record + 0x14), e.mem.u32(record + 0x18));
    e.mem.set_u32(OFFSET, 5);
    let x = at + 10;
    e.call(execute, &[params, x, 0, 0, SCRIPT, LIST, RESULT, OFFSET]);
    assert_eq!(e.mem.f64(RESULT), f64::from(id));
    assert_eq!(text_of(&e, f64::from(id)), b"new");
    vars::clean_strings(&mut e);
    assert!(vars::string(&e, id).is_some(), "no longer temporary");
    assert_eq!(local_value(&e, local), f64::from(id));
    // A variable holding no string: a new one, not temporary.
    e.mem.set_f64(local + 8, 0.0);
    e.call(execute, &[params, x, 0, 0, SCRIPT, LIST, RESULT, OFFSET]);
    let fresh = e.mem.f64(RESULT);
    vars::clean_strings(&mut e);
    assert_eq!(text_of(&e, fresh), b"new");
    // `sv_Destruct` in a `set` line deletes the variable's string.
    e.mem.set_f64(local + 8, fresh);
    e.mem.set_u16(at + 11, 0x156a);
    let record = crate::table::record_of(&e, 0x156a);
    let (params, execute) = (e.mem.u32(record + 0x14), e.mem.u32(record + 0x18));
    e.call(execute, &[params, x, 0, 0, SCRIPT, LIST, RESULT, OFFSET]);
    assert!(vars::string(&e, fresh as u32).is_none());
}

// ---- expression commands ----

fn var(var_type: u8, idx: u16) -> Vec<u8> {
    let mut b = vec![b'V', var_type, 0, 0];
    b.extend_from_slice(&idx.to_le_bytes());
    b
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
/// The arguments of an expression command: the count and each one-token expression.
fn exprs(tokens: &[Vec<u8>]) -> Vec<u8> {
    let mut b = vec![tokens.len() as u8];
    for t in tokens {
        b.extend_from_slice(&(t.len() as u16 + 2).to_le_bytes());
        b.extend_from_slice(t);
    }
    b
}

fn run(e: &mut Engine, opcode: u32, this_obj: u32, tokens: &[Vec<u8>]) -> f64 {
    let bytes = exprs(tokens);
    e.mem.set_u16(DATA, opcode as u16);
    e.mem.set_u16(DATA + 2, bytes.len() as u16);
    e.mem.write(DATA + 4, &bytes);
    e.mem.set_u32(OFFSET, 4);
    e.mem.set_f64(RESULT, 0.0);
    let record = crate::table::record_of(e, opcode);
    let (params, execute) = (e.mem.u32(record + 0x14), e.mem.u32(record + 0x18));
    e.call(
        execute,
        &[params, DATA, this_obj, 0, SCRIPT, LIST, RESULT, OFFSET],
    );
    e.mem.f64(RESULT)
}

fn array_texts(e: &Engine, id: f64) -> Vec<String> {
    let a = vars::array(e, id as u32).expect("an array");
    match &a.elements {
        vars::Container::Array(v) => v
            .iter()
            .map(|x| match &x.data {
                vars::Data::Str(s) => String::from_utf8_lossy(s).into_owned(),
                other => format!("{other:?}"),
            })
            .collect(),
        _ => panic!("not packed"),
    }
}

/// `sv_Split`, `ToNumber`, `sv_Trim`, the case changes, `sv_Percentify`, `sv_Destruct` and
/// `GetRawFormIDString` on the evaluator's arguments.
#[test]
fn expression_string_commands() {
    let mut e = setup();
    let arr = run(&mut e, 0x1577, 0, &[text("a, b,,c"), text(", ")]);
    assert_eq!(array_texts(&e, arr), ["a", "b", "c"]);
    let arr = run(&mut e, 0x1577, 0, &[text("a, b,,c"), text(","), num(1.0)]);
    assert_eq!(array_texts(&e, arr), ["a", " b", "", "c"]);
    let arr = run(&mut e, 0x1577, 0, &[text("x")]);
    assert!(array_texts(&e, arr).is_empty(), "too few: an empty array");
    let numbers: &[(&str, Option<f64>, f64)] = &[
        ("12.5", None, 12.5),
        ("0x1F", None, 31.0),
        ("ff", Some(1.0), 255.0),
        ("0b101", None, 5.0),
        ("junk", None, 0.0),
    ];
    for &(t, hex, want) in numbers {
        let mut toks = vec![text(t)];
        if let Some(h) = hex {
            toks.push(num(h));
        }
        assert_eq!(run(&mut e, 0x1585, 0, &toks), want, "{t}");
    }
    assert_eq!(run(&mut e, 0x1585, 0, &[num(7.0)]), 7.0);
    let id = s_id(&mut e, "  pad me \t");
    add_local(&mut e, 4, "sv", f64::from(id));
    assert_eq!(run(&mut e, 0x1640, 0, &[var(2, 4)]), 1.0);
    assert_eq!(text_of(&e, f64::from(id)), b"pad me");
    assert_eq!(run(&mut e, 0x1640, 0, &[num(3.0)]), 0.0);
    let up = run(&mut e, 0x1579, 0, &[text("MiXeD 1")]);
    assert_eq!(text_of(&e, up), b"MIXED 1");
    let down = run(&mut e, 0x157a, 0, &[text("MiXeD 1")]);
    assert_eq!(text_of(&e, down), b"mixed 1");
    let pc = run(&mut e, 0x1578, 0, &[text("50% of %s")]);
    assert_eq!(text_of(&e, pc), b"50%% of %%s");
    let raw = run(&mut e, 0x158a, 0, &[var(4, 4)]);
    assert_eq!(
        text_of(&e, raw),
        format!("{:08X}", (f64::from(id)).to_bits() as u32).as_bytes()
    );
    run(&mut e, 0x156a, 0, &[var(2, 4)]);
    assert!(vars::string(&e, id).is_none());
    assert_eq!(local_value(&e, 0x0300_6000 + 4 * 0x20), 0.0);
}

/// Names, form ids, key names, actor values, the game's folder.
#[test]
fn name_and_id_getters() {
    let mut e = setup();
    let item = form(&mut e, 0x0200_1000, 0x28, 64);
    bs(&mut e, item + 0x18 + 4, "Stimpak");
    let unnamed = form(&mut e, 0x0200_2000, 0x28, 64);
    let reference = form(&mut e, 0x0200_3000, 0x3a, 1);
    e.mem.set_u32(reference + 0x20, item);
    set_script_ref(&mut e, 1, item);
    set_script_ref(&mut e, 2, unnamed);
    let name = |e: &mut Engine, this: u32, args: &[Vec<u8>]| {
        let (_, id) = call(e, 0x1587, this, args);
        String::from_utf8_lossy(&text_of(e, id)).into_owned()
    };
    assert_eq!(name(&mut e, 0, &[r(1)]), "Stimpak");
    assert_eq!(name(&mut e, 0, &[r(2)]), "<no name>");
    assert_eq!(name(&mut e, reference, &[]), "Stimpak");
    assert_eq!(name(&mut e, 0, &[]), "");
    let (_, id) = call(&mut e, 0x1589, reference, &[]);
    assert_eq!(text_of(&e, id), b"02003000");
    // Key names: a `ControlName` for key 0x1E, the mouse's first button for 255 and 256.
    e.map(KEY_NAMES & !0xfff, 0x2000);
    let control = 0x0200_4000;
    e.mem.set_u32(control + 4, 0x0200_4100);
    e.mem.write(0x0200_4100, b"A\0");
    e.mem.set_u32(KEY_NAMES + 4 * 0x1e, control);
    e.mem.set_u32(MOUSE_BUTTON_NAMES, control);
    let key = |e: &mut Engine, code: i32| {
        let (_, id) = call(e, 0x1588, 0, &[n(code)]);
        String::from_utf8_lossy(&text_of(e, id)).into_owned()
    };
    assert_eq!(key(&mut e, 0x1e), "A");
    assert_eq!(key(&mut e, 2), "<no key>");
    assert_eq!(key(&mut e, 255), "A");
    assert_eq!(key(&mut e, 264), "WheelUp");
    assert_eq!(key(&mut e, 600), "<no key>");
    // Actor values: code 5's info names.
    let info = 0x0200_5000;
    e.mem.set_u32(ACTOR_VALUES + 4 * 5, info);
    e.mem.set_u32(info + 0x38, 0x0200_5100);
    e.mem.write(0x0200_5100, b"Agility\0");
    e.mem.set_u32(info + 0x1c, 0x0200_5200);
    e.mem.write(0x0200_5200, b"AGI\0");
    let av = |e: &mut Engine, op: u32, args: &[Vec<u8>]| {
        let (_, id) = call(e, op, 0, args);
        String::from_utf8_lossy(&text_of(e, id)).into_owned()
    };
    assert_eq!(av(&mut e, 0x158d, &[n(5)]), "Agility");
    assert_eq!(av(&mut e, 0x158d, &[n(5), n(1)]), "AGI");
    assert_eq!(av(&mut e, 0x158d, &[n(5), n(2)]), "Unknown");
    assert_eq!(av(&mut e, 0x158d, &[n(76)]), "Unknown");
    e.kernel.current_directory = b"C:\\Games\\FNV".to_vec();
    let (_, dir) = call(&mut e, 0x158b, 0, &[]);
    assert_eq!(text_of(&e, dir), b"C:\\Games\\FNV\\");
}

/// Model, icon, texture, biped, scope paths and faction ranks: read and set.
#[test]
fn form_paths_and_rank_names() {
    let mut e = setup();
    let armor = form(&mut e, 0x0200_6000, 0x18, 2 | 4 | 8);
    bs(&mut e, armor + 0x104, "armor.nif");
    bs(&mut e, armor + 0x204, "armor.dds");
    bs(&mut e, armor + 0x330, "female.nif");
    bs(&mut e, armor + 0x39c, "icon_f.dds");
    set_script_ref(&mut e, 1, armor);
    let get = |e: &mut Engine, op: u32, args: &[Vec<u8>]| {
        let (_, id) = call(e, op, 0, args);
        String::from_utf8_lossy(&text_of(e, id)).into_owned()
    };
    assert_eq!(get(&mut e, 0x158e, &[r(1)]), "armor.nif");
    assert_eq!(get(&mut e, 0x158f, &[r(1)]), "armor.dds");
    assert_eq!(get(&mut e, 0x1592, &[r(1)]), "armor.dds");
    assert_eq!(get(&mut e, 0x1590, &[n(1), r(1)]), "female.nif");
    assert_eq!(get(&mut e, 0x1590, &[n(0), r(1)]), "");
    assert_eq!(get(&mut e, 0x1591, &[n(1), r(1)]), "icon_f.dds");
    call(&mut e, 0x1593, 0, &[s("new%g.nif"), z(2.0), r(1)]);
    assert_eq!(get(&mut e, 0x158e, &[r(1)]), "new2.nif");
    call(&mut e, 0x1596, 0, &[s("male.nif"), n(0), r(1)]);
    assert_eq!(get(&mut e, 0x1590, &[n(0), r(1)]), "male.nif");
    let reference = form(&mut e, 0x0200_7000, 0x3a, 1);
    e.mem.set_u32(reference + 0x20, armor);
    assert_eq!(run(&mut e, 0x166d, reference, &[text("set.nif")]), 1.0);
    assert_eq!(get(&mut e, 0x158e, &[r(1)]), "set.nif");
    assert_eq!(run(&mut e, 0x1597, 0, &[text("tex.dds")]), 0.0, "no form");
    // A weapon with a scope.
    let gun = form(&mut e, 0x0200_8000, 0x28, 32);
    e.mem.set_u8(gun + 0x100, 4);
    bs(&mut e, gun + 0x1ec, "scope.nif");
    set_script_ref(&mut e, 2, gun);
    assert_eq!(get(&mut e, 0x15b3, &[r(2)]), "scope.nif");
    call(&mut e, 0x15b4, 0, &[s("other.nif"), r(2)]);
    assert_eq!(get(&mut e, 0x15b3, &[r(2)]), "other.nif");
    e.mem.set_u8(gun + 0x100, 0);
    assert_eq!(get(&mut e, 0x15b3, &[r(2)]), "");
    // A faction with two ranks.
    let faction = form(&mut e, 0x0200_9000, TYPE_FACTION, 16);
    let (r0, r1) = (0x0200_9400, 0x0200_9800);
    bs(&mut e, r0, "Recruit");
    bs(&mut e, r1, "Captain");
    bs(&mut e, r1 + 8, "Captaine");
    e.mem.set_u32(faction + 0x3c, r0);
    e.mem.set_u32(faction + 0x40, 0x0200_9a00);
    e.mem.set_u32(0x0200_9a00, r1);
    set_script_ref(&mut e, 3, faction);
    assert_eq!(get(&mut e, 0x1598, &[r(3), n(1)]), "Captain");
    assert_eq!(get(&mut e, 0x1598, &[r(3), n(1), n(1)]), "Captaine");
    assert_eq!(get(&mut e, 0x1598, &[r(3), n(2)]), "");
    call(&mut e, 0x1599, 0, &[s("Boss"), r(3), n(0)]);
    assert_eq!(get(&mut e, 0x1598, &[r(3), n(0)]), "Boss");
}

/// Game settings by name (`NiTMap::Lookup`) and INI settings by name without case.
#[test]
fn settings_are_read_and_set() {
    let mut e = setup();
    e.map(GAME_SETTINGS & !0xfff, 0x1000);
    e.map(INI_SETTINGS & !0xfff, 0x1000);
    e.map(INI_PREFS & !0xfff, 0x1000);
    let setting = |e: &mut Engine, at: u32, name: &str, value: &str| {
        e.mem.write(at + 0x20, name.as_bytes());
        e.mem.set_u8(at + 0x20 + name.len() as u32, 0);
        e.mem.write(at + 0x60, value.as_bytes());
        e.mem.set_u8(at + 0x60 + value.len() as u32, 0);
        e.mem.set_u32(at + 8, at + 0x20);
        e.mem.set_u32(at + 4, at + 0x60);
        at
    };
    let gmst = setting(&mut e, 0x0200_a000, "sGreeting", "Hello");
    let gmst_num = setting(&mut e, 0x0200_a100, "fNumber", "");
    let collection = 0x0200_b000;
    e.mem.set_u32(GAME_SETTINGS, collection);
    e.mem.set_u32(0x0200_bf00, gmst);
    e.mem.set_u32(0x0200_bf04, gmst_num);
    e.register(NI_TMAP_LOOKUP, |e, a| {
        let name = guest_text(e, a[1]);
        let found = match name.as_slice() {
            b"sGreeting" => e.mem.u32(0x0200_bf00),
            b"fNumber" => e.mem.u32(0x0200_bf04),
            _ => 0,
        };
        if found != 0 {
            e.mem.set_u32(a[2], found);
        }
        ret(u32::from(found != 0))
    });
    let ini = setting(&mut e, 0x0200_c000, "sLanguage:General", "ENGLISH");
    let prefs = 0x0200_d000;
    e.mem.set_u32(INI_PREFS, prefs);
    e.mem.set_u32(INI_SETTINGS, 0x0200_e000);
    e.mem.set_u32(0x0200_e000 + 0x10c, ini);
    let get = |e: &mut Engine, op: u32, name: &str| {
        let (_, id) = call(e, op, 0, &[s(name)]);
        (
            id,
            vars::string(e, id as u32).map(|t| String::from_utf8_lossy(&t).into_owned()),
        )
    };
    assert_eq!(get(&mut e, 0x159a, "sGreeting").1.as_deref(), Some("Hello"));
    assert_eq!(get(&mut e, 0x159a, "fNumber").1.as_deref(), Some(""));
    assert_eq!(
        get(&mut e, 0x15c1, "SLANGUAGE:general").1.as_deref(),
        Some("ENGLISH")
    );
    assert_eq!(get(&mut e, 0x15c1, "sMissing:General").0, -1.0);
    assert_eq!(
        call(&mut e, 0x165d, 0, &[s("sLanguage:General"), s("GERMAN")]).1,
        1.0
    );
    assert_eq!(
        get(&mut e, 0x15c1, "sLanguage:General").1.as_deref(),
        Some("GERMAN")
    );
    assert_eq!(call(&mut e, 0x15c2, 0, &[s("sGreeting|Hi")]).1, 1.0);
    assert_eq!(get(&mut e, 0x159a, "sGreeting").1.as_deref(), Some("Hi"));
    assert_eq!(
        call(&mut e, 0x15c2, 0, &[s("sGreeting")]).1,
        0.0,
        "no separator"
    );
    assert_eq!(
        call(&mut e, 0x159b, 0, &[s("sLanguage:General|FRENCH")]).1,
        1.0
    );
    assert_eq!(
        get(&mut e, 0x15c1, "sLanguage:General").1.as_deref(),
        Some("FRENCH")
    );
}

/// `ValidateRegex`'s parse, pattern by pattern (0 for none, else xNVSE's error code), and the
/// command's result and message.
#[test]
fn regexes_are_checked_as_msvc_parses_them() {
    let cases: &[(&str, u8)] = &[
        (r"abc", 0),
        (r"a(b", 6),
        (r"a)b", 6),
        (r"[ab", 5),
        (r"a{2", 8),
        (r"a{2,1}", 8),
        (r"*a", 11),
        (r"a**", 11),
        (r"a\", 3),
        (r"[z-a]", 9),
        (r"\5", 4),
        (r"[[:foo:]]", 2),
        (r"[[:DIGIT:]x]", 0),
        (r"[[.foo.]]", 1),
        (r"[[.a.]]", 0),
        (r"a{x}", 8),
        (r"a]", 5),
        (r"a}", 7),
        (r"(?:a)", 0),
        (r"(?=a)b", 0),
        (r"(?<a)", 11),
        (r"\d+", 0),
        (r"a{,2}", 8),
        (r"(a)\1", 0),
        (r"(a\1)", 4),
        (r"[\w-z]", 9),
        (r"a|*b", 11),
        (r"()", 0),
        (r"a{99999999999}", 8),
        (r"\x4", 3),
        (r"\u00e9", 0),
        (r"\u0100", 3),
        (r"\c1", 3),
        (r"a{2,}?", 0),
    ];
    for &(pattern, want) in cases {
        let got = regex_check::check(pattern.as_bytes()).map_or_else(|e| e.code(), |()| 0);
        assert_eq!(got, want, "{pattern}");
    }
    let mut e = setup();
    let id = s_id(&mut e, "none");
    add_local(&mut e, 6, "err", f64::from(id));
    assert_eq!(run(&mut e, 0x1654, 0, &[text("a(b"), var(2, 6)]), 6.0);
    assert_eq!(
        text_of(&e, f64::from(id)),
        b"regex_error(error_paren): The expression contained mismatched ( and )."
    );
    assert_eq!(run(&mut e, 0x1654, 0, &[text("ok")]), 0.0);
}

/// What the real game with xNVSE 6.4.9 printed for the same format strings and numbers
/// (`measure/extenders/real-results/str3.txt`, run `str3`, 2026-11-08): `%d` takes 4 bytes of a
/// double, so the `%x` after it reads across two of them; `-1` as `%X` is all ones.
#[test]
#[allow(clippy::approx_constant)]
fn format_strings_match_the_real_game() {
    let mut e = setup();
    let cases: Vec<(&str, Vec<Vec<u8>>, &str)> = vec![
        (
            "p1 [%g] [%g] [%g] [%g] [%g]",
            vec![z(0.1), z(0.00001), z(123456789.0), z(100000.0), z(1e6)],
            "p1 [0.1] [1e-05] [1.23457e+08] [100000] [1e+06]",
        ),
        (
            "p2 [%e] [%E] [%.3e] [%G]",
            vec![z(12345.678), z(0.0), z(0.000123456), z(1e-10)],
            "p2 [] [] [1.235e+04] [0]",
        ),
        (
            "p3 [%.2f] [%.2f] [%.0f] [%.0f] [%.1f]",
            vec![z(2.675), z(0.125), z(0.5), z(2.5), z(0.25)],
            "p3 [2.67] [0.12] [0] [2] [0.2]",
        ),
        (
            "p4 [%5.1f] [%-6g] [%+g] [%08.3f] [%#g]",
            vec![z(-2.25), z(1.5), z(2.0), z(3.14159), z(1.0)],
            "p4 [ -2.2] [1.5   ] [+2] [0003.142] [1.00000]",
        ),
        (
            "p5 [%d] [%x] [%X] [%x4] [%x8]",
            vec![z(1.0), z(255.0), z(-1.0), z(255.0), z(3e9)],
            "p5 [0] [FF3FF00000] [FFFFFFFF00000000] [FFFFFFFFFF] [B2D05E0000000000]",
        ),
        (
            "p6 [%a%a] [%q] [%e]",
            vec![z(72.0), z(105.0)],
            "p6 [Hi] [\"] []",
        ),
        (
            "p7 [x%{hid %g%}y] [x%{shown %g%}y]",
            vec![z(0.0), z(5.0), z(1.0), z(6.0)],
            "p7 [xy] [xshown 6y]",
        ),
        (
            "p8 [%a] [%.3a] [%5.3d] [%c]",
            vec![z(1.0), z(1.5), z(7.0), z(65.0)],
            "p8 [\u{1}] [0x1.800p+0] [  000] []",
        ),
        (
            "p9 [%g] [%.1f]",
            vec![z(1e-300), z(-0.04)],
            "p9 [1e-300] [-0.0]",
        ),
    ];
    for (format, args, want) in cases {
        let mut all = vec![s(format)];
        all.extend(args);
        let (_, id) = call(&mut e, 0x156b, 0, &all);
        assert_eq!(String::from_utf8_lossy(&text_of(&e, id)), want, "{format}");
    }
    let big = ucrt_format(&e, b"%f", &1e300f64.to_le_bytes());
    assert!(big.starts_with(b"10000000000000000525047602552044202487044685811081591549158541"));
    assert!(big.ends_with(b"6865459400540160.000000"));
}

/// What the shipping xNVSE 6.4.9 DLL's `std::regex` answered for each pattern of
/// `regex_cases.txt` (its fuzzed and hand-made mismatches with the 14.44 source, run natively).
#[test]
fn regexes_match_the_shipping_dll() {
    let decode = |hex: &str| -> Vec<u8> {
        (0..hex.len() / 2)
            .map(|i| u8::from_str_radix(&hex[2 * i..2 * i + 2], 16).unwrap())
            .collect()
    };
    let mut checked = 0;
    for line in include_str!("regex_cases.txt").lines() {
        if line.starts_with('#') || line.is_empty() {
            continue;
        }
        let (hex, code) = line.split_once('|').unwrap();
        let pattern = decode(hex);
        let want: u8 = code.parse().unwrap();
        let got = regex_check::check(c_str(&pattern)).map_or_else(|e| e.code(), |()| 0);
        assert_eq!(got, want, "{:?}", String::from_utf8_lossy(&pattern));
        checked += 1;
    }
    assert!(checked > 400);
    // 1000 groups open at once, or 1000 capture groups in all, are too many.
    let deep = |n: usize, open: &str| {
        let mut p = open.repeat(n).into_bytes();
        p.push(b'a');
        p.extend(")".repeat(n).into_bytes());
        regex_check::check(&p).map_or_else(|e| e.code(), |()| 0)
    };
    assert_eq!(deep(999, "(?:"), 0);
    assert_eq!(deep(1000, "(?:"), 13);
    assert_eq!(deep(1000, "("), 13);
    assert_eq!(deep(5000, "(?="), 13);
    assert_eq!(regex_check::check("(a)".repeat(999).as_bytes()), Ok(()));
    assert_eq!(
        regex_check::check("(a)".repeat(1000).as_bytes()).map_err(|e| e.code()),
        Err(13)
    );
    // `(?x` is a bad repeat before any group is counted, past 999 groups too.
    let past = |tail: &str| {
        let p = "(a)".repeat(999) + tail;
        regex_check::check(p.as_bytes()).map_err(|e| e.code())
    };
    assert_eq!(past("(?x)"), Err(11));
    assert_eq!(past("(?"), Err(11));
    assert_eq!(past("(x)"), Err(13));
    assert_eq!(past("(?:x)"), Ok(()));
}

/// The printf's limits: what `snprintf` keeps (0x3FFD bytes) bounds every width and precision,
/// so none allocates more or panics; the UCRT's refusals give `""`, an unknown conversion its
/// character, `%0Nc`/`%0Ns` zeros, `%lc` the byte.
#[test]
fn printf_keeps_to_the_buffer_and_the_ucrts_rules() {
    let e = Engine::new();
    let list = |vs: &[u64]| vs.iter().flat_map(|v| v.to_le_bytes()).collect::<Vec<u8>>();
    let one = list(&[1.0f64.to_bits()]);
    for format in [
        "%.100000f",
        "%.70000e",
        "%.99999g",
        "%#.99999g",
        "%.70000a",
        "%2000000000d",
        "%-2000000000d|",
        "%.1000000d",
        "%*d",
        "%.*f",
    ] {
        let out = ucrt_format(&e, format.as_bytes(), &one);
        assert!(out.len() <= BUDGET, "{format}: {}", out.len());
    }
    let big = ucrt_format(&e, b"%.100000f", &one);
    assert!(big.starts_with(b"1.000") && big.len() == BUDGET);
    let cases: &[(&str, u64, &str)] = &[
        ("%1.0qd|%d", 0, "qd|0"),
        ("%qf|", 0, "qf|"),
        ("%Ld|%d", 0, ""),
        ("%wd", 5, ""),
        ("%n", 0, ""),
        ("[%05c]", 0x41, "[0000A]"),
        ("[%06s]", 0, "[(null)]"),
        ("[%08s]", 0, "[00(null)]"),
        ("[%lc]", 0xe9, "[\u{e9}]"),
        ("[%lc]", 0x141, "[]"),
        ("[%5Tc]", 0x41, "[    A]"),
        ("%Lf", 0x3ff8_0000_0000_0000, "1.500000"),
    ];
    for &(format, arg, want) in cases {
        let out = ucrt_format(&e, format.as_bytes(), &list(&[arg]));
        let want: Vec<u8> = want.chars().map(|c| c as u32 as u8).collect();
        assert_eq!(out, want, "{format}");
    }
    // A string outside the game's memory: the game stops (vanilla), or the text ends there.
    let stopped = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        ucrt_format(&e, b"a%sb", &list(&[0x10]))
    }));
    assert!(stopped.is_err());
    let mut guarded = Engine::new();
    super::super::array::set_fix(&mut guarded, super::super::array::NULL_GUARD, true);
    assert_eq!(ucrt_format(&guarded, b"a%sb", &list(&[0x10])), b"a");
}

/// The real UCRT's text where the port's differs (the review of #346 ran `vsnprintf` on a 32-bit
/// harness): format, arguments and text in hex, a case a line. A case marked `known` must still
/// differ, so its mark comes off when the formatter is fixed (issue #357); the others must match.
#[test]
fn printf_follows_the_ucrts_state_machine() {
    use super::super::array::{set_fix, NULL_GUARD};
    let unhex = |s: &str| {
        (0..s.len() / 2)
            .map(|i| u8::from_str_radix(&s[2 * i..2 * i + 2], 16).unwrap())
            .collect::<Vec<u8>>()
    };
    let mut e = Engine::new();
    set_fix(&mut e, NULL_GUARD, true);
    let (mut matched, mut known) = (0, 0);
    for line in include_str!("printf_cases.txt").lines() {
        if line.starts_with('#') {
            continue;
        }
        let mut parts = line.split('|');
        let format = unhex(parts.next().unwrap());
        let args = unhex(parts.next().unwrap());
        let want = unhex(parts.next().unwrap());
        let got = ucrt_format(&e, &format, &args);
        let name = String::from_utf8_lossy(&format);
        if parts.next() == Some("known") {
            assert_ne!(got, want, "{name} now matches: take its known mark off");
            known += 1;
        } else {
            assert_eq!(got, want, "{name}");
            matched += 1;
        }
    }
    assert!(matched + known > 1000);
}

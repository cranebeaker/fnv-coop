//! Tests of user functions and lambdas (I-7), through the commands' records, with the game's
//! functions they call stood in by small test functions.

use super::call;
use super::*;
use crate::commands::test_support::*;

/// Guest memory for function scripts, their event lists and the lambdas' allocations.
const AREA: u32 = 0x0400_0000;
const FN_SCRIPT: u32 = AREA;
const FN_DATA: u32 = AREA + 0x100;
/// A bump allocator for `FormHeap_Allocate` and the event lists the stand-ins make.
const HEAP: u32 = AREA + 0x8000;
const HEAP_TOP: u32 = AREA + 0x7ffc;
/// A fake vtable for made scripts: `SetFormID` at `+0x128`.
const VTABLE: u32 = AREA + 0x7000;
const SET_FORM_ID_FN: u32 = 0x7eff_e000;

fn alloc(e: &mut Engine, size: u32) -> u32 {
    let at = e.mem.u32(HEAP_TOP).max(HEAP);
    e.mem.set_u32(HEAP_TOP, at + size.next_multiple_of(8));
    for i in 0..size {
        e.mem.set_u8(at + i, 0);
    }
    at
}

fn form_heap(e: &mut Engine, a: &[u32]) -> Ret {
    let at = alloc(e, a[0]);
    Ret {
        eax: at,
        ..Ret::default()
    }
}

/// An event list with locals 1 and 2 (`CreateEventList`); counts how many were made at
/// `AREA + 0x7f00`.
fn create_event_list(e: &mut Engine, a: &[u32]) -> Ret {
    let list = alloc(e, 0x14);
    e.mem.set_u32(list, a[0]);
    let mut prev = 0;
    for id in [2u32, 1] {
        let local = alloc(e, 0x10);
        e.mem.set_u32(local, id);
        let node = alloc(e, 8);
        e.mem.set_u32(node, local);
        e.mem.set_u32(node + 4, prev);
        prev = node;
    }
    e.mem.set_u32(list + 0xc, prev);
    let n = e.mem.u32(AREA + 0x7f00);
    e.mem.set_u32(AREA + 0x7f00, n + 1);
    Ret {
        eax: list,
        ..Ret::default()
    }
}

fn destroy_event_list(e: &mut Engine, _: &[u32]) -> Ret {
    let n = e.mem.u32(AREA + 0x7f04);
    e.mem.set_u32(AREA + 0x7f04, n + 1);
    Ret::default()
}

/// `LookupFormByID` over the forms the test made (their id at `+0xC`).
fn lookup(e: &mut Engine, a: &[u32]) -> Ret {
    let id = a[0];
    for form in [FN_SCRIPT, SCRIPT] {
        if id != 0 && e.mem.u32(form + 0xc) == id {
            return Ret {
                eax: form,
                ..Ret::default()
            };
        }
    }
    let mut at = HEAP;
    while at < e.mem.u32(HEAP_TOP) {
        if e.mem.u32(at) == VTABLE && e.mem.u32(at + 0xc) == id && id != 0 {
            return Ret {
                eax: at,
                ..Ret::default()
            };
        }
        at += 8;
    }
    Ret::default()
}

fn upper(_: &mut Engine, a: &[u32]) -> Ret {
    Ret {
        eax: u32::from((a[0] as u8).to_ascii_uppercase()),
        ..Ret::default()
    }
}

fn nothing(_: &mut Engine, _: &[u32]) -> Ret {
    Ret::default()
}

/// `Script`'s constructor: the fake vtable and the script form type.
fn script_constructor(e: &mut Engine, a: &[u32]) -> Ret {
    e.mem.set_u32(a[0], VTABLE);
    e.mem.set_u8(a[0] + 4, 0x11);
    Ret {
        eax: a[0],
        ..Ret::default()
    }
}

fn set_form_id(e: &mut Engine, a: &[u32]) -> Ret {
    e.mem.set_u32(a[0] + 0xc, a[1]);
    Ret::default()
}

/// The function body the `Script::Execute` stand-in runs: `SetFunctionValue` over the
/// expression stored at the script's data `+0x40` (a count byte and one expression); counts
/// the runs at `AREA + 0x7f08`; with a nonzero word at `AREA + 0x7f0c`, calls the script again.
fn execute(e: &mut Engine, a: &[u32]) -> Ret {
    let (script, this_obj, list, containing) = (a[0], a[1], a[2], a[3]);
    let n = e.mem.u32(AREA + 0x7f08);
    e.mem.set_u32(AREA + 0x7f08, n + 1);
    if e.mem.u32(AREA + 0x7f0c) != 0 {
        call(e, Caller::internal(script, 0, 0, vec![1, 0]));
        return Ret::default();
    }
    let data = script_data(e, script);
    let record = crate::table::record_of(e, OP_SET_FUNCTION_VALUE);
    let (params, handler) = (e.mem.u32(record + 0x14), e.mem.u32(record + 0x18));
    e.with_stack(12, |e, f| {
        e.mem.set_u32(f.addr() + 8, 0x40);
        e.call(
            handler,
            &[
                params,
                data,
                this_obj,
                containing,
                script,
                list,
                f.addr(),
                f.addr() + 8,
            ],
        );
    });
    Ret::default()
}

fn setup() -> Engine {
    let mut e = engine_with_table();
    e.map(AREA, 0x2_0000);
    // `DataHandler`'s singleton word (none: the lambda path asks it only for a run-time parent).
    e.map(0x011c_3000, 0x1000);
    e.mem.set_u32(SCRIPT + 0xc, 0x0500_0d00);
    e.mem.set_u8(SCRIPT + 4, 0x11);
    e.register(0x00ec_a7f4, upper);
    e.register(0x0048_39c0, lookup);
    e.register(FORM_HEAP_ALLOCATE_FOR_TESTS, form_heap);
    e.register(CREATE_EVENT_LIST, create_event_list);
    e.register(EVENT_LIST_DESTROY, destroy_event_list);
    e.register(SCRIPT_EXECUTE, execute);
    e.register(0x005a_a0f0, script_constructor);
    for addr in [0x005a_b930, 0x005a_b7f0, 0x0046_4d30] {
        e.register(addr, nothing);
    }
    e.register(SET_FORM_ID_FN, set_form_id);
    e.mem.set_u32(VTABLE + 0x128, SET_FORM_ID_FN);
    // The block type table's entry 0x0D.
    e.mem.set_u32(FUNCTION_BLOCK_RECORD + 8, 0x0d);
    install(&mut e);
    e
}

const FORM_HEAP_ALLOCATE_FOR_TESTS: u32 = 0x0040_1000;

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
fn script_ref(idx: u16) -> Vec<u8> {
    let mut b = vec![b'R'];
    b.extend_from_slice(&idx.to_le_bytes());
    b
}
fn expression(tokens: &[Vec<u8>]) -> Vec<u8> {
    let body = tokens.concat();
    let mut b = (body.len() as u16 + 2).to_le_bytes().to_vec();
    b.extend(body);
    b
}

/// A function script: `scn`, `Begin Function` with `params`, then (at data `+0x40`) the
/// `SetFunctionValue` arguments the execute stand-in runs.
fn function_script(e: &mut Engine, at: u32, data: u32, id: u32, params: &[(u16, u8)], body: &[u8]) {
    let mut d = vec![0x1d, 0, 0, 0, 0x10, 0];
    let mut block = vec![0x0d, 0, 0, 0, 0, 0, 1, params.len() as u8];
    for (idx, ty) in params {
        block.extend_from_slice(&idx.to_le_bytes());
        block.push(*ty);
    }
    d.extend_from_slice(&(block.len() as u16).to_le_bytes());
    d.extend(block);
    d.resize(0x40, 0);
    d.extend_from_slice(body);
    e.mem.write(data, &d);
    e.mem.set_u32(at + 0xc, id);
    e.mem.set_u8(at + 4, 0x11);
    e.mem.set_u32(at + 0x30, data);
    e.mem.set_u32(at + 0x20, d.len() as u32);
}

/// Runs `opcode` with `bytes` as its argument data.
fn run(e: &mut Engine, opcode: u32, bytes: &[u8]) -> f64 {
    e.mem.set_u16(DATA, opcode as u16);
    e.mem.set_u16(DATA + 2, bytes.len() as u16);
    e.mem.write(DATA + 4, bytes);
    e.mem.set_u32(OFFSET, 4);
    e.mem.set_f64(RESULT, 0.0);
    let record = crate::table::record_of(e, opcode);
    let (params, execute) = (e.mem.u32(record + 0x14), e.mem.u32(record + 0x18));
    e.call(execute, &[params, DATA, 0, 0, SCRIPT, LIST, RESULT, OFFSET]);
    e.mem.f64(RESULT)
}

/// `Call fn 3.7 2.5` on `function (int a, float b) SetFunctionValue a + b`: the int argument
/// floored, the result 5.5; the kept event list made once and reset after, its locals 0.
#[test]
fn call_passes_arguments_and_returns_the_value() {
    let mut e = setup();
    let body = [vec![1u8], expression(&[var(1, 1), var(0, 2), vec![14]])].concat();
    function_script(
        &mut e,
        FN_SCRIPT,
        FN_DATA,
        0x0500_0e00,
        &[(1, 1), (2, 0)],
        &body,
    );
    set_script_ref(&mut e, 1, FN_SCRIPT);
    let call_args = [
        vec![1u8],
        expression(&[script_ref(1)]),
        vec![2u8],
        expression(&[num(3.7)]),
        expression(&[num(2.5)]),
    ]
    .concat();
    assert_eq!(run(&mut e, OP_CALL, &call_args), 5.5);
    assert_eq!(e.mem.u32(AREA + 0x7f00), 1, "one event list made");
    assert_eq!(run(&mut e, OP_CALL, &call_args), 5.5);
    assert_eq!(e.mem.u32(AREA + 0x7f00), 1, "the kept list is reused");
    assert_eq!(
        e.mem.u32(AREA + 0x7f04),
        0,
        "the kept list is not destroyed"
    );
    // The real game (`real-results/udf3.txt`): `-3.7` and `2.5` give -1.5 (the int floored).
    let negative = [
        vec![1u8],
        expression(&[script_ref(1)]),
        vec![2u8],
        expression(&[num(-3.7)]),
        expression(&[num(2.5)]),
    ]
    .concat();
    assert_eq!(run(&mut e, OP_CALL, &negative), -1.5);
    let info = manager(&mut e).infos.get(&FN_SCRIPT).cloned().unwrap();
    assert_eq!(
        info.params,
        [
            Param {
                idx: 1,
                var_type: 1
            },
            Param {
                idx: 2,
                var_type: 0
            }
        ]
    );
    let local = event_list_variable(&e, info.event_list, 1);
    assert_eq!(e.mem.f64(local + 8), 0.0, "reset after the call");
    assert!(manager(&mut e).stack.is_empty());
}

/// The `Begin Function` block's record: `Function` copied over block type 0x0D, its opcode
/// kept; its handler answers 1 for a function script, 0 for one whose header is bad.
#[test]
fn begin_function_runs_through_the_block_type_table() {
    let mut e = setup();
    let record = crate::table::record_of(&e, OP_FUNCTION);
    assert_eq!(
        e.mem.u32(FUNCTION_BLOCK_RECORD + 0x18),
        e.mem.u32(record + 0x18)
    );
    assert_eq!(e.mem.u32(FUNCTION_BLOCK_RECORD + 8), 0x0d);
    function_script(&mut e, FN_SCRIPT, FN_DATA, 0x0500_0e00, &[], &[]);
    let handler = e.mem.u32(FUNCTION_BLOCK_RECORD + 0x18);
    e.mem.set_f64(RESULT, 9.0);
    e.call(handler, &[0, FN_DATA, 0, 0, FN_SCRIPT, 0, RESULT, OFFSET]);
    assert_eq!(e.mem.f64(RESULT), 1.0);
}

/// A function calling itself stops at 30 calls deep; every frame is taken down again and the
/// nested calls' own event lists are destroyed.
#[test]
fn nesting_stops_at_thirty() {
    let mut e = setup();
    function_script(
        &mut e,
        FN_SCRIPT,
        FN_DATA,
        0x0500_0e00,
        &[(1, 1), (2, 0)],
        &[],
    );
    e.mem.set_u32(AREA + 0x7f0c, 1);
    call(&mut e, Caller::internal(FN_SCRIPT, 0, 0, vec![1, 0]));
    assert_eq!(e.mem.u32(AREA + 0x7f08), MAX_NEST_DEPTH);
    // The first call has the kept list; each nested one made and destroyed its own.
    assert_eq!(e.mem.u32(AREA + 0x7f00), MAX_NEST_DEPTH);
    assert_eq!(e.mem.u32(AREA + 0x7f04), MAX_NEST_DEPTH - 1);
    assert_eq!(manager(&mut e).depth, 0);
    assert!(manager(&mut e).stack.is_empty());
}

/// Internal callers: an `int` word, a `float`'s bits, a form's id; the alt kind reads the int
/// parameter from float bits, floored.
#[test]
fn internal_callers_convert_their_words() {
    let mut e = setup();
    function_script(
        &mut e,
        FN_SCRIPT,
        FN_DATA,
        0x0500_0e00,
        &[(1, 1), (2, 0)],
        &[],
    );
    e.mem.set_u32(AREA + 0x7f0c, 0);
    // An empty body (count 0): no result.
    let r = call(
        &mut e,
        Caller::internal(FN_SCRIPT, 0, 0, vec![(-7i32) as u32, 2.25f32.to_bits()]),
    );
    assert_eq!(r, None);
    let list = manager(&mut e).infos[&FN_SCRIPT].event_list;
    // Reset after the call: check through a frame that keeps them, the alt caller with a
    // second list (the kept one is busy while a call runs).
    assert_eq!(e.mem.f64(event_list_variable(&e, list, 1) + 8), 0.0);
    let mut seen = Vec::new();
    e.register(SCRIPT_EXECUTE, |e, a| {
        let l1 = event_list_variable(e, a[2], 1);
        let l2 = event_list_variable(e, a[2], 2);
        let v = [e.mem.f64(l1 + 8), e.mem.f64(l2 + 8)];
        e.mem.set_f64(AREA + 0x7e00, v[0]);
        e.mem.set_f64(AREA + 0x7e08, v[1]);
        Ret::default()
    });
    call(
        &mut e,
        Caller::internal(FN_SCRIPT, 0, 0, vec![(-7i32) as u32, 2.25f32.to_bits()]),
    );
    seen.push((e.mem.f64(AREA + 0x7e00), e.mem.f64(AREA + 0x7e08)));
    call(
        &mut e,
        Caller::Internal {
            script: FN_SCRIPT,
            this_obj: 0,
            containing: 0,
            args: vec![3.9f32.to_bits(), 0.5f32.to_bits()],
            alt: true,
            allow_surplus: false,
            event_types: None,
        },
    );
    seen.push((e.mem.f64(AREA + 0x7e00), e.mem.f64(AREA + 0x7e08)));
    assert_eq!(seen, [(-7.0, 2.25), (3.0, 0.5)]);
    // A surplus argument is an error unless allowed: the script does not run.
    e.mem.set_f64(AREA + 0x7e00, 99.0);
    call(&mut e, Caller::internal(FN_SCRIPT, 0, 0, vec![1, 2, 3]));
    assert_eq!(e.mem.f64(AREA + 0x7e00), 99.0);
}

/// A lambda (`F` token) assigned to a ref variable: a new script form with the next free id
/// of the parent's file, one per expression position and owner, its parent list the
/// evaluating one; calling it runs its single `SetFunctionValue` line on the parent's locals.
#[test]
fn lambdas_become_scripts_on_their_parent_list() {
    let mut e = setup();
    let r = add_local(&mut e, 1, "r", 0.0);
    let x = add_local(&mut e, 2, "x", 4.0);
    // The lambda's own data: scn, Begin Function (no parameters), SetFunctionValue x * 2, End.
    let sfv = [vec![1u8], expression(&[var(0, 2), num(2.0), vec![16]])].concat();
    let mut lambda_data = vec![0x1d, 0, 0, 0, 0x10, 0, 8, 0, 0x0d, 0, 0, 0, 0, 0, 1, 0];
    lambda_data.extend_from_slice(&(OP_SET_FUNCTION_VALUE as u16).to_le_bytes());
    lambda_data.extend_from_slice(&(sfv.len() as u16).to_le_bytes());
    lambda_data.extend(sfv);
    lambda_data.extend([0x11, 0, 0, 0]);
    let mut f = vec![b'F'];
    f.extend_from_slice(&(lambda_data.len() as u32).to_le_bytes());
    f.extend(lambda_data);
    let let_args = [vec![1u8], expression(&[var(4, 1), f.clone(), vec![0]])].concat();
    run(&mut e, 0x1539, &let_args);
    let id = e.mem.u32(r + 8);
    assert_eq!(id, 0x0500_0d01, "the next free id after the parent's");
    let lambda = lookup(&mut e, &[id]).eax;
    assert!(crate::lambda::is_lambda(&e, lambda));
    assert_eq!(crate::lambda::parent_event_list(&e, lambda), LIST);
    run(&mut e, 0x1539, &let_args);
    assert_eq!(e.mem.u32(r + 8), id, "one lambda per position and owner");
    // `Call` on it: the single line runs on the parent's locals.
    let call_args = [vec![1u8], expression(&[var(4, 1)]), vec![0u8]].concat();
    e.register(SCRIPT_EXECUTE, |_, _| {
        panic!("a single-line lambda runs its line directly")
    });
    assert_eq!(run(&mut e, OP_CALL, &call_args), 8.0);
    e.mem.set_f64(x + 8, 10.0);
    assert_eq!(run(&mut e, OP_CALL, &call_args), 20.0);
    // The parent list goes: the lambda has none any more.
    crate::lambda::mark_parent_deleted(&mut e, LIST);
    assert_eq!(crate::lambda::parent_event_list(&e, lambda), 0);
}

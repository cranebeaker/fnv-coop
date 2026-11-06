//! The bytes the exe hands on as they are: getters (`MOV AL`), and functions that pass on the
//! answer of a call. A 0 or 1 here would be a different byte for a caller that compares, stores
//! or counts it.

#![allow(dead_code)]

use crate::prelude::*;

/// A byte that is neither 0 nor 1, in a word whose upper bits a byte result drops.
const ANSWER: u32 = 0x142;
const BYTE: u8 = 0x42;
/// Where the test slots point.
const SLOT_TARGET: u32 = 0x7ffe_0000;

fn answer(_e: &mut Engine, _words: &[u32]) -> Ret {
    Ret {
        eax: ANSWER,
        ..Ret::default()
    }
}

/// A zeroed block whose every vtable slot answers [`ANSWER`].
fn object(e: &mut Engine) -> u32 {
    let table = e.mem.alloc(0x1000);
    for slot in 0..0x400 {
        e.mem.set_u32(table + 4 * slot, SLOT_TARGET);
    }
    e.register_double(SLOT_TARGET, answer);
    let block = e.mem.alloc(0x1000);
    e.mem.set_u32(block, table);
    block
}

/// The row's answer with its callees answering [`ANSWER`], every word the object.
fn run(address: u32, callees: &[u32], words: usize) -> u8 {
    let mut e = Engine::new();
    let this = object(&mut e);
    for &callee in callees {
        e.register_double(callee, answer);
    }
    e.call(address, &vec![this; words]).u8()
}

#[test]
fn getters_answer_the_byte_as_it_is() {
    let mut e = Engine::new();
    let this = e.mem.alloc(0x1000);
    let mut wrong = Vec::new();
    for (address, offset) in [(0x0062f2f0_u32, 0x8_u32), (0x006fe520_u32, 0x48_u32)] {
        e.mem.set_u8(this + offset, BYTE);
        if e.call(address, &[this]).u8() != BYTE {
            wrong.push(address);
        }
    }
    assert!(wrong.is_empty(), "{wrong:08x?}");
}

#[test]
fn calls_pass_on_the_answer_as_it_is() {
    let rows: &[(u32, &[u32], usize)] = &[(0x0061f890, &[0x00405430, 0x007af430, 0x0082eb70], 3)];
    let wrong: Vec<u32> = rows
        .iter()
        .filter(|&&(address, callees, words)| run(address, callees, words) != BYTE)
        .map(|row| row.0)
        .collect();
    assert!(wrong.is_empty(), "{wrong:08x?}");
}

/// 00635fe9, 0063600c: `MOVZX` of the byte argument, pushed on.
#[test]
fn byte_arguments_are_passed_on_as_they_are() {
    for (address, callee) in [(0x00635fe0_u32, 0x0043b370_u32), (0x00636000, 0x00633ba0)] {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x10);
        let cell = e.mem.alloc(4);
        e.register_double(callee, move |e, words| {
            e.mem.set_u32(cell, words[1]);
            Ret::default()
        });
        e.call(address, &[this, ANSWER]);
        assert_eq!(e.mem.u32(cell), u32::from(BYTE), "{address:08x}");
    }
}

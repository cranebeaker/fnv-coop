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

/// 00404d57: `MOVZX` of the byte argument, pushed on to 00405570.
#[test]
fn the_copy_byte_is_passed_on_as_it_is() {
    let mut e = Engine::new();
    let this = e.mem.alloc(0x40);
    let cell = e.mem.alloc(4);
    e.register_double(0x0040_5570, move |e, words| {
        e.mem.set_u32(cell, words[2]);
        Ret::default()
    });
    e.call(0x00404d50, &[this, 8, ANSWER]);
    assert_eq!(e.mem.u32(cell), u32::from(BYTE));
}

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
fn calls_pass_on_the_answer_as_it_is() {
    let rows: &[(u32, &[u32], usize)] = &[
        (0x00c43160, &[0x00a742a0], 2),
        (0x00c46810, &[0x00c31670], 2),
        (0x00c4fa80, &[0x00a5d470], 2),
        (0x00c54150, &[0x00a7b290], 2),
        (0x00c5ffb0, &[0x00c31670], 2),
        (0x00c60930, &[0x00c31670], 2),
        (0x00c614e0, &[0x00a998e0], 2),
        (0x00c61f60, &[0x00c2f4f0], 2),
        (0x00c61fa0, &[0x00c31b80], 2),
        (0x00c63950, &[0x00c31670], 2),
        (0x00c64b70, &[0x00a5d2f0], 2),
    ];
    let wrong: Vec<u32> = rows
        .iter()
        .filter(|&&(address, callees, words)| run(address, callees, words) != BYTE)
        .map(|row| row.0)
        .collect();
    assert!(wrong.is_empty(), "{wrong:08x?}");
}

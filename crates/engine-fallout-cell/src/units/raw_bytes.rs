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
    for (address, offset) in [
        (0x00437bb0_u32, 0x87_u32),
        (0x004516b0_u32, 0x61a_u32),
        (0x004a0350_u32, 0x2d_u32),
        (0x00528790_u32, 0x60_u32),
        (0x00542d20_u32, 0x619_u32),
        (0x00552bb0_u32, 0x210_u32),
        (0x0055b9a0_u32, 0x2a_u32),
    ] {
        e.mem.set_u8(this + offset, BYTE);
        if e.call(address, &[this]).u8() != BYTE {
            wrong.push(address);
        }
    }
    assert!(wrong.is_empty(), "{wrong:08x?}");
}

#[test]
fn calls_pass_on_the_answer_as_it_is() {
    let rows: &[(u32, &[u32], usize)] = &[
        (0x00453490, &[0x00425fd0, 0x00454bd0, 0x00454e20], 2),
        (
            0x004efdc0,
            &[
                0x00559450, 0x005f2420, 0x00a5c570, 0x005f36f0, 0x009611e0, 0x004f0380, 0x00a35030,
            ],
            3,
        ),
        (0x00528080, &[0x00587410, 0x00528cb0, 0x0044cbf0], 3),
        (
            0x00528b60,
            &[0x0040fbf0, 0x00449f80, 0x0044c000, 0x0040fba0],
            4,
        ),
    ];
    let wrong: Vec<u32> = rows
        .iter()
        .filter(|&&(address, callees, words)| run(address, callees, words) != BYTE)
        .map(|row| row.0)
        .collect();
    assert!(wrong.is_empty(), "{wrong:08x?}");
}

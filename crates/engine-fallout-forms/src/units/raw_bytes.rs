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
        (0x00401680_u32, 0x299_u32),
        (0x004600f0_u32, 0x618_u32),
        (0x00473640_u32, 0x298_u32),
        (0x0058a200_u32, 0x30_u32),
        (0x0085feb0_u32, 0x158_u32),
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
        (0x004601d0, &[0x004844f0, 0x00471c20, 0x00471d60], 2),
        (0x00485270, &[0x0047c050, 0x0047c070, 0x0047c670], 2),
        (0x00597990, &[0x00598550, 0x00598060, 0x00598890], 4),
        (0x006189a0, &[], 2),
        (0x00619430, &[0x00401170, 0x0084e3a0, 0x00484020], 2),
    ];
    let wrong: Vec<u32> = rows
        .iter()
        .filter(|&&(address, callees, words)| run(address, callees, words) != BYTE)
        .map(|row| row.0)
        .collect();
    assert!(wrong.is_empty(), "{wrong:08x?}");
}

/// 0052577e: the raw byte goes on to 0050f670.
#[test]
fn the_weapon_flag_is_passed_on_as_it_is() {
    let mut e = Engine::new();
    let this = e.mem.alloc(0x10);
    let cell = e.mem.alloc(4);
    e.register(0x00ec43fb, |_, _| Ret::default());
    e.register_double(0x0050_f670, move |e, words| {
        e.mem.set_u32(cell, words[3]);
        Ret::default()
    });
    e.call(0x00525750, &[this, 0, 0, ANSWER, 0, 0]);
    assert_eq!(e.mem.u32(cell), u32::from(BYTE));
}

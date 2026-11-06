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
        (0x00c6b730, &[0x00a81010], 2),
        (0x00c85800, &[0x00ba87d0], 2),
        (0x00c87fa0, &[0x00c6b730], 2),
        (0x00c87fb0, &[0x00c6b760], 2),
        (0x00c8b240, &[0x00aa0ad0], 2),
        (0x00c8bf40, &[0x00c84dc0], 2),
        (0x00c8c390, &[0x00aa0ad0], 2),
        (0x00c8d090, &[0x00c85ac0], 2),
        (0x00c8ffc0, &[0x00c815b0], 2),
        (0x00ca4f40, &[0x00aa0ad0], 2),
        (0x00ca5040, &[0x00c84ec0], 2),
        (0x00ca6610, &[0x00c84ec0], 2),
        (0x00cb9920, &[0x00a7b3d0], 2),
        (0x00cb9c70, &[0x00a7b290], 2),
        (0x00cbe890, &[0x00c85810, 0x00cabb00, 0x00c68e30], 2),
        (0x00cc1990, &[0x00a7b290], 2),
        (0x00cced20, &[0x00aa0ad0], 2),
        (0x00ccf990, &[0x00aa0ad0], 2),
        (0x00ccfa50, &[0x00c84ec0], 2),
    ];
    let wrong: Vec<u32> = rows
        .iter()
        .filter(|&&(address, callees, words)| run(address, callees, words) != BYTE)
        .map(|row| row.0)
        .collect();
    assert!(wrong.is_empty(), "{wrong:08x?}");
}

/// 00c84899, 00c84922, 00c84949: `IsCollisionEnabled`'s AL is stored at `out` as it is.
#[test]
fn the_collision_answer_is_stored_as_it_is() {
    for (address, words) in [
        (0x00c84880_u32, 2_usize),
        (0x00c84900, 4),
        (0x00c84930, 2),
        (0x00c848b0, 5),
        (0x00c84960, 7),
    ] {
        let mut e = Engine::new();
        let this = object(&mut e);
        let out = e.mem.alloc(4);
        e.register_double(0x00c84740, answer);
        let mut call = vec![this, out];
        call.extend(vec![this; words]);
        e.call(address, &call);
        assert_eq!(e.mem.u8(out), BYTE, "{address:08x}");
    }
}

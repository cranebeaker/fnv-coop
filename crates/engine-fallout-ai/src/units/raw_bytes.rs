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
        (0x00566950_u32, 0x18d_u32),
        (0x005ce8f0_u32, 0x125_u32),
        (0x00649b80_u32, 0x18b_u32),
        (0x00886b10_u32, 0x146_u32),
        (0x00888a50_u32, 0xb3_u32),
        (0x008d1e70_u32, 0x1c0_u32),
        (0x008d1ed0_u32, 0x1c1_u32),
        (0x008d81a0_u32, 0x168_u32),
        (0x008d8200_u32, 0x126_u32),
        (0x008d8220_u32, 0x124_u32),
        (0x008d8270_u32, 0x127_u32),
        (0x008d8290_u32, 0x128_u32),
        (0x008d82b0_u32, 0x129_u32),
        (0x008d82d0_u32, 0x12a_u32),
        (0x008d8600_u32, 0x17c_u32),
        (0x008d8660_u32, 0xe0_u32),
        (0x008d86c0_u32, 0x188_u32),
        (0x0093a5d0_u32, 0x798_u32),
        (0x0093a640_u32, 0x799_u32),
        (0x00944340_u32, 0x681_u32),
        (0x00944360_u32, 0x66e_u32),
        (0x0096c950_u32, 0x14e_u32),
        (0x009e20a0_u32, 0x9f_u32),
        (0x009e3be0_u32, 0xde_u32),
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
        (0x00880910, &[0x00418520, 0x004bfc80], 1),
        (0x008bb630, &[0x009dba30], 2),
        (0x008bc7d0, &[0x005d43c0, 0x00422550], 2),
        (
            0x009062e0,
            &[0x0040fbf0, 0x00449f80, 0x00528bb0, 0x0040fba0],
            2,
        ),
        (0x009397e0, &[0x005a03f0], 1),
        (0x0093d4f0, &[0x00454af0], 0),
        (0x0094c7a0, &[0x008c3400], 2),
        (0x0094ca20, &[0x008230f0], 5),
        (0x009b4700, &[0x009c43e0], 3),
    ];
    let wrong: Vec<u32> = rows
        .iter()
        .filter(|&&(address, callees, words)| run(address, callees, words) != BYTE)
        .map(|row| row.0)
        .collect();
    assert!(wrong.is_empty(), "{wrong:08x?}");
}

/// A cell that keeps word `word` of the last call to `callee`.
fn recording(e: &mut Engine, callee: u32, word: usize) -> u32 {
    let cell = e.mem.alloc(4);
    e.register_double(callee, move |e, words| {
        e.mem.set_u32(cell, words[word]);
        Ret::default()
    });
    cell
}

/// `MOV CL,[arg]`, `MOV [this+offset],CL` (0087e96a, 0093a5fd, and the same at +0x799).
#[test]
fn byte_arguments_are_stored_as_they_are() {
    let mut e = Engine::new();
    let this = object(&mut e);
    for (address, offset) in [
        (0x0087e960_u32, 0x1b0_u32),
        (0x0093a5f0, 0x798),
        (0x0093a6f0, 0x799),
    ] {
        e.call(address, &[this, ANSWER]);
        assert_eq!(e.mem.u8(this + offset), BYTE, "{address:08x}");
    }
}

/// `MOVZX reg,[arg]`, `PUSH reg`: the callee is given the byte, not 0 or 1.
#[test]
fn byte_arguments_are_passed_on_as_they_are() {
    for (address, callee, word) in [
        (0x00886c80_u32, 0x00629670_u32, 2_usize),
        (0x008879d0, 0x00629670, 2),
    ] {
        let mut e = Engine::new();
        let this = object(&mut e);
        let cell = recording(&mut e, callee, word);
        e.call(address, &[this, ANSWER]);
        assert_eq!(e.mem.u32(cell), u32::from(BYTE), "{address:08x}");
    }
    // 008c4610: `(this, flag, value)` on to 008153a0.
    let mut e = Engine::new();
    let this = object(&mut e);
    let cell = recording(&mut e, 0x008153a0, 1);
    e.call(0x008c4610, &[this, ANSWER, 0]);
    assert_eq!(e.mem.u32(cell), u32::from(BYTE));
    // 0087fa60: the process at +0x68, slot 0x120.
    let mut e = Engine::new();
    let this = object(&mut e);
    let process = object(&mut e);
    e.mem.set_u32(this + 0x68, process);
    let cell = recording(&mut e, SLOT_TARGET, 1);
    e.call(0x0087fa60, &[this, ANSWER]);
    assert_eq!(e.mem.u32(cell), u32::from(BYTE));
}

/// 009f3e20: `-1` false, 0 the bed byte, 1 to 3 the food byte, the rest true (table at 9f3e68).
#[test]
fn the_sandbox_answer_is_the_byte() {
    let mut e = Engine::new();
    let this = e.mem.alloc(0x100);
    e.mem.set_u8(this + 0x37, 0x42);
    e.mem.set_u8(this + 0x35, 0x21);
    for (activity, expected) in [
        (0_u32, 0x42_u8),
        (1, 0x21),
        (3, 0x21),
        (u32::MAX, 0),
        (4, 1),
    ] {
        e.mem.set_u32(this + 0x58, activity);
        assert_eq!(e.call(0x009f3e20, &[this]).u8(), expected, "{activity}");
    }
}

/// 008d8240: the three bytes ORed (8d825b, 8d8267), not 0 or 1.
#[test]
fn grenade_or_mine_answers_the_bytes_ored() {
    let mut e = Engine::new();
    let this = e.mem.alloc(0x200);
    e.mem.set_u8(this + 0x124, 0x40);
    e.mem.set_u8(this + 0x126, 0x02);
    assert_eq!(e.call(0x008d8240, &[this]).u8(), 0x42);
}

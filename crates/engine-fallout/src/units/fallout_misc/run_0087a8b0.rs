//! `fallout/misc/run_0087a8b0` (Xbox PDB source unit), subsystem `fallout/misc`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The command table: 0x28-byte entries for commands from 0x1000, with the parameter count
/// (`u16`) at `+2` and the parameter descriptors (0xc bytes each, the type at `+4`) at `+4`.
const COMMAND_TABLE_01190920: u32 = 0x0119_0920;

// Translated from 0087a8b0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0087a8b0 - the /GS cookie check (00ec408c) is not translated
/// Unnamed in the engine map (`cdecl`, variable arguments: `queue`, `command`, `reference`,
/// then the command's parameters): builds an eight-word task on the stack (0x1f, `command`,
/// the reference's form id through `0084e3a0`, or 0) and fills words 3 to 7 from the
/// parameters the command table (`01190922 + (command - 0x1000) * 0x28`) describes: a type 2
/// parameter is a `double` stored as a `float`, types 1 and 8 a word copied as it is, types 4
/// and 6 a form whose id (`0084e3a0`, 0 for none) is stored; types 3, 5 and 7 (and others)
/// take no word and store nothing. Parameters past the eighth word are not read. The task goes
/// to `TaskQueueInterface::QueueIfActive(queue, &task)` (`0087d160`). The variable arguments
/// are copied to the stack in order and read from there, as the exe reads its own.
pub fn fn_0087a8b0(e: &mut Engine, queue: u32, command: u32, reference: u32, rest: &[u32]) {
    let words = rest.len() as u32;
    e.with_stack(0x20 + 4 * words + 8, |e, frame| {
        let task = frame.addr();
        let arguments = task + 0x20;
        for (index, word) in rest.iter().enumerate() {
            e.mem.set_u32(arguments + 4 * index as u32, *word);
        }
        e.mem.set_u32(task, 0x1f);
        e.mem.set_u32(task + 4, command);
        let id = if reference != 0 {
            e.call(0x0084_e3a0, &args![reference]).u32()
        } else {
            0
        };
        e.mem.set_u32(task + 8, id);
        let mut next = arguments;
        let mut slot = 3u32;
        let entry =
            COMMAND_TABLE_01190920.wrapping_add(command.wrapping_sub(0x1000).wrapping_mul(0x28));
        let count = e.mem.u16(entry.wrapping_add(2));
        let mut index = 0i32;
        while index < i32::from(count) {
            if slot < 8 {
                let descriptors = e.mem.u32(entry.wrapping_add(4));
                let kind = e.mem.u32(
                    descriptors
                        .wrapping_add((index as u32).wrapping_mul(0xc))
                        .wrapping_add(4),
                );
                match kind.wrapping_sub(1) {
                    0 | 7 => {
                        next = next.wrapping_add(4);
                        let word = e.mem.u32(next.wrapping_sub(4));
                        e.mem.set_u32(task + 4 * slot, word);
                        slot += 1;
                    }
                    1 => {
                        next = next.wrapping_add(8);
                        let value = x87_load64(e.mem.u64(next.wrapping_sub(8)));
                        e.mem.set_u32(task + 4 * slot, x87_store32(value));
                        slot += 1;
                    }
                    3 | 5 => {
                        next = next.wrapping_add(4);
                        let form = e.mem.u32(next.wrapping_sub(4));
                        let id = if form != 0 {
                            e.call(0x0084_e3a0, &args![form]).u32()
                        } else {
                            0
                        };
                        e.mem.set_u32(task + 4 * slot, id);
                        slot += 1;
                    }
                    // Types 3, 5 and 7, and any type past 8, take nothing.
                    _ => {}
                }
            }
            index += 1;
        }
        e.call(0x0087_d160, &args![queue, task]);
    });
}

/// The uniform form of `0087a8b0`: the three fixed words, then the parameters.
fn fn_0087a8b0_entry(e: &mut Engine, words: &[u32]) -> Ret {
    assert!(
        words.len() >= 3,
        "0087a8b0 needs at least 3 argument words, got {}",
        words.len()
    );
    fn_0087a8b0(e, words[0], words[1], words[2], &words[3..]);
    Ret::default()
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![(0x0087_a8b0, fn_0087a8b0_entry as AbiFn)]
}

#[cfg(test)]
mod b035_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    #[test]
    fn fn_0087a8b0_packs_the_parameters_by_type() {
        let mut e = Engine::new();
        e.map(COMMAND_TABLE_01190920 & !0xfff, 0x2000);
        // Command 0x1002: six parameters of types 2, 1, 3, 4, 6 and 9.
        let entry = COMMAND_TABLE_01190920 + 2 * 0x28;
        e.mem.set_u16(entry + 2, 6);
        let descriptors = e.mem.alloc(6 * 0xc);
        for (index, kind) in [2u32, 1, 3, 4, 6, 9].iter().enumerate() {
            e.mem.set_u32(descriptors + index as u32 * 0xc + 4, *kind);
        }
        e.mem.set_u32(entry + 4, descriptors);
        e.register(0x0084_e3a0, |_, a| (a[0] + 0x100).into_ret());
        let task = Rc::new(RefCell::new(vec![]));
        let seen = task.clone();
        e.register_double(0x0087_d160, move |e, a| {
            *seen.borrow_mut() = (0..8).map(|word| e.mem.u32(a[1] + 4 * word)).collect();
            assert_eq!(a[0], 0x50);
            Ret::default()
        });
        let half = 0.5f64.to_bits();
        let words = [
            0x50u32,
            0x1002,
            0x20,
            half as u32,
            (half >> 32) as u32,
            0x77,
            0x30,
            0,
        ];
        e.call(0x0087_a8b0, &words);
        assert_eq!(
            *task.borrow(),
            vec![0x1f, 0x1002, 0x120, 0.5f32.to_bits(), 0x77, 0x130, 0, 0]
        );
        // Without a reference the third word is 0; past the eighth word nothing is read.
        e.mem.set_u16(entry + 2, 0);
        e.call(0x0087_a8b0, &[0x50, 0x1002, 0]);
        assert_eq!(task.borrow()[2], 0);
    }
}

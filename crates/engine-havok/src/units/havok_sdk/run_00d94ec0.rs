//! `Havok SDK/run_00d94ec0` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The thread's memory router (`00c85750`); its `+0x10` is the allocator whose slot `+4` takes
/// the size.
const MEMORY_ROUTER: u32 = 0x00c8_5750;

/// Allocates `size` bytes through the router's allocator (slot `+4`).
fn router_allocate(e: &mut Engine, size: u32) -> u32 {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    e.vcall(allocator, 4, &args![size]).u32()
}

// Translated from 00d94ec0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Cdecl with the register `eax` first (a list header: first block at `+0`, count at `+4`) and
/// then the fifteen stack words `a0` ... `a14` (`a12` and `a13` are tested as bytes). Names are
/// positions: the exe's own are unknown (the unit is `Havok SDK/run_00d94ec0`, a job
/// that cuts a batch of constraint entries into a 0x1000-byte block and a linked 0x30-byte
/// node).
///
/// First part: a 0x1000-byte block is allocated (its words at `+0`, `+0x10`, `+0x14` zeroed) and
/// linked in: through `*a2` (the previous block) when there is one (`*a2 = block`, its `+0x10`
/// word set to `*a6`), or as the header's first block when there is none and the byte `a12` is
/// zero (`eax[0] = block`, `eax[1] = 0`); then `eax[1]` is incremented. The block gets `a0` at
/// `+4`, the count `[a8 + 4]` at `+0x14`, `[a0 + 0x30]` at `+8` and the original `*a6` at `+0xc`.
/// For each of the `[a8 + 4]` entries (pairs of words at `*a8`, 8 bytes apiece) a record of
/// `0x1c` bytes is written from the entry's first word `q` at block `+0x1c`, `+0x38`, ...: a
/// `q` whose byte at `+0x12` has any of the bits `0xb` first appends `(entry words 0 and 1,
/// the record's address - 4)` to the 16-byte-element array at `a14`; the record takes `q`'s
/// words at `+0xc`, `+0`, `+0x1c` and 16-bit words at `+0x10` and `+0x20`, the entry's 16-bit
/// words plus one, `q`'s words at `+4` and `+8` plus `0xf0`; `*a6` grows by `q`'s 16-bit word
/// at `+0x14` and `*a7` by four times the 16-bit word at `+0x18`. Afterwards `[a8 + 4]` is
/// zeroed, `*a6` grows by 0x10 and is stored at the block's `+0x10`, `*a7` is rounded up to 16
/// and `*a2 = block`; with the byte `a13` zero it ends.
///
/// Second part (byte `a13` non-zero): a 0x30-byte node is allocated (words at `+0`, `+0x20`, the
/// byte at `+0x1c` and the 16-bit word at `+0x1e` zeroed) and put after `**a3` (`**a3 = node`
/// when `*a3` is set). When `a10` differs from `*a9`: `*a1 = node` if `a10` is 0, `*a9 = a10`,
/// and when `*a3` is set the chain from `*a4` back to `*a5` gets the 16-bit word at `**a11`
/// stored at `+0x1e` and `*a5` at `+0x20` of each node (following the word at `+0`), the node
/// `*a3` is marked (byte `+0x1c` = 1) and `*a11` becomes `&(*a3)[0x1e]`, then `*a4 = *a5` and
/// `*a5 = node`. Finally the 16-bit word at `**a11` is incremented, the node takes `a0`, `[a0 +
/// 0x30]`, the original `*a6`, the original `*a7`, `*a6 - original *a6` and `*a7 - original
/// *a7` at `+4` ... `+0x18`, and `*a3 = node`.
#[allow(clippy::too_many_arguments)]
pub fn fn_00d94ec0(
    e: &mut Engine,
    eax: u32,
    a0: u32,
    a1: u32,
    a2: u32,
    a3: u32,
    a4: u32,
    a5: u32,
    a6: u32,
    a7: u32,
    a8: u32,
    a9: u32,
    a10: u32,
    a11: u32,
    a12: u8,
    a13: u8,
    a14: u32,
) {
    let original_a7 = e.mem.u32(a7);
    let original_a6 = e.mem.u32(a6);
    let block = router_allocate(e, 0x1000);
    if block != 0 {
        e.mem.set_u32(block + 0x14, 0);
        e.mem.set_u32(block + 0x10, 0);
        e.mem.set_u32(block, 0);
    }
    let tail = e.mem.u32(a2);
    if tail == 0 {
        if a12 == 0 {
            e.mem.set_u32(eax, block);
            e.mem.set_u32(eax + 4, 0);
        }
    } else {
        e.mem.set_u32(tail, block);
        let last = e.mem.u32(a2);
        let current = e.mem.u32(a6);
        e.mem.set_u32(last + 0x10, current);
    }
    let count_word = e.mem.u32(eax + 4).wrapping_add(1);
    e.mem.set_u32(eax + 4, count_word);
    e.mem.set_u32(block + 4, a0);
    let entry_count = e.mem.u32(a8 + 4);
    e.mem.set_u32(block + 0x14, entry_count);
    let word = e.mem.u32(a0 + 0x30);
    e.mem.set_u32(block + 8, word);
    e.mem.set_u32(block + 0xc, original_a6);
    let count = e.mem.u32(a8 + 4);
    if (count as i32) > 0 {
        let mut record = block + 0x1c;
        let mut index = 0u32;
        loop {
            let data = e.mem.u32(a8);
            let pair = data.wrapping_add(index.wrapping_mul(8));
            let q = e.mem.u32(pair);
            if e.mem.u8(q + 0x12) & 0xb != 0 {
                let appended = e.mem.u32(a14 + 4);
                e.mem.set_u32(a14 + 4, appended.wrapping_add(1));
                let slot = (appended << 4).wrapping_add(e.mem.u32(a14));
                let first = e.mem.u32(pair);
                e.mem.set_u32(slot, first);
                let second = e.mem.u32(pair + 4);
                e.mem.set_u32(slot + 4, second);
                e.mem.set_u32(slot + 8, record.wrapping_sub(4));
            }
            let word = e.mem.u32(q + 0xc);
            e.mem.set_u32(record.wrapping_sub(4), word);
            let half = e.mem.u16(q + 0x10);
            e.mem.set_u16(record + 0x10, half);
            let word = e.mem.u32(q);
            e.mem.set_u32(record, word);
            let word = e.mem.u32(q + 0x1c);
            e.mem.set_u32(record + 4, word);
            let half = e.mem.u16(q + 0x20);
            e.mem.set_u16(record + 0x12, half);
            let second_half = e.mem.u16(pair + 6);
            let first_half = e.mem.u16(pair + 4).wrapping_add(1);
            e.mem.set_u16(record + 0x14, first_half);
            e.mem
                .set_u16(record + 0x16, u32::from(second_half).wrapping_add(1) as u16);
            let word = e.mem.u32(q + 4).wrapping_add(0xf0);
            e.mem.set_u32(record + 8, word);
            let word = e.mem.u32(q + 8).wrapping_add(0xf0);
            e.mem.set_u32(record + 0xc, word);
            let grow = u32::from(e.mem.u16(q + 0x14));
            let total = e.mem.u32(a6).wrapping_add(grow);
            e.mem.set_u32(a6, total);
            let grow = u32::from(e.mem.u16(q + 0x18)).wrapping_mul(4);
            let total = e.mem.u32(a7).wrapping_add(grow);
            e.mem.set_u32(a7, total);
            index = index.wrapping_add(1);
            record = record.wrapping_add(0x1c);
            if (index as i32) >= (count as i32) {
                break;
            }
        }
    }
    e.mem.set_u32(a8 + 4, 0);
    let total = e.mem.u32(a6).wrapping_add(0x10);
    e.mem.set_u32(a6, total);
    let word = e.mem.u32(a6);
    e.mem.set_u32(block + 0x10, word);
    let rounded = e.mem.u32(a7).wrapping_add(0xf) & 0xffff_fff0;
    e.mem.set_u32(a7, rounded);
    e.mem.set_u32(a2, block);
    if a13 == 0 {
        return;
    }

    let node = router_allocate(e, 0x30);
    if node != 0 {
        e.mem.set_u32(node, 0);
        e.mem.set_u8(node + 0x1c, 0);
        e.mem.set_u16(node + 0x1e, 0);
        e.mem.set_u32(node + 0x20, 0);
    }
    let previous = e.mem.u32(a3);
    if previous != 0 {
        e.mem.set_u32(previous, node);
    }
    if a10 != e.mem.u32(a9) {
        if a10 == 0 {
            e.mem.set_u32(a1, node);
        }
        e.mem.set_u32(a9, a10);
        if e.mem.u32(a3) == 0 {
            // The chain walk is skipped.
        } else {
            let mut chain = e.mem.u32(a4);
            let stamp = u32::from(e.mem.u16(e.mem.u32(a11)));
            if chain != 0 && chain != e.mem.u32(a5) {
                loop {
                    e.mem.set_u16(chain + 0x1e, stamp as u16);
                    let link = e.mem.u32(a5);
                    e.mem.set_u32(chain + 0x20, link);
                    chain = e.mem.u32(chain);
                    if chain == 0 || chain == e.mem.u32(a5) {
                        break;
                    }
                }
            }
            let owner = e.mem.u32(a3);
            e.mem.set_u8(owner + 0x1c, 1);
            let owner = e.mem.u32(a3);
            e.mem.set_u32(a11, owner.wrapping_add(0x1e));
        }
        let shared = e.mem.u32(a5);
        e.mem.set_u32(a4, shared);
        e.mem.set_u32(a5, node);
    }
    let counter = e.mem.u32(a11);
    let value = e.mem.u16(counter).wrapping_add(1);
    e.mem.set_u16(counter, value);
    e.mem.set_u32(node + 4, a0);
    let word = e.mem.u32(a0 + 0x30);
    e.mem.set_u32(node + 8, word);
    e.mem.set_u32(node + 0xc, original_a6);
    e.mem.set_u32(node + 0x10, original_a7);
    let used = e.mem.u32(a6).wrapping_sub(original_a6);
    e.mem.set_u32(node + 0x14, used);
    let used_second = e.mem.u32(a7).wrapping_sub(original_a7);
    e.mem.set_u32(node + 0x18, used_second);
    e.mem.set_u32(a3, node);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(
        0x00d9_4ec0,
        fn_00d94ec0(u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u8, u8, u32)
    )]
}

#[cfg(test)]
mod a09_tests {
    use super::*;

    /// One call's inputs: the buffers and the pointers-to-pointers the function works through.
    struct Setup {
        header: u32,
        a: [u32; 15],
        blocks: Vec<u32>,
    }

    fn make_router(e: &mut Engine) {
        let vtable = e.mem.alloc(0x10);
        e.mem.set_u32(vtable + 4, 0x0900_0001);
        let allocator = e.mem.alloc(0x10);
        e.mem.set_u32(allocator, vtable);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        e.register(0x0900_0001, |e, a| Ret {
            eax: e.mem.alloc(a[1]),
            ..Ret::default()
        });
    }

    /// A pointer cell holding `value`.
    fn cell(e: &mut Engine, value: u32) -> u32 {
        let cell = e.mem.alloc(8);
        e.mem.set_u32(cell, value);
        cell
    }

    /// Entry `i` of the batch has the constraint record `q_i` (flag byte at `+0x12`).
    fn setup(e: &mut Engine, flags: &[u8]) -> Setup {
        make_router(e);
        let header = e.mem.alloc(8);
        let a0 = e.mem.alloc(0x40);
        e.mem.set_u32(a0 + 0x30, 0x3030);
        let a2 = cell(e, 0);
        let a3 = cell(e, 0);
        let a4 = cell(e, 0);
        let a5 = cell(e, 0);
        let a6 = cell(e, 0x100);
        let a7 = cell(e, 0x201);
        let a8 = e.mem.alloc(8);
        let entries = e.mem.alloc(8 * flags.len() as u32 + 8);
        let mut blocks = Vec::new();
        for (i, flag) in flags.iter().enumerate() {
            let q = e.mem.alloc(0x40);
            e.mem.set_u8(q + 0x12, *flag);
            e.mem.set_u32(q + 0xc, 0x1000 + i as u32);
            e.mem.set_u16(q + 0x10, 0x20 + i as u16);
            e.mem.set_u32(q, 0x2000 + i as u32);
            e.mem.set_u32(q + 0x1c, 0x3000 + i as u32);
            e.mem.set_u16(q + 0x20, 0x40 + i as u16);
            e.mem.set_u32(q + 4, 0x10 + i as u32);
            e.mem.set_u32(q + 8, 0x20 + i as u32);
            e.mem.set_u16(q + 0x14, 3);
            e.mem.set_u16(q + 0x18, 5);
            e.mem.set_u32(entries + 8 * i as u32, q);
            e.mem.set_u16(entries + 8 * i as u32 + 4, 10 + i as u16);
            e.mem.set_u16(entries + 8 * i as u32 + 6, 20 + i as u16);
            blocks.push(q);
        }
        e.mem.set_u32(a8, entries);
        e.mem.set_u32(a8 + 4, flags.len() as u32);
        let a9 = cell(e, 0);
        let a11_counter = cell(e, 0);
        let a11 = cell(e, a11_counter);
        let a14 = e.mem.alloc(8);
        let a14_data = e.mem.alloc(0x40);
        e.mem.set_u32(a14, a14_data);
        let a1 = cell(e, 0);
        Setup {
            header,
            a: [a0, a1, a2, a3, a4, a5, a6, a7, a8, a9, 0, a11, 0, 0, a14],
            blocks,
        }
    }

    fn call(e: &mut Engine, s: &Setup, a10: u32, a12: u8, a13: u8) {
        let a = s.a;
        e.call(
            0x00d9_4ec0,
            &args![
                s.header, a[0], a[1], a[2], a[3], a[4], a[5], a[6], a[7], a[8], a[9], a10, a[11],
                a12, a13, a[14]
            ],
        );
    }

    #[test]
    fn the_first_part_fills_the_block_and_the_records_and_stops_without_the_second_flag() {
        let mut e = Engine::new();
        let s = setup(&mut e, &[0, 0x02]);
        call(&mut e, &s, 0, 0, 0);
        let block = e.mem.u32(s.a[2]);
        // No previous block and `a12` zero: the header takes the block; its count goes to 1.
        assert_eq!(e.mem.u32(s.header), block);
        assert_eq!(e.mem.u32(s.header + 4), 1);
        assert_eq!(e.mem.u32(block + 4), s.a[0]);
        assert_eq!(e.mem.u32(block + 0x14), 2);
        assert_eq!(e.mem.u32(block + 8), 0x3030);
        assert_eq!(e.mem.u32(block + 0xc), 0x100);
        // Record 0 at block + 0x1c (word -4 at +0x18).
        let record = block + 0x1c;
        assert_eq!(e.mem.u32(record - 4), 0x1000);
        assert_eq!(e.mem.u16(record + 0x10), 0x20);
        assert_eq!(e.mem.u32(record), 0x2000);
        assert_eq!(e.mem.u32(record + 4), 0x3000);
        assert_eq!(e.mem.u16(record + 0x12), 0x40);
        assert_eq!(e.mem.u16(record + 0x14), 11);
        assert_eq!(e.mem.u16(record + 0x16), 21);
        assert_eq!(e.mem.u32(record + 8), 0x10 + 0xf0);
        assert_eq!(e.mem.u32(record + 0xc), 0x20 + 0xf0);
        // Record 1 follows 0x1c later.
        assert_eq!(e.mem.u32(record + 0x1c), 0x2001);
        // Counters: *a6 grew by 3 per entry plus 0x10, *a7 by 20 per entry, rounded to 16.
        assert_eq!(e.mem.u32(s.a[6]), 0x100 + 6 + 0x10);
        assert_eq!(e.mem.u32(block + 0x10), 0x100 + 6 + 0x10);
        assert_eq!(e.mem.u32(s.a[7]), (0x201 + 40 + 0xf) & !0xf);
        // The entry array's count is reset; only the flagged entry went to the extra array.
        assert_eq!(e.mem.u32(s.a[8] + 4), 0);
        assert_eq!(e.mem.u32(s.a[14] + 4), 1);
        let slot = e.mem.u32(s.a[14]);
        assert_eq!(e.mem.u32(slot), s.blocks[1]);
        assert_eq!(e.mem.u32(slot + 8), block + 0x1c + 0x1c - 4);
        // No node was made.
        assert_eq!(e.mem.u32(s.a[3]), 0);
    }

    #[test]
    fn a_previous_block_is_linked_and_keeps_the_running_pointer() {
        let mut e = Engine::new();
        let s = setup(&mut e, &[]);
        let previous = e.mem.alloc(0x20);
        e.mem.set_u32(s.a[2], previous);
        call(&mut e, &s, 0, 0, 0);
        let block = e.mem.u32(s.a[2]);
        assert_eq!(e.mem.u32(previous), block);
        assert_eq!(e.mem.u32(previous + 0x10), 0x100);
        // The header was not touched but its count still rises.
        assert_eq!(e.mem.u32(s.header), 0);
        assert_eq!(e.mem.u32(s.header + 4), 1);
        // With a12 set and no previous block the header is left alone as well.
        let mut e = Engine::new();
        let s = setup(&mut e, &[]);
        call(&mut e, &s, 0, 1, 0);
        assert_eq!(e.mem.u32(s.header), 0);
        assert_eq!(e.mem.u32(s.header + 4), 1);
    }

    #[test]
    fn the_second_part_makes_and_links_a_node() {
        let mut e = Engine::new();
        let s = setup(&mut e, &[0]);
        call(&mut e, &s, 7, 0, 1);
        let node = e.mem.u32(s.a[3]);
        assert_ne!(node, 0);
        // a10 (7) differs from *a9 (0) and is non-zero: *a9 = 7; no chain; *a4 = old *a5, *a5 = node.
        assert_eq!(e.mem.u32(s.a[9]), 7);
        assert_eq!(e.mem.u32(s.a[5]), node);
        assert_eq!(e.mem.u32(s.a[4]), 0);
        assert_eq!(e.mem.u32(s.a[1]), 0);
        let counter = e.mem.u32(s.a[11]);
        assert_eq!(e.mem.u16(counter), 1);
        assert_eq!(e.mem.u32(node + 4), s.a[0]);
        assert_eq!(e.mem.u32(node + 8), 0x3030);
        assert_eq!(e.mem.u32(node + 0xc), 0x100);
        assert_eq!(e.mem.u32(node + 0x10), 0x201);
        assert_eq!(e.mem.u32(node + 0x14), 3 + 0x10);
        assert_eq!(e.mem.u32(node + 0x18), ((0x201 + 20 + 0xf) & !0xf) - 0x201);
        assert_eq!(e.mem.u32(node), 0);
        assert_eq!(e.mem.u8(node + 0x1c), 0);
    }

    #[test]
    fn a_zero_a10_sets_the_head_and_a_matching_a10_skips_the_chain_work() {
        let mut e = Engine::new();
        let s = setup(&mut e, &[]);
        // *a9 = 5 and a10 = 5: equal, so nothing but the closing part runs.
        e.mem.set_u32(s.a[9], 5);
        call(&mut e, &s, 5, 0, 1);
        let node = e.mem.u32(s.a[3]);
        assert_eq!(e.mem.u32(s.a[5]), 0);
        assert_eq!(e.mem.u32(s.a[1]), 0);
        assert_ne!(node, 0);
        // A second batch with a10 = 0 (different from *a9 = 5): *a1 = the new node.
        call(&mut e, &s, 0, 0, 1);
        let second = e.mem.u32(s.a[3]);
        assert_ne!(second, node);
        assert_eq!(e.mem.u32(s.a[1]), second);
        assert_eq!(e.mem.u32(s.a[9]), 0);
        // *a3 pointed to the first node: its first word links to the second.
        assert_eq!(e.mem.u32(node), second);
        // *a3 was set, so the chain part ran: the node was marked and *a11 moved.
        assert_eq!(e.mem.u8(node + 0x1c), 1);
        assert_eq!(e.mem.u32(s.a[11]), node + 0x1e);
    }

    #[test]
    fn the_chain_walk_stamps_each_node_up_to_the_shared_one() {
        let mut e = Engine::new();
        let s = setup(&mut e, &[]);
        // First batch: node n1 becomes *a3 and *a5.
        call(&mut e, &s, 1, 0, 1);
        let n1 = e.mem.u32(s.a[3]);
        // Prepare a chain n2 -> n3 -> (shared = n1): `*a4 = n2`, `*a5 = n1`.
        let n2 = e.mem.alloc(0x30);
        let n3 = e.mem.alloc(0x30);
        e.mem.set_u32(n2, n3);
        e.mem.set_u32(n3, n1);
        e.mem.set_u32(s.a[4], n2);
        // The stamp is the 16-bit word at `**a11`: give it a value.
        let stamp_cell = e.mem.alloc(8);
        e.mem.set_u16(stamp_cell, 0x77);
        e.mem.set_u32(s.a[11], stamp_cell);
        call(&mut e, &s, 2, 0, 1);
        assert_eq!(e.mem.u16(n2 + 0x1e), 0x77);
        assert_eq!(e.mem.u32(n2 + 0x20), n1);
        assert_eq!(e.mem.u16(n3 + 0x1e), 0x77);
        assert_eq!(e.mem.u32(n3 + 0x20), n1);
        // n1 is the shared node: untouched by the walk but marked as `*a3`'s owner.
        assert_eq!(e.mem.u8(n1 + 0x1c), 1);
        // `*a4 = old *a5 (n1)` and `*a5 = the new node`.
        assert_eq!(e.mem.u32(s.a[4]), n1);
        assert_eq!(e.mem.u32(s.a[5]), e.mem.u32(s.a[3]));
    }
}

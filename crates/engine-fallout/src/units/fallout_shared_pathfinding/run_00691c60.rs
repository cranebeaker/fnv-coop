//! `fallout shared/pathfinding/run_00691c60` (Xbox PDB source unit), subsystem `fallout shared/pathfinding`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `0069a690` (`ECX` = the destination's +4, the source's +4; `RET 4`).
const COPY_TAIL: u32 = 0x0069_a690;
/// `0042f5d0` (cdecl: address, size).
const SIZED_0042F5D0: u32 = 0x0042_f5d0;

// Translated from 00691c60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Copy-constructor shape (one word, the source; `RET 4`): the first word
/// is copied, then `0069a690` on `this + 4` with `source + 4`. Returns
/// `this`. The engine map gives it no name.
pub fn fn_00691c60(e: &mut Engine, this: Ptr, source: Ptr) -> Ptr {
    let word = e.mem.u32(source.addr());
    e.mem.set_u32(this.addr(), word);
    e.call(COPY_TAIL, &args![this.addr() + 4, source.addr() + 4]);
    this
}

// Translated from 00691d20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `0042f5d0` with an address and 12 times a count (`thiscall`, `ECX` not
/// read; `RET 8`), answering the block it gives (the exe ends on that call
/// and leaves its `EAX`): a `BSSimpleArray` `_Reallocate` (vtable slot
/// `+0xC`, Xbox PDB). The engine map gives it no name.
pub fn fn_00691d20(e: &mut Engine, _unused_0: Ptr, address: u32, count: u32) -> u32 {
    e.call(SIZED_0042F5D0, &args![address, count.wrapping_mul(0xc)])
        .u32()
}

/// `0042f5a0` (`cdecl`, two words; no engine-map name), `0072ba80` (`thiscall`, two words:
/// the first element and a count; the element destruction) and `006dc6e0` (`thiscall`,
/// three words: destination, source and count; the element move).
const TAIL_COUNT_0042F5A0: u32 = 0x0042_f5a0;
const DESTROY_ELEMENTS: u32 = 0x0072_ba80;
const MOVE_ELEMENTS: u32 = 0x006d_c6e0;

// Translated from 00691c90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: removes `count` 12-byte elements at `index` from the array
/// (data at +4, size at +8; `thiscall`, `RET 8`). With
/// `tail = 0042f5a0(size - index - count, count)` it destroys the removed elements
/// (`0072ba80`), moves `tail` elements from `data + (size - tail) * 12` down to the gap
/// (`006dc6e0`) and lowers the size by `count` (wrapping 32-bit arithmetic).
pub fn fn_00691c90(e: &mut Engine, this: Ptr, index: u32, count: u32) {
    let base = this.addr();
    let size = e.mem.u32(base + 8);
    let rest = size.wrapping_sub(index).wrapping_sub(count);
    let tail = e.call(TAIL_COUNT_0042F5A0, &args![rest, count]).u32();
    let data = e.mem.u32(base + 4);
    let gap = index.wrapping_mul(0xc).wrapping_add(data);
    e.call(DESTROY_ELEMENTS, &args![this, gap, count]);
    let size = e.mem.u32(base + 8);
    let data = e.mem.u32(base + 4);
    let source = data
        .wrapping_add(size.wrapping_mul(0xc))
        .wrapping_sub(tail.wrapping_mul(0xc));
    let gap = index.wrapping_mul(0xc).wrapping_add(e.mem.u32(base + 4));
    e.call(MOVE_ELEMENTS, &args![this, gap, source, tail]);
    let size = e.mem.u32(base + 8).wrapping_sub(count);
    e.mem.set_u32(base + 8, size);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0069_1c90, fn_00691c90(Ptr, u32, u32)),
        entry!(0x0069_1c60, fn_00691c60(Ptr, Ptr) -> Ptr),
        entry!(0x0069_1d20, fn_00691d20(Ptr, u32, u32) -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn log_of(e: &mut Engine, addr: u32, words: &[u32]) -> Vec<(u32, Vec<u32>)> {
        e.call_log = Some(vec![]);
        e.call(addr, words);
        e.call_log
            .take()
            .unwrap()
            .into_iter()
            .filter(|(a, _)| *a != addr)
            .collect()
    }

    #[test]
    fn at_00691c60_copies_the_head_and_the_tail() {
        let mut e = Engine::new();
        e.register(COPY_TAIL, |_, _| Ret::default());
        let this = e.mem.alloc(0x10);
        let source = e.mem.alloc(0x10);
        e.mem.set_u32(source, 0x1234);
        assert_eq!(
            log_of(&mut e, 0x0069_1c60, &[this, source]),
            vec![(COPY_TAIL, vec![this + 4, source + 4])]
        );
        assert_eq!(e.mem.u32(this), 0x1234);
        assert_eq!(e.call(0x0069_1c60, &args![this, source]).u32(), this);
    }

    #[test]
    fn at_00691d20_passes_twelve_bytes_per_element() {
        let mut e = Engine::new();
        e.register(SIZED_0042F5D0, |_, _| Ret::default());
        assert_eq!(
            log_of(&mut e, 0x0069_1d20, &[0, 0x5000, 3]),
            vec![(SIZED_0042F5D0, vec![0x5000, 0x24])]
        );
    }

    #[test]
    fn at_00691c90_moves_the_tail_down_over_the_removed_range() {
        let mut e = Engine::new();
        e.register(TAIL_COUNT_0042F5A0, |_, a| Ret {
            eax: a[0].min(a[1]),
            ..Ret::default()
        });
        e.register(DESTROY_ELEMENTS, |_, _| Ret::default());
        e.register(MOVE_ELEMENTS, |_, _| Ret::default());
        let this = e.mem.alloc(0x10);
        e.mem.set_u32(this + 4, 0x9000);
        e.mem.set_u32(this + 8, 10);
        // Remove 2 at index 3: 5 elements follow, 2 of them move.
        assert_eq!(
            log_of(&mut e, 0x0069_1c90, &[this, 3, 2]),
            vec![
                (TAIL_COUNT_0042F5A0, vec![5, 2]),
                (DESTROY_ELEMENTS, vec![this, 0x9024, 2]),
                (MOVE_ELEMENTS, vec![this, 0x9024, 0x9000 + 8 * 0xc, 2]),
            ]
        );
        assert_eq!(e.mem.u32(this + 8), 8);
        // Remove the last 3: nothing follows, nothing moves.
        assert_eq!(
            log_of(&mut e, 0x0069_1c90, &[this, 5, 3]),
            vec![
                (TAIL_COUNT_0042F5A0, vec![0, 3]),
                (DESTROY_ELEMENTS, vec![this, 0x903c, 3]),
                (MOVE_ELEMENTS, vec![this, 0x903c, 0x9060, 0]),
            ]
        );
        assert_eq!(e.mem.u32(this + 8), 5);
    }

    #[test]
    fn at_00691d20_answers_the_new_block() {
        let mut e = Engine::new();
        e.register(SIZED_0042F5D0, |_, a| (a[0] + a[1]).into_ret());
        assert_eq!(
            e.call(0x0069_1d20, &args![0u32, 0x5000u32, 3u32]).u32(),
            0x5024
        );
    }
}

#[cfg(test)]
mod la1_tests {
    use super::*;

    #[test]
    fn fn_00691c90_destroys_moves_and_shrinks() {
        let mut e = Engine::new();
        // The tail count double: the smaller of the two words.
        e.register(TAIL_COUNT_0042F5A0, |_, a| a[0].min(a[1]).into_ret());
        e.register(DESTROY_ELEMENTS, |_, _| Ret::default());
        e.register(MOVE_ELEMENTS, |_, _| Ret::default());
        let this = e.mem.alloc(0xc);
        e.mem.set_u32(this + 4, 0x1000);
        e.mem.set_u32(this + 8, 10);
        e.call_log = Some(vec![]);
        // Remove 3 at index 2: 5 follow, the tail count is min(5, 3) = 3.
        e.call(0x0069_1c90, &args![this, 2u32, 3u32]);
        let log = e.call_log.take().unwrap();
        let calls: Vec<(u32, Vec<u32>)> = log.into_iter().skip(1).collect();
        assert_eq!(
            calls,
            vec![
                (TAIL_COUNT_0042F5A0, vec![5, 3]),
                (DESTROY_ELEMENTS, vec![this, 0x1000 + 24, 3]),
                (MOVE_ELEMENTS, vec![this, 0x1000 + 24, 0x1000 + 7 * 12, 3]),
            ]
        );
        assert_eq!(e.mem.u32(this + 8), 7);
    }
}

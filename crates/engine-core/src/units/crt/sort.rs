//! The C runtime's quicksort (MSVC 2008 LIBCMT, statically linked into FalloutNV.exe
//! 1.4.0.525), translated from the exe: `_qsort` (`00ec6f20`, Ghidra's function-ID label,
//! "Library Function - Single Match", Visual Studio 2008 Release) with its two helpers,
//! `swap` (`00ec6e60`) and `shortsort` (`00ec6e90`), which take arguments in registers.

use super::*;

/// `_errno` (`00ec85e3`, cdecl, no arguments: the address of `errno`) and the
/// invalid-parameter report (`00ec7c30`, cdecl, five words): translated elsewhere.
const ERRNO_ADDR: u32 = 0x00ec_85e3;
const INVALID_PARAMETER_ADDR: u32 = 0x00ec_7c30;
/// The helpers by address, as `_qsort` calls them.
const SWAP_ADDR: u32 = 0x00ec_6e60;
const SHORTSORT_ADDR: u32 = 0x00ec_6e90;

// Translated from 00ec6e60 (disassembled, FalloutNV.exe 1.4.0.525)
/// `swap` (Ghidra's function-ID label), registers only: `EAX` and `ECX` the two elements,
/// `EDX` their size. Unless the two are the same address or the size is 0, the bytes are
/// exchanged one at a time from the first up (the byte of `ECX` stored at `EAX` first).
pub fn swap(e: &mut Engine, a: u32, b: u32, width: u32) {
    if a == b || width == 0 {
        return;
    }
    let (mut a, mut b, mut left) = (a, b, width);
    loop {
        let from_b = e.mem.u8(b);
        let from_a = e.mem.u8(a);
        e.mem.set_u8(a, from_b);
        left = left.wrapping_sub(1);
        e.mem.set_u8(b, from_a);
        a = a.wrapping_add(1);
        b = b.wrapping_add(1);
        if left == 0 {
            break;
        }
    }
}

// Translated from 00ec6e90 (disassembled, FalloutNV.exe 1.4.0.525)
/// `shortsort` (Ghidra's function-ID label): `EAX` the last element (here the first word),
/// then cdecl the first element, the element size and the comparison function. A selection
/// sort from the top: while the last element is above the first (unsigned), the largest of
/// the elements up to it is found (each element from the second up is compared with the
/// largest so far, `compare(element, largest)`; a positive result, signed, makes it the
/// largest) and, unless it is the last one or the size is 0, its bytes are exchanged with the
/// last's (inline, the same loop as `swap`); then the last element moves down one.
pub fn shortsort(e: &mut Engine, hi: u32, lo: u32, width: u32, compare: u32) {
    let mut hi = hi;
    if hi <= lo {
        return;
    }
    // [EBP-4]: the second element, fixed for the whole sort.
    let second = lo.wrapping_add(width);
    loop {
        let mut p = second;
        let mut max = lo;
        if p <= hi {
            loop {
                let result = e.call(compare, &args![p, max]).i32();
                if result > 0 {
                    max = p;
                }
                p = p.wrapping_add(width);
                if p > hi {
                    break;
                }
            }
        }
        if max != hi && width != 0 {
            let distance = max.wrapping_sub(hi);
            let mut at = hi;
            let mut left = width;
            loop {
                let low = e.mem.u8(at);
                let high = e.mem.u8(distance.wrapping_add(at));
                e.mem.set_u8(distance.wrapping_add(at), low);
                left = left.wrapping_sub(1);
                e.mem.set_u8(at, high);
                at = at.wrapping_add(1);
                if left == 0 {
                    break;
                }
            }
        }
        hi = hi.wrapping_sub(width);
        if hi <= lo {
            break;
        }
    }
}

/// The registry entry of [`shortsort`]: `EAX`, then the three stack words.
fn shortsort_entry(e: &mut Engine, words: &[u32]) -> Ret {
    shortsort(e, words[0], words[1], words[2], words[3]);
    Ret::default()
}

/// The registry entry of [`swap`]: `EAX`, `ECX`, `EDX`.
fn swap_entry(e: &mut Engine, words: &[u32]) -> Ret {
    swap(e, words[0], words[1], words[2]);
    Ret::default()
}

/// Where `_qsort`'s partition loop goes on (its jump targets).
#[derive(Clone, Copy, PartialEq, Eq)]
enum Step {
    /// `00ec6fa8`: a new range.
    Range,
    /// `00ec7020`: the scan from the low end, while the middle is above it.
    ScanLow,
    /// `00ec7040`: the scan from the low end above the middle.
    ScanLowAbove,
    /// `00ec7054`: the scan from the high end.
    ScanHigh,
    /// `00ec7141`: the next range from the stack.
    Pop,
}

// Translated from 00ec6f20 (disassembled, FalloutNV.exe 1.4.0.525)
/// `_qsort` (Ghidra's function-ID label), cdecl: base, element count, element size,
/// comparison function (cdecl, two element addresses).
///
/// A null base with a nonzero count, then a size of 0 or a null comparison function, sets
/// `errno` to `EINVAL` (0x16) and calls the invalid-parameter report with five zeros. Fewer
/// than two elements do nothing. Otherwise the range from the base to the last element
/// (`(count - 1) * size + base`) is sorted: a range of at most 8 elements (`(hi - lo) / size
/// + 1`, unsigned) goes to `shortsort`; a longer one takes the middle element (`(n >> 1) *
/// size + lo`), orders the first, middle and last with three compares and `swap`s (first with
/// middle, first with last, middle with last, each when the compare is positive) and
/// partitions: the low cursor moves up over elements that compare at or below the middle
/// (first up to the middle, then up to the last element), the high cursor down over those
/// above it (down to the middle); while they have not crossed the two are exchanged byte by
/// byte (the middle follows an exchanged element). Then the high cursor moves back down over
/// elements equal to the middle (to the middle, then to the first element), and of the two
/// parts the larger is pushed on the 30-entry stacks of the frame (low ends at `EBP-0x88`,
/// high ends at `EBP-0x100`) and the smaller sorted next; a part of one element is neither.
/// When nothing is left the next range is popped, until the stack is empty. The stack's depth
/// stays below 30 (the smaller part is the one sorted on), so it is kept here as a list.
pub fn qsort(e: &mut Engine, base: u32, num: u32, width: u32, compare: u32) {
    if base == 0 && num != 0 {
        let slot = e.call(ERRNO_ADDR, &args![]).u32();
        e.mem.set_u32(slot, 0x16);
        e.call(INVALID_PARAMETER_ADDR, &args![0u32, 0u32, 0u32, 0u32, 0u32]);
        return;
    }
    if width == 0 || compare == 0 {
        let slot = e.call(ERRNO_ADDR, &args![]).u32();
        e.mem.set_u32(slot, 0x16);
        e.call(INVALID_PARAMETER_ADDR, &args![0u32, 0u32, 0u32, 0u32, 0u32]);
        return;
    }
    if num < 2 {
        return;
    }
    let mut lo = base;
    let mut hi = num.wrapping_sub(1).wrapping_mul(width).wrapping_add(base);
    // The two stacks, `[EBP-0x88 + 4 * i]` and `[EBP-0x100 + 4 * i]`, and their count
    // `[EBP-0xc]`.
    let mut stack: Vec<(u32, u32)> = Vec::new();
    let (mut loguy, mut higuy, mut mid) = (0u32, 0u32, 0u32);
    let mut step = Step::Range;
    loop {
        match step {
            Step::Range => {
                let size = hi.wrapping_sub(lo) / width + 1;
                if size <= 8 {
                    e.call(SHORTSORT_ADDR, &args![hi, lo, width, compare]);
                    step = Step::Pop;
                    continue;
                }
                mid = (size >> 1).wrapping_mul(width).wrapping_add(lo);
                if e.call(compare, &args![lo, mid]).i32() > 0 {
                    e.call(SWAP_ADDR, &args![lo, mid, width]);
                }
                if e.call(compare, &args![lo, hi]).i32() > 0 {
                    e.call(SWAP_ADDR, &args![lo, hi, width]);
                }
                if e.call(compare, &args![mid, hi]).i32() > 0 {
                    e.call(SWAP_ADDR, &args![mid, hi, width]);
                }
                loguy = lo;
                higuy = hi;
                step = Step::ScanLow;
            }
            Step::ScanLow => {
                step = Step::ScanLowAbove;
                if mid > loguy {
                    loop {
                        loguy = loguy.wrapping_add(width);
                        if loguy >= mid {
                            break;
                        }
                        if e.call(compare, &args![loguy, mid]).i32() > 0 {
                            if mid > loguy {
                                step = Step::ScanHigh;
                            }
                            break;
                        }
                    }
                }
            }
            Step::ScanLowAbove => {
                loop {
                    loguy = loguy.wrapping_add(width);
                    if loguy > hi {
                        break;
                    }
                    if e.call(compare, &args![loguy, mid]).i32() > 0 {
                        break;
                    }
                }
                step = Step::ScanHigh;
            }
            Step::ScanHigh => {
                loop {
                    higuy = higuy.wrapping_sub(width);
                    if higuy <= mid {
                        break;
                    }
                    if e.call(compare, &args![higuy, mid]).i32() <= 0 {
                        break;
                    }
                }
                if loguy <= higuy {
                    if loguy != higuy {
                        // 00ec7072: the bytes exchanged inline, the count kept at [EBP-0x10].
                        let distance = loguy.wrapping_sub(higuy);
                        let mut at = higuy;
                        let mut left = width;
                        loop {
                            left = left.wrapping_sub(1);
                            let low = e.mem.u8(distance.wrapping_add(at));
                            let high = e.mem.u8(at);
                            e.mem.set_u8(distance.wrapping_add(at), high);
                            e.mem.set_u8(at, low);
                            at = at.wrapping_add(1);
                            if left == 0 {
                                break;
                            }
                        }
                    }
                    if mid == higuy {
                        mid = loguy;
                    }
                    step = Step::ScanLow;
                    continue;
                }
                // 00ec709d: back down over elements equal to the middle.
                higuy = higuy.wrapping_add(width);
                let mut above = false;
                if mid < higuy {
                    loop {
                        higuy = higuy.wrapping_sub(width);
                        if higuy <= mid {
                            break;
                        }
                        if e.call(compare, &args![higuy, mid]).i32() != 0 {
                            above = mid < higuy;
                            break;
                        }
                    }
                }
                if !above {
                    loop {
                        higuy = higuy.wrapping_sub(width);
                        if higuy <= lo {
                            break;
                        }
                        if e.call(compare, &args![higuy, mid]).i32() != 0 {
                            break;
                        }
                    }
                }
                // 00ec70d9: the larger part pushed, the smaller sorted next.
                let high_part = hi.wrapping_sub(loguy) as i32;
                let low_part = higuy.wrapping_sub(lo) as i32;
                if low_part >= high_part {
                    if lo < higuy {
                        stack.push((lo, higuy));
                    }
                    if loguy < hi {
                        lo = loguy;
                        step = Step::Range;
                    } else {
                        step = Step::Pop;
                    }
                } else {
                    if loguy < hi {
                        stack.push((loguy, hi));
                    }
                    if lo < higuy {
                        hi = higuy;
                        step = Step::Range;
                    } else {
                        step = Step::Pop;
                    }
                }
            }
            Step::Pop => match stack.pop() {
                Some((low, high)) => {
                    lo = low;
                    hi = high;
                    step = Step::Range;
                }
                None => return,
            },
        }
    }
}

pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00ec_6f20, qsort(u32, u32, u32, u32)),
        (SHORTSORT_ADDR, shortsort_entry as AbiFn),
        (SWAP_ADDR, swap_entry as AbiFn),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    type Calls = Vec<Vec<u32>>;

    /// The comparison function's address in the tests (a double).
    const COMPARE: u32 = 0x0070_0000;

    fn calls(e: &Engine, addr: u32) -> Calls {
        let log = e.call_log.as_ref().unwrap();
        log.iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    /// An engine with the sort's helpers registered, `_errno` doubled to one slot, the
    /// invalid-parameter report doubled to do nothing and the comparison function doubled to
    /// compare the elements' first dwords as signed numbers (-1, 0 or 1). Returns the engine
    /// and the `errno` slot.
    fn engine() -> (Engine, u32) {
        let mut e = Engine::new();
        for (addr, f) in funcs() {
            e.register(addr, f);
        }
        let slot = e.mem.alloc(4);
        e.register_double(ERRNO_ADDR, move |_, _| Ret {
            eax: slot,
            ..Ret::default()
        });
        e.register_double(INVALID_PARAMETER_ADDR, |_, _| Ret::default());
        e.register_double(COMPARE, |e, a| {
            let x = e.mem.u32(a[0]) as i32;
            let y = e.mem.u32(a[1]) as i32;
            Ret {
                eax: x.cmp(&y) as i32 as u32,
                ..Ret::default()
            }
        });
        (e, slot)
    }

    /// The values as `width`-byte elements (the value in the first dword, the element's index
    /// in the second when there is room, to see that whole elements move).
    fn array(e: &mut Engine, values: &[i32], width: u32) -> u32 {
        let base = e.mem.alloc(width * values.len().max(1) as u32);
        for (i, &v) in values.iter().enumerate() {
            let at = base + width * i as u32;
            e.mem.set_u32(at, v as u32);
            if width >= 8 {
                e.mem.set_u32(at + 4, 0x100 + i as u32);
            }
        }
        base
    }

    fn read(e: &Engine, base: u32, n: usize, width: u32) -> Vec<(i32, u32)> {
        (0..n as u32)
            .map(|i| {
                let at = base + width * i;
                let tag = if width >= 8 { e.mem.u32(at + 4) } else { 0 };
                (e.mem.u32(at) as i32, tag)
            })
            .collect()
    }

    /// Sorts `values` with `_qsort`; the sorted values and the elements' tags.
    fn sorted(values: &[i32], width: u32) -> Vec<(i32, u32)> {
        let (mut e, _) = engine();
        let base = array(&mut e, values, width);
        qsort(&mut e, base, values.len() as u32, width, COMPARE);
        read(&e, base, values.len(), width)
    }

    /// A small deterministic generator (xorshift) for test data.
    fn data(n: usize, seed: u32, modulo: u32) -> Vec<i32> {
        let mut s = seed;
        (0..n)
            .map(|_| {
                s ^= s << 13;
                s ^= s >> 17;
                s ^= s << 5;
                (s % modulo) as i32 - (modulo / 2) as i32
            })
            .collect()
    }

    #[test]
    fn qsort_sorts_and_moves_whole_elements() {
        for n in 0..40usize {
            for (seed, modulo) in [(1u32, 1000u32), (7, 5), (99, 2)] {
                let v = data(n, seed + n as u32, modulo);
                let out = sorted(&v, 8);
                let mut want = v.clone();
                want.sort();
                let values: Vec<i32> = out.iter().map(|p| p.0).collect();
                assert_eq!(values, want, "n {n} seed {seed}");
                // Every element's tag came along with its value.
                for (value, tag) in &out {
                    assert_eq!(v[(tag - 0x100) as usize], *value);
                }
            }
        }
    }

    #[test]
    fn qsort_of_sorted_reverse_and_equal_input() {
        let up: Vec<i32> = (0..100).collect();
        let down: Vec<i32> = (0..100).rev().collect();
        let same = vec![5; 50];
        let want: Vec<i32> = (0..100).collect();
        let values = |o: Vec<(i32, u32)>| o.into_iter().map(|p| p.0).collect::<Vec<i32>>();
        assert_eq!(values(sorted(&up, 4)), want);
        assert_eq!(values(sorted(&down, 4)), want);
        assert_eq!(values(sorted(&same, 4)), same);
        // Odd element sizes.
        let v = data(33, 3, 50);
        let mut w = v.clone();
        w.sort();
        assert_eq!(values(sorted(&v, 12)), w);
    }

    #[test]
    fn qsort_small_ranges_go_to_shortsort_whole() {
        // Eight elements: one shortsort call (EAX the last element) and no swap.
        let (mut e, _) = engine();
        let base = array(&mut e, &[3, 1, 2, 8, 7, 6, 5, 4], 4);
        e.call_log = Some(vec![]);
        qsort(&mut e, base, 8, 4, COMPARE);
        assert_eq!(
            calls(&e, SHORTSORT_ADDR),
            vec![vec![base + 28, base, 4, COMPARE]]
        );
        assert!(calls(&e, SWAP_ADDR).is_empty());
        let values: Vec<i32> = read(&e, base, 8, 4).iter().map(|p| p.0).collect();
        assert_eq!(values, vec![1, 2, 3, 4, 5, 6, 7, 8]);
    }

    #[test]
    fn qsort_nine_elements_take_the_median_of_three() {
        // Nine elements: the middle is index 4; the first compares are (0, 4), (0, 8), (4, 8).
        let (mut e, _) = engine();
        let base = array(&mut e, &[9, 2, 3, 4, 1, 6, 7, 8, 5], 4);
        e.call_log = Some(vec![]);
        qsort(&mut e, base, 9, 4, COMPARE);
        let first: Calls = calls(&e, COMPARE).into_iter().take(3).collect();
        assert_eq!(
            first,
            vec![
                vec![base, base + 16],
                vec![base, base + 32],
                vec![base + 16, base + 32],
            ]
        );
        // 9 > 1: first and middle exchanged; then 1 < 5; then 9 > 5: middle and last.
        assert_eq!(
            calls(&e, SWAP_ADDR),
            vec![vec![base, base + 16, 4], vec![base + 16, base + 32, 4]]
        );
        let values: Vec<i32> = read(&e, base, 9, 4).iter().map(|p| p.0).collect();
        assert_eq!(values, (1..=9).collect::<Vec<i32>>());
    }

    #[test]
    fn qsort_fewer_than_two_elements_does_nothing() {
        for n in [0u32, 1] {
            let (mut e, slot) = engine();
            let base = array(&mut e, &[4], 4);
            e.call_log = Some(vec![]);
            qsort(&mut e, base, n, 4, COMPARE);
            assert!(e.call_log.as_ref().unwrap().is_empty());
            assert_eq!(e.mem.u32(slot), 0);
        }
        // A null base with no elements is not an error.
        let (mut e, _) = engine();
        e.call_log = Some(vec![]);
        qsort(&mut e, 0, 0, 4, COMPARE);
        assert!(e.call_log.as_ref().unwrap().is_empty());
    }

    #[test]
    fn qsort_bad_arguments_report_einval() {
        type Case = (u32, u32, u32, u32);
        let base = 0x0001_0000;
        let cases: [Case; 4] = [
            (0, 3, 4, COMPARE),
            (base, 4, 0, COMPARE),
            (base, 4, 4, 0),
            (base, 1, 0, 0),
        ];
        for (b, n, w, c) in cases {
            let (mut e, slot) = engine();
            e.call_log = Some(vec![]);
            qsort(&mut e, b, n, w, c);
            let order: Vec<u32> = e.call_log.as_ref().unwrap().iter().map(|c| c.0).collect();
            assert_eq!(order, vec![ERRNO_ADDR, INVALID_PARAMETER_ADDR]);
            assert_eq!(calls(&e, INVALID_PARAMETER_ADDR), vec![vec![0; 5]]);
            assert_eq!(e.mem.u32(slot), 0x16);
        }
    }

    #[test]
    fn shortsort_selects_the_largest_downwards() {
        // Three elements [2, 3, 1]: the largest (3, index 1) goes last, then [2, 1] -> [1, 2].
        let (mut e, _) = engine();
        let base = array(&mut e, &[2, 3, 1], 8);
        e.call_log = Some(vec![]);
        shortsort(&mut e, base + 16, base, 8, COMPARE);
        assert_eq!(
            calls(&e, COMPARE),
            vec![
                vec![base + 8, base],
                vec![base + 16, base + 8],
                vec![base + 8, base],
            ]
        );
        assert_eq!(
            read(&e, base, 3, 8),
            vec![(1, 0x102), (2, 0x100), (3, 0x101)]
        );
        // Equal elements are not exchanged: the first of them stays the largest.
        let base = array(&mut e, &[5, 5], 8);
        shortsort(&mut e, base + 8, base, 8, COMPARE);
        assert_eq!(read(&e, base, 2, 8), vec![(5, 0x101), (5, 0x100)]);
        // The last at or below the first does nothing.
        e.call_log = Some(vec![]);
        shortsort(&mut e, base, base, 8, COMPARE);
        assert!(e.call_log.as_ref().unwrap().is_empty());
    }

    #[test]
    fn swap_exchanges_bytes_unless_same_or_empty() {
        let mut e = Engine::new();
        let a = e.mem.alloc(8);
        e.mem.write(a, &[1, 2, 3, 4, 5, 6, 7, 8]);
        swap(&mut e, a, a + 4, 3);
        assert_eq!(e.mem.bytes(a, 8), vec![5, 6, 7, 4, 1, 2, 3, 8]);
        swap(&mut e, a, a, 4);
        swap(&mut e, a, a + 4, 0);
        assert_eq!(e.mem.bytes(a, 8), vec![5, 6, 7, 4, 1, 2, 3, 8]);
    }

    #[test]
    fn sort_functions_are_registered() {
        let table = funcs();
        for addr in [0x00ec_6f20, 0x00ec_6e90, 0x00ec_6e60] {
            assert!(table.iter().any(|f| f.0 == addr), "{addr:08x}");
        }
    }
}

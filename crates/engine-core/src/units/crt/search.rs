//! The C runtime's binary search (MSVC 2008 LIBCMT, statically linked into FalloutNV.exe
//! 1.4.0.525), translated from the exe: `_bsearch` (`00ec716d`, Ghidra's function-ID label,
//! "Library Function - Single Match", Visual Studio 2008 Release).

use super::*;

/// `_errno` (`00ec85e3`, cdecl, no arguments: the address of `errno`) and the
/// invalid-parameter report (`00ec7c30`, cdecl, five words): translated elsewhere.
const ERRNO_ADDR: u32 = 0x00ec_85e3;
const INVALID_PARAMETER_ADDR: u32 = 0x00ec_7c30;

// Translated from 00ec716d (decompiled, FalloutNV.exe 1.4.0.525)
/// `_bsearch` (Ghidra's function-ID label), cdecl: key, base, element count, element size,
/// comparison function; the matching element's address or 0.
///
/// The last element's address `(count - 1) * size + base` is taken first (32-bit, wrapping).
/// A null base with a nonzero count, a size of 0 or a null comparison function sets `errno`
/// to `EINVAL` (0x16), calls the invalid-parameter report with five zeros and answers 0
/// (`_errno` is called first and the store made before the report). Otherwise, while the low
/// address is at or below the high one (unsigned): half the count; with no half left, a count
/// of 0 answers 0 and a count of 1 compares the low element and answers it if the comparison
/// gives 0, else 0. Otherwise the middle element is `half` elements above the low one for an
/// odd count and `half - 1` for an even one; the comparison function is called by address,
/// cdecl, with the key and that element (`compare(key, element)`). 0 answers the element; a
/// negative result (signed) makes the high address the element before it and the count `half`
/// (odd) or `half - 1` (even); a positive one makes the low address the element after it and
/// the count `half`.
pub fn bsearch(e: &mut Engine, key: u32, base: u32, num: u32, width: u32, compare: u32) -> u32 {
    let mut lo = base;
    let mut num = num;
    let mut hi = num.wrapping_sub(1).wrapping_mul(width).wrapping_add(lo);
    if (lo == 0 && num != 0) || width == 0 || compare == 0 {
        let slot = e.call(ERRNO_ADDR, &args![]).u32();
        e.mem.set_u32(slot, 0x16);
        e.call(INVALID_PARAMETER_ADDR, &args![0u32, 0u32, 0u32, 0u32, 0u32]);
        return 0;
    }
    while lo <= hi {
        let half = num >> 1;
        if half == 0 {
            if num == 0 {
                return 0;
            }
            let result = e.call(compare, &args![key, lo]).u32();
            // NEG / SBB / NOT / AND: the low address when the comparison gave 0.
            return if result != 0 { 0 } else { lo };
        }
        let odd = num & 1 != 0;
        let step = if odd { half } else { half - 1 };
        let mid = step.wrapping_mul(width).wrapping_add(lo);
        let result = e.call(compare, &args![key, mid]).i32();
        if result == 0 {
            return mid;
        }
        if result < 0 {
            hi = mid.wrapping_sub(width);
            num = if odd { half } else { half - 1 };
        } else {
            lo = mid.wrapping_add(width);
            num = half;
        }
    }
    0
}

pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00ec_716d, bsearch(u32, u32, u32, u32, u32) -> u32)]
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

    /// An engine with `_errno` doubled to one slot, the invalid-parameter report doubled to do
    /// nothing, and the comparison function doubled to compare the key's dword with the
    /// element's as signed numbers (key minus element, so results other than -1 and 1 occur).
    /// The call log is on. Returns the engine and the `errno` slot.
    fn engine() -> (Engine, u32) {
        let mut e = Engine::new();
        let slot = e.mem.alloc(4);
        e.register_double(ERRNO_ADDR, move |_, _| Ret {
            eax: slot,
            ..Ret::default()
        });
        e.register_double(INVALID_PARAMETER_ADDR, |_, _| Ret::default());
        e.register_double(COMPARE, |e, a| {
            let key = e.mem.u32(a[0]) as i32;
            let element = e.mem.u32(a[1]) as i32;
            Ret {
                eax: key.wrapping_sub(element) as u32,
                ..Ret::default()
            }
        });
        e.call_log = Some(vec![]);
        (e, slot)
    }

    /// The values as an array of `width`-byte elements (the value in the first dword).
    fn array(e: &mut Engine, values: &[u32], width: u32) -> u32 {
        let base = e.mem.alloc(width * values.len().max(1) as u32);
        for (i, &v) in values.iter().enumerate() {
            e.mem.set_u32(base + width * i as u32, v);
        }
        base
    }

    fn key(e: &mut Engine, value: u32) -> u32 {
        let at = e.mem.alloc(4);
        e.mem.set_u32(at, value);
        at
    }

    /// Searches `values` (elements of `width` bytes) for `wanted`: the answer as an element
    /// index (or `None` for 0) and the indices the comparison function was given, in order.
    fn search(values: &[u32], width: u32, wanted: u32) -> (Option<u32>, Vec<u32>) {
        let (mut e, _) = engine();
        let base = array(&mut e, values, width);
        let k = key(&mut e, wanted);
        e.call_log = Some(vec![]);
        let found = bsearch(&mut e, k, base, values.len() as u32, width, COMPARE);
        let order = calls(&e, COMPARE)
            .iter()
            .map(|a| {
                assert_eq!(a.len(), 2);
                assert_eq!(a[0], k, "the key goes first");
                assert_eq!((a[1] - base) % width, 0);
                (a[1] - base) / width
            })
            .collect();
        let index = if found == 0 {
            None
        } else {
            assert_eq!((found - base) % width, 0);
            Some((found - base) / width)
        };
        assert!(calls(&e, INVALID_PARAMETER_ADDR).is_empty());
        (index, order)
    }

    fn tens(n: u32) -> Vec<u32> {
        (1..=n).map(|i| 10 * i).collect()
    }

    #[test]
    fn bsearch_even_count_halves_below_the_middle() {
        // Eight elements: the first compare is index 3 (half 4, even: half - 1).
        let v = tens(8);
        assert_eq!(search(&v, 4, 40), (Some(3), vec![3]));
        assert_eq!(search(&v, 4, 5), (None, vec![3, 1, 0]));
        assert_eq!(search(&v, 4, 10), (Some(0), vec![3, 1, 0]));
        assert_eq!(search(&v, 4, 20), (Some(1), vec![3, 1]));
        assert_eq!(search(&v, 4, 30), (Some(2), vec![3, 1, 2]));
        assert_eq!(search(&v, 4, 50), (Some(4), vec![3, 5, 4]));
        assert_eq!(search(&v, 4, 60), (Some(5), vec![3, 5]));
        assert_eq!(search(&v, 4, 70), (Some(6), vec![3, 5, 6]));
        assert_eq!(search(&v, 4, 80), (Some(7), vec![3, 5, 6, 7]));
        assert_eq!(search(&v, 4, 85), (None, vec![3, 5, 6, 7]));
        assert_eq!(search(&v, 4, 45), (None, vec![3, 5, 4]));
        // Two elements: index 0 first; below it the count becomes 0.
        let two = tens(2);
        assert_eq!(search(&two, 4, 5), (None, vec![0]));
        assert_eq!(search(&two, 4, 10), (Some(0), vec![0]));
        assert_eq!(search(&two, 4, 20), (Some(1), vec![0, 1]));
        assert_eq!(search(&two, 4, 25), (None, vec![0, 1]));
    }

    #[test]
    fn bsearch_odd_count_compares_the_true_middle() {
        // Seven elements: index 3 first (half 3, odd).
        let v = tens(7);
        assert_eq!(search(&v, 4, 40), (Some(3), vec![3]));
        assert_eq!(search(&v, 4, 5), (None, vec![3, 1, 0]));
        assert_eq!(search(&v, 4, 10), (Some(0), vec![3, 1, 0]));
        assert_eq!(search(&v, 4, 20), (Some(1), vec![3, 1]));
        assert_eq!(search(&v, 4, 30), (Some(2), vec![3, 1, 2]));
        assert_eq!(search(&v, 4, 50), (Some(4), vec![3, 5, 4]));
        assert_eq!(search(&v, 4, 60), (Some(5), vec![3, 5]));
        assert_eq!(search(&v, 4, 70), (Some(6), vec![3, 5, 6]));
        assert_eq!(search(&v, 4, 75), (None, vec![3, 5, 6]));
        // Three elements and nine.
        let three = tens(3);
        assert_eq!(search(&three, 4, 5), (None, vec![1, 0]));
        assert_eq!(search(&three, 4, 35), (None, vec![1, 2]));
        let nine = tens(9);
        assert_eq!(search(&nine, 4, 90), (Some(8), vec![4, 6, 7, 8]));
        assert_eq!(search(&nine, 4, 10), (Some(0), vec![4, 1, 0]));
        assert_eq!(search(&nine, 4, 25), (None, vec![4, 1, 2]));
    }

    #[test]
    fn bsearch_wide_elements_step_by_the_size() {
        // Twelve-byte elements, six of them: index 2 first.
        let v = tens(6);
        assert_eq!(search(&v, 12, 30), (Some(2), vec![2]));
        assert_eq!(search(&v, 12, 50), (Some(4), vec![2, 4]));
        assert_eq!(search(&v, 12, 60), (Some(5), vec![2, 4, 5]));
        assert_eq!(search(&v, 12, 15), (None, vec![2, 0, 1]));
    }

    #[test]
    fn bsearch_duplicates_answer_the_first_one_compared() {
        assert_eq!(search(&[1, 2, 2, 2, 2, 3], 4, 2), (Some(2), vec![2]));
        assert_eq!(search(&[2, 2, 2, 2], 4, 2), (Some(1), vec![1]));
        assert_eq!(
            search(&[1, 1, 1, 2, 2, 2, 2, 2], 4, 1),
            (Some(1), vec![3, 1])
        );
        assert_eq!(search(&[1, 2, 2, 2, 2, 2, 2, 2], 4, 2), (Some(3), vec![3]));
        assert_eq!(search(&[1, 1, 1, 1, 1, 2, 2], 4, 2), (Some(5), vec![3, 5]));
    }

    #[test]
    fn bsearch_one_element_compares_it_once() {
        assert_eq!(search(&[7], 4, 7), (Some(0), vec![0]));
        assert_eq!(search(&[7], 4, 3), (None, vec![0]));
        assert_eq!(search(&[7], 4, 9), (None, vec![0]));
    }

    #[test]
    fn bsearch_zero_elements_compares_nothing() {
        assert_eq!(search(&[], 4, 7), (None, vec![]));
        // A null base with no elements is allowed: the last element's address wraps above
        // the base, the loop runs once and the count of 0 answers 0.
        let (mut e, slot) = engine();
        e.mem.set_u32(slot, 0);
        let k = key(&mut e, 7);
        e.call_log = Some(vec![]);
        assert_eq!(bsearch(&mut e, k, 0, 0, 4, COMPARE), 0);
        assert!(calls(&e, COMPARE).is_empty());
        assert!(calls(&e, INVALID_PARAMETER_ADDR).is_empty());
        assert!(calls(&e, ERRNO_ADDR).is_empty());
        assert_eq!(e.mem.u32(slot), 0);
    }

    #[test]
    fn bsearch_last_address_wrapping_below_the_base_finds_nothing() {
        // 0xffffff00 + 0xff * 4 wraps to 0x2fc, below the base: 0 without a compare.
        let (mut e, _) = engine();
        let k = key(&mut e, 7);
        e.call_log = Some(vec![]);
        assert_eq!(bsearch(&mut e, k, 0xffff_ff00, 0x100, 4, COMPARE), 0);
        assert!(calls(&e, COMPARE).is_empty());
        assert!(calls(&e, INVALID_PARAMETER_ADDR).is_empty());
    }

    #[test]
    fn bsearch_finds_every_element_and_misses_every_gap() {
        for n in 0..=17u32 {
            let v = tens(n);
            for wanted in (0..=10 * n + 10).step_by(5) {
                let (found, order) = search(&v, 4, wanted);
                let expected = if wanted % 10 == 0 && (10..=10 * n).contains(&wanted) {
                    Some(wanted / 10 - 1)
                } else {
                    None
                };
                assert_eq!(found, expected, "n {n} key {wanted}");
                assert!(order.len() <= 6, "n {n} key {wanted}: {order:?}");
            }
        }
    }

    #[test]
    fn bsearch_bad_arguments_report_einval() {
        type Case = (u32, u32, u32, u32);
        // (base, count, size, compare); nothing is read, so the base need not be mapped.
        let base = 0x0001_0000;
        let cases: [Case; 4] = [
            (0, 3, 4, COMPARE),
            (base, 4, 0, COMPARE),
            (base, 4, 4, 0),
            (0, 0, 0, 0),
        ];
        for (b, n, w, c) in cases {
            let (mut e, slot) = engine();
            let k = key(&mut e, 20);
            e.mem.set_u32(slot, 0);
            e.call_log = Some(vec![]);
            assert_eq!(bsearch(&mut e, k, b, n, w, c), 0);
            let order: Vec<u32> = e.call_log.as_ref().unwrap().iter().map(|c| c.0).collect();
            assert_eq!(order, vec![ERRNO_ADDR, INVALID_PARAMETER_ADDR]);
            assert_eq!(calls(&e, INVALID_PARAMETER_ADDR), vec![vec![0; 5]]);
            assert_eq!(e.mem.u32(slot), 0x16);
        }
    }

    #[test]
    fn bsearch_result_sign_is_signed() {
        // A comparison answering 0x80000000 (negative) moves the high end down.
        let (mut e, _) = engine();
        let base = array(&mut e, &tens(4), 4);
        let k = key(&mut e, 0);
        e.register_double(COMPARE, |_, _| Ret {
            eax: 0x8000_0000,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        assert_eq!(bsearch(&mut e, k, base, 4, 4, COMPARE), 0);
        let order: Vec<u32> = calls(&e, COMPARE)
            .iter()
            .map(|a| (a[1] - base) / 4)
            .collect();
        assert_eq!(order, vec![1, 0]);
    }

    #[test]
    fn bsearch_is_registered() {
        assert!(funcs().iter().any(|f| f.0 == 0x00ec_716d));
    }
}

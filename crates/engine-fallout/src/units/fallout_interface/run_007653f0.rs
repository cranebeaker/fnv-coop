//! `fallout/interface/run_007653f0` (Xbox PDB source unit), subsystem `fallout/interface`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 007653f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No Xbox PDB name: a Shell sort of `count` words at `array` with the cdecl compare
/// function `compare(a, b)` (negative: `a` sorts first). `__thiscall`, but `this` is
/// never read (`RET 0xc`). The gap starts at 1 and grows as `3h + 1` while it is at most
/// `(count - 1) / 9`, then shrinks by thirds; each pass is an insertion sort over the
/// elements one gap apart.
pub fn fn_007653f0(e: &mut Engine, _this: Ptr, array: Ptr, compare: u32, count: i32) {
    let low: i32 = 0;
    let high = count.wrapping_sub(1);
    let base = array.addr();
    let slot = |i: i32| base.wrapping_add((i as u32).wrapping_mul(4));
    let mut gap: i32 = 1;
    while gap <= high.wrapping_sub(low) / 9 {
        gap = gap.wrapping_mul(3).wrapping_add(1);
    }
    while gap > 0 {
        let mut i = low.wrapping_add(gap);
        while i <= high {
            let mut j = i;
            let value = e.mem.u32(slot(i));
            while j >= low.wrapping_add(gap) {
                let other = e.mem.u32(slot(j.wrapping_sub(gap)));
                if e.call(compare, &args![value, other]).i32() >= 0 {
                    break;
                }
                e.mem.set_u32(slot(j), other);
                j = j.wrapping_sub(gap);
            }
            e.mem.set_u32(slot(j), value);
            i = i.wrapping_add(1);
        }
        gap /= 3;
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0076_53f0, fn_007653f0(Ptr, Ptr, u32, i32))]
}

#[cfg(test)]
mod tests {
    use super::*;

    const COMPARE: u32 = 0x0090_0000;

    fn compare_words(_: &mut Engine, a: &[u32]) -> Ret {
        Ret {
            eax: (a[0] as i32).wrapping_sub(a[1] as i32) as u32,
            ..Ret::default()
        }
    }

    fn sort(values: &[u32]) -> Vec<u32> {
        let mut e = Engine::new();
        e.register(COMPARE, compare_words);
        let array = e.mem.alloc(4 * values.len() as u32 + 4);
        for (i, v) in values.iter().enumerate() {
            e.mem.set_u32(array + 4 * i as u32, *v);
        }
        e.mem.set_u32(array + 4 * values.len() as u32, 0xdead);
        e.call(
            0x0076_53f0,
            &args![0u32, array, COMPARE, values.len() as u32],
        );
        assert_eq!(e.mem.u32(array + 4 * values.len() as u32), 0xdead);
        (0..values.len() as u32)
            .map(|i| e.mem.u32(array + 4 * i))
            .collect()
    }

    #[test]
    fn fn_007653f0_sorts_with_the_compare_function() {
        assert_eq!(sort(&[]), Vec::<u32>::new());
        assert_eq!(sort(&[5]), vec![5]);
        assert_eq!(sort(&[3, 1, 2]), vec![1, 2, 3]);
        // Long enough for gaps 4 and 13 (count - 1 >= 36 gives gap 13).
        let values: Vec<u32> = (0..40).map(|i| (i * 17 + 5) % 40).collect();
        let mut want = values.clone();
        want.sort();
        assert_eq!(sort(&values), want);
    }
}

//! `Havok SDK/run_00d74a20` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `00c905b0` (`hkarrayutil.cpp`): `thiscall`, no stack words; releases the object it is given
/// (called with each non-null pointer of an element that is going away).
const RELEASE: u32 = 0x00c9_05b0;

// Translated from 00d74a20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Drops from the back of an array of 12-byte elements (data pointer at `this`, count at
/// `+4`) every element whose third word is below 2 (signed): the two pointers at the
/// element's start are released through `00c905b0` when not null, the count shrinks, and
/// unless the element was the last the last element is copied into its place (word by word).
/// The class is unconfirmed.
pub fn fn_00d74a20(e: &mut Engine, this: Ptr) {
    let this = this.addr();
    let mut index = e.mem.u32(this.wrapping_add(4)).wrapping_sub(1) as i32;
    if index < 0 {
        return;
    }
    let mut offset = (index as u32).wrapping_mul(12);
    loop {
        let element = e.mem.u32(this).wrapping_add(offset);
        if e.mem.i32(element.wrapping_add(8)) < 2 {
            let first = e.mem.u32(element);
            if first != 0 {
                e.call(RELEASE, &args![first]);
            }
            let second = e
                .mem
                .u32(offset.wrapping_add(e.mem.u32(this)).wrapping_add(4));
            if second != 0 {
                e.call(RELEASE, &args![second]);
            }
            let count = e.mem.u32(this.wrapping_add(4)).wrapping_sub(1);
            e.mem.set_u32(this.wrapping_add(4), count);
            if count != index as u32 {
                let data = e.mem.u32(this);
                let mut target = data.wrapping_add(offset);
                let distance = data
                    .wrapping_add(count.wrapping_mul(3).wrapping_mul(4))
                    .wrapping_sub(target);
                for _ in 0..3 {
                    let word = e.mem.u32(distance.wrapping_add(target));
                    e.mem.set_u32(target, word);
                    target = target.wrapping_add(4);
                }
            }
        }
        index = index.wrapping_sub(1);
        offset = offset.wrapping_sub(12);
        if index < 0 {
            break;
        }
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00d7_4a20, fn_00d74a20(Ptr))]
}

#[cfg(test)]
mod a09_tests {
    use super::*;

    /// An array of the given 12-byte elements `(first, second, count_word)`.
    fn array(e: &mut Engine, elements: &[(u32, u32, u32)]) -> u32 {
        let this = e.mem.alloc(8);
        let data = e.mem.alloc(12 * elements.len() as u32);
        for (i, (a, b, c)) in elements.iter().enumerate() {
            let at = data + 12 * i as u32;
            e.mem.set_u32(at, *a);
            e.mem.set_u32(at + 4, *b);
            e.mem.set_u32(at + 8, *c);
        }
        e.mem.set_u32(this, data);
        e.mem.set_u32(this + 4, elements.len() as u32);
        this
    }

    fn contents(e: &Engine, this: u32) -> Vec<(u32, u32, u32)> {
        let data = e.mem.u32(this);
        (0..e.mem.u32(this + 4))
            .map(|i| {
                let at = data + 12 * i;
                (e.mem.u32(at), e.mem.u32(at + 4), e.mem.u32(at + 8))
            })
            .collect()
    }

    fn released(e: &mut Engine) -> Vec<u32> {
        e.call_log
            .take()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == RELEASE)
            .map(|(_, args)| args[0])
            .collect()
    }

    #[test]
    fn elements_below_two_are_released_and_replaced_by_the_last() {
        let mut e = Engine::new();
        e.register(RELEASE, |_, _| Ret::default());
        let this = array(
            &mut e,
            &[
                (0x11, 0x12, 5),
                (0x21, 0, 1),
                (0, 0x32, 2),
                (0x41, 0x42, 0),
                (0x51, 0x52, 9),
            ],
        );
        e.call_log = Some(vec![]);
        e.call(0x00d7_4a20, &args![this]);
        // Visited from the back: index 3 (both pointers), index 1 (only the first). Index 3
        // takes the last element (0x51), index 1 then takes the new last (0x51 again moved).
        assert_eq!(released(&mut e), vec![0x41, 0x42, 0x21]);
        assert_eq!(
            contents(&e, this),
            vec![(0x11, 0x12, 5), (0x51, 0x52, 9), (0, 0x32, 2)]
        );
    }

    #[test]
    fn the_last_element_is_just_dropped_and_an_empty_array_does_nothing() {
        let mut e = Engine::new();
        e.register(RELEASE, |_, _| Ret::default());
        let this = array(&mut e, &[(0x11, 0x12, 7), (0x21, 0x22, 1)]);
        e.call_log = Some(vec![]);
        e.call(0x00d7_4a20, &args![this]);
        assert_eq!(released(&mut e), vec![0x21, 0x22]);
        assert_eq!(contents(&e, this), vec![(0x11, 0x12, 7)]);
        let empty = array(&mut e, &[]);
        e.call_log = Some(vec![]);
        e.call(0x00d7_4a20, &args![empty]);
        assert!(released(&mut e).is_empty());
        // A negative word (signed compare) is below 2 as well.
        let negative = array(&mut e, &[(0x61, 0, 0xffff_ffff)]);
        e.call_log = Some(vec![]);
        e.call(0x00d7_4a20, &args![negative]);
        assert_eq!(released(&mut e), vec![0x61]);
        assert!(contents(&e, negative).is_empty());
    }
}

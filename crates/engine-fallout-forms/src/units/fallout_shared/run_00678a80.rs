//! `fallout shared/run_00678a80` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// One key of `00678a80`'s comparison: `getter` on both objects (an `i32`,
/// or a byte sign-extended with `byte`). Unless this object's key is
/// `unset_mine` and the other's is -1, differing keys decide: this key -1
/// gives 1, the other's -1 gives -1, else -1 when this key is smaller and
/// 1 otherwise. `None` when the key does not decide.
fn compare_key(
    e: &mut Engine,
    mine: u32,
    theirs: u32,
    getter: u32,
    byte: bool,
    unset_mine: i32,
) -> Option<i32> {
    let key = |e: &mut Engine, object: u32| {
        let value = e.call(getter, &args![object]).u32();
        if byte {
            i32::from(value as u8 as i8)
        } else {
            value as i32
        }
    };
    if key(e, mine) == unset_mine && key(e, theirs) == -1 {
        return None;
    }
    if key(e, mine) == key(e, theirs) {
        return None;
    }
    if key(e, mine) == -1 {
        return Some(1);
    }
    if key(e, theirs) == -1 {
        return Some(-1);
    }
    if key(e, mine) < key(e, theirs) {
        Some(-1)
    } else {
        Some(1)
    }
}

// Translated from 00678a80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Orders two package keys (`RET 4`; the engine map gives it no name):
/// this object's key (`0041d8a0`) against `other`, by `00678ca0`, then
/// `00678ce0` (a signed byte; its first test is against 0, not -1), then
/// `00678cc0`, then `0086a460` (a signed byte), each as `compare_key`:
/// -1, 1, or 0 when no key decides.
pub fn fn_00678a80(e: &mut Engine, this: Ptr, other: u32) -> i32 {
    let mine = e.call(0x0041_d8a0, &args![this]).u32();
    for (getter, byte, unset_mine) in [
        (0x0067_8ca0, false, -1),
        (0x0067_8ce0, true, 0),
        (0x0067_8cc0, false, -1),
        (0x0086_a460, true, -1),
    ] {
        if let Some(order) = compare_key(e, mine, other, getter, byte, unset_mine) {
            return order;
        }
    }
    0
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0067_8a80, fn_00678a80(Ptr, u32) -> i32)]
}

#[cfg(test)]
mod tests_n042 {
    use super::*;

    /// Keys: an object's four keys are the words at +0, +4 (byte), +8 and
    /// +0xC (byte); `0041d8a0` returns the word at +0x10.
    fn engine() -> Engine {
        let mut e = Engine::new();
        e.register(0x0041_d8a0, |e, a| e.mem.u32(a[0] + 0x10).into_ret());
        e.register(0x0067_8ca0, |e, a| e.mem.u32(a[0]).into_ret());
        e.register(0x0067_8ce0, |e, a| {
            (0xabcd_ef00 | e.mem.u32(a[0] + 4) & 0xff).into_ret()
        });
        e.register(0x0067_8cc0, |e, a| e.mem.u32(a[0] + 8).into_ret());
        e.register(0x0086_a460, |e, a| {
            (0x1234_5600 | e.mem.u32(a[0] + 0xc) & 0xff).into_ret()
        });
        e
    }

    fn key(e: &mut Engine, keys: [i32; 4]) -> u32 {
        let object = e.mem.alloc(0x14);
        for (i, k) in keys.iter().enumerate() {
            e.mem.set_u32(object + 4 * i as u32, *k as u32);
        }
        object
    }

    fn order(e: &mut Engine, mine: [i32; 4], theirs: [i32; 4]) -> i32 {
        let this = e.mem.alloc(0x14);
        let mine = key(e, mine);
        e.mem.set_u32(this + 0x10, mine);
        let theirs = key(e, theirs);
        e.call(0x0067_8a80, &args![this, theirs]).i32()
    }

    #[test]
    fn keys_decide_in_order() {
        let mut e = engine();
        assert_eq!(order(&mut e, [1, 0, 0, 0], [2, 0, 0, 0]), -1);
        assert_eq!(order(&mut e, [3, 0, 0, 0], [2, 0, 0, 0]), 1);
        assert_eq!(order(&mut e, [-1, 0, 0, 0], [2, 0, 0, 0]), 1);
        assert_eq!(order(&mut e, [2, 0, 0, 0], [-1, 0, 0, 0]), -1);
        // Both -1: the next key decides.
        assert_eq!(order(&mut e, [-1, 1, 0, 0], [-1, 2, 0, 0]), -1);
        // The byte key: 0 against -1 does not decide, -1 against 0 does.
        assert_eq!(order(&mut e, [5, 0, 1, 0], [5, -1, 2, 0]), -1);
        assert_eq!(order(&mut e, [5, -1, 0, 0], [5, 0, 0, 0]), 1);
        assert_eq!(order(&mut e, [5, -2, 0, 0], [5, 3, 0, 0]), -1);
        assert_eq!(order(&mut e, [5, 1, 7, 0], [5, 1, 6, 0]), 1);
        assert_eq!(order(&mut e, [5, 1, 7, -1], [5, 1, 7, 4]), 1);
        assert_eq!(order(&mut e, [5, 1, 7, 4], [5, 1, 7, 4]), 0);
        assert_eq!(order(&mut e, [5, 1, 7, -1], [5, 1, 7, -1]), 0);
    }
}

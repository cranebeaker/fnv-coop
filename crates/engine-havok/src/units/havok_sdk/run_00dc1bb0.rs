//! `Havok SDK/run_00dc1bb0` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `hkMapBase<unsigned long,unsigned long,hkMapOperations<unsigned long>>::findKey` (`00cb18f0`,
/// `thiscall` on the map, `RET 4`: the key); answers the slot index of the key.
const MAP_FIND_KEY: u32 = 0x00cb_18f0;
/// `retargetClassInplace` (`00dc1960`, cdecl: four words).
const RETARGET_CLASS_INPLACE: u32 = 0x00dc_1960;

// Translated from 00dc1bb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 0xc`: `this`, `key`, `map`, `extra`). Looks the
/// `key` up in the map (`findKey`); when the index it answers is above the map's word at `+8`
/// (signed compare) it calls `retargetClassInplace(key, this, map, extra)` (the argument order
/// the disassembly pushes: the decompiler's list names them differently).
pub fn fn_00dc1bb0(e: &mut Engine, this: u32, key: u32, map: u32, extra: u32) {
    let found = e.call(MAP_FIND_KEY, &args![map, key]).i32();
    if found > e.mem.i32(map + 8) {
        e.call(RETARGET_CLASS_INPLACE, &args![key, this, map, extra]);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00dc1bb0, fn_00dc1bb0(u32, u32, u32, u32))]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    /// The calls the function makes when `findKey` answers `found` and the map's word at `+8`
    /// is `limit` (the map's address is shown as 0xaa).
    fn run(found: i32, limit: i32) -> Vec<(u32, Vec<u32>)> {
        let mut e = Engine::new();
        let map = e.mem.alloc(0x20);
        e.mem.set_i32(map + 8, limit);
        let log: Rc<RefCell<Vec<(u32, Vec<u32>)>>> = Rc::default();
        let seen = Rc::clone(&log);
        e.register_double(MAP_FIND_KEY, move |_, a| {
            seen.borrow_mut().push((MAP_FIND_KEY, a.to_vec()));
            Ret {
                eax: found as u32,
                ..Ret::default()
            }
        });
        let seen = Rc::clone(&log);
        e.register_double(RETARGET_CLASS_INPLACE, move |_, a| {
            seen.borrow_mut().push((RETARGET_CLASS_INPLACE, a.to_vec()));
            Ret::default()
        });
        e.call(0x00dc_1bb0, &args![0x11u32, 0x22u32, map, 0x44u32]);
        let calls = log.borrow().clone();
        calls
            .into_iter()
            .map(|(address, words)| {
                let words = words.into_iter().map(|w| if w == map { 0xaa } else { w });
                (address, words.collect())
            })
            .collect()
    }

    #[test]
    fn a_key_found_past_the_limit_is_retargeted() {
        assert_eq!(
            run(5, 3),
            vec![
                (MAP_FIND_KEY, vec![0xaa, 0x22]),
                (RETARGET_CLASS_INPLACE, vec![0x22, 0x11, 0xaa, 0x44]),
            ]
        );
    }

    #[test]
    fn a_key_at_or_below_the_limit_is_left_alone_and_the_compare_is_signed() {
        assert_eq!(run(3, 3), vec![(MAP_FIND_KEY, vec![0xaa, 0x22])]);
        assert_eq!(run(-1, -2).len(), 2);
        assert_eq!(run(-3, -2).len(), 1);
    }
}

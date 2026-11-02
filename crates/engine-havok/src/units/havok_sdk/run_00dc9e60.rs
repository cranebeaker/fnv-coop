//! `Havok SDK/run_00dc9e60` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The memory router getter (`00c85750`): its word at `+0x10` is the allocator.
const MEMORY_ROUTER: u32 = 0x00c8_5750;
/// `hkMapBase<unsigned long,unsigned long,hkMapOperations<unsigned long>>::reserve` (`00cb1fe0`,
/// `thiscall` on the map, `RET 8`: the allocator, the number of pairs).
const MAP_RESERVE: u32 = 0x00cb_1fe0;

// Translated from 00dc9e60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: answers 10.
pub fn fn_00dc9e60(_e: &mut Engine) -> u32 {
    10
}

// Translated from 00dc9e70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`fastcall` on `this`). Clears an object that holds an empty
/// array (words 0 to 2: nothing, no elements, capacity word `0x80000000`) and an embedded
/// map at `+0xc` (words `+0xc`, `+0x10` zero, `+0x14` all ones), has the embedded map reserve
/// room for no pairs through the router's allocator (`00cb1fe0`) and sets the word at `+0x18`
/// to all ones. Answers `this`.
pub fn fn_00dc9e70(e: &mut Engine, this: u32) -> u32 {
    e.mem.set_u32(this, 0);
    e.mem.set_u32(this + 4, 0);
    e.mem.set_u32(this + 8, 0x8000_0000);
    let map = this + 0xc;
    e.mem.set_u32(map, 0);
    e.mem.set_u32(map + 4, 0);
    e.mem.set_u32(map + 8, 0xffff_ffff);
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    e.call(MAP_RESERVE, &args![map, allocator, 0u32]);
    e.mem.set_u32(this + 0x18, 0xffff_ffff);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00dc9e60, fn_00dc9e60() -> u32),
        entry!(0x00dc9e70, fn_00dc9e70(u32) -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    #[test]
    fn the_first_function_answers_ten() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00dc_9e60, &args![]).u32(), 10);
    }

    #[test]
    fn the_object_is_cleared_and_its_map_reserves_nothing() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x20);
        for word in 0..8 {
            e.mem.set_u32(this + 4 * word, 0xdead_0000 + word);
        }
        let router = e.mem.alloc(0x14);
        e.mem.set_u32(router + 0x10, 0xa110);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        let log: Rc<RefCell<Vec<Vec<u32>>>> = Rc::default();
        let seen = Rc::clone(&log);
        e.register_double(MAP_RESERVE, move |e, a| {
            // What the map looked like when it was asked to reserve.
            let mut words = a.to_vec();
            words.extend((0..3).map(|i| e.mem.u32(a[0] + 4 * i)));
            seen.borrow_mut().push(words);
            Ret::default()
        });
        assert_eq!(e.call(0x00dc_9e70, &args![this]).u32(), this);
        let words: Vec<u32> = (0..7).map(|i| e.mem.u32(this + 4 * i)).collect();
        assert_eq!(
            words,
            vec![0, 0, 0x8000_0000, 0, 0, 0xffff_ffff, 0xffff_ffff]
        );
        assert_eq!(
            *log.borrow(),
            vec![vec![this + 0xc, 0xa110, 0, 0, 0, 0xffff_ffff]]
        );
    }
}

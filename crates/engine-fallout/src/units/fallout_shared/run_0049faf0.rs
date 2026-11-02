//! `fallout shared/run_0049faf0` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `0049fc80` (`thiscall`, `RET 8`: the words at +4 and +8).
const RELEASE_ELEMENTS: u32 = 0x0049_fc80;
/// `006a8500` (`thiscall`, no stack word; unit `navmeshsearch.cpp` by the
/// engine map).
const FREE_STORAGE: u32 = 0x006a_8500;
/// `006e6da0` (`cdecl`, `(size, place)`): the placement `new`, answers
/// `place`.
const PLACEMENT_NEW: u32 = 0x006e_6da0;
/// `0049f1c0` (`thiscall`, no stack word): the constructor run on each
/// 16-byte element.
const ELEMENT_CONSTRUCTOR: u32 = 0x0049_f1c0;

// Translated from 0049faf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Empties the array (`thiscall`, `RET 4`; the engine map gives it no
/// name): nothing when the storage (+4) is null; otherwise `0049fc80` with
/// the storage and the count (+8), then, when the low byte of `free` is
/// set, `006a8500` and the storage and +0xc cleared; the count is cleared.
pub fn fn_0049faf0(e: &mut Engine, this: Ptr, free: u32) {
    let base = this.addr();
    let storage = e.mem.u32(base + 4);
    if storage == 0 {
        return;
    }
    let count = e.mem.u32(base + 8);
    e.call(RELEASE_ELEMENTS, &args![this, storage, count]);
    if free as u8 != 0 {
        e.call(FREE_STORAGE, &args![this]);
        e.mem.set_u32(base + 4, 0);
        e.mem.set_u32(base + 0xc, 0);
    }
    e.mem.set_u32(base + 8, 0);
}

// Translated from 0049fb50 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0049fb50 - the exception-unwinding frame (FS:[0]) is not translated.
/// Constructs `count` 16-byte elements in the array at `array` (`thiscall`,
/// `RET 8`; `this` is not read; the engine map gives it no name): for each
/// the placement `new` (`006e6da0` with 0x10 and the element's address)
/// and, when it answers non-null, `0049f1c0` on it.
pub fn fn_0049fb50(e: &mut Engine, _this: Ptr, array: u32, count: u32) {
    let mut index = 0u32;
    while index < count {
        let place = e
            .call(
                PLACEMENT_NEW,
                &args![0x10u32, (index << 4).wrapping_add(array)],
            )
            .u32();
        if place != 0 {
            e.call(ELEMENT_CONSTRUCTOR, &args![place]);
        }
        index += 1;
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0049_faf0, fn_0049faf0(Ptr, u32)),
        entry!(0x0049_fb50, fn_0049fb50(Ptr, u32, u32)),
    ]
}

#[cfg(test)]
mod n038_tests {
    use super::*;

    #[test]
    fn at_0049faf0_releases_and_frees_on_the_flag() {
        let mut e = Engine::new();
        e.register(RELEASE_ELEMENTS, |_, _| Ret::default());
        e.register(FREE_STORAGE, |_, _| Ret::default());
        let this = e.mem.alloc(0x10);
        e.call_log = Some(vec![]);
        e.call(0x0049_faf0, &args![this, 1u32]);
        assert_eq!(e.call_log.take().unwrap().len(), 1);
        for (flag, frees) in [(0x100u32, false), (1, true)] {
            e.mem.set_u32(this + 4, 0x5000);
            e.mem.set_u32(this + 8, 3);
            e.mem.set_u32(this + 0xc, 7);
            e.call_log = Some(vec![]);
            e.call(0x0049_faf0, &args![this, flag]);
            let log = e.call_log.take().unwrap();
            assert_eq!(log[1], (RELEASE_ELEMENTS, vec![this, 0x5000, 3]));
            assert_eq!(log.len(), if frees { 3 } else { 2 });
            assert_eq!(e.mem.u32(this + 8), 0);
            let kept = if frees { 0 } else { 0x5000 };
            assert_eq!(e.mem.u32(this + 4), kept);
            assert_eq!(e.mem.u32(this + 0xc), if frees { 0 } else { 7 });
        }
    }

    #[test]
    fn at_0049fb50_constructs_each_placed_element() {
        let mut e = Engine::new();
        e.register(PLACEMENT_NEW, |_, a| Ret {
            eax: if a[1] == 0x5010 { 0 } else { a[1] },
            ..Ret::default()
        });
        e.register(ELEMENT_CONSTRUCTOR, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0049_fb50, &args![0u32, 0x5000u32, 3u32]);
        let log = e.call_log.take().unwrap();
        let built: Vec<_> = log
            .iter()
            .filter(|c| c.0 == ELEMENT_CONSTRUCTOR)
            .map(|c| c.1[0])
            .collect();
        assert_eq!(built, vec![0x5000, 0x5020]);
        assert!(log.contains(&(PLACEMENT_NEW, vec![0x10, 0x5010])));
    }
}

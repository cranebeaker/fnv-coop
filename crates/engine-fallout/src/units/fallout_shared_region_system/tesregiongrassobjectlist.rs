//! `fallout shared/region system/tesregiongrassobjectlist.cpp` (Xbox PDB source unit), subsystem `fallout shared/region system`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 004f5df0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESRegionGrassObjectList` scalar deleting destructor (engine map name): runs the destructor body `004f5e20` on `this`, then frees `this` with `00401030` when bit 0 of `flags` is set. Returns `this`.
pub fn tes_region_grass_object_list_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    e.call(0x004f_5e20, &args![this]);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

/// `004f5ef0` (`thiscall`, no stack word): empties the object list.
const EMPTY_OBJECT_LIST: u32 = 0x004f_5ef0;
/// `0046ffb0` (`thiscall`, no stack word): the list's destructor.
const LIST_DESTRUCTOR: u32 = 0x0046_ffb0;

// Translated from 004f5e20 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 004f5e20 - the C++ exception-unwinding frame (FS:[0] chain, handler at 00f08f1b, state variable) is not translated
/// Unnamed in the engine map (unit `tesregiongrassobjectlist.cpp`, `thiscall`, no stack
/// word): the `TESRegionGrassObjectList` destructor run by its scalar deleting destructor.
/// Sets the vtable `01024094`, empties the list (`004f5ef0`) and destroys the list at +4
/// (`0046ffb0`).
pub fn fn_004f5e20(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), 0x0102_4094);
    e.call(EMPTY_OBJECT_LIST, &args![this]);
    e.call(LIST_DESTRUCTOR, &args![this.addr() + 4]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x004f_5df0, tes_region_grass_object_list_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x004f_5e20, fn_004f5e20(Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn at_004f5df0_runs_destructor_body_then_deletes_on_flag_one() {
        use crate::prelude::*;

        let mut e = Engine::new();
        let this: Ptr = Ptr::new(e.mem.alloc(16));
        e.register(0x004f_5e20, |_, _| Ret::default());
        e.register(0x0040_1030, |_, _| Ret::default());
        for (flags, deletes) in [(0u32, false), (2, false), (1, true), (0xffff_ffff, true)] {
            e.call_log = Some(vec![]);
            let result = e.call(0x004f_5df0, &args![this, flags]);
            assert_eq!(result.u32(), this.addr());
            let log = e.call_log.take().unwrap();
            let calls: Vec<(u32, Vec<u32>)> = log
                .into_iter()
                .filter(|(address, _)| *address != 0x004f_5df0)
                .collect();
            let mut expected = vec![(0x004f_5e20, vec![this.addr()])];
            if deletes {
                expected.push((0x0040_1030, vec![this.addr()]));
            }
            assert_eq!(calls, expected);
        }
    }
}

#[cfg(test)]
mod tests_b043 {
    use super::*;

    #[test]
    fn fn_004f5e20_empties_then_destroys_the_list() {
        let mut e = Engine::new();
        e.register(EMPTY_OBJECT_LIST, |_, _| Ret::default());
        e.register(LIST_DESTRUCTOR, |_, _| Ret::default());
        let this = e.mem.alloc(0x14);
        e.call_log = Some(vec![]);
        e.call(0x004f_5e20, &args![this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log[1..],
            [
                (EMPTY_OBJECT_LIST, vec![this]),
                (LIST_DESTRUCTOR, vec![this + 4]),
            ]
        );
        assert_eq!(e.mem.u32(this), 0x0102_4094);
    }
}

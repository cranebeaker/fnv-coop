//! `fallout shared/run_005822a0` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `0096a2d0` (`ECX`): clears a two-word `BSSimpleList` node (item, next), answers `this`.
const LIST_NODE_CTOR_0096A2D0: u32 = 0x0096_a2d0;
/// `00582780` (`ECX` = a `BSSimpleList` of heap items): deletes the items and empties the
/// list.
const FREE_ITEMS_00582780: u32 = 0x0058_2780;
/// `0046ffb0` (`ECX`): the `BSSimpleList` destructor body (empties it).
const LIST_DESTRUCTOR_0046FFB0: u32 = 0x0046_ffb0;

// Translated from 005822a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the map, `ECX` and one stack word (`RET 4`); called on the
/// `TESImageSpaceModifiableCountForm<6>` of a weather by `CopyComponent`, `CompareComponent`
/// and `UpdateHDRValues`: the modifier pointer `index` (`+4 + index * 4`).
pub fn fn_005822a0(e: &mut Engine, this: Ptr, index: u32) -> u32 {
    e.mem.u32(
        this.addr()
            .wrapping_add(index.wrapping_mul(4))
            .wrapping_add(4),
    )
}

// Translated from 005822c0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 005822c0 - the C++ exception-unwinding frame (FS:[0] chain, handler at 00eeaa58, state variable) is not translated
/// Unnamed in the map: the weather list constructor (`ECX`, no stack words; the climate's
/// `0052aaa0` runs it on its list): clears the list's head node (`0096a2d0`), then frees its
/// items and empties it (`00582780`); answers `this`.
pub fn fn_005822c0(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(LIST_NODE_CTOR_0096A2D0, &args![this]);
    e.call(FREE_ITEMS_00582780, &args![this]);
    this
}

// Translated from 00582320 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00582320 - the C++ exception-unwinding frame (FS:[0] chain, handler at 00eeaa58, state variable) is not translated
/// Unnamed in the map: the weather list destructor (`ECX`, no stack words): frees the items
/// and empties the list (`00582780`), then the list's own destructor body (`0046ffb0`).
pub fn fn_00582320(e: &mut Engine, this: Ptr) {
    e.call(FREE_ITEMS_00582780, &args![this]);
    e.call(LIST_DESTRUCTOR_0046FFB0, &args![this]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0058_22a0, fn_005822a0(Ptr, u32) -> u32),
        entry!(0x0058_22c0, fn_005822c0(Ptr) -> Ptr),
        entry!(0x0058_2320, fn_00582320(Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn order(e: &Engine) -> Vec<(u32, Vec<u32>)> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .skip(1)
            .cloned()
            .collect()
    }

    #[test]
    fn fn_005822a0_reads_a_modifier() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x1c);
        e.mem.set_u32(this + 4, 0x11);
        e.mem.set_u32(this + 0x18, 0x66);
        assert_eq!(e.call(0x0058_22a0, &args![this, 0u32]).u32(), 0x11);
        assert_eq!(e.call(0x0058_22a0, &args![this, 5u32]).u32(), 0x66);
    }

    #[test]
    fn fn_005822c0_clears_then_frees() {
        let mut e = Engine::new();
        e.register(LIST_NODE_CTOR_0096A2D0, |_, a| a[0].into_ret());
        e.register(FREE_ITEMS_00582780, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0058_22c0, &args![0x5000u32]).u32(), 0x5000);
        assert_eq!(
            order(&e),
            vec![
                (LIST_NODE_CTOR_0096A2D0, vec![0x5000]),
                (FREE_ITEMS_00582780, vec![0x5000])
            ]
        );
    }

    #[test]
    fn fn_00582320_frees_then_destroys() {
        let mut e = Engine::new();
        e.register(FREE_ITEMS_00582780, |_, _| Ret::default());
        e.register(LIST_DESTRUCTOR_0046FFB0, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0058_2320, &args![0x5000u32]);
        assert_eq!(
            order(&e),
            vec![
                (FREE_ITEMS_00582780, vec![0x5000]),
                (LIST_DESTRUCTOR_0046FFB0, vec![0x5000])
            ]
        );
    }
}

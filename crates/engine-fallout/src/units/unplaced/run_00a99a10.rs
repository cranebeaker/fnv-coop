//! `(unplaced)/run_00a99a10` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `00aa13e0` (`cdecl`, one word: the size): the Gamebryo allocation.
const NI_ALLOCATE: u32 = 0x00aa_13e0;
/// The `NiExtraData` constructor (engine map, `00a7b2e0` in `NiMain/niextradata.cpp`;
/// `thiscall`, no stack words).
const EXTRA_DATA_CONSTRUCT: u32 = 0x00a7_b2e0;
/// `NiIntegersExtraData`'s vtable (RTTI `.?AVNiIntegersExtraData@@` at the locator
/// `011275b0`).
const INTEGERS_EXTRA_DATA_VTABLE: u32 = 0x010a_0c7c;
/// The object's size and the two words cleared at +0xC and +0x10.
const OBJECT_SIZE: u32 = 0x14;
const WORD_0C: u32 = 0xc;
const WORD_10: u32 = 0x10;

// Translated from 00a99a10 (disassembled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, no arguments; its address is pushed at `00a89eba`,
/// after the `NiIntegersExtraData` unit): the `NiIntegersExtraData` factory. Allocates 0x14
/// bytes (`00aa13e0`); on success it runs the `NiExtraData` constructor (`00a7b2e0`), stores
/// the `NiIntegersExtraData` vtable and clears +0x10 and +0xC. Returns the object, or 0 when
/// the allocation fails.
pub fn fn_00a99a10(e: &mut Engine) -> u32 {
    let object = e.call(NI_ALLOCATE, &args![OBJECT_SIZE]).u32();
    if object == 0 {
        return 0;
    }
    e.call(EXTRA_DATA_CONSTRUCT, &args![object]);
    e.mem.set_u32(object, INTEGERS_EXTRA_DATA_VTABLE);
    e.mem.set_u32(object + WORD_10, 0);
    e.mem.set_u32(object + WORD_0C, 0);
    object
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00a9_9a10, fn_00a99a10() -> u32)]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn rig(succeeds: bool) -> Engine {
        let mut e = Engine::new();
        e.register_double(NI_ALLOCATE, move |e, a| {
            assert_eq!(a[0], OBJECT_SIZE);
            let block = if succeeds { e.mem.alloc(a[0]) } else { 0 };
            block.into_ret()
        });
        e.register(EXTRA_DATA_CONSTRUCT, |e, a| {
            for offset in (0..OBJECT_SIZE).step_by(4) {
                e.mem.set_u32(a[0] + offset, 0xdddd_dddd);
            }
            Ret::default()
        });
        e
    }

    #[test]
    fn the_factory_builds_an_integers_extra_data() {
        let mut e = rig(true);
        e.call_log = Some(vec![]);
        let object = e.call(0x00a9_9a10, &args![]).u32();
        assert_ne!(object, 0);
        assert_eq!(e.mem.u32(object), INTEGERS_EXTRA_DATA_VTABLE);
        assert_eq!(e.mem.u32(object + 4), 0xdddd_dddd);
        assert_eq!(e.mem.u32(object + 8), 0xdddd_dddd);
        assert_eq!(e.mem.u32(object + 0xc), 0);
        assert_eq!(e.mem.u32(object + 0x10), 0);
        let log = e.call_log.take().unwrap();
        let construct = log
            .iter()
            .find(|(a, _)| *a == EXTRA_DATA_CONSTRUCT)
            .unwrap();
        assert_eq!(construct.1, vec![object]);
    }

    #[test]
    fn a_failed_allocation_answers_null() {
        let mut e = rig(false);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00a9_9a10, &args![]).u32(), 0);
        let log = e.call_log.take().unwrap();
        assert!(log.iter().all(|(a, _)| *a != EXTRA_DATA_CONSTRUCT));
    }
}

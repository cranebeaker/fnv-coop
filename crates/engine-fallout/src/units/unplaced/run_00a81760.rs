//! `(unplaced)/run_00a81760` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `00aa13e0` (`cdecl`, one word: the size): the Gamebryo allocation.
const NI_ALLOCATE: u32 = 0x00aa_13e0;
/// `NiTriShapeData::NiTriShapeData` (engine map `NiTriShapeData_ov2`, `00a7b690`;
/// `thiscall`, no stack words).
const TRI_SHAPE_DATA_CONSTRUCT: u32 = 0x00a7_b690;
/// `NiTriShapeDynamicData`'s vtable (RTTI `.?AVNiTriShapeDynamicData@@` at the locator
/// `011261d4`).
const TRI_SHAPE_DYNAMIC_DATA_VTABLE: u32 = 0x0109_e32c;
/// The object's size and the two 16-bit fields cleared at +0x58 and +0x5A.
const OBJECT_SIZE: u32 = 0x5c;
const SHORT_58: u32 = 0x58;
const SHORT_5A: u32 = 0x5a;

// Translated from 00a81760 (disassembled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, no arguments; its address is pushed at `00a8a130`,
/// between the `NiTriShapeDynamicData` and `BSStripPSysData` units): the
/// `NiTriShapeDynamicData` factory. Allocates 0x5C bytes (`00aa13e0`); on success it runs the
/// `NiTriShapeData` constructor (`00a7b690`), clears the words at +0x58 and +0x5A and stores
/// the `NiTriShapeDynamicData` vtable. Returns the object, or 0 when the allocation fails.
pub fn fn_00a81760(e: &mut Engine) -> u32 {
    let object = e.call(NI_ALLOCATE, &args![OBJECT_SIZE]).u32();
    if object == 0 {
        return 0;
    }
    e.call(TRI_SHAPE_DATA_CONSTRUCT, &args![object]);
    e.mem.set_u16(object + SHORT_58, 0);
    e.mem.set_u32(object, TRI_SHAPE_DYNAMIC_DATA_VTABLE);
    e.mem.set_u16(object + SHORT_5A, 0);
    object
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00a8_1760, fn_00a81760() -> u32)]
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
        e.register(TRI_SHAPE_DATA_CONSTRUCT, |e, a| {
            for offset in (0..OBJECT_SIZE).step_by(4) {
                e.mem.set_u32(a[0] + offset, 0xdddd_dddd);
            }
            Ret::default()
        });
        e
    }

    #[test]
    fn the_factory_builds_a_dynamic_tri_shape_data() {
        let mut e = rig(true);
        e.call_log = Some(vec![]);
        let object = e.call(0x00a8_1760, &args![]).u32();
        assert_ne!(object, 0);
        assert_eq!(e.mem.u32(object), TRI_SHAPE_DYNAMIC_DATA_VTABLE);
        assert_eq!(e.mem.u32(object + 0x58), 0);
        assert_eq!(e.mem.u32(object + 0x54), 0xdddd_dddd);
        let log = e.call_log.take().unwrap();
        let construct = log
            .iter()
            .find(|(a, _)| *a == TRI_SHAPE_DATA_CONSTRUCT)
            .unwrap();
        assert_eq!(construct.1, vec![object]);
    }

    #[test]
    fn a_failed_allocation_answers_null() {
        let mut e = rig(false);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00a8_1760, &args![]).u32(), 0);
        let log = e.call_log.take().unwrap();
        assert!(log.iter().all(|(a, _)| *a != TRI_SHAPE_DATA_CONSTRUCT));
    }
}

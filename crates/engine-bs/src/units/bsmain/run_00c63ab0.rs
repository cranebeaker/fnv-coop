//! `BSMain/run_00c63ab0` (Xbox PDB source unit), subsystem `BSMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `00a7b2e0`: the `NiExtraData` constructor (`thiscall`, no stack word).
const EXTRA_DATA_CONSTRUCTOR: u32 = 0x00a7_b2e0;
/// The virtual tables the constructor stores: the base extra data class's (`010a0d24`),
/// then `BSDecalPlacementVectorExtraData`'s (`010c38ac`).
const BASE_VTABLE: u32 = 0x010a_0d24;
const DECAL_PLACEMENT_VECTOR_VTABLE: u32 = 0x010c_38ac;

// Translated from 00c63ab0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSDecalPlacementVectorExtraData`'s constructor (unnamed in the engine map;
/// `thiscall`, `RET 4`: the value at `+0xC`): the `NiExtraData` constructor, the base
/// class's virtual table and value, the class's virtual table, the empty skinned-object
/// list at `+0x34` (count, head, tail cleared), the group count (16-bit, `+0x10`) and the
/// eight group pointers (`+0x14`..`+0x30`) cleared. Answers `this`.
pub fn fn_00c63ab0(e: &mut Engine, this: Ptr, value: u32) -> Ptr {
    let object = this.addr();
    e.call(EXTRA_DATA_CONSTRUCTOR, &args![this]);
    e.mem.set_u32(object, BASE_VTABLE);
    e.mem.set_u32(object + 0xc, value);
    e.mem.set_u32(object, DECAL_PLACEMENT_VECTOR_VTABLE);
    e.mem.set_u32(object + 0x34 + 8, 0);
    e.mem.set_u32(object + 0x34, 0);
    e.mem.set_u32(object + 0x34 + 4, 0);
    e.mem.set_u16(object + 0x10, 0);
    for index in 0..8 {
        e.mem.set_u32(object + 0x14 + 4 * index, 0);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00c6_3ab0, fn_00c63ab0(Ptr, u32) -> Ptr)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constructor_clears_the_groups_and_the_list() {
        let mut e = Engine::new();
        e.register(EXTRA_DATA_CONSTRUCTOR, |_, _| Ret::default());
        let this = e.mem.alloc(0x40);
        for offset in (0x10..0x40).step_by(4) {
            e.mem.set_u32(this + offset, 0xffff_ffff);
        }
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00c6_3ab0, &args![this, 0x42u32]).u32(), this);
        assert_eq!(e.mem.u32(this), DECAL_PLACEMENT_VECTOR_VTABLE);
        assert_eq!(e.mem.u32(this + 0xc), 0x42);
        assert_eq!(e.mem.u16(this + 0x10), 0);
        assert_eq!(e.mem.u16(this + 0x12), 0xffff);
        for offset in (0x14..0x40).step_by(4) {
            assert_eq!(e.mem.u32(this + offset), 0, "{offset:#x}");
        }
        let log = e.call_log.take().unwrap();
        assert!(log
            .iter()
            .any(|c| c.0 == EXTRA_DATA_CONSTRUCTOR && c.1 == vec![this]));
    }
}

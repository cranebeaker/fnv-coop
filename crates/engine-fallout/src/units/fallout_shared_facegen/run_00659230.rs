//! `fallout shared/facegen/run_00659230` (Xbox PDB source unit), subsystem `fallout shared/facegen`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The base constructor (`00659330`: `this`; size, flag).
const MAP_BASE_CONSTRUCT: u32 = 0x0065_9330;
/// The vtable of `NiTStringMap<NiPointer<BSFaceGenModelMap::Entry>>`
/// (RTTI `.?AV?$NiTStringMap@V?$NiPointer@VEntry@BSFaceGenModelMap@@@@@@`).
const STRING_MAP_VTABLE: u32 = 0x0106_58f0;

// Translated from 00659230 (decompiled, FalloutNV.exe 1.4.0.525)
/// The constructor of `NiTStringMap<NiPointer<BSFaceGenModelMap::Entry>>`
/// (per its vtable's RTTI): the base with `size` and `flag`
/// (zero-extended), then the vtable. Returns `this`.
pub fn fn_00659230(e: &mut Engine, this: Ptr, size: u32, flag: u8) -> Ptr {
    e.call(MAP_BASE_CONSTRUCT, &args![this, size, u32::from(flag)]);
    e.mem.set_u32(this.addr(), STRING_MAP_VTABLE);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0065_9230, fn_00659230(Ptr, u32, u8) -> Ptr)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn base_then_vtable() {
        let mut e = Engine::new();
        e.register(MAP_BASE_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], 0x1111);
            Ret::default()
        });
        let map = e.mem.alloc(0x14);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0065_9230, &args![map, 37u32, 0x101u32]).u32(), map);
        assert_eq!(e.mem.u32(map), STRING_MAP_VTABLE);
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1], (MAP_BASE_CONSTRUCT, vec![map, 37, 1]));
    }
}

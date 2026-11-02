//! `NiMain/run_00aa09a0` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00aa09d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: a virtual method (in the vtables of many
/// classes) that takes one argument it does not read and answers `true`.
pub fn fn_00aa09d0(_e: &mut Engine, _this: Ptr, _unused_1: u32) -> bool {
    true
}

/// `NiLODData`'s vtable (its RTTI locator names `.?AVNiLODData@@`) and the `NiObject`
/// constructor (`00a5d3a0`) and destructor (`00a5d3d0`) of unit `niobject.cpp` (unnamed in
/// the engine map; `thiscall`, no stack words).
const NI_LOD_DATA_VTABLE: u32 = 0x010a_2254;
const OBJECT_CONSTRUCT: u32 = 0x00a5_d3a0;
const OBJECT_DESTRUCT: u32 = 0x00a5_d3d0;

// Translated from 00aa09a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, no stack words): the `NiLODData` constructor by
/// its vtable: the `NiObject` base (`00a5d3a0`), then the vtable. Answers `this`.
pub fn fn_00aa09a0(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(OBJECT_CONSTRUCT, &args![this]);
    e.mem.set_u32(this.addr(), NI_LOD_DATA_VTABLE);
    this
}

// Translated from 00aa09c0 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (a function Ghidra did not create; `thiscall`, no stack
/// words): the `NiLODData` destructor body: the vtable, then a jump to the `NiObject`
/// destructor (`00a5d3d0`), whose result it passes on.
pub fn fn_00aa09c0(e: &mut Engine, this: Ptr) -> u32 {
    e.mem.set_u32(this.addr(), NI_LOD_DATA_VTABLE);
    e.call(OBJECT_DESTRUCT, &args![this]).u32()
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00aa09d0, fn_00aa09d0(Ptr, u32) -> bool),
        entry!(0x00aa_09a0, fn_00aa09a0(Ptr) -> Ptr),
        entry!(0x00aa_09c0, fn_00aa09c0(Ptr) -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn answers_true() {
        let mut e = Engine::new();
        assert!(e.call(0x00aa09d0, &args![0x1000u32, 0u32]).bool());
    }
}

#[cfg(test)]
mod lod_data_tests {
    use super::*;

    #[test]
    fn constructor_and_destructor_set_the_vtable_around_the_base() {
        let mut e = Engine::new();
        e.register(OBJECT_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], 0x1111);
            Ret::default()
        });
        e.register(OBJECT_DESTRUCT, |e, a| {
            assert_eq!(e.mem.u32(a[0]), NI_LOD_DATA_VTABLE);
            0x77u32.into_ret()
        });
        let this = e.mem.alloc(0x10);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00aa_09a0, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), NI_LOD_DATA_VTABLE);
        e.mem.set_u32(this, 0);
        assert_eq!(e.call(0x00aa_09c0, &args![this]).u32(), 0x77);
        let log = e.call_log.take().unwrap();
        assert!(log.contains(&(OBJECT_CONSTRUCT, vec![this])));
        assert!(log.contains(&(OBJECT_DESTRUCT, vec![this])));
    }
}

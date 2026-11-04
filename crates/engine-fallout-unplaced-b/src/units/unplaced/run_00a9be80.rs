//! `(unplaced)/run_00a9be80` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiRefObject`'s vtable (`0101dce4`), its object count `NiRefObject::ms_uiObjects`
/// (`011f4400`, raised through the imported `InterlockedIncrement`) and `NiShaderConstantMap`'s
/// vtable (`010a1478`) (RTTI names read in Ghidra).
const NI_REF_OBJECT_VTABLE_0101DCE4: u32 = 0x0101_dce4;
const NI_REF_OBJECT_COUNT_011F4400: u32 = 0x011f_4400;
const IMPORT_INTERLOCKED_INCREMENT: u32 = 0x00fd_f1c0;
const CONSTANT_MAP_VTABLE_010A1478: u32 = 0x010a_1478;

// Translated from 00a9be80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (no unit): the `NiShaderConstantMap` constructor (the vtable it
/// stores has that class's RTTI), `thiscall` with one stack word: the program type. The
/// `NiRefObject` part (vtable `0101dce4`, count +4 cleared, `ms_uiObjects` at `011f4400` raised),
/// then `m_eProgramType` (+8, Xbox PDB) and the vtable `010a1478`. Answers `this`.
pub fn fn_00a9be80(e: &mut Engine, this: Ptr, program_type: u32) -> Ptr {
    let base = this.addr();
    e.mem.set_u32(base, NI_REF_OBJECT_VTABLE_0101DCE4);
    e.mem.set_u32(base + 4, 0);
    e.call(
        IMPORT_INTERLOCKED_INCREMENT,
        &args![NI_REF_OBJECT_COUNT_011F4400],
    );
    e.mem.set_u32(base + 8, program_type);
    e.mem.set_u32(base, CONSTANT_MAP_VTABLE_010A1478);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00a9_be80, fn_00a9be80(Ptr, u32) -> Ptr)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fn_00a9be80_builds_the_constant_map_base() {
        let mut e = Engine::new();
        e.register(IMPORT_INTERLOCKED_INCREMENT, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        let this = e.mem.alloc(0xc);
        e.mem.write(this, &[0xcc; 0xc]);
        assert_eq!(e.call(0x00a9_be80, &args![this, 2u32]).u32(), this);
        assert_eq!(e.mem.u32(this), CONSTANT_MAP_VTABLE_010A1478);
        assert_eq!(e.mem.u32(this + 4), 0);
        assert_eq!(e.mem.u32(this + 8), 2);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log[1],
            (
                IMPORT_INTERLOCKED_INCREMENT,
                vec![NI_REF_OBJECT_COUNT_011F4400]
            )
        );
    }
}

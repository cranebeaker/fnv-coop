//! `(unplaced)/run_00a8f440` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiRefObject`'s vtable (`0101dce4`), its object count `NiRefObject::ms_uiObjects`
/// (`011f4400`, raised through the imported `InterlockedIncrement`), `NiShader`'s vtable
/// (`0109f584`, RTTI `.?AVNiShader@@`) and the empty fixed string (`0109b220`).
const NI_REF_OBJECT_VTABLE_0101DCE4: u32 = 0x0101_dce4;
const NI_REF_OBJECT_COUNT_011F4400: u32 = 0x011f_4400;
const IMPORT_INTERLOCKED_INCREMENT: u32 = 0x00fd_f1c0;
const NI_SHADER_VTABLE_0109F584: u32 = 0x0109_f584;
const FIXED_STRING_EMPTY_0109B220: u32 = 0x0109_b220;

// Translated from 00a8f440 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (no unit): the `NiShader` constructor (the vtable it stores has
/// that class's RTTI), `thiscall`, no stack words. The `NiRefObject` part (vtable `0101dce4`,
/// count +4 cleared, `ms_uiObjects` at `011f4400` raised), then the vtable `0109f584`, the name
/// +8 set to the empty fixed string (the word at `0109b220`, without a reference), +0xc cleared
/// and the byte +0x10 cleared. Answers `this`.
pub fn fn_00a8f440(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.mem.set_u32(base, NI_REF_OBJECT_VTABLE_0101DCE4);
    e.mem.set_u32(base + 4, 0);
    e.call(
        IMPORT_INTERLOCKED_INCREMENT,
        &args![NI_REF_OBJECT_COUNT_011F4400],
    );
    e.mem.set_u32(base, NI_SHADER_VTABLE_0109F584);
    let empty = e.global::<u32>(FIXED_STRING_EMPTY_0109B220);
    e.mem.set_u32(base + 8, empty);
    e.mem.set_u32(base + 0xc, 0);
    e.mem.set_u8(base + 0x10, 0);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00a8_f440, fn_00a8f440(Ptr) -> Ptr)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fn_00a8f440_builds_the_shader_base() {
        let mut e = Engine::new();
        e.map(0x0109_b000, 0x1000);
        e.set_global(FIXED_STRING_EMPTY_0109B220, 0x0e00_0008u32);
        e.register(IMPORT_INTERLOCKED_INCREMENT, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        let this = e.mem.alloc(0x14);
        e.mem.write(this, &[0xcc; 0x14]);
        assert_eq!(e.call(0x00a8_f440, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), NI_SHADER_VTABLE_0109F584);
        assert_eq!(e.mem.u32(this + 4), 0);
        assert_eq!(e.mem.u32(this + 8), 0x0e00_0008);
        assert_eq!(e.mem.u32(this + 0xc), 0);
        assert_eq!(e.mem.u8(this + 0x10), 0);
        assert_eq!(e.mem.u8(this + 0x11), 0xcc);
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

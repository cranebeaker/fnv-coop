//! `BSShader/run_00b668b0` (Xbox PDB source unit), subsystem `BSShader`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `InterlockedIncrement` / `InterlockedDecrement` (KERNEL32 import slots).
const INTERLOCKED_INCREMENT: u32 = 0x00fd_f1c0;
const INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;
/// `NiRefObject`'s vtable (`0101dce4`) and its live-object count `ms_uiObjects` (`011f4400`).
const REF_OBJECT_VTABLE: u32 = 0x0101_dce4;
const REF_OBJECT_COUNT: u32 = 0x011f_4400;
/// The vtable the constructor stores (`010ae0c8`: the scalar deleting destructor `00b675a0`,
/// which frees 0x74 bytes, and `DeleteThis`).
const TEXTURE_EFFECT_DATA_VTABLE: u32 = 0x010a_e0c8;

/// Releases the `NiPointer` at `slot` as the constructor inlines it: when it holds an object,
/// `InterlockedDecrement` on its `+4` (slot +4, `DeleteThis`, at zero), then the slot cleared.
fn clear_pointer(e: &mut Engine, slot: u32) {
    let object = e.mem.u32(slot);
    if object != 0 {
        if e.call(INTERLOCKED_DECREMENT, &args![object.wrapping_add(4)])
            .u32()
            == 0
        {
            e.vcall(object, 4, &args![]);
        }
        e.mem.set_u32(slot, 0);
    }
}

// Translated from 00b668b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: a constructor (`thiscall`, no words) of a 0x74-byte
/// `NiRefObject` whose fields match the Xbox PDB's `BSShaderPPLightingProperty::
/// TextureEffectData` (size 0x74; `TintScenegraph` `004b63bd` and `00507830` allocate 0x74 bytes
/// for it). The `NiRefObject` part (vtable `0101dce4`, count 0, `ms_uiObjects` incremented),
/// the vtable `010ae0c8`, then:
/// - `spTexture` (+0x8) cleared, the sixteen floats of the four colours (+0xc to +0x48) 0.0
///   (`FLDZ`, `FST`), `spBlockOutTexture` (+0x6c) cleared;
/// - `spTexture` released and cleared when it holds an object (it never does here: it was
///   just cleared), the sixteen floats 0.0 again;
/// - `eSrcBlendMode` (+0x5c) 2, `eDestBlendMode` (+0x60) 1, `eBlendOperation` (+0x64) 1,
///   `eZTestFunction` (+0x68) 3, and the four floats +0x4c to +0x58 0.0;
/// - `spBlockOutTexture` released and cleared when it holds an object (also just cleared),
///   `iAlphaTest` (+0x70) 0.
///
/// Answers `this`. The exception frame (`FS:[0]`) is not translated.
// Translation incomplete: 00b668b0 - the FS:[0] exception frame (unwinding on a throw) is not translated
pub fn fn_00b668b0(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.mem.set_u32(base, REF_OBJECT_VTABLE);
    e.mem.set_u32(base.wrapping_add(4), 0);
    e.call(INTERLOCKED_INCREMENT, &args![REF_OBJECT_COUNT]);
    e.mem.set_u32(base, TEXTURE_EFFECT_DATA_VTABLE);
    e.mem.set_u32(base.wrapping_add(8), 0);
    // `FLDZ` then `FST` into each colour float: +0.0.
    let zero = x87_store32(0.0);
    for offset in (0xcu32..=0x48).step_by(4) {
        e.mem.set_u32(base.wrapping_add(offset), zero);
    }
    e.mem.set_u32(base.wrapping_add(0x6c), 0);
    clear_pointer(e, base.wrapping_add(8));
    for offset in (0xcu32..=0x48).step_by(4) {
        e.mem.set_u32(base.wrapping_add(offset), zero);
    }
    e.mem.set_u32(base.wrapping_add(0x5c), 2);
    e.mem.set_u32(base.wrapping_add(0x4c), zero);
    e.mem.set_u32(base.wrapping_add(0x60), 1);
    e.mem.set_u32(base.wrapping_add(0x50), zero);
    e.mem.set_u32(base.wrapping_add(0x64), 1);
    e.mem.set_u32(base.wrapping_add(0x54), zero);
    e.mem.set_u32(base.wrapping_add(0x68), 3);
    e.mem.set_u32(base.wrapping_add(0x58), zero);
    clear_pointer(e, base.wrapping_add(0x6c));
    e.mem.set_u32(base.wrapping_add(0x70), 0);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00b6_68b0, fn_00b668b0(Ptr) -> Ptr)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constructs_the_texture_effect_data() {
        let mut e = Engine::new();
        e.register(INTERLOCKED_INCREMENT, |e, a| {
            let v = e.mem.u32(a[0]).wrapping_add(1);
            e.mem.set_u32(a[0], v);
            v.into_ret()
        });
        e.map(0x011f_4000, 0x1000);
        e.set_global(REF_OBJECT_COUNT, 3u32);
        let this = e.mem.alloc(0x74);
        for offset in (0..0x74).step_by(4) {
            e.mem.set_u32(this + offset, 0xdead_beef);
        }
        assert_eq!(e.call(0x00b6_68b0, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), TEXTURE_EFFECT_DATA_VTABLE);
        assert_eq!(e.mem.u32(this + 4), 0);
        assert_eq!(e.global::<u32>(REF_OBJECT_COUNT), 4);
        for offset in (0x8..=0x58).step_by(4) {
            assert_eq!(e.mem.u32(this + offset), 0, "offset {offset:#x}");
        }
        assert_eq!(
            [
                e.mem.u32(this + 0x5c),
                e.mem.u32(this + 0x60),
                e.mem.u32(this + 0x64),
                e.mem.u32(this + 0x68)
            ],
            [2, 1, 1, 3]
        );
        assert_eq!(e.mem.u32(this + 0x6c), 0);
        assert_eq!(e.mem.u32(this + 0x70), 0);
    }
}

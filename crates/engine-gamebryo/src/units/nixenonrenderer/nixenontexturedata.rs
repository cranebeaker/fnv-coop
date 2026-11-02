//! `NiXenonRenderer/nixenontexturedata.cpp` (Xbox PDB source unit), subsystem `NiXenonRenderer`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiTexture::RendererData`'s vtable (`010a080c`, RTTI `.?AVRendererData@NiTexture@@`).
const RENDERER_DATA_VTABLE: u32 = 0x010a_080c;
/// The destructor body run after the vtable store (`00a5d3d0`, unit `NiMain/niobject.cpp`,
/// `thiscall`, no stack words).
const BASE_DESTRUCTOR: u32 = 0x00a5_d3d0;
/// Gamebryo's sized object release (`00aa1460`, `cdecl`: object, size).
const NI_DELETE: u32 = 0x00aa_1460;

// Translated from 00a981a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTexture::RendererData::~RendererData`'s scalar deleting destructor (engine map name):
/// stores the class's vtable, runs `00a5d3d0` on `this` and, when `flags & 1`, frees the
/// 0x60 bytes. Returns `this`.
pub fn ni_texture_renderer_data_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    e.mem.set_u32(this.addr(), RENDERER_DATA_VTABLE);
    e.call(BASE_DESTRUCTOR, &args![this]);
    if flags & 1 != 0 {
        e.call(NI_DELETE, &args![this, 0x60u32]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(
        0x00a9_81a0,
        ni_texture_renderer_data_scalar_deleting_destructor(Ptr, u32) -> Ptr
    )]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scalar_deleting_destructor_frees_with_bit_zero() {
        let mut e = Engine::new();
        e.register(BASE_DESTRUCTOR, |_, _| Ret::default());
        e.register(NI_DELETE, |_, _| Ret::default());
        let this = e.mem.alloc(0x60);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00a9_81a0, &args![this, 0u32]).u32(), this);
        assert_eq!(e.mem.u32(this), RENDERER_DATA_VTABLE);
        let log = e.call_log.take().unwrap();
        assert!(log.contains(&(BASE_DESTRUCTOR, vec![this])));
        assert!(!log.iter().any(|c| c.0 == NI_DELETE));
        e.call_log = Some(vec![]);
        e.call(0x00a9_81a0, &args![this, 0x11u32]);
        let log = e.call_log.take().unwrap();
        assert!(log.contains(&(NI_DELETE, vec![this, 0x60])));
    }
}

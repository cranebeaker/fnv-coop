//! `NiMain/nidepthstencilbuffer.cpp` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiDepthStencilBuffer::ms_RTTI` (the class's `NiRTTI` record).
const NI_DEPTH_STENCIL_BUFFER_RTTI: u32 = 0x011f_603c;

// Translated from 00a9fe50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiDepthStencilBuffer::GetRTTI` (Xbox PDB): the address of the class's `NiRTTI` record.
pub fn ni_depth_stencil_buffer_get_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(NI_DEPTH_STENCIL_BUFFER_RTTI)
}

/// `NiDepthStencilBuffer`'s vtable (`010a2084`; RTTI
/// `.?AVNiDepthStencilBuffer@@`).
const VTABLE_NI_DEPTH_STENCIL_BUFFER: u32 = 0x010a_2084;
/// `00a8f1c0`: the base destructor (a jump to `00a5d3d0`), `thiscall`.
const BASE_DESTRUCTOR: u32 = 0x00a8_f1c0;
/// `00aa1460`: frees a block of a known size (`cdecl`: block, size).
const FREE_SIZED: u32 = 0x00aa_1460;

// Translated from 00a9fe60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiDepthStencilBuffer::~NiDepthStencilBuffer` (engine map; the scalar
/// deleting destructor): sets the class's vtable, runs the base destructor
/// (`00a8f1c0`), then frees the 0x14-byte object (`00aa1460`) when bit 0 of
/// `flags` is set. Returns `this`.
pub fn ni_depth_stencil_buffer_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    e.mem.set_u32(this.addr(), VTABLE_NI_DEPTH_STENCIL_BUFFER);
    e.call(BASE_DESTRUCTOR, &args![this]);
    if flags & 1 != 0 {
        e.call(FREE_SIZED, &args![this, 0x14u32]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a9fe50, ni_depth_stencil_buffer_get_rtti() -> Ptr),
        entry!(0x00a9_fe60, ni_depth_stencil_buffer_scalar_deleting_destructor(Ptr, u32) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ni_depth_stencil_buffer_get_rtti_returns_the_class_rtti() {
        let mut e = Engine::new();
        assert_eq!(
            e.call(0x00a9fe50, &args![]).u32(),
            NI_DEPTH_STENCIL_BUFFER_RTTI
        );
    }
}

#[cfg(test)]
mod b040_tests {
    use super::*;

    #[test]
    fn deleting_destructor_frees_0x14_bytes_with_bit_0() {
        let mut e = Engine::new();
        e.register(BASE_DESTRUCTOR, |_, _| Ret::default());
        e.register(FREE_SIZED, |_, _| Ret::default());
        let this = e.mem.alloc(0x14);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00a9_fe60, &args![this, 1u32]).u32(), this);
        assert_eq!(e.mem.u32(this), VTABLE_NI_DEPTH_STENCIL_BUFFER);
        let log = e.call_log.take().unwrap();
        assert!(log.contains(&(BASE_DESTRUCTOR, vec![this])));
        assert!(log.contains(&(FREE_SIZED, vec![this, 0x14])));
        e.call_log = Some(vec![]);
        e.call(0x00a9_fe60, &args![this, 0u32]);
        let log = e.call_log.take().unwrap();
        assert!(log.iter().all(|c| c.0 != FREE_SIZED));
    }
}

//! `NiSystem/nibinarystream.cpp` (Xbox PDB source unit), subsystem `NiSystem`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The `NiBinaryStream` vtable (`010a24b8`).
const BINARY_STREAM_VTABLE: u32 = 0x010a_24b8;
/// The memory manager singleton and its `Deallocate` (`00aa4060`, thiscall, one word).
const MEMORY_MANAGER: u32 = 0x011f_6238;
const MEMORY_DEALLOCATE: u32 = 0x00aa_4060;

// Translated from 00aa1db0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBinaryStream::scalar deleting destructor` (engine map; thiscall, one word): stores the
/// `NiBinaryStream` vtable (`010a24b8`) and, with bit 0 of `flags`, gives the stream back to
/// the memory manager. Answers `this`.
pub fn ni_binary_stream_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    e.mem.set_u32(this.addr(), BINARY_STREAM_VTABLE);
    if flags & 1 != 0 {
        e.call(MEMORY_DEALLOCATE, &args![MEMORY_MANAGER, this]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(
        0x00aa_1db0,
        ni_binary_stream_scalar_deleting_destructor(Ptr, u8) -> Ptr
    )]
}

#[cfg(test)]
mod n083_tests {
    use super::*;

    #[test]
    fn deleting_destructor_sets_the_vtable_and_frees_on_bit_zero() {
        let mut e = Engine::new();
        e.register(MEMORY_DEALLOCATE, |_, _| Ret::default());
        let this = e.mem.alloc(8);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00aa_1db0, &args![this, 0x100u32]).u32(), this);
        assert_eq!(e.mem.u32(this), BINARY_STREAM_VTABLE);
        e.call(0x00aa_1db0, &args![this, 0x101u32]);
        let log = e.call_log.take().unwrap();
        let freed: Vec<_> = log.iter().filter(|c| c.0 == MEMORY_DEALLOCATE).collect();
        assert_eq!(freed.len(), 1);
        assert_eq!(freed[0].1, [MEMORY_MANAGER, this]);
    }
}

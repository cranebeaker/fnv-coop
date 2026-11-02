//! `Havok SDK/hkserverdebugdisplayhandler.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The vtable the destructor writes (`010d7cfc`).
const HANDLER_VTABLE: u32 = 0x010d_7cfc;
/// The function the object is freed with when the flag says so (`00401030`, cdecl, one stack
/// word: the block; unnamed in the engine map).
const FREE_BLOCK: u32 = 0x0040_1030;

// Translated from 00dd97c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkDebugDisplayHandler::_scalar_deleting_destructor_` (Xbox PDB), `(flags)`: sets the
/// object's vtable to `010d7cfc` and, when bit 0 of the flag byte is set, frees the object with
/// `00401030`. Returns `this`.
pub fn hk_debug_display_handler_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u8,
) -> Ptr {
    e.mem.set_u32(this.addr(), HANDLER_VTABLE);
    if flags & 1 != 0 {
        e.call(FREE_BLOCK, &args![this]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(
        0x00dd97c0,
        hk_debug_display_handler_scalar_deleting_destructor(Ptr, u8) -> Ptr
    )]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_destructor_frees_the_object_only_when_bit_zero_is_set() {
        let mut e = Engine::new();
        e.register(FREE_BLOCK, |_, _| Ret::default());
        let this = e.mem.alloc(8);
        e.call_log = Some(vec![]);
        // only the low byte of the word is the flag: 0xfe has bit 0 clear
        assert_eq!(
            e.call(0x00dd_97c0, &args![this, 0xffff_fffeu32]).u32(),
            this
        );
        assert_eq!(e.mem.u32(this), 0x010d_7cfc);
        assert_eq!(e.call_log.clone().unwrap().len(), 1);
        e.mem.set_u32(this, 0);
        assert_eq!(e.call(0x00dd_97c0, &args![this, 1u32]).u32(), this);
        assert_eq!(e.mem.u32(this), 0x010d_7cfc);
        let log = e.call_log.clone().unwrap();
        assert!(log.contains(&(FREE_BLOCK, vec![this])));
    }
}

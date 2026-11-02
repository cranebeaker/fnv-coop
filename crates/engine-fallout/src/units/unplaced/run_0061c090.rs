//! `(unplaced)/run_0061c090` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The game's `operator delete` (`00401030`, `cdecl`, one word).
const OPERATOR_DELETE: u32 = 0x0040_1030;
/// The destructor bodies the two scalar deleting destructors run.
const LARGE_ARRAY_DESTRUCTOR: u32 = 0x0061_bcf0;
const LARGE_PRIMITIVE_ARRAY_DESTRUCTOR: u32 = 0x0061_8940;

fn scalar_deleting(e: &mut Engine, this: Ptr, flags: u32, body: u32) -> Ptr {
    e.call(body, &args![this]);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 0061c090 (decompiled, FalloutNV.exe 1.4.0.525)
/// The scalar deleting destructor in slot 0 of the vtable `0104d448`, whose RTTI names
/// `NiTLargeArray<TESTopicInfo*,NiTMallocInterface<TESTopicInfo*>>` (unnamed in the engine
/// map; `ECX` and one stack word, `RET 4`): the destructor body `0061bcf0`, then
/// `operator delete` when bit 0 of `flags` is set. Answers `this`.
pub fn fn_0061c090(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    scalar_deleting(e, this, flags, LARGE_ARRAY_DESTRUCTOR)
}

// Translated from 0061c0c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The scalar deleting destructor in slot 0 of the vtable `0104d450`, whose RTTI names
/// `NiTLargePrimitiveArray<TESTopicInfo*>` (unnamed in the engine map; `ECX` and one stack
/// word, `RET 4`): the destructor body `00618940`, then `operator delete` when bit 0 of
/// `flags` is set. Answers `this`.
pub fn fn_0061c0c0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    scalar_deleting(e, this, flags, LARGE_PRIMITIVE_ARRAY_DESTRUCTOR)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0061_c090, fn_0061c090(Ptr, u32) -> Ptr),
        entry!(0x0061_c0c0, fn_0061c0c0(Ptr, u32) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn both_destructors_free_only_with_bit_0() {
        for (address, body) in [
            (0x0061_c090u32, LARGE_ARRAY_DESTRUCTOR),
            (0x0061_c0c0, LARGE_PRIMITIVE_ARRAY_DESTRUCTOR),
        ] {
            let mut e = Engine::new();
            e.register(body, |_, _| Ret::default());
            e.register(OPERATOR_DELETE, |_, _| Ret::default());
            e.call_log = Some(vec![]);
            assert_eq!(e.call(address, &args![0x5000u32, 3u32]).u32(), 0x5000);
            let log = e.call_log.take().unwrap();
            assert_eq!(
                log[1..],
                [(body, vec![0x5000]), (OPERATOR_DELETE, vec![0x5000])]
            );
            e.call_log = Some(vec![]);
            e.call(address, &args![0x5000u32, 2u32]);
            assert_eq!(e.call_log.take().unwrap().len(), 2);
        }
    }
}

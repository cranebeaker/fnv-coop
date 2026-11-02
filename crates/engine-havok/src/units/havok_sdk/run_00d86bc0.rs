//! `Havok SDK/run_00d86bc0` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `hkClassMember::getSizeInBytes` (`00d86980`, thiscall on the member).
const MEMBER_GET_SIZE_IN_BYTES: u32 = 0x00d8_6980;

// Translated from 00d86bc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thiscall on a class member, two stack words `(target, value)` (`RET 8`): stores `value` at
/// `target` as a byte, a 16-bit word or a dword, by the member's size in bytes
/// (`hkClassMember::getSizeInBytes`: 1, 2 or 4); any other size stores nothing. The exe's name
/// for it is unknown (an integer write through a member).
pub fn fn_00d86bc0(e: &mut Engine, member: u32, target: u32, value: u32) {
    match e.call(MEMBER_GET_SIZE_IN_BYTES, &args![member]).u32() {
        1 => e.mem.set_u8(target, value as u8),
        2 => e.mem.set_u16(target, value as u16),
        4 => e.mem.set_u32(target, value),
        _ => {}
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00d8_6bc0, fn_00d86bc0(u32, u32, u32))]
}

#[cfg(test)]
mod a09_tests {
    use super::*;

    fn store_with_size(size: u32) -> (u32, u32) {
        let mut e = Engine::new();
        e.register_double(MEMBER_GET_SIZE_IN_BYTES, move |_, _| Ret {
            eax: size,
            ..Ret::default()
        });
        let target = e.mem.alloc(8);
        e.mem.set_u32(target, 0x1111_1111);
        e.mem.set_u32(target + 4, 0x2222_2222);
        e.call(0x00d8_6bc0, &args![0x77u32, target, 0xaabb_ccddu32]);
        (e.mem.u32(target), e.mem.u32(target + 4))
    }

    #[test]
    fn the_value_is_stored_by_the_members_size() {
        assert_eq!(store_with_size(1), (0x1111_11dd, 0x2222_2222));
        assert_eq!(store_with_size(2), (0x1111_ccdd, 0x2222_2222));
        assert_eq!(store_with_size(4), (0xaabb_ccdd, 0x2222_2222));
    }

    #[test]
    fn other_sizes_store_nothing() {
        for size in [0, 3, 8, 16] {
            assert_eq!(store_with_size(size), (0x1111_1111, 0x2222_2222));
        }
    }

    #[test]
    fn the_member_is_passed_to_the_size_function() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register(MEMBER_GET_SIZE_IN_BYTES, |_, _| Ret::default());
        let target = e.mem.alloc(8);
        e.call(0x00d8_6bc0, &args![0x77u32, target, 5u32]);
        let log = e.call_log.as_ref().unwrap();
        assert!(log.contains(&(MEMBER_GET_SIZE_IN_BYTES, vec![0x77])));
    }
}

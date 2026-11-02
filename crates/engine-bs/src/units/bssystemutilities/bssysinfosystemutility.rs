//! `BSSystemUtilities/bssysinfosystemutility.cpp` (Xbox PDB source unit), subsystem
//! `BSSystemUtilities`: its functions in FalloutNV.exe 1.4.0.525, translated
//! (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The memory manager (`011f6238`) and its release (`00aa4060`, `thiscall`, one word), and
/// the utility's destructor body (`00af3b50`, `thiscall`).
const MEMORY_MANAGER: u32 = 0x011f_6238;
const DEALLOCATE: u32 = 0x00aa_4060;
const DESTRUCT: u32 = 0x00af_3b50;

// Translated from 00af3b70 (disassembled, FalloutNV.exe 1.4.0.525)
/// No label (`thiscall`, plain `RET`): does nothing.
pub fn fn_00af3b70(_e: &mut Engine, _this: u32) {}

// Translated from 00af3b80 (disassembled, FalloutNV.exe 1.4.0.525)
/// `BSSysInfoSystemUtility::Destroy` (engine map; `thiscall`, plain `RET`): stores 0x10 at
/// +4, as the constructor does.
pub fn bs_sys_info_system_utility_destroy(e: &mut Engine, this: u32) {
    e.mem.set_u32(this.wrapping_add(4), 0x10);
}

// Translated from 00af3ba0 (disassembled, FalloutNV.exe 1.4.0.525)
/// `BSSysInfoSystemUtility`'s scalar deleting destructor (engine map; `thiscall`, one word:
/// the flags, `RET 4`): the destructor body (`00af3b50`), then with bit 0 of the flags the
/// block is freed (`00aa4060`). Answers `this`.
pub fn bs_sys_info_system_utility_scalar_deleting_destructor(
    e: &mut Engine,
    this: u32,
    flags: u32,
) -> u32 {
    e.call(DESTRUCT, &args![this]);
    if flags & 1 != 0 {
        e.call(DEALLOCATE, &args![MEMORY_MANAGER, this]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00af_3b70, fn_00af3b70(u32)),
        entry!(0x00af_3b80, bs_sys_info_system_utility_destroy(u32)),
        entry!(
            0x00af_3ba0,
            bs_sys_info_system_utility_scalar_deleting_destructor(u32, u32) -> u32
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    #[test]
    fn destroy_resets_the_word_at_4() {
        let mut e = Engine::new();
        let this = e.mem.alloc(8);
        e.call(0x00af_3b70, &args![this]);
        assert_eq!(e.mem.u32(this + 4), 0);
        e.call(0x00af_3b80, &args![this]);
        assert_eq!(e.mem.u32(this + 4), 0x10);
    }

    #[test]
    fn the_deleting_destructor_frees_with_bit_0() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register(DESTRUCT, |_, _| Ret::default());
        e.register(DEALLOCATE, |_, _| Ret::default());
        assert_eq!(e.call(0x00af_3ba0, &args![0x300u32, 0u32]).u32(), 0x300);
        assert!(calls(&e, DEALLOCATE).is_empty());
        assert_eq!(e.call(0x00af_3ba0, &args![0x300u32, 1u32]).u32(), 0x300);
        assert_eq!(calls(&e, DESTRUCT), vec![vec![0x300], vec![0x300]]);
        assert_eq!(calls(&e, DEALLOCATE), vec![vec![MEMORY_MANAGER, 0x300]]);
    }
}

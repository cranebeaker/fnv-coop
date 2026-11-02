//! `BSSystemUtilities/bssystemutilitiesstrings.cpp` (Xbox PDB source unit), subsystem
//! `BSSystemUtilities`: its functions in FalloutNV.exe 1.4.0.525, translated
//! (docs/ENGINE_CRATE.md). The object holds eight string pointers after its vtable (+4..+0x24),
//! each a block from the memory manager.

#[allow(unused_imports)]
use crate::prelude::*;

/// The memory manager (`011f6238`) and its allocation and release (`00aa3e40`, `00aa4060`,
/// `thiscall`, one word), the object's destructor body (`00af35c0`, `thiscall`) and the C
/// runtime's `strlen` (`00ec6130`), `memset` (`00ec61c0`) and `strcpy_s` (`00ec65a6`), cdecl.
const MEMORY_MANAGER: u32 = 0x011f_6238;
const ALLOCATE: u32 = 0x00aa_3e40;
const DEALLOCATE: u32 = 0x00aa_4060;
const DESTRUCT: u32 = 0x00af_35c0;
const STRLEN: u32 = 0x00ec_6130;
const MEMSET: u32 = 0x00ec_61c0;
const STRCPY_S: u32 = 0x00ec_65a6;
/// The string pointers: eight words from +4.
const STRINGS: u32 = 4;
const COUNT: u32 = 8;

// Translated from 00af3660 (disassembled, FalloutNV.exe 1.4.0.525)
/// No label (`thiscall`, plain `RET`): calls the object's own vtable slot +8 and passes its
/// answer on (EAX).
pub fn fn_00af3660(e: &mut Engine, this: u32) -> u32 {
    e.vcall(this, 0x8, &args![]).u32()
}

// Translated from 00af3680 (disassembled, FalloutNV.exe 1.4.0.525)
/// `BSSystemUtilitiesStrings::Destroy` (engine map; `thiscall`, plain `RET`): each of the
/// eight string pointers that is not null is freed (`00aa4060`) and cleared.
pub fn bs_system_utilities_strings_destroy(e: &mut Engine, this: u32) {
    for i in 0..COUNT {
        let slot = this.wrapping_add(i * 4 + STRINGS);
        if e.mem.u32(slot) != 0 {
            let text = e.mem.u32(slot);
            e.call(DEALLOCATE, &args![MEMORY_MANAGER, text]);
            e.mem.set_u32(slot, 0);
        }
    }
}

// Translated from 00af36e0 (disassembled, FalloutNV.exe 1.4.0.525)
/// Sets one string (no label; `thiscall`, two words: the index and the text, `RET 8`): the
/// index's old string, when not null, is freed (`00aa4060`) and cleared. A non-null text is
/// copied into a new block of its length plus one (`00aa3e40`), which is zeroed (`memset`) and
/// filled (`strcpy_s` with that size). The index is not checked: the slot is
/// `this + index * 4 + 4` as the exe addresses it.
pub fn fn_00af36e0(e: &mut Engine, this: u32, index: u32, text: u32) {
    let slot = this
        .wrapping_add(index.wrapping_mul(4))
        .wrapping_add(STRINGS);
    if e.mem.u32(slot) != 0 {
        let old = e.mem.u32(slot);
        e.call(DEALLOCATE, &args![MEMORY_MANAGER, old]);
        e.mem.set_u32(slot, 0);
    }
    if text != 0 {
        let size = e.call(STRLEN, &args![text]).u32().wrapping_add(1);
        let block = e.call(ALLOCATE, &args![MEMORY_MANAGER, size]).u32();
        e.mem.set_u32(slot, block);
        let block = e.mem.u32(slot);
        e.call(MEMSET, &args![block, 0u32, size]);
        let block = e.mem.u32(slot);
        e.call(STRCPY_S, &args![block, size, text]);
    }
}

// Translated from 00af37a0 (disassembled, FalloutNV.exe 1.4.0.525)
/// `BSSystemUtilitiesStrings`' scalar deleting destructor (engine map; `thiscall`, one word:
/// the flags, `RET 4`): the destructor body (`00af35c0`), then with bit 0 of the flags the
/// block is freed (`00aa4060`). Answers `this`.
pub fn bs_system_utilities_strings_scalar_deleting_destructor(
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
        entry!(0x00af_3660, fn_00af3660(u32) -> u32),
        entry!(0x00af_3680, bs_system_utilities_strings_destroy(u32)),
        entry!(0x00af_36e0, fn_00af36e0(u32, u32, u32)),
        entry!(
            0x00af_37a0,
            bs_system_utilities_strings_scalar_deleting_destructor(u32, u32) -> u32
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
    fn set_copies_the_text_and_destroy_frees_every_string() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        let this = e.mem.alloc(0x24);
        let text = e.mem.alloc(8);
        e.mem.write(text, b"hello\0");
        e.call(0x00af_36e0, &args![this, 2u32, text]);
        let first = e.mem.u32(this + 12);
        assert_eq!(e.mem.cstr(first), b"hello".to_vec());
        assert_eq!(calls(&e, ALLOCATE), vec![vec![MEMORY_MANAGER, 6]]);
        assert_eq!(calls(&e, STRCPY_S), vec![vec![first, 6, text]]);
        // Set again: the old one freed first.
        e.call(0x00af_36e0, &args![this, 2u32, text]);
        assert_eq!(calls(&e, DEALLOCATE), vec![vec![MEMORY_MANAGER, first]]);
        let second = e.mem.u32(this + 12);
        // A null text only clears.
        e.call(0x00af_36e0, &args![this, 7u32, text]);
        let last = e.mem.u32(this + 32);
        fn_00af36e0(&mut e, this, 7, 0);
        assert_eq!(e.mem.u32(this + 32), 0);
        e.call(0x00af_36e0, &args![this, 7u32, text]);
        let last2 = e.mem.u32(this + 32);
        e.call_log = Some(vec![]);
        e.call(0x00af_3680, &args![this]);
        assert_eq!(
            calls(&e, DEALLOCATE),
            vec![vec![MEMORY_MANAGER, second], vec![MEMORY_MANAGER, last2]]
        );
        assert_ne!(last, 0);
        for i in 0..8 {
            assert_eq!(e.mem.u32(this + 4 + 4 * i), 0);
        }
    }

    #[test]
    fn the_slot_call_and_the_deleting_destructor() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        let this = e.mem.alloc(0x24);
        e.put_vtable(0x0900_0000, &[0, 0, 0x0a00_0000]);
        e.mem.set_u32(this, 0x0900_0000);
        e.register(0x0a00_0000, |_, _| 0x99u32.into_ret());
        assert_eq!(e.call(0x00af_3660, &args![this]).u32(), 0x99);
        e.register(DESTRUCT, |_, _| Ret::default());
        e.register(DEALLOCATE, |_, _| Ret::default());
        assert_eq!(e.call(0x00af_37a0, &args![this, 0u32]).u32(), this);
        assert!(calls(&e, DEALLOCATE).is_empty());
        assert_eq!(e.call(0x00af_37a0, &args![this, 1u32]).u32(), this);
        assert_eq!(calls(&e, DESTRUCT), vec![vec![this], vec![this]]);
        assert_eq!(calls(&e, DEALLOCATE), vec![vec![MEMORY_MANAGER, this]]);
    }
}

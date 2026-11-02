//! `BSSystemUtilities/bssavedatasystemutility.cpp` (Xbox PDB source unit), subsystem
//! `BSSystemUtilities`: its functions in FalloutNV.exe 1.4.0.525, translated
//! (docs/ENGINE_CRATE.md). The singleton getter, constructor and destructor body
//! (`00af2e50`, `00af2ed0`, `00af2f20`) are in `engine-core`'s `platform.rs`.

#[allow(unused_imports)]
use crate::prelude::*;

/// The memory manager (`011f6238`) and its release (`00aa4060`, `thiscall`, one word), and
/// the utility's destructor body (`00af2f20`, `thiscall`).
const MEMORY_MANAGER: u32 = 0x011f_6238;
const DEALLOCATE: u32 = 0x00aa_4060;
const DESTRUCT: u32 = 0x00af_2f20;

// Translated from 00af2f40 (disassembled, FalloutNV.exe 1.4.0.525)
/// No label (`thiscall`, plain `RET`): calls the object's own vtable slot +8 and passes its
/// answer on (EAX).
pub fn fn_00af2f40(e: &mut Engine, this: u32) -> u32 {
    e.vcall(this, 0x8, &args![]).u32()
}

// Translated from 00af2f60 (disassembled, FalloutNV.exe 1.4.0.525)
/// No label (`thiscall`, plain `RET`): clears the word at +8.
pub fn fn_00af2f60(e: &mut Engine, this: u32) {
    e.mem.set_u32(this.wrapping_add(8), 0);
}

// Translated from 00af2fc0 (disassembled, FalloutNV.exe 1.4.0.525)
/// `BSSaveDataSystemUtility`'s scalar deleting destructor (engine map; `thiscall`, one word:
/// the flags, `RET 4`): the destructor body (`00af2f20`), then with bit 0 of the flags the
/// block is freed (`00aa4060`). Answers `this`.
pub fn bs_save_data_system_utility_scalar_deleting_destructor(
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

// Translated from 00af2ff0 (disassembled, FalloutNV.exe 1.4.0.525)
/// No label (`thiscall`, two words it does not read, `RET 8`): answers 0 (`AL`).
pub fn fn_00af2ff0(_e: &mut Engine, _this: u32, _unused_1: u32, _unused_2: u32) -> u8 {
    0
}

// Translated from 00af3000 (disassembled, FalloutNV.exe 1.4.0.525)
/// No label (`thiscall`, one word, `RET 4`): stores the word at +8.
pub fn fn_00af3000(e: &mut Engine, this: u32, value: u32) {
    e.mem.set_u32(this.wrapping_add(8), value);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00af_2f40, fn_00af2f40(u32) -> u32),
        entry!(0x00af_2f60, fn_00af2f60(u32)),
        entry!(
            0x00af_2fc0,
            bs_save_data_system_utility_scalar_deleting_destructor(u32, u32) -> u32
        ),
        entry!(0x00af_2ff0, fn_00af2ff0(u32, u32, u32) -> u8),
        entry!(0x00af_3000, fn_00af3000(u32, u32)),
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
    fn the_word_at_8_and_the_slot_call() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x48);
        e.call(0x00af_3000, &args![this, 0x77u32]);
        assert_eq!(e.mem.u32(this + 8), 0x77);
        e.call(0x00af_2f60, &args![this]);
        assert_eq!(e.mem.u32(this + 8), 0);
        e.put_vtable(0x0900_0000, &[0, 0, 0x0a00_0000]);
        e.mem.set_u32(this, 0x0900_0000);
        e.register(0x0a00_0000, |_, _| 0x42u32.into_ret());
        assert_eq!(e.call(0x00af_2f40, &args![this]).u32(), 0x42);
        assert_eq!(e.call(0x00af_2ff0, &args![this, 1u32, 1u32]).u8(), 0);
    }

    #[test]
    fn the_deleting_destructor_restores_the_vtable_and_frees_with_bit_0() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        let this = e.mem.alloc(0x48);
        e.register(DEALLOCATE, |_, _| Ret::default());
        assert_eq!(e.call(0x00af_2fc0, &args![this, 0u32]).u32(), this);
        // The real destructor body stores the vtable 010a4288.
        assert_eq!(e.mem.u32(this), 0x010a_4288);
        assert!(calls(&e, DEALLOCATE).is_empty());
        assert_eq!(e.call(0x00af_2fc0, &args![this, 1u32]).u32(), this);
        assert_eq!(calls(&e, DEALLOCATE), vec![vec![MEMORY_MANAGER, this]]);
    }
}

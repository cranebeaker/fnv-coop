//! `BSSystemUtilities/bsdiscbootsystemutility.cpp` (Xbox PDB source unit), subsystem
//! `BSSystemUtilities`: its functions in FalloutNV.exe 1.4.0.525, translated
//! (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The memory manager (`011f6238`) and its release (`00aa4060`, `thiscall`, one word), the
/// utility's destructor body (`00af3a80`, `thiscall`) and the C runtime's `strcpy_s`
/// (`00ec65a6`, cdecl: destination, size, source).
const MEMORY_MANAGER: u32 = 0x011f_6238;
const DEALLOCATE: u32 = 0x00aa_4060;
const DESTRUCT: u32 = 0x00af_3a80;
const STRCPY_S: u32 = 0x00ec_65a6;

// Translated from 00af3aa0 (disassembled, FalloutNV.exe 1.4.0.525)
/// `BSDiscBootSystemUtility::Initialize` (engine map; `thiscall`, two words: a name and a
/// value, `RET 8`): the object's own vtable slot +8 first; a non-null name is copied into the
/// 0x40 bytes at +4 (`strcpy_s`); the value is stored at +0x44. EAX holds `this` at the end.
pub fn bs_disc_boot_system_utility_initialize(
    e: &mut Engine,
    this: u32,
    name: u32,
    value: u32,
) -> u32 {
    e.vcall(this, 0x8, &args![]);
    if name != 0 {
        e.call(STRCPY_S, &args![this.wrapping_add(4), 0x40u32, name]);
    }
    e.mem.set_u32(this.wrapping_add(0x44), value);
    this
}

// Translated from 00af3ae0 (disassembled, FalloutNV.exe 1.4.0.525)
/// No label (`thiscall`, plain `RET`): does nothing.
pub fn fn_00af3ae0(_e: &mut Engine, _this: u32) {}

// Translated from 00af3af0 (disassembled, FalloutNV.exe 1.4.0.525)
/// `BSDiscBootSystemUtility`'s scalar deleting destructor (engine map; `thiscall`, one word:
/// the flags, `RET 4`): the destructor body (`00af3a80`), then with bit 0 of the flags the
/// block is freed (`00aa4060`). Answers `this`.
pub fn bs_disc_boot_system_utility_scalar_deleting_destructor(
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
        entry!(
            0x00af_3aa0,
            bs_disc_boot_system_utility_initialize(u32, u32, u32) -> u32
        ),
        entry!(0x00af_3ae0, fn_00af3ae0(u32)),
        entry!(
            0x00af_3af0,
            bs_disc_boot_system_utility_scalar_deleting_destructor(u32, u32) -> u32
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
    fn initialize_copies_the_name_and_keeps_the_value() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        let this = e.mem.alloc(0x50);
        e.put_vtable(0x0900_0000, &[0, 0, 0x0a00_0000]);
        e.mem.set_u32(this, 0x0900_0000);
        e.register(0x0a00_0000, |_, _| Ret::default());
        let name = e.mem.alloc(8);
        e.mem.write(name, b"disc\0");
        assert_eq!(e.call(0x00af_3aa0, &args![this, name, 7u32]).u32(), this);
        assert_eq!(e.mem.bytes(this + 4, 5), b"disc\0".to_vec());
        assert_eq!(e.mem.u32(this + 0x44), 7);
        assert_eq!(calls(&e, 0x0a00_0000), vec![vec![this]]);
        assert_eq!(calls(&e, STRCPY_S), vec![vec![this + 4, 0x40, name]]);
        // No name: no copy, the value still stored.
        bs_disc_boot_system_utility_initialize(&mut e, this, 0, 8);
        assert_eq!(calls(&e, STRCPY_S).len(), 1);
        assert_eq!(e.mem.u32(this + 0x44), 8);
        e.call(0x00af_3ae0, &args![this]);
    }

    #[test]
    fn the_deleting_destructor_frees_with_bit_0() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register(DESTRUCT, |_, _| Ret::default());
        e.register(DEALLOCATE, |_, _| Ret::default());
        assert_eq!(e.call(0x00af_3af0, &args![0x100u32, 0u32]).u32(), 0x100);
        assert_eq!(e.call(0x00af_3af0, &args![0x100u32, 1u32]).u32(), 0x100);
        assert_eq!(calls(&e, DESTRUCT).len(), 2);
        assert_eq!(calls(&e, DEALLOCATE), vec![vec![MEMORY_MANAGER, 0x100]]);
    }
}

//! `BSSystemUtilities/bsgamedatasystemutility.cpp` (Xbox PDB source unit), subsystem
//! `BSSystemUtilities`: its functions in FalloutNV.exe 1.4.0.525, translated
//! (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The memory manager (`011f6238`) and its release (`00aa4060`, `thiscall`, one word), and
/// the utility's destructor body (`00af39c0`, `thiscall`).
const MEMORY_MANAGER: u32 = 0x011f_6238;
const DEALLOCATE: u32 = 0x00aa_4060;
const DESTRUCT: u32 = 0x00af_39c0;

// Translated from 00af39e0 (disassembled, FalloutNV.exe 1.4.0.525)
/// No label (`thiscall`, plain `RET`): calls the object's own vtable slot +8 and passes its
/// answer on (EAX).
pub fn fn_00af39e0(e: &mut Engine, this: u32) -> u32 {
    e.vcall(this, 0x8, &args![]).u32()
}

// Translated from 00af3a00 (disassembled, FalloutNV.exe 1.4.0.525)
/// No label (`thiscall`, plain `RET`): does nothing.
pub fn fn_00af3a00(_e: &mut Engine, _this: u32) {}

// Translated from 00af3a10 (disassembled, FalloutNV.exe 1.4.0.525)
/// `BSGameDataSystemUtility`'s scalar deleting destructor (engine map; `thiscall`, one word:
/// the flags, `RET 4`): the destructor body (`00af39c0`), then with bit 0 of the flags the
/// block is freed (`00aa4060`). Answers `this`.
pub fn bs_game_data_system_utility_scalar_deleting_destructor(
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
        entry!(0x00af_39e0, fn_00af39e0(u32) -> u32),
        entry!(0x00af_3a00, fn_00af3a00(u32)),
        entry!(
            0x00af_3a10,
            bs_game_data_system_utility_scalar_deleting_destructor(u32, u32) -> u32
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
    fn the_slot_call_and_the_deleting_destructor() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        let this = e.mem.alloc(0x10);
        e.put_vtable(0x0900_0000, &[0, 0, 0x0a00_0000]);
        e.mem.set_u32(this, 0x0900_0000);
        e.register(0x0a00_0000, |_, a| (a[0] + 3).into_ret());
        assert_eq!(e.call(0x00af_39e0, &args![this]).u32(), this + 3);
        e.call(0x00af_3a00, &args![this]);
        e.register(DESTRUCT, |_, _| Ret::default());
        e.register(DEALLOCATE, |_, _| Ret::default());
        assert_eq!(e.call(0x00af_3a10, &args![this, 0u32]).u32(), this);
        assert!(calls(&e, DEALLOCATE).is_empty());
        assert_eq!(e.call(0x00af_3a10, &args![this, 1u32]).u32(), this);
        assert_eq!(calls(&e, DESTRUCT), vec![vec![this], vec![this]]);
        assert_eq!(calls(&e, DEALLOCATE), vec![vec![MEMORY_MANAGER, this]]);
    }
}

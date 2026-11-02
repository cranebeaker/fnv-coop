//! `BSShader/run_00ba11e0` (Xbox PDB source unit), subsystem `BSShader`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The destructor body `00ba0400` (unnamed in the engine map; it stores the vtable
/// `010b7ec8`, which has no RTTI locator; `thiscall`, no stack words).
const DESTRUCT_00BA0400: u32 = 0x00ba_0400;
/// The memory manager singleton (`011f6238`) and its free (`00aa4060`, `thiscall`: block).
const MEMORY_MANAGER: u32 = 0x011f_6238;
const MEMORY_FREE: u32 = 0x00aa_4060;

// Translated from 00ba1230 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, one stack word, `RET 4`; the only reference is the
/// vtable `010b7ec8`): the scalar deleting destructor of the class whose destructor body is
/// `00ba0400`. Runs that body, then frees `this` through the memory manager when bit 0 of
/// `flags` is set. Answers `this`.
pub fn fn_00ba1230(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(DESTRUCT_00BA0400, &args![this]);
    if flags & 1 != 0 {
        e.call(MEMORY_FREE, &args![MEMORY_MANAGER, this]);
    }
    this
}

/// The vtable `fn_00ba11e0` stores (`010b7f00`, no RTTI locator before it) and its slot +8,
/// which the code calls directly through the table's word (`CALL dword ptr [010b7f08]`;
/// `thiscall`, one word: the buffer).
const ARRAY_VTABLE_010B7F00: u32 = 0x010b_7f00;
const ARRAY_DEALLOCATE_SLOT_010B7F08: u32 = 0x010b_7f08;

// Translated from 00ba11e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the scalar deleting destructor of a `BSSimpleArray`-shaped
/// object (slot 0 of the vtable `010b7f00`; `thiscall`, one word: the flags). The vtable is
/// stored; with a buffer (`+4`) the table's slot +8 frees it (with `this` and the buffer) and
/// the buffer, the reserved size (`+0xc`) and the size (`+8`) are cleared. With bit 0 of the
/// flags `this` goes back to the memory manager. Answers `this`.
pub fn fn_00ba11e0(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    let base = this.addr();
    let buffer = e.mem.u32(base + 4);
    e.mem.set_u32(base, ARRAY_VTABLE_010B7F00);
    if buffer != 0 {
        let deallocate = e.mem.u32(ARRAY_DEALLOCATE_SLOT_010B7F08);
        e.call(deallocate, &args![base, buffer]);
        e.mem.set_u32(base + 4, 0);
        e.mem.set_u32(base + 0xc, 0);
        e.mem.set_u32(base + 8, 0);
    }
    if flags & 1 != 0 {
        e.call(MEMORY_FREE, &args![MEMORY_MANAGER, this]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00ba_1230, fn_00ba1230(Ptr, u32) -> Ptr),
        entry!(0x00ba_11e0, fn_00ba11e0(Ptr, u8) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fn_00ba1230_destructs_then_frees_on_bit_zero() {
        for (flags, frees) in [(0u32, false), (2, false), (1, true), (0xffff_ffff, true)] {
            let mut e = Engine::new();
            e.register(DESTRUCT_00BA0400, |_, _| Ret::default());
            e.register(MEMORY_FREE, |_, _| Ret::default());
            e.call_log = Some(vec![]);
            assert_eq!(
                e.call(0x00ba_1230, &args![0x0e00_4000u32, flags]).u32(),
                0x0e00_4000
            );
            let log = e.call_log.take().unwrap();
            let mut expected = vec![(DESTRUCT_00BA0400, vec![0x0e00_4000])];
            if frees {
                expected.push((MEMORY_FREE, vec![MEMORY_MANAGER, 0x0e00_4000]));
            }
            assert_eq!(log[1..].to_vec(), expected);
        }
    }
}

#[cfg(test)]
mod n013_tests {
    use super::*;

    #[test]
    fn fn_00ba11e0_frees_the_buffer_and_on_bit_0_itself() {
        const DEALLOCATE: u32 = 0x0e00_9000;
        let mut e = Engine::new();
        e.put_vtable(ARRAY_VTABLE_010B7F00, &[0, 0, DEALLOCATE]);
        e.register(DEALLOCATE, |_, _| Ret::default());
        e.register(MEMORY_FREE, |_, _| Ret::default());
        let this = e.mem.alloc(0x10);
        e.mem.set_u32(this + 4, 0x7000);
        e.mem.set_u32(this + 8, 3);
        e.mem.set_u32(this + 0xc, 4);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00ba_11e0, &args![this, 0u32]).u32(), this);
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1..].to_vec(), vec![(DEALLOCATE, vec![this, 0x7000])]);
        assert_eq!(e.mem.u32(this), ARRAY_VTABLE_010B7F00);
        assert_eq!(
            [
                e.mem.u32(this + 4),
                e.mem.u32(this + 8),
                e.mem.u32(this + 0xc)
            ],
            [0, 0, 0]
        );
        // No buffer now: only the free with bit 0.
        e.call_log = Some(vec![]);
        e.call(0x00ba_11e0, &args![this, 1u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log[1..].to_vec(),
            vec![(MEMORY_FREE, vec![MEMORY_MANAGER, this])]
        );
    }
}

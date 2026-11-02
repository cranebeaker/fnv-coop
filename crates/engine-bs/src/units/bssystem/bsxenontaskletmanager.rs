//! `BSSystem/bsxenontaskletmanager.cpp` (Xbox PDB source unit), subsystem `BSSystem`: its
//! functions in FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The base tasklet manager's vtable (`010a48ac`), the import slot of KERNEL32 `CloseHandle`
/// (`00fdf07c`), the memory manager (`011f6238`) and its release (`00aa4060`, `thiscall`, one
/// word).
const BASE_VTABLE: u32 = 0x010a_48ac;
const CLOSE_HANDLE: u32 = 0x00fd_f07c;
const MEMORY_MANAGER: u32 = 0x011f_6238;
const DEALLOCATE: u32 = 0x00aa_4060;

// Translated from 00b02780 (disassembled, FalloutNV.exe 1.4.0.525)
/// `BSTaskletManager`'s scalar deleting destructor (engine map; `thiscall`, one word: the
/// flags, `RET 4`): stores the vtable `010a48ac` and closes the handle at +0xc
/// (`CloseHandle`), then with bit 0 of the flags the block is freed (`00aa4060`). Answers
/// `this`.
pub fn bs_tasklet_manager_scalar_deleting_destructor(e: &mut Engine, this: u32, flags: u32) -> u32 {
    e.mem.set_u32(this, BASE_VTABLE);
    let handle = e.mem.u32(this.wrapping_add(0xc));
    e.call(CLOSE_HANDLE, &args![handle]);
    if flags & 1 != 0 {
        e.call(DEALLOCATE, &args![MEMORY_MANAGER, this]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(
        0x00b0_2780,
        bs_tasklet_manager_scalar_deleting_destructor(u32, u32) -> u32
    )]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn closes_the_handle_and_frees_with_bit_0() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register(CLOSE_HANDLE, |_, _| 1u32.into_ret());
        e.register(DEALLOCATE, |_, _| Ret::default());
        let this = e.mem.alloc(0x10);
        e.mem.set_u32(this + 0xc, 0x44);
        assert_eq!(e.call(0x00b0_2780, &args![this, 0u32]).u32(), this);
        assert_eq!(e.mem.u32(this), BASE_VTABLE);
        assert_eq!(e.call(0x00b0_2780, &args![this, 1u32]).u32(), this);
        let log = e.call_log.take().unwrap();
        let closes: Vec<_> = log.iter().filter(|c| c.0 == CLOSE_HANDLE).collect();
        assert_eq!(closes.len(), 2);
        assert_eq!(closes[0].1, vec![0x44]);
        let frees: Vec<_> = log.iter().filter(|c| c.0 == DEALLOCATE).collect();
        assert_eq!(frees.len(), 1);
        assert_eq!(frees[0].1, vec![MEMORY_MANAGER, this]);
    }
}

//! `(unplaced)/run_00a96800` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The memory manager singleton and its deallocate (`00aa4060`, `thiscall`, one word).
const MEMORY_MANAGER: u32 = 0x011f_6238;
const DEALLOCATE: u32 = 0x00aa_4060;
/// `00aa10f0` (`cdecl`, one word): frees a block.
const FREE_BLOCK: u32 = 0x00aa_10f0;
/// `00a67c70`: the base destructor, reached with a `JMP` (same `this`).
const BASE_DESTRUCTOR: u32 = 0x00a6_7c70;

// Translated from 00a96800 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, no stack word): a destructor. Sets the vtable `010a05e4`; frees +0x4c and +0x54
/// (`00aa10f0`), gives +0x58 to the memory manager, frees +0x5c, gives +0x60 to the memory
/// manager (all five also when null); gives +0x44 and +0x48 to the memory manager and frees
/// +0x64 and +0x68 (`00aa10f0`) when they are set; then continues in the base destructor
/// `00a67c70` (a `JMP`; its result is passed on).
pub fn fn_00a96800(e: &mut Engine, this: Ptr) -> u32 {
    let base = this.addr();
    let first = e.mem.u32(base + 0x4c);
    e.mem.set_u32(base, 0x010a_05e4);
    e.call(FREE_BLOCK, &args![first]);
    let word = e.mem.u32(base + 0x54);
    e.call(FREE_BLOCK, &args![word]);
    let word = e.mem.u32(base + 0x58);
    e.call(DEALLOCATE, &args![MEMORY_MANAGER, word]);
    let word = e.mem.u32(base + 0x5c);
    e.call(FREE_BLOCK, &args![word]);
    let word = e.mem.u32(base + 0x60);
    e.call(DEALLOCATE, &args![MEMORY_MANAGER, word]);
    for offset in [0x44u32, 0x48] {
        let word = e.mem.u32(base + offset);
        if word != 0 {
            e.call(DEALLOCATE, &args![MEMORY_MANAGER, word]);
        }
    }
    for offset in [0x64u32, 0x68] {
        let word = e.mem.u32(base + offset);
        if word != 0 {
            e.call(FREE_BLOCK, &args![word]);
        }
    }
    e.call(BASE_DESTRUCTOR, &args![this]).u32()
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00a9_6800, fn_00a96800(Ptr) -> u32)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn frees_every_block_then_runs_the_base() {
        let mut e = Engine::new();
        for callee in [FREE_BLOCK, DEALLOCATE] {
            e.register(callee, |_, _| Ret::default());
        }
        e.register(BASE_DESTRUCTOR, |_, _| 0x31u32.into_ret());
        let this = e.mem.alloc(0x70);
        for (offset, value) in [
            (0x4cu32, 0x4c00u32),
            (0x54, 0x5400),
            (0x58, 0),
            (0x5c, 0x5c00),
        ] {
            e.mem.set_u32(this + offset, value);
        }
        e.mem.set_u32(this + 0x60, 0x6000);
        e.mem.set_u32(this + 0x48, 0x4800);
        e.mem.set_u32(this + 0x64, 0x6400);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00a9_6800, &args![this]).u32(), 0x31);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log[1..],
            [
                (FREE_BLOCK, vec![0x4c00]),
                (FREE_BLOCK, vec![0x5400]),
                (DEALLOCATE, vec![MEMORY_MANAGER, 0]),
                (FREE_BLOCK, vec![0x5c00]),
                (DEALLOCATE, vec![MEMORY_MANAGER, 0x6000]),
                (DEALLOCATE, vec![MEMORY_MANAGER, 0x4800]),
                (FREE_BLOCK, vec![0x6400]),
                (BASE_DESTRUCTOR, vec![this]),
            ]
        );
        assert_eq!(e.mem.u32(this), 0x010a_05e4);
    }
}

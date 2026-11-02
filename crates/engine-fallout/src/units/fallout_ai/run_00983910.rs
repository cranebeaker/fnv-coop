//! `fallout/ai/run_00983910` (Xbox PDB source unit), subsystem `fallout/ai`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The block pointer the two functions keep (`.data`).
const BLOCK: u32 = 0x011f_1708;
/// `00401000` (`cdecl`, the size) and `00401030` (`cdecl`, the block): the allocation and
/// free used here (no engine-map names).
const ALLOCATE: u32 = 0x0040_1000;
const FREE: u32 = 0x0040_1030;

// Translated from 00983910 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (no words; called by `00990f20`): when the pointer at
/// `011f1708` is null, stores there a 1-byte block from `00401000`.
pub fn fn_00983910(e: &mut Engine) {
    if e.mem.u32(BLOCK) != 0 {
        return;
    }
    let block = e.call(ALLOCATE, &args![1u32]).u32();
    e.mem.set_u32(BLOCK, block);
}

// Translated from 00983940 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (no words): hands the pointer at `011f1708` to `00401030`
/// (also when it is null) and clears it.
pub fn fn_00983940(e: &mut Engine) {
    let block = e.mem.u32(BLOCK);
    e.call(FREE, &args![block]);
    e.mem.set_u32(BLOCK, 0);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0098_3910, fn_00983910()),
        entry!(0x0098_3940, fn_00983940()),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fn_00983910_allocates_once() {
        let mut e = Engine::new();
        e.map(BLOCK & !0xfff, 0x1000);
        e.register(ALLOCATE, |_, _| 0x4444u32.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x0098_3910, &[]);
        assert_eq!(e.mem.u32(BLOCK), 0x4444);
        e.call(0x0098_3910, &[]);
        let allocations: Vec<_> = e
            .call_log
            .take()
            .unwrap()
            .into_iter()
            .filter(|(a, _)| *a == ALLOCATE)
            .collect();
        assert_eq!(allocations, vec![(ALLOCATE, vec![1])]);
    }

    #[test]
    fn fn_00983940_frees_and_clears() {
        let mut e = Engine::new();
        e.map(BLOCK & !0xfff, 0x1000);
        e.mem.set_u32(BLOCK, 0x4444);
        e.register(FREE, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0098_3940, &[]);
        e.call(0x0098_3940, &[]);
        let frees: Vec<_> = e
            .call_log
            .take()
            .unwrap()
            .into_iter()
            .filter(|(a, _)| *a == FREE)
            .collect();
        assert_eq!(frees, vec![(FREE, vec![0x4444]), (FREE, vec![0])]);
        assert_eq!(e.mem.u32(BLOCK), 0);
    }
}

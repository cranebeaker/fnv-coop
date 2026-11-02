//! `NiXenonRenderer/run_00aa1ad0` (Xbox PDB source unit), subsystem `NiXenonRenderer`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The block allocation `00aa1070` (`cdecl`, the size), `_memcpy` (`00ec44d0`, `cdecl`:
/// destination, source, size), `00aa1de0` (`cdecl`, four words: the block, its size and two
/// more words), `00aa1810` (`thiscall`, two words: the block and its size) and `NiFree`
/// (`00aa10f0`, `cdecl`).
const ALLOCATE_00AA1070: u32 = 0x00aa_1070;
const MEMCPY: u32 = 0x00ec_44d0;
const TRANSFORM_00AA1DE0: u32 = 0x00aa_1de0;
const CONSUME_00AA1810: u32 = 0x00aa_1810;
const NI_FREE: u32 = 0x00aa_10f0;

// Translated from 00aa1ad0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, five words; the address is stored by `00aa1b30`):
/// 0 for an empty block (`size` 0). Otherwise the `size` bytes at `source` are copied into a
/// new block (`00aa1070`), the block goes through `00aa1de0` with `a4` and `a5`, then
/// `00aa1810` runs on `target` (the first word, its `this`) with the block and its size, the
/// block is freed and `00aa1810`'s result returned.
pub fn fn_00aa1ad0(e: &mut Engine, target: u32, source: u32, size: u32, a4: u32, a5: u32) -> u32 {
    if size == 0 {
        return 0;
    }
    let block = e.call(ALLOCATE_00AA1070, &args![size]).u32();
    e.call(MEMCPY, &args![block, source, size]);
    e.call(TRANSFORM_00AA1DE0, &args![block, size, a4, a5]);
    let result = e.call(CONSUME_00AA1810, &args![target, block, size]).u32();
    e.call(NI_FREE, &args![block]);
    result
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(
        0x00aa_1ad0,
        fn_00aa1ad0(u32, u32, u32, u32, u32) -> u32
    )]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fn_00aa1ad0_copies_transforms_and_hands_over_the_block() {
        let mut e = Engine::new();
        e.register(ALLOCATE_00AA1070, |_, _| 0x0e10_0000u32.into_ret());
        e.register(MEMCPY, |_, _| Ret::default());
        e.register(TRANSFORM_00AA1DE0, |_, _| Ret::default());
        e.register(CONSUME_00AA1810, |_, _| 0x55u32.into_ret());
        e.register(NI_FREE, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        let result = e
            .call(
                0x00aa_1ad0,
                &args![0x0e20_0000u32, 0x0e30_0000u32, 0x40u32, 7u32, 8u32],
            )
            .u32();
        assert_eq!(result, 0x55);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log[1..].to_vec(),
            vec![
                (ALLOCATE_00AA1070, vec![0x40]),
                (MEMCPY, vec![0x0e10_0000, 0x0e30_0000, 0x40]),
                (TRANSFORM_00AA1DE0, vec![0x0e10_0000, 0x40, 7, 8]),
                (CONSUME_00AA1810, vec![0x0e20_0000, 0x0e10_0000, 0x40]),
                (NI_FREE, vec![0x0e10_0000]),
            ]
        );
    }

    #[test]
    fn fn_00aa1ad0_returns_zero_for_an_empty_block() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x00aa_1ad0, &args![1u32, 2u32, 0u32, 4u32, 5u32])
                .u32(),
            0
        );
        assert_eq!(e.call_log.take().unwrap().len(), 1);
    }
}
